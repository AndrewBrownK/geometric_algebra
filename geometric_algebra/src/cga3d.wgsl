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
    g0_: vec2<f32>,
    g1_: vec3<f32>,
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

fn scalar_dual(self_8: Scalar) -> AntiScalar {
    var self_9: Scalar;

    self_9 = self_8;
    let _e2: Scalar = self_9;
    return AntiScalar(_e2.g0_);
}

fn scalar_scalar_add(self_10: Scalar, other: Scalar) -> Scalar {
    var self_11: Scalar;
    var other_1: Scalar;

    self_11 = self_10;
    other_1 = other;
    let _e4: Scalar = self_11;
    let _e6: Scalar = other_1;
    return Scalar((_e4.g0_ + _e6.g0_));
}

fn scalar_scalar_sub(self_12: Scalar, other_2: Scalar) -> Scalar {
    var self_13: Scalar;
    var other_3: Scalar;

    self_13 = self_12;
    other_3 = other_2;
    let _e4: Scalar = self_13;
    let _e6: Scalar = other_3;
    return Scalar((_e4.g0_ - _e6.g0_));
}

fn scalar_scalar_mul(self_14: Scalar, other_4: Scalar) -> Scalar {
    var self_15: Scalar;
    var other_5: Scalar;

    self_15 = self_14;
    other_5 = other_4;
    let _e4: Scalar = self_15;
    let _e6: Scalar = other_5;
    return Scalar((_e4.g0_ * _e6.g0_));
}

fn scalar_scalar_div(self_16: Scalar, other_6: Scalar) -> Scalar {
    var self_17: Scalar;
    var other_7: Scalar;

    self_17 = self_16;
    other_7 = other_6;
    let _e4: Scalar = self_17;
    let _e8: Scalar = other_7;
    return Scalar((((_e4.g0_ * 1.0) / _e8.g0_) * 1.0));
}

fn scalar_scalar_geometric_product(self_18: Scalar, other_8: Scalar) -> Scalar {
    var self_19: Scalar;
    var other_9: Scalar;

    self_19 = self_18;
    other_9 = other_8;
    let _e4: Scalar = self_19;
    let _e6: Scalar = other_9;
    return Scalar((_e4.g0_ * _e6.g0_));
}

fn scalar_scalar_outer_product(self_20: Scalar, other_10: Scalar) -> Scalar {
    var self_21: Scalar;
    var other_11: Scalar;

    self_21 = self_20;
    other_11 = other_10;
    let _e4: Scalar = self_21;
    let _e6: Scalar = other_11;
    return Scalar((_e4.g0_ * _e6.g0_));
}

fn scalar_scalar_inner_product(self_22: Scalar, other_12: Scalar) -> Scalar {
    var self_23: Scalar;
    var other_13: Scalar;

    self_23 = self_22;
    other_13 = other_12;
    let _e4: Scalar = self_23;
    let _e6: Scalar = other_13;
    return Scalar((_e4.g0_ * _e6.g0_));
}

fn scalar_scalar_left_contraction(self_24: Scalar, other_14: Scalar) -> Scalar {
    var self_25: Scalar;
    var other_15: Scalar;

    self_25 = self_24;
    other_15 = other_14;
    let _e4: Scalar = self_25;
    let _e6: Scalar = other_15;
    return Scalar((_e4.g0_ * _e6.g0_));
}

fn scalar_scalar_right_contraction(self_26: Scalar, other_16: Scalar) -> Scalar {
    var self_27: Scalar;
    var other_17: Scalar;

    self_27 = self_26;
    other_17 = other_16;
    let _e4: Scalar = self_27;
    let _e6: Scalar = other_17;
    return Scalar((_e4.g0_ * _e6.g0_));
}

fn scalar_scalar_scalar_product(self_28: Scalar, other_18: Scalar) -> Scalar {
    var self_29: Scalar;
    var other_19: Scalar;

    self_29 = self_28;
    other_19 = other_18;
    let _e4: Scalar = self_29;
    let _e6: Scalar = other_19;
    return Scalar((_e4.g0_ * _e6.g0_));
}

fn scalar_anti_scalar_geometric_product(self_30: Scalar, other_20: AntiScalar) -> AntiScalar {
    var self_31: Scalar;
    var other_21: AntiScalar;

    self_31 = self_30;
    other_21 = other_20;
    let _e4: Scalar = self_31;
    let _e6: AntiScalar = other_21;
    return AntiScalar((_e4.g0_ * _e6.g0_));
}

fn scalar_anti_scalar_regressive_product(self_32: Scalar, other_22: AntiScalar) -> Scalar {
    var self_33: Scalar;
    var other_23: AntiScalar;

    self_33 = self_32;
    other_23 = other_22;
    let _e4: Scalar = self_33;
    let _e6: AntiScalar = other_23;
    return Scalar((_e4.g0_ * _e6.g0_));
}

fn scalar_anti_scalar_outer_product(self_34: Scalar, other_24: AntiScalar) -> AntiScalar {
    var self_35: Scalar;
    var other_25: AntiScalar;

    self_35 = self_34;
    other_25 = other_24;
    let _e4: Scalar = self_35;
    let _e6: AntiScalar = other_25;
    return AntiScalar((_e4.g0_ * _e6.g0_));
}

fn scalar_anti_scalar_inner_product(self_36: Scalar, other_26: AntiScalar) -> AntiScalar {
    var self_37: Scalar;
    var other_27: AntiScalar;

    self_37 = self_36;
    other_27 = other_26;
    let _e4: Scalar = self_37;
    let _e6: AntiScalar = other_27;
    return AntiScalar((_e4.g0_ * _e6.g0_));
}

fn scalar_anti_scalar_left_contraction(self_38: Scalar, other_28: AntiScalar) -> AntiScalar {
    var self_39: Scalar;
    var other_29: AntiScalar;

    self_39 = self_38;
    other_29 = other_28;
    let _e4: Scalar = self_39;
    let _e6: AntiScalar = other_29;
    return AntiScalar((_e4.g0_ * _e6.g0_));
}

fn scalar_radial_point_geometric_product(self_40: Scalar, other_30: RadialPoint) -> RadialPoint {
    var self_41: Scalar;
    var other_31: RadialPoint;

    self_41 = self_40;
    other_31 = other_30;
    let _e4: Scalar = self_41;
    let _e7: RadialPoint = other_31;
    let _e10: Scalar = self_41;
    let _e13: RadialPoint = other_31;
    return RadialPoint((vec3<f32>(_e4.g0_) * _e7.g0_), (vec2<f32>(_e10.g0_) * _e13.g1_));
}

fn scalar_radial_point_outer_product(self_42: Scalar, other_32: RadialPoint) -> RadialPoint {
    var self_43: Scalar;
    var other_33: RadialPoint;

    self_43 = self_42;
    other_33 = other_32;
    let _e4: Scalar = self_43;
    let _e7: RadialPoint = other_33;
    let _e10: Scalar = self_43;
    let _e13: RadialPoint = other_33;
    return RadialPoint((vec3<f32>(_e4.g0_) * _e7.g0_), (vec2<f32>(_e10.g0_) * _e13.g1_));
}

fn scalar_radial_point_inner_product(self_44: Scalar, other_34: RadialPoint) -> RadialPoint {
    var self_45: Scalar;
    var other_35: RadialPoint;

    self_45 = self_44;
    other_35 = other_34;
    let _e4: Scalar = self_45;
    let _e7: RadialPoint = other_35;
    let _e10: Scalar = self_45;
    let _e13: RadialPoint = other_35;
    return RadialPoint((vec3<f32>(_e4.g0_) * _e7.g0_), (vec2<f32>(_e10.g0_) * _e13.g1_));
}

fn scalar_radial_point_left_contraction(self_46: Scalar, other_36: RadialPoint) -> RadialPoint {
    var self_47: Scalar;
    var other_37: RadialPoint;

    self_47 = self_46;
    other_37 = other_36;
    let _e4: Scalar = self_47;
    let _e7: RadialPoint = other_37;
    let _e10: Scalar = self_47;
    let _e13: RadialPoint = other_37;
    return RadialPoint((vec3<f32>(_e4.g0_) * _e7.g0_), (vec2<f32>(_e10.g0_) * _e13.g1_));
}

fn scalar_flat_point_geometric_product(self_48: Scalar, other_38: FlatPoint) -> FlatPoint {
    var self_49: Scalar;
    var other_39: FlatPoint;

    self_49 = self_48;
    other_39 = other_38;
    let _e4: Scalar = self_49;
    let _e7: FlatPoint = other_39;
    return FlatPoint((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_flat_point_outer_product(self_50: Scalar, other_40: FlatPoint) -> FlatPoint {
    var self_51: Scalar;
    var other_41: FlatPoint;

    self_51 = self_50;
    other_41 = other_40;
    let _e4: Scalar = self_51;
    let _e7: FlatPoint = other_41;
    return FlatPoint((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_flat_point_inner_product(self_52: Scalar, other_42: FlatPoint) -> FlatPoint {
    var self_53: Scalar;
    var other_43: FlatPoint;

    self_53 = self_52;
    other_43 = other_42;
    let _e4: Scalar = self_53;
    let _e7: FlatPoint = other_43;
    return FlatPoint((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_flat_point_left_contraction(self_54: Scalar, other_44: FlatPoint) -> FlatPoint {
    var self_55: Scalar;
    var other_45: FlatPoint;

    self_55 = self_54;
    other_45 = other_44;
    let _e4: Scalar = self_55;
    let _e7: FlatPoint = other_45;
    return FlatPoint((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_dipole_geometric_product(self_56: Scalar, other_46: Dipole) -> Dipole {
    var self_57: Scalar;
    var other_47: Dipole;

    self_57 = self_56;
    other_47 = other_46;
    let _e4: Scalar = self_57;
    let _e7: Dipole = other_47;
    let _e10: Scalar = self_57;
    let _e13: Dipole = other_47;
    let _e16: Scalar = self_57;
    let _e19: Dipole = other_47;
    return Dipole((vec3<f32>(_e4.g0_) * _e7.g0_), (vec3<f32>(_e10.g0_) * _e13.g1_), (vec4<f32>(_e16.g0_) * _e19.g2_));
}

fn scalar_dipole_outer_product(self_58: Scalar, other_48: Dipole) -> Dipole {
    var self_59: Scalar;
    var other_49: Dipole;

    self_59 = self_58;
    other_49 = other_48;
    let _e4: Scalar = self_59;
    let _e7: Dipole = other_49;
    let _e10: Scalar = self_59;
    let _e13: Dipole = other_49;
    let _e16: Scalar = self_59;
    let _e19: Dipole = other_49;
    return Dipole((vec3<f32>(_e4.g0_) * _e7.g0_), (vec3<f32>(_e10.g0_) * _e13.g1_), (vec4<f32>(_e16.g0_) * _e19.g2_));
}

fn scalar_dipole_inner_product(self_60: Scalar, other_50: Dipole) -> Dipole {
    var self_61: Scalar;
    var other_51: Dipole;

    self_61 = self_60;
    other_51 = other_50;
    let _e4: Scalar = self_61;
    let _e7: Dipole = other_51;
    let _e10: Scalar = self_61;
    let _e13: Dipole = other_51;
    let _e16: Scalar = self_61;
    let _e19: Dipole = other_51;
    return Dipole((vec3<f32>(_e4.g0_) * _e7.g0_), (vec3<f32>(_e10.g0_) * _e13.g1_), (vec4<f32>(_e16.g0_) * _e19.g2_));
}

fn scalar_dipole_left_contraction(self_62: Scalar, other_52: Dipole) -> Dipole {
    var self_63: Scalar;
    var other_53: Dipole;

    self_63 = self_62;
    other_53 = other_52;
    let _e4: Scalar = self_63;
    let _e7: Dipole = other_53;
    let _e10: Scalar = self_63;
    let _e13: Dipole = other_53;
    let _e16: Scalar = self_63;
    let _e19: Dipole = other_53;
    return Dipole((vec3<f32>(_e4.g0_) * _e7.g0_), (vec3<f32>(_e10.g0_) * _e13.g1_), (vec4<f32>(_e16.g0_) * _e19.g2_));
}

fn scalar_line_geometric_product(self_64: Scalar, other_54: Line) -> Line {
    var self_65: Scalar;
    var other_55: Line;

    self_65 = self_64;
    other_55 = other_54;
    let _e4: Scalar = self_65;
    let _e7: Line = other_55;
    let _e10: Scalar = self_65;
    let _e13: Line = other_55;
    return Line((vec3<f32>(_e4.g0_) * _e7.g0_), (vec3<f32>(_e10.g0_) * _e13.g1_));
}

fn scalar_line_outer_product(self_66: Scalar, other_56: Line) -> Line {
    var self_67: Scalar;
    var other_57: Line;

    self_67 = self_66;
    other_57 = other_56;
    let _e4: Scalar = self_67;
    let _e7: Line = other_57;
    let _e10: Scalar = self_67;
    let _e13: Line = other_57;
    return Line((vec3<f32>(_e4.g0_) * _e7.g0_), (vec3<f32>(_e10.g0_) * _e13.g1_));
}

fn scalar_line_inner_product(self_68: Scalar, other_58: Line) -> Line {
    var self_69: Scalar;
    var other_59: Line;

    self_69 = self_68;
    other_59 = other_58;
    let _e4: Scalar = self_69;
    let _e7: Line = other_59;
    let _e10: Scalar = self_69;
    let _e13: Line = other_59;
    return Line((vec3<f32>(_e4.g0_) * _e7.g0_), (vec3<f32>(_e10.g0_) * _e13.g1_));
}

fn scalar_line_left_contraction(self_70: Scalar, other_60: Line) -> Line {
    var self_71: Scalar;
    var other_61: Line;

    self_71 = self_70;
    other_61 = other_60;
    let _e4: Scalar = self_71;
    let _e7: Line = other_61;
    let _e10: Scalar = self_71;
    let _e13: Line = other_61;
    return Line((vec3<f32>(_e4.g0_) * _e7.g0_), (vec3<f32>(_e10.g0_) * _e13.g1_));
}

fn scalar_circle_geometric_product(self_72: Scalar, other_62: Circle) -> Circle {
    var self_73: Scalar;
    var other_63: Circle;

    self_73 = self_72;
    other_63 = other_62;
    let _e4: Scalar = self_73;
    let _e7: Circle = other_63;
    let _e10: Scalar = self_73;
    let _e13: Circle = other_63;
    let _e16: Scalar = self_73;
    let _e19: Circle = other_63;
    return Circle((vec4<f32>(_e4.g0_) * _e7.g0_), (vec3<f32>(_e10.g0_) * _e13.g1_), (vec3<f32>(_e16.g0_) * _e19.g2_));
}

fn scalar_circle_outer_product(self_74: Scalar, other_64: Circle) -> Circle {
    var self_75: Scalar;
    var other_65: Circle;

    self_75 = self_74;
    other_65 = other_64;
    let _e4: Scalar = self_75;
    let _e7: Circle = other_65;
    let _e10: Scalar = self_75;
    let _e13: Circle = other_65;
    let _e16: Scalar = self_75;
    let _e19: Circle = other_65;
    return Circle((vec4<f32>(_e4.g0_) * _e7.g0_), (vec3<f32>(_e10.g0_) * _e13.g1_), (vec3<f32>(_e16.g0_) * _e19.g2_));
}

fn scalar_circle_inner_product(self_76: Scalar, other_66: Circle) -> Circle {
    var self_77: Scalar;
    var other_67: Circle;

    self_77 = self_76;
    other_67 = other_66;
    let _e4: Scalar = self_77;
    let _e7: Circle = other_67;
    let _e10: Scalar = self_77;
    let _e13: Circle = other_67;
    let _e16: Scalar = self_77;
    let _e19: Circle = other_67;
    return Circle((vec4<f32>(_e4.g0_) * _e7.g0_), (vec3<f32>(_e10.g0_) * _e13.g1_), (vec3<f32>(_e16.g0_) * _e19.g2_));
}

fn scalar_circle_left_contraction(self_78: Scalar, other_68: Circle) -> Circle {
    var self_79: Scalar;
    var other_69: Circle;

    self_79 = self_78;
    other_69 = other_68;
    let _e4: Scalar = self_79;
    let _e7: Circle = other_69;
    let _e10: Scalar = self_79;
    let _e13: Circle = other_69;
    let _e16: Scalar = self_79;
    let _e19: Circle = other_69;
    return Circle((vec4<f32>(_e4.g0_) * _e7.g0_), (vec3<f32>(_e10.g0_) * _e13.g1_), (vec3<f32>(_e16.g0_) * _e19.g2_));
}

fn scalar_plane_geometric_product(self_80: Scalar, other_70: Plane) -> Plane {
    var self_81: Scalar;
    var other_71: Plane;

    self_81 = self_80;
    other_71 = other_70;
    let _e4: Scalar = self_81;
    let _e7: Plane = other_71;
    return Plane((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_plane_outer_product(self_82: Scalar, other_72: Plane) -> Plane {
    var self_83: Scalar;
    var other_73: Plane;

    self_83 = self_82;
    other_73 = other_72;
    let _e4: Scalar = self_83;
    let _e7: Plane = other_73;
    return Plane((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_plane_inner_product(self_84: Scalar, other_74: Plane) -> Plane {
    var self_85: Scalar;
    var other_75: Plane;

    self_85 = self_84;
    other_75 = other_74;
    let _e4: Scalar = self_85;
    let _e7: Plane = other_75;
    return Plane((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_plane_left_contraction(self_86: Scalar, other_76: Plane) -> Plane {
    var self_87: Scalar;
    var other_77: Plane;

    self_87 = self_86;
    other_77 = other_76;
    let _e4: Scalar = self_87;
    let _e7: Plane = other_77;
    return Plane((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_sphere_geometric_product(self_88: Scalar, other_78: Sphere) -> Sphere {
    var self_89: Scalar;
    var other_79: Sphere;

    self_89 = self_88;
    other_79 = other_78;
    let _e4: Scalar = self_89;
    let _e7: Sphere = other_79;
    let _e10: Scalar = self_89;
    let _e13: Sphere = other_79;
    return Sphere((vec2<f32>(_e4.g0_) * _e7.g0_), (vec3<f32>(_e10.g0_) * _e13.g1_));
}

fn scalar_sphere_outer_product(self_90: Scalar, other_80: Sphere) -> Sphere {
    var self_91: Scalar;
    var other_81: Sphere;

    self_91 = self_90;
    other_81 = other_80;
    let _e4: Scalar = self_91;
    let _e7: Sphere = other_81;
    let _e10: Scalar = self_91;
    let _e13: Sphere = other_81;
    return Sphere((vec2<f32>(_e4.g0_) * _e7.g0_), (vec3<f32>(_e10.g0_) * _e13.g1_));
}

fn scalar_sphere_inner_product(self_92: Scalar, other_82: Sphere) -> Sphere {
    var self_93: Scalar;
    var other_83: Sphere;

    self_93 = self_92;
    other_83 = other_82;
    let _e4: Scalar = self_93;
    let _e7: Sphere = other_83;
    let _e10: Scalar = self_93;
    let _e13: Sphere = other_83;
    return Sphere((vec2<f32>(_e4.g0_) * _e7.g0_), (vec3<f32>(_e10.g0_) * _e13.g1_));
}

fn scalar_sphere_left_contraction(self_94: Scalar, other_84: Sphere) -> Sphere {
    var self_95: Scalar;
    var other_85: Sphere;

    self_95 = self_94;
    other_85 = other_84;
    let _e4: Scalar = self_95;
    let _e7: Sphere = other_85;
    let _e10: Scalar = self_95;
    let _e13: Sphere = other_85;
    return Sphere((vec2<f32>(_e4.g0_) * _e7.g0_), (vec3<f32>(_e10.g0_) * _e13.g1_));
}

fn scalar_motor_geometric_product(self_96: Scalar, other_86: Motor) -> Motor {
    var self_97: Scalar;
    var other_87: Motor;

    self_97 = self_96;
    other_87 = other_86;
    let _e4: Scalar = self_97;
    let _e7: Motor = other_87;
    let _e10: Scalar = self_97;
    let _e13: Motor = other_87;
    return Motor((vec4<f32>(_e4.g0_) * _e7.g0_), (vec4<f32>(_e10.g0_) * _e13.g1_));
}

fn scalar_motor_regressive_product(self_98: Scalar, other_88: Motor) -> Scalar {
    var self_99: Scalar;
    var other_89: Motor;

    self_99 = self_98;
    other_89 = other_88;
    let _e4: Scalar = self_99;
    let _e6: Motor = other_89;
    return Scalar((_e4.g0_ * _e6.g0_.w));
}

fn scalar_motor_outer_product(self_100: Scalar, other_90: Motor) -> Motor {
    var self_101: Scalar;
    var other_91: Motor;

    self_101 = self_100;
    other_91 = other_90;
    let _e4: Scalar = self_101;
    let _e7: Motor = other_91;
    let _e10: Scalar = self_101;
    let _e13: Motor = other_91;
    return Motor((vec4<f32>(_e4.g0_) * _e7.g0_), (vec4<f32>(_e10.g0_) * _e13.g1_));
}

fn scalar_motor_inner_product(self_102: Scalar, other_92: Motor) -> Motor {
    var self_103: Scalar;
    var other_93: Motor;

    self_103 = self_102;
    other_93 = other_92;
    let _e4: Scalar = self_103;
    let _e7: Motor = other_93;
    let _e10: Scalar = self_103;
    let _e13: Motor = other_93;
    return Motor((vec4<f32>(_e4.g0_) * _e7.g0_), (vec4<f32>(_e10.g0_) * _e13.g1_));
}

fn scalar_motor_left_contraction(self_104: Scalar, other_94: Motor) -> Motor {
    var self_105: Scalar;
    var other_95: Motor;

    self_105 = self_104;
    other_95 = other_94;
    let _e4: Scalar = self_105;
    let _e7: Motor = other_95;
    let _e10: Scalar = self_105;
    let _e13: Motor = other_95;
    return Motor((vec4<f32>(_e4.g0_) * _e7.g0_), (vec4<f32>(_e10.g0_) * _e13.g1_));
}

fn scalar_rotor_geometric_product(self_106: Scalar, other_96: Rotor) -> Rotor {
    var self_107: Scalar;
    var other_97: Rotor;

    self_107 = self_106;
    other_97 = other_96;
    let _e4: Scalar = self_107;
    let _e7: Rotor = other_97;
    return Rotor((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_rotor_regressive_product(self_108: Scalar, other_98: Rotor) -> Scalar {
    var self_109: Scalar;
    var other_99: Rotor;

    self_109 = self_108;
    other_99 = other_98;
    let _e4: Scalar = self_109;
    let _e6: Rotor = other_99;
    return Scalar((_e4.g0_ * _e6.g0_.w));
}

fn scalar_rotor_outer_product(self_110: Scalar, other_100: Rotor) -> Rotor {
    var self_111: Scalar;
    var other_101: Rotor;

    self_111 = self_110;
    other_101 = other_100;
    let _e4: Scalar = self_111;
    let _e7: Rotor = other_101;
    return Rotor((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_rotor_inner_product(self_112: Scalar, other_102: Rotor) -> Rotor {
    var self_113: Scalar;
    var other_103: Rotor;

    self_113 = self_112;
    other_103 = other_102;
    let _e4: Scalar = self_113;
    let _e7: Rotor = other_103;
    return Rotor((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_rotor_left_contraction(self_114: Scalar, other_104: Rotor) -> Rotor {
    var self_115: Scalar;
    var other_105: Rotor;

    self_115 = self_114;
    other_105 = other_104;
    let _e4: Scalar = self_115;
    let _e7: Rotor = other_105;
    return Rotor((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_translator_geometric_product(self_116: Scalar, other_106: Translator) -> Translator {
    var self_117: Scalar;
    var other_107: Translator;

    self_117 = self_116;
    other_107 = other_106;
    let _e4: Scalar = self_117;
    let _e7: Translator = other_107;
    return Translator((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_translator_regressive_product(self_118: Scalar, other_108: Translator) -> Scalar {
    var self_119: Scalar;
    var other_109: Translator;

    self_119 = self_118;
    other_109 = other_108;
    let _e4: Scalar = self_119;
    let _e6: Translator = other_109;
    return Scalar((_e4.g0_ * _e6.g0_.w));
}

fn scalar_translator_outer_product(self_120: Scalar, other_110: Translator) -> Translator {
    var self_121: Scalar;
    var other_111: Translator;

    self_121 = self_120;
    other_111 = other_110;
    let _e4: Scalar = self_121;
    let _e7: Translator = other_111;
    return Translator((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_translator_inner_product(self_122: Scalar, other_112: Translator) -> Translator {
    var self_123: Scalar;
    var other_113: Translator;

    self_123 = self_122;
    other_113 = other_112;
    let _e4: Scalar = self_123;
    let _e7: Translator = other_113;
    return Translator((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_translator_left_contraction(self_124: Scalar, other_114: Translator) -> Translator {
    var self_125: Scalar;
    var other_115: Translator;

    self_125 = self_124;
    other_115 = other_114;
    let _e4: Scalar = self_125;
    let _e7: Translator = other_115;
    return Translator((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_flector_geometric_product(self_126: Scalar, other_116: Flector) -> Flector {
    var self_127: Scalar;
    var other_117: Flector;

    self_127 = self_126;
    other_117 = other_116;
    let _e4: Scalar = self_127;
    let _e7: Flector = other_117;
    let _e10: Scalar = self_127;
    let _e13: Flector = other_117;
    return Flector((vec4<f32>(_e4.g0_) * _e7.g0_), (vec4<f32>(_e10.g0_) * _e13.g1_));
}

fn scalar_flector_outer_product(self_128: Scalar, other_118: Flector) -> Flector {
    var self_129: Scalar;
    var other_119: Flector;

    self_129 = self_128;
    other_119 = other_118;
    let _e4: Scalar = self_129;
    let _e7: Flector = other_119;
    let _e10: Scalar = self_129;
    let _e13: Flector = other_119;
    return Flector((vec4<f32>(_e4.g0_) * _e7.g0_), (vec4<f32>(_e10.g0_) * _e13.g1_));
}

fn scalar_flector_inner_product(self_130: Scalar, other_120: Flector) -> Flector {
    var self_131: Scalar;
    var other_121: Flector;

    self_131 = self_130;
    other_121 = other_120;
    let _e4: Scalar = self_131;
    let _e7: Flector = other_121;
    let _e10: Scalar = self_131;
    let _e13: Flector = other_121;
    return Flector((vec4<f32>(_e4.g0_) * _e7.g0_), (vec4<f32>(_e10.g0_) * _e13.g1_));
}

fn scalar_flector_left_contraction(self_132: Scalar, other_122: Flector) -> Flector {
    var self_133: Scalar;
    var other_123: Flector;

    self_133 = self_132;
    other_123 = other_122;
    let _e4: Scalar = self_133;
    let _e7: Flector = other_123;
    let _e10: Scalar = self_133;
    let _e13: Flector = other_123;
    return Flector((vec4<f32>(_e4.g0_) * _e7.g0_), (vec4<f32>(_e10.g0_) * _e13.g1_));
}

fn scalar_multi_vector_add(self_134: Scalar, other_124: MultiVector) -> MultiVector {
    var self_135: Scalar;
    var other_125: MultiVector;

    self_135 = self_134;
    other_125 = other_124;
    let _e4: Scalar = self_135;
    let _e12: MultiVector = other_125;
    let _e15: MultiVector = other_125;
    let _e17: MultiVector = other_125;
    let _e19: MultiVector = other_125;
    let _e21: MultiVector = other_125;
    let _e23: MultiVector = other_125;
    let _e25: MultiVector = other_125;
    let _e27: MultiVector = other_125;
    let _e29: MultiVector = other_125;
    let _e31: MultiVector = other_125;
    return MultiVector(((vec3<f32>(_e4.g0_) * vec3<f32>(1.0, 0.0, 0.0)) + _e12.g0_), _e15.g1_, _e17.g2_, _e19.g3_, _e21.g4_, _e23.g5_, _e25.g6_, _e27.g7_, _e29.g8_, _e31.g9_);
}

fn scalar_multi_vector_sub(self_136: Scalar, other_126: MultiVector) -> MultiVector {
    var self_137: Scalar;
    var other_127: MultiVector;

    self_137 = self_136;
    other_127 = other_126;
    let _e4: Scalar = self_137;
    let _e12: MultiVector = other_127;
    let _e17: MultiVector = other_127;
    let _e22: MultiVector = other_127;
    let _e27: MultiVector = other_127;
    let _e32: MultiVector = other_127;
    let _e37: MultiVector = other_127;
    let _e42: MultiVector = other_127;
    let _e47: MultiVector = other_127;
    let _e52: MultiVector = other_127;
    let _e57: MultiVector = other_127;
    return MultiVector(((vec3<f32>(_e4.g0_) * vec3<f32>(1.0, 0.0, 0.0)) - _e12.g0_), (vec3<f32>(0.0) - _e17.g1_), (vec2<f32>(0.0) - _e22.g2_), (vec4<f32>(0.0) - _e27.g3_), (vec3<f32>(0.0) - _e32.g4_), (vec3<f32>(0.0) - _e37.g5_), (vec3<f32>(0.0) - _e42.g6_), (vec3<f32>(0.0) - _e47.g7_), (vec4<f32>(0.0) - _e52.g8_), (vec4<f32>(0.0) - _e57.g9_));
}

fn scalar_multi_vector_geometric_product(self_138: Scalar, other_128: MultiVector) -> MultiVector {
    var self_139: Scalar;
    var other_129: MultiVector;

    self_139 = self_138;
    other_129 = other_128;
    let _e4: Scalar = self_139;
    let _e7: MultiVector = other_129;
    let _e10: Scalar = self_139;
    let _e13: MultiVector = other_129;
    let _e16: Scalar = self_139;
    let _e19: MultiVector = other_129;
    let _e22: Scalar = self_139;
    let _e25: MultiVector = other_129;
    let _e28: Scalar = self_139;
    let _e31: MultiVector = other_129;
    let _e34: Scalar = self_139;
    let _e37: MultiVector = other_129;
    let _e40: Scalar = self_139;
    let _e43: MultiVector = other_129;
    let _e46: Scalar = self_139;
    let _e49: MultiVector = other_129;
    let _e52: Scalar = self_139;
    let _e55: MultiVector = other_129;
    let _e58: Scalar = self_139;
    let _e61: MultiVector = other_129;
    return MultiVector((vec3<f32>(_e4.g0_) * _e7.g0_), (vec3<f32>(_e10.g0_) * _e13.g1_), (vec2<f32>(_e16.g0_) * _e19.g2_), (vec4<f32>(_e22.g0_) * _e25.g3_), (vec3<f32>(_e28.g0_) * _e31.g4_), (vec3<f32>(_e34.g0_) * _e37.g5_), (vec3<f32>(_e40.g0_) * _e43.g6_), (vec3<f32>(_e46.g0_) * _e49.g7_), (vec4<f32>(_e52.g0_) * _e55.g8_), (vec4<f32>(_e58.g0_) * _e61.g9_));
}

fn scalar_multi_vector_regressive_product(self_140: Scalar, other_130: MultiVector) -> Scalar {
    var self_141: Scalar;
    var other_131: MultiVector;

    self_141 = self_140;
    other_131 = other_130;
    let _e4: Scalar = self_141;
    let _e6: MultiVector = other_131;
    return Scalar((_e4.g0_ * _e6.g0_.z));
}

fn scalar_multi_vector_outer_product(self_142: Scalar, other_132: MultiVector) -> MultiVector {
    var self_143: Scalar;
    var other_133: MultiVector;

    self_143 = self_142;
    other_133 = other_132;
    let _e4: Scalar = self_143;
    let _e7: MultiVector = other_133;
    let _e10: Scalar = self_143;
    let _e13: MultiVector = other_133;
    let _e16: Scalar = self_143;
    let _e19: MultiVector = other_133;
    let _e22: Scalar = self_143;
    let _e25: MultiVector = other_133;
    let _e28: Scalar = self_143;
    let _e31: MultiVector = other_133;
    let _e34: Scalar = self_143;
    let _e37: MultiVector = other_133;
    let _e40: Scalar = self_143;
    let _e43: MultiVector = other_133;
    let _e46: Scalar = self_143;
    let _e49: MultiVector = other_133;
    let _e52: Scalar = self_143;
    let _e55: MultiVector = other_133;
    let _e58: Scalar = self_143;
    let _e61: MultiVector = other_133;
    return MultiVector((vec3<f32>(_e4.g0_) * _e7.g0_), (vec3<f32>(_e10.g0_) * _e13.g1_), (vec2<f32>(_e16.g0_) * _e19.g2_), (vec4<f32>(_e22.g0_) * _e25.g3_), (vec3<f32>(_e28.g0_) * _e31.g4_), (vec3<f32>(_e34.g0_) * _e37.g5_), (vec3<f32>(_e40.g0_) * _e43.g6_), (vec3<f32>(_e46.g0_) * _e49.g7_), (vec4<f32>(_e52.g0_) * _e55.g8_), (vec4<f32>(_e58.g0_) * _e61.g9_));
}

fn scalar_multi_vector_inner_product(self_144: Scalar, other_134: MultiVector) -> MultiVector {
    var self_145: Scalar;
    var other_135: MultiVector;

    self_145 = self_144;
    other_135 = other_134;
    let _e4: Scalar = self_145;
    let _e7: MultiVector = other_135;
    let _e10: Scalar = self_145;
    let _e13: MultiVector = other_135;
    let _e16: Scalar = self_145;
    let _e19: MultiVector = other_135;
    let _e22: Scalar = self_145;
    let _e25: MultiVector = other_135;
    let _e28: Scalar = self_145;
    let _e31: MultiVector = other_135;
    let _e34: Scalar = self_145;
    let _e37: MultiVector = other_135;
    let _e40: Scalar = self_145;
    let _e43: MultiVector = other_135;
    let _e46: Scalar = self_145;
    let _e49: MultiVector = other_135;
    let _e52: Scalar = self_145;
    let _e55: MultiVector = other_135;
    let _e58: Scalar = self_145;
    let _e61: MultiVector = other_135;
    return MultiVector((vec3<f32>(_e4.g0_) * _e7.g0_), (vec3<f32>(_e10.g0_) * _e13.g1_), (vec2<f32>(_e16.g0_) * _e19.g2_), (vec4<f32>(_e22.g0_) * _e25.g3_), (vec3<f32>(_e28.g0_) * _e31.g4_), (vec3<f32>(_e34.g0_) * _e37.g5_), (vec3<f32>(_e40.g0_) * _e43.g6_), (vec3<f32>(_e46.g0_) * _e49.g7_), (vec4<f32>(_e52.g0_) * _e55.g8_), (vec4<f32>(_e58.g0_) * _e61.g9_));
}

fn scalar_multi_vector_left_contraction(self_146: Scalar, other_136: MultiVector) -> MultiVector {
    var self_147: Scalar;
    var other_137: MultiVector;

    self_147 = self_146;
    other_137 = other_136;
    let _e4: Scalar = self_147;
    let _e7: MultiVector = other_137;
    let _e10: Scalar = self_147;
    let _e13: MultiVector = other_137;
    let _e16: Scalar = self_147;
    let _e19: MultiVector = other_137;
    let _e22: Scalar = self_147;
    let _e25: MultiVector = other_137;
    let _e28: Scalar = self_147;
    let _e31: MultiVector = other_137;
    let _e34: Scalar = self_147;
    let _e37: MultiVector = other_137;
    let _e40: Scalar = self_147;
    let _e43: MultiVector = other_137;
    let _e46: Scalar = self_147;
    let _e49: MultiVector = other_137;
    let _e52: Scalar = self_147;
    let _e55: MultiVector = other_137;
    let _e58: Scalar = self_147;
    let _e61: MultiVector = other_137;
    return MultiVector((vec3<f32>(_e4.g0_) * _e7.g0_), (vec3<f32>(_e10.g0_) * _e13.g1_), (vec2<f32>(_e16.g0_) * _e19.g2_), (vec4<f32>(_e22.g0_) * _e25.g3_), (vec3<f32>(_e28.g0_) * _e31.g4_), (vec3<f32>(_e34.g0_) * _e37.g5_), (vec3<f32>(_e40.g0_) * _e43.g6_), (vec3<f32>(_e46.g0_) * _e49.g7_), (vec4<f32>(_e52.g0_) * _e55.g8_), (vec4<f32>(_e58.g0_) * _e61.g9_));
}

fn scalar_multi_vector_right_contraction(self_148: Scalar, other_138: MultiVector) -> Scalar {
    var self_149: Scalar;
    var other_139: MultiVector;

    self_149 = self_148;
    other_139 = other_138;
    let _e4: Scalar = self_149;
    let _e6: MultiVector = other_139;
    return Scalar((_e4.g0_ * _e6.g0_.x));
}

fn scalar_multi_vector_scalar_product(self_150: Scalar, other_140: MultiVector) -> Scalar {
    var self_151: Scalar;
    var other_141: MultiVector;

    self_151 = self_150;
    other_141 = other_140;
    let _e4: Scalar = self_151;
    let _e6: MultiVector = other_141;
    return Scalar((_e4.g0_ * _e6.g0_.x));
}

fn scalar_squared_magnitude(self_152: Scalar) -> Scalar {
    var self_153: Scalar;

    self_153 = self_152;
    let _e2: Scalar = self_153;
    let _e3: Scalar = self_153;
    let _e4: Scalar = scalar_reversal(_e3);
    let _e5: Scalar = scalar_scalar_scalar_product(_e2, _e4);
    return _e5;
}

fn scalar_magnitude(self_154: Scalar) -> Scalar {
    var self_155: Scalar;

    self_155 = self_154;
    let _e2: Scalar = self_155;
    let _e3: Scalar = scalar_squared_magnitude(_e2);
    return Scalar(sqrt(_e3.g0_));
}

fn scalar_scale(self_156: Scalar, other_142: f32) -> Scalar {
    var self_157: Scalar;
    var other_143: f32;

    self_157 = self_156;
    other_143 = other_142;
    let _e4: Scalar = self_157;
    let _e5: f32 = other_143;
    let _e7: Scalar = scalar_scalar_geometric_product(_e4, Scalar(_e5));
    return _e7;
}

fn scalar_signum(self_158: Scalar) -> Scalar {
    var self_159: Scalar;

    self_159 = self_158;
    let _e2: Scalar = self_159;
    let _e3: Scalar = self_159;
    let _e4: Scalar = scalar_magnitude(_e3);
    let _e9: Scalar = scalar_scalar_geometric_product(_e2, Scalar((1.0 / _e4.g0_)));
    return _e9;
}

fn scalar_inverse(self_160: Scalar) -> Scalar {
    var self_161: Scalar;

    self_161 = self_160;
    let _e2: Scalar = self_161;
    let _e3: Scalar = scalar_reversal(_e2);
    let _e4: Scalar = self_161;
    let _e5: Scalar = scalar_squared_magnitude(_e4);
    let _e10: Scalar = scalar_scalar_geometric_product(_e3, Scalar((1.0 / _e5.g0_)));
    return _e10;
}

fn anti_scalar_zero() -> AntiScalar {
    return AntiScalar(0.0);
}

fn anti_scalar_one() -> AntiScalar {
    return AntiScalar(0.0);
}

fn anti_scalar_neg(self_162: AntiScalar) -> AntiScalar {
    var self_163: AntiScalar;

    self_163 = self_162;
    let _e2: AntiScalar = self_163;
    return AntiScalar((_e2.g0_ * -(1.0)));
}

fn anti_scalar_automorphism(self_164: AntiScalar) -> AntiScalar {
    var self_165: AntiScalar;

    self_165 = self_164;
    let _e2: AntiScalar = self_165;
    return AntiScalar((_e2.g0_ * -(1.0)));
}

fn anti_scalar_reversal(self_166: AntiScalar) -> AntiScalar {
    var self_167: AntiScalar;

    self_167 = self_166;
    let _e2: AntiScalar = self_167;
    return AntiScalar(_e2.g0_);
}

fn anti_scalar_conjugation(self_168: AntiScalar) -> AntiScalar {
    var self_169: AntiScalar;

    self_169 = self_168;
    let _e2: AntiScalar = self_169;
    return AntiScalar((_e2.g0_ * -(1.0)));
}

fn anti_scalar_dual(self_170: AntiScalar) -> Scalar {
    var self_171: AntiScalar;

    self_171 = self_170;
    let _e2: AntiScalar = self_171;
    return Scalar(_e2.g0_);
}

fn anti_scalar_scalar_geometric_product(self_172: AntiScalar, other_144: Scalar) -> AntiScalar {
    var self_173: AntiScalar;
    var other_145: Scalar;

    self_173 = self_172;
    other_145 = other_144;
    let _e4: AntiScalar = self_173;
    let _e6: Scalar = other_145;
    return AntiScalar((_e4.g0_ * _e6.g0_));
}

fn anti_scalar_scalar_regressive_product(self_174: AntiScalar, other_146: Scalar) -> Scalar {
    var self_175: AntiScalar;
    var other_147: Scalar;

    self_175 = self_174;
    other_147 = other_146;
    let _e4: AntiScalar = self_175;
    let _e6: Scalar = other_147;
    return Scalar((_e4.g0_ * _e6.g0_));
}

fn anti_scalar_scalar_outer_product(self_176: AntiScalar, other_148: Scalar) -> AntiScalar {
    var self_177: AntiScalar;
    var other_149: Scalar;

    self_177 = self_176;
    other_149 = other_148;
    let _e4: AntiScalar = self_177;
    let _e6: Scalar = other_149;
    return AntiScalar((_e4.g0_ * _e6.g0_));
}

fn anti_scalar_scalar_inner_product(self_178: AntiScalar, other_150: Scalar) -> AntiScalar {
    var self_179: AntiScalar;
    var other_151: Scalar;

    self_179 = self_178;
    other_151 = other_150;
    let _e4: AntiScalar = self_179;
    let _e6: Scalar = other_151;
    return AntiScalar((_e4.g0_ * _e6.g0_));
}

fn anti_scalar_scalar_right_contraction(self_180: AntiScalar, other_152: Scalar) -> AntiScalar {
    var self_181: AntiScalar;
    var other_153: Scalar;

    self_181 = self_180;
    other_153 = other_152;
    let _e4: AntiScalar = self_181;
    let _e6: Scalar = other_153;
    return AntiScalar((_e4.g0_ * _e6.g0_));
}

fn anti_scalar_anti_scalar_add(self_182: AntiScalar, other_154: AntiScalar) -> AntiScalar {
    var self_183: AntiScalar;
    var other_155: AntiScalar;

    self_183 = self_182;
    other_155 = other_154;
    let _e4: AntiScalar = self_183;
    let _e6: AntiScalar = other_155;
    return AntiScalar((_e4.g0_ + _e6.g0_));
}

fn anti_scalar_anti_scalar_sub(self_184: AntiScalar, other_156: AntiScalar) -> AntiScalar {
    var self_185: AntiScalar;
    var other_157: AntiScalar;

    self_185 = self_184;
    other_157 = other_156;
    let _e4: AntiScalar = self_185;
    let _e6: AntiScalar = other_157;
    return AntiScalar((_e4.g0_ - _e6.g0_));
}

fn anti_scalar_anti_scalar_mul(self_186: AntiScalar, other_158: AntiScalar) -> AntiScalar {
    var self_187: AntiScalar;
    var other_159: AntiScalar;

    self_187 = self_186;
    other_159 = other_158;
    let _e4: AntiScalar = self_187;
    let _e6: AntiScalar = other_159;
    return AntiScalar((_e4.g0_ * _e6.g0_));
}

fn anti_scalar_anti_scalar_div(self_188: AntiScalar, other_160: AntiScalar) -> AntiScalar {
    var self_189: AntiScalar;
    var other_161: AntiScalar;

    self_189 = self_188;
    other_161 = other_160;
    let _e4: AntiScalar = self_189;
    let _e8: AntiScalar = other_161;
    return AntiScalar((((_e4.g0_ * 1.0) / _e8.g0_) * 1.0));
}

fn anti_scalar_anti_scalar_regressive_product(self_190: AntiScalar, other_162: AntiScalar) -> AntiScalar {
    var self_191: AntiScalar;
    var other_163: AntiScalar;

    self_191 = self_190;
    other_163 = other_162;
    let _e4: AntiScalar = self_191;
    let _e6: AntiScalar = other_163;
    return AntiScalar((_e4.g0_ * _e6.g0_));
}

fn anti_scalar_radial_point_regressive_product(self_192: AntiScalar, other_164: RadialPoint) -> RadialPoint {
    var self_193: AntiScalar;
    var other_165: RadialPoint;

    self_193 = self_192;
    other_165 = other_164;
    let _e4: AntiScalar = self_193;
    let _e7: RadialPoint = other_165;
    let _e10: AntiScalar = self_193;
    let _e13: RadialPoint = other_165;
    return RadialPoint((vec3<f32>(_e4.g0_) * _e7.g0_), (vec2<f32>(_e10.g0_) * _e13.g1_));
}

fn anti_scalar_flat_point_regressive_product(self_194: AntiScalar, other_166: FlatPoint) -> FlatPoint {
    var self_195: AntiScalar;
    var other_167: FlatPoint;

    self_195 = self_194;
    other_167 = other_166;
    let _e4: AntiScalar = self_195;
    let _e7: FlatPoint = other_167;
    return FlatPoint((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn anti_scalar_dipole_regressive_product(self_196: AntiScalar, other_168: Dipole) -> Dipole {
    var self_197: AntiScalar;
    var other_169: Dipole;

    self_197 = self_196;
    other_169 = other_168;
    let _e4: AntiScalar = self_197;
    let _e7: Dipole = other_169;
    let _e10: AntiScalar = self_197;
    let _e13: Dipole = other_169;
    let _e16: AntiScalar = self_197;
    let _e19: Dipole = other_169;
    return Dipole((vec3<f32>(_e4.g0_) * _e7.g0_), (vec3<f32>(_e10.g0_) * _e13.g1_), (vec4<f32>(_e16.g0_) * _e19.g2_));
}

fn anti_scalar_line_regressive_product(self_198: AntiScalar, other_170: Line) -> Line {
    var self_199: AntiScalar;
    var other_171: Line;

    self_199 = self_198;
    other_171 = other_170;
    let _e4: AntiScalar = self_199;
    let _e7: Line = other_171;
    let _e10: AntiScalar = self_199;
    let _e13: Line = other_171;
    return Line((vec3<f32>(_e4.g0_) * _e7.g0_), (vec3<f32>(_e10.g0_) * _e13.g1_));
}

fn anti_scalar_circle_regressive_product(self_200: AntiScalar, other_172: Circle) -> Circle {
    var self_201: AntiScalar;
    var other_173: Circle;

    self_201 = self_200;
    other_173 = other_172;
    let _e4: AntiScalar = self_201;
    let _e7: Circle = other_173;
    let _e10: AntiScalar = self_201;
    let _e13: Circle = other_173;
    let _e16: AntiScalar = self_201;
    let _e19: Circle = other_173;
    return Circle((vec4<f32>(_e4.g0_) * _e7.g0_), (vec3<f32>(_e10.g0_) * _e13.g1_), (vec3<f32>(_e16.g0_) * _e19.g2_));
}

fn anti_scalar_plane_regressive_product(self_202: AntiScalar, other_174: Plane) -> Plane {
    var self_203: AntiScalar;
    var other_175: Plane;

    self_203 = self_202;
    other_175 = other_174;
    let _e4: AntiScalar = self_203;
    let _e7: Plane = other_175;
    return Plane((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn anti_scalar_sphere_regressive_product(self_204: AntiScalar, other_176: Sphere) -> Sphere {
    var self_205: AntiScalar;
    var other_177: Sphere;

    self_205 = self_204;
    other_177 = other_176;
    let _e4: AntiScalar = self_205;
    let _e7: Sphere = other_177;
    let _e10: AntiScalar = self_205;
    let _e13: Sphere = other_177;
    return Sphere((vec2<f32>(_e4.g0_) * _e7.g0_), (vec3<f32>(_e10.g0_) * _e13.g1_));
}

fn anti_scalar_motor_add(self_206: AntiScalar, other_178: Motor) -> Motor {
    var self_207: AntiScalar;
    var other_179: Motor;

    self_207 = self_206;
    other_179 = other_178;
    let _e4: AntiScalar = self_207;
    let _e13: Motor = other_179;
    let _e16: Motor = other_179;
    return Motor(((vec4<f32>(_e4.g0_) * vec4<f32>(0.0, 0.0, 0.0, 1.0)) + _e13.g0_), _e16.g1_);
}

fn anti_scalar_motor_sub(self_208: AntiScalar, other_180: Motor) -> Motor {
    var self_209: AntiScalar;
    var other_181: Motor;

    self_209 = self_208;
    other_181 = other_180;
    let _e4: AntiScalar = self_209;
    let _e13: Motor = other_181;
    let _e18: Motor = other_181;
    return Motor(((vec4<f32>(_e4.g0_) * vec4<f32>(0.0, 0.0, 0.0, 1.0)) - _e13.g0_), (vec4<f32>(0.0) - _e18.g1_));
}

fn anti_scalar_motor_regressive_product(self_210: AntiScalar, other_182: Motor) -> Motor {
    var self_211: AntiScalar;
    var other_183: Motor;

    self_211 = self_210;
    other_183 = other_182;
    let _e4: AntiScalar = self_211;
    let _e7: Motor = other_183;
    let _e10: AntiScalar = self_211;
    let _e13: Motor = other_183;
    return Motor((vec4<f32>(_e4.g0_) * _e7.g0_), (vec4<f32>(_e10.g0_) * _e13.g1_));
}

fn anti_scalar_rotor_add(self_212: AntiScalar, other_184: Rotor) -> Rotor {
    var self_213: AntiScalar;
    var other_185: Rotor;

    self_213 = self_212;
    other_185 = other_184;
    let _e4: AntiScalar = self_213;
    let _e13: Rotor = other_185;
    return Rotor(((vec4<f32>(_e4.g0_) * vec4<f32>(0.0, 0.0, 0.0, 1.0)) + _e13.g0_));
}

fn anti_scalar_rotor_sub(self_214: AntiScalar, other_186: Rotor) -> Rotor {
    var self_215: AntiScalar;
    var other_187: Rotor;

    self_215 = self_214;
    other_187 = other_186;
    let _e4: AntiScalar = self_215;
    let _e13: Rotor = other_187;
    return Rotor(((vec4<f32>(_e4.g0_) * vec4<f32>(0.0, 0.0, 0.0, 1.0)) - _e13.g0_));
}

fn anti_scalar_rotor_regressive_product(self_216: AntiScalar, other_188: Rotor) -> Rotor {
    var self_217: AntiScalar;
    var other_189: Rotor;

    self_217 = self_216;
    other_189 = other_188;
    let _e4: AntiScalar = self_217;
    let _e7: Rotor = other_189;
    return Rotor((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn anti_scalar_translator_add(self_218: AntiScalar, other_190: Translator) -> Translator {
    var self_219: AntiScalar;
    var other_191: Translator;

    self_219 = self_218;
    other_191 = other_190;
    let _e4: AntiScalar = self_219;
    let _e13: Translator = other_191;
    return Translator(((vec4<f32>(_e4.g0_) * vec4<f32>(0.0, 0.0, 0.0, 1.0)) + _e13.g0_));
}

fn anti_scalar_translator_sub(self_220: AntiScalar, other_192: Translator) -> Translator {
    var self_221: AntiScalar;
    var other_193: Translator;

    self_221 = self_220;
    other_193 = other_192;
    let _e4: AntiScalar = self_221;
    let _e13: Translator = other_193;
    return Translator(((vec4<f32>(_e4.g0_) * vec4<f32>(0.0, 0.0, 0.0, 1.0)) - _e13.g0_));
}

fn anti_scalar_translator_regressive_product(self_222: AntiScalar, other_194: Translator) -> Translator {
    var self_223: AntiScalar;
    var other_195: Translator;

    self_223 = self_222;
    other_195 = other_194;
    let _e4: AntiScalar = self_223;
    let _e7: Translator = other_195;
    return Translator((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn anti_scalar_flector_regressive_product(self_224: AntiScalar, other_196: Flector) -> Flector {
    var self_225: AntiScalar;
    var other_197: Flector;

    self_225 = self_224;
    other_197 = other_196;
    let _e4: AntiScalar = self_225;
    let _e7: Flector = other_197;
    let _e10: AntiScalar = self_225;
    let _e13: Flector = other_197;
    return Flector((vec4<f32>(_e4.g0_) * _e7.g0_), (vec4<f32>(_e10.g0_) * _e13.g1_));
}

fn anti_scalar_multi_vector_add(self_226: AntiScalar, other_198: MultiVector) -> MultiVector {
    var self_227: AntiScalar;
    var other_199: MultiVector;

    self_227 = self_226;
    other_199 = other_198;
    let _e4: AntiScalar = self_227;
    let _e12: MultiVector = other_199;
    let _e15: MultiVector = other_199;
    let _e17: MultiVector = other_199;
    let _e19: MultiVector = other_199;
    let _e21: MultiVector = other_199;
    let _e23: MultiVector = other_199;
    let _e25: MultiVector = other_199;
    let _e27: MultiVector = other_199;
    let _e29: MultiVector = other_199;
    let _e31: MultiVector = other_199;
    return MultiVector(((vec3<f32>(_e4.g0_) * vec3<f32>(0.0, 0.0, 1.0)) + _e12.g0_), _e15.g1_, _e17.g2_, _e19.g3_, _e21.g4_, _e23.g5_, _e25.g6_, _e27.g7_, _e29.g8_, _e31.g9_);
}

fn anti_scalar_multi_vector_sub(self_228: AntiScalar, other_200: MultiVector) -> MultiVector {
    var self_229: AntiScalar;
    var other_201: MultiVector;

    self_229 = self_228;
    other_201 = other_200;
    let _e4: AntiScalar = self_229;
    let _e12: MultiVector = other_201;
    let _e17: MultiVector = other_201;
    let _e22: MultiVector = other_201;
    let _e27: MultiVector = other_201;
    let _e32: MultiVector = other_201;
    let _e37: MultiVector = other_201;
    let _e42: MultiVector = other_201;
    let _e47: MultiVector = other_201;
    let _e52: MultiVector = other_201;
    let _e57: MultiVector = other_201;
    return MultiVector(((vec3<f32>(_e4.g0_) * vec3<f32>(0.0, 0.0, 1.0)) - _e12.g0_), (vec3<f32>(0.0) - _e17.g1_), (vec2<f32>(0.0) - _e22.g2_), (vec4<f32>(0.0) - _e27.g3_), (vec3<f32>(0.0) - _e32.g4_), (vec3<f32>(0.0) - _e37.g5_), (vec3<f32>(0.0) - _e42.g6_), (vec3<f32>(0.0) - _e47.g7_), (vec4<f32>(0.0) - _e52.g8_), (vec4<f32>(0.0) - _e57.g9_));
}

fn anti_scalar_multi_vector_regressive_product(self_230: AntiScalar, other_202: MultiVector) -> MultiVector {
    var self_231: AntiScalar;
    var other_203: MultiVector;

    self_231 = self_230;
    other_203 = other_202;
    let _e4: AntiScalar = self_231;
    let _e7: MultiVector = other_203;
    let _e10: AntiScalar = self_231;
    let _e13: MultiVector = other_203;
    let _e16: AntiScalar = self_231;
    let _e19: MultiVector = other_203;
    let _e22: AntiScalar = self_231;
    let _e25: MultiVector = other_203;
    let _e28: AntiScalar = self_231;
    let _e31: MultiVector = other_203;
    let _e34: AntiScalar = self_231;
    let _e37: MultiVector = other_203;
    let _e40: AntiScalar = self_231;
    let _e43: MultiVector = other_203;
    let _e46: AntiScalar = self_231;
    let _e49: MultiVector = other_203;
    let _e52: AntiScalar = self_231;
    let _e55: MultiVector = other_203;
    let _e58: AntiScalar = self_231;
    let _e61: MultiVector = other_203;
    return MultiVector((vec3<f32>(_e4.g0_) * _e7.g0_), (vec3<f32>(_e10.g0_) * _e13.g1_), (vec2<f32>(_e16.g0_) * _e19.g2_), (vec4<f32>(_e22.g0_) * _e25.g3_), (vec3<f32>(_e28.g0_) * _e31.g4_), (vec3<f32>(_e34.g0_) * _e37.g5_), (vec3<f32>(_e40.g0_) * _e43.g6_), (vec3<f32>(_e46.g0_) * _e49.g7_), (vec4<f32>(_e52.g0_) * _e55.g8_), (vec4<f32>(_e58.g0_) * _e61.g9_));
}

fn anti_scalar_multi_vector_outer_product(self_232: AntiScalar, other_204: MultiVector) -> AntiScalar {
    var self_233: AntiScalar;
    var other_205: MultiVector;

    self_233 = self_232;
    other_205 = other_204;
    let _e4: AntiScalar = self_233;
    let _e6: MultiVector = other_205;
    return AntiScalar((_e4.g0_ * _e6.g0_.x));
}

fn anti_scalar_scale(self_234: AntiScalar, other_206: f32) -> AntiScalar {
    var self_235: AntiScalar;
    var other_207: f32;

    self_235 = self_234;
    other_207 = other_206;
    let _e4: AntiScalar = self_235;
    let _e5: f32 = other_207;
    let _e7: AntiScalar = anti_scalar_scalar_geometric_product(_e4, Scalar(_e5));
    return _e7;
}

fn radial_point_zero() -> RadialPoint {
    return RadialPoint(vec3<f32>(0.0), vec2<f32>(0.0));
}

fn radial_point_one() -> RadialPoint {
    return RadialPoint(vec3<f32>(0.0), vec2<f32>(0.0));
}

fn radial_point_neg(self_236: RadialPoint) -> RadialPoint {
    var self_237: RadialPoint;

    self_237 = self_236;
    let _e2: RadialPoint = self_237;
    let _e8: RadialPoint = self_237;
    return RadialPoint((_e2.g0_ * vec3<f32>(-(1.0))), (_e8.g1_ * vec2<f32>(-(1.0))));
}

fn radial_point_automorphism(self_238: RadialPoint) -> RadialPoint {
    var self_239: RadialPoint;

    self_239 = self_238;
    let _e2: RadialPoint = self_239;
    let _e8: RadialPoint = self_239;
    return RadialPoint((_e2.g0_ * vec3<f32>(-(1.0))), (_e8.g1_ * vec2<f32>(-(1.0))));
}

fn radial_point_reversal(self_240: RadialPoint) -> RadialPoint {
    var self_241: RadialPoint;

    self_241 = self_240;
    let _e2: RadialPoint = self_241;
    let _e4: RadialPoint = self_241;
    return RadialPoint(_e2.g0_, _e4.g1_);
}

fn radial_point_conjugation(self_242: RadialPoint) -> RadialPoint {
    var self_243: RadialPoint;

    self_243 = self_242;
    let _e2: RadialPoint = self_243;
    let _e8: RadialPoint = self_243;
    return RadialPoint((_e2.g0_ * vec3<f32>(-(1.0))), (_e8.g1_ * vec2<f32>(-(1.0))));
}

fn radial_point_dual(self_244: RadialPoint) -> Sphere {
    var self_245: RadialPoint;

    self_245 = self_244;
    let _e2: RadialPoint = self_245;
    let _e5: RadialPoint = self_245;
    return Sphere(_e2.g1_.yx, _e5.g0_);
}

fn radial_point_scalar_geometric_product(self_246: RadialPoint, other_208: Scalar) -> RadialPoint {
    var self_247: RadialPoint;
    var other_209: Scalar;

    self_247 = self_246;
    other_209 = other_208;
    let _e4: RadialPoint = self_247;
    let _e6: Scalar = other_209;
    let _e10: RadialPoint = self_247;
    let _e12: Scalar = other_209;
    return RadialPoint((_e4.g0_ * vec3<f32>(_e6.g0_)), (_e10.g1_ * vec2<f32>(_e12.g0_)));
}

fn radial_point_scalar_outer_product(self_248: RadialPoint, other_210: Scalar) -> RadialPoint {
    var self_249: RadialPoint;
    var other_211: Scalar;

    self_249 = self_248;
    other_211 = other_210;
    let _e4: RadialPoint = self_249;
    let _e6: Scalar = other_211;
    let _e10: RadialPoint = self_249;
    let _e12: Scalar = other_211;
    return RadialPoint((_e4.g0_ * vec3<f32>(_e6.g0_)), (_e10.g1_ * vec2<f32>(_e12.g0_)));
}

fn radial_point_scalar_inner_product(self_250: RadialPoint, other_212: Scalar) -> RadialPoint {
    var self_251: RadialPoint;
    var other_213: Scalar;

    self_251 = self_250;
    other_213 = other_212;
    let _e4: RadialPoint = self_251;
    let _e6: Scalar = other_213;
    let _e10: RadialPoint = self_251;
    let _e12: Scalar = other_213;
    return RadialPoint((_e4.g0_ * vec3<f32>(_e6.g0_)), (_e10.g1_ * vec2<f32>(_e12.g0_)));
}

fn radial_point_scalar_right_contraction(self_252: RadialPoint, other_214: Scalar) -> RadialPoint {
    var self_253: RadialPoint;
    var other_215: Scalar;

    self_253 = self_252;
    other_215 = other_214;
    let _e4: RadialPoint = self_253;
    let _e6: Scalar = other_215;
    let _e10: RadialPoint = self_253;
    let _e12: Scalar = other_215;
    return RadialPoint((_e4.g0_ * vec3<f32>(_e6.g0_)), (_e10.g1_ * vec2<f32>(_e12.g0_)));
}

fn radial_point_anti_scalar_regressive_product(self_254: RadialPoint, other_216: AntiScalar) -> RadialPoint {
    var self_255: RadialPoint;
    var other_217: AntiScalar;

    self_255 = self_254;
    other_217 = other_216;
    let _e4: RadialPoint = self_255;
    let _e6: AntiScalar = other_217;
    let _e10: RadialPoint = self_255;
    let _e12: AntiScalar = other_217;
    return RadialPoint((_e4.g0_ * vec3<f32>(_e6.g0_)), (_e10.g1_ * vec2<f32>(_e12.g0_)));
}

fn radial_point_radial_point_add(self_256: RadialPoint, other_218: RadialPoint) -> RadialPoint {
    var self_257: RadialPoint;
    var other_219: RadialPoint;

    self_257 = self_256;
    other_219 = other_218;
    let _e4: RadialPoint = self_257;
    let _e6: RadialPoint = other_219;
    let _e9: RadialPoint = self_257;
    let _e11: RadialPoint = other_219;
    return RadialPoint((_e4.g0_ + _e6.g0_), (_e9.g1_ + _e11.g1_));
}

fn radial_point_radial_point_sub(self_258: RadialPoint, other_220: RadialPoint) -> RadialPoint {
    var self_259: RadialPoint;
    var other_221: RadialPoint;

    self_259 = self_258;
    other_221 = other_220;
    let _e4: RadialPoint = self_259;
    let _e6: RadialPoint = other_221;
    let _e9: RadialPoint = self_259;
    let _e11: RadialPoint = other_221;
    return RadialPoint((_e4.g0_ - _e6.g0_), (_e9.g1_ - _e11.g1_));
}

fn radial_point_radial_point_mul(self_260: RadialPoint, other_222: RadialPoint) -> RadialPoint {
    var self_261: RadialPoint;
    var other_223: RadialPoint;

    self_261 = self_260;
    other_223 = other_222;
    let _e4: RadialPoint = self_261;
    let _e6: RadialPoint = other_223;
    let _e9: RadialPoint = self_261;
    let _e11: RadialPoint = other_223;
    return RadialPoint((_e4.g0_ * _e6.g0_), (_e9.g1_ * _e11.g1_));
}

fn radial_point_radial_point_div(self_262: RadialPoint, other_224: RadialPoint) -> RadialPoint {
    var self_263: RadialPoint;
    var other_225: RadialPoint;

    self_263 = self_262;
    other_225 = other_224;
    let _e4: RadialPoint = self_263;
    let _e7: RadialPoint = self_263;
    let _e10: RadialPoint = self_263;
    let _e19: RadialPoint = other_225;
    let _e22: RadialPoint = other_225;
    let _e25: RadialPoint = other_225;
    let _e35: RadialPoint = self_263;
    let _e38: RadialPoint = self_263;
    let _e46: RadialPoint = other_225;
    let _e49: RadialPoint = other_225;
    return RadialPoint((((vec3<f32>(_e4.g0_.x, _e7.g0_.y, _e10.g0_.z) * vec3<f32>(1.0, 1.0, 1.0)) / vec3<f32>(_e19.g0_.x, _e22.g0_.y, _e25.g0_.z)) * vec3<f32>(1.0, 1.0, 1.0)), (((vec2<f32>(_e35.g1_.x, _e38.g1_.y) * vec2<f32>(1.0, 1.0)) / vec2<f32>(_e46.g1_.x, _e49.g1_.y)) * vec2<f32>(1.0, 1.0)));
}

fn radial_point_radial_point_outer_product(self_264: RadialPoint, other_226: RadialPoint) -> Dipole {
    var self_265: RadialPoint;
    var other_227: RadialPoint;

    self_265 = self_264;
    other_227 = other_226;
    let _e4: RadialPoint = self_265;
    let _e8: RadialPoint = other_227;
    let _e11: RadialPoint = self_265;
    let _e13: RadialPoint = other_227;
    let _e23: RadialPoint = self_265;
    let _e27: RadialPoint = other_227;
    let _e37: RadialPoint = self_265;
    let _e41: RadialPoint = other_227;
    let _e52: RadialPoint = self_265;
    let _e56: RadialPoint = other_227;
    let _e67: RadialPoint = self_265;
    let _e71: RadialPoint = other_227;
    let _e82: RadialPoint = self_265;
    let _e86: RadialPoint = other_227;
    let _e89: RadialPoint = other_227;
    let _e92: RadialPoint = other_227;
    let _e95: RadialPoint = other_227;
    let _e101: RadialPoint = self_265;
    let _e104: RadialPoint = self_265;
    let _e107: RadialPoint = self_265;
    let _e110: RadialPoint = self_265;
    let _e114: RadialPoint = other_227;
    let _e117: RadialPoint = other_227;
    let _e120: RadialPoint = other_227;
    let _e123: RadialPoint = other_227;
    return Dipole(((vec3<f32>(_e4.g1_.x) * _e8.g0_) + ((_e11.g0_ * vec3<f32>(_e13.g1_.x)) * vec3<f32>(-(1.0)))), ((((vec3<f32>(_e23.g0_.y) * _e27.g0_.zzx) * vec3<f32>(1.0, 0.0, -(1.0))) + ((vec3<f32>(_e37.g0_.z) * _e41.g0_.yxy) * vec3<f32>(-(1.0), 1.0, 0.0))) + ((vec3<f32>(_e52.g0_.x) * _e56.g0_.xzy) * vec3<f32>(0.0, -(1.0), 1.0))), ((((vec4<f32>(_e67.g1_.x) * vec4<f32>(_e71.g1_.y)) * vec4<f32>(0.0, 0.0, 0.0, 1.0)) - (vec4<f32>(_e82.g1_.y) * vec4<f32>(_e86.g0_.x, _e89.g0_.y, _e92.g0_.z, _e95.g1_.x))) + ((vec4<f32>(_e101.g0_.x, _e104.g0_.y, _e107.g0_.z, _e110.g0_.x) * vec4<f32>(_e114.g1_.y, _e117.g1_.y, _e120.g1_.y, _e123.g1_.x)) * vec4<f32>(1.0, 1.0, 1.0, 0.0))));
}

fn radial_point_radial_point_inner_product(self_266: RadialPoint, other_228: RadialPoint) -> Scalar {
    var self_267: RadialPoint;
    var other_229: RadialPoint;

    self_267 = self_266;
    other_229 = other_228;
    let _e4: RadialPoint = self_267;
    let _e7: RadialPoint = other_229;
    let _e11: RadialPoint = self_267;
    let _e14: RadialPoint = other_229;
    let _e19: RadialPoint = self_267;
    let _e22: RadialPoint = other_229;
    return Scalar((((_e4.g0_.x * _e7.g0_.x) + (_e11.g0_.y * _e14.g0_.y)) + (_e19.g0_.z * _e22.g0_.z)));
}

fn radial_point_radial_point_left_contraction(self_268: RadialPoint, other_230: RadialPoint) -> Scalar {
    var self_269: RadialPoint;
    var other_231: RadialPoint;

    self_269 = self_268;
    other_231 = other_230;
    let _e4: RadialPoint = self_269;
    let _e7: RadialPoint = other_231;
    let _e11: RadialPoint = self_269;
    let _e14: RadialPoint = other_231;
    let _e19: RadialPoint = self_269;
    let _e22: RadialPoint = other_231;
    return Scalar((((_e4.g0_.x * _e7.g0_.x) + (_e11.g0_.y * _e14.g0_.y)) + (_e19.g0_.z * _e22.g0_.z)));
}

fn radial_point_radial_point_right_contraction(self_270: RadialPoint, other_232: RadialPoint) -> Scalar {
    var self_271: RadialPoint;
    var other_233: RadialPoint;

    self_271 = self_270;
    other_233 = other_232;
    let _e4: RadialPoint = self_271;
    let _e7: RadialPoint = other_233;
    let _e11: RadialPoint = self_271;
    let _e14: RadialPoint = other_233;
    let _e19: RadialPoint = self_271;
    let _e22: RadialPoint = other_233;
    return Scalar((((_e4.g0_.x * _e7.g0_.x) + (_e11.g0_.y * _e14.g0_.y)) + (_e19.g0_.z * _e22.g0_.z)));
}

fn radial_point_radial_point_scalar_product(self_272: RadialPoint, other_234: RadialPoint) -> Scalar {
    var self_273: RadialPoint;
    var other_235: RadialPoint;

    self_273 = self_272;
    other_235 = other_234;
    let _e4: RadialPoint = self_273;
    let _e7: RadialPoint = other_235;
    let _e11: RadialPoint = self_273;
    let _e14: RadialPoint = other_235;
    let _e19: RadialPoint = self_273;
    let _e22: RadialPoint = other_235;
    return Scalar((((_e4.g0_.x * _e7.g0_.x) + (_e11.g0_.y * _e14.g0_.y)) + (_e19.g0_.z * _e22.g0_.z)));
}

fn radial_point_flat_point_outer_product(self_274: RadialPoint, other_236: FlatPoint) -> Line {
    var self_275: RadialPoint;
    var other_237: FlatPoint;

    self_275 = self_274;
    other_237 = other_236;
    let _e4: RadialPoint = self_275;
    let _e8: FlatPoint = other_237;
    let _e11: FlatPoint = other_237;
    let _e14: FlatPoint = other_237;
    let _e19: RadialPoint = self_275;
    let _e21: FlatPoint = other_237;
    let _e31: RadialPoint = self_275;
    let _e35: FlatPoint = other_237;
    let _e38: FlatPoint = other_237;
    let _e41: FlatPoint = other_237;
    let _e52: RadialPoint = self_275;
    let _e56: FlatPoint = other_237;
    let _e59: FlatPoint = other_237;
    let _e62: FlatPoint = other_237;
    let _e74: RadialPoint = self_275;
    let _e78: FlatPoint = other_237;
    let _e81: FlatPoint = other_237;
    let _e84: FlatPoint = other_237;
    return Line(((vec3<f32>(_e4.g1_.x) * vec3<f32>(_e8.g0_.x, _e11.g0_.y, _e14.g0_.z)) + ((_e19.g0_ * vec3<f32>(_e21.g0_.w)) * vec3<f32>(-(1.0)))), ((((vec3<f32>(_e31.g0_.y) * vec3<f32>(_e35.g0_.z, _e38.g0_.z, _e41.g0_.x)) * vec3<f32>(1.0, 0.0, -(1.0))) + ((vec3<f32>(_e52.g0_.z) * vec3<f32>(_e56.g0_.y, _e59.g0_.x, _e62.g0_.y)) * vec3<f32>(-(1.0), 1.0, 0.0))) + ((vec3<f32>(_e74.g0_.x) * vec3<f32>(_e78.g0_.x, _e81.g0_.z, _e84.g0_.y)) * vec3<f32>(0.0, -(1.0), 1.0))));
}

fn radial_point_dipole_outer_product(self_276: RadialPoint, other_238: Dipole) -> Circle {
    var self_277: RadialPoint;
    var other_239: Dipole;

    self_277 = self_276;
    other_239 = other_238;
    let _e4: RadialPoint = self_277;
    let _e8: Dipole = other_239;
    let _e11: Dipole = other_239;
    let _e14: Dipole = other_239;
    let _e17: Dipole = other_239;
    let _e30: RadialPoint = self_277;
    let _e34: Dipole = other_239;
    let _e37: Dipole = other_239;
    let _e40: Dipole = other_239;
    let _e43: Dipole = other_239;
    let _e57: RadialPoint = self_277;
    let _e61: Dipole = other_239;
    let _e64: Dipole = other_239;
    let _e67: Dipole = other_239;
    let _e70: Dipole = other_239;
    let _e82: RadialPoint = self_277;
    let _e86: Dipole = other_239;
    let _e89: Dipole = other_239;
    let _e92: Dipole = other_239;
    let _e95: Dipole = other_239;
    let _e109: RadialPoint = self_277;
    let _e113: Dipole = other_239;
    let _e116: Dipole = other_239;
    let _e119: Dipole = other_239;
    let _e124: RadialPoint = self_277;
    let _e128: Dipole = other_239;
    let _e132: RadialPoint = self_277;
    let _e134: Dipole = other_239;
    let _e144: RadialPoint = self_277;
    let _e148: Dipole = other_239;
    let _e151: Dipole = other_239;
    let _e154: Dipole = other_239;
    let _e165: RadialPoint = self_277;
    let _e169: Dipole = other_239;
    let _e172: Dipole = other_239;
    let _e175: Dipole = other_239;
    let _e187: RadialPoint = self_277;
    let _e191: Dipole = other_239;
    let _e195: RadialPoint = self_277;
    let _e199: Dipole = other_239;
    let _e202: Dipole = other_239;
    let _e205: Dipole = other_239;
    return Circle((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.z, _e11.g0_.z, _e14.g0_.x, _e17.g1_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e30.g0_.z) * vec4<f32>(_e34.g0_.y, _e37.g0_.x, _e40.g0_.y, _e43.g1_.z)) * vec4<f32>(1.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e57.g1_.x) * vec4<f32>(_e61.g1_.x, _e64.g1_.y, _e67.g1_.z, _e70.g1_.x)) * vec4<f32>(1.0, 1.0, 1.0, 0.0))) + ((vec4<f32>(_e82.g0_.x) * vec4<f32>(_e86.g0_.x, _e89.g0_.z, _e92.g0_.y, _e95.g1_.x)) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))), (((vec3<f32>(_e109.g1_.x) * vec3<f32>(_e113.g2_.x, _e116.g2_.y, _e119.g2_.z)) + (vec3<f32>(_e124.g1_.y) * _e128.g0_)) + ((_e132.g0_ * vec3<f32>(_e134.g2_.w)) * vec3<f32>(-(1.0)))), (((((vec3<f32>(_e144.g0_.y) * vec3<f32>(_e148.g2_.z, _e151.g2_.z, _e154.g2_.x)) * vec3<f32>(1.0, 0.0, -(1.0))) + ((vec3<f32>(_e165.g0_.z) * vec3<f32>(_e169.g2_.y, _e172.g2_.x, _e175.g2_.y)) * vec3<f32>(-(1.0), 1.0, 0.0))) + (vec3<f32>(_e187.g1_.y) * _e191.g1_)) + ((vec3<f32>(_e195.g0_.x) * vec3<f32>(_e199.g2_.x, _e202.g2_.z, _e205.g2_.y)) * vec3<f32>(0.0, -(1.0), 1.0))));
}

fn radial_point_dipole_inner_product(self_278: RadialPoint, other_240: Dipole) -> RadialPoint {
    var self_279: RadialPoint;
    var other_241: Dipole;

    self_279 = self_278;
    other_241 = other_240;
    let _e4: RadialPoint = self_279;
    let _e8: Dipole = other_241;
    let _e18: RadialPoint = self_279;
    let _e22: Dipole = other_241;
    let _e33: RadialPoint = self_279;
    let _e37: Dipole = other_241;
    let _e48: RadialPoint = self_279;
    let _e52: Dipole = other_241;
    let _e55: Dipole = other_241;
    let _e65: RadialPoint = self_279;
    let _e69: Dipole = other_241;
    let _e72: Dipole = other_241;
    let _e83: RadialPoint = self_279;
    let _e87: Dipole = other_241;
    let _e90: Dipole = other_241;
    return RadialPoint(((((vec3<f32>(_e4.g0_.y) * _e8.g1_.zzx) * vec3<f32>(-(1.0), 0.0, 1.0)) + ((vec3<f32>(_e18.g0_.z) * _e22.g1_.yxy) * vec3<f32>(1.0, -(1.0), 0.0))) + ((vec3<f32>(_e33.g0_.x) * _e37.g1_.xzy) * vec3<f32>(0.0, 1.0, -(1.0)))), ((((vec2<f32>(_e48.g0_.x) * vec2<f32>(_e52.g0_.x, _e55.g2_.x)) * vec2<f32>(-(1.0), 1.0)) + ((vec2<f32>(_e65.g0_.y) * vec2<f32>(_e69.g0_.y, _e72.g2_.y)) * vec2<f32>(-(1.0), 1.0))) + ((vec2<f32>(_e83.g0_.z) * vec2<f32>(_e87.g0_.z, _e90.g2_.z)) * vec2<f32>(-(1.0), 1.0))));
}

fn radial_point_dipole_left_contraction(self_280: RadialPoint, other_242: Dipole) -> RadialPoint {
    var self_281: RadialPoint;
    var other_243: Dipole;

    self_281 = self_280;
    other_243 = other_242;
    let _e4: RadialPoint = self_281;
    let _e8: Dipole = other_243;
    let _e18: RadialPoint = self_281;
    let _e22: Dipole = other_243;
    let _e33: RadialPoint = self_281;
    let _e37: Dipole = other_243;
    let _e48: RadialPoint = self_281;
    let _e52: Dipole = other_243;
    let _e55: Dipole = other_243;
    let _e65: RadialPoint = self_281;
    let _e69: Dipole = other_243;
    let _e72: Dipole = other_243;
    let _e83: RadialPoint = self_281;
    let _e87: Dipole = other_243;
    let _e90: Dipole = other_243;
    return RadialPoint(((((vec3<f32>(_e4.g0_.y) * _e8.g1_.zzx) * vec3<f32>(-(1.0), 0.0, 1.0)) + ((vec3<f32>(_e18.g0_.z) * _e22.g1_.yxy) * vec3<f32>(1.0, -(1.0), 0.0))) + ((vec3<f32>(_e33.g0_.x) * _e37.g1_.xzy) * vec3<f32>(0.0, 1.0, -(1.0)))), ((((vec2<f32>(_e48.g0_.x) * vec2<f32>(_e52.g0_.x, _e55.g2_.x)) * vec2<f32>(-(1.0), 1.0)) + ((vec2<f32>(_e65.g0_.y) * vec2<f32>(_e69.g0_.y, _e72.g2_.y)) * vec2<f32>(-(1.0), 1.0))) + ((vec2<f32>(_e83.g0_.z) * vec2<f32>(_e87.g0_.z, _e90.g2_.z)) * vec2<f32>(-(1.0), 1.0))));
}

fn radial_point_line_geometric_product(self_282: RadialPoint, other_244: Line) -> Flector {
    var self_283: RadialPoint;
    var other_245: Line;

    self_283 = self_282;
    other_245 = other_244;
    let _e4: RadialPoint = self_283;
    let _e8: Line = other_245;
    let _e11: Line = other_245;
    let _e14: Line = other_245;
    let _e17: Line = other_245;
    let _e30: RadialPoint = self_283;
    let _e34: Line = other_245;
    let _e37: Line = other_245;
    let _e40: Line = other_245;
    let _e43: Line = other_245;
    let _e57: RadialPoint = self_283;
    let _e61: Line = other_245;
    let _e64: Line = other_245;
    let _e67: Line = other_245;
    let _e70: Line = other_245;
    let _e84: RadialPoint = self_283;
    let _e88: Line = other_245;
    let _e91: Line = other_245;
    let _e94: Line = other_245;
    let _e97: Line = other_245;
    let _e110: RadialPoint = self_283;
    let _e114: Line = other_245;
    let _e117: Line = other_245;
    let _e120: Line = other_245;
    let _e123: Line = other_245;
    let _e137: RadialPoint = self_283;
    let _e141: Line = other_245;
    let _e144: Line = other_245;
    let _e147: Line = other_245;
    let _e150: Line = other_245;
    let _e162: RadialPoint = self_283;
    let _e166: Line = other_245;
    let _e169: Line = other_245;
    let _e172: Line = other_245;
    let _e175: Line = other_245;
    return Flector(((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g1_.z, _e11.g1_.z, _e14.g1_.x, _e17.g0_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e30.g0_.z) * vec4<f32>(_e34.g1_.y, _e37.g1_.x, _e40.g1_.y, _e43.g0_.z)) * vec4<f32>(1.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e57.g0_.x) * vec4<f32>(_e61.g1_.x, _e64.g1_.z, _e67.g1_.y, _e70.g0_.x)) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))), (((((vec4<f32>(_e84.g0_.y) * vec4<f32>(_e88.g0_.z, _e91.g0_.z, _e94.g0_.x, _e97.g1_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e110.g0_.z) * vec4<f32>(_e114.g0_.y, _e117.g0_.x, _e120.g0_.y, _e123.g1_.z)) * vec4<f32>(1.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e137.g1_.x) * vec4<f32>(_e141.g1_.x, _e144.g1_.y, _e147.g1_.z, _e150.g1_.x)) * vec4<f32>(1.0, 1.0, 1.0, 0.0))) + ((vec4<f32>(_e162.g0_.x) * vec4<f32>(_e166.g0_.x, _e169.g0_.z, _e172.g0_.y, _e175.g1_.x)) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))));
}

fn radial_point_line_outer_product(self_284: RadialPoint, other_246: Line) -> Plane {
    var self_285: RadialPoint;
    var other_247: Line;

    self_285 = self_284;
    other_247 = other_246;
    let _e4: RadialPoint = self_285;
    let _e8: Line = other_247;
    let _e11: Line = other_247;
    let _e14: Line = other_247;
    let _e17: Line = other_247;
    let _e30: RadialPoint = self_285;
    let _e34: Line = other_247;
    let _e37: Line = other_247;
    let _e40: Line = other_247;
    let _e43: Line = other_247;
    let _e57: RadialPoint = self_285;
    let _e61: Line = other_247;
    let _e64: Line = other_247;
    let _e67: Line = other_247;
    let _e70: Line = other_247;
    let _e82: RadialPoint = self_285;
    let _e86: Line = other_247;
    let _e89: Line = other_247;
    let _e92: Line = other_247;
    let _e95: Line = other_247;
    return Plane((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.z, _e11.g0_.z, _e14.g0_.x, _e17.g1_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e30.g0_.z) * vec4<f32>(_e34.g0_.y, _e37.g0_.x, _e40.g0_.y, _e43.g1_.z)) * vec4<f32>(1.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e57.g1_.x) * vec4<f32>(_e61.g1_.x, _e64.g1_.y, _e67.g1_.z, _e70.g1_.x)) * vec4<f32>(1.0, 1.0, 1.0, 0.0))) + ((vec4<f32>(_e82.g0_.x) * vec4<f32>(_e86.g0_.x, _e89.g0_.z, _e92.g0_.y, _e95.g1_.x)) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))));
}

fn radial_point_line_inner_product(self_286: RadialPoint, other_248: Line) -> FlatPoint {
    var self_287: RadialPoint;
    var other_249: Line;

    self_287 = self_286;
    other_249 = other_248;
    let _e4: RadialPoint = self_287;
    let _e8: Line = other_249;
    let _e11: Line = other_249;
    let _e14: Line = other_249;
    let _e17: Line = other_249;
    let _e30: RadialPoint = self_287;
    let _e34: Line = other_249;
    let _e37: Line = other_249;
    let _e40: Line = other_249;
    let _e43: Line = other_249;
    let _e57: RadialPoint = self_287;
    let _e61: Line = other_249;
    let _e64: Line = other_249;
    let _e67: Line = other_249;
    let _e70: Line = other_249;
    return FlatPoint(((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g1_.z, _e11.g1_.z, _e14.g1_.x, _e17.g0_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e30.g0_.z) * vec4<f32>(_e34.g1_.y, _e37.g1_.x, _e40.g1_.y, _e43.g0_.z)) * vec4<f32>(1.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e57.g0_.x) * vec4<f32>(_e61.g1_.x, _e64.g1_.z, _e67.g1_.y, _e70.g0_.x)) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))));
}

fn radial_point_line_left_contraction(self_288: RadialPoint, other_250: Line) -> FlatPoint {
    var self_289: RadialPoint;
    var other_251: Line;

    self_289 = self_288;
    other_251 = other_250;
    let _e4: RadialPoint = self_289;
    let _e8: Line = other_251;
    let _e11: Line = other_251;
    let _e14: Line = other_251;
    let _e17: Line = other_251;
    let _e30: RadialPoint = self_289;
    let _e34: Line = other_251;
    let _e37: Line = other_251;
    let _e40: Line = other_251;
    let _e43: Line = other_251;
    let _e57: RadialPoint = self_289;
    let _e61: Line = other_251;
    let _e64: Line = other_251;
    let _e67: Line = other_251;
    let _e70: Line = other_251;
    return FlatPoint(((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g1_.z, _e11.g1_.z, _e14.g1_.x, _e17.g0_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e30.g0_.z) * vec4<f32>(_e34.g1_.y, _e37.g1_.x, _e40.g1_.y, _e43.g0_.z)) * vec4<f32>(1.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e57.g0_.x) * vec4<f32>(_e61.g1_.x, _e64.g1_.z, _e67.g1_.y, _e70.g0_.x)) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))));
}

fn radial_point_circle_outer_product(self_290: RadialPoint, other_252: Circle) -> Sphere {
    var self_291: RadialPoint;
    var other_253: Circle;

    self_291 = self_290;
    other_253 = other_252;
    let _e4: RadialPoint = self_291;
    let _e8: Circle = other_253;
    let _e11: Circle = other_253;
    let _e21: RadialPoint = self_291;
    let _e25: Circle = other_253;
    let _e28: Circle = other_253;
    let _e39: RadialPoint = self_291;
    let _e43: Circle = other_253;
    let _e46: Circle = other_253;
    let _e57: RadialPoint = self_291;
    let _e59: Circle = other_253;
    let _e70: RadialPoint = self_291;
    let _e74: Circle = other_253;
    let _e84: RadialPoint = self_291;
    let _e88: Circle = other_253;
    let _e99: RadialPoint = self_291;
    let _e103: Circle = other_253;
    let _e107: RadialPoint = self_291;
    let _e111: Circle = other_253;
    let _e114: Circle = other_253;
    let _e117: Circle = other_253;
    let _e123: RadialPoint = self_291;
    let _e127: Circle = other_253;
    return Sphere((((((vec2<f32>(_e4.g0_.x) * vec2<f32>(_e8.g0_.x, _e11.g2_.x)) * vec2<f32>(1.0, -(1.0))) + ((vec2<f32>(_e21.g0_.y) * vec2<f32>(_e25.g0_.y, _e28.g2_.y)) * vec2<f32>(1.0, -(1.0)))) + ((vec2<f32>(_e39.g0_.z) * vec2<f32>(_e43.g0_.z, _e46.g2_.z)) * vec2<f32>(1.0, -(1.0)))) + ((_e57.g1_ * vec2<f32>(_e59.g0_.w)) * vec2<f32>(1.0, -(1.0)))), ((((((vec3<f32>(_e70.g0_.y) * _e74.g1_.zzx) * vec3<f32>(-(1.0), 0.0, 1.0)) + ((vec3<f32>(_e84.g0_.z) * _e88.g1_.yxy) * vec3<f32>(1.0, -(1.0), 0.0))) + (vec3<f32>(_e99.g1_.x) * _e103.g2_)) - (vec3<f32>(_e107.g1_.y) * vec3<f32>(_e111.g0_.x, _e114.g0_.y, _e117.g0_.z))) + ((vec3<f32>(_e123.g0_.x) * _e127.g1_.xzy) * vec3<f32>(0.0, 1.0, -(1.0)))));
}

fn radial_point_circle_inner_product(self_292: RadialPoint, other_254: Circle) -> Dipole {
    var self_293: RadialPoint;
    var other_255: Circle;

    self_293 = self_292;
    other_255 = other_254;
    let _e4: RadialPoint = self_293;
    let _e8: Circle = other_255;
    let _e11: Circle = other_255;
    let _e14: Circle = other_255;
    let _e25: RadialPoint = self_293;
    let _e29: Circle = other_255;
    let _e32: Circle = other_255;
    let _e35: Circle = other_255;
    let _e47: RadialPoint = self_293;
    let _e51: Circle = other_255;
    let _e54: Circle = other_255;
    let _e57: Circle = other_255;
    let _e69: RadialPoint = self_293;
    let _e71: Circle = other_255;
    let _e80: RadialPoint = self_293;
    let _e84: Circle = other_255;
    let _e87: Circle = other_255;
    let _e90: Circle = other_255;
    let _e93: Circle = other_255;
    let _e106: RadialPoint = self_293;
    let _e110: Circle = other_255;
    let _e113: Circle = other_255;
    let _e116: Circle = other_255;
    let _e119: Circle = other_255;
    let _e133: RadialPoint = self_293;
    let _e137: Circle = other_255;
    let _e140: Circle = other_255;
    let _e143: Circle = other_255;
    let _e146: Circle = other_255;
    return Dipole(((((vec3<f32>(_e4.g0_.y) * vec3<f32>(_e8.g0_.z, _e11.g0_.z, _e14.g0_.x)) * vec3<f32>(1.0, 0.0, -(1.0))) + ((vec3<f32>(_e25.g0_.z) * vec3<f32>(_e29.g0_.y, _e32.g0_.x, _e35.g0_.y)) * vec3<f32>(-(1.0), 1.0, 0.0))) + ((vec3<f32>(_e47.g0_.x) * vec3<f32>(_e51.g0_.x, _e54.g0_.z, _e57.g0_.y)) * vec3<f32>(0.0, -(1.0), 1.0))), ((_e69.g0_ * vec3<f32>(_e71.g0_.w)) * vec3<f32>(-(1.0))), ((((vec4<f32>(_e80.g0_.y) * vec4<f32>(_e84.g2_.z, _e87.g2_.z, _e90.g2_.x, _e93.g1_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e106.g0_.z) * vec4<f32>(_e110.g2_.y, _e113.g2_.x, _e116.g2_.y, _e119.g1_.z)) * vec4<f32>(1.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e133.g0_.x) * vec4<f32>(_e137.g2_.x, _e140.g2_.z, _e143.g2_.y, _e146.g1_.x)) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))));
}

fn radial_point_circle_left_contraction(self_294: RadialPoint, other_256: Circle) -> Dipole {
    var self_295: RadialPoint;
    var other_257: Circle;

    self_295 = self_294;
    other_257 = other_256;
    let _e4: RadialPoint = self_295;
    let _e8: Circle = other_257;
    let _e11: Circle = other_257;
    let _e14: Circle = other_257;
    let _e25: RadialPoint = self_295;
    let _e29: Circle = other_257;
    let _e32: Circle = other_257;
    let _e35: Circle = other_257;
    let _e47: RadialPoint = self_295;
    let _e51: Circle = other_257;
    let _e54: Circle = other_257;
    let _e57: Circle = other_257;
    let _e69: RadialPoint = self_295;
    let _e71: Circle = other_257;
    let _e80: RadialPoint = self_295;
    let _e84: Circle = other_257;
    let _e87: Circle = other_257;
    let _e90: Circle = other_257;
    let _e93: Circle = other_257;
    let _e106: RadialPoint = self_295;
    let _e110: Circle = other_257;
    let _e113: Circle = other_257;
    let _e116: Circle = other_257;
    let _e119: Circle = other_257;
    let _e133: RadialPoint = self_295;
    let _e137: Circle = other_257;
    let _e140: Circle = other_257;
    let _e143: Circle = other_257;
    let _e146: Circle = other_257;
    return Dipole(((((vec3<f32>(_e4.g0_.y) * vec3<f32>(_e8.g0_.z, _e11.g0_.z, _e14.g0_.x)) * vec3<f32>(1.0, 0.0, -(1.0))) + ((vec3<f32>(_e25.g0_.z) * vec3<f32>(_e29.g0_.y, _e32.g0_.x, _e35.g0_.y)) * vec3<f32>(-(1.0), 1.0, 0.0))) + ((vec3<f32>(_e47.g0_.x) * vec3<f32>(_e51.g0_.x, _e54.g0_.z, _e57.g0_.y)) * vec3<f32>(0.0, -(1.0), 1.0))), ((_e69.g0_ * vec3<f32>(_e71.g0_.w)) * vec3<f32>(-(1.0))), ((((vec4<f32>(_e80.g0_.y) * vec4<f32>(_e84.g2_.z, _e87.g2_.z, _e90.g2_.x, _e93.g1_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e106.g0_.z) * vec4<f32>(_e110.g2_.y, _e113.g2_.x, _e116.g2_.y, _e119.g1_.z)) * vec4<f32>(1.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e133.g0_.x) * vec4<f32>(_e137.g2_.x, _e140.g2_.z, _e143.g2_.y, _e146.g1_.x)) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))));
}

fn radial_point_plane_regressive_product(self_296: RadialPoint, other_258: Plane) -> Scalar {
    var self_297: RadialPoint;
    var other_259: Plane;

    self_297 = self_296;
    other_259 = other_258;
    let _e4: RadialPoint = self_297;
    let _e7: Plane = other_259;
    let _e11: RadialPoint = self_297;
    let _e14: Plane = other_259;
    let _e19: RadialPoint = self_297;
    let _e22: Plane = other_259;
    let _e27: RadialPoint = self_297;
    let _e30: Plane = other_259;
    return Scalar(((((_e4.g0_.x * _e7.g0_.x) + (_e11.g0_.y * _e14.g0_.y)) + (_e19.g0_.z * _e22.g0_.z)) + (_e27.g1_.x * _e30.g0_.w)));
}

fn radial_point_plane_outer_product(self_298: RadialPoint, other_260: Plane) -> AntiScalar {
    var self_299: RadialPoint;
    var other_261: Plane;

    self_299 = self_298;
    other_261 = other_260;
    let _e4: RadialPoint = self_299;
    let _e7: Plane = other_261;
    let _e11: RadialPoint = self_299;
    let _e14: Plane = other_261;
    let _e19: RadialPoint = self_299;
    let _e22: Plane = other_261;
    let _e27: RadialPoint = self_299;
    let _e30: Plane = other_261;
    return AntiScalar(((((_e4.g0_.x * _e7.g0_.x) + (_e11.g0_.y * _e14.g0_.y)) + (_e19.g0_.z * _e22.g0_.z)) + (_e27.g1_.x * _e30.g0_.w)));
}

fn radial_point_plane_inner_product(self_300: RadialPoint, other_262: Plane) -> Line {
    var self_301: RadialPoint;
    var other_263: Plane;

    self_301 = self_300;
    other_263 = other_262;
    let _e4: RadialPoint = self_301;
    let _e8: Plane = other_263;
    let _e11: Plane = other_263;
    let _e14: Plane = other_263;
    let _e25: RadialPoint = self_301;
    let _e29: Plane = other_263;
    let _e32: Plane = other_263;
    let _e35: Plane = other_263;
    let _e47: RadialPoint = self_301;
    let _e51: Plane = other_263;
    let _e54: Plane = other_263;
    let _e57: Plane = other_263;
    let _e69: RadialPoint = self_301;
    let _e71: Plane = other_263;
    return Line(((((vec3<f32>(_e4.g0_.y) * vec3<f32>(_e8.g0_.z, _e11.g0_.z, _e14.g0_.x)) * vec3<f32>(1.0, 0.0, -(1.0))) + ((vec3<f32>(_e25.g0_.z) * vec3<f32>(_e29.g0_.y, _e32.g0_.x, _e35.g0_.y)) * vec3<f32>(-(1.0), 1.0, 0.0))) + ((vec3<f32>(_e47.g0_.x) * vec3<f32>(_e51.g0_.x, _e54.g0_.z, _e57.g0_.y)) * vec3<f32>(0.0, -(1.0), 1.0))), ((_e69.g0_ * vec3<f32>(_e71.g0_.w)) * vec3<f32>(-(1.0))));
}

fn radial_point_plane_left_contraction(self_302: RadialPoint, other_264: Plane) -> Line {
    var self_303: RadialPoint;
    var other_265: Plane;

    self_303 = self_302;
    other_265 = other_264;
    let _e4: RadialPoint = self_303;
    let _e8: Plane = other_265;
    let _e11: Plane = other_265;
    let _e14: Plane = other_265;
    let _e25: RadialPoint = self_303;
    let _e29: Plane = other_265;
    let _e32: Plane = other_265;
    let _e35: Plane = other_265;
    let _e47: RadialPoint = self_303;
    let _e51: Plane = other_265;
    let _e54: Plane = other_265;
    let _e57: Plane = other_265;
    let _e69: RadialPoint = self_303;
    let _e71: Plane = other_265;
    return Line(((((vec3<f32>(_e4.g0_.y) * vec3<f32>(_e8.g0_.z, _e11.g0_.z, _e14.g0_.x)) * vec3<f32>(1.0, 0.0, -(1.0))) + ((vec3<f32>(_e25.g0_.z) * vec3<f32>(_e29.g0_.y, _e32.g0_.x, _e35.g0_.y)) * vec3<f32>(-(1.0), 1.0, 0.0))) + ((vec3<f32>(_e47.g0_.x) * vec3<f32>(_e51.g0_.x, _e54.g0_.z, _e57.g0_.y)) * vec3<f32>(0.0, -(1.0), 1.0))), ((_e69.g0_ * vec3<f32>(_e71.g0_.w)) * vec3<f32>(-(1.0))));
}

fn radial_point_sphere_regressive_product(self_304: RadialPoint, other_266: Sphere) -> Scalar {
    var self_305: RadialPoint;
    var other_267: Sphere;

    self_305 = self_304;
    other_267 = other_266;
    let _e4: RadialPoint = self_305;
    let _e7: Sphere = other_267;
    let _e11: RadialPoint = self_305;
    let _e14: Sphere = other_267;
    let _e19: RadialPoint = self_305;
    let _e22: Sphere = other_267;
    let _e27: RadialPoint = self_305;
    let _e30: Sphere = other_267;
    let _e35: RadialPoint = self_305;
    let _e38: Sphere = other_267;
    return Scalar((((((_e4.g0_.x * _e7.g1_.x) + (_e11.g0_.y * _e14.g1_.y)) + (_e19.g0_.z * _e22.g1_.z)) + (_e27.g1_.x * _e30.g0_.y)) + (_e35.g1_.y * _e38.g0_.x)));
}

fn radial_point_sphere_outer_product(self_306: RadialPoint, other_268: Sphere) -> AntiScalar {
    var self_307: RadialPoint;
    var other_269: Sphere;

    self_307 = self_306;
    other_269 = other_268;
    let _e4: RadialPoint = self_307;
    let _e7: Sphere = other_269;
    let _e11: RadialPoint = self_307;
    let _e14: Sphere = other_269;
    let _e19: RadialPoint = self_307;
    let _e22: Sphere = other_269;
    let _e27: RadialPoint = self_307;
    let _e30: Sphere = other_269;
    let _e35: RadialPoint = self_307;
    let _e38: Sphere = other_269;
    return AntiScalar((((((_e4.g0_.x * _e7.g1_.x) + (_e11.g0_.y * _e14.g1_.y)) + (_e19.g0_.z * _e22.g1_.z)) + (_e27.g1_.x * _e30.g0_.y)) + (_e35.g1_.y * _e38.g0_.x)));
}

fn radial_point_motor_geometric_product(self_308: RadialPoint, other_270: Motor) -> Flector {
    var self_309: RadialPoint;
    var other_271: Motor;

    self_309 = self_308;
    other_271 = other_270;
    let _e4: RadialPoint = self_309;
    let _e8: Motor = other_271;
    let _e11: Motor = other_271;
    let _e14: Motor = other_271;
    let _e17: Motor = other_271;
    let _e30: RadialPoint = self_309;
    let _e34: Motor = other_271;
    let _e37: Motor = other_271;
    let _e40: Motor = other_271;
    let _e43: Motor = other_271;
    let _e57: RadialPoint = self_309;
    let _e61: Motor = other_271;
    let _e64: Motor = other_271;
    let _e67: Motor = other_271;
    let _e70: Motor = other_271;
    let _e84: RadialPoint = self_309;
    let _e88: Motor = other_271;
    let _e99: RadialPoint = self_309;
    let _e103: Motor = other_271;
    let _e106: Motor = other_271;
    let _e109: Motor = other_271;
    let _e112: Motor = other_271;
    let _e125: RadialPoint = self_309;
    let _e129: Motor = other_271;
    let _e132: Motor = other_271;
    let _e135: Motor = other_271;
    let _e138: Motor = other_271;
    let _e152: RadialPoint = self_309;
    let _e156: Motor = other_271;
    let _e159: Motor = other_271;
    let _e162: Motor = other_271;
    let _e165: Motor = other_271;
    let _e179: RadialPoint = self_309;
    let _e183: Motor = other_271;
    return Flector((((((vec4<f32>(_e4.g0_.x) * vec4<f32>(_e8.g1_.w, _e11.g1_.z, _e14.g1_.y, _e17.g0_.x)) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0))) + ((vec4<f32>(_e30.g0_.y) * vec4<f32>(_e34.g1_.z, _e37.g1_.w, _e40.g1_.x, _e43.g0_.y)) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e57.g0_.z) * vec4<f32>(_e61.g1_.y, _e64.g1_.x, _e67.g1_.w, _e70.g0_.z)) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e84.g1_.x) * _e88.g1_.xxxw) * vec4<f32>(0.0, 0.0, 0.0, 1.0))), (((((vec4<f32>(_e99.g0_.x) * vec4<f32>(_e103.g0_.w, _e106.g0_.z, _e109.g0_.y, _e112.g1_.x)) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0))) + ((vec4<f32>(_e125.g0_.y) * vec4<f32>(_e129.g0_.z, _e132.g0_.w, _e135.g0_.x, _e138.g1_.y)) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e152.g0_.z) * vec4<f32>(_e156.g0_.y, _e159.g0_.x, _e162.g0_.w, _e165.g1_.z)) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e179.g1_.x) * _e183.g1_.xyzx) * vec4<f32>(1.0, 1.0, 1.0, 0.0))));
}

fn radial_point_motor_regressive_product(self_310: RadialPoint, other_272: Motor) -> RadialPoint {
    var self_311: RadialPoint;
    var other_273: Motor;

    self_311 = self_310;
    other_273 = other_272;
    let _e4: RadialPoint = self_311;
    let _e6: Motor = other_273;
    let _e11: RadialPoint = self_311;
    let _e13: Motor = other_273;
    return RadialPoint((_e4.g0_ * vec3<f32>(_e6.g0_.w)), (_e11.g1_ * vec2<f32>(_e13.g0_.w)));
}

fn radial_point_motor_outer_product(self_312: RadialPoint, other_274: Motor) -> Flector {
    var self_313: RadialPoint;
    var other_275: Motor;

    self_313 = self_312;
    other_275 = other_274;
    let _e4: RadialPoint = self_313;
    let _e8: Motor = other_275;
    let _e19: RadialPoint = self_313;
    let _e22: RadialPoint = self_313;
    let _e25: RadialPoint = self_313;
    let _e28: RadialPoint = self_313;
    let _e32: Motor = other_275;
    let _e43: RadialPoint = self_313;
    let _e47: Motor = other_275;
    let _e50: Motor = other_275;
    let _e53: Motor = other_275;
    let _e56: Motor = other_275;
    let _e69: RadialPoint = self_313;
    let _e73: Motor = other_275;
    let _e76: Motor = other_275;
    let _e79: Motor = other_275;
    let _e82: Motor = other_275;
    let _e96: RadialPoint = self_313;
    let _e100: Motor = other_275;
    let _e111: RadialPoint = self_313;
    let _e115: Motor = other_275;
    let _e118: Motor = other_275;
    let _e121: Motor = other_275;
    let _e124: Motor = other_275;
    return Flector((((vec4<f32>(_e4.g1_.x) * vec4<f32>(_e8.g1_.w)) * vec4<f32>(0.0, 0.0, 0.0, 1.0)) + ((vec4<f32>(_e19.g0_.x, _e22.g0_.y, _e25.g0_.z, _e28.g0_.x) * _e32.g1_.wwwx) * vec4<f32>(1.0, 1.0, 1.0, 0.0))), (((((vec4<f32>(_e43.g0_.y) * vec4<f32>(_e47.g0_.z, _e50.g0_.z, _e53.g0_.x, _e56.g1_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e69.g0_.z) * vec4<f32>(_e73.g0_.y, _e76.g0_.x, _e79.g0_.y, _e82.g1_.z)) * vec4<f32>(1.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e96.g1_.x) * _e100.g1_.xyzx) * vec4<f32>(1.0, 1.0, 1.0, 0.0))) + ((vec4<f32>(_e111.g0_.x) * vec4<f32>(_e115.g0_.x, _e118.g0_.z, _e121.g0_.y, _e124.g1_.x)) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))));
}

fn radial_point_rotor_regressive_product(self_314: RadialPoint, other_276: Rotor) -> RadialPoint {
    var self_315: RadialPoint;
    var other_277: Rotor;

    self_315 = self_314;
    other_277 = other_276;
    let _e4: RadialPoint = self_315;
    let _e6: Rotor = other_277;
    let _e11: RadialPoint = self_315;
    let _e13: Rotor = other_277;
    return RadialPoint((_e4.g0_ * vec3<f32>(_e6.g0_.w)), (_e11.g1_ * vec2<f32>(_e13.g0_.w)));
}

fn radial_point_translator_regressive_product(self_316: RadialPoint, other_278: Translator) -> RadialPoint {
    var self_317: RadialPoint;
    var other_279: Translator;

    self_317 = self_316;
    other_279 = other_278;
    let _e4: RadialPoint = self_317;
    let _e6: Translator = other_279;
    let _e11: RadialPoint = self_317;
    let _e13: Translator = other_279;
    return RadialPoint((_e4.g0_ * vec3<f32>(_e6.g0_.w)), (_e11.g1_ * vec2<f32>(_e13.g0_.w)));
}

fn radial_point_translator_outer_product(self_318: RadialPoint, other_280: Translator) -> Plane {
    var self_319: RadialPoint;
    var other_281: Translator;

    self_319 = self_318;
    other_281 = other_280;
    let _e4: RadialPoint = self_319;
    let _e8: Translator = other_281;
    let _e20: RadialPoint = self_319;
    let _e24: Translator = other_281;
    let _e37: RadialPoint = self_319;
    let _e41: Translator = other_281;
    let _e52: RadialPoint = self_319;
    let _e56: Translator = other_281;
    return Plane((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0))) + ((vec4<f32>(_e20.g0_.z) * vec4<f32>(_e24.g0_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e37.g1_.x) * _e41.g0_.xyzx) * vec4<f32>(1.0, 1.0, 1.0, 0.0))) + ((vec4<f32>(_e52.g0_.x) * vec4<f32>(_e56.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))));
}

fn radial_point_flector_geometric_product(self_320: RadialPoint, other_282: Flector) -> Motor {
    var self_321: RadialPoint;
    var other_283: Flector;

    self_321 = self_320;
    other_283 = other_282;
    let _e4: RadialPoint = self_321;
    let _e8: Flector = other_283;
    let _e11: Flector = other_283;
    let _e14: Flector = other_283;
    let _e17: Flector = other_283;
    let _e30: RadialPoint = self_321;
    let _e34: Flector = other_283;
    let _e37: Flector = other_283;
    let _e40: Flector = other_283;
    let _e43: Flector = other_283;
    let _e57: RadialPoint = self_321;
    let _e61: Flector = other_283;
    let _e64: Flector = other_283;
    let _e67: Flector = other_283;
    let _e70: Flector = other_283;
    let _e84: RadialPoint = self_321;
    let _e88: Flector = other_283;
    let _e91: Flector = other_283;
    let _e94: Flector = other_283;
    let _e97: Flector = other_283;
    let _e103: RadialPoint = self_321;
    let _e107: Flector = other_283;
    let _e110: Flector = other_283;
    let _e113: Flector = other_283;
    let _e116: Flector = other_283;
    let _e129: RadialPoint = self_321;
    let _e133: Flector = other_283;
    let _e136: Flector = other_283;
    let _e139: Flector = other_283;
    let _e142: Flector = other_283;
    let _e156: RadialPoint = self_321;
    let _e160: Flector = other_283;
    let _e163: Flector = other_283;
    let _e166: Flector = other_283;
    let _e169: Flector = other_283;
    return Motor((((((vec4<f32>(_e4.g0_.x) * vec4<f32>(_e8.g0_.w, _e11.g1_.z, _e14.g1_.y, _e17.g1_.x)) * vec4<f32>(-(1.0), -(1.0), 1.0, 1.0)) + ((vec4<f32>(_e30.g0_.y) * vec4<f32>(_e34.g1_.z, _e37.g0_.w, _e40.g1_.x, _e43.g1_.y)) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e57.g0_.z) * vec4<f32>(_e61.g1_.y, _e64.g1_.x, _e67.g0_.w, _e70.g1_.z)) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))) + (vec4<f32>(_e84.g1_.x) * vec4<f32>(_e88.g0_.x, _e91.g0_.y, _e94.g0_.z, _e97.g1_.w))), ((((vec4<f32>(_e103.g0_.x) * vec4<f32>(_e107.g1_.w, _e110.g0_.z, _e113.g0_.y, _e116.g0_.x)) * vec4<f32>(-(1.0), -(1.0), 1.0, 1.0)) + ((vec4<f32>(_e129.g0_.y) * vec4<f32>(_e133.g0_.z, _e136.g1_.w, _e139.g0_.x, _e142.g0_.y)) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e156.g0_.z) * vec4<f32>(_e160.g0_.y, _e163.g0_.x, _e166.g1_.w, _e169.g0_.z)) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))));
}

fn radial_point_flector_regressive_product(self_322: RadialPoint, other_284: Flector) -> Scalar {
    var self_323: RadialPoint;
    var other_285: Flector;

    self_323 = self_322;
    other_285 = other_284;
    let _e4: RadialPoint = self_323;
    let _e7: Flector = other_285;
    let _e11: RadialPoint = self_323;
    let _e14: Flector = other_285;
    let _e19: RadialPoint = self_323;
    let _e22: Flector = other_285;
    let _e27: RadialPoint = self_323;
    let _e30: Flector = other_285;
    return Scalar(((((_e4.g0_.x * _e7.g1_.x) + (_e11.g0_.y * _e14.g1_.y)) + (_e19.g0_.z * _e22.g1_.z)) + (_e27.g1_.x * _e30.g1_.w)));
}

fn radial_point_multi_vector_add(self_324: RadialPoint, other_286: MultiVector) -> MultiVector {
    var self_325: RadialPoint;
    var other_287: MultiVector;

    self_325 = self_324;
    other_287 = other_286;
    let _e4: MultiVector = other_287;
    let _e6: RadialPoint = self_325;
    let _e8: MultiVector = other_287;
    let _e11: RadialPoint = self_325;
    let _e13: MultiVector = other_287;
    let _e16: MultiVector = other_287;
    let _e18: MultiVector = other_287;
    let _e20: MultiVector = other_287;
    let _e22: MultiVector = other_287;
    let _e24: MultiVector = other_287;
    let _e26: MultiVector = other_287;
    let _e28: MultiVector = other_287;
    return MultiVector(_e4.g0_, (_e6.g0_ + _e8.g1_), (_e11.g1_ + _e13.g2_), _e16.g3_, _e18.g4_, _e20.g5_, _e22.g6_, _e24.g7_, _e26.g8_, _e28.g9_);
}

fn radial_point_multi_vector_sub(self_326: RadialPoint, other_288: MultiVector) -> MultiVector {
    var self_327: RadialPoint;
    var other_289: MultiVector;

    self_327 = self_326;
    other_289 = other_288;
    let _e6: MultiVector = other_289;
    let _e9: RadialPoint = self_327;
    let _e11: MultiVector = other_289;
    let _e14: RadialPoint = self_327;
    let _e16: MultiVector = other_289;
    let _e21: MultiVector = other_289;
    let _e26: MultiVector = other_289;
    let _e31: MultiVector = other_289;
    let _e36: MultiVector = other_289;
    let _e41: MultiVector = other_289;
    let _e46: MultiVector = other_289;
    let _e51: MultiVector = other_289;
    return MultiVector((vec3<f32>(0.0) - _e6.g0_), (_e9.g0_ - _e11.g1_), (_e14.g1_ - _e16.g2_), (vec4<f32>(0.0) - _e21.g3_), (vec3<f32>(0.0) - _e26.g4_), (vec3<f32>(0.0) - _e31.g5_), (vec3<f32>(0.0) - _e36.g6_), (vec3<f32>(0.0) - _e41.g7_), (vec4<f32>(0.0) - _e46.g8_), (vec4<f32>(0.0) - _e51.g9_));
}

fn radial_point_multi_vector_geometric_product(self_328: RadialPoint, other_290: MultiVector) -> MultiVector {
    var self_329: RadialPoint;
    var other_291: MultiVector;

    self_329 = self_328;
    other_291 = other_290;
    let _e4: RadialPoint = self_329;
    let _e8: MultiVector = other_291;
    let _e11: MultiVector = other_291;
    let _e14: MultiVector = other_291;
    let _e19: RadialPoint = self_329;
    let _e23: MultiVector = other_291;
    let _e26: MultiVector = other_291;
    let _e29: MultiVector = other_291;
    let _e35: RadialPoint = self_329;
    let _e39: MultiVector = other_291;
    let _e42: MultiVector = other_291;
    let _e45: MultiVector = other_291;
    let _e51: RadialPoint = self_329;
    let _e55: MultiVector = other_291;
    let _e66: RadialPoint = self_329;
    let _e70: MultiVector = other_291;
    let _e73: MultiVector = other_291;
    let _e76: MultiVector = other_291;
    let _e87: RadialPoint = self_329;
    let _e91: MultiVector = other_291;
    let _e94: MultiVector = other_291;
    let _e97: MultiVector = other_291;
    let _e108: RadialPoint = self_329;
    let _e112: MultiVector = other_291;
    let _e115: MultiVector = other_291;
    let _e118: MultiVector = other_291;
    let _e130: RadialPoint = self_329;
    let _e134: MultiVector = other_291;
    let _e137: MultiVector = other_291;
    let _e140: MultiVector = other_291;
    let _e152: RadialPoint = self_329;
    let _e156: MultiVector = other_291;
    let _e159: MultiVector = other_291;
    let _e169: RadialPoint = self_329;
    let _e173: MultiVector = other_291;
    let _e176: MultiVector = other_291;
    let _e187: RadialPoint = self_329;
    let _e191: MultiVector = other_291;
    let _e194: MultiVector = other_291;
    let _e205: RadialPoint = self_329;
    let _e207: MultiVector = other_291;
    let _e213: RadialPoint = self_329;
    let _e217: MultiVector = other_291;
    let _e220: MultiVector = other_291;
    let _e223: MultiVector = other_291;
    let _e226: MultiVector = other_291;
    let _e239: RadialPoint = self_329;
    let _e243: MultiVector = other_291;
    let _e246: MultiVector = other_291;
    let _e249: MultiVector = other_291;
    let _e252: MultiVector = other_291;
    let _e266: RadialPoint = self_329;
    let _e270: MultiVector = other_291;
    let _e273: MultiVector = other_291;
    let _e276: MultiVector = other_291;
    let _e279: MultiVector = other_291;
    let _e293: RadialPoint = self_329;
    let _e297: MultiVector = other_291;
    let _e300: MultiVector = other_291;
    let _e303: MultiVector = other_291;
    let _e306: MultiVector = other_291;
    let _e312: RadialPoint = self_329;
    let _e316: MultiVector = other_291;
    let _e319: MultiVector = other_291;
    let _e322: MultiVector = other_291;
    let _e325: MultiVector = other_291;
    let _e337: RadialPoint = self_329;
    let _e341: MultiVector = other_291;
    let _e344: MultiVector = other_291;
    let _e347: MultiVector = other_291;
    let _e359: RadialPoint = self_329;
    let _e363: MultiVector = other_291;
    let _e366: MultiVector = other_291;
    let _e369: MultiVector = other_291;
    let _e382: RadialPoint = self_329;
    let _e386: MultiVector = other_291;
    let _e389: MultiVector = other_291;
    let _e392: MultiVector = other_291;
    let _e405: RadialPoint = self_329;
    let _e409: MultiVector = other_291;
    let _e413: RadialPoint = self_329;
    let _e417: MultiVector = other_291;
    let _e420: MultiVector = other_291;
    let _e423: MultiVector = other_291;
    let _e435: RadialPoint = self_329;
    let _e439: MultiVector = other_291;
    let _e442: MultiVector = other_291;
    let _e445: MultiVector = other_291;
    let _e458: RadialPoint = self_329;
    let _e462: MultiVector = other_291;
    let _e465: MultiVector = other_291;
    let _e468: MultiVector = other_291;
    let _e481: RadialPoint = self_329;
    let _e485: MultiVector = other_291;
    let _e488: MultiVector = other_291;
    let _e491: MultiVector = other_291;
    let _e503: RadialPoint = self_329;
    let _e507: MultiVector = other_291;
    let _e510: MultiVector = other_291;
    let _e513: MultiVector = other_291;
    let _e526: RadialPoint = self_329;
    let _e530: MultiVector = other_291;
    let _e533: MultiVector = other_291;
    let _e536: MultiVector = other_291;
    let _e549: RadialPoint = self_329;
    let _e553: MultiVector = other_291;
    let _e556: MultiVector = other_291;
    let _e559: MultiVector = other_291;
    let _e565: RadialPoint = self_329;
    let _e569: MultiVector = other_291;
    let _e573: RadialPoint = self_329;
    let _e577: MultiVector = other_291;
    let _e580: MultiVector = other_291;
    let _e583: MultiVector = other_291;
    let _e595: RadialPoint = self_329;
    let _e599: MultiVector = other_291;
    let _e602: MultiVector = other_291;
    let _e605: MultiVector = other_291;
    let _e618: RadialPoint = self_329;
    let _e622: MultiVector = other_291;
    let _e625: MultiVector = other_291;
    let _e628: MultiVector = other_291;
    let _e641: RadialPoint = self_329;
    let _e645: MultiVector = other_291;
    let _e649: RadialPoint = self_329;
    let _e653: MultiVector = other_291;
    let _e656: MultiVector = other_291;
    let _e659: MultiVector = other_291;
    let _e662: MultiVector = other_291;
    let _e675: RadialPoint = self_329;
    let _e679: MultiVector = other_291;
    let _e682: MultiVector = other_291;
    let _e685: MultiVector = other_291;
    let _e688: MultiVector = other_291;
    let _e702: RadialPoint = self_329;
    let _e706: MultiVector = other_291;
    let _e709: MultiVector = other_291;
    let _e712: MultiVector = other_291;
    let _e715: MultiVector = other_291;
    let _e729: RadialPoint = self_329;
    let _e733: MultiVector = other_291;
    let _e736: MultiVector = other_291;
    let _e739: MultiVector = other_291;
    let _e742: MultiVector = other_291;
    let _e754: RadialPoint = self_329;
    let _e758: MultiVector = other_291;
    let _e761: MultiVector = other_291;
    let _e764: MultiVector = other_291;
    let _e767: MultiVector = other_291;
    let _e780: RadialPoint = self_329;
    let _e784: MultiVector = other_291;
    let _e787: MultiVector = other_291;
    let _e790: MultiVector = other_291;
    let _e793: MultiVector = other_291;
    let _e807: RadialPoint = self_329;
    let _e811: MultiVector = other_291;
    let _e814: MultiVector = other_291;
    let _e817: MultiVector = other_291;
    let _e820: MultiVector = other_291;
    let _e834: RadialPoint = self_329;
    let _e838: MultiVector = other_291;
    let _e842: RadialPoint = self_329;
    let _e846: MultiVector = other_291;
    let _e849: MultiVector = other_291;
    let _e852: MultiVector = other_291;
    let _e855: MultiVector = other_291;
    return MultiVector((((((vec3<f32>(_e4.g0_.x) * vec3<f32>(_e8.g1_.x, _e11.g8_.x, _e14.g9_.x)) + (vec3<f32>(_e19.g0_.y) * vec3<f32>(_e23.g1_.y, _e26.g8_.y, _e29.g9_.y))) + (vec3<f32>(_e35.g0_.z) * vec3<f32>(_e39.g1_.z, _e42.g8_.z, _e45.g9_.z))) + ((vec3<f32>(_e51.g1_.y) * vec3<f32>(_e55.g0_.y)) * vec3<f32>(0.0, 0.0, 1.0))) + ((vec3<f32>(_e66.g1_.x) * vec3<f32>(_e70.g8_.x, _e73.g8_.w, _e76.g9_.w)) * vec3<f32>(0.0, 1.0, 1.0))), ((((vec3<f32>(_e87.g0_.x) * vec3<f32>(_e91.g0_.x, _e94.g5_.z, _e97.g5_.y)) * vec3<f32>(1.0, 1.0, -(1.0))) + ((vec3<f32>(_e108.g0_.y) * vec3<f32>(_e112.g5_.z, _e115.g0_.x, _e118.g5_.x)) * vec3<f32>(-(1.0), 1.0, 1.0))) + ((vec3<f32>(_e130.g0_.z) * vec3<f32>(_e134.g5_.y, _e137.g5_.x, _e140.g0_.x)) * vec3<f32>(1.0, -(1.0), 1.0))), (((((vec2<f32>(_e152.g0_.x) * vec2<f32>(_e156.g4_.x, _e159.g3_.x)) * vec2<f32>(-(1.0), 1.0)) + ((vec2<f32>(_e169.g0_.y) * vec2<f32>(_e173.g4_.y, _e176.g3_.y)) * vec2<f32>(-(1.0), 1.0))) + ((vec2<f32>(_e187.g0_.z) * vec2<f32>(_e191.g4_.z, _e194.g3_.z)) * vec2<f32>(-(1.0), 1.0))) + (_e205.g1_ * vec2<f32>(_e207.g0_.x))), ((((((vec4<f32>(_e213.g0_.x) * vec4<f32>(_e217.g2_.y, _e220.g7_.z, _e223.g7_.y, _e226.g6_.x)) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0))) + ((vec4<f32>(_e239.g0_.y) * vec4<f32>(_e243.g7_.z, _e246.g2_.y, _e249.g7_.x, _e252.g6_.y)) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e266.g0_.z) * vec4<f32>(_e270.g7_.y, _e273.g7_.x, _e276.g2_.y, _e279.g6_.z)) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))) - (vec4<f32>(_e293.g1_.y) * vec4<f32>(_e297.g1_.x, _e300.g1_.y, _e303.g1_.z, _e306.g2_.x))) + ((vec4<f32>(_e312.g1_.x) * vec4<f32>(_e316.g2_.x, _e319.g2_.x, _e322.g2_.x, _e325.g2_.y)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))), (((((vec3<f32>(_e337.g0_.x) * vec3<f32>(_e341.g2_.x, _e344.g8_.z, _e347.g8_.y)) * vec3<f32>(-(1.0), -(1.0), 1.0)) + ((vec3<f32>(_e359.g0_.y) * vec3<f32>(_e363.g8_.z, _e366.g2_.x, _e369.g8_.x)) * vec3<f32>(1.0, -(1.0), -(1.0)))) + ((vec3<f32>(_e382.g0_.z) * vec3<f32>(_e386.g8_.y, _e389.g8_.x, _e392.g2_.x)) * vec3<f32>(-(1.0), 1.0, -(1.0)))) + (vec3<f32>(_e405.g1_.x) * _e409.g1_)), ((((vec3<f32>(_e413.g0_.x) * vec3<f32>(_e417.g8_.w, _e420.g1_.z, _e423.g1_.y)) * vec3<f32>(-(1.0), -(1.0), 1.0)) + ((vec3<f32>(_e435.g0_.y) * vec3<f32>(_e439.g1_.z, _e442.g8_.w, _e445.g1_.x)) * vec3<f32>(1.0, -(1.0), -(1.0)))) + ((vec3<f32>(_e458.g0_.z) * vec3<f32>(_e462.g1_.y, _e465.g1_.x, _e468.g8_.w)) * vec3<f32>(-(1.0), 1.0, -(1.0)))), ((((((vec3<f32>(_e481.g0_.x) * vec3<f32>(_e485.g3_.w, _e488.g9_.z, _e491.g9_.y)) * vec3<f32>(-(1.0), -(1.0), 1.0)) + ((vec3<f32>(_e503.g0_.y) * vec3<f32>(_e507.g9_.z, _e510.g3_.w, _e513.g9_.x)) * vec3<f32>(1.0, -(1.0), -(1.0)))) + ((vec3<f32>(_e526.g0_.z) * vec3<f32>(_e530.g9_.y, _e533.g9_.x, _e536.g3_.w)) * vec3<f32>(-(1.0), 1.0, -(1.0)))) + (vec3<f32>(_e549.g1_.x) * vec3<f32>(_e553.g3_.x, _e556.g3_.y, _e559.g3_.z))) + (vec3<f32>(_e565.g1_.y) * _e569.g4_)), (((((vec3<f32>(_e573.g0_.x) * vec3<f32>(_e577.g9_.w, _e580.g3_.z, _e583.g3_.y)) * vec3<f32>(-(1.0), -(1.0), 1.0)) + ((vec3<f32>(_e595.g0_.y) * vec3<f32>(_e599.g3_.z, _e602.g9_.w, _e605.g3_.x)) * vec3<f32>(1.0, -(1.0), -(1.0)))) + ((vec3<f32>(_e618.g0_.z) * vec3<f32>(_e622.g3_.y, _e625.g3_.x, _e628.g9_.w)) * vec3<f32>(-(1.0), 1.0, -(1.0)))) + (vec3<f32>(_e641.g1_.y) * _e645.g5_)), (((((vec4<f32>(_e649.g0_.x) * vec4<f32>(_e653.g0_.y, _e656.g4_.z, _e659.g4_.y, _e662.g5_.x)) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0))) + ((vec4<f32>(_e675.g0_.y) * vec4<f32>(_e679.g4_.z, _e682.g0_.y, _e685.g4_.x, _e688.g5_.y)) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e702.g0_.z) * vec4<f32>(_e706.g4_.y, _e709.g4_.x, _e712.g0_.y, _e715.g5_.z)) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e729.g1_.x) * vec4<f32>(_e733.g5_.x, _e736.g5_.y, _e739.g5_.z, _e742.g5_.x)) * vec4<f32>(1.0, 1.0, 1.0, 0.0))), ((((((vec4<f32>(_e754.g0_.x) * vec4<f32>(_e758.g0_.z, _e761.g6_.z, _e764.g6_.y, _e767.g7_.x)) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0))) + ((vec4<f32>(_e780.g0_.y) * vec4<f32>(_e784.g6_.z, _e787.g0_.z, _e790.g6_.x, _e793.g7_.y)) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e807.g0_.z) * vec4<f32>(_e811.g6_.y, _e814.g6_.x, _e817.g0_.z, _e820.g7_.z)) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))) - (vec4<f32>(_e834.g1_.y) * _e838.g8_)) + ((vec4<f32>(_e842.g1_.x) * vec4<f32>(_e846.g7_.x, _e849.g7_.y, _e852.g7_.z, _e855.g7_.x)) * vec4<f32>(1.0, 1.0, 1.0, 0.0))));
}

fn radial_point_multi_vector_scalar_product(self_330: RadialPoint, other_292: MultiVector) -> Scalar {
    var self_331: RadialPoint;
    var other_293: MultiVector;

    self_331 = self_330;
    other_293 = other_292;
    let _e4: RadialPoint = self_331;
    let _e7: MultiVector = other_293;
    let _e11: RadialPoint = self_331;
    let _e14: MultiVector = other_293;
    let _e19: RadialPoint = self_331;
    let _e22: MultiVector = other_293;
    return Scalar((((_e4.g0_.x * _e7.g1_.x) + (_e11.g0_.y * _e14.g1_.y)) + (_e19.g0_.z * _e22.g1_.z)));
}

fn radial_point_squared_magnitude(self_332: RadialPoint) -> Scalar {
    var self_333: RadialPoint;

    self_333 = self_332;
    let _e2: RadialPoint = self_333;
    let _e3: RadialPoint = self_333;
    let _e4: RadialPoint = radial_point_reversal(_e3);
    let _e5: Scalar = radial_point_radial_point_scalar_product(_e2, _e4);
    return _e5;
}

fn radial_point_magnitude(self_334: RadialPoint) -> Scalar {
    var self_335: RadialPoint;

    self_335 = self_334;
    let _e2: RadialPoint = self_335;
    let _e3: Scalar = radial_point_squared_magnitude(_e2);
    return Scalar(sqrt(_e3.g0_));
}

fn radial_point_scale(self_336: RadialPoint, other_294: f32) -> RadialPoint {
    var self_337: RadialPoint;
    var other_295: f32;

    self_337 = self_336;
    other_295 = other_294;
    let _e4: RadialPoint = self_337;
    let _e5: f32 = other_295;
    let _e7: RadialPoint = radial_point_scalar_geometric_product(_e4, Scalar(_e5));
    return _e7;
}

fn radial_point_signum(self_338: RadialPoint) -> RadialPoint {
    var self_339: RadialPoint;

    self_339 = self_338;
    let _e2: RadialPoint = self_339;
    let _e3: RadialPoint = self_339;
    let _e4: Scalar = radial_point_magnitude(_e3);
    let _e9: RadialPoint = radial_point_scalar_geometric_product(_e2, Scalar((1.0 / _e4.g0_)));
    return _e9;
}

fn radial_point_inverse(self_340: RadialPoint) -> RadialPoint {
    var self_341: RadialPoint;

    self_341 = self_340;
    let _e2: RadialPoint = self_341;
    let _e3: RadialPoint = radial_point_reversal(_e2);
    let _e4: RadialPoint = self_341;
    let _e5: Scalar = radial_point_squared_magnitude(_e4);
    let _e10: RadialPoint = radial_point_scalar_geometric_product(_e3, Scalar((1.0 / _e5.g0_)));
    return _e10;
}

fn flat_point_zero() -> FlatPoint {
    return FlatPoint(vec4<f32>(0.0));
}

fn flat_point_one() -> FlatPoint {
    return FlatPoint(vec4<f32>(0.0));
}

fn flat_point_neg(self_342: FlatPoint) -> FlatPoint {
    var self_343: FlatPoint;

    self_343 = self_342;
    let _e2: FlatPoint = self_343;
    return FlatPoint((_e2.g0_ * vec4<f32>(-(1.0))));
}

fn flat_point_automorphism(self_344: FlatPoint) -> FlatPoint {
    var self_345: FlatPoint;

    self_345 = self_344;
    let _e2: FlatPoint = self_345;
    return FlatPoint(_e2.g0_);
}

fn flat_point_reversal(self_346: FlatPoint) -> FlatPoint {
    var self_347: FlatPoint;

    self_347 = self_346;
    let _e2: FlatPoint = self_347;
    return FlatPoint((_e2.g0_ * vec4<f32>(-(1.0))));
}

fn flat_point_conjugation(self_348: FlatPoint) -> FlatPoint {
    var self_349: FlatPoint;

    self_349 = self_348;
    let _e2: FlatPoint = self_349;
    return FlatPoint((_e2.g0_ * vec4<f32>(-(1.0))));
}

fn flat_point_scalar_geometric_product(self_350: FlatPoint, other_296: Scalar) -> FlatPoint {
    var self_351: FlatPoint;
    var other_297: Scalar;

    self_351 = self_350;
    other_297 = other_296;
    let _e4: FlatPoint = self_351;
    let _e6: Scalar = other_297;
    return FlatPoint((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn flat_point_scalar_outer_product(self_352: FlatPoint, other_298: Scalar) -> FlatPoint {
    var self_353: FlatPoint;
    var other_299: Scalar;

    self_353 = self_352;
    other_299 = other_298;
    let _e4: FlatPoint = self_353;
    let _e6: Scalar = other_299;
    return FlatPoint((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn flat_point_scalar_inner_product(self_354: FlatPoint, other_300: Scalar) -> FlatPoint {
    var self_355: FlatPoint;
    var other_301: Scalar;

    self_355 = self_354;
    other_301 = other_300;
    let _e4: FlatPoint = self_355;
    let _e6: Scalar = other_301;
    return FlatPoint((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn flat_point_scalar_right_contraction(self_356: FlatPoint, other_302: Scalar) -> FlatPoint {
    var self_357: FlatPoint;
    var other_303: Scalar;

    self_357 = self_356;
    other_303 = other_302;
    let _e4: FlatPoint = self_357;
    let _e6: Scalar = other_303;
    return FlatPoint((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn flat_point_anti_scalar_regressive_product(self_358: FlatPoint, other_304: AntiScalar) -> FlatPoint {
    var self_359: FlatPoint;
    var other_305: AntiScalar;

    self_359 = self_358;
    other_305 = other_304;
    let _e4: FlatPoint = self_359;
    let _e6: AntiScalar = other_305;
    return FlatPoint((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn flat_point_radial_point_outer_product(self_360: FlatPoint, other_306: RadialPoint) -> Line {
    var self_361: FlatPoint;
    var other_307: RadialPoint;

    self_361 = self_360;
    other_307 = other_306;
    let _e6: FlatPoint = self_361;
    let _e10: RadialPoint = other_307;
    let _e14: FlatPoint = self_361;
    let _e17: FlatPoint = self_361;
    let _e20: FlatPoint = self_361;
    let _e24: RadialPoint = other_307;
    let _e30: FlatPoint = self_361;
    let _e34: RadialPoint = other_307;
    let _e44: FlatPoint = self_361;
    let _e48: RadialPoint = other_307;
    let _e59: FlatPoint = self_361;
    let _e63: RadialPoint = other_307;
    return Line(((vec3<f32>(0.0) - (vec3<f32>(_e6.g0_.w) * _e10.g0_)) + (vec3<f32>(_e14.g0_.x, _e17.g0_.y, _e20.g0_.z) * vec3<f32>(_e24.g1_.x))), ((((vec3<f32>(_e30.g0_.y) * _e34.g0_.zzx) * vec3<f32>(-(1.0), 0.0, 1.0)) + ((vec3<f32>(_e44.g0_.z) * _e48.g0_.yxy) * vec3<f32>(1.0, -(1.0), 0.0))) + ((vec3<f32>(_e59.g0_.x) * _e63.g0_.xzy) * vec3<f32>(0.0, 1.0, -(1.0)))));
}

fn flat_point_flat_point_add(self_362: FlatPoint, other_308: FlatPoint) -> FlatPoint {
    var self_363: FlatPoint;
    var other_309: FlatPoint;

    self_363 = self_362;
    other_309 = other_308;
    let _e4: FlatPoint = self_363;
    let _e6: FlatPoint = other_309;
    return FlatPoint((_e4.g0_ + _e6.g0_));
}

fn flat_point_flat_point_sub(self_364: FlatPoint, other_310: FlatPoint) -> FlatPoint {
    var self_365: FlatPoint;
    var other_311: FlatPoint;

    self_365 = self_364;
    other_311 = other_310;
    let _e4: FlatPoint = self_365;
    let _e6: FlatPoint = other_311;
    return FlatPoint((_e4.g0_ - _e6.g0_));
}

fn flat_point_flat_point_mul(self_366: FlatPoint, other_312: FlatPoint) -> FlatPoint {
    var self_367: FlatPoint;
    var other_313: FlatPoint;

    self_367 = self_366;
    other_313 = other_312;
    let _e4: FlatPoint = self_367;
    let _e6: FlatPoint = other_313;
    return FlatPoint((_e4.g0_ * _e6.g0_));
}

fn flat_point_flat_point_div(self_368: FlatPoint, other_314: FlatPoint) -> FlatPoint {
    var self_369: FlatPoint;
    var other_315: FlatPoint;

    self_369 = self_368;
    other_315 = other_314;
    let _e4: FlatPoint = self_369;
    let _e7: FlatPoint = self_369;
    let _e10: FlatPoint = self_369;
    let _e13: FlatPoint = self_369;
    let _e23: FlatPoint = other_315;
    let _e26: FlatPoint = other_315;
    let _e29: FlatPoint = other_315;
    let _e32: FlatPoint = other_315;
    return FlatPoint((((vec4<f32>(_e4.g0_.x, _e7.g0_.y, _e10.g0_.z, _e13.g0_.w) * vec4<f32>(1.0, 1.0, 1.0, 1.0)) / vec4<f32>(_e23.g0_.x, _e26.g0_.y, _e29.g0_.z, _e32.g0_.w)) * vec4<f32>(1.0, 1.0, 1.0, 1.0)));
}

fn flat_point_dipole_add(self_370: FlatPoint, other_316: Dipole) -> Dipole {
    var self_371: FlatPoint;
    var other_317: Dipole;

    self_371 = self_370;
    other_317 = other_316;
    let _e4: Dipole = other_317;
    let _e6: Dipole = other_317;
    let _e8: FlatPoint = self_371;
    let _e10: Dipole = other_317;
    return Dipole(_e4.g0_, _e6.g1_, (_e8.g0_ + _e10.g2_));
}

fn flat_point_dipole_sub(self_372: FlatPoint, other_318: Dipole) -> Dipole {
    var self_373: FlatPoint;
    var other_319: Dipole;

    self_373 = self_372;
    other_319 = other_318;
    let _e6: Dipole = other_319;
    let _e11: Dipole = other_319;
    let _e14: FlatPoint = self_373;
    let _e16: Dipole = other_319;
    return Dipole((vec3<f32>(0.0) - _e6.g0_), (vec3<f32>(0.0) - _e11.g1_), (_e14.g0_ - _e16.g2_));
}

fn flat_point_dipole_geometric_product(self_374: FlatPoint, other_320: Dipole) -> Flector {
    var self_375: FlatPoint;
    var other_321: Dipole;

    self_375 = self_374;
    other_321 = other_320;
    let _e4: FlatPoint = self_375;
    let _e8: Dipole = other_321;
    let _e11: Dipole = other_321;
    let _e14: Dipole = other_321;
    let _e17: Dipole = other_321;
    let _e30: FlatPoint = self_375;
    let _e34: Dipole = other_321;
    let _e37: Dipole = other_321;
    let _e40: Dipole = other_321;
    let _e43: Dipole = other_321;
    let _e57: FlatPoint = self_375;
    let _e61: Dipole = other_321;
    let _e64: Dipole = other_321;
    let _e67: Dipole = other_321;
    let _e70: Dipole = other_321;
    let _e84: FlatPoint = self_375;
    let _e88: Dipole = other_321;
    let _e91: Dipole = other_321;
    let _e94: Dipole = other_321;
    let _e97: Dipole = other_321;
    let _e110: FlatPoint = self_375;
    let _e114: Dipole = other_321;
    let _e117: Dipole = other_321;
    let _e120: Dipole = other_321;
    let _e123: Dipole = other_321;
    let _e137: FlatPoint = self_375;
    let _e141: Dipole = other_321;
    let _e144: Dipole = other_321;
    let _e147: Dipole = other_321;
    let _e150: Dipole = other_321;
    let _e162: FlatPoint = self_375;
    let _e166: Dipole = other_321;
    let _e169: Dipole = other_321;
    let _e172: Dipole = other_321;
    let _e175: Dipole = other_321;
    return Flector(((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g1_.z, _e11.g1_.z, _e14.g1_.x, _e17.g0_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e30.g0_.z) * vec4<f32>(_e34.g1_.y, _e37.g1_.x, _e40.g1_.y, _e43.g0_.z)) * vec4<f32>(1.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e57.g0_.x) * vec4<f32>(_e61.g1_.x, _e64.g1_.z, _e67.g1_.y, _e70.g0_.x)) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))), (((((vec4<f32>(_e84.g0_.y) * vec4<f32>(_e88.g0_.z, _e91.g0_.z, _e94.g0_.x, _e97.g1_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e110.g0_.z) * vec4<f32>(_e114.g0_.y, _e117.g0_.x, _e120.g0_.y, _e123.g1_.z)) * vec4<f32>(1.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e137.g0_.w) * vec4<f32>(_e141.g1_.x, _e144.g1_.y, _e147.g1_.z, _e150.g1_.x)) * vec4<f32>(1.0, 1.0, 1.0, 0.0))) + ((vec4<f32>(_e162.g0_.x) * vec4<f32>(_e166.g0_.x, _e169.g0_.z, _e172.g0_.y, _e175.g1_.x)) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))));
}

fn flat_point_dipole_outer_product(self_376: FlatPoint, other_322: Dipole) -> Plane {
    var self_377: FlatPoint;
    var other_323: Dipole;

    self_377 = self_376;
    other_323 = other_322;
    let _e4: FlatPoint = self_377;
    let _e8: Dipole = other_323;
    let _e11: Dipole = other_323;
    let _e14: Dipole = other_323;
    let _e17: Dipole = other_323;
    let _e30: FlatPoint = self_377;
    let _e34: Dipole = other_323;
    let _e37: Dipole = other_323;
    let _e40: Dipole = other_323;
    let _e43: Dipole = other_323;
    let _e57: FlatPoint = self_377;
    let _e61: Dipole = other_323;
    let _e64: Dipole = other_323;
    let _e67: Dipole = other_323;
    let _e70: Dipole = other_323;
    let _e82: FlatPoint = self_377;
    let _e86: Dipole = other_323;
    let _e89: Dipole = other_323;
    let _e92: Dipole = other_323;
    let _e95: Dipole = other_323;
    return Plane((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.z, _e11.g0_.z, _e14.g0_.x, _e17.g1_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e30.g0_.z) * vec4<f32>(_e34.g0_.y, _e37.g0_.x, _e40.g0_.y, _e43.g1_.z)) * vec4<f32>(1.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e57.g0_.w) * vec4<f32>(_e61.g1_.x, _e64.g1_.y, _e67.g1_.z, _e70.g1_.x)) * vec4<f32>(1.0, 1.0, 1.0, 0.0))) + ((vec4<f32>(_e82.g0_.x) * vec4<f32>(_e86.g0_.x, _e89.g0_.z, _e92.g0_.y, _e95.g1_.x)) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))));
}

fn flat_point_circle_regressive_product(self_378: FlatPoint, other_324: Circle) -> Scalar {
    var self_379: FlatPoint;
    var other_325: Circle;

    self_379 = self_378;
    other_325 = other_324;
    let _e5: FlatPoint = self_379;
    let _e8: Circle = other_325;
    let _e13: FlatPoint = self_379;
    let _e16: Circle = other_325;
    let _e21: FlatPoint = self_379;
    let _e24: Circle = other_325;
    let _e29: FlatPoint = self_379;
    let _e32: Circle = other_325;
    return Scalar(((((0.0 - (_e5.g0_.x * _e8.g0_.x)) - (_e13.g0_.y * _e16.g0_.y)) - (_e21.g0_.z * _e24.g0_.z)) - (_e29.g0_.w * _e32.g0_.w)));
}

fn flat_point_circle_outer_product(self_380: FlatPoint, other_326: Circle) -> AntiScalar {
    var self_381: FlatPoint;
    var other_327: Circle;

    self_381 = self_380;
    other_327 = other_326;
    let _e5: FlatPoint = self_381;
    let _e8: Circle = other_327;
    let _e13: FlatPoint = self_381;
    let _e16: Circle = other_327;
    let _e21: FlatPoint = self_381;
    let _e24: Circle = other_327;
    let _e29: FlatPoint = self_381;
    let _e32: Circle = other_327;
    return AntiScalar(((((0.0 - (_e5.g0_.x * _e8.g0_.x)) - (_e13.g0_.y * _e16.g0_.y)) - (_e21.g0_.z * _e24.g0_.z)) - (_e29.g0_.w * _e32.g0_.w)));
}

fn flat_point_plane_add(self_382: FlatPoint, other_328: Plane) -> Flector {
    var self_383: FlatPoint;
    var other_329: Plane;

    self_383 = self_382;
    other_329 = other_328;
    let _e4: FlatPoint = self_383;
    let _e6: Plane = other_329;
    return Flector(_e4.g0_, _e6.g0_);
}

fn flat_point_plane_sub(self_384: FlatPoint, other_330: Plane) -> Flector {
    var self_385: FlatPoint;
    var other_331: Plane;

    self_385 = self_384;
    other_331 = other_330;
    let _e4: FlatPoint = self_385;
    let _e8: Plane = other_331;
    return Flector(_e4.g0_, (vec4<f32>(0.0) - _e8.g0_));
}

fn flat_point_sphere_regressive_product(self_386: FlatPoint, other_332: Sphere) -> RadialPoint {
    var self_387: FlatPoint;
    var other_333: Sphere;

    self_387 = self_386;
    other_333 = other_332;
    let _e4: FlatPoint = self_387;
    let _e7: FlatPoint = self_387;
    let _e10: FlatPoint = self_387;
    let _e14: Sphere = other_333;
    let _e23: FlatPoint = self_387;
    let _e27: Sphere = other_333;
    let _e36: FlatPoint = self_387;
    let _e40: Sphere = other_333;
    let _e50: FlatPoint = self_387;
    let _e54: Sphere = other_333;
    let _e63: FlatPoint = self_387;
    let _e67: Sphere = other_333;
    return RadialPoint(((vec3<f32>(_e4.g0_.x, _e7.g0_.y, _e10.g0_.z) * vec3<f32>(_e14.g0_.x)) * vec3<f32>(-(1.0))), (((((vec2<f32>(_e23.g0_.y) * vec2<f32>(_e27.g1_.y)) * vec2<f32>(0.0, 1.0)) + ((vec2<f32>(_e36.g0_.z) * vec2<f32>(_e40.g1_.z)) * vec2<f32>(0.0, 1.0))) + ((vec2<f32>(_e50.g0_.w) * _e54.g0_) * vec2<f32>(-(1.0), 1.0))) + ((vec2<f32>(_e63.g0_.x) * vec2<f32>(_e67.g1_.x)) * vec2<f32>(0.0, 1.0))));
}

fn flat_point_motor_regressive_product(self_388: FlatPoint, other_334: Motor) -> FlatPoint {
    var self_389: FlatPoint;
    var other_335: Motor;

    self_389 = self_388;
    other_335 = other_334;
    let _e4: FlatPoint = self_389;
    let _e6: Motor = other_335;
    return FlatPoint((_e4.g0_ * vec4<f32>(_e6.g0_.w)));
}

fn flat_point_rotor_regressive_product(self_390: FlatPoint, other_336: Rotor) -> FlatPoint {
    var self_391: FlatPoint;
    var other_337: Rotor;

    self_391 = self_390;
    other_337 = other_336;
    let _e4: FlatPoint = self_391;
    let _e6: Rotor = other_337;
    return FlatPoint((_e4.g0_ * vec4<f32>(_e6.g0_.w)));
}

fn flat_point_translator_regressive_product(self_392: FlatPoint, other_338: Translator) -> FlatPoint {
    var self_393: FlatPoint;
    var other_339: Translator;

    self_393 = self_392;
    other_339 = other_338;
    let _e4: FlatPoint = self_393;
    let _e6: Translator = other_339;
    return FlatPoint((_e4.g0_ * vec4<f32>(_e6.g0_.w)));
}

fn flat_point_flector_add(self_394: FlatPoint, other_340: Flector) -> Flector {
    var self_395: FlatPoint;
    var other_341: Flector;

    self_395 = self_394;
    other_341 = other_340;
    let _e4: FlatPoint = self_395;
    let _e6: Flector = other_341;
    let _e9: Flector = other_341;
    return Flector((_e4.g0_ + _e6.g0_), _e9.g1_);
}

fn flat_point_flector_sub(self_396: FlatPoint, other_342: Flector) -> Flector {
    var self_397: FlatPoint;
    var other_343: Flector;

    self_397 = self_396;
    other_343 = other_342;
    let _e4: FlatPoint = self_397;
    let _e6: Flector = other_343;
    let _e11: Flector = other_343;
    return Flector((_e4.g0_ - _e6.g0_), (vec4<f32>(0.0) - _e11.g1_));
}

fn flat_point_multi_vector_add(self_398: FlatPoint, other_344: MultiVector) -> MultiVector {
    var self_399: FlatPoint;
    var other_345: MultiVector;

    self_399 = self_398;
    other_345 = other_344;
    let _e4: MultiVector = other_345;
    let _e6: MultiVector = other_345;
    let _e8: MultiVector = other_345;
    let _e10: FlatPoint = self_399;
    let _e12: MultiVector = other_345;
    let _e15: MultiVector = other_345;
    let _e17: MultiVector = other_345;
    let _e19: MultiVector = other_345;
    let _e21: MultiVector = other_345;
    let _e23: MultiVector = other_345;
    let _e25: MultiVector = other_345;
    return MultiVector(_e4.g0_, _e6.g1_, _e8.g2_, (_e10.g0_ + _e12.g3_), _e15.g4_, _e17.g5_, _e19.g6_, _e21.g7_, _e23.g8_, _e25.g9_);
}

fn flat_point_multi_vector_sub(self_400: FlatPoint, other_346: MultiVector) -> MultiVector {
    var self_401: FlatPoint;
    var other_347: MultiVector;

    self_401 = self_400;
    other_347 = other_346;
    let _e6: MultiVector = other_347;
    let _e11: MultiVector = other_347;
    let _e16: MultiVector = other_347;
    let _e19: FlatPoint = self_401;
    let _e21: MultiVector = other_347;
    let _e26: MultiVector = other_347;
    let _e31: MultiVector = other_347;
    let _e36: MultiVector = other_347;
    let _e41: MultiVector = other_347;
    let _e46: MultiVector = other_347;
    let _e51: MultiVector = other_347;
    return MultiVector((vec3<f32>(0.0) - _e6.g0_), (vec3<f32>(0.0) - _e11.g1_), (vec2<f32>(0.0) - _e16.g2_), (_e19.g0_ - _e21.g3_), (vec3<f32>(0.0) - _e26.g4_), (vec3<f32>(0.0) - _e31.g5_), (vec3<f32>(0.0) - _e36.g6_), (vec3<f32>(0.0) - _e41.g7_), (vec4<f32>(0.0) - _e46.g8_), (vec4<f32>(0.0) - _e51.g9_));
}

fn flat_point_scale(self_402: FlatPoint, other_348: f32) -> FlatPoint {
    var self_403: FlatPoint;
    var other_349: f32;

    self_403 = self_402;
    other_349 = other_348;
    let _e4: FlatPoint = self_403;
    let _e5: f32 = other_349;
    let _e7: FlatPoint = flat_point_scalar_geometric_product(_e4, Scalar(_e5));
    return _e7;
}

fn dipole_zero() -> Dipole {
    return Dipole(vec3<f32>(0.0), vec3<f32>(0.0), vec4<f32>(0.0));
}

fn dipole_one() -> Dipole {
    return Dipole(vec3<f32>(0.0), vec3<f32>(0.0), vec4<f32>(0.0));
}

fn dipole_neg(self_404: Dipole) -> Dipole {
    var self_405: Dipole;

    self_405 = self_404;
    let _e2: Dipole = self_405;
    let _e8: Dipole = self_405;
    let _e14: Dipole = self_405;
    return Dipole((_e2.g0_ * vec3<f32>(-(1.0))), (_e8.g1_ * vec3<f32>(-(1.0))), (_e14.g2_ * vec4<f32>(-(1.0))));
}

fn dipole_automorphism(self_406: Dipole) -> Dipole {
    var self_407: Dipole;

    self_407 = self_406;
    let _e2: Dipole = self_407;
    let _e4: Dipole = self_407;
    let _e6: Dipole = self_407;
    return Dipole(_e2.g0_, _e4.g1_, _e6.g2_);
}

fn dipole_reversal(self_408: Dipole) -> Dipole {
    var self_409: Dipole;

    self_409 = self_408;
    let _e2: Dipole = self_409;
    let _e8: Dipole = self_409;
    let _e14: Dipole = self_409;
    return Dipole((_e2.g0_ * vec3<f32>(-(1.0))), (_e8.g1_ * vec3<f32>(-(1.0))), (_e14.g2_ * vec4<f32>(-(1.0))));
}

fn dipole_conjugation(self_410: Dipole) -> Dipole {
    var self_411: Dipole;

    self_411 = self_410;
    let _e2: Dipole = self_411;
    let _e8: Dipole = self_411;
    let _e14: Dipole = self_411;
    return Dipole((_e2.g0_ * vec3<f32>(-(1.0))), (_e8.g1_ * vec3<f32>(-(1.0))), (_e14.g2_ * vec4<f32>(-(1.0))));
}

fn dipole_dual(self_412: Dipole) -> Circle {
    var self_413: Dipole;

    self_413 = self_412;
    let _e2: Dipole = self_413;
    let _e8: Dipole = self_413;
    let _e14: Dipole = self_413;
    return Circle((_e2.g2_ * vec4<f32>(-(1.0))), (_e8.g1_ * vec3<f32>(-(1.0))), (_e14.g0_ * vec3<f32>(-(1.0))));
}

fn dipole_scalar_geometric_product(self_414: Dipole, other_350: Scalar) -> Dipole {
    var self_415: Dipole;
    var other_351: Scalar;

    self_415 = self_414;
    other_351 = other_350;
    let _e4: Dipole = self_415;
    let _e6: Scalar = other_351;
    let _e10: Dipole = self_415;
    let _e12: Scalar = other_351;
    let _e16: Dipole = self_415;
    let _e18: Scalar = other_351;
    return Dipole((_e4.g0_ * vec3<f32>(_e6.g0_)), (_e10.g1_ * vec3<f32>(_e12.g0_)), (_e16.g2_ * vec4<f32>(_e18.g0_)));
}

fn dipole_scalar_outer_product(self_416: Dipole, other_352: Scalar) -> Dipole {
    var self_417: Dipole;
    var other_353: Scalar;

    self_417 = self_416;
    other_353 = other_352;
    let _e4: Dipole = self_417;
    let _e6: Scalar = other_353;
    let _e10: Dipole = self_417;
    let _e12: Scalar = other_353;
    let _e16: Dipole = self_417;
    let _e18: Scalar = other_353;
    return Dipole((_e4.g0_ * vec3<f32>(_e6.g0_)), (_e10.g1_ * vec3<f32>(_e12.g0_)), (_e16.g2_ * vec4<f32>(_e18.g0_)));
}

fn dipole_scalar_inner_product(self_418: Dipole, other_354: Scalar) -> Dipole {
    var self_419: Dipole;
    var other_355: Scalar;

    self_419 = self_418;
    other_355 = other_354;
    let _e4: Dipole = self_419;
    let _e6: Scalar = other_355;
    let _e10: Dipole = self_419;
    let _e12: Scalar = other_355;
    let _e16: Dipole = self_419;
    let _e18: Scalar = other_355;
    return Dipole((_e4.g0_ * vec3<f32>(_e6.g0_)), (_e10.g1_ * vec3<f32>(_e12.g0_)), (_e16.g2_ * vec4<f32>(_e18.g0_)));
}

fn dipole_scalar_right_contraction(self_420: Dipole, other_356: Scalar) -> Dipole {
    var self_421: Dipole;
    var other_357: Scalar;

    self_421 = self_420;
    other_357 = other_356;
    let _e4: Dipole = self_421;
    let _e6: Scalar = other_357;
    let _e10: Dipole = self_421;
    let _e12: Scalar = other_357;
    let _e16: Dipole = self_421;
    let _e18: Scalar = other_357;
    return Dipole((_e4.g0_ * vec3<f32>(_e6.g0_)), (_e10.g1_ * vec3<f32>(_e12.g0_)), (_e16.g2_ * vec4<f32>(_e18.g0_)));
}

fn dipole_anti_scalar_regressive_product(self_422: Dipole, other_358: AntiScalar) -> Dipole {
    var self_423: Dipole;
    var other_359: AntiScalar;

    self_423 = self_422;
    other_359 = other_358;
    let _e4: Dipole = self_423;
    let _e6: AntiScalar = other_359;
    let _e10: Dipole = self_423;
    let _e12: AntiScalar = other_359;
    let _e16: Dipole = self_423;
    let _e18: AntiScalar = other_359;
    return Dipole((_e4.g0_ * vec3<f32>(_e6.g0_)), (_e10.g1_ * vec3<f32>(_e12.g0_)), (_e16.g2_ * vec4<f32>(_e18.g0_)));
}

fn dipole_radial_point_outer_product(self_424: Dipole, other_360: RadialPoint) -> Circle {
    var self_425: Dipole;
    var other_361: RadialPoint;

    self_425 = self_424;
    other_361 = other_360;
    let _e4: Dipole = self_425;
    let _e8: RadialPoint = other_361;
    let _e11: RadialPoint = other_361;
    let _e14: RadialPoint = other_361;
    let _e17: RadialPoint = other_361;
    let _e29: Dipole = self_425;
    let _e33: RadialPoint = other_361;
    let _e36: RadialPoint = other_361;
    let _e39: RadialPoint = other_361;
    let _e42: RadialPoint = other_361;
    let _e55: Dipole = self_425;
    let _e59: RadialPoint = other_361;
    let _e62: RadialPoint = other_361;
    let _e65: RadialPoint = other_361;
    let _e68: RadialPoint = other_361;
    let _e81: Dipole = self_425;
    let _e85: RadialPoint = other_361;
    let _e88: RadialPoint = other_361;
    let _e91: RadialPoint = other_361;
    let _e94: RadialPoint = other_361;
    let _e107: Dipole = self_425;
    let _e111: RadialPoint = other_361;
    let _e114: RadialPoint = other_361;
    let _e117: RadialPoint = other_361;
    let _e120: RadialPoint = other_361;
    let _e133: Dipole = self_425;
    let _e137: RadialPoint = other_361;
    let _e140: RadialPoint = other_361;
    let _e143: RadialPoint = other_361;
    let _e146: RadialPoint = other_361;
    let _e159: Dipole = self_425;
    let _e163: RadialPoint = other_361;
    let _e173: Dipole = self_425;
    let _e177: RadialPoint = other_361;
    let _e188: Dipole = self_425;
    let _e192: RadialPoint = other_361;
    let _e203: Dipole = self_425;
    let _e207: RadialPoint = other_361;
    let _e211: Dipole = self_425;
    let _e213: RadialPoint = other_361;
    let _e219: Dipole = self_425;
    let _e223: RadialPoint = other_361;
    let _e233: Dipole = self_425;
    let _e237: RadialPoint = other_361;
    let _e248: Dipole = self_425;
    let _e252: RadialPoint = other_361;
    let _e263: Dipole = self_425;
    let _e265: RadialPoint = other_361;
    return Circle((((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.z, _e11.g0_.z, _e14.g0_.x, _e17.g0_.z)) * vec4<f32>(1.0, 0.0, -(1.0), 0.0)) + ((vec4<f32>(_e29.g0_.z) * vec4<f32>(_e33.g0_.y, _e36.g0_.x, _e39.g0_.y, _e42.g0_.y)) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))) + ((vec4<f32>(_e55.g1_.x) * vec4<f32>(_e59.g1_.x, _e62.g1_.x, _e65.g1_.x, _e68.g0_.x)) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e81.g1_.y) * vec4<f32>(_e85.g1_.x, _e88.g1_.x, _e91.g1_.x, _e94.g0_.y)) * vec4<f32>(0.0, 1.0, 0.0, -(1.0)))) + ((vec4<f32>(_e107.g1_.z) * vec4<f32>(_e111.g1_.x, _e114.g1_.x, _e117.g1_.x, _e120.g0_.z)) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e133.g0_.x) * vec4<f32>(_e137.g0_.x, _e140.g0_.z, _e143.g0_.y, _e146.g0_.x)) * vec4<f32>(0.0, -(1.0), 1.0, 0.0))), ((((((vec3<f32>(_e159.g2_.x) * vec3<f32>(_e163.g1_.x)) * vec3<f32>(1.0, 0.0, 0.0)) + ((vec3<f32>(_e173.g2_.y) * vec3<f32>(_e177.g1_.x)) * vec3<f32>(0.0, 1.0, 0.0))) + ((vec3<f32>(_e188.g2_.z) * vec3<f32>(_e192.g1_.x)) * vec3<f32>(0.0, 0.0, 1.0))) - (vec3<f32>(_e203.g2_.w) * _e207.g0_)) + (_e211.g0_ * vec3<f32>(_e213.g1_.y))), (((((vec3<f32>(_e219.g2_.x) * _e223.g0_.zzy) * vec3<f32>(0.0, 1.0, -(1.0))) + ((vec3<f32>(_e233.g2_.y) * _e237.g0_.zzx) * vec3<f32>(-(1.0), 0.0, 1.0))) + ((vec3<f32>(_e248.g2_.z) * _e252.g0_.yxy) * vec3<f32>(1.0, -(1.0), 0.0))) + (_e263.g1_ * vec3<f32>(_e265.g1_.y))));
}

fn dipole_radial_point_inner_product(self_426: Dipole, other_362: RadialPoint) -> RadialPoint {
    var self_427: Dipole;
    var other_363: RadialPoint;

    self_427 = self_426;
    other_363 = other_362;
    let _e4: Dipole = self_427;
    let _e8: RadialPoint = other_363;
    let _e18: Dipole = self_427;
    let _e22: RadialPoint = other_363;
    let _e33: Dipole = self_427;
    let _e37: RadialPoint = other_363;
    let _e48: Dipole = self_427;
    let _e52: RadialPoint = other_363;
    let _e61: Dipole = self_427;
    let _e65: RadialPoint = other_363;
    let _e75: Dipole = self_427;
    let _e79: RadialPoint = other_363;
    let _e90: Dipole = self_427;
    let _e94: RadialPoint = other_363;
    let _e105: Dipole = self_427;
    let _e109: RadialPoint = other_363;
    let _e120: Dipole = self_427;
    let _e124: RadialPoint = other_363;
    return RadialPoint(((((vec3<f32>(_e4.g1_.y) * _e8.g0_.zzx) * vec3<f32>(-(1.0), 0.0, 1.0)) + ((vec3<f32>(_e18.g1_.z) * _e22.g0_.yxy) * vec3<f32>(1.0, -(1.0), 0.0))) + ((vec3<f32>(_e33.g1_.x) * _e37.g0_.xzy) * vec3<f32>(0.0, 1.0, -(1.0)))), (((((((vec2<f32>(_e48.g0_.y) * vec2<f32>(_e52.g0_.y)) * vec2<f32>(1.0, 0.0)) + ((vec2<f32>(_e61.g0_.z) * vec2<f32>(_e65.g0_.z)) * vec2<f32>(1.0, 0.0))) + ((vec2<f32>(_e75.g2_.x) * vec2<f32>(_e79.g0_.x)) * vec2<f32>(0.0, -(1.0)))) + ((vec2<f32>(_e90.g2_.y) * vec2<f32>(_e94.g0_.y)) * vec2<f32>(0.0, -(1.0)))) + ((vec2<f32>(_e105.g2_.z) * vec2<f32>(_e109.g0_.z)) * vec2<f32>(0.0, -(1.0)))) + ((vec2<f32>(_e120.g0_.x) * vec2<f32>(_e124.g0_.x)) * vec2<f32>(1.0, 0.0))));
}

fn dipole_radial_point_right_contraction(self_428: Dipole, other_364: RadialPoint) -> RadialPoint {
    var self_429: Dipole;
    var other_365: RadialPoint;

    self_429 = self_428;
    other_365 = other_364;
    let _e4: Dipole = self_429;
    let _e8: RadialPoint = other_365;
    let _e18: Dipole = self_429;
    let _e22: RadialPoint = other_365;
    let _e33: Dipole = self_429;
    let _e37: RadialPoint = other_365;
    let _e48: Dipole = self_429;
    let _e52: RadialPoint = other_365;
    let _e61: Dipole = self_429;
    let _e65: RadialPoint = other_365;
    let _e75: Dipole = self_429;
    let _e79: RadialPoint = other_365;
    let _e90: Dipole = self_429;
    let _e94: RadialPoint = other_365;
    let _e105: Dipole = self_429;
    let _e109: RadialPoint = other_365;
    let _e120: Dipole = self_429;
    let _e124: RadialPoint = other_365;
    return RadialPoint(((((vec3<f32>(_e4.g1_.y) * _e8.g0_.zzx) * vec3<f32>(-(1.0), 0.0, 1.0)) + ((vec3<f32>(_e18.g1_.z) * _e22.g0_.yxy) * vec3<f32>(1.0, -(1.0), 0.0))) + ((vec3<f32>(_e33.g1_.x) * _e37.g0_.xzy) * vec3<f32>(0.0, 1.0, -(1.0)))), (((((((vec2<f32>(_e48.g0_.y) * vec2<f32>(_e52.g0_.y)) * vec2<f32>(1.0, 0.0)) + ((vec2<f32>(_e61.g0_.z) * vec2<f32>(_e65.g0_.z)) * vec2<f32>(1.0, 0.0))) + ((vec2<f32>(_e75.g2_.x) * vec2<f32>(_e79.g0_.x)) * vec2<f32>(0.0, -(1.0)))) + ((vec2<f32>(_e90.g2_.y) * vec2<f32>(_e94.g0_.y)) * vec2<f32>(0.0, -(1.0)))) + ((vec2<f32>(_e105.g2_.z) * vec2<f32>(_e109.g0_.z)) * vec2<f32>(0.0, -(1.0)))) + ((vec2<f32>(_e120.g0_.x) * vec2<f32>(_e124.g0_.x)) * vec2<f32>(1.0, 0.0))));
}

fn dipole_flat_point_into(self_430: Dipole) -> FlatPoint {
    var self_431: Dipole;

    self_431 = self_430;
    let _e2: Dipole = self_431;
    return FlatPoint(_e2.g2_);
}

fn dipole_flat_point_add(self_432: Dipole, other_366: FlatPoint) -> Dipole {
    var self_433: Dipole;
    var other_367: FlatPoint;

    self_433 = self_432;
    other_367 = other_366;
    let _e4: Dipole = self_433;
    let _e6: Dipole = self_433;
    let _e8: Dipole = self_433;
    let _e10: FlatPoint = other_367;
    return Dipole(_e4.g0_, _e6.g1_, (_e8.g2_ + _e10.g0_));
}

fn dipole_flat_point_sub(self_434: Dipole, other_368: FlatPoint) -> Dipole {
    var self_435: Dipole;
    var other_369: FlatPoint;

    self_435 = self_434;
    other_369 = other_368;
    let _e4: Dipole = self_435;
    let _e6: Dipole = self_435;
    let _e8: Dipole = self_435;
    let _e10: FlatPoint = other_369;
    return Dipole(_e4.g0_, _e6.g1_, (_e8.g2_ - _e10.g0_));
}

fn dipole_flat_point_geometric_product(self_436: Dipole, other_370: FlatPoint) -> Flector {
    var self_437: Dipole;
    var other_371: FlatPoint;

    self_437 = self_436;
    other_371 = other_370;
    let _e4: Dipole = self_437;
    let _e8: FlatPoint = other_371;
    let _e19: Dipole = self_437;
    let _e23: FlatPoint = other_371;
    let _e35: Dipole = self_437;
    let _e39: FlatPoint = other_371;
    let _e51: Dipole = self_437;
    let _e55: FlatPoint = other_371;
    let _e67: Dipole = self_437;
    let _e70: Dipole = self_437;
    let _e73: Dipole = self_437;
    let _e76: Dipole = self_437;
    let _e80: FlatPoint = other_371;
    let _e92: Dipole = self_437;
    let _e96: FlatPoint = other_371;
    let _e107: Dipole = self_437;
    let _e111: FlatPoint = other_371;
    let _e123: Dipole = self_437;
    let _e127: FlatPoint = other_371;
    let _e139: Dipole = self_437;
    let _e143: FlatPoint = other_371;
    let _e155: Dipole = self_437;
    let _e158: Dipole = self_437;
    let _e161: Dipole = self_437;
    let _e164: Dipole = self_437;
    let _e168: FlatPoint = other_371;
    return Flector(((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y)) * vec4<f32>(0.0, 0.0, 0.0, 1.0)) + ((vec4<f32>(_e19.g0_.z) * vec4<f32>(_e23.g0_.z)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e35.g1_.y) * _e39.g0_.zzxz) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0))) + ((vec4<f32>(_e51.g1_.z) * _e55.g0_.yxyy) * vec4<f32>(1.0, -(1.0), 0.0, 0.0))) + ((vec4<f32>(_e67.g0_.x, _e70.g1_.x, _e73.g1_.x, _e76.g0_.x) * _e80.g0_.xzyx) * vec4<f32>(0.0, 1.0, -(1.0), 1.0))), ((((((vec4<f32>(_e92.g0_.y) * _e96.g0_.zzxz) * vec4<f32>(1.0, 0.0, -(1.0), 0.0)) + ((vec4<f32>(_e107.g0_.z) * _e111.g0_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))) + ((vec4<f32>(_e123.g1_.y) * _e127.g0_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0)))) + ((vec4<f32>(_e139.g1_.z) * _e143.g0_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e155.g1_.x, _e158.g0_.x, _e161.g0_.x, _e164.g1_.x) * _e168.g0_.wzyx) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))));
}

fn dipole_flat_point_outer_product(self_438: Dipole, other_372: FlatPoint) -> Plane {
    var self_439: Dipole;
    var other_373: FlatPoint;

    self_439 = self_438;
    other_373 = other_372;
    let _e4: Dipole = self_439;
    let _e8: FlatPoint = other_373;
    let _e19: Dipole = self_439;
    let _e23: FlatPoint = other_373;
    let _e35: Dipole = self_439;
    let _e39: FlatPoint = other_373;
    let _e51: Dipole = self_439;
    let _e55: FlatPoint = other_373;
    let _e67: Dipole = self_439;
    let _e70: Dipole = self_439;
    let _e73: Dipole = self_439;
    let _e76: Dipole = self_439;
    let _e80: FlatPoint = other_373;
    return Plane(((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.zzxz) * vec4<f32>(1.0, 0.0, -(1.0), 0.0)) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))) + ((vec4<f32>(_e35.g1_.y) * _e39.g0_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0)))) + ((vec4<f32>(_e51.g1_.z) * _e55.g0_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e67.g1_.x, _e70.g0_.x, _e73.g0_.x, _e76.g1_.x) * _e80.g0_.wzyx) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))));
}

fn dipole_dipole_add(self_440: Dipole, other_374: Dipole) -> Dipole {
    var self_441: Dipole;
    var other_375: Dipole;

    self_441 = self_440;
    other_375 = other_374;
    let _e4: Dipole = self_441;
    let _e6: Dipole = other_375;
    let _e9: Dipole = self_441;
    let _e11: Dipole = other_375;
    let _e14: Dipole = self_441;
    let _e16: Dipole = other_375;
    return Dipole((_e4.g0_ + _e6.g0_), (_e9.g1_ + _e11.g1_), (_e14.g2_ + _e16.g2_));
}

fn dipole_dipole_sub(self_442: Dipole, other_376: Dipole) -> Dipole {
    var self_443: Dipole;
    var other_377: Dipole;

    self_443 = self_442;
    other_377 = other_376;
    let _e4: Dipole = self_443;
    let _e6: Dipole = other_377;
    let _e9: Dipole = self_443;
    let _e11: Dipole = other_377;
    let _e14: Dipole = self_443;
    let _e16: Dipole = other_377;
    return Dipole((_e4.g0_ - _e6.g0_), (_e9.g1_ - _e11.g1_), (_e14.g2_ - _e16.g2_));
}

fn dipole_dipole_mul(self_444: Dipole, other_378: Dipole) -> Dipole {
    var self_445: Dipole;
    var other_379: Dipole;

    self_445 = self_444;
    other_379 = other_378;
    let _e4: Dipole = self_445;
    let _e6: Dipole = other_379;
    let _e9: Dipole = self_445;
    let _e11: Dipole = other_379;
    let _e14: Dipole = self_445;
    let _e16: Dipole = other_379;
    return Dipole((_e4.g0_ * _e6.g0_), (_e9.g1_ * _e11.g1_), (_e14.g2_ * _e16.g2_));
}

fn dipole_dipole_div(self_446: Dipole, other_380: Dipole) -> Dipole {
    var self_447: Dipole;
    var other_381: Dipole;

    self_447 = self_446;
    other_381 = other_380;
    let _e4: Dipole = self_447;
    let _e7: Dipole = self_447;
    let _e10: Dipole = self_447;
    let _e19: Dipole = other_381;
    let _e22: Dipole = other_381;
    let _e25: Dipole = other_381;
    let _e35: Dipole = self_447;
    let _e38: Dipole = self_447;
    let _e41: Dipole = self_447;
    let _e50: Dipole = other_381;
    let _e53: Dipole = other_381;
    let _e56: Dipole = other_381;
    let _e66: Dipole = self_447;
    let _e69: Dipole = self_447;
    let _e72: Dipole = self_447;
    let _e75: Dipole = self_447;
    let _e85: Dipole = other_381;
    let _e88: Dipole = other_381;
    let _e91: Dipole = other_381;
    let _e94: Dipole = other_381;
    return Dipole((((vec3<f32>(_e4.g0_.x, _e7.g0_.y, _e10.g0_.z) * vec3<f32>(1.0, 1.0, 1.0)) / vec3<f32>(_e19.g0_.x, _e22.g0_.y, _e25.g0_.z)) * vec3<f32>(1.0, 1.0, 1.0)), (((vec3<f32>(_e35.g1_.x, _e38.g1_.y, _e41.g1_.z) * vec3<f32>(1.0, 1.0, 1.0)) / vec3<f32>(_e50.g1_.x, _e53.g1_.y, _e56.g1_.z)) * vec3<f32>(1.0, 1.0, 1.0)), (((vec4<f32>(_e66.g2_.x, _e69.g2_.y, _e72.g2_.z, _e75.g2_.w) * vec4<f32>(1.0, 1.0, 1.0, 1.0)) / vec4<f32>(_e85.g2_.x, _e88.g2_.y, _e91.g2_.z, _e94.g2_.w)) * vec4<f32>(1.0, 1.0, 1.0, 1.0)));
}

fn dipole_dipole_outer_product(self_448: Dipole, other_382: Dipole) -> Sphere {
    var self_449: Dipole;
    var other_383: Dipole;

    self_449 = self_448;
    other_383 = other_382;
    let _e4: Dipole = self_449;
    let _e8: Dipole = other_383;
    let _e18: Dipole = self_449;
    let _e22: Dipole = other_383;
    let _e33: Dipole = self_449;
    let _e37: Dipole = other_383;
    let _e40: Dipole = other_383;
    let _e46: Dipole = self_449;
    let _e50: Dipole = other_383;
    let _e53: Dipole = other_383;
    let _e59: Dipole = self_449;
    let _e63: Dipole = other_383;
    let _e66: Dipole = other_383;
    let _e72: Dipole = self_449;
    let _e76: Dipole = other_383;
    let _e87: Dipole = self_449;
    let _e91: Dipole = other_383;
    let _e102: Dipole = self_449;
    let _e106: Dipole = other_383;
    let _e117: Dipole = self_449;
    let _e121: Dipole = other_383;
    let _e132: Dipole = self_449;
    let _e136: Dipole = other_383;
    let _e139: Dipole = other_383;
    let _e142: Dipole = other_383;
    let _e153: Dipole = self_449;
    let _e157: Dipole = other_383;
    let _e160: Dipole = other_383;
    let _e163: Dipole = other_383;
    let _e175: Dipole = self_449;
    let _e179: Dipole = other_383;
    let _e190: Dipole = self_449;
    let _e194: Dipole = other_383;
    let _e205: Dipole = self_449;
    let _e209: Dipole = other_383;
    let _e220: Dipole = self_449;
    let _e224: Dipole = other_383;
    let _e235: Dipole = self_449;
    let _e239: Dipole = other_383;
    let _e250: Dipole = self_449;
    let _e254: Dipole = other_383;
    let _e258: Dipole = self_449;
    let _e261: Dipole = self_449;
    let _e264: Dipole = self_449;
    let _e268: Dipole = other_383;
    let _e271: Dipole = other_383;
    let _e274: Dipole = other_383;
    return Sphere(((((((((((vec2<f32>(_e4.g0_.y) * vec2<f32>(_e8.g1_.y)) * vec2<f32>(-(1.0), 0.0)) + ((vec2<f32>(_e18.g0_.z) * vec2<f32>(_e22.g1_.z)) * vec2<f32>(-(1.0), 0.0))) - (vec2<f32>(_e33.g1_.x) * vec2<f32>(_e37.g0_.x, _e40.g2_.x))) - (vec2<f32>(_e46.g1_.y) * vec2<f32>(_e50.g0_.y, _e53.g2_.y))) - (vec2<f32>(_e59.g1_.z) * vec2<f32>(_e63.g0_.z, _e66.g2_.z))) + ((vec2<f32>(_e72.g2_.x) * vec2<f32>(_e76.g1_.x)) * vec2<f32>(0.0, -(1.0)))) + ((vec2<f32>(_e87.g2_.y) * vec2<f32>(_e91.g1_.y)) * vec2<f32>(0.0, -(1.0)))) + ((vec2<f32>(_e102.g2_.z) * vec2<f32>(_e106.g1_.z)) * vec2<f32>(0.0, -(1.0)))) + ((vec2<f32>(_e117.g0_.x) * vec2<f32>(_e121.g1_.x)) * vec2<f32>(-(1.0), 0.0))), ((((((((((vec3<f32>(_e132.g0_.y) * vec3<f32>(_e136.g2_.z, _e139.g2_.z, _e142.g2_.x)) * vec3<f32>(1.0, 0.0, -(1.0))) + ((vec3<f32>(_e153.g0_.z) * vec3<f32>(_e157.g2_.y, _e160.g2_.x, _e163.g2_.y)) * vec3<f32>(-(1.0), 1.0, 0.0))) + ((vec3<f32>(_e175.g1_.y) * vec3<f32>(_e179.g2_.w)) * vec3<f32>(0.0, 1.0, 0.0))) + ((vec3<f32>(_e190.g1_.z) * vec3<f32>(_e194.g2_.w)) * vec3<f32>(0.0, 0.0, 1.0))) + ((vec3<f32>(_e205.g2_.x) * _e209.g0_.zzy) * vec3<f32>(0.0, 1.0, -(1.0)))) + ((vec3<f32>(_e220.g2_.y) * _e224.g0_.zzx) * vec3<f32>(-(1.0), 0.0, 1.0))) + ((vec3<f32>(_e235.g2_.z) * _e239.g0_.yxy) * vec3<f32>(1.0, -(1.0), 0.0))) + (vec3<f32>(_e250.g2_.w) * _e254.g1_)) + ((vec3<f32>(_e258.g1_.x, _e261.g0_.x, _e264.g0_.x) * vec3<f32>(_e268.g2_.w, _e271.g2_.z, _e274.g2_.y)) * vec3<f32>(1.0, -(1.0), 1.0))));
}

fn dipole_dipole_inner_product(self_450: Dipole, other_384: Dipole) -> Scalar {
    var self_451: Dipole;
    var other_385: Dipole;

    self_451 = self_450;
    other_385 = other_384;
    let _e5: Dipole = self_451;
    let _e8: Dipole = other_385;
    let _e13: Dipole = self_451;
    let _e16: Dipole = other_385;
    let _e21: Dipole = self_451;
    let _e24: Dipole = other_385;
    return Scalar((((0.0 - (_e5.g1_.x * _e8.g1_.x)) - (_e13.g1_.y * _e16.g1_.y)) - (_e21.g1_.z * _e24.g1_.z)));
}

fn dipole_dipole_left_contraction(self_452: Dipole, other_386: Dipole) -> Scalar {
    var self_453: Dipole;
    var other_387: Dipole;

    self_453 = self_452;
    other_387 = other_386;
    let _e5: Dipole = self_453;
    let _e8: Dipole = other_387;
    let _e13: Dipole = self_453;
    let _e16: Dipole = other_387;
    let _e21: Dipole = self_453;
    let _e24: Dipole = other_387;
    return Scalar((((0.0 - (_e5.g1_.x * _e8.g1_.x)) - (_e13.g1_.y * _e16.g1_.y)) - (_e21.g1_.z * _e24.g1_.z)));
}

fn dipole_dipole_right_contraction(self_454: Dipole, other_388: Dipole) -> Scalar {
    var self_455: Dipole;
    var other_389: Dipole;

    self_455 = self_454;
    other_389 = other_388;
    let _e5: Dipole = self_455;
    let _e8: Dipole = other_389;
    let _e13: Dipole = self_455;
    let _e16: Dipole = other_389;
    let _e21: Dipole = self_455;
    let _e24: Dipole = other_389;
    return Scalar((((0.0 - (_e5.g1_.x * _e8.g1_.x)) - (_e13.g1_.y * _e16.g1_.y)) - (_e21.g1_.z * _e24.g1_.z)));
}

fn dipole_dipole_scalar_product(self_456: Dipole, other_390: Dipole) -> Scalar {
    var self_457: Dipole;
    var other_391: Dipole;

    self_457 = self_456;
    other_391 = other_390;
    let _e5: Dipole = self_457;
    let _e8: Dipole = other_391;
    let _e13: Dipole = self_457;
    let _e16: Dipole = other_391;
    let _e21: Dipole = self_457;
    let _e24: Dipole = other_391;
    return Scalar((((0.0 - (_e5.g1_.x * _e8.g1_.x)) - (_e13.g1_.y * _e16.g1_.y)) - (_e21.g1_.z * _e24.g1_.z)));
}

fn dipole_line_geometric_product(self_458: Dipole, other_392: Line) -> Motor {
    var self_459: Dipole;
    var other_393: Line;

    self_459 = self_458;
    other_393 = other_392;
    let _e4: Dipole = self_459;
    let _e8: Line = other_393;
    let _e11: Line = other_393;
    let _e14: Line = other_393;
    let _e17: Line = other_393;
    let _e30: Dipole = self_459;
    let _e34: Line = other_393;
    let _e37: Line = other_393;
    let _e40: Line = other_393;
    let _e43: Line = other_393;
    let _e57: Dipole = self_459;
    let _e61: Line = other_393;
    let _e64: Line = other_393;
    let _e67: Line = other_393;
    let _e70: Line = other_393;
    let _e84: Dipole = self_459;
    let _e88: Line = other_393;
    let _e91: Line = other_393;
    let _e94: Line = other_393;
    let _e97: Line = other_393;
    let _e111: Dipole = self_459;
    let _e115: Line = other_393;
    let _e118: Line = other_393;
    let _e121: Line = other_393;
    let _e124: Line = other_393;
    let _e138: Dipole = self_459;
    let _e142: Line = other_393;
    let _e145: Line = other_393;
    let _e148: Line = other_393;
    let _e151: Line = other_393;
    let _e165: Dipole = self_459;
    let _e169: Line = other_393;
    let _e172: Line = other_393;
    let _e175: Line = other_393;
    let _e178: Line = other_393;
    let _e191: Dipole = self_459;
    let _e195: Line = other_393;
    let _e198: Line = other_393;
    let _e201: Line = other_393;
    let _e204: Line = other_393;
    let _e218: Dipole = self_459;
    let _e222: Line = other_393;
    let _e225: Line = other_393;
    let _e228: Line = other_393;
    let _e231: Line = other_393;
    return Motor((((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g1_.z, _e11.g1_.z, _e14.g1_.x, _e17.g1_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e30.g0_.z) * vec4<f32>(_e34.g1_.y, _e37.g1_.x, _e40.g1_.y, _e43.g1_.z)) * vec4<f32>(1.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e57.g1_.x) * vec4<f32>(_e61.g0_.z, _e64.g0_.z, _e67.g0_.y, _e70.g0_.x)) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e84.g1_.y) * vec4<f32>(_e88.g0_.z, _e91.g0_.z, _e94.g0_.x, _e97.g0_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e111.g1_.z) * vec4<f32>(_e115.g0_.y, _e118.g0_.x, _e121.g0_.y, _e124.g0_.z)) * vec4<f32>(1.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e138.g0_.x) * vec4<f32>(_e142.g1_.x, _e145.g1_.z, _e148.g1_.y, _e151.g1_.x)) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))), ((((vec4<f32>(_e165.g1_.y) * vec4<f32>(_e169.g1_.z, _e172.g1_.z, _e175.g1_.x, _e178.g1_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e191.g1_.z) * vec4<f32>(_e195.g1_.y, _e198.g1_.x, _e201.g1_.y, _e204.g1_.z)) * vec4<f32>(1.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e218.g1_.x) * vec4<f32>(_e222.g1_.x, _e225.g1_.z, _e228.g1_.y, _e231.g1_.x)) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))));
}

fn dipole_line_regressive_product(self_460: Dipole, other_394: Line) -> Scalar {
    var self_461: Dipole;
    var other_395: Line;

    self_461 = self_460;
    other_395 = other_394;
    let _e5: Dipole = self_461;
    let _e8: Line = other_395;
    let _e13: Dipole = self_461;
    let _e16: Line = other_395;
    let _e21: Dipole = self_461;
    let _e24: Line = other_395;
    let _e29: Dipole = self_461;
    let _e32: Line = other_395;
    let _e37: Dipole = self_461;
    let _e40: Line = other_395;
    let _e45: Dipole = self_461;
    let _e48: Line = other_395;
    return Scalar(((((((0.0 - (_e5.g0_.x * _e8.g1_.x)) - (_e13.g0_.y * _e16.g1_.y)) - (_e21.g0_.z * _e24.g1_.z)) - (_e29.g1_.x * _e32.g0_.x)) - (_e37.g1_.y * _e40.g0_.y)) - (_e45.g1_.z * _e48.g0_.z)));
}

fn dipole_line_outer_product(self_462: Dipole, other_396: Line) -> AntiScalar {
    var self_463: Dipole;
    var other_397: Line;

    self_463 = self_462;
    other_397 = other_396;
    let _e5: Dipole = self_463;
    let _e8: Line = other_397;
    let _e13: Dipole = self_463;
    let _e16: Line = other_397;
    let _e21: Dipole = self_463;
    let _e24: Line = other_397;
    let _e29: Dipole = self_463;
    let _e32: Line = other_397;
    let _e37: Dipole = self_463;
    let _e40: Line = other_397;
    let _e45: Dipole = self_463;
    let _e48: Line = other_397;
    return AntiScalar(((((((0.0 - (_e5.g0_.x * _e8.g1_.x)) - (_e13.g0_.y * _e16.g1_.y)) - (_e21.g0_.z * _e24.g1_.z)) - (_e29.g1_.x * _e32.g0_.x)) - (_e37.g1_.y * _e40.g0_.y)) - (_e45.g1_.z * _e48.g0_.z)));
}

fn dipole_circle_regressive_product(self_464: Dipole, other_398: Circle) -> Scalar {
    var self_465: Dipole;
    var other_399: Circle;

    self_465 = self_464;
    other_399 = other_398;
    let _e5: Dipole = self_465;
    let _e8: Circle = other_399;
    let _e13: Dipole = self_465;
    let _e16: Circle = other_399;
    let _e21: Dipole = self_465;
    let _e24: Circle = other_399;
    let _e29: Dipole = self_465;
    let _e32: Circle = other_399;
    let _e37: Dipole = self_465;
    let _e40: Circle = other_399;
    let _e45: Dipole = self_465;
    let _e48: Circle = other_399;
    let _e53: Dipole = self_465;
    let _e56: Circle = other_399;
    let _e61: Dipole = self_465;
    let _e64: Circle = other_399;
    let _e69: Dipole = self_465;
    let _e72: Circle = other_399;
    let _e77: Dipole = self_465;
    let _e80: Circle = other_399;
    return Scalar(((((((((((0.0 - (_e5.g0_.x * _e8.g2_.x)) - (_e13.g0_.y * _e16.g2_.y)) - (_e21.g0_.z * _e24.g2_.z)) - (_e29.g1_.x * _e32.g1_.x)) - (_e37.g1_.y * _e40.g1_.y)) - (_e45.g1_.z * _e48.g1_.z)) - (_e53.g2_.x * _e56.g0_.x)) - (_e61.g2_.y * _e64.g0_.y)) - (_e69.g2_.z * _e72.g0_.z)) - (_e77.g2_.w * _e80.g0_.w)));
}

fn dipole_circle_outer_product(self_466: Dipole, other_400: Circle) -> AntiScalar {
    var self_467: Dipole;
    var other_401: Circle;

    self_467 = self_466;
    other_401 = other_400;
    let _e5: Dipole = self_467;
    let _e8: Circle = other_401;
    let _e13: Dipole = self_467;
    let _e16: Circle = other_401;
    let _e21: Dipole = self_467;
    let _e24: Circle = other_401;
    let _e29: Dipole = self_467;
    let _e32: Circle = other_401;
    let _e37: Dipole = self_467;
    let _e40: Circle = other_401;
    let _e45: Dipole = self_467;
    let _e48: Circle = other_401;
    let _e53: Dipole = self_467;
    let _e56: Circle = other_401;
    let _e61: Dipole = self_467;
    let _e64: Circle = other_401;
    let _e69: Dipole = self_467;
    let _e72: Circle = other_401;
    let _e77: Dipole = self_467;
    let _e80: Circle = other_401;
    return AntiScalar(((((((((((0.0 - (_e5.g0_.x * _e8.g2_.x)) - (_e13.g0_.y * _e16.g2_.y)) - (_e21.g0_.z * _e24.g2_.z)) - (_e29.g1_.x * _e32.g1_.x)) - (_e37.g1_.y * _e40.g1_.y)) - (_e45.g1_.z * _e48.g1_.z)) - (_e53.g2_.x * _e56.g0_.x)) - (_e61.g2_.y * _e64.g0_.y)) - (_e69.g2_.z * _e72.g0_.z)) - (_e77.g2_.w * _e80.g0_.w)));
}

fn dipole_circle_inner_product(self_468: Dipole, other_402: Circle) -> RadialPoint {
    var self_469: Dipole;
    var other_403: Circle;

    self_469 = self_468;
    other_403 = other_402;
    let _e4: Dipole = self_469;
    let _e6: Circle = other_403;
    let _e13: Dipole = self_469;
    let _e17: Circle = other_403;
    let _e20: Circle = other_403;
    let _e26: Dipole = self_469;
    let _e30: Circle = other_403;
    let _e33: Circle = other_403;
    let _e39: Dipole = self_469;
    let _e43: Circle = other_403;
    let _e46: Circle = other_403;
    return RadialPoint((_e4.g1_ * vec3<f32>(_e6.g0_.w)), (((vec2<f32>(0.0) - (vec2<f32>(_e13.g1_.x) * vec2<f32>(_e17.g0_.x, _e20.g2_.x))) - (vec2<f32>(_e26.g1_.y) * vec2<f32>(_e30.g0_.y, _e33.g2_.y))) - (vec2<f32>(_e39.g1_.z) * vec2<f32>(_e43.g0_.z, _e46.g2_.z))));
}

fn dipole_circle_left_contraction(self_470: Dipole, other_404: Circle) -> RadialPoint {
    var self_471: Dipole;
    var other_405: Circle;

    self_471 = self_470;
    other_405 = other_404;
    let _e4: Dipole = self_471;
    let _e6: Circle = other_405;
    let _e13: Dipole = self_471;
    let _e17: Circle = other_405;
    let _e20: Circle = other_405;
    let _e26: Dipole = self_471;
    let _e30: Circle = other_405;
    let _e33: Circle = other_405;
    let _e39: Dipole = self_471;
    let _e43: Circle = other_405;
    let _e46: Circle = other_405;
    return RadialPoint((_e4.g1_ * vec3<f32>(_e6.g0_.w)), (((vec2<f32>(0.0) - (vec2<f32>(_e13.g1_.x) * vec2<f32>(_e17.g0_.x, _e20.g2_.x))) - (vec2<f32>(_e26.g1_.y) * vec2<f32>(_e30.g0_.y, _e33.g2_.y))) - (vec2<f32>(_e39.g1_.z) * vec2<f32>(_e43.g0_.z, _e46.g2_.z))));
}

fn dipole_plane_regressive_product(self_472: Dipole, other_406: Plane) -> RadialPoint {
    var self_473: Dipole;
    var other_407: Plane;

    self_473 = self_472;
    other_407 = other_406;
    let _e4: Dipole = self_473;
    let _e8: Plane = other_407;
    let _e11: Plane = other_407;
    let _e14: Plane = other_407;
    let _e25: Dipole = self_473;
    let _e29: Plane = other_407;
    let _e32: Plane = other_407;
    let _e35: Plane = other_407;
    let _e47: Dipole = self_473;
    let _e51: Plane = other_407;
    let _e54: Plane = other_407;
    let _e57: Plane = other_407;
    let _e69: Dipole = self_473;
    let _e71: Plane = other_407;
    let _e77: Dipole = self_473;
    let _e81: Plane = other_407;
    let _e91: Dipole = self_473;
    let _e95: Plane = other_407;
    let _e106: Dipole = self_473;
    let _e110: Plane = other_407;
    let _e120: Dipole = self_473;
    let _e124: Plane = other_407;
    let _e134: Dipole = self_473;
    let _e138: Plane = other_407;
    let _e148: Dipole = self_473;
    let _e152: Plane = other_407;
    let _e162: Dipole = self_473;
    let _e166: Plane = other_407;
    return RadialPoint((((((vec3<f32>(_e4.g1_.x) * vec3<f32>(_e8.g0_.z, _e11.g0_.z, _e14.g0_.y)) * vec3<f32>(0.0, -(1.0), 1.0)) + ((vec3<f32>(_e25.g1_.y) * vec3<f32>(_e29.g0_.z, _e32.g0_.z, _e35.g0_.x)) * vec3<f32>(1.0, 0.0, -(1.0)))) + ((vec3<f32>(_e47.g1_.z) * vec3<f32>(_e51.g0_.y, _e54.g0_.x, _e57.g0_.y)) * vec3<f32>(-(1.0), 1.0, 0.0))) + (_e69.g0_ * vec3<f32>(_e71.g0_.w))), ((((((((vec2<f32>(_e77.g0_.y) * vec2<f32>(_e81.g0_.y)) * vec2<f32>(-(1.0), 0.0)) + ((vec2<f32>(_e91.g0_.z) * vec2<f32>(_e95.g0_.z)) * vec2<f32>(-(1.0), 0.0))) + ((vec2<f32>(_e106.g2_.x) * vec2<f32>(_e110.g0_.x)) * vec2<f32>(0.0, 1.0))) + ((vec2<f32>(_e120.g2_.y) * vec2<f32>(_e124.g0_.y)) * vec2<f32>(0.0, 1.0))) + ((vec2<f32>(_e134.g2_.z) * vec2<f32>(_e138.g0_.z)) * vec2<f32>(0.0, 1.0))) + ((vec2<f32>(_e148.g2_.w) * vec2<f32>(_e152.g0_.w)) * vec2<f32>(0.0, 1.0))) + ((vec2<f32>(_e162.g0_.x) * vec2<f32>(_e166.g0_.x)) * vec2<f32>(-(1.0), 0.0))));
}

fn dipole_plane_inner_product(self_474: Dipole, other_408: Plane) -> FlatPoint {
    var self_475: Dipole;
    var other_409: Plane;

    self_475 = self_474;
    other_409 = other_408;
    let _e4: Dipole = self_475;
    let _e8: Plane = other_409;
    let _e19: Dipole = self_475;
    let _e23: Plane = other_409;
    let _e35: Dipole = self_475;
    let _e39: Plane = other_409;
    return FlatPoint(((((vec4<f32>(_e4.g1_.y) * _e8.g0_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e19.g1_.z) * _e23.g0_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e35.g1_.x) * _e39.g0_.wxxx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))));
}

fn dipole_plane_left_contraction(self_476: Dipole, other_410: Plane) -> FlatPoint {
    var self_477: Dipole;
    var other_411: Plane;

    self_477 = self_476;
    other_411 = other_410;
    let _e4: Dipole = self_477;
    let _e8: Plane = other_411;
    let _e19: Dipole = self_477;
    let _e23: Plane = other_411;
    let _e35: Dipole = self_477;
    let _e39: Plane = other_411;
    return FlatPoint(((((vec4<f32>(_e4.g1_.y) * _e8.g0_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e19.g1_.z) * _e23.g0_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e35.g1_.x) * _e39.g0_.wxxx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))));
}

fn dipole_sphere_regressive_product(self_478: Dipole, other_412: Sphere) -> RadialPoint {
    var self_479: Dipole;
    var other_413: Sphere;

    self_479 = self_478;
    other_413 = other_412;
    let _e4: Dipole = self_479;
    let _e8: Sphere = other_413;
    let _e18: Dipole = self_479;
    let _e22: Sphere = other_413;
    let _e33: Dipole = self_479;
    let _e37: Sphere = other_413;
    let _e48: Dipole = self_479;
    let _e52: Sphere = other_413;
    let _e64: Dipole = self_479;
    let _e68: Sphere = other_413;
    let _e80: Dipole = self_479;
    let _e84: Sphere = other_413;
    let _e96: Dipole = self_479;
    let _e98: Sphere = other_413;
    let _e104: Dipole = self_479;
    let _e108: Sphere = other_413;
    let _e118: Dipole = self_479;
    let _e122: Sphere = other_413;
    let _e133: Dipole = self_479;
    let _e137: Sphere = other_413;
    let _e147: Dipole = self_479;
    let _e151: Sphere = other_413;
    let _e161: Dipole = self_479;
    let _e165: Sphere = other_413;
    let _e175: Dipole = self_479;
    let _e179: Sphere = other_413;
    let _e188: Dipole = self_479;
    let _e192: Sphere = other_413;
    return RadialPoint(((((((((vec3<f32>(_e4.g1_.x) * _e8.g1_.zzy) * vec3<f32>(0.0, -(1.0), 1.0)) + ((vec3<f32>(_e18.g1_.y) * _e22.g1_.zzx) * vec3<f32>(1.0, 0.0, -(1.0)))) + ((vec3<f32>(_e33.g1_.z) * _e37.g1_.yxy) * vec3<f32>(-(1.0), 1.0, 0.0))) + ((vec3<f32>(_e48.g2_.x) * vec3<f32>(_e52.g0_.x)) * vec3<f32>(-(1.0), 0.0, 0.0))) + ((vec3<f32>(_e64.g2_.y) * vec3<f32>(_e68.g0_.x)) * vec3<f32>(0.0, -(1.0), 0.0))) + ((vec3<f32>(_e80.g2_.z) * vec3<f32>(_e84.g0_.x)) * vec3<f32>(0.0, 0.0, -(1.0)))) + (_e96.g0_ * vec3<f32>(_e98.g0_.y))), ((((((((vec2<f32>(_e104.g0_.y) * vec2<f32>(_e108.g1_.y)) * vec2<f32>(-(1.0), 0.0)) + ((vec2<f32>(_e118.g0_.z) * vec2<f32>(_e122.g1_.z)) * vec2<f32>(-(1.0), 0.0))) + ((vec2<f32>(_e133.g2_.x) * vec2<f32>(_e137.g1_.x)) * vec2<f32>(0.0, 1.0))) + ((vec2<f32>(_e147.g2_.y) * vec2<f32>(_e151.g1_.y)) * vec2<f32>(0.0, 1.0))) + ((vec2<f32>(_e161.g2_.z) * vec2<f32>(_e165.g1_.z)) * vec2<f32>(0.0, 1.0))) + ((vec2<f32>(_e175.g2_.w) * _e179.g0_) * vec2<f32>(-(1.0), 1.0))) + ((vec2<f32>(_e188.g0_.x) * vec2<f32>(_e192.g1_.x)) * vec2<f32>(-(1.0), 0.0))));
}

fn dipole_motor_geometric_product(self_480: Dipole, other_414: Motor) -> Motor {
    var self_481: Dipole;
    var other_415: Motor;

    self_481 = self_480;
    other_415 = other_414;
    let _e4: Dipole = self_481;
    let _e8: Motor = other_415;
    let _e20: Dipole = self_481;
    let _e24: Motor = other_415;
    let _e37: Dipole = self_481;
    let _e41: Motor = other_415;
    let _e54: Dipole = self_481;
    let _e58: Motor = other_415;
    let _e71: Dipole = self_481;
    let _e75: Motor = other_415;
    let _e88: Dipole = self_481;
    let _e92: Motor = other_415;
    let _e105: Dipole = self_481;
    let _e109: Motor = other_415;
    let _e121: Dipole = self_481;
    let _e125: Motor = other_415;
    let _e138: Dipole = self_481;
    let _e142: Motor = other_415;
    return Motor((((((((vec4<f32>(_e4.g0_.x) * _e8.g1_.wzyx) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0))) + ((vec4<f32>(_e20.g0_.y) * _e24.g1_.zwxy) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e37.g0_.z) * _e41.g1_.yxwz) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e54.g1_.x) * _e58.g0_.wzyx) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e71.g1_.y) * _e75.g0_.zwxy) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e88.g1_.z) * _e92.g0_.yxwz) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))), ((((vec4<f32>(_e105.g1_.x) * _e109.g1_.wzyx) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0))) + ((vec4<f32>(_e121.g1_.y) * _e125.g1_.zwxy) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e138.g1_.z) * _e142.g1_.yxwz) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))));
}

fn dipole_rotor_geometric_product(self_482: Dipole, other_416: Rotor) -> Rotor {
    var self_483: Dipole;
    var other_417: Rotor;

    self_483 = self_482;
    other_417 = other_416;
    let _e4: Dipole = self_483;
    let _e8: Rotor = other_417;
    let _e20: Dipole = self_483;
    let _e24: Rotor = other_417;
    let _e37: Dipole = self_483;
    let _e41: Rotor = other_417;
    return Rotor(((((vec4<f32>(_e4.g1_.x) * _e8.g0_.wzyx) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0))) + ((vec4<f32>(_e20.g1_.y) * _e24.g0_.zwxy) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e37.g1_.z) * _e41.g0_.yxwz) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))));
}

fn dipole_rotor_outer_product(self_484: Dipole, other_418: Rotor) -> AntiScalar {
    var self_485: Dipole;
    var other_419: Rotor;

    self_485 = self_484;
    other_419 = other_418;
    let _e5: Dipole = self_485;
    let _e8: Rotor = other_419;
    let _e13: Dipole = self_485;
    let _e16: Rotor = other_419;
    let _e21: Dipole = self_485;
    let _e24: Rotor = other_419;
    return AntiScalar((((0.0 - (_e5.g1_.x * _e8.g0_.x)) - (_e13.g1_.y * _e16.g0_.y)) - (_e21.g1_.z * _e24.g0_.z)));
}

fn dipole_translator_geometric_product(self_486: Dipole, other_420: Translator) -> Motor {
    var self_487: Dipole;
    var other_421: Translator;

    self_487 = self_486;
    other_421 = other_420;
    let _e4: Dipole = self_487;
    let _e8: Translator = other_421;
    let _e20: Dipole = self_487;
    let _e24: Translator = other_421;
    let _e37: Dipole = self_487;
    let _e41: Translator = other_421;
    let _e53: Dipole = self_487;
    let _e57: Translator = other_421;
    let _e69: Dipole = self_487;
    let _e72: Dipole = self_487;
    let _e75: Dipole = self_487;
    let _e78: Dipole = self_487;
    let _e82: Translator = other_421;
    let _e95: Dipole = self_487;
    let _e99: Translator = other_421;
    let _e111: Dipole = self_487;
    let _e115: Translator = other_421;
    let _e128: Dipole = self_487;
    let _e132: Translator = other_421;
    return Motor(((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.zzxy) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e20.g0_.z) * _e24.g0_.yxyz) * vec4<f32>(1.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e37.g1_.y) * vec4<f32>(_e41.g0_.w)) * vec4<f32>(0.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e53.g1_.z) * vec4<f32>(_e57.g0_.w)) * vec4<f32>(0.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e69.g1_.x, _e72.g0_.x, _e75.g0_.x, _e78.g0_.x) * _e82.g0_.wzyx) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))), ((((vec4<f32>(_e95.g1_.y) * _e99.g0_.zzxy) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e111.g1_.z) * _e115.g0_.yxyz) * vec4<f32>(1.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e128.g1_.x) * _e132.g0_.xzyx) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))));
}

fn dipole_translator_outer_product(self_488: Dipole, other_422: Translator) -> AntiScalar {
    var self_489: Dipole;
    var other_423: Translator;

    self_489 = self_488;
    other_423 = other_422;
    let _e5: Dipole = self_489;
    let _e8: Translator = other_423;
    let _e13: Dipole = self_489;
    let _e16: Translator = other_423;
    let _e21: Dipole = self_489;
    let _e24: Translator = other_423;
    return AntiScalar((((0.0 - (_e5.g0_.x * _e8.g0_.x)) - (_e13.g0_.y * _e16.g0_.y)) - (_e21.g0_.z * _e24.g0_.z)));
}

fn dipole_flector_geometric_product(self_490: Dipole, other_424: Flector) -> Flector {
    var self_491: Dipole;
    var other_425: Flector;

    self_491 = self_490;
    other_425 = other_424;
    let _e4: Dipole = self_491;
    let _e8: Flector = other_425;
    let _e19: Dipole = self_491;
    let _e23: Flector = other_425;
    let _e35: Dipole = self_491;
    let _e39: Flector = other_425;
    let _e42: Flector = other_425;
    let _e45: Flector = other_425;
    let _e48: Flector = other_425;
    let _e62: Dipole = self_491;
    let _e66: Flector = other_425;
    let _e69: Flector = other_425;
    let _e72: Flector = other_425;
    let _e75: Flector = other_425;
    let _e89: Dipole = self_491;
    let _e93: Flector = other_425;
    let _e96: Flector = other_425;
    let _e99: Flector = other_425;
    let _e102: Flector = other_425;
    let _e116: Dipole = self_491;
    let _e120: Flector = other_425;
    let _e132: Dipole = self_491;
    let _e136: Flector = other_425;
    let _e139: Flector = other_425;
    let _e142: Flector = other_425;
    let _e145: Flector = other_425;
    let _e158: Dipole = self_491;
    let _e162: Flector = other_425;
    let _e165: Flector = other_425;
    let _e168: Flector = other_425;
    let _e171: Flector = other_425;
    let _e185: Dipole = self_491;
    let _e189: Flector = other_425;
    let _e192: Flector = other_425;
    let _e195: Flector = other_425;
    let _e198: Flector = other_425;
    let _e212: Dipole = self_491;
    let _e216: Flector = other_425;
    let _e219: Flector = other_425;
    let _e222: Flector = other_425;
    let _e225: Flector = other_425;
    let _e239: Dipole = self_491;
    let _e243: Flector = other_425;
    let _e246: Flector = other_425;
    let _e249: Flector = other_425;
    let _e252: Flector = other_425;
    let _e266: Dipole = self_491;
    let _e270: Flector = other_425;
    let _e273: Flector = other_425;
    let _e276: Flector = other_425;
    let _e279: Flector = other_425;
    return Flector((((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y)) * vec4<f32>(0.0, 0.0, 0.0, 1.0)) + ((vec4<f32>(_e19.g0_.z) * vec4<f32>(_e23.g0_.z)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e35.g1_.x) * vec4<f32>(_e39.g1_.w, _e42.g0_.z, _e45.g0_.y, _e48.g1_.x)) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e62.g1_.y) * vec4<f32>(_e66.g0_.z, _e69.g1_.w, _e72.g0_.x, _e75.g1_.y)) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e89.g1_.z) * vec4<f32>(_e93.g0_.y, _e96.g0_.x, _e99.g1_.w, _e102.g1_.z)) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e116.g0_.x) * vec4<f32>(_e120.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))), (((((((vec4<f32>(_e132.g0_.y) * vec4<f32>(_e136.g0_.z, _e139.g1_.w, _e142.g0_.x, _e145.g0_.z)) * vec4<f32>(1.0, -(1.0), -(1.0), 0.0)) + ((vec4<f32>(_e158.g0_.z) * vec4<f32>(_e162.g0_.y, _e165.g0_.x, _e168.g1_.w, _e171.g0_.y)) * vec4<f32>(-(1.0), 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e185.g1_.x) * vec4<f32>(_e189.g0_.w, _e192.g1_.z, _e195.g1_.y, _e198.g0_.x)) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e212.g1_.y) * vec4<f32>(_e216.g1_.z, _e219.g0_.w, _e222.g1_.x, _e225.g0_.y)) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e239.g1_.z) * vec4<f32>(_e243.g1_.y, _e246.g1_.x, _e249.g0_.w, _e252.g0_.z)) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e266.g0_.x) * vec4<f32>(_e270.g1_.w, _e273.g0_.z, _e276.g0_.y, _e279.g1_.x)) * vec4<f32>(-(1.0), -(1.0), 1.0, 0.0))));
}

fn dipole_flector_regressive_product(self_492: Dipole, other_426: Flector) -> RadialPoint {
    var self_493: Dipole;
    var other_427: Flector;

    self_493 = self_492;
    other_427 = other_426;
    let _e4: Dipole = self_493;
    let _e8: Flector = other_427;
    let _e11: Flector = other_427;
    let _e14: Flector = other_427;
    let _e25: Dipole = self_493;
    let _e29: Flector = other_427;
    let _e32: Flector = other_427;
    let _e35: Flector = other_427;
    let _e47: Dipole = self_493;
    let _e51: Flector = other_427;
    let _e54: Flector = other_427;
    let _e57: Flector = other_427;
    let _e69: Dipole = self_493;
    let _e71: Flector = other_427;
    let _e77: Dipole = self_493;
    let _e81: Flector = other_427;
    let _e91: Dipole = self_493;
    let _e95: Flector = other_427;
    let _e106: Dipole = self_493;
    let _e110: Flector = other_427;
    let _e120: Dipole = self_493;
    let _e124: Flector = other_427;
    let _e134: Dipole = self_493;
    let _e138: Flector = other_427;
    let _e148: Dipole = self_493;
    let _e152: Flector = other_427;
    let _e162: Dipole = self_493;
    let _e166: Flector = other_427;
    return RadialPoint((((((vec3<f32>(_e4.g1_.x) * vec3<f32>(_e8.g1_.z, _e11.g1_.z, _e14.g1_.y)) * vec3<f32>(0.0, -(1.0), 1.0)) + ((vec3<f32>(_e25.g1_.y) * vec3<f32>(_e29.g1_.z, _e32.g1_.z, _e35.g1_.x)) * vec3<f32>(1.0, 0.0, -(1.0)))) + ((vec3<f32>(_e47.g1_.z) * vec3<f32>(_e51.g1_.y, _e54.g1_.x, _e57.g1_.y)) * vec3<f32>(-(1.0), 1.0, 0.0))) + (_e69.g0_ * vec3<f32>(_e71.g1_.w))), ((((((((vec2<f32>(_e77.g0_.y) * vec2<f32>(_e81.g1_.y)) * vec2<f32>(-(1.0), 0.0)) + ((vec2<f32>(_e91.g0_.z) * vec2<f32>(_e95.g1_.z)) * vec2<f32>(-(1.0), 0.0))) + ((vec2<f32>(_e106.g2_.x) * vec2<f32>(_e110.g1_.x)) * vec2<f32>(0.0, 1.0))) + ((vec2<f32>(_e120.g2_.y) * vec2<f32>(_e124.g1_.y)) * vec2<f32>(0.0, 1.0))) + ((vec2<f32>(_e134.g2_.z) * vec2<f32>(_e138.g1_.z)) * vec2<f32>(0.0, 1.0))) + ((vec2<f32>(_e148.g2_.w) * vec2<f32>(_e152.g1_.w)) * vec2<f32>(0.0, 1.0))) + ((vec2<f32>(_e162.g0_.x) * vec2<f32>(_e166.g1_.x)) * vec2<f32>(-(1.0), 0.0))));
}

fn dipole_flector_outer_product(self_494: Dipole, other_428: Flector) -> Plane {
    var self_495: Dipole;
    var other_429: Flector;

    self_495 = self_494;
    other_429 = other_428;
    let _e4: Dipole = self_495;
    let _e8: Flector = other_429;
    let _e19: Dipole = self_495;
    let _e23: Flector = other_429;
    let _e35: Dipole = self_495;
    let _e39: Flector = other_429;
    let _e51: Dipole = self_495;
    let _e55: Flector = other_429;
    let _e67: Dipole = self_495;
    let _e70: Dipole = self_495;
    let _e73: Dipole = self_495;
    let _e76: Dipole = self_495;
    let _e80: Flector = other_429;
    return Plane(((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.zzxz) * vec4<f32>(1.0, 0.0, -(1.0), 0.0)) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))) + ((vec4<f32>(_e35.g1_.y) * _e39.g0_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0)))) + ((vec4<f32>(_e51.g1_.z) * _e55.g0_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e67.g1_.x, _e70.g0_.x, _e73.g0_.x, _e76.g1_.x) * _e80.g0_.wzyx) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))));
}

fn dipole_flector_inner_product(self_496: Dipole, other_430: Flector) -> FlatPoint {
    var self_497: Dipole;
    var other_431: Flector;

    self_497 = self_496;
    other_431 = other_430;
    let _e4: Dipole = self_497;
    let _e8: Flector = other_431;
    let _e19: Dipole = self_497;
    let _e23: Flector = other_431;
    let _e35: Dipole = self_497;
    let _e39: Flector = other_431;
    return FlatPoint(((((vec4<f32>(_e4.g1_.y) * _e8.g1_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e19.g1_.z) * _e23.g1_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e35.g1_.x) * _e39.g1_.wxxx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))));
}

fn dipole_flector_left_contraction(self_498: Dipole, other_432: Flector) -> FlatPoint {
    var self_499: Dipole;
    var other_433: Flector;

    self_499 = self_498;
    other_433 = other_432;
    let _e4: Dipole = self_499;
    let _e8: Flector = other_433;
    let _e19: Dipole = self_499;
    let _e23: Flector = other_433;
    let _e35: Dipole = self_499;
    let _e39: Flector = other_433;
    return FlatPoint(((((vec4<f32>(_e4.g1_.y) * _e8.g1_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e19.g1_.z) * _e23.g1_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e35.g1_.x) * _e39.g1_.wxxx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))));
}

fn dipole_multi_vector_add(self_500: Dipole, other_434: MultiVector) -> MultiVector {
    var self_501: Dipole;
    var other_435: MultiVector;

    self_501 = self_500;
    other_435 = other_434;
    let _e4: MultiVector = other_435;
    let _e6: MultiVector = other_435;
    let _e8: MultiVector = other_435;
    let _e10: Dipole = self_501;
    let _e12: MultiVector = other_435;
    let _e15: Dipole = self_501;
    let _e17: MultiVector = other_435;
    let _e20: Dipole = self_501;
    let _e22: MultiVector = other_435;
    let _e25: MultiVector = other_435;
    let _e27: MultiVector = other_435;
    let _e29: MultiVector = other_435;
    let _e31: MultiVector = other_435;
    return MultiVector(_e4.g0_, _e6.g1_, _e8.g2_, (_e10.g2_ + _e12.g3_), (_e15.g0_ + _e17.g4_), (_e20.g1_ + _e22.g5_), _e25.g6_, _e27.g7_, _e29.g8_, _e31.g9_);
}

fn dipole_multi_vector_sub(self_502: Dipole, other_436: MultiVector) -> MultiVector {
    var self_503: Dipole;
    var other_437: MultiVector;

    self_503 = self_502;
    other_437 = other_436;
    let _e6: MultiVector = other_437;
    let _e11: MultiVector = other_437;
    let _e16: MultiVector = other_437;
    let _e19: Dipole = self_503;
    let _e21: MultiVector = other_437;
    let _e24: Dipole = self_503;
    let _e26: MultiVector = other_437;
    let _e29: Dipole = self_503;
    let _e31: MultiVector = other_437;
    let _e36: MultiVector = other_437;
    let _e41: MultiVector = other_437;
    let _e46: MultiVector = other_437;
    let _e51: MultiVector = other_437;
    return MultiVector((vec3<f32>(0.0) - _e6.g0_), (vec3<f32>(0.0) - _e11.g1_), (vec2<f32>(0.0) - _e16.g2_), (_e19.g2_ - _e21.g3_), (_e24.g0_ - _e26.g4_), (_e29.g1_ - _e31.g5_), (vec3<f32>(0.0) - _e36.g6_), (vec3<f32>(0.0) - _e41.g7_), (vec4<f32>(0.0) - _e46.g8_), (vec4<f32>(0.0) - _e51.g9_));
}

fn dipole_multi_vector_geometric_product(self_504: Dipole, other_438: MultiVector) -> MultiVector {
    var self_505: Dipole;
    var other_439: MultiVector;

    self_505 = self_504;
    other_439 = other_438;
    let _e4: Dipole = self_505;
    let _e8: MultiVector = other_439;
    let _e11: MultiVector = other_439;
    let _e14: MultiVector = other_439;
    let _e26: Dipole = self_505;
    let _e30: MultiVector = other_439;
    let _e33: MultiVector = other_439;
    let _e36: MultiVector = other_439;
    let _e49: Dipole = self_505;
    let _e53: MultiVector = other_439;
    let _e56: MultiVector = other_439;
    let _e59: MultiVector = other_439;
    let _e65: Dipole = self_505;
    let _e69: MultiVector = other_439;
    let _e72: MultiVector = other_439;
    let _e75: MultiVector = other_439;
    let _e81: Dipole = self_505;
    let _e85: MultiVector = other_439;
    let _e88: MultiVector = other_439;
    let _e91: MultiVector = other_439;
    let _e97: Dipole = self_505;
    let _e101: MultiVector = other_439;
    let _e113: Dipole = self_505;
    let _e117: MultiVector = other_439;
    let _e129: Dipole = self_505;
    let _e133: MultiVector = other_439;
    let _e145: Dipole = self_505;
    let _e149: MultiVector = other_439;
    let _e161: Dipole = self_505;
    let _e165: MultiVector = other_439;
    let _e168: MultiVector = other_439;
    let _e171: MultiVector = other_439;
    let _e184: Dipole = self_505;
    let _e188: MultiVector = other_439;
    let _e191: MultiVector = other_439;
    let _e194: MultiVector = other_439;
    let _e205: Dipole = self_505;
    let _e209: MultiVector = other_439;
    let _e212: MultiVector = other_439;
    let _e215: MultiVector = other_439;
    let _e227: Dipole = self_505;
    let _e231: MultiVector = other_439;
    let _e234: MultiVector = other_439;
    let _e237: MultiVector = other_439;
    let _e249: Dipole = self_505;
    let _e253: MultiVector = other_439;
    let _e262: Dipole = self_505;
    let _e266: MultiVector = other_439;
    let _e276: Dipole = self_505;
    let _e280: MultiVector = other_439;
    let _e283: MultiVector = other_439;
    let _e289: Dipole = self_505;
    let _e293: MultiVector = other_439;
    let _e296: MultiVector = other_439;
    let _e302: Dipole = self_505;
    let _e306: MultiVector = other_439;
    let _e309: MultiVector = other_439;
    let _e315: Dipole = self_505;
    let _e319: MultiVector = other_439;
    let _e330: Dipole = self_505;
    let _e334: MultiVector = other_439;
    let _e345: Dipole = self_505;
    let _e349: MultiVector = other_439;
    let _e360: Dipole = self_505;
    let _e364: MultiVector = other_439;
    let _e374: Dipole = self_505;
    let _e378: MultiVector = other_439;
    let _e389: Dipole = self_505;
    let _e393: MultiVector = other_439;
    let _e405: Dipole = self_505;
    let _e409: MultiVector = other_439;
    let _e412: MultiVector = other_439;
    let _e415: MultiVector = other_439;
    let _e418: MultiVector = other_439;
    let _e432: Dipole = self_505;
    let _e436: MultiVector = other_439;
    let _e439: MultiVector = other_439;
    let _e442: MultiVector = other_439;
    let _e445: MultiVector = other_439;
    let _e459: Dipole = self_505;
    let _e463: MultiVector = other_439;
    let _e466: MultiVector = other_439;
    let _e469: MultiVector = other_439;
    let _e472: MultiVector = other_439;
    let _e486: Dipole = self_505;
    let _e490: MultiVector = other_439;
    let _e493: MultiVector = other_439;
    let _e496: MultiVector = other_439;
    let _e499: MultiVector = other_439;
    let _e513: Dipole = self_505;
    let _e517: MultiVector = other_439;
    let _e520: MultiVector = other_439;
    let _e523: MultiVector = other_439;
    let _e526: MultiVector = other_439;
    let _e540: Dipole = self_505;
    let _e544: MultiVector = other_439;
    let _e547: MultiVector = other_439;
    let _e550: MultiVector = other_439;
    let _e553: MultiVector = other_439;
    let _e567: Dipole = self_505;
    let _e571: MultiVector = other_439;
    let _e583: Dipole = self_505;
    let _e587: MultiVector = other_439;
    let _e599: Dipole = self_505;
    let _e603: MultiVector = other_439;
    let _e606: MultiVector = other_439;
    let _e609: MultiVector = other_439;
    let _e620: Dipole = self_505;
    let _e624: MultiVector = other_439;
    let _e627: MultiVector = other_439;
    let _e630: MultiVector = other_439;
    let _e642: Dipole = self_505;
    let _e646: MultiVector = other_439;
    let _e649: MultiVector = other_439;
    let _e652: MultiVector = other_439;
    let _e664: Dipole = self_505;
    let _e668: MultiVector = other_439;
    let _e671: MultiVector = other_439;
    let _e674: MultiVector = other_439;
    let _e686: Dipole = self_505;
    let _e690: MultiVector = other_439;
    let _e693: MultiVector = other_439;
    let _e696: MultiVector = other_439;
    let _e708: Dipole = self_505;
    let _e712: MultiVector = other_439;
    let _e715: MultiVector = other_439;
    let _e718: MultiVector = other_439;
    let _e730: Dipole = self_505;
    let _e734: MultiVector = other_439;
    let _e737: MultiVector = other_439;
    let _e740: MultiVector = other_439;
    let _e751: Dipole = self_505;
    let _e755: MultiVector = other_439;
    let _e758: MultiVector = other_439;
    let _e761: MultiVector = other_439;
    let _e773: Dipole = self_505;
    let _e777: MultiVector = other_439;
    let _e780: MultiVector = other_439;
    let _e783: MultiVector = other_439;
    let _e795: Dipole = self_505;
    let _e799: MultiVector = other_439;
    let _e802: MultiVector = other_439;
    let _e805: MultiVector = other_439;
    let _e816: Dipole = self_505;
    let _e820: MultiVector = other_439;
    let _e823: MultiVector = other_439;
    let _e826: MultiVector = other_439;
    let _e838: Dipole = self_505;
    let _e842: MultiVector = other_439;
    let _e845: MultiVector = other_439;
    let _e848: MultiVector = other_439;
    let _e860: Dipole = self_505;
    let _e864: MultiVector = other_439;
    let _e867: MultiVector = other_439;
    let _e870: MultiVector = other_439;
    let _e882: Dipole = self_505;
    let _e886: MultiVector = other_439;
    let _e889: MultiVector = other_439;
    let _e892: MultiVector = other_439;
    let _e904: Dipole = self_505;
    let _e908: MultiVector = other_439;
    let _e911: MultiVector = other_439;
    let _e914: MultiVector = other_439;
    let _e926: Dipole = self_505;
    let _e930: MultiVector = other_439;
    let _e933: MultiVector = other_439;
    let _e936: MultiVector = other_439;
    let _e948: Dipole = self_505;
    let _e952: MultiVector = other_439;
    let _e955: MultiVector = other_439;
    let _e958: MultiVector = other_439;
    let _e970: Dipole = self_505;
    let _e974: MultiVector = other_439;
    let _e977: MultiVector = other_439;
    let _e980: MultiVector = other_439;
    let _e992: Dipole = self_505;
    let _e996: MultiVector = other_439;
    let _e1000: Dipole = self_505;
    let _e1004: MultiVector = other_439;
    let _e1007: MultiVector = other_439;
    let _e1010: MultiVector = other_439;
    let _e1021: Dipole = self_505;
    let _e1025: MultiVector = other_439;
    let _e1028: MultiVector = other_439;
    let _e1031: MultiVector = other_439;
    let _e1043: Dipole = self_505;
    let _e1047: MultiVector = other_439;
    let _e1050: MultiVector = other_439;
    let _e1053: MultiVector = other_439;
    let _e1065: Dipole = self_505;
    let _e1069: MultiVector = other_439;
    let _e1072: MultiVector = other_439;
    let _e1075: MultiVector = other_439;
    let _e1087: Dipole = self_505;
    let _e1091: MultiVector = other_439;
    let _e1094: MultiVector = other_439;
    let _e1097: MultiVector = other_439;
    let _e1109: Dipole = self_505;
    let _e1113: MultiVector = other_439;
    let _e1116: MultiVector = other_439;
    let _e1119: MultiVector = other_439;
    let _e1131: Dipole = self_505;
    let _e1135: MultiVector = other_439;
    let _e1138: MultiVector = other_439;
    let _e1141: MultiVector = other_439;
    let _e1144: MultiVector = other_439;
    let _e1157: Dipole = self_505;
    let _e1161: MultiVector = other_439;
    let _e1164: MultiVector = other_439;
    let _e1167: MultiVector = other_439;
    let _e1170: MultiVector = other_439;
    let _e1184: Dipole = self_505;
    let _e1188: MultiVector = other_439;
    let _e1191: MultiVector = other_439;
    let _e1194: MultiVector = other_439;
    let _e1197: MultiVector = other_439;
    let _e1211: Dipole = self_505;
    let _e1215: MultiVector = other_439;
    let _e1218: MultiVector = other_439;
    let _e1221: MultiVector = other_439;
    let _e1224: MultiVector = other_439;
    let _e1238: Dipole = self_505;
    let _e1242: MultiVector = other_439;
    let _e1245: MultiVector = other_439;
    let _e1248: MultiVector = other_439;
    let _e1251: MultiVector = other_439;
    let _e1265: Dipole = self_505;
    let _e1269: MultiVector = other_439;
    let _e1272: MultiVector = other_439;
    let _e1275: MultiVector = other_439;
    let _e1278: MultiVector = other_439;
    let _e1292: Dipole = self_505;
    let _e1296: MultiVector = other_439;
    let _e1299: MultiVector = other_439;
    let _e1302: MultiVector = other_439;
    let _e1305: MultiVector = other_439;
    let _e1318: Dipole = self_505;
    let _e1322: MultiVector = other_439;
    let _e1325: MultiVector = other_439;
    let _e1328: MultiVector = other_439;
    let _e1331: MultiVector = other_439;
    let _e1345: Dipole = self_505;
    let _e1349: MultiVector = other_439;
    let _e1352: MultiVector = other_439;
    let _e1355: MultiVector = other_439;
    let _e1358: MultiVector = other_439;
    let _e1372: Dipole = self_505;
    let _e1376: MultiVector = other_439;
    let _e1379: MultiVector = other_439;
    let _e1382: MultiVector = other_439;
    let _e1385: MultiVector = other_439;
    let _e1399: Dipole = self_505;
    let _e1403: MultiVector = other_439;
    let _e1406: MultiVector = other_439;
    let _e1409: MultiVector = other_439;
    let _e1412: MultiVector = other_439;
    let _e1426: Dipole = self_505;
    let _e1430: MultiVector = other_439;
    let _e1433: MultiVector = other_439;
    let _e1436: MultiVector = other_439;
    let _e1439: MultiVector = other_439;
    let _e1453: Dipole = self_505;
    let _e1457: MultiVector = other_439;
    let _e1460: MultiVector = other_439;
    let _e1463: MultiVector = other_439;
    let _e1466: MultiVector = other_439;
    let _e1480: Dipole = self_505;
    let _e1484: MultiVector = other_439;
    let _e1487: MultiVector = other_439;
    let _e1490: MultiVector = other_439;
    let _e1493: MultiVector = other_439;
    let _e1507: Dipole = self_505;
    let _e1511: MultiVector = other_439;
    let _e1514: MultiVector = other_439;
    let _e1517: MultiVector = other_439;
    let _e1520: MultiVector = other_439;
    let _e1532: Dipole = self_505;
    let _e1536: MultiVector = other_439;
    let _e1539: MultiVector = other_439;
    let _e1542: MultiVector = other_439;
    let _e1545: MultiVector = other_439;
    return MultiVector((((((((((((vec3<f32>(_e4.g0_.y) * vec3<f32>(_e8.g5_.y, _e11.g5_.y, _e14.g7_.y)) * vec3<f32>(0.0, -(1.0), -(1.0))) + ((vec3<f32>(_e26.g0_.z) * vec3<f32>(_e30.g5_.z, _e33.g5_.z, _e36.g7_.z)) * vec3<f32>(0.0, -(1.0), -(1.0)))) - (vec3<f32>(_e49.g1_.x) * vec3<f32>(_e53.g5_.x, _e56.g4_.x, _e59.g6_.x))) - (vec3<f32>(_e65.g1_.y) * vec3<f32>(_e69.g5_.y, _e72.g4_.y, _e75.g6_.y))) - (vec3<f32>(_e81.g1_.z) * vec3<f32>(_e85.g5_.z, _e88.g4_.z, _e91.g6_.z))) + ((vec3<f32>(_e97.g2_.x) * vec3<f32>(_e101.g8_.x)) * vec3<f32>(0.0, 0.0, -(1.0)))) + ((vec3<f32>(_e113.g2_.y) * vec3<f32>(_e117.g8_.y)) * vec3<f32>(0.0, 0.0, -(1.0)))) + ((vec3<f32>(_e129.g2_.z) * vec3<f32>(_e133.g8_.z)) * vec3<f32>(0.0, 0.0, -(1.0)))) + ((vec3<f32>(_e145.g2_.w) * vec3<f32>(_e149.g8_.w)) * vec3<f32>(0.0, 0.0, -(1.0)))) + ((vec3<f32>(_e161.g0_.x) * vec3<f32>(_e165.g5_.x, _e168.g5_.x, _e171.g7_.x)) * vec3<f32>(0.0, -(1.0), -(1.0)))), ((((vec3<f32>(_e184.g1_.x) * vec3<f32>(_e188.g8_.w, _e191.g1_.z, _e194.g1_.y)) * vec3<f32>(1.0, 1.0, -(1.0))) + ((vec3<f32>(_e205.g1_.y) * vec3<f32>(_e209.g1_.z, _e212.g8_.w, _e215.g1_.x)) * vec3<f32>(-(1.0), 1.0, 1.0))) + ((vec3<f32>(_e227.g1_.z) * vec3<f32>(_e231.g1_.y, _e234.g1_.x, _e237.g8_.w)) * vec3<f32>(1.0, -(1.0), 1.0))), ((((((((((vec2<f32>(_e249.g0_.y) * vec2<f32>(_e253.g1_.y)) * vec2<f32>(1.0, 0.0)) + ((vec2<f32>(_e262.g0_.z) * vec2<f32>(_e266.g1_.z)) * vec2<f32>(1.0, 0.0))) - (vec2<f32>(_e276.g1_.x) * vec2<f32>(_e280.g8_.x, _e283.g7_.x))) - (vec2<f32>(_e289.g1_.y) * vec2<f32>(_e293.g8_.y, _e296.g7_.y))) - (vec2<f32>(_e302.g1_.z) * vec2<f32>(_e306.g8_.z, _e309.g7_.z))) + ((vec2<f32>(_e315.g2_.x) * vec2<f32>(_e319.g1_.x)) * vec2<f32>(0.0, -(1.0)))) + ((vec2<f32>(_e330.g2_.y) * vec2<f32>(_e334.g1_.y)) * vec2<f32>(0.0, -(1.0)))) + ((vec2<f32>(_e345.g2_.z) * vec2<f32>(_e349.g1_.z)) * vec2<f32>(0.0, -(1.0)))) + ((vec2<f32>(_e360.g0_.x) * vec2<f32>(_e364.g1_.x)) * vec2<f32>(1.0, 0.0))), (((((((((((vec4<f32>(_e374.g0_.y) * vec4<f32>(_e378.g3_.y)) * vec4<f32>(0.0, 0.0, 0.0, 1.0)) + ((vec4<f32>(_e389.g0_.z) * vec4<f32>(_e393.g3_.z)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e405.g1_.x) * vec4<f32>(_e409.g9_.w, _e412.g3_.z, _e415.g3_.y, _e418.g9_.x)) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e432.g1_.y) * vec4<f32>(_e436.g3_.z, _e439.g9_.w, _e442.g3_.x, _e445.g9_.y)) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e459.g1_.z) * vec4<f32>(_e463.g3_.y, _e466.g3_.x, _e469.g9_.w, _e472.g9_.z)) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e486.g2_.x) * vec4<f32>(_e490.g0_.x, _e493.g5_.z, _e496.g5_.y, _e499.g4_.x)) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e513.g2_.y) * vec4<f32>(_e517.g5_.z, _e520.g0_.x, _e523.g5_.x, _e526.g4_.y)) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e540.g2_.z) * vec4<f32>(_e544.g5_.y, _e547.g5_.x, _e550.g0_.x, _e553.g4_.z)) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e567.g2_.w) * vec4<f32>(_e571.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e583.g0_.x) * vec4<f32>(_e587.g3_.x)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))), (((((((vec3<f32>(_e599.g0_.x) * vec3<f32>(_e603.g0_.x, _e606.g5_.z, _e609.g5_.y)) * vec3<f32>(1.0, 1.0, -(1.0))) + ((vec3<f32>(_e620.g0_.y) * vec3<f32>(_e624.g5_.z, _e627.g0_.x, _e630.g5_.x)) * vec3<f32>(-(1.0), 1.0, 1.0))) + ((vec3<f32>(_e642.g0_.z) * vec3<f32>(_e646.g5_.y, _e649.g5_.x, _e652.g0_.x)) * vec3<f32>(1.0, -(1.0), 1.0))) + ((vec3<f32>(_e664.g1_.x) * vec3<f32>(_e668.g0_.y, _e671.g4_.z, _e674.g4_.y)) * vec3<f32>(1.0, 1.0, -(1.0)))) + ((vec3<f32>(_e686.g1_.y) * vec3<f32>(_e690.g4_.z, _e693.g0_.y, _e696.g4_.x)) * vec3<f32>(-(1.0), 1.0, 1.0))) + ((vec3<f32>(_e708.g1_.z) * vec3<f32>(_e712.g4_.y, _e715.g4_.x, _e718.g0_.y)) * vec3<f32>(1.0, -(1.0), 1.0))), ((((vec3<f32>(_e730.g1_.x) * vec3<f32>(_e734.g0_.x, _e737.g5_.z, _e740.g5_.y)) * vec3<f32>(1.0, 1.0, -(1.0))) + ((vec3<f32>(_e751.g1_.y) * vec3<f32>(_e755.g5_.z, _e758.g0_.x, _e761.g5_.x)) * vec3<f32>(-(1.0), 1.0, 1.0))) + ((vec3<f32>(_e773.g1_.z) * vec3<f32>(_e777.g5_.y, _e780.g5_.x, _e783.g0_.x)) * vec3<f32>(1.0, -(1.0), 1.0))), (((((((((((vec3<f32>(_e795.g0_.x) * vec3<f32>(_e799.g2_.y, _e802.g7_.z, _e805.g7_.y)) * vec3<f32>(1.0, 1.0, -(1.0))) + ((vec3<f32>(_e816.g0_.y) * vec3<f32>(_e820.g7_.z, _e823.g2_.y, _e826.g7_.x)) * vec3<f32>(-(1.0), 1.0, 1.0))) + ((vec3<f32>(_e838.g0_.z) * vec3<f32>(_e842.g7_.y, _e845.g7_.x, _e848.g2_.y)) * vec3<f32>(1.0, -(1.0), 1.0))) + ((vec3<f32>(_e860.g1_.x) * vec3<f32>(_e864.g0_.z, _e867.g6_.z, _e870.g6_.y)) * vec3<f32>(1.0, 1.0, -(1.0)))) + ((vec3<f32>(_e882.g1_.y) * vec3<f32>(_e886.g6_.z, _e889.g0_.z, _e892.g6_.x)) * vec3<f32>(-(1.0), 1.0, 1.0))) + ((vec3<f32>(_e904.g1_.z) * vec3<f32>(_e908.g6_.y, _e911.g6_.x, _e914.g0_.z)) * vec3<f32>(1.0, -(1.0), 1.0))) + ((vec3<f32>(_e926.g2_.x) * vec3<f32>(_e930.g2_.x, _e933.g8_.z, _e936.g8_.y)) * vec3<f32>(1.0, 1.0, -(1.0)))) + ((vec3<f32>(_e948.g2_.y) * vec3<f32>(_e952.g8_.z, _e955.g2_.x, _e958.g8_.x)) * vec3<f32>(-(1.0), 1.0, 1.0))) + ((vec3<f32>(_e970.g2_.z) * vec3<f32>(_e974.g8_.y, _e977.g8_.x, _e980.g2_.x)) * vec3<f32>(1.0, -(1.0), 1.0))) - (vec3<f32>(_e992.g2_.w) * _e996.g1_)), (((((((vec3<f32>(_e1000.g1_.x) * vec3<f32>(_e1004.g2_.y, _e1007.g7_.z, _e1010.g7_.y)) * vec3<f32>(1.0, 1.0, -(1.0))) + ((vec3<f32>(_e1021.g1_.y) * vec3<f32>(_e1025.g7_.z, _e1028.g2_.y, _e1031.g7_.x)) * vec3<f32>(-(1.0), 1.0, 1.0))) + ((vec3<f32>(_e1043.g1_.z) * vec3<f32>(_e1047.g7_.y, _e1050.g7_.x, _e1053.g2_.y)) * vec3<f32>(1.0, -(1.0), 1.0))) + ((vec3<f32>(_e1065.g2_.x) * vec3<f32>(_e1069.g8_.w, _e1072.g1_.z, _e1075.g1_.y)) * vec3<f32>(1.0, 1.0, -(1.0)))) + ((vec3<f32>(_e1087.g2_.y) * vec3<f32>(_e1091.g1_.z, _e1094.g8_.w, _e1097.g1_.x)) * vec3<f32>(-(1.0), 1.0, 1.0))) + ((vec3<f32>(_e1109.g2_.z) * vec3<f32>(_e1113.g1_.y, _e1116.g1_.x, _e1119.g8_.w)) * vec3<f32>(1.0, -(1.0), 1.0))), (((((((vec4<f32>(_e1131.g0_.y) * vec4<f32>(_e1135.g1_.z, _e1138.g8_.w, _e1141.g1_.x, _e1144.g1_.z)) * vec4<f32>(1.0, -(1.0), -(1.0), 0.0)) + ((vec4<f32>(_e1157.g0_.z) * vec4<f32>(_e1161.g1_.y, _e1164.g1_.x, _e1167.g8_.w, _e1170.g1_.y)) * vec4<f32>(-(1.0), 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e1184.g1_.x) * vec4<f32>(_e1188.g2_.x, _e1191.g8_.z, _e1194.g8_.y, _e1197.g1_.x)) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e1211.g1_.y) * vec4<f32>(_e1215.g8_.z, _e1218.g2_.x, _e1221.g8_.x, _e1224.g1_.y)) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e1238.g1_.z) * vec4<f32>(_e1242.g8_.y, _e1245.g8_.x, _e1248.g2_.x, _e1251.g1_.z)) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e1265.g0_.x) * vec4<f32>(_e1269.g8_.w, _e1272.g1_.z, _e1275.g1_.y, _e1278.g8_.x)) * vec4<f32>(-(1.0), -(1.0), 1.0, 0.0))), (((((((((((vec4<f32>(_e1292.g0_.y) * vec4<f32>(_e1296.g3_.z, _e1299.g9_.w, _e1302.g3_.x, _e1305.g3_.z)) * vec4<f32>(1.0, -(1.0), -(1.0), 0.0)) + ((vec4<f32>(_e1318.g0_.z) * vec4<f32>(_e1322.g3_.y, _e1325.g3_.x, _e1328.g9_.w, _e1331.g3_.y)) * vec4<f32>(-(1.0), 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e1345.g1_.x) * vec4<f32>(_e1349.g3_.w, _e1352.g9_.z, _e1355.g9_.y, _e1358.g3_.x)) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e1372.g1_.y) * vec4<f32>(_e1376.g9_.z, _e1379.g3_.w, _e1382.g9_.x, _e1385.g3_.y)) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e1399.g1_.z) * vec4<f32>(_e1403.g9_.y, _e1406.g9_.x, _e1409.g3_.w, _e1412.g3_.z)) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e1426.g2_.x) * vec4<f32>(_e1430.g0_.y, _e1433.g4_.z, _e1436.g4_.y, _e1439.g5_.x)) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e1453.g2_.y) * vec4<f32>(_e1457.g4_.z, _e1460.g0_.y, _e1463.g4_.x, _e1466.g5_.y)) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e1480.g2_.z) * vec4<f32>(_e1484.g4_.y, _e1487.g4_.x, _e1490.g0_.y, _e1493.g5_.z)) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e1507.g2_.w) * vec4<f32>(_e1511.g5_.x, _e1514.g5_.y, _e1517.g5_.z, _e1520.g5_.x)) * vec4<f32>(1.0, 1.0, 1.0, 0.0))) + ((vec4<f32>(_e1532.g0_.x) * vec4<f32>(_e1536.g9_.w, _e1539.g3_.z, _e1542.g3_.y, _e1545.g9_.x)) * vec4<f32>(-(1.0), -(1.0), 1.0, 0.0))));
}

fn dipole_multi_vector_scalar_product(self_506: Dipole, other_440: MultiVector) -> Scalar {
    var self_507: Dipole;
    var other_441: MultiVector;

    self_507 = self_506;
    other_441 = other_440;
    let _e5: Dipole = self_507;
    let _e8: MultiVector = other_441;
    let _e13: Dipole = self_507;
    let _e16: MultiVector = other_441;
    let _e21: Dipole = self_507;
    let _e24: MultiVector = other_441;
    return Scalar((((0.0 - (_e5.g1_.x * _e8.g5_.x)) - (_e13.g1_.y * _e16.g5_.y)) - (_e21.g1_.z * _e24.g5_.z)));
}

fn dipole_squared_magnitude(self_508: Dipole) -> Scalar {
    var self_509: Dipole;

    self_509 = self_508;
    let _e2: Dipole = self_509;
    let _e3: Dipole = self_509;
    let _e4: Dipole = dipole_reversal(_e3);
    let _e5: Scalar = dipole_dipole_scalar_product(_e2, _e4);
    return _e5;
}

fn dipole_magnitude(self_510: Dipole) -> Scalar {
    var self_511: Dipole;

    self_511 = self_510;
    let _e2: Dipole = self_511;
    let _e3: Scalar = dipole_squared_magnitude(_e2);
    return Scalar(sqrt(_e3.g0_));
}

fn dipole_scale(self_512: Dipole, other_442: f32) -> Dipole {
    var self_513: Dipole;
    var other_443: f32;

    self_513 = self_512;
    other_443 = other_442;
    let _e4: Dipole = self_513;
    let _e5: f32 = other_443;
    let _e7: Dipole = dipole_scalar_geometric_product(_e4, Scalar(_e5));
    return _e7;
}

fn dipole_signum(self_514: Dipole) -> Dipole {
    var self_515: Dipole;

    self_515 = self_514;
    let _e2: Dipole = self_515;
    let _e3: Dipole = self_515;
    let _e4: Scalar = dipole_magnitude(_e3);
    let _e9: Dipole = dipole_scalar_geometric_product(_e2, Scalar((1.0 / _e4.g0_)));
    return _e9;
}

fn dipole_inverse(self_516: Dipole) -> Dipole {
    var self_517: Dipole;

    self_517 = self_516;
    let _e2: Dipole = self_517;
    let _e3: Dipole = dipole_reversal(_e2);
    let _e4: Dipole = self_517;
    let _e5: Scalar = dipole_squared_magnitude(_e4);
    let _e10: Dipole = dipole_scalar_geometric_product(_e3, Scalar((1.0 / _e5.g0_)));
    return _e10;
}

fn line_zero() -> Line {
    return Line(vec3<f32>(0.0), vec3<f32>(0.0));
}

fn line_one() -> Line {
    return Line(vec3<f32>(0.0), vec3<f32>(0.0));
}

fn line_neg(self_518: Line) -> Line {
    var self_519: Line;

    self_519 = self_518;
    let _e2: Line = self_519;
    let _e8: Line = self_519;
    return Line((_e2.g0_ * vec3<f32>(-(1.0))), (_e8.g1_ * vec3<f32>(-(1.0))));
}

fn line_automorphism(self_520: Line) -> Line {
    var self_521: Line;

    self_521 = self_520;
    let _e2: Line = self_521;
    let _e8: Line = self_521;
    return Line((_e2.g0_ * vec3<f32>(-(1.0))), (_e8.g1_ * vec3<f32>(-(1.0))));
}

fn line_reversal(self_522: Line) -> Line {
    var self_523: Line;

    self_523 = self_522;
    let _e2: Line = self_523;
    let _e8: Line = self_523;
    return Line((_e2.g0_ * vec3<f32>(-(1.0))), (_e8.g1_ * vec3<f32>(-(1.0))));
}

fn line_conjugation(self_524: Line) -> Line {
    var self_525: Line;

    self_525 = self_524;
    let _e2: Line = self_525;
    let _e4: Line = self_525;
    return Line(_e2.g0_, _e4.g1_);
}

fn line_scalar_geometric_product(self_526: Line, other_444: Scalar) -> Line {
    var self_527: Line;
    var other_445: Scalar;

    self_527 = self_526;
    other_445 = other_444;
    let _e4: Line = self_527;
    let _e6: Scalar = other_445;
    let _e10: Line = self_527;
    let _e12: Scalar = other_445;
    return Line((_e4.g0_ * vec3<f32>(_e6.g0_)), (_e10.g1_ * vec3<f32>(_e12.g0_)));
}

fn line_scalar_outer_product(self_528: Line, other_446: Scalar) -> Line {
    var self_529: Line;
    var other_447: Scalar;

    self_529 = self_528;
    other_447 = other_446;
    let _e4: Line = self_529;
    let _e6: Scalar = other_447;
    let _e10: Line = self_529;
    let _e12: Scalar = other_447;
    return Line((_e4.g0_ * vec3<f32>(_e6.g0_)), (_e10.g1_ * vec3<f32>(_e12.g0_)));
}

fn line_scalar_inner_product(self_530: Line, other_448: Scalar) -> Line {
    var self_531: Line;
    var other_449: Scalar;

    self_531 = self_530;
    other_449 = other_448;
    let _e4: Line = self_531;
    let _e6: Scalar = other_449;
    let _e10: Line = self_531;
    let _e12: Scalar = other_449;
    return Line((_e4.g0_ * vec3<f32>(_e6.g0_)), (_e10.g1_ * vec3<f32>(_e12.g0_)));
}

fn line_scalar_right_contraction(self_532: Line, other_450: Scalar) -> Line {
    var self_533: Line;
    var other_451: Scalar;

    self_533 = self_532;
    other_451 = other_450;
    let _e4: Line = self_533;
    let _e6: Scalar = other_451;
    let _e10: Line = self_533;
    let _e12: Scalar = other_451;
    return Line((_e4.g0_ * vec3<f32>(_e6.g0_)), (_e10.g1_ * vec3<f32>(_e12.g0_)));
}

fn line_anti_scalar_regressive_product(self_534: Line, other_452: AntiScalar) -> Line {
    var self_535: Line;
    var other_453: AntiScalar;

    self_535 = self_534;
    other_453 = other_452;
    let _e4: Line = self_535;
    let _e6: AntiScalar = other_453;
    let _e10: Line = self_535;
    let _e12: AntiScalar = other_453;
    return Line((_e4.g0_ * vec3<f32>(_e6.g0_)), (_e10.g1_ * vec3<f32>(_e12.g0_)));
}

fn line_radial_point_geometric_product(self_536: Line, other_454: RadialPoint) -> Flector {
    var self_537: Line;
    var other_455: RadialPoint;

    self_537 = self_536;
    other_455 = other_454;
    let _e4: Line = self_537;
    let _e8: RadialPoint = other_455;
    let _e20: Line = self_537;
    let _e24: RadialPoint = other_455;
    let _e37: Line = self_537;
    let _e41: RadialPoint = other_455;
    let _e44: RadialPoint = other_455;
    let _e47: RadialPoint = other_455;
    let _e50: RadialPoint = other_455;
    let _e63: Line = self_537;
    let _e67: RadialPoint = other_455;
    let _e70: RadialPoint = other_455;
    let _e73: RadialPoint = other_455;
    let _e76: RadialPoint = other_455;
    let _e89: Line = self_537;
    let _e92: Line = self_537;
    let _e95: Line = self_537;
    let _e98: Line = self_537;
    let _e102: RadialPoint = other_455;
    let _e105: RadialPoint = other_455;
    let _e108: RadialPoint = other_455;
    let _e111: RadialPoint = other_455;
    let _e125: Line = self_537;
    let _e129: RadialPoint = other_455;
    let _e132: RadialPoint = other_455;
    let _e135: RadialPoint = other_455;
    let _e138: RadialPoint = other_455;
    let _e150: Line = self_537;
    let _e154: RadialPoint = other_455;
    let _e157: RadialPoint = other_455;
    let _e160: RadialPoint = other_455;
    let _e163: RadialPoint = other_455;
    let _e176: Line = self_537;
    let _e180: RadialPoint = other_455;
    let _e183: RadialPoint = other_455;
    let _e186: RadialPoint = other_455;
    let _e189: RadialPoint = other_455;
    let _e202: Line = self_537;
    let _e206: RadialPoint = other_455;
    let _e209: RadialPoint = other_455;
    let _e212: RadialPoint = other_455;
    let _e215: RadialPoint = other_455;
    let _e228: Line = self_537;
    let _e232: RadialPoint = other_455;
    let _e235: RadialPoint = other_455;
    let _e238: RadialPoint = other_455;
    let _e241: RadialPoint = other_455;
    let _e254: Line = self_537;
    let _e258: RadialPoint = other_455;
    let _e261: RadialPoint = other_455;
    let _e264: RadialPoint = other_455;
    let _e267: RadialPoint = other_455;
    return Flector(((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0))) + ((vec4<f32>(_e20.g0_.z) * vec4<f32>(_e24.g0_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e37.g1_.y) * vec4<f32>(_e41.g0_.z, _e44.g0_.z, _e47.g0_.x, _e50.g0_.z)) * vec4<f32>(1.0, 0.0, -(1.0), 0.0))) + ((vec4<f32>(_e63.g1_.z) * vec4<f32>(_e67.g0_.y, _e70.g0_.x, _e73.g0_.y, _e76.g0_.y)) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))) + ((vec4<f32>(_e89.g0_.x, _e92.g1_.x, _e95.g1_.x, _e98.g0_.x) * vec4<f32>(_e102.g0_.x, _e105.g0_.z, _e108.g0_.y, _e111.g0_.x)) * vec4<f32>(0.0, -(1.0), 1.0, -(1.0)))), (((((((vec4<f32>(_e125.g0_.y) * vec4<f32>(_e129.g0_.z, _e132.g0_.z, _e135.g0_.x, _e138.g0_.z)) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0)) + ((vec4<f32>(_e150.g0_.z) * vec4<f32>(_e154.g0_.y, _e157.g0_.x, _e160.g0_.y, _e163.g0_.y)) * vec4<f32>(1.0, -(1.0), 0.0, 0.0))) + ((vec4<f32>(_e176.g1_.x) * vec4<f32>(_e180.g1_.x, _e183.g1_.x, _e186.g1_.x, _e189.g0_.x)) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))) + ((vec4<f32>(_e202.g1_.y) * vec4<f32>(_e206.g1_.x, _e209.g1_.x, _e212.g1_.x, _e215.g0_.y)) * vec4<f32>(0.0, -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e228.g1_.z) * vec4<f32>(_e232.g1_.x, _e235.g1_.x, _e238.g1_.x, _e241.g0_.z)) * vec4<f32>(0.0, 0.0, -(1.0), 1.0))) + ((vec4<f32>(_e254.g0_.x) * vec4<f32>(_e258.g0_.x, _e261.g0_.z, _e264.g0_.y, _e267.g0_.x)) * vec4<f32>(0.0, 1.0, -(1.0), 0.0))));
}

fn line_radial_point_outer_product(self_538: Line, other_456: RadialPoint) -> Plane {
    var self_539: Line;
    var other_457: RadialPoint;

    self_539 = self_538;
    other_457 = other_456;
    let _e4: Line = self_539;
    let _e8: RadialPoint = other_457;
    let _e11: RadialPoint = other_457;
    let _e14: RadialPoint = other_457;
    let _e17: RadialPoint = other_457;
    let _e29: Line = self_539;
    let _e33: RadialPoint = other_457;
    let _e36: RadialPoint = other_457;
    let _e39: RadialPoint = other_457;
    let _e42: RadialPoint = other_457;
    let _e55: Line = self_539;
    let _e59: RadialPoint = other_457;
    let _e62: RadialPoint = other_457;
    let _e65: RadialPoint = other_457;
    let _e68: RadialPoint = other_457;
    let _e81: Line = self_539;
    let _e85: RadialPoint = other_457;
    let _e88: RadialPoint = other_457;
    let _e91: RadialPoint = other_457;
    let _e94: RadialPoint = other_457;
    let _e107: Line = self_539;
    let _e111: RadialPoint = other_457;
    let _e114: RadialPoint = other_457;
    let _e117: RadialPoint = other_457;
    let _e120: RadialPoint = other_457;
    let _e133: Line = self_539;
    let _e137: RadialPoint = other_457;
    let _e140: RadialPoint = other_457;
    let _e143: RadialPoint = other_457;
    let _e146: RadialPoint = other_457;
    return Plane((((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.z, _e11.g0_.z, _e14.g0_.x, _e17.g0_.z)) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0)) + ((vec4<f32>(_e29.g0_.z) * vec4<f32>(_e33.g0_.y, _e36.g0_.x, _e39.g0_.y, _e42.g0_.y)) * vec4<f32>(1.0, -(1.0), 0.0, 0.0))) + ((vec4<f32>(_e55.g1_.x) * vec4<f32>(_e59.g1_.x, _e62.g1_.x, _e65.g1_.x, _e68.g0_.x)) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))) + ((vec4<f32>(_e81.g1_.y) * vec4<f32>(_e85.g1_.x, _e88.g1_.x, _e91.g1_.x, _e94.g0_.y)) * vec4<f32>(0.0, -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e107.g1_.z) * vec4<f32>(_e111.g1_.x, _e114.g1_.x, _e117.g1_.x, _e120.g0_.z)) * vec4<f32>(0.0, 0.0, -(1.0), 1.0))) + ((vec4<f32>(_e133.g0_.x) * vec4<f32>(_e137.g0_.x, _e140.g0_.z, _e143.g0_.y, _e146.g0_.x)) * vec4<f32>(0.0, 1.0, -(1.0), 0.0))));
}

fn line_radial_point_inner_product(self_540: Line, other_458: RadialPoint) -> FlatPoint {
    var self_541: Line;
    var other_459: RadialPoint;

    self_541 = self_540;
    other_459 = other_458;
    let _e4: Line = self_541;
    let _e8: RadialPoint = other_459;
    let _e20: Line = self_541;
    let _e24: RadialPoint = other_459;
    let _e37: Line = self_541;
    let _e41: RadialPoint = other_459;
    let _e44: RadialPoint = other_459;
    let _e47: RadialPoint = other_459;
    let _e50: RadialPoint = other_459;
    let _e63: Line = self_541;
    let _e67: RadialPoint = other_459;
    let _e70: RadialPoint = other_459;
    let _e73: RadialPoint = other_459;
    let _e76: RadialPoint = other_459;
    let _e89: Line = self_541;
    let _e92: Line = self_541;
    let _e95: Line = self_541;
    let _e98: Line = self_541;
    let _e102: RadialPoint = other_459;
    let _e105: RadialPoint = other_459;
    let _e108: RadialPoint = other_459;
    let _e111: RadialPoint = other_459;
    return FlatPoint(((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0))) + ((vec4<f32>(_e20.g0_.z) * vec4<f32>(_e24.g0_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e37.g1_.y) * vec4<f32>(_e41.g0_.z, _e44.g0_.z, _e47.g0_.x, _e50.g0_.z)) * vec4<f32>(1.0, 0.0, -(1.0), 0.0))) + ((vec4<f32>(_e63.g1_.z) * vec4<f32>(_e67.g0_.y, _e70.g0_.x, _e73.g0_.y, _e76.g0_.y)) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))) + ((vec4<f32>(_e89.g0_.x, _e92.g1_.x, _e95.g1_.x, _e98.g0_.x) * vec4<f32>(_e102.g0_.x, _e105.g0_.z, _e108.g0_.y, _e111.g0_.x)) * vec4<f32>(0.0, -(1.0), 1.0, -(1.0)))));
}

fn line_radial_point_right_contraction(self_542: Line, other_460: RadialPoint) -> FlatPoint {
    var self_543: Line;
    var other_461: RadialPoint;

    self_543 = self_542;
    other_461 = other_460;
    let _e4: Line = self_543;
    let _e8: RadialPoint = other_461;
    let _e20: Line = self_543;
    let _e24: RadialPoint = other_461;
    let _e37: Line = self_543;
    let _e41: RadialPoint = other_461;
    let _e44: RadialPoint = other_461;
    let _e47: RadialPoint = other_461;
    let _e50: RadialPoint = other_461;
    let _e63: Line = self_543;
    let _e67: RadialPoint = other_461;
    let _e70: RadialPoint = other_461;
    let _e73: RadialPoint = other_461;
    let _e76: RadialPoint = other_461;
    let _e89: Line = self_543;
    let _e92: Line = self_543;
    let _e95: Line = self_543;
    let _e98: Line = self_543;
    let _e102: RadialPoint = other_461;
    let _e105: RadialPoint = other_461;
    let _e108: RadialPoint = other_461;
    let _e111: RadialPoint = other_461;
    return FlatPoint(((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0))) + ((vec4<f32>(_e20.g0_.z) * vec4<f32>(_e24.g0_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e37.g1_.y) * vec4<f32>(_e41.g0_.z, _e44.g0_.z, _e47.g0_.x, _e50.g0_.z)) * vec4<f32>(1.0, 0.0, -(1.0), 0.0))) + ((vec4<f32>(_e63.g1_.z) * vec4<f32>(_e67.g0_.y, _e70.g0_.x, _e73.g0_.y, _e76.g0_.y)) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))) + ((vec4<f32>(_e89.g0_.x, _e92.g1_.x, _e95.g1_.x, _e98.g0_.x) * vec4<f32>(_e102.g0_.x, _e105.g0_.z, _e108.g0_.y, _e111.g0_.x)) * vec4<f32>(0.0, -(1.0), 1.0, -(1.0)))));
}

fn line_dipole_geometric_product(self_544: Line, other_462: Dipole) -> Motor {
    var self_545: Line;
    var other_463: Dipole;

    self_545 = self_544;
    other_463 = other_462;
    let _e4: Line = self_545;
    let _e8: Dipole = other_463;
    let _e11: Dipole = other_463;
    let _e14: Dipole = other_463;
    let _e17: Dipole = other_463;
    let _e30: Line = self_545;
    let _e34: Dipole = other_463;
    let _e37: Dipole = other_463;
    let _e40: Dipole = other_463;
    let _e43: Dipole = other_463;
    let _e57: Line = self_545;
    let _e61: Dipole = other_463;
    let _e64: Dipole = other_463;
    let _e67: Dipole = other_463;
    let _e70: Dipole = other_463;
    let _e84: Line = self_545;
    let _e88: Dipole = other_463;
    let _e91: Dipole = other_463;
    let _e94: Dipole = other_463;
    let _e97: Dipole = other_463;
    let _e111: Line = self_545;
    let _e115: Dipole = other_463;
    let _e118: Dipole = other_463;
    let _e121: Dipole = other_463;
    let _e124: Dipole = other_463;
    let _e138: Line = self_545;
    let _e142: Dipole = other_463;
    let _e145: Dipole = other_463;
    let _e148: Dipole = other_463;
    let _e151: Dipole = other_463;
    let _e165: Line = self_545;
    let _e169: Dipole = other_463;
    let _e172: Dipole = other_463;
    let _e175: Dipole = other_463;
    let _e178: Dipole = other_463;
    let _e191: Line = self_545;
    let _e195: Dipole = other_463;
    let _e198: Dipole = other_463;
    let _e201: Dipole = other_463;
    let _e204: Dipole = other_463;
    let _e218: Line = self_545;
    let _e222: Dipole = other_463;
    let _e225: Dipole = other_463;
    let _e228: Dipole = other_463;
    let _e231: Dipole = other_463;
    return Motor((((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g1_.z, _e11.g1_.z, _e14.g1_.x, _e17.g1_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e30.g0_.z) * vec4<f32>(_e34.g1_.y, _e37.g1_.x, _e40.g1_.y, _e43.g1_.z)) * vec4<f32>(1.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e57.g1_.x) * vec4<f32>(_e61.g0_.z, _e64.g0_.z, _e67.g0_.y, _e70.g0_.x)) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e84.g1_.y) * vec4<f32>(_e88.g0_.z, _e91.g0_.z, _e94.g0_.x, _e97.g0_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e111.g1_.z) * vec4<f32>(_e115.g0_.y, _e118.g0_.x, _e121.g0_.y, _e124.g0_.z)) * vec4<f32>(1.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e138.g0_.x) * vec4<f32>(_e142.g1_.x, _e145.g1_.z, _e148.g1_.y, _e151.g1_.x)) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))), ((((vec4<f32>(_e165.g1_.y) * vec4<f32>(_e169.g1_.z, _e172.g1_.z, _e175.g1_.x, _e178.g1_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e191.g1_.z) * vec4<f32>(_e195.g1_.y, _e198.g1_.x, _e201.g1_.y, _e204.g1_.z)) * vec4<f32>(1.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e218.g1_.x) * vec4<f32>(_e222.g1_.x, _e225.g1_.z, _e228.g1_.y, _e231.g1_.x)) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))));
}

fn line_dipole_regressive_product(self_546: Line, other_464: Dipole) -> Scalar {
    var self_547: Line;
    var other_465: Dipole;

    self_547 = self_546;
    other_465 = other_464;
    let _e5: Line = self_547;
    let _e8: Dipole = other_465;
    let _e13: Line = self_547;
    let _e16: Dipole = other_465;
    let _e21: Line = self_547;
    let _e24: Dipole = other_465;
    let _e29: Line = self_547;
    let _e32: Dipole = other_465;
    let _e37: Line = self_547;
    let _e40: Dipole = other_465;
    let _e45: Line = self_547;
    let _e48: Dipole = other_465;
    return Scalar(((((((0.0 - (_e5.g0_.x * _e8.g1_.x)) - (_e13.g0_.y * _e16.g1_.y)) - (_e21.g0_.z * _e24.g1_.z)) - (_e29.g1_.x * _e32.g0_.x)) - (_e37.g1_.y * _e40.g0_.y)) - (_e45.g1_.z * _e48.g0_.z)));
}

fn line_dipole_outer_product(self_548: Line, other_466: Dipole) -> AntiScalar {
    var self_549: Line;
    var other_467: Dipole;

    self_549 = self_548;
    other_467 = other_466;
    let _e5: Line = self_549;
    let _e8: Dipole = other_467;
    let _e13: Line = self_549;
    let _e16: Dipole = other_467;
    let _e21: Line = self_549;
    let _e24: Dipole = other_467;
    let _e29: Line = self_549;
    let _e32: Dipole = other_467;
    let _e37: Line = self_549;
    let _e40: Dipole = other_467;
    let _e45: Line = self_549;
    let _e48: Dipole = other_467;
    return AntiScalar(((((((0.0 - (_e5.g0_.x * _e8.g1_.x)) - (_e13.g0_.y * _e16.g1_.y)) - (_e21.g0_.z * _e24.g1_.z)) - (_e29.g1_.x * _e32.g0_.x)) - (_e37.g1_.y * _e40.g0_.y)) - (_e45.g1_.z * _e48.g0_.z)));
}

fn line_line_add(self_550: Line, other_468: Line) -> Line {
    var self_551: Line;
    var other_469: Line;

    self_551 = self_550;
    other_469 = other_468;
    let _e4: Line = self_551;
    let _e6: Line = other_469;
    let _e9: Line = self_551;
    let _e11: Line = other_469;
    return Line((_e4.g0_ + _e6.g0_), (_e9.g1_ + _e11.g1_));
}

fn line_line_sub(self_552: Line, other_470: Line) -> Line {
    var self_553: Line;
    var other_471: Line;

    self_553 = self_552;
    other_471 = other_470;
    let _e4: Line = self_553;
    let _e6: Line = other_471;
    let _e9: Line = self_553;
    let _e11: Line = other_471;
    return Line((_e4.g0_ - _e6.g0_), (_e9.g1_ - _e11.g1_));
}

fn line_line_mul(self_554: Line, other_472: Line) -> Line {
    var self_555: Line;
    var other_473: Line;

    self_555 = self_554;
    other_473 = other_472;
    let _e4: Line = self_555;
    let _e6: Line = other_473;
    let _e9: Line = self_555;
    let _e11: Line = other_473;
    return Line((_e4.g0_ * _e6.g0_), (_e9.g1_ * _e11.g1_));
}

fn line_line_div(self_556: Line, other_474: Line) -> Line {
    var self_557: Line;
    var other_475: Line;

    self_557 = self_556;
    other_475 = other_474;
    let _e4: Line = self_557;
    let _e7: Line = self_557;
    let _e10: Line = self_557;
    let _e19: Line = other_475;
    let _e22: Line = other_475;
    let _e25: Line = other_475;
    let _e35: Line = self_557;
    let _e38: Line = self_557;
    let _e41: Line = self_557;
    let _e50: Line = other_475;
    let _e53: Line = other_475;
    let _e56: Line = other_475;
    return Line((((vec3<f32>(_e4.g0_.x, _e7.g0_.y, _e10.g0_.z) * vec3<f32>(1.0, 1.0, 1.0)) / vec3<f32>(_e19.g0_.x, _e22.g0_.y, _e25.g0_.z)) * vec3<f32>(1.0, 1.0, 1.0)), (((vec3<f32>(_e35.g1_.x, _e38.g1_.y, _e41.g1_.z) * vec3<f32>(1.0, 1.0, 1.0)) / vec3<f32>(_e50.g1_.x, _e53.g1_.y, _e56.g1_.z)) * vec3<f32>(1.0, 1.0, 1.0)));
}

fn line_circle_add(self_558: Line, other_476: Circle) -> Circle {
    var self_559: Line;
    var other_477: Circle;

    self_559 = self_558;
    other_477 = other_476;
    let _e4: Circle = other_477;
    let _e6: Line = self_559;
    let _e8: Circle = other_477;
    let _e11: Line = self_559;
    let _e13: Circle = other_477;
    return Circle(_e4.g0_, (_e6.g0_ + _e8.g1_), (_e11.g1_ + _e13.g2_));
}

fn line_circle_sub(self_560: Line, other_478: Circle) -> Circle {
    var self_561: Line;
    var other_479: Circle;

    self_561 = self_560;
    other_479 = other_478;
    let _e6: Circle = other_479;
    let _e9: Line = self_561;
    let _e11: Circle = other_479;
    let _e14: Line = self_561;
    let _e16: Circle = other_479;
    return Circle((vec4<f32>(0.0) - _e6.g0_), (_e9.g0_ - _e11.g1_), (_e14.g1_ - _e16.g2_));
}

fn line_circle_regressive_product(self_562: Line, other_480: Circle) -> RadialPoint {
    var self_563: Line;
    var other_481: Circle;

    self_563 = self_562;
    other_481 = other_480;
    let _e4: Line = self_563;
    let _e8: Circle = other_481;
    let _e11: Circle = other_481;
    let _e14: Circle = other_481;
    let _e25: Line = self_563;
    let _e29: Circle = other_481;
    let _e32: Circle = other_481;
    let _e35: Circle = other_481;
    let _e47: Line = self_563;
    let _e51: Circle = other_481;
    let _e54: Circle = other_481;
    let _e57: Circle = other_481;
    let _e69: Line = self_563;
    let _e71: Circle = other_481;
    let _e79: Line = self_563;
    let _e83: Circle = other_481;
    let _e86: Circle = other_481;
    let _e92: Line = self_563;
    let _e96: Circle = other_481;
    let _e99: Circle = other_481;
    let _e105: Line = self_563;
    let _e109: Circle = other_481;
    let _e112: Circle = other_481;
    let _e118: Line = self_563;
    let _e122: Circle = other_481;
    let _e133: Line = self_563;
    let _e137: Circle = other_481;
    let _e148: Line = self_563;
    let _e152: Circle = other_481;
    return RadialPoint((((((vec3<f32>(_e4.g1_.x) * vec3<f32>(_e8.g0_.z, _e11.g0_.z, _e14.g0_.y)) * vec3<f32>(0.0, -(1.0), 1.0)) + ((vec3<f32>(_e25.g1_.y) * vec3<f32>(_e29.g0_.z, _e32.g0_.z, _e35.g0_.x)) * vec3<f32>(1.0, 0.0, -(1.0)))) + ((vec3<f32>(_e47.g1_.z) * vec3<f32>(_e51.g0_.y, _e54.g0_.x, _e57.g0_.y)) * vec3<f32>(-(1.0), 1.0, 0.0))) + (_e69.g0_ * vec3<f32>(_e71.g0_.w))), ((((((vec2<f32>(0.0) - (vec2<f32>(_e79.g0_.x) * vec2<f32>(_e83.g0_.x, _e86.g2_.x))) - (vec2<f32>(_e92.g0_.y) * vec2<f32>(_e96.g0_.y, _e99.g2_.y))) - (vec2<f32>(_e105.g0_.z) * vec2<f32>(_e109.g0_.z, _e112.g2_.z))) + ((vec2<f32>(_e118.g1_.y) * vec2<f32>(_e122.g1_.y)) * vec2<f32>(0.0, -(1.0)))) + ((vec2<f32>(_e133.g1_.z) * vec2<f32>(_e137.g1_.z)) * vec2<f32>(0.0, -(1.0)))) + ((vec2<f32>(_e148.g1_.x) * vec2<f32>(_e152.g1_.x)) * vec2<f32>(0.0, -(1.0)))));
}

fn line_plane_regressive_product(self_564: Line, other_482: Plane) -> FlatPoint {
    var self_565: Line;
    var other_483: Plane;

    self_565 = self_564;
    other_483 = other_482;
    let _e4: Line = self_565;
    let _e8: Plane = other_483;
    let _e19: Line = self_565;
    let _e23: Plane = other_483;
    let _e35: Line = self_565;
    let _e39: Plane = other_483;
    let _e51: Line = self_565;
    let _e55: Plane = other_483;
    let _e67: Line = self_565;
    let _e70: Line = self_565;
    let _e73: Line = self_565;
    let _e76: Line = self_565;
    let _e80: Plane = other_483;
    return FlatPoint(((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e35.g1_.y) * _e39.g0_.zzxz) * vec4<f32>(1.0, 0.0, -(1.0), 0.0))) + ((vec4<f32>(_e51.g1_.z) * _e55.g0_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))) + ((vec4<f32>(_e67.g0_.x, _e70.g1_.x, _e73.g1_.x, _e76.g0_.x) * _e80.g0_.wzyx) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))));
}

fn line_sphere_regressive_product(self_566: Line, other_484: Sphere) -> Dipole {
    var self_567: Line;
    var other_485: Sphere;

    self_567 = self_566;
    other_485 = other_484;
    let _e4: Line = self_567;
    let _e6: Sphere = other_485;
    let _e11: Line = self_567;
    let _e13: Sphere = other_485;
    let _e18: Line = self_567;
    let _e22: Sphere = other_485;
    let _e25: Sphere = other_485;
    let _e28: Sphere = other_485;
    let _e31: Sphere = other_485;
    let _e43: Line = self_567;
    let _e47: Sphere = other_485;
    let _e50: Sphere = other_485;
    let _e53: Sphere = other_485;
    let _e56: Sphere = other_485;
    let _e69: Line = self_567;
    let _e73: Sphere = other_485;
    let _e76: Sphere = other_485;
    let _e79: Sphere = other_485;
    let _e82: Sphere = other_485;
    let _e95: Line = self_567;
    let _e99: Sphere = other_485;
    let _e102: Sphere = other_485;
    let _e105: Sphere = other_485;
    let _e108: Sphere = other_485;
    let _e121: Line = self_567;
    let _e125: Sphere = other_485;
    let _e128: Sphere = other_485;
    let _e131: Sphere = other_485;
    let _e134: Sphere = other_485;
    let _e147: Line = self_567;
    let _e151: Sphere = other_485;
    let _e154: Sphere = other_485;
    let _e157: Sphere = other_485;
    let _e160: Sphere = other_485;
    return Dipole((_e4.g0_ * vec3<f32>(_e6.g0_.x)), (_e11.g1_ * vec3<f32>(_e13.g0_.x)), (((((((vec4<f32>(_e18.g0_.y) * vec4<f32>(_e22.g0_.y, _e25.g0_.y, _e28.g0_.y, _e31.g1_.y)) * vec4<f32>(0.0, 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e43.g0_.z) * vec4<f32>(_e47.g0_.y, _e50.g0_.y, _e53.g0_.y, _e56.g1_.z)) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e69.g1_.x) * vec4<f32>(_e73.g1_.z, _e76.g1_.z, _e79.g1_.y, _e82.g1_.z)) * vec4<f32>(0.0, -(1.0), 1.0, 0.0))) + ((vec4<f32>(_e95.g1_.y) * vec4<f32>(_e99.g1_.z, _e102.g1_.z, _e105.g1_.x, _e108.g1_.z)) * vec4<f32>(1.0, 0.0, -(1.0), 0.0))) + ((vec4<f32>(_e121.g1_.z) * vec4<f32>(_e125.g1_.y, _e128.g1_.x, _e131.g1_.y, _e134.g1_.y)) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))) + ((vec4<f32>(_e147.g0_.x) * vec4<f32>(_e151.g0_.y, _e154.g0_.x, _e157.g0_.x, _e160.g1_.x)) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))));
}

fn line_motor_add(self_568: Line, other_486: Motor) -> Motor {
    var self_569: Line;
    var other_487: Motor;

    self_569 = self_568;
    other_487 = other_486;
    let _e4: Line = self_569;
    let _e7: Line = self_569;
    let _e10: Line = self_569;
    let _e13: Line = self_569;
    let _e23: Motor = other_487;
    let _e26: Line = self_569;
    let _e29: Line = self_569;
    let _e32: Line = self_569;
    let _e35: Line = self_569;
    let _e45: Motor = other_487;
    return Motor(((vec4<f32>(_e4.g0_.x, _e7.g0_.y, _e10.g0_.z, _e13.g0_.x) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) + _e23.g0_), ((vec4<f32>(_e26.g1_.x, _e29.g1_.y, _e32.g1_.z, _e35.g0_.x) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) + _e45.g1_));
}

fn line_motor_sub(self_570: Line, other_488: Motor) -> Motor {
    var self_571: Line;
    var other_489: Motor;

    self_571 = self_570;
    other_489 = other_488;
    let _e4: Line = self_571;
    let _e7: Line = self_571;
    let _e10: Line = self_571;
    let _e13: Line = self_571;
    let _e23: Motor = other_489;
    let _e26: Line = self_571;
    let _e29: Line = self_571;
    let _e32: Line = self_571;
    let _e35: Line = self_571;
    let _e45: Motor = other_489;
    return Motor(((vec4<f32>(_e4.g0_.x, _e7.g0_.y, _e10.g0_.z, _e13.g0_.x) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) - _e23.g0_), ((vec4<f32>(_e26.g1_.x, _e29.g1_.y, _e32.g1_.z, _e35.g0_.x) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) - _e45.g1_));
}

fn line_flector_regressive_product(self_572: Line, other_490: Flector) -> FlatPoint {
    var self_573: Line;
    var other_491: Flector;

    self_573 = self_572;
    other_491 = other_490;
    let _e4: Line = self_573;
    let _e8: Flector = other_491;
    let _e19: Line = self_573;
    let _e23: Flector = other_491;
    let _e35: Line = self_573;
    let _e39: Flector = other_491;
    let _e51: Line = self_573;
    let _e55: Flector = other_491;
    let _e67: Line = self_573;
    let _e70: Line = self_573;
    let _e73: Line = self_573;
    let _e76: Line = self_573;
    let _e80: Flector = other_491;
    return FlatPoint(((((((vec4<f32>(_e4.g0_.y) * _e8.g1_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e19.g0_.z) * _e23.g1_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e35.g1_.y) * _e39.g1_.zzxz) * vec4<f32>(1.0, 0.0, -(1.0), 0.0))) + ((vec4<f32>(_e51.g1_.z) * _e55.g1_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))) + ((vec4<f32>(_e67.g0_.x, _e70.g1_.x, _e73.g1_.x, _e76.g0_.x) * _e80.g1_.wzyx) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))));
}

fn line_multi_vector_add(self_574: Line, other_492: MultiVector) -> MultiVector {
    var self_575: Line;
    var other_493: MultiVector;

    self_575 = self_574;
    other_493 = other_492;
    let _e4: MultiVector = other_493;
    let _e6: MultiVector = other_493;
    let _e8: MultiVector = other_493;
    let _e10: MultiVector = other_493;
    let _e12: MultiVector = other_493;
    let _e14: MultiVector = other_493;
    let _e16: Line = self_575;
    let _e18: MultiVector = other_493;
    let _e21: Line = self_575;
    let _e23: MultiVector = other_493;
    let _e26: MultiVector = other_493;
    let _e28: MultiVector = other_493;
    return MultiVector(_e4.g0_, _e6.g1_, _e8.g2_, _e10.g3_, _e12.g4_, _e14.g5_, (_e16.g0_ + _e18.g6_), (_e21.g1_ + _e23.g7_), _e26.g8_, _e28.g9_);
}

fn line_multi_vector_sub(self_576: Line, other_494: MultiVector) -> MultiVector {
    var self_577: Line;
    var other_495: MultiVector;

    self_577 = self_576;
    other_495 = other_494;
    let _e6: MultiVector = other_495;
    let _e11: MultiVector = other_495;
    let _e16: MultiVector = other_495;
    let _e21: MultiVector = other_495;
    let _e26: MultiVector = other_495;
    let _e31: MultiVector = other_495;
    let _e34: Line = self_577;
    let _e36: MultiVector = other_495;
    let _e39: Line = self_577;
    let _e41: MultiVector = other_495;
    let _e46: MultiVector = other_495;
    let _e51: MultiVector = other_495;
    return MultiVector((vec3<f32>(0.0) - _e6.g0_), (vec3<f32>(0.0) - _e11.g1_), (vec2<f32>(0.0) - _e16.g2_), (vec4<f32>(0.0) - _e21.g3_), (vec3<f32>(0.0) - _e26.g4_), (vec3<f32>(0.0) - _e31.g5_), (_e34.g0_ - _e36.g6_), (_e39.g1_ - _e41.g7_), (vec4<f32>(0.0) - _e46.g8_), (vec4<f32>(0.0) - _e51.g9_));
}

fn line_scale(self_578: Line, other_496: f32) -> Line {
    var self_579: Line;
    var other_497: f32;

    self_579 = self_578;
    other_497 = other_496;
    let _e4: Line = self_579;
    let _e5: f32 = other_497;
    let _e7: Line = line_scalar_geometric_product(_e4, Scalar(_e5));
    return _e7;
}

fn circle_zero() -> Circle {
    return Circle(vec4<f32>(0.0), vec3<f32>(0.0), vec3<f32>(0.0));
}

fn circle_one() -> Circle {
    return Circle(vec4<f32>(0.0), vec3<f32>(0.0), vec3<f32>(0.0));
}

fn circle_neg(self_580: Circle) -> Circle {
    var self_581: Circle;

    self_581 = self_580;
    let _e2: Circle = self_581;
    let _e8: Circle = self_581;
    let _e14: Circle = self_581;
    return Circle((_e2.g0_ * vec4<f32>(-(1.0))), (_e8.g1_ * vec3<f32>(-(1.0))), (_e14.g2_ * vec3<f32>(-(1.0))));
}

fn circle_automorphism(self_582: Circle) -> Circle {
    var self_583: Circle;

    self_583 = self_582;
    let _e2: Circle = self_583;
    let _e8: Circle = self_583;
    let _e14: Circle = self_583;
    return Circle((_e2.g0_ * vec4<f32>(-(1.0))), (_e8.g1_ * vec3<f32>(-(1.0))), (_e14.g2_ * vec3<f32>(-(1.0))));
}

fn circle_reversal(self_584: Circle) -> Circle {
    var self_585: Circle;

    self_585 = self_584;
    let _e2: Circle = self_585;
    let _e8: Circle = self_585;
    let _e14: Circle = self_585;
    return Circle((_e2.g0_ * vec4<f32>(-(1.0))), (_e8.g1_ * vec3<f32>(-(1.0))), (_e14.g2_ * vec3<f32>(-(1.0))));
}

fn circle_conjugation(self_586: Circle) -> Circle {
    var self_587: Circle;

    self_587 = self_586;
    let _e2: Circle = self_587;
    let _e4: Circle = self_587;
    let _e6: Circle = self_587;
    return Circle(_e2.g0_, _e4.g1_, _e6.g2_);
}

fn circle_dual(self_588: Circle) -> Dipole {
    var self_589: Circle;

    self_589 = self_588;
    let _e2: Circle = self_589;
    let _e8: Circle = self_589;
    let _e14: Circle = self_589;
    return Dipole((_e2.g2_ * vec3<f32>(-(1.0))), (_e8.g1_ * vec3<f32>(-(1.0))), (_e14.g0_ * vec4<f32>(-(1.0))));
}

fn circle_scalar_geometric_product(self_590: Circle, other_498: Scalar) -> Circle {
    var self_591: Circle;
    var other_499: Scalar;

    self_591 = self_590;
    other_499 = other_498;
    let _e4: Circle = self_591;
    let _e6: Scalar = other_499;
    let _e10: Circle = self_591;
    let _e12: Scalar = other_499;
    let _e16: Circle = self_591;
    let _e18: Scalar = other_499;
    return Circle((_e4.g0_ * vec4<f32>(_e6.g0_)), (_e10.g1_ * vec3<f32>(_e12.g0_)), (_e16.g2_ * vec3<f32>(_e18.g0_)));
}

fn circle_scalar_outer_product(self_592: Circle, other_500: Scalar) -> Circle {
    var self_593: Circle;
    var other_501: Scalar;

    self_593 = self_592;
    other_501 = other_500;
    let _e4: Circle = self_593;
    let _e6: Scalar = other_501;
    let _e10: Circle = self_593;
    let _e12: Scalar = other_501;
    let _e16: Circle = self_593;
    let _e18: Scalar = other_501;
    return Circle((_e4.g0_ * vec4<f32>(_e6.g0_)), (_e10.g1_ * vec3<f32>(_e12.g0_)), (_e16.g2_ * vec3<f32>(_e18.g0_)));
}

fn circle_scalar_inner_product(self_594: Circle, other_502: Scalar) -> Circle {
    var self_595: Circle;
    var other_503: Scalar;

    self_595 = self_594;
    other_503 = other_502;
    let _e4: Circle = self_595;
    let _e6: Scalar = other_503;
    let _e10: Circle = self_595;
    let _e12: Scalar = other_503;
    let _e16: Circle = self_595;
    let _e18: Scalar = other_503;
    return Circle((_e4.g0_ * vec4<f32>(_e6.g0_)), (_e10.g1_ * vec3<f32>(_e12.g0_)), (_e16.g2_ * vec3<f32>(_e18.g0_)));
}

fn circle_scalar_right_contraction(self_596: Circle, other_504: Scalar) -> Circle {
    var self_597: Circle;
    var other_505: Scalar;

    self_597 = self_596;
    other_505 = other_504;
    let _e4: Circle = self_597;
    let _e6: Scalar = other_505;
    let _e10: Circle = self_597;
    let _e12: Scalar = other_505;
    let _e16: Circle = self_597;
    let _e18: Scalar = other_505;
    return Circle((_e4.g0_ * vec4<f32>(_e6.g0_)), (_e10.g1_ * vec3<f32>(_e12.g0_)), (_e16.g2_ * vec3<f32>(_e18.g0_)));
}

fn circle_anti_scalar_regressive_product(self_598: Circle, other_506: AntiScalar) -> Circle {
    var self_599: Circle;
    var other_507: AntiScalar;

    self_599 = self_598;
    other_507 = other_506;
    let _e4: Circle = self_599;
    let _e6: AntiScalar = other_507;
    let _e10: Circle = self_599;
    let _e12: AntiScalar = other_507;
    let _e16: Circle = self_599;
    let _e18: AntiScalar = other_507;
    return Circle((_e4.g0_ * vec4<f32>(_e6.g0_)), (_e10.g1_ * vec3<f32>(_e12.g0_)), (_e16.g2_ * vec3<f32>(_e18.g0_)));
}

fn circle_radial_point_outer_product(self_600: Circle, other_508: RadialPoint) -> Sphere {
    var self_601: Circle;
    var other_509: RadialPoint;

    self_601 = self_600;
    other_509 = other_508;
    let _e4: Circle = self_601;
    let _e8: RadialPoint = other_509;
    let _e18: Circle = self_601;
    let _e22: RadialPoint = other_509;
    let _e33: Circle = self_601;
    let _e37: RadialPoint = other_509;
    let _e46: Circle = self_601;
    let _e50: RadialPoint = other_509;
    let _e60: Circle = self_601;
    let _e64: RadialPoint = other_509;
    let _e74: Circle = self_601;
    let _e78: RadialPoint = other_509;
    let _e88: Circle = self_601;
    let _e92: RadialPoint = other_509;
    let _e103: Circle = self_601;
    let _e107: RadialPoint = other_509;
    let _e117: Circle = self_601;
    let _e121: RadialPoint = other_509;
    let _e132: Circle = self_601;
    let _e136: RadialPoint = other_509;
    let _e147: Circle = self_601;
    let _e151: RadialPoint = other_509;
    let _e163: Circle = self_601;
    let _e167: RadialPoint = other_509;
    let _e179: Circle = self_601;
    let _e183: RadialPoint = other_509;
    let _e195: Circle = self_601;
    let _e198: Circle = self_601;
    let _e201: Circle = self_601;
    let _e205: RadialPoint = other_509;
    return Sphere(((((((((vec2<f32>(_e4.g0_.y) * vec2<f32>(_e8.g0_.y)) * vec2<f32>(-(1.0), 0.0)) + ((vec2<f32>(_e18.g0_.z) * vec2<f32>(_e22.g0_.z)) * vec2<f32>(-(1.0), 0.0))) + ((vec2<f32>(_e33.g0_.w) * _e37.g1_) * vec2<f32>(-(1.0), 1.0))) + ((vec2<f32>(_e46.g2_.x) * vec2<f32>(_e50.g0_.x)) * vec2<f32>(0.0, 1.0))) + ((vec2<f32>(_e60.g2_.y) * vec2<f32>(_e64.g0_.y)) * vec2<f32>(0.0, 1.0))) + ((vec2<f32>(_e74.g2_.z) * vec2<f32>(_e78.g0_.z)) * vec2<f32>(0.0, 1.0))) + ((vec2<f32>(_e88.g0_.x) * vec2<f32>(_e92.g0_.x)) * vec2<f32>(-(1.0), 0.0))), ((((((((vec3<f32>(_e103.g1_.x) * _e107.g0_.zzy) * vec3<f32>(0.0, 1.0, -(1.0))) + ((vec3<f32>(_e117.g1_.y) * _e121.g0_.zzx) * vec3<f32>(-(1.0), 0.0, 1.0))) + ((vec3<f32>(_e132.g1_.z) * _e136.g0_.yxy) * vec3<f32>(1.0, -(1.0), 0.0))) + ((vec3<f32>(_e147.g2_.x) * vec3<f32>(_e151.g1_.x)) * vec3<f32>(-(1.0), 0.0, 0.0))) + ((vec3<f32>(_e163.g2_.y) * vec3<f32>(_e167.g1_.x)) * vec3<f32>(0.0, -(1.0), 0.0))) + ((vec3<f32>(_e179.g2_.z) * vec3<f32>(_e183.g1_.x)) * vec3<f32>(0.0, 0.0, -(1.0)))) + (vec3<f32>(_e195.g0_.x, _e198.g0_.y, _e201.g0_.z) * vec3<f32>(_e205.g1_.y))));
}

fn circle_radial_point_inner_product(self_602: Circle, other_510: RadialPoint) -> Dipole {
    var self_603: Circle;
    var other_511: RadialPoint;

    self_603 = self_602;
    other_511 = other_510;
    let _e4: Circle = self_603;
    let _e8: RadialPoint = other_511;
    let _e18: Circle = self_603;
    let _e22: RadialPoint = other_511;
    let _e33: Circle = self_603;
    let _e37: RadialPoint = other_511;
    let _e50: Circle = self_603;
    let _e54: RadialPoint = other_511;
    let _e58: Circle = self_603;
    let _e62: RadialPoint = other_511;
    let _e74: Circle = self_603;
    let _e78: RadialPoint = other_511;
    let _e91: Circle = self_603;
    let _e95: RadialPoint = other_511;
    let _e98: RadialPoint = other_511;
    let _e101: RadialPoint = other_511;
    let _e104: RadialPoint = other_511;
    let _e117: Circle = self_603;
    let _e121: RadialPoint = other_511;
    let _e124: RadialPoint = other_511;
    let _e127: RadialPoint = other_511;
    let _e130: RadialPoint = other_511;
    let _e143: Circle = self_603;
    let _e146: Circle = self_603;
    let _e149: Circle = self_603;
    let _e152: Circle = self_603;
    let _e156: RadialPoint = other_511;
    let _e159: RadialPoint = other_511;
    let _e162: RadialPoint = other_511;
    let _e165: RadialPoint = other_511;
    return Dipole(((((vec3<f32>(_e4.g0_.y) * _e8.g0_.zzx) * vec3<f32>(-(1.0), 0.0, 1.0)) + ((vec3<f32>(_e18.g0_.z) * _e22.g0_.yxy) * vec3<f32>(1.0, -(1.0), 0.0))) + ((vec3<f32>(_e33.g0_.x) * _e37.g0_.xzy) * vec3<f32>(0.0, 1.0, -(1.0)))), (vec3<f32>(0.0) - (vec3<f32>(_e50.g0_.w) * _e54.g0_)), ((((((vec4<f32>(_e58.g1_.y) * vec4<f32>(_e62.g0_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0))) + ((vec4<f32>(_e74.g1_.z) * vec4<f32>(_e78.g0_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e91.g2_.y) * vec4<f32>(_e95.g0_.z, _e98.g0_.z, _e101.g0_.x, _e104.g0_.z)) * vec4<f32>(1.0, 0.0, -(1.0), 0.0))) + ((vec4<f32>(_e117.g2_.z) * vec4<f32>(_e121.g0_.y, _e124.g0_.x, _e127.g0_.y, _e130.g0_.y)) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))) + ((vec4<f32>(_e143.g1_.x, _e146.g2_.x, _e149.g2_.x, _e152.g1_.x) * vec4<f32>(_e156.g0_.x, _e159.g0_.z, _e162.g0_.y, _e165.g0_.x)) * vec4<f32>(0.0, -(1.0), 1.0, -(1.0)))));
}

fn circle_radial_point_right_contraction(self_604: Circle, other_512: RadialPoint) -> Dipole {
    var self_605: Circle;
    var other_513: RadialPoint;

    self_605 = self_604;
    other_513 = other_512;
    let _e4: Circle = self_605;
    let _e8: RadialPoint = other_513;
    let _e18: Circle = self_605;
    let _e22: RadialPoint = other_513;
    let _e33: Circle = self_605;
    let _e37: RadialPoint = other_513;
    let _e50: Circle = self_605;
    let _e54: RadialPoint = other_513;
    let _e58: Circle = self_605;
    let _e62: RadialPoint = other_513;
    let _e74: Circle = self_605;
    let _e78: RadialPoint = other_513;
    let _e91: Circle = self_605;
    let _e95: RadialPoint = other_513;
    let _e98: RadialPoint = other_513;
    let _e101: RadialPoint = other_513;
    let _e104: RadialPoint = other_513;
    let _e117: Circle = self_605;
    let _e121: RadialPoint = other_513;
    let _e124: RadialPoint = other_513;
    let _e127: RadialPoint = other_513;
    let _e130: RadialPoint = other_513;
    let _e143: Circle = self_605;
    let _e146: Circle = self_605;
    let _e149: Circle = self_605;
    let _e152: Circle = self_605;
    let _e156: RadialPoint = other_513;
    let _e159: RadialPoint = other_513;
    let _e162: RadialPoint = other_513;
    let _e165: RadialPoint = other_513;
    return Dipole(((((vec3<f32>(_e4.g0_.y) * _e8.g0_.zzx) * vec3<f32>(-(1.0), 0.0, 1.0)) + ((vec3<f32>(_e18.g0_.z) * _e22.g0_.yxy) * vec3<f32>(1.0, -(1.0), 0.0))) + ((vec3<f32>(_e33.g0_.x) * _e37.g0_.xzy) * vec3<f32>(0.0, 1.0, -(1.0)))), (vec3<f32>(0.0) - (vec3<f32>(_e50.g0_.w) * _e54.g0_)), ((((((vec4<f32>(_e58.g1_.y) * vec4<f32>(_e62.g0_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0))) + ((vec4<f32>(_e74.g1_.z) * vec4<f32>(_e78.g0_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e91.g2_.y) * vec4<f32>(_e95.g0_.z, _e98.g0_.z, _e101.g0_.x, _e104.g0_.z)) * vec4<f32>(1.0, 0.0, -(1.0), 0.0))) + ((vec4<f32>(_e117.g2_.z) * vec4<f32>(_e121.g0_.y, _e124.g0_.x, _e127.g0_.y, _e130.g0_.y)) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))) + ((vec4<f32>(_e143.g1_.x, _e146.g2_.x, _e149.g2_.x, _e152.g1_.x) * vec4<f32>(_e156.g0_.x, _e159.g0_.z, _e162.g0_.y, _e165.g0_.x)) * vec4<f32>(0.0, -(1.0), 1.0, -(1.0)))));
}

fn circle_flat_point_regressive_product(self_606: Circle, other_514: FlatPoint) -> Scalar {
    var self_607: Circle;
    var other_515: FlatPoint;

    self_607 = self_606;
    other_515 = other_514;
    let _e5: Circle = self_607;
    let _e8: FlatPoint = other_515;
    let _e13: Circle = self_607;
    let _e16: FlatPoint = other_515;
    let _e21: Circle = self_607;
    let _e24: FlatPoint = other_515;
    let _e29: Circle = self_607;
    let _e32: FlatPoint = other_515;
    return Scalar(((((0.0 - (_e5.g0_.x * _e8.g0_.x)) - (_e13.g0_.y * _e16.g0_.y)) - (_e21.g0_.z * _e24.g0_.z)) - (_e29.g0_.w * _e32.g0_.w)));
}

fn circle_flat_point_outer_product(self_608: Circle, other_516: FlatPoint) -> AntiScalar {
    var self_609: Circle;
    var other_517: FlatPoint;

    self_609 = self_608;
    other_517 = other_516;
    let _e5: Circle = self_609;
    let _e8: FlatPoint = other_517;
    let _e13: Circle = self_609;
    let _e16: FlatPoint = other_517;
    let _e21: Circle = self_609;
    let _e24: FlatPoint = other_517;
    let _e29: Circle = self_609;
    let _e32: FlatPoint = other_517;
    return AntiScalar(((((0.0 - (_e5.g0_.x * _e8.g0_.x)) - (_e13.g0_.y * _e16.g0_.y)) - (_e21.g0_.z * _e24.g0_.z)) - (_e29.g0_.w * _e32.g0_.w)));
}

fn circle_dipole_regressive_product(self_610: Circle, other_518: Dipole) -> Scalar {
    var self_611: Circle;
    var other_519: Dipole;

    self_611 = self_610;
    other_519 = other_518;
    let _e5: Circle = self_611;
    let _e8: Dipole = other_519;
    let _e13: Circle = self_611;
    let _e16: Dipole = other_519;
    let _e21: Circle = self_611;
    let _e24: Dipole = other_519;
    let _e29: Circle = self_611;
    let _e32: Dipole = other_519;
    let _e37: Circle = self_611;
    let _e40: Dipole = other_519;
    let _e45: Circle = self_611;
    let _e48: Dipole = other_519;
    let _e53: Circle = self_611;
    let _e56: Dipole = other_519;
    let _e61: Circle = self_611;
    let _e64: Dipole = other_519;
    let _e69: Circle = self_611;
    let _e72: Dipole = other_519;
    let _e77: Circle = self_611;
    let _e80: Dipole = other_519;
    return Scalar(((((((((((0.0 - (_e5.g0_.x * _e8.g2_.x)) - (_e13.g0_.y * _e16.g2_.y)) - (_e21.g0_.z * _e24.g2_.z)) - (_e29.g0_.w * _e32.g2_.w)) - (_e37.g1_.x * _e40.g1_.x)) - (_e45.g1_.y * _e48.g1_.y)) - (_e53.g1_.z * _e56.g1_.z)) - (_e61.g2_.x * _e64.g0_.x)) - (_e69.g2_.y * _e72.g0_.y)) - (_e77.g2_.z * _e80.g0_.z)));
}

fn circle_dipole_outer_product(self_612: Circle, other_520: Dipole) -> AntiScalar {
    var self_613: Circle;
    var other_521: Dipole;

    self_613 = self_612;
    other_521 = other_520;
    let _e5: Circle = self_613;
    let _e8: Dipole = other_521;
    let _e13: Circle = self_613;
    let _e16: Dipole = other_521;
    let _e21: Circle = self_613;
    let _e24: Dipole = other_521;
    let _e29: Circle = self_613;
    let _e32: Dipole = other_521;
    let _e37: Circle = self_613;
    let _e40: Dipole = other_521;
    let _e45: Circle = self_613;
    let _e48: Dipole = other_521;
    let _e53: Circle = self_613;
    let _e56: Dipole = other_521;
    let _e61: Circle = self_613;
    let _e64: Dipole = other_521;
    let _e69: Circle = self_613;
    let _e72: Dipole = other_521;
    let _e77: Circle = self_613;
    let _e80: Dipole = other_521;
    return AntiScalar(((((((((((0.0 - (_e5.g0_.x * _e8.g2_.x)) - (_e13.g0_.y * _e16.g2_.y)) - (_e21.g0_.z * _e24.g2_.z)) - (_e29.g0_.w * _e32.g2_.w)) - (_e37.g1_.x * _e40.g1_.x)) - (_e45.g1_.y * _e48.g1_.y)) - (_e53.g1_.z * _e56.g1_.z)) - (_e61.g2_.x * _e64.g0_.x)) - (_e69.g2_.y * _e72.g0_.y)) - (_e77.g2_.z * _e80.g0_.z)));
}

fn circle_dipole_inner_product(self_614: Circle, other_522: Dipole) -> RadialPoint {
    var self_615: Circle;
    var other_523: Dipole;

    self_615 = self_614;
    other_523 = other_522;
    let _e4: Circle = self_615;
    let _e8: Dipole = other_523;
    let _e11: Circle = self_615;
    let _e15: Dipole = other_523;
    let _e25: Circle = self_615;
    let _e29: Dipole = other_523;
    let _e40: Circle = self_615;
    let _e44: Dipole = other_523;
    let _e55: Circle = self_615;
    let _e59: Dipole = other_523;
    let _e70: Circle = self_615;
    let _e74: Dipole = other_523;
    let _e85: Circle = self_615;
    let _e89: Dipole = other_523;
    return RadialPoint((vec3<f32>(_e4.g0_.w) * _e8.g1_), (((((((vec2<f32>(_e11.g0_.y) * vec2<f32>(_e15.g1_.y)) * vec2<f32>(-(1.0), 0.0)) + ((vec2<f32>(_e25.g0_.z) * vec2<f32>(_e29.g1_.z)) * vec2<f32>(-(1.0), 0.0))) + ((vec2<f32>(_e40.g2_.x) * vec2<f32>(_e44.g1_.x)) * vec2<f32>(0.0, -(1.0)))) + ((vec2<f32>(_e55.g2_.y) * vec2<f32>(_e59.g1_.y)) * vec2<f32>(0.0, -(1.0)))) + ((vec2<f32>(_e70.g2_.z) * vec2<f32>(_e74.g1_.z)) * vec2<f32>(0.0, -(1.0)))) + ((vec2<f32>(_e85.g0_.x) * vec2<f32>(_e89.g1_.x)) * vec2<f32>(-(1.0), 0.0))));
}

fn circle_dipole_right_contraction(self_616: Circle, other_524: Dipole) -> RadialPoint {
    var self_617: Circle;
    var other_525: Dipole;

    self_617 = self_616;
    other_525 = other_524;
    let _e4: Circle = self_617;
    let _e8: Dipole = other_525;
    let _e11: Circle = self_617;
    let _e15: Dipole = other_525;
    let _e25: Circle = self_617;
    let _e29: Dipole = other_525;
    let _e40: Circle = self_617;
    let _e44: Dipole = other_525;
    let _e55: Circle = self_617;
    let _e59: Dipole = other_525;
    let _e70: Circle = self_617;
    let _e74: Dipole = other_525;
    let _e85: Circle = self_617;
    let _e89: Dipole = other_525;
    return RadialPoint((vec3<f32>(_e4.g0_.w) * _e8.g1_), (((((((vec2<f32>(_e11.g0_.y) * vec2<f32>(_e15.g1_.y)) * vec2<f32>(-(1.0), 0.0)) + ((vec2<f32>(_e25.g0_.z) * vec2<f32>(_e29.g1_.z)) * vec2<f32>(-(1.0), 0.0))) + ((vec2<f32>(_e40.g2_.x) * vec2<f32>(_e44.g1_.x)) * vec2<f32>(0.0, -(1.0)))) + ((vec2<f32>(_e55.g2_.y) * vec2<f32>(_e59.g1_.y)) * vec2<f32>(0.0, -(1.0)))) + ((vec2<f32>(_e70.g2_.z) * vec2<f32>(_e74.g1_.z)) * vec2<f32>(0.0, -(1.0)))) + ((vec2<f32>(_e85.g0_.x) * vec2<f32>(_e89.g1_.x)) * vec2<f32>(-(1.0), 0.0))));
}

fn circle_line_into(self_618: Circle) -> Line {
    var self_619: Circle;

    self_619 = self_618;
    let _e2: Circle = self_619;
    let _e4: Circle = self_619;
    return Line(_e2.g1_, _e4.g2_);
}

fn circle_line_add(self_620: Circle, other_526: Line) -> Circle {
    var self_621: Circle;
    var other_527: Line;

    self_621 = self_620;
    other_527 = other_526;
    let _e4: Circle = self_621;
    let _e6: Circle = self_621;
    let _e8: Line = other_527;
    let _e11: Circle = self_621;
    let _e13: Line = other_527;
    return Circle(_e4.g0_, (_e6.g1_ + _e8.g0_), (_e11.g2_ + _e13.g1_));
}

fn circle_line_sub(self_622: Circle, other_528: Line) -> Circle {
    var self_623: Circle;
    var other_529: Line;

    self_623 = self_622;
    other_529 = other_528;
    let _e4: Circle = self_623;
    let _e6: Circle = self_623;
    let _e8: Line = other_529;
    let _e11: Circle = self_623;
    let _e13: Line = other_529;
    return Circle(_e4.g0_, (_e6.g1_ - _e8.g0_), (_e11.g2_ - _e13.g1_));
}

fn circle_line_regressive_product(self_624: Circle, other_530: Line) -> RadialPoint {
    var self_625: Circle;
    var other_531: Line;

    self_625 = self_624;
    other_531 = other_530;
    let _e4: Circle = self_625;
    let _e8: Line = other_531;
    let _e18: Circle = self_625;
    let _e22: Line = other_531;
    let _e33: Circle = self_625;
    let _e37: Line = other_531;
    let _e41: Circle = self_625;
    let _e45: Line = other_531;
    let _e56: Circle = self_625;
    let _e60: Line = other_531;
    let _e70: Circle = self_625;
    let _e74: Line = other_531;
    let _e85: Circle = self_625;
    let _e89: Line = other_531;
    let _e100: Circle = self_625;
    let _e104: Line = other_531;
    let _e115: Circle = self_625;
    let _e119: Line = other_531;
    let _e130: Circle = self_625;
    let _e134: Line = other_531;
    let _e145: Circle = self_625;
    let _e149: Line = other_531;
    let _e160: Circle = self_625;
    let _e164: Line = other_531;
    let _e175: Circle = self_625;
    let _e179: Line = other_531;
    return RadialPoint((((((vec3<f32>(_e4.g0_.y) * _e8.g1_.zzx) * vec3<f32>(-(1.0), 0.0, 1.0)) + ((vec3<f32>(_e18.g0_.z) * _e22.g1_.yxy) * vec3<f32>(1.0, -(1.0), 0.0))) + (vec3<f32>(_e33.g0_.w) * _e37.g0_)) + ((vec3<f32>(_e41.g0_.x) * _e45.g1_.xzy) * vec3<f32>(0.0, 1.0, -(1.0)))), ((((((((((vec2<f32>(_e56.g0_.y) * vec2<f32>(_e60.g0_.y)) * vec2<f32>(-(1.0), 0.0)) + ((vec2<f32>(_e70.g0_.z) * vec2<f32>(_e74.g0_.z)) * vec2<f32>(-(1.0), 0.0))) + ((vec2<f32>(_e85.g1_.x) * vec2<f32>(_e89.g1_.x)) * vec2<f32>(0.0, -(1.0)))) + ((vec2<f32>(_e100.g1_.y) * vec2<f32>(_e104.g1_.y)) * vec2<f32>(0.0, -(1.0)))) + ((vec2<f32>(_e115.g1_.z) * vec2<f32>(_e119.g1_.z)) * vec2<f32>(0.0, -(1.0)))) + ((vec2<f32>(_e130.g2_.x) * vec2<f32>(_e134.g0_.x)) * vec2<f32>(0.0, -(1.0)))) + ((vec2<f32>(_e145.g2_.y) * vec2<f32>(_e149.g0_.y)) * vec2<f32>(0.0, -(1.0)))) + ((vec2<f32>(_e160.g2_.z) * vec2<f32>(_e164.g0_.z)) * vec2<f32>(0.0, -(1.0)))) + ((vec2<f32>(_e175.g0_.x) * vec2<f32>(_e179.g0_.x)) * vec2<f32>(-(1.0), 0.0))));
}

fn circle_circle_add(self_626: Circle, other_532: Circle) -> Circle {
    var self_627: Circle;
    var other_533: Circle;

    self_627 = self_626;
    other_533 = other_532;
    let _e4: Circle = self_627;
    let _e6: Circle = other_533;
    let _e9: Circle = self_627;
    let _e11: Circle = other_533;
    let _e14: Circle = self_627;
    let _e16: Circle = other_533;
    return Circle((_e4.g0_ + _e6.g0_), (_e9.g1_ + _e11.g1_), (_e14.g2_ + _e16.g2_));
}

fn circle_circle_sub(self_628: Circle, other_534: Circle) -> Circle {
    var self_629: Circle;
    var other_535: Circle;

    self_629 = self_628;
    other_535 = other_534;
    let _e4: Circle = self_629;
    let _e6: Circle = other_535;
    let _e9: Circle = self_629;
    let _e11: Circle = other_535;
    let _e14: Circle = self_629;
    let _e16: Circle = other_535;
    return Circle((_e4.g0_ - _e6.g0_), (_e9.g1_ - _e11.g1_), (_e14.g2_ - _e16.g2_));
}

fn circle_circle_mul(self_630: Circle, other_536: Circle) -> Circle {
    var self_631: Circle;
    var other_537: Circle;

    self_631 = self_630;
    other_537 = other_536;
    let _e4: Circle = self_631;
    let _e6: Circle = other_537;
    let _e9: Circle = self_631;
    let _e11: Circle = other_537;
    let _e14: Circle = self_631;
    let _e16: Circle = other_537;
    return Circle((_e4.g0_ * _e6.g0_), (_e9.g1_ * _e11.g1_), (_e14.g2_ * _e16.g2_));
}

fn circle_circle_div(self_632: Circle, other_538: Circle) -> Circle {
    var self_633: Circle;
    var other_539: Circle;

    self_633 = self_632;
    other_539 = other_538;
    let _e4: Circle = self_633;
    let _e7: Circle = self_633;
    let _e10: Circle = self_633;
    let _e13: Circle = self_633;
    let _e23: Circle = other_539;
    let _e26: Circle = other_539;
    let _e29: Circle = other_539;
    let _e32: Circle = other_539;
    let _e43: Circle = self_633;
    let _e46: Circle = self_633;
    let _e49: Circle = self_633;
    let _e58: Circle = other_539;
    let _e61: Circle = other_539;
    let _e64: Circle = other_539;
    let _e74: Circle = self_633;
    let _e77: Circle = self_633;
    let _e80: Circle = self_633;
    let _e89: Circle = other_539;
    let _e92: Circle = other_539;
    let _e95: Circle = other_539;
    return Circle((((vec4<f32>(_e4.g0_.x, _e7.g0_.y, _e10.g0_.z, _e13.g0_.w) * vec4<f32>(1.0, 1.0, 1.0, 1.0)) / vec4<f32>(_e23.g0_.x, _e26.g0_.y, _e29.g0_.z, _e32.g0_.w)) * vec4<f32>(1.0, 1.0, 1.0, 1.0)), (((vec3<f32>(_e43.g1_.x, _e46.g1_.y, _e49.g1_.z) * vec3<f32>(1.0, 1.0, 1.0)) / vec3<f32>(_e58.g1_.x, _e61.g1_.y, _e64.g1_.z)) * vec3<f32>(1.0, 1.0, 1.0)), (((vec3<f32>(_e74.g2_.x, _e77.g2_.y, _e80.g2_.z) * vec3<f32>(1.0, 1.0, 1.0)) / vec3<f32>(_e89.g2_.x, _e92.g2_.y, _e95.g2_.z)) * vec3<f32>(1.0, 1.0, 1.0)));
}

fn circle_circle_regressive_product(self_634: Circle, other_540: Circle) -> RadialPoint {
    var self_635: Circle;
    var other_541: Circle;

    self_635 = self_634;
    other_541 = other_540;
    let _e4: Circle = self_635;
    let _e8: Circle = other_541;
    let _e18: Circle = self_635;
    let _e22: Circle = other_541;
    let _e33: Circle = self_635;
    let _e37: Circle = other_541;
    let _e41: Circle = self_635;
    let _e45: Circle = other_541;
    let _e56: Circle = self_635;
    let _e60: Circle = other_541;
    let _e71: Circle = self_635;
    let _e75: Circle = other_541;
    let _e86: Circle = self_635;
    let _e90: Circle = other_541;
    let _e93: Circle = other_541;
    let _e96: Circle = other_541;
    let _e108: Circle = self_635;
    let _e112: Circle = other_541;
    let _e115: Circle = other_541;
    let _e118: Circle = other_541;
    let _e130: Circle = self_635;
    let _e134: Circle = other_541;
    let _e137: Circle = other_541;
    let _e140: Circle = other_541;
    let _e152: Circle = self_635;
    let _e156: Circle = other_541;
    let _e167: Circle = self_635;
    let _e171: Circle = other_541;
    let _e181: Circle = self_635;
    let _e185: Circle = other_541;
    let _e196: Circle = self_635;
    let _e200: Circle = other_541;
    let _e203: Circle = other_541;
    let _e209: Circle = self_635;
    let _e213: Circle = other_541;
    let _e216: Circle = other_541;
    let _e222: Circle = self_635;
    let _e226: Circle = other_541;
    let _e229: Circle = other_541;
    let _e235: Circle = self_635;
    let _e239: Circle = other_541;
    let _e250: Circle = self_635;
    let _e254: Circle = other_541;
    let _e265: Circle = self_635;
    let _e269: Circle = other_541;
    let _e280: Circle = self_635;
    let _e284: Circle = other_541;
    return RadialPoint((((((((((((vec3<f32>(_e4.g0_.y) * _e8.g2_.zzx) * vec3<f32>(-(1.0), 0.0, 1.0)) + ((vec3<f32>(_e18.g0_.z) * _e22.g2_.yxy) * vec3<f32>(1.0, -(1.0), 0.0))) + (vec3<f32>(_e33.g0_.w) * _e37.g1_)) + ((vec3<f32>(_e41.g1_.x) * vec3<f32>(_e45.g0_.w)) * vec3<f32>(1.0, 0.0, 0.0))) + ((vec3<f32>(_e56.g1_.y) * vec3<f32>(_e60.g0_.w)) * vec3<f32>(0.0, 1.0, 0.0))) + ((vec3<f32>(_e71.g1_.z) * vec3<f32>(_e75.g0_.w)) * vec3<f32>(0.0, 0.0, 1.0))) + ((vec3<f32>(_e86.g2_.x) * vec3<f32>(_e90.g0_.z, _e93.g0_.z, _e96.g0_.y)) * vec3<f32>(0.0, -(1.0), 1.0))) + ((vec3<f32>(_e108.g2_.y) * vec3<f32>(_e112.g0_.z, _e115.g0_.z, _e118.g0_.x)) * vec3<f32>(1.0, 0.0, -(1.0)))) + ((vec3<f32>(_e130.g2_.z) * vec3<f32>(_e134.g0_.y, _e137.g0_.x, _e140.g0_.y)) * vec3<f32>(-(1.0), 1.0, 0.0))) + ((vec3<f32>(_e152.g0_.x) * _e156.g2_.xzy) * vec3<f32>(0.0, 1.0, -(1.0)))), ((((((((((vec2<f32>(_e167.g0_.y) * vec2<f32>(_e171.g1_.y)) * vec2<f32>(-(1.0), 0.0)) + ((vec2<f32>(_e181.g0_.z) * vec2<f32>(_e185.g1_.z)) * vec2<f32>(-(1.0), 0.0))) - (vec2<f32>(_e196.g1_.x) * vec2<f32>(_e200.g0_.x, _e203.g2_.x))) - (vec2<f32>(_e209.g1_.y) * vec2<f32>(_e213.g0_.y, _e216.g2_.y))) - (vec2<f32>(_e222.g1_.z) * vec2<f32>(_e226.g0_.z, _e229.g2_.z))) + ((vec2<f32>(_e235.g2_.x) * vec2<f32>(_e239.g1_.x)) * vec2<f32>(0.0, -(1.0)))) + ((vec2<f32>(_e250.g2_.y) * vec2<f32>(_e254.g1_.y)) * vec2<f32>(0.0, -(1.0)))) + ((vec2<f32>(_e265.g2_.z) * vec2<f32>(_e269.g1_.z)) * vec2<f32>(0.0, -(1.0)))) + ((vec2<f32>(_e280.g0_.x) * vec2<f32>(_e284.g1_.x)) * vec2<f32>(-(1.0), 0.0))));
}

fn circle_circle_inner_product(self_636: Circle, other_542: Circle) -> Scalar {
    var self_637: Circle;
    var other_543: Circle;

    self_637 = self_636;
    other_543 = other_542;
    let _e5: Circle = self_637;
    let _e8: Circle = other_543;
    return Scalar((0.0 - (_e5.g0_.w * _e8.g0_.w)));
}

fn circle_circle_left_contraction(self_638: Circle, other_544: Circle) -> Scalar {
    var self_639: Circle;
    var other_545: Circle;

    self_639 = self_638;
    other_545 = other_544;
    let _e5: Circle = self_639;
    let _e8: Circle = other_545;
    return Scalar((0.0 - (_e5.g0_.w * _e8.g0_.w)));
}

fn circle_circle_right_contraction(self_640: Circle, other_546: Circle) -> Scalar {
    var self_641: Circle;
    var other_547: Circle;

    self_641 = self_640;
    other_547 = other_546;
    let _e5: Circle = self_641;
    let _e8: Circle = other_547;
    return Scalar((0.0 - (_e5.g0_.w * _e8.g0_.w)));
}

fn circle_circle_scalar_product(self_642: Circle, other_548: Circle) -> Scalar {
    var self_643: Circle;
    var other_549: Circle;

    self_643 = self_642;
    other_549 = other_548;
    let _e5: Circle = self_643;
    let _e8: Circle = other_549;
    return Scalar((0.0 - (_e5.g0_.w * _e8.g0_.w)));
}

fn circle_plane_regressive_product(self_644: Circle, other_550: Plane) -> Dipole {
    var self_645: Circle;
    var other_551: Plane;

    self_645 = self_644;
    other_551 = other_550;
    let _e4: Circle = self_645;
    let _e8: Plane = other_551;
    let _e11: Plane = other_551;
    let _e14: Plane = other_551;
    let _e25: Circle = self_645;
    let _e29: Plane = other_551;
    let _e32: Plane = other_551;
    let _e35: Plane = other_551;
    let _e47: Circle = self_645;
    let _e51: Plane = other_551;
    let _e54: Plane = other_551;
    let _e57: Plane = other_551;
    let _e71: Circle = self_645;
    let _e75: Plane = other_551;
    let _e78: Plane = other_551;
    let _e81: Plane = other_551;
    let _e87: Circle = self_645;
    let _e90: Circle = self_645;
    let _e93: Circle = self_645;
    let _e97: Plane = other_551;
    let _e103: Circle = self_645;
    let _e107: Plane = other_551;
    let _e118: Circle = self_645;
    let _e122: Plane = other_551;
    let _e134: Circle = self_645;
    let _e138: Plane = other_551;
    let _e150: Circle = self_645;
    let _e154: Plane = other_551;
    let _e166: Circle = self_645;
    let _e169: Circle = self_645;
    let _e172: Circle = self_645;
    let _e175: Circle = self_645;
    let _e179: Plane = other_551;
    return Dipole(((((vec3<f32>(_e4.g0_.y) * vec3<f32>(_e8.g0_.z, _e11.g0_.z, _e14.g0_.x)) * vec3<f32>(-(1.0), 0.0, 1.0)) + ((vec3<f32>(_e25.g0_.z) * vec3<f32>(_e29.g0_.y, _e32.g0_.x, _e35.g0_.y)) * vec3<f32>(1.0, -(1.0), 0.0))) + ((vec3<f32>(_e47.g0_.x) * vec3<f32>(_e51.g0_.x, _e54.g0_.z, _e57.g0_.y)) * vec3<f32>(0.0, 1.0, -(1.0)))), ((vec3<f32>(0.0) - (vec3<f32>(_e71.g0_.w) * vec3<f32>(_e75.g0_.x, _e78.g0_.y, _e81.g0_.z))) + (vec3<f32>(_e87.g0_.x, _e90.g0_.y, _e93.g0_.z) * vec3<f32>(_e97.g0_.w))), ((((((vec4<f32>(_e103.g1_.y) * _e107.g0_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e118.g1_.z) * _e122.g0_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e134.g2_.y) * _e138.g0_.zzxz) * vec4<f32>(1.0, 0.0, -(1.0), 0.0))) + ((vec4<f32>(_e150.g2_.z) * _e154.g0_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))) + ((vec4<f32>(_e166.g1_.x, _e169.g2_.x, _e172.g2_.x, _e175.g1_.x) * _e179.g0_.wzyx) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))));
}

fn circle_sphere_regressive_product(self_646: Circle, other_552: Sphere) -> Dipole {
    var self_647: Circle;
    var other_553: Sphere;

    self_647 = self_646;
    other_553 = other_552;
    let _e4: Circle = self_647;
    let _e8: Sphere = other_553;
    let _e18: Circle = self_647;
    let _e22: Sphere = other_553;
    let _e33: Circle = self_647;
    let _e37: Sphere = other_553;
    let _e48: Circle = self_647;
    let _e52: Sphere = other_553;
    let _e63: Circle = self_647;
    let _e67: Sphere = other_553;
    let _e78: Circle = self_647;
    let _e82: Sphere = other_553;
    let _e95: Circle = self_647;
    let _e99: Sphere = other_553;
    let _e103: Circle = self_647;
    let _e107: Sphere = other_553;
    let _e118: Circle = self_647;
    let _e122: Sphere = other_553;
    let _e133: Circle = self_647;
    let _e137: Sphere = other_553;
    let _e148: Circle = self_647;
    let _e151: Circle = self_647;
    let _e154: Circle = self_647;
    let _e158: Sphere = other_553;
    let _e164: Circle = self_647;
    let _e168: Sphere = other_553;
    let _e171: Sphere = other_553;
    let _e174: Sphere = other_553;
    let _e177: Sphere = other_553;
    let _e189: Circle = self_647;
    let _e193: Sphere = other_553;
    let _e196: Sphere = other_553;
    let _e199: Sphere = other_553;
    let _e202: Sphere = other_553;
    let _e215: Circle = self_647;
    let _e219: Sphere = other_553;
    let _e222: Sphere = other_553;
    let _e225: Sphere = other_553;
    let _e228: Sphere = other_553;
    let _e241: Circle = self_647;
    let _e245: Sphere = other_553;
    let _e248: Sphere = other_553;
    let _e251: Sphere = other_553;
    let _e254: Sphere = other_553;
    let _e267: Circle = self_647;
    let _e271: Sphere = other_553;
    let _e274: Sphere = other_553;
    let _e277: Sphere = other_553;
    let _e280: Sphere = other_553;
    let _e293: Circle = self_647;
    let _e297: Sphere = other_553;
    let _e300: Sphere = other_553;
    let _e303: Sphere = other_553;
    let _e306: Sphere = other_553;
    return Dipole((((((((vec3<f32>(_e4.g0_.y) * _e8.g1_.zzx) * vec3<f32>(-(1.0), 0.0, 1.0)) + ((vec3<f32>(_e18.g0_.z) * _e22.g1_.yxy) * vec3<f32>(1.0, -(1.0), 0.0))) + ((vec3<f32>(_e33.g1_.x) * vec3<f32>(_e37.g0_.x)) * vec3<f32>(1.0, 0.0, 0.0))) + ((vec3<f32>(_e48.g1_.y) * vec3<f32>(_e52.g0_.x)) * vec3<f32>(0.0, 1.0, 0.0))) + ((vec3<f32>(_e63.g1_.z) * vec3<f32>(_e67.g0_.x)) * vec3<f32>(0.0, 0.0, 1.0))) + ((vec3<f32>(_e78.g0_.x) * _e82.g1_.xzy) * vec3<f32>(0.0, 1.0, -(1.0)))), (((((vec3<f32>(0.0) - (vec3<f32>(_e95.g0_.w) * _e99.g1_)) + ((vec3<f32>(_e103.g2_.x) * vec3<f32>(_e107.g0_.x)) * vec3<f32>(1.0, 0.0, 0.0))) + ((vec3<f32>(_e118.g2_.y) * vec3<f32>(_e122.g0_.x)) * vec3<f32>(0.0, 1.0, 0.0))) + ((vec3<f32>(_e133.g2_.z) * vec3<f32>(_e137.g0_.x)) * vec3<f32>(0.0, 0.0, 1.0))) + (vec3<f32>(_e148.g0_.x, _e151.g0_.y, _e154.g0_.z) * vec3<f32>(_e158.g0_.y))), (((((((vec4<f32>(_e164.g1_.y) * vec4<f32>(_e168.g0_.y, _e171.g0_.y, _e174.g0_.y, _e177.g1_.y)) * vec4<f32>(0.0, 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e189.g1_.z) * vec4<f32>(_e193.g0_.y, _e196.g0_.y, _e199.g0_.y, _e202.g1_.z)) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e215.g2_.x) * vec4<f32>(_e219.g1_.z, _e222.g1_.z, _e225.g1_.y, _e228.g1_.z)) * vec4<f32>(0.0, -(1.0), 1.0, 0.0))) + ((vec4<f32>(_e241.g2_.y) * vec4<f32>(_e245.g1_.z, _e248.g1_.z, _e251.g1_.x, _e254.g1_.z)) * vec4<f32>(1.0, 0.0, -(1.0), 0.0))) + ((vec4<f32>(_e267.g2_.z) * vec4<f32>(_e271.g1_.y, _e274.g1_.x, _e277.g1_.y, _e280.g1_.y)) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))) + ((vec4<f32>(_e293.g1_.x) * vec4<f32>(_e297.g0_.y, _e300.g0_.x, _e303.g0_.x, _e306.g1_.x)) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))));
}

fn circle_motor_geometric_product(self_648: Circle, other_554: Motor) -> Flector {
    var self_649: Circle;
    var other_555: Motor;

    self_649 = self_648;
    other_555 = other_554;
    let _e4: Circle = self_649;
    let _e8: Motor = other_555;
    let _e20: Circle = self_649;
    let _e24: Motor = other_555;
    let _e37: Circle = self_649;
    let _e41: Motor = other_555;
    let _e44: Motor = other_555;
    let _e47: Motor = other_555;
    let _e50: Motor = other_555;
    let _e56: Circle = self_649;
    let _e60: Motor = other_555;
    let _e73: Circle = self_649;
    let _e77: Motor = other_555;
    let _e88: Circle = self_649;
    let _e92: Motor = other_555;
    let _e104: Circle = self_649;
    let _e108: Motor = other_555;
    let _e111: Motor = other_555;
    let _e114: Motor = other_555;
    let _e117: Motor = other_555;
    let _e123: Circle = self_649;
    let _e127: Motor = other_555;
    return Flector((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g1_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0))) + ((vec4<f32>(_e20.g0_.z) * vec4<f32>(_e24.g1_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + (vec4<f32>(_e37.g0_.w) * vec4<f32>(_e41.g1_.x, _e44.g1_.y, _e47.g1_.z, _e50.g0_.w))) + ((vec4<f32>(_e56.g0_.x) * vec4<f32>(_e60.g1_.x)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))), (((((vec4<f32>(_e73.g0_.y) * _e77.g1_.zwxz) * vec4<f32>(-(1.0), 1.0, 1.0, 0.0)) + ((vec4<f32>(_e88.g0_.z) * _e92.g1_.yxwy) * vec4<f32>(1.0, -(1.0), 1.0, 0.0))) + (vec4<f32>(_e104.g0_.w) * vec4<f32>(_e108.g0_.x, _e111.g0_.y, _e114.g0_.z, _e117.g1_.w))) + ((vec4<f32>(_e123.g0_.x) * _e127.g1_.wzyx) * vec4<f32>(1.0, 1.0, -(1.0), 0.0))));
}

fn circle_motor_outer_product(self_650: Circle, other_556: Motor) -> Plane {
    var self_651: Circle;
    var other_557: Motor;

    self_651 = self_650;
    other_557 = other_556;
    let _e4: Circle = self_651;
    let _e6: Motor = other_557;
    return Plane((_e4.g0_ * vec4<f32>(_e6.g1_.w)));
}

fn circle_flector_geometric_product(self_652: Circle, other_558: Flector) -> Motor {
    var self_653: Circle;
    var other_559: Flector;

    self_653 = self_652;
    other_559 = other_558;
    let _e4: Circle = self_653;
    let _e8: Flector = other_559;
    let _e11: Flector = other_559;
    let _e14: Flector = other_559;
    let _e17: Flector = other_559;
    let _e30: Circle = self_653;
    let _e34: Flector = other_559;
    let _e37: Flector = other_559;
    let _e40: Flector = other_559;
    let _e43: Flector = other_559;
    let _e57: Circle = self_653;
    let _e61: Flector = other_559;
    let _e64: Flector = other_559;
    let _e67: Flector = other_559;
    let _e70: Flector = other_559;
    let _e84: Circle = self_653;
    let _e88: Flector = other_559;
    let _e91: Flector = other_559;
    let _e94: Flector = other_559;
    let _e97: Flector = other_559;
    let _e105: Circle = self_653;
    let _e109: Flector = other_559;
    let _e112: Flector = other_559;
    let _e115: Flector = other_559;
    let _e118: Flector = other_559;
    return Motor((((((vec4<f32>(_e4.g0_.x) * vec4<f32>(_e8.g1_.w, _e11.g0_.z, _e14.g0_.y, _e17.g0_.x)) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0))) + ((vec4<f32>(_e30.g0_.y) * vec4<f32>(_e34.g0_.z, _e37.g1_.w, _e40.g0_.x, _e43.g0_.y)) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e57.g0_.z) * vec4<f32>(_e61.g0_.y, _e64.g0_.x, _e67.g1_.w, _e70.g0_.z)) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))) - (vec4<f32>(_e84.g0_.w) * vec4<f32>(_e88.g1_.x, _e91.g1_.y, _e94.g1_.z, _e97.g0_.w))), (vec4<f32>(0.0) - (vec4<f32>(_e105.g0_.w) * vec4<f32>(_e109.g0_.x, _e112.g0_.y, _e115.g0_.z, _e118.g1_.w))));
}

fn circle_flector_outer_product(self_654: Circle, other_560: Flector) -> AntiScalar {
    var self_655: Circle;
    var other_561: Flector;

    self_655 = self_654;
    other_561 = other_560;
    let _e5: Circle = self_655;
    let _e8: Flector = other_561;
    let _e13: Circle = self_655;
    let _e16: Flector = other_561;
    let _e21: Circle = self_655;
    let _e24: Flector = other_561;
    let _e29: Circle = self_655;
    let _e32: Flector = other_561;
    return AntiScalar(((((0.0 - (_e5.g0_.x * _e8.g0_.x)) - (_e13.g0_.y * _e16.g0_.y)) - (_e21.g0_.z * _e24.g0_.z)) - (_e29.g0_.w * _e32.g0_.w)));
}

fn circle_multi_vector_add(self_656: Circle, other_562: MultiVector) -> MultiVector {
    var self_657: Circle;
    var other_563: MultiVector;

    self_657 = self_656;
    other_563 = other_562;
    let _e4: MultiVector = other_563;
    let _e6: MultiVector = other_563;
    let _e8: MultiVector = other_563;
    let _e10: MultiVector = other_563;
    let _e12: MultiVector = other_563;
    let _e14: MultiVector = other_563;
    let _e16: Circle = self_657;
    let _e18: MultiVector = other_563;
    let _e21: Circle = self_657;
    let _e23: MultiVector = other_563;
    let _e26: Circle = self_657;
    let _e28: MultiVector = other_563;
    let _e31: MultiVector = other_563;
    return MultiVector(_e4.g0_, _e6.g1_, _e8.g2_, _e10.g3_, _e12.g4_, _e14.g5_, (_e16.g1_ + _e18.g6_), (_e21.g2_ + _e23.g7_), (_e26.g0_ + _e28.g8_), _e31.g9_);
}

fn circle_multi_vector_sub(self_658: Circle, other_564: MultiVector) -> MultiVector {
    var self_659: Circle;
    var other_565: MultiVector;

    self_659 = self_658;
    other_565 = other_564;
    let _e6: MultiVector = other_565;
    let _e11: MultiVector = other_565;
    let _e16: MultiVector = other_565;
    let _e21: MultiVector = other_565;
    let _e26: MultiVector = other_565;
    let _e31: MultiVector = other_565;
    let _e34: Circle = self_659;
    let _e36: MultiVector = other_565;
    let _e39: Circle = self_659;
    let _e41: MultiVector = other_565;
    let _e44: Circle = self_659;
    let _e46: MultiVector = other_565;
    let _e51: MultiVector = other_565;
    return MultiVector((vec3<f32>(0.0) - _e6.g0_), (vec3<f32>(0.0) - _e11.g1_), (vec2<f32>(0.0) - _e16.g2_), (vec4<f32>(0.0) - _e21.g3_), (vec3<f32>(0.0) - _e26.g4_), (vec3<f32>(0.0) - _e31.g5_), (_e34.g1_ - _e36.g6_), (_e39.g2_ - _e41.g7_), (_e44.g0_ - _e46.g8_), (vec4<f32>(0.0) - _e51.g9_));
}

fn circle_multi_vector_geometric_product(self_660: Circle, other_566: MultiVector) -> MultiVector {
    var self_661: Circle;
    var other_567: MultiVector;

    self_661 = self_660;
    other_567 = other_566;
    let _e4: Circle = self_661;
    let _e8: MultiVector = other_567;
    let _e11: MultiVector = other_567;
    let _e14: MultiVector = other_567;
    let _e26: Circle = self_661;
    let _e30: MultiVector = other_567;
    let _e33: MultiVector = other_567;
    let _e36: MultiVector = other_567;
    let _e49: Circle = self_661;
    let _e53: MultiVector = other_567;
    let _e56: MultiVector = other_567;
    let _e59: MultiVector = other_567;
    let _e65: Circle = self_661;
    let _e69: MultiVector = other_567;
    let _e81: Circle = self_661;
    let _e85: MultiVector = other_567;
    let _e97: Circle = self_661;
    let _e101: MultiVector = other_567;
    let _e113: Circle = self_661;
    let _e117: MultiVector = other_567;
    let _e129: Circle = self_661;
    let _e133: MultiVector = other_567;
    let _e145: Circle = self_661;
    let _e149: MultiVector = other_567;
    let _e161: Circle = self_661;
    let _e165: MultiVector = other_567;
    let _e168: MultiVector = other_567;
    let _e171: MultiVector = other_567;
    let _e184: Circle = self_661;
    let _e188: MultiVector = other_567;
    let _e191: Circle = self_661;
    let _e195: MultiVector = other_567;
    let _e205: Circle = self_661;
    let _e209: MultiVector = other_567;
    let _e220: Circle = self_661;
    let _e224: MultiVector = other_567;
    let _e227: MultiVector = other_567;
    let _e238: Circle = self_661;
    let _e242: MultiVector = other_567;
    let _e253: Circle = self_661;
    let _e257: MultiVector = other_567;
    let _e268: Circle = self_661;
    let _e272: MultiVector = other_567;
    let _e283: Circle = self_661;
    let _e287: MultiVector = other_567;
    let _e298: Circle = self_661;
    let _e302: MultiVector = other_567;
    let _e314: Circle = self_661;
    let _e318: MultiVector = other_567;
    let _e331: Circle = self_661;
    let _e335: MultiVector = other_567;
    let _e338: MultiVector = other_567;
    let _e341: MultiVector = other_567;
    let _e344: MultiVector = other_567;
    let _e350: Circle = self_661;
    let _e354: MultiVector = other_567;
    let _e367: Circle = self_661;
    let _e371: MultiVector = other_567;
    let _e384: Circle = self_661;
    let _e388: MultiVector = other_567;
    let _e401: Circle = self_661;
    let _e405: MultiVector = other_567;
    let _e408: MultiVector = other_567;
    let _e411: MultiVector = other_567;
    let _e414: MultiVector = other_567;
    let _e428: Circle = self_661;
    let _e432: MultiVector = other_567;
    let _e435: MultiVector = other_567;
    let _e438: MultiVector = other_567;
    let _e441: MultiVector = other_567;
    let _e455: Circle = self_661;
    let _e459: MultiVector = other_567;
    let _e462: MultiVector = other_567;
    let _e465: MultiVector = other_567;
    let _e468: MultiVector = other_567;
    let _e482: Circle = self_661;
    let _e486: MultiVector = other_567;
    let _e499: Circle = self_661;
    let _e503: MultiVector = other_567;
    let _e506: MultiVector = other_567;
    let _e509: MultiVector = other_567;
    let _e520: Circle = self_661;
    let _e524: MultiVector = other_567;
    let _e527: MultiVector = other_567;
    let _e530: MultiVector = other_567;
    let _e542: Circle = self_661;
    let _e546: MultiVector = other_567;
    let _e549: MultiVector = other_567;
    let _e552: MultiVector = other_567;
    let _e564: Circle = self_661;
    let _e568: MultiVector = other_567;
    let _e571: MultiVector = other_567;
    let _e574: MultiVector = other_567;
    let _e582: Circle = self_661;
    let _e586: MultiVector = other_567;
    let _e590: Circle = self_661;
    let _e594: MultiVector = other_567;
    let _e597: MultiVector = other_567;
    let _e600: MultiVector = other_567;
    let _e611: Circle = self_661;
    let _e615: MultiVector = other_567;
    let _e618: MultiVector = other_567;
    let _e621: MultiVector = other_567;
    let _e633: Circle = self_661;
    let _e637: MultiVector = other_567;
    let _e640: MultiVector = other_567;
    let _e643: MultiVector = other_567;
    let _e655: Circle = self_661;
    let _e659: MultiVector = other_567;
    let _e662: MultiVector = other_567;
    let _e665: MultiVector = other_567;
    let _e671: Circle = self_661;
    let _e675: MultiVector = other_567;
    let _e678: MultiVector = other_567;
    let _e681: MultiVector = other_567;
    let _e693: Circle = self_661;
    let _e697: MultiVector = other_567;
    let _e700: MultiVector = other_567;
    let _e703: MultiVector = other_567;
    let _e715: Circle = self_661;
    let _e719: MultiVector = other_567;
    let _e722: MultiVector = other_567;
    let _e725: MultiVector = other_567;
    let _e737: Circle = self_661;
    let _e741: MultiVector = other_567;
    let _e744: MultiVector = other_567;
    let _e747: MultiVector = other_567;
    let _e759: Circle = self_661;
    let _e763: MultiVector = other_567;
    let _e766: MultiVector = other_567;
    let _e769: MultiVector = other_567;
    let _e781: Circle = self_661;
    let _e785: MultiVector = other_567;
    let _e788: MultiVector = other_567;
    let _e791: MultiVector = other_567;
    let _e805: Circle = self_661;
    let _e809: MultiVector = other_567;
    let _e812: MultiVector = other_567;
    let _e815: MultiVector = other_567;
    let _e821: Circle = self_661;
    let _e825: MultiVector = other_567;
    let _e828: MultiVector = other_567;
    let _e831: MultiVector = other_567;
    let _e843: Circle = self_661;
    let _e847: MultiVector = other_567;
    let _e850: MultiVector = other_567;
    let _e853: MultiVector = other_567;
    let _e865: Circle = self_661;
    let _e869: MultiVector = other_567;
    let _e872: MultiVector = other_567;
    let _e875: MultiVector = other_567;
    let _e887: Circle = self_661;
    let _e891: MultiVector = other_567;
    let _e894: MultiVector = other_567;
    let _e897: MultiVector = other_567;
    let _e900: MultiVector = other_567;
    let _e912: Circle = self_661;
    let _e916: MultiVector = other_567;
    let _e919: MultiVector = other_567;
    let _e922: MultiVector = other_567;
    let _e925: MultiVector = other_567;
    let _e938: Circle = self_661;
    let _e942: MultiVector = other_567;
    let _e945: MultiVector = other_567;
    let _e948: MultiVector = other_567;
    let _e951: MultiVector = other_567;
    let _e957: Circle = self_661;
    let _e961: MultiVector = other_567;
    let _e964: MultiVector = other_567;
    let _e967: MultiVector = other_567;
    let _e970: MultiVector = other_567;
    let _e983: Circle = self_661;
    let _e987: MultiVector = other_567;
    let _e990: MultiVector = other_567;
    let _e993: MultiVector = other_567;
    let _e996: MultiVector = other_567;
    let _e1008: Circle = self_661;
    let _e1012: MultiVector = other_567;
    let _e1015: MultiVector = other_567;
    let _e1018: MultiVector = other_567;
    let _e1021: MultiVector = other_567;
    let _e1034: Circle = self_661;
    let _e1038: MultiVector = other_567;
    let _e1041: MultiVector = other_567;
    let _e1044: MultiVector = other_567;
    let _e1047: MultiVector = other_567;
    let _e1053: Circle = self_661;
    let _e1057: MultiVector = other_567;
    let _e1060: MultiVector = other_567;
    let _e1063: MultiVector = other_567;
    let _e1066: MultiVector = other_567;
    let _e1079: Circle = self_661;
    let _e1083: MultiVector = other_567;
    let _e1086: MultiVector = other_567;
    let _e1089: MultiVector = other_567;
    let _e1092: MultiVector = other_567;
    let _e1105: Circle = self_661;
    let _e1109: MultiVector = other_567;
    let _e1112: MultiVector = other_567;
    let _e1115: MultiVector = other_567;
    let _e1118: MultiVector = other_567;
    let _e1131: Circle = self_661;
    let _e1135: MultiVector = other_567;
    let _e1138: MultiVector = other_567;
    let _e1141: MultiVector = other_567;
    let _e1144: MultiVector = other_567;
    let _e1158: Circle = self_661;
    let _e1162: MultiVector = other_567;
    let _e1165: MultiVector = other_567;
    let _e1168: MultiVector = other_567;
    let _e1171: MultiVector = other_567;
    let _e1185: Circle = self_661;
    let _e1189: MultiVector = other_567;
    let _e1192: MultiVector = other_567;
    let _e1195: MultiVector = other_567;
    let _e1198: MultiVector = other_567;
    let _e1212: Circle = self_661;
    let _e1216: MultiVector = other_567;
    let _e1219: MultiVector = other_567;
    let _e1222: MultiVector = other_567;
    let _e1225: MultiVector = other_567;
    return MultiVector((((((((((((vec3<f32>(_e4.g0_.y) * vec3<f32>(_e8.g1_.y, _e11.g1_.y, _e14.g3_.y)) * vec3<f32>(0.0, -(1.0), -(1.0))) + ((vec3<f32>(_e26.g0_.z) * vec3<f32>(_e30.g1_.z, _e33.g1_.z, _e36.g3_.z)) * vec3<f32>(0.0, -(1.0), -(1.0)))) - (vec3<f32>(_e49.g0_.w) * vec3<f32>(_e53.g8_.w, _e56.g2_.x, _e59.g3_.w))) + ((vec3<f32>(_e65.g1_.x) * vec3<f32>(_e69.g5_.x)) * vec3<f32>(0.0, 0.0, -(1.0)))) + ((vec3<f32>(_e81.g1_.y) * vec3<f32>(_e85.g5_.y)) * vec3<f32>(0.0, 0.0, -(1.0)))) + ((vec3<f32>(_e97.g1_.z) * vec3<f32>(_e101.g5_.z)) * vec3<f32>(0.0, 0.0, -(1.0)))) + ((vec3<f32>(_e113.g2_.x) * vec3<f32>(_e117.g4_.x)) * vec3<f32>(0.0, 0.0, -(1.0)))) + ((vec3<f32>(_e129.g2_.y) * vec3<f32>(_e133.g4_.y)) * vec3<f32>(0.0, 0.0, -(1.0)))) + ((vec3<f32>(_e145.g2_.z) * vec3<f32>(_e149.g4_.z)) * vec3<f32>(0.0, 0.0, -(1.0)))) + ((vec3<f32>(_e161.g0_.x) * vec3<f32>(_e165.g1_.x, _e168.g1_.x, _e171.g3_.x)) * vec3<f32>(0.0, -(1.0), -(1.0)))), (vec3<f32>(_e184.g0_.w) * _e188.g5_), ((((((((vec2<f32>(_e191.g0_.y) * vec2<f32>(_e195.g5_.y)) * vec2<f32>(-(1.0), 0.0)) + ((vec2<f32>(_e205.g0_.z) * vec2<f32>(_e209.g5_.z)) * vec2<f32>(-(1.0), 0.0))) + ((vec2<f32>(_e220.g0_.w) * vec2<f32>(_e224.g0_.y, _e227.g9_.w)) * vec2<f32>(1.0, -(1.0)))) + ((vec2<f32>(_e238.g2_.x) * vec2<f32>(_e242.g5_.x)) * vec2<f32>(0.0, -(1.0)))) + ((vec2<f32>(_e253.g2_.y) * vec2<f32>(_e257.g5_.y)) * vec2<f32>(0.0, -(1.0)))) + ((vec2<f32>(_e268.g2_.z) * vec2<f32>(_e272.g5_.z)) * vec2<f32>(0.0, -(1.0)))) + ((vec2<f32>(_e283.g0_.x) * vec2<f32>(_e287.g5_.x)) * vec2<f32>(-(1.0), 0.0))), (((((((((((vec4<f32>(_e298.g0_.y) * vec4<f32>(_e302.g7_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0))) + ((vec4<f32>(_e314.g0_.z) * vec4<f32>(_e318.g7_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + (vec4<f32>(_e331.g0_.w) * vec4<f32>(_e335.g7_.x, _e338.g7_.y, _e341.g7_.z, _e344.g0_.z))) + ((vec4<f32>(_e350.g1_.x) * vec4<f32>(_e354.g1_.x)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e367.g1_.y) * vec4<f32>(_e371.g1_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e384.g1_.z) * vec4<f32>(_e388.g1_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e401.g2_.x) * vec4<f32>(_e405.g8_.w, _e408.g1_.z, _e411.g1_.y, _e414.g8_.x)) * vec4<f32>(-(1.0), -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e428.g2_.y) * vec4<f32>(_e432.g1_.z, _e435.g8_.w, _e438.g1_.x, _e441.g8_.y)) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e455.g2_.z) * vec4<f32>(_e459.g1_.y, _e462.g1_.x, _e465.g8_.w, _e468.g8_.z)) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))) + ((vec4<f32>(_e482.g0_.x) * vec4<f32>(_e486.g7_.x)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))), (((((vec3<f32>(_e499.g0_.x) * vec3<f32>(_e503.g8_.w, _e506.g1_.z, _e509.g1_.y)) * vec3<f32>(1.0, 1.0, -(1.0))) + ((vec3<f32>(_e520.g0_.y) * vec3<f32>(_e524.g1_.z, _e527.g8_.w, _e530.g1_.x)) * vec3<f32>(-(1.0), 1.0, 1.0))) + ((vec3<f32>(_e542.g0_.z) * vec3<f32>(_e546.g1_.y, _e549.g1_.x, _e552.g8_.w)) * vec3<f32>(1.0, -(1.0), 1.0))) - (vec3<f32>(_e564.g0_.w) * vec3<f32>(_e568.g8_.x, _e571.g8_.y, _e574.g8_.z))), (vec3<f32>(0.0) - (vec3<f32>(_e582.g0_.w) * _e586.g1_)), (((((((((((vec3<f32>(_e590.g0_.x) * vec3<f32>(_e594.g9_.w, _e597.g3_.z, _e600.g3_.y)) * vec3<f32>(1.0, 1.0, -(1.0))) + ((vec3<f32>(_e611.g0_.y) * vec3<f32>(_e615.g3_.z, _e618.g9_.w, _e621.g3_.x)) * vec3<f32>(-(1.0), 1.0, 1.0))) + ((vec3<f32>(_e633.g0_.z) * vec3<f32>(_e637.g3_.y, _e640.g3_.x, _e643.g9_.w)) * vec3<f32>(1.0, -(1.0), 1.0))) - (vec3<f32>(_e655.g0_.w) * vec3<f32>(_e659.g9_.x, _e662.g9_.y, _e665.g9_.z))) + ((vec3<f32>(_e671.g1_.x) * vec3<f32>(_e675.g0_.x, _e678.g5_.z, _e681.g5_.y)) * vec3<f32>(1.0, 1.0, -(1.0)))) + ((vec3<f32>(_e693.g1_.y) * vec3<f32>(_e697.g5_.z, _e700.g0_.x, _e703.g5_.x)) * vec3<f32>(-(1.0), 1.0, 1.0))) + ((vec3<f32>(_e715.g1_.z) * vec3<f32>(_e719.g5_.y, _e722.g5_.x, _e725.g0_.x)) * vec3<f32>(1.0, -(1.0), 1.0))) + ((vec3<f32>(_e737.g2_.x) * vec3<f32>(_e741.g0_.y, _e744.g4_.z, _e747.g4_.y)) * vec3<f32>(1.0, 1.0, -(1.0)))) + ((vec3<f32>(_e759.g2_.y) * vec3<f32>(_e763.g4_.z, _e766.g0_.y, _e769.g4_.x)) * vec3<f32>(-(1.0), 1.0, 1.0))) + ((vec3<f32>(_e781.g2_.z) * vec3<f32>(_e785.g4_.y, _e788.g4_.x, _e791.g0_.y)) * vec3<f32>(1.0, -(1.0), 1.0))), ((((vec3<f32>(0.0) - (vec3<f32>(_e805.g0_.w) * vec3<f32>(_e809.g3_.x, _e812.g3_.y, _e815.g3_.z))) + ((vec3<f32>(_e821.g2_.x) * vec3<f32>(_e825.g0_.x, _e828.g5_.z, _e831.g5_.y)) * vec3<f32>(1.0, 1.0, -(1.0)))) + ((vec3<f32>(_e843.g2_.y) * vec3<f32>(_e847.g5_.z, _e850.g0_.x, _e853.g5_.x)) * vec3<f32>(-(1.0), 1.0, 1.0))) + ((vec3<f32>(_e865.g2_.z) * vec3<f32>(_e869.g5_.y, _e872.g5_.x, _e875.g0_.x)) * vec3<f32>(1.0, -(1.0), 1.0))), (((((vec4<f32>(_e887.g0_.y) * vec4<f32>(_e891.g5_.z, _e894.g0_.x, _e897.g5_.x, _e900.g5_.z)) * vec4<f32>(-(1.0), 1.0, 1.0, 0.0)) + ((vec4<f32>(_e912.g0_.z) * vec4<f32>(_e916.g5_.y, _e919.g5_.x, _e922.g0_.x, _e925.g5_.y)) * vec4<f32>(1.0, -(1.0), 1.0, 0.0))) + (vec4<f32>(_e938.g0_.w) * vec4<f32>(_e942.g4_.x, _e945.g4_.y, _e948.g4_.z, _e951.g0_.x))) + ((vec4<f32>(_e957.g0_.x) * vec4<f32>(_e961.g0_.x, _e964.g5_.z, _e967.g5_.y, _e970.g0_.x)) * vec4<f32>(1.0, 1.0, -(1.0), 0.0))), (((((((((((vec4<f32>(_e983.g0_.y) * vec4<f32>(_e987.g7_.z, _e990.g2_.y, _e993.g7_.x, _e996.g7_.z)) * vec4<f32>(-(1.0), 1.0, 1.0, 0.0)) + ((vec4<f32>(_e1008.g0_.z) * vec4<f32>(_e1012.g7_.y, _e1015.g7_.x, _e1018.g2_.y, _e1021.g7_.y)) * vec4<f32>(1.0, -(1.0), 1.0, 0.0))) + (vec4<f32>(_e1034.g0_.w) * vec4<f32>(_e1038.g6_.x, _e1041.g6_.y, _e1044.g6_.z, _e1047.g2_.y))) + ((vec4<f32>(_e1053.g1_.x) * vec4<f32>(_e1057.g8_.w, _e1060.g1_.z, _e1063.g1_.y, _e1066.g8_.w)) * vec4<f32>(1.0, 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e1079.g1_.y) * vec4<f32>(_e1083.g1_.z, _e1086.g8_.w, _e1089.g1_.x, _e1092.g1_.z)) * vec4<f32>(-(1.0), 1.0, 1.0, 0.0))) + ((vec4<f32>(_e1105.g1_.z) * vec4<f32>(_e1109.g1_.y, _e1112.g1_.x, _e1115.g8_.w, _e1118.g1_.y)) * vec4<f32>(1.0, -(1.0), 1.0, 0.0))) + ((vec4<f32>(_e1131.g2_.x) * vec4<f32>(_e1135.g2_.x, _e1138.g8_.z, _e1141.g8_.y, _e1144.g1_.x)) * vec4<f32>(-(1.0), -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e1158.g2_.y) * vec4<f32>(_e1162.g8_.z, _e1165.g2_.x, _e1168.g8_.x, _e1171.g1_.y)) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e1185.g2_.z) * vec4<f32>(_e1189.g8_.y, _e1192.g8_.x, _e1195.g2_.x, _e1198.g1_.z)) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))) + ((vec4<f32>(_e1212.g0_.x) * vec4<f32>(_e1216.g2_.y, _e1219.g7_.z, _e1222.g7_.y, _e1225.g2_.x)) * vec4<f32>(1.0, 1.0, -(1.0), 0.0))));
}

fn circle_multi_vector_scalar_product(self_662: Circle, other_568: MultiVector) -> Scalar {
    var self_663: Circle;
    var other_569: MultiVector;

    self_663 = self_662;
    other_569 = other_568;
    let _e5: Circle = self_663;
    let _e8: MultiVector = other_569;
    return Scalar((0.0 - (_e5.g0_.w * _e8.g8_.w)));
}

fn circle_squared_magnitude(self_664: Circle) -> Scalar {
    var self_665: Circle;

    self_665 = self_664;
    let _e2: Circle = self_665;
    let _e3: Circle = self_665;
    let _e4: Circle = circle_reversal(_e3);
    let _e5: Scalar = circle_circle_scalar_product(_e2, _e4);
    return _e5;
}

fn circle_magnitude(self_666: Circle) -> Scalar {
    var self_667: Circle;

    self_667 = self_666;
    let _e2: Circle = self_667;
    let _e3: Scalar = circle_squared_magnitude(_e2);
    return Scalar(sqrt(_e3.g0_));
}

fn circle_scale(self_668: Circle, other_570: f32) -> Circle {
    var self_669: Circle;
    var other_571: f32;

    self_669 = self_668;
    other_571 = other_570;
    let _e4: Circle = self_669;
    let _e5: f32 = other_571;
    let _e7: Circle = circle_scalar_geometric_product(_e4, Scalar(_e5));
    return _e7;
}

fn circle_signum(self_670: Circle) -> Circle {
    var self_671: Circle;

    self_671 = self_670;
    let _e2: Circle = self_671;
    let _e3: Circle = self_671;
    let _e4: Scalar = circle_magnitude(_e3);
    let _e9: Circle = circle_scalar_geometric_product(_e2, Scalar((1.0 / _e4.g0_)));
    return _e9;
}

fn circle_inverse(self_672: Circle) -> Circle {
    var self_673: Circle;

    self_673 = self_672;
    let _e2: Circle = self_673;
    let _e3: Circle = circle_reversal(_e2);
    let _e4: Circle = self_673;
    let _e5: Scalar = circle_squared_magnitude(_e4);
    let _e10: Circle = circle_scalar_geometric_product(_e3, Scalar((1.0 / _e5.g0_)));
    return _e10;
}

fn plane_zero() -> Plane {
    return Plane(vec4<f32>(0.0));
}

fn plane_one() -> Plane {
    return Plane(vec4<f32>(0.0));
}

fn plane_neg(self_674: Plane) -> Plane {
    var self_675: Plane;

    self_675 = self_674;
    let _e2: Plane = self_675;
    return Plane((_e2.g0_ * vec4<f32>(-(1.0))));
}

fn plane_automorphism(self_676: Plane) -> Plane {
    var self_677: Plane;

    self_677 = self_676;
    let _e2: Plane = self_677;
    return Plane(_e2.g0_);
}

fn plane_reversal(self_678: Plane) -> Plane {
    var self_679: Plane;

    self_679 = self_678;
    let _e2: Plane = self_679;
    return Plane(_e2.g0_);
}

fn plane_conjugation(self_680: Plane) -> Plane {
    var self_681: Plane;

    self_681 = self_680;
    let _e2: Plane = self_681;
    return Plane(_e2.g0_);
}

fn plane_scalar_geometric_product(self_682: Plane, other_572: Scalar) -> Plane {
    var self_683: Plane;
    var other_573: Scalar;

    self_683 = self_682;
    other_573 = other_572;
    let _e4: Plane = self_683;
    let _e6: Scalar = other_573;
    return Plane((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn plane_scalar_outer_product(self_684: Plane, other_574: Scalar) -> Plane {
    var self_685: Plane;
    var other_575: Scalar;

    self_685 = self_684;
    other_575 = other_574;
    let _e4: Plane = self_685;
    let _e6: Scalar = other_575;
    return Plane((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn plane_scalar_inner_product(self_686: Plane, other_576: Scalar) -> Plane {
    var self_687: Plane;
    var other_577: Scalar;

    self_687 = self_686;
    other_577 = other_576;
    let _e4: Plane = self_687;
    let _e6: Scalar = other_577;
    return Plane((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn plane_scalar_right_contraction(self_688: Plane, other_578: Scalar) -> Plane {
    var self_689: Plane;
    var other_579: Scalar;

    self_689 = self_688;
    other_579 = other_578;
    let _e4: Plane = self_689;
    let _e6: Scalar = other_579;
    return Plane((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn plane_anti_scalar_regressive_product(self_690: Plane, other_580: AntiScalar) -> Plane {
    var self_691: Plane;
    var other_581: AntiScalar;

    self_691 = self_690;
    other_581 = other_580;
    let _e4: Plane = self_691;
    let _e6: AntiScalar = other_581;
    return Plane((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn plane_radial_point_regressive_product(self_692: Plane, other_582: RadialPoint) -> Scalar {
    var self_693: Plane;
    var other_583: RadialPoint;

    self_693 = self_692;
    other_583 = other_582;
    let _e4: Plane = self_693;
    let _e7: RadialPoint = other_583;
    let _e11: Plane = self_693;
    let _e14: RadialPoint = other_583;
    let _e19: Plane = self_693;
    let _e22: RadialPoint = other_583;
    let _e27: Plane = self_693;
    let _e30: RadialPoint = other_583;
    return Scalar(((((_e4.g0_.x * _e7.g0_.x) + (_e11.g0_.y * _e14.g0_.y)) + (_e19.g0_.z * _e22.g0_.z)) + (_e27.g0_.w * _e30.g1_.x)));
}

fn plane_radial_point_outer_product(self_694: Plane, other_584: RadialPoint) -> AntiScalar {
    var self_695: Plane;
    var other_585: RadialPoint;

    self_695 = self_694;
    other_585 = other_584;
    let _e4: Plane = self_695;
    let _e7: RadialPoint = other_585;
    let _e11: Plane = self_695;
    let _e14: RadialPoint = other_585;
    let _e19: Plane = self_695;
    let _e22: RadialPoint = other_585;
    let _e27: Plane = self_695;
    let _e30: RadialPoint = other_585;
    return AntiScalar(((((_e4.g0_.x * _e7.g0_.x) + (_e11.g0_.y * _e14.g0_.y)) + (_e19.g0_.z * _e22.g0_.z)) + (_e27.g0_.w * _e30.g1_.x)));
}

fn plane_radial_point_inner_product(self_696: Plane, other_586: RadialPoint) -> Line {
    var self_697: Plane;
    var other_587: RadialPoint;

    self_697 = self_696;
    other_587 = other_586;
    let _e4: Plane = self_697;
    let _e8: RadialPoint = other_587;
    let _e18: Plane = self_697;
    let _e22: RadialPoint = other_587;
    let _e33: Plane = self_697;
    let _e37: RadialPoint = other_587;
    let _e48: Plane = self_697;
    let _e52: RadialPoint = other_587;
    return Line(((((vec3<f32>(_e4.g0_.y) * _e8.g0_.zzx) * vec3<f32>(1.0, 0.0, -(1.0))) + ((vec3<f32>(_e18.g0_.z) * _e22.g0_.yxy) * vec3<f32>(-(1.0), 1.0, 0.0))) + ((vec3<f32>(_e33.g0_.x) * _e37.g0_.xzy) * vec3<f32>(0.0, -(1.0), 1.0))), (vec3<f32>(_e48.g0_.w) * _e52.g0_));
}

fn plane_radial_point_right_contraction(self_698: Plane, other_588: RadialPoint) -> Line {
    var self_699: Plane;
    var other_589: RadialPoint;

    self_699 = self_698;
    other_589 = other_588;
    let _e4: Plane = self_699;
    let _e8: RadialPoint = other_589;
    let _e18: Plane = self_699;
    let _e22: RadialPoint = other_589;
    let _e33: Plane = self_699;
    let _e37: RadialPoint = other_589;
    let _e48: Plane = self_699;
    let _e52: RadialPoint = other_589;
    return Line(((((vec3<f32>(_e4.g0_.y) * _e8.g0_.zzx) * vec3<f32>(1.0, 0.0, -(1.0))) + ((vec3<f32>(_e18.g0_.z) * _e22.g0_.yxy) * vec3<f32>(-(1.0), 1.0, 0.0))) + ((vec3<f32>(_e33.g0_.x) * _e37.g0_.xzy) * vec3<f32>(0.0, -(1.0), 1.0))), (vec3<f32>(_e48.g0_.w) * _e52.g0_));
}

fn plane_flat_point_add(self_700: Plane, other_590: FlatPoint) -> Flector {
    var self_701: Plane;
    var other_591: FlatPoint;

    self_701 = self_700;
    other_591 = other_590;
    let _e4: FlatPoint = other_591;
    let _e6: Plane = self_701;
    return Flector(_e4.g0_, _e6.g0_);
}

fn plane_flat_point_sub(self_702: Plane, other_592: FlatPoint) -> Flector {
    var self_703: Plane;
    var other_593: FlatPoint;

    self_703 = self_702;
    other_593 = other_592;
    let _e6: FlatPoint = other_593;
    let _e9: Plane = self_703;
    return Flector((vec4<f32>(0.0) - _e6.g0_), _e9.g0_);
}

fn plane_dipole_regressive_product(self_704: Plane, other_594: Dipole) -> RadialPoint {
    var self_705: Plane;
    var other_595: Dipole;

    self_705 = self_704;
    other_595 = other_594;
    let _e4: Plane = self_705;
    let _e8: Dipole = other_595;
    let _e18: Plane = self_705;
    let _e22: Dipole = other_595;
    let _e33: Plane = self_705;
    let _e37: Dipole = other_595;
    let _e41: Plane = self_705;
    let _e45: Dipole = other_595;
    let _e56: Plane = self_705;
    let _e60: Dipole = other_595;
    let _e63: Dipole = other_595;
    let _e73: Plane = self_705;
    let _e77: Dipole = other_595;
    let _e80: Dipole = other_595;
    let _e91: Plane = self_705;
    let _e95: Dipole = other_595;
    let _e98: Dipole = other_595;
    let _e109: Plane = self_705;
    let _e112: Plane = self_705;
    let _e116: Dipole = other_595;
    let _e119: Dipole = other_595;
    return RadialPoint((((((vec3<f32>(_e4.g0_.y) * _e8.g1_.zzx) * vec3<f32>(1.0, 0.0, -(1.0))) + ((vec3<f32>(_e18.g0_.z) * _e22.g1_.yxy) * vec3<f32>(-(1.0), 1.0, 0.0))) - (vec3<f32>(_e33.g0_.w) * _e37.g0_)) + ((vec3<f32>(_e41.g0_.x) * _e45.g1_.xzy) * vec3<f32>(0.0, -(1.0), 1.0))), (((((vec2<f32>(_e56.g0_.x) * vec2<f32>(_e60.g0_.x, _e63.g2_.x)) * vec2<f32>(1.0, -(1.0))) + ((vec2<f32>(_e73.g0_.y) * vec2<f32>(_e77.g0_.y, _e80.g2_.y)) * vec2<f32>(1.0, -(1.0)))) + ((vec2<f32>(_e91.g0_.z) * vec2<f32>(_e95.g0_.z, _e98.g2_.z)) * vec2<f32>(1.0, -(1.0)))) + ((vec2<f32>(_e109.g0_.x, _e112.g0_.w) * vec2<f32>(_e116.g2_.x, _e119.g2_.w)) * vec2<f32>(0.0, -(1.0)))));
}

fn plane_dipole_inner_product(self_706: Plane, other_596: Dipole) -> FlatPoint {
    var self_707: Plane;
    var other_597: Dipole;

    self_707 = self_706;
    other_597 = other_596;
    let _e4: Plane = self_707;
    let _e8: Dipole = other_597;
    let _e20: Plane = self_707;
    let _e24: Dipole = other_597;
    let _e37: Plane = self_707;
    let _e40: Dipole = other_597;
    let _e43: Dipole = other_597;
    let _e46: Dipole = other_597;
    let _e49: Dipole = other_597;
    return FlatPoint(((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g1_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0))) + ((vec4<f32>(_e20.g0_.z) * vec4<f32>(_e24.g1_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((_e37.g0_.wwwx * vec4<f32>(_e40.g1_.x, _e43.g1_.y, _e46.g1_.z, _e49.g1_.x)) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))));
}

fn plane_dipole_right_contraction(self_708: Plane, other_598: Dipole) -> FlatPoint {
    var self_709: Plane;
    var other_599: Dipole;

    self_709 = self_708;
    other_599 = other_598;
    let _e4: Plane = self_709;
    let _e8: Dipole = other_599;
    let _e20: Plane = self_709;
    let _e24: Dipole = other_599;
    let _e37: Plane = self_709;
    let _e40: Dipole = other_599;
    let _e43: Dipole = other_599;
    let _e46: Dipole = other_599;
    let _e49: Dipole = other_599;
    return FlatPoint(((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g1_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0))) + ((vec4<f32>(_e20.g0_.z) * vec4<f32>(_e24.g1_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((_e37.g0_.wwwx * vec4<f32>(_e40.g1_.x, _e43.g1_.y, _e46.g1_.z, _e49.g1_.x)) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))));
}

fn plane_line_regressive_product(self_710: Plane, other_600: Line) -> FlatPoint {
    var self_711: Plane;
    var other_601: Line;

    self_711 = self_710;
    other_601 = other_600;
    let _e4: Plane = self_711;
    let _e8: Line = other_601;
    let _e11: Line = other_601;
    let _e14: Line = other_601;
    let _e17: Line = other_601;
    let _e30: Plane = self_711;
    let _e34: Line = other_601;
    let _e37: Line = other_601;
    let _e40: Line = other_601;
    let _e43: Line = other_601;
    let _e57: Plane = self_711;
    let _e61: Line = other_601;
    let _e64: Line = other_601;
    let _e67: Line = other_601;
    let _e70: Line = other_601;
    let _e82: Plane = self_711;
    let _e86: Line = other_601;
    let _e89: Line = other_601;
    let _e92: Line = other_601;
    let _e95: Line = other_601;
    return FlatPoint((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g1_.z, _e11.g1_.z, _e14.g1_.x, _e17.g0_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e30.g0_.z) * vec4<f32>(_e34.g1_.y, _e37.g1_.x, _e40.g1_.y, _e43.g0_.z)) * vec4<f32>(1.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e57.g0_.w) * vec4<f32>(_e61.g0_.x, _e64.g0_.y, _e67.g0_.z, _e70.g0_.x)) * vec4<f32>(1.0, 1.0, 1.0, 0.0))) + ((vec4<f32>(_e82.g0_.x) * vec4<f32>(_e86.g1_.x, _e89.g1_.z, _e92.g1_.y, _e95.g0_.x)) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))));
}

fn plane_circle_regressive_product(self_712: Plane, other_602: Circle) -> Dipole {
    var self_713: Plane;
    var other_603: Circle;

    self_713 = self_712;
    other_603 = other_602;
    let _e4: Plane = self_713;
    let _e8: Circle = other_603;
    let _e11: Circle = other_603;
    let _e14: Circle = other_603;
    let _e25: Plane = self_713;
    let _e29: Circle = other_603;
    let _e32: Circle = other_603;
    let _e35: Circle = other_603;
    let _e47: Plane = self_713;
    let _e51: Circle = other_603;
    let _e54: Circle = other_603;
    let _e57: Circle = other_603;
    let _e69: Plane = self_713;
    let _e73: Circle = other_603;
    let _e76: Circle = other_603;
    let _e79: Circle = other_603;
    let _e84: Plane = self_713;
    let _e87: Plane = self_713;
    let _e90: Plane = self_713;
    let _e94: Circle = other_603;
    let _e104: Plane = self_713;
    let _e108: Circle = other_603;
    let _e111: Circle = other_603;
    let _e114: Circle = other_603;
    let _e117: Circle = other_603;
    let _e130: Plane = self_713;
    let _e134: Circle = other_603;
    let _e137: Circle = other_603;
    let _e140: Circle = other_603;
    let _e143: Circle = other_603;
    let _e157: Plane = self_713;
    let _e161: Circle = other_603;
    let _e164: Circle = other_603;
    let _e167: Circle = other_603;
    let _e170: Circle = other_603;
    let _e182: Plane = self_713;
    let _e186: Circle = other_603;
    let _e189: Circle = other_603;
    let _e192: Circle = other_603;
    let _e195: Circle = other_603;
    return Dipole(((((vec3<f32>(_e4.g0_.y) * vec3<f32>(_e8.g0_.z, _e11.g0_.z, _e14.g0_.x)) * vec3<f32>(1.0, 0.0, -(1.0))) + ((vec3<f32>(_e25.g0_.z) * vec3<f32>(_e29.g0_.y, _e32.g0_.x, _e35.g0_.y)) * vec3<f32>(-(1.0), 1.0, 0.0))) + ((vec3<f32>(_e47.g0_.x) * vec3<f32>(_e51.g0_.x, _e54.g0_.z, _e57.g0_.y)) * vec3<f32>(0.0, -(1.0), 1.0))), ((vec3<f32>(_e69.g0_.w) * vec3<f32>(_e73.g0_.x, _e76.g0_.y, _e79.g0_.z)) + ((vec3<f32>(_e84.g0_.x, _e87.g0_.y, _e90.g0_.z) * vec3<f32>(_e94.g0_.w)) * vec3<f32>(-(1.0)))), (((((vec4<f32>(_e104.g0_.y) * vec4<f32>(_e108.g2_.z, _e111.g2_.z, _e114.g2_.x, _e117.g1_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e130.g0_.z) * vec4<f32>(_e134.g2_.y, _e137.g2_.x, _e140.g2_.y, _e143.g1_.z)) * vec4<f32>(1.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e157.g0_.w) * vec4<f32>(_e161.g1_.x, _e164.g1_.y, _e167.g1_.z, _e170.g1_.x)) * vec4<f32>(1.0, 1.0, 1.0, 0.0))) + ((vec4<f32>(_e182.g0_.x) * vec4<f32>(_e186.g2_.x, _e189.g2_.z, _e192.g2_.y, _e195.g1_.x)) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))));
}

fn plane_plane_add(self_714: Plane, other_604: Plane) -> Plane {
    var self_715: Plane;
    var other_605: Plane;

    self_715 = self_714;
    other_605 = other_604;
    let _e4: Plane = self_715;
    let _e6: Plane = other_605;
    return Plane((_e4.g0_ + _e6.g0_));
}

fn plane_plane_sub(self_716: Plane, other_606: Plane) -> Plane {
    var self_717: Plane;
    var other_607: Plane;

    self_717 = self_716;
    other_607 = other_606;
    let _e4: Plane = self_717;
    let _e6: Plane = other_607;
    return Plane((_e4.g0_ - _e6.g0_));
}

fn plane_plane_mul(self_718: Plane, other_608: Plane) -> Plane {
    var self_719: Plane;
    var other_609: Plane;

    self_719 = self_718;
    other_609 = other_608;
    let _e4: Plane = self_719;
    let _e6: Plane = other_609;
    return Plane((_e4.g0_ * _e6.g0_));
}

fn plane_plane_div(self_720: Plane, other_610: Plane) -> Plane {
    var self_721: Plane;
    var other_611: Plane;

    self_721 = self_720;
    other_611 = other_610;
    let _e4: Plane = self_721;
    let _e7: Plane = self_721;
    let _e10: Plane = self_721;
    let _e13: Plane = self_721;
    let _e23: Plane = other_611;
    let _e26: Plane = other_611;
    let _e29: Plane = other_611;
    let _e32: Plane = other_611;
    return Plane((((vec4<f32>(_e4.g0_.x, _e7.g0_.y, _e10.g0_.z, _e13.g0_.w) * vec4<f32>(1.0, 1.0, 1.0, 1.0)) / vec4<f32>(_e23.g0_.x, _e26.g0_.y, _e29.g0_.z, _e32.g0_.w)) * vec4<f32>(1.0, 1.0, 1.0, 1.0)));
}

fn plane_plane_regressive_product(self_722: Plane, other_612: Plane) -> Line {
    var self_723: Plane;
    var other_613: Plane;

    self_723 = self_722;
    other_613 = other_612;
    let _e4: Plane = self_723;
    let _e8: Plane = other_613;
    let _e11: Plane = other_613;
    let _e14: Plane = other_613;
    let _e25: Plane = self_723;
    let _e29: Plane = other_613;
    let _e32: Plane = other_613;
    let _e35: Plane = other_613;
    let _e47: Plane = self_723;
    let _e51: Plane = other_613;
    let _e54: Plane = other_613;
    let _e57: Plane = other_613;
    let _e71: Plane = self_723;
    let _e75: Plane = other_613;
    let _e78: Plane = other_613;
    let _e81: Plane = other_613;
    let _e87: Plane = self_723;
    let _e90: Plane = self_723;
    let _e93: Plane = self_723;
    let _e97: Plane = other_613;
    return Line(((((vec3<f32>(_e4.g0_.y) * vec3<f32>(_e8.g0_.z, _e11.g0_.z, _e14.g0_.x)) * vec3<f32>(-(1.0), 0.0, 1.0)) + ((vec3<f32>(_e25.g0_.z) * vec3<f32>(_e29.g0_.y, _e32.g0_.x, _e35.g0_.y)) * vec3<f32>(1.0, -(1.0), 0.0))) + ((vec3<f32>(_e47.g0_.x) * vec3<f32>(_e51.g0_.x, _e54.g0_.z, _e57.g0_.y)) * vec3<f32>(0.0, 1.0, -(1.0)))), ((vec3<f32>(0.0) - (vec3<f32>(_e71.g0_.w) * vec3<f32>(_e75.g0_.x, _e78.g0_.y, _e81.g0_.z))) + (vec3<f32>(_e87.g0_.x, _e90.g0_.y, _e93.g0_.z) * vec3<f32>(_e97.g0_.w))));
}

fn plane_sphere_add(self_724: Plane, other_614: Sphere) -> Sphere {
    var self_725: Plane;
    var other_615: Sphere;

    self_725 = self_724;
    other_615 = other_614;
    let _e4: Plane = self_725;
    let _e7: Plane = self_725;
    let _e15: Sphere = other_615;
    let _e18: Plane = self_725;
    let _e21: Plane = self_725;
    let _e24: Plane = self_725;
    let _e28: Sphere = other_615;
    return Sphere(((vec2<f32>(_e4.g0_.x, _e7.g0_.w) * vec2<f32>(0.0, 1.0)) + _e15.g0_), (vec3<f32>(_e18.g0_.x, _e21.g0_.y, _e24.g0_.z) + _e28.g1_));
}

fn plane_sphere_sub(self_726: Plane, other_616: Sphere) -> Sphere {
    var self_727: Plane;
    var other_617: Sphere;

    self_727 = self_726;
    other_617 = other_616;
    let _e4: Plane = self_727;
    let _e7: Plane = self_727;
    let _e15: Sphere = other_617;
    let _e18: Plane = self_727;
    let _e21: Plane = self_727;
    let _e24: Plane = self_727;
    let _e28: Sphere = other_617;
    return Sphere(((vec2<f32>(_e4.g0_.x, _e7.g0_.w) * vec2<f32>(0.0, 1.0)) - _e15.g0_), (vec3<f32>(_e18.g0_.x, _e21.g0_.y, _e24.g0_.z) - _e28.g1_));
}

fn plane_sphere_regressive_product(self_728: Plane, other_618: Sphere) -> Circle {
    var self_729: Plane;
    var other_619: Sphere;

    self_729 = self_728;
    other_619 = other_618;
    let _e4: Plane = self_729;
    let _e6: Sphere = other_619;
    let _e15: Plane = self_729;
    let _e19: Sphere = other_619;
    let _e29: Plane = self_729;
    let _e33: Sphere = other_619;
    let _e44: Plane = self_729;
    let _e48: Sphere = other_619;
    let _e61: Plane = self_729;
    let _e65: Sphere = other_619;
    let _e69: Plane = self_729;
    let _e72: Plane = self_729;
    let _e75: Plane = self_729;
    let _e79: Sphere = other_619;
    return Circle(((_e4.g0_ * vec4<f32>(_e6.g0_.x)) * vec4<f32>(-(1.0))), ((((vec3<f32>(_e15.g0_.y) * _e19.g1_.zzx) * vec3<f32>(-(1.0), 0.0, 1.0)) + ((vec3<f32>(_e29.g0_.z) * _e33.g1_.yxy) * vec3<f32>(1.0, -(1.0), 0.0))) + ((vec3<f32>(_e44.g0_.x) * _e48.g1_.xzy) * vec3<f32>(0.0, 1.0, -(1.0)))), ((vec3<f32>(0.0) - (vec3<f32>(_e61.g0_.w) * _e65.g1_)) + (vec3<f32>(_e69.g0_.x, _e72.g0_.y, _e75.g0_.z) * vec3<f32>(_e79.g0_.y))));
}

fn plane_motor_regressive_product(self_730: Plane, other_620: Motor) -> Flector {
    var self_731: Plane;
    var other_621: Motor;

    self_731 = self_730;
    other_621 = other_620;
    let _e4: Plane = self_731;
    let _e8: Motor = other_621;
    let _e11: Motor = other_621;
    let _e14: Motor = other_621;
    let _e17: Motor = other_621;
    let _e30: Plane = self_731;
    let _e34: Motor = other_621;
    let _e37: Motor = other_621;
    let _e40: Motor = other_621;
    let _e43: Motor = other_621;
    let _e57: Plane = self_731;
    let _e61: Motor = other_621;
    let _e72: Plane = self_731;
    let _e76: Motor = other_621;
    let _e79: Motor = other_621;
    let _e82: Motor = other_621;
    let _e85: Motor = other_621;
    let _e99: Plane = self_731;
    let _e101: Motor = other_621;
    return Flector((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g1_.z, _e11.g1_.z, _e14.g1_.x, _e17.g0_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e30.g0_.z) * vec4<f32>(_e34.g1_.y, _e37.g1_.x, _e40.g1_.y, _e43.g0_.z)) * vec4<f32>(1.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e57.g0_.w) * _e61.g0_.xyzx) * vec4<f32>(1.0, 1.0, 1.0, 0.0))) + ((vec4<f32>(_e72.g0_.x) * vec4<f32>(_e76.g1_.x, _e79.g1_.z, _e82.g1_.y, _e85.g0_.x)) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))), (_e99.g0_ * vec4<f32>(_e101.g0_.w)));
}

fn plane_rotor_regressive_product(self_732: Plane, other_622: Rotor) -> Flector {
    var self_733: Plane;
    var other_623: Rotor;

    self_733 = self_732;
    other_623 = other_622;
    let _e4: Plane = self_733;
    let _e8: Rotor = other_623;
    let _e20: Plane = self_733;
    let _e24: Rotor = other_623;
    let _e37: Plane = self_733;
    let _e40: Rotor = other_623;
    let _e52: Plane = self_733;
    let _e54: Rotor = other_623;
    return Flector(((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0))) + ((vec4<f32>(_e20.g0_.z) * vec4<f32>(_e24.g0_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((_e37.g0_.wwwx * _e40.g0_.xyzx) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))), (_e52.g0_ * vec4<f32>(_e54.g0_.w)));
}

fn plane_flector_add(self_734: Plane, other_624: Flector) -> Flector {
    var self_735: Plane;
    var other_625: Flector;

    self_735 = self_734;
    other_625 = other_624;
    let _e4: Flector = other_625;
    let _e6: Plane = self_735;
    let _e8: Flector = other_625;
    return Flector(_e4.g0_, (_e6.g0_ + _e8.g1_));
}

fn plane_flector_sub(self_736: Plane, other_626: Flector) -> Flector {
    var self_737: Plane;
    var other_627: Flector;

    self_737 = self_736;
    other_627 = other_626;
    let _e6: Flector = other_627;
    let _e9: Plane = self_737;
    let _e11: Flector = other_627;
    return Flector((vec4<f32>(0.0) - _e6.g0_), (_e9.g0_ - _e11.g1_));
}

fn plane_multi_vector_add(self_738: Plane, other_628: MultiVector) -> MultiVector {
    var self_739: Plane;
    var other_629: MultiVector;

    self_739 = self_738;
    other_629 = other_628;
    let _e4: MultiVector = other_629;
    let _e6: MultiVector = other_629;
    let _e8: MultiVector = other_629;
    let _e10: MultiVector = other_629;
    let _e12: MultiVector = other_629;
    let _e14: MultiVector = other_629;
    let _e16: MultiVector = other_629;
    let _e18: MultiVector = other_629;
    let _e20: MultiVector = other_629;
    let _e22: Plane = self_739;
    let _e24: MultiVector = other_629;
    return MultiVector(_e4.g0_, _e6.g1_, _e8.g2_, _e10.g3_, _e12.g4_, _e14.g5_, _e16.g6_, _e18.g7_, _e20.g8_, (_e22.g0_ + _e24.g9_));
}

fn plane_multi_vector_sub(self_740: Plane, other_630: MultiVector) -> MultiVector {
    var self_741: Plane;
    var other_631: MultiVector;

    self_741 = self_740;
    other_631 = other_630;
    let _e6: MultiVector = other_631;
    let _e11: MultiVector = other_631;
    let _e16: MultiVector = other_631;
    let _e21: MultiVector = other_631;
    let _e26: MultiVector = other_631;
    let _e31: MultiVector = other_631;
    let _e36: MultiVector = other_631;
    let _e41: MultiVector = other_631;
    let _e46: MultiVector = other_631;
    let _e49: Plane = self_741;
    let _e51: MultiVector = other_631;
    return MultiVector((vec3<f32>(0.0) - _e6.g0_), (vec3<f32>(0.0) - _e11.g1_), (vec2<f32>(0.0) - _e16.g2_), (vec4<f32>(0.0) - _e21.g3_), (vec3<f32>(0.0) - _e26.g4_), (vec3<f32>(0.0) - _e31.g5_), (vec3<f32>(0.0) - _e36.g6_), (vec3<f32>(0.0) - _e41.g7_), (vec4<f32>(0.0) - _e46.g8_), (_e49.g0_ - _e51.g9_));
}

fn plane_scale(self_742: Plane, other_632: f32) -> Plane {
    var self_743: Plane;
    var other_633: f32;

    self_743 = self_742;
    other_633 = other_632;
    let _e4: Plane = self_743;
    let _e5: f32 = other_633;
    let _e7: Plane = plane_scalar_geometric_product(_e4, Scalar(_e5));
    return _e7;
}

fn sphere_zero() -> Sphere {
    return Sphere(vec2<f32>(0.0), vec3<f32>(0.0));
}

fn sphere_one() -> Sphere {
    return Sphere(vec2<f32>(0.0), vec3<f32>(0.0));
}

fn sphere_neg(self_744: Sphere) -> Sphere {
    var self_745: Sphere;

    self_745 = self_744;
    let _e2: Sphere = self_745;
    let _e8: Sphere = self_745;
    return Sphere((_e2.g0_ * vec2<f32>(-(1.0))), (_e8.g1_ * vec3<f32>(-(1.0))));
}

fn sphere_automorphism(self_746: Sphere) -> Sphere {
    var self_747: Sphere;

    self_747 = self_746;
    let _e2: Sphere = self_747;
    let _e4: Sphere = self_747;
    return Sphere(_e2.g0_, _e4.g1_);
}

fn sphere_reversal(self_748: Sphere) -> Sphere {
    var self_749: Sphere;

    self_749 = self_748;
    let _e2: Sphere = self_749;
    let _e4: Sphere = self_749;
    return Sphere(_e2.g0_, _e4.g1_);
}

fn sphere_conjugation(self_750: Sphere) -> Sphere {
    var self_751: Sphere;

    self_751 = self_750;
    let _e2: Sphere = self_751;
    let _e4: Sphere = self_751;
    return Sphere(_e2.g0_, _e4.g1_);
}

fn sphere_dual(self_752: Sphere) -> RadialPoint {
    var self_753: Sphere;

    self_753 = self_752;
    let _e2: Sphere = self_753;
    let _e4: Sphere = self_753;
    return RadialPoint(_e2.g1_, _e4.g0_.yx);
}

fn sphere_scalar_geometric_product(self_754: Sphere, other_634: Scalar) -> Sphere {
    var self_755: Sphere;
    var other_635: Scalar;

    self_755 = self_754;
    other_635 = other_634;
    let _e4: Sphere = self_755;
    let _e6: Scalar = other_635;
    let _e10: Sphere = self_755;
    let _e12: Scalar = other_635;
    return Sphere((_e4.g0_ * vec2<f32>(_e6.g0_)), (_e10.g1_ * vec3<f32>(_e12.g0_)));
}

fn sphere_scalar_outer_product(self_756: Sphere, other_636: Scalar) -> Sphere {
    var self_757: Sphere;
    var other_637: Scalar;

    self_757 = self_756;
    other_637 = other_636;
    let _e4: Sphere = self_757;
    let _e6: Scalar = other_637;
    let _e10: Sphere = self_757;
    let _e12: Scalar = other_637;
    return Sphere((_e4.g0_ * vec2<f32>(_e6.g0_)), (_e10.g1_ * vec3<f32>(_e12.g0_)));
}

fn sphere_scalar_inner_product(self_758: Sphere, other_638: Scalar) -> Sphere {
    var self_759: Sphere;
    var other_639: Scalar;

    self_759 = self_758;
    other_639 = other_638;
    let _e4: Sphere = self_759;
    let _e6: Scalar = other_639;
    let _e10: Sphere = self_759;
    let _e12: Scalar = other_639;
    return Sphere((_e4.g0_ * vec2<f32>(_e6.g0_)), (_e10.g1_ * vec3<f32>(_e12.g0_)));
}

fn sphere_scalar_right_contraction(self_760: Sphere, other_640: Scalar) -> Sphere {
    var self_761: Sphere;
    var other_641: Scalar;

    self_761 = self_760;
    other_641 = other_640;
    let _e4: Sphere = self_761;
    let _e6: Scalar = other_641;
    let _e10: Sphere = self_761;
    let _e12: Scalar = other_641;
    return Sphere((_e4.g0_ * vec2<f32>(_e6.g0_)), (_e10.g1_ * vec3<f32>(_e12.g0_)));
}

fn sphere_anti_scalar_regressive_product(self_762: Sphere, other_642: AntiScalar) -> Sphere {
    var self_763: Sphere;
    var other_643: AntiScalar;

    self_763 = self_762;
    other_643 = other_642;
    let _e4: Sphere = self_763;
    let _e6: AntiScalar = other_643;
    let _e10: Sphere = self_763;
    let _e12: AntiScalar = other_643;
    return Sphere((_e4.g0_ * vec2<f32>(_e6.g0_)), (_e10.g1_ * vec3<f32>(_e12.g0_)));
}

fn sphere_radial_point_regressive_product(self_764: Sphere, other_644: RadialPoint) -> Scalar {
    var self_765: Sphere;
    var other_645: RadialPoint;

    self_765 = self_764;
    other_645 = other_644;
    let _e4: Sphere = self_765;
    let _e7: RadialPoint = other_645;
    let _e11: Sphere = self_765;
    let _e14: RadialPoint = other_645;
    let _e19: Sphere = self_765;
    let _e22: RadialPoint = other_645;
    let _e27: Sphere = self_765;
    let _e30: RadialPoint = other_645;
    let _e35: Sphere = self_765;
    let _e38: RadialPoint = other_645;
    return Scalar((((((_e4.g0_.x * _e7.g1_.y) + (_e11.g0_.y * _e14.g1_.x)) + (_e19.g1_.x * _e22.g0_.x)) + (_e27.g1_.y * _e30.g0_.y)) + (_e35.g1_.z * _e38.g0_.z)));
}

fn sphere_radial_point_outer_product(self_766: Sphere, other_646: RadialPoint) -> AntiScalar {
    var self_767: Sphere;
    var other_647: RadialPoint;

    self_767 = self_766;
    other_647 = other_646;
    let _e4: Sphere = self_767;
    let _e7: RadialPoint = other_647;
    let _e11: Sphere = self_767;
    let _e14: RadialPoint = other_647;
    let _e19: Sphere = self_767;
    let _e22: RadialPoint = other_647;
    let _e27: Sphere = self_767;
    let _e30: RadialPoint = other_647;
    let _e35: Sphere = self_767;
    let _e38: RadialPoint = other_647;
    return AntiScalar((((((_e4.g0_.x * _e7.g1_.y) + (_e11.g0_.y * _e14.g1_.x)) + (_e19.g1_.x * _e22.g0_.x)) + (_e27.g1_.y * _e30.g0_.y)) + (_e35.g1_.z * _e38.g0_.z)));
}

fn sphere_flat_point_regressive_product(self_768: Sphere, other_648: FlatPoint) -> RadialPoint {
    var self_769: Sphere;
    var other_649: FlatPoint;

    self_769 = self_768;
    other_649 = other_648;
    let _e4: Sphere = self_769;
    let _e8: FlatPoint = other_649;
    let _e11: FlatPoint = other_649;
    let _e14: FlatPoint = other_649;
    let _e19: Sphere = self_769;
    let _e23: FlatPoint = other_649;
    let _e33: Sphere = self_769;
    let _e37: FlatPoint = other_649;
    let _e48: Sphere = self_769;
    let _e52: FlatPoint = other_649;
    let _e63: Sphere = self_769;
    let _e65: FlatPoint = other_649;
    return RadialPoint((vec3<f32>(_e4.g0_.x) * vec3<f32>(_e8.g0_.x, _e11.g0_.y, _e14.g0_.z)), (((((vec2<f32>(_e19.g1_.x) * vec2<f32>(_e23.g0_.x)) * vec2<f32>(0.0, -(1.0))) + ((vec2<f32>(_e33.g1_.y) * vec2<f32>(_e37.g0_.y)) * vec2<f32>(0.0, -(1.0)))) + ((vec2<f32>(_e48.g1_.z) * vec2<f32>(_e52.g0_.z)) * vec2<f32>(0.0, -(1.0)))) + ((_e63.g0_ * vec2<f32>(_e65.g0_.w)) * vec2<f32>(1.0, -(1.0)))));
}

fn sphere_dipole_regressive_product(self_770: Sphere, other_650: Dipole) -> RadialPoint {
    var self_771: Sphere;
    var other_651: Dipole;

    self_771 = self_770;
    other_651 = other_650;
    let _e4: Sphere = self_771;
    let _e8: Dipole = other_651;
    let _e11: Dipole = other_651;
    let _e14: Dipole = other_651;
    let _e19: Sphere = self_771;
    let _e23: Dipole = other_651;
    let _e27: Sphere = self_771;
    let _e31: Dipole = other_651;
    let _e42: Sphere = self_771;
    let _e46: Dipole = other_651;
    let _e57: Sphere = self_771;
    let _e61: Dipole = other_651;
    let _e72: Sphere = self_771;
    let _e76: Dipole = other_651;
    let _e79: Dipole = other_651;
    let _e89: Sphere = self_771;
    let _e93: Dipole = other_651;
    let _e96: Dipole = other_651;
    let _e107: Sphere = self_771;
    let _e111: Dipole = other_651;
    let _e114: Dipole = other_651;
    let _e125: Sphere = self_771;
    let _e127: Dipole = other_651;
    return RadialPoint((((((vec3<f32>(_e4.g0_.x) * vec3<f32>(_e8.g2_.x, _e11.g2_.y, _e14.g2_.z)) - (vec3<f32>(_e19.g0_.y) * _e23.g0_)) + ((vec3<f32>(_e27.g1_.y) * _e31.g1_.zzx) * vec3<f32>(1.0, 0.0, -(1.0)))) + ((vec3<f32>(_e42.g1_.z) * _e46.g1_.yxy) * vec3<f32>(-(1.0), 1.0, 0.0))) + ((vec3<f32>(_e57.g1_.x) * _e61.g1_.xzy) * vec3<f32>(0.0, -(1.0), 1.0))), (((((vec2<f32>(_e72.g1_.x) * vec2<f32>(_e76.g0_.x, _e79.g2_.x)) * vec2<f32>(1.0, -(1.0))) + ((vec2<f32>(_e89.g1_.y) * vec2<f32>(_e93.g0_.y, _e96.g2_.y)) * vec2<f32>(1.0, -(1.0)))) + ((vec2<f32>(_e107.g1_.z) * vec2<f32>(_e111.g0_.z, _e114.g2_.z)) * vec2<f32>(1.0, -(1.0)))) + ((_e125.g0_ * vec2<f32>(_e127.g2_.w)) * vec2<f32>(1.0, -(1.0)))));
}

fn sphere_line_regressive_product(self_772: Sphere, other_652: Line) -> Dipole {
    var self_773: Sphere;
    var other_653: Line;

    self_773 = self_772;
    other_653 = other_652;
    let _e4: Sphere = self_773;
    let _e8: Line = other_653;
    let _e11: Sphere = self_773;
    let _e15: Line = other_653;
    let _e18: Sphere = self_773;
    let _e22: Line = other_653;
    let _e25: Line = other_653;
    let _e28: Line = other_653;
    let _e31: Line = other_653;
    let _e44: Sphere = self_773;
    let _e48: Line = other_653;
    let _e51: Line = other_653;
    let _e54: Line = other_653;
    let _e57: Line = other_653;
    let _e71: Sphere = self_773;
    let _e75: Line = other_653;
    let _e78: Line = other_653;
    let _e81: Line = other_653;
    let _e84: Line = other_653;
    let _e98: Sphere = self_773;
    let _e101: Sphere = self_773;
    let _e104: Sphere = self_773;
    let _e107: Sphere = self_773;
    let _e111: Line = other_653;
    let _e114: Line = other_653;
    let _e117: Line = other_653;
    let _e120: Line = other_653;
    return Dipole((vec3<f32>(_e4.g0_.x) * _e8.g0_), (vec3<f32>(_e11.g0_.x) * _e15.g1_), (((((vec4<f32>(_e18.g1_.x) * vec4<f32>(_e22.g1_.z, _e25.g1_.z, _e28.g1_.y, _e31.g0_.x)) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0))) + ((vec4<f32>(_e44.g1_.y) * vec4<f32>(_e48.g1_.z, _e51.g1_.z, _e54.g1_.x, _e57.g0_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e71.g1_.z) * vec4<f32>(_e75.g1_.y, _e78.g1_.x, _e81.g1_.y, _e84.g0_.z)) * vec4<f32>(1.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e98.g0_.y, _e101.g0_.y, _e104.g0_.y, _e107.g0_.x) * vec4<f32>(_e111.g0_.x, _e114.g0_.y, _e117.g0_.z, _e120.g0_.x)) * vec4<f32>(1.0, 1.0, 1.0, 0.0))));
}

fn sphere_circle_regressive_product(self_774: Sphere, other_654: Circle) -> Dipole {
    var self_775: Sphere;
    var other_655: Circle;

    self_775 = self_774;
    other_655 = other_654;
    let _e4: Sphere = self_775;
    let _e8: Circle = other_655;
    let _e11: Sphere = self_775;
    let _e15: Circle = other_655;
    let _e18: Circle = other_655;
    let _e21: Circle = other_655;
    let _e33: Sphere = self_775;
    let _e37: Circle = other_655;
    let _e40: Circle = other_655;
    let _e43: Circle = other_655;
    let _e55: Sphere = self_775;
    let _e59: Circle = other_655;
    let _e62: Circle = other_655;
    let _e65: Circle = other_655;
    let _e77: Sphere = self_775;
    let _e81: Circle = other_655;
    let _e84: Sphere = self_775;
    let _e88: Circle = other_655;
    let _e91: Circle = other_655;
    let _e94: Circle = other_655;
    let _e100: Sphere = self_775;
    let _e102: Circle = other_655;
    let _e112: Sphere = self_775;
    let _e116: Circle = other_655;
    let _e119: Circle = other_655;
    let _e122: Circle = other_655;
    let _e125: Circle = other_655;
    let _e138: Sphere = self_775;
    let _e142: Circle = other_655;
    let _e145: Circle = other_655;
    let _e148: Circle = other_655;
    let _e151: Circle = other_655;
    let _e165: Sphere = self_775;
    let _e169: Circle = other_655;
    let _e172: Circle = other_655;
    let _e175: Circle = other_655;
    let _e178: Circle = other_655;
    let _e192: Sphere = self_775;
    let _e195: Sphere = self_775;
    let _e198: Sphere = self_775;
    let _e201: Sphere = self_775;
    let _e205: Circle = other_655;
    let _e208: Circle = other_655;
    let _e211: Circle = other_655;
    let _e214: Circle = other_655;
    return Dipole(((((vec3<f32>(_e4.g0_.x) * _e8.g1_) + ((vec3<f32>(_e11.g1_.y) * vec3<f32>(_e15.g0_.z, _e18.g0_.z, _e21.g0_.x)) * vec3<f32>(1.0, 0.0, -(1.0)))) + ((vec3<f32>(_e33.g1_.z) * vec3<f32>(_e37.g0_.y, _e40.g0_.x, _e43.g0_.y)) * vec3<f32>(-(1.0), 1.0, 0.0))) + ((vec3<f32>(_e55.g1_.x) * vec3<f32>(_e59.g0_.x, _e62.g0_.z, _e65.g0_.y)) * vec3<f32>(0.0, -(1.0), 1.0))), (((vec3<f32>(_e77.g0_.x) * _e81.g2_) + (vec3<f32>(_e84.g0_.y) * vec3<f32>(_e88.g0_.x, _e91.g0_.y, _e94.g0_.z))) + ((_e100.g1_ * vec3<f32>(_e102.g0_.w)) * vec3<f32>(-(1.0)))), (((((vec4<f32>(_e112.g1_.x) * vec4<f32>(_e116.g2_.z, _e119.g2_.z, _e122.g2_.y, _e125.g1_.x)) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0))) + ((vec4<f32>(_e138.g1_.y) * vec4<f32>(_e142.g2_.z, _e145.g2_.z, _e148.g2_.x, _e151.g1_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e165.g1_.z) * vec4<f32>(_e169.g2_.y, _e172.g2_.x, _e175.g2_.y, _e178.g1_.z)) * vec4<f32>(1.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e192.g0_.y, _e195.g0_.y, _e198.g0_.y, _e201.g0_.x) * vec4<f32>(_e205.g1_.x, _e208.g1_.y, _e211.g1_.z, _e214.g1_.x)) * vec4<f32>(1.0, 1.0, 1.0, 0.0))));
}

fn sphere_plane_into(self_776: Sphere) -> Plane {
    var self_777: Sphere;

    self_777 = self_776;
    let _e2: Sphere = self_777;
    let _e5: Sphere = self_777;
    let _e8: Sphere = self_777;
    let _e11: Sphere = self_777;
    return Plane(vec4<f32>(_e2.g1_.x, _e5.g1_.y, _e8.g1_.z, _e11.g0_.y));
}

fn sphere_plane_add(self_778: Sphere, other_656: Plane) -> Sphere {
    var self_779: Sphere;
    var other_657: Plane;

    self_779 = self_778;
    other_657 = other_656;
    let _e4: Sphere = self_779;
    let _e6: Plane = other_657;
    let _e9: Plane = other_657;
    let _e18: Sphere = self_779;
    let _e20: Plane = other_657;
    let _e23: Plane = other_657;
    let _e26: Plane = other_657;
    return Sphere((_e4.g0_ + (vec2<f32>(_e6.g0_.x, _e9.g0_.w) * vec2<f32>(0.0, 1.0))), (_e18.g1_ + vec3<f32>(_e20.g0_.x, _e23.g0_.y, _e26.g0_.z)));
}

fn sphere_plane_sub(self_780: Sphere, other_658: Plane) -> Sphere {
    var self_781: Sphere;
    var other_659: Plane;

    self_781 = self_780;
    other_659 = other_658;
    let _e4: Sphere = self_781;
    let _e6: Plane = other_659;
    let _e9: Plane = other_659;
    let _e18: Sphere = self_781;
    let _e20: Plane = other_659;
    let _e23: Plane = other_659;
    let _e26: Plane = other_659;
    return Sphere((_e4.g0_ - (vec2<f32>(_e6.g0_.x, _e9.g0_.w) * vec2<f32>(0.0, 1.0))), (_e18.g1_ - vec3<f32>(_e20.g0_.x, _e23.g0_.y, _e26.g0_.z)));
}

fn sphere_plane_regressive_product(self_782: Sphere, other_660: Plane) -> Circle {
    var self_783: Sphere;
    var other_661: Plane;

    self_783 = self_782;
    other_661 = other_660;
    let _e4: Sphere = self_783;
    let _e8: Plane = other_661;
    let _e11: Sphere = self_783;
    let _e15: Plane = other_661;
    let _e18: Plane = other_661;
    let _e21: Plane = other_661;
    let _e32: Sphere = self_783;
    let _e36: Plane = other_661;
    let _e39: Plane = other_661;
    let _e42: Plane = other_661;
    let _e54: Sphere = self_783;
    let _e58: Plane = other_661;
    let _e61: Plane = other_661;
    let _e64: Plane = other_661;
    let _e78: Sphere = self_783;
    let _e82: Plane = other_661;
    let _e85: Plane = other_661;
    let _e88: Plane = other_661;
    let _e94: Sphere = self_783;
    let _e96: Plane = other_661;
    return Circle((vec4<f32>(_e4.g0_.x) * _e8.g0_), ((((vec3<f32>(_e11.g1_.y) * vec3<f32>(_e15.g0_.z, _e18.g0_.z, _e21.g0_.x)) * vec3<f32>(-(1.0), 0.0, 1.0)) + ((vec3<f32>(_e32.g1_.z) * vec3<f32>(_e36.g0_.y, _e39.g0_.x, _e42.g0_.y)) * vec3<f32>(1.0, -(1.0), 0.0))) + ((vec3<f32>(_e54.g1_.x) * vec3<f32>(_e58.g0_.x, _e61.g0_.z, _e64.g0_.y)) * vec3<f32>(0.0, 1.0, -(1.0)))), ((vec3<f32>(0.0) - (vec3<f32>(_e78.g0_.y) * vec3<f32>(_e82.g0_.x, _e85.g0_.y, _e88.g0_.z))) + (_e94.g1_ * vec3<f32>(_e96.g0_.w))));
}

fn sphere_sphere_add(self_784: Sphere, other_662: Sphere) -> Sphere {
    var self_785: Sphere;
    var other_663: Sphere;

    self_785 = self_784;
    other_663 = other_662;
    let _e4: Sphere = self_785;
    let _e6: Sphere = other_663;
    let _e9: Sphere = self_785;
    let _e11: Sphere = other_663;
    return Sphere((_e4.g0_ + _e6.g0_), (_e9.g1_ + _e11.g1_));
}

fn sphere_sphere_sub(self_786: Sphere, other_664: Sphere) -> Sphere {
    var self_787: Sphere;
    var other_665: Sphere;

    self_787 = self_786;
    other_665 = other_664;
    let _e4: Sphere = self_787;
    let _e6: Sphere = other_665;
    let _e9: Sphere = self_787;
    let _e11: Sphere = other_665;
    return Sphere((_e4.g0_ - _e6.g0_), (_e9.g1_ - _e11.g1_));
}

fn sphere_sphere_mul(self_788: Sphere, other_666: Sphere) -> Sphere {
    var self_789: Sphere;
    var other_667: Sphere;

    self_789 = self_788;
    other_667 = other_666;
    let _e4: Sphere = self_789;
    let _e6: Sphere = other_667;
    let _e9: Sphere = self_789;
    let _e11: Sphere = other_667;
    return Sphere((_e4.g0_ * _e6.g0_), (_e9.g1_ * _e11.g1_));
}

fn sphere_sphere_div(self_790: Sphere, other_668: Sphere) -> Sphere {
    var self_791: Sphere;
    var other_669: Sphere;

    self_791 = self_790;
    other_669 = other_668;
    let _e4: Sphere = self_791;
    let _e7: Sphere = self_791;
    let _e15: Sphere = other_669;
    let _e18: Sphere = other_669;
    let _e27: Sphere = self_791;
    let _e30: Sphere = self_791;
    let _e33: Sphere = self_791;
    let _e42: Sphere = other_669;
    let _e45: Sphere = other_669;
    let _e48: Sphere = other_669;
    return Sphere((((vec2<f32>(_e4.g0_.x, _e7.g0_.y) * vec2<f32>(1.0, 1.0)) / vec2<f32>(_e15.g0_.x, _e18.g0_.y)) * vec2<f32>(1.0, 1.0)), (((vec3<f32>(_e27.g1_.x, _e30.g1_.y, _e33.g1_.z) * vec3<f32>(1.0, 1.0, 1.0)) / vec3<f32>(_e42.g1_.x, _e45.g1_.y, _e48.g1_.z)) * vec3<f32>(1.0, 1.0, 1.0)));
}

fn sphere_sphere_regressive_product(self_792: Sphere, other_670: Sphere) -> Circle {
    var self_793: Sphere;
    var other_671: Sphere;

    self_793 = self_792;
    other_671 = other_670;
    let _e4: Sphere = self_793;
    let _e8: Sphere = other_671;
    let _e11: Sphere = other_671;
    let _e14: Sphere = other_671;
    let _e17: Sphere = other_671;
    let _e22: Sphere = self_793;
    let _e26: Sphere = other_671;
    let _e39: Sphere = self_793;
    let _e43: Sphere = other_671;
    let _e56: Sphere = self_793;
    let _e60: Sphere = other_671;
    let _e73: Sphere = self_793;
    let _e76: Sphere = self_793;
    let _e79: Sphere = self_793;
    let _e82: Sphere = self_793;
    let _e86: Sphere = other_671;
    let _e99: Sphere = self_793;
    let _e103: Sphere = other_671;
    let _e113: Sphere = self_793;
    let _e117: Sphere = other_671;
    let _e128: Sphere = self_793;
    let _e132: Sphere = other_671;
    let _e145: Sphere = self_793;
    let _e149: Sphere = other_671;
    let _e153: Sphere = self_793;
    let _e155: Sphere = other_671;
    return Circle((((((vec4<f32>(_e4.g0_.x) * vec4<f32>(_e8.g1_.x, _e11.g1_.y, _e14.g1_.z, _e17.g0_.y)) + ((vec4<f32>(_e22.g1_.x) * vec4<f32>(_e26.g0_.x)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((vec4<f32>(_e39.g1_.y) * vec4<f32>(_e43.g0_.x)) * vec4<f32>(0.0, -(1.0), 0.0, 0.0))) + ((vec4<f32>(_e56.g1_.z) * vec4<f32>(_e60.g0_.x)) * vec4<f32>(0.0, 0.0, -(1.0), 0.0))) + ((vec4<f32>(_e73.g0_.x, _e76.g0_.x, _e79.g0_.x, _e82.g0_.y) * vec4<f32>(_e86.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))), ((((vec3<f32>(_e99.g1_.y) * _e103.g1_.zzx) * vec3<f32>(-(1.0), 0.0, 1.0)) + ((vec3<f32>(_e113.g1_.z) * _e117.g1_.yxy) * vec3<f32>(1.0, -(1.0), 0.0))) + ((vec3<f32>(_e128.g1_.x) * _e132.g1_.xzy) * vec3<f32>(0.0, 1.0, -(1.0)))), ((vec3<f32>(0.0) - (vec3<f32>(_e145.g0_.y) * _e149.g1_)) + (_e153.g1_ * vec3<f32>(_e155.g0_.y))));
}

fn sphere_motor_geometric_product(self_794: Sphere, other_672: Motor) -> Rotor {
    var self_795: Sphere;
    var other_673: Motor;

    self_795 = self_794;
    other_673 = other_672;
    let _e4: Sphere = self_795;
    let _e8: Motor = other_673;
    return Rotor((vec4<f32>(_e4.g0_.x) * _e8.g1_));
}

fn sphere_motor_outer_product(self_796: Sphere, other_674: Motor) -> AntiScalar {
    var self_797: Sphere;
    var other_675: Motor;

    self_797 = self_796;
    other_675 = other_674;
    let _e4: Sphere = self_797;
    let _e7: Motor = other_675;
    return AntiScalar((_e4.g0_.x * _e7.g1_.w));
}

fn sphere_multi_vector_add(self_798: Sphere, other_676: MultiVector) -> MultiVector {
    var self_799: Sphere;
    var other_677: MultiVector;

    self_799 = self_798;
    other_677 = other_676;
    let _e4: Sphere = self_799;
    let _e13: MultiVector = other_677;
    let _e16: MultiVector = other_677;
    let _e18: MultiVector = other_677;
    let _e20: MultiVector = other_677;
    let _e22: MultiVector = other_677;
    let _e24: MultiVector = other_677;
    let _e26: MultiVector = other_677;
    let _e28: MultiVector = other_677;
    let _e30: MultiVector = other_677;
    let _e32: Sphere = self_799;
    let _e35: Sphere = self_799;
    let _e38: Sphere = self_799;
    let _e41: Sphere = self_799;
    let _e45: MultiVector = other_677;
    return MultiVector(((vec3<f32>(_e4.g0_.x) * vec3<f32>(0.0, 1.0, 0.0)) + _e13.g0_), _e16.g1_, _e18.g2_, _e20.g3_, _e22.g4_, _e24.g5_, _e26.g6_, _e28.g7_, _e30.g8_, (vec4<f32>(_e32.g1_.x, _e35.g1_.y, _e38.g1_.z, _e41.g0_.y) + _e45.g9_));
}

fn sphere_multi_vector_sub(self_800: Sphere, other_678: MultiVector) -> MultiVector {
    var self_801: Sphere;
    var other_679: MultiVector;

    self_801 = self_800;
    other_679 = other_678;
    let _e4: Sphere = self_801;
    let _e13: MultiVector = other_679;
    let _e18: MultiVector = other_679;
    let _e23: MultiVector = other_679;
    let _e28: MultiVector = other_679;
    let _e33: MultiVector = other_679;
    let _e38: MultiVector = other_679;
    let _e43: MultiVector = other_679;
    let _e48: MultiVector = other_679;
    let _e53: MultiVector = other_679;
    let _e56: Sphere = self_801;
    let _e59: Sphere = self_801;
    let _e62: Sphere = self_801;
    let _e65: Sphere = self_801;
    let _e69: MultiVector = other_679;
    return MultiVector(((vec3<f32>(_e4.g0_.x) * vec3<f32>(0.0, 1.0, 0.0)) - _e13.g0_), (vec3<f32>(0.0) - _e18.g1_), (vec2<f32>(0.0) - _e23.g2_), (vec4<f32>(0.0) - _e28.g3_), (vec3<f32>(0.0) - _e33.g4_), (vec3<f32>(0.0) - _e38.g5_), (vec3<f32>(0.0) - _e43.g6_), (vec3<f32>(0.0) - _e48.g7_), (vec4<f32>(0.0) - _e53.g8_), (vec4<f32>(_e56.g1_.x, _e59.g1_.y, _e62.g1_.z, _e65.g0_.y) - _e69.g9_));
}

fn sphere_scale(self_802: Sphere, other_680: f32) -> Sphere {
    var self_803: Sphere;
    var other_681: f32;

    self_803 = self_802;
    other_681 = other_680;
    let _e4: Sphere = self_803;
    let _e5: f32 = other_681;
    let _e7: Sphere = sphere_scalar_geometric_product(_e4, Scalar(_e5));
    return _e7;
}

fn motor_zero() -> Motor {
    return Motor(vec4<f32>(0.0), vec4<f32>(0.0));
}

fn motor_one() -> Motor {
    return Motor(vec4<f32>(0.0), vec4<f32>(0.0));
}

fn motor_neg(self_804: Motor) -> Motor {
    var self_805: Motor;

    self_805 = self_804;
    let _e2: Motor = self_805;
    let _e8: Motor = self_805;
    return Motor((_e2.g0_ * vec4<f32>(-(1.0))), (_e8.g1_ * vec4<f32>(-(1.0))));
}

fn motor_automorphism(self_806: Motor) -> Motor {
    var self_807: Motor;

    self_807 = self_806;
    let _e2: Motor = self_807;
    let _e8: Motor = self_807;
    return Motor((_e2.g0_ * vec4<f32>(-(1.0))), (_e8.g1_ * vec4<f32>(-(1.0))));
}

fn motor_reversal(self_808: Motor) -> Motor {
    var self_809: Motor;

    self_809 = self_808;
    let _e2: Motor = self_809;
    let _e13: Motor = self_809;
    return Motor((_e2.g0_ * vec4<f32>(-(1.0), -(1.0), -(1.0), 1.0)), (_e13.g1_ * vec4<f32>(-(1.0), -(1.0), -(1.0), 1.0)));
}

fn motor_conjugation(self_810: Motor) -> Motor {
    var self_811: Motor;

    self_811 = self_810;
    let _e2: Motor = self_811;
    let _e11: Motor = self_811;
    return Motor((_e2.g0_ * vec4<f32>(1.0, 1.0, 1.0, -(1.0))), (_e11.g1_ * vec4<f32>(1.0, 1.0, 1.0, -(1.0))));
}

fn motor_scalar_geometric_product(self_812: Motor, other_682: Scalar) -> Motor {
    var self_813: Motor;
    var other_683: Scalar;

    self_813 = self_812;
    other_683 = other_682;
    let _e4: Motor = self_813;
    let _e6: Scalar = other_683;
    let _e10: Motor = self_813;
    let _e12: Scalar = other_683;
    return Motor((_e4.g0_ * vec4<f32>(_e6.g0_)), (_e10.g1_ * vec4<f32>(_e12.g0_)));
}

fn motor_scalar_regressive_product(self_814: Motor, other_684: Scalar) -> Scalar {
    var self_815: Motor;
    var other_685: Scalar;

    self_815 = self_814;
    other_685 = other_684;
    let _e4: Motor = self_815;
    let _e7: Scalar = other_685;
    return Scalar((_e4.g0_.w * _e7.g0_));
}

fn motor_scalar_outer_product(self_816: Motor, other_686: Scalar) -> Motor {
    var self_817: Motor;
    var other_687: Scalar;

    self_817 = self_816;
    other_687 = other_686;
    let _e4: Motor = self_817;
    let _e6: Scalar = other_687;
    let _e10: Motor = self_817;
    let _e12: Scalar = other_687;
    return Motor((_e4.g0_ * vec4<f32>(_e6.g0_)), (_e10.g1_ * vec4<f32>(_e12.g0_)));
}

fn motor_scalar_inner_product(self_818: Motor, other_688: Scalar) -> Motor {
    var self_819: Motor;
    var other_689: Scalar;

    self_819 = self_818;
    other_689 = other_688;
    let _e4: Motor = self_819;
    let _e6: Scalar = other_689;
    let _e10: Motor = self_819;
    let _e12: Scalar = other_689;
    return Motor((_e4.g0_ * vec4<f32>(_e6.g0_)), (_e10.g1_ * vec4<f32>(_e12.g0_)));
}

fn motor_scalar_right_contraction(self_820: Motor, other_690: Scalar) -> Motor {
    var self_821: Motor;
    var other_691: Scalar;

    self_821 = self_820;
    other_691 = other_690;
    let _e4: Motor = self_821;
    let _e6: Scalar = other_691;
    let _e10: Motor = self_821;
    let _e12: Scalar = other_691;
    return Motor((_e4.g0_ * vec4<f32>(_e6.g0_)), (_e10.g1_ * vec4<f32>(_e12.g0_)));
}

fn motor_anti_scalar_into(self_822: Motor) -> AntiScalar {
    var self_823: Motor;

    self_823 = self_822;
    let _e2: Motor = self_823;
    return AntiScalar(_e2.g0_.w);
}

fn motor_anti_scalar_add(self_824: Motor, other_692: AntiScalar) -> Motor {
    var self_825: Motor;
    var other_693: AntiScalar;

    self_825 = self_824;
    other_693 = other_692;
    let _e4: Motor = self_825;
    let _e6: AntiScalar = other_693;
    let _e16: Motor = self_825;
    return Motor((_e4.g0_ + (vec4<f32>(_e6.g0_) * vec4<f32>(0.0, 0.0, 0.0, 1.0))), _e16.g1_);
}

fn motor_anti_scalar_sub(self_826: Motor, other_694: AntiScalar) -> Motor {
    var self_827: Motor;
    var other_695: AntiScalar;

    self_827 = self_826;
    other_695 = other_694;
    let _e4: Motor = self_827;
    let _e6: AntiScalar = other_695;
    let _e16: Motor = self_827;
    return Motor((_e4.g0_ - (vec4<f32>(_e6.g0_) * vec4<f32>(0.0, 0.0, 0.0, 1.0))), _e16.g1_);
}

fn motor_anti_scalar_regressive_product(self_828: Motor, other_696: AntiScalar) -> Motor {
    var self_829: Motor;
    var other_697: AntiScalar;

    self_829 = self_828;
    other_697 = other_696;
    let _e4: Motor = self_829;
    let _e6: AntiScalar = other_697;
    let _e10: Motor = self_829;
    let _e12: AntiScalar = other_697;
    return Motor((_e4.g0_ * vec4<f32>(_e6.g0_)), (_e10.g1_ * vec4<f32>(_e12.g0_)));
}

fn motor_radial_point_geometric_product(self_830: Motor, other_698: RadialPoint) -> Flector {
    var self_831: Motor;
    var other_699: RadialPoint;

    self_831 = self_830;
    other_699 = other_698;
    let _e4: Motor = self_831;
    let _e8: RadialPoint = other_699;
    let _e20: Motor = self_831;
    let _e24: RadialPoint = other_699;
    let _e37: Motor = self_831;
    let _e41: RadialPoint = other_699;
    let _e44: RadialPoint = other_699;
    let _e47: RadialPoint = other_699;
    let _e50: RadialPoint = other_699;
    let _e63: Motor = self_831;
    let _e67: RadialPoint = other_699;
    let _e70: RadialPoint = other_699;
    let _e73: RadialPoint = other_699;
    let _e76: RadialPoint = other_699;
    let _e89: Motor = self_831;
    let _e93: RadialPoint = other_699;
    let _e96: RadialPoint = other_699;
    let _e99: RadialPoint = other_699;
    let _e102: RadialPoint = other_699;
    let _e108: Motor = self_831;
    let _e111: Motor = self_831;
    let _e114: Motor = self_831;
    let _e117: Motor = self_831;
    let _e121: RadialPoint = other_699;
    let _e124: RadialPoint = other_699;
    let _e127: RadialPoint = other_699;
    let _e130: RadialPoint = other_699;
    let _e144: Motor = self_831;
    let _e148: RadialPoint = other_699;
    let _e151: RadialPoint = other_699;
    let _e154: RadialPoint = other_699;
    let _e157: RadialPoint = other_699;
    let _e169: Motor = self_831;
    let _e173: RadialPoint = other_699;
    let _e176: RadialPoint = other_699;
    let _e179: RadialPoint = other_699;
    let _e182: RadialPoint = other_699;
    let _e195: Motor = self_831;
    let _e199: RadialPoint = other_699;
    let _e202: RadialPoint = other_699;
    let _e205: RadialPoint = other_699;
    let _e208: RadialPoint = other_699;
    let _e220: Motor = self_831;
    let _e224: RadialPoint = other_699;
    let _e227: RadialPoint = other_699;
    let _e230: RadialPoint = other_699;
    let _e233: RadialPoint = other_699;
    let _e246: Motor = self_831;
    let _e250: RadialPoint = other_699;
    let _e253: RadialPoint = other_699;
    let _e256: RadialPoint = other_699;
    let _e259: RadialPoint = other_699;
    let _e272: Motor = self_831;
    let _e276: RadialPoint = other_699;
    let _e279: RadialPoint = other_699;
    let _e282: RadialPoint = other_699;
    let _e285: RadialPoint = other_699;
    let _e298: Motor = self_831;
    let _e302: RadialPoint = other_699;
    let _e305: RadialPoint = other_699;
    let _e308: RadialPoint = other_699;
    let _e311: RadialPoint = other_699;
    return Flector((((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0))) + ((vec4<f32>(_e20.g0_.z) * vec4<f32>(_e24.g0_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e37.g1_.y) * vec4<f32>(_e41.g0_.z, _e44.g0_.z, _e47.g0_.x, _e50.g0_.z)) * vec4<f32>(1.0, 0.0, -(1.0), 0.0))) + ((vec4<f32>(_e63.g1_.z) * vec4<f32>(_e67.g0_.y, _e70.g0_.x, _e73.g0_.y, _e76.g0_.y)) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))) - (vec4<f32>(_e89.g1_.w) * vec4<f32>(_e93.g0_.x, _e96.g0_.y, _e99.g0_.z, _e102.g1_.x))) + ((vec4<f32>(_e108.g0_.x, _e111.g1_.x, _e114.g1_.x, _e117.g0_.x) * vec4<f32>(_e121.g0_.x, _e124.g0_.z, _e127.g0_.y, _e130.g0_.x)) * vec4<f32>(0.0, -(1.0), 1.0, -(1.0)))), ((((((((vec4<f32>(_e144.g0_.y) * vec4<f32>(_e148.g0_.z, _e151.g0_.z, _e154.g0_.x, _e157.g0_.z)) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0)) + ((vec4<f32>(_e169.g0_.z) * vec4<f32>(_e173.g0_.y, _e176.g0_.x, _e179.g0_.y, _e182.g0_.y)) * vec4<f32>(1.0, -(1.0), 0.0, 0.0))) + ((vec4<f32>(_e195.g0_.w) * vec4<f32>(_e199.g0_.x, _e202.g0_.y, _e205.g0_.z, _e208.g0_.x)) * vec4<f32>(1.0, 1.0, 1.0, 0.0))) + ((vec4<f32>(_e220.g1_.x) * vec4<f32>(_e224.g1_.x, _e227.g1_.x, _e230.g1_.x, _e233.g0_.x)) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))) + ((vec4<f32>(_e246.g1_.y) * vec4<f32>(_e250.g1_.x, _e253.g1_.x, _e256.g1_.x, _e259.g0_.y)) * vec4<f32>(0.0, -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e272.g1_.z) * vec4<f32>(_e276.g1_.x, _e279.g1_.x, _e282.g1_.x, _e285.g0_.z)) * vec4<f32>(0.0, 0.0, -(1.0), 1.0))) + ((vec4<f32>(_e298.g0_.x) * vec4<f32>(_e302.g0_.x, _e305.g0_.z, _e308.g0_.y, _e311.g0_.x)) * vec4<f32>(0.0, 1.0, -(1.0), 0.0))));
}

fn motor_radial_point_regressive_product(self_832: Motor, other_700: RadialPoint) -> RadialPoint {
    var self_833: Motor;
    var other_701: RadialPoint;

    self_833 = self_832;
    other_701 = other_700;
    let _e4: Motor = self_833;
    let _e8: RadialPoint = other_701;
    let _e11: Motor = self_833;
    let _e15: RadialPoint = other_701;
    return RadialPoint((vec3<f32>(_e4.g0_.w) * _e8.g0_), (vec2<f32>(_e11.g0_.w) * _e15.g1_));
}

fn motor_radial_point_outer_product(self_834: Motor, other_702: RadialPoint) -> Flector {
    var self_835: Motor;
    var other_703: RadialPoint;

    self_835 = self_834;
    other_703 = other_702;
    let _e6: Motor = self_835;
    let _e10: RadialPoint = other_703;
    let _e13: RadialPoint = other_703;
    let _e16: RadialPoint = other_703;
    let _e19: RadialPoint = other_703;
    let _e25: Motor = self_835;
    let _e29: RadialPoint = other_703;
    let _e32: RadialPoint = other_703;
    let _e35: RadialPoint = other_703;
    let _e38: RadialPoint = other_703;
    let _e50: Motor = self_835;
    let _e54: RadialPoint = other_703;
    let _e57: RadialPoint = other_703;
    let _e60: RadialPoint = other_703;
    let _e63: RadialPoint = other_703;
    let _e76: Motor = self_835;
    let _e80: RadialPoint = other_703;
    let _e83: RadialPoint = other_703;
    let _e86: RadialPoint = other_703;
    let _e89: RadialPoint = other_703;
    let _e102: Motor = self_835;
    let _e106: RadialPoint = other_703;
    let _e109: RadialPoint = other_703;
    let _e112: RadialPoint = other_703;
    let _e115: RadialPoint = other_703;
    let _e128: Motor = self_835;
    let _e132: RadialPoint = other_703;
    let _e135: RadialPoint = other_703;
    let _e138: RadialPoint = other_703;
    let _e141: RadialPoint = other_703;
    let _e154: Motor = self_835;
    let _e158: RadialPoint = other_703;
    let _e161: RadialPoint = other_703;
    let _e164: RadialPoint = other_703;
    let _e167: RadialPoint = other_703;
    return Flector((vec4<f32>(0.0) - (vec4<f32>(_e6.g1_.w) * vec4<f32>(_e10.g0_.x, _e13.g0_.y, _e16.g0_.z, _e19.g1_.x))), (((((((vec4<f32>(_e25.g0_.y) * vec4<f32>(_e29.g0_.z, _e32.g0_.z, _e35.g0_.x, _e38.g0_.z)) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0)) + ((vec4<f32>(_e50.g0_.z) * vec4<f32>(_e54.g0_.y, _e57.g0_.x, _e60.g0_.y, _e63.g0_.y)) * vec4<f32>(1.0, -(1.0), 0.0, 0.0))) + ((vec4<f32>(_e76.g1_.x) * vec4<f32>(_e80.g1_.x, _e83.g1_.x, _e86.g1_.x, _e89.g0_.x)) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))) + ((vec4<f32>(_e102.g1_.y) * vec4<f32>(_e106.g1_.x, _e109.g1_.x, _e112.g1_.x, _e115.g0_.y)) * vec4<f32>(0.0, -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e128.g1_.z) * vec4<f32>(_e132.g1_.x, _e135.g1_.x, _e138.g1_.x, _e141.g0_.z)) * vec4<f32>(0.0, 0.0, -(1.0), 1.0))) + ((vec4<f32>(_e154.g0_.x) * vec4<f32>(_e158.g0_.x, _e161.g0_.z, _e164.g0_.y, _e167.g0_.x)) * vec4<f32>(0.0, 1.0, -(1.0), 0.0))));
}

fn motor_flat_point_regressive_product(self_836: Motor, other_704: FlatPoint) -> FlatPoint {
    var self_837: Motor;
    var other_705: FlatPoint;

    self_837 = self_836;
    other_705 = other_704;
    let _e4: Motor = self_837;
    let _e8: FlatPoint = other_705;
    return FlatPoint((vec4<f32>(_e4.g0_.w) * _e8.g0_));
}

fn motor_dipole_geometric_product(self_838: Motor, other_706: Dipole) -> Motor {
    var self_839: Motor;
    var other_707: Dipole;

    self_839 = self_838;
    other_707 = other_706;
    let _e4: Motor = self_839;
    let _e8: Dipole = other_707;
    let _e11: Dipole = other_707;
    let _e14: Dipole = other_707;
    let _e17: Dipole = other_707;
    let _e30: Motor = self_839;
    let _e34: Dipole = other_707;
    let _e37: Dipole = other_707;
    let _e40: Dipole = other_707;
    let _e43: Dipole = other_707;
    let _e57: Motor = self_839;
    let _e61: Dipole = other_707;
    let _e64: Dipole = other_707;
    let _e67: Dipole = other_707;
    let _e70: Dipole = other_707;
    let _e82: Motor = self_839;
    let _e86: Dipole = other_707;
    let _e89: Dipole = other_707;
    let _e92: Dipole = other_707;
    let _e95: Dipole = other_707;
    let _e109: Motor = self_839;
    let _e113: Dipole = other_707;
    let _e116: Dipole = other_707;
    let _e119: Dipole = other_707;
    let _e122: Dipole = other_707;
    let _e136: Motor = self_839;
    let _e140: Dipole = other_707;
    let _e143: Dipole = other_707;
    let _e146: Dipole = other_707;
    let _e149: Dipole = other_707;
    let _e163: Motor = self_839;
    let _e167: Dipole = other_707;
    let _e170: Dipole = other_707;
    let _e173: Dipole = other_707;
    let _e176: Dipole = other_707;
    let _e188: Motor = self_839;
    let _e192: Dipole = other_707;
    let _e195: Dipole = other_707;
    let _e198: Dipole = other_707;
    let _e201: Dipole = other_707;
    let _e215: Motor = self_839;
    let _e219: Dipole = other_707;
    let _e222: Dipole = other_707;
    let _e225: Dipole = other_707;
    let _e228: Dipole = other_707;
    let _e241: Motor = self_839;
    let _e245: Dipole = other_707;
    let _e248: Dipole = other_707;
    let _e251: Dipole = other_707;
    let _e254: Dipole = other_707;
    let _e268: Motor = self_839;
    let _e272: Dipole = other_707;
    let _e275: Dipole = other_707;
    let _e278: Dipole = other_707;
    let _e281: Dipole = other_707;
    let _e293: Motor = self_839;
    let _e297: Dipole = other_707;
    let _e300: Dipole = other_707;
    let _e303: Dipole = other_707;
    let _e306: Dipole = other_707;
    return Motor((((((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g1_.z, _e11.g1_.z, _e14.g1_.x, _e17.g1_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e30.g0_.z) * vec4<f32>(_e34.g1_.y, _e37.g1_.x, _e40.g1_.y, _e43.g1_.z)) * vec4<f32>(1.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e57.g0_.w) * vec4<f32>(_e61.g1_.x, _e64.g1_.y, _e67.g1_.z, _e70.g1_.x)) * vec4<f32>(1.0, 1.0, 1.0, 0.0))) + ((vec4<f32>(_e82.g1_.x) * vec4<f32>(_e86.g0_.z, _e89.g0_.z, _e92.g0_.y, _e95.g0_.x)) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e109.g1_.y) * vec4<f32>(_e113.g0_.z, _e116.g0_.z, _e119.g0_.x, _e122.g0_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e136.g1_.z) * vec4<f32>(_e140.g0_.y, _e143.g0_.x, _e146.g0_.y, _e149.g0_.z)) * vec4<f32>(1.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e163.g1_.w) * vec4<f32>(_e167.g0_.x, _e170.g0_.y, _e173.g0_.z, _e176.g0_.x)) * vec4<f32>(1.0, 1.0, 1.0, 0.0))) + ((vec4<f32>(_e188.g0_.x) * vec4<f32>(_e192.g1_.x, _e195.g1_.z, _e198.g1_.y, _e201.g1_.x)) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))), (((((vec4<f32>(_e215.g1_.y) * vec4<f32>(_e219.g1_.z, _e222.g1_.z, _e225.g1_.x, _e228.g1_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e241.g1_.z) * vec4<f32>(_e245.g1_.y, _e248.g1_.x, _e251.g1_.y, _e254.g1_.z)) * vec4<f32>(1.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e268.g1_.w) * vec4<f32>(_e272.g1_.x, _e275.g1_.y, _e278.g1_.z, _e281.g1_.x)) * vec4<f32>(1.0, 1.0, 1.0, 0.0))) + ((vec4<f32>(_e293.g1_.x) * vec4<f32>(_e297.g1_.x, _e300.g1_.z, _e303.g1_.y, _e306.g1_.x)) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))));
}

fn motor_line_into(self_840: Motor) -> Line {
    var self_841: Motor;

    self_841 = self_840;
    let _e2: Motor = self_841;
    let _e5: Motor = self_841;
    let _e8: Motor = self_841;
    let _e12: Motor = self_841;
    let _e15: Motor = self_841;
    let _e18: Motor = self_841;
    return Line(vec3<f32>(_e2.g0_.x, _e5.g0_.y, _e8.g0_.z), vec3<f32>(_e12.g1_.x, _e15.g1_.y, _e18.g1_.z));
}

fn motor_line_add(self_842: Motor, other_708: Line) -> Motor {
    var self_843: Motor;
    var other_709: Line;

    self_843 = self_842;
    other_709 = other_708;
    let _e4: Motor = self_843;
    let _e6: Line = other_709;
    let _e9: Line = other_709;
    let _e12: Line = other_709;
    let _e15: Line = other_709;
    let _e26: Motor = self_843;
    let _e28: Line = other_709;
    let _e31: Line = other_709;
    let _e34: Line = other_709;
    let _e37: Line = other_709;
    return Motor((_e4.g0_ + (vec4<f32>(_e6.g0_.x, _e9.g0_.y, _e12.g0_.z, _e15.g0_.x) * vec4<f32>(1.0, 1.0, 1.0, 0.0))), (_e26.g1_ + (vec4<f32>(_e28.g1_.x, _e31.g1_.y, _e34.g1_.z, _e37.g0_.x) * vec4<f32>(1.0, 1.0, 1.0, 0.0))));
}

fn motor_line_sub(self_844: Motor, other_710: Line) -> Motor {
    var self_845: Motor;
    var other_711: Line;

    self_845 = self_844;
    other_711 = other_710;
    let _e4: Motor = self_845;
    let _e6: Line = other_711;
    let _e9: Line = other_711;
    let _e12: Line = other_711;
    let _e15: Line = other_711;
    let _e26: Motor = self_845;
    let _e28: Line = other_711;
    let _e31: Line = other_711;
    let _e34: Line = other_711;
    let _e37: Line = other_711;
    return Motor((_e4.g0_ - (vec4<f32>(_e6.g0_.x, _e9.g0_.y, _e12.g0_.z, _e15.g0_.x) * vec4<f32>(1.0, 1.0, 1.0, 0.0))), (_e26.g1_ - (vec4<f32>(_e28.g1_.x, _e31.g1_.y, _e34.g1_.z, _e37.g0_.x) * vec4<f32>(1.0, 1.0, 1.0, 0.0))));
}

fn motor_circle_geometric_product(self_846: Motor, other_712: Circle) -> Flector {
    var self_847: Motor;
    var other_713: Circle;

    self_847 = self_846;
    other_713 = other_712;
    let _e4: Motor = self_847;
    let _e8: Circle = other_713;
    let _e19: Motor = self_847;
    let _e23: Circle = other_713;
    let _e35: Motor = self_847;
    let _e39: Circle = other_713;
    let _e51: Motor = self_847;
    let _e54: Circle = other_713;
    let _e65: Motor = self_847;
    let _e69: Circle = other_713;
    let _e80: Motor = self_847;
    let _e84: Circle = other_713;
    let _e96: Motor = self_847;
    let _e100: Circle = other_713;
    let _e112: Motor = self_847;
    let _e116: Circle = other_713;
    let _e120: Motor = self_847;
    let _e123: Circle = other_713;
    return Flector((((((vec4<f32>(_e4.g1_.x) * _e8.g0_.wwwx) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0)) + ((vec4<f32>(_e19.g1_.y) * _e23.g0_.wwwy) * vec4<f32>(0.0, -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e35.g1_.z) * _e39.g0_.wwwz) * vec4<f32>(0.0, 0.0, -(1.0), 1.0))) + ((_e51.g0_.xxxw * _e54.g0_.xxxw) * vec4<f32>(0.0, 0.0, 0.0, 1.0))), ((((((vec4<f32>(_e65.g1_.x) * _e69.g0_.zzyz) * vec4<f32>(0.0, -(1.0), 1.0, 0.0)) + ((vec4<f32>(_e80.g1_.y) * _e84.g0_.zzxz) * vec4<f32>(1.0, 0.0, -(1.0), 0.0))) + ((vec4<f32>(_e96.g1_.z) * _e100.g0_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))) - (vec4<f32>(_e112.g1_.w) * _e116.g0_)) + ((_e120.g0_.xyzx * _e123.g0_.wwwx) * vec4<f32>(1.0, 1.0, 1.0, 0.0))));
}

fn motor_circle_outer_product(self_848: Motor, other_714: Circle) -> Plane {
    var self_849: Motor;
    var other_715: Circle;

    self_849 = self_848;
    other_715 = other_714;
    let _e6: Motor = self_849;
    let _e10: Circle = other_715;
    return Plane((vec4<f32>(0.0) - (vec4<f32>(_e6.g1_.w) * _e10.g0_)));
}

fn motor_plane_regressive_product(self_850: Motor, other_716: Plane) -> Flector {
    var self_851: Motor;
    var other_717: Plane;

    self_851 = self_850;
    other_717 = other_716;
    let _e4: Motor = self_851;
    let _e8: Plane = other_717;
    let _e19: Motor = self_851;
    let _e23: Plane = other_717;
    let _e35: Motor = self_851;
    let _e39: Plane = other_717;
    let _e51: Motor = self_851;
    let _e55: Plane = other_717;
    let _e67: Motor = self_851;
    let _e70: Motor = self_851;
    let _e73: Motor = self_851;
    let _e76: Motor = self_851;
    let _e80: Plane = other_717;
    let _e93: Motor = self_851;
    let _e97: Plane = other_717;
    return Flector(((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e35.g1_.y) * _e39.g0_.zzxz) * vec4<f32>(1.0, 0.0, -(1.0), 0.0))) + ((vec4<f32>(_e51.g1_.z) * _e55.g0_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))) + ((vec4<f32>(_e67.g0_.x, _e70.g1_.x, _e73.g1_.x, _e76.g0_.x) * _e80.g0_.wzyx) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))), (vec4<f32>(_e93.g0_.w) * _e97.g0_));
}

fn motor_sphere_geometric_product(self_852: Motor, other_718: Sphere) -> Rotor {
    var self_853: Motor;
    var other_719: Sphere;

    self_853 = self_852;
    other_719 = other_718;
    let _e4: Motor = self_853;
    let _e6: Sphere = other_719;
    return Rotor((_e4.g1_ * vec4<f32>(_e6.g0_.x)));
}

fn motor_sphere_outer_product(self_854: Motor, other_720: Sphere) -> AntiScalar {
    var self_855: Motor;
    var other_721: Sphere;

    self_855 = self_854;
    other_721 = other_720;
    let _e4: Motor = self_855;
    let _e7: Sphere = other_721;
    return AntiScalar((_e4.g1_.w * _e7.g0_.x));
}

fn motor_motor_add(self_856: Motor, other_722: Motor) -> Motor {
    var self_857: Motor;
    var other_723: Motor;

    self_857 = self_856;
    other_723 = other_722;
    let _e4: Motor = self_857;
    let _e6: Motor = other_723;
    let _e9: Motor = self_857;
    let _e11: Motor = other_723;
    return Motor((_e4.g0_ + _e6.g0_), (_e9.g1_ + _e11.g1_));
}

fn motor_motor_sub(self_858: Motor, other_724: Motor) -> Motor {
    var self_859: Motor;
    var other_725: Motor;

    self_859 = self_858;
    other_725 = other_724;
    let _e4: Motor = self_859;
    let _e6: Motor = other_725;
    let _e9: Motor = self_859;
    let _e11: Motor = other_725;
    return Motor((_e4.g0_ - _e6.g0_), (_e9.g1_ - _e11.g1_));
}

fn motor_motor_mul(self_860: Motor, other_726: Motor) -> Motor {
    var self_861: Motor;
    var other_727: Motor;

    self_861 = self_860;
    other_727 = other_726;
    let _e4: Motor = self_861;
    let _e6: Motor = other_727;
    let _e9: Motor = self_861;
    let _e11: Motor = other_727;
    return Motor((_e4.g0_ * _e6.g0_), (_e9.g1_ * _e11.g1_));
}

fn motor_motor_div(self_862: Motor, other_728: Motor) -> Motor {
    var self_863: Motor;
    var other_729: Motor;

    self_863 = self_862;
    other_729 = other_728;
    let _e4: Motor = self_863;
    let _e7: Motor = self_863;
    let _e10: Motor = self_863;
    let _e13: Motor = self_863;
    let _e23: Motor = other_729;
    let _e26: Motor = other_729;
    let _e29: Motor = other_729;
    let _e32: Motor = other_729;
    let _e43: Motor = self_863;
    let _e46: Motor = self_863;
    let _e49: Motor = self_863;
    let _e52: Motor = self_863;
    let _e62: Motor = other_729;
    let _e65: Motor = other_729;
    let _e68: Motor = other_729;
    let _e71: Motor = other_729;
    return Motor((((vec4<f32>(_e4.g0_.x, _e7.g0_.y, _e10.g0_.z, _e13.g0_.w) * vec4<f32>(1.0, 1.0, 1.0, 1.0)) / vec4<f32>(_e23.g0_.x, _e26.g0_.y, _e29.g0_.z, _e32.g0_.w)) * vec4<f32>(1.0, 1.0, 1.0, 1.0)), (((vec4<f32>(_e43.g1_.x, _e46.g1_.y, _e49.g1_.z, _e52.g1_.w) * vec4<f32>(1.0, 1.0, 1.0, 1.0)) / vec4<f32>(_e62.g1_.x, _e65.g1_.y, _e68.g1_.z, _e71.g1_.w)) * vec4<f32>(1.0, 1.0, 1.0, 1.0)));
}

fn motor_motor_regressive_product(self_864: Motor, other_730: Motor) -> Motor {
    var self_865: Motor;
    var other_731: Motor;

    self_865 = self_864;
    other_731 = other_730;
    let _e4: Motor = self_865;
    let _e8: Motor = other_731;
    let _e11: Motor = self_865;
    let _e14: Motor = other_731;
    let _e25: Motor = self_865;
    let _e29: Motor = other_731;
    let _e41: Motor = self_865;
    let _e45: Motor = other_731;
    let _e58: Motor = self_865;
    let _e62: Motor = other_731;
    let _e66: Motor = self_865;
    let _e70: Motor = other_731;
    let _e82: Motor = self_865;
    let _e86: Motor = other_731;
    let _e98: Motor = self_865;
    let _e102: Motor = other_731;
    let _e114: Motor = self_865;
    let _e118: Motor = other_731;
    let _e130: Motor = self_865;
    let _e134: Motor = other_731;
    return Motor(((vec4<f32>(_e4.g0_.w) * _e8.g0_) + ((_e11.g0_.xyzx * _e14.g0_.wwwx) * vec4<f32>(1.0, 1.0, 1.0, 0.0))), (((((((((vec4<f32>(_e25.g0_.y) * vec4<f32>(_e29.g1_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0))) + ((vec4<f32>(_e41.g0_.z) * vec4<f32>(_e45.g1_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + (vec4<f32>(_e58.g0_.w) * _e62.g1_)) + ((vec4<f32>(_e66.g1_.x) * _e70.g0_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e82.g1_.y) * _e86.g0_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0)))) + ((vec4<f32>(_e98.g1_.z) * _e102.g0_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e114.g1_.w) * vec4<f32>(_e118.g0_.w)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e130.g0_.x) * vec4<f32>(_e134.g1_.x)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))));
}

fn motor_rotor_into(self_866: Motor) -> Rotor {
    var self_867: Motor;

    self_867 = self_866;
    let _e2: Motor = self_867;
    return Rotor(_e2.g0_);
}

fn motor_rotor_add(self_868: Motor, other_732: Rotor) -> Motor {
    var self_869: Motor;
    var other_733: Rotor;

    self_869 = self_868;
    other_733 = other_732;
    let _e4: Motor = self_869;
    let _e6: Rotor = other_733;
    let _e9: Motor = self_869;
    return Motor((_e4.g0_ + _e6.g0_), _e9.g1_);
}

fn motor_rotor_sub(self_870: Motor, other_734: Rotor) -> Motor {
    var self_871: Motor;
    var other_735: Rotor;

    self_871 = self_870;
    other_735 = other_734;
    let _e4: Motor = self_871;
    let _e6: Rotor = other_735;
    let _e9: Motor = self_871;
    return Motor((_e4.g0_ - _e6.g0_), _e9.g1_);
}

fn motor_rotor_regressive_product(self_872: Motor, other_736: Rotor) -> Motor {
    var self_873: Motor;
    var other_737: Rotor;

    self_873 = self_872;
    other_737 = other_736;
    let _e4: Motor = self_873;
    let _e8: Rotor = other_737;
    let _e11: Motor = self_873;
    let _e14: Rotor = other_737;
    let _e25: Motor = self_873;
    let _e29: Rotor = other_737;
    let _e40: Motor = self_873;
    let _e44: Rotor = other_737;
    let _e56: Motor = self_873;
    let _e60: Rotor = other_737;
    let _e72: Motor = self_873;
    let _e76: Rotor = other_737;
    return Motor(((vec4<f32>(_e4.g0_.w) * _e8.g0_) + ((_e11.g0_.xyzx * _e14.g0_.wwwx) * vec4<f32>(1.0, 1.0, 1.0, 0.0))), (((((vec4<f32>(_e25.g1_.y) * _e29.g0_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e40.g1_.z) * _e44.g0_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e56.g1_.w) * vec4<f32>(_e60.g0_.w)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e72.g1_.x) * _e76.g0_.wxxx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))));
}

fn motor_translator_into(self_874: Motor) -> Translator {
    var self_875: Motor;

    self_875 = self_874;
    let _e2: Motor = self_875;
    let _e5: Motor = self_875;
    let _e8: Motor = self_875;
    let _e11: Motor = self_875;
    return Translator(vec4<f32>(_e2.g1_.x, _e5.g1_.y, _e8.g1_.z, _e11.g0_.w));
}

fn motor_translator_add(self_876: Motor, other_738: Translator) -> Motor {
    var self_877: Motor;
    var other_739: Translator;

    self_877 = self_876;
    other_739 = other_738;
    let _e4: Motor = self_877;
    let _e6: Translator = other_739;
    let _e16: Motor = self_877;
    let _e18: Translator = other_739;
    return Motor((_e4.g0_ + (_e6.g0_.xxxw * vec4<f32>(0.0, 0.0, 0.0, 1.0))), (_e16.g1_ + (_e18.g0_.xyzx * vec4<f32>(1.0, 1.0, 1.0, 0.0))));
}

fn motor_translator_sub(self_878: Motor, other_740: Translator) -> Motor {
    var self_879: Motor;
    var other_741: Translator;

    self_879 = self_878;
    other_741 = other_740;
    let _e4: Motor = self_879;
    let _e6: Translator = other_741;
    let _e16: Motor = self_879;
    let _e18: Translator = other_741;
    return Motor((_e4.g0_ - (_e6.g0_.xxxw * vec4<f32>(0.0, 0.0, 0.0, 1.0))), (_e16.g1_ - (_e18.g0_.xyzx * vec4<f32>(1.0, 1.0, 1.0, 0.0))));
}

fn motor_translator_regressive_product(self_880: Motor, other_742: Translator) -> Motor {
    var self_881: Motor;
    var other_743: Translator;

    self_881 = self_880;
    other_743 = other_742;
    let _e4: Motor = self_881;
    let _e6: Translator = other_743;
    let _e11: Motor = self_881;
    let _e15: Translator = other_743;
    let _e27: Motor = self_881;
    let _e31: Translator = other_743;
    let _e44: Motor = self_881;
    let _e48: Translator = other_743;
    let _e60: Motor = self_881;
    let _e64: Translator = other_743;
    let _e76: Motor = self_881;
    let _e80: Translator = other_743;
    let _e92: Motor = self_881;
    let _e96: Translator = other_743;
    let _e108: Motor = self_881;
    let _e111: Translator = other_743;
    return Motor((_e4.g0_ * vec4<f32>(_e6.g0_.w)), ((((((((vec4<f32>(_e11.g0_.y) * vec4<f32>(_e15.g0_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0))) + ((vec4<f32>(_e27.g0_.z) * vec4<f32>(_e31.g0_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e44.g1_.x) * vec4<f32>(_e48.g0_.w)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e60.g1_.y) * vec4<f32>(_e64.g0_.w)) * vec4<f32>(0.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e76.g1_.z) * vec4<f32>(_e80.g0_.w)) * vec4<f32>(0.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e92.g1_.w) * vec4<f32>(_e96.g0_.w)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((_e108.g0_.wwwx * _e111.g0_.xyzx) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))));
}

fn motor_flector_regressive_product(self_882: Motor, other_744: Flector) -> Flector {
    var self_883: Motor;
    var other_745: Flector;

    self_883 = self_882;
    other_745 = other_744;
    let _e4: Motor = self_883;
    let _e8: Flector = other_745;
    let _e19: Motor = self_883;
    let _e23: Flector = other_745;
    let _e35: Motor = self_883;
    let _e39: Flector = other_745;
    let _e43: Motor = self_883;
    let _e47: Flector = other_745;
    let _e59: Motor = self_883;
    let _e63: Flector = other_745;
    let _e75: Motor = self_883;
    let _e78: Motor = self_883;
    let _e81: Motor = self_883;
    let _e84: Motor = self_883;
    let _e88: Flector = other_745;
    let _e101: Motor = self_883;
    let _e105: Flector = other_745;
    return Flector((((((((vec4<f32>(_e4.g0_.y) * _e8.g1_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e19.g0_.z) * _e23.g1_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + (vec4<f32>(_e35.g0_.w) * _e39.g0_)) + ((vec4<f32>(_e43.g1_.y) * _e47.g1_.zzxz) * vec4<f32>(1.0, 0.0, -(1.0), 0.0))) + ((vec4<f32>(_e59.g1_.z) * _e63.g1_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))) + ((vec4<f32>(_e75.g0_.x, _e78.g1_.x, _e81.g1_.x, _e84.g0_.x) * _e88.g1_.wzyx) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))), (vec4<f32>(_e101.g0_.w) * _e105.g1_));
}

fn motor_multi_vector_add(self_884: Motor, other_746: MultiVector) -> MultiVector {
    var self_885: Motor;
    var other_747: MultiVector;

    self_885 = self_884;
    other_747 = other_746;
    let _e4: Motor = self_885;
    let _e7: Motor = self_885;
    let _e10: Motor = self_885;
    let _e19: MultiVector = other_747;
    let _e22: MultiVector = other_747;
    let _e24: Motor = self_885;
    let _e27: Motor = self_885;
    let _e35: MultiVector = other_747;
    let _e38: MultiVector = other_747;
    let _e40: MultiVector = other_747;
    let _e42: MultiVector = other_747;
    let _e44: Motor = self_885;
    let _e47: Motor = self_885;
    let _e50: Motor = self_885;
    let _e54: MultiVector = other_747;
    let _e57: Motor = self_885;
    let _e60: Motor = self_885;
    let _e63: Motor = self_885;
    let _e67: MultiVector = other_747;
    let _e70: MultiVector = other_747;
    let _e72: MultiVector = other_747;
    return MultiVector(((vec3<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.w) * vec3<f32>(0.0, 0.0, 1.0)) + _e19.g0_), _e22.g1_, ((vec2<f32>(_e24.g0_.x, _e27.g1_.w) * vec2<f32>(0.0, 1.0)) + _e35.g2_), _e38.g3_, _e40.g4_, _e42.g5_, (vec3<f32>(_e44.g0_.x, _e47.g0_.y, _e50.g0_.z) + _e54.g6_), (vec3<f32>(_e57.g1_.x, _e60.g1_.y, _e63.g1_.z) + _e67.g7_), _e70.g8_, _e72.g9_);
}

fn motor_multi_vector_sub(self_886: Motor, other_748: MultiVector) -> MultiVector {
    var self_887: Motor;
    var other_749: MultiVector;

    self_887 = self_886;
    other_749 = other_748;
    let _e4: Motor = self_887;
    let _e7: Motor = self_887;
    let _e10: Motor = self_887;
    let _e19: MultiVector = other_749;
    let _e24: MultiVector = other_749;
    let _e27: Motor = self_887;
    let _e30: Motor = self_887;
    let _e38: MultiVector = other_749;
    let _e43: MultiVector = other_749;
    let _e48: MultiVector = other_749;
    let _e53: MultiVector = other_749;
    let _e56: Motor = self_887;
    let _e59: Motor = self_887;
    let _e62: Motor = self_887;
    let _e66: MultiVector = other_749;
    let _e69: Motor = self_887;
    let _e72: Motor = self_887;
    let _e75: Motor = self_887;
    let _e79: MultiVector = other_749;
    let _e84: MultiVector = other_749;
    let _e89: MultiVector = other_749;
    return MultiVector(((vec3<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.w) * vec3<f32>(0.0, 0.0, 1.0)) - _e19.g0_), (vec3<f32>(0.0) - _e24.g1_), ((vec2<f32>(_e27.g0_.x, _e30.g1_.w) * vec2<f32>(0.0, 1.0)) - _e38.g2_), (vec4<f32>(0.0) - _e43.g3_), (vec3<f32>(0.0) - _e48.g4_), (vec3<f32>(0.0) - _e53.g5_), (vec3<f32>(_e56.g0_.x, _e59.g0_.y, _e62.g0_.z) - _e66.g6_), (vec3<f32>(_e69.g1_.x, _e72.g1_.y, _e75.g1_.z) - _e79.g7_), (vec4<f32>(0.0) - _e84.g8_), (vec4<f32>(0.0) - _e89.g9_));
}

fn motor_multi_vector_regressive_product(self_888: Motor, other_750: MultiVector) -> MultiVector {
    var self_889: Motor;
    var other_751: MultiVector;

    self_889 = self_888;
    other_751 = other_750;
    let _e4: Motor = self_889;
    let _e8: MultiVector = other_751;
    let _e19: Motor = self_889;
    let _e23: MultiVector = other_751;
    let _e35: Motor = self_889;
    let _e39: MultiVector = other_751;
    let _e43: Motor = self_889;
    let _e47: MultiVector = other_751;
    let _e59: Motor = self_889;
    let _e63: MultiVector = other_751;
    let _e75: Motor = self_889;
    let _e79: MultiVector = other_751;
    let _e91: Motor = self_889;
    let _e95: MultiVector = other_751;
    let _e106: Motor = self_889;
    let _e110: MultiVector = other_751;
    let _e122: Motor = self_889;
    let _e126: MultiVector = other_751;
    let _e129: Motor = self_889;
    let _e133: MultiVector = other_751;
    let _e136: MultiVector = other_751;
    let _e139: MultiVector = other_751;
    let _e151: Motor = self_889;
    let _e155: MultiVector = other_751;
    let _e158: MultiVector = other_751;
    let _e161: MultiVector = other_751;
    let _e173: Motor = self_889;
    let _e177: MultiVector = other_751;
    let _e180: MultiVector = other_751;
    let _e183: MultiVector = other_751;
    let _e195: Motor = self_889;
    let _e198: Motor = self_889;
    let _e201: Motor = self_889;
    let _e205: MultiVector = other_751;
    let _e213: Motor = self_889;
    let _e217: MultiVector = other_751;
    let _e220: MultiVector = other_751;
    let _e226: Motor = self_889;
    let _e230: MultiVector = other_751;
    let _e233: MultiVector = other_751;
    let _e239: Motor = self_889;
    let _e243: MultiVector = other_751;
    let _e246: MultiVector = other_751;
    let _e252: Motor = self_889;
    let _e256: MultiVector = other_751;
    let _e260: Motor = self_889;
    let _e264: MultiVector = other_751;
    let _e275: Motor = self_889;
    let _e279: MultiVector = other_751;
    let _e290: Motor = self_889;
    let _e294: MultiVector = other_751;
    let _e304: Motor = self_889;
    let _e308: MultiVector = other_751;
    let _e319: Motor = self_889;
    let _e323: MultiVector = other_751;
    let _e334: Motor = self_889;
    let _e338: MultiVector = other_751;
    let _e350: Motor = self_889;
    let _e354: MultiVector = other_751;
    let _e358: Motor = self_889;
    let _e362: MultiVector = other_751;
    let _e374: Motor = self_889;
    let _e378: MultiVector = other_751;
    let _e390: Motor = self_889;
    let _e393: Motor = self_889;
    let _e396: Motor = self_889;
    let _e399: Motor = self_889;
    let _e403: MultiVector = other_751;
    let _e416: Motor = self_889;
    let _e420: MultiVector = other_751;
    let _e423: Motor = self_889;
    let _e426: Motor = self_889;
    let _e429: Motor = self_889;
    let _e433: MultiVector = other_751;
    let _e439: Motor = self_889;
    let _e443: MultiVector = other_751;
    let _e446: Motor = self_889;
    let _e449: Motor = self_889;
    let _e452: Motor = self_889;
    let _e456: MultiVector = other_751;
    let _e462: Motor = self_889;
    let _e466: MultiVector = other_751;
    let _e469: Motor = self_889;
    let _e472: Motor = self_889;
    let _e475: Motor = self_889;
    let _e479: MultiVector = other_751;
    let _e485: Motor = self_889;
    let _e489: MultiVector = other_751;
    let _e492: Motor = self_889;
    let _e495: Motor = self_889;
    let _e498: Motor = self_889;
    let _e502: MultiVector = other_751;
    let _e508: Motor = self_889;
    let _e512: MultiVector = other_751;
    let _e515: Motor = self_889;
    let _e519: MultiVector = other_751;
    return MultiVector((((((((((vec3<f32>(_e4.g0_.y) * vec3<f32>(_e8.g5_.y)) * vec3<f32>(-(1.0), 0.0, 0.0)) + ((vec3<f32>(_e19.g0_.z) * vec3<f32>(_e23.g5_.z)) * vec3<f32>(-(1.0), 0.0, 0.0))) + (vec3<f32>(_e35.g0_.w) * _e39.g0_)) + ((vec3<f32>(_e43.g1_.x) * vec3<f32>(_e47.g4_.x)) * vec3<f32>(-(1.0), 0.0, 0.0))) + ((vec3<f32>(_e59.g1_.y) * vec3<f32>(_e63.g4_.y)) * vec3<f32>(-(1.0), 0.0, 0.0))) + ((vec3<f32>(_e75.g1_.z) * vec3<f32>(_e79.g4_.z)) * vec3<f32>(-(1.0), 0.0, 0.0))) + ((vec3<f32>(_e91.g1_.w) * vec3<f32>(_e95.g0_.y)) * vec3<f32>(1.0, 0.0, 0.0))) + ((vec3<f32>(_e106.g0_.x) * vec3<f32>(_e110.g5_.x)) * vec3<f32>(-(1.0), 0.0, 0.0))), (((((vec3<f32>(_e122.g0_.w) * _e126.g1_) + ((vec3<f32>(_e129.g1_.x) * vec3<f32>(_e133.g8_.z, _e136.g8_.z, _e139.g8_.y)) * vec3<f32>(0.0, -(1.0), 1.0))) + ((vec3<f32>(_e151.g1_.y) * vec3<f32>(_e155.g8_.z, _e158.g8_.z, _e161.g8_.x)) * vec3<f32>(1.0, 0.0, -(1.0)))) + ((vec3<f32>(_e173.g1_.z) * vec3<f32>(_e177.g8_.y, _e180.g8_.x, _e183.g8_.y)) * vec3<f32>(-(1.0), 1.0, 0.0))) + (vec3<f32>(_e195.g0_.x, _e198.g0_.y, _e201.g0_.z) * vec3<f32>(_e205.g8_.w))), ((((((((vec2<f32>(0.0) - (vec2<f32>(_e213.g0_.x) * vec2<f32>(_e217.g8_.x, _e220.g7_.x))) - (vec2<f32>(_e226.g0_.y) * vec2<f32>(_e230.g8_.y, _e233.g7_.y))) - (vec2<f32>(_e239.g0_.z) * vec2<f32>(_e243.g8_.z, _e246.g7_.z))) + (vec2<f32>(_e252.g0_.w) * _e256.g2_)) + ((vec2<f32>(_e260.g1_.y) * vec2<f32>(_e264.g6_.y)) * vec2<f32>(0.0, -(1.0)))) + ((vec2<f32>(_e275.g1_.z) * vec2<f32>(_e279.g6_.z)) * vec2<f32>(0.0, -(1.0)))) + ((vec2<f32>(_e290.g1_.w) * vec2<f32>(_e294.g0_.z)) * vec2<f32>(0.0, 1.0))) + ((vec2<f32>(_e304.g1_.x) * vec2<f32>(_e308.g6_.x)) * vec2<f32>(0.0, -(1.0)))), (((((((vec4<f32>(_e319.g0_.y) * _e323.g9_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e334.g0_.z) * _e338.g9_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + (vec4<f32>(_e350.g0_.w) * _e354.g3_)) + ((vec4<f32>(_e358.g1_.y) * _e362.g9_.zzxz) * vec4<f32>(1.0, 0.0, -(1.0), 0.0))) + ((vec4<f32>(_e374.g1_.z) * _e378.g9_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))) + ((vec4<f32>(_e390.g0_.x, _e393.g1_.x, _e396.g1_.x, _e399.g0_.x) * _e403.g9_.wzyx) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))), ((vec3<f32>(_e416.g0_.w) * _e420.g4_) + (vec3<f32>(_e423.g0_.x, _e426.g0_.y, _e429.g0_.z) * vec3<f32>(_e433.g0_.y))), ((vec3<f32>(_e439.g0_.w) * _e443.g5_) + (vec3<f32>(_e446.g1_.x, _e449.g1_.y, _e452.g1_.z) * vec3<f32>(_e456.g0_.y))), ((vec3<f32>(_e462.g0_.w) * _e466.g6_) + (vec3<f32>(_e469.g0_.x, _e472.g0_.y, _e475.g0_.z) * vec3<f32>(_e479.g0_.z))), ((vec3<f32>(_e485.g0_.w) * _e489.g7_) + (vec3<f32>(_e492.g1_.x, _e495.g1_.y, _e498.g1_.z) * vec3<f32>(_e502.g0_.z))), (vec4<f32>(_e508.g0_.w) * _e512.g8_), (vec4<f32>(_e515.g0_.w) * _e519.g9_));
}

fn motor_scale(self_890: Motor, other_752: f32) -> Motor {
    var self_891: Motor;
    var other_753: f32;

    self_891 = self_890;
    other_753 = other_752;
    let _e4: Motor = self_891;
    let _e5: f32 = other_753;
    let _e7: Motor = motor_scalar_geometric_product(_e4, Scalar(_e5));
    return _e7;
}

fn rotor_zero() -> Rotor {
    return Rotor(vec4<f32>(0.0));
}

fn rotor_one() -> Rotor {
    return Rotor(vec4<f32>(0.0));
}

fn rotor_neg(self_892: Rotor) -> Rotor {
    var self_893: Rotor;

    self_893 = self_892;
    let _e2: Rotor = self_893;
    return Rotor((_e2.g0_ * vec4<f32>(-(1.0))));
}

fn rotor_automorphism(self_894: Rotor) -> Rotor {
    var self_895: Rotor;

    self_895 = self_894;
    let _e2: Rotor = self_895;
    return Rotor((_e2.g0_ * vec4<f32>(-(1.0))));
}

fn rotor_reversal(self_896: Rotor) -> Rotor {
    var self_897: Rotor;

    self_897 = self_896;
    let _e2: Rotor = self_897;
    return Rotor((_e2.g0_ * vec4<f32>(-(1.0), -(1.0), -(1.0), 1.0)));
}

fn rotor_conjugation(self_898: Rotor) -> Rotor {
    var self_899: Rotor;

    self_899 = self_898;
    let _e2: Rotor = self_899;
    return Rotor((_e2.g0_ * vec4<f32>(1.0, 1.0, 1.0, -(1.0))));
}

fn rotor_scalar_geometric_product(self_900: Rotor, other_754: Scalar) -> Rotor {
    var self_901: Rotor;
    var other_755: Scalar;

    self_901 = self_900;
    other_755 = other_754;
    let _e4: Rotor = self_901;
    let _e6: Scalar = other_755;
    return Rotor((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn rotor_scalar_regressive_product(self_902: Rotor, other_756: Scalar) -> Scalar {
    var self_903: Rotor;
    var other_757: Scalar;

    self_903 = self_902;
    other_757 = other_756;
    let _e4: Rotor = self_903;
    let _e7: Scalar = other_757;
    return Scalar((_e4.g0_.w * _e7.g0_));
}

fn rotor_scalar_outer_product(self_904: Rotor, other_758: Scalar) -> Rotor {
    var self_905: Rotor;
    var other_759: Scalar;

    self_905 = self_904;
    other_759 = other_758;
    let _e4: Rotor = self_905;
    let _e6: Scalar = other_759;
    return Rotor((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn rotor_scalar_inner_product(self_906: Rotor, other_760: Scalar) -> Rotor {
    var self_907: Rotor;
    var other_761: Scalar;

    self_907 = self_906;
    other_761 = other_760;
    let _e4: Rotor = self_907;
    let _e6: Scalar = other_761;
    return Rotor((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn rotor_scalar_right_contraction(self_908: Rotor, other_762: Scalar) -> Rotor {
    var self_909: Rotor;
    var other_763: Scalar;

    self_909 = self_908;
    other_763 = other_762;
    let _e4: Rotor = self_909;
    let _e6: Scalar = other_763;
    return Rotor((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn rotor_anti_scalar_into(self_910: Rotor) -> AntiScalar {
    var self_911: Rotor;

    self_911 = self_910;
    let _e2: Rotor = self_911;
    return AntiScalar(_e2.g0_.w);
}

fn rotor_anti_scalar_add(self_912: Rotor, other_764: AntiScalar) -> Rotor {
    var self_913: Rotor;
    var other_765: AntiScalar;

    self_913 = self_912;
    other_765 = other_764;
    let _e4: Rotor = self_913;
    let _e6: AntiScalar = other_765;
    return Rotor((_e4.g0_ + (vec4<f32>(_e6.g0_) * vec4<f32>(0.0, 0.0, 0.0, 1.0))));
}

fn rotor_anti_scalar_sub(self_914: Rotor, other_766: AntiScalar) -> Rotor {
    var self_915: Rotor;
    var other_767: AntiScalar;

    self_915 = self_914;
    other_767 = other_766;
    let _e4: Rotor = self_915;
    let _e6: AntiScalar = other_767;
    return Rotor((_e4.g0_ - (vec4<f32>(_e6.g0_) * vec4<f32>(0.0, 0.0, 0.0, 1.0))));
}

fn rotor_anti_scalar_regressive_product(self_916: Rotor, other_768: AntiScalar) -> Rotor {
    var self_917: Rotor;
    var other_769: AntiScalar;

    self_917 = self_916;
    other_769 = other_768;
    let _e4: Rotor = self_917;
    let _e6: AntiScalar = other_769;
    return Rotor((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn rotor_radial_point_regressive_product(self_918: Rotor, other_770: RadialPoint) -> RadialPoint {
    var self_919: Rotor;
    var other_771: RadialPoint;

    self_919 = self_918;
    other_771 = other_770;
    let _e4: Rotor = self_919;
    let _e8: RadialPoint = other_771;
    let _e11: Rotor = self_919;
    let _e15: RadialPoint = other_771;
    return RadialPoint((vec3<f32>(_e4.g0_.w) * _e8.g0_), (vec2<f32>(_e11.g0_.w) * _e15.g1_));
}

fn rotor_flat_point_regressive_product(self_920: Rotor, other_772: FlatPoint) -> FlatPoint {
    var self_921: Rotor;
    var other_773: FlatPoint;

    self_921 = self_920;
    other_773 = other_772;
    let _e4: Rotor = self_921;
    let _e8: FlatPoint = other_773;
    return FlatPoint((vec4<f32>(_e4.g0_.w) * _e8.g0_));
}

fn rotor_dipole_geometric_product(self_922: Rotor, other_774: Dipole) -> Rotor {
    var self_923: Rotor;
    var other_775: Dipole;

    self_923 = self_922;
    other_775 = other_774;
    let _e4: Rotor = self_923;
    let _e8: Dipole = other_775;
    let _e11: Dipole = other_775;
    let _e14: Dipole = other_775;
    let _e17: Dipole = other_775;
    let _e30: Rotor = self_923;
    let _e34: Dipole = other_775;
    let _e37: Dipole = other_775;
    let _e40: Dipole = other_775;
    let _e43: Dipole = other_775;
    let _e57: Rotor = self_923;
    let _e61: Dipole = other_775;
    let _e64: Dipole = other_775;
    let _e67: Dipole = other_775;
    let _e70: Dipole = other_775;
    let _e82: Rotor = self_923;
    let _e86: Dipole = other_775;
    let _e89: Dipole = other_775;
    let _e92: Dipole = other_775;
    let _e95: Dipole = other_775;
    return Rotor((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g1_.z, _e11.g1_.z, _e14.g1_.x, _e17.g1_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e30.g0_.z) * vec4<f32>(_e34.g1_.y, _e37.g1_.x, _e40.g1_.y, _e43.g1_.z)) * vec4<f32>(1.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e57.g0_.w) * vec4<f32>(_e61.g1_.x, _e64.g1_.y, _e67.g1_.z, _e70.g1_.x)) * vec4<f32>(1.0, 1.0, 1.0, 0.0))) + ((vec4<f32>(_e82.g0_.x) * vec4<f32>(_e86.g1_.x, _e89.g1_.z, _e92.g1_.y, _e95.g1_.x)) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))));
}

fn rotor_dipole_outer_product(self_924: Rotor, other_776: Dipole) -> AntiScalar {
    var self_925: Rotor;
    var other_777: Dipole;

    self_925 = self_924;
    other_777 = other_776;
    let _e5: Rotor = self_925;
    let _e8: Dipole = other_777;
    let _e13: Rotor = self_925;
    let _e16: Dipole = other_777;
    let _e21: Rotor = self_925;
    let _e24: Dipole = other_777;
    return AntiScalar((((0.0 - (_e5.g0_.x * _e8.g1_.x)) - (_e13.g0_.y * _e16.g1_.y)) - (_e21.g0_.z * _e24.g1_.z)));
}

fn rotor_plane_regressive_product(self_926: Rotor, other_778: Plane) -> Flector {
    var self_927: Rotor;
    var other_779: Plane;

    self_927 = self_926;
    other_779 = other_778;
    let _e4: Rotor = self_927;
    let _e8: Plane = other_779;
    let _e19: Rotor = self_927;
    let _e23: Plane = other_779;
    let _e35: Rotor = self_927;
    let _e39: Plane = other_779;
    let _e51: Rotor = self_927;
    let _e55: Plane = other_779;
    return Flector(((((vec4<f32>(_e4.g0_.y) * _e8.g0_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e35.g0_.x) * _e39.g0_.wxxx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))), (vec4<f32>(_e51.g0_.w) * _e55.g0_));
}

fn rotor_motor_add(self_928: Rotor, other_780: Motor) -> Motor {
    var self_929: Rotor;
    var other_781: Motor;

    self_929 = self_928;
    other_781 = other_780;
    let _e4: Rotor = self_929;
    let _e6: Motor = other_781;
    let _e9: Motor = other_781;
    return Motor((_e4.g0_ + _e6.g0_), _e9.g1_);
}

fn rotor_motor_sub(self_930: Rotor, other_782: Motor) -> Motor {
    var self_931: Rotor;
    var other_783: Motor;

    self_931 = self_930;
    other_783 = other_782;
    let _e4: Rotor = self_931;
    let _e6: Motor = other_783;
    let _e11: Motor = other_783;
    return Motor((_e4.g0_ - _e6.g0_), (vec4<f32>(0.0) - _e11.g1_));
}

fn rotor_motor_regressive_product(self_932: Rotor, other_784: Motor) -> Motor {
    var self_933: Rotor;
    var other_785: Motor;

    self_933 = self_932;
    other_785 = other_784;
    let _e4: Rotor = self_933;
    let _e8: Motor = other_785;
    let _e11: Rotor = self_933;
    let _e14: Motor = other_785;
    let _e25: Rotor = self_933;
    let _e29: Motor = other_785;
    let _e41: Rotor = self_933;
    let _e45: Motor = other_785;
    let _e58: Rotor = self_933;
    let _e62: Motor = other_785;
    let _e66: Rotor = self_933;
    let _e70: Motor = other_785;
    return Motor(((vec4<f32>(_e4.g0_.w) * _e8.g0_) + ((_e11.g0_.xyzx * _e14.g0_.wwwx) * vec4<f32>(1.0, 1.0, 1.0, 0.0))), (((((vec4<f32>(_e25.g0_.y) * vec4<f32>(_e29.g1_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0))) + ((vec4<f32>(_e41.g0_.z) * vec4<f32>(_e45.g1_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + (vec4<f32>(_e58.g0_.w) * _e62.g1_)) + ((vec4<f32>(_e66.g0_.x) * vec4<f32>(_e70.g1_.x)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))));
}

fn rotor_rotor_add(self_934: Rotor, other_786: Rotor) -> Rotor {
    var self_935: Rotor;
    var other_787: Rotor;

    self_935 = self_934;
    other_787 = other_786;
    let _e4: Rotor = self_935;
    let _e6: Rotor = other_787;
    return Rotor((_e4.g0_ + _e6.g0_));
}

fn rotor_rotor_sub(self_936: Rotor, other_788: Rotor) -> Rotor {
    var self_937: Rotor;
    var other_789: Rotor;

    self_937 = self_936;
    other_789 = other_788;
    let _e4: Rotor = self_937;
    let _e6: Rotor = other_789;
    return Rotor((_e4.g0_ - _e6.g0_));
}

fn rotor_rotor_mul(self_938: Rotor, other_790: Rotor) -> Rotor {
    var self_939: Rotor;
    var other_791: Rotor;

    self_939 = self_938;
    other_791 = other_790;
    let _e4: Rotor = self_939;
    let _e6: Rotor = other_791;
    return Rotor((_e4.g0_ * _e6.g0_));
}

fn rotor_rotor_div(self_940: Rotor, other_792: Rotor) -> Rotor {
    var self_941: Rotor;
    var other_793: Rotor;

    self_941 = self_940;
    other_793 = other_792;
    let _e4: Rotor = self_941;
    let _e7: Rotor = self_941;
    let _e10: Rotor = self_941;
    let _e13: Rotor = self_941;
    let _e23: Rotor = other_793;
    let _e26: Rotor = other_793;
    let _e29: Rotor = other_793;
    let _e32: Rotor = other_793;
    return Rotor((((vec4<f32>(_e4.g0_.x, _e7.g0_.y, _e10.g0_.z, _e13.g0_.w) * vec4<f32>(1.0, 1.0, 1.0, 1.0)) / vec4<f32>(_e23.g0_.x, _e26.g0_.y, _e29.g0_.z, _e32.g0_.w)) * vec4<f32>(1.0, 1.0, 1.0, 1.0)));
}

fn rotor_rotor_regressive_product(self_942: Rotor, other_794: Rotor) -> Rotor {
    var self_943: Rotor;
    var other_795: Rotor;

    self_943 = self_942;
    other_795 = other_794;
    let _e4: Rotor = self_943;
    let _e8: Rotor = other_795;
    let _e11: Rotor = self_943;
    let _e14: Rotor = other_795;
    return Rotor(((vec4<f32>(_e4.g0_.w) * _e8.g0_) + ((_e11.g0_.xyzx * _e14.g0_.wwwx) * vec4<f32>(1.0, 1.0, 1.0, 0.0))));
}

fn rotor_translator_regressive_product(self_944: Rotor, other_796: Translator) -> Motor {
    var self_945: Rotor;
    var other_797: Translator;

    self_945 = self_944;
    other_797 = other_796;
    let _e4: Rotor = self_945;
    let _e6: Translator = other_797;
    let _e11: Rotor = self_945;
    let _e15: Translator = other_797;
    let _e27: Rotor = self_945;
    let _e31: Translator = other_797;
    let _e44: Rotor = self_945;
    let _e47: Translator = other_797;
    return Motor((_e4.g0_ * vec4<f32>(_e6.g0_.w)), ((((vec4<f32>(_e11.g0_.y) * vec4<f32>(_e15.g0_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0))) + ((vec4<f32>(_e27.g0_.z) * vec4<f32>(_e31.g0_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((_e44.g0_.wwwx * _e47.g0_.xyzx) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))));
}

fn rotor_flector_regressive_product(self_946: Rotor, other_798: Flector) -> Flector {
    var self_947: Rotor;
    var other_799: Flector;

    self_947 = self_946;
    other_799 = other_798;
    let _e4: Rotor = self_947;
    let _e8: Flector = other_799;
    let _e19: Rotor = self_947;
    let _e23: Flector = other_799;
    let _e35: Rotor = self_947;
    let _e39: Flector = other_799;
    let _e43: Rotor = self_947;
    let _e47: Flector = other_799;
    let _e59: Rotor = self_947;
    let _e63: Flector = other_799;
    return Flector((((((vec4<f32>(_e4.g0_.y) * _e8.g1_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e19.g0_.z) * _e23.g1_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + (vec4<f32>(_e35.g0_.w) * _e39.g0_)) + ((vec4<f32>(_e43.g0_.x) * _e47.g1_.wxxx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))), (vec4<f32>(_e59.g0_.w) * _e63.g1_));
}

fn rotor_multi_vector_add(self_948: Rotor, other_800: MultiVector) -> MultiVector {
    var self_949: Rotor;
    var other_801: MultiVector;

    self_949 = self_948;
    other_801 = other_800;
    let _e4: Rotor = self_949;
    let _e7: Rotor = self_949;
    let _e10: Rotor = self_949;
    let _e19: MultiVector = other_801;
    let _e22: MultiVector = other_801;
    let _e24: MultiVector = other_801;
    let _e26: MultiVector = other_801;
    let _e28: MultiVector = other_801;
    let _e30: MultiVector = other_801;
    let _e32: Rotor = self_949;
    let _e35: Rotor = self_949;
    let _e38: Rotor = self_949;
    let _e42: MultiVector = other_801;
    let _e45: MultiVector = other_801;
    let _e47: MultiVector = other_801;
    let _e49: MultiVector = other_801;
    return MultiVector(((vec3<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.w) * vec3<f32>(0.0, 0.0, 1.0)) + _e19.g0_), _e22.g1_, _e24.g2_, _e26.g3_, _e28.g4_, _e30.g5_, (vec3<f32>(_e32.g0_.x, _e35.g0_.y, _e38.g0_.z) + _e42.g6_), _e45.g7_, _e47.g8_, _e49.g9_);
}

fn rotor_multi_vector_sub(self_950: Rotor, other_802: MultiVector) -> MultiVector {
    var self_951: Rotor;
    var other_803: MultiVector;

    self_951 = self_950;
    other_803 = other_802;
    let _e4: Rotor = self_951;
    let _e7: Rotor = self_951;
    let _e10: Rotor = self_951;
    let _e19: MultiVector = other_803;
    let _e24: MultiVector = other_803;
    let _e29: MultiVector = other_803;
    let _e34: MultiVector = other_803;
    let _e39: MultiVector = other_803;
    let _e44: MultiVector = other_803;
    let _e47: Rotor = self_951;
    let _e50: Rotor = self_951;
    let _e53: Rotor = self_951;
    let _e57: MultiVector = other_803;
    let _e62: MultiVector = other_803;
    let _e67: MultiVector = other_803;
    let _e72: MultiVector = other_803;
    return MultiVector(((vec3<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.w) * vec3<f32>(0.0, 0.0, 1.0)) - _e19.g0_), (vec3<f32>(0.0) - _e24.g1_), (vec2<f32>(0.0) - _e29.g2_), (vec4<f32>(0.0) - _e34.g3_), (vec3<f32>(0.0) - _e39.g4_), (vec3<f32>(0.0) - _e44.g5_), (vec3<f32>(_e47.g0_.x, _e50.g0_.y, _e53.g0_.z) - _e57.g6_), (vec3<f32>(0.0) - _e62.g7_), (vec4<f32>(0.0) - _e67.g8_), (vec4<f32>(0.0) - _e72.g9_));
}

fn rotor_multi_vector_regressive_product(self_952: Rotor, other_804: MultiVector) -> MultiVector {
    var self_953: Rotor;
    var other_805: MultiVector;

    self_953 = self_952;
    other_805 = other_804;
    let _e4: Rotor = self_953;
    let _e8: MultiVector = other_805;
    let _e19: Rotor = self_953;
    let _e23: MultiVector = other_805;
    let _e35: Rotor = self_953;
    let _e39: MultiVector = other_805;
    let _e43: Rotor = self_953;
    let _e47: MultiVector = other_805;
    let _e59: Rotor = self_953;
    let _e63: MultiVector = other_805;
    let _e66: Rotor = self_953;
    let _e69: Rotor = self_953;
    let _e72: Rotor = self_953;
    let _e76: MultiVector = other_805;
    let _e84: Rotor = self_953;
    let _e88: MultiVector = other_805;
    let _e91: MultiVector = other_805;
    let _e97: Rotor = self_953;
    let _e101: MultiVector = other_805;
    let _e104: MultiVector = other_805;
    let _e110: Rotor = self_953;
    let _e114: MultiVector = other_805;
    let _e117: MultiVector = other_805;
    let _e123: Rotor = self_953;
    let _e127: MultiVector = other_805;
    let _e131: Rotor = self_953;
    let _e135: MultiVector = other_805;
    let _e146: Rotor = self_953;
    let _e150: MultiVector = other_805;
    let _e162: Rotor = self_953;
    let _e166: MultiVector = other_805;
    let _e170: Rotor = self_953;
    let _e174: MultiVector = other_805;
    let _e186: Rotor = self_953;
    let _e190: MultiVector = other_805;
    let _e193: Rotor = self_953;
    let _e196: Rotor = self_953;
    let _e199: Rotor = self_953;
    let _e203: MultiVector = other_805;
    let _e209: Rotor = self_953;
    let _e213: MultiVector = other_805;
    let _e216: Rotor = self_953;
    let _e220: MultiVector = other_805;
    let _e223: Rotor = self_953;
    let _e226: Rotor = self_953;
    let _e229: Rotor = self_953;
    let _e233: MultiVector = other_805;
    let _e239: Rotor = self_953;
    let _e243: MultiVector = other_805;
    let _e246: Rotor = self_953;
    let _e250: MultiVector = other_805;
    let _e253: Rotor = self_953;
    let _e257: MultiVector = other_805;
    return MultiVector((((((vec3<f32>(_e4.g0_.y) * vec3<f32>(_e8.g5_.y)) * vec3<f32>(-(1.0), 0.0, 0.0)) + ((vec3<f32>(_e19.g0_.z) * vec3<f32>(_e23.g5_.z)) * vec3<f32>(-(1.0), 0.0, 0.0))) + (vec3<f32>(_e35.g0_.w) * _e39.g0_)) + ((vec3<f32>(_e43.g0_.x) * vec3<f32>(_e47.g5_.x)) * vec3<f32>(-(1.0), 0.0, 0.0))), ((vec3<f32>(_e59.g0_.w) * _e63.g1_) + (vec3<f32>(_e66.g0_.x, _e69.g0_.y, _e72.g0_.z) * vec3<f32>(_e76.g8_.w))), ((((vec2<f32>(0.0) - (vec2<f32>(_e84.g0_.x) * vec2<f32>(_e88.g8_.x, _e91.g7_.x))) - (vec2<f32>(_e97.g0_.y) * vec2<f32>(_e101.g8_.y, _e104.g7_.y))) - (vec2<f32>(_e110.g0_.z) * vec2<f32>(_e114.g8_.z, _e117.g7_.z))) + (vec2<f32>(_e123.g0_.w) * _e127.g2_)), (((((vec4<f32>(_e131.g0_.y) * _e135.g9_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e146.g0_.z) * _e150.g9_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + (vec4<f32>(_e162.g0_.w) * _e166.g3_)) + ((vec4<f32>(_e170.g0_.x) * _e174.g9_.wxxx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))), ((vec3<f32>(_e186.g0_.w) * _e190.g4_) + (vec3<f32>(_e193.g0_.x, _e196.g0_.y, _e199.g0_.z) * vec3<f32>(_e203.g0_.y))), (vec3<f32>(_e209.g0_.w) * _e213.g5_), ((vec3<f32>(_e216.g0_.w) * _e220.g6_) + (vec3<f32>(_e223.g0_.x, _e226.g0_.y, _e229.g0_.z) * vec3<f32>(_e233.g0_.z))), (vec3<f32>(_e239.g0_.w) * _e243.g7_), (vec4<f32>(_e246.g0_.w) * _e250.g8_), (vec4<f32>(_e253.g0_.w) * _e257.g9_));
}

fn rotor_scale(self_954: Rotor, other_806: f32) -> Rotor {
    var self_955: Rotor;
    var other_807: f32;

    self_955 = self_954;
    other_807 = other_806;
    let _e4: Rotor = self_955;
    let _e5: f32 = other_807;
    let _e7: Rotor = rotor_scalar_geometric_product(_e4, Scalar(_e5));
    return _e7;
}

fn translator_zero() -> Translator {
    return Translator(vec4<f32>(0.0));
}

fn translator_one() -> Translator {
    return Translator(vec4<f32>(0.0));
}

fn translator_neg(self_956: Translator) -> Translator {
    var self_957: Translator;

    self_957 = self_956;
    let _e2: Translator = self_957;
    return Translator((_e2.g0_ * vec4<f32>(-(1.0))));
}

fn translator_automorphism(self_958: Translator) -> Translator {
    var self_959: Translator;

    self_959 = self_958;
    let _e2: Translator = self_959;
    return Translator((_e2.g0_ * vec4<f32>(-(1.0))));
}

fn translator_reversal(self_960: Translator) -> Translator {
    var self_961: Translator;

    self_961 = self_960;
    let _e2: Translator = self_961;
    return Translator((_e2.g0_ * vec4<f32>(-(1.0), -(1.0), -(1.0), 1.0)));
}

fn translator_conjugation(self_962: Translator) -> Translator {
    var self_963: Translator;

    self_963 = self_962;
    let _e2: Translator = self_963;
    return Translator((_e2.g0_ * vec4<f32>(1.0, 1.0, 1.0, -(1.0))));
}

fn translator_scalar_geometric_product(self_964: Translator, other_808: Scalar) -> Translator {
    var self_965: Translator;
    var other_809: Scalar;

    self_965 = self_964;
    other_809 = other_808;
    let _e4: Translator = self_965;
    let _e6: Scalar = other_809;
    return Translator((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn translator_scalar_regressive_product(self_966: Translator, other_810: Scalar) -> Scalar {
    var self_967: Translator;
    var other_811: Scalar;

    self_967 = self_966;
    other_811 = other_810;
    let _e4: Translator = self_967;
    let _e7: Scalar = other_811;
    return Scalar((_e4.g0_.w * _e7.g0_));
}

fn translator_scalar_outer_product(self_968: Translator, other_812: Scalar) -> Translator {
    var self_969: Translator;
    var other_813: Scalar;

    self_969 = self_968;
    other_813 = other_812;
    let _e4: Translator = self_969;
    let _e6: Scalar = other_813;
    return Translator((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn translator_scalar_inner_product(self_970: Translator, other_814: Scalar) -> Translator {
    var self_971: Translator;
    var other_815: Scalar;

    self_971 = self_970;
    other_815 = other_814;
    let _e4: Translator = self_971;
    let _e6: Scalar = other_815;
    return Translator((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn translator_scalar_right_contraction(self_972: Translator, other_816: Scalar) -> Translator {
    var self_973: Translator;
    var other_817: Scalar;

    self_973 = self_972;
    other_817 = other_816;
    let _e4: Translator = self_973;
    let _e6: Scalar = other_817;
    return Translator((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn translator_anti_scalar_into(self_974: Translator) -> AntiScalar {
    var self_975: Translator;

    self_975 = self_974;
    let _e2: Translator = self_975;
    return AntiScalar(_e2.g0_.w);
}

fn translator_anti_scalar_add(self_976: Translator, other_818: AntiScalar) -> Translator {
    var self_977: Translator;
    var other_819: AntiScalar;

    self_977 = self_976;
    other_819 = other_818;
    let _e4: Translator = self_977;
    let _e6: AntiScalar = other_819;
    return Translator((_e4.g0_ + (vec4<f32>(_e6.g0_) * vec4<f32>(0.0, 0.0, 0.0, 1.0))));
}

fn translator_anti_scalar_sub(self_978: Translator, other_820: AntiScalar) -> Translator {
    var self_979: Translator;
    var other_821: AntiScalar;

    self_979 = self_978;
    other_821 = other_820;
    let _e4: Translator = self_979;
    let _e6: AntiScalar = other_821;
    return Translator((_e4.g0_ - (vec4<f32>(_e6.g0_) * vec4<f32>(0.0, 0.0, 0.0, 1.0))));
}

fn translator_anti_scalar_regressive_product(self_980: Translator, other_822: AntiScalar) -> Translator {
    var self_981: Translator;
    var other_823: AntiScalar;

    self_981 = self_980;
    other_823 = other_822;
    let _e4: Translator = self_981;
    let _e6: AntiScalar = other_823;
    return Translator((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn translator_radial_point_regressive_product(self_982: Translator, other_824: RadialPoint) -> RadialPoint {
    var self_983: Translator;
    var other_825: RadialPoint;

    self_983 = self_982;
    other_825 = other_824;
    let _e4: Translator = self_983;
    let _e8: RadialPoint = other_825;
    let _e11: Translator = self_983;
    let _e15: RadialPoint = other_825;
    return RadialPoint((vec3<f32>(_e4.g0_.w) * _e8.g0_), (vec2<f32>(_e11.g0_.w) * _e15.g1_));
}

fn translator_radial_point_outer_product(self_984: Translator, other_826: RadialPoint) -> Plane {
    var self_985: Translator;
    var other_827: RadialPoint;

    self_985 = self_984;
    other_827 = other_826;
    let _e4: Translator = self_985;
    let _e8: RadialPoint = other_827;
    let _e11: RadialPoint = other_827;
    let _e14: RadialPoint = other_827;
    let _e17: RadialPoint = other_827;
    let _e29: Translator = self_985;
    let _e33: RadialPoint = other_827;
    let _e36: RadialPoint = other_827;
    let _e39: RadialPoint = other_827;
    let _e42: RadialPoint = other_827;
    let _e55: Translator = self_985;
    let _e59: RadialPoint = other_827;
    let _e62: RadialPoint = other_827;
    let _e65: RadialPoint = other_827;
    let _e68: RadialPoint = other_827;
    return Plane(((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g1_.x, _e11.g1_.x, _e14.g1_.x, _e17.g0_.y)) * vec4<f32>(0.0, -(1.0), 0.0, 1.0)) + ((vec4<f32>(_e29.g0_.z) * vec4<f32>(_e33.g1_.x, _e36.g1_.x, _e39.g1_.x, _e42.g0_.z)) * vec4<f32>(0.0, 0.0, -(1.0), 1.0))) + ((vec4<f32>(_e55.g0_.x) * vec4<f32>(_e59.g1_.x, _e62.g1_.x, _e65.g1_.x, _e68.g0_.x)) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))));
}

fn translator_flat_point_regressive_product(self_986: Translator, other_828: FlatPoint) -> FlatPoint {
    var self_987: Translator;
    var other_829: FlatPoint;

    self_987 = self_986;
    other_829 = other_828;
    let _e4: Translator = self_987;
    let _e8: FlatPoint = other_829;
    return FlatPoint((vec4<f32>(_e4.g0_.w) * _e8.g0_));
}

fn translator_dipole_geometric_product(self_988: Translator, other_830: Dipole) -> Motor {
    var self_989: Translator;
    var other_831: Dipole;

    self_989 = self_988;
    other_831 = other_830;
    let _e4: Translator = self_989;
    let _e8: Dipole = other_831;
    let _e11: Dipole = other_831;
    let _e14: Dipole = other_831;
    let _e17: Dipole = other_831;
    let _e30: Translator = self_989;
    let _e34: Dipole = other_831;
    let _e37: Dipole = other_831;
    let _e40: Dipole = other_831;
    let _e43: Dipole = other_831;
    let _e57: Translator = self_989;
    let _e61: Dipole = other_831;
    let _e64: Dipole = other_831;
    let _e67: Dipole = other_831;
    let _e70: Dipole = other_831;
    let _e82: Translator = self_989;
    let _e86: Dipole = other_831;
    let _e89: Dipole = other_831;
    let _e92: Dipole = other_831;
    let _e95: Dipole = other_831;
    let _e109: Translator = self_989;
    let _e113: Dipole = other_831;
    let _e116: Dipole = other_831;
    let _e119: Dipole = other_831;
    let _e122: Dipole = other_831;
    let _e135: Translator = self_989;
    let _e139: Dipole = other_831;
    let _e142: Dipole = other_831;
    let _e145: Dipole = other_831;
    let _e148: Dipole = other_831;
    let _e162: Translator = self_989;
    let _e166: Dipole = other_831;
    let _e169: Dipole = other_831;
    let _e172: Dipole = other_831;
    let _e175: Dipole = other_831;
    return Motor((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.z, _e11.g0_.z, _e14.g0_.x, _e17.g0_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e30.g0_.z) * vec4<f32>(_e34.g0_.y, _e37.g0_.x, _e40.g0_.y, _e43.g0_.z)) * vec4<f32>(1.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e57.g0_.w) * vec4<f32>(_e61.g1_.x, _e64.g1_.y, _e67.g1_.z, _e70.g1_.x)) * vec4<f32>(1.0, 1.0, 1.0, 0.0))) + ((vec4<f32>(_e82.g0_.x) * vec4<f32>(_e86.g0_.x, _e89.g0_.z, _e92.g0_.y, _e95.g0_.x)) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))), ((((vec4<f32>(_e109.g0_.y) * vec4<f32>(_e113.g1_.z, _e116.g1_.z, _e119.g1_.x, _e122.g1_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e135.g0_.z) * vec4<f32>(_e139.g1_.y, _e142.g1_.x, _e145.g1_.y, _e148.g1_.z)) * vec4<f32>(1.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e162.g0_.x) * vec4<f32>(_e166.g1_.x, _e169.g1_.z, _e172.g1_.y, _e175.g1_.x)) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))));
}

fn translator_dipole_outer_product(self_990: Translator, other_832: Dipole) -> AntiScalar {
    var self_991: Translator;
    var other_833: Dipole;

    self_991 = self_990;
    other_833 = other_832;
    let _e5: Translator = self_991;
    let _e8: Dipole = other_833;
    let _e13: Translator = self_991;
    let _e16: Dipole = other_833;
    let _e21: Translator = self_991;
    let _e24: Dipole = other_833;
    return AntiScalar((((0.0 - (_e5.g0_.x * _e8.g0_.x)) - (_e13.g0_.y * _e16.g0_.y)) - (_e21.g0_.z * _e24.g0_.z)));
}

fn translator_motor_add(self_992: Translator, other_834: Motor) -> Motor {
    var self_993: Translator;
    var other_835: Motor;

    self_993 = self_992;
    other_835 = other_834;
    let _e4: Translator = self_993;
    let _e13: Motor = other_835;
    let _e16: Translator = self_993;
    let _e25: Motor = other_835;
    return Motor(((_e4.g0_.xxxw * vec4<f32>(0.0, 0.0, 0.0, 1.0)) + _e13.g0_), ((_e16.g0_.xyzx * vec4<f32>(1.0, 1.0, 1.0, 0.0)) + _e25.g1_));
}

fn translator_motor_sub(self_994: Translator, other_836: Motor) -> Motor {
    var self_995: Translator;
    var other_837: Motor;

    self_995 = self_994;
    other_837 = other_836;
    let _e4: Translator = self_995;
    let _e13: Motor = other_837;
    let _e16: Translator = self_995;
    let _e25: Motor = other_837;
    return Motor(((_e4.g0_.xxxw * vec4<f32>(0.0, 0.0, 0.0, 1.0)) - _e13.g0_), ((_e16.g0_.xyzx * vec4<f32>(1.0, 1.0, 1.0, 0.0)) - _e25.g1_));
}

fn translator_motor_regressive_product(self_996: Translator, other_838: Motor) -> Motor {
    var self_997: Translator;
    var other_839: Motor;

    self_997 = self_996;
    other_839 = other_838;
    let _e4: Translator = self_997;
    let _e8: Motor = other_839;
    let _e11: Translator = self_997;
    let _e15: Motor = other_839;
    let _e26: Translator = self_997;
    let _e30: Motor = other_839;
    let _e42: Translator = self_997;
    let _e46: Motor = other_839;
    let _e50: Translator = self_997;
    let _e54: Motor = other_839;
    return Motor((vec4<f32>(_e4.g0_.w) * _e8.g0_), (((((vec4<f32>(_e11.g0_.y) * _e15.g0_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e26.g0_.z) * _e30.g0_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + (vec4<f32>(_e42.g0_.w) * _e46.g1_)) + ((vec4<f32>(_e50.g0_.x) * _e54.g0_.wxxx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))));
}

fn translator_rotor_regressive_product(self_998: Translator, other_840: Rotor) -> Motor {
    var self_999: Translator;
    var other_841: Rotor;

    self_999 = self_998;
    other_841 = other_840;
    let _e4: Translator = self_999;
    let _e8: Rotor = other_841;
    let _e11: Translator = self_999;
    let _e15: Rotor = other_841;
    let _e26: Translator = self_999;
    let _e30: Rotor = other_841;
    let _e42: Translator = self_999;
    let _e46: Rotor = other_841;
    return Motor((vec4<f32>(_e4.g0_.w) * _e8.g0_), ((((vec4<f32>(_e11.g0_.y) * _e15.g0_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e26.g0_.z) * _e30.g0_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e42.g0_.x) * _e46.g0_.wxxx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))));
}

fn translator_translator_add(self_1000: Translator, other_842: Translator) -> Translator {
    var self_1001: Translator;
    var other_843: Translator;

    self_1001 = self_1000;
    other_843 = other_842;
    let _e4: Translator = self_1001;
    let _e6: Translator = other_843;
    return Translator((_e4.g0_ + _e6.g0_));
}

fn translator_translator_sub(self_1002: Translator, other_844: Translator) -> Translator {
    var self_1003: Translator;
    var other_845: Translator;

    self_1003 = self_1002;
    other_845 = other_844;
    let _e4: Translator = self_1003;
    let _e6: Translator = other_845;
    return Translator((_e4.g0_ - _e6.g0_));
}

fn translator_translator_mul(self_1004: Translator, other_846: Translator) -> Translator {
    var self_1005: Translator;
    var other_847: Translator;

    self_1005 = self_1004;
    other_847 = other_846;
    let _e4: Translator = self_1005;
    let _e6: Translator = other_847;
    return Translator((_e4.g0_ * _e6.g0_));
}

fn translator_translator_div(self_1006: Translator, other_848: Translator) -> Translator {
    var self_1007: Translator;
    var other_849: Translator;

    self_1007 = self_1006;
    other_849 = other_848;
    let _e4: Translator = self_1007;
    let _e7: Translator = self_1007;
    let _e10: Translator = self_1007;
    let _e13: Translator = self_1007;
    let _e23: Translator = other_849;
    let _e26: Translator = other_849;
    let _e29: Translator = other_849;
    let _e32: Translator = other_849;
    return Translator((((vec4<f32>(_e4.g0_.x, _e7.g0_.y, _e10.g0_.z, _e13.g0_.w) * vec4<f32>(1.0, 1.0, 1.0, 1.0)) / vec4<f32>(_e23.g0_.x, _e26.g0_.y, _e29.g0_.z, _e32.g0_.w)) * vec4<f32>(1.0, 1.0, 1.0, 1.0)));
}

fn translator_translator_regressive_product(self_1008: Translator, other_850: Translator) -> Translator {
    var self_1009: Translator;
    var other_851: Translator;

    self_1009 = self_1008;
    other_851 = other_850;
    let _e4: Translator = self_1009;
    let _e8: Translator = other_851;
    let _e11: Translator = self_1009;
    let _e14: Translator = other_851;
    return Translator(((vec4<f32>(_e4.g0_.w) * _e8.g0_) + ((_e11.g0_.xyzx * _e14.g0_.wwwx) * vec4<f32>(1.0, 1.0, 1.0, 0.0))));
}

fn translator_flector_regressive_product(self_1010: Translator, other_852: Flector) -> Flector {
    var self_1011: Translator;
    var other_853: Flector;

    self_1011 = self_1010;
    other_853 = other_852;
    let _e4: Translator = self_1011;
    let _e8: Flector = other_853;
    let _e19: Translator = self_1011;
    let _e23: Flector = other_853;
    let _e35: Translator = self_1011;
    let _e39: Flector = other_853;
    let _e43: Translator = self_1011;
    let _e47: Flector = other_853;
    let _e59: Translator = self_1011;
    let _e63: Flector = other_853;
    return Flector((((((vec4<f32>(_e4.g0_.y) * _e8.g1_.zzxz) * vec4<f32>(1.0, 0.0, -(1.0), 0.0)) + ((vec4<f32>(_e19.g0_.z) * _e23.g1_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))) + (vec4<f32>(_e35.g0_.w) * _e39.g0_)) + ((vec4<f32>(_e43.g0_.x) * _e47.g1_.xzyx) * vec4<f32>(0.0, -(1.0), 1.0, 0.0))), (vec4<f32>(_e59.g0_.w) * _e63.g1_));
}

fn translator_multi_vector_add(self_1012: Translator, other_854: MultiVector) -> MultiVector {
    var self_1013: Translator;
    var other_855: MultiVector;

    self_1013 = self_1012;
    other_855 = other_854;
    let _e4: Translator = self_1013;
    let _e7: Translator = self_1013;
    let _e10: Translator = self_1013;
    let _e19: MultiVector = other_855;
    let _e22: MultiVector = other_855;
    let _e24: MultiVector = other_855;
    let _e26: MultiVector = other_855;
    let _e28: MultiVector = other_855;
    let _e30: MultiVector = other_855;
    let _e32: MultiVector = other_855;
    let _e34: Translator = self_1013;
    let _e37: Translator = self_1013;
    let _e40: Translator = self_1013;
    let _e44: MultiVector = other_855;
    let _e47: MultiVector = other_855;
    let _e49: MultiVector = other_855;
    return MultiVector(((vec3<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.w) * vec3<f32>(0.0, 0.0, 1.0)) + _e19.g0_), _e22.g1_, _e24.g2_, _e26.g3_, _e28.g4_, _e30.g5_, _e32.g6_, (vec3<f32>(_e34.g0_.x, _e37.g0_.y, _e40.g0_.z) + _e44.g7_), _e47.g8_, _e49.g9_);
}

fn translator_multi_vector_sub(self_1014: Translator, other_856: MultiVector) -> MultiVector {
    var self_1015: Translator;
    var other_857: MultiVector;

    self_1015 = self_1014;
    other_857 = other_856;
    let _e4: Translator = self_1015;
    let _e7: Translator = self_1015;
    let _e10: Translator = self_1015;
    let _e19: MultiVector = other_857;
    let _e24: MultiVector = other_857;
    let _e29: MultiVector = other_857;
    let _e34: MultiVector = other_857;
    let _e39: MultiVector = other_857;
    let _e44: MultiVector = other_857;
    let _e49: MultiVector = other_857;
    let _e52: Translator = self_1015;
    let _e55: Translator = self_1015;
    let _e58: Translator = self_1015;
    let _e62: MultiVector = other_857;
    let _e67: MultiVector = other_857;
    let _e72: MultiVector = other_857;
    return MultiVector(((vec3<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.w) * vec3<f32>(0.0, 0.0, 1.0)) - _e19.g0_), (vec3<f32>(0.0) - _e24.g1_), (vec2<f32>(0.0) - _e29.g2_), (vec4<f32>(0.0) - _e34.g3_), (vec3<f32>(0.0) - _e39.g4_), (vec3<f32>(0.0) - _e44.g5_), (vec3<f32>(0.0) - _e49.g6_), (vec3<f32>(_e52.g0_.x, _e55.g0_.y, _e58.g0_.z) - _e62.g7_), (vec4<f32>(0.0) - _e67.g8_), (vec4<f32>(0.0) - _e72.g9_));
}

fn translator_multi_vector_regressive_product(self_1016: Translator, other_858: MultiVector) -> MultiVector {
    var self_1017: Translator;
    var other_859: MultiVector;

    self_1017 = self_1016;
    other_859 = other_858;
    let _e4: Translator = self_1017;
    let _e8: MultiVector = other_859;
    let _e19: Translator = self_1017;
    let _e23: MultiVector = other_859;
    let _e35: Translator = self_1017;
    let _e39: MultiVector = other_859;
    let _e43: Translator = self_1017;
    let _e47: MultiVector = other_859;
    let _e59: Translator = self_1017;
    let _e63: MultiVector = other_859;
    let _e66: MultiVector = other_859;
    let _e69: MultiVector = other_859;
    let _e80: Translator = self_1017;
    let _e84: MultiVector = other_859;
    let _e87: MultiVector = other_859;
    let _e90: MultiVector = other_859;
    let _e102: Translator = self_1017;
    let _e106: MultiVector = other_859;
    let _e110: Translator = self_1017;
    let _e114: MultiVector = other_859;
    let _e117: MultiVector = other_859;
    let _e120: MultiVector = other_859;
    let _e132: Translator = self_1017;
    let _e136: MultiVector = other_859;
    let _e146: Translator = self_1017;
    let _e150: MultiVector = other_859;
    let _e161: Translator = self_1017;
    let _e165: MultiVector = other_859;
    let _e169: Translator = self_1017;
    let _e173: MultiVector = other_859;
    let _e184: Translator = self_1017;
    let _e188: MultiVector = other_859;
    let _e199: Translator = self_1017;
    let _e203: MultiVector = other_859;
    let _e215: Translator = self_1017;
    let _e219: MultiVector = other_859;
    let _e223: Translator = self_1017;
    let _e227: MultiVector = other_859;
    let _e239: Translator = self_1017;
    let _e243: MultiVector = other_859;
    let _e246: Translator = self_1017;
    let _e250: MultiVector = other_859;
    let _e253: Translator = self_1017;
    let _e256: Translator = self_1017;
    let _e259: Translator = self_1017;
    let _e263: MultiVector = other_859;
    let _e269: Translator = self_1017;
    let _e273: MultiVector = other_859;
    let _e276: Translator = self_1017;
    let _e280: MultiVector = other_859;
    let _e283: Translator = self_1017;
    let _e286: Translator = self_1017;
    let _e289: Translator = self_1017;
    let _e293: MultiVector = other_859;
    let _e299: Translator = self_1017;
    let _e303: MultiVector = other_859;
    let _e306: Translator = self_1017;
    let _e310: MultiVector = other_859;
    return MultiVector((((((vec3<f32>(_e4.g0_.y) * vec3<f32>(_e8.g4_.y)) * vec3<f32>(-(1.0), 0.0, 0.0)) + ((vec3<f32>(_e19.g0_.z) * vec3<f32>(_e23.g4_.z)) * vec3<f32>(-(1.0), 0.0, 0.0))) + (vec3<f32>(_e35.g0_.w) * _e39.g0_)) + ((vec3<f32>(_e43.g0_.x) * vec3<f32>(_e47.g4_.x)) * vec3<f32>(-(1.0), 0.0, 0.0))), (((((vec3<f32>(_e59.g0_.y) * vec3<f32>(_e63.g8_.z, _e66.g8_.z, _e69.g8_.x)) * vec3<f32>(1.0, 0.0, -(1.0))) + ((vec3<f32>(_e80.g0_.z) * vec3<f32>(_e84.g8_.y, _e87.g8_.x, _e90.g8_.y)) * vec3<f32>(-(1.0), 1.0, 0.0))) + (vec3<f32>(_e102.g0_.w) * _e106.g1_)) + ((vec3<f32>(_e110.g0_.x) * vec3<f32>(_e114.g8_.x, _e117.g8_.z, _e120.g8_.y)) * vec3<f32>(0.0, -(1.0), 1.0))), (((((vec2<f32>(_e132.g0_.y) * vec2<f32>(_e136.g6_.y)) * vec2<f32>(0.0, -(1.0))) + ((vec2<f32>(_e146.g0_.z) * vec2<f32>(_e150.g6_.z)) * vec2<f32>(0.0, -(1.0)))) + (vec2<f32>(_e161.g0_.w) * _e165.g2_)) + ((vec2<f32>(_e169.g0_.x) * vec2<f32>(_e173.g6_.x)) * vec2<f32>(0.0, -(1.0)))), (((((vec4<f32>(_e184.g0_.y) * _e188.g9_.zzxz) * vec4<f32>(1.0, 0.0, -(1.0), 0.0)) + ((vec4<f32>(_e199.g0_.z) * _e203.g9_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))) + (vec4<f32>(_e215.g0_.w) * _e219.g3_)) + ((vec4<f32>(_e223.g0_.x) * _e227.g9_.xzyx) * vec4<f32>(0.0, -(1.0), 1.0, 0.0))), (vec3<f32>(_e239.g0_.w) * _e243.g4_), ((vec3<f32>(_e246.g0_.w) * _e250.g5_) + (vec3<f32>(_e253.g0_.x, _e256.g0_.y, _e259.g0_.z) * vec3<f32>(_e263.g0_.y))), (vec3<f32>(_e269.g0_.w) * _e273.g6_), ((vec3<f32>(_e276.g0_.w) * _e280.g7_) + (vec3<f32>(_e283.g0_.x, _e286.g0_.y, _e289.g0_.z) * vec3<f32>(_e293.g0_.z))), (vec4<f32>(_e299.g0_.w) * _e303.g8_), (vec4<f32>(_e306.g0_.w) * _e310.g9_));
}

fn translator_scale(self_1018: Translator, other_860: f32) -> Translator {
    var self_1019: Translator;
    var other_861: f32;

    self_1019 = self_1018;
    other_861 = other_860;
    let _e4: Translator = self_1019;
    let _e5: f32 = other_861;
    let _e7: Translator = translator_scalar_geometric_product(_e4, Scalar(_e5));
    return _e7;
}

fn flector_zero() -> Flector {
    return Flector(vec4<f32>(0.0), vec4<f32>(0.0));
}

fn flector_one() -> Flector {
    return Flector(vec4<f32>(0.0), vec4<f32>(0.0));
}

fn flector_neg(self_1020: Flector) -> Flector {
    var self_1021: Flector;

    self_1021 = self_1020;
    let _e2: Flector = self_1021;
    let _e8: Flector = self_1021;
    return Flector((_e2.g0_ * vec4<f32>(-(1.0))), (_e8.g1_ * vec4<f32>(-(1.0))));
}

fn flector_automorphism(self_1022: Flector) -> Flector {
    var self_1023: Flector;

    self_1023 = self_1022;
    let _e2: Flector = self_1023;
    let _e4: Flector = self_1023;
    return Flector(_e2.g0_, _e4.g1_);
}

fn flector_reversal(self_1024: Flector) -> Flector {
    var self_1025: Flector;

    self_1025 = self_1024;
    let _e2: Flector = self_1025;
    let _e8: Flector = self_1025;
    return Flector((_e2.g0_ * vec4<f32>(-(1.0))), _e8.g1_);
}

fn flector_conjugation(self_1026: Flector) -> Flector {
    var self_1027: Flector;

    self_1027 = self_1026;
    let _e2: Flector = self_1027;
    let _e8: Flector = self_1027;
    return Flector((_e2.g0_ * vec4<f32>(-(1.0))), _e8.g1_);
}

fn flector_scalar_geometric_product(self_1028: Flector, other_862: Scalar) -> Flector {
    var self_1029: Flector;
    var other_863: Scalar;

    self_1029 = self_1028;
    other_863 = other_862;
    let _e4: Flector = self_1029;
    let _e6: Scalar = other_863;
    let _e10: Flector = self_1029;
    let _e12: Scalar = other_863;
    return Flector((_e4.g0_ * vec4<f32>(_e6.g0_)), (_e10.g1_ * vec4<f32>(_e12.g0_)));
}

fn flector_scalar_outer_product(self_1030: Flector, other_864: Scalar) -> Flector {
    var self_1031: Flector;
    var other_865: Scalar;

    self_1031 = self_1030;
    other_865 = other_864;
    let _e4: Flector = self_1031;
    let _e6: Scalar = other_865;
    let _e10: Flector = self_1031;
    let _e12: Scalar = other_865;
    return Flector((_e4.g0_ * vec4<f32>(_e6.g0_)), (_e10.g1_ * vec4<f32>(_e12.g0_)));
}

fn flector_scalar_inner_product(self_1032: Flector, other_866: Scalar) -> Flector {
    var self_1033: Flector;
    var other_867: Scalar;

    self_1033 = self_1032;
    other_867 = other_866;
    let _e4: Flector = self_1033;
    let _e6: Scalar = other_867;
    let _e10: Flector = self_1033;
    let _e12: Scalar = other_867;
    return Flector((_e4.g0_ * vec4<f32>(_e6.g0_)), (_e10.g1_ * vec4<f32>(_e12.g0_)));
}

fn flector_scalar_right_contraction(self_1034: Flector, other_868: Scalar) -> Flector {
    var self_1035: Flector;
    var other_869: Scalar;

    self_1035 = self_1034;
    other_869 = other_868;
    let _e4: Flector = self_1035;
    let _e6: Scalar = other_869;
    let _e10: Flector = self_1035;
    let _e12: Scalar = other_869;
    return Flector((_e4.g0_ * vec4<f32>(_e6.g0_)), (_e10.g1_ * vec4<f32>(_e12.g0_)));
}

fn flector_anti_scalar_regressive_product(self_1036: Flector, other_870: AntiScalar) -> Flector {
    var self_1037: Flector;
    var other_871: AntiScalar;

    self_1037 = self_1036;
    other_871 = other_870;
    let _e4: Flector = self_1037;
    let _e6: AntiScalar = other_871;
    let _e10: Flector = self_1037;
    let _e12: AntiScalar = other_871;
    return Flector((_e4.g0_ * vec4<f32>(_e6.g0_)), (_e10.g1_ * vec4<f32>(_e12.g0_)));
}

fn flector_radial_point_geometric_product(self_1038: Flector, other_872: RadialPoint) -> Motor {
    var self_1039: Flector;
    var other_873: RadialPoint;

    self_1039 = self_1038;
    other_873 = other_872;
    let _e4: Flector = self_1039;
    let _e8: RadialPoint = other_873;
    let _e11: RadialPoint = other_873;
    let _e14: RadialPoint = other_873;
    let _e17: RadialPoint = other_873;
    let _e31: Flector = self_1039;
    let _e35: RadialPoint = other_873;
    let _e38: RadialPoint = other_873;
    let _e41: RadialPoint = other_873;
    let _e44: RadialPoint = other_873;
    let _e57: Flector = self_1039;
    let _e61: RadialPoint = other_873;
    let _e64: RadialPoint = other_873;
    let _e67: RadialPoint = other_873;
    let _e70: RadialPoint = other_873;
    let _e83: Flector = self_1039;
    let _e87: RadialPoint = other_873;
    let _e90: RadialPoint = other_873;
    let _e93: RadialPoint = other_873;
    let _e96: RadialPoint = other_873;
    let _e109: Flector = self_1039;
    let _e112: Flector = self_1039;
    let _e115: Flector = self_1039;
    let _e118: Flector = self_1039;
    let _e122: RadialPoint = other_873;
    let _e128: Flector = self_1039;
    let _e132: RadialPoint = other_873;
    let _e135: RadialPoint = other_873;
    let _e138: RadialPoint = other_873;
    let _e141: RadialPoint = other_873;
    let _e154: Flector = self_1039;
    let _e158: RadialPoint = other_873;
    let _e161: RadialPoint = other_873;
    let _e164: RadialPoint = other_873;
    let _e167: RadialPoint = other_873;
    let _e181: Flector = self_1039;
    let _e185: RadialPoint = other_873;
    let _e188: RadialPoint = other_873;
    let _e191: RadialPoint = other_873;
    let _e194: RadialPoint = other_873;
    let _e206: Flector = self_1039;
    let _e210: RadialPoint = other_873;
    let _e213: RadialPoint = other_873;
    let _e216: RadialPoint = other_873;
    let _e219: RadialPoint = other_873;
    return Motor(((((((vec4<f32>(_e4.g0_.w) * vec4<f32>(_e8.g0_.x, _e11.g0_.y, _e14.g0_.z, _e17.g0_.x)) * vec4<f32>(-(1.0), -(1.0), -(1.0), 0.0)) + ((vec4<f32>(_e31.g1_.x) * vec4<f32>(_e35.g0_.z, _e38.g0_.z, _e41.g0_.y, _e44.g0_.x)) * vec4<f32>(0.0, -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e57.g1_.y) * vec4<f32>(_e61.g0_.z, _e64.g0_.z, _e67.g0_.x, _e70.g0_.y)) * vec4<f32>(1.0, 0.0, -(1.0), 1.0))) + ((vec4<f32>(_e83.g1_.z) * vec4<f32>(_e87.g0_.y, _e90.g0_.x, _e93.g0_.y, _e96.g0_.z)) * vec4<f32>(-(1.0), 1.0, 0.0, 1.0))) + (vec4<f32>(_e109.g0_.x, _e112.g0_.y, _e115.g0_.z, _e118.g1_.w) * vec4<f32>(_e122.g1_.x))), (((((vec4<f32>(_e128.g0_.y) * vec4<f32>(_e132.g0_.z, _e135.g0_.z, _e138.g0_.x, _e141.g0_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e154.g0_.z) * vec4<f32>(_e158.g0_.y, _e161.g0_.x, _e164.g0_.y, _e167.g0_.z)) * vec4<f32>(1.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e181.g1_.w) * vec4<f32>(_e185.g0_.x, _e188.g0_.y, _e191.g0_.z, _e194.g0_.x)) * vec4<f32>(1.0, 1.0, 1.0, 0.0))) + ((vec4<f32>(_e206.g0_.x) * vec4<f32>(_e210.g0_.x, _e213.g0_.z, _e216.g0_.y, _e219.g0_.x)) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))));
}

fn flector_radial_point_regressive_product(self_1040: Flector, other_874: RadialPoint) -> Scalar {
    var self_1041: Flector;
    var other_875: RadialPoint;

    self_1041 = self_1040;
    other_875 = other_874;
    let _e4: Flector = self_1041;
    let _e7: RadialPoint = other_875;
    let _e11: Flector = self_1041;
    let _e14: RadialPoint = other_875;
    let _e19: Flector = self_1041;
    let _e22: RadialPoint = other_875;
    let _e27: Flector = self_1041;
    let _e30: RadialPoint = other_875;
    return Scalar(((((_e4.g1_.x * _e7.g0_.x) + (_e11.g1_.y * _e14.g0_.y)) + (_e19.g1_.z * _e22.g0_.z)) + (_e27.g1_.w * _e30.g1_.x)));
}

fn flector_flat_point_into(self_1042: Flector) -> FlatPoint {
    var self_1043: Flector;

    self_1043 = self_1042;
    let _e2: Flector = self_1043;
    return FlatPoint(_e2.g0_);
}

fn flector_flat_point_add(self_1044: Flector, other_876: FlatPoint) -> Flector {
    var self_1045: Flector;
    var other_877: FlatPoint;

    self_1045 = self_1044;
    other_877 = other_876;
    let _e4: Flector = self_1045;
    let _e6: FlatPoint = other_877;
    let _e9: Flector = self_1045;
    return Flector((_e4.g0_ + _e6.g0_), _e9.g1_);
}

fn flector_flat_point_sub(self_1046: Flector, other_878: FlatPoint) -> Flector {
    var self_1047: Flector;
    var other_879: FlatPoint;

    self_1047 = self_1046;
    other_879 = other_878;
    let _e4: Flector = self_1047;
    let _e6: FlatPoint = other_879;
    let _e9: Flector = self_1047;
    return Flector((_e4.g0_ - _e6.g0_), _e9.g1_);
}

fn flector_dipole_geometric_product(self_1048: Flector, other_880: Dipole) -> Flector {
    var self_1049: Flector;
    var other_881: Dipole;

    self_1049 = self_1048;
    other_881 = other_880;
    let _e4: Flector = self_1049;
    let _e8: Dipole = other_881;
    let _e11: Dipole = other_881;
    let _e14: Dipole = other_881;
    let _e17: Dipole = other_881;
    let _e30: Flector = self_1049;
    let _e34: Dipole = other_881;
    let _e37: Dipole = other_881;
    let _e40: Dipole = other_881;
    let _e43: Dipole = other_881;
    let _e57: Flector = self_1049;
    let _e61: Dipole = other_881;
    let _e74: Flector = self_1049;
    let _e78: Dipole = other_881;
    let _e91: Flector = self_1049;
    let _e95: Dipole = other_881;
    let _e108: Flector = self_1049;
    let _e112: Dipole = other_881;
    let _e115: Dipole = other_881;
    let _e118: Dipole = other_881;
    let _e121: Dipole = other_881;
    let _e133: Flector = self_1049;
    let _e137: Dipole = other_881;
    let _e140: Dipole = other_881;
    let _e143: Dipole = other_881;
    let _e146: Dipole = other_881;
    let _e160: Flector = self_1049;
    let _e164: Dipole = other_881;
    let _e167: Dipole = other_881;
    let _e170: Dipole = other_881;
    let _e173: Dipole = other_881;
    let _e186: Flector = self_1049;
    let _e190: Dipole = other_881;
    let _e193: Dipole = other_881;
    let _e196: Dipole = other_881;
    let _e199: Dipole = other_881;
    let _e213: Flector = self_1049;
    let _e217: Dipole = other_881;
    let _e220: Dipole = other_881;
    let _e223: Dipole = other_881;
    let _e226: Dipole = other_881;
    let _e238: Flector = self_1049;
    let _e242: Dipole = other_881;
    let _e245: Dipole = other_881;
    let _e248: Dipole = other_881;
    let _e251: Dipole = other_881;
    let _e264: Flector = self_1049;
    let _e268: Dipole = other_881;
    let _e271: Dipole = other_881;
    let _e274: Dipole = other_881;
    let _e277: Dipole = other_881;
    let _e290: Flector = self_1049;
    let _e294: Dipole = other_881;
    let _e297: Dipole = other_881;
    let _e300: Dipole = other_881;
    let _e303: Dipole = other_881;
    let _e316: Flector = self_1049;
    let _e320: Dipole = other_881;
    let _e323: Dipole = other_881;
    let _e326: Dipole = other_881;
    let _e329: Dipole = other_881;
    let _e341: Flector = self_1049;
    let _e345: Dipole = other_881;
    let _e348: Dipole = other_881;
    let _e351: Dipole = other_881;
    let _e354: Dipole = other_881;
    return Flector(((((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g1_.z, _e11.g1_.z, _e14.g1_.x, _e17.g0_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e30.g0_.z) * vec4<f32>(_e34.g1_.y, _e37.g1_.x, _e40.g1_.y, _e43.g0_.z)) * vec4<f32>(1.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e57.g1_.x) * vec4<f32>(_e61.g1_.x)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e74.g1_.y) * vec4<f32>(_e78.g1_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e91.g1_.z) * vec4<f32>(_e95.g1_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e108.g1_.w) * vec4<f32>(_e112.g1_.x, _e115.g1_.y, _e118.g1_.z, _e121.g1_.x)) * vec4<f32>(1.0, 1.0, 1.0, 0.0))) + ((vec4<f32>(_e133.g0_.x) * vec4<f32>(_e137.g1_.x, _e140.g1_.z, _e143.g1_.y, _e146.g0_.x)) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))), (((((((((vec4<f32>(_e160.g0_.y) * vec4<f32>(_e164.g0_.z, _e167.g0_.z, _e170.g0_.x, _e173.g1_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e186.g0_.z) * vec4<f32>(_e190.g0_.y, _e193.g0_.x, _e196.g0_.y, _e199.g1_.z)) * vec4<f32>(1.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e213.g0_.w) * vec4<f32>(_e217.g1_.x, _e220.g1_.y, _e223.g1_.z, _e226.g1_.x)) * vec4<f32>(1.0, 1.0, 1.0, 0.0))) + ((vec4<f32>(_e238.g1_.x) * vec4<f32>(_e242.g1_.z, _e245.g1_.z, _e248.g1_.y, _e251.g1_.z)) * vec4<f32>(0.0, 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e264.g1_.y) * vec4<f32>(_e268.g1_.z, _e271.g1_.z, _e274.g1_.x, _e277.g1_.z)) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0))) + ((vec4<f32>(_e290.g1_.z) * vec4<f32>(_e294.g1_.y, _e297.g1_.x, _e300.g1_.y, _e303.g1_.y)) * vec4<f32>(1.0, -(1.0), 0.0, 0.0))) + ((vec4<f32>(_e316.g1_.w) * vec4<f32>(_e320.g0_.x, _e323.g0_.y, _e326.g0_.z, _e329.g0_.x)) * vec4<f32>(1.0, 1.0, 1.0, 0.0))) + ((vec4<f32>(_e341.g0_.x) * vec4<f32>(_e345.g0_.x, _e348.g0_.z, _e351.g0_.y, _e354.g1_.x)) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))));
}

fn flector_dipole_regressive_product(self_1050: Flector, other_882: Dipole) -> RadialPoint {
    var self_1051: Flector;
    var other_883: Dipole;

    self_1051 = self_1050;
    other_883 = other_882;
    let _e4: Flector = self_1051;
    let _e8: Dipole = other_883;
    let _e18: Flector = self_1051;
    let _e22: Dipole = other_883;
    let _e33: Flector = self_1051;
    let _e37: Dipole = other_883;
    let _e41: Flector = self_1051;
    let _e45: Dipole = other_883;
    let _e56: Flector = self_1051;
    let _e60: Dipole = other_883;
    let _e63: Dipole = other_883;
    let _e73: Flector = self_1051;
    let _e77: Dipole = other_883;
    let _e80: Dipole = other_883;
    let _e91: Flector = self_1051;
    let _e95: Dipole = other_883;
    let _e98: Dipole = other_883;
    let _e109: Flector = self_1051;
    let _e112: Flector = self_1051;
    let _e116: Dipole = other_883;
    let _e119: Dipole = other_883;
    return RadialPoint((((((vec3<f32>(_e4.g1_.y) * _e8.g1_.zzx) * vec3<f32>(1.0, 0.0, -(1.0))) + ((vec3<f32>(_e18.g1_.z) * _e22.g1_.yxy) * vec3<f32>(-(1.0), 1.0, 0.0))) - (vec3<f32>(_e33.g1_.w) * _e37.g0_)) + ((vec3<f32>(_e41.g1_.x) * _e45.g1_.xzy) * vec3<f32>(0.0, -(1.0), 1.0))), (((((vec2<f32>(_e56.g1_.x) * vec2<f32>(_e60.g0_.x, _e63.g2_.x)) * vec2<f32>(1.0, -(1.0))) + ((vec2<f32>(_e73.g1_.y) * vec2<f32>(_e77.g0_.y, _e80.g2_.y)) * vec2<f32>(1.0, -(1.0)))) + ((vec2<f32>(_e91.g1_.z) * vec2<f32>(_e95.g0_.z, _e98.g2_.z)) * vec2<f32>(1.0, -(1.0)))) + ((vec2<f32>(_e109.g1_.x, _e112.g1_.w) * vec2<f32>(_e116.g2_.x, _e119.g2_.w)) * vec2<f32>(0.0, -(1.0)))));
}

fn flector_dipole_outer_product(self_1052: Flector, other_884: Dipole) -> Plane {
    var self_1053: Flector;
    var other_885: Dipole;

    self_1053 = self_1052;
    other_885 = other_884;
    let _e4: Flector = self_1053;
    let _e8: Dipole = other_885;
    let _e11: Dipole = other_885;
    let _e14: Dipole = other_885;
    let _e17: Dipole = other_885;
    let _e30: Flector = self_1053;
    let _e34: Dipole = other_885;
    let _e37: Dipole = other_885;
    let _e40: Dipole = other_885;
    let _e43: Dipole = other_885;
    let _e57: Flector = self_1053;
    let _e61: Dipole = other_885;
    let _e64: Dipole = other_885;
    let _e67: Dipole = other_885;
    let _e70: Dipole = other_885;
    let _e82: Flector = self_1053;
    let _e86: Dipole = other_885;
    let _e89: Dipole = other_885;
    let _e92: Dipole = other_885;
    let _e95: Dipole = other_885;
    return Plane((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.z, _e11.g0_.z, _e14.g0_.x, _e17.g1_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e30.g0_.z) * vec4<f32>(_e34.g0_.y, _e37.g0_.x, _e40.g0_.y, _e43.g1_.z)) * vec4<f32>(1.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e57.g0_.w) * vec4<f32>(_e61.g1_.x, _e64.g1_.y, _e67.g1_.z, _e70.g1_.x)) * vec4<f32>(1.0, 1.0, 1.0, 0.0))) + ((vec4<f32>(_e82.g0_.x) * vec4<f32>(_e86.g0_.x, _e89.g0_.z, _e92.g0_.y, _e95.g1_.x)) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))));
}

fn flector_dipole_inner_product(self_1054: Flector, other_886: Dipole) -> FlatPoint {
    var self_1055: Flector;
    var other_887: Dipole;

    self_1055 = self_1054;
    other_887 = other_886;
    let _e4: Flector = self_1055;
    let _e8: Dipole = other_887;
    let _e20: Flector = self_1055;
    let _e24: Dipole = other_887;
    let _e37: Flector = self_1055;
    let _e40: Dipole = other_887;
    let _e43: Dipole = other_887;
    let _e46: Dipole = other_887;
    let _e49: Dipole = other_887;
    return FlatPoint(((((vec4<f32>(_e4.g1_.y) * vec4<f32>(_e8.g1_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0))) + ((vec4<f32>(_e20.g1_.z) * vec4<f32>(_e24.g1_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((_e37.g1_.wwwx * vec4<f32>(_e40.g1_.x, _e43.g1_.y, _e46.g1_.z, _e49.g1_.x)) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))));
}

fn flector_dipole_right_contraction(self_1056: Flector, other_888: Dipole) -> FlatPoint {
    var self_1057: Flector;
    var other_889: Dipole;

    self_1057 = self_1056;
    other_889 = other_888;
    let _e4: Flector = self_1057;
    let _e8: Dipole = other_889;
    let _e20: Flector = self_1057;
    let _e24: Dipole = other_889;
    let _e37: Flector = self_1057;
    let _e40: Dipole = other_889;
    let _e43: Dipole = other_889;
    let _e46: Dipole = other_889;
    let _e49: Dipole = other_889;
    return FlatPoint(((((vec4<f32>(_e4.g1_.y) * vec4<f32>(_e8.g1_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0))) + ((vec4<f32>(_e20.g1_.z) * vec4<f32>(_e24.g1_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((_e37.g1_.wwwx * vec4<f32>(_e40.g1_.x, _e43.g1_.y, _e46.g1_.z, _e49.g1_.x)) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))));
}

fn flector_line_regressive_product(self_1058: Flector, other_890: Line) -> FlatPoint {
    var self_1059: Flector;
    var other_891: Line;

    self_1059 = self_1058;
    other_891 = other_890;
    let _e4: Flector = self_1059;
    let _e8: Line = other_891;
    let _e11: Line = other_891;
    let _e14: Line = other_891;
    let _e17: Line = other_891;
    let _e30: Flector = self_1059;
    let _e34: Line = other_891;
    let _e37: Line = other_891;
    let _e40: Line = other_891;
    let _e43: Line = other_891;
    let _e57: Flector = self_1059;
    let _e61: Line = other_891;
    let _e64: Line = other_891;
    let _e67: Line = other_891;
    let _e70: Line = other_891;
    let _e82: Flector = self_1059;
    let _e86: Line = other_891;
    let _e89: Line = other_891;
    let _e92: Line = other_891;
    let _e95: Line = other_891;
    return FlatPoint((((((vec4<f32>(_e4.g1_.y) * vec4<f32>(_e8.g1_.z, _e11.g1_.z, _e14.g1_.x, _e17.g0_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e30.g1_.z) * vec4<f32>(_e34.g1_.y, _e37.g1_.x, _e40.g1_.y, _e43.g0_.z)) * vec4<f32>(1.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e57.g1_.w) * vec4<f32>(_e61.g0_.x, _e64.g0_.y, _e67.g0_.z, _e70.g0_.x)) * vec4<f32>(1.0, 1.0, 1.0, 0.0))) + ((vec4<f32>(_e82.g1_.x) * vec4<f32>(_e86.g1_.x, _e89.g1_.z, _e92.g1_.y, _e95.g0_.x)) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))));
}

fn flector_circle_geometric_product(self_1060: Flector, other_892: Circle) -> Motor {
    var self_1061: Flector;
    var other_893: Circle;

    self_1061 = self_1060;
    other_893 = other_892;
    let _e4: Flector = self_1061;
    let _e8: Circle = other_893;
    let _e20: Flector = self_1061;
    let _e24: Circle = other_893;
    let _e37: Flector = self_1061;
    let _e41: Circle = other_893;
    let _e54: Flector = self_1061;
    let _e58: Circle = other_893;
    let _e71: Flector = self_1061;
    let _e75: Circle = other_893;
    let _e88: Flector = self_1061;
    let _e92: Circle = other_893;
    let _e103: Flector = self_1061;
    let _e106: Flector = self_1061;
    let _e109: Flector = self_1061;
    let _e112: Flector = self_1061;
    let _e116: Circle = other_893;
    let _e130: Flector = self_1061;
    let _e133: Flector = self_1061;
    let _e136: Flector = self_1061;
    let _e139: Flector = self_1061;
    let _e143: Circle = other_893;
    return Motor(((((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.zzxy) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e20.g0_.z) * _e24.g0_.yxyz) * vec4<f32>(1.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e37.g0_.w) * vec4<f32>(_e41.g0_.w)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e54.g1_.y) * vec4<f32>(_e58.g0_.w)) * vec4<f32>(0.0, -(1.0), 0.0, 0.0))) + ((vec4<f32>(_e71.g1_.z) * vec4<f32>(_e75.g0_.w)) * vec4<f32>(0.0, 0.0, -(1.0), 0.0))) + ((vec4<f32>(_e88.g1_.w) * _e92.g0_.xyzx) * vec4<f32>(1.0, 1.0, 1.0, 0.0))) + ((vec4<f32>(_e103.g1_.x, _e106.g0_.x, _e109.g0_.x, _e112.g0_.x) * _e116.g0_.wzyx) * vec4<f32>(-(1.0), 1.0, -(1.0), -(1.0)))), (vec4<f32>(_e130.g0_.x, _e133.g0_.y, _e136.g0_.z, _e139.g1_.w) * vec4<f32>(_e143.g0_.w)));
}

fn flector_circle_outer_product(self_1062: Flector, other_894: Circle) -> AntiScalar {
    var self_1063: Flector;
    var other_895: Circle;

    self_1063 = self_1062;
    other_895 = other_894;
    let _e5: Flector = self_1063;
    let _e8: Circle = other_895;
    let _e13: Flector = self_1063;
    let _e16: Circle = other_895;
    let _e21: Flector = self_1063;
    let _e24: Circle = other_895;
    let _e29: Flector = self_1063;
    let _e32: Circle = other_895;
    return AntiScalar(((((0.0 - (_e5.g0_.x * _e8.g0_.x)) - (_e13.g0_.y * _e16.g0_.y)) - (_e21.g0_.z * _e24.g0_.z)) - (_e29.g0_.w * _e32.g0_.w)));
}

fn flector_plane_into(self_1064: Flector) -> Plane {
    var self_1065: Flector;

    self_1065 = self_1064;
    let _e2: Flector = self_1065;
    return Plane(_e2.g1_);
}

fn flector_plane_add(self_1066: Flector, other_896: Plane) -> Flector {
    var self_1067: Flector;
    var other_897: Plane;

    self_1067 = self_1066;
    other_897 = other_896;
    let _e4: Flector = self_1067;
    let _e6: Flector = self_1067;
    let _e8: Plane = other_897;
    return Flector(_e4.g0_, (_e6.g1_ + _e8.g0_));
}

fn flector_plane_sub(self_1068: Flector, other_898: Plane) -> Flector {
    var self_1069: Flector;
    var other_899: Plane;

    self_1069 = self_1068;
    other_899 = other_898;
    let _e4: Flector = self_1069;
    let _e6: Flector = self_1069;
    let _e8: Plane = other_899;
    return Flector(_e4.g0_, (_e6.g1_ - _e8.g0_));
}

fn flector_motor_regressive_product(self_1070: Flector, other_900: Motor) -> Flector {
    var self_1071: Flector;
    var other_901: Motor;

    self_1071 = self_1070;
    other_901 = other_900;
    let _e4: Flector = self_1071;
    let _e8: Motor = other_901;
    let _e11: Motor = other_901;
    let _e14: Motor = other_901;
    let _e17: Motor = other_901;
    let _e30: Flector = self_1071;
    let _e34: Motor = other_901;
    let _e37: Motor = other_901;
    let _e40: Motor = other_901;
    let _e43: Motor = other_901;
    let _e57: Flector = self_1071;
    let _e61: Motor = other_901;
    let _e64: Motor = other_901;
    let _e67: Motor = other_901;
    let _e70: Motor = other_901;
    let _e84: Flector = self_1071;
    let _e88: Motor = other_901;
    let _e99: Flector = self_1071;
    let _e101: Motor = other_901;
    let _e107: Flector = self_1071;
    let _e109: Motor = other_901;
    return Flector(((((((vec4<f32>(_e4.g1_.x) * vec4<f32>(_e8.g1_.z, _e11.g1_.z, _e14.g1_.y, _e17.g0_.x)) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0))) + ((vec4<f32>(_e30.g1_.y) * vec4<f32>(_e34.g1_.z, _e37.g1_.z, _e40.g1_.x, _e43.g0_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e57.g1_.z) * vec4<f32>(_e61.g1_.y, _e64.g1_.x, _e67.g1_.y, _e70.g0_.z)) * vec4<f32>(1.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e84.g1_.w) * _e88.g0_.xyzx) * vec4<f32>(1.0, 1.0, 1.0, 0.0))) + (_e99.g0_ * vec4<f32>(_e101.g0_.w))), (_e107.g1_ * vec4<f32>(_e109.g0_.w)));
}

fn flector_rotor_regressive_product(self_1072: Flector, other_902: Rotor) -> Flector {
    var self_1073: Flector;
    var other_903: Rotor;

    self_1073 = self_1072;
    other_903 = other_902;
    let _e4: Flector = self_1073;
    let _e8: Rotor = other_903;
    let _e20: Flector = self_1073;
    let _e24: Rotor = other_903;
    let _e37: Flector = self_1073;
    let _e41: Rotor = other_903;
    let _e54: Flector = self_1073;
    let _e58: Rotor = other_903;
    let _e69: Flector = self_1073;
    let _e71: Rotor = other_903;
    let _e77: Flector = self_1073;
    let _e79: Rotor = other_903;
    return Flector(((((((vec4<f32>(_e4.g1_.x) * vec4<f32>(_e8.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0))) + ((vec4<f32>(_e20.g1_.y) * vec4<f32>(_e24.g0_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e37.g1_.z) * vec4<f32>(_e41.g0_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e54.g1_.w) * _e58.g0_.xyzx) * vec4<f32>(1.0, 1.0, 1.0, 0.0))) + (_e69.g0_ * vec4<f32>(_e71.g0_.w))), (_e77.g1_ * vec4<f32>(_e79.g0_.w)));
}

fn flector_translator_regressive_product(self_1074: Flector, other_904: Translator) -> Flector {
    var self_1075: Flector;
    var other_905: Translator;

    self_1075 = self_1074;
    other_905 = other_904;
    let _e4: Flector = self_1075;
    let _e8: Translator = other_905;
    let _e19: Flector = self_1075;
    let _e23: Translator = other_905;
    let _e35: Flector = self_1075;
    let _e39: Translator = other_905;
    let _e51: Flector = self_1075;
    let _e53: Translator = other_905;
    let _e59: Flector = self_1075;
    let _e61: Translator = other_905;
    return Flector((((((vec4<f32>(_e4.g1_.x) * _e8.g0_.zzyz) * vec4<f32>(0.0, 1.0, -(1.0), 0.0)) + ((vec4<f32>(_e19.g1_.y) * _e23.g0_.zzxz) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0))) + ((vec4<f32>(_e35.g1_.z) * _e39.g0_.yxyy) * vec4<f32>(1.0, -(1.0), 0.0, 0.0))) + (_e51.g0_ * vec4<f32>(_e53.g0_.w))), (_e59.g1_ * vec4<f32>(_e61.g0_.w)));
}

fn flector_flector_add(self_1076: Flector, other_906: Flector) -> Flector {
    var self_1077: Flector;
    var other_907: Flector;

    self_1077 = self_1076;
    other_907 = other_906;
    let _e4: Flector = self_1077;
    let _e6: Flector = other_907;
    let _e9: Flector = self_1077;
    let _e11: Flector = other_907;
    return Flector((_e4.g0_ + _e6.g0_), (_e9.g1_ + _e11.g1_));
}

fn flector_flector_sub(self_1078: Flector, other_908: Flector) -> Flector {
    var self_1079: Flector;
    var other_909: Flector;

    self_1079 = self_1078;
    other_909 = other_908;
    let _e4: Flector = self_1079;
    let _e6: Flector = other_909;
    let _e9: Flector = self_1079;
    let _e11: Flector = other_909;
    return Flector((_e4.g0_ - _e6.g0_), (_e9.g1_ - _e11.g1_));
}

fn flector_flector_mul(self_1080: Flector, other_910: Flector) -> Flector {
    var self_1081: Flector;
    var other_911: Flector;

    self_1081 = self_1080;
    other_911 = other_910;
    let _e4: Flector = self_1081;
    let _e6: Flector = other_911;
    let _e9: Flector = self_1081;
    let _e11: Flector = other_911;
    return Flector((_e4.g0_ * _e6.g0_), (_e9.g1_ * _e11.g1_));
}

fn flector_flector_div(self_1082: Flector, other_912: Flector) -> Flector {
    var self_1083: Flector;
    var other_913: Flector;

    self_1083 = self_1082;
    other_913 = other_912;
    let _e4: Flector = self_1083;
    let _e7: Flector = self_1083;
    let _e10: Flector = self_1083;
    let _e13: Flector = self_1083;
    let _e23: Flector = other_913;
    let _e26: Flector = other_913;
    let _e29: Flector = other_913;
    let _e32: Flector = other_913;
    let _e43: Flector = self_1083;
    let _e46: Flector = self_1083;
    let _e49: Flector = self_1083;
    let _e52: Flector = self_1083;
    let _e62: Flector = other_913;
    let _e65: Flector = other_913;
    let _e68: Flector = other_913;
    let _e71: Flector = other_913;
    return Flector((((vec4<f32>(_e4.g0_.x, _e7.g0_.y, _e10.g0_.z, _e13.g0_.w) * vec4<f32>(1.0, 1.0, 1.0, 1.0)) / vec4<f32>(_e23.g0_.x, _e26.g0_.y, _e29.g0_.z, _e32.g0_.w)) * vec4<f32>(1.0, 1.0, 1.0, 1.0)), (((vec4<f32>(_e43.g1_.x, _e46.g1_.y, _e49.g1_.z, _e52.g1_.w) * vec4<f32>(1.0, 1.0, 1.0, 1.0)) / vec4<f32>(_e62.g1_.x, _e65.g1_.y, _e68.g1_.z, _e71.g1_.w)) * vec4<f32>(1.0, 1.0, 1.0, 1.0)));
}

fn flector_multi_vector_add(self_1084: Flector, other_914: MultiVector) -> MultiVector {
    var self_1085: Flector;
    var other_915: MultiVector;

    self_1085 = self_1084;
    other_915 = other_914;
    let _e4: MultiVector = other_915;
    let _e6: MultiVector = other_915;
    let _e8: MultiVector = other_915;
    let _e10: Flector = self_1085;
    let _e12: MultiVector = other_915;
    let _e15: MultiVector = other_915;
    let _e17: MultiVector = other_915;
    let _e19: MultiVector = other_915;
    let _e21: MultiVector = other_915;
    let _e23: MultiVector = other_915;
    let _e25: Flector = self_1085;
    let _e27: MultiVector = other_915;
    return MultiVector(_e4.g0_, _e6.g1_, _e8.g2_, (_e10.g0_ + _e12.g3_), _e15.g4_, _e17.g5_, _e19.g6_, _e21.g7_, _e23.g8_, (_e25.g1_ + _e27.g9_));
}

fn flector_multi_vector_sub(self_1086: Flector, other_916: MultiVector) -> MultiVector {
    var self_1087: Flector;
    var other_917: MultiVector;

    self_1087 = self_1086;
    other_917 = other_916;
    let _e6: MultiVector = other_917;
    let _e11: MultiVector = other_917;
    let _e16: MultiVector = other_917;
    let _e19: Flector = self_1087;
    let _e21: MultiVector = other_917;
    let _e26: MultiVector = other_917;
    let _e31: MultiVector = other_917;
    let _e36: MultiVector = other_917;
    let _e41: MultiVector = other_917;
    let _e46: MultiVector = other_917;
    let _e49: Flector = self_1087;
    let _e51: MultiVector = other_917;
    return MultiVector((vec3<f32>(0.0) - _e6.g0_), (vec3<f32>(0.0) - _e11.g1_), (vec2<f32>(0.0) - _e16.g2_), (_e19.g0_ - _e21.g3_), (vec3<f32>(0.0) - _e26.g4_), (vec3<f32>(0.0) - _e31.g5_), (vec3<f32>(0.0) - _e36.g6_), (vec3<f32>(0.0) - _e41.g7_), (vec4<f32>(0.0) - _e46.g8_), (_e49.g1_ - _e51.g9_));
}

fn flector_scale(self_1088: Flector, other_918: f32) -> Flector {
    var self_1089: Flector;
    var other_919: f32;

    self_1089 = self_1088;
    other_919 = other_918;
    let _e4: Flector = self_1089;
    let _e5: f32 = other_919;
    let _e7: Flector = flector_scalar_geometric_product(_e4, Scalar(_e5));
    return _e7;
}

fn multi_vector_zero() -> MultiVector {
    return MultiVector(vec3<f32>(0.0), vec3<f32>(0.0), vec2<f32>(0.0), vec4<f32>(0.0), vec3<f32>(0.0), vec3<f32>(0.0), vec3<f32>(0.0), vec3<f32>(0.0), vec4<f32>(0.0), vec4<f32>(0.0));
}

fn multi_vector_one() -> MultiVector {
    return MultiVector(vec3<f32>(1.0, 0.0, 0.0), vec3<f32>(0.0), vec2<f32>(0.0), vec4<f32>(0.0), vec3<f32>(0.0), vec3<f32>(0.0), vec3<f32>(0.0), vec3<f32>(0.0), vec4<f32>(0.0), vec4<f32>(0.0));
}

fn multi_vector_neg(self_1090: MultiVector) -> MultiVector {
    var self_1091: MultiVector;

    self_1091 = self_1090;
    let _e2: MultiVector = self_1091;
    let _e8: MultiVector = self_1091;
    let _e14: MultiVector = self_1091;
    let _e20: MultiVector = self_1091;
    let _e26: MultiVector = self_1091;
    let _e32: MultiVector = self_1091;
    let _e38: MultiVector = self_1091;
    let _e44: MultiVector = self_1091;
    let _e50: MultiVector = self_1091;
    let _e56: MultiVector = self_1091;
    return MultiVector((_e2.g0_ * vec3<f32>(-(1.0))), (_e8.g1_ * vec3<f32>(-(1.0))), (_e14.g2_ * vec2<f32>(-(1.0))), (_e20.g3_ * vec4<f32>(-(1.0))), (_e26.g4_ * vec3<f32>(-(1.0))), (_e32.g5_ * vec3<f32>(-(1.0))), (_e38.g6_ * vec3<f32>(-(1.0))), (_e44.g7_ * vec3<f32>(-(1.0))), (_e50.g8_ * vec4<f32>(-(1.0))), (_e56.g9_ * vec4<f32>(-(1.0))));
}

fn multi_vector_automorphism(self_1092: MultiVector) -> MultiVector {
    var self_1093: MultiVector;

    self_1093 = self_1092;
    let _e2: MultiVector = self_1093;
    let _e10: MultiVector = self_1093;
    let _e16: MultiVector = self_1093;
    let _e22: MultiVector = self_1093;
    let _e24: MultiVector = self_1093;
    let _e26: MultiVector = self_1093;
    let _e28: MultiVector = self_1093;
    let _e34: MultiVector = self_1093;
    let _e40: MultiVector = self_1093;
    let _e46: MultiVector = self_1093;
    return MultiVector((_e2.g0_ * vec3<f32>(1.0, 1.0, -(1.0))), (_e10.g1_ * vec3<f32>(-(1.0))), (_e16.g2_ * vec2<f32>(-(1.0))), _e22.g3_, _e24.g4_, _e26.g5_, (_e28.g6_ * vec3<f32>(-(1.0))), (_e34.g7_ * vec3<f32>(-(1.0))), (_e40.g8_ * vec4<f32>(-(1.0))), _e46.g9_);
}

fn multi_vector_reversal(self_1094: MultiVector) -> MultiVector {
    var self_1095: MultiVector;

    self_1095 = self_1094;
    let _e2: MultiVector = self_1095;
    let _e4: MultiVector = self_1095;
    let _e6: MultiVector = self_1095;
    let _e8: MultiVector = self_1095;
    let _e14: MultiVector = self_1095;
    let _e20: MultiVector = self_1095;
    let _e26: MultiVector = self_1095;
    let _e32: MultiVector = self_1095;
    let _e38: MultiVector = self_1095;
    let _e44: MultiVector = self_1095;
    return MultiVector(_e2.g0_, _e4.g1_, _e6.g2_, (_e8.g3_ * vec4<f32>(-(1.0))), (_e14.g4_ * vec3<f32>(-(1.0))), (_e20.g5_ * vec3<f32>(-(1.0))), (_e26.g6_ * vec3<f32>(-(1.0))), (_e32.g7_ * vec3<f32>(-(1.0))), (_e38.g8_ * vec4<f32>(-(1.0))), _e44.g9_);
}

fn multi_vector_conjugation(self_1096: MultiVector) -> MultiVector {
    var self_1097: MultiVector;

    self_1097 = self_1096;
    let _e2: MultiVector = self_1097;
    let _e10: MultiVector = self_1097;
    let _e16: MultiVector = self_1097;
    let _e22: MultiVector = self_1097;
    let _e28: MultiVector = self_1097;
    let _e34: MultiVector = self_1097;
    let _e40: MultiVector = self_1097;
    let _e42: MultiVector = self_1097;
    let _e44: MultiVector = self_1097;
    let _e46: MultiVector = self_1097;
    return MultiVector((_e2.g0_ * vec3<f32>(1.0, 1.0, -(1.0))), (_e10.g1_ * vec3<f32>(-(1.0))), (_e16.g2_ * vec2<f32>(-(1.0))), (_e22.g3_ * vec4<f32>(-(1.0))), (_e28.g4_ * vec3<f32>(-(1.0))), (_e34.g5_ * vec3<f32>(-(1.0))), _e40.g6_, _e42.g7_, _e44.g8_, _e46.g9_);
}

fn multi_vector_dual(self_1098: MultiVector) -> MultiVector {
    var self_1099: MultiVector;

    self_1099 = self_1098;
    let _e2: MultiVector = self_1099;
    let _e5: MultiVector = self_1099;
    let _e8: MultiVector = self_1099;
    let _e12: MultiVector = self_1099;
    let _e15: MultiVector = self_1099;
    let _e18: MultiVector = self_1099;
    let _e22: MultiVector = self_1099;
    let _e25: MultiVector = self_1099;
    let _e29: MultiVector = self_1099;
    let _e35: MultiVector = self_1099;
    let _e41: MultiVector = self_1099;
    let _e47: MultiVector = self_1099;
    let _e53: MultiVector = self_1099;
    let _e59: MultiVector = self_1099;
    let _e65: MultiVector = self_1099;
    let _e68: MultiVector = self_1099;
    let _e71: MultiVector = self_1099;
    let _e74: MultiVector = self_1099;
    return MultiVector(vec3<f32>(_e2.g0_.z, _e5.g2_.y, _e8.g0_.x), vec3<f32>(_e12.g9_.x, _e15.g9_.y, _e18.g9_.z), vec2<f32>(_e22.g9_.w, _e25.g0_.y), (_e29.g8_ * vec4<f32>(-(1.0))), (_e35.g7_ * vec3<f32>(-(1.0))), (_e41.g6_ * vec3<f32>(-(1.0))), (_e47.g5_ * vec3<f32>(-(1.0))), (_e53.g4_ * vec3<f32>(-(1.0))), (_e59.g3_ * vec4<f32>(-(1.0))), vec4<f32>(_e65.g1_.x, _e68.g1_.y, _e71.g1_.z, _e74.g2_.x));
}

fn multi_vector_scalar_into(self_1100: MultiVector) -> Scalar {
    var self_1101: MultiVector;

    self_1101 = self_1100;
    let _e2: MultiVector = self_1101;
    return Scalar(_e2.g0_.x);
}

fn multi_vector_scalar_add(self_1102: MultiVector, other_920: Scalar) -> MultiVector {
    var self_1103: MultiVector;
    var other_921: Scalar;

    self_1103 = self_1102;
    other_921 = other_920;
    let _e4: MultiVector = self_1103;
    let _e6: Scalar = other_921;
    let _e15: MultiVector = self_1103;
    let _e17: MultiVector = self_1103;
    let _e19: MultiVector = self_1103;
    let _e21: MultiVector = self_1103;
    let _e23: MultiVector = self_1103;
    let _e25: MultiVector = self_1103;
    let _e27: MultiVector = self_1103;
    let _e29: MultiVector = self_1103;
    let _e31: MultiVector = self_1103;
    return MultiVector((_e4.g0_ + (vec3<f32>(_e6.g0_) * vec3<f32>(1.0, 0.0, 0.0))), _e15.g1_, _e17.g2_, _e19.g3_, _e21.g4_, _e23.g5_, _e25.g6_, _e27.g7_, _e29.g8_, _e31.g9_);
}

fn multi_vector_scalar_sub(self_1104: MultiVector, other_922: Scalar) -> MultiVector {
    var self_1105: MultiVector;
    var other_923: Scalar;

    self_1105 = self_1104;
    other_923 = other_922;
    let _e4: MultiVector = self_1105;
    let _e6: Scalar = other_923;
    let _e15: MultiVector = self_1105;
    let _e17: MultiVector = self_1105;
    let _e19: MultiVector = self_1105;
    let _e21: MultiVector = self_1105;
    let _e23: MultiVector = self_1105;
    let _e25: MultiVector = self_1105;
    let _e27: MultiVector = self_1105;
    let _e29: MultiVector = self_1105;
    let _e31: MultiVector = self_1105;
    return MultiVector((_e4.g0_ - (vec3<f32>(_e6.g0_) * vec3<f32>(1.0, 0.0, 0.0))), _e15.g1_, _e17.g2_, _e19.g3_, _e21.g4_, _e23.g5_, _e25.g6_, _e27.g7_, _e29.g8_, _e31.g9_);
}

fn multi_vector_scalar_geometric_product(self_1106: MultiVector, other_924: Scalar) -> MultiVector {
    var self_1107: MultiVector;
    var other_925: Scalar;

    self_1107 = self_1106;
    other_925 = other_924;
    let _e4: MultiVector = self_1107;
    let _e6: Scalar = other_925;
    let _e10: MultiVector = self_1107;
    let _e12: Scalar = other_925;
    let _e16: MultiVector = self_1107;
    let _e18: Scalar = other_925;
    let _e22: MultiVector = self_1107;
    let _e24: Scalar = other_925;
    let _e28: MultiVector = self_1107;
    let _e30: Scalar = other_925;
    let _e34: MultiVector = self_1107;
    let _e36: Scalar = other_925;
    let _e40: MultiVector = self_1107;
    let _e42: Scalar = other_925;
    let _e46: MultiVector = self_1107;
    let _e48: Scalar = other_925;
    let _e52: MultiVector = self_1107;
    let _e54: Scalar = other_925;
    let _e58: MultiVector = self_1107;
    let _e60: Scalar = other_925;
    return MultiVector((_e4.g0_ * vec3<f32>(_e6.g0_)), (_e10.g1_ * vec3<f32>(_e12.g0_)), (_e16.g2_ * vec2<f32>(_e18.g0_)), (_e22.g3_ * vec4<f32>(_e24.g0_)), (_e28.g4_ * vec3<f32>(_e30.g0_)), (_e34.g5_ * vec3<f32>(_e36.g0_)), (_e40.g6_ * vec3<f32>(_e42.g0_)), (_e46.g7_ * vec3<f32>(_e48.g0_)), (_e52.g8_ * vec4<f32>(_e54.g0_)), (_e58.g9_ * vec4<f32>(_e60.g0_)));
}

fn multi_vector_scalar_regressive_product(self_1108: MultiVector, other_926: Scalar) -> Scalar {
    var self_1109: MultiVector;
    var other_927: Scalar;

    self_1109 = self_1108;
    other_927 = other_926;
    let _e4: MultiVector = self_1109;
    let _e7: Scalar = other_927;
    return Scalar((_e4.g0_.z * _e7.g0_));
}

fn multi_vector_scalar_outer_product(self_1110: MultiVector, other_928: Scalar) -> MultiVector {
    var self_1111: MultiVector;
    var other_929: Scalar;

    self_1111 = self_1110;
    other_929 = other_928;
    let _e4: MultiVector = self_1111;
    let _e6: Scalar = other_929;
    let _e10: MultiVector = self_1111;
    let _e12: Scalar = other_929;
    let _e16: MultiVector = self_1111;
    let _e18: Scalar = other_929;
    let _e22: MultiVector = self_1111;
    let _e24: Scalar = other_929;
    let _e28: MultiVector = self_1111;
    let _e30: Scalar = other_929;
    let _e34: MultiVector = self_1111;
    let _e36: Scalar = other_929;
    let _e40: MultiVector = self_1111;
    let _e42: Scalar = other_929;
    let _e46: MultiVector = self_1111;
    let _e48: Scalar = other_929;
    let _e52: MultiVector = self_1111;
    let _e54: Scalar = other_929;
    let _e58: MultiVector = self_1111;
    let _e60: Scalar = other_929;
    return MultiVector((_e4.g0_ * vec3<f32>(_e6.g0_)), (_e10.g1_ * vec3<f32>(_e12.g0_)), (_e16.g2_ * vec2<f32>(_e18.g0_)), (_e22.g3_ * vec4<f32>(_e24.g0_)), (_e28.g4_ * vec3<f32>(_e30.g0_)), (_e34.g5_ * vec3<f32>(_e36.g0_)), (_e40.g6_ * vec3<f32>(_e42.g0_)), (_e46.g7_ * vec3<f32>(_e48.g0_)), (_e52.g8_ * vec4<f32>(_e54.g0_)), (_e58.g9_ * vec4<f32>(_e60.g0_)));
}

fn multi_vector_scalar_inner_product(self_1112: MultiVector, other_930: Scalar) -> MultiVector {
    var self_1113: MultiVector;
    var other_931: Scalar;

    self_1113 = self_1112;
    other_931 = other_930;
    let _e4: MultiVector = self_1113;
    let _e6: Scalar = other_931;
    let _e10: MultiVector = self_1113;
    let _e12: Scalar = other_931;
    let _e16: MultiVector = self_1113;
    let _e18: Scalar = other_931;
    let _e22: MultiVector = self_1113;
    let _e24: Scalar = other_931;
    let _e28: MultiVector = self_1113;
    let _e30: Scalar = other_931;
    let _e34: MultiVector = self_1113;
    let _e36: Scalar = other_931;
    let _e40: MultiVector = self_1113;
    let _e42: Scalar = other_931;
    let _e46: MultiVector = self_1113;
    let _e48: Scalar = other_931;
    let _e52: MultiVector = self_1113;
    let _e54: Scalar = other_931;
    let _e58: MultiVector = self_1113;
    let _e60: Scalar = other_931;
    return MultiVector((_e4.g0_ * vec3<f32>(_e6.g0_)), (_e10.g1_ * vec3<f32>(_e12.g0_)), (_e16.g2_ * vec2<f32>(_e18.g0_)), (_e22.g3_ * vec4<f32>(_e24.g0_)), (_e28.g4_ * vec3<f32>(_e30.g0_)), (_e34.g5_ * vec3<f32>(_e36.g0_)), (_e40.g6_ * vec3<f32>(_e42.g0_)), (_e46.g7_ * vec3<f32>(_e48.g0_)), (_e52.g8_ * vec4<f32>(_e54.g0_)), (_e58.g9_ * vec4<f32>(_e60.g0_)));
}

fn multi_vector_scalar_left_contraction(self_1114: MultiVector, other_932: Scalar) -> Scalar {
    var self_1115: MultiVector;
    var other_933: Scalar;

    self_1115 = self_1114;
    other_933 = other_932;
    let _e4: MultiVector = self_1115;
    let _e7: Scalar = other_933;
    return Scalar((_e4.g0_.x * _e7.g0_));
}

fn multi_vector_scalar_right_contraction(self_1116: MultiVector, other_934: Scalar) -> MultiVector {
    var self_1117: MultiVector;
    var other_935: Scalar;

    self_1117 = self_1116;
    other_935 = other_934;
    let _e4: MultiVector = self_1117;
    let _e6: Scalar = other_935;
    let _e10: MultiVector = self_1117;
    let _e12: Scalar = other_935;
    let _e16: MultiVector = self_1117;
    let _e18: Scalar = other_935;
    let _e22: MultiVector = self_1117;
    let _e24: Scalar = other_935;
    let _e28: MultiVector = self_1117;
    let _e30: Scalar = other_935;
    let _e34: MultiVector = self_1117;
    let _e36: Scalar = other_935;
    let _e40: MultiVector = self_1117;
    let _e42: Scalar = other_935;
    let _e46: MultiVector = self_1117;
    let _e48: Scalar = other_935;
    let _e52: MultiVector = self_1117;
    let _e54: Scalar = other_935;
    let _e58: MultiVector = self_1117;
    let _e60: Scalar = other_935;
    return MultiVector((_e4.g0_ * vec3<f32>(_e6.g0_)), (_e10.g1_ * vec3<f32>(_e12.g0_)), (_e16.g2_ * vec2<f32>(_e18.g0_)), (_e22.g3_ * vec4<f32>(_e24.g0_)), (_e28.g4_ * vec3<f32>(_e30.g0_)), (_e34.g5_ * vec3<f32>(_e36.g0_)), (_e40.g6_ * vec3<f32>(_e42.g0_)), (_e46.g7_ * vec3<f32>(_e48.g0_)), (_e52.g8_ * vec4<f32>(_e54.g0_)), (_e58.g9_ * vec4<f32>(_e60.g0_)));
}

fn multi_vector_scalar_scalar_product(self_1118: MultiVector, other_936: Scalar) -> Scalar {
    var self_1119: MultiVector;
    var other_937: Scalar;

    self_1119 = self_1118;
    other_937 = other_936;
    let _e4: MultiVector = self_1119;
    let _e7: Scalar = other_937;
    return Scalar((_e4.g0_.x * _e7.g0_));
}

fn multi_vector_anti_scalar_into(self_1120: MultiVector) -> AntiScalar {
    var self_1121: MultiVector;

    self_1121 = self_1120;
    let _e2: MultiVector = self_1121;
    return AntiScalar(_e2.g0_.z);
}

fn multi_vector_anti_scalar_add(self_1122: MultiVector, other_938: AntiScalar) -> MultiVector {
    var self_1123: MultiVector;
    var other_939: AntiScalar;

    self_1123 = self_1122;
    other_939 = other_938;
    let _e4: MultiVector = self_1123;
    let _e6: AntiScalar = other_939;
    let _e15: MultiVector = self_1123;
    let _e17: MultiVector = self_1123;
    let _e19: MultiVector = self_1123;
    let _e21: MultiVector = self_1123;
    let _e23: MultiVector = self_1123;
    let _e25: MultiVector = self_1123;
    let _e27: MultiVector = self_1123;
    let _e29: MultiVector = self_1123;
    let _e31: MultiVector = self_1123;
    return MultiVector((_e4.g0_ + (vec3<f32>(_e6.g0_) * vec3<f32>(0.0, 0.0, 1.0))), _e15.g1_, _e17.g2_, _e19.g3_, _e21.g4_, _e23.g5_, _e25.g6_, _e27.g7_, _e29.g8_, _e31.g9_);
}

fn multi_vector_anti_scalar_sub(self_1124: MultiVector, other_940: AntiScalar) -> MultiVector {
    var self_1125: MultiVector;
    var other_941: AntiScalar;

    self_1125 = self_1124;
    other_941 = other_940;
    let _e4: MultiVector = self_1125;
    let _e6: AntiScalar = other_941;
    let _e15: MultiVector = self_1125;
    let _e17: MultiVector = self_1125;
    let _e19: MultiVector = self_1125;
    let _e21: MultiVector = self_1125;
    let _e23: MultiVector = self_1125;
    let _e25: MultiVector = self_1125;
    let _e27: MultiVector = self_1125;
    let _e29: MultiVector = self_1125;
    let _e31: MultiVector = self_1125;
    return MultiVector((_e4.g0_ - (vec3<f32>(_e6.g0_) * vec3<f32>(0.0, 0.0, 1.0))), _e15.g1_, _e17.g2_, _e19.g3_, _e21.g4_, _e23.g5_, _e25.g6_, _e27.g7_, _e29.g8_, _e31.g9_);
}

fn multi_vector_anti_scalar_regressive_product(self_1126: MultiVector, other_942: AntiScalar) -> MultiVector {
    var self_1127: MultiVector;
    var other_943: AntiScalar;

    self_1127 = self_1126;
    other_943 = other_942;
    let _e4: MultiVector = self_1127;
    let _e6: AntiScalar = other_943;
    let _e10: MultiVector = self_1127;
    let _e12: AntiScalar = other_943;
    let _e16: MultiVector = self_1127;
    let _e18: AntiScalar = other_943;
    let _e22: MultiVector = self_1127;
    let _e24: AntiScalar = other_943;
    let _e28: MultiVector = self_1127;
    let _e30: AntiScalar = other_943;
    let _e34: MultiVector = self_1127;
    let _e36: AntiScalar = other_943;
    let _e40: MultiVector = self_1127;
    let _e42: AntiScalar = other_943;
    let _e46: MultiVector = self_1127;
    let _e48: AntiScalar = other_943;
    let _e52: MultiVector = self_1127;
    let _e54: AntiScalar = other_943;
    let _e58: MultiVector = self_1127;
    let _e60: AntiScalar = other_943;
    return MultiVector((_e4.g0_ * vec3<f32>(_e6.g0_)), (_e10.g1_ * vec3<f32>(_e12.g0_)), (_e16.g2_ * vec2<f32>(_e18.g0_)), (_e22.g3_ * vec4<f32>(_e24.g0_)), (_e28.g4_ * vec3<f32>(_e30.g0_)), (_e34.g5_ * vec3<f32>(_e36.g0_)), (_e40.g6_ * vec3<f32>(_e42.g0_)), (_e46.g7_ * vec3<f32>(_e48.g0_)), (_e52.g8_ * vec4<f32>(_e54.g0_)), (_e58.g9_ * vec4<f32>(_e60.g0_)));
}

fn multi_vector_anti_scalar_outer_product(self_1128: MultiVector, other_944: AntiScalar) -> AntiScalar {
    var self_1129: MultiVector;
    var other_945: AntiScalar;

    self_1129 = self_1128;
    other_945 = other_944;
    let _e4: MultiVector = self_1129;
    let _e7: AntiScalar = other_945;
    return AntiScalar((_e4.g0_.x * _e7.g0_));
}

fn multi_vector_radial_point_into(self_1130: MultiVector) -> RadialPoint {
    var self_1131: MultiVector;

    self_1131 = self_1130;
    let _e2: MultiVector = self_1131;
    let _e4: MultiVector = self_1131;
    return RadialPoint(_e2.g1_, _e4.g2_);
}

fn multi_vector_radial_point_add(self_1132: MultiVector, other_946: RadialPoint) -> MultiVector {
    var self_1133: MultiVector;
    var other_947: RadialPoint;

    self_1133 = self_1132;
    other_947 = other_946;
    let _e4: MultiVector = self_1133;
    let _e6: MultiVector = self_1133;
    let _e8: RadialPoint = other_947;
    let _e11: MultiVector = self_1133;
    let _e13: RadialPoint = other_947;
    let _e16: MultiVector = self_1133;
    let _e18: MultiVector = self_1133;
    let _e20: MultiVector = self_1133;
    let _e22: MultiVector = self_1133;
    let _e24: MultiVector = self_1133;
    let _e26: MultiVector = self_1133;
    let _e28: MultiVector = self_1133;
    return MultiVector(_e4.g0_, (_e6.g1_ + _e8.g0_), (_e11.g2_ + _e13.g1_), _e16.g3_, _e18.g4_, _e20.g5_, _e22.g6_, _e24.g7_, _e26.g8_, _e28.g9_);
}

fn multi_vector_radial_point_sub(self_1134: MultiVector, other_948: RadialPoint) -> MultiVector {
    var self_1135: MultiVector;
    var other_949: RadialPoint;

    self_1135 = self_1134;
    other_949 = other_948;
    let _e4: MultiVector = self_1135;
    let _e6: MultiVector = self_1135;
    let _e8: RadialPoint = other_949;
    let _e11: MultiVector = self_1135;
    let _e13: RadialPoint = other_949;
    let _e16: MultiVector = self_1135;
    let _e18: MultiVector = self_1135;
    let _e20: MultiVector = self_1135;
    let _e22: MultiVector = self_1135;
    let _e24: MultiVector = self_1135;
    let _e26: MultiVector = self_1135;
    let _e28: MultiVector = self_1135;
    return MultiVector(_e4.g0_, (_e6.g1_ - _e8.g0_), (_e11.g2_ - _e13.g1_), _e16.g3_, _e18.g4_, _e20.g5_, _e22.g6_, _e24.g7_, _e26.g8_, _e28.g9_);
}

fn multi_vector_radial_point_geometric_product(self_1136: MultiVector, other_950: RadialPoint) -> MultiVector {
    var self_1137: MultiVector;
    var other_951: RadialPoint;

    self_1137 = self_1136;
    other_951 = other_950;
    let _e4: MultiVector = self_1137;
    let _e8: RadialPoint = other_951;
    let _e18: MultiVector = self_1137;
    let _e22: RadialPoint = other_951;
    let _e33: MultiVector = self_1137;
    let _e37: RadialPoint = other_951;
    let _e48: MultiVector = self_1137;
    let _e52: RadialPoint = other_951;
    let _e64: MultiVector = self_1137;
    let _e68: RadialPoint = other_951;
    let _e80: MultiVector = self_1137;
    let _e84: RadialPoint = other_951;
    let _e96: MultiVector = self_1137;
    let _e100: RadialPoint = other_951;
    let _e112: MultiVector = self_1137;
    let _e116: RadialPoint = other_951;
    let _e127: MultiVector = self_1137;
    let _e131: RadialPoint = other_951;
    let _e142: MultiVector = self_1137;
    let _e146: RadialPoint = other_951;
    let _e157: MultiVector = self_1137;
    let _e161: RadialPoint = other_951;
    let _e172: MultiVector = self_1137;
    let _e175: RadialPoint = other_951;
    let _e178: RadialPoint = other_951;
    let _e181: RadialPoint = other_951;
    let _e192: MultiVector = self_1137;
    let _e196: RadialPoint = other_951;
    let _e199: MultiVector = self_1137;
    let _e203: RadialPoint = other_951;
    let _e214: MultiVector = self_1137;
    let _e218: RadialPoint = other_951;
    let _e229: MultiVector = self_1137;
    let _e233: RadialPoint = other_951;
    let _e244: MultiVector = self_1137;
    let _e248: RadialPoint = other_951;
    let _e251: MultiVector = self_1137;
    let _e255: RadialPoint = other_951;
    let _e266: MultiVector = self_1137;
    let _e270: RadialPoint = other_951;
    let _e281: MultiVector = self_1137;
    let _e285: RadialPoint = other_951;
    let _e295: MultiVector = self_1137;
    let _e299: RadialPoint = other_951;
    let _e309: MultiVector = self_1137;
    let _e313: RadialPoint = other_951;
    let _e323: MultiVector = self_1137;
    let _e327: RadialPoint = other_951;
    let _e338: MultiVector = self_1137;
    let _e342: RadialPoint = other_951;
    let _e353: MultiVector = self_1137;
    let _e357: RadialPoint = other_951;
    let _e360: RadialPoint = other_951;
    let _e363: RadialPoint = other_951;
    let _e366: RadialPoint = other_951;
    let _e372: MultiVector = self_1137;
    let _e376: RadialPoint = other_951;
    let _e389: MultiVector = self_1137;
    let _e393: RadialPoint = other_951;
    let _e406: MultiVector = self_1137;
    let _e410: RadialPoint = other_951;
    let _e423: MultiVector = self_1137;
    let _e427: RadialPoint = other_951;
    let _e430: RadialPoint = other_951;
    let _e433: RadialPoint = other_951;
    let _e436: RadialPoint = other_951;
    let _e449: MultiVector = self_1137;
    let _e453: RadialPoint = other_951;
    let _e456: RadialPoint = other_951;
    let _e459: RadialPoint = other_951;
    let _e462: RadialPoint = other_951;
    let _e475: MultiVector = self_1137;
    let _e479: RadialPoint = other_951;
    let _e482: RadialPoint = other_951;
    let _e485: RadialPoint = other_951;
    let _e488: RadialPoint = other_951;
    let _e501: MultiVector = self_1137;
    let _e504: MultiVector = self_1137;
    let _e507: MultiVector = self_1137;
    let _e510: MultiVector = self_1137;
    let _e514: RadialPoint = other_951;
    let _e517: RadialPoint = other_951;
    let _e520: RadialPoint = other_951;
    let _e523: RadialPoint = other_951;
    let _e535: MultiVector = self_1137;
    let _e539: RadialPoint = other_951;
    let _e542: MultiVector = self_1137;
    let _e546: RadialPoint = other_951;
    let _e557: MultiVector = self_1137;
    let _e561: RadialPoint = other_951;
    let _e572: MultiVector = self_1137;
    let _e576: RadialPoint = other_951;
    let _e587: MultiVector = self_1137;
    let _e589: RadialPoint = other_951;
    let _e599: MultiVector = self_1137;
    let _e603: RadialPoint = other_951;
    let _e613: MultiVector = self_1137;
    let _e617: RadialPoint = other_951;
    let _e628: MultiVector = self_1137;
    let _e632: RadialPoint = other_951;
    let _e636: MultiVector = self_1137;
    let _e640: RadialPoint = other_951;
    let _e653: MultiVector = self_1137;
    let _e657: RadialPoint = other_951;
    let _e661: MultiVector = self_1137;
    let _e665: RadialPoint = other_951;
    let _e676: MultiVector = self_1137;
    let _e680: RadialPoint = other_951;
    let _e691: MultiVector = self_1137;
    let _e695: RadialPoint = other_951;
    let _e706: MultiVector = self_1137;
    let _e710: RadialPoint = other_951;
    let _e721: MultiVector = self_1137;
    let _e725: RadialPoint = other_951;
    let _e736: MultiVector = self_1137;
    let _e740: RadialPoint = other_951;
    let _e751: MultiVector = self_1137;
    let _e754: MultiVector = self_1137;
    let _e757: MultiVector = self_1137;
    let _e761: RadialPoint = other_951;
    let _e767: MultiVector = self_1137;
    let _e771: RadialPoint = other_951;
    let _e781: MultiVector = self_1137;
    let _e785: RadialPoint = other_951;
    let _e796: MultiVector = self_1137;
    let _e800: RadialPoint = other_951;
    let _e811: MultiVector = self_1137;
    let _e815: RadialPoint = other_951;
    let _e826: MultiVector = self_1137;
    let _e830: RadialPoint = other_951;
    let _e841: MultiVector = self_1137;
    let _e845: RadialPoint = other_951;
    let _e849: MultiVector = self_1137;
    let _e853: RadialPoint = other_951;
    let _e864: MultiVector = self_1137;
    let _e868: RadialPoint = other_951;
    let _e871: RadialPoint = other_951;
    let _e874: RadialPoint = other_951;
    let _e877: RadialPoint = other_951;
    let _e889: MultiVector = self_1137;
    let _e893: RadialPoint = other_951;
    let _e896: RadialPoint = other_951;
    let _e899: RadialPoint = other_951;
    let _e902: RadialPoint = other_951;
    let _e915: MultiVector = self_1137;
    let _e919: RadialPoint = other_951;
    let _e922: RadialPoint = other_951;
    let _e925: RadialPoint = other_951;
    let _e928: RadialPoint = other_951;
    let _e941: MultiVector = self_1137;
    let _e945: RadialPoint = other_951;
    let _e948: RadialPoint = other_951;
    let _e951: RadialPoint = other_951;
    let _e954: RadialPoint = other_951;
    let _e967: MultiVector = self_1137;
    let _e971: RadialPoint = other_951;
    let _e974: RadialPoint = other_951;
    let _e977: RadialPoint = other_951;
    let _e980: RadialPoint = other_951;
    let _e993: MultiVector = self_1137;
    let _e997: RadialPoint = other_951;
    let _e1000: RadialPoint = other_951;
    let _e1003: RadialPoint = other_951;
    let _e1006: RadialPoint = other_951;
    let _e1019: MultiVector = self_1137;
    let _e1022: MultiVector = self_1137;
    let _e1025: MultiVector = self_1137;
    let _e1028: MultiVector = self_1137;
    let _e1032: RadialPoint = other_951;
    let _e1035: RadialPoint = other_951;
    let _e1038: RadialPoint = other_951;
    let _e1041: RadialPoint = other_951;
    let _e1056: MultiVector = self_1137;
    let _e1060: RadialPoint = other_951;
    let _e1063: RadialPoint = other_951;
    let _e1066: RadialPoint = other_951;
    let _e1069: RadialPoint = other_951;
    let _e1081: MultiVector = self_1137;
    let _e1085: RadialPoint = other_951;
    let _e1088: RadialPoint = other_951;
    let _e1091: RadialPoint = other_951;
    let _e1094: RadialPoint = other_951;
    let _e1107: MultiVector = self_1137;
    let _e1111: RadialPoint = other_951;
    let _e1114: RadialPoint = other_951;
    let _e1117: RadialPoint = other_951;
    let _e1120: RadialPoint = other_951;
    let _e1133: MultiVector = self_1137;
    let _e1137: RadialPoint = other_951;
    let _e1140: RadialPoint = other_951;
    let _e1143: RadialPoint = other_951;
    let _e1146: RadialPoint = other_951;
    let _e1159: MultiVector = self_1137;
    let _e1163: RadialPoint = other_951;
    let _e1166: RadialPoint = other_951;
    let _e1169: RadialPoint = other_951;
    let _e1172: RadialPoint = other_951;
    let _e1185: MultiVector = self_1137;
    let _e1189: RadialPoint = other_951;
    let _e1192: RadialPoint = other_951;
    let _e1195: RadialPoint = other_951;
    let _e1198: RadialPoint = other_951;
    let _e1211: MultiVector = self_1137;
    let _e1215: RadialPoint = other_951;
    let _e1227: MultiVector = self_1137;
    let _e1231: RadialPoint = other_951;
    let _e1243: MultiVector = self_1137;
    let _e1247: RadialPoint = other_951;
    let _e1259: MultiVector = self_1137;
    let _e1263: RadialPoint = other_951;
    let _e1275: MultiVector = self_1137;
    let _e1278: MultiVector = self_1137;
    let _e1281: MultiVector = self_1137;
    let _e1284: MultiVector = self_1137;
    let _e1288: RadialPoint = other_951;
    let _e1291: RadialPoint = other_951;
    let _e1294: RadialPoint = other_951;
    let _e1297: RadialPoint = other_951;
    return MultiVector((((((((((((((vec3<f32>(_e4.g1_.x) * vec3<f32>(_e8.g0_.x)) * vec3<f32>(1.0, 0.0, 0.0)) + ((vec3<f32>(_e18.g1_.y) * vec3<f32>(_e22.g0_.y)) * vec3<f32>(1.0, 0.0, 0.0))) + ((vec3<f32>(_e33.g1_.z) * vec3<f32>(_e37.g0_.z)) * vec3<f32>(1.0, 0.0, 0.0))) + ((vec3<f32>(_e48.g8_.x) * vec3<f32>(_e52.g0_.x)) * vec3<f32>(0.0, -(1.0), 0.0))) + ((vec3<f32>(_e64.g8_.y) * vec3<f32>(_e68.g0_.y)) * vec3<f32>(0.0, -(1.0), 0.0))) + ((vec3<f32>(_e80.g8_.z) * vec3<f32>(_e84.g0_.z)) * vec3<f32>(0.0, -(1.0), 0.0))) + ((vec3<f32>(_e96.g8_.w) * vec3<f32>(_e100.g1_.x)) * vec3<f32>(0.0, -(1.0), 0.0))) + ((vec3<f32>(_e112.g9_.x) * vec3<f32>(_e116.g0_.x)) * vec3<f32>(0.0, 0.0, 1.0))) + ((vec3<f32>(_e127.g9_.y) * vec3<f32>(_e131.g0_.y)) * vec3<f32>(0.0, 0.0, 1.0))) + ((vec3<f32>(_e142.g9_.z) * vec3<f32>(_e146.g0_.z)) * vec3<f32>(0.0, 0.0, 1.0))) + ((vec3<f32>(_e157.g9_.w) * vec3<f32>(_e161.g1_.x)) * vec3<f32>(0.0, 0.0, 1.0))) + ((_e172.g0_.xxy * vec3<f32>(_e175.g1_.x, _e178.g1_.x, _e181.g1_.y)) * vec3<f32>(0.0, 0.0, 1.0))), ((((vec3<f32>(_e192.g0_.x) * _e196.g0_) + ((vec3<f32>(_e199.g5_.y) * _e203.g0_.zzx) * vec3<f32>(-(1.0), 0.0, 1.0))) + ((vec3<f32>(_e214.g5_.z) * _e218.g0_.yxy) * vec3<f32>(1.0, -(1.0), 0.0))) + ((vec3<f32>(_e229.g5_.x) * _e233.g0_.xzy) * vec3<f32>(0.0, 1.0, -(1.0)))), (((((((vec2<f32>(_e244.g0_.x) * _e248.g1_) + ((vec2<f32>(_e251.g3_.y) * vec2<f32>(_e255.g0_.y)) * vec2<f32>(0.0, -(1.0)))) + ((vec2<f32>(_e266.g3_.z) * vec2<f32>(_e270.g0_.z)) * vec2<f32>(0.0, -(1.0)))) + ((vec2<f32>(_e281.g4_.x) * vec2<f32>(_e285.g0_.x)) * vec2<f32>(1.0, 0.0))) + ((vec2<f32>(_e295.g4_.y) * vec2<f32>(_e299.g0_.y)) * vec2<f32>(1.0, 0.0))) + ((vec2<f32>(_e309.g4_.z) * vec2<f32>(_e313.g0_.z)) * vec2<f32>(1.0, 0.0))) + ((vec2<f32>(_e323.g3_.x) * vec2<f32>(_e327.g0_.x)) * vec2<f32>(0.0, -(1.0)))), ((((((((((vec4<f32>(_e338.g2_.x) * vec4<f32>(_e342.g1_.y)) * vec4<f32>(0.0, 0.0, 0.0, 1.0)) - (vec4<f32>(_e353.g2_.y) * vec4<f32>(_e357.g0_.x, _e360.g0_.y, _e363.g0_.z, _e366.g1_.x))) + ((vec4<f32>(_e372.g6_.x) * vec4<f32>(_e376.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e389.g6_.y) * vec4<f32>(_e393.g0_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e406.g6_.z) * vec4<f32>(_e410.g0_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e423.g7_.x) * vec4<f32>(_e427.g0_.z, _e430.g0_.z, _e433.g0_.y, _e436.g0_.z)) * vec4<f32>(0.0, -(1.0), 1.0, 0.0))) + ((vec4<f32>(_e449.g7_.y) * vec4<f32>(_e453.g0_.z, _e456.g0_.z, _e459.g0_.x, _e462.g0_.z)) * vec4<f32>(1.0, 0.0, -(1.0), 0.0))) + ((vec4<f32>(_e475.g7_.z) * vec4<f32>(_e479.g0_.y, _e482.g0_.x, _e485.g0_.y, _e488.g0_.y)) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))) + ((vec4<f32>(_e501.g1_.x, _e504.g1_.y, _e507.g1_.z, _e510.g1_.x) * vec4<f32>(_e514.g1_.y, _e517.g1_.y, _e520.g1_.y, _e523.g1_.x)) * vec4<f32>(1.0, 1.0, 1.0, 0.0))), (((((vec3<f32>(_e535.g2_.x) * _e539.g0_) + ((vec3<f32>(_e542.g8_.x) * _e546.g0_.zzy) * vec3<f32>(0.0, 1.0, -(1.0)))) + ((vec3<f32>(_e557.g8_.y) * _e561.g0_.zzx) * vec3<f32>(-(1.0), 0.0, 1.0))) + ((vec3<f32>(_e572.g8_.z) * _e576.g0_.yxy) * vec3<f32>(1.0, -(1.0), 0.0))) + ((_e587.g1_ * vec3<f32>(_e589.g1_.x)) * vec3<f32>(-(1.0)))), (((((vec3<f32>(_e599.g1_.y) * _e603.g0_.zzx) * vec3<f32>(1.0, 0.0, -(1.0))) + ((vec3<f32>(_e613.g1_.z) * _e617.g0_.yxy) * vec3<f32>(-(1.0), 1.0, 0.0))) - (vec3<f32>(_e628.g8_.w) * _e632.g0_)) + ((vec3<f32>(_e636.g1_.x) * _e640.g0_.xzy) * vec3<f32>(0.0, -(1.0), 1.0))), ((((((((vec3<f32>(0.0) - (vec3<f32>(_e653.g3_.w) * _e657.g0_)) + ((vec3<f32>(_e661.g4_.x) * vec3<f32>(_e665.g1_.y)) * vec3<f32>(1.0, 0.0, 0.0))) + ((vec3<f32>(_e676.g4_.y) * vec3<f32>(_e680.g1_.y)) * vec3<f32>(0.0, 1.0, 0.0))) + ((vec3<f32>(_e691.g4_.z) * vec3<f32>(_e695.g1_.y)) * vec3<f32>(0.0, 0.0, 1.0))) + ((vec3<f32>(_e706.g9_.x) * _e710.g0_.zzy) * vec3<f32>(0.0, -(1.0), 1.0))) + ((vec3<f32>(_e721.g9_.y) * _e725.g0_.zzx) * vec3<f32>(1.0, 0.0, -(1.0)))) + ((vec3<f32>(_e736.g9_.z) * _e740.g0_.yxy) * vec3<f32>(-(1.0), 1.0, 0.0))) + (vec3<f32>(_e751.g3_.x, _e754.g3_.y, _e757.g3_.z) * vec3<f32>(_e761.g1_.x))), ((((((((vec3<f32>(_e767.g3_.y) * _e771.g0_.zzx) * vec3<f32>(-(1.0), 0.0, 1.0)) + ((vec3<f32>(_e781.g3_.z) * _e785.g0_.yxy) * vec3<f32>(1.0, -(1.0), 0.0))) + ((vec3<f32>(_e796.g5_.x) * vec3<f32>(_e800.g1_.y)) * vec3<f32>(1.0, 0.0, 0.0))) + ((vec3<f32>(_e811.g5_.y) * vec3<f32>(_e815.g1_.y)) * vec3<f32>(0.0, 1.0, 0.0))) + ((vec3<f32>(_e826.g5_.z) * vec3<f32>(_e830.g1_.y)) * vec3<f32>(0.0, 0.0, 1.0))) + (vec3<f32>(_e841.g9_.w) * _e845.g0_)) + ((vec3<f32>(_e849.g3_.x) * _e853.g0_.xzy) * vec3<f32>(0.0, 1.0, -(1.0)))), ((((((((vec4<f32>(_e864.g4_.x) * vec4<f32>(_e868.g0_.z, _e871.g0_.z, _e874.g0_.y, _e877.g0_.z)) * vec4<f32>(0.0, -(1.0), 1.0, 0.0)) + ((vec4<f32>(_e889.g4_.y) * vec4<f32>(_e893.g0_.z, _e896.g0_.z, _e899.g0_.x, _e902.g0_.z)) * vec4<f32>(1.0, 0.0, -(1.0), 0.0))) + ((vec4<f32>(_e915.g4_.z) * vec4<f32>(_e919.g0_.y, _e922.g0_.x, _e925.g0_.y, _e928.g0_.y)) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))) + ((vec4<f32>(_e941.g5_.x) * vec4<f32>(_e945.g1_.x, _e948.g1_.x, _e951.g1_.x, _e954.g0_.x)) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e967.g5_.y) * vec4<f32>(_e971.g1_.x, _e974.g1_.x, _e977.g1_.x, _e980.g0_.y)) * vec4<f32>(0.0, 1.0, 0.0, -(1.0)))) + ((vec4<f32>(_e993.g5_.z) * vec4<f32>(_e997.g1_.x, _e1000.g1_.x, _e1003.g1_.x, _e1006.g0_.z)) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e1019.g0_.y, _e1022.g0_.y, _e1025.g0_.y, _e1028.g0_.x) * vec4<f32>(_e1032.g0_.x, _e1035.g0_.y, _e1038.g0_.z, _e1041.g0_.x)) * vec4<f32>(-(1.0), -(1.0), -(1.0), 0.0))), ((((((((((((vec4<f32>(_e1056.g6_.x) * vec4<f32>(_e1060.g0_.z, _e1063.g0_.z, _e1066.g0_.y, _e1069.g0_.z)) * vec4<f32>(0.0, 1.0, -(1.0), 0.0)) + ((vec4<f32>(_e1081.g6_.y) * vec4<f32>(_e1085.g0_.z, _e1088.g0_.z, _e1091.g0_.x, _e1094.g0_.z)) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0))) + ((vec4<f32>(_e1107.g6_.z) * vec4<f32>(_e1111.g0_.y, _e1114.g0_.x, _e1117.g0_.y, _e1120.g0_.y)) * vec4<f32>(1.0, -(1.0), 0.0, 0.0))) + ((vec4<f32>(_e1133.g7_.x) * vec4<f32>(_e1137.g1_.x, _e1140.g1_.x, _e1143.g1_.x, _e1146.g0_.x)) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))) + ((vec4<f32>(_e1159.g7_.y) * vec4<f32>(_e1163.g1_.x, _e1166.g1_.x, _e1169.g1_.x, _e1172.g0_.y)) * vec4<f32>(0.0, -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e1185.g7_.z) * vec4<f32>(_e1189.g1_.x, _e1192.g1_.x, _e1195.g1_.x, _e1198.g0_.z)) * vec4<f32>(0.0, 0.0, -(1.0), 1.0))) + ((vec4<f32>(_e1211.g8_.x) * vec4<f32>(_e1215.g1_.y)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e1227.g8_.y) * vec4<f32>(_e1231.g1_.y)) * vec4<f32>(0.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e1243.g8_.z) * vec4<f32>(_e1247.g1_.y)) * vec4<f32>(0.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e1259.g8_.w) * vec4<f32>(_e1263.g1_.y)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e1275.g0_.z, _e1278.g0_.z, _e1281.g0_.z, _e1284.g0_.x) * vec4<f32>(_e1288.g0_.x, _e1291.g0_.y, _e1294.g0_.z, _e1297.g0_.x)) * vec4<f32>(1.0, 1.0, 1.0, 0.0))));
}

fn multi_vector_radial_point_scalar_product(self_1138: MultiVector, other_952: RadialPoint) -> Scalar {
    var self_1139: MultiVector;
    var other_953: RadialPoint;

    self_1139 = self_1138;
    other_953 = other_952;
    let _e4: MultiVector = self_1139;
    let _e7: RadialPoint = other_953;
    let _e11: MultiVector = self_1139;
    let _e14: RadialPoint = other_953;
    let _e19: MultiVector = self_1139;
    let _e22: RadialPoint = other_953;
    return Scalar((((_e4.g1_.x * _e7.g0_.x) + (_e11.g1_.y * _e14.g0_.y)) + (_e19.g1_.z * _e22.g0_.z)));
}

fn multi_vector_flat_point_into(self_1140: MultiVector) -> FlatPoint {
    var self_1141: MultiVector;

    self_1141 = self_1140;
    let _e2: MultiVector = self_1141;
    return FlatPoint(_e2.g3_);
}

fn multi_vector_flat_point_add(self_1142: MultiVector, other_954: FlatPoint) -> MultiVector {
    var self_1143: MultiVector;
    var other_955: FlatPoint;

    self_1143 = self_1142;
    other_955 = other_954;
    let _e4: MultiVector = self_1143;
    let _e6: MultiVector = self_1143;
    let _e8: MultiVector = self_1143;
    let _e10: MultiVector = self_1143;
    let _e12: FlatPoint = other_955;
    let _e15: MultiVector = self_1143;
    let _e17: MultiVector = self_1143;
    let _e19: MultiVector = self_1143;
    let _e21: MultiVector = self_1143;
    let _e23: MultiVector = self_1143;
    let _e25: MultiVector = self_1143;
    return MultiVector(_e4.g0_, _e6.g1_, _e8.g2_, (_e10.g3_ + _e12.g0_), _e15.g4_, _e17.g5_, _e19.g6_, _e21.g7_, _e23.g8_, _e25.g9_);
}

fn multi_vector_flat_point_sub(self_1144: MultiVector, other_956: FlatPoint) -> MultiVector {
    var self_1145: MultiVector;
    var other_957: FlatPoint;

    self_1145 = self_1144;
    other_957 = other_956;
    let _e4: MultiVector = self_1145;
    let _e6: MultiVector = self_1145;
    let _e8: MultiVector = self_1145;
    let _e10: MultiVector = self_1145;
    let _e12: FlatPoint = other_957;
    let _e15: MultiVector = self_1145;
    let _e17: MultiVector = self_1145;
    let _e19: MultiVector = self_1145;
    let _e21: MultiVector = self_1145;
    let _e23: MultiVector = self_1145;
    let _e25: MultiVector = self_1145;
    return MultiVector(_e4.g0_, _e6.g1_, _e8.g2_, (_e10.g3_ - _e12.g0_), _e15.g4_, _e17.g5_, _e19.g6_, _e21.g7_, _e23.g8_, _e25.g9_);
}

fn multi_vector_dipole_into(self_1146: MultiVector) -> Dipole {
    var self_1147: MultiVector;

    self_1147 = self_1146;
    let _e2: MultiVector = self_1147;
    let _e4: MultiVector = self_1147;
    let _e6: MultiVector = self_1147;
    return Dipole(_e2.g4_, _e4.g5_, _e6.g3_);
}

fn multi_vector_dipole_add(self_1148: MultiVector, other_958: Dipole) -> MultiVector {
    var self_1149: MultiVector;
    var other_959: Dipole;

    self_1149 = self_1148;
    other_959 = other_958;
    let _e4: MultiVector = self_1149;
    let _e6: MultiVector = self_1149;
    let _e8: MultiVector = self_1149;
    let _e10: MultiVector = self_1149;
    let _e12: Dipole = other_959;
    let _e15: MultiVector = self_1149;
    let _e17: Dipole = other_959;
    let _e20: MultiVector = self_1149;
    let _e22: Dipole = other_959;
    let _e25: MultiVector = self_1149;
    let _e27: MultiVector = self_1149;
    let _e29: MultiVector = self_1149;
    let _e31: MultiVector = self_1149;
    return MultiVector(_e4.g0_, _e6.g1_, _e8.g2_, (_e10.g3_ + _e12.g2_), (_e15.g4_ + _e17.g0_), (_e20.g5_ + _e22.g1_), _e25.g6_, _e27.g7_, _e29.g8_, _e31.g9_);
}

fn multi_vector_dipole_sub(self_1150: MultiVector, other_960: Dipole) -> MultiVector {
    var self_1151: MultiVector;
    var other_961: Dipole;

    self_1151 = self_1150;
    other_961 = other_960;
    let _e4: MultiVector = self_1151;
    let _e6: MultiVector = self_1151;
    let _e8: MultiVector = self_1151;
    let _e10: MultiVector = self_1151;
    let _e12: Dipole = other_961;
    let _e15: MultiVector = self_1151;
    let _e17: Dipole = other_961;
    let _e20: MultiVector = self_1151;
    let _e22: Dipole = other_961;
    let _e25: MultiVector = self_1151;
    let _e27: MultiVector = self_1151;
    let _e29: MultiVector = self_1151;
    let _e31: MultiVector = self_1151;
    return MultiVector(_e4.g0_, _e6.g1_, _e8.g2_, (_e10.g3_ - _e12.g2_), (_e15.g4_ - _e17.g0_), (_e20.g5_ - _e22.g1_), _e25.g6_, _e27.g7_, _e29.g8_, _e31.g9_);
}

fn multi_vector_dipole_geometric_product(self_1152: MultiVector, other_962: Dipole) -> MultiVector {
    var self_1153: MultiVector;
    var other_963: Dipole;

    self_1153 = self_1152;
    other_963 = other_962;
    let _e4: MultiVector = self_1153;
    let _e8: Dipole = other_963;
    let _e19: MultiVector = self_1153;
    let _e23: Dipole = other_963;
    let _e35: MultiVector = self_1153;
    let _e39: Dipole = other_963;
    let _e42: Dipole = other_963;
    let _e45: Dipole = other_963;
    let _e58: MultiVector = self_1153;
    let _e62: Dipole = other_963;
    let _e65: Dipole = other_963;
    let _e68: Dipole = other_963;
    let _e81: MultiVector = self_1153;
    let _e85: Dipole = other_963;
    let _e88: Dipole = other_963;
    let _e91: Dipole = other_963;
    let _e104: MultiVector = self_1153;
    let _e108: Dipole = other_963;
    let _e120: MultiVector = self_1153;
    let _e124: Dipole = other_963;
    let _e136: MultiVector = self_1153;
    let _e140: Dipole = other_963;
    let _e152: MultiVector = self_1153;
    let _e156: Dipole = other_963;
    let _e168: MultiVector = self_1153;
    let _e172: Dipole = other_963;
    let _e184: MultiVector = self_1153;
    let _e188: Dipole = other_963;
    let _e200: MultiVector = self_1153;
    let _e204: Dipole = other_963;
    let _e216: MultiVector = self_1153;
    let _e220: Dipole = other_963;
    let _e232: MultiVector = self_1153;
    let _e236: Dipole = other_963;
    let _e248: MultiVector = self_1153;
    let _e251: MultiVector = self_1153;
    let _e254: MultiVector = self_1153;
    let _e258: Dipole = other_963;
    let _e271: MultiVector = self_1153;
    let _e275: Dipole = other_963;
    let _e285: MultiVector = self_1153;
    let _e289: Dipole = other_963;
    let _e300: MultiVector = self_1153;
    let _e304: Dipole = other_963;
    let _e308: MultiVector = self_1153;
    let _e312: Dipole = other_963;
    let _e323: MultiVector = self_1153;
    let _e327: Dipole = other_963;
    let _e330: Dipole = other_963;
    let _e340: MultiVector = self_1153;
    let _e344: Dipole = other_963;
    let _e347: Dipole = other_963;
    let _e358: MultiVector = self_1153;
    let _e362: Dipole = other_963;
    let _e365: Dipole = other_963;
    let _e376: MultiVector = self_1153;
    let _e380: Dipole = other_963;
    let _e391: MultiVector = self_1153;
    let _e395: Dipole = other_963;
    let _e406: MultiVector = self_1153;
    let _e410: Dipole = other_963;
    let _e421: MultiVector = self_1153;
    let _e425: Dipole = other_963;
    let _e436: MultiVector = self_1153;
    let _e440: Dipole = other_963;
    let _e451: MultiVector = self_1153;
    let _e455: Dipole = other_963;
    let _e466: MultiVector = self_1153;
    let _e470: Dipole = other_963;
    let _e473: MultiVector = self_1153;
    let _e477: Dipole = other_963;
    let _e480: Dipole = other_963;
    let _e483: Dipole = other_963;
    let _e486: Dipole = other_963;
    let _e500: MultiVector = self_1153;
    let _e504: Dipole = other_963;
    let _e507: Dipole = other_963;
    let _e510: Dipole = other_963;
    let _e513: Dipole = other_963;
    let _e527: MultiVector = self_1153;
    let _e531: Dipole = other_963;
    let _e543: MultiVector = self_1153;
    let _e547: Dipole = other_963;
    let _e559: MultiVector = self_1153;
    let _e563: Dipole = other_963;
    let _e575: MultiVector = self_1153;
    let _e579: Dipole = other_963;
    let _e591: MultiVector = self_1153;
    let _e595: Dipole = other_963;
    let _e607: MultiVector = self_1153;
    let _e611: Dipole = other_963;
    let _e623: MultiVector = self_1153;
    let _e627: Dipole = other_963;
    let _e640: MultiVector = self_1153;
    let _e644: Dipole = other_963;
    let _e657: MultiVector = self_1153;
    let _e661: Dipole = other_963;
    let _e674: MultiVector = self_1153;
    let _e678: Dipole = other_963;
    let _e681: Dipole = other_963;
    let _e684: Dipole = other_963;
    let _e687: Dipole = other_963;
    let _e699: MultiVector = self_1153;
    let _e703: Dipole = other_963;
    let _e706: Dipole = other_963;
    let _e709: Dipole = other_963;
    let _e712: Dipole = other_963;
    let _e726: MultiVector = self_1153;
    let _e730: Dipole = other_963;
    let _e733: MultiVector = self_1153;
    let _e737: Dipole = other_963;
    let _e741: MultiVector = self_1153;
    let _e745: Dipole = other_963;
    let _e756: MultiVector = self_1153;
    let _e760: Dipole = other_963;
    let _e771: MultiVector = self_1153;
    let _e775: Dipole = other_963;
    let _e786: MultiVector = self_1153;
    let _e790: Dipole = other_963;
    let _e801: MultiVector = self_1153;
    let _e805: Dipole = other_963;
    let _e816: MultiVector = self_1153;
    let _e820: Dipole = other_963;
    let _e831: MultiVector = self_1153;
    let _e835: Dipole = other_963;
    let _e838: MultiVector = self_1153;
    let _e842: Dipole = other_963;
    let _e853: MultiVector = self_1153;
    let _e857: Dipole = other_963;
    let _e868: MultiVector = self_1153;
    let _e872: Dipole = other_963;
    let _e883: MultiVector = self_1153;
    let _e887: Dipole = other_963;
    let _e890: MultiVector = self_1153;
    let _e894: Dipole = other_963;
    let _e897: Dipole = other_963;
    let _e900: Dipole = other_963;
    let _e906: MultiVector = self_1153;
    let _e910: Dipole = other_963;
    let _e914: MultiVector = self_1153;
    let _e918: Dipole = other_963;
    let _e929: MultiVector = self_1153;
    let _e933: Dipole = other_963;
    let _e944: MultiVector = self_1153;
    let _e948: Dipole = other_963;
    let _e959: MultiVector = self_1153;
    let _e963: Dipole = other_963;
    let _e974: MultiVector = self_1153;
    let _e978: Dipole = other_963;
    let _e989: MultiVector = self_1153;
    let _e993: Dipole = other_963;
    let _e1004: MultiVector = self_1153;
    let _e1008: Dipole = other_963;
    let _e1011: Dipole = other_963;
    let _e1014: Dipole = other_963;
    let _e1026: MultiVector = self_1153;
    let _e1030: Dipole = other_963;
    let _e1033: Dipole = other_963;
    let _e1036: Dipole = other_963;
    let _e1048: MultiVector = self_1153;
    let _e1052: Dipole = other_963;
    let _e1055: Dipole = other_963;
    let _e1058: Dipole = other_963;
    let _e1070: MultiVector = self_1153;
    let _e1072: Dipole = other_963;
    let _e1082: MultiVector = self_1153;
    let _e1086: Dipole = other_963;
    let _e1089: Dipole = other_963;
    let _e1092: Dipole = other_963;
    let _e1103: MultiVector = self_1153;
    let _e1107: Dipole = other_963;
    let _e1110: Dipole = other_963;
    let _e1113: Dipole = other_963;
    let _e1125: MultiVector = self_1153;
    let _e1129: Dipole = other_963;
    let _e1133: MultiVector = self_1153;
    let _e1137: Dipole = other_963;
    let _e1148: MultiVector = self_1153;
    let _e1152: Dipole = other_963;
    let _e1163: MultiVector = self_1153;
    let _e1167: Dipole = other_963;
    let _e1178: MultiVector = self_1153;
    let _e1182: Dipole = other_963;
    let _e1185: Dipole = other_963;
    let _e1188: Dipole = other_963;
    let _e1194: MultiVector = self_1153;
    let _e1198: Dipole = other_963;
    let _e1201: Dipole = other_963;
    let _e1204: Dipole = other_963;
    let _e1216: MultiVector = self_1153;
    let _e1220: Dipole = other_963;
    let _e1223: Dipole = other_963;
    let _e1226: Dipole = other_963;
    let _e1229: Dipole = other_963;
    let _e1242: MultiVector = self_1153;
    let _e1246: Dipole = other_963;
    let _e1249: Dipole = other_963;
    let _e1252: Dipole = other_963;
    let _e1255: Dipole = other_963;
    let _e1269: MultiVector = self_1153;
    let _e1273: Dipole = other_963;
    let _e1276: Dipole = other_963;
    let _e1279: Dipole = other_963;
    let _e1282: Dipole = other_963;
    let _e1294: MultiVector = self_1153;
    let _e1298: Dipole = other_963;
    let _e1301: Dipole = other_963;
    let _e1304: Dipole = other_963;
    let _e1307: Dipole = other_963;
    let _e1320: MultiVector = self_1153;
    let _e1324: Dipole = other_963;
    let _e1327: Dipole = other_963;
    let _e1330: Dipole = other_963;
    let _e1333: Dipole = other_963;
    let _e1346: MultiVector = self_1153;
    let _e1350: Dipole = other_963;
    let _e1353: Dipole = other_963;
    let _e1356: Dipole = other_963;
    let _e1359: Dipole = other_963;
    let _e1372: MultiVector = self_1153;
    let _e1376: Dipole = other_963;
    let _e1379: Dipole = other_963;
    let _e1382: Dipole = other_963;
    let _e1385: Dipole = other_963;
    let _e1397: MultiVector = self_1153;
    let _e1401: Dipole = other_963;
    let _e1404: Dipole = other_963;
    let _e1407: Dipole = other_963;
    let _e1410: Dipole = other_963;
    let _e1424: MultiVector = self_1153;
    let _e1428: Dipole = other_963;
    let _e1431: Dipole = other_963;
    let _e1434: Dipole = other_963;
    let _e1437: Dipole = other_963;
    let _e1450: MultiVector = self_1153;
    let _e1454: Dipole = other_963;
    let _e1457: Dipole = other_963;
    let _e1460: Dipole = other_963;
    let _e1463: Dipole = other_963;
    let _e1477: MultiVector = self_1153;
    let _e1481: Dipole = other_963;
    let _e1484: Dipole = other_963;
    let _e1487: Dipole = other_963;
    let _e1490: Dipole = other_963;
    let _e1504: MultiVector = self_1153;
    let _e1508: Dipole = other_963;
    let _e1511: Dipole = other_963;
    let _e1514: Dipole = other_963;
    let _e1517: Dipole = other_963;
    let _e1529: MultiVector = self_1153;
    let _e1533: Dipole = other_963;
    let _e1545: MultiVector = self_1153;
    let _e1549: Dipole = other_963;
    let _e1561: MultiVector = self_1153;
    let _e1565: Dipole = other_963;
    let _e1577: MultiVector = self_1153;
    let _e1581: Dipole = other_963;
    let _e1593: MultiVector = self_1153;
    let _e1597: Dipole = other_963;
    let _e1609: MultiVector = self_1153;
    let _e1613: Dipole = other_963;
    let _e1625: MultiVector = self_1153;
    let _e1629: Dipole = other_963;
    let _e1632: Dipole = other_963;
    let _e1635: Dipole = other_963;
    let _e1638: Dipole = other_963;
    let _e1651: MultiVector = self_1153;
    let _e1655: Dipole = other_963;
    let _e1658: Dipole = other_963;
    let _e1661: Dipole = other_963;
    let _e1664: Dipole = other_963;
    let _e1677: MultiVector = self_1153;
    let _e1681: Dipole = other_963;
    let _e1684: Dipole = other_963;
    let _e1687: Dipole = other_963;
    let _e1690: Dipole = other_963;
    let _e1703: MultiVector = self_1153;
    let _e1707: Dipole = other_963;
    let _e1710: Dipole = other_963;
    let _e1713: Dipole = other_963;
    let _e1716: Dipole = other_963;
    let _e1728: MultiVector = self_1153;
    let _e1731: MultiVector = self_1153;
    let _e1734: MultiVector = self_1153;
    let _e1737: MultiVector = self_1153;
    let _e1741: Dipole = other_963;
    return MultiVector(((((((((((((((((vec3<f32>(_e4.g4_.y) * vec3<f32>(_e8.g1_.y)) * vec3<f32>(0.0, -(1.0), 0.0)) + ((vec3<f32>(_e19.g4_.z) * vec3<f32>(_e23.g1_.z)) * vec3<f32>(0.0, -(1.0), 0.0))) + ((vec3<f32>(_e35.g5_.x) * vec3<f32>(_e39.g1_.x, _e42.g0_.x, _e45.g1_.x)) * vec3<f32>(-(1.0), -(1.0), 0.0))) + ((vec3<f32>(_e58.g5_.y) * vec3<f32>(_e62.g1_.y, _e65.g0_.y, _e68.g1_.y)) * vec3<f32>(-(1.0), -(1.0), 0.0))) + ((vec3<f32>(_e81.g5_.z) * vec3<f32>(_e85.g1_.z, _e88.g0_.z, _e91.g1_.z)) * vec3<f32>(-(1.0), -(1.0), 0.0))) + ((vec3<f32>(_e104.g6_.y) * vec3<f32>(_e108.g1_.y)) * vec3<f32>(0.0, 0.0, -(1.0)))) + ((vec3<f32>(_e120.g6_.z) * vec3<f32>(_e124.g1_.z)) * vec3<f32>(0.0, 0.0, -(1.0)))) + ((vec3<f32>(_e136.g7_.x) * vec3<f32>(_e140.g0_.x)) * vec3<f32>(0.0, 0.0, -(1.0)))) + ((vec3<f32>(_e152.g7_.y) * vec3<f32>(_e156.g0_.y)) * vec3<f32>(0.0, 0.0, -(1.0)))) + ((vec3<f32>(_e168.g7_.z) * vec3<f32>(_e172.g0_.z)) * vec3<f32>(0.0, 0.0, -(1.0)))) + ((vec3<f32>(_e184.g8_.x) * vec3<f32>(_e188.g2_.x)) * vec3<f32>(0.0, 0.0, -(1.0)))) + ((vec3<f32>(_e200.g8_.y) * vec3<f32>(_e204.g2_.y)) * vec3<f32>(0.0, 0.0, -(1.0)))) + ((vec3<f32>(_e216.g8_.z) * vec3<f32>(_e220.g2_.z)) * vec3<f32>(0.0, 0.0, -(1.0)))) + ((vec3<f32>(_e232.g8_.w) * vec3<f32>(_e236.g2_.w)) * vec3<f32>(0.0, 0.0, -(1.0)))) + ((vec3<f32>(_e248.g4_.x, _e251.g4_.x, _e254.g6_.x) * vec3<f32>(_e258.g1_.x)) * vec3<f32>(0.0, -(1.0), -(1.0)))), (((((vec3<f32>(_e271.g1_.y) * _e275.g1_.zzx) * vec3<f32>(-(1.0), 0.0, 1.0)) + ((vec3<f32>(_e285.g1_.z) * _e289.g1_.yxy) * vec3<f32>(1.0, -(1.0), 0.0))) + (vec3<f32>(_e300.g8_.w) * _e304.g1_)) + ((vec3<f32>(_e308.g1_.x) * _e312.g1_.xzy) * vec3<f32>(0.0, 1.0, -(1.0)))), ((((((((((vec2<f32>(_e323.g1_.x) * vec2<f32>(_e327.g0_.x, _e330.g2_.x)) * vec2<f32>(-(1.0), 1.0)) + ((vec2<f32>(_e340.g1_.y) * vec2<f32>(_e344.g0_.y, _e347.g2_.y)) * vec2<f32>(-(1.0), 1.0))) + ((vec2<f32>(_e358.g1_.z) * vec2<f32>(_e362.g0_.z, _e365.g2_.z)) * vec2<f32>(-(1.0), 1.0))) + ((vec2<f32>(_e376.g7_.y) * vec2<f32>(_e380.g1_.y)) * vec2<f32>(0.0, -(1.0)))) + ((vec2<f32>(_e391.g7_.z) * vec2<f32>(_e395.g1_.z)) * vec2<f32>(0.0, -(1.0)))) + ((vec2<f32>(_e406.g8_.x) * vec2<f32>(_e410.g1_.x)) * vec2<f32>(-(1.0), 0.0))) + ((vec2<f32>(_e421.g8_.y) * vec2<f32>(_e425.g1_.y)) * vec2<f32>(-(1.0), 0.0))) + ((vec2<f32>(_e436.g8_.z) * vec2<f32>(_e440.g1_.z)) * vec2<f32>(-(1.0), 0.0))) + ((vec2<f32>(_e451.g7_.x) * vec2<f32>(_e455.g1_.x)) * vec2<f32>(0.0, -(1.0)))), ((((((((((((((vec4<f32>(_e466.g0_.x) * _e470.g2_) + ((vec4<f32>(_e473.g3_.y) * vec4<f32>(_e477.g1_.z, _e480.g1_.z, _e483.g1_.x, _e486.g0_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e500.g3_.z) * vec4<f32>(_e504.g1_.y, _e507.g1_.x, _e510.g1_.y, _e513.g0_.z)) * vec4<f32>(1.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e527.g4_.x) * vec4<f32>(_e531.g2_.x)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e543.g4_.y) * vec4<f32>(_e547.g2_.y)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e559.g4_.z) * vec4<f32>(_e563.g2_.z)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e575.g5_.x) * _e579.g2_.zzyz) * vec4<f32>(0.0, 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e591.g5_.y) * _e595.g2_.zzxz) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0))) + ((vec4<f32>(_e607.g5_.z) * _e611.g2_.yxyy) * vec4<f32>(1.0, -(1.0), 0.0, 0.0))) + ((vec4<f32>(_e623.g9_.x) * vec4<f32>(_e627.g1_.x)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e640.g9_.y) * vec4<f32>(_e644.g1_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e657.g9_.z) * vec4<f32>(_e661.g1_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e674.g9_.w) * vec4<f32>(_e678.g1_.x, _e681.g1_.y, _e684.g1_.z, _e687.g1_.x)) * vec4<f32>(1.0, 1.0, 1.0, 0.0))) + ((vec4<f32>(_e699.g3_.x) * vec4<f32>(_e703.g1_.x, _e706.g1_.z, _e709.g1_.y, _e712.g0_.x)) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))), ((((((((vec3<f32>(_e726.g0_.x) * _e730.g0_) + (vec3<f32>(_e733.g0_.y) * _e737.g1_)) + ((vec3<f32>(_e741.g4_.y) * _e745.g1_.zzx) * vec3<f32>(-(1.0), 0.0, 1.0))) + ((vec3<f32>(_e756.g4_.z) * _e760.g1_.yxy) * vec3<f32>(1.0, -(1.0), 0.0))) + ((vec3<f32>(_e771.g5_.x) * _e775.g0_.zzy) * vec3<f32>(0.0, 1.0, -(1.0)))) + ((vec3<f32>(_e786.g5_.y) * _e790.g0_.zzx) * vec3<f32>(-(1.0), 0.0, 1.0))) + ((vec3<f32>(_e801.g5_.z) * _e805.g0_.yxy) * vec3<f32>(1.0, -(1.0), 0.0))) + ((vec3<f32>(_e816.g4_.x) * _e820.g1_.xzy) * vec3<f32>(0.0, 1.0, -(1.0)))), ((((vec3<f32>(_e831.g0_.x) * _e835.g1_) + ((vec3<f32>(_e838.g5_.y) * _e842.g1_.zzx) * vec3<f32>(-(1.0), 0.0, 1.0))) + ((vec3<f32>(_e853.g5_.z) * _e857.g1_.yxy) * vec3<f32>(1.0, -(1.0), 0.0))) + ((vec3<f32>(_e868.g5_.x) * _e872.g1_.xzy) * vec3<f32>(0.0, 1.0, -(1.0)))), (((((((((((((vec3<f32>(_e883.g0_.z) * _e887.g1_) + (vec3<f32>(_e890.g2_.x) * vec3<f32>(_e894.g2_.x, _e897.g2_.y, _e900.g2_.z))) + (vec3<f32>(_e906.g2_.y) * _e910.g0_)) + ((vec3<f32>(_e914.g6_.x) * _e918.g1_.zzy) * vec3<f32>(0.0, 1.0, -(1.0)))) + ((vec3<f32>(_e929.g6_.y) * _e933.g1_.zzx) * vec3<f32>(-(1.0), 0.0, 1.0))) + ((vec3<f32>(_e944.g6_.z) * _e948.g1_.yxy) * vec3<f32>(1.0, -(1.0), 0.0))) + ((vec3<f32>(_e959.g7_.x) * _e963.g0_.zzy) * vec3<f32>(0.0, 1.0, -(1.0)))) + ((vec3<f32>(_e974.g7_.y) * _e978.g0_.zzx) * vec3<f32>(-(1.0), 0.0, 1.0))) + ((vec3<f32>(_e989.g7_.z) * _e993.g0_.yxy) * vec3<f32>(1.0, -(1.0), 0.0))) + ((vec3<f32>(_e1004.g8_.x) * vec3<f32>(_e1008.g2_.z, _e1011.g2_.z, _e1014.g2_.y)) * vec3<f32>(0.0, 1.0, -(1.0)))) + ((vec3<f32>(_e1026.g8_.y) * vec3<f32>(_e1030.g2_.z, _e1033.g2_.z, _e1036.g2_.x)) * vec3<f32>(-(1.0), 0.0, 1.0))) + ((vec3<f32>(_e1048.g8_.z) * vec3<f32>(_e1052.g2_.y, _e1055.g2_.x, _e1058.g2_.y)) * vec3<f32>(1.0, -(1.0), 0.0))) + ((_e1070.g1_ * vec3<f32>(_e1072.g2_.w)) * vec3<f32>(-(1.0)))), (((((((((vec3<f32>(_e1082.g1_.y) * vec3<f32>(_e1086.g2_.z, _e1089.g2_.z, _e1092.g2_.x)) * vec3<f32>(1.0, 0.0, -(1.0))) + ((vec3<f32>(_e1103.g1_.z) * vec3<f32>(_e1107.g2_.y, _e1110.g2_.x, _e1113.g2_.y)) * vec3<f32>(-(1.0), 1.0, 0.0))) + (vec3<f32>(_e1125.g2_.y) * _e1129.g1_)) + ((vec3<f32>(_e1133.g7_.x) * _e1137.g1_.zzy) * vec3<f32>(0.0, 1.0, -(1.0)))) + ((vec3<f32>(_e1148.g7_.y) * _e1152.g1_.zzx) * vec3<f32>(-(1.0), 0.0, 1.0))) + ((vec3<f32>(_e1163.g7_.z) * _e1167.g1_.yxy) * vec3<f32>(1.0, -(1.0), 0.0))) - (vec3<f32>(_e1178.g8_.w) * vec3<f32>(_e1182.g2_.x, _e1185.g2_.y, _e1188.g2_.z))) + ((vec3<f32>(_e1194.g1_.x) * vec3<f32>(_e1198.g2_.x, _e1201.g2_.z, _e1204.g2_.y)) * vec3<f32>(0.0, -(1.0), 1.0))), (((((((((vec4<f32>(_e1216.g1_.y) * vec4<f32>(_e1220.g0_.z, _e1223.g0_.z, _e1226.g0_.x, _e1229.g1_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e1242.g1_.z) * vec4<f32>(_e1246.g0_.y, _e1249.g0_.x, _e1252.g0_.y, _e1255.g1_.z)) * vec4<f32>(1.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e1269.g2_.x) * vec4<f32>(_e1273.g1_.x, _e1276.g1_.y, _e1279.g1_.z, _e1282.g1_.x)) * vec4<f32>(1.0, 1.0, 1.0, 0.0))) + ((vec4<f32>(_e1294.g8_.x) * vec4<f32>(_e1298.g1_.z, _e1301.g1_.z, _e1304.g1_.y, _e1307.g1_.z)) * vec4<f32>(0.0, 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e1320.g8_.y) * vec4<f32>(_e1324.g1_.z, _e1327.g1_.z, _e1330.g1_.x, _e1333.g1_.z)) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0))) + ((vec4<f32>(_e1346.g8_.z) * vec4<f32>(_e1350.g1_.y, _e1353.g1_.x, _e1356.g1_.y, _e1359.g1_.y)) * vec4<f32>(1.0, -(1.0), 0.0, 0.0))) + ((vec4<f32>(_e1372.g8_.w) * vec4<f32>(_e1376.g0_.x, _e1379.g0_.y, _e1382.g0_.z, _e1385.g0_.x)) * vec4<f32>(1.0, 1.0, 1.0, 0.0))) + ((vec4<f32>(_e1397.g1_.x) * vec4<f32>(_e1401.g0_.x, _e1404.g0_.z, _e1407.g0_.y, _e1410.g1_.x)) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))), ((((((((((((((((vec4<f32>(_e1424.g3_.x) * vec4<f32>(_e1428.g0_.z, _e1431.g0_.z, _e1434.g0_.y, _e1437.g1_.x)) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0))) + ((vec4<f32>(_e1450.g3_.y) * vec4<f32>(_e1454.g0_.z, _e1457.g0_.z, _e1460.g0_.x, _e1463.g1_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e1477.g3_.z) * vec4<f32>(_e1481.g0_.y, _e1484.g0_.x, _e1487.g0_.y, _e1490.g1_.z)) * vec4<f32>(1.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e1504.g3_.w) * vec4<f32>(_e1508.g1_.x, _e1511.g1_.y, _e1514.g1_.z, _e1517.g1_.x)) * vec4<f32>(1.0, 1.0, 1.0, 0.0))) + ((vec4<f32>(_e1529.g4_.x) * _e1533.g2_.zzyz) * vec4<f32>(0.0, -(1.0), 1.0, 0.0))) + ((vec4<f32>(_e1545.g4_.y) * _e1549.g2_.zzxz) * vec4<f32>(1.0, 0.0, -(1.0), 0.0))) + ((vec4<f32>(_e1561.g4_.z) * _e1565.g2_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))) + ((vec4<f32>(_e1577.g5_.x) * _e1581.g2_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e1593.g5_.y) * _e1597.g2_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0)))) + ((vec4<f32>(_e1609.g5_.z) * _e1613.g2_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e1625.g9_.x) * vec4<f32>(_e1629.g1_.z, _e1632.g1_.z, _e1635.g1_.y, _e1638.g1_.z)) * vec4<f32>(0.0, 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e1651.g9_.y) * vec4<f32>(_e1655.g1_.z, _e1658.g1_.z, _e1661.g1_.x, _e1664.g1_.z)) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0))) + ((vec4<f32>(_e1677.g9_.z) * vec4<f32>(_e1681.g1_.y, _e1684.g1_.x, _e1687.g1_.y, _e1690.g1_.y)) * vec4<f32>(1.0, -(1.0), 0.0, 0.0))) + ((vec4<f32>(_e1703.g9_.w) * vec4<f32>(_e1707.g0_.x, _e1710.g0_.y, _e1713.g0_.z, _e1716.g0_.x)) * vec4<f32>(1.0, 1.0, 1.0, 0.0))) + ((vec4<f32>(_e1728.g0_.y, _e1731.g0_.y, _e1734.g0_.y, _e1737.g0_.x) * _e1741.g2_.xyzx) * vec4<f32>(-(1.0), -(1.0), -(1.0), 0.0))));
}

fn multi_vector_dipole_scalar_product(self_1154: MultiVector, other_964: Dipole) -> Scalar {
    var self_1155: MultiVector;
    var other_965: Dipole;

    self_1155 = self_1154;
    other_965 = other_964;
    let _e5: MultiVector = self_1155;
    let _e8: Dipole = other_965;
    let _e13: MultiVector = self_1155;
    let _e16: Dipole = other_965;
    let _e21: MultiVector = self_1155;
    let _e24: Dipole = other_965;
    return Scalar((((0.0 - (_e5.g5_.x * _e8.g1_.x)) - (_e13.g5_.y * _e16.g1_.y)) - (_e21.g5_.z * _e24.g1_.z)));
}

fn multi_vector_line_into(self_1156: MultiVector) -> Line {
    var self_1157: MultiVector;

    self_1157 = self_1156;
    let _e2: MultiVector = self_1157;
    let _e4: MultiVector = self_1157;
    return Line(_e2.g6_, _e4.g7_);
}

fn multi_vector_line_add(self_1158: MultiVector, other_966: Line) -> MultiVector {
    var self_1159: MultiVector;
    var other_967: Line;

    self_1159 = self_1158;
    other_967 = other_966;
    let _e4: MultiVector = self_1159;
    let _e6: MultiVector = self_1159;
    let _e8: MultiVector = self_1159;
    let _e10: MultiVector = self_1159;
    let _e12: MultiVector = self_1159;
    let _e14: MultiVector = self_1159;
    let _e16: MultiVector = self_1159;
    let _e18: Line = other_967;
    let _e21: MultiVector = self_1159;
    let _e23: Line = other_967;
    let _e26: MultiVector = self_1159;
    let _e28: MultiVector = self_1159;
    return MultiVector(_e4.g0_, _e6.g1_, _e8.g2_, _e10.g3_, _e12.g4_, _e14.g5_, (_e16.g6_ + _e18.g0_), (_e21.g7_ + _e23.g1_), _e26.g8_, _e28.g9_);
}

fn multi_vector_line_sub(self_1160: MultiVector, other_968: Line) -> MultiVector {
    var self_1161: MultiVector;
    var other_969: Line;

    self_1161 = self_1160;
    other_969 = other_968;
    let _e4: MultiVector = self_1161;
    let _e6: MultiVector = self_1161;
    let _e8: MultiVector = self_1161;
    let _e10: MultiVector = self_1161;
    let _e12: MultiVector = self_1161;
    let _e14: MultiVector = self_1161;
    let _e16: MultiVector = self_1161;
    let _e18: Line = other_969;
    let _e21: MultiVector = self_1161;
    let _e23: Line = other_969;
    let _e26: MultiVector = self_1161;
    let _e28: MultiVector = self_1161;
    return MultiVector(_e4.g0_, _e6.g1_, _e8.g2_, _e10.g3_, _e12.g4_, _e14.g5_, (_e16.g6_ - _e18.g0_), (_e21.g7_ - _e23.g1_), _e26.g8_, _e28.g9_);
}

fn multi_vector_circle_into(self_1162: MultiVector) -> Circle {
    var self_1163: MultiVector;

    self_1163 = self_1162;
    let _e2: MultiVector = self_1163;
    let _e4: MultiVector = self_1163;
    let _e6: MultiVector = self_1163;
    return Circle(_e2.g8_, _e4.g6_, _e6.g7_);
}

fn multi_vector_circle_add(self_1164: MultiVector, other_970: Circle) -> MultiVector {
    var self_1165: MultiVector;
    var other_971: Circle;

    self_1165 = self_1164;
    other_971 = other_970;
    let _e4: MultiVector = self_1165;
    let _e6: MultiVector = self_1165;
    let _e8: MultiVector = self_1165;
    let _e10: MultiVector = self_1165;
    let _e12: MultiVector = self_1165;
    let _e14: MultiVector = self_1165;
    let _e16: MultiVector = self_1165;
    let _e18: Circle = other_971;
    let _e21: MultiVector = self_1165;
    let _e23: Circle = other_971;
    let _e26: MultiVector = self_1165;
    let _e28: Circle = other_971;
    let _e31: MultiVector = self_1165;
    return MultiVector(_e4.g0_, _e6.g1_, _e8.g2_, _e10.g3_, _e12.g4_, _e14.g5_, (_e16.g6_ + _e18.g1_), (_e21.g7_ + _e23.g2_), (_e26.g8_ + _e28.g0_), _e31.g9_);
}

fn multi_vector_circle_sub(self_1166: MultiVector, other_972: Circle) -> MultiVector {
    var self_1167: MultiVector;
    var other_973: Circle;

    self_1167 = self_1166;
    other_973 = other_972;
    let _e4: MultiVector = self_1167;
    let _e6: MultiVector = self_1167;
    let _e8: MultiVector = self_1167;
    let _e10: MultiVector = self_1167;
    let _e12: MultiVector = self_1167;
    let _e14: MultiVector = self_1167;
    let _e16: MultiVector = self_1167;
    let _e18: Circle = other_973;
    let _e21: MultiVector = self_1167;
    let _e23: Circle = other_973;
    let _e26: MultiVector = self_1167;
    let _e28: Circle = other_973;
    let _e31: MultiVector = self_1167;
    return MultiVector(_e4.g0_, _e6.g1_, _e8.g2_, _e10.g3_, _e12.g4_, _e14.g5_, (_e16.g6_ - _e18.g1_), (_e21.g7_ - _e23.g2_), (_e26.g8_ - _e28.g0_), _e31.g9_);
}

fn multi_vector_circle_geometric_product(self_1168: MultiVector, other_974: Circle) -> MultiVector {
    var self_1169: MultiVector;
    var other_975: Circle;

    self_1169 = self_1168;
    other_975 = other_974;
    let _e4: MultiVector = self_1169;
    let _e8: Circle = other_975;
    let _e18: MultiVector = self_1169;
    let _e22: Circle = other_975;
    let _e33: MultiVector = self_1169;
    let _e37: Circle = other_975;
    let _e48: MultiVector = self_1169;
    let _e52: Circle = other_975;
    let _e64: MultiVector = self_1169;
    let _e68: Circle = other_975;
    let _e80: MultiVector = self_1169;
    let _e84: Circle = other_975;
    let _e96: MultiVector = self_1169;
    let _e100: Circle = other_975;
    let _e112: MultiVector = self_1169;
    let _e116: Circle = other_975;
    let _e128: MultiVector = self_1169;
    let _e132: Circle = other_975;
    let _e144: MultiVector = self_1169;
    let _e148: Circle = other_975;
    let _e160: MultiVector = self_1169;
    let _e164: Circle = other_975;
    let _e176: MultiVector = self_1169;
    let _e180: Circle = other_975;
    let _e192: MultiVector = self_1169;
    let _e196: Circle = other_975;
    let _e208: MultiVector = self_1169;
    let _e212: Circle = other_975;
    let _e224: MultiVector = self_1169;
    let _e228: Circle = other_975;
    let _e239: MultiVector = self_1169;
    let _e241: Circle = other_975;
    let _e248: MultiVector = self_1169;
    let _e252: Circle = other_975;
    let _e255: Circle = other_975;
    let _e261: MultiVector = self_1169;
    let _e265: Circle = other_975;
    let _e268: Circle = other_975;
    let _e274: MultiVector = self_1169;
    let _e278: Circle = other_975;
    let _e281: Circle = other_975;
    let _e287: MultiVector = self_1169;
    let _e291: Circle = other_975;
    let _e301: MultiVector = self_1169;
    let _e304: MultiVector = self_1169;
    let _e308: Circle = other_975;
    let _e311: Circle = other_975;
    let _e322: MultiVector = self_1169;
    let _e326: Circle = other_975;
    let _e329: Circle = other_975;
    let _e332: Circle = other_975;
    let _e335: Circle = other_975;
    let _e348: MultiVector = self_1169;
    let _e352: Circle = other_975;
    let _e355: Circle = other_975;
    let _e358: Circle = other_975;
    let _e361: Circle = other_975;
    let _e375: MultiVector = self_1169;
    let _e379: Circle = other_975;
    let _e382: Circle = other_975;
    let _e385: Circle = other_975;
    let _e388: Circle = other_975;
    let _e402: MultiVector = self_1169;
    let _e406: Circle = other_975;
    let _e418: MultiVector = self_1169;
    let _e422: Circle = other_975;
    let _e434: MultiVector = self_1169;
    let _e438: Circle = other_975;
    let _e450: MultiVector = self_1169;
    let _e454: Circle = other_975;
    let _e467: MultiVector = self_1169;
    let _e471: Circle = other_975;
    let _e484: MultiVector = self_1169;
    let _e488: Circle = other_975;
    let _e501: MultiVector = self_1169;
    let _e505: Circle = other_975;
    let _e508: Circle = other_975;
    let _e511: Circle = other_975;
    let _e514: Circle = other_975;
    let _e526: MultiVector = self_1169;
    let _e529: MultiVector = self_1169;
    let _e532: MultiVector = self_1169;
    let _e535: MultiVector = self_1169;
    let _e539: Circle = other_975;
    let _e550: MultiVector = self_1169;
    let _e554: Circle = other_975;
    let _e557: Circle = other_975;
    let _e560: Circle = other_975;
    let _e571: MultiVector = self_1169;
    let _e575: Circle = other_975;
    let _e578: Circle = other_975;
    let _e581: Circle = other_975;
    let _e593: MultiVector = self_1169;
    let _e597: Circle = other_975;
    let _e608: MultiVector = self_1169;
    let _e612: Circle = other_975;
    let _e623: MultiVector = self_1169;
    let _e627: Circle = other_975;
    let _e638: MultiVector = self_1169;
    let _e642: Circle = other_975;
    let _e645: Circle = other_975;
    let _e648: Circle = other_975;
    let _e654: MultiVector = self_1169;
    let _e658: Circle = other_975;
    let _e661: Circle = other_975;
    let _e664: Circle = other_975;
    let _e676: MultiVector = self_1169;
    let _e678: Circle = other_975;
    let _e687: MultiVector = self_1169;
    let _e691: Circle = other_975;
    let _e694: MultiVector = self_1169;
    let _e698: Circle = other_975;
    let _e702: MultiVector = self_1169;
    let _e706: Circle = other_975;
    let _e709: Circle = other_975;
    let _e712: Circle = other_975;
    let _e724: MultiVector = self_1169;
    let _e728: Circle = other_975;
    let _e731: Circle = other_975;
    let _e734: Circle = other_975;
    let _e746: MultiVector = self_1169;
    let _e750: Circle = other_975;
    let _e761: MultiVector = self_1169;
    let _e765: Circle = other_975;
    let _e776: MultiVector = self_1169;
    let _e780: Circle = other_975;
    let _e791: MultiVector = self_1169;
    let _e795: Circle = other_975;
    let _e806: MultiVector = self_1169;
    let _e810: Circle = other_975;
    let _e821: MultiVector = self_1169;
    let _e825: Circle = other_975;
    let _e836: MultiVector = self_1169;
    let _e840: Circle = other_975;
    let _e852: MultiVector = self_1169;
    let _e856: Circle = other_975;
    let _e868: MultiVector = self_1169;
    let _e872: Circle = other_975;
    let _e875: Circle = other_975;
    let _e878: Circle = other_975;
    let _e884: MultiVector = self_1169;
    let _e887: MultiVector = self_1169;
    let _e890: MultiVector = self_1169;
    let _e894: Circle = other_975;
    let _e897: Circle = other_975;
    let _e900: Circle = other_975;
    let _e913: MultiVector = self_1169;
    let _e917: Circle = other_975;
    let _e920: MultiVector = self_1169;
    let _e924: Circle = other_975;
    let _e935: MultiVector = self_1169;
    let _e939: Circle = other_975;
    let _e950: MultiVector = self_1169;
    let _e954: Circle = other_975;
    let _e965: MultiVector = self_1169;
    let _e968: MultiVector = self_1169;
    let _e971: MultiVector = self_1169;
    let _e975: Circle = other_975;
    let _e981: MultiVector = self_1169;
    let _e985: Circle = other_975;
    let _e988: MultiVector = self_1169;
    let _e992: Circle = other_975;
    let _e1004: MultiVector = self_1169;
    let _e1008: Circle = other_975;
    let _e1020: MultiVector = self_1169;
    let _e1024: Circle = other_975;
    let _e1036: MultiVector = self_1169;
    let _e1039: MultiVector = self_1169;
    let _e1042: MultiVector = self_1169;
    let _e1045: MultiVector = self_1169;
    let _e1049: Circle = other_975;
    let _e1063: MultiVector = self_1169;
    let _e1067: Circle = other_975;
    let _e1070: Circle = other_975;
    let _e1073: Circle = other_975;
    let _e1076: Circle = other_975;
    let _e1089: MultiVector = self_1169;
    let _e1093: Circle = other_975;
    let _e1096: Circle = other_975;
    let _e1099: Circle = other_975;
    let _e1102: Circle = other_975;
    let _e1116: MultiVector = self_1169;
    let _e1120: Circle = other_975;
    let _e1123: Circle = other_975;
    let _e1126: Circle = other_975;
    let _e1129: Circle = other_975;
    let _e1141: MultiVector = self_1169;
    let _e1145: Circle = other_975;
    let _e1149: MultiVector = self_1169;
    let _e1153: Circle = other_975;
    let _e1165: MultiVector = self_1169;
    let _e1169: Circle = other_975;
    let _e1181: MultiVector = self_1169;
    let _e1185: Circle = other_975;
    let _e1197: MultiVector = self_1169;
    let _e1201: Circle = other_975;
    let _e1213: MultiVector = self_1169;
    let _e1217: Circle = other_975;
    let _e1229: MultiVector = self_1169;
    let _e1233: Circle = other_975;
    let _e1245: MultiVector = self_1169;
    let _e1249: Circle = other_975;
    let _e1252: Circle = other_975;
    let _e1255: Circle = other_975;
    let _e1258: Circle = other_975;
    let _e1271: MultiVector = self_1169;
    let _e1275: Circle = other_975;
    let _e1278: Circle = other_975;
    let _e1281: Circle = other_975;
    let _e1284: Circle = other_975;
    let _e1297: MultiVector = self_1169;
    let _e1301: Circle = other_975;
    let _e1304: Circle = other_975;
    let _e1307: Circle = other_975;
    let _e1310: Circle = other_975;
    let _e1323: MultiVector = self_1169;
    let _e1327: Circle = other_975;
    let _e1330: Circle = other_975;
    let _e1333: Circle = other_975;
    let _e1336: Circle = other_975;
    let _e1348: MultiVector = self_1169;
    let _e1352: Circle = other_975;
    let _e1355: Circle = other_975;
    let _e1358: Circle = other_975;
    let _e1361: Circle = other_975;
    return MultiVector(((((((((((((((((vec3<f32>(_e4.g1_.y) * vec3<f32>(_e8.g0_.y)) * vec3<f32>(0.0, 1.0, 0.0)) + ((vec3<f32>(_e18.g1_.z) * vec3<f32>(_e22.g0_.z)) * vec3<f32>(0.0, 1.0, 0.0))) + ((vec3<f32>(_e33.g2_.x) * vec3<f32>(_e37.g0_.w)) * vec3<f32>(0.0, 1.0, 0.0))) + ((vec3<f32>(_e48.g3_.x) * vec3<f32>(_e52.g0_.x)) * vec3<f32>(0.0, 0.0, -(1.0)))) + ((vec3<f32>(_e64.g3_.y) * vec3<f32>(_e68.g0_.y)) * vec3<f32>(0.0, 0.0, -(1.0)))) + ((vec3<f32>(_e80.g3_.z) * vec3<f32>(_e84.g0_.z)) * vec3<f32>(0.0, 0.0, -(1.0)))) + ((vec3<f32>(_e96.g3_.w) * vec3<f32>(_e100.g0_.w)) * vec3<f32>(0.0, 0.0, -(1.0)))) + ((vec3<f32>(_e112.g4_.x) * vec3<f32>(_e116.g2_.x)) * vec3<f32>(0.0, 0.0, -(1.0)))) + ((vec3<f32>(_e128.g4_.y) * vec3<f32>(_e132.g2_.y)) * vec3<f32>(0.0, 0.0, -(1.0)))) + ((vec3<f32>(_e144.g4_.z) * vec3<f32>(_e148.g2_.z)) * vec3<f32>(0.0, 0.0, -(1.0)))) + ((vec3<f32>(_e160.g5_.x) * vec3<f32>(_e164.g1_.x)) * vec3<f32>(0.0, 0.0, -(1.0)))) + ((vec3<f32>(_e176.g5_.y) * vec3<f32>(_e180.g1_.y)) * vec3<f32>(0.0, 0.0, -(1.0)))) + ((vec3<f32>(_e192.g5_.z) * vec3<f32>(_e196.g1_.z)) * vec3<f32>(0.0, 0.0, -(1.0)))) + ((vec3<f32>(_e208.g8_.w) * vec3<f32>(_e212.g0_.w)) * vec3<f32>(-(1.0), 0.0, 0.0))) + ((vec3<f32>(_e224.g1_.x) * vec3<f32>(_e228.g0_.x)) * vec3<f32>(0.0, 1.0, 0.0))), (_e239.g5_ * vec3<f32>(_e241.g0_.w)), (((((vec2<f32>(0.0) - (vec2<f32>(_e248.g5_.x) * vec2<f32>(_e252.g0_.x, _e255.g2_.x))) - (vec2<f32>(_e261.g5_.y) * vec2<f32>(_e265.g0_.y, _e268.g2_.y))) - (vec2<f32>(_e274.g5_.z) * vec2<f32>(_e278.g0_.z, _e281.g2_.z))) + ((vec2<f32>(_e287.g9_.w) * vec2<f32>(_e291.g0_.w)) * vec2<f32>(0.0, 1.0))) + ((vec2<f32>(_e301.g0_.y, _e304.g0_.x) * vec2<f32>(_e308.g0_.w, _e311.g0_.x)) * vec2<f32>(-(1.0), 0.0))), ((((((((((((vec4<f32>(_e322.g1_.x) * vec4<f32>(_e326.g2_.z, _e329.g2_.z, _e332.g2_.y, _e335.g1_.x)) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0))) + ((vec4<f32>(_e348.g1_.y) * vec4<f32>(_e352.g2_.z, _e355.g2_.z, _e358.g2_.x, _e361.g1_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e375.g1_.z) * vec4<f32>(_e379.g2_.y, _e382.g2_.x, _e385.g2_.y, _e388.g1_.z)) * vec4<f32>(1.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e402.g7_.x) * _e406.g0_.wwwx) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))) + ((vec4<f32>(_e418.g7_.y) * _e422.g0_.wwwy) * vec4<f32>(0.0, -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e434.g7_.z) * _e438.g0_.wwwz) * vec4<f32>(0.0, 0.0, -(1.0), 1.0))) + ((vec4<f32>(_e450.g8_.x) * vec4<f32>(_e454.g2_.x)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e467.g8_.y) * vec4<f32>(_e471.g2_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e484.g8_.z) * vec4<f32>(_e488.g2_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e501.g8_.w) * vec4<f32>(_e505.g2_.x, _e508.g2_.y, _e511.g2_.z, _e514.g2_.x)) * vec4<f32>(1.0, 1.0, 1.0, 0.0))) + ((vec4<f32>(_e526.g0_.x, _e529.g0_.x, _e532.g0_.x, _e535.g0_.z) * _e539.g0_.xxxw) * vec4<f32>(0.0, 0.0, 0.0, 1.0))), ((((((((vec3<f32>(_e550.g1_.y) * vec3<f32>(_e554.g0_.z, _e557.g0_.z, _e560.g0_.x)) * vec3<f32>(1.0, 0.0, -(1.0))) + ((vec3<f32>(_e571.g1_.z) * vec3<f32>(_e575.g0_.y, _e578.g0_.x, _e581.g0_.y)) * vec3<f32>(-(1.0), 1.0, 0.0))) + ((vec3<f32>(_e593.g8_.x) * vec3<f32>(_e597.g0_.w)) * vec3<f32>(1.0, 0.0, 0.0))) + ((vec3<f32>(_e608.g8_.y) * vec3<f32>(_e612.g0_.w)) * vec3<f32>(0.0, 1.0, 0.0))) + ((vec3<f32>(_e623.g8_.z) * vec3<f32>(_e627.g0_.w)) * vec3<f32>(0.0, 0.0, 1.0))) - (vec3<f32>(_e638.g8_.w) * vec3<f32>(_e642.g0_.x, _e645.g0_.y, _e648.g0_.z))) + ((vec3<f32>(_e654.g1_.x) * vec3<f32>(_e658.g0_.x, _e661.g0_.z, _e664.g0_.y)) * vec3<f32>(0.0, -(1.0), 1.0))), ((_e676.g1_ * vec3<f32>(_e678.g0_.w)) * vec3<f32>(-(1.0))), ((((((((((((((vec3<f32>(_e687.g0_.x) * _e691.g1_) + (vec3<f32>(_e694.g0_.y) * _e698.g2_)) + ((vec3<f32>(_e702.g3_.y) * vec3<f32>(_e706.g0_.z, _e709.g0_.z, _e712.g0_.x)) * vec3<f32>(-(1.0), 0.0, 1.0))) + ((vec3<f32>(_e724.g3_.z) * vec3<f32>(_e728.g0_.y, _e731.g0_.x, _e734.g0_.y)) * vec3<f32>(1.0, -(1.0), 0.0))) + ((vec3<f32>(_e746.g4_.x) * _e750.g2_.zzy) * vec3<f32>(0.0, 1.0, -(1.0)))) + ((vec3<f32>(_e761.g4_.y) * _e765.g2_.zzx) * vec3<f32>(-(1.0), 0.0, 1.0))) + ((vec3<f32>(_e776.g4_.z) * _e780.g2_.yxy) * vec3<f32>(1.0, -(1.0), 0.0))) + ((vec3<f32>(_e791.g5_.x) * _e795.g1_.zzy) * vec3<f32>(0.0, 1.0, -(1.0)))) + ((vec3<f32>(_e806.g5_.y) * _e810.g1_.zzx) * vec3<f32>(-(1.0), 0.0, 1.0))) + ((vec3<f32>(_e821.g5_.z) * _e825.g1_.yxy) * vec3<f32>(1.0, -(1.0), 0.0))) + ((vec3<f32>(_e836.g9_.y) * vec3<f32>(_e840.g0_.w)) * vec3<f32>(0.0, -(1.0), 0.0))) + ((vec3<f32>(_e852.g9_.z) * vec3<f32>(_e856.g0_.w)) * vec3<f32>(0.0, 0.0, -(1.0)))) + (vec3<f32>(_e868.g9_.w) * vec3<f32>(_e872.g0_.x, _e875.g0_.y, _e878.g0_.z))) + ((vec3<f32>(_e884.g9_.x, _e887.g3_.x, _e890.g3_.x) * vec3<f32>(_e894.g0_.w, _e897.g0_.z, _e900.g0_.y)) * vec3<f32>(-(1.0), 1.0, -(1.0)))), (((((vec3<f32>(_e913.g0_.x) * _e917.g2_) + ((vec3<f32>(_e920.g5_.x) * _e924.g2_.zzy) * vec3<f32>(0.0, 1.0, -(1.0)))) + ((vec3<f32>(_e935.g5_.y) * _e939.g2_.zzx) * vec3<f32>(-(1.0), 0.0, 1.0))) + ((vec3<f32>(_e950.g5_.z) * _e954.g2_.yxy) * vec3<f32>(1.0, -(1.0), 0.0))) + (vec3<f32>(_e965.g3_.x, _e968.g3_.y, _e971.g3_.z) * vec3<f32>(_e975.g0_.w))), (((((vec4<f32>(_e981.g0_.x) * _e985.g0_) + ((vec4<f32>(_e988.g5_.x) * _e992.g0_.zzyz) * vec4<f32>(0.0, 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e1004.g5_.y) * _e1008.g0_.zzxz) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0))) + ((vec4<f32>(_e1020.g5_.z) * _e1024.g0_.yxyy) * vec4<f32>(1.0, -(1.0), 0.0, 0.0))) + ((vec4<f32>(_e1036.g4_.x, _e1039.g4_.y, _e1042.g4_.z, _e1045.g4_.x) * _e1049.g0_.wwwx) * vec4<f32>(-(1.0), -(1.0), -(1.0), 0.0))), ((((((((((((((((vec4<f32>(_e1063.g1_.y) * vec4<f32>(_e1067.g1_.z, _e1070.g1_.z, _e1073.g1_.x, _e1076.g2_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e1089.g1_.z) * vec4<f32>(_e1093.g1_.y, _e1096.g1_.x, _e1099.g1_.y, _e1102.g2_.z)) * vec4<f32>(1.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e1116.g2_.x) * vec4<f32>(_e1120.g2_.x, _e1123.g2_.y, _e1126.g2_.z, _e1129.g2_.x)) * vec4<f32>(1.0, 1.0, 1.0, 0.0))) - (vec4<f32>(_e1141.g2_.y) * _e1145.g0_)) + ((vec4<f32>(_e1149.g6_.x) * vec4<f32>(_e1153.g0_.w)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e1165.g6_.y) * vec4<f32>(_e1169.g0_.w)) * vec4<f32>(0.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e1181.g6_.z) * vec4<f32>(_e1185.g0_.w)) * vec4<f32>(0.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e1197.g7_.x) * _e1201.g0_.zzyz) * vec4<f32>(0.0, -(1.0), 1.0, 0.0))) + ((vec4<f32>(_e1213.g7_.y) * _e1217.g0_.zzxz) * vec4<f32>(1.0, 0.0, -(1.0), 0.0))) + ((vec4<f32>(_e1229.g7_.z) * _e1233.g0_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))) + ((vec4<f32>(_e1245.g8_.x) * vec4<f32>(_e1249.g2_.z, _e1252.g2_.z, _e1255.g2_.y, _e1258.g2_.z)) * vec4<f32>(0.0, 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e1271.g8_.y) * vec4<f32>(_e1275.g2_.z, _e1278.g2_.z, _e1281.g2_.x, _e1284.g2_.z)) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0))) + ((vec4<f32>(_e1297.g8_.z) * vec4<f32>(_e1301.g2_.y, _e1304.g2_.x, _e1307.g2_.y, _e1310.g2_.y)) * vec4<f32>(1.0, -(1.0), 0.0, 0.0))) + ((vec4<f32>(_e1323.g8_.w) * vec4<f32>(_e1327.g1_.x, _e1330.g1_.y, _e1333.g1_.z, _e1336.g1_.x)) * vec4<f32>(1.0, 1.0, 1.0, 0.0))) + ((vec4<f32>(_e1348.g1_.x) * vec4<f32>(_e1352.g1_.x, _e1355.g1_.z, _e1358.g1_.y, _e1361.g2_.x)) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))));
}

fn multi_vector_circle_scalar_product(self_1170: MultiVector, other_976: Circle) -> Scalar {
    var self_1171: MultiVector;
    var other_977: Circle;

    self_1171 = self_1170;
    other_977 = other_976;
    let _e5: MultiVector = self_1171;
    let _e8: Circle = other_977;
    return Scalar((0.0 - (_e5.g8_.w * _e8.g0_.w)));
}

fn multi_vector_plane_into(self_1172: MultiVector) -> Plane {
    var self_1173: MultiVector;

    self_1173 = self_1172;
    let _e2: MultiVector = self_1173;
    return Plane(_e2.g9_);
}

fn multi_vector_plane_add(self_1174: MultiVector, other_978: Plane) -> MultiVector {
    var self_1175: MultiVector;
    var other_979: Plane;

    self_1175 = self_1174;
    other_979 = other_978;
    let _e4: MultiVector = self_1175;
    let _e6: MultiVector = self_1175;
    let _e8: MultiVector = self_1175;
    let _e10: MultiVector = self_1175;
    let _e12: MultiVector = self_1175;
    let _e14: MultiVector = self_1175;
    let _e16: MultiVector = self_1175;
    let _e18: MultiVector = self_1175;
    let _e20: MultiVector = self_1175;
    let _e22: MultiVector = self_1175;
    let _e24: Plane = other_979;
    return MultiVector(_e4.g0_, _e6.g1_, _e8.g2_, _e10.g3_, _e12.g4_, _e14.g5_, _e16.g6_, _e18.g7_, _e20.g8_, (_e22.g9_ + _e24.g0_));
}

fn multi_vector_plane_sub(self_1176: MultiVector, other_980: Plane) -> MultiVector {
    var self_1177: MultiVector;
    var other_981: Plane;

    self_1177 = self_1176;
    other_981 = other_980;
    let _e4: MultiVector = self_1177;
    let _e6: MultiVector = self_1177;
    let _e8: MultiVector = self_1177;
    let _e10: MultiVector = self_1177;
    let _e12: MultiVector = self_1177;
    let _e14: MultiVector = self_1177;
    let _e16: MultiVector = self_1177;
    let _e18: MultiVector = self_1177;
    let _e20: MultiVector = self_1177;
    let _e22: MultiVector = self_1177;
    let _e24: Plane = other_981;
    return MultiVector(_e4.g0_, _e6.g1_, _e8.g2_, _e10.g3_, _e12.g4_, _e14.g5_, _e16.g6_, _e18.g7_, _e20.g8_, (_e22.g9_ - _e24.g0_));
}

fn multi_vector_sphere_into(self_1178: MultiVector) -> Sphere {
    var self_1179: MultiVector;

    self_1179 = self_1178;
    let _e2: MultiVector = self_1179;
    let _e5: MultiVector = self_1179;
    let _e9: MultiVector = self_1179;
    let _e12: MultiVector = self_1179;
    let _e15: MultiVector = self_1179;
    return Sphere(vec2<f32>(_e2.g0_.y, _e5.g9_.w), vec3<f32>(_e9.g9_.x, _e12.g9_.y, _e15.g9_.z));
}

fn multi_vector_sphere_add(self_1180: MultiVector, other_982: Sphere) -> MultiVector {
    var self_1181: MultiVector;
    var other_983: Sphere;

    self_1181 = self_1180;
    other_983 = other_982;
    let _e4: MultiVector = self_1181;
    let _e6: Sphere = other_983;
    let _e16: MultiVector = self_1181;
    let _e18: MultiVector = self_1181;
    let _e20: MultiVector = self_1181;
    let _e22: MultiVector = self_1181;
    let _e24: MultiVector = self_1181;
    let _e26: MultiVector = self_1181;
    let _e28: MultiVector = self_1181;
    let _e30: MultiVector = self_1181;
    let _e32: MultiVector = self_1181;
    let _e34: Sphere = other_983;
    let _e37: Sphere = other_983;
    let _e40: Sphere = other_983;
    let _e43: Sphere = other_983;
    return MultiVector((_e4.g0_ + (vec3<f32>(_e6.g0_.x) * vec3<f32>(0.0, 1.0, 0.0))), _e16.g1_, _e18.g2_, _e20.g3_, _e22.g4_, _e24.g5_, _e26.g6_, _e28.g7_, _e30.g8_, (_e32.g9_ + vec4<f32>(_e34.g1_.x, _e37.g1_.y, _e40.g1_.z, _e43.g0_.y)));
}

fn multi_vector_sphere_sub(self_1182: MultiVector, other_984: Sphere) -> MultiVector {
    var self_1183: MultiVector;
    var other_985: Sphere;

    self_1183 = self_1182;
    other_985 = other_984;
    let _e4: MultiVector = self_1183;
    let _e6: Sphere = other_985;
    let _e16: MultiVector = self_1183;
    let _e18: MultiVector = self_1183;
    let _e20: MultiVector = self_1183;
    let _e22: MultiVector = self_1183;
    let _e24: MultiVector = self_1183;
    let _e26: MultiVector = self_1183;
    let _e28: MultiVector = self_1183;
    let _e30: MultiVector = self_1183;
    let _e32: MultiVector = self_1183;
    let _e34: Sphere = other_985;
    let _e37: Sphere = other_985;
    let _e40: Sphere = other_985;
    let _e43: Sphere = other_985;
    return MultiVector((_e4.g0_ - (vec3<f32>(_e6.g0_.x) * vec3<f32>(0.0, 1.0, 0.0))), _e16.g1_, _e18.g2_, _e20.g3_, _e22.g4_, _e24.g5_, _e26.g6_, _e28.g7_, _e30.g8_, (_e32.g9_ - vec4<f32>(_e34.g1_.x, _e37.g1_.y, _e40.g1_.z, _e43.g0_.y)));
}

fn multi_vector_motor_into(self_1184: MultiVector) -> Motor {
    var self_1185: MultiVector;

    self_1185 = self_1184;
    let _e2: MultiVector = self_1185;
    let _e5: MultiVector = self_1185;
    let _e8: MultiVector = self_1185;
    let _e11: MultiVector = self_1185;
    let _e15: MultiVector = self_1185;
    let _e18: MultiVector = self_1185;
    let _e21: MultiVector = self_1185;
    let _e24: MultiVector = self_1185;
    return Motor(vec4<f32>(_e2.g6_.x, _e5.g6_.y, _e8.g6_.z, _e11.g0_.z), vec4<f32>(_e15.g7_.x, _e18.g7_.y, _e21.g7_.z, _e24.g2_.y));
}

fn multi_vector_motor_add(self_1186: MultiVector, other_986: Motor) -> MultiVector {
    var self_1187: MultiVector;
    var other_987: Motor;

    self_1187 = self_1186;
    other_987 = other_986;
    let _e4: MultiVector = self_1187;
    let _e6: Motor = other_987;
    let _e9: Motor = other_987;
    let _e12: Motor = other_987;
    let _e22: MultiVector = self_1187;
    let _e24: MultiVector = self_1187;
    let _e26: Motor = other_987;
    let _e29: Motor = other_987;
    let _e38: MultiVector = self_1187;
    let _e40: MultiVector = self_1187;
    let _e42: MultiVector = self_1187;
    let _e44: MultiVector = self_1187;
    let _e46: Motor = other_987;
    let _e49: Motor = other_987;
    let _e52: Motor = other_987;
    let _e57: MultiVector = self_1187;
    let _e59: Motor = other_987;
    let _e62: Motor = other_987;
    let _e65: Motor = other_987;
    let _e70: MultiVector = self_1187;
    let _e72: MultiVector = self_1187;
    return MultiVector((_e4.g0_ + (vec3<f32>(_e6.g0_.x, _e9.g0_.x, _e12.g0_.w) * vec3<f32>(0.0, 0.0, 1.0))), _e22.g1_, (_e24.g2_ + (vec2<f32>(_e26.g0_.x, _e29.g1_.w) * vec2<f32>(0.0, 1.0))), _e38.g3_, _e40.g4_, _e42.g5_, (_e44.g6_ + vec3<f32>(_e46.g0_.x, _e49.g0_.y, _e52.g0_.z)), (_e57.g7_ + vec3<f32>(_e59.g1_.x, _e62.g1_.y, _e65.g1_.z)), _e70.g8_, _e72.g9_);
}

fn multi_vector_motor_sub(self_1188: MultiVector, other_988: Motor) -> MultiVector {
    var self_1189: MultiVector;
    var other_989: Motor;

    self_1189 = self_1188;
    other_989 = other_988;
    let _e4: MultiVector = self_1189;
    let _e6: Motor = other_989;
    let _e9: Motor = other_989;
    let _e12: Motor = other_989;
    let _e22: MultiVector = self_1189;
    let _e24: MultiVector = self_1189;
    let _e26: Motor = other_989;
    let _e29: Motor = other_989;
    let _e38: MultiVector = self_1189;
    let _e40: MultiVector = self_1189;
    let _e42: MultiVector = self_1189;
    let _e44: MultiVector = self_1189;
    let _e46: Motor = other_989;
    let _e49: Motor = other_989;
    let _e52: Motor = other_989;
    let _e57: MultiVector = self_1189;
    let _e59: Motor = other_989;
    let _e62: Motor = other_989;
    let _e65: Motor = other_989;
    let _e70: MultiVector = self_1189;
    let _e72: MultiVector = self_1189;
    return MultiVector((_e4.g0_ - (vec3<f32>(_e6.g0_.x, _e9.g0_.x, _e12.g0_.w) * vec3<f32>(0.0, 0.0, 1.0))), _e22.g1_, (_e24.g2_ - (vec2<f32>(_e26.g0_.x, _e29.g1_.w) * vec2<f32>(0.0, 1.0))), _e38.g3_, _e40.g4_, _e42.g5_, (_e44.g6_ - vec3<f32>(_e46.g0_.x, _e49.g0_.y, _e52.g0_.z)), (_e57.g7_ - vec3<f32>(_e59.g1_.x, _e62.g1_.y, _e65.g1_.z)), _e70.g8_, _e72.g9_);
}

fn multi_vector_motor_regressive_product(self_1190: MultiVector, other_990: Motor) -> MultiVector {
    var self_1191: MultiVector;
    var other_991: Motor;

    self_1191 = self_1190;
    other_991 = other_990;
    let _e4: MultiVector = self_1191;
    let _e8: Motor = other_991;
    let _e11: Motor = other_991;
    let _e14: Motor = other_991;
    let _e24: MultiVector = self_1191;
    let _e28: Motor = other_991;
    let _e40: MultiVector = self_1191;
    let _e44: Motor = other_991;
    let _e56: MultiVector = self_1191;
    let _e60: Motor = other_991;
    let _e72: MultiVector = self_1191;
    let _e76: Motor = other_991;
    let _e88: MultiVector = self_1191;
    let _e92: Motor = other_991;
    let _e104: MultiVector = self_1191;
    let _e108: Motor = other_991;
    let _e120: MultiVector = self_1191;
    let _e123: Motor = other_991;
    let _e126: Motor = other_991;
    let _e129: Motor = other_991;
    let _e140: MultiVector = self_1191;
    let _e144: Motor = other_991;
    let _e147: Motor = other_991;
    let _e150: Motor = other_991;
    let _e161: MultiVector = self_1191;
    let _e165: Motor = other_991;
    let _e168: Motor = other_991;
    let _e171: Motor = other_991;
    let _e183: MultiVector = self_1191;
    let _e187: Motor = other_991;
    let _e190: Motor = other_991;
    let _e193: Motor = other_991;
    let _e205: MultiVector = self_1191;
    let _e209: Motor = other_991;
    let _e212: Motor = other_991;
    let _e215: Motor = other_991;
    let _e221: MultiVector = self_1191;
    let _e223: Motor = other_991;
    let _e229: MultiVector = self_1191;
    let _e233: Motor = other_991;
    let _e242: MultiVector = self_1191;
    let _e246: Motor = other_991;
    let _e256: MultiVector = self_1191;
    let _e260: Motor = other_991;
    let _e271: MultiVector = self_1191;
    let _e275: Motor = other_991;
    let _e286: MultiVector = self_1191;
    let _e290: Motor = other_991;
    let _e301: MultiVector = self_1191;
    let _e305: Motor = other_991;
    let _e316: MultiVector = self_1191;
    let _e320: Motor = other_991;
    let _e331: MultiVector = self_1191;
    let _e335: Motor = other_991;
    let _e346: MultiVector = self_1191;
    let _e350: Motor = other_991;
    let _e361: MultiVector = self_1191;
    let _e365: Motor = other_991;
    let _e376: MultiVector = self_1191;
    let _e380: Motor = other_991;
    let _e391: MultiVector = self_1191;
    let _e394: MultiVector = self_1191;
    let _e398: Motor = other_991;
    let _e401: Motor = other_991;
    let _e411: MultiVector = self_1191;
    let _e415: Motor = other_991;
    let _e418: Motor = other_991;
    let _e421: Motor = other_991;
    let _e424: Motor = other_991;
    let _e437: MultiVector = self_1191;
    let _e441: Motor = other_991;
    let _e444: Motor = other_991;
    let _e447: Motor = other_991;
    let _e450: Motor = other_991;
    let _e464: MultiVector = self_1191;
    let _e468: Motor = other_991;
    let _e471: Motor = other_991;
    let _e474: Motor = other_991;
    let _e477: Motor = other_991;
    let _e491: MultiVector = self_1191;
    let _e495: Motor = other_991;
    let _e506: MultiVector = self_1191;
    let _e508: Motor = other_991;
    let _e514: MultiVector = self_1191;
    let _e518: Motor = other_991;
    let _e521: Motor = other_991;
    let _e524: Motor = other_991;
    let _e529: MultiVector = self_1191;
    let _e531: Motor = other_991;
    let _e537: MultiVector = self_1191;
    let _e541: Motor = other_991;
    let _e544: Motor = other_991;
    let _e547: Motor = other_991;
    let _e552: MultiVector = self_1191;
    let _e554: Motor = other_991;
    let _e560: MultiVector = self_1191;
    let _e564: Motor = other_991;
    let _e567: Motor = other_991;
    let _e570: Motor = other_991;
    let _e575: MultiVector = self_1191;
    let _e577: Motor = other_991;
    let _e583: MultiVector = self_1191;
    let _e587: Motor = other_991;
    let _e590: Motor = other_991;
    let _e593: Motor = other_991;
    let _e598: MultiVector = self_1191;
    let _e600: Motor = other_991;
    let _e606: MultiVector = self_1191;
    let _e608: Motor = other_991;
    let _e613: MultiVector = self_1191;
    let _e615: Motor = other_991;
    return MultiVector((((((((((vec3<f32>(_e4.g0_.y) * vec3<f32>(_e8.g1_.w, _e11.g0_.w, _e14.g1_.w)) * vec3<f32>(1.0, 1.0, 0.0)) + ((vec3<f32>(_e24.g4_.x) * vec3<f32>(_e28.g1_.x)) * vec3<f32>(-(1.0), 0.0, 0.0))) + ((vec3<f32>(_e40.g4_.y) * vec3<f32>(_e44.g1_.y)) * vec3<f32>(-(1.0), 0.0, 0.0))) + ((vec3<f32>(_e56.g4_.z) * vec3<f32>(_e60.g1_.z)) * vec3<f32>(-(1.0), 0.0, 0.0))) + ((vec3<f32>(_e72.g5_.x) * vec3<f32>(_e76.g0_.x)) * vec3<f32>(-(1.0), 0.0, 0.0))) + ((vec3<f32>(_e88.g5_.y) * vec3<f32>(_e92.g0_.y)) * vec3<f32>(-(1.0), 0.0, 0.0))) + ((vec3<f32>(_e104.g5_.z) * vec3<f32>(_e108.g0_.z)) * vec3<f32>(-(1.0), 0.0, 0.0))) + ((_e120.g0_.xxz * vec3<f32>(_e123.g0_.w, _e126.g0_.x, _e129.g0_.w)) * vec3<f32>(1.0, 0.0, 1.0))), ((((((vec3<f32>(_e140.g8_.x) * vec3<f32>(_e144.g1_.z, _e147.g1_.z, _e150.g1_.y)) * vec3<f32>(0.0, 1.0, -(1.0))) + ((vec3<f32>(_e161.g8_.y) * vec3<f32>(_e165.g1_.z, _e168.g1_.z, _e171.g1_.x)) * vec3<f32>(-(1.0), 0.0, 1.0))) + ((vec3<f32>(_e183.g8_.z) * vec3<f32>(_e187.g1_.y, _e190.g1_.x, _e193.g1_.y)) * vec3<f32>(1.0, -(1.0), 0.0))) + (vec3<f32>(_e205.g8_.w) * vec3<f32>(_e209.g0_.x, _e212.g0_.y, _e215.g0_.z))) + (_e221.g1_ * vec3<f32>(_e223.g0_.w))), (((((((((((((vec2<f32>(_e229.g2_.x) * vec2<f32>(_e233.g0_.w)) * vec2<f32>(1.0, 0.0)) + ((vec2<f32>(_e242.g2_.y) * vec2<f32>(_e246.g0_.w)) * vec2<f32>(0.0, 1.0))) + ((vec2<f32>(_e256.g6_.x) * vec2<f32>(_e260.g1_.x)) * vec2<f32>(0.0, -(1.0)))) + ((vec2<f32>(_e271.g6_.y) * vec2<f32>(_e275.g1_.y)) * vec2<f32>(0.0, -(1.0)))) + ((vec2<f32>(_e286.g6_.z) * vec2<f32>(_e290.g1_.z)) * vec2<f32>(0.0, -(1.0)))) + ((vec2<f32>(_e301.g7_.x) * vec2<f32>(_e305.g0_.x)) * vec2<f32>(0.0, -(1.0)))) + ((vec2<f32>(_e316.g7_.y) * vec2<f32>(_e320.g0_.y)) * vec2<f32>(0.0, -(1.0)))) + ((vec2<f32>(_e331.g7_.z) * vec2<f32>(_e335.g0_.z)) * vec2<f32>(0.0, -(1.0)))) + ((vec2<f32>(_e346.g8_.x) * vec2<f32>(_e350.g0_.x)) * vec2<f32>(-(1.0), 0.0))) + ((vec2<f32>(_e361.g8_.y) * vec2<f32>(_e365.g0_.y)) * vec2<f32>(-(1.0), 0.0))) + ((vec2<f32>(_e376.g8_.z) * vec2<f32>(_e380.g0_.z)) * vec2<f32>(-(1.0), 0.0))) + ((vec2<f32>(_e391.g0_.x, _e394.g0_.z) * vec2<f32>(_e398.g1_.x, _e401.g1_.w)) * vec2<f32>(0.0, 1.0))), ((((((vec4<f32>(_e411.g9_.x) * vec4<f32>(_e415.g1_.z, _e418.g1_.z, _e421.g1_.y, _e424.g0_.x)) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0))) + ((vec4<f32>(_e437.g9_.y) * vec4<f32>(_e441.g1_.z, _e444.g1_.z, _e447.g1_.x, _e450.g0_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e464.g9_.z) * vec4<f32>(_e468.g1_.y, _e471.g1_.x, _e474.g1_.y, _e477.g0_.z)) * vec4<f32>(1.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e491.g9_.w) * _e495.g0_.xyzx) * vec4<f32>(1.0, 1.0, 1.0, 0.0))) + (_e506.g3_ * vec4<f32>(_e508.g0_.w))), ((vec3<f32>(_e514.g0_.y) * vec3<f32>(_e518.g0_.x, _e521.g0_.y, _e524.g0_.z)) + (_e529.g4_ * vec3<f32>(_e531.g0_.w))), ((vec3<f32>(_e537.g0_.y) * vec3<f32>(_e541.g1_.x, _e544.g1_.y, _e547.g1_.z)) + (_e552.g5_ * vec3<f32>(_e554.g0_.w))), ((vec3<f32>(_e560.g0_.z) * vec3<f32>(_e564.g0_.x, _e567.g0_.y, _e570.g0_.z)) + (_e575.g6_ * vec3<f32>(_e577.g0_.w))), ((vec3<f32>(_e583.g0_.z) * vec3<f32>(_e587.g1_.x, _e590.g1_.y, _e593.g1_.z)) + (_e598.g7_ * vec3<f32>(_e600.g0_.w))), (_e606.g8_ * vec4<f32>(_e608.g0_.w)), (_e613.g9_ * vec4<f32>(_e615.g0_.w)));
}

fn multi_vector_rotor_into(self_1192: MultiVector) -> Rotor {
    var self_1193: MultiVector;

    self_1193 = self_1192;
    let _e2: MultiVector = self_1193;
    let _e5: MultiVector = self_1193;
    let _e8: MultiVector = self_1193;
    let _e11: MultiVector = self_1193;
    return Rotor(vec4<f32>(_e2.g6_.x, _e5.g6_.y, _e8.g6_.z, _e11.g0_.z));
}

fn multi_vector_rotor_add(self_1194: MultiVector, other_992: Rotor) -> MultiVector {
    var self_1195: MultiVector;
    var other_993: Rotor;

    self_1195 = self_1194;
    other_993 = other_992;
    let _e4: MultiVector = self_1195;
    let _e6: Rotor = other_993;
    let _e9: Rotor = other_993;
    let _e12: Rotor = other_993;
    let _e22: MultiVector = self_1195;
    let _e24: MultiVector = self_1195;
    let _e26: MultiVector = self_1195;
    let _e28: MultiVector = self_1195;
    let _e30: MultiVector = self_1195;
    let _e32: MultiVector = self_1195;
    let _e34: Rotor = other_993;
    let _e37: Rotor = other_993;
    let _e40: Rotor = other_993;
    let _e45: MultiVector = self_1195;
    let _e47: MultiVector = self_1195;
    let _e49: MultiVector = self_1195;
    return MultiVector((_e4.g0_ + (vec3<f32>(_e6.g0_.x, _e9.g0_.x, _e12.g0_.w) * vec3<f32>(0.0, 0.0, 1.0))), _e22.g1_, _e24.g2_, _e26.g3_, _e28.g4_, _e30.g5_, (_e32.g6_ + vec3<f32>(_e34.g0_.x, _e37.g0_.y, _e40.g0_.z)), _e45.g7_, _e47.g8_, _e49.g9_);
}

fn multi_vector_rotor_sub(self_1196: MultiVector, other_994: Rotor) -> MultiVector {
    var self_1197: MultiVector;
    var other_995: Rotor;

    self_1197 = self_1196;
    other_995 = other_994;
    let _e4: MultiVector = self_1197;
    let _e6: Rotor = other_995;
    let _e9: Rotor = other_995;
    let _e12: Rotor = other_995;
    let _e22: MultiVector = self_1197;
    let _e24: MultiVector = self_1197;
    let _e26: MultiVector = self_1197;
    let _e28: MultiVector = self_1197;
    let _e30: MultiVector = self_1197;
    let _e32: MultiVector = self_1197;
    let _e34: Rotor = other_995;
    let _e37: Rotor = other_995;
    let _e40: Rotor = other_995;
    let _e45: MultiVector = self_1197;
    let _e47: MultiVector = self_1197;
    let _e49: MultiVector = self_1197;
    return MultiVector((_e4.g0_ - (vec3<f32>(_e6.g0_.x, _e9.g0_.x, _e12.g0_.w) * vec3<f32>(0.0, 0.0, 1.0))), _e22.g1_, _e24.g2_, _e26.g3_, _e28.g4_, _e30.g5_, (_e32.g6_ - vec3<f32>(_e34.g0_.x, _e37.g0_.y, _e40.g0_.z)), _e45.g7_, _e47.g8_, _e49.g9_);
}

fn multi_vector_rotor_regressive_product(self_1198: MultiVector, other_996: Rotor) -> MultiVector {
    var self_1199: MultiVector;
    var other_997: Rotor;

    self_1199 = self_1198;
    other_997 = other_996;
    let _e4: MultiVector = self_1199;
    let _e8: Rotor = other_997;
    let _e19: MultiVector = self_1199;
    let _e23: Rotor = other_997;
    let _e35: MultiVector = self_1199;
    let _e39: Rotor = other_997;
    let _e51: MultiVector = self_1199;
    let _e53: Rotor = other_997;
    let _e59: MultiVector = self_1199;
    let _e63: Rotor = other_997;
    let _e66: Rotor = other_997;
    let _e69: Rotor = other_997;
    let _e74: MultiVector = self_1199;
    let _e76: Rotor = other_997;
    let _e82: MultiVector = self_1199;
    let _e86: Rotor = other_997;
    let _e96: MultiVector = self_1199;
    let _e100: Rotor = other_997;
    let _e111: MultiVector = self_1199;
    let _e115: Rotor = other_997;
    let _e126: MultiVector = self_1199;
    let _e130: Rotor = other_997;
    let _e141: MultiVector = self_1199;
    let _e145: Rotor = other_997;
    let _e156: MultiVector = self_1199;
    let _e160: Rotor = other_997;
    let _e171: MultiVector = self_1199;
    let _e173: Rotor = other_997;
    let _e179: MultiVector = self_1199;
    let _e183: Rotor = other_997;
    let _e195: MultiVector = self_1199;
    let _e199: Rotor = other_997;
    let _e212: MultiVector = self_1199;
    let _e216: Rotor = other_997;
    let _e229: MultiVector = self_1199;
    let _e233: Rotor = other_997;
    let _e244: MultiVector = self_1199;
    let _e246: Rotor = other_997;
    let _e252: MultiVector = self_1199;
    let _e256: Rotor = other_997;
    let _e259: Rotor = other_997;
    let _e262: Rotor = other_997;
    let _e267: MultiVector = self_1199;
    let _e269: Rotor = other_997;
    let _e275: MultiVector = self_1199;
    let _e277: Rotor = other_997;
    let _e282: MultiVector = self_1199;
    let _e286: Rotor = other_997;
    let _e289: Rotor = other_997;
    let _e292: Rotor = other_997;
    let _e297: MultiVector = self_1199;
    let _e299: Rotor = other_997;
    let _e305: MultiVector = self_1199;
    let _e307: Rotor = other_997;
    let _e312: MultiVector = self_1199;
    let _e314: Rotor = other_997;
    let _e319: MultiVector = self_1199;
    let _e321: Rotor = other_997;
    return MultiVector((((((vec3<f32>(_e4.g5_.x) * vec3<f32>(_e8.g0_.x)) * vec3<f32>(-(1.0), 0.0, 0.0)) + ((vec3<f32>(_e19.g5_.y) * vec3<f32>(_e23.g0_.y)) * vec3<f32>(-(1.0), 0.0, 0.0))) + ((vec3<f32>(_e35.g5_.z) * vec3<f32>(_e39.g0_.z)) * vec3<f32>(-(1.0), 0.0, 0.0))) + (_e51.g0_ * vec3<f32>(_e53.g0_.w))), ((vec3<f32>(_e59.g8_.w) * vec3<f32>(_e63.g0_.x, _e66.g0_.y, _e69.g0_.z)) + (_e74.g1_ * vec3<f32>(_e76.g0_.w))), ((((((((vec2<f32>(_e82.g7_.x) * vec2<f32>(_e86.g0_.x)) * vec2<f32>(0.0, -(1.0))) + ((vec2<f32>(_e96.g7_.y) * vec2<f32>(_e100.g0_.y)) * vec2<f32>(0.0, -(1.0)))) + ((vec2<f32>(_e111.g7_.z) * vec2<f32>(_e115.g0_.z)) * vec2<f32>(0.0, -(1.0)))) + ((vec2<f32>(_e126.g8_.x) * vec2<f32>(_e130.g0_.x)) * vec2<f32>(-(1.0), 0.0))) + ((vec2<f32>(_e141.g8_.y) * vec2<f32>(_e145.g0_.y)) * vec2<f32>(-(1.0), 0.0))) + ((vec2<f32>(_e156.g8_.z) * vec2<f32>(_e160.g0_.z)) * vec2<f32>(-(1.0), 0.0))) + (_e171.g2_ * vec2<f32>(_e173.g0_.w))), ((((((vec4<f32>(_e179.g9_.x) * vec4<f32>(_e183.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0))) + ((vec4<f32>(_e195.g9_.y) * vec4<f32>(_e199.g0_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e212.g9_.z) * vec4<f32>(_e216.g0_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e229.g9_.w) * _e233.g0_.xyzx) * vec4<f32>(1.0, 1.0, 1.0, 0.0))) + (_e244.g3_ * vec4<f32>(_e246.g0_.w))), ((vec3<f32>(_e252.g0_.y) * vec3<f32>(_e256.g0_.x, _e259.g0_.y, _e262.g0_.z)) + (_e267.g4_ * vec3<f32>(_e269.g0_.w))), (_e275.g5_ * vec3<f32>(_e277.g0_.w)), ((vec3<f32>(_e282.g0_.z) * vec3<f32>(_e286.g0_.x, _e289.g0_.y, _e292.g0_.z)) + (_e297.g6_ * vec3<f32>(_e299.g0_.w))), (_e305.g7_ * vec3<f32>(_e307.g0_.w)), (_e312.g8_ * vec4<f32>(_e314.g0_.w)), (_e319.g9_ * vec4<f32>(_e321.g0_.w)));
}

fn multi_vector_translator_into(self_1200: MultiVector) -> Translator {
    var self_1201: MultiVector;

    self_1201 = self_1200;
    let _e2: MultiVector = self_1201;
    let _e5: MultiVector = self_1201;
    let _e8: MultiVector = self_1201;
    let _e11: MultiVector = self_1201;
    return Translator(vec4<f32>(_e2.g7_.x, _e5.g7_.y, _e8.g7_.z, _e11.g0_.z));
}

fn multi_vector_translator_add(self_1202: MultiVector, other_998: Translator) -> MultiVector {
    var self_1203: MultiVector;
    var other_999: Translator;

    self_1203 = self_1202;
    other_999 = other_998;
    let _e4: MultiVector = self_1203;
    let _e6: Translator = other_999;
    let _e9: Translator = other_999;
    let _e12: Translator = other_999;
    let _e22: MultiVector = self_1203;
    let _e24: MultiVector = self_1203;
    let _e26: MultiVector = self_1203;
    let _e28: MultiVector = self_1203;
    let _e30: MultiVector = self_1203;
    let _e32: MultiVector = self_1203;
    let _e34: MultiVector = self_1203;
    let _e36: Translator = other_999;
    let _e39: Translator = other_999;
    let _e42: Translator = other_999;
    let _e47: MultiVector = self_1203;
    let _e49: MultiVector = self_1203;
    return MultiVector((_e4.g0_ + (vec3<f32>(_e6.g0_.x, _e9.g0_.x, _e12.g0_.w) * vec3<f32>(0.0, 0.0, 1.0))), _e22.g1_, _e24.g2_, _e26.g3_, _e28.g4_, _e30.g5_, _e32.g6_, (_e34.g7_ + vec3<f32>(_e36.g0_.x, _e39.g0_.y, _e42.g0_.z)), _e47.g8_, _e49.g9_);
}

fn multi_vector_translator_sub(self_1204: MultiVector, other_1000: Translator) -> MultiVector {
    var self_1205: MultiVector;
    var other_1001: Translator;

    self_1205 = self_1204;
    other_1001 = other_1000;
    let _e4: MultiVector = self_1205;
    let _e6: Translator = other_1001;
    let _e9: Translator = other_1001;
    let _e12: Translator = other_1001;
    let _e22: MultiVector = self_1205;
    let _e24: MultiVector = self_1205;
    let _e26: MultiVector = self_1205;
    let _e28: MultiVector = self_1205;
    let _e30: MultiVector = self_1205;
    let _e32: MultiVector = self_1205;
    let _e34: MultiVector = self_1205;
    let _e36: Translator = other_1001;
    let _e39: Translator = other_1001;
    let _e42: Translator = other_1001;
    let _e47: MultiVector = self_1205;
    let _e49: MultiVector = self_1205;
    return MultiVector((_e4.g0_ - (vec3<f32>(_e6.g0_.x, _e9.g0_.x, _e12.g0_.w) * vec3<f32>(0.0, 0.0, 1.0))), _e22.g1_, _e24.g2_, _e26.g3_, _e28.g4_, _e30.g5_, _e32.g6_, (_e34.g7_ - vec3<f32>(_e36.g0_.x, _e39.g0_.y, _e42.g0_.z)), _e47.g8_, _e49.g9_);
}

fn multi_vector_translator_regressive_product(self_1206: MultiVector, other_1002: Translator) -> MultiVector {
    var self_1207: MultiVector;
    var other_1003: Translator;

    self_1207 = self_1206;
    other_1003 = other_1002;
    let _e4: MultiVector = self_1207;
    let _e8: Translator = other_1003;
    let _e19: MultiVector = self_1207;
    let _e23: Translator = other_1003;
    let _e35: MultiVector = self_1207;
    let _e39: Translator = other_1003;
    let _e51: MultiVector = self_1207;
    let _e53: Translator = other_1003;
    let _e59: MultiVector = self_1207;
    let _e63: Translator = other_1003;
    let _e66: Translator = other_1003;
    let _e69: Translator = other_1003;
    let _e80: MultiVector = self_1207;
    let _e84: Translator = other_1003;
    let _e87: Translator = other_1003;
    let _e90: Translator = other_1003;
    let _e102: MultiVector = self_1207;
    let _e106: Translator = other_1003;
    let _e109: Translator = other_1003;
    let _e112: Translator = other_1003;
    let _e124: MultiVector = self_1207;
    let _e126: Translator = other_1003;
    let _e132: MultiVector = self_1207;
    let _e136: Translator = other_1003;
    let _e146: MultiVector = self_1207;
    let _e150: Translator = other_1003;
    let _e161: MultiVector = self_1207;
    let _e165: Translator = other_1003;
    let _e176: MultiVector = self_1207;
    let _e178: Translator = other_1003;
    let _e184: MultiVector = self_1207;
    let _e188: Translator = other_1003;
    let _e199: MultiVector = self_1207;
    let _e203: Translator = other_1003;
    let _e215: MultiVector = self_1207;
    let _e219: Translator = other_1003;
    let _e231: MultiVector = self_1207;
    let _e233: Translator = other_1003;
    let _e239: MultiVector = self_1207;
    let _e241: Translator = other_1003;
    let _e246: MultiVector = self_1207;
    let _e250: Translator = other_1003;
    let _e253: Translator = other_1003;
    let _e256: Translator = other_1003;
    let _e261: MultiVector = self_1207;
    let _e263: Translator = other_1003;
    let _e269: MultiVector = self_1207;
    let _e271: Translator = other_1003;
    let _e276: MultiVector = self_1207;
    let _e280: Translator = other_1003;
    let _e283: Translator = other_1003;
    let _e286: Translator = other_1003;
    let _e291: MultiVector = self_1207;
    let _e293: Translator = other_1003;
    let _e299: MultiVector = self_1207;
    let _e301: Translator = other_1003;
    let _e306: MultiVector = self_1207;
    let _e308: Translator = other_1003;
    return MultiVector((((((vec3<f32>(_e4.g4_.x) * vec3<f32>(_e8.g0_.x)) * vec3<f32>(-(1.0), 0.0, 0.0)) + ((vec3<f32>(_e19.g4_.y) * vec3<f32>(_e23.g0_.y)) * vec3<f32>(-(1.0), 0.0, 0.0))) + ((vec3<f32>(_e35.g4_.z) * vec3<f32>(_e39.g0_.z)) * vec3<f32>(-(1.0), 0.0, 0.0))) + (_e51.g0_ * vec3<f32>(_e53.g0_.w))), (((((vec3<f32>(_e59.g8_.x) * vec3<f32>(_e63.g0_.z, _e66.g0_.z, _e69.g0_.y)) * vec3<f32>(0.0, 1.0, -(1.0))) + ((vec3<f32>(_e80.g8_.y) * vec3<f32>(_e84.g0_.z, _e87.g0_.z, _e90.g0_.x)) * vec3<f32>(-(1.0), 0.0, 1.0))) + ((vec3<f32>(_e102.g8_.z) * vec3<f32>(_e106.g0_.y, _e109.g0_.x, _e112.g0_.y)) * vec3<f32>(1.0, -(1.0), 0.0))) + (_e124.g1_ * vec3<f32>(_e126.g0_.w))), (((((vec2<f32>(_e132.g6_.x) * vec2<f32>(_e136.g0_.x)) * vec2<f32>(0.0, -(1.0))) + ((vec2<f32>(_e146.g6_.y) * vec2<f32>(_e150.g0_.y)) * vec2<f32>(0.0, -(1.0)))) + ((vec2<f32>(_e161.g6_.z) * vec2<f32>(_e165.g0_.z)) * vec2<f32>(0.0, -(1.0)))) + (_e176.g2_ * vec2<f32>(_e178.g0_.w))), (((((vec4<f32>(_e184.g9_.x) * _e188.g0_.zzyz) * vec4<f32>(0.0, 1.0, -(1.0), 0.0)) + ((vec4<f32>(_e199.g9_.y) * _e203.g0_.zzxz) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0))) + ((vec4<f32>(_e215.g9_.z) * _e219.g0_.yxyy) * vec4<f32>(1.0, -(1.0), 0.0, 0.0))) + (_e231.g3_ * vec4<f32>(_e233.g0_.w))), (_e239.g4_ * vec3<f32>(_e241.g0_.w)), ((vec3<f32>(_e246.g0_.y) * vec3<f32>(_e250.g0_.x, _e253.g0_.y, _e256.g0_.z)) + (_e261.g5_ * vec3<f32>(_e263.g0_.w))), (_e269.g6_ * vec3<f32>(_e271.g0_.w)), ((vec3<f32>(_e276.g0_.z) * vec3<f32>(_e280.g0_.x, _e283.g0_.y, _e286.g0_.z)) + (_e291.g7_ * vec3<f32>(_e293.g0_.w))), (_e299.g8_ * vec4<f32>(_e301.g0_.w)), (_e306.g9_ * vec4<f32>(_e308.g0_.w)));
}

fn multi_vector_flector_into(self_1208: MultiVector) -> Flector {
    var self_1209: MultiVector;

    self_1209 = self_1208;
    let _e2: MultiVector = self_1209;
    let _e4: MultiVector = self_1209;
    return Flector(_e2.g3_, _e4.g9_);
}

fn multi_vector_flector_add(self_1210: MultiVector, other_1004: Flector) -> MultiVector {
    var self_1211: MultiVector;
    var other_1005: Flector;

    self_1211 = self_1210;
    other_1005 = other_1004;
    let _e4: MultiVector = self_1211;
    let _e6: MultiVector = self_1211;
    let _e8: MultiVector = self_1211;
    let _e10: MultiVector = self_1211;
    let _e12: Flector = other_1005;
    let _e15: MultiVector = self_1211;
    let _e17: MultiVector = self_1211;
    let _e19: MultiVector = self_1211;
    let _e21: MultiVector = self_1211;
    let _e23: MultiVector = self_1211;
    let _e25: MultiVector = self_1211;
    let _e27: Flector = other_1005;
    return MultiVector(_e4.g0_, _e6.g1_, _e8.g2_, (_e10.g3_ + _e12.g0_), _e15.g4_, _e17.g5_, _e19.g6_, _e21.g7_, _e23.g8_, (_e25.g9_ + _e27.g1_));
}

fn multi_vector_flector_sub(self_1212: MultiVector, other_1006: Flector) -> MultiVector {
    var self_1213: MultiVector;
    var other_1007: Flector;

    self_1213 = self_1212;
    other_1007 = other_1006;
    let _e4: MultiVector = self_1213;
    let _e6: MultiVector = self_1213;
    let _e8: MultiVector = self_1213;
    let _e10: MultiVector = self_1213;
    let _e12: Flector = other_1007;
    let _e15: MultiVector = self_1213;
    let _e17: MultiVector = self_1213;
    let _e19: MultiVector = self_1213;
    let _e21: MultiVector = self_1213;
    let _e23: MultiVector = self_1213;
    let _e25: MultiVector = self_1213;
    let _e27: Flector = other_1007;
    return MultiVector(_e4.g0_, _e6.g1_, _e8.g2_, (_e10.g3_ - _e12.g0_), _e15.g4_, _e17.g5_, _e19.g6_, _e21.g7_, _e23.g8_, (_e25.g9_ - _e27.g1_));
}

fn multi_vector_multi_vector_add(self_1214: MultiVector, other_1008: MultiVector) -> MultiVector {
    var self_1215: MultiVector;
    var other_1009: MultiVector;

    self_1215 = self_1214;
    other_1009 = other_1008;
    let _e4: MultiVector = self_1215;
    let _e6: MultiVector = other_1009;
    let _e9: MultiVector = self_1215;
    let _e11: MultiVector = other_1009;
    let _e14: MultiVector = self_1215;
    let _e16: MultiVector = other_1009;
    let _e19: MultiVector = self_1215;
    let _e21: MultiVector = other_1009;
    let _e24: MultiVector = self_1215;
    let _e26: MultiVector = other_1009;
    let _e29: MultiVector = self_1215;
    let _e31: MultiVector = other_1009;
    let _e34: MultiVector = self_1215;
    let _e36: MultiVector = other_1009;
    let _e39: MultiVector = self_1215;
    let _e41: MultiVector = other_1009;
    let _e44: MultiVector = self_1215;
    let _e46: MultiVector = other_1009;
    let _e49: MultiVector = self_1215;
    let _e51: MultiVector = other_1009;
    return MultiVector((_e4.g0_ + _e6.g0_), (_e9.g1_ + _e11.g1_), (_e14.g2_ + _e16.g2_), (_e19.g3_ + _e21.g3_), (_e24.g4_ + _e26.g4_), (_e29.g5_ + _e31.g5_), (_e34.g6_ + _e36.g6_), (_e39.g7_ + _e41.g7_), (_e44.g8_ + _e46.g8_), (_e49.g9_ + _e51.g9_));
}

fn multi_vector_multi_vector_sub(self_1216: MultiVector, other_1010: MultiVector) -> MultiVector {
    var self_1217: MultiVector;
    var other_1011: MultiVector;

    self_1217 = self_1216;
    other_1011 = other_1010;
    let _e4: MultiVector = self_1217;
    let _e6: MultiVector = other_1011;
    let _e9: MultiVector = self_1217;
    let _e11: MultiVector = other_1011;
    let _e14: MultiVector = self_1217;
    let _e16: MultiVector = other_1011;
    let _e19: MultiVector = self_1217;
    let _e21: MultiVector = other_1011;
    let _e24: MultiVector = self_1217;
    let _e26: MultiVector = other_1011;
    let _e29: MultiVector = self_1217;
    let _e31: MultiVector = other_1011;
    let _e34: MultiVector = self_1217;
    let _e36: MultiVector = other_1011;
    let _e39: MultiVector = self_1217;
    let _e41: MultiVector = other_1011;
    let _e44: MultiVector = self_1217;
    let _e46: MultiVector = other_1011;
    let _e49: MultiVector = self_1217;
    let _e51: MultiVector = other_1011;
    return MultiVector((_e4.g0_ - _e6.g0_), (_e9.g1_ - _e11.g1_), (_e14.g2_ - _e16.g2_), (_e19.g3_ - _e21.g3_), (_e24.g4_ - _e26.g4_), (_e29.g5_ - _e31.g5_), (_e34.g6_ - _e36.g6_), (_e39.g7_ - _e41.g7_), (_e44.g8_ - _e46.g8_), (_e49.g9_ - _e51.g9_));
}

fn multi_vector_multi_vector_mul(self_1218: MultiVector, other_1012: MultiVector) -> MultiVector {
    var self_1219: MultiVector;
    var other_1013: MultiVector;

    self_1219 = self_1218;
    other_1013 = other_1012;
    let _e4: MultiVector = self_1219;
    let _e6: MultiVector = other_1013;
    let _e9: MultiVector = self_1219;
    let _e11: MultiVector = other_1013;
    let _e14: MultiVector = self_1219;
    let _e16: MultiVector = other_1013;
    let _e19: MultiVector = self_1219;
    let _e21: MultiVector = other_1013;
    let _e24: MultiVector = self_1219;
    let _e26: MultiVector = other_1013;
    let _e29: MultiVector = self_1219;
    let _e31: MultiVector = other_1013;
    let _e34: MultiVector = self_1219;
    let _e36: MultiVector = other_1013;
    let _e39: MultiVector = self_1219;
    let _e41: MultiVector = other_1013;
    let _e44: MultiVector = self_1219;
    let _e46: MultiVector = other_1013;
    let _e49: MultiVector = self_1219;
    let _e51: MultiVector = other_1013;
    return MultiVector((_e4.g0_ * _e6.g0_), (_e9.g1_ * _e11.g1_), (_e14.g2_ * _e16.g2_), (_e19.g3_ * _e21.g3_), (_e24.g4_ * _e26.g4_), (_e29.g5_ * _e31.g5_), (_e34.g6_ * _e36.g6_), (_e39.g7_ * _e41.g7_), (_e44.g8_ * _e46.g8_), (_e49.g9_ * _e51.g9_));
}

fn multi_vector_multi_vector_div(self_1220: MultiVector, other_1014: MultiVector) -> MultiVector {
    var self_1221: MultiVector;
    var other_1015: MultiVector;

    self_1221 = self_1220;
    other_1015 = other_1014;
    let _e4: MultiVector = self_1221;
    let _e7: MultiVector = self_1221;
    let _e10: MultiVector = self_1221;
    let _e19: MultiVector = other_1015;
    let _e22: MultiVector = other_1015;
    let _e25: MultiVector = other_1015;
    let _e35: MultiVector = self_1221;
    let _e38: MultiVector = self_1221;
    let _e41: MultiVector = self_1221;
    let _e50: MultiVector = other_1015;
    let _e53: MultiVector = other_1015;
    let _e56: MultiVector = other_1015;
    let _e66: MultiVector = self_1221;
    let _e69: MultiVector = self_1221;
    let _e77: MultiVector = other_1015;
    let _e80: MultiVector = other_1015;
    let _e89: MultiVector = self_1221;
    let _e92: MultiVector = self_1221;
    let _e95: MultiVector = self_1221;
    let _e98: MultiVector = self_1221;
    let _e108: MultiVector = other_1015;
    let _e111: MultiVector = other_1015;
    let _e114: MultiVector = other_1015;
    let _e117: MultiVector = other_1015;
    let _e128: MultiVector = self_1221;
    let _e131: MultiVector = self_1221;
    let _e134: MultiVector = self_1221;
    let _e143: MultiVector = other_1015;
    let _e146: MultiVector = other_1015;
    let _e149: MultiVector = other_1015;
    let _e159: MultiVector = self_1221;
    let _e162: MultiVector = self_1221;
    let _e165: MultiVector = self_1221;
    let _e174: MultiVector = other_1015;
    let _e177: MultiVector = other_1015;
    let _e180: MultiVector = other_1015;
    let _e190: MultiVector = self_1221;
    let _e193: MultiVector = self_1221;
    let _e196: MultiVector = self_1221;
    let _e205: MultiVector = other_1015;
    let _e208: MultiVector = other_1015;
    let _e211: MultiVector = other_1015;
    let _e221: MultiVector = self_1221;
    let _e224: MultiVector = self_1221;
    let _e227: MultiVector = self_1221;
    let _e236: MultiVector = other_1015;
    let _e239: MultiVector = other_1015;
    let _e242: MultiVector = other_1015;
    let _e252: MultiVector = self_1221;
    let _e255: MultiVector = self_1221;
    let _e258: MultiVector = self_1221;
    let _e261: MultiVector = self_1221;
    let _e271: MultiVector = other_1015;
    let _e274: MultiVector = other_1015;
    let _e277: MultiVector = other_1015;
    let _e280: MultiVector = other_1015;
    let _e291: MultiVector = self_1221;
    let _e294: MultiVector = self_1221;
    let _e297: MultiVector = self_1221;
    let _e300: MultiVector = self_1221;
    let _e310: MultiVector = other_1015;
    let _e313: MultiVector = other_1015;
    let _e316: MultiVector = other_1015;
    let _e319: MultiVector = other_1015;
    return MultiVector((((vec3<f32>(_e4.g0_.x, _e7.g0_.y, _e10.g0_.z) * vec3<f32>(1.0, 1.0, 1.0)) / vec3<f32>(_e19.g0_.x, _e22.g0_.y, _e25.g0_.z)) * vec3<f32>(1.0, 1.0, 1.0)), (((vec3<f32>(_e35.g1_.x, _e38.g1_.y, _e41.g1_.z) * vec3<f32>(1.0, 1.0, 1.0)) / vec3<f32>(_e50.g1_.x, _e53.g1_.y, _e56.g1_.z)) * vec3<f32>(1.0, 1.0, 1.0)), (((vec2<f32>(_e66.g2_.x, _e69.g2_.y) * vec2<f32>(1.0, 1.0)) / vec2<f32>(_e77.g2_.x, _e80.g2_.y)) * vec2<f32>(1.0, 1.0)), (((vec4<f32>(_e89.g3_.x, _e92.g3_.y, _e95.g3_.z, _e98.g3_.w) * vec4<f32>(1.0, 1.0, 1.0, 1.0)) / vec4<f32>(_e108.g3_.x, _e111.g3_.y, _e114.g3_.z, _e117.g3_.w)) * vec4<f32>(1.0, 1.0, 1.0, 1.0)), (((vec3<f32>(_e128.g4_.x, _e131.g4_.y, _e134.g4_.z) * vec3<f32>(1.0, 1.0, 1.0)) / vec3<f32>(_e143.g4_.x, _e146.g4_.y, _e149.g4_.z)) * vec3<f32>(1.0, 1.0, 1.0)), (((vec3<f32>(_e159.g5_.x, _e162.g5_.y, _e165.g5_.z) * vec3<f32>(1.0, 1.0, 1.0)) / vec3<f32>(_e174.g5_.x, _e177.g5_.y, _e180.g5_.z)) * vec3<f32>(1.0, 1.0, 1.0)), (((vec3<f32>(_e190.g6_.x, _e193.g6_.y, _e196.g6_.z) * vec3<f32>(1.0, 1.0, 1.0)) / vec3<f32>(_e205.g6_.x, _e208.g6_.y, _e211.g6_.z)) * vec3<f32>(1.0, 1.0, 1.0)), (((vec3<f32>(_e221.g7_.x, _e224.g7_.y, _e227.g7_.z) * vec3<f32>(1.0, 1.0, 1.0)) / vec3<f32>(_e236.g7_.x, _e239.g7_.y, _e242.g7_.z)) * vec3<f32>(1.0, 1.0, 1.0)), (((vec4<f32>(_e252.g8_.x, _e255.g8_.y, _e258.g8_.z, _e261.g8_.w) * vec4<f32>(1.0, 1.0, 1.0, 1.0)) / vec4<f32>(_e271.g8_.x, _e274.g8_.y, _e277.g8_.z, _e280.g8_.w)) * vec4<f32>(1.0, 1.0, 1.0, 1.0)), (((vec4<f32>(_e291.g9_.x, _e294.g9_.y, _e297.g9_.z, _e300.g9_.w) * vec4<f32>(1.0, 1.0, 1.0, 1.0)) / vec4<f32>(_e310.g9_.x, _e313.g9_.y, _e316.g9_.z, _e319.g9_.w)) * vec4<f32>(1.0, 1.0, 1.0, 1.0)));
}

fn multi_vector_multi_vector_geometric_product(self_1222: MultiVector, other_1016: MultiVector) -> MultiVector {
    var self_1223: MultiVector;
    var other_1017: MultiVector;

    self_1223 = self_1222;
    other_1017 = other_1016;
    let _e4: MultiVector = self_1223;
    let _e8: MultiVector = other_1017;
    let _e11: MultiVector = self_1223;
    let _e15: MultiVector = other_1017;
    let _e26: MultiVector = self_1223;
    let _e30: MultiVector = other_1017;
    let _e33: MultiVector = other_1017;
    let _e36: MultiVector = other_1017;
    let _e42: MultiVector = self_1223;
    let _e46: MultiVector = other_1017;
    let _e49: MultiVector = other_1017;
    let _e52: MultiVector = other_1017;
    let _e58: MultiVector = self_1223;
    let _e62: MultiVector = other_1017;
    let _e65: MultiVector = other_1017;
    let _e68: MultiVector = other_1017;
    let _e74: MultiVector = self_1223;
    let _e78: MultiVector = other_1017;
    let _e81: MultiVector = other_1017;
    let _e84: MultiVector = other_1017;
    let _e95: MultiVector = self_1223;
    let _e99: MultiVector = other_1017;
    let _e110: MultiVector = self_1223;
    let _e114: MultiVector = other_1017;
    let _e126: MultiVector = self_1223;
    let _e130: MultiVector = other_1017;
    let _e142: MultiVector = self_1223;
    let _e146: MultiVector = other_1017;
    let _e158: MultiVector = self_1223;
    let _e162: MultiVector = other_1017;
    let _e174: MultiVector = self_1223;
    let _e178: MultiVector = other_1017;
    let _e181: MultiVector = other_1017;
    let _e184: MultiVector = other_1017;
    let _e197: MultiVector = self_1223;
    let _e201: MultiVector = other_1017;
    let _e204: MultiVector = other_1017;
    let _e207: MultiVector = other_1017;
    let _e220: MultiVector = self_1223;
    let _e224: MultiVector = other_1017;
    let _e227: MultiVector = other_1017;
    let _e230: MultiVector = other_1017;
    let _e243: MultiVector = self_1223;
    let _e247: MultiVector = other_1017;
    let _e250: MultiVector = other_1017;
    let _e253: MultiVector = other_1017;
    let _e259: MultiVector = self_1223;
    let _e263: MultiVector = other_1017;
    let _e266: MultiVector = other_1017;
    let _e269: MultiVector = other_1017;
    let _e275: MultiVector = self_1223;
    let _e279: MultiVector = other_1017;
    let _e282: MultiVector = other_1017;
    let _e285: MultiVector = other_1017;
    let _e291: MultiVector = self_1223;
    let _e295: MultiVector = other_1017;
    let _e307: MultiVector = self_1223;
    let _e311: MultiVector = other_1017;
    let _e323: MultiVector = self_1223;
    let _e327: MultiVector = other_1017;
    let _e339: MultiVector = self_1223;
    let _e343: MultiVector = other_1017;
    let _e355: MultiVector = self_1223;
    let _e359: MultiVector = other_1017;
    let _e371: MultiVector = self_1223;
    let _e375: MultiVector = other_1017;
    let _e387: MultiVector = self_1223;
    let _e391: MultiVector = other_1017;
    let _e394: MultiVector = other_1017;
    let _e397: MultiVector = other_1017;
    let _e410: MultiVector = self_1223;
    let _e414: MultiVector = other_1017;
    let _e417: MultiVector = other_1017;
    let _e420: MultiVector = other_1017;
    let _e433: MultiVector = self_1223;
    let _e437: MultiVector = other_1017;
    let _e440: MultiVector = other_1017;
    let _e443: MultiVector = other_1017;
    let _e456: MultiVector = self_1223;
    let _e460: MultiVector = other_1017;
    let _e463: MultiVector = other_1017;
    let _e466: MultiVector = other_1017;
    let _e472: MultiVector = self_1223;
    let _e476: MultiVector = other_1017;
    let _e487: MultiVector = self_1223;
    let _e491: MultiVector = other_1017;
    let _e502: MultiVector = self_1223;
    let _e506: MultiVector = other_1017;
    let _e517: MultiVector = self_1223;
    let _e521: MultiVector = other_1017;
    let _e532: MultiVector = self_1223;
    let _e535: MultiVector = other_1017;
    let _e538: MultiVector = other_1017;
    let _e541: MultiVector = other_1017;
    let _e552: MultiVector = self_1223;
    let _e556: MultiVector = other_1017;
    let _e559: MultiVector = self_1223;
    let _e563: MultiVector = other_1017;
    let _e566: MultiVector = other_1017;
    let _e569: MultiVector = other_1017;
    let _e581: MultiVector = self_1223;
    let _e585: MultiVector = other_1017;
    let _e588: MultiVector = other_1017;
    let _e591: MultiVector = other_1017;
    let _e603: MultiVector = self_1223;
    let _e607: MultiVector = other_1017;
    let _e610: MultiVector = other_1017;
    let _e613: MultiVector = other_1017;
    let _e625: MultiVector = self_1223;
    let _e629: MultiVector = other_1017;
    let _e632: MultiVector = other_1017;
    let _e635: MultiVector = other_1017;
    let _e647: MultiVector = self_1223;
    let _e651: MultiVector = other_1017;
    let _e654: MultiVector = other_1017;
    let _e657: MultiVector = other_1017;
    let _e669: MultiVector = self_1223;
    let _e673: MultiVector = other_1017;
    let _e676: MultiVector = other_1017;
    let _e679: MultiVector = other_1017;
    let _e691: MultiVector = self_1223;
    let _e695: MultiVector = other_1017;
    let _e699: MultiVector = self_1223;
    let _e703: MultiVector = other_1017;
    let _e706: MultiVector = self_1223;
    let _e710: MultiVector = other_1017;
    let _e713: MultiVector = other_1017;
    let _e724: MultiVector = self_1223;
    let _e728: MultiVector = other_1017;
    let _e731: MultiVector = other_1017;
    let _e742: MultiVector = self_1223;
    let _e746: MultiVector = other_1017;
    let _e749: MultiVector = other_1017;
    let _e760: MultiVector = self_1223;
    let _e764: MultiVector = other_1017;
    let _e774: MultiVector = self_1223;
    let _e778: MultiVector = other_1017;
    let _e788: MultiVector = self_1223;
    let _e792: MultiVector = other_1017;
    let _e803: MultiVector = self_1223;
    let _e807: MultiVector = other_1017;
    let _e818: MultiVector = self_1223;
    let _e822: MultiVector = other_1017;
    let _e833: MultiVector = self_1223;
    let _e837: MultiVector = other_1017;
    let _e847: MultiVector = self_1223;
    let _e851: MultiVector = other_1017;
    let _e861: MultiVector = self_1223;
    let _e865: MultiVector = other_1017;
    let _e875: MultiVector = self_1223;
    let _e879: MultiVector = other_1017;
    let _e882: MultiVector = other_1017;
    let _e888: MultiVector = self_1223;
    let _e892: MultiVector = other_1017;
    let _e895: MultiVector = other_1017;
    let _e901: MultiVector = self_1223;
    let _e905: MultiVector = other_1017;
    let _e908: MultiVector = other_1017;
    let _e914: MultiVector = self_1223;
    let _e918: MultiVector = other_1017;
    let _e929: MultiVector = self_1223;
    let _e933: MultiVector = other_1017;
    let _e944: MultiVector = self_1223;
    let _e948: MultiVector = other_1017;
    let _e959: MultiVector = self_1223;
    let _e963: MultiVector = other_1017;
    let _e974: MultiVector = self_1223;
    let _e978: MultiVector = other_1017;
    let _e989: MultiVector = self_1223;
    let _e993: MultiVector = other_1017;
    let _e1004: MultiVector = self_1223;
    let _e1008: MultiVector = other_1017;
    let _e1011: MultiVector = other_1017;
    let _e1022: MultiVector = self_1223;
    let _e1026: MultiVector = other_1017;
    let _e1036: MultiVector = self_1223;
    let _e1039: MultiVector = self_1223;
    let _e1043: MultiVector = other_1017;
    let _e1046: MultiVector = other_1017;
    let _e1057: MultiVector = self_1223;
    let _e1061: MultiVector = other_1017;
    let _e1064: MultiVector = self_1223;
    let _e1068: MultiVector = other_1017;
    let _e1080: MultiVector = self_1223;
    let _e1084: MultiVector = other_1017;
    let _e1087: MultiVector = other_1017;
    let _e1090: MultiVector = other_1017;
    let _e1093: MultiVector = other_1017;
    let _e1107: MultiVector = self_1223;
    let _e1111: MultiVector = other_1017;
    let _e1114: MultiVector = other_1017;
    let _e1117: MultiVector = other_1017;
    let _e1120: MultiVector = other_1017;
    let _e1134: MultiVector = self_1223;
    let _e1138: MultiVector = other_1017;
    let _e1141: MultiVector = other_1017;
    let _e1144: MultiVector = other_1017;
    let _e1147: MultiVector = other_1017;
    let _e1161: MultiVector = self_1223;
    let _e1165: MultiVector = other_1017;
    let _e1177: MultiVector = self_1223;
    let _e1181: MultiVector = other_1017;
    let _e1184: MultiVector = other_1017;
    let _e1187: MultiVector = other_1017;
    let _e1190: MultiVector = other_1017;
    let _e1196: MultiVector = self_1223;
    let _e1200: MultiVector = other_1017;
    let _e1203: MultiVector = other_1017;
    let _e1206: MultiVector = other_1017;
    let _e1209: MultiVector = other_1017;
    let _e1223: MultiVector = self_1223;
    let _e1227: MultiVector = other_1017;
    let _e1230: MultiVector = other_1017;
    let _e1233: MultiVector = other_1017;
    let _e1236: MultiVector = other_1017;
    let _e1250: MultiVector = self_1223;
    let _e1254: MultiVector = other_1017;
    let _e1257: MultiVector = other_1017;
    let _e1260: MultiVector = other_1017;
    let _e1263: MultiVector = other_1017;
    let _e1277: MultiVector = self_1223;
    let _e1281: MultiVector = other_1017;
    let _e1293: MultiVector = self_1223;
    let _e1297: MultiVector = other_1017;
    let _e1309: MultiVector = self_1223;
    let _e1313: MultiVector = other_1017;
    let _e1325: MultiVector = self_1223;
    let _e1329: MultiVector = other_1017;
    let _e1341: MultiVector = self_1223;
    let _e1345: MultiVector = other_1017;
    let _e1348: MultiVector = other_1017;
    let _e1351: MultiVector = other_1017;
    let _e1354: MultiVector = other_1017;
    let _e1368: MultiVector = self_1223;
    let _e1372: MultiVector = other_1017;
    let _e1375: MultiVector = other_1017;
    let _e1378: MultiVector = other_1017;
    let _e1381: MultiVector = other_1017;
    let _e1395: MultiVector = self_1223;
    let _e1399: MultiVector = other_1017;
    let _e1402: MultiVector = other_1017;
    let _e1405: MultiVector = other_1017;
    let _e1408: MultiVector = other_1017;
    let _e1422: MultiVector = self_1223;
    let _e1426: MultiVector = other_1017;
    let _e1439: MultiVector = self_1223;
    let _e1443: MultiVector = other_1017;
    let _e1456: MultiVector = self_1223;
    let _e1460: MultiVector = other_1017;
    let _e1473: MultiVector = self_1223;
    let _e1477: MultiVector = other_1017;
    let _e1480: MultiVector = other_1017;
    let _e1483: MultiVector = other_1017;
    let _e1486: MultiVector = other_1017;
    let _e1500: MultiVector = self_1223;
    let _e1504: MultiVector = other_1017;
    let _e1507: MultiVector = other_1017;
    let _e1510: MultiVector = other_1017;
    let _e1513: MultiVector = other_1017;
    let _e1527: MultiVector = self_1223;
    let _e1531: MultiVector = other_1017;
    let _e1534: MultiVector = other_1017;
    let _e1537: MultiVector = other_1017;
    let _e1540: MultiVector = other_1017;
    let _e1554: MultiVector = self_1223;
    let _e1558: MultiVector = other_1017;
    let _e1571: MultiVector = self_1223;
    let _e1575: MultiVector = other_1017;
    let _e1588: MultiVector = self_1223;
    let _e1592: MultiVector = other_1017;
    let _e1605: MultiVector = self_1223;
    let _e1609: MultiVector = other_1017;
    let _e1612: MultiVector = other_1017;
    let _e1615: MultiVector = other_1017;
    let _e1618: MultiVector = other_1017;
    let _e1624: MultiVector = self_1223;
    let _e1628: MultiVector = other_1017;
    let _e1641: MultiVector = self_1223;
    let _e1645: MultiVector = other_1017;
    let _e1658: MultiVector = self_1223;
    let _e1662: MultiVector = other_1017;
    let _e1675: MultiVector = self_1223;
    let _e1679: MultiVector = other_1017;
    let _e1682: MultiVector = other_1017;
    let _e1685: MultiVector = other_1017;
    let _e1688: MultiVector = other_1017;
    let _e1694: MultiVector = self_1223;
    let _e1697: MultiVector = self_1223;
    let _e1700: MultiVector = self_1223;
    let _e1703: MultiVector = self_1223;
    let _e1707: MultiVector = other_1017;
    let _e1719: MultiVector = self_1223;
    let _e1723: MultiVector = other_1017;
    let _e1726: MultiVector = self_1223;
    let _e1730: MultiVector = other_1017;
    let _e1734: MultiVector = self_1223;
    let _e1738: MultiVector = other_1017;
    let _e1741: MultiVector = other_1017;
    let _e1744: MultiVector = other_1017;
    let _e1757: MultiVector = self_1223;
    let _e1761: MultiVector = other_1017;
    let _e1764: MultiVector = other_1017;
    let _e1767: MultiVector = other_1017;
    let _e1780: MultiVector = self_1223;
    let _e1784: MultiVector = other_1017;
    let _e1787: MultiVector = other_1017;
    let _e1790: MultiVector = other_1017;
    let _e1803: MultiVector = self_1223;
    let _e1807: MultiVector = other_1017;
    let _e1811: MultiVector = self_1223;
    let _e1815: MultiVector = other_1017;
    let _e1818: MultiVector = other_1017;
    let _e1821: MultiVector = other_1017;
    let _e1833: MultiVector = self_1223;
    let _e1837: MultiVector = other_1017;
    let _e1840: MultiVector = other_1017;
    let _e1843: MultiVector = other_1017;
    let _e1855: MultiVector = self_1223;
    let _e1859: MultiVector = other_1017;
    let _e1862: MultiVector = other_1017;
    let _e1865: MultiVector = other_1017;
    let _e1877: MultiVector = self_1223;
    let _e1881: MultiVector = other_1017;
    let _e1884: MultiVector = other_1017;
    let _e1887: MultiVector = other_1017;
    let _e1899: MultiVector = self_1223;
    let _e1903: MultiVector = other_1017;
    let _e1906: MultiVector = other_1017;
    let _e1909: MultiVector = other_1017;
    let _e1921: MultiVector = self_1223;
    let _e1925: MultiVector = other_1017;
    let _e1928: MultiVector = other_1017;
    let _e1931: MultiVector = other_1017;
    let _e1943: MultiVector = self_1223;
    let _e1947: MultiVector = other_1017;
    let _e1950: MultiVector = other_1017;
    let _e1953: MultiVector = other_1017;
    let _e1965: MultiVector = self_1223;
    let _e1969: MultiVector = other_1017;
    let _e1972: MultiVector = other_1017;
    let _e1975: MultiVector = other_1017;
    let _e1987: MultiVector = self_1223;
    let _e1991: MultiVector = other_1017;
    let _e1994: MultiVector = other_1017;
    let _e1997: MultiVector = other_1017;
    let _e2009: MultiVector = self_1223;
    let _e2013: MultiVector = other_1017;
    let _e2016: MultiVector = other_1017;
    let _e2019: MultiVector = other_1017;
    let _e2025: MultiVector = self_1223;
    let _e2029: MultiVector = other_1017;
    let _e2032: MultiVector = self_1223;
    let _e2036: MultiVector = other_1017;
    let _e2039: MultiVector = other_1017;
    let _e2042: MultiVector = other_1017;
    let _e2055: MultiVector = self_1223;
    let _e2059: MultiVector = other_1017;
    let _e2062: MultiVector = other_1017;
    let _e2065: MultiVector = other_1017;
    let _e2078: MultiVector = self_1223;
    let _e2082: MultiVector = other_1017;
    let _e2085: MultiVector = other_1017;
    let _e2088: MultiVector = other_1017;
    let _e2101: MultiVector = self_1223;
    let _e2105: MultiVector = other_1017;
    let _e2108: MultiVector = other_1017;
    let _e2111: MultiVector = other_1017;
    let _e2123: MultiVector = self_1223;
    let _e2127: MultiVector = other_1017;
    let _e2130: MultiVector = other_1017;
    let _e2133: MultiVector = other_1017;
    let _e2145: MultiVector = self_1223;
    let _e2149: MultiVector = other_1017;
    let _e2152: MultiVector = other_1017;
    let _e2155: MultiVector = other_1017;
    let _e2167: MultiVector = self_1223;
    let _e2171: MultiVector = other_1017;
    let _e2175: MultiVector = self_1223;
    let _e2179: MultiVector = other_1017;
    let _e2182: MultiVector = self_1223;
    let _e2186: MultiVector = other_1017;
    let _e2190: MultiVector = self_1223;
    let _e2194: MultiVector = other_1017;
    let _e2198: MultiVector = self_1223;
    let _e2202: MultiVector = other_1017;
    let _e2205: MultiVector = other_1017;
    let _e2208: MultiVector = other_1017;
    let _e2221: MultiVector = self_1223;
    let _e2225: MultiVector = other_1017;
    let _e2228: MultiVector = other_1017;
    let _e2231: MultiVector = other_1017;
    let _e2244: MultiVector = self_1223;
    let _e2248: MultiVector = other_1017;
    let _e2251: MultiVector = other_1017;
    let _e2254: MultiVector = other_1017;
    let _e2267: MultiVector = self_1223;
    let _e2271: MultiVector = other_1017;
    let _e2274: MultiVector = other_1017;
    let _e2277: MultiVector = other_1017;
    let _e2283: MultiVector = self_1223;
    let _e2287: MultiVector = other_1017;
    let _e2291: MultiVector = self_1223;
    let _e2295: MultiVector = other_1017;
    let _e2298: MultiVector = other_1017;
    let _e2301: MultiVector = other_1017;
    let _e2313: MultiVector = self_1223;
    let _e2317: MultiVector = other_1017;
    let _e2320: MultiVector = other_1017;
    let _e2323: MultiVector = other_1017;
    let _e2335: MultiVector = self_1223;
    let _e2339: MultiVector = other_1017;
    let _e2342: MultiVector = other_1017;
    let _e2345: MultiVector = other_1017;
    let _e2357: MultiVector = self_1223;
    let _e2361: MultiVector = other_1017;
    let _e2365: MultiVector = self_1223;
    let _e2369: MultiVector = other_1017;
    let _e2372: MultiVector = other_1017;
    let _e2375: MultiVector = other_1017;
    let _e2387: MultiVector = self_1223;
    let _e2391: MultiVector = other_1017;
    let _e2394: MultiVector = other_1017;
    let _e2397: MultiVector = other_1017;
    let _e2409: MultiVector = self_1223;
    let _e2413: MultiVector = other_1017;
    let _e2416: MultiVector = other_1017;
    let _e2419: MultiVector = other_1017;
    let _e2431: MultiVector = self_1223;
    let _e2435: MultiVector = other_1017;
    let _e2438: MultiVector = other_1017;
    let _e2441: MultiVector = other_1017;
    let _e2453: MultiVector = self_1223;
    let _e2457: MultiVector = other_1017;
    let _e2460: MultiVector = other_1017;
    let _e2463: MultiVector = other_1017;
    let _e2475: MultiVector = self_1223;
    let _e2479: MultiVector = other_1017;
    let _e2482: MultiVector = other_1017;
    let _e2485: MultiVector = other_1017;
    let _e2497: MultiVector = self_1223;
    let _e2501: MultiVector = other_1017;
    let _e2504: MultiVector = other_1017;
    let _e2507: MultiVector = other_1017;
    let _e2519: MultiVector = self_1223;
    let _e2523: MultiVector = other_1017;
    let _e2526: MultiVector = other_1017;
    let _e2529: MultiVector = other_1017;
    let _e2541: MultiVector = self_1223;
    let _e2545: MultiVector = other_1017;
    let _e2548: MultiVector = other_1017;
    let _e2551: MultiVector = other_1017;
    let _e2563: MultiVector = self_1223;
    let _e2567: MultiVector = other_1017;
    let _e2570: MultiVector = other_1017;
    let _e2573: MultiVector = other_1017;
    let _e2585: MultiVector = self_1223;
    let _e2589: MultiVector = other_1017;
    let _e2592: MultiVector = other_1017;
    let _e2595: MultiVector = other_1017;
    let _e2607: MultiVector = self_1223;
    let _e2611: MultiVector = other_1017;
    let _e2614: MultiVector = other_1017;
    let _e2617: MultiVector = other_1017;
    let _e2629: MultiVector = self_1223;
    let _e2633: MultiVector = other_1017;
    let _e2636: MultiVector = other_1017;
    let _e2639: MultiVector = other_1017;
    let _e2651: MultiVector = self_1223;
    let _e2655: MultiVector = other_1017;
    let _e2658: MultiVector = other_1017;
    let _e2661: MultiVector = other_1017;
    let _e2673: MultiVector = self_1223;
    let _e2677: MultiVector = other_1017;
    let _e2680: MultiVector = other_1017;
    let _e2683: MultiVector = other_1017;
    let _e2695: MultiVector = self_1223;
    let _e2699: MultiVector = other_1017;
    let _e2702: MultiVector = other_1017;
    let _e2705: MultiVector = other_1017;
    let _e2711: MultiVector = self_1223;
    let _e2715: MultiVector = other_1017;
    let _e2718: MultiVector = other_1017;
    let _e2721: MultiVector = other_1017;
    let _e2734: MultiVector = self_1223;
    let _e2738: MultiVector = other_1017;
    let _e2741: MultiVector = other_1017;
    let _e2744: MultiVector = other_1017;
    let _e2757: MultiVector = self_1223;
    let _e2761: MultiVector = other_1017;
    let _e2764: MultiVector = other_1017;
    let _e2767: MultiVector = other_1017;
    let _e2780: MultiVector = self_1223;
    let _e2784: MultiVector = other_1017;
    let _e2787: MultiVector = other_1017;
    let _e2790: MultiVector = other_1017;
    let _e2796: MultiVector = self_1223;
    let _e2800: MultiVector = other_1017;
    let _e2803: MultiVector = self_1223;
    let _e2807: MultiVector = other_1017;
    let _e2810: MultiVector = other_1017;
    let _e2813: MultiVector = other_1017;
    let _e2826: MultiVector = self_1223;
    let _e2830: MultiVector = other_1017;
    let _e2833: MultiVector = other_1017;
    let _e2836: MultiVector = other_1017;
    let _e2849: MultiVector = self_1223;
    let _e2853: MultiVector = other_1017;
    let _e2856: MultiVector = other_1017;
    let _e2859: MultiVector = other_1017;
    let _e2872: MultiVector = self_1223;
    let _e2876: MultiVector = other_1017;
    let _e2880: MultiVector = self_1223;
    let _e2884: MultiVector = other_1017;
    let _e2887: MultiVector = other_1017;
    let _e2890: MultiVector = other_1017;
    let _e2902: MultiVector = self_1223;
    let _e2906: MultiVector = other_1017;
    let _e2909: MultiVector = other_1017;
    let _e2912: MultiVector = other_1017;
    let _e2924: MultiVector = self_1223;
    let _e2928: MultiVector = other_1017;
    let _e2931: MultiVector = other_1017;
    let _e2934: MultiVector = other_1017;
    let _e2946: MultiVector = self_1223;
    let _e2950: MultiVector = other_1017;
    let _e2953: MultiVector = other_1017;
    let _e2956: MultiVector = other_1017;
    let _e2968: MultiVector = self_1223;
    let _e2972: MultiVector = other_1017;
    let _e2975: MultiVector = other_1017;
    let _e2978: MultiVector = other_1017;
    let _e2990: MultiVector = self_1223;
    let _e2994: MultiVector = other_1017;
    let _e2997: MultiVector = other_1017;
    let _e3000: MultiVector = other_1017;
    let _e3012: MultiVector = self_1223;
    let _e3016: MultiVector = other_1017;
    let _e3019: MultiVector = other_1017;
    let _e3022: MultiVector = other_1017;
    let _e3034: MultiVector = self_1223;
    let _e3038: MultiVector = other_1017;
    let _e3041: MultiVector = other_1017;
    let _e3044: MultiVector = other_1017;
    let _e3056: MultiVector = self_1223;
    let _e3060: MultiVector = other_1017;
    let _e3063: MultiVector = other_1017;
    let _e3066: MultiVector = other_1017;
    let _e3078: MultiVector = self_1223;
    let _e3082: MultiVector = other_1017;
    let _e3085: MultiVector = other_1017;
    let _e3088: MultiVector = other_1017;
    let _e3094: MultiVector = self_1223;
    let _e3098: MultiVector = other_1017;
    let _e3102: MultiVector = self_1223;
    let _e3106: MultiVector = other_1017;
    let _e3109: MultiVector = self_1223;
    let _e3113: MultiVector = other_1017;
    let _e3116: MultiVector = other_1017;
    let _e3119: MultiVector = other_1017;
    let _e3122: MultiVector = other_1017;
    let _e3136: MultiVector = self_1223;
    let _e3140: MultiVector = other_1017;
    let _e3143: MultiVector = other_1017;
    let _e3146: MultiVector = other_1017;
    let _e3149: MultiVector = other_1017;
    let _e3163: MultiVector = self_1223;
    let _e3167: MultiVector = other_1017;
    let _e3170: MultiVector = other_1017;
    let _e3173: MultiVector = other_1017;
    let _e3176: MultiVector = other_1017;
    let _e3190: MultiVector = self_1223;
    let _e3194: MultiVector = other_1017;
    let _e3197: MultiVector = other_1017;
    let _e3200: MultiVector = other_1017;
    let _e3203: MultiVector = other_1017;
    let _e3215: MultiVector = self_1223;
    let _e3219: MultiVector = other_1017;
    let _e3222: MultiVector = other_1017;
    let _e3225: MultiVector = other_1017;
    let _e3228: MultiVector = other_1017;
    let _e3242: MultiVector = self_1223;
    let _e3246: MultiVector = other_1017;
    let _e3249: MultiVector = other_1017;
    let _e3252: MultiVector = other_1017;
    let _e3255: MultiVector = other_1017;
    let _e3269: MultiVector = self_1223;
    let _e3273: MultiVector = other_1017;
    let _e3276: MultiVector = other_1017;
    let _e3279: MultiVector = other_1017;
    let _e3282: MultiVector = other_1017;
    let _e3296: MultiVector = self_1223;
    let _e3300: MultiVector = other_1017;
    let _e3303: MultiVector = other_1017;
    let _e3306: MultiVector = other_1017;
    let _e3309: MultiVector = other_1017;
    let _e3323: MultiVector = self_1223;
    let _e3327: MultiVector = other_1017;
    let _e3330: MultiVector = other_1017;
    let _e3333: MultiVector = other_1017;
    let _e3336: MultiVector = other_1017;
    let _e3350: MultiVector = self_1223;
    let _e3354: MultiVector = other_1017;
    let _e3357: MultiVector = other_1017;
    let _e3360: MultiVector = other_1017;
    let _e3363: MultiVector = other_1017;
    let _e3377: MultiVector = self_1223;
    let _e3381: MultiVector = other_1017;
    let _e3384: MultiVector = other_1017;
    let _e3387: MultiVector = other_1017;
    let _e3390: MultiVector = other_1017;
    let _e3403: MultiVector = self_1223;
    let _e3407: MultiVector = other_1017;
    let _e3410: MultiVector = other_1017;
    let _e3413: MultiVector = other_1017;
    let _e3416: MultiVector = other_1017;
    let _e3429: MultiVector = self_1223;
    let _e3433: MultiVector = other_1017;
    let _e3436: MultiVector = other_1017;
    let _e3439: MultiVector = other_1017;
    let _e3442: MultiVector = other_1017;
    let _e3455: MultiVector = self_1223;
    let _e3459: MultiVector = other_1017;
    let _e3462: MultiVector = other_1017;
    let _e3465: MultiVector = other_1017;
    let _e3468: MultiVector = other_1017;
    let _e3474: MultiVector = self_1223;
    let _e3477: MultiVector = self_1223;
    let _e3480: MultiVector = self_1223;
    let _e3483: MultiVector = self_1223;
    let _e3487: MultiVector = other_1017;
    let _e3490: MultiVector = other_1017;
    let _e3493: MultiVector = other_1017;
    let _e3496: MultiVector = other_1017;
    let _e3511: MultiVector = self_1223;
    let _e3515: MultiVector = other_1017;
    let _e3518: MultiVector = self_1223;
    let _e3522: MultiVector = other_1017;
    let _e3525: MultiVector = other_1017;
    let _e3528: MultiVector = other_1017;
    let _e3531: MultiVector = other_1017;
    let _e3543: MultiVector = self_1223;
    let _e3547: MultiVector = other_1017;
    let _e3550: MultiVector = other_1017;
    let _e3553: MultiVector = other_1017;
    let _e3556: MultiVector = other_1017;
    let _e3570: MultiVector = self_1223;
    let _e3574: MultiVector = other_1017;
    let _e3577: MultiVector = other_1017;
    let _e3580: MultiVector = other_1017;
    let _e3583: MultiVector = other_1017;
    let _e3597: MultiVector = self_1223;
    let _e3601: MultiVector = other_1017;
    let _e3604: MultiVector = other_1017;
    let _e3607: MultiVector = other_1017;
    let _e3610: MultiVector = other_1017;
    let _e3624: MultiVector = self_1223;
    let _e3628: MultiVector = other_1017;
    let _e3631: MultiVector = other_1017;
    let _e3634: MultiVector = other_1017;
    let _e3637: MultiVector = other_1017;
    let _e3649: MultiVector = self_1223;
    let _e3653: MultiVector = other_1017;
    let _e3657: MultiVector = self_1223;
    let _e3661: MultiVector = other_1017;
    let _e3664: MultiVector = other_1017;
    let _e3667: MultiVector = other_1017;
    let _e3670: MultiVector = other_1017;
    let _e3684: MultiVector = self_1223;
    let _e3688: MultiVector = other_1017;
    let _e3691: MultiVector = other_1017;
    let _e3694: MultiVector = other_1017;
    let _e3697: MultiVector = other_1017;
    let _e3711: MultiVector = self_1223;
    let _e3715: MultiVector = other_1017;
    let _e3718: MultiVector = other_1017;
    let _e3721: MultiVector = other_1017;
    let _e3724: MultiVector = other_1017;
    let _e3738: MultiVector = self_1223;
    let _e3742: MultiVector = other_1017;
    let _e3745: MultiVector = other_1017;
    let _e3748: MultiVector = other_1017;
    let _e3751: MultiVector = other_1017;
    let _e3763: MultiVector = self_1223;
    let _e3767: MultiVector = other_1017;
    let _e3770: MultiVector = other_1017;
    let _e3773: MultiVector = other_1017;
    let _e3776: MultiVector = other_1017;
    let _e3790: MultiVector = self_1223;
    let _e3794: MultiVector = other_1017;
    let _e3797: MultiVector = other_1017;
    let _e3800: MultiVector = other_1017;
    let _e3803: MultiVector = other_1017;
    let _e3817: MultiVector = self_1223;
    let _e3821: MultiVector = other_1017;
    let _e3824: MultiVector = other_1017;
    let _e3827: MultiVector = other_1017;
    let _e3830: MultiVector = other_1017;
    let _e3844: MultiVector = self_1223;
    let _e3848: MultiVector = other_1017;
    let _e3851: MultiVector = other_1017;
    let _e3854: MultiVector = other_1017;
    let _e3857: MultiVector = other_1017;
    let _e3871: MultiVector = self_1223;
    let _e3875: MultiVector = other_1017;
    let _e3878: MultiVector = other_1017;
    let _e3881: MultiVector = other_1017;
    let _e3884: MultiVector = other_1017;
    let _e3898: MultiVector = self_1223;
    let _e3902: MultiVector = other_1017;
    let _e3905: MultiVector = other_1017;
    let _e3908: MultiVector = other_1017;
    let _e3911: MultiVector = other_1017;
    let _e3925: MultiVector = self_1223;
    let _e3929: MultiVector = other_1017;
    let _e3932: MultiVector = other_1017;
    let _e3935: MultiVector = other_1017;
    let _e3938: MultiVector = other_1017;
    let _e3951: MultiVector = self_1223;
    let _e3955: MultiVector = other_1017;
    let _e3958: MultiVector = other_1017;
    let _e3961: MultiVector = other_1017;
    let _e3964: MultiVector = other_1017;
    let _e3977: MultiVector = self_1223;
    let _e3981: MultiVector = other_1017;
    let _e3984: MultiVector = other_1017;
    let _e3987: MultiVector = other_1017;
    let _e3990: MultiVector = other_1017;
    let _e4003: MultiVector = self_1223;
    let _e4007: MultiVector = other_1017;
    let _e4010: MultiVector = other_1017;
    let _e4013: MultiVector = other_1017;
    let _e4016: MultiVector = other_1017;
    let _e4030: MultiVector = self_1223;
    let _e4034: MultiVector = other_1017;
    let _e4037: MultiVector = other_1017;
    let _e4040: MultiVector = other_1017;
    let _e4043: MultiVector = other_1017;
    let _e4057: MultiVector = self_1223;
    let _e4061: MultiVector = other_1017;
    let _e4064: MultiVector = other_1017;
    let _e4067: MultiVector = other_1017;
    let _e4070: MultiVector = other_1017;
    let _e4084: MultiVector = self_1223;
    let _e4088: MultiVector = other_1017;
    let _e4091: MultiVector = other_1017;
    let _e4094: MultiVector = other_1017;
    let _e4097: MultiVector = other_1017;
    let _e4110: MultiVector = self_1223;
    let _e4114: MultiVector = other_1017;
    let _e4117: MultiVector = other_1017;
    let _e4120: MultiVector = other_1017;
    let _e4123: MultiVector = other_1017;
    let _e4136: MultiVector = self_1223;
    let _e4140: MultiVector = other_1017;
    let _e4143: MultiVector = other_1017;
    let _e4146: MultiVector = other_1017;
    let _e4149: MultiVector = other_1017;
    let _e4162: MultiVector = self_1223;
    let _e4166: MultiVector = other_1017;
    let _e4169: MultiVector = other_1017;
    let _e4172: MultiVector = other_1017;
    let _e4175: MultiVector = other_1017;
    let _e4181: MultiVector = self_1223;
    let _e4185: MultiVector = other_1017;
    let _e4188: MultiVector = other_1017;
    let _e4191: MultiVector = other_1017;
    let _e4194: MultiVector = other_1017;
    let _e4207: MultiVector = self_1223;
    let _e4211: MultiVector = other_1017;
    let _e4214: MultiVector = other_1017;
    let _e4217: MultiVector = other_1017;
    let _e4220: MultiVector = other_1017;
    let _e4233: MultiVector = self_1223;
    let _e4237: MultiVector = other_1017;
    let _e4240: MultiVector = other_1017;
    let _e4243: MultiVector = other_1017;
    let _e4246: MultiVector = other_1017;
    let _e4259: MultiVector = self_1223;
    let _e4263: MultiVector = other_1017;
    let _e4266: MultiVector = other_1017;
    let _e4269: MultiVector = other_1017;
    let _e4272: MultiVector = other_1017;
    let _e4278: MultiVector = self_1223;
    let _e4281: MultiVector = self_1223;
    let _e4284: MultiVector = self_1223;
    let _e4287: MultiVector = self_1223;
    let _e4291: MultiVector = other_1017;
    return MultiVector(((((((((((((((((((((((((((((((((vec3<f32>(_e4.g0_.x) * _e8.g0_) + ((vec3<f32>(_e11.g0_.z) * vec3<f32>(_e15.g0_.x)) * vec3<f32>(0.0, 0.0, 1.0))) + (vec3<f32>(_e26.g1_.x) * vec3<f32>(_e30.g1_.x, _e33.g8_.x, _e36.g9_.x))) + (vec3<f32>(_e42.g1_.y) * vec3<f32>(_e46.g1_.y, _e49.g8_.y, _e52.g9_.y))) + (vec3<f32>(_e58.g1_.z) * vec3<f32>(_e62.g1_.z, _e65.g8_.z, _e68.g9_.z))) + ((vec3<f32>(_e74.g2_.x) * vec3<f32>(_e78.g8_.w, _e81.g8_.w, _e84.g9_.w)) * vec3<f32>(0.0, 1.0, 1.0))) + ((vec3<f32>(_e95.g2_.y) * vec3<f32>(_e99.g0_.y)) * vec3<f32>(0.0, 0.0, 1.0))) + ((vec3<f32>(_e110.g3_.x) * vec3<f32>(_e114.g8_.x)) * vec3<f32>(0.0, 0.0, -(1.0)))) + ((vec3<f32>(_e126.g3_.y) * vec3<f32>(_e130.g8_.y)) * vec3<f32>(0.0, 0.0, -(1.0)))) + ((vec3<f32>(_e142.g3_.z) * vec3<f32>(_e146.g8_.z)) * vec3<f32>(0.0, 0.0, -(1.0)))) + ((vec3<f32>(_e158.g3_.w) * vec3<f32>(_e162.g8_.w)) * vec3<f32>(0.0, 0.0, -(1.0)))) + ((vec3<f32>(_e174.g4_.x) * vec3<f32>(_e178.g5_.x, _e181.g5_.x, _e184.g7_.x)) * vec3<f32>(0.0, -(1.0), -(1.0)))) + ((vec3<f32>(_e197.g4_.y) * vec3<f32>(_e201.g5_.y, _e204.g5_.y, _e207.g7_.y)) * vec3<f32>(0.0, -(1.0), -(1.0)))) + ((vec3<f32>(_e220.g4_.z) * vec3<f32>(_e224.g5_.z, _e227.g5_.z, _e230.g7_.z)) * vec3<f32>(0.0, -(1.0), -(1.0)))) - (vec3<f32>(_e243.g5_.x) * vec3<f32>(_e247.g5_.x, _e250.g4_.x, _e253.g6_.x))) - (vec3<f32>(_e259.g5_.y) * vec3<f32>(_e263.g5_.y, _e266.g4_.y, _e269.g6_.y))) - (vec3<f32>(_e275.g5_.z) * vec3<f32>(_e279.g5_.z, _e282.g4_.z, _e285.g6_.z))) + ((vec3<f32>(_e291.g6_.x) * vec3<f32>(_e295.g5_.x)) * vec3<f32>(0.0, 0.0, -(1.0)))) + ((vec3<f32>(_e307.g6_.y) * vec3<f32>(_e311.g5_.y)) * vec3<f32>(0.0, 0.0, -(1.0)))) + ((vec3<f32>(_e323.g6_.z) * vec3<f32>(_e327.g5_.z)) * vec3<f32>(0.0, 0.0, -(1.0)))) + ((vec3<f32>(_e339.g7_.x) * vec3<f32>(_e343.g4_.x)) * vec3<f32>(0.0, 0.0, -(1.0)))) + ((vec3<f32>(_e355.g7_.y) * vec3<f32>(_e359.g4_.y)) * vec3<f32>(0.0, 0.0, -(1.0)))) + ((vec3<f32>(_e371.g7_.z) * vec3<f32>(_e375.g4_.z)) * vec3<f32>(0.0, 0.0, -(1.0)))) + ((vec3<f32>(_e387.g8_.x) * vec3<f32>(_e391.g1_.x, _e394.g1_.x, _e397.g3_.x)) * vec3<f32>(0.0, -(1.0), -(1.0)))) + ((vec3<f32>(_e410.g8_.y) * vec3<f32>(_e414.g1_.y, _e417.g1_.y, _e420.g3_.y)) * vec3<f32>(0.0, -(1.0), -(1.0)))) + ((vec3<f32>(_e433.g8_.z) * vec3<f32>(_e437.g1_.z, _e440.g1_.z, _e443.g3_.z)) * vec3<f32>(0.0, -(1.0), -(1.0)))) - (vec3<f32>(_e456.g8_.w) * vec3<f32>(_e460.g8_.w, _e463.g2_.x, _e466.g3_.w))) + ((vec3<f32>(_e472.g9_.x) * vec3<f32>(_e476.g1_.x)) * vec3<f32>(0.0, 0.0, 1.0))) + ((vec3<f32>(_e487.g9_.y) * vec3<f32>(_e491.g1_.y)) * vec3<f32>(0.0, 0.0, 1.0))) + ((vec3<f32>(_e502.g9_.z) * vec3<f32>(_e506.g1_.z)) * vec3<f32>(0.0, 0.0, 1.0))) + ((vec3<f32>(_e517.g9_.w) * vec3<f32>(_e521.g2_.x)) * vec3<f32>(0.0, 0.0, 1.0))) + ((_e532.g0_.xyy * vec3<f32>(_e535.g0_.x, _e538.g0_.x, _e541.g2_.y)) * vec3<f32>(0.0, 1.0, 1.0))), ((((((((vec3<f32>(_e552.g0_.x) * _e556.g1_) + ((vec3<f32>(_e559.g1_.x) * vec3<f32>(_e563.g0_.x, _e566.g5_.z, _e569.g5_.y)) * vec3<f32>(1.0, 1.0, -(1.0)))) + ((vec3<f32>(_e581.g1_.y) * vec3<f32>(_e585.g5_.z, _e588.g0_.x, _e591.g5_.x)) * vec3<f32>(-(1.0), 1.0, 1.0))) + ((vec3<f32>(_e603.g1_.z) * vec3<f32>(_e607.g5_.y, _e610.g5_.x, _e613.g0_.x)) * vec3<f32>(1.0, -(1.0), 1.0))) + ((vec3<f32>(_e625.g5_.x) * vec3<f32>(_e629.g8_.w, _e632.g1_.z, _e635.g1_.y)) * vec3<f32>(1.0, 1.0, -(1.0)))) + ((vec3<f32>(_e647.g5_.y) * vec3<f32>(_e651.g1_.z, _e654.g8_.w, _e657.g1_.x)) * vec3<f32>(-(1.0), 1.0, 1.0))) + ((vec3<f32>(_e669.g5_.z) * vec3<f32>(_e673.g1_.y, _e676.g1_.x, _e679.g8_.w)) * vec3<f32>(1.0, -(1.0), 1.0))) + (vec3<f32>(_e691.g8_.w) * _e695.g5_)), ((((((((((((((((((((((((vec2<f32>(_e699.g0_.x) * _e703.g2_) + ((vec2<f32>(_e706.g1_.x) * vec2<f32>(_e710.g4_.x, _e713.g3_.x)) * vec2<f32>(-(1.0), 1.0))) + ((vec2<f32>(_e724.g1_.y) * vec2<f32>(_e728.g4_.y, _e731.g3_.y)) * vec2<f32>(-(1.0), 1.0))) + ((vec2<f32>(_e742.g1_.z) * vec2<f32>(_e746.g4_.z, _e749.g3_.z)) * vec2<f32>(-(1.0), 1.0))) + ((vec2<f32>(_e760.g2_.x) * vec2<f32>(_e764.g0_.x)) * vec2<f32>(1.0, 0.0))) + ((vec2<f32>(_e774.g2_.y) * vec2<f32>(_e778.g0_.x)) * vec2<f32>(0.0, 1.0))) + ((vec2<f32>(_e788.g3_.x) * vec2<f32>(_e792.g1_.x)) * vec2<f32>(0.0, -(1.0)))) + ((vec2<f32>(_e803.g3_.y) * vec2<f32>(_e807.g1_.y)) * vec2<f32>(0.0, -(1.0)))) + ((vec2<f32>(_e818.g3_.z) * vec2<f32>(_e822.g1_.z)) * vec2<f32>(0.0, -(1.0)))) + ((vec2<f32>(_e833.g4_.x) * vec2<f32>(_e837.g1_.x)) * vec2<f32>(1.0, 0.0))) + ((vec2<f32>(_e847.g4_.y) * vec2<f32>(_e851.g1_.y)) * vec2<f32>(1.0, 0.0))) + ((vec2<f32>(_e861.g4_.z) * vec2<f32>(_e865.g1_.z)) * vec2<f32>(1.0, 0.0))) - (vec2<f32>(_e875.g5_.x) * vec2<f32>(_e879.g8_.x, _e882.g7_.x))) - (vec2<f32>(_e888.g5_.y) * vec2<f32>(_e892.g8_.y, _e895.g7_.y))) - (vec2<f32>(_e901.g5_.z) * vec2<f32>(_e905.g8_.z, _e908.g7_.z))) + ((vec2<f32>(_e914.g7_.x) * vec2<f32>(_e918.g5_.x)) * vec2<f32>(0.0, -(1.0)))) + ((vec2<f32>(_e929.g7_.y) * vec2<f32>(_e933.g5_.y)) * vec2<f32>(0.0, -(1.0)))) + ((vec2<f32>(_e944.g7_.z) * vec2<f32>(_e948.g5_.z)) * vec2<f32>(0.0, -(1.0)))) + ((vec2<f32>(_e959.g8_.x) * vec2<f32>(_e963.g5_.x)) * vec2<f32>(-(1.0), 0.0))) + ((vec2<f32>(_e974.g8_.y) * vec2<f32>(_e978.g5_.y)) * vec2<f32>(-(1.0), 0.0))) + ((vec2<f32>(_e989.g8_.z) * vec2<f32>(_e993.g5_.z)) * vec2<f32>(-(1.0), 0.0))) + ((vec2<f32>(_e1004.g8_.w) * vec2<f32>(_e1008.g0_.y, _e1011.g9_.w)) * vec2<f32>(1.0, -(1.0)))) + ((vec2<f32>(_e1022.g9_.w) * vec2<f32>(_e1026.g8_.w)) * vec2<f32>(0.0, 1.0))) + ((vec2<f32>(_e1036.g0_.y, _e1039.g0_.x) * vec2<f32>(_e1043.g8_.w, _e1046.g8_.x)) * vec2<f32>(-(1.0), 0.0))), ((((((((((((((((((((((((((((((((vec4<f32>(_e1057.g0_.x) * _e1061.g3_) + ((vec4<f32>(_e1064.g0_.z) * vec4<f32>(_e1068.g8_.w)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e1080.g1_.x) * vec4<f32>(_e1084.g2_.y, _e1087.g7_.z, _e1090.g7_.y, _e1093.g6_.x)) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e1107.g1_.y) * vec4<f32>(_e1111.g7_.z, _e1114.g2_.y, _e1117.g7_.x, _e1120.g6_.y)) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e1134.g1_.z) * vec4<f32>(_e1138.g7_.y, _e1141.g7_.x, _e1144.g2_.y, _e1147.g6_.z)) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e1161.g2_.x) * vec4<f32>(_e1165.g2_.y)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) - (vec4<f32>(_e1177.g2_.y) * vec4<f32>(_e1181.g1_.x, _e1184.g1_.y, _e1187.g1_.z, _e1190.g2_.x))) + ((vec4<f32>(_e1196.g3_.x) * vec4<f32>(_e1200.g0_.x, _e1203.g5_.z, _e1206.g5_.y, _e1209.g4_.x)) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e1223.g3_.y) * vec4<f32>(_e1227.g5_.z, _e1230.g0_.x, _e1233.g5_.x, _e1236.g4_.y)) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e1250.g3_.z) * vec4<f32>(_e1254.g5_.y, _e1257.g5_.x, _e1260.g0_.x, _e1263.g4_.z)) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e1277.g3_.w) * vec4<f32>(_e1281.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e1293.g4_.x) * vec4<f32>(_e1297.g3_.x)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e1309.g4_.y) * vec4<f32>(_e1313.g3_.y)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e1325.g4_.z) * vec4<f32>(_e1329.g3_.z)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e1341.g5_.x) * vec4<f32>(_e1345.g9_.w, _e1348.g3_.z, _e1351.g3_.y, _e1354.g9_.x)) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e1368.g5_.y) * vec4<f32>(_e1372.g3_.z, _e1375.g9_.w, _e1378.g3_.x, _e1381.g9_.y)) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e1395.g5_.z) * vec4<f32>(_e1399.g3_.y, _e1402.g3_.x, _e1405.g9_.w, _e1408.g9_.z)) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e1422.g6_.x) * vec4<f32>(_e1426.g1_.x)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e1439.g6_.y) * vec4<f32>(_e1443.g1_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e1456.g6_.z) * vec4<f32>(_e1460.g1_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e1473.g7_.x) * vec4<f32>(_e1477.g8_.w, _e1480.g1_.z, _e1483.g1_.y, _e1486.g8_.x)) * vec4<f32>(-(1.0), -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e1500.g7_.y) * vec4<f32>(_e1504.g1_.z, _e1507.g8_.w, _e1510.g1_.x, _e1513.g8_.y)) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e1527.g7_.z) * vec4<f32>(_e1531.g1_.y, _e1534.g1_.x, _e1537.g8_.w, _e1540.g8_.z)) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))) + ((vec4<f32>(_e1554.g8_.x) * vec4<f32>(_e1558.g7_.x)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e1571.g8_.y) * vec4<f32>(_e1575.g7_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e1588.g8_.z) * vec4<f32>(_e1592.g7_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + (vec4<f32>(_e1605.g8_.w) * vec4<f32>(_e1609.g7_.x, _e1612.g7_.y, _e1615.g7_.z, _e1618.g0_.z))) + ((vec4<f32>(_e1624.g9_.x) * vec4<f32>(_e1628.g5_.x)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e1641.g9_.y) * vec4<f32>(_e1645.g5_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e1658.g9_.z) * vec4<f32>(_e1662.g5_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + (vec4<f32>(_e1675.g9_.w) * vec4<f32>(_e1679.g5_.x, _e1682.g5_.y, _e1685.g5_.z, _e1688.g0_.y))) + ((vec4<f32>(_e1694.g0_.x, _e1697.g0_.x, _e1700.g0_.x, _e1703.g0_.y) * _e1707.g9_.xxxw) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))), ((((((((((((((((vec3<f32>(_e1719.g0_.x) * _e1723.g4_) + (vec3<f32>(_e1726.g0_.y) * _e1730.g5_)) + ((vec3<f32>(_e1734.g1_.x) * vec3<f32>(_e1738.g2_.x, _e1741.g8_.z, _e1744.g8_.y)) * vec3<f32>(-(1.0), -(1.0), 1.0))) + ((vec3<f32>(_e1757.g1_.y) * vec3<f32>(_e1761.g8_.z, _e1764.g2_.x, _e1767.g8_.x)) * vec3<f32>(1.0, -(1.0), -(1.0)))) + ((vec3<f32>(_e1780.g1_.z) * vec3<f32>(_e1784.g8_.y, _e1787.g8_.x, _e1790.g2_.x)) * vec3<f32>(-(1.0), 1.0, -(1.0)))) + (vec3<f32>(_e1803.g2_.x) * _e1807.g1_)) + ((vec3<f32>(_e1811.g4_.x) * vec3<f32>(_e1815.g0_.x, _e1818.g5_.z, _e1821.g5_.y)) * vec3<f32>(1.0, 1.0, -(1.0)))) + ((vec3<f32>(_e1833.g4_.y) * vec3<f32>(_e1837.g5_.z, _e1840.g0_.x, _e1843.g5_.x)) * vec3<f32>(-(1.0), 1.0, 1.0))) + ((vec3<f32>(_e1855.g4_.z) * vec3<f32>(_e1859.g5_.y, _e1862.g5_.x, _e1865.g0_.x)) * vec3<f32>(1.0, -(1.0), 1.0))) + ((vec3<f32>(_e1877.g5_.x) * vec3<f32>(_e1881.g0_.y, _e1884.g4_.z, _e1887.g4_.y)) * vec3<f32>(1.0, 1.0, -(1.0)))) + ((vec3<f32>(_e1899.g5_.y) * vec3<f32>(_e1903.g4_.z, _e1906.g0_.y, _e1909.g4_.x)) * vec3<f32>(-(1.0), 1.0, 1.0))) + ((vec3<f32>(_e1921.g5_.z) * vec3<f32>(_e1925.g4_.y, _e1928.g4_.x, _e1931.g0_.y)) * vec3<f32>(1.0, -(1.0), 1.0))) + ((vec3<f32>(_e1943.g8_.x) * vec3<f32>(_e1947.g8_.w, _e1950.g1_.z, _e1953.g1_.y)) * vec3<f32>(1.0, 1.0, -(1.0)))) + ((vec3<f32>(_e1965.g8_.y) * vec3<f32>(_e1969.g1_.z, _e1972.g8_.w, _e1975.g1_.x)) * vec3<f32>(-(1.0), 1.0, 1.0))) + ((vec3<f32>(_e1987.g8_.z) * vec3<f32>(_e1991.g1_.y, _e1994.g1_.x, _e1997.g8_.w)) * vec3<f32>(1.0, -(1.0), 1.0))) - (vec3<f32>(_e2009.g8_.w) * vec3<f32>(_e2013.g8_.x, _e2016.g8_.y, _e2019.g8_.z))), ((((((((vec3<f32>(_e2025.g0_.x) * _e2029.g5_) + ((vec3<f32>(_e2032.g1_.x) * vec3<f32>(_e2036.g8_.w, _e2039.g1_.z, _e2042.g1_.y)) * vec3<f32>(-(1.0), -(1.0), 1.0))) + ((vec3<f32>(_e2055.g1_.y) * vec3<f32>(_e2059.g1_.z, _e2062.g8_.w, _e2065.g1_.x)) * vec3<f32>(1.0, -(1.0), -(1.0)))) + ((vec3<f32>(_e2078.g1_.z) * vec3<f32>(_e2082.g1_.y, _e2085.g1_.x, _e2088.g8_.w)) * vec3<f32>(-(1.0), 1.0, -(1.0)))) + ((vec3<f32>(_e2101.g5_.x) * vec3<f32>(_e2105.g0_.x, _e2108.g5_.z, _e2111.g5_.y)) * vec3<f32>(1.0, 1.0, -(1.0)))) + ((vec3<f32>(_e2123.g5_.y) * vec3<f32>(_e2127.g5_.z, _e2130.g0_.x, _e2133.g5_.x)) * vec3<f32>(-(1.0), 1.0, 1.0))) + ((vec3<f32>(_e2145.g5_.z) * vec3<f32>(_e2149.g5_.y, _e2152.g5_.x, _e2155.g0_.x)) * vec3<f32>(1.0, -(1.0), 1.0))) - (vec3<f32>(_e2167.g8_.w) * _e2171.g1_)), ((((((((((((((((((((((((((((((((vec3<f32>(_e2175.g0_.x) * _e2179.g6_) + (vec3<f32>(_e2182.g0_.y) * _e2186.g7_)) + (vec3<f32>(_e2190.g0_.z) * _e2194.g5_)) + ((vec3<f32>(_e2198.g1_.x) * vec3<f32>(_e2202.g3_.w, _e2205.g9_.z, _e2208.g9_.y)) * vec3<f32>(-(1.0), -(1.0), 1.0))) + ((vec3<f32>(_e2221.g1_.y) * vec3<f32>(_e2225.g9_.z, _e2228.g3_.w, _e2231.g9_.x)) * vec3<f32>(1.0, -(1.0), -(1.0)))) + ((vec3<f32>(_e2244.g1_.z) * vec3<f32>(_e2248.g9_.y, _e2251.g9_.x, _e2254.g3_.w)) * vec3<f32>(-(1.0), 1.0, -(1.0)))) + (vec3<f32>(_e2267.g2_.x) * vec3<f32>(_e2271.g3_.x, _e2274.g3_.y, _e2277.g3_.z))) + (vec3<f32>(_e2283.g2_.y) * _e2287.g4_)) + ((vec3<f32>(_e2291.g3_.x) * vec3<f32>(_e2295.g2_.x, _e2298.g8_.z, _e2301.g8_.y)) * vec3<f32>(1.0, 1.0, -(1.0)))) + ((vec3<f32>(_e2313.g3_.y) * vec3<f32>(_e2317.g8_.z, _e2320.g2_.x, _e2323.g8_.x)) * vec3<f32>(-(1.0), 1.0, 1.0))) + ((vec3<f32>(_e2335.g3_.z) * vec3<f32>(_e2339.g8_.y, _e2342.g8_.x, _e2345.g2_.x)) * vec3<f32>(1.0, -(1.0), 1.0))) - (vec3<f32>(_e2357.g3_.w) * _e2361.g1_)) + ((vec3<f32>(_e2365.g4_.x) * vec3<f32>(_e2369.g2_.y, _e2372.g7_.z, _e2375.g7_.y)) * vec3<f32>(1.0, 1.0, -(1.0)))) + ((vec3<f32>(_e2387.g4_.y) * vec3<f32>(_e2391.g7_.z, _e2394.g2_.y, _e2397.g7_.x)) * vec3<f32>(-(1.0), 1.0, 1.0))) + ((vec3<f32>(_e2409.g4_.z) * vec3<f32>(_e2413.g7_.y, _e2416.g7_.x, _e2419.g2_.y)) * vec3<f32>(1.0, -(1.0), 1.0))) + ((vec3<f32>(_e2431.g5_.x) * vec3<f32>(_e2435.g0_.z, _e2438.g6_.z, _e2441.g6_.y)) * vec3<f32>(1.0, 1.0, -(1.0)))) + ((vec3<f32>(_e2453.g5_.y) * vec3<f32>(_e2457.g6_.z, _e2460.g0_.z, _e2463.g6_.x)) * vec3<f32>(-(1.0), 1.0, 1.0))) + ((vec3<f32>(_e2475.g5_.z) * vec3<f32>(_e2479.g6_.y, _e2482.g6_.x, _e2485.g0_.z)) * vec3<f32>(1.0, -(1.0), 1.0))) + ((vec3<f32>(_e2497.g6_.x) * vec3<f32>(_e2501.g0_.x, _e2504.g5_.z, _e2507.g5_.y)) * vec3<f32>(1.0, 1.0, -(1.0)))) + ((vec3<f32>(_e2519.g6_.y) * vec3<f32>(_e2523.g5_.z, _e2526.g0_.x, _e2529.g5_.x)) * vec3<f32>(-(1.0), 1.0, 1.0))) + ((vec3<f32>(_e2541.g6_.z) * vec3<f32>(_e2545.g5_.y, _e2548.g5_.x, _e2551.g0_.x)) * vec3<f32>(1.0, -(1.0), 1.0))) + ((vec3<f32>(_e2563.g7_.x) * vec3<f32>(_e2567.g0_.y, _e2570.g4_.z, _e2573.g4_.y)) * vec3<f32>(1.0, 1.0, -(1.0)))) + ((vec3<f32>(_e2585.g7_.y) * vec3<f32>(_e2589.g4_.z, _e2592.g0_.y, _e2595.g4_.x)) * vec3<f32>(-(1.0), 1.0, 1.0))) + ((vec3<f32>(_e2607.g7_.z) * vec3<f32>(_e2611.g4_.y, _e2614.g4_.x, _e2617.g0_.y)) * vec3<f32>(1.0, -(1.0), 1.0))) + ((vec3<f32>(_e2629.g8_.x) * vec3<f32>(_e2633.g9_.w, _e2636.g3_.z, _e2639.g3_.y)) * vec3<f32>(1.0, 1.0, -(1.0)))) + ((vec3<f32>(_e2651.g8_.y) * vec3<f32>(_e2655.g3_.z, _e2658.g9_.w, _e2661.g3_.x)) * vec3<f32>(-(1.0), 1.0, 1.0))) + ((vec3<f32>(_e2673.g8_.z) * vec3<f32>(_e2677.g3_.y, _e2680.g3_.x, _e2683.g9_.w)) * vec3<f32>(1.0, -(1.0), 1.0))) - (vec3<f32>(_e2695.g8_.w) * vec3<f32>(_e2699.g9_.x, _e2702.g9_.y, _e2705.g9_.z))) + ((vec3<f32>(_e2711.g9_.x) * vec3<f32>(_e2715.g8_.w, _e2718.g1_.z, _e2721.g1_.y)) * vec3<f32>(-(1.0), -(1.0), 1.0))) + ((vec3<f32>(_e2734.g9_.y) * vec3<f32>(_e2738.g1_.z, _e2741.g8_.w, _e2744.g1_.x)) * vec3<f32>(1.0, -(1.0), -(1.0)))) + ((vec3<f32>(_e2757.g9_.z) * vec3<f32>(_e2761.g1_.y, _e2764.g1_.x, _e2767.g8_.w)) * vec3<f32>(-(1.0), 1.0, -(1.0)))) + (vec3<f32>(_e2780.g9_.w) * vec3<f32>(_e2784.g8_.x, _e2787.g8_.y, _e2790.g8_.z))), ((((((((((((((((vec3<f32>(_e2796.g0_.x) * _e2800.g7_) + ((vec3<f32>(_e2803.g1_.x) * vec3<f32>(_e2807.g9_.w, _e2810.g3_.z, _e2813.g3_.y)) * vec3<f32>(-(1.0), -(1.0), 1.0))) + ((vec3<f32>(_e2826.g1_.y) * vec3<f32>(_e2830.g3_.z, _e2833.g9_.w, _e2836.g3_.x)) * vec3<f32>(1.0, -(1.0), -(1.0)))) + ((vec3<f32>(_e2849.g1_.z) * vec3<f32>(_e2853.g3_.y, _e2856.g3_.x, _e2859.g9_.w)) * vec3<f32>(-(1.0), 1.0, -(1.0)))) + (vec3<f32>(_e2872.g2_.y) * _e2876.g5_)) + ((vec3<f32>(_e2880.g3_.x) * vec3<f32>(_e2884.g8_.w, _e2887.g1_.z, _e2890.g1_.y)) * vec3<f32>(1.0, 1.0, -(1.0)))) + ((vec3<f32>(_e2902.g3_.y) * vec3<f32>(_e2906.g1_.z, _e2909.g8_.w, _e2912.g1_.x)) * vec3<f32>(-(1.0), 1.0, 1.0))) + ((vec3<f32>(_e2924.g3_.z) * vec3<f32>(_e2928.g1_.y, _e2931.g1_.x, _e2934.g8_.w)) * vec3<f32>(1.0, -(1.0), 1.0))) + ((vec3<f32>(_e2946.g5_.x) * vec3<f32>(_e2950.g2_.y, _e2953.g7_.z, _e2956.g7_.y)) * vec3<f32>(1.0, 1.0, -(1.0)))) + ((vec3<f32>(_e2968.g5_.y) * vec3<f32>(_e2972.g7_.z, _e2975.g2_.y, _e2978.g7_.x)) * vec3<f32>(-(1.0), 1.0, 1.0))) + ((vec3<f32>(_e2990.g5_.z) * vec3<f32>(_e2994.g7_.y, _e2997.g7_.x, _e3000.g2_.y)) * vec3<f32>(1.0, -(1.0), 1.0))) + ((vec3<f32>(_e3012.g7_.x) * vec3<f32>(_e3016.g0_.x, _e3019.g5_.z, _e3022.g5_.y)) * vec3<f32>(1.0, 1.0, -(1.0)))) + ((vec3<f32>(_e3034.g7_.y) * vec3<f32>(_e3038.g5_.z, _e3041.g0_.x, _e3044.g5_.x)) * vec3<f32>(-(1.0), 1.0, 1.0))) + ((vec3<f32>(_e3056.g7_.z) * vec3<f32>(_e3060.g5_.y, _e3063.g5_.x, _e3066.g0_.x)) * vec3<f32>(1.0, -(1.0), 1.0))) - (vec3<f32>(_e3078.g8_.w) * vec3<f32>(_e3082.g3_.x, _e3085.g3_.y, _e3088.g3_.z))) + (vec3<f32>(_e3094.g9_.w) * _e3098.g1_)), ((((((((((((((((vec4<f32>(_e3102.g0_.x) * _e3106.g8_) + ((vec4<f32>(_e3109.g1_.x) * vec4<f32>(_e3113.g0_.y, _e3116.g4_.z, _e3119.g4_.y, _e3122.g5_.x)) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e3136.g1_.y) * vec4<f32>(_e3140.g4_.z, _e3143.g0_.y, _e3146.g4_.x, _e3149.g5_.y)) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e3163.g1_.z) * vec4<f32>(_e3167.g4_.y, _e3170.g4_.x, _e3173.g0_.y, _e3176.g5_.z)) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e3190.g2_.x) * vec4<f32>(_e3194.g5_.x, _e3197.g5_.y, _e3200.g5_.z, _e3203.g5_.x)) * vec4<f32>(1.0, 1.0, 1.0, 0.0))) + ((vec4<f32>(_e3215.g4_.x) * vec4<f32>(_e3219.g8_.w, _e3222.g1_.z, _e3225.g1_.y, _e3228.g8_.w)) * vec4<f32>(-(1.0), -(1.0), 1.0, 0.0))) + ((vec4<f32>(_e3242.g4_.y) * vec4<f32>(_e3246.g1_.z, _e3249.g8_.w, _e3252.g1_.x, _e3255.g1_.z)) * vec4<f32>(1.0, -(1.0), -(1.0), 0.0))) + ((vec4<f32>(_e3269.g4_.z) * vec4<f32>(_e3273.g1_.y, _e3276.g1_.x, _e3279.g8_.w, _e3282.g1_.y)) * vec4<f32>(-(1.0), 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e3296.g5_.x) * vec4<f32>(_e3300.g2_.x, _e3303.g8_.z, _e3306.g8_.y, _e3309.g1_.x)) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e3323.g5_.y) * vec4<f32>(_e3327.g8_.z, _e3330.g2_.x, _e3333.g8_.x, _e3336.g1_.y)) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e3350.g5_.z) * vec4<f32>(_e3354.g8_.y, _e3357.g8_.x, _e3360.g2_.x, _e3363.g1_.z)) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e3377.g8_.x) * vec4<f32>(_e3381.g0_.x, _e3384.g5_.z, _e3387.g5_.y, _e3390.g0_.x)) * vec4<f32>(1.0, 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e3403.g8_.y) * vec4<f32>(_e3407.g5_.z, _e3410.g0_.x, _e3413.g5_.x, _e3416.g5_.z)) * vec4<f32>(-(1.0), 1.0, 1.0, 0.0))) + ((vec4<f32>(_e3429.g8_.z) * vec4<f32>(_e3433.g5_.y, _e3436.g5_.x, _e3439.g0_.x, _e3442.g5_.y)) * vec4<f32>(1.0, -(1.0), 1.0, 0.0))) + (vec4<f32>(_e3455.g8_.w) * vec4<f32>(_e3459.g4_.x, _e3462.g4_.y, _e3465.g4_.z, _e3468.g0_.x))) + ((vec4<f32>(_e3474.g0_.y, _e3477.g0_.y, _e3480.g0_.y, _e3483.g0_.x) * vec4<f32>(_e3487.g1_.x, _e3490.g1_.y, _e3493.g1_.z, _e3496.g1_.x)) * vec4<f32>(-(1.0), -(1.0), -(1.0), 0.0))), ((((((((((((((((((((((((((((((((vec4<f32>(_e3511.g0_.x) * _e3515.g9_) + ((vec4<f32>(_e3518.g0_.z) * vec4<f32>(_e3522.g1_.x, _e3525.g1_.y, _e3528.g1_.z, _e3531.g1_.x)) * vec4<f32>(1.0, 1.0, 1.0, 0.0))) + ((vec4<f32>(_e3543.g1_.x) * vec4<f32>(_e3547.g0_.z, _e3550.g6_.z, _e3553.g6_.y, _e3556.g7_.x)) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e3570.g1_.y) * vec4<f32>(_e3574.g6_.z, _e3577.g0_.z, _e3580.g6_.x, _e3583.g7_.y)) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e3597.g1_.z) * vec4<f32>(_e3601.g6_.y, _e3604.g6_.x, _e3607.g0_.z, _e3610.g7_.z)) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e3624.g2_.x) * vec4<f32>(_e3628.g7_.x, _e3631.g7_.y, _e3634.g7_.z, _e3637.g7_.x)) * vec4<f32>(1.0, 1.0, 1.0, 0.0))) - (vec4<f32>(_e3649.g2_.y) * _e3653.g8_)) + ((vec4<f32>(_e3657.g3_.x) * vec4<f32>(_e3661.g0_.y, _e3664.g4_.z, _e3667.g4_.y, _e3670.g5_.x)) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e3684.g3_.y) * vec4<f32>(_e3688.g4_.z, _e3691.g0_.y, _e3694.g4_.x, _e3697.g5_.y)) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e3711.g3_.z) * vec4<f32>(_e3715.g4_.y, _e3718.g4_.x, _e3721.g0_.y, _e3724.g5_.z)) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e3738.g3_.w) * vec4<f32>(_e3742.g5_.x, _e3745.g5_.y, _e3748.g5_.z, _e3751.g5_.x)) * vec4<f32>(1.0, 1.0, 1.0, 0.0))) + ((vec4<f32>(_e3763.g4_.x) * vec4<f32>(_e3767.g9_.w, _e3770.g3_.z, _e3773.g3_.y, _e3776.g9_.w)) * vec4<f32>(-(1.0), -(1.0), 1.0, 0.0))) + ((vec4<f32>(_e3790.g4_.y) * vec4<f32>(_e3794.g3_.z, _e3797.g9_.w, _e3800.g3_.x, _e3803.g3_.z)) * vec4<f32>(1.0, -(1.0), -(1.0), 0.0))) + ((vec4<f32>(_e3817.g4_.z) * vec4<f32>(_e3821.g3_.y, _e3824.g3_.x, _e3827.g9_.w, _e3830.g3_.y)) * vec4<f32>(-(1.0), 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e3844.g5_.x) * vec4<f32>(_e3848.g3_.w, _e3851.g9_.z, _e3854.g9_.y, _e3857.g3_.x)) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e3871.g5_.y) * vec4<f32>(_e3875.g9_.z, _e3878.g3_.w, _e3881.g9_.x, _e3884.g3_.y)) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e3898.g5_.z) * vec4<f32>(_e3902.g9_.y, _e3905.g9_.x, _e3908.g3_.w, _e3911.g3_.z)) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e3925.g6_.x) * vec4<f32>(_e3929.g8_.w, _e3932.g1_.z, _e3935.g1_.y, _e3938.g8_.w)) * vec4<f32>(1.0, 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e3951.g6_.y) * vec4<f32>(_e3955.g1_.z, _e3958.g8_.w, _e3961.g1_.x, _e3964.g1_.z)) * vec4<f32>(-(1.0), 1.0, 1.0, 0.0))) + ((vec4<f32>(_e3977.g6_.z) * vec4<f32>(_e3981.g1_.y, _e3984.g1_.x, _e3987.g8_.w, _e3990.g1_.y)) * vec4<f32>(1.0, -(1.0), 1.0, 0.0))) + ((vec4<f32>(_e4003.g7_.x) * vec4<f32>(_e4007.g2_.x, _e4010.g8_.z, _e4013.g8_.y, _e4016.g1_.x)) * vec4<f32>(-(1.0), -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e4030.g7_.y) * vec4<f32>(_e4034.g8_.z, _e4037.g2_.x, _e4040.g8_.x, _e4043.g1_.y)) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e4057.g7_.z) * vec4<f32>(_e4061.g8_.y, _e4064.g8_.x, _e4067.g2_.x, _e4070.g1_.z)) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))) + ((vec4<f32>(_e4084.g8_.x) * vec4<f32>(_e4088.g2_.y, _e4091.g7_.z, _e4094.g7_.y, _e4097.g2_.y)) * vec4<f32>(1.0, 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e4110.g8_.y) * vec4<f32>(_e4114.g7_.z, _e4117.g2_.y, _e4120.g7_.x, _e4123.g7_.z)) * vec4<f32>(-(1.0), 1.0, 1.0, 0.0))) + ((vec4<f32>(_e4136.g8_.z) * vec4<f32>(_e4140.g7_.y, _e4143.g7_.x, _e4146.g2_.y, _e4149.g7_.y)) * vec4<f32>(1.0, -(1.0), 1.0, 0.0))) + (vec4<f32>(_e4162.g8_.w) * vec4<f32>(_e4166.g6_.x, _e4169.g6_.y, _e4172.g6_.z, _e4175.g2_.y))) + ((vec4<f32>(_e4181.g9_.x) * vec4<f32>(_e4185.g0_.x, _e4188.g5_.z, _e4191.g5_.y, _e4194.g0_.x)) * vec4<f32>(1.0, 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e4207.g9_.y) * vec4<f32>(_e4211.g5_.z, _e4214.g0_.x, _e4217.g5_.x, _e4220.g5_.z)) * vec4<f32>(-(1.0), 1.0, 1.0, 0.0))) + ((vec4<f32>(_e4233.g9_.z) * vec4<f32>(_e4237.g5_.y, _e4240.g5_.x, _e4243.g0_.x, _e4246.g5_.y)) * vec4<f32>(1.0, -(1.0), 1.0, 0.0))) + (vec4<f32>(_e4259.g9_.w) * vec4<f32>(_e4263.g4_.x, _e4266.g4_.y, _e4269.g4_.z, _e4272.g0_.x))) + ((vec4<f32>(_e4278.g0_.y, _e4281.g0_.y, _e4284.g0_.y, _e4287.g0_.x) * _e4291.g3_.xyzx) * vec4<f32>(-(1.0), -(1.0), -(1.0), 0.0))));
}

fn multi_vector_multi_vector_regressive_product(self_1224: MultiVector, other_1018: MultiVector) -> MultiVector {
    var self_1225: MultiVector;
    var other_1019: MultiVector;

    self_1225 = self_1224;
    other_1019 = other_1018;
    let _e4: MultiVector = self_1225;
    let _e8: MultiVector = other_1019;
    let _e11: MultiVector = other_1019;
    let _e14: MultiVector = other_1019;
    let _e24: MultiVector = self_1225;
    let _e28: MultiVector = other_1019;
    let _e32: MultiVector = self_1225;
    let _e36: MultiVector = other_1019;
    let _e47: MultiVector = self_1225;
    let _e51: MultiVector = other_1019;
    let _e62: MultiVector = self_1225;
    let _e66: MultiVector = other_1019;
    let _e77: MultiVector = self_1225;
    let _e81: MultiVector = other_1019;
    let _e92: MultiVector = self_1225;
    let _e96: MultiVector = other_1019;
    let _e107: MultiVector = self_1225;
    let _e111: MultiVector = other_1019;
    let _e123: MultiVector = self_1225;
    let _e127: MultiVector = other_1019;
    let _e139: MultiVector = self_1225;
    let _e143: MultiVector = other_1019;
    let _e155: MultiVector = self_1225;
    let _e159: MultiVector = other_1019;
    let _e171: MultiVector = self_1225;
    let _e175: MultiVector = other_1019;
    let _e187: MultiVector = self_1225;
    let _e191: MultiVector = other_1019;
    let _e203: MultiVector = self_1225;
    let _e207: MultiVector = other_1019;
    let _e219: MultiVector = self_1225;
    let _e223: MultiVector = other_1019;
    let _e235: MultiVector = self_1225;
    let _e239: MultiVector = other_1019;
    let _e251: MultiVector = self_1225;
    let _e255: MultiVector = other_1019;
    let _e267: MultiVector = self_1225;
    let _e271: MultiVector = other_1019;
    let _e283: MultiVector = self_1225;
    let _e287: MultiVector = other_1019;
    let _e299: MultiVector = self_1225;
    let _e303: MultiVector = other_1019;
    let _e315: MultiVector = self_1225;
    let _e319: MultiVector = other_1019;
    let _e331: MultiVector = self_1225;
    let _e335: MultiVector = other_1019;
    let _e347: MultiVector = self_1225;
    let _e351: MultiVector = other_1019;
    let _e363: MultiVector = self_1225;
    let _e367: MultiVector = other_1019;
    let _e379: MultiVector = self_1225;
    let _e383: MultiVector = other_1019;
    let _e395: MultiVector = self_1225;
    let _e399: MultiVector = other_1019;
    let _e411: MultiVector = self_1225;
    let _e415: MultiVector = other_1019;
    let _e427: MultiVector = self_1225;
    let _e431: MultiVector = other_1019;
    let _e442: MultiVector = self_1225;
    let _e446: MultiVector = other_1019;
    let _e457: MultiVector = self_1225;
    let _e461: MultiVector = other_1019;
    let _e472: MultiVector = self_1225;
    let _e476: MultiVector = other_1019;
    let _e487: MultiVector = self_1225;
    let _e491: MultiVector = other_1019;
    let _e501: MultiVector = self_1225;
    let _e505: MultiVector = other_1019;
    let _e508: MultiVector = other_1019;
    let _e511: MultiVector = other_1019;
    let _e516: MultiVector = self_1225;
    let _e520: MultiVector = other_1019;
    let _e524: MultiVector = self_1225;
    let _e528: MultiVector = other_1019;
    let _e540: MultiVector = self_1225;
    let _e544: MultiVector = other_1019;
    let _e556: MultiVector = self_1225;
    let _e560: MultiVector = other_1019;
    let _e572: MultiVector = self_1225;
    let _e576: MultiVector = other_1019;
    let _e587: MultiVector = self_1225;
    let _e591: MultiVector = other_1019;
    let _e602: MultiVector = self_1225;
    let _e606: MultiVector = other_1019;
    let _e617: MultiVector = self_1225;
    let _e621: MultiVector = other_1019;
    let _e624: MultiVector = other_1019;
    let _e627: MultiVector = other_1019;
    let _e639: MultiVector = self_1225;
    let _e643: MultiVector = other_1019;
    let _e646: MultiVector = other_1019;
    let _e649: MultiVector = other_1019;
    let _e661: MultiVector = self_1225;
    let _e665: MultiVector = other_1019;
    let _e668: MultiVector = other_1019;
    let _e671: MultiVector = other_1019;
    let _e683: MultiVector = self_1225;
    let _e687: MultiVector = other_1019;
    let _e698: MultiVector = self_1225;
    let _e702: MultiVector = other_1019;
    let _e713: MultiVector = self_1225;
    let _e717: MultiVector = other_1019;
    let _e728: MultiVector = self_1225;
    let _e732: MultiVector = other_1019;
    let _e735: MultiVector = other_1019;
    let _e738: MultiVector = other_1019;
    let _e750: MultiVector = self_1225;
    let _e754: MultiVector = other_1019;
    let _e757: MultiVector = other_1019;
    let _e760: MultiVector = other_1019;
    let _e772: MultiVector = self_1225;
    let _e776: MultiVector = other_1019;
    let _e779: MultiVector = other_1019;
    let _e782: MultiVector = other_1019;
    let _e794: MultiVector = self_1225;
    let _e798: MultiVector = other_1019;
    let _e809: MultiVector = self_1225;
    let _e813: MultiVector = other_1019;
    let _e824: MultiVector = self_1225;
    let _e828: MultiVector = other_1019;
    let _e839: MultiVector = self_1225;
    let _e843: MultiVector = other_1019;
    let _e847: MultiVector = self_1225;
    let _e851: MultiVector = other_1019;
    let _e862: MultiVector = self_1225;
    let _e866: MultiVector = other_1019;
    let _e877: MultiVector = self_1225;
    let _e881: MultiVector = other_1019;
    let _e892: MultiVector = self_1225;
    let _e896: MultiVector = other_1019;
    let _e900: MultiVector = self_1225;
    let _e902: MultiVector = other_1019;
    let _e908: MultiVector = self_1225;
    let _e912: MultiVector = other_1019;
    let _e915: MultiVector = self_1225;
    let _e919: MultiVector = other_1019;
    let _e929: MultiVector = self_1225;
    let _e933: MultiVector = other_1019;
    let _e943: MultiVector = self_1225;
    let _e947: MultiVector = other_1019;
    let _e957: MultiVector = self_1225;
    let _e961: MultiVector = other_1019;
    let _e971: MultiVector = self_1225;
    let _e975: MultiVector = other_1019;
    let _e985: MultiVector = self_1225;
    let _e989: MultiVector = other_1019;
    let _e992: MultiVector = other_1019;
    let _e1003: MultiVector = self_1225;
    let _e1007: MultiVector = other_1019;
    let _e1018: MultiVector = self_1225;
    let _e1022: MultiVector = other_1019;
    let _e1033: MultiVector = self_1225;
    let _e1037: MultiVector = other_1019;
    let _e1048: MultiVector = self_1225;
    let _e1052: MultiVector = other_1019;
    let _e1055: MultiVector = other_1019;
    let _e1061: MultiVector = self_1225;
    let _e1065: MultiVector = other_1019;
    let _e1068: MultiVector = other_1019;
    let _e1074: MultiVector = self_1225;
    let _e1078: MultiVector = other_1019;
    let _e1081: MultiVector = other_1019;
    let _e1087: MultiVector = self_1225;
    let _e1091: MultiVector = other_1019;
    let _e1102: MultiVector = self_1225;
    let _e1106: MultiVector = other_1019;
    let _e1117: MultiVector = self_1225;
    let _e1121: MultiVector = other_1019;
    let _e1132: MultiVector = self_1225;
    let _e1136: MultiVector = other_1019;
    let _e1147: MultiVector = self_1225;
    let _e1151: MultiVector = other_1019;
    let _e1162: MultiVector = self_1225;
    let _e1166: MultiVector = other_1019;
    let _e1177: MultiVector = self_1225;
    let _e1181: MultiVector = other_1019;
    let _e1184: MultiVector = other_1019;
    let _e1195: MultiVector = self_1225;
    let _e1199: MultiVector = other_1019;
    let _e1202: MultiVector = other_1019;
    let _e1213: MultiVector = self_1225;
    let _e1217: MultiVector = other_1019;
    let _e1220: MultiVector = other_1019;
    let _e1231: MultiVector = self_1225;
    let _e1235: MultiVector = other_1019;
    let _e1246: MultiVector = self_1225;
    let _e1249: MultiVector = self_1225;
    let _e1253: MultiVector = other_1019;
    let _e1256: MultiVector = other_1019;
    let _e1266: MultiVector = self_1225;
    let _e1270: MultiVector = other_1019;
    let _e1273: MultiVector = self_1225;
    let _e1277: MultiVector = other_1019;
    let _e1289: MultiVector = self_1225;
    let _e1293: MultiVector = other_1019;
    let _e1305: MultiVector = self_1225;
    let _e1309: MultiVector = other_1019;
    let _e1321: MultiVector = self_1225;
    let _e1325: MultiVector = other_1019;
    let _e1337: MultiVector = self_1225;
    let _e1341: MultiVector = other_1019;
    let _e1353: MultiVector = self_1225;
    let _e1357: MultiVector = other_1019;
    let _e1369: MultiVector = self_1225;
    let _e1373: MultiVector = other_1019;
    let _e1376: MultiVector = other_1019;
    let _e1379: MultiVector = other_1019;
    let _e1382: MultiVector = other_1019;
    let _e1396: MultiVector = self_1225;
    let _e1400: MultiVector = other_1019;
    let _e1403: MultiVector = other_1019;
    let _e1406: MultiVector = other_1019;
    let _e1409: MultiVector = other_1019;
    let _e1423: MultiVector = self_1225;
    let _e1427: MultiVector = other_1019;
    let _e1430: MultiVector = other_1019;
    let _e1433: MultiVector = other_1019;
    let _e1436: MultiVector = other_1019;
    let _e1450: MultiVector = self_1225;
    let _e1454: MultiVector = other_1019;
    let _e1457: MultiVector = other_1019;
    let _e1460: MultiVector = other_1019;
    let _e1463: MultiVector = other_1019;
    let _e1475: MultiVector = self_1225;
    let _e1477: MultiVector = other_1019;
    let _e1483: MultiVector = self_1225;
    let _e1487: MultiVector = other_1019;
    let _e1490: MultiVector = self_1225;
    let _e1494: MultiVector = other_1019;
    let _e1498: MultiVector = self_1225;
    let _e1502: MultiVector = other_1019;
    let _e1513: MultiVector = self_1225;
    let _e1517: MultiVector = other_1019;
    let _e1528: MultiVector = self_1225;
    let _e1532: MultiVector = other_1019;
    let _e1543: MultiVector = self_1225;
    let _e1547: MultiVector = other_1019;
    let _e1550: MultiVector = other_1019;
    let _e1553: MultiVector = other_1019;
    let _e1565: MultiVector = self_1225;
    let _e1569: MultiVector = other_1019;
    let _e1572: MultiVector = other_1019;
    let _e1575: MultiVector = other_1019;
    let _e1587: MultiVector = self_1225;
    let _e1591: MultiVector = other_1019;
    let _e1594: MultiVector = other_1019;
    let _e1597: MultiVector = other_1019;
    let _e1609: MultiVector = self_1225;
    let _e1613: MultiVector = other_1019;
    let _e1616: MultiVector = other_1019;
    let _e1619: MultiVector = other_1019;
    let _e1631: MultiVector = self_1225;
    let _e1635: MultiVector = other_1019;
    let _e1638: MultiVector = other_1019;
    let _e1641: MultiVector = other_1019;
    let _e1653: MultiVector = self_1225;
    let _e1657: MultiVector = other_1019;
    let _e1660: MultiVector = other_1019;
    let _e1663: MultiVector = other_1019;
    let _e1675: MultiVector = self_1225;
    let _e1677: MultiVector = other_1019;
    let _e1683: MultiVector = self_1225;
    let _e1687: MultiVector = other_1019;
    let _e1690: MultiVector = self_1225;
    let _e1694: MultiVector = other_1019;
    let _e1698: MultiVector = self_1225;
    let _e1702: MultiVector = other_1019;
    let _e1713: MultiVector = self_1225;
    let _e1717: MultiVector = other_1019;
    let _e1728: MultiVector = self_1225;
    let _e1732: MultiVector = other_1019;
    let _e1743: MultiVector = self_1225;
    let _e1747: MultiVector = other_1019;
    let _e1758: MultiVector = self_1225;
    let _e1762: MultiVector = other_1019;
    let _e1773: MultiVector = self_1225;
    let _e1777: MultiVector = other_1019;
    let _e1788: MultiVector = self_1225;
    let _e1792: MultiVector = other_1019;
    let _e1795: MultiVector = other_1019;
    let _e1798: MultiVector = other_1019;
    let _e1804: MultiVector = self_1225;
    let _e1808: MultiVector = other_1019;
    let _e1820: MultiVector = self_1225;
    let _e1824: MultiVector = other_1019;
    let _e1836: MultiVector = self_1225;
    let _e1840: MultiVector = other_1019;
    let _e1852: MultiVector = self_1225;
    let _e1856: MultiVector = other_1019;
    let _e1859: MultiVector = other_1019;
    let _e1862: MultiVector = other_1019;
    let _e1868: MultiVector = self_1225;
    let _e1870: MultiVector = other_1019;
    let _e1876: MultiVector = self_1225;
    let _e1880: MultiVector = other_1019;
    let _e1883: MultiVector = self_1225;
    let _e1887: MultiVector = other_1019;
    let _e1890: MultiVector = other_1019;
    let _e1893: MultiVector = other_1019;
    let _e1905: MultiVector = self_1225;
    let _e1909: MultiVector = other_1019;
    let _e1912: MultiVector = other_1019;
    let _e1915: MultiVector = other_1019;
    let _e1927: MultiVector = self_1225;
    let _e1931: MultiVector = other_1019;
    let _e1934: MultiVector = other_1019;
    let _e1937: MultiVector = other_1019;
    let _e1949: MultiVector = self_1225;
    let _e1951: MultiVector = other_1019;
    let _e1957: MultiVector = self_1225;
    let _e1961: MultiVector = other_1019;
    let _e1964: MultiVector = self_1225;
    let _e1968: MultiVector = other_1019;
    let _e1979: MultiVector = self_1225;
    let _e1983: MultiVector = other_1019;
    let _e1994: MultiVector = self_1225;
    let _e1998: MultiVector = other_1019;
    let _e2009: MultiVector = self_1225;
    let _e2013: MultiVector = other_1019;
    let _e2016: MultiVector = other_1019;
    let _e2019: MultiVector = other_1019;
    let _e2025: MultiVector = self_1225;
    let _e2027: MultiVector = other_1019;
    let _e2033: MultiVector = self_1225;
    let _e2037: MultiVector = other_1019;
    let _e2040: MultiVector = self_1225;
    let _e2044: MultiVector = other_1019;
    let _e2048: MultiVector = self_1225;
    let _e2052: MultiVector = other_1019;
    let _e2065: MultiVector = self_1225;
    let _e2069: MultiVector = other_1019;
    let _e2082: MultiVector = self_1225;
    let _e2086: MultiVector = other_1019;
    let _e2099: MultiVector = self_1225;
    let _e2103: MultiVector = other_1019;
    let _e2116: MultiVector = self_1225;
    let _e2118: MultiVector = other_1019;
    let _e2124: MultiVector = self_1225;
    let _e2128: MultiVector = other_1019;
    let _e2131: MultiVector = self_1225;
    let _e2133: MultiVector = other_1019;
    return MultiVector((((((((((((((((((((((((((((((((((vec3<f32>(_e4.g0_.y) * vec3<f32>(_e8.g2_.y, _e11.g0_.z, _e14.g2_.y)) * vec3<f32>(1.0, 1.0, 0.0)) + (vec3<f32>(_e24.g0_.z) * _e28.g0_)) + ((vec3<f32>(_e32.g1_.x) * vec3<f32>(_e36.g9_.x)) * vec3<f32>(1.0, 0.0, 0.0))) + ((vec3<f32>(_e47.g1_.y) * vec3<f32>(_e51.g9_.y)) * vec3<f32>(1.0, 0.0, 0.0))) + ((vec3<f32>(_e62.g1_.z) * vec3<f32>(_e66.g9_.z)) * vec3<f32>(1.0, 0.0, 0.0))) + ((vec3<f32>(_e77.g2_.x) * vec3<f32>(_e81.g9_.w)) * vec3<f32>(1.0, 0.0, 0.0))) + ((vec3<f32>(_e92.g2_.y) * vec3<f32>(_e96.g0_.y)) * vec3<f32>(1.0, 0.0, 0.0))) + ((vec3<f32>(_e107.g3_.x) * vec3<f32>(_e111.g8_.x)) * vec3<f32>(-(1.0), 0.0, 0.0))) + ((vec3<f32>(_e123.g3_.y) * vec3<f32>(_e127.g8_.y)) * vec3<f32>(-(1.0), 0.0, 0.0))) + ((vec3<f32>(_e139.g3_.z) * vec3<f32>(_e143.g8_.z)) * vec3<f32>(-(1.0), 0.0, 0.0))) + ((vec3<f32>(_e155.g3_.w) * vec3<f32>(_e159.g8_.w)) * vec3<f32>(-(1.0), 0.0, 0.0))) + ((vec3<f32>(_e171.g4_.x) * vec3<f32>(_e175.g7_.x)) * vec3<f32>(-(1.0), 0.0, 0.0))) + ((vec3<f32>(_e187.g4_.y) * vec3<f32>(_e191.g7_.y)) * vec3<f32>(-(1.0), 0.0, 0.0))) + ((vec3<f32>(_e203.g4_.z) * vec3<f32>(_e207.g7_.z)) * vec3<f32>(-(1.0), 0.0, 0.0))) + ((vec3<f32>(_e219.g5_.x) * vec3<f32>(_e223.g6_.x)) * vec3<f32>(-(1.0), 0.0, 0.0))) + ((vec3<f32>(_e235.g5_.y) * vec3<f32>(_e239.g6_.y)) * vec3<f32>(-(1.0), 0.0, 0.0))) + ((vec3<f32>(_e251.g5_.z) * vec3<f32>(_e255.g6_.z)) * vec3<f32>(-(1.0), 0.0, 0.0))) + ((vec3<f32>(_e267.g6_.x) * vec3<f32>(_e271.g5_.x)) * vec3<f32>(-(1.0), 0.0, 0.0))) + ((vec3<f32>(_e283.g6_.y) * vec3<f32>(_e287.g5_.y)) * vec3<f32>(-(1.0), 0.0, 0.0))) + ((vec3<f32>(_e299.g6_.z) * vec3<f32>(_e303.g5_.z)) * vec3<f32>(-(1.0), 0.0, 0.0))) + ((vec3<f32>(_e315.g7_.x) * vec3<f32>(_e319.g4_.x)) * vec3<f32>(-(1.0), 0.0, 0.0))) + ((vec3<f32>(_e331.g7_.y) * vec3<f32>(_e335.g4_.y)) * vec3<f32>(-(1.0), 0.0, 0.0))) + ((vec3<f32>(_e347.g7_.z) * vec3<f32>(_e351.g4_.z)) * vec3<f32>(-(1.0), 0.0, 0.0))) + ((vec3<f32>(_e363.g8_.x) * vec3<f32>(_e367.g3_.x)) * vec3<f32>(-(1.0), 0.0, 0.0))) + ((vec3<f32>(_e379.g8_.y) * vec3<f32>(_e383.g3_.y)) * vec3<f32>(-(1.0), 0.0, 0.0))) + ((vec3<f32>(_e395.g8_.z) * vec3<f32>(_e399.g3_.z)) * vec3<f32>(-(1.0), 0.0, 0.0))) + ((vec3<f32>(_e411.g8_.w) * vec3<f32>(_e415.g3_.w)) * vec3<f32>(-(1.0), 0.0, 0.0))) + ((vec3<f32>(_e427.g9_.x) * vec3<f32>(_e431.g1_.x)) * vec3<f32>(1.0, 0.0, 0.0))) + ((vec3<f32>(_e442.g9_.y) * vec3<f32>(_e446.g1_.y)) * vec3<f32>(1.0, 0.0, 0.0))) + ((vec3<f32>(_e457.g9_.z) * vec3<f32>(_e461.g1_.z)) * vec3<f32>(1.0, 0.0, 0.0))) + ((vec3<f32>(_e472.g9_.w) * vec3<f32>(_e476.g2_.x)) * vec3<f32>(1.0, 0.0, 0.0))) + ((vec3<f32>(_e487.g0_.x) * _e491.g0_.zxx) * vec3<f32>(1.0, 0.0, 0.0))), ((((((((((((((((((((((((((vec3<f32>(_e501.g0_.y) * vec3<f32>(_e505.g3_.x, _e508.g3_.y, _e511.g3_.z)) + (vec3<f32>(_e516.g0_.z) * _e520.g1_)) + ((vec3<f32>(_e524.g3_.x) * vec3<f32>(_e528.g0_.y)) * vec3<f32>(-(1.0), 0.0, 0.0))) + ((vec3<f32>(_e540.g3_.y) * vec3<f32>(_e544.g0_.y)) * vec3<f32>(0.0, -(1.0), 0.0))) + ((vec3<f32>(_e556.g3_.z) * vec3<f32>(_e560.g0_.y)) * vec3<f32>(0.0, 0.0, -(1.0)))) + ((vec3<f32>(_e572.g4_.x) * vec3<f32>(_e576.g9_.w)) * vec3<f32>(1.0, 0.0, 0.0))) + ((vec3<f32>(_e587.g4_.y) * vec3<f32>(_e591.g9_.w)) * vec3<f32>(0.0, 1.0, 0.0))) + ((vec3<f32>(_e602.g4_.z) * vec3<f32>(_e606.g9_.w)) * vec3<f32>(0.0, 0.0, 1.0))) + ((vec3<f32>(_e617.g5_.x) * vec3<f32>(_e621.g9_.z, _e624.g9_.z, _e627.g9_.y)) * vec3<f32>(0.0, -(1.0), 1.0))) + ((vec3<f32>(_e639.g5_.y) * vec3<f32>(_e643.g9_.z, _e646.g9_.z, _e649.g9_.x)) * vec3<f32>(1.0, 0.0, -(1.0)))) + ((vec3<f32>(_e661.g5_.z) * vec3<f32>(_e665.g9_.y, _e668.g9_.x, _e671.g9_.y)) * vec3<f32>(-(1.0), 1.0, 0.0))) + ((vec3<f32>(_e683.g6_.x) * vec3<f32>(_e687.g8_.w)) * vec3<f32>(1.0, 0.0, 0.0))) + ((vec3<f32>(_e698.g6_.y) * vec3<f32>(_e702.g8_.w)) * vec3<f32>(0.0, 1.0, 0.0))) + ((vec3<f32>(_e713.g6_.z) * vec3<f32>(_e717.g8_.w)) * vec3<f32>(0.0, 0.0, 1.0))) + ((vec3<f32>(_e728.g7_.x) * vec3<f32>(_e732.g8_.z, _e735.g8_.z, _e738.g8_.y)) * vec3<f32>(0.0, -(1.0), 1.0))) + ((vec3<f32>(_e750.g7_.y) * vec3<f32>(_e754.g8_.z, _e757.g8_.z, _e760.g8_.x)) * vec3<f32>(1.0, 0.0, -(1.0)))) + ((vec3<f32>(_e772.g7_.z) * vec3<f32>(_e776.g8_.y, _e779.g8_.x, _e782.g8_.y)) * vec3<f32>(-(1.0), 1.0, 0.0))) + ((vec3<f32>(_e794.g8_.x) * _e798.g7_.zzy) * vec3<f32>(0.0, 1.0, -(1.0)))) + ((vec3<f32>(_e809.g8_.y) * _e813.g7_.zzx) * vec3<f32>(-(1.0), 0.0, 1.0))) + ((vec3<f32>(_e824.g8_.z) * _e828.g7_.yxy) * vec3<f32>(1.0, -(1.0), 0.0))) + (vec3<f32>(_e839.g8_.w) * _e843.g6_)) + ((vec3<f32>(_e847.g9_.x) * _e851.g5_.zzy) * vec3<f32>(0.0, -(1.0), 1.0))) + ((vec3<f32>(_e862.g9_.y) * _e866.g5_.zzx) * vec3<f32>(1.0, 0.0, -(1.0)))) + ((vec3<f32>(_e877.g9_.z) * _e881.g5_.yxy) * vec3<f32>(-(1.0), 1.0, 0.0))) - (vec3<f32>(_e892.g9_.w) * _e896.g4_)) + (_e900.g1_ * vec3<f32>(_e902.g0_.z))), ((((((((((((((((((((((((vec2<f32>(_e908.g0_.z) * _e912.g2_) + ((vec2<f32>(_e915.g2_.x) * vec2<f32>(_e919.g0_.z)) * vec2<f32>(1.0, 0.0))) + ((vec2<f32>(_e929.g2_.y) * vec2<f32>(_e933.g0_.z)) * vec2<f32>(0.0, 1.0))) + ((vec2<f32>(_e943.g3_.x) * vec2<f32>(_e947.g9_.x)) * vec2<f32>(0.0, 1.0))) + ((vec2<f32>(_e957.g3_.y) * vec2<f32>(_e961.g9_.y)) * vec2<f32>(0.0, 1.0))) + ((vec2<f32>(_e971.g3_.z) * vec2<f32>(_e975.g9_.z)) * vec2<f32>(0.0, 1.0))) + ((vec2<f32>(_e985.g3_.w) * vec2<f32>(_e989.g0_.y, _e992.g9_.w)) * vec2<f32>(-(1.0), 1.0))) + ((vec2<f32>(_e1003.g4_.x) * vec2<f32>(_e1007.g9_.x)) * vec2<f32>(-(1.0), 0.0))) + ((vec2<f32>(_e1018.g4_.y) * vec2<f32>(_e1022.g9_.y)) * vec2<f32>(-(1.0), 0.0))) + ((vec2<f32>(_e1033.g4_.z) * vec2<f32>(_e1037.g9_.z)) * vec2<f32>(-(1.0), 0.0))) - (vec2<f32>(_e1048.g6_.x) * vec2<f32>(_e1052.g8_.x, _e1055.g7_.x))) - (vec2<f32>(_e1061.g6_.y) * vec2<f32>(_e1065.g8_.y, _e1068.g7_.y))) - (vec2<f32>(_e1074.g6_.z) * vec2<f32>(_e1078.g8_.z, _e1081.g7_.z))) + ((vec2<f32>(_e1087.g7_.x) * vec2<f32>(_e1091.g6_.x)) * vec2<f32>(0.0, -(1.0)))) + ((vec2<f32>(_e1102.g7_.y) * vec2<f32>(_e1106.g6_.y)) * vec2<f32>(0.0, -(1.0)))) + ((vec2<f32>(_e1117.g7_.z) * vec2<f32>(_e1121.g6_.z)) * vec2<f32>(0.0, -(1.0)))) + ((vec2<f32>(_e1132.g8_.x) * vec2<f32>(_e1136.g6_.x)) * vec2<f32>(-(1.0), 0.0))) + ((vec2<f32>(_e1147.g8_.y) * vec2<f32>(_e1151.g6_.y)) * vec2<f32>(-(1.0), 0.0))) + ((vec2<f32>(_e1162.g8_.z) * vec2<f32>(_e1166.g6_.z)) * vec2<f32>(-(1.0), 0.0))) + ((vec2<f32>(_e1177.g9_.x) * vec2<f32>(_e1181.g4_.x, _e1184.g3_.x)) * vec2<f32>(1.0, -(1.0)))) + ((vec2<f32>(_e1195.g9_.y) * vec2<f32>(_e1199.g4_.y, _e1202.g3_.y)) * vec2<f32>(1.0, -(1.0)))) + ((vec2<f32>(_e1213.g9_.z) * vec2<f32>(_e1217.g4_.z, _e1220.g3_.z)) * vec2<f32>(1.0, -(1.0)))) + ((vec2<f32>(_e1231.g9_.w) * vec2<f32>(_e1235.g3_.w)) * vec2<f32>(0.0, -(1.0)))) + ((vec2<f32>(_e1246.g0_.y, _e1249.g0_.x) * vec2<f32>(_e1253.g3_.w, _e1256.g3_.x)) * vec2<f32>(1.0, 0.0))), ((((((((((((vec4<f32>(_e1266.g0_.z) * _e1270.g3_) + ((vec4<f32>(_e1273.g6_.x) * _e1277.g9_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e1289.g6_.y) * _e1293.g9_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0)))) + ((vec4<f32>(_e1305.g6_.z) * _e1309.g9_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e1321.g7_.x) * _e1325.g9_.zzyz) * vec4<f32>(0.0, -(1.0), 1.0, 0.0))) + ((vec4<f32>(_e1337.g7_.y) * _e1341.g9_.zzxz) * vec4<f32>(1.0, 0.0, -(1.0), 0.0))) + ((vec4<f32>(_e1353.g7_.z) * _e1357.g9_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))) + ((vec4<f32>(_e1369.g9_.x) * vec4<f32>(_e1373.g7_.z, _e1376.g7_.z, _e1379.g7_.y, _e1382.g6_.x)) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e1396.g9_.y) * vec4<f32>(_e1400.g7_.z, _e1403.g7_.z, _e1406.g7_.x, _e1409.g6_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e1423.g9_.z) * vec4<f32>(_e1427.g7_.y, _e1430.g7_.x, _e1433.g7_.y, _e1436.g6_.z)) * vec4<f32>(1.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e1450.g9_.w) * vec4<f32>(_e1454.g6_.x, _e1457.g6_.y, _e1460.g6_.z, _e1463.g6_.x)) * vec4<f32>(1.0, 1.0, 1.0, 0.0))) + (_e1475.g3_ * vec4<f32>(_e1477.g0_.z))), ((((((((((((vec3<f32>(_e1483.g0_.y) * _e1487.g6_) + (vec3<f32>(_e1490.g0_.z) * _e1494.g4_)) + ((vec3<f32>(_e1498.g6_.x) * vec3<f32>(_e1502.g0_.y)) * vec3<f32>(1.0, 0.0, 0.0))) + ((vec3<f32>(_e1513.g6_.y) * vec3<f32>(_e1517.g0_.y)) * vec3<f32>(0.0, 1.0, 0.0))) + ((vec3<f32>(_e1528.g6_.z) * vec3<f32>(_e1532.g0_.y)) * vec3<f32>(0.0, 0.0, 1.0))) + ((vec3<f32>(_e1543.g8_.x) * vec3<f32>(_e1547.g9_.z, _e1550.g9_.z, _e1553.g9_.y)) * vec3<f32>(0.0, 1.0, -(1.0)))) + ((vec3<f32>(_e1565.g8_.y) * vec3<f32>(_e1569.g9_.z, _e1572.g9_.z, _e1575.g9_.x)) * vec3<f32>(-(1.0), 0.0, 1.0))) + ((vec3<f32>(_e1587.g8_.z) * vec3<f32>(_e1591.g9_.y, _e1594.g9_.x, _e1597.g9_.y)) * vec3<f32>(1.0, -(1.0), 0.0))) + ((vec3<f32>(_e1609.g9_.x) * vec3<f32>(_e1613.g8_.z, _e1616.g8_.z, _e1619.g8_.y)) * vec3<f32>(0.0, -(1.0), 1.0))) + ((vec3<f32>(_e1631.g9_.y) * vec3<f32>(_e1635.g8_.z, _e1638.g8_.z, _e1641.g8_.x)) * vec3<f32>(1.0, 0.0, -(1.0)))) + ((vec3<f32>(_e1653.g9_.z) * vec3<f32>(_e1657.g8_.y, _e1660.g8_.x, _e1663.g8_.y)) * vec3<f32>(-(1.0), 1.0, 0.0))) + (_e1675.g4_ * vec3<f32>(_e1677.g0_.z))), ((((((((((((((vec3<f32>(_e1683.g0_.y) * _e1687.g7_) + (vec3<f32>(_e1690.g0_.z) * _e1694.g5_)) + ((vec3<f32>(_e1698.g7_.x) * vec3<f32>(_e1702.g0_.y)) * vec3<f32>(1.0, 0.0, 0.0))) + ((vec3<f32>(_e1713.g7_.y) * vec3<f32>(_e1717.g0_.y)) * vec3<f32>(0.0, 1.0, 0.0))) + ((vec3<f32>(_e1728.g7_.z) * vec3<f32>(_e1732.g0_.y)) * vec3<f32>(0.0, 0.0, 1.0))) + ((vec3<f32>(_e1743.g8_.x) * vec3<f32>(_e1747.g9_.w)) * vec3<f32>(1.0, 0.0, 0.0))) + ((vec3<f32>(_e1758.g8_.y) * vec3<f32>(_e1762.g9_.w)) * vec3<f32>(0.0, 1.0, 0.0))) + ((vec3<f32>(_e1773.g8_.z) * vec3<f32>(_e1777.g9_.w)) * vec3<f32>(0.0, 0.0, 1.0))) - (vec3<f32>(_e1788.g8_.w) * vec3<f32>(_e1792.g9_.x, _e1795.g9_.y, _e1798.g9_.z))) + ((vec3<f32>(_e1804.g9_.x) * vec3<f32>(_e1808.g8_.w)) * vec3<f32>(-(1.0), 0.0, 0.0))) + ((vec3<f32>(_e1820.g9_.y) * vec3<f32>(_e1824.g8_.w)) * vec3<f32>(0.0, -(1.0), 0.0))) + ((vec3<f32>(_e1836.g9_.z) * vec3<f32>(_e1840.g8_.w)) * vec3<f32>(0.0, 0.0, -(1.0)))) + (vec3<f32>(_e1852.g9_.w) * vec3<f32>(_e1856.g8_.x, _e1859.g8_.y, _e1862.g8_.z))) + (_e1868.g5_ * vec3<f32>(_e1870.g0_.z))), (((((vec3<f32>(_e1876.g0_.z) * _e1880.g6_) + ((vec3<f32>(_e1883.g9_.x) * vec3<f32>(_e1887.g9_.z, _e1890.g9_.z, _e1893.g9_.y)) * vec3<f32>(0.0, 1.0, -(1.0)))) + ((vec3<f32>(_e1905.g9_.y) * vec3<f32>(_e1909.g9_.z, _e1912.g9_.z, _e1915.g9_.x)) * vec3<f32>(-(1.0), 0.0, 1.0))) + ((vec3<f32>(_e1927.g9_.z) * vec3<f32>(_e1931.g9_.y, _e1934.g9_.x, _e1937.g9_.y)) * vec3<f32>(1.0, -(1.0), 0.0))) + (_e1949.g6_ * vec3<f32>(_e1951.g0_.z))), ((((((vec3<f32>(_e1957.g0_.z) * _e1961.g7_) + ((vec3<f32>(_e1964.g9_.x) * vec3<f32>(_e1968.g9_.w)) * vec3<f32>(1.0, 0.0, 0.0))) + ((vec3<f32>(_e1979.g9_.y) * vec3<f32>(_e1983.g9_.w)) * vec3<f32>(0.0, 1.0, 0.0))) + ((vec3<f32>(_e1994.g9_.z) * vec3<f32>(_e1998.g9_.w)) * vec3<f32>(0.0, 0.0, 1.0))) - (vec3<f32>(_e2009.g9_.w) * vec3<f32>(_e2013.g9_.x, _e2016.g9_.y, _e2019.g9_.z))) + (_e2025.g7_ * vec3<f32>(_e2027.g0_.z))), (((((((vec4<f32>(_e2033.g0_.y) * _e2037.g9_) + (vec4<f32>(_e2040.g0_.z) * _e2044.g8_)) + ((vec4<f32>(_e2048.g9_.x) * vec4<f32>(_e2052.g0_.y)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((vec4<f32>(_e2065.g9_.y) * vec4<f32>(_e2069.g0_.y)) * vec4<f32>(0.0, -(1.0), 0.0, 0.0))) + ((vec4<f32>(_e2082.g9_.z) * vec4<f32>(_e2086.g0_.y)) * vec4<f32>(0.0, 0.0, -(1.0), 0.0))) + ((vec4<f32>(_e2099.g9_.w) * vec4<f32>(_e2103.g0_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + (_e2116.g8_ * vec4<f32>(_e2118.g0_.z))), ((vec4<f32>(_e2124.g0_.z) * _e2128.g9_) + (_e2131.g9_ * vec4<f32>(_e2133.g0_.z))));
}

fn multi_vector_multi_vector_outer_product(self_1226: MultiVector, other_1020: MultiVector) -> MultiVector {
    var self_1227: MultiVector;
    var other_1021: MultiVector;

    self_1227 = self_1226;
    other_1021 = other_1020;
    let _e4: MultiVector = self_1227;
    let _e8: MultiVector = other_1021;
    let _e11: MultiVector = self_1227;
    let _e15: MultiVector = other_1021;
    let _e26: MultiVector = self_1227;
    let _e30: MultiVector = other_1021;
    let _e33: MultiVector = other_1021;
    let _e36: MultiVector = other_1021;
    let _e47: MultiVector = self_1227;
    let _e51: MultiVector = other_1021;
    let _e54: MultiVector = other_1021;
    let _e57: MultiVector = other_1021;
    let _e68: MultiVector = self_1227;
    let _e72: MultiVector = other_1021;
    let _e75: MultiVector = other_1021;
    let _e78: MultiVector = other_1021;
    let _e89: MultiVector = self_1227;
    let _e93: MultiVector = other_1021;
    let _e96: MultiVector = other_1021;
    let _e99: MultiVector = other_1021;
    let _e110: MultiVector = self_1227;
    let _e114: MultiVector = other_1021;
    let _e125: MultiVector = self_1227;
    let _e129: MultiVector = other_1021;
    let _e141: MultiVector = self_1227;
    let _e145: MultiVector = other_1021;
    let _e157: MultiVector = self_1227;
    let _e161: MultiVector = other_1021;
    let _e173: MultiVector = self_1227;
    let _e177: MultiVector = other_1021;
    let _e189: MultiVector = self_1227;
    let _e193: MultiVector = other_1021;
    let _e196: MultiVector = other_1021;
    let _e199: MultiVector = other_1021;
    let _e212: MultiVector = self_1227;
    let _e216: MultiVector = other_1021;
    let _e219: MultiVector = other_1021;
    let _e222: MultiVector = other_1021;
    let _e235: MultiVector = self_1227;
    let _e239: MultiVector = other_1021;
    let _e242: MultiVector = other_1021;
    let _e245: MultiVector = other_1021;
    let _e258: MultiVector = self_1227;
    let _e262: MultiVector = other_1021;
    let _e265: MultiVector = other_1021;
    let _e268: MultiVector = other_1021;
    let _e281: MultiVector = self_1227;
    let _e285: MultiVector = other_1021;
    let _e288: MultiVector = other_1021;
    let _e291: MultiVector = other_1021;
    let _e304: MultiVector = self_1227;
    let _e308: MultiVector = other_1021;
    let _e311: MultiVector = other_1021;
    let _e314: MultiVector = other_1021;
    let _e327: MultiVector = self_1227;
    let _e331: MultiVector = other_1021;
    let _e343: MultiVector = self_1227;
    let _e347: MultiVector = other_1021;
    let _e359: MultiVector = self_1227;
    let _e363: MultiVector = other_1021;
    let _e375: MultiVector = self_1227;
    let _e379: MultiVector = other_1021;
    let _e391: MultiVector = self_1227;
    let _e395: MultiVector = other_1021;
    let _e407: MultiVector = self_1227;
    let _e411: MultiVector = other_1021;
    let _e423: MultiVector = self_1227;
    let _e427: MultiVector = other_1021;
    let _e430: MultiVector = other_1021;
    let _e433: MultiVector = other_1021;
    let _e446: MultiVector = self_1227;
    let _e450: MultiVector = other_1021;
    let _e453: MultiVector = other_1021;
    let _e456: MultiVector = other_1021;
    let _e469: MultiVector = self_1227;
    let _e473: MultiVector = other_1021;
    let _e476: MultiVector = other_1021;
    let _e479: MultiVector = other_1021;
    let _e492: MultiVector = self_1227;
    let _e496: MultiVector = other_1021;
    let _e499: MultiVector = other_1021;
    let _e502: MultiVector = other_1021;
    let _e515: MultiVector = self_1227;
    let _e519: MultiVector = other_1021;
    let _e530: MultiVector = self_1227;
    let _e534: MultiVector = other_1021;
    let _e545: MultiVector = self_1227;
    let _e549: MultiVector = other_1021;
    let _e560: MultiVector = self_1227;
    let _e564: MultiVector = other_1021;
    let _e575: MultiVector = self_1227;
    let _e578: MultiVector = other_1021;
    let _e581: MultiVector = other_1021;
    let _e584: MultiVector = other_1021;
    let _e595: MultiVector = self_1227;
    let _e599: MultiVector = other_1021;
    let _e602: MultiVector = self_1227;
    let _e604: MultiVector = other_1021;
    let _e610: MultiVector = self_1227;
    let _e614: MultiVector = other_1021;
    let _e617: MultiVector = self_1227;
    let _e619: MultiVector = other_1021;
    let _e625: MultiVector = self_1227;
    let _e629: MultiVector = other_1021;
    let _e632: MultiVector = self_1227;
    let _e636: MultiVector = other_1021;
    let _e648: MultiVector = self_1227;
    let _e652: MultiVector = other_1021;
    let _e655: MultiVector = other_1021;
    let _e658: MultiVector = other_1021;
    let _e661: MultiVector = other_1021;
    let _e667: MultiVector = self_1227;
    let _e671: MultiVector = other_1021;
    let _e683: MultiVector = self_1227;
    let _e687: MultiVector = other_1021;
    let _e699: MultiVector = self_1227;
    let _e703: MultiVector = other_1021;
    let _e715: MultiVector = self_1227;
    let _e719: MultiVector = other_1021;
    let _e731: MultiVector = self_1227;
    let _e734: MultiVector = self_1227;
    let _e737: MultiVector = self_1227;
    let _e740: MultiVector = self_1227;
    let _e744: MultiVector = other_1021;
    let _e747: MultiVector = other_1021;
    let _e750: MultiVector = other_1021;
    let _e753: MultiVector = other_1021;
    let _e765: MultiVector = self_1227;
    let _e769: MultiVector = other_1021;
    let _e772: MultiVector = self_1227;
    let _e776: MultiVector = other_1021;
    let _e780: MultiVector = self_1227;
    let _e784: MultiVector = other_1021;
    let _e795: MultiVector = self_1227;
    let _e799: MultiVector = other_1021;
    let _e810: MultiVector = self_1227;
    let _e814: MultiVector = other_1021;
    let _e825: MultiVector = self_1227;
    let _e827: MultiVector = other_1021;
    let _e837: MultiVector = self_1227;
    let _e841: MultiVector = other_1021;
    let _e844: MultiVector = self_1227;
    let _e848: MultiVector = other_1021;
    let _e859: MultiVector = self_1227;
    let _e863: MultiVector = other_1021;
    let _e874: MultiVector = self_1227;
    let _e878: MultiVector = other_1021;
    let _e889: MultiVector = self_1227;
    let _e893: MultiVector = other_1021;
    let _e904: MultiVector = self_1227;
    let _e907: MultiVector = self_1227;
    let _e910: MultiVector = self_1227;
    let _e914: MultiVector = other_1021;
    let _e917: MultiVector = other_1021;
    let _e920: MultiVector = other_1021;
    let _e932: MultiVector = self_1227;
    let _e936: MultiVector = other_1021;
    let _e939: MultiVector = self_1227;
    let _e943: MultiVector = other_1021;
    let _e946: MultiVector = other_1021;
    let _e949: MultiVector = other_1021;
    let _e955: MultiVector = self_1227;
    let _e959: MultiVector = other_1021;
    let _e963: MultiVector = self_1227;
    let _e967: MultiVector = other_1021;
    let _e978: MultiVector = self_1227;
    let _e982: MultiVector = other_1021;
    let _e993: MultiVector = self_1227;
    let _e997: MultiVector = other_1021;
    let _e1008: MultiVector = self_1227;
    let _e1012: MultiVector = other_1021;
    let _e1016: MultiVector = self_1227;
    let _e1020: MultiVector = other_1021;
    let _e1031: MultiVector = self_1227;
    let _e1035: MultiVector = other_1021;
    let _e1046: MultiVector = self_1227;
    let _e1050: MultiVector = other_1021;
    let _e1061: MultiVector = self_1227;
    let _e1065: MultiVector = other_1021;
    let _e1076: MultiVector = self_1227;
    let _e1080: MultiVector = other_1021;
    let _e1091: MultiVector = self_1227;
    let _e1095: MultiVector = other_1021;
    let _e1106: MultiVector = self_1227;
    let _e1108: MultiVector = other_1021;
    let _e1118: MultiVector = self_1227;
    let _e1122: MultiVector = other_1021;
    let _e1125: MultiVector = self_1227;
    let _e1129: MultiVector = other_1021;
    let _e1132: MultiVector = other_1021;
    let _e1135: MultiVector = other_1021;
    let _e1147: MultiVector = self_1227;
    let _e1151: MultiVector = other_1021;
    let _e1154: MultiVector = other_1021;
    let _e1157: MultiVector = other_1021;
    let _e1169: MultiVector = self_1227;
    let _e1173: MultiVector = other_1021;
    let _e1177: MultiVector = self_1227;
    let _e1181: MultiVector = other_1021;
    let _e1192: MultiVector = self_1227;
    let _e1196: MultiVector = other_1021;
    let _e1207: MultiVector = self_1227;
    let _e1211: MultiVector = other_1021;
    let _e1222: MultiVector = self_1227;
    let _e1226: MultiVector = other_1021;
    let _e1237: MultiVector = self_1227;
    let _e1241: MultiVector = other_1021;
    let _e1252: MultiVector = self_1227;
    let _e1256: MultiVector = other_1021;
    let _e1267: MultiVector = self_1227;
    let _e1271: MultiVector = other_1021;
    let _e1282: MultiVector = self_1227;
    let _e1286: MultiVector = other_1021;
    let _e1297: MultiVector = self_1227;
    let _e1301: MultiVector = other_1021;
    let _e1312: MultiVector = self_1227;
    let _e1316: MultiVector = other_1021;
    let _e1319: MultiVector = other_1021;
    let _e1322: MultiVector = other_1021;
    let _e1334: MultiVector = self_1227;
    let _e1338: MultiVector = other_1021;
    let _e1341: MultiVector = self_1227;
    let _e1345: MultiVector = other_1021;
    let _e1348: MultiVector = other_1021;
    let _e1351: MultiVector = other_1021;
    let _e1354: MultiVector = other_1021;
    let _e1368: MultiVector = self_1227;
    let _e1372: MultiVector = other_1021;
    let _e1375: MultiVector = other_1021;
    let _e1378: MultiVector = other_1021;
    let _e1381: MultiVector = other_1021;
    let _e1395: MultiVector = self_1227;
    let _e1399: MultiVector = other_1021;
    let _e1402: MultiVector = other_1021;
    let _e1405: MultiVector = other_1021;
    let _e1408: MultiVector = other_1021;
    let _e1420: MultiVector = self_1227;
    let _e1424: MultiVector = other_1021;
    let _e1427: MultiVector = other_1021;
    let _e1430: MultiVector = other_1021;
    let _e1433: MultiVector = other_1021;
    let _e1446: MultiVector = self_1227;
    let _e1450: MultiVector = other_1021;
    let _e1453: MultiVector = other_1021;
    let _e1456: MultiVector = other_1021;
    let _e1459: MultiVector = other_1021;
    let _e1472: MultiVector = self_1227;
    let _e1476: MultiVector = other_1021;
    let _e1479: MultiVector = other_1021;
    let _e1482: MultiVector = other_1021;
    let _e1485: MultiVector = other_1021;
    let _e1498: MultiVector = self_1227;
    let _e1502: MultiVector = other_1021;
    let _e1505: MultiVector = other_1021;
    let _e1508: MultiVector = other_1021;
    let _e1511: MultiVector = other_1021;
    let _e1524: MultiVector = self_1227;
    let _e1528: MultiVector = other_1021;
    let _e1531: MultiVector = other_1021;
    let _e1534: MultiVector = other_1021;
    let _e1537: MultiVector = other_1021;
    let _e1550: MultiVector = self_1227;
    let _e1554: MultiVector = other_1021;
    let _e1557: MultiVector = other_1021;
    let _e1560: MultiVector = other_1021;
    let _e1563: MultiVector = other_1021;
    let _e1576: MultiVector = self_1227;
    let _e1580: MultiVector = other_1021;
    let _e1592: MultiVector = self_1227;
    let _e1596: MultiVector = other_1021;
    let _e1608: MultiVector = self_1227;
    let _e1612: MultiVector = other_1021;
    let _e1624: MultiVector = self_1227;
    let _e1628: MultiVector = other_1021;
    let _e1640: MultiVector = self_1227;
    let _e1644: MultiVector = other_1021;
    let _e1647: MultiVector = other_1021;
    let _e1650: MultiVector = other_1021;
    let _e1653: MultiVector = other_1021;
    let _e1667: MultiVector = self_1227;
    let _e1671: MultiVector = other_1021;
    let _e1674: MultiVector = self_1227;
    let _e1678: MultiVector = other_1021;
    let _e1681: MultiVector = other_1021;
    let _e1684: MultiVector = other_1021;
    let _e1687: MultiVector = other_1021;
    let _e1701: MultiVector = self_1227;
    let _e1705: MultiVector = other_1021;
    let _e1708: MultiVector = other_1021;
    let _e1711: MultiVector = other_1021;
    let _e1714: MultiVector = other_1021;
    let _e1728: MultiVector = self_1227;
    let _e1732: MultiVector = other_1021;
    let _e1735: MultiVector = other_1021;
    let _e1738: MultiVector = other_1021;
    let _e1741: MultiVector = other_1021;
    let _e1753: MultiVector = self_1227;
    let _e1757: MultiVector = other_1021;
    let _e1761: MultiVector = self_1227;
    let _e1765: MultiVector = other_1021;
    let _e1768: MultiVector = other_1021;
    let _e1771: MultiVector = other_1021;
    let _e1774: MultiVector = other_1021;
    let _e1788: MultiVector = self_1227;
    let _e1792: MultiVector = other_1021;
    let _e1795: MultiVector = other_1021;
    let _e1798: MultiVector = other_1021;
    let _e1801: MultiVector = other_1021;
    let _e1815: MultiVector = self_1227;
    let _e1819: MultiVector = other_1021;
    let _e1822: MultiVector = other_1021;
    let _e1825: MultiVector = other_1021;
    let _e1828: MultiVector = other_1021;
    let _e1842: MultiVector = self_1227;
    let _e1846: MultiVector = other_1021;
    let _e1849: MultiVector = other_1021;
    let _e1852: MultiVector = other_1021;
    let _e1855: MultiVector = other_1021;
    let _e1867: MultiVector = self_1227;
    let _e1871: MultiVector = other_1021;
    let _e1883: MultiVector = self_1227;
    let _e1887: MultiVector = other_1021;
    let _e1899: MultiVector = self_1227;
    let _e1903: MultiVector = other_1021;
    let _e1915: MultiVector = self_1227;
    let _e1919: MultiVector = other_1021;
    let _e1931: MultiVector = self_1227;
    let _e1935: MultiVector = other_1021;
    let _e1947: MultiVector = self_1227;
    let _e1951: MultiVector = other_1021;
    let _e1963: MultiVector = self_1227;
    let _e1967: MultiVector = other_1021;
    let _e1970: MultiVector = other_1021;
    let _e1973: MultiVector = other_1021;
    let _e1976: MultiVector = other_1021;
    let _e1989: MultiVector = self_1227;
    let _e1993: MultiVector = other_1021;
    let _e1996: MultiVector = other_1021;
    let _e1999: MultiVector = other_1021;
    let _e2002: MultiVector = other_1021;
    let _e2015: MultiVector = self_1227;
    let _e2019: MultiVector = other_1021;
    let _e2022: MultiVector = other_1021;
    let _e2025: MultiVector = other_1021;
    let _e2028: MultiVector = other_1021;
    let _e2041: MultiVector = self_1227;
    let _e2045: MultiVector = other_1021;
    let _e2048: MultiVector = other_1021;
    let _e2051: MultiVector = other_1021;
    let _e2054: MultiVector = other_1021;
    let _e2067: MultiVector = self_1227;
    let _e2071: MultiVector = other_1021;
    let _e2074: MultiVector = other_1021;
    let _e2077: MultiVector = other_1021;
    let _e2080: MultiVector = other_1021;
    let _e2093: MultiVector = self_1227;
    let _e2097: MultiVector = other_1021;
    let _e2100: MultiVector = other_1021;
    let _e2103: MultiVector = other_1021;
    let _e2106: MultiVector = other_1021;
    let _e2119: MultiVector = self_1227;
    let _e2123: MultiVector = other_1021;
    let _e2135: MultiVector = self_1227;
    let _e2139: MultiVector = other_1021;
    let _e2151: MultiVector = self_1227;
    let _e2155: MultiVector = other_1021;
    let _e2167: MultiVector = self_1227;
    let _e2171: MultiVector = other_1021;
    let _e2183: MultiVector = self_1227;
    let _e2187: MultiVector = other_1021;
    let _e2199: MultiVector = self_1227;
    let _e2203: MultiVector = other_1021;
    let _e2215: MultiVector = self_1227;
    let _e2219: MultiVector = other_1021;
    let _e2231: MultiVector = self_1227;
    let _e2235: MultiVector = other_1021;
    let _e2247: MultiVector = self_1227;
    let _e2251: MultiVector = other_1021;
    let _e2254: MultiVector = other_1021;
    let _e2257: MultiVector = other_1021;
    let _e2260: MultiVector = other_1021;
    return MultiVector(((((((((((((((((((((((((((((((((vec3<f32>(_e4.g0_.x) * _e8.g0_) + ((vec3<f32>(_e11.g0_.z) * vec3<f32>(_e15.g0_.x)) * vec3<f32>(0.0, 0.0, 1.0))) + ((vec3<f32>(_e26.g1_.x) * vec3<f32>(_e30.g8_.x, _e33.g8_.x, _e36.g9_.x)) * vec3<f32>(0.0, 1.0, 1.0))) + ((vec3<f32>(_e47.g1_.y) * vec3<f32>(_e51.g8_.y, _e54.g8_.y, _e57.g9_.y)) * vec3<f32>(0.0, 1.0, 1.0))) + ((vec3<f32>(_e68.g1_.z) * vec3<f32>(_e72.g8_.z, _e75.g8_.z, _e78.g9_.z)) * vec3<f32>(0.0, 1.0, 1.0))) + ((vec3<f32>(_e89.g2_.x) * vec3<f32>(_e93.g8_.w, _e96.g8_.w, _e99.g9_.w)) * vec3<f32>(0.0, 1.0, 1.0))) + ((vec3<f32>(_e110.g2_.y) * vec3<f32>(_e114.g0_.y)) * vec3<f32>(0.0, 0.0, 1.0))) + ((vec3<f32>(_e125.g3_.x) * vec3<f32>(_e129.g8_.x)) * vec3<f32>(0.0, 0.0, -(1.0)))) + ((vec3<f32>(_e141.g3_.y) * vec3<f32>(_e145.g8_.y)) * vec3<f32>(0.0, 0.0, -(1.0)))) + ((vec3<f32>(_e157.g3_.z) * vec3<f32>(_e161.g8_.z)) * vec3<f32>(0.0, 0.0, -(1.0)))) + ((vec3<f32>(_e173.g3_.w) * vec3<f32>(_e177.g8_.w)) * vec3<f32>(0.0, 0.0, -(1.0)))) + ((vec3<f32>(_e189.g4_.x) * vec3<f32>(_e193.g5_.x, _e196.g5_.x, _e199.g7_.x)) * vec3<f32>(0.0, -(1.0), -(1.0)))) + ((vec3<f32>(_e212.g4_.y) * vec3<f32>(_e216.g5_.y, _e219.g5_.y, _e222.g7_.y)) * vec3<f32>(0.0, -(1.0), -(1.0)))) + ((vec3<f32>(_e235.g4_.z) * vec3<f32>(_e239.g5_.z, _e242.g5_.z, _e245.g7_.z)) * vec3<f32>(0.0, -(1.0), -(1.0)))) + ((vec3<f32>(_e258.g5_.x) * vec3<f32>(_e262.g4_.x, _e265.g4_.x, _e268.g6_.x)) * vec3<f32>(0.0, -(1.0), -(1.0)))) + ((vec3<f32>(_e281.g5_.y) * vec3<f32>(_e285.g4_.y, _e288.g4_.y, _e291.g6_.y)) * vec3<f32>(0.0, -(1.0), -(1.0)))) + ((vec3<f32>(_e304.g5_.z) * vec3<f32>(_e308.g4_.z, _e311.g4_.z, _e314.g6_.z)) * vec3<f32>(0.0, -(1.0), -(1.0)))) + ((vec3<f32>(_e327.g6_.x) * vec3<f32>(_e331.g5_.x)) * vec3<f32>(0.0, 0.0, -(1.0)))) + ((vec3<f32>(_e343.g6_.y) * vec3<f32>(_e347.g5_.y)) * vec3<f32>(0.0, 0.0, -(1.0)))) + ((vec3<f32>(_e359.g6_.z) * vec3<f32>(_e363.g5_.z)) * vec3<f32>(0.0, 0.0, -(1.0)))) + ((vec3<f32>(_e375.g7_.x) * vec3<f32>(_e379.g4_.x)) * vec3<f32>(0.0, 0.0, -(1.0)))) + ((vec3<f32>(_e391.g7_.y) * vec3<f32>(_e395.g4_.y)) * vec3<f32>(0.0, 0.0, -(1.0)))) + ((vec3<f32>(_e407.g7_.z) * vec3<f32>(_e411.g4_.z)) * vec3<f32>(0.0, 0.0, -(1.0)))) + ((vec3<f32>(_e423.g8_.x) * vec3<f32>(_e427.g1_.x, _e430.g1_.x, _e433.g3_.x)) * vec3<f32>(0.0, -(1.0), -(1.0)))) + ((vec3<f32>(_e446.g8_.y) * vec3<f32>(_e450.g1_.y, _e453.g1_.y, _e456.g3_.y)) * vec3<f32>(0.0, -(1.0), -(1.0)))) + ((vec3<f32>(_e469.g8_.z) * vec3<f32>(_e473.g1_.z, _e476.g1_.z, _e479.g3_.z)) * vec3<f32>(0.0, -(1.0), -(1.0)))) + ((vec3<f32>(_e492.g8_.w) * vec3<f32>(_e496.g2_.x, _e499.g2_.x, _e502.g3_.w)) * vec3<f32>(0.0, -(1.0), -(1.0)))) + ((vec3<f32>(_e515.g9_.x) * vec3<f32>(_e519.g1_.x)) * vec3<f32>(0.0, 0.0, 1.0))) + ((vec3<f32>(_e530.g9_.y) * vec3<f32>(_e534.g1_.y)) * vec3<f32>(0.0, 0.0, 1.0))) + ((vec3<f32>(_e545.g9_.z) * vec3<f32>(_e549.g1_.z)) * vec3<f32>(0.0, 0.0, 1.0))) + ((vec3<f32>(_e560.g9_.w) * vec3<f32>(_e564.g2_.x)) * vec3<f32>(0.0, 0.0, 1.0))) + ((_e575.g0_.xyy * vec3<f32>(_e578.g0_.x, _e581.g0_.x, _e584.g2_.y)) * vec3<f32>(0.0, 1.0, 1.0))), ((vec3<f32>(_e595.g0_.x) * _e599.g1_) + (_e602.g1_ * vec3<f32>(_e604.g0_.x))), ((vec2<f32>(_e610.g0_.x) * _e614.g2_) + (_e617.g2_ * vec2<f32>(_e619.g0_.x))), ((((((((vec4<f32>(_e625.g0_.x) * _e629.g3_) + ((vec4<f32>(_e632.g2_.x) * vec4<f32>(_e636.g2_.y)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) - (vec4<f32>(_e648.g2_.y) * vec4<f32>(_e652.g1_.x, _e655.g1_.y, _e658.g1_.z, _e661.g2_.x))) + ((vec4<f32>(_e667.g3_.x) * vec4<f32>(_e671.g0_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e683.g3_.y) * vec4<f32>(_e687.g0_.x)) * vec4<f32>(0.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e699.g3_.z) * vec4<f32>(_e703.g0_.x)) * vec4<f32>(0.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e715.g3_.w) * vec4<f32>(_e719.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e731.g1_.x, _e734.g1_.y, _e737.g1_.z, _e740.g1_.x) * vec4<f32>(_e744.g2_.y, _e747.g2_.y, _e750.g2_.y, _e753.g2_.x)) * vec4<f32>(1.0, 1.0, 1.0, 0.0))), ((((((vec3<f32>(_e765.g0_.x) * _e769.g4_) + (vec3<f32>(_e772.g2_.x) * _e776.g1_)) + ((vec3<f32>(_e780.g4_.x) * vec3<f32>(_e784.g0_.x)) * vec3<f32>(1.0, 0.0, 0.0))) + ((vec3<f32>(_e795.g4_.y) * vec3<f32>(_e799.g0_.x)) * vec3<f32>(0.0, 1.0, 0.0))) + ((vec3<f32>(_e810.g4_.z) * vec3<f32>(_e814.g0_.x)) * vec3<f32>(0.0, 0.0, 1.0))) + ((_e825.g1_ * vec3<f32>(_e827.g2_.x)) * vec3<f32>(-(1.0)))), ((((((vec3<f32>(_e837.g0_.x) * _e841.g5_) + ((vec3<f32>(_e844.g1_.y) * _e848.g1_.zzx) * vec3<f32>(1.0, 0.0, -(1.0)))) + ((vec3<f32>(_e859.g1_.z) * _e863.g1_.yxy) * vec3<f32>(-(1.0), 1.0, 0.0))) + ((vec3<f32>(_e874.g5_.y) * vec3<f32>(_e878.g0_.x)) * vec3<f32>(0.0, 1.0, 0.0))) + ((vec3<f32>(_e889.g5_.z) * vec3<f32>(_e893.g0_.x)) * vec3<f32>(0.0, 0.0, 1.0))) + ((vec3<f32>(_e904.g5_.x, _e907.g1_.x, _e910.g1_.x) * vec3<f32>(_e914.g0_.x, _e917.g1_.z, _e920.g1_.y)) * vec3<f32>(1.0, -(1.0), 1.0))), ((((((((((((((vec3<f32>(_e932.g0_.x) * _e936.g6_) + (vec3<f32>(_e939.g2_.x) * vec3<f32>(_e943.g3_.x, _e946.g3_.y, _e949.g3_.z))) + (vec3<f32>(_e955.g2_.y) * _e959.g4_)) + ((vec3<f32>(_e963.g3_.x) * vec3<f32>(_e967.g2_.x)) * vec3<f32>(1.0, 0.0, 0.0))) + ((vec3<f32>(_e978.g3_.y) * vec3<f32>(_e982.g2_.x)) * vec3<f32>(0.0, 1.0, 0.0))) + ((vec3<f32>(_e993.g3_.z) * vec3<f32>(_e997.g2_.x)) * vec3<f32>(0.0, 0.0, 1.0))) - (vec3<f32>(_e1008.g3_.w) * _e1012.g1_)) + ((vec3<f32>(_e1016.g4_.x) * vec3<f32>(_e1020.g2_.y)) * vec3<f32>(1.0, 0.0, 0.0))) + ((vec3<f32>(_e1031.g4_.y) * vec3<f32>(_e1035.g2_.y)) * vec3<f32>(0.0, 1.0, 0.0))) + ((vec3<f32>(_e1046.g4_.z) * vec3<f32>(_e1050.g2_.y)) * vec3<f32>(0.0, 0.0, 1.0))) + ((vec3<f32>(_e1061.g6_.x) * vec3<f32>(_e1065.g0_.x)) * vec3<f32>(1.0, 0.0, 0.0))) + ((vec3<f32>(_e1076.g6_.y) * vec3<f32>(_e1080.g0_.x)) * vec3<f32>(0.0, 1.0, 0.0))) + ((vec3<f32>(_e1091.g6_.z) * vec3<f32>(_e1095.g0_.x)) * vec3<f32>(0.0, 0.0, 1.0))) + ((_e1106.g1_ * vec3<f32>(_e1108.g3_.w)) * vec3<f32>(-(1.0)))), ((((((((((((((vec3<f32>(_e1118.g0_.x) * _e1122.g7_) + ((vec3<f32>(_e1125.g1_.y) * vec3<f32>(_e1129.g3_.z, _e1132.g3_.z, _e1135.g3_.x)) * vec3<f32>(1.0, 0.0, -(1.0)))) + ((vec3<f32>(_e1147.g1_.z) * vec3<f32>(_e1151.g3_.y, _e1154.g3_.x, _e1157.g3_.y)) * vec3<f32>(-(1.0), 1.0, 0.0))) + (vec3<f32>(_e1169.g2_.y) * _e1173.g5_)) + ((vec3<f32>(_e1177.g3_.x) * _e1181.g1_.zzy) * vec3<f32>(0.0, 1.0, -(1.0)))) + ((vec3<f32>(_e1192.g3_.y) * _e1196.g1_.zzx) * vec3<f32>(-(1.0), 0.0, 1.0))) + ((vec3<f32>(_e1207.g3_.z) * _e1211.g1_.yxy) * vec3<f32>(1.0, -(1.0), 0.0))) + ((vec3<f32>(_e1222.g5_.x) * vec3<f32>(_e1226.g2_.y)) * vec3<f32>(1.0, 0.0, 0.0))) + ((vec3<f32>(_e1237.g5_.y) * vec3<f32>(_e1241.g2_.y)) * vec3<f32>(0.0, 1.0, 0.0))) + ((vec3<f32>(_e1252.g5_.z) * vec3<f32>(_e1256.g2_.y)) * vec3<f32>(0.0, 0.0, 1.0))) + ((vec3<f32>(_e1267.g7_.x) * vec3<f32>(_e1271.g0_.x)) * vec3<f32>(1.0, 0.0, 0.0))) + ((vec3<f32>(_e1282.g7_.y) * vec3<f32>(_e1286.g0_.x)) * vec3<f32>(0.0, 1.0, 0.0))) + ((vec3<f32>(_e1297.g7_.z) * vec3<f32>(_e1301.g0_.x)) * vec3<f32>(0.0, 0.0, 1.0))) + ((vec3<f32>(_e1312.g1_.x) * vec3<f32>(_e1316.g3_.x, _e1319.g3_.z, _e1322.g3_.y)) * vec3<f32>(0.0, -(1.0), 1.0))), (((((((((((((((vec4<f32>(_e1334.g0_.x) * _e1338.g8_) + ((vec4<f32>(_e1341.g1_.y) * vec4<f32>(_e1345.g4_.z, _e1348.g4_.z, _e1351.g4_.x, _e1354.g5_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e1368.g1_.z) * vec4<f32>(_e1372.g4_.y, _e1375.g4_.x, _e1378.g4_.y, _e1381.g5_.z)) * vec4<f32>(1.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e1395.g2_.x) * vec4<f32>(_e1399.g5_.x, _e1402.g5_.y, _e1405.g5_.z, _e1408.g5_.x)) * vec4<f32>(1.0, 1.0, 1.0, 0.0))) + ((vec4<f32>(_e1420.g4_.x) * vec4<f32>(_e1424.g1_.z, _e1427.g1_.z, _e1430.g1_.y, _e1433.g1_.z)) * vec4<f32>(0.0, -(1.0), 1.0, 0.0))) + ((vec4<f32>(_e1446.g4_.y) * vec4<f32>(_e1450.g1_.z, _e1453.g1_.z, _e1456.g1_.x, _e1459.g1_.z)) * vec4<f32>(1.0, 0.0, -(1.0), 0.0))) + ((vec4<f32>(_e1472.g4_.z) * vec4<f32>(_e1476.g1_.y, _e1479.g1_.x, _e1482.g1_.y, _e1485.g1_.y)) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))) + ((vec4<f32>(_e1498.g5_.x) * vec4<f32>(_e1502.g2_.x, _e1505.g2_.x, _e1508.g2_.x, _e1511.g1_.x)) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e1524.g5_.y) * vec4<f32>(_e1528.g2_.x, _e1531.g2_.x, _e1534.g2_.x, _e1537.g1_.y)) * vec4<f32>(0.0, 1.0, 0.0, -(1.0)))) + ((vec4<f32>(_e1550.g5_.z) * vec4<f32>(_e1554.g2_.x, _e1557.g2_.x, _e1560.g2_.x, _e1563.g1_.z)) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e1576.g8_.x) * vec4<f32>(_e1580.g0_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e1592.g8_.y) * vec4<f32>(_e1596.g0_.x)) * vec4<f32>(0.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e1608.g8_.z) * vec4<f32>(_e1612.g0_.x)) * vec4<f32>(0.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e1624.g8_.w) * vec4<f32>(_e1628.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e1640.g1_.x) * vec4<f32>(_e1644.g4_.x, _e1647.g4_.z, _e1650.g4_.y, _e1653.g5_.x)) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))), ((((((((((((((((((((((((((((((vec4<f32>(_e1667.g0_.x) * _e1671.g9_) + ((vec4<f32>(_e1674.g1_.y) * vec4<f32>(_e1678.g6_.z, _e1681.g6_.z, _e1684.g6_.x, _e1687.g7_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e1701.g1_.z) * vec4<f32>(_e1705.g6_.y, _e1708.g6_.x, _e1711.g6_.y, _e1714.g7_.z)) * vec4<f32>(1.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e1728.g2_.x) * vec4<f32>(_e1732.g7_.x, _e1735.g7_.y, _e1738.g7_.z, _e1741.g7_.x)) * vec4<f32>(1.0, 1.0, 1.0, 0.0))) - (vec4<f32>(_e1753.g2_.y) * _e1757.g8_)) + ((vec4<f32>(_e1761.g3_.x) * vec4<f32>(_e1765.g4_.z, _e1768.g4_.z, _e1771.g4_.y, _e1774.g5_.x)) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e1788.g3_.y) * vec4<f32>(_e1792.g4_.z, _e1795.g4_.z, _e1798.g4_.x, _e1801.g5_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e1815.g3_.z) * vec4<f32>(_e1819.g4_.y, _e1822.g4_.x, _e1825.g4_.y, _e1828.g5_.z)) * vec4<f32>(1.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e1842.g3_.w) * vec4<f32>(_e1846.g5_.x, _e1849.g5_.y, _e1852.g5_.z, _e1855.g5_.x)) * vec4<f32>(1.0, 1.0, 1.0, 0.0))) + ((vec4<f32>(_e1867.g4_.x) * _e1871.g3_.zzyz) * vec4<f32>(0.0, -(1.0), 1.0, 0.0))) + ((vec4<f32>(_e1883.g4_.y) * _e1887.g3_.zzxz) * vec4<f32>(1.0, 0.0, -(1.0), 0.0))) + ((vec4<f32>(_e1899.g4_.z) * _e1903.g3_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))) + ((vec4<f32>(_e1915.g5_.x) * _e1919.g3_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e1931.g5_.y) * _e1935.g3_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0)))) + ((vec4<f32>(_e1947.g5_.z) * _e1951.g3_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e1963.g6_.x) * vec4<f32>(_e1967.g1_.z, _e1970.g1_.z, _e1973.g1_.y, _e1976.g1_.z)) * vec4<f32>(0.0, 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e1989.g6_.y) * vec4<f32>(_e1993.g1_.z, _e1996.g1_.z, _e1999.g1_.x, _e2002.g1_.z)) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0))) + ((vec4<f32>(_e2015.g6_.z) * vec4<f32>(_e2019.g1_.y, _e2022.g1_.x, _e2025.g1_.y, _e2028.g1_.y)) * vec4<f32>(1.0, -(1.0), 0.0, 0.0))) + ((vec4<f32>(_e2041.g7_.x) * vec4<f32>(_e2045.g2_.x, _e2048.g2_.x, _e2051.g2_.x, _e2054.g1_.x)) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))) + ((vec4<f32>(_e2067.g7_.y) * vec4<f32>(_e2071.g2_.x, _e2074.g2_.x, _e2077.g2_.x, _e2080.g1_.y)) * vec4<f32>(0.0, -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e2093.g7_.z) * vec4<f32>(_e2097.g2_.x, _e2100.g2_.x, _e2103.g2_.x, _e2106.g1_.z)) * vec4<f32>(0.0, 0.0, -(1.0), 1.0))) + ((vec4<f32>(_e2119.g8_.x) * vec4<f32>(_e2123.g2_.y)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e2135.g8_.y) * vec4<f32>(_e2139.g2_.y)) * vec4<f32>(0.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e2151.g8_.z) * vec4<f32>(_e2155.g2_.y)) * vec4<f32>(0.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e2167.g8_.w) * vec4<f32>(_e2171.g2_.y)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e2183.g9_.x) * vec4<f32>(_e2187.g0_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e2199.g9_.y) * vec4<f32>(_e2203.g0_.x)) * vec4<f32>(0.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e2215.g9_.z) * vec4<f32>(_e2219.g0_.x)) * vec4<f32>(0.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e2231.g9_.w) * vec4<f32>(_e2235.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e2247.g1_.x) * vec4<f32>(_e2251.g6_.x, _e2254.g6_.z, _e2257.g6_.y, _e2260.g7_.x)) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))));
}

fn multi_vector_multi_vector_inner_product(self_1228: MultiVector, other_1022: MultiVector) -> MultiVector {
    var self_1229: MultiVector;
    var other_1023: MultiVector;

    self_1229 = self_1228;
    other_1023 = other_1022;
    let _e4: MultiVector = self_1229;
    let _e8: MultiVector = other_1023;
    let _e11: MultiVector = self_1229;
    let _e15: MultiVector = other_1023;
    let _e26: MultiVector = self_1229;
    let _e30: MultiVector = other_1023;
    let _e41: MultiVector = self_1229;
    let _e45: MultiVector = other_1023;
    let _e57: MultiVector = self_1229;
    let _e61: MultiVector = other_1023;
    let _e73: MultiVector = self_1229;
    let _e77: MultiVector = other_1023;
    let _e89: MultiVector = self_1229;
    let _e93: MultiVector = other_1023;
    let _e105: MultiVector = self_1229;
    let _e108: MultiVector = self_1229;
    let _e111: MultiVector = self_1229;
    let _e115: MultiVector = other_1023;
    let _e118: MultiVector = other_1023;
    let _e121: MultiVector = other_1023;
    let _e127: MultiVector = self_1229;
    let _e131: MultiVector = other_1023;
    let _e134: MultiVector = self_1229;
    let _e138: MultiVector = other_1023;
    let _e141: MultiVector = other_1023;
    let _e144: MultiVector = other_1023;
    let _e156: MultiVector = self_1229;
    let _e160: MultiVector = other_1023;
    let _e163: MultiVector = other_1023;
    let _e166: MultiVector = other_1023;
    let _e178: MultiVector = self_1229;
    let _e182: MultiVector = other_1023;
    let _e185: MultiVector = other_1023;
    let _e188: MultiVector = other_1023;
    let _e200: MultiVector = self_1229;
    let _e204: MultiVector = other_1023;
    let _e207: MultiVector = other_1023;
    let _e210: MultiVector = other_1023;
    let _e222: MultiVector = self_1229;
    let _e226: MultiVector = other_1023;
    let _e229: MultiVector = other_1023;
    let _e232: MultiVector = other_1023;
    let _e244: MultiVector = self_1229;
    let _e248: MultiVector = other_1023;
    let _e251: MultiVector = other_1023;
    let _e254: MultiVector = other_1023;
    let _e266: MultiVector = self_1229;
    let _e270: MultiVector = other_1023;
    let _e274: MultiVector = self_1229;
    let _e278: MultiVector = other_1023;
    let _e281: MultiVector = self_1229;
    let _e285: MultiVector = other_1023;
    let _e288: MultiVector = other_1023;
    let _e299: MultiVector = self_1229;
    let _e303: MultiVector = other_1023;
    let _e306: MultiVector = other_1023;
    let _e317: MultiVector = self_1229;
    let _e321: MultiVector = other_1023;
    let _e324: MultiVector = other_1023;
    let _e335: MultiVector = self_1229;
    let _e339: MultiVector = other_1023;
    let _e349: MultiVector = self_1229;
    let _e353: MultiVector = other_1023;
    let _e363: MultiVector = self_1229;
    let _e367: MultiVector = other_1023;
    let _e378: MultiVector = self_1229;
    let _e382: MultiVector = other_1023;
    let _e393: MultiVector = self_1229;
    let _e397: MultiVector = other_1023;
    let _e408: MultiVector = self_1229;
    let _e412: MultiVector = other_1023;
    let _e422: MultiVector = self_1229;
    let _e426: MultiVector = other_1023;
    let _e436: MultiVector = self_1229;
    let _e440: MultiVector = other_1023;
    let _e450: MultiVector = self_1229;
    let _e454: MultiVector = other_1023;
    let _e457: MultiVector = other_1023;
    let _e463: MultiVector = self_1229;
    let _e467: MultiVector = other_1023;
    let _e470: MultiVector = other_1023;
    let _e476: MultiVector = self_1229;
    let _e480: MultiVector = other_1023;
    let _e483: MultiVector = other_1023;
    let _e489: MultiVector = self_1229;
    let _e493: MultiVector = other_1023;
    let _e504: MultiVector = self_1229;
    let _e508: MultiVector = other_1023;
    let _e519: MultiVector = self_1229;
    let _e523: MultiVector = other_1023;
    let _e534: MultiVector = self_1229;
    let _e538: MultiVector = other_1023;
    let _e549: MultiVector = self_1229;
    let _e553: MultiVector = other_1023;
    let _e564: MultiVector = self_1229;
    let _e568: MultiVector = other_1023;
    let _e579: MultiVector = self_1229;
    let _e583: MultiVector = other_1023;
    let _e586: MultiVector = other_1023;
    let _e597: MultiVector = self_1229;
    let _e601: MultiVector = other_1023;
    let _e611: MultiVector = self_1229;
    let _e614: MultiVector = self_1229;
    let _e618: MultiVector = other_1023;
    let _e621: MultiVector = other_1023;
    let _e632: MultiVector = self_1229;
    let _e636: MultiVector = other_1023;
    let _e639: MultiVector = self_1229;
    let _e643: MultiVector = other_1023;
    let _e646: MultiVector = other_1023;
    let _e649: MultiVector = other_1023;
    let _e652: MultiVector = other_1023;
    let _e666: MultiVector = self_1229;
    let _e670: MultiVector = other_1023;
    let _e673: MultiVector = other_1023;
    let _e676: MultiVector = other_1023;
    let _e679: MultiVector = other_1023;
    let _e693: MultiVector = self_1229;
    let _e697: MultiVector = other_1023;
    let _e700: MultiVector = other_1023;
    let _e703: MultiVector = other_1023;
    let _e706: MultiVector = other_1023;
    let _e720: MultiVector = self_1229;
    let _e724: MultiVector = other_1023;
    let _e736: MultiVector = self_1229;
    let _e740: MultiVector = other_1023;
    let _e752: MultiVector = self_1229;
    let _e756: MultiVector = other_1023;
    let _e768: MultiVector = self_1229;
    let _e772: MultiVector = other_1023;
    let _e784: MultiVector = self_1229;
    let _e788: MultiVector = other_1023;
    let _e800: MultiVector = self_1229;
    let _e804: MultiVector = other_1023;
    let _e816: MultiVector = self_1229;
    let _e820: MultiVector = other_1023;
    let _e832: MultiVector = self_1229;
    let _e836: MultiVector = other_1023;
    let _e849: MultiVector = self_1229;
    let _e853: MultiVector = other_1023;
    let _e866: MultiVector = self_1229;
    let _e870: MultiVector = other_1023;
    let _e883: MultiVector = self_1229;
    let _e887: MultiVector = other_1023;
    let _e890: MultiVector = other_1023;
    let _e893: MultiVector = other_1023;
    let _e896: MultiVector = other_1023;
    let _e909: MultiVector = self_1229;
    let _e913: MultiVector = other_1023;
    let _e916: MultiVector = other_1023;
    let _e919: MultiVector = other_1023;
    let _e922: MultiVector = other_1023;
    let _e935: MultiVector = self_1229;
    let _e939: MultiVector = other_1023;
    let _e942: MultiVector = other_1023;
    let _e945: MultiVector = other_1023;
    let _e948: MultiVector = other_1023;
    let _e961: MultiVector = self_1229;
    let _e965: MultiVector = other_1023;
    let _e977: MultiVector = self_1229;
    let _e981: MultiVector = other_1023;
    let _e994: MultiVector = self_1229;
    let _e998: MultiVector = other_1023;
    let _e1011: MultiVector = self_1229;
    let _e1015: MultiVector = other_1023;
    let _e1028: MultiVector = self_1229;
    let _e1032: MultiVector = other_1023;
    let _e1035: MultiVector = other_1023;
    let _e1038: MultiVector = other_1023;
    let _e1041: MultiVector = other_1023;
    let _e1053: MultiVector = self_1229;
    let _e1056: MultiVector = self_1229;
    let _e1059: MultiVector = self_1229;
    let _e1062: MultiVector = self_1229;
    let _e1066: MultiVector = other_1023;
    let _e1077: MultiVector = self_1229;
    let _e1081: MultiVector = other_1023;
    let _e1084: MultiVector = self_1229;
    let _e1088: MultiVector = other_1023;
    let _e1092: MultiVector = self_1229;
    let _e1096: MultiVector = other_1023;
    let _e1099: MultiVector = other_1023;
    let _e1102: MultiVector = other_1023;
    let _e1114: MultiVector = self_1229;
    let _e1118: MultiVector = other_1023;
    let _e1121: MultiVector = other_1023;
    let _e1124: MultiVector = other_1023;
    let _e1136: MultiVector = self_1229;
    let _e1140: MultiVector = other_1023;
    let _e1151: MultiVector = self_1229;
    let _e1155: MultiVector = other_1023;
    let _e1166: MultiVector = self_1229;
    let _e1170: MultiVector = other_1023;
    let _e1181: MultiVector = self_1229;
    let _e1185: MultiVector = other_1023;
    let _e1196: MultiVector = self_1229;
    let _e1200: MultiVector = other_1023;
    let _e1211: MultiVector = self_1229;
    let _e1215: MultiVector = other_1023;
    let _e1226: MultiVector = self_1229;
    let _e1230: MultiVector = other_1023;
    let _e1241: MultiVector = self_1229;
    let _e1245: MultiVector = other_1023;
    let _e1256: MultiVector = self_1229;
    let _e1260: MultiVector = other_1023;
    let _e1271: MultiVector = self_1229;
    let _e1275: MultiVector = other_1023;
    let _e1278: MultiVector = other_1023;
    let _e1281: MultiVector = other_1023;
    let _e1293: MultiVector = self_1229;
    let _e1297: MultiVector = other_1023;
    let _e1300: MultiVector = self_1229;
    let _e1304: MultiVector = other_1023;
    let _e1315: MultiVector = self_1229;
    let _e1319: MultiVector = other_1023;
    let _e1330: MultiVector = self_1229;
    let _e1334: MultiVector = other_1023;
    let _e1345: MultiVector = self_1229;
    let _e1349: MultiVector = other_1023;
    let _e1353: MultiVector = self_1229;
    let _e1355: MultiVector = other_1023;
    let _e1365: MultiVector = self_1229;
    let _e1369: MultiVector = other_1023;
    let _e1372: MultiVector = self_1229;
    let _e1376: MultiVector = other_1023;
    let _e1380: MultiVector = self_1229;
    let _e1384: MultiVector = other_1023;
    let _e1387: MultiVector = other_1023;
    let _e1390: MultiVector = other_1023;
    let _e1402: MultiVector = self_1229;
    let _e1406: MultiVector = other_1023;
    let _e1409: MultiVector = other_1023;
    let _e1412: MultiVector = other_1023;
    let _e1424: MultiVector = self_1229;
    let _e1428: MultiVector = other_1023;
    let _e1439: MultiVector = self_1229;
    let _e1443: MultiVector = other_1023;
    let _e1454: MultiVector = self_1229;
    let _e1458: MultiVector = other_1023;
    let _e1469: MultiVector = self_1229;
    let _e1473: MultiVector = other_1023;
    let _e1484: MultiVector = self_1229;
    let _e1488: MultiVector = other_1023;
    let _e1499: MultiVector = self_1229;
    let _e1503: MultiVector = other_1023;
    let _e1514: MultiVector = self_1229;
    let _e1518: MultiVector = other_1023;
    let _e1529: MultiVector = self_1229;
    let _e1533: MultiVector = other_1023;
    let _e1544: MultiVector = self_1229;
    let _e1548: MultiVector = other_1023;
    let _e1559: MultiVector = self_1229;
    let _e1563: MultiVector = other_1023;
    let _e1566: MultiVector = other_1023;
    let _e1569: MultiVector = other_1023;
    let _e1581: MultiVector = self_1229;
    let _e1585: MultiVector = other_1023;
    let _e1588: MultiVector = self_1229;
    let _e1592: MultiVector = other_1023;
    let _e1603: MultiVector = self_1229;
    let _e1607: MultiVector = other_1023;
    let _e1618: MultiVector = self_1229;
    let _e1622: MultiVector = other_1023;
    let _e1633: MultiVector = self_1229;
    let _e1637: MultiVector = other_1023;
    let _e1641: MultiVector = self_1229;
    let _e1643: MultiVector = other_1023;
    let _e1653: MultiVector = self_1229;
    let _e1657: MultiVector = other_1023;
    let _e1660: MultiVector = self_1229;
    let _e1664: MultiVector = other_1023;
    let _e1676: MultiVector = self_1229;
    let _e1680: MultiVector = other_1023;
    let _e1692: MultiVector = self_1229;
    let _e1696: MultiVector = other_1023;
    let _e1708: MultiVector = self_1229;
    let _e1712: MultiVector = other_1023;
    let _e1724: MultiVector = self_1229;
    let _e1728: MultiVector = other_1023;
    let _e1740: MultiVector = self_1229;
    let _e1744: MultiVector = other_1023;
    let _e1756: MultiVector = self_1229;
    let _e1760: MultiVector = other_1023;
    let _e1772: MultiVector = self_1229;
    let _e1775: MultiVector = self_1229;
    let _e1778: MultiVector = self_1229;
    let _e1781: MultiVector = self_1229;
    let _e1785: MultiVector = other_1023;
    let _e1788: MultiVector = other_1023;
    let _e1791: MultiVector = other_1023;
    let _e1794: MultiVector = other_1023;
    let _e1809: MultiVector = self_1229;
    let _e1813: MultiVector = other_1023;
    let _e1816: MultiVector = self_1229;
    let _e1820: MultiVector = other_1023;
    let _e1832: MultiVector = self_1229;
    let _e1836: MultiVector = other_1023;
    let _e1848: MultiVector = self_1229;
    let _e1852: MultiVector = other_1023;
    let _e1864: MultiVector = self_1229;
    let _e1868: MultiVector = other_1023;
    let _e1880: MultiVector = self_1229;
    let _e1884: MultiVector = other_1023;
    let _e1896: MultiVector = self_1229;
    let _e1900: MultiVector = other_1023;
    let _e1912: MultiVector = self_1229;
    let _e1916: MultiVector = other_1023;
    let _e1928: MultiVector = self_1229;
    let _e1931: MultiVector = self_1229;
    let _e1934: MultiVector = self_1229;
    let _e1937: MultiVector = self_1229;
    let _e1941: MultiVector = other_1023;
    let _e1944: MultiVector = other_1023;
    let _e1947: MultiVector = other_1023;
    let _e1950: MultiVector = other_1023;
    return MultiVector(((((((((vec3<f32>(_e4.g0_.x) * _e8.g0_) + ((vec3<f32>(_e11.g1_.y) * vec3<f32>(_e15.g1_.y)) * vec3<f32>(1.0, 0.0, 0.0))) + ((vec3<f32>(_e26.g1_.z) * vec3<f32>(_e30.g1_.z)) * vec3<f32>(1.0, 0.0, 0.0))) + ((vec3<f32>(_e41.g5_.x) * vec3<f32>(_e45.g5_.x)) * vec3<f32>(-(1.0), 0.0, 0.0))) + ((vec3<f32>(_e57.g5_.y) * vec3<f32>(_e61.g5_.y)) * vec3<f32>(-(1.0), 0.0, 0.0))) + ((vec3<f32>(_e73.g5_.z) * vec3<f32>(_e77.g5_.z)) * vec3<f32>(-(1.0), 0.0, 0.0))) + ((vec3<f32>(_e89.g8_.w) * vec3<f32>(_e93.g8_.w)) * vec3<f32>(-(1.0), 0.0, 0.0))) + (vec3<f32>(_e105.g1_.x, _e108.g0_.y, _e111.g0_.z) * vec3<f32>(_e115.g1_.x, _e118.g0_.x, _e121.g0_.x))), ((((((((vec3<f32>(_e127.g0_.x) * _e131.g1_) + ((vec3<f32>(_e134.g1_.x) * vec3<f32>(_e138.g0_.x, _e141.g5_.z, _e144.g5_.y)) * vec3<f32>(1.0, 1.0, -(1.0)))) + ((vec3<f32>(_e156.g1_.y) * vec3<f32>(_e160.g5_.z, _e163.g0_.x, _e166.g5_.x)) * vec3<f32>(-(1.0), 1.0, 1.0))) + ((vec3<f32>(_e178.g1_.z) * vec3<f32>(_e182.g5_.y, _e185.g5_.x, _e188.g0_.x)) * vec3<f32>(1.0, -(1.0), 1.0))) + ((vec3<f32>(_e200.g5_.x) * vec3<f32>(_e204.g8_.w, _e207.g1_.z, _e210.g1_.y)) * vec3<f32>(1.0, 1.0, -(1.0)))) + ((vec3<f32>(_e222.g5_.y) * vec3<f32>(_e226.g1_.z, _e229.g8_.w, _e232.g1_.x)) * vec3<f32>(-(1.0), 1.0, 1.0))) + ((vec3<f32>(_e244.g5_.z) * vec3<f32>(_e248.g1_.y, _e251.g1_.x, _e254.g8_.w)) * vec3<f32>(1.0, -(1.0), 1.0))) + (vec3<f32>(_e266.g8_.w) * _e270.g5_)), ((((((((((((((((((((((((vec2<f32>(_e274.g0_.x) * _e278.g2_) + ((vec2<f32>(_e281.g1_.x) * vec2<f32>(_e285.g4_.x, _e288.g3_.x)) * vec2<f32>(-(1.0), 1.0))) + ((vec2<f32>(_e299.g1_.y) * vec2<f32>(_e303.g4_.y, _e306.g3_.y)) * vec2<f32>(-(1.0), 1.0))) + ((vec2<f32>(_e317.g1_.z) * vec2<f32>(_e321.g4_.z, _e324.g3_.z)) * vec2<f32>(-(1.0), 1.0))) + ((vec2<f32>(_e335.g2_.x) * vec2<f32>(_e339.g0_.x)) * vec2<f32>(1.0, 0.0))) + ((vec2<f32>(_e349.g2_.y) * vec2<f32>(_e353.g0_.x)) * vec2<f32>(0.0, 1.0))) + ((vec2<f32>(_e363.g3_.x) * vec2<f32>(_e367.g1_.x)) * vec2<f32>(0.0, -(1.0)))) + ((vec2<f32>(_e378.g3_.y) * vec2<f32>(_e382.g1_.y)) * vec2<f32>(0.0, -(1.0)))) + ((vec2<f32>(_e393.g3_.z) * vec2<f32>(_e397.g1_.z)) * vec2<f32>(0.0, -(1.0)))) + ((vec2<f32>(_e408.g4_.x) * vec2<f32>(_e412.g1_.x)) * vec2<f32>(1.0, 0.0))) + ((vec2<f32>(_e422.g4_.y) * vec2<f32>(_e426.g1_.y)) * vec2<f32>(1.0, 0.0))) + ((vec2<f32>(_e436.g4_.z) * vec2<f32>(_e440.g1_.z)) * vec2<f32>(1.0, 0.0))) - (vec2<f32>(_e450.g5_.x) * vec2<f32>(_e454.g8_.x, _e457.g7_.x))) - (vec2<f32>(_e463.g5_.y) * vec2<f32>(_e467.g8_.y, _e470.g7_.y))) - (vec2<f32>(_e476.g5_.z) * vec2<f32>(_e480.g8_.z, _e483.g7_.z))) + ((vec2<f32>(_e489.g7_.x) * vec2<f32>(_e493.g5_.x)) * vec2<f32>(0.0, -(1.0)))) + ((vec2<f32>(_e504.g7_.y) * vec2<f32>(_e508.g5_.y)) * vec2<f32>(0.0, -(1.0)))) + ((vec2<f32>(_e519.g7_.z) * vec2<f32>(_e523.g5_.z)) * vec2<f32>(0.0, -(1.0)))) + ((vec2<f32>(_e534.g8_.x) * vec2<f32>(_e538.g5_.x)) * vec2<f32>(-(1.0), 0.0))) + ((vec2<f32>(_e549.g8_.y) * vec2<f32>(_e553.g5_.y)) * vec2<f32>(-(1.0), 0.0))) + ((vec2<f32>(_e564.g8_.z) * vec2<f32>(_e568.g5_.z)) * vec2<f32>(-(1.0), 0.0))) + ((vec2<f32>(_e579.g8_.w) * vec2<f32>(_e583.g0_.y, _e586.g9_.w)) * vec2<f32>(1.0, -(1.0)))) + ((vec2<f32>(_e597.g9_.w) * vec2<f32>(_e601.g8_.w)) * vec2<f32>(0.0, 1.0))) + ((vec2<f32>(_e611.g0_.y, _e614.g0_.x) * vec2<f32>(_e618.g8_.w, _e621.g8_.x)) * vec2<f32>(-(1.0), 0.0))), (((((((((((((((((((((((vec4<f32>(_e632.g0_.x) * _e636.g3_) + ((vec4<f32>(_e639.g1_.x) * vec4<f32>(_e643.g7_.z, _e646.g7_.z, _e649.g7_.y, _e652.g6_.x)) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e666.g1_.y) * vec4<f32>(_e670.g7_.z, _e673.g7_.z, _e676.g7_.x, _e679.g6_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e693.g1_.z) * vec4<f32>(_e697.g7_.y, _e700.g7_.x, _e703.g7_.y, _e706.g6_.z)) * vec4<f32>(1.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e720.g3_.x) * vec4<f32>(_e724.g0_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e736.g3_.y) * vec4<f32>(_e740.g0_.x)) * vec4<f32>(0.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e752.g3_.z) * vec4<f32>(_e756.g0_.x)) * vec4<f32>(0.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e768.g3_.w) * vec4<f32>(_e772.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e784.g5_.x) * _e788.g9_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e800.g5_.y) * _e804.g9_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0)))) + ((vec4<f32>(_e816.g5_.z) * _e820.g9_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e832.g6_.x) * vec4<f32>(_e836.g1_.x)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e849.g6_.y) * vec4<f32>(_e853.g1_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e866.g6_.z) * vec4<f32>(_e870.g1_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e883.g7_.x) * vec4<f32>(_e887.g1_.z, _e890.g1_.z, _e893.g1_.y, _e896.g1_.z)) * vec4<f32>(0.0, -(1.0), 1.0, 0.0))) + ((vec4<f32>(_e909.g7_.y) * vec4<f32>(_e913.g1_.z, _e916.g1_.z, _e919.g1_.x, _e922.g1_.z)) * vec4<f32>(1.0, 0.0, -(1.0), 0.0))) + ((vec4<f32>(_e935.g7_.z) * vec4<f32>(_e939.g1_.y, _e942.g1_.x, _e945.g1_.y, _e948.g1_.y)) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))) + ((vec4<f32>(_e961.g8_.w) * vec4<f32>(_e965.g0_.z)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e977.g9_.x) * vec4<f32>(_e981.g5_.x)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e994.g9_.y) * vec4<f32>(_e998.g5_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e1011.g9_.z) * vec4<f32>(_e1015.g5_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e1028.g9_.w) * vec4<f32>(_e1032.g5_.x, _e1035.g5_.y, _e1038.g5_.z, _e1041.g5_.x)) * vec4<f32>(1.0, 1.0, 1.0, 0.0))) + ((vec4<f32>(_e1053.g0_.x, _e1056.g0_.x, _e1059.g0_.x, _e1062.g0_.z) * _e1066.g8_.xxxw) * vec4<f32>(0.0, 0.0, 0.0, 1.0))), ((((((((((((((vec3<f32>(_e1077.g0_.x) * _e1081.g4_) + (vec3<f32>(_e1084.g0_.y) * _e1088.g5_)) + ((vec3<f32>(_e1092.g1_.y) * vec3<f32>(_e1096.g8_.z, _e1099.g8_.z, _e1102.g8_.x)) * vec3<f32>(1.0, 0.0, -(1.0)))) + ((vec3<f32>(_e1114.g1_.z) * vec3<f32>(_e1118.g8_.y, _e1121.g8_.x, _e1124.g8_.y)) * vec3<f32>(-(1.0), 1.0, 0.0))) + ((vec3<f32>(_e1136.g4_.x) * vec3<f32>(_e1140.g0_.x)) * vec3<f32>(1.0, 0.0, 0.0))) + ((vec3<f32>(_e1151.g4_.y) * vec3<f32>(_e1155.g0_.x)) * vec3<f32>(0.0, 1.0, 0.0))) + ((vec3<f32>(_e1166.g4_.z) * vec3<f32>(_e1170.g0_.x)) * vec3<f32>(0.0, 0.0, 1.0))) + ((vec3<f32>(_e1181.g5_.x) * vec3<f32>(_e1185.g0_.y)) * vec3<f32>(1.0, 0.0, 0.0))) + ((vec3<f32>(_e1196.g5_.y) * vec3<f32>(_e1200.g0_.y)) * vec3<f32>(0.0, 1.0, 0.0))) + ((vec3<f32>(_e1211.g5_.z) * vec3<f32>(_e1215.g0_.y)) * vec3<f32>(0.0, 0.0, 1.0))) + ((vec3<f32>(_e1226.g8_.x) * _e1230.g1_.zzy) * vec3<f32>(0.0, 1.0, -(1.0)))) + ((vec3<f32>(_e1241.g8_.y) * _e1245.g1_.zzx) * vec3<f32>(-(1.0), 0.0, 1.0))) + ((vec3<f32>(_e1256.g8_.z) * _e1260.g1_.yxy) * vec3<f32>(1.0, -(1.0), 0.0))) + ((vec3<f32>(_e1271.g1_.x) * vec3<f32>(_e1275.g8_.x, _e1278.g8_.z, _e1281.g8_.y)) * vec3<f32>(0.0, -(1.0), 1.0))), ((((((vec3<f32>(_e1293.g0_.x) * _e1297.g5_) + ((vec3<f32>(_e1300.g5_.x) * vec3<f32>(_e1304.g0_.x)) * vec3<f32>(1.0, 0.0, 0.0))) + ((vec3<f32>(_e1315.g5_.y) * vec3<f32>(_e1319.g0_.x)) * vec3<f32>(0.0, 1.0, 0.0))) + ((vec3<f32>(_e1330.g5_.z) * vec3<f32>(_e1334.g0_.x)) * vec3<f32>(0.0, 0.0, 1.0))) - (vec3<f32>(_e1345.g8_.w) * _e1349.g1_)) + ((_e1353.g1_ * vec3<f32>(_e1355.g8_.w)) * vec3<f32>(-(1.0)))), ((((((((((((((vec3<f32>(_e1365.g0_.x) * _e1369.g6_) + (vec3<f32>(_e1372.g0_.z) * _e1376.g5_)) + ((vec3<f32>(_e1380.g1_.y) * vec3<f32>(_e1384.g9_.z, _e1387.g9_.z, _e1390.g9_.x)) * vec3<f32>(1.0, 0.0, -(1.0)))) + ((vec3<f32>(_e1402.g1_.z) * vec3<f32>(_e1406.g9_.y, _e1409.g9_.x, _e1412.g9_.y)) * vec3<f32>(-(1.0), 1.0, 0.0))) + ((vec3<f32>(_e1424.g5_.x) * vec3<f32>(_e1428.g0_.z)) * vec3<f32>(1.0, 0.0, 0.0))) + ((vec3<f32>(_e1439.g5_.y) * vec3<f32>(_e1443.g0_.z)) * vec3<f32>(0.0, 1.0, 0.0))) + ((vec3<f32>(_e1454.g5_.z) * vec3<f32>(_e1458.g0_.z)) * vec3<f32>(0.0, 0.0, 1.0))) + ((vec3<f32>(_e1469.g6_.x) * vec3<f32>(_e1473.g0_.x)) * vec3<f32>(1.0, 0.0, 0.0))) + ((vec3<f32>(_e1484.g6_.y) * vec3<f32>(_e1488.g0_.x)) * vec3<f32>(0.0, 1.0, 0.0))) + ((vec3<f32>(_e1499.g6_.z) * vec3<f32>(_e1503.g0_.x)) * vec3<f32>(0.0, 0.0, 1.0))) + ((vec3<f32>(_e1514.g9_.x) * _e1518.g1_.zzy) * vec3<f32>(0.0, -(1.0), 1.0))) + ((vec3<f32>(_e1529.g9_.y) * _e1533.g1_.zzx) * vec3<f32>(1.0, 0.0, -(1.0)))) + ((vec3<f32>(_e1544.g9_.z) * _e1548.g1_.yxy) * vec3<f32>(-(1.0), 1.0, 0.0))) + ((vec3<f32>(_e1559.g1_.x) * vec3<f32>(_e1563.g9_.x, _e1566.g9_.z, _e1569.g9_.y)) * vec3<f32>(0.0, -(1.0), 1.0))), ((((((vec3<f32>(_e1581.g0_.x) * _e1585.g7_) + ((vec3<f32>(_e1588.g7_.x) * vec3<f32>(_e1592.g0_.x)) * vec3<f32>(1.0, 0.0, 0.0))) + ((vec3<f32>(_e1603.g7_.y) * vec3<f32>(_e1607.g0_.x)) * vec3<f32>(0.0, 1.0, 0.0))) + ((vec3<f32>(_e1618.g7_.z) * vec3<f32>(_e1622.g0_.x)) * vec3<f32>(0.0, 0.0, 1.0))) + (vec3<f32>(_e1633.g9_.w) * _e1637.g1_)) + ((_e1641.g1_ * vec3<f32>(_e1643.g9_.w)) * vec3<f32>(-(1.0)))), (((((((((vec4<f32>(_e1653.g0_.x) * _e1657.g8_) + ((vec4<f32>(_e1660.g1_.x) * vec4<f32>(_e1664.g0_.y)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e1676.g1_.y) * vec4<f32>(_e1680.g0_.y)) * vec4<f32>(0.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e1692.g1_.z) * vec4<f32>(_e1696.g0_.y)) * vec4<f32>(0.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e1708.g8_.x) * vec4<f32>(_e1712.g0_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e1724.g8_.y) * vec4<f32>(_e1728.g0_.x)) * vec4<f32>(0.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e1740.g8_.z) * vec4<f32>(_e1744.g0_.x)) * vec4<f32>(0.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e1756.g8_.w) * vec4<f32>(_e1760.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e1772.g0_.y, _e1775.g0_.y, _e1778.g0_.y, _e1781.g0_.x) * vec4<f32>(_e1785.g1_.x, _e1788.g1_.y, _e1791.g1_.z, _e1794.g1_.x)) * vec4<f32>(-(1.0), -(1.0), -(1.0), 0.0))), (((((((((vec4<f32>(_e1809.g0_.x) * _e1813.g9_) + ((vec4<f32>(_e1816.g1_.x) * vec4<f32>(_e1820.g0_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e1832.g1_.y) * vec4<f32>(_e1836.g0_.z)) * vec4<f32>(0.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e1848.g1_.z) * vec4<f32>(_e1852.g0_.z)) * vec4<f32>(0.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e1864.g9_.x) * vec4<f32>(_e1868.g0_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e1880.g9_.y) * vec4<f32>(_e1884.g0_.x)) * vec4<f32>(0.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e1896.g9_.z) * vec4<f32>(_e1900.g0_.x)) * vec4<f32>(0.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e1912.g9_.w) * vec4<f32>(_e1916.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e1928.g0_.z, _e1931.g0_.z, _e1934.g0_.z, _e1937.g0_.x) * vec4<f32>(_e1941.g1_.x, _e1944.g1_.y, _e1947.g1_.z, _e1950.g1_.x)) * vec4<f32>(1.0, 1.0, 1.0, 0.0))));
}

fn multi_vector_multi_vector_left_contraction(self_1230: MultiVector, other_1024: MultiVector) -> MultiVector {
    var self_1231: MultiVector;
    var other_1025: MultiVector;

    self_1231 = self_1230;
    other_1025 = other_1024;
    let _e4: MultiVector = self_1231;
    let _e8: MultiVector = other_1025;
    let _e11: MultiVector = self_1231;
    let _e15: MultiVector = other_1025;
    let _e26: MultiVector = self_1231;
    let _e30: MultiVector = other_1025;
    let _e41: MultiVector = self_1231;
    let _e45: MultiVector = other_1025;
    let _e57: MultiVector = self_1231;
    let _e61: MultiVector = other_1025;
    let _e73: MultiVector = self_1231;
    let _e77: MultiVector = other_1025;
    let _e89: MultiVector = self_1231;
    let _e93: MultiVector = other_1025;
    let _e105: MultiVector = self_1231;
    let _e109: MultiVector = other_1025;
    let _e120: MultiVector = self_1231;
    let _e124: MultiVector = other_1025;
    let _e127: MultiVector = self_1231;
    let _e131: MultiVector = other_1025;
    let _e142: MultiVector = self_1231;
    let _e146: MultiVector = other_1025;
    let _e157: MultiVector = self_1231;
    let _e161: MultiVector = other_1025;
    let _e172: MultiVector = self_1231;
    let _e176: MultiVector = other_1025;
    let _e187: MultiVector = self_1231;
    let _e191: MultiVector = other_1025;
    let _e202: MultiVector = self_1231;
    let _e206: MultiVector = other_1025;
    let _e217: MultiVector = self_1231;
    let _e221: MultiVector = other_1025;
    let _e224: MultiVector = self_1231;
    let _e228: MultiVector = other_1025;
    let _e231: MultiVector = other_1025;
    let _e242: MultiVector = self_1231;
    let _e246: MultiVector = other_1025;
    let _e249: MultiVector = other_1025;
    let _e260: MultiVector = self_1231;
    let _e264: MultiVector = other_1025;
    let _e267: MultiVector = other_1025;
    let _e278: MultiVector = self_1231;
    let _e282: MultiVector = other_1025;
    let _e285: MultiVector = other_1025;
    let _e291: MultiVector = self_1231;
    let _e295: MultiVector = other_1025;
    let _e298: MultiVector = other_1025;
    let _e304: MultiVector = self_1231;
    let _e308: MultiVector = other_1025;
    let _e311: MultiVector = other_1025;
    let _e317: MultiVector = self_1231;
    let _e321: MultiVector = other_1025;
    let _e324: MultiVector = other_1025;
    let _e335: MultiVector = self_1231;
    let _e339: MultiVector = other_1025;
    let _e342: MultiVector = self_1231;
    let _e346: MultiVector = other_1025;
    let _e349: MultiVector = other_1025;
    let _e352: MultiVector = other_1025;
    let _e355: MultiVector = other_1025;
    let _e369: MultiVector = self_1231;
    let _e373: MultiVector = other_1025;
    let _e376: MultiVector = other_1025;
    let _e379: MultiVector = other_1025;
    let _e382: MultiVector = other_1025;
    let _e396: MultiVector = self_1231;
    let _e400: MultiVector = other_1025;
    let _e412: MultiVector = self_1231;
    let _e416: MultiVector = other_1025;
    let _e428: MultiVector = self_1231;
    let _e432: MultiVector = other_1025;
    let _e444: MultiVector = self_1231;
    let _e448: MultiVector = other_1025;
    let _e460: MultiVector = self_1231;
    let _e464: MultiVector = other_1025;
    let _e467: MultiVector = other_1025;
    let _e470: MultiVector = other_1025;
    let _e473: MultiVector = other_1025;
    let _e487: MultiVector = self_1231;
    let _e491: MultiVector = other_1025;
    let _e494: MultiVector = self_1231;
    let _e498: MultiVector = other_1025;
    let _e501: MultiVector = other_1025;
    let _e504: MultiVector = other_1025;
    let _e516: MultiVector = self_1231;
    let _e520: MultiVector = other_1025;
    let _e523: MultiVector = other_1025;
    let _e526: MultiVector = other_1025;
    let _e538: MultiVector = self_1231;
    let _e542: MultiVector = other_1025;
    let _e553: MultiVector = self_1231;
    let _e557: MultiVector = other_1025;
    let _e568: MultiVector = self_1231;
    let _e572: MultiVector = other_1025;
    let _e583: MultiVector = self_1231;
    let _e587: MultiVector = other_1025;
    let _e590: MultiVector = other_1025;
    let _e593: MultiVector = other_1025;
    let _e605: MultiVector = self_1231;
    let _e609: MultiVector = other_1025;
    let _e612: MultiVector = self_1231;
    let _e614: MultiVector = other_1025;
    let _e624: MultiVector = self_1231;
    let _e628: MultiVector = other_1025;
    let _e631: MultiVector = self_1231;
    let _e635: MultiVector = other_1025;
    let _e638: MultiVector = other_1025;
    let _e641: MultiVector = other_1025;
    let _e653: MultiVector = self_1231;
    let _e657: MultiVector = other_1025;
    let _e660: MultiVector = other_1025;
    let _e663: MultiVector = other_1025;
    let _e675: MultiVector = self_1231;
    let _e679: MultiVector = other_1025;
    let _e690: MultiVector = self_1231;
    let _e694: MultiVector = other_1025;
    let _e705: MultiVector = self_1231;
    let _e709: MultiVector = other_1025;
    let _e720: MultiVector = self_1231;
    let _e724: MultiVector = other_1025;
    let _e727: MultiVector = other_1025;
    let _e730: MultiVector = other_1025;
    let _e742: MultiVector = self_1231;
    let _e746: MultiVector = other_1025;
    let _e749: MultiVector = self_1231;
    let _e751: MultiVector = other_1025;
    let _e761: MultiVector = self_1231;
    let _e765: MultiVector = other_1025;
    let _e768: MultiVector = self_1231;
    let _e771: MultiVector = self_1231;
    let _e774: MultiVector = self_1231;
    let _e777: MultiVector = self_1231;
    let _e781: MultiVector = other_1025;
    let _e784: MultiVector = other_1025;
    let _e787: MultiVector = other_1025;
    let _e790: MultiVector = other_1025;
    let _e802: MultiVector = self_1231;
    let _e806: MultiVector = other_1025;
    let _e809: MultiVector = self_1231;
    let _e812: MultiVector = self_1231;
    let _e815: MultiVector = self_1231;
    let _e818: MultiVector = self_1231;
    let _e822: MultiVector = other_1025;
    let _e825: MultiVector = other_1025;
    let _e828: MultiVector = other_1025;
    let _e831: MultiVector = other_1025;
    return MultiVector(((((((((vec3<f32>(_e4.g0_.x) * _e8.g0_) + ((vec3<f32>(_e11.g1_.y) * vec3<f32>(_e15.g1_.y)) * vec3<f32>(1.0, 0.0, 0.0))) + ((vec3<f32>(_e26.g1_.z) * vec3<f32>(_e30.g1_.z)) * vec3<f32>(1.0, 0.0, 0.0))) + ((vec3<f32>(_e41.g5_.x) * vec3<f32>(_e45.g5_.x)) * vec3<f32>(-(1.0), 0.0, 0.0))) + ((vec3<f32>(_e57.g5_.y) * vec3<f32>(_e61.g5_.y)) * vec3<f32>(-(1.0), 0.0, 0.0))) + ((vec3<f32>(_e73.g5_.z) * vec3<f32>(_e77.g5_.z)) * vec3<f32>(-(1.0), 0.0, 0.0))) + ((vec3<f32>(_e89.g8_.w) * vec3<f32>(_e93.g8_.w)) * vec3<f32>(-(1.0), 0.0, 0.0))) + ((vec3<f32>(_e105.g1_.x) * vec3<f32>(_e109.g1_.x)) * vec3<f32>(1.0, 0.0, 0.0))), (((((((vec3<f32>(_e120.g0_.x) * _e124.g1_) + ((vec3<f32>(_e127.g1_.y) * _e131.g5_.zzx) * vec3<f32>(-(1.0), 0.0, 1.0))) + ((vec3<f32>(_e142.g1_.z) * _e146.g5_.yxy) * vec3<f32>(1.0, -(1.0), 0.0))) + ((vec3<f32>(_e157.g5_.x) * vec3<f32>(_e161.g8_.w)) * vec3<f32>(1.0, 0.0, 0.0))) + ((vec3<f32>(_e172.g5_.y) * vec3<f32>(_e176.g8_.w)) * vec3<f32>(0.0, 1.0, 0.0))) + ((vec3<f32>(_e187.g5_.z) * vec3<f32>(_e191.g8_.w)) * vec3<f32>(0.0, 0.0, 1.0))) + ((vec3<f32>(_e202.g1_.x) * _e206.g5_.xzy) * vec3<f32>(0.0, 1.0, -(1.0)))), ((((((((vec2<f32>(_e217.g0_.x) * _e221.g2_) + ((vec2<f32>(_e224.g1_.x) * vec2<f32>(_e228.g4_.x, _e231.g3_.x)) * vec2<f32>(-(1.0), 1.0))) + ((vec2<f32>(_e242.g1_.y) * vec2<f32>(_e246.g4_.y, _e249.g3_.y)) * vec2<f32>(-(1.0), 1.0))) + ((vec2<f32>(_e260.g1_.z) * vec2<f32>(_e264.g4_.z, _e267.g3_.z)) * vec2<f32>(-(1.0), 1.0))) - (vec2<f32>(_e278.g5_.x) * vec2<f32>(_e282.g8_.x, _e285.g7_.x))) - (vec2<f32>(_e291.g5_.y) * vec2<f32>(_e295.g8_.y, _e298.g7_.y))) - (vec2<f32>(_e304.g5_.z) * vec2<f32>(_e308.g8_.z, _e311.g7_.z))) + ((vec2<f32>(_e317.g8_.w) * vec2<f32>(_e321.g0_.y, _e324.g9_.w)) * vec2<f32>(1.0, -(1.0)))), ((((((((vec4<f32>(_e335.g0_.x) * _e339.g3_) + ((vec4<f32>(_e342.g1_.y) * vec4<f32>(_e346.g7_.z, _e349.g7_.z, _e352.g7_.x, _e355.g6_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e369.g1_.z) * vec4<f32>(_e373.g7_.y, _e376.g7_.x, _e379.g7_.y, _e382.g6_.z)) * vec4<f32>(1.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e396.g5_.x) * _e400.g9_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e412.g5_.y) * _e416.g9_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0)))) + ((vec4<f32>(_e428.g5_.z) * _e432.g9_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e444.g8_.w) * vec4<f32>(_e448.g0_.z)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e460.g1_.x) * vec4<f32>(_e464.g7_.x, _e467.g7_.z, _e470.g7_.y, _e473.g6_.x)) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))), (((((((vec3<f32>(_e487.g0_.x) * _e491.g4_) + ((vec3<f32>(_e494.g1_.y) * vec3<f32>(_e498.g8_.z, _e501.g8_.z, _e504.g8_.x)) * vec3<f32>(1.0, 0.0, -(1.0)))) + ((vec3<f32>(_e516.g1_.z) * vec3<f32>(_e520.g8_.y, _e523.g8_.x, _e526.g8_.y)) * vec3<f32>(-(1.0), 1.0, 0.0))) + ((vec3<f32>(_e538.g5_.x) * vec3<f32>(_e542.g0_.y)) * vec3<f32>(1.0, 0.0, 0.0))) + ((vec3<f32>(_e553.g5_.y) * vec3<f32>(_e557.g0_.y)) * vec3<f32>(0.0, 1.0, 0.0))) + ((vec3<f32>(_e568.g5_.z) * vec3<f32>(_e572.g0_.y)) * vec3<f32>(0.0, 0.0, 1.0))) + ((vec3<f32>(_e583.g1_.x) * vec3<f32>(_e587.g8_.x, _e590.g8_.z, _e593.g8_.y)) * vec3<f32>(0.0, -(1.0), 1.0))), ((vec3<f32>(_e605.g0_.x) * _e609.g5_) + ((_e612.g1_ * vec3<f32>(_e614.g8_.w)) * vec3<f32>(-(1.0)))), (((((((vec3<f32>(_e624.g0_.x) * _e628.g6_) + ((vec3<f32>(_e631.g1_.y) * vec3<f32>(_e635.g9_.z, _e638.g9_.z, _e641.g9_.x)) * vec3<f32>(1.0, 0.0, -(1.0)))) + ((vec3<f32>(_e653.g1_.z) * vec3<f32>(_e657.g9_.y, _e660.g9_.x, _e663.g9_.y)) * vec3<f32>(-(1.0), 1.0, 0.0))) + ((vec3<f32>(_e675.g5_.x) * vec3<f32>(_e679.g0_.z)) * vec3<f32>(1.0, 0.0, 0.0))) + ((vec3<f32>(_e690.g5_.y) * vec3<f32>(_e694.g0_.z)) * vec3<f32>(0.0, 1.0, 0.0))) + ((vec3<f32>(_e705.g5_.z) * vec3<f32>(_e709.g0_.z)) * vec3<f32>(0.0, 0.0, 1.0))) + ((vec3<f32>(_e720.g1_.x) * vec3<f32>(_e724.g9_.x, _e727.g9_.z, _e730.g9_.y)) * vec3<f32>(0.0, -(1.0), 1.0))), ((vec3<f32>(_e742.g0_.x) * _e746.g7_) + ((_e749.g1_ * vec3<f32>(_e751.g9_.w)) * vec3<f32>(-(1.0)))), ((vec4<f32>(_e761.g0_.x) * _e765.g8_) + ((vec4<f32>(_e768.g1_.x, _e771.g1_.y, _e774.g1_.z, _e777.g1_.x) * vec4<f32>(_e781.g0_.y, _e784.g0_.y, _e787.g0_.y, _e790.g0_.x)) * vec4<f32>(1.0, 1.0, 1.0, 0.0))), ((vec4<f32>(_e802.g0_.x) * _e806.g9_) + ((vec4<f32>(_e809.g1_.x, _e812.g1_.y, _e815.g1_.z, _e818.g1_.x) * vec4<f32>(_e822.g0_.z, _e825.g0_.z, _e828.g0_.z, _e831.g0_.x)) * vec4<f32>(1.0, 1.0, 1.0, 0.0))));
}

fn multi_vector_multi_vector_right_contraction(self_1232: MultiVector, other_1026: MultiVector) -> MultiVector {
    var self_1233: MultiVector;
    var other_1027: MultiVector;

    self_1233 = self_1232;
    other_1027 = other_1026;
    let _e4: MultiVector = self_1233;
    let _e8: MultiVector = other_1027;
    let _e18: MultiVector = self_1233;
    let _e22: MultiVector = other_1027;
    let _e33: MultiVector = self_1233;
    let _e37: MultiVector = other_1027;
    let _e48: MultiVector = self_1233;
    let _e52: MultiVector = other_1027;
    let _e64: MultiVector = self_1233;
    let _e68: MultiVector = other_1027;
    let _e80: MultiVector = self_1233;
    let _e84: MultiVector = other_1027;
    let _e96: MultiVector = self_1233;
    let _e100: MultiVector = other_1027;
    let _e112: MultiVector = self_1233;
    let _e114: MultiVector = other_1027;
    let _e120: MultiVector = self_1233;
    let _e124: MultiVector = other_1027;
    let _e134: MultiVector = self_1233;
    let _e138: MultiVector = other_1027;
    let _e149: MultiVector = self_1233;
    let _e153: MultiVector = other_1027;
    let _e164: MultiVector = self_1233;
    let _e168: MultiVector = other_1027;
    let _e172: MultiVector = self_1233;
    let _e174: MultiVector = other_1027;
    let _e180: MultiVector = self_1233;
    let _e184: MultiVector = other_1027;
    let _e193: MultiVector = self_1233;
    let _e197: MultiVector = other_1027;
    let _e207: MultiVector = self_1233;
    let _e211: MultiVector = other_1027;
    let _e222: MultiVector = self_1233;
    let _e226: MultiVector = other_1027;
    let _e237: MultiVector = self_1233;
    let _e241: MultiVector = other_1027;
    let _e252: MultiVector = self_1233;
    let _e256: MultiVector = other_1027;
    let _e266: MultiVector = self_1233;
    let _e270: MultiVector = other_1027;
    let _e280: MultiVector = self_1233;
    let _e284: MultiVector = other_1027;
    let _e294: MultiVector = self_1233;
    let _e298: MultiVector = other_1027;
    let _e309: MultiVector = self_1233;
    let _e313: MultiVector = other_1027;
    let _e324: MultiVector = self_1233;
    let _e328: MultiVector = other_1027;
    let _e339: MultiVector = self_1233;
    let _e343: MultiVector = other_1027;
    let _e354: MultiVector = self_1233;
    let _e358: MultiVector = other_1027;
    let _e369: MultiVector = self_1233;
    let _e373: MultiVector = other_1027;
    let _e384: MultiVector = self_1233;
    let _e388: MultiVector = other_1027;
    let _e398: MultiVector = self_1233;
    let _e401: MultiVector = self_1233;
    let _e405: MultiVector = other_1027;
    let _e408: MultiVector = other_1027;
    let _e419: MultiVector = self_1233;
    let _e423: MultiVector = other_1027;
    let _e434: MultiVector = self_1233;
    let _e438: MultiVector = other_1027;
    let _e450: MultiVector = self_1233;
    let _e454: MultiVector = other_1027;
    let _e466: MultiVector = self_1233;
    let _e470: MultiVector = other_1027;
    let _e482: MultiVector = self_1233;
    let _e486: MultiVector = other_1027;
    let _e499: MultiVector = self_1233;
    let _e503: MultiVector = other_1027;
    let _e516: MultiVector = self_1233;
    let _e520: MultiVector = other_1027;
    let _e533: MultiVector = self_1233;
    let _e537: MultiVector = other_1027;
    let _e540: MultiVector = other_1027;
    let _e543: MultiVector = other_1027;
    let _e546: MultiVector = other_1027;
    let _e559: MultiVector = self_1233;
    let _e563: MultiVector = other_1027;
    let _e566: MultiVector = other_1027;
    let _e569: MultiVector = other_1027;
    let _e572: MultiVector = other_1027;
    let _e585: MultiVector = self_1233;
    let _e589: MultiVector = other_1027;
    let _e592: MultiVector = other_1027;
    let _e595: MultiVector = other_1027;
    let _e598: MultiVector = other_1027;
    let _e611: MultiVector = self_1233;
    let _e615: MultiVector = other_1027;
    let _e628: MultiVector = self_1233;
    let _e632: MultiVector = other_1027;
    let _e645: MultiVector = self_1233;
    let _e649: MultiVector = other_1027;
    let _e662: MultiVector = self_1233;
    let _e666: MultiVector = other_1027;
    let _e669: MultiVector = other_1027;
    let _e672: MultiVector = other_1027;
    let _e675: MultiVector = other_1027;
    let _e687: MultiVector = self_1233;
    let _e690: MultiVector = self_1233;
    let _e693: MultiVector = self_1233;
    let _e696: MultiVector = self_1233;
    let _e700: MultiVector = other_1027;
    let _e711: MultiVector = self_1233;
    let _e715: MultiVector = other_1027;
    let _e718: MultiVector = self_1233;
    let _e722: MultiVector = other_1027;
    let _e733: MultiVector = self_1233;
    let _e737: MultiVector = other_1027;
    let _e748: MultiVector = self_1233;
    let _e752: MultiVector = other_1027;
    let _e763: MultiVector = self_1233;
    let _e765: MultiVector = other_1027;
    let _e773: MultiVector = self_1233;
    let _e777: MultiVector = other_1027;
    let _e781: MultiVector = self_1233;
    let _e783: MultiVector = other_1027;
    let _e789: MultiVector = self_1233;
    let _e793: MultiVector = other_1027;
    let _e796: MultiVector = self_1233;
    let _e800: MultiVector = other_1027;
    let _e811: MultiVector = self_1233;
    let _e815: MultiVector = other_1027;
    let _e826: MultiVector = self_1233;
    let _e830: MultiVector = other_1027;
    let _e841: MultiVector = self_1233;
    let _e843: MultiVector = other_1027;
    let _e849: MultiVector = self_1233;
    let _e853: MultiVector = other_1027;
    let _e856: MultiVector = self_1233;
    let _e858: MultiVector = other_1027;
    let _e864: MultiVector = self_1233;
    let _e868: MultiVector = other_1027;
    let _e879: MultiVector = self_1233;
    let _e883: MultiVector = other_1027;
    let _e895: MultiVector = self_1233;
    let _e899: MultiVector = other_1027;
    let _e911: MultiVector = self_1233;
    let _e915: MultiVector = other_1027;
    let _e927: MultiVector = self_1233;
    let _e930: MultiVector = self_1233;
    let _e933: MultiVector = self_1233;
    let _e936: MultiVector = self_1233;
    let _e940: MultiVector = other_1027;
    let _e943: MultiVector = other_1027;
    let _e946: MultiVector = other_1027;
    let _e949: MultiVector = other_1027;
    let _e964: MultiVector = self_1233;
    let _e968: MultiVector = other_1027;
    let _e979: MultiVector = self_1233;
    let _e983: MultiVector = other_1027;
    let _e995: MultiVector = self_1233;
    let _e999: MultiVector = other_1027;
    let _e1011: MultiVector = self_1233;
    let _e1015: MultiVector = other_1027;
    let _e1027: MultiVector = self_1233;
    let _e1030: MultiVector = self_1233;
    let _e1033: MultiVector = self_1233;
    let _e1036: MultiVector = self_1233;
    let _e1040: MultiVector = other_1027;
    let _e1043: MultiVector = other_1027;
    let _e1046: MultiVector = other_1027;
    let _e1049: MultiVector = other_1027;
    return MultiVector((((((((((vec3<f32>(_e4.g1_.x) * vec3<f32>(_e8.g1_.x)) * vec3<f32>(1.0, 0.0, 0.0)) + ((vec3<f32>(_e18.g1_.y) * vec3<f32>(_e22.g1_.y)) * vec3<f32>(1.0, 0.0, 0.0))) + ((vec3<f32>(_e33.g1_.z) * vec3<f32>(_e37.g1_.z)) * vec3<f32>(1.0, 0.0, 0.0))) + ((vec3<f32>(_e48.g5_.x) * vec3<f32>(_e52.g5_.x)) * vec3<f32>(-(1.0), 0.0, 0.0))) + ((vec3<f32>(_e64.g5_.y) * vec3<f32>(_e68.g5_.y)) * vec3<f32>(-(1.0), 0.0, 0.0))) + ((vec3<f32>(_e80.g5_.z) * vec3<f32>(_e84.g5_.z)) * vec3<f32>(-(1.0), 0.0, 0.0))) + ((vec3<f32>(_e96.g8_.w) * vec3<f32>(_e100.g8_.w)) * vec3<f32>(-(1.0), 0.0, 0.0))) + (_e112.g0_ * vec3<f32>(_e114.g0_.x))), ((((((vec3<f32>(_e120.g5_.x) * _e124.g1_.zzy) * vec3<f32>(0.0, 1.0, -(1.0))) + ((vec3<f32>(_e134.g5_.y) * _e138.g1_.zzx) * vec3<f32>(-(1.0), 0.0, 1.0))) + ((vec3<f32>(_e149.g5_.z) * _e153.g1_.yxy) * vec3<f32>(1.0, -(1.0), 0.0))) + (vec3<f32>(_e164.g8_.w) * _e168.g5_)) + (_e172.g1_ * vec3<f32>(_e174.g0_.x))), (((((((((((((((((vec2<f32>(_e180.g2_.x) * vec2<f32>(_e184.g0_.x)) * vec2<f32>(1.0, 0.0)) + ((vec2<f32>(_e193.g2_.y) * vec2<f32>(_e197.g0_.x)) * vec2<f32>(0.0, 1.0))) + ((vec2<f32>(_e207.g3_.x) * vec2<f32>(_e211.g1_.x)) * vec2<f32>(0.0, -(1.0)))) + ((vec2<f32>(_e222.g3_.y) * vec2<f32>(_e226.g1_.y)) * vec2<f32>(0.0, -(1.0)))) + ((vec2<f32>(_e237.g3_.z) * vec2<f32>(_e241.g1_.z)) * vec2<f32>(0.0, -(1.0)))) + ((vec2<f32>(_e252.g4_.x) * vec2<f32>(_e256.g1_.x)) * vec2<f32>(1.0, 0.0))) + ((vec2<f32>(_e266.g4_.y) * vec2<f32>(_e270.g1_.y)) * vec2<f32>(1.0, 0.0))) + ((vec2<f32>(_e280.g4_.z) * vec2<f32>(_e284.g1_.z)) * vec2<f32>(1.0, 0.0))) + ((vec2<f32>(_e294.g7_.x) * vec2<f32>(_e298.g5_.x)) * vec2<f32>(0.0, -(1.0)))) + ((vec2<f32>(_e309.g7_.y) * vec2<f32>(_e313.g5_.y)) * vec2<f32>(0.0, -(1.0)))) + ((vec2<f32>(_e324.g7_.z) * vec2<f32>(_e328.g5_.z)) * vec2<f32>(0.0, -(1.0)))) + ((vec2<f32>(_e339.g8_.x) * vec2<f32>(_e343.g5_.x)) * vec2<f32>(-(1.0), 0.0))) + ((vec2<f32>(_e354.g8_.y) * vec2<f32>(_e358.g5_.y)) * vec2<f32>(-(1.0), 0.0))) + ((vec2<f32>(_e369.g8_.z) * vec2<f32>(_e373.g5_.z)) * vec2<f32>(-(1.0), 0.0))) + ((vec2<f32>(_e384.g9_.w) * vec2<f32>(_e388.g8_.w)) * vec2<f32>(0.0, 1.0))) + ((vec2<f32>(_e398.g0_.y, _e401.g0_.x) * vec2<f32>(_e405.g8_.w, _e408.g8_.x)) * vec2<f32>(-(1.0), 0.0))), ((((((((((((((((vec4<f32>(_e419.g3_.x) * vec4<f32>(_e423.g0_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) + ((vec4<f32>(_e434.g3_.y) * vec4<f32>(_e438.g0_.x)) * vec4<f32>(0.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e450.g3_.z) * vec4<f32>(_e454.g0_.x)) * vec4<f32>(0.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e466.g3_.w) * vec4<f32>(_e470.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e482.g6_.x) * vec4<f32>(_e486.g1_.x)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e499.g6_.y) * vec4<f32>(_e503.g1_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e516.g6_.z) * vec4<f32>(_e520.g1_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e533.g7_.x) * vec4<f32>(_e537.g1_.z, _e540.g1_.z, _e543.g1_.y, _e546.g1_.z)) * vec4<f32>(0.0, -(1.0), 1.0, 0.0))) + ((vec4<f32>(_e559.g7_.y) * vec4<f32>(_e563.g1_.z, _e566.g1_.z, _e569.g1_.x, _e572.g1_.z)) * vec4<f32>(1.0, 0.0, -(1.0), 0.0))) + ((vec4<f32>(_e585.g7_.z) * vec4<f32>(_e589.g1_.y, _e592.g1_.x, _e595.g1_.y, _e598.g1_.y)) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))) + ((vec4<f32>(_e611.g9_.x) * vec4<f32>(_e615.g5_.x)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e628.g9_.y) * vec4<f32>(_e632.g5_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e645.g9_.z) * vec4<f32>(_e649.g5_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e662.g9_.w) * vec4<f32>(_e666.g5_.x, _e669.g5_.y, _e672.g5_.z, _e675.g5_.x)) * vec4<f32>(1.0, 1.0, 1.0, 0.0))) + ((vec4<f32>(_e687.g0_.x, _e690.g0_.x, _e693.g0_.x, _e696.g0_.z) * _e700.g8_.xxxw) * vec4<f32>(0.0, 0.0, 0.0, 1.0))), (((((vec3<f32>(_e711.g0_.y) * _e715.g5_) + ((vec3<f32>(_e718.g8_.x) * _e722.g1_.zzy) * vec3<f32>(0.0, 1.0, -(1.0)))) + ((vec3<f32>(_e733.g8_.y) * _e737.g1_.zzx) * vec3<f32>(-(1.0), 0.0, 1.0))) + ((vec3<f32>(_e748.g8_.z) * _e752.g1_.yxy) * vec3<f32>(1.0, -(1.0), 0.0))) + (_e763.g4_ * vec3<f32>(_e765.g0_.x))), ((vec3<f32>(0.0) - (vec3<f32>(_e773.g8_.w) * _e777.g1_)) + (_e781.g5_ * vec3<f32>(_e783.g0_.x))), (((((vec3<f32>(_e789.g0_.z) * _e793.g5_) + ((vec3<f32>(_e796.g9_.x) * _e800.g1_.zzy) * vec3<f32>(0.0, -(1.0), 1.0))) + ((vec3<f32>(_e811.g9_.y) * _e815.g1_.zzx) * vec3<f32>(1.0, 0.0, -(1.0)))) + ((vec3<f32>(_e826.g9_.z) * _e830.g1_.yxy) * vec3<f32>(-(1.0), 1.0, 0.0))) + (_e841.g6_ * vec3<f32>(_e843.g0_.x))), ((vec3<f32>(_e849.g9_.w) * _e853.g1_) + (_e856.g7_ * vec3<f32>(_e858.g0_.x))), ((((((vec4<f32>(_e864.g8_.x) * vec4<f32>(_e868.g0_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) + ((vec4<f32>(_e879.g8_.y) * vec4<f32>(_e883.g0_.x)) * vec4<f32>(0.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e895.g8_.z) * vec4<f32>(_e899.g0_.x)) * vec4<f32>(0.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e911.g8_.w) * vec4<f32>(_e915.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e927.g0_.y, _e930.g0_.y, _e933.g0_.y, _e936.g0_.x) * vec4<f32>(_e940.g1_.x, _e943.g1_.y, _e946.g1_.z, _e949.g1_.x)) * vec4<f32>(-(1.0), -(1.0), -(1.0), 0.0))), ((((((vec4<f32>(_e964.g9_.x) * vec4<f32>(_e968.g0_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) + ((vec4<f32>(_e979.g9_.y) * vec4<f32>(_e983.g0_.x)) * vec4<f32>(0.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e995.g9_.z) * vec4<f32>(_e999.g0_.x)) * vec4<f32>(0.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e1011.g9_.w) * vec4<f32>(_e1015.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e1027.g0_.z, _e1030.g0_.z, _e1033.g0_.z, _e1036.g0_.x) * vec4<f32>(_e1040.g1_.x, _e1043.g1_.y, _e1046.g1_.z, _e1049.g1_.x)) * vec4<f32>(1.0, 1.0, 1.0, 0.0))));
}

fn multi_vector_multi_vector_scalar_product(self_1234: MultiVector, other_1028: MultiVector) -> Scalar {
    var self_1235: MultiVector;
    var other_1029: MultiVector;

    self_1235 = self_1234;
    other_1029 = other_1028;
    let _e4: MultiVector = self_1235;
    let _e7: MultiVector = other_1029;
    let _e11: MultiVector = self_1235;
    let _e14: MultiVector = other_1029;
    let _e19: MultiVector = self_1235;
    let _e22: MultiVector = other_1029;
    let _e27: MultiVector = self_1235;
    let _e30: MultiVector = other_1029;
    let _e35: MultiVector = self_1235;
    let _e38: MultiVector = other_1029;
    let _e43: MultiVector = self_1235;
    let _e46: MultiVector = other_1029;
    let _e51: MultiVector = self_1235;
    let _e54: MultiVector = other_1029;
    let _e59: MultiVector = self_1235;
    let _e62: MultiVector = other_1029;
    return Scalar(((((((((_e4.g0_.x * _e7.g0_.x) + (_e11.g1_.x * _e14.g1_.x)) + (_e19.g1_.y * _e22.g1_.y)) + (_e27.g1_.z * _e30.g1_.z)) - (_e35.g5_.x * _e38.g5_.x)) - (_e43.g5_.y * _e46.g5_.y)) - (_e51.g5_.z * _e54.g5_.z)) - (_e59.g8_.w * _e62.g8_.w)));
}

fn multi_vector_squared_magnitude(self_1236: MultiVector) -> Scalar {
    var self_1237: MultiVector;

    self_1237 = self_1236;
    let _e2: MultiVector = self_1237;
    let _e3: MultiVector = self_1237;
    let _e4: MultiVector = multi_vector_reversal(_e3);
    let _e5: Scalar = multi_vector_multi_vector_scalar_product(_e2, _e4);
    return _e5;
}

fn multi_vector_magnitude(self_1238: MultiVector) -> Scalar {
    var self_1239: MultiVector;

    self_1239 = self_1238;
    let _e2: MultiVector = self_1239;
    let _e3: Scalar = multi_vector_squared_magnitude(_e2);
    return Scalar(sqrt(_e3.g0_));
}

fn multi_vector_scale(self_1240: MultiVector, other_1030: f32) -> MultiVector {
    var self_1241: MultiVector;
    var other_1031: f32;

    self_1241 = self_1240;
    other_1031 = other_1030;
    let _e4: MultiVector = self_1241;
    let _e5: f32 = other_1031;
    let _e7: MultiVector = multi_vector_scalar_geometric_product(_e4, Scalar(_e5));
    return _e7;
}

fn multi_vector_signum(self_1242: MultiVector) -> MultiVector {
    var self_1243: MultiVector;

    self_1243 = self_1242;
    let _e2: MultiVector = self_1243;
    let _e3: MultiVector = self_1243;
    let _e4: Scalar = multi_vector_magnitude(_e3);
    let _e9: MultiVector = multi_vector_scalar_geometric_product(_e2, Scalar((1.0 / _e4.g0_)));
    return _e9;
}

fn multi_vector_inverse(self_1244: MultiVector) -> MultiVector {
    var self_1245: MultiVector;

    self_1245 = self_1244;
    let _e2: MultiVector = self_1245;
    let _e3: MultiVector = multi_vector_reversal(_e2);
    let _e4: MultiVector = self_1245;
    let _e5: Scalar = multi_vector_squared_magnitude(_e4);
    let _e10: MultiVector = multi_vector_scalar_geometric_product(_e3, Scalar((1.0 / _e5.g0_)));
    return _e10;
}

fn anti_scalar_scalar_geometric_quotient(self_1246: AntiScalar, other_1032: Scalar) -> AntiScalar {
    var self_1247: AntiScalar;
    var other_1033: Scalar;

    self_1247 = self_1246;
    other_1033 = other_1032;
    let _e4: AntiScalar = self_1247;
    let _e5: Scalar = other_1033;
    let _e6: Scalar = scalar_inverse(_e5);
    let _e7: AntiScalar = anti_scalar_scalar_geometric_product(_e4, _e6);
    return _e7;
}

fn circle_flector_transformation(self_1248: Circle, other_1034: Flector) -> Flector {
    var self_1249: Circle;
    var other_1035: Flector;

    self_1249 = self_1248;
    other_1035 = other_1034;
    let _e4: Circle = self_1249;
    let _e5: Flector = other_1035;
    let _e6: Motor = circle_flector_geometric_product(_e4, _e5);
    let _e7: Circle = self_1249;
    let _e8: Circle = circle_reversal(_e7);
    let _e9: Flector = motor_circle_geometric_product(_e6, _e8);
    return _e9;
}

fn circle_motor_transformation(self_1250: Circle, other_1036: Motor) -> Motor {
    var self_1251: Circle;
    var other_1037: Motor;

    self_1251 = self_1250;
    other_1037 = other_1036;
    let _e4: Circle = self_1251;
    let _e5: Motor = other_1037;
    let _e6: Flector = circle_motor_geometric_product(_e4, _e5);
    let _e7: Circle = self_1251;
    let _e8: Circle = circle_reversal(_e7);
    let _e9: Motor = flector_circle_geometric_product(_e6, _e8);
    return _e9;
}

fn circle_multi_vector_geometric_quotient(self_1252: Circle, other_1038: MultiVector) -> MultiVector {
    var self_1253: Circle;
    var other_1039: MultiVector;

    self_1253 = self_1252;
    other_1039 = other_1038;
    let _e4: Circle = self_1253;
    let _e5: MultiVector = other_1039;
    let _e6: MultiVector = multi_vector_inverse(_e5);
    let _e7: MultiVector = circle_multi_vector_geometric_product(_e4, _e6);
    return _e7;
}

fn circle_multi_vector_transformation(self_1254: Circle, other_1040: MultiVector) -> MultiVector {
    var self_1255: Circle;
    var other_1041: MultiVector;

    self_1255 = self_1254;
    other_1041 = other_1040;
    let _e4: Circle = self_1255;
    let _e5: MultiVector = other_1041;
    let _e6: MultiVector = circle_multi_vector_geometric_product(_e4, _e5);
    let _e7: Circle = self_1255;
    let _e8: Circle = circle_reversal(_e7);
    let _e9: MultiVector = multi_vector_circle_geometric_product(_e6, _e8);
    return _e9;
}

fn circle_scalar_geometric_quotient(self_1256: Circle, other_1042: Scalar) -> Circle {
    var self_1257: Circle;
    var other_1043: Scalar;

    self_1257 = self_1256;
    other_1043 = other_1042;
    let _e4: Circle = self_1257;
    let _e5: Scalar = other_1043;
    let _e6: Scalar = scalar_inverse(_e5);
    let _e7: Circle = circle_scalar_geometric_product(_e4, _e6);
    return _e7;
}

fn dipole_flat_point_transformation(self_1258: Dipole, other_1044: FlatPoint) -> FlatPoint {
    var self_1259: Dipole;
    var other_1045: FlatPoint;

    self_1259 = self_1258;
    other_1045 = other_1044;
    let _e4: Dipole = self_1259;
    let _e5: FlatPoint = other_1045;
    let _e6: Flector = dipole_flat_point_geometric_product(_e4, _e5);
    let _e7: Dipole = self_1259;
    let _e8: Dipole = dipole_reversal(_e7);
    let _e9: Flector = flector_dipole_geometric_product(_e6, _e8);
    let _e10: FlatPoint = flector_flat_point_into(_e9);
    return _e10;
}

fn dipole_flector_transformation(self_1260: Dipole, other_1046: Flector) -> Flector {
    var self_1261: Dipole;
    var other_1047: Flector;

    self_1261 = self_1260;
    other_1047 = other_1046;
    let _e4: Dipole = self_1261;
    let _e5: Flector = other_1047;
    let _e6: Flector = dipole_flector_geometric_product(_e4, _e5);
    let _e7: Dipole = self_1261;
    let _e8: Dipole = dipole_reversal(_e7);
    let _e9: Flector = flector_dipole_geometric_product(_e6, _e8);
    return _e9;
}

fn dipole_line_transformation(self_1262: Dipole, other_1048: Line) -> Line {
    var self_1263: Dipole;
    var other_1049: Line;

    self_1263 = self_1262;
    other_1049 = other_1048;
    let _e4: Dipole = self_1263;
    let _e5: Line = other_1049;
    let _e6: Motor = dipole_line_geometric_product(_e4, _e5);
    let _e7: Dipole = self_1263;
    let _e8: Dipole = dipole_reversal(_e7);
    let _e9: Motor = motor_dipole_geometric_product(_e6, _e8);
    let _e10: Line = motor_line_into(_e9);
    return _e10;
}

fn dipole_motor_transformation(self_1264: Dipole, other_1050: Motor) -> Motor {
    var self_1265: Dipole;
    var other_1051: Motor;

    self_1265 = self_1264;
    other_1051 = other_1050;
    let _e4: Dipole = self_1265;
    let _e5: Motor = other_1051;
    let _e6: Motor = dipole_motor_geometric_product(_e4, _e5);
    let _e7: Dipole = self_1265;
    let _e8: Dipole = dipole_reversal(_e7);
    let _e9: Motor = motor_dipole_geometric_product(_e6, _e8);
    return _e9;
}

fn dipole_multi_vector_geometric_quotient(self_1266: Dipole, other_1052: MultiVector) -> MultiVector {
    var self_1267: Dipole;
    var other_1053: MultiVector;

    self_1267 = self_1266;
    other_1053 = other_1052;
    let _e4: Dipole = self_1267;
    let _e5: MultiVector = other_1053;
    let _e6: MultiVector = multi_vector_inverse(_e5);
    let _e7: MultiVector = dipole_multi_vector_geometric_product(_e4, _e6);
    return _e7;
}

fn dipole_multi_vector_transformation(self_1268: Dipole, other_1054: MultiVector) -> MultiVector {
    var self_1269: Dipole;
    var other_1055: MultiVector;

    self_1269 = self_1268;
    other_1055 = other_1054;
    let _e4: Dipole = self_1269;
    let _e5: MultiVector = other_1055;
    let _e6: MultiVector = dipole_multi_vector_geometric_product(_e4, _e5);
    let _e7: Dipole = self_1269;
    let _e8: Dipole = dipole_reversal(_e7);
    let _e9: MultiVector = multi_vector_dipole_geometric_product(_e6, _e8);
    return _e9;
}

fn dipole_rotor_transformation(self_1270: Dipole, other_1056: Rotor) -> Rotor {
    var self_1271: Dipole;
    var other_1057: Rotor;

    self_1271 = self_1270;
    other_1057 = other_1056;
    let _e4: Dipole = self_1271;
    let _e5: Rotor = other_1057;
    let _e6: Rotor = dipole_rotor_geometric_product(_e4, _e5);
    let _e7: Dipole = self_1271;
    let _e8: Dipole = dipole_reversal(_e7);
    let _e9: Rotor = rotor_dipole_geometric_product(_e6, _e8);
    return _e9;
}

fn dipole_scalar_geometric_quotient(self_1272: Dipole, other_1058: Scalar) -> Dipole {
    var self_1273: Dipole;
    var other_1059: Scalar;

    self_1273 = self_1272;
    other_1059 = other_1058;
    let _e4: Dipole = self_1273;
    let _e5: Scalar = other_1059;
    let _e6: Scalar = scalar_inverse(_e5);
    let _e7: Dipole = dipole_scalar_geometric_product(_e4, _e6);
    return _e7;
}

fn dipole_translator_transformation(self_1274: Dipole, other_1060: Translator) -> Translator {
    var self_1275: Dipole;
    var other_1061: Translator;

    self_1275 = self_1274;
    other_1061 = other_1060;
    let _e4: Dipole = self_1275;
    let _e5: Translator = other_1061;
    let _e6: Motor = dipole_translator_geometric_product(_e4, _e5);
    let _e7: Dipole = self_1275;
    let _e8: Dipole = dipole_reversal(_e7);
    let _e9: Motor = motor_dipole_geometric_product(_e6, _e8);
    let _e10: Translator = motor_translator_into(_e9);
    return _e10;
}

fn flat_point_dipole_geometric_quotient(self_1276: FlatPoint, other_1062: Dipole) -> Flector {
    var self_1277: FlatPoint;
    var other_1063: Dipole;

    self_1277 = self_1276;
    other_1063 = other_1062;
    let _e4: FlatPoint = self_1277;
    let _e5: Dipole = other_1063;
    let _e6: Dipole = dipole_inverse(_e5);
    let _e7: Flector = flat_point_dipole_geometric_product(_e4, _e6);
    return _e7;
}

fn flat_point_scalar_geometric_quotient(self_1278: FlatPoint, other_1064: Scalar) -> FlatPoint {
    var self_1279: FlatPoint;
    var other_1065: Scalar;

    self_1279 = self_1278;
    other_1065 = other_1064;
    let _e4: FlatPoint = self_1279;
    let _e5: Scalar = other_1065;
    let _e6: Scalar = scalar_inverse(_e5);
    let _e7: FlatPoint = flat_point_scalar_geometric_product(_e4, _e6);
    return _e7;
}

fn flector_circle_geometric_quotient(self_1280: Flector, other_1066: Circle) -> Motor {
    var self_1281: Flector;
    var other_1067: Circle;

    self_1281 = self_1280;
    other_1067 = other_1066;
    let _e4: Flector = self_1281;
    let _e5: Circle = other_1067;
    let _e6: Circle = circle_inverse(_e5);
    let _e7: Motor = flector_circle_geometric_product(_e4, _e6);
    return _e7;
}

fn flector_dipole_geometric_quotient(self_1282: Flector, other_1068: Dipole) -> Flector {
    var self_1283: Flector;
    var other_1069: Dipole;

    self_1283 = self_1282;
    other_1069 = other_1068;
    let _e4: Flector = self_1283;
    let _e5: Dipole = other_1069;
    let _e6: Dipole = dipole_inverse(_e5);
    let _e7: Flector = flector_dipole_geometric_product(_e4, _e6);
    return _e7;
}

fn flector_radial_point_geometric_quotient(self_1284: Flector, other_1070: RadialPoint) -> Motor {
    var self_1285: Flector;
    var other_1071: RadialPoint;

    self_1285 = self_1284;
    other_1071 = other_1070;
    let _e4: Flector = self_1285;
    let _e5: RadialPoint = other_1071;
    let _e6: RadialPoint = radial_point_inverse(_e5);
    let _e7: Motor = flector_radial_point_geometric_product(_e4, _e6);
    return _e7;
}

fn flector_scalar_geometric_quotient(self_1286: Flector, other_1072: Scalar) -> Flector {
    var self_1287: Flector;
    var other_1073: Scalar;

    self_1287 = self_1286;
    other_1073 = other_1072;
    let _e4: Flector = self_1287;
    let _e5: Scalar = other_1073;
    let _e6: Scalar = scalar_inverse(_e5);
    let _e7: Flector = flector_scalar_geometric_product(_e4, _e6);
    return _e7;
}

fn line_dipole_geometric_quotient(self_1288: Line, other_1074: Dipole) -> Motor {
    var self_1289: Line;
    var other_1075: Dipole;

    self_1289 = self_1288;
    other_1075 = other_1074;
    let _e4: Line = self_1289;
    let _e5: Dipole = other_1075;
    let _e6: Dipole = dipole_inverse(_e5);
    let _e7: Motor = line_dipole_geometric_product(_e4, _e6);
    return _e7;
}

fn line_radial_point_geometric_quotient(self_1290: Line, other_1076: RadialPoint) -> Flector {
    var self_1291: Line;
    var other_1077: RadialPoint;

    self_1291 = self_1290;
    other_1077 = other_1076;
    let _e4: Line = self_1291;
    let _e5: RadialPoint = other_1077;
    let _e6: RadialPoint = radial_point_inverse(_e5);
    let _e7: Flector = line_radial_point_geometric_product(_e4, _e6);
    return _e7;
}

fn line_scalar_geometric_quotient(self_1292: Line, other_1078: Scalar) -> Line {
    var self_1293: Line;
    var other_1079: Scalar;

    self_1293 = self_1292;
    other_1079 = other_1078;
    let _e4: Line = self_1293;
    let _e5: Scalar = other_1079;
    let _e6: Scalar = scalar_inverse(_e5);
    let _e7: Line = line_scalar_geometric_product(_e4, _e6);
    return _e7;
}

fn motor_circle_geometric_quotient(self_1294: Motor, other_1080: Circle) -> Flector {
    var self_1295: Motor;
    var other_1081: Circle;

    self_1295 = self_1294;
    other_1081 = other_1080;
    let _e4: Motor = self_1295;
    let _e5: Circle = other_1081;
    let _e6: Circle = circle_inverse(_e5);
    let _e7: Flector = motor_circle_geometric_product(_e4, _e6);
    return _e7;
}

fn motor_dipole_geometric_quotient(self_1296: Motor, other_1082: Dipole) -> Motor {
    var self_1297: Motor;
    var other_1083: Dipole;

    self_1297 = self_1296;
    other_1083 = other_1082;
    let _e4: Motor = self_1297;
    let _e5: Dipole = other_1083;
    let _e6: Dipole = dipole_inverse(_e5);
    let _e7: Motor = motor_dipole_geometric_product(_e4, _e6);
    return _e7;
}

fn motor_radial_point_geometric_quotient(self_1298: Motor, other_1084: RadialPoint) -> Flector {
    var self_1299: Motor;
    var other_1085: RadialPoint;

    self_1299 = self_1298;
    other_1085 = other_1084;
    let _e4: Motor = self_1299;
    let _e5: RadialPoint = other_1085;
    let _e6: RadialPoint = radial_point_inverse(_e5);
    let _e7: Flector = motor_radial_point_geometric_product(_e4, _e6);
    return _e7;
}

fn motor_scalar_geometric_quotient(self_1300: Motor, other_1086: Scalar) -> Motor {
    var self_1301: Motor;
    var other_1087: Scalar;

    self_1301 = self_1300;
    other_1087 = other_1086;
    let _e4: Motor = self_1301;
    let _e5: Scalar = other_1087;
    let _e6: Scalar = scalar_inverse(_e5);
    let _e7: Motor = motor_scalar_geometric_product(_e4, _e6);
    return _e7;
}

fn multi_vector_circle_geometric_quotient(self_1302: MultiVector, other_1088: Circle) -> MultiVector {
    var self_1303: MultiVector;
    var other_1089: Circle;

    self_1303 = self_1302;
    other_1089 = other_1088;
    let _e4: MultiVector = self_1303;
    let _e5: Circle = other_1089;
    let _e6: Circle = circle_inverse(_e5);
    let _e7: MultiVector = multi_vector_circle_geometric_product(_e4, _e6);
    return _e7;
}

fn multi_vector_circle_transformation(self_1304: MultiVector, other_1090: Circle) -> Circle {
    var self_1305: MultiVector;
    var other_1091: Circle;

    self_1305 = self_1304;
    other_1091 = other_1090;
    let _e4: MultiVector = self_1305;
    let _e5: Circle = other_1091;
    let _e6: MultiVector = multi_vector_circle_geometric_product(_e4, _e5);
    let _e7: MultiVector = self_1305;
    let _e8: MultiVector = multi_vector_reversal(_e7);
    let _e9: MultiVector = multi_vector_multi_vector_geometric_product(_e6, _e8);
    let _e10: Circle = multi_vector_circle_into(_e9);
    return _e10;
}

fn multi_vector_dipole_geometric_quotient(self_1306: MultiVector, other_1092: Dipole) -> MultiVector {
    var self_1307: MultiVector;
    var other_1093: Dipole;

    self_1307 = self_1306;
    other_1093 = other_1092;
    let _e4: MultiVector = self_1307;
    let _e5: Dipole = other_1093;
    let _e6: Dipole = dipole_inverse(_e5);
    let _e7: MultiVector = multi_vector_dipole_geometric_product(_e4, _e6);
    return _e7;
}

fn multi_vector_dipole_transformation(self_1308: MultiVector, other_1094: Dipole) -> Dipole {
    var self_1309: MultiVector;
    var other_1095: Dipole;

    self_1309 = self_1308;
    other_1095 = other_1094;
    let _e4: MultiVector = self_1309;
    let _e5: Dipole = other_1095;
    let _e6: MultiVector = multi_vector_dipole_geometric_product(_e4, _e5);
    let _e7: MultiVector = self_1309;
    let _e8: MultiVector = multi_vector_reversal(_e7);
    let _e9: MultiVector = multi_vector_multi_vector_geometric_product(_e6, _e8);
    let _e10: Dipole = multi_vector_dipole_into(_e9);
    return _e10;
}

fn multi_vector_powi(self_1310: MultiVector, exponent: i32) -> MultiVector {
    var self_1311: MultiVector;
    var exponent_1: i32;
    var local: MultiVector;
    var x: MultiVector;
    var y: MultiVector;
    var n: i32;

    self_1311 = self_1310;
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
        let _e11: MultiVector = self_1311;
        let _e12: MultiVector = multi_vector_inverse(_e11);
        local = _e12;
    } else {
        let _e14: MultiVector = self_1311;
        local = _e14;
    }
    let _e15: MultiVector = local;
    x = _e15;
    let _e17: MultiVector = multi_vector_one();
    y = _e17;
    let _e19: i32 = exponent_1;
    n = abs(_e19);
    loop {
        let _e23: i32 = n;
        if !((1 < _e23)) {
            break;
        }
        {
            let _e26: i32 = n;
            if ((_e26 & 1) == 1) {
                {
                    let _e31: MultiVector = x;
                    let _e32: MultiVector = y;
                    let _e33: MultiVector = multi_vector_multi_vector_geometric_product(_e31, _e32);
                    y = _e33;
                }
            }
            let _e34: MultiVector = x;
            let _e35: MultiVector = x;
            let _e36: MultiVector = multi_vector_multi_vector_geometric_product(_e34, _e35);
            x = _e36;
            let _e37: i32 = n;
            n = (_e37 >> u32(1));
        }
    }
    let _e41: MultiVector = x;
    let _e42: MultiVector = y;
    let _e43: MultiVector = multi_vector_multi_vector_geometric_product(_e41, _e42);
    return _e43;
}

fn multi_vector_multi_vector_geometric_quotient(self_1312: MultiVector, other_1096: MultiVector) -> MultiVector {
    var self_1313: MultiVector;
    var other_1097: MultiVector;

    self_1313 = self_1312;
    other_1097 = other_1096;
    let _e4: MultiVector = self_1313;
    let _e5: MultiVector = other_1097;
    let _e6: MultiVector = multi_vector_inverse(_e5);
    let _e7: MultiVector = multi_vector_multi_vector_geometric_product(_e4, _e6);
    return _e7;
}

fn multi_vector_multi_vector_transformation(self_1314: MultiVector, other_1098: MultiVector) -> MultiVector {
    var self_1315: MultiVector;
    var other_1099: MultiVector;

    self_1315 = self_1314;
    other_1099 = other_1098;
    let _e4: MultiVector = self_1315;
    let _e5: MultiVector = other_1099;
    let _e6: MultiVector = multi_vector_multi_vector_geometric_product(_e4, _e5);
    let _e7: MultiVector = self_1315;
    let _e8: MultiVector = multi_vector_reversal(_e7);
    let _e9: MultiVector = multi_vector_multi_vector_geometric_product(_e6, _e8);
    return _e9;
}

fn multi_vector_radial_point_geometric_quotient(self_1316: MultiVector, other_1100: RadialPoint) -> MultiVector {
    var self_1317: MultiVector;
    var other_1101: RadialPoint;

    self_1317 = self_1316;
    other_1101 = other_1100;
    let _e4: MultiVector = self_1317;
    let _e5: RadialPoint = other_1101;
    let _e6: RadialPoint = radial_point_inverse(_e5);
    let _e7: MultiVector = multi_vector_radial_point_geometric_product(_e4, _e6);
    return _e7;
}

fn multi_vector_radial_point_transformation(self_1318: MultiVector, other_1102: RadialPoint) -> RadialPoint {
    var self_1319: MultiVector;
    var other_1103: RadialPoint;

    self_1319 = self_1318;
    other_1103 = other_1102;
    let _e4: MultiVector = self_1319;
    let _e5: RadialPoint = other_1103;
    let _e6: MultiVector = multi_vector_radial_point_geometric_product(_e4, _e5);
    let _e7: MultiVector = self_1319;
    let _e8: MultiVector = multi_vector_reversal(_e7);
    let _e9: MultiVector = multi_vector_multi_vector_geometric_product(_e6, _e8);
    let _e10: RadialPoint = multi_vector_radial_point_into(_e9);
    return _e10;
}

fn multi_vector_scalar_geometric_quotient(self_1320: MultiVector, other_1104: Scalar) -> MultiVector {
    var self_1321: MultiVector;
    var other_1105: Scalar;

    self_1321 = self_1320;
    other_1105 = other_1104;
    let _e4: MultiVector = self_1321;
    let _e5: Scalar = other_1105;
    let _e6: Scalar = scalar_inverse(_e5);
    let _e7: MultiVector = multi_vector_scalar_geometric_product(_e4, _e6);
    return _e7;
}

fn multi_vector_scalar_transformation(self_1322: MultiVector, other_1106: Scalar) -> Scalar {
    var self_1323: MultiVector;
    var other_1107: Scalar;

    self_1323 = self_1322;
    other_1107 = other_1106;
    let _e4: MultiVector = self_1323;
    let _e5: Scalar = other_1107;
    let _e6: MultiVector = multi_vector_scalar_geometric_product(_e4, _e5);
    let _e7: MultiVector = self_1323;
    let _e8: MultiVector = multi_vector_reversal(_e7);
    let _e9: MultiVector = multi_vector_multi_vector_geometric_product(_e6, _e8);
    let _e10: Scalar = multi_vector_scalar_into(_e9);
    return _e10;
}

fn plane_scalar_geometric_quotient(self_1324: Plane, other_1108: Scalar) -> Plane {
    var self_1325: Plane;
    var other_1109: Scalar;

    self_1325 = self_1324;
    other_1109 = other_1108;
    let _e4: Plane = self_1325;
    let _e5: Scalar = other_1109;
    let _e6: Scalar = scalar_inverse(_e5);
    let _e7: Plane = plane_scalar_geometric_product(_e4, _e6);
    return _e7;
}

fn radial_point_flector_transformation(self_1326: RadialPoint, other_1110: Flector) -> Flector {
    var self_1327: RadialPoint;
    var other_1111: Flector;

    self_1327 = self_1326;
    other_1111 = other_1110;
    let _e4: RadialPoint = self_1327;
    let _e5: Flector = other_1111;
    let _e6: Motor = radial_point_flector_geometric_product(_e4, _e5);
    let _e7: RadialPoint = self_1327;
    let _e8: RadialPoint = radial_point_reversal(_e7);
    let _e9: Flector = motor_radial_point_geometric_product(_e6, _e8);
    return _e9;
}

fn radial_point_line_transformation(self_1328: RadialPoint, other_1112: Line) -> Line {
    var self_1329: RadialPoint;
    var other_1113: Line;

    self_1329 = self_1328;
    other_1113 = other_1112;
    let _e4: RadialPoint = self_1329;
    let _e5: Line = other_1113;
    let _e6: Flector = radial_point_line_geometric_product(_e4, _e5);
    let _e7: RadialPoint = self_1329;
    let _e8: RadialPoint = radial_point_reversal(_e7);
    let _e9: Motor = flector_radial_point_geometric_product(_e6, _e8);
    let _e10: Line = motor_line_into(_e9);
    return _e10;
}

fn radial_point_motor_transformation(self_1330: RadialPoint, other_1114: Motor) -> Motor {
    var self_1331: RadialPoint;
    var other_1115: Motor;

    self_1331 = self_1330;
    other_1115 = other_1114;
    let _e4: RadialPoint = self_1331;
    let _e5: Motor = other_1115;
    let _e6: Flector = radial_point_motor_geometric_product(_e4, _e5);
    let _e7: RadialPoint = self_1331;
    let _e8: RadialPoint = radial_point_reversal(_e7);
    let _e9: Motor = flector_radial_point_geometric_product(_e6, _e8);
    return _e9;
}

fn radial_point_multi_vector_geometric_quotient(self_1332: RadialPoint, other_1116: MultiVector) -> MultiVector {
    var self_1333: RadialPoint;
    var other_1117: MultiVector;

    self_1333 = self_1332;
    other_1117 = other_1116;
    let _e4: RadialPoint = self_1333;
    let _e5: MultiVector = other_1117;
    let _e6: MultiVector = multi_vector_inverse(_e5);
    let _e7: MultiVector = radial_point_multi_vector_geometric_product(_e4, _e6);
    return _e7;
}

fn radial_point_multi_vector_transformation(self_1334: RadialPoint, other_1118: MultiVector) -> MultiVector {
    var self_1335: RadialPoint;
    var other_1119: MultiVector;

    self_1335 = self_1334;
    other_1119 = other_1118;
    let _e4: RadialPoint = self_1335;
    let _e5: MultiVector = other_1119;
    let _e6: MultiVector = radial_point_multi_vector_geometric_product(_e4, _e5);
    let _e7: RadialPoint = self_1335;
    let _e8: RadialPoint = radial_point_reversal(_e7);
    let _e9: MultiVector = multi_vector_radial_point_geometric_product(_e6, _e8);
    return _e9;
}

fn radial_point_scalar_geometric_quotient(self_1336: RadialPoint, other_1120: Scalar) -> RadialPoint {
    var self_1337: RadialPoint;
    var other_1121: Scalar;

    self_1337 = self_1336;
    other_1121 = other_1120;
    let _e4: RadialPoint = self_1337;
    let _e5: Scalar = other_1121;
    let _e6: Scalar = scalar_inverse(_e5);
    let _e7: RadialPoint = radial_point_scalar_geometric_product(_e4, _e6);
    return _e7;
}

fn rotor_dipole_geometric_quotient(self_1338: Rotor, other_1122: Dipole) -> Rotor {
    var self_1339: Rotor;
    var other_1123: Dipole;

    self_1339 = self_1338;
    other_1123 = other_1122;
    let _e4: Rotor = self_1339;
    let _e5: Dipole = other_1123;
    let _e6: Dipole = dipole_inverse(_e5);
    let _e7: Rotor = rotor_dipole_geometric_product(_e4, _e6);
    return _e7;
}

fn rotor_scalar_geometric_quotient(self_1340: Rotor, other_1124: Scalar) -> Rotor {
    var self_1341: Rotor;
    var other_1125: Scalar;

    self_1341 = self_1340;
    other_1125 = other_1124;
    let _e4: Rotor = self_1341;
    let _e5: Scalar = other_1125;
    let _e6: Scalar = scalar_inverse(_e5);
    let _e7: Rotor = rotor_scalar_geometric_product(_e4, _e6);
    return _e7;
}

fn scalar_anti_scalar_transformation(self_1342: Scalar, other_1126: AntiScalar) -> AntiScalar {
    var self_1343: Scalar;
    var other_1127: AntiScalar;

    self_1343 = self_1342;
    other_1127 = other_1126;
    let _e4: Scalar = self_1343;
    let _e5: AntiScalar = other_1127;
    let _e6: AntiScalar = scalar_anti_scalar_geometric_product(_e4, _e5);
    let _e7: Scalar = self_1343;
    let _e8: Scalar = scalar_reversal(_e7);
    let _e9: AntiScalar = anti_scalar_scalar_geometric_product(_e6, _e8);
    return _e9;
}

fn scalar_circle_geometric_quotient(self_1344: Scalar, other_1128: Circle) -> Circle {
    var self_1345: Scalar;
    var other_1129: Circle;

    self_1345 = self_1344;
    other_1129 = other_1128;
    let _e4: Scalar = self_1345;
    let _e5: Circle = other_1129;
    let _e6: Circle = circle_inverse(_e5);
    let _e7: Circle = scalar_circle_geometric_product(_e4, _e6);
    return _e7;
}

fn scalar_circle_transformation(self_1346: Scalar, other_1130: Circle) -> Circle {
    var self_1347: Scalar;
    var other_1131: Circle;

    self_1347 = self_1346;
    other_1131 = other_1130;
    let _e4: Scalar = self_1347;
    let _e5: Circle = other_1131;
    let _e6: Circle = scalar_circle_geometric_product(_e4, _e5);
    let _e7: Scalar = self_1347;
    let _e8: Scalar = scalar_reversal(_e7);
    let _e9: Circle = circle_scalar_geometric_product(_e6, _e8);
    return _e9;
}

fn scalar_dipole_geometric_quotient(self_1348: Scalar, other_1132: Dipole) -> Dipole {
    var self_1349: Scalar;
    var other_1133: Dipole;

    self_1349 = self_1348;
    other_1133 = other_1132;
    let _e4: Scalar = self_1349;
    let _e5: Dipole = other_1133;
    let _e6: Dipole = dipole_inverse(_e5);
    let _e7: Dipole = scalar_dipole_geometric_product(_e4, _e6);
    return _e7;
}

fn scalar_dipole_transformation(self_1350: Scalar, other_1134: Dipole) -> Dipole {
    var self_1351: Scalar;
    var other_1135: Dipole;

    self_1351 = self_1350;
    other_1135 = other_1134;
    let _e4: Scalar = self_1351;
    let _e5: Dipole = other_1135;
    let _e6: Dipole = scalar_dipole_geometric_product(_e4, _e5);
    let _e7: Scalar = self_1351;
    let _e8: Scalar = scalar_reversal(_e7);
    let _e9: Dipole = dipole_scalar_geometric_product(_e6, _e8);
    return _e9;
}

fn scalar_flat_point_transformation(self_1352: Scalar, other_1136: FlatPoint) -> FlatPoint {
    var self_1353: Scalar;
    var other_1137: FlatPoint;

    self_1353 = self_1352;
    other_1137 = other_1136;
    let _e4: Scalar = self_1353;
    let _e5: FlatPoint = other_1137;
    let _e6: FlatPoint = scalar_flat_point_geometric_product(_e4, _e5);
    let _e7: Scalar = self_1353;
    let _e8: Scalar = scalar_reversal(_e7);
    let _e9: FlatPoint = flat_point_scalar_geometric_product(_e6, _e8);
    return _e9;
}

fn scalar_flector_transformation(self_1354: Scalar, other_1138: Flector) -> Flector {
    var self_1355: Scalar;
    var other_1139: Flector;

    self_1355 = self_1354;
    other_1139 = other_1138;
    let _e4: Scalar = self_1355;
    let _e5: Flector = other_1139;
    let _e6: Flector = scalar_flector_geometric_product(_e4, _e5);
    let _e7: Scalar = self_1355;
    let _e8: Scalar = scalar_reversal(_e7);
    let _e9: Flector = flector_scalar_geometric_product(_e6, _e8);
    return _e9;
}

fn scalar_line_transformation(self_1356: Scalar, other_1140: Line) -> Line {
    var self_1357: Scalar;
    var other_1141: Line;

    self_1357 = self_1356;
    other_1141 = other_1140;
    let _e4: Scalar = self_1357;
    let _e5: Line = other_1141;
    let _e6: Line = scalar_line_geometric_product(_e4, _e5);
    let _e7: Scalar = self_1357;
    let _e8: Scalar = scalar_reversal(_e7);
    let _e9: Line = line_scalar_geometric_product(_e6, _e8);
    return _e9;
}

fn scalar_motor_transformation(self_1358: Scalar, other_1142: Motor) -> Motor {
    var self_1359: Scalar;
    var other_1143: Motor;

    self_1359 = self_1358;
    other_1143 = other_1142;
    let _e4: Scalar = self_1359;
    let _e5: Motor = other_1143;
    let _e6: Motor = scalar_motor_geometric_product(_e4, _e5);
    let _e7: Scalar = self_1359;
    let _e8: Scalar = scalar_reversal(_e7);
    let _e9: Motor = motor_scalar_geometric_product(_e6, _e8);
    return _e9;
}

fn scalar_multi_vector_geometric_quotient(self_1360: Scalar, other_1144: MultiVector) -> MultiVector {
    var self_1361: Scalar;
    var other_1145: MultiVector;

    self_1361 = self_1360;
    other_1145 = other_1144;
    let _e4: Scalar = self_1361;
    let _e5: MultiVector = other_1145;
    let _e6: MultiVector = multi_vector_inverse(_e5);
    let _e7: MultiVector = scalar_multi_vector_geometric_product(_e4, _e6);
    return _e7;
}

fn scalar_multi_vector_transformation(self_1362: Scalar, other_1146: MultiVector) -> MultiVector {
    var self_1363: Scalar;
    var other_1147: MultiVector;

    self_1363 = self_1362;
    other_1147 = other_1146;
    let _e4: Scalar = self_1363;
    let _e5: MultiVector = other_1147;
    let _e6: MultiVector = scalar_multi_vector_geometric_product(_e4, _e5);
    let _e7: Scalar = self_1363;
    let _e8: Scalar = scalar_reversal(_e7);
    let _e9: MultiVector = multi_vector_scalar_geometric_product(_e6, _e8);
    return _e9;
}

fn scalar_plane_transformation(self_1364: Scalar, other_1148: Plane) -> Plane {
    var self_1365: Scalar;
    var other_1149: Plane;

    self_1365 = self_1364;
    other_1149 = other_1148;
    let _e4: Scalar = self_1365;
    let _e5: Plane = other_1149;
    let _e6: Plane = scalar_plane_geometric_product(_e4, _e5);
    let _e7: Scalar = self_1365;
    let _e8: Scalar = scalar_reversal(_e7);
    let _e9: Plane = plane_scalar_geometric_product(_e6, _e8);
    return _e9;
}

fn scalar_radial_point_geometric_quotient(self_1366: Scalar, other_1150: RadialPoint) -> RadialPoint {
    var self_1367: Scalar;
    var other_1151: RadialPoint;

    self_1367 = self_1366;
    other_1151 = other_1150;
    let _e4: Scalar = self_1367;
    let _e5: RadialPoint = other_1151;
    let _e6: RadialPoint = radial_point_inverse(_e5);
    let _e7: RadialPoint = scalar_radial_point_geometric_product(_e4, _e6);
    return _e7;
}

fn scalar_radial_point_transformation(self_1368: Scalar, other_1152: RadialPoint) -> RadialPoint {
    var self_1369: Scalar;
    var other_1153: RadialPoint;

    self_1369 = self_1368;
    other_1153 = other_1152;
    let _e4: Scalar = self_1369;
    let _e5: RadialPoint = other_1153;
    let _e6: RadialPoint = scalar_radial_point_geometric_product(_e4, _e5);
    let _e7: Scalar = self_1369;
    let _e8: Scalar = scalar_reversal(_e7);
    let _e9: RadialPoint = radial_point_scalar_geometric_product(_e6, _e8);
    return _e9;
}

fn scalar_rotor_transformation(self_1370: Scalar, other_1154: Rotor) -> Rotor {
    var self_1371: Scalar;
    var other_1155: Rotor;

    self_1371 = self_1370;
    other_1155 = other_1154;
    let _e4: Scalar = self_1371;
    let _e5: Rotor = other_1155;
    let _e6: Rotor = scalar_rotor_geometric_product(_e4, _e5);
    let _e7: Scalar = self_1371;
    let _e8: Scalar = scalar_reversal(_e7);
    let _e9: Rotor = rotor_scalar_geometric_product(_e6, _e8);
    return _e9;
}

fn scalar_powi(self_1372: Scalar, exponent_2: i32) -> Scalar {
    var self_1373: Scalar;
    var exponent_3: i32;
    var local_1: Scalar;
    var x_1: Scalar;
    var y_1: Scalar;
    var n_1: i32;

    self_1373 = self_1372;
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
        let _e11: Scalar = self_1373;
        let _e12: Scalar = scalar_inverse(_e11);
        local_1 = _e12;
    } else {
        let _e14: Scalar = self_1373;
        local_1 = _e14;
    }
    let _e15: Scalar = local_1;
    x_1 = _e15;
    let _e17: Scalar = scalar_one();
    y_1 = _e17;
    let _e19: i32 = exponent_3;
    n_1 = abs(_e19);
    loop {
        let _e23: i32 = n_1;
        if !((1 < _e23)) {
            break;
        }
        {
            let _e26: i32 = n_1;
            if ((_e26 & 1) == 1) {
                {
                    let _e31: Scalar = x_1;
                    let _e32: Scalar = y_1;
                    let _e33: Scalar = scalar_scalar_geometric_product(_e31, _e32);
                    y_1 = _e33;
                }
            }
            let _e34: Scalar = x_1;
            let _e35: Scalar = x_1;
            let _e36: Scalar = scalar_scalar_geometric_product(_e34, _e35);
            x_1 = _e36;
            let _e37: i32 = n_1;
            n_1 = (_e37 >> u32(1));
        }
    }
    let _e41: Scalar = x_1;
    let _e42: Scalar = y_1;
    let _e43: Scalar = scalar_scalar_geometric_product(_e41, _e42);
    return _e43;
}

fn scalar_scalar_geometric_quotient(self_1374: Scalar, other_1156: Scalar) -> Scalar {
    var self_1375: Scalar;
    var other_1157: Scalar;

    self_1375 = self_1374;
    other_1157 = other_1156;
    let _e4: Scalar = self_1375;
    let _e5: Scalar = other_1157;
    let _e6: Scalar = scalar_inverse(_e5);
    let _e7: Scalar = scalar_scalar_geometric_product(_e4, _e6);
    return _e7;
}

fn scalar_scalar_transformation(self_1376: Scalar, other_1158: Scalar) -> Scalar {
    var self_1377: Scalar;
    var other_1159: Scalar;

    self_1377 = self_1376;
    other_1159 = other_1158;
    let _e4: Scalar = self_1377;
    let _e5: Scalar = other_1159;
    let _e6: Scalar = scalar_scalar_geometric_product(_e4, _e5);
    let _e7: Scalar = self_1377;
    let _e8: Scalar = scalar_reversal(_e7);
    let _e9: Scalar = scalar_scalar_geometric_product(_e6, _e8);
    return _e9;
}

fn scalar_sphere_transformation(self_1378: Scalar, other_1160: Sphere) -> Sphere {
    var self_1379: Scalar;
    var other_1161: Sphere;

    self_1379 = self_1378;
    other_1161 = other_1160;
    let _e4: Scalar = self_1379;
    let _e5: Sphere = other_1161;
    let _e6: Sphere = scalar_sphere_geometric_product(_e4, _e5);
    let _e7: Scalar = self_1379;
    let _e8: Scalar = scalar_reversal(_e7);
    let _e9: Sphere = sphere_scalar_geometric_product(_e6, _e8);
    return _e9;
}

fn scalar_translator_transformation(self_1380: Scalar, other_1162: Translator) -> Translator {
    var self_1381: Scalar;
    var other_1163: Translator;

    self_1381 = self_1380;
    other_1163 = other_1162;
    let _e4: Scalar = self_1381;
    let _e5: Translator = other_1163;
    let _e6: Translator = scalar_translator_geometric_product(_e4, _e5);
    let _e7: Scalar = self_1381;
    let _e8: Scalar = scalar_reversal(_e7);
    let _e9: Translator = translator_scalar_geometric_product(_e6, _e8);
    return _e9;
}

fn sphere_scalar_geometric_quotient(self_1382: Sphere, other_1164: Scalar) -> Sphere {
    var self_1383: Sphere;
    var other_1165: Scalar;

    self_1383 = self_1382;
    other_1165 = other_1164;
    let _e4: Sphere = self_1383;
    let _e5: Scalar = other_1165;
    let _e6: Scalar = scalar_inverse(_e5);
    let _e7: Sphere = sphere_scalar_geometric_product(_e4, _e6);
    return _e7;
}

fn translator_dipole_geometric_quotient(self_1384: Translator, other_1166: Dipole) -> Motor {
    var self_1385: Translator;
    var other_1167: Dipole;

    self_1385 = self_1384;
    other_1167 = other_1166;
    let _e4: Translator = self_1385;
    let _e5: Dipole = other_1167;
    let _e6: Dipole = dipole_inverse(_e5);
    let _e7: Motor = translator_dipole_geometric_product(_e4, _e6);
    return _e7;
}

fn translator_scalar_geometric_quotient(self_1386: Translator, other_1168: Scalar) -> Translator {
    var self_1387: Translator;
    var other_1169: Scalar;

    self_1387 = self_1386;
    other_1169 = other_1168;
    let _e4: Translator = self_1387;
    let _e5: Scalar = other_1169;
    let _e6: Scalar = scalar_inverse(_e5);
    let _e7: Translator = translator_scalar_geometric_product(_e4, _e6);
    return _e7;
}


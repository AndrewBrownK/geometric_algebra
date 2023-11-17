struct Scalar {
    g0_: f32,
}

struct AntiScalar {
    g0_: f32,
}

struct HomogeneousMagnitude {
    g0_: vec2<f32>,
}

struct Point {
    g0_: vec4<f32>,
}

struct Line {
    g0_: vec3<f32>,
    g1_: vec3<f32>,
}

struct Plane {
    g0_: vec4<f32>,
}

struct Motor {
    g0_: vec4<f32>,
    g1_: vec3<f32>,
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
    g1_: vec4<f32>,
    g2_: vec3<f32>,
    g3_: vec3<f32>,
    g4_: vec3<f32>,
    g5_: vec3<f32>,
    g6_: vec4<f32>,
    g7_: vec4<f32>,
}

fn scalar_zero() -> Scalar {
    return Scalar(0.0);
}

fn scalar_one() -> Scalar {
    return Scalar(1.0);
}

fn scalar_grade(self_: Scalar) -> i32 {
    return 0;
}

fn scalar_anti_grade(self_1: Scalar) -> i32 {
    return 4;
}

fn scalar_neg(self_2: Scalar) -> Scalar {
    var self_3: Scalar;

    self_3 = self_2;
    let _e2: Scalar = self_3;
    return Scalar((_e2.g0_ * -(1.0)));
}

fn scalar_automorphism(self_4: Scalar) -> Scalar {
    var self_5: Scalar;

    self_5 = self_4;
    let _e2: Scalar = self_5;
    return Scalar(_e2.g0_);
}

fn scalar_reversal(self_6: Scalar) -> Scalar {
    var self_7: Scalar;

    self_7 = self_6;
    let _e2: Scalar = self_7;
    return Scalar(_e2.g0_);
}

fn scalar_conjugation(self_8: Scalar) -> Scalar {
    var self_9: Scalar;

    self_9 = self_8;
    let _e2: Scalar = self_9;
    return Scalar(_e2.g0_);
}

fn scalar_dual(self_10: Scalar) -> AntiScalar {
    var self_11: Scalar;

    self_11 = self_10;
    let _e2: Scalar = self_11;
    return AntiScalar(_e2.g0_);
}

fn scalar_anti_reversal(self_12: Scalar) -> Scalar {
    var self_13: Scalar;

    self_13 = self_12;
    let _e2: Scalar = self_13;
    return Scalar(_e2.g0_);
}

fn scalar_right_complement(self_14: Scalar) -> AntiScalar {
    var self_15: Scalar;

    self_15 = self_14;
    let _e2: Scalar = self_15;
    return AntiScalar(_e2.g0_);
}

fn scalar_left_complement(self_16: Scalar) -> AntiScalar {
    var self_17: Scalar;

    self_17 = self_16;
    let _e2: Scalar = self_17;
    return AntiScalar(_e2.g0_);
}

fn scalar_double_complement(self_18: Scalar) -> Scalar {
    var self_19: Scalar;

    self_19 = self_18;
    let _e2: Scalar = self_19;
    return Scalar(_e2.g0_);
}

fn scalar_scalar_add(self_20: Scalar, other: Scalar) -> Scalar {
    var self_21: Scalar;
    var other_1: Scalar;

    self_21 = self_20;
    other_1 = other;
    let _e4: Scalar = self_21;
    let _e6: Scalar = other_1;
    return Scalar((_e4.g0_ + _e6.g0_));
}

fn scalar_scalar_sub(self_22: Scalar, other_2: Scalar) -> Scalar {
    var self_23: Scalar;
    var other_3: Scalar;

    self_23 = self_22;
    other_3 = other_2;
    let _e4: Scalar = self_23;
    let _e6: Scalar = other_3;
    return Scalar((_e4.g0_ - _e6.g0_));
}

fn scalar_scalar_mul(self_24: Scalar, other_4: Scalar) -> Scalar {
    var self_25: Scalar;
    var other_5: Scalar;

    self_25 = self_24;
    other_5 = other_4;
    let _e4: Scalar = self_25;
    let _e6: Scalar = other_5;
    return Scalar((_e4.g0_ * _e6.g0_));
}

fn scalar_scalar_div(self_26: Scalar, other_6: Scalar) -> Scalar {
    var self_27: Scalar;
    var other_7: Scalar;

    self_27 = self_26;
    other_7 = other_6;
    let _e4: Scalar = self_27;
    let _e8: Scalar = other_7;
    return Scalar((((_e4.g0_ * 1.0) / _e8.g0_) * 1.0));
}

fn scalar_scalar_geometric_product(self_28: Scalar, other_8: Scalar) -> Scalar {
    var self_29: Scalar;
    var other_9: Scalar;

    self_29 = self_28;
    other_9 = other_8;
    let _e4: Scalar = self_29;
    let _e6: Scalar = other_9;
    return Scalar((_e4.g0_ * _e6.g0_));
}

fn scalar_scalar_outer_product(self_30: Scalar, other_10: Scalar) -> Scalar {
    var self_31: Scalar;
    var other_11: Scalar;

    self_31 = self_30;
    other_11 = other_10;
    let _e4: Scalar = self_31;
    let _e6: Scalar = other_11;
    return Scalar((_e4.g0_ * _e6.g0_));
}

fn scalar_scalar_inner_product(self_32: Scalar, other_12: Scalar) -> Scalar {
    var self_33: Scalar;
    var other_13: Scalar;

    self_33 = self_32;
    other_13 = other_12;
    let _e4: Scalar = self_33;
    let _e6: Scalar = other_13;
    return Scalar((_e4.g0_ * _e6.g0_));
}

fn scalar_scalar_left_contraction(self_34: Scalar, other_14: Scalar) -> Scalar {
    var self_35: Scalar;
    var other_15: Scalar;

    self_35 = self_34;
    other_15 = other_14;
    let _e4: Scalar = self_35;
    let _e6: Scalar = other_15;
    return Scalar((_e4.g0_ * _e6.g0_));
}

fn scalar_scalar_right_contraction(self_36: Scalar, other_16: Scalar) -> Scalar {
    var self_37: Scalar;
    var other_17: Scalar;

    self_37 = self_36;
    other_17 = other_16;
    let _e4: Scalar = self_37;
    let _e6: Scalar = other_17;
    return Scalar((_e4.g0_ * _e6.g0_));
}

fn scalar_scalar_scalar_product(self_38: Scalar, other_18: Scalar) -> Scalar {
    var self_39: Scalar;
    var other_19: Scalar;

    self_39 = self_38;
    other_19 = other_18;
    let _e4: Scalar = self_39;
    let _e6: Scalar = other_19;
    return Scalar((_e4.g0_ * _e6.g0_));
}

fn scalar_anti_scalar_add(self_40: Scalar, other_20: AntiScalar) -> HomogeneousMagnitude {
    var self_41: Scalar;
    var other_21: AntiScalar;

    self_41 = self_40;
    other_21 = other_20;
    let _e4: Scalar = self_41;
    let _e11: AntiScalar = other_21;
    return HomogeneousMagnitude(((vec2<f32>(_e4.g0_) * vec2<f32>(1.0, 0.0)) + (vec2<f32>(_e11.g0_) * vec2<f32>(0.0, 1.0))));
}

fn scalar_anti_scalar_sub(self_42: Scalar, other_22: AntiScalar) -> HomogeneousMagnitude {
    var self_43: Scalar;
    var other_23: AntiScalar;

    self_43 = self_42;
    other_23 = other_22;
    let _e4: Scalar = self_43;
    let _e11: AntiScalar = other_23;
    return HomogeneousMagnitude(((vec2<f32>(_e4.g0_) * vec2<f32>(1.0, 0.0)) - (vec2<f32>(_e11.g0_) * vec2<f32>(0.0, 1.0))));
}

fn scalar_anti_scalar_geometric_product(self_44: Scalar, other_24: AntiScalar) -> AntiScalar {
    var self_45: Scalar;
    var other_25: AntiScalar;

    self_45 = self_44;
    other_25 = other_24;
    let _e4: Scalar = self_45;
    let _e6: AntiScalar = other_25;
    return AntiScalar((_e4.g0_ * _e6.g0_));
}

fn scalar_anti_scalar_regressive_product(self_46: Scalar, other_26: AntiScalar) -> Scalar {
    var self_47: Scalar;
    var other_27: AntiScalar;

    self_47 = self_46;
    other_27 = other_26;
    let _e4: Scalar = self_47;
    let _e6: AntiScalar = other_27;
    return Scalar((_e4.g0_ * _e6.g0_));
}

fn scalar_anti_scalar_outer_product(self_48: Scalar, other_28: AntiScalar) -> AntiScalar {
    var self_49: Scalar;
    var other_29: AntiScalar;

    self_49 = self_48;
    other_29 = other_28;
    let _e4: Scalar = self_49;
    let _e6: AntiScalar = other_29;
    return AntiScalar((_e4.g0_ * _e6.g0_));
}

fn scalar_anti_scalar_inner_product(self_50: Scalar, other_30: AntiScalar) -> AntiScalar {
    var self_51: Scalar;
    var other_31: AntiScalar;

    self_51 = self_50;
    other_31 = other_30;
    let _e4: Scalar = self_51;
    let _e6: AntiScalar = other_31;
    return AntiScalar((_e4.g0_ * _e6.g0_));
}

fn scalar_anti_scalar_geometric_anti_product(self_52: Scalar, other_32: AntiScalar) -> Scalar {
    var self_53: Scalar;
    var other_33: AntiScalar;

    self_53 = self_52;
    other_33 = other_32;
    let _e4: Scalar = self_53;
    let _e6: AntiScalar = other_33;
    return Scalar((_e4.g0_ * _e6.g0_));
}

fn scalar_anti_scalar_inner_anti_product(self_54: Scalar, other_34: AntiScalar) -> Scalar {
    var self_55: Scalar;
    var other_35: AntiScalar;

    self_55 = self_54;
    other_35 = other_34;
    let _e4: Scalar = self_55;
    let _e6: AntiScalar = other_35;
    return Scalar((_e4.g0_ * _e6.g0_));
}

fn scalar_anti_scalar_left_contraction(self_56: Scalar, other_36: AntiScalar) -> AntiScalar {
    var self_57: Scalar;
    var other_37: AntiScalar;

    self_57 = self_56;
    other_37 = other_36;
    let _e4: Scalar = self_57;
    let _e6: AntiScalar = other_37;
    return AntiScalar((_e4.g0_ * _e6.g0_));
}

fn scalar_anti_scalar_right_anti_contraction(self_58: Scalar, other_38: AntiScalar) -> Scalar {
    var self_59: Scalar;
    var other_39: AntiScalar;

    self_59 = self_58;
    other_39 = other_38;
    let _e4: Scalar = self_59;
    let _e6: AntiScalar = other_39;
    return Scalar((_e4.g0_ * _e6.g0_));
}

fn scalar_homogeneous_magnitude_add(self_60: Scalar, other_40: HomogeneousMagnitude) -> HomogeneousMagnitude {
    var self_61: Scalar;
    var other_41: HomogeneousMagnitude;

    self_61 = self_60;
    other_41 = other_40;
    let _e4: Scalar = self_61;
    let _e11: HomogeneousMagnitude = other_41;
    return HomogeneousMagnitude(((vec2<f32>(_e4.g0_) * vec2<f32>(1.0, 0.0)) + _e11.g0_));
}

fn scalar_homogeneous_magnitude_sub(self_62: Scalar, other_42: HomogeneousMagnitude) -> HomogeneousMagnitude {
    var self_63: Scalar;
    var other_43: HomogeneousMagnitude;

    self_63 = self_62;
    other_43 = other_42;
    let _e4: Scalar = self_63;
    let _e11: HomogeneousMagnitude = other_43;
    return HomogeneousMagnitude(((vec2<f32>(_e4.g0_) * vec2<f32>(1.0, 0.0)) - _e11.g0_));
}

fn scalar_homogeneous_magnitude_geometric_product(self_64: Scalar, other_44: HomogeneousMagnitude) -> HomogeneousMagnitude {
    var self_65: Scalar;
    var other_45: HomogeneousMagnitude;

    self_65 = self_64;
    other_45 = other_44;
    let _e4: Scalar = self_65;
    let _e7: HomogeneousMagnitude = other_45;
    return HomogeneousMagnitude((vec2<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_homogeneous_magnitude_regressive_product(self_66: Scalar, other_46: HomogeneousMagnitude) -> Scalar {
    var self_67: Scalar;
    var other_47: HomogeneousMagnitude;

    self_67 = self_66;
    other_47 = other_46;
    let _e4: Scalar = self_67;
    let _e6: HomogeneousMagnitude = other_47;
    return Scalar((_e4.g0_ * _e6.g0_.y));
}

fn scalar_homogeneous_magnitude_outer_product(self_68: Scalar, other_48: HomogeneousMagnitude) -> HomogeneousMagnitude {
    var self_69: Scalar;
    var other_49: HomogeneousMagnitude;

    self_69 = self_68;
    other_49 = other_48;
    let _e4: Scalar = self_69;
    let _e7: HomogeneousMagnitude = other_49;
    return HomogeneousMagnitude((vec2<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_homogeneous_magnitude_inner_product(self_70: Scalar, other_50: HomogeneousMagnitude) -> HomogeneousMagnitude {
    var self_71: Scalar;
    var other_51: HomogeneousMagnitude;

    self_71 = self_70;
    other_51 = other_50;
    let _e4: Scalar = self_71;
    let _e7: HomogeneousMagnitude = other_51;
    return HomogeneousMagnitude((vec2<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_homogeneous_magnitude_geometric_anti_product(self_72: Scalar, other_52: HomogeneousMagnitude) -> Scalar {
    var self_73: Scalar;
    var other_53: HomogeneousMagnitude;

    self_73 = self_72;
    other_53 = other_52;
    let _e4: Scalar = self_73;
    let _e6: HomogeneousMagnitude = other_53;
    return Scalar((_e4.g0_ * _e6.g0_.y));
}

fn scalar_homogeneous_magnitude_inner_anti_product(self_74: Scalar, other_54: HomogeneousMagnitude) -> Scalar {
    var self_75: Scalar;
    var other_55: HomogeneousMagnitude;

    self_75 = self_74;
    other_55 = other_54;
    let _e4: Scalar = self_75;
    let _e6: HomogeneousMagnitude = other_55;
    return Scalar((_e4.g0_ * _e6.g0_.y));
}

fn scalar_homogeneous_magnitude_left_contraction(self_76: Scalar, other_56: HomogeneousMagnitude) -> HomogeneousMagnitude {
    var self_77: Scalar;
    var other_57: HomogeneousMagnitude;

    self_77 = self_76;
    other_57 = other_56;
    let _e4: Scalar = self_77;
    let _e7: HomogeneousMagnitude = other_57;
    return HomogeneousMagnitude((vec2<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_homogeneous_magnitude_right_contraction(self_78: Scalar, other_58: HomogeneousMagnitude) -> Scalar {
    var self_79: Scalar;
    var other_59: HomogeneousMagnitude;

    self_79 = self_78;
    other_59 = other_58;
    let _e4: Scalar = self_79;
    let _e6: HomogeneousMagnitude = other_59;
    return Scalar((_e4.g0_ * _e6.g0_.x));
}

fn scalar_homogeneous_magnitude_right_anti_contraction(self_80: Scalar, other_60: HomogeneousMagnitude) -> Scalar {
    var self_81: Scalar;
    var other_61: HomogeneousMagnitude;

    self_81 = self_80;
    other_61 = other_60;
    let _e4: Scalar = self_81;
    let _e6: HomogeneousMagnitude = other_61;
    return Scalar((_e4.g0_ * _e6.g0_.y));
}

fn scalar_homogeneous_magnitude_scalar_product(self_82: Scalar, other_62: HomogeneousMagnitude) -> Scalar {
    var self_83: Scalar;
    var other_63: HomogeneousMagnitude;

    self_83 = self_82;
    other_63 = other_62;
    let _e4: Scalar = self_83;
    let _e6: HomogeneousMagnitude = other_63;
    return Scalar((_e4.g0_ * _e6.g0_.x));
}

fn scalar_point_geometric_product(self_84: Scalar, other_64: Point) -> Point {
    var self_85: Scalar;
    var other_65: Point;

    self_85 = self_84;
    other_65 = other_64;
    let _e4: Scalar = self_85;
    let _e7: Point = other_65;
    return Point((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_point_outer_product(self_86: Scalar, other_66: Point) -> Point {
    var self_87: Scalar;
    var other_67: Point;

    self_87 = self_86;
    other_67 = other_66;
    let _e4: Scalar = self_87;
    let _e7: Point = other_67;
    return Point((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_point_inner_product(self_88: Scalar, other_68: Point) -> Point {
    var self_89: Scalar;
    var other_69: Point;

    self_89 = self_88;
    other_69 = other_68;
    let _e4: Scalar = self_89;
    let _e7: Point = other_69;
    return Point((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_point_left_contraction(self_90: Scalar, other_70: Point) -> Point {
    var self_91: Scalar;
    var other_71: Point;

    self_91 = self_90;
    other_71 = other_70;
    let _e4: Scalar = self_91;
    let _e7: Point = other_71;
    return Point((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_line_geometric_product(self_92: Scalar, other_72: Line) -> Line {
    var self_93: Scalar;
    var other_73: Line;

    self_93 = self_92;
    other_73 = other_72;
    let _e4: Scalar = self_93;
    let _e7: Line = other_73;
    let _e10: Scalar = self_93;
    let _e13: Line = other_73;
    return Line((vec3<f32>(_e4.g0_) * _e7.g0_), (vec3<f32>(_e10.g0_) * _e13.g1_));
}

fn scalar_line_outer_product(self_94: Scalar, other_74: Line) -> Line {
    var self_95: Scalar;
    var other_75: Line;

    self_95 = self_94;
    other_75 = other_74;
    let _e4: Scalar = self_95;
    let _e7: Line = other_75;
    let _e10: Scalar = self_95;
    let _e13: Line = other_75;
    return Line((vec3<f32>(_e4.g0_) * _e7.g0_), (vec3<f32>(_e10.g0_) * _e13.g1_));
}

fn scalar_line_inner_product(self_96: Scalar, other_76: Line) -> Line {
    var self_97: Scalar;
    var other_77: Line;

    self_97 = self_96;
    other_77 = other_76;
    let _e4: Scalar = self_97;
    let _e7: Line = other_77;
    let _e10: Scalar = self_97;
    let _e13: Line = other_77;
    return Line((vec3<f32>(_e4.g0_) * _e7.g0_), (vec3<f32>(_e10.g0_) * _e13.g1_));
}

fn scalar_line_left_contraction(self_98: Scalar, other_78: Line) -> Line {
    var self_99: Scalar;
    var other_79: Line;

    self_99 = self_98;
    other_79 = other_78;
    let _e4: Scalar = self_99;
    let _e7: Line = other_79;
    let _e10: Scalar = self_99;
    let _e13: Line = other_79;
    return Line((vec3<f32>(_e4.g0_) * _e7.g0_), (vec3<f32>(_e10.g0_) * _e13.g1_));
}

fn scalar_plane_geometric_product(self_100: Scalar, other_80: Plane) -> Plane {
    var self_101: Scalar;
    var other_81: Plane;

    self_101 = self_100;
    other_81 = other_80;
    let _e4: Scalar = self_101;
    let _e7: Plane = other_81;
    return Plane((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_plane_outer_product(self_102: Scalar, other_82: Plane) -> Plane {
    var self_103: Scalar;
    var other_83: Plane;

    self_103 = self_102;
    other_83 = other_82;
    let _e4: Scalar = self_103;
    let _e7: Plane = other_83;
    return Plane((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_plane_inner_product(self_104: Scalar, other_84: Plane) -> Plane {
    var self_105: Scalar;
    var other_85: Plane;

    self_105 = self_104;
    other_85 = other_84;
    let _e4: Scalar = self_105;
    let _e7: Plane = other_85;
    return Plane((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_plane_left_contraction(self_106: Scalar, other_86: Plane) -> Plane {
    var self_107: Scalar;
    var other_87: Plane;

    self_107 = self_106;
    other_87 = other_86;
    let _e4: Scalar = self_107;
    let _e7: Plane = other_87;
    return Plane((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_motor_geometric_product(self_108: Scalar, other_88: Motor) -> Motor {
    var self_109: Scalar;
    var other_89: Motor;

    self_109 = self_108;
    other_89 = other_88;
    let _e4: Scalar = self_109;
    let _e7: Motor = other_89;
    let _e10: Scalar = self_109;
    let _e13: Motor = other_89;
    return Motor((vec4<f32>(_e4.g0_) * _e7.g0_), (vec3<f32>(_e10.g0_) * _e13.g1_));
}

fn scalar_motor_regressive_product(self_110: Scalar, other_90: Motor) -> Scalar {
    var self_111: Scalar;
    var other_91: Motor;

    self_111 = self_110;
    other_91 = other_90;
    let _e4: Scalar = self_111;
    let _e6: Motor = other_91;
    return Scalar((_e4.g0_ * _e6.g0_.w));
}

fn scalar_motor_outer_product(self_112: Scalar, other_92: Motor) -> Motor {
    var self_113: Scalar;
    var other_93: Motor;

    self_113 = self_112;
    other_93 = other_92;
    let _e4: Scalar = self_113;
    let _e7: Motor = other_93;
    let _e10: Scalar = self_113;
    let _e13: Motor = other_93;
    return Motor((vec4<f32>(_e4.g0_) * _e7.g0_), (vec3<f32>(_e10.g0_) * _e13.g1_));
}

fn scalar_motor_inner_product(self_114: Scalar, other_94: Motor) -> Motor {
    var self_115: Scalar;
    var other_95: Motor;

    self_115 = self_114;
    other_95 = other_94;
    let _e4: Scalar = self_115;
    let _e7: Motor = other_95;
    let _e10: Scalar = self_115;
    let _e13: Motor = other_95;
    return Motor((vec4<f32>(_e4.g0_) * _e7.g0_), (vec3<f32>(_e10.g0_) * _e13.g1_));
}

fn scalar_motor_left_contraction(self_116: Scalar, other_96: Motor) -> Motor {
    var self_117: Scalar;
    var other_97: Motor;

    self_117 = self_116;
    other_97 = other_96;
    let _e4: Scalar = self_117;
    let _e7: Motor = other_97;
    let _e10: Scalar = self_117;
    let _e13: Motor = other_97;
    return Motor((vec4<f32>(_e4.g0_) * _e7.g0_), (vec3<f32>(_e10.g0_) * _e13.g1_));
}

fn scalar_rotor_geometric_product(self_118: Scalar, other_98: Rotor) -> Rotor {
    var self_119: Scalar;
    var other_99: Rotor;

    self_119 = self_118;
    other_99 = other_98;
    let _e4: Scalar = self_119;
    let _e7: Rotor = other_99;
    return Rotor((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_rotor_regressive_product(self_120: Scalar, other_100: Rotor) -> Scalar {
    var self_121: Scalar;
    var other_101: Rotor;

    self_121 = self_120;
    other_101 = other_100;
    let _e4: Scalar = self_121;
    let _e6: Rotor = other_101;
    return Scalar((_e4.g0_ * _e6.g0_.w));
}

fn scalar_rotor_outer_product(self_122: Scalar, other_102: Rotor) -> Rotor {
    var self_123: Scalar;
    var other_103: Rotor;

    self_123 = self_122;
    other_103 = other_102;
    let _e4: Scalar = self_123;
    let _e7: Rotor = other_103;
    return Rotor((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_rotor_inner_product(self_124: Scalar, other_104: Rotor) -> Rotor {
    var self_125: Scalar;
    var other_105: Rotor;

    self_125 = self_124;
    other_105 = other_104;
    let _e4: Scalar = self_125;
    let _e7: Rotor = other_105;
    return Rotor((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_rotor_left_contraction(self_126: Scalar, other_106: Rotor) -> Rotor {
    var self_127: Scalar;
    var other_107: Rotor;

    self_127 = self_126;
    other_107 = other_106;
    let _e4: Scalar = self_127;
    let _e7: Rotor = other_107;
    return Rotor((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_translator_geometric_product(self_128: Scalar, other_108: Translator) -> Translator {
    var self_129: Scalar;
    var other_109: Translator;

    self_129 = self_128;
    other_109 = other_108;
    let _e4: Scalar = self_129;
    let _e7: Translator = other_109;
    return Translator((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_translator_regressive_product(self_130: Scalar, other_110: Translator) -> Scalar {
    var self_131: Scalar;
    var other_111: Translator;

    self_131 = self_130;
    other_111 = other_110;
    let _e4: Scalar = self_131;
    let _e6: Translator = other_111;
    return Scalar((_e4.g0_ * _e6.g0_.w));
}

fn scalar_translator_outer_product(self_132: Scalar, other_112: Translator) -> Translator {
    var self_133: Scalar;
    var other_113: Translator;

    self_133 = self_132;
    other_113 = other_112;
    let _e4: Scalar = self_133;
    let _e7: Translator = other_113;
    return Translator((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_translator_inner_product(self_134: Scalar, other_114: Translator) -> Translator {
    var self_135: Scalar;
    var other_115: Translator;

    self_135 = self_134;
    other_115 = other_114;
    let _e4: Scalar = self_135;
    let _e7: Translator = other_115;
    return Translator((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_translator_geometric_anti_product(self_136: Scalar, other_116: Translator) -> Scalar {
    var self_137: Scalar;
    var other_117: Translator;

    self_137 = self_136;
    other_117 = other_116;
    let _e4: Scalar = self_137;
    let _e6: Translator = other_117;
    return Scalar((_e4.g0_ * _e6.g0_.w));
}

fn scalar_translator_inner_anti_product(self_138: Scalar, other_118: Translator) -> Scalar {
    var self_139: Scalar;
    var other_119: Translator;

    self_139 = self_138;
    other_119 = other_118;
    let _e4: Scalar = self_139;
    let _e6: Translator = other_119;
    return Scalar((_e4.g0_ * _e6.g0_.w));
}

fn scalar_translator_left_contraction(self_140: Scalar, other_120: Translator) -> Translator {
    var self_141: Scalar;
    var other_121: Translator;

    self_141 = self_140;
    other_121 = other_120;
    let _e4: Scalar = self_141;
    let _e7: Translator = other_121;
    return Translator((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_translator_right_anti_contraction(self_142: Scalar, other_122: Translator) -> Scalar {
    var self_143: Scalar;
    var other_123: Translator;

    self_143 = self_142;
    other_123 = other_122;
    let _e4: Scalar = self_143;
    let _e6: Translator = other_123;
    return Scalar((_e4.g0_ * _e6.g0_.w));
}

fn scalar_flector_geometric_product(self_144: Scalar, other_124: Flector) -> Flector {
    var self_145: Scalar;
    var other_125: Flector;

    self_145 = self_144;
    other_125 = other_124;
    let _e4: Scalar = self_145;
    let _e7: Flector = other_125;
    let _e10: Scalar = self_145;
    let _e13: Flector = other_125;
    return Flector((vec4<f32>(_e4.g0_) * _e7.g0_), (vec4<f32>(_e10.g0_) * _e13.g1_));
}

fn scalar_flector_outer_product(self_146: Scalar, other_126: Flector) -> Flector {
    var self_147: Scalar;
    var other_127: Flector;

    self_147 = self_146;
    other_127 = other_126;
    let _e4: Scalar = self_147;
    let _e7: Flector = other_127;
    let _e10: Scalar = self_147;
    let _e13: Flector = other_127;
    return Flector((vec4<f32>(_e4.g0_) * _e7.g0_), (vec4<f32>(_e10.g0_) * _e13.g1_));
}

fn scalar_flector_inner_product(self_148: Scalar, other_128: Flector) -> Flector {
    var self_149: Scalar;
    var other_129: Flector;

    self_149 = self_148;
    other_129 = other_128;
    let _e4: Scalar = self_149;
    let _e7: Flector = other_129;
    let _e10: Scalar = self_149;
    let _e13: Flector = other_129;
    return Flector((vec4<f32>(_e4.g0_) * _e7.g0_), (vec4<f32>(_e10.g0_) * _e13.g1_));
}

fn scalar_flector_left_contraction(self_150: Scalar, other_130: Flector) -> Flector {
    var self_151: Scalar;
    var other_131: Flector;

    self_151 = self_150;
    other_131 = other_130;
    let _e4: Scalar = self_151;
    let _e7: Flector = other_131;
    let _e10: Scalar = self_151;
    let _e13: Flector = other_131;
    return Flector((vec4<f32>(_e4.g0_) * _e7.g0_), (vec4<f32>(_e10.g0_) * _e13.g1_));
}

fn scalar_multi_vector_regressive_product(self_152: Scalar, other_132: MultiVector) -> Scalar {
    var self_153: Scalar;
    var other_133: MultiVector;

    self_153 = self_152;
    other_133 = other_132;
    let _e4: Scalar = self_153;
    let _e6: MultiVector = other_133;
    return Scalar((_e4.g0_ * _e6.g0_.y));
}

fn scalar_multi_vector_right_contraction(self_154: Scalar, other_134: MultiVector) -> Scalar {
    var self_155: Scalar;
    var other_135: MultiVector;

    self_155 = self_154;
    other_135 = other_134;
    let _e4: Scalar = self_155;
    let _e6: MultiVector = other_135;
    return Scalar((_e4.g0_ * _e6.g0_.x));
}

fn scalar_multi_vector_scalar_product(self_156: Scalar, other_136: MultiVector) -> Scalar {
    var self_157: Scalar;
    var other_137: MultiVector;

    self_157 = self_156;
    other_137 = other_136;
    let _e4: Scalar = self_157;
    let _e6: MultiVector = other_137;
    return Scalar((_e4.g0_ * _e6.g0_.x));
}

fn scalar_squared_magnitude(self_158: Scalar) -> Scalar {
    var self_159: Scalar;

    self_159 = self_158;
    let _e2: Scalar = self_159;
    let _e3: Scalar = self_159;
    let _e4: Scalar = scalar_reversal(_e3);
    let _e5: Scalar = scalar_scalar_scalar_product(_e2, _e4);
    return _e5;
}

fn scalar_magnitude(self_160: Scalar) -> Scalar {
    var self_161: Scalar;

    self_161 = self_160;
    let _e2: Scalar = self_161;
    let _e3: Scalar = scalar_squared_magnitude(_e2);
    return Scalar(sqrt(_e3.g0_));
}

fn scalar_scale(self_162: Scalar, other_138: f32) -> Scalar {
    var self_163: Scalar;
    var other_139: f32;

    self_163 = self_162;
    other_139 = other_138;
    let _e4: Scalar = self_163;
    let _e5: f32 = other_139;
    let _e7: Scalar = scalar_scalar_geometric_product(_e4, Scalar(_e5));
    return _e7;
}

fn scalar_signum(self_164: Scalar) -> Scalar {
    var self_165: Scalar;

    self_165 = self_164;
    let _e2: Scalar = self_165;
    let _e3: Scalar = self_165;
    let _e4: Scalar = scalar_magnitude(_e3);
    let _e9: Scalar = scalar_scalar_geometric_product(_e2, Scalar((1.0 / _e4.g0_)));
    return _e9;
}

fn scalar_inverse(self_166: Scalar) -> Scalar {
    var self_167: Scalar;

    self_167 = self_166;
    let _e2: Scalar = self_167;
    let _e3: Scalar = scalar_reversal(_e2);
    let _e4: Scalar = self_167;
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

fn anti_scalar_grade(self_168: AntiScalar) -> i32 {
    return 4;
}

fn anti_scalar_anti_grade(self_169: AntiScalar) -> i32 {
    return 0;
}

fn anti_scalar_neg(self_170: AntiScalar) -> AntiScalar {
    var self_171: AntiScalar;

    self_171 = self_170;
    let _e2: AntiScalar = self_171;
    return AntiScalar((_e2.g0_ * -(1.0)));
}

fn anti_scalar_automorphism(self_172: AntiScalar) -> AntiScalar {
    var self_173: AntiScalar;

    self_173 = self_172;
    let _e2: AntiScalar = self_173;
    return AntiScalar(_e2.g0_);
}

fn anti_scalar_reversal(self_174: AntiScalar) -> AntiScalar {
    var self_175: AntiScalar;

    self_175 = self_174;
    let _e2: AntiScalar = self_175;
    return AntiScalar(_e2.g0_);
}

fn anti_scalar_conjugation(self_176: AntiScalar) -> AntiScalar {
    var self_177: AntiScalar;

    self_177 = self_176;
    let _e2: AntiScalar = self_177;
    return AntiScalar(_e2.g0_);
}

fn anti_scalar_dual(self_178: AntiScalar) -> Scalar {
    var self_179: AntiScalar;

    self_179 = self_178;
    let _e2: AntiScalar = self_179;
    return Scalar(_e2.g0_);
}

fn anti_scalar_anti_reversal(self_180: AntiScalar) -> AntiScalar {
    var self_181: AntiScalar;

    self_181 = self_180;
    let _e2: AntiScalar = self_181;
    return AntiScalar(_e2.g0_);
}

fn anti_scalar_right_complement(self_182: AntiScalar) -> Scalar {
    var self_183: AntiScalar;

    self_183 = self_182;
    let _e2: AntiScalar = self_183;
    return Scalar(_e2.g0_);
}

fn anti_scalar_left_complement(self_184: AntiScalar) -> Scalar {
    var self_185: AntiScalar;

    self_185 = self_184;
    let _e2: AntiScalar = self_185;
    return Scalar(_e2.g0_);
}

fn anti_scalar_double_complement(self_186: AntiScalar) -> AntiScalar {
    var self_187: AntiScalar;

    self_187 = self_186;
    let _e2: AntiScalar = self_187;
    return AntiScalar(_e2.g0_);
}

fn anti_scalar_scalar_add(self_188: AntiScalar, other_140: Scalar) -> HomogeneousMagnitude {
    var self_189: AntiScalar;
    var other_141: Scalar;

    self_189 = self_188;
    other_141 = other_140;
    let _e4: AntiScalar = self_189;
    let _e11: Scalar = other_141;
    return HomogeneousMagnitude(((vec2<f32>(_e4.g0_) * vec2<f32>(0.0, 1.0)) + (vec2<f32>(_e11.g0_) * vec2<f32>(1.0, 0.0))));
}

fn anti_scalar_scalar_sub(self_190: AntiScalar, other_142: Scalar) -> HomogeneousMagnitude {
    var self_191: AntiScalar;
    var other_143: Scalar;

    self_191 = self_190;
    other_143 = other_142;
    let _e4: AntiScalar = self_191;
    let _e11: Scalar = other_143;
    return HomogeneousMagnitude(((vec2<f32>(_e4.g0_) * vec2<f32>(0.0, 1.0)) - (vec2<f32>(_e11.g0_) * vec2<f32>(1.0, 0.0))));
}

fn anti_scalar_scalar_geometric_product(self_192: AntiScalar, other_144: Scalar) -> AntiScalar {
    var self_193: AntiScalar;
    var other_145: Scalar;

    self_193 = self_192;
    other_145 = other_144;
    let _e4: AntiScalar = self_193;
    let _e6: Scalar = other_145;
    return AntiScalar((_e4.g0_ * _e6.g0_));
}

fn anti_scalar_scalar_regressive_product(self_194: AntiScalar, other_146: Scalar) -> Scalar {
    var self_195: AntiScalar;
    var other_147: Scalar;

    self_195 = self_194;
    other_147 = other_146;
    let _e4: AntiScalar = self_195;
    let _e6: Scalar = other_147;
    return Scalar((_e4.g0_ * _e6.g0_));
}

fn anti_scalar_scalar_outer_product(self_196: AntiScalar, other_148: Scalar) -> AntiScalar {
    var self_197: AntiScalar;
    var other_149: Scalar;

    self_197 = self_196;
    other_149 = other_148;
    let _e4: AntiScalar = self_197;
    let _e6: Scalar = other_149;
    return AntiScalar((_e4.g0_ * _e6.g0_));
}

fn anti_scalar_scalar_inner_product(self_198: AntiScalar, other_150: Scalar) -> AntiScalar {
    var self_199: AntiScalar;
    var other_151: Scalar;

    self_199 = self_198;
    other_151 = other_150;
    let _e4: AntiScalar = self_199;
    let _e6: Scalar = other_151;
    return AntiScalar((_e4.g0_ * _e6.g0_));
}

fn anti_scalar_scalar_geometric_anti_product(self_200: AntiScalar, other_152: Scalar) -> Scalar {
    var self_201: AntiScalar;
    var other_153: Scalar;

    self_201 = self_200;
    other_153 = other_152;
    let _e4: AntiScalar = self_201;
    let _e6: Scalar = other_153;
    return Scalar((_e4.g0_ * _e6.g0_));
}

fn anti_scalar_scalar_inner_anti_product(self_202: AntiScalar, other_154: Scalar) -> Scalar {
    var self_203: AntiScalar;
    var other_155: Scalar;

    self_203 = self_202;
    other_155 = other_154;
    let _e4: AntiScalar = self_203;
    let _e6: Scalar = other_155;
    return Scalar((_e4.g0_ * _e6.g0_));
}

fn anti_scalar_scalar_right_contraction(self_204: AntiScalar, other_156: Scalar) -> AntiScalar {
    var self_205: AntiScalar;
    var other_157: Scalar;

    self_205 = self_204;
    other_157 = other_156;
    let _e4: AntiScalar = self_205;
    let _e6: Scalar = other_157;
    return AntiScalar((_e4.g0_ * _e6.g0_));
}

fn anti_scalar_scalar_left_anti_contraction(self_206: AntiScalar, other_158: Scalar) -> Scalar {
    var self_207: AntiScalar;
    var other_159: Scalar;

    self_207 = self_206;
    other_159 = other_158;
    let _e4: AntiScalar = self_207;
    let _e6: Scalar = other_159;
    return Scalar((_e4.g0_ * _e6.g0_));
}

fn anti_scalar_anti_scalar_add(self_208: AntiScalar, other_160: AntiScalar) -> AntiScalar {
    var self_209: AntiScalar;
    var other_161: AntiScalar;

    self_209 = self_208;
    other_161 = other_160;
    let _e4: AntiScalar = self_209;
    let _e6: AntiScalar = other_161;
    return AntiScalar((_e4.g0_ + _e6.g0_));
}

fn anti_scalar_anti_scalar_sub(self_210: AntiScalar, other_162: AntiScalar) -> AntiScalar {
    var self_211: AntiScalar;
    var other_163: AntiScalar;

    self_211 = self_210;
    other_163 = other_162;
    let _e4: AntiScalar = self_211;
    let _e6: AntiScalar = other_163;
    return AntiScalar((_e4.g0_ - _e6.g0_));
}

fn anti_scalar_anti_scalar_mul(self_212: AntiScalar, other_164: AntiScalar) -> AntiScalar {
    var self_213: AntiScalar;
    var other_165: AntiScalar;

    self_213 = self_212;
    other_165 = other_164;
    let _e4: AntiScalar = self_213;
    let _e6: AntiScalar = other_165;
    return AntiScalar((_e4.g0_ * _e6.g0_));
}

fn anti_scalar_anti_scalar_div(self_214: AntiScalar, other_166: AntiScalar) -> AntiScalar {
    var self_215: AntiScalar;
    var other_167: AntiScalar;

    self_215 = self_214;
    other_167 = other_166;
    let _e4: AntiScalar = self_215;
    let _e8: AntiScalar = other_167;
    return AntiScalar((((_e4.g0_ * 1.0) / _e8.g0_) * 1.0));
}

fn anti_scalar_anti_scalar_regressive_product(self_216: AntiScalar, other_168: AntiScalar) -> AntiScalar {
    var self_217: AntiScalar;
    var other_169: AntiScalar;

    self_217 = self_216;
    other_169 = other_168;
    let _e4: AntiScalar = self_217;
    let _e6: AntiScalar = other_169;
    return AntiScalar((_e4.g0_ * _e6.g0_));
}

fn anti_scalar_anti_scalar_geometric_anti_product(self_218: AntiScalar, other_170: AntiScalar) -> AntiScalar {
    var self_219: AntiScalar;
    var other_171: AntiScalar;

    self_219 = self_218;
    other_171 = other_170;
    let _e4: AntiScalar = self_219;
    let _e6: AntiScalar = other_171;
    return AntiScalar((_e4.g0_ * _e6.g0_));
}

fn anti_scalar_anti_scalar_inner_anti_product(self_220: AntiScalar, other_172: AntiScalar) -> AntiScalar {
    var self_221: AntiScalar;
    var other_173: AntiScalar;

    self_221 = self_220;
    other_173 = other_172;
    let _e4: AntiScalar = self_221;
    let _e6: AntiScalar = other_173;
    return AntiScalar((_e4.g0_ * _e6.g0_));
}

fn anti_scalar_anti_scalar_left_anti_contraction(self_222: AntiScalar, other_174: AntiScalar) -> AntiScalar {
    var self_223: AntiScalar;
    var other_175: AntiScalar;

    self_223 = self_222;
    other_175 = other_174;
    let _e4: AntiScalar = self_223;
    let _e6: AntiScalar = other_175;
    return AntiScalar((_e4.g0_ * _e6.g0_));
}

fn anti_scalar_anti_scalar_right_anti_contraction(self_224: AntiScalar, other_176: AntiScalar) -> AntiScalar {
    var self_225: AntiScalar;
    var other_177: AntiScalar;

    self_225 = self_224;
    other_177 = other_176;
    let _e4: AntiScalar = self_225;
    let _e6: AntiScalar = other_177;
    return AntiScalar((_e4.g0_ * _e6.g0_));
}

fn anti_scalar_anti_scalar_anti_scalar_product(self_226: AntiScalar, other_178: AntiScalar) -> AntiScalar {
    var self_227: AntiScalar;
    var other_179: AntiScalar;

    self_227 = self_226;
    other_179 = other_178;
    let _e4: AntiScalar = self_227;
    let _e6: AntiScalar = other_179;
    return AntiScalar((_e4.g0_ * _e6.g0_));
}

fn anti_scalar_homogeneous_magnitude_add(self_228: AntiScalar, other_180: HomogeneousMagnitude) -> HomogeneousMagnitude {
    var self_229: AntiScalar;
    var other_181: HomogeneousMagnitude;

    self_229 = self_228;
    other_181 = other_180;
    let _e4: AntiScalar = self_229;
    let _e11: HomogeneousMagnitude = other_181;
    return HomogeneousMagnitude(((vec2<f32>(_e4.g0_) * vec2<f32>(0.0, 1.0)) + _e11.g0_));
}

fn anti_scalar_homogeneous_magnitude_sub(self_230: AntiScalar, other_182: HomogeneousMagnitude) -> HomogeneousMagnitude {
    var self_231: AntiScalar;
    var other_183: HomogeneousMagnitude;

    self_231 = self_230;
    other_183 = other_182;
    let _e4: AntiScalar = self_231;
    let _e11: HomogeneousMagnitude = other_183;
    return HomogeneousMagnitude(((vec2<f32>(_e4.g0_) * vec2<f32>(0.0, 1.0)) - _e11.g0_));
}

fn anti_scalar_homogeneous_magnitude_geometric_product(self_232: AntiScalar, other_184: HomogeneousMagnitude) -> AntiScalar {
    var self_233: AntiScalar;
    var other_185: HomogeneousMagnitude;

    self_233 = self_232;
    other_185 = other_184;
    let _e4: AntiScalar = self_233;
    let _e6: HomogeneousMagnitude = other_185;
    return AntiScalar((_e4.g0_ * _e6.g0_.x));
}

fn anti_scalar_homogeneous_magnitude_regressive_product(self_234: AntiScalar, other_186: HomogeneousMagnitude) -> HomogeneousMagnitude {
    var self_235: AntiScalar;
    var other_187: HomogeneousMagnitude;

    self_235 = self_234;
    other_187 = other_186;
    let _e4: AntiScalar = self_235;
    let _e7: HomogeneousMagnitude = other_187;
    return HomogeneousMagnitude((vec2<f32>(_e4.g0_) * _e7.g0_));
}

fn anti_scalar_homogeneous_magnitude_outer_product(self_236: AntiScalar, other_188: HomogeneousMagnitude) -> AntiScalar {
    var self_237: AntiScalar;
    var other_189: HomogeneousMagnitude;

    self_237 = self_236;
    other_189 = other_188;
    let _e4: AntiScalar = self_237;
    let _e6: HomogeneousMagnitude = other_189;
    return AntiScalar((_e4.g0_ * _e6.g0_.x));
}

fn anti_scalar_homogeneous_magnitude_inner_product(self_238: AntiScalar, other_190: HomogeneousMagnitude) -> AntiScalar {
    var self_239: AntiScalar;
    var other_191: HomogeneousMagnitude;

    self_239 = self_238;
    other_191 = other_190;
    let _e4: AntiScalar = self_239;
    let _e6: HomogeneousMagnitude = other_191;
    return AntiScalar((_e4.g0_ * _e6.g0_.x));
}

fn anti_scalar_homogeneous_magnitude_geometric_anti_product(self_240: AntiScalar, other_192: HomogeneousMagnitude) -> HomogeneousMagnitude {
    var self_241: AntiScalar;
    var other_193: HomogeneousMagnitude;

    self_241 = self_240;
    other_193 = other_192;
    let _e4: AntiScalar = self_241;
    let _e7: HomogeneousMagnitude = other_193;
    return HomogeneousMagnitude((vec2<f32>(_e4.g0_) * _e7.g0_));
}

fn anti_scalar_homogeneous_magnitude_inner_anti_product(self_242: AntiScalar, other_194: HomogeneousMagnitude) -> HomogeneousMagnitude {
    var self_243: AntiScalar;
    var other_195: HomogeneousMagnitude;

    self_243 = self_242;
    other_195 = other_194;
    let _e4: AntiScalar = self_243;
    let _e7: HomogeneousMagnitude = other_195;
    return HomogeneousMagnitude((vec2<f32>(_e4.g0_) * _e7.g0_));
}

fn anti_scalar_homogeneous_magnitude_right_contraction(self_244: AntiScalar, other_196: HomogeneousMagnitude) -> AntiScalar {
    var self_245: AntiScalar;
    var other_197: HomogeneousMagnitude;

    self_245 = self_244;
    other_197 = other_196;
    let _e4: AntiScalar = self_245;
    let _e6: HomogeneousMagnitude = other_197;
    return AntiScalar((_e4.g0_ * _e6.g0_.x));
}

fn anti_scalar_homogeneous_magnitude_left_anti_contraction(self_246: AntiScalar, other_198: HomogeneousMagnitude) -> HomogeneousMagnitude {
    var self_247: AntiScalar;
    var other_199: HomogeneousMagnitude;

    self_247 = self_246;
    other_199 = other_198;
    let _e4: AntiScalar = self_247;
    let _e7: HomogeneousMagnitude = other_199;
    return HomogeneousMagnitude((vec2<f32>(_e4.g0_) * _e7.g0_));
}

fn anti_scalar_homogeneous_magnitude_right_anti_contraction(self_248: AntiScalar, other_200: HomogeneousMagnitude) -> AntiScalar {
    var self_249: AntiScalar;
    var other_201: HomogeneousMagnitude;

    self_249 = self_248;
    other_201 = other_200;
    let _e4: AntiScalar = self_249;
    let _e6: HomogeneousMagnitude = other_201;
    return AntiScalar((_e4.g0_ * _e6.g0_.y));
}

fn anti_scalar_homogeneous_magnitude_anti_scalar_product(self_250: AntiScalar, other_202: HomogeneousMagnitude) -> AntiScalar {
    var self_251: AntiScalar;
    var other_203: HomogeneousMagnitude;

    self_251 = self_250;
    other_203 = other_202;
    let _e4: AntiScalar = self_251;
    let _e6: HomogeneousMagnitude = other_203;
    return AntiScalar((_e4.g0_ * _e6.g0_.y));
}

fn anti_scalar_point_regressive_product(self_252: AntiScalar, other_204: Point) -> Point {
    var self_253: AntiScalar;
    var other_205: Point;

    self_253 = self_252;
    other_205 = other_204;
    let _e4: AntiScalar = self_253;
    let _e7: Point = other_205;
    return Point((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn anti_scalar_point_geometric_anti_product(self_254: AntiScalar, other_206: Point) -> Point {
    var self_255: AntiScalar;
    var other_207: Point;

    self_255 = self_254;
    other_207 = other_206;
    let _e4: AntiScalar = self_255;
    let _e7: Point = other_207;
    return Point((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn anti_scalar_point_inner_anti_product(self_256: AntiScalar, other_208: Point) -> Point {
    var self_257: AntiScalar;
    var other_209: Point;

    self_257 = self_256;
    other_209 = other_208;
    let _e4: AntiScalar = self_257;
    let _e7: Point = other_209;
    return Point((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn anti_scalar_point_left_anti_contraction(self_258: AntiScalar, other_210: Point) -> Point {
    var self_259: AntiScalar;
    var other_211: Point;

    self_259 = self_258;
    other_211 = other_210;
    let _e4: AntiScalar = self_259;
    let _e7: Point = other_211;
    return Point((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn anti_scalar_line_add(self_260: AntiScalar, other_212: Line) -> Motor {
    var self_261: AntiScalar;
    var other_213: Line;

    self_261 = self_260;
    other_213 = other_212;
    let _e4: AntiScalar = self_261;
    let _e13: Line = other_213;
    let _e16: Line = other_213;
    let _e19: Line = other_213;
    let _e22: Line = other_213;
    let _e33: Line = other_213;
    return Motor(((vec4<f32>(_e4.g0_) * vec4<f32>(0.0, 0.0, 0.0, 1.0)) + (vec4<f32>(_e13.g0_.x, _e16.g0_.y, _e19.g0_.z, _e22.g0_.x) * vec4<f32>(1.0, 1.0, 1.0, 0.0))), _e33.g1_);
}

fn anti_scalar_line_sub(self_262: AntiScalar, other_214: Line) -> Motor {
    var self_263: AntiScalar;
    var other_215: Line;

    self_263 = self_262;
    other_215 = other_214;
    let _e4: AntiScalar = self_263;
    let _e13: Line = other_215;
    let _e16: Line = other_215;
    let _e19: Line = other_215;
    let _e22: Line = other_215;
    let _e35: Line = other_215;
    return Motor(((vec4<f32>(_e4.g0_) * vec4<f32>(0.0, 0.0, 0.0, 1.0)) - (vec4<f32>(_e13.g0_.x, _e16.g0_.y, _e19.g0_.z, _e22.g0_.x) * vec4<f32>(1.0, 1.0, 1.0, 0.0))), (vec3<f32>(0.0) - _e35.g1_));
}

fn anti_scalar_line_regressive_product(self_264: AntiScalar, other_216: Line) -> Line {
    var self_265: AntiScalar;
    var other_217: Line;

    self_265 = self_264;
    other_217 = other_216;
    let _e4: AntiScalar = self_265;
    let _e7: Line = other_217;
    let _e10: AntiScalar = self_265;
    let _e13: Line = other_217;
    return Line((vec3<f32>(_e4.g0_) * _e7.g0_), (vec3<f32>(_e10.g0_) * _e13.g1_));
}

fn anti_scalar_line_geometric_anti_product(self_266: AntiScalar, other_218: Line) -> Line {
    var self_267: AntiScalar;
    var other_219: Line;

    self_267 = self_266;
    other_219 = other_218;
    let _e4: AntiScalar = self_267;
    let _e7: Line = other_219;
    let _e10: AntiScalar = self_267;
    let _e13: Line = other_219;
    return Line((vec3<f32>(_e4.g0_) * _e7.g0_), (vec3<f32>(_e10.g0_) * _e13.g1_));
}

fn anti_scalar_line_inner_anti_product(self_268: AntiScalar, other_220: Line) -> Line {
    var self_269: AntiScalar;
    var other_221: Line;

    self_269 = self_268;
    other_221 = other_220;
    let _e4: AntiScalar = self_269;
    let _e7: Line = other_221;
    let _e10: AntiScalar = self_269;
    let _e13: Line = other_221;
    return Line((vec3<f32>(_e4.g0_) * _e7.g0_), (vec3<f32>(_e10.g0_) * _e13.g1_));
}

fn anti_scalar_line_left_anti_contraction(self_270: AntiScalar, other_222: Line) -> Line {
    var self_271: AntiScalar;
    var other_223: Line;

    self_271 = self_270;
    other_223 = other_222;
    let _e4: AntiScalar = self_271;
    let _e7: Line = other_223;
    let _e10: AntiScalar = self_271;
    let _e13: Line = other_223;
    return Line((vec3<f32>(_e4.g0_) * _e7.g0_), (vec3<f32>(_e10.g0_) * _e13.g1_));
}

fn anti_scalar_plane_regressive_product(self_272: AntiScalar, other_224: Plane) -> Plane {
    var self_273: AntiScalar;
    var other_225: Plane;

    self_273 = self_272;
    other_225 = other_224;
    let _e4: AntiScalar = self_273;
    let _e7: Plane = other_225;
    return Plane((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn anti_scalar_plane_geometric_anti_product(self_274: AntiScalar, other_226: Plane) -> Plane {
    var self_275: AntiScalar;
    var other_227: Plane;

    self_275 = self_274;
    other_227 = other_226;
    let _e4: AntiScalar = self_275;
    let _e7: Plane = other_227;
    return Plane((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn anti_scalar_plane_inner_anti_product(self_276: AntiScalar, other_228: Plane) -> Plane {
    var self_277: AntiScalar;
    var other_229: Plane;

    self_277 = self_276;
    other_229 = other_228;
    let _e4: AntiScalar = self_277;
    let _e7: Plane = other_229;
    return Plane((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn anti_scalar_plane_left_anti_contraction(self_278: AntiScalar, other_230: Plane) -> Plane {
    var self_279: AntiScalar;
    var other_231: Plane;

    self_279 = self_278;
    other_231 = other_230;
    let _e4: AntiScalar = self_279;
    let _e7: Plane = other_231;
    return Plane((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn anti_scalar_motor_add(self_280: AntiScalar, other_232: Motor) -> Motor {
    var self_281: AntiScalar;
    var other_233: Motor;

    self_281 = self_280;
    other_233 = other_232;
    let _e4: AntiScalar = self_281;
    let _e13: Motor = other_233;
    let _e16: Motor = other_233;
    return Motor(((vec4<f32>(_e4.g0_) * vec4<f32>(0.0, 0.0, 0.0, 1.0)) + _e13.g0_), _e16.g1_);
}

fn anti_scalar_motor_sub(self_282: AntiScalar, other_234: Motor) -> Motor {
    var self_283: AntiScalar;
    var other_235: Motor;

    self_283 = self_282;
    other_235 = other_234;
    let _e4: AntiScalar = self_283;
    let _e13: Motor = other_235;
    let _e18: Motor = other_235;
    return Motor(((vec4<f32>(_e4.g0_) * vec4<f32>(0.0, 0.0, 0.0, 1.0)) - _e13.g0_), (vec3<f32>(0.0) - _e18.g1_));
}

fn anti_scalar_motor_regressive_product(self_284: AntiScalar, other_236: Motor) -> Motor {
    var self_285: AntiScalar;
    var other_237: Motor;

    self_285 = self_284;
    other_237 = other_236;
    let _e4: AntiScalar = self_285;
    let _e7: Motor = other_237;
    let _e10: AntiScalar = self_285;
    let _e13: Motor = other_237;
    return Motor((vec4<f32>(_e4.g0_) * _e7.g0_), (vec3<f32>(_e10.g0_) * _e13.g1_));
}

fn anti_scalar_motor_geometric_anti_product(self_286: AntiScalar, other_238: Motor) -> Motor {
    var self_287: AntiScalar;
    var other_239: Motor;

    self_287 = self_286;
    other_239 = other_238;
    let _e4: AntiScalar = self_287;
    let _e7: Motor = other_239;
    let _e10: AntiScalar = self_287;
    let _e13: Motor = other_239;
    return Motor((vec4<f32>(_e4.g0_) * _e7.g0_), (vec3<f32>(_e10.g0_) * _e13.g1_));
}

fn anti_scalar_motor_inner_anti_product(self_288: AntiScalar, other_240: Motor) -> Motor {
    var self_289: AntiScalar;
    var other_241: Motor;

    self_289 = self_288;
    other_241 = other_240;
    let _e4: AntiScalar = self_289;
    let _e7: Motor = other_241;
    let _e10: AntiScalar = self_289;
    let _e13: Motor = other_241;
    return Motor((vec4<f32>(_e4.g0_) * _e7.g0_), (vec3<f32>(_e10.g0_) * _e13.g1_));
}

fn anti_scalar_motor_left_anti_contraction(self_290: AntiScalar, other_242: Motor) -> Motor {
    var self_291: AntiScalar;
    var other_243: Motor;

    self_291 = self_290;
    other_243 = other_242;
    let _e4: AntiScalar = self_291;
    let _e7: Motor = other_243;
    let _e10: AntiScalar = self_291;
    let _e13: Motor = other_243;
    return Motor((vec4<f32>(_e4.g0_) * _e7.g0_), (vec3<f32>(_e10.g0_) * _e13.g1_));
}

fn anti_scalar_motor_right_anti_contraction(self_292: AntiScalar, other_244: Motor) -> AntiScalar {
    var self_293: AntiScalar;
    var other_245: Motor;

    self_293 = self_292;
    other_245 = other_244;
    let _e4: AntiScalar = self_293;
    let _e6: Motor = other_245;
    return AntiScalar((_e4.g0_ * _e6.g0_.w));
}

fn anti_scalar_motor_anti_scalar_product(self_294: AntiScalar, other_246: Motor) -> AntiScalar {
    var self_295: AntiScalar;
    var other_247: Motor;

    self_295 = self_294;
    other_247 = other_246;
    let _e4: AntiScalar = self_295;
    let _e6: Motor = other_247;
    return AntiScalar((_e4.g0_ * _e6.g0_.w));
}

fn anti_scalar_rotor_add(self_296: AntiScalar, other_248: Rotor) -> Rotor {
    var self_297: AntiScalar;
    var other_249: Rotor;

    self_297 = self_296;
    other_249 = other_248;
    let _e4: AntiScalar = self_297;
    let _e13: Rotor = other_249;
    return Rotor(((vec4<f32>(_e4.g0_) * vec4<f32>(0.0, 0.0, 0.0, 1.0)) + _e13.g0_));
}

fn anti_scalar_rotor_sub(self_298: AntiScalar, other_250: Rotor) -> Rotor {
    var self_299: AntiScalar;
    var other_251: Rotor;

    self_299 = self_298;
    other_251 = other_250;
    let _e4: AntiScalar = self_299;
    let _e13: Rotor = other_251;
    return Rotor(((vec4<f32>(_e4.g0_) * vec4<f32>(0.0, 0.0, 0.0, 1.0)) - _e13.g0_));
}

fn anti_scalar_rotor_regressive_product(self_300: AntiScalar, other_252: Rotor) -> Rotor {
    var self_301: AntiScalar;
    var other_253: Rotor;

    self_301 = self_300;
    other_253 = other_252;
    let _e4: AntiScalar = self_301;
    let _e7: Rotor = other_253;
    return Rotor((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn anti_scalar_rotor_geometric_anti_product(self_302: AntiScalar, other_254: Rotor) -> Rotor {
    var self_303: AntiScalar;
    var other_255: Rotor;

    self_303 = self_302;
    other_255 = other_254;
    let _e4: AntiScalar = self_303;
    let _e7: Rotor = other_255;
    return Rotor((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn anti_scalar_rotor_inner_anti_product(self_304: AntiScalar, other_256: Rotor) -> Rotor {
    var self_305: AntiScalar;
    var other_257: Rotor;

    self_305 = self_304;
    other_257 = other_256;
    let _e4: AntiScalar = self_305;
    let _e7: Rotor = other_257;
    return Rotor((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn anti_scalar_rotor_left_anti_contraction(self_306: AntiScalar, other_258: Rotor) -> Rotor {
    var self_307: AntiScalar;
    var other_259: Rotor;

    self_307 = self_306;
    other_259 = other_258;
    let _e4: AntiScalar = self_307;
    let _e7: Rotor = other_259;
    return Rotor((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn anti_scalar_rotor_right_anti_contraction(self_308: AntiScalar, other_260: Rotor) -> AntiScalar {
    var self_309: AntiScalar;
    var other_261: Rotor;

    self_309 = self_308;
    other_261 = other_260;
    let _e4: AntiScalar = self_309;
    let _e6: Rotor = other_261;
    return AntiScalar((_e4.g0_ * _e6.g0_.w));
}

fn anti_scalar_rotor_anti_scalar_product(self_310: AntiScalar, other_262: Rotor) -> AntiScalar {
    var self_311: AntiScalar;
    var other_263: Rotor;

    self_311 = self_310;
    other_263 = other_262;
    let _e4: AntiScalar = self_311;
    let _e6: Rotor = other_263;
    return AntiScalar((_e4.g0_ * _e6.g0_.w));
}

fn anti_scalar_translator_add(self_312: AntiScalar, other_264: Translator) -> Translator {
    var self_313: AntiScalar;
    var other_265: Translator;

    self_313 = self_312;
    other_265 = other_264;
    let _e4: AntiScalar = self_313;
    let _e13: Translator = other_265;
    return Translator(((vec4<f32>(_e4.g0_) * vec4<f32>(0.0, 0.0, 0.0, 1.0)) + _e13.g0_));
}

fn anti_scalar_translator_sub(self_314: AntiScalar, other_266: Translator) -> Translator {
    var self_315: AntiScalar;
    var other_267: Translator;

    self_315 = self_314;
    other_267 = other_266;
    let _e4: AntiScalar = self_315;
    let _e13: Translator = other_267;
    return Translator(((vec4<f32>(_e4.g0_) * vec4<f32>(0.0, 0.0, 0.0, 1.0)) - _e13.g0_));
}

fn anti_scalar_translator_regressive_product(self_316: AntiScalar, other_268: Translator) -> Translator {
    var self_317: AntiScalar;
    var other_269: Translator;

    self_317 = self_316;
    other_269 = other_268;
    let _e4: AntiScalar = self_317;
    let _e7: Translator = other_269;
    return Translator((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn anti_scalar_translator_geometric_anti_product(self_318: AntiScalar, other_270: Translator) -> Translator {
    var self_319: AntiScalar;
    var other_271: Translator;

    self_319 = self_318;
    other_271 = other_270;
    let _e4: AntiScalar = self_319;
    let _e7: Translator = other_271;
    return Translator((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn anti_scalar_translator_inner_anti_product(self_320: AntiScalar, other_272: Translator) -> Translator {
    var self_321: AntiScalar;
    var other_273: Translator;

    self_321 = self_320;
    other_273 = other_272;
    let _e4: AntiScalar = self_321;
    let _e7: Translator = other_273;
    return Translator((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn anti_scalar_translator_left_anti_contraction(self_322: AntiScalar, other_274: Translator) -> Translator {
    var self_323: AntiScalar;
    var other_275: Translator;

    self_323 = self_322;
    other_275 = other_274;
    let _e4: AntiScalar = self_323;
    let _e7: Translator = other_275;
    return Translator((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn anti_scalar_translator_right_anti_contraction(self_324: AntiScalar, other_276: Translator) -> AntiScalar {
    var self_325: AntiScalar;
    var other_277: Translator;

    self_325 = self_324;
    other_277 = other_276;
    let _e4: AntiScalar = self_325;
    let _e6: Translator = other_277;
    return AntiScalar((_e4.g0_ * _e6.g0_.w));
}

fn anti_scalar_translator_anti_scalar_product(self_326: AntiScalar, other_278: Translator) -> AntiScalar {
    var self_327: AntiScalar;
    var other_279: Translator;

    self_327 = self_326;
    other_279 = other_278;
    let _e4: AntiScalar = self_327;
    let _e6: Translator = other_279;
    return AntiScalar((_e4.g0_ * _e6.g0_.w));
}

fn anti_scalar_flector_regressive_product(self_328: AntiScalar, other_280: Flector) -> Flector {
    var self_329: AntiScalar;
    var other_281: Flector;

    self_329 = self_328;
    other_281 = other_280;
    let _e4: AntiScalar = self_329;
    let _e7: Flector = other_281;
    let _e10: AntiScalar = self_329;
    let _e13: Flector = other_281;
    return Flector((vec4<f32>(_e4.g0_) * _e7.g0_), (vec4<f32>(_e10.g0_) * _e13.g1_));
}

fn anti_scalar_flector_geometric_anti_product(self_330: AntiScalar, other_282: Flector) -> Flector {
    var self_331: AntiScalar;
    var other_283: Flector;

    self_331 = self_330;
    other_283 = other_282;
    let _e4: AntiScalar = self_331;
    let _e7: Flector = other_283;
    let _e10: AntiScalar = self_331;
    let _e13: Flector = other_283;
    return Flector((vec4<f32>(_e4.g0_) * _e7.g0_), (vec4<f32>(_e10.g0_) * _e13.g1_));
}

fn anti_scalar_flector_inner_anti_product(self_332: AntiScalar, other_284: Flector) -> Flector {
    var self_333: AntiScalar;
    var other_285: Flector;

    self_333 = self_332;
    other_285 = other_284;
    let _e4: AntiScalar = self_333;
    let _e7: Flector = other_285;
    let _e10: AntiScalar = self_333;
    let _e13: Flector = other_285;
    return Flector((vec4<f32>(_e4.g0_) * _e7.g0_), (vec4<f32>(_e10.g0_) * _e13.g1_));
}

fn anti_scalar_flector_left_anti_contraction(self_334: AntiScalar, other_286: Flector) -> Flector {
    var self_335: AntiScalar;
    var other_287: Flector;

    self_335 = self_334;
    other_287 = other_286;
    let _e4: AntiScalar = self_335;
    let _e7: Flector = other_287;
    let _e10: AntiScalar = self_335;
    let _e13: Flector = other_287;
    return Flector((vec4<f32>(_e4.g0_) * _e7.g0_), (vec4<f32>(_e10.g0_) * _e13.g1_));
}

fn anti_scalar_multi_vector_outer_product(self_336: AntiScalar, other_288: MultiVector) -> AntiScalar {
    var self_337: AntiScalar;
    var other_289: MultiVector;

    self_337 = self_336;
    other_289 = other_288;
    let _e4: AntiScalar = self_337;
    let _e6: MultiVector = other_289;
    return AntiScalar((_e4.g0_ * _e6.g0_.x));
}

fn anti_scalar_multi_vector_right_anti_contraction(self_338: AntiScalar, other_290: MultiVector) -> AntiScalar {
    var self_339: AntiScalar;
    var other_291: MultiVector;

    self_339 = self_338;
    other_291 = other_290;
    let _e4: AntiScalar = self_339;
    let _e6: MultiVector = other_291;
    return AntiScalar((_e4.g0_ * _e6.g0_.y));
}

fn anti_scalar_multi_vector_anti_scalar_product(self_340: AntiScalar, other_292: MultiVector) -> AntiScalar {
    var self_341: AntiScalar;
    var other_293: MultiVector;

    self_341 = self_340;
    other_293 = other_292;
    let _e4: AntiScalar = self_341;
    let _e6: MultiVector = other_293;
    return AntiScalar((_e4.g0_ * _e6.g0_.y));
}

fn anti_scalar_scale(self_342: AntiScalar, other_294: f32) -> AntiScalar {
    var self_343: AntiScalar;
    var other_295: f32;

    self_343 = self_342;
    other_295 = other_294;
    let _e4: AntiScalar = self_343;
    let _e5: f32 = other_295;
    let _e7: AntiScalar = anti_scalar_scalar_geometric_product(_e4, Scalar(_e5));
    return _e7;
}

fn homogeneous_magnitude_zero() -> HomogeneousMagnitude {
    return HomogeneousMagnitude(vec2<f32>(0.0));
}

fn homogeneous_magnitude_one() -> HomogeneousMagnitude {
    return HomogeneousMagnitude(vec2<f32>(1.0, 0.0));
}

fn homogeneous_magnitude_neg(self_344: HomogeneousMagnitude) -> HomogeneousMagnitude {
    var self_345: HomogeneousMagnitude;

    self_345 = self_344;
    let _e2: HomogeneousMagnitude = self_345;
    return HomogeneousMagnitude((_e2.g0_ * vec2<f32>(-(1.0))));
}

fn homogeneous_magnitude_automorphism(self_346: HomogeneousMagnitude) -> HomogeneousMagnitude {
    var self_347: HomogeneousMagnitude;

    self_347 = self_346;
    let _e2: HomogeneousMagnitude = self_347;
    return HomogeneousMagnitude(_e2.g0_);
}

fn homogeneous_magnitude_reversal(self_348: HomogeneousMagnitude) -> HomogeneousMagnitude {
    var self_349: HomogeneousMagnitude;

    self_349 = self_348;
    let _e2: HomogeneousMagnitude = self_349;
    return HomogeneousMagnitude(_e2.g0_);
}

fn homogeneous_magnitude_conjugation(self_350: HomogeneousMagnitude) -> HomogeneousMagnitude {
    var self_351: HomogeneousMagnitude;

    self_351 = self_350;
    let _e2: HomogeneousMagnitude = self_351;
    return HomogeneousMagnitude(_e2.g0_);
}

fn homogeneous_magnitude_dual(self_352: HomogeneousMagnitude) -> HomogeneousMagnitude {
    var self_353: HomogeneousMagnitude;

    self_353 = self_352;
    let _e2: HomogeneousMagnitude = self_353;
    return HomogeneousMagnitude(_e2.g0_.yx);
}

fn homogeneous_magnitude_anti_reversal(self_354: HomogeneousMagnitude) -> HomogeneousMagnitude {
    var self_355: HomogeneousMagnitude;

    self_355 = self_354;
    let _e2: HomogeneousMagnitude = self_355;
    return HomogeneousMagnitude(_e2.g0_);
}

fn homogeneous_magnitude_right_complement(self_356: HomogeneousMagnitude) -> HomogeneousMagnitude {
    var self_357: HomogeneousMagnitude;

    self_357 = self_356;
    let _e2: HomogeneousMagnitude = self_357;
    return HomogeneousMagnitude(_e2.g0_.yx);
}

fn homogeneous_magnitude_left_complement(self_358: HomogeneousMagnitude) -> HomogeneousMagnitude {
    var self_359: HomogeneousMagnitude;

    self_359 = self_358;
    let _e2: HomogeneousMagnitude = self_359;
    return HomogeneousMagnitude(_e2.g0_.yx);
}

fn homogeneous_magnitude_double_complement(self_360: HomogeneousMagnitude) -> HomogeneousMagnitude {
    var self_361: HomogeneousMagnitude;

    self_361 = self_360;
    let _e2: HomogeneousMagnitude = self_361;
    return HomogeneousMagnitude(_e2.g0_);
}

fn homogeneous_magnitude_scalar_into(self_362: HomogeneousMagnitude) -> Scalar {
    var self_363: HomogeneousMagnitude;

    self_363 = self_362;
    let _e2: HomogeneousMagnitude = self_363;
    return Scalar(_e2.g0_.x);
}

fn homogeneous_magnitude_scalar_add(self_364: HomogeneousMagnitude, other_296: Scalar) -> HomogeneousMagnitude {
    var self_365: HomogeneousMagnitude;
    var other_297: Scalar;

    self_365 = self_364;
    other_297 = other_296;
    let _e4: HomogeneousMagnitude = self_365;
    let _e6: Scalar = other_297;
    return HomogeneousMagnitude((_e4.g0_ + (vec2<f32>(_e6.g0_) * vec2<f32>(1.0, 0.0))));
}

fn homogeneous_magnitude_scalar_sub(self_366: HomogeneousMagnitude, other_298: Scalar) -> HomogeneousMagnitude {
    var self_367: HomogeneousMagnitude;
    var other_299: Scalar;

    self_367 = self_366;
    other_299 = other_298;
    let _e4: HomogeneousMagnitude = self_367;
    let _e6: Scalar = other_299;
    return HomogeneousMagnitude((_e4.g0_ - (vec2<f32>(_e6.g0_) * vec2<f32>(1.0, 0.0))));
}

fn homogeneous_magnitude_scalar_geometric_product(self_368: HomogeneousMagnitude, other_300: Scalar) -> HomogeneousMagnitude {
    var self_369: HomogeneousMagnitude;
    var other_301: Scalar;

    self_369 = self_368;
    other_301 = other_300;
    let _e4: HomogeneousMagnitude = self_369;
    let _e6: Scalar = other_301;
    return HomogeneousMagnitude((_e4.g0_ * vec2<f32>(_e6.g0_)));
}

fn homogeneous_magnitude_scalar_regressive_product(self_370: HomogeneousMagnitude, other_302: Scalar) -> Scalar {
    var self_371: HomogeneousMagnitude;
    var other_303: Scalar;

    self_371 = self_370;
    other_303 = other_302;
    let _e4: HomogeneousMagnitude = self_371;
    let _e7: Scalar = other_303;
    return Scalar((_e4.g0_.y * _e7.g0_));
}

fn homogeneous_magnitude_scalar_outer_product(self_372: HomogeneousMagnitude, other_304: Scalar) -> HomogeneousMagnitude {
    var self_373: HomogeneousMagnitude;
    var other_305: Scalar;

    self_373 = self_372;
    other_305 = other_304;
    let _e4: HomogeneousMagnitude = self_373;
    let _e6: Scalar = other_305;
    return HomogeneousMagnitude((_e4.g0_ * vec2<f32>(_e6.g0_)));
}

fn homogeneous_magnitude_scalar_inner_product(self_374: HomogeneousMagnitude, other_306: Scalar) -> HomogeneousMagnitude {
    var self_375: HomogeneousMagnitude;
    var other_307: Scalar;

    self_375 = self_374;
    other_307 = other_306;
    let _e4: HomogeneousMagnitude = self_375;
    let _e6: Scalar = other_307;
    return HomogeneousMagnitude((_e4.g0_ * vec2<f32>(_e6.g0_)));
}

fn homogeneous_magnitude_scalar_geometric_anti_product(self_376: HomogeneousMagnitude, other_308: Scalar) -> Scalar {
    var self_377: HomogeneousMagnitude;
    var other_309: Scalar;

    self_377 = self_376;
    other_309 = other_308;
    let _e4: HomogeneousMagnitude = self_377;
    let _e7: Scalar = other_309;
    return Scalar((_e4.g0_.y * _e7.g0_));
}

fn homogeneous_magnitude_scalar_inner_anti_product(self_378: HomogeneousMagnitude, other_310: Scalar) -> Scalar {
    var self_379: HomogeneousMagnitude;
    var other_311: Scalar;

    self_379 = self_378;
    other_311 = other_310;
    let _e4: HomogeneousMagnitude = self_379;
    let _e7: Scalar = other_311;
    return Scalar((_e4.g0_.y * _e7.g0_));
}

fn homogeneous_magnitude_scalar_left_contraction(self_380: HomogeneousMagnitude, other_312: Scalar) -> Scalar {
    var self_381: HomogeneousMagnitude;
    var other_313: Scalar;

    self_381 = self_380;
    other_313 = other_312;
    let _e4: HomogeneousMagnitude = self_381;
    let _e7: Scalar = other_313;
    return Scalar((_e4.g0_.x * _e7.g0_));
}

fn homogeneous_magnitude_scalar_right_contraction(self_382: HomogeneousMagnitude, other_314: Scalar) -> HomogeneousMagnitude {
    var self_383: HomogeneousMagnitude;
    var other_315: Scalar;

    self_383 = self_382;
    other_315 = other_314;
    let _e4: HomogeneousMagnitude = self_383;
    let _e6: Scalar = other_315;
    return HomogeneousMagnitude((_e4.g0_ * vec2<f32>(_e6.g0_)));
}

fn homogeneous_magnitude_scalar_left_anti_contraction(self_384: HomogeneousMagnitude, other_316: Scalar) -> Scalar {
    var self_385: HomogeneousMagnitude;
    var other_317: Scalar;

    self_385 = self_384;
    other_317 = other_316;
    let _e4: HomogeneousMagnitude = self_385;
    let _e7: Scalar = other_317;
    return Scalar((_e4.g0_.y * _e7.g0_));
}

fn homogeneous_magnitude_scalar_scalar_product(self_386: HomogeneousMagnitude, other_318: Scalar) -> Scalar {
    var self_387: HomogeneousMagnitude;
    var other_319: Scalar;

    self_387 = self_386;
    other_319 = other_318;
    let _e4: HomogeneousMagnitude = self_387;
    let _e7: Scalar = other_319;
    return Scalar((_e4.g0_.x * _e7.g0_));
}

fn homogeneous_magnitude_anti_scalar_into(self_388: HomogeneousMagnitude) -> AntiScalar {
    var self_389: HomogeneousMagnitude;

    self_389 = self_388;
    let _e2: HomogeneousMagnitude = self_389;
    return AntiScalar(_e2.g0_.y);
}

fn homogeneous_magnitude_anti_scalar_add(self_390: HomogeneousMagnitude, other_320: AntiScalar) -> HomogeneousMagnitude {
    var self_391: HomogeneousMagnitude;
    var other_321: AntiScalar;

    self_391 = self_390;
    other_321 = other_320;
    let _e4: HomogeneousMagnitude = self_391;
    let _e6: AntiScalar = other_321;
    return HomogeneousMagnitude((_e4.g0_ + (vec2<f32>(_e6.g0_) * vec2<f32>(0.0, 1.0))));
}

fn homogeneous_magnitude_anti_scalar_sub(self_392: HomogeneousMagnitude, other_322: AntiScalar) -> HomogeneousMagnitude {
    var self_393: HomogeneousMagnitude;
    var other_323: AntiScalar;

    self_393 = self_392;
    other_323 = other_322;
    let _e4: HomogeneousMagnitude = self_393;
    let _e6: AntiScalar = other_323;
    return HomogeneousMagnitude((_e4.g0_ - (vec2<f32>(_e6.g0_) * vec2<f32>(0.0, 1.0))));
}

fn homogeneous_magnitude_anti_scalar_geometric_product(self_394: HomogeneousMagnitude, other_324: AntiScalar) -> AntiScalar {
    var self_395: HomogeneousMagnitude;
    var other_325: AntiScalar;

    self_395 = self_394;
    other_325 = other_324;
    let _e4: HomogeneousMagnitude = self_395;
    let _e7: AntiScalar = other_325;
    return AntiScalar((_e4.g0_.x * _e7.g0_));
}

fn homogeneous_magnitude_anti_scalar_regressive_product(self_396: HomogeneousMagnitude, other_326: AntiScalar) -> HomogeneousMagnitude {
    var self_397: HomogeneousMagnitude;
    var other_327: AntiScalar;

    self_397 = self_396;
    other_327 = other_326;
    let _e4: HomogeneousMagnitude = self_397;
    let _e6: AntiScalar = other_327;
    return HomogeneousMagnitude((_e4.g0_ * vec2<f32>(_e6.g0_)));
}

fn homogeneous_magnitude_anti_scalar_outer_product(self_398: HomogeneousMagnitude, other_328: AntiScalar) -> AntiScalar {
    var self_399: HomogeneousMagnitude;
    var other_329: AntiScalar;

    self_399 = self_398;
    other_329 = other_328;
    let _e4: HomogeneousMagnitude = self_399;
    let _e7: AntiScalar = other_329;
    return AntiScalar((_e4.g0_.x * _e7.g0_));
}

fn homogeneous_magnitude_anti_scalar_inner_product(self_400: HomogeneousMagnitude, other_330: AntiScalar) -> AntiScalar {
    var self_401: HomogeneousMagnitude;
    var other_331: AntiScalar;

    self_401 = self_400;
    other_331 = other_330;
    let _e4: HomogeneousMagnitude = self_401;
    let _e7: AntiScalar = other_331;
    return AntiScalar((_e4.g0_.x * _e7.g0_));
}

fn homogeneous_magnitude_anti_scalar_geometric_anti_product(self_402: HomogeneousMagnitude, other_332: AntiScalar) -> HomogeneousMagnitude {
    var self_403: HomogeneousMagnitude;
    var other_333: AntiScalar;

    self_403 = self_402;
    other_333 = other_332;
    let _e4: HomogeneousMagnitude = self_403;
    let _e6: AntiScalar = other_333;
    return HomogeneousMagnitude((_e4.g0_ * vec2<f32>(_e6.g0_)));
}

fn homogeneous_magnitude_anti_scalar_inner_anti_product(self_404: HomogeneousMagnitude, other_334: AntiScalar) -> HomogeneousMagnitude {
    var self_405: HomogeneousMagnitude;
    var other_335: AntiScalar;

    self_405 = self_404;
    other_335 = other_334;
    let _e4: HomogeneousMagnitude = self_405;
    let _e6: AntiScalar = other_335;
    return HomogeneousMagnitude((_e4.g0_ * vec2<f32>(_e6.g0_)));
}

fn homogeneous_magnitude_anti_scalar_left_contraction(self_406: HomogeneousMagnitude, other_336: AntiScalar) -> AntiScalar {
    var self_407: HomogeneousMagnitude;
    var other_337: AntiScalar;

    self_407 = self_406;
    other_337 = other_336;
    let _e4: HomogeneousMagnitude = self_407;
    let _e7: AntiScalar = other_337;
    return AntiScalar((_e4.g0_.x * _e7.g0_));
}

fn homogeneous_magnitude_anti_scalar_left_anti_contraction(self_408: HomogeneousMagnitude, other_338: AntiScalar) -> AntiScalar {
    var self_409: HomogeneousMagnitude;
    var other_339: AntiScalar;

    self_409 = self_408;
    other_339 = other_338;
    let _e4: HomogeneousMagnitude = self_409;
    let _e7: AntiScalar = other_339;
    return AntiScalar((_e4.g0_.y * _e7.g0_));
}

fn homogeneous_magnitude_anti_scalar_right_anti_contraction(self_410: HomogeneousMagnitude, other_340: AntiScalar) -> HomogeneousMagnitude {
    var self_411: HomogeneousMagnitude;
    var other_341: AntiScalar;

    self_411 = self_410;
    other_341 = other_340;
    let _e4: HomogeneousMagnitude = self_411;
    let _e6: AntiScalar = other_341;
    return HomogeneousMagnitude((_e4.g0_ * vec2<f32>(_e6.g0_)));
}

fn homogeneous_magnitude_anti_scalar_anti_scalar_product(self_412: HomogeneousMagnitude, other_342: AntiScalar) -> AntiScalar {
    var self_413: HomogeneousMagnitude;
    var other_343: AntiScalar;

    self_413 = self_412;
    other_343 = other_342;
    let _e4: HomogeneousMagnitude = self_413;
    let _e7: AntiScalar = other_343;
    return AntiScalar((_e4.g0_.y * _e7.g0_));
}

fn homogeneous_magnitude_homogeneous_magnitude_add(self_414: HomogeneousMagnitude, other_344: HomogeneousMagnitude) -> HomogeneousMagnitude {
    var self_415: HomogeneousMagnitude;
    var other_345: HomogeneousMagnitude;

    self_415 = self_414;
    other_345 = other_344;
    let _e4: HomogeneousMagnitude = self_415;
    let _e6: HomogeneousMagnitude = other_345;
    return HomogeneousMagnitude((_e4.g0_ + _e6.g0_));
}

fn homogeneous_magnitude_homogeneous_magnitude_sub(self_416: HomogeneousMagnitude, other_346: HomogeneousMagnitude) -> HomogeneousMagnitude {
    var self_417: HomogeneousMagnitude;
    var other_347: HomogeneousMagnitude;

    self_417 = self_416;
    other_347 = other_346;
    let _e4: HomogeneousMagnitude = self_417;
    let _e6: HomogeneousMagnitude = other_347;
    return HomogeneousMagnitude((_e4.g0_ - _e6.g0_));
}

fn homogeneous_magnitude_homogeneous_magnitude_mul(self_418: HomogeneousMagnitude, other_348: HomogeneousMagnitude) -> HomogeneousMagnitude {
    var self_419: HomogeneousMagnitude;
    var other_349: HomogeneousMagnitude;

    self_419 = self_418;
    other_349 = other_348;
    let _e4: HomogeneousMagnitude = self_419;
    let _e6: HomogeneousMagnitude = other_349;
    return HomogeneousMagnitude((_e4.g0_ * _e6.g0_));
}

fn homogeneous_magnitude_homogeneous_magnitude_div(self_420: HomogeneousMagnitude, other_350: HomogeneousMagnitude) -> HomogeneousMagnitude {
    var self_421: HomogeneousMagnitude;
    var other_351: HomogeneousMagnitude;

    self_421 = self_420;
    other_351 = other_350;
    let _e4: HomogeneousMagnitude = self_421;
    let _e7: HomogeneousMagnitude = self_421;
    let _e15: HomogeneousMagnitude = other_351;
    let _e18: HomogeneousMagnitude = other_351;
    return HomogeneousMagnitude((((vec2<f32>(_e4.g0_.x, _e7.g0_.y) * vec2<f32>(1.0, 1.0)) / vec2<f32>(_e15.g0_.x, _e18.g0_.y)) * vec2<f32>(1.0, 1.0)));
}

fn homogeneous_magnitude_homogeneous_magnitude_geometric_product(self_422: HomogeneousMagnitude, other_352: HomogeneousMagnitude) -> HomogeneousMagnitude {
    var self_423: HomogeneousMagnitude;
    var other_353: HomogeneousMagnitude;

    self_423 = self_422;
    other_353 = other_352;
    let _e4: HomogeneousMagnitude = self_423;
    let _e8: HomogeneousMagnitude = other_353;
    let _e11: HomogeneousMagnitude = self_423;
    let _e13: HomogeneousMagnitude = other_353;
    return HomogeneousMagnitude(((vec2<f32>(_e4.g0_.x) * _e8.g0_) + ((_e11.g0_ * vec2<f32>(_e13.g0_.x)) * vec2<f32>(0.0, 1.0))));
}

fn homogeneous_magnitude_homogeneous_magnitude_regressive_product(self_424: HomogeneousMagnitude, other_354: HomogeneousMagnitude) -> HomogeneousMagnitude {
    var self_425: HomogeneousMagnitude;
    var other_355: HomogeneousMagnitude;

    self_425 = self_424;
    other_355 = other_354;
    let _e4: HomogeneousMagnitude = self_425;
    let _e8: HomogeneousMagnitude = other_355;
    let _e11: HomogeneousMagnitude = self_425;
    let _e15: HomogeneousMagnitude = other_355;
    return HomogeneousMagnitude(((vec2<f32>(_e4.g0_.y) * _e8.g0_) + ((vec2<f32>(_e11.g0_.x) * _e15.g0_.yx) * vec2<f32>(1.0, 0.0))));
}

fn homogeneous_magnitude_homogeneous_magnitude_outer_product(self_426: HomogeneousMagnitude, other_356: HomogeneousMagnitude) -> HomogeneousMagnitude {
    var self_427: HomogeneousMagnitude;
    var other_357: HomogeneousMagnitude;

    self_427 = self_426;
    other_357 = other_356;
    let _e4: HomogeneousMagnitude = self_427;
    let _e8: HomogeneousMagnitude = other_357;
    let _e11: HomogeneousMagnitude = self_427;
    let _e13: HomogeneousMagnitude = other_357;
    return HomogeneousMagnitude(((vec2<f32>(_e4.g0_.x) * _e8.g0_) + ((_e11.g0_ * vec2<f32>(_e13.g0_.x)) * vec2<f32>(0.0, 1.0))));
}

fn homogeneous_magnitude_homogeneous_magnitude_inner_product(self_428: HomogeneousMagnitude, other_358: HomogeneousMagnitude) -> HomogeneousMagnitude {
    var self_429: HomogeneousMagnitude;
    var other_359: HomogeneousMagnitude;

    self_429 = self_428;
    other_359 = other_358;
    let _e4: HomogeneousMagnitude = self_429;
    let _e8: HomogeneousMagnitude = other_359;
    let _e11: HomogeneousMagnitude = self_429;
    let _e13: HomogeneousMagnitude = other_359;
    return HomogeneousMagnitude(((vec2<f32>(_e4.g0_.x) * _e8.g0_) + ((_e11.g0_ * vec2<f32>(_e13.g0_.x)) * vec2<f32>(0.0, 1.0))));
}

fn homogeneous_magnitude_homogeneous_magnitude_geometric_anti_product(self_430: HomogeneousMagnitude, other_360: HomogeneousMagnitude) -> HomogeneousMagnitude {
    var self_431: HomogeneousMagnitude;
    var other_361: HomogeneousMagnitude;

    self_431 = self_430;
    other_361 = other_360;
    let _e4: HomogeneousMagnitude = self_431;
    let _e8: HomogeneousMagnitude = other_361;
    let _e11: HomogeneousMagnitude = self_431;
    let _e15: HomogeneousMagnitude = other_361;
    return HomogeneousMagnitude(((vec2<f32>(_e4.g0_.y) * _e8.g0_) + ((vec2<f32>(_e11.g0_.x) * _e15.g0_.yx) * vec2<f32>(1.0, 0.0))));
}

fn homogeneous_magnitude_homogeneous_magnitude_inner_anti_product(self_432: HomogeneousMagnitude, other_362: HomogeneousMagnitude) -> HomogeneousMagnitude {
    var self_433: HomogeneousMagnitude;
    var other_363: HomogeneousMagnitude;

    self_433 = self_432;
    other_363 = other_362;
    let _e4: HomogeneousMagnitude = self_433;
    let _e8: HomogeneousMagnitude = other_363;
    let _e11: HomogeneousMagnitude = self_433;
    let _e15: HomogeneousMagnitude = other_363;
    return HomogeneousMagnitude(((vec2<f32>(_e4.g0_.y) * _e8.g0_) + ((vec2<f32>(_e11.g0_.x) * _e15.g0_.yx) * vec2<f32>(1.0, 0.0))));
}

fn homogeneous_magnitude_homogeneous_magnitude_left_contraction(self_434: HomogeneousMagnitude, other_364: HomogeneousMagnitude) -> HomogeneousMagnitude {
    var self_435: HomogeneousMagnitude;
    var other_365: HomogeneousMagnitude;

    self_435 = self_434;
    other_365 = other_364;
    let _e4: HomogeneousMagnitude = self_435;
    let _e8: HomogeneousMagnitude = other_365;
    return HomogeneousMagnitude((vec2<f32>(_e4.g0_.x) * _e8.g0_));
}

fn homogeneous_magnitude_homogeneous_magnitude_right_contraction(self_436: HomogeneousMagnitude, other_366: HomogeneousMagnitude) -> HomogeneousMagnitude {
    var self_437: HomogeneousMagnitude;
    var other_367: HomogeneousMagnitude;

    self_437 = self_436;
    other_367 = other_366;
    let _e4: HomogeneousMagnitude = self_437;
    let _e6: HomogeneousMagnitude = other_367;
    return HomogeneousMagnitude((_e4.g0_ * vec2<f32>(_e6.g0_.x)));
}

fn homogeneous_magnitude_homogeneous_magnitude_left_anti_contraction(self_438: HomogeneousMagnitude, other_368: HomogeneousMagnitude) -> HomogeneousMagnitude {
    var self_439: HomogeneousMagnitude;
    var other_369: HomogeneousMagnitude;

    self_439 = self_438;
    other_369 = other_368;
    let _e4: HomogeneousMagnitude = self_439;
    let _e8: HomogeneousMagnitude = other_369;
    return HomogeneousMagnitude((vec2<f32>(_e4.g0_.y) * _e8.g0_));
}

fn homogeneous_magnitude_homogeneous_magnitude_right_anti_contraction(self_440: HomogeneousMagnitude, other_370: HomogeneousMagnitude) -> HomogeneousMagnitude {
    var self_441: HomogeneousMagnitude;
    var other_371: HomogeneousMagnitude;

    self_441 = self_440;
    other_371 = other_370;
    let _e4: HomogeneousMagnitude = self_441;
    let _e6: HomogeneousMagnitude = other_371;
    return HomogeneousMagnitude((_e4.g0_ * vec2<f32>(_e6.g0_.y)));
}

fn homogeneous_magnitude_homogeneous_magnitude_scalar_product(self_442: HomogeneousMagnitude, other_372: HomogeneousMagnitude) -> Scalar {
    var self_443: HomogeneousMagnitude;
    var other_373: HomogeneousMagnitude;

    self_443 = self_442;
    other_373 = other_372;
    let _e4: HomogeneousMagnitude = self_443;
    let _e7: HomogeneousMagnitude = other_373;
    return Scalar((_e4.g0_.x * _e7.g0_.x));
}

fn homogeneous_magnitude_homogeneous_magnitude_anti_scalar_product(self_444: HomogeneousMagnitude, other_374: HomogeneousMagnitude) -> AntiScalar {
    var self_445: HomogeneousMagnitude;
    var other_375: HomogeneousMagnitude;

    self_445 = self_444;
    other_375 = other_374;
    let _e4: HomogeneousMagnitude = self_445;
    let _e7: HomogeneousMagnitude = other_375;
    return AntiScalar((_e4.g0_.y * _e7.g0_.y));
}

fn homogeneous_magnitude_point_regressive_product(self_446: HomogeneousMagnitude, other_376: Point) -> Point {
    var self_447: HomogeneousMagnitude;
    var other_377: Point;

    self_447 = self_446;
    other_377 = other_376;
    let _e4: HomogeneousMagnitude = self_447;
    let _e8: Point = other_377;
    return Point((vec4<f32>(_e4.g0_.y) * _e8.g0_));
}

fn homogeneous_magnitude_point_outer_product(self_448: HomogeneousMagnitude, other_378: Point) -> Point {
    var self_449: HomogeneousMagnitude;
    var other_379: Point;

    self_449 = self_448;
    other_379 = other_378;
    let _e4: HomogeneousMagnitude = self_449;
    let _e8: Point = other_379;
    return Point((vec4<f32>(_e4.g0_.x) * _e8.g0_));
}

fn homogeneous_magnitude_point_left_contraction(self_450: HomogeneousMagnitude, other_380: Point) -> Point {
    var self_451: HomogeneousMagnitude;
    var other_381: Point;

    self_451 = self_450;
    other_381 = other_380;
    let _e4: HomogeneousMagnitude = self_451;
    let _e8: Point = other_381;
    return Point((vec4<f32>(_e4.g0_.x) * _e8.g0_));
}

fn homogeneous_magnitude_point_left_anti_contraction(self_452: HomogeneousMagnitude, other_382: Point) -> Point {
    var self_453: HomogeneousMagnitude;
    var other_383: Point;

    self_453 = self_452;
    other_383 = other_382;
    let _e4: HomogeneousMagnitude = self_453;
    let _e8: Point = other_383;
    return Point((vec4<f32>(_e4.g0_.y) * _e8.g0_));
}

fn homogeneous_magnitude_line_geometric_product(self_454: HomogeneousMagnitude, other_384: Line) -> Line {
    var self_455: HomogeneousMagnitude;
    var other_385: Line;

    self_455 = self_454;
    other_385 = other_384;
    let _e4: HomogeneousMagnitude = self_455;
    let _e8: Line = other_385;
    let _e11: HomogeneousMagnitude = self_455;
    let _e15: Line = other_385;
    let _e19: HomogeneousMagnitude = self_455;
    let _e23: Line = other_385;
    return Line(((vec3<f32>(_e4.g0_.x) * _e8.g0_) + (vec3<f32>(_e11.g0_.y) * _e15.g1_)), (vec3<f32>(_e19.g0_.x) * _e23.g1_));
}

fn homogeneous_magnitude_line_regressive_product(self_456: HomogeneousMagnitude, other_386: Line) -> Line {
    var self_457: HomogeneousMagnitude;
    var other_387: Line;

    self_457 = self_456;
    other_387 = other_386;
    let _e4: HomogeneousMagnitude = self_457;
    let _e8: Line = other_387;
    let _e11: HomogeneousMagnitude = self_457;
    let _e15: Line = other_387;
    return Line((vec3<f32>(_e4.g0_.y) * _e8.g0_), (vec3<f32>(_e11.g0_.y) * _e15.g1_));
}

fn homogeneous_magnitude_line_outer_product(self_458: HomogeneousMagnitude, other_388: Line) -> Line {
    var self_459: HomogeneousMagnitude;
    var other_389: Line;

    self_459 = self_458;
    other_389 = other_388;
    let _e4: HomogeneousMagnitude = self_459;
    let _e8: Line = other_389;
    let _e11: HomogeneousMagnitude = self_459;
    let _e15: Line = other_389;
    return Line((vec3<f32>(_e4.g0_.x) * _e8.g0_), (vec3<f32>(_e11.g0_.x) * _e15.g1_));
}

fn homogeneous_magnitude_line_inner_product(self_460: HomogeneousMagnitude, other_390: Line) -> Line {
    var self_461: HomogeneousMagnitude;
    var other_391: Line;

    self_461 = self_460;
    other_391 = other_390;
    let _e4: HomogeneousMagnitude = self_461;
    let _e8: Line = other_391;
    let _e11: HomogeneousMagnitude = self_461;
    let _e15: Line = other_391;
    let _e19: HomogeneousMagnitude = self_461;
    let _e23: Line = other_391;
    return Line(((vec3<f32>(_e4.g0_.x) * _e8.g0_) + (vec3<f32>(_e11.g0_.y) * _e15.g1_)), (vec3<f32>(_e19.g0_.x) * _e23.g1_));
}

fn homogeneous_magnitude_line_geometric_anti_product(self_462: HomogeneousMagnitude, other_392: Line) -> Line {
    var self_463: HomogeneousMagnitude;
    var other_393: Line;

    self_463 = self_462;
    other_393 = other_392;
    let _e4: HomogeneousMagnitude = self_463;
    let _e8: Line = other_393;
    let _e11: HomogeneousMagnitude = self_463;
    let _e15: Line = other_393;
    let _e18: HomogeneousMagnitude = self_463;
    let _e22: Line = other_393;
    return Line((vec3<f32>(_e4.g0_.y) * _e8.g0_), ((vec3<f32>(_e11.g0_.x) * _e15.g0_) + (vec3<f32>(_e18.g0_.y) * _e22.g1_)));
}

fn homogeneous_magnitude_line_inner_anti_product(self_464: HomogeneousMagnitude, other_394: Line) -> Line {
    var self_465: HomogeneousMagnitude;
    var other_395: Line;

    self_465 = self_464;
    other_395 = other_394;
    let _e4: HomogeneousMagnitude = self_465;
    let _e8: Line = other_395;
    let _e11: HomogeneousMagnitude = self_465;
    let _e15: Line = other_395;
    let _e18: HomogeneousMagnitude = self_465;
    let _e22: Line = other_395;
    return Line((vec3<f32>(_e4.g0_.y) * _e8.g0_), ((vec3<f32>(_e11.g0_.x) * _e15.g0_) + (vec3<f32>(_e18.g0_.y) * _e22.g1_)));
}

fn homogeneous_magnitude_line_left_contraction(self_466: HomogeneousMagnitude, other_396: Line) -> Line {
    var self_467: HomogeneousMagnitude;
    var other_397: Line;

    self_467 = self_466;
    other_397 = other_396;
    let _e4: HomogeneousMagnitude = self_467;
    let _e8: Line = other_397;
    let _e11: HomogeneousMagnitude = self_467;
    let _e15: Line = other_397;
    return Line((vec3<f32>(_e4.g0_.x) * _e8.g0_), (vec3<f32>(_e11.g0_.x) * _e15.g1_));
}

fn homogeneous_magnitude_line_left_anti_contraction(self_468: HomogeneousMagnitude, other_398: Line) -> Line {
    var self_469: HomogeneousMagnitude;
    var other_399: Line;

    self_469 = self_468;
    other_399 = other_398;
    let _e4: HomogeneousMagnitude = self_469;
    let _e8: Line = other_399;
    let _e11: HomogeneousMagnitude = self_469;
    let _e15: Line = other_399;
    return Line((vec3<f32>(_e4.g0_.y) * _e8.g0_), (vec3<f32>(_e11.g0_.y) * _e15.g1_));
}

fn homogeneous_magnitude_plane_regressive_product(self_470: HomogeneousMagnitude, other_400: Plane) -> Plane {
    var self_471: HomogeneousMagnitude;
    var other_401: Plane;

    self_471 = self_470;
    other_401 = other_400;
    let _e4: HomogeneousMagnitude = self_471;
    let _e8: Plane = other_401;
    return Plane((vec4<f32>(_e4.g0_.y) * _e8.g0_));
}

fn homogeneous_magnitude_plane_outer_product(self_472: HomogeneousMagnitude, other_402: Plane) -> Plane {
    var self_473: HomogeneousMagnitude;
    var other_403: Plane;

    self_473 = self_472;
    other_403 = other_402;
    let _e4: HomogeneousMagnitude = self_473;
    let _e8: Plane = other_403;
    return Plane((vec4<f32>(_e4.g0_.x) * _e8.g0_));
}

fn homogeneous_magnitude_plane_left_contraction(self_474: HomogeneousMagnitude, other_404: Plane) -> Plane {
    var self_475: HomogeneousMagnitude;
    var other_405: Plane;

    self_475 = self_474;
    other_405 = other_404;
    let _e4: HomogeneousMagnitude = self_475;
    let _e8: Plane = other_405;
    return Plane((vec4<f32>(_e4.g0_.x) * _e8.g0_));
}

fn homogeneous_magnitude_plane_left_anti_contraction(self_476: HomogeneousMagnitude, other_406: Plane) -> Plane {
    var self_477: HomogeneousMagnitude;
    var other_407: Plane;

    self_477 = self_476;
    other_407 = other_406;
    let _e4: HomogeneousMagnitude = self_477;
    let _e8: Plane = other_407;
    return Plane((vec4<f32>(_e4.g0_.y) * _e8.g0_));
}

fn homogeneous_magnitude_motor_geometric_product(self_478: HomogeneousMagnitude, other_408: Motor) -> Motor {
    var self_479: HomogeneousMagnitude;
    var other_409: Motor;

    self_479 = self_478;
    other_409 = other_408;
    let _e4: HomogeneousMagnitude = self_479;
    let _e8: Motor = other_409;
    let _e11: HomogeneousMagnitude = self_479;
    let _e14: HomogeneousMagnitude = self_479;
    let _e17: HomogeneousMagnitude = self_479;
    let _e20: HomogeneousMagnitude = self_479;
    let _e24: Motor = other_409;
    let _e27: Motor = other_409;
    let _e30: Motor = other_409;
    let _e33: Motor = other_409;
    let _e45: HomogeneousMagnitude = self_479;
    let _e49: Motor = other_409;
    return Motor(((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.y, _e14.g0_.y, _e17.g0_.y, _e20.g0_.x) * vec4<f32>(_e24.g1_.x, _e27.g1_.y, _e30.g1_.z, _e33.g1_.x)) * vec4<f32>(1.0, 1.0, 1.0, 0.0))), (vec3<f32>(_e45.g0_.x) * _e49.g1_));
}

fn homogeneous_magnitude_motor_outer_product(self_480: HomogeneousMagnitude, other_410: Motor) -> Motor {
    var self_481: HomogeneousMagnitude;
    var other_411: Motor;

    self_481 = self_480;
    other_411 = other_410;
    let _e4: HomogeneousMagnitude = self_481;
    let _e8: Motor = other_411;
    let _e11: HomogeneousMagnitude = self_481;
    let _e15: Motor = other_411;
    return Motor((vec4<f32>(_e4.g0_.x) * _e8.g0_), (vec3<f32>(_e11.g0_.x) * _e15.g1_));
}

fn homogeneous_magnitude_motor_inner_product(self_482: HomogeneousMagnitude, other_412: Motor) -> Motor {
    var self_483: HomogeneousMagnitude;
    var other_413: Motor;

    self_483 = self_482;
    other_413 = other_412;
    let _e4: HomogeneousMagnitude = self_483;
    let _e8: Motor = other_413;
    let _e11: HomogeneousMagnitude = self_483;
    let _e14: HomogeneousMagnitude = self_483;
    let _e17: HomogeneousMagnitude = self_483;
    let _e20: HomogeneousMagnitude = self_483;
    let _e24: Motor = other_413;
    let _e27: Motor = other_413;
    let _e30: Motor = other_413;
    let _e33: Motor = other_413;
    let _e45: HomogeneousMagnitude = self_483;
    let _e49: Motor = other_413;
    return Motor(((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.y, _e14.g0_.y, _e17.g0_.y, _e20.g0_.x) * vec4<f32>(_e24.g1_.x, _e27.g1_.y, _e30.g1_.z, _e33.g1_.x)) * vec4<f32>(1.0, 1.0, 1.0, 0.0))), (vec3<f32>(_e45.g0_.x) * _e49.g1_));
}

fn homogeneous_magnitude_motor_left_contraction(self_484: HomogeneousMagnitude, other_414: Motor) -> Motor {
    var self_485: HomogeneousMagnitude;
    var other_415: Motor;

    self_485 = self_484;
    other_415 = other_414;
    let _e4: HomogeneousMagnitude = self_485;
    let _e8: Motor = other_415;
    let _e11: HomogeneousMagnitude = self_485;
    let _e15: Motor = other_415;
    return Motor((vec4<f32>(_e4.g0_.x) * _e8.g0_), (vec3<f32>(_e11.g0_.x) * _e15.g1_));
}

fn homogeneous_magnitude_motor_left_anti_contraction(self_486: HomogeneousMagnitude, other_416: Motor) -> Motor {
    var self_487: HomogeneousMagnitude;
    var other_417: Motor;

    self_487 = self_486;
    other_417 = other_416;
    let _e4: HomogeneousMagnitude = self_487;
    let _e8: Motor = other_417;
    let _e11: HomogeneousMagnitude = self_487;
    let _e15: Motor = other_417;
    return Motor((vec4<f32>(_e4.g0_.y) * _e8.g0_), (vec3<f32>(_e11.g0_.y) * _e15.g1_));
}

fn homogeneous_magnitude_motor_anti_scalar_product(self_488: HomogeneousMagnitude, other_418: Motor) -> AntiScalar {
    var self_489: HomogeneousMagnitude;
    var other_419: Motor;

    self_489 = self_488;
    other_419 = other_418;
    let _e4: HomogeneousMagnitude = self_489;
    let _e7: Motor = other_419;
    return AntiScalar((_e4.g0_.y * _e7.g0_.w));
}

fn homogeneous_magnitude_rotor_geometric_product(self_490: HomogeneousMagnitude, other_420: Rotor) -> Rotor {
    var self_491: HomogeneousMagnitude;
    var other_421: Rotor;

    self_491 = self_490;
    other_421 = other_420;
    let _e4: HomogeneousMagnitude = self_491;
    let _e8: Rotor = other_421;
    return Rotor((vec4<f32>(_e4.g0_.x) * _e8.g0_));
}

fn homogeneous_magnitude_rotor_outer_product(self_492: HomogeneousMagnitude, other_422: Rotor) -> Rotor {
    var self_493: HomogeneousMagnitude;
    var other_423: Rotor;

    self_493 = self_492;
    other_423 = other_422;
    let _e4: HomogeneousMagnitude = self_493;
    let _e8: Rotor = other_423;
    return Rotor((vec4<f32>(_e4.g0_.x) * _e8.g0_));
}

fn homogeneous_magnitude_rotor_inner_product(self_494: HomogeneousMagnitude, other_424: Rotor) -> Rotor {
    var self_495: HomogeneousMagnitude;
    var other_425: Rotor;

    self_495 = self_494;
    other_425 = other_424;
    let _e4: HomogeneousMagnitude = self_495;
    let _e8: Rotor = other_425;
    return Rotor((vec4<f32>(_e4.g0_.x) * _e8.g0_));
}

fn homogeneous_magnitude_rotor_left_contraction(self_496: HomogeneousMagnitude, other_426: Rotor) -> Rotor {
    var self_497: HomogeneousMagnitude;
    var other_427: Rotor;

    self_497 = self_496;
    other_427 = other_426;
    let _e4: HomogeneousMagnitude = self_497;
    let _e8: Rotor = other_427;
    return Rotor((vec4<f32>(_e4.g0_.x) * _e8.g0_));
}

fn homogeneous_magnitude_rotor_left_anti_contraction(self_498: HomogeneousMagnitude, other_428: Rotor) -> Rotor {
    var self_499: HomogeneousMagnitude;
    var other_429: Rotor;

    self_499 = self_498;
    other_429 = other_428;
    let _e4: HomogeneousMagnitude = self_499;
    let _e8: Rotor = other_429;
    return Rotor((vec4<f32>(_e4.g0_.y) * _e8.g0_));
}

fn homogeneous_magnitude_rotor_anti_scalar_product(self_500: HomogeneousMagnitude, other_430: Rotor) -> AntiScalar {
    var self_501: HomogeneousMagnitude;
    var other_431: Rotor;

    self_501 = self_500;
    other_431 = other_430;
    let _e4: HomogeneousMagnitude = self_501;
    let _e7: Rotor = other_431;
    return AntiScalar((_e4.g0_.y * _e7.g0_.w));
}

fn homogeneous_magnitude_translator_geometric_product(self_502: HomogeneousMagnitude, other_432: Translator) -> Motor {
    var self_503: HomogeneousMagnitude;
    var other_433: Translator;

    self_503 = self_502;
    other_433 = other_432;
    let _e4: HomogeneousMagnitude = self_503;
    let _e7: HomogeneousMagnitude = self_503;
    let _e10: HomogeneousMagnitude = self_503;
    let _e13: HomogeneousMagnitude = self_503;
    let _e17: Translator = other_433;
    let _e20: HomogeneousMagnitude = self_503;
    let _e24: Translator = other_433;
    let _e27: Translator = other_433;
    let _e30: Translator = other_433;
    return Motor((vec4<f32>(_e4.g0_.y, _e7.g0_.y, _e10.g0_.y, _e13.g0_.x) * _e17.g0_), (vec3<f32>(_e20.g0_.x) * vec3<f32>(_e24.g0_.x, _e27.g0_.y, _e30.g0_.z)));
}

fn homogeneous_magnitude_translator_outer_product(self_504: HomogeneousMagnitude, other_434: Translator) -> Translator {
    var self_505: HomogeneousMagnitude;
    var other_435: Translator;

    self_505 = self_504;
    other_435 = other_434;
    let _e4: HomogeneousMagnitude = self_505;
    let _e8: Translator = other_435;
    return Translator((vec4<f32>(_e4.g0_.x) * _e8.g0_));
}

fn homogeneous_magnitude_translator_inner_product(self_506: HomogeneousMagnitude, other_436: Translator) -> Motor {
    var self_507: HomogeneousMagnitude;
    var other_437: Translator;

    self_507 = self_506;
    other_437 = other_436;
    let _e4: HomogeneousMagnitude = self_507;
    let _e7: HomogeneousMagnitude = self_507;
    let _e10: HomogeneousMagnitude = self_507;
    let _e13: HomogeneousMagnitude = self_507;
    let _e17: Translator = other_437;
    let _e20: HomogeneousMagnitude = self_507;
    let _e24: Translator = other_437;
    let _e27: Translator = other_437;
    let _e30: Translator = other_437;
    return Motor((vec4<f32>(_e4.g0_.y, _e7.g0_.y, _e10.g0_.y, _e13.g0_.x) * _e17.g0_), (vec3<f32>(_e20.g0_.x) * vec3<f32>(_e24.g0_.x, _e27.g0_.y, _e30.g0_.z)));
}

fn homogeneous_magnitude_translator_left_contraction(self_508: HomogeneousMagnitude, other_438: Translator) -> Translator {
    var self_509: HomogeneousMagnitude;
    var other_439: Translator;

    self_509 = self_508;
    other_439 = other_438;
    let _e4: HomogeneousMagnitude = self_509;
    let _e8: Translator = other_439;
    return Translator((vec4<f32>(_e4.g0_.x) * _e8.g0_));
}

fn homogeneous_magnitude_translator_left_anti_contraction(self_510: HomogeneousMagnitude, other_440: Translator) -> Translator {
    var self_511: HomogeneousMagnitude;
    var other_441: Translator;

    self_511 = self_510;
    other_441 = other_440;
    let _e4: HomogeneousMagnitude = self_511;
    let _e8: Translator = other_441;
    return Translator((vec4<f32>(_e4.g0_.y) * _e8.g0_));
}

fn homogeneous_magnitude_translator_right_anti_contraction(self_512: HomogeneousMagnitude, other_442: Translator) -> HomogeneousMagnitude {
    var self_513: HomogeneousMagnitude;
    var other_443: Translator;

    self_513 = self_512;
    other_443 = other_442;
    let _e4: HomogeneousMagnitude = self_513;
    let _e6: Translator = other_443;
    return HomogeneousMagnitude((_e4.g0_ * vec2<f32>(_e6.g0_.w)));
}

fn homogeneous_magnitude_translator_anti_scalar_product(self_514: HomogeneousMagnitude, other_444: Translator) -> AntiScalar {
    var self_515: HomogeneousMagnitude;
    var other_445: Translator;

    self_515 = self_514;
    other_445 = other_444;
    let _e4: HomogeneousMagnitude = self_515;
    let _e7: Translator = other_445;
    return AntiScalar((_e4.g0_.y * _e7.g0_.w));
}

fn homogeneous_magnitude_flector_geometric_product(self_516: HomogeneousMagnitude, other_446: Flector) -> Flector {
    var self_517: HomogeneousMagnitude;
    var other_447: Flector;

    self_517 = self_516;
    other_447 = other_446;
    let _e4: HomogeneousMagnitude = self_517;
    let _e8: Flector = other_447;
    let _e11: HomogeneousMagnitude = self_517;
    let _e14: HomogeneousMagnitude = self_517;
    let _e17: HomogeneousMagnitude = self_517;
    let _e20: HomogeneousMagnitude = self_517;
    let _e24: Flector = other_447;
    let _e36: HomogeneousMagnitude = self_517;
    let _e40: Flector = other_447;
    let _e43: HomogeneousMagnitude = self_517;
    let _e46: HomogeneousMagnitude = self_517;
    let _e49: HomogeneousMagnitude = self_517;
    let _e52: HomogeneousMagnitude = self_517;
    let _e56: Flector = other_447;
    return Flector(((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.x, _e14.g0_.x, _e17.g0_.x, _e20.g0_.y) * _e24.g1_.xxxw) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))), ((vec4<f32>(_e36.g0_.x) * _e40.g1_) + ((vec4<f32>(_e43.g0_.y, _e46.g0_.y, _e49.g0_.y, _e52.g0_.x) * _e56.g0_.xyzx) * vec4<f32>(-(1.0), -(1.0), -(1.0), 0.0))));
}

fn homogeneous_magnitude_flector_regressive_product(self_518: HomogeneousMagnitude, other_448: Flector) -> Flector {
    var self_519: HomogeneousMagnitude;
    var other_449: Flector;

    self_519 = self_518;
    other_449 = other_448;
    let _e4: HomogeneousMagnitude = self_519;
    let _e8: Flector = other_449;
    let _e11: HomogeneousMagnitude = self_519;
    let _e15: Flector = other_449;
    return Flector((vec4<f32>(_e4.g0_.y) * _e8.g0_), (vec4<f32>(_e11.g0_.y) * _e15.g1_));
}

fn homogeneous_magnitude_flector_outer_product(self_520: HomogeneousMagnitude, other_450: Flector) -> Flector {
    var self_521: HomogeneousMagnitude;
    var other_451: Flector;

    self_521 = self_520;
    other_451 = other_450;
    let _e4: HomogeneousMagnitude = self_521;
    let _e8: Flector = other_451;
    let _e11: HomogeneousMagnitude = self_521;
    let _e15: Flector = other_451;
    return Flector((vec4<f32>(_e4.g0_.x) * _e8.g0_), (vec4<f32>(_e11.g0_.x) * _e15.g1_));
}

fn homogeneous_magnitude_flector_inner_product(self_522: HomogeneousMagnitude, other_452: Flector) -> Flector {
    var self_523: HomogeneousMagnitude;
    var other_453: Flector;

    self_523 = self_522;
    other_453 = other_452;
    let _e4: HomogeneousMagnitude = self_523;
    let _e8: Flector = other_453;
    let _e11: HomogeneousMagnitude = self_523;
    let _e14: HomogeneousMagnitude = self_523;
    let _e17: HomogeneousMagnitude = self_523;
    let _e20: HomogeneousMagnitude = self_523;
    let _e24: Flector = other_453;
    let _e36: HomogeneousMagnitude = self_523;
    let _e40: Flector = other_453;
    let _e43: HomogeneousMagnitude = self_523;
    let _e46: HomogeneousMagnitude = self_523;
    let _e49: HomogeneousMagnitude = self_523;
    let _e52: HomogeneousMagnitude = self_523;
    let _e56: Flector = other_453;
    return Flector(((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.x, _e14.g0_.x, _e17.g0_.x, _e20.g0_.y) * _e24.g1_.xxxw) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))), ((vec4<f32>(_e36.g0_.x) * _e40.g1_) + ((vec4<f32>(_e43.g0_.y, _e46.g0_.y, _e49.g0_.y, _e52.g0_.x) * _e56.g0_.xyzx) * vec4<f32>(-(1.0), -(1.0), -(1.0), 0.0))));
}

fn homogeneous_magnitude_flector_geometric_anti_product(self_524: HomogeneousMagnitude, other_454: Flector) -> Flector {
    var self_525: HomogeneousMagnitude;
    var other_455: Flector;

    self_525 = self_524;
    other_455 = other_454;
    let _e4: HomogeneousMagnitude = self_525;
    let _e8: Flector = other_455;
    let _e11: HomogeneousMagnitude = self_525;
    let _e15: Flector = other_455;
    let _e26: HomogeneousMagnitude = self_525;
    let _e30: Flector = other_455;
    let _e33: HomogeneousMagnitude = self_525;
    let _e37: Flector = other_455;
    return Flector(((vec4<f32>(_e4.g0_.y) * _e8.g0_) + ((vec4<f32>(_e11.g0_.x) * _e15.g1_.xyzx) * vec4<f32>(1.0, 1.0, 1.0, 0.0))), ((vec4<f32>(_e26.g0_.y) * _e30.g1_) + ((vec4<f32>(_e33.g0_.x) * _e37.g0_.xxxw) * vec4<f32>(0.0, 0.0, 0.0, 1.0))));
}

fn homogeneous_magnitude_flector_inner_anti_product(self_526: HomogeneousMagnitude, other_456: Flector) -> Flector {
    var self_527: HomogeneousMagnitude;
    var other_457: Flector;

    self_527 = self_526;
    other_457 = other_456;
    let _e4: HomogeneousMagnitude = self_527;
    let _e8: Flector = other_457;
    let _e11: HomogeneousMagnitude = self_527;
    let _e15: Flector = other_457;
    let _e26: HomogeneousMagnitude = self_527;
    let _e30: Flector = other_457;
    let _e33: HomogeneousMagnitude = self_527;
    let _e37: Flector = other_457;
    return Flector(((vec4<f32>(_e4.g0_.y) * _e8.g0_) + ((vec4<f32>(_e11.g0_.x) * _e15.g1_.xyzx) * vec4<f32>(1.0, 1.0, 1.0, 0.0))), ((vec4<f32>(_e26.g0_.y) * _e30.g1_) + ((vec4<f32>(_e33.g0_.x) * _e37.g0_.xxxw) * vec4<f32>(0.0, 0.0, 0.0, 1.0))));
}

fn homogeneous_magnitude_flector_left_contraction(self_528: HomogeneousMagnitude, other_458: Flector) -> Flector {
    var self_529: HomogeneousMagnitude;
    var other_459: Flector;

    self_529 = self_528;
    other_459 = other_458;
    let _e4: HomogeneousMagnitude = self_529;
    let _e8: Flector = other_459;
    let _e11: HomogeneousMagnitude = self_529;
    let _e15: Flector = other_459;
    return Flector((vec4<f32>(_e4.g0_.x) * _e8.g0_), (vec4<f32>(_e11.g0_.x) * _e15.g1_));
}

fn homogeneous_magnitude_flector_left_anti_contraction(self_530: HomogeneousMagnitude, other_460: Flector) -> Flector {
    var self_531: HomogeneousMagnitude;
    var other_461: Flector;

    self_531 = self_530;
    other_461 = other_460;
    let _e4: HomogeneousMagnitude = self_531;
    let _e8: Flector = other_461;
    let _e11: HomogeneousMagnitude = self_531;
    let _e15: Flector = other_461;
    return Flector((vec4<f32>(_e4.g0_.y) * _e8.g0_), (vec4<f32>(_e11.g0_.y) * _e15.g1_));
}

fn homogeneous_magnitude_multi_vector_scalar_product(self_532: HomogeneousMagnitude, other_462: MultiVector) -> Scalar {
    var self_533: HomogeneousMagnitude;
    var other_463: MultiVector;

    self_533 = self_532;
    other_463 = other_462;
    let _e4: HomogeneousMagnitude = self_533;
    let _e7: MultiVector = other_463;
    return Scalar((_e4.g0_.x * _e7.g0_.x));
}

fn homogeneous_magnitude_multi_vector_anti_scalar_product(self_534: HomogeneousMagnitude, other_464: MultiVector) -> AntiScalar {
    var self_535: HomogeneousMagnitude;
    var other_465: MultiVector;

    self_535 = self_534;
    other_465 = other_464;
    let _e4: HomogeneousMagnitude = self_535;
    let _e7: MultiVector = other_465;
    return AntiScalar((_e4.g0_.y * _e7.g0_.y));
}

fn homogeneous_magnitude_squared_magnitude(self_536: HomogeneousMagnitude) -> Scalar {
    var self_537: HomogeneousMagnitude;

    self_537 = self_536;
    let _e2: HomogeneousMagnitude = self_537;
    let _e3: HomogeneousMagnitude = self_537;
    let _e4: HomogeneousMagnitude = homogeneous_magnitude_reversal(_e3);
    let _e5: Scalar = homogeneous_magnitude_homogeneous_magnitude_scalar_product(_e2, _e4);
    return _e5;
}

fn homogeneous_magnitude_magnitude(self_538: HomogeneousMagnitude) -> Scalar {
    var self_539: HomogeneousMagnitude;

    self_539 = self_538;
    let _e2: HomogeneousMagnitude = self_539;
    let _e3: Scalar = homogeneous_magnitude_squared_magnitude(_e2);
    return Scalar(sqrt(_e3.g0_));
}

fn homogeneous_magnitude_bulk_norm(self_540: HomogeneousMagnitude) -> Scalar {
    var self_541: HomogeneousMagnitude;

    self_541 = self_540;
    let _e2: HomogeneousMagnitude = self_541;
    let _e3: Scalar = homogeneous_magnitude_squared_magnitude(_e2);
    return Scalar(sqrt(_e3.g0_));
}

fn homogeneous_magnitude_squared_anti_magnitude(self_542: HomogeneousMagnitude) -> AntiScalar {
    var self_543: HomogeneousMagnitude;

    self_543 = self_542;
    let _e2: HomogeneousMagnitude = self_543;
    let _e3: HomogeneousMagnitude = self_543;
    let _e4: HomogeneousMagnitude = homogeneous_magnitude_anti_reversal(_e3);
    let _e5: AntiScalar = homogeneous_magnitude_homogeneous_magnitude_anti_scalar_product(_e2, _e4);
    return _e5;
}

fn homogeneous_magnitude_weight_norm(self_544: HomogeneousMagnitude) -> AntiScalar {
    var self_545: HomogeneousMagnitude;

    self_545 = self_544;
    let _e2: HomogeneousMagnitude = self_545;
    let _e3: AntiScalar = homogeneous_magnitude_squared_anti_magnitude(_e2);
    return AntiScalar(sqrt(_e3.g0_));
}

fn homogeneous_magnitude_geometric_norm(self_546: HomogeneousMagnitude) -> HomogeneousMagnitude {
    var self_547: HomogeneousMagnitude;

    self_547 = self_546;
    let _e2: HomogeneousMagnitude = self_547;
    let _e3: Scalar = homogeneous_magnitude_bulk_norm(_e2);
    let _e4: HomogeneousMagnitude = self_547;
    let _e5: AntiScalar = homogeneous_magnitude_weight_norm(_e4);
    return (_e3 + _e5);
}

fn homogeneous_magnitude_scale(self_548: HomogeneousMagnitude, other_466: f32) -> HomogeneousMagnitude {
    var self_549: HomogeneousMagnitude;
    var other_467: f32;

    self_549 = self_548;
    other_467 = other_466;
    let _e4: HomogeneousMagnitude = self_549;
    let _e5: f32 = other_467;
    let _e7: HomogeneousMagnitude = homogeneous_magnitude_scalar_geometric_product(_e4, Scalar(_e5));
    return _e7;
}

fn homogeneous_magnitude_signum(self_550: HomogeneousMagnitude) -> HomogeneousMagnitude {
    var self_551: HomogeneousMagnitude;

    self_551 = self_550;
    let _e2: HomogeneousMagnitude = self_551;
    let _e3: HomogeneousMagnitude = self_551;
    let _e4: Scalar = homogeneous_magnitude_magnitude(_e3);
    let _e9: HomogeneousMagnitude = homogeneous_magnitude_scalar_geometric_product(_e2, Scalar((1.0 / _e4.g0_)));
    return _e9;
}

fn homogeneous_magnitude_inverse(self_552: HomogeneousMagnitude) -> HomogeneousMagnitude {
    var self_553: HomogeneousMagnitude;

    self_553 = self_552;
    let _e2: HomogeneousMagnitude = self_553;
    let _e3: HomogeneousMagnitude = homogeneous_magnitude_reversal(_e2);
    let _e4: HomogeneousMagnitude = self_553;
    let _e5: Scalar = homogeneous_magnitude_squared_magnitude(_e4);
    let _e10: HomogeneousMagnitude = homogeneous_magnitude_scalar_geometric_product(_e3, Scalar((1.0 / _e5.g0_)));
    return _e10;
}

fn homogeneous_magnitude_unitize(self_554: HomogeneousMagnitude) -> HomogeneousMagnitude {
    var self_555: HomogeneousMagnitude;

    self_555 = self_554;
    let _e2: HomogeneousMagnitude = self_555;
    let _e3: HomogeneousMagnitude = self_555;
    let _e4: AntiScalar = homogeneous_magnitude_weight_norm(_e3);
    let _e9: HomogeneousMagnitude = homogeneous_magnitude_scalar_geometric_product(_e2, Scalar((1.0 / _e4.g0_)));
    return _e9;
}

fn point_zero() -> Point {
    return Point(vec4<f32>(0.0));
}

fn point_one() -> Point {
    return Point(vec4<f32>(0.0));
}

fn point_grade(self_556: Point) -> i32 {
    return 1;
}

fn point_anti_grade(self_557: Point) -> i32 {
    return 3;
}

fn point_neg(self_558: Point) -> Point {
    var self_559: Point;

    self_559 = self_558;
    let _e2: Point = self_559;
    return Point((_e2.g0_ * vec4<f32>(-(1.0))));
}

fn point_automorphism(self_560: Point) -> Point {
    var self_561: Point;

    self_561 = self_560;
    let _e2: Point = self_561;
    return Point((_e2.g0_ * vec4<f32>(-(1.0))));
}

fn point_reversal(self_562: Point) -> Point {
    var self_563: Point;

    self_563 = self_562;
    let _e2: Point = self_563;
    return Point(_e2.g0_);
}

fn point_conjugation(self_564: Point) -> Point {
    var self_565: Point;

    self_565 = self_564;
    let _e2: Point = self_565;
    return Point((_e2.g0_ * vec4<f32>(-(1.0))));
}

fn point_dual(self_566: Point) -> Plane {
    var self_567: Point;

    self_567 = self_566;
    let _e2: Point = self_567;
    return Plane(_e2.g0_);
}

fn point_anti_reversal(self_568: Point) -> Point {
    var self_569: Point;

    self_569 = self_568;
    let _e2: Point = self_569;
    return Point((_e2.g0_ * vec4<f32>(-(1.0))));
}

fn point_right_complement(self_570: Point) -> Plane {
    var self_571: Point;

    self_571 = self_570;
    let _e2: Point = self_571;
    return Plane(_e2.g0_);
}

fn point_left_complement(self_572: Point) -> Plane {
    var self_573: Point;

    self_573 = self_572;
    let _e2: Point = self_573;
    return Plane((_e2.g0_ * vec4<f32>(-(1.0))));
}

fn point_double_complement(self_574: Point) -> Point {
    var self_575: Point;

    self_575 = self_574;
    let _e2: Point = self_575;
    return Point((_e2.g0_ * vec4<f32>(-(1.0))));
}

fn point_scalar_geometric_product(self_576: Point, other_468: Scalar) -> Point {
    var self_577: Point;
    var other_469: Scalar;

    self_577 = self_576;
    other_469 = other_468;
    let _e4: Point = self_577;
    let _e6: Scalar = other_469;
    return Point((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn point_scalar_outer_product(self_578: Point, other_470: Scalar) -> Point {
    var self_579: Point;
    var other_471: Scalar;

    self_579 = self_578;
    other_471 = other_470;
    let _e4: Point = self_579;
    let _e6: Scalar = other_471;
    return Point((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn point_scalar_inner_product(self_580: Point, other_472: Scalar) -> Point {
    var self_581: Point;
    var other_473: Scalar;

    self_581 = self_580;
    other_473 = other_472;
    let _e4: Point = self_581;
    let _e6: Scalar = other_473;
    return Point((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn point_scalar_right_contraction(self_582: Point, other_474: Scalar) -> Point {
    var self_583: Point;
    var other_475: Scalar;

    self_583 = self_582;
    other_475 = other_474;
    let _e4: Point = self_583;
    let _e6: Scalar = other_475;
    return Point((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn point_anti_scalar_regressive_product(self_584: Point, other_476: AntiScalar) -> Point {
    var self_585: Point;
    var other_477: AntiScalar;

    self_585 = self_584;
    other_477 = other_476;
    let _e4: Point = self_585;
    let _e6: AntiScalar = other_477;
    return Point((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn point_anti_scalar_geometric_anti_product(self_586: Point, other_478: AntiScalar) -> Point {
    var self_587: Point;
    var other_479: AntiScalar;

    self_587 = self_586;
    other_479 = other_478;
    let _e4: Point = self_587;
    let _e6: AntiScalar = other_479;
    return Point((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn point_anti_scalar_inner_anti_product(self_588: Point, other_480: AntiScalar) -> Point {
    var self_589: Point;
    var other_481: AntiScalar;

    self_589 = self_588;
    other_481 = other_480;
    let _e4: Point = self_589;
    let _e6: AntiScalar = other_481;
    return Point((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn point_anti_scalar_right_anti_contraction(self_590: Point, other_482: AntiScalar) -> Point {
    var self_591: Point;
    var other_483: AntiScalar;

    self_591 = self_590;
    other_483 = other_482;
    let _e4: Point = self_591;
    let _e6: AntiScalar = other_483;
    return Point((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn point_homogeneous_magnitude_regressive_product(self_592: Point, other_484: HomogeneousMagnitude) -> Point {
    var self_593: Point;
    var other_485: HomogeneousMagnitude;

    self_593 = self_592;
    other_485 = other_484;
    let _e4: Point = self_593;
    let _e6: HomogeneousMagnitude = other_485;
    return Point((_e4.g0_ * vec4<f32>(_e6.g0_.y)));
}

fn point_homogeneous_magnitude_outer_product(self_594: Point, other_486: HomogeneousMagnitude) -> Point {
    var self_595: Point;
    var other_487: HomogeneousMagnitude;

    self_595 = self_594;
    other_487 = other_486;
    let _e4: Point = self_595;
    let _e6: HomogeneousMagnitude = other_487;
    return Point((_e4.g0_ * vec4<f32>(_e6.g0_.x)));
}

fn point_homogeneous_magnitude_right_contraction(self_596: Point, other_488: HomogeneousMagnitude) -> Point {
    var self_597: Point;
    var other_489: HomogeneousMagnitude;

    self_597 = self_596;
    other_489 = other_488;
    let _e4: Point = self_597;
    let _e6: HomogeneousMagnitude = other_489;
    return Point((_e4.g0_ * vec4<f32>(_e6.g0_.x)));
}

fn point_homogeneous_magnitude_right_anti_contraction(self_598: Point, other_490: HomogeneousMagnitude) -> Point {
    var self_599: Point;
    var other_491: HomogeneousMagnitude;

    self_599 = self_598;
    other_491 = other_490;
    let _e4: Point = self_599;
    let _e6: HomogeneousMagnitude = other_491;
    return Point((_e4.g0_ * vec4<f32>(_e6.g0_.y)));
}

fn point_point_add(self_600: Point, other_492: Point) -> Point {
    var self_601: Point;
    var other_493: Point;

    self_601 = self_600;
    other_493 = other_492;
    let _e4: Point = self_601;
    let _e6: Point = other_493;
    return Point((_e4.g0_ + _e6.g0_));
}

fn point_point_sub(self_602: Point, other_494: Point) -> Point {
    var self_603: Point;
    var other_495: Point;

    self_603 = self_602;
    other_495 = other_494;
    let _e4: Point = self_603;
    let _e6: Point = other_495;
    return Point((_e4.g0_ - _e6.g0_));
}

fn point_point_mul(self_604: Point, other_496: Point) -> Point {
    var self_605: Point;
    var other_497: Point;

    self_605 = self_604;
    other_497 = other_496;
    let _e4: Point = self_605;
    let _e6: Point = other_497;
    return Point((_e4.g0_ * _e6.g0_));
}

fn point_point_div(self_606: Point, other_498: Point) -> Point {
    var self_607: Point;
    var other_499: Point;

    self_607 = self_606;
    other_499 = other_498;
    let _e4: Point = self_607;
    let _e7: Point = self_607;
    let _e10: Point = self_607;
    let _e13: Point = self_607;
    let _e23: Point = other_499;
    let _e26: Point = other_499;
    let _e29: Point = other_499;
    let _e32: Point = other_499;
    return Point((((vec4<f32>(_e4.g0_.x, _e7.g0_.y, _e10.g0_.z, _e13.g0_.w) * vec4<f32>(1.0, 1.0, 1.0, 1.0)) / vec4<f32>(_e23.g0_.x, _e26.g0_.y, _e29.g0_.z, _e32.g0_.w)) * vec4<f32>(1.0, 1.0, 1.0, 1.0)));
}

fn point_point_outer_product(self_608: Point, other_500: Point) -> Line {
    var self_609: Point;
    var other_501: Point;

    self_609 = self_608;
    other_501 = other_500;
    let _e4: Point = self_609;
    let _e8: Point = other_501;
    let _e11: Point = other_501;
    let _e14: Point = other_501;
    let _e19: Point = self_609;
    let _e22: Point = self_609;
    let _e25: Point = self_609;
    let _e29: Point = other_501;
    let _e39: Point = self_609;
    let _e43: Point = other_501;
    let _e46: Point = other_501;
    let _e49: Point = other_501;
    let _e60: Point = self_609;
    let _e64: Point = other_501;
    let _e67: Point = other_501;
    let _e70: Point = other_501;
    let _e82: Point = self_609;
    let _e86: Point = other_501;
    let _e89: Point = other_501;
    let _e92: Point = other_501;
    return Line(((vec3<f32>(_e4.g0_.w) * vec3<f32>(_e8.g0_.x, _e11.g0_.y, _e14.g0_.z)) + ((vec3<f32>(_e19.g0_.x, _e22.g0_.y, _e25.g0_.z) * vec3<f32>(_e29.g0_.w)) * vec3<f32>(-(1.0)))), ((((vec3<f32>(_e39.g0_.y) * vec3<f32>(_e43.g0_.z, _e46.g0_.z, _e49.g0_.x)) * vec3<f32>(1.0, 0.0, -(1.0))) + ((vec3<f32>(_e60.g0_.z) * vec3<f32>(_e64.g0_.y, _e67.g0_.x, _e70.g0_.y)) * vec3<f32>(-(1.0), 1.0, 0.0))) + ((vec3<f32>(_e82.g0_.x) * vec3<f32>(_e86.g0_.x, _e89.g0_.z, _e92.g0_.y)) * vec3<f32>(0.0, -(1.0), 1.0))));
}

fn point_point_inner_product(self_610: Point, other_502: Point) -> Scalar {
    var self_611: Point;
    var other_503: Point;

    self_611 = self_610;
    other_503 = other_502;
    let _e4: Point = self_611;
    let _e7: Point = other_503;
    let _e11: Point = self_611;
    let _e14: Point = other_503;
    let _e19: Point = self_611;
    let _e22: Point = other_503;
    return Scalar((((_e4.g0_.x * _e7.g0_.x) + (_e11.g0_.y * _e14.g0_.y)) + (_e19.g0_.z * _e22.g0_.z)));
}

fn point_point_geometric_anti_product(self_612: Point, other_504: Point) -> Translator {
    var self_613: Point;
    var other_505: Point;

    self_613 = self_612;
    other_505 = other_504;
    let _e4: Point = self_613;
    let _e8: Point = other_505;
    let _e18: Point = self_613;
    let _e21: Point = other_505;
    return Translator((((vec4<f32>(_e4.g0_.w) * _e8.g0_) * vec4<f32>(1.0, 1.0, 1.0, -(1.0))) + ((_e18.g0_.xyzx * _e21.g0_.wwwx) * vec4<f32>(-(1.0), -(1.0), -(1.0), 0.0))));
}

fn point_point_inner_anti_product(self_614: Point, other_506: Point) -> AntiScalar {
    var self_615: Point;
    var other_507: Point;

    self_615 = self_614;
    other_507 = other_506;
    let _e5: Point = self_615;
    let _e8: Point = other_507;
    return AntiScalar((0.0 - (_e5.g0_.w * _e8.g0_.w)));
}

fn point_point_left_contraction(self_616: Point, other_508: Point) -> Scalar {
    var self_617: Point;
    var other_509: Point;

    self_617 = self_616;
    other_509 = other_508;
    let _e4: Point = self_617;
    let _e7: Point = other_509;
    let _e11: Point = self_617;
    let _e14: Point = other_509;
    let _e19: Point = self_617;
    let _e22: Point = other_509;
    return Scalar((((_e4.g0_.x * _e7.g0_.x) + (_e11.g0_.y * _e14.g0_.y)) + (_e19.g0_.z * _e22.g0_.z)));
}

fn point_point_right_contraction(self_618: Point, other_510: Point) -> Scalar {
    var self_619: Point;
    var other_511: Point;

    self_619 = self_618;
    other_511 = other_510;
    let _e4: Point = self_619;
    let _e7: Point = other_511;
    let _e11: Point = self_619;
    let _e14: Point = other_511;
    let _e19: Point = self_619;
    let _e22: Point = other_511;
    return Scalar((((_e4.g0_.x * _e7.g0_.x) + (_e11.g0_.y * _e14.g0_.y)) + (_e19.g0_.z * _e22.g0_.z)));
}

fn point_point_left_anti_contraction(self_620: Point, other_512: Point) -> AntiScalar {
    var self_621: Point;
    var other_513: Point;

    self_621 = self_620;
    other_513 = other_512;
    let _e5: Point = self_621;
    let _e8: Point = other_513;
    return AntiScalar((0.0 - (_e5.g0_.w * _e8.g0_.w)));
}

fn point_point_right_anti_contraction(self_622: Point, other_514: Point) -> AntiScalar {
    var self_623: Point;
    var other_515: Point;

    self_623 = self_622;
    other_515 = other_514;
    let _e5: Point = self_623;
    let _e8: Point = other_515;
    return AntiScalar((0.0 - (_e5.g0_.w * _e8.g0_.w)));
}

fn point_point_scalar_product(self_624: Point, other_516: Point) -> Scalar {
    var self_625: Point;
    var other_517: Point;

    self_625 = self_624;
    other_517 = other_516;
    let _e4: Point = self_625;
    let _e7: Point = other_517;
    let _e11: Point = self_625;
    let _e14: Point = other_517;
    let _e19: Point = self_625;
    let _e22: Point = other_517;
    return Scalar((((_e4.g0_.x * _e7.g0_.x) + (_e11.g0_.y * _e14.g0_.y)) + (_e19.g0_.z * _e22.g0_.z)));
}

fn point_point_anti_scalar_product(self_626: Point, other_518: Point) -> AntiScalar {
    var self_627: Point;
    var other_519: Point;

    self_627 = self_626;
    other_519 = other_518;
    let _e5: Point = self_627;
    let _e8: Point = other_519;
    return AntiScalar((0.0 - (_e5.g0_.w * _e8.g0_.w)));
}

fn point_line_geometric_product(self_628: Point, other_520: Line) -> Flector {
    var self_629: Point;
    var other_521: Line;

    self_629 = self_628;
    other_521 = other_520;
    let _e4: Point = self_629;
    let _e8: Line = other_521;
    let _e11: Line = other_521;
    let _e14: Line = other_521;
    let _e17: Line = other_521;
    let _e30: Point = self_629;
    let _e34: Line = other_521;
    let _e37: Line = other_521;
    let _e40: Line = other_521;
    let _e43: Line = other_521;
    let _e57: Point = self_629;
    let _e61: Line = other_521;
    let _e64: Line = other_521;
    let _e67: Line = other_521;
    let _e70: Line = other_521;
    let _e84: Point = self_629;
    let _e88: Line = other_521;
    let _e91: Line = other_521;
    let _e94: Line = other_521;
    let _e97: Line = other_521;
    let _e110: Point = self_629;
    let _e114: Line = other_521;
    let _e117: Line = other_521;
    let _e120: Line = other_521;
    let _e123: Line = other_521;
    let _e137: Point = self_629;
    let _e141: Line = other_521;
    let _e144: Line = other_521;
    let _e147: Line = other_521;
    let _e150: Line = other_521;
    let _e162: Point = self_629;
    let _e166: Line = other_521;
    let _e169: Line = other_521;
    let _e172: Line = other_521;
    let _e175: Line = other_521;
    return Flector(((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g1_.z, _e11.g1_.z, _e14.g1_.x, _e17.g0_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e30.g0_.z) * vec4<f32>(_e34.g1_.y, _e37.g1_.x, _e40.g1_.y, _e43.g0_.z)) * vec4<f32>(1.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e57.g0_.x) * vec4<f32>(_e61.g1_.x, _e64.g1_.z, _e67.g1_.y, _e70.g0_.x)) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))), (((((vec4<f32>(_e84.g0_.y) * vec4<f32>(_e88.g0_.z, _e91.g0_.z, _e94.g0_.x, _e97.g1_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e110.g0_.z) * vec4<f32>(_e114.g0_.y, _e117.g0_.x, _e120.g0_.y, _e123.g1_.z)) * vec4<f32>(1.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e137.g0_.w) * vec4<f32>(_e141.g1_.x, _e144.g1_.y, _e147.g1_.z, _e150.g1_.x)) * vec4<f32>(1.0, 1.0, 1.0, 0.0))) + ((vec4<f32>(_e162.g0_.x) * vec4<f32>(_e166.g0_.x, _e169.g0_.z, _e172.g0_.y, _e175.g1_.x)) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))));
}

fn point_line_outer_product(self_630: Point, other_522: Line) -> Plane {
    var self_631: Point;
    var other_523: Line;

    self_631 = self_630;
    other_523 = other_522;
    let _e4: Point = self_631;
    let _e8: Line = other_523;
    let _e11: Line = other_523;
    let _e14: Line = other_523;
    let _e17: Line = other_523;
    let _e30: Point = self_631;
    let _e34: Line = other_523;
    let _e37: Line = other_523;
    let _e40: Line = other_523;
    let _e43: Line = other_523;
    let _e57: Point = self_631;
    let _e61: Line = other_523;
    let _e64: Line = other_523;
    let _e67: Line = other_523;
    let _e70: Line = other_523;
    let _e82: Point = self_631;
    let _e86: Line = other_523;
    let _e89: Line = other_523;
    let _e92: Line = other_523;
    let _e95: Line = other_523;
    return Plane((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.z, _e11.g0_.z, _e14.g0_.x, _e17.g1_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e30.g0_.z) * vec4<f32>(_e34.g0_.y, _e37.g0_.x, _e40.g0_.y, _e43.g1_.z)) * vec4<f32>(1.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e57.g0_.w) * vec4<f32>(_e61.g1_.x, _e64.g1_.y, _e67.g1_.z, _e70.g1_.x)) * vec4<f32>(1.0, 1.0, 1.0, 0.0))) + ((vec4<f32>(_e82.g0_.x) * vec4<f32>(_e86.g0_.x, _e89.g0_.z, _e92.g0_.y, _e95.g1_.x)) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))));
}

fn point_line_inner_product(self_632: Point, other_524: Line) -> Point {
    var self_633: Point;
    var other_525: Line;

    self_633 = self_632;
    other_525 = other_524;
    let _e4: Point = self_633;
    let _e8: Line = other_525;
    let _e11: Line = other_525;
    let _e14: Line = other_525;
    let _e17: Line = other_525;
    let _e30: Point = self_633;
    let _e34: Line = other_525;
    let _e37: Line = other_525;
    let _e40: Line = other_525;
    let _e43: Line = other_525;
    let _e57: Point = self_633;
    let _e61: Line = other_525;
    let _e64: Line = other_525;
    let _e67: Line = other_525;
    let _e70: Line = other_525;
    return Point(((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g1_.z, _e11.g1_.z, _e14.g1_.x, _e17.g0_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e30.g0_.z) * vec4<f32>(_e34.g1_.y, _e37.g1_.x, _e40.g1_.y, _e43.g0_.z)) * vec4<f32>(1.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e57.g0_.x) * vec4<f32>(_e61.g1_.x, _e64.g1_.z, _e67.g1_.y, _e70.g0_.x)) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))));
}

fn point_line_inner_anti_product(self_634: Point, other_526: Line) -> Plane {
    var self_635: Point;
    var other_527: Line;

    self_635 = self_634;
    other_527 = other_526;
    let _e4: Point = self_635;
    let _e8: Line = other_527;
    let _e20: Point = self_635;
    let _e24: Line = other_527;
    let _e37: Point = self_635;
    let _e40: Line = other_527;
    let _e43: Line = other_527;
    let _e46: Line = other_527;
    let _e49: Line = other_527;
    return Plane(((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0))) + ((vec4<f32>(_e20.g0_.z) * vec4<f32>(_e24.g0_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((_e37.g0_.wwwx * vec4<f32>(_e40.g0_.x, _e43.g0_.y, _e46.g0_.z, _e49.g0_.x)) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))));
}

fn point_line_left_contraction(self_636: Point, other_528: Line) -> Point {
    var self_637: Point;
    var other_529: Line;

    self_637 = self_636;
    other_529 = other_528;
    let _e4: Point = self_637;
    let _e8: Line = other_529;
    let _e11: Line = other_529;
    let _e14: Line = other_529;
    let _e17: Line = other_529;
    let _e30: Point = self_637;
    let _e34: Line = other_529;
    let _e37: Line = other_529;
    let _e40: Line = other_529;
    let _e43: Line = other_529;
    let _e57: Point = self_637;
    let _e61: Line = other_529;
    let _e64: Line = other_529;
    let _e67: Line = other_529;
    let _e70: Line = other_529;
    return Point(((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g1_.z, _e11.g1_.z, _e14.g1_.x, _e17.g0_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e30.g0_.z) * vec4<f32>(_e34.g1_.y, _e37.g1_.x, _e40.g1_.y, _e43.g0_.z)) * vec4<f32>(1.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e57.g0_.x) * vec4<f32>(_e61.g1_.x, _e64.g1_.z, _e67.g1_.y, _e70.g0_.x)) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))));
}

fn point_line_right_anti_contraction(self_638: Point, other_530: Line) -> Plane {
    var self_639: Point;
    var other_531: Line;

    self_639 = self_638;
    other_531 = other_530;
    let _e4: Point = self_639;
    let _e8: Line = other_531;
    let _e20: Point = self_639;
    let _e24: Line = other_531;
    let _e37: Point = self_639;
    let _e40: Line = other_531;
    let _e43: Line = other_531;
    let _e46: Line = other_531;
    let _e49: Line = other_531;
    return Plane(((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0))) + ((vec4<f32>(_e20.g0_.z) * vec4<f32>(_e24.g0_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((_e37.g0_.wwwx * vec4<f32>(_e40.g0_.x, _e43.g0_.y, _e46.g0_.z, _e49.g0_.x)) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))));
}

fn point_plane_add(self_640: Point, other_532: Plane) -> Flector {
    var self_641: Point;
    var other_533: Plane;

    self_641 = self_640;
    other_533 = other_532;
    let _e4: Point = self_641;
    let _e6: Plane = other_533;
    return Flector(_e4.g0_, _e6.g0_);
}

fn point_plane_sub(self_642: Point, other_534: Plane) -> Flector {
    var self_643: Point;
    var other_535: Plane;

    self_643 = self_642;
    other_535 = other_534;
    let _e4: Point = self_643;
    let _e8: Plane = other_535;
    return Flector(_e4.g0_, (vec4<f32>(0.0) - _e8.g0_));
}

fn point_plane_geometric_product(self_644: Point, other_536: Plane) -> Motor {
    var self_645: Point;
    var other_537: Plane;

    self_645 = self_644;
    other_537 = other_536;
    let _e4: Point = self_645;
    let _e8: Plane = other_537;
    let _e19: Point = self_645;
    let _e23: Plane = other_537;
    let _e35: Point = self_645;
    let _e39: Plane = other_537;
    let _e51: Point = self_645;
    let _e55: Plane = other_537;
    let _e67: Point = self_645;
    let _e70: Point = self_645;
    let _e73: Point = self_645;
    let _e77: Plane = other_537;
    return Motor((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.zzxy) * vec4<f32>(1.0, 0.0, -(1.0), 1.0)) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.yxyz) * vec4<f32>(-(1.0), 1.0, 0.0, 1.0))) + ((vec4<f32>(_e35.g0_.w) * vec4<f32>(_e39.g0_.w)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e51.g0_.x) * _e55.g0_.xzyx) * vec4<f32>(0.0, -(1.0), 1.0, 1.0))), ((vec3<f32>(_e67.g0_.x, _e70.g0_.y, _e73.g0_.z) * vec3<f32>(_e77.g0_.w)) * vec3<f32>(-(1.0))));
}

fn point_plane_regressive_product(self_646: Point, other_538: Plane) -> Scalar {
    var self_647: Point;
    var other_539: Plane;

    self_647 = self_646;
    other_539 = other_538;
    let _e4: Point = self_647;
    let _e7: Plane = other_539;
    let _e11: Point = self_647;
    let _e14: Plane = other_539;
    let _e19: Point = self_647;
    let _e22: Plane = other_539;
    let _e27: Point = self_647;
    let _e30: Plane = other_539;
    return Scalar(((((_e4.g0_.x * _e7.g0_.x) + (_e11.g0_.y * _e14.g0_.y)) + (_e19.g0_.z * _e22.g0_.z)) + (_e27.g0_.w * _e30.g0_.w)));
}

fn point_plane_outer_product(self_648: Point, other_540: Plane) -> AntiScalar {
    var self_649: Point;
    var other_541: Plane;

    self_649 = self_648;
    other_541 = other_540;
    let _e4: Point = self_649;
    let _e7: Plane = other_541;
    let _e11: Point = self_649;
    let _e14: Plane = other_541;
    let _e19: Point = self_649;
    let _e22: Plane = other_541;
    let _e27: Point = self_649;
    let _e30: Plane = other_541;
    return AntiScalar(((((_e4.g0_.x * _e7.g0_.x) + (_e11.g0_.y * _e14.g0_.y)) + (_e19.g0_.z * _e22.g0_.z)) + (_e27.g0_.w * _e30.g0_.w)));
}

fn point_plane_inner_product(self_650: Point, other_542: Plane) -> Line {
    var self_651: Point;
    var other_543: Plane;

    self_651 = self_650;
    other_543 = other_542;
    let _e4: Point = self_651;
    let _e8: Plane = other_543;
    let _e11: Plane = other_543;
    let _e14: Plane = other_543;
    let _e25: Point = self_651;
    let _e29: Plane = other_543;
    let _e32: Plane = other_543;
    let _e35: Plane = other_543;
    let _e47: Point = self_651;
    let _e51: Plane = other_543;
    let _e54: Plane = other_543;
    let _e57: Plane = other_543;
    let _e69: Point = self_651;
    let _e72: Point = self_651;
    let _e75: Point = self_651;
    let _e79: Plane = other_543;
    return Line(((((vec3<f32>(_e4.g0_.y) * vec3<f32>(_e8.g0_.z, _e11.g0_.z, _e14.g0_.x)) * vec3<f32>(1.0, 0.0, -(1.0))) + ((vec3<f32>(_e25.g0_.z) * vec3<f32>(_e29.g0_.y, _e32.g0_.x, _e35.g0_.y)) * vec3<f32>(-(1.0), 1.0, 0.0))) + ((vec3<f32>(_e47.g0_.x) * vec3<f32>(_e51.g0_.x, _e54.g0_.z, _e57.g0_.y)) * vec3<f32>(0.0, -(1.0), 1.0))), ((vec3<f32>(_e69.g0_.x, _e72.g0_.y, _e75.g0_.z) * vec3<f32>(_e79.g0_.w)) * vec3<f32>(-(1.0))));
}

fn point_plane_inner_anti_product(self_652: Point, other_544: Plane) -> Line {
    var self_653: Point;
    var other_545: Plane;

    self_653 = self_652;
    other_545 = other_544;
    let _e6: Point = self_653;
    let _e10: Plane = other_545;
    let _e13: Plane = other_545;
    let _e16: Plane = other_545;
    let _e22: Point = self_653;
    let _e26: Plane = other_545;
    let _e29: Plane = other_545;
    let _e32: Plane = other_545;
    let _e43: Point = self_653;
    let _e47: Plane = other_545;
    let _e50: Plane = other_545;
    let _e53: Plane = other_545;
    let _e65: Point = self_653;
    let _e69: Plane = other_545;
    let _e72: Plane = other_545;
    let _e75: Plane = other_545;
    return Line((vec3<f32>(0.0) - (vec3<f32>(_e6.g0_.w) * vec3<f32>(_e10.g0_.x, _e13.g0_.y, _e16.g0_.z))), ((((vec3<f32>(_e22.g0_.y) * vec3<f32>(_e26.g0_.z, _e29.g0_.z, _e32.g0_.x)) * vec3<f32>(-(1.0), 0.0, 1.0)) + ((vec3<f32>(_e43.g0_.z) * vec3<f32>(_e47.g0_.y, _e50.g0_.x, _e53.g0_.y)) * vec3<f32>(1.0, -(1.0), 0.0))) + ((vec3<f32>(_e65.g0_.x) * vec3<f32>(_e69.g0_.x, _e72.g0_.z, _e75.g0_.y)) * vec3<f32>(0.0, 1.0, -(1.0)))));
}

fn point_plane_left_contraction(self_654: Point, other_546: Plane) -> Line {
    var self_655: Point;
    var other_547: Plane;

    self_655 = self_654;
    other_547 = other_546;
    let _e4: Point = self_655;
    let _e8: Plane = other_547;
    let _e11: Plane = other_547;
    let _e14: Plane = other_547;
    let _e25: Point = self_655;
    let _e29: Plane = other_547;
    let _e32: Plane = other_547;
    let _e35: Plane = other_547;
    let _e47: Point = self_655;
    let _e51: Plane = other_547;
    let _e54: Plane = other_547;
    let _e57: Plane = other_547;
    let _e69: Point = self_655;
    let _e72: Point = self_655;
    let _e75: Point = self_655;
    let _e79: Plane = other_547;
    return Line(((((vec3<f32>(_e4.g0_.y) * vec3<f32>(_e8.g0_.z, _e11.g0_.z, _e14.g0_.x)) * vec3<f32>(1.0, 0.0, -(1.0))) + ((vec3<f32>(_e25.g0_.z) * vec3<f32>(_e29.g0_.y, _e32.g0_.x, _e35.g0_.y)) * vec3<f32>(-(1.0), 1.0, 0.0))) + ((vec3<f32>(_e47.g0_.x) * vec3<f32>(_e51.g0_.x, _e54.g0_.z, _e57.g0_.y)) * vec3<f32>(0.0, -(1.0), 1.0))), ((vec3<f32>(_e69.g0_.x, _e72.g0_.y, _e75.g0_.z) * vec3<f32>(_e79.g0_.w)) * vec3<f32>(-(1.0))));
}

fn point_plane_right_anti_contraction(self_656: Point, other_548: Plane) -> Line {
    var self_657: Point;
    var other_549: Plane;

    self_657 = self_656;
    other_549 = other_548;
    let _e6: Point = self_657;
    let _e10: Plane = other_549;
    let _e13: Plane = other_549;
    let _e16: Plane = other_549;
    let _e22: Point = self_657;
    let _e26: Plane = other_549;
    let _e29: Plane = other_549;
    let _e32: Plane = other_549;
    let _e43: Point = self_657;
    let _e47: Plane = other_549;
    let _e50: Plane = other_549;
    let _e53: Plane = other_549;
    let _e65: Point = self_657;
    let _e69: Plane = other_549;
    let _e72: Plane = other_549;
    let _e75: Plane = other_549;
    return Line((vec3<f32>(0.0) - (vec3<f32>(_e6.g0_.w) * vec3<f32>(_e10.g0_.x, _e13.g0_.y, _e16.g0_.z))), ((((vec3<f32>(_e22.g0_.y) * vec3<f32>(_e26.g0_.z, _e29.g0_.z, _e32.g0_.x)) * vec3<f32>(-(1.0), 0.0, 1.0)) + ((vec3<f32>(_e43.g0_.z) * vec3<f32>(_e47.g0_.y, _e50.g0_.x, _e53.g0_.y)) * vec3<f32>(1.0, -(1.0), 0.0))) + ((vec3<f32>(_e65.g0_.x) * vec3<f32>(_e69.g0_.x, _e72.g0_.z, _e75.g0_.y)) * vec3<f32>(0.0, 1.0, -(1.0)))));
}

fn point_motor_geometric_product(self_658: Point, other_550: Motor) -> Flector {
    var self_659: Point;
    var other_551: Motor;

    self_659 = self_658;
    other_551 = other_550;
    let _e4: Point = self_659;
    let _e8: Motor = other_551;
    let _e11: Motor = other_551;
    let _e14: Motor = other_551;
    let _e17: Motor = other_551;
    let _e30: Point = self_659;
    let _e34: Motor = other_551;
    let _e37: Motor = other_551;
    let _e40: Motor = other_551;
    let _e43: Motor = other_551;
    let _e57: Point = self_659;
    let _e61: Motor = other_551;
    let _e64: Motor = other_551;
    let _e67: Motor = other_551;
    let _e70: Motor = other_551;
    let _e84: Point = self_659;
    let _e88: Motor = other_551;
    let _e91: Motor = other_551;
    let _e94: Motor = other_551;
    let _e97: Motor = other_551;
    let _e110: Point = self_659;
    let _e114: Motor = other_551;
    let _e117: Motor = other_551;
    let _e120: Motor = other_551;
    let _e123: Motor = other_551;
    let _e137: Point = self_659;
    let _e141: Motor = other_551;
    let _e144: Motor = other_551;
    let _e147: Motor = other_551;
    let _e150: Motor = other_551;
    let _e164: Point = self_659;
    let _e167: Motor = other_551;
    let _e170: Motor = other_551;
    let _e173: Motor = other_551;
    let _e176: Motor = other_551;
    return Flector(((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g1_.z, _e11.g1_.z, _e14.g1_.x, _e17.g0_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e30.g0_.z) * vec4<f32>(_e34.g1_.y, _e37.g1_.x, _e40.g1_.y, _e43.g0_.z)) * vec4<f32>(1.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e57.g0_.x) * vec4<f32>(_e61.g1_.x, _e64.g1_.z, _e67.g1_.y, _e70.g0_.x)) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))), (((((vec4<f32>(_e84.g0_.x) * vec4<f32>(_e88.g0_.w, _e91.g0_.z, _e94.g0_.y, _e97.g1_.x)) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0))) + ((vec4<f32>(_e110.g0_.y) * vec4<f32>(_e114.g0_.z, _e117.g0_.w, _e120.g0_.x, _e123.g1_.y)) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e137.g0_.z) * vec4<f32>(_e141.g0_.y, _e144.g0_.x, _e147.g0_.w, _e150.g1_.z)) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))) + ((_e164.g0_.wwwx * vec4<f32>(_e167.g1_.x, _e170.g1_.y, _e173.g1_.z, _e176.g1_.x)) * vec4<f32>(1.0, 1.0, 1.0, 0.0))));
}

fn point_motor_regressive_product(self_660: Point, other_552: Motor) -> Point {
    var self_661: Point;
    var other_553: Motor;

    self_661 = self_660;
    other_553 = other_552;
    let _e4: Point = self_661;
    let _e6: Motor = other_553;
    return Point((_e4.g0_ * vec4<f32>(_e6.g0_.w)));
}

fn point_motor_outer_product(self_662: Point, other_554: Motor) -> Plane {
    var self_663: Point;
    var other_555: Motor;

    self_663 = self_662;
    other_555 = other_554;
    let _e4: Point = self_663;
    let _e8: Motor = other_555;
    let _e11: Motor = other_555;
    let _e14: Motor = other_555;
    let _e17: Motor = other_555;
    let _e30: Point = self_663;
    let _e34: Motor = other_555;
    let _e37: Motor = other_555;
    let _e40: Motor = other_555;
    let _e43: Motor = other_555;
    let _e57: Point = self_663;
    let _e61: Motor = other_555;
    let _e64: Motor = other_555;
    let _e67: Motor = other_555;
    let _e70: Motor = other_555;
    let _e82: Point = self_663;
    let _e86: Motor = other_555;
    let _e89: Motor = other_555;
    let _e92: Motor = other_555;
    let _e95: Motor = other_555;
    return Plane((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.z, _e11.g0_.z, _e14.g0_.x, _e17.g1_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e30.g0_.z) * vec4<f32>(_e34.g0_.y, _e37.g0_.x, _e40.g0_.y, _e43.g1_.z)) * vec4<f32>(1.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e57.g0_.w) * vec4<f32>(_e61.g1_.x, _e64.g1_.y, _e67.g1_.z, _e70.g1_.x)) * vec4<f32>(1.0, 1.0, 1.0, 0.0))) + ((vec4<f32>(_e82.g0_.x) * vec4<f32>(_e86.g0_.x, _e89.g0_.z, _e92.g0_.y, _e95.g1_.x)) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))));
}

fn point_motor_geometric_anti_product(self_664: Point, other_556: Motor) -> Flector {
    var self_665: Point;
    var other_557: Motor;

    self_665 = self_664;
    other_557 = other_556;
    let _e4: Point = self_665;
    let _e8: Motor = other_557;
    let _e19: Point = self_665;
    let _e23: Motor = other_557;
    let _e35: Point = self_665;
    let _e39: Motor = other_557;
    let _e42: Motor = other_557;
    let _e45: Motor = other_557;
    let _e48: Motor = other_557;
    let _e63: Point = self_665;
    let _e67: Motor = other_557;
    let _e79: Point = self_665;
    let _e83: Motor = other_557;
    let _e95: Point = self_665;
    let _e99: Motor = other_557;
    let _e112: Point = self_665;
    let _e115: Motor = other_557;
    return Flector((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.zwxz) * vec4<f32>(1.0, 1.0, -(1.0), 0.0)) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.yxwy) * vec4<f32>(-(1.0), 1.0, 1.0, 0.0))) + ((vec4<f32>(_e35.g0_.w) * vec4<f32>(_e39.g1_.x, _e42.g1_.y, _e45.g1_.z, _e48.g0_.w)) * vec4<f32>(-(1.0), -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e63.g0_.x) * _e67.g0_.wzyx) * vec4<f32>(1.0, -(1.0), 1.0, 0.0))), ((((vec4<f32>(_e79.g0_.y) * vec4<f32>(_e83.g0_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0))) + ((vec4<f32>(_e95.g0_.z) * vec4<f32>(_e99.g0_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((_e112.g0_.wwwx * _e115.g0_.xyzx) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))));
}

fn point_motor_inner_anti_product(self_666: Point, other_558: Motor) -> Flector {
    var self_667: Point;
    var other_559: Motor;

    self_667 = self_666;
    other_559 = other_558;
    let _e4: Point = self_667;
    let _e6: Motor = other_559;
    let _e11: Point = self_667;
    let _e15: Motor = other_559;
    let _e27: Point = self_667;
    let _e31: Motor = other_559;
    let _e44: Point = self_667;
    let _e47: Motor = other_559;
    return Flector((_e4.g0_ * vec4<f32>(_e6.g0_.w)), ((((vec4<f32>(_e11.g0_.y) * vec4<f32>(_e15.g0_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0))) + ((vec4<f32>(_e27.g0_.z) * vec4<f32>(_e31.g0_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((_e44.g0_.wwwx * _e47.g0_.xyzx) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))));
}

fn point_motor_right_anti_contraction(self_668: Point, other_560: Motor) -> Flector {
    var self_669: Point;
    var other_561: Motor;

    self_669 = self_668;
    other_561 = other_560;
    let _e4: Point = self_669;
    let _e6: Motor = other_561;
    let _e11: Point = self_669;
    let _e15: Motor = other_561;
    let _e27: Point = self_669;
    let _e31: Motor = other_561;
    let _e44: Point = self_669;
    let _e47: Motor = other_561;
    return Flector((_e4.g0_ * vec4<f32>(_e6.g0_.w)), ((((vec4<f32>(_e11.g0_.y) * vec4<f32>(_e15.g0_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0))) + ((vec4<f32>(_e27.g0_.z) * vec4<f32>(_e31.g0_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((_e44.g0_.wwwx * _e47.g0_.xyzx) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))));
}

fn point_rotor_regressive_product(self_670: Point, other_562: Rotor) -> Point {
    var self_671: Point;
    var other_563: Rotor;

    self_671 = self_670;
    other_563 = other_562;
    let _e4: Point = self_671;
    let _e6: Rotor = other_563;
    return Point((_e4.g0_ * vec4<f32>(_e6.g0_.w)));
}

fn point_rotor_geometric_anti_product(self_672: Point, other_564: Rotor) -> Flector {
    var self_673: Point;
    var other_565: Rotor;

    self_673 = self_672;
    other_565 = other_564;
    let _e4: Point = self_673;
    let _e8: Rotor = other_565;
    let _e19: Point = self_673;
    let _e23: Rotor = other_565;
    let _e35: Point = self_673;
    let _e38: Rotor = other_565;
    let _e50: Point = self_673;
    let _e54: Rotor = other_565;
    let _e66: Point = self_673;
    let _e70: Rotor = other_565;
    let _e83: Point = self_673;
    let _e86: Rotor = other_565;
    return Flector(((((vec4<f32>(_e4.g0_.y) * _e8.g0_.zwxz) * vec4<f32>(1.0, 1.0, -(1.0), 0.0)) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.yxwy) * vec4<f32>(-(1.0), 1.0, 1.0, 0.0))) + ((_e35.g0_.xxxw * _e38.g0_.wzyw) * vec4<f32>(1.0, -(1.0), 1.0, 1.0))), ((((vec4<f32>(_e50.g0_.y) * vec4<f32>(_e54.g0_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0))) + ((vec4<f32>(_e66.g0_.z) * vec4<f32>(_e70.g0_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((_e83.g0_.wwwx * _e86.g0_.xyzx) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))));
}

fn point_rotor_inner_anti_product(self_674: Point, other_566: Rotor) -> Flector {
    var self_675: Point;
    var other_567: Rotor;

    self_675 = self_674;
    other_567 = other_566;
    let _e4: Point = self_675;
    let _e6: Rotor = other_567;
    let _e11: Point = self_675;
    let _e15: Rotor = other_567;
    let _e27: Point = self_675;
    let _e31: Rotor = other_567;
    let _e44: Point = self_675;
    let _e47: Rotor = other_567;
    return Flector((_e4.g0_ * vec4<f32>(_e6.g0_.w)), ((((vec4<f32>(_e11.g0_.y) * vec4<f32>(_e15.g0_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0))) + ((vec4<f32>(_e27.g0_.z) * vec4<f32>(_e31.g0_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((_e44.g0_.wwwx * _e47.g0_.xyzx) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))));
}

fn point_rotor_right_anti_contraction(self_676: Point, other_568: Rotor) -> Flector {
    var self_677: Point;
    var other_569: Rotor;

    self_677 = self_676;
    other_569 = other_568;
    let _e4: Point = self_677;
    let _e6: Rotor = other_569;
    let _e11: Point = self_677;
    let _e15: Rotor = other_569;
    let _e27: Point = self_677;
    let _e31: Rotor = other_569;
    let _e44: Point = self_677;
    let _e47: Rotor = other_569;
    return Flector((_e4.g0_ * vec4<f32>(_e6.g0_.w)), ((((vec4<f32>(_e11.g0_.y) * vec4<f32>(_e15.g0_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0))) + ((vec4<f32>(_e27.g0_.z) * vec4<f32>(_e31.g0_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((_e44.g0_.wwwx * _e47.g0_.xyzx) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))));
}

fn point_translator_regressive_product(self_678: Point, other_570: Translator) -> Point {
    var self_679: Point;
    var other_571: Translator;

    self_679 = self_678;
    other_571 = other_570;
    let _e4: Point = self_679;
    let _e6: Translator = other_571;
    return Point((_e4.g0_ * vec4<f32>(_e6.g0_.w)));
}

fn point_translator_outer_product(self_680: Point, other_572: Translator) -> Plane {
    var self_681: Point;
    var other_573: Translator;

    self_681 = self_680;
    other_573 = other_572;
    let _e4: Point = self_681;
    let _e8: Translator = other_573;
    let _e20: Point = self_681;
    let _e24: Translator = other_573;
    let _e37: Point = self_681;
    let _e40: Translator = other_573;
    return Plane(((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0))) + ((vec4<f32>(_e20.g0_.z) * vec4<f32>(_e24.g0_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((_e37.g0_.wwwx * _e40.g0_.xyzx) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))));
}

fn point_translator_geometric_anti_product(self_682: Point, other_574: Translator) -> Point {
    var self_683: Point;
    var other_575: Translator;

    self_683 = self_682;
    other_575 = other_574;
    let _e4: Point = self_683;
    let _e8: Translator = other_575;
    let _e20: Point = self_683;
    let _e23: Translator = other_575;
    return Point((((vec4<f32>(_e4.g0_.w) * _e8.g0_) * vec4<f32>(-(1.0), -(1.0), -(1.0), 1.0)) + ((_e20.g0_.xyzx * _e23.g0_.wwwx) * vec4<f32>(1.0, 1.0, 1.0, 0.0))));
}

fn point_translator_inner_anti_product(self_684: Point, other_576: Translator) -> Point {
    var self_685: Point;
    var other_577: Translator;

    self_685 = self_684;
    other_577 = other_576;
    let _e4: Point = self_685;
    let _e6: Translator = other_577;
    return Point((_e4.g0_ * vec4<f32>(_e6.g0_.w)));
}

fn point_translator_right_anti_contraction(self_686: Point, other_578: Translator) -> Point {
    var self_687: Point;
    var other_579: Translator;

    self_687 = self_686;
    other_579 = other_578;
    let _e4: Point = self_687;
    let _e6: Translator = other_579;
    return Point((_e4.g0_ * vec4<f32>(_e6.g0_.w)));
}

fn point_flector_add(self_688: Point, other_580: Flector) -> Flector {
    var self_689: Point;
    var other_581: Flector;

    self_689 = self_688;
    other_581 = other_580;
    let _e4: Point = self_689;
    let _e6: Flector = other_581;
    let _e9: Flector = other_581;
    return Flector((_e4.g0_ + _e6.g0_), _e9.g1_);
}

fn point_flector_sub(self_690: Point, other_582: Flector) -> Flector {
    var self_691: Point;
    var other_583: Flector;

    self_691 = self_690;
    other_583 = other_582;
    let _e4: Point = self_691;
    let _e6: Flector = other_583;
    let _e11: Flector = other_583;
    return Flector((_e4.g0_ - _e6.g0_), (vec4<f32>(0.0) - _e11.g1_));
}

fn point_flector_regressive_product(self_692: Point, other_584: Flector) -> Scalar {
    var self_693: Point;
    var other_585: Flector;

    self_693 = self_692;
    other_585 = other_584;
    let _e4: Point = self_693;
    let _e7: Flector = other_585;
    let _e11: Point = self_693;
    let _e14: Flector = other_585;
    let _e19: Point = self_693;
    let _e22: Flector = other_585;
    let _e27: Point = self_693;
    let _e30: Flector = other_585;
    return Scalar(((((_e4.g0_.x * _e7.g1_.x) + (_e11.g0_.y * _e14.g1_.y)) + (_e19.g0_.z * _e22.g1_.z)) + (_e27.g0_.w * _e30.g1_.w)));
}

fn point_flector_outer_product(self_694: Point, other_586: Flector) -> Motor {
    var self_695: Point;
    var other_587: Flector;

    self_695 = self_694;
    other_587 = other_586;
    let _e4: Point = self_695;
    let _e8: Flector = other_587;
    let _e11: Flector = other_587;
    let _e14: Flector = other_587;
    let _e17: Flector = other_587;
    let _e29: Point = self_695;
    let _e33: Flector = other_587;
    let _e36: Flector = other_587;
    let _e39: Flector = other_587;
    let _e42: Flector = other_587;
    let _e55: Point = self_695;
    let _e59: Flector = other_587;
    let _e62: Flector = other_587;
    let _e65: Flector = other_587;
    let _e68: Flector = other_587;
    let _e74: Point = self_695;
    let _e78: Flector = other_587;
    let _e81: Flector = other_587;
    let _e84: Flector = other_587;
    let _e87: Flector = other_587;
    let _e100: Point = self_695;
    let _e104: Flector = other_587;
    let _e107: Flector = other_587;
    let _e110: Flector = other_587;
    let _e121: Point = self_695;
    let _e125: Flector = other_587;
    let _e128: Flector = other_587;
    let _e131: Flector = other_587;
    let _e143: Point = self_695;
    let _e147: Flector = other_587;
    let _e150: Flector = other_587;
    let _e153: Flector = other_587;
    return Motor((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.w, _e11.g0_.w, _e14.g0_.w, _e17.g1_.y)) * vec4<f32>(0.0, -(1.0), 0.0, 1.0)) + ((vec4<f32>(_e29.g0_.z) * vec4<f32>(_e33.g0_.w, _e36.g0_.w, _e39.g0_.w, _e42.g1_.z)) * vec4<f32>(0.0, 0.0, -(1.0), 1.0))) + (vec4<f32>(_e55.g0_.w) * vec4<f32>(_e59.g0_.x, _e62.g0_.y, _e65.g0_.z, _e68.g1_.w))) + ((vec4<f32>(_e74.g0_.x) * vec4<f32>(_e78.g0_.w, _e81.g0_.x, _e84.g0_.x, _e87.g1_.x)) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))), ((((vec3<f32>(_e100.g0_.y) * vec3<f32>(_e104.g0_.z, _e107.g0_.z, _e110.g0_.x)) * vec3<f32>(1.0, 0.0, -(1.0))) + ((vec3<f32>(_e121.g0_.z) * vec3<f32>(_e125.g0_.y, _e128.g0_.x, _e131.g0_.y)) * vec3<f32>(-(1.0), 1.0, 0.0))) + ((vec3<f32>(_e143.g0_.x) * vec3<f32>(_e147.g0_.x, _e150.g0_.z, _e153.g0_.y)) * vec3<f32>(0.0, -(1.0), 1.0))));
}

fn point_flector_inner_anti_product(self_696: Point, other_588: Flector) -> Motor {
    var self_697: Point;
    var other_589: Flector;

    self_697 = self_696;
    other_589 = other_588;
    let _e6: Point = self_697;
    let _e10: Flector = other_589;
    let _e13: Flector = other_589;
    let _e16: Flector = other_589;
    let _e19: Flector = other_589;
    let _e25: Point = self_697;
    let _e29: Flector = other_589;
    let _e32: Flector = other_589;
    let _e35: Flector = other_589;
    let _e46: Point = self_697;
    let _e50: Flector = other_589;
    let _e53: Flector = other_589;
    let _e56: Flector = other_589;
    let _e68: Point = self_697;
    let _e72: Flector = other_589;
    let _e75: Flector = other_589;
    let _e78: Flector = other_589;
    return Motor((vec4<f32>(0.0) - (vec4<f32>(_e6.g0_.w) * vec4<f32>(_e10.g1_.x, _e13.g1_.y, _e16.g1_.z, _e19.g0_.w))), ((((vec3<f32>(_e25.g0_.y) * vec3<f32>(_e29.g1_.z, _e32.g1_.z, _e35.g1_.x)) * vec3<f32>(-(1.0), 0.0, 1.0)) + ((vec3<f32>(_e46.g0_.z) * vec3<f32>(_e50.g1_.y, _e53.g1_.x, _e56.g1_.y)) * vec3<f32>(1.0, -(1.0), 0.0))) + ((vec3<f32>(_e68.g0_.x) * vec3<f32>(_e72.g1_.x, _e75.g1_.z, _e78.g1_.y)) * vec3<f32>(0.0, 1.0, -(1.0)))));
}

fn point_flector_right_contraction(self_698: Point, other_590: Flector) -> Scalar {
    var self_699: Point;
    var other_591: Flector;

    self_699 = self_698;
    other_591 = other_590;
    let _e4: Point = self_699;
    let _e7: Flector = other_591;
    let _e11: Point = self_699;
    let _e14: Flector = other_591;
    let _e19: Point = self_699;
    let _e22: Flector = other_591;
    return Scalar((((_e4.g0_.x * _e7.g0_.x) + (_e11.g0_.y * _e14.g0_.y)) + (_e19.g0_.z * _e22.g0_.z)));
}

fn point_flector_left_anti_contraction(self_700: Point, other_592: Flector) -> AntiScalar {
    var self_701: Point;
    var other_593: Flector;

    self_701 = self_700;
    other_593 = other_592;
    let _e5: Point = self_701;
    let _e8: Flector = other_593;
    return AntiScalar((0.0 - (_e5.g0_.w * _e8.g0_.w)));
}

fn point_flector_right_anti_contraction(self_702: Point, other_594: Flector) -> Motor {
    var self_703: Point;
    var other_595: Flector;

    self_703 = self_702;
    other_595 = other_594;
    let _e6: Point = self_703;
    let _e10: Flector = other_595;
    let _e13: Flector = other_595;
    let _e16: Flector = other_595;
    let _e19: Flector = other_595;
    let _e25: Point = self_703;
    let _e29: Flector = other_595;
    let _e32: Flector = other_595;
    let _e35: Flector = other_595;
    let _e46: Point = self_703;
    let _e50: Flector = other_595;
    let _e53: Flector = other_595;
    let _e56: Flector = other_595;
    let _e68: Point = self_703;
    let _e72: Flector = other_595;
    let _e75: Flector = other_595;
    let _e78: Flector = other_595;
    return Motor((vec4<f32>(0.0) - (vec4<f32>(_e6.g0_.w) * vec4<f32>(_e10.g1_.x, _e13.g1_.y, _e16.g1_.z, _e19.g0_.w))), ((((vec3<f32>(_e25.g0_.y) * vec3<f32>(_e29.g1_.z, _e32.g1_.z, _e35.g1_.x)) * vec3<f32>(-(1.0), 0.0, 1.0)) + ((vec3<f32>(_e46.g0_.z) * vec3<f32>(_e50.g1_.y, _e53.g1_.x, _e56.g1_.y)) * vec3<f32>(1.0, -(1.0), 0.0))) + ((vec3<f32>(_e68.g0_.x) * vec3<f32>(_e72.g1_.x, _e75.g1_.z, _e78.g1_.y)) * vec3<f32>(0.0, 1.0, -(1.0)))));
}

fn point_flector_scalar_product(self_704: Point, other_596: Flector) -> Scalar {
    var self_705: Point;
    var other_597: Flector;

    self_705 = self_704;
    other_597 = other_596;
    let _e4: Point = self_705;
    let _e7: Flector = other_597;
    let _e11: Point = self_705;
    let _e14: Flector = other_597;
    let _e19: Point = self_705;
    let _e22: Flector = other_597;
    return Scalar((((_e4.g0_.x * _e7.g0_.x) + (_e11.g0_.y * _e14.g0_.y)) + (_e19.g0_.z * _e22.g0_.z)));
}

fn point_flector_anti_scalar_product(self_706: Point, other_598: Flector) -> AntiScalar {
    var self_707: Point;
    var other_599: Flector;

    self_707 = self_706;
    other_599 = other_598;
    let _e5: Point = self_707;
    let _e8: Flector = other_599;
    return AntiScalar((0.0 - (_e5.g0_.w * _e8.g0_.w)));
}

fn point_multi_vector_scalar_product(self_708: Point, other_600: MultiVector) -> Scalar {
    var self_709: Point;
    var other_601: MultiVector;

    self_709 = self_708;
    other_601 = other_600;
    let _e4: Point = self_709;
    let _e7: MultiVector = other_601;
    let _e11: Point = self_709;
    let _e14: MultiVector = other_601;
    let _e19: Point = self_709;
    let _e22: MultiVector = other_601;
    return Scalar((((_e4.g0_.x * _e7.g1_.x) + (_e11.g0_.y * _e14.g1_.y)) + (_e19.g0_.z * _e22.g1_.z)));
}

fn point_multi_vector_anti_scalar_product(self_710: Point, other_602: MultiVector) -> AntiScalar {
    var self_711: Point;
    var other_603: MultiVector;

    self_711 = self_710;
    other_603 = other_602;
    let _e5: Point = self_711;
    let _e8: MultiVector = other_603;
    return AntiScalar((0.0 - (_e5.g0_.w * _e8.g1_.w)));
}

fn point_squared_magnitude(self_712: Point) -> Scalar {
    var self_713: Point;

    self_713 = self_712;
    let _e2: Point = self_713;
    let _e3: Point = self_713;
    let _e4: Point = point_reversal(_e3);
    let _e5: Scalar = point_point_scalar_product(_e2, _e4);
    return _e5;
}

fn point_magnitude(self_714: Point) -> Scalar {
    var self_715: Point;

    self_715 = self_714;
    let _e2: Point = self_715;
    let _e3: Scalar = point_squared_magnitude(_e2);
    return Scalar(sqrt(_e3.g0_));
}

fn point_bulk_norm(self_716: Point) -> Scalar {
    var self_717: Point;

    self_717 = self_716;
    let _e2: Point = self_717;
    let _e3: Scalar = point_squared_magnitude(_e2);
    return Scalar(sqrt(_e3.g0_));
}

fn point_squared_anti_magnitude(self_718: Point) -> AntiScalar {
    var self_719: Point;

    self_719 = self_718;
    let _e2: Point = self_719;
    let _e3: Point = self_719;
    let _e4: Point = point_anti_reversal(_e3);
    let _e5: AntiScalar = point_point_anti_scalar_product(_e2, _e4);
    return _e5;
}

fn point_weight_norm(self_720: Point) -> AntiScalar {
    var self_721: Point;

    self_721 = self_720;
    let _e2: Point = self_721;
    let _e3: AntiScalar = point_squared_anti_magnitude(_e2);
    return AntiScalar(sqrt(_e3.g0_));
}

fn point_geometric_norm(self_722: Point) -> HomogeneousMagnitude {
    var self_723: Point;

    self_723 = self_722;
    let _e2: Point = self_723;
    let _e3: Scalar = point_bulk_norm(_e2);
    let _e4: Point = self_723;
    let _e5: AntiScalar = point_weight_norm(_e4);
    return (_e3 + _e5);
}

fn point_scale(self_724: Point, other_604: f32) -> Point {
    var self_725: Point;
    var other_605: f32;

    self_725 = self_724;
    other_605 = other_604;
    let _e4: Point = self_725;
    let _e5: f32 = other_605;
    let _e7: Point = point_scalar_geometric_product(_e4, Scalar(_e5));
    return _e7;
}

fn point_signum(self_726: Point) -> Point {
    var self_727: Point;

    self_727 = self_726;
    let _e2: Point = self_727;
    let _e3: Point = self_727;
    let _e4: Scalar = point_magnitude(_e3);
    let _e9: Point = point_scalar_geometric_product(_e2, Scalar((1.0 / _e4.g0_)));
    return _e9;
}

fn point_inverse(self_728: Point) -> Point {
    var self_729: Point;

    self_729 = self_728;
    let _e2: Point = self_729;
    let _e3: Point = point_reversal(_e2);
    let _e4: Point = self_729;
    let _e5: Scalar = point_squared_magnitude(_e4);
    let _e10: Point = point_scalar_geometric_product(_e3, Scalar((1.0 / _e5.g0_)));
    return _e10;
}

fn point_unitize(self_730: Point) -> Point {
    var self_731: Point;

    self_731 = self_730;
    let _e2: Point = self_731;
    let _e3: Point = self_731;
    let _e4: AntiScalar = point_weight_norm(_e3);
    let _e9: Point = point_scalar_geometric_product(_e2, Scalar((1.0 / _e4.g0_)));
    return _e9;
}

fn line_zero() -> Line {
    return Line(vec3<f32>(0.0), vec3<f32>(0.0));
}

fn line_one() -> Line {
    return Line(vec3<f32>(0.0), vec3<f32>(0.0));
}

fn line_grade(self_732: Line) -> i32 {
    return 2;
}

fn line_anti_grade(self_733: Line) -> i32 {
    return 2;
}

fn line_neg(self_734: Line) -> Line {
    var self_735: Line;

    self_735 = self_734;
    let _e2: Line = self_735;
    let _e8: Line = self_735;
    return Line((_e2.g0_ * vec3<f32>(-(1.0))), (_e8.g1_ * vec3<f32>(-(1.0))));
}

fn line_automorphism(self_736: Line) -> Line {
    var self_737: Line;

    self_737 = self_736;
    let _e2: Line = self_737;
    let _e4: Line = self_737;
    return Line(_e2.g0_, _e4.g1_);
}

fn line_reversal(self_738: Line) -> Line {
    var self_739: Line;

    self_739 = self_738;
    let _e2: Line = self_739;
    let _e8: Line = self_739;
    return Line((_e2.g0_ * vec3<f32>(-(1.0))), (_e8.g1_ * vec3<f32>(-(1.0))));
}

fn line_conjugation(self_740: Line) -> Line {
    var self_741: Line;

    self_741 = self_740;
    let _e2: Line = self_741;
    let _e8: Line = self_741;
    return Line((_e2.g0_ * vec3<f32>(-(1.0))), (_e8.g1_ * vec3<f32>(-(1.0))));
}

fn line_dual(self_742: Line) -> Line {
    var self_743: Line;

    self_743 = self_742;
    let _e2: Line = self_743;
    let _e8: Line = self_743;
    return Line((_e2.g1_ * vec3<f32>(-(1.0))), (_e8.g0_ * vec3<f32>(-(1.0))));
}

fn line_anti_reversal(self_744: Line) -> Line {
    var self_745: Line;

    self_745 = self_744;
    let _e2: Line = self_745;
    let _e8: Line = self_745;
    return Line((_e2.g0_ * vec3<f32>(-(1.0))), (_e8.g1_ * vec3<f32>(-(1.0))));
}

fn line_right_complement(self_746: Line) -> Line {
    var self_747: Line;

    self_747 = self_746;
    let _e2: Line = self_747;
    let _e8: Line = self_747;
    return Line((_e2.g1_ * vec3<f32>(-(1.0))), (_e8.g0_ * vec3<f32>(-(1.0))));
}

fn line_left_complement(self_748: Line) -> Line {
    var self_749: Line;

    self_749 = self_748;
    let _e2: Line = self_749;
    let _e8: Line = self_749;
    return Line((_e2.g1_ * vec3<f32>(-(1.0))), (_e8.g0_ * vec3<f32>(-(1.0))));
}

fn line_double_complement(self_750: Line) -> Line {
    var self_751: Line;

    self_751 = self_750;
    let _e2: Line = self_751;
    let _e4: Line = self_751;
    return Line(_e2.g0_, _e4.g1_);
}

fn line_scalar_geometric_product(self_752: Line, other_606: Scalar) -> Line {
    var self_753: Line;
    var other_607: Scalar;

    self_753 = self_752;
    other_607 = other_606;
    let _e4: Line = self_753;
    let _e6: Scalar = other_607;
    let _e10: Line = self_753;
    let _e12: Scalar = other_607;
    return Line((_e4.g0_ * vec3<f32>(_e6.g0_)), (_e10.g1_ * vec3<f32>(_e12.g0_)));
}

fn line_scalar_outer_product(self_754: Line, other_608: Scalar) -> Line {
    var self_755: Line;
    var other_609: Scalar;

    self_755 = self_754;
    other_609 = other_608;
    let _e4: Line = self_755;
    let _e6: Scalar = other_609;
    let _e10: Line = self_755;
    let _e12: Scalar = other_609;
    return Line((_e4.g0_ * vec3<f32>(_e6.g0_)), (_e10.g1_ * vec3<f32>(_e12.g0_)));
}

fn line_scalar_inner_product(self_756: Line, other_610: Scalar) -> Line {
    var self_757: Line;
    var other_611: Scalar;

    self_757 = self_756;
    other_611 = other_610;
    let _e4: Line = self_757;
    let _e6: Scalar = other_611;
    let _e10: Line = self_757;
    let _e12: Scalar = other_611;
    return Line((_e4.g0_ * vec3<f32>(_e6.g0_)), (_e10.g1_ * vec3<f32>(_e12.g0_)));
}

fn line_scalar_right_contraction(self_758: Line, other_612: Scalar) -> Line {
    var self_759: Line;
    var other_613: Scalar;

    self_759 = self_758;
    other_613 = other_612;
    let _e4: Line = self_759;
    let _e6: Scalar = other_613;
    let _e10: Line = self_759;
    let _e12: Scalar = other_613;
    return Line((_e4.g0_ * vec3<f32>(_e6.g0_)), (_e10.g1_ * vec3<f32>(_e12.g0_)));
}

fn line_anti_scalar_add(self_760: Line, other_614: AntiScalar) -> Motor {
    var self_761: Line;
    var other_615: AntiScalar;

    self_761 = self_760;
    other_615 = other_614;
    let _e4: Line = self_761;
    let _e7: Line = self_761;
    let _e10: Line = self_761;
    let _e13: Line = self_761;
    let _e23: AntiScalar = other_615;
    let _e33: Line = self_761;
    return Motor(((vec4<f32>(_e4.g0_.x, _e7.g0_.y, _e10.g0_.z, _e13.g0_.x) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) + (vec4<f32>(_e23.g0_) * vec4<f32>(0.0, 0.0, 0.0, 1.0))), _e33.g1_);
}

fn line_anti_scalar_sub(self_762: Line, other_616: AntiScalar) -> Motor {
    var self_763: Line;
    var other_617: AntiScalar;

    self_763 = self_762;
    other_617 = other_616;
    let _e4: Line = self_763;
    let _e7: Line = self_763;
    let _e10: Line = self_763;
    let _e13: Line = self_763;
    let _e23: AntiScalar = other_617;
    let _e33: Line = self_763;
    return Motor(((vec4<f32>(_e4.g0_.x, _e7.g0_.y, _e10.g0_.z, _e13.g0_.x) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) - (vec4<f32>(_e23.g0_) * vec4<f32>(0.0, 0.0, 0.0, 1.0))), _e33.g1_);
}

fn line_anti_scalar_regressive_product(self_764: Line, other_618: AntiScalar) -> Line {
    var self_765: Line;
    var other_619: AntiScalar;

    self_765 = self_764;
    other_619 = other_618;
    let _e4: Line = self_765;
    let _e6: AntiScalar = other_619;
    let _e10: Line = self_765;
    let _e12: AntiScalar = other_619;
    return Line((_e4.g0_ * vec3<f32>(_e6.g0_)), (_e10.g1_ * vec3<f32>(_e12.g0_)));
}

fn line_anti_scalar_geometric_anti_product(self_766: Line, other_620: AntiScalar) -> Line {
    var self_767: Line;
    var other_621: AntiScalar;

    self_767 = self_766;
    other_621 = other_620;
    let _e4: Line = self_767;
    let _e6: AntiScalar = other_621;
    let _e10: Line = self_767;
    let _e12: AntiScalar = other_621;
    return Line((_e4.g0_ * vec3<f32>(_e6.g0_)), (_e10.g1_ * vec3<f32>(_e12.g0_)));
}

fn line_anti_scalar_inner_anti_product(self_768: Line, other_622: AntiScalar) -> Line {
    var self_769: Line;
    var other_623: AntiScalar;

    self_769 = self_768;
    other_623 = other_622;
    let _e4: Line = self_769;
    let _e6: AntiScalar = other_623;
    let _e10: Line = self_769;
    let _e12: AntiScalar = other_623;
    return Line((_e4.g0_ * vec3<f32>(_e6.g0_)), (_e10.g1_ * vec3<f32>(_e12.g0_)));
}

fn line_anti_scalar_right_anti_contraction(self_770: Line, other_624: AntiScalar) -> Line {
    var self_771: Line;
    var other_625: AntiScalar;

    self_771 = self_770;
    other_625 = other_624;
    let _e4: Line = self_771;
    let _e6: AntiScalar = other_625;
    let _e10: Line = self_771;
    let _e12: AntiScalar = other_625;
    return Line((_e4.g0_ * vec3<f32>(_e6.g0_)), (_e10.g1_ * vec3<f32>(_e12.g0_)));
}

fn line_homogeneous_magnitude_geometric_product(self_772: Line, other_626: HomogeneousMagnitude) -> Line {
    var self_773: Line;
    var other_627: HomogeneousMagnitude;

    self_773 = self_772;
    other_627 = other_626;
    let _e4: Line = self_773;
    let _e8: HomogeneousMagnitude = other_627;
    let _e18: Line = self_773;
    let _e22: HomogeneousMagnitude = other_627;
    let _e33: Line = self_773;
    let _e37: HomogeneousMagnitude = other_627;
    let _e48: Line = self_773;
    let _e50: HomogeneousMagnitude = other_627;
    let _e56: Line = self_773;
    let _e58: HomogeneousMagnitude = other_627;
    return Line((((((vec3<f32>(_e4.g1_.x) * vec3<f32>(_e8.g0_.y)) * vec3<f32>(1.0, 0.0, 0.0)) + ((vec3<f32>(_e18.g1_.y) * vec3<f32>(_e22.g0_.y)) * vec3<f32>(0.0, 1.0, 0.0))) + ((vec3<f32>(_e33.g1_.z) * vec3<f32>(_e37.g0_.y)) * vec3<f32>(0.0, 0.0, 1.0))) + (_e48.g0_ * vec3<f32>(_e50.g0_.x))), (_e56.g1_ * vec3<f32>(_e58.g0_.x)));
}

fn line_homogeneous_magnitude_regressive_product(self_774: Line, other_628: HomogeneousMagnitude) -> Line {
    var self_775: Line;
    var other_629: HomogeneousMagnitude;

    self_775 = self_774;
    other_629 = other_628;
    let _e4: Line = self_775;
    let _e6: HomogeneousMagnitude = other_629;
    let _e11: Line = self_775;
    let _e13: HomogeneousMagnitude = other_629;
    return Line((_e4.g0_ * vec3<f32>(_e6.g0_.y)), (_e11.g1_ * vec3<f32>(_e13.g0_.y)));
}

fn line_homogeneous_magnitude_outer_product(self_776: Line, other_630: HomogeneousMagnitude) -> Line {
    var self_777: Line;
    var other_631: HomogeneousMagnitude;

    self_777 = self_776;
    other_631 = other_630;
    let _e4: Line = self_777;
    let _e6: HomogeneousMagnitude = other_631;
    let _e11: Line = self_777;
    let _e13: HomogeneousMagnitude = other_631;
    return Line((_e4.g0_ * vec3<f32>(_e6.g0_.x)), (_e11.g1_ * vec3<f32>(_e13.g0_.x)));
}

fn line_homogeneous_magnitude_inner_product(self_778: Line, other_632: HomogeneousMagnitude) -> Line {
    var self_779: Line;
    var other_633: HomogeneousMagnitude;

    self_779 = self_778;
    other_633 = other_632;
    let _e4: Line = self_779;
    let _e8: HomogeneousMagnitude = other_633;
    let _e18: Line = self_779;
    let _e22: HomogeneousMagnitude = other_633;
    let _e33: Line = self_779;
    let _e37: HomogeneousMagnitude = other_633;
    let _e48: Line = self_779;
    let _e50: HomogeneousMagnitude = other_633;
    let _e56: Line = self_779;
    let _e58: HomogeneousMagnitude = other_633;
    return Line((((((vec3<f32>(_e4.g1_.x) * vec3<f32>(_e8.g0_.y)) * vec3<f32>(1.0, 0.0, 0.0)) + ((vec3<f32>(_e18.g1_.y) * vec3<f32>(_e22.g0_.y)) * vec3<f32>(0.0, 1.0, 0.0))) + ((vec3<f32>(_e33.g1_.z) * vec3<f32>(_e37.g0_.y)) * vec3<f32>(0.0, 0.0, 1.0))) + (_e48.g0_ * vec3<f32>(_e50.g0_.x))), (_e56.g1_ * vec3<f32>(_e58.g0_.x)));
}

fn line_homogeneous_magnitude_geometric_anti_product(self_780: Line, other_634: HomogeneousMagnitude) -> Line {
    var self_781: Line;
    var other_635: HomogeneousMagnitude;

    self_781 = self_780;
    other_635 = other_634;
    let _e4: Line = self_781;
    let _e6: HomogeneousMagnitude = other_635;
    let _e11: Line = self_781;
    let _e15: HomogeneousMagnitude = other_635;
    let _e25: Line = self_781;
    let _e29: HomogeneousMagnitude = other_635;
    let _e40: Line = self_781;
    let _e44: HomogeneousMagnitude = other_635;
    let _e55: Line = self_781;
    let _e57: HomogeneousMagnitude = other_635;
    return Line((_e4.g0_ * vec3<f32>(_e6.g0_.y)), (((((vec3<f32>(_e11.g1_.x) * vec3<f32>(_e15.g0_.y)) * vec3<f32>(1.0, 0.0, 0.0)) + ((vec3<f32>(_e25.g1_.y) * vec3<f32>(_e29.g0_.y)) * vec3<f32>(0.0, 1.0, 0.0))) + ((vec3<f32>(_e40.g1_.z) * vec3<f32>(_e44.g0_.y)) * vec3<f32>(0.0, 0.0, 1.0))) + (_e55.g0_ * vec3<f32>(_e57.g0_.x))));
}

fn line_homogeneous_magnitude_inner_anti_product(self_782: Line, other_636: HomogeneousMagnitude) -> Line {
    var self_783: Line;
    var other_637: HomogeneousMagnitude;

    self_783 = self_782;
    other_637 = other_636;
    let _e4: Line = self_783;
    let _e6: HomogeneousMagnitude = other_637;
    let _e11: Line = self_783;
    let _e15: HomogeneousMagnitude = other_637;
    let _e25: Line = self_783;
    let _e29: HomogeneousMagnitude = other_637;
    let _e40: Line = self_783;
    let _e44: HomogeneousMagnitude = other_637;
    let _e55: Line = self_783;
    let _e57: HomogeneousMagnitude = other_637;
    return Line((_e4.g0_ * vec3<f32>(_e6.g0_.y)), (((((vec3<f32>(_e11.g1_.x) * vec3<f32>(_e15.g0_.y)) * vec3<f32>(1.0, 0.0, 0.0)) + ((vec3<f32>(_e25.g1_.y) * vec3<f32>(_e29.g0_.y)) * vec3<f32>(0.0, 1.0, 0.0))) + ((vec3<f32>(_e40.g1_.z) * vec3<f32>(_e44.g0_.y)) * vec3<f32>(0.0, 0.0, 1.0))) + (_e55.g0_ * vec3<f32>(_e57.g0_.x))));
}

fn line_homogeneous_magnitude_right_contraction(self_784: Line, other_638: HomogeneousMagnitude) -> Line {
    var self_785: Line;
    var other_639: HomogeneousMagnitude;

    self_785 = self_784;
    other_639 = other_638;
    let _e4: Line = self_785;
    let _e6: HomogeneousMagnitude = other_639;
    let _e11: Line = self_785;
    let _e13: HomogeneousMagnitude = other_639;
    return Line((_e4.g0_ * vec3<f32>(_e6.g0_.x)), (_e11.g1_ * vec3<f32>(_e13.g0_.x)));
}

fn line_homogeneous_magnitude_right_anti_contraction(self_786: Line, other_640: HomogeneousMagnitude) -> Line {
    var self_787: Line;
    var other_641: HomogeneousMagnitude;

    self_787 = self_786;
    other_641 = other_640;
    let _e4: Line = self_787;
    let _e6: HomogeneousMagnitude = other_641;
    let _e11: Line = self_787;
    let _e13: HomogeneousMagnitude = other_641;
    return Line((_e4.g0_ * vec3<f32>(_e6.g0_.y)), (_e11.g1_ * vec3<f32>(_e13.g0_.y)));
}

fn line_point_geometric_product(self_788: Line, other_642: Point) -> Flector {
    var self_789: Line;
    var other_643: Point;

    self_789 = self_788;
    other_643 = other_642;
    let _e4: Line = self_789;
    let _e8: Point = other_643;
    let _e19: Line = self_789;
    let _e23: Point = other_643;
    let _e35: Line = self_789;
    let _e39: Point = other_643;
    let _e51: Line = self_789;
    let _e55: Point = other_643;
    let _e67: Line = self_789;
    let _e70: Line = self_789;
    let _e73: Line = self_789;
    let _e76: Line = self_789;
    let _e80: Point = other_643;
    let _e92: Line = self_789;
    let _e96: Point = other_643;
    let _e107: Line = self_789;
    let _e111: Point = other_643;
    let _e123: Line = self_789;
    let _e127: Point = other_643;
    let _e139: Line = self_789;
    let _e143: Point = other_643;
    let _e155: Line = self_789;
    let _e158: Line = self_789;
    let _e161: Line = self_789;
    let _e164: Line = self_789;
    let _e168: Point = other_643;
    return Flector(((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y)) * vec4<f32>(0.0, 0.0, 0.0, 1.0)) + ((vec4<f32>(_e19.g0_.z) * vec4<f32>(_e23.g0_.z)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e35.g1_.y) * _e39.g0_.zzxz) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0))) + ((vec4<f32>(_e51.g1_.z) * _e55.g0_.yxyy) * vec4<f32>(1.0, -(1.0), 0.0, 0.0))) + ((vec4<f32>(_e67.g0_.x, _e70.g1_.x, _e73.g1_.x, _e76.g0_.x) * _e80.g0_.xzyx) * vec4<f32>(0.0, 1.0, -(1.0), 1.0))), ((((((vec4<f32>(_e92.g0_.y) * _e96.g0_.zzxz) * vec4<f32>(1.0, 0.0, -(1.0), 0.0)) + ((vec4<f32>(_e107.g0_.z) * _e111.g0_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))) + ((vec4<f32>(_e123.g1_.y) * _e127.g0_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0)))) + ((vec4<f32>(_e139.g1_.z) * _e143.g0_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e155.g1_.x, _e158.g0_.x, _e161.g0_.x, _e164.g1_.x) * _e168.g0_.wzyx) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))));
}

fn line_point_outer_product(self_790: Line, other_644: Point) -> Plane {
    var self_791: Line;
    var other_645: Point;

    self_791 = self_790;
    other_645 = other_644;
    let _e4: Line = self_791;
    let _e8: Point = other_645;
    let _e19: Line = self_791;
    let _e23: Point = other_645;
    let _e35: Line = self_791;
    let _e39: Point = other_645;
    let _e51: Line = self_791;
    let _e55: Point = other_645;
    let _e67: Line = self_791;
    let _e70: Line = self_791;
    let _e73: Line = self_791;
    let _e76: Line = self_791;
    let _e80: Point = other_645;
    return Plane(((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.zzxz) * vec4<f32>(1.0, 0.0, -(1.0), 0.0)) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))) + ((vec4<f32>(_e35.g1_.y) * _e39.g0_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0)))) + ((vec4<f32>(_e51.g1_.z) * _e55.g0_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e67.g1_.x, _e70.g0_.x, _e73.g0_.x, _e76.g1_.x) * _e80.g0_.wzyx) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))));
}

fn line_point_inner_product(self_792: Line, other_646: Point) -> Point {
    var self_793: Line;
    var other_647: Point;

    self_793 = self_792;
    other_647 = other_646;
    let _e4: Line = self_793;
    let _e8: Point = other_647;
    let _e19: Line = self_793;
    let _e23: Point = other_647;
    let _e35: Line = self_793;
    let _e39: Point = other_647;
    let _e51: Line = self_793;
    let _e55: Point = other_647;
    let _e67: Line = self_793;
    let _e70: Line = self_793;
    let _e73: Line = self_793;
    let _e76: Line = self_793;
    let _e80: Point = other_647;
    return Point(((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y)) * vec4<f32>(0.0, 0.0, 0.0, 1.0)) + ((vec4<f32>(_e19.g0_.z) * vec4<f32>(_e23.g0_.z)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e35.g1_.y) * _e39.g0_.zzxz) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0))) + ((vec4<f32>(_e51.g1_.z) * _e55.g0_.yxyy) * vec4<f32>(1.0, -(1.0), 0.0, 0.0))) + ((vec4<f32>(_e67.g0_.x, _e70.g1_.x, _e73.g1_.x, _e76.g0_.x) * _e80.g0_.xzyx) * vec4<f32>(0.0, 1.0, -(1.0), 1.0))));
}

fn line_point_inner_anti_product(self_794: Line, other_648: Point) -> Plane {
    var self_795: Line;
    var other_649: Point;

    self_795 = self_794;
    other_649 = other_648;
    let _e4: Line = self_795;
    let _e8: Point = other_649;
    let _e19: Line = self_795;
    let _e23: Point = other_649;
    let _e35: Line = self_795;
    let _e39: Point = other_649;
    return Plane(((((vec4<f32>(_e4.g0_.y) * _e8.g0_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e35.g0_.x) * _e39.g0_.wxxx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))));
}

fn line_point_right_contraction(self_796: Line, other_650: Point) -> Point {
    var self_797: Line;
    var other_651: Point;

    self_797 = self_796;
    other_651 = other_650;
    let _e4: Line = self_797;
    let _e8: Point = other_651;
    let _e19: Line = self_797;
    let _e23: Point = other_651;
    let _e35: Line = self_797;
    let _e39: Point = other_651;
    let _e51: Line = self_797;
    let _e55: Point = other_651;
    let _e67: Line = self_797;
    let _e70: Line = self_797;
    let _e73: Line = self_797;
    let _e76: Line = self_797;
    let _e80: Point = other_651;
    return Point(((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y)) * vec4<f32>(0.0, 0.0, 0.0, 1.0)) + ((vec4<f32>(_e19.g0_.z) * vec4<f32>(_e23.g0_.z)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e35.g1_.y) * _e39.g0_.zzxz) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0))) + ((vec4<f32>(_e51.g1_.z) * _e55.g0_.yxyy) * vec4<f32>(1.0, -(1.0), 0.0, 0.0))) + ((vec4<f32>(_e67.g0_.x, _e70.g1_.x, _e73.g1_.x, _e76.g0_.x) * _e80.g0_.xzyx) * vec4<f32>(0.0, 1.0, -(1.0), 1.0))));
}

fn line_point_left_anti_contraction(self_798: Line, other_652: Point) -> Plane {
    var self_799: Line;
    var other_653: Point;

    self_799 = self_798;
    other_653 = other_652;
    let _e4: Line = self_799;
    let _e8: Point = other_653;
    let _e19: Line = self_799;
    let _e23: Point = other_653;
    let _e35: Line = self_799;
    let _e39: Point = other_653;
    return Plane(((((vec4<f32>(_e4.g0_.y) * _e8.g0_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e35.g0_.x) * _e39.g0_.wxxx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))));
}

fn line_line_add(self_800: Line, other_654: Line) -> Line {
    var self_801: Line;
    var other_655: Line;

    self_801 = self_800;
    other_655 = other_654;
    let _e4: Line = self_801;
    let _e6: Line = other_655;
    let _e9: Line = self_801;
    let _e11: Line = other_655;
    return Line((_e4.g0_ + _e6.g0_), (_e9.g1_ + _e11.g1_));
}

fn line_line_sub(self_802: Line, other_656: Line) -> Line {
    var self_803: Line;
    var other_657: Line;

    self_803 = self_802;
    other_657 = other_656;
    let _e4: Line = self_803;
    let _e6: Line = other_657;
    let _e9: Line = self_803;
    let _e11: Line = other_657;
    return Line((_e4.g0_ - _e6.g0_), (_e9.g1_ - _e11.g1_));
}

fn line_line_mul(self_804: Line, other_658: Line) -> Line {
    var self_805: Line;
    var other_659: Line;

    self_805 = self_804;
    other_659 = other_658;
    let _e4: Line = self_805;
    let _e6: Line = other_659;
    let _e9: Line = self_805;
    let _e11: Line = other_659;
    return Line((_e4.g0_ * _e6.g0_), (_e9.g1_ * _e11.g1_));
}

fn line_line_div(self_806: Line, other_660: Line) -> Line {
    var self_807: Line;
    var other_661: Line;

    self_807 = self_806;
    other_661 = other_660;
    let _e4: Line = self_807;
    let _e7: Line = self_807;
    let _e10: Line = self_807;
    let _e19: Line = other_661;
    let _e22: Line = other_661;
    let _e25: Line = other_661;
    let _e35: Line = self_807;
    let _e38: Line = self_807;
    let _e41: Line = self_807;
    let _e50: Line = other_661;
    let _e53: Line = other_661;
    let _e56: Line = other_661;
    return Line((((vec3<f32>(_e4.g0_.x, _e7.g0_.y, _e10.g0_.z) * vec3<f32>(1.0, 1.0, 1.0)) / vec3<f32>(_e19.g0_.x, _e22.g0_.y, _e25.g0_.z)) * vec3<f32>(1.0, 1.0, 1.0)), (((vec3<f32>(_e35.g1_.x, _e38.g1_.y, _e41.g1_.z) * vec3<f32>(1.0, 1.0, 1.0)) / vec3<f32>(_e50.g1_.x, _e53.g1_.y, _e56.g1_.z)) * vec3<f32>(1.0, 1.0, 1.0)));
}

fn line_line_regressive_product(self_808: Line, other_662: Line) -> Scalar {
    var self_809: Line;
    var other_663: Line;

    self_809 = self_808;
    other_663 = other_662;
    let _e5: Line = self_809;
    let _e8: Line = other_663;
    let _e13: Line = self_809;
    let _e16: Line = other_663;
    let _e21: Line = self_809;
    let _e24: Line = other_663;
    let _e29: Line = self_809;
    let _e32: Line = other_663;
    let _e37: Line = self_809;
    let _e40: Line = other_663;
    let _e45: Line = self_809;
    let _e48: Line = other_663;
    return Scalar(((((((0.0 - (_e5.g0_.x * _e8.g1_.x)) - (_e13.g0_.y * _e16.g1_.y)) - (_e21.g0_.z * _e24.g1_.z)) - (_e29.g1_.x * _e32.g0_.x)) - (_e37.g1_.y * _e40.g0_.y)) - (_e45.g1_.z * _e48.g0_.z)));
}

fn line_line_outer_product(self_810: Line, other_664: Line) -> AntiScalar {
    var self_811: Line;
    var other_665: Line;

    self_811 = self_810;
    other_665 = other_664;
    let _e5: Line = self_811;
    let _e8: Line = other_665;
    let _e13: Line = self_811;
    let _e16: Line = other_665;
    let _e21: Line = self_811;
    let _e24: Line = other_665;
    let _e29: Line = self_811;
    let _e32: Line = other_665;
    let _e37: Line = self_811;
    let _e40: Line = other_665;
    let _e45: Line = self_811;
    let _e48: Line = other_665;
    return AntiScalar(((((((0.0 - (_e5.g0_.x * _e8.g1_.x)) - (_e13.g0_.y * _e16.g1_.y)) - (_e21.g0_.z * _e24.g1_.z)) - (_e29.g1_.x * _e32.g0_.x)) - (_e37.g1_.y * _e40.g0_.y)) - (_e45.g1_.z * _e48.g0_.z)));
}

fn line_line_inner_product(self_812: Line, other_666: Line) -> Scalar {
    var self_813: Line;
    var other_667: Line;

    self_813 = self_812;
    other_667 = other_666;
    let _e5: Line = self_813;
    let _e8: Line = other_667;
    let _e13: Line = self_813;
    let _e16: Line = other_667;
    let _e21: Line = self_813;
    let _e24: Line = other_667;
    return Scalar((((0.0 - (_e5.g1_.x * _e8.g1_.x)) - (_e13.g1_.y * _e16.g1_.y)) - (_e21.g1_.z * _e24.g1_.z)));
}

fn line_line_inner_anti_product(self_814: Line, other_668: Line) -> AntiScalar {
    var self_815: Line;
    var other_669: Line;

    self_815 = self_814;
    other_669 = other_668;
    let _e5: Line = self_815;
    let _e8: Line = other_669;
    let _e13: Line = self_815;
    let _e16: Line = other_669;
    let _e21: Line = self_815;
    let _e24: Line = other_669;
    return AntiScalar((((0.0 - (_e5.g0_.x * _e8.g0_.x)) - (_e13.g0_.y * _e16.g0_.y)) - (_e21.g0_.z * _e24.g0_.z)));
}

fn line_line_left_contraction(self_816: Line, other_670: Line) -> Scalar {
    var self_817: Line;
    var other_671: Line;

    self_817 = self_816;
    other_671 = other_670;
    let _e5: Line = self_817;
    let _e8: Line = other_671;
    let _e13: Line = self_817;
    let _e16: Line = other_671;
    let _e21: Line = self_817;
    let _e24: Line = other_671;
    return Scalar((((0.0 - (_e5.g1_.x * _e8.g1_.x)) - (_e13.g1_.y * _e16.g1_.y)) - (_e21.g1_.z * _e24.g1_.z)));
}

fn line_line_right_contraction(self_818: Line, other_672: Line) -> Scalar {
    var self_819: Line;
    var other_673: Line;

    self_819 = self_818;
    other_673 = other_672;
    let _e5: Line = self_819;
    let _e8: Line = other_673;
    let _e13: Line = self_819;
    let _e16: Line = other_673;
    let _e21: Line = self_819;
    let _e24: Line = other_673;
    return Scalar((((0.0 - (_e5.g1_.x * _e8.g1_.x)) - (_e13.g1_.y * _e16.g1_.y)) - (_e21.g1_.z * _e24.g1_.z)));
}

fn line_line_left_anti_contraction(self_820: Line, other_674: Line) -> AntiScalar {
    var self_821: Line;
    var other_675: Line;

    self_821 = self_820;
    other_675 = other_674;
    let _e5: Line = self_821;
    let _e8: Line = other_675;
    let _e13: Line = self_821;
    let _e16: Line = other_675;
    let _e21: Line = self_821;
    let _e24: Line = other_675;
    return AntiScalar((((0.0 - (_e5.g0_.x * _e8.g0_.x)) - (_e13.g0_.y * _e16.g0_.y)) - (_e21.g0_.z * _e24.g0_.z)));
}

fn line_line_right_anti_contraction(self_822: Line, other_676: Line) -> AntiScalar {
    var self_823: Line;
    var other_677: Line;

    self_823 = self_822;
    other_677 = other_676;
    let _e5: Line = self_823;
    let _e8: Line = other_677;
    let _e13: Line = self_823;
    let _e16: Line = other_677;
    let _e21: Line = self_823;
    let _e24: Line = other_677;
    return AntiScalar((((0.0 - (_e5.g0_.x * _e8.g0_.x)) - (_e13.g0_.y * _e16.g0_.y)) - (_e21.g0_.z * _e24.g0_.z)));
}

fn line_line_scalar_product(self_824: Line, other_678: Line) -> Scalar {
    var self_825: Line;
    var other_679: Line;

    self_825 = self_824;
    other_679 = other_678;
    let _e5: Line = self_825;
    let _e8: Line = other_679;
    let _e13: Line = self_825;
    let _e16: Line = other_679;
    let _e21: Line = self_825;
    let _e24: Line = other_679;
    return Scalar((((0.0 - (_e5.g1_.x * _e8.g1_.x)) - (_e13.g1_.y * _e16.g1_.y)) - (_e21.g1_.z * _e24.g1_.z)));
}

fn line_line_anti_scalar_product(self_826: Line, other_680: Line) -> AntiScalar {
    var self_827: Line;
    var other_681: Line;

    self_827 = self_826;
    other_681 = other_680;
    let _e5: Line = self_827;
    let _e8: Line = other_681;
    let _e13: Line = self_827;
    let _e16: Line = other_681;
    let _e21: Line = self_827;
    let _e24: Line = other_681;
    return AntiScalar((((0.0 - (_e5.g0_.x * _e8.g0_.x)) - (_e13.g0_.y * _e16.g0_.y)) - (_e21.g0_.z * _e24.g0_.z)));
}

fn line_plane_regressive_product(self_828: Line, other_682: Plane) -> Point {
    var self_829: Line;
    var other_683: Plane;

    self_829 = self_828;
    other_683 = other_682;
    let _e4: Line = self_829;
    let _e8: Plane = other_683;
    let _e19: Line = self_829;
    let _e23: Plane = other_683;
    let _e35: Line = self_829;
    let _e39: Plane = other_683;
    let _e51: Line = self_829;
    let _e55: Plane = other_683;
    let _e67: Line = self_829;
    let _e70: Line = self_829;
    let _e73: Line = self_829;
    let _e76: Line = self_829;
    let _e80: Plane = other_683;
    return Point(((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e35.g1_.y) * _e39.g0_.zzxz) * vec4<f32>(1.0, 0.0, -(1.0), 0.0))) + ((vec4<f32>(_e51.g1_.z) * _e55.g0_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))) + ((vec4<f32>(_e67.g0_.x, _e70.g1_.x, _e73.g1_.x, _e76.g0_.x) * _e80.g0_.wzyx) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))));
}

fn line_plane_inner_product(self_830: Line, other_684: Plane) -> Point {
    var self_831: Line;
    var other_685: Plane;

    self_831 = self_830;
    other_685 = other_684;
    let _e4: Line = self_831;
    let _e8: Plane = other_685;
    let _e19: Line = self_831;
    let _e23: Plane = other_685;
    let _e35: Line = self_831;
    let _e39: Plane = other_685;
    return Point(((((vec4<f32>(_e4.g1_.y) * _e8.g0_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e19.g1_.z) * _e23.g0_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e35.g1_.x) * _e39.g0_.wxxx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))));
}

fn line_plane_geometric_anti_product(self_832: Line, other_686: Plane) -> Flector {
    var self_833: Line;
    var other_687: Plane;

    self_833 = self_832;
    other_687 = other_686;
    let _e4: Line = self_833;
    let _e8: Plane = other_687;
    let _e19: Line = self_833;
    let _e23: Plane = other_687;
    let _e35: Line = self_833;
    let _e39: Plane = other_687;
    let _e51: Line = self_833;
    let _e55: Plane = other_687;
    let _e67: Line = self_833;
    let _e70: Line = self_833;
    let _e73: Line = self_833;
    let _e76: Line = self_833;
    let _e80: Plane = other_687;
    let _e93: Line = self_833;
    let _e97: Plane = other_687;
    let _e108: Line = self_833;
    let _e112: Plane = other_687;
    let _e124: Line = self_833;
    let _e128: Plane = other_687;
    let _e141: Line = self_833;
    let _e145: Plane = other_687;
    let _e158: Line = self_833;
    let _e161: Line = self_833;
    let _e164: Line = self_833;
    let _e167: Line = self_833;
    let _e171: Plane = other_687;
    return Flector(((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e35.g1_.y) * _e39.g0_.zzxz) * vec4<f32>(1.0, 0.0, -(1.0), 0.0))) + ((vec4<f32>(_e51.g1_.z) * _e55.g0_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))) + ((vec4<f32>(_e67.g0_.x, _e70.g1_.x, _e73.g1_.x, _e76.g0_.x) * _e80.g0_.wzyx) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))), ((((((vec4<f32>(_e93.g0_.y) * _e97.g0_.zzxz) * vec4<f32>(1.0, 0.0, -(1.0), 0.0)) + ((vec4<f32>(_e108.g0_.z) * _e112.g0_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))) + ((vec4<f32>(_e124.g1_.y) * vec4<f32>(_e128.g0_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e141.g1_.z) * vec4<f32>(_e145.g0_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e158.g0_.x, _e161.g0_.x, _e164.g0_.x, _e167.g1_.x) * _e171.g0_.xzyx) * vec4<f32>(0.0, -(1.0), 1.0, -(1.0)))));
}

fn line_plane_inner_anti_product(self_834: Line, other_688: Plane) -> Plane {
    var self_835: Line;
    var other_689: Plane;

    self_835 = self_834;
    other_689 = other_688;
    let _e4: Line = self_835;
    let _e8: Plane = other_689;
    let _e19: Line = self_835;
    let _e23: Plane = other_689;
    let _e35: Line = self_835;
    let _e39: Plane = other_689;
    let _e52: Line = self_835;
    let _e56: Plane = other_689;
    let _e69: Line = self_835;
    let _e72: Line = self_835;
    let _e75: Line = self_835;
    let _e78: Line = self_835;
    let _e82: Plane = other_689;
    return Plane(((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.zzxz) * vec4<f32>(1.0, 0.0, -(1.0), 0.0)) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))) + ((vec4<f32>(_e35.g1_.y) * vec4<f32>(_e39.g0_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e52.g1_.z) * vec4<f32>(_e56.g0_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e69.g0_.x, _e72.g0_.x, _e75.g0_.x, _e78.g1_.x) * _e82.g0_.xzyx) * vec4<f32>(0.0, -(1.0), 1.0, -(1.0)))));
}

fn line_plane_left_contraction(self_836: Line, other_690: Plane) -> Point {
    var self_837: Line;
    var other_691: Plane;

    self_837 = self_836;
    other_691 = other_690;
    let _e4: Line = self_837;
    let _e8: Plane = other_691;
    let _e19: Line = self_837;
    let _e23: Plane = other_691;
    let _e35: Line = self_837;
    let _e39: Plane = other_691;
    return Point(((((vec4<f32>(_e4.g1_.y) * _e8.g0_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e19.g1_.z) * _e23.g0_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e35.g1_.x) * _e39.g0_.wxxx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))));
}

fn line_plane_right_anti_contraction(self_838: Line, other_692: Plane) -> Plane {
    var self_839: Line;
    var other_693: Plane;

    self_839 = self_838;
    other_693 = other_692;
    let _e4: Line = self_839;
    let _e8: Plane = other_693;
    let _e19: Line = self_839;
    let _e23: Plane = other_693;
    let _e35: Line = self_839;
    let _e39: Plane = other_693;
    let _e52: Line = self_839;
    let _e56: Plane = other_693;
    let _e69: Line = self_839;
    let _e72: Line = self_839;
    let _e75: Line = self_839;
    let _e78: Line = self_839;
    let _e82: Plane = other_693;
    return Plane(((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.zzxz) * vec4<f32>(1.0, 0.0, -(1.0), 0.0)) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))) + ((vec4<f32>(_e35.g1_.y) * vec4<f32>(_e39.g0_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e52.g1_.z) * vec4<f32>(_e56.g0_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e69.g0_.x, _e72.g0_.x, _e75.g0_.x, _e78.g1_.x) * _e82.g0_.xzyx) * vec4<f32>(0.0, -(1.0), 1.0, -(1.0)))));
}

fn line_motor_add(self_840: Line, other_694: Motor) -> Motor {
    var self_841: Line;
    var other_695: Motor;

    self_841 = self_840;
    other_695 = other_694;
    let _e4: Line = self_841;
    let _e7: Line = self_841;
    let _e10: Line = self_841;
    let _e13: Line = self_841;
    let _e23: Motor = other_695;
    let _e26: Line = self_841;
    let _e28: Motor = other_695;
    return Motor(((vec4<f32>(_e4.g0_.x, _e7.g0_.y, _e10.g0_.z, _e13.g0_.x) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) + _e23.g0_), (_e26.g1_ + _e28.g1_));
}

fn line_motor_sub(self_842: Line, other_696: Motor) -> Motor {
    var self_843: Line;
    var other_697: Motor;

    self_843 = self_842;
    other_697 = other_696;
    let _e4: Line = self_843;
    let _e7: Line = self_843;
    let _e10: Line = self_843;
    let _e13: Line = self_843;
    let _e23: Motor = other_697;
    let _e26: Line = self_843;
    let _e28: Motor = other_697;
    return Motor(((vec4<f32>(_e4.g0_.x, _e7.g0_.y, _e10.g0_.z, _e13.g0_.x) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) - _e23.g0_), (_e26.g1_ - _e28.g1_));
}

fn line_motor_outer_product(self_844: Line, other_698: Motor) -> AntiScalar {
    var self_845: Line;
    var other_699: Motor;

    self_845 = self_844;
    other_699 = other_698;
    let _e5: Line = self_845;
    let _e8: Motor = other_699;
    let _e13: Line = self_845;
    let _e16: Motor = other_699;
    let _e21: Line = self_845;
    let _e24: Motor = other_699;
    let _e29: Line = self_845;
    let _e32: Motor = other_699;
    let _e37: Line = self_845;
    let _e40: Motor = other_699;
    let _e45: Line = self_845;
    let _e48: Motor = other_699;
    return AntiScalar(((((((0.0 - (_e5.g0_.x * _e8.g1_.x)) - (_e13.g0_.y * _e16.g1_.y)) - (_e21.g0_.z * _e24.g1_.z)) - (_e29.g1_.x * _e32.g0_.x)) - (_e37.g1_.y * _e40.g0_.y)) - (_e45.g1_.z * _e48.g0_.z)));
}

fn line_motor_inner_anti_product(self_846: Line, other_700: Motor) -> Motor {
    var self_847: Line;
    var other_701: Motor;

    self_847 = self_846;
    other_701 = other_700;
    let _e4: Line = self_847;
    let _e8: Motor = other_701;
    let _e19: Line = self_847;
    let _e23: Motor = other_701;
    let _e35: Line = self_847;
    let _e39: Motor = other_701;
    let _e51: Line = self_847;
    let _e53: Motor = other_701;
    return Motor(((((vec4<f32>(_e4.g0_.y) * _e8.g0_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e35.g0_.x) * _e39.g0_.wxxx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))), (_e51.g1_ * vec3<f32>(_e53.g0_.w)));
}

fn line_motor_right_contraction(self_848: Line, other_702: Motor) -> Scalar {
    var self_849: Line;
    var other_703: Motor;

    self_849 = self_848;
    other_703 = other_702;
    let _e5: Line = self_849;
    let _e8: Motor = other_703;
    let _e13: Line = self_849;
    let _e16: Motor = other_703;
    let _e21: Line = self_849;
    let _e24: Motor = other_703;
    return Scalar((((0.0 - (_e5.g1_.x * _e8.g1_.x)) - (_e13.g1_.y * _e16.g1_.y)) - (_e21.g1_.z * _e24.g1_.z)));
}

fn line_motor_left_anti_contraction(self_850: Line, other_704: Motor) -> AntiScalar {
    var self_851: Line;
    var other_705: Motor;

    self_851 = self_850;
    other_705 = other_704;
    let _e5: Line = self_851;
    let _e8: Motor = other_705;
    let _e13: Line = self_851;
    let _e16: Motor = other_705;
    let _e21: Line = self_851;
    let _e24: Motor = other_705;
    return AntiScalar((((0.0 - (_e5.g0_.x * _e8.g0_.x)) - (_e13.g0_.y * _e16.g0_.y)) - (_e21.g0_.z * _e24.g0_.z)));
}

fn line_motor_right_anti_contraction(self_852: Line, other_706: Motor) -> Motor {
    var self_853: Line;
    var other_707: Motor;

    self_853 = self_852;
    other_707 = other_706;
    let _e4: Line = self_853;
    let _e8: Motor = other_707;
    let _e19: Line = self_853;
    let _e23: Motor = other_707;
    let _e35: Line = self_853;
    let _e39: Motor = other_707;
    let _e51: Line = self_853;
    let _e53: Motor = other_707;
    return Motor(((((vec4<f32>(_e4.g0_.y) * _e8.g0_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e35.g0_.x) * _e39.g0_.wxxx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))), (_e51.g1_ * vec3<f32>(_e53.g0_.w)));
}

fn line_motor_scalar_product(self_854: Line, other_708: Motor) -> Scalar {
    var self_855: Line;
    var other_709: Motor;

    self_855 = self_854;
    other_709 = other_708;
    let _e5: Line = self_855;
    let _e8: Motor = other_709;
    let _e13: Line = self_855;
    let _e16: Motor = other_709;
    let _e21: Line = self_855;
    let _e24: Motor = other_709;
    return Scalar((((0.0 - (_e5.g1_.x * _e8.g1_.x)) - (_e13.g1_.y * _e16.g1_.y)) - (_e21.g1_.z * _e24.g1_.z)));
}

fn line_motor_anti_scalar_product(self_856: Line, other_710: Motor) -> AntiScalar {
    var self_857: Line;
    var other_711: Motor;

    self_857 = self_856;
    other_711 = other_710;
    let _e5: Line = self_857;
    let _e8: Motor = other_711;
    let _e13: Line = self_857;
    let _e16: Motor = other_711;
    let _e21: Line = self_857;
    let _e24: Motor = other_711;
    return AntiScalar((((0.0 - (_e5.g0_.x * _e8.g0_.x)) - (_e13.g0_.y * _e16.g0_.y)) - (_e21.g0_.z * _e24.g0_.z)));
}

fn line_rotor_add(self_858: Line, other_712: Rotor) -> Motor {
    var self_859: Line;
    var other_713: Rotor;

    self_859 = self_858;
    other_713 = other_712;
    let _e4: Line = self_859;
    let _e7: Line = self_859;
    let _e10: Line = self_859;
    let _e13: Line = self_859;
    let _e23: Rotor = other_713;
    let _e26: Line = self_859;
    return Motor(((vec4<f32>(_e4.g0_.x, _e7.g0_.y, _e10.g0_.z, _e13.g0_.x) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) + _e23.g0_), _e26.g1_);
}

fn line_rotor_sub(self_860: Line, other_714: Rotor) -> Motor {
    var self_861: Line;
    var other_715: Rotor;

    self_861 = self_860;
    other_715 = other_714;
    let _e4: Line = self_861;
    let _e7: Line = self_861;
    let _e10: Line = self_861;
    let _e13: Line = self_861;
    let _e23: Rotor = other_715;
    let _e26: Line = self_861;
    return Motor(((vec4<f32>(_e4.g0_.x, _e7.g0_.y, _e10.g0_.z, _e13.g0_.x) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) - _e23.g0_), _e26.g1_);
}

fn line_rotor_geometric_product(self_862: Line, other_716: Rotor) -> Rotor {
    var self_863: Line;
    var other_717: Rotor;

    self_863 = self_862;
    other_717 = other_716;
    let _e4: Line = self_863;
    let _e8: Rotor = other_717;
    let _e20: Line = self_863;
    let _e24: Rotor = other_717;
    let _e37: Line = self_863;
    let _e41: Rotor = other_717;
    return Rotor(((((vec4<f32>(_e4.g1_.x) * _e8.g0_.wzyx) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0))) + ((vec4<f32>(_e20.g1_.y) * _e24.g0_.zwxy) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e37.g1_.z) * _e41.g0_.yxwz) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))));
}

fn line_rotor_outer_product(self_864: Line, other_718: Rotor) -> AntiScalar {
    var self_865: Line;
    var other_719: Rotor;

    self_865 = self_864;
    other_719 = other_718;
    let _e5: Line = self_865;
    let _e8: Rotor = other_719;
    let _e13: Line = self_865;
    let _e16: Rotor = other_719;
    let _e21: Line = self_865;
    let _e24: Rotor = other_719;
    return AntiScalar((((0.0 - (_e5.g1_.x * _e8.g0_.x)) - (_e13.g1_.y * _e16.g0_.y)) - (_e21.g1_.z * _e24.g0_.z)));
}

fn line_rotor_inner_anti_product(self_866: Line, other_720: Rotor) -> Motor {
    var self_867: Line;
    var other_721: Rotor;

    self_867 = self_866;
    other_721 = other_720;
    let _e4: Line = self_867;
    let _e8: Rotor = other_721;
    let _e19: Line = self_867;
    let _e23: Rotor = other_721;
    let _e35: Line = self_867;
    let _e39: Rotor = other_721;
    let _e51: Line = self_867;
    let _e53: Rotor = other_721;
    return Motor(((((vec4<f32>(_e4.g0_.y) * _e8.g0_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e35.g0_.x) * _e39.g0_.wxxx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))), (_e51.g1_ * vec3<f32>(_e53.g0_.w)));
}

fn line_rotor_left_anti_contraction(self_868: Line, other_722: Rotor) -> AntiScalar {
    var self_869: Line;
    var other_723: Rotor;

    self_869 = self_868;
    other_723 = other_722;
    let _e5: Line = self_869;
    let _e8: Rotor = other_723;
    let _e13: Line = self_869;
    let _e16: Rotor = other_723;
    let _e21: Line = self_869;
    let _e24: Rotor = other_723;
    return AntiScalar((((0.0 - (_e5.g0_.x * _e8.g0_.x)) - (_e13.g0_.y * _e16.g0_.y)) - (_e21.g0_.z * _e24.g0_.z)));
}

fn line_rotor_right_anti_contraction(self_870: Line, other_724: Rotor) -> Motor {
    var self_871: Line;
    var other_725: Rotor;

    self_871 = self_870;
    other_725 = other_724;
    let _e4: Line = self_871;
    let _e8: Rotor = other_725;
    let _e19: Line = self_871;
    let _e23: Rotor = other_725;
    let _e35: Line = self_871;
    let _e39: Rotor = other_725;
    let _e51: Line = self_871;
    let _e53: Rotor = other_725;
    return Motor(((((vec4<f32>(_e4.g0_.y) * _e8.g0_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e35.g0_.x) * _e39.g0_.wxxx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))), (_e51.g1_ * vec3<f32>(_e53.g0_.w)));
}

fn line_rotor_anti_scalar_product(self_872: Line, other_726: Rotor) -> AntiScalar {
    var self_873: Line;
    var other_727: Rotor;

    self_873 = self_872;
    other_727 = other_726;
    let _e5: Line = self_873;
    let _e8: Rotor = other_727;
    let _e13: Line = self_873;
    let _e16: Rotor = other_727;
    let _e21: Line = self_873;
    let _e24: Rotor = other_727;
    return AntiScalar((((0.0 - (_e5.g0_.x * _e8.g0_.x)) - (_e13.g0_.y * _e16.g0_.y)) - (_e21.g0_.z * _e24.g0_.z)));
}

fn line_translator_add(self_874: Line, other_728: Translator) -> Motor {
    var self_875: Line;
    var other_729: Translator;

    self_875 = self_874;
    other_729 = other_728;
    let _e4: Line = self_875;
    let _e7: Line = self_875;
    let _e10: Line = self_875;
    let _e13: Line = self_875;
    let _e23: Translator = other_729;
    let _e33: Line = self_875;
    let _e35: Translator = other_729;
    let _e38: Translator = other_729;
    let _e41: Translator = other_729;
    return Motor(((vec4<f32>(_e4.g0_.x, _e7.g0_.y, _e10.g0_.z, _e13.g0_.x) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) + (_e23.g0_.xxxw * vec4<f32>(0.0, 0.0, 0.0, 1.0))), (_e33.g1_ + vec3<f32>(_e35.g0_.x, _e38.g0_.y, _e41.g0_.z)));
}

fn line_translator_sub(self_876: Line, other_730: Translator) -> Motor {
    var self_877: Line;
    var other_731: Translator;

    self_877 = self_876;
    other_731 = other_730;
    let _e4: Line = self_877;
    let _e7: Line = self_877;
    let _e10: Line = self_877;
    let _e13: Line = self_877;
    let _e23: Translator = other_731;
    let _e33: Line = self_877;
    let _e35: Translator = other_731;
    let _e38: Translator = other_731;
    let _e41: Translator = other_731;
    return Motor(((vec4<f32>(_e4.g0_.x, _e7.g0_.y, _e10.g0_.z, _e13.g0_.x) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) - (_e23.g0_.xxxw * vec4<f32>(0.0, 0.0, 0.0, 1.0))), (_e33.g1_ - vec3<f32>(_e35.g0_.x, _e38.g0_.y, _e41.g0_.z)));
}

fn line_translator_outer_product(self_878: Line, other_732: Translator) -> AntiScalar {
    var self_879: Line;
    var other_733: Translator;

    self_879 = self_878;
    other_733 = other_732;
    let _e5: Line = self_879;
    let _e8: Translator = other_733;
    let _e13: Line = self_879;
    let _e16: Translator = other_733;
    let _e21: Line = self_879;
    let _e24: Translator = other_733;
    return AntiScalar((((0.0 - (_e5.g0_.x * _e8.g0_.x)) - (_e13.g0_.y * _e16.g0_.y)) - (_e21.g0_.z * _e24.g0_.z)));
}

fn line_translator_inner_anti_product(self_880: Line, other_734: Translator) -> Line {
    var self_881: Line;
    var other_735: Translator;

    self_881 = self_880;
    other_735 = other_734;
    let _e4: Line = self_881;
    let _e6: Translator = other_735;
    let _e11: Line = self_881;
    let _e13: Translator = other_735;
    return Line((_e4.g0_ * vec3<f32>(_e6.g0_.w)), (_e11.g1_ * vec3<f32>(_e13.g0_.w)));
}

fn line_translator_right_contraction(self_882: Line, other_736: Translator) -> Scalar {
    var self_883: Line;
    var other_737: Translator;

    self_883 = self_882;
    other_737 = other_736;
    let _e5: Line = self_883;
    let _e8: Translator = other_737;
    let _e13: Line = self_883;
    let _e16: Translator = other_737;
    let _e21: Line = self_883;
    let _e24: Translator = other_737;
    return Scalar((((0.0 - (_e5.g1_.x * _e8.g0_.x)) - (_e13.g1_.y * _e16.g0_.y)) - (_e21.g1_.z * _e24.g0_.z)));
}

fn line_translator_right_anti_contraction(self_884: Line, other_738: Translator) -> Line {
    var self_885: Line;
    var other_739: Translator;

    self_885 = self_884;
    other_739 = other_738;
    let _e4: Line = self_885;
    let _e6: Translator = other_739;
    let _e11: Line = self_885;
    let _e13: Translator = other_739;
    return Line((_e4.g0_ * vec3<f32>(_e6.g0_.w)), (_e11.g1_ * vec3<f32>(_e13.g0_.w)));
}

fn line_translator_scalar_product(self_886: Line, other_740: Translator) -> Scalar {
    var self_887: Line;
    var other_741: Translator;

    self_887 = self_886;
    other_741 = other_740;
    let _e5: Line = self_887;
    let _e8: Translator = other_741;
    let _e13: Line = self_887;
    let _e16: Translator = other_741;
    let _e21: Line = self_887;
    let _e24: Translator = other_741;
    return Scalar((((0.0 - (_e5.g1_.x * _e8.g0_.x)) - (_e13.g1_.y * _e16.g0_.y)) - (_e21.g1_.z * _e24.g0_.z)));
}

fn line_flector_geometric_product(self_888: Line, other_742: Flector) -> Flector {
    var self_889: Line;
    var other_743: Flector;

    self_889 = self_888;
    other_743 = other_742;
    let _e4: Line = self_889;
    let _e8: Flector = other_743;
    let _e19: Line = self_889;
    let _e23: Flector = other_743;
    let _e35: Line = self_889;
    let _e39: Flector = other_743;
    let _e42: Flector = other_743;
    let _e45: Flector = other_743;
    let _e48: Flector = other_743;
    let _e62: Line = self_889;
    let _e66: Flector = other_743;
    let _e69: Flector = other_743;
    let _e72: Flector = other_743;
    let _e75: Flector = other_743;
    let _e89: Line = self_889;
    let _e93: Flector = other_743;
    let _e96: Flector = other_743;
    let _e99: Flector = other_743;
    let _e102: Flector = other_743;
    let _e116: Line = self_889;
    let _e120: Flector = other_743;
    let _e132: Line = self_889;
    let _e136: Flector = other_743;
    let _e139: Flector = other_743;
    let _e142: Flector = other_743;
    let _e145: Flector = other_743;
    let _e158: Line = self_889;
    let _e162: Flector = other_743;
    let _e165: Flector = other_743;
    let _e168: Flector = other_743;
    let _e171: Flector = other_743;
    let _e185: Line = self_889;
    let _e189: Flector = other_743;
    let _e192: Flector = other_743;
    let _e195: Flector = other_743;
    let _e198: Flector = other_743;
    let _e212: Line = self_889;
    let _e216: Flector = other_743;
    let _e219: Flector = other_743;
    let _e222: Flector = other_743;
    let _e225: Flector = other_743;
    let _e239: Line = self_889;
    let _e243: Flector = other_743;
    let _e246: Flector = other_743;
    let _e249: Flector = other_743;
    let _e252: Flector = other_743;
    let _e266: Line = self_889;
    let _e270: Flector = other_743;
    let _e273: Flector = other_743;
    let _e276: Flector = other_743;
    let _e279: Flector = other_743;
    return Flector((((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y)) * vec4<f32>(0.0, 0.0, 0.0, 1.0)) + ((vec4<f32>(_e19.g0_.z) * vec4<f32>(_e23.g0_.z)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e35.g1_.x) * vec4<f32>(_e39.g1_.w, _e42.g0_.z, _e45.g0_.y, _e48.g1_.x)) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e62.g1_.y) * vec4<f32>(_e66.g0_.z, _e69.g1_.w, _e72.g0_.x, _e75.g1_.y)) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e89.g1_.z) * vec4<f32>(_e93.g0_.y, _e96.g0_.x, _e99.g1_.w, _e102.g1_.z)) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e116.g0_.x) * vec4<f32>(_e120.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))), (((((((vec4<f32>(_e132.g0_.y) * vec4<f32>(_e136.g0_.z, _e139.g1_.w, _e142.g0_.x, _e145.g0_.z)) * vec4<f32>(1.0, -(1.0), -(1.0), 0.0)) + ((vec4<f32>(_e158.g0_.z) * vec4<f32>(_e162.g0_.y, _e165.g0_.x, _e168.g1_.w, _e171.g0_.y)) * vec4<f32>(-(1.0), 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e185.g1_.x) * vec4<f32>(_e189.g0_.w, _e192.g1_.z, _e195.g1_.y, _e198.g0_.x)) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e212.g1_.y) * vec4<f32>(_e216.g1_.z, _e219.g0_.w, _e222.g1_.x, _e225.g0_.y)) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e239.g1_.z) * vec4<f32>(_e243.g1_.y, _e246.g1_.x, _e249.g0_.w, _e252.g0_.z)) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e266.g0_.x) * vec4<f32>(_e270.g1_.w, _e273.g0_.z, _e276.g0_.y, _e279.g1_.x)) * vec4<f32>(-(1.0), -(1.0), 1.0, 0.0))));
}

fn line_flector_regressive_product(self_890: Line, other_744: Flector) -> Point {
    var self_891: Line;
    var other_745: Flector;

    self_891 = self_890;
    other_745 = other_744;
    let _e4: Line = self_891;
    let _e8: Flector = other_745;
    let _e19: Line = self_891;
    let _e23: Flector = other_745;
    let _e35: Line = self_891;
    let _e39: Flector = other_745;
    let _e51: Line = self_891;
    let _e55: Flector = other_745;
    let _e67: Line = self_891;
    let _e70: Line = self_891;
    let _e73: Line = self_891;
    let _e76: Line = self_891;
    let _e80: Flector = other_745;
    return Point(((((((vec4<f32>(_e4.g0_.y) * _e8.g1_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e19.g0_.z) * _e23.g1_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e35.g1_.y) * _e39.g1_.zzxz) * vec4<f32>(1.0, 0.0, -(1.0), 0.0))) + ((vec4<f32>(_e51.g1_.z) * _e55.g1_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))) + ((vec4<f32>(_e67.g0_.x, _e70.g1_.x, _e73.g1_.x, _e76.g0_.x) * _e80.g1_.wzyx) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))));
}

fn line_flector_outer_product(self_892: Line, other_746: Flector) -> Plane {
    var self_893: Line;
    var other_747: Flector;

    self_893 = self_892;
    other_747 = other_746;
    let _e4: Line = self_893;
    let _e8: Flector = other_747;
    let _e19: Line = self_893;
    let _e23: Flector = other_747;
    let _e35: Line = self_893;
    let _e39: Flector = other_747;
    let _e51: Line = self_893;
    let _e55: Flector = other_747;
    let _e67: Line = self_893;
    let _e70: Line = self_893;
    let _e73: Line = self_893;
    let _e76: Line = self_893;
    let _e80: Flector = other_747;
    return Plane(((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.zzxz) * vec4<f32>(1.0, 0.0, -(1.0), 0.0)) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))) + ((vec4<f32>(_e35.g1_.y) * _e39.g0_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0)))) + ((vec4<f32>(_e51.g1_.z) * _e55.g0_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e67.g1_.x, _e70.g0_.x, _e73.g0_.x, _e76.g1_.x) * _e80.g0_.wzyx) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))));
}

fn line_flector_inner_product(self_894: Line, other_748: Flector) -> Point {
    var self_895: Line;
    var other_749: Flector;

    self_895 = self_894;
    other_749 = other_748;
    let _e4: Line = self_895;
    let _e8: Flector = other_749;
    let _e19: Line = self_895;
    let _e23: Flector = other_749;
    let _e35: Line = self_895;
    let _e39: Flector = other_749;
    let _e42: Flector = other_749;
    let _e45: Flector = other_749;
    let _e48: Flector = other_749;
    let _e62: Line = self_895;
    let _e66: Flector = other_749;
    let _e69: Flector = other_749;
    let _e72: Flector = other_749;
    let _e75: Flector = other_749;
    let _e89: Line = self_895;
    let _e93: Flector = other_749;
    let _e96: Flector = other_749;
    let _e99: Flector = other_749;
    let _e102: Flector = other_749;
    let _e116: Line = self_895;
    let _e120: Flector = other_749;
    return Point((((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y)) * vec4<f32>(0.0, 0.0, 0.0, 1.0)) + ((vec4<f32>(_e19.g0_.z) * vec4<f32>(_e23.g0_.z)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e35.g1_.x) * vec4<f32>(_e39.g1_.w, _e42.g0_.z, _e45.g0_.y, _e48.g1_.x)) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e62.g1_.y) * vec4<f32>(_e66.g0_.z, _e69.g1_.w, _e72.g0_.x, _e75.g1_.y)) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e89.g1_.z) * vec4<f32>(_e93.g0_.y, _e96.g0_.x, _e99.g1_.w, _e102.g1_.z)) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e116.g0_.x) * vec4<f32>(_e120.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))));
}

fn line_flector_geometric_anti_product(self_896: Line, other_750: Flector) -> Flector {
    var self_897: Line;
    var other_751: Flector;

    self_897 = self_896;
    other_751 = other_750;
    let _e4: Line = self_897;
    let _e8: Flector = other_751;
    let _e11: Flector = other_751;
    let _e14: Flector = other_751;
    let _e17: Flector = other_751;
    let _e30: Line = self_897;
    let _e34: Flector = other_751;
    let _e37: Flector = other_751;
    let _e40: Flector = other_751;
    let _e43: Flector = other_751;
    let _e57: Line = self_897;
    let _e61: Flector = other_751;
    let _e64: Flector = other_751;
    let _e67: Flector = other_751;
    let _e70: Flector = other_751;
    let _e84: Line = self_897;
    let _e88: Flector = other_751;
    let _e91: Flector = other_751;
    let _e94: Flector = other_751;
    let _e97: Flector = other_751;
    let _e110: Line = self_897;
    let _e114: Flector = other_751;
    let _e117: Flector = other_751;
    let _e120: Flector = other_751;
    let _e123: Flector = other_751;
    let _e136: Line = self_897;
    let _e140: Flector = other_751;
    let _e143: Flector = other_751;
    let _e146: Flector = other_751;
    let _e149: Flector = other_751;
    let _e162: Line = self_897;
    let _e166: Flector = other_751;
    let _e169: Flector = other_751;
    let _e172: Flector = other_751;
    let _e175: Flector = other_751;
    let _e188: Line = self_897;
    let _e192: Flector = other_751;
    let _e195: Flector = other_751;
    let _e198: Flector = other_751;
    let _e201: Flector = other_751;
    let _e215: Line = self_897;
    let _e219: Flector = other_751;
    let _e222: Flector = other_751;
    let _e225: Flector = other_751;
    let _e228: Flector = other_751;
    let _e242: Line = self_897;
    let _e246: Flector = other_751;
    let _e259: Line = self_897;
    let _e263: Flector = other_751;
    let _e276: Line = self_897;
    let _e280: Flector = other_751;
    return Flector((((((((vec4<f32>(_e4.g0_.x) * vec4<f32>(_e8.g1_.w, _e11.g0_.z, _e14.g0_.y, _e17.g1_.x)) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0))) + ((vec4<f32>(_e30.g0_.y) * vec4<f32>(_e34.g0_.z, _e37.g1_.w, _e40.g0_.x, _e43.g1_.y)) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e57.g0_.z) * vec4<f32>(_e61.g0_.y, _e64.g0_.x, _e67.g1_.w, _e70.g1_.z)) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e84.g1_.y) * vec4<f32>(_e88.g1_.z, _e91.g0_.w, _e94.g1_.x, _e97.g1_.z)) * vec4<f32>(1.0, 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e110.g1_.z) * vec4<f32>(_e114.g1_.y, _e117.g1_.x, _e120.g0_.w, _e123.g1_.y)) * vec4<f32>(-(1.0), 1.0, 1.0, 0.0))) + ((vec4<f32>(_e136.g1_.x) * vec4<f32>(_e140.g0_.w, _e143.g1_.z, _e146.g1_.y, _e149.g0_.x)) * vec4<f32>(1.0, -(1.0), 1.0, 0.0))), (((((((vec4<f32>(_e162.g0_.x) * vec4<f32>(_e166.g0_.w, _e169.g1_.z, _e172.g1_.y, _e175.g0_.x)) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0))) + ((vec4<f32>(_e188.g0_.y) * vec4<f32>(_e192.g1_.z, _e195.g0_.w, _e198.g1_.x, _e201.g0_.y)) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e215.g0_.z) * vec4<f32>(_e219.g1_.y, _e222.g1_.x, _e225.g0_.w, _e228.g0_.z)) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e242.g1_.y) * vec4<f32>(_e246.g1_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e259.g1_.z) * vec4<f32>(_e263.g1_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e276.g1_.x) * vec4<f32>(_e280.g1_.x)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))));
}

fn line_flector_inner_anti_product(self_898: Line, other_752: Flector) -> Plane {
    var self_899: Line;
    var other_753: Flector;

    self_899 = self_898;
    other_753 = other_752;
    let _e4: Line = self_899;
    let _e8: Flector = other_753;
    let _e11: Flector = other_753;
    let _e14: Flector = other_753;
    let _e17: Flector = other_753;
    let _e30: Line = self_899;
    let _e34: Flector = other_753;
    let _e37: Flector = other_753;
    let _e40: Flector = other_753;
    let _e43: Flector = other_753;
    let _e57: Line = self_899;
    let _e61: Flector = other_753;
    let _e64: Flector = other_753;
    let _e67: Flector = other_753;
    let _e70: Flector = other_753;
    let _e84: Line = self_899;
    let _e88: Flector = other_753;
    let _e101: Line = self_899;
    let _e105: Flector = other_753;
    let _e118: Line = self_899;
    let _e122: Flector = other_753;
    return Plane((((((((vec4<f32>(_e4.g0_.x) * vec4<f32>(_e8.g0_.w, _e11.g1_.z, _e14.g1_.y, _e17.g0_.x)) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0))) + ((vec4<f32>(_e30.g0_.y) * vec4<f32>(_e34.g1_.z, _e37.g0_.w, _e40.g1_.x, _e43.g0_.y)) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e57.g0_.z) * vec4<f32>(_e61.g1_.y, _e64.g1_.x, _e67.g0_.w, _e70.g0_.z)) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e84.g1_.y) * vec4<f32>(_e88.g1_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e101.g1_.z) * vec4<f32>(_e105.g1_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e118.g1_.x) * vec4<f32>(_e122.g1_.x)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))));
}

fn line_flector_left_contraction(self_900: Line, other_754: Flector) -> Point {
    var self_901: Line;
    var other_755: Flector;

    self_901 = self_900;
    other_755 = other_754;
    let _e4: Line = self_901;
    let _e8: Flector = other_755;
    let _e19: Line = self_901;
    let _e23: Flector = other_755;
    let _e35: Line = self_901;
    let _e39: Flector = other_755;
    return Point(((((vec4<f32>(_e4.g1_.y) * _e8.g1_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e19.g1_.z) * _e23.g1_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e35.g1_.x) * _e39.g1_.wxxx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))));
}

fn line_flector_right_contraction(self_902: Line, other_756: Flector) -> Point {
    var self_903: Line;
    var other_757: Flector;

    self_903 = self_902;
    other_757 = other_756;
    let _e4: Line = self_903;
    let _e8: Flector = other_757;
    let _e19: Line = self_903;
    let _e23: Flector = other_757;
    let _e35: Line = self_903;
    let _e39: Flector = other_757;
    let _e51: Line = self_903;
    let _e55: Flector = other_757;
    let _e67: Line = self_903;
    let _e70: Line = self_903;
    let _e73: Line = self_903;
    let _e76: Line = self_903;
    let _e80: Flector = other_757;
    return Point(((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y)) * vec4<f32>(0.0, 0.0, 0.0, 1.0)) + ((vec4<f32>(_e19.g0_.z) * vec4<f32>(_e23.g0_.z)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e35.g1_.y) * _e39.g0_.zzxz) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0))) + ((vec4<f32>(_e51.g1_.z) * _e55.g0_.yxyy) * vec4<f32>(1.0, -(1.0), 0.0, 0.0))) + ((vec4<f32>(_e67.g0_.x, _e70.g1_.x, _e73.g1_.x, _e76.g0_.x) * _e80.g0_.xzyx) * vec4<f32>(0.0, 1.0, -(1.0), 1.0))));
}

fn line_flector_left_anti_contraction(self_904: Line, other_758: Flector) -> Plane {
    var self_905: Line;
    var other_759: Flector;

    self_905 = self_904;
    other_759 = other_758;
    let _e4: Line = self_905;
    let _e8: Flector = other_759;
    let _e19: Line = self_905;
    let _e23: Flector = other_759;
    let _e35: Line = self_905;
    let _e39: Flector = other_759;
    return Plane(((((vec4<f32>(_e4.g0_.y) * _e8.g0_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e35.g0_.x) * _e39.g0_.wxxx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))));
}

fn line_flector_right_anti_contraction(self_906: Line, other_760: Flector) -> Plane {
    var self_907: Line;
    var other_761: Flector;

    self_907 = self_906;
    other_761 = other_760;
    let _e4: Line = self_907;
    let _e8: Flector = other_761;
    let _e19: Line = self_907;
    let _e23: Flector = other_761;
    let _e35: Line = self_907;
    let _e39: Flector = other_761;
    let _e52: Line = self_907;
    let _e56: Flector = other_761;
    let _e69: Line = self_907;
    let _e72: Line = self_907;
    let _e75: Line = self_907;
    let _e78: Line = self_907;
    let _e82: Flector = other_761;
    return Plane(((((((vec4<f32>(_e4.g0_.y) * _e8.g1_.zzxz) * vec4<f32>(1.0, 0.0, -(1.0), 0.0)) + ((vec4<f32>(_e19.g0_.z) * _e23.g1_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))) + ((vec4<f32>(_e35.g1_.y) * vec4<f32>(_e39.g1_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e52.g1_.z) * vec4<f32>(_e56.g1_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e69.g0_.x, _e72.g0_.x, _e75.g0_.x, _e78.g1_.x) * _e82.g1_.xzyx) * vec4<f32>(0.0, -(1.0), 1.0, -(1.0)))));
}

fn line_multi_vector_scalar_product(self_908: Line, other_762: MultiVector) -> Scalar {
    var self_909: Line;
    var other_763: MultiVector;

    self_909 = self_908;
    other_763 = other_762;
    let _e5: Line = self_909;
    let _e8: MultiVector = other_763;
    let _e13: Line = self_909;
    let _e16: MultiVector = other_763;
    let _e21: Line = self_909;
    let _e24: MultiVector = other_763;
    return Scalar((((0.0 - (_e5.g1_.x * _e8.g3_.x)) - (_e13.g1_.y * _e16.g3_.y)) - (_e21.g1_.z * _e24.g3_.z)));
}

fn line_multi_vector_anti_scalar_product(self_910: Line, other_764: MultiVector) -> AntiScalar {
    var self_911: Line;
    var other_765: MultiVector;

    self_911 = self_910;
    other_765 = other_764;
    let _e5: Line = self_911;
    let _e8: MultiVector = other_765;
    let _e13: Line = self_911;
    let _e16: MultiVector = other_765;
    let _e21: Line = self_911;
    let _e24: MultiVector = other_765;
    return AntiScalar((((0.0 - (_e5.g0_.x * _e8.g2_.x)) - (_e13.g0_.y * _e16.g2_.y)) - (_e21.g0_.z * _e24.g2_.z)));
}

fn line_squared_magnitude(self_912: Line) -> Scalar {
    var self_913: Line;

    self_913 = self_912;
    let _e2: Line = self_913;
    let _e3: Line = self_913;
    let _e4: Line = line_reversal(_e3);
    let _e5: Scalar = line_line_scalar_product(_e2, _e4);
    return _e5;
}

fn line_magnitude(self_914: Line) -> Scalar {
    var self_915: Line;

    self_915 = self_914;
    let _e2: Line = self_915;
    let _e3: Scalar = line_squared_magnitude(_e2);
    return Scalar(sqrt(_e3.g0_));
}

fn line_bulk_norm(self_916: Line) -> Scalar {
    var self_917: Line;

    self_917 = self_916;
    let _e2: Line = self_917;
    let _e3: Scalar = line_squared_magnitude(_e2);
    return Scalar(sqrt(_e3.g0_));
}

fn line_squared_anti_magnitude(self_918: Line) -> AntiScalar {
    var self_919: Line;

    self_919 = self_918;
    let _e2: Line = self_919;
    let _e3: Line = self_919;
    let _e4: Line = line_anti_reversal(_e3);
    let _e5: AntiScalar = line_line_anti_scalar_product(_e2, _e4);
    return _e5;
}

fn line_weight_norm(self_920: Line) -> AntiScalar {
    var self_921: Line;

    self_921 = self_920;
    let _e2: Line = self_921;
    let _e3: AntiScalar = line_squared_anti_magnitude(_e2);
    return AntiScalar(sqrt(_e3.g0_));
}

fn line_geometric_norm(self_922: Line) -> HomogeneousMagnitude {
    var self_923: Line;

    self_923 = self_922;
    let _e2: Line = self_923;
    let _e3: Scalar = line_bulk_norm(_e2);
    let _e4: Line = self_923;
    let _e5: AntiScalar = line_weight_norm(_e4);
    return (_e3 + _e5);
}

fn line_scale(self_924: Line, other_766: f32) -> Line {
    var self_925: Line;
    var other_767: f32;

    self_925 = self_924;
    other_767 = other_766;
    let _e4: Line = self_925;
    let _e5: f32 = other_767;
    let _e7: Line = line_scalar_geometric_product(_e4, Scalar(_e5));
    return _e7;
}

fn line_signum(self_926: Line) -> Line {
    var self_927: Line;

    self_927 = self_926;
    let _e2: Line = self_927;
    let _e3: Line = self_927;
    let _e4: Scalar = line_magnitude(_e3);
    let _e9: Line = line_scalar_geometric_product(_e2, Scalar((1.0 / _e4.g0_)));
    return _e9;
}

fn line_inverse(self_928: Line) -> Line {
    var self_929: Line;

    self_929 = self_928;
    let _e2: Line = self_929;
    let _e3: Line = line_reversal(_e2);
    let _e4: Line = self_929;
    let _e5: Scalar = line_squared_magnitude(_e4);
    let _e10: Line = line_scalar_geometric_product(_e3, Scalar((1.0 / _e5.g0_)));
    return _e10;
}

fn line_unitize(self_930: Line) -> Line {
    var self_931: Line;

    self_931 = self_930;
    let _e2: Line = self_931;
    let _e3: Line = self_931;
    let _e4: AntiScalar = line_weight_norm(_e3);
    let _e9: Line = line_scalar_geometric_product(_e2, Scalar((1.0 / _e4.g0_)));
    return _e9;
}

fn plane_zero() -> Plane {
    return Plane(vec4<f32>(0.0));
}

fn plane_one() -> Plane {
    return Plane(vec4<f32>(0.0));
}

fn plane_grade(self_932: Plane) -> i32 {
    return 3;
}

fn plane_anti_grade(self_933: Plane) -> i32 {
    return 1;
}

fn plane_neg(self_934: Plane) -> Plane {
    var self_935: Plane;

    self_935 = self_934;
    let _e2: Plane = self_935;
    return Plane((_e2.g0_ * vec4<f32>(-(1.0))));
}

fn plane_automorphism(self_936: Plane) -> Plane {
    var self_937: Plane;

    self_937 = self_936;
    let _e2: Plane = self_937;
    return Plane((_e2.g0_ * vec4<f32>(-(1.0))));
}

fn plane_reversal(self_938: Plane) -> Plane {
    var self_939: Plane;

    self_939 = self_938;
    let _e2: Plane = self_939;
    return Plane((_e2.g0_ * vec4<f32>(-(1.0))));
}

fn plane_conjugation(self_940: Plane) -> Plane {
    var self_941: Plane;

    self_941 = self_940;
    let _e2: Plane = self_941;
    return Plane(_e2.g0_);
}

fn plane_dual(self_942: Plane) -> Point {
    var self_943: Plane;

    self_943 = self_942;
    let _e2: Plane = self_943;
    return Point((_e2.g0_ * vec4<f32>(-(1.0))));
}

fn plane_anti_reversal(self_944: Plane) -> Plane {
    var self_945: Plane;

    self_945 = self_944;
    let _e2: Plane = self_945;
    return Plane(_e2.g0_);
}

fn plane_right_complement(self_946: Plane) -> Point {
    var self_947: Plane;

    self_947 = self_946;
    let _e2: Plane = self_947;
    return Point((_e2.g0_ * vec4<f32>(-(1.0))));
}

fn plane_left_complement(self_948: Plane) -> Point {
    var self_949: Plane;

    self_949 = self_948;
    let _e2: Plane = self_949;
    return Point(_e2.g0_);
}

fn plane_double_complement(self_950: Plane) -> Plane {
    var self_951: Plane;

    self_951 = self_950;
    let _e2: Plane = self_951;
    return Plane((_e2.g0_ * vec4<f32>(-(1.0))));
}

fn plane_scalar_geometric_product(self_952: Plane, other_768: Scalar) -> Plane {
    var self_953: Plane;
    var other_769: Scalar;

    self_953 = self_952;
    other_769 = other_768;
    let _e4: Plane = self_953;
    let _e6: Scalar = other_769;
    return Plane((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn plane_scalar_outer_product(self_954: Plane, other_770: Scalar) -> Plane {
    var self_955: Plane;
    var other_771: Scalar;

    self_955 = self_954;
    other_771 = other_770;
    let _e4: Plane = self_955;
    let _e6: Scalar = other_771;
    return Plane((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn plane_scalar_inner_product(self_956: Plane, other_772: Scalar) -> Plane {
    var self_957: Plane;
    var other_773: Scalar;

    self_957 = self_956;
    other_773 = other_772;
    let _e4: Plane = self_957;
    let _e6: Scalar = other_773;
    return Plane((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn plane_scalar_right_contraction(self_958: Plane, other_774: Scalar) -> Plane {
    var self_959: Plane;
    var other_775: Scalar;

    self_959 = self_958;
    other_775 = other_774;
    let _e4: Plane = self_959;
    let _e6: Scalar = other_775;
    return Plane((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn plane_anti_scalar_regressive_product(self_960: Plane, other_776: AntiScalar) -> Plane {
    var self_961: Plane;
    var other_777: AntiScalar;

    self_961 = self_960;
    other_777 = other_776;
    let _e4: Plane = self_961;
    let _e6: AntiScalar = other_777;
    return Plane((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn plane_anti_scalar_geometric_anti_product(self_962: Plane, other_778: AntiScalar) -> Plane {
    var self_963: Plane;
    var other_779: AntiScalar;

    self_963 = self_962;
    other_779 = other_778;
    let _e4: Plane = self_963;
    let _e6: AntiScalar = other_779;
    return Plane((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn plane_anti_scalar_inner_anti_product(self_964: Plane, other_780: AntiScalar) -> Plane {
    var self_965: Plane;
    var other_781: AntiScalar;

    self_965 = self_964;
    other_781 = other_780;
    let _e4: Plane = self_965;
    let _e6: AntiScalar = other_781;
    return Plane((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn plane_anti_scalar_right_anti_contraction(self_966: Plane, other_782: AntiScalar) -> Plane {
    var self_967: Plane;
    var other_783: AntiScalar;

    self_967 = self_966;
    other_783 = other_782;
    let _e4: Plane = self_967;
    let _e6: AntiScalar = other_783;
    return Plane((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn plane_homogeneous_magnitude_regressive_product(self_968: Plane, other_784: HomogeneousMagnitude) -> Plane {
    var self_969: Plane;
    var other_785: HomogeneousMagnitude;

    self_969 = self_968;
    other_785 = other_784;
    let _e4: Plane = self_969;
    let _e6: HomogeneousMagnitude = other_785;
    return Plane((_e4.g0_ * vec4<f32>(_e6.g0_.y)));
}

fn plane_homogeneous_magnitude_outer_product(self_970: Plane, other_786: HomogeneousMagnitude) -> Plane {
    var self_971: Plane;
    var other_787: HomogeneousMagnitude;

    self_971 = self_970;
    other_787 = other_786;
    let _e4: Plane = self_971;
    let _e6: HomogeneousMagnitude = other_787;
    return Plane((_e4.g0_ * vec4<f32>(_e6.g0_.x)));
}

fn plane_homogeneous_magnitude_right_contraction(self_972: Plane, other_788: HomogeneousMagnitude) -> Plane {
    var self_973: Plane;
    var other_789: HomogeneousMagnitude;

    self_973 = self_972;
    other_789 = other_788;
    let _e4: Plane = self_973;
    let _e6: HomogeneousMagnitude = other_789;
    return Plane((_e4.g0_ * vec4<f32>(_e6.g0_.x)));
}

fn plane_homogeneous_magnitude_right_anti_contraction(self_974: Plane, other_790: HomogeneousMagnitude) -> Plane {
    var self_975: Plane;
    var other_791: HomogeneousMagnitude;

    self_975 = self_974;
    other_791 = other_790;
    let _e4: Plane = self_975;
    let _e6: HomogeneousMagnitude = other_791;
    return Plane((_e4.g0_ * vec4<f32>(_e6.g0_.y)));
}

fn plane_point_add(self_976: Plane, other_792: Point) -> Flector {
    var self_977: Plane;
    var other_793: Point;

    self_977 = self_976;
    other_793 = other_792;
    let _e4: Point = other_793;
    let _e6: Plane = self_977;
    return Flector(_e4.g0_, _e6.g0_);
}

fn plane_point_sub(self_978: Plane, other_794: Point) -> Flector {
    var self_979: Plane;
    var other_795: Point;

    self_979 = self_978;
    other_795 = other_794;
    let _e6: Point = other_795;
    let _e9: Plane = self_979;
    return Flector((vec4<f32>(0.0) - _e6.g0_), _e9.g0_);
}

fn plane_point_geometric_product(self_980: Plane, other_796: Point) -> Motor {
    var self_981: Plane;
    var other_797: Point;

    self_981 = self_980;
    other_797 = other_796;
    let _e4: Plane = self_981;
    let _e8: Point = other_797;
    let _e20: Plane = self_981;
    let _e24: Point = other_797;
    let _e37: Plane = self_981;
    let _e41: Point = other_797;
    let _e54: Plane = self_981;
    let _e58: Point = other_797;
    let _e73: Plane = self_981;
    let _e77: Point = other_797;
    let _e80: Point = other_797;
    let _e83: Point = other_797;
    return Motor((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.zzxy) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e20.g0_.z) * _e24.g0_.yxyz) * vec4<f32>(1.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e37.g0_.w) * vec4<f32>(_e41.g0_.w)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e54.g0_.x) * _e58.g0_.xzyx) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))), (vec3<f32>(0.0) - (vec3<f32>(_e73.g0_.w) * vec3<f32>(_e77.g0_.x, _e80.g0_.y, _e83.g0_.z))));
}

fn plane_point_regressive_product(self_982: Plane, other_798: Point) -> Scalar {
    var self_983: Plane;
    var other_799: Point;

    self_983 = self_982;
    other_799 = other_798;
    let _e5: Plane = self_983;
    let _e8: Point = other_799;
    let _e13: Plane = self_983;
    let _e16: Point = other_799;
    let _e21: Plane = self_983;
    let _e24: Point = other_799;
    let _e29: Plane = self_983;
    let _e32: Point = other_799;
    return Scalar(((((0.0 - (_e5.g0_.x * _e8.g0_.x)) - (_e13.g0_.y * _e16.g0_.y)) - (_e21.g0_.z * _e24.g0_.z)) - (_e29.g0_.w * _e32.g0_.w)));
}

fn plane_point_outer_product(self_984: Plane, other_800: Point) -> AntiScalar {
    var self_985: Plane;
    var other_801: Point;

    self_985 = self_984;
    other_801 = other_800;
    let _e5: Plane = self_985;
    let _e8: Point = other_801;
    let _e13: Plane = self_985;
    let _e16: Point = other_801;
    let _e21: Plane = self_985;
    let _e24: Point = other_801;
    let _e29: Plane = self_985;
    let _e32: Point = other_801;
    return AntiScalar(((((0.0 - (_e5.g0_.x * _e8.g0_.x)) - (_e13.g0_.y * _e16.g0_.y)) - (_e21.g0_.z * _e24.g0_.z)) - (_e29.g0_.w * _e32.g0_.w)));
}

fn plane_point_inner_product(self_986: Plane, other_802: Point) -> Line {
    var self_987: Plane;
    var other_803: Point;

    self_987 = self_986;
    other_803 = other_802;
    let _e4: Plane = self_987;
    let _e8: Point = other_803;
    let _e11: Point = other_803;
    let _e14: Point = other_803;
    let _e25: Plane = self_987;
    let _e29: Point = other_803;
    let _e32: Point = other_803;
    let _e35: Point = other_803;
    let _e47: Plane = self_987;
    let _e51: Point = other_803;
    let _e54: Point = other_803;
    let _e57: Point = other_803;
    let _e71: Plane = self_987;
    let _e75: Point = other_803;
    let _e78: Point = other_803;
    let _e81: Point = other_803;
    return Line(((((vec3<f32>(_e4.g0_.y) * vec3<f32>(_e8.g0_.z, _e11.g0_.z, _e14.g0_.x)) * vec3<f32>(-(1.0), 0.0, 1.0)) + ((vec3<f32>(_e25.g0_.z) * vec3<f32>(_e29.g0_.y, _e32.g0_.x, _e35.g0_.y)) * vec3<f32>(1.0, -(1.0), 0.0))) + ((vec3<f32>(_e47.g0_.x) * vec3<f32>(_e51.g0_.x, _e54.g0_.z, _e57.g0_.y)) * vec3<f32>(0.0, 1.0, -(1.0)))), (vec3<f32>(0.0) - (vec3<f32>(_e71.g0_.w) * vec3<f32>(_e75.g0_.x, _e78.g0_.y, _e81.g0_.z))));
}

fn plane_point_inner_anti_product(self_988: Plane, other_804: Point) -> Line {
    var self_989: Plane;
    var other_805: Point;

    self_989 = self_988;
    other_805 = other_804;
    let _e4: Plane = self_989;
    let _e7: Plane = self_989;
    let _e10: Plane = self_989;
    let _e14: Point = other_805;
    let _e23: Plane = self_989;
    let _e27: Point = other_805;
    let _e30: Point = other_805;
    let _e33: Point = other_805;
    let _e44: Plane = self_989;
    let _e48: Point = other_805;
    let _e51: Point = other_805;
    let _e54: Point = other_805;
    let _e66: Plane = self_989;
    let _e70: Point = other_805;
    let _e73: Point = other_805;
    let _e76: Point = other_805;
    return Line(((vec3<f32>(_e4.g0_.x, _e7.g0_.y, _e10.g0_.z) * vec3<f32>(_e14.g0_.w)) * vec3<f32>(-(1.0))), ((((vec3<f32>(_e23.g0_.y) * vec3<f32>(_e27.g0_.z, _e30.g0_.z, _e33.g0_.x)) * vec3<f32>(1.0, 0.0, -(1.0))) + ((vec3<f32>(_e44.g0_.z) * vec3<f32>(_e48.g0_.y, _e51.g0_.x, _e54.g0_.y)) * vec3<f32>(-(1.0), 1.0, 0.0))) + ((vec3<f32>(_e66.g0_.x) * vec3<f32>(_e70.g0_.x, _e73.g0_.z, _e76.g0_.y)) * vec3<f32>(0.0, -(1.0), 1.0))));
}

fn plane_point_right_contraction(self_990: Plane, other_806: Point) -> Line {
    var self_991: Plane;
    var other_807: Point;

    self_991 = self_990;
    other_807 = other_806;
    let _e4: Plane = self_991;
    let _e8: Point = other_807;
    let _e11: Point = other_807;
    let _e14: Point = other_807;
    let _e25: Plane = self_991;
    let _e29: Point = other_807;
    let _e32: Point = other_807;
    let _e35: Point = other_807;
    let _e47: Plane = self_991;
    let _e51: Point = other_807;
    let _e54: Point = other_807;
    let _e57: Point = other_807;
    let _e71: Plane = self_991;
    let _e75: Point = other_807;
    let _e78: Point = other_807;
    let _e81: Point = other_807;
    return Line(((((vec3<f32>(_e4.g0_.y) * vec3<f32>(_e8.g0_.z, _e11.g0_.z, _e14.g0_.x)) * vec3<f32>(-(1.0), 0.0, 1.0)) + ((vec3<f32>(_e25.g0_.z) * vec3<f32>(_e29.g0_.y, _e32.g0_.x, _e35.g0_.y)) * vec3<f32>(1.0, -(1.0), 0.0))) + ((vec3<f32>(_e47.g0_.x) * vec3<f32>(_e51.g0_.x, _e54.g0_.z, _e57.g0_.y)) * vec3<f32>(0.0, 1.0, -(1.0)))), (vec3<f32>(0.0) - (vec3<f32>(_e71.g0_.w) * vec3<f32>(_e75.g0_.x, _e78.g0_.y, _e81.g0_.z))));
}

fn plane_point_left_anti_contraction(self_992: Plane, other_808: Point) -> Line {
    var self_993: Plane;
    var other_809: Point;

    self_993 = self_992;
    other_809 = other_808;
    let _e4: Plane = self_993;
    let _e7: Plane = self_993;
    let _e10: Plane = self_993;
    let _e14: Point = other_809;
    let _e23: Plane = self_993;
    let _e27: Point = other_809;
    let _e30: Point = other_809;
    let _e33: Point = other_809;
    let _e44: Plane = self_993;
    let _e48: Point = other_809;
    let _e51: Point = other_809;
    let _e54: Point = other_809;
    let _e66: Plane = self_993;
    let _e70: Point = other_809;
    let _e73: Point = other_809;
    let _e76: Point = other_809;
    return Line(((vec3<f32>(_e4.g0_.x, _e7.g0_.y, _e10.g0_.z) * vec3<f32>(_e14.g0_.w)) * vec3<f32>(-(1.0))), ((((vec3<f32>(_e23.g0_.y) * vec3<f32>(_e27.g0_.z, _e30.g0_.z, _e33.g0_.x)) * vec3<f32>(1.0, 0.0, -(1.0))) + ((vec3<f32>(_e44.g0_.z) * vec3<f32>(_e48.g0_.y, _e51.g0_.x, _e54.g0_.y)) * vec3<f32>(-(1.0), 1.0, 0.0))) + ((vec3<f32>(_e66.g0_.x) * vec3<f32>(_e70.g0_.x, _e73.g0_.z, _e76.g0_.y)) * vec3<f32>(0.0, -(1.0), 1.0))));
}

fn plane_line_regressive_product(self_994: Plane, other_810: Line) -> Point {
    var self_995: Plane;
    var other_811: Line;

    self_995 = self_994;
    other_811 = other_810;
    let _e4: Plane = self_995;
    let _e8: Line = other_811;
    let _e11: Line = other_811;
    let _e14: Line = other_811;
    let _e17: Line = other_811;
    let _e30: Plane = self_995;
    let _e34: Line = other_811;
    let _e37: Line = other_811;
    let _e40: Line = other_811;
    let _e43: Line = other_811;
    let _e57: Plane = self_995;
    let _e61: Line = other_811;
    let _e64: Line = other_811;
    let _e67: Line = other_811;
    let _e70: Line = other_811;
    let _e82: Plane = self_995;
    let _e86: Line = other_811;
    let _e89: Line = other_811;
    let _e92: Line = other_811;
    let _e95: Line = other_811;
    return Point((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g1_.z, _e11.g1_.z, _e14.g1_.x, _e17.g0_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e30.g0_.z) * vec4<f32>(_e34.g1_.y, _e37.g1_.x, _e40.g1_.y, _e43.g0_.z)) * vec4<f32>(1.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e57.g0_.w) * vec4<f32>(_e61.g0_.x, _e64.g0_.y, _e67.g0_.z, _e70.g0_.x)) * vec4<f32>(1.0, 1.0, 1.0, 0.0))) + ((vec4<f32>(_e82.g0_.x) * vec4<f32>(_e86.g1_.x, _e89.g1_.z, _e92.g1_.y, _e95.g0_.x)) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))));
}

fn plane_line_inner_product(self_996: Plane, other_812: Line) -> Point {
    var self_997: Plane;
    var other_813: Line;

    self_997 = self_996;
    other_813 = other_812;
    let _e4: Plane = self_997;
    let _e8: Line = other_813;
    let _e20: Plane = self_997;
    let _e24: Line = other_813;
    let _e37: Plane = self_997;
    let _e40: Line = other_813;
    let _e43: Line = other_813;
    let _e46: Line = other_813;
    let _e49: Line = other_813;
    return Point(((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g1_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0))) + ((vec4<f32>(_e20.g0_.z) * vec4<f32>(_e24.g1_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((_e37.g0_.wwwx * vec4<f32>(_e40.g1_.x, _e43.g1_.y, _e46.g1_.z, _e49.g1_.x)) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))));
}

fn plane_line_geometric_anti_product(self_998: Plane, other_814: Line) -> Flector {
    var self_999: Plane;
    var other_815: Line;

    self_999 = self_998;
    other_815 = other_814;
    let _e4: Plane = self_999;
    let _e8: Line = other_815;
    let _e11: Line = other_815;
    let _e14: Line = other_815;
    let _e17: Line = other_815;
    let _e30: Plane = self_999;
    let _e34: Line = other_815;
    let _e37: Line = other_815;
    let _e40: Line = other_815;
    let _e43: Line = other_815;
    let _e57: Plane = self_999;
    let _e61: Line = other_815;
    let _e64: Line = other_815;
    let _e67: Line = other_815;
    let _e70: Line = other_815;
    let _e82: Plane = self_999;
    let _e86: Line = other_815;
    let _e89: Line = other_815;
    let _e92: Line = other_815;
    let _e95: Line = other_815;
    let _e109: Plane = self_999;
    let _e113: Line = other_815;
    let _e116: Line = other_815;
    let _e119: Line = other_815;
    let _e122: Line = other_815;
    let _e134: Plane = self_999;
    let _e138: Line = other_815;
    let _e141: Line = other_815;
    let _e144: Line = other_815;
    let _e147: Line = other_815;
    let _e160: Plane = self_999;
    let _e164: Line = other_815;
    let _e167: Line = other_815;
    let _e170: Line = other_815;
    let _e173: Line = other_815;
    return Flector((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g1_.z, _e11.g1_.z, _e14.g1_.x, _e17.g0_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e30.g0_.z) * vec4<f32>(_e34.g1_.y, _e37.g1_.x, _e40.g1_.y, _e43.g0_.z)) * vec4<f32>(1.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e57.g0_.w) * vec4<f32>(_e61.g0_.x, _e64.g0_.y, _e67.g0_.z, _e70.g0_.x)) * vec4<f32>(1.0, 1.0, 1.0, 0.0))) + ((vec4<f32>(_e82.g0_.x) * vec4<f32>(_e86.g1_.x, _e89.g1_.z, _e92.g1_.y, _e95.g0_.x)) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))), ((((vec4<f32>(_e109.g0_.y) * vec4<f32>(_e113.g0_.z, _e116.g0_.z, _e119.g0_.x, _e122.g1_.y)) * vec4<f32>(1.0, 0.0, -(1.0), 1.0)) + ((vec4<f32>(_e134.g0_.z) * vec4<f32>(_e138.g0_.y, _e141.g0_.x, _e144.g0_.y, _e147.g1_.z)) * vec4<f32>(-(1.0), 1.0, 0.0, 1.0))) + ((vec4<f32>(_e160.g0_.x) * vec4<f32>(_e164.g0_.x, _e167.g0_.z, _e170.g0_.y, _e173.g1_.x)) * vec4<f32>(0.0, -(1.0), 1.0, 1.0))));
}

fn plane_line_inner_anti_product(self_1000: Plane, other_816: Line) -> Plane {
    var self_1001: Plane;
    var other_817: Line;

    self_1001 = self_1000;
    other_817 = other_816;
    let _e4: Plane = self_1001;
    let _e8: Line = other_817;
    let _e11: Line = other_817;
    let _e14: Line = other_817;
    let _e17: Line = other_817;
    let _e29: Plane = self_1001;
    let _e33: Line = other_817;
    let _e36: Line = other_817;
    let _e39: Line = other_817;
    let _e42: Line = other_817;
    let _e55: Plane = self_1001;
    let _e59: Line = other_817;
    let _e62: Line = other_817;
    let _e65: Line = other_817;
    let _e68: Line = other_817;
    return Plane(((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.z, _e11.g0_.z, _e14.g0_.x, _e17.g1_.y)) * vec4<f32>(1.0, 0.0, -(1.0), 1.0)) + ((vec4<f32>(_e29.g0_.z) * vec4<f32>(_e33.g0_.y, _e36.g0_.x, _e39.g0_.y, _e42.g1_.z)) * vec4<f32>(-(1.0), 1.0, 0.0, 1.0))) + ((vec4<f32>(_e55.g0_.x) * vec4<f32>(_e59.g0_.x, _e62.g0_.z, _e65.g0_.y, _e68.g1_.x)) * vec4<f32>(0.0, -(1.0), 1.0, 1.0))));
}

fn plane_line_right_contraction(self_1002: Plane, other_818: Line) -> Point {
    var self_1003: Plane;
    var other_819: Line;

    self_1003 = self_1002;
    other_819 = other_818;
    let _e4: Plane = self_1003;
    let _e8: Line = other_819;
    let _e20: Plane = self_1003;
    let _e24: Line = other_819;
    let _e37: Plane = self_1003;
    let _e40: Line = other_819;
    let _e43: Line = other_819;
    let _e46: Line = other_819;
    let _e49: Line = other_819;
    return Point(((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g1_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0))) + ((vec4<f32>(_e20.g0_.z) * vec4<f32>(_e24.g1_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((_e37.g0_.wwwx * vec4<f32>(_e40.g1_.x, _e43.g1_.y, _e46.g1_.z, _e49.g1_.x)) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))));
}

fn plane_line_left_anti_contraction(self_1004: Plane, other_820: Line) -> Plane {
    var self_1005: Plane;
    var other_821: Line;

    self_1005 = self_1004;
    other_821 = other_820;
    let _e4: Plane = self_1005;
    let _e8: Line = other_821;
    let _e11: Line = other_821;
    let _e14: Line = other_821;
    let _e17: Line = other_821;
    let _e29: Plane = self_1005;
    let _e33: Line = other_821;
    let _e36: Line = other_821;
    let _e39: Line = other_821;
    let _e42: Line = other_821;
    let _e55: Plane = self_1005;
    let _e59: Line = other_821;
    let _e62: Line = other_821;
    let _e65: Line = other_821;
    let _e68: Line = other_821;
    return Plane(((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.z, _e11.g0_.z, _e14.g0_.x, _e17.g1_.y)) * vec4<f32>(1.0, 0.0, -(1.0), 1.0)) + ((vec4<f32>(_e29.g0_.z) * vec4<f32>(_e33.g0_.y, _e36.g0_.x, _e39.g0_.y, _e42.g1_.z)) * vec4<f32>(-(1.0), 1.0, 0.0, 1.0))) + ((vec4<f32>(_e55.g0_.x) * vec4<f32>(_e59.g0_.x, _e62.g0_.z, _e65.g0_.y, _e68.g1_.x)) * vec4<f32>(0.0, -(1.0), 1.0, 1.0))));
}

fn plane_plane_add(self_1006: Plane, other_822: Plane) -> Plane {
    var self_1007: Plane;
    var other_823: Plane;

    self_1007 = self_1006;
    other_823 = other_822;
    let _e4: Plane = self_1007;
    let _e6: Plane = other_823;
    return Plane((_e4.g0_ + _e6.g0_));
}

fn plane_plane_sub(self_1008: Plane, other_824: Plane) -> Plane {
    var self_1009: Plane;
    var other_825: Plane;

    self_1009 = self_1008;
    other_825 = other_824;
    let _e4: Plane = self_1009;
    let _e6: Plane = other_825;
    return Plane((_e4.g0_ - _e6.g0_));
}

fn plane_plane_mul(self_1010: Plane, other_826: Plane) -> Plane {
    var self_1011: Plane;
    var other_827: Plane;

    self_1011 = self_1010;
    other_827 = other_826;
    let _e4: Plane = self_1011;
    let _e6: Plane = other_827;
    return Plane((_e4.g0_ * _e6.g0_));
}

fn plane_plane_div(self_1012: Plane, other_828: Plane) -> Plane {
    var self_1013: Plane;
    var other_829: Plane;

    self_1013 = self_1012;
    other_829 = other_828;
    let _e4: Plane = self_1013;
    let _e7: Plane = self_1013;
    let _e10: Plane = self_1013;
    let _e13: Plane = self_1013;
    let _e23: Plane = other_829;
    let _e26: Plane = other_829;
    let _e29: Plane = other_829;
    let _e32: Plane = other_829;
    return Plane((((vec4<f32>(_e4.g0_.x, _e7.g0_.y, _e10.g0_.z, _e13.g0_.w) * vec4<f32>(1.0, 1.0, 1.0, 1.0)) / vec4<f32>(_e23.g0_.x, _e26.g0_.y, _e29.g0_.z, _e32.g0_.w)) * vec4<f32>(1.0, 1.0, 1.0, 1.0)));
}

fn plane_plane_regressive_product(self_1014: Plane, other_830: Plane) -> Line {
    var self_1015: Plane;
    var other_831: Plane;

    self_1015 = self_1014;
    other_831 = other_830;
    let _e4: Plane = self_1015;
    let _e8: Plane = other_831;
    let _e11: Plane = other_831;
    let _e14: Plane = other_831;
    let _e25: Plane = self_1015;
    let _e29: Plane = other_831;
    let _e32: Plane = other_831;
    let _e35: Plane = other_831;
    let _e47: Plane = self_1015;
    let _e51: Plane = other_831;
    let _e54: Plane = other_831;
    let _e57: Plane = other_831;
    let _e71: Plane = self_1015;
    let _e75: Plane = other_831;
    let _e78: Plane = other_831;
    let _e81: Plane = other_831;
    let _e87: Plane = self_1015;
    let _e90: Plane = self_1015;
    let _e93: Plane = self_1015;
    let _e97: Plane = other_831;
    return Line(((((vec3<f32>(_e4.g0_.y) * vec3<f32>(_e8.g0_.z, _e11.g0_.z, _e14.g0_.x)) * vec3<f32>(-(1.0), 0.0, 1.0)) + ((vec3<f32>(_e25.g0_.z) * vec3<f32>(_e29.g0_.y, _e32.g0_.x, _e35.g0_.y)) * vec3<f32>(1.0, -(1.0), 0.0))) + ((vec3<f32>(_e47.g0_.x) * vec3<f32>(_e51.g0_.x, _e54.g0_.z, _e57.g0_.y)) * vec3<f32>(0.0, 1.0, -(1.0)))), ((vec3<f32>(0.0) - (vec3<f32>(_e71.g0_.w) * vec3<f32>(_e75.g0_.x, _e78.g0_.y, _e81.g0_.z))) + (vec3<f32>(_e87.g0_.x, _e90.g0_.y, _e93.g0_.z) * vec3<f32>(_e97.g0_.w))));
}

fn plane_plane_inner_product(self_1016: Plane, other_832: Plane) -> Scalar {
    var self_1017: Plane;
    var other_833: Plane;

    self_1017 = self_1016;
    other_833 = other_832;
    let _e5: Plane = self_1017;
    let _e8: Plane = other_833;
    return Scalar((0.0 - (_e5.g0_.w * _e8.g0_.w)));
}

fn plane_plane_geometric_anti_product(self_1018: Plane, other_834: Plane) -> Motor {
    var self_1019: Plane;
    var other_835: Plane;

    self_1019 = self_1018;
    other_835 = other_834;
    let _e4: Plane = self_1019;
    let _e8: Plane = other_835;
    let _e19: Plane = self_1019;
    let _e23: Plane = other_835;
    let _e35: Plane = self_1019;
    let _e39: Plane = other_835;
    let _e53: Plane = self_1019;
    let _e57: Plane = other_835;
    let _e60: Plane = other_835;
    let _e63: Plane = other_835;
    let _e69: Plane = self_1019;
    let _e72: Plane = self_1019;
    let _e75: Plane = self_1019;
    let _e79: Plane = other_835;
    return Motor(((((vec4<f32>(_e4.g0_.y) * _e8.g0_.zzxy) * vec4<f32>(-(1.0), 0.0, 1.0, 1.0)) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.yxyz) * vec4<f32>(1.0, -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e35.g0_.x) * _e39.g0_.xzyx) * vec4<f32>(0.0, 1.0, -(1.0), 1.0))), ((vec3<f32>(0.0) - (vec3<f32>(_e53.g0_.w) * vec3<f32>(_e57.g0_.x, _e60.g0_.y, _e63.g0_.z))) + (vec3<f32>(_e69.g0_.x, _e72.g0_.y, _e75.g0_.z) * vec3<f32>(_e79.g0_.w))));
}

fn plane_plane_inner_anti_product(self_1020: Plane, other_836: Plane) -> AntiScalar {
    var self_1021: Plane;
    var other_837: Plane;

    self_1021 = self_1020;
    other_837 = other_836;
    let _e4: Plane = self_1021;
    let _e7: Plane = other_837;
    let _e11: Plane = self_1021;
    let _e14: Plane = other_837;
    let _e19: Plane = self_1021;
    let _e22: Plane = other_837;
    return AntiScalar((((_e4.g0_.x * _e7.g0_.x) + (_e11.g0_.y * _e14.g0_.y)) + (_e19.g0_.z * _e22.g0_.z)));
}

fn plane_plane_left_contraction(self_1022: Plane, other_838: Plane) -> Scalar {
    var self_1023: Plane;
    var other_839: Plane;

    self_1023 = self_1022;
    other_839 = other_838;
    let _e5: Plane = self_1023;
    let _e8: Plane = other_839;
    return Scalar((0.0 - (_e5.g0_.w * _e8.g0_.w)));
}

fn plane_plane_right_contraction(self_1024: Plane, other_840: Plane) -> Scalar {
    var self_1025: Plane;
    var other_841: Plane;

    self_1025 = self_1024;
    other_841 = other_840;
    let _e5: Plane = self_1025;
    let _e8: Plane = other_841;
    return Scalar((0.0 - (_e5.g0_.w * _e8.g0_.w)));
}

fn plane_plane_left_anti_contraction(self_1026: Plane, other_842: Plane) -> AntiScalar {
    var self_1027: Plane;
    var other_843: Plane;

    self_1027 = self_1026;
    other_843 = other_842;
    let _e4: Plane = self_1027;
    let _e7: Plane = other_843;
    let _e11: Plane = self_1027;
    let _e14: Plane = other_843;
    let _e19: Plane = self_1027;
    let _e22: Plane = other_843;
    return AntiScalar((((_e4.g0_.x * _e7.g0_.x) + (_e11.g0_.y * _e14.g0_.y)) + (_e19.g0_.z * _e22.g0_.z)));
}

fn plane_plane_right_anti_contraction(self_1028: Plane, other_844: Plane) -> AntiScalar {
    var self_1029: Plane;
    var other_845: Plane;

    self_1029 = self_1028;
    other_845 = other_844;
    let _e4: Plane = self_1029;
    let _e7: Plane = other_845;
    let _e11: Plane = self_1029;
    let _e14: Plane = other_845;
    let _e19: Plane = self_1029;
    let _e22: Plane = other_845;
    return AntiScalar((((_e4.g0_.x * _e7.g0_.x) + (_e11.g0_.y * _e14.g0_.y)) + (_e19.g0_.z * _e22.g0_.z)));
}

fn plane_plane_scalar_product(self_1030: Plane, other_846: Plane) -> Scalar {
    var self_1031: Plane;
    var other_847: Plane;

    self_1031 = self_1030;
    other_847 = other_846;
    let _e5: Plane = self_1031;
    let _e8: Plane = other_847;
    return Scalar((0.0 - (_e5.g0_.w * _e8.g0_.w)));
}

fn plane_plane_anti_scalar_product(self_1032: Plane, other_848: Plane) -> AntiScalar {
    var self_1033: Plane;
    var other_849: Plane;

    self_1033 = self_1032;
    other_849 = other_848;
    let _e4: Plane = self_1033;
    let _e7: Plane = other_849;
    let _e11: Plane = self_1033;
    let _e14: Plane = other_849;
    let _e19: Plane = self_1033;
    let _e22: Plane = other_849;
    return AntiScalar((((_e4.g0_.x * _e7.g0_.x) + (_e11.g0_.y * _e14.g0_.y)) + (_e19.g0_.z * _e22.g0_.z)));
}

fn plane_motor_regressive_product(self_1034: Plane, other_850: Motor) -> Flector {
    var self_1035: Plane;
    var other_851: Motor;

    self_1035 = self_1034;
    other_851 = other_850;
    let _e4: Plane = self_1035;
    let _e8: Motor = other_851;
    let _e11: Motor = other_851;
    let _e14: Motor = other_851;
    let _e17: Motor = other_851;
    let _e30: Plane = self_1035;
    let _e34: Motor = other_851;
    let _e37: Motor = other_851;
    let _e40: Motor = other_851;
    let _e43: Motor = other_851;
    let _e57: Plane = self_1035;
    let _e61: Motor = other_851;
    let _e72: Plane = self_1035;
    let _e76: Motor = other_851;
    let _e79: Motor = other_851;
    let _e82: Motor = other_851;
    let _e85: Motor = other_851;
    let _e99: Plane = self_1035;
    let _e101: Motor = other_851;
    return Flector((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g1_.z, _e11.g1_.z, _e14.g1_.x, _e17.g0_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e30.g0_.z) * vec4<f32>(_e34.g1_.y, _e37.g1_.x, _e40.g1_.y, _e43.g0_.z)) * vec4<f32>(1.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e57.g0_.w) * _e61.g0_.xyzx) * vec4<f32>(1.0, 1.0, 1.0, 0.0))) + ((vec4<f32>(_e72.g0_.x) * vec4<f32>(_e76.g1_.x, _e79.g1_.z, _e82.g1_.y, _e85.g0_.x)) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))), (_e99.g0_ * vec4<f32>(_e101.g0_.w)));
}

fn plane_motor_inner_product(self_1036: Plane, other_852: Motor) -> Point {
    var self_1037: Plane;
    var other_853: Motor;

    self_1037 = self_1036;
    other_853 = other_852;
    let _e4: Plane = self_1037;
    let _e8: Motor = other_853;
    let _e20: Plane = self_1037;
    let _e24: Motor = other_853;
    let _e37: Plane = self_1037;
    let _e41: Motor = other_853;
    let _e44: Motor = other_853;
    let _e47: Motor = other_853;
    let _e50: Motor = other_853;
    let _e56: Plane = self_1037;
    let _e60: Motor = other_853;
    return Point((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g1_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0))) + ((vec4<f32>(_e20.g0_.z) * vec4<f32>(_e24.g1_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + (vec4<f32>(_e37.g0_.w) * vec4<f32>(_e41.g1_.x, _e44.g1_.y, _e47.g1_.z, _e50.g0_.w))) + ((vec4<f32>(_e56.g0_.x) * vec4<f32>(_e60.g1_.x)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))));
}

fn plane_motor_geometric_anti_product(self_1038: Plane, other_854: Motor) -> Flector {
    var self_1039: Plane;
    var other_855: Motor;

    self_1039 = self_1038;
    other_855 = other_854;
    let _e4: Plane = self_1039;
    let _e8: Motor = other_855;
    let _e11: Motor = other_855;
    let _e14: Motor = other_855;
    let _e17: Motor = other_855;
    let _e30: Plane = self_1039;
    let _e34: Motor = other_855;
    let _e37: Motor = other_855;
    let _e40: Motor = other_855;
    let _e43: Motor = other_855;
    let _e57: Plane = self_1039;
    let _e61: Motor = other_855;
    let _e72: Plane = self_1039;
    let _e76: Motor = other_855;
    let _e79: Motor = other_855;
    let _e82: Motor = other_855;
    let _e85: Motor = other_855;
    let _e99: Plane = self_1039;
    let _e103: Motor = other_855;
    let _e106: Motor = other_855;
    let _e109: Motor = other_855;
    let _e112: Motor = other_855;
    let _e124: Plane = self_1039;
    let _e128: Motor = other_855;
    let _e131: Motor = other_855;
    let _e134: Motor = other_855;
    let _e137: Motor = other_855;
    let _e150: Plane = self_1039;
    let _e154: Motor = other_855;
    let _e157: Motor = other_855;
    let _e160: Motor = other_855;
    let _e163: Motor = other_855;
    let _e176: Plane = self_1039;
    let _e179: Motor = other_855;
    return Flector((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g1_.z, _e11.g1_.z, _e14.g1_.x, _e17.g0_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e30.g0_.z) * vec4<f32>(_e34.g1_.y, _e37.g1_.x, _e40.g1_.y, _e43.g0_.z)) * vec4<f32>(1.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e57.g0_.w) * _e61.g0_.xyzx) * vec4<f32>(1.0, 1.0, 1.0, 0.0))) + ((vec4<f32>(_e72.g0_.x) * vec4<f32>(_e76.g1_.x, _e79.g1_.z, _e82.g1_.y, _e85.g0_.x)) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))), (((((vec4<f32>(_e99.g0_.x) * vec4<f32>(_e103.g0_.w, _e106.g0_.z, _e109.g0_.y, _e112.g1_.x)) * vec4<f32>(1.0, -(1.0), 1.0, 1.0)) + ((vec4<f32>(_e124.g0_.y) * vec4<f32>(_e128.g0_.z, _e131.g0_.w, _e134.g0_.x, _e137.g1_.y)) * vec4<f32>(1.0, 1.0, -(1.0), 1.0))) + ((vec4<f32>(_e150.g0_.z) * vec4<f32>(_e154.g0_.y, _e157.g0_.x, _e160.g0_.w, _e163.g1_.z)) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0))) + ((_e176.g0_.xxxw * _e179.g0_.xxxw) * vec4<f32>(0.0, 0.0, 0.0, 1.0))));
}

fn plane_motor_inner_anti_product(self_1040: Plane, other_856: Motor) -> Plane {
    var self_1041: Plane;
    var other_857: Motor;

    self_1041 = self_1040;
    other_857 = other_856;
    let _e4: Plane = self_1041;
    let _e8: Motor = other_857;
    let _e11: Motor = other_857;
    let _e14: Motor = other_857;
    let _e17: Motor = other_857;
    let _e29: Plane = self_1041;
    let _e33: Motor = other_857;
    let _e36: Motor = other_857;
    let _e39: Motor = other_857;
    let _e42: Motor = other_857;
    let _e55: Plane = self_1041;
    let _e59: Motor = other_857;
    let _e62: Motor = other_857;
    let _e65: Motor = other_857;
    let _e68: Motor = other_857;
    let _e81: Plane = self_1041;
    let _e84: Motor = other_857;
    return Plane((((((vec4<f32>(_e4.g0_.x) * vec4<f32>(_e8.g0_.w, _e11.g0_.z, _e14.g0_.y, _e17.g1_.x)) * vec4<f32>(1.0, -(1.0), 1.0, 1.0)) + ((vec4<f32>(_e29.g0_.y) * vec4<f32>(_e33.g0_.z, _e36.g0_.w, _e39.g0_.x, _e42.g1_.y)) * vec4<f32>(1.0, 1.0, -(1.0), 1.0))) + ((vec4<f32>(_e55.g0_.z) * vec4<f32>(_e59.g0_.y, _e62.g0_.x, _e65.g0_.w, _e68.g1_.z)) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0))) + ((_e81.g0_.xxxw * _e84.g0_.xxxw) * vec4<f32>(0.0, 0.0, 0.0, 1.0))));
}

fn plane_motor_right_contraction(self_1042: Plane, other_858: Motor) -> Point {
    var self_1043: Plane;
    var other_859: Motor;

    self_1043 = self_1042;
    other_859 = other_858;
    let _e4: Plane = self_1043;
    let _e8: Motor = other_859;
    let _e20: Plane = self_1043;
    let _e24: Motor = other_859;
    let _e37: Plane = self_1043;
    let _e40: Motor = other_859;
    let _e43: Motor = other_859;
    let _e46: Motor = other_859;
    let _e49: Motor = other_859;
    return Point(((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g1_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0))) + ((vec4<f32>(_e20.g0_.z) * vec4<f32>(_e24.g1_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((_e37.g0_.wwwx * vec4<f32>(_e40.g1_.x, _e43.g1_.y, _e46.g1_.z, _e49.g1_.x)) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))));
}

fn plane_motor_left_anti_contraction(self_1044: Plane, other_860: Motor) -> Plane {
    var self_1045: Plane;
    var other_861: Motor;

    self_1045 = self_1044;
    other_861 = other_860;
    let _e4: Plane = self_1045;
    let _e8: Motor = other_861;
    let _e11: Motor = other_861;
    let _e14: Motor = other_861;
    let _e17: Motor = other_861;
    let _e29: Plane = self_1045;
    let _e33: Motor = other_861;
    let _e36: Motor = other_861;
    let _e39: Motor = other_861;
    let _e42: Motor = other_861;
    let _e55: Plane = self_1045;
    let _e59: Motor = other_861;
    let _e62: Motor = other_861;
    let _e65: Motor = other_861;
    let _e68: Motor = other_861;
    return Plane(((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.z, _e11.g0_.z, _e14.g0_.x, _e17.g1_.y)) * vec4<f32>(1.0, 0.0, -(1.0), 1.0)) + ((vec4<f32>(_e29.g0_.z) * vec4<f32>(_e33.g0_.y, _e36.g0_.x, _e39.g0_.y, _e42.g1_.z)) * vec4<f32>(-(1.0), 1.0, 0.0, 1.0))) + ((vec4<f32>(_e55.g0_.x) * vec4<f32>(_e59.g0_.x, _e62.g0_.z, _e65.g0_.y, _e68.g1_.x)) * vec4<f32>(0.0, -(1.0), 1.0, 1.0))));
}

fn plane_motor_right_anti_contraction(self_1046: Plane, other_862: Motor) -> Plane {
    var self_1047: Plane;
    var other_863: Motor;

    self_1047 = self_1046;
    other_863 = other_862;
    let _e4: Plane = self_1047;
    let _e6: Motor = other_863;
    return Plane((_e4.g0_ * vec4<f32>(_e6.g0_.w)));
}

fn plane_rotor_regressive_product(self_1048: Plane, other_864: Rotor) -> Flector {
    var self_1049: Plane;
    var other_865: Rotor;

    self_1049 = self_1048;
    other_865 = other_864;
    let _e4: Plane = self_1049;
    let _e8: Rotor = other_865;
    let _e20: Plane = self_1049;
    let _e24: Rotor = other_865;
    let _e37: Plane = self_1049;
    let _e40: Rotor = other_865;
    let _e52: Plane = self_1049;
    let _e54: Rotor = other_865;
    return Flector(((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0))) + ((vec4<f32>(_e20.g0_.z) * vec4<f32>(_e24.g0_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((_e37.g0_.wwwx * _e40.g0_.xyzx) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))), (_e52.g0_ * vec4<f32>(_e54.g0_.w)));
}

fn plane_rotor_geometric_anti_product(self_1050: Plane, other_866: Rotor) -> Flector {
    var self_1051: Plane;
    var other_867: Rotor;

    self_1051 = self_1050;
    other_867 = other_866;
    let _e4: Plane = self_1051;
    let _e8: Rotor = other_867;
    let _e20: Plane = self_1051;
    let _e24: Rotor = other_867;
    let _e37: Plane = self_1051;
    let _e40: Rotor = other_867;
    let _e52: Plane = self_1051;
    let _e56: Rotor = other_867;
    let _e67: Plane = self_1051;
    let _e71: Rotor = other_867;
    let _e83: Plane = self_1051;
    let _e86: Rotor = other_867;
    return Flector(((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0))) + ((vec4<f32>(_e20.g0_.z) * vec4<f32>(_e24.g0_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((_e37.g0_.wwwx * _e40.g0_.xyzx) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))), ((((vec4<f32>(_e52.g0_.y) * _e56.g0_.zwxz) * vec4<f32>(1.0, 1.0, -(1.0), 0.0)) + ((vec4<f32>(_e67.g0_.z) * _e71.g0_.yxwy) * vec4<f32>(-(1.0), 1.0, 1.0, 0.0))) + ((_e83.g0_.xxxw * _e86.g0_.wzyw) * vec4<f32>(1.0, -(1.0), 1.0, 1.0))));
}

fn plane_rotor_inner_anti_product(self_1052: Plane, other_868: Rotor) -> Plane {
    var self_1053: Plane;
    var other_869: Rotor;

    self_1053 = self_1052;
    other_869 = other_868;
    let _e4: Plane = self_1053;
    let _e8: Rotor = other_869;
    let _e19: Plane = self_1053;
    let _e23: Rotor = other_869;
    let _e35: Plane = self_1053;
    let _e38: Rotor = other_869;
    return Plane(((((vec4<f32>(_e4.g0_.y) * _e8.g0_.zwxz) * vec4<f32>(1.0, 1.0, -(1.0), 0.0)) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.yxwy) * vec4<f32>(-(1.0), 1.0, 1.0, 0.0))) + ((_e35.g0_.xxxw * _e38.g0_.wzyw) * vec4<f32>(1.0, -(1.0), 1.0, 1.0))));
}

fn plane_rotor_right_anti_contraction(self_1054: Plane, other_870: Rotor) -> Plane {
    var self_1055: Plane;
    var other_871: Rotor;

    self_1055 = self_1054;
    other_871 = other_870;
    let _e4: Plane = self_1055;
    let _e6: Rotor = other_871;
    return Plane((_e4.g0_ * vec4<f32>(_e6.g0_.w)));
}

fn plane_translator_inner_product(self_1056: Plane, other_872: Translator) -> Point {
    var self_1057: Plane;
    var other_873: Translator;

    self_1057 = self_1056;
    other_873 = other_872;
    let _e4: Plane = self_1057;
    let _e8: Translator = other_873;
    let _e20: Plane = self_1057;
    let _e24: Translator = other_873;
    let _e37: Plane = self_1057;
    let _e41: Translator = other_873;
    let _e45: Plane = self_1057;
    let _e49: Translator = other_873;
    return Point((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0))) + ((vec4<f32>(_e20.g0_.z) * vec4<f32>(_e24.g0_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + (vec4<f32>(_e37.g0_.w) * _e41.g0_)) + ((vec4<f32>(_e45.g0_.x) * vec4<f32>(_e49.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))));
}

fn plane_translator_inner_anti_product(self_1058: Plane, other_874: Translator) -> Plane {
    var self_1059: Plane;
    var other_875: Translator;

    self_1059 = self_1058;
    other_875 = other_874;
    let _e4: Plane = self_1059;
    let _e8: Translator = other_875;
    let _e18: Plane = self_1059;
    let _e22: Translator = other_875;
    let _e33: Plane = self_1059;
    let _e37: Translator = other_875;
    let _e49: Plane = self_1059;
    let _e53: Translator = other_875;
    return Plane((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, 1.0)) + ((vec4<f32>(_e18.g0_.z) * _e22.g0_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, 1.0))) + ((vec4<f32>(_e33.g0_.w) * vec4<f32>(_e37.g0_.w)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e49.g0_.x) * _e53.g0_.wxxx) * vec4<f32>(1.0, 0.0, 0.0, 1.0))));
}

fn plane_translator_right_contraction(self_1060: Plane, other_876: Translator) -> Point {
    var self_1061: Plane;
    var other_877: Translator;

    self_1061 = self_1060;
    other_877 = other_876;
    let _e4: Plane = self_1061;
    let _e8: Translator = other_877;
    let _e20: Plane = self_1061;
    let _e24: Translator = other_877;
    let _e37: Plane = self_1061;
    let _e40: Translator = other_877;
    return Point(((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0))) + ((vec4<f32>(_e20.g0_.z) * vec4<f32>(_e24.g0_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((_e37.g0_.wwwx * _e40.g0_.xyzx) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))));
}

fn plane_translator_right_anti_contraction(self_1062: Plane, other_878: Translator) -> Plane {
    var self_1063: Plane;
    var other_879: Translator;

    self_1063 = self_1062;
    other_879 = other_878;
    let _e4: Plane = self_1063;
    let _e6: Translator = other_879;
    return Plane((_e4.g0_ * vec4<f32>(_e6.g0_.w)));
}

fn plane_flector_add(self_1064: Plane, other_880: Flector) -> Flector {
    var self_1065: Plane;
    var other_881: Flector;

    self_1065 = self_1064;
    other_881 = other_880;
    let _e4: Flector = other_881;
    let _e6: Plane = self_1065;
    let _e8: Flector = other_881;
    return Flector(_e4.g0_, (_e6.g0_ + _e8.g1_));
}

fn plane_flector_sub(self_1066: Plane, other_882: Flector) -> Flector {
    var self_1067: Plane;
    var other_883: Flector;

    self_1067 = self_1066;
    other_883 = other_882;
    let _e6: Flector = other_883;
    let _e9: Plane = self_1067;
    let _e11: Flector = other_883;
    return Flector((vec4<f32>(0.0) - _e6.g0_), (_e9.g0_ - _e11.g1_));
}

fn plane_flector_outer_product(self_1068: Plane, other_884: Flector) -> AntiScalar {
    var self_1069: Plane;
    var other_885: Flector;

    self_1069 = self_1068;
    other_885 = other_884;
    let _e5: Plane = self_1069;
    let _e8: Flector = other_885;
    let _e13: Plane = self_1069;
    let _e16: Flector = other_885;
    let _e21: Plane = self_1069;
    let _e24: Flector = other_885;
    let _e29: Plane = self_1069;
    let _e32: Flector = other_885;
    return AntiScalar(((((0.0 - (_e5.g0_.x * _e8.g0_.x)) - (_e13.g0_.y * _e16.g0_.y)) - (_e21.g0_.z * _e24.g0_.z)) - (_e29.g0_.w * _e32.g0_.w)));
}

fn plane_flector_inner_anti_product(self_1070: Plane, other_886: Flector) -> Motor {
    var self_1071: Plane;
    var other_887: Flector;

    self_1071 = self_1070;
    other_887 = other_886;
    let _e4: Plane = self_1071;
    let _e8: Flector = other_887;
    let _e11: Flector = other_887;
    let _e14: Flector = other_887;
    let _e17: Flector = other_887;
    let _e29: Plane = self_1071;
    let _e33: Flector = other_887;
    let _e36: Flector = other_887;
    let _e39: Flector = other_887;
    let _e42: Flector = other_887;
    let _e55: Plane = self_1071;
    let _e59: Flector = other_887;
    let _e62: Flector = other_887;
    let _e65: Flector = other_887;
    let _e68: Flector = other_887;
    let _e81: Plane = self_1071;
    let _e85: Flector = other_887;
    let _e88: Flector = other_887;
    let _e91: Flector = other_887;
    let _e102: Plane = self_1071;
    let _e106: Flector = other_887;
    let _e109: Flector = other_887;
    let _e112: Flector = other_887;
    let _e124: Plane = self_1071;
    let _e128: Flector = other_887;
    let _e131: Flector = other_887;
    let _e134: Flector = other_887;
    return Motor(((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.w, _e11.g0_.w, _e14.g0_.w, _e17.g1_.y)) * vec4<f32>(0.0, -(1.0), 0.0, 1.0)) + ((vec4<f32>(_e29.g0_.z) * vec4<f32>(_e33.g0_.w, _e36.g0_.w, _e39.g0_.w, _e42.g1_.z)) * vec4<f32>(0.0, 0.0, -(1.0), 1.0))) + ((vec4<f32>(_e55.g0_.x) * vec4<f32>(_e59.g0_.w, _e62.g0_.x, _e65.g0_.x, _e68.g1_.x)) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))), ((((vec3<f32>(_e81.g0_.y) * vec3<f32>(_e85.g0_.z, _e88.g0_.z, _e91.g0_.x)) * vec3<f32>(1.0, 0.0, -(1.0))) + ((vec3<f32>(_e102.g0_.z) * vec3<f32>(_e106.g0_.y, _e109.g0_.x, _e112.g0_.y)) * vec3<f32>(-(1.0), 1.0, 0.0))) + ((vec3<f32>(_e124.g0_.x) * vec3<f32>(_e128.g0_.x, _e131.g0_.z, _e134.g0_.y)) * vec3<f32>(0.0, -(1.0), 1.0))));
}

fn plane_flector_left_contraction(self_1072: Plane, other_888: Flector) -> Scalar {
    var self_1073: Plane;
    var other_889: Flector;

    self_1073 = self_1072;
    other_889 = other_888;
    let _e5: Plane = self_1073;
    let _e8: Flector = other_889;
    return Scalar((0.0 - (_e5.g0_.w * _e8.g1_.w)));
}

fn plane_flector_left_anti_contraction(self_1074: Plane, other_890: Flector) -> Motor {
    var self_1075: Plane;
    var other_891: Flector;

    self_1075 = self_1074;
    other_891 = other_890;
    let _e4: Plane = self_1075;
    let _e8: Flector = other_891;
    let _e11: Flector = other_891;
    let _e14: Flector = other_891;
    let _e17: Flector = other_891;
    let _e29: Plane = self_1075;
    let _e33: Flector = other_891;
    let _e36: Flector = other_891;
    let _e39: Flector = other_891;
    let _e42: Flector = other_891;
    let _e55: Plane = self_1075;
    let _e59: Flector = other_891;
    let _e62: Flector = other_891;
    let _e65: Flector = other_891;
    let _e68: Flector = other_891;
    let _e81: Plane = self_1075;
    let _e85: Flector = other_891;
    let _e88: Flector = other_891;
    let _e91: Flector = other_891;
    let _e102: Plane = self_1075;
    let _e106: Flector = other_891;
    let _e109: Flector = other_891;
    let _e112: Flector = other_891;
    let _e124: Plane = self_1075;
    let _e128: Flector = other_891;
    let _e131: Flector = other_891;
    let _e134: Flector = other_891;
    return Motor(((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.w, _e11.g0_.w, _e14.g0_.w, _e17.g1_.y)) * vec4<f32>(0.0, -(1.0), 0.0, 1.0)) + ((vec4<f32>(_e29.g0_.z) * vec4<f32>(_e33.g0_.w, _e36.g0_.w, _e39.g0_.w, _e42.g1_.z)) * vec4<f32>(0.0, 0.0, -(1.0), 1.0))) + ((vec4<f32>(_e55.g0_.x) * vec4<f32>(_e59.g0_.w, _e62.g0_.x, _e65.g0_.x, _e68.g1_.x)) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))), ((((vec3<f32>(_e81.g0_.y) * vec3<f32>(_e85.g0_.z, _e88.g0_.z, _e91.g0_.x)) * vec3<f32>(1.0, 0.0, -(1.0))) + ((vec3<f32>(_e102.g0_.z) * vec3<f32>(_e106.g0_.y, _e109.g0_.x, _e112.g0_.y)) * vec3<f32>(-(1.0), 1.0, 0.0))) + ((vec3<f32>(_e124.g0_.x) * vec3<f32>(_e128.g0_.x, _e131.g0_.z, _e134.g0_.y)) * vec3<f32>(0.0, -(1.0), 1.0))));
}

fn plane_flector_right_anti_contraction(self_1076: Plane, other_892: Flector) -> AntiScalar {
    var self_1077: Plane;
    var other_893: Flector;

    self_1077 = self_1076;
    other_893 = other_892;
    let _e4: Plane = self_1077;
    let _e7: Flector = other_893;
    let _e11: Plane = self_1077;
    let _e14: Flector = other_893;
    let _e19: Plane = self_1077;
    let _e22: Flector = other_893;
    return AntiScalar((((_e4.g0_.x * _e7.g1_.x) + (_e11.g0_.y * _e14.g1_.y)) + (_e19.g0_.z * _e22.g1_.z)));
}

fn plane_flector_scalar_product(self_1078: Plane, other_894: Flector) -> Scalar {
    var self_1079: Plane;
    var other_895: Flector;

    self_1079 = self_1078;
    other_895 = other_894;
    let _e5: Plane = self_1079;
    let _e8: Flector = other_895;
    return Scalar((0.0 - (_e5.g0_.w * _e8.g1_.w)));
}

fn plane_flector_anti_scalar_product(self_1080: Plane, other_896: Flector) -> AntiScalar {
    var self_1081: Plane;
    var other_897: Flector;

    self_1081 = self_1080;
    other_897 = other_896;
    let _e4: Plane = self_1081;
    let _e7: Flector = other_897;
    let _e11: Plane = self_1081;
    let _e14: Flector = other_897;
    let _e19: Plane = self_1081;
    let _e22: Flector = other_897;
    return AntiScalar((((_e4.g0_.x * _e7.g1_.x) + (_e11.g0_.y * _e14.g1_.y)) + (_e19.g0_.z * _e22.g1_.z)));
}

fn plane_multi_vector_scalar_product(self_1082: Plane, other_898: MultiVector) -> Scalar {
    var self_1083: Plane;
    var other_899: MultiVector;

    self_1083 = self_1082;
    other_899 = other_898;
    let _e5: Plane = self_1083;
    let _e8: MultiVector = other_899;
    return Scalar((0.0 - (_e5.g0_.w * _e8.g6_.w)));
}

fn plane_multi_vector_anti_scalar_product(self_1084: Plane, other_900: MultiVector) -> AntiScalar {
    var self_1085: Plane;
    var other_901: MultiVector;

    self_1085 = self_1084;
    other_901 = other_900;
    let _e4: Plane = self_1085;
    let _e7: MultiVector = other_901;
    let _e11: Plane = self_1085;
    let _e14: MultiVector = other_901;
    let _e19: Plane = self_1085;
    let _e22: MultiVector = other_901;
    return AntiScalar((((_e4.g0_.x * _e7.g6_.x) + (_e11.g0_.y * _e14.g6_.y)) + (_e19.g0_.z * _e22.g6_.z)));
}

fn plane_squared_magnitude(self_1086: Plane) -> Scalar {
    var self_1087: Plane;

    self_1087 = self_1086;
    let _e2: Plane = self_1087;
    let _e3: Plane = self_1087;
    let _e4: Plane = plane_reversal(_e3);
    let _e5: Scalar = plane_plane_scalar_product(_e2, _e4);
    return _e5;
}

fn plane_magnitude(self_1088: Plane) -> Scalar {
    var self_1089: Plane;

    self_1089 = self_1088;
    let _e2: Plane = self_1089;
    let _e3: Scalar = plane_squared_magnitude(_e2);
    return Scalar(sqrt(_e3.g0_));
}

fn plane_bulk_norm(self_1090: Plane) -> Scalar {
    var self_1091: Plane;

    self_1091 = self_1090;
    let _e2: Plane = self_1091;
    let _e3: Scalar = plane_squared_magnitude(_e2);
    return Scalar(sqrt(_e3.g0_));
}

fn plane_squared_anti_magnitude(self_1092: Plane) -> AntiScalar {
    var self_1093: Plane;

    self_1093 = self_1092;
    let _e2: Plane = self_1093;
    let _e3: Plane = self_1093;
    let _e4: Plane = plane_anti_reversal(_e3);
    let _e5: AntiScalar = plane_plane_anti_scalar_product(_e2, _e4);
    return _e5;
}

fn plane_weight_norm(self_1094: Plane) -> AntiScalar {
    var self_1095: Plane;

    self_1095 = self_1094;
    let _e2: Plane = self_1095;
    let _e3: AntiScalar = plane_squared_anti_magnitude(_e2);
    return AntiScalar(sqrt(_e3.g0_));
}

fn plane_geometric_norm(self_1096: Plane) -> HomogeneousMagnitude {
    var self_1097: Plane;

    self_1097 = self_1096;
    let _e2: Plane = self_1097;
    let _e3: Scalar = plane_bulk_norm(_e2);
    let _e4: Plane = self_1097;
    let _e5: AntiScalar = plane_weight_norm(_e4);
    return (_e3 + _e5);
}

fn plane_scale(self_1098: Plane, other_902: f32) -> Plane {
    var self_1099: Plane;
    var other_903: f32;

    self_1099 = self_1098;
    other_903 = other_902;
    let _e4: Plane = self_1099;
    let _e5: f32 = other_903;
    let _e7: Plane = plane_scalar_geometric_product(_e4, Scalar(_e5));
    return _e7;
}

fn plane_signum(self_1100: Plane) -> Plane {
    var self_1101: Plane;

    self_1101 = self_1100;
    let _e2: Plane = self_1101;
    let _e3: Plane = self_1101;
    let _e4: Scalar = plane_magnitude(_e3);
    let _e9: Plane = plane_scalar_geometric_product(_e2, Scalar((1.0 / _e4.g0_)));
    return _e9;
}

fn plane_inverse(self_1102: Plane) -> Plane {
    var self_1103: Plane;

    self_1103 = self_1102;
    let _e2: Plane = self_1103;
    let _e3: Plane = plane_reversal(_e2);
    let _e4: Plane = self_1103;
    let _e5: Scalar = plane_squared_magnitude(_e4);
    let _e10: Plane = plane_scalar_geometric_product(_e3, Scalar((1.0 / _e5.g0_)));
    return _e10;
}

fn plane_unitize(self_1104: Plane) -> Plane {
    var self_1105: Plane;

    self_1105 = self_1104;
    let _e2: Plane = self_1105;
    let _e3: Plane = self_1105;
    let _e4: AntiScalar = plane_weight_norm(_e3);
    let _e9: Plane = plane_scalar_geometric_product(_e2, Scalar((1.0 / _e4.g0_)));
    return _e9;
}

fn motor_zero() -> Motor {
    return Motor(vec4<f32>(0.0), vec3<f32>(0.0));
}

fn motor_one() -> Motor {
    return Motor(vec4<f32>(0.0), vec3<f32>(0.0));
}

fn motor_neg(self_1106: Motor) -> Motor {
    var self_1107: Motor;

    self_1107 = self_1106;
    let _e2: Motor = self_1107;
    let _e8: Motor = self_1107;
    return Motor((_e2.g0_ * vec4<f32>(-(1.0))), (_e8.g1_ * vec3<f32>(-(1.0))));
}

fn motor_automorphism(self_1108: Motor) -> Motor {
    var self_1109: Motor;

    self_1109 = self_1108;
    let _e2: Motor = self_1109;
    let _e4: Motor = self_1109;
    return Motor(_e2.g0_, _e4.g1_);
}

fn motor_reversal(self_1110: Motor) -> Motor {
    var self_1111: Motor;

    self_1111 = self_1110;
    let _e2: Motor = self_1111;
    let _e13: Motor = self_1111;
    return Motor((_e2.g0_ * vec4<f32>(-(1.0), -(1.0), -(1.0), 1.0)), (_e13.g1_ * vec3<f32>(-(1.0))));
}

fn motor_conjugation(self_1112: Motor) -> Motor {
    var self_1113: Motor;

    self_1113 = self_1112;
    let _e2: Motor = self_1113;
    let _e13: Motor = self_1113;
    return Motor((_e2.g0_ * vec4<f32>(-(1.0), -(1.0), -(1.0), 1.0)), (_e13.g1_ * vec3<f32>(-(1.0))));
}

fn motor_anti_reversal(self_1114: Motor) -> Motor {
    var self_1115: Motor;

    self_1115 = self_1114;
    let _e2: Motor = self_1115;
    let _e13: Motor = self_1115;
    return Motor((_e2.g0_ * vec4<f32>(-(1.0), -(1.0), -(1.0), 1.0)), (_e13.g1_ * vec3<f32>(-(1.0))));
}

fn motor_double_complement(self_1116: Motor) -> Motor {
    var self_1117: Motor;

    self_1117 = self_1116;
    let _e2: Motor = self_1117;
    let _e4: Motor = self_1117;
    return Motor(_e2.g0_, _e4.g1_);
}

fn motor_scalar_geometric_product(self_1118: Motor, other_904: Scalar) -> Motor {
    var self_1119: Motor;
    var other_905: Scalar;

    self_1119 = self_1118;
    other_905 = other_904;
    let _e4: Motor = self_1119;
    let _e6: Scalar = other_905;
    let _e10: Motor = self_1119;
    let _e12: Scalar = other_905;
    return Motor((_e4.g0_ * vec4<f32>(_e6.g0_)), (_e10.g1_ * vec3<f32>(_e12.g0_)));
}

fn motor_scalar_regressive_product(self_1120: Motor, other_906: Scalar) -> Scalar {
    var self_1121: Motor;
    var other_907: Scalar;

    self_1121 = self_1120;
    other_907 = other_906;
    let _e4: Motor = self_1121;
    let _e7: Scalar = other_907;
    return Scalar((_e4.g0_.w * _e7.g0_));
}

fn motor_scalar_outer_product(self_1122: Motor, other_908: Scalar) -> Motor {
    var self_1123: Motor;
    var other_909: Scalar;

    self_1123 = self_1122;
    other_909 = other_908;
    let _e4: Motor = self_1123;
    let _e6: Scalar = other_909;
    let _e10: Motor = self_1123;
    let _e12: Scalar = other_909;
    return Motor((_e4.g0_ * vec4<f32>(_e6.g0_)), (_e10.g1_ * vec3<f32>(_e12.g0_)));
}

fn motor_scalar_inner_product(self_1124: Motor, other_910: Scalar) -> Motor {
    var self_1125: Motor;
    var other_911: Scalar;

    self_1125 = self_1124;
    other_911 = other_910;
    let _e4: Motor = self_1125;
    let _e6: Scalar = other_911;
    let _e10: Motor = self_1125;
    let _e12: Scalar = other_911;
    return Motor((_e4.g0_ * vec4<f32>(_e6.g0_)), (_e10.g1_ * vec3<f32>(_e12.g0_)));
}

fn motor_scalar_right_contraction(self_1126: Motor, other_912: Scalar) -> Motor {
    var self_1127: Motor;
    var other_913: Scalar;

    self_1127 = self_1126;
    other_913 = other_912;
    let _e4: Motor = self_1127;
    let _e6: Scalar = other_913;
    let _e10: Motor = self_1127;
    let _e12: Scalar = other_913;
    return Motor((_e4.g0_ * vec4<f32>(_e6.g0_)), (_e10.g1_ * vec3<f32>(_e12.g0_)));
}

fn motor_anti_scalar_into(self_1128: Motor) -> AntiScalar {
    var self_1129: Motor;

    self_1129 = self_1128;
    let _e2: Motor = self_1129;
    return AntiScalar(_e2.g0_.w);
}

fn motor_anti_scalar_add(self_1130: Motor, other_914: AntiScalar) -> Motor {
    var self_1131: Motor;
    var other_915: AntiScalar;

    self_1131 = self_1130;
    other_915 = other_914;
    let _e4: Motor = self_1131;
    let _e6: AntiScalar = other_915;
    let _e16: Motor = self_1131;
    return Motor((_e4.g0_ + (vec4<f32>(_e6.g0_) * vec4<f32>(0.0, 0.0, 0.0, 1.0))), _e16.g1_);
}

fn motor_anti_scalar_sub(self_1132: Motor, other_916: AntiScalar) -> Motor {
    var self_1133: Motor;
    var other_917: AntiScalar;

    self_1133 = self_1132;
    other_917 = other_916;
    let _e4: Motor = self_1133;
    let _e6: AntiScalar = other_917;
    let _e16: Motor = self_1133;
    return Motor((_e4.g0_ - (vec4<f32>(_e6.g0_) * vec4<f32>(0.0, 0.0, 0.0, 1.0))), _e16.g1_);
}

fn motor_anti_scalar_regressive_product(self_1134: Motor, other_918: AntiScalar) -> Motor {
    var self_1135: Motor;
    var other_919: AntiScalar;

    self_1135 = self_1134;
    other_919 = other_918;
    let _e4: Motor = self_1135;
    let _e6: AntiScalar = other_919;
    let _e10: Motor = self_1135;
    let _e12: AntiScalar = other_919;
    return Motor((_e4.g0_ * vec4<f32>(_e6.g0_)), (_e10.g1_ * vec3<f32>(_e12.g0_)));
}

fn motor_anti_scalar_geometric_anti_product(self_1136: Motor, other_920: AntiScalar) -> Motor {
    var self_1137: Motor;
    var other_921: AntiScalar;

    self_1137 = self_1136;
    other_921 = other_920;
    let _e4: Motor = self_1137;
    let _e6: AntiScalar = other_921;
    let _e10: Motor = self_1137;
    let _e12: AntiScalar = other_921;
    return Motor((_e4.g0_ * vec4<f32>(_e6.g0_)), (_e10.g1_ * vec3<f32>(_e12.g0_)));
}

fn motor_anti_scalar_inner_anti_product(self_1138: Motor, other_922: AntiScalar) -> Motor {
    var self_1139: Motor;
    var other_923: AntiScalar;

    self_1139 = self_1138;
    other_923 = other_922;
    let _e4: Motor = self_1139;
    let _e6: AntiScalar = other_923;
    let _e10: Motor = self_1139;
    let _e12: AntiScalar = other_923;
    return Motor((_e4.g0_ * vec4<f32>(_e6.g0_)), (_e10.g1_ * vec3<f32>(_e12.g0_)));
}

fn motor_anti_scalar_left_anti_contraction(self_1140: Motor, other_924: AntiScalar) -> AntiScalar {
    var self_1141: Motor;
    var other_925: AntiScalar;

    self_1141 = self_1140;
    other_925 = other_924;
    let _e4: Motor = self_1141;
    let _e7: AntiScalar = other_925;
    return AntiScalar((_e4.g0_.w * _e7.g0_));
}

fn motor_anti_scalar_right_anti_contraction(self_1142: Motor, other_926: AntiScalar) -> Motor {
    var self_1143: Motor;
    var other_927: AntiScalar;

    self_1143 = self_1142;
    other_927 = other_926;
    let _e4: Motor = self_1143;
    let _e6: AntiScalar = other_927;
    let _e10: Motor = self_1143;
    let _e12: AntiScalar = other_927;
    return Motor((_e4.g0_ * vec4<f32>(_e6.g0_)), (_e10.g1_ * vec3<f32>(_e12.g0_)));
}

fn motor_anti_scalar_anti_scalar_product(self_1144: Motor, other_928: AntiScalar) -> AntiScalar {
    var self_1145: Motor;
    var other_929: AntiScalar;

    self_1145 = self_1144;
    other_929 = other_928;
    let _e4: Motor = self_1145;
    let _e7: AntiScalar = other_929;
    return AntiScalar((_e4.g0_.w * _e7.g0_));
}

fn motor_homogeneous_magnitude_geometric_product(self_1146: Motor, other_930: HomogeneousMagnitude) -> Motor {
    var self_1147: Motor;
    var other_931: HomogeneousMagnitude;

    self_1147 = self_1146;
    other_931 = other_930;
    let _e4: Motor = self_1147;
    let _e8: HomogeneousMagnitude = other_931;
    let _e19: Motor = self_1147;
    let _e23: HomogeneousMagnitude = other_931;
    let _e35: Motor = self_1147;
    let _e39: HomogeneousMagnitude = other_931;
    let _e51: Motor = self_1147;
    let _e53: HomogeneousMagnitude = other_931;
    let _e59: Motor = self_1147;
    let _e61: HomogeneousMagnitude = other_931;
    return Motor((((((vec4<f32>(_e4.g1_.x) * vec4<f32>(_e8.g0_.y)) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) + ((vec4<f32>(_e19.g1_.y) * vec4<f32>(_e23.g0_.y)) * vec4<f32>(0.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e35.g1_.z) * vec4<f32>(_e39.g0_.y)) * vec4<f32>(0.0, 0.0, 1.0, 0.0))) + (_e51.g0_ * vec4<f32>(_e53.g0_.x))), (_e59.g1_ * vec3<f32>(_e61.g0_.x)));
}

fn motor_homogeneous_magnitude_outer_product(self_1148: Motor, other_932: HomogeneousMagnitude) -> Motor {
    var self_1149: Motor;
    var other_933: HomogeneousMagnitude;

    self_1149 = self_1148;
    other_933 = other_932;
    let _e4: Motor = self_1149;
    let _e6: HomogeneousMagnitude = other_933;
    let _e11: Motor = self_1149;
    let _e13: HomogeneousMagnitude = other_933;
    return Motor((_e4.g0_ * vec4<f32>(_e6.g0_.x)), (_e11.g1_ * vec3<f32>(_e13.g0_.x)));
}

fn motor_homogeneous_magnitude_inner_product(self_1150: Motor, other_934: HomogeneousMagnitude) -> Motor {
    var self_1151: Motor;
    var other_935: HomogeneousMagnitude;

    self_1151 = self_1150;
    other_935 = other_934;
    let _e4: Motor = self_1151;
    let _e8: HomogeneousMagnitude = other_935;
    let _e19: Motor = self_1151;
    let _e23: HomogeneousMagnitude = other_935;
    let _e35: Motor = self_1151;
    let _e39: HomogeneousMagnitude = other_935;
    let _e51: Motor = self_1151;
    let _e53: HomogeneousMagnitude = other_935;
    let _e59: Motor = self_1151;
    let _e61: HomogeneousMagnitude = other_935;
    return Motor((((((vec4<f32>(_e4.g1_.x) * vec4<f32>(_e8.g0_.y)) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) + ((vec4<f32>(_e19.g1_.y) * vec4<f32>(_e23.g0_.y)) * vec4<f32>(0.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e35.g1_.z) * vec4<f32>(_e39.g0_.y)) * vec4<f32>(0.0, 0.0, 1.0, 0.0))) + (_e51.g0_ * vec4<f32>(_e53.g0_.x))), (_e59.g1_ * vec3<f32>(_e61.g0_.x)));
}

fn motor_homogeneous_magnitude_right_contraction(self_1152: Motor, other_936: HomogeneousMagnitude) -> Motor {
    var self_1153: Motor;
    var other_937: HomogeneousMagnitude;

    self_1153 = self_1152;
    other_937 = other_936;
    let _e4: Motor = self_1153;
    let _e6: HomogeneousMagnitude = other_937;
    let _e11: Motor = self_1153;
    let _e13: HomogeneousMagnitude = other_937;
    return Motor((_e4.g0_ * vec4<f32>(_e6.g0_.x)), (_e11.g1_ * vec3<f32>(_e13.g0_.x)));
}

fn motor_homogeneous_magnitude_right_anti_contraction(self_1154: Motor, other_938: HomogeneousMagnitude) -> Motor {
    var self_1155: Motor;
    var other_939: HomogeneousMagnitude;

    self_1155 = self_1154;
    other_939 = other_938;
    let _e4: Motor = self_1155;
    let _e6: HomogeneousMagnitude = other_939;
    let _e11: Motor = self_1155;
    let _e13: HomogeneousMagnitude = other_939;
    return Motor((_e4.g0_ * vec4<f32>(_e6.g0_.y)), (_e11.g1_ * vec3<f32>(_e13.g0_.y)));
}

fn motor_homogeneous_magnitude_anti_scalar_product(self_1156: Motor, other_940: HomogeneousMagnitude) -> AntiScalar {
    var self_1157: Motor;
    var other_941: HomogeneousMagnitude;

    self_1157 = self_1156;
    other_941 = other_940;
    let _e4: Motor = self_1157;
    let _e7: HomogeneousMagnitude = other_941;
    return AntiScalar((_e4.g0_.w * _e7.g0_.y));
}

fn motor_point_geometric_product(self_1158: Motor, other_942: Point) -> Flector {
    var self_1159: Motor;
    var other_943: Point;

    self_1159 = self_1158;
    other_943 = other_942;
    let _e4: Motor = self_1159;
    let _e8: Point = other_943;
    let _e19: Motor = self_1159;
    let _e23: Point = other_943;
    let _e35: Motor = self_1159;
    let _e39: Point = other_943;
    let _e51: Motor = self_1159;
    let _e55: Point = other_943;
    let _e67: Motor = self_1159;
    let _e71: Point = other_943;
    let _e83: Motor = self_1159;
    let _e87: Point = other_943;
    let _e99: Motor = self_1159;
    let _e103: Point = other_943;
    let _e114: Motor = self_1159;
    let _e118: Point = other_943;
    let _e130: Motor = self_1159;
    let _e134: Point = other_943;
    let _e148: Motor = self_1159;
    let _e152: Point = other_943;
    let _e164: Motor = self_1159;
    let _e168: Point = other_943;
    let _e180: Motor = self_1159;
    let _e184: Point = other_943;
    let _e196: Motor = self_1159;
    let _e200: Point = other_943;
    return Flector((((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y)) * vec4<f32>(0.0, 0.0, 0.0, 1.0)) + ((vec4<f32>(_e19.g0_.z) * vec4<f32>(_e23.g0_.z)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e35.g1_.x) * _e39.g0_.zzyz) * vec4<f32>(0.0, 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e51.g1_.y) * _e55.g0_.zzxz) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0))) + ((vec4<f32>(_e67.g1_.z) * _e71.g0_.yxyy) * vec4<f32>(1.0, -(1.0), 0.0, 0.0))) + ((vec4<f32>(_e83.g0_.x) * vec4<f32>(_e87.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))), ((((((((vec4<f32>(_e99.g0_.y) * _e103.g0_.zzxz) * vec4<f32>(1.0, 0.0, -(1.0), 0.0)) + ((vec4<f32>(_e114.g0_.z) * _e118.g0_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))) + ((vec4<f32>(_e130.g0_.w) * _e134.g0_.xyzx) * vec4<f32>(-(1.0), -(1.0), -(1.0), 0.0))) + ((vec4<f32>(_e148.g1_.x) * _e152.g0_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e164.g1_.y) * _e168.g0_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0)))) + ((vec4<f32>(_e180.g1_.z) * _e184.g0_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e196.g0_.x) * _e200.g0_.xzyx) * vec4<f32>(0.0, -(1.0), 1.0, 0.0))));
}

fn motor_point_regressive_product(self_1160: Motor, other_944: Point) -> Point {
    var self_1161: Motor;
    var other_945: Point;

    self_1161 = self_1160;
    other_945 = other_944;
    let _e4: Motor = self_1161;
    let _e8: Point = other_945;
    return Point((vec4<f32>(_e4.g0_.w) * _e8.g0_));
}

fn motor_point_outer_product(self_1162: Motor, other_946: Point) -> Plane {
    var self_1163: Motor;
    var other_947: Point;

    self_1163 = self_1162;
    other_947 = other_946;
    let _e4: Motor = self_1163;
    let _e8: Point = other_947;
    let _e19: Motor = self_1163;
    let _e23: Point = other_947;
    let _e35: Motor = self_1163;
    let _e39: Point = other_947;
    let _e51: Motor = self_1163;
    let _e55: Point = other_947;
    let _e67: Motor = self_1163;
    let _e71: Point = other_947;
    let _e83: Motor = self_1163;
    let _e87: Point = other_947;
    return Plane((((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.zzxz) * vec4<f32>(1.0, 0.0, -(1.0), 0.0)) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))) + ((vec4<f32>(_e35.g1_.x) * _e39.g0_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e51.g1_.y) * _e55.g0_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0)))) + ((vec4<f32>(_e67.g1_.z) * _e71.g0_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e83.g0_.x) * _e87.g0_.xzyx) * vec4<f32>(0.0, -(1.0), 1.0, 0.0))));
}

fn motor_point_geometric_anti_product(self_1164: Motor, other_948: Point) -> Flector {
    var self_1165: Motor;
    var other_949: Point;

    self_1165 = self_1164;
    other_949 = other_948;
    let _e4: Motor = self_1165;
    let _e8: Point = other_949;
    let _e19: Motor = self_1165;
    let _e23: Point = other_949;
    let _e35: Motor = self_1165;
    let _e39: Point = other_949;
    let _e43: Motor = self_1165;
    let _e47: Point = other_949;
    let _e59: Motor = self_1165;
    let _e63: Point = other_949;
    let _e75: Motor = self_1165;
    let _e79: Point = other_949;
    let _e91: Motor = self_1165;
    let _e95: Point = other_949;
    let _e107: Motor = self_1165;
    let _e111: Point = other_949;
    let _e122: Motor = self_1165;
    let _e126: Point = other_949;
    let _e138: Motor = self_1165;
    let _e142: Point = other_949;
    return Flector(((((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.zzxz) * vec4<f32>(1.0, 0.0, -(1.0), 0.0)) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))) + (vec4<f32>(_e35.g0_.w) * _e39.g0_)) + ((vec4<f32>(_e43.g1_.x) * vec4<f32>(_e47.g0_.w)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e59.g1_.y) * vec4<f32>(_e63.g0_.w)) * vec4<f32>(0.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e75.g1_.z) * vec4<f32>(_e79.g0_.w)) * vec4<f32>(0.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e91.g0_.x) * _e95.g0_.xzyx) * vec4<f32>(0.0, -(1.0), 1.0, 0.0))), ((((vec4<f32>(_e107.g0_.y) * _e111.g0_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e122.g0_.z) * _e126.g0_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e138.g0_.x) * _e142.g0_.wxxx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))));
}

fn motor_point_inner_anti_product(self_1166: Motor, other_950: Point) -> Flector {
    var self_1167: Motor;
    var other_951: Point;

    self_1167 = self_1166;
    other_951 = other_950;
    let _e4: Motor = self_1167;
    let _e8: Point = other_951;
    let _e11: Motor = self_1167;
    let _e15: Point = other_951;
    let _e26: Motor = self_1167;
    let _e30: Point = other_951;
    let _e42: Motor = self_1167;
    let _e46: Point = other_951;
    return Flector((vec4<f32>(_e4.g0_.w) * _e8.g0_), ((((vec4<f32>(_e11.g0_.y) * _e15.g0_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e26.g0_.z) * _e30.g0_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e42.g0_.x) * _e46.g0_.wxxx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))));
}

fn motor_point_left_anti_contraction(self_1168: Motor, other_952: Point) -> Flector {
    var self_1169: Motor;
    var other_953: Point;

    self_1169 = self_1168;
    other_953 = other_952;
    let _e4: Motor = self_1169;
    let _e8: Point = other_953;
    let _e11: Motor = self_1169;
    let _e15: Point = other_953;
    let _e26: Motor = self_1169;
    let _e30: Point = other_953;
    let _e42: Motor = self_1169;
    let _e46: Point = other_953;
    return Flector((vec4<f32>(_e4.g0_.w) * _e8.g0_), ((((vec4<f32>(_e11.g0_.y) * _e15.g0_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e26.g0_.z) * _e30.g0_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e42.g0_.x) * _e46.g0_.wxxx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))));
}

fn motor_line_into(self_1170: Motor) -> Line {
    var self_1171: Motor;

    self_1171 = self_1170;
    let _e2: Motor = self_1171;
    let _e5: Motor = self_1171;
    let _e8: Motor = self_1171;
    let _e12: Motor = self_1171;
    return Line(vec3<f32>(_e2.g0_.x, _e5.g0_.y, _e8.g0_.z), _e12.g1_);
}

fn motor_line_add(self_1172: Motor, other_954: Line) -> Motor {
    var self_1173: Motor;
    var other_955: Line;

    self_1173 = self_1172;
    other_955 = other_954;
    let _e4: Motor = self_1173;
    let _e6: Line = other_955;
    let _e9: Line = other_955;
    let _e12: Line = other_955;
    let _e15: Line = other_955;
    let _e26: Motor = self_1173;
    let _e28: Line = other_955;
    return Motor((_e4.g0_ + (vec4<f32>(_e6.g0_.x, _e9.g0_.y, _e12.g0_.z, _e15.g0_.x) * vec4<f32>(1.0, 1.0, 1.0, 0.0))), (_e26.g1_ + _e28.g1_));
}

fn motor_line_sub(self_1174: Motor, other_956: Line) -> Motor {
    var self_1175: Motor;
    var other_957: Line;

    self_1175 = self_1174;
    other_957 = other_956;
    let _e4: Motor = self_1175;
    let _e6: Line = other_957;
    let _e9: Line = other_957;
    let _e12: Line = other_957;
    let _e15: Line = other_957;
    let _e26: Motor = self_1175;
    let _e28: Line = other_957;
    return Motor((_e4.g0_ - (vec4<f32>(_e6.g0_.x, _e9.g0_.y, _e12.g0_.z, _e15.g0_.x) * vec4<f32>(1.0, 1.0, 1.0, 0.0))), (_e26.g1_ - _e28.g1_));
}

fn motor_line_outer_product(self_1176: Motor, other_958: Line) -> AntiScalar {
    var self_1177: Motor;
    var other_959: Line;

    self_1177 = self_1176;
    other_959 = other_958;
    let _e5: Motor = self_1177;
    let _e8: Line = other_959;
    let _e13: Motor = self_1177;
    let _e16: Line = other_959;
    let _e21: Motor = self_1177;
    let _e24: Line = other_959;
    let _e29: Motor = self_1177;
    let _e32: Line = other_959;
    let _e37: Motor = self_1177;
    let _e40: Line = other_959;
    let _e45: Motor = self_1177;
    let _e48: Line = other_959;
    return AntiScalar(((((((0.0 - (_e5.g0_.x * _e8.g1_.x)) - (_e13.g0_.y * _e16.g1_.y)) - (_e21.g0_.z * _e24.g1_.z)) - (_e29.g1_.x * _e32.g0_.x)) - (_e37.g1_.y * _e40.g0_.y)) - (_e45.g1_.z * _e48.g0_.z)));
}

fn motor_line_inner_anti_product(self_1178: Motor, other_960: Line) -> Motor {
    var self_1179: Motor;
    var other_961: Line;

    self_1179 = self_1178;
    other_961 = other_960;
    let _e4: Motor = self_1179;
    let _e8: Line = other_961;
    let _e20: Motor = self_1179;
    let _e24: Line = other_961;
    let _e37: Motor = self_1179;
    let _e40: Line = other_961;
    let _e43: Line = other_961;
    let _e46: Line = other_961;
    let _e49: Line = other_961;
    let _e62: Motor = self_1179;
    let _e66: Line = other_961;
    return Motor(((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0))) + ((vec4<f32>(_e20.g0_.z) * vec4<f32>(_e24.g0_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((_e37.g0_.wwwx * vec4<f32>(_e40.g0_.x, _e43.g0_.y, _e46.g0_.z, _e49.g0_.x)) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))), (vec3<f32>(_e62.g0_.w) * _e66.g1_));
}

fn motor_line_left_contraction(self_1180: Motor, other_962: Line) -> Scalar {
    var self_1181: Motor;
    var other_963: Line;

    self_1181 = self_1180;
    other_963 = other_962;
    let _e5: Motor = self_1181;
    let _e8: Line = other_963;
    let _e13: Motor = self_1181;
    let _e16: Line = other_963;
    let _e21: Motor = self_1181;
    let _e24: Line = other_963;
    return Scalar((((0.0 - (_e5.g1_.x * _e8.g1_.x)) - (_e13.g1_.y * _e16.g1_.y)) - (_e21.g1_.z * _e24.g1_.z)));
}

fn motor_line_left_anti_contraction(self_1182: Motor, other_964: Line) -> Motor {
    var self_1183: Motor;
    var other_965: Line;

    self_1183 = self_1182;
    other_965 = other_964;
    let _e4: Motor = self_1183;
    let _e8: Line = other_965;
    let _e20: Motor = self_1183;
    let _e24: Line = other_965;
    let _e37: Motor = self_1183;
    let _e40: Line = other_965;
    let _e43: Line = other_965;
    let _e46: Line = other_965;
    let _e49: Line = other_965;
    let _e62: Motor = self_1183;
    let _e66: Line = other_965;
    return Motor(((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0))) + ((vec4<f32>(_e20.g0_.z) * vec4<f32>(_e24.g0_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((_e37.g0_.wwwx * vec4<f32>(_e40.g0_.x, _e43.g0_.y, _e46.g0_.z, _e49.g0_.x)) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))), (vec3<f32>(_e62.g0_.w) * _e66.g1_));
}

fn motor_line_right_anti_contraction(self_1184: Motor, other_966: Line) -> AntiScalar {
    var self_1185: Motor;
    var other_967: Line;

    self_1185 = self_1184;
    other_967 = other_966;
    let _e5: Motor = self_1185;
    let _e8: Line = other_967;
    let _e13: Motor = self_1185;
    let _e16: Line = other_967;
    let _e21: Motor = self_1185;
    let _e24: Line = other_967;
    return AntiScalar((((0.0 - (_e5.g0_.x * _e8.g0_.x)) - (_e13.g0_.y * _e16.g0_.y)) - (_e21.g0_.z * _e24.g0_.z)));
}

fn motor_line_scalar_product(self_1186: Motor, other_968: Line) -> Scalar {
    var self_1187: Motor;
    var other_969: Line;

    self_1187 = self_1186;
    other_969 = other_968;
    let _e5: Motor = self_1187;
    let _e8: Line = other_969;
    let _e13: Motor = self_1187;
    let _e16: Line = other_969;
    let _e21: Motor = self_1187;
    let _e24: Line = other_969;
    return Scalar((((0.0 - (_e5.g1_.x * _e8.g1_.x)) - (_e13.g1_.y * _e16.g1_.y)) - (_e21.g1_.z * _e24.g1_.z)));
}

fn motor_line_anti_scalar_product(self_1188: Motor, other_970: Line) -> AntiScalar {
    var self_1189: Motor;
    var other_971: Line;

    self_1189 = self_1188;
    other_971 = other_970;
    let _e5: Motor = self_1189;
    let _e8: Line = other_971;
    let _e13: Motor = self_1189;
    let _e16: Line = other_971;
    let _e21: Motor = self_1189;
    let _e24: Line = other_971;
    return AntiScalar((((0.0 - (_e5.g0_.x * _e8.g0_.x)) - (_e13.g0_.y * _e16.g0_.y)) - (_e21.g0_.z * _e24.g0_.z)));
}

fn motor_plane_regressive_product(self_1190: Motor, other_972: Plane) -> Flector {
    var self_1191: Motor;
    var other_973: Plane;

    self_1191 = self_1190;
    other_973 = other_972;
    let _e4: Motor = self_1191;
    let _e8: Plane = other_973;
    let _e19: Motor = self_1191;
    let _e23: Plane = other_973;
    let _e35: Motor = self_1191;
    let _e39: Plane = other_973;
    let _e51: Motor = self_1191;
    let _e55: Plane = other_973;
    let _e67: Motor = self_1191;
    let _e71: Plane = other_973;
    let _e83: Motor = self_1191;
    let _e87: Plane = other_973;
    let _e99: Motor = self_1191;
    let _e103: Plane = other_973;
    return Flector((((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e35.g1_.x) * _e39.g0_.zzyz) * vec4<f32>(0.0, -(1.0), 1.0, 0.0))) + ((vec4<f32>(_e51.g1_.y) * _e55.g0_.zzxz) * vec4<f32>(1.0, 0.0, -(1.0), 0.0))) + ((vec4<f32>(_e67.g1_.z) * _e71.g0_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))) + ((vec4<f32>(_e83.g0_.x) * _e87.g0_.wxxx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))), (vec4<f32>(_e99.g0_.w) * _e103.g0_));
}

fn motor_plane_inner_product(self_1192: Motor, other_974: Plane) -> Point {
    var self_1193: Motor;
    var other_975: Plane;

    self_1193 = self_1192;
    other_975 = other_974;
    let _e4: Motor = self_1193;
    let _e8: Plane = other_975;
    let _e19: Motor = self_1193;
    let _e23: Plane = other_975;
    let _e35: Motor = self_1193;
    let _e39: Plane = other_975;
    let _e51: Motor = self_1193;
    let _e54: Plane = other_975;
    return Point((((((vec4<f32>(_e4.g1_.x) * _e8.g0_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0))) + ((vec4<f32>(_e19.g1_.y) * _e23.g0_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0)))) + ((vec4<f32>(_e35.g1_.z) * _e39.g0_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((_e51.g0_.xxxw * _e54.g0_.xxxw) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))));
}

fn motor_plane_geometric_anti_product(self_1194: Motor, other_976: Plane) -> Flector {
    var self_1195: Motor;
    var other_977: Plane;

    self_1195 = self_1194;
    other_977 = other_976;
    let _e4: Motor = self_1195;
    let _e8: Plane = other_977;
    let _e19: Motor = self_1195;
    let _e23: Plane = other_977;
    let _e35: Motor = self_1195;
    let _e39: Plane = other_977;
    let _e51: Motor = self_1195;
    let _e55: Plane = other_977;
    let _e67: Motor = self_1195;
    let _e71: Plane = other_977;
    let _e83: Motor = self_1195;
    let _e87: Plane = other_977;
    let _e99: Motor = self_1195;
    let _e103: Plane = other_977;
    let _e114: Motor = self_1195;
    let _e118: Plane = other_977;
    let _e130: Motor = self_1195;
    let _e134: Plane = other_977;
    let _e138: Motor = self_1195;
    let _e142: Plane = other_977;
    let _e155: Motor = self_1195;
    let _e159: Plane = other_977;
    let _e172: Motor = self_1195;
    let _e176: Plane = other_977;
    let _e189: Motor = self_1195;
    let _e193: Plane = other_977;
    return Flector((((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e35.g1_.x) * _e39.g0_.zzyz) * vec4<f32>(0.0, -(1.0), 1.0, 0.0))) + ((vec4<f32>(_e51.g1_.y) * _e55.g0_.zzxz) * vec4<f32>(1.0, 0.0, -(1.0), 0.0))) + ((vec4<f32>(_e67.g1_.z) * _e71.g0_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))) + ((vec4<f32>(_e83.g0_.x) * _e87.g0_.wxxx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))), ((((((((vec4<f32>(_e99.g0_.y) * _e103.g0_.zzxz) * vec4<f32>(1.0, 0.0, -(1.0), 0.0)) + ((vec4<f32>(_e114.g0_.z) * _e118.g0_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))) + (vec4<f32>(_e130.g0_.w) * _e134.g0_)) + ((vec4<f32>(_e138.g1_.x) * vec4<f32>(_e142.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e155.g1_.y) * vec4<f32>(_e159.g0_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e172.g1_.z) * vec4<f32>(_e176.g0_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e189.g0_.x) * _e193.g0_.xzyx) * vec4<f32>(0.0, -(1.0), 1.0, 0.0))));
}

fn motor_plane_inner_anti_product(self_1196: Motor, other_978: Plane) -> Plane {
    var self_1197: Motor;
    var other_979: Plane;

    self_1197 = self_1196;
    other_979 = other_978;
    let _e4: Motor = self_1197;
    let _e8: Plane = other_979;
    let _e19: Motor = self_1197;
    let _e23: Plane = other_979;
    let _e35: Motor = self_1197;
    let _e39: Plane = other_979;
    let _e43: Motor = self_1197;
    let _e47: Plane = other_979;
    let _e60: Motor = self_1197;
    let _e64: Plane = other_979;
    let _e77: Motor = self_1197;
    let _e81: Plane = other_979;
    let _e94: Motor = self_1197;
    let _e98: Plane = other_979;
    return Plane(((((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.zzxz) * vec4<f32>(1.0, 0.0, -(1.0), 0.0)) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))) + (vec4<f32>(_e35.g0_.w) * _e39.g0_)) + ((vec4<f32>(_e43.g1_.x) * vec4<f32>(_e47.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e60.g1_.y) * vec4<f32>(_e64.g0_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e77.g1_.z) * vec4<f32>(_e81.g0_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e94.g0_.x) * _e98.g0_.xzyx) * vec4<f32>(0.0, -(1.0), 1.0, 0.0))));
}

fn motor_plane_left_contraction(self_1198: Motor, other_980: Plane) -> Point {
    var self_1199: Motor;
    var other_981: Plane;

    self_1199 = self_1198;
    other_981 = other_980;
    let _e4: Motor = self_1199;
    let _e8: Plane = other_981;
    let _e19: Motor = self_1199;
    let _e23: Plane = other_981;
    let _e35: Motor = self_1199;
    let _e39: Plane = other_981;
    return Point(((((vec4<f32>(_e4.g1_.y) * _e8.g0_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e19.g1_.z) * _e23.g0_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e35.g1_.x) * _e39.g0_.wxxx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))));
}

fn motor_plane_left_anti_contraction(self_1200: Motor, other_982: Plane) -> Plane {
    var self_1201: Motor;
    var other_983: Plane;

    self_1201 = self_1200;
    other_983 = other_982;
    let _e4: Motor = self_1201;
    let _e8: Plane = other_983;
    return Plane((vec4<f32>(_e4.g0_.w) * _e8.g0_));
}

fn motor_plane_right_anti_contraction(self_1202: Motor, other_984: Plane) -> Plane {
    var self_1203: Motor;
    var other_985: Plane;

    self_1203 = self_1202;
    other_985 = other_984;
    let _e4: Motor = self_1203;
    let _e8: Plane = other_985;
    let _e19: Motor = self_1203;
    let _e23: Plane = other_985;
    let _e35: Motor = self_1203;
    let _e39: Plane = other_985;
    let _e52: Motor = self_1203;
    let _e56: Plane = other_985;
    let _e69: Motor = self_1203;
    let _e73: Plane = other_985;
    let _e86: Motor = self_1203;
    let _e90: Plane = other_985;
    return Plane((((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.zzxz) * vec4<f32>(1.0, 0.0, -(1.0), 0.0)) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))) + ((vec4<f32>(_e35.g1_.x) * vec4<f32>(_e39.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e52.g1_.y) * vec4<f32>(_e56.g0_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e69.g1_.z) * vec4<f32>(_e73.g0_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e86.g0_.x) * _e90.g0_.xzyx) * vec4<f32>(0.0, -(1.0), 1.0, 0.0))));
}

fn motor_motor_add(self_1204: Motor, other_986: Motor) -> Motor {
    var self_1205: Motor;
    var other_987: Motor;

    self_1205 = self_1204;
    other_987 = other_986;
    let _e4: Motor = self_1205;
    let _e6: Motor = other_987;
    let _e9: Motor = self_1205;
    let _e11: Motor = other_987;
    return Motor((_e4.g0_ + _e6.g0_), (_e9.g1_ + _e11.g1_));
}

fn motor_motor_sub(self_1206: Motor, other_988: Motor) -> Motor {
    var self_1207: Motor;
    var other_989: Motor;

    self_1207 = self_1206;
    other_989 = other_988;
    let _e4: Motor = self_1207;
    let _e6: Motor = other_989;
    let _e9: Motor = self_1207;
    let _e11: Motor = other_989;
    return Motor((_e4.g0_ - _e6.g0_), (_e9.g1_ - _e11.g1_));
}

fn motor_motor_mul(self_1208: Motor, other_990: Motor) -> Motor {
    var self_1209: Motor;
    var other_991: Motor;

    self_1209 = self_1208;
    other_991 = other_990;
    let _e4: Motor = self_1209;
    let _e6: Motor = other_991;
    let _e9: Motor = self_1209;
    let _e11: Motor = other_991;
    return Motor((_e4.g0_ * _e6.g0_), (_e9.g1_ * _e11.g1_));
}

fn motor_motor_div(self_1210: Motor, other_992: Motor) -> Motor {
    var self_1211: Motor;
    var other_993: Motor;

    self_1211 = self_1210;
    other_993 = other_992;
    let _e4: Motor = self_1211;
    let _e7: Motor = self_1211;
    let _e10: Motor = self_1211;
    let _e13: Motor = self_1211;
    let _e23: Motor = other_993;
    let _e26: Motor = other_993;
    let _e29: Motor = other_993;
    let _e32: Motor = other_993;
    let _e43: Motor = self_1211;
    let _e46: Motor = self_1211;
    let _e49: Motor = self_1211;
    let _e58: Motor = other_993;
    let _e61: Motor = other_993;
    let _e64: Motor = other_993;
    return Motor((((vec4<f32>(_e4.g0_.x, _e7.g0_.y, _e10.g0_.z, _e13.g0_.w) * vec4<f32>(1.0, 1.0, 1.0, 1.0)) / vec4<f32>(_e23.g0_.x, _e26.g0_.y, _e29.g0_.z, _e32.g0_.w)) * vec4<f32>(1.0, 1.0, 1.0, 1.0)), (((vec3<f32>(_e43.g1_.x, _e46.g1_.y, _e49.g1_.z) * vec3<f32>(1.0, 1.0, 1.0)) / vec3<f32>(_e58.g1_.x, _e61.g1_.y, _e64.g1_.z)) * vec3<f32>(1.0, 1.0, 1.0)));
}

fn motor_motor_outer_product(self_1212: Motor, other_994: Motor) -> AntiScalar {
    var self_1213: Motor;
    var other_995: Motor;

    self_1213 = self_1212;
    other_995 = other_994;
    let _e5: Motor = self_1213;
    let _e8: Motor = other_995;
    let _e13: Motor = self_1213;
    let _e16: Motor = other_995;
    let _e21: Motor = self_1213;
    let _e24: Motor = other_995;
    let _e29: Motor = self_1213;
    let _e32: Motor = other_995;
    let _e37: Motor = self_1213;
    let _e40: Motor = other_995;
    let _e45: Motor = self_1213;
    let _e48: Motor = other_995;
    return AntiScalar(((((((0.0 - (_e5.g0_.x * _e8.g1_.x)) - (_e13.g0_.y * _e16.g1_.y)) - (_e21.g0_.z * _e24.g1_.z)) - (_e29.g1_.x * _e32.g0_.x)) - (_e37.g1_.y * _e40.g0_.y)) - (_e45.g1_.z * _e48.g0_.z)));
}

fn motor_motor_inner_anti_product(self_1214: Motor, other_996: Motor) -> Motor {
    var self_1215: Motor;
    var other_997: Motor;

    self_1215 = self_1214;
    other_997 = other_996;
    let _e4: Motor = self_1215;
    let _e8: Motor = other_997;
    let _e19: Motor = self_1215;
    let _e23: Motor = other_997;
    let _e35: Motor = self_1215;
    let _e39: Motor = other_997;
    let _e43: Motor = self_1215;
    let _e47: Motor = other_997;
    let _e59: Motor = self_1215;
    let _e63: Motor = other_997;
    let _e66: Motor = self_1215;
    let _e68: Motor = other_997;
    return Motor((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + (vec4<f32>(_e35.g0_.w) * _e39.g0_)) + ((vec4<f32>(_e43.g0_.x) * _e47.g0_.wxxx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))), ((vec3<f32>(_e59.g0_.w) * _e63.g1_) + (_e66.g1_ * vec3<f32>(_e68.g0_.w))));
}

fn motor_motor_left_anti_contraction(self_1216: Motor, other_998: Motor) -> Motor {
    var self_1217: Motor;
    var other_999: Motor;

    self_1217 = self_1216;
    other_999 = other_998;
    let _e4: Motor = self_1217;
    let _e8: Motor = other_999;
    let _e20: Motor = self_1217;
    let _e24: Motor = other_999;
    let _e37: Motor = self_1217;
    let _e41: Motor = other_999;
    let _e45: Motor = self_1217;
    let _e49: Motor = other_999;
    let _e62: Motor = self_1217;
    let _e66: Motor = other_999;
    return Motor((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0))) + ((vec4<f32>(_e20.g0_.z) * vec4<f32>(_e24.g0_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + (vec4<f32>(_e37.g0_.w) * _e41.g0_)) + ((vec4<f32>(_e45.g0_.x) * vec4<f32>(_e49.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))), (vec3<f32>(_e62.g0_.w) * _e66.g1_));
}

fn motor_motor_right_anti_contraction(self_1218: Motor, other_1000: Motor) -> Motor {
    var self_1219: Motor;
    var other_1001: Motor;

    self_1219 = self_1218;
    other_1001 = other_1000;
    let _e4: Motor = self_1219;
    let _e8: Motor = other_1001;
    let _e19: Motor = self_1219;
    let _e23: Motor = other_1001;
    let _e35: Motor = self_1219;
    let _e39: Motor = other_1001;
    let _e51: Motor = self_1219;
    let _e55: Motor = other_1001;
    let _e67: Motor = self_1219;
    let _e69: Motor = other_1001;
    return Motor((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e35.g0_.w) * vec4<f32>(_e39.g0_.w)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e51.g0_.x) * _e55.g0_.wxxx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))), (_e67.g1_ * vec3<f32>(_e69.g0_.w)));
}

fn motor_motor_scalar_product(self_1220: Motor, other_1002: Motor) -> Scalar {
    var self_1221: Motor;
    var other_1003: Motor;

    self_1221 = self_1220;
    other_1003 = other_1002;
    let _e5: Motor = self_1221;
    let _e8: Motor = other_1003;
    let _e13: Motor = self_1221;
    let _e16: Motor = other_1003;
    let _e21: Motor = self_1221;
    let _e24: Motor = other_1003;
    return Scalar((((0.0 - (_e5.g1_.x * _e8.g1_.x)) - (_e13.g1_.y * _e16.g1_.y)) - (_e21.g1_.z * _e24.g1_.z)));
}

fn motor_motor_anti_scalar_product(self_1222: Motor, other_1004: Motor) -> AntiScalar {
    var self_1223: Motor;
    var other_1005: Motor;

    self_1223 = self_1222;
    other_1005 = other_1004;
    let _e5: Motor = self_1223;
    let _e8: Motor = other_1005;
    let _e13: Motor = self_1223;
    let _e16: Motor = other_1005;
    let _e21: Motor = self_1223;
    let _e24: Motor = other_1005;
    let _e29: Motor = self_1223;
    let _e32: Motor = other_1005;
    return AntiScalar(((((0.0 - (_e5.g0_.x * _e8.g0_.x)) - (_e13.g0_.y * _e16.g0_.y)) - (_e21.g0_.z * _e24.g0_.z)) + (_e29.g0_.w * _e32.g0_.w)));
}

fn motor_rotor_into(self_1224: Motor) -> Rotor {
    var self_1225: Motor;

    self_1225 = self_1224;
    let _e2: Motor = self_1225;
    return Rotor(_e2.g0_);
}

fn motor_rotor_add(self_1226: Motor, other_1006: Rotor) -> Motor {
    var self_1227: Motor;
    var other_1007: Rotor;

    self_1227 = self_1226;
    other_1007 = other_1006;
    let _e4: Motor = self_1227;
    let _e6: Rotor = other_1007;
    let _e9: Motor = self_1227;
    return Motor((_e4.g0_ + _e6.g0_), _e9.g1_);
}

fn motor_rotor_sub(self_1228: Motor, other_1008: Rotor) -> Motor {
    var self_1229: Motor;
    var other_1009: Rotor;

    self_1229 = self_1228;
    other_1009 = other_1008;
    let _e4: Motor = self_1229;
    let _e6: Rotor = other_1009;
    let _e9: Motor = self_1229;
    return Motor((_e4.g0_ - _e6.g0_), _e9.g1_);
}

fn motor_rotor_geometric_product(self_1230: Motor, other_1010: Rotor) -> Rotor {
    var self_1231: Motor;
    var other_1011: Rotor;

    self_1231 = self_1230;
    other_1011 = other_1010;
    let _e4: Motor = self_1231;
    let _e8: Rotor = other_1011;
    let _e20: Motor = self_1231;
    let _e24: Rotor = other_1011;
    let _e37: Motor = self_1231;
    let _e41: Rotor = other_1011;
    return Rotor(((((vec4<f32>(_e4.g1_.x) * _e8.g0_.wzyx) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0))) + ((vec4<f32>(_e20.g1_.y) * _e24.g0_.zwxy) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e37.g1_.z) * _e41.g0_.yxwz) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))));
}

fn motor_rotor_outer_product(self_1232: Motor, other_1012: Rotor) -> AntiScalar {
    var self_1233: Motor;
    var other_1013: Rotor;

    self_1233 = self_1232;
    other_1013 = other_1012;
    let _e5: Motor = self_1233;
    let _e8: Rotor = other_1013;
    let _e13: Motor = self_1233;
    let _e16: Rotor = other_1013;
    let _e21: Motor = self_1233;
    let _e24: Rotor = other_1013;
    return AntiScalar((((0.0 - (_e5.g1_.x * _e8.g0_.x)) - (_e13.g1_.y * _e16.g0_.y)) - (_e21.g1_.z * _e24.g0_.z)));
}

fn motor_rotor_inner_anti_product(self_1234: Motor, other_1014: Rotor) -> Motor {
    var self_1235: Motor;
    var other_1015: Rotor;

    self_1235 = self_1234;
    other_1015 = other_1014;
    let _e4: Motor = self_1235;
    let _e8: Rotor = other_1015;
    let _e19: Motor = self_1235;
    let _e23: Rotor = other_1015;
    let _e35: Motor = self_1235;
    let _e39: Rotor = other_1015;
    let _e43: Motor = self_1235;
    let _e47: Rotor = other_1015;
    let _e59: Motor = self_1235;
    let _e61: Rotor = other_1015;
    return Motor((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + (vec4<f32>(_e35.g0_.w) * _e39.g0_)) + ((vec4<f32>(_e43.g0_.x) * _e47.g0_.wxxx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))), (_e59.g1_ * vec3<f32>(_e61.g0_.w)));
}

fn motor_rotor_left_anti_contraction(self_1236: Motor, other_1016: Rotor) -> Rotor {
    var self_1237: Motor;
    var other_1017: Rotor;

    self_1237 = self_1236;
    other_1017 = other_1016;
    let _e4: Motor = self_1237;
    let _e8: Rotor = other_1017;
    let _e20: Motor = self_1237;
    let _e24: Rotor = other_1017;
    let _e37: Motor = self_1237;
    let _e41: Rotor = other_1017;
    let _e45: Motor = self_1237;
    let _e49: Rotor = other_1017;
    return Rotor((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0))) + ((vec4<f32>(_e20.g0_.z) * vec4<f32>(_e24.g0_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + (vec4<f32>(_e37.g0_.w) * _e41.g0_)) + ((vec4<f32>(_e45.g0_.x) * vec4<f32>(_e49.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))));
}

fn motor_rotor_right_anti_contraction(self_1238: Motor, other_1018: Rotor) -> Motor {
    var self_1239: Motor;
    var other_1019: Rotor;

    self_1239 = self_1238;
    other_1019 = other_1018;
    let _e4: Motor = self_1239;
    let _e8: Rotor = other_1019;
    let _e19: Motor = self_1239;
    let _e23: Rotor = other_1019;
    let _e35: Motor = self_1239;
    let _e39: Rotor = other_1019;
    let _e51: Motor = self_1239;
    let _e55: Rotor = other_1019;
    let _e67: Motor = self_1239;
    let _e69: Rotor = other_1019;
    return Motor((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e35.g0_.w) * vec4<f32>(_e39.g0_.w)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e51.g0_.x) * _e55.g0_.wxxx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))), (_e67.g1_ * vec3<f32>(_e69.g0_.w)));
}

fn motor_rotor_anti_scalar_product(self_1240: Motor, other_1020: Rotor) -> AntiScalar {
    var self_1241: Motor;
    var other_1021: Rotor;

    self_1241 = self_1240;
    other_1021 = other_1020;
    let _e5: Motor = self_1241;
    let _e8: Rotor = other_1021;
    let _e13: Motor = self_1241;
    let _e16: Rotor = other_1021;
    let _e21: Motor = self_1241;
    let _e24: Rotor = other_1021;
    let _e29: Motor = self_1241;
    let _e32: Rotor = other_1021;
    return AntiScalar(((((0.0 - (_e5.g0_.x * _e8.g0_.x)) - (_e13.g0_.y * _e16.g0_.y)) - (_e21.g0_.z * _e24.g0_.z)) + (_e29.g0_.w * _e32.g0_.w)));
}

fn motor_translator_into(self_1242: Motor) -> Translator {
    var self_1243: Motor;

    self_1243 = self_1242;
    let _e2: Motor = self_1243;
    let _e5: Motor = self_1243;
    let _e8: Motor = self_1243;
    let _e11: Motor = self_1243;
    return Translator(vec4<f32>(_e2.g1_.x, _e5.g1_.y, _e8.g1_.z, _e11.g0_.w));
}

fn motor_translator_add(self_1244: Motor, other_1022: Translator) -> Motor {
    var self_1245: Motor;
    var other_1023: Translator;

    self_1245 = self_1244;
    other_1023 = other_1022;
    let _e4: Motor = self_1245;
    let _e6: Translator = other_1023;
    let _e16: Motor = self_1245;
    let _e18: Translator = other_1023;
    let _e21: Translator = other_1023;
    let _e24: Translator = other_1023;
    return Motor((_e4.g0_ + (_e6.g0_.xxxw * vec4<f32>(0.0, 0.0, 0.0, 1.0))), (_e16.g1_ + vec3<f32>(_e18.g0_.x, _e21.g0_.y, _e24.g0_.z)));
}

fn motor_translator_sub(self_1246: Motor, other_1024: Translator) -> Motor {
    var self_1247: Motor;
    var other_1025: Translator;

    self_1247 = self_1246;
    other_1025 = other_1024;
    let _e4: Motor = self_1247;
    let _e6: Translator = other_1025;
    let _e16: Motor = self_1247;
    let _e18: Translator = other_1025;
    let _e21: Translator = other_1025;
    let _e24: Translator = other_1025;
    return Motor((_e4.g0_ - (_e6.g0_.xxxw * vec4<f32>(0.0, 0.0, 0.0, 1.0))), (_e16.g1_ - vec3<f32>(_e18.g0_.x, _e21.g0_.y, _e24.g0_.z)));
}

fn motor_translator_outer_product(self_1248: Motor, other_1026: Translator) -> AntiScalar {
    var self_1249: Motor;
    var other_1027: Translator;

    self_1249 = self_1248;
    other_1027 = other_1026;
    let _e5: Motor = self_1249;
    let _e8: Translator = other_1027;
    let _e13: Motor = self_1249;
    let _e16: Translator = other_1027;
    let _e21: Motor = self_1249;
    let _e24: Translator = other_1027;
    return AntiScalar((((0.0 - (_e5.g0_.x * _e8.g0_.x)) - (_e13.g0_.y * _e16.g0_.y)) - (_e21.g0_.z * _e24.g0_.z)));
}

fn motor_translator_inner_anti_product(self_1250: Motor, other_1028: Translator) -> Motor {
    var self_1251: Motor;
    var other_1029: Translator;

    self_1251 = self_1250;
    other_1029 = other_1028;
    let _e4: Motor = self_1251;
    let _e6: Translator = other_1029;
    let _e11: Motor = self_1251;
    let _e15: Translator = other_1029;
    let _e18: Translator = other_1029;
    let _e21: Translator = other_1029;
    let _e26: Motor = self_1251;
    let _e28: Translator = other_1029;
    return Motor((_e4.g0_ * vec4<f32>(_e6.g0_.w)), ((vec3<f32>(_e11.g0_.w) * vec3<f32>(_e15.g0_.x, _e18.g0_.y, _e21.g0_.z)) + (_e26.g1_ * vec3<f32>(_e28.g0_.w))));
}

fn motor_translator_left_anti_contraction(self_1252: Motor, other_1030: Translator) -> Translator {
    var self_1253: Motor;
    var other_1031: Translator;

    self_1253 = self_1252;
    other_1031 = other_1030;
    let _e4: Motor = self_1253;
    let _e8: Translator = other_1031;
    return Translator((vec4<f32>(_e4.g0_.w) * _e8.g0_));
}

fn motor_translator_right_anti_contraction(self_1254: Motor, other_1032: Translator) -> Motor {
    var self_1255: Motor;
    var other_1033: Translator;

    self_1255 = self_1254;
    other_1033 = other_1032;
    let _e4: Motor = self_1255;
    let _e6: Translator = other_1033;
    let _e11: Motor = self_1255;
    let _e13: Translator = other_1033;
    return Motor((_e4.g0_ * vec4<f32>(_e6.g0_.w)), (_e11.g1_ * vec3<f32>(_e13.g0_.w)));
}

fn motor_translator_scalar_product(self_1256: Motor, other_1034: Translator) -> Scalar {
    var self_1257: Motor;
    var other_1035: Translator;

    self_1257 = self_1256;
    other_1035 = other_1034;
    let _e5: Motor = self_1257;
    let _e8: Translator = other_1035;
    let _e13: Motor = self_1257;
    let _e16: Translator = other_1035;
    let _e21: Motor = self_1257;
    let _e24: Translator = other_1035;
    return Scalar((((0.0 - (_e5.g1_.x * _e8.g0_.x)) - (_e13.g1_.y * _e16.g0_.y)) - (_e21.g1_.z * _e24.g0_.z)));
}

fn motor_translator_anti_scalar_product(self_1258: Motor, other_1036: Translator) -> AntiScalar {
    var self_1259: Motor;
    var other_1037: Translator;

    self_1259 = self_1258;
    other_1037 = other_1036;
    let _e4: Motor = self_1259;
    let _e7: Translator = other_1037;
    return AntiScalar((_e4.g0_.w * _e7.g0_.w));
}

fn motor_flector_geometric_product(self_1260: Motor, other_1038: Flector) -> Flector {
    var self_1261: Motor;
    var other_1039: Flector;

    self_1261 = self_1260;
    other_1039 = other_1038;
    let _e4: Motor = self_1261;
    let _e8: Flector = other_1039;
    let _e19: Motor = self_1261;
    let _e23: Flector = other_1039;
    let _e35: Motor = self_1261;
    let _e39: Flector = other_1039;
    let _e52: Motor = self_1261;
    let _e56: Flector = other_1039;
    let _e59: Flector = other_1039;
    let _e62: Flector = other_1039;
    let _e65: Flector = other_1039;
    let _e79: Motor = self_1261;
    let _e83: Flector = other_1039;
    let _e86: Flector = other_1039;
    let _e89: Flector = other_1039;
    let _e92: Flector = other_1039;
    let _e106: Motor = self_1261;
    let _e110: Flector = other_1039;
    let _e113: Flector = other_1039;
    let _e116: Flector = other_1039;
    let _e119: Flector = other_1039;
    let _e133: Motor = self_1261;
    let _e137: Flector = other_1039;
    let _e149: Motor = self_1261;
    let _e153: Flector = other_1039;
    let _e156: Flector = other_1039;
    let _e159: Flector = other_1039;
    let _e162: Flector = other_1039;
    let _e175: Motor = self_1261;
    let _e179: Flector = other_1039;
    let _e182: Flector = other_1039;
    let _e185: Flector = other_1039;
    let _e188: Flector = other_1039;
    let _e202: Motor = self_1261;
    let _e206: Flector = other_1039;
    let _e220: Motor = self_1261;
    let _e224: Flector = other_1039;
    let _e227: Flector = other_1039;
    let _e230: Flector = other_1039;
    let _e233: Flector = other_1039;
    let _e247: Motor = self_1261;
    let _e251: Flector = other_1039;
    let _e254: Flector = other_1039;
    let _e257: Flector = other_1039;
    let _e260: Flector = other_1039;
    let _e274: Motor = self_1261;
    let _e278: Flector = other_1039;
    let _e281: Flector = other_1039;
    let _e284: Flector = other_1039;
    let _e287: Flector = other_1039;
    let _e301: Motor = self_1261;
    let _e305: Flector = other_1039;
    let _e308: Flector = other_1039;
    let _e311: Flector = other_1039;
    let _e314: Flector = other_1039;
    return Flector(((((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y)) * vec4<f32>(0.0, 0.0, 0.0, 1.0)) + ((vec4<f32>(_e19.g0_.z) * vec4<f32>(_e23.g0_.z)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e35.g0_.w) * vec4<f32>(_e39.g1_.w)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e52.g1_.x) * vec4<f32>(_e56.g1_.w, _e59.g0_.z, _e62.g0_.y, _e65.g1_.x)) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e79.g1_.y) * vec4<f32>(_e83.g0_.z, _e86.g1_.w, _e89.g0_.x, _e92.g1_.y)) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e106.g1_.z) * vec4<f32>(_e110.g0_.y, _e113.g0_.x, _e116.g1_.w, _e119.g1_.z)) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e133.g0_.x) * vec4<f32>(_e137.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))), ((((((((vec4<f32>(_e149.g0_.y) * vec4<f32>(_e153.g0_.z, _e156.g1_.w, _e159.g0_.x, _e162.g0_.z)) * vec4<f32>(1.0, -(1.0), -(1.0), 0.0)) + ((vec4<f32>(_e175.g0_.z) * vec4<f32>(_e179.g0_.y, _e182.g0_.x, _e185.g1_.w, _e188.g0_.y)) * vec4<f32>(-(1.0), 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e202.g0_.w) * _e206.g0_.xyzx) * vec4<f32>(-(1.0), -(1.0), -(1.0), 0.0))) + ((vec4<f32>(_e220.g1_.x) * vec4<f32>(_e224.g0_.w, _e227.g1_.z, _e230.g1_.y, _e233.g0_.x)) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e247.g1_.y) * vec4<f32>(_e251.g1_.z, _e254.g0_.w, _e257.g1_.x, _e260.g0_.y)) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e274.g1_.z) * vec4<f32>(_e278.g1_.y, _e281.g1_.x, _e284.g0_.w, _e287.g0_.z)) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e301.g0_.x) * vec4<f32>(_e305.g1_.w, _e308.g0_.z, _e311.g0_.y, _e314.g1_.x)) * vec4<f32>(-(1.0), -(1.0), 1.0, 0.0))));
}

fn motor_flector_regressive_product(self_1262: Motor, other_1040: Flector) -> Flector {
    var self_1263: Motor;
    var other_1041: Flector;

    self_1263 = self_1262;
    other_1041 = other_1040;
    let _e4: Motor = self_1263;
    let _e8: Flector = other_1041;
    let _e19: Motor = self_1263;
    let _e23: Flector = other_1041;
    let _e35: Motor = self_1263;
    let _e39: Flector = other_1041;
    let _e43: Motor = self_1263;
    let _e47: Flector = other_1041;
    let _e59: Motor = self_1263;
    let _e63: Flector = other_1041;
    let _e75: Motor = self_1263;
    let _e79: Flector = other_1041;
    let _e91: Motor = self_1263;
    let _e95: Flector = other_1041;
    let _e107: Motor = self_1263;
    let _e111: Flector = other_1041;
    return Flector(((((((((vec4<f32>(_e4.g0_.y) * _e8.g1_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e19.g0_.z) * _e23.g1_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + (vec4<f32>(_e35.g0_.w) * _e39.g0_)) + ((vec4<f32>(_e43.g1_.x) * _e47.g1_.zzyz) * vec4<f32>(0.0, -(1.0), 1.0, 0.0))) + ((vec4<f32>(_e59.g1_.y) * _e63.g1_.zzxz) * vec4<f32>(1.0, 0.0, -(1.0), 0.0))) + ((vec4<f32>(_e75.g1_.z) * _e79.g1_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))) + ((vec4<f32>(_e91.g0_.x) * _e95.g1_.wxxx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))), (vec4<f32>(_e107.g0_.w) * _e111.g1_));
}

fn motor_flector_outer_product(self_1264: Motor, other_1042: Flector) -> Plane {
    var self_1265: Motor;
    var other_1043: Flector;

    self_1265 = self_1264;
    other_1043 = other_1042;
    let _e4: Motor = self_1265;
    let _e8: Flector = other_1043;
    let _e19: Motor = self_1265;
    let _e23: Flector = other_1043;
    let _e35: Motor = self_1265;
    let _e39: Flector = other_1043;
    let _e51: Motor = self_1265;
    let _e55: Flector = other_1043;
    let _e67: Motor = self_1265;
    let _e71: Flector = other_1043;
    let _e83: Motor = self_1265;
    let _e87: Flector = other_1043;
    return Plane((((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.zzxz) * vec4<f32>(1.0, 0.0, -(1.0), 0.0)) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))) + ((vec4<f32>(_e35.g1_.x) * _e39.g0_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e51.g1_.y) * _e55.g0_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0)))) + ((vec4<f32>(_e67.g1_.z) * _e71.g0_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e83.g0_.x) * _e87.g0_.xzyx) * vec4<f32>(0.0, -(1.0), 1.0, 0.0))));
}

fn motor_flector_geometric_anti_product(self_1266: Motor, other_1044: Flector) -> Flector {
    var self_1267: Motor;
    var other_1045: Flector;

    self_1267 = self_1266;
    other_1045 = other_1044;
    let _e4: Motor = self_1267;
    let _e8: Flector = other_1045;
    let _e11: Flector = other_1045;
    let _e14: Flector = other_1045;
    let _e17: Flector = other_1045;
    let _e30: Motor = self_1267;
    let _e34: Flector = other_1045;
    let _e37: Flector = other_1045;
    let _e40: Flector = other_1045;
    let _e43: Flector = other_1045;
    let _e57: Motor = self_1267;
    let _e61: Flector = other_1045;
    let _e64: Flector = other_1045;
    let _e67: Flector = other_1045;
    let _e70: Flector = other_1045;
    let _e84: Motor = self_1267;
    let _e88: Flector = other_1045;
    let _e92: Motor = self_1267;
    let _e96: Flector = other_1045;
    let _e99: Flector = other_1045;
    let _e102: Flector = other_1045;
    let _e105: Flector = other_1045;
    let _e118: Motor = self_1267;
    let _e122: Flector = other_1045;
    let _e125: Flector = other_1045;
    let _e128: Flector = other_1045;
    let _e131: Flector = other_1045;
    let _e144: Motor = self_1267;
    let _e148: Flector = other_1045;
    let _e151: Flector = other_1045;
    let _e154: Flector = other_1045;
    let _e157: Flector = other_1045;
    let _e170: Motor = self_1267;
    let _e174: Flector = other_1045;
    let _e177: Flector = other_1045;
    let _e180: Flector = other_1045;
    let _e183: Flector = other_1045;
    let _e196: Motor = self_1267;
    let _e200: Flector = other_1045;
    let _e203: Flector = other_1045;
    let _e206: Flector = other_1045;
    let _e209: Flector = other_1045;
    let _e223: Motor = self_1267;
    let _e227: Flector = other_1045;
    let _e230: Flector = other_1045;
    let _e233: Flector = other_1045;
    let _e236: Flector = other_1045;
    let _e250: Motor = self_1267;
    let _e254: Flector = other_1045;
    let _e258: Motor = self_1267;
    let _e262: Flector = other_1045;
    let _e275: Motor = self_1267;
    let _e279: Flector = other_1045;
    let _e292: Motor = self_1267;
    let _e296: Flector = other_1045;
    return Flector(((((((((vec4<f32>(_e4.g0_.x) * vec4<f32>(_e8.g1_.w, _e11.g0_.z, _e14.g0_.y, _e17.g1_.x)) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0))) + ((vec4<f32>(_e30.g0_.y) * vec4<f32>(_e34.g0_.z, _e37.g1_.w, _e40.g0_.x, _e43.g1_.y)) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e57.g0_.z) * vec4<f32>(_e61.g0_.y, _e64.g0_.x, _e67.g1_.w, _e70.g1_.z)) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + (vec4<f32>(_e84.g0_.w) * _e88.g0_)) + ((vec4<f32>(_e92.g1_.y) * vec4<f32>(_e96.g1_.z, _e99.g0_.w, _e102.g1_.x, _e105.g1_.z)) * vec4<f32>(1.0, 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e118.g1_.z) * vec4<f32>(_e122.g1_.y, _e125.g1_.x, _e128.g0_.w, _e131.g1_.y)) * vec4<f32>(-(1.0), 1.0, 1.0, 0.0))) + ((vec4<f32>(_e144.g1_.x) * vec4<f32>(_e148.g0_.w, _e151.g1_.z, _e154.g1_.y, _e157.g0_.x)) * vec4<f32>(1.0, -(1.0), 1.0, 0.0))), ((((((((vec4<f32>(_e170.g0_.x) * vec4<f32>(_e174.g0_.w, _e177.g1_.z, _e180.g1_.y, _e183.g0_.x)) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0))) + ((vec4<f32>(_e196.g0_.y) * vec4<f32>(_e200.g1_.z, _e203.g0_.w, _e206.g1_.x, _e209.g0_.y)) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e223.g0_.z) * vec4<f32>(_e227.g1_.y, _e230.g1_.x, _e233.g0_.w, _e236.g0_.z)) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + (vec4<f32>(_e250.g0_.w) * _e254.g1_)) + ((vec4<f32>(_e258.g1_.y) * vec4<f32>(_e262.g1_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e275.g1_.z) * vec4<f32>(_e279.g1_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e292.g1_.x) * vec4<f32>(_e296.g1_.x)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))));
}

fn motor_flector_inner_anti_product(self_1268: Motor, other_1046: Flector) -> Flector {
    var self_1269: Motor;
    var other_1047: Flector;

    self_1269 = self_1268;
    other_1047 = other_1046;
    let _e4: Motor = self_1269;
    let _e8: Flector = other_1047;
    let _e11: Motor = self_1269;
    let _e15: Flector = other_1047;
    let _e18: Flector = other_1047;
    let _e21: Flector = other_1047;
    let _e24: Flector = other_1047;
    let _e37: Motor = self_1269;
    let _e41: Flector = other_1047;
    let _e44: Flector = other_1047;
    let _e47: Flector = other_1047;
    let _e50: Flector = other_1047;
    let _e64: Motor = self_1269;
    let _e68: Flector = other_1047;
    let _e71: Flector = other_1047;
    let _e74: Flector = other_1047;
    let _e77: Flector = other_1047;
    let _e91: Motor = self_1269;
    let _e95: Flector = other_1047;
    let _e99: Motor = self_1269;
    let _e103: Flector = other_1047;
    let _e116: Motor = self_1269;
    let _e120: Flector = other_1047;
    let _e133: Motor = self_1269;
    let _e137: Flector = other_1047;
    return Flector((vec4<f32>(_e4.g0_.w) * _e8.g0_), ((((((((vec4<f32>(_e11.g0_.x) * vec4<f32>(_e15.g0_.w, _e18.g1_.z, _e21.g1_.y, _e24.g0_.x)) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0))) + ((vec4<f32>(_e37.g0_.y) * vec4<f32>(_e41.g1_.z, _e44.g0_.w, _e47.g1_.x, _e50.g0_.y)) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e64.g0_.z) * vec4<f32>(_e68.g1_.y, _e71.g1_.x, _e74.g0_.w, _e77.g0_.z)) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + (vec4<f32>(_e91.g0_.w) * _e95.g1_)) + ((vec4<f32>(_e99.g1_.y) * vec4<f32>(_e103.g1_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e116.g1_.z) * vec4<f32>(_e120.g1_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e133.g1_.x) * vec4<f32>(_e137.g1_.x)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))));
}

fn motor_flector_left_contraction(self_1270: Motor, other_1048: Flector) -> Point {
    var self_1271: Motor;
    var other_1049: Flector;

    self_1271 = self_1270;
    other_1049 = other_1048;
    let _e4: Motor = self_1271;
    let _e8: Flector = other_1049;
    let _e19: Motor = self_1271;
    let _e23: Flector = other_1049;
    let _e35: Motor = self_1271;
    let _e39: Flector = other_1049;
    return Point(((((vec4<f32>(_e4.g1_.y) * _e8.g1_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e19.g1_.z) * _e23.g1_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e35.g1_.x) * _e39.g1_.wxxx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))));
}

fn motor_flector_left_anti_contraction(self_1272: Motor, other_1050: Flector) -> Flector {
    var self_1273: Motor;
    var other_1051: Flector;

    self_1273 = self_1272;
    other_1051 = other_1050;
    let _e4: Motor = self_1273;
    let _e8: Flector = other_1051;
    let _e11: Motor = self_1273;
    let _e15: Flector = other_1051;
    let _e26: Motor = self_1273;
    let _e30: Flector = other_1051;
    let _e42: Motor = self_1273;
    let _e46: Flector = other_1051;
    let _e50: Motor = self_1273;
    let _e54: Flector = other_1051;
    return Flector((vec4<f32>(_e4.g0_.w) * _e8.g0_), (((((vec4<f32>(_e11.g0_.y) * _e15.g0_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e26.g0_.z) * _e30.g0_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + (vec4<f32>(_e42.g0_.w) * _e46.g1_)) + ((vec4<f32>(_e50.g0_.x) * _e54.g0_.wxxx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))));
}

fn motor_flector_right_anti_contraction(self_1274: Motor, other_1052: Flector) -> Plane {
    var self_1275: Motor;
    var other_1053: Flector;

    self_1275 = self_1274;
    other_1053 = other_1052;
    let _e4: Motor = self_1275;
    let _e8: Flector = other_1053;
    let _e19: Motor = self_1275;
    let _e23: Flector = other_1053;
    let _e35: Motor = self_1275;
    let _e39: Flector = other_1053;
    let _e52: Motor = self_1275;
    let _e56: Flector = other_1053;
    let _e69: Motor = self_1275;
    let _e73: Flector = other_1053;
    let _e86: Motor = self_1275;
    let _e90: Flector = other_1053;
    return Plane((((((((vec4<f32>(_e4.g0_.y) * _e8.g1_.zzxz) * vec4<f32>(1.0, 0.0, -(1.0), 0.0)) + ((vec4<f32>(_e19.g0_.z) * _e23.g1_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))) + ((vec4<f32>(_e35.g1_.x) * vec4<f32>(_e39.g1_.x)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e52.g1_.y) * vec4<f32>(_e56.g1_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e69.g1_.z) * vec4<f32>(_e73.g1_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e86.g0_.x) * _e90.g1_.xzyx) * vec4<f32>(0.0, -(1.0), 1.0, 0.0))));
}

fn motor_multi_vector_scalar_product(self_1276: Motor, other_1054: MultiVector) -> Scalar {
    var self_1277: Motor;
    var other_1055: MultiVector;

    self_1277 = self_1276;
    other_1055 = other_1054;
    let _e5: Motor = self_1277;
    let _e8: MultiVector = other_1055;
    let _e13: Motor = self_1277;
    let _e16: MultiVector = other_1055;
    let _e21: Motor = self_1277;
    let _e24: MultiVector = other_1055;
    return Scalar((((0.0 - (_e5.g1_.x * _e8.g3_.x)) - (_e13.g1_.y * _e16.g3_.y)) - (_e21.g1_.z * _e24.g3_.z)));
}

fn motor_multi_vector_anti_scalar_product(self_1278: Motor, other_1056: MultiVector) -> AntiScalar {
    var self_1279: Motor;
    var other_1057: MultiVector;

    self_1279 = self_1278;
    other_1057 = other_1056;
    let _e5: Motor = self_1279;
    let _e8: MultiVector = other_1057;
    let _e13: Motor = self_1279;
    let _e16: MultiVector = other_1057;
    let _e21: Motor = self_1279;
    let _e24: MultiVector = other_1057;
    let _e29: Motor = self_1279;
    let _e32: MultiVector = other_1057;
    return AntiScalar(((((0.0 - (_e5.g0_.x * _e8.g2_.x)) - (_e13.g0_.y * _e16.g2_.y)) - (_e21.g0_.z * _e24.g2_.z)) + (_e29.g0_.w * _e32.g0_.y)));
}

fn motor_squared_magnitude(self_1280: Motor) -> Scalar {
    var self_1281: Motor;

    self_1281 = self_1280;
    let _e2: Motor = self_1281;
    let _e3: Motor = self_1281;
    let _e4: Motor = motor_reversal(_e3);
    let _e5: Scalar = motor_motor_scalar_product(_e2, _e4);
    return _e5;
}

fn motor_magnitude(self_1282: Motor) -> Scalar {
    var self_1283: Motor;

    self_1283 = self_1282;
    let _e2: Motor = self_1283;
    let _e3: Scalar = motor_squared_magnitude(_e2);
    return Scalar(sqrt(_e3.g0_));
}

fn motor_bulk_norm(self_1284: Motor) -> Scalar {
    var self_1285: Motor;

    self_1285 = self_1284;
    let _e2: Motor = self_1285;
    let _e3: Scalar = motor_squared_magnitude(_e2);
    return Scalar(sqrt(_e3.g0_));
}

fn motor_squared_anti_magnitude(self_1286: Motor) -> AntiScalar {
    var self_1287: Motor;

    self_1287 = self_1286;
    let _e2: Motor = self_1287;
    let _e3: Motor = self_1287;
    let _e4: Motor = motor_anti_reversal(_e3);
    let _e5: AntiScalar = motor_motor_anti_scalar_product(_e2, _e4);
    return _e5;
}

fn motor_weight_norm(self_1288: Motor) -> AntiScalar {
    var self_1289: Motor;

    self_1289 = self_1288;
    let _e2: Motor = self_1289;
    let _e3: AntiScalar = motor_squared_anti_magnitude(_e2);
    return AntiScalar(sqrt(_e3.g0_));
}

fn motor_geometric_norm(self_1290: Motor) -> HomogeneousMagnitude {
    var self_1291: Motor;

    self_1291 = self_1290;
    let _e2: Motor = self_1291;
    let _e3: Scalar = motor_bulk_norm(_e2);
    let _e4: Motor = self_1291;
    let _e5: AntiScalar = motor_weight_norm(_e4);
    return (_e3 + _e5);
}

fn motor_scale(self_1292: Motor, other_1058: f32) -> Motor {
    var self_1293: Motor;
    var other_1059: f32;

    self_1293 = self_1292;
    other_1059 = other_1058;
    let _e4: Motor = self_1293;
    let _e5: f32 = other_1059;
    let _e7: Motor = motor_scalar_geometric_product(_e4, Scalar(_e5));
    return _e7;
}

fn motor_signum(self_1294: Motor) -> Motor {
    var self_1295: Motor;

    self_1295 = self_1294;
    let _e2: Motor = self_1295;
    let _e3: Motor = self_1295;
    let _e4: Scalar = motor_magnitude(_e3);
    let _e9: Motor = motor_scalar_geometric_product(_e2, Scalar((1.0 / _e4.g0_)));
    return _e9;
}

fn motor_inverse(self_1296: Motor) -> Motor {
    var self_1297: Motor;

    self_1297 = self_1296;
    let _e2: Motor = self_1297;
    let _e3: Motor = motor_reversal(_e2);
    let _e4: Motor = self_1297;
    let _e5: Scalar = motor_squared_magnitude(_e4);
    let _e10: Motor = motor_scalar_geometric_product(_e3, Scalar((1.0 / _e5.g0_)));
    return _e10;
}

fn motor_unitize(self_1298: Motor) -> Motor {
    var self_1299: Motor;

    self_1299 = self_1298;
    let _e2: Motor = self_1299;
    let _e3: Motor = self_1299;
    let _e4: AntiScalar = motor_weight_norm(_e3);
    let _e9: Motor = motor_scalar_geometric_product(_e2, Scalar((1.0 / _e4.g0_)));
    return _e9;
}

fn rotor_zero() -> Rotor {
    return Rotor(vec4<f32>(0.0));
}

fn rotor_one() -> Rotor {
    return Rotor(vec4<f32>(0.0));
}

fn rotor_neg(self_1300: Rotor) -> Rotor {
    var self_1301: Rotor;

    self_1301 = self_1300;
    let _e2: Rotor = self_1301;
    return Rotor((_e2.g0_ * vec4<f32>(-(1.0))));
}

fn rotor_automorphism(self_1302: Rotor) -> Rotor {
    var self_1303: Rotor;

    self_1303 = self_1302;
    let _e2: Rotor = self_1303;
    return Rotor(_e2.g0_);
}

fn rotor_reversal(self_1304: Rotor) -> Rotor {
    var self_1305: Rotor;

    self_1305 = self_1304;
    let _e2: Rotor = self_1305;
    return Rotor((_e2.g0_ * vec4<f32>(-(1.0), -(1.0), -(1.0), 1.0)));
}

fn rotor_conjugation(self_1306: Rotor) -> Rotor {
    var self_1307: Rotor;

    self_1307 = self_1306;
    let _e2: Rotor = self_1307;
    return Rotor((_e2.g0_ * vec4<f32>(-(1.0), -(1.0), -(1.0), 1.0)));
}

fn rotor_anti_reversal(self_1308: Rotor) -> Rotor {
    var self_1309: Rotor;

    self_1309 = self_1308;
    let _e2: Rotor = self_1309;
    return Rotor((_e2.g0_ * vec4<f32>(-(1.0), -(1.0), -(1.0), 1.0)));
}

fn rotor_double_complement(self_1310: Rotor) -> Rotor {
    var self_1311: Rotor;

    self_1311 = self_1310;
    let _e2: Rotor = self_1311;
    return Rotor(_e2.g0_);
}

fn rotor_scalar_geometric_product(self_1312: Rotor, other_1060: Scalar) -> Rotor {
    var self_1313: Rotor;
    var other_1061: Scalar;

    self_1313 = self_1312;
    other_1061 = other_1060;
    let _e4: Rotor = self_1313;
    let _e6: Scalar = other_1061;
    return Rotor((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn rotor_scalar_regressive_product(self_1314: Rotor, other_1062: Scalar) -> Scalar {
    var self_1315: Rotor;
    var other_1063: Scalar;

    self_1315 = self_1314;
    other_1063 = other_1062;
    let _e4: Rotor = self_1315;
    let _e7: Scalar = other_1063;
    return Scalar((_e4.g0_.w * _e7.g0_));
}

fn rotor_scalar_outer_product(self_1316: Rotor, other_1064: Scalar) -> Rotor {
    var self_1317: Rotor;
    var other_1065: Scalar;

    self_1317 = self_1316;
    other_1065 = other_1064;
    let _e4: Rotor = self_1317;
    let _e6: Scalar = other_1065;
    return Rotor((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn rotor_scalar_inner_product(self_1318: Rotor, other_1066: Scalar) -> Rotor {
    var self_1319: Rotor;
    var other_1067: Scalar;

    self_1319 = self_1318;
    other_1067 = other_1066;
    let _e4: Rotor = self_1319;
    let _e6: Scalar = other_1067;
    return Rotor((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn rotor_scalar_right_contraction(self_1320: Rotor, other_1068: Scalar) -> Rotor {
    var self_1321: Rotor;
    var other_1069: Scalar;

    self_1321 = self_1320;
    other_1069 = other_1068;
    let _e4: Rotor = self_1321;
    let _e6: Scalar = other_1069;
    return Rotor((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn rotor_anti_scalar_into(self_1322: Rotor) -> AntiScalar {
    var self_1323: Rotor;

    self_1323 = self_1322;
    let _e2: Rotor = self_1323;
    return AntiScalar(_e2.g0_.w);
}

fn rotor_anti_scalar_add(self_1324: Rotor, other_1070: AntiScalar) -> Rotor {
    var self_1325: Rotor;
    var other_1071: AntiScalar;

    self_1325 = self_1324;
    other_1071 = other_1070;
    let _e4: Rotor = self_1325;
    let _e6: AntiScalar = other_1071;
    return Rotor((_e4.g0_ + (vec4<f32>(_e6.g0_) * vec4<f32>(0.0, 0.0, 0.0, 1.0))));
}

fn rotor_anti_scalar_sub(self_1326: Rotor, other_1072: AntiScalar) -> Rotor {
    var self_1327: Rotor;
    var other_1073: AntiScalar;

    self_1327 = self_1326;
    other_1073 = other_1072;
    let _e4: Rotor = self_1327;
    let _e6: AntiScalar = other_1073;
    return Rotor((_e4.g0_ - (vec4<f32>(_e6.g0_) * vec4<f32>(0.0, 0.0, 0.0, 1.0))));
}

fn rotor_anti_scalar_regressive_product(self_1328: Rotor, other_1074: AntiScalar) -> Rotor {
    var self_1329: Rotor;
    var other_1075: AntiScalar;

    self_1329 = self_1328;
    other_1075 = other_1074;
    let _e4: Rotor = self_1329;
    let _e6: AntiScalar = other_1075;
    return Rotor((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn rotor_anti_scalar_geometric_anti_product(self_1330: Rotor, other_1076: AntiScalar) -> Rotor {
    var self_1331: Rotor;
    var other_1077: AntiScalar;

    self_1331 = self_1330;
    other_1077 = other_1076;
    let _e4: Rotor = self_1331;
    let _e6: AntiScalar = other_1077;
    return Rotor((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn rotor_anti_scalar_inner_anti_product(self_1332: Rotor, other_1078: AntiScalar) -> Rotor {
    var self_1333: Rotor;
    var other_1079: AntiScalar;

    self_1333 = self_1332;
    other_1079 = other_1078;
    let _e4: Rotor = self_1333;
    let _e6: AntiScalar = other_1079;
    return Rotor((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn rotor_anti_scalar_left_anti_contraction(self_1334: Rotor, other_1080: AntiScalar) -> AntiScalar {
    var self_1335: Rotor;
    var other_1081: AntiScalar;

    self_1335 = self_1334;
    other_1081 = other_1080;
    let _e4: Rotor = self_1335;
    let _e7: AntiScalar = other_1081;
    return AntiScalar((_e4.g0_.w * _e7.g0_));
}

fn rotor_anti_scalar_right_anti_contraction(self_1336: Rotor, other_1082: AntiScalar) -> Rotor {
    var self_1337: Rotor;
    var other_1083: AntiScalar;

    self_1337 = self_1336;
    other_1083 = other_1082;
    let _e4: Rotor = self_1337;
    let _e6: AntiScalar = other_1083;
    return Rotor((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn rotor_anti_scalar_anti_scalar_product(self_1338: Rotor, other_1084: AntiScalar) -> AntiScalar {
    var self_1339: Rotor;
    var other_1085: AntiScalar;

    self_1339 = self_1338;
    other_1085 = other_1084;
    let _e4: Rotor = self_1339;
    let _e7: AntiScalar = other_1085;
    return AntiScalar((_e4.g0_.w * _e7.g0_));
}

fn rotor_homogeneous_magnitude_geometric_product(self_1340: Rotor, other_1086: HomogeneousMagnitude) -> Rotor {
    var self_1341: Rotor;
    var other_1087: HomogeneousMagnitude;

    self_1341 = self_1340;
    other_1087 = other_1086;
    let _e4: Rotor = self_1341;
    let _e6: HomogeneousMagnitude = other_1087;
    return Rotor((_e4.g0_ * vec4<f32>(_e6.g0_.x)));
}

fn rotor_homogeneous_magnitude_outer_product(self_1342: Rotor, other_1088: HomogeneousMagnitude) -> Rotor {
    var self_1343: Rotor;
    var other_1089: HomogeneousMagnitude;

    self_1343 = self_1342;
    other_1089 = other_1088;
    let _e4: Rotor = self_1343;
    let _e6: HomogeneousMagnitude = other_1089;
    return Rotor((_e4.g0_ * vec4<f32>(_e6.g0_.x)));
}

fn rotor_homogeneous_magnitude_inner_product(self_1344: Rotor, other_1090: HomogeneousMagnitude) -> Rotor {
    var self_1345: Rotor;
    var other_1091: HomogeneousMagnitude;

    self_1345 = self_1344;
    other_1091 = other_1090;
    let _e4: Rotor = self_1345;
    let _e6: HomogeneousMagnitude = other_1091;
    return Rotor((_e4.g0_ * vec4<f32>(_e6.g0_.x)));
}

fn rotor_homogeneous_magnitude_right_contraction(self_1346: Rotor, other_1092: HomogeneousMagnitude) -> Rotor {
    var self_1347: Rotor;
    var other_1093: HomogeneousMagnitude;

    self_1347 = self_1346;
    other_1093 = other_1092;
    let _e4: Rotor = self_1347;
    let _e6: HomogeneousMagnitude = other_1093;
    return Rotor((_e4.g0_ * vec4<f32>(_e6.g0_.x)));
}

fn rotor_homogeneous_magnitude_right_anti_contraction(self_1348: Rotor, other_1094: HomogeneousMagnitude) -> Rotor {
    var self_1349: Rotor;
    var other_1095: HomogeneousMagnitude;

    self_1349 = self_1348;
    other_1095 = other_1094;
    let _e4: Rotor = self_1349;
    let _e6: HomogeneousMagnitude = other_1095;
    return Rotor((_e4.g0_ * vec4<f32>(_e6.g0_.y)));
}

fn rotor_homogeneous_magnitude_anti_scalar_product(self_1350: Rotor, other_1096: HomogeneousMagnitude) -> AntiScalar {
    var self_1351: Rotor;
    var other_1097: HomogeneousMagnitude;

    self_1351 = self_1350;
    other_1097 = other_1096;
    let _e4: Rotor = self_1351;
    let _e7: HomogeneousMagnitude = other_1097;
    return AntiScalar((_e4.g0_.w * _e7.g0_.y));
}

fn rotor_point_regressive_product(self_1352: Rotor, other_1098: Point) -> Point {
    var self_1353: Rotor;
    var other_1099: Point;

    self_1353 = self_1352;
    other_1099 = other_1098;
    let _e4: Rotor = self_1353;
    let _e8: Point = other_1099;
    return Point((vec4<f32>(_e4.g0_.w) * _e8.g0_));
}

fn rotor_point_geometric_anti_product(self_1354: Rotor, other_1100: Point) -> Flector {
    var self_1355: Rotor;
    var other_1101: Point;

    self_1355 = self_1354;
    other_1101 = other_1100;
    let _e4: Rotor = self_1355;
    let _e8: Point = other_1101;
    let _e19: Rotor = self_1355;
    let _e23: Point = other_1101;
    let _e35: Rotor = self_1355;
    let _e39: Point = other_1101;
    let _e43: Rotor = self_1355;
    let _e47: Point = other_1101;
    let _e59: Rotor = self_1355;
    let _e63: Point = other_1101;
    let _e74: Rotor = self_1355;
    let _e78: Point = other_1101;
    let _e90: Rotor = self_1355;
    let _e94: Point = other_1101;
    return Flector((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.zzxz) * vec4<f32>(1.0, 0.0, -(1.0), 0.0)) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))) + (vec4<f32>(_e35.g0_.w) * _e39.g0_)) + ((vec4<f32>(_e43.g0_.x) * _e47.g0_.xzyx) * vec4<f32>(0.0, -(1.0), 1.0, 0.0))), ((((vec4<f32>(_e59.g0_.y) * _e63.g0_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e74.g0_.z) * _e78.g0_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e90.g0_.x) * _e94.g0_.wxxx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))));
}

fn rotor_point_inner_anti_product(self_1356: Rotor, other_1102: Point) -> Flector {
    var self_1357: Rotor;
    var other_1103: Point;

    self_1357 = self_1356;
    other_1103 = other_1102;
    let _e4: Rotor = self_1357;
    let _e8: Point = other_1103;
    let _e11: Rotor = self_1357;
    let _e15: Point = other_1103;
    let _e26: Rotor = self_1357;
    let _e30: Point = other_1103;
    let _e42: Rotor = self_1357;
    let _e46: Point = other_1103;
    return Flector((vec4<f32>(_e4.g0_.w) * _e8.g0_), ((((vec4<f32>(_e11.g0_.y) * _e15.g0_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e26.g0_.z) * _e30.g0_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e42.g0_.x) * _e46.g0_.wxxx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))));
}

fn rotor_point_left_anti_contraction(self_1358: Rotor, other_1104: Point) -> Flector {
    var self_1359: Rotor;
    var other_1105: Point;

    self_1359 = self_1358;
    other_1105 = other_1104;
    let _e4: Rotor = self_1359;
    let _e8: Point = other_1105;
    let _e11: Rotor = self_1359;
    let _e15: Point = other_1105;
    let _e26: Rotor = self_1359;
    let _e30: Point = other_1105;
    let _e42: Rotor = self_1359;
    let _e46: Point = other_1105;
    return Flector((vec4<f32>(_e4.g0_.w) * _e8.g0_), ((((vec4<f32>(_e11.g0_.y) * _e15.g0_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e26.g0_.z) * _e30.g0_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e42.g0_.x) * _e46.g0_.wxxx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))));
}

fn rotor_line_add(self_1360: Rotor, other_1106: Line) -> Motor {
    var self_1361: Rotor;
    var other_1107: Line;

    self_1361 = self_1360;
    other_1107 = other_1106;
    let _e4: Rotor = self_1361;
    let _e6: Line = other_1107;
    let _e9: Line = other_1107;
    let _e12: Line = other_1107;
    let _e15: Line = other_1107;
    let _e26: Line = other_1107;
    return Motor((_e4.g0_ + (vec4<f32>(_e6.g0_.x, _e9.g0_.y, _e12.g0_.z, _e15.g0_.x) * vec4<f32>(1.0, 1.0, 1.0, 0.0))), _e26.g1_);
}

fn rotor_line_sub(self_1362: Rotor, other_1108: Line) -> Motor {
    var self_1363: Rotor;
    var other_1109: Line;

    self_1363 = self_1362;
    other_1109 = other_1108;
    let _e4: Rotor = self_1363;
    let _e6: Line = other_1109;
    let _e9: Line = other_1109;
    let _e12: Line = other_1109;
    let _e15: Line = other_1109;
    let _e28: Line = other_1109;
    return Motor((_e4.g0_ - (vec4<f32>(_e6.g0_.x, _e9.g0_.y, _e12.g0_.z, _e15.g0_.x) * vec4<f32>(1.0, 1.0, 1.0, 0.0))), (vec3<f32>(0.0) - _e28.g1_));
}

fn rotor_line_geometric_product(self_1364: Rotor, other_1110: Line) -> Rotor {
    var self_1365: Rotor;
    var other_1111: Line;

    self_1365 = self_1364;
    other_1111 = other_1110;
    let _e4: Rotor = self_1365;
    let _e8: Line = other_1111;
    let _e11: Line = other_1111;
    let _e14: Line = other_1111;
    let _e17: Line = other_1111;
    let _e30: Rotor = self_1365;
    let _e34: Line = other_1111;
    let _e37: Line = other_1111;
    let _e40: Line = other_1111;
    let _e43: Line = other_1111;
    let _e57: Rotor = self_1365;
    let _e61: Line = other_1111;
    let _e64: Line = other_1111;
    let _e67: Line = other_1111;
    let _e70: Line = other_1111;
    let _e82: Rotor = self_1365;
    let _e86: Line = other_1111;
    let _e89: Line = other_1111;
    let _e92: Line = other_1111;
    let _e95: Line = other_1111;
    return Rotor((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g1_.z, _e11.g1_.z, _e14.g1_.x, _e17.g1_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e30.g0_.z) * vec4<f32>(_e34.g1_.y, _e37.g1_.x, _e40.g1_.y, _e43.g1_.z)) * vec4<f32>(1.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e57.g0_.w) * vec4<f32>(_e61.g1_.x, _e64.g1_.y, _e67.g1_.z, _e70.g1_.x)) * vec4<f32>(1.0, 1.0, 1.0, 0.0))) + ((vec4<f32>(_e82.g0_.x) * vec4<f32>(_e86.g1_.x, _e89.g1_.z, _e92.g1_.y, _e95.g1_.x)) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))));
}

fn rotor_line_outer_product(self_1366: Rotor, other_1112: Line) -> AntiScalar {
    var self_1367: Rotor;
    var other_1113: Line;

    self_1367 = self_1366;
    other_1113 = other_1112;
    let _e5: Rotor = self_1367;
    let _e8: Line = other_1113;
    let _e13: Rotor = self_1367;
    let _e16: Line = other_1113;
    let _e21: Rotor = self_1367;
    let _e24: Line = other_1113;
    return AntiScalar((((0.0 - (_e5.g0_.x * _e8.g1_.x)) - (_e13.g0_.y * _e16.g1_.y)) - (_e21.g0_.z * _e24.g1_.z)));
}

fn rotor_line_inner_anti_product(self_1368: Rotor, other_1114: Line) -> Motor {
    var self_1369: Rotor;
    var other_1115: Line;

    self_1369 = self_1368;
    other_1115 = other_1114;
    let _e4: Rotor = self_1369;
    let _e8: Line = other_1115;
    let _e20: Rotor = self_1369;
    let _e24: Line = other_1115;
    let _e37: Rotor = self_1369;
    let _e40: Line = other_1115;
    let _e43: Line = other_1115;
    let _e46: Line = other_1115;
    let _e49: Line = other_1115;
    let _e62: Rotor = self_1369;
    let _e66: Line = other_1115;
    return Motor(((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0))) + ((vec4<f32>(_e20.g0_.z) * vec4<f32>(_e24.g0_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((_e37.g0_.wwwx * vec4<f32>(_e40.g0_.x, _e43.g0_.y, _e46.g0_.z, _e49.g0_.x)) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))), (vec3<f32>(_e62.g0_.w) * _e66.g1_));
}

fn rotor_line_left_anti_contraction(self_1370: Rotor, other_1116: Line) -> Motor {
    var self_1371: Rotor;
    var other_1117: Line;

    self_1371 = self_1370;
    other_1117 = other_1116;
    let _e4: Rotor = self_1371;
    let _e8: Line = other_1117;
    let _e20: Rotor = self_1371;
    let _e24: Line = other_1117;
    let _e37: Rotor = self_1371;
    let _e40: Line = other_1117;
    let _e43: Line = other_1117;
    let _e46: Line = other_1117;
    let _e49: Line = other_1117;
    let _e62: Rotor = self_1371;
    let _e66: Line = other_1117;
    return Motor(((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0))) + ((vec4<f32>(_e20.g0_.z) * vec4<f32>(_e24.g0_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((_e37.g0_.wwwx * vec4<f32>(_e40.g0_.x, _e43.g0_.y, _e46.g0_.z, _e49.g0_.x)) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))), (vec3<f32>(_e62.g0_.w) * _e66.g1_));
}

fn rotor_line_right_anti_contraction(self_1372: Rotor, other_1118: Line) -> AntiScalar {
    var self_1373: Rotor;
    var other_1119: Line;

    self_1373 = self_1372;
    other_1119 = other_1118;
    let _e5: Rotor = self_1373;
    let _e8: Line = other_1119;
    let _e13: Rotor = self_1373;
    let _e16: Line = other_1119;
    let _e21: Rotor = self_1373;
    let _e24: Line = other_1119;
    return AntiScalar((((0.0 - (_e5.g0_.x * _e8.g0_.x)) - (_e13.g0_.y * _e16.g0_.y)) - (_e21.g0_.z * _e24.g0_.z)));
}

fn rotor_line_anti_scalar_product(self_1374: Rotor, other_1120: Line) -> AntiScalar {
    var self_1375: Rotor;
    var other_1121: Line;

    self_1375 = self_1374;
    other_1121 = other_1120;
    let _e5: Rotor = self_1375;
    let _e8: Line = other_1121;
    let _e13: Rotor = self_1375;
    let _e16: Line = other_1121;
    let _e21: Rotor = self_1375;
    let _e24: Line = other_1121;
    return AntiScalar((((0.0 - (_e5.g0_.x * _e8.g0_.x)) - (_e13.g0_.y * _e16.g0_.y)) - (_e21.g0_.z * _e24.g0_.z)));
}

fn rotor_plane_regressive_product(self_1376: Rotor, other_1122: Plane) -> Flector {
    var self_1377: Rotor;
    var other_1123: Plane;

    self_1377 = self_1376;
    other_1123 = other_1122;
    let _e4: Rotor = self_1377;
    let _e8: Plane = other_1123;
    let _e19: Rotor = self_1377;
    let _e23: Plane = other_1123;
    let _e35: Rotor = self_1377;
    let _e39: Plane = other_1123;
    let _e51: Rotor = self_1377;
    let _e55: Plane = other_1123;
    return Flector(((((vec4<f32>(_e4.g0_.y) * _e8.g0_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e35.g0_.x) * _e39.g0_.wxxx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))), (vec4<f32>(_e51.g0_.w) * _e55.g0_));
}

fn rotor_plane_geometric_anti_product(self_1378: Rotor, other_1124: Plane) -> Flector {
    var self_1379: Rotor;
    var other_1125: Plane;

    self_1379 = self_1378;
    other_1125 = other_1124;
    let _e4: Rotor = self_1379;
    let _e8: Plane = other_1125;
    let _e19: Rotor = self_1379;
    let _e23: Plane = other_1125;
    let _e35: Rotor = self_1379;
    let _e39: Plane = other_1125;
    let _e51: Rotor = self_1379;
    let _e55: Plane = other_1125;
    let _e66: Rotor = self_1379;
    let _e70: Plane = other_1125;
    let _e82: Rotor = self_1379;
    let _e86: Plane = other_1125;
    let _e90: Rotor = self_1379;
    let _e94: Plane = other_1125;
    return Flector(((((vec4<f32>(_e4.g0_.y) * _e8.g0_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e35.g0_.x) * _e39.g0_.wxxx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))), (((((vec4<f32>(_e51.g0_.y) * _e55.g0_.zzxz) * vec4<f32>(1.0, 0.0, -(1.0), 0.0)) + ((vec4<f32>(_e66.g0_.z) * _e70.g0_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))) + (vec4<f32>(_e82.g0_.w) * _e86.g0_)) + ((vec4<f32>(_e90.g0_.x) * _e94.g0_.xzyx) * vec4<f32>(0.0, -(1.0), 1.0, 0.0))));
}

fn rotor_plane_inner_anti_product(self_1380: Rotor, other_1126: Plane) -> Plane {
    var self_1381: Rotor;
    var other_1127: Plane;

    self_1381 = self_1380;
    other_1127 = other_1126;
    let _e4: Rotor = self_1381;
    let _e8: Plane = other_1127;
    let _e19: Rotor = self_1381;
    let _e23: Plane = other_1127;
    let _e35: Rotor = self_1381;
    let _e39: Plane = other_1127;
    let _e43: Rotor = self_1381;
    let _e47: Plane = other_1127;
    return Plane((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.zzxz) * vec4<f32>(1.0, 0.0, -(1.0), 0.0)) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))) + (vec4<f32>(_e35.g0_.w) * _e39.g0_)) + ((vec4<f32>(_e43.g0_.x) * _e47.g0_.xzyx) * vec4<f32>(0.0, -(1.0), 1.0, 0.0))));
}

fn rotor_plane_left_anti_contraction(self_1382: Rotor, other_1128: Plane) -> Plane {
    var self_1383: Rotor;
    var other_1129: Plane;

    self_1383 = self_1382;
    other_1129 = other_1128;
    let _e4: Rotor = self_1383;
    let _e8: Plane = other_1129;
    return Plane((vec4<f32>(_e4.g0_.w) * _e8.g0_));
}

fn rotor_motor_add(self_1384: Rotor, other_1130: Motor) -> Motor {
    var self_1385: Rotor;
    var other_1131: Motor;

    self_1385 = self_1384;
    other_1131 = other_1130;
    let _e4: Rotor = self_1385;
    let _e6: Motor = other_1131;
    let _e9: Motor = other_1131;
    return Motor((_e4.g0_ + _e6.g0_), _e9.g1_);
}

fn rotor_motor_sub(self_1386: Rotor, other_1132: Motor) -> Motor {
    var self_1387: Rotor;
    var other_1133: Motor;

    self_1387 = self_1386;
    other_1133 = other_1132;
    let _e4: Rotor = self_1387;
    let _e6: Motor = other_1133;
    let _e11: Motor = other_1133;
    return Motor((_e4.g0_ - _e6.g0_), (vec3<f32>(0.0) - _e11.g1_));
}

fn rotor_motor_geometric_product(self_1388: Rotor, other_1134: Motor) -> Rotor {
    var self_1389: Rotor;
    var other_1135: Motor;

    self_1389 = self_1388;
    other_1135 = other_1134;
    let _e4: Rotor = self_1389;
    let _e8: Motor = other_1135;
    let _e11: Motor = other_1135;
    let _e14: Motor = other_1135;
    let _e17: Motor = other_1135;
    let _e30: Rotor = self_1389;
    let _e34: Motor = other_1135;
    let _e37: Motor = other_1135;
    let _e40: Motor = other_1135;
    let _e43: Motor = other_1135;
    let _e57: Rotor = self_1389;
    let _e61: Motor = other_1135;
    let _e64: Motor = other_1135;
    let _e67: Motor = other_1135;
    let _e70: Motor = other_1135;
    let _e82: Rotor = self_1389;
    let _e86: Motor = other_1135;
    let _e89: Motor = other_1135;
    let _e92: Motor = other_1135;
    let _e95: Motor = other_1135;
    return Rotor((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g1_.z, _e11.g1_.z, _e14.g1_.x, _e17.g1_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e30.g0_.z) * vec4<f32>(_e34.g1_.y, _e37.g1_.x, _e40.g1_.y, _e43.g1_.z)) * vec4<f32>(1.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e57.g0_.w) * vec4<f32>(_e61.g1_.x, _e64.g1_.y, _e67.g1_.z, _e70.g1_.x)) * vec4<f32>(1.0, 1.0, 1.0, 0.0))) + ((vec4<f32>(_e82.g0_.x) * vec4<f32>(_e86.g1_.x, _e89.g1_.z, _e92.g1_.y, _e95.g1_.x)) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))));
}

fn rotor_motor_outer_product(self_1390: Rotor, other_1136: Motor) -> AntiScalar {
    var self_1391: Rotor;
    var other_1137: Motor;

    self_1391 = self_1390;
    other_1137 = other_1136;
    let _e5: Rotor = self_1391;
    let _e8: Motor = other_1137;
    let _e13: Rotor = self_1391;
    let _e16: Motor = other_1137;
    let _e21: Rotor = self_1391;
    let _e24: Motor = other_1137;
    return AntiScalar((((0.0 - (_e5.g0_.x * _e8.g1_.x)) - (_e13.g0_.y * _e16.g1_.y)) - (_e21.g0_.z * _e24.g1_.z)));
}

fn rotor_motor_inner_anti_product(self_1392: Rotor, other_1138: Motor) -> Motor {
    var self_1393: Rotor;
    var other_1139: Motor;

    self_1393 = self_1392;
    other_1139 = other_1138;
    let _e4: Rotor = self_1393;
    let _e8: Motor = other_1139;
    let _e19: Rotor = self_1393;
    let _e23: Motor = other_1139;
    let _e35: Rotor = self_1393;
    let _e39: Motor = other_1139;
    let _e43: Rotor = self_1393;
    let _e47: Motor = other_1139;
    let _e59: Rotor = self_1393;
    let _e63: Motor = other_1139;
    return Motor((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + (vec4<f32>(_e35.g0_.w) * _e39.g0_)) + ((vec4<f32>(_e43.g0_.x) * _e47.g0_.wxxx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))), (vec3<f32>(_e59.g0_.w) * _e63.g1_));
}

fn rotor_motor_left_anti_contraction(self_1394: Rotor, other_1140: Motor) -> Motor {
    var self_1395: Rotor;
    var other_1141: Motor;

    self_1395 = self_1394;
    other_1141 = other_1140;
    let _e4: Rotor = self_1395;
    let _e8: Motor = other_1141;
    let _e20: Rotor = self_1395;
    let _e24: Motor = other_1141;
    let _e37: Rotor = self_1395;
    let _e41: Motor = other_1141;
    let _e45: Rotor = self_1395;
    let _e49: Motor = other_1141;
    let _e62: Rotor = self_1395;
    let _e66: Motor = other_1141;
    return Motor((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0))) + ((vec4<f32>(_e20.g0_.z) * vec4<f32>(_e24.g0_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + (vec4<f32>(_e37.g0_.w) * _e41.g0_)) + ((vec4<f32>(_e45.g0_.x) * vec4<f32>(_e49.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))), (vec3<f32>(_e62.g0_.w) * _e66.g1_));
}

fn rotor_motor_right_anti_contraction(self_1396: Rotor, other_1142: Motor) -> Rotor {
    var self_1397: Rotor;
    var other_1143: Motor;

    self_1397 = self_1396;
    other_1143 = other_1142;
    let _e4: Rotor = self_1397;
    let _e8: Motor = other_1143;
    let _e19: Rotor = self_1397;
    let _e23: Motor = other_1143;
    let _e35: Rotor = self_1397;
    let _e39: Motor = other_1143;
    let _e51: Rotor = self_1397;
    let _e55: Motor = other_1143;
    return Rotor((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e35.g0_.w) * vec4<f32>(_e39.g0_.w)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e51.g0_.x) * _e55.g0_.wxxx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))));
}

fn rotor_motor_anti_scalar_product(self_1398: Rotor, other_1144: Motor) -> AntiScalar {
    var self_1399: Rotor;
    var other_1145: Motor;

    self_1399 = self_1398;
    other_1145 = other_1144;
    let _e5: Rotor = self_1399;
    let _e8: Motor = other_1145;
    let _e13: Rotor = self_1399;
    let _e16: Motor = other_1145;
    let _e21: Rotor = self_1399;
    let _e24: Motor = other_1145;
    let _e29: Rotor = self_1399;
    let _e32: Motor = other_1145;
    return AntiScalar(((((0.0 - (_e5.g0_.x * _e8.g0_.x)) - (_e13.g0_.y * _e16.g0_.y)) - (_e21.g0_.z * _e24.g0_.z)) + (_e29.g0_.w * _e32.g0_.w)));
}

fn rotor_rotor_add(self_1400: Rotor, other_1146: Rotor) -> Rotor {
    var self_1401: Rotor;
    var other_1147: Rotor;

    self_1401 = self_1400;
    other_1147 = other_1146;
    let _e4: Rotor = self_1401;
    let _e6: Rotor = other_1147;
    return Rotor((_e4.g0_ + _e6.g0_));
}

fn rotor_rotor_sub(self_1402: Rotor, other_1148: Rotor) -> Rotor {
    var self_1403: Rotor;
    var other_1149: Rotor;

    self_1403 = self_1402;
    other_1149 = other_1148;
    let _e4: Rotor = self_1403;
    let _e6: Rotor = other_1149;
    return Rotor((_e4.g0_ - _e6.g0_));
}

fn rotor_rotor_mul(self_1404: Rotor, other_1150: Rotor) -> Rotor {
    var self_1405: Rotor;
    var other_1151: Rotor;

    self_1405 = self_1404;
    other_1151 = other_1150;
    let _e4: Rotor = self_1405;
    let _e6: Rotor = other_1151;
    return Rotor((_e4.g0_ * _e6.g0_));
}

fn rotor_rotor_div(self_1406: Rotor, other_1152: Rotor) -> Rotor {
    var self_1407: Rotor;
    var other_1153: Rotor;

    self_1407 = self_1406;
    other_1153 = other_1152;
    let _e4: Rotor = self_1407;
    let _e7: Rotor = self_1407;
    let _e10: Rotor = self_1407;
    let _e13: Rotor = self_1407;
    let _e23: Rotor = other_1153;
    let _e26: Rotor = other_1153;
    let _e29: Rotor = other_1153;
    let _e32: Rotor = other_1153;
    return Rotor((((vec4<f32>(_e4.g0_.x, _e7.g0_.y, _e10.g0_.z, _e13.g0_.w) * vec4<f32>(1.0, 1.0, 1.0, 1.0)) / vec4<f32>(_e23.g0_.x, _e26.g0_.y, _e29.g0_.z, _e32.g0_.w)) * vec4<f32>(1.0, 1.0, 1.0, 1.0)));
}

fn rotor_rotor_regressive_product(self_1408: Rotor, other_1154: Rotor) -> Rotor {
    var self_1409: Rotor;
    var other_1155: Rotor;

    self_1409 = self_1408;
    other_1155 = other_1154;
    let _e4: Rotor = self_1409;
    let _e8: Rotor = other_1155;
    let _e11: Rotor = self_1409;
    let _e14: Rotor = other_1155;
    return Rotor(((vec4<f32>(_e4.g0_.w) * _e8.g0_) + ((_e11.g0_.xyzx * _e14.g0_.wwwx) * vec4<f32>(1.0, 1.0, 1.0, 0.0))));
}

fn rotor_rotor_geometric_anti_product(self_1410: Rotor, other_1156: Rotor) -> Rotor {
    var self_1411: Rotor;
    var other_1157: Rotor;

    self_1411 = self_1410;
    other_1157 = other_1156;
    let _e4: Rotor = self_1411;
    let _e8: Rotor = other_1157;
    let _e20: Rotor = self_1411;
    let _e24: Rotor = other_1157;
    let _e37: Rotor = self_1411;
    let _e41: Rotor = other_1157;
    let _e54: Rotor = self_1411;
    let _e58: Rotor = other_1157;
    return Rotor((((((vec4<f32>(_e4.g0_.x) * _e8.g0_.wzyx) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0))) + ((vec4<f32>(_e20.g0_.y) * _e24.g0_.zwxy) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e37.g0_.z) * _e41.g0_.yxwz) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + (vec4<f32>(_e54.g0_.w) * _e58.g0_)));
}

fn rotor_rotor_inner_anti_product(self_1412: Rotor, other_1158: Rotor) -> Rotor {
    var self_1413: Rotor;
    var other_1159: Rotor;

    self_1413 = self_1412;
    other_1159 = other_1158;
    let _e4: Rotor = self_1413;
    let _e8: Rotor = other_1159;
    let _e19: Rotor = self_1413;
    let _e23: Rotor = other_1159;
    let _e35: Rotor = self_1413;
    let _e39: Rotor = other_1159;
    let _e43: Rotor = self_1413;
    let _e47: Rotor = other_1159;
    return Rotor((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + (vec4<f32>(_e35.g0_.w) * _e39.g0_)) + ((vec4<f32>(_e43.g0_.x) * _e47.g0_.wxxx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))));
}

fn rotor_rotor_left_anti_contraction(self_1414: Rotor, other_1160: Rotor) -> Rotor {
    var self_1415: Rotor;
    var other_1161: Rotor;

    self_1415 = self_1414;
    other_1161 = other_1160;
    let _e4: Rotor = self_1415;
    let _e8: Rotor = other_1161;
    let _e20: Rotor = self_1415;
    let _e24: Rotor = other_1161;
    let _e37: Rotor = self_1415;
    let _e41: Rotor = other_1161;
    let _e45: Rotor = self_1415;
    let _e49: Rotor = other_1161;
    return Rotor((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0))) + ((vec4<f32>(_e20.g0_.z) * vec4<f32>(_e24.g0_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + (vec4<f32>(_e37.g0_.w) * _e41.g0_)) + ((vec4<f32>(_e45.g0_.x) * vec4<f32>(_e49.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))));
}

fn rotor_rotor_right_anti_contraction(self_1416: Rotor, other_1162: Rotor) -> Rotor {
    var self_1417: Rotor;
    var other_1163: Rotor;

    self_1417 = self_1416;
    other_1163 = other_1162;
    let _e4: Rotor = self_1417;
    let _e8: Rotor = other_1163;
    let _e19: Rotor = self_1417;
    let _e23: Rotor = other_1163;
    let _e35: Rotor = self_1417;
    let _e39: Rotor = other_1163;
    let _e51: Rotor = self_1417;
    let _e55: Rotor = other_1163;
    return Rotor((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e35.g0_.w) * vec4<f32>(_e39.g0_.w)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e51.g0_.x) * _e55.g0_.wxxx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))));
}

fn rotor_rotor_anti_scalar_product(self_1418: Rotor, other_1164: Rotor) -> AntiScalar {
    var self_1419: Rotor;
    var other_1165: Rotor;

    self_1419 = self_1418;
    other_1165 = other_1164;
    let _e5: Rotor = self_1419;
    let _e8: Rotor = other_1165;
    let _e13: Rotor = self_1419;
    let _e16: Rotor = other_1165;
    let _e21: Rotor = self_1419;
    let _e24: Rotor = other_1165;
    let _e29: Rotor = self_1419;
    let _e32: Rotor = other_1165;
    return AntiScalar(((((0.0 - (_e5.g0_.x * _e8.g0_.x)) - (_e13.g0_.y * _e16.g0_.y)) - (_e21.g0_.z * _e24.g0_.z)) + (_e29.g0_.w * _e32.g0_.w)));
}

fn rotor_translator_add(self_1420: Rotor, other_1166: Translator) -> Motor {
    var self_1421: Rotor;
    var other_1167: Translator;

    self_1421 = self_1420;
    other_1167 = other_1166;
    let _e4: Rotor = self_1421;
    let _e6: Translator = other_1167;
    let _e16: Translator = other_1167;
    let _e19: Translator = other_1167;
    let _e22: Translator = other_1167;
    return Motor((_e4.g0_ + (_e6.g0_.xxxw * vec4<f32>(0.0, 0.0, 0.0, 1.0))), vec3<f32>(_e16.g0_.x, _e19.g0_.y, _e22.g0_.z));
}

fn rotor_translator_sub(self_1422: Rotor, other_1168: Translator) -> Motor {
    var self_1423: Rotor;
    var other_1169: Translator;

    self_1423 = self_1422;
    other_1169 = other_1168;
    let _e4: Rotor = self_1423;
    let _e6: Translator = other_1169;
    let _e18: Translator = other_1169;
    let _e21: Translator = other_1169;
    let _e24: Translator = other_1169;
    return Motor((_e4.g0_ - (_e6.g0_.xxxw * vec4<f32>(0.0, 0.0, 0.0, 1.0))), (vec3<f32>(0.0) - vec3<f32>(_e18.g0_.x, _e21.g0_.y, _e24.g0_.z)));
}

fn rotor_translator_geometric_product(self_1424: Rotor, other_1170: Translator) -> Rotor {
    var self_1425: Rotor;
    var other_1171: Translator;

    self_1425 = self_1424;
    other_1171 = other_1170;
    let _e4: Rotor = self_1425;
    let _e8: Translator = other_1171;
    let _e20: Rotor = self_1425;
    let _e24: Translator = other_1171;
    let _e37: Rotor = self_1425;
    let _e41: Translator = other_1171;
    let _e52: Rotor = self_1425;
    let _e56: Translator = other_1171;
    return Rotor((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.zzxy) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e20.g0_.z) * _e24.g0_.yxyz) * vec4<f32>(1.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e37.g0_.w) * _e41.g0_.xyzx) * vec4<f32>(1.0, 1.0, 1.0, 0.0))) + ((vec4<f32>(_e52.g0_.x) * _e56.g0_.xzyx) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))));
}

fn rotor_translator_outer_product(self_1426: Rotor, other_1172: Translator) -> AntiScalar {
    var self_1427: Rotor;
    var other_1173: Translator;

    self_1427 = self_1426;
    other_1173 = other_1172;
    let _e5: Rotor = self_1427;
    let _e8: Translator = other_1173;
    let _e13: Rotor = self_1427;
    let _e16: Translator = other_1173;
    let _e21: Rotor = self_1427;
    let _e24: Translator = other_1173;
    return AntiScalar((((0.0 - (_e5.g0_.x * _e8.g0_.x)) - (_e13.g0_.y * _e16.g0_.y)) - (_e21.g0_.z * _e24.g0_.z)));
}

fn rotor_translator_inner_anti_product(self_1428: Rotor, other_1174: Translator) -> Motor {
    var self_1429: Rotor;
    var other_1175: Translator;

    self_1429 = self_1428;
    other_1175 = other_1174;
    let _e4: Rotor = self_1429;
    let _e6: Translator = other_1175;
    let _e11: Rotor = self_1429;
    let _e15: Translator = other_1175;
    let _e18: Translator = other_1175;
    let _e21: Translator = other_1175;
    return Motor((_e4.g0_ * vec4<f32>(_e6.g0_.w)), (vec3<f32>(_e11.g0_.w) * vec3<f32>(_e15.g0_.x, _e18.g0_.y, _e21.g0_.z)));
}

fn rotor_translator_left_anti_contraction(self_1430: Rotor, other_1176: Translator) -> Translator {
    var self_1431: Rotor;
    var other_1177: Translator;

    self_1431 = self_1430;
    other_1177 = other_1176;
    let _e4: Rotor = self_1431;
    let _e8: Translator = other_1177;
    return Translator((vec4<f32>(_e4.g0_.w) * _e8.g0_));
}

fn rotor_translator_right_anti_contraction(self_1432: Rotor, other_1178: Translator) -> Rotor {
    var self_1433: Rotor;
    var other_1179: Translator;

    self_1433 = self_1432;
    other_1179 = other_1178;
    let _e4: Rotor = self_1433;
    let _e6: Translator = other_1179;
    return Rotor((_e4.g0_ * vec4<f32>(_e6.g0_.w)));
}

fn rotor_translator_anti_scalar_product(self_1434: Rotor, other_1180: Translator) -> AntiScalar {
    var self_1435: Rotor;
    var other_1181: Translator;

    self_1435 = self_1434;
    other_1181 = other_1180;
    let _e4: Rotor = self_1435;
    let _e7: Translator = other_1181;
    return AntiScalar((_e4.g0_.w * _e7.g0_.w));
}

fn rotor_flector_regressive_product(self_1436: Rotor, other_1182: Flector) -> Flector {
    var self_1437: Rotor;
    var other_1183: Flector;

    self_1437 = self_1436;
    other_1183 = other_1182;
    let _e4: Rotor = self_1437;
    let _e8: Flector = other_1183;
    let _e19: Rotor = self_1437;
    let _e23: Flector = other_1183;
    let _e35: Rotor = self_1437;
    let _e39: Flector = other_1183;
    let _e43: Rotor = self_1437;
    let _e47: Flector = other_1183;
    let _e59: Rotor = self_1437;
    let _e63: Flector = other_1183;
    return Flector((((((vec4<f32>(_e4.g0_.y) * _e8.g1_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e19.g0_.z) * _e23.g1_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + (vec4<f32>(_e35.g0_.w) * _e39.g0_)) + ((vec4<f32>(_e43.g0_.x) * _e47.g1_.wxxx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))), (vec4<f32>(_e59.g0_.w) * _e63.g1_));
}

fn rotor_flector_geometric_anti_product(self_1438: Rotor, other_1184: Flector) -> Flector {
    var self_1439: Rotor;
    var other_1185: Flector;

    self_1439 = self_1438;
    other_1185 = other_1184;
    let _e4: Rotor = self_1439;
    let _e8: Flector = other_1185;
    let _e11: Flector = other_1185;
    let _e14: Flector = other_1185;
    let _e17: Flector = other_1185;
    let _e30: Rotor = self_1439;
    let _e34: Flector = other_1185;
    let _e37: Flector = other_1185;
    let _e40: Flector = other_1185;
    let _e43: Flector = other_1185;
    let _e57: Rotor = self_1439;
    let _e61: Flector = other_1185;
    let _e64: Flector = other_1185;
    let _e67: Flector = other_1185;
    let _e70: Flector = other_1185;
    let _e84: Rotor = self_1439;
    let _e88: Flector = other_1185;
    let _e92: Rotor = self_1439;
    let _e96: Flector = other_1185;
    let _e99: Flector = other_1185;
    let _e102: Flector = other_1185;
    let _e105: Flector = other_1185;
    let _e118: Rotor = self_1439;
    let _e122: Flector = other_1185;
    let _e125: Flector = other_1185;
    let _e128: Flector = other_1185;
    let _e131: Flector = other_1185;
    let _e145: Rotor = self_1439;
    let _e149: Flector = other_1185;
    let _e152: Flector = other_1185;
    let _e155: Flector = other_1185;
    let _e158: Flector = other_1185;
    let _e172: Rotor = self_1439;
    let _e176: Flector = other_1185;
    return Flector((((((vec4<f32>(_e4.g0_.x) * vec4<f32>(_e8.g1_.w, _e11.g0_.z, _e14.g0_.y, _e17.g1_.x)) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0))) + ((vec4<f32>(_e30.g0_.y) * vec4<f32>(_e34.g0_.z, _e37.g1_.w, _e40.g0_.x, _e43.g1_.y)) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e57.g0_.z) * vec4<f32>(_e61.g0_.y, _e64.g0_.x, _e67.g1_.w, _e70.g1_.z)) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + (vec4<f32>(_e84.g0_.w) * _e88.g0_)), (((((vec4<f32>(_e92.g0_.x) * vec4<f32>(_e96.g0_.w, _e99.g1_.z, _e102.g1_.y, _e105.g0_.x)) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0))) + ((vec4<f32>(_e118.g0_.y) * vec4<f32>(_e122.g1_.z, _e125.g0_.w, _e128.g1_.x, _e131.g0_.y)) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e145.g0_.z) * vec4<f32>(_e149.g1_.y, _e152.g1_.x, _e155.g0_.w, _e158.g0_.z)) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + (vec4<f32>(_e172.g0_.w) * _e176.g1_)));
}

fn rotor_flector_inner_anti_product(self_1440: Rotor, other_1186: Flector) -> Flector {
    var self_1441: Rotor;
    var other_1187: Flector;

    self_1441 = self_1440;
    other_1187 = other_1186;
    let _e4: Rotor = self_1441;
    let _e8: Flector = other_1187;
    let _e11: Rotor = self_1441;
    let _e15: Flector = other_1187;
    let _e18: Flector = other_1187;
    let _e21: Flector = other_1187;
    let _e24: Flector = other_1187;
    let _e37: Rotor = self_1441;
    let _e41: Flector = other_1187;
    let _e44: Flector = other_1187;
    let _e47: Flector = other_1187;
    let _e50: Flector = other_1187;
    let _e64: Rotor = self_1441;
    let _e68: Flector = other_1187;
    let _e71: Flector = other_1187;
    let _e74: Flector = other_1187;
    let _e77: Flector = other_1187;
    let _e91: Rotor = self_1441;
    let _e95: Flector = other_1187;
    return Flector((vec4<f32>(_e4.g0_.w) * _e8.g0_), (((((vec4<f32>(_e11.g0_.x) * vec4<f32>(_e15.g0_.w, _e18.g1_.z, _e21.g1_.y, _e24.g0_.x)) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0))) + ((vec4<f32>(_e37.g0_.y) * vec4<f32>(_e41.g1_.z, _e44.g0_.w, _e47.g1_.x, _e50.g0_.y)) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e64.g0_.z) * vec4<f32>(_e68.g1_.y, _e71.g1_.x, _e74.g0_.w, _e77.g0_.z)) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + (vec4<f32>(_e91.g0_.w) * _e95.g1_)));
}

fn rotor_flector_left_anti_contraction(self_1442: Rotor, other_1188: Flector) -> Flector {
    var self_1443: Rotor;
    var other_1189: Flector;

    self_1443 = self_1442;
    other_1189 = other_1188;
    let _e4: Rotor = self_1443;
    let _e8: Flector = other_1189;
    let _e11: Rotor = self_1443;
    let _e15: Flector = other_1189;
    let _e26: Rotor = self_1443;
    let _e30: Flector = other_1189;
    let _e42: Rotor = self_1443;
    let _e46: Flector = other_1189;
    let _e50: Rotor = self_1443;
    let _e54: Flector = other_1189;
    return Flector((vec4<f32>(_e4.g0_.w) * _e8.g0_), (((((vec4<f32>(_e11.g0_.y) * _e15.g0_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e26.g0_.z) * _e30.g0_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + (vec4<f32>(_e42.g0_.w) * _e46.g1_)) + ((vec4<f32>(_e50.g0_.x) * _e54.g0_.wxxx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))));
}

fn rotor_multi_vector_anti_scalar_product(self_1444: Rotor, other_1190: MultiVector) -> AntiScalar {
    var self_1445: Rotor;
    var other_1191: MultiVector;

    self_1445 = self_1444;
    other_1191 = other_1190;
    let _e5: Rotor = self_1445;
    let _e8: MultiVector = other_1191;
    let _e13: Rotor = self_1445;
    let _e16: MultiVector = other_1191;
    let _e21: Rotor = self_1445;
    let _e24: MultiVector = other_1191;
    let _e29: Rotor = self_1445;
    let _e32: MultiVector = other_1191;
    return AntiScalar(((((0.0 - (_e5.g0_.x * _e8.g2_.x)) - (_e13.g0_.y * _e16.g2_.y)) - (_e21.g0_.z * _e24.g2_.z)) + (_e29.g0_.w * _e32.g0_.y)));
}

fn rotor_scale(self_1446: Rotor, other_1192: f32) -> Rotor {
    var self_1447: Rotor;
    var other_1193: f32;

    self_1447 = self_1446;
    other_1193 = other_1192;
    let _e4: Rotor = self_1447;
    let _e5: f32 = other_1193;
    let _e7: Rotor = rotor_scalar_geometric_product(_e4, Scalar(_e5));
    return _e7;
}

fn translator_zero() -> Translator {
    return Translator(vec4<f32>(0.0));
}

fn translator_one() -> Translator {
    return Translator(vec4<f32>(0.0));
}

fn translator_neg(self_1448: Translator) -> Translator {
    var self_1449: Translator;

    self_1449 = self_1448;
    let _e2: Translator = self_1449;
    return Translator((_e2.g0_ * vec4<f32>(-(1.0))));
}

fn translator_automorphism(self_1450: Translator) -> Translator {
    var self_1451: Translator;

    self_1451 = self_1450;
    let _e2: Translator = self_1451;
    return Translator(_e2.g0_);
}

fn translator_reversal(self_1452: Translator) -> Translator {
    var self_1453: Translator;

    self_1453 = self_1452;
    let _e2: Translator = self_1453;
    return Translator((_e2.g0_ * vec4<f32>(-(1.0), -(1.0), -(1.0), 1.0)));
}

fn translator_conjugation(self_1454: Translator) -> Translator {
    var self_1455: Translator;

    self_1455 = self_1454;
    let _e2: Translator = self_1455;
    return Translator((_e2.g0_ * vec4<f32>(-(1.0), -(1.0), -(1.0), 1.0)));
}

fn translator_anti_reversal(self_1456: Translator) -> Translator {
    var self_1457: Translator;

    self_1457 = self_1456;
    let _e2: Translator = self_1457;
    return Translator((_e2.g0_ * vec4<f32>(-(1.0), -(1.0), -(1.0), 1.0)));
}

fn translator_double_complement(self_1458: Translator) -> Translator {
    var self_1459: Translator;

    self_1459 = self_1458;
    let _e2: Translator = self_1459;
    return Translator(_e2.g0_);
}

fn translator_scalar_geometric_product(self_1460: Translator, other_1194: Scalar) -> Translator {
    var self_1461: Translator;
    var other_1195: Scalar;

    self_1461 = self_1460;
    other_1195 = other_1194;
    let _e4: Translator = self_1461;
    let _e6: Scalar = other_1195;
    return Translator((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn translator_scalar_regressive_product(self_1462: Translator, other_1196: Scalar) -> Scalar {
    var self_1463: Translator;
    var other_1197: Scalar;

    self_1463 = self_1462;
    other_1197 = other_1196;
    let _e4: Translator = self_1463;
    let _e7: Scalar = other_1197;
    return Scalar((_e4.g0_.w * _e7.g0_));
}

fn translator_scalar_outer_product(self_1464: Translator, other_1198: Scalar) -> Translator {
    var self_1465: Translator;
    var other_1199: Scalar;

    self_1465 = self_1464;
    other_1199 = other_1198;
    let _e4: Translator = self_1465;
    let _e6: Scalar = other_1199;
    return Translator((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn translator_scalar_inner_product(self_1466: Translator, other_1200: Scalar) -> Translator {
    var self_1467: Translator;
    var other_1201: Scalar;

    self_1467 = self_1466;
    other_1201 = other_1200;
    let _e4: Translator = self_1467;
    let _e6: Scalar = other_1201;
    return Translator((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn translator_scalar_geometric_anti_product(self_1468: Translator, other_1202: Scalar) -> Scalar {
    var self_1469: Translator;
    var other_1203: Scalar;

    self_1469 = self_1468;
    other_1203 = other_1202;
    let _e4: Translator = self_1469;
    let _e7: Scalar = other_1203;
    return Scalar((_e4.g0_.w * _e7.g0_));
}

fn translator_scalar_inner_anti_product(self_1470: Translator, other_1204: Scalar) -> Scalar {
    var self_1471: Translator;
    var other_1205: Scalar;

    self_1471 = self_1470;
    other_1205 = other_1204;
    let _e4: Translator = self_1471;
    let _e7: Scalar = other_1205;
    return Scalar((_e4.g0_.w * _e7.g0_));
}

fn translator_scalar_right_contraction(self_1472: Translator, other_1206: Scalar) -> Translator {
    var self_1473: Translator;
    var other_1207: Scalar;

    self_1473 = self_1472;
    other_1207 = other_1206;
    let _e4: Translator = self_1473;
    let _e6: Scalar = other_1207;
    return Translator((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn translator_scalar_left_anti_contraction(self_1474: Translator, other_1208: Scalar) -> Scalar {
    var self_1475: Translator;
    var other_1209: Scalar;

    self_1475 = self_1474;
    other_1209 = other_1208;
    let _e4: Translator = self_1475;
    let _e7: Scalar = other_1209;
    return Scalar((_e4.g0_.w * _e7.g0_));
}

fn translator_anti_scalar_into(self_1476: Translator) -> AntiScalar {
    var self_1477: Translator;

    self_1477 = self_1476;
    let _e2: Translator = self_1477;
    return AntiScalar(_e2.g0_.w);
}

fn translator_anti_scalar_add(self_1478: Translator, other_1210: AntiScalar) -> Translator {
    var self_1479: Translator;
    var other_1211: AntiScalar;

    self_1479 = self_1478;
    other_1211 = other_1210;
    let _e4: Translator = self_1479;
    let _e6: AntiScalar = other_1211;
    return Translator((_e4.g0_ + (vec4<f32>(_e6.g0_) * vec4<f32>(0.0, 0.0, 0.0, 1.0))));
}

fn translator_anti_scalar_sub(self_1480: Translator, other_1212: AntiScalar) -> Translator {
    var self_1481: Translator;
    var other_1213: AntiScalar;

    self_1481 = self_1480;
    other_1213 = other_1212;
    let _e4: Translator = self_1481;
    let _e6: AntiScalar = other_1213;
    return Translator((_e4.g0_ - (vec4<f32>(_e6.g0_) * vec4<f32>(0.0, 0.0, 0.0, 1.0))));
}

fn translator_anti_scalar_regressive_product(self_1482: Translator, other_1214: AntiScalar) -> Translator {
    var self_1483: Translator;
    var other_1215: AntiScalar;

    self_1483 = self_1482;
    other_1215 = other_1214;
    let _e4: Translator = self_1483;
    let _e6: AntiScalar = other_1215;
    return Translator((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn translator_anti_scalar_geometric_anti_product(self_1484: Translator, other_1216: AntiScalar) -> Translator {
    var self_1485: Translator;
    var other_1217: AntiScalar;

    self_1485 = self_1484;
    other_1217 = other_1216;
    let _e4: Translator = self_1485;
    let _e6: AntiScalar = other_1217;
    return Translator((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn translator_anti_scalar_inner_anti_product(self_1486: Translator, other_1218: AntiScalar) -> Translator {
    var self_1487: Translator;
    var other_1219: AntiScalar;

    self_1487 = self_1486;
    other_1219 = other_1218;
    let _e4: Translator = self_1487;
    let _e6: AntiScalar = other_1219;
    return Translator((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn translator_anti_scalar_left_anti_contraction(self_1488: Translator, other_1220: AntiScalar) -> AntiScalar {
    var self_1489: Translator;
    var other_1221: AntiScalar;

    self_1489 = self_1488;
    other_1221 = other_1220;
    let _e4: Translator = self_1489;
    let _e7: AntiScalar = other_1221;
    return AntiScalar((_e4.g0_.w * _e7.g0_));
}

fn translator_anti_scalar_right_anti_contraction(self_1490: Translator, other_1222: AntiScalar) -> Translator {
    var self_1491: Translator;
    var other_1223: AntiScalar;

    self_1491 = self_1490;
    other_1223 = other_1222;
    let _e4: Translator = self_1491;
    let _e6: AntiScalar = other_1223;
    return Translator((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn translator_anti_scalar_anti_scalar_product(self_1492: Translator, other_1224: AntiScalar) -> AntiScalar {
    var self_1493: Translator;
    var other_1225: AntiScalar;

    self_1493 = self_1492;
    other_1225 = other_1224;
    let _e4: Translator = self_1493;
    let _e7: AntiScalar = other_1225;
    return AntiScalar((_e4.g0_.w * _e7.g0_));
}

fn translator_homogeneous_magnitude_geometric_product(self_1494: Translator, other_1226: HomogeneousMagnitude) -> Motor {
    var self_1495: Translator;
    var other_1227: HomogeneousMagnitude;

    self_1495 = self_1494;
    other_1227 = other_1226;
    let _e4: Translator = self_1495;
    let _e6: HomogeneousMagnitude = other_1227;
    let _e9: HomogeneousMagnitude = other_1227;
    let _e12: HomogeneousMagnitude = other_1227;
    let _e15: HomogeneousMagnitude = other_1227;
    let _e20: Translator = self_1495;
    let _e23: Translator = self_1495;
    let _e26: Translator = self_1495;
    let _e30: HomogeneousMagnitude = other_1227;
    return Motor((_e4.g0_ * vec4<f32>(_e6.g0_.y, _e9.g0_.y, _e12.g0_.y, _e15.g0_.x)), (vec3<f32>(_e20.g0_.x, _e23.g0_.y, _e26.g0_.z) * vec3<f32>(_e30.g0_.x)));
}

fn translator_homogeneous_magnitude_outer_product(self_1496: Translator, other_1228: HomogeneousMagnitude) -> Translator {
    var self_1497: Translator;
    var other_1229: HomogeneousMagnitude;

    self_1497 = self_1496;
    other_1229 = other_1228;
    let _e4: Translator = self_1497;
    let _e6: HomogeneousMagnitude = other_1229;
    return Translator((_e4.g0_ * vec4<f32>(_e6.g0_.x)));
}

fn translator_homogeneous_magnitude_inner_product(self_1498: Translator, other_1230: HomogeneousMagnitude) -> Motor {
    var self_1499: Translator;
    var other_1231: HomogeneousMagnitude;

    self_1499 = self_1498;
    other_1231 = other_1230;
    let _e4: Translator = self_1499;
    let _e6: HomogeneousMagnitude = other_1231;
    let _e9: HomogeneousMagnitude = other_1231;
    let _e12: HomogeneousMagnitude = other_1231;
    let _e15: HomogeneousMagnitude = other_1231;
    let _e20: Translator = self_1499;
    let _e23: Translator = self_1499;
    let _e26: Translator = self_1499;
    let _e30: HomogeneousMagnitude = other_1231;
    return Motor((_e4.g0_ * vec4<f32>(_e6.g0_.y, _e9.g0_.y, _e12.g0_.y, _e15.g0_.x)), (vec3<f32>(_e20.g0_.x, _e23.g0_.y, _e26.g0_.z) * vec3<f32>(_e30.g0_.x)));
}

fn translator_homogeneous_magnitude_right_contraction(self_1500: Translator, other_1232: HomogeneousMagnitude) -> Translator {
    var self_1501: Translator;
    var other_1233: HomogeneousMagnitude;

    self_1501 = self_1500;
    other_1233 = other_1232;
    let _e4: Translator = self_1501;
    let _e6: HomogeneousMagnitude = other_1233;
    return Translator((_e4.g0_ * vec4<f32>(_e6.g0_.x)));
}

fn translator_homogeneous_magnitude_left_anti_contraction(self_1502: Translator, other_1234: HomogeneousMagnitude) -> HomogeneousMagnitude {
    var self_1503: Translator;
    var other_1235: HomogeneousMagnitude;

    self_1503 = self_1502;
    other_1235 = other_1234;
    let _e4: Translator = self_1503;
    let _e8: HomogeneousMagnitude = other_1235;
    return HomogeneousMagnitude((vec2<f32>(_e4.g0_.w) * _e8.g0_));
}

fn translator_homogeneous_magnitude_right_anti_contraction(self_1504: Translator, other_1236: HomogeneousMagnitude) -> Translator {
    var self_1505: Translator;
    var other_1237: HomogeneousMagnitude;

    self_1505 = self_1504;
    other_1237 = other_1236;
    let _e4: Translator = self_1505;
    let _e6: HomogeneousMagnitude = other_1237;
    return Translator((_e4.g0_ * vec4<f32>(_e6.g0_.y)));
}

fn translator_homogeneous_magnitude_anti_scalar_product(self_1506: Translator, other_1238: HomogeneousMagnitude) -> AntiScalar {
    var self_1507: Translator;
    var other_1239: HomogeneousMagnitude;

    self_1507 = self_1506;
    other_1239 = other_1238;
    let _e4: Translator = self_1507;
    let _e7: HomogeneousMagnitude = other_1239;
    return AntiScalar((_e4.g0_.w * _e7.g0_.y));
}

fn translator_point_regressive_product(self_1508: Translator, other_1240: Point) -> Point {
    var self_1509: Translator;
    var other_1241: Point;

    self_1509 = self_1508;
    other_1241 = other_1240;
    let _e4: Translator = self_1509;
    let _e8: Point = other_1241;
    return Point((vec4<f32>(_e4.g0_.w) * _e8.g0_));
}

fn translator_point_outer_product(self_1510: Translator, other_1242: Point) -> Plane {
    var self_1511: Translator;
    var other_1243: Point;

    self_1511 = self_1510;
    other_1243 = other_1242;
    let _e4: Translator = self_1511;
    let _e8: Point = other_1243;
    let _e19: Translator = self_1511;
    let _e23: Point = other_1243;
    let _e35: Translator = self_1511;
    let _e39: Point = other_1243;
    return Plane(((((vec4<f32>(_e4.g0_.y) * _e8.g0_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e35.g0_.x) * _e39.g0_.wxxx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))));
}

fn translator_point_geometric_anti_product(self_1512: Translator, other_1244: Point) -> Point {
    var self_1513: Translator;
    var other_1245: Point;

    self_1513 = self_1512;
    other_1245 = other_1244;
    let _e4: Translator = self_1513;
    let _e8: Point = other_1245;
    let _e11: Translator = self_1513;
    let _e14: Point = other_1245;
    return Point(((vec4<f32>(_e4.g0_.w) * _e8.g0_) + ((_e11.g0_.xyzx * _e14.g0_.wwwx) * vec4<f32>(1.0, 1.0, 1.0, 0.0))));
}

fn translator_point_inner_anti_product(self_1514: Translator, other_1246: Point) -> Point {
    var self_1515: Translator;
    var other_1247: Point;

    self_1515 = self_1514;
    other_1247 = other_1246;
    let _e4: Translator = self_1515;
    let _e8: Point = other_1247;
    return Point((vec4<f32>(_e4.g0_.w) * _e8.g0_));
}

fn translator_point_left_anti_contraction(self_1516: Translator, other_1248: Point) -> Point {
    var self_1517: Translator;
    var other_1249: Point;

    self_1517 = self_1516;
    other_1249 = other_1248;
    let _e4: Translator = self_1517;
    let _e8: Point = other_1249;
    return Point((vec4<f32>(_e4.g0_.w) * _e8.g0_));
}

fn translator_line_add(self_1518: Translator, other_1250: Line) -> Motor {
    var self_1519: Translator;
    var other_1251: Line;

    self_1519 = self_1518;
    other_1251 = other_1250;
    let _e4: Translator = self_1519;
    let _e13: Line = other_1251;
    let _e16: Line = other_1251;
    let _e19: Line = other_1251;
    let _e22: Line = other_1251;
    let _e33: Translator = self_1519;
    let _e36: Translator = self_1519;
    let _e39: Translator = self_1519;
    let _e43: Line = other_1251;
    return Motor(((_e4.g0_.xxxw * vec4<f32>(0.0, 0.0, 0.0, 1.0)) + (vec4<f32>(_e13.g0_.x, _e16.g0_.y, _e19.g0_.z, _e22.g0_.x) * vec4<f32>(1.0, 1.0, 1.0, 0.0))), (vec3<f32>(_e33.g0_.x, _e36.g0_.y, _e39.g0_.z) + _e43.g1_));
}

fn translator_line_sub(self_1520: Translator, other_1252: Line) -> Motor {
    var self_1521: Translator;
    var other_1253: Line;

    self_1521 = self_1520;
    other_1253 = other_1252;
    let _e4: Translator = self_1521;
    let _e13: Line = other_1253;
    let _e16: Line = other_1253;
    let _e19: Line = other_1253;
    let _e22: Line = other_1253;
    let _e33: Translator = self_1521;
    let _e36: Translator = self_1521;
    let _e39: Translator = self_1521;
    let _e43: Line = other_1253;
    return Motor(((_e4.g0_.xxxw * vec4<f32>(0.0, 0.0, 0.0, 1.0)) - (vec4<f32>(_e13.g0_.x, _e16.g0_.y, _e19.g0_.z, _e22.g0_.x) * vec4<f32>(1.0, 1.0, 1.0, 0.0))), (vec3<f32>(_e33.g0_.x, _e36.g0_.y, _e39.g0_.z) - _e43.g1_));
}

fn translator_line_outer_product(self_1522: Translator, other_1254: Line) -> AntiScalar {
    var self_1523: Translator;
    var other_1255: Line;

    self_1523 = self_1522;
    other_1255 = other_1254;
    let _e5: Translator = self_1523;
    let _e8: Line = other_1255;
    let _e13: Translator = self_1523;
    let _e16: Line = other_1255;
    let _e21: Translator = self_1523;
    let _e24: Line = other_1255;
    return AntiScalar((((0.0 - (_e5.g0_.x * _e8.g0_.x)) - (_e13.g0_.y * _e16.g0_.y)) - (_e21.g0_.z * _e24.g0_.z)));
}

fn translator_line_inner_anti_product(self_1524: Translator, other_1256: Line) -> Line {
    var self_1525: Translator;
    var other_1257: Line;

    self_1525 = self_1524;
    other_1257 = other_1256;
    let _e4: Translator = self_1525;
    let _e8: Line = other_1257;
    let _e11: Translator = self_1525;
    let _e15: Line = other_1257;
    return Line((vec3<f32>(_e4.g0_.w) * _e8.g0_), (vec3<f32>(_e11.g0_.w) * _e15.g1_));
}

fn translator_line_left_contraction(self_1526: Translator, other_1258: Line) -> Scalar {
    var self_1527: Translator;
    var other_1259: Line;

    self_1527 = self_1526;
    other_1259 = other_1258;
    let _e5: Translator = self_1527;
    let _e8: Line = other_1259;
    let _e13: Translator = self_1527;
    let _e16: Line = other_1259;
    let _e21: Translator = self_1527;
    let _e24: Line = other_1259;
    return Scalar((((0.0 - (_e5.g0_.x * _e8.g1_.x)) - (_e13.g0_.y * _e16.g1_.y)) - (_e21.g0_.z * _e24.g1_.z)));
}

fn translator_line_left_anti_contraction(self_1528: Translator, other_1260: Line) -> Line {
    var self_1529: Translator;
    var other_1261: Line;

    self_1529 = self_1528;
    other_1261 = other_1260;
    let _e4: Translator = self_1529;
    let _e8: Line = other_1261;
    let _e11: Translator = self_1529;
    let _e15: Line = other_1261;
    return Line((vec3<f32>(_e4.g0_.w) * _e8.g0_), (vec3<f32>(_e11.g0_.w) * _e15.g1_));
}

fn translator_line_scalar_product(self_1530: Translator, other_1262: Line) -> Scalar {
    var self_1531: Translator;
    var other_1263: Line;

    self_1531 = self_1530;
    other_1263 = other_1262;
    let _e5: Translator = self_1531;
    let _e8: Line = other_1263;
    let _e13: Translator = self_1531;
    let _e16: Line = other_1263;
    let _e21: Translator = self_1531;
    let _e24: Line = other_1263;
    return Scalar((((0.0 - (_e5.g0_.x * _e8.g1_.x)) - (_e13.g0_.y * _e16.g1_.y)) - (_e21.g0_.z * _e24.g1_.z)));
}

fn translator_plane_inner_product(self_1532: Translator, other_1264: Plane) -> Point {
    var self_1533: Translator;
    var other_1265: Plane;

    self_1533 = self_1532;
    other_1265 = other_1264;
    let _e4: Translator = self_1533;
    let _e8: Plane = other_1265;
    let _e19: Translator = self_1533;
    let _e23: Plane = other_1265;
    let _e35: Translator = self_1533;
    let _e39: Plane = other_1265;
    let _e52: Translator = self_1533;
    let _e56: Plane = other_1265;
    return Point((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e35.g0_.w) * vec4<f32>(_e39.g0_.w)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e52.g0_.x) * _e56.g0_.wxxx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))));
}

fn translator_plane_inner_anti_product(self_1534: Translator, other_1266: Plane) -> Plane {
    var self_1535: Translator;
    var other_1267: Plane;

    self_1535 = self_1534;
    other_1267 = other_1266;
    let _e4: Translator = self_1535;
    let _e8: Plane = other_1267;
    let _e20: Translator = self_1535;
    let _e24: Plane = other_1267;
    let _e37: Translator = self_1535;
    let _e41: Plane = other_1267;
    let _e45: Translator = self_1535;
    let _e49: Plane = other_1267;
    return Plane((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0))) + ((vec4<f32>(_e20.g0_.z) * vec4<f32>(_e24.g0_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + (vec4<f32>(_e37.g0_.w) * _e41.g0_)) + ((vec4<f32>(_e45.g0_.x) * vec4<f32>(_e49.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))));
}

fn translator_plane_left_contraction(self_1536: Translator, other_1268: Plane) -> Point {
    var self_1537: Translator;
    var other_1269: Plane;

    self_1537 = self_1536;
    other_1269 = other_1268;
    let _e4: Translator = self_1537;
    let _e8: Plane = other_1269;
    let _e19: Translator = self_1537;
    let _e23: Plane = other_1269;
    let _e35: Translator = self_1537;
    let _e39: Plane = other_1269;
    return Point(((((vec4<f32>(_e4.g0_.y) * _e8.g0_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e35.g0_.x) * _e39.g0_.wxxx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))));
}

fn translator_plane_left_anti_contraction(self_1538: Translator, other_1270: Plane) -> Plane {
    var self_1539: Translator;
    var other_1271: Plane;

    self_1539 = self_1538;
    other_1271 = other_1270;
    let _e4: Translator = self_1539;
    let _e8: Plane = other_1271;
    return Plane((vec4<f32>(_e4.g0_.w) * _e8.g0_));
}

fn translator_motor_add(self_1540: Translator, other_1272: Motor) -> Motor {
    var self_1541: Translator;
    var other_1273: Motor;

    self_1541 = self_1540;
    other_1273 = other_1272;
    let _e4: Translator = self_1541;
    let _e13: Motor = other_1273;
    let _e16: Translator = self_1541;
    let _e19: Translator = self_1541;
    let _e22: Translator = self_1541;
    let _e26: Motor = other_1273;
    return Motor(((_e4.g0_.xxxw * vec4<f32>(0.0, 0.0, 0.0, 1.0)) + _e13.g0_), (vec3<f32>(_e16.g0_.x, _e19.g0_.y, _e22.g0_.z) + _e26.g1_));
}

fn translator_motor_sub(self_1542: Translator, other_1274: Motor) -> Motor {
    var self_1543: Translator;
    var other_1275: Motor;

    self_1543 = self_1542;
    other_1275 = other_1274;
    let _e4: Translator = self_1543;
    let _e13: Motor = other_1275;
    let _e16: Translator = self_1543;
    let _e19: Translator = self_1543;
    let _e22: Translator = self_1543;
    let _e26: Motor = other_1275;
    return Motor(((_e4.g0_.xxxw * vec4<f32>(0.0, 0.0, 0.0, 1.0)) - _e13.g0_), (vec3<f32>(_e16.g0_.x, _e19.g0_.y, _e22.g0_.z) - _e26.g1_));
}

fn translator_motor_outer_product(self_1544: Translator, other_1276: Motor) -> AntiScalar {
    var self_1545: Translator;
    var other_1277: Motor;

    self_1545 = self_1544;
    other_1277 = other_1276;
    let _e5: Translator = self_1545;
    let _e8: Motor = other_1277;
    let _e13: Translator = self_1545;
    let _e16: Motor = other_1277;
    let _e21: Translator = self_1545;
    let _e24: Motor = other_1277;
    return AntiScalar((((0.0 - (_e5.g0_.x * _e8.g0_.x)) - (_e13.g0_.y * _e16.g0_.y)) - (_e21.g0_.z * _e24.g0_.z)));
}

fn translator_motor_inner_anti_product(self_1546: Translator, other_1278: Motor) -> Motor {
    var self_1547: Translator;
    var other_1279: Motor;

    self_1547 = self_1546;
    other_1279 = other_1278;
    let _e4: Translator = self_1547;
    let _e8: Motor = other_1279;
    let _e11: Translator = self_1547;
    let _e15: Motor = other_1279;
    let _e18: Translator = self_1547;
    let _e21: Translator = self_1547;
    let _e24: Translator = self_1547;
    let _e28: Motor = other_1279;
    return Motor((vec4<f32>(_e4.g0_.w) * _e8.g0_), ((vec3<f32>(_e11.g0_.w) * _e15.g1_) + (vec3<f32>(_e18.g0_.x, _e21.g0_.y, _e24.g0_.z) * vec3<f32>(_e28.g0_.w))));
}

fn translator_motor_left_anti_contraction(self_1548: Translator, other_1280: Motor) -> Motor {
    var self_1549: Translator;
    var other_1281: Motor;

    self_1549 = self_1548;
    other_1281 = other_1280;
    let _e4: Translator = self_1549;
    let _e8: Motor = other_1281;
    let _e11: Translator = self_1549;
    let _e15: Motor = other_1281;
    return Motor((vec4<f32>(_e4.g0_.w) * _e8.g0_), (vec3<f32>(_e11.g0_.w) * _e15.g1_));
}

fn translator_motor_right_anti_contraction(self_1550: Translator, other_1282: Motor) -> Translator {
    var self_1551: Translator;
    var other_1283: Motor;

    self_1551 = self_1550;
    other_1283 = other_1282;
    let _e4: Translator = self_1551;
    let _e6: Motor = other_1283;
    return Translator((_e4.g0_ * vec4<f32>(_e6.g0_.w)));
}

fn translator_motor_scalar_product(self_1552: Translator, other_1284: Motor) -> Scalar {
    var self_1553: Translator;
    var other_1285: Motor;

    self_1553 = self_1552;
    other_1285 = other_1284;
    let _e5: Translator = self_1553;
    let _e8: Motor = other_1285;
    let _e13: Translator = self_1553;
    let _e16: Motor = other_1285;
    let _e21: Translator = self_1553;
    let _e24: Motor = other_1285;
    return Scalar((((0.0 - (_e5.g0_.x * _e8.g1_.x)) - (_e13.g0_.y * _e16.g1_.y)) - (_e21.g0_.z * _e24.g1_.z)));
}

fn translator_motor_anti_scalar_product(self_1554: Translator, other_1286: Motor) -> AntiScalar {
    var self_1555: Translator;
    var other_1287: Motor;

    self_1555 = self_1554;
    other_1287 = other_1286;
    let _e4: Translator = self_1555;
    let _e7: Motor = other_1287;
    return AntiScalar((_e4.g0_.w * _e7.g0_.w));
}

fn translator_rotor_add(self_1556: Translator, other_1288: Rotor) -> Motor {
    var self_1557: Translator;
    var other_1289: Rotor;

    self_1557 = self_1556;
    other_1289 = other_1288;
    let _e4: Translator = self_1557;
    let _e13: Rotor = other_1289;
    let _e16: Translator = self_1557;
    let _e19: Translator = self_1557;
    let _e22: Translator = self_1557;
    return Motor(((_e4.g0_.xxxw * vec4<f32>(0.0, 0.0, 0.0, 1.0)) + _e13.g0_), vec3<f32>(_e16.g0_.x, _e19.g0_.y, _e22.g0_.z));
}

fn translator_rotor_sub(self_1558: Translator, other_1290: Rotor) -> Motor {
    var self_1559: Translator;
    var other_1291: Rotor;

    self_1559 = self_1558;
    other_1291 = other_1290;
    let _e4: Translator = self_1559;
    let _e13: Rotor = other_1291;
    let _e16: Translator = self_1559;
    let _e19: Translator = self_1559;
    let _e22: Translator = self_1559;
    return Motor(((_e4.g0_.xxxw * vec4<f32>(0.0, 0.0, 0.0, 1.0)) - _e13.g0_), vec3<f32>(_e16.g0_.x, _e19.g0_.y, _e22.g0_.z));
}

fn translator_rotor_geometric_product(self_1560: Translator, other_1292: Rotor) -> Rotor {
    var self_1561: Translator;
    var other_1293: Rotor;

    self_1561 = self_1560;
    other_1293 = other_1292;
    let _e4: Translator = self_1561;
    let _e8: Rotor = other_1293;
    let _e20: Translator = self_1561;
    let _e24: Rotor = other_1293;
    let _e37: Translator = self_1561;
    let _e41: Rotor = other_1293;
    return Rotor(((((vec4<f32>(_e4.g0_.x) * _e8.g0_.wzyx) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0))) + ((vec4<f32>(_e20.g0_.y) * _e24.g0_.zwxy) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e37.g0_.z) * _e41.g0_.yxwz) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))));
}

fn translator_rotor_outer_product(self_1562: Translator, other_1294: Rotor) -> AntiScalar {
    var self_1563: Translator;
    var other_1295: Rotor;

    self_1563 = self_1562;
    other_1295 = other_1294;
    let _e5: Translator = self_1563;
    let _e8: Rotor = other_1295;
    let _e13: Translator = self_1563;
    let _e16: Rotor = other_1295;
    let _e21: Translator = self_1563;
    let _e24: Rotor = other_1295;
    return AntiScalar((((0.0 - (_e5.g0_.x * _e8.g0_.x)) - (_e13.g0_.y * _e16.g0_.y)) - (_e21.g0_.z * _e24.g0_.z)));
}

fn translator_rotor_inner_anti_product(self_1564: Translator, other_1296: Rotor) -> Motor {
    var self_1565: Translator;
    var other_1297: Rotor;

    self_1565 = self_1564;
    other_1297 = other_1296;
    let _e4: Translator = self_1565;
    let _e8: Rotor = other_1297;
    let _e11: Translator = self_1565;
    let _e14: Translator = self_1565;
    let _e17: Translator = self_1565;
    let _e21: Rotor = other_1297;
    return Motor((vec4<f32>(_e4.g0_.w) * _e8.g0_), (vec3<f32>(_e11.g0_.x, _e14.g0_.y, _e17.g0_.z) * vec3<f32>(_e21.g0_.w)));
}

fn translator_rotor_left_anti_contraction(self_1566: Translator, other_1298: Rotor) -> Rotor {
    var self_1567: Translator;
    var other_1299: Rotor;

    self_1567 = self_1566;
    other_1299 = other_1298;
    let _e4: Translator = self_1567;
    let _e8: Rotor = other_1299;
    return Rotor((vec4<f32>(_e4.g0_.w) * _e8.g0_));
}

fn translator_rotor_right_anti_contraction(self_1568: Translator, other_1300: Rotor) -> Translator {
    var self_1569: Translator;
    var other_1301: Rotor;

    self_1569 = self_1568;
    other_1301 = other_1300;
    let _e4: Translator = self_1569;
    let _e6: Rotor = other_1301;
    return Translator((_e4.g0_ * vec4<f32>(_e6.g0_.w)));
}

fn translator_rotor_anti_scalar_product(self_1570: Translator, other_1302: Rotor) -> AntiScalar {
    var self_1571: Translator;
    var other_1303: Rotor;

    self_1571 = self_1570;
    other_1303 = other_1302;
    let _e4: Translator = self_1571;
    let _e7: Rotor = other_1303;
    return AntiScalar((_e4.g0_.w * _e7.g0_.w));
}

fn translator_translator_add(self_1572: Translator, other_1304: Translator) -> Translator {
    var self_1573: Translator;
    var other_1305: Translator;

    self_1573 = self_1572;
    other_1305 = other_1304;
    let _e4: Translator = self_1573;
    let _e6: Translator = other_1305;
    return Translator((_e4.g0_ + _e6.g0_));
}

fn translator_translator_sub(self_1574: Translator, other_1306: Translator) -> Translator {
    var self_1575: Translator;
    var other_1307: Translator;

    self_1575 = self_1574;
    other_1307 = other_1306;
    let _e4: Translator = self_1575;
    let _e6: Translator = other_1307;
    return Translator((_e4.g0_ - _e6.g0_));
}

fn translator_translator_mul(self_1576: Translator, other_1308: Translator) -> Translator {
    var self_1577: Translator;
    var other_1309: Translator;

    self_1577 = self_1576;
    other_1309 = other_1308;
    let _e4: Translator = self_1577;
    let _e6: Translator = other_1309;
    return Translator((_e4.g0_ * _e6.g0_));
}

fn translator_translator_div(self_1578: Translator, other_1310: Translator) -> Translator {
    var self_1579: Translator;
    var other_1311: Translator;

    self_1579 = self_1578;
    other_1311 = other_1310;
    let _e4: Translator = self_1579;
    let _e7: Translator = self_1579;
    let _e10: Translator = self_1579;
    let _e13: Translator = self_1579;
    let _e23: Translator = other_1311;
    let _e26: Translator = other_1311;
    let _e29: Translator = other_1311;
    let _e32: Translator = other_1311;
    return Translator((((vec4<f32>(_e4.g0_.x, _e7.g0_.y, _e10.g0_.z, _e13.g0_.w) * vec4<f32>(1.0, 1.0, 1.0, 1.0)) / vec4<f32>(_e23.g0_.x, _e26.g0_.y, _e29.g0_.z, _e32.g0_.w)) * vec4<f32>(1.0, 1.0, 1.0, 1.0)));
}

fn translator_translator_regressive_product(self_1580: Translator, other_1312: Translator) -> Translator {
    var self_1581: Translator;
    var other_1313: Translator;

    self_1581 = self_1580;
    other_1313 = other_1312;
    let _e4: Translator = self_1581;
    let _e8: Translator = other_1313;
    let _e11: Translator = self_1581;
    let _e14: Translator = other_1313;
    return Translator(((vec4<f32>(_e4.g0_.w) * _e8.g0_) + ((_e11.g0_.xyzx * _e14.g0_.wwwx) * vec4<f32>(1.0, 1.0, 1.0, 0.0))));
}

fn translator_translator_geometric_anti_product(self_1582: Translator, other_1314: Translator) -> Translator {
    var self_1583: Translator;
    var other_1315: Translator;

    self_1583 = self_1582;
    other_1315 = other_1314;
    let _e4: Translator = self_1583;
    let _e8: Translator = other_1315;
    let _e11: Translator = self_1583;
    let _e14: Translator = other_1315;
    return Translator(((vec4<f32>(_e4.g0_.w) * _e8.g0_) + ((_e11.g0_.xyzx * _e14.g0_.wwwx) * vec4<f32>(1.0, 1.0, 1.0, 0.0))));
}

fn translator_translator_inner_anti_product(self_1584: Translator, other_1316: Translator) -> Translator {
    var self_1585: Translator;
    var other_1317: Translator;

    self_1585 = self_1584;
    other_1317 = other_1316;
    let _e4: Translator = self_1585;
    let _e8: Translator = other_1317;
    let _e11: Translator = self_1585;
    let _e14: Translator = other_1317;
    return Translator(((vec4<f32>(_e4.g0_.w) * _e8.g0_) + ((_e11.g0_.xyzx * _e14.g0_.wwwx) * vec4<f32>(1.0, 1.0, 1.0, 0.0))));
}

fn translator_translator_left_anti_contraction(self_1586: Translator, other_1318: Translator) -> Translator {
    var self_1587: Translator;
    var other_1319: Translator;

    self_1587 = self_1586;
    other_1319 = other_1318;
    let _e4: Translator = self_1587;
    let _e8: Translator = other_1319;
    return Translator((vec4<f32>(_e4.g0_.w) * _e8.g0_));
}

fn translator_translator_right_anti_contraction(self_1588: Translator, other_1320: Translator) -> Translator {
    var self_1589: Translator;
    var other_1321: Translator;

    self_1589 = self_1588;
    other_1321 = other_1320;
    let _e4: Translator = self_1589;
    let _e6: Translator = other_1321;
    return Translator((_e4.g0_ * vec4<f32>(_e6.g0_.w)));
}

fn translator_translator_scalar_product(self_1590: Translator, other_1322: Translator) -> Scalar {
    var self_1591: Translator;
    var other_1323: Translator;

    self_1591 = self_1590;
    other_1323 = other_1322;
    let _e5: Translator = self_1591;
    let _e8: Translator = other_1323;
    let _e13: Translator = self_1591;
    let _e16: Translator = other_1323;
    let _e21: Translator = self_1591;
    let _e24: Translator = other_1323;
    return Scalar((((0.0 - (_e5.g0_.x * _e8.g0_.x)) - (_e13.g0_.y * _e16.g0_.y)) - (_e21.g0_.z * _e24.g0_.z)));
}

fn translator_translator_anti_scalar_product(self_1592: Translator, other_1324: Translator) -> AntiScalar {
    var self_1593: Translator;
    var other_1325: Translator;

    self_1593 = self_1592;
    other_1325 = other_1324;
    let _e4: Translator = self_1593;
    let _e7: Translator = other_1325;
    return AntiScalar((_e4.g0_.w * _e7.g0_.w));
}

fn translator_flector_geometric_product(self_1594: Translator, other_1326: Flector) -> Flector {
    var self_1595: Translator;
    var other_1327: Flector;

    self_1595 = self_1594;
    other_1327 = other_1326;
    let _e4: Translator = self_1595;
    let _e8: Flector = other_1327;
    let _e11: Flector = other_1327;
    let _e14: Flector = other_1327;
    let _e17: Flector = other_1327;
    let _e30: Translator = self_1595;
    let _e34: Flector = other_1327;
    let _e37: Flector = other_1327;
    let _e40: Flector = other_1327;
    let _e43: Flector = other_1327;
    let _e57: Translator = self_1595;
    let _e61: Flector = other_1327;
    let _e64: Flector = other_1327;
    let _e67: Flector = other_1327;
    let _e70: Flector = other_1327;
    let _e84: Translator = self_1595;
    let _e87: Flector = other_1327;
    let _e99: Translator = self_1595;
    let _e103: Flector = other_1327;
    let _e106: Flector = other_1327;
    let _e109: Flector = other_1327;
    let _e112: Flector = other_1327;
    let _e125: Translator = self_1595;
    let _e129: Flector = other_1327;
    let _e132: Flector = other_1327;
    let _e135: Flector = other_1327;
    let _e138: Flector = other_1327;
    let _e152: Translator = self_1595;
    let _e156: Flector = other_1327;
    let _e159: Flector = other_1327;
    let _e162: Flector = other_1327;
    let _e165: Flector = other_1327;
    let _e179: Translator = self_1595;
    let _e182: Flector = other_1327;
    return Flector((((((vec4<f32>(_e4.g0_.x) * vec4<f32>(_e8.g1_.w, _e11.g0_.z, _e14.g0_.y, _e17.g1_.x)) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0))) + ((vec4<f32>(_e30.g0_.y) * vec4<f32>(_e34.g0_.z, _e37.g1_.w, _e40.g0_.x, _e43.g1_.y)) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e57.g0_.z) * vec4<f32>(_e61.g0_.y, _e64.g0_.x, _e67.g1_.w, _e70.g1_.z)) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))) + ((_e84.g0_.xxxw * _e87.g1_.xxxw) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))), (((((vec4<f32>(_e99.g0_.x) * vec4<f32>(_e103.g0_.w, _e106.g1_.z, _e109.g1_.y, _e112.g0_.x)) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0))) + ((vec4<f32>(_e125.g0_.y) * vec4<f32>(_e129.g1_.z, _e132.g0_.w, _e135.g1_.x, _e138.g0_.y)) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e152.g0_.z) * vec4<f32>(_e156.g1_.y, _e159.g1_.x, _e162.g0_.w, _e165.g0_.z)) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))) + ((_e179.g0_.wwwx * _e182.g0_.xyzx) * vec4<f32>(-(1.0), -(1.0), -(1.0), 0.0))));
}

fn translator_flector_regressive_product(self_1596: Translator, other_1328: Flector) -> Flector {
    var self_1597: Translator;
    var other_1329: Flector;

    self_1597 = self_1596;
    other_1329 = other_1328;
    let _e4: Translator = self_1597;
    let _e8: Flector = other_1329;
    let _e19: Translator = self_1597;
    let _e23: Flector = other_1329;
    let _e35: Translator = self_1597;
    let _e39: Flector = other_1329;
    let _e43: Translator = self_1597;
    let _e47: Flector = other_1329;
    let _e59: Translator = self_1597;
    let _e63: Flector = other_1329;
    return Flector((((((vec4<f32>(_e4.g0_.y) * _e8.g1_.zzxz) * vec4<f32>(1.0, 0.0, -(1.0), 0.0)) + ((vec4<f32>(_e19.g0_.z) * _e23.g1_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))) + (vec4<f32>(_e35.g0_.w) * _e39.g0_)) + ((vec4<f32>(_e43.g0_.x) * _e47.g1_.xzyx) * vec4<f32>(0.0, -(1.0), 1.0, 0.0))), (vec4<f32>(_e59.g0_.w) * _e63.g1_));
}

fn translator_flector_outer_product(self_1598: Translator, other_1330: Flector) -> Plane {
    var self_1599: Translator;
    var other_1331: Flector;

    self_1599 = self_1598;
    other_1331 = other_1330;
    let _e4: Translator = self_1599;
    let _e8: Flector = other_1331;
    let _e19: Translator = self_1599;
    let _e23: Flector = other_1331;
    let _e35: Translator = self_1599;
    let _e39: Flector = other_1331;
    return Plane(((((vec4<f32>(_e4.g0_.y) * _e8.g0_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e35.g0_.x) * _e39.g0_.wxxx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))));
}

fn translator_flector_geometric_anti_product(self_1600: Translator, other_1332: Flector) -> Flector {
    var self_1601: Translator;
    var other_1333: Flector;

    self_1601 = self_1600;
    other_1333 = other_1332;
    let _e4: Translator = self_1601;
    let _e8: Flector = other_1333;
    let _e11: Flector = other_1333;
    let _e14: Flector = other_1333;
    let _e17: Flector = other_1333;
    let _e29: Translator = self_1601;
    let _e33: Flector = other_1333;
    let _e36: Flector = other_1333;
    let _e39: Flector = other_1333;
    let _e42: Flector = other_1333;
    let _e55: Translator = self_1601;
    let _e59: Flector = other_1333;
    let _e63: Translator = self_1601;
    let _e67: Flector = other_1333;
    let _e70: Flector = other_1333;
    let _e73: Flector = other_1333;
    let _e76: Flector = other_1333;
    let _e89: Translator = self_1601;
    let _e93: Flector = other_1333;
    let _e105: Translator = self_1601;
    let _e109: Flector = other_1333;
    let _e122: Translator = self_1601;
    let _e126: Flector = other_1333;
    let _e130: Translator = self_1601;
    let _e134: Flector = other_1333;
    return Flector((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g1_.z, _e11.g0_.w, _e14.g1_.x, _e17.g1_.z)) * vec4<f32>(1.0, 1.0, -(1.0), 0.0)) + ((vec4<f32>(_e29.g0_.z) * vec4<f32>(_e33.g1_.y, _e36.g1_.x, _e39.g0_.w, _e42.g1_.y)) * vec4<f32>(-(1.0), 1.0, 1.0, 0.0))) + (vec4<f32>(_e55.g0_.w) * _e59.g0_)) + ((vec4<f32>(_e63.g0_.x) * vec4<f32>(_e67.g0_.w, _e70.g1_.z, _e73.g1_.y, _e76.g0_.x)) * vec4<f32>(1.0, -(1.0), 1.0, 0.0))), (((((vec4<f32>(_e89.g0_.y) * vec4<f32>(_e93.g1_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0))) + ((vec4<f32>(_e105.g0_.z) * vec4<f32>(_e109.g1_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + (vec4<f32>(_e122.g0_.w) * _e126.g1_)) + ((vec4<f32>(_e130.g0_.x) * vec4<f32>(_e134.g1_.x)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))));
}

fn translator_flector_inner_anti_product(self_1602: Translator, other_1334: Flector) -> Flector {
    var self_1603: Translator;
    var other_1335: Flector;

    self_1603 = self_1602;
    other_1335 = other_1334;
    let _e4: Translator = self_1603;
    let _e8: Flector = other_1335;
    let _e11: Translator = self_1603;
    let _e15: Flector = other_1335;
    let _e27: Translator = self_1603;
    let _e31: Flector = other_1335;
    let _e44: Translator = self_1603;
    let _e48: Flector = other_1335;
    let _e52: Translator = self_1603;
    let _e56: Flector = other_1335;
    return Flector((vec4<f32>(_e4.g0_.w) * _e8.g0_), (((((vec4<f32>(_e11.g0_.y) * vec4<f32>(_e15.g1_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0))) + ((vec4<f32>(_e27.g0_.z) * vec4<f32>(_e31.g1_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + (vec4<f32>(_e44.g0_.w) * _e48.g1_)) + ((vec4<f32>(_e52.g0_.x) * vec4<f32>(_e56.g1_.x)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))));
}

fn translator_flector_left_contraction(self_1604: Translator, other_1336: Flector) -> Point {
    var self_1605: Translator;
    var other_1337: Flector;

    self_1605 = self_1604;
    other_1337 = other_1336;
    let _e4: Translator = self_1605;
    let _e8: Flector = other_1337;
    let _e19: Translator = self_1605;
    let _e23: Flector = other_1337;
    let _e35: Translator = self_1605;
    let _e39: Flector = other_1337;
    return Point(((((vec4<f32>(_e4.g0_.y) * _e8.g1_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e19.g0_.z) * _e23.g1_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e35.g0_.x) * _e39.g1_.wxxx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))));
}

fn translator_flector_left_anti_contraction(self_1606: Translator, other_1338: Flector) -> Flector {
    var self_1607: Translator;
    var other_1339: Flector;

    self_1607 = self_1606;
    other_1339 = other_1338;
    let _e4: Translator = self_1607;
    let _e8: Flector = other_1339;
    let _e11: Translator = self_1607;
    let _e15: Flector = other_1339;
    return Flector((vec4<f32>(_e4.g0_.w) * _e8.g0_), (vec4<f32>(_e11.g0_.w) * _e15.g1_));
}

fn translator_multi_vector_scalar_product(self_1608: Translator, other_1340: MultiVector) -> Scalar {
    var self_1609: Translator;
    var other_1341: MultiVector;

    self_1609 = self_1608;
    other_1341 = other_1340;
    let _e5: Translator = self_1609;
    let _e8: MultiVector = other_1341;
    let _e13: Translator = self_1609;
    let _e16: MultiVector = other_1341;
    let _e21: Translator = self_1609;
    let _e24: MultiVector = other_1341;
    return Scalar((((0.0 - (_e5.g0_.x * _e8.g3_.x)) - (_e13.g0_.y * _e16.g3_.y)) - (_e21.g0_.z * _e24.g3_.z)));
}

fn translator_multi_vector_anti_scalar_product(self_1610: Translator, other_1342: MultiVector) -> AntiScalar {
    var self_1611: Translator;
    var other_1343: MultiVector;

    self_1611 = self_1610;
    other_1343 = other_1342;
    let _e4: Translator = self_1611;
    let _e7: MultiVector = other_1343;
    return AntiScalar((_e4.g0_.w * _e7.g0_.y));
}

fn translator_squared_magnitude(self_1612: Translator) -> Scalar {
    var self_1613: Translator;

    self_1613 = self_1612;
    let _e2: Translator = self_1613;
    let _e3: Translator = self_1613;
    let _e4: Translator = translator_reversal(_e3);
    let _e5: Scalar = translator_translator_scalar_product(_e2, _e4);
    return _e5;
}

fn translator_magnitude(self_1614: Translator) -> Scalar {
    var self_1615: Translator;

    self_1615 = self_1614;
    let _e2: Translator = self_1615;
    let _e3: Scalar = translator_squared_magnitude(_e2);
    return Scalar(sqrt(_e3.g0_));
}

fn translator_bulk_norm(self_1616: Translator) -> Scalar {
    var self_1617: Translator;

    self_1617 = self_1616;
    let _e2: Translator = self_1617;
    let _e3: Scalar = translator_squared_magnitude(_e2);
    return Scalar(sqrt(_e3.g0_));
}

fn translator_squared_anti_magnitude(self_1618: Translator) -> AntiScalar {
    var self_1619: Translator;

    self_1619 = self_1618;
    let _e2: Translator = self_1619;
    let _e3: Translator = self_1619;
    let _e4: Translator = translator_anti_reversal(_e3);
    let _e5: AntiScalar = translator_translator_anti_scalar_product(_e2, _e4);
    return _e5;
}

fn translator_weight_norm(self_1620: Translator) -> AntiScalar {
    var self_1621: Translator;

    self_1621 = self_1620;
    let _e2: Translator = self_1621;
    let _e3: AntiScalar = translator_squared_anti_magnitude(_e2);
    return AntiScalar(sqrt(_e3.g0_));
}

fn translator_geometric_norm(self_1622: Translator) -> HomogeneousMagnitude {
    var self_1623: Translator;

    self_1623 = self_1622;
    let _e2: Translator = self_1623;
    let _e3: Scalar = translator_bulk_norm(_e2);
    let _e4: Translator = self_1623;
    let _e5: AntiScalar = translator_weight_norm(_e4);
    return (_e3 + _e5);
}

fn translator_scale(self_1624: Translator, other_1344: f32) -> Translator {
    var self_1625: Translator;
    var other_1345: f32;

    self_1625 = self_1624;
    other_1345 = other_1344;
    let _e4: Translator = self_1625;
    let _e5: f32 = other_1345;
    let _e7: Translator = translator_scalar_geometric_product(_e4, Scalar(_e5));
    return _e7;
}

fn translator_signum(self_1626: Translator) -> Translator {
    var self_1627: Translator;

    self_1627 = self_1626;
    let _e2: Translator = self_1627;
    let _e3: Translator = self_1627;
    let _e4: Scalar = translator_magnitude(_e3);
    let _e9: Translator = translator_scalar_geometric_product(_e2, Scalar((1.0 / _e4.g0_)));
    return _e9;
}

fn translator_inverse(self_1628: Translator) -> Translator {
    var self_1629: Translator;

    self_1629 = self_1628;
    let _e2: Translator = self_1629;
    let _e3: Translator = translator_reversal(_e2);
    let _e4: Translator = self_1629;
    let _e5: Scalar = translator_squared_magnitude(_e4);
    let _e10: Translator = translator_scalar_geometric_product(_e3, Scalar((1.0 / _e5.g0_)));
    return _e10;
}

fn translator_unitize(self_1630: Translator) -> Translator {
    var self_1631: Translator;

    self_1631 = self_1630;
    let _e2: Translator = self_1631;
    let _e3: Translator = self_1631;
    let _e4: AntiScalar = translator_weight_norm(_e3);
    let _e9: Translator = translator_scalar_geometric_product(_e2, Scalar((1.0 / _e4.g0_)));
    return _e9;
}

fn flector_zero() -> Flector {
    return Flector(vec4<f32>(0.0), vec4<f32>(0.0));
}

fn flector_one() -> Flector {
    return Flector(vec4<f32>(0.0), vec4<f32>(0.0));
}

fn flector_neg(self_1632: Flector) -> Flector {
    var self_1633: Flector;

    self_1633 = self_1632;
    let _e2: Flector = self_1633;
    let _e8: Flector = self_1633;
    return Flector((_e2.g0_ * vec4<f32>(-(1.0))), (_e8.g1_ * vec4<f32>(-(1.0))));
}

fn flector_automorphism(self_1634: Flector) -> Flector {
    var self_1635: Flector;

    self_1635 = self_1634;
    let _e2: Flector = self_1635;
    let _e8: Flector = self_1635;
    return Flector((_e2.g0_ * vec4<f32>(-(1.0))), (_e8.g1_ * vec4<f32>(-(1.0))));
}

fn flector_reversal(self_1636: Flector) -> Flector {
    var self_1637: Flector;

    self_1637 = self_1636;
    let _e2: Flector = self_1637;
    let _e4: Flector = self_1637;
    return Flector(_e2.g0_, (_e4.g1_ * vec4<f32>(-(1.0))));
}

fn flector_conjugation(self_1638: Flector) -> Flector {
    var self_1639: Flector;

    self_1639 = self_1638;
    let _e2: Flector = self_1639;
    let _e8: Flector = self_1639;
    return Flector((_e2.g0_ * vec4<f32>(-(1.0))), _e8.g1_);
}

fn flector_dual(self_1640: Flector) -> Flector {
    var self_1641: Flector;

    self_1641 = self_1640;
    let _e2: Flector = self_1641;
    let _e8: Flector = self_1641;
    return Flector((_e2.g1_ * vec4<f32>(-(1.0))), _e8.g0_);
}

fn flector_anti_reversal(self_1642: Flector) -> Flector {
    var self_1643: Flector;

    self_1643 = self_1642;
    let _e2: Flector = self_1643;
    let _e8: Flector = self_1643;
    return Flector((_e2.g0_ * vec4<f32>(-(1.0))), _e8.g1_);
}

fn flector_right_complement(self_1644: Flector) -> Flector {
    var self_1645: Flector;

    self_1645 = self_1644;
    let _e2: Flector = self_1645;
    let _e8: Flector = self_1645;
    return Flector((_e2.g1_ * vec4<f32>(-(1.0))), _e8.g0_);
}

fn flector_left_complement(self_1646: Flector) -> Flector {
    var self_1647: Flector;

    self_1647 = self_1646;
    let _e2: Flector = self_1647;
    let _e4: Flector = self_1647;
    return Flector(_e2.g1_, (_e4.g0_ * vec4<f32>(-(1.0))));
}

fn flector_double_complement(self_1648: Flector) -> Flector {
    var self_1649: Flector;

    self_1649 = self_1648;
    let _e2: Flector = self_1649;
    let _e8: Flector = self_1649;
    return Flector((_e2.g0_ * vec4<f32>(-(1.0))), (_e8.g1_ * vec4<f32>(-(1.0))));
}

fn flector_scalar_geometric_product(self_1650: Flector, other_1346: Scalar) -> Flector {
    var self_1651: Flector;
    var other_1347: Scalar;

    self_1651 = self_1650;
    other_1347 = other_1346;
    let _e4: Flector = self_1651;
    let _e6: Scalar = other_1347;
    let _e10: Flector = self_1651;
    let _e12: Scalar = other_1347;
    return Flector((_e4.g0_ * vec4<f32>(_e6.g0_)), (_e10.g1_ * vec4<f32>(_e12.g0_)));
}

fn flector_scalar_outer_product(self_1652: Flector, other_1348: Scalar) -> Flector {
    var self_1653: Flector;
    var other_1349: Scalar;

    self_1653 = self_1652;
    other_1349 = other_1348;
    let _e4: Flector = self_1653;
    let _e6: Scalar = other_1349;
    let _e10: Flector = self_1653;
    let _e12: Scalar = other_1349;
    return Flector((_e4.g0_ * vec4<f32>(_e6.g0_)), (_e10.g1_ * vec4<f32>(_e12.g0_)));
}

fn flector_scalar_inner_product(self_1654: Flector, other_1350: Scalar) -> Flector {
    var self_1655: Flector;
    var other_1351: Scalar;

    self_1655 = self_1654;
    other_1351 = other_1350;
    let _e4: Flector = self_1655;
    let _e6: Scalar = other_1351;
    let _e10: Flector = self_1655;
    let _e12: Scalar = other_1351;
    return Flector((_e4.g0_ * vec4<f32>(_e6.g0_)), (_e10.g1_ * vec4<f32>(_e12.g0_)));
}

fn flector_scalar_right_contraction(self_1656: Flector, other_1352: Scalar) -> Flector {
    var self_1657: Flector;
    var other_1353: Scalar;

    self_1657 = self_1656;
    other_1353 = other_1352;
    let _e4: Flector = self_1657;
    let _e6: Scalar = other_1353;
    let _e10: Flector = self_1657;
    let _e12: Scalar = other_1353;
    return Flector((_e4.g0_ * vec4<f32>(_e6.g0_)), (_e10.g1_ * vec4<f32>(_e12.g0_)));
}

fn flector_anti_scalar_regressive_product(self_1658: Flector, other_1354: AntiScalar) -> Flector {
    var self_1659: Flector;
    var other_1355: AntiScalar;

    self_1659 = self_1658;
    other_1355 = other_1354;
    let _e4: Flector = self_1659;
    let _e6: AntiScalar = other_1355;
    let _e10: Flector = self_1659;
    let _e12: AntiScalar = other_1355;
    return Flector((_e4.g0_ * vec4<f32>(_e6.g0_)), (_e10.g1_ * vec4<f32>(_e12.g0_)));
}

fn flector_anti_scalar_geometric_anti_product(self_1660: Flector, other_1356: AntiScalar) -> Flector {
    var self_1661: Flector;
    var other_1357: AntiScalar;

    self_1661 = self_1660;
    other_1357 = other_1356;
    let _e4: Flector = self_1661;
    let _e6: AntiScalar = other_1357;
    let _e10: Flector = self_1661;
    let _e12: AntiScalar = other_1357;
    return Flector((_e4.g0_ * vec4<f32>(_e6.g0_)), (_e10.g1_ * vec4<f32>(_e12.g0_)));
}

fn flector_anti_scalar_inner_anti_product(self_1662: Flector, other_1358: AntiScalar) -> Flector {
    var self_1663: Flector;
    var other_1359: AntiScalar;

    self_1663 = self_1662;
    other_1359 = other_1358;
    let _e4: Flector = self_1663;
    let _e6: AntiScalar = other_1359;
    let _e10: Flector = self_1663;
    let _e12: AntiScalar = other_1359;
    return Flector((_e4.g0_ * vec4<f32>(_e6.g0_)), (_e10.g1_ * vec4<f32>(_e12.g0_)));
}

fn flector_anti_scalar_right_anti_contraction(self_1664: Flector, other_1360: AntiScalar) -> Flector {
    var self_1665: Flector;
    var other_1361: AntiScalar;

    self_1665 = self_1664;
    other_1361 = other_1360;
    let _e4: Flector = self_1665;
    let _e6: AntiScalar = other_1361;
    let _e10: Flector = self_1665;
    let _e12: AntiScalar = other_1361;
    return Flector((_e4.g0_ * vec4<f32>(_e6.g0_)), (_e10.g1_ * vec4<f32>(_e12.g0_)));
}

fn flector_homogeneous_magnitude_geometric_product(self_1666: Flector, other_1362: HomogeneousMagnitude) -> Flector {
    var self_1667: Flector;
    var other_1363: HomogeneousMagnitude;

    self_1667 = self_1666;
    other_1363 = other_1362;
    let _e4: Flector = self_1667;
    let _e8: HomogeneousMagnitude = other_1363;
    let _e19: Flector = self_1667;
    let _e21: HomogeneousMagnitude = other_1363;
    let _e27: Flector = self_1667;
    let _e31: HomogeneousMagnitude = other_1363;
    let _e42: Flector = self_1667;
    let _e46: HomogeneousMagnitude = other_1363;
    let _e58: Flector = self_1667;
    let _e62: HomogeneousMagnitude = other_1363;
    let _e74: Flector = self_1667;
    let _e77: Flector = self_1667;
    let _e80: Flector = self_1667;
    let _e83: Flector = self_1667;
    let _e87: HomogeneousMagnitude = other_1363;
    let _e90: HomogeneousMagnitude = other_1363;
    let _e93: HomogeneousMagnitude = other_1363;
    let _e96: HomogeneousMagnitude = other_1363;
    return Flector((((vec4<f32>(_e4.g1_.w) * vec4<f32>(_e8.g0_.y)) * vec4<f32>(0.0, 0.0, 0.0, 1.0)) + (_e19.g0_ * vec4<f32>(_e21.g0_.x))), (((((vec4<f32>(_e27.g1_.x) * vec4<f32>(_e31.g0_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) + ((vec4<f32>(_e42.g1_.y) * vec4<f32>(_e46.g0_.x)) * vec4<f32>(0.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e58.g1_.z) * vec4<f32>(_e62.g0_.x)) * vec4<f32>(0.0, 0.0, 1.0, 0.0))) + (vec4<f32>(_e74.g0_.x, _e77.g0_.y, _e80.g0_.z, _e83.g1_.w) * vec4<f32>(_e87.g0_.y, _e90.g0_.y, _e93.g0_.y, _e96.g0_.x))));
}

fn flector_homogeneous_magnitude_regressive_product(self_1668: Flector, other_1364: HomogeneousMagnitude) -> Flector {
    var self_1669: Flector;
    var other_1365: HomogeneousMagnitude;

    self_1669 = self_1668;
    other_1365 = other_1364;
    let _e4: Flector = self_1669;
    let _e6: HomogeneousMagnitude = other_1365;
    let _e11: Flector = self_1669;
    let _e13: HomogeneousMagnitude = other_1365;
    return Flector((_e4.g0_ * vec4<f32>(_e6.g0_.y)), (_e11.g1_ * vec4<f32>(_e13.g0_.y)));
}

fn flector_homogeneous_magnitude_outer_product(self_1670: Flector, other_1366: HomogeneousMagnitude) -> Flector {
    var self_1671: Flector;
    var other_1367: HomogeneousMagnitude;

    self_1671 = self_1670;
    other_1367 = other_1366;
    let _e4: Flector = self_1671;
    let _e6: HomogeneousMagnitude = other_1367;
    let _e11: Flector = self_1671;
    let _e13: HomogeneousMagnitude = other_1367;
    return Flector((_e4.g0_ * vec4<f32>(_e6.g0_.x)), (_e11.g1_ * vec4<f32>(_e13.g0_.x)));
}

fn flector_homogeneous_magnitude_inner_product(self_1672: Flector, other_1368: HomogeneousMagnitude) -> Flector {
    var self_1673: Flector;
    var other_1369: HomogeneousMagnitude;

    self_1673 = self_1672;
    other_1369 = other_1368;
    let _e4: Flector = self_1673;
    let _e8: HomogeneousMagnitude = other_1369;
    let _e19: Flector = self_1673;
    let _e21: HomogeneousMagnitude = other_1369;
    let _e27: Flector = self_1673;
    let _e31: HomogeneousMagnitude = other_1369;
    let _e42: Flector = self_1673;
    let _e46: HomogeneousMagnitude = other_1369;
    let _e58: Flector = self_1673;
    let _e62: HomogeneousMagnitude = other_1369;
    let _e74: Flector = self_1673;
    let _e77: Flector = self_1673;
    let _e80: Flector = self_1673;
    let _e83: Flector = self_1673;
    let _e87: HomogeneousMagnitude = other_1369;
    let _e90: HomogeneousMagnitude = other_1369;
    let _e93: HomogeneousMagnitude = other_1369;
    let _e96: HomogeneousMagnitude = other_1369;
    return Flector((((vec4<f32>(_e4.g1_.w) * vec4<f32>(_e8.g0_.y)) * vec4<f32>(0.0, 0.0, 0.0, 1.0)) + (_e19.g0_ * vec4<f32>(_e21.g0_.x))), (((((vec4<f32>(_e27.g1_.x) * vec4<f32>(_e31.g0_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) + ((vec4<f32>(_e42.g1_.y) * vec4<f32>(_e46.g0_.x)) * vec4<f32>(0.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e58.g1_.z) * vec4<f32>(_e62.g0_.x)) * vec4<f32>(0.0, 0.0, 1.0, 0.0))) + (vec4<f32>(_e74.g0_.x, _e77.g0_.y, _e80.g0_.z, _e83.g1_.w) * vec4<f32>(_e87.g0_.y, _e90.g0_.y, _e93.g0_.y, _e96.g0_.x))));
}

fn flector_homogeneous_magnitude_geometric_anti_product(self_1674: Flector, other_1370: HomogeneousMagnitude) -> Flector {
    var self_1675: Flector;
    var other_1371: HomogeneousMagnitude;

    self_1675 = self_1674;
    other_1371 = other_1370;
    let _e4: Flector = self_1675;
    let _e8: HomogeneousMagnitude = other_1371;
    let _e20: Flector = self_1675;
    let _e24: HomogeneousMagnitude = other_1371;
    let _e37: Flector = self_1675;
    let _e41: HomogeneousMagnitude = other_1371;
    let _e54: Flector = self_1675;
    let _e56: HomogeneousMagnitude = other_1371;
    let _e62: Flector = self_1675;
    let _e66: HomogeneousMagnitude = other_1371;
    let _e77: Flector = self_1675;
    let _e80: Flector = self_1675;
    let _e83: Flector = self_1675;
    let _e86: Flector = self_1675;
    let _e90: HomogeneousMagnitude = other_1371;
    let _e93: HomogeneousMagnitude = other_1371;
    let _e96: HomogeneousMagnitude = other_1371;
    let _e99: HomogeneousMagnitude = other_1371;
    return Flector((((((vec4<f32>(_e4.g1_.x) * vec4<f32>(_e8.g0_.x)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0)) + ((vec4<f32>(_e20.g1_.y) * vec4<f32>(_e24.g0_.x)) * vec4<f32>(0.0, -(1.0), 0.0, 0.0))) + ((vec4<f32>(_e37.g1_.z) * vec4<f32>(_e41.g0_.x)) * vec4<f32>(0.0, 0.0, -(1.0), 0.0))) + (_e54.g0_ * vec4<f32>(_e56.g0_.y))), (((vec4<f32>(_e62.g1_.w) * vec4<f32>(_e66.g0_.y)) * vec4<f32>(0.0, 0.0, 0.0, 1.0)) + ((vec4<f32>(_e77.g1_.x, _e80.g1_.y, _e83.g1_.z, _e86.g0_.w) * vec4<f32>(_e90.g0_.y, _e93.g0_.y, _e96.g0_.y, _e99.g0_.x)) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))));
}

fn flector_homogeneous_magnitude_inner_anti_product(self_1676: Flector, other_1372: HomogeneousMagnitude) -> Flector {
    var self_1677: Flector;
    var other_1373: HomogeneousMagnitude;

    self_1677 = self_1676;
    other_1373 = other_1372;
    let _e4: Flector = self_1677;
    let _e8: HomogeneousMagnitude = other_1373;
    let _e20: Flector = self_1677;
    let _e24: HomogeneousMagnitude = other_1373;
    let _e37: Flector = self_1677;
    let _e41: HomogeneousMagnitude = other_1373;
    let _e54: Flector = self_1677;
    let _e56: HomogeneousMagnitude = other_1373;
    let _e62: Flector = self_1677;
    let _e66: HomogeneousMagnitude = other_1373;
    let _e77: Flector = self_1677;
    let _e80: Flector = self_1677;
    let _e83: Flector = self_1677;
    let _e86: Flector = self_1677;
    let _e90: HomogeneousMagnitude = other_1373;
    let _e93: HomogeneousMagnitude = other_1373;
    let _e96: HomogeneousMagnitude = other_1373;
    let _e99: HomogeneousMagnitude = other_1373;
    return Flector((((((vec4<f32>(_e4.g1_.x) * vec4<f32>(_e8.g0_.x)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0)) + ((vec4<f32>(_e20.g1_.y) * vec4<f32>(_e24.g0_.x)) * vec4<f32>(0.0, -(1.0), 0.0, 0.0))) + ((vec4<f32>(_e37.g1_.z) * vec4<f32>(_e41.g0_.x)) * vec4<f32>(0.0, 0.0, -(1.0), 0.0))) + (_e54.g0_ * vec4<f32>(_e56.g0_.y))), (((vec4<f32>(_e62.g1_.w) * vec4<f32>(_e66.g0_.y)) * vec4<f32>(0.0, 0.0, 0.0, 1.0)) + ((vec4<f32>(_e77.g1_.x, _e80.g1_.y, _e83.g1_.z, _e86.g0_.w) * vec4<f32>(_e90.g0_.y, _e93.g0_.y, _e96.g0_.y, _e99.g0_.x)) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))));
}

fn flector_homogeneous_magnitude_right_contraction(self_1678: Flector, other_1374: HomogeneousMagnitude) -> Flector {
    var self_1679: Flector;
    var other_1375: HomogeneousMagnitude;

    self_1679 = self_1678;
    other_1375 = other_1374;
    let _e4: Flector = self_1679;
    let _e6: HomogeneousMagnitude = other_1375;
    let _e11: Flector = self_1679;
    let _e13: HomogeneousMagnitude = other_1375;
    return Flector((_e4.g0_ * vec4<f32>(_e6.g0_.x)), (_e11.g1_ * vec4<f32>(_e13.g0_.x)));
}

fn flector_homogeneous_magnitude_right_anti_contraction(self_1680: Flector, other_1376: HomogeneousMagnitude) -> Flector {
    var self_1681: Flector;
    var other_1377: HomogeneousMagnitude;

    self_1681 = self_1680;
    other_1377 = other_1376;
    let _e4: Flector = self_1681;
    let _e6: HomogeneousMagnitude = other_1377;
    let _e11: Flector = self_1681;
    let _e13: HomogeneousMagnitude = other_1377;
    return Flector((_e4.g0_ * vec4<f32>(_e6.g0_.y)), (_e11.g1_ * vec4<f32>(_e13.g0_.y)));
}

fn flector_point_into(self_1682: Flector) -> Point {
    var self_1683: Flector;

    self_1683 = self_1682;
    let _e2: Flector = self_1683;
    return Point(_e2.g0_);
}

fn flector_point_add(self_1684: Flector, other_1378: Point) -> Flector {
    var self_1685: Flector;
    var other_1379: Point;

    self_1685 = self_1684;
    other_1379 = other_1378;
    let _e4: Flector = self_1685;
    let _e6: Point = other_1379;
    let _e9: Flector = self_1685;
    return Flector((_e4.g0_ + _e6.g0_), _e9.g1_);
}

fn flector_point_sub(self_1686: Flector, other_1380: Point) -> Flector {
    var self_1687: Flector;
    var other_1381: Point;

    self_1687 = self_1686;
    other_1381 = other_1380;
    let _e4: Flector = self_1687;
    let _e6: Point = other_1381;
    let _e9: Flector = self_1687;
    return Flector((_e4.g0_ - _e6.g0_), _e9.g1_);
}

fn flector_point_regressive_product(self_1688: Flector, other_1382: Point) -> Scalar {
    var self_1689: Flector;
    var other_1383: Point;

    self_1689 = self_1688;
    other_1383 = other_1382;
    let _e5: Flector = self_1689;
    let _e8: Point = other_1383;
    let _e13: Flector = self_1689;
    let _e16: Point = other_1383;
    let _e21: Flector = self_1689;
    let _e24: Point = other_1383;
    let _e29: Flector = self_1689;
    let _e32: Point = other_1383;
    return Scalar(((((0.0 - (_e5.g1_.x * _e8.g0_.x)) - (_e13.g1_.y * _e16.g0_.y)) - (_e21.g1_.z * _e24.g0_.z)) - (_e29.g1_.w * _e32.g0_.w)));
}

fn flector_point_outer_product(self_1690: Flector, other_1384: Point) -> Motor {
    var self_1691: Flector;
    var other_1385: Point;

    self_1691 = self_1690;
    other_1385 = other_1384;
    let _e4: Flector = self_1691;
    let _e8: Point = other_1385;
    let _e18: Flector = self_1691;
    let _e22: Point = other_1385;
    let _e35: Flector = self_1691;
    let _e39: Point = other_1385;
    let _e52: Flector = self_1691;
    let _e56: Point = other_1385;
    let _e69: Flector = self_1691;
    let _e72: Flector = self_1691;
    let _e75: Flector = self_1691;
    let _e78: Flector = self_1691;
    let _e82: Point = other_1385;
    let _e91: Flector = self_1691;
    let _e95: Point = other_1385;
    let _e98: Point = other_1385;
    let _e101: Point = other_1385;
    let _e112: Flector = self_1691;
    let _e116: Point = other_1385;
    let _e119: Point = other_1385;
    let _e122: Point = other_1385;
    let _e134: Flector = self_1691;
    let _e138: Point = other_1385;
    let _e141: Point = other_1385;
    let _e144: Point = other_1385;
    return Motor(((((((vec4<f32>(_e4.g0_.w) * _e8.g0_.xyzx) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) + ((vec4<f32>(_e18.g1_.y) * vec4<f32>(_e22.g0_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e35.g1_.z) * vec4<f32>(_e39.g0_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e52.g1_.w) * vec4<f32>(_e56.g0_.w)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e69.g0_.x, _e72.g0_.y, _e75.g0_.z, _e78.g1_.x) * _e82.g0_.wwwx) * vec4<f32>(-(1.0)))), ((((vec3<f32>(_e91.g0_.y) * vec3<f32>(_e95.g0_.z, _e98.g0_.z, _e101.g0_.x)) * vec3<f32>(1.0, 0.0, -(1.0))) + ((vec3<f32>(_e112.g0_.z) * vec3<f32>(_e116.g0_.y, _e119.g0_.x, _e122.g0_.y)) * vec3<f32>(-(1.0), 1.0, 0.0))) + ((vec3<f32>(_e134.g0_.x) * vec3<f32>(_e138.g0_.x, _e141.g0_.z, _e144.g0_.y)) * vec3<f32>(0.0, -(1.0), 1.0))));
}

fn flector_point_inner_anti_product(self_1692: Flector, other_1386: Point) -> Motor {
    var self_1693: Flector;
    var other_1387: Point;

    self_1693 = self_1692;
    other_1387 = other_1386;
    let _e4: Flector = self_1693;
    let _e7: Flector = self_1693;
    let _e10: Flector = self_1693;
    let _e13: Flector = self_1693;
    let _e17: Point = other_1387;
    let _e26: Flector = self_1693;
    let _e30: Point = other_1387;
    let _e33: Point = other_1387;
    let _e36: Point = other_1387;
    let _e47: Flector = self_1693;
    let _e51: Point = other_1387;
    let _e54: Point = other_1387;
    let _e57: Point = other_1387;
    let _e69: Flector = self_1693;
    let _e73: Point = other_1387;
    let _e76: Point = other_1387;
    let _e79: Point = other_1387;
    return Motor(((vec4<f32>(_e4.g1_.x, _e7.g1_.y, _e10.g1_.z, _e13.g0_.w) * vec4<f32>(_e17.g0_.w)) * vec4<f32>(-(1.0))), ((((vec3<f32>(_e26.g1_.y) * vec3<f32>(_e30.g0_.z, _e33.g0_.z, _e36.g0_.x)) * vec3<f32>(1.0, 0.0, -(1.0))) + ((vec3<f32>(_e47.g1_.z) * vec3<f32>(_e51.g0_.y, _e54.g0_.x, _e57.g0_.y)) * vec3<f32>(-(1.0), 1.0, 0.0))) + ((vec3<f32>(_e69.g1_.x) * vec3<f32>(_e73.g0_.x, _e76.g0_.z, _e79.g0_.y)) * vec3<f32>(0.0, -(1.0), 1.0))));
}

fn flector_point_left_contraction(self_1694: Flector, other_1388: Point) -> Scalar {
    var self_1695: Flector;
    var other_1389: Point;

    self_1695 = self_1694;
    other_1389 = other_1388;
    let _e4: Flector = self_1695;
    let _e7: Point = other_1389;
    let _e11: Flector = self_1695;
    let _e14: Point = other_1389;
    let _e19: Flector = self_1695;
    let _e22: Point = other_1389;
    return Scalar((((_e4.g0_.x * _e7.g0_.x) + (_e11.g0_.y * _e14.g0_.y)) + (_e19.g0_.z * _e22.g0_.z)));
}

fn flector_point_left_anti_contraction(self_1696: Flector, other_1390: Point) -> Motor {
    var self_1697: Flector;
    var other_1391: Point;

    self_1697 = self_1696;
    other_1391 = other_1390;
    let _e4: Flector = self_1697;
    let _e7: Flector = self_1697;
    let _e10: Flector = self_1697;
    let _e13: Flector = self_1697;
    let _e17: Point = other_1391;
    let _e26: Flector = self_1697;
    let _e30: Point = other_1391;
    let _e33: Point = other_1391;
    let _e36: Point = other_1391;
    let _e47: Flector = self_1697;
    let _e51: Point = other_1391;
    let _e54: Point = other_1391;
    let _e57: Point = other_1391;
    let _e69: Flector = self_1697;
    let _e73: Point = other_1391;
    let _e76: Point = other_1391;
    let _e79: Point = other_1391;
    return Motor(((vec4<f32>(_e4.g1_.x, _e7.g1_.y, _e10.g1_.z, _e13.g0_.w) * vec4<f32>(_e17.g0_.w)) * vec4<f32>(-(1.0))), ((((vec3<f32>(_e26.g1_.y) * vec3<f32>(_e30.g0_.z, _e33.g0_.z, _e36.g0_.x)) * vec3<f32>(1.0, 0.0, -(1.0))) + ((vec3<f32>(_e47.g1_.z) * vec3<f32>(_e51.g0_.y, _e54.g0_.x, _e57.g0_.y)) * vec3<f32>(-(1.0), 1.0, 0.0))) + ((vec3<f32>(_e69.g1_.x) * vec3<f32>(_e73.g0_.x, _e76.g0_.z, _e79.g0_.y)) * vec3<f32>(0.0, -(1.0), 1.0))));
}

fn flector_point_right_anti_contraction(self_1698: Flector, other_1392: Point) -> AntiScalar {
    var self_1699: Flector;
    var other_1393: Point;

    self_1699 = self_1698;
    other_1393 = other_1392;
    let _e5: Flector = self_1699;
    let _e8: Point = other_1393;
    return AntiScalar((0.0 - (_e5.g0_.w * _e8.g0_.w)));
}

fn flector_point_scalar_product(self_1700: Flector, other_1394: Point) -> Scalar {
    var self_1701: Flector;
    var other_1395: Point;

    self_1701 = self_1700;
    other_1395 = other_1394;
    let _e4: Flector = self_1701;
    let _e7: Point = other_1395;
    let _e11: Flector = self_1701;
    let _e14: Point = other_1395;
    let _e19: Flector = self_1701;
    let _e22: Point = other_1395;
    return Scalar((((_e4.g0_.x * _e7.g0_.x) + (_e11.g0_.y * _e14.g0_.y)) + (_e19.g0_.z * _e22.g0_.z)));
}

fn flector_point_anti_scalar_product(self_1702: Flector, other_1396: Point) -> AntiScalar {
    var self_1703: Flector;
    var other_1397: Point;

    self_1703 = self_1702;
    other_1397 = other_1396;
    let _e5: Flector = self_1703;
    let _e8: Point = other_1397;
    return AntiScalar((0.0 - (_e5.g0_.w * _e8.g0_.w)));
}

fn flector_line_geometric_product(self_1704: Flector, other_1398: Line) -> Flector {
    var self_1705: Flector;
    var other_1399: Line;

    self_1705 = self_1704;
    other_1399 = other_1398;
    let _e4: Flector = self_1705;
    let _e8: Line = other_1399;
    let _e11: Line = other_1399;
    let _e14: Line = other_1399;
    let _e17: Line = other_1399;
    let _e30: Flector = self_1705;
    let _e34: Line = other_1399;
    let _e37: Line = other_1399;
    let _e40: Line = other_1399;
    let _e43: Line = other_1399;
    let _e57: Flector = self_1705;
    let _e61: Line = other_1399;
    let _e74: Flector = self_1705;
    let _e78: Line = other_1399;
    let _e91: Flector = self_1705;
    let _e95: Line = other_1399;
    let _e108: Flector = self_1705;
    let _e112: Line = other_1399;
    let _e115: Line = other_1399;
    let _e118: Line = other_1399;
    let _e121: Line = other_1399;
    let _e133: Flector = self_1705;
    let _e137: Line = other_1399;
    let _e140: Line = other_1399;
    let _e143: Line = other_1399;
    let _e146: Line = other_1399;
    let _e160: Flector = self_1705;
    let _e164: Line = other_1399;
    let _e167: Line = other_1399;
    let _e170: Line = other_1399;
    let _e173: Line = other_1399;
    let _e186: Flector = self_1705;
    let _e190: Line = other_1399;
    let _e193: Line = other_1399;
    let _e196: Line = other_1399;
    let _e199: Line = other_1399;
    let _e213: Flector = self_1705;
    let _e217: Line = other_1399;
    let _e220: Line = other_1399;
    let _e223: Line = other_1399;
    let _e226: Line = other_1399;
    let _e238: Flector = self_1705;
    let _e242: Line = other_1399;
    let _e245: Line = other_1399;
    let _e248: Line = other_1399;
    let _e251: Line = other_1399;
    let _e264: Flector = self_1705;
    let _e268: Line = other_1399;
    let _e271: Line = other_1399;
    let _e274: Line = other_1399;
    let _e277: Line = other_1399;
    let _e290: Flector = self_1705;
    let _e294: Line = other_1399;
    let _e297: Line = other_1399;
    let _e300: Line = other_1399;
    let _e303: Line = other_1399;
    let _e316: Flector = self_1705;
    let _e320: Line = other_1399;
    let _e323: Line = other_1399;
    let _e326: Line = other_1399;
    let _e329: Line = other_1399;
    let _e341: Flector = self_1705;
    let _e345: Line = other_1399;
    let _e348: Line = other_1399;
    let _e351: Line = other_1399;
    let _e354: Line = other_1399;
    return Flector(((((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g1_.z, _e11.g1_.z, _e14.g1_.x, _e17.g0_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e30.g0_.z) * vec4<f32>(_e34.g1_.y, _e37.g1_.x, _e40.g1_.y, _e43.g0_.z)) * vec4<f32>(1.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e57.g1_.x) * vec4<f32>(_e61.g1_.x)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e74.g1_.y) * vec4<f32>(_e78.g1_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e91.g1_.z) * vec4<f32>(_e95.g1_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e108.g1_.w) * vec4<f32>(_e112.g1_.x, _e115.g1_.y, _e118.g1_.z, _e121.g1_.x)) * vec4<f32>(1.0, 1.0, 1.0, 0.0))) + ((vec4<f32>(_e133.g0_.x) * vec4<f32>(_e137.g1_.x, _e140.g1_.z, _e143.g1_.y, _e146.g0_.x)) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))), (((((((((vec4<f32>(_e160.g0_.y) * vec4<f32>(_e164.g0_.z, _e167.g0_.z, _e170.g0_.x, _e173.g1_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e186.g0_.z) * vec4<f32>(_e190.g0_.y, _e193.g0_.x, _e196.g0_.y, _e199.g1_.z)) * vec4<f32>(1.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e213.g0_.w) * vec4<f32>(_e217.g1_.x, _e220.g1_.y, _e223.g1_.z, _e226.g1_.x)) * vec4<f32>(1.0, 1.0, 1.0, 0.0))) + ((vec4<f32>(_e238.g1_.x) * vec4<f32>(_e242.g1_.z, _e245.g1_.z, _e248.g1_.y, _e251.g1_.z)) * vec4<f32>(0.0, 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e264.g1_.y) * vec4<f32>(_e268.g1_.z, _e271.g1_.z, _e274.g1_.x, _e277.g1_.z)) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0))) + ((vec4<f32>(_e290.g1_.z) * vec4<f32>(_e294.g1_.y, _e297.g1_.x, _e300.g1_.y, _e303.g1_.y)) * vec4<f32>(1.0, -(1.0), 0.0, 0.0))) + ((vec4<f32>(_e316.g1_.w) * vec4<f32>(_e320.g0_.x, _e323.g0_.y, _e326.g0_.z, _e329.g0_.x)) * vec4<f32>(1.0, 1.0, 1.0, 0.0))) + ((vec4<f32>(_e341.g0_.x) * vec4<f32>(_e345.g0_.x, _e348.g0_.z, _e351.g0_.y, _e354.g1_.x)) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))));
}

fn flector_line_regressive_product(self_1706: Flector, other_1400: Line) -> Point {
    var self_1707: Flector;
    var other_1401: Line;

    self_1707 = self_1706;
    other_1401 = other_1400;
    let _e4: Flector = self_1707;
    let _e8: Line = other_1401;
    let _e11: Line = other_1401;
    let _e14: Line = other_1401;
    let _e17: Line = other_1401;
    let _e30: Flector = self_1707;
    let _e34: Line = other_1401;
    let _e37: Line = other_1401;
    let _e40: Line = other_1401;
    let _e43: Line = other_1401;
    let _e57: Flector = self_1707;
    let _e61: Line = other_1401;
    let _e64: Line = other_1401;
    let _e67: Line = other_1401;
    let _e70: Line = other_1401;
    let _e82: Flector = self_1707;
    let _e86: Line = other_1401;
    let _e89: Line = other_1401;
    let _e92: Line = other_1401;
    let _e95: Line = other_1401;
    return Point((((((vec4<f32>(_e4.g1_.y) * vec4<f32>(_e8.g1_.z, _e11.g1_.z, _e14.g1_.x, _e17.g0_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e30.g1_.z) * vec4<f32>(_e34.g1_.y, _e37.g1_.x, _e40.g1_.y, _e43.g0_.z)) * vec4<f32>(1.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e57.g1_.w) * vec4<f32>(_e61.g0_.x, _e64.g0_.y, _e67.g0_.z, _e70.g0_.x)) * vec4<f32>(1.0, 1.0, 1.0, 0.0))) + ((vec4<f32>(_e82.g1_.x) * vec4<f32>(_e86.g1_.x, _e89.g1_.z, _e92.g1_.y, _e95.g0_.x)) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))));
}

fn flector_line_outer_product(self_1708: Flector, other_1402: Line) -> Plane {
    var self_1709: Flector;
    var other_1403: Line;

    self_1709 = self_1708;
    other_1403 = other_1402;
    let _e4: Flector = self_1709;
    let _e8: Line = other_1403;
    let _e11: Line = other_1403;
    let _e14: Line = other_1403;
    let _e17: Line = other_1403;
    let _e30: Flector = self_1709;
    let _e34: Line = other_1403;
    let _e37: Line = other_1403;
    let _e40: Line = other_1403;
    let _e43: Line = other_1403;
    let _e57: Flector = self_1709;
    let _e61: Line = other_1403;
    let _e64: Line = other_1403;
    let _e67: Line = other_1403;
    let _e70: Line = other_1403;
    let _e82: Flector = self_1709;
    let _e86: Line = other_1403;
    let _e89: Line = other_1403;
    let _e92: Line = other_1403;
    let _e95: Line = other_1403;
    return Plane((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.z, _e11.g0_.z, _e14.g0_.x, _e17.g1_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e30.g0_.z) * vec4<f32>(_e34.g0_.y, _e37.g0_.x, _e40.g0_.y, _e43.g1_.z)) * vec4<f32>(1.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e57.g0_.w) * vec4<f32>(_e61.g1_.x, _e64.g1_.y, _e67.g1_.z, _e70.g1_.x)) * vec4<f32>(1.0, 1.0, 1.0, 0.0))) + ((vec4<f32>(_e82.g0_.x) * vec4<f32>(_e86.g0_.x, _e89.g0_.z, _e92.g0_.y, _e95.g1_.x)) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))));
}

fn flector_line_inner_product(self_1710: Flector, other_1404: Line) -> Point {
    var self_1711: Flector;
    var other_1405: Line;

    self_1711 = self_1710;
    other_1405 = other_1404;
    let _e4: Flector = self_1711;
    let _e8: Line = other_1405;
    let _e11: Line = other_1405;
    let _e14: Line = other_1405;
    let _e17: Line = other_1405;
    let _e30: Flector = self_1711;
    let _e34: Line = other_1405;
    let _e37: Line = other_1405;
    let _e40: Line = other_1405;
    let _e43: Line = other_1405;
    let _e57: Flector = self_1711;
    let _e61: Line = other_1405;
    let _e74: Flector = self_1711;
    let _e78: Line = other_1405;
    let _e91: Flector = self_1711;
    let _e95: Line = other_1405;
    let _e108: Flector = self_1711;
    let _e112: Line = other_1405;
    let _e115: Line = other_1405;
    let _e118: Line = other_1405;
    let _e121: Line = other_1405;
    let _e133: Flector = self_1711;
    let _e137: Line = other_1405;
    let _e140: Line = other_1405;
    let _e143: Line = other_1405;
    let _e146: Line = other_1405;
    return Point(((((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g1_.z, _e11.g1_.z, _e14.g1_.x, _e17.g0_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e30.g0_.z) * vec4<f32>(_e34.g1_.y, _e37.g1_.x, _e40.g1_.y, _e43.g0_.z)) * vec4<f32>(1.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e57.g1_.x) * vec4<f32>(_e61.g1_.x)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e74.g1_.y) * vec4<f32>(_e78.g1_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e91.g1_.z) * vec4<f32>(_e95.g1_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e108.g1_.w) * vec4<f32>(_e112.g1_.x, _e115.g1_.y, _e118.g1_.z, _e121.g1_.x)) * vec4<f32>(1.0, 1.0, 1.0, 0.0))) + ((vec4<f32>(_e133.g0_.x) * vec4<f32>(_e137.g1_.x, _e140.g1_.z, _e143.g1_.y, _e146.g0_.x)) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))));
}

fn flector_line_geometric_anti_product(self_1712: Flector, other_1406: Line) -> Flector {
    var self_1713: Flector;
    var other_1407: Line;

    self_1713 = self_1712;
    other_1407 = other_1406;
    let _e4: Flector = self_1713;
    let _e8: Line = other_1407;
    let _e11: Line = other_1407;
    let _e14: Line = other_1407;
    let _e17: Line = other_1407;
    let _e29: Flector = self_1713;
    let _e33: Line = other_1407;
    let _e36: Line = other_1407;
    let _e39: Line = other_1407;
    let _e42: Line = other_1407;
    let _e55: Flector = self_1713;
    let _e59: Line = other_1407;
    let _e62: Line = other_1407;
    let _e65: Line = other_1407;
    let _e68: Line = other_1407;
    let _e83: Flector = self_1713;
    let _e87: Line = other_1407;
    let _e90: Line = other_1407;
    let _e93: Line = other_1407;
    let _e96: Line = other_1407;
    let _e110: Flector = self_1713;
    let _e114: Line = other_1407;
    let _e117: Line = other_1407;
    let _e120: Line = other_1407;
    let _e123: Line = other_1407;
    let _e137: Flector = self_1713;
    let _e141: Line = other_1407;
    let _e144: Line = other_1407;
    let _e147: Line = other_1407;
    let _e150: Line = other_1407;
    let _e164: Flector = self_1713;
    let _e168: Line = other_1407;
    let _e171: Line = other_1407;
    let _e174: Line = other_1407;
    let _e177: Line = other_1407;
    let _e189: Flector = self_1713;
    let _e193: Line = other_1407;
    let _e196: Line = other_1407;
    let _e199: Line = other_1407;
    let _e202: Line = other_1407;
    let _e215: Flector = self_1713;
    let _e219: Line = other_1407;
    let _e231: Flector = self_1713;
    let _e235: Line = other_1407;
    let _e248: Flector = self_1713;
    let _e252: Line = other_1407;
    let _e255: Line = other_1407;
    let _e258: Line = other_1407;
    let _e261: Line = other_1407;
    let _e274: Flector = self_1713;
    let _e278: Line = other_1407;
    let _e281: Line = other_1407;
    let _e284: Line = other_1407;
    let _e287: Line = other_1407;
    let _e300: Flector = self_1713;
    let _e304: Line = other_1407;
    let _e307: Line = other_1407;
    let _e310: Line = other_1407;
    let _e313: Line = other_1407;
    let _e326: Flector = self_1713;
    let _e329: Line = other_1407;
    let _e332: Line = other_1407;
    let _e335: Line = other_1407;
    let _e338: Line = other_1407;
    return Flector((((((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.z, _e11.g0_.z, _e14.g0_.x, _e17.g0_.z)) * vec4<f32>(1.0, 0.0, -(1.0), 0.0)) + ((vec4<f32>(_e29.g0_.z) * vec4<f32>(_e33.g0_.y, _e36.g0_.x, _e39.g0_.y, _e42.g0_.y)) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))) + ((vec4<f32>(_e55.g0_.w) * vec4<f32>(_e59.g1_.x, _e62.g1_.y, _e65.g1_.z, _e68.g1_.x)) * vec4<f32>(-(1.0), -(1.0), -(1.0), 0.0))) + ((vec4<f32>(_e83.g1_.x) * vec4<f32>(_e87.g1_.z, _e90.g1_.z, _e93.g1_.y, _e96.g0_.x)) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e110.g1_.y) * vec4<f32>(_e114.g1_.z, _e117.g1_.z, _e120.g1_.x, _e123.g0_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e137.g1_.z) * vec4<f32>(_e141.g1_.y, _e144.g1_.x, _e147.g1_.y, _e150.g0_.z)) * vec4<f32>(1.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e164.g1_.w) * vec4<f32>(_e168.g0_.x, _e171.g0_.y, _e174.g0_.z, _e177.g0_.x)) * vec4<f32>(1.0, 1.0, 1.0, 0.0))) + ((vec4<f32>(_e189.g0_.x) * vec4<f32>(_e193.g0_.x, _e196.g0_.z, _e199.g0_.y, _e202.g0_.x)) * vec4<f32>(0.0, -(1.0), 1.0, 0.0))), (((((((vec4<f32>(_e215.g0_.y) * vec4<f32>(_e219.g0_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0))) + ((vec4<f32>(_e231.g0_.z) * vec4<f32>(_e235.g0_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e248.g1_.x) * vec4<f32>(_e252.g0_.z, _e255.g0_.z, _e258.g0_.y, _e261.g1_.x)) * vec4<f32>(0.0, -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e274.g1_.y) * vec4<f32>(_e278.g0_.z, _e281.g0_.z, _e284.g0_.x, _e287.g1_.y)) * vec4<f32>(1.0, 0.0, -(1.0), 1.0))) + ((vec4<f32>(_e300.g1_.z) * vec4<f32>(_e304.g0_.y, _e307.g0_.x, _e310.g0_.y, _e313.g1_.z)) * vec4<f32>(-(1.0), 1.0, 0.0, 1.0))) + ((_e326.g0_.wwwx * vec4<f32>(_e329.g0_.x, _e332.g0_.y, _e335.g0_.z, _e338.g0_.x)) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))));
}

fn flector_line_inner_anti_product(self_1714: Flector, other_1408: Line) -> Plane {
    var self_1715: Flector;
    var other_1409: Line;

    self_1715 = self_1714;
    other_1409 = other_1408;
    let _e4: Flector = self_1715;
    let _e8: Line = other_1409;
    let _e20: Flector = self_1715;
    let _e24: Line = other_1409;
    let _e37: Flector = self_1715;
    let _e41: Line = other_1409;
    let _e44: Line = other_1409;
    let _e47: Line = other_1409;
    let _e50: Line = other_1409;
    let _e63: Flector = self_1715;
    let _e67: Line = other_1409;
    let _e70: Line = other_1409;
    let _e73: Line = other_1409;
    let _e76: Line = other_1409;
    let _e89: Flector = self_1715;
    let _e93: Line = other_1409;
    let _e96: Line = other_1409;
    let _e99: Line = other_1409;
    let _e102: Line = other_1409;
    let _e115: Flector = self_1715;
    let _e118: Line = other_1409;
    let _e121: Line = other_1409;
    let _e124: Line = other_1409;
    let _e127: Line = other_1409;
    return Plane((((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0))) + ((vec4<f32>(_e20.g0_.z) * vec4<f32>(_e24.g0_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e37.g1_.x) * vec4<f32>(_e41.g0_.z, _e44.g0_.z, _e47.g0_.y, _e50.g1_.x)) * vec4<f32>(0.0, -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e63.g1_.y) * vec4<f32>(_e67.g0_.z, _e70.g0_.z, _e73.g0_.x, _e76.g1_.y)) * vec4<f32>(1.0, 0.0, -(1.0), 1.0))) + ((vec4<f32>(_e89.g1_.z) * vec4<f32>(_e93.g0_.y, _e96.g0_.x, _e99.g0_.y, _e102.g1_.z)) * vec4<f32>(-(1.0), 1.0, 0.0, 1.0))) + ((_e115.g0_.wwwx * vec4<f32>(_e118.g0_.x, _e121.g0_.y, _e124.g0_.z, _e127.g0_.x)) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))));
}

fn flector_line_left_contraction(self_1716: Flector, other_1410: Line) -> Point {
    var self_1717: Flector;
    var other_1411: Line;

    self_1717 = self_1716;
    other_1411 = other_1410;
    let _e4: Flector = self_1717;
    let _e8: Line = other_1411;
    let _e11: Line = other_1411;
    let _e14: Line = other_1411;
    let _e17: Line = other_1411;
    let _e30: Flector = self_1717;
    let _e34: Line = other_1411;
    let _e37: Line = other_1411;
    let _e40: Line = other_1411;
    let _e43: Line = other_1411;
    let _e57: Flector = self_1717;
    let _e61: Line = other_1411;
    let _e64: Line = other_1411;
    let _e67: Line = other_1411;
    let _e70: Line = other_1411;
    return Point(((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g1_.z, _e11.g1_.z, _e14.g1_.x, _e17.g0_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e30.g0_.z) * vec4<f32>(_e34.g1_.y, _e37.g1_.x, _e40.g1_.y, _e43.g0_.z)) * vec4<f32>(1.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e57.g0_.x) * vec4<f32>(_e61.g1_.x, _e64.g1_.z, _e67.g1_.y, _e70.g0_.x)) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))));
}

fn flector_line_right_contraction(self_1718: Flector, other_1412: Line) -> Point {
    var self_1719: Flector;
    var other_1413: Line;

    self_1719 = self_1718;
    other_1413 = other_1412;
    let _e4: Flector = self_1719;
    let _e8: Line = other_1413;
    let _e20: Flector = self_1719;
    let _e24: Line = other_1413;
    let _e37: Flector = self_1719;
    let _e40: Line = other_1413;
    let _e43: Line = other_1413;
    let _e46: Line = other_1413;
    let _e49: Line = other_1413;
    return Point(((((vec4<f32>(_e4.g1_.y) * vec4<f32>(_e8.g1_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0))) + ((vec4<f32>(_e20.g1_.z) * vec4<f32>(_e24.g1_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((_e37.g1_.wwwx * vec4<f32>(_e40.g1_.x, _e43.g1_.y, _e46.g1_.z, _e49.g1_.x)) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))));
}

fn flector_line_left_anti_contraction(self_1720: Flector, other_1414: Line) -> Plane {
    var self_1721: Flector;
    var other_1415: Line;

    self_1721 = self_1720;
    other_1415 = other_1414;
    let _e4: Flector = self_1721;
    let _e8: Line = other_1415;
    let _e11: Line = other_1415;
    let _e14: Line = other_1415;
    let _e17: Line = other_1415;
    let _e29: Flector = self_1721;
    let _e33: Line = other_1415;
    let _e36: Line = other_1415;
    let _e39: Line = other_1415;
    let _e42: Line = other_1415;
    let _e55: Flector = self_1721;
    let _e59: Line = other_1415;
    let _e62: Line = other_1415;
    let _e65: Line = other_1415;
    let _e68: Line = other_1415;
    return Plane(((((vec4<f32>(_e4.g1_.y) * vec4<f32>(_e8.g0_.z, _e11.g0_.z, _e14.g0_.x, _e17.g1_.y)) * vec4<f32>(1.0, 0.0, -(1.0), 1.0)) + ((vec4<f32>(_e29.g1_.z) * vec4<f32>(_e33.g0_.y, _e36.g0_.x, _e39.g0_.y, _e42.g1_.z)) * vec4<f32>(-(1.0), 1.0, 0.0, 1.0))) + ((vec4<f32>(_e55.g1_.x) * vec4<f32>(_e59.g0_.x, _e62.g0_.z, _e65.g0_.y, _e68.g1_.x)) * vec4<f32>(0.0, -(1.0), 1.0, 1.0))));
}

fn flector_line_right_anti_contraction(self_1722: Flector, other_1416: Line) -> Plane {
    var self_1723: Flector;
    var other_1417: Line;

    self_1723 = self_1722;
    other_1417 = other_1416;
    let _e4: Flector = self_1723;
    let _e8: Line = other_1417;
    let _e20: Flector = self_1723;
    let _e24: Line = other_1417;
    let _e37: Flector = self_1723;
    let _e40: Line = other_1417;
    let _e43: Line = other_1417;
    let _e46: Line = other_1417;
    let _e49: Line = other_1417;
    return Plane(((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0))) + ((vec4<f32>(_e20.g0_.z) * vec4<f32>(_e24.g0_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((_e37.g0_.wwwx * vec4<f32>(_e40.g0_.x, _e43.g0_.y, _e46.g0_.z, _e49.g0_.x)) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))));
}

fn flector_plane_into(self_1724: Flector) -> Plane {
    var self_1725: Flector;

    self_1725 = self_1724;
    let _e2: Flector = self_1725;
    return Plane(_e2.g1_);
}

fn flector_plane_add(self_1726: Flector, other_1418: Plane) -> Flector {
    var self_1727: Flector;
    var other_1419: Plane;

    self_1727 = self_1726;
    other_1419 = other_1418;
    let _e4: Flector = self_1727;
    let _e6: Flector = self_1727;
    let _e8: Plane = other_1419;
    return Flector(_e4.g0_, (_e6.g1_ + _e8.g0_));
}

fn flector_plane_sub(self_1728: Flector, other_1420: Plane) -> Flector {
    var self_1729: Flector;
    var other_1421: Plane;

    self_1729 = self_1728;
    other_1421 = other_1420;
    let _e4: Flector = self_1729;
    let _e6: Flector = self_1729;
    let _e8: Plane = other_1421;
    return Flector(_e4.g0_, (_e6.g1_ - _e8.g0_));
}

fn flector_plane_outer_product(self_1730: Flector, other_1422: Plane) -> AntiScalar {
    var self_1731: Flector;
    var other_1423: Plane;

    self_1731 = self_1730;
    other_1423 = other_1422;
    let _e4: Flector = self_1731;
    let _e7: Plane = other_1423;
    let _e11: Flector = self_1731;
    let _e14: Plane = other_1423;
    let _e19: Flector = self_1731;
    let _e22: Plane = other_1423;
    let _e27: Flector = self_1731;
    let _e30: Plane = other_1423;
    return AntiScalar(((((_e4.g0_.x * _e7.g0_.x) + (_e11.g0_.y * _e14.g0_.y)) + (_e19.g0_.z * _e22.g0_.z)) + (_e27.g0_.w * _e30.g0_.w)));
}

fn flector_plane_inner_anti_product(self_1732: Flector, other_1424: Plane) -> Motor {
    var self_1733: Flector;
    var other_1425: Plane;

    self_1733 = self_1732;
    other_1425 = other_1424;
    let _e4: Flector = self_1733;
    let _e8: Plane = other_1425;
    let _e19: Flector = self_1733;
    let _e23: Plane = other_1425;
    let _e35: Flector = self_1733;
    let _e38: Flector = self_1733;
    let _e41: Flector = self_1733;
    let _e44: Flector = self_1733;
    let _e48: Plane = other_1425;
    let _e62: Flector = self_1733;
    let _e66: Plane = other_1425;
    let _e69: Plane = other_1425;
    let _e72: Plane = other_1425;
    let _e83: Flector = self_1733;
    let _e87: Plane = other_1425;
    let _e90: Plane = other_1425;
    let _e93: Plane = other_1425;
    let _e105: Flector = self_1733;
    let _e109: Plane = other_1425;
    let _e112: Plane = other_1425;
    let _e115: Plane = other_1425;
    return Motor(((((vec4<f32>(_e4.g1_.y) * vec4<f32>(_e8.g0_.y)) * vec4<f32>(0.0, 0.0, 0.0, 1.0)) + ((vec4<f32>(_e19.g1_.z) * vec4<f32>(_e23.g0_.z)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e35.g0_.w, _e38.g0_.w, _e41.g0_.w, _e44.g1_.x) * _e48.g0_.xyzx) * vec4<f32>(-(1.0), -(1.0), -(1.0), 1.0))), ((((vec3<f32>(_e62.g0_.y) * vec3<f32>(_e66.g0_.z, _e69.g0_.z, _e72.g0_.x)) * vec3<f32>(-(1.0), 0.0, 1.0)) + ((vec3<f32>(_e83.g0_.z) * vec3<f32>(_e87.g0_.y, _e90.g0_.x, _e93.g0_.y)) * vec3<f32>(1.0, -(1.0), 0.0))) + ((vec3<f32>(_e105.g0_.x) * vec3<f32>(_e109.g0_.x, _e112.g0_.z, _e115.g0_.y)) * vec3<f32>(0.0, 1.0, -(1.0)))));
}

fn flector_plane_right_contraction(self_1734: Flector, other_1426: Plane) -> Scalar {
    var self_1735: Flector;
    var other_1427: Plane;

    self_1735 = self_1734;
    other_1427 = other_1426;
    let _e5: Flector = self_1735;
    let _e8: Plane = other_1427;
    return Scalar((0.0 - (_e5.g1_.w * _e8.g0_.w)));
}

fn flector_plane_left_anti_contraction(self_1736: Flector, other_1428: Plane) -> AntiScalar {
    var self_1737: Flector;
    var other_1429: Plane;

    self_1737 = self_1736;
    other_1429 = other_1428;
    let _e4: Flector = self_1737;
    let _e7: Plane = other_1429;
    let _e11: Flector = self_1737;
    let _e14: Plane = other_1429;
    let _e19: Flector = self_1737;
    let _e22: Plane = other_1429;
    return AntiScalar((((_e4.g1_.x * _e7.g0_.x) + (_e11.g1_.y * _e14.g0_.y)) + (_e19.g1_.z * _e22.g0_.z)));
}

fn flector_plane_right_anti_contraction(self_1738: Flector, other_1430: Plane) -> Motor {
    var self_1739: Flector;
    var other_1431: Plane;

    self_1739 = self_1738;
    other_1431 = other_1430;
    let _e4: Flector = self_1739;
    let _e8: Plane = other_1431;
    let _e19: Flector = self_1739;
    let _e23: Plane = other_1431;
    let _e35: Flector = self_1739;
    let _e38: Flector = self_1739;
    let _e41: Flector = self_1739;
    let _e44: Flector = self_1739;
    let _e48: Plane = other_1431;
    let _e62: Flector = self_1739;
    let _e66: Plane = other_1431;
    let _e69: Plane = other_1431;
    let _e72: Plane = other_1431;
    let _e83: Flector = self_1739;
    let _e87: Plane = other_1431;
    let _e90: Plane = other_1431;
    let _e93: Plane = other_1431;
    let _e105: Flector = self_1739;
    let _e109: Plane = other_1431;
    let _e112: Plane = other_1431;
    let _e115: Plane = other_1431;
    return Motor(((((vec4<f32>(_e4.g1_.y) * vec4<f32>(_e8.g0_.y)) * vec4<f32>(0.0, 0.0, 0.0, 1.0)) + ((vec4<f32>(_e19.g1_.z) * vec4<f32>(_e23.g0_.z)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e35.g0_.w, _e38.g0_.w, _e41.g0_.w, _e44.g1_.x) * _e48.g0_.xyzx) * vec4<f32>(-(1.0), -(1.0), -(1.0), 1.0))), ((((vec3<f32>(_e62.g0_.y) * vec3<f32>(_e66.g0_.z, _e69.g0_.z, _e72.g0_.x)) * vec3<f32>(-(1.0), 0.0, 1.0)) + ((vec3<f32>(_e83.g0_.z) * vec3<f32>(_e87.g0_.y, _e90.g0_.x, _e93.g0_.y)) * vec3<f32>(1.0, -(1.0), 0.0))) + ((vec3<f32>(_e105.g0_.x) * vec3<f32>(_e109.g0_.x, _e112.g0_.z, _e115.g0_.y)) * vec3<f32>(0.0, 1.0, -(1.0)))));
}

fn flector_plane_scalar_product(self_1740: Flector, other_1432: Plane) -> Scalar {
    var self_1741: Flector;
    var other_1433: Plane;

    self_1741 = self_1740;
    other_1433 = other_1432;
    let _e5: Flector = self_1741;
    let _e8: Plane = other_1433;
    return Scalar((0.0 - (_e5.g1_.w * _e8.g0_.w)));
}

fn flector_plane_anti_scalar_product(self_1742: Flector, other_1434: Plane) -> AntiScalar {
    var self_1743: Flector;
    var other_1435: Plane;

    self_1743 = self_1742;
    other_1435 = other_1434;
    let _e4: Flector = self_1743;
    let _e7: Plane = other_1435;
    let _e11: Flector = self_1743;
    let _e14: Plane = other_1435;
    let _e19: Flector = self_1743;
    let _e22: Plane = other_1435;
    return AntiScalar((((_e4.g1_.x * _e7.g0_.x) + (_e11.g1_.y * _e14.g0_.y)) + (_e19.g1_.z * _e22.g0_.z)));
}

fn flector_motor_geometric_product(self_1744: Flector, other_1436: Motor) -> Flector {
    var self_1745: Flector;
    var other_1437: Motor;

    self_1745 = self_1744;
    other_1437 = other_1436;
    let _e4: Flector = self_1745;
    let _e8: Motor = other_1437;
    let _e11: Motor = other_1437;
    let _e14: Motor = other_1437;
    let _e17: Motor = other_1437;
    let _e30: Flector = self_1745;
    let _e34: Motor = other_1437;
    let _e37: Motor = other_1437;
    let _e40: Motor = other_1437;
    let _e43: Motor = other_1437;
    let _e57: Flector = self_1745;
    let _e61: Motor = other_1437;
    let _e74: Flector = self_1745;
    let _e78: Motor = other_1437;
    let _e91: Flector = self_1745;
    let _e95: Motor = other_1437;
    let _e108: Flector = self_1745;
    let _e112: Motor = other_1437;
    let _e115: Motor = other_1437;
    let _e118: Motor = other_1437;
    let _e121: Motor = other_1437;
    let _e127: Flector = self_1745;
    let _e131: Motor = other_1437;
    let _e134: Motor = other_1437;
    let _e137: Motor = other_1437;
    let _e140: Motor = other_1437;
    let _e154: Flector = self_1745;
    let _e158: Motor = other_1437;
    let _e161: Motor = other_1437;
    let _e164: Motor = other_1437;
    let _e167: Motor = other_1437;
    let _e180: Flector = self_1745;
    let _e184: Motor = other_1437;
    let _e187: Motor = other_1437;
    let _e190: Motor = other_1437;
    let _e193: Motor = other_1437;
    let _e207: Flector = self_1745;
    let _e211: Motor = other_1437;
    let _e214: Motor = other_1437;
    let _e217: Motor = other_1437;
    let _e220: Motor = other_1437;
    let _e234: Flector = self_1745;
    let _e238: Motor = other_1437;
    let _e241: Motor = other_1437;
    let _e244: Motor = other_1437;
    let _e247: Motor = other_1437;
    let _e260: Flector = self_1745;
    let _e264: Motor = other_1437;
    let _e267: Motor = other_1437;
    let _e270: Motor = other_1437;
    let _e273: Motor = other_1437;
    let _e286: Flector = self_1745;
    let _e290: Motor = other_1437;
    let _e293: Motor = other_1437;
    let _e296: Motor = other_1437;
    let _e299: Motor = other_1437;
    let _e312: Flector = self_1745;
    let _e316: Motor = other_1437;
    let _e327: Flector = self_1745;
    let _e330: Motor = other_1437;
    let _e333: Motor = other_1437;
    let _e336: Motor = other_1437;
    let _e339: Motor = other_1437;
    return Flector(((((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g1_.z, _e11.g1_.z, _e14.g1_.x, _e17.g0_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e30.g0_.z) * vec4<f32>(_e34.g1_.y, _e37.g1_.x, _e40.g1_.y, _e43.g0_.z)) * vec4<f32>(1.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e57.g1_.x) * vec4<f32>(_e61.g1_.x)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e74.g1_.y) * vec4<f32>(_e78.g1_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e91.g1_.z) * vec4<f32>(_e95.g1_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + (vec4<f32>(_e108.g1_.w) * vec4<f32>(_e112.g1_.x, _e115.g1_.y, _e118.g1_.z, _e121.g0_.w))) + ((vec4<f32>(_e127.g0_.x) * vec4<f32>(_e131.g1_.x, _e134.g1_.z, _e137.g1_.y, _e140.g0_.x)) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))), (((((((((vec4<f32>(_e154.g0_.x) * vec4<f32>(_e158.g0_.w, _e161.g0_.z, _e164.g0_.y, _e167.g1_.x)) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0))) + ((vec4<f32>(_e180.g0_.y) * vec4<f32>(_e184.g0_.z, _e187.g0_.w, _e190.g0_.x, _e193.g1_.y)) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e207.g0_.z) * vec4<f32>(_e211.g0_.y, _e214.g0_.x, _e217.g0_.w, _e220.g1_.z)) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e234.g1_.x) * vec4<f32>(_e238.g1_.z, _e241.g1_.z, _e244.g1_.y, _e247.g1_.z)) * vec4<f32>(0.0, 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e260.g1_.y) * vec4<f32>(_e264.g1_.z, _e267.g1_.z, _e270.g1_.x, _e273.g1_.z)) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0))) + ((vec4<f32>(_e286.g1_.z) * vec4<f32>(_e290.g1_.y, _e293.g1_.x, _e296.g1_.y, _e299.g1_.y)) * vec4<f32>(1.0, -(1.0), 0.0, 0.0))) + ((vec4<f32>(_e312.g1_.w) * _e316.g0_.xyzx) * vec4<f32>(1.0, 1.0, 1.0, 0.0))) + ((_e327.g0_.wwwx * vec4<f32>(_e330.g1_.x, _e333.g1_.y, _e336.g1_.z, _e339.g1_.x)) * vec4<f32>(1.0, 1.0, 1.0, 0.0))));
}

fn flector_motor_regressive_product(self_1746: Flector, other_1438: Motor) -> Flector {
    var self_1747: Flector;
    var other_1439: Motor;

    self_1747 = self_1746;
    other_1439 = other_1438;
    let _e4: Flector = self_1747;
    let _e8: Motor = other_1439;
    let _e11: Motor = other_1439;
    let _e14: Motor = other_1439;
    let _e17: Motor = other_1439;
    let _e30: Flector = self_1747;
    let _e34: Motor = other_1439;
    let _e37: Motor = other_1439;
    let _e40: Motor = other_1439;
    let _e43: Motor = other_1439;
    let _e57: Flector = self_1747;
    let _e61: Motor = other_1439;
    let _e64: Motor = other_1439;
    let _e67: Motor = other_1439;
    let _e70: Motor = other_1439;
    let _e84: Flector = self_1747;
    let _e88: Motor = other_1439;
    let _e99: Flector = self_1747;
    let _e101: Motor = other_1439;
    let _e107: Flector = self_1747;
    let _e109: Motor = other_1439;
    return Flector(((((((vec4<f32>(_e4.g1_.x) * vec4<f32>(_e8.g1_.z, _e11.g1_.z, _e14.g1_.y, _e17.g0_.x)) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0))) + ((vec4<f32>(_e30.g1_.y) * vec4<f32>(_e34.g1_.z, _e37.g1_.z, _e40.g1_.x, _e43.g0_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e57.g1_.z) * vec4<f32>(_e61.g1_.y, _e64.g1_.x, _e67.g1_.y, _e70.g0_.z)) * vec4<f32>(1.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e84.g1_.w) * _e88.g0_.xyzx) * vec4<f32>(1.0, 1.0, 1.0, 0.0))) + (_e99.g0_ * vec4<f32>(_e101.g0_.w))), (_e107.g1_ * vec4<f32>(_e109.g0_.w)));
}

fn flector_motor_outer_product(self_1748: Flector, other_1440: Motor) -> Plane {
    var self_1749: Flector;
    var other_1441: Motor;

    self_1749 = self_1748;
    other_1441 = other_1440;
    let _e4: Flector = self_1749;
    let _e8: Motor = other_1441;
    let _e11: Motor = other_1441;
    let _e14: Motor = other_1441;
    let _e17: Motor = other_1441;
    let _e30: Flector = self_1749;
    let _e34: Motor = other_1441;
    let _e37: Motor = other_1441;
    let _e40: Motor = other_1441;
    let _e43: Motor = other_1441;
    let _e57: Flector = self_1749;
    let _e61: Motor = other_1441;
    let _e64: Motor = other_1441;
    let _e67: Motor = other_1441;
    let _e70: Motor = other_1441;
    let _e82: Flector = self_1749;
    let _e86: Motor = other_1441;
    let _e89: Motor = other_1441;
    let _e92: Motor = other_1441;
    let _e95: Motor = other_1441;
    return Plane((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.z, _e11.g0_.z, _e14.g0_.x, _e17.g1_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e30.g0_.z) * vec4<f32>(_e34.g0_.y, _e37.g0_.x, _e40.g0_.y, _e43.g1_.z)) * vec4<f32>(1.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e57.g0_.w) * vec4<f32>(_e61.g1_.x, _e64.g1_.y, _e67.g1_.z, _e70.g1_.x)) * vec4<f32>(1.0, 1.0, 1.0, 0.0))) + ((vec4<f32>(_e82.g0_.x) * vec4<f32>(_e86.g0_.x, _e89.g0_.z, _e92.g0_.y, _e95.g1_.x)) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))));
}

fn flector_motor_geometric_anti_product(self_1750: Flector, other_1442: Motor) -> Flector {
    var self_1751: Flector;
    var other_1443: Motor;

    self_1751 = self_1750;
    other_1443 = other_1442;
    let _e4: Flector = self_1751;
    let _e8: Motor = other_1443;
    let _e19: Flector = self_1751;
    let _e23: Motor = other_1443;
    let _e35: Flector = self_1751;
    let _e39: Motor = other_1443;
    let _e42: Motor = other_1443;
    let _e45: Motor = other_1443;
    let _e48: Motor = other_1443;
    let _e63: Flector = self_1751;
    let _e67: Motor = other_1443;
    let _e70: Motor = other_1443;
    let _e73: Motor = other_1443;
    let _e76: Motor = other_1443;
    let _e90: Flector = self_1751;
    let _e94: Motor = other_1443;
    let _e97: Motor = other_1443;
    let _e100: Motor = other_1443;
    let _e103: Motor = other_1443;
    let _e117: Flector = self_1751;
    let _e121: Motor = other_1443;
    let _e124: Motor = other_1443;
    let _e127: Motor = other_1443;
    let _e130: Motor = other_1443;
    let _e144: Flector = self_1751;
    let _e148: Motor = other_1443;
    let _e159: Flector = self_1751;
    let _e163: Motor = other_1443;
    let _e175: Flector = self_1751;
    let _e179: Motor = other_1443;
    let _e191: Flector = self_1751;
    let _e195: Motor = other_1443;
    let _e208: Flector = self_1751;
    let _e212: Motor = other_1443;
    let _e215: Motor = other_1443;
    let _e218: Motor = other_1443;
    let _e221: Motor = other_1443;
    let _e234: Flector = self_1751;
    let _e238: Motor = other_1443;
    let _e241: Motor = other_1443;
    let _e244: Motor = other_1443;
    let _e247: Motor = other_1443;
    let _e260: Flector = self_1751;
    let _e264: Motor = other_1443;
    let _e267: Motor = other_1443;
    let _e270: Motor = other_1443;
    let _e273: Motor = other_1443;
    let _e286: Flector = self_1751;
    let _e290: Motor = other_1443;
    let _e302: Flector = self_1751;
    let _e305: Motor = other_1443;
    return Flector((((((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.zwxz) * vec4<f32>(1.0, 1.0, -(1.0), 0.0)) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.yxwy) * vec4<f32>(-(1.0), 1.0, 1.0, 0.0))) + ((vec4<f32>(_e35.g0_.w) * vec4<f32>(_e39.g1_.x, _e42.g1_.y, _e45.g1_.z, _e48.g0_.w)) * vec4<f32>(-(1.0), -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e63.g1_.x) * vec4<f32>(_e67.g1_.z, _e70.g1_.z, _e73.g1_.y, _e76.g0_.x)) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e90.g1_.y) * vec4<f32>(_e94.g1_.z, _e97.g1_.z, _e100.g1_.x, _e103.g0_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e117.g1_.z) * vec4<f32>(_e121.g1_.y, _e124.g1_.x, _e127.g1_.y, _e130.g0_.z)) * vec4<f32>(1.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e144.g1_.w) * _e148.g0_.xyzx) * vec4<f32>(1.0, 1.0, 1.0, 0.0))) + ((vec4<f32>(_e159.g0_.x) * _e163.g0_.wzyx) * vec4<f32>(1.0, -(1.0), 1.0, 0.0))), ((((((((vec4<f32>(_e175.g0_.y) * vec4<f32>(_e179.g0_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0))) + ((vec4<f32>(_e191.g0_.z) * vec4<f32>(_e195.g0_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e208.g1_.x) * vec4<f32>(_e212.g0_.w, _e215.g0_.z, _e218.g0_.y, _e221.g1_.x)) * vec4<f32>(1.0, -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e234.g1_.y) * vec4<f32>(_e238.g0_.z, _e241.g0_.w, _e244.g0_.x, _e247.g1_.y)) * vec4<f32>(1.0, 1.0, -(1.0), 1.0))) + ((vec4<f32>(_e260.g1_.z) * vec4<f32>(_e264.g0_.y, _e267.g0_.x, _e270.g0_.w, _e273.g1_.z)) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0))) + ((vec4<f32>(_e286.g1_.w) * vec4<f32>(_e290.g0_.w)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((_e302.g0_.wwwx * _e305.g0_.xyzx) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))));
}

fn flector_motor_inner_anti_product(self_1752: Flector, other_1444: Motor) -> Flector {
    var self_1753: Flector;
    var other_1445: Motor;

    self_1753 = self_1752;
    other_1445 = other_1444;
    let _e4: Flector = self_1753;
    let _e6: Motor = other_1445;
    let _e11: Flector = self_1753;
    let _e15: Motor = other_1445;
    let _e27: Flector = self_1753;
    let _e31: Motor = other_1445;
    let _e44: Flector = self_1753;
    let _e48: Motor = other_1445;
    let _e51: Motor = other_1445;
    let _e54: Motor = other_1445;
    let _e57: Motor = other_1445;
    let _e70: Flector = self_1753;
    let _e74: Motor = other_1445;
    let _e77: Motor = other_1445;
    let _e80: Motor = other_1445;
    let _e83: Motor = other_1445;
    let _e96: Flector = self_1753;
    let _e100: Motor = other_1445;
    let _e103: Motor = other_1445;
    let _e106: Motor = other_1445;
    let _e109: Motor = other_1445;
    let _e122: Flector = self_1753;
    let _e126: Motor = other_1445;
    let _e138: Flector = self_1753;
    let _e141: Motor = other_1445;
    return Flector((_e4.g0_ * vec4<f32>(_e6.g0_.w)), ((((((((vec4<f32>(_e11.g0_.y) * vec4<f32>(_e15.g0_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0))) + ((vec4<f32>(_e27.g0_.z) * vec4<f32>(_e31.g0_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e44.g1_.x) * vec4<f32>(_e48.g0_.w, _e51.g0_.z, _e54.g0_.y, _e57.g1_.x)) * vec4<f32>(1.0, -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e70.g1_.y) * vec4<f32>(_e74.g0_.z, _e77.g0_.w, _e80.g0_.x, _e83.g1_.y)) * vec4<f32>(1.0, 1.0, -(1.0), 1.0))) + ((vec4<f32>(_e96.g1_.z) * vec4<f32>(_e100.g0_.y, _e103.g0_.x, _e106.g0_.w, _e109.g1_.z)) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0))) + ((vec4<f32>(_e122.g1_.w) * vec4<f32>(_e126.g0_.w)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((_e138.g0_.wwwx * _e141.g0_.xyzx) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))));
}

fn flector_motor_right_contraction(self_1754: Flector, other_1446: Motor) -> Point {
    var self_1755: Flector;
    var other_1447: Motor;

    self_1755 = self_1754;
    other_1447 = other_1446;
    let _e4: Flector = self_1755;
    let _e8: Motor = other_1447;
    let _e20: Flector = self_1755;
    let _e24: Motor = other_1447;
    let _e37: Flector = self_1755;
    let _e40: Motor = other_1447;
    let _e43: Motor = other_1447;
    let _e46: Motor = other_1447;
    let _e49: Motor = other_1447;
    return Point(((((vec4<f32>(_e4.g1_.y) * vec4<f32>(_e8.g1_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0))) + ((vec4<f32>(_e20.g1_.z) * vec4<f32>(_e24.g1_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((_e37.g1_.wwwx * vec4<f32>(_e40.g1_.x, _e43.g1_.y, _e46.g1_.z, _e49.g1_.x)) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))));
}

fn flector_motor_left_anti_contraction(self_1756: Flector, other_1448: Motor) -> Plane {
    var self_1757: Flector;
    var other_1449: Motor;

    self_1757 = self_1756;
    other_1449 = other_1448;
    let _e4: Flector = self_1757;
    let _e8: Motor = other_1449;
    let _e11: Motor = other_1449;
    let _e14: Motor = other_1449;
    let _e17: Motor = other_1449;
    let _e29: Flector = self_1757;
    let _e33: Motor = other_1449;
    let _e36: Motor = other_1449;
    let _e39: Motor = other_1449;
    let _e42: Motor = other_1449;
    let _e55: Flector = self_1757;
    let _e59: Motor = other_1449;
    let _e62: Motor = other_1449;
    let _e65: Motor = other_1449;
    let _e68: Motor = other_1449;
    return Plane(((((vec4<f32>(_e4.g1_.y) * vec4<f32>(_e8.g0_.z, _e11.g0_.z, _e14.g0_.x, _e17.g1_.y)) * vec4<f32>(1.0, 0.0, -(1.0), 1.0)) + ((vec4<f32>(_e29.g1_.z) * vec4<f32>(_e33.g0_.y, _e36.g0_.x, _e39.g0_.y, _e42.g1_.z)) * vec4<f32>(-(1.0), 1.0, 0.0, 1.0))) + ((vec4<f32>(_e55.g1_.x) * vec4<f32>(_e59.g0_.x, _e62.g0_.z, _e65.g0_.y, _e68.g1_.x)) * vec4<f32>(0.0, -(1.0), 1.0, 1.0))));
}

fn flector_motor_right_anti_contraction(self_1758: Flector, other_1450: Motor) -> Flector {
    var self_1759: Flector;
    var other_1451: Motor;

    self_1759 = self_1758;
    other_1451 = other_1450;
    let _e4: Flector = self_1759;
    let _e6: Motor = other_1451;
    let _e11: Flector = self_1759;
    let _e15: Motor = other_1451;
    let _e27: Flector = self_1759;
    let _e31: Motor = other_1451;
    let _e44: Flector = self_1759;
    let _e48: Motor = other_1451;
    let _e60: Flector = self_1759;
    let _e64: Motor = other_1451;
    let _e76: Flector = self_1759;
    let _e80: Motor = other_1451;
    let _e92: Flector = self_1759;
    let _e96: Motor = other_1451;
    let _e108: Flector = self_1759;
    let _e111: Motor = other_1451;
    return Flector((_e4.g0_ * vec4<f32>(_e6.g0_.w)), ((((((((vec4<f32>(_e11.g0_.y) * vec4<f32>(_e15.g0_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0))) + ((vec4<f32>(_e27.g0_.z) * vec4<f32>(_e31.g0_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e44.g1_.x) * vec4<f32>(_e48.g0_.w)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e60.g1_.y) * vec4<f32>(_e64.g0_.w)) * vec4<f32>(0.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e76.g1_.z) * vec4<f32>(_e80.g0_.w)) * vec4<f32>(0.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e92.g1_.w) * vec4<f32>(_e96.g0_.w)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((_e108.g0_.wwwx * _e111.g0_.xyzx) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))));
}

fn flector_rotor_regressive_product(self_1760: Flector, other_1452: Rotor) -> Flector {
    var self_1761: Flector;
    var other_1453: Rotor;

    self_1761 = self_1760;
    other_1453 = other_1452;
    let _e4: Flector = self_1761;
    let _e8: Rotor = other_1453;
    let _e20: Flector = self_1761;
    let _e24: Rotor = other_1453;
    let _e37: Flector = self_1761;
    let _e41: Rotor = other_1453;
    let _e54: Flector = self_1761;
    let _e58: Rotor = other_1453;
    let _e69: Flector = self_1761;
    let _e71: Rotor = other_1453;
    let _e77: Flector = self_1761;
    let _e79: Rotor = other_1453;
    return Flector(((((((vec4<f32>(_e4.g1_.x) * vec4<f32>(_e8.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0))) + ((vec4<f32>(_e20.g1_.y) * vec4<f32>(_e24.g0_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e37.g1_.z) * vec4<f32>(_e41.g0_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e54.g1_.w) * _e58.g0_.xyzx) * vec4<f32>(1.0, 1.0, 1.0, 0.0))) + (_e69.g0_ * vec4<f32>(_e71.g0_.w))), (_e77.g1_ * vec4<f32>(_e79.g0_.w)));
}

fn flector_rotor_geometric_anti_product(self_1762: Flector, other_1454: Rotor) -> Flector {
    var self_1763: Flector;
    var other_1455: Rotor;

    self_1763 = self_1762;
    other_1455 = other_1454;
    let _e4: Flector = self_1763;
    let _e8: Rotor = other_1455;
    let _e19: Flector = self_1763;
    let _e23: Rotor = other_1455;
    let _e35: Flector = self_1763;
    let _e39: Rotor = other_1455;
    let _e52: Flector = self_1763;
    let _e56: Rotor = other_1455;
    let _e69: Flector = self_1763;
    let _e73: Rotor = other_1455;
    let _e86: Flector = self_1763;
    let _e90: Rotor = other_1455;
    let _e101: Flector = self_1763;
    let _e104: Rotor = other_1455;
    let _e116: Flector = self_1763;
    let _e120: Rotor = other_1455;
    let _e132: Flector = self_1763;
    let _e136: Rotor = other_1455;
    let _e149: Flector = self_1763;
    let _e153: Rotor = other_1455;
    let _e165: Flector = self_1763;
    let _e169: Rotor = other_1455;
    let _e181: Flector = self_1763;
    let _e185: Rotor = other_1455;
    let _e197: Flector = self_1763;
    let _e201: Rotor = other_1455;
    let _e213: Flector = self_1763;
    let _e216: Rotor = other_1455;
    return Flector(((((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.zwxz) * vec4<f32>(1.0, 1.0, -(1.0), 0.0)) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.yxwy) * vec4<f32>(-(1.0), 1.0, 1.0, 0.0))) + ((vec4<f32>(_e35.g1_.x) * vec4<f32>(_e39.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e52.g1_.y) * vec4<f32>(_e56.g0_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e69.g1_.z) * vec4<f32>(_e73.g0_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e86.g1_.w) * _e90.g0_.xyzx) * vec4<f32>(1.0, 1.0, 1.0, 0.0))) + ((_e101.g0_.xxxw * _e104.g0_.wzyw) * vec4<f32>(1.0, -(1.0), 1.0, 1.0))), ((((((((vec4<f32>(_e116.g0_.y) * vec4<f32>(_e120.g0_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0))) + ((vec4<f32>(_e132.g0_.z) * vec4<f32>(_e136.g0_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e149.g1_.x) * _e153.g0_.wzyw) * vec4<f32>(1.0, -(1.0), 1.0, 0.0))) + ((vec4<f32>(_e165.g1_.y) * _e169.g0_.zwxz) * vec4<f32>(1.0, 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e181.g1_.z) * _e185.g0_.yxwy) * vec4<f32>(-(1.0), 1.0, 1.0, 0.0))) + ((vec4<f32>(_e197.g1_.w) * vec4<f32>(_e201.g0_.w)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((_e213.g0_.wwwx * _e216.g0_.xyzx) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))));
}

fn flector_rotor_inner_anti_product(self_1764: Flector, other_1456: Rotor) -> Flector {
    var self_1765: Flector;
    var other_1457: Rotor;

    self_1765 = self_1764;
    other_1457 = other_1456;
    let _e4: Flector = self_1765;
    let _e6: Rotor = other_1457;
    let _e11: Flector = self_1765;
    let _e15: Rotor = other_1457;
    let _e27: Flector = self_1765;
    let _e31: Rotor = other_1457;
    let _e44: Flector = self_1765;
    let _e48: Rotor = other_1457;
    let _e60: Flector = self_1765;
    let _e64: Rotor = other_1457;
    let _e76: Flector = self_1765;
    let _e80: Rotor = other_1457;
    let _e92: Flector = self_1765;
    let _e96: Rotor = other_1457;
    let _e108: Flector = self_1765;
    let _e111: Rotor = other_1457;
    return Flector((_e4.g0_ * vec4<f32>(_e6.g0_.w)), ((((((((vec4<f32>(_e11.g0_.y) * vec4<f32>(_e15.g0_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0))) + ((vec4<f32>(_e27.g0_.z) * vec4<f32>(_e31.g0_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e44.g1_.x) * _e48.g0_.wzyw) * vec4<f32>(1.0, -(1.0), 1.0, 0.0))) + ((vec4<f32>(_e60.g1_.y) * _e64.g0_.zwxz) * vec4<f32>(1.0, 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e76.g1_.z) * _e80.g0_.yxwy) * vec4<f32>(-(1.0), 1.0, 1.0, 0.0))) + ((vec4<f32>(_e92.g1_.w) * vec4<f32>(_e96.g0_.w)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((_e108.g0_.wwwx * _e111.g0_.xyzx) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))));
}

fn flector_rotor_right_anti_contraction(self_1766: Flector, other_1458: Rotor) -> Flector {
    var self_1767: Flector;
    var other_1459: Rotor;

    self_1767 = self_1766;
    other_1459 = other_1458;
    let _e4: Flector = self_1767;
    let _e6: Rotor = other_1459;
    let _e11: Flector = self_1767;
    let _e15: Rotor = other_1459;
    let _e27: Flector = self_1767;
    let _e31: Rotor = other_1459;
    let _e44: Flector = self_1767;
    let _e48: Rotor = other_1459;
    let _e60: Flector = self_1767;
    let _e64: Rotor = other_1459;
    let _e76: Flector = self_1767;
    let _e80: Rotor = other_1459;
    let _e92: Flector = self_1767;
    let _e96: Rotor = other_1459;
    let _e108: Flector = self_1767;
    let _e111: Rotor = other_1459;
    return Flector((_e4.g0_ * vec4<f32>(_e6.g0_.w)), ((((((((vec4<f32>(_e11.g0_.y) * vec4<f32>(_e15.g0_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0))) + ((vec4<f32>(_e27.g0_.z) * vec4<f32>(_e31.g0_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e44.g1_.x) * vec4<f32>(_e48.g0_.w)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e60.g1_.y) * vec4<f32>(_e64.g0_.w)) * vec4<f32>(0.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e76.g1_.z) * vec4<f32>(_e80.g0_.w)) * vec4<f32>(0.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e92.g1_.w) * vec4<f32>(_e96.g0_.w)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((_e108.g0_.wwwx * _e111.g0_.xyzx) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))));
}

fn flector_translator_geometric_product(self_1768: Flector, other_1460: Translator) -> Flector {
    var self_1769: Flector;
    var other_1461: Translator;

    self_1769 = self_1768;
    other_1461 = other_1460;
    let _e4: Flector = self_1769;
    let _e8: Translator = other_1461;
    let _e19: Flector = self_1769;
    let _e23: Translator = other_1461;
    let _e35: Flector = self_1769;
    let _e39: Translator = other_1461;
    let _e52: Flector = self_1769;
    let _e56: Translator = other_1461;
    let _e69: Flector = self_1769;
    let _e73: Translator = other_1461;
    let _e77: Flector = self_1769;
    let _e80: Flector = self_1769;
    let _e83: Flector = self_1769;
    let _e86: Flector = self_1769;
    let _e90: Translator = other_1461;
    let _e103: Flector = self_1769;
    let _e107: Translator = other_1461;
    let _e118: Flector = self_1769;
    let _e122: Translator = other_1461;
    let _e134: Flector = self_1769;
    let _e138: Translator = other_1461;
    let _e149: Flector = self_1769;
    let _e153: Translator = other_1461;
    let _e165: Flector = self_1769;
    let _e169: Translator = other_1461;
    let _e181: Flector = self_1769;
    let _e184: Flector = self_1769;
    let _e187: Flector = self_1769;
    let _e190: Flector = self_1769;
    let _e194: Translator = other_1461;
    return Flector((((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.zzxz) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0)) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.yxyy) * vec4<f32>(1.0, -(1.0), 0.0, 0.0))) + ((vec4<f32>(_e35.g1_.y) * vec4<f32>(_e39.g0_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e52.g1_.z) * vec4<f32>(_e56.g0_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + (vec4<f32>(_e69.g1_.w) * _e73.g0_)) + ((vec4<f32>(_e77.g0_.x, _e80.g0_.x, _e83.g0_.x, _e86.g1_.x) * _e90.g0_.xzyx) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))), (((((((vec4<f32>(_e103.g0_.y) * _e107.g0_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e118.g0_.z) * _e122.g0_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e134.g0_.w) * _e138.g0_.xyzx) * vec4<f32>(1.0, 1.0, 1.0, 0.0))) + ((vec4<f32>(_e149.g1_.y) * _e153.g0_.zzxz) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0))) + ((vec4<f32>(_e165.g1_.z) * _e169.g0_.yxyy) * vec4<f32>(1.0, -(1.0), 0.0, 0.0))) + ((vec4<f32>(_e181.g0_.x, _e184.g1_.x, _e187.g1_.x, _e190.g0_.x) * _e194.g0_.wzyx) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))));
}

fn flector_translator_regressive_product(self_1770: Flector, other_1462: Translator) -> Flector {
    var self_1771: Flector;
    var other_1463: Translator;

    self_1771 = self_1770;
    other_1463 = other_1462;
    let _e4: Flector = self_1771;
    let _e8: Translator = other_1463;
    let _e19: Flector = self_1771;
    let _e23: Translator = other_1463;
    let _e35: Flector = self_1771;
    let _e39: Translator = other_1463;
    let _e51: Flector = self_1771;
    let _e53: Translator = other_1463;
    let _e59: Flector = self_1771;
    let _e61: Translator = other_1463;
    return Flector((((((vec4<f32>(_e4.g1_.x) * _e8.g0_.zzyz) * vec4<f32>(0.0, 1.0, -(1.0), 0.0)) + ((vec4<f32>(_e19.g1_.y) * _e23.g0_.zzxz) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0))) + ((vec4<f32>(_e35.g1_.z) * _e39.g0_.yxyy) * vec4<f32>(1.0, -(1.0), 0.0, 0.0))) + (_e51.g0_ * vec4<f32>(_e53.g0_.w))), (_e59.g1_ * vec4<f32>(_e61.g0_.w)));
}

fn flector_translator_outer_product(self_1772: Flector, other_1464: Translator) -> Plane {
    var self_1773: Flector;
    var other_1465: Translator;

    self_1773 = self_1772;
    other_1465 = other_1464;
    let _e4: Flector = self_1773;
    let _e8: Translator = other_1465;
    let _e20: Flector = self_1773;
    let _e24: Translator = other_1465;
    let _e37: Flector = self_1773;
    let _e40: Translator = other_1465;
    return Plane(((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0))) + ((vec4<f32>(_e20.g0_.z) * vec4<f32>(_e24.g0_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((_e37.g0_.wwwx * _e40.g0_.xyzx) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))));
}

fn flector_translator_geometric_anti_product(self_1774: Flector, other_1466: Translator) -> Flector {
    var self_1775: Flector;
    var other_1467: Translator;

    self_1775 = self_1774;
    other_1467 = other_1466;
    let _e4: Flector = self_1775;
    let _e8: Translator = other_1467;
    let _e20: Flector = self_1775;
    let _e24: Translator = other_1467;
    let _e36: Flector = self_1775;
    let _e40: Translator = other_1467;
    let _e52: Flector = self_1775;
    let _e56: Translator = other_1467;
    let _e68: Flector = self_1775;
    let _e71: Translator = other_1467;
    let _e82: Flector = self_1775;
    let _e86: Translator = other_1467;
    let _e96: Flector = self_1775;
    let _e100: Translator = other_1467;
    let _e111: Flector = self_1775;
    let _e115: Translator = other_1467;
    let _e127: Flector = self_1775;
    let _e131: Translator = other_1467;
    return Flector(((((((vec4<f32>(_e4.g0_.w) * _e8.g0_) * vec4<f32>(-(1.0), -(1.0), -(1.0), 1.0)) + ((vec4<f32>(_e20.g1_.x) * _e24.g0_.zzyz) * vec4<f32>(0.0, 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e36.g1_.y) * _e40.g0_.zzxz) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0))) + ((vec4<f32>(_e52.g1_.z) * _e56.g0_.yxyy) * vec4<f32>(1.0, -(1.0), 0.0, 0.0))) + ((_e68.g0_.xyzx * _e71.g0_.wwwx) * vec4<f32>(1.0, 1.0, 1.0, 0.0))), (((((vec4<f32>(_e82.g1_.y) * _e86.g0_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, 1.0)) + ((vec4<f32>(_e96.g1_.z) * _e100.g0_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, 1.0))) + ((vec4<f32>(_e111.g1_.w) * vec4<f32>(_e115.g0_.w)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e127.g1_.x) * _e131.g0_.wxxx) * vec4<f32>(1.0, 0.0, 0.0, 1.0))));
}

fn flector_translator_inner_anti_product(self_1776: Flector, other_1468: Translator) -> Flector {
    var self_1777: Flector;
    var other_1469: Translator;

    self_1777 = self_1776;
    other_1469 = other_1468;
    let _e4: Flector = self_1777;
    let _e6: Translator = other_1469;
    let _e11: Flector = self_1777;
    let _e15: Translator = other_1469;
    let _e25: Flector = self_1777;
    let _e29: Translator = other_1469;
    let _e40: Flector = self_1777;
    let _e44: Translator = other_1469;
    let _e56: Flector = self_1777;
    let _e60: Translator = other_1469;
    return Flector((_e4.g0_ * vec4<f32>(_e6.g0_.w)), (((((vec4<f32>(_e11.g1_.y) * _e15.g0_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, 1.0)) + ((vec4<f32>(_e25.g1_.z) * _e29.g0_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, 1.0))) + ((vec4<f32>(_e40.g1_.w) * vec4<f32>(_e44.g0_.w)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e56.g1_.x) * _e60.g0_.wxxx) * vec4<f32>(1.0, 0.0, 0.0, 1.0))));
}

fn flector_translator_right_contraction(self_1778: Flector, other_1470: Translator) -> Point {
    var self_1779: Flector;
    var other_1471: Translator;

    self_1779 = self_1778;
    other_1471 = other_1470;
    let _e4: Flector = self_1779;
    let _e8: Translator = other_1471;
    let _e20: Flector = self_1779;
    let _e24: Translator = other_1471;
    let _e37: Flector = self_1779;
    let _e40: Translator = other_1471;
    return Point(((((vec4<f32>(_e4.g1_.y) * vec4<f32>(_e8.g0_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0))) + ((vec4<f32>(_e20.g1_.z) * vec4<f32>(_e24.g0_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((_e37.g1_.wwwx * _e40.g0_.xyzx) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))));
}

fn flector_translator_right_anti_contraction(self_1780: Flector, other_1472: Translator) -> Flector {
    var self_1781: Flector;
    var other_1473: Translator;

    self_1781 = self_1780;
    other_1473 = other_1472;
    let _e4: Flector = self_1781;
    let _e6: Translator = other_1473;
    let _e11: Flector = self_1781;
    let _e13: Translator = other_1473;
    return Flector((_e4.g0_ * vec4<f32>(_e6.g0_.w)), (_e11.g1_ * vec4<f32>(_e13.g0_.w)));
}

fn flector_flector_add(self_1782: Flector, other_1474: Flector) -> Flector {
    var self_1783: Flector;
    var other_1475: Flector;

    self_1783 = self_1782;
    other_1475 = other_1474;
    let _e4: Flector = self_1783;
    let _e6: Flector = other_1475;
    let _e9: Flector = self_1783;
    let _e11: Flector = other_1475;
    return Flector((_e4.g0_ + _e6.g0_), (_e9.g1_ + _e11.g1_));
}

fn flector_flector_sub(self_1784: Flector, other_1476: Flector) -> Flector {
    var self_1785: Flector;
    var other_1477: Flector;

    self_1785 = self_1784;
    other_1477 = other_1476;
    let _e4: Flector = self_1785;
    let _e6: Flector = other_1477;
    let _e9: Flector = self_1785;
    let _e11: Flector = other_1477;
    return Flector((_e4.g0_ - _e6.g0_), (_e9.g1_ - _e11.g1_));
}

fn flector_flector_mul(self_1786: Flector, other_1478: Flector) -> Flector {
    var self_1787: Flector;
    var other_1479: Flector;

    self_1787 = self_1786;
    other_1479 = other_1478;
    let _e4: Flector = self_1787;
    let _e6: Flector = other_1479;
    let _e9: Flector = self_1787;
    let _e11: Flector = other_1479;
    return Flector((_e4.g0_ * _e6.g0_), (_e9.g1_ * _e11.g1_));
}

fn flector_flector_div(self_1788: Flector, other_1480: Flector) -> Flector {
    var self_1789: Flector;
    var other_1481: Flector;

    self_1789 = self_1788;
    other_1481 = other_1480;
    let _e4: Flector = self_1789;
    let _e7: Flector = self_1789;
    let _e10: Flector = self_1789;
    let _e13: Flector = self_1789;
    let _e23: Flector = other_1481;
    let _e26: Flector = other_1481;
    let _e29: Flector = other_1481;
    let _e32: Flector = other_1481;
    let _e43: Flector = self_1789;
    let _e46: Flector = self_1789;
    let _e49: Flector = self_1789;
    let _e52: Flector = self_1789;
    let _e62: Flector = other_1481;
    let _e65: Flector = other_1481;
    let _e68: Flector = other_1481;
    let _e71: Flector = other_1481;
    return Flector((((vec4<f32>(_e4.g0_.x, _e7.g0_.y, _e10.g0_.z, _e13.g0_.w) * vec4<f32>(1.0, 1.0, 1.0, 1.0)) / vec4<f32>(_e23.g0_.x, _e26.g0_.y, _e29.g0_.z, _e32.g0_.w)) * vec4<f32>(1.0, 1.0, 1.0, 1.0)), (((vec4<f32>(_e43.g1_.x, _e46.g1_.y, _e49.g1_.z, _e52.g1_.w) * vec4<f32>(1.0, 1.0, 1.0, 1.0)) / vec4<f32>(_e62.g1_.x, _e65.g1_.y, _e68.g1_.z, _e71.g1_.w)) * vec4<f32>(1.0, 1.0, 1.0, 1.0)));
}

fn flector_flector_outer_product(self_1790: Flector, other_1482: Flector) -> Motor {
    var self_1791: Flector;
    var other_1483: Flector;

    self_1791 = self_1790;
    other_1483 = other_1482;
    let _e4: Flector = self_1791;
    let _e8: Flector = other_1483;
    let _e11: Flector = other_1483;
    let _e14: Flector = other_1483;
    let _e17: Flector = other_1483;
    let _e29: Flector = self_1791;
    let _e33: Flector = other_1483;
    let _e36: Flector = other_1483;
    let _e39: Flector = other_1483;
    let _e42: Flector = other_1483;
    let _e55: Flector = self_1791;
    let _e59: Flector = other_1483;
    let _e62: Flector = other_1483;
    let _e65: Flector = other_1483;
    let _e68: Flector = other_1483;
    let _e74: Flector = self_1791;
    let _e78: Flector = other_1483;
    let _e91: Flector = self_1791;
    let _e95: Flector = other_1483;
    let _e108: Flector = self_1791;
    let _e112: Flector = other_1483;
    let _e125: Flector = self_1791;
    let _e129: Flector = other_1483;
    let _e142: Flector = self_1791;
    let _e146: Flector = other_1483;
    let _e149: Flector = other_1483;
    let _e152: Flector = other_1483;
    let _e155: Flector = other_1483;
    let _e168: Flector = self_1791;
    let _e172: Flector = other_1483;
    let _e175: Flector = other_1483;
    let _e178: Flector = other_1483;
    let _e189: Flector = self_1791;
    let _e193: Flector = other_1483;
    let _e196: Flector = other_1483;
    let _e199: Flector = other_1483;
    let _e211: Flector = self_1791;
    let _e215: Flector = other_1483;
    let _e218: Flector = other_1483;
    let _e221: Flector = other_1483;
    return Motor((((((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.w, _e11.g0_.w, _e14.g0_.w, _e17.g1_.y)) * vec4<f32>(0.0, -(1.0), 0.0, 1.0)) + ((vec4<f32>(_e29.g0_.z) * vec4<f32>(_e33.g0_.w, _e36.g0_.w, _e39.g0_.w, _e42.g1_.z)) * vec4<f32>(0.0, 0.0, -(1.0), 1.0))) + (vec4<f32>(_e55.g0_.w) * vec4<f32>(_e59.g0_.x, _e62.g0_.y, _e65.g0_.z, _e68.g1_.w))) + ((vec4<f32>(_e74.g1_.x) * vec4<f32>(_e78.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e91.g1_.y) * vec4<f32>(_e95.g0_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e108.g1_.z) * vec4<f32>(_e112.g0_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e125.g1_.w) * vec4<f32>(_e129.g0_.w)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e142.g0_.x) * vec4<f32>(_e146.g0_.w, _e149.g0_.x, _e152.g0_.x, _e155.g1_.x)) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))), ((((vec3<f32>(_e168.g0_.y) * vec3<f32>(_e172.g0_.z, _e175.g0_.z, _e178.g0_.x)) * vec3<f32>(1.0, 0.0, -(1.0))) + ((vec3<f32>(_e189.g0_.z) * vec3<f32>(_e193.g0_.y, _e196.g0_.x, _e199.g0_.y)) * vec3<f32>(-(1.0), 1.0, 0.0))) + ((vec3<f32>(_e211.g0_.x) * vec3<f32>(_e215.g0_.x, _e218.g0_.z, _e221.g0_.y)) * vec3<f32>(0.0, -(1.0), 1.0))));
}

fn flector_flector_inner_anti_product(self_1792: Flector, other_1484: Flector) -> Motor {
    var self_1793: Flector;
    var other_1485: Flector;

    self_1793 = self_1792;
    other_1485 = other_1484;
    let _e6: Flector = self_1793;
    let _e10: Flector = other_1485;
    let _e13: Flector = other_1485;
    let _e16: Flector = other_1485;
    let _e19: Flector = other_1485;
    let _e25: Flector = self_1793;
    let _e29: Flector = other_1485;
    let _e32: Flector = other_1485;
    let _e35: Flector = other_1485;
    let _e38: Flector = other_1485;
    let _e51: Flector = self_1793;
    let _e55: Flector = other_1485;
    let _e58: Flector = other_1485;
    let _e61: Flector = other_1485;
    let _e64: Flector = other_1485;
    let _e77: Flector = self_1793;
    let _e81: Flector = other_1485;
    let _e84: Flector = other_1485;
    let _e87: Flector = other_1485;
    let _e90: Flector = other_1485;
    let _e103: Flector = self_1793;
    let _e107: Flector = other_1485;
    let _e110: Flector = other_1485;
    let _e113: Flector = other_1485;
    let _e124: Flector = self_1793;
    let _e128: Flector = other_1485;
    let _e131: Flector = other_1485;
    let _e134: Flector = other_1485;
    let _e146: Flector = self_1793;
    let _e150: Flector = other_1485;
    let _e153: Flector = other_1485;
    let _e156: Flector = other_1485;
    let _e168: Flector = self_1793;
    let _e172: Flector = other_1485;
    let _e175: Flector = other_1485;
    let _e178: Flector = other_1485;
    let _e190: Flector = self_1793;
    let _e194: Flector = other_1485;
    let _e197: Flector = other_1485;
    let _e200: Flector = other_1485;
    let _e212: Flector = self_1793;
    let _e216: Flector = other_1485;
    let _e219: Flector = other_1485;
    let _e222: Flector = other_1485;
    return Motor(((((vec4<f32>(0.0) - (vec4<f32>(_e6.g0_.w) * vec4<f32>(_e10.g1_.x, _e13.g1_.y, _e16.g1_.z, _e19.g0_.w))) + ((vec4<f32>(_e25.g1_.y) * vec4<f32>(_e29.g0_.w, _e32.g0_.w, _e35.g0_.w, _e38.g1_.y)) * vec4<f32>(0.0, -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e51.g1_.z) * vec4<f32>(_e55.g0_.w, _e58.g0_.w, _e61.g0_.w, _e64.g1_.z)) * vec4<f32>(0.0, 0.0, -(1.0), 1.0))) + ((vec4<f32>(_e77.g1_.x) * vec4<f32>(_e81.g0_.w, _e84.g0_.x, _e87.g0_.x, _e90.g1_.x)) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))), (((((((vec3<f32>(_e103.g0_.y) * vec3<f32>(_e107.g1_.z, _e110.g1_.z, _e113.g1_.x)) * vec3<f32>(-(1.0), 0.0, 1.0)) + ((vec3<f32>(_e124.g0_.z) * vec3<f32>(_e128.g1_.y, _e131.g1_.x, _e134.g1_.y)) * vec3<f32>(1.0, -(1.0), 0.0))) + ((vec3<f32>(_e146.g1_.x) * vec3<f32>(_e150.g0_.z, _e153.g0_.z, _e156.g0_.y)) * vec3<f32>(0.0, -(1.0), 1.0))) + ((vec3<f32>(_e168.g1_.y) * vec3<f32>(_e172.g0_.z, _e175.g0_.z, _e178.g0_.x)) * vec3<f32>(1.0, 0.0, -(1.0)))) + ((vec3<f32>(_e190.g1_.z) * vec3<f32>(_e194.g0_.y, _e197.g0_.x, _e200.g0_.y)) * vec3<f32>(-(1.0), 1.0, 0.0))) + ((vec3<f32>(_e212.g0_.x) * vec3<f32>(_e216.g1_.x, _e219.g1_.z, _e222.g1_.y)) * vec3<f32>(0.0, 1.0, -(1.0)))));
}

fn flector_flector_left_anti_contraction(self_1794: Flector, other_1486: Flector) -> Motor {
    var self_1795: Flector;
    var other_1487: Flector;

    self_1795 = self_1794;
    other_1487 = other_1486;
    let _e4: Flector = self_1795;
    let _e8: Flector = other_1487;
    let _e11: Flector = other_1487;
    let _e14: Flector = other_1487;
    let _e17: Flector = other_1487;
    let _e29: Flector = self_1795;
    let _e33: Flector = other_1487;
    let _e36: Flector = other_1487;
    let _e39: Flector = other_1487;
    let _e42: Flector = other_1487;
    let _e55: Flector = self_1795;
    let _e59: Flector = other_1487;
    let _e62: Flector = other_1487;
    let _e65: Flector = other_1487;
    let _e68: Flector = other_1487;
    let _e81: Flector = self_1795;
    let _e84: Flector = other_1487;
    let _e96: Flector = self_1795;
    let _e100: Flector = other_1487;
    let _e103: Flector = other_1487;
    let _e106: Flector = other_1487;
    let _e117: Flector = self_1795;
    let _e121: Flector = other_1487;
    let _e124: Flector = other_1487;
    let _e127: Flector = other_1487;
    let _e139: Flector = self_1795;
    let _e143: Flector = other_1487;
    let _e146: Flector = other_1487;
    let _e149: Flector = other_1487;
    return Motor((((((vec4<f32>(_e4.g1_.x) * vec4<f32>(_e8.g0_.w, _e11.g0_.w, _e14.g0_.w, _e17.g1_.x)) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0)) + ((vec4<f32>(_e29.g1_.y) * vec4<f32>(_e33.g0_.w, _e36.g0_.w, _e39.g0_.w, _e42.g1_.y)) * vec4<f32>(0.0, -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e55.g1_.z) * vec4<f32>(_e59.g0_.w, _e62.g0_.w, _e65.g0_.w, _e68.g1_.z)) * vec4<f32>(0.0, 0.0, -(1.0), 1.0))) + ((_e81.g0_.xxxw * _e84.g0_.xxxw) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))), ((((vec3<f32>(_e96.g1_.y) * vec3<f32>(_e100.g0_.z, _e103.g0_.z, _e106.g0_.x)) * vec3<f32>(1.0, 0.0, -(1.0))) + ((vec3<f32>(_e117.g1_.z) * vec3<f32>(_e121.g0_.y, _e124.g0_.x, _e127.g0_.y)) * vec3<f32>(-(1.0), 1.0, 0.0))) + ((vec3<f32>(_e139.g1_.x) * vec3<f32>(_e143.g0_.x, _e146.g0_.z, _e149.g0_.y)) * vec3<f32>(0.0, -(1.0), 1.0))));
}

fn flector_flector_right_anti_contraction(self_1796: Flector, other_1488: Flector) -> Motor {
    var self_1797: Flector;
    var other_1489: Flector;

    self_1797 = self_1796;
    other_1489 = other_1488;
    let _e6: Flector = self_1797;
    let _e10: Flector = other_1489;
    let _e13: Flector = other_1489;
    let _e16: Flector = other_1489;
    let _e19: Flector = other_1489;
    let _e25: Flector = self_1797;
    let _e29: Flector = other_1489;
    let _e41: Flector = self_1797;
    let _e45: Flector = other_1489;
    let _e57: Flector = self_1797;
    let _e61: Flector = other_1489;
    let _e73: Flector = self_1797;
    let _e77: Flector = other_1489;
    let _e80: Flector = other_1489;
    let _e83: Flector = other_1489;
    let _e94: Flector = self_1797;
    let _e98: Flector = other_1489;
    let _e101: Flector = other_1489;
    let _e104: Flector = other_1489;
    let _e116: Flector = self_1797;
    let _e120: Flector = other_1489;
    let _e123: Flector = other_1489;
    let _e126: Flector = other_1489;
    return Motor(((((vec4<f32>(0.0) - (vec4<f32>(_e6.g0_.w) * vec4<f32>(_e10.g1_.x, _e13.g1_.y, _e16.g1_.z, _e19.g0_.w))) + ((vec4<f32>(_e25.g1_.y) * vec4<f32>(_e29.g1_.y)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e41.g1_.z) * vec4<f32>(_e45.g1_.z)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e57.g1_.x) * vec4<f32>(_e61.g1_.x)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))), ((((vec3<f32>(_e73.g0_.y) * vec3<f32>(_e77.g1_.z, _e80.g1_.z, _e83.g1_.x)) * vec3<f32>(-(1.0), 0.0, 1.0)) + ((vec3<f32>(_e94.g0_.z) * vec3<f32>(_e98.g1_.y, _e101.g1_.x, _e104.g1_.y)) * vec3<f32>(1.0, -(1.0), 0.0))) + ((vec3<f32>(_e116.g0_.x) * vec3<f32>(_e120.g1_.x, _e123.g1_.z, _e126.g1_.y)) * vec3<f32>(0.0, 1.0, -(1.0)))));
}

fn flector_flector_scalar_product(self_1798: Flector, other_1490: Flector) -> Scalar {
    var self_1799: Flector;
    var other_1491: Flector;

    self_1799 = self_1798;
    other_1491 = other_1490;
    let _e4: Flector = self_1799;
    let _e7: Flector = other_1491;
    let _e11: Flector = self_1799;
    let _e14: Flector = other_1491;
    let _e19: Flector = self_1799;
    let _e22: Flector = other_1491;
    let _e27: Flector = self_1799;
    let _e30: Flector = other_1491;
    return Scalar(((((_e4.g0_.x * _e7.g0_.x) + (_e11.g0_.y * _e14.g0_.y)) + (_e19.g0_.z * _e22.g0_.z)) - (_e27.g1_.w * _e30.g1_.w)));
}

fn flector_flector_anti_scalar_product(self_1800: Flector, other_1492: Flector) -> AntiScalar {
    var self_1801: Flector;
    var other_1493: Flector;

    self_1801 = self_1800;
    other_1493 = other_1492;
    let _e5: Flector = self_1801;
    let _e8: Flector = other_1493;
    let _e13: Flector = self_1801;
    let _e16: Flector = other_1493;
    let _e21: Flector = self_1801;
    let _e24: Flector = other_1493;
    let _e29: Flector = self_1801;
    let _e32: Flector = other_1493;
    return AntiScalar(((((0.0 - (_e5.g0_.w * _e8.g0_.w)) + (_e13.g1_.x * _e16.g1_.x)) + (_e21.g1_.y * _e24.g1_.y)) + (_e29.g1_.z * _e32.g1_.z)));
}

fn flector_multi_vector_scalar_product(self_1802: Flector, other_1494: MultiVector) -> Scalar {
    var self_1803: Flector;
    var other_1495: MultiVector;

    self_1803 = self_1802;
    other_1495 = other_1494;
    let _e4: Flector = self_1803;
    let _e7: MultiVector = other_1495;
    let _e11: Flector = self_1803;
    let _e14: MultiVector = other_1495;
    let _e19: Flector = self_1803;
    let _e22: MultiVector = other_1495;
    let _e27: Flector = self_1803;
    let _e30: MultiVector = other_1495;
    return Scalar(((((_e4.g0_.x * _e7.g1_.x) + (_e11.g0_.y * _e14.g1_.y)) + (_e19.g0_.z * _e22.g1_.z)) - (_e27.g1_.w * _e30.g6_.w)));
}

fn flector_multi_vector_anti_scalar_product(self_1804: Flector, other_1496: MultiVector) -> AntiScalar {
    var self_1805: Flector;
    var other_1497: MultiVector;

    self_1805 = self_1804;
    other_1497 = other_1496;
    let _e5: Flector = self_1805;
    let _e8: MultiVector = other_1497;
    let _e13: Flector = self_1805;
    let _e16: MultiVector = other_1497;
    let _e21: Flector = self_1805;
    let _e24: MultiVector = other_1497;
    let _e29: Flector = self_1805;
    let _e32: MultiVector = other_1497;
    return AntiScalar(((((0.0 - (_e5.g0_.w * _e8.g1_.w)) + (_e13.g1_.x * _e16.g6_.x)) + (_e21.g1_.y * _e24.g6_.y)) + (_e29.g1_.z * _e32.g6_.z)));
}

fn flector_squared_magnitude(self_1806: Flector) -> Scalar {
    var self_1807: Flector;

    self_1807 = self_1806;
    let _e2: Flector = self_1807;
    let _e3: Flector = self_1807;
    let _e4: Flector = flector_reversal(_e3);
    let _e5: Scalar = flector_flector_scalar_product(_e2, _e4);
    return _e5;
}

fn flector_magnitude(self_1808: Flector) -> Scalar {
    var self_1809: Flector;

    self_1809 = self_1808;
    let _e2: Flector = self_1809;
    let _e3: Scalar = flector_squared_magnitude(_e2);
    return Scalar(sqrt(_e3.g0_));
}

fn flector_bulk_norm(self_1810: Flector) -> Scalar {
    var self_1811: Flector;

    self_1811 = self_1810;
    let _e2: Flector = self_1811;
    let _e3: Scalar = flector_squared_magnitude(_e2);
    return Scalar(sqrt(_e3.g0_));
}

fn flector_squared_anti_magnitude(self_1812: Flector) -> AntiScalar {
    var self_1813: Flector;

    self_1813 = self_1812;
    let _e2: Flector = self_1813;
    let _e3: Flector = self_1813;
    let _e4: Flector = flector_anti_reversal(_e3);
    let _e5: AntiScalar = flector_flector_anti_scalar_product(_e2, _e4);
    return _e5;
}

fn flector_weight_norm(self_1814: Flector) -> AntiScalar {
    var self_1815: Flector;

    self_1815 = self_1814;
    let _e2: Flector = self_1815;
    let _e3: AntiScalar = flector_squared_anti_magnitude(_e2);
    return AntiScalar(sqrt(_e3.g0_));
}

fn flector_geometric_norm(self_1816: Flector) -> HomogeneousMagnitude {
    var self_1817: Flector;

    self_1817 = self_1816;
    let _e2: Flector = self_1817;
    let _e3: Scalar = flector_bulk_norm(_e2);
    let _e4: Flector = self_1817;
    let _e5: AntiScalar = flector_weight_norm(_e4);
    return (_e3 + _e5);
}

fn flector_scale(self_1818: Flector, other_1498: f32) -> Flector {
    var self_1819: Flector;
    var other_1499: f32;

    self_1819 = self_1818;
    other_1499 = other_1498;
    let _e4: Flector = self_1819;
    let _e5: f32 = other_1499;
    let _e7: Flector = flector_scalar_geometric_product(_e4, Scalar(_e5));
    return _e7;
}

fn flector_signum(self_1820: Flector) -> Flector {
    var self_1821: Flector;

    self_1821 = self_1820;
    let _e2: Flector = self_1821;
    let _e3: Flector = self_1821;
    let _e4: Scalar = flector_magnitude(_e3);
    let _e9: Flector = flector_scalar_geometric_product(_e2, Scalar((1.0 / _e4.g0_)));
    return _e9;
}

fn flector_inverse(self_1822: Flector) -> Flector {
    var self_1823: Flector;

    self_1823 = self_1822;
    let _e2: Flector = self_1823;
    let _e3: Flector = flector_reversal(_e2);
    let _e4: Flector = self_1823;
    let _e5: Scalar = flector_squared_magnitude(_e4);
    let _e10: Flector = flector_scalar_geometric_product(_e3, Scalar((1.0 / _e5.g0_)));
    return _e10;
}

fn flector_unitize(self_1824: Flector) -> Flector {
    var self_1825: Flector;

    self_1825 = self_1824;
    let _e2: Flector = self_1825;
    let _e3: Flector = self_1825;
    let _e4: AntiScalar = flector_weight_norm(_e3);
    let _e9: Flector = flector_scalar_geometric_product(_e2, Scalar((1.0 / _e4.g0_)));
    return _e9;
}

fn multi_vector_zero() -> MultiVector {
    return MultiVector(vec3<f32>(0.0), vec4<f32>(0.0), vec3<f32>(0.0), vec3<f32>(0.0), vec3<f32>(0.0), vec3<f32>(0.0), vec4<f32>(0.0), vec4<f32>(0.0));
}

fn multi_vector_one() -> MultiVector {
    return MultiVector(vec3<f32>(1.0, 0.0, 0.0), vec4<f32>(0.0), vec3<f32>(0.0), vec3<f32>(0.0), vec3<f32>(0.0), vec3<f32>(0.0), vec4<f32>(0.0), vec4<f32>(0.0));
}

fn multi_vector_neg(self_1826: MultiVector) -> MultiVector {
    var self_1827: MultiVector;

    self_1827 = self_1826;
    let _e2: MultiVector = self_1827;
    let _e9: MultiVector = self_1827;
    let _e15: MultiVector = self_1827;
    let _e21: MultiVector = self_1827;
    let _e27: MultiVector = self_1827;
    let _e33: MultiVector = self_1827;
    let _e39: MultiVector = self_1827;
    let _e45: MultiVector = self_1827;
    return MultiVector((_e2.g0_.xyy * vec3<f32>(-(1.0))), (_e9.g1_ * vec4<f32>(-(1.0))), (_e15.g2_ * vec3<f32>(-(1.0))), (_e21.g3_ * vec3<f32>(-(1.0))), (_e27.g2_ * vec3<f32>(-(1.0))), (_e33.g3_ * vec3<f32>(-(1.0))), (_e39.g6_ * vec4<f32>(-(1.0))), (_e45.g6_ * vec4<f32>(-(1.0))));
}

fn multi_vector_automorphism(self_1828: MultiVector) -> MultiVector {
    var self_1829: MultiVector;

    self_1829 = self_1828;
    let _e2: MultiVector = self_1829;
    let _e5: MultiVector = self_1829;
    let _e11: MultiVector = self_1829;
    let _e13: MultiVector = self_1829;
    let _e15: MultiVector = self_1829;
    let _e17: MultiVector = self_1829;
    let _e19: MultiVector = self_1829;
    let _e25: MultiVector = self_1829;
    return MultiVector(_e2.g0_.xyy, (_e5.g1_ * vec4<f32>(-(1.0))), _e11.g2_, _e13.g3_, _e15.g2_, _e17.g3_, (_e19.g6_ * vec4<f32>(-(1.0))), (_e25.g6_ * vec4<f32>(-(1.0))));
}

fn multi_vector_reversal(self_1830: MultiVector) -> MultiVector {
    var self_1831: MultiVector;

    self_1831 = self_1830;
    let _e2: MultiVector = self_1831;
    let _e5: MultiVector = self_1831;
    let _e7: MultiVector = self_1831;
    let _e13: MultiVector = self_1831;
    let _e19: MultiVector = self_1831;
    let _e25: MultiVector = self_1831;
    let _e31: MultiVector = self_1831;
    let _e37: MultiVector = self_1831;
    return MultiVector(_e2.g0_.xyy, _e5.g1_, (_e7.g2_ * vec3<f32>(-(1.0))), (_e13.g3_ * vec3<f32>(-(1.0))), (_e19.g2_ * vec3<f32>(-(1.0))), (_e25.g3_ * vec3<f32>(-(1.0))), (_e31.g6_ * vec4<f32>(-(1.0))), (_e37.g6_ * vec4<f32>(-(1.0))));
}

fn multi_vector_conjugation(self_1832: MultiVector) -> MultiVector {
    var self_1833: MultiVector;

    self_1833 = self_1832;
    let _e2: MultiVector = self_1833;
    let _e5: MultiVector = self_1833;
    let _e11: MultiVector = self_1833;
    let _e17: MultiVector = self_1833;
    let _e23: MultiVector = self_1833;
    let _e29: MultiVector = self_1833;
    let _e35: MultiVector = self_1833;
    let _e37: MultiVector = self_1833;
    return MultiVector(_e2.g0_.xyy, (_e5.g1_ * vec4<f32>(-(1.0))), (_e11.g2_ * vec3<f32>(-(1.0))), (_e17.g3_ * vec3<f32>(-(1.0))), (_e23.g2_ * vec3<f32>(-(1.0))), (_e29.g3_ * vec3<f32>(-(1.0))), _e35.g6_, _e37.g6_);
}

fn multi_vector_anti_reversal(self_1834: MultiVector) -> MultiVector {
    var self_1835: MultiVector;

    self_1835 = self_1834;
    let _e2: MultiVector = self_1835;
    let _e5: MultiVector = self_1835;
    let _e11: MultiVector = self_1835;
    let _e17: MultiVector = self_1835;
    let _e23: MultiVector = self_1835;
    let _e29: MultiVector = self_1835;
    let _e35: MultiVector = self_1835;
    let _e37: MultiVector = self_1835;
    return MultiVector(_e2.g0_.xyy, (_e5.g1_ * vec4<f32>(-(1.0))), (_e11.g2_ * vec3<f32>(-(1.0))), (_e17.g3_ * vec3<f32>(-(1.0))), (_e23.g2_ * vec3<f32>(-(1.0))), (_e29.g3_ * vec3<f32>(-(1.0))), _e35.g6_, _e37.g6_);
}

fn multi_vector_double_complement(self_1836: MultiVector) -> MultiVector {
    var self_1837: MultiVector;

    self_1837 = self_1836;
    let _e2: MultiVector = self_1837;
    let _e5: MultiVector = self_1837;
    let _e11: MultiVector = self_1837;
    let _e13: MultiVector = self_1837;
    let _e15: MultiVector = self_1837;
    let _e17: MultiVector = self_1837;
    let _e19: MultiVector = self_1837;
    let _e25: MultiVector = self_1837;
    return MultiVector(_e2.g0_.xyy, (_e5.g1_ * vec4<f32>(-(1.0))), _e11.g2_, _e13.g3_, _e15.g2_, _e17.g3_, (_e19.g6_ * vec4<f32>(-(1.0))), (_e25.g6_ * vec4<f32>(-(1.0))));
}

fn multi_vector_scalar_into(self_1838: MultiVector) -> Scalar {
    var self_1839: MultiVector;

    self_1839 = self_1838;
    let _e2: MultiVector = self_1839;
    return Scalar(_e2.g0_.x);
}

fn multi_vector_scalar_regressive_product(self_1840: MultiVector, other_1500: Scalar) -> Scalar {
    var self_1841: MultiVector;
    var other_1501: Scalar;

    self_1841 = self_1840;
    other_1501 = other_1500;
    let _e4: MultiVector = self_1841;
    let _e7: Scalar = other_1501;
    return Scalar((_e4.g0_.y * _e7.g0_));
}

fn multi_vector_scalar_left_contraction(self_1842: MultiVector, other_1502: Scalar) -> Scalar {
    var self_1843: MultiVector;
    var other_1503: Scalar;

    self_1843 = self_1842;
    other_1503 = other_1502;
    let _e4: MultiVector = self_1843;
    let _e7: Scalar = other_1503;
    return Scalar((_e4.g0_.x * _e7.g0_));
}

fn multi_vector_scalar_scalar_product(self_1844: MultiVector, other_1504: Scalar) -> Scalar {
    var self_1845: MultiVector;
    var other_1505: Scalar;

    self_1845 = self_1844;
    other_1505 = other_1504;
    let _e4: MultiVector = self_1845;
    let _e7: Scalar = other_1505;
    return Scalar((_e4.g0_.x * _e7.g0_));
}

fn multi_vector_anti_scalar_outer_product(self_1846: MultiVector, other_1506: AntiScalar) -> AntiScalar {
    var self_1847: MultiVector;
    var other_1507: AntiScalar;

    self_1847 = self_1846;
    other_1507 = other_1506;
    let _e4: MultiVector = self_1847;
    let _e7: AntiScalar = other_1507;
    return AntiScalar((_e4.g0_.x * _e7.g0_));
}

fn multi_vector_anti_scalar_left_anti_contraction(self_1848: MultiVector, other_1508: AntiScalar) -> AntiScalar {
    var self_1849: MultiVector;
    var other_1509: AntiScalar;

    self_1849 = self_1848;
    other_1509 = other_1508;
    let _e4: MultiVector = self_1849;
    let _e7: AntiScalar = other_1509;
    return AntiScalar((_e4.g0_.y * _e7.g0_));
}

fn multi_vector_anti_scalar_anti_scalar_product(self_1850: MultiVector, other_1510: AntiScalar) -> AntiScalar {
    var self_1851: MultiVector;
    var other_1511: AntiScalar;

    self_1851 = self_1850;
    other_1511 = other_1510;
    let _e4: MultiVector = self_1851;
    let _e7: AntiScalar = other_1511;
    return AntiScalar((_e4.g0_.y * _e7.g0_));
}

fn multi_vector_homogeneous_magnitude_scalar_product(self_1852: MultiVector, other_1512: HomogeneousMagnitude) -> Scalar {
    var self_1853: MultiVector;
    var other_1513: HomogeneousMagnitude;

    self_1853 = self_1852;
    other_1513 = other_1512;
    let _e4: MultiVector = self_1853;
    let _e7: HomogeneousMagnitude = other_1513;
    return Scalar((_e4.g0_.x * _e7.g0_.x));
}

fn multi_vector_homogeneous_magnitude_anti_scalar_product(self_1854: MultiVector, other_1514: HomogeneousMagnitude) -> AntiScalar {
    var self_1855: MultiVector;
    var other_1515: HomogeneousMagnitude;

    self_1855 = self_1854;
    other_1515 = other_1514;
    let _e4: MultiVector = self_1855;
    let _e7: HomogeneousMagnitude = other_1515;
    return AntiScalar((_e4.g0_.y * _e7.g0_.y));
}

fn multi_vector_point_into(self_1856: MultiVector) -> Point {
    var self_1857: MultiVector;

    self_1857 = self_1856;
    let _e2: MultiVector = self_1857;
    return Point(_e2.g1_);
}

fn multi_vector_point_scalar_product(self_1858: MultiVector, other_1516: Point) -> Scalar {
    var self_1859: MultiVector;
    var other_1517: Point;

    self_1859 = self_1858;
    other_1517 = other_1516;
    let _e4: MultiVector = self_1859;
    let _e7: Point = other_1517;
    let _e11: MultiVector = self_1859;
    let _e14: Point = other_1517;
    let _e19: MultiVector = self_1859;
    let _e22: Point = other_1517;
    return Scalar((((_e4.g1_.x * _e7.g0_.x) + (_e11.g1_.y * _e14.g0_.y)) + (_e19.g1_.z * _e22.g0_.z)));
}

fn multi_vector_point_anti_scalar_product(self_1860: MultiVector, other_1518: Point) -> AntiScalar {
    var self_1861: MultiVector;
    var other_1519: Point;

    self_1861 = self_1860;
    other_1519 = other_1518;
    let _e5: MultiVector = self_1861;
    let _e8: Point = other_1519;
    return AntiScalar((0.0 - (_e5.g1_.w * _e8.g0_.w)));
}

fn multi_vector_line_scalar_product(self_1862: MultiVector, other_1520: Line) -> Scalar {
    var self_1863: MultiVector;
    var other_1521: Line;

    self_1863 = self_1862;
    other_1521 = other_1520;
    let _e5: MultiVector = self_1863;
    let _e8: Line = other_1521;
    let _e13: MultiVector = self_1863;
    let _e16: Line = other_1521;
    let _e21: MultiVector = self_1863;
    let _e24: Line = other_1521;
    return Scalar((((0.0 - (_e5.g3_.x * _e8.g1_.x)) - (_e13.g3_.y * _e16.g1_.y)) - (_e21.g3_.z * _e24.g1_.z)));
}

fn multi_vector_line_anti_scalar_product(self_1864: MultiVector, other_1522: Line) -> AntiScalar {
    var self_1865: MultiVector;
    var other_1523: Line;

    self_1865 = self_1864;
    other_1523 = other_1522;
    let _e5: MultiVector = self_1865;
    let _e8: Line = other_1523;
    let _e13: MultiVector = self_1865;
    let _e16: Line = other_1523;
    let _e21: MultiVector = self_1865;
    let _e24: Line = other_1523;
    return AntiScalar((((0.0 - (_e5.g2_.x * _e8.g0_.x)) - (_e13.g2_.y * _e16.g0_.y)) - (_e21.g2_.z * _e24.g0_.z)));
}

fn multi_vector_plane_scalar_product(self_1866: MultiVector, other_1524: Plane) -> Scalar {
    var self_1867: MultiVector;
    var other_1525: Plane;

    self_1867 = self_1866;
    other_1525 = other_1524;
    let _e5: MultiVector = self_1867;
    let _e8: Plane = other_1525;
    return Scalar((0.0 - (_e5.g6_.w * _e8.g0_.w)));
}

fn multi_vector_plane_anti_scalar_product(self_1868: MultiVector, other_1526: Plane) -> AntiScalar {
    var self_1869: MultiVector;
    var other_1527: Plane;

    self_1869 = self_1868;
    other_1527 = other_1526;
    let _e4: MultiVector = self_1869;
    let _e7: Plane = other_1527;
    let _e11: MultiVector = self_1869;
    let _e14: Plane = other_1527;
    let _e19: MultiVector = self_1869;
    let _e22: Plane = other_1527;
    return AntiScalar((((_e4.g6_.x * _e7.g0_.x) + (_e11.g6_.y * _e14.g0_.y)) + (_e19.g6_.z * _e22.g0_.z)));
}

fn multi_vector_motor_scalar_product(self_1870: MultiVector, other_1528: Motor) -> Scalar {
    var self_1871: MultiVector;
    var other_1529: Motor;

    self_1871 = self_1870;
    other_1529 = other_1528;
    let _e5: MultiVector = self_1871;
    let _e8: Motor = other_1529;
    let _e13: MultiVector = self_1871;
    let _e16: Motor = other_1529;
    let _e21: MultiVector = self_1871;
    let _e24: Motor = other_1529;
    return Scalar((((0.0 - (_e5.g3_.x * _e8.g1_.x)) - (_e13.g3_.y * _e16.g1_.y)) - (_e21.g3_.z * _e24.g1_.z)));
}

fn multi_vector_motor_anti_scalar_product(self_1872: MultiVector, other_1530: Motor) -> AntiScalar {
    var self_1873: MultiVector;
    var other_1531: Motor;

    self_1873 = self_1872;
    other_1531 = other_1530;
    let _e4: MultiVector = self_1873;
    let _e7: Motor = other_1531;
    let _e11: MultiVector = self_1873;
    let _e14: Motor = other_1531;
    let _e19: MultiVector = self_1873;
    let _e22: Motor = other_1531;
    let _e27: MultiVector = self_1873;
    let _e30: Motor = other_1531;
    return AntiScalar(((((_e4.g0_.y * _e7.g0_.w) - (_e11.g2_.x * _e14.g0_.x)) - (_e19.g2_.y * _e22.g0_.y)) - (_e27.g2_.z * _e30.g0_.z)));
}

fn multi_vector_rotor_anti_scalar_product(self_1874: MultiVector, other_1532: Rotor) -> AntiScalar {
    var self_1875: MultiVector;
    var other_1533: Rotor;

    self_1875 = self_1874;
    other_1533 = other_1532;
    let _e4: MultiVector = self_1875;
    let _e7: Rotor = other_1533;
    let _e11: MultiVector = self_1875;
    let _e14: Rotor = other_1533;
    let _e19: MultiVector = self_1875;
    let _e22: Rotor = other_1533;
    let _e27: MultiVector = self_1875;
    let _e30: Rotor = other_1533;
    return AntiScalar(((((_e4.g0_.y * _e7.g0_.w) - (_e11.g2_.x * _e14.g0_.x)) - (_e19.g2_.y * _e22.g0_.y)) - (_e27.g2_.z * _e30.g0_.z)));
}

fn multi_vector_translator_scalar_product(self_1876: MultiVector, other_1534: Translator) -> Scalar {
    var self_1877: MultiVector;
    var other_1535: Translator;

    self_1877 = self_1876;
    other_1535 = other_1534;
    let _e5: MultiVector = self_1877;
    let _e8: Translator = other_1535;
    let _e13: MultiVector = self_1877;
    let _e16: Translator = other_1535;
    let _e21: MultiVector = self_1877;
    let _e24: Translator = other_1535;
    return Scalar((((0.0 - (_e5.g3_.x * _e8.g0_.x)) - (_e13.g3_.y * _e16.g0_.y)) - (_e21.g3_.z * _e24.g0_.z)));
}

fn multi_vector_translator_anti_scalar_product(self_1878: MultiVector, other_1536: Translator) -> AntiScalar {
    var self_1879: MultiVector;
    var other_1537: Translator;

    self_1879 = self_1878;
    other_1537 = other_1536;
    let _e4: MultiVector = self_1879;
    let _e7: Translator = other_1537;
    return AntiScalar((_e4.g0_.y * _e7.g0_.w));
}

fn multi_vector_flector_scalar_product(self_1880: MultiVector, other_1538: Flector) -> Scalar {
    var self_1881: MultiVector;
    var other_1539: Flector;

    self_1881 = self_1880;
    other_1539 = other_1538;
    let _e4: MultiVector = self_1881;
    let _e7: Flector = other_1539;
    let _e11: MultiVector = self_1881;
    let _e14: Flector = other_1539;
    let _e19: MultiVector = self_1881;
    let _e22: Flector = other_1539;
    let _e27: MultiVector = self_1881;
    let _e30: Flector = other_1539;
    return Scalar(((((_e4.g1_.x * _e7.g0_.x) + (_e11.g1_.y * _e14.g0_.y)) + (_e19.g1_.z * _e22.g0_.z)) - (_e27.g6_.w * _e30.g1_.w)));
}

fn multi_vector_flector_anti_scalar_product(self_1882: MultiVector, other_1540: Flector) -> AntiScalar {
    var self_1883: MultiVector;
    var other_1541: Flector;

    self_1883 = self_1882;
    other_1541 = other_1540;
    let _e5: MultiVector = self_1883;
    let _e8: Flector = other_1541;
    let _e13: MultiVector = self_1883;
    let _e16: Flector = other_1541;
    let _e21: MultiVector = self_1883;
    let _e24: Flector = other_1541;
    let _e29: MultiVector = self_1883;
    let _e32: Flector = other_1541;
    return AntiScalar(((((0.0 - (_e5.g1_.w * _e8.g0_.w)) + (_e13.g6_.x * _e16.g1_.x)) + (_e21.g6_.y * _e24.g1_.y)) + (_e29.g6_.z * _e32.g1_.z)));
}

fn multi_vector_multi_vector_scalar_product(self_1884: MultiVector, other_1542: MultiVector) -> Scalar {
    var self_1885: MultiVector;
    var other_1543: MultiVector;

    self_1885 = self_1884;
    other_1543 = other_1542;
    let _e4: MultiVector = self_1885;
    let _e7: MultiVector = other_1543;
    let _e11: MultiVector = self_1885;
    let _e14: MultiVector = other_1543;
    let _e19: MultiVector = self_1885;
    let _e22: MultiVector = other_1543;
    let _e27: MultiVector = self_1885;
    let _e30: MultiVector = other_1543;
    let _e35: MultiVector = self_1885;
    let _e38: MultiVector = other_1543;
    let _e43: MultiVector = self_1885;
    let _e46: MultiVector = other_1543;
    let _e51: MultiVector = self_1885;
    let _e54: MultiVector = other_1543;
    let _e59: MultiVector = self_1885;
    let _e62: MultiVector = other_1543;
    return Scalar(((((((((_e4.g0_.x * _e7.g0_.x) + (_e11.g1_.x * _e14.g1_.x)) + (_e19.g1_.y * _e22.g1_.y)) + (_e27.g1_.z * _e30.g1_.z)) - (_e35.g3_.x * _e38.g3_.x)) - (_e43.g3_.y * _e46.g3_.y)) - (_e51.g3_.z * _e54.g3_.z)) - (_e59.g6_.w * _e62.g6_.w)));
}

fn multi_vector_multi_vector_anti_scalar_product(self_1886: MultiVector, other_1544: MultiVector) -> AntiScalar {
    var self_1887: MultiVector;
    var other_1545: MultiVector;

    self_1887 = self_1886;
    other_1545 = other_1544;
    let _e4: MultiVector = self_1887;
    let _e7: MultiVector = other_1545;
    let _e11: MultiVector = self_1887;
    let _e14: MultiVector = other_1545;
    let _e19: MultiVector = self_1887;
    let _e22: MultiVector = other_1545;
    let _e27: MultiVector = self_1887;
    let _e30: MultiVector = other_1545;
    let _e35: MultiVector = self_1887;
    let _e38: MultiVector = other_1545;
    let _e43: MultiVector = self_1887;
    let _e46: MultiVector = other_1545;
    let _e51: MultiVector = self_1887;
    let _e54: MultiVector = other_1545;
    let _e59: MultiVector = self_1887;
    let _e62: MultiVector = other_1545;
    return AntiScalar(((((((((_e4.g0_.y * _e7.g0_.y) - (_e11.g1_.w * _e14.g1_.w)) - (_e19.g2_.x * _e22.g2_.x)) - (_e27.g2_.y * _e30.g2_.y)) - (_e35.g2_.z * _e38.g2_.z)) + (_e43.g6_.x * _e46.g6_.x)) + (_e51.g6_.y * _e54.g6_.y)) + (_e59.g6_.z * _e62.g6_.z)));
}

fn multi_vector_squared_magnitude(self_1888: MultiVector) -> Scalar {
    var self_1889: MultiVector;

    self_1889 = self_1888;
    let _e2: MultiVector = self_1889;
    let _e3: MultiVector = self_1889;
    let _e4: MultiVector = multi_vector_reversal(_e3);
    let _e5: Scalar = multi_vector_multi_vector_scalar_product(_e2, _e4);
    return _e5;
}

fn multi_vector_magnitude(self_1890: MultiVector) -> Scalar {
    var self_1891: MultiVector;

    self_1891 = self_1890;
    let _e2: MultiVector = self_1891;
    let _e3: Scalar = multi_vector_squared_magnitude(_e2);
    return Scalar(sqrt(_e3.g0_));
}

fn multi_vector_bulk_norm(self_1892: MultiVector) -> Scalar {
    var self_1893: MultiVector;

    self_1893 = self_1892;
    let _e2: MultiVector = self_1893;
    let _e3: Scalar = multi_vector_squared_magnitude(_e2);
    return Scalar(sqrt(_e3.g0_));
}

fn multi_vector_squared_anti_magnitude(self_1894: MultiVector) -> AntiScalar {
    var self_1895: MultiVector;

    self_1895 = self_1894;
    let _e2: MultiVector = self_1895;
    let _e3: MultiVector = self_1895;
    let _e4: MultiVector = multi_vector_anti_reversal(_e3);
    let _e5: AntiScalar = multi_vector_multi_vector_anti_scalar_product(_e2, _e4);
    return _e5;
}

fn multi_vector_weight_norm(self_1896: MultiVector) -> AntiScalar {
    var self_1897: MultiVector;

    self_1897 = self_1896;
    let _e2: MultiVector = self_1897;
    let _e3: AntiScalar = multi_vector_squared_anti_magnitude(_e2);
    return AntiScalar(sqrt(_e3.g0_));
}

fn multi_vector_geometric_norm(self_1898: MultiVector) -> HomogeneousMagnitude {
    var self_1899: MultiVector;

    self_1899 = self_1898;
    let _e2: MultiVector = self_1899;
    let _e3: Scalar = multi_vector_bulk_norm(_e2);
    let _e4: MultiVector = self_1899;
    let _e5: AntiScalar = multi_vector_weight_norm(_e4);
    return (_e3 + _e5);
}

fn anti_scalar_homogeneous_magnitude_geometric_quotient(self_1900: AntiScalar, other_1546: HomogeneousMagnitude) -> AntiScalar {
    var self_1901: AntiScalar;
    var other_1547: HomogeneousMagnitude;

    self_1901 = self_1900;
    other_1547 = other_1546;
    let _e4: AntiScalar = self_1901;
    let _e5: HomogeneousMagnitude = other_1547;
    let _e6: HomogeneousMagnitude = homogeneous_magnitude_inverse(_e5);
    let _e7: AntiScalar = anti_scalar_homogeneous_magnitude_geometric_product(_e4, _e6);
    return _e7;
}

fn anti_scalar_scalar_geometric_quotient(self_1902: AntiScalar, other_1548: Scalar) -> AntiScalar {
    var self_1903: AntiScalar;
    var other_1549: Scalar;

    self_1903 = self_1902;
    other_1549 = other_1548;
    let _e4: AntiScalar = self_1903;
    let _e5: Scalar = other_1549;
    let _e6: Scalar = scalar_inverse(_e5);
    let _e7: AntiScalar = anti_scalar_scalar_geometric_product(_e4, _e6);
    return _e7;
}

fn flector_homogeneous_magnitude_geometric_quotient(self_1904: Flector, other_1550: HomogeneousMagnitude) -> Flector {
    var self_1905: Flector;
    var other_1551: HomogeneousMagnitude;

    self_1905 = self_1904;
    other_1551 = other_1550;
    let _e4: Flector = self_1905;
    let _e5: HomogeneousMagnitude = other_1551;
    let _e6: HomogeneousMagnitude = homogeneous_magnitude_inverse(_e5);
    let _e7: Flector = flector_homogeneous_magnitude_geometric_product(_e4, _e6);
    return _e7;
}

fn flector_line_geometric_quotient(self_1906: Flector, other_1552: Line) -> Flector {
    var self_1907: Flector;
    var other_1553: Line;

    self_1907 = self_1906;
    other_1553 = other_1552;
    let _e4: Flector = self_1907;
    let _e5: Line = other_1553;
    let _e6: Line = line_inverse(_e5);
    let _e7: Flector = flector_line_geometric_product(_e4, _e6);
    return _e7;
}

fn flector_motor_geometric_quotient(self_1908: Flector, other_1554: Motor) -> Flector {
    var self_1909: Flector;
    var other_1555: Motor;

    self_1909 = self_1908;
    other_1555 = other_1554;
    let _e4: Flector = self_1909;
    let _e5: Motor = other_1555;
    let _e6: Motor = motor_inverse(_e5);
    let _e7: Flector = flector_motor_geometric_product(_e4, _e6);
    return _e7;
}

fn flector_scalar_geometric_quotient(self_1910: Flector, other_1556: Scalar) -> Flector {
    var self_1911: Flector;
    var other_1557: Scalar;

    self_1911 = self_1910;
    other_1557 = other_1556;
    let _e4: Flector = self_1911;
    let _e5: Scalar = other_1557;
    let _e6: Scalar = scalar_inverse(_e5);
    let _e7: Flector = flector_scalar_geometric_product(_e4, _e6);
    return _e7;
}

fn flector_translator_geometric_quotient(self_1912: Flector, other_1558: Translator) -> Flector {
    var self_1913: Flector;
    var other_1559: Translator;

    self_1913 = self_1912;
    other_1559 = other_1558;
    let _e4: Flector = self_1913;
    let _e5: Translator = other_1559;
    let _e6: Translator = translator_inverse(_e5);
    let _e7: Flector = flector_translator_geometric_product(_e4, _e6);
    return _e7;
}

fn homogeneous_magnitude_anti_scalar_transformation(self_1914: HomogeneousMagnitude, other_1560: AntiScalar) -> AntiScalar {
    var self_1915: HomogeneousMagnitude;
    var other_1561: AntiScalar;

    self_1915 = self_1914;
    other_1561 = other_1560;
    let _e4: HomogeneousMagnitude = self_1915;
    let _e5: AntiScalar = other_1561;
    let _e6: AntiScalar = homogeneous_magnitude_anti_scalar_geometric_product(_e4, _e5);
    let _e7: HomogeneousMagnitude = self_1915;
    let _e8: HomogeneousMagnitude = homogeneous_magnitude_reversal(_e7);
    let _e9: AntiScalar = anti_scalar_homogeneous_magnitude_geometric_product(_e6, _e8);
    return _e9;
}

fn homogeneous_magnitude_flector_geometric_quotient(self_1916: HomogeneousMagnitude, other_1562: Flector) -> Flector {
    var self_1917: HomogeneousMagnitude;
    var other_1563: Flector;

    self_1917 = self_1916;
    other_1563 = other_1562;
    let _e4: HomogeneousMagnitude = self_1917;
    let _e5: Flector = other_1563;
    let _e6: Flector = flector_inverse(_e5);
    let _e7: Flector = homogeneous_magnitude_flector_geometric_product(_e4, _e6);
    return _e7;
}

fn homogeneous_magnitude_flector_transformation(self_1918: HomogeneousMagnitude, other_1564: Flector) -> Flector {
    var self_1919: HomogeneousMagnitude;
    var other_1565: Flector;

    self_1919 = self_1918;
    other_1565 = other_1564;
    let _e4: HomogeneousMagnitude = self_1919;
    let _e5: Flector = other_1565;
    let _e6: Flector = homogeneous_magnitude_flector_geometric_product(_e4, _e5);
    let _e7: HomogeneousMagnitude = self_1919;
    let _e8: HomogeneousMagnitude = homogeneous_magnitude_reversal(_e7);
    let _e9: Flector = flector_homogeneous_magnitude_geometric_product(_e6, _e8);
    return _e9;
}

fn homogeneous_magnitude_powi(self_1920: HomogeneousMagnitude, exponent: i32) -> HomogeneousMagnitude {
    var self_1921: HomogeneousMagnitude;
    var exponent_1: i32;
    var local: HomogeneousMagnitude;
    var x: HomogeneousMagnitude;
    var y: HomogeneousMagnitude;
    var n: i32;

    self_1921 = self_1920;
    exponent_1 = exponent;
    let _e4: i32 = exponent_1;
    if (_e4 == 0) {
        {
            let _e7: HomogeneousMagnitude = homogeneous_magnitude_one();
            return _e7;
        }
    }
    let _e8: i32 = exponent_1;
    if (_e8 < 0) {
        let _e11: HomogeneousMagnitude = self_1921;
        let _e12: HomogeneousMagnitude = homogeneous_magnitude_inverse(_e11);
        local = _e12;
    } else {
        let _e14: HomogeneousMagnitude = self_1921;
        local = _e14;
    }
    let _e15: HomogeneousMagnitude = local;
    x = _e15;
    let _e17: HomogeneousMagnitude = homogeneous_magnitude_one();
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
                    let _e31: HomogeneousMagnitude = x;
                    let _e32: HomogeneousMagnitude = y;
                    let _e33: HomogeneousMagnitude = homogeneous_magnitude_homogeneous_magnitude_geometric_product(_e31, _e32);
                    y = _e33;
                }
            }
            let _e34: HomogeneousMagnitude = x;
            let _e35: HomogeneousMagnitude = x;
            let _e36: HomogeneousMagnitude = homogeneous_magnitude_homogeneous_magnitude_geometric_product(_e34, _e35);
            x = _e36;
            let _e37: i32 = n;
            n = (_e37 >> u32(1));
        }
    }
    let _e41: HomogeneousMagnitude = x;
    let _e42: HomogeneousMagnitude = y;
    let _e43: HomogeneousMagnitude = homogeneous_magnitude_homogeneous_magnitude_geometric_product(_e41, _e42);
    return _e43;
}

fn homogeneous_magnitude_homogeneous_magnitude_geometric_quotient(self_1922: HomogeneousMagnitude, other_1566: HomogeneousMagnitude) -> HomogeneousMagnitude {
    var self_1923: HomogeneousMagnitude;
    var other_1567: HomogeneousMagnitude;

    self_1923 = self_1922;
    other_1567 = other_1566;
    let _e4: HomogeneousMagnitude = self_1923;
    let _e5: HomogeneousMagnitude = other_1567;
    let _e6: HomogeneousMagnitude = homogeneous_magnitude_inverse(_e5);
    let _e7: HomogeneousMagnitude = homogeneous_magnitude_homogeneous_magnitude_geometric_product(_e4, _e6);
    return _e7;
}

fn homogeneous_magnitude_homogeneous_magnitude_transformation(self_1924: HomogeneousMagnitude, other_1568: HomogeneousMagnitude) -> HomogeneousMagnitude {
    var self_1925: HomogeneousMagnitude;
    var other_1569: HomogeneousMagnitude;

    self_1925 = self_1924;
    other_1569 = other_1568;
    let _e4: HomogeneousMagnitude = self_1925;
    let _e5: HomogeneousMagnitude = other_1569;
    let _e6: HomogeneousMagnitude = homogeneous_magnitude_homogeneous_magnitude_geometric_product(_e4, _e5);
    let _e7: HomogeneousMagnitude = self_1925;
    let _e8: HomogeneousMagnitude = homogeneous_magnitude_reversal(_e7);
    let _e9: HomogeneousMagnitude = homogeneous_magnitude_homogeneous_magnitude_geometric_product(_e6, _e8);
    return _e9;
}

fn homogeneous_magnitude_line_geometric_quotient(self_1926: HomogeneousMagnitude, other_1570: Line) -> Line {
    var self_1927: HomogeneousMagnitude;
    var other_1571: Line;

    self_1927 = self_1926;
    other_1571 = other_1570;
    let _e4: HomogeneousMagnitude = self_1927;
    let _e5: Line = other_1571;
    let _e6: Line = line_inverse(_e5);
    let _e7: Line = homogeneous_magnitude_line_geometric_product(_e4, _e6);
    return _e7;
}

fn homogeneous_magnitude_line_transformation(self_1928: HomogeneousMagnitude, other_1572: Line) -> Line {
    var self_1929: HomogeneousMagnitude;
    var other_1573: Line;

    self_1929 = self_1928;
    other_1573 = other_1572;
    let _e4: HomogeneousMagnitude = self_1929;
    let _e5: Line = other_1573;
    let _e6: Line = homogeneous_magnitude_line_geometric_product(_e4, _e5);
    let _e7: HomogeneousMagnitude = self_1929;
    let _e8: HomogeneousMagnitude = homogeneous_magnitude_reversal(_e7);
    let _e9: Line = line_homogeneous_magnitude_geometric_product(_e6, _e8);
    return _e9;
}

fn homogeneous_magnitude_motor_geometric_quotient(self_1930: HomogeneousMagnitude, other_1574: Motor) -> Motor {
    var self_1931: HomogeneousMagnitude;
    var other_1575: Motor;

    self_1931 = self_1930;
    other_1575 = other_1574;
    let _e4: HomogeneousMagnitude = self_1931;
    let _e5: Motor = other_1575;
    let _e6: Motor = motor_inverse(_e5);
    let _e7: Motor = homogeneous_magnitude_motor_geometric_product(_e4, _e6);
    return _e7;
}

fn homogeneous_magnitude_motor_transformation(self_1932: HomogeneousMagnitude, other_1576: Motor) -> Motor {
    var self_1933: HomogeneousMagnitude;
    var other_1577: Motor;

    self_1933 = self_1932;
    other_1577 = other_1576;
    let _e4: HomogeneousMagnitude = self_1933;
    let _e5: Motor = other_1577;
    let _e6: Motor = homogeneous_magnitude_motor_geometric_product(_e4, _e5);
    let _e7: HomogeneousMagnitude = self_1933;
    let _e8: HomogeneousMagnitude = homogeneous_magnitude_reversal(_e7);
    let _e9: Motor = motor_homogeneous_magnitude_geometric_product(_e6, _e8);
    return _e9;
}

fn homogeneous_magnitude_rotor_transformation(self_1934: HomogeneousMagnitude, other_1578: Rotor) -> Rotor {
    var self_1935: HomogeneousMagnitude;
    var other_1579: Rotor;

    self_1935 = self_1934;
    other_1579 = other_1578;
    let _e4: HomogeneousMagnitude = self_1935;
    let _e5: Rotor = other_1579;
    let _e6: Rotor = homogeneous_magnitude_rotor_geometric_product(_e4, _e5);
    let _e7: HomogeneousMagnitude = self_1935;
    let _e8: HomogeneousMagnitude = homogeneous_magnitude_reversal(_e7);
    let _e9: Rotor = rotor_homogeneous_magnitude_geometric_product(_e6, _e8);
    return _e9;
}

fn homogeneous_magnitude_scalar_geometric_quotient(self_1936: HomogeneousMagnitude, other_1580: Scalar) -> HomogeneousMagnitude {
    var self_1937: HomogeneousMagnitude;
    var other_1581: Scalar;

    self_1937 = self_1936;
    other_1581 = other_1580;
    let _e4: HomogeneousMagnitude = self_1937;
    let _e5: Scalar = other_1581;
    let _e6: Scalar = scalar_inverse(_e5);
    let _e7: HomogeneousMagnitude = homogeneous_magnitude_scalar_geometric_product(_e4, _e6);
    return _e7;
}

fn homogeneous_magnitude_scalar_transformation(self_1938: HomogeneousMagnitude, other_1582: Scalar) -> Scalar {
    var self_1939: HomogeneousMagnitude;
    var other_1583: Scalar;

    self_1939 = self_1938;
    other_1583 = other_1582;
    let _e4: HomogeneousMagnitude = self_1939;
    let _e5: Scalar = other_1583;
    let _e6: HomogeneousMagnitude = homogeneous_magnitude_scalar_geometric_product(_e4, _e5);
    let _e7: HomogeneousMagnitude = self_1939;
    let _e8: HomogeneousMagnitude = homogeneous_magnitude_reversal(_e7);
    let _e9: HomogeneousMagnitude = homogeneous_magnitude_homogeneous_magnitude_geometric_product(_e6, _e8);
    let _e10: Scalar = homogeneous_magnitude_scalar_into(_e9);
    return _e10;
}

fn homogeneous_magnitude_translator_geometric_quotient(self_1940: HomogeneousMagnitude, other_1584: Translator) -> Motor {
    var self_1941: HomogeneousMagnitude;
    var other_1585: Translator;

    self_1941 = self_1940;
    other_1585 = other_1584;
    let _e4: HomogeneousMagnitude = self_1941;
    let _e5: Translator = other_1585;
    let _e6: Translator = translator_inverse(_e5);
    let _e7: Motor = homogeneous_magnitude_translator_geometric_product(_e4, _e6);
    return _e7;
}

fn homogeneous_magnitude_translator_transformation(self_1942: HomogeneousMagnitude, other_1586: Translator) -> Translator {
    var self_1943: HomogeneousMagnitude;
    var other_1587: Translator;

    self_1943 = self_1942;
    other_1587 = other_1586;
    let _e4: HomogeneousMagnitude = self_1943;
    let _e5: Translator = other_1587;
    let _e6: Motor = homogeneous_magnitude_translator_geometric_product(_e4, _e5);
    let _e7: HomogeneousMagnitude = self_1943;
    let _e8: HomogeneousMagnitude = homogeneous_magnitude_reversal(_e7);
    let _e9: Motor = motor_homogeneous_magnitude_geometric_product(_e6, _e8);
    let _e10: Translator = motor_translator_into(_e9);
    return _e10;
}

fn line_flector_geometric_quotient(self_1944: Line, other_1588: Flector) -> Flector {
    var self_1945: Line;
    var other_1589: Flector;

    self_1945 = self_1944;
    other_1589 = other_1588;
    let _e4: Line = self_1945;
    let _e5: Flector = other_1589;
    let _e6: Flector = flector_inverse(_e5);
    let _e7: Flector = line_flector_geometric_product(_e4, _e6);
    return _e7;
}

fn line_flector_transformation(self_1946: Line, other_1590: Flector) -> Flector {
    var self_1947: Line;
    var other_1591: Flector;

    self_1947 = self_1946;
    other_1591 = other_1590;
    let _e4: Line = self_1947;
    let _e5: Flector = other_1591;
    let _e6: Flector = line_flector_geometric_product(_e4, _e5);
    let _e7: Line = self_1947;
    let _e8: Line = line_reversal(_e7);
    let _e9: Flector = flector_line_geometric_product(_e6, _e8);
    return _e9;
}

fn line_homogeneous_magnitude_geometric_quotient(self_1948: Line, other_1592: HomogeneousMagnitude) -> Line {
    var self_1949: Line;
    var other_1593: HomogeneousMagnitude;

    self_1949 = self_1948;
    other_1593 = other_1592;
    let _e4: Line = self_1949;
    let _e5: HomogeneousMagnitude = other_1593;
    let _e6: HomogeneousMagnitude = homogeneous_magnitude_inverse(_e5);
    let _e7: Line = line_homogeneous_magnitude_geometric_product(_e4, _e6);
    return _e7;
}

fn line_point_geometric_quotient(self_1950: Line, other_1594: Point) -> Flector {
    var self_1951: Line;
    var other_1595: Point;

    self_1951 = self_1950;
    other_1595 = other_1594;
    let _e4: Line = self_1951;
    let _e5: Point = other_1595;
    let _e6: Point = point_inverse(_e5);
    let _e7: Flector = line_point_geometric_product(_e4, _e6);
    return _e7;
}

fn line_point_transformation(self_1952: Line, other_1596: Point) -> Point {
    var self_1953: Line;
    var other_1597: Point;

    self_1953 = self_1952;
    other_1597 = other_1596;
    let _e4: Line = self_1953;
    let _e5: Point = other_1597;
    let _e6: Flector = line_point_geometric_product(_e4, _e5);
    let _e7: Line = self_1953;
    let _e8: Line = line_reversal(_e7);
    let _e9: Flector = flector_line_geometric_product(_e6, _e8);
    let _e10: Point = flector_point_into(_e9);
    return _e10;
}

fn line_rotor_transformation(self_1954: Line, other_1598: Rotor) -> Rotor {
    var self_1955: Line;
    var other_1599: Rotor;

    self_1955 = self_1954;
    other_1599 = other_1598;
    let _e4: Line = self_1955;
    let _e5: Rotor = other_1599;
    let _e6: Rotor = line_rotor_geometric_product(_e4, _e5);
    let _e7: Line = self_1955;
    let _e8: Line = line_reversal(_e7);
    let _e9: Rotor = rotor_line_geometric_product(_e6, _e8);
    return _e9;
}

fn line_scalar_geometric_quotient(self_1956: Line, other_1600: Scalar) -> Line {
    var self_1957: Line;
    var other_1601: Scalar;

    self_1957 = self_1956;
    other_1601 = other_1600;
    let _e4: Line = self_1957;
    let _e5: Scalar = other_1601;
    let _e6: Scalar = scalar_inverse(_e5);
    let _e7: Line = line_scalar_geometric_product(_e4, _e6);
    return _e7;
}

fn motor_flector_geometric_quotient(self_1958: Motor, other_1602: Flector) -> Flector {
    var self_1959: Motor;
    var other_1603: Flector;

    self_1959 = self_1958;
    other_1603 = other_1602;
    let _e4: Motor = self_1959;
    let _e5: Flector = other_1603;
    let _e6: Flector = flector_inverse(_e5);
    let _e7: Flector = motor_flector_geometric_product(_e4, _e6);
    return _e7;
}

fn motor_flector_transformation(self_1960: Motor, other_1604: Flector) -> Flector {
    var self_1961: Motor;
    var other_1605: Flector;

    self_1961 = self_1960;
    other_1605 = other_1604;
    let _e4: Motor = self_1961;
    let _e5: Flector = other_1605;
    let _e6: Flector = motor_flector_geometric_product(_e4, _e5);
    let _e7: Motor = self_1961;
    let _e8: Motor = motor_reversal(_e7);
    let _e9: Flector = flector_motor_geometric_product(_e6, _e8);
    return _e9;
}

fn motor_homogeneous_magnitude_geometric_quotient(self_1962: Motor, other_1606: HomogeneousMagnitude) -> Motor {
    var self_1963: Motor;
    var other_1607: HomogeneousMagnitude;

    self_1963 = self_1962;
    other_1607 = other_1606;
    let _e4: Motor = self_1963;
    let _e5: HomogeneousMagnitude = other_1607;
    let _e6: HomogeneousMagnitude = homogeneous_magnitude_inverse(_e5);
    let _e7: Motor = motor_homogeneous_magnitude_geometric_product(_e4, _e6);
    return _e7;
}

fn motor_point_geometric_quotient(self_1964: Motor, other_1608: Point) -> Flector {
    var self_1965: Motor;
    var other_1609: Point;

    self_1965 = self_1964;
    other_1609 = other_1608;
    let _e4: Motor = self_1965;
    let _e5: Point = other_1609;
    let _e6: Point = point_inverse(_e5);
    let _e7: Flector = motor_point_geometric_product(_e4, _e6);
    return _e7;
}

fn motor_point_transformation(self_1966: Motor, other_1610: Point) -> Point {
    var self_1967: Motor;
    var other_1611: Point;

    self_1967 = self_1966;
    other_1611 = other_1610;
    let _e4: Motor = self_1967;
    let _e5: Point = other_1611;
    let _e6: Flector = motor_point_geometric_product(_e4, _e5);
    let _e7: Motor = self_1967;
    let _e8: Motor = motor_reversal(_e7);
    let _e9: Flector = flector_motor_geometric_product(_e6, _e8);
    let _e10: Point = flector_point_into(_e9);
    return _e10;
}

fn motor_rotor_transformation(self_1968: Motor, other_1612: Rotor) -> Rotor {
    var self_1969: Motor;
    var other_1613: Rotor;

    self_1969 = self_1968;
    other_1613 = other_1612;
    let _e4: Motor = self_1969;
    let _e5: Rotor = other_1613;
    let _e6: Rotor = motor_rotor_geometric_product(_e4, _e5);
    let _e7: Motor = self_1969;
    let _e8: Motor = motor_reversal(_e7);
    let _e9: Rotor = rotor_motor_geometric_product(_e6, _e8);
    return _e9;
}

fn motor_scalar_geometric_quotient(self_1970: Motor, other_1614: Scalar) -> Motor {
    var self_1971: Motor;
    var other_1615: Scalar;

    self_1971 = self_1970;
    other_1615 = other_1614;
    let _e4: Motor = self_1971;
    let _e5: Scalar = other_1615;
    let _e6: Scalar = scalar_inverse(_e5);
    let _e7: Motor = motor_scalar_geometric_product(_e4, _e6);
    return _e7;
}

fn plane_point_geometric_quotient(self_1972: Plane, other_1616: Point) -> Motor {
    var self_1973: Plane;
    var other_1617: Point;

    self_1973 = self_1972;
    other_1617 = other_1616;
    let _e4: Plane = self_1973;
    let _e5: Point = other_1617;
    let _e6: Point = point_inverse(_e5);
    let _e7: Motor = plane_point_geometric_product(_e4, _e6);
    return _e7;
}

fn plane_scalar_geometric_quotient(self_1974: Plane, other_1618: Scalar) -> Plane {
    var self_1975: Plane;
    var other_1619: Scalar;

    self_1975 = self_1974;
    other_1619 = other_1618;
    let _e4: Plane = self_1975;
    let _e5: Scalar = other_1619;
    let _e6: Scalar = scalar_inverse(_e5);
    let _e7: Plane = plane_scalar_geometric_product(_e4, _e6);
    return _e7;
}

fn point_line_geometric_quotient(self_1976: Point, other_1620: Line) -> Flector {
    var self_1977: Point;
    var other_1621: Line;

    self_1977 = self_1976;
    other_1621 = other_1620;
    let _e4: Point = self_1977;
    let _e5: Line = other_1621;
    let _e6: Line = line_inverse(_e5);
    let _e7: Flector = point_line_geometric_product(_e4, _e6);
    return _e7;
}

fn point_motor_geometric_quotient(self_1978: Point, other_1622: Motor) -> Flector {
    var self_1979: Point;
    var other_1623: Motor;

    self_1979 = self_1978;
    other_1623 = other_1622;
    let _e4: Point = self_1979;
    let _e5: Motor = other_1623;
    let _e6: Motor = motor_inverse(_e5);
    let _e7: Flector = point_motor_geometric_product(_e4, _e6);
    return _e7;
}

fn point_plane_geometric_quotient(self_1980: Point, other_1624: Plane) -> Motor {
    var self_1981: Point;
    var other_1625: Plane;

    self_1981 = self_1980;
    other_1625 = other_1624;
    let _e4: Point = self_1981;
    let _e5: Plane = other_1625;
    let _e6: Plane = plane_inverse(_e5);
    let _e7: Motor = point_plane_geometric_product(_e4, _e6);
    return _e7;
}

fn point_plane_transformation(self_1982: Point, other_1626: Plane) -> Plane {
    var self_1983: Point;
    var other_1627: Plane;

    self_1983 = self_1982;
    other_1627 = other_1626;
    let _e4: Point = self_1983;
    let _e5: Plane = other_1627;
    let _e6: Motor = point_plane_geometric_product(_e4, _e5);
    let _e7: Point = self_1983;
    let _e8: Point = point_reversal(_e7);
    let _e9: Flector = motor_point_geometric_product(_e6, _e8);
    let _e10: Plane = flector_plane_into(_e9);
    return _e10;
}

fn point_scalar_geometric_quotient(self_1984: Point, other_1628: Scalar) -> Point {
    var self_1985: Point;
    var other_1629: Scalar;

    self_1985 = self_1984;
    other_1629 = other_1628;
    let _e4: Point = self_1985;
    let _e5: Scalar = other_1629;
    let _e6: Scalar = scalar_inverse(_e5);
    let _e7: Point = point_scalar_geometric_product(_e4, _e6);
    return _e7;
}

fn rotor_homogeneous_magnitude_geometric_quotient(self_1986: Rotor, other_1630: HomogeneousMagnitude) -> Rotor {
    var self_1987: Rotor;
    var other_1631: HomogeneousMagnitude;

    self_1987 = self_1986;
    other_1631 = other_1630;
    let _e4: Rotor = self_1987;
    let _e5: HomogeneousMagnitude = other_1631;
    let _e6: HomogeneousMagnitude = homogeneous_magnitude_inverse(_e5);
    let _e7: Rotor = rotor_homogeneous_magnitude_geometric_product(_e4, _e6);
    return _e7;
}

fn rotor_line_geometric_quotient(self_1988: Rotor, other_1632: Line) -> Rotor {
    var self_1989: Rotor;
    var other_1633: Line;

    self_1989 = self_1988;
    other_1633 = other_1632;
    let _e4: Rotor = self_1989;
    let _e5: Line = other_1633;
    let _e6: Line = line_inverse(_e5);
    let _e7: Rotor = rotor_line_geometric_product(_e4, _e6);
    return _e7;
}

fn rotor_motor_geometric_quotient(self_1990: Rotor, other_1634: Motor) -> Rotor {
    var self_1991: Rotor;
    var other_1635: Motor;

    self_1991 = self_1990;
    other_1635 = other_1634;
    let _e4: Rotor = self_1991;
    let _e5: Motor = other_1635;
    let _e6: Motor = motor_inverse(_e5);
    let _e7: Rotor = rotor_motor_geometric_product(_e4, _e6);
    return _e7;
}

fn rotor_scalar_geometric_quotient(self_1992: Rotor, other_1636: Scalar) -> Rotor {
    var self_1993: Rotor;
    var other_1637: Scalar;

    self_1993 = self_1992;
    other_1637 = other_1636;
    let _e4: Rotor = self_1993;
    let _e5: Scalar = other_1637;
    let _e6: Scalar = scalar_inverse(_e5);
    let _e7: Rotor = rotor_scalar_geometric_product(_e4, _e6);
    return _e7;
}

fn rotor_translator_geometric_quotient(self_1994: Rotor, other_1638: Translator) -> Rotor {
    var self_1995: Rotor;
    var other_1639: Translator;

    self_1995 = self_1994;
    other_1639 = other_1638;
    let _e4: Rotor = self_1995;
    let _e5: Translator = other_1639;
    let _e6: Translator = translator_inverse(_e5);
    let _e7: Rotor = rotor_translator_geometric_product(_e4, _e6);
    return _e7;
}

fn scalar_anti_scalar_transformation(self_1996: Scalar, other_1640: AntiScalar) -> AntiScalar {
    var self_1997: Scalar;
    var other_1641: AntiScalar;

    self_1997 = self_1996;
    other_1641 = other_1640;
    let _e4: Scalar = self_1997;
    let _e5: AntiScalar = other_1641;
    let _e6: AntiScalar = scalar_anti_scalar_geometric_product(_e4, _e5);
    let _e7: Scalar = self_1997;
    let _e8: Scalar = scalar_reversal(_e7);
    let _e9: AntiScalar = anti_scalar_scalar_geometric_product(_e6, _e8);
    return _e9;
}

fn scalar_flector_geometric_quotient(self_1998: Scalar, other_1642: Flector) -> Flector {
    var self_1999: Scalar;
    var other_1643: Flector;

    self_1999 = self_1998;
    other_1643 = other_1642;
    let _e4: Scalar = self_1999;
    let _e5: Flector = other_1643;
    let _e6: Flector = flector_inverse(_e5);
    let _e7: Flector = scalar_flector_geometric_product(_e4, _e6);
    return _e7;
}

fn scalar_flector_transformation(self_2000: Scalar, other_1644: Flector) -> Flector {
    var self_2001: Scalar;
    var other_1645: Flector;

    self_2001 = self_2000;
    other_1645 = other_1644;
    let _e4: Scalar = self_2001;
    let _e5: Flector = other_1645;
    let _e6: Flector = scalar_flector_geometric_product(_e4, _e5);
    let _e7: Scalar = self_2001;
    let _e8: Scalar = scalar_reversal(_e7);
    let _e9: Flector = flector_scalar_geometric_product(_e6, _e8);
    return _e9;
}

fn scalar_homogeneous_magnitude_geometric_quotient(self_2002: Scalar, other_1646: HomogeneousMagnitude) -> HomogeneousMagnitude {
    var self_2003: Scalar;
    var other_1647: HomogeneousMagnitude;

    self_2003 = self_2002;
    other_1647 = other_1646;
    let _e4: Scalar = self_2003;
    let _e5: HomogeneousMagnitude = other_1647;
    let _e6: HomogeneousMagnitude = homogeneous_magnitude_inverse(_e5);
    let _e7: HomogeneousMagnitude = scalar_homogeneous_magnitude_geometric_product(_e4, _e6);
    return _e7;
}

fn scalar_homogeneous_magnitude_transformation(self_2004: Scalar, other_1648: HomogeneousMagnitude) -> HomogeneousMagnitude {
    var self_2005: Scalar;
    var other_1649: HomogeneousMagnitude;

    self_2005 = self_2004;
    other_1649 = other_1648;
    let _e4: Scalar = self_2005;
    let _e5: HomogeneousMagnitude = other_1649;
    let _e6: HomogeneousMagnitude = scalar_homogeneous_magnitude_geometric_product(_e4, _e5);
    let _e7: Scalar = self_2005;
    let _e8: Scalar = scalar_reversal(_e7);
    let _e9: HomogeneousMagnitude = homogeneous_magnitude_scalar_geometric_product(_e6, _e8);
    return _e9;
}

fn scalar_line_geometric_quotient(self_2006: Scalar, other_1650: Line) -> Line {
    var self_2007: Scalar;
    var other_1651: Line;

    self_2007 = self_2006;
    other_1651 = other_1650;
    let _e4: Scalar = self_2007;
    let _e5: Line = other_1651;
    let _e6: Line = line_inverse(_e5);
    let _e7: Line = scalar_line_geometric_product(_e4, _e6);
    return _e7;
}

fn scalar_line_transformation(self_2008: Scalar, other_1652: Line) -> Line {
    var self_2009: Scalar;
    var other_1653: Line;

    self_2009 = self_2008;
    other_1653 = other_1652;
    let _e4: Scalar = self_2009;
    let _e5: Line = other_1653;
    let _e6: Line = scalar_line_geometric_product(_e4, _e5);
    let _e7: Scalar = self_2009;
    let _e8: Scalar = scalar_reversal(_e7);
    let _e9: Line = line_scalar_geometric_product(_e6, _e8);
    return _e9;
}

fn scalar_motor_geometric_quotient(self_2010: Scalar, other_1654: Motor) -> Motor {
    var self_2011: Scalar;
    var other_1655: Motor;

    self_2011 = self_2010;
    other_1655 = other_1654;
    let _e4: Scalar = self_2011;
    let _e5: Motor = other_1655;
    let _e6: Motor = motor_inverse(_e5);
    let _e7: Motor = scalar_motor_geometric_product(_e4, _e6);
    return _e7;
}

fn scalar_motor_transformation(self_2012: Scalar, other_1656: Motor) -> Motor {
    var self_2013: Scalar;
    var other_1657: Motor;

    self_2013 = self_2012;
    other_1657 = other_1656;
    let _e4: Scalar = self_2013;
    let _e5: Motor = other_1657;
    let _e6: Motor = scalar_motor_geometric_product(_e4, _e5);
    let _e7: Scalar = self_2013;
    let _e8: Scalar = scalar_reversal(_e7);
    let _e9: Motor = motor_scalar_geometric_product(_e6, _e8);
    return _e9;
}

fn scalar_plane_geometric_quotient(self_2014: Scalar, other_1658: Plane) -> Plane {
    var self_2015: Scalar;
    var other_1659: Plane;

    self_2015 = self_2014;
    other_1659 = other_1658;
    let _e4: Scalar = self_2015;
    let _e5: Plane = other_1659;
    let _e6: Plane = plane_inverse(_e5);
    let _e7: Plane = scalar_plane_geometric_product(_e4, _e6);
    return _e7;
}

fn scalar_plane_transformation(self_2016: Scalar, other_1660: Plane) -> Plane {
    var self_2017: Scalar;
    var other_1661: Plane;

    self_2017 = self_2016;
    other_1661 = other_1660;
    let _e4: Scalar = self_2017;
    let _e5: Plane = other_1661;
    let _e6: Plane = scalar_plane_geometric_product(_e4, _e5);
    let _e7: Scalar = self_2017;
    let _e8: Scalar = scalar_reversal(_e7);
    let _e9: Plane = plane_scalar_geometric_product(_e6, _e8);
    return _e9;
}

fn scalar_point_geometric_quotient(self_2018: Scalar, other_1662: Point) -> Point {
    var self_2019: Scalar;
    var other_1663: Point;

    self_2019 = self_2018;
    other_1663 = other_1662;
    let _e4: Scalar = self_2019;
    let _e5: Point = other_1663;
    let _e6: Point = point_inverse(_e5);
    let _e7: Point = scalar_point_geometric_product(_e4, _e6);
    return _e7;
}

fn scalar_point_transformation(self_2020: Scalar, other_1664: Point) -> Point {
    var self_2021: Scalar;
    var other_1665: Point;

    self_2021 = self_2020;
    other_1665 = other_1664;
    let _e4: Scalar = self_2021;
    let _e5: Point = other_1665;
    let _e6: Point = scalar_point_geometric_product(_e4, _e5);
    let _e7: Scalar = self_2021;
    let _e8: Scalar = scalar_reversal(_e7);
    let _e9: Point = point_scalar_geometric_product(_e6, _e8);
    return _e9;
}

fn scalar_rotor_transformation(self_2022: Scalar, other_1666: Rotor) -> Rotor {
    var self_2023: Scalar;
    var other_1667: Rotor;

    self_2023 = self_2022;
    other_1667 = other_1666;
    let _e4: Scalar = self_2023;
    let _e5: Rotor = other_1667;
    let _e6: Rotor = scalar_rotor_geometric_product(_e4, _e5);
    let _e7: Scalar = self_2023;
    let _e8: Scalar = scalar_reversal(_e7);
    let _e9: Rotor = rotor_scalar_geometric_product(_e6, _e8);
    return _e9;
}

fn scalar_powi(self_2024: Scalar, exponent_2: i32) -> Scalar {
    var self_2025: Scalar;
    var exponent_3: i32;
    var local_1: Scalar;
    var x_1: Scalar;
    var y_1: Scalar;
    var n_1: i32;

    self_2025 = self_2024;
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
        let _e11: Scalar = self_2025;
        let _e12: Scalar = scalar_inverse(_e11);
        local_1 = _e12;
    } else {
        let _e14: Scalar = self_2025;
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

fn scalar_scalar_geometric_quotient(self_2026: Scalar, other_1668: Scalar) -> Scalar {
    var self_2027: Scalar;
    var other_1669: Scalar;

    self_2027 = self_2026;
    other_1669 = other_1668;
    let _e4: Scalar = self_2027;
    let _e5: Scalar = other_1669;
    let _e6: Scalar = scalar_inverse(_e5);
    let _e7: Scalar = scalar_scalar_geometric_product(_e4, _e6);
    return _e7;
}

fn scalar_scalar_transformation(self_2028: Scalar, other_1670: Scalar) -> Scalar {
    var self_2029: Scalar;
    var other_1671: Scalar;

    self_2029 = self_2028;
    other_1671 = other_1670;
    let _e4: Scalar = self_2029;
    let _e5: Scalar = other_1671;
    let _e6: Scalar = scalar_scalar_geometric_product(_e4, _e5);
    let _e7: Scalar = self_2029;
    let _e8: Scalar = scalar_reversal(_e7);
    let _e9: Scalar = scalar_scalar_geometric_product(_e6, _e8);
    return _e9;
}

fn scalar_translator_geometric_quotient(self_2030: Scalar, other_1672: Translator) -> Translator {
    var self_2031: Scalar;
    var other_1673: Translator;

    self_2031 = self_2030;
    other_1673 = other_1672;
    let _e4: Scalar = self_2031;
    let _e5: Translator = other_1673;
    let _e6: Translator = translator_inverse(_e5);
    let _e7: Translator = scalar_translator_geometric_product(_e4, _e6);
    return _e7;
}

fn scalar_translator_transformation(self_2032: Scalar, other_1674: Translator) -> Translator {
    var self_2033: Scalar;
    var other_1675: Translator;

    self_2033 = self_2032;
    other_1675 = other_1674;
    let _e4: Scalar = self_2033;
    let _e5: Translator = other_1675;
    let _e6: Translator = scalar_translator_geometric_product(_e4, _e5);
    let _e7: Scalar = self_2033;
    let _e8: Scalar = scalar_reversal(_e7);
    let _e9: Translator = translator_scalar_geometric_product(_e6, _e8);
    return _e9;
}

fn translator_flector_geometric_quotient(self_2034: Translator, other_1676: Flector) -> Flector {
    var self_2035: Translator;
    var other_1677: Flector;

    self_2035 = self_2034;
    other_1677 = other_1676;
    let _e4: Translator = self_2035;
    let _e5: Flector = other_1677;
    let _e6: Flector = flector_inverse(_e5);
    let _e7: Flector = translator_flector_geometric_product(_e4, _e6);
    return _e7;
}

fn translator_flector_transformation(self_2036: Translator, other_1678: Flector) -> Flector {
    var self_2037: Translator;
    var other_1679: Flector;

    self_2037 = self_2036;
    other_1679 = other_1678;
    let _e4: Translator = self_2037;
    let _e5: Flector = other_1679;
    let _e6: Flector = translator_flector_geometric_product(_e4, _e5);
    let _e7: Translator = self_2037;
    let _e8: Translator = translator_reversal(_e7);
    let _e9: Flector = flector_translator_geometric_product(_e6, _e8);
    return _e9;
}

fn translator_homogeneous_magnitude_geometric_quotient(self_2038: Translator, other_1680: HomogeneousMagnitude) -> Motor {
    var self_2039: Translator;
    var other_1681: HomogeneousMagnitude;

    self_2039 = self_2038;
    other_1681 = other_1680;
    let _e4: Translator = self_2039;
    let _e5: HomogeneousMagnitude = other_1681;
    let _e6: HomogeneousMagnitude = homogeneous_magnitude_inverse(_e5);
    let _e7: Motor = translator_homogeneous_magnitude_geometric_product(_e4, _e6);
    return _e7;
}

fn translator_rotor_transformation(self_2040: Translator, other_1682: Rotor) -> Rotor {
    var self_2041: Translator;
    var other_1683: Rotor;

    self_2041 = self_2040;
    other_1683 = other_1682;
    let _e4: Translator = self_2041;
    let _e5: Rotor = other_1683;
    let _e6: Rotor = translator_rotor_geometric_product(_e4, _e5);
    let _e7: Translator = self_2041;
    let _e8: Translator = translator_reversal(_e7);
    let _e9: Rotor = rotor_translator_geometric_product(_e6, _e8);
    return _e9;
}

fn translator_scalar_geometric_quotient(self_2042: Translator, other_1684: Scalar) -> Translator {
    var self_2043: Translator;
    var other_1685: Scalar;

    self_2043 = self_2042;
    other_1685 = other_1684;
    let _e4: Translator = self_2043;
    let _e5: Scalar = other_1685;
    let _e6: Scalar = scalar_inverse(_e5);
    let _e7: Translator = translator_scalar_geometric_product(_e4, _e6);
    return _e7;
}


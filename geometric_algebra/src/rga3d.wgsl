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

fn scalar_scalar_wedge(self_32: Scalar, other_12: Scalar) -> Scalar {
    var self_33: Scalar;
    var other_13: Scalar;

    self_33 = self_32;
    other_13 = other_12;
    let _e4: Scalar = self_33;
    let _e6: Scalar = other_13;
    return Scalar((_e4.g0_ * _e6.g0_));
}

fn scalar_scalar_join(self_34: Scalar, other_14: Scalar) -> Scalar {
    var self_35: Scalar;
    var other_15: Scalar;

    self_35 = self_34;
    other_15 = other_14;
    let _e4: Scalar = self_35;
    let _e6: Scalar = other_15;
    return Scalar((_e4.g0_ * _e6.g0_));
}

fn scalar_scalar_inner_product(self_36: Scalar, other_16: Scalar) -> Scalar {
    var self_37: Scalar;
    var other_17: Scalar;

    self_37 = self_36;
    other_17 = other_16;
    let _e4: Scalar = self_37;
    let _e6: Scalar = other_17;
    return Scalar((_e4.g0_ * _e6.g0_));
}

fn scalar_scalar_left_contraction(self_38: Scalar, other_18: Scalar) -> Scalar {
    var self_39: Scalar;
    var other_19: Scalar;

    self_39 = self_38;
    other_19 = other_18;
    let _e4: Scalar = self_39;
    let _e6: Scalar = other_19;
    return Scalar((_e4.g0_ * _e6.g0_));
}

fn scalar_scalar_right_contraction(self_40: Scalar, other_20: Scalar) -> Scalar {
    var self_41: Scalar;
    var other_21: Scalar;

    self_41 = self_40;
    other_21 = other_20;
    let _e4: Scalar = self_41;
    let _e6: Scalar = other_21;
    return Scalar((_e4.g0_ * _e6.g0_));
}

fn scalar_scalar_scalar_product(self_42: Scalar, other_22: Scalar) -> Scalar {
    var self_43: Scalar;
    var other_23: Scalar;

    self_43 = self_42;
    other_23 = other_22;
    let _e4: Scalar = self_43;
    let _e6: Scalar = other_23;
    return Scalar((_e4.g0_ * _e6.g0_));
}

fn scalar_scalar_dot(self_44: Scalar, other_24: Scalar) -> Scalar {
    var self_45: Scalar;
    var other_25: Scalar;

    self_45 = self_44;
    other_25 = other_24;
    let _e4: Scalar = self_45;
    let _e6: Scalar = other_25;
    return Scalar((_e4.g0_ * _e6.g0_));
}

fn scalar_anti_scalar_add(self_46: Scalar, other_26: AntiScalar) -> HomogeneousMagnitude {
    var self_47: Scalar;
    var other_27: AntiScalar;

    self_47 = self_46;
    other_27 = other_26;
    let _e4: Scalar = self_47;
    let _e11: AntiScalar = other_27;
    return HomogeneousMagnitude(((vec2<f32>(_e4.g0_) * vec2<f32>(1.0, 0.0)) + (vec2<f32>(_e11.g0_) * vec2<f32>(0.0, 1.0))));
}

fn scalar_anti_scalar_sub(self_48: Scalar, other_28: AntiScalar) -> HomogeneousMagnitude {
    var self_49: Scalar;
    var other_29: AntiScalar;

    self_49 = self_48;
    other_29 = other_28;
    let _e4: Scalar = self_49;
    let _e11: AntiScalar = other_29;
    return HomogeneousMagnitude(((vec2<f32>(_e4.g0_) * vec2<f32>(1.0, 0.0)) - (vec2<f32>(_e11.g0_) * vec2<f32>(0.0, 1.0))));
}

fn scalar_anti_scalar_geometric_product(self_50: Scalar, other_30: AntiScalar) -> AntiScalar {
    var self_51: Scalar;
    var other_31: AntiScalar;

    self_51 = self_50;
    other_31 = other_30;
    let _e4: Scalar = self_51;
    let _e6: AntiScalar = other_31;
    return AntiScalar((_e4.g0_ * _e6.g0_));
}

fn scalar_anti_scalar_regressive_product(self_52: Scalar, other_32: AntiScalar) -> Scalar {
    var self_53: Scalar;
    var other_33: AntiScalar;

    self_53 = self_52;
    other_33 = other_32;
    let _e4: Scalar = self_53;
    let _e6: AntiScalar = other_33;
    return Scalar((_e4.g0_ * _e6.g0_));
}

fn scalar_anti_scalar_anti_wedge(self_54: Scalar, other_34: AntiScalar) -> Scalar {
    var self_55: Scalar;
    var other_35: AntiScalar;

    self_55 = self_54;
    other_35 = other_34;
    let _e4: Scalar = self_55;
    let _e6: AntiScalar = other_35;
    return Scalar((_e4.g0_ * _e6.g0_));
}

fn scalar_anti_scalar_meet(self_56: Scalar, other_36: AntiScalar) -> Scalar {
    var self_57: Scalar;
    var other_37: AntiScalar;

    self_57 = self_56;
    other_37 = other_36;
    let _e4: Scalar = self_57;
    let _e6: AntiScalar = other_37;
    return Scalar((_e4.g0_ * _e6.g0_));
}

fn scalar_anti_scalar_outer_product(self_58: Scalar, other_38: AntiScalar) -> AntiScalar {
    var self_59: Scalar;
    var other_39: AntiScalar;

    self_59 = self_58;
    other_39 = other_38;
    let _e4: Scalar = self_59;
    let _e6: AntiScalar = other_39;
    return AntiScalar((_e4.g0_ * _e6.g0_));
}

fn scalar_anti_scalar_wedge(self_60: Scalar, other_40: AntiScalar) -> AntiScalar {
    var self_61: Scalar;
    var other_41: AntiScalar;

    self_61 = self_60;
    other_41 = other_40;
    let _e4: Scalar = self_61;
    let _e6: AntiScalar = other_41;
    return AntiScalar((_e4.g0_ * _e6.g0_));
}

fn scalar_anti_scalar_join(self_62: Scalar, other_42: AntiScalar) -> AntiScalar {
    var self_63: Scalar;
    var other_43: AntiScalar;

    self_63 = self_62;
    other_43 = other_42;
    let _e4: Scalar = self_63;
    let _e6: AntiScalar = other_43;
    return AntiScalar((_e4.g0_ * _e6.g0_));
}

fn scalar_anti_scalar_inner_product(self_64: Scalar, other_44: AntiScalar) -> AntiScalar {
    var self_65: Scalar;
    var other_45: AntiScalar;

    self_65 = self_64;
    other_45 = other_44;
    let _e4: Scalar = self_65;
    let _e6: AntiScalar = other_45;
    return AntiScalar((_e4.g0_ * _e6.g0_));
}

fn scalar_anti_scalar_geometric_anti_product(self_66: Scalar, other_46: AntiScalar) -> Scalar {
    var self_67: Scalar;
    var other_47: AntiScalar;

    self_67 = self_66;
    other_47 = other_46;
    let _e4: Scalar = self_67;
    let _e6: AntiScalar = other_47;
    return Scalar((_e4.g0_ * _e6.g0_));
}

fn scalar_anti_scalar_inner_anti_product(self_68: Scalar, other_48: AntiScalar) -> Scalar {
    var self_69: Scalar;
    var other_49: AntiScalar;

    self_69 = self_68;
    other_49 = other_48;
    let _e4: Scalar = self_69;
    let _e6: AntiScalar = other_49;
    return Scalar((_e4.g0_ * _e6.g0_));
}

fn scalar_anti_scalar_left_contraction(self_70: Scalar, other_50: AntiScalar) -> AntiScalar {
    var self_71: Scalar;
    var other_51: AntiScalar;

    self_71 = self_70;
    other_51 = other_50;
    let _e4: Scalar = self_71;
    let _e6: AntiScalar = other_51;
    return AntiScalar((_e4.g0_ * _e6.g0_));
}

fn scalar_anti_scalar_right_anti_contraction(self_72: Scalar, other_52: AntiScalar) -> Scalar {
    var self_73: Scalar;
    var other_53: AntiScalar;

    self_73 = self_72;
    other_53 = other_52;
    let _e4: Scalar = self_73;
    let _e6: AntiScalar = other_53;
    return Scalar((_e4.g0_ * _e6.g0_));
}

fn scalar_homogeneous_magnitude_add(self_74: Scalar, other_54: HomogeneousMagnitude) -> HomogeneousMagnitude {
    var self_75: Scalar;
    var other_55: HomogeneousMagnitude;

    self_75 = self_74;
    other_55 = other_54;
    let _e4: Scalar = self_75;
    let _e11: HomogeneousMagnitude = other_55;
    return HomogeneousMagnitude(((vec2<f32>(_e4.g0_) * vec2<f32>(1.0, 0.0)) + _e11.g0_));
}

fn scalar_homogeneous_magnitude_sub(self_76: Scalar, other_56: HomogeneousMagnitude) -> HomogeneousMagnitude {
    var self_77: Scalar;
    var other_57: HomogeneousMagnitude;

    self_77 = self_76;
    other_57 = other_56;
    let _e4: Scalar = self_77;
    let _e11: HomogeneousMagnitude = other_57;
    return HomogeneousMagnitude(((vec2<f32>(_e4.g0_) * vec2<f32>(1.0, 0.0)) - _e11.g0_));
}

fn scalar_homogeneous_magnitude_geometric_product(self_78: Scalar, other_58: HomogeneousMagnitude) -> HomogeneousMagnitude {
    var self_79: Scalar;
    var other_59: HomogeneousMagnitude;

    self_79 = self_78;
    other_59 = other_58;
    let _e4: Scalar = self_79;
    let _e7: HomogeneousMagnitude = other_59;
    return HomogeneousMagnitude((vec2<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_homogeneous_magnitude_regressive_product(self_80: Scalar, other_60: HomogeneousMagnitude) -> Scalar {
    var self_81: Scalar;
    var other_61: HomogeneousMagnitude;

    self_81 = self_80;
    other_61 = other_60;
    let _e4: Scalar = self_81;
    let _e6: HomogeneousMagnitude = other_61;
    return Scalar((_e4.g0_ * _e6.g0_.y));
}

fn scalar_homogeneous_magnitude_anti_wedge(self_82: Scalar, other_62: HomogeneousMagnitude) -> Scalar {
    var self_83: Scalar;
    var other_63: HomogeneousMagnitude;

    self_83 = self_82;
    other_63 = other_62;
    let _e4: Scalar = self_83;
    let _e6: HomogeneousMagnitude = other_63;
    return Scalar((_e4.g0_ * _e6.g0_.y));
}

fn scalar_homogeneous_magnitude_meet(self_84: Scalar, other_64: HomogeneousMagnitude) -> Scalar {
    var self_85: Scalar;
    var other_65: HomogeneousMagnitude;

    self_85 = self_84;
    other_65 = other_64;
    let _e4: Scalar = self_85;
    let _e6: HomogeneousMagnitude = other_65;
    return Scalar((_e4.g0_ * _e6.g0_.y));
}

fn scalar_homogeneous_magnitude_outer_product(self_86: Scalar, other_66: HomogeneousMagnitude) -> HomogeneousMagnitude {
    var self_87: Scalar;
    var other_67: HomogeneousMagnitude;

    self_87 = self_86;
    other_67 = other_66;
    let _e4: Scalar = self_87;
    let _e7: HomogeneousMagnitude = other_67;
    return HomogeneousMagnitude((vec2<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_homogeneous_magnitude_wedge(self_88: Scalar, other_68: HomogeneousMagnitude) -> HomogeneousMagnitude {
    var self_89: Scalar;
    var other_69: HomogeneousMagnitude;

    self_89 = self_88;
    other_69 = other_68;
    let _e4: Scalar = self_89;
    let _e7: HomogeneousMagnitude = other_69;
    return HomogeneousMagnitude((vec2<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_homogeneous_magnitude_join(self_90: Scalar, other_70: HomogeneousMagnitude) -> HomogeneousMagnitude {
    var self_91: Scalar;
    var other_71: HomogeneousMagnitude;

    self_91 = self_90;
    other_71 = other_70;
    let _e4: Scalar = self_91;
    let _e7: HomogeneousMagnitude = other_71;
    return HomogeneousMagnitude((vec2<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_homogeneous_magnitude_inner_product(self_92: Scalar, other_72: HomogeneousMagnitude) -> HomogeneousMagnitude {
    var self_93: Scalar;
    var other_73: HomogeneousMagnitude;

    self_93 = self_92;
    other_73 = other_72;
    let _e4: Scalar = self_93;
    let _e7: HomogeneousMagnitude = other_73;
    return HomogeneousMagnitude((vec2<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_homogeneous_magnitude_geometric_anti_product(self_94: Scalar, other_74: HomogeneousMagnitude) -> Scalar {
    var self_95: Scalar;
    var other_75: HomogeneousMagnitude;

    self_95 = self_94;
    other_75 = other_74;
    let _e4: Scalar = self_95;
    let _e6: HomogeneousMagnitude = other_75;
    return Scalar((_e4.g0_ * _e6.g0_.y));
}

fn scalar_homogeneous_magnitude_inner_anti_product(self_96: Scalar, other_76: HomogeneousMagnitude) -> Scalar {
    var self_97: Scalar;
    var other_77: HomogeneousMagnitude;

    self_97 = self_96;
    other_77 = other_76;
    let _e4: Scalar = self_97;
    let _e6: HomogeneousMagnitude = other_77;
    return Scalar((_e4.g0_ * _e6.g0_.y));
}

fn scalar_homogeneous_magnitude_left_contraction(self_98: Scalar, other_78: HomogeneousMagnitude) -> HomogeneousMagnitude {
    var self_99: Scalar;
    var other_79: HomogeneousMagnitude;

    self_99 = self_98;
    other_79 = other_78;
    let _e4: Scalar = self_99;
    let _e7: HomogeneousMagnitude = other_79;
    return HomogeneousMagnitude((vec2<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_homogeneous_magnitude_right_contraction(self_100: Scalar, other_80: HomogeneousMagnitude) -> Scalar {
    var self_101: Scalar;
    var other_81: HomogeneousMagnitude;

    self_101 = self_100;
    other_81 = other_80;
    let _e4: Scalar = self_101;
    let _e6: HomogeneousMagnitude = other_81;
    return Scalar((_e4.g0_ * _e6.g0_.x));
}

fn scalar_homogeneous_magnitude_right_anti_contraction(self_102: Scalar, other_82: HomogeneousMagnitude) -> Scalar {
    var self_103: Scalar;
    var other_83: HomogeneousMagnitude;

    self_103 = self_102;
    other_83 = other_82;
    let _e4: Scalar = self_103;
    let _e6: HomogeneousMagnitude = other_83;
    return Scalar((_e4.g0_ * _e6.g0_.y));
}

fn scalar_homogeneous_magnitude_scalar_product(self_104: Scalar, other_84: HomogeneousMagnitude) -> Scalar {
    var self_105: Scalar;
    var other_85: HomogeneousMagnitude;

    self_105 = self_104;
    other_85 = other_84;
    let _e4: Scalar = self_105;
    let _e6: HomogeneousMagnitude = other_85;
    return Scalar((_e4.g0_ * _e6.g0_.x));
}

fn scalar_homogeneous_magnitude_dot(self_106: Scalar, other_86: HomogeneousMagnitude) -> Scalar {
    var self_107: Scalar;
    var other_87: HomogeneousMagnitude;

    self_107 = self_106;
    other_87 = other_86;
    let _e4: Scalar = self_107;
    let _e6: HomogeneousMagnitude = other_87;
    return Scalar((_e4.g0_ * _e6.g0_.x));
}

fn scalar_point_geometric_product(self_108: Scalar, other_88: Point) -> Point {
    var self_109: Scalar;
    var other_89: Point;

    self_109 = self_108;
    other_89 = other_88;
    let _e4: Scalar = self_109;
    let _e7: Point = other_89;
    return Point((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_point_outer_product(self_110: Scalar, other_90: Point) -> Point {
    var self_111: Scalar;
    var other_91: Point;

    self_111 = self_110;
    other_91 = other_90;
    let _e4: Scalar = self_111;
    let _e7: Point = other_91;
    return Point((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_point_wedge(self_112: Scalar, other_92: Point) -> Point {
    var self_113: Scalar;
    var other_93: Point;

    self_113 = self_112;
    other_93 = other_92;
    let _e4: Scalar = self_113;
    let _e7: Point = other_93;
    return Point((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_point_join(self_114: Scalar, other_94: Point) -> Point {
    var self_115: Scalar;
    var other_95: Point;

    self_115 = self_114;
    other_95 = other_94;
    let _e4: Scalar = self_115;
    let _e7: Point = other_95;
    return Point((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_point_inner_product(self_116: Scalar, other_96: Point) -> Point {
    var self_117: Scalar;
    var other_97: Point;

    self_117 = self_116;
    other_97 = other_96;
    let _e4: Scalar = self_117;
    let _e7: Point = other_97;
    return Point((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_point_left_contraction(self_118: Scalar, other_98: Point) -> Point {
    var self_119: Scalar;
    var other_99: Point;

    self_119 = self_118;
    other_99 = other_98;
    let _e4: Scalar = self_119;
    let _e7: Point = other_99;
    return Point((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_line_geometric_product(self_120: Scalar, other_100: Line) -> Line {
    var self_121: Scalar;
    var other_101: Line;

    self_121 = self_120;
    other_101 = other_100;
    let _e4: Scalar = self_121;
    let _e7: Line = other_101;
    let _e10: Scalar = self_121;
    let _e13: Line = other_101;
    return Line((vec3<f32>(_e4.g0_) * _e7.g0_), (vec3<f32>(_e10.g0_) * _e13.g1_));
}

fn scalar_line_outer_product(self_122: Scalar, other_102: Line) -> Line {
    var self_123: Scalar;
    var other_103: Line;

    self_123 = self_122;
    other_103 = other_102;
    let _e4: Scalar = self_123;
    let _e7: Line = other_103;
    let _e10: Scalar = self_123;
    let _e13: Line = other_103;
    return Line((vec3<f32>(_e4.g0_) * _e7.g0_), (vec3<f32>(_e10.g0_) * _e13.g1_));
}

fn scalar_line_wedge(self_124: Scalar, other_104: Line) -> Line {
    var self_125: Scalar;
    var other_105: Line;

    self_125 = self_124;
    other_105 = other_104;
    let _e4: Scalar = self_125;
    let _e7: Line = other_105;
    let _e10: Scalar = self_125;
    let _e13: Line = other_105;
    return Line((vec3<f32>(_e4.g0_) * _e7.g0_), (vec3<f32>(_e10.g0_) * _e13.g1_));
}

fn scalar_line_join(self_126: Scalar, other_106: Line) -> Line {
    var self_127: Scalar;
    var other_107: Line;

    self_127 = self_126;
    other_107 = other_106;
    let _e4: Scalar = self_127;
    let _e7: Line = other_107;
    let _e10: Scalar = self_127;
    let _e13: Line = other_107;
    return Line((vec3<f32>(_e4.g0_) * _e7.g0_), (vec3<f32>(_e10.g0_) * _e13.g1_));
}

fn scalar_line_inner_product(self_128: Scalar, other_108: Line) -> Line {
    var self_129: Scalar;
    var other_109: Line;

    self_129 = self_128;
    other_109 = other_108;
    let _e4: Scalar = self_129;
    let _e7: Line = other_109;
    let _e10: Scalar = self_129;
    let _e13: Line = other_109;
    return Line((vec3<f32>(_e4.g0_) * _e7.g0_), (vec3<f32>(_e10.g0_) * _e13.g1_));
}

fn scalar_line_left_contraction(self_130: Scalar, other_110: Line) -> Line {
    var self_131: Scalar;
    var other_111: Line;

    self_131 = self_130;
    other_111 = other_110;
    let _e4: Scalar = self_131;
    let _e7: Line = other_111;
    let _e10: Scalar = self_131;
    let _e13: Line = other_111;
    return Line((vec3<f32>(_e4.g0_) * _e7.g0_), (vec3<f32>(_e10.g0_) * _e13.g1_));
}

fn scalar_plane_geometric_product(self_132: Scalar, other_112: Plane) -> Plane {
    var self_133: Scalar;
    var other_113: Plane;

    self_133 = self_132;
    other_113 = other_112;
    let _e4: Scalar = self_133;
    let _e7: Plane = other_113;
    return Plane((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_plane_outer_product(self_134: Scalar, other_114: Plane) -> Plane {
    var self_135: Scalar;
    var other_115: Plane;

    self_135 = self_134;
    other_115 = other_114;
    let _e4: Scalar = self_135;
    let _e7: Plane = other_115;
    return Plane((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_plane_wedge(self_136: Scalar, other_116: Plane) -> Plane {
    var self_137: Scalar;
    var other_117: Plane;

    self_137 = self_136;
    other_117 = other_116;
    let _e4: Scalar = self_137;
    let _e7: Plane = other_117;
    return Plane((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_plane_join(self_138: Scalar, other_118: Plane) -> Plane {
    var self_139: Scalar;
    var other_119: Plane;

    self_139 = self_138;
    other_119 = other_118;
    let _e4: Scalar = self_139;
    let _e7: Plane = other_119;
    return Plane((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_plane_inner_product(self_140: Scalar, other_120: Plane) -> Plane {
    var self_141: Scalar;
    var other_121: Plane;

    self_141 = self_140;
    other_121 = other_120;
    let _e4: Scalar = self_141;
    let _e7: Plane = other_121;
    return Plane((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_plane_left_contraction(self_142: Scalar, other_122: Plane) -> Plane {
    var self_143: Scalar;
    var other_123: Plane;

    self_143 = self_142;
    other_123 = other_122;
    let _e4: Scalar = self_143;
    let _e7: Plane = other_123;
    return Plane((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_motor_geometric_product(self_144: Scalar, other_124: Motor) -> Motor {
    var self_145: Scalar;
    var other_125: Motor;

    self_145 = self_144;
    other_125 = other_124;
    let _e4: Scalar = self_145;
    let _e7: Motor = other_125;
    let _e10: Scalar = self_145;
    let _e13: Motor = other_125;
    return Motor((vec4<f32>(_e4.g0_) * _e7.g0_), (vec3<f32>(_e10.g0_) * _e13.g1_));
}

fn scalar_motor_regressive_product(self_146: Scalar, other_126: Motor) -> Scalar {
    var self_147: Scalar;
    var other_127: Motor;

    self_147 = self_146;
    other_127 = other_126;
    let _e4: Scalar = self_147;
    let _e6: Motor = other_127;
    return Scalar((_e4.g0_ * _e6.g0_.w));
}

fn scalar_motor_anti_wedge(self_148: Scalar, other_128: Motor) -> Scalar {
    var self_149: Scalar;
    var other_129: Motor;

    self_149 = self_148;
    other_129 = other_128;
    let _e4: Scalar = self_149;
    let _e6: Motor = other_129;
    return Scalar((_e4.g0_ * _e6.g0_.w));
}

fn scalar_motor_meet(self_150: Scalar, other_130: Motor) -> Scalar {
    var self_151: Scalar;
    var other_131: Motor;

    self_151 = self_150;
    other_131 = other_130;
    let _e4: Scalar = self_151;
    let _e6: Motor = other_131;
    return Scalar((_e4.g0_ * _e6.g0_.w));
}

fn scalar_motor_outer_product(self_152: Scalar, other_132: Motor) -> Motor {
    var self_153: Scalar;
    var other_133: Motor;

    self_153 = self_152;
    other_133 = other_132;
    let _e4: Scalar = self_153;
    let _e7: Motor = other_133;
    let _e10: Scalar = self_153;
    let _e13: Motor = other_133;
    return Motor((vec4<f32>(_e4.g0_) * _e7.g0_), (vec3<f32>(_e10.g0_) * _e13.g1_));
}

fn scalar_motor_wedge(self_154: Scalar, other_134: Motor) -> Motor {
    var self_155: Scalar;
    var other_135: Motor;

    self_155 = self_154;
    other_135 = other_134;
    let _e4: Scalar = self_155;
    let _e7: Motor = other_135;
    let _e10: Scalar = self_155;
    let _e13: Motor = other_135;
    return Motor((vec4<f32>(_e4.g0_) * _e7.g0_), (vec3<f32>(_e10.g0_) * _e13.g1_));
}

fn scalar_motor_join(self_156: Scalar, other_136: Motor) -> Motor {
    var self_157: Scalar;
    var other_137: Motor;

    self_157 = self_156;
    other_137 = other_136;
    let _e4: Scalar = self_157;
    let _e7: Motor = other_137;
    let _e10: Scalar = self_157;
    let _e13: Motor = other_137;
    return Motor((vec4<f32>(_e4.g0_) * _e7.g0_), (vec3<f32>(_e10.g0_) * _e13.g1_));
}

fn scalar_motor_inner_product(self_158: Scalar, other_138: Motor) -> Motor {
    var self_159: Scalar;
    var other_139: Motor;

    self_159 = self_158;
    other_139 = other_138;
    let _e4: Scalar = self_159;
    let _e7: Motor = other_139;
    let _e10: Scalar = self_159;
    let _e13: Motor = other_139;
    return Motor((vec4<f32>(_e4.g0_) * _e7.g0_), (vec3<f32>(_e10.g0_) * _e13.g1_));
}

fn scalar_motor_left_contraction(self_160: Scalar, other_140: Motor) -> Motor {
    var self_161: Scalar;
    var other_141: Motor;

    self_161 = self_160;
    other_141 = other_140;
    let _e4: Scalar = self_161;
    let _e7: Motor = other_141;
    let _e10: Scalar = self_161;
    let _e13: Motor = other_141;
    return Motor((vec4<f32>(_e4.g0_) * _e7.g0_), (vec3<f32>(_e10.g0_) * _e13.g1_));
}

fn scalar_rotor_geometric_product(self_162: Scalar, other_142: Rotor) -> Rotor {
    var self_163: Scalar;
    var other_143: Rotor;

    self_163 = self_162;
    other_143 = other_142;
    let _e4: Scalar = self_163;
    let _e7: Rotor = other_143;
    return Rotor((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_rotor_regressive_product(self_164: Scalar, other_144: Rotor) -> Scalar {
    var self_165: Scalar;
    var other_145: Rotor;

    self_165 = self_164;
    other_145 = other_144;
    let _e4: Scalar = self_165;
    let _e6: Rotor = other_145;
    return Scalar((_e4.g0_ * _e6.g0_.w));
}

fn scalar_rotor_anti_wedge(self_166: Scalar, other_146: Rotor) -> Scalar {
    var self_167: Scalar;
    var other_147: Rotor;

    self_167 = self_166;
    other_147 = other_146;
    let _e4: Scalar = self_167;
    let _e6: Rotor = other_147;
    return Scalar((_e4.g0_ * _e6.g0_.w));
}

fn scalar_rotor_meet(self_168: Scalar, other_148: Rotor) -> Scalar {
    var self_169: Scalar;
    var other_149: Rotor;

    self_169 = self_168;
    other_149 = other_148;
    let _e4: Scalar = self_169;
    let _e6: Rotor = other_149;
    return Scalar((_e4.g0_ * _e6.g0_.w));
}

fn scalar_rotor_outer_product(self_170: Scalar, other_150: Rotor) -> Rotor {
    var self_171: Scalar;
    var other_151: Rotor;

    self_171 = self_170;
    other_151 = other_150;
    let _e4: Scalar = self_171;
    let _e7: Rotor = other_151;
    return Rotor((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_rotor_wedge(self_172: Scalar, other_152: Rotor) -> Rotor {
    var self_173: Scalar;
    var other_153: Rotor;

    self_173 = self_172;
    other_153 = other_152;
    let _e4: Scalar = self_173;
    let _e7: Rotor = other_153;
    return Rotor((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_rotor_join(self_174: Scalar, other_154: Rotor) -> Rotor {
    var self_175: Scalar;
    var other_155: Rotor;

    self_175 = self_174;
    other_155 = other_154;
    let _e4: Scalar = self_175;
    let _e7: Rotor = other_155;
    return Rotor((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_rotor_inner_product(self_176: Scalar, other_156: Rotor) -> Rotor {
    var self_177: Scalar;
    var other_157: Rotor;

    self_177 = self_176;
    other_157 = other_156;
    let _e4: Scalar = self_177;
    let _e7: Rotor = other_157;
    return Rotor((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_rotor_left_contraction(self_178: Scalar, other_158: Rotor) -> Rotor {
    var self_179: Scalar;
    var other_159: Rotor;

    self_179 = self_178;
    other_159 = other_158;
    let _e4: Scalar = self_179;
    let _e7: Rotor = other_159;
    return Rotor((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_translator_geometric_product(self_180: Scalar, other_160: Translator) -> Translator {
    var self_181: Scalar;
    var other_161: Translator;

    self_181 = self_180;
    other_161 = other_160;
    let _e4: Scalar = self_181;
    let _e7: Translator = other_161;
    return Translator((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_translator_regressive_product(self_182: Scalar, other_162: Translator) -> Scalar {
    var self_183: Scalar;
    var other_163: Translator;

    self_183 = self_182;
    other_163 = other_162;
    let _e4: Scalar = self_183;
    let _e6: Translator = other_163;
    return Scalar((_e4.g0_ * _e6.g0_.w));
}

fn scalar_translator_anti_wedge(self_184: Scalar, other_164: Translator) -> Scalar {
    var self_185: Scalar;
    var other_165: Translator;

    self_185 = self_184;
    other_165 = other_164;
    let _e4: Scalar = self_185;
    let _e6: Translator = other_165;
    return Scalar((_e4.g0_ * _e6.g0_.w));
}

fn scalar_translator_meet(self_186: Scalar, other_166: Translator) -> Scalar {
    var self_187: Scalar;
    var other_167: Translator;

    self_187 = self_186;
    other_167 = other_166;
    let _e4: Scalar = self_187;
    let _e6: Translator = other_167;
    return Scalar((_e4.g0_ * _e6.g0_.w));
}

fn scalar_translator_outer_product(self_188: Scalar, other_168: Translator) -> Translator {
    var self_189: Scalar;
    var other_169: Translator;

    self_189 = self_188;
    other_169 = other_168;
    let _e4: Scalar = self_189;
    let _e7: Translator = other_169;
    return Translator((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_translator_wedge(self_190: Scalar, other_170: Translator) -> Translator {
    var self_191: Scalar;
    var other_171: Translator;

    self_191 = self_190;
    other_171 = other_170;
    let _e4: Scalar = self_191;
    let _e7: Translator = other_171;
    return Translator((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_translator_join(self_192: Scalar, other_172: Translator) -> Translator {
    var self_193: Scalar;
    var other_173: Translator;

    self_193 = self_192;
    other_173 = other_172;
    let _e4: Scalar = self_193;
    let _e7: Translator = other_173;
    return Translator((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_translator_inner_product(self_194: Scalar, other_174: Translator) -> Translator {
    var self_195: Scalar;
    var other_175: Translator;

    self_195 = self_194;
    other_175 = other_174;
    let _e4: Scalar = self_195;
    let _e7: Translator = other_175;
    return Translator((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_translator_geometric_anti_product(self_196: Scalar, other_176: Translator) -> Scalar {
    var self_197: Scalar;
    var other_177: Translator;

    self_197 = self_196;
    other_177 = other_176;
    let _e4: Scalar = self_197;
    let _e6: Translator = other_177;
    return Scalar((_e4.g0_ * _e6.g0_.w));
}

fn scalar_translator_inner_anti_product(self_198: Scalar, other_178: Translator) -> Scalar {
    var self_199: Scalar;
    var other_179: Translator;

    self_199 = self_198;
    other_179 = other_178;
    let _e4: Scalar = self_199;
    let _e6: Translator = other_179;
    return Scalar((_e4.g0_ * _e6.g0_.w));
}

fn scalar_translator_left_contraction(self_200: Scalar, other_180: Translator) -> Translator {
    var self_201: Scalar;
    var other_181: Translator;

    self_201 = self_200;
    other_181 = other_180;
    let _e4: Scalar = self_201;
    let _e7: Translator = other_181;
    return Translator((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_translator_right_anti_contraction(self_202: Scalar, other_182: Translator) -> Scalar {
    var self_203: Scalar;
    var other_183: Translator;

    self_203 = self_202;
    other_183 = other_182;
    let _e4: Scalar = self_203;
    let _e6: Translator = other_183;
    return Scalar((_e4.g0_ * _e6.g0_.w));
}

fn scalar_flector_geometric_product(self_204: Scalar, other_184: Flector) -> Flector {
    var self_205: Scalar;
    var other_185: Flector;

    self_205 = self_204;
    other_185 = other_184;
    let _e4: Scalar = self_205;
    let _e7: Flector = other_185;
    let _e10: Scalar = self_205;
    let _e13: Flector = other_185;
    return Flector((vec4<f32>(_e4.g0_) * _e7.g0_), (vec4<f32>(_e10.g0_) * _e13.g1_));
}

fn scalar_flector_outer_product(self_206: Scalar, other_186: Flector) -> Flector {
    var self_207: Scalar;
    var other_187: Flector;

    self_207 = self_206;
    other_187 = other_186;
    let _e4: Scalar = self_207;
    let _e7: Flector = other_187;
    let _e10: Scalar = self_207;
    let _e13: Flector = other_187;
    return Flector((vec4<f32>(_e4.g0_) * _e7.g0_), (vec4<f32>(_e10.g0_) * _e13.g1_));
}

fn scalar_flector_wedge(self_208: Scalar, other_188: Flector) -> Flector {
    var self_209: Scalar;
    var other_189: Flector;

    self_209 = self_208;
    other_189 = other_188;
    let _e4: Scalar = self_209;
    let _e7: Flector = other_189;
    let _e10: Scalar = self_209;
    let _e13: Flector = other_189;
    return Flector((vec4<f32>(_e4.g0_) * _e7.g0_), (vec4<f32>(_e10.g0_) * _e13.g1_));
}

fn scalar_flector_join(self_210: Scalar, other_190: Flector) -> Flector {
    var self_211: Scalar;
    var other_191: Flector;

    self_211 = self_210;
    other_191 = other_190;
    let _e4: Scalar = self_211;
    let _e7: Flector = other_191;
    let _e10: Scalar = self_211;
    let _e13: Flector = other_191;
    return Flector((vec4<f32>(_e4.g0_) * _e7.g0_), (vec4<f32>(_e10.g0_) * _e13.g1_));
}

fn scalar_flector_inner_product(self_212: Scalar, other_192: Flector) -> Flector {
    var self_213: Scalar;
    var other_193: Flector;

    self_213 = self_212;
    other_193 = other_192;
    let _e4: Scalar = self_213;
    let _e7: Flector = other_193;
    let _e10: Scalar = self_213;
    let _e13: Flector = other_193;
    return Flector((vec4<f32>(_e4.g0_) * _e7.g0_), (vec4<f32>(_e10.g0_) * _e13.g1_));
}

fn scalar_flector_left_contraction(self_214: Scalar, other_194: Flector) -> Flector {
    var self_215: Scalar;
    var other_195: Flector;

    self_215 = self_214;
    other_195 = other_194;
    let _e4: Scalar = self_215;
    let _e7: Flector = other_195;
    let _e10: Scalar = self_215;
    let _e13: Flector = other_195;
    return Flector((vec4<f32>(_e4.g0_) * _e7.g0_), (vec4<f32>(_e10.g0_) * _e13.g1_));
}

fn scalar_multi_vector_regressive_product(self_216: Scalar, other_196: MultiVector) -> Scalar {
    var self_217: Scalar;
    var other_197: MultiVector;

    self_217 = self_216;
    other_197 = other_196;
    let _e4: Scalar = self_217;
    let _e6: MultiVector = other_197;
    return Scalar((_e4.g0_ * _e6.g0_.y));
}

fn scalar_multi_vector_anti_wedge(self_218: Scalar, other_198: MultiVector) -> Scalar {
    var self_219: Scalar;
    var other_199: MultiVector;

    self_219 = self_218;
    other_199 = other_198;
    let _e4: Scalar = self_219;
    let _e6: MultiVector = other_199;
    return Scalar((_e4.g0_ * _e6.g0_.y));
}

fn scalar_multi_vector_meet(self_220: Scalar, other_200: MultiVector) -> Scalar {
    var self_221: Scalar;
    var other_201: MultiVector;

    self_221 = self_220;
    other_201 = other_200;
    let _e4: Scalar = self_221;
    let _e6: MultiVector = other_201;
    return Scalar((_e4.g0_ * _e6.g0_.y));
}

fn scalar_multi_vector_right_contraction(self_222: Scalar, other_202: MultiVector) -> Scalar {
    var self_223: Scalar;
    var other_203: MultiVector;

    self_223 = self_222;
    other_203 = other_202;
    let _e4: Scalar = self_223;
    let _e6: MultiVector = other_203;
    return Scalar((_e4.g0_ * _e6.g0_.x));
}

fn scalar_multi_vector_scalar_product(self_224: Scalar, other_204: MultiVector) -> Scalar {
    var self_225: Scalar;
    var other_205: MultiVector;

    self_225 = self_224;
    other_205 = other_204;
    let _e4: Scalar = self_225;
    let _e6: MultiVector = other_205;
    return Scalar((_e4.g0_ * _e6.g0_.x));
}

fn scalar_multi_vector_dot(self_226: Scalar, other_206: MultiVector) -> Scalar {
    var self_227: Scalar;
    var other_207: MultiVector;

    self_227 = self_226;
    other_207 = other_206;
    let _e4: Scalar = self_227;
    let _e6: MultiVector = other_207;
    return Scalar((_e4.g0_ * _e6.g0_.x));
}

fn scalar_squared_magnitude(self_228: Scalar) -> Scalar {
    var self_229: Scalar;

    self_229 = self_228;
    let _e2: Scalar = self_229;
    let _e3: Scalar = self_229;
    let _e4: Scalar = scalar_reversal(_e3);
    let _e5: Scalar = scalar_scalar_scalar_product(_e2, _e4);
    return _e5;
}

fn scalar_magnitude(self_230: Scalar) -> Scalar {
    var self_231: Scalar;

    self_231 = self_230;
    let _e2: Scalar = self_231;
    let _e3: Scalar = scalar_squared_magnitude(_e2);
    return Scalar(sqrt(_e3.g0_));
}

fn scalar_scale(self_232: Scalar, other_208: f32) -> Scalar {
    var self_233: Scalar;
    var other_209: f32;

    self_233 = self_232;
    other_209 = other_208;
    let _e4: Scalar = self_233;
    let _e5: f32 = other_209;
    let _e7: Scalar = scalar_scalar_geometric_product(_e4, Scalar(_e5));
    return _e7;
}

fn scalar_signum(self_234: Scalar) -> Scalar {
    var self_235: Scalar;

    self_235 = self_234;
    let _e2: Scalar = self_235;
    let _e3: Scalar = self_235;
    let _e4: Scalar = scalar_magnitude(_e3);
    let _e9: Scalar = scalar_scalar_geometric_product(_e2, Scalar((1.0 / _e4.g0_)));
    return _e9;
}

fn scalar_inverse(self_236: Scalar) -> Scalar {
    var self_237: Scalar;

    self_237 = self_236;
    let _e2: Scalar = self_237;
    let _e3: Scalar = scalar_reversal(_e2);
    let _e4: Scalar = self_237;
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

fn anti_scalar_grade(self_238: AntiScalar) -> i32 {
    return 4;
}

fn anti_scalar_anti_grade(self_239: AntiScalar) -> i32 {
    return 0;
}

fn anti_scalar_neg(self_240: AntiScalar) -> AntiScalar {
    var self_241: AntiScalar;

    self_241 = self_240;
    let _e2: AntiScalar = self_241;
    return AntiScalar((_e2.g0_ * -(1.0)));
}

fn anti_scalar_automorphism(self_242: AntiScalar) -> AntiScalar {
    var self_243: AntiScalar;

    self_243 = self_242;
    let _e2: AntiScalar = self_243;
    return AntiScalar(_e2.g0_);
}

fn anti_scalar_reversal(self_244: AntiScalar) -> AntiScalar {
    var self_245: AntiScalar;

    self_245 = self_244;
    let _e2: AntiScalar = self_245;
    return AntiScalar(_e2.g0_);
}

fn anti_scalar_conjugation(self_246: AntiScalar) -> AntiScalar {
    var self_247: AntiScalar;

    self_247 = self_246;
    let _e2: AntiScalar = self_247;
    return AntiScalar(_e2.g0_);
}

fn anti_scalar_dual(self_248: AntiScalar) -> Scalar {
    var self_249: AntiScalar;

    self_249 = self_248;
    let _e2: AntiScalar = self_249;
    return Scalar(_e2.g0_);
}

fn anti_scalar_anti_reversal(self_250: AntiScalar) -> AntiScalar {
    var self_251: AntiScalar;

    self_251 = self_250;
    let _e2: AntiScalar = self_251;
    return AntiScalar(_e2.g0_);
}

fn anti_scalar_right_complement(self_252: AntiScalar) -> Scalar {
    var self_253: AntiScalar;

    self_253 = self_252;
    let _e2: AntiScalar = self_253;
    return Scalar(_e2.g0_);
}

fn anti_scalar_left_complement(self_254: AntiScalar) -> Scalar {
    var self_255: AntiScalar;

    self_255 = self_254;
    let _e2: AntiScalar = self_255;
    return Scalar(_e2.g0_);
}

fn anti_scalar_double_complement(self_256: AntiScalar) -> AntiScalar {
    var self_257: AntiScalar;

    self_257 = self_256;
    let _e2: AntiScalar = self_257;
    return AntiScalar(_e2.g0_);
}

fn anti_scalar_scalar_add(self_258: AntiScalar, other_210: Scalar) -> HomogeneousMagnitude {
    var self_259: AntiScalar;
    var other_211: Scalar;

    self_259 = self_258;
    other_211 = other_210;
    let _e4: AntiScalar = self_259;
    let _e11: Scalar = other_211;
    return HomogeneousMagnitude(((vec2<f32>(_e4.g0_) * vec2<f32>(0.0, 1.0)) + (vec2<f32>(_e11.g0_) * vec2<f32>(1.0, 0.0))));
}

fn anti_scalar_scalar_sub(self_260: AntiScalar, other_212: Scalar) -> HomogeneousMagnitude {
    var self_261: AntiScalar;
    var other_213: Scalar;

    self_261 = self_260;
    other_213 = other_212;
    let _e4: AntiScalar = self_261;
    let _e11: Scalar = other_213;
    return HomogeneousMagnitude(((vec2<f32>(_e4.g0_) * vec2<f32>(0.0, 1.0)) - (vec2<f32>(_e11.g0_) * vec2<f32>(1.0, 0.0))));
}

fn anti_scalar_scalar_geometric_product(self_262: AntiScalar, other_214: Scalar) -> AntiScalar {
    var self_263: AntiScalar;
    var other_215: Scalar;

    self_263 = self_262;
    other_215 = other_214;
    let _e4: AntiScalar = self_263;
    let _e6: Scalar = other_215;
    return AntiScalar((_e4.g0_ * _e6.g0_));
}

fn anti_scalar_scalar_regressive_product(self_264: AntiScalar, other_216: Scalar) -> Scalar {
    var self_265: AntiScalar;
    var other_217: Scalar;

    self_265 = self_264;
    other_217 = other_216;
    let _e4: AntiScalar = self_265;
    let _e6: Scalar = other_217;
    return Scalar((_e4.g0_ * _e6.g0_));
}

fn anti_scalar_scalar_anti_wedge(self_266: AntiScalar, other_218: Scalar) -> Scalar {
    var self_267: AntiScalar;
    var other_219: Scalar;

    self_267 = self_266;
    other_219 = other_218;
    let _e4: AntiScalar = self_267;
    let _e6: Scalar = other_219;
    return Scalar((_e4.g0_ * _e6.g0_));
}

fn anti_scalar_scalar_meet(self_268: AntiScalar, other_220: Scalar) -> Scalar {
    var self_269: AntiScalar;
    var other_221: Scalar;

    self_269 = self_268;
    other_221 = other_220;
    let _e4: AntiScalar = self_269;
    let _e6: Scalar = other_221;
    return Scalar((_e4.g0_ * _e6.g0_));
}

fn anti_scalar_scalar_outer_product(self_270: AntiScalar, other_222: Scalar) -> AntiScalar {
    var self_271: AntiScalar;
    var other_223: Scalar;

    self_271 = self_270;
    other_223 = other_222;
    let _e4: AntiScalar = self_271;
    let _e6: Scalar = other_223;
    return AntiScalar((_e4.g0_ * _e6.g0_));
}

fn anti_scalar_scalar_wedge(self_272: AntiScalar, other_224: Scalar) -> AntiScalar {
    var self_273: AntiScalar;
    var other_225: Scalar;

    self_273 = self_272;
    other_225 = other_224;
    let _e4: AntiScalar = self_273;
    let _e6: Scalar = other_225;
    return AntiScalar((_e4.g0_ * _e6.g0_));
}

fn anti_scalar_scalar_join(self_274: AntiScalar, other_226: Scalar) -> AntiScalar {
    var self_275: AntiScalar;
    var other_227: Scalar;

    self_275 = self_274;
    other_227 = other_226;
    let _e4: AntiScalar = self_275;
    let _e6: Scalar = other_227;
    return AntiScalar((_e4.g0_ * _e6.g0_));
}

fn anti_scalar_scalar_inner_product(self_276: AntiScalar, other_228: Scalar) -> AntiScalar {
    var self_277: AntiScalar;
    var other_229: Scalar;

    self_277 = self_276;
    other_229 = other_228;
    let _e4: AntiScalar = self_277;
    let _e6: Scalar = other_229;
    return AntiScalar((_e4.g0_ * _e6.g0_));
}

fn anti_scalar_scalar_geometric_anti_product(self_278: AntiScalar, other_230: Scalar) -> Scalar {
    var self_279: AntiScalar;
    var other_231: Scalar;

    self_279 = self_278;
    other_231 = other_230;
    let _e4: AntiScalar = self_279;
    let _e6: Scalar = other_231;
    return Scalar((_e4.g0_ * _e6.g0_));
}

fn anti_scalar_scalar_inner_anti_product(self_280: AntiScalar, other_232: Scalar) -> Scalar {
    var self_281: AntiScalar;
    var other_233: Scalar;

    self_281 = self_280;
    other_233 = other_232;
    let _e4: AntiScalar = self_281;
    let _e6: Scalar = other_233;
    return Scalar((_e4.g0_ * _e6.g0_));
}

fn anti_scalar_scalar_right_contraction(self_282: AntiScalar, other_234: Scalar) -> AntiScalar {
    var self_283: AntiScalar;
    var other_235: Scalar;

    self_283 = self_282;
    other_235 = other_234;
    let _e4: AntiScalar = self_283;
    let _e6: Scalar = other_235;
    return AntiScalar((_e4.g0_ * _e6.g0_));
}

fn anti_scalar_scalar_left_anti_contraction(self_284: AntiScalar, other_236: Scalar) -> Scalar {
    var self_285: AntiScalar;
    var other_237: Scalar;

    self_285 = self_284;
    other_237 = other_236;
    let _e4: AntiScalar = self_285;
    let _e6: Scalar = other_237;
    return Scalar((_e4.g0_ * _e6.g0_));
}

fn anti_scalar_anti_scalar_add(self_286: AntiScalar, other_238: AntiScalar) -> AntiScalar {
    var self_287: AntiScalar;
    var other_239: AntiScalar;

    self_287 = self_286;
    other_239 = other_238;
    let _e4: AntiScalar = self_287;
    let _e6: AntiScalar = other_239;
    return AntiScalar((_e4.g0_ + _e6.g0_));
}

fn anti_scalar_anti_scalar_sub(self_288: AntiScalar, other_240: AntiScalar) -> AntiScalar {
    var self_289: AntiScalar;
    var other_241: AntiScalar;

    self_289 = self_288;
    other_241 = other_240;
    let _e4: AntiScalar = self_289;
    let _e6: AntiScalar = other_241;
    return AntiScalar((_e4.g0_ - _e6.g0_));
}

fn anti_scalar_anti_scalar_mul(self_290: AntiScalar, other_242: AntiScalar) -> AntiScalar {
    var self_291: AntiScalar;
    var other_243: AntiScalar;

    self_291 = self_290;
    other_243 = other_242;
    let _e4: AntiScalar = self_291;
    let _e6: AntiScalar = other_243;
    return AntiScalar((_e4.g0_ * _e6.g0_));
}

fn anti_scalar_anti_scalar_div(self_292: AntiScalar, other_244: AntiScalar) -> AntiScalar {
    var self_293: AntiScalar;
    var other_245: AntiScalar;

    self_293 = self_292;
    other_245 = other_244;
    let _e4: AntiScalar = self_293;
    let _e8: AntiScalar = other_245;
    return AntiScalar((((_e4.g0_ * 1.0) / _e8.g0_) * 1.0));
}

fn anti_scalar_anti_scalar_regressive_product(self_294: AntiScalar, other_246: AntiScalar) -> AntiScalar {
    var self_295: AntiScalar;
    var other_247: AntiScalar;

    self_295 = self_294;
    other_247 = other_246;
    let _e4: AntiScalar = self_295;
    let _e6: AntiScalar = other_247;
    return AntiScalar((_e4.g0_ * _e6.g0_));
}

fn anti_scalar_anti_scalar_anti_wedge(self_296: AntiScalar, other_248: AntiScalar) -> AntiScalar {
    var self_297: AntiScalar;
    var other_249: AntiScalar;

    self_297 = self_296;
    other_249 = other_248;
    let _e4: AntiScalar = self_297;
    let _e6: AntiScalar = other_249;
    return AntiScalar((_e4.g0_ * _e6.g0_));
}

fn anti_scalar_anti_scalar_meet(self_298: AntiScalar, other_250: AntiScalar) -> AntiScalar {
    var self_299: AntiScalar;
    var other_251: AntiScalar;

    self_299 = self_298;
    other_251 = other_250;
    let _e4: AntiScalar = self_299;
    let _e6: AntiScalar = other_251;
    return AntiScalar((_e4.g0_ * _e6.g0_));
}

fn anti_scalar_anti_scalar_geometric_anti_product(self_300: AntiScalar, other_252: AntiScalar) -> AntiScalar {
    var self_301: AntiScalar;
    var other_253: AntiScalar;

    self_301 = self_300;
    other_253 = other_252;
    let _e4: AntiScalar = self_301;
    let _e6: AntiScalar = other_253;
    return AntiScalar((_e4.g0_ * _e6.g0_));
}

fn anti_scalar_anti_scalar_inner_anti_product(self_302: AntiScalar, other_254: AntiScalar) -> AntiScalar {
    var self_303: AntiScalar;
    var other_255: AntiScalar;

    self_303 = self_302;
    other_255 = other_254;
    let _e4: AntiScalar = self_303;
    let _e6: AntiScalar = other_255;
    return AntiScalar((_e4.g0_ * _e6.g0_));
}

fn anti_scalar_anti_scalar_left_anti_contraction(self_304: AntiScalar, other_256: AntiScalar) -> AntiScalar {
    var self_305: AntiScalar;
    var other_257: AntiScalar;

    self_305 = self_304;
    other_257 = other_256;
    let _e4: AntiScalar = self_305;
    let _e6: AntiScalar = other_257;
    return AntiScalar((_e4.g0_ * _e6.g0_));
}

fn anti_scalar_anti_scalar_right_anti_contraction(self_306: AntiScalar, other_258: AntiScalar) -> AntiScalar {
    var self_307: AntiScalar;
    var other_259: AntiScalar;

    self_307 = self_306;
    other_259 = other_258;
    let _e4: AntiScalar = self_307;
    let _e6: AntiScalar = other_259;
    return AntiScalar((_e4.g0_ * _e6.g0_));
}

fn anti_scalar_anti_scalar_anti_scalar_product(self_308: AntiScalar, other_260: AntiScalar) -> AntiScalar {
    var self_309: AntiScalar;
    var other_261: AntiScalar;

    self_309 = self_308;
    other_261 = other_260;
    let _e4: AntiScalar = self_309;
    let _e6: AntiScalar = other_261;
    return AntiScalar((_e4.g0_ * _e6.g0_));
}

fn anti_scalar_anti_scalar_anti_dot(self_310: AntiScalar, other_262: AntiScalar) -> AntiScalar {
    var self_311: AntiScalar;
    var other_263: AntiScalar;

    self_311 = self_310;
    other_263 = other_262;
    let _e4: AntiScalar = self_311;
    let _e6: AntiScalar = other_263;
    return AntiScalar((_e4.g0_ * _e6.g0_));
}

fn anti_scalar_homogeneous_magnitude_add(self_312: AntiScalar, other_264: HomogeneousMagnitude) -> HomogeneousMagnitude {
    var self_313: AntiScalar;
    var other_265: HomogeneousMagnitude;

    self_313 = self_312;
    other_265 = other_264;
    let _e4: AntiScalar = self_313;
    let _e11: HomogeneousMagnitude = other_265;
    return HomogeneousMagnitude(((vec2<f32>(_e4.g0_) * vec2<f32>(0.0, 1.0)) + _e11.g0_));
}

fn anti_scalar_homogeneous_magnitude_sub(self_314: AntiScalar, other_266: HomogeneousMagnitude) -> HomogeneousMagnitude {
    var self_315: AntiScalar;
    var other_267: HomogeneousMagnitude;

    self_315 = self_314;
    other_267 = other_266;
    let _e4: AntiScalar = self_315;
    let _e11: HomogeneousMagnitude = other_267;
    return HomogeneousMagnitude(((vec2<f32>(_e4.g0_) * vec2<f32>(0.0, 1.0)) - _e11.g0_));
}

fn anti_scalar_homogeneous_magnitude_geometric_product(self_316: AntiScalar, other_268: HomogeneousMagnitude) -> AntiScalar {
    var self_317: AntiScalar;
    var other_269: HomogeneousMagnitude;

    self_317 = self_316;
    other_269 = other_268;
    let _e4: AntiScalar = self_317;
    let _e6: HomogeneousMagnitude = other_269;
    return AntiScalar((_e4.g0_ * _e6.g0_.x));
}

fn anti_scalar_homogeneous_magnitude_regressive_product(self_318: AntiScalar, other_270: HomogeneousMagnitude) -> HomogeneousMagnitude {
    var self_319: AntiScalar;
    var other_271: HomogeneousMagnitude;

    self_319 = self_318;
    other_271 = other_270;
    let _e4: AntiScalar = self_319;
    let _e7: HomogeneousMagnitude = other_271;
    return HomogeneousMagnitude((vec2<f32>(_e4.g0_) * _e7.g0_));
}

fn anti_scalar_homogeneous_magnitude_anti_wedge(self_320: AntiScalar, other_272: HomogeneousMagnitude) -> HomogeneousMagnitude {
    var self_321: AntiScalar;
    var other_273: HomogeneousMagnitude;

    self_321 = self_320;
    other_273 = other_272;
    let _e4: AntiScalar = self_321;
    let _e7: HomogeneousMagnitude = other_273;
    return HomogeneousMagnitude((vec2<f32>(_e4.g0_) * _e7.g0_));
}

fn anti_scalar_homogeneous_magnitude_meet(self_322: AntiScalar, other_274: HomogeneousMagnitude) -> HomogeneousMagnitude {
    var self_323: AntiScalar;
    var other_275: HomogeneousMagnitude;

    self_323 = self_322;
    other_275 = other_274;
    let _e4: AntiScalar = self_323;
    let _e7: HomogeneousMagnitude = other_275;
    return HomogeneousMagnitude((vec2<f32>(_e4.g0_) * _e7.g0_));
}

fn anti_scalar_homogeneous_magnitude_outer_product(self_324: AntiScalar, other_276: HomogeneousMagnitude) -> AntiScalar {
    var self_325: AntiScalar;
    var other_277: HomogeneousMagnitude;

    self_325 = self_324;
    other_277 = other_276;
    let _e4: AntiScalar = self_325;
    let _e6: HomogeneousMagnitude = other_277;
    return AntiScalar((_e4.g0_ * _e6.g0_.x));
}

fn anti_scalar_homogeneous_magnitude_wedge(self_326: AntiScalar, other_278: HomogeneousMagnitude) -> AntiScalar {
    var self_327: AntiScalar;
    var other_279: HomogeneousMagnitude;

    self_327 = self_326;
    other_279 = other_278;
    let _e4: AntiScalar = self_327;
    let _e6: HomogeneousMagnitude = other_279;
    return AntiScalar((_e4.g0_ * _e6.g0_.x));
}

fn anti_scalar_homogeneous_magnitude_join(self_328: AntiScalar, other_280: HomogeneousMagnitude) -> AntiScalar {
    var self_329: AntiScalar;
    var other_281: HomogeneousMagnitude;

    self_329 = self_328;
    other_281 = other_280;
    let _e4: AntiScalar = self_329;
    let _e6: HomogeneousMagnitude = other_281;
    return AntiScalar((_e4.g0_ * _e6.g0_.x));
}

fn anti_scalar_homogeneous_magnitude_inner_product(self_330: AntiScalar, other_282: HomogeneousMagnitude) -> AntiScalar {
    var self_331: AntiScalar;
    var other_283: HomogeneousMagnitude;

    self_331 = self_330;
    other_283 = other_282;
    let _e4: AntiScalar = self_331;
    let _e6: HomogeneousMagnitude = other_283;
    return AntiScalar((_e4.g0_ * _e6.g0_.x));
}

fn anti_scalar_homogeneous_magnitude_geometric_anti_product(self_332: AntiScalar, other_284: HomogeneousMagnitude) -> HomogeneousMagnitude {
    var self_333: AntiScalar;
    var other_285: HomogeneousMagnitude;

    self_333 = self_332;
    other_285 = other_284;
    let _e4: AntiScalar = self_333;
    let _e7: HomogeneousMagnitude = other_285;
    return HomogeneousMagnitude((vec2<f32>(_e4.g0_) * _e7.g0_));
}

fn anti_scalar_homogeneous_magnitude_inner_anti_product(self_334: AntiScalar, other_286: HomogeneousMagnitude) -> HomogeneousMagnitude {
    var self_335: AntiScalar;
    var other_287: HomogeneousMagnitude;

    self_335 = self_334;
    other_287 = other_286;
    let _e4: AntiScalar = self_335;
    let _e7: HomogeneousMagnitude = other_287;
    return HomogeneousMagnitude((vec2<f32>(_e4.g0_) * _e7.g0_));
}

fn anti_scalar_homogeneous_magnitude_right_contraction(self_336: AntiScalar, other_288: HomogeneousMagnitude) -> AntiScalar {
    var self_337: AntiScalar;
    var other_289: HomogeneousMagnitude;

    self_337 = self_336;
    other_289 = other_288;
    let _e4: AntiScalar = self_337;
    let _e6: HomogeneousMagnitude = other_289;
    return AntiScalar((_e4.g0_ * _e6.g0_.x));
}

fn anti_scalar_homogeneous_magnitude_left_anti_contraction(self_338: AntiScalar, other_290: HomogeneousMagnitude) -> HomogeneousMagnitude {
    var self_339: AntiScalar;
    var other_291: HomogeneousMagnitude;

    self_339 = self_338;
    other_291 = other_290;
    let _e4: AntiScalar = self_339;
    let _e7: HomogeneousMagnitude = other_291;
    return HomogeneousMagnitude((vec2<f32>(_e4.g0_) * _e7.g0_));
}

fn anti_scalar_homogeneous_magnitude_right_anti_contraction(self_340: AntiScalar, other_292: HomogeneousMagnitude) -> AntiScalar {
    var self_341: AntiScalar;
    var other_293: HomogeneousMagnitude;

    self_341 = self_340;
    other_293 = other_292;
    let _e4: AntiScalar = self_341;
    let _e6: HomogeneousMagnitude = other_293;
    return AntiScalar((_e4.g0_ * _e6.g0_.y));
}

fn anti_scalar_homogeneous_magnitude_anti_scalar_product(self_342: AntiScalar, other_294: HomogeneousMagnitude) -> AntiScalar {
    var self_343: AntiScalar;
    var other_295: HomogeneousMagnitude;

    self_343 = self_342;
    other_295 = other_294;
    let _e4: AntiScalar = self_343;
    let _e6: HomogeneousMagnitude = other_295;
    return AntiScalar((_e4.g0_ * _e6.g0_.y));
}

fn anti_scalar_homogeneous_magnitude_anti_dot(self_344: AntiScalar, other_296: HomogeneousMagnitude) -> AntiScalar {
    var self_345: AntiScalar;
    var other_297: HomogeneousMagnitude;

    self_345 = self_344;
    other_297 = other_296;
    let _e4: AntiScalar = self_345;
    let _e6: HomogeneousMagnitude = other_297;
    return AntiScalar((_e4.g0_ * _e6.g0_.y));
}

fn anti_scalar_point_regressive_product(self_346: AntiScalar, other_298: Point) -> Point {
    var self_347: AntiScalar;
    var other_299: Point;

    self_347 = self_346;
    other_299 = other_298;
    let _e4: AntiScalar = self_347;
    let _e7: Point = other_299;
    return Point((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn anti_scalar_point_anti_wedge(self_348: AntiScalar, other_300: Point) -> Point {
    var self_349: AntiScalar;
    var other_301: Point;

    self_349 = self_348;
    other_301 = other_300;
    let _e4: AntiScalar = self_349;
    let _e7: Point = other_301;
    return Point((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn anti_scalar_point_meet(self_350: AntiScalar, other_302: Point) -> Point {
    var self_351: AntiScalar;
    var other_303: Point;

    self_351 = self_350;
    other_303 = other_302;
    let _e4: AntiScalar = self_351;
    let _e7: Point = other_303;
    return Point((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn anti_scalar_point_geometric_anti_product(self_352: AntiScalar, other_304: Point) -> Point {
    var self_353: AntiScalar;
    var other_305: Point;

    self_353 = self_352;
    other_305 = other_304;
    let _e4: AntiScalar = self_353;
    let _e7: Point = other_305;
    return Point((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn anti_scalar_point_inner_anti_product(self_354: AntiScalar, other_306: Point) -> Point {
    var self_355: AntiScalar;
    var other_307: Point;

    self_355 = self_354;
    other_307 = other_306;
    let _e4: AntiScalar = self_355;
    let _e7: Point = other_307;
    return Point((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn anti_scalar_point_left_anti_contraction(self_356: AntiScalar, other_308: Point) -> Point {
    var self_357: AntiScalar;
    var other_309: Point;

    self_357 = self_356;
    other_309 = other_308;
    let _e4: AntiScalar = self_357;
    let _e7: Point = other_309;
    return Point((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn anti_scalar_line_add(self_358: AntiScalar, other_310: Line) -> Motor {
    var self_359: AntiScalar;
    var other_311: Line;

    self_359 = self_358;
    other_311 = other_310;
    let _e4: AntiScalar = self_359;
    let _e13: Line = other_311;
    let _e16: Line = other_311;
    let _e19: Line = other_311;
    let _e22: Line = other_311;
    let _e33: Line = other_311;
    return Motor(((vec4<f32>(_e4.g0_) * vec4<f32>(0.0, 0.0, 0.0, 1.0)) + (vec4<f32>(_e13.g0_.x, _e16.g0_.y, _e19.g0_.z, _e22.g0_.x) * vec4<f32>(1.0, 1.0, 1.0, 0.0))), _e33.g1_);
}

fn anti_scalar_line_sub(self_360: AntiScalar, other_312: Line) -> Motor {
    var self_361: AntiScalar;
    var other_313: Line;

    self_361 = self_360;
    other_313 = other_312;
    let _e4: AntiScalar = self_361;
    let _e13: Line = other_313;
    let _e16: Line = other_313;
    let _e19: Line = other_313;
    let _e22: Line = other_313;
    let _e35: Line = other_313;
    return Motor(((vec4<f32>(_e4.g0_) * vec4<f32>(0.0, 0.0, 0.0, 1.0)) - (vec4<f32>(_e13.g0_.x, _e16.g0_.y, _e19.g0_.z, _e22.g0_.x) * vec4<f32>(1.0, 1.0, 1.0, 0.0))), (vec3<f32>(0.0) - _e35.g1_));
}

fn anti_scalar_line_regressive_product(self_362: AntiScalar, other_314: Line) -> Line {
    var self_363: AntiScalar;
    var other_315: Line;

    self_363 = self_362;
    other_315 = other_314;
    let _e4: AntiScalar = self_363;
    let _e7: Line = other_315;
    let _e10: AntiScalar = self_363;
    let _e13: Line = other_315;
    return Line((vec3<f32>(_e4.g0_) * _e7.g0_), (vec3<f32>(_e10.g0_) * _e13.g1_));
}

fn anti_scalar_line_anti_wedge(self_364: AntiScalar, other_316: Line) -> Line {
    var self_365: AntiScalar;
    var other_317: Line;

    self_365 = self_364;
    other_317 = other_316;
    let _e4: AntiScalar = self_365;
    let _e7: Line = other_317;
    let _e10: AntiScalar = self_365;
    let _e13: Line = other_317;
    return Line((vec3<f32>(_e4.g0_) * _e7.g0_), (vec3<f32>(_e10.g0_) * _e13.g1_));
}

fn anti_scalar_line_meet(self_366: AntiScalar, other_318: Line) -> Line {
    var self_367: AntiScalar;
    var other_319: Line;

    self_367 = self_366;
    other_319 = other_318;
    let _e4: AntiScalar = self_367;
    let _e7: Line = other_319;
    let _e10: AntiScalar = self_367;
    let _e13: Line = other_319;
    return Line((vec3<f32>(_e4.g0_) * _e7.g0_), (vec3<f32>(_e10.g0_) * _e13.g1_));
}

fn anti_scalar_line_geometric_anti_product(self_368: AntiScalar, other_320: Line) -> Line {
    var self_369: AntiScalar;
    var other_321: Line;

    self_369 = self_368;
    other_321 = other_320;
    let _e4: AntiScalar = self_369;
    let _e7: Line = other_321;
    let _e10: AntiScalar = self_369;
    let _e13: Line = other_321;
    return Line((vec3<f32>(_e4.g0_) * _e7.g0_), (vec3<f32>(_e10.g0_) * _e13.g1_));
}

fn anti_scalar_line_inner_anti_product(self_370: AntiScalar, other_322: Line) -> Line {
    var self_371: AntiScalar;
    var other_323: Line;

    self_371 = self_370;
    other_323 = other_322;
    let _e4: AntiScalar = self_371;
    let _e7: Line = other_323;
    let _e10: AntiScalar = self_371;
    let _e13: Line = other_323;
    return Line((vec3<f32>(_e4.g0_) * _e7.g0_), (vec3<f32>(_e10.g0_) * _e13.g1_));
}

fn anti_scalar_line_left_anti_contraction(self_372: AntiScalar, other_324: Line) -> Line {
    var self_373: AntiScalar;
    var other_325: Line;

    self_373 = self_372;
    other_325 = other_324;
    let _e4: AntiScalar = self_373;
    let _e7: Line = other_325;
    let _e10: AntiScalar = self_373;
    let _e13: Line = other_325;
    return Line((vec3<f32>(_e4.g0_) * _e7.g0_), (vec3<f32>(_e10.g0_) * _e13.g1_));
}

fn anti_scalar_plane_regressive_product(self_374: AntiScalar, other_326: Plane) -> Plane {
    var self_375: AntiScalar;
    var other_327: Plane;

    self_375 = self_374;
    other_327 = other_326;
    let _e4: AntiScalar = self_375;
    let _e7: Plane = other_327;
    return Plane((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn anti_scalar_plane_anti_wedge(self_376: AntiScalar, other_328: Plane) -> Plane {
    var self_377: AntiScalar;
    var other_329: Plane;

    self_377 = self_376;
    other_329 = other_328;
    let _e4: AntiScalar = self_377;
    let _e7: Plane = other_329;
    return Plane((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn anti_scalar_plane_meet(self_378: AntiScalar, other_330: Plane) -> Plane {
    var self_379: AntiScalar;
    var other_331: Plane;

    self_379 = self_378;
    other_331 = other_330;
    let _e4: AntiScalar = self_379;
    let _e7: Plane = other_331;
    return Plane((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn anti_scalar_plane_geometric_anti_product(self_380: AntiScalar, other_332: Plane) -> Plane {
    var self_381: AntiScalar;
    var other_333: Plane;

    self_381 = self_380;
    other_333 = other_332;
    let _e4: AntiScalar = self_381;
    let _e7: Plane = other_333;
    return Plane((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn anti_scalar_plane_inner_anti_product(self_382: AntiScalar, other_334: Plane) -> Plane {
    var self_383: AntiScalar;
    var other_335: Plane;

    self_383 = self_382;
    other_335 = other_334;
    let _e4: AntiScalar = self_383;
    let _e7: Plane = other_335;
    return Plane((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn anti_scalar_plane_left_anti_contraction(self_384: AntiScalar, other_336: Plane) -> Plane {
    var self_385: AntiScalar;
    var other_337: Plane;

    self_385 = self_384;
    other_337 = other_336;
    let _e4: AntiScalar = self_385;
    let _e7: Plane = other_337;
    return Plane((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn anti_scalar_motor_add(self_386: AntiScalar, other_338: Motor) -> Motor {
    var self_387: AntiScalar;
    var other_339: Motor;

    self_387 = self_386;
    other_339 = other_338;
    let _e4: AntiScalar = self_387;
    let _e13: Motor = other_339;
    let _e16: Motor = other_339;
    return Motor(((vec4<f32>(_e4.g0_) * vec4<f32>(0.0, 0.0, 0.0, 1.0)) + _e13.g0_), _e16.g1_);
}

fn anti_scalar_motor_sub(self_388: AntiScalar, other_340: Motor) -> Motor {
    var self_389: AntiScalar;
    var other_341: Motor;

    self_389 = self_388;
    other_341 = other_340;
    let _e4: AntiScalar = self_389;
    let _e13: Motor = other_341;
    let _e18: Motor = other_341;
    return Motor(((vec4<f32>(_e4.g0_) * vec4<f32>(0.0, 0.0, 0.0, 1.0)) - _e13.g0_), (vec3<f32>(0.0) - _e18.g1_));
}

fn anti_scalar_motor_regressive_product(self_390: AntiScalar, other_342: Motor) -> Motor {
    var self_391: AntiScalar;
    var other_343: Motor;

    self_391 = self_390;
    other_343 = other_342;
    let _e4: AntiScalar = self_391;
    let _e7: Motor = other_343;
    let _e10: AntiScalar = self_391;
    let _e13: Motor = other_343;
    return Motor((vec4<f32>(_e4.g0_) * _e7.g0_), (vec3<f32>(_e10.g0_) * _e13.g1_));
}

fn anti_scalar_motor_anti_wedge(self_392: AntiScalar, other_344: Motor) -> Motor {
    var self_393: AntiScalar;
    var other_345: Motor;

    self_393 = self_392;
    other_345 = other_344;
    let _e4: AntiScalar = self_393;
    let _e7: Motor = other_345;
    let _e10: AntiScalar = self_393;
    let _e13: Motor = other_345;
    return Motor((vec4<f32>(_e4.g0_) * _e7.g0_), (vec3<f32>(_e10.g0_) * _e13.g1_));
}

fn anti_scalar_motor_meet(self_394: AntiScalar, other_346: Motor) -> Motor {
    var self_395: AntiScalar;
    var other_347: Motor;

    self_395 = self_394;
    other_347 = other_346;
    let _e4: AntiScalar = self_395;
    let _e7: Motor = other_347;
    let _e10: AntiScalar = self_395;
    let _e13: Motor = other_347;
    return Motor((vec4<f32>(_e4.g0_) * _e7.g0_), (vec3<f32>(_e10.g0_) * _e13.g1_));
}

fn anti_scalar_motor_geometric_anti_product(self_396: AntiScalar, other_348: Motor) -> Motor {
    var self_397: AntiScalar;
    var other_349: Motor;

    self_397 = self_396;
    other_349 = other_348;
    let _e4: AntiScalar = self_397;
    let _e7: Motor = other_349;
    let _e10: AntiScalar = self_397;
    let _e13: Motor = other_349;
    return Motor((vec4<f32>(_e4.g0_) * _e7.g0_), (vec3<f32>(_e10.g0_) * _e13.g1_));
}

fn anti_scalar_motor_inner_anti_product(self_398: AntiScalar, other_350: Motor) -> Motor {
    var self_399: AntiScalar;
    var other_351: Motor;

    self_399 = self_398;
    other_351 = other_350;
    let _e4: AntiScalar = self_399;
    let _e7: Motor = other_351;
    let _e10: AntiScalar = self_399;
    let _e13: Motor = other_351;
    return Motor((vec4<f32>(_e4.g0_) * _e7.g0_), (vec3<f32>(_e10.g0_) * _e13.g1_));
}

fn anti_scalar_motor_left_anti_contraction(self_400: AntiScalar, other_352: Motor) -> Motor {
    var self_401: AntiScalar;
    var other_353: Motor;

    self_401 = self_400;
    other_353 = other_352;
    let _e4: AntiScalar = self_401;
    let _e7: Motor = other_353;
    let _e10: AntiScalar = self_401;
    let _e13: Motor = other_353;
    return Motor((vec4<f32>(_e4.g0_) * _e7.g0_), (vec3<f32>(_e10.g0_) * _e13.g1_));
}

fn anti_scalar_motor_right_anti_contraction(self_402: AntiScalar, other_354: Motor) -> AntiScalar {
    var self_403: AntiScalar;
    var other_355: Motor;

    self_403 = self_402;
    other_355 = other_354;
    let _e4: AntiScalar = self_403;
    let _e6: Motor = other_355;
    return AntiScalar((_e4.g0_ * _e6.g0_.w));
}

fn anti_scalar_motor_anti_scalar_product(self_404: AntiScalar, other_356: Motor) -> AntiScalar {
    var self_405: AntiScalar;
    var other_357: Motor;

    self_405 = self_404;
    other_357 = other_356;
    let _e4: AntiScalar = self_405;
    let _e6: Motor = other_357;
    return AntiScalar((_e4.g0_ * _e6.g0_.w));
}

fn anti_scalar_motor_anti_dot(self_406: AntiScalar, other_358: Motor) -> AntiScalar {
    var self_407: AntiScalar;
    var other_359: Motor;

    self_407 = self_406;
    other_359 = other_358;
    let _e4: AntiScalar = self_407;
    let _e6: Motor = other_359;
    return AntiScalar((_e4.g0_ * _e6.g0_.w));
}

fn anti_scalar_rotor_add(self_408: AntiScalar, other_360: Rotor) -> Rotor {
    var self_409: AntiScalar;
    var other_361: Rotor;

    self_409 = self_408;
    other_361 = other_360;
    let _e4: AntiScalar = self_409;
    let _e13: Rotor = other_361;
    return Rotor(((vec4<f32>(_e4.g0_) * vec4<f32>(0.0, 0.0, 0.0, 1.0)) + _e13.g0_));
}

fn anti_scalar_rotor_sub(self_410: AntiScalar, other_362: Rotor) -> Rotor {
    var self_411: AntiScalar;
    var other_363: Rotor;

    self_411 = self_410;
    other_363 = other_362;
    let _e4: AntiScalar = self_411;
    let _e13: Rotor = other_363;
    return Rotor(((vec4<f32>(_e4.g0_) * vec4<f32>(0.0, 0.0, 0.0, 1.0)) - _e13.g0_));
}

fn anti_scalar_rotor_regressive_product(self_412: AntiScalar, other_364: Rotor) -> Rotor {
    var self_413: AntiScalar;
    var other_365: Rotor;

    self_413 = self_412;
    other_365 = other_364;
    let _e4: AntiScalar = self_413;
    let _e7: Rotor = other_365;
    return Rotor((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn anti_scalar_rotor_anti_wedge(self_414: AntiScalar, other_366: Rotor) -> Rotor {
    var self_415: AntiScalar;
    var other_367: Rotor;

    self_415 = self_414;
    other_367 = other_366;
    let _e4: AntiScalar = self_415;
    let _e7: Rotor = other_367;
    return Rotor((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn anti_scalar_rotor_meet(self_416: AntiScalar, other_368: Rotor) -> Rotor {
    var self_417: AntiScalar;
    var other_369: Rotor;

    self_417 = self_416;
    other_369 = other_368;
    let _e4: AntiScalar = self_417;
    let _e7: Rotor = other_369;
    return Rotor((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn anti_scalar_rotor_geometric_anti_product(self_418: AntiScalar, other_370: Rotor) -> Rotor {
    var self_419: AntiScalar;
    var other_371: Rotor;

    self_419 = self_418;
    other_371 = other_370;
    let _e4: AntiScalar = self_419;
    let _e7: Rotor = other_371;
    return Rotor((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn anti_scalar_rotor_inner_anti_product(self_420: AntiScalar, other_372: Rotor) -> Rotor {
    var self_421: AntiScalar;
    var other_373: Rotor;

    self_421 = self_420;
    other_373 = other_372;
    let _e4: AntiScalar = self_421;
    let _e7: Rotor = other_373;
    return Rotor((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn anti_scalar_rotor_left_anti_contraction(self_422: AntiScalar, other_374: Rotor) -> Rotor {
    var self_423: AntiScalar;
    var other_375: Rotor;

    self_423 = self_422;
    other_375 = other_374;
    let _e4: AntiScalar = self_423;
    let _e7: Rotor = other_375;
    return Rotor((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn anti_scalar_rotor_right_anti_contraction(self_424: AntiScalar, other_376: Rotor) -> AntiScalar {
    var self_425: AntiScalar;
    var other_377: Rotor;

    self_425 = self_424;
    other_377 = other_376;
    let _e4: AntiScalar = self_425;
    let _e6: Rotor = other_377;
    return AntiScalar((_e4.g0_ * _e6.g0_.w));
}

fn anti_scalar_rotor_anti_scalar_product(self_426: AntiScalar, other_378: Rotor) -> AntiScalar {
    var self_427: AntiScalar;
    var other_379: Rotor;

    self_427 = self_426;
    other_379 = other_378;
    let _e4: AntiScalar = self_427;
    let _e6: Rotor = other_379;
    return AntiScalar((_e4.g0_ * _e6.g0_.w));
}

fn anti_scalar_rotor_anti_dot(self_428: AntiScalar, other_380: Rotor) -> AntiScalar {
    var self_429: AntiScalar;
    var other_381: Rotor;

    self_429 = self_428;
    other_381 = other_380;
    let _e4: AntiScalar = self_429;
    let _e6: Rotor = other_381;
    return AntiScalar((_e4.g0_ * _e6.g0_.w));
}

fn anti_scalar_translator_add(self_430: AntiScalar, other_382: Translator) -> Translator {
    var self_431: AntiScalar;
    var other_383: Translator;

    self_431 = self_430;
    other_383 = other_382;
    let _e4: AntiScalar = self_431;
    let _e13: Translator = other_383;
    return Translator(((vec4<f32>(_e4.g0_) * vec4<f32>(0.0, 0.0, 0.0, 1.0)) + _e13.g0_));
}

fn anti_scalar_translator_sub(self_432: AntiScalar, other_384: Translator) -> Translator {
    var self_433: AntiScalar;
    var other_385: Translator;

    self_433 = self_432;
    other_385 = other_384;
    let _e4: AntiScalar = self_433;
    let _e13: Translator = other_385;
    return Translator(((vec4<f32>(_e4.g0_) * vec4<f32>(0.0, 0.0, 0.0, 1.0)) - _e13.g0_));
}

fn anti_scalar_translator_regressive_product(self_434: AntiScalar, other_386: Translator) -> Translator {
    var self_435: AntiScalar;
    var other_387: Translator;

    self_435 = self_434;
    other_387 = other_386;
    let _e4: AntiScalar = self_435;
    let _e7: Translator = other_387;
    return Translator((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn anti_scalar_translator_anti_wedge(self_436: AntiScalar, other_388: Translator) -> Translator {
    var self_437: AntiScalar;
    var other_389: Translator;

    self_437 = self_436;
    other_389 = other_388;
    let _e4: AntiScalar = self_437;
    let _e7: Translator = other_389;
    return Translator((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn anti_scalar_translator_meet(self_438: AntiScalar, other_390: Translator) -> Translator {
    var self_439: AntiScalar;
    var other_391: Translator;

    self_439 = self_438;
    other_391 = other_390;
    let _e4: AntiScalar = self_439;
    let _e7: Translator = other_391;
    return Translator((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn anti_scalar_translator_geometric_anti_product(self_440: AntiScalar, other_392: Translator) -> Translator {
    var self_441: AntiScalar;
    var other_393: Translator;

    self_441 = self_440;
    other_393 = other_392;
    let _e4: AntiScalar = self_441;
    let _e7: Translator = other_393;
    return Translator((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn anti_scalar_translator_inner_anti_product(self_442: AntiScalar, other_394: Translator) -> Translator {
    var self_443: AntiScalar;
    var other_395: Translator;

    self_443 = self_442;
    other_395 = other_394;
    let _e4: AntiScalar = self_443;
    let _e7: Translator = other_395;
    return Translator((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn anti_scalar_translator_left_anti_contraction(self_444: AntiScalar, other_396: Translator) -> Translator {
    var self_445: AntiScalar;
    var other_397: Translator;

    self_445 = self_444;
    other_397 = other_396;
    let _e4: AntiScalar = self_445;
    let _e7: Translator = other_397;
    return Translator((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn anti_scalar_translator_right_anti_contraction(self_446: AntiScalar, other_398: Translator) -> AntiScalar {
    var self_447: AntiScalar;
    var other_399: Translator;

    self_447 = self_446;
    other_399 = other_398;
    let _e4: AntiScalar = self_447;
    let _e6: Translator = other_399;
    return AntiScalar((_e4.g0_ * _e6.g0_.w));
}

fn anti_scalar_translator_anti_scalar_product(self_448: AntiScalar, other_400: Translator) -> AntiScalar {
    var self_449: AntiScalar;
    var other_401: Translator;

    self_449 = self_448;
    other_401 = other_400;
    let _e4: AntiScalar = self_449;
    let _e6: Translator = other_401;
    return AntiScalar((_e4.g0_ * _e6.g0_.w));
}

fn anti_scalar_translator_anti_dot(self_450: AntiScalar, other_402: Translator) -> AntiScalar {
    var self_451: AntiScalar;
    var other_403: Translator;

    self_451 = self_450;
    other_403 = other_402;
    let _e4: AntiScalar = self_451;
    let _e6: Translator = other_403;
    return AntiScalar((_e4.g0_ * _e6.g0_.w));
}

fn anti_scalar_flector_regressive_product(self_452: AntiScalar, other_404: Flector) -> Flector {
    var self_453: AntiScalar;
    var other_405: Flector;

    self_453 = self_452;
    other_405 = other_404;
    let _e4: AntiScalar = self_453;
    let _e7: Flector = other_405;
    let _e10: AntiScalar = self_453;
    let _e13: Flector = other_405;
    return Flector((vec4<f32>(_e4.g0_) * _e7.g0_), (vec4<f32>(_e10.g0_) * _e13.g1_));
}

fn anti_scalar_flector_anti_wedge(self_454: AntiScalar, other_406: Flector) -> Flector {
    var self_455: AntiScalar;
    var other_407: Flector;

    self_455 = self_454;
    other_407 = other_406;
    let _e4: AntiScalar = self_455;
    let _e7: Flector = other_407;
    let _e10: AntiScalar = self_455;
    let _e13: Flector = other_407;
    return Flector((vec4<f32>(_e4.g0_) * _e7.g0_), (vec4<f32>(_e10.g0_) * _e13.g1_));
}

fn anti_scalar_flector_meet(self_456: AntiScalar, other_408: Flector) -> Flector {
    var self_457: AntiScalar;
    var other_409: Flector;

    self_457 = self_456;
    other_409 = other_408;
    let _e4: AntiScalar = self_457;
    let _e7: Flector = other_409;
    let _e10: AntiScalar = self_457;
    let _e13: Flector = other_409;
    return Flector((vec4<f32>(_e4.g0_) * _e7.g0_), (vec4<f32>(_e10.g0_) * _e13.g1_));
}

fn anti_scalar_flector_geometric_anti_product(self_458: AntiScalar, other_410: Flector) -> Flector {
    var self_459: AntiScalar;
    var other_411: Flector;

    self_459 = self_458;
    other_411 = other_410;
    let _e4: AntiScalar = self_459;
    let _e7: Flector = other_411;
    let _e10: AntiScalar = self_459;
    let _e13: Flector = other_411;
    return Flector((vec4<f32>(_e4.g0_) * _e7.g0_), (vec4<f32>(_e10.g0_) * _e13.g1_));
}

fn anti_scalar_flector_inner_anti_product(self_460: AntiScalar, other_412: Flector) -> Flector {
    var self_461: AntiScalar;
    var other_413: Flector;

    self_461 = self_460;
    other_413 = other_412;
    let _e4: AntiScalar = self_461;
    let _e7: Flector = other_413;
    let _e10: AntiScalar = self_461;
    let _e13: Flector = other_413;
    return Flector((vec4<f32>(_e4.g0_) * _e7.g0_), (vec4<f32>(_e10.g0_) * _e13.g1_));
}

fn anti_scalar_flector_left_anti_contraction(self_462: AntiScalar, other_414: Flector) -> Flector {
    var self_463: AntiScalar;
    var other_415: Flector;

    self_463 = self_462;
    other_415 = other_414;
    let _e4: AntiScalar = self_463;
    let _e7: Flector = other_415;
    let _e10: AntiScalar = self_463;
    let _e13: Flector = other_415;
    return Flector((vec4<f32>(_e4.g0_) * _e7.g0_), (vec4<f32>(_e10.g0_) * _e13.g1_));
}

fn anti_scalar_multi_vector_outer_product(self_464: AntiScalar, other_416: MultiVector) -> AntiScalar {
    var self_465: AntiScalar;
    var other_417: MultiVector;

    self_465 = self_464;
    other_417 = other_416;
    let _e4: AntiScalar = self_465;
    let _e6: MultiVector = other_417;
    return AntiScalar((_e4.g0_ * _e6.g0_.x));
}

fn anti_scalar_multi_vector_wedge(self_466: AntiScalar, other_418: MultiVector) -> AntiScalar {
    var self_467: AntiScalar;
    var other_419: MultiVector;

    self_467 = self_466;
    other_419 = other_418;
    let _e4: AntiScalar = self_467;
    let _e6: MultiVector = other_419;
    return AntiScalar((_e4.g0_ * _e6.g0_.x));
}

fn anti_scalar_multi_vector_join(self_468: AntiScalar, other_420: MultiVector) -> AntiScalar {
    var self_469: AntiScalar;
    var other_421: MultiVector;

    self_469 = self_468;
    other_421 = other_420;
    let _e4: AntiScalar = self_469;
    let _e6: MultiVector = other_421;
    return AntiScalar((_e4.g0_ * _e6.g0_.x));
}

fn anti_scalar_multi_vector_right_anti_contraction(self_470: AntiScalar, other_422: MultiVector) -> AntiScalar {
    var self_471: AntiScalar;
    var other_423: MultiVector;

    self_471 = self_470;
    other_423 = other_422;
    let _e4: AntiScalar = self_471;
    let _e6: MultiVector = other_423;
    return AntiScalar((_e4.g0_ * _e6.g0_.y));
}

fn anti_scalar_multi_vector_anti_scalar_product(self_472: AntiScalar, other_424: MultiVector) -> AntiScalar {
    var self_473: AntiScalar;
    var other_425: MultiVector;

    self_473 = self_472;
    other_425 = other_424;
    let _e4: AntiScalar = self_473;
    let _e6: MultiVector = other_425;
    return AntiScalar((_e4.g0_ * _e6.g0_.y));
}

fn anti_scalar_multi_vector_anti_dot(self_474: AntiScalar, other_426: MultiVector) -> AntiScalar {
    var self_475: AntiScalar;
    var other_427: MultiVector;

    self_475 = self_474;
    other_427 = other_426;
    let _e4: AntiScalar = self_475;
    let _e6: MultiVector = other_427;
    return AntiScalar((_e4.g0_ * _e6.g0_.y));
}

fn anti_scalar_scale(self_476: AntiScalar, other_428: f32) -> AntiScalar {
    var self_477: AntiScalar;
    var other_429: f32;

    self_477 = self_476;
    other_429 = other_428;
    let _e4: AntiScalar = self_477;
    let _e5: f32 = other_429;
    let _e7: AntiScalar = anti_scalar_scalar_geometric_product(_e4, Scalar(_e5));
    return _e7;
}

fn anti_scalar_attitude(self_478: AntiScalar) -> Plane {
    var self_479: AntiScalar;

    self_479 = self_478;
    let _e2: AntiScalar = self_479;
    let _e9: Plane = anti_scalar_plane_regressive_product(_e2, Plane(vec4<f32>(0.0, 0.0, 0.0, 1.0)));
    return _e9;
}

fn homogeneous_magnitude_zero() -> HomogeneousMagnitude {
    return HomogeneousMagnitude(vec2<f32>(0.0));
}

fn homogeneous_magnitude_one() -> HomogeneousMagnitude {
    return HomogeneousMagnitude(vec2<f32>(1.0, 0.0));
}

fn homogeneous_magnitude_neg(self_480: HomogeneousMagnitude) -> HomogeneousMagnitude {
    var self_481: HomogeneousMagnitude;

    self_481 = self_480;
    let _e2: HomogeneousMagnitude = self_481;
    return HomogeneousMagnitude((_e2.g0_ * vec2<f32>(-(1.0))));
}

fn homogeneous_magnitude_automorphism(self_482: HomogeneousMagnitude) -> HomogeneousMagnitude {
    var self_483: HomogeneousMagnitude;

    self_483 = self_482;
    let _e2: HomogeneousMagnitude = self_483;
    return HomogeneousMagnitude(_e2.g0_);
}

fn homogeneous_magnitude_reversal(self_484: HomogeneousMagnitude) -> HomogeneousMagnitude {
    var self_485: HomogeneousMagnitude;

    self_485 = self_484;
    let _e2: HomogeneousMagnitude = self_485;
    return HomogeneousMagnitude(_e2.g0_);
}

fn homogeneous_magnitude_conjugation(self_486: HomogeneousMagnitude) -> HomogeneousMagnitude {
    var self_487: HomogeneousMagnitude;

    self_487 = self_486;
    let _e2: HomogeneousMagnitude = self_487;
    return HomogeneousMagnitude(_e2.g0_);
}

fn homogeneous_magnitude_dual(self_488: HomogeneousMagnitude) -> HomogeneousMagnitude {
    var self_489: HomogeneousMagnitude;

    self_489 = self_488;
    let _e2: HomogeneousMagnitude = self_489;
    return HomogeneousMagnitude(_e2.g0_.yx);
}

fn homogeneous_magnitude_anti_reversal(self_490: HomogeneousMagnitude) -> HomogeneousMagnitude {
    var self_491: HomogeneousMagnitude;

    self_491 = self_490;
    let _e2: HomogeneousMagnitude = self_491;
    return HomogeneousMagnitude(_e2.g0_);
}

fn homogeneous_magnitude_right_complement(self_492: HomogeneousMagnitude) -> HomogeneousMagnitude {
    var self_493: HomogeneousMagnitude;

    self_493 = self_492;
    let _e2: HomogeneousMagnitude = self_493;
    return HomogeneousMagnitude(_e2.g0_.yx);
}

fn homogeneous_magnitude_left_complement(self_494: HomogeneousMagnitude) -> HomogeneousMagnitude {
    var self_495: HomogeneousMagnitude;

    self_495 = self_494;
    let _e2: HomogeneousMagnitude = self_495;
    return HomogeneousMagnitude(_e2.g0_.yx);
}

fn homogeneous_magnitude_double_complement(self_496: HomogeneousMagnitude) -> HomogeneousMagnitude {
    var self_497: HomogeneousMagnitude;

    self_497 = self_496;
    let _e2: HomogeneousMagnitude = self_497;
    return HomogeneousMagnitude(_e2.g0_);
}

fn homogeneous_magnitude_scalar_into(self_498: HomogeneousMagnitude) -> Scalar {
    var self_499: HomogeneousMagnitude;

    self_499 = self_498;
    let _e2: HomogeneousMagnitude = self_499;
    return Scalar(_e2.g0_.x);
}

fn homogeneous_magnitude_scalar_add(self_500: HomogeneousMagnitude, other_430: Scalar) -> HomogeneousMagnitude {
    var self_501: HomogeneousMagnitude;
    var other_431: Scalar;

    self_501 = self_500;
    other_431 = other_430;
    let _e4: HomogeneousMagnitude = self_501;
    let _e6: Scalar = other_431;
    return HomogeneousMagnitude((_e4.g0_ + (vec2<f32>(_e6.g0_) * vec2<f32>(1.0, 0.0))));
}

fn homogeneous_magnitude_scalar_sub(self_502: HomogeneousMagnitude, other_432: Scalar) -> HomogeneousMagnitude {
    var self_503: HomogeneousMagnitude;
    var other_433: Scalar;

    self_503 = self_502;
    other_433 = other_432;
    let _e4: HomogeneousMagnitude = self_503;
    let _e6: Scalar = other_433;
    return HomogeneousMagnitude((_e4.g0_ - (vec2<f32>(_e6.g0_) * vec2<f32>(1.0, 0.0))));
}

fn homogeneous_magnitude_scalar_geometric_product(self_504: HomogeneousMagnitude, other_434: Scalar) -> HomogeneousMagnitude {
    var self_505: HomogeneousMagnitude;
    var other_435: Scalar;

    self_505 = self_504;
    other_435 = other_434;
    let _e4: HomogeneousMagnitude = self_505;
    let _e6: Scalar = other_435;
    return HomogeneousMagnitude((_e4.g0_ * vec2<f32>(_e6.g0_)));
}

fn homogeneous_magnitude_scalar_regressive_product(self_506: HomogeneousMagnitude, other_436: Scalar) -> Scalar {
    var self_507: HomogeneousMagnitude;
    var other_437: Scalar;

    self_507 = self_506;
    other_437 = other_436;
    let _e4: HomogeneousMagnitude = self_507;
    let _e7: Scalar = other_437;
    return Scalar((_e4.g0_.y * _e7.g0_));
}

fn homogeneous_magnitude_scalar_anti_wedge(self_508: HomogeneousMagnitude, other_438: Scalar) -> Scalar {
    var self_509: HomogeneousMagnitude;
    var other_439: Scalar;

    self_509 = self_508;
    other_439 = other_438;
    let _e4: HomogeneousMagnitude = self_509;
    let _e7: Scalar = other_439;
    return Scalar((_e4.g0_.y * _e7.g0_));
}

fn homogeneous_magnitude_scalar_meet(self_510: HomogeneousMagnitude, other_440: Scalar) -> Scalar {
    var self_511: HomogeneousMagnitude;
    var other_441: Scalar;

    self_511 = self_510;
    other_441 = other_440;
    let _e4: HomogeneousMagnitude = self_511;
    let _e7: Scalar = other_441;
    return Scalar((_e4.g0_.y * _e7.g0_));
}

fn homogeneous_magnitude_scalar_outer_product(self_512: HomogeneousMagnitude, other_442: Scalar) -> HomogeneousMagnitude {
    var self_513: HomogeneousMagnitude;
    var other_443: Scalar;

    self_513 = self_512;
    other_443 = other_442;
    let _e4: HomogeneousMagnitude = self_513;
    let _e6: Scalar = other_443;
    return HomogeneousMagnitude((_e4.g0_ * vec2<f32>(_e6.g0_)));
}

fn homogeneous_magnitude_scalar_wedge(self_514: HomogeneousMagnitude, other_444: Scalar) -> HomogeneousMagnitude {
    var self_515: HomogeneousMagnitude;
    var other_445: Scalar;

    self_515 = self_514;
    other_445 = other_444;
    let _e4: HomogeneousMagnitude = self_515;
    let _e6: Scalar = other_445;
    return HomogeneousMagnitude((_e4.g0_ * vec2<f32>(_e6.g0_)));
}

fn homogeneous_magnitude_scalar_join(self_516: HomogeneousMagnitude, other_446: Scalar) -> HomogeneousMagnitude {
    var self_517: HomogeneousMagnitude;
    var other_447: Scalar;

    self_517 = self_516;
    other_447 = other_446;
    let _e4: HomogeneousMagnitude = self_517;
    let _e6: Scalar = other_447;
    return HomogeneousMagnitude((_e4.g0_ * vec2<f32>(_e6.g0_)));
}

fn homogeneous_magnitude_scalar_inner_product(self_518: HomogeneousMagnitude, other_448: Scalar) -> HomogeneousMagnitude {
    var self_519: HomogeneousMagnitude;
    var other_449: Scalar;

    self_519 = self_518;
    other_449 = other_448;
    let _e4: HomogeneousMagnitude = self_519;
    let _e6: Scalar = other_449;
    return HomogeneousMagnitude((_e4.g0_ * vec2<f32>(_e6.g0_)));
}

fn homogeneous_magnitude_scalar_geometric_anti_product(self_520: HomogeneousMagnitude, other_450: Scalar) -> Scalar {
    var self_521: HomogeneousMagnitude;
    var other_451: Scalar;

    self_521 = self_520;
    other_451 = other_450;
    let _e4: HomogeneousMagnitude = self_521;
    let _e7: Scalar = other_451;
    return Scalar((_e4.g0_.y * _e7.g0_));
}

fn homogeneous_magnitude_scalar_inner_anti_product(self_522: HomogeneousMagnitude, other_452: Scalar) -> Scalar {
    var self_523: HomogeneousMagnitude;
    var other_453: Scalar;

    self_523 = self_522;
    other_453 = other_452;
    let _e4: HomogeneousMagnitude = self_523;
    let _e7: Scalar = other_453;
    return Scalar((_e4.g0_.y * _e7.g0_));
}

fn homogeneous_magnitude_scalar_left_contraction(self_524: HomogeneousMagnitude, other_454: Scalar) -> Scalar {
    var self_525: HomogeneousMagnitude;
    var other_455: Scalar;

    self_525 = self_524;
    other_455 = other_454;
    let _e4: HomogeneousMagnitude = self_525;
    let _e7: Scalar = other_455;
    return Scalar((_e4.g0_.x * _e7.g0_));
}

fn homogeneous_magnitude_scalar_right_contraction(self_526: HomogeneousMagnitude, other_456: Scalar) -> HomogeneousMagnitude {
    var self_527: HomogeneousMagnitude;
    var other_457: Scalar;

    self_527 = self_526;
    other_457 = other_456;
    let _e4: HomogeneousMagnitude = self_527;
    let _e6: Scalar = other_457;
    return HomogeneousMagnitude((_e4.g0_ * vec2<f32>(_e6.g0_)));
}

fn homogeneous_magnitude_scalar_left_anti_contraction(self_528: HomogeneousMagnitude, other_458: Scalar) -> Scalar {
    var self_529: HomogeneousMagnitude;
    var other_459: Scalar;

    self_529 = self_528;
    other_459 = other_458;
    let _e4: HomogeneousMagnitude = self_529;
    let _e7: Scalar = other_459;
    return Scalar((_e4.g0_.y * _e7.g0_));
}

fn homogeneous_magnitude_scalar_scalar_product(self_530: HomogeneousMagnitude, other_460: Scalar) -> Scalar {
    var self_531: HomogeneousMagnitude;
    var other_461: Scalar;

    self_531 = self_530;
    other_461 = other_460;
    let _e4: HomogeneousMagnitude = self_531;
    let _e7: Scalar = other_461;
    return Scalar((_e4.g0_.x * _e7.g0_));
}

fn homogeneous_magnitude_scalar_dot(self_532: HomogeneousMagnitude, other_462: Scalar) -> Scalar {
    var self_533: HomogeneousMagnitude;
    var other_463: Scalar;

    self_533 = self_532;
    other_463 = other_462;
    let _e4: HomogeneousMagnitude = self_533;
    let _e7: Scalar = other_463;
    return Scalar((_e4.g0_.x * _e7.g0_));
}

fn homogeneous_magnitude_anti_scalar_into(self_534: HomogeneousMagnitude) -> AntiScalar {
    var self_535: HomogeneousMagnitude;

    self_535 = self_534;
    let _e2: HomogeneousMagnitude = self_535;
    return AntiScalar(_e2.g0_.y);
}

fn homogeneous_magnitude_anti_scalar_add(self_536: HomogeneousMagnitude, other_464: AntiScalar) -> HomogeneousMagnitude {
    var self_537: HomogeneousMagnitude;
    var other_465: AntiScalar;

    self_537 = self_536;
    other_465 = other_464;
    let _e4: HomogeneousMagnitude = self_537;
    let _e6: AntiScalar = other_465;
    return HomogeneousMagnitude((_e4.g0_ + (vec2<f32>(_e6.g0_) * vec2<f32>(0.0, 1.0))));
}

fn homogeneous_magnitude_anti_scalar_sub(self_538: HomogeneousMagnitude, other_466: AntiScalar) -> HomogeneousMagnitude {
    var self_539: HomogeneousMagnitude;
    var other_467: AntiScalar;

    self_539 = self_538;
    other_467 = other_466;
    let _e4: HomogeneousMagnitude = self_539;
    let _e6: AntiScalar = other_467;
    return HomogeneousMagnitude((_e4.g0_ - (vec2<f32>(_e6.g0_) * vec2<f32>(0.0, 1.0))));
}

fn homogeneous_magnitude_anti_scalar_geometric_product(self_540: HomogeneousMagnitude, other_468: AntiScalar) -> AntiScalar {
    var self_541: HomogeneousMagnitude;
    var other_469: AntiScalar;

    self_541 = self_540;
    other_469 = other_468;
    let _e4: HomogeneousMagnitude = self_541;
    let _e7: AntiScalar = other_469;
    return AntiScalar((_e4.g0_.x * _e7.g0_));
}

fn homogeneous_magnitude_anti_scalar_regressive_product(self_542: HomogeneousMagnitude, other_470: AntiScalar) -> HomogeneousMagnitude {
    var self_543: HomogeneousMagnitude;
    var other_471: AntiScalar;

    self_543 = self_542;
    other_471 = other_470;
    let _e4: HomogeneousMagnitude = self_543;
    let _e6: AntiScalar = other_471;
    return HomogeneousMagnitude((_e4.g0_ * vec2<f32>(_e6.g0_)));
}

fn homogeneous_magnitude_anti_scalar_anti_wedge(self_544: HomogeneousMagnitude, other_472: AntiScalar) -> HomogeneousMagnitude {
    var self_545: HomogeneousMagnitude;
    var other_473: AntiScalar;

    self_545 = self_544;
    other_473 = other_472;
    let _e4: HomogeneousMagnitude = self_545;
    let _e6: AntiScalar = other_473;
    return HomogeneousMagnitude((_e4.g0_ * vec2<f32>(_e6.g0_)));
}

fn homogeneous_magnitude_anti_scalar_meet(self_546: HomogeneousMagnitude, other_474: AntiScalar) -> HomogeneousMagnitude {
    var self_547: HomogeneousMagnitude;
    var other_475: AntiScalar;

    self_547 = self_546;
    other_475 = other_474;
    let _e4: HomogeneousMagnitude = self_547;
    let _e6: AntiScalar = other_475;
    return HomogeneousMagnitude((_e4.g0_ * vec2<f32>(_e6.g0_)));
}

fn homogeneous_magnitude_anti_scalar_outer_product(self_548: HomogeneousMagnitude, other_476: AntiScalar) -> AntiScalar {
    var self_549: HomogeneousMagnitude;
    var other_477: AntiScalar;

    self_549 = self_548;
    other_477 = other_476;
    let _e4: HomogeneousMagnitude = self_549;
    let _e7: AntiScalar = other_477;
    return AntiScalar((_e4.g0_.x * _e7.g0_));
}

fn homogeneous_magnitude_anti_scalar_wedge(self_550: HomogeneousMagnitude, other_478: AntiScalar) -> AntiScalar {
    var self_551: HomogeneousMagnitude;
    var other_479: AntiScalar;

    self_551 = self_550;
    other_479 = other_478;
    let _e4: HomogeneousMagnitude = self_551;
    let _e7: AntiScalar = other_479;
    return AntiScalar((_e4.g0_.x * _e7.g0_));
}

fn homogeneous_magnitude_anti_scalar_join(self_552: HomogeneousMagnitude, other_480: AntiScalar) -> AntiScalar {
    var self_553: HomogeneousMagnitude;
    var other_481: AntiScalar;

    self_553 = self_552;
    other_481 = other_480;
    let _e4: HomogeneousMagnitude = self_553;
    let _e7: AntiScalar = other_481;
    return AntiScalar((_e4.g0_.x * _e7.g0_));
}

fn homogeneous_magnitude_anti_scalar_inner_product(self_554: HomogeneousMagnitude, other_482: AntiScalar) -> AntiScalar {
    var self_555: HomogeneousMagnitude;
    var other_483: AntiScalar;

    self_555 = self_554;
    other_483 = other_482;
    let _e4: HomogeneousMagnitude = self_555;
    let _e7: AntiScalar = other_483;
    return AntiScalar((_e4.g0_.x * _e7.g0_));
}

fn homogeneous_magnitude_anti_scalar_geometric_anti_product(self_556: HomogeneousMagnitude, other_484: AntiScalar) -> HomogeneousMagnitude {
    var self_557: HomogeneousMagnitude;
    var other_485: AntiScalar;

    self_557 = self_556;
    other_485 = other_484;
    let _e4: HomogeneousMagnitude = self_557;
    let _e6: AntiScalar = other_485;
    return HomogeneousMagnitude((_e4.g0_ * vec2<f32>(_e6.g0_)));
}

fn homogeneous_magnitude_anti_scalar_inner_anti_product(self_558: HomogeneousMagnitude, other_486: AntiScalar) -> HomogeneousMagnitude {
    var self_559: HomogeneousMagnitude;
    var other_487: AntiScalar;

    self_559 = self_558;
    other_487 = other_486;
    let _e4: HomogeneousMagnitude = self_559;
    let _e6: AntiScalar = other_487;
    return HomogeneousMagnitude((_e4.g0_ * vec2<f32>(_e6.g0_)));
}

fn homogeneous_magnitude_anti_scalar_left_contraction(self_560: HomogeneousMagnitude, other_488: AntiScalar) -> AntiScalar {
    var self_561: HomogeneousMagnitude;
    var other_489: AntiScalar;

    self_561 = self_560;
    other_489 = other_488;
    let _e4: HomogeneousMagnitude = self_561;
    let _e7: AntiScalar = other_489;
    return AntiScalar((_e4.g0_.x * _e7.g0_));
}

fn homogeneous_magnitude_anti_scalar_left_anti_contraction(self_562: HomogeneousMagnitude, other_490: AntiScalar) -> AntiScalar {
    var self_563: HomogeneousMagnitude;
    var other_491: AntiScalar;

    self_563 = self_562;
    other_491 = other_490;
    let _e4: HomogeneousMagnitude = self_563;
    let _e7: AntiScalar = other_491;
    return AntiScalar((_e4.g0_.y * _e7.g0_));
}

fn homogeneous_magnitude_anti_scalar_right_anti_contraction(self_564: HomogeneousMagnitude, other_492: AntiScalar) -> HomogeneousMagnitude {
    var self_565: HomogeneousMagnitude;
    var other_493: AntiScalar;

    self_565 = self_564;
    other_493 = other_492;
    let _e4: HomogeneousMagnitude = self_565;
    let _e6: AntiScalar = other_493;
    return HomogeneousMagnitude((_e4.g0_ * vec2<f32>(_e6.g0_)));
}

fn homogeneous_magnitude_anti_scalar_anti_scalar_product(self_566: HomogeneousMagnitude, other_494: AntiScalar) -> AntiScalar {
    var self_567: HomogeneousMagnitude;
    var other_495: AntiScalar;

    self_567 = self_566;
    other_495 = other_494;
    let _e4: HomogeneousMagnitude = self_567;
    let _e7: AntiScalar = other_495;
    return AntiScalar((_e4.g0_.y * _e7.g0_));
}

fn homogeneous_magnitude_anti_scalar_anti_dot(self_568: HomogeneousMagnitude, other_496: AntiScalar) -> AntiScalar {
    var self_569: HomogeneousMagnitude;
    var other_497: AntiScalar;

    self_569 = self_568;
    other_497 = other_496;
    let _e4: HomogeneousMagnitude = self_569;
    let _e7: AntiScalar = other_497;
    return AntiScalar((_e4.g0_.y * _e7.g0_));
}

fn homogeneous_magnitude_homogeneous_magnitude_add(self_570: HomogeneousMagnitude, other_498: HomogeneousMagnitude) -> HomogeneousMagnitude {
    var self_571: HomogeneousMagnitude;
    var other_499: HomogeneousMagnitude;

    self_571 = self_570;
    other_499 = other_498;
    let _e4: HomogeneousMagnitude = self_571;
    let _e6: HomogeneousMagnitude = other_499;
    return HomogeneousMagnitude((_e4.g0_ + _e6.g0_));
}

fn homogeneous_magnitude_homogeneous_magnitude_sub(self_572: HomogeneousMagnitude, other_500: HomogeneousMagnitude) -> HomogeneousMagnitude {
    var self_573: HomogeneousMagnitude;
    var other_501: HomogeneousMagnitude;

    self_573 = self_572;
    other_501 = other_500;
    let _e4: HomogeneousMagnitude = self_573;
    let _e6: HomogeneousMagnitude = other_501;
    return HomogeneousMagnitude((_e4.g0_ - _e6.g0_));
}

fn homogeneous_magnitude_homogeneous_magnitude_mul(self_574: HomogeneousMagnitude, other_502: HomogeneousMagnitude) -> HomogeneousMagnitude {
    var self_575: HomogeneousMagnitude;
    var other_503: HomogeneousMagnitude;

    self_575 = self_574;
    other_503 = other_502;
    let _e4: HomogeneousMagnitude = self_575;
    let _e6: HomogeneousMagnitude = other_503;
    return HomogeneousMagnitude((_e4.g0_ * _e6.g0_));
}

fn homogeneous_magnitude_homogeneous_magnitude_div(self_576: HomogeneousMagnitude, other_504: HomogeneousMagnitude) -> HomogeneousMagnitude {
    var self_577: HomogeneousMagnitude;
    var other_505: HomogeneousMagnitude;

    self_577 = self_576;
    other_505 = other_504;
    let _e4: HomogeneousMagnitude = self_577;
    let _e7: HomogeneousMagnitude = self_577;
    let _e15: HomogeneousMagnitude = other_505;
    let _e18: HomogeneousMagnitude = other_505;
    return HomogeneousMagnitude((((vec2<f32>(_e4.g0_.x, _e7.g0_.y) * vec2<f32>(1.0, 1.0)) / vec2<f32>(_e15.g0_.x, _e18.g0_.y)) * vec2<f32>(1.0, 1.0)));
}

fn homogeneous_magnitude_homogeneous_magnitude_geometric_product(self_578: HomogeneousMagnitude, other_506: HomogeneousMagnitude) -> HomogeneousMagnitude {
    var self_579: HomogeneousMagnitude;
    var other_507: HomogeneousMagnitude;

    self_579 = self_578;
    other_507 = other_506;
    let _e4: HomogeneousMagnitude = self_579;
    let _e8: HomogeneousMagnitude = other_507;
    let _e11: HomogeneousMagnitude = self_579;
    let _e13: HomogeneousMagnitude = other_507;
    return HomogeneousMagnitude(((vec2<f32>(_e4.g0_.x) * _e8.g0_) + ((_e11.g0_ * vec2<f32>(_e13.g0_.x)) * vec2<f32>(0.0, 1.0))));
}

fn homogeneous_magnitude_homogeneous_magnitude_regressive_product(self_580: HomogeneousMagnitude, other_508: HomogeneousMagnitude) -> HomogeneousMagnitude {
    var self_581: HomogeneousMagnitude;
    var other_509: HomogeneousMagnitude;

    self_581 = self_580;
    other_509 = other_508;
    let _e4: HomogeneousMagnitude = self_581;
    let _e8: HomogeneousMagnitude = other_509;
    let _e11: HomogeneousMagnitude = self_581;
    let _e15: HomogeneousMagnitude = other_509;
    return HomogeneousMagnitude(((vec2<f32>(_e4.g0_.y) * _e8.g0_) + ((vec2<f32>(_e11.g0_.x) * _e15.g0_.yx) * vec2<f32>(1.0, 0.0))));
}

fn homogeneous_magnitude_homogeneous_magnitude_anti_wedge(self_582: HomogeneousMagnitude, other_510: HomogeneousMagnitude) -> HomogeneousMagnitude {
    var self_583: HomogeneousMagnitude;
    var other_511: HomogeneousMagnitude;

    self_583 = self_582;
    other_511 = other_510;
    let _e4: HomogeneousMagnitude = self_583;
    let _e8: HomogeneousMagnitude = other_511;
    let _e11: HomogeneousMagnitude = self_583;
    let _e15: HomogeneousMagnitude = other_511;
    return HomogeneousMagnitude(((vec2<f32>(_e4.g0_.y) * _e8.g0_) + ((vec2<f32>(_e11.g0_.x) * _e15.g0_.yx) * vec2<f32>(1.0, 0.0))));
}

fn homogeneous_magnitude_homogeneous_magnitude_meet(self_584: HomogeneousMagnitude, other_512: HomogeneousMagnitude) -> HomogeneousMagnitude {
    var self_585: HomogeneousMagnitude;
    var other_513: HomogeneousMagnitude;

    self_585 = self_584;
    other_513 = other_512;
    let _e4: HomogeneousMagnitude = self_585;
    let _e8: HomogeneousMagnitude = other_513;
    let _e11: HomogeneousMagnitude = self_585;
    let _e15: HomogeneousMagnitude = other_513;
    return HomogeneousMagnitude(((vec2<f32>(_e4.g0_.y) * _e8.g0_) + ((vec2<f32>(_e11.g0_.x) * _e15.g0_.yx) * vec2<f32>(1.0, 0.0))));
}

fn homogeneous_magnitude_homogeneous_magnitude_outer_product(self_586: HomogeneousMagnitude, other_514: HomogeneousMagnitude) -> HomogeneousMagnitude {
    var self_587: HomogeneousMagnitude;
    var other_515: HomogeneousMagnitude;

    self_587 = self_586;
    other_515 = other_514;
    let _e4: HomogeneousMagnitude = self_587;
    let _e8: HomogeneousMagnitude = other_515;
    let _e11: HomogeneousMagnitude = self_587;
    let _e13: HomogeneousMagnitude = other_515;
    return HomogeneousMagnitude(((vec2<f32>(_e4.g0_.x) * _e8.g0_) + ((_e11.g0_ * vec2<f32>(_e13.g0_.x)) * vec2<f32>(0.0, 1.0))));
}

fn homogeneous_magnitude_homogeneous_magnitude_wedge(self_588: HomogeneousMagnitude, other_516: HomogeneousMagnitude) -> HomogeneousMagnitude {
    var self_589: HomogeneousMagnitude;
    var other_517: HomogeneousMagnitude;

    self_589 = self_588;
    other_517 = other_516;
    let _e4: HomogeneousMagnitude = self_589;
    let _e8: HomogeneousMagnitude = other_517;
    let _e11: HomogeneousMagnitude = self_589;
    let _e13: HomogeneousMagnitude = other_517;
    return HomogeneousMagnitude(((vec2<f32>(_e4.g0_.x) * _e8.g0_) + ((_e11.g0_ * vec2<f32>(_e13.g0_.x)) * vec2<f32>(0.0, 1.0))));
}

fn homogeneous_magnitude_homogeneous_magnitude_join(self_590: HomogeneousMagnitude, other_518: HomogeneousMagnitude) -> HomogeneousMagnitude {
    var self_591: HomogeneousMagnitude;
    var other_519: HomogeneousMagnitude;

    self_591 = self_590;
    other_519 = other_518;
    let _e4: HomogeneousMagnitude = self_591;
    let _e8: HomogeneousMagnitude = other_519;
    let _e11: HomogeneousMagnitude = self_591;
    let _e13: HomogeneousMagnitude = other_519;
    return HomogeneousMagnitude(((vec2<f32>(_e4.g0_.x) * _e8.g0_) + ((_e11.g0_ * vec2<f32>(_e13.g0_.x)) * vec2<f32>(0.0, 1.0))));
}

fn homogeneous_magnitude_homogeneous_magnitude_inner_product(self_592: HomogeneousMagnitude, other_520: HomogeneousMagnitude) -> HomogeneousMagnitude {
    var self_593: HomogeneousMagnitude;
    var other_521: HomogeneousMagnitude;

    self_593 = self_592;
    other_521 = other_520;
    let _e4: HomogeneousMagnitude = self_593;
    let _e8: HomogeneousMagnitude = other_521;
    let _e11: HomogeneousMagnitude = self_593;
    let _e13: HomogeneousMagnitude = other_521;
    return HomogeneousMagnitude(((vec2<f32>(_e4.g0_.x) * _e8.g0_) + ((_e11.g0_ * vec2<f32>(_e13.g0_.x)) * vec2<f32>(0.0, 1.0))));
}

fn homogeneous_magnitude_homogeneous_magnitude_geometric_anti_product(self_594: HomogeneousMagnitude, other_522: HomogeneousMagnitude) -> HomogeneousMagnitude {
    var self_595: HomogeneousMagnitude;
    var other_523: HomogeneousMagnitude;

    self_595 = self_594;
    other_523 = other_522;
    let _e4: HomogeneousMagnitude = self_595;
    let _e8: HomogeneousMagnitude = other_523;
    let _e11: HomogeneousMagnitude = self_595;
    let _e15: HomogeneousMagnitude = other_523;
    return HomogeneousMagnitude(((vec2<f32>(_e4.g0_.y) * _e8.g0_) + ((vec2<f32>(_e11.g0_.x) * _e15.g0_.yx) * vec2<f32>(1.0, 0.0))));
}

fn homogeneous_magnitude_homogeneous_magnitude_inner_anti_product(self_596: HomogeneousMagnitude, other_524: HomogeneousMagnitude) -> HomogeneousMagnitude {
    var self_597: HomogeneousMagnitude;
    var other_525: HomogeneousMagnitude;

    self_597 = self_596;
    other_525 = other_524;
    let _e4: HomogeneousMagnitude = self_597;
    let _e8: HomogeneousMagnitude = other_525;
    let _e11: HomogeneousMagnitude = self_597;
    let _e15: HomogeneousMagnitude = other_525;
    return HomogeneousMagnitude(((vec2<f32>(_e4.g0_.y) * _e8.g0_) + ((vec2<f32>(_e11.g0_.x) * _e15.g0_.yx) * vec2<f32>(1.0, 0.0))));
}

fn homogeneous_magnitude_homogeneous_magnitude_left_contraction(self_598: HomogeneousMagnitude, other_526: HomogeneousMagnitude) -> HomogeneousMagnitude {
    var self_599: HomogeneousMagnitude;
    var other_527: HomogeneousMagnitude;

    self_599 = self_598;
    other_527 = other_526;
    let _e4: HomogeneousMagnitude = self_599;
    let _e8: HomogeneousMagnitude = other_527;
    return HomogeneousMagnitude((vec2<f32>(_e4.g0_.x) * _e8.g0_));
}

fn homogeneous_magnitude_homogeneous_magnitude_right_contraction(self_600: HomogeneousMagnitude, other_528: HomogeneousMagnitude) -> HomogeneousMagnitude {
    var self_601: HomogeneousMagnitude;
    var other_529: HomogeneousMagnitude;

    self_601 = self_600;
    other_529 = other_528;
    let _e4: HomogeneousMagnitude = self_601;
    let _e6: HomogeneousMagnitude = other_529;
    return HomogeneousMagnitude((_e4.g0_ * vec2<f32>(_e6.g0_.x)));
}

fn homogeneous_magnitude_homogeneous_magnitude_left_anti_contraction(self_602: HomogeneousMagnitude, other_530: HomogeneousMagnitude) -> HomogeneousMagnitude {
    var self_603: HomogeneousMagnitude;
    var other_531: HomogeneousMagnitude;

    self_603 = self_602;
    other_531 = other_530;
    let _e4: HomogeneousMagnitude = self_603;
    let _e8: HomogeneousMagnitude = other_531;
    return HomogeneousMagnitude((vec2<f32>(_e4.g0_.y) * _e8.g0_));
}

fn homogeneous_magnitude_homogeneous_magnitude_right_anti_contraction(self_604: HomogeneousMagnitude, other_532: HomogeneousMagnitude) -> HomogeneousMagnitude {
    var self_605: HomogeneousMagnitude;
    var other_533: HomogeneousMagnitude;

    self_605 = self_604;
    other_533 = other_532;
    let _e4: HomogeneousMagnitude = self_605;
    let _e6: HomogeneousMagnitude = other_533;
    return HomogeneousMagnitude((_e4.g0_ * vec2<f32>(_e6.g0_.y)));
}

fn homogeneous_magnitude_homogeneous_magnitude_scalar_product(self_606: HomogeneousMagnitude, other_534: HomogeneousMagnitude) -> Scalar {
    var self_607: HomogeneousMagnitude;
    var other_535: HomogeneousMagnitude;

    self_607 = self_606;
    other_535 = other_534;
    let _e4: HomogeneousMagnitude = self_607;
    let _e7: HomogeneousMagnitude = other_535;
    return Scalar((_e4.g0_.x * _e7.g0_.x));
}

fn homogeneous_magnitude_homogeneous_magnitude_dot(self_608: HomogeneousMagnitude, other_536: HomogeneousMagnitude) -> Scalar {
    var self_609: HomogeneousMagnitude;
    var other_537: HomogeneousMagnitude;

    self_609 = self_608;
    other_537 = other_536;
    let _e4: HomogeneousMagnitude = self_609;
    let _e7: HomogeneousMagnitude = other_537;
    return Scalar((_e4.g0_.x * _e7.g0_.x));
}

fn homogeneous_magnitude_homogeneous_magnitude_anti_scalar_product(self_610: HomogeneousMagnitude, other_538: HomogeneousMagnitude) -> AntiScalar {
    var self_611: HomogeneousMagnitude;
    var other_539: HomogeneousMagnitude;

    self_611 = self_610;
    other_539 = other_538;
    let _e4: HomogeneousMagnitude = self_611;
    let _e7: HomogeneousMagnitude = other_539;
    return AntiScalar((_e4.g0_.y * _e7.g0_.y));
}

fn homogeneous_magnitude_homogeneous_magnitude_anti_dot(self_612: HomogeneousMagnitude, other_540: HomogeneousMagnitude) -> AntiScalar {
    var self_613: HomogeneousMagnitude;
    var other_541: HomogeneousMagnitude;

    self_613 = self_612;
    other_541 = other_540;
    let _e4: HomogeneousMagnitude = self_613;
    let _e7: HomogeneousMagnitude = other_541;
    return AntiScalar((_e4.g0_.y * _e7.g0_.y));
}

fn homogeneous_magnitude_point_regressive_product(self_614: HomogeneousMagnitude, other_542: Point) -> Point {
    var self_615: HomogeneousMagnitude;
    var other_543: Point;

    self_615 = self_614;
    other_543 = other_542;
    let _e4: HomogeneousMagnitude = self_615;
    let _e8: Point = other_543;
    return Point((vec4<f32>(_e4.g0_.y) * _e8.g0_));
}

fn homogeneous_magnitude_point_anti_wedge(self_616: HomogeneousMagnitude, other_544: Point) -> Point {
    var self_617: HomogeneousMagnitude;
    var other_545: Point;

    self_617 = self_616;
    other_545 = other_544;
    let _e4: HomogeneousMagnitude = self_617;
    let _e8: Point = other_545;
    return Point((vec4<f32>(_e4.g0_.y) * _e8.g0_));
}

fn homogeneous_magnitude_point_meet(self_618: HomogeneousMagnitude, other_546: Point) -> Point {
    var self_619: HomogeneousMagnitude;
    var other_547: Point;

    self_619 = self_618;
    other_547 = other_546;
    let _e4: HomogeneousMagnitude = self_619;
    let _e8: Point = other_547;
    return Point((vec4<f32>(_e4.g0_.y) * _e8.g0_));
}

fn homogeneous_magnitude_point_outer_product(self_620: HomogeneousMagnitude, other_548: Point) -> Point {
    var self_621: HomogeneousMagnitude;
    var other_549: Point;

    self_621 = self_620;
    other_549 = other_548;
    let _e4: HomogeneousMagnitude = self_621;
    let _e8: Point = other_549;
    return Point((vec4<f32>(_e4.g0_.x) * _e8.g0_));
}

fn homogeneous_magnitude_point_wedge(self_622: HomogeneousMagnitude, other_550: Point) -> Point {
    var self_623: HomogeneousMagnitude;
    var other_551: Point;

    self_623 = self_622;
    other_551 = other_550;
    let _e4: HomogeneousMagnitude = self_623;
    let _e8: Point = other_551;
    return Point((vec4<f32>(_e4.g0_.x) * _e8.g0_));
}

fn homogeneous_magnitude_point_join(self_624: HomogeneousMagnitude, other_552: Point) -> Point {
    var self_625: HomogeneousMagnitude;
    var other_553: Point;

    self_625 = self_624;
    other_553 = other_552;
    let _e4: HomogeneousMagnitude = self_625;
    let _e8: Point = other_553;
    return Point((vec4<f32>(_e4.g0_.x) * _e8.g0_));
}

fn homogeneous_magnitude_point_left_contraction(self_626: HomogeneousMagnitude, other_554: Point) -> Point {
    var self_627: HomogeneousMagnitude;
    var other_555: Point;

    self_627 = self_626;
    other_555 = other_554;
    let _e4: HomogeneousMagnitude = self_627;
    let _e8: Point = other_555;
    return Point((vec4<f32>(_e4.g0_.x) * _e8.g0_));
}

fn homogeneous_magnitude_point_left_anti_contraction(self_628: HomogeneousMagnitude, other_556: Point) -> Point {
    var self_629: HomogeneousMagnitude;
    var other_557: Point;

    self_629 = self_628;
    other_557 = other_556;
    let _e4: HomogeneousMagnitude = self_629;
    let _e8: Point = other_557;
    return Point((vec4<f32>(_e4.g0_.y) * _e8.g0_));
}

fn homogeneous_magnitude_line_geometric_product(self_630: HomogeneousMagnitude, other_558: Line) -> Line {
    var self_631: HomogeneousMagnitude;
    var other_559: Line;

    self_631 = self_630;
    other_559 = other_558;
    let _e4: HomogeneousMagnitude = self_631;
    let _e8: Line = other_559;
    let _e11: HomogeneousMagnitude = self_631;
    let _e15: Line = other_559;
    let _e19: HomogeneousMagnitude = self_631;
    let _e23: Line = other_559;
    return Line(((vec3<f32>(_e4.g0_.x) * _e8.g0_) + (vec3<f32>(_e11.g0_.y) * _e15.g1_)), (vec3<f32>(_e19.g0_.x) * _e23.g1_));
}

fn homogeneous_magnitude_line_regressive_product(self_632: HomogeneousMagnitude, other_560: Line) -> Line {
    var self_633: HomogeneousMagnitude;
    var other_561: Line;

    self_633 = self_632;
    other_561 = other_560;
    let _e4: HomogeneousMagnitude = self_633;
    let _e8: Line = other_561;
    let _e11: HomogeneousMagnitude = self_633;
    let _e15: Line = other_561;
    return Line((vec3<f32>(_e4.g0_.y) * _e8.g0_), (vec3<f32>(_e11.g0_.y) * _e15.g1_));
}

fn homogeneous_magnitude_line_anti_wedge(self_634: HomogeneousMagnitude, other_562: Line) -> Line {
    var self_635: HomogeneousMagnitude;
    var other_563: Line;

    self_635 = self_634;
    other_563 = other_562;
    let _e4: HomogeneousMagnitude = self_635;
    let _e8: Line = other_563;
    let _e11: HomogeneousMagnitude = self_635;
    let _e15: Line = other_563;
    return Line((vec3<f32>(_e4.g0_.y) * _e8.g0_), (vec3<f32>(_e11.g0_.y) * _e15.g1_));
}

fn homogeneous_magnitude_line_meet(self_636: HomogeneousMagnitude, other_564: Line) -> Line {
    var self_637: HomogeneousMagnitude;
    var other_565: Line;

    self_637 = self_636;
    other_565 = other_564;
    let _e4: HomogeneousMagnitude = self_637;
    let _e8: Line = other_565;
    let _e11: HomogeneousMagnitude = self_637;
    let _e15: Line = other_565;
    return Line((vec3<f32>(_e4.g0_.y) * _e8.g0_), (vec3<f32>(_e11.g0_.y) * _e15.g1_));
}

fn homogeneous_magnitude_line_outer_product(self_638: HomogeneousMagnitude, other_566: Line) -> Line {
    var self_639: HomogeneousMagnitude;
    var other_567: Line;

    self_639 = self_638;
    other_567 = other_566;
    let _e4: HomogeneousMagnitude = self_639;
    let _e8: Line = other_567;
    let _e11: HomogeneousMagnitude = self_639;
    let _e15: Line = other_567;
    return Line((vec3<f32>(_e4.g0_.x) * _e8.g0_), (vec3<f32>(_e11.g0_.x) * _e15.g1_));
}

fn homogeneous_magnitude_line_wedge(self_640: HomogeneousMagnitude, other_568: Line) -> Line {
    var self_641: HomogeneousMagnitude;
    var other_569: Line;

    self_641 = self_640;
    other_569 = other_568;
    let _e4: HomogeneousMagnitude = self_641;
    let _e8: Line = other_569;
    let _e11: HomogeneousMagnitude = self_641;
    let _e15: Line = other_569;
    return Line((vec3<f32>(_e4.g0_.x) * _e8.g0_), (vec3<f32>(_e11.g0_.x) * _e15.g1_));
}

fn homogeneous_magnitude_line_join(self_642: HomogeneousMagnitude, other_570: Line) -> Line {
    var self_643: HomogeneousMagnitude;
    var other_571: Line;

    self_643 = self_642;
    other_571 = other_570;
    let _e4: HomogeneousMagnitude = self_643;
    let _e8: Line = other_571;
    let _e11: HomogeneousMagnitude = self_643;
    let _e15: Line = other_571;
    return Line((vec3<f32>(_e4.g0_.x) * _e8.g0_), (vec3<f32>(_e11.g0_.x) * _e15.g1_));
}

fn homogeneous_magnitude_line_inner_product(self_644: HomogeneousMagnitude, other_572: Line) -> Line {
    var self_645: HomogeneousMagnitude;
    var other_573: Line;

    self_645 = self_644;
    other_573 = other_572;
    let _e4: HomogeneousMagnitude = self_645;
    let _e8: Line = other_573;
    let _e11: HomogeneousMagnitude = self_645;
    let _e15: Line = other_573;
    let _e19: HomogeneousMagnitude = self_645;
    let _e23: Line = other_573;
    return Line(((vec3<f32>(_e4.g0_.x) * _e8.g0_) + (vec3<f32>(_e11.g0_.y) * _e15.g1_)), (vec3<f32>(_e19.g0_.x) * _e23.g1_));
}

fn homogeneous_magnitude_line_geometric_anti_product(self_646: HomogeneousMagnitude, other_574: Line) -> Line {
    var self_647: HomogeneousMagnitude;
    var other_575: Line;

    self_647 = self_646;
    other_575 = other_574;
    let _e4: HomogeneousMagnitude = self_647;
    let _e8: Line = other_575;
    let _e11: HomogeneousMagnitude = self_647;
    let _e15: Line = other_575;
    let _e18: HomogeneousMagnitude = self_647;
    let _e22: Line = other_575;
    return Line((vec3<f32>(_e4.g0_.y) * _e8.g0_), ((vec3<f32>(_e11.g0_.x) * _e15.g0_) + (vec3<f32>(_e18.g0_.y) * _e22.g1_)));
}

fn homogeneous_magnitude_line_inner_anti_product(self_648: HomogeneousMagnitude, other_576: Line) -> Line {
    var self_649: HomogeneousMagnitude;
    var other_577: Line;

    self_649 = self_648;
    other_577 = other_576;
    let _e4: HomogeneousMagnitude = self_649;
    let _e8: Line = other_577;
    let _e11: HomogeneousMagnitude = self_649;
    let _e15: Line = other_577;
    let _e18: HomogeneousMagnitude = self_649;
    let _e22: Line = other_577;
    return Line((vec3<f32>(_e4.g0_.y) * _e8.g0_), ((vec3<f32>(_e11.g0_.x) * _e15.g0_) + (vec3<f32>(_e18.g0_.y) * _e22.g1_)));
}

fn homogeneous_magnitude_line_left_contraction(self_650: HomogeneousMagnitude, other_578: Line) -> Line {
    var self_651: HomogeneousMagnitude;
    var other_579: Line;

    self_651 = self_650;
    other_579 = other_578;
    let _e4: HomogeneousMagnitude = self_651;
    let _e8: Line = other_579;
    let _e11: HomogeneousMagnitude = self_651;
    let _e15: Line = other_579;
    return Line((vec3<f32>(_e4.g0_.x) * _e8.g0_), (vec3<f32>(_e11.g0_.x) * _e15.g1_));
}

fn homogeneous_magnitude_line_left_anti_contraction(self_652: HomogeneousMagnitude, other_580: Line) -> Line {
    var self_653: HomogeneousMagnitude;
    var other_581: Line;

    self_653 = self_652;
    other_581 = other_580;
    let _e4: HomogeneousMagnitude = self_653;
    let _e8: Line = other_581;
    let _e11: HomogeneousMagnitude = self_653;
    let _e15: Line = other_581;
    return Line((vec3<f32>(_e4.g0_.y) * _e8.g0_), (vec3<f32>(_e11.g0_.y) * _e15.g1_));
}

fn homogeneous_magnitude_plane_regressive_product(self_654: HomogeneousMagnitude, other_582: Plane) -> Plane {
    var self_655: HomogeneousMagnitude;
    var other_583: Plane;

    self_655 = self_654;
    other_583 = other_582;
    let _e4: HomogeneousMagnitude = self_655;
    let _e8: Plane = other_583;
    return Plane((vec4<f32>(_e4.g0_.y) * _e8.g0_));
}

fn homogeneous_magnitude_plane_anti_wedge(self_656: HomogeneousMagnitude, other_584: Plane) -> Plane {
    var self_657: HomogeneousMagnitude;
    var other_585: Plane;

    self_657 = self_656;
    other_585 = other_584;
    let _e4: HomogeneousMagnitude = self_657;
    let _e8: Plane = other_585;
    return Plane((vec4<f32>(_e4.g0_.y) * _e8.g0_));
}

fn homogeneous_magnitude_plane_meet(self_658: HomogeneousMagnitude, other_586: Plane) -> Plane {
    var self_659: HomogeneousMagnitude;
    var other_587: Plane;

    self_659 = self_658;
    other_587 = other_586;
    let _e4: HomogeneousMagnitude = self_659;
    let _e8: Plane = other_587;
    return Plane((vec4<f32>(_e4.g0_.y) * _e8.g0_));
}

fn homogeneous_magnitude_plane_outer_product(self_660: HomogeneousMagnitude, other_588: Plane) -> Plane {
    var self_661: HomogeneousMagnitude;
    var other_589: Plane;

    self_661 = self_660;
    other_589 = other_588;
    let _e4: HomogeneousMagnitude = self_661;
    let _e8: Plane = other_589;
    return Plane((vec4<f32>(_e4.g0_.x) * _e8.g0_));
}

fn homogeneous_magnitude_plane_wedge(self_662: HomogeneousMagnitude, other_590: Plane) -> Plane {
    var self_663: HomogeneousMagnitude;
    var other_591: Plane;

    self_663 = self_662;
    other_591 = other_590;
    let _e4: HomogeneousMagnitude = self_663;
    let _e8: Plane = other_591;
    return Plane((vec4<f32>(_e4.g0_.x) * _e8.g0_));
}

fn homogeneous_magnitude_plane_join(self_664: HomogeneousMagnitude, other_592: Plane) -> Plane {
    var self_665: HomogeneousMagnitude;
    var other_593: Plane;

    self_665 = self_664;
    other_593 = other_592;
    let _e4: HomogeneousMagnitude = self_665;
    let _e8: Plane = other_593;
    return Plane((vec4<f32>(_e4.g0_.x) * _e8.g0_));
}

fn homogeneous_magnitude_plane_left_contraction(self_666: HomogeneousMagnitude, other_594: Plane) -> Plane {
    var self_667: HomogeneousMagnitude;
    var other_595: Plane;

    self_667 = self_666;
    other_595 = other_594;
    let _e4: HomogeneousMagnitude = self_667;
    let _e8: Plane = other_595;
    return Plane((vec4<f32>(_e4.g0_.x) * _e8.g0_));
}

fn homogeneous_magnitude_plane_left_anti_contraction(self_668: HomogeneousMagnitude, other_596: Plane) -> Plane {
    var self_669: HomogeneousMagnitude;
    var other_597: Plane;

    self_669 = self_668;
    other_597 = other_596;
    let _e4: HomogeneousMagnitude = self_669;
    let _e8: Plane = other_597;
    return Plane((vec4<f32>(_e4.g0_.y) * _e8.g0_));
}

fn homogeneous_magnitude_motor_geometric_product(self_670: HomogeneousMagnitude, other_598: Motor) -> Motor {
    var self_671: HomogeneousMagnitude;
    var other_599: Motor;

    self_671 = self_670;
    other_599 = other_598;
    let _e4: HomogeneousMagnitude = self_671;
    let _e8: Motor = other_599;
    let _e11: HomogeneousMagnitude = self_671;
    let _e14: HomogeneousMagnitude = self_671;
    let _e17: HomogeneousMagnitude = self_671;
    let _e20: HomogeneousMagnitude = self_671;
    let _e24: Motor = other_599;
    let _e27: Motor = other_599;
    let _e30: Motor = other_599;
    let _e33: Motor = other_599;
    let _e45: HomogeneousMagnitude = self_671;
    let _e49: Motor = other_599;
    return Motor(((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.y, _e14.g0_.y, _e17.g0_.y, _e20.g0_.x) * vec4<f32>(_e24.g1_.x, _e27.g1_.y, _e30.g1_.z, _e33.g1_.x)) * vec4<f32>(1.0, 1.0, 1.0, 0.0))), (vec3<f32>(_e45.g0_.x) * _e49.g1_));
}

fn homogeneous_magnitude_motor_outer_product(self_672: HomogeneousMagnitude, other_600: Motor) -> Motor {
    var self_673: HomogeneousMagnitude;
    var other_601: Motor;

    self_673 = self_672;
    other_601 = other_600;
    let _e4: HomogeneousMagnitude = self_673;
    let _e8: Motor = other_601;
    let _e11: HomogeneousMagnitude = self_673;
    let _e15: Motor = other_601;
    return Motor((vec4<f32>(_e4.g0_.x) * _e8.g0_), (vec3<f32>(_e11.g0_.x) * _e15.g1_));
}

fn homogeneous_magnitude_motor_wedge(self_674: HomogeneousMagnitude, other_602: Motor) -> Motor {
    var self_675: HomogeneousMagnitude;
    var other_603: Motor;

    self_675 = self_674;
    other_603 = other_602;
    let _e4: HomogeneousMagnitude = self_675;
    let _e8: Motor = other_603;
    let _e11: HomogeneousMagnitude = self_675;
    let _e15: Motor = other_603;
    return Motor((vec4<f32>(_e4.g0_.x) * _e8.g0_), (vec3<f32>(_e11.g0_.x) * _e15.g1_));
}

fn homogeneous_magnitude_motor_join(self_676: HomogeneousMagnitude, other_604: Motor) -> Motor {
    var self_677: HomogeneousMagnitude;
    var other_605: Motor;

    self_677 = self_676;
    other_605 = other_604;
    let _e4: HomogeneousMagnitude = self_677;
    let _e8: Motor = other_605;
    let _e11: HomogeneousMagnitude = self_677;
    let _e15: Motor = other_605;
    return Motor((vec4<f32>(_e4.g0_.x) * _e8.g0_), (vec3<f32>(_e11.g0_.x) * _e15.g1_));
}

fn homogeneous_magnitude_motor_inner_product(self_678: HomogeneousMagnitude, other_606: Motor) -> Motor {
    var self_679: HomogeneousMagnitude;
    var other_607: Motor;

    self_679 = self_678;
    other_607 = other_606;
    let _e4: HomogeneousMagnitude = self_679;
    let _e8: Motor = other_607;
    let _e11: HomogeneousMagnitude = self_679;
    let _e14: HomogeneousMagnitude = self_679;
    let _e17: HomogeneousMagnitude = self_679;
    let _e20: HomogeneousMagnitude = self_679;
    let _e24: Motor = other_607;
    let _e27: Motor = other_607;
    let _e30: Motor = other_607;
    let _e33: Motor = other_607;
    let _e45: HomogeneousMagnitude = self_679;
    let _e49: Motor = other_607;
    return Motor(((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.y, _e14.g0_.y, _e17.g0_.y, _e20.g0_.x) * vec4<f32>(_e24.g1_.x, _e27.g1_.y, _e30.g1_.z, _e33.g1_.x)) * vec4<f32>(1.0, 1.0, 1.0, 0.0))), (vec3<f32>(_e45.g0_.x) * _e49.g1_));
}

fn homogeneous_magnitude_motor_left_contraction(self_680: HomogeneousMagnitude, other_608: Motor) -> Motor {
    var self_681: HomogeneousMagnitude;
    var other_609: Motor;

    self_681 = self_680;
    other_609 = other_608;
    let _e4: HomogeneousMagnitude = self_681;
    let _e8: Motor = other_609;
    let _e11: HomogeneousMagnitude = self_681;
    let _e15: Motor = other_609;
    return Motor((vec4<f32>(_e4.g0_.x) * _e8.g0_), (vec3<f32>(_e11.g0_.x) * _e15.g1_));
}

fn homogeneous_magnitude_motor_left_anti_contraction(self_682: HomogeneousMagnitude, other_610: Motor) -> Motor {
    var self_683: HomogeneousMagnitude;
    var other_611: Motor;

    self_683 = self_682;
    other_611 = other_610;
    let _e4: HomogeneousMagnitude = self_683;
    let _e8: Motor = other_611;
    let _e11: HomogeneousMagnitude = self_683;
    let _e15: Motor = other_611;
    return Motor((vec4<f32>(_e4.g0_.y) * _e8.g0_), (vec3<f32>(_e11.g0_.y) * _e15.g1_));
}

fn homogeneous_magnitude_motor_anti_scalar_product(self_684: HomogeneousMagnitude, other_612: Motor) -> AntiScalar {
    var self_685: HomogeneousMagnitude;
    var other_613: Motor;

    self_685 = self_684;
    other_613 = other_612;
    let _e4: HomogeneousMagnitude = self_685;
    let _e7: Motor = other_613;
    return AntiScalar((_e4.g0_.y * _e7.g0_.w));
}

fn homogeneous_magnitude_motor_anti_dot(self_686: HomogeneousMagnitude, other_614: Motor) -> AntiScalar {
    var self_687: HomogeneousMagnitude;
    var other_615: Motor;

    self_687 = self_686;
    other_615 = other_614;
    let _e4: HomogeneousMagnitude = self_687;
    let _e7: Motor = other_615;
    return AntiScalar((_e4.g0_.y * _e7.g0_.w));
}

fn homogeneous_magnitude_rotor_geometric_product(self_688: HomogeneousMagnitude, other_616: Rotor) -> Rotor {
    var self_689: HomogeneousMagnitude;
    var other_617: Rotor;

    self_689 = self_688;
    other_617 = other_616;
    let _e4: HomogeneousMagnitude = self_689;
    let _e8: Rotor = other_617;
    return Rotor((vec4<f32>(_e4.g0_.x) * _e8.g0_));
}

fn homogeneous_magnitude_rotor_outer_product(self_690: HomogeneousMagnitude, other_618: Rotor) -> Rotor {
    var self_691: HomogeneousMagnitude;
    var other_619: Rotor;

    self_691 = self_690;
    other_619 = other_618;
    let _e4: HomogeneousMagnitude = self_691;
    let _e8: Rotor = other_619;
    return Rotor((vec4<f32>(_e4.g0_.x) * _e8.g0_));
}

fn homogeneous_magnitude_rotor_wedge(self_692: HomogeneousMagnitude, other_620: Rotor) -> Rotor {
    var self_693: HomogeneousMagnitude;
    var other_621: Rotor;

    self_693 = self_692;
    other_621 = other_620;
    let _e4: HomogeneousMagnitude = self_693;
    let _e8: Rotor = other_621;
    return Rotor((vec4<f32>(_e4.g0_.x) * _e8.g0_));
}

fn homogeneous_magnitude_rotor_join(self_694: HomogeneousMagnitude, other_622: Rotor) -> Rotor {
    var self_695: HomogeneousMagnitude;
    var other_623: Rotor;

    self_695 = self_694;
    other_623 = other_622;
    let _e4: HomogeneousMagnitude = self_695;
    let _e8: Rotor = other_623;
    return Rotor((vec4<f32>(_e4.g0_.x) * _e8.g0_));
}

fn homogeneous_magnitude_rotor_inner_product(self_696: HomogeneousMagnitude, other_624: Rotor) -> Rotor {
    var self_697: HomogeneousMagnitude;
    var other_625: Rotor;

    self_697 = self_696;
    other_625 = other_624;
    let _e4: HomogeneousMagnitude = self_697;
    let _e8: Rotor = other_625;
    return Rotor((vec4<f32>(_e4.g0_.x) * _e8.g0_));
}

fn homogeneous_magnitude_rotor_left_contraction(self_698: HomogeneousMagnitude, other_626: Rotor) -> Rotor {
    var self_699: HomogeneousMagnitude;
    var other_627: Rotor;

    self_699 = self_698;
    other_627 = other_626;
    let _e4: HomogeneousMagnitude = self_699;
    let _e8: Rotor = other_627;
    return Rotor((vec4<f32>(_e4.g0_.x) * _e8.g0_));
}

fn homogeneous_magnitude_rotor_left_anti_contraction(self_700: HomogeneousMagnitude, other_628: Rotor) -> Rotor {
    var self_701: HomogeneousMagnitude;
    var other_629: Rotor;

    self_701 = self_700;
    other_629 = other_628;
    let _e4: HomogeneousMagnitude = self_701;
    let _e8: Rotor = other_629;
    return Rotor((vec4<f32>(_e4.g0_.y) * _e8.g0_));
}

fn homogeneous_magnitude_rotor_anti_scalar_product(self_702: HomogeneousMagnitude, other_630: Rotor) -> AntiScalar {
    var self_703: HomogeneousMagnitude;
    var other_631: Rotor;

    self_703 = self_702;
    other_631 = other_630;
    let _e4: HomogeneousMagnitude = self_703;
    let _e7: Rotor = other_631;
    return AntiScalar((_e4.g0_.y * _e7.g0_.w));
}

fn homogeneous_magnitude_rotor_anti_dot(self_704: HomogeneousMagnitude, other_632: Rotor) -> AntiScalar {
    var self_705: HomogeneousMagnitude;
    var other_633: Rotor;

    self_705 = self_704;
    other_633 = other_632;
    let _e4: HomogeneousMagnitude = self_705;
    let _e7: Rotor = other_633;
    return AntiScalar((_e4.g0_.y * _e7.g0_.w));
}

fn homogeneous_magnitude_translator_geometric_product(self_706: HomogeneousMagnitude, other_634: Translator) -> Motor {
    var self_707: HomogeneousMagnitude;
    var other_635: Translator;

    self_707 = self_706;
    other_635 = other_634;
    let _e4: HomogeneousMagnitude = self_707;
    let _e7: HomogeneousMagnitude = self_707;
    let _e10: HomogeneousMagnitude = self_707;
    let _e13: HomogeneousMagnitude = self_707;
    let _e17: Translator = other_635;
    let _e20: HomogeneousMagnitude = self_707;
    let _e24: Translator = other_635;
    let _e27: Translator = other_635;
    let _e30: Translator = other_635;
    return Motor((vec4<f32>(_e4.g0_.y, _e7.g0_.y, _e10.g0_.y, _e13.g0_.x) * _e17.g0_), (vec3<f32>(_e20.g0_.x) * vec3<f32>(_e24.g0_.x, _e27.g0_.y, _e30.g0_.z)));
}

fn homogeneous_magnitude_translator_outer_product(self_708: HomogeneousMagnitude, other_636: Translator) -> Translator {
    var self_709: HomogeneousMagnitude;
    var other_637: Translator;

    self_709 = self_708;
    other_637 = other_636;
    let _e4: HomogeneousMagnitude = self_709;
    let _e8: Translator = other_637;
    return Translator((vec4<f32>(_e4.g0_.x) * _e8.g0_));
}

fn homogeneous_magnitude_translator_wedge(self_710: HomogeneousMagnitude, other_638: Translator) -> Translator {
    var self_711: HomogeneousMagnitude;
    var other_639: Translator;

    self_711 = self_710;
    other_639 = other_638;
    let _e4: HomogeneousMagnitude = self_711;
    let _e8: Translator = other_639;
    return Translator((vec4<f32>(_e4.g0_.x) * _e8.g0_));
}

fn homogeneous_magnitude_translator_join(self_712: HomogeneousMagnitude, other_640: Translator) -> Translator {
    var self_713: HomogeneousMagnitude;
    var other_641: Translator;

    self_713 = self_712;
    other_641 = other_640;
    let _e4: HomogeneousMagnitude = self_713;
    let _e8: Translator = other_641;
    return Translator((vec4<f32>(_e4.g0_.x) * _e8.g0_));
}

fn homogeneous_magnitude_translator_inner_product(self_714: HomogeneousMagnitude, other_642: Translator) -> Motor {
    var self_715: HomogeneousMagnitude;
    var other_643: Translator;

    self_715 = self_714;
    other_643 = other_642;
    let _e4: HomogeneousMagnitude = self_715;
    let _e7: HomogeneousMagnitude = self_715;
    let _e10: HomogeneousMagnitude = self_715;
    let _e13: HomogeneousMagnitude = self_715;
    let _e17: Translator = other_643;
    let _e20: HomogeneousMagnitude = self_715;
    let _e24: Translator = other_643;
    let _e27: Translator = other_643;
    let _e30: Translator = other_643;
    return Motor((vec4<f32>(_e4.g0_.y, _e7.g0_.y, _e10.g0_.y, _e13.g0_.x) * _e17.g0_), (vec3<f32>(_e20.g0_.x) * vec3<f32>(_e24.g0_.x, _e27.g0_.y, _e30.g0_.z)));
}

fn homogeneous_magnitude_translator_left_contraction(self_716: HomogeneousMagnitude, other_644: Translator) -> Translator {
    var self_717: HomogeneousMagnitude;
    var other_645: Translator;

    self_717 = self_716;
    other_645 = other_644;
    let _e4: HomogeneousMagnitude = self_717;
    let _e8: Translator = other_645;
    return Translator((vec4<f32>(_e4.g0_.x) * _e8.g0_));
}

fn homogeneous_magnitude_translator_left_anti_contraction(self_718: HomogeneousMagnitude, other_646: Translator) -> Translator {
    var self_719: HomogeneousMagnitude;
    var other_647: Translator;

    self_719 = self_718;
    other_647 = other_646;
    let _e4: HomogeneousMagnitude = self_719;
    let _e8: Translator = other_647;
    return Translator((vec4<f32>(_e4.g0_.y) * _e8.g0_));
}

fn homogeneous_magnitude_translator_right_anti_contraction(self_720: HomogeneousMagnitude, other_648: Translator) -> HomogeneousMagnitude {
    var self_721: HomogeneousMagnitude;
    var other_649: Translator;

    self_721 = self_720;
    other_649 = other_648;
    let _e4: HomogeneousMagnitude = self_721;
    let _e6: Translator = other_649;
    return HomogeneousMagnitude((_e4.g0_ * vec2<f32>(_e6.g0_.w)));
}

fn homogeneous_magnitude_translator_anti_scalar_product(self_722: HomogeneousMagnitude, other_650: Translator) -> AntiScalar {
    var self_723: HomogeneousMagnitude;
    var other_651: Translator;

    self_723 = self_722;
    other_651 = other_650;
    let _e4: HomogeneousMagnitude = self_723;
    let _e7: Translator = other_651;
    return AntiScalar((_e4.g0_.y * _e7.g0_.w));
}

fn homogeneous_magnitude_translator_anti_dot(self_724: HomogeneousMagnitude, other_652: Translator) -> AntiScalar {
    var self_725: HomogeneousMagnitude;
    var other_653: Translator;

    self_725 = self_724;
    other_653 = other_652;
    let _e4: HomogeneousMagnitude = self_725;
    let _e7: Translator = other_653;
    return AntiScalar((_e4.g0_.y * _e7.g0_.w));
}

fn homogeneous_magnitude_flector_geometric_product(self_726: HomogeneousMagnitude, other_654: Flector) -> Flector {
    var self_727: HomogeneousMagnitude;
    var other_655: Flector;

    self_727 = self_726;
    other_655 = other_654;
    let _e4: HomogeneousMagnitude = self_727;
    let _e8: Flector = other_655;
    let _e11: HomogeneousMagnitude = self_727;
    let _e14: HomogeneousMagnitude = self_727;
    let _e17: HomogeneousMagnitude = self_727;
    let _e20: HomogeneousMagnitude = self_727;
    let _e24: Flector = other_655;
    let _e36: HomogeneousMagnitude = self_727;
    let _e40: Flector = other_655;
    let _e43: HomogeneousMagnitude = self_727;
    let _e46: HomogeneousMagnitude = self_727;
    let _e49: HomogeneousMagnitude = self_727;
    let _e52: HomogeneousMagnitude = self_727;
    let _e56: Flector = other_655;
    return Flector(((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.x, _e14.g0_.x, _e17.g0_.x, _e20.g0_.y) * _e24.g1_.xxxw) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))), ((vec4<f32>(_e36.g0_.x) * _e40.g1_) + ((vec4<f32>(_e43.g0_.y, _e46.g0_.y, _e49.g0_.y, _e52.g0_.x) * _e56.g0_.xyzx) * vec4<f32>(-(1.0), -(1.0), -(1.0), 0.0))));
}

fn homogeneous_magnitude_flector_regressive_product(self_728: HomogeneousMagnitude, other_656: Flector) -> Flector {
    var self_729: HomogeneousMagnitude;
    var other_657: Flector;

    self_729 = self_728;
    other_657 = other_656;
    let _e4: HomogeneousMagnitude = self_729;
    let _e8: Flector = other_657;
    let _e11: HomogeneousMagnitude = self_729;
    let _e15: Flector = other_657;
    return Flector((vec4<f32>(_e4.g0_.y) * _e8.g0_), (vec4<f32>(_e11.g0_.y) * _e15.g1_));
}

fn homogeneous_magnitude_flector_anti_wedge(self_730: HomogeneousMagnitude, other_658: Flector) -> Flector {
    var self_731: HomogeneousMagnitude;
    var other_659: Flector;

    self_731 = self_730;
    other_659 = other_658;
    let _e4: HomogeneousMagnitude = self_731;
    let _e8: Flector = other_659;
    let _e11: HomogeneousMagnitude = self_731;
    let _e15: Flector = other_659;
    return Flector((vec4<f32>(_e4.g0_.y) * _e8.g0_), (vec4<f32>(_e11.g0_.y) * _e15.g1_));
}

fn homogeneous_magnitude_flector_meet(self_732: HomogeneousMagnitude, other_660: Flector) -> Flector {
    var self_733: HomogeneousMagnitude;
    var other_661: Flector;

    self_733 = self_732;
    other_661 = other_660;
    let _e4: HomogeneousMagnitude = self_733;
    let _e8: Flector = other_661;
    let _e11: HomogeneousMagnitude = self_733;
    let _e15: Flector = other_661;
    return Flector((vec4<f32>(_e4.g0_.y) * _e8.g0_), (vec4<f32>(_e11.g0_.y) * _e15.g1_));
}

fn homogeneous_magnitude_flector_outer_product(self_734: HomogeneousMagnitude, other_662: Flector) -> Flector {
    var self_735: HomogeneousMagnitude;
    var other_663: Flector;

    self_735 = self_734;
    other_663 = other_662;
    let _e4: HomogeneousMagnitude = self_735;
    let _e8: Flector = other_663;
    let _e11: HomogeneousMagnitude = self_735;
    let _e15: Flector = other_663;
    return Flector((vec4<f32>(_e4.g0_.x) * _e8.g0_), (vec4<f32>(_e11.g0_.x) * _e15.g1_));
}

fn homogeneous_magnitude_flector_wedge(self_736: HomogeneousMagnitude, other_664: Flector) -> Flector {
    var self_737: HomogeneousMagnitude;
    var other_665: Flector;

    self_737 = self_736;
    other_665 = other_664;
    let _e4: HomogeneousMagnitude = self_737;
    let _e8: Flector = other_665;
    let _e11: HomogeneousMagnitude = self_737;
    let _e15: Flector = other_665;
    return Flector((vec4<f32>(_e4.g0_.x) * _e8.g0_), (vec4<f32>(_e11.g0_.x) * _e15.g1_));
}

fn homogeneous_magnitude_flector_join(self_738: HomogeneousMagnitude, other_666: Flector) -> Flector {
    var self_739: HomogeneousMagnitude;
    var other_667: Flector;

    self_739 = self_738;
    other_667 = other_666;
    let _e4: HomogeneousMagnitude = self_739;
    let _e8: Flector = other_667;
    let _e11: HomogeneousMagnitude = self_739;
    let _e15: Flector = other_667;
    return Flector((vec4<f32>(_e4.g0_.x) * _e8.g0_), (vec4<f32>(_e11.g0_.x) * _e15.g1_));
}

fn homogeneous_magnitude_flector_inner_product(self_740: HomogeneousMagnitude, other_668: Flector) -> Flector {
    var self_741: HomogeneousMagnitude;
    var other_669: Flector;

    self_741 = self_740;
    other_669 = other_668;
    let _e4: HomogeneousMagnitude = self_741;
    let _e8: Flector = other_669;
    let _e11: HomogeneousMagnitude = self_741;
    let _e14: HomogeneousMagnitude = self_741;
    let _e17: HomogeneousMagnitude = self_741;
    let _e20: HomogeneousMagnitude = self_741;
    let _e24: Flector = other_669;
    let _e36: HomogeneousMagnitude = self_741;
    let _e40: Flector = other_669;
    let _e43: HomogeneousMagnitude = self_741;
    let _e46: HomogeneousMagnitude = self_741;
    let _e49: HomogeneousMagnitude = self_741;
    let _e52: HomogeneousMagnitude = self_741;
    let _e56: Flector = other_669;
    return Flector(((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.x, _e14.g0_.x, _e17.g0_.x, _e20.g0_.y) * _e24.g1_.xxxw) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))), ((vec4<f32>(_e36.g0_.x) * _e40.g1_) + ((vec4<f32>(_e43.g0_.y, _e46.g0_.y, _e49.g0_.y, _e52.g0_.x) * _e56.g0_.xyzx) * vec4<f32>(-(1.0), -(1.0), -(1.0), 0.0))));
}

fn homogeneous_magnitude_flector_geometric_anti_product(self_742: HomogeneousMagnitude, other_670: Flector) -> Flector {
    var self_743: HomogeneousMagnitude;
    var other_671: Flector;

    self_743 = self_742;
    other_671 = other_670;
    let _e4: HomogeneousMagnitude = self_743;
    let _e8: Flector = other_671;
    let _e11: HomogeneousMagnitude = self_743;
    let _e15: Flector = other_671;
    let _e26: HomogeneousMagnitude = self_743;
    let _e30: Flector = other_671;
    let _e33: HomogeneousMagnitude = self_743;
    let _e37: Flector = other_671;
    return Flector(((vec4<f32>(_e4.g0_.y) * _e8.g0_) + ((vec4<f32>(_e11.g0_.x) * _e15.g1_.xyzx) * vec4<f32>(1.0, 1.0, 1.0, 0.0))), ((vec4<f32>(_e26.g0_.y) * _e30.g1_) + ((vec4<f32>(_e33.g0_.x) * _e37.g0_.xxxw) * vec4<f32>(0.0, 0.0, 0.0, 1.0))));
}

fn homogeneous_magnitude_flector_inner_anti_product(self_744: HomogeneousMagnitude, other_672: Flector) -> Flector {
    var self_745: HomogeneousMagnitude;
    var other_673: Flector;

    self_745 = self_744;
    other_673 = other_672;
    let _e4: HomogeneousMagnitude = self_745;
    let _e8: Flector = other_673;
    let _e11: HomogeneousMagnitude = self_745;
    let _e15: Flector = other_673;
    let _e26: HomogeneousMagnitude = self_745;
    let _e30: Flector = other_673;
    let _e33: HomogeneousMagnitude = self_745;
    let _e37: Flector = other_673;
    return Flector(((vec4<f32>(_e4.g0_.y) * _e8.g0_) + ((vec4<f32>(_e11.g0_.x) * _e15.g1_.xyzx) * vec4<f32>(1.0, 1.0, 1.0, 0.0))), ((vec4<f32>(_e26.g0_.y) * _e30.g1_) + ((vec4<f32>(_e33.g0_.x) * _e37.g0_.xxxw) * vec4<f32>(0.0, 0.0, 0.0, 1.0))));
}

fn homogeneous_magnitude_flector_left_contraction(self_746: HomogeneousMagnitude, other_674: Flector) -> Flector {
    var self_747: HomogeneousMagnitude;
    var other_675: Flector;

    self_747 = self_746;
    other_675 = other_674;
    let _e4: HomogeneousMagnitude = self_747;
    let _e8: Flector = other_675;
    let _e11: HomogeneousMagnitude = self_747;
    let _e15: Flector = other_675;
    return Flector((vec4<f32>(_e4.g0_.x) * _e8.g0_), (vec4<f32>(_e11.g0_.x) * _e15.g1_));
}

fn homogeneous_magnitude_flector_left_anti_contraction(self_748: HomogeneousMagnitude, other_676: Flector) -> Flector {
    var self_749: HomogeneousMagnitude;
    var other_677: Flector;

    self_749 = self_748;
    other_677 = other_676;
    let _e4: HomogeneousMagnitude = self_749;
    let _e8: Flector = other_677;
    let _e11: HomogeneousMagnitude = self_749;
    let _e15: Flector = other_677;
    return Flector((vec4<f32>(_e4.g0_.y) * _e8.g0_), (vec4<f32>(_e11.g0_.y) * _e15.g1_));
}

fn homogeneous_magnitude_multi_vector_scalar_product(self_750: HomogeneousMagnitude, other_678: MultiVector) -> Scalar {
    var self_751: HomogeneousMagnitude;
    var other_679: MultiVector;

    self_751 = self_750;
    other_679 = other_678;
    let _e4: HomogeneousMagnitude = self_751;
    let _e7: MultiVector = other_679;
    return Scalar((_e4.g0_.x * _e7.g0_.x));
}

fn homogeneous_magnitude_multi_vector_dot(self_752: HomogeneousMagnitude, other_680: MultiVector) -> Scalar {
    var self_753: HomogeneousMagnitude;
    var other_681: MultiVector;

    self_753 = self_752;
    other_681 = other_680;
    let _e4: HomogeneousMagnitude = self_753;
    let _e7: MultiVector = other_681;
    return Scalar((_e4.g0_.x * _e7.g0_.x));
}

fn homogeneous_magnitude_multi_vector_anti_scalar_product(self_754: HomogeneousMagnitude, other_682: MultiVector) -> AntiScalar {
    var self_755: HomogeneousMagnitude;
    var other_683: MultiVector;

    self_755 = self_754;
    other_683 = other_682;
    let _e4: HomogeneousMagnitude = self_755;
    let _e7: MultiVector = other_683;
    return AntiScalar((_e4.g0_.y * _e7.g0_.y));
}

fn homogeneous_magnitude_multi_vector_anti_dot(self_756: HomogeneousMagnitude, other_684: MultiVector) -> AntiScalar {
    var self_757: HomogeneousMagnitude;
    var other_685: MultiVector;

    self_757 = self_756;
    other_685 = other_684;
    let _e4: HomogeneousMagnitude = self_757;
    let _e7: MultiVector = other_685;
    return AntiScalar((_e4.g0_.y * _e7.g0_.y));
}

fn homogeneous_magnitude_squared_magnitude(self_758: HomogeneousMagnitude) -> Scalar {
    var self_759: HomogeneousMagnitude;

    self_759 = self_758;
    let _e2: HomogeneousMagnitude = self_759;
    let _e3: HomogeneousMagnitude = self_759;
    let _e4: HomogeneousMagnitude = homogeneous_magnitude_reversal(_e3);
    let _e5: Scalar = homogeneous_magnitude_homogeneous_magnitude_scalar_product(_e2, _e4);
    return _e5;
}

fn homogeneous_magnitude_magnitude(self_760: HomogeneousMagnitude) -> Scalar {
    var self_761: HomogeneousMagnitude;

    self_761 = self_760;
    let _e2: HomogeneousMagnitude = self_761;
    let _e3: Scalar = homogeneous_magnitude_squared_magnitude(_e2);
    return Scalar(sqrt(_e3.g0_));
}

fn homogeneous_magnitude_bulk_norm(self_762: HomogeneousMagnitude) -> Scalar {
    var self_763: HomogeneousMagnitude;

    self_763 = self_762;
    let _e2: HomogeneousMagnitude = self_763;
    let _e3: Scalar = homogeneous_magnitude_squared_magnitude(_e2);
    return Scalar(sqrt(_e3.g0_));
}

fn homogeneous_magnitude_squared_anti_magnitude(self_764: HomogeneousMagnitude) -> AntiScalar {
    var self_765: HomogeneousMagnitude;

    self_765 = self_764;
    let _e2: HomogeneousMagnitude = self_765;
    let _e3: HomogeneousMagnitude = self_765;
    let _e4: HomogeneousMagnitude = homogeneous_magnitude_anti_reversal(_e3);
    let _e5: AntiScalar = homogeneous_magnitude_homogeneous_magnitude_anti_scalar_product(_e2, _e4);
    return _e5;
}

fn homogeneous_magnitude_weight_norm(self_766: HomogeneousMagnitude) -> AntiScalar {
    var self_767: HomogeneousMagnitude;

    self_767 = self_766;
    let _e2: HomogeneousMagnitude = self_767;
    let _e3: AntiScalar = homogeneous_magnitude_squared_anti_magnitude(_e2);
    return AntiScalar(sqrt(_e3.g0_));
}

fn homogeneous_magnitude_geometric_norm(self_768: HomogeneousMagnitude) -> HomogeneousMagnitude {
    var self_769: HomogeneousMagnitude;

    self_769 = self_768;
    let _e2: HomogeneousMagnitude = self_769;
    let _e3: Scalar = homogeneous_magnitude_bulk_norm(_e2);
    let _e4: HomogeneousMagnitude = self_769;
    let _e5: AntiScalar = homogeneous_magnitude_weight_norm(_e4);
    let _e6: HomogeneousMagnitude = scalar_anti_scalar_add(_e3, _e5);
    return _e6;
}

fn homogeneous_magnitude_scale(self_770: HomogeneousMagnitude, other_686: f32) -> HomogeneousMagnitude {
    var self_771: HomogeneousMagnitude;
    var other_687: f32;

    self_771 = self_770;
    other_687 = other_686;
    let _e4: HomogeneousMagnitude = self_771;
    let _e5: f32 = other_687;
    let _e7: HomogeneousMagnitude = homogeneous_magnitude_scalar_geometric_product(_e4, Scalar(_e5));
    return _e7;
}

fn homogeneous_magnitude_signum(self_772: HomogeneousMagnitude) -> HomogeneousMagnitude {
    var self_773: HomogeneousMagnitude;

    self_773 = self_772;
    let _e2: HomogeneousMagnitude = self_773;
    let _e3: HomogeneousMagnitude = self_773;
    let _e4: Scalar = homogeneous_magnitude_magnitude(_e3);
    let _e9: HomogeneousMagnitude = homogeneous_magnitude_scalar_geometric_product(_e2, Scalar((1.0 / _e4.g0_)));
    return _e9;
}

fn homogeneous_magnitude_inverse(self_774: HomogeneousMagnitude) -> HomogeneousMagnitude {
    var self_775: HomogeneousMagnitude;

    self_775 = self_774;
    let _e2: HomogeneousMagnitude = self_775;
    let _e3: HomogeneousMagnitude = homogeneous_magnitude_reversal(_e2);
    let _e4: HomogeneousMagnitude = self_775;
    let _e5: Scalar = homogeneous_magnitude_squared_magnitude(_e4);
    let _e10: HomogeneousMagnitude = homogeneous_magnitude_scalar_geometric_product(_e3, Scalar((1.0 / _e5.g0_)));
    return _e10;
}

fn homogeneous_magnitude_unitize(self_776: HomogeneousMagnitude) -> HomogeneousMagnitude {
    var self_777: HomogeneousMagnitude;

    self_777 = self_776;
    let _e2: HomogeneousMagnitude = self_777;
    let _e3: HomogeneousMagnitude = self_777;
    let _e4: AntiScalar = homogeneous_magnitude_weight_norm(_e3);
    let _e9: HomogeneousMagnitude = homogeneous_magnitude_scalar_geometric_product(_e2, Scalar((1.0 / _e4.g0_)));
    return _e9;
}

fn homogeneous_magnitude_attitude(self_778: HomogeneousMagnitude) -> Plane {
    var self_779: HomogeneousMagnitude;

    self_779 = self_778;
    let _e2: HomogeneousMagnitude = self_779;
    let _e9: Plane = homogeneous_magnitude_plane_regressive_product(_e2, Plane(vec4<f32>(0.0, 0.0, 0.0, 1.0)));
    return _e9;
}

fn point_zero() -> Point {
    return Point(vec4<f32>(0.0));
}

fn point_one() -> Point {
    return Point(vec4<f32>(0.0));
}

fn point_grade(self_780: Point) -> i32 {
    return 1;
}

fn point_anti_grade(self_781: Point) -> i32 {
    return 3;
}

fn point_neg(self_782: Point) -> Point {
    var self_783: Point;

    self_783 = self_782;
    let _e2: Point = self_783;
    return Point((_e2.g0_ * vec4<f32>(-(1.0))));
}

fn point_automorphism(self_784: Point) -> Point {
    var self_785: Point;

    self_785 = self_784;
    let _e2: Point = self_785;
    return Point((_e2.g0_ * vec4<f32>(-(1.0))));
}

fn point_reversal(self_786: Point) -> Point {
    var self_787: Point;

    self_787 = self_786;
    let _e2: Point = self_787;
    return Point(_e2.g0_);
}

fn point_conjugation(self_788: Point) -> Point {
    var self_789: Point;

    self_789 = self_788;
    let _e2: Point = self_789;
    return Point((_e2.g0_ * vec4<f32>(-(1.0))));
}

fn point_dual(self_790: Point) -> Plane {
    var self_791: Point;

    self_791 = self_790;
    let _e2: Point = self_791;
    return Plane(_e2.g0_);
}

fn point_anti_reversal(self_792: Point) -> Point {
    var self_793: Point;

    self_793 = self_792;
    let _e2: Point = self_793;
    return Point((_e2.g0_ * vec4<f32>(-(1.0))));
}

fn point_right_complement(self_794: Point) -> Plane {
    var self_795: Point;

    self_795 = self_794;
    let _e2: Point = self_795;
    return Plane(_e2.g0_);
}

fn point_left_complement(self_796: Point) -> Plane {
    var self_797: Point;

    self_797 = self_796;
    let _e2: Point = self_797;
    return Plane((_e2.g0_ * vec4<f32>(-(1.0))));
}

fn point_double_complement(self_798: Point) -> Point {
    var self_799: Point;

    self_799 = self_798;
    let _e2: Point = self_799;
    return Point((_e2.g0_ * vec4<f32>(-(1.0))));
}

fn point_scalar_geometric_product(self_800: Point, other_688: Scalar) -> Point {
    var self_801: Point;
    var other_689: Scalar;

    self_801 = self_800;
    other_689 = other_688;
    let _e4: Point = self_801;
    let _e6: Scalar = other_689;
    return Point((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn point_scalar_outer_product(self_802: Point, other_690: Scalar) -> Point {
    var self_803: Point;
    var other_691: Scalar;

    self_803 = self_802;
    other_691 = other_690;
    let _e4: Point = self_803;
    let _e6: Scalar = other_691;
    return Point((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn point_scalar_wedge(self_804: Point, other_692: Scalar) -> Point {
    var self_805: Point;
    var other_693: Scalar;

    self_805 = self_804;
    other_693 = other_692;
    let _e4: Point = self_805;
    let _e6: Scalar = other_693;
    return Point((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn point_scalar_join(self_806: Point, other_694: Scalar) -> Point {
    var self_807: Point;
    var other_695: Scalar;

    self_807 = self_806;
    other_695 = other_694;
    let _e4: Point = self_807;
    let _e6: Scalar = other_695;
    return Point((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn point_scalar_inner_product(self_808: Point, other_696: Scalar) -> Point {
    var self_809: Point;
    var other_697: Scalar;

    self_809 = self_808;
    other_697 = other_696;
    let _e4: Point = self_809;
    let _e6: Scalar = other_697;
    return Point((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn point_scalar_right_contraction(self_810: Point, other_698: Scalar) -> Point {
    var self_811: Point;
    var other_699: Scalar;

    self_811 = self_810;
    other_699 = other_698;
    let _e4: Point = self_811;
    let _e6: Scalar = other_699;
    return Point((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn point_anti_scalar_regressive_product(self_812: Point, other_700: AntiScalar) -> Point {
    var self_813: Point;
    var other_701: AntiScalar;

    self_813 = self_812;
    other_701 = other_700;
    let _e4: Point = self_813;
    let _e6: AntiScalar = other_701;
    return Point((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn point_anti_scalar_anti_wedge(self_814: Point, other_702: AntiScalar) -> Point {
    var self_815: Point;
    var other_703: AntiScalar;

    self_815 = self_814;
    other_703 = other_702;
    let _e4: Point = self_815;
    let _e6: AntiScalar = other_703;
    return Point((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn point_anti_scalar_meet(self_816: Point, other_704: AntiScalar) -> Point {
    var self_817: Point;
    var other_705: AntiScalar;

    self_817 = self_816;
    other_705 = other_704;
    let _e4: Point = self_817;
    let _e6: AntiScalar = other_705;
    return Point((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn point_anti_scalar_geometric_anti_product(self_818: Point, other_706: AntiScalar) -> Point {
    var self_819: Point;
    var other_707: AntiScalar;

    self_819 = self_818;
    other_707 = other_706;
    let _e4: Point = self_819;
    let _e6: AntiScalar = other_707;
    return Point((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn point_anti_scalar_inner_anti_product(self_820: Point, other_708: AntiScalar) -> Point {
    var self_821: Point;
    var other_709: AntiScalar;

    self_821 = self_820;
    other_709 = other_708;
    let _e4: Point = self_821;
    let _e6: AntiScalar = other_709;
    return Point((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn point_anti_scalar_right_anti_contraction(self_822: Point, other_710: AntiScalar) -> Point {
    var self_823: Point;
    var other_711: AntiScalar;

    self_823 = self_822;
    other_711 = other_710;
    let _e4: Point = self_823;
    let _e6: AntiScalar = other_711;
    return Point((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn point_homogeneous_magnitude_regressive_product(self_824: Point, other_712: HomogeneousMagnitude) -> Point {
    var self_825: Point;
    var other_713: HomogeneousMagnitude;

    self_825 = self_824;
    other_713 = other_712;
    let _e4: Point = self_825;
    let _e6: HomogeneousMagnitude = other_713;
    return Point((_e4.g0_ * vec4<f32>(_e6.g0_.y)));
}

fn point_homogeneous_magnitude_anti_wedge(self_826: Point, other_714: HomogeneousMagnitude) -> Point {
    var self_827: Point;
    var other_715: HomogeneousMagnitude;

    self_827 = self_826;
    other_715 = other_714;
    let _e4: Point = self_827;
    let _e6: HomogeneousMagnitude = other_715;
    return Point((_e4.g0_ * vec4<f32>(_e6.g0_.y)));
}

fn point_homogeneous_magnitude_meet(self_828: Point, other_716: HomogeneousMagnitude) -> Point {
    var self_829: Point;
    var other_717: HomogeneousMagnitude;

    self_829 = self_828;
    other_717 = other_716;
    let _e4: Point = self_829;
    let _e6: HomogeneousMagnitude = other_717;
    return Point((_e4.g0_ * vec4<f32>(_e6.g0_.y)));
}

fn point_homogeneous_magnitude_outer_product(self_830: Point, other_718: HomogeneousMagnitude) -> Point {
    var self_831: Point;
    var other_719: HomogeneousMagnitude;

    self_831 = self_830;
    other_719 = other_718;
    let _e4: Point = self_831;
    let _e6: HomogeneousMagnitude = other_719;
    return Point((_e4.g0_ * vec4<f32>(_e6.g0_.x)));
}

fn point_homogeneous_magnitude_wedge(self_832: Point, other_720: HomogeneousMagnitude) -> Point {
    var self_833: Point;
    var other_721: HomogeneousMagnitude;

    self_833 = self_832;
    other_721 = other_720;
    let _e4: Point = self_833;
    let _e6: HomogeneousMagnitude = other_721;
    return Point((_e4.g0_ * vec4<f32>(_e6.g0_.x)));
}

fn point_homogeneous_magnitude_join(self_834: Point, other_722: HomogeneousMagnitude) -> Point {
    var self_835: Point;
    var other_723: HomogeneousMagnitude;

    self_835 = self_834;
    other_723 = other_722;
    let _e4: Point = self_835;
    let _e6: HomogeneousMagnitude = other_723;
    return Point((_e4.g0_ * vec4<f32>(_e6.g0_.x)));
}

fn point_homogeneous_magnitude_right_contraction(self_836: Point, other_724: HomogeneousMagnitude) -> Point {
    var self_837: Point;
    var other_725: HomogeneousMagnitude;

    self_837 = self_836;
    other_725 = other_724;
    let _e4: Point = self_837;
    let _e6: HomogeneousMagnitude = other_725;
    return Point((_e4.g0_ * vec4<f32>(_e6.g0_.x)));
}

fn point_homogeneous_magnitude_right_anti_contraction(self_838: Point, other_726: HomogeneousMagnitude) -> Point {
    var self_839: Point;
    var other_727: HomogeneousMagnitude;

    self_839 = self_838;
    other_727 = other_726;
    let _e4: Point = self_839;
    let _e6: HomogeneousMagnitude = other_727;
    return Point((_e4.g0_ * vec4<f32>(_e6.g0_.y)));
}

fn point_point_add(self_840: Point, other_728: Point) -> Point {
    var self_841: Point;
    var other_729: Point;

    self_841 = self_840;
    other_729 = other_728;
    let _e4: Point = self_841;
    let _e6: Point = other_729;
    return Point((_e4.g0_ + _e6.g0_));
}

fn point_point_sub(self_842: Point, other_730: Point) -> Point {
    var self_843: Point;
    var other_731: Point;

    self_843 = self_842;
    other_731 = other_730;
    let _e4: Point = self_843;
    let _e6: Point = other_731;
    return Point((_e4.g0_ - _e6.g0_));
}

fn point_point_mul(self_844: Point, other_732: Point) -> Point {
    var self_845: Point;
    var other_733: Point;

    self_845 = self_844;
    other_733 = other_732;
    let _e4: Point = self_845;
    let _e6: Point = other_733;
    return Point((_e4.g0_ * _e6.g0_));
}

fn point_point_div(self_846: Point, other_734: Point) -> Point {
    var self_847: Point;
    var other_735: Point;

    self_847 = self_846;
    other_735 = other_734;
    let _e4: Point = self_847;
    let _e7: Point = self_847;
    let _e10: Point = self_847;
    let _e13: Point = self_847;
    let _e23: Point = other_735;
    let _e26: Point = other_735;
    let _e29: Point = other_735;
    let _e32: Point = other_735;
    return Point((((vec4<f32>(_e4.g0_.x, _e7.g0_.y, _e10.g0_.z, _e13.g0_.w) * vec4<f32>(1.0, 1.0, 1.0, 1.0)) / vec4<f32>(_e23.g0_.x, _e26.g0_.y, _e29.g0_.z, _e32.g0_.w)) * vec4<f32>(1.0, 1.0, 1.0, 1.0)));
}

fn point_point_outer_product(self_848: Point, other_736: Point) -> Line {
    var self_849: Point;
    var other_737: Point;

    self_849 = self_848;
    other_737 = other_736;
    let _e4: Point = self_849;
    let _e8: Point = other_737;
    let _e11: Point = other_737;
    let _e14: Point = other_737;
    let _e19: Point = self_849;
    let _e22: Point = self_849;
    let _e25: Point = self_849;
    let _e29: Point = other_737;
    let _e39: Point = self_849;
    let _e43: Point = other_737;
    let _e46: Point = other_737;
    let _e49: Point = other_737;
    let _e60: Point = self_849;
    let _e64: Point = other_737;
    let _e67: Point = other_737;
    let _e70: Point = other_737;
    let _e82: Point = self_849;
    let _e86: Point = other_737;
    let _e89: Point = other_737;
    let _e92: Point = other_737;
    return Line(((vec3<f32>(_e4.g0_.w) * vec3<f32>(_e8.g0_.x, _e11.g0_.y, _e14.g0_.z)) + ((vec3<f32>(_e19.g0_.x, _e22.g0_.y, _e25.g0_.z) * vec3<f32>(_e29.g0_.w)) * vec3<f32>(-(1.0)))), ((((vec3<f32>(_e39.g0_.y) * vec3<f32>(_e43.g0_.z, _e46.g0_.z, _e49.g0_.x)) * vec3<f32>(1.0, 0.0, -(1.0))) + ((vec3<f32>(_e60.g0_.z) * vec3<f32>(_e64.g0_.y, _e67.g0_.x, _e70.g0_.y)) * vec3<f32>(-(1.0), 1.0, 0.0))) + ((vec3<f32>(_e82.g0_.x) * vec3<f32>(_e86.g0_.x, _e89.g0_.z, _e92.g0_.y)) * vec3<f32>(0.0, -(1.0), 1.0))));
}

fn point_point_wedge(self_850: Point, other_738: Point) -> Line {
    var self_851: Point;
    var other_739: Point;

    self_851 = self_850;
    other_739 = other_738;
    let _e4: Point = self_851;
    let _e8: Point = other_739;
    let _e11: Point = other_739;
    let _e14: Point = other_739;
    let _e19: Point = self_851;
    let _e22: Point = self_851;
    let _e25: Point = self_851;
    let _e29: Point = other_739;
    let _e39: Point = self_851;
    let _e43: Point = other_739;
    let _e46: Point = other_739;
    let _e49: Point = other_739;
    let _e60: Point = self_851;
    let _e64: Point = other_739;
    let _e67: Point = other_739;
    let _e70: Point = other_739;
    let _e82: Point = self_851;
    let _e86: Point = other_739;
    let _e89: Point = other_739;
    let _e92: Point = other_739;
    return Line(((vec3<f32>(_e4.g0_.w) * vec3<f32>(_e8.g0_.x, _e11.g0_.y, _e14.g0_.z)) + ((vec3<f32>(_e19.g0_.x, _e22.g0_.y, _e25.g0_.z) * vec3<f32>(_e29.g0_.w)) * vec3<f32>(-(1.0)))), ((((vec3<f32>(_e39.g0_.y) * vec3<f32>(_e43.g0_.z, _e46.g0_.z, _e49.g0_.x)) * vec3<f32>(1.0, 0.0, -(1.0))) + ((vec3<f32>(_e60.g0_.z) * vec3<f32>(_e64.g0_.y, _e67.g0_.x, _e70.g0_.y)) * vec3<f32>(-(1.0), 1.0, 0.0))) + ((vec3<f32>(_e82.g0_.x) * vec3<f32>(_e86.g0_.x, _e89.g0_.z, _e92.g0_.y)) * vec3<f32>(0.0, -(1.0), 1.0))));
}

fn point_point_join(self_852: Point, other_740: Point) -> Line {
    var self_853: Point;
    var other_741: Point;

    self_853 = self_852;
    other_741 = other_740;
    let _e4: Point = self_853;
    let _e8: Point = other_741;
    let _e11: Point = other_741;
    let _e14: Point = other_741;
    let _e19: Point = self_853;
    let _e22: Point = self_853;
    let _e25: Point = self_853;
    let _e29: Point = other_741;
    let _e39: Point = self_853;
    let _e43: Point = other_741;
    let _e46: Point = other_741;
    let _e49: Point = other_741;
    let _e60: Point = self_853;
    let _e64: Point = other_741;
    let _e67: Point = other_741;
    let _e70: Point = other_741;
    let _e82: Point = self_853;
    let _e86: Point = other_741;
    let _e89: Point = other_741;
    let _e92: Point = other_741;
    return Line(((vec3<f32>(_e4.g0_.w) * vec3<f32>(_e8.g0_.x, _e11.g0_.y, _e14.g0_.z)) + ((vec3<f32>(_e19.g0_.x, _e22.g0_.y, _e25.g0_.z) * vec3<f32>(_e29.g0_.w)) * vec3<f32>(-(1.0)))), ((((vec3<f32>(_e39.g0_.y) * vec3<f32>(_e43.g0_.z, _e46.g0_.z, _e49.g0_.x)) * vec3<f32>(1.0, 0.0, -(1.0))) + ((vec3<f32>(_e60.g0_.z) * vec3<f32>(_e64.g0_.y, _e67.g0_.x, _e70.g0_.y)) * vec3<f32>(-(1.0), 1.0, 0.0))) + ((vec3<f32>(_e82.g0_.x) * vec3<f32>(_e86.g0_.x, _e89.g0_.z, _e92.g0_.y)) * vec3<f32>(0.0, -(1.0), 1.0))));
}

fn point_point_inner_product(self_854: Point, other_742: Point) -> Scalar {
    var self_855: Point;
    var other_743: Point;

    self_855 = self_854;
    other_743 = other_742;
    let _e4: Point = self_855;
    let _e7: Point = other_743;
    let _e11: Point = self_855;
    let _e14: Point = other_743;
    let _e19: Point = self_855;
    let _e22: Point = other_743;
    return Scalar((((_e4.g0_.x * _e7.g0_.x) + (_e11.g0_.y * _e14.g0_.y)) + (_e19.g0_.z * _e22.g0_.z)));
}

fn point_point_geometric_anti_product(self_856: Point, other_744: Point) -> Translator {
    var self_857: Point;
    var other_745: Point;

    self_857 = self_856;
    other_745 = other_744;
    let _e4: Point = self_857;
    let _e8: Point = other_745;
    let _e18: Point = self_857;
    let _e21: Point = other_745;
    return Translator((((vec4<f32>(_e4.g0_.w) * _e8.g0_) * vec4<f32>(1.0, 1.0, 1.0, -(1.0))) + ((_e18.g0_.xyzx * _e21.g0_.wwwx) * vec4<f32>(-(1.0), -(1.0), -(1.0), 0.0))));
}

fn point_point_inner_anti_product(self_858: Point, other_746: Point) -> AntiScalar {
    var self_859: Point;
    var other_747: Point;

    self_859 = self_858;
    other_747 = other_746;
    let _e5: Point = self_859;
    let _e8: Point = other_747;
    return AntiScalar((0.0 - (_e5.g0_.w * _e8.g0_.w)));
}

fn point_point_left_contraction(self_860: Point, other_748: Point) -> Scalar {
    var self_861: Point;
    var other_749: Point;

    self_861 = self_860;
    other_749 = other_748;
    let _e4: Point = self_861;
    let _e7: Point = other_749;
    let _e11: Point = self_861;
    let _e14: Point = other_749;
    let _e19: Point = self_861;
    let _e22: Point = other_749;
    return Scalar((((_e4.g0_.x * _e7.g0_.x) + (_e11.g0_.y * _e14.g0_.y)) + (_e19.g0_.z * _e22.g0_.z)));
}

fn point_point_right_contraction(self_862: Point, other_750: Point) -> Scalar {
    var self_863: Point;
    var other_751: Point;

    self_863 = self_862;
    other_751 = other_750;
    let _e4: Point = self_863;
    let _e7: Point = other_751;
    let _e11: Point = self_863;
    let _e14: Point = other_751;
    let _e19: Point = self_863;
    let _e22: Point = other_751;
    return Scalar((((_e4.g0_.x * _e7.g0_.x) + (_e11.g0_.y * _e14.g0_.y)) + (_e19.g0_.z * _e22.g0_.z)));
}

fn point_point_left_anti_contraction(self_864: Point, other_752: Point) -> AntiScalar {
    var self_865: Point;
    var other_753: Point;

    self_865 = self_864;
    other_753 = other_752;
    let _e5: Point = self_865;
    let _e8: Point = other_753;
    return AntiScalar((0.0 - (_e5.g0_.w * _e8.g0_.w)));
}

fn point_point_right_anti_contraction(self_866: Point, other_754: Point) -> AntiScalar {
    var self_867: Point;
    var other_755: Point;

    self_867 = self_866;
    other_755 = other_754;
    let _e5: Point = self_867;
    let _e8: Point = other_755;
    return AntiScalar((0.0 - (_e5.g0_.w * _e8.g0_.w)));
}

fn point_point_scalar_product(self_868: Point, other_756: Point) -> Scalar {
    var self_869: Point;
    var other_757: Point;

    self_869 = self_868;
    other_757 = other_756;
    let _e4: Point = self_869;
    let _e7: Point = other_757;
    let _e11: Point = self_869;
    let _e14: Point = other_757;
    let _e19: Point = self_869;
    let _e22: Point = other_757;
    return Scalar((((_e4.g0_.x * _e7.g0_.x) + (_e11.g0_.y * _e14.g0_.y)) + (_e19.g0_.z * _e22.g0_.z)));
}

fn point_point_dot(self_870: Point, other_758: Point) -> Scalar {
    var self_871: Point;
    var other_759: Point;

    self_871 = self_870;
    other_759 = other_758;
    let _e4: Point = self_871;
    let _e7: Point = other_759;
    let _e11: Point = self_871;
    let _e14: Point = other_759;
    let _e19: Point = self_871;
    let _e22: Point = other_759;
    return Scalar((((_e4.g0_.x * _e7.g0_.x) + (_e11.g0_.y * _e14.g0_.y)) + (_e19.g0_.z * _e22.g0_.z)));
}

fn point_point_anti_scalar_product(self_872: Point, other_760: Point) -> AntiScalar {
    var self_873: Point;
    var other_761: Point;

    self_873 = self_872;
    other_761 = other_760;
    let _e5: Point = self_873;
    let _e8: Point = other_761;
    return AntiScalar((0.0 - (_e5.g0_.w * _e8.g0_.w)));
}

fn point_point_anti_dot(self_874: Point, other_762: Point) -> AntiScalar {
    var self_875: Point;
    var other_763: Point;

    self_875 = self_874;
    other_763 = other_762;
    let _e5: Point = self_875;
    let _e8: Point = other_763;
    return AntiScalar((0.0 - (_e5.g0_.w * _e8.g0_.w)));
}

fn point_line_geometric_product(self_876: Point, other_764: Line) -> Flector {
    var self_877: Point;
    var other_765: Line;

    self_877 = self_876;
    other_765 = other_764;
    let _e4: Point = self_877;
    let _e8: Line = other_765;
    let _e11: Line = other_765;
    let _e14: Line = other_765;
    let _e17: Line = other_765;
    let _e30: Point = self_877;
    let _e34: Line = other_765;
    let _e37: Line = other_765;
    let _e40: Line = other_765;
    let _e43: Line = other_765;
    let _e57: Point = self_877;
    let _e61: Line = other_765;
    let _e64: Line = other_765;
    let _e67: Line = other_765;
    let _e70: Line = other_765;
    let _e84: Point = self_877;
    let _e88: Line = other_765;
    let _e91: Line = other_765;
    let _e94: Line = other_765;
    let _e97: Line = other_765;
    let _e110: Point = self_877;
    let _e114: Line = other_765;
    let _e117: Line = other_765;
    let _e120: Line = other_765;
    let _e123: Line = other_765;
    let _e137: Point = self_877;
    let _e141: Line = other_765;
    let _e144: Line = other_765;
    let _e147: Line = other_765;
    let _e150: Line = other_765;
    let _e162: Point = self_877;
    let _e166: Line = other_765;
    let _e169: Line = other_765;
    let _e172: Line = other_765;
    let _e175: Line = other_765;
    return Flector(((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g1_.z, _e11.g1_.z, _e14.g1_.x, _e17.g0_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e30.g0_.z) * vec4<f32>(_e34.g1_.y, _e37.g1_.x, _e40.g1_.y, _e43.g0_.z)) * vec4<f32>(1.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e57.g0_.x) * vec4<f32>(_e61.g1_.x, _e64.g1_.z, _e67.g1_.y, _e70.g0_.x)) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))), (((((vec4<f32>(_e84.g0_.y) * vec4<f32>(_e88.g0_.z, _e91.g0_.z, _e94.g0_.x, _e97.g1_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e110.g0_.z) * vec4<f32>(_e114.g0_.y, _e117.g0_.x, _e120.g0_.y, _e123.g1_.z)) * vec4<f32>(1.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e137.g0_.w) * vec4<f32>(_e141.g1_.x, _e144.g1_.y, _e147.g1_.z, _e150.g1_.x)) * vec4<f32>(1.0, 1.0, 1.0, 0.0))) + ((vec4<f32>(_e162.g0_.x) * vec4<f32>(_e166.g0_.x, _e169.g0_.z, _e172.g0_.y, _e175.g1_.x)) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))));
}

fn point_line_outer_product(self_878: Point, other_766: Line) -> Plane {
    var self_879: Point;
    var other_767: Line;

    self_879 = self_878;
    other_767 = other_766;
    let _e4: Point = self_879;
    let _e8: Line = other_767;
    let _e11: Line = other_767;
    let _e14: Line = other_767;
    let _e17: Line = other_767;
    let _e30: Point = self_879;
    let _e34: Line = other_767;
    let _e37: Line = other_767;
    let _e40: Line = other_767;
    let _e43: Line = other_767;
    let _e57: Point = self_879;
    let _e61: Line = other_767;
    let _e64: Line = other_767;
    let _e67: Line = other_767;
    let _e70: Line = other_767;
    let _e82: Point = self_879;
    let _e86: Line = other_767;
    let _e89: Line = other_767;
    let _e92: Line = other_767;
    let _e95: Line = other_767;
    return Plane((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.z, _e11.g0_.z, _e14.g0_.x, _e17.g1_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e30.g0_.z) * vec4<f32>(_e34.g0_.y, _e37.g0_.x, _e40.g0_.y, _e43.g1_.z)) * vec4<f32>(1.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e57.g0_.w) * vec4<f32>(_e61.g1_.x, _e64.g1_.y, _e67.g1_.z, _e70.g1_.x)) * vec4<f32>(1.0, 1.0, 1.0, 0.0))) + ((vec4<f32>(_e82.g0_.x) * vec4<f32>(_e86.g0_.x, _e89.g0_.z, _e92.g0_.y, _e95.g1_.x)) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))));
}

fn point_line_wedge(self_880: Point, other_768: Line) -> Plane {
    var self_881: Point;
    var other_769: Line;

    self_881 = self_880;
    other_769 = other_768;
    let _e4: Point = self_881;
    let _e8: Line = other_769;
    let _e11: Line = other_769;
    let _e14: Line = other_769;
    let _e17: Line = other_769;
    let _e30: Point = self_881;
    let _e34: Line = other_769;
    let _e37: Line = other_769;
    let _e40: Line = other_769;
    let _e43: Line = other_769;
    let _e57: Point = self_881;
    let _e61: Line = other_769;
    let _e64: Line = other_769;
    let _e67: Line = other_769;
    let _e70: Line = other_769;
    let _e82: Point = self_881;
    let _e86: Line = other_769;
    let _e89: Line = other_769;
    let _e92: Line = other_769;
    let _e95: Line = other_769;
    return Plane((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.z, _e11.g0_.z, _e14.g0_.x, _e17.g1_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e30.g0_.z) * vec4<f32>(_e34.g0_.y, _e37.g0_.x, _e40.g0_.y, _e43.g1_.z)) * vec4<f32>(1.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e57.g0_.w) * vec4<f32>(_e61.g1_.x, _e64.g1_.y, _e67.g1_.z, _e70.g1_.x)) * vec4<f32>(1.0, 1.0, 1.0, 0.0))) + ((vec4<f32>(_e82.g0_.x) * vec4<f32>(_e86.g0_.x, _e89.g0_.z, _e92.g0_.y, _e95.g1_.x)) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))));
}

fn point_line_join(self_882: Point, other_770: Line) -> Plane {
    var self_883: Point;
    var other_771: Line;

    self_883 = self_882;
    other_771 = other_770;
    let _e4: Point = self_883;
    let _e8: Line = other_771;
    let _e11: Line = other_771;
    let _e14: Line = other_771;
    let _e17: Line = other_771;
    let _e30: Point = self_883;
    let _e34: Line = other_771;
    let _e37: Line = other_771;
    let _e40: Line = other_771;
    let _e43: Line = other_771;
    let _e57: Point = self_883;
    let _e61: Line = other_771;
    let _e64: Line = other_771;
    let _e67: Line = other_771;
    let _e70: Line = other_771;
    let _e82: Point = self_883;
    let _e86: Line = other_771;
    let _e89: Line = other_771;
    let _e92: Line = other_771;
    let _e95: Line = other_771;
    return Plane((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.z, _e11.g0_.z, _e14.g0_.x, _e17.g1_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e30.g0_.z) * vec4<f32>(_e34.g0_.y, _e37.g0_.x, _e40.g0_.y, _e43.g1_.z)) * vec4<f32>(1.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e57.g0_.w) * vec4<f32>(_e61.g1_.x, _e64.g1_.y, _e67.g1_.z, _e70.g1_.x)) * vec4<f32>(1.0, 1.0, 1.0, 0.0))) + ((vec4<f32>(_e82.g0_.x) * vec4<f32>(_e86.g0_.x, _e89.g0_.z, _e92.g0_.y, _e95.g1_.x)) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))));
}

fn point_line_inner_product(self_884: Point, other_772: Line) -> Point {
    var self_885: Point;
    var other_773: Line;

    self_885 = self_884;
    other_773 = other_772;
    let _e4: Point = self_885;
    let _e8: Line = other_773;
    let _e11: Line = other_773;
    let _e14: Line = other_773;
    let _e17: Line = other_773;
    let _e30: Point = self_885;
    let _e34: Line = other_773;
    let _e37: Line = other_773;
    let _e40: Line = other_773;
    let _e43: Line = other_773;
    let _e57: Point = self_885;
    let _e61: Line = other_773;
    let _e64: Line = other_773;
    let _e67: Line = other_773;
    let _e70: Line = other_773;
    return Point(((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g1_.z, _e11.g1_.z, _e14.g1_.x, _e17.g0_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e30.g0_.z) * vec4<f32>(_e34.g1_.y, _e37.g1_.x, _e40.g1_.y, _e43.g0_.z)) * vec4<f32>(1.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e57.g0_.x) * vec4<f32>(_e61.g1_.x, _e64.g1_.z, _e67.g1_.y, _e70.g0_.x)) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))));
}

fn point_line_inner_anti_product(self_886: Point, other_774: Line) -> Plane {
    var self_887: Point;
    var other_775: Line;

    self_887 = self_886;
    other_775 = other_774;
    let _e4: Point = self_887;
    let _e8: Line = other_775;
    let _e20: Point = self_887;
    let _e24: Line = other_775;
    let _e37: Point = self_887;
    let _e40: Line = other_775;
    let _e43: Line = other_775;
    let _e46: Line = other_775;
    let _e49: Line = other_775;
    return Plane(((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0))) + ((vec4<f32>(_e20.g0_.z) * vec4<f32>(_e24.g0_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((_e37.g0_.wwwx * vec4<f32>(_e40.g0_.x, _e43.g0_.y, _e46.g0_.z, _e49.g0_.x)) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))));
}

fn point_line_left_contraction(self_888: Point, other_776: Line) -> Point {
    var self_889: Point;
    var other_777: Line;

    self_889 = self_888;
    other_777 = other_776;
    let _e4: Point = self_889;
    let _e8: Line = other_777;
    let _e11: Line = other_777;
    let _e14: Line = other_777;
    let _e17: Line = other_777;
    let _e30: Point = self_889;
    let _e34: Line = other_777;
    let _e37: Line = other_777;
    let _e40: Line = other_777;
    let _e43: Line = other_777;
    let _e57: Point = self_889;
    let _e61: Line = other_777;
    let _e64: Line = other_777;
    let _e67: Line = other_777;
    let _e70: Line = other_777;
    return Point(((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g1_.z, _e11.g1_.z, _e14.g1_.x, _e17.g0_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e30.g0_.z) * vec4<f32>(_e34.g1_.y, _e37.g1_.x, _e40.g1_.y, _e43.g0_.z)) * vec4<f32>(1.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e57.g0_.x) * vec4<f32>(_e61.g1_.x, _e64.g1_.z, _e67.g1_.y, _e70.g0_.x)) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))));
}

fn point_line_right_anti_contraction(self_890: Point, other_778: Line) -> Plane {
    var self_891: Point;
    var other_779: Line;

    self_891 = self_890;
    other_779 = other_778;
    let _e4: Point = self_891;
    let _e8: Line = other_779;
    let _e20: Point = self_891;
    let _e24: Line = other_779;
    let _e37: Point = self_891;
    let _e40: Line = other_779;
    let _e43: Line = other_779;
    let _e46: Line = other_779;
    let _e49: Line = other_779;
    return Plane(((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0))) + ((vec4<f32>(_e20.g0_.z) * vec4<f32>(_e24.g0_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((_e37.g0_.wwwx * vec4<f32>(_e40.g0_.x, _e43.g0_.y, _e46.g0_.z, _e49.g0_.x)) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))));
}

fn point_plane_add(self_892: Point, other_780: Plane) -> Flector {
    var self_893: Point;
    var other_781: Plane;

    self_893 = self_892;
    other_781 = other_780;
    let _e4: Point = self_893;
    let _e6: Plane = other_781;
    return Flector(_e4.g0_, _e6.g0_);
}

fn point_plane_sub(self_894: Point, other_782: Plane) -> Flector {
    var self_895: Point;
    var other_783: Plane;

    self_895 = self_894;
    other_783 = other_782;
    let _e4: Point = self_895;
    let _e8: Plane = other_783;
    return Flector(_e4.g0_, (vec4<f32>(0.0) - _e8.g0_));
}

fn point_plane_geometric_product(self_896: Point, other_784: Plane) -> Motor {
    var self_897: Point;
    var other_785: Plane;

    self_897 = self_896;
    other_785 = other_784;
    let _e4: Point = self_897;
    let _e8: Plane = other_785;
    let _e19: Point = self_897;
    let _e23: Plane = other_785;
    let _e35: Point = self_897;
    let _e39: Plane = other_785;
    let _e51: Point = self_897;
    let _e55: Plane = other_785;
    let _e67: Point = self_897;
    let _e70: Point = self_897;
    let _e73: Point = self_897;
    let _e77: Plane = other_785;
    return Motor((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.zzxy) * vec4<f32>(1.0, 0.0, -(1.0), 1.0)) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.yxyz) * vec4<f32>(-(1.0), 1.0, 0.0, 1.0))) + ((vec4<f32>(_e35.g0_.w) * vec4<f32>(_e39.g0_.w)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e51.g0_.x) * _e55.g0_.xzyx) * vec4<f32>(0.0, -(1.0), 1.0, 1.0))), ((vec3<f32>(_e67.g0_.x, _e70.g0_.y, _e73.g0_.z) * vec3<f32>(_e77.g0_.w)) * vec3<f32>(-(1.0))));
}

fn point_plane_regressive_product(self_898: Point, other_786: Plane) -> Scalar {
    var self_899: Point;
    var other_787: Plane;

    self_899 = self_898;
    other_787 = other_786;
    let _e4: Point = self_899;
    let _e7: Plane = other_787;
    let _e11: Point = self_899;
    let _e14: Plane = other_787;
    let _e19: Point = self_899;
    let _e22: Plane = other_787;
    let _e27: Point = self_899;
    let _e30: Plane = other_787;
    return Scalar(((((_e4.g0_.x * _e7.g0_.x) + (_e11.g0_.y * _e14.g0_.y)) + (_e19.g0_.z * _e22.g0_.z)) + (_e27.g0_.w * _e30.g0_.w)));
}

fn point_plane_anti_wedge(self_900: Point, other_788: Plane) -> Scalar {
    var self_901: Point;
    var other_789: Plane;

    self_901 = self_900;
    other_789 = other_788;
    let _e4: Point = self_901;
    let _e7: Plane = other_789;
    let _e11: Point = self_901;
    let _e14: Plane = other_789;
    let _e19: Point = self_901;
    let _e22: Plane = other_789;
    let _e27: Point = self_901;
    let _e30: Plane = other_789;
    return Scalar(((((_e4.g0_.x * _e7.g0_.x) + (_e11.g0_.y * _e14.g0_.y)) + (_e19.g0_.z * _e22.g0_.z)) + (_e27.g0_.w * _e30.g0_.w)));
}

fn point_plane_meet(self_902: Point, other_790: Plane) -> Scalar {
    var self_903: Point;
    var other_791: Plane;

    self_903 = self_902;
    other_791 = other_790;
    let _e4: Point = self_903;
    let _e7: Plane = other_791;
    let _e11: Point = self_903;
    let _e14: Plane = other_791;
    let _e19: Point = self_903;
    let _e22: Plane = other_791;
    let _e27: Point = self_903;
    let _e30: Plane = other_791;
    return Scalar(((((_e4.g0_.x * _e7.g0_.x) + (_e11.g0_.y * _e14.g0_.y)) + (_e19.g0_.z * _e22.g0_.z)) + (_e27.g0_.w * _e30.g0_.w)));
}

fn point_plane_outer_product(self_904: Point, other_792: Plane) -> AntiScalar {
    var self_905: Point;
    var other_793: Plane;

    self_905 = self_904;
    other_793 = other_792;
    let _e4: Point = self_905;
    let _e7: Plane = other_793;
    let _e11: Point = self_905;
    let _e14: Plane = other_793;
    let _e19: Point = self_905;
    let _e22: Plane = other_793;
    let _e27: Point = self_905;
    let _e30: Plane = other_793;
    return AntiScalar(((((_e4.g0_.x * _e7.g0_.x) + (_e11.g0_.y * _e14.g0_.y)) + (_e19.g0_.z * _e22.g0_.z)) + (_e27.g0_.w * _e30.g0_.w)));
}

fn point_plane_wedge(self_906: Point, other_794: Plane) -> AntiScalar {
    var self_907: Point;
    var other_795: Plane;

    self_907 = self_906;
    other_795 = other_794;
    let _e4: Point = self_907;
    let _e7: Plane = other_795;
    let _e11: Point = self_907;
    let _e14: Plane = other_795;
    let _e19: Point = self_907;
    let _e22: Plane = other_795;
    let _e27: Point = self_907;
    let _e30: Plane = other_795;
    return AntiScalar(((((_e4.g0_.x * _e7.g0_.x) + (_e11.g0_.y * _e14.g0_.y)) + (_e19.g0_.z * _e22.g0_.z)) + (_e27.g0_.w * _e30.g0_.w)));
}

fn point_plane_join(self_908: Point, other_796: Plane) -> AntiScalar {
    var self_909: Point;
    var other_797: Plane;

    self_909 = self_908;
    other_797 = other_796;
    let _e4: Point = self_909;
    let _e7: Plane = other_797;
    let _e11: Point = self_909;
    let _e14: Plane = other_797;
    let _e19: Point = self_909;
    let _e22: Plane = other_797;
    let _e27: Point = self_909;
    let _e30: Plane = other_797;
    return AntiScalar(((((_e4.g0_.x * _e7.g0_.x) + (_e11.g0_.y * _e14.g0_.y)) + (_e19.g0_.z * _e22.g0_.z)) + (_e27.g0_.w * _e30.g0_.w)));
}

fn point_plane_inner_product(self_910: Point, other_798: Plane) -> Line {
    var self_911: Point;
    var other_799: Plane;

    self_911 = self_910;
    other_799 = other_798;
    let _e4: Point = self_911;
    let _e8: Plane = other_799;
    let _e11: Plane = other_799;
    let _e14: Plane = other_799;
    let _e25: Point = self_911;
    let _e29: Plane = other_799;
    let _e32: Plane = other_799;
    let _e35: Plane = other_799;
    let _e47: Point = self_911;
    let _e51: Plane = other_799;
    let _e54: Plane = other_799;
    let _e57: Plane = other_799;
    let _e69: Point = self_911;
    let _e72: Point = self_911;
    let _e75: Point = self_911;
    let _e79: Plane = other_799;
    return Line(((((vec3<f32>(_e4.g0_.y) * vec3<f32>(_e8.g0_.z, _e11.g0_.z, _e14.g0_.x)) * vec3<f32>(1.0, 0.0, -(1.0))) + ((vec3<f32>(_e25.g0_.z) * vec3<f32>(_e29.g0_.y, _e32.g0_.x, _e35.g0_.y)) * vec3<f32>(-(1.0), 1.0, 0.0))) + ((vec3<f32>(_e47.g0_.x) * vec3<f32>(_e51.g0_.x, _e54.g0_.z, _e57.g0_.y)) * vec3<f32>(0.0, -(1.0), 1.0))), ((vec3<f32>(_e69.g0_.x, _e72.g0_.y, _e75.g0_.z) * vec3<f32>(_e79.g0_.w)) * vec3<f32>(-(1.0))));
}

fn point_plane_inner_anti_product(self_912: Point, other_800: Plane) -> Line {
    var self_913: Point;
    var other_801: Plane;

    self_913 = self_912;
    other_801 = other_800;
    let _e6: Point = self_913;
    let _e10: Plane = other_801;
    let _e13: Plane = other_801;
    let _e16: Plane = other_801;
    let _e22: Point = self_913;
    let _e26: Plane = other_801;
    let _e29: Plane = other_801;
    let _e32: Plane = other_801;
    let _e43: Point = self_913;
    let _e47: Plane = other_801;
    let _e50: Plane = other_801;
    let _e53: Plane = other_801;
    let _e65: Point = self_913;
    let _e69: Plane = other_801;
    let _e72: Plane = other_801;
    let _e75: Plane = other_801;
    return Line((vec3<f32>(0.0) - (vec3<f32>(_e6.g0_.w) * vec3<f32>(_e10.g0_.x, _e13.g0_.y, _e16.g0_.z))), ((((vec3<f32>(_e22.g0_.y) * vec3<f32>(_e26.g0_.z, _e29.g0_.z, _e32.g0_.x)) * vec3<f32>(-(1.0), 0.0, 1.0)) + ((vec3<f32>(_e43.g0_.z) * vec3<f32>(_e47.g0_.y, _e50.g0_.x, _e53.g0_.y)) * vec3<f32>(1.0, -(1.0), 0.0))) + ((vec3<f32>(_e65.g0_.x) * vec3<f32>(_e69.g0_.x, _e72.g0_.z, _e75.g0_.y)) * vec3<f32>(0.0, 1.0, -(1.0)))));
}

fn point_plane_left_contraction(self_914: Point, other_802: Plane) -> Line {
    var self_915: Point;
    var other_803: Plane;

    self_915 = self_914;
    other_803 = other_802;
    let _e4: Point = self_915;
    let _e8: Plane = other_803;
    let _e11: Plane = other_803;
    let _e14: Plane = other_803;
    let _e25: Point = self_915;
    let _e29: Plane = other_803;
    let _e32: Plane = other_803;
    let _e35: Plane = other_803;
    let _e47: Point = self_915;
    let _e51: Plane = other_803;
    let _e54: Plane = other_803;
    let _e57: Plane = other_803;
    let _e69: Point = self_915;
    let _e72: Point = self_915;
    let _e75: Point = self_915;
    let _e79: Plane = other_803;
    return Line(((((vec3<f32>(_e4.g0_.y) * vec3<f32>(_e8.g0_.z, _e11.g0_.z, _e14.g0_.x)) * vec3<f32>(1.0, 0.0, -(1.0))) + ((vec3<f32>(_e25.g0_.z) * vec3<f32>(_e29.g0_.y, _e32.g0_.x, _e35.g0_.y)) * vec3<f32>(-(1.0), 1.0, 0.0))) + ((vec3<f32>(_e47.g0_.x) * vec3<f32>(_e51.g0_.x, _e54.g0_.z, _e57.g0_.y)) * vec3<f32>(0.0, -(1.0), 1.0))), ((vec3<f32>(_e69.g0_.x, _e72.g0_.y, _e75.g0_.z) * vec3<f32>(_e79.g0_.w)) * vec3<f32>(-(1.0))));
}

fn point_plane_right_anti_contraction(self_916: Point, other_804: Plane) -> Line {
    var self_917: Point;
    var other_805: Plane;

    self_917 = self_916;
    other_805 = other_804;
    let _e6: Point = self_917;
    let _e10: Plane = other_805;
    let _e13: Plane = other_805;
    let _e16: Plane = other_805;
    let _e22: Point = self_917;
    let _e26: Plane = other_805;
    let _e29: Plane = other_805;
    let _e32: Plane = other_805;
    let _e43: Point = self_917;
    let _e47: Plane = other_805;
    let _e50: Plane = other_805;
    let _e53: Plane = other_805;
    let _e65: Point = self_917;
    let _e69: Plane = other_805;
    let _e72: Plane = other_805;
    let _e75: Plane = other_805;
    return Line((vec3<f32>(0.0) - (vec3<f32>(_e6.g0_.w) * vec3<f32>(_e10.g0_.x, _e13.g0_.y, _e16.g0_.z))), ((((vec3<f32>(_e22.g0_.y) * vec3<f32>(_e26.g0_.z, _e29.g0_.z, _e32.g0_.x)) * vec3<f32>(-(1.0), 0.0, 1.0)) + ((vec3<f32>(_e43.g0_.z) * vec3<f32>(_e47.g0_.y, _e50.g0_.x, _e53.g0_.y)) * vec3<f32>(1.0, -(1.0), 0.0))) + ((vec3<f32>(_e65.g0_.x) * vec3<f32>(_e69.g0_.x, _e72.g0_.z, _e75.g0_.y)) * vec3<f32>(0.0, 1.0, -(1.0)))));
}

fn point_motor_geometric_product(self_918: Point, other_806: Motor) -> Flector {
    var self_919: Point;
    var other_807: Motor;

    self_919 = self_918;
    other_807 = other_806;
    let _e4: Point = self_919;
    let _e8: Motor = other_807;
    let _e11: Motor = other_807;
    let _e14: Motor = other_807;
    let _e17: Motor = other_807;
    let _e30: Point = self_919;
    let _e34: Motor = other_807;
    let _e37: Motor = other_807;
    let _e40: Motor = other_807;
    let _e43: Motor = other_807;
    let _e57: Point = self_919;
    let _e61: Motor = other_807;
    let _e64: Motor = other_807;
    let _e67: Motor = other_807;
    let _e70: Motor = other_807;
    let _e84: Point = self_919;
    let _e88: Motor = other_807;
    let _e91: Motor = other_807;
    let _e94: Motor = other_807;
    let _e97: Motor = other_807;
    let _e110: Point = self_919;
    let _e114: Motor = other_807;
    let _e117: Motor = other_807;
    let _e120: Motor = other_807;
    let _e123: Motor = other_807;
    let _e137: Point = self_919;
    let _e141: Motor = other_807;
    let _e144: Motor = other_807;
    let _e147: Motor = other_807;
    let _e150: Motor = other_807;
    let _e164: Point = self_919;
    let _e167: Motor = other_807;
    let _e170: Motor = other_807;
    let _e173: Motor = other_807;
    let _e176: Motor = other_807;
    return Flector(((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g1_.z, _e11.g1_.z, _e14.g1_.x, _e17.g0_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e30.g0_.z) * vec4<f32>(_e34.g1_.y, _e37.g1_.x, _e40.g1_.y, _e43.g0_.z)) * vec4<f32>(1.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e57.g0_.x) * vec4<f32>(_e61.g1_.x, _e64.g1_.z, _e67.g1_.y, _e70.g0_.x)) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))), (((((vec4<f32>(_e84.g0_.x) * vec4<f32>(_e88.g0_.w, _e91.g0_.z, _e94.g0_.y, _e97.g1_.x)) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0))) + ((vec4<f32>(_e110.g0_.y) * vec4<f32>(_e114.g0_.z, _e117.g0_.w, _e120.g0_.x, _e123.g1_.y)) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e137.g0_.z) * vec4<f32>(_e141.g0_.y, _e144.g0_.x, _e147.g0_.w, _e150.g1_.z)) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))) + ((_e164.g0_.wwwx * vec4<f32>(_e167.g1_.x, _e170.g1_.y, _e173.g1_.z, _e176.g1_.x)) * vec4<f32>(1.0, 1.0, 1.0, 0.0))));
}

fn point_motor_regressive_product(self_920: Point, other_808: Motor) -> Point {
    var self_921: Point;
    var other_809: Motor;

    self_921 = self_920;
    other_809 = other_808;
    let _e4: Point = self_921;
    let _e6: Motor = other_809;
    return Point((_e4.g0_ * vec4<f32>(_e6.g0_.w)));
}

fn point_motor_anti_wedge(self_922: Point, other_810: Motor) -> Point {
    var self_923: Point;
    var other_811: Motor;

    self_923 = self_922;
    other_811 = other_810;
    let _e4: Point = self_923;
    let _e6: Motor = other_811;
    return Point((_e4.g0_ * vec4<f32>(_e6.g0_.w)));
}

fn point_motor_meet(self_924: Point, other_812: Motor) -> Point {
    var self_925: Point;
    var other_813: Motor;

    self_925 = self_924;
    other_813 = other_812;
    let _e4: Point = self_925;
    let _e6: Motor = other_813;
    return Point((_e4.g0_ * vec4<f32>(_e6.g0_.w)));
}

fn point_motor_outer_product(self_926: Point, other_814: Motor) -> Plane {
    var self_927: Point;
    var other_815: Motor;

    self_927 = self_926;
    other_815 = other_814;
    let _e4: Point = self_927;
    let _e8: Motor = other_815;
    let _e11: Motor = other_815;
    let _e14: Motor = other_815;
    let _e17: Motor = other_815;
    let _e30: Point = self_927;
    let _e34: Motor = other_815;
    let _e37: Motor = other_815;
    let _e40: Motor = other_815;
    let _e43: Motor = other_815;
    let _e57: Point = self_927;
    let _e61: Motor = other_815;
    let _e64: Motor = other_815;
    let _e67: Motor = other_815;
    let _e70: Motor = other_815;
    let _e82: Point = self_927;
    let _e86: Motor = other_815;
    let _e89: Motor = other_815;
    let _e92: Motor = other_815;
    let _e95: Motor = other_815;
    return Plane((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.z, _e11.g0_.z, _e14.g0_.x, _e17.g1_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e30.g0_.z) * vec4<f32>(_e34.g0_.y, _e37.g0_.x, _e40.g0_.y, _e43.g1_.z)) * vec4<f32>(1.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e57.g0_.w) * vec4<f32>(_e61.g1_.x, _e64.g1_.y, _e67.g1_.z, _e70.g1_.x)) * vec4<f32>(1.0, 1.0, 1.0, 0.0))) + ((vec4<f32>(_e82.g0_.x) * vec4<f32>(_e86.g0_.x, _e89.g0_.z, _e92.g0_.y, _e95.g1_.x)) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))));
}

fn point_motor_wedge(self_928: Point, other_816: Motor) -> Plane {
    var self_929: Point;
    var other_817: Motor;

    self_929 = self_928;
    other_817 = other_816;
    let _e4: Point = self_929;
    let _e8: Motor = other_817;
    let _e11: Motor = other_817;
    let _e14: Motor = other_817;
    let _e17: Motor = other_817;
    let _e30: Point = self_929;
    let _e34: Motor = other_817;
    let _e37: Motor = other_817;
    let _e40: Motor = other_817;
    let _e43: Motor = other_817;
    let _e57: Point = self_929;
    let _e61: Motor = other_817;
    let _e64: Motor = other_817;
    let _e67: Motor = other_817;
    let _e70: Motor = other_817;
    let _e82: Point = self_929;
    let _e86: Motor = other_817;
    let _e89: Motor = other_817;
    let _e92: Motor = other_817;
    let _e95: Motor = other_817;
    return Plane((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.z, _e11.g0_.z, _e14.g0_.x, _e17.g1_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e30.g0_.z) * vec4<f32>(_e34.g0_.y, _e37.g0_.x, _e40.g0_.y, _e43.g1_.z)) * vec4<f32>(1.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e57.g0_.w) * vec4<f32>(_e61.g1_.x, _e64.g1_.y, _e67.g1_.z, _e70.g1_.x)) * vec4<f32>(1.0, 1.0, 1.0, 0.0))) + ((vec4<f32>(_e82.g0_.x) * vec4<f32>(_e86.g0_.x, _e89.g0_.z, _e92.g0_.y, _e95.g1_.x)) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))));
}

fn point_motor_join(self_930: Point, other_818: Motor) -> Plane {
    var self_931: Point;
    var other_819: Motor;

    self_931 = self_930;
    other_819 = other_818;
    let _e4: Point = self_931;
    let _e8: Motor = other_819;
    let _e11: Motor = other_819;
    let _e14: Motor = other_819;
    let _e17: Motor = other_819;
    let _e30: Point = self_931;
    let _e34: Motor = other_819;
    let _e37: Motor = other_819;
    let _e40: Motor = other_819;
    let _e43: Motor = other_819;
    let _e57: Point = self_931;
    let _e61: Motor = other_819;
    let _e64: Motor = other_819;
    let _e67: Motor = other_819;
    let _e70: Motor = other_819;
    let _e82: Point = self_931;
    let _e86: Motor = other_819;
    let _e89: Motor = other_819;
    let _e92: Motor = other_819;
    let _e95: Motor = other_819;
    return Plane((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.z, _e11.g0_.z, _e14.g0_.x, _e17.g1_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e30.g0_.z) * vec4<f32>(_e34.g0_.y, _e37.g0_.x, _e40.g0_.y, _e43.g1_.z)) * vec4<f32>(1.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e57.g0_.w) * vec4<f32>(_e61.g1_.x, _e64.g1_.y, _e67.g1_.z, _e70.g1_.x)) * vec4<f32>(1.0, 1.0, 1.0, 0.0))) + ((vec4<f32>(_e82.g0_.x) * vec4<f32>(_e86.g0_.x, _e89.g0_.z, _e92.g0_.y, _e95.g1_.x)) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))));
}

fn point_motor_geometric_anti_product(self_932: Point, other_820: Motor) -> Flector {
    var self_933: Point;
    var other_821: Motor;

    self_933 = self_932;
    other_821 = other_820;
    let _e4: Point = self_933;
    let _e8: Motor = other_821;
    let _e19: Point = self_933;
    let _e23: Motor = other_821;
    let _e35: Point = self_933;
    let _e39: Motor = other_821;
    let _e42: Motor = other_821;
    let _e45: Motor = other_821;
    let _e48: Motor = other_821;
    let _e63: Point = self_933;
    let _e67: Motor = other_821;
    let _e79: Point = self_933;
    let _e83: Motor = other_821;
    let _e95: Point = self_933;
    let _e99: Motor = other_821;
    let _e112: Point = self_933;
    let _e115: Motor = other_821;
    return Flector((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.zwxz) * vec4<f32>(1.0, 1.0, -(1.0), 0.0)) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.yxwy) * vec4<f32>(-(1.0), 1.0, 1.0, 0.0))) + ((vec4<f32>(_e35.g0_.w) * vec4<f32>(_e39.g1_.x, _e42.g1_.y, _e45.g1_.z, _e48.g0_.w)) * vec4<f32>(-(1.0), -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e63.g0_.x) * _e67.g0_.wzyx) * vec4<f32>(1.0, -(1.0), 1.0, 0.0))), ((((vec4<f32>(_e79.g0_.y) * vec4<f32>(_e83.g0_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0))) + ((vec4<f32>(_e95.g0_.z) * vec4<f32>(_e99.g0_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((_e112.g0_.wwwx * _e115.g0_.xyzx) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))));
}

fn point_motor_inner_anti_product(self_934: Point, other_822: Motor) -> Flector {
    var self_935: Point;
    var other_823: Motor;

    self_935 = self_934;
    other_823 = other_822;
    let _e4: Point = self_935;
    let _e6: Motor = other_823;
    let _e11: Point = self_935;
    let _e15: Motor = other_823;
    let _e27: Point = self_935;
    let _e31: Motor = other_823;
    let _e44: Point = self_935;
    let _e47: Motor = other_823;
    return Flector((_e4.g0_ * vec4<f32>(_e6.g0_.w)), ((((vec4<f32>(_e11.g0_.y) * vec4<f32>(_e15.g0_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0))) + ((vec4<f32>(_e27.g0_.z) * vec4<f32>(_e31.g0_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((_e44.g0_.wwwx * _e47.g0_.xyzx) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))));
}

fn point_motor_right_anti_contraction(self_936: Point, other_824: Motor) -> Flector {
    var self_937: Point;
    var other_825: Motor;

    self_937 = self_936;
    other_825 = other_824;
    let _e4: Point = self_937;
    let _e6: Motor = other_825;
    let _e11: Point = self_937;
    let _e15: Motor = other_825;
    let _e27: Point = self_937;
    let _e31: Motor = other_825;
    let _e44: Point = self_937;
    let _e47: Motor = other_825;
    return Flector((_e4.g0_ * vec4<f32>(_e6.g0_.w)), ((((vec4<f32>(_e11.g0_.y) * vec4<f32>(_e15.g0_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0))) + ((vec4<f32>(_e27.g0_.z) * vec4<f32>(_e31.g0_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((_e44.g0_.wwwx * _e47.g0_.xyzx) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))));
}

fn point_rotor_regressive_product(self_938: Point, other_826: Rotor) -> Point {
    var self_939: Point;
    var other_827: Rotor;

    self_939 = self_938;
    other_827 = other_826;
    let _e4: Point = self_939;
    let _e6: Rotor = other_827;
    return Point((_e4.g0_ * vec4<f32>(_e6.g0_.w)));
}

fn point_rotor_anti_wedge(self_940: Point, other_828: Rotor) -> Point {
    var self_941: Point;
    var other_829: Rotor;

    self_941 = self_940;
    other_829 = other_828;
    let _e4: Point = self_941;
    let _e6: Rotor = other_829;
    return Point((_e4.g0_ * vec4<f32>(_e6.g0_.w)));
}

fn point_rotor_meet(self_942: Point, other_830: Rotor) -> Point {
    var self_943: Point;
    var other_831: Rotor;

    self_943 = self_942;
    other_831 = other_830;
    let _e4: Point = self_943;
    let _e6: Rotor = other_831;
    return Point((_e4.g0_ * vec4<f32>(_e6.g0_.w)));
}

fn point_rotor_geometric_anti_product(self_944: Point, other_832: Rotor) -> Flector {
    var self_945: Point;
    var other_833: Rotor;

    self_945 = self_944;
    other_833 = other_832;
    let _e4: Point = self_945;
    let _e8: Rotor = other_833;
    let _e19: Point = self_945;
    let _e23: Rotor = other_833;
    let _e35: Point = self_945;
    let _e38: Rotor = other_833;
    let _e50: Point = self_945;
    let _e54: Rotor = other_833;
    let _e66: Point = self_945;
    let _e70: Rotor = other_833;
    let _e83: Point = self_945;
    let _e86: Rotor = other_833;
    return Flector(((((vec4<f32>(_e4.g0_.y) * _e8.g0_.zwxz) * vec4<f32>(1.0, 1.0, -(1.0), 0.0)) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.yxwy) * vec4<f32>(-(1.0), 1.0, 1.0, 0.0))) + ((_e35.g0_.xxxw * _e38.g0_.wzyw) * vec4<f32>(1.0, -(1.0), 1.0, 1.0))), ((((vec4<f32>(_e50.g0_.y) * vec4<f32>(_e54.g0_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0))) + ((vec4<f32>(_e66.g0_.z) * vec4<f32>(_e70.g0_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((_e83.g0_.wwwx * _e86.g0_.xyzx) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))));
}

fn point_rotor_inner_anti_product(self_946: Point, other_834: Rotor) -> Flector {
    var self_947: Point;
    var other_835: Rotor;

    self_947 = self_946;
    other_835 = other_834;
    let _e4: Point = self_947;
    let _e6: Rotor = other_835;
    let _e11: Point = self_947;
    let _e15: Rotor = other_835;
    let _e27: Point = self_947;
    let _e31: Rotor = other_835;
    let _e44: Point = self_947;
    let _e47: Rotor = other_835;
    return Flector((_e4.g0_ * vec4<f32>(_e6.g0_.w)), ((((vec4<f32>(_e11.g0_.y) * vec4<f32>(_e15.g0_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0))) + ((vec4<f32>(_e27.g0_.z) * vec4<f32>(_e31.g0_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((_e44.g0_.wwwx * _e47.g0_.xyzx) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))));
}

fn point_rotor_right_anti_contraction(self_948: Point, other_836: Rotor) -> Flector {
    var self_949: Point;
    var other_837: Rotor;

    self_949 = self_948;
    other_837 = other_836;
    let _e4: Point = self_949;
    let _e6: Rotor = other_837;
    let _e11: Point = self_949;
    let _e15: Rotor = other_837;
    let _e27: Point = self_949;
    let _e31: Rotor = other_837;
    let _e44: Point = self_949;
    let _e47: Rotor = other_837;
    return Flector((_e4.g0_ * vec4<f32>(_e6.g0_.w)), ((((vec4<f32>(_e11.g0_.y) * vec4<f32>(_e15.g0_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0))) + ((vec4<f32>(_e27.g0_.z) * vec4<f32>(_e31.g0_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((_e44.g0_.wwwx * _e47.g0_.xyzx) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))));
}

fn point_translator_regressive_product(self_950: Point, other_838: Translator) -> Point {
    var self_951: Point;
    var other_839: Translator;

    self_951 = self_950;
    other_839 = other_838;
    let _e4: Point = self_951;
    let _e6: Translator = other_839;
    return Point((_e4.g0_ * vec4<f32>(_e6.g0_.w)));
}

fn point_translator_anti_wedge(self_952: Point, other_840: Translator) -> Point {
    var self_953: Point;
    var other_841: Translator;

    self_953 = self_952;
    other_841 = other_840;
    let _e4: Point = self_953;
    let _e6: Translator = other_841;
    return Point((_e4.g0_ * vec4<f32>(_e6.g0_.w)));
}

fn point_translator_meet(self_954: Point, other_842: Translator) -> Point {
    var self_955: Point;
    var other_843: Translator;

    self_955 = self_954;
    other_843 = other_842;
    let _e4: Point = self_955;
    let _e6: Translator = other_843;
    return Point((_e4.g0_ * vec4<f32>(_e6.g0_.w)));
}

fn point_translator_outer_product(self_956: Point, other_844: Translator) -> Plane {
    var self_957: Point;
    var other_845: Translator;

    self_957 = self_956;
    other_845 = other_844;
    let _e4: Point = self_957;
    let _e8: Translator = other_845;
    let _e20: Point = self_957;
    let _e24: Translator = other_845;
    let _e37: Point = self_957;
    let _e40: Translator = other_845;
    return Plane(((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0))) + ((vec4<f32>(_e20.g0_.z) * vec4<f32>(_e24.g0_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((_e37.g0_.wwwx * _e40.g0_.xyzx) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))));
}

fn point_translator_wedge(self_958: Point, other_846: Translator) -> Plane {
    var self_959: Point;
    var other_847: Translator;

    self_959 = self_958;
    other_847 = other_846;
    let _e4: Point = self_959;
    let _e8: Translator = other_847;
    let _e20: Point = self_959;
    let _e24: Translator = other_847;
    let _e37: Point = self_959;
    let _e40: Translator = other_847;
    return Plane(((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0))) + ((vec4<f32>(_e20.g0_.z) * vec4<f32>(_e24.g0_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((_e37.g0_.wwwx * _e40.g0_.xyzx) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))));
}

fn point_translator_join(self_960: Point, other_848: Translator) -> Plane {
    var self_961: Point;
    var other_849: Translator;

    self_961 = self_960;
    other_849 = other_848;
    let _e4: Point = self_961;
    let _e8: Translator = other_849;
    let _e20: Point = self_961;
    let _e24: Translator = other_849;
    let _e37: Point = self_961;
    let _e40: Translator = other_849;
    return Plane(((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0))) + ((vec4<f32>(_e20.g0_.z) * vec4<f32>(_e24.g0_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((_e37.g0_.wwwx * _e40.g0_.xyzx) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))));
}

fn point_translator_geometric_anti_product(self_962: Point, other_850: Translator) -> Point {
    var self_963: Point;
    var other_851: Translator;

    self_963 = self_962;
    other_851 = other_850;
    let _e4: Point = self_963;
    let _e8: Translator = other_851;
    let _e20: Point = self_963;
    let _e23: Translator = other_851;
    return Point((((vec4<f32>(_e4.g0_.w) * _e8.g0_) * vec4<f32>(-(1.0), -(1.0), -(1.0), 1.0)) + ((_e20.g0_.xyzx * _e23.g0_.wwwx) * vec4<f32>(1.0, 1.0, 1.0, 0.0))));
}

fn point_translator_inner_anti_product(self_964: Point, other_852: Translator) -> Point {
    var self_965: Point;
    var other_853: Translator;

    self_965 = self_964;
    other_853 = other_852;
    let _e4: Point = self_965;
    let _e6: Translator = other_853;
    return Point((_e4.g0_ * vec4<f32>(_e6.g0_.w)));
}

fn point_translator_right_anti_contraction(self_966: Point, other_854: Translator) -> Point {
    var self_967: Point;
    var other_855: Translator;

    self_967 = self_966;
    other_855 = other_854;
    let _e4: Point = self_967;
    let _e6: Translator = other_855;
    return Point((_e4.g0_ * vec4<f32>(_e6.g0_.w)));
}

fn point_flector_add(self_968: Point, other_856: Flector) -> Flector {
    var self_969: Point;
    var other_857: Flector;

    self_969 = self_968;
    other_857 = other_856;
    let _e4: Point = self_969;
    let _e6: Flector = other_857;
    let _e9: Flector = other_857;
    return Flector((_e4.g0_ + _e6.g0_), _e9.g1_);
}

fn point_flector_sub(self_970: Point, other_858: Flector) -> Flector {
    var self_971: Point;
    var other_859: Flector;

    self_971 = self_970;
    other_859 = other_858;
    let _e4: Point = self_971;
    let _e6: Flector = other_859;
    let _e11: Flector = other_859;
    return Flector((_e4.g0_ - _e6.g0_), (vec4<f32>(0.0) - _e11.g1_));
}

fn point_flector_regressive_product(self_972: Point, other_860: Flector) -> Scalar {
    var self_973: Point;
    var other_861: Flector;

    self_973 = self_972;
    other_861 = other_860;
    let _e4: Point = self_973;
    let _e7: Flector = other_861;
    let _e11: Point = self_973;
    let _e14: Flector = other_861;
    let _e19: Point = self_973;
    let _e22: Flector = other_861;
    let _e27: Point = self_973;
    let _e30: Flector = other_861;
    return Scalar(((((_e4.g0_.x * _e7.g1_.x) + (_e11.g0_.y * _e14.g1_.y)) + (_e19.g0_.z * _e22.g1_.z)) + (_e27.g0_.w * _e30.g1_.w)));
}

fn point_flector_anti_wedge(self_974: Point, other_862: Flector) -> Scalar {
    var self_975: Point;
    var other_863: Flector;

    self_975 = self_974;
    other_863 = other_862;
    let _e4: Point = self_975;
    let _e7: Flector = other_863;
    let _e11: Point = self_975;
    let _e14: Flector = other_863;
    let _e19: Point = self_975;
    let _e22: Flector = other_863;
    let _e27: Point = self_975;
    let _e30: Flector = other_863;
    return Scalar(((((_e4.g0_.x * _e7.g1_.x) + (_e11.g0_.y * _e14.g1_.y)) + (_e19.g0_.z * _e22.g1_.z)) + (_e27.g0_.w * _e30.g1_.w)));
}

fn point_flector_meet(self_976: Point, other_864: Flector) -> Scalar {
    var self_977: Point;
    var other_865: Flector;

    self_977 = self_976;
    other_865 = other_864;
    let _e4: Point = self_977;
    let _e7: Flector = other_865;
    let _e11: Point = self_977;
    let _e14: Flector = other_865;
    let _e19: Point = self_977;
    let _e22: Flector = other_865;
    let _e27: Point = self_977;
    let _e30: Flector = other_865;
    return Scalar(((((_e4.g0_.x * _e7.g1_.x) + (_e11.g0_.y * _e14.g1_.y)) + (_e19.g0_.z * _e22.g1_.z)) + (_e27.g0_.w * _e30.g1_.w)));
}

fn point_flector_outer_product(self_978: Point, other_866: Flector) -> Motor {
    var self_979: Point;
    var other_867: Flector;

    self_979 = self_978;
    other_867 = other_866;
    let _e4: Point = self_979;
    let _e8: Flector = other_867;
    let _e11: Flector = other_867;
    let _e14: Flector = other_867;
    let _e17: Flector = other_867;
    let _e29: Point = self_979;
    let _e33: Flector = other_867;
    let _e36: Flector = other_867;
    let _e39: Flector = other_867;
    let _e42: Flector = other_867;
    let _e55: Point = self_979;
    let _e59: Flector = other_867;
    let _e62: Flector = other_867;
    let _e65: Flector = other_867;
    let _e68: Flector = other_867;
    let _e74: Point = self_979;
    let _e78: Flector = other_867;
    let _e81: Flector = other_867;
    let _e84: Flector = other_867;
    let _e87: Flector = other_867;
    let _e100: Point = self_979;
    let _e104: Flector = other_867;
    let _e107: Flector = other_867;
    let _e110: Flector = other_867;
    let _e121: Point = self_979;
    let _e125: Flector = other_867;
    let _e128: Flector = other_867;
    let _e131: Flector = other_867;
    let _e143: Point = self_979;
    let _e147: Flector = other_867;
    let _e150: Flector = other_867;
    let _e153: Flector = other_867;
    return Motor((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.w, _e11.g0_.w, _e14.g0_.w, _e17.g1_.y)) * vec4<f32>(0.0, -(1.0), 0.0, 1.0)) + ((vec4<f32>(_e29.g0_.z) * vec4<f32>(_e33.g0_.w, _e36.g0_.w, _e39.g0_.w, _e42.g1_.z)) * vec4<f32>(0.0, 0.0, -(1.0), 1.0))) + (vec4<f32>(_e55.g0_.w) * vec4<f32>(_e59.g0_.x, _e62.g0_.y, _e65.g0_.z, _e68.g1_.w))) + ((vec4<f32>(_e74.g0_.x) * vec4<f32>(_e78.g0_.w, _e81.g0_.x, _e84.g0_.x, _e87.g1_.x)) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))), ((((vec3<f32>(_e100.g0_.y) * vec3<f32>(_e104.g0_.z, _e107.g0_.z, _e110.g0_.x)) * vec3<f32>(1.0, 0.0, -(1.0))) + ((vec3<f32>(_e121.g0_.z) * vec3<f32>(_e125.g0_.y, _e128.g0_.x, _e131.g0_.y)) * vec3<f32>(-(1.0), 1.0, 0.0))) + ((vec3<f32>(_e143.g0_.x) * vec3<f32>(_e147.g0_.x, _e150.g0_.z, _e153.g0_.y)) * vec3<f32>(0.0, -(1.0), 1.0))));
}

fn point_flector_wedge(self_980: Point, other_868: Flector) -> Motor {
    var self_981: Point;
    var other_869: Flector;

    self_981 = self_980;
    other_869 = other_868;
    let _e4: Point = self_981;
    let _e8: Flector = other_869;
    let _e11: Flector = other_869;
    let _e14: Flector = other_869;
    let _e17: Flector = other_869;
    let _e29: Point = self_981;
    let _e33: Flector = other_869;
    let _e36: Flector = other_869;
    let _e39: Flector = other_869;
    let _e42: Flector = other_869;
    let _e55: Point = self_981;
    let _e59: Flector = other_869;
    let _e62: Flector = other_869;
    let _e65: Flector = other_869;
    let _e68: Flector = other_869;
    let _e74: Point = self_981;
    let _e78: Flector = other_869;
    let _e81: Flector = other_869;
    let _e84: Flector = other_869;
    let _e87: Flector = other_869;
    let _e100: Point = self_981;
    let _e104: Flector = other_869;
    let _e107: Flector = other_869;
    let _e110: Flector = other_869;
    let _e121: Point = self_981;
    let _e125: Flector = other_869;
    let _e128: Flector = other_869;
    let _e131: Flector = other_869;
    let _e143: Point = self_981;
    let _e147: Flector = other_869;
    let _e150: Flector = other_869;
    let _e153: Flector = other_869;
    return Motor((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.w, _e11.g0_.w, _e14.g0_.w, _e17.g1_.y)) * vec4<f32>(0.0, -(1.0), 0.0, 1.0)) + ((vec4<f32>(_e29.g0_.z) * vec4<f32>(_e33.g0_.w, _e36.g0_.w, _e39.g0_.w, _e42.g1_.z)) * vec4<f32>(0.0, 0.0, -(1.0), 1.0))) + (vec4<f32>(_e55.g0_.w) * vec4<f32>(_e59.g0_.x, _e62.g0_.y, _e65.g0_.z, _e68.g1_.w))) + ((vec4<f32>(_e74.g0_.x) * vec4<f32>(_e78.g0_.w, _e81.g0_.x, _e84.g0_.x, _e87.g1_.x)) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))), ((((vec3<f32>(_e100.g0_.y) * vec3<f32>(_e104.g0_.z, _e107.g0_.z, _e110.g0_.x)) * vec3<f32>(1.0, 0.0, -(1.0))) + ((vec3<f32>(_e121.g0_.z) * vec3<f32>(_e125.g0_.y, _e128.g0_.x, _e131.g0_.y)) * vec3<f32>(-(1.0), 1.0, 0.0))) + ((vec3<f32>(_e143.g0_.x) * vec3<f32>(_e147.g0_.x, _e150.g0_.z, _e153.g0_.y)) * vec3<f32>(0.0, -(1.0), 1.0))));
}

fn point_flector_join(self_982: Point, other_870: Flector) -> Motor {
    var self_983: Point;
    var other_871: Flector;

    self_983 = self_982;
    other_871 = other_870;
    let _e4: Point = self_983;
    let _e8: Flector = other_871;
    let _e11: Flector = other_871;
    let _e14: Flector = other_871;
    let _e17: Flector = other_871;
    let _e29: Point = self_983;
    let _e33: Flector = other_871;
    let _e36: Flector = other_871;
    let _e39: Flector = other_871;
    let _e42: Flector = other_871;
    let _e55: Point = self_983;
    let _e59: Flector = other_871;
    let _e62: Flector = other_871;
    let _e65: Flector = other_871;
    let _e68: Flector = other_871;
    let _e74: Point = self_983;
    let _e78: Flector = other_871;
    let _e81: Flector = other_871;
    let _e84: Flector = other_871;
    let _e87: Flector = other_871;
    let _e100: Point = self_983;
    let _e104: Flector = other_871;
    let _e107: Flector = other_871;
    let _e110: Flector = other_871;
    let _e121: Point = self_983;
    let _e125: Flector = other_871;
    let _e128: Flector = other_871;
    let _e131: Flector = other_871;
    let _e143: Point = self_983;
    let _e147: Flector = other_871;
    let _e150: Flector = other_871;
    let _e153: Flector = other_871;
    return Motor((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.w, _e11.g0_.w, _e14.g0_.w, _e17.g1_.y)) * vec4<f32>(0.0, -(1.0), 0.0, 1.0)) + ((vec4<f32>(_e29.g0_.z) * vec4<f32>(_e33.g0_.w, _e36.g0_.w, _e39.g0_.w, _e42.g1_.z)) * vec4<f32>(0.0, 0.0, -(1.0), 1.0))) + (vec4<f32>(_e55.g0_.w) * vec4<f32>(_e59.g0_.x, _e62.g0_.y, _e65.g0_.z, _e68.g1_.w))) + ((vec4<f32>(_e74.g0_.x) * vec4<f32>(_e78.g0_.w, _e81.g0_.x, _e84.g0_.x, _e87.g1_.x)) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))), ((((vec3<f32>(_e100.g0_.y) * vec3<f32>(_e104.g0_.z, _e107.g0_.z, _e110.g0_.x)) * vec3<f32>(1.0, 0.0, -(1.0))) + ((vec3<f32>(_e121.g0_.z) * vec3<f32>(_e125.g0_.y, _e128.g0_.x, _e131.g0_.y)) * vec3<f32>(-(1.0), 1.0, 0.0))) + ((vec3<f32>(_e143.g0_.x) * vec3<f32>(_e147.g0_.x, _e150.g0_.z, _e153.g0_.y)) * vec3<f32>(0.0, -(1.0), 1.0))));
}

fn point_flector_inner_anti_product(self_984: Point, other_872: Flector) -> Motor {
    var self_985: Point;
    var other_873: Flector;

    self_985 = self_984;
    other_873 = other_872;
    let _e6: Point = self_985;
    let _e10: Flector = other_873;
    let _e13: Flector = other_873;
    let _e16: Flector = other_873;
    let _e19: Flector = other_873;
    let _e25: Point = self_985;
    let _e29: Flector = other_873;
    let _e32: Flector = other_873;
    let _e35: Flector = other_873;
    let _e46: Point = self_985;
    let _e50: Flector = other_873;
    let _e53: Flector = other_873;
    let _e56: Flector = other_873;
    let _e68: Point = self_985;
    let _e72: Flector = other_873;
    let _e75: Flector = other_873;
    let _e78: Flector = other_873;
    return Motor((vec4<f32>(0.0) - (vec4<f32>(_e6.g0_.w) * vec4<f32>(_e10.g1_.x, _e13.g1_.y, _e16.g1_.z, _e19.g0_.w))), ((((vec3<f32>(_e25.g0_.y) * vec3<f32>(_e29.g1_.z, _e32.g1_.z, _e35.g1_.x)) * vec3<f32>(-(1.0), 0.0, 1.0)) + ((vec3<f32>(_e46.g0_.z) * vec3<f32>(_e50.g1_.y, _e53.g1_.x, _e56.g1_.y)) * vec3<f32>(1.0, -(1.0), 0.0))) + ((vec3<f32>(_e68.g0_.x) * vec3<f32>(_e72.g1_.x, _e75.g1_.z, _e78.g1_.y)) * vec3<f32>(0.0, 1.0, -(1.0)))));
}

fn point_flector_right_contraction(self_986: Point, other_874: Flector) -> Scalar {
    var self_987: Point;
    var other_875: Flector;

    self_987 = self_986;
    other_875 = other_874;
    let _e4: Point = self_987;
    let _e7: Flector = other_875;
    let _e11: Point = self_987;
    let _e14: Flector = other_875;
    let _e19: Point = self_987;
    let _e22: Flector = other_875;
    return Scalar((((_e4.g0_.x * _e7.g0_.x) + (_e11.g0_.y * _e14.g0_.y)) + (_e19.g0_.z * _e22.g0_.z)));
}

fn point_flector_left_anti_contraction(self_988: Point, other_876: Flector) -> AntiScalar {
    var self_989: Point;
    var other_877: Flector;

    self_989 = self_988;
    other_877 = other_876;
    let _e5: Point = self_989;
    let _e8: Flector = other_877;
    return AntiScalar((0.0 - (_e5.g0_.w * _e8.g0_.w)));
}

fn point_flector_right_anti_contraction(self_990: Point, other_878: Flector) -> Motor {
    var self_991: Point;
    var other_879: Flector;

    self_991 = self_990;
    other_879 = other_878;
    let _e6: Point = self_991;
    let _e10: Flector = other_879;
    let _e13: Flector = other_879;
    let _e16: Flector = other_879;
    let _e19: Flector = other_879;
    let _e25: Point = self_991;
    let _e29: Flector = other_879;
    let _e32: Flector = other_879;
    let _e35: Flector = other_879;
    let _e46: Point = self_991;
    let _e50: Flector = other_879;
    let _e53: Flector = other_879;
    let _e56: Flector = other_879;
    let _e68: Point = self_991;
    let _e72: Flector = other_879;
    let _e75: Flector = other_879;
    let _e78: Flector = other_879;
    return Motor((vec4<f32>(0.0) - (vec4<f32>(_e6.g0_.w) * vec4<f32>(_e10.g1_.x, _e13.g1_.y, _e16.g1_.z, _e19.g0_.w))), ((((vec3<f32>(_e25.g0_.y) * vec3<f32>(_e29.g1_.z, _e32.g1_.z, _e35.g1_.x)) * vec3<f32>(-(1.0), 0.0, 1.0)) + ((vec3<f32>(_e46.g0_.z) * vec3<f32>(_e50.g1_.y, _e53.g1_.x, _e56.g1_.y)) * vec3<f32>(1.0, -(1.0), 0.0))) + ((vec3<f32>(_e68.g0_.x) * vec3<f32>(_e72.g1_.x, _e75.g1_.z, _e78.g1_.y)) * vec3<f32>(0.0, 1.0, -(1.0)))));
}

fn point_flector_scalar_product(self_992: Point, other_880: Flector) -> Scalar {
    var self_993: Point;
    var other_881: Flector;

    self_993 = self_992;
    other_881 = other_880;
    let _e4: Point = self_993;
    let _e7: Flector = other_881;
    let _e11: Point = self_993;
    let _e14: Flector = other_881;
    let _e19: Point = self_993;
    let _e22: Flector = other_881;
    return Scalar((((_e4.g0_.x * _e7.g0_.x) + (_e11.g0_.y * _e14.g0_.y)) + (_e19.g0_.z * _e22.g0_.z)));
}

fn point_flector_dot(self_994: Point, other_882: Flector) -> Scalar {
    var self_995: Point;
    var other_883: Flector;

    self_995 = self_994;
    other_883 = other_882;
    let _e4: Point = self_995;
    let _e7: Flector = other_883;
    let _e11: Point = self_995;
    let _e14: Flector = other_883;
    let _e19: Point = self_995;
    let _e22: Flector = other_883;
    return Scalar((((_e4.g0_.x * _e7.g0_.x) + (_e11.g0_.y * _e14.g0_.y)) + (_e19.g0_.z * _e22.g0_.z)));
}

fn point_flector_anti_scalar_product(self_996: Point, other_884: Flector) -> AntiScalar {
    var self_997: Point;
    var other_885: Flector;

    self_997 = self_996;
    other_885 = other_884;
    let _e5: Point = self_997;
    let _e8: Flector = other_885;
    return AntiScalar((0.0 - (_e5.g0_.w * _e8.g0_.w)));
}

fn point_flector_anti_dot(self_998: Point, other_886: Flector) -> AntiScalar {
    var self_999: Point;
    var other_887: Flector;

    self_999 = self_998;
    other_887 = other_886;
    let _e5: Point = self_999;
    let _e8: Flector = other_887;
    return AntiScalar((0.0 - (_e5.g0_.w * _e8.g0_.w)));
}

fn point_multi_vector_scalar_product(self_1000: Point, other_888: MultiVector) -> Scalar {
    var self_1001: Point;
    var other_889: MultiVector;

    self_1001 = self_1000;
    other_889 = other_888;
    let _e4: Point = self_1001;
    let _e7: MultiVector = other_889;
    let _e11: Point = self_1001;
    let _e14: MultiVector = other_889;
    let _e19: Point = self_1001;
    let _e22: MultiVector = other_889;
    return Scalar((((_e4.g0_.x * _e7.g1_.x) + (_e11.g0_.y * _e14.g1_.y)) + (_e19.g0_.z * _e22.g1_.z)));
}

fn point_multi_vector_dot(self_1002: Point, other_890: MultiVector) -> Scalar {
    var self_1003: Point;
    var other_891: MultiVector;

    self_1003 = self_1002;
    other_891 = other_890;
    let _e4: Point = self_1003;
    let _e7: MultiVector = other_891;
    let _e11: Point = self_1003;
    let _e14: MultiVector = other_891;
    let _e19: Point = self_1003;
    let _e22: MultiVector = other_891;
    return Scalar((((_e4.g0_.x * _e7.g1_.x) + (_e11.g0_.y * _e14.g1_.y)) + (_e19.g0_.z * _e22.g1_.z)));
}

fn point_multi_vector_anti_scalar_product(self_1004: Point, other_892: MultiVector) -> AntiScalar {
    var self_1005: Point;
    var other_893: MultiVector;

    self_1005 = self_1004;
    other_893 = other_892;
    let _e5: Point = self_1005;
    let _e8: MultiVector = other_893;
    return AntiScalar((0.0 - (_e5.g0_.w * _e8.g1_.w)));
}

fn point_multi_vector_anti_dot(self_1006: Point, other_894: MultiVector) -> AntiScalar {
    var self_1007: Point;
    var other_895: MultiVector;

    self_1007 = self_1006;
    other_895 = other_894;
    let _e5: Point = self_1007;
    let _e8: MultiVector = other_895;
    return AntiScalar((0.0 - (_e5.g0_.w * _e8.g1_.w)));
}

fn point_squared_magnitude(self_1008: Point) -> Scalar {
    var self_1009: Point;

    self_1009 = self_1008;
    let _e2: Point = self_1009;
    let _e3: Point = self_1009;
    let _e4: Point = point_reversal(_e3);
    let _e5: Scalar = point_point_scalar_product(_e2, _e4);
    return _e5;
}

fn point_magnitude(self_1010: Point) -> Scalar {
    var self_1011: Point;

    self_1011 = self_1010;
    let _e2: Point = self_1011;
    let _e3: Scalar = point_squared_magnitude(_e2);
    return Scalar(sqrt(_e3.g0_));
}

fn point_bulk_norm(self_1012: Point) -> Scalar {
    var self_1013: Point;

    self_1013 = self_1012;
    let _e2: Point = self_1013;
    let _e3: Scalar = point_squared_magnitude(_e2);
    return Scalar(sqrt(_e3.g0_));
}

fn point_squared_anti_magnitude(self_1014: Point) -> AntiScalar {
    var self_1015: Point;

    self_1015 = self_1014;
    let _e2: Point = self_1015;
    let _e3: Point = self_1015;
    let _e4: Point = point_anti_reversal(_e3);
    let _e5: AntiScalar = point_point_anti_scalar_product(_e2, _e4);
    return _e5;
}

fn point_weight_norm(self_1016: Point) -> AntiScalar {
    var self_1017: Point;

    self_1017 = self_1016;
    let _e2: Point = self_1017;
    let _e3: AntiScalar = point_squared_anti_magnitude(_e2);
    return AntiScalar(sqrt(_e3.g0_));
}

fn point_geometric_norm(self_1018: Point) -> HomogeneousMagnitude {
    var self_1019: Point;

    self_1019 = self_1018;
    let _e2: Point = self_1019;
    let _e3: Scalar = point_bulk_norm(_e2);
    let _e4: Point = self_1019;
    let _e5: AntiScalar = point_weight_norm(_e4);
    let _e6: HomogeneousMagnitude = scalar_anti_scalar_add(_e3, _e5);
    return _e6;
}

fn point_scale(self_1020: Point, other_896: f32) -> Point {
    var self_1021: Point;
    var other_897: f32;

    self_1021 = self_1020;
    other_897 = other_896;
    let _e4: Point = self_1021;
    let _e5: f32 = other_897;
    let _e7: Point = point_scalar_geometric_product(_e4, Scalar(_e5));
    return _e7;
}

fn point_signum(self_1022: Point) -> Point {
    var self_1023: Point;

    self_1023 = self_1022;
    let _e2: Point = self_1023;
    let _e3: Point = self_1023;
    let _e4: Scalar = point_magnitude(_e3);
    let _e9: Point = point_scalar_geometric_product(_e2, Scalar((1.0 / _e4.g0_)));
    return _e9;
}

fn point_inverse(self_1024: Point) -> Point {
    var self_1025: Point;

    self_1025 = self_1024;
    let _e2: Point = self_1025;
    let _e3: Point = point_reversal(_e2);
    let _e4: Point = self_1025;
    let _e5: Scalar = point_squared_magnitude(_e4);
    let _e10: Point = point_scalar_geometric_product(_e3, Scalar((1.0 / _e5.g0_)));
    return _e10;
}

fn point_unitize(self_1026: Point) -> Point {
    var self_1027: Point;

    self_1027 = self_1026;
    let _e2: Point = self_1027;
    let _e3: Point = self_1027;
    let _e4: AntiScalar = point_weight_norm(_e3);
    let _e9: Point = point_scalar_geometric_product(_e2, Scalar((1.0 / _e4.g0_)));
    return _e9;
}

fn point_attitude(self_1028: Point) -> Scalar {
    var self_1029: Point;

    self_1029 = self_1028;
    let _e2: Point = self_1029;
    let _e9: Scalar = point_plane_regressive_product(_e2, Plane(vec4<f32>(0.0, 0.0, 0.0, 1.0)));
    return _e9;
}

fn line_zero() -> Line {
    return Line(vec3<f32>(0.0), vec3<f32>(0.0));
}

fn line_one() -> Line {
    return Line(vec3<f32>(0.0), vec3<f32>(0.0));
}

fn line_grade(self_1030: Line) -> i32 {
    return 2;
}

fn line_anti_grade(self_1031: Line) -> i32 {
    return 2;
}

fn line_neg(self_1032: Line) -> Line {
    var self_1033: Line;

    self_1033 = self_1032;
    let _e2: Line = self_1033;
    let _e8: Line = self_1033;
    return Line((_e2.g0_ * vec3<f32>(-(1.0))), (_e8.g1_ * vec3<f32>(-(1.0))));
}

fn line_automorphism(self_1034: Line) -> Line {
    var self_1035: Line;

    self_1035 = self_1034;
    let _e2: Line = self_1035;
    let _e4: Line = self_1035;
    return Line(_e2.g0_, _e4.g1_);
}

fn line_reversal(self_1036: Line) -> Line {
    var self_1037: Line;

    self_1037 = self_1036;
    let _e2: Line = self_1037;
    let _e8: Line = self_1037;
    return Line((_e2.g0_ * vec3<f32>(-(1.0))), (_e8.g1_ * vec3<f32>(-(1.0))));
}

fn line_conjugation(self_1038: Line) -> Line {
    var self_1039: Line;

    self_1039 = self_1038;
    let _e2: Line = self_1039;
    let _e8: Line = self_1039;
    return Line((_e2.g0_ * vec3<f32>(-(1.0))), (_e8.g1_ * vec3<f32>(-(1.0))));
}

fn line_dual(self_1040: Line) -> Line {
    var self_1041: Line;

    self_1041 = self_1040;
    let _e2: Line = self_1041;
    let _e8: Line = self_1041;
    return Line((_e2.g1_ * vec3<f32>(-(1.0))), (_e8.g0_ * vec3<f32>(-(1.0))));
}

fn line_anti_reversal(self_1042: Line) -> Line {
    var self_1043: Line;

    self_1043 = self_1042;
    let _e2: Line = self_1043;
    let _e8: Line = self_1043;
    return Line((_e2.g0_ * vec3<f32>(-(1.0))), (_e8.g1_ * vec3<f32>(-(1.0))));
}

fn line_right_complement(self_1044: Line) -> Line {
    var self_1045: Line;

    self_1045 = self_1044;
    let _e2: Line = self_1045;
    let _e8: Line = self_1045;
    return Line((_e2.g1_ * vec3<f32>(-(1.0))), (_e8.g0_ * vec3<f32>(-(1.0))));
}

fn line_left_complement(self_1046: Line) -> Line {
    var self_1047: Line;

    self_1047 = self_1046;
    let _e2: Line = self_1047;
    let _e8: Line = self_1047;
    return Line((_e2.g1_ * vec3<f32>(-(1.0))), (_e8.g0_ * vec3<f32>(-(1.0))));
}

fn line_double_complement(self_1048: Line) -> Line {
    var self_1049: Line;

    self_1049 = self_1048;
    let _e2: Line = self_1049;
    let _e4: Line = self_1049;
    return Line(_e2.g0_, _e4.g1_);
}

fn line_scalar_geometric_product(self_1050: Line, other_898: Scalar) -> Line {
    var self_1051: Line;
    var other_899: Scalar;

    self_1051 = self_1050;
    other_899 = other_898;
    let _e4: Line = self_1051;
    let _e6: Scalar = other_899;
    let _e10: Line = self_1051;
    let _e12: Scalar = other_899;
    return Line((_e4.g0_ * vec3<f32>(_e6.g0_)), (_e10.g1_ * vec3<f32>(_e12.g0_)));
}

fn line_scalar_outer_product(self_1052: Line, other_900: Scalar) -> Line {
    var self_1053: Line;
    var other_901: Scalar;

    self_1053 = self_1052;
    other_901 = other_900;
    let _e4: Line = self_1053;
    let _e6: Scalar = other_901;
    let _e10: Line = self_1053;
    let _e12: Scalar = other_901;
    return Line((_e4.g0_ * vec3<f32>(_e6.g0_)), (_e10.g1_ * vec3<f32>(_e12.g0_)));
}

fn line_scalar_wedge(self_1054: Line, other_902: Scalar) -> Line {
    var self_1055: Line;
    var other_903: Scalar;

    self_1055 = self_1054;
    other_903 = other_902;
    let _e4: Line = self_1055;
    let _e6: Scalar = other_903;
    let _e10: Line = self_1055;
    let _e12: Scalar = other_903;
    return Line((_e4.g0_ * vec3<f32>(_e6.g0_)), (_e10.g1_ * vec3<f32>(_e12.g0_)));
}

fn line_scalar_join(self_1056: Line, other_904: Scalar) -> Line {
    var self_1057: Line;
    var other_905: Scalar;

    self_1057 = self_1056;
    other_905 = other_904;
    let _e4: Line = self_1057;
    let _e6: Scalar = other_905;
    let _e10: Line = self_1057;
    let _e12: Scalar = other_905;
    return Line((_e4.g0_ * vec3<f32>(_e6.g0_)), (_e10.g1_ * vec3<f32>(_e12.g0_)));
}

fn line_scalar_inner_product(self_1058: Line, other_906: Scalar) -> Line {
    var self_1059: Line;
    var other_907: Scalar;

    self_1059 = self_1058;
    other_907 = other_906;
    let _e4: Line = self_1059;
    let _e6: Scalar = other_907;
    let _e10: Line = self_1059;
    let _e12: Scalar = other_907;
    return Line((_e4.g0_ * vec3<f32>(_e6.g0_)), (_e10.g1_ * vec3<f32>(_e12.g0_)));
}

fn line_scalar_right_contraction(self_1060: Line, other_908: Scalar) -> Line {
    var self_1061: Line;
    var other_909: Scalar;

    self_1061 = self_1060;
    other_909 = other_908;
    let _e4: Line = self_1061;
    let _e6: Scalar = other_909;
    let _e10: Line = self_1061;
    let _e12: Scalar = other_909;
    return Line((_e4.g0_ * vec3<f32>(_e6.g0_)), (_e10.g1_ * vec3<f32>(_e12.g0_)));
}

fn line_anti_scalar_add(self_1062: Line, other_910: AntiScalar) -> Motor {
    var self_1063: Line;
    var other_911: AntiScalar;

    self_1063 = self_1062;
    other_911 = other_910;
    let _e4: Line = self_1063;
    let _e7: Line = self_1063;
    let _e10: Line = self_1063;
    let _e13: Line = self_1063;
    let _e23: AntiScalar = other_911;
    let _e33: Line = self_1063;
    return Motor(((vec4<f32>(_e4.g0_.x, _e7.g0_.y, _e10.g0_.z, _e13.g0_.x) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) + (vec4<f32>(_e23.g0_) * vec4<f32>(0.0, 0.0, 0.0, 1.0))), _e33.g1_);
}

fn line_anti_scalar_sub(self_1064: Line, other_912: AntiScalar) -> Motor {
    var self_1065: Line;
    var other_913: AntiScalar;

    self_1065 = self_1064;
    other_913 = other_912;
    let _e4: Line = self_1065;
    let _e7: Line = self_1065;
    let _e10: Line = self_1065;
    let _e13: Line = self_1065;
    let _e23: AntiScalar = other_913;
    let _e33: Line = self_1065;
    return Motor(((vec4<f32>(_e4.g0_.x, _e7.g0_.y, _e10.g0_.z, _e13.g0_.x) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) - (vec4<f32>(_e23.g0_) * vec4<f32>(0.0, 0.0, 0.0, 1.0))), _e33.g1_);
}

fn line_anti_scalar_regressive_product(self_1066: Line, other_914: AntiScalar) -> Line {
    var self_1067: Line;
    var other_915: AntiScalar;

    self_1067 = self_1066;
    other_915 = other_914;
    let _e4: Line = self_1067;
    let _e6: AntiScalar = other_915;
    let _e10: Line = self_1067;
    let _e12: AntiScalar = other_915;
    return Line((_e4.g0_ * vec3<f32>(_e6.g0_)), (_e10.g1_ * vec3<f32>(_e12.g0_)));
}

fn line_anti_scalar_anti_wedge(self_1068: Line, other_916: AntiScalar) -> Line {
    var self_1069: Line;
    var other_917: AntiScalar;

    self_1069 = self_1068;
    other_917 = other_916;
    let _e4: Line = self_1069;
    let _e6: AntiScalar = other_917;
    let _e10: Line = self_1069;
    let _e12: AntiScalar = other_917;
    return Line((_e4.g0_ * vec3<f32>(_e6.g0_)), (_e10.g1_ * vec3<f32>(_e12.g0_)));
}

fn line_anti_scalar_meet(self_1070: Line, other_918: AntiScalar) -> Line {
    var self_1071: Line;
    var other_919: AntiScalar;

    self_1071 = self_1070;
    other_919 = other_918;
    let _e4: Line = self_1071;
    let _e6: AntiScalar = other_919;
    let _e10: Line = self_1071;
    let _e12: AntiScalar = other_919;
    return Line((_e4.g0_ * vec3<f32>(_e6.g0_)), (_e10.g1_ * vec3<f32>(_e12.g0_)));
}

fn line_anti_scalar_geometric_anti_product(self_1072: Line, other_920: AntiScalar) -> Line {
    var self_1073: Line;
    var other_921: AntiScalar;

    self_1073 = self_1072;
    other_921 = other_920;
    let _e4: Line = self_1073;
    let _e6: AntiScalar = other_921;
    let _e10: Line = self_1073;
    let _e12: AntiScalar = other_921;
    return Line((_e4.g0_ * vec3<f32>(_e6.g0_)), (_e10.g1_ * vec3<f32>(_e12.g0_)));
}

fn line_anti_scalar_inner_anti_product(self_1074: Line, other_922: AntiScalar) -> Line {
    var self_1075: Line;
    var other_923: AntiScalar;

    self_1075 = self_1074;
    other_923 = other_922;
    let _e4: Line = self_1075;
    let _e6: AntiScalar = other_923;
    let _e10: Line = self_1075;
    let _e12: AntiScalar = other_923;
    return Line((_e4.g0_ * vec3<f32>(_e6.g0_)), (_e10.g1_ * vec3<f32>(_e12.g0_)));
}

fn line_anti_scalar_right_anti_contraction(self_1076: Line, other_924: AntiScalar) -> Line {
    var self_1077: Line;
    var other_925: AntiScalar;

    self_1077 = self_1076;
    other_925 = other_924;
    let _e4: Line = self_1077;
    let _e6: AntiScalar = other_925;
    let _e10: Line = self_1077;
    let _e12: AntiScalar = other_925;
    return Line((_e4.g0_ * vec3<f32>(_e6.g0_)), (_e10.g1_ * vec3<f32>(_e12.g0_)));
}

fn line_homogeneous_magnitude_geometric_product(self_1078: Line, other_926: HomogeneousMagnitude) -> Line {
    var self_1079: Line;
    var other_927: HomogeneousMagnitude;

    self_1079 = self_1078;
    other_927 = other_926;
    let _e4: Line = self_1079;
    let _e8: HomogeneousMagnitude = other_927;
    let _e18: Line = self_1079;
    let _e22: HomogeneousMagnitude = other_927;
    let _e33: Line = self_1079;
    let _e37: HomogeneousMagnitude = other_927;
    let _e48: Line = self_1079;
    let _e50: HomogeneousMagnitude = other_927;
    let _e56: Line = self_1079;
    let _e58: HomogeneousMagnitude = other_927;
    return Line((((((vec3<f32>(_e4.g1_.x) * vec3<f32>(_e8.g0_.y)) * vec3<f32>(1.0, 0.0, 0.0)) + ((vec3<f32>(_e18.g1_.y) * vec3<f32>(_e22.g0_.y)) * vec3<f32>(0.0, 1.0, 0.0))) + ((vec3<f32>(_e33.g1_.z) * vec3<f32>(_e37.g0_.y)) * vec3<f32>(0.0, 0.0, 1.0))) + (_e48.g0_ * vec3<f32>(_e50.g0_.x))), (_e56.g1_ * vec3<f32>(_e58.g0_.x)));
}

fn line_homogeneous_magnitude_regressive_product(self_1080: Line, other_928: HomogeneousMagnitude) -> Line {
    var self_1081: Line;
    var other_929: HomogeneousMagnitude;

    self_1081 = self_1080;
    other_929 = other_928;
    let _e4: Line = self_1081;
    let _e6: HomogeneousMagnitude = other_929;
    let _e11: Line = self_1081;
    let _e13: HomogeneousMagnitude = other_929;
    return Line((_e4.g0_ * vec3<f32>(_e6.g0_.y)), (_e11.g1_ * vec3<f32>(_e13.g0_.y)));
}

fn line_homogeneous_magnitude_anti_wedge(self_1082: Line, other_930: HomogeneousMagnitude) -> Line {
    var self_1083: Line;
    var other_931: HomogeneousMagnitude;

    self_1083 = self_1082;
    other_931 = other_930;
    let _e4: Line = self_1083;
    let _e6: HomogeneousMagnitude = other_931;
    let _e11: Line = self_1083;
    let _e13: HomogeneousMagnitude = other_931;
    return Line((_e4.g0_ * vec3<f32>(_e6.g0_.y)), (_e11.g1_ * vec3<f32>(_e13.g0_.y)));
}

fn line_homogeneous_magnitude_meet(self_1084: Line, other_932: HomogeneousMagnitude) -> Line {
    var self_1085: Line;
    var other_933: HomogeneousMagnitude;

    self_1085 = self_1084;
    other_933 = other_932;
    let _e4: Line = self_1085;
    let _e6: HomogeneousMagnitude = other_933;
    let _e11: Line = self_1085;
    let _e13: HomogeneousMagnitude = other_933;
    return Line((_e4.g0_ * vec3<f32>(_e6.g0_.y)), (_e11.g1_ * vec3<f32>(_e13.g0_.y)));
}

fn line_homogeneous_magnitude_outer_product(self_1086: Line, other_934: HomogeneousMagnitude) -> Line {
    var self_1087: Line;
    var other_935: HomogeneousMagnitude;

    self_1087 = self_1086;
    other_935 = other_934;
    let _e4: Line = self_1087;
    let _e6: HomogeneousMagnitude = other_935;
    let _e11: Line = self_1087;
    let _e13: HomogeneousMagnitude = other_935;
    return Line((_e4.g0_ * vec3<f32>(_e6.g0_.x)), (_e11.g1_ * vec3<f32>(_e13.g0_.x)));
}

fn line_homogeneous_magnitude_wedge(self_1088: Line, other_936: HomogeneousMagnitude) -> Line {
    var self_1089: Line;
    var other_937: HomogeneousMagnitude;

    self_1089 = self_1088;
    other_937 = other_936;
    let _e4: Line = self_1089;
    let _e6: HomogeneousMagnitude = other_937;
    let _e11: Line = self_1089;
    let _e13: HomogeneousMagnitude = other_937;
    return Line((_e4.g0_ * vec3<f32>(_e6.g0_.x)), (_e11.g1_ * vec3<f32>(_e13.g0_.x)));
}

fn line_homogeneous_magnitude_join(self_1090: Line, other_938: HomogeneousMagnitude) -> Line {
    var self_1091: Line;
    var other_939: HomogeneousMagnitude;

    self_1091 = self_1090;
    other_939 = other_938;
    let _e4: Line = self_1091;
    let _e6: HomogeneousMagnitude = other_939;
    let _e11: Line = self_1091;
    let _e13: HomogeneousMagnitude = other_939;
    return Line((_e4.g0_ * vec3<f32>(_e6.g0_.x)), (_e11.g1_ * vec3<f32>(_e13.g0_.x)));
}

fn line_homogeneous_magnitude_inner_product(self_1092: Line, other_940: HomogeneousMagnitude) -> Line {
    var self_1093: Line;
    var other_941: HomogeneousMagnitude;

    self_1093 = self_1092;
    other_941 = other_940;
    let _e4: Line = self_1093;
    let _e8: HomogeneousMagnitude = other_941;
    let _e18: Line = self_1093;
    let _e22: HomogeneousMagnitude = other_941;
    let _e33: Line = self_1093;
    let _e37: HomogeneousMagnitude = other_941;
    let _e48: Line = self_1093;
    let _e50: HomogeneousMagnitude = other_941;
    let _e56: Line = self_1093;
    let _e58: HomogeneousMagnitude = other_941;
    return Line((((((vec3<f32>(_e4.g1_.x) * vec3<f32>(_e8.g0_.y)) * vec3<f32>(1.0, 0.0, 0.0)) + ((vec3<f32>(_e18.g1_.y) * vec3<f32>(_e22.g0_.y)) * vec3<f32>(0.0, 1.0, 0.0))) + ((vec3<f32>(_e33.g1_.z) * vec3<f32>(_e37.g0_.y)) * vec3<f32>(0.0, 0.0, 1.0))) + (_e48.g0_ * vec3<f32>(_e50.g0_.x))), (_e56.g1_ * vec3<f32>(_e58.g0_.x)));
}

fn line_homogeneous_magnitude_geometric_anti_product(self_1094: Line, other_942: HomogeneousMagnitude) -> Line {
    var self_1095: Line;
    var other_943: HomogeneousMagnitude;

    self_1095 = self_1094;
    other_943 = other_942;
    let _e4: Line = self_1095;
    let _e6: HomogeneousMagnitude = other_943;
    let _e11: Line = self_1095;
    let _e15: HomogeneousMagnitude = other_943;
    let _e25: Line = self_1095;
    let _e29: HomogeneousMagnitude = other_943;
    let _e40: Line = self_1095;
    let _e44: HomogeneousMagnitude = other_943;
    let _e55: Line = self_1095;
    let _e57: HomogeneousMagnitude = other_943;
    return Line((_e4.g0_ * vec3<f32>(_e6.g0_.y)), (((((vec3<f32>(_e11.g1_.x) * vec3<f32>(_e15.g0_.y)) * vec3<f32>(1.0, 0.0, 0.0)) + ((vec3<f32>(_e25.g1_.y) * vec3<f32>(_e29.g0_.y)) * vec3<f32>(0.0, 1.0, 0.0))) + ((vec3<f32>(_e40.g1_.z) * vec3<f32>(_e44.g0_.y)) * vec3<f32>(0.0, 0.0, 1.0))) + (_e55.g0_ * vec3<f32>(_e57.g0_.x))));
}

fn line_homogeneous_magnitude_inner_anti_product(self_1096: Line, other_944: HomogeneousMagnitude) -> Line {
    var self_1097: Line;
    var other_945: HomogeneousMagnitude;

    self_1097 = self_1096;
    other_945 = other_944;
    let _e4: Line = self_1097;
    let _e6: HomogeneousMagnitude = other_945;
    let _e11: Line = self_1097;
    let _e15: HomogeneousMagnitude = other_945;
    let _e25: Line = self_1097;
    let _e29: HomogeneousMagnitude = other_945;
    let _e40: Line = self_1097;
    let _e44: HomogeneousMagnitude = other_945;
    let _e55: Line = self_1097;
    let _e57: HomogeneousMagnitude = other_945;
    return Line((_e4.g0_ * vec3<f32>(_e6.g0_.y)), (((((vec3<f32>(_e11.g1_.x) * vec3<f32>(_e15.g0_.y)) * vec3<f32>(1.0, 0.0, 0.0)) + ((vec3<f32>(_e25.g1_.y) * vec3<f32>(_e29.g0_.y)) * vec3<f32>(0.0, 1.0, 0.0))) + ((vec3<f32>(_e40.g1_.z) * vec3<f32>(_e44.g0_.y)) * vec3<f32>(0.0, 0.0, 1.0))) + (_e55.g0_ * vec3<f32>(_e57.g0_.x))));
}

fn line_homogeneous_magnitude_right_contraction(self_1098: Line, other_946: HomogeneousMagnitude) -> Line {
    var self_1099: Line;
    var other_947: HomogeneousMagnitude;

    self_1099 = self_1098;
    other_947 = other_946;
    let _e4: Line = self_1099;
    let _e6: HomogeneousMagnitude = other_947;
    let _e11: Line = self_1099;
    let _e13: HomogeneousMagnitude = other_947;
    return Line((_e4.g0_ * vec3<f32>(_e6.g0_.x)), (_e11.g1_ * vec3<f32>(_e13.g0_.x)));
}

fn line_homogeneous_magnitude_right_anti_contraction(self_1100: Line, other_948: HomogeneousMagnitude) -> Line {
    var self_1101: Line;
    var other_949: HomogeneousMagnitude;

    self_1101 = self_1100;
    other_949 = other_948;
    let _e4: Line = self_1101;
    let _e6: HomogeneousMagnitude = other_949;
    let _e11: Line = self_1101;
    let _e13: HomogeneousMagnitude = other_949;
    return Line((_e4.g0_ * vec3<f32>(_e6.g0_.y)), (_e11.g1_ * vec3<f32>(_e13.g0_.y)));
}

fn line_point_geometric_product(self_1102: Line, other_950: Point) -> Flector {
    var self_1103: Line;
    var other_951: Point;

    self_1103 = self_1102;
    other_951 = other_950;
    let _e4: Line = self_1103;
    let _e8: Point = other_951;
    let _e19: Line = self_1103;
    let _e23: Point = other_951;
    let _e35: Line = self_1103;
    let _e39: Point = other_951;
    let _e51: Line = self_1103;
    let _e55: Point = other_951;
    let _e67: Line = self_1103;
    let _e70: Line = self_1103;
    let _e73: Line = self_1103;
    let _e76: Line = self_1103;
    let _e80: Point = other_951;
    let _e92: Line = self_1103;
    let _e96: Point = other_951;
    let _e107: Line = self_1103;
    let _e111: Point = other_951;
    let _e123: Line = self_1103;
    let _e127: Point = other_951;
    let _e139: Line = self_1103;
    let _e143: Point = other_951;
    let _e155: Line = self_1103;
    let _e158: Line = self_1103;
    let _e161: Line = self_1103;
    let _e164: Line = self_1103;
    let _e168: Point = other_951;
    return Flector(((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y)) * vec4<f32>(0.0, 0.0, 0.0, 1.0)) + ((vec4<f32>(_e19.g0_.z) * vec4<f32>(_e23.g0_.z)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e35.g1_.y) * _e39.g0_.zzxz) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0))) + ((vec4<f32>(_e51.g1_.z) * _e55.g0_.yxyy) * vec4<f32>(1.0, -(1.0), 0.0, 0.0))) + ((vec4<f32>(_e67.g0_.x, _e70.g1_.x, _e73.g1_.x, _e76.g0_.x) * _e80.g0_.xzyx) * vec4<f32>(0.0, 1.0, -(1.0), 1.0))), ((((((vec4<f32>(_e92.g0_.y) * _e96.g0_.zzxz) * vec4<f32>(1.0, 0.0, -(1.0), 0.0)) + ((vec4<f32>(_e107.g0_.z) * _e111.g0_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))) + ((vec4<f32>(_e123.g1_.y) * _e127.g0_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0)))) + ((vec4<f32>(_e139.g1_.z) * _e143.g0_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e155.g1_.x, _e158.g0_.x, _e161.g0_.x, _e164.g1_.x) * _e168.g0_.wzyx) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))));
}

fn line_point_outer_product(self_1104: Line, other_952: Point) -> Plane {
    var self_1105: Line;
    var other_953: Point;

    self_1105 = self_1104;
    other_953 = other_952;
    let _e4: Line = self_1105;
    let _e8: Point = other_953;
    let _e19: Line = self_1105;
    let _e23: Point = other_953;
    let _e35: Line = self_1105;
    let _e39: Point = other_953;
    let _e51: Line = self_1105;
    let _e55: Point = other_953;
    let _e67: Line = self_1105;
    let _e70: Line = self_1105;
    let _e73: Line = self_1105;
    let _e76: Line = self_1105;
    let _e80: Point = other_953;
    return Plane(((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.zzxz) * vec4<f32>(1.0, 0.0, -(1.0), 0.0)) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))) + ((vec4<f32>(_e35.g1_.y) * _e39.g0_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0)))) + ((vec4<f32>(_e51.g1_.z) * _e55.g0_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e67.g1_.x, _e70.g0_.x, _e73.g0_.x, _e76.g1_.x) * _e80.g0_.wzyx) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))));
}

fn line_point_wedge(self_1106: Line, other_954: Point) -> Plane {
    var self_1107: Line;
    var other_955: Point;

    self_1107 = self_1106;
    other_955 = other_954;
    let _e4: Line = self_1107;
    let _e8: Point = other_955;
    let _e19: Line = self_1107;
    let _e23: Point = other_955;
    let _e35: Line = self_1107;
    let _e39: Point = other_955;
    let _e51: Line = self_1107;
    let _e55: Point = other_955;
    let _e67: Line = self_1107;
    let _e70: Line = self_1107;
    let _e73: Line = self_1107;
    let _e76: Line = self_1107;
    let _e80: Point = other_955;
    return Plane(((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.zzxz) * vec4<f32>(1.0, 0.0, -(1.0), 0.0)) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))) + ((vec4<f32>(_e35.g1_.y) * _e39.g0_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0)))) + ((vec4<f32>(_e51.g1_.z) * _e55.g0_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e67.g1_.x, _e70.g0_.x, _e73.g0_.x, _e76.g1_.x) * _e80.g0_.wzyx) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))));
}

fn line_point_join(self_1108: Line, other_956: Point) -> Plane {
    var self_1109: Line;
    var other_957: Point;

    self_1109 = self_1108;
    other_957 = other_956;
    let _e4: Line = self_1109;
    let _e8: Point = other_957;
    let _e19: Line = self_1109;
    let _e23: Point = other_957;
    let _e35: Line = self_1109;
    let _e39: Point = other_957;
    let _e51: Line = self_1109;
    let _e55: Point = other_957;
    let _e67: Line = self_1109;
    let _e70: Line = self_1109;
    let _e73: Line = self_1109;
    let _e76: Line = self_1109;
    let _e80: Point = other_957;
    return Plane(((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.zzxz) * vec4<f32>(1.0, 0.0, -(1.0), 0.0)) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))) + ((vec4<f32>(_e35.g1_.y) * _e39.g0_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0)))) + ((vec4<f32>(_e51.g1_.z) * _e55.g0_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e67.g1_.x, _e70.g0_.x, _e73.g0_.x, _e76.g1_.x) * _e80.g0_.wzyx) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))));
}

fn line_point_inner_product(self_1110: Line, other_958: Point) -> Point {
    var self_1111: Line;
    var other_959: Point;

    self_1111 = self_1110;
    other_959 = other_958;
    let _e4: Line = self_1111;
    let _e8: Point = other_959;
    let _e19: Line = self_1111;
    let _e23: Point = other_959;
    let _e35: Line = self_1111;
    let _e39: Point = other_959;
    let _e51: Line = self_1111;
    let _e55: Point = other_959;
    let _e67: Line = self_1111;
    let _e70: Line = self_1111;
    let _e73: Line = self_1111;
    let _e76: Line = self_1111;
    let _e80: Point = other_959;
    return Point(((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y)) * vec4<f32>(0.0, 0.0, 0.0, 1.0)) + ((vec4<f32>(_e19.g0_.z) * vec4<f32>(_e23.g0_.z)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e35.g1_.y) * _e39.g0_.zzxz) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0))) + ((vec4<f32>(_e51.g1_.z) * _e55.g0_.yxyy) * vec4<f32>(1.0, -(1.0), 0.0, 0.0))) + ((vec4<f32>(_e67.g0_.x, _e70.g1_.x, _e73.g1_.x, _e76.g0_.x) * _e80.g0_.xzyx) * vec4<f32>(0.0, 1.0, -(1.0), 1.0))));
}

fn line_point_inner_anti_product(self_1112: Line, other_960: Point) -> Plane {
    var self_1113: Line;
    var other_961: Point;

    self_1113 = self_1112;
    other_961 = other_960;
    let _e4: Line = self_1113;
    let _e8: Point = other_961;
    let _e19: Line = self_1113;
    let _e23: Point = other_961;
    let _e35: Line = self_1113;
    let _e39: Point = other_961;
    return Plane(((((vec4<f32>(_e4.g0_.y) * _e8.g0_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e35.g0_.x) * _e39.g0_.wxxx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))));
}

fn line_point_right_contraction(self_1114: Line, other_962: Point) -> Point {
    var self_1115: Line;
    var other_963: Point;

    self_1115 = self_1114;
    other_963 = other_962;
    let _e4: Line = self_1115;
    let _e8: Point = other_963;
    let _e19: Line = self_1115;
    let _e23: Point = other_963;
    let _e35: Line = self_1115;
    let _e39: Point = other_963;
    let _e51: Line = self_1115;
    let _e55: Point = other_963;
    let _e67: Line = self_1115;
    let _e70: Line = self_1115;
    let _e73: Line = self_1115;
    let _e76: Line = self_1115;
    let _e80: Point = other_963;
    return Point(((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y)) * vec4<f32>(0.0, 0.0, 0.0, 1.0)) + ((vec4<f32>(_e19.g0_.z) * vec4<f32>(_e23.g0_.z)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e35.g1_.y) * _e39.g0_.zzxz) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0))) + ((vec4<f32>(_e51.g1_.z) * _e55.g0_.yxyy) * vec4<f32>(1.0, -(1.0), 0.0, 0.0))) + ((vec4<f32>(_e67.g0_.x, _e70.g1_.x, _e73.g1_.x, _e76.g0_.x) * _e80.g0_.xzyx) * vec4<f32>(0.0, 1.0, -(1.0), 1.0))));
}

fn line_point_left_anti_contraction(self_1116: Line, other_964: Point) -> Plane {
    var self_1117: Line;
    var other_965: Point;

    self_1117 = self_1116;
    other_965 = other_964;
    let _e4: Line = self_1117;
    let _e8: Point = other_965;
    let _e19: Line = self_1117;
    let _e23: Point = other_965;
    let _e35: Line = self_1117;
    let _e39: Point = other_965;
    return Plane(((((vec4<f32>(_e4.g0_.y) * _e8.g0_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e35.g0_.x) * _e39.g0_.wxxx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))));
}

fn line_line_add(self_1118: Line, other_966: Line) -> Line {
    var self_1119: Line;
    var other_967: Line;

    self_1119 = self_1118;
    other_967 = other_966;
    let _e4: Line = self_1119;
    let _e6: Line = other_967;
    let _e9: Line = self_1119;
    let _e11: Line = other_967;
    return Line((_e4.g0_ + _e6.g0_), (_e9.g1_ + _e11.g1_));
}

fn line_line_sub(self_1120: Line, other_968: Line) -> Line {
    var self_1121: Line;
    var other_969: Line;

    self_1121 = self_1120;
    other_969 = other_968;
    let _e4: Line = self_1121;
    let _e6: Line = other_969;
    let _e9: Line = self_1121;
    let _e11: Line = other_969;
    return Line((_e4.g0_ - _e6.g0_), (_e9.g1_ - _e11.g1_));
}

fn line_line_mul(self_1122: Line, other_970: Line) -> Line {
    var self_1123: Line;
    var other_971: Line;

    self_1123 = self_1122;
    other_971 = other_970;
    let _e4: Line = self_1123;
    let _e6: Line = other_971;
    let _e9: Line = self_1123;
    let _e11: Line = other_971;
    return Line((_e4.g0_ * _e6.g0_), (_e9.g1_ * _e11.g1_));
}

fn line_line_div(self_1124: Line, other_972: Line) -> Line {
    var self_1125: Line;
    var other_973: Line;

    self_1125 = self_1124;
    other_973 = other_972;
    let _e4: Line = self_1125;
    let _e7: Line = self_1125;
    let _e10: Line = self_1125;
    let _e19: Line = other_973;
    let _e22: Line = other_973;
    let _e25: Line = other_973;
    let _e35: Line = self_1125;
    let _e38: Line = self_1125;
    let _e41: Line = self_1125;
    let _e50: Line = other_973;
    let _e53: Line = other_973;
    let _e56: Line = other_973;
    return Line((((vec3<f32>(_e4.g0_.x, _e7.g0_.y, _e10.g0_.z) * vec3<f32>(1.0, 1.0, 1.0)) / vec3<f32>(_e19.g0_.x, _e22.g0_.y, _e25.g0_.z)) * vec3<f32>(1.0, 1.0, 1.0)), (((vec3<f32>(_e35.g1_.x, _e38.g1_.y, _e41.g1_.z) * vec3<f32>(1.0, 1.0, 1.0)) / vec3<f32>(_e50.g1_.x, _e53.g1_.y, _e56.g1_.z)) * vec3<f32>(1.0, 1.0, 1.0)));
}

fn line_line_regressive_product(self_1126: Line, other_974: Line) -> Scalar {
    var self_1127: Line;
    var other_975: Line;

    self_1127 = self_1126;
    other_975 = other_974;
    let _e5: Line = self_1127;
    let _e8: Line = other_975;
    let _e13: Line = self_1127;
    let _e16: Line = other_975;
    let _e21: Line = self_1127;
    let _e24: Line = other_975;
    let _e29: Line = self_1127;
    let _e32: Line = other_975;
    let _e37: Line = self_1127;
    let _e40: Line = other_975;
    let _e45: Line = self_1127;
    let _e48: Line = other_975;
    return Scalar(((((((0.0 - (_e5.g0_.x * _e8.g1_.x)) - (_e13.g0_.y * _e16.g1_.y)) - (_e21.g0_.z * _e24.g1_.z)) - (_e29.g1_.x * _e32.g0_.x)) - (_e37.g1_.y * _e40.g0_.y)) - (_e45.g1_.z * _e48.g0_.z)));
}

fn line_line_anti_wedge(self_1128: Line, other_976: Line) -> Scalar {
    var self_1129: Line;
    var other_977: Line;

    self_1129 = self_1128;
    other_977 = other_976;
    let _e5: Line = self_1129;
    let _e8: Line = other_977;
    let _e13: Line = self_1129;
    let _e16: Line = other_977;
    let _e21: Line = self_1129;
    let _e24: Line = other_977;
    let _e29: Line = self_1129;
    let _e32: Line = other_977;
    let _e37: Line = self_1129;
    let _e40: Line = other_977;
    let _e45: Line = self_1129;
    let _e48: Line = other_977;
    return Scalar(((((((0.0 - (_e5.g0_.x * _e8.g1_.x)) - (_e13.g0_.y * _e16.g1_.y)) - (_e21.g0_.z * _e24.g1_.z)) - (_e29.g1_.x * _e32.g0_.x)) - (_e37.g1_.y * _e40.g0_.y)) - (_e45.g1_.z * _e48.g0_.z)));
}

fn line_line_meet(self_1130: Line, other_978: Line) -> Scalar {
    var self_1131: Line;
    var other_979: Line;

    self_1131 = self_1130;
    other_979 = other_978;
    let _e5: Line = self_1131;
    let _e8: Line = other_979;
    let _e13: Line = self_1131;
    let _e16: Line = other_979;
    let _e21: Line = self_1131;
    let _e24: Line = other_979;
    let _e29: Line = self_1131;
    let _e32: Line = other_979;
    let _e37: Line = self_1131;
    let _e40: Line = other_979;
    let _e45: Line = self_1131;
    let _e48: Line = other_979;
    return Scalar(((((((0.0 - (_e5.g0_.x * _e8.g1_.x)) - (_e13.g0_.y * _e16.g1_.y)) - (_e21.g0_.z * _e24.g1_.z)) - (_e29.g1_.x * _e32.g0_.x)) - (_e37.g1_.y * _e40.g0_.y)) - (_e45.g1_.z * _e48.g0_.z)));
}

fn line_line_outer_product(self_1132: Line, other_980: Line) -> AntiScalar {
    var self_1133: Line;
    var other_981: Line;

    self_1133 = self_1132;
    other_981 = other_980;
    let _e5: Line = self_1133;
    let _e8: Line = other_981;
    let _e13: Line = self_1133;
    let _e16: Line = other_981;
    let _e21: Line = self_1133;
    let _e24: Line = other_981;
    let _e29: Line = self_1133;
    let _e32: Line = other_981;
    let _e37: Line = self_1133;
    let _e40: Line = other_981;
    let _e45: Line = self_1133;
    let _e48: Line = other_981;
    return AntiScalar(((((((0.0 - (_e5.g0_.x * _e8.g1_.x)) - (_e13.g0_.y * _e16.g1_.y)) - (_e21.g0_.z * _e24.g1_.z)) - (_e29.g1_.x * _e32.g0_.x)) - (_e37.g1_.y * _e40.g0_.y)) - (_e45.g1_.z * _e48.g0_.z)));
}

fn line_line_wedge(self_1134: Line, other_982: Line) -> AntiScalar {
    var self_1135: Line;
    var other_983: Line;

    self_1135 = self_1134;
    other_983 = other_982;
    let _e5: Line = self_1135;
    let _e8: Line = other_983;
    let _e13: Line = self_1135;
    let _e16: Line = other_983;
    let _e21: Line = self_1135;
    let _e24: Line = other_983;
    let _e29: Line = self_1135;
    let _e32: Line = other_983;
    let _e37: Line = self_1135;
    let _e40: Line = other_983;
    let _e45: Line = self_1135;
    let _e48: Line = other_983;
    return AntiScalar(((((((0.0 - (_e5.g0_.x * _e8.g1_.x)) - (_e13.g0_.y * _e16.g1_.y)) - (_e21.g0_.z * _e24.g1_.z)) - (_e29.g1_.x * _e32.g0_.x)) - (_e37.g1_.y * _e40.g0_.y)) - (_e45.g1_.z * _e48.g0_.z)));
}

fn line_line_join(self_1136: Line, other_984: Line) -> AntiScalar {
    var self_1137: Line;
    var other_985: Line;

    self_1137 = self_1136;
    other_985 = other_984;
    let _e5: Line = self_1137;
    let _e8: Line = other_985;
    let _e13: Line = self_1137;
    let _e16: Line = other_985;
    let _e21: Line = self_1137;
    let _e24: Line = other_985;
    let _e29: Line = self_1137;
    let _e32: Line = other_985;
    let _e37: Line = self_1137;
    let _e40: Line = other_985;
    let _e45: Line = self_1137;
    let _e48: Line = other_985;
    return AntiScalar(((((((0.0 - (_e5.g0_.x * _e8.g1_.x)) - (_e13.g0_.y * _e16.g1_.y)) - (_e21.g0_.z * _e24.g1_.z)) - (_e29.g1_.x * _e32.g0_.x)) - (_e37.g1_.y * _e40.g0_.y)) - (_e45.g1_.z * _e48.g0_.z)));
}

fn line_line_inner_product(self_1138: Line, other_986: Line) -> Scalar {
    var self_1139: Line;
    var other_987: Line;

    self_1139 = self_1138;
    other_987 = other_986;
    let _e5: Line = self_1139;
    let _e8: Line = other_987;
    let _e13: Line = self_1139;
    let _e16: Line = other_987;
    let _e21: Line = self_1139;
    let _e24: Line = other_987;
    return Scalar((((0.0 - (_e5.g1_.x * _e8.g1_.x)) - (_e13.g1_.y * _e16.g1_.y)) - (_e21.g1_.z * _e24.g1_.z)));
}

fn line_line_inner_anti_product(self_1140: Line, other_988: Line) -> AntiScalar {
    var self_1141: Line;
    var other_989: Line;

    self_1141 = self_1140;
    other_989 = other_988;
    let _e5: Line = self_1141;
    let _e8: Line = other_989;
    let _e13: Line = self_1141;
    let _e16: Line = other_989;
    let _e21: Line = self_1141;
    let _e24: Line = other_989;
    return AntiScalar((((0.0 - (_e5.g0_.x * _e8.g0_.x)) - (_e13.g0_.y * _e16.g0_.y)) - (_e21.g0_.z * _e24.g0_.z)));
}

fn line_line_left_contraction(self_1142: Line, other_990: Line) -> Scalar {
    var self_1143: Line;
    var other_991: Line;

    self_1143 = self_1142;
    other_991 = other_990;
    let _e5: Line = self_1143;
    let _e8: Line = other_991;
    let _e13: Line = self_1143;
    let _e16: Line = other_991;
    let _e21: Line = self_1143;
    let _e24: Line = other_991;
    return Scalar((((0.0 - (_e5.g1_.x * _e8.g1_.x)) - (_e13.g1_.y * _e16.g1_.y)) - (_e21.g1_.z * _e24.g1_.z)));
}

fn line_line_right_contraction(self_1144: Line, other_992: Line) -> Scalar {
    var self_1145: Line;
    var other_993: Line;

    self_1145 = self_1144;
    other_993 = other_992;
    let _e5: Line = self_1145;
    let _e8: Line = other_993;
    let _e13: Line = self_1145;
    let _e16: Line = other_993;
    let _e21: Line = self_1145;
    let _e24: Line = other_993;
    return Scalar((((0.0 - (_e5.g1_.x * _e8.g1_.x)) - (_e13.g1_.y * _e16.g1_.y)) - (_e21.g1_.z * _e24.g1_.z)));
}

fn line_line_left_anti_contraction(self_1146: Line, other_994: Line) -> AntiScalar {
    var self_1147: Line;
    var other_995: Line;

    self_1147 = self_1146;
    other_995 = other_994;
    let _e5: Line = self_1147;
    let _e8: Line = other_995;
    let _e13: Line = self_1147;
    let _e16: Line = other_995;
    let _e21: Line = self_1147;
    let _e24: Line = other_995;
    return AntiScalar((((0.0 - (_e5.g0_.x * _e8.g0_.x)) - (_e13.g0_.y * _e16.g0_.y)) - (_e21.g0_.z * _e24.g0_.z)));
}

fn line_line_right_anti_contraction(self_1148: Line, other_996: Line) -> AntiScalar {
    var self_1149: Line;
    var other_997: Line;

    self_1149 = self_1148;
    other_997 = other_996;
    let _e5: Line = self_1149;
    let _e8: Line = other_997;
    let _e13: Line = self_1149;
    let _e16: Line = other_997;
    let _e21: Line = self_1149;
    let _e24: Line = other_997;
    return AntiScalar((((0.0 - (_e5.g0_.x * _e8.g0_.x)) - (_e13.g0_.y * _e16.g0_.y)) - (_e21.g0_.z * _e24.g0_.z)));
}

fn line_line_scalar_product(self_1150: Line, other_998: Line) -> Scalar {
    var self_1151: Line;
    var other_999: Line;

    self_1151 = self_1150;
    other_999 = other_998;
    let _e5: Line = self_1151;
    let _e8: Line = other_999;
    let _e13: Line = self_1151;
    let _e16: Line = other_999;
    let _e21: Line = self_1151;
    let _e24: Line = other_999;
    return Scalar((((0.0 - (_e5.g1_.x * _e8.g1_.x)) - (_e13.g1_.y * _e16.g1_.y)) - (_e21.g1_.z * _e24.g1_.z)));
}

fn line_line_dot(self_1152: Line, other_1000: Line) -> Scalar {
    var self_1153: Line;
    var other_1001: Line;

    self_1153 = self_1152;
    other_1001 = other_1000;
    let _e5: Line = self_1153;
    let _e8: Line = other_1001;
    let _e13: Line = self_1153;
    let _e16: Line = other_1001;
    let _e21: Line = self_1153;
    let _e24: Line = other_1001;
    return Scalar((((0.0 - (_e5.g1_.x * _e8.g1_.x)) - (_e13.g1_.y * _e16.g1_.y)) - (_e21.g1_.z * _e24.g1_.z)));
}

fn line_line_anti_scalar_product(self_1154: Line, other_1002: Line) -> AntiScalar {
    var self_1155: Line;
    var other_1003: Line;

    self_1155 = self_1154;
    other_1003 = other_1002;
    let _e5: Line = self_1155;
    let _e8: Line = other_1003;
    let _e13: Line = self_1155;
    let _e16: Line = other_1003;
    let _e21: Line = self_1155;
    let _e24: Line = other_1003;
    return AntiScalar((((0.0 - (_e5.g0_.x * _e8.g0_.x)) - (_e13.g0_.y * _e16.g0_.y)) - (_e21.g0_.z * _e24.g0_.z)));
}

fn line_line_anti_dot(self_1156: Line, other_1004: Line) -> AntiScalar {
    var self_1157: Line;
    var other_1005: Line;

    self_1157 = self_1156;
    other_1005 = other_1004;
    let _e5: Line = self_1157;
    let _e8: Line = other_1005;
    let _e13: Line = self_1157;
    let _e16: Line = other_1005;
    let _e21: Line = self_1157;
    let _e24: Line = other_1005;
    return AntiScalar((((0.0 - (_e5.g0_.x * _e8.g0_.x)) - (_e13.g0_.y * _e16.g0_.y)) - (_e21.g0_.z * _e24.g0_.z)));
}

fn line_plane_regressive_product(self_1158: Line, other_1006: Plane) -> Point {
    var self_1159: Line;
    var other_1007: Plane;

    self_1159 = self_1158;
    other_1007 = other_1006;
    let _e4: Line = self_1159;
    let _e8: Plane = other_1007;
    let _e19: Line = self_1159;
    let _e23: Plane = other_1007;
    let _e35: Line = self_1159;
    let _e39: Plane = other_1007;
    let _e51: Line = self_1159;
    let _e55: Plane = other_1007;
    let _e67: Line = self_1159;
    let _e70: Line = self_1159;
    let _e73: Line = self_1159;
    let _e76: Line = self_1159;
    let _e80: Plane = other_1007;
    return Point(((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e35.g1_.y) * _e39.g0_.zzxz) * vec4<f32>(1.0, 0.0, -(1.0), 0.0))) + ((vec4<f32>(_e51.g1_.z) * _e55.g0_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))) + ((vec4<f32>(_e67.g0_.x, _e70.g1_.x, _e73.g1_.x, _e76.g0_.x) * _e80.g0_.wzyx) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))));
}

fn line_plane_anti_wedge(self_1160: Line, other_1008: Plane) -> Point {
    var self_1161: Line;
    var other_1009: Plane;

    self_1161 = self_1160;
    other_1009 = other_1008;
    let _e4: Line = self_1161;
    let _e8: Plane = other_1009;
    let _e19: Line = self_1161;
    let _e23: Plane = other_1009;
    let _e35: Line = self_1161;
    let _e39: Plane = other_1009;
    let _e51: Line = self_1161;
    let _e55: Plane = other_1009;
    let _e67: Line = self_1161;
    let _e70: Line = self_1161;
    let _e73: Line = self_1161;
    let _e76: Line = self_1161;
    let _e80: Plane = other_1009;
    return Point(((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e35.g1_.y) * _e39.g0_.zzxz) * vec4<f32>(1.0, 0.0, -(1.0), 0.0))) + ((vec4<f32>(_e51.g1_.z) * _e55.g0_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))) + ((vec4<f32>(_e67.g0_.x, _e70.g1_.x, _e73.g1_.x, _e76.g0_.x) * _e80.g0_.wzyx) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))));
}

fn line_plane_meet(self_1162: Line, other_1010: Plane) -> Point {
    var self_1163: Line;
    var other_1011: Plane;

    self_1163 = self_1162;
    other_1011 = other_1010;
    let _e4: Line = self_1163;
    let _e8: Plane = other_1011;
    let _e19: Line = self_1163;
    let _e23: Plane = other_1011;
    let _e35: Line = self_1163;
    let _e39: Plane = other_1011;
    let _e51: Line = self_1163;
    let _e55: Plane = other_1011;
    let _e67: Line = self_1163;
    let _e70: Line = self_1163;
    let _e73: Line = self_1163;
    let _e76: Line = self_1163;
    let _e80: Plane = other_1011;
    return Point(((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e35.g1_.y) * _e39.g0_.zzxz) * vec4<f32>(1.0, 0.0, -(1.0), 0.0))) + ((vec4<f32>(_e51.g1_.z) * _e55.g0_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))) + ((vec4<f32>(_e67.g0_.x, _e70.g1_.x, _e73.g1_.x, _e76.g0_.x) * _e80.g0_.wzyx) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))));
}

fn line_plane_inner_product(self_1164: Line, other_1012: Plane) -> Point {
    var self_1165: Line;
    var other_1013: Plane;

    self_1165 = self_1164;
    other_1013 = other_1012;
    let _e4: Line = self_1165;
    let _e8: Plane = other_1013;
    let _e19: Line = self_1165;
    let _e23: Plane = other_1013;
    let _e35: Line = self_1165;
    let _e39: Plane = other_1013;
    return Point(((((vec4<f32>(_e4.g1_.y) * _e8.g0_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e19.g1_.z) * _e23.g0_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e35.g1_.x) * _e39.g0_.wxxx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))));
}

fn line_plane_geometric_anti_product(self_1166: Line, other_1014: Plane) -> Flector {
    var self_1167: Line;
    var other_1015: Plane;

    self_1167 = self_1166;
    other_1015 = other_1014;
    let _e4: Line = self_1167;
    let _e8: Plane = other_1015;
    let _e19: Line = self_1167;
    let _e23: Plane = other_1015;
    let _e35: Line = self_1167;
    let _e39: Plane = other_1015;
    let _e51: Line = self_1167;
    let _e55: Plane = other_1015;
    let _e67: Line = self_1167;
    let _e70: Line = self_1167;
    let _e73: Line = self_1167;
    let _e76: Line = self_1167;
    let _e80: Plane = other_1015;
    let _e93: Line = self_1167;
    let _e97: Plane = other_1015;
    let _e108: Line = self_1167;
    let _e112: Plane = other_1015;
    let _e124: Line = self_1167;
    let _e128: Plane = other_1015;
    let _e141: Line = self_1167;
    let _e145: Plane = other_1015;
    let _e158: Line = self_1167;
    let _e161: Line = self_1167;
    let _e164: Line = self_1167;
    let _e167: Line = self_1167;
    let _e171: Plane = other_1015;
    return Flector(((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e35.g1_.y) * _e39.g0_.zzxz) * vec4<f32>(1.0, 0.0, -(1.0), 0.0))) + ((vec4<f32>(_e51.g1_.z) * _e55.g0_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))) + ((vec4<f32>(_e67.g0_.x, _e70.g1_.x, _e73.g1_.x, _e76.g0_.x) * _e80.g0_.wzyx) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))), ((((((vec4<f32>(_e93.g0_.y) * _e97.g0_.zzxz) * vec4<f32>(1.0, 0.0, -(1.0), 0.0)) + ((vec4<f32>(_e108.g0_.z) * _e112.g0_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))) + ((vec4<f32>(_e124.g1_.y) * vec4<f32>(_e128.g0_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e141.g1_.z) * vec4<f32>(_e145.g0_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e158.g0_.x, _e161.g0_.x, _e164.g0_.x, _e167.g1_.x) * _e171.g0_.xzyx) * vec4<f32>(0.0, -(1.0), 1.0, -(1.0)))));
}

fn line_plane_inner_anti_product(self_1168: Line, other_1016: Plane) -> Plane {
    var self_1169: Line;
    var other_1017: Plane;

    self_1169 = self_1168;
    other_1017 = other_1016;
    let _e4: Line = self_1169;
    let _e8: Plane = other_1017;
    let _e19: Line = self_1169;
    let _e23: Plane = other_1017;
    let _e35: Line = self_1169;
    let _e39: Plane = other_1017;
    let _e52: Line = self_1169;
    let _e56: Plane = other_1017;
    let _e69: Line = self_1169;
    let _e72: Line = self_1169;
    let _e75: Line = self_1169;
    let _e78: Line = self_1169;
    let _e82: Plane = other_1017;
    return Plane(((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.zzxz) * vec4<f32>(1.0, 0.0, -(1.0), 0.0)) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))) + ((vec4<f32>(_e35.g1_.y) * vec4<f32>(_e39.g0_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e52.g1_.z) * vec4<f32>(_e56.g0_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e69.g0_.x, _e72.g0_.x, _e75.g0_.x, _e78.g1_.x) * _e82.g0_.xzyx) * vec4<f32>(0.0, -(1.0), 1.0, -(1.0)))));
}

fn line_plane_left_contraction(self_1170: Line, other_1018: Plane) -> Point {
    var self_1171: Line;
    var other_1019: Plane;

    self_1171 = self_1170;
    other_1019 = other_1018;
    let _e4: Line = self_1171;
    let _e8: Plane = other_1019;
    let _e19: Line = self_1171;
    let _e23: Plane = other_1019;
    let _e35: Line = self_1171;
    let _e39: Plane = other_1019;
    return Point(((((vec4<f32>(_e4.g1_.y) * _e8.g0_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e19.g1_.z) * _e23.g0_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e35.g1_.x) * _e39.g0_.wxxx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))));
}

fn line_plane_right_anti_contraction(self_1172: Line, other_1020: Plane) -> Plane {
    var self_1173: Line;
    var other_1021: Plane;

    self_1173 = self_1172;
    other_1021 = other_1020;
    let _e4: Line = self_1173;
    let _e8: Plane = other_1021;
    let _e19: Line = self_1173;
    let _e23: Plane = other_1021;
    let _e35: Line = self_1173;
    let _e39: Plane = other_1021;
    let _e52: Line = self_1173;
    let _e56: Plane = other_1021;
    let _e69: Line = self_1173;
    let _e72: Line = self_1173;
    let _e75: Line = self_1173;
    let _e78: Line = self_1173;
    let _e82: Plane = other_1021;
    return Plane(((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.zzxz) * vec4<f32>(1.0, 0.0, -(1.0), 0.0)) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))) + ((vec4<f32>(_e35.g1_.y) * vec4<f32>(_e39.g0_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e52.g1_.z) * vec4<f32>(_e56.g0_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e69.g0_.x, _e72.g0_.x, _e75.g0_.x, _e78.g1_.x) * _e82.g0_.xzyx) * vec4<f32>(0.0, -(1.0), 1.0, -(1.0)))));
}

fn line_motor_add(self_1174: Line, other_1022: Motor) -> Motor {
    var self_1175: Line;
    var other_1023: Motor;

    self_1175 = self_1174;
    other_1023 = other_1022;
    let _e4: Line = self_1175;
    let _e7: Line = self_1175;
    let _e10: Line = self_1175;
    let _e13: Line = self_1175;
    let _e23: Motor = other_1023;
    let _e26: Line = self_1175;
    let _e28: Motor = other_1023;
    return Motor(((vec4<f32>(_e4.g0_.x, _e7.g0_.y, _e10.g0_.z, _e13.g0_.x) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) + _e23.g0_), (_e26.g1_ + _e28.g1_));
}

fn line_motor_sub(self_1176: Line, other_1024: Motor) -> Motor {
    var self_1177: Line;
    var other_1025: Motor;

    self_1177 = self_1176;
    other_1025 = other_1024;
    let _e4: Line = self_1177;
    let _e7: Line = self_1177;
    let _e10: Line = self_1177;
    let _e13: Line = self_1177;
    let _e23: Motor = other_1025;
    let _e26: Line = self_1177;
    let _e28: Motor = other_1025;
    return Motor(((vec4<f32>(_e4.g0_.x, _e7.g0_.y, _e10.g0_.z, _e13.g0_.x) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) - _e23.g0_), (_e26.g1_ - _e28.g1_));
}

fn line_motor_outer_product(self_1178: Line, other_1026: Motor) -> AntiScalar {
    var self_1179: Line;
    var other_1027: Motor;

    self_1179 = self_1178;
    other_1027 = other_1026;
    let _e5: Line = self_1179;
    let _e8: Motor = other_1027;
    let _e13: Line = self_1179;
    let _e16: Motor = other_1027;
    let _e21: Line = self_1179;
    let _e24: Motor = other_1027;
    let _e29: Line = self_1179;
    let _e32: Motor = other_1027;
    let _e37: Line = self_1179;
    let _e40: Motor = other_1027;
    let _e45: Line = self_1179;
    let _e48: Motor = other_1027;
    return AntiScalar(((((((0.0 - (_e5.g0_.x * _e8.g1_.x)) - (_e13.g0_.y * _e16.g1_.y)) - (_e21.g0_.z * _e24.g1_.z)) - (_e29.g1_.x * _e32.g0_.x)) - (_e37.g1_.y * _e40.g0_.y)) - (_e45.g1_.z * _e48.g0_.z)));
}

fn line_motor_wedge(self_1180: Line, other_1028: Motor) -> AntiScalar {
    var self_1181: Line;
    var other_1029: Motor;

    self_1181 = self_1180;
    other_1029 = other_1028;
    let _e5: Line = self_1181;
    let _e8: Motor = other_1029;
    let _e13: Line = self_1181;
    let _e16: Motor = other_1029;
    let _e21: Line = self_1181;
    let _e24: Motor = other_1029;
    let _e29: Line = self_1181;
    let _e32: Motor = other_1029;
    let _e37: Line = self_1181;
    let _e40: Motor = other_1029;
    let _e45: Line = self_1181;
    let _e48: Motor = other_1029;
    return AntiScalar(((((((0.0 - (_e5.g0_.x * _e8.g1_.x)) - (_e13.g0_.y * _e16.g1_.y)) - (_e21.g0_.z * _e24.g1_.z)) - (_e29.g1_.x * _e32.g0_.x)) - (_e37.g1_.y * _e40.g0_.y)) - (_e45.g1_.z * _e48.g0_.z)));
}

fn line_motor_join(self_1182: Line, other_1030: Motor) -> AntiScalar {
    var self_1183: Line;
    var other_1031: Motor;

    self_1183 = self_1182;
    other_1031 = other_1030;
    let _e5: Line = self_1183;
    let _e8: Motor = other_1031;
    let _e13: Line = self_1183;
    let _e16: Motor = other_1031;
    let _e21: Line = self_1183;
    let _e24: Motor = other_1031;
    let _e29: Line = self_1183;
    let _e32: Motor = other_1031;
    let _e37: Line = self_1183;
    let _e40: Motor = other_1031;
    let _e45: Line = self_1183;
    let _e48: Motor = other_1031;
    return AntiScalar(((((((0.0 - (_e5.g0_.x * _e8.g1_.x)) - (_e13.g0_.y * _e16.g1_.y)) - (_e21.g0_.z * _e24.g1_.z)) - (_e29.g1_.x * _e32.g0_.x)) - (_e37.g1_.y * _e40.g0_.y)) - (_e45.g1_.z * _e48.g0_.z)));
}

fn line_motor_inner_anti_product(self_1184: Line, other_1032: Motor) -> Motor {
    var self_1185: Line;
    var other_1033: Motor;

    self_1185 = self_1184;
    other_1033 = other_1032;
    let _e4: Line = self_1185;
    let _e8: Motor = other_1033;
    let _e19: Line = self_1185;
    let _e23: Motor = other_1033;
    let _e35: Line = self_1185;
    let _e39: Motor = other_1033;
    let _e51: Line = self_1185;
    let _e53: Motor = other_1033;
    return Motor(((((vec4<f32>(_e4.g0_.y) * _e8.g0_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e35.g0_.x) * _e39.g0_.wxxx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))), (_e51.g1_ * vec3<f32>(_e53.g0_.w)));
}

fn line_motor_right_contraction(self_1186: Line, other_1034: Motor) -> Scalar {
    var self_1187: Line;
    var other_1035: Motor;

    self_1187 = self_1186;
    other_1035 = other_1034;
    let _e5: Line = self_1187;
    let _e8: Motor = other_1035;
    let _e13: Line = self_1187;
    let _e16: Motor = other_1035;
    let _e21: Line = self_1187;
    let _e24: Motor = other_1035;
    return Scalar((((0.0 - (_e5.g1_.x * _e8.g1_.x)) - (_e13.g1_.y * _e16.g1_.y)) - (_e21.g1_.z * _e24.g1_.z)));
}

fn line_motor_left_anti_contraction(self_1188: Line, other_1036: Motor) -> AntiScalar {
    var self_1189: Line;
    var other_1037: Motor;

    self_1189 = self_1188;
    other_1037 = other_1036;
    let _e5: Line = self_1189;
    let _e8: Motor = other_1037;
    let _e13: Line = self_1189;
    let _e16: Motor = other_1037;
    let _e21: Line = self_1189;
    let _e24: Motor = other_1037;
    return AntiScalar((((0.0 - (_e5.g0_.x * _e8.g0_.x)) - (_e13.g0_.y * _e16.g0_.y)) - (_e21.g0_.z * _e24.g0_.z)));
}

fn line_motor_right_anti_contraction(self_1190: Line, other_1038: Motor) -> Motor {
    var self_1191: Line;
    var other_1039: Motor;

    self_1191 = self_1190;
    other_1039 = other_1038;
    let _e4: Line = self_1191;
    let _e8: Motor = other_1039;
    let _e19: Line = self_1191;
    let _e23: Motor = other_1039;
    let _e35: Line = self_1191;
    let _e39: Motor = other_1039;
    let _e51: Line = self_1191;
    let _e53: Motor = other_1039;
    return Motor(((((vec4<f32>(_e4.g0_.y) * _e8.g0_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e35.g0_.x) * _e39.g0_.wxxx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))), (_e51.g1_ * vec3<f32>(_e53.g0_.w)));
}

fn line_motor_scalar_product(self_1192: Line, other_1040: Motor) -> Scalar {
    var self_1193: Line;
    var other_1041: Motor;

    self_1193 = self_1192;
    other_1041 = other_1040;
    let _e5: Line = self_1193;
    let _e8: Motor = other_1041;
    let _e13: Line = self_1193;
    let _e16: Motor = other_1041;
    let _e21: Line = self_1193;
    let _e24: Motor = other_1041;
    return Scalar((((0.0 - (_e5.g1_.x * _e8.g1_.x)) - (_e13.g1_.y * _e16.g1_.y)) - (_e21.g1_.z * _e24.g1_.z)));
}

fn line_motor_dot(self_1194: Line, other_1042: Motor) -> Scalar {
    var self_1195: Line;
    var other_1043: Motor;

    self_1195 = self_1194;
    other_1043 = other_1042;
    let _e5: Line = self_1195;
    let _e8: Motor = other_1043;
    let _e13: Line = self_1195;
    let _e16: Motor = other_1043;
    let _e21: Line = self_1195;
    let _e24: Motor = other_1043;
    return Scalar((((0.0 - (_e5.g1_.x * _e8.g1_.x)) - (_e13.g1_.y * _e16.g1_.y)) - (_e21.g1_.z * _e24.g1_.z)));
}

fn line_motor_anti_scalar_product(self_1196: Line, other_1044: Motor) -> AntiScalar {
    var self_1197: Line;
    var other_1045: Motor;

    self_1197 = self_1196;
    other_1045 = other_1044;
    let _e5: Line = self_1197;
    let _e8: Motor = other_1045;
    let _e13: Line = self_1197;
    let _e16: Motor = other_1045;
    let _e21: Line = self_1197;
    let _e24: Motor = other_1045;
    return AntiScalar((((0.0 - (_e5.g0_.x * _e8.g0_.x)) - (_e13.g0_.y * _e16.g0_.y)) - (_e21.g0_.z * _e24.g0_.z)));
}

fn line_motor_anti_dot(self_1198: Line, other_1046: Motor) -> AntiScalar {
    var self_1199: Line;
    var other_1047: Motor;

    self_1199 = self_1198;
    other_1047 = other_1046;
    let _e5: Line = self_1199;
    let _e8: Motor = other_1047;
    let _e13: Line = self_1199;
    let _e16: Motor = other_1047;
    let _e21: Line = self_1199;
    let _e24: Motor = other_1047;
    return AntiScalar((((0.0 - (_e5.g0_.x * _e8.g0_.x)) - (_e13.g0_.y * _e16.g0_.y)) - (_e21.g0_.z * _e24.g0_.z)));
}

fn line_rotor_add(self_1200: Line, other_1048: Rotor) -> Motor {
    var self_1201: Line;
    var other_1049: Rotor;

    self_1201 = self_1200;
    other_1049 = other_1048;
    let _e4: Line = self_1201;
    let _e7: Line = self_1201;
    let _e10: Line = self_1201;
    let _e13: Line = self_1201;
    let _e23: Rotor = other_1049;
    let _e26: Line = self_1201;
    return Motor(((vec4<f32>(_e4.g0_.x, _e7.g0_.y, _e10.g0_.z, _e13.g0_.x) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) + _e23.g0_), _e26.g1_);
}

fn line_rotor_sub(self_1202: Line, other_1050: Rotor) -> Motor {
    var self_1203: Line;
    var other_1051: Rotor;

    self_1203 = self_1202;
    other_1051 = other_1050;
    let _e4: Line = self_1203;
    let _e7: Line = self_1203;
    let _e10: Line = self_1203;
    let _e13: Line = self_1203;
    let _e23: Rotor = other_1051;
    let _e26: Line = self_1203;
    return Motor(((vec4<f32>(_e4.g0_.x, _e7.g0_.y, _e10.g0_.z, _e13.g0_.x) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) - _e23.g0_), _e26.g1_);
}

fn line_rotor_geometric_product(self_1204: Line, other_1052: Rotor) -> Rotor {
    var self_1205: Line;
    var other_1053: Rotor;

    self_1205 = self_1204;
    other_1053 = other_1052;
    let _e4: Line = self_1205;
    let _e8: Rotor = other_1053;
    let _e20: Line = self_1205;
    let _e24: Rotor = other_1053;
    let _e37: Line = self_1205;
    let _e41: Rotor = other_1053;
    return Rotor(((((vec4<f32>(_e4.g1_.x) * _e8.g0_.wzyx) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0))) + ((vec4<f32>(_e20.g1_.y) * _e24.g0_.zwxy) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e37.g1_.z) * _e41.g0_.yxwz) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))));
}

fn line_rotor_outer_product(self_1206: Line, other_1054: Rotor) -> AntiScalar {
    var self_1207: Line;
    var other_1055: Rotor;

    self_1207 = self_1206;
    other_1055 = other_1054;
    let _e5: Line = self_1207;
    let _e8: Rotor = other_1055;
    let _e13: Line = self_1207;
    let _e16: Rotor = other_1055;
    let _e21: Line = self_1207;
    let _e24: Rotor = other_1055;
    return AntiScalar((((0.0 - (_e5.g1_.x * _e8.g0_.x)) - (_e13.g1_.y * _e16.g0_.y)) - (_e21.g1_.z * _e24.g0_.z)));
}

fn line_rotor_wedge(self_1208: Line, other_1056: Rotor) -> AntiScalar {
    var self_1209: Line;
    var other_1057: Rotor;

    self_1209 = self_1208;
    other_1057 = other_1056;
    let _e5: Line = self_1209;
    let _e8: Rotor = other_1057;
    let _e13: Line = self_1209;
    let _e16: Rotor = other_1057;
    let _e21: Line = self_1209;
    let _e24: Rotor = other_1057;
    return AntiScalar((((0.0 - (_e5.g1_.x * _e8.g0_.x)) - (_e13.g1_.y * _e16.g0_.y)) - (_e21.g1_.z * _e24.g0_.z)));
}

fn line_rotor_join(self_1210: Line, other_1058: Rotor) -> AntiScalar {
    var self_1211: Line;
    var other_1059: Rotor;

    self_1211 = self_1210;
    other_1059 = other_1058;
    let _e5: Line = self_1211;
    let _e8: Rotor = other_1059;
    let _e13: Line = self_1211;
    let _e16: Rotor = other_1059;
    let _e21: Line = self_1211;
    let _e24: Rotor = other_1059;
    return AntiScalar((((0.0 - (_e5.g1_.x * _e8.g0_.x)) - (_e13.g1_.y * _e16.g0_.y)) - (_e21.g1_.z * _e24.g0_.z)));
}

fn line_rotor_inner_anti_product(self_1212: Line, other_1060: Rotor) -> Motor {
    var self_1213: Line;
    var other_1061: Rotor;

    self_1213 = self_1212;
    other_1061 = other_1060;
    let _e4: Line = self_1213;
    let _e8: Rotor = other_1061;
    let _e19: Line = self_1213;
    let _e23: Rotor = other_1061;
    let _e35: Line = self_1213;
    let _e39: Rotor = other_1061;
    let _e51: Line = self_1213;
    let _e53: Rotor = other_1061;
    return Motor(((((vec4<f32>(_e4.g0_.y) * _e8.g0_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e35.g0_.x) * _e39.g0_.wxxx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))), (_e51.g1_ * vec3<f32>(_e53.g0_.w)));
}

fn line_rotor_left_anti_contraction(self_1214: Line, other_1062: Rotor) -> AntiScalar {
    var self_1215: Line;
    var other_1063: Rotor;

    self_1215 = self_1214;
    other_1063 = other_1062;
    let _e5: Line = self_1215;
    let _e8: Rotor = other_1063;
    let _e13: Line = self_1215;
    let _e16: Rotor = other_1063;
    let _e21: Line = self_1215;
    let _e24: Rotor = other_1063;
    return AntiScalar((((0.0 - (_e5.g0_.x * _e8.g0_.x)) - (_e13.g0_.y * _e16.g0_.y)) - (_e21.g0_.z * _e24.g0_.z)));
}

fn line_rotor_right_anti_contraction(self_1216: Line, other_1064: Rotor) -> Motor {
    var self_1217: Line;
    var other_1065: Rotor;

    self_1217 = self_1216;
    other_1065 = other_1064;
    let _e4: Line = self_1217;
    let _e8: Rotor = other_1065;
    let _e19: Line = self_1217;
    let _e23: Rotor = other_1065;
    let _e35: Line = self_1217;
    let _e39: Rotor = other_1065;
    let _e51: Line = self_1217;
    let _e53: Rotor = other_1065;
    return Motor(((((vec4<f32>(_e4.g0_.y) * _e8.g0_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e35.g0_.x) * _e39.g0_.wxxx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))), (_e51.g1_ * vec3<f32>(_e53.g0_.w)));
}

fn line_rotor_anti_scalar_product(self_1218: Line, other_1066: Rotor) -> AntiScalar {
    var self_1219: Line;
    var other_1067: Rotor;

    self_1219 = self_1218;
    other_1067 = other_1066;
    let _e5: Line = self_1219;
    let _e8: Rotor = other_1067;
    let _e13: Line = self_1219;
    let _e16: Rotor = other_1067;
    let _e21: Line = self_1219;
    let _e24: Rotor = other_1067;
    return AntiScalar((((0.0 - (_e5.g0_.x * _e8.g0_.x)) - (_e13.g0_.y * _e16.g0_.y)) - (_e21.g0_.z * _e24.g0_.z)));
}

fn line_rotor_anti_dot(self_1220: Line, other_1068: Rotor) -> AntiScalar {
    var self_1221: Line;
    var other_1069: Rotor;

    self_1221 = self_1220;
    other_1069 = other_1068;
    let _e5: Line = self_1221;
    let _e8: Rotor = other_1069;
    let _e13: Line = self_1221;
    let _e16: Rotor = other_1069;
    let _e21: Line = self_1221;
    let _e24: Rotor = other_1069;
    return AntiScalar((((0.0 - (_e5.g0_.x * _e8.g0_.x)) - (_e13.g0_.y * _e16.g0_.y)) - (_e21.g0_.z * _e24.g0_.z)));
}

fn line_translator_add(self_1222: Line, other_1070: Translator) -> Motor {
    var self_1223: Line;
    var other_1071: Translator;

    self_1223 = self_1222;
    other_1071 = other_1070;
    let _e4: Line = self_1223;
    let _e7: Line = self_1223;
    let _e10: Line = self_1223;
    let _e13: Line = self_1223;
    let _e23: Translator = other_1071;
    let _e33: Line = self_1223;
    let _e35: Translator = other_1071;
    let _e38: Translator = other_1071;
    let _e41: Translator = other_1071;
    return Motor(((vec4<f32>(_e4.g0_.x, _e7.g0_.y, _e10.g0_.z, _e13.g0_.x) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) + (_e23.g0_.xxxw * vec4<f32>(0.0, 0.0, 0.0, 1.0))), (_e33.g1_ + vec3<f32>(_e35.g0_.x, _e38.g0_.y, _e41.g0_.z)));
}

fn line_translator_sub(self_1224: Line, other_1072: Translator) -> Motor {
    var self_1225: Line;
    var other_1073: Translator;

    self_1225 = self_1224;
    other_1073 = other_1072;
    let _e4: Line = self_1225;
    let _e7: Line = self_1225;
    let _e10: Line = self_1225;
    let _e13: Line = self_1225;
    let _e23: Translator = other_1073;
    let _e33: Line = self_1225;
    let _e35: Translator = other_1073;
    let _e38: Translator = other_1073;
    let _e41: Translator = other_1073;
    return Motor(((vec4<f32>(_e4.g0_.x, _e7.g0_.y, _e10.g0_.z, _e13.g0_.x) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) - (_e23.g0_.xxxw * vec4<f32>(0.0, 0.0, 0.0, 1.0))), (_e33.g1_ - vec3<f32>(_e35.g0_.x, _e38.g0_.y, _e41.g0_.z)));
}

fn line_translator_outer_product(self_1226: Line, other_1074: Translator) -> AntiScalar {
    var self_1227: Line;
    var other_1075: Translator;

    self_1227 = self_1226;
    other_1075 = other_1074;
    let _e5: Line = self_1227;
    let _e8: Translator = other_1075;
    let _e13: Line = self_1227;
    let _e16: Translator = other_1075;
    let _e21: Line = self_1227;
    let _e24: Translator = other_1075;
    return AntiScalar((((0.0 - (_e5.g0_.x * _e8.g0_.x)) - (_e13.g0_.y * _e16.g0_.y)) - (_e21.g0_.z * _e24.g0_.z)));
}

fn line_translator_wedge(self_1228: Line, other_1076: Translator) -> AntiScalar {
    var self_1229: Line;
    var other_1077: Translator;

    self_1229 = self_1228;
    other_1077 = other_1076;
    let _e5: Line = self_1229;
    let _e8: Translator = other_1077;
    let _e13: Line = self_1229;
    let _e16: Translator = other_1077;
    let _e21: Line = self_1229;
    let _e24: Translator = other_1077;
    return AntiScalar((((0.0 - (_e5.g0_.x * _e8.g0_.x)) - (_e13.g0_.y * _e16.g0_.y)) - (_e21.g0_.z * _e24.g0_.z)));
}

fn line_translator_join(self_1230: Line, other_1078: Translator) -> AntiScalar {
    var self_1231: Line;
    var other_1079: Translator;

    self_1231 = self_1230;
    other_1079 = other_1078;
    let _e5: Line = self_1231;
    let _e8: Translator = other_1079;
    let _e13: Line = self_1231;
    let _e16: Translator = other_1079;
    let _e21: Line = self_1231;
    let _e24: Translator = other_1079;
    return AntiScalar((((0.0 - (_e5.g0_.x * _e8.g0_.x)) - (_e13.g0_.y * _e16.g0_.y)) - (_e21.g0_.z * _e24.g0_.z)));
}

fn line_translator_inner_anti_product(self_1232: Line, other_1080: Translator) -> Line {
    var self_1233: Line;
    var other_1081: Translator;

    self_1233 = self_1232;
    other_1081 = other_1080;
    let _e4: Line = self_1233;
    let _e6: Translator = other_1081;
    let _e11: Line = self_1233;
    let _e13: Translator = other_1081;
    return Line((_e4.g0_ * vec3<f32>(_e6.g0_.w)), (_e11.g1_ * vec3<f32>(_e13.g0_.w)));
}

fn line_translator_right_contraction(self_1234: Line, other_1082: Translator) -> Scalar {
    var self_1235: Line;
    var other_1083: Translator;

    self_1235 = self_1234;
    other_1083 = other_1082;
    let _e5: Line = self_1235;
    let _e8: Translator = other_1083;
    let _e13: Line = self_1235;
    let _e16: Translator = other_1083;
    let _e21: Line = self_1235;
    let _e24: Translator = other_1083;
    return Scalar((((0.0 - (_e5.g1_.x * _e8.g0_.x)) - (_e13.g1_.y * _e16.g0_.y)) - (_e21.g1_.z * _e24.g0_.z)));
}

fn line_translator_right_anti_contraction(self_1236: Line, other_1084: Translator) -> Line {
    var self_1237: Line;
    var other_1085: Translator;

    self_1237 = self_1236;
    other_1085 = other_1084;
    let _e4: Line = self_1237;
    let _e6: Translator = other_1085;
    let _e11: Line = self_1237;
    let _e13: Translator = other_1085;
    return Line((_e4.g0_ * vec3<f32>(_e6.g0_.w)), (_e11.g1_ * vec3<f32>(_e13.g0_.w)));
}

fn line_translator_scalar_product(self_1238: Line, other_1086: Translator) -> Scalar {
    var self_1239: Line;
    var other_1087: Translator;

    self_1239 = self_1238;
    other_1087 = other_1086;
    let _e5: Line = self_1239;
    let _e8: Translator = other_1087;
    let _e13: Line = self_1239;
    let _e16: Translator = other_1087;
    let _e21: Line = self_1239;
    let _e24: Translator = other_1087;
    return Scalar((((0.0 - (_e5.g1_.x * _e8.g0_.x)) - (_e13.g1_.y * _e16.g0_.y)) - (_e21.g1_.z * _e24.g0_.z)));
}

fn line_translator_dot(self_1240: Line, other_1088: Translator) -> Scalar {
    var self_1241: Line;
    var other_1089: Translator;

    self_1241 = self_1240;
    other_1089 = other_1088;
    let _e5: Line = self_1241;
    let _e8: Translator = other_1089;
    let _e13: Line = self_1241;
    let _e16: Translator = other_1089;
    let _e21: Line = self_1241;
    let _e24: Translator = other_1089;
    return Scalar((((0.0 - (_e5.g1_.x * _e8.g0_.x)) - (_e13.g1_.y * _e16.g0_.y)) - (_e21.g1_.z * _e24.g0_.z)));
}

fn line_flector_geometric_product(self_1242: Line, other_1090: Flector) -> Flector {
    var self_1243: Line;
    var other_1091: Flector;

    self_1243 = self_1242;
    other_1091 = other_1090;
    let _e4: Line = self_1243;
    let _e8: Flector = other_1091;
    let _e19: Line = self_1243;
    let _e23: Flector = other_1091;
    let _e35: Line = self_1243;
    let _e39: Flector = other_1091;
    let _e42: Flector = other_1091;
    let _e45: Flector = other_1091;
    let _e48: Flector = other_1091;
    let _e62: Line = self_1243;
    let _e66: Flector = other_1091;
    let _e69: Flector = other_1091;
    let _e72: Flector = other_1091;
    let _e75: Flector = other_1091;
    let _e89: Line = self_1243;
    let _e93: Flector = other_1091;
    let _e96: Flector = other_1091;
    let _e99: Flector = other_1091;
    let _e102: Flector = other_1091;
    let _e116: Line = self_1243;
    let _e120: Flector = other_1091;
    let _e132: Line = self_1243;
    let _e136: Flector = other_1091;
    let _e139: Flector = other_1091;
    let _e142: Flector = other_1091;
    let _e145: Flector = other_1091;
    let _e158: Line = self_1243;
    let _e162: Flector = other_1091;
    let _e165: Flector = other_1091;
    let _e168: Flector = other_1091;
    let _e171: Flector = other_1091;
    let _e185: Line = self_1243;
    let _e189: Flector = other_1091;
    let _e192: Flector = other_1091;
    let _e195: Flector = other_1091;
    let _e198: Flector = other_1091;
    let _e212: Line = self_1243;
    let _e216: Flector = other_1091;
    let _e219: Flector = other_1091;
    let _e222: Flector = other_1091;
    let _e225: Flector = other_1091;
    let _e239: Line = self_1243;
    let _e243: Flector = other_1091;
    let _e246: Flector = other_1091;
    let _e249: Flector = other_1091;
    let _e252: Flector = other_1091;
    let _e266: Line = self_1243;
    let _e270: Flector = other_1091;
    let _e273: Flector = other_1091;
    let _e276: Flector = other_1091;
    let _e279: Flector = other_1091;
    return Flector((((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y)) * vec4<f32>(0.0, 0.0, 0.0, 1.0)) + ((vec4<f32>(_e19.g0_.z) * vec4<f32>(_e23.g0_.z)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e35.g1_.x) * vec4<f32>(_e39.g1_.w, _e42.g0_.z, _e45.g0_.y, _e48.g1_.x)) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e62.g1_.y) * vec4<f32>(_e66.g0_.z, _e69.g1_.w, _e72.g0_.x, _e75.g1_.y)) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e89.g1_.z) * vec4<f32>(_e93.g0_.y, _e96.g0_.x, _e99.g1_.w, _e102.g1_.z)) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e116.g0_.x) * vec4<f32>(_e120.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))), (((((((vec4<f32>(_e132.g0_.y) * vec4<f32>(_e136.g0_.z, _e139.g1_.w, _e142.g0_.x, _e145.g0_.z)) * vec4<f32>(1.0, -(1.0), -(1.0), 0.0)) + ((vec4<f32>(_e158.g0_.z) * vec4<f32>(_e162.g0_.y, _e165.g0_.x, _e168.g1_.w, _e171.g0_.y)) * vec4<f32>(-(1.0), 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e185.g1_.x) * vec4<f32>(_e189.g0_.w, _e192.g1_.z, _e195.g1_.y, _e198.g0_.x)) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e212.g1_.y) * vec4<f32>(_e216.g1_.z, _e219.g0_.w, _e222.g1_.x, _e225.g0_.y)) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e239.g1_.z) * vec4<f32>(_e243.g1_.y, _e246.g1_.x, _e249.g0_.w, _e252.g0_.z)) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e266.g0_.x) * vec4<f32>(_e270.g1_.w, _e273.g0_.z, _e276.g0_.y, _e279.g1_.x)) * vec4<f32>(-(1.0), -(1.0), 1.0, 0.0))));
}

fn line_flector_regressive_product(self_1244: Line, other_1092: Flector) -> Point {
    var self_1245: Line;
    var other_1093: Flector;

    self_1245 = self_1244;
    other_1093 = other_1092;
    let _e4: Line = self_1245;
    let _e8: Flector = other_1093;
    let _e19: Line = self_1245;
    let _e23: Flector = other_1093;
    let _e35: Line = self_1245;
    let _e39: Flector = other_1093;
    let _e51: Line = self_1245;
    let _e55: Flector = other_1093;
    let _e67: Line = self_1245;
    let _e70: Line = self_1245;
    let _e73: Line = self_1245;
    let _e76: Line = self_1245;
    let _e80: Flector = other_1093;
    return Point(((((((vec4<f32>(_e4.g0_.y) * _e8.g1_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e19.g0_.z) * _e23.g1_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e35.g1_.y) * _e39.g1_.zzxz) * vec4<f32>(1.0, 0.0, -(1.0), 0.0))) + ((vec4<f32>(_e51.g1_.z) * _e55.g1_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))) + ((vec4<f32>(_e67.g0_.x, _e70.g1_.x, _e73.g1_.x, _e76.g0_.x) * _e80.g1_.wzyx) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))));
}

fn line_flector_anti_wedge(self_1246: Line, other_1094: Flector) -> Point {
    var self_1247: Line;
    var other_1095: Flector;

    self_1247 = self_1246;
    other_1095 = other_1094;
    let _e4: Line = self_1247;
    let _e8: Flector = other_1095;
    let _e19: Line = self_1247;
    let _e23: Flector = other_1095;
    let _e35: Line = self_1247;
    let _e39: Flector = other_1095;
    let _e51: Line = self_1247;
    let _e55: Flector = other_1095;
    let _e67: Line = self_1247;
    let _e70: Line = self_1247;
    let _e73: Line = self_1247;
    let _e76: Line = self_1247;
    let _e80: Flector = other_1095;
    return Point(((((((vec4<f32>(_e4.g0_.y) * _e8.g1_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e19.g0_.z) * _e23.g1_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e35.g1_.y) * _e39.g1_.zzxz) * vec4<f32>(1.0, 0.0, -(1.0), 0.0))) + ((vec4<f32>(_e51.g1_.z) * _e55.g1_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))) + ((vec4<f32>(_e67.g0_.x, _e70.g1_.x, _e73.g1_.x, _e76.g0_.x) * _e80.g1_.wzyx) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))));
}

fn line_flector_meet(self_1248: Line, other_1096: Flector) -> Point {
    var self_1249: Line;
    var other_1097: Flector;

    self_1249 = self_1248;
    other_1097 = other_1096;
    let _e4: Line = self_1249;
    let _e8: Flector = other_1097;
    let _e19: Line = self_1249;
    let _e23: Flector = other_1097;
    let _e35: Line = self_1249;
    let _e39: Flector = other_1097;
    let _e51: Line = self_1249;
    let _e55: Flector = other_1097;
    let _e67: Line = self_1249;
    let _e70: Line = self_1249;
    let _e73: Line = self_1249;
    let _e76: Line = self_1249;
    let _e80: Flector = other_1097;
    return Point(((((((vec4<f32>(_e4.g0_.y) * _e8.g1_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e19.g0_.z) * _e23.g1_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e35.g1_.y) * _e39.g1_.zzxz) * vec4<f32>(1.0, 0.0, -(1.0), 0.0))) + ((vec4<f32>(_e51.g1_.z) * _e55.g1_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))) + ((vec4<f32>(_e67.g0_.x, _e70.g1_.x, _e73.g1_.x, _e76.g0_.x) * _e80.g1_.wzyx) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))));
}

fn line_flector_outer_product(self_1250: Line, other_1098: Flector) -> Plane {
    var self_1251: Line;
    var other_1099: Flector;

    self_1251 = self_1250;
    other_1099 = other_1098;
    let _e4: Line = self_1251;
    let _e8: Flector = other_1099;
    let _e19: Line = self_1251;
    let _e23: Flector = other_1099;
    let _e35: Line = self_1251;
    let _e39: Flector = other_1099;
    let _e51: Line = self_1251;
    let _e55: Flector = other_1099;
    let _e67: Line = self_1251;
    let _e70: Line = self_1251;
    let _e73: Line = self_1251;
    let _e76: Line = self_1251;
    let _e80: Flector = other_1099;
    return Plane(((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.zzxz) * vec4<f32>(1.0, 0.0, -(1.0), 0.0)) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))) + ((vec4<f32>(_e35.g1_.y) * _e39.g0_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0)))) + ((vec4<f32>(_e51.g1_.z) * _e55.g0_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e67.g1_.x, _e70.g0_.x, _e73.g0_.x, _e76.g1_.x) * _e80.g0_.wzyx) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))));
}

fn line_flector_wedge(self_1252: Line, other_1100: Flector) -> Plane {
    var self_1253: Line;
    var other_1101: Flector;

    self_1253 = self_1252;
    other_1101 = other_1100;
    let _e4: Line = self_1253;
    let _e8: Flector = other_1101;
    let _e19: Line = self_1253;
    let _e23: Flector = other_1101;
    let _e35: Line = self_1253;
    let _e39: Flector = other_1101;
    let _e51: Line = self_1253;
    let _e55: Flector = other_1101;
    let _e67: Line = self_1253;
    let _e70: Line = self_1253;
    let _e73: Line = self_1253;
    let _e76: Line = self_1253;
    let _e80: Flector = other_1101;
    return Plane(((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.zzxz) * vec4<f32>(1.0, 0.0, -(1.0), 0.0)) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))) + ((vec4<f32>(_e35.g1_.y) * _e39.g0_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0)))) + ((vec4<f32>(_e51.g1_.z) * _e55.g0_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e67.g1_.x, _e70.g0_.x, _e73.g0_.x, _e76.g1_.x) * _e80.g0_.wzyx) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))));
}

fn line_flector_join(self_1254: Line, other_1102: Flector) -> Plane {
    var self_1255: Line;
    var other_1103: Flector;

    self_1255 = self_1254;
    other_1103 = other_1102;
    let _e4: Line = self_1255;
    let _e8: Flector = other_1103;
    let _e19: Line = self_1255;
    let _e23: Flector = other_1103;
    let _e35: Line = self_1255;
    let _e39: Flector = other_1103;
    let _e51: Line = self_1255;
    let _e55: Flector = other_1103;
    let _e67: Line = self_1255;
    let _e70: Line = self_1255;
    let _e73: Line = self_1255;
    let _e76: Line = self_1255;
    let _e80: Flector = other_1103;
    return Plane(((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.zzxz) * vec4<f32>(1.0, 0.0, -(1.0), 0.0)) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))) + ((vec4<f32>(_e35.g1_.y) * _e39.g0_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0)))) + ((vec4<f32>(_e51.g1_.z) * _e55.g0_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e67.g1_.x, _e70.g0_.x, _e73.g0_.x, _e76.g1_.x) * _e80.g0_.wzyx) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))));
}

fn line_flector_inner_product(self_1256: Line, other_1104: Flector) -> Point {
    var self_1257: Line;
    var other_1105: Flector;

    self_1257 = self_1256;
    other_1105 = other_1104;
    let _e4: Line = self_1257;
    let _e8: Flector = other_1105;
    let _e19: Line = self_1257;
    let _e23: Flector = other_1105;
    let _e35: Line = self_1257;
    let _e39: Flector = other_1105;
    let _e42: Flector = other_1105;
    let _e45: Flector = other_1105;
    let _e48: Flector = other_1105;
    let _e62: Line = self_1257;
    let _e66: Flector = other_1105;
    let _e69: Flector = other_1105;
    let _e72: Flector = other_1105;
    let _e75: Flector = other_1105;
    let _e89: Line = self_1257;
    let _e93: Flector = other_1105;
    let _e96: Flector = other_1105;
    let _e99: Flector = other_1105;
    let _e102: Flector = other_1105;
    let _e116: Line = self_1257;
    let _e120: Flector = other_1105;
    return Point((((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y)) * vec4<f32>(0.0, 0.0, 0.0, 1.0)) + ((vec4<f32>(_e19.g0_.z) * vec4<f32>(_e23.g0_.z)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e35.g1_.x) * vec4<f32>(_e39.g1_.w, _e42.g0_.z, _e45.g0_.y, _e48.g1_.x)) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e62.g1_.y) * vec4<f32>(_e66.g0_.z, _e69.g1_.w, _e72.g0_.x, _e75.g1_.y)) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e89.g1_.z) * vec4<f32>(_e93.g0_.y, _e96.g0_.x, _e99.g1_.w, _e102.g1_.z)) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e116.g0_.x) * vec4<f32>(_e120.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))));
}

fn line_flector_geometric_anti_product(self_1258: Line, other_1106: Flector) -> Flector {
    var self_1259: Line;
    var other_1107: Flector;

    self_1259 = self_1258;
    other_1107 = other_1106;
    let _e4: Line = self_1259;
    let _e8: Flector = other_1107;
    let _e11: Flector = other_1107;
    let _e14: Flector = other_1107;
    let _e17: Flector = other_1107;
    let _e30: Line = self_1259;
    let _e34: Flector = other_1107;
    let _e37: Flector = other_1107;
    let _e40: Flector = other_1107;
    let _e43: Flector = other_1107;
    let _e57: Line = self_1259;
    let _e61: Flector = other_1107;
    let _e64: Flector = other_1107;
    let _e67: Flector = other_1107;
    let _e70: Flector = other_1107;
    let _e84: Line = self_1259;
    let _e88: Flector = other_1107;
    let _e91: Flector = other_1107;
    let _e94: Flector = other_1107;
    let _e97: Flector = other_1107;
    let _e110: Line = self_1259;
    let _e114: Flector = other_1107;
    let _e117: Flector = other_1107;
    let _e120: Flector = other_1107;
    let _e123: Flector = other_1107;
    let _e136: Line = self_1259;
    let _e140: Flector = other_1107;
    let _e143: Flector = other_1107;
    let _e146: Flector = other_1107;
    let _e149: Flector = other_1107;
    let _e162: Line = self_1259;
    let _e166: Flector = other_1107;
    let _e169: Flector = other_1107;
    let _e172: Flector = other_1107;
    let _e175: Flector = other_1107;
    let _e188: Line = self_1259;
    let _e192: Flector = other_1107;
    let _e195: Flector = other_1107;
    let _e198: Flector = other_1107;
    let _e201: Flector = other_1107;
    let _e215: Line = self_1259;
    let _e219: Flector = other_1107;
    let _e222: Flector = other_1107;
    let _e225: Flector = other_1107;
    let _e228: Flector = other_1107;
    let _e242: Line = self_1259;
    let _e246: Flector = other_1107;
    let _e259: Line = self_1259;
    let _e263: Flector = other_1107;
    let _e276: Line = self_1259;
    let _e280: Flector = other_1107;
    return Flector((((((((vec4<f32>(_e4.g0_.x) * vec4<f32>(_e8.g1_.w, _e11.g0_.z, _e14.g0_.y, _e17.g1_.x)) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0))) + ((vec4<f32>(_e30.g0_.y) * vec4<f32>(_e34.g0_.z, _e37.g1_.w, _e40.g0_.x, _e43.g1_.y)) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e57.g0_.z) * vec4<f32>(_e61.g0_.y, _e64.g0_.x, _e67.g1_.w, _e70.g1_.z)) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e84.g1_.y) * vec4<f32>(_e88.g1_.z, _e91.g0_.w, _e94.g1_.x, _e97.g1_.z)) * vec4<f32>(1.0, 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e110.g1_.z) * vec4<f32>(_e114.g1_.y, _e117.g1_.x, _e120.g0_.w, _e123.g1_.y)) * vec4<f32>(-(1.0), 1.0, 1.0, 0.0))) + ((vec4<f32>(_e136.g1_.x) * vec4<f32>(_e140.g0_.w, _e143.g1_.z, _e146.g1_.y, _e149.g0_.x)) * vec4<f32>(1.0, -(1.0), 1.0, 0.0))), (((((((vec4<f32>(_e162.g0_.x) * vec4<f32>(_e166.g0_.w, _e169.g1_.z, _e172.g1_.y, _e175.g0_.x)) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0))) + ((vec4<f32>(_e188.g0_.y) * vec4<f32>(_e192.g1_.z, _e195.g0_.w, _e198.g1_.x, _e201.g0_.y)) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e215.g0_.z) * vec4<f32>(_e219.g1_.y, _e222.g1_.x, _e225.g0_.w, _e228.g0_.z)) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e242.g1_.y) * vec4<f32>(_e246.g1_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e259.g1_.z) * vec4<f32>(_e263.g1_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e276.g1_.x) * vec4<f32>(_e280.g1_.x)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))));
}

fn line_flector_inner_anti_product(self_1260: Line, other_1108: Flector) -> Plane {
    var self_1261: Line;
    var other_1109: Flector;

    self_1261 = self_1260;
    other_1109 = other_1108;
    let _e4: Line = self_1261;
    let _e8: Flector = other_1109;
    let _e11: Flector = other_1109;
    let _e14: Flector = other_1109;
    let _e17: Flector = other_1109;
    let _e30: Line = self_1261;
    let _e34: Flector = other_1109;
    let _e37: Flector = other_1109;
    let _e40: Flector = other_1109;
    let _e43: Flector = other_1109;
    let _e57: Line = self_1261;
    let _e61: Flector = other_1109;
    let _e64: Flector = other_1109;
    let _e67: Flector = other_1109;
    let _e70: Flector = other_1109;
    let _e84: Line = self_1261;
    let _e88: Flector = other_1109;
    let _e101: Line = self_1261;
    let _e105: Flector = other_1109;
    let _e118: Line = self_1261;
    let _e122: Flector = other_1109;
    return Plane((((((((vec4<f32>(_e4.g0_.x) * vec4<f32>(_e8.g0_.w, _e11.g1_.z, _e14.g1_.y, _e17.g0_.x)) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0))) + ((vec4<f32>(_e30.g0_.y) * vec4<f32>(_e34.g1_.z, _e37.g0_.w, _e40.g1_.x, _e43.g0_.y)) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e57.g0_.z) * vec4<f32>(_e61.g1_.y, _e64.g1_.x, _e67.g0_.w, _e70.g0_.z)) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e84.g1_.y) * vec4<f32>(_e88.g1_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e101.g1_.z) * vec4<f32>(_e105.g1_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e118.g1_.x) * vec4<f32>(_e122.g1_.x)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))));
}

fn line_flector_left_contraction(self_1262: Line, other_1110: Flector) -> Point {
    var self_1263: Line;
    var other_1111: Flector;

    self_1263 = self_1262;
    other_1111 = other_1110;
    let _e4: Line = self_1263;
    let _e8: Flector = other_1111;
    let _e19: Line = self_1263;
    let _e23: Flector = other_1111;
    let _e35: Line = self_1263;
    let _e39: Flector = other_1111;
    return Point(((((vec4<f32>(_e4.g1_.y) * _e8.g1_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e19.g1_.z) * _e23.g1_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e35.g1_.x) * _e39.g1_.wxxx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))));
}

fn line_flector_right_contraction(self_1264: Line, other_1112: Flector) -> Point {
    var self_1265: Line;
    var other_1113: Flector;

    self_1265 = self_1264;
    other_1113 = other_1112;
    let _e4: Line = self_1265;
    let _e8: Flector = other_1113;
    let _e19: Line = self_1265;
    let _e23: Flector = other_1113;
    let _e35: Line = self_1265;
    let _e39: Flector = other_1113;
    let _e51: Line = self_1265;
    let _e55: Flector = other_1113;
    let _e67: Line = self_1265;
    let _e70: Line = self_1265;
    let _e73: Line = self_1265;
    let _e76: Line = self_1265;
    let _e80: Flector = other_1113;
    return Point(((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y)) * vec4<f32>(0.0, 0.0, 0.0, 1.0)) + ((vec4<f32>(_e19.g0_.z) * vec4<f32>(_e23.g0_.z)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e35.g1_.y) * _e39.g0_.zzxz) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0))) + ((vec4<f32>(_e51.g1_.z) * _e55.g0_.yxyy) * vec4<f32>(1.0, -(1.0), 0.0, 0.0))) + ((vec4<f32>(_e67.g0_.x, _e70.g1_.x, _e73.g1_.x, _e76.g0_.x) * _e80.g0_.xzyx) * vec4<f32>(0.0, 1.0, -(1.0), 1.0))));
}

fn line_flector_left_anti_contraction(self_1266: Line, other_1114: Flector) -> Plane {
    var self_1267: Line;
    var other_1115: Flector;

    self_1267 = self_1266;
    other_1115 = other_1114;
    let _e4: Line = self_1267;
    let _e8: Flector = other_1115;
    let _e19: Line = self_1267;
    let _e23: Flector = other_1115;
    let _e35: Line = self_1267;
    let _e39: Flector = other_1115;
    return Plane(((((vec4<f32>(_e4.g0_.y) * _e8.g0_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e35.g0_.x) * _e39.g0_.wxxx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))));
}

fn line_flector_right_anti_contraction(self_1268: Line, other_1116: Flector) -> Plane {
    var self_1269: Line;
    var other_1117: Flector;

    self_1269 = self_1268;
    other_1117 = other_1116;
    let _e4: Line = self_1269;
    let _e8: Flector = other_1117;
    let _e19: Line = self_1269;
    let _e23: Flector = other_1117;
    let _e35: Line = self_1269;
    let _e39: Flector = other_1117;
    let _e52: Line = self_1269;
    let _e56: Flector = other_1117;
    let _e69: Line = self_1269;
    let _e72: Line = self_1269;
    let _e75: Line = self_1269;
    let _e78: Line = self_1269;
    let _e82: Flector = other_1117;
    return Plane(((((((vec4<f32>(_e4.g0_.y) * _e8.g1_.zzxz) * vec4<f32>(1.0, 0.0, -(1.0), 0.0)) + ((vec4<f32>(_e19.g0_.z) * _e23.g1_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))) + ((vec4<f32>(_e35.g1_.y) * vec4<f32>(_e39.g1_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e52.g1_.z) * vec4<f32>(_e56.g1_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e69.g0_.x, _e72.g0_.x, _e75.g0_.x, _e78.g1_.x) * _e82.g1_.xzyx) * vec4<f32>(0.0, -(1.0), 1.0, -(1.0)))));
}

fn line_multi_vector_scalar_product(self_1270: Line, other_1118: MultiVector) -> Scalar {
    var self_1271: Line;
    var other_1119: MultiVector;

    self_1271 = self_1270;
    other_1119 = other_1118;
    let _e5: Line = self_1271;
    let _e8: MultiVector = other_1119;
    let _e13: Line = self_1271;
    let _e16: MultiVector = other_1119;
    let _e21: Line = self_1271;
    let _e24: MultiVector = other_1119;
    return Scalar((((0.0 - (_e5.g1_.x * _e8.g3_.x)) - (_e13.g1_.y * _e16.g3_.y)) - (_e21.g1_.z * _e24.g3_.z)));
}

fn line_multi_vector_dot(self_1272: Line, other_1120: MultiVector) -> Scalar {
    var self_1273: Line;
    var other_1121: MultiVector;

    self_1273 = self_1272;
    other_1121 = other_1120;
    let _e5: Line = self_1273;
    let _e8: MultiVector = other_1121;
    let _e13: Line = self_1273;
    let _e16: MultiVector = other_1121;
    let _e21: Line = self_1273;
    let _e24: MultiVector = other_1121;
    return Scalar((((0.0 - (_e5.g1_.x * _e8.g3_.x)) - (_e13.g1_.y * _e16.g3_.y)) - (_e21.g1_.z * _e24.g3_.z)));
}

fn line_multi_vector_anti_scalar_product(self_1274: Line, other_1122: MultiVector) -> AntiScalar {
    var self_1275: Line;
    var other_1123: MultiVector;

    self_1275 = self_1274;
    other_1123 = other_1122;
    let _e5: Line = self_1275;
    let _e8: MultiVector = other_1123;
    let _e13: Line = self_1275;
    let _e16: MultiVector = other_1123;
    let _e21: Line = self_1275;
    let _e24: MultiVector = other_1123;
    return AntiScalar((((0.0 - (_e5.g0_.x * _e8.g2_.x)) - (_e13.g0_.y * _e16.g2_.y)) - (_e21.g0_.z * _e24.g2_.z)));
}

fn line_multi_vector_anti_dot(self_1276: Line, other_1124: MultiVector) -> AntiScalar {
    var self_1277: Line;
    var other_1125: MultiVector;

    self_1277 = self_1276;
    other_1125 = other_1124;
    let _e5: Line = self_1277;
    let _e8: MultiVector = other_1125;
    let _e13: Line = self_1277;
    let _e16: MultiVector = other_1125;
    let _e21: Line = self_1277;
    let _e24: MultiVector = other_1125;
    return AntiScalar((((0.0 - (_e5.g0_.x * _e8.g2_.x)) - (_e13.g0_.y * _e16.g2_.y)) - (_e21.g0_.z * _e24.g2_.z)));
}

fn line_squared_magnitude(self_1278: Line) -> Scalar {
    var self_1279: Line;

    self_1279 = self_1278;
    let _e2: Line = self_1279;
    let _e3: Line = self_1279;
    let _e4: Line = line_reversal(_e3);
    let _e5: Scalar = line_line_scalar_product(_e2, _e4);
    return _e5;
}

fn line_magnitude(self_1280: Line) -> Scalar {
    var self_1281: Line;

    self_1281 = self_1280;
    let _e2: Line = self_1281;
    let _e3: Scalar = line_squared_magnitude(_e2);
    return Scalar(sqrt(_e3.g0_));
}

fn line_bulk_norm(self_1282: Line) -> Scalar {
    var self_1283: Line;

    self_1283 = self_1282;
    let _e2: Line = self_1283;
    let _e3: Scalar = line_squared_magnitude(_e2);
    return Scalar(sqrt(_e3.g0_));
}

fn line_squared_anti_magnitude(self_1284: Line) -> AntiScalar {
    var self_1285: Line;

    self_1285 = self_1284;
    let _e2: Line = self_1285;
    let _e3: Line = self_1285;
    let _e4: Line = line_anti_reversal(_e3);
    let _e5: AntiScalar = line_line_anti_scalar_product(_e2, _e4);
    return _e5;
}

fn line_weight_norm(self_1286: Line) -> AntiScalar {
    var self_1287: Line;

    self_1287 = self_1286;
    let _e2: Line = self_1287;
    let _e3: AntiScalar = line_squared_anti_magnitude(_e2);
    return AntiScalar(sqrt(_e3.g0_));
}

fn line_geometric_norm(self_1288: Line) -> HomogeneousMagnitude {
    var self_1289: Line;

    self_1289 = self_1288;
    let _e2: Line = self_1289;
    let _e3: Scalar = line_bulk_norm(_e2);
    let _e4: Line = self_1289;
    let _e5: AntiScalar = line_weight_norm(_e4);
    let _e6: HomogeneousMagnitude = scalar_anti_scalar_add(_e3, _e5);
    return _e6;
}

fn line_scale(self_1290: Line, other_1126: f32) -> Line {
    var self_1291: Line;
    var other_1127: f32;

    self_1291 = self_1290;
    other_1127 = other_1126;
    let _e4: Line = self_1291;
    let _e5: f32 = other_1127;
    let _e7: Line = line_scalar_geometric_product(_e4, Scalar(_e5));
    return _e7;
}

fn line_signum(self_1292: Line) -> Line {
    var self_1293: Line;

    self_1293 = self_1292;
    let _e2: Line = self_1293;
    let _e3: Line = self_1293;
    let _e4: Scalar = line_magnitude(_e3);
    let _e9: Line = line_scalar_geometric_product(_e2, Scalar((1.0 / _e4.g0_)));
    return _e9;
}

fn line_inverse(self_1294: Line) -> Line {
    var self_1295: Line;

    self_1295 = self_1294;
    let _e2: Line = self_1295;
    let _e3: Line = line_reversal(_e2);
    let _e4: Line = self_1295;
    let _e5: Scalar = line_squared_magnitude(_e4);
    let _e10: Line = line_scalar_geometric_product(_e3, Scalar((1.0 / _e5.g0_)));
    return _e10;
}

fn line_unitize(self_1296: Line) -> Line {
    var self_1297: Line;

    self_1297 = self_1296;
    let _e2: Line = self_1297;
    let _e3: Line = self_1297;
    let _e4: AntiScalar = line_weight_norm(_e3);
    let _e9: Line = line_scalar_geometric_product(_e2, Scalar((1.0 / _e4.g0_)));
    return _e9;
}

fn line_attitude(self_1298: Line) -> Point {
    var self_1299: Line;

    self_1299 = self_1298;
    let _e2: Line = self_1299;
    let _e9: Point = line_plane_regressive_product(_e2, Plane(vec4<f32>(0.0, 0.0, 0.0, 1.0)));
    return _e9;
}

fn plane_zero() -> Plane {
    return Plane(vec4<f32>(0.0));
}

fn plane_one() -> Plane {
    return Plane(vec4<f32>(0.0));
}

fn plane_grade(self_1300: Plane) -> i32 {
    return 3;
}

fn plane_anti_grade(self_1301: Plane) -> i32 {
    return 1;
}

fn plane_neg(self_1302: Plane) -> Plane {
    var self_1303: Plane;

    self_1303 = self_1302;
    let _e2: Plane = self_1303;
    return Plane((_e2.g0_ * vec4<f32>(-(1.0))));
}

fn plane_automorphism(self_1304: Plane) -> Plane {
    var self_1305: Plane;

    self_1305 = self_1304;
    let _e2: Plane = self_1305;
    return Plane((_e2.g0_ * vec4<f32>(-(1.0))));
}

fn plane_reversal(self_1306: Plane) -> Plane {
    var self_1307: Plane;

    self_1307 = self_1306;
    let _e2: Plane = self_1307;
    return Plane((_e2.g0_ * vec4<f32>(-(1.0))));
}

fn plane_conjugation(self_1308: Plane) -> Plane {
    var self_1309: Plane;

    self_1309 = self_1308;
    let _e2: Plane = self_1309;
    return Plane(_e2.g0_);
}

fn plane_dual(self_1310: Plane) -> Point {
    var self_1311: Plane;

    self_1311 = self_1310;
    let _e2: Plane = self_1311;
    return Point((_e2.g0_ * vec4<f32>(-(1.0))));
}

fn plane_anti_reversal(self_1312: Plane) -> Plane {
    var self_1313: Plane;

    self_1313 = self_1312;
    let _e2: Plane = self_1313;
    return Plane(_e2.g0_);
}

fn plane_right_complement(self_1314: Plane) -> Point {
    var self_1315: Plane;

    self_1315 = self_1314;
    let _e2: Plane = self_1315;
    return Point((_e2.g0_ * vec4<f32>(-(1.0))));
}

fn plane_left_complement(self_1316: Plane) -> Point {
    var self_1317: Plane;

    self_1317 = self_1316;
    let _e2: Plane = self_1317;
    return Point(_e2.g0_);
}

fn plane_double_complement(self_1318: Plane) -> Plane {
    var self_1319: Plane;

    self_1319 = self_1318;
    let _e2: Plane = self_1319;
    return Plane((_e2.g0_ * vec4<f32>(-(1.0))));
}

fn plane_scalar_geometric_product(self_1320: Plane, other_1128: Scalar) -> Plane {
    var self_1321: Plane;
    var other_1129: Scalar;

    self_1321 = self_1320;
    other_1129 = other_1128;
    let _e4: Plane = self_1321;
    let _e6: Scalar = other_1129;
    return Plane((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn plane_scalar_outer_product(self_1322: Plane, other_1130: Scalar) -> Plane {
    var self_1323: Plane;
    var other_1131: Scalar;

    self_1323 = self_1322;
    other_1131 = other_1130;
    let _e4: Plane = self_1323;
    let _e6: Scalar = other_1131;
    return Plane((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn plane_scalar_wedge(self_1324: Plane, other_1132: Scalar) -> Plane {
    var self_1325: Plane;
    var other_1133: Scalar;

    self_1325 = self_1324;
    other_1133 = other_1132;
    let _e4: Plane = self_1325;
    let _e6: Scalar = other_1133;
    return Plane((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn plane_scalar_join(self_1326: Plane, other_1134: Scalar) -> Plane {
    var self_1327: Plane;
    var other_1135: Scalar;

    self_1327 = self_1326;
    other_1135 = other_1134;
    let _e4: Plane = self_1327;
    let _e6: Scalar = other_1135;
    return Plane((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn plane_scalar_inner_product(self_1328: Plane, other_1136: Scalar) -> Plane {
    var self_1329: Plane;
    var other_1137: Scalar;

    self_1329 = self_1328;
    other_1137 = other_1136;
    let _e4: Plane = self_1329;
    let _e6: Scalar = other_1137;
    return Plane((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn plane_scalar_right_contraction(self_1330: Plane, other_1138: Scalar) -> Plane {
    var self_1331: Plane;
    var other_1139: Scalar;

    self_1331 = self_1330;
    other_1139 = other_1138;
    let _e4: Plane = self_1331;
    let _e6: Scalar = other_1139;
    return Plane((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn plane_anti_scalar_regressive_product(self_1332: Plane, other_1140: AntiScalar) -> Plane {
    var self_1333: Plane;
    var other_1141: AntiScalar;

    self_1333 = self_1332;
    other_1141 = other_1140;
    let _e4: Plane = self_1333;
    let _e6: AntiScalar = other_1141;
    return Plane((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn plane_anti_scalar_anti_wedge(self_1334: Plane, other_1142: AntiScalar) -> Plane {
    var self_1335: Plane;
    var other_1143: AntiScalar;

    self_1335 = self_1334;
    other_1143 = other_1142;
    let _e4: Plane = self_1335;
    let _e6: AntiScalar = other_1143;
    return Plane((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn plane_anti_scalar_meet(self_1336: Plane, other_1144: AntiScalar) -> Plane {
    var self_1337: Plane;
    var other_1145: AntiScalar;

    self_1337 = self_1336;
    other_1145 = other_1144;
    let _e4: Plane = self_1337;
    let _e6: AntiScalar = other_1145;
    return Plane((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn plane_anti_scalar_geometric_anti_product(self_1338: Plane, other_1146: AntiScalar) -> Plane {
    var self_1339: Plane;
    var other_1147: AntiScalar;

    self_1339 = self_1338;
    other_1147 = other_1146;
    let _e4: Plane = self_1339;
    let _e6: AntiScalar = other_1147;
    return Plane((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn plane_anti_scalar_inner_anti_product(self_1340: Plane, other_1148: AntiScalar) -> Plane {
    var self_1341: Plane;
    var other_1149: AntiScalar;

    self_1341 = self_1340;
    other_1149 = other_1148;
    let _e4: Plane = self_1341;
    let _e6: AntiScalar = other_1149;
    return Plane((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn plane_anti_scalar_right_anti_contraction(self_1342: Plane, other_1150: AntiScalar) -> Plane {
    var self_1343: Plane;
    var other_1151: AntiScalar;

    self_1343 = self_1342;
    other_1151 = other_1150;
    let _e4: Plane = self_1343;
    let _e6: AntiScalar = other_1151;
    return Plane((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn plane_homogeneous_magnitude_regressive_product(self_1344: Plane, other_1152: HomogeneousMagnitude) -> Plane {
    var self_1345: Plane;
    var other_1153: HomogeneousMagnitude;

    self_1345 = self_1344;
    other_1153 = other_1152;
    let _e4: Plane = self_1345;
    let _e6: HomogeneousMagnitude = other_1153;
    return Plane((_e4.g0_ * vec4<f32>(_e6.g0_.y)));
}

fn plane_homogeneous_magnitude_anti_wedge(self_1346: Plane, other_1154: HomogeneousMagnitude) -> Plane {
    var self_1347: Plane;
    var other_1155: HomogeneousMagnitude;

    self_1347 = self_1346;
    other_1155 = other_1154;
    let _e4: Plane = self_1347;
    let _e6: HomogeneousMagnitude = other_1155;
    return Plane((_e4.g0_ * vec4<f32>(_e6.g0_.y)));
}

fn plane_homogeneous_magnitude_meet(self_1348: Plane, other_1156: HomogeneousMagnitude) -> Plane {
    var self_1349: Plane;
    var other_1157: HomogeneousMagnitude;

    self_1349 = self_1348;
    other_1157 = other_1156;
    let _e4: Plane = self_1349;
    let _e6: HomogeneousMagnitude = other_1157;
    return Plane((_e4.g0_ * vec4<f32>(_e6.g0_.y)));
}

fn plane_homogeneous_magnitude_outer_product(self_1350: Plane, other_1158: HomogeneousMagnitude) -> Plane {
    var self_1351: Plane;
    var other_1159: HomogeneousMagnitude;

    self_1351 = self_1350;
    other_1159 = other_1158;
    let _e4: Plane = self_1351;
    let _e6: HomogeneousMagnitude = other_1159;
    return Plane((_e4.g0_ * vec4<f32>(_e6.g0_.x)));
}

fn plane_homogeneous_magnitude_wedge(self_1352: Plane, other_1160: HomogeneousMagnitude) -> Plane {
    var self_1353: Plane;
    var other_1161: HomogeneousMagnitude;

    self_1353 = self_1352;
    other_1161 = other_1160;
    let _e4: Plane = self_1353;
    let _e6: HomogeneousMagnitude = other_1161;
    return Plane((_e4.g0_ * vec4<f32>(_e6.g0_.x)));
}

fn plane_homogeneous_magnitude_join(self_1354: Plane, other_1162: HomogeneousMagnitude) -> Plane {
    var self_1355: Plane;
    var other_1163: HomogeneousMagnitude;

    self_1355 = self_1354;
    other_1163 = other_1162;
    let _e4: Plane = self_1355;
    let _e6: HomogeneousMagnitude = other_1163;
    return Plane((_e4.g0_ * vec4<f32>(_e6.g0_.x)));
}

fn plane_homogeneous_magnitude_right_contraction(self_1356: Plane, other_1164: HomogeneousMagnitude) -> Plane {
    var self_1357: Plane;
    var other_1165: HomogeneousMagnitude;

    self_1357 = self_1356;
    other_1165 = other_1164;
    let _e4: Plane = self_1357;
    let _e6: HomogeneousMagnitude = other_1165;
    return Plane((_e4.g0_ * vec4<f32>(_e6.g0_.x)));
}

fn plane_homogeneous_magnitude_right_anti_contraction(self_1358: Plane, other_1166: HomogeneousMagnitude) -> Plane {
    var self_1359: Plane;
    var other_1167: HomogeneousMagnitude;

    self_1359 = self_1358;
    other_1167 = other_1166;
    let _e4: Plane = self_1359;
    let _e6: HomogeneousMagnitude = other_1167;
    return Plane((_e4.g0_ * vec4<f32>(_e6.g0_.y)));
}

fn plane_point_add(self_1360: Plane, other_1168: Point) -> Flector {
    var self_1361: Plane;
    var other_1169: Point;

    self_1361 = self_1360;
    other_1169 = other_1168;
    let _e4: Point = other_1169;
    let _e6: Plane = self_1361;
    return Flector(_e4.g0_, _e6.g0_);
}

fn plane_point_sub(self_1362: Plane, other_1170: Point) -> Flector {
    var self_1363: Plane;
    var other_1171: Point;

    self_1363 = self_1362;
    other_1171 = other_1170;
    let _e6: Point = other_1171;
    let _e9: Plane = self_1363;
    return Flector((vec4<f32>(0.0) - _e6.g0_), _e9.g0_);
}

fn plane_point_geometric_product(self_1364: Plane, other_1172: Point) -> Motor {
    var self_1365: Plane;
    var other_1173: Point;

    self_1365 = self_1364;
    other_1173 = other_1172;
    let _e4: Plane = self_1365;
    let _e8: Point = other_1173;
    let _e20: Plane = self_1365;
    let _e24: Point = other_1173;
    let _e37: Plane = self_1365;
    let _e41: Point = other_1173;
    let _e54: Plane = self_1365;
    let _e58: Point = other_1173;
    let _e73: Plane = self_1365;
    let _e77: Point = other_1173;
    let _e80: Point = other_1173;
    let _e83: Point = other_1173;
    return Motor((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.zzxy) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e20.g0_.z) * _e24.g0_.yxyz) * vec4<f32>(1.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e37.g0_.w) * vec4<f32>(_e41.g0_.w)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e54.g0_.x) * _e58.g0_.xzyx) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))), (vec3<f32>(0.0) - (vec3<f32>(_e73.g0_.w) * vec3<f32>(_e77.g0_.x, _e80.g0_.y, _e83.g0_.z))));
}

fn plane_point_regressive_product(self_1366: Plane, other_1174: Point) -> Scalar {
    var self_1367: Plane;
    var other_1175: Point;

    self_1367 = self_1366;
    other_1175 = other_1174;
    let _e5: Plane = self_1367;
    let _e8: Point = other_1175;
    let _e13: Plane = self_1367;
    let _e16: Point = other_1175;
    let _e21: Plane = self_1367;
    let _e24: Point = other_1175;
    let _e29: Plane = self_1367;
    let _e32: Point = other_1175;
    return Scalar(((((0.0 - (_e5.g0_.x * _e8.g0_.x)) - (_e13.g0_.y * _e16.g0_.y)) - (_e21.g0_.z * _e24.g0_.z)) - (_e29.g0_.w * _e32.g0_.w)));
}

fn plane_point_anti_wedge(self_1368: Plane, other_1176: Point) -> Scalar {
    var self_1369: Plane;
    var other_1177: Point;

    self_1369 = self_1368;
    other_1177 = other_1176;
    let _e5: Plane = self_1369;
    let _e8: Point = other_1177;
    let _e13: Plane = self_1369;
    let _e16: Point = other_1177;
    let _e21: Plane = self_1369;
    let _e24: Point = other_1177;
    let _e29: Plane = self_1369;
    let _e32: Point = other_1177;
    return Scalar(((((0.0 - (_e5.g0_.x * _e8.g0_.x)) - (_e13.g0_.y * _e16.g0_.y)) - (_e21.g0_.z * _e24.g0_.z)) - (_e29.g0_.w * _e32.g0_.w)));
}

fn plane_point_meet(self_1370: Plane, other_1178: Point) -> Scalar {
    var self_1371: Plane;
    var other_1179: Point;

    self_1371 = self_1370;
    other_1179 = other_1178;
    let _e5: Plane = self_1371;
    let _e8: Point = other_1179;
    let _e13: Plane = self_1371;
    let _e16: Point = other_1179;
    let _e21: Plane = self_1371;
    let _e24: Point = other_1179;
    let _e29: Plane = self_1371;
    let _e32: Point = other_1179;
    return Scalar(((((0.0 - (_e5.g0_.x * _e8.g0_.x)) - (_e13.g0_.y * _e16.g0_.y)) - (_e21.g0_.z * _e24.g0_.z)) - (_e29.g0_.w * _e32.g0_.w)));
}

fn plane_point_outer_product(self_1372: Plane, other_1180: Point) -> AntiScalar {
    var self_1373: Plane;
    var other_1181: Point;

    self_1373 = self_1372;
    other_1181 = other_1180;
    let _e5: Plane = self_1373;
    let _e8: Point = other_1181;
    let _e13: Plane = self_1373;
    let _e16: Point = other_1181;
    let _e21: Plane = self_1373;
    let _e24: Point = other_1181;
    let _e29: Plane = self_1373;
    let _e32: Point = other_1181;
    return AntiScalar(((((0.0 - (_e5.g0_.x * _e8.g0_.x)) - (_e13.g0_.y * _e16.g0_.y)) - (_e21.g0_.z * _e24.g0_.z)) - (_e29.g0_.w * _e32.g0_.w)));
}

fn plane_point_wedge(self_1374: Plane, other_1182: Point) -> AntiScalar {
    var self_1375: Plane;
    var other_1183: Point;

    self_1375 = self_1374;
    other_1183 = other_1182;
    let _e5: Plane = self_1375;
    let _e8: Point = other_1183;
    let _e13: Plane = self_1375;
    let _e16: Point = other_1183;
    let _e21: Plane = self_1375;
    let _e24: Point = other_1183;
    let _e29: Plane = self_1375;
    let _e32: Point = other_1183;
    return AntiScalar(((((0.0 - (_e5.g0_.x * _e8.g0_.x)) - (_e13.g0_.y * _e16.g0_.y)) - (_e21.g0_.z * _e24.g0_.z)) - (_e29.g0_.w * _e32.g0_.w)));
}

fn plane_point_join(self_1376: Plane, other_1184: Point) -> AntiScalar {
    var self_1377: Plane;
    var other_1185: Point;

    self_1377 = self_1376;
    other_1185 = other_1184;
    let _e5: Plane = self_1377;
    let _e8: Point = other_1185;
    let _e13: Plane = self_1377;
    let _e16: Point = other_1185;
    let _e21: Plane = self_1377;
    let _e24: Point = other_1185;
    let _e29: Plane = self_1377;
    let _e32: Point = other_1185;
    return AntiScalar(((((0.0 - (_e5.g0_.x * _e8.g0_.x)) - (_e13.g0_.y * _e16.g0_.y)) - (_e21.g0_.z * _e24.g0_.z)) - (_e29.g0_.w * _e32.g0_.w)));
}

fn plane_point_inner_product(self_1378: Plane, other_1186: Point) -> Line {
    var self_1379: Plane;
    var other_1187: Point;

    self_1379 = self_1378;
    other_1187 = other_1186;
    let _e4: Plane = self_1379;
    let _e8: Point = other_1187;
    let _e11: Point = other_1187;
    let _e14: Point = other_1187;
    let _e25: Plane = self_1379;
    let _e29: Point = other_1187;
    let _e32: Point = other_1187;
    let _e35: Point = other_1187;
    let _e47: Plane = self_1379;
    let _e51: Point = other_1187;
    let _e54: Point = other_1187;
    let _e57: Point = other_1187;
    let _e71: Plane = self_1379;
    let _e75: Point = other_1187;
    let _e78: Point = other_1187;
    let _e81: Point = other_1187;
    return Line(((((vec3<f32>(_e4.g0_.y) * vec3<f32>(_e8.g0_.z, _e11.g0_.z, _e14.g0_.x)) * vec3<f32>(-(1.0), 0.0, 1.0)) + ((vec3<f32>(_e25.g0_.z) * vec3<f32>(_e29.g0_.y, _e32.g0_.x, _e35.g0_.y)) * vec3<f32>(1.0, -(1.0), 0.0))) + ((vec3<f32>(_e47.g0_.x) * vec3<f32>(_e51.g0_.x, _e54.g0_.z, _e57.g0_.y)) * vec3<f32>(0.0, 1.0, -(1.0)))), (vec3<f32>(0.0) - (vec3<f32>(_e71.g0_.w) * vec3<f32>(_e75.g0_.x, _e78.g0_.y, _e81.g0_.z))));
}

fn plane_point_inner_anti_product(self_1380: Plane, other_1188: Point) -> Line {
    var self_1381: Plane;
    var other_1189: Point;

    self_1381 = self_1380;
    other_1189 = other_1188;
    let _e4: Plane = self_1381;
    let _e7: Plane = self_1381;
    let _e10: Plane = self_1381;
    let _e14: Point = other_1189;
    let _e23: Plane = self_1381;
    let _e27: Point = other_1189;
    let _e30: Point = other_1189;
    let _e33: Point = other_1189;
    let _e44: Plane = self_1381;
    let _e48: Point = other_1189;
    let _e51: Point = other_1189;
    let _e54: Point = other_1189;
    let _e66: Plane = self_1381;
    let _e70: Point = other_1189;
    let _e73: Point = other_1189;
    let _e76: Point = other_1189;
    return Line(((vec3<f32>(_e4.g0_.x, _e7.g0_.y, _e10.g0_.z) * vec3<f32>(_e14.g0_.w)) * vec3<f32>(-(1.0))), ((((vec3<f32>(_e23.g0_.y) * vec3<f32>(_e27.g0_.z, _e30.g0_.z, _e33.g0_.x)) * vec3<f32>(1.0, 0.0, -(1.0))) + ((vec3<f32>(_e44.g0_.z) * vec3<f32>(_e48.g0_.y, _e51.g0_.x, _e54.g0_.y)) * vec3<f32>(-(1.0), 1.0, 0.0))) + ((vec3<f32>(_e66.g0_.x) * vec3<f32>(_e70.g0_.x, _e73.g0_.z, _e76.g0_.y)) * vec3<f32>(0.0, -(1.0), 1.0))));
}

fn plane_point_right_contraction(self_1382: Plane, other_1190: Point) -> Line {
    var self_1383: Plane;
    var other_1191: Point;

    self_1383 = self_1382;
    other_1191 = other_1190;
    let _e4: Plane = self_1383;
    let _e8: Point = other_1191;
    let _e11: Point = other_1191;
    let _e14: Point = other_1191;
    let _e25: Plane = self_1383;
    let _e29: Point = other_1191;
    let _e32: Point = other_1191;
    let _e35: Point = other_1191;
    let _e47: Plane = self_1383;
    let _e51: Point = other_1191;
    let _e54: Point = other_1191;
    let _e57: Point = other_1191;
    let _e71: Plane = self_1383;
    let _e75: Point = other_1191;
    let _e78: Point = other_1191;
    let _e81: Point = other_1191;
    return Line(((((vec3<f32>(_e4.g0_.y) * vec3<f32>(_e8.g0_.z, _e11.g0_.z, _e14.g0_.x)) * vec3<f32>(-(1.0), 0.0, 1.0)) + ((vec3<f32>(_e25.g0_.z) * vec3<f32>(_e29.g0_.y, _e32.g0_.x, _e35.g0_.y)) * vec3<f32>(1.0, -(1.0), 0.0))) + ((vec3<f32>(_e47.g0_.x) * vec3<f32>(_e51.g0_.x, _e54.g0_.z, _e57.g0_.y)) * vec3<f32>(0.0, 1.0, -(1.0)))), (vec3<f32>(0.0) - (vec3<f32>(_e71.g0_.w) * vec3<f32>(_e75.g0_.x, _e78.g0_.y, _e81.g0_.z))));
}

fn plane_point_left_anti_contraction(self_1384: Plane, other_1192: Point) -> Line {
    var self_1385: Plane;
    var other_1193: Point;

    self_1385 = self_1384;
    other_1193 = other_1192;
    let _e4: Plane = self_1385;
    let _e7: Plane = self_1385;
    let _e10: Plane = self_1385;
    let _e14: Point = other_1193;
    let _e23: Plane = self_1385;
    let _e27: Point = other_1193;
    let _e30: Point = other_1193;
    let _e33: Point = other_1193;
    let _e44: Plane = self_1385;
    let _e48: Point = other_1193;
    let _e51: Point = other_1193;
    let _e54: Point = other_1193;
    let _e66: Plane = self_1385;
    let _e70: Point = other_1193;
    let _e73: Point = other_1193;
    let _e76: Point = other_1193;
    return Line(((vec3<f32>(_e4.g0_.x, _e7.g0_.y, _e10.g0_.z) * vec3<f32>(_e14.g0_.w)) * vec3<f32>(-(1.0))), ((((vec3<f32>(_e23.g0_.y) * vec3<f32>(_e27.g0_.z, _e30.g0_.z, _e33.g0_.x)) * vec3<f32>(1.0, 0.0, -(1.0))) + ((vec3<f32>(_e44.g0_.z) * vec3<f32>(_e48.g0_.y, _e51.g0_.x, _e54.g0_.y)) * vec3<f32>(-(1.0), 1.0, 0.0))) + ((vec3<f32>(_e66.g0_.x) * vec3<f32>(_e70.g0_.x, _e73.g0_.z, _e76.g0_.y)) * vec3<f32>(0.0, -(1.0), 1.0))));
}

fn plane_line_regressive_product(self_1386: Plane, other_1194: Line) -> Point {
    var self_1387: Plane;
    var other_1195: Line;

    self_1387 = self_1386;
    other_1195 = other_1194;
    let _e4: Plane = self_1387;
    let _e8: Line = other_1195;
    let _e11: Line = other_1195;
    let _e14: Line = other_1195;
    let _e17: Line = other_1195;
    let _e30: Plane = self_1387;
    let _e34: Line = other_1195;
    let _e37: Line = other_1195;
    let _e40: Line = other_1195;
    let _e43: Line = other_1195;
    let _e57: Plane = self_1387;
    let _e61: Line = other_1195;
    let _e64: Line = other_1195;
    let _e67: Line = other_1195;
    let _e70: Line = other_1195;
    let _e82: Plane = self_1387;
    let _e86: Line = other_1195;
    let _e89: Line = other_1195;
    let _e92: Line = other_1195;
    let _e95: Line = other_1195;
    return Point((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g1_.z, _e11.g1_.z, _e14.g1_.x, _e17.g0_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e30.g0_.z) * vec4<f32>(_e34.g1_.y, _e37.g1_.x, _e40.g1_.y, _e43.g0_.z)) * vec4<f32>(1.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e57.g0_.w) * vec4<f32>(_e61.g0_.x, _e64.g0_.y, _e67.g0_.z, _e70.g0_.x)) * vec4<f32>(1.0, 1.0, 1.0, 0.0))) + ((vec4<f32>(_e82.g0_.x) * vec4<f32>(_e86.g1_.x, _e89.g1_.z, _e92.g1_.y, _e95.g0_.x)) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))));
}

fn plane_line_anti_wedge(self_1388: Plane, other_1196: Line) -> Point {
    var self_1389: Plane;
    var other_1197: Line;

    self_1389 = self_1388;
    other_1197 = other_1196;
    let _e4: Plane = self_1389;
    let _e8: Line = other_1197;
    let _e11: Line = other_1197;
    let _e14: Line = other_1197;
    let _e17: Line = other_1197;
    let _e30: Plane = self_1389;
    let _e34: Line = other_1197;
    let _e37: Line = other_1197;
    let _e40: Line = other_1197;
    let _e43: Line = other_1197;
    let _e57: Plane = self_1389;
    let _e61: Line = other_1197;
    let _e64: Line = other_1197;
    let _e67: Line = other_1197;
    let _e70: Line = other_1197;
    let _e82: Plane = self_1389;
    let _e86: Line = other_1197;
    let _e89: Line = other_1197;
    let _e92: Line = other_1197;
    let _e95: Line = other_1197;
    return Point((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g1_.z, _e11.g1_.z, _e14.g1_.x, _e17.g0_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e30.g0_.z) * vec4<f32>(_e34.g1_.y, _e37.g1_.x, _e40.g1_.y, _e43.g0_.z)) * vec4<f32>(1.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e57.g0_.w) * vec4<f32>(_e61.g0_.x, _e64.g0_.y, _e67.g0_.z, _e70.g0_.x)) * vec4<f32>(1.0, 1.0, 1.0, 0.0))) + ((vec4<f32>(_e82.g0_.x) * vec4<f32>(_e86.g1_.x, _e89.g1_.z, _e92.g1_.y, _e95.g0_.x)) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))));
}

fn plane_line_meet(self_1390: Plane, other_1198: Line) -> Point {
    var self_1391: Plane;
    var other_1199: Line;

    self_1391 = self_1390;
    other_1199 = other_1198;
    let _e4: Plane = self_1391;
    let _e8: Line = other_1199;
    let _e11: Line = other_1199;
    let _e14: Line = other_1199;
    let _e17: Line = other_1199;
    let _e30: Plane = self_1391;
    let _e34: Line = other_1199;
    let _e37: Line = other_1199;
    let _e40: Line = other_1199;
    let _e43: Line = other_1199;
    let _e57: Plane = self_1391;
    let _e61: Line = other_1199;
    let _e64: Line = other_1199;
    let _e67: Line = other_1199;
    let _e70: Line = other_1199;
    let _e82: Plane = self_1391;
    let _e86: Line = other_1199;
    let _e89: Line = other_1199;
    let _e92: Line = other_1199;
    let _e95: Line = other_1199;
    return Point((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g1_.z, _e11.g1_.z, _e14.g1_.x, _e17.g0_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e30.g0_.z) * vec4<f32>(_e34.g1_.y, _e37.g1_.x, _e40.g1_.y, _e43.g0_.z)) * vec4<f32>(1.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e57.g0_.w) * vec4<f32>(_e61.g0_.x, _e64.g0_.y, _e67.g0_.z, _e70.g0_.x)) * vec4<f32>(1.0, 1.0, 1.0, 0.0))) + ((vec4<f32>(_e82.g0_.x) * vec4<f32>(_e86.g1_.x, _e89.g1_.z, _e92.g1_.y, _e95.g0_.x)) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))));
}

fn plane_line_inner_product(self_1392: Plane, other_1200: Line) -> Point {
    var self_1393: Plane;
    var other_1201: Line;

    self_1393 = self_1392;
    other_1201 = other_1200;
    let _e4: Plane = self_1393;
    let _e8: Line = other_1201;
    let _e20: Plane = self_1393;
    let _e24: Line = other_1201;
    let _e37: Plane = self_1393;
    let _e40: Line = other_1201;
    let _e43: Line = other_1201;
    let _e46: Line = other_1201;
    let _e49: Line = other_1201;
    return Point(((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g1_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0))) + ((vec4<f32>(_e20.g0_.z) * vec4<f32>(_e24.g1_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((_e37.g0_.wwwx * vec4<f32>(_e40.g1_.x, _e43.g1_.y, _e46.g1_.z, _e49.g1_.x)) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))));
}

fn plane_line_geometric_anti_product(self_1394: Plane, other_1202: Line) -> Flector {
    var self_1395: Plane;
    var other_1203: Line;

    self_1395 = self_1394;
    other_1203 = other_1202;
    let _e4: Plane = self_1395;
    let _e8: Line = other_1203;
    let _e11: Line = other_1203;
    let _e14: Line = other_1203;
    let _e17: Line = other_1203;
    let _e30: Plane = self_1395;
    let _e34: Line = other_1203;
    let _e37: Line = other_1203;
    let _e40: Line = other_1203;
    let _e43: Line = other_1203;
    let _e57: Plane = self_1395;
    let _e61: Line = other_1203;
    let _e64: Line = other_1203;
    let _e67: Line = other_1203;
    let _e70: Line = other_1203;
    let _e82: Plane = self_1395;
    let _e86: Line = other_1203;
    let _e89: Line = other_1203;
    let _e92: Line = other_1203;
    let _e95: Line = other_1203;
    let _e109: Plane = self_1395;
    let _e113: Line = other_1203;
    let _e116: Line = other_1203;
    let _e119: Line = other_1203;
    let _e122: Line = other_1203;
    let _e134: Plane = self_1395;
    let _e138: Line = other_1203;
    let _e141: Line = other_1203;
    let _e144: Line = other_1203;
    let _e147: Line = other_1203;
    let _e160: Plane = self_1395;
    let _e164: Line = other_1203;
    let _e167: Line = other_1203;
    let _e170: Line = other_1203;
    let _e173: Line = other_1203;
    return Flector((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g1_.z, _e11.g1_.z, _e14.g1_.x, _e17.g0_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e30.g0_.z) * vec4<f32>(_e34.g1_.y, _e37.g1_.x, _e40.g1_.y, _e43.g0_.z)) * vec4<f32>(1.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e57.g0_.w) * vec4<f32>(_e61.g0_.x, _e64.g0_.y, _e67.g0_.z, _e70.g0_.x)) * vec4<f32>(1.0, 1.0, 1.0, 0.0))) + ((vec4<f32>(_e82.g0_.x) * vec4<f32>(_e86.g1_.x, _e89.g1_.z, _e92.g1_.y, _e95.g0_.x)) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))), ((((vec4<f32>(_e109.g0_.y) * vec4<f32>(_e113.g0_.z, _e116.g0_.z, _e119.g0_.x, _e122.g1_.y)) * vec4<f32>(1.0, 0.0, -(1.0), 1.0)) + ((vec4<f32>(_e134.g0_.z) * vec4<f32>(_e138.g0_.y, _e141.g0_.x, _e144.g0_.y, _e147.g1_.z)) * vec4<f32>(-(1.0), 1.0, 0.0, 1.0))) + ((vec4<f32>(_e160.g0_.x) * vec4<f32>(_e164.g0_.x, _e167.g0_.z, _e170.g0_.y, _e173.g1_.x)) * vec4<f32>(0.0, -(1.0), 1.0, 1.0))));
}

fn plane_line_inner_anti_product(self_1396: Plane, other_1204: Line) -> Plane {
    var self_1397: Plane;
    var other_1205: Line;

    self_1397 = self_1396;
    other_1205 = other_1204;
    let _e4: Plane = self_1397;
    let _e8: Line = other_1205;
    let _e11: Line = other_1205;
    let _e14: Line = other_1205;
    let _e17: Line = other_1205;
    let _e29: Plane = self_1397;
    let _e33: Line = other_1205;
    let _e36: Line = other_1205;
    let _e39: Line = other_1205;
    let _e42: Line = other_1205;
    let _e55: Plane = self_1397;
    let _e59: Line = other_1205;
    let _e62: Line = other_1205;
    let _e65: Line = other_1205;
    let _e68: Line = other_1205;
    return Plane(((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.z, _e11.g0_.z, _e14.g0_.x, _e17.g1_.y)) * vec4<f32>(1.0, 0.0, -(1.0), 1.0)) + ((vec4<f32>(_e29.g0_.z) * vec4<f32>(_e33.g0_.y, _e36.g0_.x, _e39.g0_.y, _e42.g1_.z)) * vec4<f32>(-(1.0), 1.0, 0.0, 1.0))) + ((vec4<f32>(_e55.g0_.x) * vec4<f32>(_e59.g0_.x, _e62.g0_.z, _e65.g0_.y, _e68.g1_.x)) * vec4<f32>(0.0, -(1.0), 1.0, 1.0))));
}

fn plane_line_right_contraction(self_1398: Plane, other_1206: Line) -> Point {
    var self_1399: Plane;
    var other_1207: Line;

    self_1399 = self_1398;
    other_1207 = other_1206;
    let _e4: Plane = self_1399;
    let _e8: Line = other_1207;
    let _e20: Plane = self_1399;
    let _e24: Line = other_1207;
    let _e37: Plane = self_1399;
    let _e40: Line = other_1207;
    let _e43: Line = other_1207;
    let _e46: Line = other_1207;
    let _e49: Line = other_1207;
    return Point(((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g1_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0))) + ((vec4<f32>(_e20.g0_.z) * vec4<f32>(_e24.g1_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((_e37.g0_.wwwx * vec4<f32>(_e40.g1_.x, _e43.g1_.y, _e46.g1_.z, _e49.g1_.x)) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))));
}

fn plane_line_left_anti_contraction(self_1400: Plane, other_1208: Line) -> Plane {
    var self_1401: Plane;
    var other_1209: Line;

    self_1401 = self_1400;
    other_1209 = other_1208;
    let _e4: Plane = self_1401;
    let _e8: Line = other_1209;
    let _e11: Line = other_1209;
    let _e14: Line = other_1209;
    let _e17: Line = other_1209;
    let _e29: Plane = self_1401;
    let _e33: Line = other_1209;
    let _e36: Line = other_1209;
    let _e39: Line = other_1209;
    let _e42: Line = other_1209;
    let _e55: Plane = self_1401;
    let _e59: Line = other_1209;
    let _e62: Line = other_1209;
    let _e65: Line = other_1209;
    let _e68: Line = other_1209;
    return Plane(((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.z, _e11.g0_.z, _e14.g0_.x, _e17.g1_.y)) * vec4<f32>(1.0, 0.0, -(1.0), 1.0)) + ((vec4<f32>(_e29.g0_.z) * vec4<f32>(_e33.g0_.y, _e36.g0_.x, _e39.g0_.y, _e42.g1_.z)) * vec4<f32>(-(1.0), 1.0, 0.0, 1.0))) + ((vec4<f32>(_e55.g0_.x) * vec4<f32>(_e59.g0_.x, _e62.g0_.z, _e65.g0_.y, _e68.g1_.x)) * vec4<f32>(0.0, -(1.0), 1.0, 1.0))));
}

fn plane_plane_add(self_1402: Plane, other_1210: Plane) -> Plane {
    var self_1403: Plane;
    var other_1211: Plane;

    self_1403 = self_1402;
    other_1211 = other_1210;
    let _e4: Plane = self_1403;
    let _e6: Plane = other_1211;
    return Plane((_e4.g0_ + _e6.g0_));
}

fn plane_plane_sub(self_1404: Plane, other_1212: Plane) -> Plane {
    var self_1405: Plane;
    var other_1213: Plane;

    self_1405 = self_1404;
    other_1213 = other_1212;
    let _e4: Plane = self_1405;
    let _e6: Plane = other_1213;
    return Plane((_e4.g0_ - _e6.g0_));
}

fn plane_plane_mul(self_1406: Plane, other_1214: Plane) -> Plane {
    var self_1407: Plane;
    var other_1215: Plane;

    self_1407 = self_1406;
    other_1215 = other_1214;
    let _e4: Plane = self_1407;
    let _e6: Plane = other_1215;
    return Plane((_e4.g0_ * _e6.g0_));
}

fn plane_plane_div(self_1408: Plane, other_1216: Plane) -> Plane {
    var self_1409: Plane;
    var other_1217: Plane;

    self_1409 = self_1408;
    other_1217 = other_1216;
    let _e4: Plane = self_1409;
    let _e7: Plane = self_1409;
    let _e10: Plane = self_1409;
    let _e13: Plane = self_1409;
    let _e23: Plane = other_1217;
    let _e26: Plane = other_1217;
    let _e29: Plane = other_1217;
    let _e32: Plane = other_1217;
    return Plane((((vec4<f32>(_e4.g0_.x, _e7.g0_.y, _e10.g0_.z, _e13.g0_.w) * vec4<f32>(1.0, 1.0, 1.0, 1.0)) / vec4<f32>(_e23.g0_.x, _e26.g0_.y, _e29.g0_.z, _e32.g0_.w)) * vec4<f32>(1.0, 1.0, 1.0, 1.0)));
}

fn plane_plane_regressive_product(self_1410: Plane, other_1218: Plane) -> Line {
    var self_1411: Plane;
    var other_1219: Plane;

    self_1411 = self_1410;
    other_1219 = other_1218;
    let _e4: Plane = self_1411;
    let _e8: Plane = other_1219;
    let _e11: Plane = other_1219;
    let _e14: Plane = other_1219;
    let _e25: Plane = self_1411;
    let _e29: Plane = other_1219;
    let _e32: Plane = other_1219;
    let _e35: Plane = other_1219;
    let _e47: Plane = self_1411;
    let _e51: Plane = other_1219;
    let _e54: Plane = other_1219;
    let _e57: Plane = other_1219;
    let _e71: Plane = self_1411;
    let _e75: Plane = other_1219;
    let _e78: Plane = other_1219;
    let _e81: Plane = other_1219;
    let _e87: Plane = self_1411;
    let _e90: Plane = self_1411;
    let _e93: Plane = self_1411;
    let _e97: Plane = other_1219;
    return Line(((((vec3<f32>(_e4.g0_.y) * vec3<f32>(_e8.g0_.z, _e11.g0_.z, _e14.g0_.x)) * vec3<f32>(-(1.0), 0.0, 1.0)) + ((vec3<f32>(_e25.g0_.z) * vec3<f32>(_e29.g0_.y, _e32.g0_.x, _e35.g0_.y)) * vec3<f32>(1.0, -(1.0), 0.0))) + ((vec3<f32>(_e47.g0_.x) * vec3<f32>(_e51.g0_.x, _e54.g0_.z, _e57.g0_.y)) * vec3<f32>(0.0, 1.0, -(1.0)))), ((vec3<f32>(0.0) - (vec3<f32>(_e71.g0_.w) * vec3<f32>(_e75.g0_.x, _e78.g0_.y, _e81.g0_.z))) + (vec3<f32>(_e87.g0_.x, _e90.g0_.y, _e93.g0_.z) * vec3<f32>(_e97.g0_.w))));
}

fn plane_plane_anti_wedge(self_1412: Plane, other_1220: Plane) -> Line {
    var self_1413: Plane;
    var other_1221: Plane;

    self_1413 = self_1412;
    other_1221 = other_1220;
    let _e4: Plane = self_1413;
    let _e8: Plane = other_1221;
    let _e11: Plane = other_1221;
    let _e14: Plane = other_1221;
    let _e25: Plane = self_1413;
    let _e29: Plane = other_1221;
    let _e32: Plane = other_1221;
    let _e35: Plane = other_1221;
    let _e47: Plane = self_1413;
    let _e51: Plane = other_1221;
    let _e54: Plane = other_1221;
    let _e57: Plane = other_1221;
    let _e71: Plane = self_1413;
    let _e75: Plane = other_1221;
    let _e78: Plane = other_1221;
    let _e81: Plane = other_1221;
    let _e87: Plane = self_1413;
    let _e90: Plane = self_1413;
    let _e93: Plane = self_1413;
    let _e97: Plane = other_1221;
    return Line(((((vec3<f32>(_e4.g0_.y) * vec3<f32>(_e8.g0_.z, _e11.g0_.z, _e14.g0_.x)) * vec3<f32>(-(1.0), 0.0, 1.0)) + ((vec3<f32>(_e25.g0_.z) * vec3<f32>(_e29.g0_.y, _e32.g0_.x, _e35.g0_.y)) * vec3<f32>(1.0, -(1.0), 0.0))) + ((vec3<f32>(_e47.g0_.x) * vec3<f32>(_e51.g0_.x, _e54.g0_.z, _e57.g0_.y)) * vec3<f32>(0.0, 1.0, -(1.0)))), ((vec3<f32>(0.0) - (vec3<f32>(_e71.g0_.w) * vec3<f32>(_e75.g0_.x, _e78.g0_.y, _e81.g0_.z))) + (vec3<f32>(_e87.g0_.x, _e90.g0_.y, _e93.g0_.z) * vec3<f32>(_e97.g0_.w))));
}

fn plane_plane_meet(self_1414: Plane, other_1222: Plane) -> Line {
    var self_1415: Plane;
    var other_1223: Plane;

    self_1415 = self_1414;
    other_1223 = other_1222;
    let _e4: Plane = self_1415;
    let _e8: Plane = other_1223;
    let _e11: Plane = other_1223;
    let _e14: Plane = other_1223;
    let _e25: Plane = self_1415;
    let _e29: Plane = other_1223;
    let _e32: Plane = other_1223;
    let _e35: Plane = other_1223;
    let _e47: Plane = self_1415;
    let _e51: Plane = other_1223;
    let _e54: Plane = other_1223;
    let _e57: Plane = other_1223;
    let _e71: Plane = self_1415;
    let _e75: Plane = other_1223;
    let _e78: Plane = other_1223;
    let _e81: Plane = other_1223;
    let _e87: Plane = self_1415;
    let _e90: Plane = self_1415;
    let _e93: Plane = self_1415;
    let _e97: Plane = other_1223;
    return Line(((((vec3<f32>(_e4.g0_.y) * vec3<f32>(_e8.g0_.z, _e11.g0_.z, _e14.g0_.x)) * vec3<f32>(-(1.0), 0.0, 1.0)) + ((vec3<f32>(_e25.g0_.z) * vec3<f32>(_e29.g0_.y, _e32.g0_.x, _e35.g0_.y)) * vec3<f32>(1.0, -(1.0), 0.0))) + ((vec3<f32>(_e47.g0_.x) * vec3<f32>(_e51.g0_.x, _e54.g0_.z, _e57.g0_.y)) * vec3<f32>(0.0, 1.0, -(1.0)))), ((vec3<f32>(0.0) - (vec3<f32>(_e71.g0_.w) * vec3<f32>(_e75.g0_.x, _e78.g0_.y, _e81.g0_.z))) + (vec3<f32>(_e87.g0_.x, _e90.g0_.y, _e93.g0_.z) * vec3<f32>(_e97.g0_.w))));
}

fn plane_plane_inner_product(self_1416: Plane, other_1224: Plane) -> Scalar {
    var self_1417: Plane;
    var other_1225: Plane;

    self_1417 = self_1416;
    other_1225 = other_1224;
    let _e5: Plane = self_1417;
    let _e8: Plane = other_1225;
    return Scalar((0.0 - (_e5.g0_.w * _e8.g0_.w)));
}

fn plane_plane_geometric_anti_product(self_1418: Plane, other_1226: Plane) -> Motor {
    var self_1419: Plane;
    var other_1227: Plane;

    self_1419 = self_1418;
    other_1227 = other_1226;
    let _e4: Plane = self_1419;
    let _e8: Plane = other_1227;
    let _e19: Plane = self_1419;
    let _e23: Plane = other_1227;
    let _e35: Plane = self_1419;
    let _e39: Plane = other_1227;
    let _e53: Plane = self_1419;
    let _e57: Plane = other_1227;
    let _e60: Plane = other_1227;
    let _e63: Plane = other_1227;
    let _e69: Plane = self_1419;
    let _e72: Plane = self_1419;
    let _e75: Plane = self_1419;
    let _e79: Plane = other_1227;
    return Motor(((((vec4<f32>(_e4.g0_.y) * _e8.g0_.zzxy) * vec4<f32>(-(1.0), 0.0, 1.0, 1.0)) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.yxyz) * vec4<f32>(1.0, -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e35.g0_.x) * _e39.g0_.xzyx) * vec4<f32>(0.0, 1.0, -(1.0), 1.0))), ((vec3<f32>(0.0) - (vec3<f32>(_e53.g0_.w) * vec3<f32>(_e57.g0_.x, _e60.g0_.y, _e63.g0_.z))) + (vec3<f32>(_e69.g0_.x, _e72.g0_.y, _e75.g0_.z) * vec3<f32>(_e79.g0_.w))));
}

fn plane_plane_inner_anti_product(self_1420: Plane, other_1228: Plane) -> AntiScalar {
    var self_1421: Plane;
    var other_1229: Plane;

    self_1421 = self_1420;
    other_1229 = other_1228;
    let _e4: Plane = self_1421;
    let _e7: Plane = other_1229;
    let _e11: Plane = self_1421;
    let _e14: Plane = other_1229;
    let _e19: Plane = self_1421;
    let _e22: Plane = other_1229;
    return AntiScalar((((_e4.g0_.x * _e7.g0_.x) + (_e11.g0_.y * _e14.g0_.y)) + (_e19.g0_.z * _e22.g0_.z)));
}

fn plane_plane_left_contraction(self_1422: Plane, other_1230: Plane) -> Scalar {
    var self_1423: Plane;
    var other_1231: Plane;

    self_1423 = self_1422;
    other_1231 = other_1230;
    let _e5: Plane = self_1423;
    let _e8: Plane = other_1231;
    return Scalar((0.0 - (_e5.g0_.w * _e8.g0_.w)));
}

fn plane_plane_right_contraction(self_1424: Plane, other_1232: Plane) -> Scalar {
    var self_1425: Plane;
    var other_1233: Plane;

    self_1425 = self_1424;
    other_1233 = other_1232;
    let _e5: Plane = self_1425;
    let _e8: Plane = other_1233;
    return Scalar((0.0 - (_e5.g0_.w * _e8.g0_.w)));
}

fn plane_plane_left_anti_contraction(self_1426: Plane, other_1234: Plane) -> AntiScalar {
    var self_1427: Plane;
    var other_1235: Plane;

    self_1427 = self_1426;
    other_1235 = other_1234;
    let _e4: Plane = self_1427;
    let _e7: Plane = other_1235;
    let _e11: Plane = self_1427;
    let _e14: Plane = other_1235;
    let _e19: Plane = self_1427;
    let _e22: Plane = other_1235;
    return AntiScalar((((_e4.g0_.x * _e7.g0_.x) + (_e11.g0_.y * _e14.g0_.y)) + (_e19.g0_.z * _e22.g0_.z)));
}

fn plane_plane_right_anti_contraction(self_1428: Plane, other_1236: Plane) -> AntiScalar {
    var self_1429: Plane;
    var other_1237: Plane;

    self_1429 = self_1428;
    other_1237 = other_1236;
    let _e4: Plane = self_1429;
    let _e7: Plane = other_1237;
    let _e11: Plane = self_1429;
    let _e14: Plane = other_1237;
    let _e19: Plane = self_1429;
    let _e22: Plane = other_1237;
    return AntiScalar((((_e4.g0_.x * _e7.g0_.x) + (_e11.g0_.y * _e14.g0_.y)) + (_e19.g0_.z * _e22.g0_.z)));
}

fn plane_plane_scalar_product(self_1430: Plane, other_1238: Plane) -> Scalar {
    var self_1431: Plane;
    var other_1239: Plane;

    self_1431 = self_1430;
    other_1239 = other_1238;
    let _e5: Plane = self_1431;
    let _e8: Plane = other_1239;
    return Scalar((0.0 - (_e5.g0_.w * _e8.g0_.w)));
}

fn plane_plane_dot(self_1432: Plane, other_1240: Plane) -> Scalar {
    var self_1433: Plane;
    var other_1241: Plane;

    self_1433 = self_1432;
    other_1241 = other_1240;
    let _e5: Plane = self_1433;
    let _e8: Plane = other_1241;
    return Scalar((0.0 - (_e5.g0_.w * _e8.g0_.w)));
}

fn plane_plane_anti_scalar_product(self_1434: Plane, other_1242: Plane) -> AntiScalar {
    var self_1435: Plane;
    var other_1243: Plane;

    self_1435 = self_1434;
    other_1243 = other_1242;
    let _e4: Plane = self_1435;
    let _e7: Plane = other_1243;
    let _e11: Plane = self_1435;
    let _e14: Plane = other_1243;
    let _e19: Plane = self_1435;
    let _e22: Plane = other_1243;
    return AntiScalar((((_e4.g0_.x * _e7.g0_.x) + (_e11.g0_.y * _e14.g0_.y)) + (_e19.g0_.z * _e22.g0_.z)));
}

fn plane_plane_anti_dot(self_1436: Plane, other_1244: Plane) -> AntiScalar {
    var self_1437: Plane;
    var other_1245: Plane;

    self_1437 = self_1436;
    other_1245 = other_1244;
    let _e4: Plane = self_1437;
    let _e7: Plane = other_1245;
    let _e11: Plane = self_1437;
    let _e14: Plane = other_1245;
    let _e19: Plane = self_1437;
    let _e22: Plane = other_1245;
    return AntiScalar((((_e4.g0_.x * _e7.g0_.x) + (_e11.g0_.y * _e14.g0_.y)) + (_e19.g0_.z * _e22.g0_.z)));
}

fn plane_motor_regressive_product(self_1438: Plane, other_1246: Motor) -> Flector {
    var self_1439: Plane;
    var other_1247: Motor;

    self_1439 = self_1438;
    other_1247 = other_1246;
    let _e4: Plane = self_1439;
    let _e8: Motor = other_1247;
    let _e11: Motor = other_1247;
    let _e14: Motor = other_1247;
    let _e17: Motor = other_1247;
    let _e30: Plane = self_1439;
    let _e34: Motor = other_1247;
    let _e37: Motor = other_1247;
    let _e40: Motor = other_1247;
    let _e43: Motor = other_1247;
    let _e57: Plane = self_1439;
    let _e61: Motor = other_1247;
    let _e72: Plane = self_1439;
    let _e76: Motor = other_1247;
    let _e79: Motor = other_1247;
    let _e82: Motor = other_1247;
    let _e85: Motor = other_1247;
    let _e99: Plane = self_1439;
    let _e101: Motor = other_1247;
    return Flector((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g1_.z, _e11.g1_.z, _e14.g1_.x, _e17.g0_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e30.g0_.z) * vec4<f32>(_e34.g1_.y, _e37.g1_.x, _e40.g1_.y, _e43.g0_.z)) * vec4<f32>(1.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e57.g0_.w) * _e61.g0_.xyzx) * vec4<f32>(1.0, 1.0, 1.0, 0.0))) + ((vec4<f32>(_e72.g0_.x) * vec4<f32>(_e76.g1_.x, _e79.g1_.z, _e82.g1_.y, _e85.g0_.x)) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))), (_e99.g0_ * vec4<f32>(_e101.g0_.w)));
}

fn plane_motor_anti_wedge(self_1440: Plane, other_1248: Motor) -> Flector {
    var self_1441: Plane;
    var other_1249: Motor;

    self_1441 = self_1440;
    other_1249 = other_1248;
    let _e4: Plane = self_1441;
    let _e8: Motor = other_1249;
    let _e11: Motor = other_1249;
    let _e14: Motor = other_1249;
    let _e17: Motor = other_1249;
    let _e30: Plane = self_1441;
    let _e34: Motor = other_1249;
    let _e37: Motor = other_1249;
    let _e40: Motor = other_1249;
    let _e43: Motor = other_1249;
    let _e57: Plane = self_1441;
    let _e61: Motor = other_1249;
    let _e72: Plane = self_1441;
    let _e76: Motor = other_1249;
    let _e79: Motor = other_1249;
    let _e82: Motor = other_1249;
    let _e85: Motor = other_1249;
    let _e99: Plane = self_1441;
    let _e101: Motor = other_1249;
    return Flector((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g1_.z, _e11.g1_.z, _e14.g1_.x, _e17.g0_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e30.g0_.z) * vec4<f32>(_e34.g1_.y, _e37.g1_.x, _e40.g1_.y, _e43.g0_.z)) * vec4<f32>(1.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e57.g0_.w) * _e61.g0_.xyzx) * vec4<f32>(1.0, 1.0, 1.0, 0.0))) + ((vec4<f32>(_e72.g0_.x) * vec4<f32>(_e76.g1_.x, _e79.g1_.z, _e82.g1_.y, _e85.g0_.x)) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))), (_e99.g0_ * vec4<f32>(_e101.g0_.w)));
}

fn plane_motor_meet(self_1442: Plane, other_1250: Motor) -> Flector {
    var self_1443: Plane;
    var other_1251: Motor;

    self_1443 = self_1442;
    other_1251 = other_1250;
    let _e4: Plane = self_1443;
    let _e8: Motor = other_1251;
    let _e11: Motor = other_1251;
    let _e14: Motor = other_1251;
    let _e17: Motor = other_1251;
    let _e30: Plane = self_1443;
    let _e34: Motor = other_1251;
    let _e37: Motor = other_1251;
    let _e40: Motor = other_1251;
    let _e43: Motor = other_1251;
    let _e57: Plane = self_1443;
    let _e61: Motor = other_1251;
    let _e72: Plane = self_1443;
    let _e76: Motor = other_1251;
    let _e79: Motor = other_1251;
    let _e82: Motor = other_1251;
    let _e85: Motor = other_1251;
    let _e99: Plane = self_1443;
    let _e101: Motor = other_1251;
    return Flector((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g1_.z, _e11.g1_.z, _e14.g1_.x, _e17.g0_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e30.g0_.z) * vec4<f32>(_e34.g1_.y, _e37.g1_.x, _e40.g1_.y, _e43.g0_.z)) * vec4<f32>(1.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e57.g0_.w) * _e61.g0_.xyzx) * vec4<f32>(1.0, 1.0, 1.0, 0.0))) + ((vec4<f32>(_e72.g0_.x) * vec4<f32>(_e76.g1_.x, _e79.g1_.z, _e82.g1_.y, _e85.g0_.x)) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))), (_e99.g0_ * vec4<f32>(_e101.g0_.w)));
}

fn plane_motor_inner_product(self_1444: Plane, other_1252: Motor) -> Point {
    var self_1445: Plane;
    var other_1253: Motor;

    self_1445 = self_1444;
    other_1253 = other_1252;
    let _e4: Plane = self_1445;
    let _e8: Motor = other_1253;
    let _e20: Plane = self_1445;
    let _e24: Motor = other_1253;
    let _e37: Plane = self_1445;
    let _e41: Motor = other_1253;
    let _e44: Motor = other_1253;
    let _e47: Motor = other_1253;
    let _e50: Motor = other_1253;
    let _e56: Plane = self_1445;
    let _e60: Motor = other_1253;
    return Point((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g1_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0))) + ((vec4<f32>(_e20.g0_.z) * vec4<f32>(_e24.g1_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + (vec4<f32>(_e37.g0_.w) * vec4<f32>(_e41.g1_.x, _e44.g1_.y, _e47.g1_.z, _e50.g0_.w))) + ((vec4<f32>(_e56.g0_.x) * vec4<f32>(_e60.g1_.x)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))));
}

fn plane_motor_geometric_anti_product(self_1446: Plane, other_1254: Motor) -> Flector {
    var self_1447: Plane;
    var other_1255: Motor;

    self_1447 = self_1446;
    other_1255 = other_1254;
    let _e4: Plane = self_1447;
    let _e8: Motor = other_1255;
    let _e11: Motor = other_1255;
    let _e14: Motor = other_1255;
    let _e17: Motor = other_1255;
    let _e30: Plane = self_1447;
    let _e34: Motor = other_1255;
    let _e37: Motor = other_1255;
    let _e40: Motor = other_1255;
    let _e43: Motor = other_1255;
    let _e57: Plane = self_1447;
    let _e61: Motor = other_1255;
    let _e72: Plane = self_1447;
    let _e76: Motor = other_1255;
    let _e79: Motor = other_1255;
    let _e82: Motor = other_1255;
    let _e85: Motor = other_1255;
    let _e99: Plane = self_1447;
    let _e103: Motor = other_1255;
    let _e106: Motor = other_1255;
    let _e109: Motor = other_1255;
    let _e112: Motor = other_1255;
    let _e124: Plane = self_1447;
    let _e128: Motor = other_1255;
    let _e131: Motor = other_1255;
    let _e134: Motor = other_1255;
    let _e137: Motor = other_1255;
    let _e150: Plane = self_1447;
    let _e154: Motor = other_1255;
    let _e157: Motor = other_1255;
    let _e160: Motor = other_1255;
    let _e163: Motor = other_1255;
    let _e176: Plane = self_1447;
    let _e179: Motor = other_1255;
    return Flector((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g1_.z, _e11.g1_.z, _e14.g1_.x, _e17.g0_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e30.g0_.z) * vec4<f32>(_e34.g1_.y, _e37.g1_.x, _e40.g1_.y, _e43.g0_.z)) * vec4<f32>(1.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e57.g0_.w) * _e61.g0_.xyzx) * vec4<f32>(1.0, 1.0, 1.0, 0.0))) + ((vec4<f32>(_e72.g0_.x) * vec4<f32>(_e76.g1_.x, _e79.g1_.z, _e82.g1_.y, _e85.g0_.x)) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))), (((((vec4<f32>(_e99.g0_.x) * vec4<f32>(_e103.g0_.w, _e106.g0_.z, _e109.g0_.y, _e112.g1_.x)) * vec4<f32>(1.0, -(1.0), 1.0, 1.0)) + ((vec4<f32>(_e124.g0_.y) * vec4<f32>(_e128.g0_.z, _e131.g0_.w, _e134.g0_.x, _e137.g1_.y)) * vec4<f32>(1.0, 1.0, -(1.0), 1.0))) + ((vec4<f32>(_e150.g0_.z) * vec4<f32>(_e154.g0_.y, _e157.g0_.x, _e160.g0_.w, _e163.g1_.z)) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0))) + ((_e176.g0_.xxxw * _e179.g0_.xxxw) * vec4<f32>(0.0, 0.0, 0.0, 1.0))));
}

fn plane_motor_inner_anti_product(self_1448: Plane, other_1256: Motor) -> Plane {
    var self_1449: Plane;
    var other_1257: Motor;

    self_1449 = self_1448;
    other_1257 = other_1256;
    let _e4: Plane = self_1449;
    let _e8: Motor = other_1257;
    let _e11: Motor = other_1257;
    let _e14: Motor = other_1257;
    let _e17: Motor = other_1257;
    let _e29: Plane = self_1449;
    let _e33: Motor = other_1257;
    let _e36: Motor = other_1257;
    let _e39: Motor = other_1257;
    let _e42: Motor = other_1257;
    let _e55: Plane = self_1449;
    let _e59: Motor = other_1257;
    let _e62: Motor = other_1257;
    let _e65: Motor = other_1257;
    let _e68: Motor = other_1257;
    let _e81: Plane = self_1449;
    let _e84: Motor = other_1257;
    return Plane((((((vec4<f32>(_e4.g0_.x) * vec4<f32>(_e8.g0_.w, _e11.g0_.z, _e14.g0_.y, _e17.g1_.x)) * vec4<f32>(1.0, -(1.0), 1.0, 1.0)) + ((vec4<f32>(_e29.g0_.y) * vec4<f32>(_e33.g0_.z, _e36.g0_.w, _e39.g0_.x, _e42.g1_.y)) * vec4<f32>(1.0, 1.0, -(1.0), 1.0))) + ((vec4<f32>(_e55.g0_.z) * vec4<f32>(_e59.g0_.y, _e62.g0_.x, _e65.g0_.w, _e68.g1_.z)) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0))) + ((_e81.g0_.xxxw * _e84.g0_.xxxw) * vec4<f32>(0.0, 0.0, 0.0, 1.0))));
}

fn plane_motor_right_contraction(self_1450: Plane, other_1258: Motor) -> Point {
    var self_1451: Plane;
    var other_1259: Motor;

    self_1451 = self_1450;
    other_1259 = other_1258;
    let _e4: Plane = self_1451;
    let _e8: Motor = other_1259;
    let _e20: Plane = self_1451;
    let _e24: Motor = other_1259;
    let _e37: Plane = self_1451;
    let _e40: Motor = other_1259;
    let _e43: Motor = other_1259;
    let _e46: Motor = other_1259;
    let _e49: Motor = other_1259;
    return Point(((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g1_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0))) + ((vec4<f32>(_e20.g0_.z) * vec4<f32>(_e24.g1_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((_e37.g0_.wwwx * vec4<f32>(_e40.g1_.x, _e43.g1_.y, _e46.g1_.z, _e49.g1_.x)) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))));
}

fn plane_motor_left_anti_contraction(self_1452: Plane, other_1260: Motor) -> Plane {
    var self_1453: Plane;
    var other_1261: Motor;

    self_1453 = self_1452;
    other_1261 = other_1260;
    let _e4: Plane = self_1453;
    let _e8: Motor = other_1261;
    let _e11: Motor = other_1261;
    let _e14: Motor = other_1261;
    let _e17: Motor = other_1261;
    let _e29: Plane = self_1453;
    let _e33: Motor = other_1261;
    let _e36: Motor = other_1261;
    let _e39: Motor = other_1261;
    let _e42: Motor = other_1261;
    let _e55: Plane = self_1453;
    let _e59: Motor = other_1261;
    let _e62: Motor = other_1261;
    let _e65: Motor = other_1261;
    let _e68: Motor = other_1261;
    return Plane(((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.z, _e11.g0_.z, _e14.g0_.x, _e17.g1_.y)) * vec4<f32>(1.0, 0.0, -(1.0), 1.0)) + ((vec4<f32>(_e29.g0_.z) * vec4<f32>(_e33.g0_.y, _e36.g0_.x, _e39.g0_.y, _e42.g1_.z)) * vec4<f32>(-(1.0), 1.0, 0.0, 1.0))) + ((vec4<f32>(_e55.g0_.x) * vec4<f32>(_e59.g0_.x, _e62.g0_.z, _e65.g0_.y, _e68.g1_.x)) * vec4<f32>(0.0, -(1.0), 1.0, 1.0))));
}

fn plane_motor_right_anti_contraction(self_1454: Plane, other_1262: Motor) -> Plane {
    var self_1455: Plane;
    var other_1263: Motor;

    self_1455 = self_1454;
    other_1263 = other_1262;
    let _e4: Plane = self_1455;
    let _e6: Motor = other_1263;
    return Plane((_e4.g0_ * vec4<f32>(_e6.g0_.w)));
}

fn plane_rotor_regressive_product(self_1456: Plane, other_1264: Rotor) -> Flector {
    var self_1457: Plane;
    var other_1265: Rotor;

    self_1457 = self_1456;
    other_1265 = other_1264;
    let _e4: Plane = self_1457;
    let _e8: Rotor = other_1265;
    let _e20: Plane = self_1457;
    let _e24: Rotor = other_1265;
    let _e37: Plane = self_1457;
    let _e40: Rotor = other_1265;
    let _e52: Plane = self_1457;
    let _e54: Rotor = other_1265;
    return Flector(((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0))) + ((vec4<f32>(_e20.g0_.z) * vec4<f32>(_e24.g0_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((_e37.g0_.wwwx * _e40.g0_.xyzx) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))), (_e52.g0_ * vec4<f32>(_e54.g0_.w)));
}

fn plane_rotor_anti_wedge(self_1458: Plane, other_1266: Rotor) -> Flector {
    var self_1459: Plane;
    var other_1267: Rotor;

    self_1459 = self_1458;
    other_1267 = other_1266;
    let _e4: Plane = self_1459;
    let _e8: Rotor = other_1267;
    let _e20: Plane = self_1459;
    let _e24: Rotor = other_1267;
    let _e37: Plane = self_1459;
    let _e40: Rotor = other_1267;
    let _e52: Plane = self_1459;
    let _e54: Rotor = other_1267;
    return Flector(((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0))) + ((vec4<f32>(_e20.g0_.z) * vec4<f32>(_e24.g0_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((_e37.g0_.wwwx * _e40.g0_.xyzx) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))), (_e52.g0_ * vec4<f32>(_e54.g0_.w)));
}

fn plane_rotor_meet(self_1460: Plane, other_1268: Rotor) -> Flector {
    var self_1461: Plane;
    var other_1269: Rotor;

    self_1461 = self_1460;
    other_1269 = other_1268;
    let _e4: Plane = self_1461;
    let _e8: Rotor = other_1269;
    let _e20: Plane = self_1461;
    let _e24: Rotor = other_1269;
    let _e37: Plane = self_1461;
    let _e40: Rotor = other_1269;
    let _e52: Plane = self_1461;
    let _e54: Rotor = other_1269;
    return Flector(((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0))) + ((vec4<f32>(_e20.g0_.z) * vec4<f32>(_e24.g0_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((_e37.g0_.wwwx * _e40.g0_.xyzx) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))), (_e52.g0_ * vec4<f32>(_e54.g0_.w)));
}

fn plane_rotor_geometric_anti_product(self_1462: Plane, other_1270: Rotor) -> Flector {
    var self_1463: Plane;
    var other_1271: Rotor;

    self_1463 = self_1462;
    other_1271 = other_1270;
    let _e4: Plane = self_1463;
    let _e8: Rotor = other_1271;
    let _e20: Plane = self_1463;
    let _e24: Rotor = other_1271;
    let _e37: Plane = self_1463;
    let _e40: Rotor = other_1271;
    let _e52: Plane = self_1463;
    let _e56: Rotor = other_1271;
    let _e67: Plane = self_1463;
    let _e71: Rotor = other_1271;
    let _e83: Plane = self_1463;
    let _e86: Rotor = other_1271;
    return Flector(((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0))) + ((vec4<f32>(_e20.g0_.z) * vec4<f32>(_e24.g0_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((_e37.g0_.wwwx * _e40.g0_.xyzx) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))), ((((vec4<f32>(_e52.g0_.y) * _e56.g0_.zwxz) * vec4<f32>(1.0, 1.0, -(1.0), 0.0)) + ((vec4<f32>(_e67.g0_.z) * _e71.g0_.yxwy) * vec4<f32>(-(1.0), 1.0, 1.0, 0.0))) + ((_e83.g0_.xxxw * _e86.g0_.wzyw) * vec4<f32>(1.0, -(1.0), 1.0, 1.0))));
}

fn plane_rotor_inner_anti_product(self_1464: Plane, other_1272: Rotor) -> Plane {
    var self_1465: Plane;
    var other_1273: Rotor;

    self_1465 = self_1464;
    other_1273 = other_1272;
    let _e4: Plane = self_1465;
    let _e8: Rotor = other_1273;
    let _e19: Plane = self_1465;
    let _e23: Rotor = other_1273;
    let _e35: Plane = self_1465;
    let _e38: Rotor = other_1273;
    return Plane(((((vec4<f32>(_e4.g0_.y) * _e8.g0_.zwxz) * vec4<f32>(1.0, 1.0, -(1.0), 0.0)) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.yxwy) * vec4<f32>(-(1.0), 1.0, 1.0, 0.0))) + ((_e35.g0_.xxxw * _e38.g0_.wzyw) * vec4<f32>(1.0, -(1.0), 1.0, 1.0))));
}

fn plane_rotor_right_anti_contraction(self_1466: Plane, other_1274: Rotor) -> Plane {
    var self_1467: Plane;
    var other_1275: Rotor;

    self_1467 = self_1466;
    other_1275 = other_1274;
    let _e4: Plane = self_1467;
    let _e6: Rotor = other_1275;
    return Plane((_e4.g0_ * vec4<f32>(_e6.g0_.w)));
}

fn plane_translator_inner_product(self_1468: Plane, other_1276: Translator) -> Point {
    var self_1469: Plane;
    var other_1277: Translator;

    self_1469 = self_1468;
    other_1277 = other_1276;
    let _e4: Plane = self_1469;
    let _e8: Translator = other_1277;
    let _e20: Plane = self_1469;
    let _e24: Translator = other_1277;
    let _e37: Plane = self_1469;
    let _e41: Translator = other_1277;
    let _e45: Plane = self_1469;
    let _e49: Translator = other_1277;
    return Point((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0))) + ((vec4<f32>(_e20.g0_.z) * vec4<f32>(_e24.g0_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + (vec4<f32>(_e37.g0_.w) * _e41.g0_)) + ((vec4<f32>(_e45.g0_.x) * vec4<f32>(_e49.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))));
}

fn plane_translator_inner_anti_product(self_1470: Plane, other_1278: Translator) -> Plane {
    var self_1471: Plane;
    var other_1279: Translator;

    self_1471 = self_1470;
    other_1279 = other_1278;
    let _e4: Plane = self_1471;
    let _e8: Translator = other_1279;
    let _e18: Plane = self_1471;
    let _e22: Translator = other_1279;
    let _e33: Plane = self_1471;
    let _e37: Translator = other_1279;
    let _e49: Plane = self_1471;
    let _e53: Translator = other_1279;
    return Plane((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, 1.0)) + ((vec4<f32>(_e18.g0_.z) * _e22.g0_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, 1.0))) + ((vec4<f32>(_e33.g0_.w) * vec4<f32>(_e37.g0_.w)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e49.g0_.x) * _e53.g0_.wxxx) * vec4<f32>(1.0, 0.0, 0.0, 1.0))));
}

fn plane_translator_right_contraction(self_1472: Plane, other_1280: Translator) -> Point {
    var self_1473: Plane;
    var other_1281: Translator;

    self_1473 = self_1472;
    other_1281 = other_1280;
    let _e4: Plane = self_1473;
    let _e8: Translator = other_1281;
    let _e20: Plane = self_1473;
    let _e24: Translator = other_1281;
    let _e37: Plane = self_1473;
    let _e40: Translator = other_1281;
    return Point(((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0))) + ((vec4<f32>(_e20.g0_.z) * vec4<f32>(_e24.g0_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((_e37.g0_.wwwx * _e40.g0_.xyzx) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))));
}

fn plane_translator_right_anti_contraction(self_1474: Plane, other_1282: Translator) -> Plane {
    var self_1475: Plane;
    var other_1283: Translator;

    self_1475 = self_1474;
    other_1283 = other_1282;
    let _e4: Plane = self_1475;
    let _e6: Translator = other_1283;
    return Plane((_e4.g0_ * vec4<f32>(_e6.g0_.w)));
}

fn plane_flector_add(self_1476: Plane, other_1284: Flector) -> Flector {
    var self_1477: Plane;
    var other_1285: Flector;

    self_1477 = self_1476;
    other_1285 = other_1284;
    let _e4: Flector = other_1285;
    let _e6: Plane = self_1477;
    let _e8: Flector = other_1285;
    return Flector(_e4.g0_, (_e6.g0_ + _e8.g1_));
}

fn plane_flector_sub(self_1478: Plane, other_1286: Flector) -> Flector {
    var self_1479: Plane;
    var other_1287: Flector;

    self_1479 = self_1478;
    other_1287 = other_1286;
    let _e6: Flector = other_1287;
    let _e9: Plane = self_1479;
    let _e11: Flector = other_1287;
    return Flector((vec4<f32>(0.0) - _e6.g0_), (_e9.g0_ - _e11.g1_));
}

fn plane_flector_outer_product(self_1480: Plane, other_1288: Flector) -> AntiScalar {
    var self_1481: Plane;
    var other_1289: Flector;

    self_1481 = self_1480;
    other_1289 = other_1288;
    let _e5: Plane = self_1481;
    let _e8: Flector = other_1289;
    let _e13: Plane = self_1481;
    let _e16: Flector = other_1289;
    let _e21: Plane = self_1481;
    let _e24: Flector = other_1289;
    let _e29: Plane = self_1481;
    let _e32: Flector = other_1289;
    return AntiScalar(((((0.0 - (_e5.g0_.x * _e8.g0_.x)) - (_e13.g0_.y * _e16.g0_.y)) - (_e21.g0_.z * _e24.g0_.z)) - (_e29.g0_.w * _e32.g0_.w)));
}

fn plane_flector_wedge(self_1482: Plane, other_1290: Flector) -> AntiScalar {
    var self_1483: Plane;
    var other_1291: Flector;

    self_1483 = self_1482;
    other_1291 = other_1290;
    let _e5: Plane = self_1483;
    let _e8: Flector = other_1291;
    let _e13: Plane = self_1483;
    let _e16: Flector = other_1291;
    let _e21: Plane = self_1483;
    let _e24: Flector = other_1291;
    let _e29: Plane = self_1483;
    let _e32: Flector = other_1291;
    return AntiScalar(((((0.0 - (_e5.g0_.x * _e8.g0_.x)) - (_e13.g0_.y * _e16.g0_.y)) - (_e21.g0_.z * _e24.g0_.z)) - (_e29.g0_.w * _e32.g0_.w)));
}

fn plane_flector_join(self_1484: Plane, other_1292: Flector) -> AntiScalar {
    var self_1485: Plane;
    var other_1293: Flector;

    self_1485 = self_1484;
    other_1293 = other_1292;
    let _e5: Plane = self_1485;
    let _e8: Flector = other_1293;
    let _e13: Plane = self_1485;
    let _e16: Flector = other_1293;
    let _e21: Plane = self_1485;
    let _e24: Flector = other_1293;
    let _e29: Plane = self_1485;
    let _e32: Flector = other_1293;
    return AntiScalar(((((0.0 - (_e5.g0_.x * _e8.g0_.x)) - (_e13.g0_.y * _e16.g0_.y)) - (_e21.g0_.z * _e24.g0_.z)) - (_e29.g0_.w * _e32.g0_.w)));
}

fn plane_flector_inner_anti_product(self_1486: Plane, other_1294: Flector) -> Motor {
    var self_1487: Plane;
    var other_1295: Flector;

    self_1487 = self_1486;
    other_1295 = other_1294;
    let _e4: Plane = self_1487;
    let _e8: Flector = other_1295;
    let _e11: Flector = other_1295;
    let _e14: Flector = other_1295;
    let _e17: Flector = other_1295;
    let _e29: Plane = self_1487;
    let _e33: Flector = other_1295;
    let _e36: Flector = other_1295;
    let _e39: Flector = other_1295;
    let _e42: Flector = other_1295;
    let _e55: Plane = self_1487;
    let _e59: Flector = other_1295;
    let _e62: Flector = other_1295;
    let _e65: Flector = other_1295;
    let _e68: Flector = other_1295;
    let _e81: Plane = self_1487;
    let _e85: Flector = other_1295;
    let _e88: Flector = other_1295;
    let _e91: Flector = other_1295;
    let _e102: Plane = self_1487;
    let _e106: Flector = other_1295;
    let _e109: Flector = other_1295;
    let _e112: Flector = other_1295;
    let _e124: Plane = self_1487;
    let _e128: Flector = other_1295;
    let _e131: Flector = other_1295;
    let _e134: Flector = other_1295;
    return Motor(((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.w, _e11.g0_.w, _e14.g0_.w, _e17.g1_.y)) * vec4<f32>(0.0, -(1.0), 0.0, 1.0)) + ((vec4<f32>(_e29.g0_.z) * vec4<f32>(_e33.g0_.w, _e36.g0_.w, _e39.g0_.w, _e42.g1_.z)) * vec4<f32>(0.0, 0.0, -(1.0), 1.0))) + ((vec4<f32>(_e55.g0_.x) * vec4<f32>(_e59.g0_.w, _e62.g0_.x, _e65.g0_.x, _e68.g1_.x)) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))), ((((vec3<f32>(_e81.g0_.y) * vec3<f32>(_e85.g0_.z, _e88.g0_.z, _e91.g0_.x)) * vec3<f32>(1.0, 0.0, -(1.0))) + ((vec3<f32>(_e102.g0_.z) * vec3<f32>(_e106.g0_.y, _e109.g0_.x, _e112.g0_.y)) * vec3<f32>(-(1.0), 1.0, 0.0))) + ((vec3<f32>(_e124.g0_.x) * vec3<f32>(_e128.g0_.x, _e131.g0_.z, _e134.g0_.y)) * vec3<f32>(0.0, -(1.0), 1.0))));
}

fn plane_flector_left_contraction(self_1488: Plane, other_1296: Flector) -> Scalar {
    var self_1489: Plane;
    var other_1297: Flector;

    self_1489 = self_1488;
    other_1297 = other_1296;
    let _e5: Plane = self_1489;
    let _e8: Flector = other_1297;
    return Scalar((0.0 - (_e5.g0_.w * _e8.g1_.w)));
}

fn plane_flector_left_anti_contraction(self_1490: Plane, other_1298: Flector) -> Motor {
    var self_1491: Plane;
    var other_1299: Flector;

    self_1491 = self_1490;
    other_1299 = other_1298;
    let _e4: Plane = self_1491;
    let _e8: Flector = other_1299;
    let _e11: Flector = other_1299;
    let _e14: Flector = other_1299;
    let _e17: Flector = other_1299;
    let _e29: Plane = self_1491;
    let _e33: Flector = other_1299;
    let _e36: Flector = other_1299;
    let _e39: Flector = other_1299;
    let _e42: Flector = other_1299;
    let _e55: Plane = self_1491;
    let _e59: Flector = other_1299;
    let _e62: Flector = other_1299;
    let _e65: Flector = other_1299;
    let _e68: Flector = other_1299;
    let _e81: Plane = self_1491;
    let _e85: Flector = other_1299;
    let _e88: Flector = other_1299;
    let _e91: Flector = other_1299;
    let _e102: Plane = self_1491;
    let _e106: Flector = other_1299;
    let _e109: Flector = other_1299;
    let _e112: Flector = other_1299;
    let _e124: Plane = self_1491;
    let _e128: Flector = other_1299;
    let _e131: Flector = other_1299;
    let _e134: Flector = other_1299;
    return Motor(((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.w, _e11.g0_.w, _e14.g0_.w, _e17.g1_.y)) * vec4<f32>(0.0, -(1.0), 0.0, 1.0)) + ((vec4<f32>(_e29.g0_.z) * vec4<f32>(_e33.g0_.w, _e36.g0_.w, _e39.g0_.w, _e42.g1_.z)) * vec4<f32>(0.0, 0.0, -(1.0), 1.0))) + ((vec4<f32>(_e55.g0_.x) * vec4<f32>(_e59.g0_.w, _e62.g0_.x, _e65.g0_.x, _e68.g1_.x)) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))), ((((vec3<f32>(_e81.g0_.y) * vec3<f32>(_e85.g0_.z, _e88.g0_.z, _e91.g0_.x)) * vec3<f32>(1.0, 0.0, -(1.0))) + ((vec3<f32>(_e102.g0_.z) * vec3<f32>(_e106.g0_.y, _e109.g0_.x, _e112.g0_.y)) * vec3<f32>(-(1.0), 1.0, 0.0))) + ((vec3<f32>(_e124.g0_.x) * vec3<f32>(_e128.g0_.x, _e131.g0_.z, _e134.g0_.y)) * vec3<f32>(0.0, -(1.0), 1.0))));
}

fn plane_flector_right_anti_contraction(self_1492: Plane, other_1300: Flector) -> AntiScalar {
    var self_1493: Plane;
    var other_1301: Flector;

    self_1493 = self_1492;
    other_1301 = other_1300;
    let _e4: Plane = self_1493;
    let _e7: Flector = other_1301;
    let _e11: Plane = self_1493;
    let _e14: Flector = other_1301;
    let _e19: Plane = self_1493;
    let _e22: Flector = other_1301;
    return AntiScalar((((_e4.g0_.x * _e7.g1_.x) + (_e11.g0_.y * _e14.g1_.y)) + (_e19.g0_.z * _e22.g1_.z)));
}

fn plane_flector_scalar_product(self_1494: Plane, other_1302: Flector) -> Scalar {
    var self_1495: Plane;
    var other_1303: Flector;

    self_1495 = self_1494;
    other_1303 = other_1302;
    let _e5: Plane = self_1495;
    let _e8: Flector = other_1303;
    return Scalar((0.0 - (_e5.g0_.w * _e8.g1_.w)));
}

fn plane_flector_dot(self_1496: Plane, other_1304: Flector) -> Scalar {
    var self_1497: Plane;
    var other_1305: Flector;

    self_1497 = self_1496;
    other_1305 = other_1304;
    let _e5: Plane = self_1497;
    let _e8: Flector = other_1305;
    return Scalar((0.0 - (_e5.g0_.w * _e8.g1_.w)));
}

fn plane_flector_anti_scalar_product(self_1498: Plane, other_1306: Flector) -> AntiScalar {
    var self_1499: Plane;
    var other_1307: Flector;

    self_1499 = self_1498;
    other_1307 = other_1306;
    let _e4: Plane = self_1499;
    let _e7: Flector = other_1307;
    let _e11: Plane = self_1499;
    let _e14: Flector = other_1307;
    let _e19: Plane = self_1499;
    let _e22: Flector = other_1307;
    return AntiScalar((((_e4.g0_.x * _e7.g1_.x) + (_e11.g0_.y * _e14.g1_.y)) + (_e19.g0_.z * _e22.g1_.z)));
}

fn plane_flector_anti_dot(self_1500: Plane, other_1308: Flector) -> AntiScalar {
    var self_1501: Plane;
    var other_1309: Flector;

    self_1501 = self_1500;
    other_1309 = other_1308;
    let _e4: Plane = self_1501;
    let _e7: Flector = other_1309;
    let _e11: Plane = self_1501;
    let _e14: Flector = other_1309;
    let _e19: Plane = self_1501;
    let _e22: Flector = other_1309;
    return AntiScalar((((_e4.g0_.x * _e7.g1_.x) + (_e11.g0_.y * _e14.g1_.y)) + (_e19.g0_.z * _e22.g1_.z)));
}

fn plane_multi_vector_scalar_product(self_1502: Plane, other_1310: MultiVector) -> Scalar {
    var self_1503: Plane;
    var other_1311: MultiVector;

    self_1503 = self_1502;
    other_1311 = other_1310;
    let _e5: Plane = self_1503;
    let _e8: MultiVector = other_1311;
    return Scalar((0.0 - (_e5.g0_.w * _e8.g6_.w)));
}

fn plane_multi_vector_dot(self_1504: Plane, other_1312: MultiVector) -> Scalar {
    var self_1505: Plane;
    var other_1313: MultiVector;

    self_1505 = self_1504;
    other_1313 = other_1312;
    let _e5: Plane = self_1505;
    let _e8: MultiVector = other_1313;
    return Scalar((0.0 - (_e5.g0_.w * _e8.g6_.w)));
}

fn plane_multi_vector_anti_scalar_product(self_1506: Plane, other_1314: MultiVector) -> AntiScalar {
    var self_1507: Plane;
    var other_1315: MultiVector;

    self_1507 = self_1506;
    other_1315 = other_1314;
    let _e4: Plane = self_1507;
    let _e7: MultiVector = other_1315;
    let _e11: Plane = self_1507;
    let _e14: MultiVector = other_1315;
    let _e19: Plane = self_1507;
    let _e22: MultiVector = other_1315;
    return AntiScalar((((_e4.g0_.x * _e7.g6_.x) + (_e11.g0_.y * _e14.g6_.y)) + (_e19.g0_.z * _e22.g6_.z)));
}

fn plane_multi_vector_anti_dot(self_1508: Plane, other_1316: MultiVector) -> AntiScalar {
    var self_1509: Plane;
    var other_1317: MultiVector;

    self_1509 = self_1508;
    other_1317 = other_1316;
    let _e4: Plane = self_1509;
    let _e7: MultiVector = other_1317;
    let _e11: Plane = self_1509;
    let _e14: MultiVector = other_1317;
    let _e19: Plane = self_1509;
    let _e22: MultiVector = other_1317;
    return AntiScalar((((_e4.g0_.x * _e7.g6_.x) + (_e11.g0_.y * _e14.g6_.y)) + (_e19.g0_.z * _e22.g6_.z)));
}

fn plane_squared_magnitude(self_1510: Plane) -> Scalar {
    var self_1511: Plane;

    self_1511 = self_1510;
    let _e2: Plane = self_1511;
    let _e3: Plane = self_1511;
    let _e4: Plane = plane_reversal(_e3);
    let _e5: Scalar = plane_plane_scalar_product(_e2, _e4);
    return _e5;
}

fn plane_magnitude(self_1512: Plane) -> Scalar {
    var self_1513: Plane;

    self_1513 = self_1512;
    let _e2: Plane = self_1513;
    let _e3: Scalar = plane_squared_magnitude(_e2);
    return Scalar(sqrt(_e3.g0_));
}

fn plane_bulk_norm(self_1514: Plane) -> Scalar {
    var self_1515: Plane;

    self_1515 = self_1514;
    let _e2: Plane = self_1515;
    let _e3: Scalar = plane_squared_magnitude(_e2);
    return Scalar(sqrt(_e3.g0_));
}

fn plane_squared_anti_magnitude(self_1516: Plane) -> AntiScalar {
    var self_1517: Plane;

    self_1517 = self_1516;
    let _e2: Plane = self_1517;
    let _e3: Plane = self_1517;
    let _e4: Plane = plane_anti_reversal(_e3);
    let _e5: AntiScalar = plane_plane_anti_scalar_product(_e2, _e4);
    return _e5;
}

fn plane_weight_norm(self_1518: Plane) -> AntiScalar {
    var self_1519: Plane;

    self_1519 = self_1518;
    let _e2: Plane = self_1519;
    let _e3: AntiScalar = plane_squared_anti_magnitude(_e2);
    return AntiScalar(sqrt(_e3.g0_));
}

fn plane_geometric_norm(self_1520: Plane) -> HomogeneousMagnitude {
    var self_1521: Plane;

    self_1521 = self_1520;
    let _e2: Plane = self_1521;
    let _e3: Scalar = plane_bulk_norm(_e2);
    let _e4: Plane = self_1521;
    let _e5: AntiScalar = plane_weight_norm(_e4);
    let _e6: HomogeneousMagnitude = scalar_anti_scalar_add(_e3, _e5);
    return _e6;
}

fn plane_scale(self_1522: Plane, other_1318: f32) -> Plane {
    var self_1523: Plane;
    var other_1319: f32;

    self_1523 = self_1522;
    other_1319 = other_1318;
    let _e4: Plane = self_1523;
    let _e5: f32 = other_1319;
    let _e7: Plane = plane_scalar_geometric_product(_e4, Scalar(_e5));
    return _e7;
}

fn plane_signum(self_1524: Plane) -> Plane {
    var self_1525: Plane;

    self_1525 = self_1524;
    let _e2: Plane = self_1525;
    let _e3: Plane = self_1525;
    let _e4: Scalar = plane_magnitude(_e3);
    let _e9: Plane = plane_scalar_geometric_product(_e2, Scalar((1.0 / _e4.g0_)));
    return _e9;
}

fn plane_inverse(self_1526: Plane) -> Plane {
    var self_1527: Plane;

    self_1527 = self_1526;
    let _e2: Plane = self_1527;
    let _e3: Plane = plane_reversal(_e2);
    let _e4: Plane = self_1527;
    let _e5: Scalar = plane_squared_magnitude(_e4);
    let _e10: Plane = plane_scalar_geometric_product(_e3, Scalar((1.0 / _e5.g0_)));
    return _e10;
}

fn plane_unitize(self_1528: Plane) -> Plane {
    var self_1529: Plane;

    self_1529 = self_1528;
    let _e2: Plane = self_1529;
    let _e3: Plane = self_1529;
    let _e4: AntiScalar = plane_weight_norm(_e3);
    let _e9: Plane = plane_scalar_geometric_product(_e2, Scalar((1.0 / _e4.g0_)));
    return _e9;
}

fn plane_attitude(self_1530: Plane) -> Line {
    var self_1531: Plane;

    self_1531 = self_1530;
    let _e2: Plane = self_1531;
    let _e9: Line = plane_plane_regressive_product(_e2, Plane(vec4<f32>(0.0, 0.0, 0.0, 1.0)));
    return _e9;
}

fn motor_zero() -> Motor {
    return Motor(vec4<f32>(0.0), vec3<f32>(0.0));
}

fn motor_one() -> Motor {
    return Motor(vec4<f32>(0.0), vec3<f32>(0.0));
}

fn motor_neg(self_1532: Motor) -> Motor {
    var self_1533: Motor;

    self_1533 = self_1532;
    let _e2: Motor = self_1533;
    let _e8: Motor = self_1533;
    return Motor((_e2.g0_ * vec4<f32>(-(1.0))), (_e8.g1_ * vec3<f32>(-(1.0))));
}

fn motor_automorphism(self_1534: Motor) -> Motor {
    var self_1535: Motor;

    self_1535 = self_1534;
    let _e2: Motor = self_1535;
    let _e4: Motor = self_1535;
    return Motor(_e2.g0_, _e4.g1_);
}

fn motor_reversal(self_1536: Motor) -> Motor {
    var self_1537: Motor;

    self_1537 = self_1536;
    let _e2: Motor = self_1537;
    let _e13: Motor = self_1537;
    return Motor((_e2.g0_ * vec4<f32>(-(1.0), -(1.0), -(1.0), 1.0)), (_e13.g1_ * vec3<f32>(-(1.0))));
}

fn motor_conjugation(self_1538: Motor) -> Motor {
    var self_1539: Motor;

    self_1539 = self_1538;
    let _e2: Motor = self_1539;
    let _e13: Motor = self_1539;
    return Motor((_e2.g0_ * vec4<f32>(-(1.0), -(1.0), -(1.0), 1.0)), (_e13.g1_ * vec3<f32>(-(1.0))));
}

fn motor_anti_reversal(self_1540: Motor) -> Motor {
    var self_1541: Motor;

    self_1541 = self_1540;
    let _e2: Motor = self_1541;
    let _e13: Motor = self_1541;
    return Motor((_e2.g0_ * vec4<f32>(-(1.0), -(1.0), -(1.0), 1.0)), (_e13.g1_ * vec3<f32>(-(1.0))));
}

fn motor_double_complement(self_1542: Motor) -> Motor {
    var self_1543: Motor;

    self_1543 = self_1542;
    let _e2: Motor = self_1543;
    let _e4: Motor = self_1543;
    return Motor(_e2.g0_, _e4.g1_);
}

fn motor_scalar_geometric_product(self_1544: Motor, other_1320: Scalar) -> Motor {
    var self_1545: Motor;
    var other_1321: Scalar;

    self_1545 = self_1544;
    other_1321 = other_1320;
    let _e4: Motor = self_1545;
    let _e6: Scalar = other_1321;
    let _e10: Motor = self_1545;
    let _e12: Scalar = other_1321;
    return Motor((_e4.g0_ * vec4<f32>(_e6.g0_)), (_e10.g1_ * vec3<f32>(_e12.g0_)));
}

fn motor_scalar_regressive_product(self_1546: Motor, other_1322: Scalar) -> Scalar {
    var self_1547: Motor;
    var other_1323: Scalar;

    self_1547 = self_1546;
    other_1323 = other_1322;
    let _e4: Motor = self_1547;
    let _e7: Scalar = other_1323;
    return Scalar((_e4.g0_.w * _e7.g0_));
}

fn motor_scalar_anti_wedge(self_1548: Motor, other_1324: Scalar) -> Scalar {
    var self_1549: Motor;
    var other_1325: Scalar;

    self_1549 = self_1548;
    other_1325 = other_1324;
    let _e4: Motor = self_1549;
    let _e7: Scalar = other_1325;
    return Scalar((_e4.g0_.w * _e7.g0_));
}

fn motor_scalar_meet(self_1550: Motor, other_1326: Scalar) -> Scalar {
    var self_1551: Motor;
    var other_1327: Scalar;

    self_1551 = self_1550;
    other_1327 = other_1326;
    let _e4: Motor = self_1551;
    let _e7: Scalar = other_1327;
    return Scalar((_e4.g0_.w * _e7.g0_));
}

fn motor_scalar_outer_product(self_1552: Motor, other_1328: Scalar) -> Motor {
    var self_1553: Motor;
    var other_1329: Scalar;

    self_1553 = self_1552;
    other_1329 = other_1328;
    let _e4: Motor = self_1553;
    let _e6: Scalar = other_1329;
    let _e10: Motor = self_1553;
    let _e12: Scalar = other_1329;
    return Motor((_e4.g0_ * vec4<f32>(_e6.g0_)), (_e10.g1_ * vec3<f32>(_e12.g0_)));
}

fn motor_scalar_wedge(self_1554: Motor, other_1330: Scalar) -> Motor {
    var self_1555: Motor;
    var other_1331: Scalar;

    self_1555 = self_1554;
    other_1331 = other_1330;
    let _e4: Motor = self_1555;
    let _e6: Scalar = other_1331;
    let _e10: Motor = self_1555;
    let _e12: Scalar = other_1331;
    return Motor((_e4.g0_ * vec4<f32>(_e6.g0_)), (_e10.g1_ * vec3<f32>(_e12.g0_)));
}

fn motor_scalar_join(self_1556: Motor, other_1332: Scalar) -> Motor {
    var self_1557: Motor;
    var other_1333: Scalar;

    self_1557 = self_1556;
    other_1333 = other_1332;
    let _e4: Motor = self_1557;
    let _e6: Scalar = other_1333;
    let _e10: Motor = self_1557;
    let _e12: Scalar = other_1333;
    return Motor((_e4.g0_ * vec4<f32>(_e6.g0_)), (_e10.g1_ * vec3<f32>(_e12.g0_)));
}

fn motor_scalar_inner_product(self_1558: Motor, other_1334: Scalar) -> Motor {
    var self_1559: Motor;
    var other_1335: Scalar;

    self_1559 = self_1558;
    other_1335 = other_1334;
    let _e4: Motor = self_1559;
    let _e6: Scalar = other_1335;
    let _e10: Motor = self_1559;
    let _e12: Scalar = other_1335;
    return Motor((_e4.g0_ * vec4<f32>(_e6.g0_)), (_e10.g1_ * vec3<f32>(_e12.g0_)));
}

fn motor_scalar_right_contraction(self_1560: Motor, other_1336: Scalar) -> Motor {
    var self_1561: Motor;
    var other_1337: Scalar;

    self_1561 = self_1560;
    other_1337 = other_1336;
    let _e4: Motor = self_1561;
    let _e6: Scalar = other_1337;
    let _e10: Motor = self_1561;
    let _e12: Scalar = other_1337;
    return Motor((_e4.g0_ * vec4<f32>(_e6.g0_)), (_e10.g1_ * vec3<f32>(_e12.g0_)));
}

fn motor_anti_scalar_into(self_1562: Motor) -> AntiScalar {
    var self_1563: Motor;

    self_1563 = self_1562;
    let _e2: Motor = self_1563;
    return AntiScalar(_e2.g0_.w);
}

fn motor_anti_scalar_add(self_1564: Motor, other_1338: AntiScalar) -> Motor {
    var self_1565: Motor;
    var other_1339: AntiScalar;

    self_1565 = self_1564;
    other_1339 = other_1338;
    let _e4: Motor = self_1565;
    let _e6: AntiScalar = other_1339;
    let _e16: Motor = self_1565;
    return Motor((_e4.g0_ + (vec4<f32>(_e6.g0_) * vec4<f32>(0.0, 0.0, 0.0, 1.0))), _e16.g1_);
}

fn motor_anti_scalar_sub(self_1566: Motor, other_1340: AntiScalar) -> Motor {
    var self_1567: Motor;
    var other_1341: AntiScalar;

    self_1567 = self_1566;
    other_1341 = other_1340;
    let _e4: Motor = self_1567;
    let _e6: AntiScalar = other_1341;
    let _e16: Motor = self_1567;
    return Motor((_e4.g0_ - (vec4<f32>(_e6.g0_) * vec4<f32>(0.0, 0.0, 0.0, 1.0))), _e16.g1_);
}

fn motor_anti_scalar_regressive_product(self_1568: Motor, other_1342: AntiScalar) -> Motor {
    var self_1569: Motor;
    var other_1343: AntiScalar;

    self_1569 = self_1568;
    other_1343 = other_1342;
    let _e4: Motor = self_1569;
    let _e6: AntiScalar = other_1343;
    let _e10: Motor = self_1569;
    let _e12: AntiScalar = other_1343;
    return Motor((_e4.g0_ * vec4<f32>(_e6.g0_)), (_e10.g1_ * vec3<f32>(_e12.g0_)));
}

fn motor_anti_scalar_anti_wedge(self_1570: Motor, other_1344: AntiScalar) -> Motor {
    var self_1571: Motor;
    var other_1345: AntiScalar;

    self_1571 = self_1570;
    other_1345 = other_1344;
    let _e4: Motor = self_1571;
    let _e6: AntiScalar = other_1345;
    let _e10: Motor = self_1571;
    let _e12: AntiScalar = other_1345;
    return Motor((_e4.g0_ * vec4<f32>(_e6.g0_)), (_e10.g1_ * vec3<f32>(_e12.g0_)));
}

fn motor_anti_scalar_meet(self_1572: Motor, other_1346: AntiScalar) -> Motor {
    var self_1573: Motor;
    var other_1347: AntiScalar;

    self_1573 = self_1572;
    other_1347 = other_1346;
    let _e4: Motor = self_1573;
    let _e6: AntiScalar = other_1347;
    let _e10: Motor = self_1573;
    let _e12: AntiScalar = other_1347;
    return Motor((_e4.g0_ * vec4<f32>(_e6.g0_)), (_e10.g1_ * vec3<f32>(_e12.g0_)));
}

fn motor_anti_scalar_geometric_anti_product(self_1574: Motor, other_1348: AntiScalar) -> Motor {
    var self_1575: Motor;
    var other_1349: AntiScalar;

    self_1575 = self_1574;
    other_1349 = other_1348;
    let _e4: Motor = self_1575;
    let _e6: AntiScalar = other_1349;
    let _e10: Motor = self_1575;
    let _e12: AntiScalar = other_1349;
    return Motor((_e4.g0_ * vec4<f32>(_e6.g0_)), (_e10.g1_ * vec3<f32>(_e12.g0_)));
}

fn motor_anti_scalar_inner_anti_product(self_1576: Motor, other_1350: AntiScalar) -> Motor {
    var self_1577: Motor;
    var other_1351: AntiScalar;

    self_1577 = self_1576;
    other_1351 = other_1350;
    let _e4: Motor = self_1577;
    let _e6: AntiScalar = other_1351;
    let _e10: Motor = self_1577;
    let _e12: AntiScalar = other_1351;
    return Motor((_e4.g0_ * vec4<f32>(_e6.g0_)), (_e10.g1_ * vec3<f32>(_e12.g0_)));
}

fn motor_anti_scalar_left_anti_contraction(self_1578: Motor, other_1352: AntiScalar) -> AntiScalar {
    var self_1579: Motor;
    var other_1353: AntiScalar;

    self_1579 = self_1578;
    other_1353 = other_1352;
    let _e4: Motor = self_1579;
    let _e7: AntiScalar = other_1353;
    return AntiScalar((_e4.g0_.w * _e7.g0_));
}

fn motor_anti_scalar_right_anti_contraction(self_1580: Motor, other_1354: AntiScalar) -> Motor {
    var self_1581: Motor;
    var other_1355: AntiScalar;

    self_1581 = self_1580;
    other_1355 = other_1354;
    let _e4: Motor = self_1581;
    let _e6: AntiScalar = other_1355;
    let _e10: Motor = self_1581;
    let _e12: AntiScalar = other_1355;
    return Motor((_e4.g0_ * vec4<f32>(_e6.g0_)), (_e10.g1_ * vec3<f32>(_e12.g0_)));
}

fn motor_anti_scalar_anti_scalar_product(self_1582: Motor, other_1356: AntiScalar) -> AntiScalar {
    var self_1583: Motor;
    var other_1357: AntiScalar;

    self_1583 = self_1582;
    other_1357 = other_1356;
    let _e4: Motor = self_1583;
    let _e7: AntiScalar = other_1357;
    return AntiScalar((_e4.g0_.w * _e7.g0_));
}

fn motor_anti_scalar_anti_dot(self_1584: Motor, other_1358: AntiScalar) -> AntiScalar {
    var self_1585: Motor;
    var other_1359: AntiScalar;

    self_1585 = self_1584;
    other_1359 = other_1358;
    let _e4: Motor = self_1585;
    let _e7: AntiScalar = other_1359;
    return AntiScalar((_e4.g0_.w * _e7.g0_));
}

fn motor_homogeneous_magnitude_geometric_product(self_1586: Motor, other_1360: HomogeneousMagnitude) -> Motor {
    var self_1587: Motor;
    var other_1361: HomogeneousMagnitude;

    self_1587 = self_1586;
    other_1361 = other_1360;
    let _e4: Motor = self_1587;
    let _e8: HomogeneousMagnitude = other_1361;
    let _e19: Motor = self_1587;
    let _e23: HomogeneousMagnitude = other_1361;
    let _e35: Motor = self_1587;
    let _e39: HomogeneousMagnitude = other_1361;
    let _e51: Motor = self_1587;
    let _e53: HomogeneousMagnitude = other_1361;
    let _e59: Motor = self_1587;
    let _e61: HomogeneousMagnitude = other_1361;
    return Motor((((((vec4<f32>(_e4.g1_.x) * vec4<f32>(_e8.g0_.y)) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) + ((vec4<f32>(_e19.g1_.y) * vec4<f32>(_e23.g0_.y)) * vec4<f32>(0.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e35.g1_.z) * vec4<f32>(_e39.g0_.y)) * vec4<f32>(0.0, 0.0, 1.0, 0.0))) + (_e51.g0_ * vec4<f32>(_e53.g0_.x))), (_e59.g1_ * vec3<f32>(_e61.g0_.x)));
}

fn motor_homogeneous_magnitude_outer_product(self_1588: Motor, other_1362: HomogeneousMagnitude) -> Motor {
    var self_1589: Motor;
    var other_1363: HomogeneousMagnitude;

    self_1589 = self_1588;
    other_1363 = other_1362;
    let _e4: Motor = self_1589;
    let _e6: HomogeneousMagnitude = other_1363;
    let _e11: Motor = self_1589;
    let _e13: HomogeneousMagnitude = other_1363;
    return Motor((_e4.g0_ * vec4<f32>(_e6.g0_.x)), (_e11.g1_ * vec3<f32>(_e13.g0_.x)));
}

fn motor_homogeneous_magnitude_wedge(self_1590: Motor, other_1364: HomogeneousMagnitude) -> Motor {
    var self_1591: Motor;
    var other_1365: HomogeneousMagnitude;

    self_1591 = self_1590;
    other_1365 = other_1364;
    let _e4: Motor = self_1591;
    let _e6: HomogeneousMagnitude = other_1365;
    let _e11: Motor = self_1591;
    let _e13: HomogeneousMagnitude = other_1365;
    return Motor((_e4.g0_ * vec4<f32>(_e6.g0_.x)), (_e11.g1_ * vec3<f32>(_e13.g0_.x)));
}

fn motor_homogeneous_magnitude_join(self_1592: Motor, other_1366: HomogeneousMagnitude) -> Motor {
    var self_1593: Motor;
    var other_1367: HomogeneousMagnitude;

    self_1593 = self_1592;
    other_1367 = other_1366;
    let _e4: Motor = self_1593;
    let _e6: HomogeneousMagnitude = other_1367;
    let _e11: Motor = self_1593;
    let _e13: HomogeneousMagnitude = other_1367;
    return Motor((_e4.g0_ * vec4<f32>(_e6.g0_.x)), (_e11.g1_ * vec3<f32>(_e13.g0_.x)));
}

fn motor_homogeneous_magnitude_inner_product(self_1594: Motor, other_1368: HomogeneousMagnitude) -> Motor {
    var self_1595: Motor;
    var other_1369: HomogeneousMagnitude;

    self_1595 = self_1594;
    other_1369 = other_1368;
    let _e4: Motor = self_1595;
    let _e8: HomogeneousMagnitude = other_1369;
    let _e19: Motor = self_1595;
    let _e23: HomogeneousMagnitude = other_1369;
    let _e35: Motor = self_1595;
    let _e39: HomogeneousMagnitude = other_1369;
    let _e51: Motor = self_1595;
    let _e53: HomogeneousMagnitude = other_1369;
    let _e59: Motor = self_1595;
    let _e61: HomogeneousMagnitude = other_1369;
    return Motor((((((vec4<f32>(_e4.g1_.x) * vec4<f32>(_e8.g0_.y)) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) + ((vec4<f32>(_e19.g1_.y) * vec4<f32>(_e23.g0_.y)) * vec4<f32>(0.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e35.g1_.z) * vec4<f32>(_e39.g0_.y)) * vec4<f32>(0.0, 0.0, 1.0, 0.0))) + (_e51.g0_ * vec4<f32>(_e53.g0_.x))), (_e59.g1_ * vec3<f32>(_e61.g0_.x)));
}

fn motor_homogeneous_magnitude_right_contraction(self_1596: Motor, other_1370: HomogeneousMagnitude) -> Motor {
    var self_1597: Motor;
    var other_1371: HomogeneousMagnitude;

    self_1597 = self_1596;
    other_1371 = other_1370;
    let _e4: Motor = self_1597;
    let _e6: HomogeneousMagnitude = other_1371;
    let _e11: Motor = self_1597;
    let _e13: HomogeneousMagnitude = other_1371;
    return Motor((_e4.g0_ * vec4<f32>(_e6.g0_.x)), (_e11.g1_ * vec3<f32>(_e13.g0_.x)));
}

fn motor_homogeneous_magnitude_right_anti_contraction(self_1598: Motor, other_1372: HomogeneousMagnitude) -> Motor {
    var self_1599: Motor;
    var other_1373: HomogeneousMagnitude;

    self_1599 = self_1598;
    other_1373 = other_1372;
    let _e4: Motor = self_1599;
    let _e6: HomogeneousMagnitude = other_1373;
    let _e11: Motor = self_1599;
    let _e13: HomogeneousMagnitude = other_1373;
    return Motor((_e4.g0_ * vec4<f32>(_e6.g0_.y)), (_e11.g1_ * vec3<f32>(_e13.g0_.y)));
}

fn motor_homogeneous_magnitude_anti_scalar_product(self_1600: Motor, other_1374: HomogeneousMagnitude) -> AntiScalar {
    var self_1601: Motor;
    var other_1375: HomogeneousMagnitude;

    self_1601 = self_1600;
    other_1375 = other_1374;
    let _e4: Motor = self_1601;
    let _e7: HomogeneousMagnitude = other_1375;
    return AntiScalar((_e4.g0_.w * _e7.g0_.y));
}

fn motor_homogeneous_magnitude_anti_dot(self_1602: Motor, other_1376: HomogeneousMagnitude) -> AntiScalar {
    var self_1603: Motor;
    var other_1377: HomogeneousMagnitude;

    self_1603 = self_1602;
    other_1377 = other_1376;
    let _e4: Motor = self_1603;
    let _e7: HomogeneousMagnitude = other_1377;
    return AntiScalar((_e4.g0_.w * _e7.g0_.y));
}

fn motor_point_geometric_product(self_1604: Motor, other_1378: Point) -> Flector {
    var self_1605: Motor;
    var other_1379: Point;

    self_1605 = self_1604;
    other_1379 = other_1378;
    let _e4: Motor = self_1605;
    let _e8: Point = other_1379;
    let _e19: Motor = self_1605;
    let _e23: Point = other_1379;
    let _e35: Motor = self_1605;
    let _e39: Point = other_1379;
    let _e51: Motor = self_1605;
    let _e55: Point = other_1379;
    let _e67: Motor = self_1605;
    let _e71: Point = other_1379;
    let _e83: Motor = self_1605;
    let _e87: Point = other_1379;
    let _e99: Motor = self_1605;
    let _e103: Point = other_1379;
    let _e114: Motor = self_1605;
    let _e118: Point = other_1379;
    let _e130: Motor = self_1605;
    let _e134: Point = other_1379;
    let _e148: Motor = self_1605;
    let _e152: Point = other_1379;
    let _e164: Motor = self_1605;
    let _e168: Point = other_1379;
    let _e180: Motor = self_1605;
    let _e184: Point = other_1379;
    let _e196: Motor = self_1605;
    let _e200: Point = other_1379;
    return Flector((((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y)) * vec4<f32>(0.0, 0.0, 0.0, 1.0)) + ((vec4<f32>(_e19.g0_.z) * vec4<f32>(_e23.g0_.z)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e35.g1_.x) * _e39.g0_.zzyz) * vec4<f32>(0.0, 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e51.g1_.y) * _e55.g0_.zzxz) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0))) + ((vec4<f32>(_e67.g1_.z) * _e71.g0_.yxyy) * vec4<f32>(1.0, -(1.0), 0.0, 0.0))) + ((vec4<f32>(_e83.g0_.x) * vec4<f32>(_e87.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))), ((((((((vec4<f32>(_e99.g0_.y) * _e103.g0_.zzxz) * vec4<f32>(1.0, 0.0, -(1.0), 0.0)) + ((vec4<f32>(_e114.g0_.z) * _e118.g0_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))) + ((vec4<f32>(_e130.g0_.w) * _e134.g0_.xyzx) * vec4<f32>(-(1.0), -(1.0), -(1.0), 0.0))) + ((vec4<f32>(_e148.g1_.x) * _e152.g0_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e164.g1_.y) * _e168.g0_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0)))) + ((vec4<f32>(_e180.g1_.z) * _e184.g0_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e196.g0_.x) * _e200.g0_.xzyx) * vec4<f32>(0.0, -(1.0), 1.0, 0.0))));
}

fn motor_point_regressive_product(self_1606: Motor, other_1380: Point) -> Point {
    var self_1607: Motor;
    var other_1381: Point;

    self_1607 = self_1606;
    other_1381 = other_1380;
    let _e4: Motor = self_1607;
    let _e8: Point = other_1381;
    return Point((vec4<f32>(_e4.g0_.w) * _e8.g0_));
}

fn motor_point_anti_wedge(self_1608: Motor, other_1382: Point) -> Point {
    var self_1609: Motor;
    var other_1383: Point;

    self_1609 = self_1608;
    other_1383 = other_1382;
    let _e4: Motor = self_1609;
    let _e8: Point = other_1383;
    return Point((vec4<f32>(_e4.g0_.w) * _e8.g0_));
}

fn motor_point_meet(self_1610: Motor, other_1384: Point) -> Point {
    var self_1611: Motor;
    var other_1385: Point;

    self_1611 = self_1610;
    other_1385 = other_1384;
    let _e4: Motor = self_1611;
    let _e8: Point = other_1385;
    return Point((vec4<f32>(_e4.g0_.w) * _e8.g0_));
}

fn motor_point_outer_product(self_1612: Motor, other_1386: Point) -> Plane {
    var self_1613: Motor;
    var other_1387: Point;

    self_1613 = self_1612;
    other_1387 = other_1386;
    let _e4: Motor = self_1613;
    let _e8: Point = other_1387;
    let _e19: Motor = self_1613;
    let _e23: Point = other_1387;
    let _e35: Motor = self_1613;
    let _e39: Point = other_1387;
    let _e51: Motor = self_1613;
    let _e55: Point = other_1387;
    let _e67: Motor = self_1613;
    let _e71: Point = other_1387;
    let _e83: Motor = self_1613;
    let _e87: Point = other_1387;
    return Plane((((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.zzxz) * vec4<f32>(1.0, 0.0, -(1.0), 0.0)) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))) + ((vec4<f32>(_e35.g1_.x) * _e39.g0_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e51.g1_.y) * _e55.g0_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0)))) + ((vec4<f32>(_e67.g1_.z) * _e71.g0_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e83.g0_.x) * _e87.g0_.xzyx) * vec4<f32>(0.0, -(1.0), 1.0, 0.0))));
}

fn motor_point_wedge(self_1614: Motor, other_1388: Point) -> Plane {
    var self_1615: Motor;
    var other_1389: Point;

    self_1615 = self_1614;
    other_1389 = other_1388;
    let _e4: Motor = self_1615;
    let _e8: Point = other_1389;
    let _e19: Motor = self_1615;
    let _e23: Point = other_1389;
    let _e35: Motor = self_1615;
    let _e39: Point = other_1389;
    let _e51: Motor = self_1615;
    let _e55: Point = other_1389;
    let _e67: Motor = self_1615;
    let _e71: Point = other_1389;
    let _e83: Motor = self_1615;
    let _e87: Point = other_1389;
    return Plane((((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.zzxz) * vec4<f32>(1.0, 0.0, -(1.0), 0.0)) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))) + ((vec4<f32>(_e35.g1_.x) * _e39.g0_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e51.g1_.y) * _e55.g0_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0)))) + ((vec4<f32>(_e67.g1_.z) * _e71.g0_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e83.g0_.x) * _e87.g0_.xzyx) * vec4<f32>(0.0, -(1.0), 1.0, 0.0))));
}

fn motor_point_join(self_1616: Motor, other_1390: Point) -> Plane {
    var self_1617: Motor;
    var other_1391: Point;

    self_1617 = self_1616;
    other_1391 = other_1390;
    let _e4: Motor = self_1617;
    let _e8: Point = other_1391;
    let _e19: Motor = self_1617;
    let _e23: Point = other_1391;
    let _e35: Motor = self_1617;
    let _e39: Point = other_1391;
    let _e51: Motor = self_1617;
    let _e55: Point = other_1391;
    let _e67: Motor = self_1617;
    let _e71: Point = other_1391;
    let _e83: Motor = self_1617;
    let _e87: Point = other_1391;
    return Plane((((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.zzxz) * vec4<f32>(1.0, 0.0, -(1.0), 0.0)) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))) + ((vec4<f32>(_e35.g1_.x) * _e39.g0_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e51.g1_.y) * _e55.g0_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0)))) + ((vec4<f32>(_e67.g1_.z) * _e71.g0_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e83.g0_.x) * _e87.g0_.xzyx) * vec4<f32>(0.0, -(1.0), 1.0, 0.0))));
}

fn motor_point_geometric_anti_product(self_1618: Motor, other_1392: Point) -> Flector {
    var self_1619: Motor;
    var other_1393: Point;

    self_1619 = self_1618;
    other_1393 = other_1392;
    let _e4: Motor = self_1619;
    let _e8: Point = other_1393;
    let _e19: Motor = self_1619;
    let _e23: Point = other_1393;
    let _e35: Motor = self_1619;
    let _e39: Point = other_1393;
    let _e43: Motor = self_1619;
    let _e47: Point = other_1393;
    let _e59: Motor = self_1619;
    let _e63: Point = other_1393;
    let _e75: Motor = self_1619;
    let _e79: Point = other_1393;
    let _e91: Motor = self_1619;
    let _e95: Point = other_1393;
    let _e107: Motor = self_1619;
    let _e111: Point = other_1393;
    let _e122: Motor = self_1619;
    let _e126: Point = other_1393;
    let _e138: Motor = self_1619;
    let _e142: Point = other_1393;
    return Flector(((((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.zzxz) * vec4<f32>(1.0, 0.0, -(1.0), 0.0)) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))) + (vec4<f32>(_e35.g0_.w) * _e39.g0_)) + ((vec4<f32>(_e43.g1_.x) * vec4<f32>(_e47.g0_.w)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e59.g1_.y) * vec4<f32>(_e63.g0_.w)) * vec4<f32>(0.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e75.g1_.z) * vec4<f32>(_e79.g0_.w)) * vec4<f32>(0.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e91.g0_.x) * _e95.g0_.xzyx) * vec4<f32>(0.0, -(1.0), 1.0, 0.0))), ((((vec4<f32>(_e107.g0_.y) * _e111.g0_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e122.g0_.z) * _e126.g0_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e138.g0_.x) * _e142.g0_.wxxx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))));
}

fn motor_point_inner_anti_product(self_1620: Motor, other_1394: Point) -> Flector {
    var self_1621: Motor;
    var other_1395: Point;

    self_1621 = self_1620;
    other_1395 = other_1394;
    let _e4: Motor = self_1621;
    let _e8: Point = other_1395;
    let _e11: Motor = self_1621;
    let _e15: Point = other_1395;
    let _e26: Motor = self_1621;
    let _e30: Point = other_1395;
    let _e42: Motor = self_1621;
    let _e46: Point = other_1395;
    return Flector((vec4<f32>(_e4.g0_.w) * _e8.g0_), ((((vec4<f32>(_e11.g0_.y) * _e15.g0_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e26.g0_.z) * _e30.g0_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e42.g0_.x) * _e46.g0_.wxxx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))));
}

fn motor_point_left_anti_contraction(self_1622: Motor, other_1396: Point) -> Flector {
    var self_1623: Motor;
    var other_1397: Point;

    self_1623 = self_1622;
    other_1397 = other_1396;
    let _e4: Motor = self_1623;
    let _e8: Point = other_1397;
    let _e11: Motor = self_1623;
    let _e15: Point = other_1397;
    let _e26: Motor = self_1623;
    let _e30: Point = other_1397;
    let _e42: Motor = self_1623;
    let _e46: Point = other_1397;
    return Flector((vec4<f32>(_e4.g0_.w) * _e8.g0_), ((((vec4<f32>(_e11.g0_.y) * _e15.g0_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e26.g0_.z) * _e30.g0_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e42.g0_.x) * _e46.g0_.wxxx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))));
}

fn motor_line_into(self_1624: Motor) -> Line {
    var self_1625: Motor;

    self_1625 = self_1624;
    let _e2: Motor = self_1625;
    let _e5: Motor = self_1625;
    let _e8: Motor = self_1625;
    let _e12: Motor = self_1625;
    return Line(vec3<f32>(_e2.g0_.x, _e5.g0_.y, _e8.g0_.z), _e12.g1_);
}

fn motor_line_add(self_1626: Motor, other_1398: Line) -> Motor {
    var self_1627: Motor;
    var other_1399: Line;

    self_1627 = self_1626;
    other_1399 = other_1398;
    let _e4: Motor = self_1627;
    let _e6: Line = other_1399;
    let _e9: Line = other_1399;
    let _e12: Line = other_1399;
    let _e15: Line = other_1399;
    let _e26: Motor = self_1627;
    let _e28: Line = other_1399;
    return Motor((_e4.g0_ + (vec4<f32>(_e6.g0_.x, _e9.g0_.y, _e12.g0_.z, _e15.g0_.x) * vec4<f32>(1.0, 1.0, 1.0, 0.0))), (_e26.g1_ + _e28.g1_));
}

fn motor_line_sub(self_1628: Motor, other_1400: Line) -> Motor {
    var self_1629: Motor;
    var other_1401: Line;

    self_1629 = self_1628;
    other_1401 = other_1400;
    let _e4: Motor = self_1629;
    let _e6: Line = other_1401;
    let _e9: Line = other_1401;
    let _e12: Line = other_1401;
    let _e15: Line = other_1401;
    let _e26: Motor = self_1629;
    let _e28: Line = other_1401;
    return Motor((_e4.g0_ - (vec4<f32>(_e6.g0_.x, _e9.g0_.y, _e12.g0_.z, _e15.g0_.x) * vec4<f32>(1.0, 1.0, 1.0, 0.0))), (_e26.g1_ - _e28.g1_));
}

fn motor_line_outer_product(self_1630: Motor, other_1402: Line) -> AntiScalar {
    var self_1631: Motor;
    var other_1403: Line;

    self_1631 = self_1630;
    other_1403 = other_1402;
    let _e5: Motor = self_1631;
    let _e8: Line = other_1403;
    let _e13: Motor = self_1631;
    let _e16: Line = other_1403;
    let _e21: Motor = self_1631;
    let _e24: Line = other_1403;
    let _e29: Motor = self_1631;
    let _e32: Line = other_1403;
    let _e37: Motor = self_1631;
    let _e40: Line = other_1403;
    let _e45: Motor = self_1631;
    let _e48: Line = other_1403;
    return AntiScalar(((((((0.0 - (_e5.g0_.x * _e8.g1_.x)) - (_e13.g0_.y * _e16.g1_.y)) - (_e21.g0_.z * _e24.g1_.z)) - (_e29.g1_.x * _e32.g0_.x)) - (_e37.g1_.y * _e40.g0_.y)) - (_e45.g1_.z * _e48.g0_.z)));
}

fn motor_line_wedge(self_1632: Motor, other_1404: Line) -> AntiScalar {
    var self_1633: Motor;
    var other_1405: Line;

    self_1633 = self_1632;
    other_1405 = other_1404;
    let _e5: Motor = self_1633;
    let _e8: Line = other_1405;
    let _e13: Motor = self_1633;
    let _e16: Line = other_1405;
    let _e21: Motor = self_1633;
    let _e24: Line = other_1405;
    let _e29: Motor = self_1633;
    let _e32: Line = other_1405;
    let _e37: Motor = self_1633;
    let _e40: Line = other_1405;
    let _e45: Motor = self_1633;
    let _e48: Line = other_1405;
    return AntiScalar(((((((0.0 - (_e5.g0_.x * _e8.g1_.x)) - (_e13.g0_.y * _e16.g1_.y)) - (_e21.g0_.z * _e24.g1_.z)) - (_e29.g1_.x * _e32.g0_.x)) - (_e37.g1_.y * _e40.g0_.y)) - (_e45.g1_.z * _e48.g0_.z)));
}

fn motor_line_join(self_1634: Motor, other_1406: Line) -> AntiScalar {
    var self_1635: Motor;
    var other_1407: Line;

    self_1635 = self_1634;
    other_1407 = other_1406;
    let _e5: Motor = self_1635;
    let _e8: Line = other_1407;
    let _e13: Motor = self_1635;
    let _e16: Line = other_1407;
    let _e21: Motor = self_1635;
    let _e24: Line = other_1407;
    let _e29: Motor = self_1635;
    let _e32: Line = other_1407;
    let _e37: Motor = self_1635;
    let _e40: Line = other_1407;
    let _e45: Motor = self_1635;
    let _e48: Line = other_1407;
    return AntiScalar(((((((0.0 - (_e5.g0_.x * _e8.g1_.x)) - (_e13.g0_.y * _e16.g1_.y)) - (_e21.g0_.z * _e24.g1_.z)) - (_e29.g1_.x * _e32.g0_.x)) - (_e37.g1_.y * _e40.g0_.y)) - (_e45.g1_.z * _e48.g0_.z)));
}

fn motor_line_inner_anti_product(self_1636: Motor, other_1408: Line) -> Motor {
    var self_1637: Motor;
    var other_1409: Line;

    self_1637 = self_1636;
    other_1409 = other_1408;
    let _e4: Motor = self_1637;
    let _e8: Line = other_1409;
    let _e20: Motor = self_1637;
    let _e24: Line = other_1409;
    let _e37: Motor = self_1637;
    let _e40: Line = other_1409;
    let _e43: Line = other_1409;
    let _e46: Line = other_1409;
    let _e49: Line = other_1409;
    let _e62: Motor = self_1637;
    let _e66: Line = other_1409;
    return Motor(((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0))) + ((vec4<f32>(_e20.g0_.z) * vec4<f32>(_e24.g0_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((_e37.g0_.wwwx * vec4<f32>(_e40.g0_.x, _e43.g0_.y, _e46.g0_.z, _e49.g0_.x)) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))), (vec3<f32>(_e62.g0_.w) * _e66.g1_));
}

fn motor_line_left_contraction(self_1638: Motor, other_1410: Line) -> Scalar {
    var self_1639: Motor;
    var other_1411: Line;

    self_1639 = self_1638;
    other_1411 = other_1410;
    let _e5: Motor = self_1639;
    let _e8: Line = other_1411;
    let _e13: Motor = self_1639;
    let _e16: Line = other_1411;
    let _e21: Motor = self_1639;
    let _e24: Line = other_1411;
    return Scalar((((0.0 - (_e5.g1_.x * _e8.g1_.x)) - (_e13.g1_.y * _e16.g1_.y)) - (_e21.g1_.z * _e24.g1_.z)));
}

fn motor_line_left_anti_contraction(self_1640: Motor, other_1412: Line) -> Motor {
    var self_1641: Motor;
    var other_1413: Line;

    self_1641 = self_1640;
    other_1413 = other_1412;
    let _e4: Motor = self_1641;
    let _e8: Line = other_1413;
    let _e20: Motor = self_1641;
    let _e24: Line = other_1413;
    let _e37: Motor = self_1641;
    let _e40: Line = other_1413;
    let _e43: Line = other_1413;
    let _e46: Line = other_1413;
    let _e49: Line = other_1413;
    let _e62: Motor = self_1641;
    let _e66: Line = other_1413;
    return Motor(((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0))) + ((vec4<f32>(_e20.g0_.z) * vec4<f32>(_e24.g0_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((_e37.g0_.wwwx * vec4<f32>(_e40.g0_.x, _e43.g0_.y, _e46.g0_.z, _e49.g0_.x)) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))), (vec3<f32>(_e62.g0_.w) * _e66.g1_));
}

fn motor_line_right_anti_contraction(self_1642: Motor, other_1414: Line) -> AntiScalar {
    var self_1643: Motor;
    var other_1415: Line;

    self_1643 = self_1642;
    other_1415 = other_1414;
    let _e5: Motor = self_1643;
    let _e8: Line = other_1415;
    let _e13: Motor = self_1643;
    let _e16: Line = other_1415;
    let _e21: Motor = self_1643;
    let _e24: Line = other_1415;
    return AntiScalar((((0.0 - (_e5.g0_.x * _e8.g0_.x)) - (_e13.g0_.y * _e16.g0_.y)) - (_e21.g0_.z * _e24.g0_.z)));
}

fn motor_line_scalar_product(self_1644: Motor, other_1416: Line) -> Scalar {
    var self_1645: Motor;
    var other_1417: Line;

    self_1645 = self_1644;
    other_1417 = other_1416;
    let _e5: Motor = self_1645;
    let _e8: Line = other_1417;
    let _e13: Motor = self_1645;
    let _e16: Line = other_1417;
    let _e21: Motor = self_1645;
    let _e24: Line = other_1417;
    return Scalar((((0.0 - (_e5.g1_.x * _e8.g1_.x)) - (_e13.g1_.y * _e16.g1_.y)) - (_e21.g1_.z * _e24.g1_.z)));
}

fn motor_line_dot(self_1646: Motor, other_1418: Line) -> Scalar {
    var self_1647: Motor;
    var other_1419: Line;

    self_1647 = self_1646;
    other_1419 = other_1418;
    let _e5: Motor = self_1647;
    let _e8: Line = other_1419;
    let _e13: Motor = self_1647;
    let _e16: Line = other_1419;
    let _e21: Motor = self_1647;
    let _e24: Line = other_1419;
    return Scalar((((0.0 - (_e5.g1_.x * _e8.g1_.x)) - (_e13.g1_.y * _e16.g1_.y)) - (_e21.g1_.z * _e24.g1_.z)));
}

fn motor_line_anti_scalar_product(self_1648: Motor, other_1420: Line) -> AntiScalar {
    var self_1649: Motor;
    var other_1421: Line;

    self_1649 = self_1648;
    other_1421 = other_1420;
    let _e5: Motor = self_1649;
    let _e8: Line = other_1421;
    let _e13: Motor = self_1649;
    let _e16: Line = other_1421;
    let _e21: Motor = self_1649;
    let _e24: Line = other_1421;
    return AntiScalar((((0.0 - (_e5.g0_.x * _e8.g0_.x)) - (_e13.g0_.y * _e16.g0_.y)) - (_e21.g0_.z * _e24.g0_.z)));
}

fn motor_line_anti_dot(self_1650: Motor, other_1422: Line) -> AntiScalar {
    var self_1651: Motor;
    var other_1423: Line;

    self_1651 = self_1650;
    other_1423 = other_1422;
    let _e5: Motor = self_1651;
    let _e8: Line = other_1423;
    let _e13: Motor = self_1651;
    let _e16: Line = other_1423;
    let _e21: Motor = self_1651;
    let _e24: Line = other_1423;
    return AntiScalar((((0.0 - (_e5.g0_.x * _e8.g0_.x)) - (_e13.g0_.y * _e16.g0_.y)) - (_e21.g0_.z * _e24.g0_.z)));
}

fn motor_plane_regressive_product(self_1652: Motor, other_1424: Plane) -> Flector {
    var self_1653: Motor;
    var other_1425: Plane;

    self_1653 = self_1652;
    other_1425 = other_1424;
    let _e4: Motor = self_1653;
    let _e8: Plane = other_1425;
    let _e19: Motor = self_1653;
    let _e23: Plane = other_1425;
    let _e35: Motor = self_1653;
    let _e39: Plane = other_1425;
    let _e51: Motor = self_1653;
    let _e55: Plane = other_1425;
    let _e67: Motor = self_1653;
    let _e71: Plane = other_1425;
    let _e83: Motor = self_1653;
    let _e87: Plane = other_1425;
    let _e99: Motor = self_1653;
    let _e103: Plane = other_1425;
    return Flector((((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e35.g1_.x) * _e39.g0_.zzyz) * vec4<f32>(0.0, -(1.0), 1.0, 0.0))) + ((vec4<f32>(_e51.g1_.y) * _e55.g0_.zzxz) * vec4<f32>(1.0, 0.0, -(1.0), 0.0))) + ((vec4<f32>(_e67.g1_.z) * _e71.g0_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))) + ((vec4<f32>(_e83.g0_.x) * _e87.g0_.wxxx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))), (vec4<f32>(_e99.g0_.w) * _e103.g0_));
}

fn motor_plane_anti_wedge(self_1654: Motor, other_1426: Plane) -> Flector {
    var self_1655: Motor;
    var other_1427: Plane;

    self_1655 = self_1654;
    other_1427 = other_1426;
    let _e4: Motor = self_1655;
    let _e8: Plane = other_1427;
    let _e19: Motor = self_1655;
    let _e23: Plane = other_1427;
    let _e35: Motor = self_1655;
    let _e39: Plane = other_1427;
    let _e51: Motor = self_1655;
    let _e55: Plane = other_1427;
    let _e67: Motor = self_1655;
    let _e71: Plane = other_1427;
    let _e83: Motor = self_1655;
    let _e87: Plane = other_1427;
    let _e99: Motor = self_1655;
    let _e103: Plane = other_1427;
    return Flector((((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e35.g1_.x) * _e39.g0_.zzyz) * vec4<f32>(0.0, -(1.0), 1.0, 0.0))) + ((vec4<f32>(_e51.g1_.y) * _e55.g0_.zzxz) * vec4<f32>(1.0, 0.0, -(1.0), 0.0))) + ((vec4<f32>(_e67.g1_.z) * _e71.g0_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))) + ((vec4<f32>(_e83.g0_.x) * _e87.g0_.wxxx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))), (vec4<f32>(_e99.g0_.w) * _e103.g0_));
}

fn motor_plane_meet(self_1656: Motor, other_1428: Plane) -> Flector {
    var self_1657: Motor;
    var other_1429: Plane;

    self_1657 = self_1656;
    other_1429 = other_1428;
    let _e4: Motor = self_1657;
    let _e8: Plane = other_1429;
    let _e19: Motor = self_1657;
    let _e23: Plane = other_1429;
    let _e35: Motor = self_1657;
    let _e39: Plane = other_1429;
    let _e51: Motor = self_1657;
    let _e55: Plane = other_1429;
    let _e67: Motor = self_1657;
    let _e71: Plane = other_1429;
    let _e83: Motor = self_1657;
    let _e87: Plane = other_1429;
    let _e99: Motor = self_1657;
    let _e103: Plane = other_1429;
    return Flector((((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e35.g1_.x) * _e39.g0_.zzyz) * vec4<f32>(0.0, -(1.0), 1.0, 0.0))) + ((vec4<f32>(_e51.g1_.y) * _e55.g0_.zzxz) * vec4<f32>(1.0, 0.0, -(1.0), 0.0))) + ((vec4<f32>(_e67.g1_.z) * _e71.g0_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))) + ((vec4<f32>(_e83.g0_.x) * _e87.g0_.wxxx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))), (vec4<f32>(_e99.g0_.w) * _e103.g0_));
}

fn motor_plane_inner_product(self_1658: Motor, other_1430: Plane) -> Point {
    var self_1659: Motor;
    var other_1431: Plane;

    self_1659 = self_1658;
    other_1431 = other_1430;
    let _e4: Motor = self_1659;
    let _e8: Plane = other_1431;
    let _e19: Motor = self_1659;
    let _e23: Plane = other_1431;
    let _e35: Motor = self_1659;
    let _e39: Plane = other_1431;
    let _e51: Motor = self_1659;
    let _e54: Plane = other_1431;
    return Point((((((vec4<f32>(_e4.g1_.x) * _e8.g0_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0))) + ((vec4<f32>(_e19.g1_.y) * _e23.g0_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0)))) + ((vec4<f32>(_e35.g1_.z) * _e39.g0_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((_e51.g0_.xxxw * _e54.g0_.xxxw) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))));
}

fn motor_plane_geometric_anti_product(self_1660: Motor, other_1432: Plane) -> Flector {
    var self_1661: Motor;
    var other_1433: Plane;

    self_1661 = self_1660;
    other_1433 = other_1432;
    let _e4: Motor = self_1661;
    let _e8: Plane = other_1433;
    let _e19: Motor = self_1661;
    let _e23: Plane = other_1433;
    let _e35: Motor = self_1661;
    let _e39: Plane = other_1433;
    let _e51: Motor = self_1661;
    let _e55: Plane = other_1433;
    let _e67: Motor = self_1661;
    let _e71: Plane = other_1433;
    let _e83: Motor = self_1661;
    let _e87: Plane = other_1433;
    let _e99: Motor = self_1661;
    let _e103: Plane = other_1433;
    let _e114: Motor = self_1661;
    let _e118: Plane = other_1433;
    let _e130: Motor = self_1661;
    let _e134: Plane = other_1433;
    let _e138: Motor = self_1661;
    let _e142: Plane = other_1433;
    let _e155: Motor = self_1661;
    let _e159: Plane = other_1433;
    let _e172: Motor = self_1661;
    let _e176: Plane = other_1433;
    let _e189: Motor = self_1661;
    let _e193: Plane = other_1433;
    return Flector((((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e35.g1_.x) * _e39.g0_.zzyz) * vec4<f32>(0.0, -(1.0), 1.0, 0.0))) + ((vec4<f32>(_e51.g1_.y) * _e55.g0_.zzxz) * vec4<f32>(1.0, 0.0, -(1.0), 0.0))) + ((vec4<f32>(_e67.g1_.z) * _e71.g0_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))) + ((vec4<f32>(_e83.g0_.x) * _e87.g0_.wxxx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))), ((((((((vec4<f32>(_e99.g0_.y) * _e103.g0_.zzxz) * vec4<f32>(1.0, 0.0, -(1.0), 0.0)) + ((vec4<f32>(_e114.g0_.z) * _e118.g0_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))) + (vec4<f32>(_e130.g0_.w) * _e134.g0_)) + ((vec4<f32>(_e138.g1_.x) * vec4<f32>(_e142.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e155.g1_.y) * vec4<f32>(_e159.g0_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e172.g1_.z) * vec4<f32>(_e176.g0_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e189.g0_.x) * _e193.g0_.xzyx) * vec4<f32>(0.0, -(1.0), 1.0, 0.0))));
}

fn motor_plane_inner_anti_product(self_1662: Motor, other_1434: Plane) -> Plane {
    var self_1663: Motor;
    var other_1435: Plane;

    self_1663 = self_1662;
    other_1435 = other_1434;
    let _e4: Motor = self_1663;
    let _e8: Plane = other_1435;
    let _e19: Motor = self_1663;
    let _e23: Plane = other_1435;
    let _e35: Motor = self_1663;
    let _e39: Plane = other_1435;
    let _e43: Motor = self_1663;
    let _e47: Plane = other_1435;
    let _e60: Motor = self_1663;
    let _e64: Plane = other_1435;
    let _e77: Motor = self_1663;
    let _e81: Plane = other_1435;
    let _e94: Motor = self_1663;
    let _e98: Plane = other_1435;
    return Plane(((((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.zzxz) * vec4<f32>(1.0, 0.0, -(1.0), 0.0)) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))) + (vec4<f32>(_e35.g0_.w) * _e39.g0_)) + ((vec4<f32>(_e43.g1_.x) * vec4<f32>(_e47.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e60.g1_.y) * vec4<f32>(_e64.g0_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e77.g1_.z) * vec4<f32>(_e81.g0_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e94.g0_.x) * _e98.g0_.xzyx) * vec4<f32>(0.0, -(1.0), 1.0, 0.0))));
}

fn motor_plane_left_contraction(self_1664: Motor, other_1436: Plane) -> Point {
    var self_1665: Motor;
    var other_1437: Plane;

    self_1665 = self_1664;
    other_1437 = other_1436;
    let _e4: Motor = self_1665;
    let _e8: Plane = other_1437;
    let _e19: Motor = self_1665;
    let _e23: Plane = other_1437;
    let _e35: Motor = self_1665;
    let _e39: Plane = other_1437;
    return Point(((((vec4<f32>(_e4.g1_.y) * _e8.g0_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e19.g1_.z) * _e23.g0_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e35.g1_.x) * _e39.g0_.wxxx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))));
}

fn motor_plane_left_anti_contraction(self_1666: Motor, other_1438: Plane) -> Plane {
    var self_1667: Motor;
    var other_1439: Plane;

    self_1667 = self_1666;
    other_1439 = other_1438;
    let _e4: Motor = self_1667;
    let _e8: Plane = other_1439;
    return Plane((vec4<f32>(_e4.g0_.w) * _e8.g0_));
}

fn motor_plane_right_anti_contraction(self_1668: Motor, other_1440: Plane) -> Plane {
    var self_1669: Motor;
    var other_1441: Plane;

    self_1669 = self_1668;
    other_1441 = other_1440;
    let _e4: Motor = self_1669;
    let _e8: Plane = other_1441;
    let _e19: Motor = self_1669;
    let _e23: Plane = other_1441;
    let _e35: Motor = self_1669;
    let _e39: Plane = other_1441;
    let _e52: Motor = self_1669;
    let _e56: Plane = other_1441;
    let _e69: Motor = self_1669;
    let _e73: Plane = other_1441;
    let _e86: Motor = self_1669;
    let _e90: Plane = other_1441;
    return Plane((((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.zzxz) * vec4<f32>(1.0, 0.0, -(1.0), 0.0)) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))) + ((vec4<f32>(_e35.g1_.x) * vec4<f32>(_e39.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e52.g1_.y) * vec4<f32>(_e56.g0_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e69.g1_.z) * vec4<f32>(_e73.g0_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e86.g0_.x) * _e90.g0_.xzyx) * vec4<f32>(0.0, -(1.0), 1.0, 0.0))));
}

fn motor_motor_add(self_1670: Motor, other_1442: Motor) -> Motor {
    var self_1671: Motor;
    var other_1443: Motor;

    self_1671 = self_1670;
    other_1443 = other_1442;
    let _e4: Motor = self_1671;
    let _e6: Motor = other_1443;
    let _e9: Motor = self_1671;
    let _e11: Motor = other_1443;
    return Motor((_e4.g0_ + _e6.g0_), (_e9.g1_ + _e11.g1_));
}

fn motor_motor_sub(self_1672: Motor, other_1444: Motor) -> Motor {
    var self_1673: Motor;
    var other_1445: Motor;

    self_1673 = self_1672;
    other_1445 = other_1444;
    let _e4: Motor = self_1673;
    let _e6: Motor = other_1445;
    let _e9: Motor = self_1673;
    let _e11: Motor = other_1445;
    return Motor((_e4.g0_ - _e6.g0_), (_e9.g1_ - _e11.g1_));
}

fn motor_motor_mul(self_1674: Motor, other_1446: Motor) -> Motor {
    var self_1675: Motor;
    var other_1447: Motor;

    self_1675 = self_1674;
    other_1447 = other_1446;
    let _e4: Motor = self_1675;
    let _e6: Motor = other_1447;
    let _e9: Motor = self_1675;
    let _e11: Motor = other_1447;
    return Motor((_e4.g0_ * _e6.g0_), (_e9.g1_ * _e11.g1_));
}

fn motor_motor_div(self_1676: Motor, other_1448: Motor) -> Motor {
    var self_1677: Motor;
    var other_1449: Motor;

    self_1677 = self_1676;
    other_1449 = other_1448;
    let _e4: Motor = self_1677;
    let _e7: Motor = self_1677;
    let _e10: Motor = self_1677;
    let _e13: Motor = self_1677;
    let _e23: Motor = other_1449;
    let _e26: Motor = other_1449;
    let _e29: Motor = other_1449;
    let _e32: Motor = other_1449;
    let _e43: Motor = self_1677;
    let _e46: Motor = self_1677;
    let _e49: Motor = self_1677;
    let _e58: Motor = other_1449;
    let _e61: Motor = other_1449;
    let _e64: Motor = other_1449;
    return Motor((((vec4<f32>(_e4.g0_.x, _e7.g0_.y, _e10.g0_.z, _e13.g0_.w) * vec4<f32>(1.0, 1.0, 1.0, 1.0)) / vec4<f32>(_e23.g0_.x, _e26.g0_.y, _e29.g0_.z, _e32.g0_.w)) * vec4<f32>(1.0, 1.0, 1.0, 1.0)), (((vec3<f32>(_e43.g1_.x, _e46.g1_.y, _e49.g1_.z) * vec3<f32>(1.0, 1.0, 1.0)) / vec3<f32>(_e58.g1_.x, _e61.g1_.y, _e64.g1_.z)) * vec3<f32>(1.0, 1.0, 1.0)));
}

fn motor_motor_outer_product(self_1678: Motor, other_1450: Motor) -> AntiScalar {
    var self_1679: Motor;
    var other_1451: Motor;

    self_1679 = self_1678;
    other_1451 = other_1450;
    let _e5: Motor = self_1679;
    let _e8: Motor = other_1451;
    let _e13: Motor = self_1679;
    let _e16: Motor = other_1451;
    let _e21: Motor = self_1679;
    let _e24: Motor = other_1451;
    let _e29: Motor = self_1679;
    let _e32: Motor = other_1451;
    let _e37: Motor = self_1679;
    let _e40: Motor = other_1451;
    let _e45: Motor = self_1679;
    let _e48: Motor = other_1451;
    return AntiScalar(((((((0.0 - (_e5.g0_.x * _e8.g1_.x)) - (_e13.g0_.y * _e16.g1_.y)) - (_e21.g0_.z * _e24.g1_.z)) - (_e29.g1_.x * _e32.g0_.x)) - (_e37.g1_.y * _e40.g0_.y)) - (_e45.g1_.z * _e48.g0_.z)));
}

fn motor_motor_wedge(self_1680: Motor, other_1452: Motor) -> AntiScalar {
    var self_1681: Motor;
    var other_1453: Motor;

    self_1681 = self_1680;
    other_1453 = other_1452;
    let _e5: Motor = self_1681;
    let _e8: Motor = other_1453;
    let _e13: Motor = self_1681;
    let _e16: Motor = other_1453;
    let _e21: Motor = self_1681;
    let _e24: Motor = other_1453;
    let _e29: Motor = self_1681;
    let _e32: Motor = other_1453;
    let _e37: Motor = self_1681;
    let _e40: Motor = other_1453;
    let _e45: Motor = self_1681;
    let _e48: Motor = other_1453;
    return AntiScalar(((((((0.0 - (_e5.g0_.x * _e8.g1_.x)) - (_e13.g0_.y * _e16.g1_.y)) - (_e21.g0_.z * _e24.g1_.z)) - (_e29.g1_.x * _e32.g0_.x)) - (_e37.g1_.y * _e40.g0_.y)) - (_e45.g1_.z * _e48.g0_.z)));
}

fn motor_motor_join(self_1682: Motor, other_1454: Motor) -> AntiScalar {
    var self_1683: Motor;
    var other_1455: Motor;

    self_1683 = self_1682;
    other_1455 = other_1454;
    let _e5: Motor = self_1683;
    let _e8: Motor = other_1455;
    let _e13: Motor = self_1683;
    let _e16: Motor = other_1455;
    let _e21: Motor = self_1683;
    let _e24: Motor = other_1455;
    let _e29: Motor = self_1683;
    let _e32: Motor = other_1455;
    let _e37: Motor = self_1683;
    let _e40: Motor = other_1455;
    let _e45: Motor = self_1683;
    let _e48: Motor = other_1455;
    return AntiScalar(((((((0.0 - (_e5.g0_.x * _e8.g1_.x)) - (_e13.g0_.y * _e16.g1_.y)) - (_e21.g0_.z * _e24.g1_.z)) - (_e29.g1_.x * _e32.g0_.x)) - (_e37.g1_.y * _e40.g0_.y)) - (_e45.g1_.z * _e48.g0_.z)));
}

fn motor_motor_inner_anti_product(self_1684: Motor, other_1456: Motor) -> Motor {
    var self_1685: Motor;
    var other_1457: Motor;

    self_1685 = self_1684;
    other_1457 = other_1456;
    let _e4: Motor = self_1685;
    let _e8: Motor = other_1457;
    let _e19: Motor = self_1685;
    let _e23: Motor = other_1457;
    let _e35: Motor = self_1685;
    let _e39: Motor = other_1457;
    let _e43: Motor = self_1685;
    let _e47: Motor = other_1457;
    let _e59: Motor = self_1685;
    let _e63: Motor = other_1457;
    let _e66: Motor = self_1685;
    let _e68: Motor = other_1457;
    return Motor((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + (vec4<f32>(_e35.g0_.w) * _e39.g0_)) + ((vec4<f32>(_e43.g0_.x) * _e47.g0_.wxxx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))), ((vec3<f32>(_e59.g0_.w) * _e63.g1_) + (_e66.g1_ * vec3<f32>(_e68.g0_.w))));
}

fn motor_motor_left_anti_contraction(self_1686: Motor, other_1458: Motor) -> Motor {
    var self_1687: Motor;
    var other_1459: Motor;

    self_1687 = self_1686;
    other_1459 = other_1458;
    let _e4: Motor = self_1687;
    let _e8: Motor = other_1459;
    let _e20: Motor = self_1687;
    let _e24: Motor = other_1459;
    let _e37: Motor = self_1687;
    let _e41: Motor = other_1459;
    let _e45: Motor = self_1687;
    let _e49: Motor = other_1459;
    let _e62: Motor = self_1687;
    let _e66: Motor = other_1459;
    return Motor((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0))) + ((vec4<f32>(_e20.g0_.z) * vec4<f32>(_e24.g0_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + (vec4<f32>(_e37.g0_.w) * _e41.g0_)) + ((vec4<f32>(_e45.g0_.x) * vec4<f32>(_e49.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))), (vec3<f32>(_e62.g0_.w) * _e66.g1_));
}

fn motor_motor_right_anti_contraction(self_1688: Motor, other_1460: Motor) -> Motor {
    var self_1689: Motor;
    var other_1461: Motor;

    self_1689 = self_1688;
    other_1461 = other_1460;
    let _e4: Motor = self_1689;
    let _e8: Motor = other_1461;
    let _e19: Motor = self_1689;
    let _e23: Motor = other_1461;
    let _e35: Motor = self_1689;
    let _e39: Motor = other_1461;
    let _e51: Motor = self_1689;
    let _e55: Motor = other_1461;
    let _e67: Motor = self_1689;
    let _e69: Motor = other_1461;
    return Motor((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e35.g0_.w) * vec4<f32>(_e39.g0_.w)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e51.g0_.x) * _e55.g0_.wxxx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))), (_e67.g1_ * vec3<f32>(_e69.g0_.w)));
}

fn motor_motor_scalar_product(self_1690: Motor, other_1462: Motor) -> Scalar {
    var self_1691: Motor;
    var other_1463: Motor;

    self_1691 = self_1690;
    other_1463 = other_1462;
    let _e5: Motor = self_1691;
    let _e8: Motor = other_1463;
    let _e13: Motor = self_1691;
    let _e16: Motor = other_1463;
    let _e21: Motor = self_1691;
    let _e24: Motor = other_1463;
    return Scalar((((0.0 - (_e5.g1_.x * _e8.g1_.x)) - (_e13.g1_.y * _e16.g1_.y)) - (_e21.g1_.z * _e24.g1_.z)));
}

fn motor_motor_dot(self_1692: Motor, other_1464: Motor) -> Scalar {
    var self_1693: Motor;
    var other_1465: Motor;

    self_1693 = self_1692;
    other_1465 = other_1464;
    let _e5: Motor = self_1693;
    let _e8: Motor = other_1465;
    let _e13: Motor = self_1693;
    let _e16: Motor = other_1465;
    let _e21: Motor = self_1693;
    let _e24: Motor = other_1465;
    return Scalar((((0.0 - (_e5.g1_.x * _e8.g1_.x)) - (_e13.g1_.y * _e16.g1_.y)) - (_e21.g1_.z * _e24.g1_.z)));
}

fn motor_motor_anti_scalar_product(self_1694: Motor, other_1466: Motor) -> AntiScalar {
    var self_1695: Motor;
    var other_1467: Motor;

    self_1695 = self_1694;
    other_1467 = other_1466;
    let _e5: Motor = self_1695;
    let _e8: Motor = other_1467;
    let _e13: Motor = self_1695;
    let _e16: Motor = other_1467;
    let _e21: Motor = self_1695;
    let _e24: Motor = other_1467;
    let _e29: Motor = self_1695;
    let _e32: Motor = other_1467;
    return AntiScalar(((((0.0 - (_e5.g0_.x * _e8.g0_.x)) - (_e13.g0_.y * _e16.g0_.y)) - (_e21.g0_.z * _e24.g0_.z)) + (_e29.g0_.w * _e32.g0_.w)));
}

fn motor_motor_anti_dot(self_1696: Motor, other_1468: Motor) -> AntiScalar {
    var self_1697: Motor;
    var other_1469: Motor;

    self_1697 = self_1696;
    other_1469 = other_1468;
    let _e5: Motor = self_1697;
    let _e8: Motor = other_1469;
    let _e13: Motor = self_1697;
    let _e16: Motor = other_1469;
    let _e21: Motor = self_1697;
    let _e24: Motor = other_1469;
    let _e29: Motor = self_1697;
    let _e32: Motor = other_1469;
    return AntiScalar(((((0.0 - (_e5.g0_.x * _e8.g0_.x)) - (_e13.g0_.y * _e16.g0_.y)) - (_e21.g0_.z * _e24.g0_.z)) + (_e29.g0_.w * _e32.g0_.w)));
}

fn motor_rotor_into(self_1698: Motor) -> Rotor {
    var self_1699: Motor;

    self_1699 = self_1698;
    let _e2: Motor = self_1699;
    return Rotor(_e2.g0_);
}

fn motor_rotor_add(self_1700: Motor, other_1470: Rotor) -> Motor {
    var self_1701: Motor;
    var other_1471: Rotor;

    self_1701 = self_1700;
    other_1471 = other_1470;
    let _e4: Motor = self_1701;
    let _e6: Rotor = other_1471;
    let _e9: Motor = self_1701;
    return Motor((_e4.g0_ + _e6.g0_), _e9.g1_);
}

fn motor_rotor_sub(self_1702: Motor, other_1472: Rotor) -> Motor {
    var self_1703: Motor;
    var other_1473: Rotor;

    self_1703 = self_1702;
    other_1473 = other_1472;
    let _e4: Motor = self_1703;
    let _e6: Rotor = other_1473;
    let _e9: Motor = self_1703;
    return Motor((_e4.g0_ - _e6.g0_), _e9.g1_);
}

fn motor_rotor_geometric_product(self_1704: Motor, other_1474: Rotor) -> Rotor {
    var self_1705: Motor;
    var other_1475: Rotor;

    self_1705 = self_1704;
    other_1475 = other_1474;
    let _e4: Motor = self_1705;
    let _e8: Rotor = other_1475;
    let _e20: Motor = self_1705;
    let _e24: Rotor = other_1475;
    let _e37: Motor = self_1705;
    let _e41: Rotor = other_1475;
    return Rotor(((((vec4<f32>(_e4.g1_.x) * _e8.g0_.wzyx) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0))) + ((vec4<f32>(_e20.g1_.y) * _e24.g0_.zwxy) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e37.g1_.z) * _e41.g0_.yxwz) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))));
}

fn motor_rotor_outer_product(self_1706: Motor, other_1476: Rotor) -> AntiScalar {
    var self_1707: Motor;
    var other_1477: Rotor;

    self_1707 = self_1706;
    other_1477 = other_1476;
    let _e5: Motor = self_1707;
    let _e8: Rotor = other_1477;
    let _e13: Motor = self_1707;
    let _e16: Rotor = other_1477;
    let _e21: Motor = self_1707;
    let _e24: Rotor = other_1477;
    return AntiScalar((((0.0 - (_e5.g1_.x * _e8.g0_.x)) - (_e13.g1_.y * _e16.g0_.y)) - (_e21.g1_.z * _e24.g0_.z)));
}

fn motor_rotor_wedge(self_1708: Motor, other_1478: Rotor) -> AntiScalar {
    var self_1709: Motor;
    var other_1479: Rotor;

    self_1709 = self_1708;
    other_1479 = other_1478;
    let _e5: Motor = self_1709;
    let _e8: Rotor = other_1479;
    let _e13: Motor = self_1709;
    let _e16: Rotor = other_1479;
    let _e21: Motor = self_1709;
    let _e24: Rotor = other_1479;
    return AntiScalar((((0.0 - (_e5.g1_.x * _e8.g0_.x)) - (_e13.g1_.y * _e16.g0_.y)) - (_e21.g1_.z * _e24.g0_.z)));
}

fn motor_rotor_join(self_1710: Motor, other_1480: Rotor) -> AntiScalar {
    var self_1711: Motor;
    var other_1481: Rotor;

    self_1711 = self_1710;
    other_1481 = other_1480;
    let _e5: Motor = self_1711;
    let _e8: Rotor = other_1481;
    let _e13: Motor = self_1711;
    let _e16: Rotor = other_1481;
    let _e21: Motor = self_1711;
    let _e24: Rotor = other_1481;
    return AntiScalar((((0.0 - (_e5.g1_.x * _e8.g0_.x)) - (_e13.g1_.y * _e16.g0_.y)) - (_e21.g1_.z * _e24.g0_.z)));
}

fn motor_rotor_inner_anti_product(self_1712: Motor, other_1482: Rotor) -> Motor {
    var self_1713: Motor;
    var other_1483: Rotor;

    self_1713 = self_1712;
    other_1483 = other_1482;
    let _e4: Motor = self_1713;
    let _e8: Rotor = other_1483;
    let _e19: Motor = self_1713;
    let _e23: Rotor = other_1483;
    let _e35: Motor = self_1713;
    let _e39: Rotor = other_1483;
    let _e43: Motor = self_1713;
    let _e47: Rotor = other_1483;
    let _e59: Motor = self_1713;
    let _e61: Rotor = other_1483;
    return Motor((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + (vec4<f32>(_e35.g0_.w) * _e39.g0_)) + ((vec4<f32>(_e43.g0_.x) * _e47.g0_.wxxx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))), (_e59.g1_ * vec3<f32>(_e61.g0_.w)));
}

fn motor_rotor_left_anti_contraction(self_1714: Motor, other_1484: Rotor) -> Rotor {
    var self_1715: Motor;
    var other_1485: Rotor;

    self_1715 = self_1714;
    other_1485 = other_1484;
    let _e4: Motor = self_1715;
    let _e8: Rotor = other_1485;
    let _e20: Motor = self_1715;
    let _e24: Rotor = other_1485;
    let _e37: Motor = self_1715;
    let _e41: Rotor = other_1485;
    let _e45: Motor = self_1715;
    let _e49: Rotor = other_1485;
    return Rotor((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0))) + ((vec4<f32>(_e20.g0_.z) * vec4<f32>(_e24.g0_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + (vec4<f32>(_e37.g0_.w) * _e41.g0_)) + ((vec4<f32>(_e45.g0_.x) * vec4<f32>(_e49.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))));
}

fn motor_rotor_right_anti_contraction(self_1716: Motor, other_1486: Rotor) -> Motor {
    var self_1717: Motor;
    var other_1487: Rotor;

    self_1717 = self_1716;
    other_1487 = other_1486;
    let _e4: Motor = self_1717;
    let _e8: Rotor = other_1487;
    let _e19: Motor = self_1717;
    let _e23: Rotor = other_1487;
    let _e35: Motor = self_1717;
    let _e39: Rotor = other_1487;
    let _e51: Motor = self_1717;
    let _e55: Rotor = other_1487;
    let _e67: Motor = self_1717;
    let _e69: Rotor = other_1487;
    return Motor((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e35.g0_.w) * vec4<f32>(_e39.g0_.w)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e51.g0_.x) * _e55.g0_.wxxx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))), (_e67.g1_ * vec3<f32>(_e69.g0_.w)));
}

fn motor_rotor_anti_scalar_product(self_1718: Motor, other_1488: Rotor) -> AntiScalar {
    var self_1719: Motor;
    var other_1489: Rotor;

    self_1719 = self_1718;
    other_1489 = other_1488;
    let _e5: Motor = self_1719;
    let _e8: Rotor = other_1489;
    let _e13: Motor = self_1719;
    let _e16: Rotor = other_1489;
    let _e21: Motor = self_1719;
    let _e24: Rotor = other_1489;
    let _e29: Motor = self_1719;
    let _e32: Rotor = other_1489;
    return AntiScalar(((((0.0 - (_e5.g0_.x * _e8.g0_.x)) - (_e13.g0_.y * _e16.g0_.y)) - (_e21.g0_.z * _e24.g0_.z)) + (_e29.g0_.w * _e32.g0_.w)));
}

fn motor_rotor_anti_dot(self_1720: Motor, other_1490: Rotor) -> AntiScalar {
    var self_1721: Motor;
    var other_1491: Rotor;

    self_1721 = self_1720;
    other_1491 = other_1490;
    let _e5: Motor = self_1721;
    let _e8: Rotor = other_1491;
    let _e13: Motor = self_1721;
    let _e16: Rotor = other_1491;
    let _e21: Motor = self_1721;
    let _e24: Rotor = other_1491;
    let _e29: Motor = self_1721;
    let _e32: Rotor = other_1491;
    return AntiScalar(((((0.0 - (_e5.g0_.x * _e8.g0_.x)) - (_e13.g0_.y * _e16.g0_.y)) - (_e21.g0_.z * _e24.g0_.z)) + (_e29.g0_.w * _e32.g0_.w)));
}

fn motor_translator_into(self_1722: Motor) -> Translator {
    var self_1723: Motor;

    self_1723 = self_1722;
    let _e2: Motor = self_1723;
    let _e5: Motor = self_1723;
    let _e8: Motor = self_1723;
    let _e11: Motor = self_1723;
    return Translator(vec4<f32>(_e2.g1_.x, _e5.g1_.y, _e8.g1_.z, _e11.g0_.w));
}

fn motor_translator_add(self_1724: Motor, other_1492: Translator) -> Motor {
    var self_1725: Motor;
    var other_1493: Translator;

    self_1725 = self_1724;
    other_1493 = other_1492;
    let _e4: Motor = self_1725;
    let _e6: Translator = other_1493;
    let _e16: Motor = self_1725;
    let _e18: Translator = other_1493;
    let _e21: Translator = other_1493;
    let _e24: Translator = other_1493;
    return Motor((_e4.g0_ + (_e6.g0_.xxxw * vec4<f32>(0.0, 0.0, 0.0, 1.0))), (_e16.g1_ + vec3<f32>(_e18.g0_.x, _e21.g0_.y, _e24.g0_.z)));
}

fn motor_translator_sub(self_1726: Motor, other_1494: Translator) -> Motor {
    var self_1727: Motor;
    var other_1495: Translator;

    self_1727 = self_1726;
    other_1495 = other_1494;
    let _e4: Motor = self_1727;
    let _e6: Translator = other_1495;
    let _e16: Motor = self_1727;
    let _e18: Translator = other_1495;
    let _e21: Translator = other_1495;
    let _e24: Translator = other_1495;
    return Motor((_e4.g0_ - (_e6.g0_.xxxw * vec4<f32>(0.0, 0.0, 0.0, 1.0))), (_e16.g1_ - vec3<f32>(_e18.g0_.x, _e21.g0_.y, _e24.g0_.z)));
}

fn motor_translator_outer_product(self_1728: Motor, other_1496: Translator) -> AntiScalar {
    var self_1729: Motor;
    var other_1497: Translator;

    self_1729 = self_1728;
    other_1497 = other_1496;
    let _e5: Motor = self_1729;
    let _e8: Translator = other_1497;
    let _e13: Motor = self_1729;
    let _e16: Translator = other_1497;
    let _e21: Motor = self_1729;
    let _e24: Translator = other_1497;
    return AntiScalar((((0.0 - (_e5.g0_.x * _e8.g0_.x)) - (_e13.g0_.y * _e16.g0_.y)) - (_e21.g0_.z * _e24.g0_.z)));
}

fn motor_translator_wedge(self_1730: Motor, other_1498: Translator) -> AntiScalar {
    var self_1731: Motor;
    var other_1499: Translator;

    self_1731 = self_1730;
    other_1499 = other_1498;
    let _e5: Motor = self_1731;
    let _e8: Translator = other_1499;
    let _e13: Motor = self_1731;
    let _e16: Translator = other_1499;
    let _e21: Motor = self_1731;
    let _e24: Translator = other_1499;
    return AntiScalar((((0.0 - (_e5.g0_.x * _e8.g0_.x)) - (_e13.g0_.y * _e16.g0_.y)) - (_e21.g0_.z * _e24.g0_.z)));
}

fn motor_translator_join(self_1732: Motor, other_1500: Translator) -> AntiScalar {
    var self_1733: Motor;
    var other_1501: Translator;

    self_1733 = self_1732;
    other_1501 = other_1500;
    let _e5: Motor = self_1733;
    let _e8: Translator = other_1501;
    let _e13: Motor = self_1733;
    let _e16: Translator = other_1501;
    let _e21: Motor = self_1733;
    let _e24: Translator = other_1501;
    return AntiScalar((((0.0 - (_e5.g0_.x * _e8.g0_.x)) - (_e13.g0_.y * _e16.g0_.y)) - (_e21.g0_.z * _e24.g0_.z)));
}

fn motor_translator_inner_anti_product(self_1734: Motor, other_1502: Translator) -> Motor {
    var self_1735: Motor;
    var other_1503: Translator;

    self_1735 = self_1734;
    other_1503 = other_1502;
    let _e4: Motor = self_1735;
    let _e6: Translator = other_1503;
    let _e11: Motor = self_1735;
    let _e15: Translator = other_1503;
    let _e18: Translator = other_1503;
    let _e21: Translator = other_1503;
    let _e26: Motor = self_1735;
    let _e28: Translator = other_1503;
    return Motor((_e4.g0_ * vec4<f32>(_e6.g0_.w)), ((vec3<f32>(_e11.g0_.w) * vec3<f32>(_e15.g0_.x, _e18.g0_.y, _e21.g0_.z)) + (_e26.g1_ * vec3<f32>(_e28.g0_.w))));
}

fn motor_translator_left_anti_contraction(self_1736: Motor, other_1504: Translator) -> Translator {
    var self_1737: Motor;
    var other_1505: Translator;

    self_1737 = self_1736;
    other_1505 = other_1504;
    let _e4: Motor = self_1737;
    let _e8: Translator = other_1505;
    return Translator((vec4<f32>(_e4.g0_.w) * _e8.g0_));
}

fn motor_translator_right_anti_contraction(self_1738: Motor, other_1506: Translator) -> Motor {
    var self_1739: Motor;
    var other_1507: Translator;

    self_1739 = self_1738;
    other_1507 = other_1506;
    let _e4: Motor = self_1739;
    let _e6: Translator = other_1507;
    let _e11: Motor = self_1739;
    let _e13: Translator = other_1507;
    return Motor((_e4.g0_ * vec4<f32>(_e6.g0_.w)), (_e11.g1_ * vec3<f32>(_e13.g0_.w)));
}

fn motor_translator_scalar_product(self_1740: Motor, other_1508: Translator) -> Scalar {
    var self_1741: Motor;
    var other_1509: Translator;

    self_1741 = self_1740;
    other_1509 = other_1508;
    let _e5: Motor = self_1741;
    let _e8: Translator = other_1509;
    let _e13: Motor = self_1741;
    let _e16: Translator = other_1509;
    let _e21: Motor = self_1741;
    let _e24: Translator = other_1509;
    return Scalar((((0.0 - (_e5.g1_.x * _e8.g0_.x)) - (_e13.g1_.y * _e16.g0_.y)) - (_e21.g1_.z * _e24.g0_.z)));
}

fn motor_translator_dot(self_1742: Motor, other_1510: Translator) -> Scalar {
    var self_1743: Motor;
    var other_1511: Translator;

    self_1743 = self_1742;
    other_1511 = other_1510;
    let _e5: Motor = self_1743;
    let _e8: Translator = other_1511;
    let _e13: Motor = self_1743;
    let _e16: Translator = other_1511;
    let _e21: Motor = self_1743;
    let _e24: Translator = other_1511;
    return Scalar((((0.0 - (_e5.g1_.x * _e8.g0_.x)) - (_e13.g1_.y * _e16.g0_.y)) - (_e21.g1_.z * _e24.g0_.z)));
}

fn motor_translator_anti_scalar_product(self_1744: Motor, other_1512: Translator) -> AntiScalar {
    var self_1745: Motor;
    var other_1513: Translator;

    self_1745 = self_1744;
    other_1513 = other_1512;
    let _e4: Motor = self_1745;
    let _e7: Translator = other_1513;
    return AntiScalar((_e4.g0_.w * _e7.g0_.w));
}

fn motor_translator_anti_dot(self_1746: Motor, other_1514: Translator) -> AntiScalar {
    var self_1747: Motor;
    var other_1515: Translator;

    self_1747 = self_1746;
    other_1515 = other_1514;
    let _e4: Motor = self_1747;
    let _e7: Translator = other_1515;
    return AntiScalar((_e4.g0_.w * _e7.g0_.w));
}

fn motor_flector_geometric_product(self_1748: Motor, other_1516: Flector) -> Flector {
    var self_1749: Motor;
    var other_1517: Flector;

    self_1749 = self_1748;
    other_1517 = other_1516;
    let _e4: Motor = self_1749;
    let _e8: Flector = other_1517;
    let _e19: Motor = self_1749;
    let _e23: Flector = other_1517;
    let _e35: Motor = self_1749;
    let _e39: Flector = other_1517;
    let _e52: Motor = self_1749;
    let _e56: Flector = other_1517;
    let _e59: Flector = other_1517;
    let _e62: Flector = other_1517;
    let _e65: Flector = other_1517;
    let _e79: Motor = self_1749;
    let _e83: Flector = other_1517;
    let _e86: Flector = other_1517;
    let _e89: Flector = other_1517;
    let _e92: Flector = other_1517;
    let _e106: Motor = self_1749;
    let _e110: Flector = other_1517;
    let _e113: Flector = other_1517;
    let _e116: Flector = other_1517;
    let _e119: Flector = other_1517;
    let _e133: Motor = self_1749;
    let _e137: Flector = other_1517;
    let _e149: Motor = self_1749;
    let _e153: Flector = other_1517;
    let _e156: Flector = other_1517;
    let _e159: Flector = other_1517;
    let _e162: Flector = other_1517;
    let _e175: Motor = self_1749;
    let _e179: Flector = other_1517;
    let _e182: Flector = other_1517;
    let _e185: Flector = other_1517;
    let _e188: Flector = other_1517;
    let _e202: Motor = self_1749;
    let _e206: Flector = other_1517;
    let _e220: Motor = self_1749;
    let _e224: Flector = other_1517;
    let _e227: Flector = other_1517;
    let _e230: Flector = other_1517;
    let _e233: Flector = other_1517;
    let _e247: Motor = self_1749;
    let _e251: Flector = other_1517;
    let _e254: Flector = other_1517;
    let _e257: Flector = other_1517;
    let _e260: Flector = other_1517;
    let _e274: Motor = self_1749;
    let _e278: Flector = other_1517;
    let _e281: Flector = other_1517;
    let _e284: Flector = other_1517;
    let _e287: Flector = other_1517;
    let _e301: Motor = self_1749;
    let _e305: Flector = other_1517;
    let _e308: Flector = other_1517;
    let _e311: Flector = other_1517;
    let _e314: Flector = other_1517;
    return Flector(((((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y)) * vec4<f32>(0.0, 0.0, 0.0, 1.0)) + ((vec4<f32>(_e19.g0_.z) * vec4<f32>(_e23.g0_.z)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e35.g0_.w) * vec4<f32>(_e39.g1_.w)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e52.g1_.x) * vec4<f32>(_e56.g1_.w, _e59.g0_.z, _e62.g0_.y, _e65.g1_.x)) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e79.g1_.y) * vec4<f32>(_e83.g0_.z, _e86.g1_.w, _e89.g0_.x, _e92.g1_.y)) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e106.g1_.z) * vec4<f32>(_e110.g0_.y, _e113.g0_.x, _e116.g1_.w, _e119.g1_.z)) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e133.g0_.x) * vec4<f32>(_e137.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))), ((((((((vec4<f32>(_e149.g0_.y) * vec4<f32>(_e153.g0_.z, _e156.g1_.w, _e159.g0_.x, _e162.g0_.z)) * vec4<f32>(1.0, -(1.0), -(1.0), 0.0)) + ((vec4<f32>(_e175.g0_.z) * vec4<f32>(_e179.g0_.y, _e182.g0_.x, _e185.g1_.w, _e188.g0_.y)) * vec4<f32>(-(1.0), 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e202.g0_.w) * _e206.g0_.xyzx) * vec4<f32>(-(1.0), -(1.0), -(1.0), 0.0))) + ((vec4<f32>(_e220.g1_.x) * vec4<f32>(_e224.g0_.w, _e227.g1_.z, _e230.g1_.y, _e233.g0_.x)) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e247.g1_.y) * vec4<f32>(_e251.g1_.z, _e254.g0_.w, _e257.g1_.x, _e260.g0_.y)) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e274.g1_.z) * vec4<f32>(_e278.g1_.y, _e281.g1_.x, _e284.g0_.w, _e287.g0_.z)) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e301.g0_.x) * vec4<f32>(_e305.g1_.w, _e308.g0_.z, _e311.g0_.y, _e314.g1_.x)) * vec4<f32>(-(1.0), -(1.0), 1.0, 0.0))));
}

fn motor_flector_regressive_product(self_1750: Motor, other_1518: Flector) -> Flector {
    var self_1751: Motor;
    var other_1519: Flector;

    self_1751 = self_1750;
    other_1519 = other_1518;
    let _e4: Motor = self_1751;
    let _e8: Flector = other_1519;
    let _e19: Motor = self_1751;
    let _e23: Flector = other_1519;
    let _e35: Motor = self_1751;
    let _e39: Flector = other_1519;
    let _e43: Motor = self_1751;
    let _e47: Flector = other_1519;
    let _e59: Motor = self_1751;
    let _e63: Flector = other_1519;
    let _e75: Motor = self_1751;
    let _e79: Flector = other_1519;
    let _e91: Motor = self_1751;
    let _e95: Flector = other_1519;
    let _e107: Motor = self_1751;
    let _e111: Flector = other_1519;
    return Flector(((((((((vec4<f32>(_e4.g0_.y) * _e8.g1_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e19.g0_.z) * _e23.g1_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + (vec4<f32>(_e35.g0_.w) * _e39.g0_)) + ((vec4<f32>(_e43.g1_.x) * _e47.g1_.zzyz) * vec4<f32>(0.0, -(1.0), 1.0, 0.0))) + ((vec4<f32>(_e59.g1_.y) * _e63.g1_.zzxz) * vec4<f32>(1.0, 0.0, -(1.0), 0.0))) + ((vec4<f32>(_e75.g1_.z) * _e79.g1_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))) + ((vec4<f32>(_e91.g0_.x) * _e95.g1_.wxxx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))), (vec4<f32>(_e107.g0_.w) * _e111.g1_));
}

fn motor_flector_anti_wedge(self_1752: Motor, other_1520: Flector) -> Flector {
    var self_1753: Motor;
    var other_1521: Flector;

    self_1753 = self_1752;
    other_1521 = other_1520;
    let _e4: Motor = self_1753;
    let _e8: Flector = other_1521;
    let _e19: Motor = self_1753;
    let _e23: Flector = other_1521;
    let _e35: Motor = self_1753;
    let _e39: Flector = other_1521;
    let _e43: Motor = self_1753;
    let _e47: Flector = other_1521;
    let _e59: Motor = self_1753;
    let _e63: Flector = other_1521;
    let _e75: Motor = self_1753;
    let _e79: Flector = other_1521;
    let _e91: Motor = self_1753;
    let _e95: Flector = other_1521;
    let _e107: Motor = self_1753;
    let _e111: Flector = other_1521;
    return Flector(((((((((vec4<f32>(_e4.g0_.y) * _e8.g1_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e19.g0_.z) * _e23.g1_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + (vec4<f32>(_e35.g0_.w) * _e39.g0_)) + ((vec4<f32>(_e43.g1_.x) * _e47.g1_.zzyz) * vec4<f32>(0.0, -(1.0), 1.0, 0.0))) + ((vec4<f32>(_e59.g1_.y) * _e63.g1_.zzxz) * vec4<f32>(1.0, 0.0, -(1.0), 0.0))) + ((vec4<f32>(_e75.g1_.z) * _e79.g1_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))) + ((vec4<f32>(_e91.g0_.x) * _e95.g1_.wxxx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))), (vec4<f32>(_e107.g0_.w) * _e111.g1_));
}

fn motor_flector_meet(self_1754: Motor, other_1522: Flector) -> Flector {
    var self_1755: Motor;
    var other_1523: Flector;

    self_1755 = self_1754;
    other_1523 = other_1522;
    let _e4: Motor = self_1755;
    let _e8: Flector = other_1523;
    let _e19: Motor = self_1755;
    let _e23: Flector = other_1523;
    let _e35: Motor = self_1755;
    let _e39: Flector = other_1523;
    let _e43: Motor = self_1755;
    let _e47: Flector = other_1523;
    let _e59: Motor = self_1755;
    let _e63: Flector = other_1523;
    let _e75: Motor = self_1755;
    let _e79: Flector = other_1523;
    let _e91: Motor = self_1755;
    let _e95: Flector = other_1523;
    let _e107: Motor = self_1755;
    let _e111: Flector = other_1523;
    return Flector(((((((((vec4<f32>(_e4.g0_.y) * _e8.g1_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e19.g0_.z) * _e23.g1_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + (vec4<f32>(_e35.g0_.w) * _e39.g0_)) + ((vec4<f32>(_e43.g1_.x) * _e47.g1_.zzyz) * vec4<f32>(0.0, -(1.0), 1.0, 0.0))) + ((vec4<f32>(_e59.g1_.y) * _e63.g1_.zzxz) * vec4<f32>(1.0, 0.0, -(1.0), 0.0))) + ((vec4<f32>(_e75.g1_.z) * _e79.g1_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))) + ((vec4<f32>(_e91.g0_.x) * _e95.g1_.wxxx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))), (vec4<f32>(_e107.g0_.w) * _e111.g1_));
}

fn motor_flector_outer_product(self_1756: Motor, other_1524: Flector) -> Plane {
    var self_1757: Motor;
    var other_1525: Flector;

    self_1757 = self_1756;
    other_1525 = other_1524;
    let _e4: Motor = self_1757;
    let _e8: Flector = other_1525;
    let _e19: Motor = self_1757;
    let _e23: Flector = other_1525;
    let _e35: Motor = self_1757;
    let _e39: Flector = other_1525;
    let _e51: Motor = self_1757;
    let _e55: Flector = other_1525;
    let _e67: Motor = self_1757;
    let _e71: Flector = other_1525;
    let _e83: Motor = self_1757;
    let _e87: Flector = other_1525;
    return Plane((((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.zzxz) * vec4<f32>(1.0, 0.0, -(1.0), 0.0)) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))) + ((vec4<f32>(_e35.g1_.x) * _e39.g0_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e51.g1_.y) * _e55.g0_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0)))) + ((vec4<f32>(_e67.g1_.z) * _e71.g0_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e83.g0_.x) * _e87.g0_.xzyx) * vec4<f32>(0.0, -(1.0), 1.0, 0.0))));
}

fn motor_flector_wedge(self_1758: Motor, other_1526: Flector) -> Plane {
    var self_1759: Motor;
    var other_1527: Flector;

    self_1759 = self_1758;
    other_1527 = other_1526;
    let _e4: Motor = self_1759;
    let _e8: Flector = other_1527;
    let _e19: Motor = self_1759;
    let _e23: Flector = other_1527;
    let _e35: Motor = self_1759;
    let _e39: Flector = other_1527;
    let _e51: Motor = self_1759;
    let _e55: Flector = other_1527;
    let _e67: Motor = self_1759;
    let _e71: Flector = other_1527;
    let _e83: Motor = self_1759;
    let _e87: Flector = other_1527;
    return Plane((((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.zzxz) * vec4<f32>(1.0, 0.0, -(1.0), 0.0)) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))) + ((vec4<f32>(_e35.g1_.x) * _e39.g0_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e51.g1_.y) * _e55.g0_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0)))) + ((vec4<f32>(_e67.g1_.z) * _e71.g0_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e83.g0_.x) * _e87.g0_.xzyx) * vec4<f32>(0.0, -(1.0), 1.0, 0.0))));
}

fn motor_flector_join(self_1760: Motor, other_1528: Flector) -> Plane {
    var self_1761: Motor;
    var other_1529: Flector;

    self_1761 = self_1760;
    other_1529 = other_1528;
    let _e4: Motor = self_1761;
    let _e8: Flector = other_1529;
    let _e19: Motor = self_1761;
    let _e23: Flector = other_1529;
    let _e35: Motor = self_1761;
    let _e39: Flector = other_1529;
    let _e51: Motor = self_1761;
    let _e55: Flector = other_1529;
    let _e67: Motor = self_1761;
    let _e71: Flector = other_1529;
    let _e83: Motor = self_1761;
    let _e87: Flector = other_1529;
    return Plane((((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.zzxz) * vec4<f32>(1.0, 0.0, -(1.0), 0.0)) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))) + ((vec4<f32>(_e35.g1_.x) * _e39.g0_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e51.g1_.y) * _e55.g0_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0)))) + ((vec4<f32>(_e67.g1_.z) * _e71.g0_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e83.g0_.x) * _e87.g0_.xzyx) * vec4<f32>(0.0, -(1.0), 1.0, 0.0))));
}

fn motor_flector_geometric_anti_product(self_1762: Motor, other_1530: Flector) -> Flector {
    var self_1763: Motor;
    var other_1531: Flector;

    self_1763 = self_1762;
    other_1531 = other_1530;
    let _e4: Motor = self_1763;
    let _e8: Flector = other_1531;
    let _e11: Flector = other_1531;
    let _e14: Flector = other_1531;
    let _e17: Flector = other_1531;
    let _e30: Motor = self_1763;
    let _e34: Flector = other_1531;
    let _e37: Flector = other_1531;
    let _e40: Flector = other_1531;
    let _e43: Flector = other_1531;
    let _e57: Motor = self_1763;
    let _e61: Flector = other_1531;
    let _e64: Flector = other_1531;
    let _e67: Flector = other_1531;
    let _e70: Flector = other_1531;
    let _e84: Motor = self_1763;
    let _e88: Flector = other_1531;
    let _e92: Motor = self_1763;
    let _e96: Flector = other_1531;
    let _e99: Flector = other_1531;
    let _e102: Flector = other_1531;
    let _e105: Flector = other_1531;
    let _e118: Motor = self_1763;
    let _e122: Flector = other_1531;
    let _e125: Flector = other_1531;
    let _e128: Flector = other_1531;
    let _e131: Flector = other_1531;
    let _e144: Motor = self_1763;
    let _e148: Flector = other_1531;
    let _e151: Flector = other_1531;
    let _e154: Flector = other_1531;
    let _e157: Flector = other_1531;
    let _e170: Motor = self_1763;
    let _e174: Flector = other_1531;
    let _e177: Flector = other_1531;
    let _e180: Flector = other_1531;
    let _e183: Flector = other_1531;
    let _e196: Motor = self_1763;
    let _e200: Flector = other_1531;
    let _e203: Flector = other_1531;
    let _e206: Flector = other_1531;
    let _e209: Flector = other_1531;
    let _e223: Motor = self_1763;
    let _e227: Flector = other_1531;
    let _e230: Flector = other_1531;
    let _e233: Flector = other_1531;
    let _e236: Flector = other_1531;
    let _e250: Motor = self_1763;
    let _e254: Flector = other_1531;
    let _e258: Motor = self_1763;
    let _e262: Flector = other_1531;
    let _e275: Motor = self_1763;
    let _e279: Flector = other_1531;
    let _e292: Motor = self_1763;
    let _e296: Flector = other_1531;
    return Flector(((((((((vec4<f32>(_e4.g0_.x) * vec4<f32>(_e8.g1_.w, _e11.g0_.z, _e14.g0_.y, _e17.g1_.x)) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0))) + ((vec4<f32>(_e30.g0_.y) * vec4<f32>(_e34.g0_.z, _e37.g1_.w, _e40.g0_.x, _e43.g1_.y)) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e57.g0_.z) * vec4<f32>(_e61.g0_.y, _e64.g0_.x, _e67.g1_.w, _e70.g1_.z)) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + (vec4<f32>(_e84.g0_.w) * _e88.g0_)) + ((vec4<f32>(_e92.g1_.y) * vec4<f32>(_e96.g1_.z, _e99.g0_.w, _e102.g1_.x, _e105.g1_.z)) * vec4<f32>(1.0, 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e118.g1_.z) * vec4<f32>(_e122.g1_.y, _e125.g1_.x, _e128.g0_.w, _e131.g1_.y)) * vec4<f32>(-(1.0), 1.0, 1.0, 0.0))) + ((vec4<f32>(_e144.g1_.x) * vec4<f32>(_e148.g0_.w, _e151.g1_.z, _e154.g1_.y, _e157.g0_.x)) * vec4<f32>(1.0, -(1.0), 1.0, 0.0))), ((((((((vec4<f32>(_e170.g0_.x) * vec4<f32>(_e174.g0_.w, _e177.g1_.z, _e180.g1_.y, _e183.g0_.x)) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0))) + ((vec4<f32>(_e196.g0_.y) * vec4<f32>(_e200.g1_.z, _e203.g0_.w, _e206.g1_.x, _e209.g0_.y)) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e223.g0_.z) * vec4<f32>(_e227.g1_.y, _e230.g1_.x, _e233.g0_.w, _e236.g0_.z)) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + (vec4<f32>(_e250.g0_.w) * _e254.g1_)) + ((vec4<f32>(_e258.g1_.y) * vec4<f32>(_e262.g1_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e275.g1_.z) * vec4<f32>(_e279.g1_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e292.g1_.x) * vec4<f32>(_e296.g1_.x)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))));
}

fn motor_flector_inner_anti_product(self_1764: Motor, other_1532: Flector) -> Flector {
    var self_1765: Motor;
    var other_1533: Flector;

    self_1765 = self_1764;
    other_1533 = other_1532;
    let _e4: Motor = self_1765;
    let _e8: Flector = other_1533;
    let _e11: Motor = self_1765;
    let _e15: Flector = other_1533;
    let _e18: Flector = other_1533;
    let _e21: Flector = other_1533;
    let _e24: Flector = other_1533;
    let _e37: Motor = self_1765;
    let _e41: Flector = other_1533;
    let _e44: Flector = other_1533;
    let _e47: Flector = other_1533;
    let _e50: Flector = other_1533;
    let _e64: Motor = self_1765;
    let _e68: Flector = other_1533;
    let _e71: Flector = other_1533;
    let _e74: Flector = other_1533;
    let _e77: Flector = other_1533;
    let _e91: Motor = self_1765;
    let _e95: Flector = other_1533;
    let _e99: Motor = self_1765;
    let _e103: Flector = other_1533;
    let _e116: Motor = self_1765;
    let _e120: Flector = other_1533;
    let _e133: Motor = self_1765;
    let _e137: Flector = other_1533;
    return Flector((vec4<f32>(_e4.g0_.w) * _e8.g0_), ((((((((vec4<f32>(_e11.g0_.x) * vec4<f32>(_e15.g0_.w, _e18.g1_.z, _e21.g1_.y, _e24.g0_.x)) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0))) + ((vec4<f32>(_e37.g0_.y) * vec4<f32>(_e41.g1_.z, _e44.g0_.w, _e47.g1_.x, _e50.g0_.y)) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e64.g0_.z) * vec4<f32>(_e68.g1_.y, _e71.g1_.x, _e74.g0_.w, _e77.g0_.z)) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + (vec4<f32>(_e91.g0_.w) * _e95.g1_)) + ((vec4<f32>(_e99.g1_.y) * vec4<f32>(_e103.g1_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e116.g1_.z) * vec4<f32>(_e120.g1_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e133.g1_.x) * vec4<f32>(_e137.g1_.x)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))));
}

fn motor_flector_left_contraction(self_1766: Motor, other_1534: Flector) -> Point {
    var self_1767: Motor;
    var other_1535: Flector;

    self_1767 = self_1766;
    other_1535 = other_1534;
    let _e4: Motor = self_1767;
    let _e8: Flector = other_1535;
    let _e19: Motor = self_1767;
    let _e23: Flector = other_1535;
    let _e35: Motor = self_1767;
    let _e39: Flector = other_1535;
    return Point(((((vec4<f32>(_e4.g1_.y) * _e8.g1_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e19.g1_.z) * _e23.g1_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e35.g1_.x) * _e39.g1_.wxxx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))));
}

fn motor_flector_left_anti_contraction(self_1768: Motor, other_1536: Flector) -> Flector {
    var self_1769: Motor;
    var other_1537: Flector;

    self_1769 = self_1768;
    other_1537 = other_1536;
    let _e4: Motor = self_1769;
    let _e8: Flector = other_1537;
    let _e11: Motor = self_1769;
    let _e15: Flector = other_1537;
    let _e26: Motor = self_1769;
    let _e30: Flector = other_1537;
    let _e42: Motor = self_1769;
    let _e46: Flector = other_1537;
    let _e50: Motor = self_1769;
    let _e54: Flector = other_1537;
    return Flector((vec4<f32>(_e4.g0_.w) * _e8.g0_), (((((vec4<f32>(_e11.g0_.y) * _e15.g0_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e26.g0_.z) * _e30.g0_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + (vec4<f32>(_e42.g0_.w) * _e46.g1_)) + ((vec4<f32>(_e50.g0_.x) * _e54.g0_.wxxx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))));
}

fn motor_flector_right_anti_contraction(self_1770: Motor, other_1538: Flector) -> Plane {
    var self_1771: Motor;
    var other_1539: Flector;

    self_1771 = self_1770;
    other_1539 = other_1538;
    let _e4: Motor = self_1771;
    let _e8: Flector = other_1539;
    let _e19: Motor = self_1771;
    let _e23: Flector = other_1539;
    let _e35: Motor = self_1771;
    let _e39: Flector = other_1539;
    let _e52: Motor = self_1771;
    let _e56: Flector = other_1539;
    let _e69: Motor = self_1771;
    let _e73: Flector = other_1539;
    let _e86: Motor = self_1771;
    let _e90: Flector = other_1539;
    return Plane((((((((vec4<f32>(_e4.g0_.y) * _e8.g1_.zzxz) * vec4<f32>(1.0, 0.0, -(1.0), 0.0)) + ((vec4<f32>(_e19.g0_.z) * _e23.g1_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))) + ((vec4<f32>(_e35.g1_.x) * vec4<f32>(_e39.g1_.x)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e52.g1_.y) * vec4<f32>(_e56.g1_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e69.g1_.z) * vec4<f32>(_e73.g1_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e86.g0_.x) * _e90.g1_.xzyx) * vec4<f32>(0.0, -(1.0), 1.0, 0.0))));
}

fn motor_multi_vector_scalar_product(self_1772: Motor, other_1540: MultiVector) -> Scalar {
    var self_1773: Motor;
    var other_1541: MultiVector;

    self_1773 = self_1772;
    other_1541 = other_1540;
    let _e5: Motor = self_1773;
    let _e8: MultiVector = other_1541;
    let _e13: Motor = self_1773;
    let _e16: MultiVector = other_1541;
    let _e21: Motor = self_1773;
    let _e24: MultiVector = other_1541;
    return Scalar((((0.0 - (_e5.g1_.x * _e8.g3_.x)) - (_e13.g1_.y * _e16.g3_.y)) - (_e21.g1_.z * _e24.g3_.z)));
}

fn motor_multi_vector_dot(self_1774: Motor, other_1542: MultiVector) -> Scalar {
    var self_1775: Motor;
    var other_1543: MultiVector;

    self_1775 = self_1774;
    other_1543 = other_1542;
    let _e5: Motor = self_1775;
    let _e8: MultiVector = other_1543;
    let _e13: Motor = self_1775;
    let _e16: MultiVector = other_1543;
    let _e21: Motor = self_1775;
    let _e24: MultiVector = other_1543;
    return Scalar((((0.0 - (_e5.g1_.x * _e8.g3_.x)) - (_e13.g1_.y * _e16.g3_.y)) - (_e21.g1_.z * _e24.g3_.z)));
}

fn motor_multi_vector_anti_scalar_product(self_1776: Motor, other_1544: MultiVector) -> AntiScalar {
    var self_1777: Motor;
    var other_1545: MultiVector;

    self_1777 = self_1776;
    other_1545 = other_1544;
    let _e5: Motor = self_1777;
    let _e8: MultiVector = other_1545;
    let _e13: Motor = self_1777;
    let _e16: MultiVector = other_1545;
    let _e21: Motor = self_1777;
    let _e24: MultiVector = other_1545;
    let _e29: Motor = self_1777;
    let _e32: MultiVector = other_1545;
    return AntiScalar(((((0.0 - (_e5.g0_.x * _e8.g2_.x)) - (_e13.g0_.y * _e16.g2_.y)) - (_e21.g0_.z * _e24.g2_.z)) + (_e29.g0_.w * _e32.g0_.y)));
}

fn motor_multi_vector_anti_dot(self_1778: Motor, other_1546: MultiVector) -> AntiScalar {
    var self_1779: Motor;
    var other_1547: MultiVector;

    self_1779 = self_1778;
    other_1547 = other_1546;
    let _e5: Motor = self_1779;
    let _e8: MultiVector = other_1547;
    let _e13: Motor = self_1779;
    let _e16: MultiVector = other_1547;
    let _e21: Motor = self_1779;
    let _e24: MultiVector = other_1547;
    let _e29: Motor = self_1779;
    let _e32: MultiVector = other_1547;
    return AntiScalar(((((0.0 - (_e5.g0_.x * _e8.g2_.x)) - (_e13.g0_.y * _e16.g2_.y)) - (_e21.g0_.z * _e24.g2_.z)) + (_e29.g0_.w * _e32.g0_.y)));
}

fn motor_squared_magnitude(self_1780: Motor) -> Scalar {
    var self_1781: Motor;

    self_1781 = self_1780;
    let _e2: Motor = self_1781;
    let _e3: Motor = self_1781;
    let _e4: Motor = motor_reversal(_e3);
    let _e5: Scalar = motor_motor_scalar_product(_e2, _e4);
    return _e5;
}

fn motor_magnitude(self_1782: Motor) -> Scalar {
    var self_1783: Motor;

    self_1783 = self_1782;
    let _e2: Motor = self_1783;
    let _e3: Scalar = motor_squared_magnitude(_e2);
    return Scalar(sqrt(_e3.g0_));
}

fn motor_bulk_norm(self_1784: Motor) -> Scalar {
    var self_1785: Motor;

    self_1785 = self_1784;
    let _e2: Motor = self_1785;
    let _e3: Scalar = motor_squared_magnitude(_e2);
    return Scalar(sqrt(_e3.g0_));
}

fn motor_squared_anti_magnitude(self_1786: Motor) -> AntiScalar {
    var self_1787: Motor;

    self_1787 = self_1786;
    let _e2: Motor = self_1787;
    let _e3: Motor = self_1787;
    let _e4: Motor = motor_anti_reversal(_e3);
    let _e5: AntiScalar = motor_motor_anti_scalar_product(_e2, _e4);
    return _e5;
}

fn motor_weight_norm(self_1788: Motor) -> AntiScalar {
    var self_1789: Motor;

    self_1789 = self_1788;
    let _e2: Motor = self_1789;
    let _e3: AntiScalar = motor_squared_anti_magnitude(_e2);
    return AntiScalar(sqrt(_e3.g0_));
}

fn motor_geometric_norm(self_1790: Motor) -> HomogeneousMagnitude {
    var self_1791: Motor;

    self_1791 = self_1790;
    let _e2: Motor = self_1791;
    let _e3: Scalar = motor_bulk_norm(_e2);
    let _e4: Motor = self_1791;
    let _e5: AntiScalar = motor_weight_norm(_e4);
    let _e6: HomogeneousMagnitude = scalar_anti_scalar_add(_e3, _e5);
    return _e6;
}

fn motor_scale(self_1792: Motor, other_1548: f32) -> Motor {
    var self_1793: Motor;
    var other_1549: f32;

    self_1793 = self_1792;
    other_1549 = other_1548;
    let _e4: Motor = self_1793;
    let _e5: f32 = other_1549;
    let _e7: Motor = motor_scalar_geometric_product(_e4, Scalar(_e5));
    return _e7;
}

fn motor_signum(self_1794: Motor) -> Motor {
    var self_1795: Motor;

    self_1795 = self_1794;
    let _e2: Motor = self_1795;
    let _e3: Motor = self_1795;
    let _e4: Scalar = motor_magnitude(_e3);
    let _e9: Motor = motor_scalar_geometric_product(_e2, Scalar((1.0 / _e4.g0_)));
    return _e9;
}

fn motor_inverse(self_1796: Motor) -> Motor {
    var self_1797: Motor;

    self_1797 = self_1796;
    let _e2: Motor = self_1797;
    let _e3: Motor = motor_reversal(_e2);
    let _e4: Motor = self_1797;
    let _e5: Scalar = motor_squared_magnitude(_e4);
    let _e10: Motor = motor_scalar_geometric_product(_e3, Scalar((1.0 / _e5.g0_)));
    return _e10;
}

fn motor_unitize(self_1798: Motor) -> Motor {
    var self_1799: Motor;

    self_1799 = self_1798;
    let _e2: Motor = self_1799;
    let _e3: Motor = self_1799;
    let _e4: AntiScalar = motor_weight_norm(_e3);
    let _e9: Motor = motor_scalar_geometric_product(_e2, Scalar((1.0 / _e4.g0_)));
    return _e9;
}

fn motor_attitude(self_1800: Motor) -> Flector {
    var self_1801: Motor;

    self_1801 = self_1800;
    let _e2: Motor = self_1801;
    let _e9: Flector = motor_plane_regressive_product(_e2, Plane(vec4<f32>(0.0, 0.0, 0.0, 1.0)));
    return _e9;
}

fn rotor_zero() -> Rotor {
    return Rotor(vec4<f32>(0.0));
}

fn rotor_one() -> Rotor {
    return Rotor(vec4<f32>(0.0));
}

fn rotor_neg(self_1802: Rotor) -> Rotor {
    var self_1803: Rotor;

    self_1803 = self_1802;
    let _e2: Rotor = self_1803;
    return Rotor((_e2.g0_ * vec4<f32>(-(1.0))));
}

fn rotor_automorphism(self_1804: Rotor) -> Rotor {
    var self_1805: Rotor;

    self_1805 = self_1804;
    let _e2: Rotor = self_1805;
    return Rotor(_e2.g0_);
}

fn rotor_reversal(self_1806: Rotor) -> Rotor {
    var self_1807: Rotor;

    self_1807 = self_1806;
    let _e2: Rotor = self_1807;
    return Rotor((_e2.g0_ * vec4<f32>(-(1.0), -(1.0), -(1.0), 1.0)));
}

fn rotor_conjugation(self_1808: Rotor) -> Rotor {
    var self_1809: Rotor;

    self_1809 = self_1808;
    let _e2: Rotor = self_1809;
    return Rotor((_e2.g0_ * vec4<f32>(-(1.0), -(1.0), -(1.0), 1.0)));
}

fn rotor_anti_reversal(self_1810: Rotor) -> Rotor {
    var self_1811: Rotor;

    self_1811 = self_1810;
    let _e2: Rotor = self_1811;
    return Rotor((_e2.g0_ * vec4<f32>(-(1.0), -(1.0), -(1.0), 1.0)));
}

fn rotor_double_complement(self_1812: Rotor) -> Rotor {
    var self_1813: Rotor;

    self_1813 = self_1812;
    let _e2: Rotor = self_1813;
    return Rotor(_e2.g0_);
}

fn rotor_scalar_geometric_product(self_1814: Rotor, other_1550: Scalar) -> Rotor {
    var self_1815: Rotor;
    var other_1551: Scalar;

    self_1815 = self_1814;
    other_1551 = other_1550;
    let _e4: Rotor = self_1815;
    let _e6: Scalar = other_1551;
    return Rotor((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn rotor_scalar_regressive_product(self_1816: Rotor, other_1552: Scalar) -> Scalar {
    var self_1817: Rotor;
    var other_1553: Scalar;

    self_1817 = self_1816;
    other_1553 = other_1552;
    let _e4: Rotor = self_1817;
    let _e7: Scalar = other_1553;
    return Scalar((_e4.g0_.w * _e7.g0_));
}

fn rotor_scalar_anti_wedge(self_1818: Rotor, other_1554: Scalar) -> Scalar {
    var self_1819: Rotor;
    var other_1555: Scalar;

    self_1819 = self_1818;
    other_1555 = other_1554;
    let _e4: Rotor = self_1819;
    let _e7: Scalar = other_1555;
    return Scalar((_e4.g0_.w * _e7.g0_));
}

fn rotor_scalar_meet(self_1820: Rotor, other_1556: Scalar) -> Scalar {
    var self_1821: Rotor;
    var other_1557: Scalar;

    self_1821 = self_1820;
    other_1557 = other_1556;
    let _e4: Rotor = self_1821;
    let _e7: Scalar = other_1557;
    return Scalar((_e4.g0_.w * _e7.g0_));
}

fn rotor_scalar_outer_product(self_1822: Rotor, other_1558: Scalar) -> Rotor {
    var self_1823: Rotor;
    var other_1559: Scalar;

    self_1823 = self_1822;
    other_1559 = other_1558;
    let _e4: Rotor = self_1823;
    let _e6: Scalar = other_1559;
    return Rotor((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn rotor_scalar_wedge(self_1824: Rotor, other_1560: Scalar) -> Rotor {
    var self_1825: Rotor;
    var other_1561: Scalar;

    self_1825 = self_1824;
    other_1561 = other_1560;
    let _e4: Rotor = self_1825;
    let _e6: Scalar = other_1561;
    return Rotor((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn rotor_scalar_join(self_1826: Rotor, other_1562: Scalar) -> Rotor {
    var self_1827: Rotor;
    var other_1563: Scalar;

    self_1827 = self_1826;
    other_1563 = other_1562;
    let _e4: Rotor = self_1827;
    let _e6: Scalar = other_1563;
    return Rotor((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn rotor_scalar_inner_product(self_1828: Rotor, other_1564: Scalar) -> Rotor {
    var self_1829: Rotor;
    var other_1565: Scalar;

    self_1829 = self_1828;
    other_1565 = other_1564;
    let _e4: Rotor = self_1829;
    let _e6: Scalar = other_1565;
    return Rotor((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn rotor_scalar_right_contraction(self_1830: Rotor, other_1566: Scalar) -> Rotor {
    var self_1831: Rotor;
    var other_1567: Scalar;

    self_1831 = self_1830;
    other_1567 = other_1566;
    let _e4: Rotor = self_1831;
    let _e6: Scalar = other_1567;
    return Rotor((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn rotor_anti_scalar_into(self_1832: Rotor) -> AntiScalar {
    var self_1833: Rotor;

    self_1833 = self_1832;
    let _e2: Rotor = self_1833;
    return AntiScalar(_e2.g0_.w);
}

fn rotor_anti_scalar_add(self_1834: Rotor, other_1568: AntiScalar) -> Rotor {
    var self_1835: Rotor;
    var other_1569: AntiScalar;

    self_1835 = self_1834;
    other_1569 = other_1568;
    let _e4: Rotor = self_1835;
    let _e6: AntiScalar = other_1569;
    return Rotor((_e4.g0_ + (vec4<f32>(_e6.g0_) * vec4<f32>(0.0, 0.0, 0.0, 1.0))));
}

fn rotor_anti_scalar_sub(self_1836: Rotor, other_1570: AntiScalar) -> Rotor {
    var self_1837: Rotor;
    var other_1571: AntiScalar;

    self_1837 = self_1836;
    other_1571 = other_1570;
    let _e4: Rotor = self_1837;
    let _e6: AntiScalar = other_1571;
    return Rotor((_e4.g0_ - (vec4<f32>(_e6.g0_) * vec4<f32>(0.0, 0.0, 0.0, 1.0))));
}

fn rotor_anti_scalar_regressive_product(self_1838: Rotor, other_1572: AntiScalar) -> Rotor {
    var self_1839: Rotor;
    var other_1573: AntiScalar;

    self_1839 = self_1838;
    other_1573 = other_1572;
    let _e4: Rotor = self_1839;
    let _e6: AntiScalar = other_1573;
    return Rotor((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn rotor_anti_scalar_anti_wedge(self_1840: Rotor, other_1574: AntiScalar) -> Rotor {
    var self_1841: Rotor;
    var other_1575: AntiScalar;

    self_1841 = self_1840;
    other_1575 = other_1574;
    let _e4: Rotor = self_1841;
    let _e6: AntiScalar = other_1575;
    return Rotor((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn rotor_anti_scalar_meet(self_1842: Rotor, other_1576: AntiScalar) -> Rotor {
    var self_1843: Rotor;
    var other_1577: AntiScalar;

    self_1843 = self_1842;
    other_1577 = other_1576;
    let _e4: Rotor = self_1843;
    let _e6: AntiScalar = other_1577;
    return Rotor((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn rotor_anti_scalar_geometric_anti_product(self_1844: Rotor, other_1578: AntiScalar) -> Rotor {
    var self_1845: Rotor;
    var other_1579: AntiScalar;

    self_1845 = self_1844;
    other_1579 = other_1578;
    let _e4: Rotor = self_1845;
    let _e6: AntiScalar = other_1579;
    return Rotor((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn rotor_anti_scalar_inner_anti_product(self_1846: Rotor, other_1580: AntiScalar) -> Rotor {
    var self_1847: Rotor;
    var other_1581: AntiScalar;

    self_1847 = self_1846;
    other_1581 = other_1580;
    let _e4: Rotor = self_1847;
    let _e6: AntiScalar = other_1581;
    return Rotor((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn rotor_anti_scalar_left_anti_contraction(self_1848: Rotor, other_1582: AntiScalar) -> AntiScalar {
    var self_1849: Rotor;
    var other_1583: AntiScalar;

    self_1849 = self_1848;
    other_1583 = other_1582;
    let _e4: Rotor = self_1849;
    let _e7: AntiScalar = other_1583;
    return AntiScalar((_e4.g0_.w * _e7.g0_));
}

fn rotor_anti_scalar_right_anti_contraction(self_1850: Rotor, other_1584: AntiScalar) -> Rotor {
    var self_1851: Rotor;
    var other_1585: AntiScalar;

    self_1851 = self_1850;
    other_1585 = other_1584;
    let _e4: Rotor = self_1851;
    let _e6: AntiScalar = other_1585;
    return Rotor((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn rotor_anti_scalar_anti_scalar_product(self_1852: Rotor, other_1586: AntiScalar) -> AntiScalar {
    var self_1853: Rotor;
    var other_1587: AntiScalar;

    self_1853 = self_1852;
    other_1587 = other_1586;
    let _e4: Rotor = self_1853;
    let _e7: AntiScalar = other_1587;
    return AntiScalar((_e4.g0_.w * _e7.g0_));
}

fn rotor_anti_scalar_anti_dot(self_1854: Rotor, other_1588: AntiScalar) -> AntiScalar {
    var self_1855: Rotor;
    var other_1589: AntiScalar;

    self_1855 = self_1854;
    other_1589 = other_1588;
    let _e4: Rotor = self_1855;
    let _e7: AntiScalar = other_1589;
    return AntiScalar((_e4.g0_.w * _e7.g0_));
}

fn rotor_homogeneous_magnitude_geometric_product(self_1856: Rotor, other_1590: HomogeneousMagnitude) -> Rotor {
    var self_1857: Rotor;
    var other_1591: HomogeneousMagnitude;

    self_1857 = self_1856;
    other_1591 = other_1590;
    let _e4: Rotor = self_1857;
    let _e6: HomogeneousMagnitude = other_1591;
    return Rotor((_e4.g0_ * vec4<f32>(_e6.g0_.x)));
}

fn rotor_homogeneous_magnitude_outer_product(self_1858: Rotor, other_1592: HomogeneousMagnitude) -> Rotor {
    var self_1859: Rotor;
    var other_1593: HomogeneousMagnitude;

    self_1859 = self_1858;
    other_1593 = other_1592;
    let _e4: Rotor = self_1859;
    let _e6: HomogeneousMagnitude = other_1593;
    return Rotor((_e4.g0_ * vec4<f32>(_e6.g0_.x)));
}

fn rotor_homogeneous_magnitude_wedge(self_1860: Rotor, other_1594: HomogeneousMagnitude) -> Rotor {
    var self_1861: Rotor;
    var other_1595: HomogeneousMagnitude;

    self_1861 = self_1860;
    other_1595 = other_1594;
    let _e4: Rotor = self_1861;
    let _e6: HomogeneousMagnitude = other_1595;
    return Rotor((_e4.g0_ * vec4<f32>(_e6.g0_.x)));
}

fn rotor_homogeneous_magnitude_join(self_1862: Rotor, other_1596: HomogeneousMagnitude) -> Rotor {
    var self_1863: Rotor;
    var other_1597: HomogeneousMagnitude;

    self_1863 = self_1862;
    other_1597 = other_1596;
    let _e4: Rotor = self_1863;
    let _e6: HomogeneousMagnitude = other_1597;
    return Rotor((_e4.g0_ * vec4<f32>(_e6.g0_.x)));
}

fn rotor_homogeneous_magnitude_inner_product(self_1864: Rotor, other_1598: HomogeneousMagnitude) -> Rotor {
    var self_1865: Rotor;
    var other_1599: HomogeneousMagnitude;

    self_1865 = self_1864;
    other_1599 = other_1598;
    let _e4: Rotor = self_1865;
    let _e6: HomogeneousMagnitude = other_1599;
    return Rotor((_e4.g0_ * vec4<f32>(_e6.g0_.x)));
}

fn rotor_homogeneous_magnitude_right_contraction(self_1866: Rotor, other_1600: HomogeneousMagnitude) -> Rotor {
    var self_1867: Rotor;
    var other_1601: HomogeneousMagnitude;

    self_1867 = self_1866;
    other_1601 = other_1600;
    let _e4: Rotor = self_1867;
    let _e6: HomogeneousMagnitude = other_1601;
    return Rotor((_e4.g0_ * vec4<f32>(_e6.g0_.x)));
}

fn rotor_homogeneous_magnitude_right_anti_contraction(self_1868: Rotor, other_1602: HomogeneousMagnitude) -> Rotor {
    var self_1869: Rotor;
    var other_1603: HomogeneousMagnitude;

    self_1869 = self_1868;
    other_1603 = other_1602;
    let _e4: Rotor = self_1869;
    let _e6: HomogeneousMagnitude = other_1603;
    return Rotor((_e4.g0_ * vec4<f32>(_e6.g0_.y)));
}

fn rotor_homogeneous_magnitude_anti_scalar_product(self_1870: Rotor, other_1604: HomogeneousMagnitude) -> AntiScalar {
    var self_1871: Rotor;
    var other_1605: HomogeneousMagnitude;

    self_1871 = self_1870;
    other_1605 = other_1604;
    let _e4: Rotor = self_1871;
    let _e7: HomogeneousMagnitude = other_1605;
    return AntiScalar((_e4.g0_.w * _e7.g0_.y));
}

fn rotor_homogeneous_magnitude_anti_dot(self_1872: Rotor, other_1606: HomogeneousMagnitude) -> AntiScalar {
    var self_1873: Rotor;
    var other_1607: HomogeneousMagnitude;

    self_1873 = self_1872;
    other_1607 = other_1606;
    let _e4: Rotor = self_1873;
    let _e7: HomogeneousMagnitude = other_1607;
    return AntiScalar((_e4.g0_.w * _e7.g0_.y));
}

fn rotor_point_regressive_product(self_1874: Rotor, other_1608: Point) -> Point {
    var self_1875: Rotor;
    var other_1609: Point;

    self_1875 = self_1874;
    other_1609 = other_1608;
    let _e4: Rotor = self_1875;
    let _e8: Point = other_1609;
    return Point((vec4<f32>(_e4.g0_.w) * _e8.g0_));
}

fn rotor_point_anti_wedge(self_1876: Rotor, other_1610: Point) -> Point {
    var self_1877: Rotor;
    var other_1611: Point;

    self_1877 = self_1876;
    other_1611 = other_1610;
    let _e4: Rotor = self_1877;
    let _e8: Point = other_1611;
    return Point((vec4<f32>(_e4.g0_.w) * _e8.g0_));
}

fn rotor_point_meet(self_1878: Rotor, other_1612: Point) -> Point {
    var self_1879: Rotor;
    var other_1613: Point;

    self_1879 = self_1878;
    other_1613 = other_1612;
    let _e4: Rotor = self_1879;
    let _e8: Point = other_1613;
    return Point((vec4<f32>(_e4.g0_.w) * _e8.g0_));
}

fn rotor_point_geometric_anti_product(self_1880: Rotor, other_1614: Point) -> Flector {
    var self_1881: Rotor;
    var other_1615: Point;

    self_1881 = self_1880;
    other_1615 = other_1614;
    let _e4: Rotor = self_1881;
    let _e8: Point = other_1615;
    let _e19: Rotor = self_1881;
    let _e23: Point = other_1615;
    let _e35: Rotor = self_1881;
    let _e39: Point = other_1615;
    let _e43: Rotor = self_1881;
    let _e47: Point = other_1615;
    let _e59: Rotor = self_1881;
    let _e63: Point = other_1615;
    let _e74: Rotor = self_1881;
    let _e78: Point = other_1615;
    let _e90: Rotor = self_1881;
    let _e94: Point = other_1615;
    return Flector((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.zzxz) * vec4<f32>(1.0, 0.0, -(1.0), 0.0)) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))) + (vec4<f32>(_e35.g0_.w) * _e39.g0_)) + ((vec4<f32>(_e43.g0_.x) * _e47.g0_.xzyx) * vec4<f32>(0.0, -(1.0), 1.0, 0.0))), ((((vec4<f32>(_e59.g0_.y) * _e63.g0_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e74.g0_.z) * _e78.g0_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e90.g0_.x) * _e94.g0_.wxxx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))));
}

fn rotor_point_inner_anti_product(self_1882: Rotor, other_1616: Point) -> Flector {
    var self_1883: Rotor;
    var other_1617: Point;

    self_1883 = self_1882;
    other_1617 = other_1616;
    let _e4: Rotor = self_1883;
    let _e8: Point = other_1617;
    let _e11: Rotor = self_1883;
    let _e15: Point = other_1617;
    let _e26: Rotor = self_1883;
    let _e30: Point = other_1617;
    let _e42: Rotor = self_1883;
    let _e46: Point = other_1617;
    return Flector((vec4<f32>(_e4.g0_.w) * _e8.g0_), ((((vec4<f32>(_e11.g0_.y) * _e15.g0_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e26.g0_.z) * _e30.g0_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e42.g0_.x) * _e46.g0_.wxxx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))));
}

fn rotor_point_left_anti_contraction(self_1884: Rotor, other_1618: Point) -> Flector {
    var self_1885: Rotor;
    var other_1619: Point;

    self_1885 = self_1884;
    other_1619 = other_1618;
    let _e4: Rotor = self_1885;
    let _e8: Point = other_1619;
    let _e11: Rotor = self_1885;
    let _e15: Point = other_1619;
    let _e26: Rotor = self_1885;
    let _e30: Point = other_1619;
    let _e42: Rotor = self_1885;
    let _e46: Point = other_1619;
    return Flector((vec4<f32>(_e4.g0_.w) * _e8.g0_), ((((vec4<f32>(_e11.g0_.y) * _e15.g0_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e26.g0_.z) * _e30.g0_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e42.g0_.x) * _e46.g0_.wxxx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))));
}

fn rotor_line_add(self_1886: Rotor, other_1620: Line) -> Motor {
    var self_1887: Rotor;
    var other_1621: Line;

    self_1887 = self_1886;
    other_1621 = other_1620;
    let _e4: Rotor = self_1887;
    let _e6: Line = other_1621;
    let _e9: Line = other_1621;
    let _e12: Line = other_1621;
    let _e15: Line = other_1621;
    let _e26: Line = other_1621;
    return Motor((_e4.g0_ + (vec4<f32>(_e6.g0_.x, _e9.g0_.y, _e12.g0_.z, _e15.g0_.x) * vec4<f32>(1.0, 1.0, 1.0, 0.0))), _e26.g1_);
}

fn rotor_line_sub(self_1888: Rotor, other_1622: Line) -> Motor {
    var self_1889: Rotor;
    var other_1623: Line;

    self_1889 = self_1888;
    other_1623 = other_1622;
    let _e4: Rotor = self_1889;
    let _e6: Line = other_1623;
    let _e9: Line = other_1623;
    let _e12: Line = other_1623;
    let _e15: Line = other_1623;
    let _e28: Line = other_1623;
    return Motor((_e4.g0_ - (vec4<f32>(_e6.g0_.x, _e9.g0_.y, _e12.g0_.z, _e15.g0_.x) * vec4<f32>(1.0, 1.0, 1.0, 0.0))), (vec3<f32>(0.0) - _e28.g1_));
}

fn rotor_line_geometric_product(self_1890: Rotor, other_1624: Line) -> Rotor {
    var self_1891: Rotor;
    var other_1625: Line;

    self_1891 = self_1890;
    other_1625 = other_1624;
    let _e4: Rotor = self_1891;
    let _e8: Line = other_1625;
    let _e11: Line = other_1625;
    let _e14: Line = other_1625;
    let _e17: Line = other_1625;
    let _e30: Rotor = self_1891;
    let _e34: Line = other_1625;
    let _e37: Line = other_1625;
    let _e40: Line = other_1625;
    let _e43: Line = other_1625;
    let _e57: Rotor = self_1891;
    let _e61: Line = other_1625;
    let _e64: Line = other_1625;
    let _e67: Line = other_1625;
    let _e70: Line = other_1625;
    let _e82: Rotor = self_1891;
    let _e86: Line = other_1625;
    let _e89: Line = other_1625;
    let _e92: Line = other_1625;
    let _e95: Line = other_1625;
    return Rotor((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g1_.z, _e11.g1_.z, _e14.g1_.x, _e17.g1_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e30.g0_.z) * vec4<f32>(_e34.g1_.y, _e37.g1_.x, _e40.g1_.y, _e43.g1_.z)) * vec4<f32>(1.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e57.g0_.w) * vec4<f32>(_e61.g1_.x, _e64.g1_.y, _e67.g1_.z, _e70.g1_.x)) * vec4<f32>(1.0, 1.0, 1.0, 0.0))) + ((vec4<f32>(_e82.g0_.x) * vec4<f32>(_e86.g1_.x, _e89.g1_.z, _e92.g1_.y, _e95.g1_.x)) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))));
}

fn rotor_line_outer_product(self_1892: Rotor, other_1626: Line) -> AntiScalar {
    var self_1893: Rotor;
    var other_1627: Line;

    self_1893 = self_1892;
    other_1627 = other_1626;
    let _e5: Rotor = self_1893;
    let _e8: Line = other_1627;
    let _e13: Rotor = self_1893;
    let _e16: Line = other_1627;
    let _e21: Rotor = self_1893;
    let _e24: Line = other_1627;
    return AntiScalar((((0.0 - (_e5.g0_.x * _e8.g1_.x)) - (_e13.g0_.y * _e16.g1_.y)) - (_e21.g0_.z * _e24.g1_.z)));
}

fn rotor_line_wedge(self_1894: Rotor, other_1628: Line) -> AntiScalar {
    var self_1895: Rotor;
    var other_1629: Line;

    self_1895 = self_1894;
    other_1629 = other_1628;
    let _e5: Rotor = self_1895;
    let _e8: Line = other_1629;
    let _e13: Rotor = self_1895;
    let _e16: Line = other_1629;
    let _e21: Rotor = self_1895;
    let _e24: Line = other_1629;
    return AntiScalar((((0.0 - (_e5.g0_.x * _e8.g1_.x)) - (_e13.g0_.y * _e16.g1_.y)) - (_e21.g0_.z * _e24.g1_.z)));
}

fn rotor_line_join(self_1896: Rotor, other_1630: Line) -> AntiScalar {
    var self_1897: Rotor;
    var other_1631: Line;

    self_1897 = self_1896;
    other_1631 = other_1630;
    let _e5: Rotor = self_1897;
    let _e8: Line = other_1631;
    let _e13: Rotor = self_1897;
    let _e16: Line = other_1631;
    let _e21: Rotor = self_1897;
    let _e24: Line = other_1631;
    return AntiScalar((((0.0 - (_e5.g0_.x * _e8.g1_.x)) - (_e13.g0_.y * _e16.g1_.y)) - (_e21.g0_.z * _e24.g1_.z)));
}

fn rotor_line_inner_anti_product(self_1898: Rotor, other_1632: Line) -> Motor {
    var self_1899: Rotor;
    var other_1633: Line;

    self_1899 = self_1898;
    other_1633 = other_1632;
    let _e4: Rotor = self_1899;
    let _e8: Line = other_1633;
    let _e20: Rotor = self_1899;
    let _e24: Line = other_1633;
    let _e37: Rotor = self_1899;
    let _e40: Line = other_1633;
    let _e43: Line = other_1633;
    let _e46: Line = other_1633;
    let _e49: Line = other_1633;
    let _e62: Rotor = self_1899;
    let _e66: Line = other_1633;
    return Motor(((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0))) + ((vec4<f32>(_e20.g0_.z) * vec4<f32>(_e24.g0_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((_e37.g0_.wwwx * vec4<f32>(_e40.g0_.x, _e43.g0_.y, _e46.g0_.z, _e49.g0_.x)) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))), (vec3<f32>(_e62.g0_.w) * _e66.g1_));
}

fn rotor_line_left_anti_contraction(self_1900: Rotor, other_1634: Line) -> Motor {
    var self_1901: Rotor;
    var other_1635: Line;

    self_1901 = self_1900;
    other_1635 = other_1634;
    let _e4: Rotor = self_1901;
    let _e8: Line = other_1635;
    let _e20: Rotor = self_1901;
    let _e24: Line = other_1635;
    let _e37: Rotor = self_1901;
    let _e40: Line = other_1635;
    let _e43: Line = other_1635;
    let _e46: Line = other_1635;
    let _e49: Line = other_1635;
    let _e62: Rotor = self_1901;
    let _e66: Line = other_1635;
    return Motor(((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0))) + ((vec4<f32>(_e20.g0_.z) * vec4<f32>(_e24.g0_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((_e37.g0_.wwwx * vec4<f32>(_e40.g0_.x, _e43.g0_.y, _e46.g0_.z, _e49.g0_.x)) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))), (vec3<f32>(_e62.g0_.w) * _e66.g1_));
}

fn rotor_line_right_anti_contraction(self_1902: Rotor, other_1636: Line) -> AntiScalar {
    var self_1903: Rotor;
    var other_1637: Line;

    self_1903 = self_1902;
    other_1637 = other_1636;
    let _e5: Rotor = self_1903;
    let _e8: Line = other_1637;
    let _e13: Rotor = self_1903;
    let _e16: Line = other_1637;
    let _e21: Rotor = self_1903;
    let _e24: Line = other_1637;
    return AntiScalar((((0.0 - (_e5.g0_.x * _e8.g0_.x)) - (_e13.g0_.y * _e16.g0_.y)) - (_e21.g0_.z * _e24.g0_.z)));
}

fn rotor_line_anti_scalar_product(self_1904: Rotor, other_1638: Line) -> AntiScalar {
    var self_1905: Rotor;
    var other_1639: Line;

    self_1905 = self_1904;
    other_1639 = other_1638;
    let _e5: Rotor = self_1905;
    let _e8: Line = other_1639;
    let _e13: Rotor = self_1905;
    let _e16: Line = other_1639;
    let _e21: Rotor = self_1905;
    let _e24: Line = other_1639;
    return AntiScalar((((0.0 - (_e5.g0_.x * _e8.g0_.x)) - (_e13.g0_.y * _e16.g0_.y)) - (_e21.g0_.z * _e24.g0_.z)));
}

fn rotor_line_anti_dot(self_1906: Rotor, other_1640: Line) -> AntiScalar {
    var self_1907: Rotor;
    var other_1641: Line;

    self_1907 = self_1906;
    other_1641 = other_1640;
    let _e5: Rotor = self_1907;
    let _e8: Line = other_1641;
    let _e13: Rotor = self_1907;
    let _e16: Line = other_1641;
    let _e21: Rotor = self_1907;
    let _e24: Line = other_1641;
    return AntiScalar((((0.0 - (_e5.g0_.x * _e8.g0_.x)) - (_e13.g0_.y * _e16.g0_.y)) - (_e21.g0_.z * _e24.g0_.z)));
}

fn rotor_plane_regressive_product(self_1908: Rotor, other_1642: Plane) -> Flector {
    var self_1909: Rotor;
    var other_1643: Plane;

    self_1909 = self_1908;
    other_1643 = other_1642;
    let _e4: Rotor = self_1909;
    let _e8: Plane = other_1643;
    let _e19: Rotor = self_1909;
    let _e23: Plane = other_1643;
    let _e35: Rotor = self_1909;
    let _e39: Plane = other_1643;
    let _e51: Rotor = self_1909;
    let _e55: Plane = other_1643;
    return Flector(((((vec4<f32>(_e4.g0_.y) * _e8.g0_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e35.g0_.x) * _e39.g0_.wxxx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))), (vec4<f32>(_e51.g0_.w) * _e55.g0_));
}

fn rotor_plane_anti_wedge(self_1910: Rotor, other_1644: Plane) -> Flector {
    var self_1911: Rotor;
    var other_1645: Plane;

    self_1911 = self_1910;
    other_1645 = other_1644;
    let _e4: Rotor = self_1911;
    let _e8: Plane = other_1645;
    let _e19: Rotor = self_1911;
    let _e23: Plane = other_1645;
    let _e35: Rotor = self_1911;
    let _e39: Plane = other_1645;
    let _e51: Rotor = self_1911;
    let _e55: Plane = other_1645;
    return Flector(((((vec4<f32>(_e4.g0_.y) * _e8.g0_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e35.g0_.x) * _e39.g0_.wxxx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))), (vec4<f32>(_e51.g0_.w) * _e55.g0_));
}

fn rotor_plane_meet(self_1912: Rotor, other_1646: Plane) -> Flector {
    var self_1913: Rotor;
    var other_1647: Plane;

    self_1913 = self_1912;
    other_1647 = other_1646;
    let _e4: Rotor = self_1913;
    let _e8: Plane = other_1647;
    let _e19: Rotor = self_1913;
    let _e23: Plane = other_1647;
    let _e35: Rotor = self_1913;
    let _e39: Plane = other_1647;
    let _e51: Rotor = self_1913;
    let _e55: Plane = other_1647;
    return Flector(((((vec4<f32>(_e4.g0_.y) * _e8.g0_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e35.g0_.x) * _e39.g0_.wxxx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))), (vec4<f32>(_e51.g0_.w) * _e55.g0_));
}

fn rotor_plane_geometric_anti_product(self_1914: Rotor, other_1648: Plane) -> Flector {
    var self_1915: Rotor;
    var other_1649: Plane;

    self_1915 = self_1914;
    other_1649 = other_1648;
    let _e4: Rotor = self_1915;
    let _e8: Plane = other_1649;
    let _e19: Rotor = self_1915;
    let _e23: Plane = other_1649;
    let _e35: Rotor = self_1915;
    let _e39: Plane = other_1649;
    let _e51: Rotor = self_1915;
    let _e55: Plane = other_1649;
    let _e66: Rotor = self_1915;
    let _e70: Plane = other_1649;
    let _e82: Rotor = self_1915;
    let _e86: Plane = other_1649;
    let _e90: Rotor = self_1915;
    let _e94: Plane = other_1649;
    return Flector(((((vec4<f32>(_e4.g0_.y) * _e8.g0_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e35.g0_.x) * _e39.g0_.wxxx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))), (((((vec4<f32>(_e51.g0_.y) * _e55.g0_.zzxz) * vec4<f32>(1.0, 0.0, -(1.0), 0.0)) + ((vec4<f32>(_e66.g0_.z) * _e70.g0_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))) + (vec4<f32>(_e82.g0_.w) * _e86.g0_)) + ((vec4<f32>(_e90.g0_.x) * _e94.g0_.xzyx) * vec4<f32>(0.0, -(1.0), 1.0, 0.0))));
}

fn rotor_plane_inner_anti_product(self_1916: Rotor, other_1650: Plane) -> Plane {
    var self_1917: Rotor;
    var other_1651: Plane;

    self_1917 = self_1916;
    other_1651 = other_1650;
    let _e4: Rotor = self_1917;
    let _e8: Plane = other_1651;
    let _e19: Rotor = self_1917;
    let _e23: Plane = other_1651;
    let _e35: Rotor = self_1917;
    let _e39: Plane = other_1651;
    let _e43: Rotor = self_1917;
    let _e47: Plane = other_1651;
    return Plane((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.zzxz) * vec4<f32>(1.0, 0.0, -(1.0), 0.0)) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))) + (vec4<f32>(_e35.g0_.w) * _e39.g0_)) + ((vec4<f32>(_e43.g0_.x) * _e47.g0_.xzyx) * vec4<f32>(0.0, -(1.0), 1.0, 0.0))));
}

fn rotor_plane_left_anti_contraction(self_1918: Rotor, other_1652: Plane) -> Plane {
    var self_1919: Rotor;
    var other_1653: Plane;

    self_1919 = self_1918;
    other_1653 = other_1652;
    let _e4: Rotor = self_1919;
    let _e8: Plane = other_1653;
    return Plane((vec4<f32>(_e4.g0_.w) * _e8.g0_));
}

fn rotor_motor_add(self_1920: Rotor, other_1654: Motor) -> Motor {
    var self_1921: Rotor;
    var other_1655: Motor;

    self_1921 = self_1920;
    other_1655 = other_1654;
    let _e4: Rotor = self_1921;
    let _e6: Motor = other_1655;
    let _e9: Motor = other_1655;
    return Motor((_e4.g0_ + _e6.g0_), _e9.g1_);
}

fn rotor_motor_sub(self_1922: Rotor, other_1656: Motor) -> Motor {
    var self_1923: Rotor;
    var other_1657: Motor;

    self_1923 = self_1922;
    other_1657 = other_1656;
    let _e4: Rotor = self_1923;
    let _e6: Motor = other_1657;
    let _e11: Motor = other_1657;
    return Motor((_e4.g0_ - _e6.g0_), (vec3<f32>(0.0) - _e11.g1_));
}

fn rotor_motor_geometric_product(self_1924: Rotor, other_1658: Motor) -> Rotor {
    var self_1925: Rotor;
    var other_1659: Motor;

    self_1925 = self_1924;
    other_1659 = other_1658;
    let _e4: Rotor = self_1925;
    let _e8: Motor = other_1659;
    let _e11: Motor = other_1659;
    let _e14: Motor = other_1659;
    let _e17: Motor = other_1659;
    let _e30: Rotor = self_1925;
    let _e34: Motor = other_1659;
    let _e37: Motor = other_1659;
    let _e40: Motor = other_1659;
    let _e43: Motor = other_1659;
    let _e57: Rotor = self_1925;
    let _e61: Motor = other_1659;
    let _e64: Motor = other_1659;
    let _e67: Motor = other_1659;
    let _e70: Motor = other_1659;
    let _e82: Rotor = self_1925;
    let _e86: Motor = other_1659;
    let _e89: Motor = other_1659;
    let _e92: Motor = other_1659;
    let _e95: Motor = other_1659;
    return Rotor((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g1_.z, _e11.g1_.z, _e14.g1_.x, _e17.g1_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e30.g0_.z) * vec4<f32>(_e34.g1_.y, _e37.g1_.x, _e40.g1_.y, _e43.g1_.z)) * vec4<f32>(1.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e57.g0_.w) * vec4<f32>(_e61.g1_.x, _e64.g1_.y, _e67.g1_.z, _e70.g1_.x)) * vec4<f32>(1.0, 1.0, 1.0, 0.0))) + ((vec4<f32>(_e82.g0_.x) * vec4<f32>(_e86.g1_.x, _e89.g1_.z, _e92.g1_.y, _e95.g1_.x)) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))));
}

fn rotor_motor_outer_product(self_1926: Rotor, other_1660: Motor) -> AntiScalar {
    var self_1927: Rotor;
    var other_1661: Motor;

    self_1927 = self_1926;
    other_1661 = other_1660;
    let _e5: Rotor = self_1927;
    let _e8: Motor = other_1661;
    let _e13: Rotor = self_1927;
    let _e16: Motor = other_1661;
    let _e21: Rotor = self_1927;
    let _e24: Motor = other_1661;
    return AntiScalar((((0.0 - (_e5.g0_.x * _e8.g1_.x)) - (_e13.g0_.y * _e16.g1_.y)) - (_e21.g0_.z * _e24.g1_.z)));
}

fn rotor_motor_wedge(self_1928: Rotor, other_1662: Motor) -> AntiScalar {
    var self_1929: Rotor;
    var other_1663: Motor;

    self_1929 = self_1928;
    other_1663 = other_1662;
    let _e5: Rotor = self_1929;
    let _e8: Motor = other_1663;
    let _e13: Rotor = self_1929;
    let _e16: Motor = other_1663;
    let _e21: Rotor = self_1929;
    let _e24: Motor = other_1663;
    return AntiScalar((((0.0 - (_e5.g0_.x * _e8.g1_.x)) - (_e13.g0_.y * _e16.g1_.y)) - (_e21.g0_.z * _e24.g1_.z)));
}

fn rotor_motor_join(self_1930: Rotor, other_1664: Motor) -> AntiScalar {
    var self_1931: Rotor;
    var other_1665: Motor;

    self_1931 = self_1930;
    other_1665 = other_1664;
    let _e5: Rotor = self_1931;
    let _e8: Motor = other_1665;
    let _e13: Rotor = self_1931;
    let _e16: Motor = other_1665;
    let _e21: Rotor = self_1931;
    let _e24: Motor = other_1665;
    return AntiScalar((((0.0 - (_e5.g0_.x * _e8.g1_.x)) - (_e13.g0_.y * _e16.g1_.y)) - (_e21.g0_.z * _e24.g1_.z)));
}

fn rotor_motor_inner_anti_product(self_1932: Rotor, other_1666: Motor) -> Motor {
    var self_1933: Rotor;
    var other_1667: Motor;

    self_1933 = self_1932;
    other_1667 = other_1666;
    let _e4: Rotor = self_1933;
    let _e8: Motor = other_1667;
    let _e19: Rotor = self_1933;
    let _e23: Motor = other_1667;
    let _e35: Rotor = self_1933;
    let _e39: Motor = other_1667;
    let _e43: Rotor = self_1933;
    let _e47: Motor = other_1667;
    let _e59: Rotor = self_1933;
    let _e63: Motor = other_1667;
    return Motor((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + (vec4<f32>(_e35.g0_.w) * _e39.g0_)) + ((vec4<f32>(_e43.g0_.x) * _e47.g0_.wxxx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))), (vec3<f32>(_e59.g0_.w) * _e63.g1_));
}

fn rotor_motor_left_anti_contraction(self_1934: Rotor, other_1668: Motor) -> Motor {
    var self_1935: Rotor;
    var other_1669: Motor;

    self_1935 = self_1934;
    other_1669 = other_1668;
    let _e4: Rotor = self_1935;
    let _e8: Motor = other_1669;
    let _e20: Rotor = self_1935;
    let _e24: Motor = other_1669;
    let _e37: Rotor = self_1935;
    let _e41: Motor = other_1669;
    let _e45: Rotor = self_1935;
    let _e49: Motor = other_1669;
    let _e62: Rotor = self_1935;
    let _e66: Motor = other_1669;
    return Motor((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0))) + ((vec4<f32>(_e20.g0_.z) * vec4<f32>(_e24.g0_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + (vec4<f32>(_e37.g0_.w) * _e41.g0_)) + ((vec4<f32>(_e45.g0_.x) * vec4<f32>(_e49.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))), (vec3<f32>(_e62.g0_.w) * _e66.g1_));
}

fn rotor_motor_right_anti_contraction(self_1936: Rotor, other_1670: Motor) -> Rotor {
    var self_1937: Rotor;
    var other_1671: Motor;

    self_1937 = self_1936;
    other_1671 = other_1670;
    let _e4: Rotor = self_1937;
    let _e8: Motor = other_1671;
    let _e19: Rotor = self_1937;
    let _e23: Motor = other_1671;
    let _e35: Rotor = self_1937;
    let _e39: Motor = other_1671;
    let _e51: Rotor = self_1937;
    let _e55: Motor = other_1671;
    return Rotor((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e35.g0_.w) * vec4<f32>(_e39.g0_.w)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e51.g0_.x) * _e55.g0_.wxxx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))));
}

fn rotor_motor_anti_scalar_product(self_1938: Rotor, other_1672: Motor) -> AntiScalar {
    var self_1939: Rotor;
    var other_1673: Motor;

    self_1939 = self_1938;
    other_1673 = other_1672;
    let _e5: Rotor = self_1939;
    let _e8: Motor = other_1673;
    let _e13: Rotor = self_1939;
    let _e16: Motor = other_1673;
    let _e21: Rotor = self_1939;
    let _e24: Motor = other_1673;
    let _e29: Rotor = self_1939;
    let _e32: Motor = other_1673;
    return AntiScalar(((((0.0 - (_e5.g0_.x * _e8.g0_.x)) - (_e13.g0_.y * _e16.g0_.y)) - (_e21.g0_.z * _e24.g0_.z)) + (_e29.g0_.w * _e32.g0_.w)));
}

fn rotor_motor_anti_dot(self_1940: Rotor, other_1674: Motor) -> AntiScalar {
    var self_1941: Rotor;
    var other_1675: Motor;

    self_1941 = self_1940;
    other_1675 = other_1674;
    let _e5: Rotor = self_1941;
    let _e8: Motor = other_1675;
    let _e13: Rotor = self_1941;
    let _e16: Motor = other_1675;
    let _e21: Rotor = self_1941;
    let _e24: Motor = other_1675;
    let _e29: Rotor = self_1941;
    let _e32: Motor = other_1675;
    return AntiScalar(((((0.0 - (_e5.g0_.x * _e8.g0_.x)) - (_e13.g0_.y * _e16.g0_.y)) - (_e21.g0_.z * _e24.g0_.z)) + (_e29.g0_.w * _e32.g0_.w)));
}

fn rotor_rotor_add(self_1942: Rotor, other_1676: Rotor) -> Rotor {
    var self_1943: Rotor;
    var other_1677: Rotor;

    self_1943 = self_1942;
    other_1677 = other_1676;
    let _e4: Rotor = self_1943;
    let _e6: Rotor = other_1677;
    return Rotor((_e4.g0_ + _e6.g0_));
}

fn rotor_rotor_sub(self_1944: Rotor, other_1678: Rotor) -> Rotor {
    var self_1945: Rotor;
    var other_1679: Rotor;

    self_1945 = self_1944;
    other_1679 = other_1678;
    let _e4: Rotor = self_1945;
    let _e6: Rotor = other_1679;
    return Rotor((_e4.g0_ - _e6.g0_));
}

fn rotor_rotor_mul(self_1946: Rotor, other_1680: Rotor) -> Rotor {
    var self_1947: Rotor;
    var other_1681: Rotor;

    self_1947 = self_1946;
    other_1681 = other_1680;
    let _e4: Rotor = self_1947;
    let _e6: Rotor = other_1681;
    return Rotor((_e4.g0_ * _e6.g0_));
}

fn rotor_rotor_div(self_1948: Rotor, other_1682: Rotor) -> Rotor {
    var self_1949: Rotor;
    var other_1683: Rotor;

    self_1949 = self_1948;
    other_1683 = other_1682;
    let _e4: Rotor = self_1949;
    let _e7: Rotor = self_1949;
    let _e10: Rotor = self_1949;
    let _e13: Rotor = self_1949;
    let _e23: Rotor = other_1683;
    let _e26: Rotor = other_1683;
    let _e29: Rotor = other_1683;
    let _e32: Rotor = other_1683;
    return Rotor((((vec4<f32>(_e4.g0_.x, _e7.g0_.y, _e10.g0_.z, _e13.g0_.w) * vec4<f32>(1.0, 1.0, 1.0, 1.0)) / vec4<f32>(_e23.g0_.x, _e26.g0_.y, _e29.g0_.z, _e32.g0_.w)) * vec4<f32>(1.0, 1.0, 1.0, 1.0)));
}

fn rotor_rotor_regressive_product(self_1950: Rotor, other_1684: Rotor) -> Rotor {
    var self_1951: Rotor;
    var other_1685: Rotor;

    self_1951 = self_1950;
    other_1685 = other_1684;
    let _e4: Rotor = self_1951;
    let _e8: Rotor = other_1685;
    let _e11: Rotor = self_1951;
    let _e14: Rotor = other_1685;
    return Rotor(((vec4<f32>(_e4.g0_.w) * _e8.g0_) + ((_e11.g0_.xyzx * _e14.g0_.wwwx) * vec4<f32>(1.0, 1.0, 1.0, 0.0))));
}

fn rotor_rotor_anti_wedge(self_1952: Rotor, other_1686: Rotor) -> Rotor {
    var self_1953: Rotor;
    var other_1687: Rotor;

    self_1953 = self_1952;
    other_1687 = other_1686;
    let _e4: Rotor = self_1953;
    let _e8: Rotor = other_1687;
    let _e11: Rotor = self_1953;
    let _e14: Rotor = other_1687;
    return Rotor(((vec4<f32>(_e4.g0_.w) * _e8.g0_) + ((_e11.g0_.xyzx * _e14.g0_.wwwx) * vec4<f32>(1.0, 1.0, 1.0, 0.0))));
}

fn rotor_rotor_meet(self_1954: Rotor, other_1688: Rotor) -> Rotor {
    var self_1955: Rotor;
    var other_1689: Rotor;

    self_1955 = self_1954;
    other_1689 = other_1688;
    let _e4: Rotor = self_1955;
    let _e8: Rotor = other_1689;
    let _e11: Rotor = self_1955;
    let _e14: Rotor = other_1689;
    return Rotor(((vec4<f32>(_e4.g0_.w) * _e8.g0_) + ((_e11.g0_.xyzx * _e14.g0_.wwwx) * vec4<f32>(1.0, 1.0, 1.0, 0.0))));
}

fn rotor_rotor_geometric_anti_product(self_1956: Rotor, other_1690: Rotor) -> Rotor {
    var self_1957: Rotor;
    var other_1691: Rotor;

    self_1957 = self_1956;
    other_1691 = other_1690;
    let _e4: Rotor = self_1957;
    let _e8: Rotor = other_1691;
    let _e20: Rotor = self_1957;
    let _e24: Rotor = other_1691;
    let _e37: Rotor = self_1957;
    let _e41: Rotor = other_1691;
    let _e54: Rotor = self_1957;
    let _e58: Rotor = other_1691;
    return Rotor((((((vec4<f32>(_e4.g0_.x) * _e8.g0_.wzyx) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0))) + ((vec4<f32>(_e20.g0_.y) * _e24.g0_.zwxy) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e37.g0_.z) * _e41.g0_.yxwz) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + (vec4<f32>(_e54.g0_.w) * _e58.g0_)));
}

fn rotor_rotor_inner_anti_product(self_1958: Rotor, other_1692: Rotor) -> Rotor {
    var self_1959: Rotor;
    var other_1693: Rotor;

    self_1959 = self_1958;
    other_1693 = other_1692;
    let _e4: Rotor = self_1959;
    let _e8: Rotor = other_1693;
    let _e19: Rotor = self_1959;
    let _e23: Rotor = other_1693;
    let _e35: Rotor = self_1959;
    let _e39: Rotor = other_1693;
    let _e43: Rotor = self_1959;
    let _e47: Rotor = other_1693;
    return Rotor((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + (vec4<f32>(_e35.g0_.w) * _e39.g0_)) + ((vec4<f32>(_e43.g0_.x) * _e47.g0_.wxxx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))));
}

fn rotor_rotor_left_anti_contraction(self_1960: Rotor, other_1694: Rotor) -> Rotor {
    var self_1961: Rotor;
    var other_1695: Rotor;

    self_1961 = self_1960;
    other_1695 = other_1694;
    let _e4: Rotor = self_1961;
    let _e8: Rotor = other_1695;
    let _e20: Rotor = self_1961;
    let _e24: Rotor = other_1695;
    let _e37: Rotor = self_1961;
    let _e41: Rotor = other_1695;
    let _e45: Rotor = self_1961;
    let _e49: Rotor = other_1695;
    return Rotor((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0))) + ((vec4<f32>(_e20.g0_.z) * vec4<f32>(_e24.g0_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + (vec4<f32>(_e37.g0_.w) * _e41.g0_)) + ((vec4<f32>(_e45.g0_.x) * vec4<f32>(_e49.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))));
}

fn rotor_rotor_right_anti_contraction(self_1962: Rotor, other_1696: Rotor) -> Rotor {
    var self_1963: Rotor;
    var other_1697: Rotor;

    self_1963 = self_1962;
    other_1697 = other_1696;
    let _e4: Rotor = self_1963;
    let _e8: Rotor = other_1697;
    let _e19: Rotor = self_1963;
    let _e23: Rotor = other_1697;
    let _e35: Rotor = self_1963;
    let _e39: Rotor = other_1697;
    let _e51: Rotor = self_1963;
    let _e55: Rotor = other_1697;
    return Rotor((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e35.g0_.w) * vec4<f32>(_e39.g0_.w)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e51.g0_.x) * _e55.g0_.wxxx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))));
}

fn rotor_rotor_anti_scalar_product(self_1964: Rotor, other_1698: Rotor) -> AntiScalar {
    var self_1965: Rotor;
    var other_1699: Rotor;

    self_1965 = self_1964;
    other_1699 = other_1698;
    let _e5: Rotor = self_1965;
    let _e8: Rotor = other_1699;
    let _e13: Rotor = self_1965;
    let _e16: Rotor = other_1699;
    let _e21: Rotor = self_1965;
    let _e24: Rotor = other_1699;
    let _e29: Rotor = self_1965;
    let _e32: Rotor = other_1699;
    return AntiScalar(((((0.0 - (_e5.g0_.x * _e8.g0_.x)) - (_e13.g0_.y * _e16.g0_.y)) - (_e21.g0_.z * _e24.g0_.z)) + (_e29.g0_.w * _e32.g0_.w)));
}

fn rotor_rotor_anti_dot(self_1966: Rotor, other_1700: Rotor) -> AntiScalar {
    var self_1967: Rotor;
    var other_1701: Rotor;

    self_1967 = self_1966;
    other_1701 = other_1700;
    let _e5: Rotor = self_1967;
    let _e8: Rotor = other_1701;
    let _e13: Rotor = self_1967;
    let _e16: Rotor = other_1701;
    let _e21: Rotor = self_1967;
    let _e24: Rotor = other_1701;
    let _e29: Rotor = self_1967;
    let _e32: Rotor = other_1701;
    return AntiScalar(((((0.0 - (_e5.g0_.x * _e8.g0_.x)) - (_e13.g0_.y * _e16.g0_.y)) - (_e21.g0_.z * _e24.g0_.z)) + (_e29.g0_.w * _e32.g0_.w)));
}

fn rotor_translator_add(self_1968: Rotor, other_1702: Translator) -> Motor {
    var self_1969: Rotor;
    var other_1703: Translator;

    self_1969 = self_1968;
    other_1703 = other_1702;
    let _e4: Rotor = self_1969;
    let _e6: Translator = other_1703;
    let _e16: Translator = other_1703;
    let _e19: Translator = other_1703;
    let _e22: Translator = other_1703;
    return Motor((_e4.g0_ + (_e6.g0_.xxxw * vec4<f32>(0.0, 0.0, 0.0, 1.0))), vec3<f32>(_e16.g0_.x, _e19.g0_.y, _e22.g0_.z));
}

fn rotor_translator_sub(self_1970: Rotor, other_1704: Translator) -> Motor {
    var self_1971: Rotor;
    var other_1705: Translator;

    self_1971 = self_1970;
    other_1705 = other_1704;
    let _e4: Rotor = self_1971;
    let _e6: Translator = other_1705;
    let _e18: Translator = other_1705;
    let _e21: Translator = other_1705;
    let _e24: Translator = other_1705;
    return Motor((_e4.g0_ - (_e6.g0_.xxxw * vec4<f32>(0.0, 0.0, 0.0, 1.0))), (vec3<f32>(0.0) - vec3<f32>(_e18.g0_.x, _e21.g0_.y, _e24.g0_.z)));
}

fn rotor_translator_geometric_product(self_1972: Rotor, other_1706: Translator) -> Rotor {
    var self_1973: Rotor;
    var other_1707: Translator;

    self_1973 = self_1972;
    other_1707 = other_1706;
    let _e4: Rotor = self_1973;
    let _e8: Translator = other_1707;
    let _e20: Rotor = self_1973;
    let _e24: Translator = other_1707;
    let _e37: Rotor = self_1973;
    let _e41: Translator = other_1707;
    let _e52: Rotor = self_1973;
    let _e56: Translator = other_1707;
    return Rotor((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.zzxy) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e20.g0_.z) * _e24.g0_.yxyz) * vec4<f32>(1.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e37.g0_.w) * _e41.g0_.xyzx) * vec4<f32>(1.0, 1.0, 1.0, 0.0))) + ((vec4<f32>(_e52.g0_.x) * _e56.g0_.xzyx) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))));
}

fn rotor_translator_outer_product(self_1974: Rotor, other_1708: Translator) -> AntiScalar {
    var self_1975: Rotor;
    var other_1709: Translator;

    self_1975 = self_1974;
    other_1709 = other_1708;
    let _e5: Rotor = self_1975;
    let _e8: Translator = other_1709;
    let _e13: Rotor = self_1975;
    let _e16: Translator = other_1709;
    let _e21: Rotor = self_1975;
    let _e24: Translator = other_1709;
    return AntiScalar((((0.0 - (_e5.g0_.x * _e8.g0_.x)) - (_e13.g0_.y * _e16.g0_.y)) - (_e21.g0_.z * _e24.g0_.z)));
}

fn rotor_translator_wedge(self_1976: Rotor, other_1710: Translator) -> AntiScalar {
    var self_1977: Rotor;
    var other_1711: Translator;

    self_1977 = self_1976;
    other_1711 = other_1710;
    let _e5: Rotor = self_1977;
    let _e8: Translator = other_1711;
    let _e13: Rotor = self_1977;
    let _e16: Translator = other_1711;
    let _e21: Rotor = self_1977;
    let _e24: Translator = other_1711;
    return AntiScalar((((0.0 - (_e5.g0_.x * _e8.g0_.x)) - (_e13.g0_.y * _e16.g0_.y)) - (_e21.g0_.z * _e24.g0_.z)));
}

fn rotor_translator_join(self_1978: Rotor, other_1712: Translator) -> AntiScalar {
    var self_1979: Rotor;
    var other_1713: Translator;

    self_1979 = self_1978;
    other_1713 = other_1712;
    let _e5: Rotor = self_1979;
    let _e8: Translator = other_1713;
    let _e13: Rotor = self_1979;
    let _e16: Translator = other_1713;
    let _e21: Rotor = self_1979;
    let _e24: Translator = other_1713;
    return AntiScalar((((0.0 - (_e5.g0_.x * _e8.g0_.x)) - (_e13.g0_.y * _e16.g0_.y)) - (_e21.g0_.z * _e24.g0_.z)));
}

fn rotor_translator_inner_anti_product(self_1980: Rotor, other_1714: Translator) -> Motor {
    var self_1981: Rotor;
    var other_1715: Translator;

    self_1981 = self_1980;
    other_1715 = other_1714;
    let _e4: Rotor = self_1981;
    let _e6: Translator = other_1715;
    let _e11: Rotor = self_1981;
    let _e15: Translator = other_1715;
    let _e18: Translator = other_1715;
    let _e21: Translator = other_1715;
    return Motor((_e4.g0_ * vec4<f32>(_e6.g0_.w)), (vec3<f32>(_e11.g0_.w) * vec3<f32>(_e15.g0_.x, _e18.g0_.y, _e21.g0_.z)));
}

fn rotor_translator_left_anti_contraction(self_1982: Rotor, other_1716: Translator) -> Translator {
    var self_1983: Rotor;
    var other_1717: Translator;

    self_1983 = self_1982;
    other_1717 = other_1716;
    let _e4: Rotor = self_1983;
    let _e8: Translator = other_1717;
    return Translator((vec4<f32>(_e4.g0_.w) * _e8.g0_));
}

fn rotor_translator_right_anti_contraction(self_1984: Rotor, other_1718: Translator) -> Rotor {
    var self_1985: Rotor;
    var other_1719: Translator;

    self_1985 = self_1984;
    other_1719 = other_1718;
    let _e4: Rotor = self_1985;
    let _e6: Translator = other_1719;
    return Rotor((_e4.g0_ * vec4<f32>(_e6.g0_.w)));
}

fn rotor_translator_anti_scalar_product(self_1986: Rotor, other_1720: Translator) -> AntiScalar {
    var self_1987: Rotor;
    var other_1721: Translator;

    self_1987 = self_1986;
    other_1721 = other_1720;
    let _e4: Rotor = self_1987;
    let _e7: Translator = other_1721;
    return AntiScalar((_e4.g0_.w * _e7.g0_.w));
}

fn rotor_translator_anti_dot(self_1988: Rotor, other_1722: Translator) -> AntiScalar {
    var self_1989: Rotor;
    var other_1723: Translator;

    self_1989 = self_1988;
    other_1723 = other_1722;
    let _e4: Rotor = self_1989;
    let _e7: Translator = other_1723;
    return AntiScalar((_e4.g0_.w * _e7.g0_.w));
}

fn rotor_flector_regressive_product(self_1990: Rotor, other_1724: Flector) -> Flector {
    var self_1991: Rotor;
    var other_1725: Flector;

    self_1991 = self_1990;
    other_1725 = other_1724;
    let _e4: Rotor = self_1991;
    let _e8: Flector = other_1725;
    let _e19: Rotor = self_1991;
    let _e23: Flector = other_1725;
    let _e35: Rotor = self_1991;
    let _e39: Flector = other_1725;
    let _e43: Rotor = self_1991;
    let _e47: Flector = other_1725;
    let _e59: Rotor = self_1991;
    let _e63: Flector = other_1725;
    return Flector((((((vec4<f32>(_e4.g0_.y) * _e8.g1_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e19.g0_.z) * _e23.g1_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + (vec4<f32>(_e35.g0_.w) * _e39.g0_)) + ((vec4<f32>(_e43.g0_.x) * _e47.g1_.wxxx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))), (vec4<f32>(_e59.g0_.w) * _e63.g1_));
}

fn rotor_flector_anti_wedge(self_1992: Rotor, other_1726: Flector) -> Flector {
    var self_1993: Rotor;
    var other_1727: Flector;

    self_1993 = self_1992;
    other_1727 = other_1726;
    let _e4: Rotor = self_1993;
    let _e8: Flector = other_1727;
    let _e19: Rotor = self_1993;
    let _e23: Flector = other_1727;
    let _e35: Rotor = self_1993;
    let _e39: Flector = other_1727;
    let _e43: Rotor = self_1993;
    let _e47: Flector = other_1727;
    let _e59: Rotor = self_1993;
    let _e63: Flector = other_1727;
    return Flector((((((vec4<f32>(_e4.g0_.y) * _e8.g1_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e19.g0_.z) * _e23.g1_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + (vec4<f32>(_e35.g0_.w) * _e39.g0_)) + ((vec4<f32>(_e43.g0_.x) * _e47.g1_.wxxx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))), (vec4<f32>(_e59.g0_.w) * _e63.g1_));
}

fn rotor_flector_meet(self_1994: Rotor, other_1728: Flector) -> Flector {
    var self_1995: Rotor;
    var other_1729: Flector;

    self_1995 = self_1994;
    other_1729 = other_1728;
    let _e4: Rotor = self_1995;
    let _e8: Flector = other_1729;
    let _e19: Rotor = self_1995;
    let _e23: Flector = other_1729;
    let _e35: Rotor = self_1995;
    let _e39: Flector = other_1729;
    let _e43: Rotor = self_1995;
    let _e47: Flector = other_1729;
    let _e59: Rotor = self_1995;
    let _e63: Flector = other_1729;
    return Flector((((((vec4<f32>(_e4.g0_.y) * _e8.g1_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e19.g0_.z) * _e23.g1_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + (vec4<f32>(_e35.g0_.w) * _e39.g0_)) + ((vec4<f32>(_e43.g0_.x) * _e47.g1_.wxxx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))), (vec4<f32>(_e59.g0_.w) * _e63.g1_));
}

fn rotor_flector_geometric_anti_product(self_1996: Rotor, other_1730: Flector) -> Flector {
    var self_1997: Rotor;
    var other_1731: Flector;

    self_1997 = self_1996;
    other_1731 = other_1730;
    let _e4: Rotor = self_1997;
    let _e8: Flector = other_1731;
    let _e11: Flector = other_1731;
    let _e14: Flector = other_1731;
    let _e17: Flector = other_1731;
    let _e30: Rotor = self_1997;
    let _e34: Flector = other_1731;
    let _e37: Flector = other_1731;
    let _e40: Flector = other_1731;
    let _e43: Flector = other_1731;
    let _e57: Rotor = self_1997;
    let _e61: Flector = other_1731;
    let _e64: Flector = other_1731;
    let _e67: Flector = other_1731;
    let _e70: Flector = other_1731;
    let _e84: Rotor = self_1997;
    let _e88: Flector = other_1731;
    let _e92: Rotor = self_1997;
    let _e96: Flector = other_1731;
    let _e99: Flector = other_1731;
    let _e102: Flector = other_1731;
    let _e105: Flector = other_1731;
    let _e118: Rotor = self_1997;
    let _e122: Flector = other_1731;
    let _e125: Flector = other_1731;
    let _e128: Flector = other_1731;
    let _e131: Flector = other_1731;
    let _e145: Rotor = self_1997;
    let _e149: Flector = other_1731;
    let _e152: Flector = other_1731;
    let _e155: Flector = other_1731;
    let _e158: Flector = other_1731;
    let _e172: Rotor = self_1997;
    let _e176: Flector = other_1731;
    return Flector((((((vec4<f32>(_e4.g0_.x) * vec4<f32>(_e8.g1_.w, _e11.g0_.z, _e14.g0_.y, _e17.g1_.x)) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0))) + ((vec4<f32>(_e30.g0_.y) * vec4<f32>(_e34.g0_.z, _e37.g1_.w, _e40.g0_.x, _e43.g1_.y)) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e57.g0_.z) * vec4<f32>(_e61.g0_.y, _e64.g0_.x, _e67.g1_.w, _e70.g1_.z)) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + (vec4<f32>(_e84.g0_.w) * _e88.g0_)), (((((vec4<f32>(_e92.g0_.x) * vec4<f32>(_e96.g0_.w, _e99.g1_.z, _e102.g1_.y, _e105.g0_.x)) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0))) + ((vec4<f32>(_e118.g0_.y) * vec4<f32>(_e122.g1_.z, _e125.g0_.w, _e128.g1_.x, _e131.g0_.y)) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e145.g0_.z) * vec4<f32>(_e149.g1_.y, _e152.g1_.x, _e155.g0_.w, _e158.g0_.z)) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + (vec4<f32>(_e172.g0_.w) * _e176.g1_)));
}

fn rotor_flector_inner_anti_product(self_1998: Rotor, other_1732: Flector) -> Flector {
    var self_1999: Rotor;
    var other_1733: Flector;

    self_1999 = self_1998;
    other_1733 = other_1732;
    let _e4: Rotor = self_1999;
    let _e8: Flector = other_1733;
    let _e11: Rotor = self_1999;
    let _e15: Flector = other_1733;
    let _e18: Flector = other_1733;
    let _e21: Flector = other_1733;
    let _e24: Flector = other_1733;
    let _e37: Rotor = self_1999;
    let _e41: Flector = other_1733;
    let _e44: Flector = other_1733;
    let _e47: Flector = other_1733;
    let _e50: Flector = other_1733;
    let _e64: Rotor = self_1999;
    let _e68: Flector = other_1733;
    let _e71: Flector = other_1733;
    let _e74: Flector = other_1733;
    let _e77: Flector = other_1733;
    let _e91: Rotor = self_1999;
    let _e95: Flector = other_1733;
    return Flector((vec4<f32>(_e4.g0_.w) * _e8.g0_), (((((vec4<f32>(_e11.g0_.x) * vec4<f32>(_e15.g0_.w, _e18.g1_.z, _e21.g1_.y, _e24.g0_.x)) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0))) + ((vec4<f32>(_e37.g0_.y) * vec4<f32>(_e41.g1_.z, _e44.g0_.w, _e47.g1_.x, _e50.g0_.y)) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e64.g0_.z) * vec4<f32>(_e68.g1_.y, _e71.g1_.x, _e74.g0_.w, _e77.g0_.z)) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + (vec4<f32>(_e91.g0_.w) * _e95.g1_)));
}

fn rotor_flector_left_anti_contraction(self_2000: Rotor, other_1734: Flector) -> Flector {
    var self_2001: Rotor;
    var other_1735: Flector;

    self_2001 = self_2000;
    other_1735 = other_1734;
    let _e4: Rotor = self_2001;
    let _e8: Flector = other_1735;
    let _e11: Rotor = self_2001;
    let _e15: Flector = other_1735;
    let _e26: Rotor = self_2001;
    let _e30: Flector = other_1735;
    let _e42: Rotor = self_2001;
    let _e46: Flector = other_1735;
    let _e50: Rotor = self_2001;
    let _e54: Flector = other_1735;
    return Flector((vec4<f32>(_e4.g0_.w) * _e8.g0_), (((((vec4<f32>(_e11.g0_.y) * _e15.g0_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e26.g0_.z) * _e30.g0_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + (vec4<f32>(_e42.g0_.w) * _e46.g1_)) + ((vec4<f32>(_e50.g0_.x) * _e54.g0_.wxxx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))));
}

fn rotor_multi_vector_anti_scalar_product(self_2002: Rotor, other_1736: MultiVector) -> AntiScalar {
    var self_2003: Rotor;
    var other_1737: MultiVector;

    self_2003 = self_2002;
    other_1737 = other_1736;
    let _e5: Rotor = self_2003;
    let _e8: MultiVector = other_1737;
    let _e13: Rotor = self_2003;
    let _e16: MultiVector = other_1737;
    let _e21: Rotor = self_2003;
    let _e24: MultiVector = other_1737;
    let _e29: Rotor = self_2003;
    let _e32: MultiVector = other_1737;
    return AntiScalar(((((0.0 - (_e5.g0_.x * _e8.g2_.x)) - (_e13.g0_.y * _e16.g2_.y)) - (_e21.g0_.z * _e24.g2_.z)) + (_e29.g0_.w * _e32.g0_.y)));
}

fn rotor_multi_vector_anti_dot(self_2004: Rotor, other_1738: MultiVector) -> AntiScalar {
    var self_2005: Rotor;
    var other_1739: MultiVector;

    self_2005 = self_2004;
    other_1739 = other_1738;
    let _e5: Rotor = self_2005;
    let _e8: MultiVector = other_1739;
    let _e13: Rotor = self_2005;
    let _e16: MultiVector = other_1739;
    let _e21: Rotor = self_2005;
    let _e24: MultiVector = other_1739;
    let _e29: Rotor = self_2005;
    let _e32: MultiVector = other_1739;
    return AntiScalar(((((0.0 - (_e5.g0_.x * _e8.g2_.x)) - (_e13.g0_.y * _e16.g2_.y)) - (_e21.g0_.z * _e24.g2_.z)) + (_e29.g0_.w * _e32.g0_.y)));
}

fn rotor_scale(self_2006: Rotor, other_1740: f32) -> Rotor {
    var self_2007: Rotor;
    var other_1741: f32;

    self_2007 = self_2006;
    other_1741 = other_1740;
    let _e4: Rotor = self_2007;
    let _e5: f32 = other_1741;
    let _e7: Rotor = rotor_scalar_geometric_product(_e4, Scalar(_e5));
    return _e7;
}

fn rotor_attitude(self_2008: Rotor) -> Flector {
    var self_2009: Rotor;

    self_2009 = self_2008;
    let _e2: Rotor = self_2009;
    let _e9: Flector = rotor_plane_regressive_product(_e2, Plane(vec4<f32>(0.0, 0.0, 0.0, 1.0)));
    return _e9;
}

fn translator_zero() -> Translator {
    return Translator(vec4<f32>(0.0));
}

fn translator_one() -> Translator {
    return Translator(vec4<f32>(0.0));
}

fn translator_neg(self_2010: Translator) -> Translator {
    var self_2011: Translator;

    self_2011 = self_2010;
    let _e2: Translator = self_2011;
    return Translator((_e2.g0_ * vec4<f32>(-(1.0))));
}

fn translator_automorphism(self_2012: Translator) -> Translator {
    var self_2013: Translator;

    self_2013 = self_2012;
    let _e2: Translator = self_2013;
    return Translator(_e2.g0_);
}

fn translator_reversal(self_2014: Translator) -> Translator {
    var self_2015: Translator;

    self_2015 = self_2014;
    let _e2: Translator = self_2015;
    return Translator((_e2.g0_ * vec4<f32>(-(1.0), -(1.0), -(1.0), 1.0)));
}

fn translator_conjugation(self_2016: Translator) -> Translator {
    var self_2017: Translator;

    self_2017 = self_2016;
    let _e2: Translator = self_2017;
    return Translator((_e2.g0_ * vec4<f32>(-(1.0), -(1.0), -(1.0), 1.0)));
}

fn translator_anti_reversal(self_2018: Translator) -> Translator {
    var self_2019: Translator;

    self_2019 = self_2018;
    let _e2: Translator = self_2019;
    return Translator((_e2.g0_ * vec4<f32>(-(1.0), -(1.0), -(1.0), 1.0)));
}

fn translator_double_complement(self_2020: Translator) -> Translator {
    var self_2021: Translator;

    self_2021 = self_2020;
    let _e2: Translator = self_2021;
    return Translator(_e2.g0_);
}

fn translator_scalar_geometric_product(self_2022: Translator, other_1742: Scalar) -> Translator {
    var self_2023: Translator;
    var other_1743: Scalar;

    self_2023 = self_2022;
    other_1743 = other_1742;
    let _e4: Translator = self_2023;
    let _e6: Scalar = other_1743;
    return Translator((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn translator_scalar_regressive_product(self_2024: Translator, other_1744: Scalar) -> Scalar {
    var self_2025: Translator;
    var other_1745: Scalar;

    self_2025 = self_2024;
    other_1745 = other_1744;
    let _e4: Translator = self_2025;
    let _e7: Scalar = other_1745;
    return Scalar((_e4.g0_.w * _e7.g0_));
}

fn translator_scalar_anti_wedge(self_2026: Translator, other_1746: Scalar) -> Scalar {
    var self_2027: Translator;
    var other_1747: Scalar;

    self_2027 = self_2026;
    other_1747 = other_1746;
    let _e4: Translator = self_2027;
    let _e7: Scalar = other_1747;
    return Scalar((_e4.g0_.w * _e7.g0_));
}

fn translator_scalar_meet(self_2028: Translator, other_1748: Scalar) -> Scalar {
    var self_2029: Translator;
    var other_1749: Scalar;

    self_2029 = self_2028;
    other_1749 = other_1748;
    let _e4: Translator = self_2029;
    let _e7: Scalar = other_1749;
    return Scalar((_e4.g0_.w * _e7.g0_));
}

fn translator_scalar_outer_product(self_2030: Translator, other_1750: Scalar) -> Translator {
    var self_2031: Translator;
    var other_1751: Scalar;

    self_2031 = self_2030;
    other_1751 = other_1750;
    let _e4: Translator = self_2031;
    let _e6: Scalar = other_1751;
    return Translator((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn translator_scalar_wedge(self_2032: Translator, other_1752: Scalar) -> Translator {
    var self_2033: Translator;
    var other_1753: Scalar;

    self_2033 = self_2032;
    other_1753 = other_1752;
    let _e4: Translator = self_2033;
    let _e6: Scalar = other_1753;
    return Translator((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn translator_scalar_join(self_2034: Translator, other_1754: Scalar) -> Translator {
    var self_2035: Translator;
    var other_1755: Scalar;

    self_2035 = self_2034;
    other_1755 = other_1754;
    let _e4: Translator = self_2035;
    let _e6: Scalar = other_1755;
    return Translator((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn translator_scalar_inner_product(self_2036: Translator, other_1756: Scalar) -> Translator {
    var self_2037: Translator;
    var other_1757: Scalar;

    self_2037 = self_2036;
    other_1757 = other_1756;
    let _e4: Translator = self_2037;
    let _e6: Scalar = other_1757;
    return Translator((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn translator_scalar_geometric_anti_product(self_2038: Translator, other_1758: Scalar) -> Scalar {
    var self_2039: Translator;
    var other_1759: Scalar;

    self_2039 = self_2038;
    other_1759 = other_1758;
    let _e4: Translator = self_2039;
    let _e7: Scalar = other_1759;
    return Scalar((_e4.g0_.w * _e7.g0_));
}

fn translator_scalar_inner_anti_product(self_2040: Translator, other_1760: Scalar) -> Scalar {
    var self_2041: Translator;
    var other_1761: Scalar;

    self_2041 = self_2040;
    other_1761 = other_1760;
    let _e4: Translator = self_2041;
    let _e7: Scalar = other_1761;
    return Scalar((_e4.g0_.w * _e7.g0_));
}

fn translator_scalar_right_contraction(self_2042: Translator, other_1762: Scalar) -> Translator {
    var self_2043: Translator;
    var other_1763: Scalar;

    self_2043 = self_2042;
    other_1763 = other_1762;
    let _e4: Translator = self_2043;
    let _e6: Scalar = other_1763;
    return Translator((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn translator_scalar_left_anti_contraction(self_2044: Translator, other_1764: Scalar) -> Scalar {
    var self_2045: Translator;
    var other_1765: Scalar;

    self_2045 = self_2044;
    other_1765 = other_1764;
    let _e4: Translator = self_2045;
    let _e7: Scalar = other_1765;
    return Scalar((_e4.g0_.w * _e7.g0_));
}

fn translator_anti_scalar_into(self_2046: Translator) -> AntiScalar {
    var self_2047: Translator;

    self_2047 = self_2046;
    let _e2: Translator = self_2047;
    return AntiScalar(_e2.g0_.w);
}

fn translator_anti_scalar_add(self_2048: Translator, other_1766: AntiScalar) -> Translator {
    var self_2049: Translator;
    var other_1767: AntiScalar;

    self_2049 = self_2048;
    other_1767 = other_1766;
    let _e4: Translator = self_2049;
    let _e6: AntiScalar = other_1767;
    return Translator((_e4.g0_ + (vec4<f32>(_e6.g0_) * vec4<f32>(0.0, 0.0, 0.0, 1.0))));
}

fn translator_anti_scalar_sub(self_2050: Translator, other_1768: AntiScalar) -> Translator {
    var self_2051: Translator;
    var other_1769: AntiScalar;

    self_2051 = self_2050;
    other_1769 = other_1768;
    let _e4: Translator = self_2051;
    let _e6: AntiScalar = other_1769;
    return Translator((_e4.g0_ - (vec4<f32>(_e6.g0_) * vec4<f32>(0.0, 0.0, 0.0, 1.0))));
}

fn translator_anti_scalar_regressive_product(self_2052: Translator, other_1770: AntiScalar) -> Translator {
    var self_2053: Translator;
    var other_1771: AntiScalar;

    self_2053 = self_2052;
    other_1771 = other_1770;
    let _e4: Translator = self_2053;
    let _e6: AntiScalar = other_1771;
    return Translator((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn translator_anti_scalar_anti_wedge(self_2054: Translator, other_1772: AntiScalar) -> Translator {
    var self_2055: Translator;
    var other_1773: AntiScalar;

    self_2055 = self_2054;
    other_1773 = other_1772;
    let _e4: Translator = self_2055;
    let _e6: AntiScalar = other_1773;
    return Translator((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn translator_anti_scalar_meet(self_2056: Translator, other_1774: AntiScalar) -> Translator {
    var self_2057: Translator;
    var other_1775: AntiScalar;

    self_2057 = self_2056;
    other_1775 = other_1774;
    let _e4: Translator = self_2057;
    let _e6: AntiScalar = other_1775;
    return Translator((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn translator_anti_scalar_geometric_anti_product(self_2058: Translator, other_1776: AntiScalar) -> Translator {
    var self_2059: Translator;
    var other_1777: AntiScalar;

    self_2059 = self_2058;
    other_1777 = other_1776;
    let _e4: Translator = self_2059;
    let _e6: AntiScalar = other_1777;
    return Translator((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn translator_anti_scalar_inner_anti_product(self_2060: Translator, other_1778: AntiScalar) -> Translator {
    var self_2061: Translator;
    var other_1779: AntiScalar;

    self_2061 = self_2060;
    other_1779 = other_1778;
    let _e4: Translator = self_2061;
    let _e6: AntiScalar = other_1779;
    return Translator((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn translator_anti_scalar_left_anti_contraction(self_2062: Translator, other_1780: AntiScalar) -> AntiScalar {
    var self_2063: Translator;
    var other_1781: AntiScalar;

    self_2063 = self_2062;
    other_1781 = other_1780;
    let _e4: Translator = self_2063;
    let _e7: AntiScalar = other_1781;
    return AntiScalar((_e4.g0_.w * _e7.g0_));
}

fn translator_anti_scalar_right_anti_contraction(self_2064: Translator, other_1782: AntiScalar) -> Translator {
    var self_2065: Translator;
    var other_1783: AntiScalar;

    self_2065 = self_2064;
    other_1783 = other_1782;
    let _e4: Translator = self_2065;
    let _e6: AntiScalar = other_1783;
    return Translator((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn translator_anti_scalar_anti_scalar_product(self_2066: Translator, other_1784: AntiScalar) -> AntiScalar {
    var self_2067: Translator;
    var other_1785: AntiScalar;

    self_2067 = self_2066;
    other_1785 = other_1784;
    let _e4: Translator = self_2067;
    let _e7: AntiScalar = other_1785;
    return AntiScalar((_e4.g0_.w * _e7.g0_));
}

fn translator_anti_scalar_anti_dot(self_2068: Translator, other_1786: AntiScalar) -> AntiScalar {
    var self_2069: Translator;
    var other_1787: AntiScalar;

    self_2069 = self_2068;
    other_1787 = other_1786;
    let _e4: Translator = self_2069;
    let _e7: AntiScalar = other_1787;
    return AntiScalar((_e4.g0_.w * _e7.g0_));
}

fn translator_homogeneous_magnitude_geometric_product(self_2070: Translator, other_1788: HomogeneousMagnitude) -> Motor {
    var self_2071: Translator;
    var other_1789: HomogeneousMagnitude;

    self_2071 = self_2070;
    other_1789 = other_1788;
    let _e4: Translator = self_2071;
    let _e6: HomogeneousMagnitude = other_1789;
    let _e9: HomogeneousMagnitude = other_1789;
    let _e12: HomogeneousMagnitude = other_1789;
    let _e15: HomogeneousMagnitude = other_1789;
    let _e20: Translator = self_2071;
    let _e23: Translator = self_2071;
    let _e26: Translator = self_2071;
    let _e30: HomogeneousMagnitude = other_1789;
    return Motor((_e4.g0_ * vec4<f32>(_e6.g0_.y, _e9.g0_.y, _e12.g0_.y, _e15.g0_.x)), (vec3<f32>(_e20.g0_.x, _e23.g0_.y, _e26.g0_.z) * vec3<f32>(_e30.g0_.x)));
}

fn translator_homogeneous_magnitude_outer_product(self_2072: Translator, other_1790: HomogeneousMagnitude) -> Translator {
    var self_2073: Translator;
    var other_1791: HomogeneousMagnitude;

    self_2073 = self_2072;
    other_1791 = other_1790;
    let _e4: Translator = self_2073;
    let _e6: HomogeneousMagnitude = other_1791;
    return Translator((_e4.g0_ * vec4<f32>(_e6.g0_.x)));
}

fn translator_homogeneous_magnitude_wedge(self_2074: Translator, other_1792: HomogeneousMagnitude) -> Translator {
    var self_2075: Translator;
    var other_1793: HomogeneousMagnitude;

    self_2075 = self_2074;
    other_1793 = other_1792;
    let _e4: Translator = self_2075;
    let _e6: HomogeneousMagnitude = other_1793;
    return Translator((_e4.g0_ * vec4<f32>(_e6.g0_.x)));
}

fn translator_homogeneous_magnitude_join(self_2076: Translator, other_1794: HomogeneousMagnitude) -> Translator {
    var self_2077: Translator;
    var other_1795: HomogeneousMagnitude;

    self_2077 = self_2076;
    other_1795 = other_1794;
    let _e4: Translator = self_2077;
    let _e6: HomogeneousMagnitude = other_1795;
    return Translator((_e4.g0_ * vec4<f32>(_e6.g0_.x)));
}

fn translator_homogeneous_magnitude_inner_product(self_2078: Translator, other_1796: HomogeneousMagnitude) -> Motor {
    var self_2079: Translator;
    var other_1797: HomogeneousMagnitude;

    self_2079 = self_2078;
    other_1797 = other_1796;
    let _e4: Translator = self_2079;
    let _e6: HomogeneousMagnitude = other_1797;
    let _e9: HomogeneousMagnitude = other_1797;
    let _e12: HomogeneousMagnitude = other_1797;
    let _e15: HomogeneousMagnitude = other_1797;
    let _e20: Translator = self_2079;
    let _e23: Translator = self_2079;
    let _e26: Translator = self_2079;
    let _e30: HomogeneousMagnitude = other_1797;
    return Motor((_e4.g0_ * vec4<f32>(_e6.g0_.y, _e9.g0_.y, _e12.g0_.y, _e15.g0_.x)), (vec3<f32>(_e20.g0_.x, _e23.g0_.y, _e26.g0_.z) * vec3<f32>(_e30.g0_.x)));
}

fn translator_homogeneous_magnitude_right_contraction(self_2080: Translator, other_1798: HomogeneousMagnitude) -> Translator {
    var self_2081: Translator;
    var other_1799: HomogeneousMagnitude;

    self_2081 = self_2080;
    other_1799 = other_1798;
    let _e4: Translator = self_2081;
    let _e6: HomogeneousMagnitude = other_1799;
    return Translator((_e4.g0_ * vec4<f32>(_e6.g0_.x)));
}

fn translator_homogeneous_magnitude_left_anti_contraction(self_2082: Translator, other_1800: HomogeneousMagnitude) -> HomogeneousMagnitude {
    var self_2083: Translator;
    var other_1801: HomogeneousMagnitude;

    self_2083 = self_2082;
    other_1801 = other_1800;
    let _e4: Translator = self_2083;
    let _e8: HomogeneousMagnitude = other_1801;
    return HomogeneousMagnitude((vec2<f32>(_e4.g0_.w) * _e8.g0_));
}

fn translator_homogeneous_magnitude_right_anti_contraction(self_2084: Translator, other_1802: HomogeneousMagnitude) -> Translator {
    var self_2085: Translator;
    var other_1803: HomogeneousMagnitude;

    self_2085 = self_2084;
    other_1803 = other_1802;
    let _e4: Translator = self_2085;
    let _e6: HomogeneousMagnitude = other_1803;
    return Translator((_e4.g0_ * vec4<f32>(_e6.g0_.y)));
}

fn translator_homogeneous_magnitude_anti_scalar_product(self_2086: Translator, other_1804: HomogeneousMagnitude) -> AntiScalar {
    var self_2087: Translator;
    var other_1805: HomogeneousMagnitude;

    self_2087 = self_2086;
    other_1805 = other_1804;
    let _e4: Translator = self_2087;
    let _e7: HomogeneousMagnitude = other_1805;
    return AntiScalar((_e4.g0_.w * _e7.g0_.y));
}

fn translator_homogeneous_magnitude_anti_dot(self_2088: Translator, other_1806: HomogeneousMagnitude) -> AntiScalar {
    var self_2089: Translator;
    var other_1807: HomogeneousMagnitude;

    self_2089 = self_2088;
    other_1807 = other_1806;
    let _e4: Translator = self_2089;
    let _e7: HomogeneousMagnitude = other_1807;
    return AntiScalar((_e4.g0_.w * _e7.g0_.y));
}

fn translator_point_regressive_product(self_2090: Translator, other_1808: Point) -> Point {
    var self_2091: Translator;
    var other_1809: Point;

    self_2091 = self_2090;
    other_1809 = other_1808;
    let _e4: Translator = self_2091;
    let _e8: Point = other_1809;
    return Point((vec4<f32>(_e4.g0_.w) * _e8.g0_));
}

fn translator_point_anti_wedge(self_2092: Translator, other_1810: Point) -> Point {
    var self_2093: Translator;
    var other_1811: Point;

    self_2093 = self_2092;
    other_1811 = other_1810;
    let _e4: Translator = self_2093;
    let _e8: Point = other_1811;
    return Point((vec4<f32>(_e4.g0_.w) * _e8.g0_));
}

fn translator_point_meet(self_2094: Translator, other_1812: Point) -> Point {
    var self_2095: Translator;
    var other_1813: Point;

    self_2095 = self_2094;
    other_1813 = other_1812;
    let _e4: Translator = self_2095;
    let _e8: Point = other_1813;
    return Point((vec4<f32>(_e4.g0_.w) * _e8.g0_));
}

fn translator_point_outer_product(self_2096: Translator, other_1814: Point) -> Plane {
    var self_2097: Translator;
    var other_1815: Point;

    self_2097 = self_2096;
    other_1815 = other_1814;
    let _e4: Translator = self_2097;
    let _e8: Point = other_1815;
    let _e19: Translator = self_2097;
    let _e23: Point = other_1815;
    let _e35: Translator = self_2097;
    let _e39: Point = other_1815;
    return Plane(((((vec4<f32>(_e4.g0_.y) * _e8.g0_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e35.g0_.x) * _e39.g0_.wxxx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))));
}

fn translator_point_wedge(self_2098: Translator, other_1816: Point) -> Plane {
    var self_2099: Translator;
    var other_1817: Point;

    self_2099 = self_2098;
    other_1817 = other_1816;
    let _e4: Translator = self_2099;
    let _e8: Point = other_1817;
    let _e19: Translator = self_2099;
    let _e23: Point = other_1817;
    let _e35: Translator = self_2099;
    let _e39: Point = other_1817;
    return Plane(((((vec4<f32>(_e4.g0_.y) * _e8.g0_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e35.g0_.x) * _e39.g0_.wxxx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))));
}

fn translator_point_join(self_2100: Translator, other_1818: Point) -> Plane {
    var self_2101: Translator;
    var other_1819: Point;

    self_2101 = self_2100;
    other_1819 = other_1818;
    let _e4: Translator = self_2101;
    let _e8: Point = other_1819;
    let _e19: Translator = self_2101;
    let _e23: Point = other_1819;
    let _e35: Translator = self_2101;
    let _e39: Point = other_1819;
    return Plane(((((vec4<f32>(_e4.g0_.y) * _e8.g0_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e35.g0_.x) * _e39.g0_.wxxx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))));
}

fn translator_point_geometric_anti_product(self_2102: Translator, other_1820: Point) -> Point {
    var self_2103: Translator;
    var other_1821: Point;

    self_2103 = self_2102;
    other_1821 = other_1820;
    let _e4: Translator = self_2103;
    let _e8: Point = other_1821;
    let _e11: Translator = self_2103;
    let _e14: Point = other_1821;
    return Point(((vec4<f32>(_e4.g0_.w) * _e8.g0_) + ((_e11.g0_.xyzx * _e14.g0_.wwwx) * vec4<f32>(1.0, 1.0, 1.0, 0.0))));
}

fn translator_point_inner_anti_product(self_2104: Translator, other_1822: Point) -> Point {
    var self_2105: Translator;
    var other_1823: Point;

    self_2105 = self_2104;
    other_1823 = other_1822;
    let _e4: Translator = self_2105;
    let _e8: Point = other_1823;
    return Point((vec4<f32>(_e4.g0_.w) * _e8.g0_));
}

fn translator_point_left_anti_contraction(self_2106: Translator, other_1824: Point) -> Point {
    var self_2107: Translator;
    var other_1825: Point;

    self_2107 = self_2106;
    other_1825 = other_1824;
    let _e4: Translator = self_2107;
    let _e8: Point = other_1825;
    return Point((vec4<f32>(_e4.g0_.w) * _e8.g0_));
}

fn translator_line_add(self_2108: Translator, other_1826: Line) -> Motor {
    var self_2109: Translator;
    var other_1827: Line;

    self_2109 = self_2108;
    other_1827 = other_1826;
    let _e4: Translator = self_2109;
    let _e13: Line = other_1827;
    let _e16: Line = other_1827;
    let _e19: Line = other_1827;
    let _e22: Line = other_1827;
    let _e33: Translator = self_2109;
    let _e36: Translator = self_2109;
    let _e39: Translator = self_2109;
    let _e43: Line = other_1827;
    return Motor(((_e4.g0_.xxxw * vec4<f32>(0.0, 0.0, 0.0, 1.0)) + (vec4<f32>(_e13.g0_.x, _e16.g0_.y, _e19.g0_.z, _e22.g0_.x) * vec4<f32>(1.0, 1.0, 1.0, 0.0))), (vec3<f32>(_e33.g0_.x, _e36.g0_.y, _e39.g0_.z) + _e43.g1_));
}

fn translator_line_sub(self_2110: Translator, other_1828: Line) -> Motor {
    var self_2111: Translator;
    var other_1829: Line;

    self_2111 = self_2110;
    other_1829 = other_1828;
    let _e4: Translator = self_2111;
    let _e13: Line = other_1829;
    let _e16: Line = other_1829;
    let _e19: Line = other_1829;
    let _e22: Line = other_1829;
    let _e33: Translator = self_2111;
    let _e36: Translator = self_2111;
    let _e39: Translator = self_2111;
    let _e43: Line = other_1829;
    return Motor(((_e4.g0_.xxxw * vec4<f32>(0.0, 0.0, 0.0, 1.0)) - (vec4<f32>(_e13.g0_.x, _e16.g0_.y, _e19.g0_.z, _e22.g0_.x) * vec4<f32>(1.0, 1.0, 1.0, 0.0))), (vec3<f32>(_e33.g0_.x, _e36.g0_.y, _e39.g0_.z) - _e43.g1_));
}

fn translator_line_outer_product(self_2112: Translator, other_1830: Line) -> AntiScalar {
    var self_2113: Translator;
    var other_1831: Line;

    self_2113 = self_2112;
    other_1831 = other_1830;
    let _e5: Translator = self_2113;
    let _e8: Line = other_1831;
    let _e13: Translator = self_2113;
    let _e16: Line = other_1831;
    let _e21: Translator = self_2113;
    let _e24: Line = other_1831;
    return AntiScalar((((0.0 - (_e5.g0_.x * _e8.g0_.x)) - (_e13.g0_.y * _e16.g0_.y)) - (_e21.g0_.z * _e24.g0_.z)));
}

fn translator_line_wedge(self_2114: Translator, other_1832: Line) -> AntiScalar {
    var self_2115: Translator;
    var other_1833: Line;

    self_2115 = self_2114;
    other_1833 = other_1832;
    let _e5: Translator = self_2115;
    let _e8: Line = other_1833;
    let _e13: Translator = self_2115;
    let _e16: Line = other_1833;
    let _e21: Translator = self_2115;
    let _e24: Line = other_1833;
    return AntiScalar((((0.0 - (_e5.g0_.x * _e8.g0_.x)) - (_e13.g0_.y * _e16.g0_.y)) - (_e21.g0_.z * _e24.g0_.z)));
}

fn translator_line_join(self_2116: Translator, other_1834: Line) -> AntiScalar {
    var self_2117: Translator;
    var other_1835: Line;

    self_2117 = self_2116;
    other_1835 = other_1834;
    let _e5: Translator = self_2117;
    let _e8: Line = other_1835;
    let _e13: Translator = self_2117;
    let _e16: Line = other_1835;
    let _e21: Translator = self_2117;
    let _e24: Line = other_1835;
    return AntiScalar((((0.0 - (_e5.g0_.x * _e8.g0_.x)) - (_e13.g0_.y * _e16.g0_.y)) - (_e21.g0_.z * _e24.g0_.z)));
}

fn translator_line_inner_anti_product(self_2118: Translator, other_1836: Line) -> Line {
    var self_2119: Translator;
    var other_1837: Line;

    self_2119 = self_2118;
    other_1837 = other_1836;
    let _e4: Translator = self_2119;
    let _e8: Line = other_1837;
    let _e11: Translator = self_2119;
    let _e15: Line = other_1837;
    return Line((vec3<f32>(_e4.g0_.w) * _e8.g0_), (vec3<f32>(_e11.g0_.w) * _e15.g1_));
}

fn translator_line_left_contraction(self_2120: Translator, other_1838: Line) -> Scalar {
    var self_2121: Translator;
    var other_1839: Line;

    self_2121 = self_2120;
    other_1839 = other_1838;
    let _e5: Translator = self_2121;
    let _e8: Line = other_1839;
    let _e13: Translator = self_2121;
    let _e16: Line = other_1839;
    let _e21: Translator = self_2121;
    let _e24: Line = other_1839;
    return Scalar((((0.0 - (_e5.g0_.x * _e8.g1_.x)) - (_e13.g0_.y * _e16.g1_.y)) - (_e21.g0_.z * _e24.g1_.z)));
}

fn translator_line_left_anti_contraction(self_2122: Translator, other_1840: Line) -> Line {
    var self_2123: Translator;
    var other_1841: Line;

    self_2123 = self_2122;
    other_1841 = other_1840;
    let _e4: Translator = self_2123;
    let _e8: Line = other_1841;
    let _e11: Translator = self_2123;
    let _e15: Line = other_1841;
    return Line((vec3<f32>(_e4.g0_.w) * _e8.g0_), (vec3<f32>(_e11.g0_.w) * _e15.g1_));
}

fn translator_line_scalar_product(self_2124: Translator, other_1842: Line) -> Scalar {
    var self_2125: Translator;
    var other_1843: Line;

    self_2125 = self_2124;
    other_1843 = other_1842;
    let _e5: Translator = self_2125;
    let _e8: Line = other_1843;
    let _e13: Translator = self_2125;
    let _e16: Line = other_1843;
    let _e21: Translator = self_2125;
    let _e24: Line = other_1843;
    return Scalar((((0.0 - (_e5.g0_.x * _e8.g1_.x)) - (_e13.g0_.y * _e16.g1_.y)) - (_e21.g0_.z * _e24.g1_.z)));
}

fn translator_line_dot(self_2126: Translator, other_1844: Line) -> Scalar {
    var self_2127: Translator;
    var other_1845: Line;

    self_2127 = self_2126;
    other_1845 = other_1844;
    let _e5: Translator = self_2127;
    let _e8: Line = other_1845;
    let _e13: Translator = self_2127;
    let _e16: Line = other_1845;
    let _e21: Translator = self_2127;
    let _e24: Line = other_1845;
    return Scalar((((0.0 - (_e5.g0_.x * _e8.g1_.x)) - (_e13.g0_.y * _e16.g1_.y)) - (_e21.g0_.z * _e24.g1_.z)));
}

fn translator_plane_inner_product(self_2128: Translator, other_1846: Plane) -> Point {
    var self_2129: Translator;
    var other_1847: Plane;

    self_2129 = self_2128;
    other_1847 = other_1846;
    let _e4: Translator = self_2129;
    let _e8: Plane = other_1847;
    let _e19: Translator = self_2129;
    let _e23: Plane = other_1847;
    let _e35: Translator = self_2129;
    let _e39: Plane = other_1847;
    let _e52: Translator = self_2129;
    let _e56: Plane = other_1847;
    return Point((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e35.g0_.w) * vec4<f32>(_e39.g0_.w)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e52.g0_.x) * _e56.g0_.wxxx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))));
}

fn translator_plane_inner_anti_product(self_2130: Translator, other_1848: Plane) -> Plane {
    var self_2131: Translator;
    var other_1849: Plane;

    self_2131 = self_2130;
    other_1849 = other_1848;
    let _e4: Translator = self_2131;
    let _e8: Plane = other_1849;
    let _e20: Translator = self_2131;
    let _e24: Plane = other_1849;
    let _e37: Translator = self_2131;
    let _e41: Plane = other_1849;
    let _e45: Translator = self_2131;
    let _e49: Plane = other_1849;
    return Plane((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0))) + ((vec4<f32>(_e20.g0_.z) * vec4<f32>(_e24.g0_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + (vec4<f32>(_e37.g0_.w) * _e41.g0_)) + ((vec4<f32>(_e45.g0_.x) * vec4<f32>(_e49.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))));
}

fn translator_plane_left_contraction(self_2132: Translator, other_1850: Plane) -> Point {
    var self_2133: Translator;
    var other_1851: Plane;

    self_2133 = self_2132;
    other_1851 = other_1850;
    let _e4: Translator = self_2133;
    let _e8: Plane = other_1851;
    let _e19: Translator = self_2133;
    let _e23: Plane = other_1851;
    let _e35: Translator = self_2133;
    let _e39: Plane = other_1851;
    return Point(((((vec4<f32>(_e4.g0_.y) * _e8.g0_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e35.g0_.x) * _e39.g0_.wxxx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))));
}

fn translator_plane_left_anti_contraction(self_2134: Translator, other_1852: Plane) -> Plane {
    var self_2135: Translator;
    var other_1853: Plane;

    self_2135 = self_2134;
    other_1853 = other_1852;
    let _e4: Translator = self_2135;
    let _e8: Plane = other_1853;
    return Plane((vec4<f32>(_e4.g0_.w) * _e8.g0_));
}

fn translator_motor_add(self_2136: Translator, other_1854: Motor) -> Motor {
    var self_2137: Translator;
    var other_1855: Motor;

    self_2137 = self_2136;
    other_1855 = other_1854;
    let _e4: Translator = self_2137;
    let _e13: Motor = other_1855;
    let _e16: Translator = self_2137;
    let _e19: Translator = self_2137;
    let _e22: Translator = self_2137;
    let _e26: Motor = other_1855;
    return Motor(((_e4.g0_.xxxw * vec4<f32>(0.0, 0.0, 0.0, 1.0)) + _e13.g0_), (vec3<f32>(_e16.g0_.x, _e19.g0_.y, _e22.g0_.z) + _e26.g1_));
}

fn translator_motor_sub(self_2138: Translator, other_1856: Motor) -> Motor {
    var self_2139: Translator;
    var other_1857: Motor;

    self_2139 = self_2138;
    other_1857 = other_1856;
    let _e4: Translator = self_2139;
    let _e13: Motor = other_1857;
    let _e16: Translator = self_2139;
    let _e19: Translator = self_2139;
    let _e22: Translator = self_2139;
    let _e26: Motor = other_1857;
    return Motor(((_e4.g0_.xxxw * vec4<f32>(0.0, 0.0, 0.0, 1.0)) - _e13.g0_), (vec3<f32>(_e16.g0_.x, _e19.g0_.y, _e22.g0_.z) - _e26.g1_));
}

fn translator_motor_outer_product(self_2140: Translator, other_1858: Motor) -> AntiScalar {
    var self_2141: Translator;
    var other_1859: Motor;

    self_2141 = self_2140;
    other_1859 = other_1858;
    let _e5: Translator = self_2141;
    let _e8: Motor = other_1859;
    let _e13: Translator = self_2141;
    let _e16: Motor = other_1859;
    let _e21: Translator = self_2141;
    let _e24: Motor = other_1859;
    return AntiScalar((((0.0 - (_e5.g0_.x * _e8.g0_.x)) - (_e13.g0_.y * _e16.g0_.y)) - (_e21.g0_.z * _e24.g0_.z)));
}

fn translator_motor_wedge(self_2142: Translator, other_1860: Motor) -> AntiScalar {
    var self_2143: Translator;
    var other_1861: Motor;

    self_2143 = self_2142;
    other_1861 = other_1860;
    let _e5: Translator = self_2143;
    let _e8: Motor = other_1861;
    let _e13: Translator = self_2143;
    let _e16: Motor = other_1861;
    let _e21: Translator = self_2143;
    let _e24: Motor = other_1861;
    return AntiScalar((((0.0 - (_e5.g0_.x * _e8.g0_.x)) - (_e13.g0_.y * _e16.g0_.y)) - (_e21.g0_.z * _e24.g0_.z)));
}

fn translator_motor_join(self_2144: Translator, other_1862: Motor) -> AntiScalar {
    var self_2145: Translator;
    var other_1863: Motor;

    self_2145 = self_2144;
    other_1863 = other_1862;
    let _e5: Translator = self_2145;
    let _e8: Motor = other_1863;
    let _e13: Translator = self_2145;
    let _e16: Motor = other_1863;
    let _e21: Translator = self_2145;
    let _e24: Motor = other_1863;
    return AntiScalar((((0.0 - (_e5.g0_.x * _e8.g0_.x)) - (_e13.g0_.y * _e16.g0_.y)) - (_e21.g0_.z * _e24.g0_.z)));
}

fn translator_motor_inner_anti_product(self_2146: Translator, other_1864: Motor) -> Motor {
    var self_2147: Translator;
    var other_1865: Motor;

    self_2147 = self_2146;
    other_1865 = other_1864;
    let _e4: Translator = self_2147;
    let _e8: Motor = other_1865;
    let _e11: Translator = self_2147;
    let _e15: Motor = other_1865;
    let _e18: Translator = self_2147;
    let _e21: Translator = self_2147;
    let _e24: Translator = self_2147;
    let _e28: Motor = other_1865;
    return Motor((vec4<f32>(_e4.g0_.w) * _e8.g0_), ((vec3<f32>(_e11.g0_.w) * _e15.g1_) + (vec3<f32>(_e18.g0_.x, _e21.g0_.y, _e24.g0_.z) * vec3<f32>(_e28.g0_.w))));
}

fn translator_motor_left_anti_contraction(self_2148: Translator, other_1866: Motor) -> Motor {
    var self_2149: Translator;
    var other_1867: Motor;

    self_2149 = self_2148;
    other_1867 = other_1866;
    let _e4: Translator = self_2149;
    let _e8: Motor = other_1867;
    let _e11: Translator = self_2149;
    let _e15: Motor = other_1867;
    return Motor((vec4<f32>(_e4.g0_.w) * _e8.g0_), (vec3<f32>(_e11.g0_.w) * _e15.g1_));
}

fn translator_motor_right_anti_contraction(self_2150: Translator, other_1868: Motor) -> Translator {
    var self_2151: Translator;
    var other_1869: Motor;

    self_2151 = self_2150;
    other_1869 = other_1868;
    let _e4: Translator = self_2151;
    let _e6: Motor = other_1869;
    return Translator((_e4.g0_ * vec4<f32>(_e6.g0_.w)));
}

fn translator_motor_scalar_product(self_2152: Translator, other_1870: Motor) -> Scalar {
    var self_2153: Translator;
    var other_1871: Motor;

    self_2153 = self_2152;
    other_1871 = other_1870;
    let _e5: Translator = self_2153;
    let _e8: Motor = other_1871;
    let _e13: Translator = self_2153;
    let _e16: Motor = other_1871;
    let _e21: Translator = self_2153;
    let _e24: Motor = other_1871;
    return Scalar((((0.0 - (_e5.g0_.x * _e8.g1_.x)) - (_e13.g0_.y * _e16.g1_.y)) - (_e21.g0_.z * _e24.g1_.z)));
}

fn translator_motor_dot(self_2154: Translator, other_1872: Motor) -> Scalar {
    var self_2155: Translator;
    var other_1873: Motor;

    self_2155 = self_2154;
    other_1873 = other_1872;
    let _e5: Translator = self_2155;
    let _e8: Motor = other_1873;
    let _e13: Translator = self_2155;
    let _e16: Motor = other_1873;
    let _e21: Translator = self_2155;
    let _e24: Motor = other_1873;
    return Scalar((((0.0 - (_e5.g0_.x * _e8.g1_.x)) - (_e13.g0_.y * _e16.g1_.y)) - (_e21.g0_.z * _e24.g1_.z)));
}

fn translator_motor_anti_scalar_product(self_2156: Translator, other_1874: Motor) -> AntiScalar {
    var self_2157: Translator;
    var other_1875: Motor;

    self_2157 = self_2156;
    other_1875 = other_1874;
    let _e4: Translator = self_2157;
    let _e7: Motor = other_1875;
    return AntiScalar((_e4.g0_.w * _e7.g0_.w));
}

fn translator_motor_anti_dot(self_2158: Translator, other_1876: Motor) -> AntiScalar {
    var self_2159: Translator;
    var other_1877: Motor;

    self_2159 = self_2158;
    other_1877 = other_1876;
    let _e4: Translator = self_2159;
    let _e7: Motor = other_1877;
    return AntiScalar((_e4.g0_.w * _e7.g0_.w));
}

fn translator_rotor_add(self_2160: Translator, other_1878: Rotor) -> Motor {
    var self_2161: Translator;
    var other_1879: Rotor;

    self_2161 = self_2160;
    other_1879 = other_1878;
    let _e4: Translator = self_2161;
    let _e13: Rotor = other_1879;
    let _e16: Translator = self_2161;
    let _e19: Translator = self_2161;
    let _e22: Translator = self_2161;
    return Motor(((_e4.g0_.xxxw * vec4<f32>(0.0, 0.0, 0.0, 1.0)) + _e13.g0_), vec3<f32>(_e16.g0_.x, _e19.g0_.y, _e22.g0_.z));
}

fn translator_rotor_sub(self_2162: Translator, other_1880: Rotor) -> Motor {
    var self_2163: Translator;
    var other_1881: Rotor;

    self_2163 = self_2162;
    other_1881 = other_1880;
    let _e4: Translator = self_2163;
    let _e13: Rotor = other_1881;
    let _e16: Translator = self_2163;
    let _e19: Translator = self_2163;
    let _e22: Translator = self_2163;
    return Motor(((_e4.g0_.xxxw * vec4<f32>(0.0, 0.0, 0.0, 1.0)) - _e13.g0_), vec3<f32>(_e16.g0_.x, _e19.g0_.y, _e22.g0_.z));
}

fn translator_rotor_geometric_product(self_2164: Translator, other_1882: Rotor) -> Rotor {
    var self_2165: Translator;
    var other_1883: Rotor;

    self_2165 = self_2164;
    other_1883 = other_1882;
    let _e4: Translator = self_2165;
    let _e8: Rotor = other_1883;
    let _e20: Translator = self_2165;
    let _e24: Rotor = other_1883;
    let _e37: Translator = self_2165;
    let _e41: Rotor = other_1883;
    return Rotor(((((vec4<f32>(_e4.g0_.x) * _e8.g0_.wzyx) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0))) + ((vec4<f32>(_e20.g0_.y) * _e24.g0_.zwxy) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e37.g0_.z) * _e41.g0_.yxwz) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))));
}

fn translator_rotor_outer_product(self_2166: Translator, other_1884: Rotor) -> AntiScalar {
    var self_2167: Translator;
    var other_1885: Rotor;

    self_2167 = self_2166;
    other_1885 = other_1884;
    let _e5: Translator = self_2167;
    let _e8: Rotor = other_1885;
    let _e13: Translator = self_2167;
    let _e16: Rotor = other_1885;
    let _e21: Translator = self_2167;
    let _e24: Rotor = other_1885;
    return AntiScalar((((0.0 - (_e5.g0_.x * _e8.g0_.x)) - (_e13.g0_.y * _e16.g0_.y)) - (_e21.g0_.z * _e24.g0_.z)));
}

fn translator_rotor_wedge(self_2168: Translator, other_1886: Rotor) -> AntiScalar {
    var self_2169: Translator;
    var other_1887: Rotor;

    self_2169 = self_2168;
    other_1887 = other_1886;
    let _e5: Translator = self_2169;
    let _e8: Rotor = other_1887;
    let _e13: Translator = self_2169;
    let _e16: Rotor = other_1887;
    let _e21: Translator = self_2169;
    let _e24: Rotor = other_1887;
    return AntiScalar((((0.0 - (_e5.g0_.x * _e8.g0_.x)) - (_e13.g0_.y * _e16.g0_.y)) - (_e21.g0_.z * _e24.g0_.z)));
}

fn translator_rotor_join(self_2170: Translator, other_1888: Rotor) -> AntiScalar {
    var self_2171: Translator;
    var other_1889: Rotor;

    self_2171 = self_2170;
    other_1889 = other_1888;
    let _e5: Translator = self_2171;
    let _e8: Rotor = other_1889;
    let _e13: Translator = self_2171;
    let _e16: Rotor = other_1889;
    let _e21: Translator = self_2171;
    let _e24: Rotor = other_1889;
    return AntiScalar((((0.0 - (_e5.g0_.x * _e8.g0_.x)) - (_e13.g0_.y * _e16.g0_.y)) - (_e21.g0_.z * _e24.g0_.z)));
}

fn translator_rotor_inner_anti_product(self_2172: Translator, other_1890: Rotor) -> Motor {
    var self_2173: Translator;
    var other_1891: Rotor;

    self_2173 = self_2172;
    other_1891 = other_1890;
    let _e4: Translator = self_2173;
    let _e8: Rotor = other_1891;
    let _e11: Translator = self_2173;
    let _e14: Translator = self_2173;
    let _e17: Translator = self_2173;
    let _e21: Rotor = other_1891;
    return Motor((vec4<f32>(_e4.g0_.w) * _e8.g0_), (vec3<f32>(_e11.g0_.x, _e14.g0_.y, _e17.g0_.z) * vec3<f32>(_e21.g0_.w)));
}

fn translator_rotor_left_anti_contraction(self_2174: Translator, other_1892: Rotor) -> Rotor {
    var self_2175: Translator;
    var other_1893: Rotor;

    self_2175 = self_2174;
    other_1893 = other_1892;
    let _e4: Translator = self_2175;
    let _e8: Rotor = other_1893;
    return Rotor((vec4<f32>(_e4.g0_.w) * _e8.g0_));
}

fn translator_rotor_right_anti_contraction(self_2176: Translator, other_1894: Rotor) -> Translator {
    var self_2177: Translator;
    var other_1895: Rotor;

    self_2177 = self_2176;
    other_1895 = other_1894;
    let _e4: Translator = self_2177;
    let _e6: Rotor = other_1895;
    return Translator((_e4.g0_ * vec4<f32>(_e6.g0_.w)));
}

fn translator_rotor_anti_scalar_product(self_2178: Translator, other_1896: Rotor) -> AntiScalar {
    var self_2179: Translator;
    var other_1897: Rotor;

    self_2179 = self_2178;
    other_1897 = other_1896;
    let _e4: Translator = self_2179;
    let _e7: Rotor = other_1897;
    return AntiScalar((_e4.g0_.w * _e7.g0_.w));
}

fn translator_rotor_anti_dot(self_2180: Translator, other_1898: Rotor) -> AntiScalar {
    var self_2181: Translator;
    var other_1899: Rotor;

    self_2181 = self_2180;
    other_1899 = other_1898;
    let _e4: Translator = self_2181;
    let _e7: Rotor = other_1899;
    return AntiScalar((_e4.g0_.w * _e7.g0_.w));
}

fn translator_translator_add(self_2182: Translator, other_1900: Translator) -> Translator {
    var self_2183: Translator;
    var other_1901: Translator;

    self_2183 = self_2182;
    other_1901 = other_1900;
    let _e4: Translator = self_2183;
    let _e6: Translator = other_1901;
    return Translator((_e4.g0_ + _e6.g0_));
}

fn translator_translator_sub(self_2184: Translator, other_1902: Translator) -> Translator {
    var self_2185: Translator;
    var other_1903: Translator;

    self_2185 = self_2184;
    other_1903 = other_1902;
    let _e4: Translator = self_2185;
    let _e6: Translator = other_1903;
    return Translator((_e4.g0_ - _e6.g0_));
}

fn translator_translator_mul(self_2186: Translator, other_1904: Translator) -> Translator {
    var self_2187: Translator;
    var other_1905: Translator;

    self_2187 = self_2186;
    other_1905 = other_1904;
    let _e4: Translator = self_2187;
    let _e6: Translator = other_1905;
    return Translator((_e4.g0_ * _e6.g0_));
}

fn translator_translator_div(self_2188: Translator, other_1906: Translator) -> Translator {
    var self_2189: Translator;
    var other_1907: Translator;

    self_2189 = self_2188;
    other_1907 = other_1906;
    let _e4: Translator = self_2189;
    let _e7: Translator = self_2189;
    let _e10: Translator = self_2189;
    let _e13: Translator = self_2189;
    let _e23: Translator = other_1907;
    let _e26: Translator = other_1907;
    let _e29: Translator = other_1907;
    let _e32: Translator = other_1907;
    return Translator((((vec4<f32>(_e4.g0_.x, _e7.g0_.y, _e10.g0_.z, _e13.g0_.w) * vec4<f32>(1.0, 1.0, 1.0, 1.0)) / vec4<f32>(_e23.g0_.x, _e26.g0_.y, _e29.g0_.z, _e32.g0_.w)) * vec4<f32>(1.0, 1.0, 1.0, 1.0)));
}

fn translator_translator_regressive_product(self_2190: Translator, other_1908: Translator) -> Translator {
    var self_2191: Translator;
    var other_1909: Translator;

    self_2191 = self_2190;
    other_1909 = other_1908;
    let _e4: Translator = self_2191;
    let _e8: Translator = other_1909;
    let _e11: Translator = self_2191;
    let _e14: Translator = other_1909;
    return Translator(((vec4<f32>(_e4.g0_.w) * _e8.g0_) + ((_e11.g0_.xyzx * _e14.g0_.wwwx) * vec4<f32>(1.0, 1.0, 1.0, 0.0))));
}

fn translator_translator_anti_wedge(self_2192: Translator, other_1910: Translator) -> Translator {
    var self_2193: Translator;
    var other_1911: Translator;

    self_2193 = self_2192;
    other_1911 = other_1910;
    let _e4: Translator = self_2193;
    let _e8: Translator = other_1911;
    let _e11: Translator = self_2193;
    let _e14: Translator = other_1911;
    return Translator(((vec4<f32>(_e4.g0_.w) * _e8.g0_) + ((_e11.g0_.xyzx * _e14.g0_.wwwx) * vec4<f32>(1.0, 1.0, 1.0, 0.0))));
}

fn translator_translator_meet(self_2194: Translator, other_1912: Translator) -> Translator {
    var self_2195: Translator;
    var other_1913: Translator;

    self_2195 = self_2194;
    other_1913 = other_1912;
    let _e4: Translator = self_2195;
    let _e8: Translator = other_1913;
    let _e11: Translator = self_2195;
    let _e14: Translator = other_1913;
    return Translator(((vec4<f32>(_e4.g0_.w) * _e8.g0_) + ((_e11.g0_.xyzx * _e14.g0_.wwwx) * vec4<f32>(1.0, 1.0, 1.0, 0.0))));
}

fn translator_translator_geometric_anti_product(self_2196: Translator, other_1914: Translator) -> Translator {
    var self_2197: Translator;
    var other_1915: Translator;

    self_2197 = self_2196;
    other_1915 = other_1914;
    let _e4: Translator = self_2197;
    let _e8: Translator = other_1915;
    let _e11: Translator = self_2197;
    let _e14: Translator = other_1915;
    return Translator(((vec4<f32>(_e4.g0_.w) * _e8.g0_) + ((_e11.g0_.xyzx * _e14.g0_.wwwx) * vec4<f32>(1.0, 1.0, 1.0, 0.0))));
}

fn translator_translator_inner_anti_product(self_2198: Translator, other_1916: Translator) -> Translator {
    var self_2199: Translator;
    var other_1917: Translator;

    self_2199 = self_2198;
    other_1917 = other_1916;
    let _e4: Translator = self_2199;
    let _e8: Translator = other_1917;
    let _e11: Translator = self_2199;
    let _e14: Translator = other_1917;
    return Translator(((vec4<f32>(_e4.g0_.w) * _e8.g0_) + ((_e11.g0_.xyzx * _e14.g0_.wwwx) * vec4<f32>(1.0, 1.0, 1.0, 0.0))));
}

fn translator_translator_left_anti_contraction(self_2200: Translator, other_1918: Translator) -> Translator {
    var self_2201: Translator;
    var other_1919: Translator;

    self_2201 = self_2200;
    other_1919 = other_1918;
    let _e4: Translator = self_2201;
    let _e8: Translator = other_1919;
    return Translator((vec4<f32>(_e4.g0_.w) * _e8.g0_));
}

fn translator_translator_right_anti_contraction(self_2202: Translator, other_1920: Translator) -> Translator {
    var self_2203: Translator;
    var other_1921: Translator;

    self_2203 = self_2202;
    other_1921 = other_1920;
    let _e4: Translator = self_2203;
    let _e6: Translator = other_1921;
    return Translator((_e4.g0_ * vec4<f32>(_e6.g0_.w)));
}

fn translator_translator_scalar_product(self_2204: Translator, other_1922: Translator) -> Scalar {
    var self_2205: Translator;
    var other_1923: Translator;

    self_2205 = self_2204;
    other_1923 = other_1922;
    let _e5: Translator = self_2205;
    let _e8: Translator = other_1923;
    let _e13: Translator = self_2205;
    let _e16: Translator = other_1923;
    let _e21: Translator = self_2205;
    let _e24: Translator = other_1923;
    return Scalar((((0.0 - (_e5.g0_.x * _e8.g0_.x)) - (_e13.g0_.y * _e16.g0_.y)) - (_e21.g0_.z * _e24.g0_.z)));
}

fn translator_translator_dot(self_2206: Translator, other_1924: Translator) -> Scalar {
    var self_2207: Translator;
    var other_1925: Translator;

    self_2207 = self_2206;
    other_1925 = other_1924;
    let _e5: Translator = self_2207;
    let _e8: Translator = other_1925;
    let _e13: Translator = self_2207;
    let _e16: Translator = other_1925;
    let _e21: Translator = self_2207;
    let _e24: Translator = other_1925;
    return Scalar((((0.0 - (_e5.g0_.x * _e8.g0_.x)) - (_e13.g0_.y * _e16.g0_.y)) - (_e21.g0_.z * _e24.g0_.z)));
}

fn translator_translator_anti_scalar_product(self_2208: Translator, other_1926: Translator) -> AntiScalar {
    var self_2209: Translator;
    var other_1927: Translator;

    self_2209 = self_2208;
    other_1927 = other_1926;
    let _e4: Translator = self_2209;
    let _e7: Translator = other_1927;
    return AntiScalar((_e4.g0_.w * _e7.g0_.w));
}

fn translator_translator_anti_dot(self_2210: Translator, other_1928: Translator) -> AntiScalar {
    var self_2211: Translator;
    var other_1929: Translator;

    self_2211 = self_2210;
    other_1929 = other_1928;
    let _e4: Translator = self_2211;
    let _e7: Translator = other_1929;
    return AntiScalar((_e4.g0_.w * _e7.g0_.w));
}

fn translator_flector_geometric_product(self_2212: Translator, other_1930: Flector) -> Flector {
    var self_2213: Translator;
    var other_1931: Flector;

    self_2213 = self_2212;
    other_1931 = other_1930;
    let _e4: Translator = self_2213;
    let _e8: Flector = other_1931;
    let _e11: Flector = other_1931;
    let _e14: Flector = other_1931;
    let _e17: Flector = other_1931;
    let _e30: Translator = self_2213;
    let _e34: Flector = other_1931;
    let _e37: Flector = other_1931;
    let _e40: Flector = other_1931;
    let _e43: Flector = other_1931;
    let _e57: Translator = self_2213;
    let _e61: Flector = other_1931;
    let _e64: Flector = other_1931;
    let _e67: Flector = other_1931;
    let _e70: Flector = other_1931;
    let _e84: Translator = self_2213;
    let _e87: Flector = other_1931;
    let _e99: Translator = self_2213;
    let _e103: Flector = other_1931;
    let _e106: Flector = other_1931;
    let _e109: Flector = other_1931;
    let _e112: Flector = other_1931;
    let _e125: Translator = self_2213;
    let _e129: Flector = other_1931;
    let _e132: Flector = other_1931;
    let _e135: Flector = other_1931;
    let _e138: Flector = other_1931;
    let _e152: Translator = self_2213;
    let _e156: Flector = other_1931;
    let _e159: Flector = other_1931;
    let _e162: Flector = other_1931;
    let _e165: Flector = other_1931;
    let _e179: Translator = self_2213;
    let _e182: Flector = other_1931;
    return Flector((((((vec4<f32>(_e4.g0_.x) * vec4<f32>(_e8.g1_.w, _e11.g0_.z, _e14.g0_.y, _e17.g1_.x)) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0))) + ((vec4<f32>(_e30.g0_.y) * vec4<f32>(_e34.g0_.z, _e37.g1_.w, _e40.g0_.x, _e43.g1_.y)) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e57.g0_.z) * vec4<f32>(_e61.g0_.y, _e64.g0_.x, _e67.g1_.w, _e70.g1_.z)) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))) + ((_e84.g0_.xxxw * _e87.g1_.xxxw) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))), (((((vec4<f32>(_e99.g0_.x) * vec4<f32>(_e103.g0_.w, _e106.g1_.z, _e109.g1_.y, _e112.g0_.x)) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0))) + ((vec4<f32>(_e125.g0_.y) * vec4<f32>(_e129.g1_.z, _e132.g0_.w, _e135.g1_.x, _e138.g0_.y)) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e152.g0_.z) * vec4<f32>(_e156.g1_.y, _e159.g1_.x, _e162.g0_.w, _e165.g0_.z)) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))) + ((_e179.g0_.wwwx * _e182.g0_.xyzx) * vec4<f32>(-(1.0), -(1.0), -(1.0), 0.0))));
}

fn translator_flector_regressive_product(self_2214: Translator, other_1932: Flector) -> Flector {
    var self_2215: Translator;
    var other_1933: Flector;

    self_2215 = self_2214;
    other_1933 = other_1932;
    let _e4: Translator = self_2215;
    let _e8: Flector = other_1933;
    let _e19: Translator = self_2215;
    let _e23: Flector = other_1933;
    let _e35: Translator = self_2215;
    let _e39: Flector = other_1933;
    let _e43: Translator = self_2215;
    let _e47: Flector = other_1933;
    let _e59: Translator = self_2215;
    let _e63: Flector = other_1933;
    return Flector((((((vec4<f32>(_e4.g0_.y) * _e8.g1_.zzxz) * vec4<f32>(1.0, 0.0, -(1.0), 0.0)) + ((vec4<f32>(_e19.g0_.z) * _e23.g1_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))) + (vec4<f32>(_e35.g0_.w) * _e39.g0_)) + ((vec4<f32>(_e43.g0_.x) * _e47.g1_.xzyx) * vec4<f32>(0.0, -(1.0), 1.0, 0.0))), (vec4<f32>(_e59.g0_.w) * _e63.g1_));
}

fn translator_flector_anti_wedge(self_2216: Translator, other_1934: Flector) -> Flector {
    var self_2217: Translator;
    var other_1935: Flector;

    self_2217 = self_2216;
    other_1935 = other_1934;
    let _e4: Translator = self_2217;
    let _e8: Flector = other_1935;
    let _e19: Translator = self_2217;
    let _e23: Flector = other_1935;
    let _e35: Translator = self_2217;
    let _e39: Flector = other_1935;
    let _e43: Translator = self_2217;
    let _e47: Flector = other_1935;
    let _e59: Translator = self_2217;
    let _e63: Flector = other_1935;
    return Flector((((((vec4<f32>(_e4.g0_.y) * _e8.g1_.zzxz) * vec4<f32>(1.0, 0.0, -(1.0), 0.0)) + ((vec4<f32>(_e19.g0_.z) * _e23.g1_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))) + (vec4<f32>(_e35.g0_.w) * _e39.g0_)) + ((vec4<f32>(_e43.g0_.x) * _e47.g1_.xzyx) * vec4<f32>(0.0, -(1.0), 1.0, 0.0))), (vec4<f32>(_e59.g0_.w) * _e63.g1_));
}

fn translator_flector_meet(self_2218: Translator, other_1936: Flector) -> Flector {
    var self_2219: Translator;
    var other_1937: Flector;

    self_2219 = self_2218;
    other_1937 = other_1936;
    let _e4: Translator = self_2219;
    let _e8: Flector = other_1937;
    let _e19: Translator = self_2219;
    let _e23: Flector = other_1937;
    let _e35: Translator = self_2219;
    let _e39: Flector = other_1937;
    let _e43: Translator = self_2219;
    let _e47: Flector = other_1937;
    let _e59: Translator = self_2219;
    let _e63: Flector = other_1937;
    return Flector((((((vec4<f32>(_e4.g0_.y) * _e8.g1_.zzxz) * vec4<f32>(1.0, 0.0, -(1.0), 0.0)) + ((vec4<f32>(_e19.g0_.z) * _e23.g1_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))) + (vec4<f32>(_e35.g0_.w) * _e39.g0_)) + ((vec4<f32>(_e43.g0_.x) * _e47.g1_.xzyx) * vec4<f32>(0.0, -(1.0), 1.0, 0.0))), (vec4<f32>(_e59.g0_.w) * _e63.g1_));
}

fn translator_flector_outer_product(self_2220: Translator, other_1938: Flector) -> Plane {
    var self_2221: Translator;
    var other_1939: Flector;

    self_2221 = self_2220;
    other_1939 = other_1938;
    let _e4: Translator = self_2221;
    let _e8: Flector = other_1939;
    let _e19: Translator = self_2221;
    let _e23: Flector = other_1939;
    let _e35: Translator = self_2221;
    let _e39: Flector = other_1939;
    return Plane(((((vec4<f32>(_e4.g0_.y) * _e8.g0_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e35.g0_.x) * _e39.g0_.wxxx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))));
}

fn translator_flector_wedge(self_2222: Translator, other_1940: Flector) -> Plane {
    var self_2223: Translator;
    var other_1941: Flector;

    self_2223 = self_2222;
    other_1941 = other_1940;
    let _e4: Translator = self_2223;
    let _e8: Flector = other_1941;
    let _e19: Translator = self_2223;
    let _e23: Flector = other_1941;
    let _e35: Translator = self_2223;
    let _e39: Flector = other_1941;
    return Plane(((((vec4<f32>(_e4.g0_.y) * _e8.g0_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e35.g0_.x) * _e39.g0_.wxxx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))));
}

fn translator_flector_join(self_2224: Translator, other_1942: Flector) -> Plane {
    var self_2225: Translator;
    var other_1943: Flector;

    self_2225 = self_2224;
    other_1943 = other_1942;
    let _e4: Translator = self_2225;
    let _e8: Flector = other_1943;
    let _e19: Translator = self_2225;
    let _e23: Flector = other_1943;
    let _e35: Translator = self_2225;
    let _e39: Flector = other_1943;
    return Plane(((((vec4<f32>(_e4.g0_.y) * _e8.g0_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e35.g0_.x) * _e39.g0_.wxxx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))));
}

fn translator_flector_geometric_anti_product(self_2226: Translator, other_1944: Flector) -> Flector {
    var self_2227: Translator;
    var other_1945: Flector;

    self_2227 = self_2226;
    other_1945 = other_1944;
    let _e4: Translator = self_2227;
    let _e8: Flector = other_1945;
    let _e11: Flector = other_1945;
    let _e14: Flector = other_1945;
    let _e17: Flector = other_1945;
    let _e29: Translator = self_2227;
    let _e33: Flector = other_1945;
    let _e36: Flector = other_1945;
    let _e39: Flector = other_1945;
    let _e42: Flector = other_1945;
    let _e55: Translator = self_2227;
    let _e59: Flector = other_1945;
    let _e63: Translator = self_2227;
    let _e67: Flector = other_1945;
    let _e70: Flector = other_1945;
    let _e73: Flector = other_1945;
    let _e76: Flector = other_1945;
    let _e89: Translator = self_2227;
    let _e93: Flector = other_1945;
    let _e105: Translator = self_2227;
    let _e109: Flector = other_1945;
    let _e122: Translator = self_2227;
    let _e126: Flector = other_1945;
    let _e130: Translator = self_2227;
    let _e134: Flector = other_1945;
    return Flector((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g1_.z, _e11.g0_.w, _e14.g1_.x, _e17.g1_.z)) * vec4<f32>(1.0, 1.0, -(1.0), 0.0)) + ((vec4<f32>(_e29.g0_.z) * vec4<f32>(_e33.g1_.y, _e36.g1_.x, _e39.g0_.w, _e42.g1_.y)) * vec4<f32>(-(1.0), 1.0, 1.0, 0.0))) + (vec4<f32>(_e55.g0_.w) * _e59.g0_)) + ((vec4<f32>(_e63.g0_.x) * vec4<f32>(_e67.g0_.w, _e70.g1_.z, _e73.g1_.y, _e76.g0_.x)) * vec4<f32>(1.0, -(1.0), 1.0, 0.0))), (((((vec4<f32>(_e89.g0_.y) * vec4<f32>(_e93.g1_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0))) + ((vec4<f32>(_e105.g0_.z) * vec4<f32>(_e109.g1_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + (vec4<f32>(_e122.g0_.w) * _e126.g1_)) + ((vec4<f32>(_e130.g0_.x) * vec4<f32>(_e134.g1_.x)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))));
}

fn translator_flector_inner_anti_product(self_2228: Translator, other_1946: Flector) -> Flector {
    var self_2229: Translator;
    var other_1947: Flector;

    self_2229 = self_2228;
    other_1947 = other_1946;
    let _e4: Translator = self_2229;
    let _e8: Flector = other_1947;
    let _e11: Translator = self_2229;
    let _e15: Flector = other_1947;
    let _e27: Translator = self_2229;
    let _e31: Flector = other_1947;
    let _e44: Translator = self_2229;
    let _e48: Flector = other_1947;
    let _e52: Translator = self_2229;
    let _e56: Flector = other_1947;
    return Flector((vec4<f32>(_e4.g0_.w) * _e8.g0_), (((((vec4<f32>(_e11.g0_.y) * vec4<f32>(_e15.g1_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0))) + ((vec4<f32>(_e27.g0_.z) * vec4<f32>(_e31.g1_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + (vec4<f32>(_e44.g0_.w) * _e48.g1_)) + ((vec4<f32>(_e52.g0_.x) * vec4<f32>(_e56.g1_.x)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))));
}

fn translator_flector_left_contraction(self_2230: Translator, other_1948: Flector) -> Point {
    var self_2231: Translator;
    var other_1949: Flector;

    self_2231 = self_2230;
    other_1949 = other_1948;
    let _e4: Translator = self_2231;
    let _e8: Flector = other_1949;
    let _e19: Translator = self_2231;
    let _e23: Flector = other_1949;
    let _e35: Translator = self_2231;
    let _e39: Flector = other_1949;
    return Point(((((vec4<f32>(_e4.g0_.y) * _e8.g1_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e19.g0_.z) * _e23.g1_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e35.g0_.x) * _e39.g1_.wxxx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))));
}

fn translator_flector_left_anti_contraction(self_2232: Translator, other_1950: Flector) -> Flector {
    var self_2233: Translator;
    var other_1951: Flector;

    self_2233 = self_2232;
    other_1951 = other_1950;
    let _e4: Translator = self_2233;
    let _e8: Flector = other_1951;
    let _e11: Translator = self_2233;
    let _e15: Flector = other_1951;
    return Flector((vec4<f32>(_e4.g0_.w) * _e8.g0_), (vec4<f32>(_e11.g0_.w) * _e15.g1_));
}

fn translator_multi_vector_scalar_product(self_2234: Translator, other_1952: MultiVector) -> Scalar {
    var self_2235: Translator;
    var other_1953: MultiVector;

    self_2235 = self_2234;
    other_1953 = other_1952;
    let _e5: Translator = self_2235;
    let _e8: MultiVector = other_1953;
    let _e13: Translator = self_2235;
    let _e16: MultiVector = other_1953;
    let _e21: Translator = self_2235;
    let _e24: MultiVector = other_1953;
    return Scalar((((0.0 - (_e5.g0_.x * _e8.g3_.x)) - (_e13.g0_.y * _e16.g3_.y)) - (_e21.g0_.z * _e24.g3_.z)));
}

fn translator_multi_vector_dot(self_2236: Translator, other_1954: MultiVector) -> Scalar {
    var self_2237: Translator;
    var other_1955: MultiVector;

    self_2237 = self_2236;
    other_1955 = other_1954;
    let _e5: Translator = self_2237;
    let _e8: MultiVector = other_1955;
    let _e13: Translator = self_2237;
    let _e16: MultiVector = other_1955;
    let _e21: Translator = self_2237;
    let _e24: MultiVector = other_1955;
    return Scalar((((0.0 - (_e5.g0_.x * _e8.g3_.x)) - (_e13.g0_.y * _e16.g3_.y)) - (_e21.g0_.z * _e24.g3_.z)));
}

fn translator_multi_vector_anti_scalar_product(self_2238: Translator, other_1956: MultiVector) -> AntiScalar {
    var self_2239: Translator;
    var other_1957: MultiVector;

    self_2239 = self_2238;
    other_1957 = other_1956;
    let _e4: Translator = self_2239;
    let _e7: MultiVector = other_1957;
    return AntiScalar((_e4.g0_.w * _e7.g0_.y));
}

fn translator_multi_vector_anti_dot(self_2240: Translator, other_1958: MultiVector) -> AntiScalar {
    var self_2241: Translator;
    var other_1959: MultiVector;

    self_2241 = self_2240;
    other_1959 = other_1958;
    let _e4: Translator = self_2241;
    let _e7: MultiVector = other_1959;
    return AntiScalar((_e4.g0_.w * _e7.g0_.y));
}

fn translator_squared_magnitude(self_2242: Translator) -> Scalar {
    var self_2243: Translator;

    self_2243 = self_2242;
    let _e2: Translator = self_2243;
    let _e3: Translator = self_2243;
    let _e4: Translator = translator_reversal(_e3);
    let _e5: Scalar = translator_translator_scalar_product(_e2, _e4);
    return _e5;
}

fn translator_magnitude(self_2244: Translator) -> Scalar {
    var self_2245: Translator;

    self_2245 = self_2244;
    let _e2: Translator = self_2245;
    let _e3: Scalar = translator_squared_magnitude(_e2);
    return Scalar(sqrt(_e3.g0_));
}

fn translator_bulk_norm(self_2246: Translator) -> Scalar {
    var self_2247: Translator;

    self_2247 = self_2246;
    let _e2: Translator = self_2247;
    let _e3: Scalar = translator_squared_magnitude(_e2);
    return Scalar(sqrt(_e3.g0_));
}

fn translator_squared_anti_magnitude(self_2248: Translator) -> AntiScalar {
    var self_2249: Translator;

    self_2249 = self_2248;
    let _e2: Translator = self_2249;
    let _e3: Translator = self_2249;
    let _e4: Translator = translator_anti_reversal(_e3);
    let _e5: AntiScalar = translator_translator_anti_scalar_product(_e2, _e4);
    return _e5;
}

fn translator_weight_norm(self_2250: Translator) -> AntiScalar {
    var self_2251: Translator;

    self_2251 = self_2250;
    let _e2: Translator = self_2251;
    let _e3: AntiScalar = translator_squared_anti_magnitude(_e2);
    return AntiScalar(sqrt(_e3.g0_));
}

fn translator_geometric_norm(self_2252: Translator) -> HomogeneousMagnitude {
    var self_2253: Translator;

    self_2253 = self_2252;
    let _e2: Translator = self_2253;
    let _e3: Scalar = translator_bulk_norm(_e2);
    let _e4: Translator = self_2253;
    let _e5: AntiScalar = translator_weight_norm(_e4);
    let _e6: HomogeneousMagnitude = scalar_anti_scalar_add(_e3, _e5);
    return _e6;
}

fn translator_scale(self_2254: Translator, other_1960: f32) -> Translator {
    var self_2255: Translator;
    var other_1961: f32;

    self_2255 = self_2254;
    other_1961 = other_1960;
    let _e4: Translator = self_2255;
    let _e5: f32 = other_1961;
    let _e7: Translator = translator_scalar_geometric_product(_e4, Scalar(_e5));
    return _e7;
}

fn translator_signum(self_2256: Translator) -> Translator {
    var self_2257: Translator;

    self_2257 = self_2256;
    let _e2: Translator = self_2257;
    let _e3: Translator = self_2257;
    let _e4: Scalar = translator_magnitude(_e3);
    let _e9: Translator = translator_scalar_geometric_product(_e2, Scalar((1.0 / _e4.g0_)));
    return _e9;
}

fn translator_inverse(self_2258: Translator) -> Translator {
    var self_2259: Translator;

    self_2259 = self_2258;
    let _e2: Translator = self_2259;
    let _e3: Translator = translator_reversal(_e2);
    let _e4: Translator = self_2259;
    let _e5: Scalar = translator_squared_magnitude(_e4);
    let _e10: Translator = translator_scalar_geometric_product(_e3, Scalar((1.0 / _e5.g0_)));
    return _e10;
}

fn translator_unitize(self_2260: Translator) -> Translator {
    var self_2261: Translator;

    self_2261 = self_2260;
    let _e2: Translator = self_2261;
    let _e3: Translator = self_2261;
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

fn flector_neg(self_2262: Flector) -> Flector {
    var self_2263: Flector;

    self_2263 = self_2262;
    let _e2: Flector = self_2263;
    let _e8: Flector = self_2263;
    return Flector((_e2.g0_ * vec4<f32>(-(1.0))), (_e8.g1_ * vec4<f32>(-(1.0))));
}

fn flector_automorphism(self_2264: Flector) -> Flector {
    var self_2265: Flector;

    self_2265 = self_2264;
    let _e2: Flector = self_2265;
    let _e8: Flector = self_2265;
    return Flector((_e2.g0_ * vec4<f32>(-(1.0))), (_e8.g1_ * vec4<f32>(-(1.0))));
}

fn flector_reversal(self_2266: Flector) -> Flector {
    var self_2267: Flector;

    self_2267 = self_2266;
    let _e2: Flector = self_2267;
    let _e4: Flector = self_2267;
    return Flector(_e2.g0_, (_e4.g1_ * vec4<f32>(-(1.0))));
}

fn flector_conjugation(self_2268: Flector) -> Flector {
    var self_2269: Flector;

    self_2269 = self_2268;
    let _e2: Flector = self_2269;
    let _e8: Flector = self_2269;
    return Flector((_e2.g0_ * vec4<f32>(-(1.0))), _e8.g1_);
}

fn flector_dual(self_2270: Flector) -> Flector {
    var self_2271: Flector;

    self_2271 = self_2270;
    let _e2: Flector = self_2271;
    let _e8: Flector = self_2271;
    return Flector((_e2.g1_ * vec4<f32>(-(1.0))), _e8.g0_);
}

fn flector_anti_reversal(self_2272: Flector) -> Flector {
    var self_2273: Flector;

    self_2273 = self_2272;
    let _e2: Flector = self_2273;
    let _e8: Flector = self_2273;
    return Flector((_e2.g0_ * vec4<f32>(-(1.0))), _e8.g1_);
}

fn flector_right_complement(self_2274: Flector) -> Flector {
    var self_2275: Flector;

    self_2275 = self_2274;
    let _e2: Flector = self_2275;
    let _e8: Flector = self_2275;
    return Flector((_e2.g1_ * vec4<f32>(-(1.0))), _e8.g0_);
}

fn flector_left_complement(self_2276: Flector) -> Flector {
    var self_2277: Flector;

    self_2277 = self_2276;
    let _e2: Flector = self_2277;
    let _e4: Flector = self_2277;
    return Flector(_e2.g1_, (_e4.g0_ * vec4<f32>(-(1.0))));
}

fn flector_double_complement(self_2278: Flector) -> Flector {
    var self_2279: Flector;

    self_2279 = self_2278;
    let _e2: Flector = self_2279;
    let _e8: Flector = self_2279;
    return Flector((_e2.g0_ * vec4<f32>(-(1.0))), (_e8.g1_ * vec4<f32>(-(1.0))));
}

fn flector_scalar_geometric_product(self_2280: Flector, other_1962: Scalar) -> Flector {
    var self_2281: Flector;
    var other_1963: Scalar;

    self_2281 = self_2280;
    other_1963 = other_1962;
    let _e4: Flector = self_2281;
    let _e6: Scalar = other_1963;
    let _e10: Flector = self_2281;
    let _e12: Scalar = other_1963;
    return Flector((_e4.g0_ * vec4<f32>(_e6.g0_)), (_e10.g1_ * vec4<f32>(_e12.g0_)));
}

fn flector_scalar_outer_product(self_2282: Flector, other_1964: Scalar) -> Flector {
    var self_2283: Flector;
    var other_1965: Scalar;

    self_2283 = self_2282;
    other_1965 = other_1964;
    let _e4: Flector = self_2283;
    let _e6: Scalar = other_1965;
    let _e10: Flector = self_2283;
    let _e12: Scalar = other_1965;
    return Flector((_e4.g0_ * vec4<f32>(_e6.g0_)), (_e10.g1_ * vec4<f32>(_e12.g0_)));
}

fn flector_scalar_wedge(self_2284: Flector, other_1966: Scalar) -> Flector {
    var self_2285: Flector;
    var other_1967: Scalar;

    self_2285 = self_2284;
    other_1967 = other_1966;
    let _e4: Flector = self_2285;
    let _e6: Scalar = other_1967;
    let _e10: Flector = self_2285;
    let _e12: Scalar = other_1967;
    return Flector((_e4.g0_ * vec4<f32>(_e6.g0_)), (_e10.g1_ * vec4<f32>(_e12.g0_)));
}

fn flector_scalar_join(self_2286: Flector, other_1968: Scalar) -> Flector {
    var self_2287: Flector;
    var other_1969: Scalar;

    self_2287 = self_2286;
    other_1969 = other_1968;
    let _e4: Flector = self_2287;
    let _e6: Scalar = other_1969;
    let _e10: Flector = self_2287;
    let _e12: Scalar = other_1969;
    return Flector((_e4.g0_ * vec4<f32>(_e6.g0_)), (_e10.g1_ * vec4<f32>(_e12.g0_)));
}

fn flector_scalar_inner_product(self_2288: Flector, other_1970: Scalar) -> Flector {
    var self_2289: Flector;
    var other_1971: Scalar;

    self_2289 = self_2288;
    other_1971 = other_1970;
    let _e4: Flector = self_2289;
    let _e6: Scalar = other_1971;
    let _e10: Flector = self_2289;
    let _e12: Scalar = other_1971;
    return Flector((_e4.g0_ * vec4<f32>(_e6.g0_)), (_e10.g1_ * vec4<f32>(_e12.g0_)));
}

fn flector_scalar_right_contraction(self_2290: Flector, other_1972: Scalar) -> Flector {
    var self_2291: Flector;
    var other_1973: Scalar;

    self_2291 = self_2290;
    other_1973 = other_1972;
    let _e4: Flector = self_2291;
    let _e6: Scalar = other_1973;
    let _e10: Flector = self_2291;
    let _e12: Scalar = other_1973;
    return Flector((_e4.g0_ * vec4<f32>(_e6.g0_)), (_e10.g1_ * vec4<f32>(_e12.g0_)));
}

fn flector_anti_scalar_regressive_product(self_2292: Flector, other_1974: AntiScalar) -> Flector {
    var self_2293: Flector;
    var other_1975: AntiScalar;

    self_2293 = self_2292;
    other_1975 = other_1974;
    let _e4: Flector = self_2293;
    let _e6: AntiScalar = other_1975;
    let _e10: Flector = self_2293;
    let _e12: AntiScalar = other_1975;
    return Flector((_e4.g0_ * vec4<f32>(_e6.g0_)), (_e10.g1_ * vec4<f32>(_e12.g0_)));
}

fn flector_anti_scalar_anti_wedge(self_2294: Flector, other_1976: AntiScalar) -> Flector {
    var self_2295: Flector;
    var other_1977: AntiScalar;

    self_2295 = self_2294;
    other_1977 = other_1976;
    let _e4: Flector = self_2295;
    let _e6: AntiScalar = other_1977;
    let _e10: Flector = self_2295;
    let _e12: AntiScalar = other_1977;
    return Flector((_e4.g0_ * vec4<f32>(_e6.g0_)), (_e10.g1_ * vec4<f32>(_e12.g0_)));
}

fn flector_anti_scalar_meet(self_2296: Flector, other_1978: AntiScalar) -> Flector {
    var self_2297: Flector;
    var other_1979: AntiScalar;

    self_2297 = self_2296;
    other_1979 = other_1978;
    let _e4: Flector = self_2297;
    let _e6: AntiScalar = other_1979;
    let _e10: Flector = self_2297;
    let _e12: AntiScalar = other_1979;
    return Flector((_e4.g0_ * vec4<f32>(_e6.g0_)), (_e10.g1_ * vec4<f32>(_e12.g0_)));
}

fn flector_anti_scalar_geometric_anti_product(self_2298: Flector, other_1980: AntiScalar) -> Flector {
    var self_2299: Flector;
    var other_1981: AntiScalar;

    self_2299 = self_2298;
    other_1981 = other_1980;
    let _e4: Flector = self_2299;
    let _e6: AntiScalar = other_1981;
    let _e10: Flector = self_2299;
    let _e12: AntiScalar = other_1981;
    return Flector((_e4.g0_ * vec4<f32>(_e6.g0_)), (_e10.g1_ * vec4<f32>(_e12.g0_)));
}

fn flector_anti_scalar_inner_anti_product(self_2300: Flector, other_1982: AntiScalar) -> Flector {
    var self_2301: Flector;
    var other_1983: AntiScalar;

    self_2301 = self_2300;
    other_1983 = other_1982;
    let _e4: Flector = self_2301;
    let _e6: AntiScalar = other_1983;
    let _e10: Flector = self_2301;
    let _e12: AntiScalar = other_1983;
    return Flector((_e4.g0_ * vec4<f32>(_e6.g0_)), (_e10.g1_ * vec4<f32>(_e12.g0_)));
}

fn flector_anti_scalar_right_anti_contraction(self_2302: Flector, other_1984: AntiScalar) -> Flector {
    var self_2303: Flector;
    var other_1985: AntiScalar;

    self_2303 = self_2302;
    other_1985 = other_1984;
    let _e4: Flector = self_2303;
    let _e6: AntiScalar = other_1985;
    let _e10: Flector = self_2303;
    let _e12: AntiScalar = other_1985;
    return Flector((_e4.g0_ * vec4<f32>(_e6.g0_)), (_e10.g1_ * vec4<f32>(_e12.g0_)));
}

fn flector_homogeneous_magnitude_geometric_product(self_2304: Flector, other_1986: HomogeneousMagnitude) -> Flector {
    var self_2305: Flector;
    var other_1987: HomogeneousMagnitude;

    self_2305 = self_2304;
    other_1987 = other_1986;
    let _e4: Flector = self_2305;
    let _e8: HomogeneousMagnitude = other_1987;
    let _e19: Flector = self_2305;
    let _e21: HomogeneousMagnitude = other_1987;
    let _e27: Flector = self_2305;
    let _e31: HomogeneousMagnitude = other_1987;
    let _e42: Flector = self_2305;
    let _e46: HomogeneousMagnitude = other_1987;
    let _e58: Flector = self_2305;
    let _e62: HomogeneousMagnitude = other_1987;
    let _e74: Flector = self_2305;
    let _e77: Flector = self_2305;
    let _e80: Flector = self_2305;
    let _e83: Flector = self_2305;
    let _e87: HomogeneousMagnitude = other_1987;
    let _e90: HomogeneousMagnitude = other_1987;
    let _e93: HomogeneousMagnitude = other_1987;
    let _e96: HomogeneousMagnitude = other_1987;
    return Flector((((vec4<f32>(_e4.g1_.w) * vec4<f32>(_e8.g0_.y)) * vec4<f32>(0.0, 0.0, 0.0, 1.0)) + (_e19.g0_ * vec4<f32>(_e21.g0_.x))), (((((vec4<f32>(_e27.g1_.x) * vec4<f32>(_e31.g0_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) + ((vec4<f32>(_e42.g1_.y) * vec4<f32>(_e46.g0_.x)) * vec4<f32>(0.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e58.g1_.z) * vec4<f32>(_e62.g0_.x)) * vec4<f32>(0.0, 0.0, 1.0, 0.0))) + (vec4<f32>(_e74.g0_.x, _e77.g0_.y, _e80.g0_.z, _e83.g1_.w) * vec4<f32>(_e87.g0_.y, _e90.g0_.y, _e93.g0_.y, _e96.g0_.x))));
}

fn flector_homogeneous_magnitude_regressive_product(self_2306: Flector, other_1988: HomogeneousMagnitude) -> Flector {
    var self_2307: Flector;
    var other_1989: HomogeneousMagnitude;

    self_2307 = self_2306;
    other_1989 = other_1988;
    let _e4: Flector = self_2307;
    let _e6: HomogeneousMagnitude = other_1989;
    let _e11: Flector = self_2307;
    let _e13: HomogeneousMagnitude = other_1989;
    return Flector((_e4.g0_ * vec4<f32>(_e6.g0_.y)), (_e11.g1_ * vec4<f32>(_e13.g0_.y)));
}

fn flector_homogeneous_magnitude_anti_wedge(self_2308: Flector, other_1990: HomogeneousMagnitude) -> Flector {
    var self_2309: Flector;
    var other_1991: HomogeneousMagnitude;

    self_2309 = self_2308;
    other_1991 = other_1990;
    let _e4: Flector = self_2309;
    let _e6: HomogeneousMagnitude = other_1991;
    let _e11: Flector = self_2309;
    let _e13: HomogeneousMagnitude = other_1991;
    return Flector((_e4.g0_ * vec4<f32>(_e6.g0_.y)), (_e11.g1_ * vec4<f32>(_e13.g0_.y)));
}

fn flector_homogeneous_magnitude_meet(self_2310: Flector, other_1992: HomogeneousMagnitude) -> Flector {
    var self_2311: Flector;
    var other_1993: HomogeneousMagnitude;

    self_2311 = self_2310;
    other_1993 = other_1992;
    let _e4: Flector = self_2311;
    let _e6: HomogeneousMagnitude = other_1993;
    let _e11: Flector = self_2311;
    let _e13: HomogeneousMagnitude = other_1993;
    return Flector((_e4.g0_ * vec4<f32>(_e6.g0_.y)), (_e11.g1_ * vec4<f32>(_e13.g0_.y)));
}

fn flector_homogeneous_magnitude_outer_product(self_2312: Flector, other_1994: HomogeneousMagnitude) -> Flector {
    var self_2313: Flector;
    var other_1995: HomogeneousMagnitude;

    self_2313 = self_2312;
    other_1995 = other_1994;
    let _e4: Flector = self_2313;
    let _e6: HomogeneousMagnitude = other_1995;
    let _e11: Flector = self_2313;
    let _e13: HomogeneousMagnitude = other_1995;
    return Flector((_e4.g0_ * vec4<f32>(_e6.g0_.x)), (_e11.g1_ * vec4<f32>(_e13.g0_.x)));
}

fn flector_homogeneous_magnitude_wedge(self_2314: Flector, other_1996: HomogeneousMagnitude) -> Flector {
    var self_2315: Flector;
    var other_1997: HomogeneousMagnitude;

    self_2315 = self_2314;
    other_1997 = other_1996;
    let _e4: Flector = self_2315;
    let _e6: HomogeneousMagnitude = other_1997;
    let _e11: Flector = self_2315;
    let _e13: HomogeneousMagnitude = other_1997;
    return Flector((_e4.g0_ * vec4<f32>(_e6.g0_.x)), (_e11.g1_ * vec4<f32>(_e13.g0_.x)));
}

fn flector_homogeneous_magnitude_join(self_2316: Flector, other_1998: HomogeneousMagnitude) -> Flector {
    var self_2317: Flector;
    var other_1999: HomogeneousMagnitude;

    self_2317 = self_2316;
    other_1999 = other_1998;
    let _e4: Flector = self_2317;
    let _e6: HomogeneousMagnitude = other_1999;
    let _e11: Flector = self_2317;
    let _e13: HomogeneousMagnitude = other_1999;
    return Flector((_e4.g0_ * vec4<f32>(_e6.g0_.x)), (_e11.g1_ * vec4<f32>(_e13.g0_.x)));
}

fn flector_homogeneous_magnitude_inner_product(self_2318: Flector, other_2000: HomogeneousMagnitude) -> Flector {
    var self_2319: Flector;
    var other_2001: HomogeneousMagnitude;

    self_2319 = self_2318;
    other_2001 = other_2000;
    let _e4: Flector = self_2319;
    let _e8: HomogeneousMagnitude = other_2001;
    let _e19: Flector = self_2319;
    let _e21: HomogeneousMagnitude = other_2001;
    let _e27: Flector = self_2319;
    let _e31: HomogeneousMagnitude = other_2001;
    let _e42: Flector = self_2319;
    let _e46: HomogeneousMagnitude = other_2001;
    let _e58: Flector = self_2319;
    let _e62: HomogeneousMagnitude = other_2001;
    let _e74: Flector = self_2319;
    let _e77: Flector = self_2319;
    let _e80: Flector = self_2319;
    let _e83: Flector = self_2319;
    let _e87: HomogeneousMagnitude = other_2001;
    let _e90: HomogeneousMagnitude = other_2001;
    let _e93: HomogeneousMagnitude = other_2001;
    let _e96: HomogeneousMagnitude = other_2001;
    return Flector((((vec4<f32>(_e4.g1_.w) * vec4<f32>(_e8.g0_.y)) * vec4<f32>(0.0, 0.0, 0.0, 1.0)) + (_e19.g0_ * vec4<f32>(_e21.g0_.x))), (((((vec4<f32>(_e27.g1_.x) * vec4<f32>(_e31.g0_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) + ((vec4<f32>(_e42.g1_.y) * vec4<f32>(_e46.g0_.x)) * vec4<f32>(0.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e58.g1_.z) * vec4<f32>(_e62.g0_.x)) * vec4<f32>(0.0, 0.0, 1.0, 0.0))) + (vec4<f32>(_e74.g0_.x, _e77.g0_.y, _e80.g0_.z, _e83.g1_.w) * vec4<f32>(_e87.g0_.y, _e90.g0_.y, _e93.g0_.y, _e96.g0_.x))));
}

fn flector_homogeneous_magnitude_geometric_anti_product(self_2320: Flector, other_2002: HomogeneousMagnitude) -> Flector {
    var self_2321: Flector;
    var other_2003: HomogeneousMagnitude;

    self_2321 = self_2320;
    other_2003 = other_2002;
    let _e4: Flector = self_2321;
    let _e8: HomogeneousMagnitude = other_2003;
    let _e20: Flector = self_2321;
    let _e24: HomogeneousMagnitude = other_2003;
    let _e37: Flector = self_2321;
    let _e41: HomogeneousMagnitude = other_2003;
    let _e54: Flector = self_2321;
    let _e56: HomogeneousMagnitude = other_2003;
    let _e62: Flector = self_2321;
    let _e66: HomogeneousMagnitude = other_2003;
    let _e77: Flector = self_2321;
    let _e80: Flector = self_2321;
    let _e83: Flector = self_2321;
    let _e86: Flector = self_2321;
    let _e90: HomogeneousMagnitude = other_2003;
    let _e93: HomogeneousMagnitude = other_2003;
    let _e96: HomogeneousMagnitude = other_2003;
    let _e99: HomogeneousMagnitude = other_2003;
    return Flector((((((vec4<f32>(_e4.g1_.x) * vec4<f32>(_e8.g0_.x)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0)) + ((vec4<f32>(_e20.g1_.y) * vec4<f32>(_e24.g0_.x)) * vec4<f32>(0.0, -(1.0), 0.0, 0.0))) + ((vec4<f32>(_e37.g1_.z) * vec4<f32>(_e41.g0_.x)) * vec4<f32>(0.0, 0.0, -(1.0), 0.0))) + (_e54.g0_ * vec4<f32>(_e56.g0_.y))), (((vec4<f32>(_e62.g1_.w) * vec4<f32>(_e66.g0_.y)) * vec4<f32>(0.0, 0.0, 0.0, 1.0)) + ((vec4<f32>(_e77.g1_.x, _e80.g1_.y, _e83.g1_.z, _e86.g0_.w) * vec4<f32>(_e90.g0_.y, _e93.g0_.y, _e96.g0_.y, _e99.g0_.x)) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))));
}

fn flector_homogeneous_magnitude_inner_anti_product(self_2322: Flector, other_2004: HomogeneousMagnitude) -> Flector {
    var self_2323: Flector;
    var other_2005: HomogeneousMagnitude;

    self_2323 = self_2322;
    other_2005 = other_2004;
    let _e4: Flector = self_2323;
    let _e8: HomogeneousMagnitude = other_2005;
    let _e20: Flector = self_2323;
    let _e24: HomogeneousMagnitude = other_2005;
    let _e37: Flector = self_2323;
    let _e41: HomogeneousMagnitude = other_2005;
    let _e54: Flector = self_2323;
    let _e56: HomogeneousMagnitude = other_2005;
    let _e62: Flector = self_2323;
    let _e66: HomogeneousMagnitude = other_2005;
    let _e77: Flector = self_2323;
    let _e80: Flector = self_2323;
    let _e83: Flector = self_2323;
    let _e86: Flector = self_2323;
    let _e90: HomogeneousMagnitude = other_2005;
    let _e93: HomogeneousMagnitude = other_2005;
    let _e96: HomogeneousMagnitude = other_2005;
    let _e99: HomogeneousMagnitude = other_2005;
    return Flector((((((vec4<f32>(_e4.g1_.x) * vec4<f32>(_e8.g0_.x)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0)) + ((vec4<f32>(_e20.g1_.y) * vec4<f32>(_e24.g0_.x)) * vec4<f32>(0.0, -(1.0), 0.0, 0.0))) + ((vec4<f32>(_e37.g1_.z) * vec4<f32>(_e41.g0_.x)) * vec4<f32>(0.0, 0.0, -(1.0), 0.0))) + (_e54.g0_ * vec4<f32>(_e56.g0_.y))), (((vec4<f32>(_e62.g1_.w) * vec4<f32>(_e66.g0_.y)) * vec4<f32>(0.0, 0.0, 0.0, 1.0)) + ((vec4<f32>(_e77.g1_.x, _e80.g1_.y, _e83.g1_.z, _e86.g0_.w) * vec4<f32>(_e90.g0_.y, _e93.g0_.y, _e96.g0_.y, _e99.g0_.x)) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))));
}

fn flector_homogeneous_magnitude_right_contraction(self_2324: Flector, other_2006: HomogeneousMagnitude) -> Flector {
    var self_2325: Flector;
    var other_2007: HomogeneousMagnitude;

    self_2325 = self_2324;
    other_2007 = other_2006;
    let _e4: Flector = self_2325;
    let _e6: HomogeneousMagnitude = other_2007;
    let _e11: Flector = self_2325;
    let _e13: HomogeneousMagnitude = other_2007;
    return Flector((_e4.g0_ * vec4<f32>(_e6.g0_.x)), (_e11.g1_ * vec4<f32>(_e13.g0_.x)));
}

fn flector_homogeneous_magnitude_right_anti_contraction(self_2326: Flector, other_2008: HomogeneousMagnitude) -> Flector {
    var self_2327: Flector;
    var other_2009: HomogeneousMagnitude;

    self_2327 = self_2326;
    other_2009 = other_2008;
    let _e4: Flector = self_2327;
    let _e6: HomogeneousMagnitude = other_2009;
    let _e11: Flector = self_2327;
    let _e13: HomogeneousMagnitude = other_2009;
    return Flector((_e4.g0_ * vec4<f32>(_e6.g0_.y)), (_e11.g1_ * vec4<f32>(_e13.g0_.y)));
}

fn flector_point_into(self_2328: Flector) -> Point {
    var self_2329: Flector;

    self_2329 = self_2328;
    let _e2: Flector = self_2329;
    return Point(_e2.g0_);
}

fn flector_point_add(self_2330: Flector, other_2010: Point) -> Flector {
    var self_2331: Flector;
    var other_2011: Point;

    self_2331 = self_2330;
    other_2011 = other_2010;
    let _e4: Flector = self_2331;
    let _e6: Point = other_2011;
    let _e9: Flector = self_2331;
    return Flector((_e4.g0_ + _e6.g0_), _e9.g1_);
}

fn flector_point_sub(self_2332: Flector, other_2012: Point) -> Flector {
    var self_2333: Flector;
    var other_2013: Point;

    self_2333 = self_2332;
    other_2013 = other_2012;
    let _e4: Flector = self_2333;
    let _e6: Point = other_2013;
    let _e9: Flector = self_2333;
    return Flector((_e4.g0_ - _e6.g0_), _e9.g1_);
}

fn flector_point_regressive_product(self_2334: Flector, other_2014: Point) -> Scalar {
    var self_2335: Flector;
    var other_2015: Point;

    self_2335 = self_2334;
    other_2015 = other_2014;
    let _e5: Flector = self_2335;
    let _e8: Point = other_2015;
    let _e13: Flector = self_2335;
    let _e16: Point = other_2015;
    let _e21: Flector = self_2335;
    let _e24: Point = other_2015;
    let _e29: Flector = self_2335;
    let _e32: Point = other_2015;
    return Scalar(((((0.0 - (_e5.g1_.x * _e8.g0_.x)) - (_e13.g1_.y * _e16.g0_.y)) - (_e21.g1_.z * _e24.g0_.z)) - (_e29.g1_.w * _e32.g0_.w)));
}

fn flector_point_anti_wedge(self_2336: Flector, other_2016: Point) -> Scalar {
    var self_2337: Flector;
    var other_2017: Point;

    self_2337 = self_2336;
    other_2017 = other_2016;
    let _e5: Flector = self_2337;
    let _e8: Point = other_2017;
    let _e13: Flector = self_2337;
    let _e16: Point = other_2017;
    let _e21: Flector = self_2337;
    let _e24: Point = other_2017;
    let _e29: Flector = self_2337;
    let _e32: Point = other_2017;
    return Scalar(((((0.0 - (_e5.g1_.x * _e8.g0_.x)) - (_e13.g1_.y * _e16.g0_.y)) - (_e21.g1_.z * _e24.g0_.z)) - (_e29.g1_.w * _e32.g0_.w)));
}

fn flector_point_meet(self_2338: Flector, other_2018: Point) -> Scalar {
    var self_2339: Flector;
    var other_2019: Point;

    self_2339 = self_2338;
    other_2019 = other_2018;
    let _e5: Flector = self_2339;
    let _e8: Point = other_2019;
    let _e13: Flector = self_2339;
    let _e16: Point = other_2019;
    let _e21: Flector = self_2339;
    let _e24: Point = other_2019;
    let _e29: Flector = self_2339;
    let _e32: Point = other_2019;
    return Scalar(((((0.0 - (_e5.g1_.x * _e8.g0_.x)) - (_e13.g1_.y * _e16.g0_.y)) - (_e21.g1_.z * _e24.g0_.z)) - (_e29.g1_.w * _e32.g0_.w)));
}

fn flector_point_outer_product(self_2340: Flector, other_2020: Point) -> Motor {
    var self_2341: Flector;
    var other_2021: Point;

    self_2341 = self_2340;
    other_2021 = other_2020;
    let _e4: Flector = self_2341;
    let _e8: Point = other_2021;
    let _e18: Flector = self_2341;
    let _e22: Point = other_2021;
    let _e35: Flector = self_2341;
    let _e39: Point = other_2021;
    let _e52: Flector = self_2341;
    let _e56: Point = other_2021;
    let _e69: Flector = self_2341;
    let _e72: Flector = self_2341;
    let _e75: Flector = self_2341;
    let _e78: Flector = self_2341;
    let _e82: Point = other_2021;
    let _e91: Flector = self_2341;
    let _e95: Point = other_2021;
    let _e98: Point = other_2021;
    let _e101: Point = other_2021;
    let _e112: Flector = self_2341;
    let _e116: Point = other_2021;
    let _e119: Point = other_2021;
    let _e122: Point = other_2021;
    let _e134: Flector = self_2341;
    let _e138: Point = other_2021;
    let _e141: Point = other_2021;
    let _e144: Point = other_2021;
    return Motor(((((((vec4<f32>(_e4.g0_.w) * _e8.g0_.xyzx) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) + ((vec4<f32>(_e18.g1_.y) * vec4<f32>(_e22.g0_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e35.g1_.z) * vec4<f32>(_e39.g0_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e52.g1_.w) * vec4<f32>(_e56.g0_.w)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e69.g0_.x, _e72.g0_.y, _e75.g0_.z, _e78.g1_.x) * _e82.g0_.wwwx) * vec4<f32>(-(1.0)))), ((((vec3<f32>(_e91.g0_.y) * vec3<f32>(_e95.g0_.z, _e98.g0_.z, _e101.g0_.x)) * vec3<f32>(1.0, 0.0, -(1.0))) + ((vec3<f32>(_e112.g0_.z) * vec3<f32>(_e116.g0_.y, _e119.g0_.x, _e122.g0_.y)) * vec3<f32>(-(1.0), 1.0, 0.0))) + ((vec3<f32>(_e134.g0_.x) * vec3<f32>(_e138.g0_.x, _e141.g0_.z, _e144.g0_.y)) * vec3<f32>(0.0, -(1.0), 1.0))));
}

fn flector_point_wedge(self_2342: Flector, other_2022: Point) -> Motor {
    var self_2343: Flector;
    var other_2023: Point;

    self_2343 = self_2342;
    other_2023 = other_2022;
    let _e4: Flector = self_2343;
    let _e8: Point = other_2023;
    let _e18: Flector = self_2343;
    let _e22: Point = other_2023;
    let _e35: Flector = self_2343;
    let _e39: Point = other_2023;
    let _e52: Flector = self_2343;
    let _e56: Point = other_2023;
    let _e69: Flector = self_2343;
    let _e72: Flector = self_2343;
    let _e75: Flector = self_2343;
    let _e78: Flector = self_2343;
    let _e82: Point = other_2023;
    let _e91: Flector = self_2343;
    let _e95: Point = other_2023;
    let _e98: Point = other_2023;
    let _e101: Point = other_2023;
    let _e112: Flector = self_2343;
    let _e116: Point = other_2023;
    let _e119: Point = other_2023;
    let _e122: Point = other_2023;
    let _e134: Flector = self_2343;
    let _e138: Point = other_2023;
    let _e141: Point = other_2023;
    let _e144: Point = other_2023;
    return Motor(((((((vec4<f32>(_e4.g0_.w) * _e8.g0_.xyzx) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) + ((vec4<f32>(_e18.g1_.y) * vec4<f32>(_e22.g0_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e35.g1_.z) * vec4<f32>(_e39.g0_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e52.g1_.w) * vec4<f32>(_e56.g0_.w)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e69.g0_.x, _e72.g0_.y, _e75.g0_.z, _e78.g1_.x) * _e82.g0_.wwwx) * vec4<f32>(-(1.0)))), ((((vec3<f32>(_e91.g0_.y) * vec3<f32>(_e95.g0_.z, _e98.g0_.z, _e101.g0_.x)) * vec3<f32>(1.0, 0.0, -(1.0))) + ((vec3<f32>(_e112.g0_.z) * vec3<f32>(_e116.g0_.y, _e119.g0_.x, _e122.g0_.y)) * vec3<f32>(-(1.0), 1.0, 0.0))) + ((vec3<f32>(_e134.g0_.x) * vec3<f32>(_e138.g0_.x, _e141.g0_.z, _e144.g0_.y)) * vec3<f32>(0.0, -(1.0), 1.0))));
}

fn flector_point_join(self_2344: Flector, other_2024: Point) -> Motor {
    var self_2345: Flector;
    var other_2025: Point;

    self_2345 = self_2344;
    other_2025 = other_2024;
    let _e4: Flector = self_2345;
    let _e8: Point = other_2025;
    let _e18: Flector = self_2345;
    let _e22: Point = other_2025;
    let _e35: Flector = self_2345;
    let _e39: Point = other_2025;
    let _e52: Flector = self_2345;
    let _e56: Point = other_2025;
    let _e69: Flector = self_2345;
    let _e72: Flector = self_2345;
    let _e75: Flector = self_2345;
    let _e78: Flector = self_2345;
    let _e82: Point = other_2025;
    let _e91: Flector = self_2345;
    let _e95: Point = other_2025;
    let _e98: Point = other_2025;
    let _e101: Point = other_2025;
    let _e112: Flector = self_2345;
    let _e116: Point = other_2025;
    let _e119: Point = other_2025;
    let _e122: Point = other_2025;
    let _e134: Flector = self_2345;
    let _e138: Point = other_2025;
    let _e141: Point = other_2025;
    let _e144: Point = other_2025;
    return Motor(((((((vec4<f32>(_e4.g0_.w) * _e8.g0_.xyzx) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) + ((vec4<f32>(_e18.g1_.y) * vec4<f32>(_e22.g0_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e35.g1_.z) * vec4<f32>(_e39.g0_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e52.g1_.w) * vec4<f32>(_e56.g0_.w)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e69.g0_.x, _e72.g0_.y, _e75.g0_.z, _e78.g1_.x) * _e82.g0_.wwwx) * vec4<f32>(-(1.0)))), ((((vec3<f32>(_e91.g0_.y) * vec3<f32>(_e95.g0_.z, _e98.g0_.z, _e101.g0_.x)) * vec3<f32>(1.0, 0.0, -(1.0))) + ((vec3<f32>(_e112.g0_.z) * vec3<f32>(_e116.g0_.y, _e119.g0_.x, _e122.g0_.y)) * vec3<f32>(-(1.0), 1.0, 0.0))) + ((vec3<f32>(_e134.g0_.x) * vec3<f32>(_e138.g0_.x, _e141.g0_.z, _e144.g0_.y)) * vec3<f32>(0.0, -(1.0), 1.0))));
}

fn flector_point_inner_anti_product(self_2346: Flector, other_2026: Point) -> Motor {
    var self_2347: Flector;
    var other_2027: Point;

    self_2347 = self_2346;
    other_2027 = other_2026;
    let _e4: Flector = self_2347;
    let _e7: Flector = self_2347;
    let _e10: Flector = self_2347;
    let _e13: Flector = self_2347;
    let _e17: Point = other_2027;
    let _e26: Flector = self_2347;
    let _e30: Point = other_2027;
    let _e33: Point = other_2027;
    let _e36: Point = other_2027;
    let _e47: Flector = self_2347;
    let _e51: Point = other_2027;
    let _e54: Point = other_2027;
    let _e57: Point = other_2027;
    let _e69: Flector = self_2347;
    let _e73: Point = other_2027;
    let _e76: Point = other_2027;
    let _e79: Point = other_2027;
    return Motor(((vec4<f32>(_e4.g1_.x, _e7.g1_.y, _e10.g1_.z, _e13.g0_.w) * vec4<f32>(_e17.g0_.w)) * vec4<f32>(-(1.0))), ((((vec3<f32>(_e26.g1_.y) * vec3<f32>(_e30.g0_.z, _e33.g0_.z, _e36.g0_.x)) * vec3<f32>(1.0, 0.0, -(1.0))) + ((vec3<f32>(_e47.g1_.z) * vec3<f32>(_e51.g0_.y, _e54.g0_.x, _e57.g0_.y)) * vec3<f32>(-(1.0), 1.0, 0.0))) + ((vec3<f32>(_e69.g1_.x) * vec3<f32>(_e73.g0_.x, _e76.g0_.z, _e79.g0_.y)) * vec3<f32>(0.0, -(1.0), 1.0))));
}

fn flector_point_left_contraction(self_2348: Flector, other_2028: Point) -> Scalar {
    var self_2349: Flector;
    var other_2029: Point;

    self_2349 = self_2348;
    other_2029 = other_2028;
    let _e4: Flector = self_2349;
    let _e7: Point = other_2029;
    let _e11: Flector = self_2349;
    let _e14: Point = other_2029;
    let _e19: Flector = self_2349;
    let _e22: Point = other_2029;
    return Scalar((((_e4.g0_.x * _e7.g0_.x) + (_e11.g0_.y * _e14.g0_.y)) + (_e19.g0_.z * _e22.g0_.z)));
}

fn flector_point_left_anti_contraction(self_2350: Flector, other_2030: Point) -> Motor {
    var self_2351: Flector;
    var other_2031: Point;

    self_2351 = self_2350;
    other_2031 = other_2030;
    let _e4: Flector = self_2351;
    let _e7: Flector = self_2351;
    let _e10: Flector = self_2351;
    let _e13: Flector = self_2351;
    let _e17: Point = other_2031;
    let _e26: Flector = self_2351;
    let _e30: Point = other_2031;
    let _e33: Point = other_2031;
    let _e36: Point = other_2031;
    let _e47: Flector = self_2351;
    let _e51: Point = other_2031;
    let _e54: Point = other_2031;
    let _e57: Point = other_2031;
    let _e69: Flector = self_2351;
    let _e73: Point = other_2031;
    let _e76: Point = other_2031;
    let _e79: Point = other_2031;
    return Motor(((vec4<f32>(_e4.g1_.x, _e7.g1_.y, _e10.g1_.z, _e13.g0_.w) * vec4<f32>(_e17.g0_.w)) * vec4<f32>(-(1.0))), ((((vec3<f32>(_e26.g1_.y) * vec3<f32>(_e30.g0_.z, _e33.g0_.z, _e36.g0_.x)) * vec3<f32>(1.0, 0.0, -(1.0))) + ((vec3<f32>(_e47.g1_.z) * vec3<f32>(_e51.g0_.y, _e54.g0_.x, _e57.g0_.y)) * vec3<f32>(-(1.0), 1.0, 0.0))) + ((vec3<f32>(_e69.g1_.x) * vec3<f32>(_e73.g0_.x, _e76.g0_.z, _e79.g0_.y)) * vec3<f32>(0.0, -(1.0), 1.0))));
}

fn flector_point_right_anti_contraction(self_2352: Flector, other_2032: Point) -> AntiScalar {
    var self_2353: Flector;
    var other_2033: Point;

    self_2353 = self_2352;
    other_2033 = other_2032;
    let _e5: Flector = self_2353;
    let _e8: Point = other_2033;
    return AntiScalar((0.0 - (_e5.g0_.w * _e8.g0_.w)));
}

fn flector_point_scalar_product(self_2354: Flector, other_2034: Point) -> Scalar {
    var self_2355: Flector;
    var other_2035: Point;

    self_2355 = self_2354;
    other_2035 = other_2034;
    let _e4: Flector = self_2355;
    let _e7: Point = other_2035;
    let _e11: Flector = self_2355;
    let _e14: Point = other_2035;
    let _e19: Flector = self_2355;
    let _e22: Point = other_2035;
    return Scalar((((_e4.g0_.x * _e7.g0_.x) + (_e11.g0_.y * _e14.g0_.y)) + (_e19.g0_.z * _e22.g0_.z)));
}

fn flector_point_dot(self_2356: Flector, other_2036: Point) -> Scalar {
    var self_2357: Flector;
    var other_2037: Point;

    self_2357 = self_2356;
    other_2037 = other_2036;
    let _e4: Flector = self_2357;
    let _e7: Point = other_2037;
    let _e11: Flector = self_2357;
    let _e14: Point = other_2037;
    let _e19: Flector = self_2357;
    let _e22: Point = other_2037;
    return Scalar((((_e4.g0_.x * _e7.g0_.x) + (_e11.g0_.y * _e14.g0_.y)) + (_e19.g0_.z * _e22.g0_.z)));
}

fn flector_point_anti_scalar_product(self_2358: Flector, other_2038: Point) -> AntiScalar {
    var self_2359: Flector;
    var other_2039: Point;

    self_2359 = self_2358;
    other_2039 = other_2038;
    let _e5: Flector = self_2359;
    let _e8: Point = other_2039;
    return AntiScalar((0.0 - (_e5.g0_.w * _e8.g0_.w)));
}

fn flector_point_anti_dot(self_2360: Flector, other_2040: Point) -> AntiScalar {
    var self_2361: Flector;
    var other_2041: Point;

    self_2361 = self_2360;
    other_2041 = other_2040;
    let _e5: Flector = self_2361;
    let _e8: Point = other_2041;
    return AntiScalar((0.0 - (_e5.g0_.w * _e8.g0_.w)));
}

fn flector_line_geometric_product(self_2362: Flector, other_2042: Line) -> Flector {
    var self_2363: Flector;
    var other_2043: Line;

    self_2363 = self_2362;
    other_2043 = other_2042;
    let _e4: Flector = self_2363;
    let _e8: Line = other_2043;
    let _e11: Line = other_2043;
    let _e14: Line = other_2043;
    let _e17: Line = other_2043;
    let _e30: Flector = self_2363;
    let _e34: Line = other_2043;
    let _e37: Line = other_2043;
    let _e40: Line = other_2043;
    let _e43: Line = other_2043;
    let _e57: Flector = self_2363;
    let _e61: Line = other_2043;
    let _e74: Flector = self_2363;
    let _e78: Line = other_2043;
    let _e91: Flector = self_2363;
    let _e95: Line = other_2043;
    let _e108: Flector = self_2363;
    let _e112: Line = other_2043;
    let _e115: Line = other_2043;
    let _e118: Line = other_2043;
    let _e121: Line = other_2043;
    let _e133: Flector = self_2363;
    let _e137: Line = other_2043;
    let _e140: Line = other_2043;
    let _e143: Line = other_2043;
    let _e146: Line = other_2043;
    let _e160: Flector = self_2363;
    let _e164: Line = other_2043;
    let _e167: Line = other_2043;
    let _e170: Line = other_2043;
    let _e173: Line = other_2043;
    let _e186: Flector = self_2363;
    let _e190: Line = other_2043;
    let _e193: Line = other_2043;
    let _e196: Line = other_2043;
    let _e199: Line = other_2043;
    let _e213: Flector = self_2363;
    let _e217: Line = other_2043;
    let _e220: Line = other_2043;
    let _e223: Line = other_2043;
    let _e226: Line = other_2043;
    let _e238: Flector = self_2363;
    let _e242: Line = other_2043;
    let _e245: Line = other_2043;
    let _e248: Line = other_2043;
    let _e251: Line = other_2043;
    let _e264: Flector = self_2363;
    let _e268: Line = other_2043;
    let _e271: Line = other_2043;
    let _e274: Line = other_2043;
    let _e277: Line = other_2043;
    let _e290: Flector = self_2363;
    let _e294: Line = other_2043;
    let _e297: Line = other_2043;
    let _e300: Line = other_2043;
    let _e303: Line = other_2043;
    let _e316: Flector = self_2363;
    let _e320: Line = other_2043;
    let _e323: Line = other_2043;
    let _e326: Line = other_2043;
    let _e329: Line = other_2043;
    let _e341: Flector = self_2363;
    let _e345: Line = other_2043;
    let _e348: Line = other_2043;
    let _e351: Line = other_2043;
    let _e354: Line = other_2043;
    return Flector(((((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g1_.z, _e11.g1_.z, _e14.g1_.x, _e17.g0_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e30.g0_.z) * vec4<f32>(_e34.g1_.y, _e37.g1_.x, _e40.g1_.y, _e43.g0_.z)) * vec4<f32>(1.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e57.g1_.x) * vec4<f32>(_e61.g1_.x)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e74.g1_.y) * vec4<f32>(_e78.g1_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e91.g1_.z) * vec4<f32>(_e95.g1_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e108.g1_.w) * vec4<f32>(_e112.g1_.x, _e115.g1_.y, _e118.g1_.z, _e121.g1_.x)) * vec4<f32>(1.0, 1.0, 1.0, 0.0))) + ((vec4<f32>(_e133.g0_.x) * vec4<f32>(_e137.g1_.x, _e140.g1_.z, _e143.g1_.y, _e146.g0_.x)) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))), (((((((((vec4<f32>(_e160.g0_.y) * vec4<f32>(_e164.g0_.z, _e167.g0_.z, _e170.g0_.x, _e173.g1_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e186.g0_.z) * vec4<f32>(_e190.g0_.y, _e193.g0_.x, _e196.g0_.y, _e199.g1_.z)) * vec4<f32>(1.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e213.g0_.w) * vec4<f32>(_e217.g1_.x, _e220.g1_.y, _e223.g1_.z, _e226.g1_.x)) * vec4<f32>(1.0, 1.0, 1.0, 0.0))) + ((vec4<f32>(_e238.g1_.x) * vec4<f32>(_e242.g1_.z, _e245.g1_.z, _e248.g1_.y, _e251.g1_.z)) * vec4<f32>(0.0, 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e264.g1_.y) * vec4<f32>(_e268.g1_.z, _e271.g1_.z, _e274.g1_.x, _e277.g1_.z)) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0))) + ((vec4<f32>(_e290.g1_.z) * vec4<f32>(_e294.g1_.y, _e297.g1_.x, _e300.g1_.y, _e303.g1_.y)) * vec4<f32>(1.0, -(1.0), 0.0, 0.0))) + ((vec4<f32>(_e316.g1_.w) * vec4<f32>(_e320.g0_.x, _e323.g0_.y, _e326.g0_.z, _e329.g0_.x)) * vec4<f32>(1.0, 1.0, 1.0, 0.0))) + ((vec4<f32>(_e341.g0_.x) * vec4<f32>(_e345.g0_.x, _e348.g0_.z, _e351.g0_.y, _e354.g1_.x)) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))));
}

fn flector_line_regressive_product(self_2364: Flector, other_2044: Line) -> Point {
    var self_2365: Flector;
    var other_2045: Line;

    self_2365 = self_2364;
    other_2045 = other_2044;
    let _e4: Flector = self_2365;
    let _e8: Line = other_2045;
    let _e11: Line = other_2045;
    let _e14: Line = other_2045;
    let _e17: Line = other_2045;
    let _e30: Flector = self_2365;
    let _e34: Line = other_2045;
    let _e37: Line = other_2045;
    let _e40: Line = other_2045;
    let _e43: Line = other_2045;
    let _e57: Flector = self_2365;
    let _e61: Line = other_2045;
    let _e64: Line = other_2045;
    let _e67: Line = other_2045;
    let _e70: Line = other_2045;
    let _e82: Flector = self_2365;
    let _e86: Line = other_2045;
    let _e89: Line = other_2045;
    let _e92: Line = other_2045;
    let _e95: Line = other_2045;
    return Point((((((vec4<f32>(_e4.g1_.y) * vec4<f32>(_e8.g1_.z, _e11.g1_.z, _e14.g1_.x, _e17.g0_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e30.g1_.z) * vec4<f32>(_e34.g1_.y, _e37.g1_.x, _e40.g1_.y, _e43.g0_.z)) * vec4<f32>(1.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e57.g1_.w) * vec4<f32>(_e61.g0_.x, _e64.g0_.y, _e67.g0_.z, _e70.g0_.x)) * vec4<f32>(1.0, 1.0, 1.0, 0.0))) + ((vec4<f32>(_e82.g1_.x) * vec4<f32>(_e86.g1_.x, _e89.g1_.z, _e92.g1_.y, _e95.g0_.x)) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))));
}

fn flector_line_anti_wedge(self_2366: Flector, other_2046: Line) -> Point {
    var self_2367: Flector;
    var other_2047: Line;

    self_2367 = self_2366;
    other_2047 = other_2046;
    let _e4: Flector = self_2367;
    let _e8: Line = other_2047;
    let _e11: Line = other_2047;
    let _e14: Line = other_2047;
    let _e17: Line = other_2047;
    let _e30: Flector = self_2367;
    let _e34: Line = other_2047;
    let _e37: Line = other_2047;
    let _e40: Line = other_2047;
    let _e43: Line = other_2047;
    let _e57: Flector = self_2367;
    let _e61: Line = other_2047;
    let _e64: Line = other_2047;
    let _e67: Line = other_2047;
    let _e70: Line = other_2047;
    let _e82: Flector = self_2367;
    let _e86: Line = other_2047;
    let _e89: Line = other_2047;
    let _e92: Line = other_2047;
    let _e95: Line = other_2047;
    return Point((((((vec4<f32>(_e4.g1_.y) * vec4<f32>(_e8.g1_.z, _e11.g1_.z, _e14.g1_.x, _e17.g0_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e30.g1_.z) * vec4<f32>(_e34.g1_.y, _e37.g1_.x, _e40.g1_.y, _e43.g0_.z)) * vec4<f32>(1.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e57.g1_.w) * vec4<f32>(_e61.g0_.x, _e64.g0_.y, _e67.g0_.z, _e70.g0_.x)) * vec4<f32>(1.0, 1.0, 1.0, 0.0))) + ((vec4<f32>(_e82.g1_.x) * vec4<f32>(_e86.g1_.x, _e89.g1_.z, _e92.g1_.y, _e95.g0_.x)) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))));
}

fn flector_line_meet(self_2368: Flector, other_2048: Line) -> Point {
    var self_2369: Flector;
    var other_2049: Line;

    self_2369 = self_2368;
    other_2049 = other_2048;
    let _e4: Flector = self_2369;
    let _e8: Line = other_2049;
    let _e11: Line = other_2049;
    let _e14: Line = other_2049;
    let _e17: Line = other_2049;
    let _e30: Flector = self_2369;
    let _e34: Line = other_2049;
    let _e37: Line = other_2049;
    let _e40: Line = other_2049;
    let _e43: Line = other_2049;
    let _e57: Flector = self_2369;
    let _e61: Line = other_2049;
    let _e64: Line = other_2049;
    let _e67: Line = other_2049;
    let _e70: Line = other_2049;
    let _e82: Flector = self_2369;
    let _e86: Line = other_2049;
    let _e89: Line = other_2049;
    let _e92: Line = other_2049;
    let _e95: Line = other_2049;
    return Point((((((vec4<f32>(_e4.g1_.y) * vec4<f32>(_e8.g1_.z, _e11.g1_.z, _e14.g1_.x, _e17.g0_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e30.g1_.z) * vec4<f32>(_e34.g1_.y, _e37.g1_.x, _e40.g1_.y, _e43.g0_.z)) * vec4<f32>(1.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e57.g1_.w) * vec4<f32>(_e61.g0_.x, _e64.g0_.y, _e67.g0_.z, _e70.g0_.x)) * vec4<f32>(1.0, 1.0, 1.0, 0.0))) + ((vec4<f32>(_e82.g1_.x) * vec4<f32>(_e86.g1_.x, _e89.g1_.z, _e92.g1_.y, _e95.g0_.x)) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))));
}

fn flector_line_outer_product(self_2370: Flector, other_2050: Line) -> Plane {
    var self_2371: Flector;
    var other_2051: Line;

    self_2371 = self_2370;
    other_2051 = other_2050;
    let _e4: Flector = self_2371;
    let _e8: Line = other_2051;
    let _e11: Line = other_2051;
    let _e14: Line = other_2051;
    let _e17: Line = other_2051;
    let _e30: Flector = self_2371;
    let _e34: Line = other_2051;
    let _e37: Line = other_2051;
    let _e40: Line = other_2051;
    let _e43: Line = other_2051;
    let _e57: Flector = self_2371;
    let _e61: Line = other_2051;
    let _e64: Line = other_2051;
    let _e67: Line = other_2051;
    let _e70: Line = other_2051;
    let _e82: Flector = self_2371;
    let _e86: Line = other_2051;
    let _e89: Line = other_2051;
    let _e92: Line = other_2051;
    let _e95: Line = other_2051;
    return Plane((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.z, _e11.g0_.z, _e14.g0_.x, _e17.g1_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e30.g0_.z) * vec4<f32>(_e34.g0_.y, _e37.g0_.x, _e40.g0_.y, _e43.g1_.z)) * vec4<f32>(1.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e57.g0_.w) * vec4<f32>(_e61.g1_.x, _e64.g1_.y, _e67.g1_.z, _e70.g1_.x)) * vec4<f32>(1.0, 1.0, 1.0, 0.0))) + ((vec4<f32>(_e82.g0_.x) * vec4<f32>(_e86.g0_.x, _e89.g0_.z, _e92.g0_.y, _e95.g1_.x)) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))));
}

fn flector_line_wedge(self_2372: Flector, other_2052: Line) -> Plane {
    var self_2373: Flector;
    var other_2053: Line;

    self_2373 = self_2372;
    other_2053 = other_2052;
    let _e4: Flector = self_2373;
    let _e8: Line = other_2053;
    let _e11: Line = other_2053;
    let _e14: Line = other_2053;
    let _e17: Line = other_2053;
    let _e30: Flector = self_2373;
    let _e34: Line = other_2053;
    let _e37: Line = other_2053;
    let _e40: Line = other_2053;
    let _e43: Line = other_2053;
    let _e57: Flector = self_2373;
    let _e61: Line = other_2053;
    let _e64: Line = other_2053;
    let _e67: Line = other_2053;
    let _e70: Line = other_2053;
    let _e82: Flector = self_2373;
    let _e86: Line = other_2053;
    let _e89: Line = other_2053;
    let _e92: Line = other_2053;
    let _e95: Line = other_2053;
    return Plane((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.z, _e11.g0_.z, _e14.g0_.x, _e17.g1_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e30.g0_.z) * vec4<f32>(_e34.g0_.y, _e37.g0_.x, _e40.g0_.y, _e43.g1_.z)) * vec4<f32>(1.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e57.g0_.w) * vec4<f32>(_e61.g1_.x, _e64.g1_.y, _e67.g1_.z, _e70.g1_.x)) * vec4<f32>(1.0, 1.0, 1.0, 0.0))) + ((vec4<f32>(_e82.g0_.x) * vec4<f32>(_e86.g0_.x, _e89.g0_.z, _e92.g0_.y, _e95.g1_.x)) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))));
}

fn flector_line_join(self_2374: Flector, other_2054: Line) -> Plane {
    var self_2375: Flector;
    var other_2055: Line;

    self_2375 = self_2374;
    other_2055 = other_2054;
    let _e4: Flector = self_2375;
    let _e8: Line = other_2055;
    let _e11: Line = other_2055;
    let _e14: Line = other_2055;
    let _e17: Line = other_2055;
    let _e30: Flector = self_2375;
    let _e34: Line = other_2055;
    let _e37: Line = other_2055;
    let _e40: Line = other_2055;
    let _e43: Line = other_2055;
    let _e57: Flector = self_2375;
    let _e61: Line = other_2055;
    let _e64: Line = other_2055;
    let _e67: Line = other_2055;
    let _e70: Line = other_2055;
    let _e82: Flector = self_2375;
    let _e86: Line = other_2055;
    let _e89: Line = other_2055;
    let _e92: Line = other_2055;
    let _e95: Line = other_2055;
    return Plane((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.z, _e11.g0_.z, _e14.g0_.x, _e17.g1_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e30.g0_.z) * vec4<f32>(_e34.g0_.y, _e37.g0_.x, _e40.g0_.y, _e43.g1_.z)) * vec4<f32>(1.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e57.g0_.w) * vec4<f32>(_e61.g1_.x, _e64.g1_.y, _e67.g1_.z, _e70.g1_.x)) * vec4<f32>(1.0, 1.0, 1.0, 0.0))) + ((vec4<f32>(_e82.g0_.x) * vec4<f32>(_e86.g0_.x, _e89.g0_.z, _e92.g0_.y, _e95.g1_.x)) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))));
}

fn flector_line_inner_product(self_2376: Flector, other_2056: Line) -> Point {
    var self_2377: Flector;
    var other_2057: Line;

    self_2377 = self_2376;
    other_2057 = other_2056;
    let _e4: Flector = self_2377;
    let _e8: Line = other_2057;
    let _e11: Line = other_2057;
    let _e14: Line = other_2057;
    let _e17: Line = other_2057;
    let _e30: Flector = self_2377;
    let _e34: Line = other_2057;
    let _e37: Line = other_2057;
    let _e40: Line = other_2057;
    let _e43: Line = other_2057;
    let _e57: Flector = self_2377;
    let _e61: Line = other_2057;
    let _e74: Flector = self_2377;
    let _e78: Line = other_2057;
    let _e91: Flector = self_2377;
    let _e95: Line = other_2057;
    let _e108: Flector = self_2377;
    let _e112: Line = other_2057;
    let _e115: Line = other_2057;
    let _e118: Line = other_2057;
    let _e121: Line = other_2057;
    let _e133: Flector = self_2377;
    let _e137: Line = other_2057;
    let _e140: Line = other_2057;
    let _e143: Line = other_2057;
    let _e146: Line = other_2057;
    return Point(((((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g1_.z, _e11.g1_.z, _e14.g1_.x, _e17.g0_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e30.g0_.z) * vec4<f32>(_e34.g1_.y, _e37.g1_.x, _e40.g1_.y, _e43.g0_.z)) * vec4<f32>(1.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e57.g1_.x) * vec4<f32>(_e61.g1_.x)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e74.g1_.y) * vec4<f32>(_e78.g1_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e91.g1_.z) * vec4<f32>(_e95.g1_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e108.g1_.w) * vec4<f32>(_e112.g1_.x, _e115.g1_.y, _e118.g1_.z, _e121.g1_.x)) * vec4<f32>(1.0, 1.0, 1.0, 0.0))) + ((vec4<f32>(_e133.g0_.x) * vec4<f32>(_e137.g1_.x, _e140.g1_.z, _e143.g1_.y, _e146.g0_.x)) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))));
}

fn flector_line_geometric_anti_product(self_2378: Flector, other_2058: Line) -> Flector {
    var self_2379: Flector;
    var other_2059: Line;

    self_2379 = self_2378;
    other_2059 = other_2058;
    let _e4: Flector = self_2379;
    let _e8: Line = other_2059;
    let _e11: Line = other_2059;
    let _e14: Line = other_2059;
    let _e17: Line = other_2059;
    let _e29: Flector = self_2379;
    let _e33: Line = other_2059;
    let _e36: Line = other_2059;
    let _e39: Line = other_2059;
    let _e42: Line = other_2059;
    let _e55: Flector = self_2379;
    let _e59: Line = other_2059;
    let _e62: Line = other_2059;
    let _e65: Line = other_2059;
    let _e68: Line = other_2059;
    let _e83: Flector = self_2379;
    let _e87: Line = other_2059;
    let _e90: Line = other_2059;
    let _e93: Line = other_2059;
    let _e96: Line = other_2059;
    let _e110: Flector = self_2379;
    let _e114: Line = other_2059;
    let _e117: Line = other_2059;
    let _e120: Line = other_2059;
    let _e123: Line = other_2059;
    let _e137: Flector = self_2379;
    let _e141: Line = other_2059;
    let _e144: Line = other_2059;
    let _e147: Line = other_2059;
    let _e150: Line = other_2059;
    let _e164: Flector = self_2379;
    let _e168: Line = other_2059;
    let _e171: Line = other_2059;
    let _e174: Line = other_2059;
    let _e177: Line = other_2059;
    let _e189: Flector = self_2379;
    let _e193: Line = other_2059;
    let _e196: Line = other_2059;
    let _e199: Line = other_2059;
    let _e202: Line = other_2059;
    let _e215: Flector = self_2379;
    let _e219: Line = other_2059;
    let _e231: Flector = self_2379;
    let _e235: Line = other_2059;
    let _e248: Flector = self_2379;
    let _e252: Line = other_2059;
    let _e255: Line = other_2059;
    let _e258: Line = other_2059;
    let _e261: Line = other_2059;
    let _e274: Flector = self_2379;
    let _e278: Line = other_2059;
    let _e281: Line = other_2059;
    let _e284: Line = other_2059;
    let _e287: Line = other_2059;
    let _e300: Flector = self_2379;
    let _e304: Line = other_2059;
    let _e307: Line = other_2059;
    let _e310: Line = other_2059;
    let _e313: Line = other_2059;
    let _e326: Flector = self_2379;
    let _e329: Line = other_2059;
    let _e332: Line = other_2059;
    let _e335: Line = other_2059;
    let _e338: Line = other_2059;
    return Flector((((((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.z, _e11.g0_.z, _e14.g0_.x, _e17.g0_.z)) * vec4<f32>(1.0, 0.0, -(1.0), 0.0)) + ((vec4<f32>(_e29.g0_.z) * vec4<f32>(_e33.g0_.y, _e36.g0_.x, _e39.g0_.y, _e42.g0_.y)) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))) + ((vec4<f32>(_e55.g0_.w) * vec4<f32>(_e59.g1_.x, _e62.g1_.y, _e65.g1_.z, _e68.g1_.x)) * vec4<f32>(-(1.0), -(1.0), -(1.0), 0.0))) + ((vec4<f32>(_e83.g1_.x) * vec4<f32>(_e87.g1_.z, _e90.g1_.z, _e93.g1_.y, _e96.g0_.x)) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e110.g1_.y) * vec4<f32>(_e114.g1_.z, _e117.g1_.z, _e120.g1_.x, _e123.g0_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e137.g1_.z) * vec4<f32>(_e141.g1_.y, _e144.g1_.x, _e147.g1_.y, _e150.g0_.z)) * vec4<f32>(1.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e164.g1_.w) * vec4<f32>(_e168.g0_.x, _e171.g0_.y, _e174.g0_.z, _e177.g0_.x)) * vec4<f32>(1.0, 1.0, 1.0, 0.0))) + ((vec4<f32>(_e189.g0_.x) * vec4<f32>(_e193.g0_.x, _e196.g0_.z, _e199.g0_.y, _e202.g0_.x)) * vec4<f32>(0.0, -(1.0), 1.0, 0.0))), (((((((vec4<f32>(_e215.g0_.y) * vec4<f32>(_e219.g0_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0))) + ((vec4<f32>(_e231.g0_.z) * vec4<f32>(_e235.g0_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e248.g1_.x) * vec4<f32>(_e252.g0_.z, _e255.g0_.z, _e258.g0_.y, _e261.g1_.x)) * vec4<f32>(0.0, -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e274.g1_.y) * vec4<f32>(_e278.g0_.z, _e281.g0_.z, _e284.g0_.x, _e287.g1_.y)) * vec4<f32>(1.0, 0.0, -(1.0), 1.0))) + ((vec4<f32>(_e300.g1_.z) * vec4<f32>(_e304.g0_.y, _e307.g0_.x, _e310.g0_.y, _e313.g1_.z)) * vec4<f32>(-(1.0), 1.0, 0.0, 1.0))) + ((_e326.g0_.wwwx * vec4<f32>(_e329.g0_.x, _e332.g0_.y, _e335.g0_.z, _e338.g0_.x)) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))));
}

fn flector_line_inner_anti_product(self_2380: Flector, other_2060: Line) -> Plane {
    var self_2381: Flector;
    var other_2061: Line;

    self_2381 = self_2380;
    other_2061 = other_2060;
    let _e4: Flector = self_2381;
    let _e8: Line = other_2061;
    let _e20: Flector = self_2381;
    let _e24: Line = other_2061;
    let _e37: Flector = self_2381;
    let _e41: Line = other_2061;
    let _e44: Line = other_2061;
    let _e47: Line = other_2061;
    let _e50: Line = other_2061;
    let _e63: Flector = self_2381;
    let _e67: Line = other_2061;
    let _e70: Line = other_2061;
    let _e73: Line = other_2061;
    let _e76: Line = other_2061;
    let _e89: Flector = self_2381;
    let _e93: Line = other_2061;
    let _e96: Line = other_2061;
    let _e99: Line = other_2061;
    let _e102: Line = other_2061;
    let _e115: Flector = self_2381;
    let _e118: Line = other_2061;
    let _e121: Line = other_2061;
    let _e124: Line = other_2061;
    let _e127: Line = other_2061;
    return Plane((((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0))) + ((vec4<f32>(_e20.g0_.z) * vec4<f32>(_e24.g0_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e37.g1_.x) * vec4<f32>(_e41.g0_.z, _e44.g0_.z, _e47.g0_.y, _e50.g1_.x)) * vec4<f32>(0.0, -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e63.g1_.y) * vec4<f32>(_e67.g0_.z, _e70.g0_.z, _e73.g0_.x, _e76.g1_.y)) * vec4<f32>(1.0, 0.0, -(1.0), 1.0))) + ((vec4<f32>(_e89.g1_.z) * vec4<f32>(_e93.g0_.y, _e96.g0_.x, _e99.g0_.y, _e102.g1_.z)) * vec4<f32>(-(1.0), 1.0, 0.0, 1.0))) + ((_e115.g0_.wwwx * vec4<f32>(_e118.g0_.x, _e121.g0_.y, _e124.g0_.z, _e127.g0_.x)) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))));
}

fn flector_line_left_contraction(self_2382: Flector, other_2062: Line) -> Point {
    var self_2383: Flector;
    var other_2063: Line;

    self_2383 = self_2382;
    other_2063 = other_2062;
    let _e4: Flector = self_2383;
    let _e8: Line = other_2063;
    let _e11: Line = other_2063;
    let _e14: Line = other_2063;
    let _e17: Line = other_2063;
    let _e30: Flector = self_2383;
    let _e34: Line = other_2063;
    let _e37: Line = other_2063;
    let _e40: Line = other_2063;
    let _e43: Line = other_2063;
    let _e57: Flector = self_2383;
    let _e61: Line = other_2063;
    let _e64: Line = other_2063;
    let _e67: Line = other_2063;
    let _e70: Line = other_2063;
    return Point(((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g1_.z, _e11.g1_.z, _e14.g1_.x, _e17.g0_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e30.g0_.z) * vec4<f32>(_e34.g1_.y, _e37.g1_.x, _e40.g1_.y, _e43.g0_.z)) * vec4<f32>(1.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e57.g0_.x) * vec4<f32>(_e61.g1_.x, _e64.g1_.z, _e67.g1_.y, _e70.g0_.x)) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))));
}

fn flector_line_right_contraction(self_2384: Flector, other_2064: Line) -> Point {
    var self_2385: Flector;
    var other_2065: Line;

    self_2385 = self_2384;
    other_2065 = other_2064;
    let _e4: Flector = self_2385;
    let _e8: Line = other_2065;
    let _e20: Flector = self_2385;
    let _e24: Line = other_2065;
    let _e37: Flector = self_2385;
    let _e40: Line = other_2065;
    let _e43: Line = other_2065;
    let _e46: Line = other_2065;
    let _e49: Line = other_2065;
    return Point(((((vec4<f32>(_e4.g1_.y) * vec4<f32>(_e8.g1_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0))) + ((vec4<f32>(_e20.g1_.z) * vec4<f32>(_e24.g1_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((_e37.g1_.wwwx * vec4<f32>(_e40.g1_.x, _e43.g1_.y, _e46.g1_.z, _e49.g1_.x)) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))));
}

fn flector_line_left_anti_contraction(self_2386: Flector, other_2066: Line) -> Plane {
    var self_2387: Flector;
    var other_2067: Line;

    self_2387 = self_2386;
    other_2067 = other_2066;
    let _e4: Flector = self_2387;
    let _e8: Line = other_2067;
    let _e11: Line = other_2067;
    let _e14: Line = other_2067;
    let _e17: Line = other_2067;
    let _e29: Flector = self_2387;
    let _e33: Line = other_2067;
    let _e36: Line = other_2067;
    let _e39: Line = other_2067;
    let _e42: Line = other_2067;
    let _e55: Flector = self_2387;
    let _e59: Line = other_2067;
    let _e62: Line = other_2067;
    let _e65: Line = other_2067;
    let _e68: Line = other_2067;
    return Plane(((((vec4<f32>(_e4.g1_.y) * vec4<f32>(_e8.g0_.z, _e11.g0_.z, _e14.g0_.x, _e17.g1_.y)) * vec4<f32>(1.0, 0.0, -(1.0), 1.0)) + ((vec4<f32>(_e29.g1_.z) * vec4<f32>(_e33.g0_.y, _e36.g0_.x, _e39.g0_.y, _e42.g1_.z)) * vec4<f32>(-(1.0), 1.0, 0.0, 1.0))) + ((vec4<f32>(_e55.g1_.x) * vec4<f32>(_e59.g0_.x, _e62.g0_.z, _e65.g0_.y, _e68.g1_.x)) * vec4<f32>(0.0, -(1.0), 1.0, 1.0))));
}

fn flector_line_right_anti_contraction(self_2388: Flector, other_2068: Line) -> Plane {
    var self_2389: Flector;
    var other_2069: Line;

    self_2389 = self_2388;
    other_2069 = other_2068;
    let _e4: Flector = self_2389;
    let _e8: Line = other_2069;
    let _e20: Flector = self_2389;
    let _e24: Line = other_2069;
    let _e37: Flector = self_2389;
    let _e40: Line = other_2069;
    let _e43: Line = other_2069;
    let _e46: Line = other_2069;
    let _e49: Line = other_2069;
    return Plane(((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0))) + ((vec4<f32>(_e20.g0_.z) * vec4<f32>(_e24.g0_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((_e37.g0_.wwwx * vec4<f32>(_e40.g0_.x, _e43.g0_.y, _e46.g0_.z, _e49.g0_.x)) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))));
}

fn flector_plane_into(self_2390: Flector) -> Plane {
    var self_2391: Flector;

    self_2391 = self_2390;
    let _e2: Flector = self_2391;
    return Plane(_e2.g1_);
}

fn flector_plane_add(self_2392: Flector, other_2070: Plane) -> Flector {
    var self_2393: Flector;
    var other_2071: Plane;

    self_2393 = self_2392;
    other_2071 = other_2070;
    let _e4: Flector = self_2393;
    let _e6: Flector = self_2393;
    let _e8: Plane = other_2071;
    return Flector(_e4.g0_, (_e6.g1_ + _e8.g0_));
}

fn flector_plane_sub(self_2394: Flector, other_2072: Plane) -> Flector {
    var self_2395: Flector;
    var other_2073: Plane;

    self_2395 = self_2394;
    other_2073 = other_2072;
    let _e4: Flector = self_2395;
    let _e6: Flector = self_2395;
    let _e8: Plane = other_2073;
    return Flector(_e4.g0_, (_e6.g1_ - _e8.g0_));
}

fn flector_plane_outer_product(self_2396: Flector, other_2074: Plane) -> AntiScalar {
    var self_2397: Flector;
    var other_2075: Plane;

    self_2397 = self_2396;
    other_2075 = other_2074;
    let _e4: Flector = self_2397;
    let _e7: Plane = other_2075;
    let _e11: Flector = self_2397;
    let _e14: Plane = other_2075;
    let _e19: Flector = self_2397;
    let _e22: Plane = other_2075;
    let _e27: Flector = self_2397;
    let _e30: Plane = other_2075;
    return AntiScalar(((((_e4.g0_.x * _e7.g0_.x) + (_e11.g0_.y * _e14.g0_.y)) + (_e19.g0_.z * _e22.g0_.z)) + (_e27.g0_.w * _e30.g0_.w)));
}

fn flector_plane_wedge(self_2398: Flector, other_2076: Plane) -> AntiScalar {
    var self_2399: Flector;
    var other_2077: Plane;

    self_2399 = self_2398;
    other_2077 = other_2076;
    let _e4: Flector = self_2399;
    let _e7: Plane = other_2077;
    let _e11: Flector = self_2399;
    let _e14: Plane = other_2077;
    let _e19: Flector = self_2399;
    let _e22: Plane = other_2077;
    let _e27: Flector = self_2399;
    let _e30: Plane = other_2077;
    return AntiScalar(((((_e4.g0_.x * _e7.g0_.x) + (_e11.g0_.y * _e14.g0_.y)) + (_e19.g0_.z * _e22.g0_.z)) + (_e27.g0_.w * _e30.g0_.w)));
}

fn flector_plane_join(self_2400: Flector, other_2078: Plane) -> AntiScalar {
    var self_2401: Flector;
    var other_2079: Plane;

    self_2401 = self_2400;
    other_2079 = other_2078;
    let _e4: Flector = self_2401;
    let _e7: Plane = other_2079;
    let _e11: Flector = self_2401;
    let _e14: Plane = other_2079;
    let _e19: Flector = self_2401;
    let _e22: Plane = other_2079;
    let _e27: Flector = self_2401;
    let _e30: Plane = other_2079;
    return AntiScalar(((((_e4.g0_.x * _e7.g0_.x) + (_e11.g0_.y * _e14.g0_.y)) + (_e19.g0_.z * _e22.g0_.z)) + (_e27.g0_.w * _e30.g0_.w)));
}

fn flector_plane_inner_anti_product(self_2402: Flector, other_2080: Plane) -> Motor {
    var self_2403: Flector;
    var other_2081: Plane;

    self_2403 = self_2402;
    other_2081 = other_2080;
    let _e4: Flector = self_2403;
    let _e8: Plane = other_2081;
    let _e19: Flector = self_2403;
    let _e23: Plane = other_2081;
    let _e35: Flector = self_2403;
    let _e38: Flector = self_2403;
    let _e41: Flector = self_2403;
    let _e44: Flector = self_2403;
    let _e48: Plane = other_2081;
    let _e62: Flector = self_2403;
    let _e66: Plane = other_2081;
    let _e69: Plane = other_2081;
    let _e72: Plane = other_2081;
    let _e83: Flector = self_2403;
    let _e87: Plane = other_2081;
    let _e90: Plane = other_2081;
    let _e93: Plane = other_2081;
    let _e105: Flector = self_2403;
    let _e109: Plane = other_2081;
    let _e112: Plane = other_2081;
    let _e115: Plane = other_2081;
    return Motor(((((vec4<f32>(_e4.g1_.y) * vec4<f32>(_e8.g0_.y)) * vec4<f32>(0.0, 0.0, 0.0, 1.0)) + ((vec4<f32>(_e19.g1_.z) * vec4<f32>(_e23.g0_.z)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e35.g0_.w, _e38.g0_.w, _e41.g0_.w, _e44.g1_.x) * _e48.g0_.xyzx) * vec4<f32>(-(1.0), -(1.0), -(1.0), 1.0))), ((((vec3<f32>(_e62.g0_.y) * vec3<f32>(_e66.g0_.z, _e69.g0_.z, _e72.g0_.x)) * vec3<f32>(-(1.0), 0.0, 1.0)) + ((vec3<f32>(_e83.g0_.z) * vec3<f32>(_e87.g0_.y, _e90.g0_.x, _e93.g0_.y)) * vec3<f32>(1.0, -(1.0), 0.0))) + ((vec3<f32>(_e105.g0_.x) * vec3<f32>(_e109.g0_.x, _e112.g0_.z, _e115.g0_.y)) * vec3<f32>(0.0, 1.0, -(1.0)))));
}

fn flector_plane_right_contraction(self_2404: Flector, other_2082: Plane) -> Scalar {
    var self_2405: Flector;
    var other_2083: Plane;

    self_2405 = self_2404;
    other_2083 = other_2082;
    let _e5: Flector = self_2405;
    let _e8: Plane = other_2083;
    return Scalar((0.0 - (_e5.g1_.w * _e8.g0_.w)));
}

fn flector_plane_left_anti_contraction(self_2406: Flector, other_2084: Plane) -> AntiScalar {
    var self_2407: Flector;
    var other_2085: Plane;

    self_2407 = self_2406;
    other_2085 = other_2084;
    let _e4: Flector = self_2407;
    let _e7: Plane = other_2085;
    let _e11: Flector = self_2407;
    let _e14: Plane = other_2085;
    let _e19: Flector = self_2407;
    let _e22: Plane = other_2085;
    return AntiScalar((((_e4.g1_.x * _e7.g0_.x) + (_e11.g1_.y * _e14.g0_.y)) + (_e19.g1_.z * _e22.g0_.z)));
}

fn flector_plane_right_anti_contraction(self_2408: Flector, other_2086: Plane) -> Motor {
    var self_2409: Flector;
    var other_2087: Plane;

    self_2409 = self_2408;
    other_2087 = other_2086;
    let _e4: Flector = self_2409;
    let _e8: Plane = other_2087;
    let _e19: Flector = self_2409;
    let _e23: Plane = other_2087;
    let _e35: Flector = self_2409;
    let _e38: Flector = self_2409;
    let _e41: Flector = self_2409;
    let _e44: Flector = self_2409;
    let _e48: Plane = other_2087;
    let _e62: Flector = self_2409;
    let _e66: Plane = other_2087;
    let _e69: Plane = other_2087;
    let _e72: Plane = other_2087;
    let _e83: Flector = self_2409;
    let _e87: Plane = other_2087;
    let _e90: Plane = other_2087;
    let _e93: Plane = other_2087;
    let _e105: Flector = self_2409;
    let _e109: Plane = other_2087;
    let _e112: Plane = other_2087;
    let _e115: Plane = other_2087;
    return Motor(((((vec4<f32>(_e4.g1_.y) * vec4<f32>(_e8.g0_.y)) * vec4<f32>(0.0, 0.0, 0.0, 1.0)) + ((vec4<f32>(_e19.g1_.z) * vec4<f32>(_e23.g0_.z)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e35.g0_.w, _e38.g0_.w, _e41.g0_.w, _e44.g1_.x) * _e48.g0_.xyzx) * vec4<f32>(-(1.0), -(1.0), -(1.0), 1.0))), ((((vec3<f32>(_e62.g0_.y) * vec3<f32>(_e66.g0_.z, _e69.g0_.z, _e72.g0_.x)) * vec3<f32>(-(1.0), 0.0, 1.0)) + ((vec3<f32>(_e83.g0_.z) * vec3<f32>(_e87.g0_.y, _e90.g0_.x, _e93.g0_.y)) * vec3<f32>(1.0, -(1.0), 0.0))) + ((vec3<f32>(_e105.g0_.x) * vec3<f32>(_e109.g0_.x, _e112.g0_.z, _e115.g0_.y)) * vec3<f32>(0.0, 1.0, -(1.0)))));
}

fn flector_plane_scalar_product(self_2410: Flector, other_2088: Plane) -> Scalar {
    var self_2411: Flector;
    var other_2089: Plane;

    self_2411 = self_2410;
    other_2089 = other_2088;
    let _e5: Flector = self_2411;
    let _e8: Plane = other_2089;
    return Scalar((0.0 - (_e5.g1_.w * _e8.g0_.w)));
}

fn flector_plane_dot(self_2412: Flector, other_2090: Plane) -> Scalar {
    var self_2413: Flector;
    var other_2091: Plane;

    self_2413 = self_2412;
    other_2091 = other_2090;
    let _e5: Flector = self_2413;
    let _e8: Plane = other_2091;
    return Scalar((0.0 - (_e5.g1_.w * _e8.g0_.w)));
}

fn flector_plane_anti_scalar_product(self_2414: Flector, other_2092: Plane) -> AntiScalar {
    var self_2415: Flector;
    var other_2093: Plane;

    self_2415 = self_2414;
    other_2093 = other_2092;
    let _e4: Flector = self_2415;
    let _e7: Plane = other_2093;
    let _e11: Flector = self_2415;
    let _e14: Plane = other_2093;
    let _e19: Flector = self_2415;
    let _e22: Plane = other_2093;
    return AntiScalar((((_e4.g1_.x * _e7.g0_.x) + (_e11.g1_.y * _e14.g0_.y)) + (_e19.g1_.z * _e22.g0_.z)));
}

fn flector_plane_anti_dot(self_2416: Flector, other_2094: Plane) -> AntiScalar {
    var self_2417: Flector;
    var other_2095: Plane;

    self_2417 = self_2416;
    other_2095 = other_2094;
    let _e4: Flector = self_2417;
    let _e7: Plane = other_2095;
    let _e11: Flector = self_2417;
    let _e14: Plane = other_2095;
    let _e19: Flector = self_2417;
    let _e22: Plane = other_2095;
    return AntiScalar((((_e4.g1_.x * _e7.g0_.x) + (_e11.g1_.y * _e14.g0_.y)) + (_e19.g1_.z * _e22.g0_.z)));
}

fn flector_motor_geometric_product(self_2418: Flector, other_2096: Motor) -> Flector {
    var self_2419: Flector;
    var other_2097: Motor;

    self_2419 = self_2418;
    other_2097 = other_2096;
    let _e4: Flector = self_2419;
    let _e8: Motor = other_2097;
    let _e11: Motor = other_2097;
    let _e14: Motor = other_2097;
    let _e17: Motor = other_2097;
    let _e30: Flector = self_2419;
    let _e34: Motor = other_2097;
    let _e37: Motor = other_2097;
    let _e40: Motor = other_2097;
    let _e43: Motor = other_2097;
    let _e57: Flector = self_2419;
    let _e61: Motor = other_2097;
    let _e74: Flector = self_2419;
    let _e78: Motor = other_2097;
    let _e91: Flector = self_2419;
    let _e95: Motor = other_2097;
    let _e108: Flector = self_2419;
    let _e112: Motor = other_2097;
    let _e115: Motor = other_2097;
    let _e118: Motor = other_2097;
    let _e121: Motor = other_2097;
    let _e127: Flector = self_2419;
    let _e131: Motor = other_2097;
    let _e134: Motor = other_2097;
    let _e137: Motor = other_2097;
    let _e140: Motor = other_2097;
    let _e154: Flector = self_2419;
    let _e158: Motor = other_2097;
    let _e161: Motor = other_2097;
    let _e164: Motor = other_2097;
    let _e167: Motor = other_2097;
    let _e180: Flector = self_2419;
    let _e184: Motor = other_2097;
    let _e187: Motor = other_2097;
    let _e190: Motor = other_2097;
    let _e193: Motor = other_2097;
    let _e207: Flector = self_2419;
    let _e211: Motor = other_2097;
    let _e214: Motor = other_2097;
    let _e217: Motor = other_2097;
    let _e220: Motor = other_2097;
    let _e234: Flector = self_2419;
    let _e238: Motor = other_2097;
    let _e241: Motor = other_2097;
    let _e244: Motor = other_2097;
    let _e247: Motor = other_2097;
    let _e260: Flector = self_2419;
    let _e264: Motor = other_2097;
    let _e267: Motor = other_2097;
    let _e270: Motor = other_2097;
    let _e273: Motor = other_2097;
    let _e286: Flector = self_2419;
    let _e290: Motor = other_2097;
    let _e293: Motor = other_2097;
    let _e296: Motor = other_2097;
    let _e299: Motor = other_2097;
    let _e312: Flector = self_2419;
    let _e316: Motor = other_2097;
    let _e327: Flector = self_2419;
    let _e330: Motor = other_2097;
    let _e333: Motor = other_2097;
    let _e336: Motor = other_2097;
    let _e339: Motor = other_2097;
    return Flector(((((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g1_.z, _e11.g1_.z, _e14.g1_.x, _e17.g0_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e30.g0_.z) * vec4<f32>(_e34.g1_.y, _e37.g1_.x, _e40.g1_.y, _e43.g0_.z)) * vec4<f32>(1.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e57.g1_.x) * vec4<f32>(_e61.g1_.x)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e74.g1_.y) * vec4<f32>(_e78.g1_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e91.g1_.z) * vec4<f32>(_e95.g1_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + (vec4<f32>(_e108.g1_.w) * vec4<f32>(_e112.g1_.x, _e115.g1_.y, _e118.g1_.z, _e121.g0_.w))) + ((vec4<f32>(_e127.g0_.x) * vec4<f32>(_e131.g1_.x, _e134.g1_.z, _e137.g1_.y, _e140.g0_.x)) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))), (((((((((vec4<f32>(_e154.g0_.x) * vec4<f32>(_e158.g0_.w, _e161.g0_.z, _e164.g0_.y, _e167.g1_.x)) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0))) + ((vec4<f32>(_e180.g0_.y) * vec4<f32>(_e184.g0_.z, _e187.g0_.w, _e190.g0_.x, _e193.g1_.y)) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e207.g0_.z) * vec4<f32>(_e211.g0_.y, _e214.g0_.x, _e217.g0_.w, _e220.g1_.z)) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e234.g1_.x) * vec4<f32>(_e238.g1_.z, _e241.g1_.z, _e244.g1_.y, _e247.g1_.z)) * vec4<f32>(0.0, 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e260.g1_.y) * vec4<f32>(_e264.g1_.z, _e267.g1_.z, _e270.g1_.x, _e273.g1_.z)) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0))) + ((vec4<f32>(_e286.g1_.z) * vec4<f32>(_e290.g1_.y, _e293.g1_.x, _e296.g1_.y, _e299.g1_.y)) * vec4<f32>(1.0, -(1.0), 0.0, 0.0))) + ((vec4<f32>(_e312.g1_.w) * _e316.g0_.xyzx) * vec4<f32>(1.0, 1.0, 1.0, 0.0))) + ((_e327.g0_.wwwx * vec4<f32>(_e330.g1_.x, _e333.g1_.y, _e336.g1_.z, _e339.g1_.x)) * vec4<f32>(1.0, 1.0, 1.0, 0.0))));
}

fn flector_motor_regressive_product(self_2420: Flector, other_2098: Motor) -> Flector {
    var self_2421: Flector;
    var other_2099: Motor;

    self_2421 = self_2420;
    other_2099 = other_2098;
    let _e4: Flector = self_2421;
    let _e8: Motor = other_2099;
    let _e11: Motor = other_2099;
    let _e14: Motor = other_2099;
    let _e17: Motor = other_2099;
    let _e30: Flector = self_2421;
    let _e34: Motor = other_2099;
    let _e37: Motor = other_2099;
    let _e40: Motor = other_2099;
    let _e43: Motor = other_2099;
    let _e57: Flector = self_2421;
    let _e61: Motor = other_2099;
    let _e64: Motor = other_2099;
    let _e67: Motor = other_2099;
    let _e70: Motor = other_2099;
    let _e84: Flector = self_2421;
    let _e88: Motor = other_2099;
    let _e99: Flector = self_2421;
    let _e101: Motor = other_2099;
    let _e107: Flector = self_2421;
    let _e109: Motor = other_2099;
    return Flector(((((((vec4<f32>(_e4.g1_.x) * vec4<f32>(_e8.g1_.z, _e11.g1_.z, _e14.g1_.y, _e17.g0_.x)) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0))) + ((vec4<f32>(_e30.g1_.y) * vec4<f32>(_e34.g1_.z, _e37.g1_.z, _e40.g1_.x, _e43.g0_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e57.g1_.z) * vec4<f32>(_e61.g1_.y, _e64.g1_.x, _e67.g1_.y, _e70.g0_.z)) * vec4<f32>(1.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e84.g1_.w) * _e88.g0_.xyzx) * vec4<f32>(1.0, 1.0, 1.0, 0.0))) + (_e99.g0_ * vec4<f32>(_e101.g0_.w))), (_e107.g1_ * vec4<f32>(_e109.g0_.w)));
}

fn flector_motor_anti_wedge(self_2422: Flector, other_2100: Motor) -> Flector {
    var self_2423: Flector;
    var other_2101: Motor;

    self_2423 = self_2422;
    other_2101 = other_2100;
    let _e4: Flector = self_2423;
    let _e8: Motor = other_2101;
    let _e11: Motor = other_2101;
    let _e14: Motor = other_2101;
    let _e17: Motor = other_2101;
    let _e30: Flector = self_2423;
    let _e34: Motor = other_2101;
    let _e37: Motor = other_2101;
    let _e40: Motor = other_2101;
    let _e43: Motor = other_2101;
    let _e57: Flector = self_2423;
    let _e61: Motor = other_2101;
    let _e64: Motor = other_2101;
    let _e67: Motor = other_2101;
    let _e70: Motor = other_2101;
    let _e84: Flector = self_2423;
    let _e88: Motor = other_2101;
    let _e99: Flector = self_2423;
    let _e101: Motor = other_2101;
    let _e107: Flector = self_2423;
    let _e109: Motor = other_2101;
    return Flector(((((((vec4<f32>(_e4.g1_.x) * vec4<f32>(_e8.g1_.z, _e11.g1_.z, _e14.g1_.y, _e17.g0_.x)) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0))) + ((vec4<f32>(_e30.g1_.y) * vec4<f32>(_e34.g1_.z, _e37.g1_.z, _e40.g1_.x, _e43.g0_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e57.g1_.z) * vec4<f32>(_e61.g1_.y, _e64.g1_.x, _e67.g1_.y, _e70.g0_.z)) * vec4<f32>(1.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e84.g1_.w) * _e88.g0_.xyzx) * vec4<f32>(1.0, 1.0, 1.0, 0.0))) + (_e99.g0_ * vec4<f32>(_e101.g0_.w))), (_e107.g1_ * vec4<f32>(_e109.g0_.w)));
}

fn flector_motor_meet(self_2424: Flector, other_2102: Motor) -> Flector {
    var self_2425: Flector;
    var other_2103: Motor;

    self_2425 = self_2424;
    other_2103 = other_2102;
    let _e4: Flector = self_2425;
    let _e8: Motor = other_2103;
    let _e11: Motor = other_2103;
    let _e14: Motor = other_2103;
    let _e17: Motor = other_2103;
    let _e30: Flector = self_2425;
    let _e34: Motor = other_2103;
    let _e37: Motor = other_2103;
    let _e40: Motor = other_2103;
    let _e43: Motor = other_2103;
    let _e57: Flector = self_2425;
    let _e61: Motor = other_2103;
    let _e64: Motor = other_2103;
    let _e67: Motor = other_2103;
    let _e70: Motor = other_2103;
    let _e84: Flector = self_2425;
    let _e88: Motor = other_2103;
    let _e99: Flector = self_2425;
    let _e101: Motor = other_2103;
    let _e107: Flector = self_2425;
    let _e109: Motor = other_2103;
    return Flector(((((((vec4<f32>(_e4.g1_.x) * vec4<f32>(_e8.g1_.z, _e11.g1_.z, _e14.g1_.y, _e17.g0_.x)) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0))) + ((vec4<f32>(_e30.g1_.y) * vec4<f32>(_e34.g1_.z, _e37.g1_.z, _e40.g1_.x, _e43.g0_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e57.g1_.z) * vec4<f32>(_e61.g1_.y, _e64.g1_.x, _e67.g1_.y, _e70.g0_.z)) * vec4<f32>(1.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e84.g1_.w) * _e88.g0_.xyzx) * vec4<f32>(1.0, 1.0, 1.0, 0.0))) + (_e99.g0_ * vec4<f32>(_e101.g0_.w))), (_e107.g1_ * vec4<f32>(_e109.g0_.w)));
}

fn flector_motor_outer_product(self_2426: Flector, other_2104: Motor) -> Plane {
    var self_2427: Flector;
    var other_2105: Motor;

    self_2427 = self_2426;
    other_2105 = other_2104;
    let _e4: Flector = self_2427;
    let _e8: Motor = other_2105;
    let _e11: Motor = other_2105;
    let _e14: Motor = other_2105;
    let _e17: Motor = other_2105;
    let _e30: Flector = self_2427;
    let _e34: Motor = other_2105;
    let _e37: Motor = other_2105;
    let _e40: Motor = other_2105;
    let _e43: Motor = other_2105;
    let _e57: Flector = self_2427;
    let _e61: Motor = other_2105;
    let _e64: Motor = other_2105;
    let _e67: Motor = other_2105;
    let _e70: Motor = other_2105;
    let _e82: Flector = self_2427;
    let _e86: Motor = other_2105;
    let _e89: Motor = other_2105;
    let _e92: Motor = other_2105;
    let _e95: Motor = other_2105;
    return Plane((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.z, _e11.g0_.z, _e14.g0_.x, _e17.g1_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e30.g0_.z) * vec4<f32>(_e34.g0_.y, _e37.g0_.x, _e40.g0_.y, _e43.g1_.z)) * vec4<f32>(1.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e57.g0_.w) * vec4<f32>(_e61.g1_.x, _e64.g1_.y, _e67.g1_.z, _e70.g1_.x)) * vec4<f32>(1.0, 1.0, 1.0, 0.0))) + ((vec4<f32>(_e82.g0_.x) * vec4<f32>(_e86.g0_.x, _e89.g0_.z, _e92.g0_.y, _e95.g1_.x)) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))));
}

fn flector_motor_wedge(self_2428: Flector, other_2106: Motor) -> Plane {
    var self_2429: Flector;
    var other_2107: Motor;

    self_2429 = self_2428;
    other_2107 = other_2106;
    let _e4: Flector = self_2429;
    let _e8: Motor = other_2107;
    let _e11: Motor = other_2107;
    let _e14: Motor = other_2107;
    let _e17: Motor = other_2107;
    let _e30: Flector = self_2429;
    let _e34: Motor = other_2107;
    let _e37: Motor = other_2107;
    let _e40: Motor = other_2107;
    let _e43: Motor = other_2107;
    let _e57: Flector = self_2429;
    let _e61: Motor = other_2107;
    let _e64: Motor = other_2107;
    let _e67: Motor = other_2107;
    let _e70: Motor = other_2107;
    let _e82: Flector = self_2429;
    let _e86: Motor = other_2107;
    let _e89: Motor = other_2107;
    let _e92: Motor = other_2107;
    let _e95: Motor = other_2107;
    return Plane((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.z, _e11.g0_.z, _e14.g0_.x, _e17.g1_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e30.g0_.z) * vec4<f32>(_e34.g0_.y, _e37.g0_.x, _e40.g0_.y, _e43.g1_.z)) * vec4<f32>(1.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e57.g0_.w) * vec4<f32>(_e61.g1_.x, _e64.g1_.y, _e67.g1_.z, _e70.g1_.x)) * vec4<f32>(1.0, 1.0, 1.0, 0.0))) + ((vec4<f32>(_e82.g0_.x) * vec4<f32>(_e86.g0_.x, _e89.g0_.z, _e92.g0_.y, _e95.g1_.x)) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))));
}

fn flector_motor_join(self_2430: Flector, other_2108: Motor) -> Plane {
    var self_2431: Flector;
    var other_2109: Motor;

    self_2431 = self_2430;
    other_2109 = other_2108;
    let _e4: Flector = self_2431;
    let _e8: Motor = other_2109;
    let _e11: Motor = other_2109;
    let _e14: Motor = other_2109;
    let _e17: Motor = other_2109;
    let _e30: Flector = self_2431;
    let _e34: Motor = other_2109;
    let _e37: Motor = other_2109;
    let _e40: Motor = other_2109;
    let _e43: Motor = other_2109;
    let _e57: Flector = self_2431;
    let _e61: Motor = other_2109;
    let _e64: Motor = other_2109;
    let _e67: Motor = other_2109;
    let _e70: Motor = other_2109;
    let _e82: Flector = self_2431;
    let _e86: Motor = other_2109;
    let _e89: Motor = other_2109;
    let _e92: Motor = other_2109;
    let _e95: Motor = other_2109;
    return Plane((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.z, _e11.g0_.z, _e14.g0_.x, _e17.g1_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e30.g0_.z) * vec4<f32>(_e34.g0_.y, _e37.g0_.x, _e40.g0_.y, _e43.g1_.z)) * vec4<f32>(1.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e57.g0_.w) * vec4<f32>(_e61.g1_.x, _e64.g1_.y, _e67.g1_.z, _e70.g1_.x)) * vec4<f32>(1.0, 1.0, 1.0, 0.0))) + ((vec4<f32>(_e82.g0_.x) * vec4<f32>(_e86.g0_.x, _e89.g0_.z, _e92.g0_.y, _e95.g1_.x)) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))));
}

fn flector_motor_geometric_anti_product(self_2432: Flector, other_2110: Motor) -> Flector {
    var self_2433: Flector;
    var other_2111: Motor;

    self_2433 = self_2432;
    other_2111 = other_2110;
    let _e4: Flector = self_2433;
    let _e8: Motor = other_2111;
    let _e19: Flector = self_2433;
    let _e23: Motor = other_2111;
    let _e35: Flector = self_2433;
    let _e39: Motor = other_2111;
    let _e42: Motor = other_2111;
    let _e45: Motor = other_2111;
    let _e48: Motor = other_2111;
    let _e63: Flector = self_2433;
    let _e67: Motor = other_2111;
    let _e70: Motor = other_2111;
    let _e73: Motor = other_2111;
    let _e76: Motor = other_2111;
    let _e90: Flector = self_2433;
    let _e94: Motor = other_2111;
    let _e97: Motor = other_2111;
    let _e100: Motor = other_2111;
    let _e103: Motor = other_2111;
    let _e117: Flector = self_2433;
    let _e121: Motor = other_2111;
    let _e124: Motor = other_2111;
    let _e127: Motor = other_2111;
    let _e130: Motor = other_2111;
    let _e144: Flector = self_2433;
    let _e148: Motor = other_2111;
    let _e159: Flector = self_2433;
    let _e163: Motor = other_2111;
    let _e175: Flector = self_2433;
    let _e179: Motor = other_2111;
    let _e191: Flector = self_2433;
    let _e195: Motor = other_2111;
    let _e208: Flector = self_2433;
    let _e212: Motor = other_2111;
    let _e215: Motor = other_2111;
    let _e218: Motor = other_2111;
    let _e221: Motor = other_2111;
    let _e234: Flector = self_2433;
    let _e238: Motor = other_2111;
    let _e241: Motor = other_2111;
    let _e244: Motor = other_2111;
    let _e247: Motor = other_2111;
    let _e260: Flector = self_2433;
    let _e264: Motor = other_2111;
    let _e267: Motor = other_2111;
    let _e270: Motor = other_2111;
    let _e273: Motor = other_2111;
    let _e286: Flector = self_2433;
    let _e290: Motor = other_2111;
    let _e302: Flector = self_2433;
    let _e305: Motor = other_2111;
    return Flector((((((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.zwxz) * vec4<f32>(1.0, 1.0, -(1.0), 0.0)) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.yxwy) * vec4<f32>(-(1.0), 1.0, 1.0, 0.0))) + ((vec4<f32>(_e35.g0_.w) * vec4<f32>(_e39.g1_.x, _e42.g1_.y, _e45.g1_.z, _e48.g0_.w)) * vec4<f32>(-(1.0), -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e63.g1_.x) * vec4<f32>(_e67.g1_.z, _e70.g1_.z, _e73.g1_.y, _e76.g0_.x)) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e90.g1_.y) * vec4<f32>(_e94.g1_.z, _e97.g1_.z, _e100.g1_.x, _e103.g0_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e117.g1_.z) * vec4<f32>(_e121.g1_.y, _e124.g1_.x, _e127.g1_.y, _e130.g0_.z)) * vec4<f32>(1.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e144.g1_.w) * _e148.g0_.xyzx) * vec4<f32>(1.0, 1.0, 1.0, 0.0))) + ((vec4<f32>(_e159.g0_.x) * _e163.g0_.wzyx) * vec4<f32>(1.0, -(1.0), 1.0, 0.0))), ((((((((vec4<f32>(_e175.g0_.y) * vec4<f32>(_e179.g0_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0))) + ((vec4<f32>(_e191.g0_.z) * vec4<f32>(_e195.g0_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e208.g1_.x) * vec4<f32>(_e212.g0_.w, _e215.g0_.z, _e218.g0_.y, _e221.g1_.x)) * vec4<f32>(1.0, -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e234.g1_.y) * vec4<f32>(_e238.g0_.z, _e241.g0_.w, _e244.g0_.x, _e247.g1_.y)) * vec4<f32>(1.0, 1.0, -(1.0), 1.0))) + ((vec4<f32>(_e260.g1_.z) * vec4<f32>(_e264.g0_.y, _e267.g0_.x, _e270.g0_.w, _e273.g1_.z)) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0))) + ((vec4<f32>(_e286.g1_.w) * vec4<f32>(_e290.g0_.w)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((_e302.g0_.wwwx * _e305.g0_.xyzx) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))));
}

fn flector_motor_inner_anti_product(self_2434: Flector, other_2112: Motor) -> Flector {
    var self_2435: Flector;
    var other_2113: Motor;

    self_2435 = self_2434;
    other_2113 = other_2112;
    let _e4: Flector = self_2435;
    let _e6: Motor = other_2113;
    let _e11: Flector = self_2435;
    let _e15: Motor = other_2113;
    let _e27: Flector = self_2435;
    let _e31: Motor = other_2113;
    let _e44: Flector = self_2435;
    let _e48: Motor = other_2113;
    let _e51: Motor = other_2113;
    let _e54: Motor = other_2113;
    let _e57: Motor = other_2113;
    let _e70: Flector = self_2435;
    let _e74: Motor = other_2113;
    let _e77: Motor = other_2113;
    let _e80: Motor = other_2113;
    let _e83: Motor = other_2113;
    let _e96: Flector = self_2435;
    let _e100: Motor = other_2113;
    let _e103: Motor = other_2113;
    let _e106: Motor = other_2113;
    let _e109: Motor = other_2113;
    let _e122: Flector = self_2435;
    let _e126: Motor = other_2113;
    let _e138: Flector = self_2435;
    let _e141: Motor = other_2113;
    return Flector((_e4.g0_ * vec4<f32>(_e6.g0_.w)), ((((((((vec4<f32>(_e11.g0_.y) * vec4<f32>(_e15.g0_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0))) + ((vec4<f32>(_e27.g0_.z) * vec4<f32>(_e31.g0_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e44.g1_.x) * vec4<f32>(_e48.g0_.w, _e51.g0_.z, _e54.g0_.y, _e57.g1_.x)) * vec4<f32>(1.0, -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e70.g1_.y) * vec4<f32>(_e74.g0_.z, _e77.g0_.w, _e80.g0_.x, _e83.g1_.y)) * vec4<f32>(1.0, 1.0, -(1.0), 1.0))) + ((vec4<f32>(_e96.g1_.z) * vec4<f32>(_e100.g0_.y, _e103.g0_.x, _e106.g0_.w, _e109.g1_.z)) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0))) + ((vec4<f32>(_e122.g1_.w) * vec4<f32>(_e126.g0_.w)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((_e138.g0_.wwwx * _e141.g0_.xyzx) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))));
}

fn flector_motor_right_contraction(self_2436: Flector, other_2114: Motor) -> Point {
    var self_2437: Flector;
    var other_2115: Motor;

    self_2437 = self_2436;
    other_2115 = other_2114;
    let _e4: Flector = self_2437;
    let _e8: Motor = other_2115;
    let _e20: Flector = self_2437;
    let _e24: Motor = other_2115;
    let _e37: Flector = self_2437;
    let _e40: Motor = other_2115;
    let _e43: Motor = other_2115;
    let _e46: Motor = other_2115;
    let _e49: Motor = other_2115;
    return Point(((((vec4<f32>(_e4.g1_.y) * vec4<f32>(_e8.g1_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0))) + ((vec4<f32>(_e20.g1_.z) * vec4<f32>(_e24.g1_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((_e37.g1_.wwwx * vec4<f32>(_e40.g1_.x, _e43.g1_.y, _e46.g1_.z, _e49.g1_.x)) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))));
}

fn flector_motor_left_anti_contraction(self_2438: Flector, other_2116: Motor) -> Plane {
    var self_2439: Flector;
    var other_2117: Motor;

    self_2439 = self_2438;
    other_2117 = other_2116;
    let _e4: Flector = self_2439;
    let _e8: Motor = other_2117;
    let _e11: Motor = other_2117;
    let _e14: Motor = other_2117;
    let _e17: Motor = other_2117;
    let _e29: Flector = self_2439;
    let _e33: Motor = other_2117;
    let _e36: Motor = other_2117;
    let _e39: Motor = other_2117;
    let _e42: Motor = other_2117;
    let _e55: Flector = self_2439;
    let _e59: Motor = other_2117;
    let _e62: Motor = other_2117;
    let _e65: Motor = other_2117;
    let _e68: Motor = other_2117;
    return Plane(((((vec4<f32>(_e4.g1_.y) * vec4<f32>(_e8.g0_.z, _e11.g0_.z, _e14.g0_.x, _e17.g1_.y)) * vec4<f32>(1.0, 0.0, -(1.0), 1.0)) + ((vec4<f32>(_e29.g1_.z) * vec4<f32>(_e33.g0_.y, _e36.g0_.x, _e39.g0_.y, _e42.g1_.z)) * vec4<f32>(-(1.0), 1.0, 0.0, 1.0))) + ((vec4<f32>(_e55.g1_.x) * vec4<f32>(_e59.g0_.x, _e62.g0_.z, _e65.g0_.y, _e68.g1_.x)) * vec4<f32>(0.0, -(1.0), 1.0, 1.0))));
}

fn flector_motor_right_anti_contraction(self_2440: Flector, other_2118: Motor) -> Flector {
    var self_2441: Flector;
    var other_2119: Motor;

    self_2441 = self_2440;
    other_2119 = other_2118;
    let _e4: Flector = self_2441;
    let _e6: Motor = other_2119;
    let _e11: Flector = self_2441;
    let _e15: Motor = other_2119;
    let _e27: Flector = self_2441;
    let _e31: Motor = other_2119;
    let _e44: Flector = self_2441;
    let _e48: Motor = other_2119;
    let _e60: Flector = self_2441;
    let _e64: Motor = other_2119;
    let _e76: Flector = self_2441;
    let _e80: Motor = other_2119;
    let _e92: Flector = self_2441;
    let _e96: Motor = other_2119;
    let _e108: Flector = self_2441;
    let _e111: Motor = other_2119;
    return Flector((_e4.g0_ * vec4<f32>(_e6.g0_.w)), ((((((((vec4<f32>(_e11.g0_.y) * vec4<f32>(_e15.g0_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0))) + ((vec4<f32>(_e27.g0_.z) * vec4<f32>(_e31.g0_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e44.g1_.x) * vec4<f32>(_e48.g0_.w)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e60.g1_.y) * vec4<f32>(_e64.g0_.w)) * vec4<f32>(0.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e76.g1_.z) * vec4<f32>(_e80.g0_.w)) * vec4<f32>(0.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e92.g1_.w) * vec4<f32>(_e96.g0_.w)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((_e108.g0_.wwwx * _e111.g0_.xyzx) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))));
}

fn flector_rotor_regressive_product(self_2442: Flector, other_2120: Rotor) -> Flector {
    var self_2443: Flector;
    var other_2121: Rotor;

    self_2443 = self_2442;
    other_2121 = other_2120;
    let _e4: Flector = self_2443;
    let _e8: Rotor = other_2121;
    let _e20: Flector = self_2443;
    let _e24: Rotor = other_2121;
    let _e37: Flector = self_2443;
    let _e41: Rotor = other_2121;
    let _e54: Flector = self_2443;
    let _e58: Rotor = other_2121;
    let _e69: Flector = self_2443;
    let _e71: Rotor = other_2121;
    let _e77: Flector = self_2443;
    let _e79: Rotor = other_2121;
    return Flector(((((((vec4<f32>(_e4.g1_.x) * vec4<f32>(_e8.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0))) + ((vec4<f32>(_e20.g1_.y) * vec4<f32>(_e24.g0_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e37.g1_.z) * vec4<f32>(_e41.g0_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e54.g1_.w) * _e58.g0_.xyzx) * vec4<f32>(1.0, 1.0, 1.0, 0.0))) + (_e69.g0_ * vec4<f32>(_e71.g0_.w))), (_e77.g1_ * vec4<f32>(_e79.g0_.w)));
}

fn flector_rotor_anti_wedge(self_2444: Flector, other_2122: Rotor) -> Flector {
    var self_2445: Flector;
    var other_2123: Rotor;

    self_2445 = self_2444;
    other_2123 = other_2122;
    let _e4: Flector = self_2445;
    let _e8: Rotor = other_2123;
    let _e20: Flector = self_2445;
    let _e24: Rotor = other_2123;
    let _e37: Flector = self_2445;
    let _e41: Rotor = other_2123;
    let _e54: Flector = self_2445;
    let _e58: Rotor = other_2123;
    let _e69: Flector = self_2445;
    let _e71: Rotor = other_2123;
    let _e77: Flector = self_2445;
    let _e79: Rotor = other_2123;
    return Flector(((((((vec4<f32>(_e4.g1_.x) * vec4<f32>(_e8.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0))) + ((vec4<f32>(_e20.g1_.y) * vec4<f32>(_e24.g0_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e37.g1_.z) * vec4<f32>(_e41.g0_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e54.g1_.w) * _e58.g0_.xyzx) * vec4<f32>(1.0, 1.0, 1.0, 0.0))) + (_e69.g0_ * vec4<f32>(_e71.g0_.w))), (_e77.g1_ * vec4<f32>(_e79.g0_.w)));
}

fn flector_rotor_meet(self_2446: Flector, other_2124: Rotor) -> Flector {
    var self_2447: Flector;
    var other_2125: Rotor;

    self_2447 = self_2446;
    other_2125 = other_2124;
    let _e4: Flector = self_2447;
    let _e8: Rotor = other_2125;
    let _e20: Flector = self_2447;
    let _e24: Rotor = other_2125;
    let _e37: Flector = self_2447;
    let _e41: Rotor = other_2125;
    let _e54: Flector = self_2447;
    let _e58: Rotor = other_2125;
    let _e69: Flector = self_2447;
    let _e71: Rotor = other_2125;
    let _e77: Flector = self_2447;
    let _e79: Rotor = other_2125;
    return Flector(((((((vec4<f32>(_e4.g1_.x) * vec4<f32>(_e8.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0))) + ((vec4<f32>(_e20.g1_.y) * vec4<f32>(_e24.g0_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e37.g1_.z) * vec4<f32>(_e41.g0_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e54.g1_.w) * _e58.g0_.xyzx) * vec4<f32>(1.0, 1.0, 1.0, 0.0))) + (_e69.g0_ * vec4<f32>(_e71.g0_.w))), (_e77.g1_ * vec4<f32>(_e79.g0_.w)));
}

fn flector_rotor_geometric_anti_product(self_2448: Flector, other_2126: Rotor) -> Flector {
    var self_2449: Flector;
    var other_2127: Rotor;

    self_2449 = self_2448;
    other_2127 = other_2126;
    let _e4: Flector = self_2449;
    let _e8: Rotor = other_2127;
    let _e19: Flector = self_2449;
    let _e23: Rotor = other_2127;
    let _e35: Flector = self_2449;
    let _e39: Rotor = other_2127;
    let _e52: Flector = self_2449;
    let _e56: Rotor = other_2127;
    let _e69: Flector = self_2449;
    let _e73: Rotor = other_2127;
    let _e86: Flector = self_2449;
    let _e90: Rotor = other_2127;
    let _e101: Flector = self_2449;
    let _e104: Rotor = other_2127;
    let _e116: Flector = self_2449;
    let _e120: Rotor = other_2127;
    let _e132: Flector = self_2449;
    let _e136: Rotor = other_2127;
    let _e149: Flector = self_2449;
    let _e153: Rotor = other_2127;
    let _e165: Flector = self_2449;
    let _e169: Rotor = other_2127;
    let _e181: Flector = self_2449;
    let _e185: Rotor = other_2127;
    let _e197: Flector = self_2449;
    let _e201: Rotor = other_2127;
    let _e213: Flector = self_2449;
    let _e216: Rotor = other_2127;
    return Flector(((((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.zwxz) * vec4<f32>(1.0, 1.0, -(1.0), 0.0)) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.yxwy) * vec4<f32>(-(1.0), 1.0, 1.0, 0.0))) + ((vec4<f32>(_e35.g1_.x) * vec4<f32>(_e39.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e52.g1_.y) * vec4<f32>(_e56.g0_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e69.g1_.z) * vec4<f32>(_e73.g0_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e86.g1_.w) * _e90.g0_.xyzx) * vec4<f32>(1.0, 1.0, 1.0, 0.0))) + ((_e101.g0_.xxxw * _e104.g0_.wzyw) * vec4<f32>(1.0, -(1.0), 1.0, 1.0))), ((((((((vec4<f32>(_e116.g0_.y) * vec4<f32>(_e120.g0_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0))) + ((vec4<f32>(_e132.g0_.z) * vec4<f32>(_e136.g0_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e149.g1_.x) * _e153.g0_.wzyw) * vec4<f32>(1.0, -(1.0), 1.0, 0.0))) + ((vec4<f32>(_e165.g1_.y) * _e169.g0_.zwxz) * vec4<f32>(1.0, 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e181.g1_.z) * _e185.g0_.yxwy) * vec4<f32>(-(1.0), 1.0, 1.0, 0.0))) + ((vec4<f32>(_e197.g1_.w) * vec4<f32>(_e201.g0_.w)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((_e213.g0_.wwwx * _e216.g0_.xyzx) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))));
}

fn flector_rotor_inner_anti_product(self_2450: Flector, other_2128: Rotor) -> Flector {
    var self_2451: Flector;
    var other_2129: Rotor;

    self_2451 = self_2450;
    other_2129 = other_2128;
    let _e4: Flector = self_2451;
    let _e6: Rotor = other_2129;
    let _e11: Flector = self_2451;
    let _e15: Rotor = other_2129;
    let _e27: Flector = self_2451;
    let _e31: Rotor = other_2129;
    let _e44: Flector = self_2451;
    let _e48: Rotor = other_2129;
    let _e60: Flector = self_2451;
    let _e64: Rotor = other_2129;
    let _e76: Flector = self_2451;
    let _e80: Rotor = other_2129;
    let _e92: Flector = self_2451;
    let _e96: Rotor = other_2129;
    let _e108: Flector = self_2451;
    let _e111: Rotor = other_2129;
    return Flector((_e4.g0_ * vec4<f32>(_e6.g0_.w)), ((((((((vec4<f32>(_e11.g0_.y) * vec4<f32>(_e15.g0_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0))) + ((vec4<f32>(_e27.g0_.z) * vec4<f32>(_e31.g0_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e44.g1_.x) * _e48.g0_.wzyw) * vec4<f32>(1.0, -(1.0), 1.0, 0.0))) + ((vec4<f32>(_e60.g1_.y) * _e64.g0_.zwxz) * vec4<f32>(1.0, 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e76.g1_.z) * _e80.g0_.yxwy) * vec4<f32>(-(1.0), 1.0, 1.0, 0.0))) + ((vec4<f32>(_e92.g1_.w) * vec4<f32>(_e96.g0_.w)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((_e108.g0_.wwwx * _e111.g0_.xyzx) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))));
}

fn flector_rotor_right_anti_contraction(self_2452: Flector, other_2130: Rotor) -> Flector {
    var self_2453: Flector;
    var other_2131: Rotor;

    self_2453 = self_2452;
    other_2131 = other_2130;
    let _e4: Flector = self_2453;
    let _e6: Rotor = other_2131;
    let _e11: Flector = self_2453;
    let _e15: Rotor = other_2131;
    let _e27: Flector = self_2453;
    let _e31: Rotor = other_2131;
    let _e44: Flector = self_2453;
    let _e48: Rotor = other_2131;
    let _e60: Flector = self_2453;
    let _e64: Rotor = other_2131;
    let _e76: Flector = self_2453;
    let _e80: Rotor = other_2131;
    let _e92: Flector = self_2453;
    let _e96: Rotor = other_2131;
    let _e108: Flector = self_2453;
    let _e111: Rotor = other_2131;
    return Flector((_e4.g0_ * vec4<f32>(_e6.g0_.w)), ((((((((vec4<f32>(_e11.g0_.y) * vec4<f32>(_e15.g0_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0))) + ((vec4<f32>(_e27.g0_.z) * vec4<f32>(_e31.g0_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e44.g1_.x) * vec4<f32>(_e48.g0_.w)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e60.g1_.y) * vec4<f32>(_e64.g0_.w)) * vec4<f32>(0.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e76.g1_.z) * vec4<f32>(_e80.g0_.w)) * vec4<f32>(0.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e92.g1_.w) * vec4<f32>(_e96.g0_.w)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((_e108.g0_.wwwx * _e111.g0_.xyzx) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))));
}

fn flector_translator_geometric_product(self_2454: Flector, other_2132: Translator) -> Flector {
    var self_2455: Flector;
    var other_2133: Translator;

    self_2455 = self_2454;
    other_2133 = other_2132;
    let _e4: Flector = self_2455;
    let _e8: Translator = other_2133;
    let _e19: Flector = self_2455;
    let _e23: Translator = other_2133;
    let _e35: Flector = self_2455;
    let _e39: Translator = other_2133;
    let _e52: Flector = self_2455;
    let _e56: Translator = other_2133;
    let _e69: Flector = self_2455;
    let _e73: Translator = other_2133;
    let _e77: Flector = self_2455;
    let _e80: Flector = self_2455;
    let _e83: Flector = self_2455;
    let _e86: Flector = self_2455;
    let _e90: Translator = other_2133;
    let _e103: Flector = self_2455;
    let _e107: Translator = other_2133;
    let _e118: Flector = self_2455;
    let _e122: Translator = other_2133;
    let _e134: Flector = self_2455;
    let _e138: Translator = other_2133;
    let _e149: Flector = self_2455;
    let _e153: Translator = other_2133;
    let _e165: Flector = self_2455;
    let _e169: Translator = other_2133;
    let _e181: Flector = self_2455;
    let _e184: Flector = self_2455;
    let _e187: Flector = self_2455;
    let _e190: Flector = self_2455;
    let _e194: Translator = other_2133;
    return Flector((((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.zzxz) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0)) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.yxyy) * vec4<f32>(1.0, -(1.0), 0.0, 0.0))) + ((vec4<f32>(_e35.g1_.y) * vec4<f32>(_e39.g0_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e52.g1_.z) * vec4<f32>(_e56.g0_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + (vec4<f32>(_e69.g1_.w) * _e73.g0_)) + ((vec4<f32>(_e77.g0_.x, _e80.g0_.x, _e83.g0_.x, _e86.g1_.x) * _e90.g0_.xzyx) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))), (((((((vec4<f32>(_e103.g0_.y) * _e107.g0_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e118.g0_.z) * _e122.g0_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e134.g0_.w) * _e138.g0_.xyzx) * vec4<f32>(1.0, 1.0, 1.0, 0.0))) + ((vec4<f32>(_e149.g1_.y) * _e153.g0_.zzxz) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0))) + ((vec4<f32>(_e165.g1_.z) * _e169.g0_.yxyy) * vec4<f32>(1.0, -(1.0), 0.0, 0.0))) + ((vec4<f32>(_e181.g0_.x, _e184.g1_.x, _e187.g1_.x, _e190.g0_.x) * _e194.g0_.wzyx) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))));
}

fn flector_translator_regressive_product(self_2456: Flector, other_2134: Translator) -> Flector {
    var self_2457: Flector;
    var other_2135: Translator;

    self_2457 = self_2456;
    other_2135 = other_2134;
    let _e4: Flector = self_2457;
    let _e8: Translator = other_2135;
    let _e19: Flector = self_2457;
    let _e23: Translator = other_2135;
    let _e35: Flector = self_2457;
    let _e39: Translator = other_2135;
    let _e51: Flector = self_2457;
    let _e53: Translator = other_2135;
    let _e59: Flector = self_2457;
    let _e61: Translator = other_2135;
    return Flector((((((vec4<f32>(_e4.g1_.x) * _e8.g0_.zzyz) * vec4<f32>(0.0, 1.0, -(1.0), 0.0)) + ((vec4<f32>(_e19.g1_.y) * _e23.g0_.zzxz) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0))) + ((vec4<f32>(_e35.g1_.z) * _e39.g0_.yxyy) * vec4<f32>(1.0, -(1.0), 0.0, 0.0))) + (_e51.g0_ * vec4<f32>(_e53.g0_.w))), (_e59.g1_ * vec4<f32>(_e61.g0_.w)));
}

fn flector_translator_anti_wedge(self_2458: Flector, other_2136: Translator) -> Flector {
    var self_2459: Flector;
    var other_2137: Translator;

    self_2459 = self_2458;
    other_2137 = other_2136;
    let _e4: Flector = self_2459;
    let _e8: Translator = other_2137;
    let _e19: Flector = self_2459;
    let _e23: Translator = other_2137;
    let _e35: Flector = self_2459;
    let _e39: Translator = other_2137;
    let _e51: Flector = self_2459;
    let _e53: Translator = other_2137;
    let _e59: Flector = self_2459;
    let _e61: Translator = other_2137;
    return Flector((((((vec4<f32>(_e4.g1_.x) * _e8.g0_.zzyz) * vec4<f32>(0.0, 1.0, -(1.0), 0.0)) + ((vec4<f32>(_e19.g1_.y) * _e23.g0_.zzxz) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0))) + ((vec4<f32>(_e35.g1_.z) * _e39.g0_.yxyy) * vec4<f32>(1.0, -(1.0), 0.0, 0.0))) + (_e51.g0_ * vec4<f32>(_e53.g0_.w))), (_e59.g1_ * vec4<f32>(_e61.g0_.w)));
}

fn flector_translator_meet(self_2460: Flector, other_2138: Translator) -> Flector {
    var self_2461: Flector;
    var other_2139: Translator;

    self_2461 = self_2460;
    other_2139 = other_2138;
    let _e4: Flector = self_2461;
    let _e8: Translator = other_2139;
    let _e19: Flector = self_2461;
    let _e23: Translator = other_2139;
    let _e35: Flector = self_2461;
    let _e39: Translator = other_2139;
    let _e51: Flector = self_2461;
    let _e53: Translator = other_2139;
    let _e59: Flector = self_2461;
    let _e61: Translator = other_2139;
    return Flector((((((vec4<f32>(_e4.g1_.x) * _e8.g0_.zzyz) * vec4<f32>(0.0, 1.0, -(1.0), 0.0)) + ((vec4<f32>(_e19.g1_.y) * _e23.g0_.zzxz) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0))) + ((vec4<f32>(_e35.g1_.z) * _e39.g0_.yxyy) * vec4<f32>(1.0, -(1.0), 0.0, 0.0))) + (_e51.g0_ * vec4<f32>(_e53.g0_.w))), (_e59.g1_ * vec4<f32>(_e61.g0_.w)));
}

fn flector_translator_outer_product(self_2462: Flector, other_2140: Translator) -> Plane {
    var self_2463: Flector;
    var other_2141: Translator;

    self_2463 = self_2462;
    other_2141 = other_2140;
    let _e4: Flector = self_2463;
    let _e8: Translator = other_2141;
    let _e20: Flector = self_2463;
    let _e24: Translator = other_2141;
    let _e37: Flector = self_2463;
    let _e40: Translator = other_2141;
    return Plane(((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0))) + ((vec4<f32>(_e20.g0_.z) * vec4<f32>(_e24.g0_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((_e37.g0_.wwwx * _e40.g0_.xyzx) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))));
}

fn flector_translator_wedge(self_2464: Flector, other_2142: Translator) -> Plane {
    var self_2465: Flector;
    var other_2143: Translator;

    self_2465 = self_2464;
    other_2143 = other_2142;
    let _e4: Flector = self_2465;
    let _e8: Translator = other_2143;
    let _e20: Flector = self_2465;
    let _e24: Translator = other_2143;
    let _e37: Flector = self_2465;
    let _e40: Translator = other_2143;
    return Plane(((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0))) + ((vec4<f32>(_e20.g0_.z) * vec4<f32>(_e24.g0_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((_e37.g0_.wwwx * _e40.g0_.xyzx) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))));
}

fn flector_translator_join(self_2466: Flector, other_2144: Translator) -> Plane {
    var self_2467: Flector;
    var other_2145: Translator;

    self_2467 = self_2466;
    other_2145 = other_2144;
    let _e4: Flector = self_2467;
    let _e8: Translator = other_2145;
    let _e20: Flector = self_2467;
    let _e24: Translator = other_2145;
    let _e37: Flector = self_2467;
    let _e40: Translator = other_2145;
    return Plane(((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0))) + ((vec4<f32>(_e20.g0_.z) * vec4<f32>(_e24.g0_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((_e37.g0_.wwwx * _e40.g0_.xyzx) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))));
}

fn flector_translator_geometric_anti_product(self_2468: Flector, other_2146: Translator) -> Flector {
    var self_2469: Flector;
    var other_2147: Translator;

    self_2469 = self_2468;
    other_2147 = other_2146;
    let _e4: Flector = self_2469;
    let _e8: Translator = other_2147;
    let _e20: Flector = self_2469;
    let _e24: Translator = other_2147;
    let _e36: Flector = self_2469;
    let _e40: Translator = other_2147;
    let _e52: Flector = self_2469;
    let _e56: Translator = other_2147;
    let _e68: Flector = self_2469;
    let _e71: Translator = other_2147;
    let _e82: Flector = self_2469;
    let _e86: Translator = other_2147;
    let _e96: Flector = self_2469;
    let _e100: Translator = other_2147;
    let _e111: Flector = self_2469;
    let _e115: Translator = other_2147;
    let _e127: Flector = self_2469;
    let _e131: Translator = other_2147;
    return Flector(((((((vec4<f32>(_e4.g0_.w) * _e8.g0_) * vec4<f32>(-(1.0), -(1.0), -(1.0), 1.0)) + ((vec4<f32>(_e20.g1_.x) * _e24.g0_.zzyz) * vec4<f32>(0.0, 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e36.g1_.y) * _e40.g0_.zzxz) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0))) + ((vec4<f32>(_e52.g1_.z) * _e56.g0_.yxyy) * vec4<f32>(1.0, -(1.0), 0.0, 0.0))) + ((_e68.g0_.xyzx * _e71.g0_.wwwx) * vec4<f32>(1.0, 1.0, 1.0, 0.0))), (((((vec4<f32>(_e82.g1_.y) * _e86.g0_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, 1.0)) + ((vec4<f32>(_e96.g1_.z) * _e100.g0_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, 1.0))) + ((vec4<f32>(_e111.g1_.w) * vec4<f32>(_e115.g0_.w)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e127.g1_.x) * _e131.g0_.wxxx) * vec4<f32>(1.0, 0.0, 0.0, 1.0))));
}

fn flector_translator_inner_anti_product(self_2470: Flector, other_2148: Translator) -> Flector {
    var self_2471: Flector;
    var other_2149: Translator;

    self_2471 = self_2470;
    other_2149 = other_2148;
    let _e4: Flector = self_2471;
    let _e6: Translator = other_2149;
    let _e11: Flector = self_2471;
    let _e15: Translator = other_2149;
    let _e25: Flector = self_2471;
    let _e29: Translator = other_2149;
    let _e40: Flector = self_2471;
    let _e44: Translator = other_2149;
    let _e56: Flector = self_2471;
    let _e60: Translator = other_2149;
    return Flector((_e4.g0_ * vec4<f32>(_e6.g0_.w)), (((((vec4<f32>(_e11.g1_.y) * _e15.g0_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, 1.0)) + ((vec4<f32>(_e25.g1_.z) * _e29.g0_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, 1.0))) + ((vec4<f32>(_e40.g1_.w) * vec4<f32>(_e44.g0_.w)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e56.g1_.x) * _e60.g0_.wxxx) * vec4<f32>(1.0, 0.0, 0.0, 1.0))));
}

fn flector_translator_right_contraction(self_2472: Flector, other_2150: Translator) -> Point {
    var self_2473: Flector;
    var other_2151: Translator;

    self_2473 = self_2472;
    other_2151 = other_2150;
    let _e4: Flector = self_2473;
    let _e8: Translator = other_2151;
    let _e20: Flector = self_2473;
    let _e24: Translator = other_2151;
    let _e37: Flector = self_2473;
    let _e40: Translator = other_2151;
    return Point(((((vec4<f32>(_e4.g1_.y) * vec4<f32>(_e8.g0_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0))) + ((vec4<f32>(_e20.g1_.z) * vec4<f32>(_e24.g0_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((_e37.g1_.wwwx * _e40.g0_.xyzx) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))));
}

fn flector_translator_right_anti_contraction(self_2474: Flector, other_2152: Translator) -> Flector {
    var self_2475: Flector;
    var other_2153: Translator;

    self_2475 = self_2474;
    other_2153 = other_2152;
    let _e4: Flector = self_2475;
    let _e6: Translator = other_2153;
    let _e11: Flector = self_2475;
    let _e13: Translator = other_2153;
    return Flector((_e4.g0_ * vec4<f32>(_e6.g0_.w)), (_e11.g1_ * vec4<f32>(_e13.g0_.w)));
}

fn flector_flector_add(self_2476: Flector, other_2154: Flector) -> Flector {
    var self_2477: Flector;
    var other_2155: Flector;

    self_2477 = self_2476;
    other_2155 = other_2154;
    let _e4: Flector = self_2477;
    let _e6: Flector = other_2155;
    let _e9: Flector = self_2477;
    let _e11: Flector = other_2155;
    return Flector((_e4.g0_ + _e6.g0_), (_e9.g1_ + _e11.g1_));
}

fn flector_flector_sub(self_2478: Flector, other_2156: Flector) -> Flector {
    var self_2479: Flector;
    var other_2157: Flector;

    self_2479 = self_2478;
    other_2157 = other_2156;
    let _e4: Flector = self_2479;
    let _e6: Flector = other_2157;
    let _e9: Flector = self_2479;
    let _e11: Flector = other_2157;
    return Flector((_e4.g0_ - _e6.g0_), (_e9.g1_ - _e11.g1_));
}

fn flector_flector_mul(self_2480: Flector, other_2158: Flector) -> Flector {
    var self_2481: Flector;
    var other_2159: Flector;

    self_2481 = self_2480;
    other_2159 = other_2158;
    let _e4: Flector = self_2481;
    let _e6: Flector = other_2159;
    let _e9: Flector = self_2481;
    let _e11: Flector = other_2159;
    return Flector((_e4.g0_ * _e6.g0_), (_e9.g1_ * _e11.g1_));
}

fn flector_flector_div(self_2482: Flector, other_2160: Flector) -> Flector {
    var self_2483: Flector;
    var other_2161: Flector;

    self_2483 = self_2482;
    other_2161 = other_2160;
    let _e4: Flector = self_2483;
    let _e7: Flector = self_2483;
    let _e10: Flector = self_2483;
    let _e13: Flector = self_2483;
    let _e23: Flector = other_2161;
    let _e26: Flector = other_2161;
    let _e29: Flector = other_2161;
    let _e32: Flector = other_2161;
    let _e43: Flector = self_2483;
    let _e46: Flector = self_2483;
    let _e49: Flector = self_2483;
    let _e52: Flector = self_2483;
    let _e62: Flector = other_2161;
    let _e65: Flector = other_2161;
    let _e68: Flector = other_2161;
    let _e71: Flector = other_2161;
    return Flector((((vec4<f32>(_e4.g0_.x, _e7.g0_.y, _e10.g0_.z, _e13.g0_.w) * vec4<f32>(1.0, 1.0, 1.0, 1.0)) / vec4<f32>(_e23.g0_.x, _e26.g0_.y, _e29.g0_.z, _e32.g0_.w)) * vec4<f32>(1.0, 1.0, 1.0, 1.0)), (((vec4<f32>(_e43.g1_.x, _e46.g1_.y, _e49.g1_.z, _e52.g1_.w) * vec4<f32>(1.0, 1.0, 1.0, 1.0)) / vec4<f32>(_e62.g1_.x, _e65.g1_.y, _e68.g1_.z, _e71.g1_.w)) * vec4<f32>(1.0, 1.0, 1.0, 1.0)));
}

fn flector_flector_outer_product(self_2484: Flector, other_2162: Flector) -> Motor {
    var self_2485: Flector;
    var other_2163: Flector;

    self_2485 = self_2484;
    other_2163 = other_2162;
    let _e4: Flector = self_2485;
    let _e8: Flector = other_2163;
    let _e11: Flector = other_2163;
    let _e14: Flector = other_2163;
    let _e17: Flector = other_2163;
    let _e29: Flector = self_2485;
    let _e33: Flector = other_2163;
    let _e36: Flector = other_2163;
    let _e39: Flector = other_2163;
    let _e42: Flector = other_2163;
    let _e55: Flector = self_2485;
    let _e59: Flector = other_2163;
    let _e62: Flector = other_2163;
    let _e65: Flector = other_2163;
    let _e68: Flector = other_2163;
    let _e74: Flector = self_2485;
    let _e78: Flector = other_2163;
    let _e91: Flector = self_2485;
    let _e95: Flector = other_2163;
    let _e108: Flector = self_2485;
    let _e112: Flector = other_2163;
    let _e125: Flector = self_2485;
    let _e129: Flector = other_2163;
    let _e142: Flector = self_2485;
    let _e146: Flector = other_2163;
    let _e149: Flector = other_2163;
    let _e152: Flector = other_2163;
    let _e155: Flector = other_2163;
    let _e168: Flector = self_2485;
    let _e172: Flector = other_2163;
    let _e175: Flector = other_2163;
    let _e178: Flector = other_2163;
    let _e189: Flector = self_2485;
    let _e193: Flector = other_2163;
    let _e196: Flector = other_2163;
    let _e199: Flector = other_2163;
    let _e211: Flector = self_2485;
    let _e215: Flector = other_2163;
    let _e218: Flector = other_2163;
    let _e221: Flector = other_2163;
    return Motor((((((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.w, _e11.g0_.w, _e14.g0_.w, _e17.g1_.y)) * vec4<f32>(0.0, -(1.0), 0.0, 1.0)) + ((vec4<f32>(_e29.g0_.z) * vec4<f32>(_e33.g0_.w, _e36.g0_.w, _e39.g0_.w, _e42.g1_.z)) * vec4<f32>(0.0, 0.0, -(1.0), 1.0))) + (vec4<f32>(_e55.g0_.w) * vec4<f32>(_e59.g0_.x, _e62.g0_.y, _e65.g0_.z, _e68.g1_.w))) + ((vec4<f32>(_e74.g1_.x) * vec4<f32>(_e78.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e91.g1_.y) * vec4<f32>(_e95.g0_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e108.g1_.z) * vec4<f32>(_e112.g0_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e125.g1_.w) * vec4<f32>(_e129.g0_.w)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e142.g0_.x) * vec4<f32>(_e146.g0_.w, _e149.g0_.x, _e152.g0_.x, _e155.g1_.x)) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))), ((((vec3<f32>(_e168.g0_.y) * vec3<f32>(_e172.g0_.z, _e175.g0_.z, _e178.g0_.x)) * vec3<f32>(1.0, 0.0, -(1.0))) + ((vec3<f32>(_e189.g0_.z) * vec3<f32>(_e193.g0_.y, _e196.g0_.x, _e199.g0_.y)) * vec3<f32>(-(1.0), 1.0, 0.0))) + ((vec3<f32>(_e211.g0_.x) * vec3<f32>(_e215.g0_.x, _e218.g0_.z, _e221.g0_.y)) * vec3<f32>(0.0, -(1.0), 1.0))));
}

fn flector_flector_wedge(self_2486: Flector, other_2164: Flector) -> Motor {
    var self_2487: Flector;
    var other_2165: Flector;

    self_2487 = self_2486;
    other_2165 = other_2164;
    let _e4: Flector = self_2487;
    let _e8: Flector = other_2165;
    let _e11: Flector = other_2165;
    let _e14: Flector = other_2165;
    let _e17: Flector = other_2165;
    let _e29: Flector = self_2487;
    let _e33: Flector = other_2165;
    let _e36: Flector = other_2165;
    let _e39: Flector = other_2165;
    let _e42: Flector = other_2165;
    let _e55: Flector = self_2487;
    let _e59: Flector = other_2165;
    let _e62: Flector = other_2165;
    let _e65: Flector = other_2165;
    let _e68: Flector = other_2165;
    let _e74: Flector = self_2487;
    let _e78: Flector = other_2165;
    let _e91: Flector = self_2487;
    let _e95: Flector = other_2165;
    let _e108: Flector = self_2487;
    let _e112: Flector = other_2165;
    let _e125: Flector = self_2487;
    let _e129: Flector = other_2165;
    let _e142: Flector = self_2487;
    let _e146: Flector = other_2165;
    let _e149: Flector = other_2165;
    let _e152: Flector = other_2165;
    let _e155: Flector = other_2165;
    let _e168: Flector = self_2487;
    let _e172: Flector = other_2165;
    let _e175: Flector = other_2165;
    let _e178: Flector = other_2165;
    let _e189: Flector = self_2487;
    let _e193: Flector = other_2165;
    let _e196: Flector = other_2165;
    let _e199: Flector = other_2165;
    let _e211: Flector = self_2487;
    let _e215: Flector = other_2165;
    let _e218: Flector = other_2165;
    let _e221: Flector = other_2165;
    return Motor((((((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.w, _e11.g0_.w, _e14.g0_.w, _e17.g1_.y)) * vec4<f32>(0.0, -(1.0), 0.0, 1.0)) + ((vec4<f32>(_e29.g0_.z) * vec4<f32>(_e33.g0_.w, _e36.g0_.w, _e39.g0_.w, _e42.g1_.z)) * vec4<f32>(0.0, 0.0, -(1.0), 1.0))) + (vec4<f32>(_e55.g0_.w) * vec4<f32>(_e59.g0_.x, _e62.g0_.y, _e65.g0_.z, _e68.g1_.w))) + ((vec4<f32>(_e74.g1_.x) * vec4<f32>(_e78.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e91.g1_.y) * vec4<f32>(_e95.g0_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e108.g1_.z) * vec4<f32>(_e112.g0_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e125.g1_.w) * vec4<f32>(_e129.g0_.w)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e142.g0_.x) * vec4<f32>(_e146.g0_.w, _e149.g0_.x, _e152.g0_.x, _e155.g1_.x)) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))), ((((vec3<f32>(_e168.g0_.y) * vec3<f32>(_e172.g0_.z, _e175.g0_.z, _e178.g0_.x)) * vec3<f32>(1.0, 0.0, -(1.0))) + ((vec3<f32>(_e189.g0_.z) * vec3<f32>(_e193.g0_.y, _e196.g0_.x, _e199.g0_.y)) * vec3<f32>(-(1.0), 1.0, 0.0))) + ((vec3<f32>(_e211.g0_.x) * vec3<f32>(_e215.g0_.x, _e218.g0_.z, _e221.g0_.y)) * vec3<f32>(0.0, -(1.0), 1.0))));
}

fn flector_flector_join(self_2488: Flector, other_2166: Flector) -> Motor {
    var self_2489: Flector;
    var other_2167: Flector;

    self_2489 = self_2488;
    other_2167 = other_2166;
    let _e4: Flector = self_2489;
    let _e8: Flector = other_2167;
    let _e11: Flector = other_2167;
    let _e14: Flector = other_2167;
    let _e17: Flector = other_2167;
    let _e29: Flector = self_2489;
    let _e33: Flector = other_2167;
    let _e36: Flector = other_2167;
    let _e39: Flector = other_2167;
    let _e42: Flector = other_2167;
    let _e55: Flector = self_2489;
    let _e59: Flector = other_2167;
    let _e62: Flector = other_2167;
    let _e65: Flector = other_2167;
    let _e68: Flector = other_2167;
    let _e74: Flector = self_2489;
    let _e78: Flector = other_2167;
    let _e91: Flector = self_2489;
    let _e95: Flector = other_2167;
    let _e108: Flector = self_2489;
    let _e112: Flector = other_2167;
    let _e125: Flector = self_2489;
    let _e129: Flector = other_2167;
    let _e142: Flector = self_2489;
    let _e146: Flector = other_2167;
    let _e149: Flector = other_2167;
    let _e152: Flector = other_2167;
    let _e155: Flector = other_2167;
    let _e168: Flector = self_2489;
    let _e172: Flector = other_2167;
    let _e175: Flector = other_2167;
    let _e178: Flector = other_2167;
    let _e189: Flector = self_2489;
    let _e193: Flector = other_2167;
    let _e196: Flector = other_2167;
    let _e199: Flector = other_2167;
    let _e211: Flector = self_2489;
    let _e215: Flector = other_2167;
    let _e218: Flector = other_2167;
    let _e221: Flector = other_2167;
    return Motor((((((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.w, _e11.g0_.w, _e14.g0_.w, _e17.g1_.y)) * vec4<f32>(0.0, -(1.0), 0.0, 1.0)) + ((vec4<f32>(_e29.g0_.z) * vec4<f32>(_e33.g0_.w, _e36.g0_.w, _e39.g0_.w, _e42.g1_.z)) * vec4<f32>(0.0, 0.0, -(1.0), 1.0))) + (vec4<f32>(_e55.g0_.w) * vec4<f32>(_e59.g0_.x, _e62.g0_.y, _e65.g0_.z, _e68.g1_.w))) + ((vec4<f32>(_e74.g1_.x) * vec4<f32>(_e78.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e91.g1_.y) * vec4<f32>(_e95.g0_.y)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e108.g1_.z) * vec4<f32>(_e112.g0_.z)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e125.g1_.w) * vec4<f32>(_e129.g0_.w)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e142.g0_.x) * vec4<f32>(_e146.g0_.w, _e149.g0_.x, _e152.g0_.x, _e155.g1_.x)) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))), ((((vec3<f32>(_e168.g0_.y) * vec3<f32>(_e172.g0_.z, _e175.g0_.z, _e178.g0_.x)) * vec3<f32>(1.0, 0.0, -(1.0))) + ((vec3<f32>(_e189.g0_.z) * vec3<f32>(_e193.g0_.y, _e196.g0_.x, _e199.g0_.y)) * vec3<f32>(-(1.0), 1.0, 0.0))) + ((vec3<f32>(_e211.g0_.x) * vec3<f32>(_e215.g0_.x, _e218.g0_.z, _e221.g0_.y)) * vec3<f32>(0.0, -(1.0), 1.0))));
}

fn flector_flector_inner_anti_product(self_2490: Flector, other_2168: Flector) -> Motor {
    var self_2491: Flector;
    var other_2169: Flector;

    self_2491 = self_2490;
    other_2169 = other_2168;
    let _e6: Flector = self_2491;
    let _e10: Flector = other_2169;
    let _e13: Flector = other_2169;
    let _e16: Flector = other_2169;
    let _e19: Flector = other_2169;
    let _e25: Flector = self_2491;
    let _e29: Flector = other_2169;
    let _e32: Flector = other_2169;
    let _e35: Flector = other_2169;
    let _e38: Flector = other_2169;
    let _e51: Flector = self_2491;
    let _e55: Flector = other_2169;
    let _e58: Flector = other_2169;
    let _e61: Flector = other_2169;
    let _e64: Flector = other_2169;
    let _e77: Flector = self_2491;
    let _e81: Flector = other_2169;
    let _e84: Flector = other_2169;
    let _e87: Flector = other_2169;
    let _e90: Flector = other_2169;
    let _e103: Flector = self_2491;
    let _e107: Flector = other_2169;
    let _e110: Flector = other_2169;
    let _e113: Flector = other_2169;
    let _e124: Flector = self_2491;
    let _e128: Flector = other_2169;
    let _e131: Flector = other_2169;
    let _e134: Flector = other_2169;
    let _e146: Flector = self_2491;
    let _e150: Flector = other_2169;
    let _e153: Flector = other_2169;
    let _e156: Flector = other_2169;
    let _e168: Flector = self_2491;
    let _e172: Flector = other_2169;
    let _e175: Flector = other_2169;
    let _e178: Flector = other_2169;
    let _e190: Flector = self_2491;
    let _e194: Flector = other_2169;
    let _e197: Flector = other_2169;
    let _e200: Flector = other_2169;
    let _e212: Flector = self_2491;
    let _e216: Flector = other_2169;
    let _e219: Flector = other_2169;
    let _e222: Flector = other_2169;
    return Motor(((((vec4<f32>(0.0) - (vec4<f32>(_e6.g0_.w) * vec4<f32>(_e10.g1_.x, _e13.g1_.y, _e16.g1_.z, _e19.g0_.w))) + ((vec4<f32>(_e25.g1_.y) * vec4<f32>(_e29.g0_.w, _e32.g0_.w, _e35.g0_.w, _e38.g1_.y)) * vec4<f32>(0.0, -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e51.g1_.z) * vec4<f32>(_e55.g0_.w, _e58.g0_.w, _e61.g0_.w, _e64.g1_.z)) * vec4<f32>(0.0, 0.0, -(1.0), 1.0))) + ((vec4<f32>(_e77.g1_.x) * vec4<f32>(_e81.g0_.w, _e84.g0_.x, _e87.g0_.x, _e90.g1_.x)) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))), (((((((vec3<f32>(_e103.g0_.y) * vec3<f32>(_e107.g1_.z, _e110.g1_.z, _e113.g1_.x)) * vec3<f32>(-(1.0), 0.0, 1.0)) + ((vec3<f32>(_e124.g0_.z) * vec3<f32>(_e128.g1_.y, _e131.g1_.x, _e134.g1_.y)) * vec3<f32>(1.0, -(1.0), 0.0))) + ((vec3<f32>(_e146.g1_.x) * vec3<f32>(_e150.g0_.z, _e153.g0_.z, _e156.g0_.y)) * vec3<f32>(0.0, -(1.0), 1.0))) + ((vec3<f32>(_e168.g1_.y) * vec3<f32>(_e172.g0_.z, _e175.g0_.z, _e178.g0_.x)) * vec3<f32>(1.0, 0.0, -(1.0)))) + ((vec3<f32>(_e190.g1_.z) * vec3<f32>(_e194.g0_.y, _e197.g0_.x, _e200.g0_.y)) * vec3<f32>(-(1.0), 1.0, 0.0))) + ((vec3<f32>(_e212.g0_.x) * vec3<f32>(_e216.g1_.x, _e219.g1_.z, _e222.g1_.y)) * vec3<f32>(0.0, 1.0, -(1.0)))));
}

fn flector_flector_left_anti_contraction(self_2492: Flector, other_2170: Flector) -> Motor {
    var self_2493: Flector;
    var other_2171: Flector;

    self_2493 = self_2492;
    other_2171 = other_2170;
    let _e4: Flector = self_2493;
    let _e8: Flector = other_2171;
    let _e11: Flector = other_2171;
    let _e14: Flector = other_2171;
    let _e17: Flector = other_2171;
    let _e29: Flector = self_2493;
    let _e33: Flector = other_2171;
    let _e36: Flector = other_2171;
    let _e39: Flector = other_2171;
    let _e42: Flector = other_2171;
    let _e55: Flector = self_2493;
    let _e59: Flector = other_2171;
    let _e62: Flector = other_2171;
    let _e65: Flector = other_2171;
    let _e68: Flector = other_2171;
    let _e81: Flector = self_2493;
    let _e84: Flector = other_2171;
    let _e96: Flector = self_2493;
    let _e100: Flector = other_2171;
    let _e103: Flector = other_2171;
    let _e106: Flector = other_2171;
    let _e117: Flector = self_2493;
    let _e121: Flector = other_2171;
    let _e124: Flector = other_2171;
    let _e127: Flector = other_2171;
    let _e139: Flector = self_2493;
    let _e143: Flector = other_2171;
    let _e146: Flector = other_2171;
    let _e149: Flector = other_2171;
    return Motor((((((vec4<f32>(_e4.g1_.x) * vec4<f32>(_e8.g0_.w, _e11.g0_.w, _e14.g0_.w, _e17.g1_.x)) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0)) + ((vec4<f32>(_e29.g1_.y) * vec4<f32>(_e33.g0_.w, _e36.g0_.w, _e39.g0_.w, _e42.g1_.y)) * vec4<f32>(0.0, -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e55.g1_.z) * vec4<f32>(_e59.g0_.w, _e62.g0_.w, _e65.g0_.w, _e68.g1_.z)) * vec4<f32>(0.0, 0.0, -(1.0), 1.0))) + ((_e81.g0_.xxxw * _e84.g0_.xxxw) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))), ((((vec3<f32>(_e96.g1_.y) * vec3<f32>(_e100.g0_.z, _e103.g0_.z, _e106.g0_.x)) * vec3<f32>(1.0, 0.0, -(1.0))) + ((vec3<f32>(_e117.g1_.z) * vec3<f32>(_e121.g0_.y, _e124.g0_.x, _e127.g0_.y)) * vec3<f32>(-(1.0), 1.0, 0.0))) + ((vec3<f32>(_e139.g1_.x) * vec3<f32>(_e143.g0_.x, _e146.g0_.z, _e149.g0_.y)) * vec3<f32>(0.0, -(1.0), 1.0))));
}

fn flector_flector_right_anti_contraction(self_2494: Flector, other_2172: Flector) -> Motor {
    var self_2495: Flector;
    var other_2173: Flector;

    self_2495 = self_2494;
    other_2173 = other_2172;
    let _e6: Flector = self_2495;
    let _e10: Flector = other_2173;
    let _e13: Flector = other_2173;
    let _e16: Flector = other_2173;
    let _e19: Flector = other_2173;
    let _e25: Flector = self_2495;
    let _e29: Flector = other_2173;
    let _e41: Flector = self_2495;
    let _e45: Flector = other_2173;
    let _e57: Flector = self_2495;
    let _e61: Flector = other_2173;
    let _e73: Flector = self_2495;
    let _e77: Flector = other_2173;
    let _e80: Flector = other_2173;
    let _e83: Flector = other_2173;
    let _e94: Flector = self_2495;
    let _e98: Flector = other_2173;
    let _e101: Flector = other_2173;
    let _e104: Flector = other_2173;
    let _e116: Flector = self_2495;
    let _e120: Flector = other_2173;
    let _e123: Flector = other_2173;
    let _e126: Flector = other_2173;
    return Motor(((((vec4<f32>(0.0) - (vec4<f32>(_e6.g0_.w) * vec4<f32>(_e10.g1_.x, _e13.g1_.y, _e16.g1_.z, _e19.g0_.w))) + ((vec4<f32>(_e25.g1_.y) * vec4<f32>(_e29.g1_.y)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e41.g1_.z) * vec4<f32>(_e45.g1_.z)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e57.g1_.x) * vec4<f32>(_e61.g1_.x)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))), ((((vec3<f32>(_e73.g0_.y) * vec3<f32>(_e77.g1_.z, _e80.g1_.z, _e83.g1_.x)) * vec3<f32>(-(1.0), 0.0, 1.0)) + ((vec3<f32>(_e94.g0_.z) * vec3<f32>(_e98.g1_.y, _e101.g1_.x, _e104.g1_.y)) * vec3<f32>(1.0, -(1.0), 0.0))) + ((vec3<f32>(_e116.g0_.x) * vec3<f32>(_e120.g1_.x, _e123.g1_.z, _e126.g1_.y)) * vec3<f32>(0.0, 1.0, -(1.0)))));
}

fn flector_flector_scalar_product(self_2496: Flector, other_2174: Flector) -> Scalar {
    var self_2497: Flector;
    var other_2175: Flector;

    self_2497 = self_2496;
    other_2175 = other_2174;
    let _e4: Flector = self_2497;
    let _e7: Flector = other_2175;
    let _e11: Flector = self_2497;
    let _e14: Flector = other_2175;
    let _e19: Flector = self_2497;
    let _e22: Flector = other_2175;
    let _e27: Flector = self_2497;
    let _e30: Flector = other_2175;
    return Scalar(((((_e4.g0_.x * _e7.g0_.x) + (_e11.g0_.y * _e14.g0_.y)) + (_e19.g0_.z * _e22.g0_.z)) - (_e27.g1_.w * _e30.g1_.w)));
}

fn flector_flector_dot(self_2498: Flector, other_2176: Flector) -> Scalar {
    var self_2499: Flector;
    var other_2177: Flector;

    self_2499 = self_2498;
    other_2177 = other_2176;
    let _e4: Flector = self_2499;
    let _e7: Flector = other_2177;
    let _e11: Flector = self_2499;
    let _e14: Flector = other_2177;
    let _e19: Flector = self_2499;
    let _e22: Flector = other_2177;
    let _e27: Flector = self_2499;
    let _e30: Flector = other_2177;
    return Scalar(((((_e4.g0_.x * _e7.g0_.x) + (_e11.g0_.y * _e14.g0_.y)) + (_e19.g0_.z * _e22.g0_.z)) - (_e27.g1_.w * _e30.g1_.w)));
}

fn flector_flector_anti_scalar_product(self_2500: Flector, other_2178: Flector) -> AntiScalar {
    var self_2501: Flector;
    var other_2179: Flector;

    self_2501 = self_2500;
    other_2179 = other_2178;
    let _e5: Flector = self_2501;
    let _e8: Flector = other_2179;
    let _e13: Flector = self_2501;
    let _e16: Flector = other_2179;
    let _e21: Flector = self_2501;
    let _e24: Flector = other_2179;
    let _e29: Flector = self_2501;
    let _e32: Flector = other_2179;
    return AntiScalar(((((0.0 - (_e5.g0_.w * _e8.g0_.w)) + (_e13.g1_.x * _e16.g1_.x)) + (_e21.g1_.y * _e24.g1_.y)) + (_e29.g1_.z * _e32.g1_.z)));
}

fn flector_flector_anti_dot(self_2502: Flector, other_2180: Flector) -> AntiScalar {
    var self_2503: Flector;
    var other_2181: Flector;

    self_2503 = self_2502;
    other_2181 = other_2180;
    let _e5: Flector = self_2503;
    let _e8: Flector = other_2181;
    let _e13: Flector = self_2503;
    let _e16: Flector = other_2181;
    let _e21: Flector = self_2503;
    let _e24: Flector = other_2181;
    let _e29: Flector = self_2503;
    let _e32: Flector = other_2181;
    return AntiScalar(((((0.0 - (_e5.g0_.w * _e8.g0_.w)) + (_e13.g1_.x * _e16.g1_.x)) + (_e21.g1_.y * _e24.g1_.y)) + (_e29.g1_.z * _e32.g1_.z)));
}

fn flector_multi_vector_scalar_product(self_2504: Flector, other_2182: MultiVector) -> Scalar {
    var self_2505: Flector;
    var other_2183: MultiVector;

    self_2505 = self_2504;
    other_2183 = other_2182;
    let _e4: Flector = self_2505;
    let _e7: MultiVector = other_2183;
    let _e11: Flector = self_2505;
    let _e14: MultiVector = other_2183;
    let _e19: Flector = self_2505;
    let _e22: MultiVector = other_2183;
    let _e27: Flector = self_2505;
    let _e30: MultiVector = other_2183;
    return Scalar(((((_e4.g0_.x * _e7.g1_.x) + (_e11.g0_.y * _e14.g1_.y)) + (_e19.g0_.z * _e22.g1_.z)) - (_e27.g1_.w * _e30.g6_.w)));
}

fn flector_multi_vector_dot(self_2506: Flector, other_2184: MultiVector) -> Scalar {
    var self_2507: Flector;
    var other_2185: MultiVector;

    self_2507 = self_2506;
    other_2185 = other_2184;
    let _e4: Flector = self_2507;
    let _e7: MultiVector = other_2185;
    let _e11: Flector = self_2507;
    let _e14: MultiVector = other_2185;
    let _e19: Flector = self_2507;
    let _e22: MultiVector = other_2185;
    let _e27: Flector = self_2507;
    let _e30: MultiVector = other_2185;
    return Scalar(((((_e4.g0_.x * _e7.g1_.x) + (_e11.g0_.y * _e14.g1_.y)) + (_e19.g0_.z * _e22.g1_.z)) - (_e27.g1_.w * _e30.g6_.w)));
}

fn flector_multi_vector_anti_scalar_product(self_2508: Flector, other_2186: MultiVector) -> AntiScalar {
    var self_2509: Flector;
    var other_2187: MultiVector;

    self_2509 = self_2508;
    other_2187 = other_2186;
    let _e5: Flector = self_2509;
    let _e8: MultiVector = other_2187;
    let _e13: Flector = self_2509;
    let _e16: MultiVector = other_2187;
    let _e21: Flector = self_2509;
    let _e24: MultiVector = other_2187;
    let _e29: Flector = self_2509;
    let _e32: MultiVector = other_2187;
    return AntiScalar(((((0.0 - (_e5.g0_.w * _e8.g1_.w)) + (_e13.g1_.x * _e16.g6_.x)) + (_e21.g1_.y * _e24.g6_.y)) + (_e29.g1_.z * _e32.g6_.z)));
}

fn flector_multi_vector_anti_dot(self_2510: Flector, other_2188: MultiVector) -> AntiScalar {
    var self_2511: Flector;
    var other_2189: MultiVector;

    self_2511 = self_2510;
    other_2189 = other_2188;
    let _e5: Flector = self_2511;
    let _e8: MultiVector = other_2189;
    let _e13: Flector = self_2511;
    let _e16: MultiVector = other_2189;
    let _e21: Flector = self_2511;
    let _e24: MultiVector = other_2189;
    let _e29: Flector = self_2511;
    let _e32: MultiVector = other_2189;
    return AntiScalar(((((0.0 - (_e5.g0_.w * _e8.g1_.w)) + (_e13.g1_.x * _e16.g6_.x)) + (_e21.g1_.y * _e24.g6_.y)) + (_e29.g1_.z * _e32.g6_.z)));
}

fn flector_squared_magnitude(self_2512: Flector) -> Scalar {
    var self_2513: Flector;

    self_2513 = self_2512;
    let _e2: Flector = self_2513;
    let _e3: Flector = self_2513;
    let _e4: Flector = flector_reversal(_e3);
    let _e5: Scalar = flector_flector_scalar_product(_e2, _e4);
    return _e5;
}

fn flector_magnitude(self_2514: Flector) -> Scalar {
    var self_2515: Flector;

    self_2515 = self_2514;
    let _e2: Flector = self_2515;
    let _e3: Scalar = flector_squared_magnitude(_e2);
    return Scalar(sqrt(_e3.g0_));
}

fn flector_bulk_norm(self_2516: Flector) -> Scalar {
    var self_2517: Flector;

    self_2517 = self_2516;
    let _e2: Flector = self_2517;
    let _e3: Scalar = flector_squared_magnitude(_e2);
    return Scalar(sqrt(_e3.g0_));
}

fn flector_squared_anti_magnitude(self_2518: Flector) -> AntiScalar {
    var self_2519: Flector;

    self_2519 = self_2518;
    let _e2: Flector = self_2519;
    let _e3: Flector = self_2519;
    let _e4: Flector = flector_anti_reversal(_e3);
    let _e5: AntiScalar = flector_flector_anti_scalar_product(_e2, _e4);
    return _e5;
}

fn flector_weight_norm(self_2520: Flector) -> AntiScalar {
    var self_2521: Flector;

    self_2521 = self_2520;
    let _e2: Flector = self_2521;
    let _e3: AntiScalar = flector_squared_anti_magnitude(_e2);
    return AntiScalar(sqrt(_e3.g0_));
}

fn flector_geometric_norm(self_2522: Flector) -> HomogeneousMagnitude {
    var self_2523: Flector;

    self_2523 = self_2522;
    let _e2: Flector = self_2523;
    let _e3: Scalar = flector_bulk_norm(_e2);
    let _e4: Flector = self_2523;
    let _e5: AntiScalar = flector_weight_norm(_e4);
    let _e6: HomogeneousMagnitude = scalar_anti_scalar_add(_e3, _e5);
    return _e6;
}

fn flector_scale(self_2524: Flector, other_2190: f32) -> Flector {
    var self_2525: Flector;
    var other_2191: f32;

    self_2525 = self_2524;
    other_2191 = other_2190;
    let _e4: Flector = self_2525;
    let _e5: f32 = other_2191;
    let _e7: Flector = flector_scalar_geometric_product(_e4, Scalar(_e5));
    return _e7;
}

fn flector_signum(self_2526: Flector) -> Flector {
    var self_2527: Flector;

    self_2527 = self_2526;
    let _e2: Flector = self_2527;
    let _e3: Flector = self_2527;
    let _e4: Scalar = flector_magnitude(_e3);
    let _e9: Flector = flector_scalar_geometric_product(_e2, Scalar((1.0 / _e4.g0_)));
    return _e9;
}

fn flector_inverse(self_2528: Flector) -> Flector {
    var self_2529: Flector;

    self_2529 = self_2528;
    let _e2: Flector = self_2529;
    let _e3: Flector = flector_reversal(_e2);
    let _e4: Flector = self_2529;
    let _e5: Scalar = flector_squared_magnitude(_e4);
    let _e10: Flector = flector_scalar_geometric_product(_e3, Scalar((1.0 / _e5.g0_)));
    return _e10;
}

fn flector_unitize(self_2530: Flector) -> Flector {
    var self_2531: Flector;

    self_2531 = self_2530;
    let _e2: Flector = self_2531;
    let _e3: Flector = self_2531;
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

fn multi_vector_neg(self_2532: MultiVector) -> MultiVector {
    var self_2533: MultiVector;

    self_2533 = self_2532;
    let _e2: MultiVector = self_2533;
    let _e9: MultiVector = self_2533;
    let _e15: MultiVector = self_2533;
    let _e21: MultiVector = self_2533;
    let _e27: MultiVector = self_2533;
    let _e33: MultiVector = self_2533;
    let _e39: MultiVector = self_2533;
    let _e45: MultiVector = self_2533;
    return MultiVector((_e2.g0_.xyy * vec3<f32>(-(1.0))), (_e9.g1_ * vec4<f32>(-(1.0))), (_e15.g2_ * vec3<f32>(-(1.0))), (_e21.g3_ * vec3<f32>(-(1.0))), (_e27.g2_ * vec3<f32>(-(1.0))), (_e33.g3_ * vec3<f32>(-(1.0))), (_e39.g6_ * vec4<f32>(-(1.0))), (_e45.g6_ * vec4<f32>(-(1.0))));
}

fn multi_vector_automorphism(self_2534: MultiVector) -> MultiVector {
    var self_2535: MultiVector;

    self_2535 = self_2534;
    let _e2: MultiVector = self_2535;
    let _e5: MultiVector = self_2535;
    let _e11: MultiVector = self_2535;
    let _e13: MultiVector = self_2535;
    let _e15: MultiVector = self_2535;
    let _e17: MultiVector = self_2535;
    let _e19: MultiVector = self_2535;
    let _e25: MultiVector = self_2535;
    return MultiVector(_e2.g0_.xyy, (_e5.g1_ * vec4<f32>(-(1.0))), _e11.g2_, _e13.g3_, _e15.g2_, _e17.g3_, (_e19.g6_ * vec4<f32>(-(1.0))), (_e25.g6_ * vec4<f32>(-(1.0))));
}

fn multi_vector_reversal(self_2536: MultiVector) -> MultiVector {
    var self_2537: MultiVector;

    self_2537 = self_2536;
    let _e2: MultiVector = self_2537;
    let _e5: MultiVector = self_2537;
    let _e7: MultiVector = self_2537;
    let _e13: MultiVector = self_2537;
    let _e19: MultiVector = self_2537;
    let _e25: MultiVector = self_2537;
    let _e31: MultiVector = self_2537;
    let _e37: MultiVector = self_2537;
    return MultiVector(_e2.g0_.xyy, _e5.g1_, (_e7.g2_ * vec3<f32>(-(1.0))), (_e13.g3_ * vec3<f32>(-(1.0))), (_e19.g2_ * vec3<f32>(-(1.0))), (_e25.g3_ * vec3<f32>(-(1.0))), (_e31.g6_ * vec4<f32>(-(1.0))), (_e37.g6_ * vec4<f32>(-(1.0))));
}

fn multi_vector_conjugation(self_2538: MultiVector) -> MultiVector {
    var self_2539: MultiVector;

    self_2539 = self_2538;
    let _e2: MultiVector = self_2539;
    let _e5: MultiVector = self_2539;
    let _e11: MultiVector = self_2539;
    let _e17: MultiVector = self_2539;
    let _e23: MultiVector = self_2539;
    let _e29: MultiVector = self_2539;
    let _e35: MultiVector = self_2539;
    let _e37: MultiVector = self_2539;
    return MultiVector(_e2.g0_.xyy, (_e5.g1_ * vec4<f32>(-(1.0))), (_e11.g2_ * vec3<f32>(-(1.0))), (_e17.g3_ * vec3<f32>(-(1.0))), (_e23.g2_ * vec3<f32>(-(1.0))), (_e29.g3_ * vec3<f32>(-(1.0))), _e35.g6_, _e37.g6_);
}

fn multi_vector_anti_reversal(self_2540: MultiVector) -> MultiVector {
    var self_2541: MultiVector;

    self_2541 = self_2540;
    let _e2: MultiVector = self_2541;
    let _e5: MultiVector = self_2541;
    let _e11: MultiVector = self_2541;
    let _e17: MultiVector = self_2541;
    let _e23: MultiVector = self_2541;
    let _e29: MultiVector = self_2541;
    let _e35: MultiVector = self_2541;
    let _e37: MultiVector = self_2541;
    return MultiVector(_e2.g0_.xyy, (_e5.g1_ * vec4<f32>(-(1.0))), (_e11.g2_ * vec3<f32>(-(1.0))), (_e17.g3_ * vec3<f32>(-(1.0))), (_e23.g2_ * vec3<f32>(-(1.0))), (_e29.g3_ * vec3<f32>(-(1.0))), _e35.g6_, _e37.g6_);
}

fn multi_vector_double_complement(self_2542: MultiVector) -> MultiVector {
    var self_2543: MultiVector;

    self_2543 = self_2542;
    let _e2: MultiVector = self_2543;
    let _e5: MultiVector = self_2543;
    let _e11: MultiVector = self_2543;
    let _e13: MultiVector = self_2543;
    let _e15: MultiVector = self_2543;
    let _e17: MultiVector = self_2543;
    let _e19: MultiVector = self_2543;
    let _e25: MultiVector = self_2543;
    return MultiVector(_e2.g0_.xyy, (_e5.g1_ * vec4<f32>(-(1.0))), _e11.g2_, _e13.g3_, _e15.g2_, _e17.g3_, (_e19.g6_ * vec4<f32>(-(1.0))), (_e25.g6_ * vec4<f32>(-(1.0))));
}

fn multi_vector_scalar_into(self_2544: MultiVector) -> Scalar {
    var self_2545: MultiVector;

    self_2545 = self_2544;
    let _e2: MultiVector = self_2545;
    return Scalar(_e2.g0_.x);
}

fn multi_vector_scalar_regressive_product(self_2546: MultiVector, other_2192: Scalar) -> Scalar {
    var self_2547: MultiVector;
    var other_2193: Scalar;

    self_2547 = self_2546;
    other_2193 = other_2192;
    let _e4: MultiVector = self_2547;
    let _e7: Scalar = other_2193;
    return Scalar((_e4.g0_.y * _e7.g0_));
}

fn multi_vector_scalar_anti_wedge(self_2548: MultiVector, other_2194: Scalar) -> Scalar {
    var self_2549: MultiVector;
    var other_2195: Scalar;

    self_2549 = self_2548;
    other_2195 = other_2194;
    let _e4: MultiVector = self_2549;
    let _e7: Scalar = other_2195;
    return Scalar((_e4.g0_.y * _e7.g0_));
}

fn multi_vector_scalar_meet(self_2550: MultiVector, other_2196: Scalar) -> Scalar {
    var self_2551: MultiVector;
    var other_2197: Scalar;

    self_2551 = self_2550;
    other_2197 = other_2196;
    let _e4: MultiVector = self_2551;
    let _e7: Scalar = other_2197;
    return Scalar((_e4.g0_.y * _e7.g0_));
}

fn multi_vector_scalar_left_contraction(self_2552: MultiVector, other_2198: Scalar) -> Scalar {
    var self_2553: MultiVector;
    var other_2199: Scalar;

    self_2553 = self_2552;
    other_2199 = other_2198;
    let _e4: MultiVector = self_2553;
    let _e7: Scalar = other_2199;
    return Scalar((_e4.g0_.x * _e7.g0_));
}

fn multi_vector_scalar_scalar_product(self_2554: MultiVector, other_2200: Scalar) -> Scalar {
    var self_2555: MultiVector;
    var other_2201: Scalar;

    self_2555 = self_2554;
    other_2201 = other_2200;
    let _e4: MultiVector = self_2555;
    let _e7: Scalar = other_2201;
    return Scalar((_e4.g0_.x * _e7.g0_));
}

fn multi_vector_scalar_dot(self_2556: MultiVector, other_2202: Scalar) -> Scalar {
    var self_2557: MultiVector;
    var other_2203: Scalar;

    self_2557 = self_2556;
    other_2203 = other_2202;
    let _e4: MultiVector = self_2557;
    let _e7: Scalar = other_2203;
    return Scalar((_e4.g0_.x * _e7.g0_));
}

fn multi_vector_anti_scalar_outer_product(self_2558: MultiVector, other_2204: AntiScalar) -> AntiScalar {
    var self_2559: MultiVector;
    var other_2205: AntiScalar;

    self_2559 = self_2558;
    other_2205 = other_2204;
    let _e4: MultiVector = self_2559;
    let _e7: AntiScalar = other_2205;
    return AntiScalar((_e4.g0_.x * _e7.g0_));
}

fn multi_vector_anti_scalar_wedge(self_2560: MultiVector, other_2206: AntiScalar) -> AntiScalar {
    var self_2561: MultiVector;
    var other_2207: AntiScalar;

    self_2561 = self_2560;
    other_2207 = other_2206;
    let _e4: MultiVector = self_2561;
    let _e7: AntiScalar = other_2207;
    return AntiScalar((_e4.g0_.x * _e7.g0_));
}

fn multi_vector_anti_scalar_join(self_2562: MultiVector, other_2208: AntiScalar) -> AntiScalar {
    var self_2563: MultiVector;
    var other_2209: AntiScalar;

    self_2563 = self_2562;
    other_2209 = other_2208;
    let _e4: MultiVector = self_2563;
    let _e7: AntiScalar = other_2209;
    return AntiScalar((_e4.g0_.x * _e7.g0_));
}

fn multi_vector_anti_scalar_left_anti_contraction(self_2564: MultiVector, other_2210: AntiScalar) -> AntiScalar {
    var self_2565: MultiVector;
    var other_2211: AntiScalar;

    self_2565 = self_2564;
    other_2211 = other_2210;
    let _e4: MultiVector = self_2565;
    let _e7: AntiScalar = other_2211;
    return AntiScalar((_e4.g0_.y * _e7.g0_));
}

fn multi_vector_anti_scalar_anti_scalar_product(self_2566: MultiVector, other_2212: AntiScalar) -> AntiScalar {
    var self_2567: MultiVector;
    var other_2213: AntiScalar;

    self_2567 = self_2566;
    other_2213 = other_2212;
    let _e4: MultiVector = self_2567;
    let _e7: AntiScalar = other_2213;
    return AntiScalar((_e4.g0_.y * _e7.g0_));
}

fn multi_vector_anti_scalar_anti_dot(self_2568: MultiVector, other_2214: AntiScalar) -> AntiScalar {
    var self_2569: MultiVector;
    var other_2215: AntiScalar;

    self_2569 = self_2568;
    other_2215 = other_2214;
    let _e4: MultiVector = self_2569;
    let _e7: AntiScalar = other_2215;
    return AntiScalar((_e4.g0_.y * _e7.g0_));
}

fn multi_vector_homogeneous_magnitude_scalar_product(self_2570: MultiVector, other_2216: HomogeneousMagnitude) -> Scalar {
    var self_2571: MultiVector;
    var other_2217: HomogeneousMagnitude;

    self_2571 = self_2570;
    other_2217 = other_2216;
    let _e4: MultiVector = self_2571;
    let _e7: HomogeneousMagnitude = other_2217;
    return Scalar((_e4.g0_.x * _e7.g0_.x));
}

fn multi_vector_homogeneous_magnitude_dot(self_2572: MultiVector, other_2218: HomogeneousMagnitude) -> Scalar {
    var self_2573: MultiVector;
    var other_2219: HomogeneousMagnitude;

    self_2573 = self_2572;
    other_2219 = other_2218;
    let _e4: MultiVector = self_2573;
    let _e7: HomogeneousMagnitude = other_2219;
    return Scalar((_e4.g0_.x * _e7.g0_.x));
}

fn multi_vector_homogeneous_magnitude_anti_scalar_product(self_2574: MultiVector, other_2220: HomogeneousMagnitude) -> AntiScalar {
    var self_2575: MultiVector;
    var other_2221: HomogeneousMagnitude;

    self_2575 = self_2574;
    other_2221 = other_2220;
    let _e4: MultiVector = self_2575;
    let _e7: HomogeneousMagnitude = other_2221;
    return AntiScalar((_e4.g0_.y * _e7.g0_.y));
}

fn multi_vector_homogeneous_magnitude_anti_dot(self_2576: MultiVector, other_2222: HomogeneousMagnitude) -> AntiScalar {
    var self_2577: MultiVector;
    var other_2223: HomogeneousMagnitude;

    self_2577 = self_2576;
    other_2223 = other_2222;
    let _e4: MultiVector = self_2577;
    let _e7: HomogeneousMagnitude = other_2223;
    return AntiScalar((_e4.g0_.y * _e7.g0_.y));
}

fn multi_vector_point_into(self_2578: MultiVector) -> Point {
    var self_2579: MultiVector;

    self_2579 = self_2578;
    let _e2: MultiVector = self_2579;
    return Point(_e2.g1_);
}

fn multi_vector_point_scalar_product(self_2580: MultiVector, other_2224: Point) -> Scalar {
    var self_2581: MultiVector;
    var other_2225: Point;

    self_2581 = self_2580;
    other_2225 = other_2224;
    let _e4: MultiVector = self_2581;
    let _e7: Point = other_2225;
    let _e11: MultiVector = self_2581;
    let _e14: Point = other_2225;
    let _e19: MultiVector = self_2581;
    let _e22: Point = other_2225;
    return Scalar((((_e4.g1_.x * _e7.g0_.x) + (_e11.g1_.y * _e14.g0_.y)) + (_e19.g1_.z * _e22.g0_.z)));
}

fn multi_vector_point_dot(self_2582: MultiVector, other_2226: Point) -> Scalar {
    var self_2583: MultiVector;
    var other_2227: Point;

    self_2583 = self_2582;
    other_2227 = other_2226;
    let _e4: MultiVector = self_2583;
    let _e7: Point = other_2227;
    let _e11: MultiVector = self_2583;
    let _e14: Point = other_2227;
    let _e19: MultiVector = self_2583;
    let _e22: Point = other_2227;
    return Scalar((((_e4.g1_.x * _e7.g0_.x) + (_e11.g1_.y * _e14.g0_.y)) + (_e19.g1_.z * _e22.g0_.z)));
}

fn multi_vector_point_anti_scalar_product(self_2584: MultiVector, other_2228: Point) -> AntiScalar {
    var self_2585: MultiVector;
    var other_2229: Point;

    self_2585 = self_2584;
    other_2229 = other_2228;
    let _e5: MultiVector = self_2585;
    let _e8: Point = other_2229;
    return AntiScalar((0.0 - (_e5.g1_.w * _e8.g0_.w)));
}

fn multi_vector_point_anti_dot(self_2586: MultiVector, other_2230: Point) -> AntiScalar {
    var self_2587: MultiVector;
    var other_2231: Point;

    self_2587 = self_2586;
    other_2231 = other_2230;
    let _e5: MultiVector = self_2587;
    let _e8: Point = other_2231;
    return AntiScalar((0.0 - (_e5.g1_.w * _e8.g0_.w)));
}

fn multi_vector_line_scalar_product(self_2588: MultiVector, other_2232: Line) -> Scalar {
    var self_2589: MultiVector;
    var other_2233: Line;

    self_2589 = self_2588;
    other_2233 = other_2232;
    let _e5: MultiVector = self_2589;
    let _e8: Line = other_2233;
    let _e13: MultiVector = self_2589;
    let _e16: Line = other_2233;
    let _e21: MultiVector = self_2589;
    let _e24: Line = other_2233;
    return Scalar((((0.0 - (_e5.g3_.x * _e8.g1_.x)) - (_e13.g3_.y * _e16.g1_.y)) - (_e21.g3_.z * _e24.g1_.z)));
}

fn multi_vector_line_dot(self_2590: MultiVector, other_2234: Line) -> Scalar {
    var self_2591: MultiVector;
    var other_2235: Line;

    self_2591 = self_2590;
    other_2235 = other_2234;
    let _e5: MultiVector = self_2591;
    let _e8: Line = other_2235;
    let _e13: MultiVector = self_2591;
    let _e16: Line = other_2235;
    let _e21: MultiVector = self_2591;
    let _e24: Line = other_2235;
    return Scalar((((0.0 - (_e5.g3_.x * _e8.g1_.x)) - (_e13.g3_.y * _e16.g1_.y)) - (_e21.g3_.z * _e24.g1_.z)));
}

fn multi_vector_line_anti_scalar_product(self_2592: MultiVector, other_2236: Line) -> AntiScalar {
    var self_2593: MultiVector;
    var other_2237: Line;

    self_2593 = self_2592;
    other_2237 = other_2236;
    let _e5: MultiVector = self_2593;
    let _e8: Line = other_2237;
    let _e13: MultiVector = self_2593;
    let _e16: Line = other_2237;
    let _e21: MultiVector = self_2593;
    let _e24: Line = other_2237;
    return AntiScalar((((0.0 - (_e5.g2_.x * _e8.g0_.x)) - (_e13.g2_.y * _e16.g0_.y)) - (_e21.g2_.z * _e24.g0_.z)));
}

fn multi_vector_line_anti_dot(self_2594: MultiVector, other_2238: Line) -> AntiScalar {
    var self_2595: MultiVector;
    var other_2239: Line;

    self_2595 = self_2594;
    other_2239 = other_2238;
    let _e5: MultiVector = self_2595;
    let _e8: Line = other_2239;
    let _e13: MultiVector = self_2595;
    let _e16: Line = other_2239;
    let _e21: MultiVector = self_2595;
    let _e24: Line = other_2239;
    return AntiScalar((((0.0 - (_e5.g2_.x * _e8.g0_.x)) - (_e13.g2_.y * _e16.g0_.y)) - (_e21.g2_.z * _e24.g0_.z)));
}

fn multi_vector_plane_scalar_product(self_2596: MultiVector, other_2240: Plane) -> Scalar {
    var self_2597: MultiVector;
    var other_2241: Plane;

    self_2597 = self_2596;
    other_2241 = other_2240;
    let _e5: MultiVector = self_2597;
    let _e8: Plane = other_2241;
    return Scalar((0.0 - (_e5.g6_.w * _e8.g0_.w)));
}

fn multi_vector_plane_dot(self_2598: MultiVector, other_2242: Plane) -> Scalar {
    var self_2599: MultiVector;
    var other_2243: Plane;

    self_2599 = self_2598;
    other_2243 = other_2242;
    let _e5: MultiVector = self_2599;
    let _e8: Plane = other_2243;
    return Scalar((0.0 - (_e5.g6_.w * _e8.g0_.w)));
}

fn multi_vector_plane_anti_scalar_product(self_2600: MultiVector, other_2244: Plane) -> AntiScalar {
    var self_2601: MultiVector;
    var other_2245: Plane;

    self_2601 = self_2600;
    other_2245 = other_2244;
    let _e4: MultiVector = self_2601;
    let _e7: Plane = other_2245;
    let _e11: MultiVector = self_2601;
    let _e14: Plane = other_2245;
    let _e19: MultiVector = self_2601;
    let _e22: Plane = other_2245;
    return AntiScalar((((_e4.g6_.x * _e7.g0_.x) + (_e11.g6_.y * _e14.g0_.y)) + (_e19.g6_.z * _e22.g0_.z)));
}

fn multi_vector_plane_anti_dot(self_2602: MultiVector, other_2246: Plane) -> AntiScalar {
    var self_2603: MultiVector;
    var other_2247: Plane;

    self_2603 = self_2602;
    other_2247 = other_2246;
    let _e4: MultiVector = self_2603;
    let _e7: Plane = other_2247;
    let _e11: MultiVector = self_2603;
    let _e14: Plane = other_2247;
    let _e19: MultiVector = self_2603;
    let _e22: Plane = other_2247;
    return AntiScalar((((_e4.g6_.x * _e7.g0_.x) + (_e11.g6_.y * _e14.g0_.y)) + (_e19.g6_.z * _e22.g0_.z)));
}

fn multi_vector_motor_scalar_product(self_2604: MultiVector, other_2248: Motor) -> Scalar {
    var self_2605: MultiVector;
    var other_2249: Motor;

    self_2605 = self_2604;
    other_2249 = other_2248;
    let _e5: MultiVector = self_2605;
    let _e8: Motor = other_2249;
    let _e13: MultiVector = self_2605;
    let _e16: Motor = other_2249;
    let _e21: MultiVector = self_2605;
    let _e24: Motor = other_2249;
    return Scalar((((0.0 - (_e5.g3_.x * _e8.g1_.x)) - (_e13.g3_.y * _e16.g1_.y)) - (_e21.g3_.z * _e24.g1_.z)));
}

fn multi_vector_motor_dot(self_2606: MultiVector, other_2250: Motor) -> Scalar {
    var self_2607: MultiVector;
    var other_2251: Motor;

    self_2607 = self_2606;
    other_2251 = other_2250;
    let _e5: MultiVector = self_2607;
    let _e8: Motor = other_2251;
    let _e13: MultiVector = self_2607;
    let _e16: Motor = other_2251;
    let _e21: MultiVector = self_2607;
    let _e24: Motor = other_2251;
    return Scalar((((0.0 - (_e5.g3_.x * _e8.g1_.x)) - (_e13.g3_.y * _e16.g1_.y)) - (_e21.g3_.z * _e24.g1_.z)));
}

fn multi_vector_motor_anti_scalar_product(self_2608: MultiVector, other_2252: Motor) -> AntiScalar {
    var self_2609: MultiVector;
    var other_2253: Motor;

    self_2609 = self_2608;
    other_2253 = other_2252;
    let _e4: MultiVector = self_2609;
    let _e7: Motor = other_2253;
    let _e11: MultiVector = self_2609;
    let _e14: Motor = other_2253;
    let _e19: MultiVector = self_2609;
    let _e22: Motor = other_2253;
    let _e27: MultiVector = self_2609;
    let _e30: Motor = other_2253;
    return AntiScalar(((((_e4.g0_.y * _e7.g0_.w) - (_e11.g2_.x * _e14.g0_.x)) - (_e19.g2_.y * _e22.g0_.y)) - (_e27.g2_.z * _e30.g0_.z)));
}

fn multi_vector_motor_anti_dot(self_2610: MultiVector, other_2254: Motor) -> AntiScalar {
    var self_2611: MultiVector;
    var other_2255: Motor;

    self_2611 = self_2610;
    other_2255 = other_2254;
    let _e4: MultiVector = self_2611;
    let _e7: Motor = other_2255;
    let _e11: MultiVector = self_2611;
    let _e14: Motor = other_2255;
    let _e19: MultiVector = self_2611;
    let _e22: Motor = other_2255;
    let _e27: MultiVector = self_2611;
    let _e30: Motor = other_2255;
    return AntiScalar(((((_e4.g0_.y * _e7.g0_.w) - (_e11.g2_.x * _e14.g0_.x)) - (_e19.g2_.y * _e22.g0_.y)) - (_e27.g2_.z * _e30.g0_.z)));
}

fn multi_vector_rotor_anti_scalar_product(self_2612: MultiVector, other_2256: Rotor) -> AntiScalar {
    var self_2613: MultiVector;
    var other_2257: Rotor;

    self_2613 = self_2612;
    other_2257 = other_2256;
    let _e4: MultiVector = self_2613;
    let _e7: Rotor = other_2257;
    let _e11: MultiVector = self_2613;
    let _e14: Rotor = other_2257;
    let _e19: MultiVector = self_2613;
    let _e22: Rotor = other_2257;
    let _e27: MultiVector = self_2613;
    let _e30: Rotor = other_2257;
    return AntiScalar(((((_e4.g0_.y * _e7.g0_.w) - (_e11.g2_.x * _e14.g0_.x)) - (_e19.g2_.y * _e22.g0_.y)) - (_e27.g2_.z * _e30.g0_.z)));
}

fn multi_vector_rotor_anti_dot(self_2614: MultiVector, other_2258: Rotor) -> AntiScalar {
    var self_2615: MultiVector;
    var other_2259: Rotor;

    self_2615 = self_2614;
    other_2259 = other_2258;
    let _e4: MultiVector = self_2615;
    let _e7: Rotor = other_2259;
    let _e11: MultiVector = self_2615;
    let _e14: Rotor = other_2259;
    let _e19: MultiVector = self_2615;
    let _e22: Rotor = other_2259;
    let _e27: MultiVector = self_2615;
    let _e30: Rotor = other_2259;
    return AntiScalar(((((_e4.g0_.y * _e7.g0_.w) - (_e11.g2_.x * _e14.g0_.x)) - (_e19.g2_.y * _e22.g0_.y)) - (_e27.g2_.z * _e30.g0_.z)));
}

fn multi_vector_translator_scalar_product(self_2616: MultiVector, other_2260: Translator) -> Scalar {
    var self_2617: MultiVector;
    var other_2261: Translator;

    self_2617 = self_2616;
    other_2261 = other_2260;
    let _e5: MultiVector = self_2617;
    let _e8: Translator = other_2261;
    let _e13: MultiVector = self_2617;
    let _e16: Translator = other_2261;
    let _e21: MultiVector = self_2617;
    let _e24: Translator = other_2261;
    return Scalar((((0.0 - (_e5.g3_.x * _e8.g0_.x)) - (_e13.g3_.y * _e16.g0_.y)) - (_e21.g3_.z * _e24.g0_.z)));
}

fn multi_vector_translator_dot(self_2618: MultiVector, other_2262: Translator) -> Scalar {
    var self_2619: MultiVector;
    var other_2263: Translator;

    self_2619 = self_2618;
    other_2263 = other_2262;
    let _e5: MultiVector = self_2619;
    let _e8: Translator = other_2263;
    let _e13: MultiVector = self_2619;
    let _e16: Translator = other_2263;
    let _e21: MultiVector = self_2619;
    let _e24: Translator = other_2263;
    return Scalar((((0.0 - (_e5.g3_.x * _e8.g0_.x)) - (_e13.g3_.y * _e16.g0_.y)) - (_e21.g3_.z * _e24.g0_.z)));
}

fn multi_vector_translator_anti_scalar_product(self_2620: MultiVector, other_2264: Translator) -> AntiScalar {
    var self_2621: MultiVector;
    var other_2265: Translator;

    self_2621 = self_2620;
    other_2265 = other_2264;
    let _e4: MultiVector = self_2621;
    let _e7: Translator = other_2265;
    return AntiScalar((_e4.g0_.y * _e7.g0_.w));
}

fn multi_vector_translator_anti_dot(self_2622: MultiVector, other_2266: Translator) -> AntiScalar {
    var self_2623: MultiVector;
    var other_2267: Translator;

    self_2623 = self_2622;
    other_2267 = other_2266;
    let _e4: MultiVector = self_2623;
    let _e7: Translator = other_2267;
    return AntiScalar((_e4.g0_.y * _e7.g0_.w));
}

fn multi_vector_flector_scalar_product(self_2624: MultiVector, other_2268: Flector) -> Scalar {
    var self_2625: MultiVector;
    var other_2269: Flector;

    self_2625 = self_2624;
    other_2269 = other_2268;
    let _e4: MultiVector = self_2625;
    let _e7: Flector = other_2269;
    let _e11: MultiVector = self_2625;
    let _e14: Flector = other_2269;
    let _e19: MultiVector = self_2625;
    let _e22: Flector = other_2269;
    let _e27: MultiVector = self_2625;
    let _e30: Flector = other_2269;
    return Scalar(((((_e4.g1_.x * _e7.g0_.x) + (_e11.g1_.y * _e14.g0_.y)) + (_e19.g1_.z * _e22.g0_.z)) - (_e27.g6_.w * _e30.g1_.w)));
}

fn multi_vector_flector_dot(self_2626: MultiVector, other_2270: Flector) -> Scalar {
    var self_2627: MultiVector;
    var other_2271: Flector;

    self_2627 = self_2626;
    other_2271 = other_2270;
    let _e4: MultiVector = self_2627;
    let _e7: Flector = other_2271;
    let _e11: MultiVector = self_2627;
    let _e14: Flector = other_2271;
    let _e19: MultiVector = self_2627;
    let _e22: Flector = other_2271;
    let _e27: MultiVector = self_2627;
    let _e30: Flector = other_2271;
    return Scalar(((((_e4.g1_.x * _e7.g0_.x) + (_e11.g1_.y * _e14.g0_.y)) + (_e19.g1_.z * _e22.g0_.z)) - (_e27.g6_.w * _e30.g1_.w)));
}

fn multi_vector_flector_anti_scalar_product(self_2628: MultiVector, other_2272: Flector) -> AntiScalar {
    var self_2629: MultiVector;
    var other_2273: Flector;

    self_2629 = self_2628;
    other_2273 = other_2272;
    let _e5: MultiVector = self_2629;
    let _e8: Flector = other_2273;
    let _e13: MultiVector = self_2629;
    let _e16: Flector = other_2273;
    let _e21: MultiVector = self_2629;
    let _e24: Flector = other_2273;
    let _e29: MultiVector = self_2629;
    let _e32: Flector = other_2273;
    return AntiScalar(((((0.0 - (_e5.g1_.w * _e8.g0_.w)) + (_e13.g6_.x * _e16.g1_.x)) + (_e21.g6_.y * _e24.g1_.y)) + (_e29.g6_.z * _e32.g1_.z)));
}

fn multi_vector_flector_anti_dot(self_2630: MultiVector, other_2274: Flector) -> AntiScalar {
    var self_2631: MultiVector;
    var other_2275: Flector;

    self_2631 = self_2630;
    other_2275 = other_2274;
    let _e5: MultiVector = self_2631;
    let _e8: Flector = other_2275;
    let _e13: MultiVector = self_2631;
    let _e16: Flector = other_2275;
    let _e21: MultiVector = self_2631;
    let _e24: Flector = other_2275;
    let _e29: MultiVector = self_2631;
    let _e32: Flector = other_2275;
    return AntiScalar(((((0.0 - (_e5.g1_.w * _e8.g0_.w)) + (_e13.g6_.x * _e16.g1_.x)) + (_e21.g6_.y * _e24.g1_.y)) + (_e29.g6_.z * _e32.g1_.z)));
}

fn multi_vector_multi_vector_scalar_product(self_2632: MultiVector, other_2276: MultiVector) -> Scalar {
    var self_2633: MultiVector;
    var other_2277: MultiVector;

    self_2633 = self_2632;
    other_2277 = other_2276;
    let _e4: MultiVector = self_2633;
    let _e7: MultiVector = other_2277;
    let _e11: MultiVector = self_2633;
    let _e14: MultiVector = other_2277;
    let _e19: MultiVector = self_2633;
    let _e22: MultiVector = other_2277;
    let _e27: MultiVector = self_2633;
    let _e30: MultiVector = other_2277;
    let _e35: MultiVector = self_2633;
    let _e38: MultiVector = other_2277;
    let _e43: MultiVector = self_2633;
    let _e46: MultiVector = other_2277;
    let _e51: MultiVector = self_2633;
    let _e54: MultiVector = other_2277;
    let _e59: MultiVector = self_2633;
    let _e62: MultiVector = other_2277;
    return Scalar(((((((((_e4.g0_.x * _e7.g0_.x) + (_e11.g1_.x * _e14.g1_.x)) + (_e19.g1_.y * _e22.g1_.y)) + (_e27.g1_.z * _e30.g1_.z)) - (_e35.g3_.x * _e38.g3_.x)) - (_e43.g3_.y * _e46.g3_.y)) - (_e51.g3_.z * _e54.g3_.z)) - (_e59.g6_.w * _e62.g6_.w)));
}

fn multi_vector_multi_vector_dot(self_2634: MultiVector, other_2278: MultiVector) -> Scalar {
    var self_2635: MultiVector;
    var other_2279: MultiVector;

    self_2635 = self_2634;
    other_2279 = other_2278;
    let _e4: MultiVector = self_2635;
    let _e7: MultiVector = other_2279;
    let _e11: MultiVector = self_2635;
    let _e14: MultiVector = other_2279;
    let _e19: MultiVector = self_2635;
    let _e22: MultiVector = other_2279;
    let _e27: MultiVector = self_2635;
    let _e30: MultiVector = other_2279;
    let _e35: MultiVector = self_2635;
    let _e38: MultiVector = other_2279;
    let _e43: MultiVector = self_2635;
    let _e46: MultiVector = other_2279;
    let _e51: MultiVector = self_2635;
    let _e54: MultiVector = other_2279;
    let _e59: MultiVector = self_2635;
    let _e62: MultiVector = other_2279;
    return Scalar(((((((((_e4.g0_.x * _e7.g0_.x) + (_e11.g1_.x * _e14.g1_.x)) + (_e19.g1_.y * _e22.g1_.y)) + (_e27.g1_.z * _e30.g1_.z)) - (_e35.g3_.x * _e38.g3_.x)) - (_e43.g3_.y * _e46.g3_.y)) - (_e51.g3_.z * _e54.g3_.z)) - (_e59.g6_.w * _e62.g6_.w)));
}

fn multi_vector_multi_vector_anti_scalar_product(self_2636: MultiVector, other_2280: MultiVector) -> AntiScalar {
    var self_2637: MultiVector;
    var other_2281: MultiVector;

    self_2637 = self_2636;
    other_2281 = other_2280;
    let _e4: MultiVector = self_2637;
    let _e7: MultiVector = other_2281;
    let _e11: MultiVector = self_2637;
    let _e14: MultiVector = other_2281;
    let _e19: MultiVector = self_2637;
    let _e22: MultiVector = other_2281;
    let _e27: MultiVector = self_2637;
    let _e30: MultiVector = other_2281;
    let _e35: MultiVector = self_2637;
    let _e38: MultiVector = other_2281;
    let _e43: MultiVector = self_2637;
    let _e46: MultiVector = other_2281;
    let _e51: MultiVector = self_2637;
    let _e54: MultiVector = other_2281;
    let _e59: MultiVector = self_2637;
    let _e62: MultiVector = other_2281;
    return AntiScalar(((((((((_e4.g0_.y * _e7.g0_.y) - (_e11.g1_.w * _e14.g1_.w)) - (_e19.g2_.x * _e22.g2_.x)) - (_e27.g2_.y * _e30.g2_.y)) - (_e35.g2_.z * _e38.g2_.z)) + (_e43.g6_.x * _e46.g6_.x)) + (_e51.g6_.y * _e54.g6_.y)) + (_e59.g6_.z * _e62.g6_.z)));
}

fn multi_vector_multi_vector_anti_dot(self_2638: MultiVector, other_2282: MultiVector) -> AntiScalar {
    var self_2639: MultiVector;
    var other_2283: MultiVector;

    self_2639 = self_2638;
    other_2283 = other_2282;
    let _e4: MultiVector = self_2639;
    let _e7: MultiVector = other_2283;
    let _e11: MultiVector = self_2639;
    let _e14: MultiVector = other_2283;
    let _e19: MultiVector = self_2639;
    let _e22: MultiVector = other_2283;
    let _e27: MultiVector = self_2639;
    let _e30: MultiVector = other_2283;
    let _e35: MultiVector = self_2639;
    let _e38: MultiVector = other_2283;
    let _e43: MultiVector = self_2639;
    let _e46: MultiVector = other_2283;
    let _e51: MultiVector = self_2639;
    let _e54: MultiVector = other_2283;
    let _e59: MultiVector = self_2639;
    let _e62: MultiVector = other_2283;
    return AntiScalar(((((((((_e4.g0_.y * _e7.g0_.y) - (_e11.g1_.w * _e14.g1_.w)) - (_e19.g2_.x * _e22.g2_.x)) - (_e27.g2_.y * _e30.g2_.y)) - (_e35.g2_.z * _e38.g2_.z)) + (_e43.g6_.x * _e46.g6_.x)) + (_e51.g6_.y * _e54.g6_.y)) + (_e59.g6_.z * _e62.g6_.z)));
}

fn multi_vector_squared_magnitude(self_2640: MultiVector) -> Scalar {
    var self_2641: MultiVector;

    self_2641 = self_2640;
    let _e2: MultiVector = self_2641;
    let _e3: MultiVector = self_2641;
    let _e4: MultiVector = multi_vector_reversal(_e3);
    let _e5: Scalar = multi_vector_multi_vector_scalar_product(_e2, _e4);
    return _e5;
}

fn multi_vector_magnitude(self_2642: MultiVector) -> Scalar {
    var self_2643: MultiVector;

    self_2643 = self_2642;
    let _e2: MultiVector = self_2643;
    let _e3: Scalar = multi_vector_squared_magnitude(_e2);
    return Scalar(sqrt(_e3.g0_));
}

fn multi_vector_bulk_norm(self_2644: MultiVector) -> Scalar {
    var self_2645: MultiVector;

    self_2645 = self_2644;
    let _e2: MultiVector = self_2645;
    let _e3: Scalar = multi_vector_squared_magnitude(_e2);
    return Scalar(sqrt(_e3.g0_));
}

fn multi_vector_squared_anti_magnitude(self_2646: MultiVector) -> AntiScalar {
    var self_2647: MultiVector;

    self_2647 = self_2646;
    let _e2: MultiVector = self_2647;
    let _e3: MultiVector = self_2647;
    let _e4: MultiVector = multi_vector_anti_reversal(_e3);
    let _e5: AntiScalar = multi_vector_multi_vector_anti_scalar_product(_e2, _e4);
    return _e5;
}

fn multi_vector_weight_norm(self_2648: MultiVector) -> AntiScalar {
    var self_2649: MultiVector;

    self_2649 = self_2648;
    let _e2: MultiVector = self_2649;
    let _e3: AntiScalar = multi_vector_squared_anti_magnitude(_e2);
    return AntiScalar(sqrt(_e3.g0_));
}

fn multi_vector_geometric_norm(self_2650: MultiVector) -> HomogeneousMagnitude {
    var self_2651: MultiVector;

    self_2651 = self_2650;
    let _e2: MultiVector = self_2651;
    let _e3: Scalar = multi_vector_bulk_norm(_e2);
    let _e4: MultiVector = self_2651;
    let _e5: AntiScalar = multi_vector_weight_norm(_e4);
    let _e6: HomogeneousMagnitude = scalar_anti_scalar_add(_e3, _e5);
    return _e6;
}

fn anti_scalar_homogeneous_magnitude_geometric_quotient(self_2652: AntiScalar, other_2284: HomogeneousMagnitude) -> AntiScalar {
    var self_2653: AntiScalar;
    var other_2285: HomogeneousMagnitude;

    self_2653 = self_2652;
    other_2285 = other_2284;
    let _e4: AntiScalar = self_2653;
    let _e5: HomogeneousMagnitude = other_2285;
    let _e6: HomogeneousMagnitude = homogeneous_magnitude_inverse(_e5);
    let _e7: AntiScalar = anti_scalar_homogeneous_magnitude_geometric_product(_e4, _e6);
    return _e7;
}

fn anti_scalar_scalar_geometric_quotient(self_2654: AntiScalar, other_2286: Scalar) -> AntiScalar {
    var self_2655: AntiScalar;
    var other_2287: Scalar;

    self_2655 = self_2654;
    other_2287 = other_2286;
    let _e4: AntiScalar = self_2655;
    let _e5: Scalar = other_2287;
    let _e6: Scalar = scalar_inverse(_e5);
    let _e7: AntiScalar = anti_scalar_scalar_geometric_product(_e4, _e6);
    return _e7;
}

fn flector_homogeneous_magnitude_geometric_quotient(self_2656: Flector, other_2288: HomogeneousMagnitude) -> Flector {
    var self_2657: Flector;
    var other_2289: HomogeneousMagnitude;

    self_2657 = self_2656;
    other_2289 = other_2288;
    let _e4: Flector = self_2657;
    let _e5: HomogeneousMagnitude = other_2289;
    let _e6: HomogeneousMagnitude = homogeneous_magnitude_inverse(_e5);
    let _e7: Flector = flector_homogeneous_magnitude_geometric_product(_e4, _e6);
    return _e7;
}

fn flector_line_geometric_quotient(self_2658: Flector, other_2290: Line) -> Flector {
    var self_2659: Flector;
    var other_2291: Line;

    self_2659 = self_2658;
    other_2291 = other_2290;
    let _e4: Flector = self_2659;
    let _e5: Line = other_2291;
    let _e6: Line = line_inverse(_e5);
    let _e7: Flector = flector_line_geometric_product(_e4, _e6);
    return _e7;
}

fn flector_motor_geometric_quotient(self_2660: Flector, other_2292: Motor) -> Flector {
    var self_2661: Flector;
    var other_2293: Motor;

    self_2661 = self_2660;
    other_2293 = other_2292;
    let _e4: Flector = self_2661;
    let _e5: Motor = other_2293;
    let _e6: Motor = motor_inverse(_e5);
    let _e7: Flector = flector_motor_geometric_product(_e4, _e6);
    return _e7;
}

fn flector_scalar_geometric_quotient(self_2662: Flector, other_2294: Scalar) -> Flector {
    var self_2663: Flector;
    var other_2295: Scalar;

    self_2663 = self_2662;
    other_2295 = other_2294;
    let _e4: Flector = self_2663;
    let _e5: Scalar = other_2295;
    let _e6: Scalar = scalar_inverse(_e5);
    let _e7: Flector = flector_scalar_geometric_product(_e4, _e6);
    return _e7;
}

fn flector_translator_geometric_quotient(self_2664: Flector, other_2296: Translator) -> Flector {
    var self_2665: Flector;
    var other_2297: Translator;

    self_2665 = self_2664;
    other_2297 = other_2296;
    let _e4: Flector = self_2665;
    let _e5: Translator = other_2297;
    let _e6: Translator = translator_inverse(_e5);
    let _e7: Flector = flector_translator_geometric_product(_e4, _e6);
    return _e7;
}

fn homogeneous_magnitude_anti_scalar_transformation(self_2666: HomogeneousMagnitude, other_2298: AntiScalar) -> AntiScalar {
    var self_2667: HomogeneousMagnitude;
    var other_2299: AntiScalar;

    self_2667 = self_2666;
    other_2299 = other_2298;
    let _e4: HomogeneousMagnitude = self_2667;
    let _e5: AntiScalar = other_2299;
    let _e6: AntiScalar = homogeneous_magnitude_anti_scalar_geometric_product(_e4, _e5);
    let _e7: HomogeneousMagnitude = self_2667;
    let _e8: HomogeneousMagnitude = homogeneous_magnitude_reversal(_e7);
    let _e9: AntiScalar = anti_scalar_homogeneous_magnitude_geometric_product(_e6, _e8);
    return _e9;
}

fn homogeneous_magnitude_flector_geometric_quotient(self_2668: HomogeneousMagnitude, other_2300: Flector) -> Flector {
    var self_2669: HomogeneousMagnitude;
    var other_2301: Flector;

    self_2669 = self_2668;
    other_2301 = other_2300;
    let _e4: HomogeneousMagnitude = self_2669;
    let _e5: Flector = other_2301;
    let _e6: Flector = flector_inverse(_e5);
    let _e7: Flector = homogeneous_magnitude_flector_geometric_product(_e4, _e6);
    return _e7;
}

fn homogeneous_magnitude_flector_transformation(self_2670: HomogeneousMagnitude, other_2302: Flector) -> Flector {
    var self_2671: HomogeneousMagnitude;
    var other_2303: Flector;

    self_2671 = self_2670;
    other_2303 = other_2302;
    let _e4: HomogeneousMagnitude = self_2671;
    let _e5: Flector = other_2303;
    let _e6: Flector = homogeneous_magnitude_flector_geometric_product(_e4, _e5);
    let _e7: HomogeneousMagnitude = self_2671;
    let _e8: HomogeneousMagnitude = homogeneous_magnitude_reversal(_e7);
    let _e9: Flector = flector_homogeneous_magnitude_geometric_product(_e6, _e8);
    return _e9;
}

fn homogeneous_magnitude_powi(self_2672: HomogeneousMagnitude, exponent: i32) -> HomogeneousMagnitude {
    var self_2673: HomogeneousMagnitude;
    var exponent_1: i32;
    var local: HomogeneousMagnitude;
    var x: HomogeneousMagnitude;
    var y: HomogeneousMagnitude;
    var n: i32;

    self_2673 = self_2672;
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
        let _e11: HomogeneousMagnitude = self_2673;
        let _e12: HomogeneousMagnitude = homogeneous_magnitude_inverse(_e11);
        local = _e12;
    } else {
        let _e14: HomogeneousMagnitude = self_2673;
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

fn homogeneous_magnitude_homogeneous_magnitude_geometric_quotient(self_2674: HomogeneousMagnitude, other_2304: HomogeneousMagnitude) -> HomogeneousMagnitude {
    var self_2675: HomogeneousMagnitude;
    var other_2305: HomogeneousMagnitude;

    self_2675 = self_2674;
    other_2305 = other_2304;
    let _e4: HomogeneousMagnitude = self_2675;
    let _e5: HomogeneousMagnitude = other_2305;
    let _e6: HomogeneousMagnitude = homogeneous_magnitude_inverse(_e5);
    let _e7: HomogeneousMagnitude = homogeneous_magnitude_homogeneous_magnitude_geometric_product(_e4, _e6);
    return _e7;
}

fn homogeneous_magnitude_homogeneous_magnitude_transformation(self_2676: HomogeneousMagnitude, other_2306: HomogeneousMagnitude) -> HomogeneousMagnitude {
    var self_2677: HomogeneousMagnitude;
    var other_2307: HomogeneousMagnitude;

    self_2677 = self_2676;
    other_2307 = other_2306;
    let _e4: HomogeneousMagnitude = self_2677;
    let _e5: HomogeneousMagnitude = other_2307;
    let _e6: HomogeneousMagnitude = homogeneous_magnitude_homogeneous_magnitude_geometric_product(_e4, _e5);
    let _e7: HomogeneousMagnitude = self_2677;
    let _e8: HomogeneousMagnitude = homogeneous_magnitude_reversal(_e7);
    let _e9: HomogeneousMagnitude = homogeneous_magnitude_homogeneous_magnitude_geometric_product(_e6, _e8);
    return _e9;
}

fn homogeneous_magnitude_line_geometric_quotient(self_2678: HomogeneousMagnitude, other_2308: Line) -> Line {
    var self_2679: HomogeneousMagnitude;
    var other_2309: Line;

    self_2679 = self_2678;
    other_2309 = other_2308;
    let _e4: HomogeneousMagnitude = self_2679;
    let _e5: Line = other_2309;
    let _e6: Line = line_inverse(_e5);
    let _e7: Line = homogeneous_magnitude_line_geometric_product(_e4, _e6);
    return _e7;
}

fn homogeneous_magnitude_line_transformation(self_2680: HomogeneousMagnitude, other_2310: Line) -> Line {
    var self_2681: HomogeneousMagnitude;
    var other_2311: Line;

    self_2681 = self_2680;
    other_2311 = other_2310;
    let _e4: HomogeneousMagnitude = self_2681;
    let _e5: Line = other_2311;
    let _e6: Line = homogeneous_magnitude_line_geometric_product(_e4, _e5);
    let _e7: HomogeneousMagnitude = self_2681;
    let _e8: HomogeneousMagnitude = homogeneous_magnitude_reversal(_e7);
    let _e9: Line = line_homogeneous_magnitude_geometric_product(_e6, _e8);
    return _e9;
}

fn homogeneous_magnitude_motor_geometric_quotient(self_2682: HomogeneousMagnitude, other_2312: Motor) -> Motor {
    var self_2683: HomogeneousMagnitude;
    var other_2313: Motor;

    self_2683 = self_2682;
    other_2313 = other_2312;
    let _e4: HomogeneousMagnitude = self_2683;
    let _e5: Motor = other_2313;
    let _e6: Motor = motor_inverse(_e5);
    let _e7: Motor = homogeneous_magnitude_motor_geometric_product(_e4, _e6);
    return _e7;
}

fn homogeneous_magnitude_motor_transformation(self_2684: HomogeneousMagnitude, other_2314: Motor) -> Motor {
    var self_2685: HomogeneousMagnitude;
    var other_2315: Motor;

    self_2685 = self_2684;
    other_2315 = other_2314;
    let _e4: HomogeneousMagnitude = self_2685;
    let _e5: Motor = other_2315;
    let _e6: Motor = homogeneous_magnitude_motor_geometric_product(_e4, _e5);
    let _e7: HomogeneousMagnitude = self_2685;
    let _e8: HomogeneousMagnitude = homogeneous_magnitude_reversal(_e7);
    let _e9: Motor = motor_homogeneous_magnitude_geometric_product(_e6, _e8);
    return _e9;
}

fn homogeneous_magnitude_rotor_transformation(self_2686: HomogeneousMagnitude, other_2316: Rotor) -> Rotor {
    var self_2687: HomogeneousMagnitude;
    var other_2317: Rotor;

    self_2687 = self_2686;
    other_2317 = other_2316;
    let _e4: HomogeneousMagnitude = self_2687;
    let _e5: Rotor = other_2317;
    let _e6: Rotor = homogeneous_magnitude_rotor_geometric_product(_e4, _e5);
    let _e7: HomogeneousMagnitude = self_2687;
    let _e8: HomogeneousMagnitude = homogeneous_magnitude_reversal(_e7);
    let _e9: Rotor = rotor_homogeneous_magnitude_geometric_product(_e6, _e8);
    return _e9;
}

fn homogeneous_magnitude_scalar_geometric_quotient(self_2688: HomogeneousMagnitude, other_2318: Scalar) -> HomogeneousMagnitude {
    var self_2689: HomogeneousMagnitude;
    var other_2319: Scalar;

    self_2689 = self_2688;
    other_2319 = other_2318;
    let _e4: HomogeneousMagnitude = self_2689;
    let _e5: Scalar = other_2319;
    let _e6: Scalar = scalar_inverse(_e5);
    let _e7: HomogeneousMagnitude = homogeneous_magnitude_scalar_geometric_product(_e4, _e6);
    return _e7;
}

fn homogeneous_magnitude_scalar_transformation(self_2690: HomogeneousMagnitude, other_2320: Scalar) -> Scalar {
    var self_2691: HomogeneousMagnitude;
    var other_2321: Scalar;

    self_2691 = self_2690;
    other_2321 = other_2320;
    let _e4: HomogeneousMagnitude = self_2691;
    let _e5: Scalar = other_2321;
    let _e6: HomogeneousMagnitude = homogeneous_magnitude_scalar_geometric_product(_e4, _e5);
    let _e7: HomogeneousMagnitude = self_2691;
    let _e8: HomogeneousMagnitude = homogeneous_magnitude_reversal(_e7);
    let _e9: HomogeneousMagnitude = homogeneous_magnitude_homogeneous_magnitude_geometric_product(_e6, _e8);
    let _e10: Scalar = homogeneous_magnitude_scalar_into(_e9);
    return _e10;
}

fn homogeneous_magnitude_translator_geometric_quotient(self_2692: HomogeneousMagnitude, other_2322: Translator) -> Motor {
    var self_2693: HomogeneousMagnitude;
    var other_2323: Translator;

    self_2693 = self_2692;
    other_2323 = other_2322;
    let _e4: HomogeneousMagnitude = self_2693;
    let _e5: Translator = other_2323;
    let _e6: Translator = translator_inverse(_e5);
    let _e7: Motor = homogeneous_magnitude_translator_geometric_product(_e4, _e6);
    return _e7;
}

fn homogeneous_magnitude_translator_transformation(self_2694: HomogeneousMagnitude, other_2324: Translator) -> Translator {
    var self_2695: HomogeneousMagnitude;
    var other_2325: Translator;

    self_2695 = self_2694;
    other_2325 = other_2324;
    let _e4: HomogeneousMagnitude = self_2695;
    let _e5: Translator = other_2325;
    let _e6: Motor = homogeneous_magnitude_translator_geometric_product(_e4, _e5);
    let _e7: HomogeneousMagnitude = self_2695;
    let _e8: HomogeneousMagnitude = homogeneous_magnitude_reversal(_e7);
    let _e9: Motor = motor_homogeneous_magnitude_geometric_product(_e6, _e8);
    let _e10: Translator = motor_translator_into(_e9);
    return _e10;
}

fn line_flector_geometric_quotient(self_2696: Line, other_2326: Flector) -> Flector {
    var self_2697: Line;
    var other_2327: Flector;

    self_2697 = self_2696;
    other_2327 = other_2326;
    let _e4: Line = self_2697;
    let _e5: Flector = other_2327;
    let _e6: Flector = flector_inverse(_e5);
    let _e7: Flector = line_flector_geometric_product(_e4, _e6);
    return _e7;
}

fn line_flector_transformation(self_2698: Line, other_2328: Flector) -> Flector {
    var self_2699: Line;
    var other_2329: Flector;

    self_2699 = self_2698;
    other_2329 = other_2328;
    let _e4: Line = self_2699;
    let _e5: Flector = other_2329;
    let _e6: Flector = line_flector_geometric_product(_e4, _e5);
    let _e7: Line = self_2699;
    let _e8: Line = line_reversal(_e7);
    let _e9: Flector = flector_line_geometric_product(_e6, _e8);
    return _e9;
}

fn line_homogeneous_magnitude_geometric_quotient(self_2700: Line, other_2330: HomogeneousMagnitude) -> Line {
    var self_2701: Line;
    var other_2331: HomogeneousMagnitude;

    self_2701 = self_2700;
    other_2331 = other_2330;
    let _e4: Line = self_2701;
    let _e5: HomogeneousMagnitude = other_2331;
    let _e6: HomogeneousMagnitude = homogeneous_magnitude_inverse(_e5);
    let _e7: Line = line_homogeneous_magnitude_geometric_product(_e4, _e6);
    return _e7;
}

fn line_point_geometric_quotient(self_2702: Line, other_2332: Point) -> Flector {
    var self_2703: Line;
    var other_2333: Point;

    self_2703 = self_2702;
    other_2333 = other_2332;
    let _e4: Line = self_2703;
    let _e5: Point = other_2333;
    let _e6: Point = point_inverse(_e5);
    let _e7: Flector = line_point_geometric_product(_e4, _e6);
    return _e7;
}

fn line_point_transformation(self_2704: Line, other_2334: Point) -> Point {
    var self_2705: Line;
    var other_2335: Point;

    self_2705 = self_2704;
    other_2335 = other_2334;
    let _e4: Line = self_2705;
    let _e5: Point = other_2335;
    let _e6: Flector = line_point_geometric_product(_e4, _e5);
    let _e7: Line = self_2705;
    let _e8: Line = line_reversal(_e7);
    let _e9: Flector = flector_line_geometric_product(_e6, _e8);
    let _e10: Point = flector_point_into(_e9);
    return _e10;
}

fn line_rotor_transformation(self_2706: Line, other_2336: Rotor) -> Rotor {
    var self_2707: Line;
    var other_2337: Rotor;

    self_2707 = self_2706;
    other_2337 = other_2336;
    let _e4: Line = self_2707;
    let _e5: Rotor = other_2337;
    let _e6: Rotor = line_rotor_geometric_product(_e4, _e5);
    let _e7: Line = self_2707;
    let _e8: Line = line_reversal(_e7);
    let _e9: Rotor = rotor_line_geometric_product(_e6, _e8);
    return _e9;
}

fn line_scalar_geometric_quotient(self_2708: Line, other_2338: Scalar) -> Line {
    var self_2709: Line;
    var other_2339: Scalar;

    self_2709 = self_2708;
    other_2339 = other_2338;
    let _e4: Line = self_2709;
    let _e5: Scalar = other_2339;
    let _e6: Scalar = scalar_inverse(_e5);
    let _e7: Line = line_scalar_geometric_product(_e4, _e6);
    return _e7;
}

fn motor_flector_geometric_quotient(self_2710: Motor, other_2340: Flector) -> Flector {
    var self_2711: Motor;
    var other_2341: Flector;

    self_2711 = self_2710;
    other_2341 = other_2340;
    let _e4: Motor = self_2711;
    let _e5: Flector = other_2341;
    let _e6: Flector = flector_inverse(_e5);
    let _e7: Flector = motor_flector_geometric_product(_e4, _e6);
    return _e7;
}

fn motor_flector_transformation(self_2712: Motor, other_2342: Flector) -> Flector {
    var self_2713: Motor;
    var other_2343: Flector;

    self_2713 = self_2712;
    other_2343 = other_2342;
    let _e4: Motor = self_2713;
    let _e5: Flector = other_2343;
    let _e6: Flector = motor_flector_geometric_product(_e4, _e5);
    let _e7: Motor = self_2713;
    let _e8: Motor = motor_reversal(_e7);
    let _e9: Flector = flector_motor_geometric_product(_e6, _e8);
    return _e9;
}

fn motor_homogeneous_magnitude_geometric_quotient(self_2714: Motor, other_2344: HomogeneousMagnitude) -> Motor {
    var self_2715: Motor;
    var other_2345: HomogeneousMagnitude;

    self_2715 = self_2714;
    other_2345 = other_2344;
    let _e4: Motor = self_2715;
    let _e5: HomogeneousMagnitude = other_2345;
    let _e6: HomogeneousMagnitude = homogeneous_magnitude_inverse(_e5);
    let _e7: Motor = motor_homogeneous_magnitude_geometric_product(_e4, _e6);
    return _e7;
}

fn motor_point_geometric_quotient(self_2716: Motor, other_2346: Point) -> Flector {
    var self_2717: Motor;
    var other_2347: Point;

    self_2717 = self_2716;
    other_2347 = other_2346;
    let _e4: Motor = self_2717;
    let _e5: Point = other_2347;
    let _e6: Point = point_inverse(_e5);
    let _e7: Flector = motor_point_geometric_product(_e4, _e6);
    return _e7;
}

fn motor_point_transformation(self_2718: Motor, other_2348: Point) -> Point {
    var self_2719: Motor;
    var other_2349: Point;

    self_2719 = self_2718;
    other_2349 = other_2348;
    let _e4: Motor = self_2719;
    let _e5: Point = other_2349;
    let _e6: Flector = motor_point_geometric_product(_e4, _e5);
    let _e7: Motor = self_2719;
    let _e8: Motor = motor_reversal(_e7);
    let _e9: Flector = flector_motor_geometric_product(_e6, _e8);
    let _e10: Point = flector_point_into(_e9);
    return _e10;
}

fn motor_rotor_transformation(self_2720: Motor, other_2350: Rotor) -> Rotor {
    var self_2721: Motor;
    var other_2351: Rotor;

    self_2721 = self_2720;
    other_2351 = other_2350;
    let _e4: Motor = self_2721;
    let _e5: Rotor = other_2351;
    let _e6: Rotor = motor_rotor_geometric_product(_e4, _e5);
    let _e7: Motor = self_2721;
    let _e8: Motor = motor_reversal(_e7);
    let _e9: Rotor = rotor_motor_geometric_product(_e6, _e8);
    return _e9;
}

fn motor_scalar_geometric_quotient(self_2722: Motor, other_2352: Scalar) -> Motor {
    var self_2723: Motor;
    var other_2353: Scalar;

    self_2723 = self_2722;
    other_2353 = other_2352;
    let _e4: Motor = self_2723;
    let _e5: Scalar = other_2353;
    let _e6: Scalar = scalar_inverse(_e5);
    let _e7: Motor = motor_scalar_geometric_product(_e4, _e6);
    return _e7;
}

fn plane_point_geometric_quotient(self_2724: Plane, other_2354: Point) -> Motor {
    var self_2725: Plane;
    var other_2355: Point;

    self_2725 = self_2724;
    other_2355 = other_2354;
    let _e4: Plane = self_2725;
    let _e5: Point = other_2355;
    let _e6: Point = point_inverse(_e5);
    let _e7: Motor = plane_point_geometric_product(_e4, _e6);
    return _e7;
}

fn plane_scalar_geometric_quotient(self_2726: Plane, other_2356: Scalar) -> Plane {
    var self_2727: Plane;
    var other_2357: Scalar;

    self_2727 = self_2726;
    other_2357 = other_2356;
    let _e4: Plane = self_2727;
    let _e5: Scalar = other_2357;
    let _e6: Scalar = scalar_inverse(_e5);
    let _e7: Plane = plane_scalar_geometric_product(_e4, _e6);
    return _e7;
}

fn point_line_geometric_quotient(self_2728: Point, other_2358: Line) -> Flector {
    var self_2729: Point;
    var other_2359: Line;

    self_2729 = self_2728;
    other_2359 = other_2358;
    let _e4: Point = self_2729;
    let _e5: Line = other_2359;
    let _e6: Line = line_inverse(_e5);
    let _e7: Flector = point_line_geometric_product(_e4, _e6);
    return _e7;
}

fn point_motor_geometric_quotient(self_2730: Point, other_2360: Motor) -> Flector {
    var self_2731: Point;
    var other_2361: Motor;

    self_2731 = self_2730;
    other_2361 = other_2360;
    let _e4: Point = self_2731;
    let _e5: Motor = other_2361;
    let _e6: Motor = motor_inverse(_e5);
    let _e7: Flector = point_motor_geometric_product(_e4, _e6);
    return _e7;
}

fn point_plane_geometric_quotient(self_2732: Point, other_2362: Plane) -> Motor {
    var self_2733: Point;
    var other_2363: Plane;

    self_2733 = self_2732;
    other_2363 = other_2362;
    let _e4: Point = self_2733;
    let _e5: Plane = other_2363;
    let _e6: Plane = plane_inverse(_e5);
    let _e7: Motor = point_plane_geometric_product(_e4, _e6);
    return _e7;
}

fn point_plane_transformation(self_2734: Point, other_2364: Plane) -> Plane {
    var self_2735: Point;
    var other_2365: Plane;

    self_2735 = self_2734;
    other_2365 = other_2364;
    let _e4: Point = self_2735;
    let _e5: Plane = other_2365;
    let _e6: Motor = point_plane_geometric_product(_e4, _e5);
    let _e7: Point = self_2735;
    let _e8: Point = point_reversal(_e7);
    let _e9: Flector = motor_point_geometric_product(_e6, _e8);
    let _e10: Plane = flector_plane_into(_e9);
    return _e10;
}

fn point_scalar_geometric_quotient(self_2736: Point, other_2366: Scalar) -> Point {
    var self_2737: Point;
    var other_2367: Scalar;

    self_2737 = self_2736;
    other_2367 = other_2366;
    let _e4: Point = self_2737;
    let _e5: Scalar = other_2367;
    let _e6: Scalar = scalar_inverse(_e5);
    let _e7: Point = point_scalar_geometric_product(_e4, _e6);
    return _e7;
}

fn rotor_homogeneous_magnitude_geometric_quotient(self_2738: Rotor, other_2368: HomogeneousMagnitude) -> Rotor {
    var self_2739: Rotor;
    var other_2369: HomogeneousMagnitude;

    self_2739 = self_2738;
    other_2369 = other_2368;
    let _e4: Rotor = self_2739;
    let _e5: HomogeneousMagnitude = other_2369;
    let _e6: HomogeneousMagnitude = homogeneous_magnitude_inverse(_e5);
    let _e7: Rotor = rotor_homogeneous_magnitude_geometric_product(_e4, _e6);
    return _e7;
}

fn rotor_line_geometric_quotient(self_2740: Rotor, other_2370: Line) -> Rotor {
    var self_2741: Rotor;
    var other_2371: Line;

    self_2741 = self_2740;
    other_2371 = other_2370;
    let _e4: Rotor = self_2741;
    let _e5: Line = other_2371;
    let _e6: Line = line_inverse(_e5);
    let _e7: Rotor = rotor_line_geometric_product(_e4, _e6);
    return _e7;
}

fn rotor_motor_geometric_quotient(self_2742: Rotor, other_2372: Motor) -> Rotor {
    var self_2743: Rotor;
    var other_2373: Motor;

    self_2743 = self_2742;
    other_2373 = other_2372;
    let _e4: Rotor = self_2743;
    let _e5: Motor = other_2373;
    let _e6: Motor = motor_inverse(_e5);
    let _e7: Rotor = rotor_motor_geometric_product(_e4, _e6);
    return _e7;
}

fn rotor_scalar_geometric_quotient(self_2744: Rotor, other_2374: Scalar) -> Rotor {
    var self_2745: Rotor;
    var other_2375: Scalar;

    self_2745 = self_2744;
    other_2375 = other_2374;
    let _e4: Rotor = self_2745;
    let _e5: Scalar = other_2375;
    let _e6: Scalar = scalar_inverse(_e5);
    let _e7: Rotor = rotor_scalar_geometric_product(_e4, _e6);
    return _e7;
}

fn rotor_translator_geometric_quotient(self_2746: Rotor, other_2376: Translator) -> Rotor {
    var self_2747: Rotor;
    var other_2377: Translator;

    self_2747 = self_2746;
    other_2377 = other_2376;
    let _e4: Rotor = self_2747;
    let _e5: Translator = other_2377;
    let _e6: Translator = translator_inverse(_e5);
    let _e7: Rotor = rotor_translator_geometric_product(_e4, _e6);
    return _e7;
}

fn scalar_anti_scalar_transformation(self_2748: Scalar, other_2378: AntiScalar) -> AntiScalar {
    var self_2749: Scalar;
    var other_2379: AntiScalar;

    self_2749 = self_2748;
    other_2379 = other_2378;
    let _e4: Scalar = self_2749;
    let _e5: AntiScalar = other_2379;
    let _e6: AntiScalar = scalar_anti_scalar_geometric_product(_e4, _e5);
    let _e7: Scalar = self_2749;
    let _e8: Scalar = scalar_reversal(_e7);
    let _e9: AntiScalar = anti_scalar_scalar_geometric_product(_e6, _e8);
    return _e9;
}

fn scalar_flector_geometric_quotient(self_2750: Scalar, other_2380: Flector) -> Flector {
    var self_2751: Scalar;
    var other_2381: Flector;

    self_2751 = self_2750;
    other_2381 = other_2380;
    let _e4: Scalar = self_2751;
    let _e5: Flector = other_2381;
    let _e6: Flector = flector_inverse(_e5);
    let _e7: Flector = scalar_flector_geometric_product(_e4, _e6);
    return _e7;
}

fn scalar_flector_transformation(self_2752: Scalar, other_2382: Flector) -> Flector {
    var self_2753: Scalar;
    var other_2383: Flector;

    self_2753 = self_2752;
    other_2383 = other_2382;
    let _e4: Scalar = self_2753;
    let _e5: Flector = other_2383;
    let _e6: Flector = scalar_flector_geometric_product(_e4, _e5);
    let _e7: Scalar = self_2753;
    let _e8: Scalar = scalar_reversal(_e7);
    let _e9: Flector = flector_scalar_geometric_product(_e6, _e8);
    return _e9;
}

fn scalar_homogeneous_magnitude_geometric_quotient(self_2754: Scalar, other_2384: HomogeneousMagnitude) -> HomogeneousMagnitude {
    var self_2755: Scalar;
    var other_2385: HomogeneousMagnitude;

    self_2755 = self_2754;
    other_2385 = other_2384;
    let _e4: Scalar = self_2755;
    let _e5: HomogeneousMagnitude = other_2385;
    let _e6: HomogeneousMagnitude = homogeneous_magnitude_inverse(_e5);
    let _e7: HomogeneousMagnitude = scalar_homogeneous_magnitude_geometric_product(_e4, _e6);
    return _e7;
}

fn scalar_homogeneous_magnitude_transformation(self_2756: Scalar, other_2386: HomogeneousMagnitude) -> HomogeneousMagnitude {
    var self_2757: Scalar;
    var other_2387: HomogeneousMagnitude;

    self_2757 = self_2756;
    other_2387 = other_2386;
    let _e4: Scalar = self_2757;
    let _e5: HomogeneousMagnitude = other_2387;
    let _e6: HomogeneousMagnitude = scalar_homogeneous_magnitude_geometric_product(_e4, _e5);
    let _e7: Scalar = self_2757;
    let _e8: Scalar = scalar_reversal(_e7);
    let _e9: HomogeneousMagnitude = homogeneous_magnitude_scalar_geometric_product(_e6, _e8);
    return _e9;
}

fn scalar_line_geometric_quotient(self_2758: Scalar, other_2388: Line) -> Line {
    var self_2759: Scalar;
    var other_2389: Line;

    self_2759 = self_2758;
    other_2389 = other_2388;
    let _e4: Scalar = self_2759;
    let _e5: Line = other_2389;
    let _e6: Line = line_inverse(_e5);
    let _e7: Line = scalar_line_geometric_product(_e4, _e6);
    return _e7;
}

fn scalar_line_transformation(self_2760: Scalar, other_2390: Line) -> Line {
    var self_2761: Scalar;
    var other_2391: Line;

    self_2761 = self_2760;
    other_2391 = other_2390;
    let _e4: Scalar = self_2761;
    let _e5: Line = other_2391;
    let _e6: Line = scalar_line_geometric_product(_e4, _e5);
    let _e7: Scalar = self_2761;
    let _e8: Scalar = scalar_reversal(_e7);
    let _e9: Line = line_scalar_geometric_product(_e6, _e8);
    return _e9;
}

fn scalar_motor_geometric_quotient(self_2762: Scalar, other_2392: Motor) -> Motor {
    var self_2763: Scalar;
    var other_2393: Motor;

    self_2763 = self_2762;
    other_2393 = other_2392;
    let _e4: Scalar = self_2763;
    let _e5: Motor = other_2393;
    let _e6: Motor = motor_inverse(_e5);
    let _e7: Motor = scalar_motor_geometric_product(_e4, _e6);
    return _e7;
}

fn scalar_motor_transformation(self_2764: Scalar, other_2394: Motor) -> Motor {
    var self_2765: Scalar;
    var other_2395: Motor;

    self_2765 = self_2764;
    other_2395 = other_2394;
    let _e4: Scalar = self_2765;
    let _e5: Motor = other_2395;
    let _e6: Motor = scalar_motor_geometric_product(_e4, _e5);
    let _e7: Scalar = self_2765;
    let _e8: Scalar = scalar_reversal(_e7);
    let _e9: Motor = motor_scalar_geometric_product(_e6, _e8);
    return _e9;
}

fn scalar_plane_geometric_quotient(self_2766: Scalar, other_2396: Plane) -> Plane {
    var self_2767: Scalar;
    var other_2397: Plane;

    self_2767 = self_2766;
    other_2397 = other_2396;
    let _e4: Scalar = self_2767;
    let _e5: Plane = other_2397;
    let _e6: Plane = plane_inverse(_e5);
    let _e7: Plane = scalar_plane_geometric_product(_e4, _e6);
    return _e7;
}

fn scalar_plane_transformation(self_2768: Scalar, other_2398: Plane) -> Plane {
    var self_2769: Scalar;
    var other_2399: Plane;

    self_2769 = self_2768;
    other_2399 = other_2398;
    let _e4: Scalar = self_2769;
    let _e5: Plane = other_2399;
    let _e6: Plane = scalar_plane_geometric_product(_e4, _e5);
    let _e7: Scalar = self_2769;
    let _e8: Scalar = scalar_reversal(_e7);
    let _e9: Plane = plane_scalar_geometric_product(_e6, _e8);
    return _e9;
}

fn scalar_point_geometric_quotient(self_2770: Scalar, other_2400: Point) -> Point {
    var self_2771: Scalar;
    var other_2401: Point;

    self_2771 = self_2770;
    other_2401 = other_2400;
    let _e4: Scalar = self_2771;
    let _e5: Point = other_2401;
    let _e6: Point = point_inverse(_e5);
    let _e7: Point = scalar_point_geometric_product(_e4, _e6);
    return _e7;
}

fn scalar_point_transformation(self_2772: Scalar, other_2402: Point) -> Point {
    var self_2773: Scalar;
    var other_2403: Point;

    self_2773 = self_2772;
    other_2403 = other_2402;
    let _e4: Scalar = self_2773;
    let _e5: Point = other_2403;
    let _e6: Point = scalar_point_geometric_product(_e4, _e5);
    let _e7: Scalar = self_2773;
    let _e8: Scalar = scalar_reversal(_e7);
    let _e9: Point = point_scalar_geometric_product(_e6, _e8);
    return _e9;
}

fn scalar_rotor_transformation(self_2774: Scalar, other_2404: Rotor) -> Rotor {
    var self_2775: Scalar;
    var other_2405: Rotor;

    self_2775 = self_2774;
    other_2405 = other_2404;
    let _e4: Scalar = self_2775;
    let _e5: Rotor = other_2405;
    let _e6: Rotor = scalar_rotor_geometric_product(_e4, _e5);
    let _e7: Scalar = self_2775;
    let _e8: Scalar = scalar_reversal(_e7);
    let _e9: Rotor = rotor_scalar_geometric_product(_e6, _e8);
    return _e9;
}

fn scalar_powi(self_2776: Scalar, exponent_2: i32) -> Scalar {
    var self_2777: Scalar;
    var exponent_3: i32;
    var local_1: Scalar;
    var x_1: Scalar;
    var y_1: Scalar;
    var n_1: i32;

    self_2777 = self_2776;
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
        let _e11: Scalar = self_2777;
        let _e12: Scalar = scalar_inverse(_e11);
        local_1 = _e12;
    } else {
        let _e14: Scalar = self_2777;
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

fn scalar_scalar_geometric_quotient(self_2778: Scalar, other_2406: Scalar) -> Scalar {
    var self_2779: Scalar;
    var other_2407: Scalar;

    self_2779 = self_2778;
    other_2407 = other_2406;
    let _e4: Scalar = self_2779;
    let _e5: Scalar = other_2407;
    let _e6: Scalar = scalar_inverse(_e5);
    let _e7: Scalar = scalar_scalar_geometric_product(_e4, _e6);
    return _e7;
}

fn scalar_scalar_transformation(self_2780: Scalar, other_2408: Scalar) -> Scalar {
    var self_2781: Scalar;
    var other_2409: Scalar;

    self_2781 = self_2780;
    other_2409 = other_2408;
    let _e4: Scalar = self_2781;
    let _e5: Scalar = other_2409;
    let _e6: Scalar = scalar_scalar_geometric_product(_e4, _e5);
    let _e7: Scalar = self_2781;
    let _e8: Scalar = scalar_reversal(_e7);
    let _e9: Scalar = scalar_scalar_geometric_product(_e6, _e8);
    return _e9;
}

fn scalar_translator_geometric_quotient(self_2782: Scalar, other_2410: Translator) -> Translator {
    var self_2783: Scalar;
    var other_2411: Translator;

    self_2783 = self_2782;
    other_2411 = other_2410;
    let _e4: Scalar = self_2783;
    let _e5: Translator = other_2411;
    let _e6: Translator = translator_inverse(_e5);
    let _e7: Translator = scalar_translator_geometric_product(_e4, _e6);
    return _e7;
}

fn scalar_translator_transformation(self_2784: Scalar, other_2412: Translator) -> Translator {
    var self_2785: Scalar;
    var other_2413: Translator;

    self_2785 = self_2784;
    other_2413 = other_2412;
    let _e4: Scalar = self_2785;
    let _e5: Translator = other_2413;
    let _e6: Translator = scalar_translator_geometric_product(_e4, _e5);
    let _e7: Scalar = self_2785;
    let _e8: Scalar = scalar_reversal(_e7);
    let _e9: Translator = translator_scalar_geometric_product(_e6, _e8);
    return _e9;
}

fn translator_flector_geometric_quotient(self_2786: Translator, other_2414: Flector) -> Flector {
    var self_2787: Translator;
    var other_2415: Flector;

    self_2787 = self_2786;
    other_2415 = other_2414;
    let _e4: Translator = self_2787;
    let _e5: Flector = other_2415;
    let _e6: Flector = flector_inverse(_e5);
    let _e7: Flector = translator_flector_geometric_product(_e4, _e6);
    return _e7;
}

fn translator_flector_transformation(self_2788: Translator, other_2416: Flector) -> Flector {
    var self_2789: Translator;
    var other_2417: Flector;

    self_2789 = self_2788;
    other_2417 = other_2416;
    let _e4: Translator = self_2789;
    let _e5: Flector = other_2417;
    let _e6: Flector = translator_flector_geometric_product(_e4, _e5);
    let _e7: Translator = self_2789;
    let _e8: Translator = translator_reversal(_e7);
    let _e9: Flector = flector_translator_geometric_product(_e6, _e8);
    return _e9;
}

fn translator_homogeneous_magnitude_geometric_quotient(self_2790: Translator, other_2418: HomogeneousMagnitude) -> Motor {
    var self_2791: Translator;
    var other_2419: HomogeneousMagnitude;

    self_2791 = self_2790;
    other_2419 = other_2418;
    let _e4: Translator = self_2791;
    let _e5: HomogeneousMagnitude = other_2419;
    let _e6: HomogeneousMagnitude = homogeneous_magnitude_inverse(_e5);
    let _e7: Motor = translator_homogeneous_magnitude_geometric_product(_e4, _e6);
    return _e7;
}

fn translator_rotor_transformation(self_2792: Translator, other_2420: Rotor) -> Rotor {
    var self_2793: Translator;
    var other_2421: Rotor;

    self_2793 = self_2792;
    other_2421 = other_2420;
    let _e4: Translator = self_2793;
    let _e5: Rotor = other_2421;
    let _e6: Rotor = translator_rotor_geometric_product(_e4, _e5);
    let _e7: Translator = self_2793;
    let _e8: Translator = translator_reversal(_e7);
    let _e9: Rotor = rotor_translator_geometric_product(_e6, _e8);
    return _e9;
}

fn translator_scalar_geometric_quotient(self_2794: Translator, other_2422: Scalar) -> Translator {
    var self_2795: Translator;
    var other_2423: Scalar;

    self_2795 = self_2794;
    other_2423 = other_2422;
    let _e4: Translator = self_2795;
    let _e5: Scalar = other_2423;
    let _e6: Scalar = scalar_inverse(_e5);
    let _e7: Translator = translator_scalar_geometric_product(_e4, _e6);
    return _e7;
}

fn flector_line_distance(self_2796: Flector, other_2424: Line) -> HomogeneousMagnitude {
    var self_2797: Flector;
    var other_2425: Line;

    self_2797 = self_2796;
    other_2425 = other_2424;
    let _e4: Flector = self_2797;
    let _e5: Line = other_2425;
    let _e6: Plane = flector_line_outer_product(_e4, _e5);
    let _e7: Line = plane_attitude(_e6);
    let _e8: Scalar = line_bulk_norm(_e7);
    let _e9: Flector = self_2797;
    let _e10: Line = other_2425;
    let _e11: Point = line_attitude(_e10);
    let _e12: Motor = flector_point_outer_product(_e9, _e11);
    let _e13: AntiScalar = motor_weight_norm(_e12);
    let _e14: HomogeneousMagnitude = scalar_anti_scalar_add(_e8, _e13);
    return _e14;
}

fn flector_motor_distance(self_2798: Flector, other_2426: Motor) -> HomogeneousMagnitude {
    var self_2799: Flector;
    var other_2427: Motor;

    self_2799 = self_2798;
    other_2427 = other_2426;
    let _e4: Flector = self_2799;
    let _e5: Motor = other_2427;
    let _e6: Plane = flector_motor_outer_product(_e4, _e5);
    let _e7: Line = plane_attitude(_e6);
    let _e8: Scalar = line_bulk_norm(_e7);
    let _e9: Flector = self_2799;
    let _e10: Motor = other_2427;
    let _e11: Flector = motor_attitude(_e10);
    let _e12: Motor = flector_flector_outer_product(_e9, _e11);
    let _e13: AntiScalar = motor_weight_norm(_e12);
    let _e14: HomogeneousMagnitude = scalar_anti_scalar_add(_e8, _e13);
    return _e14;
}

fn flector_plane_distance(self_2800: Flector, other_2428: Plane) -> HomogeneousMagnitude {
    var self_2801: Flector;
    var other_2429: Plane;

    self_2801 = self_2800;
    other_2429 = other_2428;
    let _e4: Flector = self_2801;
    let _e5: Plane = other_2429;
    let _e6: AntiScalar = flector_plane_outer_product(_e4, _e5);
    let _e7: Plane = anti_scalar_attitude(_e6);
    let _e8: Scalar = plane_bulk_norm(_e7);
    let _e9: Flector = self_2801;
    let _e10: Plane = other_2429;
    let _e11: Line = plane_attitude(_e10);
    let _e12: Plane = flector_line_outer_product(_e9, _e11);
    let _e13: AntiScalar = plane_weight_norm(_e12);
    let _e14: HomogeneousMagnitude = scalar_anti_scalar_add(_e8, _e13);
    return _e14;
}

fn flector_point_distance(self_2802: Flector, other_2430: Point) -> HomogeneousMagnitude {
    var self_2803: Flector;
    var other_2431: Point;

    self_2803 = self_2802;
    other_2431 = other_2430;
    let _e4: Flector = self_2803;
    let _e5: Point = other_2431;
    let _e6: Motor = flector_point_outer_product(_e4, _e5);
    let _e7: Flector = motor_attitude(_e6);
    let _e8: Scalar = flector_bulk_norm(_e7);
    let _e9: Flector = self_2803;
    let _e10: Point = other_2431;
    let _e11: Scalar = point_attitude(_e10);
    let _e12: Flector = flector_scalar_outer_product(_e9, _e11);
    let _e13: AntiScalar = flector_weight_norm(_e12);
    let _e14: HomogeneousMagnitude = scalar_anti_scalar_add(_e8, _e13);
    return _e14;
}

fn homogeneous_magnitude_anti_scalar_distance(self_2804: HomogeneousMagnitude, other_2432: AntiScalar) -> HomogeneousMagnitude {
    var self_2805: HomogeneousMagnitude;
    var other_2433: AntiScalar;

    self_2805 = self_2804;
    other_2433 = other_2432;
    let _e4: HomogeneousMagnitude = self_2805;
    let _e5: AntiScalar = other_2433;
    let _e6: AntiScalar = homogeneous_magnitude_anti_scalar_outer_product(_e4, _e5);
    let _e7: Plane = anti_scalar_attitude(_e6);
    let _e8: Scalar = plane_bulk_norm(_e7);
    let _e9: HomogeneousMagnitude = self_2805;
    let _e10: AntiScalar = other_2433;
    let _e11: Plane = anti_scalar_attitude(_e10);
    let _e12: Plane = homogeneous_magnitude_plane_outer_product(_e9, _e11);
    let _e13: AntiScalar = plane_weight_norm(_e12);
    let _e14: HomogeneousMagnitude = scalar_anti_scalar_add(_e8, _e13);
    return _e14;
}

fn homogeneous_magnitude_homogeneous_magnitude_distance(self_2806: HomogeneousMagnitude, other_2434: HomogeneousMagnitude) -> HomogeneousMagnitude {
    var self_2807: HomogeneousMagnitude;
    var other_2435: HomogeneousMagnitude;

    self_2807 = self_2806;
    other_2435 = other_2434;
    let _e4: HomogeneousMagnitude = self_2807;
    let _e5: HomogeneousMagnitude = other_2435;
    let _e6: HomogeneousMagnitude = homogeneous_magnitude_homogeneous_magnitude_outer_product(_e4, _e5);
    let _e7: Plane = homogeneous_magnitude_attitude(_e6);
    let _e8: Scalar = plane_bulk_norm(_e7);
    let _e9: HomogeneousMagnitude = self_2807;
    let _e10: HomogeneousMagnitude = other_2435;
    let _e11: Plane = homogeneous_magnitude_attitude(_e10);
    let _e12: Plane = homogeneous_magnitude_plane_outer_product(_e9, _e11);
    let _e13: AntiScalar = plane_weight_norm(_e12);
    let _e14: HomogeneousMagnitude = scalar_anti_scalar_add(_e8, _e13);
    return _e14;
}

fn homogeneous_magnitude_line_distance(self_2808: HomogeneousMagnitude, other_2436: Line) -> HomogeneousMagnitude {
    var self_2809: HomogeneousMagnitude;
    var other_2437: Line;

    self_2809 = self_2808;
    other_2437 = other_2436;
    let _e4: HomogeneousMagnitude = self_2809;
    let _e5: Line = other_2437;
    let _e6: Line = homogeneous_magnitude_line_outer_product(_e4, _e5);
    let _e7: Point = line_attitude(_e6);
    let _e8: Scalar = point_bulk_norm(_e7);
    let _e9: HomogeneousMagnitude = self_2809;
    let _e10: Line = other_2437;
    let _e11: Point = line_attitude(_e10);
    let _e12: Point = homogeneous_magnitude_point_outer_product(_e9, _e11);
    let _e13: AntiScalar = point_weight_norm(_e12);
    let _e14: HomogeneousMagnitude = scalar_anti_scalar_add(_e8, _e13);
    return _e14;
}

fn homogeneous_magnitude_motor_distance(self_2810: HomogeneousMagnitude, other_2438: Motor) -> HomogeneousMagnitude {
    var self_2811: HomogeneousMagnitude;
    var other_2439: Motor;

    self_2811 = self_2810;
    other_2439 = other_2438;
    let _e4: HomogeneousMagnitude = self_2811;
    let _e5: Motor = other_2439;
    let _e6: Motor = homogeneous_magnitude_motor_outer_product(_e4, _e5);
    let _e7: Flector = motor_attitude(_e6);
    let _e8: Scalar = flector_bulk_norm(_e7);
    let _e9: HomogeneousMagnitude = self_2811;
    let _e10: Motor = other_2439;
    let _e11: Flector = motor_attitude(_e10);
    let _e12: Flector = homogeneous_magnitude_flector_outer_product(_e9, _e11);
    let _e13: AntiScalar = flector_weight_norm(_e12);
    let _e14: HomogeneousMagnitude = scalar_anti_scalar_add(_e8, _e13);
    return _e14;
}

fn homogeneous_magnitude_plane_distance(self_2812: HomogeneousMagnitude, other_2440: Plane) -> HomogeneousMagnitude {
    var self_2813: HomogeneousMagnitude;
    var other_2441: Plane;

    self_2813 = self_2812;
    other_2441 = other_2440;
    let _e4: HomogeneousMagnitude = self_2813;
    let _e5: Plane = other_2441;
    let _e6: Plane = homogeneous_magnitude_plane_outer_product(_e4, _e5);
    let _e7: Line = plane_attitude(_e6);
    let _e8: Scalar = line_bulk_norm(_e7);
    let _e9: HomogeneousMagnitude = self_2813;
    let _e10: Plane = other_2441;
    let _e11: Line = plane_attitude(_e10);
    let _e12: Line = homogeneous_magnitude_line_outer_product(_e9, _e11);
    let _e13: AntiScalar = line_weight_norm(_e12);
    let _e14: HomogeneousMagnitude = scalar_anti_scalar_add(_e8, _e13);
    return _e14;
}

fn homogeneous_magnitude_rotor_distance(self_2814: HomogeneousMagnitude, other_2442: Rotor) -> HomogeneousMagnitude {
    var self_2815: HomogeneousMagnitude;
    var other_2443: Rotor;

    self_2815 = self_2814;
    other_2443 = other_2442;
    let _e4: HomogeneousMagnitude = self_2815;
    let _e5: Rotor = other_2443;
    let _e6: Rotor = homogeneous_magnitude_rotor_outer_product(_e4, _e5);
    let _e7: Flector = rotor_attitude(_e6);
    let _e8: Scalar = flector_bulk_norm(_e7);
    let _e9: HomogeneousMagnitude = self_2815;
    let _e10: Rotor = other_2443;
    let _e11: Flector = rotor_attitude(_e10);
    let _e12: Flector = homogeneous_magnitude_flector_outer_product(_e9, _e11);
    let _e13: AntiScalar = flector_weight_norm(_e12);
    let _e14: HomogeneousMagnitude = scalar_anti_scalar_add(_e8, _e13);
    return _e14;
}

fn line_line_distance(self_2816: Line, other_2444: Line) -> HomogeneousMagnitude {
    var self_2817: Line;
    var other_2445: Line;

    self_2817 = self_2816;
    other_2445 = other_2444;
    let _e4: Line = self_2817;
    let _e5: Line = other_2445;
    let _e6: AntiScalar = line_line_outer_product(_e4, _e5);
    let _e7: Plane = anti_scalar_attitude(_e6);
    let _e8: Scalar = plane_bulk_norm(_e7);
    let _e9: Line = self_2817;
    let _e10: Line = other_2445;
    let _e11: Point = line_attitude(_e10);
    let _e12: Plane = line_point_outer_product(_e9, _e11);
    let _e13: AntiScalar = plane_weight_norm(_e12);
    let _e14: HomogeneousMagnitude = scalar_anti_scalar_add(_e8, _e13);
    return _e14;
}

fn line_motor_distance(self_2818: Line, other_2446: Motor) -> HomogeneousMagnitude {
    var self_2819: Line;
    var other_2447: Motor;

    self_2819 = self_2818;
    other_2447 = other_2446;
    let _e4: Line = self_2819;
    let _e5: Motor = other_2447;
    let _e6: AntiScalar = line_motor_outer_product(_e4, _e5);
    let _e7: Plane = anti_scalar_attitude(_e6);
    let _e8: Scalar = plane_bulk_norm(_e7);
    let _e9: Line = self_2819;
    let _e10: Motor = other_2447;
    let _e11: Flector = motor_attitude(_e10);
    let _e12: Plane = line_flector_outer_product(_e9, _e11);
    let _e13: AntiScalar = plane_weight_norm(_e12);
    let _e14: HomogeneousMagnitude = scalar_anti_scalar_add(_e8, _e13);
    return _e14;
}

fn line_point_distance(self_2820: Line, other_2448: Point) -> HomogeneousMagnitude {
    var self_2821: Line;
    var other_2449: Point;

    self_2821 = self_2820;
    other_2449 = other_2448;
    let _e4: Line = self_2821;
    let _e5: Point = other_2449;
    let _e6: Plane = line_point_outer_product(_e4, _e5);
    let _e7: Line = plane_attitude(_e6);
    let _e8: Scalar = line_bulk_norm(_e7);
    let _e9: Line = self_2821;
    let _e10: Point = other_2449;
    let _e11: Scalar = point_attitude(_e10);
    let _e12: Line = line_scalar_outer_product(_e9, _e11);
    let _e13: AntiScalar = line_weight_norm(_e12);
    let _e14: HomogeneousMagnitude = scalar_anti_scalar_add(_e8, _e13);
    return _e14;
}

fn line_rotor_distance(self_2822: Line, other_2450: Rotor) -> HomogeneousMagnitude {
    var self_2823: Line;
    var other_2451: Rotor;

    self_2823 = self_2822;
    other_2451 = other_2450;
    let _e4: Line = self_2823;
    let _e5: Rotor = other_2451;
    let _e6: AntiScalar = line_rotor_outer_product(_e4, _e5);
    let _e7: Plane = anti_scalar_attitude(_e6);
    let _e8: Scalar = plane_bulk_norm(_e7);
    let _e9: Line = self_2823;
    let _e10: Rotor = other_2451;
    let _e11: Flector = rotor_attitude(_e10);
    let _e12: Plane = line_flector_outer_product(_e9, _e11);
    let _e13: AntiScalar = plane_weight_norm(_e12);
    let _e14: HomogeneousMagnitude = scalar_anti_scalar_add(_e8, _e13);
    return _e14;
}

fn motor_line_distance(self_2824: Motor, other_2452: Line) -> HomogeneousMagnitude {
    var self_2825: Motor;
    var other_2453: Line;

    self_2825 = self_2824;
    other_2453 = other_2452;
    let _e4: Motor = self_2825;
    let _e5: Line = other_2453;
    let _e6: AntiScalar = motor_line_outer_product(_e4, _e5);
    let _e7: Plane = anti_scalar_attitude(_e6);
    let _e8: Scalar = plane_bulk_norm(_e7);
    let _e9: Motor = self_2825;
    let _e10: Line = other_2453;
    let _e11: Point = line_attitude(_e10);
    let _e12: Plane = motor_point_outer_product(_e9, _e11);
    let _e13: AntiScalar = plane_weight_norm(_e12);
    let _e14: HomogeneousMagnitude = scalar_anti_scalar_add(_e8, _e13);
    return _e14;
}

fn motor_motor_distance(self_2826: Motor, other_2454: Motor) -> HomogeneousMagnitude {
    var self_2827: Motor;
    var other_2455: Motor;

    self_2827 = self_2826;
    other_2455 = other_2454;
    let _e4: Motor = self_2827;
    let _e5: Motor = other_2455;
    let _e6: AntiScalar = motor_motor_outer_product(_e4, _e5);
    let _e7: Plane = anti_scalar_attitude(_e6);
    let _e8: Scalar = plane_bulk_norm(_e7);
    let _e9: Motor = self_2827;
    let _e10: Motor = other_2455;
    let _e11: Flector = motor_attitude(_e10);
    let _e12: Plane = motor_flector_outer_product(_e9, _e11);
    let _e13: AntiScalar = plane_weight_norm(_e12);
    let _e14: HomogeneousMagnitude = scalar_anti_scalar_add(_e8, _e13);
    return _e14;
}

fn motor_point_distance(self_2828: Motor, other_2456: Point) -> HomogeneousMagnitude {
    var self_2829: Motor;
    var other_2457: Point;

    self_2829 = self_2828;
    other_2457 = other_2456;
    let _e4: Motor = self_2829;
    let _e5: Point = other_2457;
    let _e6: Plane = motor_point_outer_product(_e4, _e5);
    let _e7: Line = plane_attitude(_e6);
    let _e8: Scalar = line_bulk_norm(_e7);
    let _e9: Motor = self_2829;
    let _e10: Point = other_2457;
    let _e11: Scalar = point_attitude(_e10);
    let _e12: Motor = motor_scalar_outer_product(_e9, _e11);
    let _e13: AntiScalar = motor_weight_norm(_e12);
    let _e14: HomogeneousMagnitude = scalar_anti_scalar_add(_e8, _e13);
    return _e14;
}

fn motor_rotor_distance(self_2830: Motor, other_2458: Rotor) -> HomogeneousMagnitude {
    var self_2831: Motor;
    var other_2459: Rotor;

    self_2831 = self_2830;
    other_2459 = other_2458;
    let _e4: Motor = self_2831;
    let _e5: Rotor = other_2459;
    let _e6: AntiScalar = motor_rotor_outer_product(_e4, _e5);
    let _e7: Plane = anti_scalar_attitude(_e6);
    let _e8: Scalar = plane_bulk_norm(_e7);
    let _e9: Motor = self_2831;
    let _e10: Rotor = other_2459;
    let _e11: Flector = rotor_attitude(_e10);
    let _e12: Plane = motor_flector_outer_product(_e9, _e11);
    let _e13: AntiScalar = plane_weight_norm(_e12);
    let _e14: HomogeneousMagnitude = scalar_anti_scalar_add(_e8, _e13);
    return _e14;
}

fn plane_point_distance(self_2832: Plane, other_2460: Point) -> HomogeneousMagnitude {
    var self_2833: Plane;
    var other_2461: Point;

    self_2833 = self_2832;
    other_2461 = other_2460;
    let _e4: Plane = self_2833;
    let _e5: Point = other_2461;
    let _e6: AntiScalar = plane_point_outer_product(_e4, _e5);
    let _e7: Plane = anti_scalar_attitude(_e6);
    let _e8: Scalar = plane_bulk_norm(_e7);
    let _e9: Plane = self_2833;
    let _e10: Point = other_2461;
    let _e11: Scalar = point_attitude(_e10);
    let _e12: Plane = plane_scalar_outer_product(_e9, _e11);
    let _e13: AntiScalar = plane_weight_norm(_e12);
    let _e14: HomogeneousMagnitude = scalar_anti_scalar_add(_e8, _e13);
    return _e14;
}

fn point_line_distance(self_2834: Point, other_2462: Line) -> HomogeneousMagnitude {
    var self_2835: Point;
    var other_2463: Line;

    self_2835 = self_2834;
    other_2463 = other_2462;
    let _e4: Point = self_2835;
    let _e5: Line = other_2463;
    let _e6: Plane = point_line_outer_product(_e4, _e5);
    let _e7: Line = plane_attitude(_e6);
    let _e8: Scalar = line_bulk_norm(_e7);
    let _e9: Point = self_2835;
    let _e10: Line = other_2463;
    let _e11: Point = line_attitude(_e10);
    let _e12: Line = point_point_outer_product(_e9, _e11);
    let _e13: AntiScalar = line_weight_norm(_e12);
    let _e14: HomogeneousMagnitude = scalar_anti_scalar_add(_e8, _e13);
    return _e14;
}

fn point_motor_distance(self_2836: Point, other_2464: Motor) -> HomogeneousMagnitude {
    var self_2837: Point;
    var other_2465: Motor;

    self_2837 = self_2836;
    other_2465 = other_2464;
    let _e4: Point = self_2837;
    let _e5: Motor = other_2465;
    let _e6: Plane = point_motor_outer_product(_e4, _e5);
    let _e7: Line = plane_attitude(_e6);
    let _e8: Scalar = line_bulk_norm(_e7);
    let _e9: Point = self_2837;
    let _e10: Motor = other_2465;
    let _e11: Flector = motor_attitude(_e10);
    let _e12: Motor = point_flector_outer_product(_e9, _e11);
    let _e13: AntiScalar = motor_weight_norm(_e12);
    let _e14: HomogeneousMagnitude = scalar_anti_scalar_add(_e8, _e13);
    return _e14;
}

fn point_plane_distance(self_2838: Point, other_2466: Plane) -> HomogeneousMagnitude {
    var self_2839: Point;
    var other_2467: Plane;

    self_2839 = self_2838;
    other_2467 = other_2466;
    let _e4: Point = self_2839;
    let _e5: Plane = other_2467;
    let _e6: AntiScalar = point_plane_outer_product(_e4, _e5);
    let _e7: Plane = anti_scalar_attitude(_e6);
    let _e8: Scalar = plane_bulk_norm(_e7);
    let _e9: Point = self_2839;
    let _e10: Plane = other_2467;
    let _e11: Line = plane_attitude(_e10);
    let _e12: Plane = point_line_outer_product(_e9, _e11);
    let _e13: AntiScalar = plane_weight_norm(_e12);
    let _e14: HomogeneousMagnitude = scalar_anti_scalar_add(_e8, _e13);
    return _e14;
}

fn point_point_distance(self_2840: Point, other_2468: Point) -> HomogeneousMagnitude {
    var self_2841: Point;
    var other_2469: Point;

    self_2841 = self_2840;
    other_2469 = other_2468;
    let _e4: Point = self_2841;
    let _e5: Point = other_2469;
    let _e6: Line = point_point_outer_product(_e4, _e5);
    let _e7: Point = line_attitude(_e6);
    let _e8: Scalar = point_bulk_norm(_e7);
    let _e9: Point = self_2841;
    let _e10: Point = other_2469;
    let _e11: Scalar = point_attitude(_e10);
    let _e12: Point = point_scalar_outer_product(_e9, _e11);
    let _e13: AntiScalar = point_weight_norm(_e12);
    let _e14: HomogeneousMagnitude = scalar_anti_scalar_add(_e8, _e13);
    return _e14;
}

fn scalar_anti_scalar_distance(self_2842: Scalar, other_2470: AntiScalar) -> HomogeneousMagnitude {
    var self_2843: Scalar;
    var other_2471: AntiScalar;

    self_2843 = self_2842;
    other_2471 = other_2470;
    let _e4: Scalar = self_2843;
    let _e5: AntiScalar = other_2471;
    let _e6: AntiScalar = scalar_anti_scalar_outer_product(_e4, _e5);
    let _e7: Plane = anti_scalar_attitude(_e6);
    let _e8: Scalar = plane_bulk_norm(_e7);
    let _e9: Scalar = self_2843;
    let _e10: AntiScalar = other_2471;
    let _e11: Plane = anti_scalar_attitude(_e10);
    let _e12: Plane = scalar_plane_outer_product(_e9, _e11);
    let _e13: AntiScalar = plane_weight_norm(_e12);
    let _e14: HomogeneousMagnitude = scalar_anti_scalar_add(_e8, _e13);
    return _e14;
}

fn scalar_homogeneous_magnitude_distance(self_2844: Scalar, other_2472: HomogeneousMagnitude) -> HomogeneousMagnitude {
    var self_2845: Scalar;
    var other_2473: HomogeneousMagnitude;

    self_2845 = self_2844;
    other_2473 = other_2472;
    let _e4: Scalar = self_2845;
    let _e5: HomogeneousMagnitude = other_2473;
    let _e6: HomogeneousMagnitude = scalar_homogeneous_magnitude_outer_product(_e4, _e5);
    let _e7: Plane = homogeneous_magnitude_attitude(_e6);
    let _e8: Scalar = plane_bulk_norm(_e7);
    let _e9: Scalar = self_2845;
    let _e10: HomogeneousMagnitude = other_2473;
    let _e11: Plane = homogeneous_magnitude_attitude(_e10);
    let _e12: Plane = scalar_plane_outer_product(_e9, _e11);
    let _e13: AntiScalar = plane_weight_norm(_e12);
    let _e14: HomogeneousMagnitude = scalar_anti_scalar_add(_e8, _e13);
    return _e14;
}

fn scalar_line_distance(self_2846: Scalar, other_2474: Line) -> HomogeneousMagnitude {
    var self_2847: Scalar;
    var other_2475: Line;

    self_2847 = self_2846;
    other_2475 = other_2474;
    let _e4: Scalar = self_2847;
    let _e5: Line = other_2475;
    let _e6: Line = scalar_line_outer_product(_e4, _e5);
    let _e7: Point = line_attitude(_e6);
    let _e8: Scalar = point_bulk_norm(_e7);
    let _e9: Scalar = self_2847;
    let _e10: Line = other_2475;
    let _e11: Point = line_attitude(_e10);
    let _e12: Point = scalar_point_outer_product(_e9, _e11);
    let _e13: AntiScalar = point_weight_norm(_e12);
    let _e14: HomogeneousMagnitude = scalar_anti_scalar_add(_e8, _e13);
    return _e14;
}

fn scalar_motor_distance(self_2848: Scalar, other_2476: Motor) -> HomogeneousMagnitude {
    var self_2849: Scalar;
    var other_2477: Motor;

    self_2849 = self_2848;
    other_2477 = other_2476;
    let _e4: Scalar = self_2849;
    let _e5: Motor = other_2477;
    let _e6: Motor = scalar_motor_outer_product(_e4, _e5);
    let _e7: Flector = motor_attitude(_e6);
    let _e8: Scalar = flector_bulk_norm(_e7);
    let _e9: Scalar = self_2849;
    let _e10: Motor = other_2477;
    let _e11: Flector = motor_attitude(_e10);
    let _e12: Flector = scalar_flector_outer_product(_e9, _e11);
    let _e13: AntiScalar = flector_weight_norm(_e12);
    let _e14: HomogeneousMagnitude = scalar_anti_scalar_add(_e8, _e13);
    return _e14;
}

fn scalar_plane_distance(self_2850: Scalar, other_2478: Plane) -> HomogeneousMagnitude {
    var self_2851: Scalar;
    var other_2479: Plane;

    self_2851 = self_2850;
    other_2479 = other_2478;
    let _e4: Scalar = self_2851;
    let _e5: Plane = other_2479;
    let _e6: Plane = scalar_plane_outer_product(_e4, _e5);
    let _e7: Line = plane_attitude(_e6);
    let _e8: Scalar = line_bulk_norm(_e7);
    let _e9: Scalar = self_2851;
    let _e10: Plane = other_2479;
    let _e11: Line = plane_attitude(_e10);
    let _e12: Line = scalar_line_outer_product(_e9, _e11);
    let _e13: AntiScalar = line_weight_norm(_e12);
    let _e14: HomogeneousMagnitude = scalar_anti_scalar_add(_e8, _e13);
    return _e14;
}

fn scalar_rotor_distance(self_2852: Scalar, other_2480: Rotor) -> HomogeneousMagnitude {
    var self_2853: Scalar;
    var other_2481: Rotor;

    self_2853 = self_2852;
    other_2481 = other_2480;
    let _e4: Scalar = self_2853;
    let _e5: Rotor = other_2481;
    let _e6: Rotor = scalar_rotor_outer_product(_e4, _e5);
    let _e7: Flector = rotor_attitude(_e6);
    let _e8: Scalar = flector_bulk_norm(_e7);
    let _e9: Scalar = self_2853;
    let _e10: Rotor = other_2481;
    let _e11: Flector = rotor_attitude(_e10);
    let _e12: Flector = scalar_flector_outer_product(_e9, _e11);
    let _e13: AntiScalar = flector_weight_norm(_e12);
    let _e14: HomogeneousMagnitude = scalar_anti_scalar_add(_e8, _e13);
    return _e14;
}

fn translator_line_distance(self_2854: Translator, other_2482: Line) -> HomogeneousMagnitude {
    var self_2855: Translator;
    var other_2483: Line;

    self_2855 = self_2854;
    other_2483 = other_2482;
    let _e4: Translator = self_2855;
    let _e5: Line = other_2483;
    let _e6: AntiScalar = translator_line_outer_product(_e4, _e5);
    let _e7: Plane = anti_scalar_attitude(_e6);
    let _e8: Scalar = plane_bulk_norm(_e7);
    let _e9: Translator = self_2855;
    let _e10: Line = other_2483;
    let _e11: Point = line_attitude(_e10);
    let _e12: Plane = translator_point_outer_product(_e9, _e11);
    let _e13: AntiScalar = plane_weight_norm(_e12);
    let _e14: HomogeneousMagnitude = scalar_anti_scalar_add(_e8, _e13);
    return _e14;
}

fn translator_motor_distance(self_2856: Translator, other_2484: Motor) -> HomogeneousMagnitude {
    var self_2857: Translator;
    var other_2485: Motor;

    self_2857 = self_2856;
    other_2485 = other_2484;
    let _e4: Translator = self_2857;
    let _e5: Motor = other_2485;
    let _e6: AntiScalar = translator_motor_outer_product(_e4, _e5);
    let _e7: Plane = anti_scalar_attitude(_e6);
    let _e8: Scalar = plane_bulk_norm(_e7);
    let _e9: Translator = self_2857;
    let _e10: Motor = other_2485;
    let _e11: Flector = motor_attitude(_e10);
    let _e12: Plane = translator_flector_outer_product(_e9, _e11);
    let _e13: AntiScalar = plane_weight_norm(_e12);
    let _e14: HomogeneousMagnitude = scalar_anti_scalar_add(_e8, _e13);
    return _e14;
}

fn translator_point_distance(self_2858: Translator, other_2486: Point) -> HomogeneousMagnitude {
    var self_2859: Translator;
    var other_2487: Point;

    self_2859 = self_2858;
    other_2487 = other_2486;
    let _e4: Translator = self_2859;
    let _e5: Point = other_2487;
    let _e6: Plane = translator_point_outer_product(_e4, _e5);
    let _e7: Line = plane_attitude(_e6);
    let _e8: Scalar = line_bulk_norm(_e7);
    let _e9: Translator = self_2859;
    let _e10: Point = other_2487;
    let _e11: Scalar = point_attitude(_e10);
    let _e12: Translator = translator_scalar_outer_product(_e9, _e11);
    let _e13: AntiScalar = translator_weight_norm(_e12);
    let _e14: HomogeneousMagnitude = scalar_anti_scalar_add(_e8, _e13);
    return _e14;
}

fn translator_rotor_distance(self_2860: Translator, other_2488: Rotor) -> HomogeneousMagnitude {
    var self_2861: Translator;
    var other_2489: Rotor;

    self_2861 = self_2860;
    other_2489 = other_2488;
    let _e4: Translator = self_2861;
    let _e5: Rotor = other_2489;
    let _e6: AntiScalar = translator_rotor_outer_product(_e4, _e5);
    let _e7: Plane = anti_scalar_attitude(_e6);
    let _e8: Scalar = plane_bulk_norm(_e7);
    let _e9: Translator = self_2861;
    let _e10: Rotor = other_2489;
    let _e11: Flector = rotor_attitude(_e10);
    let _e12: Plane = translator_flector_outer_product(_e9, _e11);
    let _e13: AntiScalar = plane_weight_norm(_e12);
    let _e14: HomogeneousMagnitude = scalar_anti_scalar_add(_e8, _e13);
    return _e14;
}


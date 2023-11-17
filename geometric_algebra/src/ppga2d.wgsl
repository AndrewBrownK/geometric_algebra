struct Scalar {
    g0_: f32,
}

struct AntiScalar {
    g0_: f32,
}

struct MultiVector {
    g0_: vec4<f32>,
    g1_: vec4<f32>,
}

struct Rotor {
    g0_: vec2<f32>,
}

struct Point {
    g0_: vec3<f32>,
}

struct Motor {
    g0_: vec4<f32>,
}

struct IdealPoint {
    g0_: vec2<f32>,
}

struct Translator {
    g0_: vec3<f32>,
}

struct Plane {
    g0_: vec3<f32>,
}

struct MotorDual {
    g0_: vec4<f32>,
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
    return 3;
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

fn scalar_anti_scalar_geometric_product(self_40: Scalar, other_20: AntiScalar) -> AntiScalar {
    var self_41: Scalar;
    var other_21: AntiScalar;

    self_41 = self_40;
    other_21 = other_20;
    let _e4: Scalar = self_41;
    let _e6: AntiScalar = other_21;
    return AntiScalar((_e4.g0_ * _e6.g0_));
}

fn scalar_anti_scalar_regressive_product(self_42: Scalar, other_22: AntiScalar) -> Scalar {
    var self_43: Scalar;
    var other_23: AntiScalar;

    self_43 = self_42;
    other_23 = other_22;
    let _e4: Scalar = self_43;
    let _e6: AntiScalar = other_23;
    return Scalar((_e4.g0_ * _e6.g0_));
}

fn scalar_anti_scalar_outer_product(self_44: Scalar, other_24: AntiScalar) -> AntiScalar {
    var self_45: Scalar;
    var other_25: AntiScalar;

    self_45 = self_44;
    other_25 = other_24;
    let _e4: Scalar = self_45;
    let _e6: AntiScalar = other_25;
    return AntiScalar((_e4.g0_ * _e6.g0_));
}

fn scalar_anti_scalar_inner_product(self_46: Scalar, other_26: AntiScalar) -> AntiScalar {
    var self_47: Scalar;
    var other_27: AntiScalar;

    self_47 = self_46;
    other_27 = other_26;
    let _e4: Scalar = self_47;
    let _e6: AntiScalar = other_27;
    return AntiScalar((_e4.g0_ * _e6.g0_));
}

fn scalar_anti_scalar_geometric_anti_product(self_48: Scalar, other_28: AntiScalar) -> Scalar {
    var self_49: Scalar;
    var other_29: AntiScalar;

    self_49 = self_48;
    other_29 = other_28;
    let _e4: Scalar = self_49;
    let _e6: AntiScalar = other_29;
    return Scalar((_e4.g0_ * _e6.g0_));
}

fn scalar_anti_scalar_inner_anti_product(self_50: Scalar, other_30: AntiScalar) -> Scalar {
    var self_51: Scalar;
    var other_31: AntiScalar;

    self_51 = self_50;
    other_31 = other_30;
    let _e4: Scalar = self_51;
    let _e6: AntiScalar = other_31;
    return Scalar((_e4.g0_ * _e6.g0_));
}

fn scalar_anti_scalar_left_contraction(self_52: Scalar, other_32: AntiScalar) -> AntiScalar {
    var self_53: Scalar;
    var other_33: AntiScalar;

    self_53 = self_52;
    other_33 = other_32;
    let _e4: Scalar = self_53;
    let _e6: AntiScalar = other_33;
    return AntiScalar((_e4.g0_ * _e6.g0_));
}

fn scalar_anti_scalar_right_anti_contraction(self_54: Scalar, other_34: AntiScalar) -> Scalar {
    var self_55: Scalar;
    var other_35: AntiScalar;

    self_55 = self_54;
    other_35 = other_34;
    let _e4: Scalar = self_55;
    let _e6: AntiScalar = other_35;
    return Scalar((_e4.g0_ * _e6.g0_));
}

fn scalar_multi_vector_add(self_56: Scalar, other_36: MultiVector) -> MultiVector {
    var self_57: Scalar;
    var other_37: MultiVector;

    self_57 = self_56;
    other_37 = other_36;
    let _e4: Scalar = self_57;
    let _e13: MultiVector = other_37;
    let _e16: MultiVector = other_37;
    return MultiVector(((vec4<f32>(_e4.g0_) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) + _e13.g0_), _e16.g1_);
}

fn scalar_multi_vector_sub(self_58: Scalar, other_38: MultiVector) -> MultiVector {
    var self_59: Scalar;
    var other_39: MultiVector;

    self_59 = self_58;
    other_39 = other_38;
    let _e4: Scalar = self_59;
    let _e13: MultiVector = other_39;
    let _e18: MultiVector = other_39;
    return MultiVector(((vec4<f32>(_e4.g0_) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) - _e13.g0_), (vec4<f32>(0.0) - _e18.g1_));
}

fn scalar_multi_vector_geometric_product(self_60: Scalar, other_40: MultiVector) -> MultiVector {
    var self_61: Scalar;
    var other_41: MultiVector;

    self_61 = self_60;
    other_41 = other_40;
    let _e4: Scalar = self_61;
    let _e7: MultiVector = other_41;
    let _e10: Scalar = self_61;
    let _e13: MultiVector = other_41;
    return MultiVector((vec4<f32>(_e4.g0_) * _e7.g0_), (vec4<f32>(_e10.g0_) * _e13.g1_));
}

fn scalar_multi_vector_regressive_product(self_62: Scalar, other_42: MultiVector) -> Scalar {
    var self_63: Scalar;
    var other_43: MultiVector;

    self_63 = self_62;
    other_43 = other_42;
    let _e4: Scalar = self_63;
    let _e6: MultiVector = other_43;
    return Scalar((_e4.g0_ * _e6.g1_.y));
}

fn scalar_multi_vector_outer_product(self_64: Scalar, other_44: MultiVector) -> MultiVector {
    var self_65: Scalar;
    var other_45: MultiVector;

    self_65 = self_64;
    other_45 = other_44;
    let _e4: Scalar = self_65;
    let _e7: MultiVector = other_45;
    let _e10: Scalar = self_65;
    let _e13: MultiVector = other_45;
    return MultiVector((vec4<f32>(_e4.g0_) * _e7.g0_), (vec4<f32>(_e10.g0_) * _e13.g1_));
}

fn scalar_multi_vector_inner_product(self_66: Scalar, other_46: MultiVector) -> MultiVector {
    var self_67: Scalar;
    var other_47: MultiVector;

    self_67 = self_66;
    other_47 = other_46;
    let _e4: Scalar = self_67;
    let _e7: MultiVector = other_47;
    let _e10: Scalar = self_67;
    let _e13: MultiVector = other_47;
    return MultiVector((vec4<f32>(_e4.g0_) * _e7.g0_), (vec4<f32>(_e10.g0_) * _e13.g1_));
}

fn scalar_multi_vector_left_contraction(self_68: Scalar, other_48: MultiVector) -> MultiVector {
    var self_69: Scalar;
    var other_49: MultiVector;

    self_69 = self_68;
    other_49 = other_48;
    let _e4: Scalar = self_69;
    let _e7: MultiVector = other_49;
    let _e10: Scalar = self_69;
    let _e13: MultiVector = other_49;
    return MultiVector((vec4<f32>(_e4.g0_) * _e7.g0_), (vec4<f32>(_e10.g0_) * _e13.g1_));
}

fn scalar_multi_vector_right_contraction(self_70: Scalar, other_50: MultiVector) -> Scalar {
    var self_71: Scalar;
    var other_51: MultiVector;

    self_71 = self_70;
    other_51 = other_50;
    let _e4: Scalar = self_71;
    let _e6: MultiVector = other_51;
    return Scalar((_e4.g0_ * _e6.g0_.x));
}

fn scalar_multi_vector_scalar_product(self_72: Scalar, other_52: MultiVector) -> Scalar {
    var self_73: Scalar;
    var other_53: MultiVector;

    self_73 = self_72;
    other_53 = other_52;
    let _e4: Scalar = self_73;
    let _e6: MultiVector = other_53;
    return Scalar((_e4.g0_ * _e6.g0_.x));
}

fn scalar_rotor_add(self_74: Scalar, other_54: Rotor) -> Rotor {
    var self_75: Scalar;
    var other_55: Rotor;

    self_75 = self_74;
    other_55 = other_54;
    let _e4: Scalar = self_75;
    let _e11: Rotor = other_55;
    return Rotor(((vec2<f32>(_e4.g0_) * vec2<f32>(1.0, 0.0)) + _e11.g0_));
}

fn scalar_rotor_sub(self_76: Scalar, other_56: Rotor) -> Rotor {
    var self_77: Scalar;
    var other_57: Rotor;

    self_77 = self_76;
    other_57 = other_56;
    let _e4: Scalar = self_77;
    let _e11: Rotor = other_57;
    return Rotor(((vec2<f32>(_e4.g0_) * vec2<f32>(1.0, 0.0)) - _e11.g0_));
}

fn scalar_rotor_geometric_product(self_78: Scalar, other_58: Rotor) -> Rotor {
    var self_79: Scalar;
    var other_59: Rotor;

    self_79 = self_78;
    other_59 = other_58;
    let _e4: Scalar = self_79;
    let _e7: Rotor = other_59;
    return Rotor((vec2<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_rotor_outer_product(self_80: Scalar, other_60: Rotor) -> Rotor {
    var self_81: Scalar;
    var other_61: Rotor;

    self_81 = self_80;
    other_61 = other_60;
    let _e4: Scalar = self_81;
    let _e7: Rotor = other_61;
    return Rotor((vec2<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_rotor_inner_product(self_82: Scalar, other_62: Rotor) -> Rotor {
    var self_83: Scalar;
    var other_63: Rotor;

    self_83 = self_82;
    other_63 = other_62;
    let _e4: Scalar = self_83;
    let _e7: Rotor = other_63;
    return Rotor((vec2<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_rotor_left_contraction(self_84: Scalar, other_64: Rotor) -> Rotor {
    var self_85: Scalar;
    var other_65: Rotor;

    self_85 = self_84;
    other_65 = other_64;
    let _e4: Scalar = self_85;
    let _e7: Rotor = other_65;
    return Rotor((vec2<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_rotor_right_contraction(self_86: Scalar, other_66: Rotor) -> Scalar {
    var self_87: Scalar;
    var other_67: Rotor;

    self_87 = self_86;
    other_67 = other_66;
    let _e4: Scalar = self_87;
    let _e6: Rotor = other_67;
    return Scalar((_e4.g0_ * _e6.g0_.x));
}

fn scalar_rotor_scalar_product(self_88: Scalar, other_68: Rotor) -> Scalar {
    var self_89: Scalar;
    var other_69: Rotor;

    self_89 = self_88;
    other_69 = other_68;
    let _e4: Scalar = self_89;
    let _e6: Rotor = other_69;
    return Scalar((_e4.g0_ * _e6.g0_.x));
}

fn scalar_point_add(self_90: Scalar, other_70: Point) -> Motor {
    var self_91: Scalar;
    var other_71: Point;

    self_91 = self_90;
    other_71 = other_70;
    let _e4: Scalar = self_91;
    let _e13: Point = other_71;
    let _e16: Point = other_71;
    let _e19: Point = other_71;
    let _e22: Point = other_71;
    return Motor(((vec4<f32>(_e4.g0_) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) + (vec4<f32>(_e13.g0_.x, _e16.g0_.x, _e19.g0_.y, _e22.g0_.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn scalar_point_sub(self_92: Scalar, other_72: Point) -> Motor {
    var self_93: Scalar;
    var other_73: Point;

    self_93 = self_92;
    other_73 = other_72;
    let _e4: Scalar = self_93;
    let _e13: Point = other_73;
    let _e16: Point = other_73;
    let _e19: Point = other_73;
    let _e22: Point = other_73;
    return Motor(((vec4<f32>(_e4.g0_) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) - (vec4<f32>(_e13.g0_.x, _e16.g0_.x, _e19.g0_.y, _e22.g0_.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn scalar_point_geometric_product(self_94: Scalar, other_74: Point) -> Point {
    var self_95: Scalar;
    var other_75: Point;

    self_95 = self_94;
    other_75 = other_74;
    let _e4: Scalar = self_95;
    let _e7: Point = other_75;
    return Point((vec3<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_point_outer_product(self_96: Scalar, other_76: Point) -> Point {
    var self_97: Scalar;
    var other_77: Point;

    self_97 = self_96;
    other_77 = other_76;
    let _e4: Scalar = self_97;
    let _e7: Point = other_77;
    return Point((vec3<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_point_inner_product(self_98: Scalar, other_78: Point) -> Point {
    var self_99: Scalar;
    var other_79: Point;

    self_99 = self_98;
    other_79 = other_78;
    let _e4: Scalar = self_99;
    let _e7: Point = other_79;
    return Point((vec3<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_point_left_contraction(self_100: Scalar, other_80: Point) -> Point {
    var self_101: Scalar;
    var other_81: Point;

    self_101 = self_100;
    other_81 = other_80;
    let _e4: Scalar = self_101;
    let _e7: Point = other_81;
    return Point((vec3<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_ideal_point_add(self_102: Scalar, other_82: IdealPoint) -> Translator {
    var self_103: Scalar;
    var other_83: IdealPoint;

    self_103 = self_102;
    other_83 = other_82;
    let _e4: Scalar = self_103;
    let _e12: IdealPoint = other_83;
    let _e15: IdealPoint = other_83;
    let _e18: IdealPoint = other_83;
    return Translator(((vec3<f32>(_e4.g0_) * vec3<f32>(1.0, 0.0, 0.0)) + (vec3<f32>(_e12.g0_.x, _e15.g0_.x, _e18.g0_.y) * vec3<f32>(0.0, 1.0, 1.0))));
}

fn scalar_ideal_point_sub(self_104: Scalar, other_84: IdealPoint) -> Translator {
    var self_105: Scalar;
    var other_85: IdealPoint;

    self_105 = self_104;
    other_85 = other_84;
    let _e4: Scalar = self_105;
    let _e12: IdealPoint = other_85;
    let _e15: IdealPoint = other_85;
    let _e18: IdealPoint = other_85;
    return Translator(((vec3<f32>(_e4.g0_) * vec3<f32>(1.0, 0.0, 0.0)) - (vec3<f32>(_e12.g0_.x, _e15.g0_.x, _e18.g0_.y) * vec3<f32>(0.0, 1.0, 1.0))));
}

fn scalar_ideal_point_geometric_product(self_106: Scalar, other_86: IdealPoint) -> IdealPoint {
    var self_107: Scalar;
    var other_87: IdealPoint;

    self_107 = self_106;
    other_87 = other_86;
    let _e4: Scalar = self_107;
    let _e7: IdealPoint = other_87;
    return IdealPoint((vec2<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_ideal_point_outer_product(self_108: Scalar, other_88: IdealPoint) -> IdealPoint {
    var self_109: Scalar;
    var other_89: IdealPoint;

    self_109 = self_108;
    other_89 = other_88;
    let _e4: Scalar = self_109;
    let _e7: IdealPoint = other_89;
    return IdealPoint((vec2<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_ideal_point_inner_product(self_110: Scalar, other_90: IdealPoint) -> IdealPoint {
    var self_111: Scalar;
    var other_91: IdealPoint;

    self_111 = self_110;
    other_91 = other_90;
    let _e4: Scalar = self_111;
    let _e7: IdealPoint = other_91;
    return IdealPoint((vec2<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_ideal_point_left_contraction(self_112: Scalar, other_92: IdealPoint) -> IdealPoint {
    var self_113: Scalar;
    var other_93: IdealPoint;

    self_113 = self_112;
    other_93 = other_92;
    let _e4: Scalar = self_113;
    let _e7: IdealPoint = other_93;
    return IdealPoint((vec2<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_plane_geometric_product(self_114: Scalar, other_94: Plane) -> Plane {
    var self_115: Scalar;
    var other_95: Plane;

    self_115 = self_114;
    other_95 = other_94;
    let _e4: Scalar = self_115;
    let _e7: Plane = other_95;
    return Plane((vec3<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_plane_outer_product(self_116: Scalar, other_96: Plane) -> Plane {
    var self_117: Scalar;
    var other_97: Plane;

    self_117 = self_116;
    other_97 = other_96;
    let _e4: Scalar = self_117;
    let _e7: Plane = other_97;
    return Plane((vec3<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_plane_inner_product(self_118: Scalar, other_98: Plane) -> Plane {
    var self_119: Scalar;
    var other_99: Plane;

    self_119 = self_118;
    other_99 = other_98;
    let _e4: Scalar = self_119;
    let _e7: Plane = other_99;
    return Plane((vec3<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_plane_left_contraction(self_120: Scalar, other_100: Plane) -> Plane {
    var self_121: Scalar;
    var other_101: Plane;

    self_121 = self_120;
    other_101 = other_100;
    let _e4: Scalar = self_121;
    let _e7: Plane = other_101;
    return Plane((vec3<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_translator_add(self_122: Scalar, other_102: Translator) -> Translator {
    var self_123: Scalar;
    var other_103: Translator;

    self_123 = self_122;
    other_103 = other_102;
    let _e4: Scalar = self_123;
    let _e12: Translator = other_103;
    return Translator(((vec3<f32>(_e4.g0_) * vec3<f32>(1.0, 0.0, 0.0)) + _e12.g0_));
}

fn scalar_translator_sub(self_124: Scalar, other_104: Translator) -> Translator {
    var self_125: Scalar;
    var other_105: Translator;

    self_125 = self_124;
    other_105 = other_104;
    let _e4: Scalar = self_125;
    let _e12: Translator = other_105;
    return Translator(((vec3<f32>(_e4.g0_) * vec3<f32>(1.0, 0.0, 0.0)) - _e12.g0_));
}

fn scalar_translator_geometric_product(self_126: Scalar, other_106: Translator) -> Translator {
    var self_127: Scalar;
    var other_107: Translator;

    self_127 = self_126;
    other_107 = other_106;
    let _e4: Scalar = self_127;
    let _e7: Translator = other_107;
    return Translator((vec3<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_translator_outer_product(self_128: Scalar, other_108: Translator) -> Translator {
    var self_129: Scalar;
    var other_109: Translator;

    self_129 = self_128;
    other_109 = other_108;
    let _e4: Scalar = self_129;
    let _e7: Translator = other_109;
    return Translator((vec3<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_translator_inner_product(self_130: Scalar, other_110: Translator) -> Translator {
    var self_131: Scalar;
    var other_111: Translator;

    self_131 = self_130;
    other_111 = other_110;
    let _e4: Scalar = self_131;
    let _e7: Translator = other_111;
    return Translator((vec3<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_translator_left_contraction(self_132: Scalar, other_112: Translator) -> Translator {
    var self_133: Scalar;
    var other_113: Translator;

    self_133 = self_132;
    other_113 = other_112;
    let _e4: Scalar = self_133;
    let _e7: Translator = other_113;
    return Translator((vec3<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_translator_right_contraction(self_134: Scalar, other_114: Translator) -> Scalar {
    var self_135: Scalar;
    var other_115: Translator;

    self_135 = self_134;
    other_115 = other_114;
    let _e4: Scalar = self_135;
    let _e6: Translator = other_115;
    return Scalar((_e4.g0_ * _e6.g0_.x));
}

fn scalar_translator_scalar_product(self_136: Scalar, other_116: Translator) -> Scalar {
    var self_137: Scalar;
    var other_117: Translator;

    self_137 = self_136;
    other_117 = other_116;
    let _e4: Scalar = self_137;
    let _e6: Translator = other_117;
    return Scalar((_e4.g0_ * _e6.g0_.x));
}

fn scalar_motor_add(self_138: Scalar, other_118: Motor) -> Motor {
    var self_139: Scalar;
    var other_119: Motor;

    self_139 = self_138;
    other_119 = other_118;
    let _e4: Scalar = self_139;
    let _e13: Motor = other_119;
    return Motor(((vec4<f32>(_e4.g0_) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) + _e13.g0_));
}

fn scalar_motor_sub(self_140: Scalar, other_120: Motor) -> Motor {
    var self_141: Scalar;
    var other_121: Motor;

    self_141 = self_140;
    other_121 = other_120;
    let _e4: Scalar = self_141;
    let _e13: Motor = other_121;
    return Motor(((vec4<f32>(_e4.g0_) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) - _e13.g0_));
}

fn scalar_motor_geometric_product(self_142: Scalar, other_122: Motor) -> Motor {
    var self_143: Scalar;
    var other_123: Motor;

    self_143 = self_142;
    other_123 = other_122;
    let _e4: Scalar = self_143;
    let _e7: Motor = other_123;
    return Motor((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_motor_outer_product(self_144: Scalar, other_124: Motor) -> Motor {
    var self_145: Scalar;
    var other_125: Motor;

    self_145 = self_144;
    other_125 = other_124;
    let _e4: Scalar = self_145;
    let _e7: Motor = other_125;
    return Motor((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_motor_inner_product(self_146: Scalar, other_126: Motor) -> Motor {
    var self_147: Scalar;
    var other_127: Motor;

    self_147 = self_146;
    other_127 = other_126;
    let _e4: Scalar = self_147;
    let _e7: Motor = other_127;
    return Motor((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_motor_left_contraction(self_148: Scalar, other_128: Motor) -> Motor {
    var self_149: Scalar;
    var other_129: Motor;

    self_149 = self_148;
    other_129 = other_128;
    let _e4: Scalar = self_149;
    let _e7: Motor = other_129;
    return Motor((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_motor_right_contraction(self_150: Scalar, other_130: Motor) -> Scalar {
    var self_151: Scalar;
    var other_131: Motor;

    self_151 = self_150;
    other_131 = other_130;
    let _e4: Scalar = self_151;
    let _e6: Motor = other_131;
    return Scalar((_e4.g0_ * _e6.g0_.x));
}

fn scalar_motor_scalar_product(self_152: Scalar, other_132: Motor) -> Scalar {
    var self_153: Scalar;
    var other_133: Motor;

    self_153 = self_152;
    other_133 = other_132;
    let _e4: Scalar = self_153;
    let _e6: Motor = other_133;
    return Scalar((_e4.g0_ * _e6.g0_.x));
}

fn scalar_motor_dual_geometric_product(self_154: Scalar, other_134: MotorDual) -> MotorDual {
    var self_155: Scalar;
    var other_135: MotorDual;

    self_155 = self_154;
    other_135 = other_134;
    let _e4: Scalar = self_155;
    let _e7: MotorDual = other_135;
    return MotorDual((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_motor_dual_regressive_product(self_156: Scalar, other_136: MotorDual) -> Scalar {
    var self_157: Scalar;
    var other_137: MotorDual;

    self_157 = self_156;
    other_137 = other_136;
    let _e4: Scalar = self_157;
    let _e6: MotorDual = other_137;
    return Scalar((_e4.g0_ * _e6.g0_.x));
}

fn scalar_motor_dual_outer_product(self_158: Scalar, other_138: MotorDual) -> MotorDual {
    var self_159: Scalar;
    var other_139: MotorDual;

    self_159 = self_158;
    other_139 = other_138;
    let _e4: Scalar = self_159;
    let _e7: MotorDual = other_139;
    return MotorDual((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_motor_dual_inner_product(self_160: Scalar, other_140: MotorDual) -> MotorDual {
    var self_161: Scalar;
    var other_141: MotorDual;

    self_161 = self_160;
    other_141 = other_140;
    let _e4: Scalar = self_161;
    let _e7: MotorDual = other_141;
    return MotorDual((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_motor_dual_geometric_anti_product(self_162: Scalar, other_142: MotorDual) -> Rotor {
    var self_163: Scalar;
    var other_143: MotorDual;

    self_163 = self_162;
    other_143 = other_142;
    let _e4: Scalar = self_163;
    let _e7: MotorDual = other_143;
    let _e10: MotorDual = other_143;
    return Rotor(((vec2<f32>(_e4.g0_) * vec2<f32>(_e7.g0_.x, _e10.g0_.y)) * vec2<f32>(1.0, -(1.0))));
}

fn scalar_motor_dual_inner_anti_product(self_164: Scalar, other_144: MotorDual) -> Rotor {
    var self_165: Scalar;
    var other_145: MotorDual;

    self_165 = self_164;
    other_145 = other_144;
    let _e4: Scalar = self_165;
    let _e7: MotorDual = other_145;
    let _e10: MotorDual = other_145;
    return Rotor(((vec2<f32>(_e4.g0_) * vec2<f32>(_e7.g0_.x, _e10.g0_.y)) * vec2<f32>(1.0, -(1.0))));
}

fn scalar_motor_dual_left_contraction(self_166: Scalar, other_146: MotorDual) -> MotorDual {
    var self_167: Scalar;
    var other_147: MotorDual;

    self_167 = self_166;
    other_147 = other_146;
    let _e4: Scalar = self_167;
    let _e7: MotorDual = other_147;
    return MotorDual((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_motor_dual_right_anti_contraction(self_168: Scalar, other_148: MotorDual) -> Rotor {
    var self_169: Scalar;
    var other_149: MotorDual;

    self_169 = self_168;
    other_149 = other_148;
    let _e4: Scalar = self_169;
    let _e7: MotorDual = other_149;
    let _e10: MotorDual = other_149;
    return Rotor(((vec2<f32>(_e4.g0_) * vec2<f32>(_e7.g0_.x, _e10.g0_.y)) * vec2<f32>(1.0, -(1.0))));
}

fn scalar_squared_magnitude(self_170: Scalar) -> Scalar {
    var self_171: Scalar;

    self_171 = self_170;
    let _e2: Scalar = self_171;
    let _e3: Scalar = self_171;
    let _e4: Scalar = scalar_reversal(_e3);
    let _e5: Scalar = scalar_scalar_scalar_product(_e2, _e4);
    return _e5;
}

fn scalar_magnitude(self_172: Scalar) -> Scalar {
    var self_173: Scalar;

    self_173 = self_172;
    let _e2: Scalar = self_173;
    let _e3: Scalar = scalar_squared_magnitude(_e2);
    return Scalar(sqrt(_e3.g0_));
}

fn scalar_scale(self_174: Scalar, other_150: f32) -> Scalar {
    var self_175: Scalar;
    var other_151: f32;

    self_175 = self_174;
    other_151 = other_150;
    let _e4: Scalar = self_175;
    let _e5: f32 = other_151;
    let _e7: Scalar = scalar_scalar_geometric_product(_e4, Scalar(_e5));
    return _e7;
}

fn scalar_signum(self_176: Scalar) -> Scalar {
    var self_177: Scalar;

    self_177 = self_176;
    let _e2: Scalar = self_177;
    let _e3: Scalar = self_177;
    let _e4: Scalar = scalar_magnitude(_e3);
    let _e9: Scalar = scalar_scalar_geometric_product(_e2, Scalar((1.0 / _e4.g0_)));
    return _e9;
}

fn scalar_inverse(self_178: Scalar) -> Scalar {
    var self_179: Scalar;

    self_179 = self_178;
    let _e2: Scalar = self_179;
    let _e3: Scalar = scalar_reversal(_e2);
    let _e4: Scalar = self_179;
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

fn anti_scalar_grade(self_180: AntiScalar) -> i32 {
    return 3;
}

fn anti_scalar_anti_grade(self_181: AntiScalar) -> i32 {
    return 0;
}

fn anti_scalar_neg(self_182: AntiScalar) -> AntiScalar {
    var self_183: AntiScalar;

    self_183 = self_182;
    let _e2: AntiScalar = self_183;
    return AntiScalar((_e2.g0_ * -(1.0)));
}

fn anti_scalar_automorphism(self_184: AntiScalar) -> AntiScalar {
    var self_185: AntiScalar;

    self_185 = self_184;
    let _e2: AntiScalar = self_185;
    return AntiScalar((_e2.g0_ * -(1.0)));
}

fn anti_scalar_reversal(self_186: AntiScalar) -> AntiScalar {
    var self_187: AntiScalar;

    self_187 = self_186;
    let _e2: AntiScalar = self_187;
    return AntiScalar((_e2.g0_ * -(1.0)));
}

fn anti_scalar_conjugation(self_188: AntiScalar) -> AntiScalar {
    var self_189: AntiScalar;

    self_189 = self_188;
    let _e2: AntiScalar = self_189;
    return AntiScalar(_e2.g0_);
}

fn anti_scalar_dual(self_190: AntiScalar) -> Scalar {
    var self_191: AntiScalar;

    self_191 = self_190;
    let _e2: AntiScalar = self_191;
    return Scalar(_e2.g0_);
}

fn anti_scalar_anti_reversal(self_192: AntiScalar) -> AntiScalar {
    var self_193: AntiScalar;

    self_193 = self_192;
    let _e2: AntiScalar = self_193;
    return AntiScalar(_e2.g0_);
}

fn anti_scalar_right_complement(self_194: AntiScalar) -> Scalar {
    var self_195: AntiScalar;

    self_195 = self_194;
    let _e2: AntiScalar = self_195;
    return Scalar(_e2.g0_);
}

fn anti_scalar_left_complement(self_196: AntiScalar) -> Scalar {
    var self_197: AntiScalar;

    self_197 = self_196;
    let _e2: AntiScalar = self_197;
    return Scalar(_e2.g0_);
}

fn anti_scalar_double_complement(self_198: AntiScalar) -> AntiScalar {
    var self_199: AntiScalar;

    self_199 = self_198;
    let _e2: AntiScalar = self_199;
    return AntiScalar(_e2.g0_);
}

fn anti_scalar_scalar_geometric_product(self_200: AntiScalar, other_152: Scalar) -> AntiScalar {
    var self_201: AntiScalar;
    var other_153: Scalar;

    self_201 = self_200;
    other_153 = other_152;
    let _e4: AntiScalar = self_201;
    let _e6: Scalar = other_153;
    return AntiScalar((_e4.g0_ * _e6.g0_));
}

fn anti_scalar_scalar_regressive_product(self_202: AntiScalar, other_154: Scalar) -> Scalar {
    var self_203: AntiScalar;
    var other_155: Scalar;

    self_203 = self_202;
    other_155 = other_154;
    let _e4: AntiScalar = self_203;
    let _e6: Scalar = other_155;
    return Scalar((_e4.g0_ * _e6.g0_));
}

fn anti_scalar_scalar_outer_product(self_204: AntiScalar, other_156: Scalar) -> AntiScalar {
    var self_205: AntiScalar;
    var other_157: Scalar;

    self_205 = self_204;
    other_157 = other_156;
    let _e4: AntiScalar = self_205;
    let _e6: Scalar = other_157;
    return AntiScalar((_e4.g0_ * _e6.g0_));
}

fn anti_scalar_scalar_inner_product(self_206: AntiScalar, other_158: Scalar) -> AntiScalar {
    var self_207: AntiScalar;
    var other_159: Scalar;

    self_207 = self_206;
    other_159 = other_158;
    let _e4: AntiScalar = self_207;
    let _e6: Scalar = other_159;
    return AntiScalar((_e4.g0_ * _e6.g0_));
}

fn anti_scalar_scalar_geometric_anti_product(self_208: AntiScalar, other_160: Scalar) -> Scalar {
    var self_209: AntiScalar;
    var other_161: Scalar;

    self_209 = self_208;
    other_161 = other_160;
    let _e4: AntiScalar = self_209;
    let _e6: Scalar = other_161;
    return Scalar((_e4.g0_ * _e6.g0_));
}

fn anti_scalar_scalar_inner_anti_product(self_210: AntiScalar, other_162: Scalar) -> Scalar {
    var self_211: AntiScalar;
    var other_163: Scalar;

    self_211 = self_210;
    other_163 = other_162;
    let _e4: AntiScalar = self_211;
    let _e6: Scalar = other_163;
    return Scalar((_e4.g0_ * _e6.g0_));
}

fn anti_scalar_scalar_right_contraction(self_212: AntiScalar, other_164: Scalar) -> AntiScalar {
    var self_213: AntiScalar;
    var other_165: Scalar;

    self_213 = self_212;
    other_165 = other_164;
    let _e4: AntiScalar = self_213;
    let _e6: Scalar = other_165;
    return AntiScalar((_e4.g0_ * _e6.g0_));
}

fn anti_scalar_scalar_left_anti_contraction(self_214: AntiScalar, other_166: Scalar) -> Scalar {
    var self_215: AntiScalar;
    var other_167: Scalar;

    self_215 = self_214;
    other_167 = other_166;
    let _e4: AntiScalar = self_215;
    let _e6: Scalar = other_167;
    return Scalar((_e4.g0_ * _e6.g0_));
}

fn anti_scalar_anti_scalar_add(self_216: AntiScalar, other_168: AntiScalar) -> AntiScalar {
    var self_217: AntiScalar;
    var other_169: AntiScalar;

    self_217 = self_216;
    other_169 = other_168;
    let _e4: AntiScalar = self_217;
    let _e6: AntiScalar = other_169;
    return AntiScalar((_e4.g0_ + _e6.g0_));
}

fn anti_scalar_anti_scalar_sub(self_218: AntiScalar, other_170: AntiScalar) -> AntiScalar {
    var self_219: AntiScalar;
    var other_171: AntiScalar;

    self_219 = self_218;
    other_171 = other_170;
    let _e4: AntiScalar = self_219;
    let _e6: AntiScalar = other_171;
    return AntiScalar((_e4.g0_ - _e6.g0_));
}

fn anti_scalar_anti_scalar_mul(self_220: AntiScalar, other_172: AntiScalar) -> AntiScalar {
    var self_221: AntiScalar;
    var other_173: AntiScalar;

    self_221 = self_220;
    other_173 = other_172;
    let _e4: AntiScalar = self_221;
    let _e6: AntiScalar = other_173;
    return AntiScalar((_e4.g0_ * _e6.g0_));
}

fn anti_scalar_anti_scalar_div(self_222: AntiScalar, other_174: AntiScalar) -> AntiScalar {
    var self_223: AntiScalar;
    var other_175: AntiScalar;

    self_223 = self_222;
    other_175 = other_174;
    let _e4: AntiScalar = self_223;
    let _e8: AntiScalar = other_175;
    return AntiScalar((((_e4.g0_ * 1.0) / _e8.g0_) * 1.0));
}

fn anti_scalar_anti_scalar_regressive_product(self_224: AntiScalar, other_176: AntiScalar) -> AntiScalar {
    var self_225: AntiScalar;
    var other_177: AntiScalar;

    self_225 = self_224;
    other_177 = other_176;
    let _e4: AntiScalar = self_225;
    let _e6: AntiScalar = other_177;
    return AntiScalar((_e4.g0_ * _e6.g0_));
}

fn anti_scalar_anti_scalar_geometric_anti_product(self_226: AntiScalar, other_178: AntiScalar) -> AntiScalar {
    var self_227: AntiScalar;
    var other_179: AntiScalar;

    self_227 = self_226;
    other_179 = other_178;
    let _e4: AntiScalar = self_227;
    let _e6: AntiScalar = other_179;
    return AntiScalar((_e4.g0_ * _e6.g0_));
}

fn anti_scalar_anti_scalar_inner_anti_product(self_228: AntiScalar, other_180: AntiScalar) -> AntiScalar {
    var self_229: AntiScalar;
    var other_181: AntiScalar;

    self_229 = self_228;
    other_181 = other_180;
    let _e4: AntiScalar = self_229;
    let _e6: AntiScalar = other_181;
    return AntiScalar((_e4.g0_ * _e6.g0_));
}

fn anti_scalar_anti_scalar_left_anti_contraction(self_230: AntiScalar, other_182: AntiScalar) -> AntiScalar {
    var self_231: AntiScalar;
    var other_183: AntiScalar;

    self_231 = self_230;
    other_183 = other_182;
    let _e4: AntiScalar = self_231;
    let _e6: AntiScalar = other_183;
    return AntiScalar((_e4.g0_ * _e6.g0_));
}

fn anti_scalar_anti_scalar_right_anti_contraction(self_232: AntiScalar, other_184: AntiScalar) -> AntiScalar {
    var self_233: AntiScalar;
    var other_185: AntiScalar;

    self_233 = self_232;
    other_185 = other_184;
    let _e4: AntiScalar = self_233;
    let _e6: AntiScalar = other_185;
    return AntiScalar((_e4.g0_ * _e6.g0_));
}

fn anti_scalar_anti_scalar_anti_scalar_product(self_234: AntiScalar, other_186: AntiScalar) -> AntiScalar {
    var self_235: AntiScalar;
    var other_187: AntiScalar;

    self_235 = self_234;
    other_187 = other_186;
    let _e4: AntiScalar = self_235;
    let _e6: AntiScalar = other_187;
    return AntiScalar((_e4.g0_ * _e6.g0_));
}

fn anti_scalar_multi_vector_add(self_236: AntiScalar, other_188: MultiVector) -> MultiVector {
    var self_237: AntiScalar;
    var other_189: MultiVector;

    self_237 = self_236;
    other_189 = other_188;
    let _e4: MultiVector = other_189;
    let _e6: AntiScalar = self_237;
    let _e15: MultiVector = other_189;
    return MultiVector(_e4.g0_, ((vec4<f32>(_e6.g0_) * vec4<f32>(0.0, 1.0, 0.0, 0.0)) + _e15.g1_));
}

fn anti_scalar_multi_vector_sub(self_238: AntiScalar, other_190: MultiVector) -> MultiVector {
    var self_239: AntiScalar;
    var other_191: MultiVector;

    self_239 = self_238;
    other_191 = other_190;
    let _e6: MultiVector = other_191;
    let _e9: AntiScalar = self_239;
    let _e18: MultiVector = other_191;
    return MultiVector((vec4<f32>(0.0) - _e6.g0_), ((vec4<f32>(_e9.g0_) * vec4<f32>(0.0, 1.0, 0.0, 0.0)) - _e18.g1_));
}

fn anti_scalar_multi_vector_regressive_product(self_240: AntiScalar, other_192: MultiVector) -> MultiVector {
    var self_241: AntiScalar;
    var other_193: MultiVector;

    self_241 = self_240;
    other_193 = other_192;
    let _e4: AntiScalar = self_241;
    let _e7: MultiVector = other_193;
    let _e10: AntiScalar = self_241;
    let _e13: MultiVector = other_193;
    return MultiVector((vec4<f32>(_e4.g0_) * _e7.g0_), (vec4<f32>(_e10.g0_) * _e13.g1_));
}

fn anti_scalar_multi_vector_outer_product(self_242: AntiScalar, other_194: MultiVector) -> AntiScalar {
    var self_243: AntiScalar;
    var other_195: MultiVector;

    self_243 = self_242;
    other_195 = other_194;
    let _e4: AntiScalar = self_243;
    let _e6: MultiVector = other_195;
    return AntiScalar((_e4.g0_ * _e6.g0_.x));
}

fn anti_scalar_multi_vector_geometric_anti_product(self_244: AntiScalar, other_196: MultiVector) -> MultiVector {
    var self_245: AntiScalar;
    var other_197: MultiVector;

    self_245 = self_244;
    other_197 = other_196;
    let _e4: AntiScalar = self_245;
    let _e7: MultiVector = other_197;
    let _e10: AntiScalar = self_245;
    let _e13: MultiVector = other_197;
    return MultiVector((vec4<f32>(_e4.g0_) * _e7.g0_), (vec4<f32>(_e10.g0_) * _e13.g1_));
}

fn anti_scalar_multi_vector_inner_anti_product(self_246: AntiScalar, other_198: MultiVector) -> MultiVector {
    var self_247: AntiScalar;
    var other_199: MultiVector;

    self_247 = self_246;
    other_199 = other_198;
    let _e4: AntiScalar = self_247;
    let _e7: MultiVector = other_199;
    let _e10: AntiScalar = self_247;
    let _e13: MultiVector = other_199;
    return MultiVector((vec4<f32>(_e4.g0_) * _e7.g0_), (vec4<f32>(_e10.g0_) * _e13.g1_));
}

fn anti_scalar_multi_vector_left_anti_contraction(self_248: AntiScalar, other_200: MultiVector) -> MultiVector {
    var self_249: AntiScalar;
    var other_201: MultiVector;

    self_249 = self_248;
    other_201 = other_200;
    let _e4: AntiScalar = self_249;
    let _e7: MultiVector = other_201;
    let _e10: AntiScalar = self_249;
    let _e13: MultiVector = other_201;
    return MultiVector((vec4<f32>(_e4.g0_) * _e7.g0_), (vec4<f32>(_e10.g0_) * _e13.g1_));
}

fn anti_scalar_multi_vector_right_anti_contraction(self_250: AntiScalar, other_202: MultiVector) -> AntiScalar {
    var self_251: AntiScalar;
    var other_203: MultiVector;

    self_251 = self_250;
    other_203 = other_202;
    let _e4: AntiScalar = self_251;
    let _e6: MultiVector = other_203;
    return AntiScalar((_e4.g0_ * _e6.g1_.y));
}

fn anti_scalar_multi_vector_anti_scalar_product(self_252: AntiScalar, other_204: MultiVector) -> AntiScalar {
    var self_253: AntiScalar;
    var other_205: MultiVector;

    self_253 = self_252;
    other_205 = other_204;
    let _e4: AntiScalar = self_253;
    let _e6: MultiVector = other_205;
    return AntiScalar((_e4.g0_ * _e6.g1_.y));
}

fn anti_scalar_rotor_regressive_product(self_254: AntiScalar, other_206: Rotor) -> Rotor {
    var self_255: AntiScalar;
    var other_207: Rotor;

    self_255 = self_254;
    other_207 = other_206;
    let _e4: AntiScalar = self_255;
    let _e7: Rotor = other_207;
    return Rotor((vec2<f32>(_e4.g0_) * _e7.g0_));
}

fn anti_scalar_rotor_outer_product(self_256: AntiScalar, other_208: Rotor) -> AntiScalar {
    var self_257: AntiScalar;
    var other_209: Rotor;

    self_257 = self_256;
    other_209 = other_208;
    let _e4: AntiScalar = self_257;
    let _e6: Rotor = other_209;
    return AntiScalar((_e4.g0_ * _e6.g0_.x));
}

fn anti_scalar_rotor_geometric_anti_product(self_258: AntiScalar, other_210: Rotor) -> Rotor {
    var self_259: AntiScalar;
    var other_211: Rotor;

    self_259 = self_258;
    other_211 = other_210;
    let _e4: AntiScalar = self_259;
    let _e7: Rotor = other_211;
    return Rotor((vec2<f32>(_e4.g0_) * _e7.g0_));
}

fn anti_scalar_rotor_inner_anti_product(self_260: AntiScalar, other_212: Rotor) -> Rotor {
    var self_261: AntiScalar;
    var other_213: Rotor;

    self_261 = self_260;
    other_213 = other_212;
    let _e4: AntiScalar = self_261;
    let _e7: Rotor = other_213;
    return Rotor((vec2<f32>(_e4.g0_) * _e7.g0_));
}

fn anti_scalar_rotor_left_anti_contraction(self_262: AntiScalar, other_214: Rotor) -> Rotor {
    var self_263: AntiScalar;
    var other_215: Rotor;

    self_263 = self_262;
    other_215 = other_214;
    let _e4: AntiScalar = self_263;
    let _e7: Rotor = other_215;
    return Rotor((vec2<f32>(_e4.g0_) * _e7.g0_));
}

fn anti_scalar_point_regressive_product(self_264: AntiScalar, other_216: Point) -> Point {
    var self_265: AntiScalar;
    var other_217: Point;

    self_265 = self_264;
    other_217 = other_216;
    let _e4: AntiScalar = self_265;
    let _e7: Point = other_217;
    return Point((vec3<f32>(_e4.g0_) * _e7.g0_));
}

fn anti_scalar_point_geometric_anti_product(self_266: AntiScalar, other_218: Point) -> Point {
    var self_267: AntiScalar;
    var other_219: Point;

    self_267 = self_266;
    other_219 = other_218;
    let _e4: AntiScalar = self_267;
    let _e7: Point = other_219;
    return Point((vec3<f32>(_e4.g0_) * _e7.g0_));
}

fn anti_scalar_point_inner_anti_product(self_268: AntiScalar, other_220: Point) -> Point {
    var self_269: AntiScalar;
    var other_221: Point;

    self_269 = self_268;
    other_221 = other_220;
    let _e4: AntiScalar = self_269;
    let _e7: Point = other_221;
    return Point((vec3<f32>(_e4.g0_) * _e7.g0_));
}

fn anti_scalar_point_left_anti_contraction(self_270: AntiScalar, other_222: Point) -> Point {
    var self_271: AntiScalar;
    var other_223: Point;

    self_271 = self_270;
    other_223 = other_222;
    let _e4: AntiScalar = self_271;
    let _e7: Point = other_223;
    return Point((vec3<f32>(_e4.g0_) * _e7.g0_));
}

fn anti_scalar_ideal_point_regressive_product(self_272: AntiScalar, other_224: IdealPoint) -> IdealPoint {
    var self_273: AntiScalar;
    var other_225: IdealPoint;

    self_273 = self_272;
    other_225 = other_224;
    let _e4: AntiScalar = self_273;
    let _e7: IdealPoint = other_225;
    return IdealPoint((vec2<f32>(_e4.g0_) * _e7.g0_));
}

fn anti_scalar_ideal_point_geometric_anti_product(self_274: AntiScalar, other_226: IdealPoint) -> IdealPoint {
    var self_275: AntiScalar;
    var other_227: IdealPoint;

    self_275 = self_274;
    other_227 = other_226;
    let _e4: AntiScalar = self_275;
    let _e7: IdealPoint = other_227;
    return IdealPoint((vec2<f32>(_e4.g0_) * _e7.g0_));
}

fn anti_scalar_ideal_point_inner_anti_product(self_276: AntiScalar, other_228: IdealPoint) -> IdealPoint {
    var self_277: AntiScalar;
    var other_229: IdealPoint;

    self_277 = self_276;
    other_229 = other_228;
    let _e4: AntiScalar = self_277;
    let _e7: IdealPoint = other_229;
    return IdealPoint((vec2<f32>(_e4.g0_) * _e7.g0_));
}

fn anti_scalar_ideal_point_left_anti_contraction(self_278: AntiScalar, other_230: IdealPoint) -> IdealPoint {
    var self_279: AntiScalar;
    var other_231: IdealPoint;

    self_279 = self_278;
    other_231 = other_230;
    let _e4: AntiScalar = self_279;
    let _e7: IdealPoint = other_231;
    return IdealPoint((vec2<f32>(_e4.g0_) * _e7.g0_));
}

fn anti_scalar_plane_add(self_280: AntiScalar, other_232: Plane) -> MotorDual {
    var self_281: AntiScalar;
    var other_233: Plane;

    self_281 = self_280;
    other_233 = other_232;
    let _e4: AntiScalar = self_281;
    let _e13: Plane = other_233;
    let _e16: Plane = other_233;
    let _e19: Plane = other_233;
    let _e22: Plane = other_233;
    return MotorDual(((vec4<f32>(_e4.g0_) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) + (vec4<f32>(_e13.g0_.x, _e16.g0_.x, _e19.g0_.y, _e22.g0_.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn anti_scalar_plane_sub(self_282: AntiScalar, other_234: Plane) -> MotorDual {
    var self_283: AntiScalar;
    var other_235: Plane;

    self_283 = self_282;
    other_235 = other_234;
    let _e4: AntiScalar = self_283;
    let _e13: Plane = other_235;
    let _e16: Plane = other_235;
    let _e19: Plane = other_235;
    let _e22: Plane = other_235;
    return MotorDual(((vec4<f32>(_e4.g0_) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) - (vec4<f32>(_e13.g0_.x, _e16.g0_.x, _e19.g0_.y, _e22.g0_.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn anti_scalar_plane_geometric_product(self_284: AntiScalar, other_236: Plane) -> IdealPoint {
    var self_285: AntiScalar;
    var other_237: Plane;

    self_285 = self_284;
    other_237 = other_236;
    let _e4: AntiScalar = self_285;
    let _e7: Plane = other_237;
    let _e10: Plane = other_237;
    return IdealPoint((vec2<f32>(_e4.g0_) * vec2<f32>(_e7.g0_.y, _e10.g0_.z)));
}

fn anti_scalar_plane_regressive_product(self_286: AntiScalar, other_238: Plane) -> Plane {
    var self_287: AntiScalar;
    var other_239: Plane;

    self_287 = self_286;
    other_239 = other_238;
    let _e4: AntiScalar = self_287;
    let _e7: Plane = other_239;
    return Plane((vec3<f32>(_e4.g0_) * _e7.g0_));
}

fn anti_scalar_plane_inner_product(self_288: AntiScalar, other_240: Plane) -> IdealPoint {
    var self_289: AntiScalar;
    var other_241: Plane;

    self_289 = self_288;
    other_241 = other_240;
    let _e4: AntiScalar = self_289;
    let _e7: Plane = other_241;
    let _e10: Plane = other_241;
    return IdealPoint((vec2<f32>(_e4.g0_) * vec2<f32>(_e7.g0_.y, _e10.g0_.z)));
}

fn anti_scalar_plane_geometric_anti_product(self_290: AntiScalar, other_242: Plane) -> Plane {
    var self_291: AntiScalar;
    var other_243: Plane;

    self_291 = self_290;
    other_243 = other_242;
    let _e4: AntiScalar = self_291;
    let _e7: Plane = other_243;
    return Plane((vec3<f32>(_e4.g0_) * _e7.g0_));
}

fn anti_scalar_plane_inner_anti_product(self_292: AntiScalar, other_244: Plane) -> Plane {
    var self_293: AntiScalar;
    var other_245: Plane;

    self_293 = self_292;
    other_245 = other_244;
    let _e4: AntiScalar = self_293;
    let _e7: Plane = other_245;
    return Plane((vec3<f32>(_e4.g0_) * _e7.g0_));
}

fn anti_scalar_plane_right_contraction(self_294: AntiScalar, other_246: Plane) -> IdealPoint {
    var self_295: AntiScalar;
    var other_247: Plane;

    self_295 = self_294;
    other_247 = other_246;
    let _e4: AntiScalar = self_295;
    let _e7: Plane = other_247;
    let _e10: Plane = other_247;
    return IdealPoint((vec2<f32>(_e4.g0_) * vec2<f32>(_e7.g0_.y, _e10.g0_.z)));
}

fn anti_scalar_plane_left_anti_contraction(self_296: AntiScalar, other_248: Plane) -> Plane {
    var self_297: AntiScalar;
    var other_249: Plane;

    self_297 = self_296;
    other_249 = other_248;
    let _e4: AntiScalar = self_297;
    let _e7: Plane = other_249;
    return Plane((vec3<f32>(_e4.g0_) * _e7.g0_));
}

fn anti_scalar_translator_geometric_product(self_298: AntiScalar, other_250: Translator) -> AntiScalar {
    var self_299: AntiScalar;
    var other_251: Translator;

    self_299 = self_298;
    other_251 = other_250;
    let _e4: AntiScalar = self_299;
    let _e6: Translator = other_251;
    return AntiScalar((_e4.g0_ * _e6.g0_.x));
}

fn anti_scalar_translator_regressive_product(self_300: AntiScalar, other_252: Translator) -> Translator {
    var self_301: AntiScalar;
    var other_253: Translator;

    self_301 = self_300;
    other_253 = other_252;
    let _e4: AntiScalar = self_301;
    let _e7: Translator = other_253;
    return Translator((vec3<f32>(_e4.g0_) * _e7.g0_));
}

fn anti_scalar_translator_outer_product(self_302: AntiScalar, other_254: Translator) -> AntiScalar {
    var self_303: AntiScalar;
    var other_255: Translator;

    self_303 = self_302;
    other_255 = other_254;
    let _e4: AntiScalar = self_303;
    let _e6: Translator = other_255;
    return AntiScalar((_e4.g0_ * _e6.g0_.x));
}

fn anti_scalar_translator_inner_product(self_304: AntiScalar, other_256: Translator) -> AntiScalar {
    var self_305: AntiScalar;
    var other_257: Translator;

    self_305 = self_304;
    other_257 = other_256;
    let _e4: AntiScalar = self_305;
    let _e6: Translator = other_257;
    return AntiScalar((_e4.g0_ * _e6.g0_.x));
}

fn anti_scalar_translator_geometric_anti_product(self_306: AntiScalar, other_258: Translator) -> Translator {
    var self_307: AntiScalar;
    var other_259: Translator;

    self_307 = self_306;
    other_259 = other_258;
    let _e4: AntiScalar = self_307;
    let _e7: Translator = other_259;
    return Translator((vec3<f32>(_e4.g0_) * _e7.g0_));
}

fn anti_scalar_translator_inner_anti_product(self_308: AntiScalar, other_260: Translator) -> Translator {
    var self_309: AntiScalar;
    var other_261: Translator;

    self_309 = self_308;
    other_261 = other_260;
    let _e4: AntiScalar = self_309;
    let _e7: Translator = other_261;
    return Translator((vec3<f32>(_e4.g0_) * _e7.g0_));
}

fn anti_scalar_translator_right_contraction(self_310: AntiScalar, other_262: Translator) -> AntiScalar {
    var self_311: AntiScalar;
    var other_263: Translator;

    self_311 = self_310;
    other_263 = other_262;
    let _e4: AntiScalar = self_311;
    let _e6: Translator = other_263;
    return AntiScalar((_e4.g0_ * _e6.g0_.x));
}

fn anti_scalar_translator_left_anti_contraction(self_312: AntiScalar, other_264: Translator) -> Translator {
    var self_313: AntiScalar;
    var other_265: Translator;

    self_313 = self_312;
    other_265 = other_264;
    let _e4: AntiScalar = self_313;
    let _e7: Translator = other_265;
    return Translator((vec3<f32>(_e4.g0_) * _e7.g0_));
}

fn anti_scalar_motor_regressive_product(self_314: AntiScalar, other_266: Motor) -> Motor {
    var self_315: AntiScalar;
    var other_267: Motor;

    self_315 = self_314;
    other_267 = other_266;
    let _e4: AntiScalar = self_315;
    let _e7: Motor = other_267;
    return Motor((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn anti_scalar_motor_outer_product(self_316: AntiScalar, other_268: Motor) -> AntiScalar {
    var self_317: AntiScalar;
    var other_269: Motor;

    self_317 = self_316;
    other_269 = other_268;
    let _e4: AntiScalar = self_317;
    let _e6: Motor = other_269;
    return AntiScalar((_e4.g0_ * _e6.g0_.x));
}

fn anti_scalar_motor_geometric_anti_product(self_318: AntiScalar, other_270: Motor) -> Motor {
    var self_319: AntiScalar;
    var other_271: Motor;

    self_319 = self_318;
    other_271 = other_270;
    let _e4: AntiScalar = self_319;
    let _e7: Motor = other_271;
    return Motor((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn anti_scalar_motor_inner_anti_product(self_320: AntiScalar, other_272: Motor) -> Motor {
    var self_321: AntiScalar;
    var other_273: Motor;

    self_321 = self_320;
    other_273 = other_272;
    let _e4: AntiScalar = self_321;
    let _e7: Motor = other_273;
    return Motor((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn anti_scalar_motor_left_anti_contraction(self_322: AntiScalar, other_274: Motor) -> Motor {
    var self_323: AntiScalar;
    var other_275: Motor;

    self_323 = self_322;
    other_275 = other_274;
    let _e4: AntiScalar = self_323;
    let _e7: Motor = other_275;
    return Motor((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn anti_scalar_motor_dual_add(self_324: AntiScalar, other_276: MotorDual) -> MotorDual {
    var self_325: AntiScalar;
    var other_277: MotorDual;

    self_325 = self_324;
    other_277 = other_276;
    let _e4: AntiScalar = self_325;
    let _e13: MotorDual = other_277;
    return MotorDual(((vec4<f32>(_e4.g0_) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) + _e13.g0_));
}

fn anti_scalar_motor_dual_sub(self_326: AntiScalar, other_278: MotorDual) -> MotorDual {
    var self_327: AntiScalar;
    var other_279: MotorDual;

    self_327 = self_326;
    other_279 = other_278;
    let _e4: AntiScalar = self_327;
    let _e13: MotorDual = other_279;
    return MotorDual(((vec4<f32>(_e4.g0_) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) - _e13.g0_));
}

fn anti_scalar_motor_dual_geometric_product(self_328: AntiScalar, other_280: MotorDual) -> IdealPoint {
    var self_329: AntiScalar;
    var other_281: MotorDual;

    self_329 = self_328;
    other_281 = other_280;
    let _e4: AntiScalar = self_329;
    let _e7: MotorDual = other_281;
    let _e10: MotorDual = other_281;
    return IdealPoint((vec2<f32>(_e4.g0_) * vec2<f32>(_e7.g0_.z, _e10.g0_.w)));
}

fn anti_scalar_motor_dual_regressive_product(self_330: AntiScalar, other_282: MotorDual) -> MotorDual {
    var self_331: AntiScalar;
    var other_283: MotorDual;

    self_331 = self_330;
    other_283 = other_282;
    let _e4: AntiScalar = self_331;
    let _e7: MotorDual = other_283;
    return MotorDual((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn anti_scalar_motor_dual_inner_product(self_332: AntiScalar, other_284: MotorDual) -> IdealPoint {
    var self_333: AntiScalar;
    var other_285: MotorDual;

    self_333 = self_332;
    other_285 = other_284;
    let _e4: AntiScalar = self_333;
    let _e7: MotorDual = other_285;
    let _e10: MotorDual = other_285;
    return IdealPoint((vec2<f32>(_e4.g0_) * vec2<f32>(_e7.g0_.z, _e10.g0_.w)));
}

fn anti_scalar_motor_dual_geometric_anti_product(self_334: AntiScalar, other_286: MotorDual) -> MotorDual {
    var self_335: AntiScalar;
    var other_287: MotorDual;

    self_335 = self_334;
    other_287 = other_286;
    let _e4: AntiScalar = self_335;
    let _e7: MotorDual = other_287;
    return MotorDual((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn anti_scalar_motor_dual_inner_anti_product(self_336: AntiScalar, other_288: MotorDual) -> MotorDual {
    var self_337: AntiScalar;
    var other_289: MotorDual;

    self_337 = self_336;
    other_289 = other_288;
    let _e4: AntiScalar = self_337;
    let _e7: MotorDual = other_289;
    return MotorDual((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn anti_scalar_motor_dual_right_contraction(self_338: AntiScalar, other_290: MotorDual) -> IdealPoint {
    var self_339: AntiScalar;
    var other_291: MotorDual;

    self_339 = self_338;
    other_291 = other_290;
    let _e4: AntiScalar = self_339;
    let _e7: MotorDual = other_291;
    let _e10: MotorDual = other_291;
    return IdealPoint((vec2<f32>(_e4.g0_) * vec2<f32>(_e7.g0_.z, _e10.g0_.w)));
}

fn anti_scalar_motor_dual_left_anti_contraction(self_340: AntiScalar, other_292: MotorDual) -> MotorDual {
    var self_341: AntiScalar;
    var other_293: MotorDual;

    self_341 = self_340;
    other_293 = other_292;
    let _e4: AntiScalar = self_341;
    let _e7: MotorDual = other_293;
    return MotorDual((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn anti_scalar_motor_dual_right_anti_contraction(self_342: AntiScalar, other_294: MotorDual) -> AntiScalar {
    var self_343: AntiScalar;
    var other_295: MotorDual;

    self_343 = self_342;
    other_295 = other_294;
    let _e4: AntiScalar = self_343;
    let _e6: MotorDual = other_295;
    return AntiScalar((_e4.g0_ * _e6.g0_.x));
}

fn anti_scalar_motor_dual_anti_scalar_product(self_344: AntiScalar, other_296: MotorDual) -> AntiScalar {
    var self_345: AntiScalar;
    var other_297: MotorDual;

    self_345 = self_344;
    other_297 = other_296;
    let _e4: AntiScalar = self_345;
    let _e6: MotorDual = other_297;
    return AntiScalar((_e4.g0_ * _e6.g0_.x));
}

fn anti_scalar_scale(self_346: AntiScalar, other_298: f32) -> AntiScalar {
    var self_347: AntiScalar;
    var other_299: f32;

    self_347 = self_346;
    other_299 = other_298;
    let _e4: AntiScalar = self_347;
    let _e5: f32 = other_299;
    let _e7: AntiScalar = anti_scalar_scalar_geometric_product(_e4, Scalar(_e5));
    return _e7;
}

fn multi_vector_zero() -> MultiVector {
    return MultiVector(vec4<f32>(0.0), vec4<f32>(0.0));
}

fn multi_vector_one() -> MultiVector {
    return MultiVector(vec4<f32>(1.0, 0.0, 0.0, 0.0), vec4<f32>(0.0));
}

fn multi_vector_neg(self_348: MultiVector) -> MultiVector {
    var self_349: MultiVector;

    self_349 = self_348;
    let _e2: MultiVector = self_349;
    let _e8: MultiVector = self_349;
    return MultiVector((_e2.g0_ * vec4<f32>(-(1.0))), (_e8.g1_ * vec4<f32>(-(1.0))));
}

fn multi_vector_automorphism(self_350: MultiVector) -> MultiVector {
    var self_351: MultiVector;

    self_351 = self_350;
    let _e2: MultiVector = self_351;
    let _e12: MultiVector = self_351;
    return MultiVector((_e2.g0_ * vec4<f32>(1.0, 1.0, -(1.0), -(1.0))), (_e12.g1_ * vec4<f32>(-(1.0), -(1.0), 1.0, 1.0)));
}

fn multi_vector_reversal(self_352: MultiVector) -> MultiVector {
    var self_353: MultiVector;

    self_353 = self_352;
    let _e2: MultiVector = self_353;
    let _e11: MultiVector = self_353;
    return MultiVector((_e2.g0_ * vec4<f32>(1.0, -(1.0), 1.0, 1.0)), (_e11.g1_ * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))));
}

fn multi_vector_conjugation(self_354: MultiVector) -> MultiVector {
    var self_355: MultiVector;

    self_355 = self_354;
    let _e2: MultiVector = self_355;
    let _e13: MultiVector = self_355;
    return MultiVector((_e2.g0_ * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))), (_e13.g1_ * vec4<f32>(-(1.0), 1.0, -(1.0), -(1.0))));
}

fn multi_vector_dual(self_356: MultiVector) -> MultiVector {
    var self_357: MultiVector;

    self_357 = self_356;
    let _e2: MultiVector = self_357;
    let _e5: MultiVector = self_357;
    return MultiVector(_e2.g1_.yxwz, _e5.g0_.yxwz);
}

fn multi_vector_anti_reversal(self_358: MultiVector) -> MultiVector {
    var self_359: MultiVector;

    self_359 = self_358;
    let _e2: MultiVector = self_359;
    let _e13: MultiVector = self_359;
    return MultiVector((_e2.g0_ * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))), (_e13.g1_ * vec4<f32>(-(1.0), 1.0, -(1.0), -(1.0))));
}

fn multi_vector_right_complement(self_360: MultiVector) -> MultiVector {
    var self_361: MultiVector;

    self_361 = self_360;
    let _e2: MultiVector = self_361;
    let _e5: MultiVector = self_361;
    return MultiVector(_e2.g1_.yxwz, _e5.g0_.yxwz);
}

fn multi_vector_left_complement(self_362: MultiVector) -> MultiVector {
    var self_363: MultiVector;

    self_363 = self_362;
    let _e2: MultiVector = self_363;
    let _e5: MultiVector = self_363;
    return MultiVector(_e2.g1_.yxwz, _e5.g0_.yxwz);
}

fn multi_vector_double_complement(self_364: MultiVector) -> MultiVector {
    var self_365: MultiVector;

    self_365 = self_364;
    let _e2: MultiVector = self_365;
    let _e4: MultiVector = self_365;
    return MultiVector(_e2.g0_, _e4.g1_);
}

fn multi_vector_scalar_into(self_366: MultiVector) -> Scalar {
    var self_367: MultiVector;

    self_367 = self_366;
    let _e2: MultiVector = self_367;
    return Scalar(_e2.g0_.x);
}

fn multi_vector_scalar_add(self_368: MultiVector, other_300: Scalar) -> MultiVector {
    var self_369: MultiVector;
    var other_301: Scalar;

    self_369 = self_368;
    other_301 = other_300;
    let _e4: MultiVector = self_369;
    let _e6: Scalar = other_301;
    let _e16: MultiVector = self_369;
    return MultiVector((_e4.g0_ + (vec4<f32>(_e6.g0_) * vec4<f32>(1.0, 0.0, 0.0, 0.0))), _e16.g1_);
}

fn multi_vector_scalar_sub(self_370: MultiVector, other_302: Scalar) -> MultiVector {
    var self_371: MultiVector;
    var other_303: Scalar;

    self_371 = self_370;
    other_303 = other_302;
    let _e4: MultiVector = self_371;
    let _e6: Scalar = other_303;
    let _e16: MultiVector = self_371;
    return MultiVector((_e4.g0_ - (vec4<f32>(_e6.g0_) * vec4<f32>(1.0, 0.0, 0.0, 0.0))), _e16.g1_);
}

fn multi_vector_scalar_geometric_product(self_372: MultiVector, other_304: Scalar) -> MultiVector {
    var self_373: MultiVector;
    var other_305: Scalar;

    self_373 = self_372;
    other_305 = other_304;
    let _e4: MultiVector = self_373;
    let _e6: Scalar = other_305;
    let _e10: MultiVector = self_373;
    let _e12: Scalar = other_305;
    return MultiVector((_e4.g0_ * vec4<f32>(_e6.g0_)), (_e10.g1_ * vec4<f32>(_e12.g0_)));
}

fn multi_vector_scalar_regressive_product(self_374: MultiVector, other_306: Scalar) -> Scalar {
    var self_375: MultiVector;
    var other_307: Scalar;

    self_375 = self_374;
    other_307 = other_306;
    let _e4: MultiVector = self_375;
    let _e7: Scalar = other_307;
    return Scalar((_e4.g1_.y * _e7.g0_));
}

fn multi_vector_scalar_outer_product(self_376: MultiVector, other_308: Scalar) -> MultiVector {
    var self_377: MultiVector;
    var other_309: Scalar;

    self_377 = self_376;
    other_309 = other_308;
    let _e4: MultiVector = self_377;
    let _e6: Scalar = other_309;
    let _e10: MultiVector = self_377;
    let _e12: Scalar = other_309;
    return MultiVector((_e4.g0_ * vec4<f32>(_e6.g0_)), (_e10.g1_ * vec4<f32>(_e12.g0_)));
}

fn multi_vector_scalar_inner_product(self_378: MultiVector, other_310: Scalar) -> MultiVector {
    var self_379: MultiVector;
    var other_311: Scalar;

    self_379 = self_378;
    other_311 = other_310;
    let _e4: MultiVector = self_379;
    let _e6: Scalar = other_311;
    let _e10: MultiVector = self_379;
    let _e12: Scalar = other_311;
    return MultiVector((_e4.g0_ * vec4<f32>(_e6.g0_)), (_e10.g1_ * vec4<f32>(_e12.g0_)));
}

fn multi_vector_scalar_left_contraction(self_380: MultiVector, other_312: Scalar) -> Scalar {
    var self_381: MultiVector;
    var other_313: Scalar;

    self_381 = self_380;
    other_313 = other_312;
    let _e4: MultiVector = self_381;
    let _e7: Scalar = other_313;
    return Scalar((_e4.g0_.x * _e7.g0_));
}

fn multi_vector_scalar_right_contraction(self_382: MultiVector, other_314: Scalar) -> MultiVector {
    var self_383: MultiVector;
    var other_315: Scalar;

    self_383 = self_382;
    other_315 = other_314;
    let _e4: MultiVector = self_383;
    let _e6: Scalar = other_315;
    let _e10: MultiVector = self_383;
    let _e12: Scalar = other_315;
    return MultiVector((_e4.g0_ * vec4<f32>(_e6.g0_)), (_e10.g1_ * vec4<f32>(_e12.g0_)));
}

fn multi_vector_scalar_scalar_product(self_384: MultiVector, other_316: Scalar) -> Scalar {
    var self_385: MultiVector;
    var other_317: Scalar;

    self_385 = self_384;
    other_317 = other_316;
    let _e4: MultiVector = self_385;
    let _e7: Scalar = other_317;
    return Scalar((_e4.g0_.x * _e7.g0_));
}

fn multi_vector_anti_scalar_into(self_386: MultiVector) -> AntiScalar {
    var self_387: MultiVector;

    self_387 = self_386;
    let _e2: MultiVector = self_387;
    return AntiScalar(_e2.g1_.y);
}

fn multi_vector_anti_scalar_add(self_388: MultiVector, other_318: AntiScalar) -> MultiVector {
    var self_389: MultiVector;
    var other_319: AntiScalar;

    self_389 = self_388;
    other_319 = other_318;
    let _e4: MultiVector = self_389;
    let _e6: MultiVector = self_389;
    let _e8: AntiScalar = other_319;
    return MultiVector(_e4.g0_, (_e6.g1_ + (vec4<f32>(_e8.g0_) * vec4<f32>(0.0, 1.0, 0.0, 0.0))));
}

fn multi_vector_anti_scalar_sub(self_390: MultiVector, other_320: AntiScalar) -> MultiVector {
    var self_391: MultiVector;
    var other_321: AntiScalar;

    self_391 = self_390;
    other_321 = other_320;
    let _e4: MultiVector = self_391;
    let _e6: MultiVector = self_391;
    let _e8: AntiScalar = other_321;
    return MultiVector(_e4.g0_, (_e6.g1_ - (vec4<f32>(_e8.g0_) * vec4<f32>(0.0, 1.0, 0.0, 0.0))));
}

fn multi_vector_anti_scalar_regressive_product(self_392: MultiVector, other_322: AntiScalar) -> MultiVector {
    var self_393: MultiVector;
    var other_323: AntiScalar;

    self_393 = self_392;
    other_323 = other_322;
    let _e4: MultiVector = self_393;
    let _e6: AntiScalar = other_323;
    let _e10: MultiVector = self_393;
    let _e12: AntiScalar = other_323;
    return MultiVector((_e4.g0_ * vec4<f32>(_e6.g0_)), (_e10.g1_ * vec4<f32>(_e12.g0_)));
}

fn multi_vector_anti_scalar_outer_product(self_394: MultiVector, other_324: AntiScalar) -> AntiScalar {
    var self_395: MultiVector;
    var other_325: AntiScalar;

    self_395 = self_394;
    other_325 = other_324;
    let _e4: MultiVector = self_395;
    let _e7: AntiScalar = other_325;
    return AntiScalar((_e4.g0_.x * _e7.g0_));
}

fn multi_vector_anti_scalar_geometric_anti_product(self_396: MultiVector, other_326: AntiScalar) -> MultiVector {
    var self_397: MultiVector;
    var other_327: AntiScalar;

    self_397 = self_396;
    other_327 = other_326;
    let _e4: MultiVector = self_397;
    let _e6: AntiScalar = other_327;
    let _e10: MultiVector = self_397;
    let _e12: AntiScalar = other_327;
    return MultiVector((_e4.g0_ * vec4<f32>(_e6.g0_)), (_e10.g1_ * vec4<f32>(_e12.g0_)));
}

fn multi_vector_anti_scalar_inner_anti_product(self_398: MultiVector, other_328: AntiScalar) -> MultiVector {
    var self_399: MultiVector;
    var other_329: AntiScalar;

    self_399 = self_398;
    other_329 = other_328;
    let _e4: MultiVector = self_399;
    let _e6: AntiScalar = other_329;
    let _e10: MultiVector = self_399;
    let _e12: AntiScalar = other_329;
    return MultiVector((_e4.g0_ * vec4<f32>(_e6.g0_)), (_e10.g1_ * vec4<f32>(_e12.g0_)));
}

fn multi_vector_anti_scalar_left_anti_contraction(self_400: MultiVector, other_330: AntiScalar) -> AntiScalar {
    var self_401: MultiVector;
    var other_331: AntiScalar;

    self_401 = self_400;
    other_331 = other_330;
    let _e4: MultiVector = self_401;
    let _e7: AntiScalar = other_331;
    return AntiScalar((_e4.g1_.y * _e7.g0_));
}

fn multi_vector_anti_scalar_right_anti_contraction(self_402: MultiVector, other_332: AntiScalar) -> MultiVector {
    var self_403: MultiVector;
    var other_333: AntiScalar;

    self_403 = self_402;
    other_333 = other_332;
    let _e4: MultiVector = self_403;
    let _e6: AntiScalar = other_333;
    let _e10: MultiVector = self_403;
    let _e12: AntiScalar = other_333;
    return MultiVector((_e4.g0_ * vec4<f32>(_e6.g0_)), (_e10.g1_ * vec4<f32>(_e12.g0_)));
}

fn multi_vector_anti_scalar_anti_scalar_product(self_404: MultiVector, other_334: AntiScalar) -> AntiScalar {
    var self_405: MultiVector;
    var other_335: AntiScalar;

    self_405 = self_404;
    other_335 = other_334;
    let _e4: MultiVector = self_405;
    let _e7: AntiScalar = other_335;
    return AntiScalar((_e4.g1_.y * _e7.g0_));
}

fn multi_vector_multi_vector_add(self_406: MultiVector, other_336: MultiVector) -> MultiVector {
    var self_407: MultiVector;
    var other_337: MultiVector;

    self_407 = self_406;
    other_337 = other_336;
    let _e4: MultiVector = self_407;
    let _e6: MultiVector = other_337;
    let _e9: MultiVector = self_407;
    let _e11: MultiVector = other_337;
    return MultiVector((_e4.g0_ + _e6.g0_), (_e9.g1_ + _e11.g1_));
}

fn multi_vector_multi_vector_sub(self_408: MultiVector, other_338: MultiVector) -> MultiVector {
    var self_409: MultiVector;
    var other_339: MultiVector;

    self_409 = self_408;
    other_339 = other_338;
    let _e4: MultiVector = self_409;
    let _e6: MultiVector = other_339;
    let _e9: MultiVector = self_409;
    let _e11: MultiVector = other_339;
    return MultiVector((_e4.g0_ - _e6.g0_), (_e9.g1_ - _e11.g1_));
}

fn multi_vector_multi_vector_mul(self_410: MultiVector, other_340: MultiVector) -> MultiVector {
    var self_411: MultiVector;
    var other_341: MultiVector;

    self_411 = self_410;
    other_341 = other_340;
    let _e4: MultiVector = self_411;
    let _e6: MultiVector = other_341;
    let _e9: MultiVector = self_411;
    let _e11: MultiVector = other_341;
    return MultiVector((_e4.g0_ * _e6.g0_), (_e9.g1_ * _e11.g1_));
}

fn multi_vector_multi_vector_div(self_412: MultiVector, other_342: MultiVector) -> MultiVector {
    var self_413: MultiVector;
    var other_343: MultiVector;

    self_413 = self_412;
    other_343 = other_342;
    let _e4: MultiVector = self_413;
    let _e7: MultiVector = self_413;
    let _e10: MultiVector = self_413;
    let _e13: MultiVector = self_413;
    let _e23: MultiVector = other_343;
    let _e26: MultiVector = other_343;
    let _e29: MultiVector = other_343;
    let _e32: MultiVector = other_343;
    let _e43: MultiVector = self_413;
    let _e46: MultiVector = self_413;
    let _e49: MultiVector = self_413;
    let _e52: MultiVector = self_413;
    let _e62: MultiVector = other_343;
    let _e65: MultiVector = other_343;
    let _e68: MultiVector = other_343;
    let _e71: MultiVector = other_343;
    return MultiVector((((vec4<f32>(_e4.g0_.x, _e7.g0_.y, _e10.g0_.z, _e13.g0_.w) * vec4<f32>(1.0, 1.0, 1.0, 1.0)) / vec4<f32>(_e23.g0_.x, _e26.g0_.y, _e29.g0_.z, _e32.g0_.w)) * vec4<f32>(1.0, 1.0, 1.0, 1.0)), (((vec4<f32>(_e43.g1_.x, _e46.g1_.y, _e49.g1_.z, _e52.g1_.w) * vec4<f32>(1.0, 1.0, 1.0, 1.0)) / vec4<f32>(_e62.g1_.x, _e65.g1_.y, _e68.g1_.z, _e71.g1_.w)) * vec4<f32>(1.0, 1.0, 1.0, 1.0)));
}

fn multi_vector_multi_vector_geometric_product(self_414: MultiVector, other_344: MultiVector) -> MultiVector {
    var self_415: MultiVector;
    var other_345: MultiVector;

    self_415 = self_414;
    other_345 = other_344;
    let _e4: MultiVector = self_415;
    let _e8: MultiVector = other_345;
    let _e11: MultiVector = self_415;
    let _e15: MultiVector = other_345;
    let _e28: MultiVector = self_415;
    let _e32: MultiVector = other_345;
    let _e37: MultiVector = self_415;
    let _e41: MultiVector = other_345;
    let _e54: MultiVector = self_415;
    let _e58: MultiVector = other_345;
    let _e61: MultiVector = self_415;
    let _e65: MultiVector = other_345;
    let _e78: MultiVector = self_415;
    let _e82: MultiVector = other_345;
    let _e95: MultiVector = self_415;
    let _e99: MultiVector = other_345;
    let _e104: MultiVector = self_415;
    let _e108: MultiVector = other_345;
    let _e119: MultiVector = self_415;
    let _e123: MultiVector = other_345;
    let _e135: MultiVector = self_415;
    let _e139: MultiVector = other_345;
    let _e151: MultiVector = self_415;
    let _e155: MultiVector = other_345;
    return MultiVector(((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.y) * _e15.g0_.yxwz) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + (vec4<f32>(_e28.g0_.z) * _e32.g0_.zwxy)) + ((vec4<f32>(_e37.g0_.w) * _e41.g0_.wzyx) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0))), ((((((((vec4<f32>(_e54.g0_.x) * _e58.g1_) + ((vec4<f32>(_e61.g0_.y) * _e65.g1_.yxwz) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))) + ((vec4<f32>(_e78.g0_.z) * _e82.g1_.zwxy) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))) + (vec4<f32>(_e95.g0_.w) * _e99.g1_.wzyx)) + ((vec4<f32>(_e104.g1_.x) * _e108.g0_) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e119.g1_.y) * _e123.g0_.yxwz) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0))) + ((vec4<f32>(_e135.g1_.z) * _e139.g0_.zwxy) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e151.g1_.w) * _e155.g0_.wzyx) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0))));
}

fn multi_vector_multi_vector_regressive_product(self_416: MultiVector, other_346: MultiVector) -> MultiVector {
    var self_417: MultiVector;
    var other_347: MultiVector;

    self_417 = self_416;
    other_347 = other_346;
    let _e4: MultiVector = self_417;
    let _e8: MultiVector = other_347;
    let _e18: MultiVector = self_417;
    let _e22: MultiVector = other_347;
    let _e33: MultiVector = self_417;
    let _e37: MultiVector = other_347;
    let _e48: MultiVector = self_417;
    let _e52: MultiVector = other_347;
    let _e64: MultiVector = self_417;
    let _e68: MultiVector = other_347;
    let _e72: MultiVector = self_417;
    let _e76: MultiVector = other_347;
    let _e87: MultiVector = self_417;
    let _e91: MultiVector = other_347;
    let _e103: MultiVector = self_417;
    let _e107: MultiVector = other_347;
    let _e118: MultiVector = self_417;
    let _e122: MultiVector = other_347;
    let _e125: MultiVector = self_417;
    let _e129: MultiVector = other_347;
    let _e141: MultiVector = self_417;
    let _e145: MultiVector = other_347;
    let _e156: MultiVector = self_417;
    let _e160: MultiVector = other_347;
    return MultiVector((((((((((vec4<f32>(_e4.g0_.y) * _e8.g1_) * vec4<f32>(1.0, 1.0, -(1.0), 1.0)) + ((vec4<f32>(_e18.g0_.z) * _e22.g1_.wwyw) * vec4<f32>(1.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e33.g0_.w) * _e37.g1_.zzzy) * vec4<f32>(1.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e48.g1_.x) * vec4<f32>(_e52.g0_.y)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + (vec4<f32>(_e64.g1_.y) * _e68.g0_)) + ((vec4<f32>(_e72.g1_.z) * _e76.g0_.wwyw) * vec4<f32>(1.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e87.g1_.w) * _e91.g0_.zzzy) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e103.g0_.x) * _e107.g1_.yxxx) * vec4<f32>(1.0, 0.0, 0.0, 0.0))), ((((vec4<f32>(_e118.g1_.y) * _e122.g1_) + ((vec4<f32>(_e125.g1_.z) * _e129.g1_.wwyw) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0))) + ((vec4<f32>(_e141.g1_.w) * _e145.g1_.zzzy) * vec4<f32>(1.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e156.g1_.x) * _e160.g1_.yxxx) * vec4<f32>(1.0, 0.0, 0.0, 0.0))));
}

fn multi_vector_multi_vector_outer_product(self_418: MultiVector, other_348: MultiVector) -> MultiVector {
    var self_419: MultiVector;
    var other_349: MultiVector;

    self_419 = self_418;
    other_349 = other_348;
    let _e4: MultiVector = self_419;
    let _e8: MultiVector = other_349;
    let _e11: MultiVector = self_419;
    let _e15: MultiVector = other_349;
    let _e26: MultiVector = self_419;
    let _e30: MultiVector = other_349;
    let _e42: MultiVector = self_419;
    let _e45: MultiVector = other_349;
    let _e57: MultiVector = self_419;
    let _e61: MultiVector = other_349;
    let _e64: MultiVector = self_419;
    let _e68: MultiVector = other_349;
    let _e80: MultiVector = self_419;
    let _e84: MultiVector = other_349;
    let _e95: MultiVector = self_419;
    let _e99: MultiVector = other_349;
    let _e110: MultiVector = self_419;
    let _e114: MultiVector = other_349;
    let _e126: MultiVector = self_419;
    let _e130: MultiVector = other_349;
    let _e141: MultiVector = self_419;
    let _e145: MultiVector = other_349;
    let _e156: MultiVector = self_419;
    let _e159: MultiVector = other_349;
    return MultiVector(((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.z) * _e15.g0_.wwxw) * vec4<f32>(0.0, 1.0, 1.0, 0.0))) + ((vec4<f32>(_e26.g0_.w) * _e30.g0_.zzzx) * vec4<f32>(0.0, -(1.0), 0.0, 1.0))) + ((_e42.g0_.xyxx * vec4<f32>(_e45.g0_.x)) * vec4<f32>(0.0, 1.0, 0.0, 0.0))), ((((((((vec4<f32>(_e57.g0_.x) * _e61.g1_) + ((vec4<f32>(_e64.g0_.z) * _e68.g1_.wwxw) * vec4<f32>(0.0, 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e80.g0_.w) * _e84.g1_.zzzx) * vec4<f32>(0.0, 1.0, 0.0, 1.0))) + ((vec4<f32>(_e95.g1_.x) * _e99.g0_) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e110.g1_.y) * vec4<f32>(_e114.g0_.x)) * vec4<f32>(0.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e126.g1_.z) * _e130.g0_.wwxw) * vec4<f32>(0.0, 1.0, 1.0, 0.0))) + ((vec4<f32>(_e141.g1_.w) * _e145.g0_.zzzx) * vec4<f32>(0.0, 1.0, 0.0, 1.0))) + ((_e156.g0_.xyxx * vec4<f32>(_e159.g1_.x)) * vec4<f32>(0.0, 1.0, 0.0, 0.0))));
}

fn multi_vector_multi_vector_inner_product(self_420: MultiVector, other_350: MultiVector) -> MultiVector {
    var self_421: MultiVector;
    var other_351: MultiVector;

    self_421 = self_420;
    other_351 = other_350;
    let _e4: MultiVector = self_421;
    let _e8: MultiVector = other_351;
    let _e11: MultiVector = self_421;
    let _e15: MultiVector = other_351;
    let _e28: MultiVector = self_421;
    let _e32: MultiVector = other_351;
    let _e44: MultiVector = self_421;
    let _e47: MultiVector = other_351;
    let _e58: MultiVector = self_421;
    let _e62: MultiVector = other_351;
    let _e65: MultiVector = self_421;
    let _e69: MultiVector = other_351;
    let _e81: MultiVector = self_421;
    let _e85: MultiVector = other_351;
    let _e96: MultiVector = self_421;
    let _e100: MultiVector = other_351;
    let _e112: MultiVector = self_421;
    let _e116: MultiVector = other_351;
    let _e128: MultiVector = self_421;
    let _e132: MultiVector = other_351;
    let _e143: MultiVector = self_421;
    let _e147: MultiVector = other_351;
    let _e159: MultiVector = self_421;
    let _e162: MultiVector = other_351;
    return MultiVector(((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.y) * _e15.g0_.yxwz) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e28.g0_.w) * _e32.g0_.wwyx) * vec4<f32>(1.0, 0.0, -(1.0), 1.0))) + ((_e44.g0_.zxzz * _e47.g0_.zxxy) * vec4<f32>(1.0, 0.0, 1.0, 1.0))), ((((((((vec4<f32>(_e58.g0_.x) * _e62.g1_) + ((vec4<f32>(_e65.g0_.z) * _e69.g1_.zzzy) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))) + ((vec4<f32>(_e81.g0_.w) * _e85.g1_.wwyw) * vec4<f32>(1.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e96.g1_.x) * vec4<f32>(_e100.g0_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e112.g1_.y) * _e116.g0_.yxwz) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0))) + ((vec4<f32>(_e128.g1_.z) * _e132.g0_.zzxz) * vec4<f32>(1.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e143.g1_.w) * _e147.g0_.wwwx) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))) + ((_e159.g0_.yxxx * _e162.g1_.yxxx) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))));
}

fn multi_vector_multi_vector_geometric_anti_product(self_422: MultiVector, other_352: MultiVector) -> MultiVector {
    var self_423: MultiVector;
    var other_353: MultiVector;

    self_423 = self_422;
    other_353 = other_352;
    let _e4: MultiVector = self_423;
    let _e8: MultiVector = other_353;
    let _e19: MultiVector = self_423;
    let _e23: MultiVector = other_353;
    let _e34: MultiVector = self_423;
    let _e38: MultiVector = other_353;
    let _e50: MultiVector = self_423;
    let _e54: MultiVector = other_353;
    let _e66: MultiVector = self_423;
    let _e70: MultiVector = other_353;
    let _e83: MultiVector = self_423;
    let _e87: MultiVector = other_353;
    let _e91: MultiVector = self_423;
    let _e95: MultiVector = other_353;
    let _e100: MultiVector = self_423;
    let _e104: MultiVector = other_353;
    let _e117: MultiVector = self_423;
    let _e121: MultiVector = other_353;
    let _e133: MultiVector = self_423;
    let _e137: MultiVector = other_353;
    let _e141: MultiVector = self_423;
    let _e145: MultiVector = other_353;
    let _e158: MultiVector = self_423;
    let _e162: MultiVector = other_353;
    return MultiVector((((((((((vec4<f32>(_e4.g0_.x) * _e8.g1_.yxwz) * vec4<f32>(1.0, -(1.0), 1.0, 1.0)) + ((vec4<f32>(_e19.g0_.y) * _e23.g1_) * vec4<f32>(1.0, 1.0, -(1.0), 1.0))) + ((vec4<f32>(_e34.g0_.z) * _e38.g1_.wzyx) * vec4<f32>(1.0, -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e50.g0_.w) * _e54.g1_.zwxy) * vec4<f32>(1.0, 1.0, -(1.0), 1.0))) + ((vec4<f32>(_e66.g1_.x) * _e70.g0_.yxwz) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))) + (vec4<f32>(_e83.g1_.y) * _e87.g0_)) + (vec4<f32>(_e91.g1_.z) * _e95.g0_.wzyx)) + ((vec4<f32>(_e100.g1_.w) * _e104.g0_.zwxy) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))), (((((vec4<f32>(_e117.g1_.x) * _e121.g1_.yxwz) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0)) + (vec4<f32>(_e133.g1_.y) * _e137.g1_)) + ((vec4<f32>(_e141.g1_.z) * _e145.g1_.wzyx) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + (vec4<f32>(_e158.g1_.w) * _e162.g1_.zwxy)));
}

fn multi_vector_multi_vector_inner_anti_product(self_424: MultiVector, other_354: MultiVector) -> MultiVector {
    var self_425: MultiVector;
    var other_355: MultiVector;

    self_425 = self_424;
    other_355 = other_354;
    let _e4: MultiVector = self_425;
    let _e8: MultiVector = other_355;
    let _e19: MultiVector = self_425;
    let _e23: MultiVector = other_355;
    let _e35: MultiVector = self_425;
    let _e39: MultiVector = other_355;
    let _e50: MultiVector = self_425;
    let _e54: MultiVector = other_355;
    let _e67: MultiVector = self_425;
    let _e71: MultiVector = other_355;
    let _e75: MultiVector = self_425;
    let _e79: MultiVector = other_355;
    let _e90: MultiVector = self_425;
    let _e94: MultiVector = other_355;
    let _e106: MultiVector = self_425;
    let _e109: MultiVector = other_355;
    let _e120: MultiVector = self_425;
    let _e124: MultiVector = other_355;
    let _e136: MultiVector = self_425;
    let _e140: MultiVector = other_355;
    let _e144: MultiVector = self_425;
    let _e148: MultiVector = other_355;
    let _e159: MultiVector = self_425;
    let _e162: MultiVector = other_355;
    return MultiVector((((((((((vec4<f32>(_e4.g0_.x) * _e8.g1_.yxwz) * vec4<f32>(1.0, -(1.0), 1.0, 1.0)) + ((vec4<f32>(_e19.g0_.z) * _e23.g1_.zzyz) * vec4<f32>(0.0, -(1.0), 1.0, 0.0))) + ((vec4<f32>(_e35.g0_.w) * _e39.g1_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, 1.0))) + ((vec4<f32>(_e50.g1_.x) * vec4<f32>(_e54.g0_.x)) * vec4<f32>(0.0, -(1.0), 0.0, 0.0))) + (vec4<f32>(_e67.g1_.y) * _e71.g0_)) + ((vec4<f32>(_e75.g1_.z) * _e79.g0_.zzzx) * vec4<f32>(0.0, 1.0, 0.0, 1.0))) + ((vec4<f32>(_e90.g1_.w) * _e94.g0_.wwxw) * vec4<f32>(0.0, -(1.0), 1.0, 0.0))) + ((_e106.g0_.xyxx * _e109.g1_.xyxx) * vec4<f32>(0.0, 1.0, 0.0, 0.0))), (((((vec4<f32>(_e120.g1_.x) * _e124.g1_.yxwz) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0)) + (vec4<f32>(_e136.g1_.y) * _e140.g1_)) + ((vec4<f32>(_e144.g1_.w) * _e148.g1_.wwxy) * vec4<f32>(0.0, 1.0, 1.0, 1.0))) + ((_e159.g1_.xzzz * _e162.g1_.xzyx) * vec4<f32>(0.0, 1.0, 1.0, -(1.0)))));
}

fn multi_vector_multi_vector_left_contraction(self_426: MultiVector, other_356: MultiVector) -> MultiVector {
    var self_427: MultiVector;
    var other_357: MultiVector;

    self_427 = self_426;
    other_357 = other_356;
    let _e4: MultiVector = self_427;
    let _e8: MultiVector = other_357;
    let _e11: MultiVector = self_427;
    let _e15: MultiVector = other_357;
    let _e26: MultiVector = self_427;
    let _e30: MultiVector = other_357;
    let _e42: MultiVector = self_427;
    let _e45: MultiVector = other_357;
    let _e57: MultiVector = self_427;
    let _e61: MultiVector = other_357;
    let _e64: MultiVector = self_427;
    let _e68: MultiVector = other_357;
    let _e80: MultiVector = self_427;
    let _e84: MultiVector = other_357;
    let _e95: MultiVector = self_427;
    let _e98: MultiVector = other_357;
    return MultiVector(((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.z) * _e15.g0_.zzzy) * vec4<f32>(1.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e26.g0_.w) * _e30.g0_.wwyw) * vec4<f32>(1.0, 0.0, -(1.0), 0.0))) + ((_e42.g0_.yxxx * _e45.g0_.yxxx) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))), ((((vec4<f32>(_e57.g0_.x) * _e61.g1_) + ((vec4<f32>(_e64.g0_.z) * _e68.g1_.zzzy) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))) + ((vec4<f32>(_e80.g0_.w) * _e84.g1_.wwyw) * vec4<f32>(1.0, 0.0, 1.0, 0.0))) + ((_e95.g0_.yxxx * _e98.g1_.yxxx) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))));
}

fn multi_vector_multi_vector_right_contraction(self_428: MultiVector, other_358: MultiVector) -> MultiVector {
    var self_429: MultiVector;
    var other_359: MultiVector;

    self_429 = self_428;
    other_359 = other_358;
    let _e4: MultiVector = self_429;
    let _e8: MultiVector = other_359;
    let _e20: MultiVector = self_429;
    let _e24: MultiVector = other_359;
    let _e35: MultiVector = self_429;
    let _e39: MultiVector = other_359;
    let _e50: MultiVector = self_429;
    let _e54: MultiVector = other_359;
    let _e66: MultiVector = self_429;
    let _e70: MultiVector = other_359;
    let _e81: MultiVector = self_429;
    let _e85: MultiVector = other_359;
    let _e96: MultiVector = self_429;
    let _e100: MultiVector = other_359;
    let _e112: MultiVector = self_429;
    let _e116: MultiVector = other_359;
    return MultiVector((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.yxwz) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0))) + ((vec4<f32>(_e20.g0_.z) * _e24.g0_.zzxz) * vec4<f32>(1.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e35.g0_.w) * _e39.g0_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e50.g0_.x) * vec4<f32>(_e54.g0_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))), (((((vec4<f32>(_e66.g1_.y) * _e70.g0_.yxwz) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0)) + ((vec4<f32>(_e81.g1_.z) * _e85.g0_.zzxz) * vec4<f32>(1.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e96.g1_.w) * _e100.g0_.wwwx) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))) + ((vec4<f32>(_e112.g1_.x) * vec4<f32>(_e116.g0_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))));
}

fn multi_vector_multi_vector_left_anti_contraction(self_430: MultiVector, other_360: MultiVector) -> MultiVector {
    var self_431: MultiVector;
    var other_361: MultiVector;

    self_431 = self_430;
    other_361 = other_360;
    let _e4: MultiVector = self_431;
    let _e8: MultiVector = other_361;
    let _e11: MultiVector = self_431;
    let _e15: MultiVector = other_361;
    let _e26: MultiVector = self_431;
    let _e30: MultiVector = other_361;
    let _e42: MultiVector = self_431;
    let _e46: MultiVector = other_361;
    let _e59: MultiVector = self_431;
    let _e63: MultiVector = other_361;
    let _e66: MultiVector = self_431;
    let _e70: MultiVector = other_361;
    let _e82: MultiVector = self_431;
    let _e86: MultiVector = other_361;
    let _e97: MultiVector = self_431;
    let _e101: MultiVector = other_361;
    return MultiVector(((((vec4<f32>(_e4.g1_.y) * _e8.g0_) + ((vec4<f32>(_e11.g1_.z) * _e15.g0_.zzzx) * vec4<f32>(0.0, 1.0, 0.0, 1.0))) + ((vec4<f32>(_e26.g1_.w) * _e30.g0_.wwxw) * vec4<f32>(0.0, -(1.0), 1.0, 0.0))) + ((vec4<f32>(_e42.g1_.x) * vec4<f32>(_e46.g0_.x)) * vec4<f32>(0.0, -(1.0), 0.0, 0.0))), ((((vec4<f32>(_e59.g1_.y) * _e63.g1_) + ((vec4<f32>(_e66.g1_.z) * _e70.g1_.zzzx) * vec4<f32>(0.0, 1.0, 0.0, -(1.0)))) + ((vec4<f32>(_e82.g1_.w) * _e86.g1_.wwxw) * vec4<f32>(0.0, 1.0, 1.0, 0.0))) + ((vec4<f32>(_e97.g1_.x) * vec4<f32>(_e101.g1_.x)) * vec4<f32>(0.0, -(1.0), 0.0, 0.0))));
}

fn multi_vector_multi_vector_right_anti_contraction(self_432: MultiVector, other_362: MultiVector) -> MultiVector {
    var self_433: MultiVector;
    var other_363: MultiVector;

    self_433 = self_432;
    other_363 = other_362;
    let _e4: MultiVector = self_433;
    let _e8: MultiVector = other_363;
    let _e19: MultiVector = self_433;
    let _e23: MultiVector = other_363;
    let _e35: MultiVector = self_433;
    let _e39: MultiVector = other_363;
    let _e50: MultiVector = self_433;
    let _e53: MultiVector = other_363;
    let _e64: MultiVector = self_433;
    let _e68: MultiVector = other_363;
    let _e80: MultiVector = self_433;
    let _e84: MultiVector = other_363;
    let _e95: MultiVector = self_433;
    let _e99: MultiVector = other_363;
    let _e110: MultiVector = self_433;
    let _e113: MultiVector = other_363;
    return MultiVector((((((vec4<f32>(_e4.g0_.x) * _e8.g1_.yxwz) * vec4<f32>(1.0, -(1.0), 1.0, 1.0)) + ((vec4<f32>(_e19.g0_.z) * _e23.g1_.zzyz) * vec4<f32>(0.0, -(1.0), 1.0, 0.0))) + ((vec4<f32>(_e35.g0_.w) * _e39.g1_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, 1.0))) + ((_e50.g0_.xyxx * _e53.g1_.xyxx) * vec4<f32>(0.0, 1.0, 0.0, 0.0))), (((((vec4<f32>(_e64.g1_.x) * _e68.g1_.yxwz) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0)) + ((vec4<f32>(_e80.g1_.z) * _e84.g1_.zzyz) * vec4<f32>(0.0, 1.0, 1.0, 0.0))) + ((vec4<f32>(_e95.g1_.w) * _e99.g1_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, 1.0))) + ((_e110.g1_.xyxx * _e113.g1_.xyxx) * vec4<f32>(0.0, 1.0, 0.0, 0.0))));
}

fn multi_vector_multi_vector_scalar_product(self_434: MultiVector, other_364: MultiVector) -> Scalar {
    var self_435: MultiVector;
    var other_365: MultiVector;

    self_435 = self_434;
    other_365 = other_364;
    let _e4: MultiVector = self_435;
    let _e7: MultiVector = other_365;
    let _e11: MultiVector = self_435;
    let _e14: MultiVector = other_365;
    let _e19: MultiVector = self_435;
    let _e22: MultiVector = other_365;
    let _e27: MultiVector = self_435;
    let _e30: MultiVector = other_365;
    return Scalar(((((_e4.g0_.x * _e7.g0_.x) - (_e11.g0_.y * _e14.g0_.y)) + (_e19.g0_.z * _e22.g0_.z)) + (_e27.g0_.w * _e30.g0_.w)));
}

fn multi_vector_multi_vector_anti_scalar_product(self_436: MultiVector, other_366: MultiVector) -> AntiScalar {
    var self_437: MultiVector;
    var other_367: MultiVector;

    self_437 = self_436;
    other_367 = other_366;
    let _e5: MultiVector = self_437;
    let _e8: MultiVector = other_367;
    let _e13: MultiVector = self_437;
    let _e16: MultiVector = other_367;
    let _e21: MultiVector = self_437;
    let _e24: MultiVector = other_367;
    let _e29: MultiVector = self_437;
    let _e32: MultiVector = other_367;
    return AntiScalar(((((0.0 - (_e5.g1_.x * _e8.g1_.x)) + (_e13.g1_.y * _e16.g1_.y)) + (_e21.g1_.z * _e24.g1_.z)) + (_e29.g1_.w * _e32.g1_.w)));
}

fn multi_vector_rotor_into(self_438: MultiVector) -> Rotor {
    var self_439: MultiVector;

    self_439 = self_438;
    let _e2: MultiVector = self_439;
    let _e5: MultiVector = self_439;
    return Rotor(vec2<f32>(_e2.g0_.x, _e5.g0_.y));
}

fn multi_vector_rotor_add(self_440: MultiVector, other_368: Rotor) -> MultiVector {
    var self_441: MultiVector;
    var other_369: Rotor;

    self_441 = self_440;
    other_369 = other_368;
    let _e4: MultiVector = self_441;
    let _e6: Rotor = other_369;
    let _e9: Rotor = other_369;
    let _e12: Rotor = other_369;
    let _e15: Rotor = other_369;
    let _e26: MultiVector = self_441;
    return MultiVector((_e4.g0_ + (vec4<f32>(_e6.g0_.x, _e9.g0_.y, _e12.g0_.x, _e15.g0_.x) * vec4<f32>(1.0, 1.0, 0.0, 0.0))), _e26.g1_);
}

fn multi_vector_rotor_sub(self_442: MultiVector, other_370: Rotor) -> MultiVector {
    var self_443: MultiVector;
    var other_371: Rotor;

    self_443 = self_442;
    other_371 = other_370;
    let _e4: MultiVector = self_443;
    let _e6: Rotor = other_371;
    let _e9: Rotor = other_371;
    let _e12: Rotor = other_371;
    let _e15: Rotor = other_371;
    let _e26: MultiVector = self_443;
    return MultiVector((_e4.g0_ - (vec4<f32>(_e6.g0_.x, _e9.g0_.y, _e12.g0_.x, _e15.g0_.x) * vec4<f32>(1.0, 1.0, 0.0, 0.0))), _e26.g1_);
}

fn multi_vector_rotor_geometric_product(self_444: MultiVector, other_372: Rotor) -> MultiVector {
    var self_445: MultiVector;
    var other_373: Rotor;

    self_445 = self_444;
    other_373 = other_372;
    let _e4: MultiVector = self_445;
    let _e8: Rotor = other_373;
    let _e11: Rotor = other_373;
    let _e14: Rotor = other_373;
    let _e17: Rotor = other_373;
    let _e29: MultiVector = self_445;
    let _e33: Rotor = other_373;
    let _e36: Rotor = other_373;
    let _e39: Rotor = other_373;
    let _e42: Rotor = other_373;
    let _e55: MultiVector = self_445;
    let _e58: Rotor = other_373;
    let _e61: Rotor = other_373;
    let _e64: Rotor = other_373;
    let _e67: Rotor = other_373;
    let _e73: MultiVector = self_445;
    let _e77: Rotor = other_373;
    let _e80: Rotor = other_373;
    let _e83: Rotor = other_373;
    let _e86: Rotor = other_373;
    let _e98: MultiVector = self_445;
    let _e102: Rotor = other_373;
    let _e105: Rotor = other_373;
    let _e108: Rotor = other_373;
    let _e111: Rotor = other_373;
    let _e123: MultiVector = self_445;
    let _e126: Rotor = other_373;
    let _e129: Rotor = other_373;
    let _e132: Rotor = other_373;
    let _e135: Rotor = other_373;
    return MultiVector(((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y, _e11.g0_.x, _e14.g0_.y, _e17.g0_.y)) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0)) + ((vec4<f32>(_e29.g0_.w) * vec4<f32>(_e33.g0_.y, _e36.g0_.y, _e39.g0_.y, _e42.g0_.x)) * vec4<f32>(0.0, 0.0, -(1.0), 1.0))) + (_e55.g0_.xxzz * vec4<f32>(_e58.g0_.x, _e61.g0_.y, _e64.g0_.x, _e67.g0_.y))), ((((vec4<f32>(_e73.g1_.y) * vec4<f32>(_e77.g0_.y, _e80.g0_.x, _e83.g0_.y, _e86.g0_.y)) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0)) + ((vec4<f32>(_e98.g1_.w) * vec4<f32>(_e102.g0_.y, _e105.g0_.y, _e108.g0_.y, _e111.g0_.x)) * vec4<f32>(0.0, 0.0, 1.0, 1.0))) + ((_e123.g1_.xxzz * vec4<f32>(_e126.g0_.x, _e129.g0_.y, _e132.g0_.x, _e135.g0_.y)) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))));
}

fn multi_vector_rotor_outer_product(self_446: MultiVector, other_374: Rotor) -> MultiVector {
    var self_447: MultiVector;
    var other_375: Rotor;

    self_447 = self_446;
    other_375 = other_374;
    let _e4: MultiVector = self_447;
    let _e8: Rotor = other_375;
    let _e19: MultiVector = self_447;
    let _e22: Rotor = other_375;
    let _e25: Rotor = other_375;
    let _e28: Rotor = other_375;
    let _e31: Rotor = other_375;
    let _e37: MultiVector = self_447;
    let _e41: Rotor = other_375;
    let _e52: MultiVector = self_447;
    let _e55: Rotor = other_375;
    let _e58: Rotor = other_375;
    let _e61: Rotor = other_375;
    let _e64: Rotor = other_375;
    return MultiVector((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.x)) * vec4<f32>(0.0, 1.0, 0.0, 0.0)) + (_e19.g0_.xxzw * vec4<f32>(_e22.g0_.x, _e25.g0_.y, _e28.g0_.x, _e31.g0_.x))), (((vec4<f32>(_e37.g1_.y) * vec4<f32>(_e41.g0_.x)) * vec4<f32>(0.0, 1.0, 0.0, 0.0)) + (_e52.g1_.xxzw * vec4<f32>(_e55.g0_.x, _e58.g0_.y, _e61.g0_.x, _e64.g0_.x))));
}

fn multi_vector_rotor_inner_product(self_448: MultiVector, other_376: Rotor) -> MultiVector {
    var self_449: MultiVector;
    var other_377: Rotor;

    self_449 = self_448;
    other_377 = other_376;
    let _e4: MultiVector = self_449;
    let _e8: Rotor = other_377;
    let _e11: Rotor = other_377;
    let _e14: Rotor = other_377;
    let _e17: Rotor = other_377;
    let _e29: MultiVector = self_449;
    let _e33: Rotor = other_377;
    let _e36: Rotor = other_377;
    let _e39: Rotor = other_377;
    let _e42: Rotor = other_377;
    let _e55: MultiVector = self_449;
    let _e58: Rotor = other_377;
    let _e61: Rotor = other_377;
    let _e64: Rotor = other_377;
    let _e67: Rotor = other_377;
    let _e73: MultiVector = self_449;
    let _e77: Rotor = other_377;
    let _e80: Rotor = other_377;
    let _e83: Rotor = other_377;
    let _e86: Rotor = other_377;
    let _e98: MultiVector = self_449;
    let _e101: Rotor = other_377;
    return MultiVector(((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y, _e11.g0_.x, _e14.g0_.y, _e17.g0_.y)) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0)) + ((vec4<f32>(_e29.g0_.w) * vec4<f32>(_e33.g0_.y, _e36.g0_.y, _e39.g0_.y, _e42.g0_.x)) * vec4<f32>(0.0, 0.0, -(1.0), 1.0))) + (_e55.g0_.xxzz * vec4<f32>(_e58.g0_.x, _e61.g0_.y, _e64.g0_.x, _e67.g0_.y))), (((vec4<f32>(_e73.g1_.y) * vec4<f32>(_e77.g0_.y, _e80.g0_.x, _e83.g0_.y, _e86.g0_.y)) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0)) + ((_e98.g1_.xxzw * vec4<f32>(_e101.g0_.x)) * vec4<f32>(1.0, 0.0, 1.0, 1.0))));
}

fn multi_vector_rotor_right_contraction(self_450: MultiVector, other_378: Rotor) -> MultiVector {
    var self_451: MultiVector;
    var other_379: Rotor;

    self_451 = self_450;
    other_379 = other_378;
    let _e4: MultiVector = self_451;
    let _e8: Rotor = other_379;
    let _e11: Rotor = other_379;
    let _e14: Rotor = other_379;
    let _e17: Rotor = other_379;
    let _e29: MultiVector = self_451;
    let _e32: Rotor = other_379;
    let _e44: MultiVector = self_451;
    let _e48: Rotor = other_379;
    let _e51: Rotor = other_379;
    let _e54: Rotor = other_379;
    let _e57: Rotor = other_379;
    let _e69: MultiVector = self_451;
    let _e72: Rotor = other_379;
    return MultiVector((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y, _e11.g0_.x, _e14.g0_.y, _e17.g0_.y)) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0)) + ((_e29.g0_.xxzw * vec4<f32>(_e32.g0_.x)) * vec4<f32>(1.0, 0.0, 1.0, 1.0))), (((vec4<f32>(_e44.g1_.y) * vec4<f32>(_e48.g0_.y, _e51.g0_.x, _e54.g0_.y, _e57.g0_.y)) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0)) + ((_e69.g1_.xxzw * vec4<f32>(_e72.g0_.x)) * vec4<f32>(1.0, 0.0, 1.0, 1.0))));
}

fn multi_vector_rotor_scalar_product(self_452: MultiVector, other_380: Rotor) -> Scalar {
    var self_453: MultiVector;
    var other_381: Rotor;

    self_453 = self_452;
    other_381 = other_380;
    let _e4: MultiVector = self_453;
    let _e7: Rotor = other_381;
    let _e11: MultiVector = self_453;
    let _e14: Rotor = other_381;
    return Scalar(((_e4.g0_.x * _e7.g0_.x) - (_e11.g0_.y * _e14.g0_.y)));
}

fn multi_vector_point_into(self_454: MultiVector) -> Point {
    var self_455: MultiVector;

    self_455 = self_454;
    let _e2: MultiVector = self_455;
    let _e5: MultiVector = self_455;
    let _e8: MultiVector = self_455;
    return Point(vec3<f32>(_e2.g0_.y, _e5.g1_.z, _e8.g1_.w));
}

fn multi_vector_point_add(self_456: MultiVector, other_382: Point) -> MultiVector {
    var self_457: MultiVector;
    var other_383: Point;

    self_457 = self_456;
    other_383 = other_382;
    let _e4: MultiVector = self_457;
    let _e6: Point = other_383;
    let _e17: MultiVector = self_457;
    let _e19: Point = other_383;
    let _e22: Point = other_383;
    let _e25: Point = other_383;
    let _e28: Point = other_383;
    return MultiVector((_e4.g0_ + (vec4<f32>(_e6.g0_.x) * vec4<f32>(0.0, 1.0, 0.0, 0.0))), (_e17.g1_ + (vec4<f32>(_e19.g0_.x, _e22.g0_.x, _e25.g0_.y, _e28.g0_.z) * vec4<f32>(0.0, 0.0, 1.0, 1.0))));
}

fn multi_vector_point_sub(self_458: MultiVector, other_384: Point) -> MultiVector {
    var self_459: MultiVector;
    var other_385: Point;

    self_459 = self_458;
    other_385 = other_384;
    let _e4: MultiVector = self_459;
    let _e6: Point = other_385;
    let _e17: MultiVector = self_459;
    let _e19: Point = other_385;
    let _e22: Point = other_385;
    let _e25: Point = other_385;
    let _e28: Point = other_385;
    return MultiVector((_e4.g0_ - (vec4<f32>(_e6.g0_.x) * vec4<f32>(0.0, 1.0, 0.0, 0.0))), (_e17.g1_ - (vec4<f32>(_e19.g0_.x, _e22.g0_.x, _e25.g0_.y, _e28.g0_.z) * vec4<f32>(0.0, 0.0, 1.0, 1.0))));
}

fn multi_vector_point_geometric_product(self_460: MultiVector, other_386: Point) -> MultiVector {
    var self_461: MultiVector;
    var other_387: Point;

    self_461 = self_460;
    other_387 = other_386;
    let _e4: MultiVector = self_461;
    let _e7: Point = other_387;
    let _e20: MultiVector = self_461;
    let _e24: Point = other_387;
    let _e27: Point = other_387;
    let _e30: Point = other_387;
    let _e33: Point = other_387;
    let _e45: MultiVector = self_461;
    let _e49: Point = other_387;
    let _e52: Point = other_387;
    let _e55: Point = other_387;
    let _e58: Point = other_387;
    let _e70: MultiVector = self_461;
    let _e74: Point = other_387;
    let _e86: MultiVector = self_461;
    let _e90: Point = other_387;
    let _e103: MultiVector = self_461;
    let _e107: Point = other_387;
    let _e120: MultiVector = self_461;
    let _e124: Point = other_387;
    let _e136: MultiVector = self_461;
    let _e139: Point = other_387;
    let _e142: Point = other_387;
    let _e145: Point = other_387;
    let _e148: Point = other_387;
    return MultiVector(((_e4.g0_.yxwz * vec4<f32>(_e7.g0_.x)) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0)), ((((((((vec4<f32>(_e20.g0_.y) * vec4<f32>(_e24.g0_.z, _e27.g0_.z, _e30.g0_.z, _e33.g0_.y)) * vec4<f32>(0.0, 0.0, -(1.0), 1.0)) + ((vec4<f32>(_e45.g0_.w) * vec4<f32>(_e49.g0_.z, _e52.g0_.y, _e55.g0_.z, _e58.g0_.z)) * vec4<f32>(1.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e70.g1_.x) * vec4<f32>(_e74.g0_.x)) * vec4<f32>(0.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e86.g1_.y) * vec4<f32>(_e90.g0_.x)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((vec4<f32>(_e103.g1_.z) * vec4<f32>(_e107.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e120.g1_.w) * vec4<f32>(_e124.g0_.x)) * vec4<f32>(0.0, 0.0, 1.0, 0.0))) + ((_e136.g0_.zzxx * vec4<f32>(_e139.g0_.y, _e142.g0_.z, _e145.g0_.y, _e148.g0_.z)) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0))));
}

fn multi_vector_point_geometric_anti_product(self_462: MultiVector, other_388: Point) -> MultiVector {
    var self_463: MultiVector;
    var other_389: Point;

    self_463 = self_462;
    other_389 = other_388;
    let _e4: MultiVector = self_463;
    let _e8: Point = other_389;
    let _e11: Point = other_389;
    let _e14: Point = other_389;
    let _e17: Point = other_389;
    let _e29: MultiVector = self_463;
    let _e33: Point = other_389;
    let _e36: Point = other_389;
    let _e39: Point = other_389;
    let _e42: Point = other_389;
    let _e54: MultiVector = self_463;
    let _e58: Point = other_389;
    let _e70: MultiVector = self_463;
    let _e74: Point = other_389;
    let _e86: MultiVector = self_463;
    let _e90: Point = other_389;
    let _e102: MultiVector = self_463;
    let _e106: Point = other_389;
    let _e119: MultiVector = self_463;
    let _e122: Point = other_389;
    let _e125: Point = other_389;
    let _e128: Point = other_389;
    let _e131: Point = other_389;
    let _e144: MultiVector = self_463;
    let _e148: Point = other_389;
    let _e151: Point = other_389;
    let _e154: Point = other_389;
    let _e157: Point = other_389;
    let _e168: MultiVector = self_463;
    let _e172: Point = other_389;
    let _e175: Point = other_389;
    let _e178: Point = other_389;
    let _e181: Point = other_389;
    let _e193: MultiVector = self_463;
    let _e196: Point = other_389;
    let _e199: Point = other_389;
    let _e202: Point = other_389;
    let _e205: Point = other_389;
    return MultiVector(((((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y, _e11.g0_.y, _e14.g0_.y, _e17.g0_.z)) * vec4<f32>(0.0, 0.0, -(1.0), 1.0)) + ((vec4<f32>(_e29.g0_.w) * vec4<f32>(_e33.g0_.y, _e36.g0_.z, _e39.g0_.y, _e42.g0_.y)) * vec4<f32>(1.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e54.g1_.x) * vec4<f32>(_e58.g0_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e70.g1_.y) * vec4<f32>(_e74.g0_.x)) * vec4<f32>(0.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e86.g1_.z) * vec4<f32>(_e90.g0_.x)) * vec4<f32>(0.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e102.g1_.w) * vec4<f32>(_e106.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((_e119.g0_.zzxx * vec4<f32>(_e122.g0_.z, _e125.g0_.y, _e128.g0_.z, _e131.g0_.y)) * vec4<f32>(1.0, -(1.0), 1.0, 1.0))), ((((vec4<f32>(_e144.g1_.y) * vec4<f32>(_e148.g0_.y, _e151.g0_.y, _e154.g0_.y, _e157.g0_.z)) * vec4<f32>(0.0, 0.0, 1.0, 1.0)) + ((vec4<f32>(_e168.g1_.w) * vec4<f32>(_e172.g0_.y, _e175.g0_.z, _e178.g0_.y, _e181.g0_.y)) * vec4<f32>(1.0, 1.0, 0.0, 0.0))) + ((_e193.g1_.zzxx * vec4<f32>(_e196.g0_.z, _e199.g0_.y, _e202.g0_.z, _e205.g0_.y)) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))));
}

fn multi_vector_point_scalar_product(self_464: MultiVector, other_390: Point) -> Scalar {
    var self_465: MultiVector;
    var other_391: Point;

    self_465 = self_464;
    other_391 = other_390;
    let _e5: MultiVector = self_465;
    let _e8: Point = other_391;
    return Scalar((0.0 - (_e5.g0_.y * _e8.g0_.x)));
}

fn multi_vector_point_anti_scalar_product(self_466: MultiVector, other_392: Point) -> AntiScalar {
    var self_467: MultiVector;
    var other_393: Point;

    self_467 = self_466;
    other_393 = other_392;
    let _e4: MultiVector = self_467;
    let _e7: Point = other_393;
    let _e11: MultiVector = self_467;
    let _e14: Point = other_393;
    return AntiScalar(((_e4.g1_.z * _e7.g0_.y) + (_e11.g1_.w * _e14.g0_.z)));
}

fn multi_vector_ideal_point_into(self_468: MultiVector) -> IdealPoint {
    var self_469: MultiVector;

    self_469 = self_468;
    let _e2: MultiVector = self_469;
    let _e5: MultiVector = self_469;
    return IdealPoint(vec2<f32>(_e2.g1_.z, _e5.g1_.w));
}

fn multi_vector_ideal_point_add(self_470: MultiVector, other_394: IdealPoint) -> MultiVector {
    var self_471: MultiVector;
    var other_395: IdealPoint;

    self_471 = self_470;
    other_395 = other_394;
    let _e4: MultiVector = self_471;
    let _e6: MultiVector = self_471;
    let _e8: IdealPoint = other_395;
    let _e11: IdealPoint = other_395;
    let _e14: IdealPoint = other_395;
    let _e17: IdealPoint = other_395;
    return MultiVector(_e4.g0_, (_e6.g1_ + (vec4<f32>(_e8.g0_.x, _e11.g0_.x, _e14.g0_.x, _e17.g0_.y) * vec4<f32>(0.0, 0.0, 1.0, 1.0))));
}

fn multi_vector_ideal_point_sub(self_472: MultiVector, other_396: IdealPoint) -> MultiVector {
    var self_473: MultiVector;
    var other_397: IdealPoint;

    self_473 = self_472;
    other_397 = other_396;
    let _e4: MultiVector = self_473;
    let _e6: MultiVector = self_473;
    let _e8: IdealPoint = other_397;
    let _e11: IdealPoint = other_397;
    let _e14: IdealPoint = other_397;
    let _e17: IdealPoint = other_397;
    return MultiVector(_e4.g0_, (_e6.g1_ - (vec4<f32>(_e8.g0_.x, _e11.g0_.x, _e14.g0_.x, _e17.g0_.y) * vec4<f32>(0.0, 0.0, 1.0, 1.0))));
}

fn multi_vector_ideal_point_geometric_anti_product(self_474: MultiVector, other_398: IdealPoint) -> MultiVector {
    var self_475: MultiVector;
    var other_399: IdealPoint;

    self_475 = self_474;
    other_399 = other_398;
    let _e4: MultiVector = self_475;
    let _e8: IdealPoint = other_399;
    let _e11: IdealPoint = other_399;
    let _e14: IdealPoint = other_399;
    let _e17: IdealPoint = other_399;
    let _e29: MultiVector = self_475;
    let _e33: IdealPoint = other_399;
    let _e36: IdealPoint = other_399;
    let _e39: IdealPoint = other_399;
    let _e42: IdealPoint = other_399;
    let _e54: MultiVector = self_475;
    let _e57: IdealPoint = other_399;
    let _e60: IdealPoint = other_399;
    let _e63: IdealPoint = other_399;
    let _e66: IdealPoint = other_399;
    let _e79: MultiVector = self_475;
    let _e83: IdealPoint = other_399;
    let _e86: IdealPoint = other_399;
    let _e89: IdealPoint = other_399;
    let _e92: IdealPoint = other_399;
    let _e103: MultiVector = self_475;
    let _e107: IdealPoint = other_399;
    let _e110: IdealPoint = other_399;
    let _e113: IdealPoint = other_399;
    let _e116: IdealPoint = other_399;
    let _e128: MultiVector = self_475;
    let _e131: IdealPoint = other_399;
    let _e134: IdealPoint = other_399;
    let _e137: IdealPoint = other_399;
    let _e140: IdealPoint = other_399;
    return MultiVector(((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.x, _e11.g0_.x, _e14.g0_.x, _e17.g0_.y)) * vec4<f32>(0.0, 0.0, -(1.0), 1.0)) + ((vec4<f32>(_e29.g0_.w) * vec4<f32>(_e33.g0_.x, _e36.g0_.y, _e39.g0_.x, _e42.g0_.x)) * vec4<f32>(1.0, 1.0, 0.0, 0.0))) + ((_e54.g0_.zzxx * vec4<f32>(_e57.g0_.y, _e60.g0_.x, _e63.g0_.y, _e66.g0_.x)) * vec4<f32>(1.0, -(1.0), 1.0, 1.0))), ((((vec4<f32>(_e79.g1_.y) * vec4<f32>(_e83.g0_.x, _e86.g0_.x, _e89.g0_.x, _e92.g0_.y)) * vec4<f32>(0.0, 0.0, 1.0, 1.0)) + ((vec4<f32>(_e103.g1_.w) * vec4<f32>(_e107.g0_.x, _e110.g0_.y, _e113.g0_.x, _e116.g0_.x)) * vec4<f32>(1.0, 1.0, 0.0, 0.0))) + ((_e128.g1_.zzxx * vec4<f32>(_e131.g0_.y, _e134.g0_.x, _e137.g0_.y, _e140.g0_.x)) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))));
}

fn multi_vector_ideal_point_anti_scalar_product(self_476: MultiVector, other_400: IdealPoint) -> AntiScalar {
    var self_477: MultiVector;
    var other_401: IdealPoint;

    self_477 = self_476;
    other_401 = other_400;
    let _e4: MultiVector = self_477;
    let _e7: IdealPoint = other_401;
    let _e11: MultiVector = self_477;
    let _e14: IdealPoint = other_401;
    return AntiScalar(((_e4.g1_.z * _e7.g0_.x) + (_e11.g1_.w * _e14.g0_.y)));
}

fn multi_vector_plane_into(self_478: MultiVector) -> Plane {
    var self_479: MultiVector;

    self_479 = self_478;
    let _e2: MultiVector = self_479;
    let _e5: MultiVector = self_479;
    let _e8: MultiVector = self_479;
    return Plane(vec3<f32>(_e2.g1_.x, _e5.g0_.w, _e8.g0_.z));
}

fn multi_vector_plane_add(self_480: MultiVector, other_402: Plane) -> MultiVector {
    var self_481: MultiVector;
    var other_403: Plane;

    self_481 = self_480;
    other_403 = other_402;
    let _e4: MultiVector = self_481;
    let _e6: Plane = other_403;
    let _e9: Plane = other_403;
    let _e12: Plane = other_403;
    let _e15: Plane = other_403;
    let _e26: MultiVector = self_481;
    let _e28: Plane = other_403;
    return MultiVector((_e4.g0_ + (vec4<f32>(_e6.g0_.x, _e9.g0_.x, _e12.g0_.z, _e15.g0_.y) * vec4<f32>(0.0, 0.0, 1.0, 1.0))), (_e26.g1_ + (vec4<f32>(_e28.g0_.x) * vec4<f32>(1.0, 0.0, 0.0, 0.0))));
}

fn multi_vector_plane_sub(self_482: MultiVector, other_404: Plane) -> MultiVector {
    var self_483: MultiVector;
    var other_405: Plane;

    self_483 = self_482;
    other_405 = other_404;
    let _e4: MultiVector = self_483;
    let _e6: Plane = other_405;
    let _e9: Plane = other_405;
    let _e12: Plane = other_405;
    let _e15: Plane = other_405;
    let _e26: MultiVector = self_483;
    let _e28: Plane = other_405;
    return MultiVector((_e4.g0_ - (vec4<f32>(_e6.g0_.x, _e9.g0_.x, _e12.g0_.z, _e15.g0_.y) * vec4<f32>(0.0, 0.0, 1.0, 1.0))), (_e26.g1_ - (vec4<f32>(_e28.g0_.x) * vec4<f32>(1.0, 0.0, 0.0, 0.0))));
}

fn multi_vector_plane_geometric_product(self_484: MultiVector, other_406: Plane) -> MultiVector {
    var self_485: MultiVector;
    var other_407: Plane;

    self_485 = self_484;
    other_407 = other_406;
    let _e4: MultiVector = self_485;
    let _e8: Plane = other_407;
    let _e11: Plane = other_407;
    let _e14: Plane = other_407;
    let _e17: Plane = other_407;
    let _e29: MultiVector = self_485;
    let _e33: Plane = other_407;
    let _e36: Plane = other_407;
    let _e39: Plane = other_407;
    let _e42: Plane = other_407;
    let _e55: MultiVector = self_485;
    let _e58: Plane = other_407;
    let _e61: Plane = other_407;
    let _e64: Plane = other_407;
    let _e67: Plane = other_407;
    let _e73: MultiVector = self_485;
    let _e77: Plane = other_407;
    let _e80: Plane = other_407;
    let _e83: Plane = other_407;
    let _e86: Plane = other_407;
    let _e98: MultiVector = self_485;
    let _e102: Plane = other_407;
    let _e105: Plane = other_407;
    let _e108: Plane = other_407;
    let _e111: Plane = other_407;
    let _e123: MultiVector = self_485;
    let _e127: Plane = other_407;
    let _e130: Plane = other_407;
    let _e133: Plane = other_407;
    let _e136: Plane = other_407;
    let _e148: MultiVector = self_485;
    let _e152: Plane = other_407;
    let _e155: Plane = other_407;
    let _e158: Plane = other_407;
    let _e161: Plane = other_407;
    let _e174: MultiVector = self_485;
    let _e176: Plane = other_407;
    return MultiVector(((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y, _e11.g0_.y, _e14.g0_.y, _e17.g0_.z)) * vec4<f32>(0.0, 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e29.g0_.w) * vec4<f32>(_e33.g0_.y, _e36.g0_.z, _e39.g0_.y, _e42.g0_.y)) * vec4<f32>(1.0, -(1.0), 0.0, 0.0))) + (_e55.g0_.zzxx * vec4<f32>(_e58.g0_.z, _e61.g0_.y, _e64.g0_.z, _e67.g0_.y))), ((((((vec4<f32>(_e73.g1_.x) * vec4<f32>(_e77.g0_.z, _e80.g0_.z, _e83.g0_.z, _e86.g0_.y)) * vec4<f32>(0.0, 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e98.g1_.y) * vec4<f32>(_e102.g0_.y, _e105.g0_.y, _e108.g0_.y, _e111.g0_.z)) * vec4<f32>(0.0, 0.0, 1.0, 1.0))) + ((vec4<f32>(_e123.g1_.z) * vec4<f32>(_e127.g0_.z, _e130.g0_.y, _e133.g0_.z, _e136.g0_.z)) * vec4<f32>(1.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e148.g1_.w) * vec4<f32>(_e152.g0_.y, _e155.g0_.z, _e158.g0_.y, _e161.g0_.y)) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))) + ((_e174.g0_ * vec4<f32>(_e176.g0_.x)) * vec4<f32>(1.0, 1.0, -(1.0), 1.0))));
}

fn multi_vector_plane_geometric_anti_product(self_486: MultiVector, other_408: Plane) -> MultiVector {
    var self_487: MultiVector;
    var other_409: Plane;

    self_487 = self_486;
    other_409 = other_408;
    let _e4: MultiVector = self_487;
    let _e8: Plane = other_409;
    let _e11: Plane = other_409;
    let _e14: Plane = other_409;
    let _e17: Plane = other_409;
    let _e29: MultiVector = self_487;
    let _e33: Plane = other_409;
    let _e36: Plane = other_409;
    let _e39: Plane = other_409;
    let _e42: Plane = other_409;
    let _e54: MultiVector = self_487;
    let _e58: Plane = other_409;
    let _e61: Plane = other_409;
    let _e64: Plane = other_409;
    let _e67: Plane = other_409;
    let _e79: MultiVector = self_487;
    let _e83: Plane = other_409;
    let _e86: Plane = other_409;
    let _e89: Plane = other_409;
    let _e92: Plane = other_409;
    let _e105: MultiVector = self_487;
    let _e108: Plane = other_409;
    let _e122: MultiVector = self_487;
    let _e125: Plane = other_409;
    return MultiVector(((((((vec4<f32>(_e4.g1_.x) * vec4<f32>(_e8.g0_.y, _e11.g0_.y, _e14.g0_.y, _e17.g0_.z)) * vec4<f32>(0.0, 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e29.g1_.y) * vec4<f32>(_e33.g0_.z, _e36.g0_.z, _e39.g0_.z, _e42.g0_.y)) * vec4<f32>(0.0, 0.0, 1.0, 1.0))) + ((vec4<f32>(_e54.g1_.z) * vec4<f32>(_e58.g0_.y, _e61.g0_.z, _e64.g0_.y, _e67.g0_.y)) * vec4<f32>(1.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e79.g1_.w) * vec4<f32>(_e83.g0_.z, _e86.g0_.y, _e89.g0_.z, _e92.g0_.z)) * vec4<f32>(1.0, -(1.0), 0.0, 0.0))) + ((_e105.g0_.yxwz * vec4<f32>(_e108.g0_.x)) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0))), ((_e122.g1_.yxwz * vec4<f32>(_e125.g0_.x)) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0))));
}

fn multi_vector_plane_scalar_product(self_488: MultiVector, other_410: Plane) -> Scalar {
    var self_489: MultiVector;
    var other_411: Plane;

    self_489 = self_488;
    other_411 = other_410;
    let _e4: MultiVector = self_489;
    let _e7: Plane = other_411;
    let _e11: MultiVector = self_489;
    let _e14: Plane = other_411;
    return Scalar(((_e4.g0_.z * _e7.g0_.z) + (_e11.g0_.w * _e14.g0_.y)));
}

fn multi_vector_plane_anti_scalar_product(self_490: MultiVector, other_412: Plane) -> AntiScalar {
    var self_491: MultiVector;
    var other_413: Plane;

    self_491 = self_490;
    other_413 = other_412;
    let _e5: MultiVector = self_491;
    let _e8: Plane = other_413;
    return AntiScalar((0.0 - (_e5.g1_.x * _e8.g0_.x)));
}

fn multi_vector_translator_into(self_492: MultiVector) -> Translator {
    var self_493: MultiVector;

    self_493 = self_492;
    let _e2: MultiVector = self_493;
    let _e5: MultiVector = self_493;
    let _e8: MultiVector = self_493;
    return Translator(vec3<f32>(_e2.g0_.x, _e5.g1_.z, _e8.g1_.w));
}

fn multi_vector_translator_add(self_494: MultiVector, other_414: Translator) -> MultiVector {
    var self_495: MultiVector;
    var other_415: Translator;

    self_495 = self_494;
    other_415 = other_414;
    let _e4: MultiVector = self_495;
    let _e6: Translator = other_415;
    let _e17: MultiVector = self_495;
    let _e19: Translator = other_415;
    let _e22: Translator = other_415;
    let _e25: Translator = other_415;
    let _e28: Translator = other_415;
    return MultiVector((_e4.g0_ + (vec4<f32>(_e6.g0_.x) * vec4<f32>(1.0, 0.0, 0.0, 0.0))), (_e17.g1_ + (vec4<f32>(_e19.g0_.x, _e22.g0_.x, _e25.g0_.y, _e28.g0_.z) * vec4<f32>(0.0, 0.0, 1.0, 1.0))));
}

fn multi_vector_translator_sub(self_496: MultiVector, other_416: Translator) -> MultiVector {
    var self_497: MultiVector;
    var other_417: Translator;

    self_497 = self_496;
    other_417 = other_416;
    let _e4: MultiVector = self_497;
    let _e6: Translator = other_417;
    let _e17: MultiVector = self_497;
    let _e19: Translator = other_417;
    let _e22: Translator = other_417;
    let _e25: Translator = other_417;
    let _e28: Translator = other_417;
    return MultiVector((_e4.g0_ - (vec4<f32>(_e6.g0_.x) * vec4<f32>(1.0, 0.0, 0.0, 0.0))), (_e17.g1_ - (vec4<f32>(_e19.g0_.x, _e22.g0_.x, _e25.g0_.y, _e28.g0_.z) * vec4<f32>(0.0, 0.0, 1.0, 1.0))));
}

fn multi_vector_translator_geometric_product(self_498: MultiVector, other_418: Translator) -> MultiVector {
    var self_499: MultiVector;
    var other_419: Translator;

    self_499 = self_498;
    other_419 = other_418;
    let _e4: MultiVector = self_499;
    let _e6: Translator = other_419;
    let _e11: MultiVector = self_499;
    let _e15: Translator = other_419;
    let _e18: Translator = other_419;
    let _e21: Translator = other_419;
    let _e24: Translator = other_419;
    let _e36: MultiVector = self_499;
    let _e40: Translator = other_419;
    let _e43: Translator = other_419;
    let _e46: Translator = other_419;
    let _e49: Translator = other_419;
    let _e61: MultiVector = self_499;
    let _e65: Translator = other_419;
    let _e77: MultiVector = self_499;
    let _e81: Translator = other_419;
    let _e93: MultiVector = self_499;
    let _e97: Translator = other_419;
    let _e109: MultiVector = self_499;
    let _e113: Translator = other_419;
    let _e125: MultiVector = self_499;
    let _e128: Translator = other_419;
    let _e131: Translator = other_419;
    let _e134: Translator = other_419;
    let _e137: Translator = other_419;
    return MultiVector((_e4.g0_ * vec4<f32>(_e6.g0_.x)), ((((((((vec4<f32>(_e11.g0_.y) * vec4<f32>(_e15.g0_.z, _e18.g0_.z, _e21.g0_.z, _e24.g0_.y)) * vec4<f32>(0.0, 0.0, -(1.0), 1.0)) + ((vec4<f32>(_e36.g0_.w) * vec4<f32>(_e40.g0_.z, _e43.g0_.y, _e46.g0_.z, _e49.g0_.z)) * vec4<f32>(1.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e61.g1_.x) * vec4<f32>(_e65.g0_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e77.g1_.y) * vec4<f32>(_e81.g0_.x)) * vec4<f32>(0.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e93.g1_.z) * vec4<f32>(_e97.g0_.x)) * vec4<f32>(0.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e109.g1_.w) * vec4<f32>(_e113.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((_e125.g0_.zzxx * vec4<f32>(_e128.g0_.y, _e131.g0_.z, _e134.g0_.y, _e137.g0_.z)) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0))));
}

fn multi_vector_translator_outer_product(self_500: MultiVector, other_420: Translator) -> MultiVector {
    var self_501: MultiVector;
    var other_421: Translator;

    self_501 = self_500;
    other_421 = other_420;
    let _e4: MultiVector = self_501;
    let _e6: Translator = other_421;
    let _e11: MultiVector = self_501;
    let _e15: Translator = other_421;
    let _e26: MultiVector = self_501;
    let _e30: Translator = other_421;
    let _e42: MultiVector = self_501;
    let _e46: Translator = other_421;
    let _e58: MultiVector = self_501;
    let _e62: Translator = other_421;
    let _e74: MultiVector = self_501;
    let _e77: MultiVector = self_501;
    let _e80: MultiVector = self_501;
    let _e83: MultiVector = self_501;
    let _e87: Translator = other_421;
    let _e90: Translator = other_421;
    let _e93: Translator = other_421;
    let _e96: Translator = other_421;
    return MultiVector((_e4.g0_ * vec4<f32>(_e6.g0_.x)), ((((((vec4<f32>(_e11.g0_.w) * vec4<f32>(_e15.g0_.y)) * vec4<f32>(0.0, 1.0, 0.0, 0.0)) + ((vec4<f32>(_e26.g1_.y) * vec4<f32>(_e30.g0_.x)) * vec4<f32>(0.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e42.g1_.z) * vec4<f32>(_e46.g0_.x)) * vec4<f32>(0.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e58.g1_.w) * vec4<f32>(_e62.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + (vec4<f32>(_e74.g1_.x, _e77.g0_.z, _e80.g0_.x, _e83.g0_.x) * vec4<f32>(_e87.g0_.x, _e90.g0_.z, _e93.g0_.y, _e96.g0_.z))));
}

fn multi_vector_translator_inner_product(self_502: MultiVector, other_422: Translator) -> MultiVector {
    var self_503: MultiVector;
    var other_423: Translator;

    self_503 = self_502;
    other_423 = other_422;
    let _e4: MultiVector = self_503;
    let _e6: Translator = other_423;
    let _e11: MultiVector = self_503;
    let _e15: Translator = other_423;
    let _e26: MultiVector = self_503;
    let _e30: Translator = other_423;
    let _e42: MultiVector = self_503;
    let _e46: Translator = other_423;
    let _e58: MultiVector = self_503;
    let _e62: Translator = other_423;
    let _e74: MultiVector = self_503;
    let _e77: MultiVector = self_503;
    let _e80: MultiVector = self_503;
    let _e83: MultiVector = self_503;
    let _e87: Translator = other_423;
    let _e90: Translator = other_423;
    let _e93: Translator = other_423;
    let _e96: Translator = other_423;
    return MultiVector((_e4.g0_ * vec4<f32>(_e6.g0_.x)), ((((((vec4<f32>(_e11.g0_.w) * vec4<f32>(_e15.g0_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) + ((vec4<f32>(_e26.g1_.x) * vec4<f32>(_e30.g0_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e42.g1_.z) * vec4<f32>(_e46.g0_.x)) * vec4<f32>(0.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e58.g1_.w) * vec4<f32>(_e62.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e74.g0_.z, _e77.g1_.y, _e80.g0_.x, _e83.g0_.x) * vec4<f32>(_e87.g0_.y, _e90.g0_.x, _e93.g0_.y, _e96.g0_.z)) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0))));
}

fn multi_vector_translator_geometric_anti_product(self_504: MultiVector, other_424: Translator) -> MultiVector {
    var self_505: MultiVector;
    var other_425: Translator;

    self_505 = self_504;
    other_425 = other_424;
    let _e4: MultiVector = self_505;
    let _e8: Translator = other_425;
    let _e11: Translator = other_425;
    let _e14: Translator = other_425;
    let _e17: Translator = other_425;
    let _e29: MultiVector = self_505;
    let _e33: Translator = other_425;
    let _e36: Translator = other_425;
    let _e39: Translator = other_425;
    let _e42: Translator = other_425;
    let _e54: MultiVector = self_505;
    let _e58: Translator = other_425;
    let _e71: MultiVector = self_505;
    let _e75: Translator = other_425;
    let _e87: MultiVector = self_505;
    let _e91: Translator = other_425;
    let _e103: MultiVector = self_505;
    let _e107: Translator = other_425;
    let _e119: MultiVector = self_505;
    let _e122: Translator = other_425;
    let _e125: Translator = other_425;
    let _e128: Translator = other_425;
    let _e131: Translator = other_425;
    let _e144: MultiVector = self_505;
    let _e148: Translator = other_425;
    let _e151: Translator = other_425;
    let _e154: Translator = other_425;
    let _e157: Translator = other_425;
    let _e168: MultiVector = self_505;
    let _e172: Translator = other_425;
    let _e175: Translator = other_425;
    let _e178: Translator = other_425;
    let _e181: Translator = other_425;
    let _e193: MultiVector = self_505;
    let _e196: Translator = other_425;
    let _e199: Translator = other_425;
    let _e202: Translator = other_425;
    let _e205: Translator = other_425;
    return MultiVector(((((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y, _e11.g0_.y, _e14.g0_.y, _e17.g0_.z)) * vec4<f32>(0.0, 0.0, -(1.0), 1.0)) + ((vec4<f32>(_e29.g0_.w) * vec4<f32>(_e33.g0_.y, _e36.g0_.z, _e39.g0_.y, _e42.g0_.y)) * vec4<f32>(1.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e54.g1_.x) * vec4<f32>(_e58.g0_.x)) * vec4<f32>(0.0, -(1.0), 0.0, 0.0))) + ((vec4<f32>(_e71.g1_.y) * vec4<f32>(_e75.g0_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e87.g1_.z) * vec4<f32>(_e91.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e103.g1_.w) * vec4<f32>(_e107.g0_.x)) * vec4<f32>(0.0, 0.0, 1.0, 0.0))) + ((_e119.g0_.zzxx * vec4<f32>(_e122.g0_.z, _e125.g0_.y, _e128.g0_.z, _e131.g0_.y)) * vec4<f32>(1.0, -(1.0), 1.0, 1.0))), ((((vec4<f32>(_e144.g1_.y) * vec4<f32>(_e148.g0_.y, _e151.g0_.y, _e154.g0_.y, _e157.g0_.z)) * vec4<f32>(0.0, 0.0, 1.0, 1.0)) + ((vec4<f32>(_e168.g1_.w) * vec4<f32>(_e172.g0_.y, _e175.g0_.z, _e178.g0_.y, _e181.g0_.y)) * vec4<f32>(1.0, 1.0, 0.0, 0.0))) + ((_e193.g1_.zzxx * vec4<f32>(_e196.g0_.z, _e199.g0_.y, _e202.g0_.z, _e205.g0_.y)) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))));
}

fn multi_vector_translator_right_contraction(self_506: MultiVector, other_426: Translator) -> MultiVector {
    var self_507: MultiVector;
    var other_427: Translator;

    self_507 = self_506;
    other_427 = other_426;
    let _e4: MultiVector = self_507;
    let _e6: Translator = other_427;
    let _e11: MultiVector = self_507;
    let _e13: Translator = other_427;
    return MultiVector((_e4.g0_ * vec4<f32>(_e6.g0_.x)), (_e11.g1_ * vec4<f32>(_e13.g0_.x)));
}

fn multi_vector_translator_scalar_product(self_508: MultiVector, other_428: Translator) -> Scalar {
    var self_509: MultiVector;
    var other_429: Translator;

    self_509 = self_508;
    other_429 = other_428;
    let _e4: MultiVector = self_509;
    let _e7: Translator = other_429;
    return Scalar((_e4.g0_.x * _e7.g0_.x));
}

fn multi_vector_translator_anti_scalar_product(self_510: MultiVector, other_430: Translator) -> AntiScalar {
    var self_511: MultiVector;
    var other_431: Translator;

    self_511 = self_510;
    other_431 = other_430;
    let _e4: MultiVector = self_511;
    let _e7: Translator = other_431;
    let _e11: MultiVector = self_511;
    let _e14: Translator = other_431;
    return AntiScalar(((_e4.g1_.z * _e7.g0_.y) + (_e11.g1_.w * _e14.g0_.z)));
}

fn multi_vector_motor_into(self_512: MultiVector) -> Motor {
    var self_513: MultiVector;

    self_513 = self_512;
    let _e2: MultiVector = self_513;
    let _e5: MultiVector = self_513;
    let _e8: MultiVector = self_513;
    let _e11: MultiVector = self_513;
    return Motor(vec4<f32>(_e2.g0_.x, _e5.g0_.y, _e8.g1_.z, _e11.g1_.w));
}

fn multi_vector_motor_add(self_514: MultiVector, other_432: Motor) -> MultiVector {
    var self_515: MultiVector;
    var other_433: Motor;

    self_515 = self_514;
    other_433 = other_432;
    let _e4: MultiVector = self_515;
    let _e6: Motor = other_433;
    let _e16: MultiVector = self_515;
    let _e18: Motor = other_433;
    return MultiVector((_e4.g0_ + (_e6.g0_.xyxx * vec4<f32>(1.0, 1.0, 0.0, 0.0))), (_e16.g1_ + (_e18.g0_.xxzw * vec4<f32>(0.0, 0.0, 1.0, 1.0))));
}

fn multi_vector_motor_sub(self_516: MultiVector, other_434: Motor) -> MultiVector {
    var self_517: MultiVector;
    var other_435: Motor;

    self_517 = self_516;
    other_435 = other_434;
    let _e4: MultiVector = self_517;
    let _e6: Motor = other_435;
    let _e16: MultiVector = self_517;
    let _e18: Motor = other_435;
    return MultiVector((_e4.g0_ - (_e6.g0_.xyxx * vec4<f32>(1.0, 1.0, 0.0, 0.0))), (_e16.g1_ - (_e18.g0_.xxzw * vec4<f32>(0.0, 0.0, 1.0, 1.0))));
}

fn multi_vector_motor_geometric_product(self_518: MultiVector, other_436: Motor) -> MultiVector {
    var self_519: MultiVector;
    var other_437: Motor;

    self_519 = self_518;
    other_437 = other_436;
    let _e4: MultiVector = self_519;
    let _e8: Motor = other_437;
    let _e19: MultiVector = self_519;
    let _e23: Motor = other_437;
    let _e35: MultiVector = self_519;
    let _e38: Motor = other_437;
    let _e43: MultiVector = self_519;
    let _e47: Motor = other_437;
    let _e58: MultiVector = self_519;
    let _e62: Motor = other_437;
    let _e73: MultiVector = self_519;
    let _e77: Motor = other_437;
    let _e88: MultiVector = self_519;
    let _e92: Motor = other_437;
    let _e104: MultiVector = self_519;
    let _e108: Motor = other_437;
    let _e120: MultiVector = self_519;
    let _e124: Motor = other_437;
    let _e135: MultiVector = self_519;
    let _e138: Motor = other_437;
    return MultiVector(((((vec4<f32>(_e4.g0_.y) * _e8.g0_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0)) + ((vec4<f32>(_e19.g0_.w) * _e23.g0_.yyyx) * vec4<f32>(0.0, 0.0, -(1.0), 1.0))) + (_e35.g0_.xxzz * _e38.g0_.xyxy)), ((((((((vec4<f32>(_e43.g0_.y) * _e47.g0_.wwwz) * vec4<f32>(0.0, 0.0, -(1.0), 1.0)) + ((vec4<f32>(_e58.g0_.w) * _e62.g0_.wzww) * vec4<f32>(1.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e73.g1_.x) * _e77.g0_.xyxx) * vec4<f32>(1.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e88.g1_.y) * _e92.g0_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))) + ((vec4<f32>(_e104.g1_.z) * _e108.g0_.xxxy) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e120.g1_.w) * _e124.g0_.yyyx) * vec4<f32>(0.0, 0.0, 1.0, 1.0))) + ((_e135.g0_.zzxx * _e138.g0_.zwzw) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0))));
}

fn multi_vector_motor_outer_product(self_520: MultiVector, other_438: Motor) -> MultiVector {
    var self_521: MultiVector;
    var other_439: Motor;

    self_521 = self_520;
    other_439 = other_438;
    let _e4: MultiVector = self_521;
    let _e8: Motor = other_439;
    let _e19: MultiVector = self_521;
    let _e22: Motor = other_439;
    let _e27: MultiVector = self_521;
    let _e31: Motor = other_439;
    let _e42: MultiVector = self_521;
    let _e46: Motor = other_439;
    let _e57: MultiVector = self_521;
    let _e61: Motor = other_439;
    let _e73: MultiVector = self_521;
    let _e77: Motor = other_439;
    let _e89: MultiVector = self_521;
    let _e93: Motor = other_439;
    let _e105: MultiVector = self_521;
    let _e108: Motor = other_439;
    return MultiVector((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.x)) * vec4<f32>(0.0, 1.0, 0.0, 0.0)) + (_e19.g0_.xxzw * _e22.g0_.xyxx)), (((((((vec4<f32>(_e27.g0_.w) * vec4<f32>(_e31.g0_.z)) * vec4<f32>(0.0, 1.0, 0.0, 0.0)) + ((vec4<f32>(_e42.g1_.x) * _e46.g0_.xyxx) * vec4<f32>(1.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e57.g1_.y) * vec4<f32>(_e61.g0_.x)) * vec4<f32>(0.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e73.g1_.z) * vec4<f32>(_e77.g0_.x)) * vec4<f32>(0.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e89.g1_.w) * vec4<f32>(_e93.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((_e105.g0_.xzxx * _e108.g0_.xwzw) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn multi_vector_motor_inner_product(self_522: MultiVector, other_440: Motor) -> MultiVector {
    var self_523: MultiVector;
    var other_441: Motor;

    self_523 = self_522;
    other_441 = other_440;
    let _e4: MultiVector = self_523;
    let _e8: Motor = other_441;
    let _e19: MultiVector = self_523;
    let _e23: Motor = other_441;
    let _e35: MultiVector = self_523;
    let _e38: Motor = other_441;
    let _e43: MultiVector = self_523;
    let _e47: Motor = other_441;
    let _e58: MultiVector = self_523;
    let _e62: Motor = other_441;
    let _e74: MultiVector = self_523;
    let _e78: Motor = other_441;
    let _e90: MultiVector = self_523;
    let _e94: Motor = other_441;
    let _e106: MultiVector = self_523;
    let _e110: Motor = other_441;
    let _e122: MultiVector = self_523;
    let _e125: Motor = other_441;
    return MultiVector(((((vec4<f32>(_e4.g0_.y) * _e8.g0_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0)) + ((vec4<f32>(_e19.g0_.w) * _e23.g0_.yyyx) * vec4<f32>(0.0, 0.0, -(1.0), 1.0))) + (_e35.g0_.xxzz * _e38.g0_.xyxy)), (((((((vec4<f32>(_e43.g0_.w) * vec4<f32>(_e47.g0_.w)) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) + ((vec4<f32>(_e58.g1_.x) * vec4<f32>(_e62.g0_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e74.g1_.y) * _e78.g0_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))) + ((vec4<f32>(_e90.g1_.z) * vec4<f32>(_e94.g0_.x)) * vec4<f32>(0.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e106.g1_.w) * vec4<f32>(_e110.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((_e122.g0_.zxxx * _e125.g0_.zxzw) * vec4<f32>(-(1.0), 0.0, 1.0, 1.0))));
}

fn multi_vector_motor_geometric_anti_product(self_524: MultiVector, other_442: Motor) -> MultiVector {
    var self_525: MultiVector;
    var other_443: Motor;

    self_525 = self_524;
    other_443 = other_442;
    let _e4: MultiVector = self_525;
    let _e8: Motor = other_443;
    let _e19: MultiVector = self_525;
    let _e23: Motor = other_443;
    let _e34: MultiVector = self_525;
    let _e38: Motor = other_443;
    let _e50: MultiVector = self_525;
    let _e54: Motor = other_443;
    let _e65: MultiVector = self_525;
    let _e69: Motor = other_443;
    let _e80: MultiVector = self_525;
    let _e84: Motor = other_443;
    let _e96: MultiVector = self_525;
    let _e99: Motor = other_443;
    let _e111: MultiVector = self_525;
    let _e115: Motor = other_443;
    let _e125: MultiVector = self_525;
    let _e129: Motor = other_443;
    let _e140: MultiVector = self_525;
    let _e143: Motor = other_443;
    return MultiVector(((((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.zzzw) * vec4<f32>(0.0, 0.0, -(1.0), 1.0)) + ((vec4<f32>(_e19.g0_.w) * _e23.g0_.zwzz) * vec4<f32>(1.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e34.g1_.x) * _e38.g0_.yxyy) * vec4<f32>(1.0, -(1.0), 0.0, 0.0))) + ((vec4<f32>(_e50.g1_.y) * _e54.g0_.xyxx) * vec4<f32>(1.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e65.g1_.z) * _e69.g0_.yyyx) * vec4<f32>(0.0, 0.0, 1.0, 1.0))) + ((vec4<f32>(_e80.g1_.w) * _e84.g0_.xxxy) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((_e96.g0_.zzxx * _e99.g0_.wzwz) * vec4<f32>(1.0, -(1.0), 1.0, 1.0))), ((((vec4<f32>(_e111.g1_.y) * _e115.g0_.zzzw) * vec4<f32>(0.0, 0.0, 1.0, 1.0)) + ((vec4<f32>(_e125.g1_.w) * _e129.g0_.zwzz) * vec4<f32>(1.0, 1.0, 0.0, 0.0))) + ((_e140.g1_.zzxx * _e143.g0_.wzwz) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))));
}

fn multi_vector_motor_right_contraction(self_526: MultiVector, other_444: Motor) -> MultiVector {
    var self_527: MultiVector;
    var other_445: Motor;

    self_527 = self_526;
    other_445 = other_444;
    let _e4: MultiVector = self_527;
    let _e8: Motor = other_445;
    let _e19: MultiVector = self_527;
    let _e22: Motor = other_445;
    let _e34: MultiVector = self_527;
    let _e38: Motor = other_445;
    let _e49: MultiVector = self_527;
    let _e52: Motor = other_445;
    return MultiVector((((vec4<f32>(_e4.g0_.y) * _e8.g0_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0)) + ((_e19.g0_.xxzw * vec4<f32>(_e22.g0_.x)) * vec4<f32>(1.0, 0.0, 1.0, 1.0))), (((vec4<f32>(_e34.g1_.y) * _e38.g0_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0)) + ((_e49.g1_.xxzw * vec4<f32>(_e52.g0_.x)) * vec4<f32>(1.0, 0.0, 1.0, 1.0))));
}

fn multi_vector_motor_scalar_product(self_528: MultiVector, other_446: Motor) -> Scalar {
    var self_529: MultiVector;
    var other_447: Motor;

    self_529 = self_528;
    other_447 = other_446;
    let _e4: MultiVector = self_529;
    let _e7: Motor = other_447;
    let _e11: MultiVector = self_529;
    let _e14: Motor = other_447;
    return Scalar(((_e4.g0_.x * _e7.g0_.x) - (_e11.g0_.y * _e14.g0_.y)));
}

fn multi_vector_motor_anti_scalar_product(self_530: MultiVector, other_448: Motor) -> AntiScalar {
    var self_531: MultiVector;
    var other_449: Motor;

    self_531 = self_530;
    other_449 = other_448;
    let _e4: MultiVector = self_531;
    let _e7: Motor = other_449;
    let _e11: MultiVector = self_531;
    let _e14: Motor = other_449;
    return AntiScalar(((_e4.g1_.z * _e7.g0_.z) + (_e11.g1_.w * _e14.g0_.w)));
}

fn multi_vector_motor_dual_into(self_532: MultiVector) -> MotorDual {
    var self_533: MultiVector;

    self_533 = self_532;
    let _e2: MultiVector = self_533;
    let _e5: MultiVector = self_533;
    let _e8: MultiVector = self_533;
    let _e11: MultiVector = self_533;
    return MotorDual(vec4<f32>(_e2.g1_.y, _e5.g1_.x, _e8.g0_.w, _e11.g0_.z));
}

fn multi_vector_motor_dual_add(self_534: MultiVector, other_450: MotorDual) -> MultiVector {
    var self_535: MultiVector;
    var other_451: MotorDual;

    self_535 = self_534;
    other_451 = other_450;
    let _e4: MultiVector = self_535;
    let _e6: MotorDual = other_451;
    let _e16: MultiVector = self_535;
    let _e18: MotorDual = other_451;
    return MultiVector((_e4.g0_ + (_e6.g0_.xxwz * vec4<f32>(0.0, 0.0, 1.0, 1.0))), (_e16.g1_ + (_e18.g0_.yxxx * vec4<f32>(1.0, 1.0, 0.0, 0.0))));
}

fn multi_vector_motor_dual_sub(self_536: MultiVector, other_452: MotorDual) -> MultiVector {
    var self_537: MultiVector;
    var other_453: MotorDual;

    self_537 = self_536;
    other_453 = other_452;
    let _e4: MultiVector = self_537;
    let _e6: MotorDual = other_453;
    let _e16: MultiVector = self_537;
    let _e18: MotorDual = other_453;
    return MultiVector((_e4.g0_ - (_e6.g0_.xxwz * vec4<f32>(0.0, 0.0, 1.0, 1.0))), (_e16.g1_ - (_e18.g0_.yxxx * vec4<f32>(1.0, 1.0, 0.0, 0.0))));
}

fn multi_vector_motor_dual_geometric_product(self_538: MultiVector, other_454: MotorDual) -> MultiVector {
    var self_539: MultiVector;
    var other_455: MotorDual;

    self_539 = self_538;
    other_455 = other_454;
    let _e4: MultiVector = self_539;
    let _e8: MotorDual = other_455;
    let _e19: MultiVector = self_539;
    let _e23: MotorDual = other_455;
    let _e35: MultiVector = self_539;
    let _e38: MotorDual = other_455;
    let _e43: MultiVector = self_539;
    let _e47: MotorDual = other_455;
    let _e58: MultiVector = self_539;
    let _e62: MotorDual = other_455;
    let _e73: MultiVector = self_539;
    let _e77: MotorDual = other_455;
    let _e89: MultiVector = self_539;
    let _e93: MotorDual = other_455;
    let _e104: MultiVector = self_539;
    let _e108: MotorDual = other_455;
    let _e119: MultiVector = self_539;
    let _e123: MotorDual = other_455;
    let _e135: MultiVector = self_539;
    let _e138: MotorDual = other_455;
    return MultiVector(((((vec4<f32>(_e4.g0_.y) * _e8.g0_.zzzw) * vec4<f32>(0.0, 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e19.g0_.w) * _e23.g0_.zwzz) * vec4<f32>(1.0, -(1.0), 0.0, 0.0))) + (_e35.g0_.zzxx * _e38.g0_.wzwz)), ((((((((vec4<f32>(_e43.g0_.y) * _e47.g0_.xyxx) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0)) + ((vec4<f32>(_e58.g0_.w) * _e62.g0_.xxxy) * vec4<f32>(0.0, 0.0, 1.0, 1.0))) + ((vec4<f32>(_e73.g1_.x) * _e77.g0_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e89.g1_.y) * _e93.g0_.zzzw) * vec4<f32>(0.0, 0.0, 1.0, 1.0))) + ((vec4<f32>(_e104.g1_.z) * _e108.g0_.wzww) * vec4<f32>(1.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e119.g1_.w) * _e123.g0_.zwzz) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))) + ((_e135.g0_.xxzz * _e138.g0_.yxyx) * vec4<f32>(1.0, 1.0, -(1.0), 1.0))));
}

fn multi_vector_motor_dual_regressive_product(self_540: MultiVector, other_456: MotorDual) -> MultiVector {
    var self_541: MultiVector;
    var other_457: MotorDual;

    self_541 = self_540;
    other_457 = other_456;
    let _e4: MultiVector = self_541;
    let _e8: MotorDual = other_457;
    let _e18: MultiVector = self_541;
    let _e22: MotorDual = other_457;
    let _e33: MultiVector = self_541;
    let _e37: MotorDual = other_457;
    let _e49: MultiVector = self_541;
    let _e53: MotorDual = other_457;
    let _e65: MultiVector = self_541;
    let _e68: MotorDual = other_457;
    let _e80: MultiVector = self_541;
    let _e84: MotorDual = other_457;
    let _e94: MultiVector = self_541;
    let _e97: MotorDual = other_457;
    return MultiVector(((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.yxyy) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) + ((vec4<f32>(_e18.g1_.y) * _e22.g0_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, 1.0))) + ((vec4<f32>(_e33.g1_.z) * vec4<f32>(_e37.g0_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e49.g1_.w) * vec4<f32>(_e53.g0_.w)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((_e65.g0_.xxzw * vec4<f32>(_e68.g0_.x)) * vec4<f32>(1.0, 0.0, 1.0, 1.0))), (((vec4<f32>(_e80.g1_.y) * _e84.g0_.yxyy) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) + ((_e94.g1_.xxzw * vec4<f32>(_e97.g0_.x)) * vec4<f32>(1.0, 0.0, 1.0, 1.0))));
}

fn multi_vector_motor_dual_geometric_anti_product(self_542: MultiVector, other_458: MotorDual) -> MultiVector {
    var self_543: MultiVector;
    var other_459: MotorDual;

    self_543 = self_542;
    other_459 = other_458;
    let _e4: MultiVector = self_543;
    let _e8: MotorDual = other_459;
    let _e18: MultiVector = self_543;
    let _e22: MotorDual = other_459;
    let _e34: MultiVector = self_543;
    let _e38: MotorDual = other_459;
    let _e50: MultiVector = self_543;
    let _e54: MotorDual = other_459;
    let _e65: MultiVector = self_543;
    let _e69: MotorDual = other_459;
    let _e80: MultiVector = self_543;
    let _e84: MotorDual = other_459;
    let _e96: MultiVector = self_543;
    let _e99: MotorDual = other_459;
    let _e111: MultiVector = self_543;
    let _e115: MotorDual = other_459;
    let _e125: MultiVector = self_543;
    let _e129: MotorDual = other_459;
    let _e140: MultiVector = self_543;
    let _e143: MotorDual = other_459;
    return MultiVector(((((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.yxyy) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) + ((vec4<f32>(_e18.g0_.w) * _e22.g0_.yyyx) * vec4<f32>(0.0, 0.0, -(1.0), 1.0))) + ((vec4<f32>(_e34.g1_.x) * _e38.g0_.zzzw) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e50.g1_.y) * _e54.g0_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, 1.0))) + ((vec4<f32>(_e65.g1_.z) * _e69.g0_.zwzz) * vec4<f32>(1.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e80.g1_.w) * _e84.g0_.wzww) * vec4<f32>(1.0, -(1.0), 0.0, 0.0))) + ((_e96.g0_.xxzz * _e99.g0_.xyxy) * vec4<f32>(1.0, -(1.0), 1.0, 1.0))), ((((vec4<f32>(_e111.g1_.y) * _e115.g0_.yxyy) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) + ((vec4<f32>(_e125.g1_.w) * _e129.g0_.yyyx) * vec4<f32>(0.0, 0.0, 1.0, 1.0))) + ((_e140.g1_.xxzz * _e143.g0_.xyxy) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))));
}

fn multi_vector_motor_dual_inner_anti_product(self_544: MultiVector, other_460: MotorDual) -> MultiVector {
    var self_545: MultiVector;
    var other_461: MotorDual;

    self_545 = self_544;
    other_461 = other_460;
    let _e4: MultiVector = self_545;
    let _e8: MotorDual = other_461;
    let _e19: MultiVector = self_545;
    let _e23: MotorDual = other_461;
    let _e34: MultiVector = self_545;
    let _e38: MotorDual = other_461;
    let _e50: MultiVector = self_545;
    let _e54: MotorDual = other_461;
    let _e67: MultiVector = self_545;
    let _e70: MotorDual = other_461;
    let _e82: MultiVector = self_545;
    let _e86: MotorDual = other_461;
    let _e96: MultiVector = self_545;
    let _e100: MotorDual = other_461;
    let _e111: MultiVector = self_545;
    let _e114: MotorDual = other_461;
    return MultiVector(((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.x)) * vec4<f32>(0.0, 1.0, 0.0, 0.0)) + ((vec4<f32>(_e19.g1_.y) * _e23.g0_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, 1.0))) + ((vec4<f32>(_e34.g1_.z) * vec4<f32>(_e38.g0_.w)) * vec4<f32>(0.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e50.g1_.w) * vec4<f32>(_e54.g0_.z)) * vec4<f32>(0.0, -(1.0), 0.0, 0.0))) + ((_e67.g0_.xxzw * _e70.g0_.xyxx) * vec4<f32>(1.0, -(1.0), 1.0, 1.0))), ((((vec4<f32>(_e82.g1_.y) * _e86.g0_.yxyy) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) + ((vec4<f32>(_e96.g1_.w) * _e100.g0_.yyyx) * vec4<f32>(0.0, 0.0, 1.0, 1.0))) + ((_e111.g1_.xxzz * _e114.g0_.xyxy) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))));
}

fn multi_vector_motor_dual_right_anti_contraction(self_546: MultiVector, other_462: MotorDual) -> MultiVector {
    var self_547: MultiVector;
    var other_463: MotorDual;

    self_547 = self_546;
    other_463 = other_462;
    let _e4: MultiVector = self_547;
    let _e8: MotorDual = other_463;
    let _e19: MultiVector = self_547;
    let _e22: MotorDual = other_463;
    let _e34: MultiVector = self_547;
    let _e38: MotorDual = other_463;
    let _e49: MultiVector = self_547;
    let _e52: MotorDual = other_463;
    return MultiVector((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.x)) * vec4<f32>(0.0, 1.0, 0.0, 0.0)) + ((_e19.g0_.xxzw * _e22.g0_.xyxx) * vec4<f32>(1.0, -(1.0), 1.0, 1.0))), (((vec4<f32>(_e34.g1_.y) * vec4<f32>(_e38.g0_.x)) * vec4<f32>(0.0, 1.0, 0.0, 0.0)) + ((_e49.g1_.xxzw * _e52.g0_.xyxx) * vec4<f32>(1.0, -(1.0), 1.0, 1.0))));
}

fn multi_vector_motor_dual_scalar_product(self_548: MultiVector, other_464: MotorDual) -> Scalar {
    var self_549: MultiVector;
    var other_465: MotorDual;

    self_549 = self_548;
    other_465 = other_464;
    let _e4: MultiVector = self_549;
    let _e7: MotorDual = other_465;
    let _e11: MultiVector = self_549;
    let _e14: MotorDual = other_465;
    return Scalar(((_e4.g0_.z * _e7.g0_.w) + (_e11.g0_.w * _e14.g0_.z)));
}

fn multi_vector_motor_dual_anti_scalar_product(self_550: MultiVector, other_466: MotorDual) -> AntiScalar {
    var self_551: MultiVector;
    var other_467: MotorDual;

    self_551 = self_550;
    other_467 = other_466;
    let _e5: MultiVector = self_551;
    let _e8: MotorDual = other_467;
    let _e13: MultiVector = self_551;
    let _e16: MotorDual = other_467;
    return AntiScalar(((0.0 - (_e5.g1_.x * _e8.g0_.y)) + (_e13.g1_.y * _e16.g0_.x)));
}

fn multi_vector_squared_magnitude(self_552: MultiVector) -> Scalar {
    var self_553: MultiVector;

    self_553 = self_552;
    let _e2: MultiVector = self_553;
    let _e3: MultiVector = self_553;
    let _e4: MultiVector = multi_vector_reversal(_e3);
    let _e5: Scalar = multi_vector_multi_vector_scalar_product(_e2, _e4);
    return _e5;
}

fn multi_vector_magnitude(self_554: MultiVector) -> Scalar {
    var self_555: MultiVector;

    self_555 = self_554;
    let _e2: MultiVector = self_555;
    let _e3: Scalar = multi_vector_squared_magnitude(_e2);
    return Scalar(sqrt(_e3.g0_));
}

fn multi_vector_bulk_norm(self_556: MultiVector) -> Scalar {
    var self_557: MultiVector;

    self_557 = self_556;
    let _e2: MultiVector = self_557;
    let _e3: Scalar = multi_vector_squared_magnitude(_e2);
    return Scalar(sqrt(_e3.g0_));
}

fn multi_vector_squared_anti_magnitude(self_558: MultiVector) -> AntiScalar {
    var self_559: MultiVector;

    self_559 = self_558;
    let _e2: MultiVector = self_559;
    let _e3: MultiVector = self_559;
    let _e4: MultiVector = multi_vector_anti_reversal(_e3);
    let _e5: AntiScalar = multi_vector_multi_vector_anti_scalar_product(_e2, _e4);
    return _e5;
}

fn multi_vector_weight_norm(self_560: MultiVector) -> AntiScalar {
    var self_561: MultiVector;

    self_561 = self_560;
    let _e2: MultiVector = self_561;
    let _e3: AntiScalar = multi_vector_squared_anti_magnitude(_e2);
    return AntiScalar(sqrt(_e3.g0_));
}

fn multi_vector_scale(self_562: MultiVector, other_468: f32) -> MultiVector {
    var self_563: MultiVector;
    var other_469: f32;

    self_563 = self_562;
    other_469 = other_468;
    let _e4: MultiVector = self_563;
    let _e5: f32 = other_469;
    let _e7: MultiVector = multi_vector_scalar_geometric_product(_e4, Scalar(_e5));
    return _e7;
}

fn multi_vector_signum(self_564: MultiVector) -> MultiVector {
    var self_565: MultiVector;

    self_565 = self_564;
    let _e2: MultiVector = self_565;
    let _e3: MultiVector = self_565;
    let _e4: Scalar = multi_vector_magnitude(_e3);
    let _e9: MultiVector = multi_vector_scalar_geometric_product(_e2, Scalar((1.0 / _e4.g0_)));
    return _e9;
}

fn multi_vector_inverse(self_566: MultiVector) -> MultiVector {
    var self_567: MultiVector;

    self_567 = self_566;
    let _e2: MultiVector = self_567;
    let _e3: MultiVector = multi_vector_reversal(_e2);
    let _e4: MultiVector = self_567;
    let _e5: Scalar = multi_vector_squared_magnitude(_e4);
    let _e10: MultiVector = multi_vector_scalar_geometric_product(_e3, Scalar((1.0 / _e5.g0_)));
    return _e10;
}

fn multi_vector_unitize(self_568: MultiVector) -> MultiVector {
    var self_569: MultiVector;

    self_569 = self_568;
    let _e2: MultiVector = self_569;
    let _e3: MultiVector = self_569;
    let _e4: AntiScalar = multi_vector_weight_norm(_e3);
    let _e9: MultiVector = multi_vector_scalar_geometric_product(_e2, Scalar((1.0 / _e4.g0_)));
    return _e9;
}

fn rotor_zero() -> Rotor {
    return Rotor(vec2<f32>(0.0));
}

fn rotor_one() -> Rotor {
    return Rotor(vec2<f32>(1.0, 0.0));
}

fn rotor_neg(self_570: Rotor) -> Rotor {
    var self_571: Rotor;

    self_571 = self_570;
    let _e2: Rotor = self_571;
    return Rotor((_e2.g0_ * vec2<f32>(-(1.0))));
}

fn rotor_automorphism(self_572: Rotor) -> Rotor {
    var self_573: Rotor;

    self_573 = self_572;
    let _e2: Rotor = self_573;
    return Rotor(_e2.g0_);
}

fn rotor_reversal(self_574: Rotor) -> Rotor {
    var self_575: Rotor;

    self_575 = self_574;
    let _e2: Rotor = self_575;
    return Rotor((_e2.g0_ * vec2<f32>(1.0, -(1.0))));
}

fn rotor_conjugation(self_576: Rotor) -> Rotor {
    var self_577: Rotor;

    self_577 = self_576;
    let _e2: Rotor = self_577;
    return Rotor((_e2.g0_ * vec2<f32>(1.0, -(1.0))));
}

fn rotor_anti_reversal(self_578: Rotor) -> Rotor {
    var self_579: Rotor;

    self_579 = self_578;
    let _e2: Rotor = self_579;
    return Rotor((_e2.g0_ * vec2<f32>(1.0, -(1.0))));
}

fn rotor_double_complement(self_580: Rotor) -> Rotor {
    var self_581: Rotor;

    self_581 = self_580;
    let _e2: Rotor = self_581;
    return Rotor(_e2.g0_);
}

fn rotor_scalar_into(self_582: Rotor) -> Scalar {
    var self_583: Rotor;

    self_583 = self_582;
    let _e2: Rotor = self_583;
    return Scalar(_e2.g0_.x);
}

fn rotor_scalar_add(self_584: Rotor, other_470: Scalar) -> Rotor {
    var self_585: Rotor;
    var other_471: Scalar;

    self_585 = self_584;
    other_471 = other_470;
    let _e4: Rotor = self_585;
    let _e6: Scalar = other_471;
    return Rotor((_e4.g0_ + (vec2<f32>(_e6.g0_) * vec2<f32>(1.0, 0.0))));
}

fn rotor_scalar_sub(self_586: Rotor, other_472: Scalar) -> Rotor {
    var self_587: Rotor;
    var other_473: Scalar;

    self_587 = self_586;
    other_473 = other_472;
    let _e4: Rotor = self_587;
    let _e6: Scalar = other_473;
    return Rotor((_e4.g0_ - (vec2<f32>(_e6.g0_) * vec2<f32>(1.0, 0.0))));
}

fn rotor_scalar_geometric_product(self_588: Rotor, other_474: Scalar) -> Rotor {
    var self_589: Rotor;
    var other_475: Scalar;

    self_589 = self_588;
    other_475 = other_474;
    let _e4: Rotor = self_589;
    let _e6: Scalar = other_475;
    return Rotor((_e4.g0_ * vec2<f32>(_e6.g0_)));
}

fn rotor_scalar_outer_product(self_590: Rotor, other_476: Scalar) -> Rotor {
    var self_591: Rotor;
    var other_477: Scalar;

    self_591 = self_590;
    other_477 = other_476;
    let _e4: Rotor = self_591;
    let _e6: Scalar = other_477;
    return Rotor((_e4.g0_ * vec2<f32>(_e6.g0_)));
}

fn rotor_scalar_inner_product(self_592: Rotor, other_478: Scalar) -> Rotor {
    var self_593: Rotor;
    var other_479: Scalar;

    self_593 = self_592;
    other_479 = other_478;
    let _e4: Rotor = self_593;
    let _e6: Scalar = other_479;
    return Rotor((_e4.g0_ * vec2<f32>(_e6.g0_)));
}

fn rotor_scalar_left_contraction(self_594: Rotor, other_480: Scalar) -> Scalar {
    var self_595: Rotor;
    var other_481: Scalar;

    self_595 = self_594;
    other_481 = other_480;
    let _e4: Rotor = self_595;
    let _e7: Scalar = other_481;
    return Scalar((_e4.g0_.x * _e7.g0_));
}

fn rotor_scalar_right_contraction(self_596: Rotor, other_482: Scalar) -> Rotor {
    var self_597: Rotor;
    var other_483: Scalar;

    self_597 = self_596;
    other_483 = other_482;
    let _e4: Rotor = self_597;
    let _e6: Scalar = other_483;
    return Rotor((_e4.g0_ * vec2<f32>(_e6.g0_)));
}

fn rotor_scalar_scalar_product(self_598: Rotor, other_484: Scalar) -> Scalar {
    var self_599: Rotor;
    var other_485: Scalar;

    self_599 = self_598;
    other_485 = other_484;
    let _e4: Rotor = self_599;
    let _e7: Scalar = other_485;
    return Scalar((_e4.g0_.x * _e7.g0_));
}

fn rotor_anti_scalar_regressive_product(self_600: Rotor, other_486: AntiScalar) -> Rotor {
    var self_601: Rotor;
    var other_487: AntiScalar;

    self_601 = self_600;
    other_487 = other_486;
    let _e4: Rotor = self_601;
    let _e6: AntiScalar = other_487;
    return Rotor((_e4.g0_ * vec2<f32>(_e6.g0_)));
}

fn rotor_anti_scalar_outer_product(self_602: Rotor, other_488: AntiScalar) -> AntiScalar {
    var self_603: Rotor;
    var other_489: AntiScalar;

    self_603 = self_602;
    other_489 = other_488;
    let _e4: Rotor = self_603;
    let _e7: AntiScalar = other_489;
    return AntiScalar((_e4.g0_.x * _e7.g0_));
}

fn rotor_anti_scalar_geometric_anti_product(self_604: Rotor, other_490: AntiScalar) -> Rotor {
    var self_605: Rotor;
    var other_491: AntiScalar;

    self_605 = self_604;
    other_491 = other_490;
    let _e4: Rotor = self_605;
    let _e6: AntiScalar = other_491;
    return Rotor((_e4.g0_ * vec2<f32>(_e6.g0_)));
}

fn rotor_anti_scalar_inner_anti_product(self_606: Rotor, other_492: AntiScalar) -> Rotor {
    var self_607: Rotor;
    var other_493: AntiScalar;

    self_607 = self_606;
    other_493 = other_492;
    let _e4: Rotor = self_607;
    let _e6: AntiScalar = other_493;
    return Rotor((_e4.g0_ * vec2<f32>(_e6.g0_)));
}

fn rotor_anti_scalar_right_anti_contraction(self_608: Rotor, other_494: AntiScalar) -> Rotor {
    var self_609: Rotor;
    var other_495: AntiScalar;

    self_609 = self_608;
    other_495 = other_494;
    let _e4: Rotor = self_609;
    let _e6: AntiScalar = other_495;
    return Rotor((_e4.g0_ * vec2<f32>(_e6.g0_)));
}

fn rotor_multi_vector_add(self_610: Rotor, other_496: MultiVector) -> MultiVector {
    var self_611: Rotor;
    var other_497: MultiVector;

    self_611 = self_610;
    other_497 = other_496;
    let _e4: Rotor = self_611;
    let _e7: Rotor = self_611;
    let _e10: Rotor = self_611;
    let _e13: Rotor = self_611;
    let _e23: MultiVector = other_497;
    let _e26: MultiVector = other_497;
    return MultiVector(((vec4<f32>(_e4.g0_.x, _e7.g0_.y, _e10.g0_.x, _e13.g0_.x) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) + _e23.g0_), _e26.g1_);
}

fn rotor_multi_vector_sub(self_612: Rotor, other_498: MultiVector) -> MultiVector {
    var self_613: Rotor;
    var other_499: MultiVector;

    self_613 = self_612;
    other_499 = other_498;
    let _e4: Rotor = self_613;
    let _e7: Rotor = self_613;
    let _e10: Rotor = self_613;
    let _e13: Rotor = self_613;
    let _e23: MultiVector = other_499;
    let _e28: MultiVector = other_499;
    return MultiVector(((vec4<f32>(_e4.g0_.x, _e7.g0_.y, _e10.g0_.x, _e13.g0_.x) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) - _e23.g0_), (vec4<f32>(0.0) - _e28.g1_));
}

fn rotor_multi_vector_geometric_product(self_614: Rotor, other_500: MultiVector) -> MultiVector {
    var self_615: Rotor;
    var other_501: MultiVector;

    self_615 = self_614;
    other_501 = other_500;
    let _e4: Rotor = self_615;
    let _e8: MultiVector = other_501;
    let _e11: Rotor = self_615;
    let _e15: MultiVector = other_501;
    let _e28: Rotor = self_615;
    let _e32: MultiVector = other_501;
    let _e35: Rotor = self_615;
    let _e39: MultiVector = other_501;
    return MultiVector(((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.y) * _e15.g0_.yxwz) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))), ((vec4<f32>(_e28.g0_.x) * _e32.g1_) + ((vec4<f32>(_e35.g0_.y) * _e39.g1_.yxwz) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))));
}

fn rotor_multi_vector_outer_product(self_616: Rotor, other_502: MultiVector) -> MultiVector {
    var self_617: Rotor;
    var other_503: MultiVector;

    self_617 = self_616;
    other_503 = other_502;
    let _e4: Rotor = self_617;
    let _e8: MultiVector = other_503;
    let _e11: Rotor = self_617;
    let _e14: Rotor = self_617;
    let _e17: Rotor = self_617;
    let _e20: Rotor = self_617;
    let _e24: MultiVector = other_503;
    let _e36: Rotor = self_617;
    let _e40: MultiVector = other_503;
    let _e43: Rotor = self_617;
    let _e46: Rotor = self_617;
    let _e49: Rotor = self_617;
    let _e52: Rotor = self_617;
    let _e56: MultiVector = other_503;
    return MultiVector(((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.x, _e14.g0_.y, _e17.g0_.x, _e20.g0_.x) * vec4<f32>(_e24.g0_.x)) * vec4<f32>(0.0, 1.0, 0.0, 0.0))), ((vec4<f32>(_e36.g0_.x) * _e40.g1_) + ((vec4<f32>(_e43.g0_.x, _e46.g0_.y, _e49.g0_.x, _e52.g0_.x) * vec4<f32>(_e56.g1_.x)) * vec4<f32>(0.0, 1.0, 0.0, 0.0))));
}

fn rotor_multi_vector_inner_product(self_618: Rotor, other_504: MultiVector) -> MultiVector {
    var self_619: Rotor;
    var other_505: MultiVector;

    self_619 = self_618;
    other_505 = other_504;
    let _e4: Rotor = self_619;
    let _e8: MultiVector = other_505;
    let _e11: Rotor = self_619;
    let _e15: MultiVector = other_505;
    let _e28: Rotor = self_619;
    let _e32: MultiVector = other_505;
    let _e35: Rotor = self_619;
    let _e38: Rotor = self_619;
    let _e41: Rotor = self_619;
    let _e44: Rotor = self_619;
    let _e48: MultiVector = other_505;
    return MultiVector(((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.y) * _e15.g0_.yxwz) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))), ((vec4<f32>(_e28.g0_.x) * _e32.g1_) + ((vec4<f32>(_e35.g0_.y, _e38.g0_.x, _e41.g0_.x, _e44.g0_.x) * _e48.g1_.yxxx) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))));
}

fn rotor_multi_vector_left_contraction(self_620: Rotor, other_506: MultiVector) -> MultiVector {
    var self_621: Rotor;
    var other_507: MultiVector;

    self_621 = self_620;
    other_507 = other_506;
    let _e4: Rotor = self_621;
    let _e8: MultiVector = other_507;
    let _e11: Rotor = self_621;
    let _e14: Rotor = self_621;
    let _e17: Rotor = self_621;
    let _e20: Rotor = self_621;
    let _e24: MultiVector = other_507;
    let _e36: Rotor = self_621;
    let _e40: MultiVector = other_507;
    let _e43: Rotor = self_621;
    let _e46: Rotor = self_621;
    let _e49: Rotor = self_621;
    let _e52: Rotor = self_621;
    let _e56: MultiVector = other_507;
    return MultiVector(((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.y, _e14.g0_.x, _e17.g0_.x, _e20.g0_.x) * _e24.g0_.yxxx) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))), ((vec4<f32>(_e36.g0_.x) * _e40.g1_) + ((vec4<f32>(_e43.g0_.y, _e46.g0_.x, _e49.g0_.x, _e52.g0_.x) * _e56.g1_.yxxx) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))));
}

fn rotor_multi_vector_scalar_product(self_622: Rotor, other_508: MultiVector) -> Scalar {
    var self_623: Rotor;
    var other_509: MultiVector;

    self_623 = self_622;
    other_509 = other_508;
    let _e4: Rotor = self_623;
    let _e7: MultiVector = other_509;
    let _e11: Rotor = self_623;
    let _e14: MultiVector = other_509;
    return Scalar(((_e4.g0_.x * _e7.g0_.x) - (_e11.g0_.y * _e14.g0_.y)));
}

fn rotor_rotor_add(self_624: Rotor, other_510: Rotor) -> Rotor {
    var self_625: Rotor;
    var other_511: Rotor;

    self_625 = self_624;
    other_511 = other_510;
    let _e4: Rotor = self_625;
    let _e6: Rotor = other_511;
    return Rotor((_e4.g0_ + _e6.g0_));
}

fn rotor_rotor_sub(self_626: Rotor, other_512: Rotor) -> Rotor {
    var self_627: Rotor;
    var other_513: Rotor;

    self_627 = self_626;
    other_513 = other_512;
    let _e4: Rotor = self_627;
    let _e6: Rotor = other_513;
    return Rotor((_e4.g0_ - _e6.g0_));
}

fn rotor_rotor_mul(self_628: Rotor, other_514: Rotor) -> Rotor {
    var self_629: Rotor;
    var other_515: Rotor;

    self_629 = self_628;
    other_515 = other_514;
    let _e4: Rotor = self_629;
    let _e6: Rotor = other_515;
    return Rotor((_e4.g0_ * _e6.g0_));
}

fn rotor_rotor_div(self_630: Rotor, other_516: Rotor) -> Rotor {
    var self_631: Rotor;
    var other_517: Rotor;

    self_631 = self_630;
    other_517 = other_516;
    let _e4: Rotor = self_631;
    let _e7: Rotor = self_631;
    let _e15: Rotor = other_517;
    let _e18: Rotor = other_517;
    return Rotor((((vec2<f32>(_e4.g0_.x, _e7.g0_.y) * vec2<f32>(1.0, 1.0)) / vec2<f32>(_e15.g0_.x, _e18.g0_.y)) * vec2<f32>(1.0, 1.0)));
}

fn rotor_rotor_geometric_product(self_632: Rotor, other_518: Rotor) -> Rotor {
    var self_633: Rotor;
    var other_519: Rotor;

    self_633 = self_632;
    other_519 = other_518;
    let _e4: Rotor = self_633;
    let _e8: Rotor = other_519;
    let _e11: Rotor = self_633;
    let _e15: Rotor = other_519;
    return Rotor(((vec2<f32>(_e4.g0_.x) * _e8.g0_) + ((vec2<f32>(_e11.g0_.y) * _e15.g0_.yx) * vec2<f32>(-(1.0), 1.0))));
}

fn rotor_rotor_outer_product(self_634: Rotor, other_520: Rotor) -> Rotor {
    var self_635: Rotor;
    var other_521: Rotor;

    self_635 = self_634;
    other_521 = other_520;
    let _e4: Rotor = self_635;
    let _e8: Rotor = other_521;
    let _e11: Rotor = self_635;
    let _e13: Rotor = other_521;
    return Rotor(((vec2<f32>(_e4.g0_.x) * _e8.g0_) + ((_e11.g0_ * vec2<f32>(_e13.g0_.x)) * vec2<f32>(0.0, 1.0))));
}

fn rotor_rotor_inner_product(self_636: Rotor, other_522: Rotor) -> Rotor {
    var self_637: Rotor;
    var other_523: Rotor;

    self_637 = self_636;
    other_523 = other_522;
    let _e4: Rotor = self_637;
    let _e8: Rotor = other_523;
    let _e11: Rotor = self_637;
    let _e15: Rotor = other_523;
    return Rotor(((vec2<f32>(_e4.g0_.x) * _e8.g0_) + ((vec2<f32>(_e11.g0_.y) * _e15.g0_.yx) * vec2<f32>(-(1.0), 1.0))));
}

fn rotor_rotor_left_contraction(self_638: Rotor, other_524: Rotor) -> Rotor {
    var self_639: Rotor;
    var other_525: Rotor;

    self_639 = self_638;
    other_525 = other_524;
    let _e4: Rotor = self_639;
    let _e8: Rotor = other_525;
    let _e11: Rotor = self_639;
    let _e14: Rotor = other_525;
    return Rotor(((vec2<f32>(_e4.g0_.x) * _e8.g0_) + ((_e11.g0_.yx * _e14.g0_.yx) * vec2<f32>(-(1.0), 0.0))));
}

fn rotor_rotor_right_contraction(self_640: Rotor, other_526: Rotor) -> Rotor {
    var self_641: Rotor;
    var other_527: Rotor;

    self_641 = self_640;
    other_527 = other_526;
    let _e4: Rotor = self_641;
    let _e8: Rotor = other_527;
    let _e17: Rotor = self_641;
    let _e21: Rotor = other_527;
    return Rotor((((vec2<f32>(_e4.g0_.y) * _e8.g0_.yx) * vec2<f32>(-(1.0), 1.0)) + ((vec2<f32>(_e17.g0_.x) * vec2<f32>(_e21.g0_.x)) * vec2<f32>(1.0, 0.0))));
}

fn rotor_rotor_scalar_product(self_642: Rotor, other_528: Rotor) -> Scalar {
    var self_643: Rotor;
    var other_529: Rotor;

    self_643 = self_642;
    other_529 = other_528;
    let _e4: Rotor = self_643;
    let _e7: Rotor = other_529;
    let _e11: Rotor = self_643;
    let _e14: Rotor = other_529;
    return Scalar(((_e4.g0_.x * _e7.g0_.x) - (_e11.g0_.y * _e14.g0_.y)));
}

fn rotor_point_add(self_644: Rotor, other_530: Point) -> Motor {
    var self_645: Rotor;
    var other_531: Point;

    self_645 = self_644;
    other_531 = other_530;
    let _e4: Rotor = self_645;
    let _e7: Rotor = self_645;
    let _e10: Rotor = self_645;
    let _e13: Rotor = self_645;
    let _e23: Point = other_531;
    let _e26: Point = other_531;
    let _e29: Point = other_531;
    let _e32: Point = other_531;
    return Motor(((vec4<f32>(_e4.g0_.x, _e7.g0_.y, _e10.g0_.x, _e13.g0_.x) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) + (vec4<f32>(_e23.g0_.x, _e26.g0_.x, _e29.g0_.y, _e32.g0_.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn rotor_point_sub(self_646: Rotor, other_532: Point) -> Motor {
    var self_647: Rotor;
    var other_533: Point;

    self_647 = self_646;
    other_533 = other_532;
    let _e4: Rotor = self_647;
    let _e7: Rotor = self_647;
    let _e10: Rotor = self_647;
    let _e13: Rotor = self_647;
    let _e23: Point = other_533;
    let _e26: Point = other_533;
    let _e29: Point = other_533;
    let _e32: Point = other_533;
    return Motor(((vec4<f32>(_e4.g0_.x, _e7.g0_.y, _e10.g0_.x, _e13.g0_.x) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) - (vec4<f32>(_e23.g0_.x, _e26.g0_.x, _e29.g0_.y, _e32.g0_.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn rotor_point_geometric_product(self_648: Rotor, other_534: Point) -> Motor {
    var self_649: Rotor;
    var other_535: Point;

    self_649 = self_648;
    other_535 = other_534;
    let _e4: Rotor = self_649;
    let _e8: Point = other_535;
    let _e11: Point = other_535;
    let _e14: Point = other_535;
    let _e17: Point = other_535;
    let _e30: Rotor = self_649;
    let _e34: Point = other_535;
    let _e37: Point = other_535;
    let _e40: Point = other_535;
    let _e43: Point = other_535;
    return Motor((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.x, _e11.g0_.x, _e14.g0_.z, _e17.g0_.y)) * vec4<f32>(-(1.0), 0.0, -(1.0), 1.0)) + ((vec4<f32>(_e30.g0_.x) * vec4<f32>(_e34.g0_.x, _e37.g0_.x, _e40.g0_.y, _e43.g0_.z)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn rotor_point_outer_product(self_650: Rotor, other_536: Point) -> Point {
    var self_651: Rotor;
    var other_537: Point;

    self_651 = self_650;
    other_537 = other_536;
    let _e4: Rotor = self_651;
    let _e8: Point = other_537;
    return Point((vec3<f32>(_e4.g0_.x) * _e8.g0_));
}

fn rotor_point_inner_product(self_652: Rotor, other_538: Point) -> Motor {
    var self_653: Rotor;
    var other_539: Point;

    self_653 = self_652;
    other_539 = other_538;
    let _e4: Rotor = self_653;
    let _e7: Rotor = self_653;
    let _e10: Rotor = self_653;
    let _e13: Rotor = self_653;
    let _e17: Point = other_539;
    let _e20: Point = other_539;
    let _e23: Point = other_539;
    let _e26: Point = other_539;
    return Motor(((vec4<f32>(_e4.g0_.y, _e7.g0_.x, _e10.g0_.x, _e13.g0_.x) * vec4<f32>(_e17.g0_.x, _e20.g0_.x, _e23.g0_.y, _e26.g0_.z)) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0)));
}

fn rotor_point_left_contraction(self_654: Rotor, other_540: Point) -> Motor {
    var self_655: Rotor;
    var other_541: Point;

    self_655 = self_654;
    other_541 = other_540;
    let _e4: Rotor = self_655;
    let _e7: Rotor = self_655;
    let _e10: Rotor = self_655;
    let _e13: Rotor = self_655;
    let _e17: Point = other_541;
    let _e20: Point = other_541;
    let _e23: Point = other_541;
    let _e26: Point = other_541;
    return Motor(((vec4<f32>(_e4.g0_.y, _e7.g0_.x, _e10.g0_.x, _e13.g0_.x) * vec4<f32>(_e17.g0_.x, _e20.g0_.x, _e23.g0_.y, _e26.g0_.z)) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0)));
}

fn rotor_point_right_contraction(self_656: Rotor, other_542: Point) -> Scalar {
    var self_657: Rotor;
    var other_543: Point;

    self_657 = self_656;
    other_543 = other_542;
    let _e5: Rotor = self_657;
    let _e8: Point = other_543;
    return Scalar((0.0 - (_e5.g0_.y * _e8.g0_.x)));
}

fn rotor_point_scalar_product(self_658: Rotor, other_544: Point) -> Scalar {
    var self_659: Rotor;
    var other_545: Point;

    self_659 = self_658;
    other_545 = other_544;
    let _e5: Rotor = self_659;
    let _e8: Point = other_545;
    return Scalar((0.0 - (_e5.g0_.y * _e8.g0_.x)));
}

fn rotor_ideal_point_add(self_660: Rotor, other_546: IdealPoint) -> Motor {
    var self_661: Rotor;
    var other_547: IdealPoint;

    self_661 = self_660;
    other_547 = other_546;
    let _e4: Rotor = self_661;
    let _e7: Rotor = self_661;
    let _e10: Rotor = self_661;
    let _e13: Rotor = self_661;
    let _e23: IdealPoint = other_547;
    let _e26: IdealPoint = other_547;
    let _e29: IdealPoint = other_547;
    let _e32: IdealPoint = other_547;
    return Motor(((vec4<f32>(_e4.g0_.x, _e7.g0_.y, _e10.g0_.x, _e13.g0_.x) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) + (vec4<f32>(_e23.g0_.x, _e26.g0_.x, _e29.g0_.x, _e32.g0_.y) * vec4<f32>(0.0, 0.0, 1.0, 1.0))));
}

fn rotor_ideal_point_sub(self_662: Rotor, other_548: IdealPoint) -> Motor {
    var self_663: Rotor;
    var other_549: IdealPoint;

    self_663 = self_662;
    other_549 = other_548;
    let _e4: Rotor = self_663;
    let _e7: Rotor = self_663;
    let _e10: Rotor = self_663;
    let _e13: Rotor = self_663;
    let _e23: IdealPoint = other_549;
    let _e26: IdealPoint = other_549;
    let _e29: IdealPoint = other_549;
    let _e32: IdealPoint = other_549;
    return Motor(((vec4<f32>(_e4.g0_.x, _e7.g0_.y, _e10.g0_.x, _e13.g0_.x) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) - (vec4<f32>(_e23.g0_.x, _e26.g0_.x, _e29.g0_.x, _e32.g0_.y) * vec4<f32>(0.0, 0.0, 1.0, 1.0))));
}

fn rotor_ideal_point_geometric_product(self_664: Rotor, other_550: IdealPoint) -> IdealPoint {
    var self_665: Rotor;
    var other_551: IdealPoint;

    self_665 = self_664;
    other_551 = other_550;
    let _e4: Rotor = self_665;
    let _e8: IdealPoint = other_551;
    let _e11: Rotor = self_665;
    let _e15: IdealPoint = other_551;
    return IdealPoint(((vec2<f32>(_e4.g0_.x) * _e8.g0_) + ((vec2<f32>(_e11.g0_.y) * _e15.g0_.yx) * vec2<f32>(-(1.0), 1.0))));
}

fn rotor_ideal_point_outer_product(self_666: Rotor, other_552: IdealPoint) -> IdealPoint {
    var self_667: Rotor;
    var other_553: IdealPoint;

    self_667 = self_666;
    other_553 = other_552;
    let _e4: Rotor = self_667;
    let _e8: IdealPoint = other_553;
    return IdealPoint((vec2<f32>(_e4.g0_.x) * _e8.g0_));
}

fn rotor_ideal_point_inner_product(self_668: Rotor, other_554: IdealPoint) -> IdealPoint {
    var self_669: Rotor;
    var other_555: IdealPoint;

    self_669 = self_668;
    other_555 = other_554;
    let _e4: Rotor = self_669;
    let _e8: IdealPoint = other_555;
    return IdealPoint((vec2<f32>(_e4.g0_.x) * _e8.g0_));
}

fn rotor_ideal_point_left_contraction(self_670: Rotor, other_556: IdealPoint) -> IdealPoint {
    var self_671: Rotor;
    var other_557: IdealPoint;

    self_671 = self_670;
    other_557 = other_556;
    let _e4: Rotor = self_671;
    let _e8: IdealPoint = other_557;
    return IdealPoint((vec2<f32>(_e4.g0_.x) * _e8.g0_));
}

fn rotor_plane_geometric_product(self_672: Rotor, other_558: Plane) -> MotorDual {
    var self_673: Rotor;
    var other_559: Plane;

    self_673 = self_672;
    other_559 = other_558;
    let _e4: Rotor = self_673;
    let _e8: Plane = other_559;
    let _e11: Plane = other_559;
    let _e14: Plane = other_559;
    let _e17: Plane = other_559;
    let _e29: Rotor = self_673;
    let _e33: Plane = other_559;
    let _e36: Plane = other_559;
    let _e39: Plane = other_559;
    let _e42: Plane = other_559;
    return MotorDual((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.x, _e11.g0_.x, _e14.g0_.z, _e17.g0_.y)) * vec4<f32>(1.0, 0.0, -(1.0), 1.0)) + ((vec4<f32>(_e29.g0_.x) * vec4<f32>(_e33.g0_.x, _e36.g0_.x, _e39.g0_.y, _e42.g0_.z)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn rotor_plane_regressive_product(self_674: Rotor, other_560: Plane) -> Scalar {
    var self_675: Rotor;
    var other_561: Plane;

    self_675 = self_674;
    other_561 = other_560;
    let _e4: Rotor = self_675;
    let _e7: Plane = other_561;
    return Scalar((_e4.g0_.y * _e7.g0_.x));
}

fn rotor_plane_outer_product(self_676: Rotor, other_562: Plane) -> MotorDual {
    var self_677: Rotor;
    var other_563: Plane;

    self_677 = self_676;
    other_563 = other_562;
    let _e4: Rotor = self_677;
    let _e7: Rotor = self_677;
    let _e10: Rotor = self_677;
    let _e13: Rotor = self_677;
    let _e17: Plane = other_563;
    let _e20: Plane = other_563;
    let _e23: Plane = other_563;
    let _e26: Plane = other_563;
    return MotorDual((vec4<f32>(_e4.g0_.y, _e7.g0_.x, _e10.g0_.x, _e13.g0_.x) * vec4<f32>(_e17.g0_.x, _e20.g0_.x, _e23.g0_.y, _e26.g0_.z)));
}

fn rotor_plane_inner_product(self_678: Rotor, other_564: Plane) -> Plane {
    var self_679: Rotor;
    var other_565: Plane;

    self_679 = self_678;
    other_565 = other_564;
    let _e4: Rotor = self_679;
    let _e8: Plane = other_565;
    let _e11: Rotor = self_679;
    let _e14: Rotor = self_679;
    let _e17: Rotor = self_679;
    let _e21: Plane = other_565;
    return Plane(((vec3<f32>(_e4.g0_.x) * _e8.g0_) + ((vec3<f32>(_e11.g0_.x, _e14.g0_.y, _e17.g0_.y) * _e21.g0_.xzy) * vec3<f32>(0.0, -(1.0), 1.0))));
}

fn rotor_plane_geometric_anti_product(self_680: Rotor, other_566: Plane) -> Rotor {
    var self_681: Rotor;
    var other_567: Plane;

    self_681 = self_680;
    other_567 = other_566;
    let _e4: Rotor = self_681;
    let _e7: Plane = other_567;
    return Rotor(((_e4.g0_.yx * vec2<f32>(_e7.g0_.x)) * vec2<f32>(1.0, -(1.0))));
}

fn rotor_plane_left_contraction(self_682: Rotor, other_568: Plane) -> Plane {
    var self_683: Rotor;
    var other_569: Plane;

    self_683 = self_682;
    other_569 = other_568;
    let _e4: Rotor = self_683;
    let _e8: Plane = other_569;
    return Plane((vec3<f32>(_e4.g0_.x) * _e8.g0_));
}

fn rotor_translator_add(self_684: Rotor, other_570: Translator) -> Motor {
    var self_685: Rotor;
    var other_571: Translator;

    self_685 = self_684;
    other_571 = other_570;
    let _e4: Rotor = self_685;
    let _e7: Rotor = self_685;
    let _e10: Rotor = self_685;
    let _e13: Rotor = self_685;
    let _e23: Translator = other_571;
    let _e26: Translator = other_571;
    let _e29: Translator = other_571;
    let _e32: Translator = other_571;
    return Motor(((vec4<f32>(_e4.g0_.x, _e7.g0_.y, _e10.g0_.x, _e13.g0_.x) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) + (vec4<f32>(_e23.g0_.x, _e26.g0_.x, _e29.g0_.y, _e32.g0_.z) * vec4<f32>(1.0, 0.0, 1.0, 1.0))));
}

fn rotor_translator_sub(self_686: Rotor, other_572: Translator) -> Motor {
    var self_687: Rotor;
    var other_573: Translator;

    self_687 = self_686;
    other_573 = other_572;
    let _e4: Rotor = self_687;
    let _e7: Rotor = self_687;
    let _e10: Rotor = self_687;
    let _e13: Rotor = self_687;
    let _e23: Translator = other_573;
    let _e26: Translator = other_573;
    let _e29: Translator = other_573;
    let _e32: Translator = other_573;
    return Motor(((vec4<f32>(_e4.g0_.x, _e7.g0_.y, _e10.g0_.x, _e13.g0_.x) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) - (vec4<f32>(_e23.g0_.x, _e26.g0_.x, _e29.g0_.y, _e32.g0_.z) * vec4<f32>(1.0, 0.0, 1.0, 1.0))));
}

fn rotor_translator_geometric_product(self_688: Rotor, other_574: Translator) -> Motor {
    var self_689: Rotor;
    var other_575: Translator;

    self_689 = self_688;
    other_575 = other_574;
    let _e4: Rotor = self_689;
    let _e8: Translator = other_575;
    let _e11: Translator = other_575;
    let _e14: Translator = other_575;
    let _e17: Translator = other_575;
    let _e29: Rotor = self_689;
    let _e33: Translator = other_575;
    let _e36: Translator = other_575;
    let _e39: Translator = other_575;
    let _e42: Translator = other_575;
    return Motor((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.x, _e11.g0_.x, _e14.g0_.z, _e17.g0_.y)) * vec4<f32>(0.0, 1.0, -(1.0), 1.0)) + ((vec4<f32>(_e29.g0_.x) * vec4<f32>(_e33.g0_.x, _e36.g0_.x, _e39.g0_.y, _e42.g0_.z)) * vec4<f32>(1.0, 0.0, 1.0, 1.0))));
}

fn rotor_translator_outer_product(self_690: Rotor, other_576: Translator) -> Motor {
    var self_691: Rotor;
    var other_577: Translator;

    self_691 = self_690;
    other_577 = other_576;
    let _e4: Rotor = self_691;
    let _e7: Rotor = self_691;
    let _e10: Rotor = self_691;
    let _e13: Rotor = self_691;
    let _e17: Translator = other_577;
    let _e20: Translator = other_577;
    let _e23: Translator = other_577;
    let _e26: Translator = other_577;
    return Motor((vec4<f32>(_e4.g0_.x, _e7.g0_.y, _e10.g0_.x, _e13.g0_.x) * vec4<f32>(_e17.g0_.x, _e20.g0_.x, _e23.g0_.y, _e26.g0_.z)));
}

fn rotor_translator_inner_product(self_692: Rotor, other_578: Translator) -> Motor {
    var self_693: Rotor;
    var other_579: Translator;

    self_693 = self_692;
    other_579 = other_578;
    let _e4: Rotor = self_693;
    let _e7: Rotor = self_693;
    let _e10: Rotor = self_693;
    let _e13: Rotor = self_693;
    let _e17: Translator = other_579;
    let _e20: Translator = other_579;
    let _e23: Translator = other_579;
    let _e26: Translator = other_579;
    return Motor((vec4<f32>(_e4.g0_.x, _e7.g0_.y, _e10.g0_.x, _e13.g0_.x) * vec4<f32>(_e17.g0_.x, _e20.g0_.x, _e23.g0_.y, _e26.g0_.z)));
}

fn rotor_translator_left_contraction(self_694: Rotor, other_580: Translator) -> Translator {
    var self_695: Rotor;
    var other_581: Translator;

    self_695 = self_694;
    other_581 = other_580;
    let _e4: Rotor = self_695;
    let _e8: Translator = other_581;
    return Translator((vec3<f32>(_e4.g0_.x) * _e8.g0_));
}

fn rotor_translator_right_contraction(self_696: Rotor, other_582: Translator) -> Rotor {
    var self_697: Rotor;
    var other_583: Translator;

    self_697 = self_696;
    other_583 = other_582;
    let _e4: Rotor = self_697;
    let _e6: Translator = other_583;
    return Rotor((_e4.g0_ * vec2<f32>(_e6.g0_.x)));
}

fn rotor_translator_scalar_product(self_698: Rotor, other_584: Translator) -> Scalar {
    var self_699: Rotor;
    var other_585: Translator;

    self_699 = self_698;
    other_585 = other_584;
    let _e4: Rotor = self_699;
    let _e7: Translator = other_585;
    return Scalar((_e4.g0_.x * _e7.g0_.x));
}

fn rotor_motor_add(self_700: Rotor, other_586: Motor) -> Motor {
    var self_701: Rotor;
    var other_587: Motor;

    self_701 = self_700;
    other_587 = other_586;
    let _e4: Rotor = self_701;
    let _e7: Rotor = self_701;
    let _e10: Rotor = self_701;
    let _e13: Rotor = self_701;
    let _e23: Motor = other_587;
    return Motor(((vec4<f32>(_e4.g0_.x, _e7.g0_.y, _e10.g0_.x, _e13.g0_.x) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) + _e23.g0_));
}

fn rotor_motor_sub(self_702: Rotor, other_588: Motor) -> Motor {
    var self_703: Rotor;
    var other_589: Motor;

    self_703 = self_702;
    other_589 = other_588;
    let _e4: Rotor = self_703;
    let _e7: Rotor = self_703;
    let _e10: Rotor = self_703;
    let _e13: Rotor = self_703;
    let _e23: Motor = other_589;
    return Motor(((vec4<f32>(_e4.g0_.x, _e7.g0_.y, _e10.g0_.x, _e13.g0_.x) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) - _e23.g0_));
}

fn rotor_motor_geometric_product(self_704: Rotor, other_590: Motor) -> Motor {
    var self_705: Rotor;
    var other_591: Motor;

    self_705 = self_704;
    other_591 = other_590;
    let _e4: Rotor = self_705;
    let _e8: Motor = other_591;
    let _e11: Rotor = self_705;
    let _e15: Motor = other_591;
    return Motor(((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.y) * _e15.g0_.yxwz) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))));
}

fn rotor_motor_outer_product(self_706: Rotor, other_592: Motor) -> Motor {
    var self_707: Rotor;
    var other_593: Motor;

    self_707 = self_706;
    other_593 = other_592;
    let _e4: Rotor = self_707;
    let _e8: Motor = other_593;
    let _e11: Rotor = self_707;
    let _e14: Rotor = self_707;
    let _e17: Rotor = self_707;
    let _e20: Rotor = self_707;
    let _e24: Motor = other_593;
    return Motor(((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.x, _e14.g0_.y, _e17.g0_.x, _e20.g0_.x) * vec4<f32>(_e24.g0_.x)) * vec4<f32>(0.0, 1.0, 0.0, 0.0))));
}

fn rotor_motor_inner_product(self_708: Rotor, other_594: Motor) -> Motor {
    var self_709: Rotor;
    var other_595: Motor;

    self_709 = self_708;
    other_595 = other_594;
    let _e4: Rotor = self_709;
    let _e8: Motor = other_595;
    let _e11: Rotor = self_709;
    let _e14: Rotor = self_709;
    let _e17: Rotor = self_709;
    let _e20: Rotor = self_709;
    let _e24: Motor = other_595;
    return Motor(((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.y, _e14.g0_.y, _e17.g0_.x, _e20.g0_.x) * _e24.g0_.yxxx) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))));
}

fn rotor_motor_left_contraction(self_710: Rotor, other_596: Motor) -> Motor {
    var self_711: Rotor;
    var other_597: Motor;

    self_711 = self_710;
    other_597 = other_596;
    let _e4: Rotor = self_711;
    let _e8: Motor = other_597;
    let _e11: Rotor = self_711;
    let _e14: Rotor = self_711;
    let _e17: Rotor = self_711;
    let _e20: Rotor = self_711;
    let _e24: Motor = other_597;
    return Motor(((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.y, _e14.g0_.x, _e17.g0_.x, _e20.g0_.x) * _e24.g0_.yxxx) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))));
}

fn rotor_motor_right_contraction(self_712: Rotor, other_598: Motor) -> Rotor {
    var self_713: Rotor;
    var other_599: Motor;

    self_713 = self_712;
    other_599 = other_598;
    let _e4: Rotor = self_713;
    let _e8: Motor = other_599;
    let _e11: Motor = other_599;
    let _e21: Rotor = self_713;
    let _e25: Motor = other_599;
    return Rotor((((vec2<f32>(_e4.g0_.y) * vec2<f32>(_e8.g0_.y, _e11.g0_.x)) * vec2<f32>(-(1.0), 1.0)) + ((vec2<f32>(_e21.g0_.x) * vec2<f32>(_e25.g0_.x)) * vec2<f32>(1.0, 0.0))));
}

fn rotor_motor_scalar_product(self_714: Rotor, other_600: Motor) -> Scalar {
    var self_715: Rotor;
    var other_601: Motor;

    self_715 = self_714;
    other_601 = other_600;
    let _e4: Rotor = self_715;
    let _e7: Motor = other_601;
    let _e11: Rotor = self_715;
    let _e14: Motor = other_601;
    return Scalar(((_e4.g0_.x * _e7.g0_.x) - (_e11.g0_.y * _e14.g0_.y)));
}

fn rotor_motor_dual_geometric_product(self_716: Rotor, other_602: MotorDual) -> MotorDual {
    var self_717: Rotor;
    var other_603: MotorDual;

    self_717 = self_716;
    other_603 = other_602;
    let _e4: Rotor = self_717;
    let _e8: MotorDual = other_603;
    let _e11: Rotor = self_717;
    let _e15: MotorDual = other_603;
    return MotorDual(((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.y) * _e15.g0_.yxwz) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0))));
}

fn rotor_motor_dual_regressive_product(self_718: Rotor, other_604: MotorDual) -> Rotor {
    var self_719: Rotor;
    var other_605: MotorDual;

    self_719 = self_718;
    other_605 = other_604;
    let _e4: Rotor = self_719;
    let _e8: MotorDual = other_605;
    let _e11: MotorDual = other_605;
    let _e16: Rotor = self_719;
    let _e20: MotorDual = other_605;
    return Rotor(((vec2<f32>(_e4.g0_.y) * vec2<f32>(_e8.g0_.y, _e11.g0_.x)) + ((vec2<f32>(_e16.g0_.x) * vec2<f32>(_e20.g0_.x)) * vec2<f32>(1.0, 0.0))));
}

fn rotor_motor_dual_outer_product(self_720: Rotor, other_606: MotorDual) -> MotorDual {
    var self_721: Rotor;
    var other_607: MotorDual;

    self_721 = self_720;
    other_607 = other_606;
    let _e4: Rotor = self_721;
    let _e8: MotorDual = other_607;
    let _e11: Rotor = self_721;
    let _e14: Rotor = self_721;
    let _e17: Rotor = self_721;
    let _e20: Rotor = self_721;
    let _e24: MotorDual = other_607;
    return MotorDual(((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.y, _e14.g0_.x, _e17.g0_.x, _e20.g0_.x) * _e24.g0_.yxxx) * vec4<f32>(1.0, 0.0, 0.0, 0.0))));
}

fn rotor_motor_dual_inner_product(self_722: Rotor, other_608: MotorDual) -> MotorDual {
    var self_723: Rotor;
    var other_609: MotorDual;

    self_723 = self_722;
    other_609 = other_608;
    let _e4: Rotor = self_723;
    let _e8: MotorDual = other_609;
    let _e11: Rotor = self_723;
    let _e14: Rotor = self_723;
    let _e17: Rotor = self_723;
    let _e20: Rotor = self_723;
    let _e24: MotorDual = other_609;
    return MotorDual(((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.x, _e14.g0_.y, _e17.g0_.y, _e20.g0_.y) * _e24.g0_.xxwz) * vec4<f32>(0.0, -(1.0), -(1.0), 1.0))));
}

fn rotor_motor_dual_geometric_anti_product(self_724: Rotor, other_610: MotorDual) -> Rotor {
    var self_725: Rotor;
    var other_611: MotorDual;

    self_725 = self_724;
    other_611 = other_610;
    let _e4: Rotor = self_725;
    let _e8: MotorDual = other_611;
    let _e11: MotorDual = other_611;
    let _e21: Rotor = self_725;
    let _e25: MotorDual = other_611;
    let _e28: MotorDual = other_611;
    return Rotor((((vec2<f32>(_e4.g0_.x) * vec2<f32>(_e8.g0_.x, _e11.g0_.y)) * vec2<f32>(1.0, -(1.0))) + (vec2<f32>(_e21.g0_.y) * vec2<f32>(_e25.g0_.y, _e28.g0_.x))));
}

fn rotor_motor_dual_inner_anti_product(self_726: Rotor, other_612: MotorDual) -> Rotor {
    var self_727: Rotor;
    var other_613: MotorDual;

    self_727 = self_726;
    other_613 = other_612;
    let _e4: Rotor = self_727;
    let _e8: MotorDual = other_613;
    let _e11: MotorDual = other_613;
    let _e21: Rotor = self_727;
    let _e23: MotorDual = other_613;
    return Rotor((((vec2<f32>(_e4.g0_.x) * vec2<f32>(_e8.g0_.x, _e11.g0_.y)) * vec2<f32>(1.0, -(1.0))) + ((_e21.g0_ * vec2<f32>(_e23.g0_.x)) * vec2<f32>(0.0, 1.0))));
}

fn rotor_motor_dual_left_contraction(self_728: Rotor, other_614: MotorDual) -> MotorDual {
    var self_729: Rotor;
    var other_615: MotorDual;

    self_729 = self_728;
    other_615 = other_614;
    let _e4: Rotor = self_729;
    let _e8: MotorDual = other_615;
    let _e11: Rotor = self_729;
    let _e14: Rotor = self_729;
    let _e17: Rotor = self_729;
    let _e20: Rotor = self_729;
    let _e24: MotorDual = other_615;
    return MotorDual(((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.x, _e14.g0_.y, _e17.g0_.x, _e20.g0_.x) * vec4<f32>(_e24.g0_.x)) * vec4<f32>(0.0, -(1.0), 0.0, 0.0))));
}

fn rotor_motor_dual_right_anti_contraction(self_730: Rotor, other_616: MotorDual) -> Rotor {
    var self_731: Rotor;
    var other_617: MotorDual;

    self_731 = self_730;
    other_617 = other_616;
    let _e4: Rotor = self_731;
    let _e8: MotorDual = other_617;
    let _e11: MotorDual = other_617;
    let _e21: Rotor = self_731;
    let _e23: MotorDual = other_617;
    return Rotor((((vec2<f32>(_e4.g0_.x) * vec2<f32>(_e8.g0_.x, _e11.g0_.y)) * vec2<f32>(1.0, -(1.0))) + ((_e21.g0_ * vec2<f32>(_e23.g0_.x)) * vec2<f32>(0.0, 1.0))));
}

fn rotor_squared_magnitude(self_732: Rotor) -> Scalar {
    var self_733: Rotor;

    self_733 = self_732;
    let _e2: Rotor = self_733;
    let _e3: Rotor = self_733;
    let _e4: Rotor = rotor_reversal(_e3);
    let _e5: Scalar = rotor_rotor_scalar_product(_e2, _e4);
    return _e5;
}

fn rotor_magnitude(self_734: Rotor) -> Scalar {
    var self_735: Rotor;

    self_735 = self_734;
    let _e2: Rotor = self_735;
    let _e3: Scalar = rotor_squared_magnitude(_e2);
    return Scalar(sqrt(_e3.g0_));
}

fn rotor_scale(self_736: Rotor, other_618: f32) -> Rotor {
    var self_737: Rotor;
    var other_619: f32;

    self_737 = self_736;
    other_619 = other_618;
    let _e4: Rotor = self_737;
    let _e5: f32 = other_619;
    let _e7: Rotor = rotor_scalar_geometric_product(_e4, Scalar(_e5));
    return _e7;
}

fn rotor_signum(self_738: Rotor) -> Rotor {
    var self_739: Rotor;

    self_739 = self_738;
    let _e2: Rotor = self_739;
    let _e3: Rotor = self_739;
    let _e4: Scalar = rotor_magnitude(_e3);
    let _e9: Rotor = rotor_scalar_geometric_product(_e2, Scalar((1.0 / _e4.g0_)));
    return _e9;
}

fn rotor_inverse(self_740: Rotor) -> Rotor {
    var self_741: Rotor;

    self_741 = self_740;
    let _e2: Rotor = self_741;
    let _e3: Rotor = rotor_reversal(_e2);
    let _e4: Rotor = self_741;
    let _e5: Scalar = rotor_squared_magnitude(_e4);
    let _e10: Rotor = rotor_scalar_geometric_product(_e3, Scalar((1.0 / _e5.g0_)));
    return _e10;
}

fn point_zero() -> Point {
    return Point(vec3<f32>(0.0));
}

fn point_one() -> Point {
    return Point(vec3<f32>(0.0));
}

fn point_grade(self_742: Point) -> i32 {
    return 2;
}

fn point_anti_grade(self_743: Point) -> i32 {
    return 1;
}

fn point_neg(self_744: Point) -> Point {
    var self_745: Point;

    self_745 = self_744;
    let _e2: Point = self_745;
    return Point((_e2.g0_ * vec3<f32>(-(1.0))));
}

fn point_automorphism(self_746: Point) -> Point {
    var self_747: Point;

    self_747 = self_746;
    let _e2: Point = self_747;
    return Point(_e2.g0_);
}

fn point_reversal(self_748: Point) -> Point {
    var self_749: Point;

    self_749 = self_748;
    let _e2: Point = self_749;
    return Point((_e2.g0_ * vec3<f32>(-(1.0))));
}

fn point_conjugation(self_750: Point) -> Point {
    var self_751: Point;

    self_751 = self_750;
    let _e2: Point = self_751;
    return Point((_e2.g0_ * vec3<f32>(-(1.0))));
}

fn point_dual(self_752: Point) -> Plane {
    var self_753: Point;

    self_753 = self_752;
    let _e2: Point = self_753;
    return Plane(_e2.g0_);
}

fn point_anti_reversal(self_754: Point) -> Point {
    var self_755: Point;

    self_755 = self_754;
    let _e2: Point = self_755;
    return Point((_e2.g0_ * vec3<f32>(-(1.0))));
}

fn point_right_complement(self_756: Point) -> Plane {
    var self_757: Point;

    self_757 = self_756;
    let _e2: Point = self_757;
    return Plane(_e2.g0_);
}

fn point_left_complement(self_758: Point) -> Plane {
    var self_759: Point;

    self_759 = self_758;
    let _e2: Point = self_759;
    return Plane(_e2.g0_);
}

fn point_double_complement(self_760: Point) -> Point {
    var self_761: Point;

    self_761 = self_760;
    let _e2: Point = self_761;
    return Point(_e2.g0_);
}

fn point_scalar_add(self_762: Point, other_620: Scalar) -> Motor {
    var self_763: Point;
    var other_621: Scalar;

    self_763 = self_762;
    other_621 = other_620;
    let _e4: Point = self_763;
    let _e7: Point = self_763;
    let _e10: Point = self_763;
    let _e13: Point = self_763;
    let _e23: Scalar = other_621;
    return Motor(((vec4<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.y, _e13.g0_.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0)) + (vec4<f32>(_e23.g0_) * vec4<f32>(1.0, 0.0, 0.0, 0.0))));
}

fn point_scalar_sub(self_764: Point, other_622: Scalar) -> Motor {
    var self_765: Point;
    var other_623: Scalar;

    self_765 = self_764;
    other_623 = other_622;
    let _e4: Point = self_765;
    let _e7: Point = self_765;
    let _e10: Point = self_765;
    let _e13: Point = self_765;
    let _e23: Scalar = other_623;
    return Motor(((vec4<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.y, _e13.g0_.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0)) - (vec4<f32>(_e23.g0_) * vec4<f32>(1.0, 0.0, 0.0, 0.0))));
}

fn point_scalar_geometric_product(self_766: Point, other_624: Scalar) -> Point {
    var self_767: Point;
    var other_625: Scalar;

    self_767 = self_766;
    other_625 = other_624;
    let _e4: Point = self_767;
    let _e6: Scalar = other_625;
    return Point((_e4.g0_ * vec3<f32>(_e6.g0_)));
}

fn point_scalar_outer_product(self_768: Point, other_626: Scalar) -> Point {
    var self_769: Point;
    var other_627: Scalar;

    self_769 = self_768;
    other_627 = other_626;
    let _e4: Point = self_769;
    let _e6: Scalar = other_627;
    return Point((_e4.g0_ * vec3<f32>(_e6.g0_)));
}

fn point_scalar_inner_product(self_770: Point, other_628: Scalar) -> Point {
    var self_771: Point;
    var other_629: Scalar;

    self_771 = self_770;
    other_629 = other_628;
    let _e4: Point = self_771;
    let _e6: Scalar = other_629;
    return Point((_e4.g0_ * vec3<f32>(_e6.g0_)));
}

fn point_scalar_right_contraction(self_772: Point, other_630: Scalar) -> Point {
    var self_773: Point;
    var other_631: Scalar;

    self_773 = self_772;
    other_631 = other_630;
    let _e4: Point = self_773;
    let _e6: Scalar = other_631;
    return Point((_e4.g0_ * vec3<f32>(_e6.g0_)));
}

fn point_anti_scalar_regressive_product(self_774: Point, other_632: AntiScalar) -> Point {
    var self_775: Point;
    var other_633: AntiScalar;

    self_775 = self_774;
    other_633 = other_632;
    let _e4: Point = self_775;
    let _e6: AntiScalar = other_633;
    return Point((_e4.g0_ * vec3<f32>(_e6.g0_)));
}

fn point_anti_scalar_geometric_anti_product(self_776: Point, other_634: AntiScalar) -> Point {
    var self_777: Point;
    var other_635: AntiScalar;

    self_777 = self_776;
    other_635 = other_634;
    let _e4: Point = self_777;
    let _e6: AntiScalar = other_635;
    return Point((_e4.g0_ * vec3<f32>(_e6.g0_)));
}

fn point_anti_scalar_inner_anti_product(self_778: Point, other_636: AntiScalar) -> Point {
    var self_779: Point;
    var other_637: AntiScalar;

    self_779 = self_778;
    other_637 = other_636;
    let _e4: Point = self_779;
    let _e6: AntiScalar = other_637;
    return Point((_e4.g0_ * vec3<f32>(_e6.g0_)));
}

fn point_anti_scalar_right_anti_contraction(self_780: Point, other_638: AntiScalar) -> Point {
    var self_781: Point;
    var other_639: AntiScalar;

    self_781 = self_780;
    other_639 = other_638;
    let _e4: Point = self_781;
    let _e6: AntiScalar = other_639;
    return Point((_e4.g0_ * vec3<f32>(_e6.g0_)));
}

fn point_multi_vector_add(self_782: Point, other_640: MultiVector) -> MultiVector {
    var self_783: Point;
    var other_641: MultiVector;

    self_783 = self_782;
    other_641 = other_640;
    let _e4: Point = self_783;
    let _e14: MultiVector = other_641;
    let _e17: Point = self_783;
    let _e20: Point = self_783;
    let _e23: Point = self_783;
    let _e26: Point = self_783;
    let _e36: MultiVector = other_641;
    return MultiVector(((vec4<f32>(_e4.g0_.x) * vec4<f32>(0.0, 1.0, 0.0, 0.0)) + _e14.g0_), ((vec4<f32>(_e17.g0_.x, _e20.g0_.x, _e23.g0_.y, _e26.g0_.z) * vec4<f32>(0.0, 0.0, 1.0, 1.0)) + _e36.g1_));
}

fn point_multi_vector_sub(self_784: Point, other_642: MultiVector) -> MultiVector {
    var self_785: Point;
    var other_643: MultiVector;

    self_785 = self_784;
    other_643 = other_642;
    let _e4: Point = self_785;
    let _e14: MultiVector = other_643;
    let _e17: Point = self_785;
    let _e20: Point = self_785;
    let _e23: Point = self_785;
    let _e26: Point = self_785;
    let _e36: MultiVector = other_643;
    return MultiVector(((vec4<f32>(_e4.g0_.x) * vec4<f32>(0.0, 1.0, 0.0, 0.0)) - _e14.g0_), ((vec4<f32>(_e17.g0_.x, _e20.g0_.x, _e23.g0_.y, _e26.g0_.z) * vec4<f32>(0.0, 0.0, 1.0, 1.0)) - _e36.g1_));
}

fn point_multi_vector_geometric_product(self_786: Point, other_644: MultiVector) -> MultiVector {
    var self_787: Point;
    var other_645: MultiVector;

    self_787 = self_786;
    other_645 = other_644;
    let _e4: Point = self_787;
    let _e8: MultiVector = other_645;
    let _e20: Point = self_787;
    let _e24: MultiVector = other_645;
    let _e36: Point = self_787;
    let _e40: MultiVector = other_645;
    let _e52: Point = self_787;
    let _e56: MultiVector = other_645;
    return MultiVector(((vec4<f32>(_e4.g0_.x) * _e8.g0_.yxwz) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0))), ((((vec4<f32>(_e20.g0_.x) * _e24.g1_.yxwz) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0)) + ((vec4<f32>(_e36.g0_.y) * _e40.g0_.zwxy) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e52.g0_.z) * _e56.g0_.wzyx) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0))));
}

fn point_multi_vector_geometric_anti_product(self_788: Point, other_646: MultiVector) -> MultiVector {
    var self_789: Point;
    var other_647: MultiVector;

    self_789 = self_788;
    other_647 = other_646;
    let _e4: Point = self_789;
    let _e8: MultiVector = other_647;
    let _e18: Point = self_789;
    let _e22: MultiVector = other_647;
    let _e27: Point = self_789;
    let _e31: MultiVector = other_647;
    let _e44: Point = self_789;
    let _e48: MultiVector = other_647;
    let _e60: Point = self_789;
    let _e64: MultiVector = other_647;
    return MultiVector(((((vec4<f32>(_e4.g0_.x) * _e8.g1_) * vec4<f32>(1.0, 1.0, -(1.0), 1.0)) + (vec4<f32>(_e18.g0_.y) * _e22.g0_.wzyx)) + ((vec4<f32>(_e27.g0_.z) * _e31.g0_.zwxy) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))), (((vec4<f32>(_e44.g0_.y) * _e48.g1_.wzyx) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0))) + (vec4<f32>(_e60.g0_.z) * _e64.g1_.zwxy)));
}

fn point_multi_vector_scalar_product(self_790: Point, other_648: MultiVector) -> Scalar {
    var self_791: Point;
    var other_649: MultiVector;

    self_791 = self_790;
    other_649 = other_648;
    let _e5: Point = self_791;
    let _e8: MultiVector = other_649;
    return Scalar((0.0 - (_e5.g0_.x * _e8.g0_.y)));
}

fn point_multi_vector_anti_scalar_product(self_792: Point, other_650: MultiVector) -> AntiScalar {
    var self_793: Point;
    var other_651: MultiVector;

    self_793 = self_792;
    other_651 = other_650;
    let _e4: Point = self_793;
    let _e7: MultiVector = other_651;
    let _e11: Point = self_793;
    let _e14: MultiVector = other_651;
    return AntiScalar(((_e4.g0_.y * _e7.g1_.z) + (_e11.g0_.z * _e14.g1_.w)));
}

fn point_rotor_add(self_794: Point, other_652: Rotor) -> Motor {
    var self_795: Point;
    var other_653: Rotor;

    self_795 = self_794;
    other_653 = other_652;
    let _e4: Point = self_795;
    let _e7: Point = self_795;
    let _e10: Point = self_795;
    let _e13: Point = self_795;
    let _e23: Rotor = other_653;
    let _e26: Rotor = other_653;
    let _e29: Rotor = other_653;
    let _e32: Rotor = other_653;
    return Motor(((vec4<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.y, _e13.g0_.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0)) + (vec4<f32>(_e23.g0_.x, _e26.g0_.y, _e29.g0_.x, _e32.g0_.x) * vec4<f32>(1.0, 1.0, 0.0, 0.0))));
}

fn point_rotor_sub(self_796: Point, other_654: Rotor) -> Motor {
    var self_797: Point;
    var other_655: Rotor;

    self_797 = self_796;
    other_655 = other_654;
    let _e4: Point = self_797;
    let _e7: Point = self_797;
    let _e10: Point = self_797;
    let _e13: Point = self_797;
    let _e23: Rotor = other_655;
    let _e26: Rotor = other_655;
    let _e29: Rotor = other_655;
    let _e32: Rotor = other_655;
    return Motor(((vec4<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.y, _e13.g0_.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0)) - (vec4<f32>(_e23.g0_.x, _e26.g0_.y, _e29.g0_.x, _e32.g0_.x) * vec4<f32>(1.0, 1.0, 0.0, 0.0))));
}

fn point_rotor_geometric_product(self_798: Point, other_656: Rotor) -> Motor {
    var self_799: Point;
    var other_657: Rotor;

    self_799 = self_798;
    other_657 = other_656;
    let _e4: Point = self_799;
    let _e8: Rotor = other_657;
    let _e11: Rotor = other_657;
    let _e14: Rotor = other_657;
    let _e17: Rotor = other_657;
    let _e28: Point = self_799;
    let _e31: Point = self_799;
    let _e34: Point = self_799;
    let _e37: Point = self_799;
    let _e41: Rotor = other_657;
    let _e44: Rotor = other_657;
    let _e47: Rotor = other_657;
    let _e50: Rotor = other_657;
    return Motor((((vec4<f32>(_e4.g0_.z) * vec4<f32>(_e8.g0_.y, _e11.g0_.y, _e14.g0_.y, _e17.g0_.x)) * vec4<f32>(0.0, 0.0, 1.0, 1.0)) + ((vec4<f32>(_e28.g0_.x, _e31.g0_.x, _e34.g0_.y, _e37.g0_.y) * vec4<f32>(_e41.g0_.y, _e44.g0_.x, _e47.g0_.x, _e50.g0_.y)) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))));
}

fn point_rotor_outer_product(self_800: Point, other_658: Rotor) -> Point {
    var self_801: Point;
    var other_659: Rotor;

    self_801 = self_800;
    other_659 = other_658;
    let _e4: Point = self_801;
    let _e6: Rotor = other_659;
    return Point((_e4.g0_ * vec3<f32>(_e6.g0_.x)));
}

fn point_rotor_inner_product(self_802: Point, other_660: Rotor) -> Motor {
    var self_803: Point;
    var other_661: Rotor;

    self_803 = self_802;
    other_661 = other_660;
    let _e4: Point = self_803;
    let _e7: Point = self_803;
    let _e10: Point = self_803;
    let _e13: Point = self_803;
    let _e17: Rotor = other_661;
    let _e20: Rotor = other_661;
    let _e23: Rotor = other_661;
    let _e26: Rotor = other_661;
    return Motor(((vec4<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.y, _e13.g0_.z) * vec4<f32>(_e17.g0_.y, _e20.g0_.x, _e23.g0_.x, _e26.g0_.x)) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0)));
}

fn point_rotor_left_contraction(self_804: Point, other_662: Rotor) -> Scalar {
    var self_805: Point;
    var other_663: Rotor;

    self_805 = self_804;
    other_663 = other_662;
    let _e5: Point = self_805;
    let _e8: Rotor = other_663;
    return Scalar((0.0 - (_e5.g0_.x * _e8.g0_.y)));
}

fn point_rotor_right_contraction(self_806: Point, other_664: Rotor) -> Motor {
    var self_807: Point;
    var other_665: Rotor;

    self_807 = self_806;
    other_665 = other_664;
    let _e4: Point = self_807;
    let _e7: Point = self_807;
    let _e10: Point = self_807;
    let _e13: Point = self_807;
    let _e17: Rotor = other_665;
    let _e20: Rotor = other_665;
    let _e23: Rotor = other_665;
    let _e26: Rotor = other_665;
    return Motor(((vec4<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.y, _e13.g0_.z) * vec4<f32>(_e17.g0_.y, _e20.g0_.x, _e23.g0_.x, _e26.g0_.x)) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0)));
}

fn point_rotor_scalar_product(self_808: Point, other_666: Rotor) -> Scalar {
    var self_809: Point;
    var other_667: Rotor;

    self_809 = self_808;
    other_667 = other_666;
    let _e5: Point = self_809;
    let _e8: Rotor = other_667;
    return Scalar((0.0 - (_e5.g0_.x * _e8.g0_.y)));
}

fn point_point_add(self_810: Point, other_668: Point) -> Point {
    var self_811: Point;
    var other_669: Point;

    self_811 = self_810;
    other_669 = other_668;
    let _e4: Point = self_811;
    let _e6: Point = other_669;
    return Point((_e4.g0_ + _e6.g0_));
}

fn point_point_sub(self_812: Point, other_670: Point) -> Point {
    var self_813: Point;
    var other_671: Point;

    self_813 = self_812;
    other_671 = other_670;
    let _e4: Point = self_813;
    let _e6: Point = other_671;
    return Point((_e4.g0_ - _e6.g0_));
}

fn point_point_mul(self_814: Point, other_672: Point) -> Point {
    var self_815: Point;
    var other_673: Point;

    self_815 = self_814;
    other_673 = other_672;
    let _e4: Point = self_815;
    let _e6: Point = other_673;
    return Point((_e4.g0_ * _e6.g0_));
}

fn point_point_div(self_816: Point, other_674: Point) -> Point {
    var self_817: Point;
    var other_675: Point;

    self_817 = self_816;
    other_675 = other_674;
    let _e4: Point = self_817;
    let _e7: Point = self_817;
    let _e10: Point = self_817;
    let _e19: Point = other_675;
    let _e22: Point = other_675;
    let _e25: Point = other_675;
    return Point((((vec3<f32>(_e4.g0_.x, _e7.g0_.y, _e10.g0_.z) * vec3<f32>(1.0, 1.0, 1.0)) / vec3<f32>(_e19.g0_.x, _e22.g0_.y, _e25.g0_.z)) * vec3<f32>(1.0, 1.0, 1.0)));
}

fn point_point_geometric_product(self_818: Point, other_676: Point) -> Translator {
    var self_819: Point;
    var other_677: Point;

    self_819 = self_818;
    other_677 = other_676;
    let _e4: Point = self_819;
    let _e8: Point = other_677;
    let _e19: Point = self_819;
    let _e22: Point = other_677;
    return Translator((((vec3<f32>(_e4.g0_.x) * _e8.g0_.xzy) * vec3<f32>(-(1.0), -(1.0), 1.0)) + ((_e19.g0_.xzy * vec3<f32>(_e22.g0_.x)) * vec3<f32>(0.0, 1.0, -(1.0)))));
}

fn point_point_regressive_product(self_820: Point, other_678: Point) -> Plane {
    var self_821: Point;
    var other_679: Point;

    self_821 = self_820;
    other_679 = other_678;
    let _e4: Point = self_821;
    let _e8: Point = other_679;
    let _e18: Point = self_821;
    let _e22: Point = other_679;
    let _e33: Point = self_821;
    let _e37: Point = other_679;
    return Plane(((((vec3<f32>(_e4.g0_.y) * _e8.g0_.zzx) * vec3<f32>(-(1.0), 0.0, 1.0)) + ((vec3<f32>(_e18.g0_.z) * _e22.g0_.yxy) * vec3<f32>(1.0, -(1.0), 0.0))) + ((vec3<f32>(_e33.g0_.x) * _e37.g0_.xzy) * vec3<f32>(0.0, 1.0, -(1.0)))));
}

fn point_point_inner_product(self_822: Point, other_680: Point) -> Scalar {
    var self_823: Point;
    var other_681: Point;

    self_823 = self_822;
    other_681 = other_680;
    let _e5: Point = self_823;
    let _e8: Point = other_681;
    return Scalar((0.0 - (_e5.g0_.x * _e8.g0_.x)));
}

fn point_point_geometric_anti_product(self_824: Point, other_682: Point) -> MotorDual {
    var self_825: Point;
    var other_683: Point;

    self_825 = self_824;
    other_683 = other_682;
    let _e4: Point = self_825;
    let _e8: Point = other_683;
    let _e11: Point = other_683;
    let _e14: Point = other_683;
    let _e17: Point = other_683;
    let _e29: Point = self_825;
    let _e33: Point = other_683;
    let _e36: Point = other_683;
    let _e39: Point = other_683;
    let _e42: Point = other_683;
    let _e55: Point = self_825;
    let _e59: Point = other_683;
    let _e62: Point = other_683;
    let _e65: Point = other_683;
    let _e68: Point = other_683;
    return MotorDual(((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y, _e11.g0_.z, _e14.g0_.y, _e17.g0_.x)) * vec4<f32>(1.0, -(1.0), 0.0, 1.0)) + ((vec4<f32>(_e29.g0_.z) * vec4<f32>(_e33.g0_.z, _e36.g0_.y, _e39.g0_.x, _e42.g0_.z)) * vec4<f32>(1.0, 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e55.g0_.x) * vec4<f32>(_e59.g0_.x, _e62.g0_.x, _e65.g0_.z, _e68.g0_.y)) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))));
}

fn point_point_inner_anti_product(self_826: Point, other_684: Point) -> AntiScalar {
    var self_827: Point;
    var other_685: Point;

    self_827 = self_826;
    other_685 = other_684;
    let _e4: Point = self_827;
    let _e7: Point = other_685;
    let _e11: Point = self_827;
    let _e14: Point = other_685;
    return AntiScalar(((_e4.g0_.y * _e7.g0_.y) + (_e11.g0_.z * _e14.g0_.z)));
}

fn point_point_left_contraction(self_828: Point, other_686: Point) -> Scalar {
    var self_829: Point;
    var other_687: Point;

    self_829 = self_828;
    other_687 = other_686;
    let _e5: Point = self_829;
    let _e8: Point = other_687;
    return Scalar((0.0 - (_e5.g0_.x * _e8.g0_.x)));
}

fn point_point_right_contraction(self_830: Point, other_688: Point) -> Scalar {
    var self_831: Point;
    var other_689: Point;

    self_831 = self_830;
    other_689 = other_688;
    let _e5: Point = self_831;
    let _e8: Point = other_689;
    return Scalar((0.0 - (_e5.g0_.x * _e8.g0_.x)));
}

fn point_point_left_anti_contraction(self_832: Point, other_690: Point) -> AntiScalar {
    var self_833: Point;
    var other_691: Point;

    self_833 = self_832;
    other_691 = other_690;
    let _e4: Point = self_833;
    let _e7: Point = other_691;
    let _e11: Point = self_833;
    let _e14: Point = other_691;
    return AntiScalar(((_e4.g0_.y * _e7.g0_.y) + (_e11.g0_.z * _e14.g0_.z)));
}

fn point_point_right_anti_contraction(self_834: Point, other_692: Point) -> AntiScalar {
    var self_835: Point;
    var other_693: Point;

    self_835 = self_834;
    other_693 = other_692;
    let _e4: Point = self_835;
    let _e7: Point = other_693;
    let _e11: Point = self_835;
    let _e14: Point = other_693;
    return AntiScalar(((_e4.g0_.y * _e7.g0_.y) + (_e11.g0_.z * _e14.g0_.z)));
}

fn point_point_scalar_product(self_836: Point, other_694: Point) -> Scalar {
    var self_837: Point;
    var other_695: Point;

    self_837 = self_836;
    other_695 = other_694;
    let _e5: Point = self_837;
    let _e8: Point = other_695;
    return Scalar((0.0 - (_e5.g0_.x * _e8.g0_.x)));
}

fn point_point_anti_scalar_product(self_838: Point, other_696: Point) -> AntiScalar {
    var self_839: Point;
    var other_697: Point;

    self_839 = self_838;
    other_697 = other_696;
    let _e4: Point = self_839;
    let _e7: Point = other_697;
    let _e11: Point = self_839;
    let _e14: Point = other_697;
    return AntiScalar(((_e4.g0_.y * _e7.g0_.y) + (_e11.g0_.z * _e14.g0_.z)));
}

fn point_ideal_point_into(self_840: Point) -> IdealPoint {
    var self_841: Point;

    self_841 = self_840;
    let _e2: Point = self_841;
    let _e5: Point = self_841;
    return IdealPoint(vec2<f32>(_e2.g0_.y, _e5.g0_.z));
}

fn point_ideal_point_add(self_842: Point, other_698: IdealPoint) -> Point {
    var self_843: Point;
    var other_699: IdealPoint;

    self_843 = self_842;
    other_699 = other_698;
    let _e4: Point = self_843;
    let _e6: IdealPoint = other_699;
    let _e9: IdealPoint = other_699;
    let _e12: IdealPoint = other_699;
    return Point((_e4.g0_ + (vec3<f32>(_e6.g0_.x, _e9.g0_.x, _e12.g0_.y) * vec3<f32>(0.0, 1.0, 1.0))));
}

fn point_ideal_point_sub(self_844: Point, other_700: IdealPoint) -> Point {
    var self_845: Point;
    var other_701: IdealPoint;

    self_845 = self_844;
    other_701 = other_700;
    let _e4: Point = self_845;
    let _e6: IdealPoint = other_701;
    let _e9: IdealPoint = other_701;
    let _e12: IdealPoint = other_701;
    return Point((_e4.g0_ - (vec3<f32>(_e6.g0_.x, _e9.g0_.x, _e12.g0_.y) * vec3<f32>(0.0, 1.0, 1.0))));
}

fn point_ideal_point_geometric_product(self_846: Point, other_702: IdealPoint) -> IdealPoint {
    var self_847: Point;
    var other_703: IdealPoint;

    self_847 = self_846;
    other_703 = other_702;
    let _e4: Point = self_847;
    let _e8: IdealPoint = other_703;
    return IdealPoint(((vec2<f32>(_e4.g0_.x) * _e8.g0_.yx) * vec2<f32>(-(1.0), 1.0)));
}

fn point_ideal_point_regressive_product(self_848: Point, other_704: IdealPoint) -> Plane {
    var self_849: Point;
    var other_705: IdealPoint;

    self_849 = self_848;
    other_705 = other_704;
    let _e4: Point = self_849;
    let _e8: IdealPoint = other_705;
    let _e18: Point = self_849;
    let _e21: IdealPoint = other_705;
    let _e24: IdealPoint = other_705;
    let _e27: IdealPoint = other_705;
    return Plane((((vec3<f32>(_e4.g0_.z) * vec3<f32>(_e8.g0_.x)) * vec3<f32>(1.0, 0.0, 0.0)) + ((_e18.g0_.yxx * vec3<f32>(_e21.g0_.y, _e24.g0_.y, _e27.g0_.x)) * vec3<f32>(-(1.0), 1.0, -(1.0)))));
}

fn point_ideal_point_geometric_anti_product(self_850: Point, other_706: IdealPoint) -> MotorDual {
    var self_851: Point;
    var other_707: IdealPoint;

    self_851 = self_850;
    other_707 = other_706;
    let _e4: Point = self_851;
    let _e8: IdealPoint = other_707;
    let _e11: IdealPoint = other_707;
    let _e14: IdealPoint = other_707;
    let _e17: IdealPoint = other_707;
    let _e28: Point = self_851;
    let _e31: Point = self_851;
    let _e34: Point = self_851;
    let _e37: Point = self_851;
    let _e41: IdealPoint = other_707;
    let _e44: IdealPoint = other_707;
    let _e47: IdealPoint = other_707;
    let _e50: IdealPoint = other_707;
    return MotorDual((((vec4<f32>(_e4.g0_.z) * vec4<f32>(_e8.g0_.y, _e11.g0_.x, _e14.g0_.y, _e17.g0_.y)) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) + ((vec4<f32>(_e28.g0_.y, _e31.g0_.y, _e34.g0_.x, _e37.g0_.x) * vec4<f32>(_e41.g0_.x, _e44.g0_.y, _e47.g0_.y, _e50.g0_.x)) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))));
}

fn point_ideal_point_inner_anti_product(self_852: Point, other_708: IdealPoint) -> AntiScalar {
    var self_853: Point;
    var other_709: IdealPoint;

    self_853 = self_852;
    other_709 = other_708;
    let _e4: Point = self_853;
    let _e7: IdealPoint = other_709;
    let _e11: Point = self_853;
    let _e14: IdealPoint = other_709;
    return AntiScalar(((_e4.g0_.y * _e7.g0_.x) + (_e11.g0_.z * _e14.g0_.y)));
}

fn point_ideal_point_left_anti_contraction(self_854: Point, other_710: IdealPoint) -> AntiScalar {
    var self_855: Point;
    var other_711: IdealPoint;

    self_855 = self_854;
    other_711 = other_710;
    let _e4: Point = self_855;
    let _e7: IdealPoint = other_711;
    let _e11: Point = self_855;
    let _e14: IdealPoint = other_711;
    return AntiScalar(((_e4.g0_.y * _e7.g0_.x) + (_e11.g0_.z * _e14.g0_.y)));
}

fn point_ideal_point_right_anti_contraction(self_856: Point, other_712: IdealPoint) -> AntiScalar {
    var self_857: Point;
    var other_713: IdealPoint;

    self_857 = self_856;
    other_713 = other_712;
    let _e4: Point = self_857;
    let _e7: IdealPoint = other_713;
    let _e11: Point = self_857;
    let _e14: IdealPoint = other_713;
    return AntiScalar(((_e4.g0_.y * _e7.g0_.x) + (_e11.g0_.z * _e14.g0_.y)));
}

fn point_ideal_point_anti_scalar_product(self_858: Point, other_714: IdealPoint) -> AntiScalar {
    var self_859: Point;
    var other_715: IdealPoint;

    self_859 = self_858;
    other_715 = other_714;
    let _e4: Point = self_859;
    let _e7: IdealPoint = other_715;
    let _e11: Point = self_859;
    let _e14: IdealPoint = other_715;
    return AntiScalar(((_e4.g0_.y * _e7.g0_.x) + (_e11.g0_.z * _e14.g0_.y)));
}

fn point_plane_geometric_product(self_860: Point, other_716: Plane) -> MotorDual {
    var self_861: Point;
    var other_717: Plane;

    self_861 = self_860;
    other_717 = other_716;
    let _e4: Point = self_861;
    let _e8: Plane = other_717;
    let _e11: Plane = other_717;
    let _e14: Plane = other_717;
    let _e17: Plane = other_717;
    let _e28: Point = self_861;
    let _e32: Plane = other_717;
    let _e35: Plane = other_717;
    let _e38: Plane = other_717;
    let _e41: Plane = other_717;
    let _e54: Point = self_861;
    let _e58: Plane = other_717;
    let _e61: Plane = other_717;
    let _e64: Plane = other_717;
    let _e67: Plane = other_717;
    return MotorDual(((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y, _e11.g0_.z, _e14.g0_.y, _e17.g0_.y)) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) + ((vec4<f32>(_e28.g0_.z) * vec4<f32>(_e32.g0_.z, _e35.g0_.y, _e38.g0_.z, _e41.g0_.z)) * vec4<f32>(1.0, -(1.0), 0.0, 0.0))) + ((vec4<f32>(_e54.g0_.x) * vec4<f32>(_e58.g0_.x, _e61.g0_.x, _e64.g0_.z, _e67.g0_.y)) * vec4<f32>(1.0, 0.0, -(1.0), 1.0))));
}

fn point_plane_regressive_product(self_862: Point, other_718: Plane) -> Scalar {
    var self_863: Point;
    var other_719: Plane;

    self_863 = self_862;
    other_719 = other_718;
    let _e4: Point = self_863;
    let _e7: Plane = other_719;
    let _e11: Point = self_863;
    let _e14: Plane = other_719;
    let _e19: Point = self_863;
    let _e22: Plane = other_719;
    return Scalar((((_e4.g0_.x * _e7.g0_.x) + (_e11.g0_.y * _e14.g0_.y)) + (_e19.g0_.z * _e22.g0_.z)));
}

fn point_plane_outer_product(self_864: Point, other_720: Plane) -> AntiScalar {
    var self_865: Point;
    var other_721: Plane;

    self_865 = self_864;
    other_721 = other_720;
    let _e4: Point = self_865;
    let _e7: Plane = other_721;
    let _e11: Point = self_865;
    let _e14: Plane = other_721;
    let _e19: Point = self_865;
    let _e22: Plane = other_721;
    return AntiScalar((((_e4.g0_.x * _e7.g0_.x) + (_e11.g0_.y * _e14.g0_.y)) + (_e19.g0_.z * _e22.g0_.z)));
}

fn point_plane_inner_product(self_866: Point, other_722: Plane) -> Plane {
    var self_867: Point;
    var other_723: Plane;

    self_867 = self_866;
    other_723 = other_722;
    let _e4: Point = self_867;
    let _e8: Plane = other_723;
    let _e19: Point = self_867;
    let _e22: Plane = other_723;
    return Plane((((vec3<f32>(_e4.g0_.z) * vec3<f32>(_e8.g0_.y)) * vec3<f32>(-(1.0), 0.0, 0.0)) + ((_e19.g0_.yxx * _e22.g0_.zzy) * vec3<f32>(1.0, -(1.0), 1.0))));
}

fn point_plane_geometric_anti_product(self_868: Point, other_724: Plane) -> Motor {
    var self_869: Point;
    var other_725: Plane;

    self_869 = self_868;
    other_725 = other_724;
    let _e4: Point = self_869;
    let _e8: Plane = other_725;
    let _e11: Plane = other_725;
    let _e14: Plane = other_725;
    let _e17: Plane = other_725;
    let _e29: Point = self_869;
    let _e33: Plane = other_725;
    let _e36: Plane = other_725;
    let _e39: Plane = other_725;
    let _e42: Plane = other_725;
    let _e55: Point = self_869;
    let _e59: Plane = other_725;
    return Motor(((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y, _e11.g0_.z, _e14.g0_.y, _e17.g0_.x)) * vec4<f32>(1.0, 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e29.g0_.z) * vec4<f32>(_e33.g0_.z, _e36.g0_.y, _e39.g0_.x, _e42.g0_.z)) * vec4<f32>(1.0, -(1.0), 1.0, 0.0))) + ((vec4<f32>(_e55.g0_.x) * vec4<f32>(_e59.g0_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))));
}

fn point_plane_inner_anti_product(self_870: Point, other_726: Plane) -> Point {
    var self_871: Point;
    var other_727: Plane;

    self_871 = self_870;
    other_727 = other_726;
    let _e4: Point = self_871;
    let _e8: Plane = other_727;
    let _e18: Point = self_871;
    let _e21: Plane = other_727;
    return Point((((vec3<f32>(_e4.g0_.z) * _e8.g0_.yxy) * vec3<f32>(-(1.0), 1.0, 0.0)) + ((_e18.g0_.yxy * _e21.g0_.zxx) * vec3<f32>(1.0, 0.0, -(1.0)))));
}

fn point_plane_right_contraction(self_872: Point, other_728: Plane) -> Plane {
    var self_873: Point;
    var other_729: Plane;

    self_873 = self_872;
    other_729 = other_728;
    let _e4: Point = self_873;
    let _e8: Plane = other_729;
    let _e19: Point = self_873;
    let _e22: Plane = other_729;
    return Plane((((vec3<f32>(_e4.g0_.z) * vec3<f32>(_e8.g0_.y)) * vec3<f32>(-(1.0), 0.0, 0.0)) + ((_e19.g0_.yxx * _e22.g0_.zzy) * vec3<f32>(1.0, -(1.0), 1.0))));
}

fn point_plane_left_anti_contraction(self_874: Point, other_730: Plane) -> Point {
    var self_875: Point;
    var other_731: Plane;

    self_875 = self_874;
    other_731 = other_730;
    let _e4: Point = self_875;
    let _e8: Plane = other_731;
    let _e18: Point = self_875;
    let _e21: Plane = other_731;
    return Point((((vec3<f32>(_e4.g0_.z) * _e8.g0_.yxy) * vec3<f32>(-(1.0), 1.0, 0.0)) + ((_e18.g0_.yxy * _e21.g0_.zxx) * vec3<f32>(1.0, 0.0, -(1.0)))));
}

fn point_translator_add(self_876: Point, other_732: Translator) -> Motor {
    var self_877: Point;
    var other_733: Translator;

    self_877 = self_876;
    other_733 = other_732;
    let _e4: Point = self_877;
    let _e7: Point = self_877;
    let _e10: Point = self_877;
    let _e13: Point = self_877;
    let _e23: Translator = other_733;
    let _e26: Translator = other_733;
    let _e29: Translator = other_733;
    let _e32: Translator = other_733;
    return Motor(((vec4<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.y, _e13.g0_.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0)) + (vec4<f32>(_e23.g0_.x, _e26.g0_.x, _e29.g0_.y, _e32.g0_.z) * vec4<f32>(1.0, 0.0, 1.0, 1.0))));
}

fn point_translator_sub(self_878: Point, other_734: Translator) -> Motor {
    var self_879: Point;
    var other_735: Translator;

    self_879 = self_878;
    other_735 = other_734;
    let _e4: Point = self_879;
    let _e7: Point = self_879;
    let _e10: Point = self_879;
    let _e13: Point = self_879;
    let _e23: Translator = other_735;
    let _e26: Translator = other_735;
    let _e29: Translator = other_735;
    let _e32: Translator = other_735;
    return Motor(((vec4<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.y, _e13.g0_.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0)) - (vec4<f32>(_e23.g0_.x, _e26.g0_.x, _e29.g0_.y, _e32.g0_.z) * vec4<f32>(1.0, 0.0, 1.0, 1.0))));
}

fn point_translator_geometric_product(self_880: Point, other_736: Translator) -> Point {
    var self_881: Point;
    var other_737: Translator;

    self_881 = self_880;
    other_737 = other_736;
    let _e4: Point = self_881;
    let _e8: Translator = other_737;
    let _e18: Point = self_881;
    let _e20: Translator = other_737;
    return Point((((vec3<f32>(_e4.g0_.x) * _e8.g0_.xzy) * vec3<f32>(1.0, -(1.0), 1.0)) + ((_e18.g0_ * vec3<f32>(_e20.g0_.x)) * vec3<f32>(0.0, 1.0, 1.0))));
}

fn point_translator_regressive_product(self_882: Point, other_738: Translator) -> Plane {
    var self_883: Point;
    var other_739: Translator;

    self_883 = self_882;
    other_739 = other_738;
    let _e4: Point = self_883;
    let _e8: Translator = other_739;
    let _e18: Point = self_883;
    let _e21: Translator = other_739;
    return Plane((((vec3<f32>(_e4.g0_.z) * vec3<f32>(_e8.g0_.y)) * vec3<f32>(1.0, 0.0, 0.0)) + ((_e18.g0_.yxx * _e21.g0_.zzy) * vec3<f32>(-(1.0), 1.0, -(1.0)))));
}

fn point_translator_outer_product(self_884: Point, other_740: Translator) -> Point {
    var self_885: Point;
    var other_741: Translator;

    self_885 = self_884;
    other_741 = other_740;
    let _e4: Point = self_885;
    let _e6: Translator = other_741;
    return Point((_e4.g0_ * vec3<f32>(_e6.g0_.x)));
}

fn point_translator_inner_product(self_886: Point, other_742: Translator) -> Point {
    var self_887: Point;
    var other_743: Translator;

    self_887 = self_886;
    other_743 = other_742;
    let _e4: Point = self_887;
    let _e6: Translator = other_743;
    return Point((_e4.g0_ * vec3<f32>(_e6.g0_.x)));
}

fn point_translator_geometric_anti_product(self_888: Point, other_744: Translator) -> MotorDual {
    var self_889: Point;
    var other_745: Translator;

    self_889 = self_888;
    other_745 = other_744;
    let _e4: Point = self_889;
    let _e8: Translator = other_745;
    let _e11: Translator = other_745;
    let _e14: Translator = other_745;
    let _e17: Translator = other_745;
    let _e29: Point = self_889;
    let _e33: Translator = other_745;
    let _e36: Translator = other_745;
    let _e39: Translator = other_745;
    let _e42: Translator = other_745;
    let _e54: Point = self_889;
    let _e58: Translator = other_745;
    let _e61: Translator = other_745;
    let _e64: Translator = other_745;
    let _e67: Translator = other_745;
    return MotorDual(((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y, _e11.g0_.z, _e14.g0_.x, _e17.g0_.y)) * vec4<f32>(1.0, -(1.0), 1.0, 0.0)) + ((vec4<f32>(_e29.g0_.z) * vec4<f32>(_e33.g0_.z, _e36.g0_.y, _e39.g0_.z, _e42.g0_.x)) * vec4<f32>(1.0, 1.0, 0.0, 1.0))) + ((vec4<f32>(_e54.g0_.x) * vec4<f32>(_e58.g0_.x, _e61.g0_.x, _e64.g0_.z, _e67.g0_.y)) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))));
}

fn point_translator_right_contraction(self_890: Point, other_746: Translator) -> Point {
    var self_891: Point;
    var other_747: Translator;

    self_891 = self_890;
    other_747 = other_746;
    let _e4: Point = self_891;
    let _e6: Translator = other_747;
    return Point((_e4.g0_ * vec3<f32>(_e6.g0_.x)));
}

fn point_translator_right_anti_contraction(self_892: Point, other_748: Translator) -> AntiScalar {
    var self_893: Point;
    var other_749: Translator;

    self_893 = self_892;
    other_749 = other_748;
    let _e4: Point = self_893;
    let _e7: Translator = other_749;
    let _e11: Point = self_893;
    let _e14: Translator = other_749;
    return AntiScalar(((_e4.g0_.y * _e7.g0_.y) + (_e11.g0_.z * _e14.g0_.z)));
}

fn point_translator_anti_scalar_product(self_894: Point, other_750: Translator) -> AntiScalar {
    var self_895: Point;
    var other_751: Translator;

    self_895 = self_894;
    other_751 = other_750;
    let _e4: Point = self_895;
    let _e7: Translator = other_751;
    let _e11: Point = self_895;
    let _e14: Translator = other_751;
    return AntiScalar(((_e4.g0_.y * _e7.g0_.y) + (_e11.g0_.z * _e14.g0_.z)));
}

fn point_motor_add(self_896: Point, other_752: Motor) -> Motor {
    var self_897: Point;
    var other_753: Motor;

    self_897 = self_896;
    other_753 = other_752;
    let _e4: Point = self_897;
    let _e7: Point = self_897;
    let _e10: Point = self_897;
    let _e13: Point = self_897;
    let _e23: Motor = other_753;
    return Motor(((vec4<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.y, _e13.g0_.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0)) + _e23.g0_));
}

fn point_motor_sub(self_898: Point, other_754: Motor) -> Motor {
    var self_899: Point;
    var other_755: Motor;

    self_899 = self_898;
    other_755 = other_754;
    let _e4: Point = self_899;
    let _e7: Point = self_899;
    let _e10: Point = self_899;
    let _e13: Point = self_899;
    let _e23: Motor = other_755;
    return Motor(((vec4<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.y, _e13.g0_.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0)) - _e23.g0_));
}

fn point_motor_geometric_product(self_900: Point, other_756: Motor) -> Motor {
    var self_901: Point;
    var other_757: Motor;

    self_901 = self_900;
    other_757 = other_756;
    let _e4: Point = self_901;
    let _e8: Motor = other_757;
    let _e20: Point = self_901;
    let _e24: Motor = other_757;
    let _e35: Point = self_901;
    let _e38: Point = self_901;
    let _e41: Point = self_901;
    let _e44: Point = self_901;
    let _e48: Motor = other_757;
    return Motor(((((vec4<f32>(_e4.g0_.x) * _e8.g0_.yxwz) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0)) + ((vec4<f32>(_e20.g0_.z) * _e24.g0_.yyyx) * vec4<f32>(0.0, 0.0, 1.0, 1.0))) + ((vec4<f32>(_e35.g0_.x, _e38.g0_.x, _e41.g0_.y, _e44.g0_.y) * _e48.g0_.xxxy) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))));
}

fn point_motor_regressive_product(self_902: Point, other_758: Motor) -> Plane {
    var self_903: Point;
    var other_759: Motor;

    self_903 = self_902;
    other_759 = other_758;
    let _e4: Point = self_903;
    let _e8: Motor = other_759;
    let _e11: Motor = other_759;
    let _e14: Motor = other_759;
    let _e25: Point = self_903;
    let _e29: Motor = other_759;
    let _e32: Motor = other_759;
    let _e35: Motor = other_759;
    let _e47: Point = self_903;
    let _e51: Motor = other_759;
    let _e54: Motor = other_759;
    let _e57: Motor = other_759;
    return Plane(((((vec3<f32>(_e4.g0_.y) * vec3<f32>(_e8.g0_.w, _e11.g0_.w, _e14.g0_.y)) * vec3<f32>(-(1.0), 0.0, 1.0)) + ((vec3<f32>(_e25.g0_.z) * vec3<f32>(_e29.g0_.z, _e32.g0_.y, _e35.g0_.z)) * vec3<f32>(1.0, -(1.0), 0.0))) + ((vec3<f32>(_e47.g0_.x) * vec3<f32>(_e51.g0_.x, _e54.g0_.w, _e57.g0_.z)) * vec3<f32>(0.0, 1.0, -(1.0)))));
}

fn point_motor_outer_product(self_904: Point, other_760: Motor) -> Point {
    var self_905: Point;
    var other_761: Motor;

    self_905 = self_904;
    other_761 = other_760;
    let _e4: Point = self_905;
    let _e6: Motor = other_761;
    return Point((_e4.g0_ * vec3<f32>(_e6.g0_.x)));
}

fn point_motor_inner_product(self_906: Point, other_762: Motor) -> Motor {
    var self_907: Point;
    var other_763: Motor;

    self_907 = self_906;
    other_763 = other_762;
    let _e4: Point = self_907;
    let _e7: Point = self_907;
    let _e10: Point = self_907;
    let _e13: Point = self_907;
    let _e17: Motor = other_763;
    return Motor(((vec4<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.y, _e13.g0_.z) * _e17.g0_.yxxx) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0)));
}

fn point_motor_geometric_anti_product(self_908: Point, other_764: Motor) -> MotorDual {
    var self_909: Point;
    var other_765: Motor;

    self_909 = self_908;
    other_765 = other_764;
    let _e4: Point = self_909;
    let _e8: Motor = other_765;
    let _e19: Point = self_909;
    let _e23: Motor = other_765;
    let _e35: Point = self_909;
    let _e39: Motor = other_765;
    return MotorDual(((((vec4<f32>(_e4.g0_.y) * _e8.g0_.zwxy) * vec4<f32>(1.0, -(1.0), 1.0, 1.0)) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.wzyx) * vec4<f32>(1.0, 1.0, -(1.0), 1.0))) + ((vec4<f32>(_e35.g0_.x) * _e39.g0_.xxwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))));
}

fn point_motor_left_contraction(self_910: Point, other_766: Motor) -> Scalar {
    var self_911: Point;
    var other_767: Motor;

    self_911 = self_910;
    other_767 = other_766;
    let _e5: Point = self_911;
    let _e8: Motor = other_767;
    return Scalar((0.0 - (_e5.g0_.x * _e8.g0_.y)));
}

fn point_motor_right_contraction(self_912: Point, other_768: Motor) -> Motor {
    var self_913: Point;
    var other_769: Motor;

    self_913 = self_912;
    other_769 = other_768;
    let _e4: Point = self_913;
    let _e7: Point = self_913;
    let _e10: Point = self_913;
    let _e13: Point = self_913;
    let _e17: Motor = other_769;
    return Motor(((vec4<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.y, _e13.g0_.z) * _e17.g0_.yxxx) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0)));
}

fn point_motor_right_anti_contraction(self_914: Point, other_770: Motor) -> AntiScalar {
    var self_915: Point;
    var other_771: Motor;

    self_915 = self_914;
    other_771 = other_770;
    let _e4: Point = self_915;
    let _e7: Motor = other_771;
    let _e11: Point = self_915;
    let _e14: Motor = other_771;
    return AntiScalar(((_e4.g0_.y * _e7.g0_.z) + (_e11.g0_.z * _e14.g0_.w)));
}

fn point_motor_scalar_product(self_916: Point, other_772: Motor) -> Scalar {
    var self_917: Point;
    var other_773: Motor;

    self_917 = self_916;
    other_773 = other_772;
    let _e5: Point = self_917;
    let _e8: Motor = other_773;
    return Scalar((0.0 - (_e5.g0_.x * _e8.g0_.y)));
}

fn point_motor_anti_scalar_product(self_918: Point, other_774: Motor) -> AntiScalar {
    var self_919: Point;
    var other_775: Motor;

    self_919 = self_918;
    other_775 = other_774;
    let _e4: Point = self_919;
    let _e7: Motor = other_775;
    let _e11: Point = self_919;
    let _e14: Motor = other_775;
    return AntiScalar(((_e4.g0_.y * _e7.g0_.z) + (_e11.g0_.z * _e14.g0_.w)));
}

fn point_motor_dual_geometric_product(self_920: Point, other_776: MotorDual) -> MotorDual {
    var self_921: Point;
    var other_777: MotorDual;

    self_921 = self_920;
    other_777 = other_776;
    let _e4: Point = self_921;
    let _e8: MotorDual = other_777;
    let _e20: Point = self_921;
    let _e24: MotorDual = other_777;
    let _e36: Point = self_921;
    let _e39: Point = self_921;
    let _e42: Point = self_921;
    let _e45: Point = self_921;
    let _e49: MotorDual = other_777;
    return MotorDual(((((vec4<f32>(_e4.g0_.x) * _e8.g0_.yxwz) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0)) + ((vec4<f32>(_e20.g0_.z) * _e24.g0_.wzww) * vec4<f32>(1.0, -(1.0), 0.0, 0.0))) + ((vec4<f32>(_e36.g0_.y, _e39.g0_.y, _e42.g0_.x, _e45.g0_.x) * _e49.g0_.zwxx) * vec4<f32>(1.0, 1.0, 0.0, 0.0))));
}

fn point_motor_dual_regressive_product(self_922: Point, other_778: MotorDual) -> Motor {
    var self_923: Point;
    var other_779: MotorDual;

    self_923 = self_922;
    other_779 = other_778;
    let _e4: Point = self_923;
    let _e8: MotorDual = other_779;
    let _e18: Point = self_923;
    let _e22: MotorDual = other_779;
    let _e33: Point = self_923;
    let _e37: MotorDual = other_779;
    return Motor(((((vec4<f32>(_e4.g0_.y) * _e8.g0_.zzxz) * vec4<f32>(1.0, 0.0, 1.0, 0.0)) + ((vec4<f32>(_e18.g0_.z) * _e22.g0_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e33.g0_.x) * _e37.g0_.yxxx) * vec4<f32>(1.0, 1.0, 0.0, 0.0))));
}

fn point_motor_dual_outer_product(self_924: Point, other_780: MotorDual) -> AntiScalar {
    var self_925: Point;
    var other_781: MotorDual;

    self_925 = self_924;
    other_781 = other_780;
    let _e4: Point = self_925;
    let _e7: MotorDual = other_781;
    let _e11: Point = self_925;
    let _e14: MotorDual = other_781;
    let _e19: Point = self_925;
    let _e22: MotorDual = other_781;
    return AntiScalar((((_e4.g0_.x * _e7.g0_.y) + (_e11.g0_.y * _e14.g0_.z)) + (_e19.g0_.z * _e22.g0_.w)));
}

fn point_motor_dual_inner_product(self_926: Point, other_782: MotorDual) -> Plane {
    var self_927: Point;
    var other_783: MotorDual;

    self_927 = self_926;
    other_783 = other_782;
    let _e4: Point = self_927;
    let _e8: MotorDual = other_783;
    let _e11: MotorDual = other_783;
    let _e14: MotorDual = other_783;
    let _e26: Point = self_927;
    let _e30: MotorDual = other_783;
    let _e42: Point = self_927;
    let _e45: MotorDual = other_783;
    let _e48: MotorDual = other_783;
    let _e51: MotorDual = other_783;
    return Plane(((((vec3<f32>(_e4.g0_.x) * vec3<f32>(_e8.g0_.x, _e11.g0_.w, _e14.g0_.z)) * vec3<f32>(-(1.0), -(1.0), 1.0)) + ((vec3<f32>(_e26.g0_.z) * vec3<f32>(_e30.g0_.z)) * vec3<f32>(-(1.0), 0.0, 0.0))) + ((_e42.g0_.yxx * vec3<f32>(_e45.g0_.w, _e48.g0_.x, _e51.g0_.x)) * vec3<f32>(1.0, 0.0, 0.0))));
}

fn point_motor_dual_geometric_anti_product(self_928: Point, other_784: MotorDual) -> Motor {
    var self_929: Point;
    var other_785: MotorDual;

    self_929 = self_928;
    other_785 = other_784;
    let _e4: Point = self_929;
    let _e8: MotorDual = other_785;
    let _e19: Point = self_929;
    let _e23: MotorDual = other_785;
    let _e35: Point = self_929;
    let _e39: MotorDual = other_785;
    return Motor(((((vec4<f32>(_e4.g0_.y) * _e8.g0_.zwxy) * vec4<f32>(1.0, 1.0, 1.0, -(1.0))) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.wzyx) * vec4<f32>(1.0, -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e35.g0_.x) * _e39.g0_.yxxx) * vec4<f32>(1.0, 1.0, 0.0, 0.0))));
}

fn point_motor_dual_inner_anti_product(self_930: Point, other_786: MotorDual) -> Point {
    var self_931: Point;
    var other_787: MotorDual;

    self_931 = self_930;
    other_787 = other_786;
    let _e4: Point = self_931;
    let _e8: MotorDual = other_787;
    let _e11: MotorDual = other_787;
    let _e14: MotorDual = other_787;
    let _e25: Point = self_931;
    let _e29: MotorDual = other_787;
    let _e32: MotorDual = other_787;
    let _e35: MotorDual = other_787;
    let _e47: Point = self_931;
    let _e51: MotorDual = other_787;
    return Point(((((vec3<f32>(_e4.g0_.y) * vec3<f32>(_e8.g0_.w, _e11.g0_.x, _e14.g0_.y)) * vec3<f32>(1.0, 1.0, -(1.0))) + ((vec3<f32>(_e25.g0_.z) * vec3<f32>(_e29.g0_.z, _e32.g0_.y, _e35.g0_.x)) * vec3<f32>(-(1.0), 1.0, 1.0))) + ((vec3<f32>(_e47.g0_.x) * vec3<f32>(_e51.g0_.x)) * vec3<f32>(1.0, 0.0, 0.0))));
}

fn point_motor_dual_right_contraction(self_932: Point, other_788: MotorDual) -> Plane {
    var self_933: Point;
    var other_789: MotorDual;

    self_933 = self_932;
    other_789 = other_788;
    let _e4: Point = self_933;
    let _e8: MotorDual = other_789;
    let _e19: Point = self_933;
    let _e22: MotorDual = other_789;
    let _e25: MotorDual = other_789;
    let _e28: MotorDual = other_789;
    return Plane((((vec3<f32>(_e4.g0_.z) * vec3<f32>(_e8.g0_.z)) * vec3<f32>(-(1.0), 0.0, 0.0)) + ((_e19.g0_.yxx * vec3<f32>(_e22.g0_.w, _e25.g0_.w, _e28.g0_.z)) * vec3<f32>(1.0, -(1.0), 1.0))));
}

fn point_motor_dual_left_anti_contraction(self_934: Point, other_790: MotorDual) -> Point {
    var self_935: Point;
    var other_791: MotorDual;

    self_935 = self_934;
    other_791 = other_790;
    let _e4: Point = self_935;
    let _e8: MotorDual = other_791;
    let _e11: MotorDual = other_791;
    let _e14: MotorDual = other_791;
    let _e25: Point = self_935;
    let _e28: MotorDual = other_791;
    let _e31: MotorDual = other_791;
    let _e34: MotorDual = other_791;
    return Point((((vec3<f32>(_e4.g0_.z) * vec3<f32>(_e8.g0_.z, _e11.g0_.y, _e14.g0_.z)) * vec3<f32>(-(1.0), 1.0, 0.0)) + ((_e25.g0_.yxy * vec3<f32>(_e28.g0_.w, _e31.g0_.x, _e34.g0_.y)) * vec3<f32>(1.0, 0.0, -(1.0)))));
}

fn point_motor_dual_right_anti_contraction(self_936: Point, other_792: MotorDual) -> Point {
    var self_937: Point;
    var other_793: MotorDual;

    self_937 = self_936;
    other_793 = other_792;
    let _e4: Point = self_937;
    let _e6: MotorDual = other_793;
    return Point((_e4.g0_ * vec3<f32>(_e6.g0_.x)));
}

fn point_squared_magnitude(self_938: Point) -> Scalar {
    var self_939: Point;

    self_939 = self_938;
    let _e2: Point = self_939;
    let _e3: Point = self_939;
    let _e4: Point = point_reversal(_e3);
    let _e5: Scalar = point_point_scalar_product(_e2, _e4);
    return _e5;
}

fn point_magnitude(self_940: Point) -> Scalar {
    var self_941: Point;

    self_941 = self_940;
    let _e2: Point = self_941;
    let _e3: Scalar = point_squared_magnitude(_e2);
    return Scalar(sqrt(_e3.g0_));
}

fn point_bulk_norm(self_942: Point) -> Scalar {
    var self_943: Point;

    self_943 = self_942;
    let _e2: Point = self_943;
    let _e3: Scalar = point_squared_magnitude(_e2);
    return Scalar(sqrt(_e3.g0_));
}

fn point_squared_anti_magnitude(self_944: Point) -> AntiScalar {
    var self_945: Point;

    self_945 = self_944;
    let _e2: Point = self_945;
    let _e3: Point = self_945;
    let _e4: Point = point_anti_reversal(_e3);
    let _e5: AntiScalar = point_point_anti_scalar_product(_e2, _e4);
    return _e5;
}

fn point_weight_norm(self_946: Point) -> AntiScalar {
    var self_947: Point;

    self_947 = self_946;
    let _e2: Point = self_947;
    let _e3: AntiScalar = point_squared_anti_magnitude(_e2);
    return AntiScalar(sqrt(_e3.g0_));
}

fn point_scale(self_948: Point, other_794: f32) -> Point {
    var self_949: Point;
    var other_795: f32;

    self_949 = self_948;
    other_795 = other_794;
    let _e4: Point = self_949;
    let _e5: f32 = other_795;
    let _e7: Point = point_scalar_geometric_product(_e4, Scalar(_e5));
    return _e7;
}

fn point_signum(self_950: Point) -> Point {
    var self_951: Point;

    self_951 = self_950;
    let _e2: Point = self_951;
    let _e3: Point = self_951;
    let _e4: Scalar = point_magnitude(_e3);
    let _e9: Point = point_scalar_geometric_product(_e2, Scalar((1.0 / _e4.g0_)));
    return _e9;
}

fn point_inverse(self_952: Point) -> Point {
    var self_953: Point;

    self_953 = self_952;
    let _e2: Point = self_953;
    let _e3: Point = point_reversal(_e2);
    let _e4: Point = self_953;
    let _e5: Scalar = point_squared_magnitude(_e4);
    let _e10: Point = point_scalar_geometric_product(_e3, Scalar((1.0 / _e5.g0_)));
    return _e10;
}

fn point_unitize(self_954: Point) -> Point {
    var self_955: Point;

    self_955 = self_954;
    let _e2: Point = self_955;
    let _e3: Point = self_955;
    let _e4: AntiScalar = point_weight_norm(_e3);
    let _e9: Point = point_scalar_geometric_product(_e2, Scalar((1.0 / _e4.g0_)));
    return _e9;
}

fn ideal_point_zero() -> IdealPoint {
    return IdealPoint(vec2<f32>(0.0));
}

fn ideal_point_one() -> IdealPoint {
    return IdealPoint(vec2<f32>(0.0));
}

fn ideal_point_grade(self_956: IdealPoint) -> i32 {
    return 2;
}

fn ideal_point_anti_grade(self_957: IdealPoint) -> i32 {
    return 1;
}

fn ideal_point_neg(self_958: IdealPoint) -> IdealPoint {
    var self_959: IdealPoint;

    self_959 = self_958;
    let _e2: IdealPoint = self_959;
    return IdealPoint((_e2.g0_ * vec2<f32>(-(1.0))));
}

fn ideal_point_automorphism(self_960: IdealPoint) -> IdealPoint {
    var self_961: IdealPoint;

    self_961 = self_960;
    let _e2: IdealPoint = self_961;
    return IdealPoint(_e2.g0_);
}

fn ideal_point_reversal(self_962: IdealPoint) -> IdealPoint {
    var self_963: IdealPoint;

    self_963 = self_962;
    let _e2: IdealPoint = self_963;
    return IdealPoint((_e2.g0_ * vec2<f32>(-(1.0))));
}

fn ideal_point_conjugation(self_964: IdealPoint) -> IdealPoint {
    var self_965: IdealPoint;

    self_965 = self_964;
    let _e2: IdealPoint = self_965;
    return IdealPoint((_e2.g0_ * vec2<f32>(-(1.0))));
}

fn ideal_point_anti_reversal(self_966: IdealPoint) -> IdealPoint {
    var self_967: IdealPoint;

    self_967 = self_966;
    let _e2: IdealPoint = self_967;
    return IdealPoint((_e2.g0_ * vec2<f32>(-(1.0))));
}

fn ideal_point_double_complement(self_968: IdealPoint) -> IdealPoint {
    var self_969: IdealPoint;

    self_969 = self_968;
    let _e2: IdealPoint = self_969;
    return IdealPoint(_e2.g0_);
}

fn ideal_point_scalar_add(self_970: IdealPoint, other_796: Scalar) -> Translator {
    var self_971: IdealPoint;
    var other_797: Scalar;

    self_971 = self_970;
    other_797 = other_796;
    let _e4: IdealPoint = self_971;
    let _e7: IdealPoint = self_971;
    let _e10: IdealPoint = self_971;
    let _e19: Scalar = other_797;
    return Translator(((vec3<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.y) * vec3<f32>(0.0, 1.0, 1.0)) + (vec3<f32>(_e19.g0_) * vec3<f32>(1.0, 0.0, 0.0))));
}

fn ideal_point_scalar_sub(self_972: IdealPoint, other_798: Scalar) -> Translator {
    var self_973: IdealPoint;
    var other_799: Scalar;

    self_973 = self_972;
    other_799 = other_798;
    let _e4: IdealPoint = self_973;
    let _e7: IdealPoint = self_973;
    let _e10: IdealPoint = self_973;
    let _e19: Scalar = other_799;
    return Translator(((vec3<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.y) * vec3<f32>(0.0, 1.0, 1.0)) - (vec3<f32>(_e19.g0_) * vec3<f32>(1.0, 0.0, 0.0))));
}

fn ideal_point_scalar_geometric_product(self_974: IdealPoint, other_800: Scalar) -> IdealPoint {
    var self_975: IdealPoint;
    var other_801: Scalar;

    self_975 = self_974;
    other_801 = other_800;
    let _e4: IdealPoint = self_975;
    let _e6: Scalar = other_801;
    return IdealPoint((_e4.g0_ * vec2<f32>(_e6.g0_)));
}

fn ideal_point_scalar_outer_product(self_976: IdealPoint, other_802: Scalar) -> IdealPoint {
    var self_977: IdealPoint;
    var other_803: Scalar;

    self_977 = self_976;
    other_803 = other_802;
    let _e4: IdealPoint = self_977;
    let _e6: Scalar = other_803;
    return IdealPoint((_e4.g0_ * vec2<f32>(_e6.g0_)));
}

fn ideal_point_scalar_inner_product(self_978: IdealPoint, other_804: Scalar) -> IdealPoint {
    var self_979: IdealPoint;
    var other_805: Scalar;

    self_979 = self_978;
    other_805 = other_804;
    let _e4: IdealPoint = self_979;
    let _e6: Scalar = other_805;
    return IdealPoint((_e4.g0_ * vec2<f32>(_e6.g0_)));
}

fn ideal_point_scalar_right_contraction(self_980: IdealPoint, other_806: Scalar) -> IdealPoint {
    var self_981: IdealPoint;
    var other_807: Scalar;

    self_981 = self_980;
    other_807 = other_806;
    let _e4: IdealPoint = self_981;
    let _e6: Scalar = other_807;
    return IdealPoint((_e4.g0_ * vec2<f32>(_e6.g0_)));
}

fn ideal_point_anti_scalar_regressive_product(self_982: IdealPoint, other_808: AntiScalar) -> IdealPoint {
    var self_983: IdealPoint;
    var other_809: AntiScalar;

    self_983 = self_982;
    other_809 = other_808;
    let _e4: IdealPoint = self_983;
    let _e6: AntiScalar = other_809;
    return IdealPoint((_e4.g0_ * vec2<f32>(_e6.g0_)));
}

fn ideal_point_anti_scalar_geometric_anti_product(self_984: IdealPoint, other_810: AntiScalar) -> IdealPoint {
    var self_985: IdealPoint;
    var other_811: AntiScalar;

    self_985 = self_984;
    other_811 = other_810;
    let _e4: IdealPoint = self_985;
    let _e6: AntiScalar = other_811;
    return IdealPoint((_e4.g0_ * vec2<f32>(_e6.g0_)));
}

fn ideal_point_anti_scalar_inner_anti_product(self_986: IdealPoint, other_812: AntiScalar) -> IdealPoint {
    var self_987: IdealPoint;
    var other_813: AntiScalar;

    self_987 = self_986;
    other_813 = other_812;
    let _e4: IdealPoint = self_987;
    let _e6: AntiScalar = other_813;
    return IdealPoint((_e4.g0_ * vec2<f32>(_e6.g0_)));
}

fn ideal_point_anti_scalar_right_anti_contraction(self_988: IdealPoint, other_814: AntiScalar) -> IdealPoint {
    var self_989: IdealPoint;
    var other_815: AntiScalar;

    self_989 = self_988;
    other_815 = other_814;
    let _e4: IdealPoint = self_989;
    let _e6: AntiScalar = other_815;
    return IdealPoint((_e4.g0_ * vec2<f32>(_e6.g0_)));
}

fn ideal_point_multi_vector_add(self_990: IdealPoint, other_816: MultiVector) -> MultiVector {
    var self_991: IdealPoint;
    var other_817: MultiVector;

    self_991 = self_990;
    other_817 = other_816;
    let _e4: MultiVector = other_817;
    let _e6: IdealPoint = self_991;
    let _e9: IdealPoint = self_991;
    let _e12: IdealPoint = self_991;
    let _e15: IdealPoint = self_991;
    let _e25: MultiVector = other_817;
    return MultiVector(_e4.g0_, ((vec4<f32>(_e6.g0_.x, _e9.g0_.x, _e12.g0_.x, _e15.g0_.y) * vec4<f32>(0.0, 0.0, 1.0, 1.0)) + _e25.g1_));
}

fn ideal_point_multi_vector_sub(self_992: IdealPoint, other_818: MultiVector) -> MultiVector {
    var self_993: IdealPoint;
    var other_819: MultiVector;

    self_993 = self_992;
    other_819 = other_818;
    let _e6: MultiVector = other_819;
    let _e9: IdealPoint = self_993;
    let _e12: IdealPoint = self_993;
    let _e15: IdealPoint = self_993;
    let _e18: IdealPoint = self_993;
    let _e28: MultiVector = other_819;
    return MultiVector((vec4<f32>(0.0) - _e6.g0_), ((vec4<f32>(_e9.g0_.x, _e12.g0_.x, _e15.g0_.x, _e18.g0_.y) * vec4<f32>(0.0, 0.0, 1.0, 1.0)) - _e28.g1_));
}

fn ideal_point_multi_vector_geometric_anti_product(self_994: IdealPoint, other_820: MultiVector) -> MultiVector {
    var self_995: IdealPoint;
    var other_821: MultiVector;

    self_995 = self_994;
    other_821 = other_820;
    let _e4: IdealPoint = self_995;
    let _e8: MultiVector = other_821;
    let _e12: IdealPoint = self_995;
    let _e16: MultiVector = other_821;
    let _e29: IdealPoint = self_995;
    let _e33: MultiVector = other_821;
    let _e45: IdealPoint = self_995;
    let _e49: MultiVector = other_821;
    return MultiVector(((vec4<f32>(_e4.g0_.x) * _e8.g0_.wzyx) + ((vec4<f32>(_e12.g0_.y) * _e16.g0_.zwxy) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))), (((vec4<f32>(_e29.g0_.x) * _e33.g1_.wzyx) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0))) + (vec4<f32>(_e45.g0_.y) * _e49.g1_.zwxy)));
}

fn ideal_point_multi_vector_anti_scalar_product(self_996: IdealPoint, other_822: MultiVector) -> AntiScalar {
    var self_997: IdealPoint;
    var other_823: MultiVector;

    self_997 = self_996;
    other_823 = other_822;
    let _e4: IdealPoint = self_997;
    let _e7: MultiVector = other_823;
    let _e11: IdealPoint = self_997;
    let _e14: MultiVector = other_823;
    return AntiScalar(((_e4.g0_.x * _e7.g1_.z) + (_e11.g0_.y * _e14.g1_.w)));
}

fn ideal_point_rotor_add(self_998: IdealPoint, other_824: Rotor) -> Motor {
    var self_999: IdealPoint;
    var other_825: Rotor;

    self_999 = self_998;
    other_825 = other_824;
    let _e4: IdealPoint = self_999;
    let _e7: IdealPoint = self_999;
    let _e10: IdealPoint = self_999;
    let _e13: IdealPoint = self_999;
    let _e23: Rotor = other_825;
    let _e26: Rotor = other_825;
    let _e29: Rotor = other_825;
    let _e32: Rotor = other_825;
    return Motor(((vec4<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.x, _e13.g0_.y) * vec4<f32>(0.0, 0.0, 1.0, 1.0)) + (vec4<f32>(_e23.g0_.x, _e26.g0_.y, _e29.g0_.x, _e32.g0_.x) * vec4<f32>(1.0, 1.0, 0.0, 0.0))));
}

fn ideal_point_rotor_sub(self_1000: IdealPoint, other_826: Rotor) -> Motor {
    var self_1001: IdealPoint;
    var other_827: Rotor;

    self_1001 = self_1000;
    other_827 = other_826;
    let _e4: IdealPoint = self_1001;
    let _e7: IdealPoint = self_1001;
    let _e10: IdealPoint = self_1001;
    let _e13: IdealPoint = self_1001;
    let _e23: Rotor = other_827;
    let _e26: Rotor = other_827;
    let _e29: Rotor = other_827;
    let _e32: Rotor = other_827;
    return Motor(((vec4<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.x, _e13.g0_.y) * vec4<f32>(0.0, 0.0, 1.0, 1.0)) - (vec4<f32>(_e23.g0_.x, _e26.g0_.y, _e29.g0_.x, _e32.g0_.x) * vec4<f32>(1.0, 1.0, 0.0, 0.0))));
}

fn ideal_point_rotor_geometric_product(self_1002: IdealPoint, other_828: Rotor) -> IdealPoint {
    var self_1003: IdealPoint;
    var other_829: Rotor;

    self_1003 = self_1002;
    other_829 = other_828;
    let _e4: IdealPoint = self_1003;
    let _e8: Rotor = other_829;
    let _e16: IdealPoint = self_1003;
    let _e20: Rotor = other_829;
    return IdealPoint((((vec2<f32>(_e4.g0_.x) * _e8.g0_) * vec2<f32>(1.0, -(1.0))) + (vec2<f32>(_e16.g0_.y) * _e20.g0_.yx)));
}

fn ideal_point_rotor_outer_product(self_1004: IdealPoint, other_830: Rotor) -> IdealPoint {
    var self_1005: IdealPoint;
    var other_831: Rotor;

    self_1005 = self_1004;
    other_831 = other_830;
    let _e4: IdealPoint = self_1005;
    let _e6: Rotor = other_831;
    return IdealPoint((_e4.g0_ * vec2<f32>(_e6.g0_.x)));
}

fn ideal_point_rotor_inner_product(self_1006: IdealPoint, other_832: Rotor) -> IdealPoint {
    var self_1007: IdealPoint;
    var other_833: Rotor;

    self_1007 = self_1006;
    other_833 = other_832;
    let _e4: IdealPoint = self_1007;
    let _e6: Rotor = other_833;
    return IdealPoint((_e4.g0_ * vec2<f32>(_e6.g0_.x)));
}

fn ideal_point_rotor_right_contraction(self_1008: IdealPoint, other_834: Rotor) -> IdealPoint {
    var self_1009: IdealPoint;
    var other_835: Rotor;

    self_1009 = self_1008;
    other_835 = other_834;
    let _e4: IdealPoint = self_1009;
    let _e6: Rotor = other_835;
    return IdealPoint((_e4.g0_ * vec2<f32>(_e6.g0_.x)));
}

fn ideal_point_point_add(self_1010: IdealPoint, other_836: Point) -> Point {
    var self_1011: IdealPoint;
    var other_837: Point;

    self_1011 = self_1010;
    other_837 = other_836;
    let _e4: IdealPoint = self_1011;
    let _e7: IdealPoint = self_1011;
    let _e10: IdealPoint = self_1011;
    let _e19: Point = other_837;
    return Point(((vec3<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.y) * vec3<f32>(0.0, 1.0, 1.0)) + _e19.g0_));
}

fn ideal_point_point_sub(self_1012: IdealPoint, other_838: Point) -> Point {
    var self_1013: IdealPoint;
    var other_839: Point;

    self_1013 = self_1012;
    other_839 = other_838;
    let _e4: IdealPoint = self_1013;
    let _e7: IdealPoint = self_1013;
    let _e10: IdealPoint = self_1013;
    let _e19: Point = other_839;
    return Point(((vec3<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.y) * vec3<f32>(0.0, 1.0, 1.0)) - _e19.g0_));
}

fn ideal_point_point_geometric_product(self_1014: IdealPoint, other_840: Point) -> IdealPoint {
    var self_1015: IdealPoint;
    var other_841: Point;

    self_1015 = self_1014;
    other_841 = other_840;
    let _e4: IdealPoint = self_1015;
    let _e7: Point = other_841;
    return IdealPoint(((_e4.g0_.yx * vec2<f32>(_e7.g0_.x)) * vec2<f32>(1.0, -(1.0))));
}

fn ideal_point_point_regressive_product(self_1016: IdealPoint, other_842: Point) -> Plane {
    var self_1017: IdealPoint;
    var other_843: Point;

    self_1017 = self_1016;
    other_843 = other_842;
    let _e4: IdealPoint = self_1017;
    let _e8: Point = other_843;
    let _e18: IdealPoint = self_1017;
    let _e22: Point = other_843;
    return Plane((((vec3<f32>(_e4.g0_.y) * _e8.g0_.yxy) * vec3<f32>(1.0, -(1.0), 0.0)) + ((vec3<f32>(_e18.g0_.x) * _e22.g0_.zxx) * vec3<f32>(-(1.0), 0.0, 1.0))));
}

fn ideal_point_point_geometric_anti_product(self_1018: IdealPoint, other_844: Point) -> MotorDual {
    var self_1019: IdealPoint;
    var other_845: Point;

    self_1019 = self_1018;
    other_845 = other_844;
    let _e4: IdealPoint = self_1019;
    let _e8: Point = other_845;
    let _e11: Point = other_845;
    let _e14: Point = other_845;
    let _e17: Point = other_845;
    let _e29: IdealPoint = self_1019;
    let _e33: Point = other_845;
    let _e36: Point = other_845;
    let _e39: Point = other_845;
    let _e42: Point = other_845;
    return MotorDual((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.z, _e11.g0_.y, _e14.g0_.x, _e17.g0_.z)) * vec4<f32>(1.0, 1.0, -(1.0), 0.0)) + ((vec4<f32>(_e29.g0_.x) * vec4<f32>(_e33.g0_.y, _e36.g0_.z, _e39.g0_.x, _e42.g0_.x)) * vec4<f32>(1.0, -(1.0), 0.0, 1.0))));
}

fn ideal_point_point_inner_anti_product(self_1020: IdealPoint, other_846: Point) -> AntiScalar {
    var self_1021: IdealPoint;
    var other_847: Point;

    self_1021 = self_1020;
    other_847 = other_846;
    let _e4: IdealPoint = self_1021;
    let _e7: Point = other_847;
    let _e11: IdealPoint = self_1021;
    let _e14: Point = other_847;
    return AntiScalar(((_e4.g0_.x * _e7.g0_.y) + (_e11.g0_.y * _e14.g0_.z)));
}

fn ideal_point_point_left_anti_contraction(self_1022: IdealPoint, other_848: Point) -> AntiScalar {
    var self_1023: IdealPoint;
    var other_849: Point;

    self_1023 = self_1022;
    other_849 = other_848;
    let _e4: IdealPoint = self_1023;
    let _e7: Point = other_849;
    let _e11: IdealPoint = self_1023;
    let _e14: Point = other_849;
    return AntiScalar(((_e4.g0_.x * _e7.g0_.y) + (_e11.g0_.y * _e14.g0_.z)));
}

fn ideal_point_point_right_anti_contraction(self_1024: IdealPoint, other_850: Point) -> AntiScalar {
    var self_1025: IdealPoint;
    var other_851: Point;

    self_1025 = self_1024;
    other_851 = other_850;
    let _e4: IdealPoint = self_1025;
    let _e7: Point = other_851;
    let _e11: IdealPoint = self_1025;
    let _e14: Point = other_851;
    return AntiScalar(((_e4.g0_.x * _e7.g0_.y) + (_e11.g0_.y * _e14.g0_.z)));
}

fn ideal_point_point_anti_scalar_product(self_1026: IdealPoint, other_852: Point) -> AntiScalar {
    var self_1027: IdealPoint;
    var other_853: Point;

    self_1027 = self_1026;
    other_853 = other_852;
    let _e4: IdealPoint = self_1027;
    let _e7: Point = other_853;
    let _e11: IdealPoint = self_1027;
    let _e14: Point = other_853;
    return AntiScalar(((_e4.g0_.x * _e7.g0_.y) + (_e11.g0_.y * _e14.g0_.z)));
}

fn ideal_point_ideal_point_add(self_1028: IdealPoint, other_854: IdealPoint) -> IdealPoint {
    var self_1029: IdealPoint;
    var other_855: IdealPoint;

    self_1029 = self_1028;
    other_855 = other_854;
    let _e4: IdealPoint = self_1029;
    let _e6: IdealPoint = other_855;
    return IdealPoint((_e4.g0_ + _e6.g0_));
}

fn ideal_point_ideal_point_sub(self_1030: IdealPoint, other_856: IdealPoint) -> IdealPoint {
    var self_1031: IdealPoint;
    var other_857: IdealPoint;

    self_1031 = self_1030;
    other_857 = other_856;
    let _e4: IdealPoint = self_1031;
    let _e6: IdealPoint = other_857;
    return IdealPoint((_e4.g0_ - _e6.g0_));
}

fn ideal_point_ideal_point_mul(self_1032: IdealPoint, other_858: IdealPoint) -> IdealPoint {
    var self_1033: IdealPoint;
    var other_859: IdealPoint;

    self_1033 = self_1032;
    other_859 = other_858;
    let _e4: IdealPoint = self_1033;
    let _e6: IdealPoint = other_859;
    return IdealPoint((_e4.g0_ * _e6.g0_));
}

fn ideal_point_ideal_point_div(self_1034: IdealPoint, other_860: IdealPoint) -> IdealPoint {
    var self_1035: IdealPoint;
    var other_861: IdealPoint;

    self_1035 = self_1034;
    other_861 = other_860;
    let _e4: IdealPoint = self_1035;
    let _e7: IdealPoint = self_1035;
    let _e15: IdealPoint = other_861;
    let _e18: IdealPoint = other_861;
    return IdealPoint((((vec2<f32>(_e4.g0_.x, _e7.g0_.y) * vec2<f32>(1.0, 1.0)) / vec2<f32>(_e15.g0_.x, _e18.g0_.y)) * vec2<f32>(1.0, 1.0)));
}

fn ideal_point_ideal_point_inner_anti_product(self_1036: IdealPoint, other_862: IdealPoint) -> AntiScalar {
    var self_1037: IdealPoint;
    var other_863: IdealPoint;

    self_1037 = self_1036;
    other_863 = other_862;
    let _e4: IdealPoint = self_1037;
    let _e7: IdealPoint = other_863;
    let _e11: IdealPoint = self_1037;
    let _e14: IdealPoint = other_863;
    return AntiScalar(((_e4.g0_.x * _e7.g0_.x) + (_e11.g0_.y * _e14.g0_.y)));
}

fn ideal_point_ideal_point_left_anti_contraction(self_1038: IdealPoint, other_864: IdealPoint) -> AntiScalar {
    var self_1039: IdealPoint;
    var other_865: IdealPoint;

    self_1039 = self_1038;
    other_865 = other_864;
    let _e4: IdealPoint = self_1039;
    let _e7: IdealPoint = other_865;
    let _e11: IdealPoint = self_1039;
    let _e14: IdealPoint = other_865;
    return AntiScalar(((_e4.g0_.x * _e7.g0_.x) + (_e11.g0_.y * _e14.g0_.y)));
}

fn ideal_point_ideal_point_right_anti_contraction(self_1040: IdealPoint, other_866: IdealPoint) -> AntiScalar {
    var self_1041: IdealPoint;
    var other_867: IdealPoint;

    self_1041 = self_1040;
    other_867 = other_866;
    let _e4: IdealPoint = self_1041;
    let _e7: IdealPoint = other_867;
    let _e11: IdealPoint = self_1041;
    let _e14: IdealPoint = other_867;
    return AntiScalar(((_e4.g0_.x * _e7.g0_.x) + (_e11.g0_.y * _e14.g0_.y)));
}

fn ideal_point_ideal_point_anti_scalar_product(self_1042: IdealPoint, other_868: IdealPoint) -> AntiScalar {
    var self_1043: IdealPoint;
    var other_869: IdealPoint;

    self_1043 = self_1042;
    other_869 = other_868;
    let _e4: IdealPoint = self_1043;
    let _e7: IdealPoint = other_869;
    let _e11: IdealPoint = self_1043;
    let _e14: IdealPoint = other_869;
    return AntiScalar(((_e4.g0_.x * _e7.g0_.x) + (_e11.g0_.y * _e14.g0_.y)));
}

fn ideal_point_plane_regressive_product(self_1044: IdealPoint, other_870: Plane) -> Scalar {
    var self_1045: IdealPoint;
    var other_871: Plane;

    self_1045 = self_1044;
    other_871 = other_870;
    let _e4: IdealPoint = self_1045;
    let _e7: Plane = other_871;
    let _e11: IdealPoint = self_1045;
    let _e14: Plane = other_871;
    return Scalar(((_e4.g0_.x * _e7.g0_.y) + (_e11.g0_.y * _e14.g0_.z)));
}

fn ideal_point_plane_outer_product(self_1046: IdealPoint, other_872: Plane) -> AntiScalar {
    var self_1047: IdealPoint;
    var other_873: Plane;

    self_1047 = self_1046;
    other_873 = other_872;
    let _e4: IdealPoint = self_1047;
    let _e7: Plane = other_873;
    let _e11: IdealPoint = self_1047;
    let _e14: Plane = other_873;
    return AntiScalar(((_e4.g0_.x * _e7.g0_.y) + (_e11.g0_.y * _e14.g0_.z)));
}

fn ideal_point_plane_geometric_anti_product(self_1048: IdealPoint, other_874: Plane) -> Motor {
    var self_1049: IdealPoint;
    var other_875: Plane;

    self_1049 = self_1048;
    other_875 = other_874;
    let _e4: IdealPoint = self_1049;
    let _e8: Plane = other_875;
    let _e11: Plane = other_875;
    let _e14: Plane = other_875;
    let _e17: Plane = other_875;
    let _e29: IdealPoint = self_1049;
    let _e33: Plane = other_875;
    let _e36: Plane = other_875;
    let _e39: Plane = other_875;
    let _e42: Plane = other_875;
    return Motor((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.z, _e11.g0_.y, _e14.g0_.x, _e17.g0_.z)) * vec4<f32>(1.0, -(1.0), 1.0, 0.0)) + ((vec4<f32>(_e29.g0_.x) * vec4<f32>(_e33.g0_.y, _e36.g0_.z, _e39.g0_.x, _e42.g0_.x)) * vec4<f32>(1.0, 1.0, 0.0, -(1.0)))));
}

fn ideal_point_plane_inner_anti_product(self_1050: IdealPoint, other_876: Plane) -> Point {
    var self_1051: IdealPoint;
    var other_877: Plane;

    self_1051 = self_1050;
    other_877 = other_876;
    let _e4: IdealPoint = self_1051;
    let _e8: Plane = other_877;
    let _e18: IdealPoint = self_1051;
    let _e22: Plane = other_877;
    return Point((((vec3<f32>(_e4.g0_.y) * _e8.g0_.yxy) * vec3<f32>(-(1.0), 1.0, 0.0)) + ((vec3<f32>(_e18.g0_.x) * _e22.g0_.zxx) * vec3<f32>(1.0, 0.0, -(1.0)))));
}

fn ideal_point_plane_left_anti_contraction(self_1052: IdealPoint, other_878: Plane) -> Point {
    var self_1053: IdealPoint;
    var other_879: Plane;

    self_1053 = self_1052;
    other_879 = other_878;
    let _e4: IdealPoint = self_1053;
    let _e8: Plane = other_879;
    let _e18: IdealPoint = self_1053;
    let _e22: Plane = other_879;
    return Point((((vec3<f32>(_e4.g0_.y) * _e8.g0_.yxy) * vec3<f32>(-(1.0), 1.0, 0.0)) + ((vec3<f32>(_e18.g0_.x) * _e22.g0_.zxx) * vec3<f32>(1.0, 0.0, -(1.0)))));
}

fn ideal_point_translator_add(self_1054: IdealPoint, other_880: Translator) -> Translator {
    var self_1055: IdealPoint;
    var other_881: Translator;

    self_1055 = self_1054;
    other_881 = other_880;
    let _e4: IdealPoint = self_1055;
    let _e7: IdealPoint = self_1055;
    let _e10: IdealPoint = self_1055;
    let _e19: Translator = other_881;
    return Translator(((vec3<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.y) * vec3<f32>(0.0, 1.0, 1.0)) + _e19.g0_));
}

fn ideal_point_translator_sub(self_1056: IdealPoint, other_882: Translator) -> Translator {
    var self_1057: IdealPoint;
    var other_883: Translator;

    self_1057 = self_1056;
    other_883 = other_882;
    let _e4: IdealPoint = self_1057;
    let _e7: IdealPoint = self_1057;
    let _e10: IdealPoint = self_1057;
    let _e19: Translator = other_883;
    return Translator(((vec3<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.y) * vec3<f32>(0.0, 1.0, 1.0)) - _e19.g0_));
}

fn ideal_point_translator_geometric_product(self_1058: IdealPoint, other_884: Translator) -> IdealPoint {
    var self_1059: IdealPoint;
    var other_885: Translator;

    self_1059 = self_1058;
    other_885 = other_884;
    let _e4: IdealPoint = self_1059;
    let _e6: Translator = other_885;
    return IdealPoint((_e4.g0_ * vec2<f32>(_e6.g0_.x)));
}

fn ideal_point_translator_outer_product(self_1060: IdealPoint, other_886: Translator) -> IdealPoint {
    var self_1061: IdealPoint;
    var other_887: Translator;

    self_1061 = self_1060;
    other_887 = other_886;
    let _e4: IdealPoint = self_1061;
    let _e6: Translator = other_887;
    return IdealPoint((_e4.g0_ * vec2<f32>(_e6.g0_.x)));
}

fn ideal_point_translator_inner_product(self_1062: IdealPoint, other_888: Translator) -> IdealPoint {
    var self_1063: IdealPoint;
    var other_889: Translator;

    self_1063 = self_1062;
    other_889 = other_888;
    let _e4: IdealPoint = self_1063;
    let _e6: Translator = other_889;
    return IdealPoint((_e4.g0_ * vec2<f32>(_e6.g0_.x)));
}

fn ideal_point_translator_geometric_anti_product(self_1064: IdealPoint, other_890: Translator) -> MotorDual {
    var self_1065: IdealPoint;
    var other_891: Translator;

    self_1065 = self_1064;
    other_891 = other_890;
    let _e4: IdealPoint = self_1065;
    let _e8: Translator = other_891;
    let _e11: Translator = other_891;
    let _e14: Translator = other_891;
    let _e17: Translator = other_891;
    let _e28: IdealPoint = self_1065;
    let _e32: Translator = other_891;
    let _e35: Translator = other_891;
    let _e38: Translator = other_891;
    let _e41: Translator = other_891;
    return MotorDual((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.z, _e11.g0_.y, _e14.g0_.z, _e17.g0_.x)) * vec4<f32>(1.0, 1.0, 0.0, 1.0)) + ((vec4<f32>(_e28.g0_.x) * vec4<f32>(_e32.g0_.y, _e35.g0_.z, _e38.g0_.x, _e41.g0_.x)) * vec4<f32>(1.0, -(1.0), 1.0, 0.0))));
}

fn ideal_point_translator_right_contraction(self_1066: IdealPoint, other_892: Translator) -> IdealPoint {
    var self_1067: IdealPoint;
    var other_893: Translator;

    self_1067 = self_1066;
    other_893 = other_892;
    let _e4: IdealPoint = self_1067;
    let _e6: Translator = other_893;
    return IdealPoint((_e4.g0_ * vec2<f32>(_e6.g0_.x)));
}

fn ideal_point_translator_right_anti_contraction(self_1068: IdealPoint, other_894: Translator) -> AntiScalar {
    var self_1069: IdealPoint;
    var other_895: Translator;

    self_1069 = self_1068;
    other_895 = other_894;
    let _e4: IdealPoint = self_1069;
    let _e7: Translator = other_895;
    let _e11: IdealPoint = self_1069;
    let _e14: Translator = other_895;
    return AntiScalar(((_e4.g0_.x * _e7.g0_.y) + (_e11.g0_.y * _e14.g0_.z)));
}

fn ideal_point_translator_anti_scalar_product(self_1070: IdealPoint, other_896: Translator) -> AntiScalar {
    var self_1071: IdealPoint;
    var other_897: Translator;

    self_1071 = self_1070;
    other_897 = other_896;
    let _e4: IdealPoint = self_1071;
    let _e7: Translator = other_897;
    let _e11: IdealPoint = self_1071;
    let _e14: Translator = other_897;
    return AntiScalar(((_e4.g0_.x * _e7.g0_.y) + (_e11.g0_.y * _e14.g0_.z)));
}

fn ideal_point_motor_add(self_1072: IdealPoint, other_898: Motor) -> Motor {
    var self_1073: IdealPoint;
    var other_899: Motor;

    self_1073 = self_1072;
    other_899 = other_898;
    let _e4: IdealPoint = self_1073;
    let _e7: IdealPoint = self_1073;
    let _e10: IdealPoint = self_1073;
    let _e13: IdealPoint = self_1073;
    let _e23: Motor = other_899;
    return Motor(((vec4<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.x, _e13.g0_.y) * vec4<f32>(0.0, 0.0, 1.0, 1.0)) + _e23.g0_));
}

fn ideal_point_motor_sub(self_1074: IdealPoint, other_900: Motor) -> Motor {
    var self_1075: IdealPoint;
    var other_901: Motor;

    self_1075 = self_1074;
    other_901 = other_900;
    let _e4: IdealPoint = self_1075;
    let _e7: IdealPoint = self_1075;
    let _e10: IdealPoint = self_1075;
    let _e13: IdealPoint = self_1075;
    let _e23: Motor = other_901;
    return Motor(((vec4<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.x, _e13.g0_.y) * vec4<f32>(0.0, 0.0, 1.0, 1.0)) - _e23.g0_));
}

fn ideal_point_motor_geometric_product(self_1076: IdealPoint, other_902: Motor) -> IdealPoint {
    var self_1077: IdealPoint;
    var other_903: Motor;

    self_1077 = self_1076;
    other_903 = other_902;
    let _e4: IdealPoint = self_1077;
    let _e8: Motor = other_903;
    let _e11: Motor = other_903;
    let _e21: IdealPoint = self_1077;
    let _e25: Motor = other_903;
    let _e28: Motor = other_903;
    return IdealPoint((((vec2<f32>(_e4.g0_.x) * vec2<f32>(_e8.g0_.x, _e11.g0_.y)) * vec2<f32>(1.0, -(1.0))) + (vec2<f32>(_e21.g0_.y) * vec2<f32>(_e25.g0_.y, _e28.g0_.x))));
}

fn ideal_point_motor_regressive_product(self_1078: IdealPoint, other_904: Motor) -> Plane {
    var self_1079: IdealPoint;
    var other_905: Motor;

    self_1079 = self_1078;
    other_905 = other_904;
    let _e4: IdealPoint = self_1079;
    let _e8: Motor = other_905;
    let _e11: Motor = other_905;
    let _e14: Motor = other_905;
    let _e25: IdealPoint = self_1079;
    let _e29: Motor = other_905;
    let _e32: Motor = other_905;
    let _e35: Motor = other_905;
    return Plane((((vec3<f32>(_e4.g0_.y) * vec3<f32>(_e8.g0_.z, _e11.g0_.y, _e14.g0_.z)) * vec3<f32>(1.0, -(1.0), 0.0)) + ((vec3<f32>(_e25.g0_.x) * vec3<f32>(_e29.g0_.w, _e32.g0_.x, _e35.g0_.y)) * vec3<f32>(-(1.0), 0.0, 1.0))));
}

fn ideal_point_motor_outer_product(self_1080: IdealPoint, other_906: Motor) -> IdealPoint {
    var self_1081: IdealPoint;
    var other_907: Motor;

    self_1081 = self_1080;
    other_907 = other_906;
    let _e4: IdealPoint = self_1081;
    let _e6: Motor = other_907;
    return IdealPoint((_e4.g0_ * vec2<f32>(_e6.g0_.x)));
}

fn ideal_point_motor_inner_product(self_1082: IdealPoint, other_908: Motor) -> IdealPoint {
    var self_1083: IdealPoint;
    var other_909: Motor;

    self_1083 = self_1082;
    other_909 = other_908;
    let _e4: IdealPoint = self_1083;
    let _e6: Motor = other_909;
    return IdealPoint((_e4.g0_ * vec2<f32>(_e6.g0_.x)));
}

fn ideal_point_motor_geometric_anti_product(self_1084: IdealPoint, other_910: Motor) -> MotorDual {
    var self_1085: IdealPoint;
    var other_911: Motor;

    self_1085 = self_1084;
    other_911 = other_910;
    let _e4: IdealPoint = self_1085;
    let _e8: Motor = other_911;
    let _e19: IdealPoint = self_1085;
    let _e23: Motor = other_911;
    return MotorDual((((vec4<f32>(_e4.g0_.x) * _e8.g0_.zwxy) * vec4<f32>(1.0, -(1.0), 1.0, 1.0)) + ((vec4<f32>(_e19.g0_.y) * _e23.g0_.wzyx) * vec4<f32>(1.0, 1.0, -(1.0), 1.0))));
}

fn ideal_point_motor_right_contraction(self_1086: IdealPoint, other_912: Motor) -> IdealPoint {
    var self_1087: IdealPoint;
    var other_913: Motor;

    self_1087 = self_1086;
    other_913 = other_912;
    let _e4: IdealPoint = self_1087;
    let _e6: Motor = other_913;
    return IdealPoint((_e4.g0_ * vec2<f32>(_e6.g0_.x)));
}

fn ideal_point_motor_right_anti_contraction(self_1088: IdealPoint, other_914: Motor) -> AntiScalar {
    var self_1089: IdealPoint;
    var other_915: Motor;

    self_1089 = self_1088;
    other_915 = other_914;
    let _e4: IdealPoint = self_1089;
    let _e7: Motor = other_915;
    let _e11: IdealPoint = self_1089;
    let _e14: Motor = other_915;
    return AntiScalar(((_e4.g0_.x * _e7.g0_.z) + (_e11.g0_.y * _e14.g0_.w)));
}

fn ideal_point_motor_anti_scalar_product(self_1090: IdealPoint, other_916: Motor) -> AntiScalar {
    var self_1091: IdealPoint;
    var other_917: Motor;

    self_1091 = self_1090;
    other_917 = other_916;
    let _e4: IdealPoint = self_1091;
    let _e7: Motor = other_917;
    let _e11: IdealPoint = self_1091;
    let _e14: Motor = other_917;
    return AntiScalar(((_e4.g0_.x * _e7.g0_.z) + (_e11.g0_.y * _e14.g0_.w)));
}

fn ideal_point_motor_dual_regressive_product(self_1092: IdealPoint, other_918: MotorDual) -> Translator {
    var self_1093: IdealPoint;
    var other_919: MotorDual;

    self_1093 = self_1092;
    other_919 = other_918;
    let _e4: IdealPoint = self_1093;
    let _e8: MotorDual = other_919;
    let _e11: MotorDual = other_919;
    let _e14: MotorDual = other_919;
    let _e24: IdealPoint = self_1093;
    let _e28: MotorDual = other_919;
    let _e31: MotorDual = other_919;
    let _e34: MotorDual = other_919;
    return Translator((((vec3<f32>(_e4.g0_.y) * vec3<f32>(_e8.g0_.w, _e11.g0_.w, _e14.g0_.x)) * vec3<f32>(1.0, 0.0, 1.0)) + ((vec3<f32>(_e24.g0_.x) * vec3<f32>(_e28.g0_.z, _e31.g0_.x, _e34.g0_.x)) * vec3<f32>(1.0, 1.0, 0.0))));
}

fn ideal_point_motor_dual_outer_product(self_1094: IdealPoint, other_920: MotorDual) -> AntiScalar {
    var self_1095: IdealPoint;
    var other_921: MotorDual;

    self_1095 = self_1094;
    other_921 = other_920;
    let _e4: IdealPoint = self_1095;
    let _e7: MotorDual = other_921;
    let _e11: IdealPoint = self_1095;
    let _e14: MotorDual = other_921;
    return AntiScalar(((_e4.g0_.x * _e7.g0_.z) + (_e11.g0_.y * _e14.g0_.w)));
}

fn ideal_point_motor_dual_geometric_anti_product(self_1096: IdealPoint, other_922: MotorDual) -> Motor {
    var self_1097: IdealPoint;
    var other_923: MotorDual;

    self_1097 = self_1096;
    other_923 = other_922;
    let _e4: IdealPoint = self_1097;
    let _e8: MotorDual = other_923;
    let _e19: IdealPoint = self_1097;
    let _e23: MotorDual = other_923;
    return Motor((((vec4<f32>(_e4.g0_.x) * _e8.g0_.zwxy) * vec4<f32>(1.0, 1.0, 1.0, -(1.0))) + ((vec4<f32>(_e19.g0_.y) * _e23.g0_.wzyx) * vec4<f32>(1.0, -(1.0), 1.0, 1.0))));
}

fn ideal_point_motor_dual_inner_anti_product(self_1098: IdealPoint, other_924: MotorDual) -> Point {
    var self_1099: IdealPoint;
    var other_925: MotorDual;

    self_1099 = self_1098;
    other_925 = other_924;
    let _e4: IdealPoint = self_1099;
    let _e8: MotorDual = other_925;
    let _e11: MotorDual = other_925;
    let _e14: MotorDual = other_925;
    let _e25: IdealPoint = self_1099;
    let _e29: MotorDual = other_925;
    let _e32: MotorDual = other_925;
    let _e35: MotorDual = other_925;
    return Point((((vec3<f32>(_e4.g0_.x) * vec3<f32>(_e8.g0_.w, _e11.g0_.x, _e14.g0_.y)) * vec3<f32>(1.0, 1.0, -(1.0))) + ((vec3<f32>(_e25.g0_.y) * vec3<f32>(_e29.g0_.z, _e32.g0_.y, _e35.g0_.x)) * vec3<f32>(-(1.0), 1.0, 1.0))));
}

fn ideal_point_motor_dual_left_anti_contraction(self_1100: IdealPoint, other_926: MotorDual) -> Point {
    var self_1101: IdealPoint;
    var other_927: MotorDual;

    self_1101 = self_1100;
    other_927 = other_926;
    let _e4: IdealPoint = self_1101;
    let _e8: MotorDual = other_927;
    let _e11: MotorDual = other_927;
    let _e14: MotorDual = other_927;
    let _e25: IdealPoint = self_1101;
    let _e29: MotorDual = other_927;
    let _e32: MotorDual = other_927;
    let _e35: MotorDual = other_927;
    return Point((((vec3<f32>(_e4.g0_.y) * vec3<f32>(_e8.g0_.z, _e11.g0_.y, _e14.g0_.z)) * vec3<f32>(-(1.0), 1.0, 0.0)) + ((vec3<f32>(_e25.g0_.x) * vec3<f32>(_e29.g0_.w, _e32.g0_.x, _e35.g0_.y)) * vec3<f32>(1.0, 0.0, -(1.0)))));
}

fn ideal_point_motor_dual_right_anti_contraction(self_1102: IdealPoint, other_928: MotorDual) -> IdealPoint {
    var self_1103: IdealPoint;
    var other_929: MotorDual;

    self_1103 = self_1102;
    other_929 = other_928;
    let _e4: IdealPoint = self_1103;
    let _e6: MotorDual = other_929;
    return IdealPoint((_e4.g0_ * vec2<f32>(_e6.g0_.x)));
}

fn ideal_point_scale(self_1104: IdealPoint, other_930: f32) -> IdealPoint {
    var self_1105: IdealPoint;
    var other_931: f32;

    self_1105 = self_1104;
    other_931 = other_930;
    let _e4: IdealPoint = self_1105;
    let _e5: f32 = other_931;
    let _e7: IdealPoint = ideal_point_scalar_geometric_product(_e4, Scalar(_e5));
    return _e7;
}

fn plane_zero() -> Plane {
    return Plane(vec3<f32>(0.0));
}

fn plane_one() -> Plane {
    return Plane(vec3<f32>(0.0));
}

fn plane_grade(self_1106: Plane) -> i32 {
    return 1;
}

fn plane_anti_grade(self_1107: Plane) -> i32 {
    return 2;
}

fn plane_neg(self_1108: Plane) -> Plane {
    var self_1109: Plane;

    self_1109 = self_1108;
    let _e2: Plane = self_1109;
    return Plane((_e2.g0_ * vec3<f32>(-(1.0))));
}

fn plane_automorphism(self_1110: Plane) -> Plane {
    var self_1111: Plane;

    self_1111 = self_1110;
    let _e2: Plane = self_1111;
    return Plane((_e2.g0_ * vec3<f32>(-(1.0))));
}

fn plane_reversal(self_1112: Plane) -> Plane {
    var self_1113: Plane;

    self_1113 = self_1112;
    let _e2: Plane = self_1113;
    return Plane(_e2.g0_);
}

fn plane_conjugation(self_1114: Plane) -> Plane {
    var self_1115: Plane;

    self_1115 = self_1114;
    let _e2: Plane = self_1115;
    return Plane((_e2.g0_ * vec3<f32>(-(1.0))));
}

fn plane_dual(self_1116: Plane) -> Point {
    var self_1117: Plane;

    self_1117 = self_1116;
    let _e2: Plane = self_1117;
    return Point(_e2.g0_);
}

fn plane_anti_reversal(self_1118: Plane) -> Plane {
    var self_1119: Plane;

    self_1119 = self_1118;
    let _e2: Plane = self_1119;
    return Plane((_e2.g0_ * vec3<f32>(-(1.0))));
}

fn plane_right_complement(self_1120: Plane) -> Point {
    var self_1121: Plane;

    self_1121 = self_1120;
    let _e2: Plane = self_1121;
    return Point(_e2.g0_);
}

fn plane_left_complement(self_1122: Plane) -> Point {
    var self_1123: Plane;

    self_1123 = self_1122;
    let _e2: Plane = self_1123;
    return Point(_e2.g0_);
}

fn plane_double_complement(self_1124: Plane) -> Plane {
    var self_1125: Plane;

    self_1125 = self_1124;
    let _e2: Plane = self_1125;
    return Plane(_e2.g0_);
}

fn plane_scalar_geometric_product(self_1126: Plane, other_932: Scalar) -> Plane {
    var self_1127: Plane;
    var other_933: Scalar;

    self_1127 = self_1126;
    other_933 = other_932;
    let _e4: Plane = self_1127;
    let _e6: Scalar = other_933;
    return Plane((_e4.g0_ * vec3<f32>(_e6.g0_)));
}

fn plane_scalar_outer_product(self_1128: Plane, other_934: Scalar) -> Plane {
    var self_1129: Plane;
    var other_935: Scalar;

    self_1129 = self_1128;
    other_935 = other_934;
    let _e4: Plane = self_1129;
    let _e6: Scalar = other_935;
    return Plane((_e4.g0_ * vec3<f32>(_e6.g0_)));
}

fn plane_scalar_inner_product(self_1130: Plane, other_936: Scalar) -> Plane {
    var self_1131: Plane;
    var other_937: Scalar;

    self_1131 = self_1130;
    other_937 = other_936;
    let _e4: Plane = self_1131;
    let _e6: Scalar = other_937;
    return Plane((_e4.g0_ * vec3<f32>(_e6.g0_)));
}

fn plane_scalar_right_contraction(self_1132: Plane, other_938: Scalar) -> Plane {
    var self_1133: Plane;
    var other_939: Scalar;

    self_1133 = self_1132;
    other_939 = other_938;
    let _e4: Plane = self_1133;
    let _e6: Scalar = other_939;
    return Plane((_e4.g0_ * vec3<f32>(_e6.g0_)));
}

fn plane_anti_scalar_add(self_1134: Plane, other_940: AntiScalar) -> MotorDual {
    var self_1135: Plane;
    var other_941: AntiScalar;

    self_1135 = self_1134;
    other_941 = other_940;
    let _e4: Plane = self_1135;
    let _e7: Plane = self_1135;
    let _e10: Plane = self_1135;
    let _e13: Plane = self_1135;
    let _e23: AntiScalar = other_941;
    return MotorDual(((vec4<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.y, _e13.g0_.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0)) + (vec4<f32>(_e23.g0_) * vec4<f32>(1.0, 0.0, 0.0, 0.0))));
}

fn plane_anti_scalar_sub(self_1136: Plane, other_942: AntiScalar) -> MotorDual {
    var self_1137: Plane;
    var other_943: AntiScalar;

    self_1137 = self_1136;
    other_943 = other_942;
    let _e4: Plane = self_1137;
    let _e7: Plane = self_1137;
    let _e10: Plane = self_1137;
    let _e13: Plane = self_1137;
    let _e23: AntiScalar = other_943;
    return MotorDual(((vec4<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.y, _e13.g0_.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0)) - (vec4<f32>(_e23.g0_) * vec4<f32>(1.0, 0.0, 0.0, 0.0))));
}

fn plane_anti_scalar_geometric_product(self_1138: Plane, other_944: AntiScalar) -> IdealPoint {
    var self_1139: Plane;
    var other_945: AntiScalar;

    self_1139 = self_1138;
    other_945 = other_944;
    let _e4: Plane = self_1139;
    let _e7: Plane = self_1139;
    let _e11: AntiScalar = other_945;
    return IdealPoint((vec2<f32>(_e4.g0_.y, _e7.g0_.z) * vec2<f32>(_e11.g0_)));
}

fn plane_anti_scalar_regressive_product(self_1140: Plane, other_946: AntiScalar) -> Plane {
    var self_1141: Plane;
    var other_947: AntiScalar;

    self_1141 = self_1140;
    other_947 = other_946;
    let _e4: Plane = self_1141;
    let _e6: AntiScalar = other_947;
    return Plane((_e4.g0_ * vec3<f32>(_e6.g0_)));
}

fn plane_anti_scalar_inner_product(self_1142: Plane, other_948: AntiScalar) -> IdealPoint {
    var self_1143: Plane;
    var other_949: AntiScalar;

    self_1143 = self_1142;
    other_949 = other_948;
    let _e4: Plane = self_1143;
    let _e7: Plane = self_1143;
    let _e11: AntiScalar = other_949;
    return IdealPoint((vec2<f32>(_e4.g0_.y, _e7.g0_.z) * vec2<f32>(_e11.g0_)));
}

fn plane_anti_scalar_geometric_anti_product(self_1144: Plane, other_950: AntiScalar) -> Plane {
    var self_1145: Plane;
    var other_951: AntiScalar;

    self_1145 = self_1144;
    other_951 = other_950;
    let _e4: Plane = self_1145;
    let _e6: AntiScalar = other_951;
    return Plane((_e4.g0_ * vec3<f32>(_e6.g0_)));
}

fn plane_anti_scalar_inner_anti_product(self_1146: Plane, other_952: AntiScalar) -> Plane {
    var self_1147: Plane;
    var other_953: AntiScalar;

    self_1147 = self_1146;
    other_953 = other_952;
    let _e4: Plane = self_1147;
    let _e6: AntiScalar = other_953;
    return Plane((_e4.g0_ * vec3<f32>(_e6.g0_)));
}

fn plane_anti_scalar_left_contraction(self_1148: Plane, other_954: AntiScalar) -> IdealPoint {
    var self_1149: Plane;
    var other_955: AntiScalar;

    self_1149 = self_1148;
    other_955 = other_954;
    let _e4: Plane = self_1149;
    let _e7: Plane = self_1149;
    let _e11: AntiScalar = other_955;
    return IdealPoint((vec2<f32>(_e4.g0_.y, _e7.g0_.z) * vec2<f32>(_e11.g0_)));
}

fn plane_anti_scalar_right_anti_contraction(self_1150: Plane, other_956: AntiScalar) -> Plane {
    var self_1151: Plane;
    var other_957: AntiScalar;

    self_1151 = self_1150;
    other_957 = other_956;
    let _e4: Plane = self_1151;
    let _e6: AntiScalar = other_957;
    return Plane((_e4.g0_ * vec3<f32>(_e6.g0_)));
}

fn plane_multi_vector_add(self_1152: Plane, other_958: MultiVector) -> MultiVector {
    var self_1153: Plane;
    var other_959: MultiVector;

    self_1153 = self_1152;
    other_959 = other_958;
    let _e4: Plane = self_1153;
    let _e7: Plane = self_1153;
    let _e10: Plane = self_1153;
    let _e13: Plane = self_1153;
    let _e23: MultiVector = other_959;
    let _e26: Plane = self_1153;
    let _e36: MultiVector = other_959;
    return MultiVector(((vec4<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.z, _e13.g0_.y) * vec4<f32>(0.0, 0.0, 1.0, 1.0)) + _e23.g0_), ((vec4<f32>(_e26.g0_.x) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) + _e36.g1_));
}

fn plane_multi_vector_sub(self_1154: Plane, other_960: MultiVector) -> MultiVector {
    var self_1155: Plane;
    var other_961: MultiVector;

    self_1155 = self_1154;
    other_961 = other_960;
    let _e4: Plane = self_1155;
    let _e7: Plane = self_1155;
    let _e10: Plane = self_1155;
    let _e13: Plane = self_1155;
    let _e23: MultiVector = other_961;
    let _e26: Plane = self_1155;
    let _e36: MultiVector = other_961;
    return MultiVector(((vec4<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.z, _e13.g0_.y) * vec4<f32>(0.0, 0.0, 1.0, 1.0)) - _e23.g0_), ((vec4<f32>(_e26.g0_.x) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) - _e36.g1_));
}

fn plane_multi_vector_geometric_product(self_1156: Plane, other_962: MultiVector) -> MultiVector {
    var self_1157: Plane;
    var other_963: MultiVector;

    self_1157 = self_1156;
    other_963 = other_962;
    let _e4: Plane = self_1157;
    let _e8: MultiVector = other_963;
    let _e20: Plane = self_1157;
    let _e24: MultiVector = other_963;
    let _e29: Plane = self_1157;
    let _e33: MultiVector = other_963;
    let _e43: Plane = self_1157;
    let _e47: MultiVector = other_963;
    let _e52: Plane = self_1157;
    let _e56: MultiVector = other_963;
    return MultiVector((((vec4<f32>(_e4.g0_.y) * _e8.g0_.wzyx) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0)) + (vec4<f32>(_e20.g0_.z) * _e24.g0_.zwxy)), ((((vec4<f32>(_e29.g0_.x) * _e33.g0_) * vec4<f32>(1.0, 1.0, 1.0, -(1.0))) + (vec4<f32>(_e43.g0_.y) * _e47.g1_.wzyx)) + ((vec4<f32>(_e52.g0_.z) * _e56.g1_.zwxy) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))));
}

fn plane_multi_vector_geometric_anti_product(self_1158: Plane, other_964: MultiVector) -> MultiVector {
    var self_1159: Plane;
    var other_965: MultiVector;

    self_1159 = self_1158;
    other_965 = other_964;
    let _e4: Plane = self_1159;
    let _e8: MultiVector = other_965;
    let _e20: Plane = self_1159;
    let _e24: MultiVector = other_965;
    let _e36: Plane = self_1159;
    let _e40: MultiVector = other_965;
    let _e52: Plane = self_1159;
    let _e56: MultiVector = other_965;
    return MultiVector(((((vec4<f32>(_e4.g0_.x) * _e8.g0_.yxwz) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0))) + ((vec4<f32>(_e20.g0_.y) * _e24.g1_.zwxy) * vec4<f32>(1.0, 1.0, -(1.0), 1.0))) + ((vec4<f32>(_e36.g0_.z) * _e40.g1_.wzyx) * vec4<f32>(1.0, -(1.0), 1.0, 1.0))), ((vec4<f32>(_e52.g0_.x) * _e56.g1_.yxwz) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0)));
}

fn plane_multi_vector_scalar_product(self_1160: Plane, other_966: MultiVector) -> Scalar {
    var self_1161: Plane;
    var other_967: MultiVector;

    self_1161 = self_1160;
    other_967 = other_966;
    let _e4: Plane = self_1161;
    let _e7: MultiVector = other_967;
    let _e11: Plane = self_1161;
    let _e14: MultiVector = other_967;
    return Scalar(((_e4.g0_.y * _e7.g0_.w) + (_e11.g0_.z * _e14.g0_.z)));
}

fn plane_multi_vector_anti_scalar_product(self_1162: Plane, other_968: MultiVector) -> AntiScalar {
    var self_1163: Plane;
    var other_969: MultiVector;

    self_1163 = self_1162;
    other_969 = other_968;
    let _e5: Plane = self_1163;
    let _e8: MultiVector = other_969;
    return AntiScalar((0.0 - (_e5.g0_.x * _e8.g1_.x)));
}

fn plane_rotor_geometric_product(self_1164: Plane, other_970: Rotor) -> MotorDual {
    var self_1165: Plane;
    var other_971: Rotor;

    self_1165 = self_1164;
    other_971 = other_970;
    let _e4: Plane = self_1165;
    let _e8: Rotor = other_971;
    let _e11: Rotor = other_971;
    let _e14: Rotor = other_971;
    let _e17: Rotor = other_971;
    let _e28: Plane = self_1165;
    let _e31: Plane = self_1165;
    let _e34: Plane = self_1165;
    let _e37: Plane = self_1165;
    let _e41: Rotor = other_971;
    let _e44: Rotor = other_971;
    let _e47: Rotor = other_971;
    let _e50: Rotor = other_971;
    return MotorDual((((vec4<f32>(_e4.g0_.z) * vec4<f32>(_e8.g0_.y, _e11.g0_.y, _e14.g0_.y, _e17.g0_.x)) * vec4<f32>(0.0, 0.0, 1.0, 1.0)) + ((vec4<f32>(_e28.g0_.x, _e31.g0_.x, _e34.g0_.y, _e37.g0_.y) * vec4<f32>(_e41.g0_.y, _e44.g0_.x, _e47.g0_.x, _e50.g0_.y)) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))));
}

fn plane_rotor_regressive_product(self_1166: Plane, other_972: Rotor) -> Scalar {
    var self_1167: Plane;
    var other_973: Rotor;

    self_1167 = self_1166;
    other_973 = other_972;
    let _e4: Plane = self_1167;
    let _e7: Rotor = other_973;
    return Scalar((_e4.g0_.x * _e7.g0_.y));
}

fn plane_rotor_outer_product(self_1168: Plane, other_974: Rotor) -> MotorDual {
    var self_1169: Plane;
    var other_975: Rotor;

    self_1169 = self_1168;
    other_975 = other_974;
    let _e4: Plane = self_1169;
    let _e7: Plane = self_1169;
    let _e10: Plane = self_1169;
    let _e13: Plane = self_1169;
    let _e17: Rotor = other_975;
    let _e20: Rotor = other_975;
    let _e23: Rotor = other_975;
    let _e26: Rotor = other_975;
    return MotorDual((vec4<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.y, _e13.g0_.z) * vec4<f32>(_e17.g0_.y, _e20.g0_.x, _e23.g0_.x, _e26.g0_.x)));
}

fn plane_rotor_inner_product(self_1170: Plane, other_976: Rotor) -> Plane {
    var self_1171: Plane;
    var other_977: Rotor;

    self_1171 = self_1170;
    other_977 = other_976;
    let _e4: Plane = self_1171;
    let _e8: Rotor = other_977;
    let _e11: Rotor = other_977;
    let _e14: Rotor = other_977;
    let _e24: Plane = self_1171;
    let _e27: Rotor = other_977;
    let _e30: Rotor = other_977;
    let _e33: Rotor = other_977;
    return Plane((((vec3<f32>(_e4.g0_.z) * vec3<f32>(_e8.g0_.y, _e11.g0_.y, _e14.g0_.x)) * vec3<f32>(0.0, 1.0, 1.0)) + ((_e24.g0_.xyy * vec3<f32>(_e27.g0_.x, _e30.g0_.x, _e33.g0_.y)) * vec3<f32>(1.0, 1.0, -(1.0)))));
}

fn plane_rotor_geometric_anti_product(self_1172: Plane, other_978: Rotor) -> Rotor {
    var self_1173: Plane;
    var other_979: Rotor;

    self_1173 = self_1172;
    other_979 = other_978;
    let _e4: Plane = self_1173;
    let _e8: Rotor = other_979;
    return Rotor(((vec2<f32>(_e4.g0_.x) * _e8.g0_.yx) * vec2<f32>(1.0, -(1.0))));
}

fn plane_rotor_right_contraction(self_1174: Plane, other_980: Rotor) -> Plane {
    var self_1175: Plane;
    var other_981: Rotor;

    self_1175 = self_1174;
    other_981 = other_980;
    let _e4: Plane = self_1175;
    let _e6: Rotor = other_981;
    return Plane((_e4.g0_ * vec3<f32>(_e6.g0_.x)));
}

fn plane_point_geometric_product(self_1176: Plane, other_982: Point) -> MotorDual {
    var self_1177: Plane;
    var other_983: Point;

    self_1177 = self_1176;
    other_983 = other_982;
    let _e4: Plane = self_1177;
    let _e8: Point = other_983;
    let _e11: Point = other_983;
    let _e14: Point = other_983;
    let _e17: Point = other_983;
    let _e29: Plane = self_1177;
    let _e33: Point = other_983;
    let _e36: Point = other_983;
    let _e39: Point = other_983;
    let _e42: Point = other_983;
    let _e55: Plane = self_1177;
    let _e59: Point = other_983;
    return MotorDual(((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y, _e11.g0_.z, _e14.g0_.y, _e17.g0_.x)) * vec4<f32>(1.0, 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e29.g0_.z) * vec4<f32>(_e33.g0_.z, _e36.g0_.y, _e39.g0_.x, _e42.g0_.z)) * vec4<f32>(1.0, -(1.0), 1.0, 0.0))) + ((vec4<f32>(_e55.g0_.x) * vec4<f32>(_e59.g0_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))));
}

fn plane_point_regressive_product(self_1178: Plane, other_984: Point) -> Scalar {
    var self_1179: Plane;
    var other_985: Point;

    self_1179 = self_1178;
    other_985 = other_984;
    let _e4: Plane = self_1179;
    let _e7: Point = other_985;
    let _e11: Plane = self_1179;
    let _e14: Point = other_985;
    let _e19: Plane = self_1179;
    let _e22: Point = other_985;
    return Scalar((((_e4.g0_.x * _e7.g0_.x) + (_e11.g0_.y * _e14.g0_.y)) + (_e19.g0_.z * _e22.g0_.z)));
}

fn plane_point_outer_product(self_1180: Plane, other_986: Point) -> AntiScalar {
    var self_1181: Plane;
    var other_987: Point;

    self_1181 = self_1180;
    other_987 = other_986;
    let _e4: Plane = self_1181;
    let _e7: Point = other_987;
    let _e11: Plane = self_1181;
    let _e14: Point = other_987;
    let _e19: Plane = self_1181;
    let _e22: Point = other_987;
    return AntiScalar((((_e4.g0_.x * _e7.g0_.x) + (_e11.g0_.y * _e14.g0_.y)) + (_e19.g0_.z * _e22.g0_.z)));
}

fn plane_point_inner_product(self_1182: Plane, other_988: Point) -> Plane {
    var self_1183: Plane;
    var other_989: Point;

    self_1183 = self_1182;
    other_989 = other_988;
    let _e4: Plane = self_1183;
    let _e8: Point = other_989;
    let _e18: Plane = self_1183;
    let _e21: Point = other_989;
    return Plane((((vec3<f32>(_e4.g0_.z) * _e8.g0_.yxy) * vec3<f32>(-(1.0), 1.0, 0.0)) + ((_e18.g0_.yxy * _e21.g0_.zxx) * vec3<f32>(1.0, 0.0, -(1.0)))));
}

fn plane_point_geometric_anti_product(self_1184: Plane, other_990: Point) -> Motor {
    var self_1185: Plane;
    var other_991: Point;

    self_1185 = self_1184;
    other_991 = other_990;
    let _e4: Plane = self_1185;
    let _e8: Point = other_991;
    let _e11: Point = other_991;
    let _e14: Point = other_991;
    let _e17: Point = other_991;
    let _e28: Plane = self_1185;
    let _e32: Point = other_991;
    let _e35: Point = other_991;
    let _e38: Point = other_991;
    let _e41: Point = other_991;
    let _e54: Plane = self_1185;
    let _e58: Point = other_991;
    let _e61: Point = other_991;
    let _e64: Point = other_991;
    let _e67: Point = other_991;
    return Motor(((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y, _e11.g0_.z, _e14.g0_.y, _e17.g0_.y)) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) + ((vec4<f32>(_e28.g0_.z) * vec4<f32>(_e32.g0_.z, _e35.g0_.y, _e38.g0_.z, _e41.g0_.z)) * vec4<f32>(1.0, -(1.0), 0.0, 0.0))) + ((vec4<f32>(_e54.g0_.x) * vec4<f32>(_e58.g0_.x, _e61.g0_.x, _e64.g0_.z, _e67.g0_.y)) * vec4<f32>(1.0, 0.0, -(1.0), 1.0))));
}

fn plane_point_inner_anti_product(self_1186: Plane, other_992: Point) -> Point {
    var self_1187: Plane;
    var other_993: Point;

    self_1187 = self_1186;
    other_993 = other_992;
    let _e4: Plane = self_1187;
    let _e8: Point = other_993;
    let _e19: Plane = self_1187;
    let _e22: Point = other_993;
    return Point((((vec3<f32>(_e4.g0_.z) * vec3<f32>(_e8.g0_.y)) * vec3<f32>(-(1.0), 0.0, 0.0)) + ((_e19.g0_.yxx * _e22.g0_.zzy) * vec3<f32>(1.0, -(1.0), 1.0))));
}

fn plane_point_left_contraction(self_1188: Plane, other_994: Point) -> Plane {
    var self_1189: Plane;
    var other_995: Point;

    self_1189 = self_1188;
    other_995 = other_994;
    let _e4: Plane = self_1189;
    let _e8: Point = other_995;
    let _e18: Plane = self_1189;
    let _e21: Point = other_995;
    return Plane((((vec3<f32>(_e4.g0_.z) * _e8.g0_.yxy) * vec3<f32>(-(1.0), 1.0, 0.0)) + ((_e18.g0_.yxy * _e21.g0_.zxx) * vec3<f32>(1.0, 0.0, -(1.0)))));
}

fn plane_point_right_anti_contraction(self_1190: Plane, other_996: Point) -> Point {
    var self_1191: Plane;
    var other_997: Point;

    self_1191 = self_1190;
    other_997 = other_996;
    let _e4: Plane = self_1191;
    let _e8: Point = other_997;
    let _e19: Plane = self_1191;
    let _e22: Point = other_997;
    return Point((((vec3<f32>(_e4.g0_.z) * vec3<f32>(_e8.g0_.y)) * vec3<f32>(-(1.0), 0.0, 0.0)) + ((_e19.g0_.yxx * _e22.g0_.zzy) * vec3<f32>(1.0, -(1.0), 1.0))));
}

fn plane_ideal_point_regressive_product(self_1192: Plane, other_998: IdealPoint) -> Scalar {
    var self_1193: Plane;
    var other_999: IdealPoint;

    self_1193 = self_1192;
    other_999 = other_998;
    let _e4: Plane = self_1193;
    let _e7: IdealPoint = other_999;
    let _e11: Plane = self_1193;
    let _e14: IdealPoint = other_999;
    return Scalar(((_e4.g0_.y * _e7.g0_.x) + (_e11.g0_.z * _e14.g0_.y)));
}

fn plane_ideal_point_outer_product(self_1194: Plane, other_1000: IdealPoint) -> AntiScalar {
    var self_1195: Plane;
    var other_1001: IdealPoint;

    self_1195 = self_1194;
    other_1001 = other_1000;
    let _e4: Plane = self_1195;
    let _e7: IdealPoint = other_1001;
    let _e11: Plane = self_1195;
    let _e14: IdealPoint = other_1001;
    return AntiScalar(((_e4.g0_.y * _e7.g0_.x) + (_e11.g0_.z * _e14.g0_.y)));
}

fn plane_ideal_point_geometric_anti_product(self_1196: Plane, other_1002: IdealPoint) -> Motor {
    var self_1197: Plane;
    var other_1003: IdealPoint;

    self_1197 = self_1196;
    other_1003 = other_1002;
    let _e4: Plane = self_1197;
    let _e8: IdealPoint = other_1003;
    let _e11: IdealPoint = other_1003;
    let _e14: IdealPoint = other_1003;
    let _e17: IdealPoint = other_1003;
    let _e29: Plane = self_1197;
    let _e32: Plane = self_1197;
    let _e35: Plane = self_1197;
    let _e38: Plane = self_1197;
    let _e42: IdealPoint = other_1003;
    let _e45: IdealPoint = other_1003;
    let _e48: IdealPoint = other_1003;
    let _e51: IdealPoint = other_1003;
    return Motor((((vec4<f32>(_e4.g0_.z) * vec4<f32>(_e8.g0_.y, _e11.g0_.x, _e14.g0_.y, _e17.g0_.y)) * vec4<f32>(1.0, -(1.0), 0.0, 0.0)) + ((vec4<f32>(_e29.g0_.y, _e32.g0_.y, _e35.g0_.x, _e38.g0_.x) * vec4<f32>(_e42.g0_.x, _e45.g0_.y, _e48.g0_.y, _e51.g0_.x)) * vec4<f32>(1.0, 1.0, -(1.0), 1.0))));
}

fn plane_ideal_point_inner_anti_product(self_1198: Plane, other_1004: IdealPoint) -> Point {
    var self_1199: Plane;
    var other_1005: IdealPoint;

    self_1199 = self_1198;
    other_1005 = other_1004;
    let _e4: Plane = self_1199;
    let _e8: IdealPoint = other_1005;
    let _e19: Plane = self_1199;
    let _e22: IdealPoint = other_1005;
    let _e25: IdealPoint = other_1005;
    let _e28: IdealPoint = other_1005;
    return Point((((vec3<f32>(_e4.g0_.z) * vec3<f32>(_e8.g0_.x)) * vec3<f32>(-(1.0), 0.0, 0.0)) + ((_e19.g0_.yxx * vec3<f32>(_e22.g0_.y, _e25.g0_.y, _e28.g0_.x)) * vec3<f32>(1.0, -(1.0), 1.0))));
}

fn plane_ideal_point_right_anti_contraction(self_1200: Plane, other_1006: IdealPoint) -> Point {
    var self_1201: Plane;
    var other_1007: IdealPoint;

    self_1201 = self_1200;
    other_1007 = other_1006;
    let _e4: Plane = self_1201;
    let _e8: IdealPoint = other_1007;
    let _e19: Plane = self_1201;
    let _e22: IdealPoint = other_1007;
    let _e25: IdealPoint = other_1007;
    let _e28: IdealPoint = other_1007;
    return Point((((vec3<f32>(_e4.g0_.z) * vec3<f32>(_e8.g0_.x)) * vec3<f32>(-(1.0), 0.0, 0.0)) + ((_e19.g0_.yxx * vec3<f32>(_e22.g0_.y, _e25.g0_.y, _e28.g0_.x)) * vec3<f32>(1.0, -(1.0), 1.0))));
}

fn plane_plane_add(self_1202: Plane, other_1008: Plane) -> Plane {
    var self_1203: Plane;
    var other_1009: Plane;

    self_1203 = self_1202;
    other_1009 = other_1008;
    let _e4: Plane = self_1203;
    let _e6: Plane = other_1009;
    return Plane((_e4.g0_ + _e6.g0_));
}

fn plane_plane_sub(self_1204: Plane, other_1010: Plane) -> Plane {
    var self_1205: Plane;
    var other_1011: Plane;

    self_1205 = self_1204;
    other_1011 = other_1010;
    let _e4: Plane = self_1205;
    let _e6: Plane = other_1011;
    return Plane((_e4.g0_ - _e6.g0_));
}

fn plane_plane_mul(self_1206: Plane, other_1012: Plane) -> Plane {
    var self_1207: Plane;
    var other_1013: Plane;

    self_1207 = self_1206;
    other_1013 = other_1012;
    let _e4: Plane = self_1207;
    let _e6: Plane = other_1013;
    return Plane((_e4.g0_ * _e6.g0_));
}

fn plane_plane_div(self_1208: Plane, other_1014: Plane) -> Plane {
    var self_1209: Plane;
    var other_1015: Plane;

    self_1209 = self_1208;
    other_1015 = other_1014;
    let _e4: Plane = self_1209;
    let _e7: Plane = self_1209;
    let _e10: Plane = self_1209;
    let _e19: Plane = other_1015;
    let _e22: Plane = other_1015;
    let _e25: Plane = other_1015;
    return Plane((((vec3<f32>(_e4.g0_.x, _e7.g0_.y, _e10.g0_.z) * vec3<f32>(1.0, 1.0, 1.0)) / vec3<f32>(_e19.g0_.x, _e22.g0_.y, _e25.g0_.z)) * vec3<f32>(1.0, 1.0, 1.0)));
}

fn plane_plane_geometric_product(self_1210: Plane, other_1016: Plane) -> Motor {
    var self_1211: Plane;
    var other_1017: Plane;

    self_1211 = self_1210;
    other_1017 = other_1016;
    let _e4: Plane = self_1211;
    let _e8: Plane = other_1017;
    let _e11: Plane = other_1017;
    let _e14: Plane = other_1017;
    let _e17: Plane = other_1017;
    let _e29: Plane = self_1211;
    let _e33: Plane = other_1017;
    let _e36: Plane = other_1017;
    let _e39: Plane = other_1017;
    let _e42: Plane = other_1017;
    let _e55: Plane = self_1211;
    let _e59: Plane = other_1017;
    let _e62: Plane = other_1017;
    let _e65: Plane = other_1017;
    let _e68: Plane = other_1017;
    return Motor(((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y, _e11.g0_.z, _e14.g0_.y, _e17.g0_.x)) * vec4<f32>(1.0, -(1.0), 0.0, 1.0)) + ((vec4<f32>(_e29.g0_.z) * vec4<f32>(_e33.g0_.z, _e36.g0_.y, _e39.g0_.x, _e42.g0_.z)) * vec4<f32>(1.0, 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e55.g0_.x) * vec4<f32>(_e59.g0_.x, _e62.g0_.x, _e65.g0_.z, _e68.g0_.y)) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))));
}

fn plane_plane_outer_product(self_1212: Plane, other_1018: Plane) -> Point {
    var self_1213: Plane;
    var other_1019: Plane;

    self_1213 = self_1212;
    other_1019 = other_1018;
    let _e4: Plane = self_1213;
    let _e8: Plane = other_1019;
    let _e18: Plane = self_1213;
    let _e22: Plane = other_1019;
    let _e33: Plane = self_1213;
    let _e37: Plane = other_1019;
    return Point(((((vec3<f32>(_e4.g0_.y) * _e8.g0_.zzx) * vec3<f32>(-(1.0), 0.0, 1.0)) + ((vec3<f32>(_e18.g0_.z) * _e22.g0_.yxy) * vec3<f32>(1.0, -(1.0), 0.0))) + ((vec3<f32>(_e33.g0_.x) * _e37.g0_.xzy) * vec3<f32>(0.0, 1.0, -(1.0)))));
}

fn plane_plane_inner_product(self_1214: Plane, other_1020: Plane) -> Scalar {
    var self_1215: Plane;
    var other_1021: Plane;

    self_1215 = self_1214;
    other_1021 = other_1020;
    let _e4: Plane = self_1215;
    let _e7: Plane = other_1021;
    let _e11: Plane = self_1215;
    let _e14: Plane = other_1021;
    return Scalar(((_e4.g0_.y * _e7.g0_.y) + (_e11.g0_.z * _e14.g0_.z)));
}

fn plane_plane_inner_anti_product(self_1216: Plane, other_1022: Plane) -> AntiScalar {
    var self_1217: Plane;
    var other_1023: Plane;

    self_1217 = self_1216;
    other_1023 = other_1022;
    let _e5: Plane = self_1217;
    let _e8: Plane = other_1023;
    return AntiScalar((0.0 - (_e5.g0_.x * _e8.g0_.x)));
}

fn plane_plane_left_contraction(self_1218: Plane, other_1024: Plane) -> Scalar {
    var self_1219: Plane;
    var other_1025: Plane;

    self_1219 = self_1218;
    other_1025 = other_1024;
    let _e4: Plane = self_1219;
    let _e7: Plane = other_1025;
    let _e11: Plane = self_1219;
    let _e14: Plane = other_1025;
    return Scalar(((_e4.g0_.y * _e7.g0_.y) + (_e11.g0_.z * _e14.g0_.z)));
}

fn plane_plane_right_contraction(self_1220: Plane, other_1026: Plane) -> Scalar {
    var self_1221: Plane;
    var other_1027: Plane;

    self_1221 = self_1220;
    other_1027 = other_1026;
    let _e4: Plane = self_1221;
    let _e7: Plane = other_1027;
    let _e11: Plane = self_1221;
    let _e14: Plane = other_1027;
    return Scalar(((_e4.g0_.y * _e7.g0_.y) + (_e11.g0_.z * _e14.g0_.z)));
}

fn plane_plane_left_anti_contraction(self_1222: Plane, other_1028: Plane) -> AntiScalar {
    var self_1223: Plane;
    var other_1029: Plane;

    self_1223 = self_1222;
    other_1029 = other_1028;
    let _e5: Plane = self_1223;
    let _e8: Plane = other_1029;
    return AntiScalar((0.0 - (_e5.g0_.x * _e8.g0_.x)));
}

fn plane_plane_right_anti_contraction(self_1224: Plane, other_1030: Plane) -> AntiScalar {
    var self_1225: Plane;
    var other_1031: Plane;

    self_1225 = self_1224;
    other_1031 = other_1030;
    let _e5: Plane = self_1225;
    let _e8: Plane = other_1031;
    return AntiScalar((0.0 - (_e5.g0_.x * _e8.g0_.x)));
}

fn plane_plane_scalar_product(self_1226: Plane, other_1032: Plane) -> Scalar {
    var self_1227: Plane;
    var other_1033: Plane;

    self_1227 = self_1226;
    other_1033 = other_1032;
    let _e4: Plane = self_1227;
    let _e7: Plane = other_1033;
    let _e11: Plane = self_1227;
    let _e14: Plane = other_1033;
    return Scalar(((_e4.g0_.y * _e7.g0_.y) + (_e11.g0_.z * _e14.g0_.z)));
}

fn plane_plane_anti_scalar_product(self_1228: Plane, other_1034: Plane) -> AntiScalar {
    var self_1229: Plane;
    var other_1035: Plane;

    self_1229 = self_1228;
    other_1035 = other_1034;
    let _e5: Plane = self_1229;
    let _e8: Plane = other_1035;
    return AntiScalar((0.0 - (_e5.g0_.x * _e8.g0_.x)));
}

fn plane_translator_geometric_product(self_1230: Plane, other_1036: Translator) -> MotorDual {
    var self_1231: Plane;
    var other_1037: Translator;

    self_1231 = self_1230;
    other_1037 = other_1036;
    let _e4: Plane = self_1231;
    let _e8: Translator = other_1037;
    let _e11: Translator = other_1037;
    let _e14: Translator = other_1037;
    let _e17: Translator = other_1037;
    let _e28: Plane = self_1231;
    let _e32: Translator = other_1037;
    let _e35: Translator = other_1037;
    let _e38: Translator = other_1037;
    let _e41: Translator = other_1037;
    let _e54: Plane = self_1231;
    let _e58: Translator = other_1037;
    return MotorDual(((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y, _e11.g0_.z, _e14.g0_.x, _e17.g0_.y)) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) + ((vec4<f32>(_e28.g0_.z) * vec4<f32>(_e32.g0_.z, _e35.g0_.y, _e38.g0_.z, _e41.g0_.x)) * vec4<f32>(1.0, -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e54.g0_.x) * vec4<f32>(_e58.g0_.x)) * vec4<f32>(0.0, 1.0, 0.0, 0.0))));
}

fn plane_translator_regressive_product(self_1232: Plane, other_1038: Translator) -> Scalar {
    var self_1233: Plane;
    var other_1039: Translator;

    self_1233 = self_1232;
    other_1039 = other_1038;
    let _e4: Plane = self_1233;
    let _e7: Translator = other_1039;
    let _e11: Plane = self_1233;
    let _e14: Translator = other_1039;
    return Scalar(((_e4.g0_.y * _e7.g0_.y) + (_e11.g0_.z * _e14.g0_.z)));
}

fn plane_translator_outer_product(self_1234: Plane, other_1040: Translator) -> MotorDual {
    var self_1235: Plane;
    var other_1041: Translator;

    self_1235 = self_1234;
    other_1041 = other_1040;
    let _e4: Plane = self_1235;
    let _e8: Translator = other_1041;
    let _e11: Translator = other_1041;
    let _e14: Translator = other_1041;
    let _e17: Translator = other_1041;
    let _e28: Plane = self_1235;
    let _e31: Plane = self_1235;
    let _e34: Plane = self_1235;
    let _e37: Plane = self_1235;
    let _e41: Translator = other_1041;
    let _e44: Translator = other_1041;
    let _e47: Translator = other_1041;
    let _e50: Translator = other_1041;
    return MotorDual((((vec4<f32>(_e4.g0_.z) * vec4<f32>(_e8.g0_.z, _e11.g0_.z, _e14.g0_.z, _e17.g0_.x)) * vec4<f32>(1.0, 0.0, 0.0, 1.0)) + ((vec4<f32>(_e28.g0_.y, _e31.g0_.x, _e34.g0_.y, _e37.g0_.x) * vec4<f32>(_e41.g0_.y, _e44.g0_.x, _e47.g0_.x, _e50.g0_.x)) * vec4<f32>(1.0, 1.0, 1.0, 0.0))));
}

fn plane_translator_inner_product(self_1236: Plane, other_1042: Translator) -> Plane {
    var self_1237: Plane;
    var other_1043: Translator;

    self_1237 = self_1236;
    other_1043 = other_1042;
    let _e4: Plane = self_1237;
    let _e8: Translator = other_1043;
    let _e17: Plane = self_1237;
    let _e21: Translator = other_1043;
    let _e32: Plane = self_1237;
    let _e36: Translator = other_1043;
    return Plane(((((vec3<f32>(_e4.g0_.y) * _e8.g0_.zxz) * vec3<f32>(1.0, 1.0, 0.0)) + ((vec3<f32>(_e17.g0_.z) * _e21.g0_.yyx) * vec3<f32>(-(1.0), 0.0, 1.0))) + ((vec3<f32>(_e32.g0_.x) * vec3<f32>(_e36.g0_.x)) * vec3<f32>(1.0, 0.0, 0.0))));
}

fn plane_translator_geometric_anti_product(self_1238: Plane, other_1044: Translator) -> Motor {
    var self_1239: Plane;
    var other_1045: Translator;

    self_1239 = self_1238;
    other_1045 = other_1044;
    let _e4: Plane = self_1239;
    let _e8: Translator = other_1045;
    let _e11: Translator = other_1045;
    let _e14: Translator = other_1045;
    let _e17: Translator = other_1045;
    let _e28: Plane = self_1239;
    let _e32: Translator = other_1045;
    let _e35: Translator = other_1045;
    let _e38: Translator = other_1045;
    let _e41: Translator = other_1045;
    let _e54: Plane = self_1239;
    let _e58: Translator = other_1045;
    let _e61: Translator = other_1045;
    let _e64: Translator = other_1045;
    let _e67: Translator = other_1045;
    return Motor(((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y, _e11.g0_.z, _e14.g0_.y, _e17.g0_.y)) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) + ((vec4<f32>(_e28.g0_.z) * vec4<f32>(_e32.g0_.z, _e35.g0_.y, _e38.g0_.z, _e41.g0_.z)) * vec4<f32>(1.0, -(1.0), 0.0, 0.0))) + ((vec4<f32>(_e54.g0_.x) * vec4<f32>(_e58.g0_.x, _e61.g0_.x, _e64.g0_.z, _e67.g0_.y)) * vec4<f32>(0.0, -(1.0), -(1.0), 1.0))));
}

fn plane_translator_inner_anti_product(self_1240: Plane, other_1046: Translator) -> Point {
    var self_1241: Plane;
    var other_1047: Translator;

    self_1241 = self_1240;
    other_1047 = other_1046;
    let _e4: Plane = self_1241;
    let _e8: Translator = other_1047;
    let _e19: Plane = self_1241;
    let _e23: Translator = other_1047;
    let _e35: Plane = self_1241;
    let _e38: Translator = other_1047;
    return Point(((((vec3<f32>(_e4.g0_.x) * _e8.g0_.xzy) * vec3<f32>(-(1.0), -(1.0), 1.0)) + ((vec3<f32>(_e19.g0_.z) * vec3<f32>(_e23.g0_.y)) * vec3<f32>(-(1.0), 0.0, 0.0))) + ((_e35.g0_.yxx * _e38.g0_.zxx) * vec3<f32>(1.0, 0.0, 0.0))));
}

fn plane_translator_right_contraction(self_1242: Plane, other_1048: Translator) -> Plane {
    var self_1243: Plane;
    var other_1049: Translator;

    self_1243 = self_1242;
    other_1049 = other_1048;
    let _e4: Plane = self_1243;
    let _e6: Translator = other_1049;
    return Plane((_e4.g0_ * vec3<f32>(_e6.g0_.x)));
}

fn plane_translator_right_anti_contraction(self_1244: Plane, other_1050: Translator) -> Point {
    var self_1245: Plane;
    var other_1051: Translator;

    self_1245 = self_1244;
    other_1051 = other_1050;
    let _e4: Plane = self_1245;
    let _e8: Translator = other_1051;
    let _e19: Plane = self_1245;
    let _e22: Translator = other_1051;
    return Point((((vec3<f32>(_e4.g0_.z) * vec3<f32>(_e8.g0_.y)) * vec3<f32>(-(1.0), 0.0, 0.0)) + ((_e19.g0_.yxx * _e22.g0_.zzy) * vec3<f32>(1.0, -(1.0), 1.0))));
}

fn plane_motor_geometric_product(self_1246: Plane, other_1052: Motor) -> MotorDual {
    var self_1247: Plane;
    var other_1053: Motor;

    self_1247 = self_1246;
    other_1053 = other_1052;
    let _e4: Plane = self_1247;
    let _e8: Motor = other_1053;
    let _e19: Plane = self_1247;
    let _e23: Motor = other_1053;
    let _e35: Plane = self_1247;
    let _e39: Motor = other_1053;
    return MotorDual(((((vec4<f32>(_e4.g0_.y) * _e8.g0_.zwxy) * vec4<f32>(1.0, 1.0, 1.0, -(1.0))) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.wzyx) * vec4<f32>(1.0, -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e35.g0_.x) * _e39.g0_.yxxx) * vec4<f32>(1.0, 1.0, 0.0, 0.0))));
}

fn plane_motor_regressive_product(self_1248: Plane, other_1054: Motor) -> Scalar {
    var self_1249: Plane;
    var other_1055: Motor;

    self_1249 = self_1248;
    other_1055 = other_1054;
    let _e4: Plane = self_1249;
    let _e7: Motor = other_1055;
    let _e11: Plane = self_1249;
    let _e14: Motor = other_1055;
    let _e19: Plane = self_1249;
    let _e22: Motor = other_1055;
    return Scalar((((_e4.g0_.x * _e7.g0_.y) + (_e11.g0_.y * _e14.g0_.z)) + (_e19.g0_.z * _e22.g0_.w)));
}

fn plane_motor_outer_product(self_1250: Plane, other_1056: Motor) -> MotorDual {
    var self_1251: Plane;
    var other_1057: Motor;

    self_1251 = self_1250;
    other_1057 = other_1056;
    let _e4: Plane = self_1251;
    let _e8: Motor = other_1057;
    let _e18: Plane = self_1251;
    let _e22: Motor = other_1057;
    let _e33: Plane = self_1251;
    let _e37: Motor = other_1057;
    return MotorDual(((((vec4<f32>(_e4.g0_.y) * _e8.g0_.zzxz) * vec4<f32>(1.0, 0.0, 1.0, 0.0)) + ((vec4<f32>(_e18.g0_.z) * _e22.g0_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e33.g0_.x) * _e37.g0_.yxxx) * vec4<f32>(1.0, 1.0, 0.0, 0.0))));
}

fn plane_motor_inner_product(self_1252: Plane, other_1058: Motor) -> Plane {
    var self_1253: Plane;
    var other_1059: Motor;

    self_1253 = self_1252;
    other_1059 = other_1058;
    let _e4: Plane = self_1253;
    let _e8: Motor = other_1059;
    let _e11: Motor = other_1059;
    let _e14: Motor = other_1059;
    let _e25: Plane = self_1253;
    let _e29: Motor = other_1059;
    let _e32: Motor = other_1059;
    let _e35: Motor = other_1059;
    let _e47: Plane = self_1253;
    let _e51: Motor = other_1059;
    return Plane(((((vec3<f32>(_e4.g0_.y) * vec3<f32>(_e8.g0_.w, _e11.g0_.x, _e14.g0_.y)) * vec3<f32>(1.0, 1.0, -(1.0))) + ((vec3<f32>(_e25.g0_.z) * vec3<f32>(_e29.g0_.z, _e32.g0_.y, _e35.g0_.x)) * vec3<f32>(-(1.0), 1.0, 1.0))) + ((vec3<f32>(_e47.g0_.x) * vec3<f32>(_e51.g0_.x)) * vec3<f32>(1.0, 0.0, 0.0))));
}

fn plane_motor_geometric_anti_product(self_1254: Plane, other_1060: Motor) -> Motor {
    var self_1255: Plane;
    var other_1061: Motor;

    self_1255 = self_1254;
    other_1061 = other_1060;
    let _e4: Plane = self_1255;
    let _e8: Motor = other_1061;
    let _e20: Plane = self_1255;
    let _e24: Motor = other_1061;
    let _e36: Plane = self_1255;
    let _e39: Plane = self_1255;
    let _e42: Plane = self_1255;
    let _e45: Plane = self_1255;
    let _e49: Motor = other_1061;
    return Motor(((((vec4<f32>(_e4.g0_.x) * _e8.g0_.yxwz) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0)) + ((vec4<f32>(_e20.g0_.z) * _e24.g0_.wzww) * vec4<f32>(1.0, -(1.0), 0.0, 0.0))) + ((vec4<f32>(_e36.g0_.y, _e39.g0_.y, _e42.g0_.x, _e45.g0_.x) * _e49.g0_.zwxx) * vec4<f32>(1.0, 1.0, 0.0, 0.0))));
}

fn plane_motor_inner_anti_product(self_1256: Plane, other_1062: Motor) -> Point {
    var self_1257: Plane;
    var other_1063: Motor;

    self_1257 = self_1256;
    other_1063 = other_1062;
    let _e4: Plane = self_1257;
    let _e8: Motor = other_1063;
    let _e11: Motor = other_1063;
    let _e14: Motor = other_1063;
    let _e26: Plane = self_1257;
    let _e30: Motor = other_1063;
    let _e42: Plane = self_1257;
    let _e45: Motor = other_1063;
    let _e48: Motor = other_1063;
    let _e51: Motor = other_1063;
    return Point(((((vec3<f32>(_e4.g0_.x) * vec3<f32>(_e8.g0_.x, _e11.g0_.w, _e14.g0_.z)) * vec3<f32>(-(1.0), -(1.0), 1.0)) + ((vec3<f32>(_e26.g0_.z) * vec3<f32>(_e30.g0_.z)) * vec3<f32>(-(1.0), 0.0, 0.0))) + ((_e42.g0_.yxx * vec3<f32>(_e45.g0_.w, _e48.g0_.x, _e51.g0_.x)) * vec3<f32>(1.0, 0.0, 0.0))));
}

fn plane_motor_left_contraction(self_1258: Plane, other_1064: Motor) -> Plane {
    var self_1259: Plane;
    var other_1065: Motor;

    self_1259 = self_1258;
    other_1065 = other_1064;
    let _e4: Plane = self_1259;
    let _e8: Motor = other_1065;
    let _e11: Motor = other_1065;
    let _e14: Motor = other_1065;
    let _e25: Plane = self_1259;
    let _e28: Motor = other_1065;
    let _e31: Motor = other_1065;
    let _e34: Motor = other_1065;
    return Plane((((vec3<f32>(_e4.g0_.z) * vec3<f32>(_e8.g0_.z, _e11.g0_.y, _e14.g0_.z)) * vec3<f32>(-(1.0), 1.0, 0.0)) + ((_e25.g0_.yxy * vec3<f32>(_e28.g0_.w, _e31.g0_.x, _e34.g0_.y)) * vec3<f32>(1.0, 0.0, -(1.0)))));
}

fn plane_motor_right_contraction(self_1260: Plane, other_1066: Motor) -> Plane {
    var self_1261: Plane;
    var other_1067: Motor;

    self_1261 = self_1260;
    other_1067 = other_1066;
    let _e4: Plane = self_1261;
    let _e6: Motor = other_1067;
    return Plane((_e4.g0_ * vec3<f32>(_e6.g0_.x)));
}

fn plane_motor_right_anti_contraction(self_1262: Plane, other_1068: Motor) -> Point {
    var self_1263: Plane;
    var other_1069: Motor;

    self_1263 = self_1262;
    other_1069 = other_1068;
    let _e4: Plane = self_1263;
    let _e8: Motor = other_1069;
    let _e19: Plane = self_1263;
    let _e22: Motor = other_1069;
    let _e25: Motor = other_1069;
    let _e28: Motor = other_1069;
    return Point((((vec3<f32>(_e4.g0_.z) * vec3<f32>(_e8.g0_.z)) * vec3<f32>(-(1.0), 0.0, 0.0)) + ((_e19.g0_.yxx * vec3<f32>(_e22.g0_.w, _e25.g0_.w, _e28.g0_.z)) * vec3<f32>(1.0, -(1.0), 1.0))));
}

fn plane_motor_dual_add(self_1264: Plane, other_1070: MotorDual) -> MotorDual {
    var self_1265: Plane;
    var other_1071: MotorDual;

    self_1265 = self_1264;
    other_1071 = other_1070;
    let _e4: Plane = self_1265;
    let _e7: Plane = self_1265;
    let _e10: Plane = self_1265;
    let _e13: Plane = self_1265;
    let _e23: MotorDual = other_1071;
    return MotorDual(((vec4<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.y, _e13.g0_.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0)) + _e23.g0_));
}

fn plane_motor_dual_sub(self_1266: Plane, other_1072: MotorDual) -> MotorDual {
    var self_1267: Plane;
    var other_1073: MotorDual;

    self_1267 = self_1266;
    other_1073 = other_1072;
    let _e4: Plane = self_1267;
    let _e7: Plane = self_1267;
    let _e10: Plane = self_1267;
    let _e13: Plane = self_1267;
    let _e23: MotorDual = other_1073;
    return MotorDual(((vec4<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.y, _e13.g0_.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0)) - _e23.g0_));
}

fn plane_motor_dual_geometric_product(self_1268: Plane, other_1074: MotorDual) -> Motor {
    var self_1269: Plane;
    var other_1075: MotorDual;

    self_1269 = self_1268;
    other_1075 = other_1074;
    let _e4: Plane = self_1269;
    let _e8: MotorDual = other_1075;
    let _e19: Plane = self_1269;
    let _e23: MotorDual = other_1075;
    let _e35: Plane = self_1269;
    let _e39: MotorDual = other_1075;
    return Motor(((((vec4<f32>(_e4.g0_.y) * _e8.g0_.zwxy) * vec4<f32>(1.0, -(1.0), 1.0, 1.0)) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.wzyx) * vec4<f32>(1.0, 1.0, -(1.0), 1.0))) + ((vec4<f32>(_e35.g0_.x) * _e39.g0_.xxwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))));
}

fn plane_motor_dual_regressive_product(self_1270: Plane, other_1076: MotorDual) -> Plane {
    var self_1271: Plane;
    var other_1077: MotorDual;

    self_1271 = self_1270;
    other_1077 = other_1076;
    let _e4: Plane = self_1271;
    let _e6: MotorDual = other_1077;
    return Plane((_e4.g0_ * vec3<f32>(_e6.g0_.x)));
}

fn plane_motor_dual_outer_product(self_1272: Plane, other_1078: MotorDual) -> Point {
    var self_1273: Plane;
    var other_1079: MotorDual;

    self_1273 = self_1272;
    other_1079 = other_1078;
    let _e4: Plane = self_1273;
    let _e8: MotorDual = other_1079;
    let _e11: MotorDual = other_1079;
    let _e14: MotorDual = other_1079;
    let _e25: Plane = self_1273;
    let _e29: MotorDual = other_1079;
    let _e32: MotorDual = other_1079;
    let _e35: MotorDual = other_1079;
    let _e47: Plane = self_1273;
    let _e51: MotorDual = other_1079;
    let _e54: MotorDual = other_1079;
    let _e57: MotorDual = other_1079;
    return Point(((((vec3<f32>(_e4.g0_.y) * vec3<f32>(_e8.g0_.w, _e11.g0_.w, _e14.g0_.y)) * vec3<f32>(-(1.0), 0.0, 1.0)) + ((vec3<f32>(_e25.g0_.z) * vec3<f32>(_e29.g0_.z, _e32.g0_.y, _e35.g0_.z)) * vec3<f32>(1.0, -(1.0), 0.0))) + ((vec3<f32>(_e47.g0_.x) * vec3<f32>(_e51.g0_.x, _e54.g0_.w, _e57.g0_.z)) * vec3<f32>(0.0, 1.0, -(1.0)))));
}

fn plane_motor_dual_inner_product(self_1274: Plane, other_1080: MotorDual) -> Translator {
    var self_1275: Plane;
    var other_1081: MotorDual;

    self_1275 = self_1274;
    other_1081 = other_1080;
    let _e4: Plane = self_1275;
    let _e8: MotorDual = other_1081;
    let _e11: MotorDual = other_1081;
    let _e14: MotorDual = other_1081;
    let _e24: Plane = self_1275;
    let _e27: MotorDual = other_1081;
    let _e30: MotorDual = other_1081;
    let _e33: MotorDual = other_1081;
    return Translator((((vec3<f32>(_e4.g0_.z) * vec3<f32>(_e8.g0_.w, _e11.g0_.w, _e14.g0_.x)) * vec3<f32>(1.0, 0.0, 1.0)) + ((_e24.g0_.yyx * vec3<f32>(_e27.g0_.z, _e30.g0_.x, _e33.g0_.x)) * vec3<f32>(1.0, 1.0, 0.0))));
}

fn plane_motor_dual_geometric_anti_product(self_1276: Plane, other_1082: MotorDual) -> MotorDual {
    var self_1277: Plane;
    var other_1083: MotorDual;

    self_1277 = self_1276;
    other_1083 = other_1082;
    let _e4: Plane = self_1277;
    let _e8: MotorDual = other_1083;
    let _e20: Plane = self_1277;
    let _e24: MotorDual = other_1083;
    let _e35: Plane = self_1277;
    let _e38: Plane = self_1277;
    let _e41: Plane = self_1277;
    let _e44: Plane = self_1277;
    let _e48: MotorDual = other_1083;
    return MotorDual(((((vec4<f32>(_e4.g0_.x) * _e8.g0_.yxwz) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0)) + ((vec4<f32>(_e20.g0_.z) * _e24.g0_.yyyx) * vec4<f32>(0.0, 0.0, 1.0, 1.0))) + ((vec4<f32>(_e35.g0_.x, _e38.g0_.x, _e41.g0_.y, _e44.g0_.y) * _e48.g0_.xxxy) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))));
}

fn plane_motor_dual_inner_anti_product(self_1278: Plane, other_1084: MotorDual) -> MotorDual {
    var self_1279: Plane;
    var other_1085: MotorDual;

    self_1279 = self_1278;
    other_1085 = other_1084;
    let _e4: Plane = self_1279;
    let _e7: Plane = self_1279;
    let _e10: Plane = self_1279;
    let _e13: Plane = self_1279;
    let _e17: MotorDual = other_1085;
    return MotorDual(((vec4<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.y, _e13.g0_.z) * _e17.g0_.yxxx) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0)));
}

fn plane_motor_dual_left_contraction(self_1280: Plane, other_1086: MotorDual) -> Translator {
    var self_1281: Plane;
    var other_1087: MotorDual;

    self_1281 = self_1280;
    other_1087 = other_1086;
    let _e4: Plane = self_1281;
    let _e8: MotorDual = other_1087;
    let _e11: MotorDual = other_1087;
    let _e14: MotorDual = other_1087;
    let _e24: Plane = self_1281;
    let _e27: MotorDual = other_1087;
    let _e30: MotorDual = other_1087;
    let _e33: MotorDual = other_1087;
    return Translator((((vec3<f32>(_e4.g0_.z) * vec3<f32>(_e8.g0_.w, _e11.g0_.w, _e14.g0_.x)) * vec3<f32>(1.0, 0.0, 1.0)) + ((_e24.g0_.yyx * vec3<f32>(_e27.g0_.z, _e30.g0_.x, _e33.g0_.x)) * vec3<f32>(1.0, 1.0, 0.0))));
}

fn plane_motor_dual_right_contraction(self_1282: Plane, other_1088: MotorDual) -> Scalar {
    var self_1283: Plane;
    var other_1089: MotorDual;

    self_1283 = self_1282;
    other_1089 = other_1088;
    let _e4: Plane = self_1283;
    let _e7: MotorDual = other_1089;
    let _e11: Plane = self_1283;
    let _e14: MotorDual = other_1089;
    return Scalar(((_e4.g0_.y * _e7.g0_.z) + (_e11.g0_.z * _e14.g0_.w)));
}

fn plane_motor_dual_left_anti_contraction(self_1284: Plane, other_1090: MotorDual) -> AntiScalar {
    var self_1285: Plane;
    var other_1091: MotorDual;

    self_1285 = self_1284;
    other_1091 = other_1090;
    let _e5: Plane = self_1285;
    let _e8: MotorDual = other_1091;
    return AntiScalar((0.0 - (_e5.g0_.x * _e8.g0_.y)));
}

fn plane_motor_dual_right_anti_contraction(self_1286: Plane, other_1092: MotorDual) -> MotorDual {
    var self_1287: Plane;
    var other_1093: MotorDual;

    self_1287 = self_1286;
    other_1093 = other_1092;
    let _e4: Plane = self_1287;
    let _e7: Plane = self_1287;
    let _e10: Plane = self_1287;
    let _e13: Plane = self_1287;
    let _e17: MotorDual = other_1093;
    return MotorDual(((vec4<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.y, _e13.g0_.z) * _e17.g0_.yxxx) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0)));
}

fn plane_motor_dual_scalar_product(self_1288: Plane, other_1094: MotorDual) -> Scalar {
    var self_1289: Plane;
    var other_1095: MotorDual;

    self_1289 = self_1288;
    other_1095 = other_1094;
    let _e4: Plane = self_1289;
    let _e7: MotorDual = other_1095;
    let _e11: Plane = self_1289;
    let _e14: MotorDual = other_1095;
    return Scalar(((_e4.g0_.y * _e7.g0_.z) + (_e11.g0_.z * _e14.g0_.w)));
}

fn plane_motor_dual_anti_scalar_product(self_1290: Plane, other_1096: MotorDual) -> AntiScalar {
    var self_1291: Plane;
    var other_1097: MotorDual;

    self_1291 = self_1290;
    other_1097 = other_1096;
    let _e5: Plane = self_1291;
    let _e8: MotorDual = other_1097;
    return AntiScalar((0.0 - (_e5.g0_.x * _e8.g0_.y)));
}

fn plane_squared_magnitude(self_1292: Plane) -> Scalar {
    var self_1293: Plane;

    self_1293 = self_1292;
    let _e2: Plane = self_1293;
    let _e3: Plane = self_1293;
    let _e4: Plane = plane_reversal(_e3);
    let _e5: Scalar = plane_plane_scalar_product(_e2, _e4);
    return _e5;
}

fn plane_magnitude(self_1294: Plane) -> Scalar {
    var self_1295: Plane;

    self_1295 = self_1294;
    let _e2: Plane = self_1295;
    let _e3: Scalar = plane_squared_magnitude(_e2);
    return Scalar(sqrt(_e3.g0_));
}

fn plane_bulk_norm(self_1296: Plane) -> Scalar {
    var self_1297: Plane;

    self_1297 = self_1296;
    let _e2: Plane = self_1297;
    let _e3: Scalar = plane_squared_magnitude(_e2);
    return Scalar(sqrt(_e3.g0_));
}

fn plane_squared_anti_magnitude(self_1298: Plane) -> AntiScalar {
    var self_1299: Plane;

    self_1299 = self_1298;
    let _e2: Plane = self_1299;
    let _e3: Plane = self_1299;
    let _e4: Plane = plane_anti_reversal(_e3);
    let _e5: AntiScalar = plane_plane_anti_scalar_product(_e2, _e4);
    return _e5;
}

fn plane_weight_norm(self_1300: Plane) -> AntiScalar {
    var self_1301: Plane;

    self_1301 = self_1300;
    let _e2: Plane = self_1301;
    let _e3: AntiScalar = plane_squared_anti_magnitude(_e2);
    return AntiScalar(sqrt(_e3.g0_));
}

fn plane_scale(self_1302: Plane, other_1098: f32) -> Plane {
    var self_1303: Plane;
    var other_1099: f32;

    self_1303 = self_1302;
    other_1099 = other_1098;
    let _e4: Plane = self_1303;
    let _e5: f32 = other_1099;
    let _e7: Plane = plane_scalar_geometric_product(_e4, Scalar(_e5));
    return _e7;
}

fn plane_signum(self_1304: Plane) -> Plane {
    var self_1305: Plane;

    self_1305 = self_1304;
    let _e2: Plane = self_1305;
    let _e3: Plane = self_1305;
    let _e4: Scalar = plane_magnitude(_e3);
    let _e9: Plane = plane_scalar_geometric_product(_e2, Scalar((1.0 / _e4.g0_)));
    return _e9;
}

fn plane_inverse(self_1306: Plane) -> Plane {
    var self_1307: Plane;

    self_1307 = self_1306;
    let _e2: Plane = self_1307;
    let _e3: Plane = plane_reversal(_e2);
    let _e4: Plane = self_1307;
    let _e5: Scalar = plane_squared_magnitude(_e4);
    let _e10: Plane = plane_scalar_geometric_product(_e3, Scalar((1.0 / _e5.g0_)));
    return _e10;
}

fn plane_unitize(self_1308: Plane) -> Plane {
    var self_1309: Plane;

    self_1309 = self_1308;
    let _e2: Plane = self_1309;
    let _e3: Plane = self_1309;
    let _e4: AntiScalar = plane_weight_norm(_e3);
    let _e9: Plane = plane_scalar_geometric_product(_e2, Scalar((1.0 / _e4.g0_)));
    return _e9;
}

fn translator_zero() -> Translator {
    return Translator(vec3<f32>(0.0));
}

fn translator_one() -> Translator {
    return Translator(vec3<f32>(1.0, 0.0, 0.0));
}

fn translator_neg(self_1310: Translator) -> Translator {
    var self_1311: Translator;

    self_1311 = self_1310;
    let _e2: Translator = self_1311;
    return Translator((_e2.g0_ * vec3<f32>(-(1.0))));
}

fn translator_automorphism(self_1312: Translator) -> Translator {
    var self_1313: Translator;

    self_1313 = self_1312;
    let _e2: Translator = self_1313;
    return Translator(_e2.g0_);
}

fn translator_reversal(self_1314: Translator) -> Translator {
    var self_1315: Translator;

    self_1315 = self_1314;
    let _e2: Translator = self_1315;
    return Translator((_e2.g0_ * vec3<f32>(1.0, -(1.0), -(1.0))));
}

fn translator_conjugation(self_1316: Translator) -> Translator {
    var self_1317: Translator;

    self_1317 = self_1316;
    let _e2: Translator = self_1317;
    return Translator((_e2.g0_ * vec3<f32>(1.0, -(1.0), -(1.0))));
}

fn translator_anti_reversal(self_1318: Translator) -> Translator {
    var self_1319: Translator;

    self_1319 = self_1318;
    let _e2: Translator = self_1319;
    return Translator((_e2.g0_ * vec3<f32>(1.0, -(1.0), -(1.0))));
}

fn translator_double_complement(self_1320: Translator) -> Translator {
    var self_1321: Translator;

    self_1321 = self_1320;
    let _e2: Translator = self_1321;
    return Translator(_e2.g0_);
}

fn translator_scalar_into(self_1322: Translator) -> Scalar {
    var self_1323: Translator;

    self_1323 = self_1322;
    let _e2: Translator = self_1323;
    return Scalar(_e2.g0_.x);
}

fn translator_scalar_add(self_1324: Translator, other_1100: Scalar) -> Translator {
    var self_1325: Translator;
    var other_1101: Scalar;

    self_1325 = self_1324;
    other_1101 = other_1100;
    let _e4: Translator = self_1325;
    let _e6: Scalar = other_1101;
    return Translator((_e4.g0_ + (vec3<f32>(_e6.g0_) * vec3<f32>(1.0, 0.0, 0.0))));
}

fn translator_scalar_sub(self_1326: Translator, other_1102: Scalar) -> Translator {
    var self_1327: Translator;
    var other_1103: Scalar;

    self_1327 = self_1326;
    other_1103 = other_1102;
    let _e4: Translator = self_1327;
    let _e6: Scalar = other_1103;
    return Translator((_e4.g0_ - (vec3<f32>(_e6.g0_) * vec3<f32>(1.0, 0.0, 0.0))));
}

fn translator_scalar_geometric_product(self_1328: Translator, other_1104: Scalar) -> Translator {
    var self_1329: Translator;
    var other_1105: Scalar;

    self_1329 = self_1328;
    other_1105 = other_1104;
    let _e4: Translator = self_1329;
    let _e6: Scalar = other_1105;
    return Translator((_e4.g0_ * vec3<f32>(_e6.g0_)));
}

fn translator_scalar_outer_product(self_1330: Translator, other_1106: Scalar) -> Translator {
    var self_1331: Translator;
    var other_1107: Scalar;

    self_1331 = self_1330;
    other_1107 = other_1106;
    let _e4: Translator = self_1331;
    let _e6: Scalar = other_1107;
    return Translator((_e4.g0_ * vec3<f32>(_e6.g0_)));
}

fn translator_scalar_inner_product(self_1332: Translator, other_1108: Scalar) -> Translator {
    var self_1333: Translator;
    var other_1109: Scalar;

    self_1333 = self_1332;
    other_1109 = other_1108;
    let _e4: Translator = self_1333;
    let _e6: Scalar = other_1109;
    return Translator((_e4.g0_ * vec3<f32>(_e6.g0_)));
}

fn translator_scalar_left_contraction(self_1334: Translator, other_1110: Scalar) -> Scalar {
    var self_1335: Translator;
    var other_1111: Scalar;

    self_1335 = self_1334;
    other_1111 = other_1110;
    let _e4: Translator = self_1335;
    let _e7: Scalar = other_1111;
    return Scalar((_e4.g0_.x * _e7.g0_));
}

fn translator_scalar_right_contraction(self_1336: Translator, other_1112: Scalar) -> Translator {
    var self_1337: Translator;
    var other_1113: Scalar;

    self_1337 = self_1336;
    other_1113 = other_1112;
    let _e4: Translator = self_1337;
    let _e6: Scalar = other_1113;
    return Translator((_e4.g0_ * vec3<f32>(_e6.g0_)));
}

fn translator_scalar_scalar_product(self_1338: Translator, other_1114: Scalar) -> Scalar {
    var self_1339: Translator;
    var other_1115: Scalar;

    self_1339 = self_1338;
    other_1115 = other_1114;
    let _e4: Translator = self_1339;
    let _e7: Scalar = other_1115;
    return Scalar((_e4.g0_.x * _e7.g0_));
}

fn translator_anti_scalar_geometric_product(self_1340: Translator, other_1116: AntiScalar) -> AntiScalar {
    var self_1341: Translator;
    var other_1117: AntiScalar;

    self_1341 = self_1340;
    other_1117 = other_1116;
    let _e4: Translator = self_1341;
    let _e7: AntiScalar = other_1117;
    return AntiScalar((_e4.g0_.x * _e7.g0_));
}

fn translator_anti_scalar_regressive_product(self_1342: Translator, other_1118: AntiScalar) -> Translator {
    var self_1343: Translator;
    var other_1119: AntiScalar;

    self_1343 = self_1342;
    other_1119 = other_1118;
    let _e4: Translator = self_1343;
    let _e6: AntiScalar = other_1119;
    return Translator((_e4.g0_ * vec3<f32>(_e6.g0_)));
}

fn translator_anti_scalar_outer_product(self_1344: Translator, other_1120: AntiScalar) -> AntiScalar {
    var self_1345: Translator;
    var other_1121: AntiScalar;

    self_1345 = self_1344;
    other_1121 = other_1120;
    let _e4: Translator = self_1345;
    let _e7: AntiScalar = other_1121;
    return AntiScalar((_e4.g0_.x * _e7.g0_));
}

fn translator_anti_scalar_inner_product(self_1346: Translator, other_1122: AntiScalar) -> AntiScalar {
    var self_1347: Translator;
    var other_1123: AntiScalar;

    self_1347 = self_1346;
    other_1123 = other_1122;
    let _e4: Translator = self_1347;
    let _e7: AntiScalar = other_1123;
    return AntiScalar((_e4.g0_.x * _e7.g0_));
}

fn translator_anti_scalar_geometric_anti_product(self_1348: Translator, other_1124: AntiScalar) -> Translator {
    var self_1349: Translator;
    var other_1125: AntiScalar;

    self_1349 = self_1348;
    other_1125 = other_1124;
    let _e4: Translator = self_1349;
    let _e6: AntiScalar = other_1125;
    return Translator((_e4.g0_ * vec3<f32>(_e6.g0_)));
}

fn translator_anti_scalar_inner_anti_product(self_1350: Translator, other_1126: AntiScalar) -> Translator {
    var self_1351: Translator;
    var other_1127: AntiScalar;

    self_1351 = self_1350;
    other_1127 = other_1126;
    let _e4: Translator = self_1351;
    let _e6: AntiScalar = other_1127;
    return Translator((_e4.g0_ * vec3<f32>(_e6.g0_)));
}

fn translator_anti_scalar_left_contraction(self_1352: Translator, other_1128: AntiScalar) -> AntiScalar {
    var self_1353: Translator;
    var other_1129: AntiScalar;

    self_1353 = self_1352;
    other_1129 = other_1128;
    let _e4: Translator = self_1353;
    let _e7: AntiScalar = other_1129;
    return AntiScalar((_e4.g0_.x * _e7.g0_));
}

fn translator_anti_scalar_right_anti_contraction(self_1354: Translator, other_1130: AntiScalar) -> Translator {
    var self_1355: Translator;
    var other_1131: AntiScalar;

    self_1355 = self_1354;
    other_1131 = other_1130;
    let _e4: Translator = self_1355;
    let _e6: AntiScalar = other_1131;
    return Translator((_e4.g0_ * vec3<f32>(_e6.g0_)));
}

fn translator_multi_vector_add(self_1356: Translator, other_1132: MultiVector) -> MultiVector {
    var self_1357: Translator;
    var other_1133: MultiVector;

    self_1357 = self_1356;
    other_1133 = other_1132;
    let _e4: Translator = self_1357;
    let _e14: MultiVector = other_1133;
    let _e17: Translator = self_1357;
    let _e20: Translator = self_1357;
    let _e23: Translator = self_1357;
    let _e26: Translator = self_1357;
    let _e36: MultiVector = other_1133;
    return MultiVector(((vec4<f32>(_e4.g0_.x) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) + _e14.g0_), ((vec4<f32>(_e17.g0_.x, _e20.g0_.x, _e23.g0_.y, _e26.g0_.z) * vec4<f32>(0.0, 0.0, 1.0, 1.0)) + _e36.g1_));
}

fn translator_multi_vector_sub(self_1358: Translator, other_1134: MultiVector) -> MultiVector {
    var self_1359: Translator;
    var other_1135: MultiVector;

    self_1359 = self_1358;
    other_1135 = other_1134;
    let _e4: Translator = self_1359;
    let _e14: MultiVector = other_1135;
    let _e17: Translator = self_1359;
    let _e20: Translator = self_1359;
    let _e23: Translator = self_1359;
    let _e26: Translator = self_1359;
    let _e36: MultiVector = other_1135;
    return MultiVector(((vec4<f32>(_e4.g0_.x) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) - _e14.g0_), ((vec4<f32>(_e17.g0_.x, _e20.g0_.x, _e23.g0_.y, _e26.g0_.z) * vec4<f32>(0.0, 0.0, 1.0, 1.0)) - _e36.g1_));
}

fn translator_multi_vector_geometric_product(self_1360: Translator, other_1136: MultiVector) -> MultiVector {
    var self_1361: Translator;
    var other_1137: MultiVector;

    self_1361 = self_1360;
    other_1137 = other_1136;
    let _e4: Translator = self_1361;
    let _e8: MultiVector = other_1137;
    let _e11: Translator = self_1361;
    let _e15: MultiVector = other_1137;
    let _e18: Translator = self_1361;
    let _e22: MultiVector = other_1137;
    let _e34: Translator = self_1361;
    let _e38: MultiVector = other_1137;
    return MultiVector((vec4<f32>(_e4.g0_.x) * _e8.g0_), (((vec4<f32>(_e11.g0_.x) * _e15.g1_) + ((vec4<f32>(_e18.g0_.y) * _e22.g0_.zwxy) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e34.g0_.z) * _e38.g0_.wzyx) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0))));
}

fn translator_multi_vector_outer_product(self_1362: Translator, other_1138: MultiVector) -> MultiVector {
    var self_1363: Translator;
    var other_1139: MultiVector;

    self_1363 = self_1362;
    other_1139 = other_1138;
    let _e4: Translator = self_1363;
    let _e8: MultiVector = other_1139;
    let _e11: Translator = self_1363;
    let _e15: MultiVector = other_1139;
    let _e18: Translator = self_1363;
    let _e22: MultiVector = other_1139;
    let _e33: Translator = self_1363;
    let _e36: Translator = self_1363;
    let _e39: Translator = self_1363;
    let _e42: Translator = self_1363;
    let _e46: MultiVector = other_1139;
    return MultiVector((vec4<f32>(_e4.g0_.x) * _e8.g0_), (((vec4<f32>(_e11.g0_.x) * _e15.g1_) + ((vec4<f32>(_e18.g0_.z) * _e22.g0_.zzzx) * vec4<f32>(0.0, 1.0, 0.0, 1.0))) + ((vec4<f32>(_e33.g0_.x, _e36.g0_.y, _e39.g0_.y, _e42.g0_.x) * _e46.g0_.xwxx) * vec4<f32>(0.0, 1.0, 1.0, 0.0))));
}

fn translator_multi_vector_inner_product(self_1364: Translator, other_1140: MultiVector) -> MultiVector {
    var self_1365: Translator;
    var other_1141: MultiVector;

    self_1365 = self_1364;
    other_1141 = other_1140;
    let _e4: Translator = self_1365;
    let _e8: MultiVector = other_1141;
    let _e11: Translator = self_1365;
    let _e15: MultiVector = other_1141;
    let _e18: Translator = self_1365;
    let _e22: MultiVector = other_1141;
    let _e34: Translator = self_1365;
    let _e37: Translator = self_1365;
    let _e40: Translator = self_1365;
    let _e43: Translator = self_1365;
    let _e47: MultiVector = other_1141;
    return MultiVector((vec4<f32>(_e4.g0_.x) * _e8.g0_), (((vec4<f32>(_e11.g0_.x) * _e15.g1_) + ((vec4<f32>(_e18.g0_.z) * _e22.g0_.wwwx) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))) + ((vec4<f32>(_e34.g0_.y, _e37.g0_.x, _e40.g0_.y, _e43.g0_.x) * _e47.g0_.zxxx) * vec4<f32>(1.0, 0.0, 1.0, 0.0))));
}

fn translator_multi_vector_geometric_anti_product(self_1366: Translator, other_1142: MultiVector) -> MultiVector {
    var self_1367: Translator;
    var other_1143: MultiVector;

    self_1367 = self_1366;
    other_1143 = other_1142;
    let _e4: Translator = self_1367;
    let _e8: MultiVector = other_1143;
    let _e19: Translator = self_1367;
    let _e23: MultiVector = other_1143;
    let _e28: Translator = self_1367;
    let _e32: MultiVector = other_1143;
    let _e45: Translator = self_1367;
    let _e49: MultiVector = other_1143;
    let _e61: Translator = self_1367;
    let _e65: MultiVector = other_1143;
    return MultiVector(((((vec4<f32>(_e4.g0_.x) * _e8.g1_.yxwz) * vec4<f32>(1.0, -(1.0), 1.0, 1.0)) + (vec4<f32>(_e19.g0_.y) * _e23.g0_.wzyx)) + ((vec4<f32>(_e28.g0_.z) * _e32.g0_.zwxy) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))), (((vec4<f32>(_e45.g0_.y) * _e49.g1_.wzyx) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0))) + (vec4<f32>(_e61.g0_.z) * _e65.g1_.zwxy)));
}

fn translator_multi_vector_left_contraction(self_1368: Translator, other_1144: MultiVector) -> MultiVector {
    var self_1369: Translator;
    var other_1145: MultiVector;

    self_1369 = self_1368;
    other_1145 = other_1144;
    let _e4: Translator = self_1369;
    let _e8: MultiVector = other_1145;
    let _e11: Translator = self_1369;
    let _e15: MultiVector = other_1145;
    return MultiVector((vec4<f32>(_e4.g0_.x) * _e8.g0_), (vec4<f32>(_e11.g0_.x) * _e15.g1_));
}

fn translator_multi_vector_scalar_product(self_1370: Translator, other_1146: MultiVector) -> Scalar {
    var self_1371: Translator;
    var other_1147: MultiVector;

    self_1371 = self_1370;
    other_1147 = other_1146;
    let _e4: Translator = self_1371;
    let _e7: MultiVector = other_1147;
    return Scalar((_e4.g0_.x * _e7.g0_.x));
}

fn translator_multi_vector_anti_scalar_product(self_1372: Translator, other_1148: MultiVector) -> AntiScalar {
    var self_1373: Translator;
    var other_1149: MultiVector;

    self_1373 = self_1372;
    other_1149 = other_1148;
    let _e4: Translator = self_1373;
    let _e7: MultiVector = other_1149;
    let _e11: Translator = self_1373;
    let _e14: MultiVector = other_1149;
    return AntiScalar(((_e4.g0_.y * _e7.g1_.z) + (_e11.g0_.z * _e14.g1_.w)));
}

fn translator_rotor_add(self_1374: Translator, other_1150: Rotor) -> Motor {
    var self_1375: Translator;
    var other_1151: Rotor;

    self_1375 = self_1374;
    other_1151 = other_1150;
    let _e4: Translator = self_1375;
    let _e7: Translator = self_1375;
    let _e10: Translator = self_1375;
    let _e13: Translator = self_1375;
    let _e23: Rotor = other_1151;
    let _e26: Rotor = other_1151;
    let _e29: Rotor = other_1151;
    let _e32: Rotor = other_1151;
    return Motor(((vec4<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.y, _e13.g0_.z) * vec4<f32>(1.0, 0.0, 1.0, 1.0)) + (vec4<f32>(_e23.g0_.x, _e26.g0_.y, _e29.g0_.x, _e32.g0_.x) * vec4<f32>(1.0, 1.0, 0.0, 0.0))));
}

fn translator_rotor_sub(self_1376: Translator, other_1152: Rotor) -> Motor {
    var self_1377: Translator;
    var other_1153: Rotor;

    self_1377 = self_1376;
    other_1153 = other_1152;
    let _e4: Translator = self_1377;
    let _e7: Translator = self_1377;
    let _e10: Translator = self_1377;
    let _e13: Translator = self_1377;
    let _e23: Rotor = other_1153;
    let _e26: Rotor = other_1153;
    let _e29: Rotor = other_1153;
    let _e32: Rotor = other_1153;
    return Motor(((vec4<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.y, _e13.g0_.z) * vec4<f32>(1.0, 0.0, 1.0, 1.0)) - (vec4<f32>(_e23.g0_.x, _e26.g0_.y, _e29.g0_.x, _e32.g0_.x) * vec4<f32>(1.0, 1.0, 0.0, 0.0))));
}

fn translator_rotor_geometric_product(self_1378: Translator, other_1154: Rotor) -> Motor {
    var self_1379: Translator;
    var other_1155: Rotor;

    self_1379 = self_1378;
    other_1155 = other_1154;
    let _e4: Translator = self_1379;
    let _e8: Rotor = other_1155;
    let _e11: Rotor = other_1155;
    let _e14: Rotor = other_1155;
    let _e17: Rotor = other_1155;
    let _e28: Translator = self_1379;
    let _e31: Translator = self_1379;
    let _e34: Translator = self_1379;
    let _e37: Translator = self_1379;
    let _e41: Rotor = other_1155;
    let _e44: Rotor = other_1155;
    let _e47: Rotor = other_1155;
    let _e50: Rotor = other_1155;
    return Motor((((vec4<f32>(_e4.g0_.z) * vec4<f32>(_e8.g0_.y, _e11.g0_.y, _e14.g0_.y, _e17.g0_.x)) * vec4<f32>(0.0, 0.0, 1.0, 1.0)) + ((vec4<f32>(_e28.g0_.x, _e31.g0_.x, _e34.g0_.y, _e37.g0_.y) * vec4<f32>(_e41.g0_.x, _e44.g0_.y, _e47.g0_.x, _e50.g0_.y)) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))));
}

fn translator_rotor_outer_product(self_1380: Translator, other_1156: Rotor) -> Motor {
    var self_1381: Translator;
    var other_1157: Rotor;

    self_1381 = self_1380;
    other_1157 = other_1156;
    let _e4: Translator = self_1381;
    let _e7: Translator = self_1381;
    let _e10: Translator = self_1381;
    let _e13: Translator = self_1381;
    let _e17: Rotor = other_1157;
    let _e20: Rotor = other_1157;
    let _e23: Rotor = other_1157;
    let _e26: Rotor = other_1157;
    return Motor((vec4<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.y, _e13.g0_.z) * vec4<f32>(_e17.g0_.x, _e20.g0_.y, _e23.g0_.x, _e26.g0_.x)));
}

fn translator_rotor_inner_product(self_1382: Translator, other_1158: Rotor) -> Motor {
    var self_1383: Translator;
    var other_1159: Rotor;

    self_1383 = self_1382;
    other_1159 = other_1158;
    let _e4: Translator = self_1383;
    let _e7: Translator = self_1383;
    let _e10: Translator = self_1383;
    let _e13: Translator = self_1383;
    let _e17: Rotor = other_1159;
    let _e20: Rotor = other_1159;
    let _e23: Rotor = other_1159;
    let _e26: Rotor = other_1159;
    return Motor((vec4<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.y, _e13.g0_.z) * vec4<f32>(_e17.g0_.x, _e20.g0_.y, _e23.g0_.x, _e26.g0_.x)));
}

fn translator_rotor_left_contraction(self_1384: Translator, other_1160: Rotor) -> Rotor {
    var self_1385: Translator;
    var other_1161: Rotor;

    self_1385 = self_1384;
    other_1161 = other_1160;
    let _e4: Translator = self_1385;
    let _e8: Rotor = other_1161;
    return Rotor((vec2<f32>(_e4.g0_.x) * _e8.g0_));
}

fn translator_rotor_right_contraction(self_1386: Translator, other_1162: Rotor) -> Translator {
    var self_1387: Translator;
    var other_1163: Rotor;

    self_1387 = self_1386;
    other_1163 = other_1162;
    let _e4: Translator = self_1387;
    let _e6: Rotor = other_1163;
    return Translator((_e4.g0_ * vec3<f32>(_e6.g0_.x)));
}

fn translator_rotor_scalar_product(self_1388: Translator, other_1164: Rotor) -> Scalar {
    var self_1389: Translator;
    var other_1165: Rotor;

    self_1389 = self_1388;
    other_1165 = other_1164;
    let _e4: Translator = self_1389;
    let _e7: Rotor = other_1165;
    return Scalar((_e4.g0_.x * _e7.g0_.x));
}

fn translator_point_add(self_1390: Translator, other_1166: Point) -> Motor {
    var self_1391: Translator;
    var other_1167: Point;

    self_1391 = self_1390;
    other_1167 = other_1166;
    let _e4: Translator = self_1391;
    let _e7: Translator = self_1391;
    let _e10: Translator = self_1391;
    let _e13: Translator = self_1391;
    let _e23: Point = other_1167;
    let _e26: Point = other_1167;
    let _e29: Point = other_1167;
    let _e32: Point = other_1167;
    return Motor(((vec4<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.y, _e13.g0_.z) * vec4<f32>(1.0, 0.0, 1.0, 1.0)) + (vec4<f32>(_e23.g0_.x, _e26.g0_.x, _e29.g0_.y, _e32.g0_.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn translator_point_sub(self_1392: Translator, other_1168: Point) -> Motor {
    var self_1393: Translator;
    var other_1169: Point;

    self_1393 = self_1392;
    other_1169 = other_1168;
    let _e4: Translator = self_1393;
    let _e7: Translator = self_1393;
    let _e10: Translator = self_1393;
    let _e13: Translator = self_1393;
    let _e23: Point = other_1169;
    let _e26: Point = other_1169;
    let _e29: Point = other_1169;
    let _e32: Point = other_1169;
    return Motor(((vec4<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.y, _e13.g0_.z) * vec4<f32>(1.0, 0.0, 1.0, 1.0)) - (vec4<f32>(_e23.g0_.x, _e26.g0_.x, _e29.g0_.y, _e32.g0_.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn translator_point_geometric_product(self_1394: Translator, other_1170: Point) -> Point {
    var self_1395: Translator;
    var other_1171: Point;

    self_1395 = self_1394;
    other_1171 = other_1170;
    let _e4: Translator = self_1395;
    let _e8: Point = other_1171;
    let _e11: Translator = self_1395;
    let _e14: Point = other_1171;
    return Point(((vec3<f32>(_e4.g0_.x) * _e8.g0_) + ((_e11.g0_.xzy * vec3<f32>(_e14.g0_.x)) * vec3<f32>(0.0, 1.0, -(1.0)))));
}

fn translator_point_regressive_product(self_1396: Translator, other_1172: Point) -> Plane {
    var self_1397: Translator;
    var other_1173: Point;

    self_1397 = self_1396;
    other_1173 = other_1172;
    let _e4: Translator = self_1397;
    let _e8: Point = other_1173;
    let _e18: Translator = self_1397;
    let _e21: Point = other_1173;
    return Plane((((vec3<f32>(_e4.g0_.z) * _e8.g0_.yxy) * vec3<f32>(1.0, -(1.0), 0.0)) + ((_e18.g0_.yxy * _e21.g0_.zxx) * vec3<f32>(-(1.0), 0.0, 1.0))));
}

fn translator_point_outer_product(self_1398: Translator, other_1174: Point) -> Point {
    var self_1399: Translator;
    var other_1175: Point;

    self_1399 = self_1398;
    other_1175 = other_1174;
    let _e4: Translator = self_1399;
    let _e8: Point = other_1175;
    return Point((vec3<f32>(_e4.g0_.x) * _e8.g0_));
}

fn translator_point_inner_product(self_1400: Translator, other_1176: Point) -> Point {
    var self_1401: Translator;
    var other_1177: Point;

    self_1401 = self_1400;
    other_1177 = other_1176;
    let _e4: Translator = self_1401;
    let _e8: Point = other_1177;
    return Point((vec3<f32>(_e4.g0_.x) * _e8.g0_));
}

fn translator_point_geometric_anti_product(self_1402: Translator, other_1178: Point) -> MotorDual {
    var self_1403: Translator;
    var other_1179: Point;

    self_1403 = self_1402;
    other_1179 = other_1178;
    let _e4: Translator = self_1403;
    let _e8: Point = other_1179;
    let _e11: Point = other_1179;
    let _e14: Point = other_1179;
    let _e17: Point = other_1179;
    let _e29: Translator = self_1403;
    let _e33: Point = other_1179;
    let _e36: Point = other_1179;
    let _e39: Point = other_1179;
    let _e42: Point = other_1179;
    let _e55: Translator = self_1403;
    let _e59: Point = other_1179;
    let _e62: Point = other_1179;
    let _e65: Point = other_1179;
    let _e68: Point = other_1179;
    return MotorDual(((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y, _e11.g0_.z, _e14.g0_.y, _e17.g0_.x)) * vec4<f32>(1.0, -(1.0), 0.0, 1.0)) + ((vec4<f32>(_e29.g0_.z) * vec4<f32>(_e33.g0_.z, _e36.g0_.y, _e39.g0_.x, _e42.g0_.z)) * vec4<f32>(1.0, 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e55.g0_.x) * vec4<f32>(_e59.g0_.x, _e62.g0_.x, _e65.g0_.y, _e68.g0_.z)) * vec4<f32>(0.0, 0.0, 1.0, 1.0))));
}

fn translator_point_left_contraction(self_1404: Translator, other_1180: Point) -> Point {
    var self_1405: Translator;
    var other_1181: Point;

    self_1405 = self_1404;
    other_1181 = other_1180;
    let _e4: Translator = self_1405;
    let _e8: Point = other_1181;
    return Point((vec3<f32>(_e4.g0_.x) * _e8.g0_));
}

fn translator_point_left_anti_contraction(self_1406: Translator, other_1182: Point) -> AntiScalar {
    var self_1407: Translator;
    var other_1183: Point;

    self_1407 = self_1406;
    other_1183 = other_1182;
    let _e4: Translator = self_1407;
    let _e7: Point = other_1183;
    let _e11: Translator = self_1407;
    let _e14: Point = other_1183;
    return AntiScalar(((_e4.g0_.y * _e7.g0_.y) + (_e11.g0_.z * _e14.g0_.z)));
}

fn translator_point_anti_scalar_product(self_1408: Translator, other_1184: Point) -> AntiScalar {
    var self_1409: Translator;
    var other_1185: Point;

    self_1409 = self_1408;
    other_1185 = other_1184;
    let _e4: Translator = self_1409;
    let _e7: Point = other_1185;
    let _e11: Translator = self_1409;
    let _e14: Point = other_1185;
    return AntiScalar(((_e4.g0_.y * _e7.g0_.y) + (_e11.g0_.z * _e14.g0_.z)));
}

fn translator_ideal_point_into(self_1410: Translator) -> IdealPoint {
    var self_1411: Translator;

    self_1411 = self_1410;
    let _e2: Translator = self_1411;
    let _e5: Translator = self_1411;
    return IdealPoint(vec2<f32>(_e2.g0_.y, _e5.g0_.z));
}

fn translator_ideal_point_add(self_1412: Translator, other_1186: IdealPoint) -> Translator {
    var self_1413: Translator;
    var other_1187: IdealPoint;

    self_1413 = self_1412;
    other_1187 = other_1186;
    let _e4: Translator = self_1413;
    let _e6: IdealPoint = other_1187;
    let _e9: IdealPoint = other_1187;
    let _e12: IdealPoint = other_1187;
    return Translator((_e4.g0_ + (vec3<f32>(_e6.g0_.x, _e9.g0_.x, _e12.g0_.y) * vec3<f32>(0.0, 1.0, 1.0))));
}

fn translator_ideal_point_sub(self_1414: Translator, other_1188: IdealPoint) -> Translator {
    var self_1415: Translator;
    var other_1189: IdealPoint;

    self_1415 = self_1414;
    other_1189 = other_1188;
    let _e4: Translator = self_1415;
    let _e6: IdealPoint = other_1189;
    let _e9: IdealPoint = other_1189;
    let _e12: IdealPoint = other_1189;
    return Translator((_e4.g0_ - (vec3<f32>(_e6.g0_.x, _e9.g0_.x, _e12.g0_.y) * vec3<f32>(0.0, 1.0, 1.0))));
}

fn translator_ideal_point_geometric_product(self_1416: Translator, other_1190: IdealPoint) -> IdealPoint {
    var self_1417: Translator;
    var other_1191: IdealPoint;

    self_1417 = self_1416;
    other_1191 = other_1190;
    let _e4: Translator = self_1417;
    let _e8: IdealPoint = other_1191;
    return IdealPoint((vec2<f32>(_e4.g0_.x) * _e8.g0_));
}

fn translator_ideal_point_outer_product(self_1418: Translator, other_1192: IdealPoint) -> IdealPoint {
    var self_1419: Translator;
    var other_1193: IdealPoint;

    self_1419 = self_1418;
    other_1193 = other_1192;
    let _e4: Translator = self_1419;
    let _e8: IdealPoint = other_1193;
    return IdealPoint((vec2<f32>(_e4.g0_.x) * _e8.g0_));
}

fn translator_ideal_point_inner_product(self_1420: Translator, other_1194: IdealPoint) -> IdealPoint {
    var self_1421: Translator;
    var other_1195: IdealPoint;

    self_1421 = self_1420;
    other_1195 = other_1194;
    let _e4: Translator = self_1421;
    let _e8: IdealPoint = other_1195;
    return IdealPoint((vec2<f32>(_e4.g0_.x) * _e8.g0_));
}

fn translator_ideal_point_geometric_anti_product(self_1422: Translator, other_1196: IdealPoint) -> MotorDual {
    var self_1423: Translator;
    var other_1197: IdealPoint;

    self_1423 = self_1422;
    other_1197 = other_1196;
    let _e4: Translator = self_1423;
    let _e8: IdealPoint = other_1197;
    let _e11: IdealPoint = other_1197;
    let _e14: IdealPoint = other_1197;
    let _e17: IdealPoint = other_1197;
    let _e28: Translator = self_1423;
    let _e31: Translator = self_1423;
    let _e34: Translator = self_1423;
    let _e37: Translator = self_1423;
    let _e41: IdealPoint = other_1197;
    let _e44: IdealPoint = other_1197;
    let _e47: IdealPoint = other_1197;
    let _e50: IdealPoint = other_1197;
    return MotorDual((((vec4<f32>(_e4.g0_.z) * vec4<f32>(_e8.g0_.y, _e11.g0_.x, _e14.g0_.y, _e17.g0_.y)) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) + ((vec4<f32>(_e28.g0_.y, _e31.g0_.y, _e34.g0_.x, _e37.g0_.x) * vec4<f32>(_e41.g0_.x, _e44.g0_.y, _e47.g0_.x, _e50.g0_.y)) * vec4<f32>(1.0, -(1.0), 1.0, 1.0))));
}

fn translator_ideal_point_left_contraction(self_1424: Translator, other_1198: IdealPoint) -> IdealPoint {
    var self_1425: Translator;
    var other_1199: IdealPoint;

    self_1425 = self_1424;
    other_1199 = other_1198;
    let _e4: Translator = self_1425;
    let _e8: IdealPoint = other_1199;
    return IdealPoint((vec2<f32>(_e4.g0_.x) * _e8.g0_));
}

fn translator_ideal_point_left_anti_contraction(self_1426: Translator, other_1200: IdealPoint) -> AntiScalar {
    var self_1427: Translator;
    var other_1201: IdealPoint;

    self_1427 = self_1426;
    other_1201 = other_1200;
    let _e4: Translator = self_1427;
    let _e7: IdealPoint = other_1201;
    let _e11: Translator = self_1427;
    let _e14: IdealPoint = other_1201;
    return AntiScalar(((_e4.g0_.y * _e7.g0_.x) + (_e11.g0_.z * _e14.g0_.y)));
}

fn translator_ideal_point_anti_scalar_product(self_1428: Translator, other_1202: IdealPoint) -> AntiScalar {
    var self_1429: Translator;
    var other_1203: IdealPoint;

    self_1429 = self_1428;
    other_1203 = other_1202;
    let _e4: Translator = self_1429;
    let _e7: IdealPoint = other_1203;
    let _e11: Translator = self_1429;
    let _e14: IdealPoint = other_1203;
    return AntiScalar(((_e4.g0_.y * _e7.g0_.x) + (_e11.g0_.z * _e14.g0_.y)));
}

fn translator_plane_geometric_product(self_1430: Translator, other_1204: Plane) -> MotorDual {
    var self_1431: Translator;
    var other_1205: Plane;

    self_1431 = self_1430;
    other_1205 = other_1204;
    let _e4: Translator = self_1431;
    let _e8: Plane = other_1205;
    let _e11: Plane = other_1205;
    let _e14: Plane = other_1205;
    let _e17: Plane = other_1205;
    let _e28: Translator = self_1431;
    let _e32: Plane = other_1205;
    let _e35: Plane = other_1205;
    let _e38: Plane = other_1205;
    let _e41: Plane = other_1205;
    let _e54: Translator = self_1431;
    let _e58: Plane = other_1205;
    let _e61: Plane = other_1205;
    let _e64: Plane = other_1205;
    let _e67: Plane = other_1205;
    return MotorDual(((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y, _e11.g0_.z, _e14.g0_.y, _e17.g0_.y)) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) + ((vec4<f32>(_e28.g0_.z) * vec4<f32>(_e32.g0_.z, _e35.g0_.y, _e38.g0_.z, _e41.g0_.z)) * vec4<f32>(1.0, -(1.0), 0.0, 0.0))) + ((vec4<f32>(_e54.g0_.x) * vec4<f32>(_e58.g0_.x, _e61.g0_.x, _e64.g0_.y, _e67.g0_.z)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn translator_plane_regressive_product(self_1432: Translator, other_1206: Plane) -> Scalar {
    var self_1433: Translator;
    var other_1207: Plane;

    self_1433 = self_1432;
    other_1207 = other_1206;
    let _e4: Translator = self_1433;
    let _e7: Plane = other_1207;
    let _e11: Translator = self_1433;
    let _e14: Plane = other_1207;
    return Scalar(((_e4.g0_.y * _e7.g0_.y) + (_e11.g0_.z * _e14.g0_.z)));
}

fn translator_plane_outer_product(self_1434: Translator, other_1208: Plane) -> MotorDual {
    var self_1435: Translator;
    var other_1209: Plane;

    self_1435 = self_1434;
    other_1209 = other_1208;
    let _e4: Translator = self_1435;
    let _e8: Plane = other_1209;
    let _e19: Translator = self_1435;
    let _e22: Translator = self_1435;
    let _e25: Translator = self_1435;
    let _e28: Translator = self_1435;
    let _e32: Plane = other_1209;
    let _e35: Plane = other_1209;
    let _e38: Plane = other_1209;
    let _e41: Plane = other_1209;
    return MotorDual((((vec4<f32>(_e4.g0_.z) * vec4<f32>(_e8.g0_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) + (vec4<f32>(_e19.g0_.y, _e22.g0_.x, _e25.g0_.x, _e28.g0_.x) * vec4<f32>(_e32.g0_.y, _e35.g0_.x, _e38.g0_.y, _e41.g0_.z))));
}

fn translator_plane_inner_product(self_1436: Translator, other_1210: Plane) -> Plane {
    var self_1437: Translator;
    var other_1211: Plane;

    self_1437 = self_1436;
    other_1211 = other_1210;
    let _e4: Translator = self_1437;
    let _e8: Plane = other_1211;
    let _e11: Translator = self_1437;
    let _e15: Plane = other_1211;
    let _e27: Translator = self_1437;
    let _e30: Plane = other_1211;
    return Plane((((vec3<f32>(_e4.g0_.x) * _e8.g0_) + ((vec3<f32>(_e11.g0_.z) * vec3<f32>(_e15.g0_.y)) * vec3<f32>(-(1.0), 0.0, 0.0))) + ((_e27.g0_.yxx * _e30.g0_.zxx) * vec3<f32>(1.0, 0.0, 0.0))));
}

fn translator_plane_geometric_anti_product(self_1438: Translator, other_1212: Plane) -> Motor {
    var self_1439: Translator;
    var other_1213: Plane;

    self_1439 = self_1438;
    other_1213 = other_1212;
    let _e4: Translator = self_1439;
    let _e8: Plane = other_1213;
    let _e11: Plane = other_1213;
    let _e14: Plane = other_1213;
    let _e17: Plane = other_1213;
    let _e29: Translator = self_1439;
    let _e33: Plane = other_1213;
    let _e36: Plane = other_1213;
    let _e39: Plane = other_1213;
    let _e42: Plane = other_1213;
    let _e55: Translator = self_1439;
    let _e59: Plane = other_1213;
    return Motor(((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y, _e11.g0_.z, _e14.g0_.y, _e17.g0_.x)) * vec4<f32>(1.0, 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e29.g0_.z) * vec4<f32>(_e33.g0_.z, _e36.g0_.y, _e39.g0_.x, _e42.g0_.z)) * vec4<f32>(1.0, -(1.0), 1.0, 0.0))) + ((vec4<f32>(_e55.g0_.x) * vec4<f32>(_e59.g0_.x)) * vec4<f32>(0.0, -(1.0), 0.0, 0.0))));
}

fn translator_plane_inner_anti_product(self_1440: Translator, other_1214: Plane) -> Point {
    var self_1441: Translator;
    var other_1215: Plane;

    self_1441 = self_1440;
    other_1215 = other_1214;
    let _e4: Translator = self_1441;
    let _e8: Plane = other_1215;
    let _e18: Translator = self_1441;
    let _e22: Plane = other_1215;
    let _e33: Translator = self_1441;
    let _e37: Plane = other_1215;
    return Point(((((vec3<f32>(_e4.g0_.y) * _e8.g0_.zzx) * vec3<f32>(1.0, 0.0, -(1.0))) + ((vec3<f32>(_e18.g0_.z) * _e22.g0_.yxy) * vec3<f32>(-(1.0), 1.0, 0.0))) + ((vec3<f32>(_e33.g0_.x) * vec3<f32>(_e37.g0_.x)) * vec3<f32>(-(1.0), 0.0, 0.0))));
}

fn translator_plane_left_contraction(self_1442: Translator, other_1216: Plane) -> Plane {
    var self_1443: Translator;
    var other_1217: Plane;

    self_1443 = self_1442;
    other_1217 = other_1216;
    let _e4: Translator = self_1443;
    let _e8: Plane = other_1217;
    return Plane((vec3<f32>(_e4.g0_.x) * _e8.g0_));
}

fn translator_plane_left_anti_contraction(self_1444: Translator, other_1218: Plane) -> Point {
    var self_1445: Translator;
    var other_1219: Plane;

    self_1445 = self_1444;
    other_1219 = other_1218;
    let _e4: Translator = self_1445;
    let _e8: Plane = other_1219;
    let _e18: Translator = self_1445;
    let _e21: Plane = other_1219;
    return Point((((vec3<f32>(_e4.g0_.z) * _e8.g0_.yxy) * vec3<f32>(-(1.0), 1.0, 0.0)) + ((_e18.g0_.yxy * _e21.g0_.zxx) * vec3<f32>(1.0, 0.0, -(1.0)))));
}

fn translator_translator_add(self_1446: Translator, other_1220: Translator) -> Translator {
    var self_1447: Translator;
    var other_1221: Translator;

    self_1447 = self_1446;
    other_1221 = other_1220;
    let _e4: Translator = self_1447;
    let _e6: Translator = other_1221;
    return Translator((_e4.g0_ + _e6.g0_));
}

fn translator_translator_sub(self_1448: Translator, other_1222: Translator) -> Translator {
    var self_1449: Translator;
    var other_1223: Translator;

    self_1449 = self_1448;
    other_1223 = other_1222;
    let _e4: Translator = self_1449;
    let _e6: Translator = other_1223;
    return Translator((_e4.g0_ - _e6.g0_));
}

fn translator_translator_mul(self_1450: Translator, other_1224: Translator) -> Translator {
    var self_1451: Translator;
    var other_1225: Translator;

    self_1451 = self_1450;
    other_1225 = other_1224;
    let _e4: Translator = self_1451;
    let _e6: Translator = other_1225;
    return Translator((_e4.g0_ * _e6.g0_));
}

fn translator_translator_div(self_1452: Translator, other_1226: Translator) -> Translator {
    var self_1453: Translator;
    var other_1227: Translator;

    self_1453 = self_1452;
    other_1227 = other_1226;
    let _e4: Translator = self_1453;
    let _e7: Translator = self_1453;
    let _e10: Translator = self_1453;
    let _e19: Translator = other_1227;
    let _e22: Translator = other_1227;
    let _e25: Translator = other_1227;
    return Translator((((vec3<f32>(_e4.g0_.x, _e7.g0_.y, _e10.g0_.z) * vec3<f32>(1.0, 1.0, 1.0)) / vec3<f32>(_e19.g0_.x, _e22.g0_.y, _e25.g0_.z)) * vec3<f32>(1.0, 1.0, 1.0)));
}

fn translator_translator_geometric_product(self_1454: Translator, other_1228: Translator) -> Translator {
    var self_1455: Translator;
    var other_1229: Translator;

    self_1455 = self_1454;
    other_1229 = other_1228;
    let _e4: Translator = self_1455;
    let _e8: Translator = other_1229;
    let _e11: Translator = self_1455;
    let _e13: Translator = other_1229;
    return Translator(((vec3<f32>(_e4.g0_.x) * _e8.g0_) + ((_e11.g0_ * vec3<f32>(_e13.g0_.x)) * vec3<f32>(0.0, 1.0, 1.0))));
}

fn translator_translator_outer_product(self_1456: Translator, other_1230: Translator) -> Translator {
    var self_1457: Translator;
    var other_1231: Translator;

    self_1457 = self_1456;
    other_1231 = other_1230;
    let _e4: Translator = self_1457;
    let _e8: Translator = other_1231;
    let _e11: Translator = self_1457;
    let _e13: Translator = other_1231;
    return Translator(((vec3<f32>(_e4.g0_.x) * _e8.g0_) + ((_e11.g0_ * vec3<f32>(_e13.g0_.x)) * vec3<f32>(0.0, 1.0, 1.0))));
}

fn translator_translator_inner_product(self_1458: Translator, other_1232: Translator) -> Translator {
    var self_1459: Translator;
    var other_1233: Translator;

    self_1459 = self_1458;
    other_1233 = other_1232;
    let _e4: Translator = self_1459;
    let _e8: Translator = other_1233;
    let _e11: Translator = self_1459;
    let _e13: Translator = other_1233;
    return Translator(((vec3<f32>(_e4.g0_.x) * _e8.g0_) + ((_e11.g0_ * vec3<f32>(_e13.g0_.x)) * vec3<f32>(0.0, 1.0, 1.0))));
}

fn translator_translator_geometric_anti_product(self_1460: Translator, other_1234: Translator) -> MotorDual {
    var self_1461: Translator;
    var other_1235: Translator;

    self_1461 = self_1460;
    other_1235 = other_1234;
    let _e4: Translator = self_1461;
    let _e8: Translator = other_1235;
    let _e11: Translator = other_1235;
    let _e14: Translator = other_1235;
    let _e17: Translator = other_1235;
    let _e29: Translator = self_1461;
    let _e33: Translator = other_1235;
    let _e36: Translator = other_1235;
    let _e39: Translator = other_1235;
    let _e42: Translator = other_1235;
    let _e54: Translator = self_1461;
    let _e58: Translator = other_1235;
    let _e61: Translator = other_1235;
    let _e64: Translator = other_1235;
    let _e67: Translator = other_1235;
    return MotorDual(((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y, _e11.g0_.z, _e14.g0_.x, _e17.g0_.y)) * vec4<f32>(1.0, -(1.0), 1.0, 0.0)) + ((vec4<f32>(_e29.g0_.z) * vec4<f32>(_e33.g0_.z, _e36.g0_.y, _e39.g0_.z, _e42.g0_.x)) * vec4<f32>(1.0, 1.0, 0.0, 1.0))) + ((vec4<f32>(_e54.g0_.x) * vec4<f32>(_e58.g0_.x, _e61.g0_.x, _e64.g0_.y, _e67.g0_.z)) * vec4<f32>(0.0, 0.0, 1.0, 1.0))));
}

fn translator_translator_left_contraction(self_1462: Translator, other_1236: Translator) -> Translator {
    var self_1463: Translator;
    var other_1237: Translator;

    self_1463 = self_1462;
    other_1237 = other_1236;
    let _e4: Translator = self_1463;
    let _e8: Translator = other_1237;
    return Translator((vec3<f32>(_e4.g0_.x) * _e8.g0_));
}

fn translator_translator_right_contraction(self_1464: Translator, other_1238: Translator) -> Translator {
    var self_1465: Translator;
    var other_1239: Translator;

    self_1465 = self_1464;
    other_1239 = other_1238;
    let _e4: Translator = self_1465;
    let _e6: Translator = other_1239;
    return Translator((_e4.g0_ * vec3<f32>(_e6.g0_.x)));
}

fn translator_translator_scalar_product(self_1466: Translator, other_1240: Translator) -> Scalar {
    var self_1467: Translator;
    var other_1241: Translator;

    self_1467 = self_1466;
    other_1241 = other_1240;
    let _e4: Translator = self_1467;
    let _e7: Translator = other_1241;
    return Scalar((_e4.g0_.x * _e7.g0_.x));
}

fn translator_translator_anti_scalar_product(self_1468: Translator, other_1242: Translator) -> AntiScalar {
    var self_1469: Translator;
    var other_1243: Translator;

    self_1469 = self_1468;
    other_1243 = other_1242;
    let _e4: Translator = self_1469;
    let _e7: Translator = other_1243;
    let _e11: Translator = self_1469;
    let _e14: Translator = other_1243;
    return AntiScalar(((_e4.g0_.y * _e7.g0_.y) + (_e11.g0_.z * _e14.g0_.z)));
}

fn translator_motor_add(self_1470: Translator, other_1244: Motor) -> Motor {
    var self_1471: Translator;
    var other_1245: Motor;

    self_1471 = self_1470;
    other_1245 = other_1244;
    let _e4: Translator = self_1471;
    let _e7: Translator = self_1471;
    let _e10: Translator = self_1471;
    let _e13: Translator = self_1471;
    let _e23: Motor = other_1245;
    return Motor(((vec4<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.y, _e13.g0_.z) * vec4<f32>(1.0, 0.0, 1.0, 1.0)) + _e23.g0_));
}

fn translator_motor_sub(self_1472: Translator, other_1246: Motor) -> Motor {
    var self_1473: Translator;
    var other_1247: Motor;

    self_1473 = self_1472;
    other_1247 = other_1246;
    let _e4: Translator = self_1473;
    let _e7: Translator = self_1473;
    let _e10: Translator = self_1473;
    let _e13: Translator = self_1473;
    let _e23: Motor = other_1247;
    return Motor(((vec4<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.y, _e13.g0_.z) * vec4<f32>(1.0, 0.0, 1.0, 1.0)) - _e23.g0_));
}

fn translator_motor_geometric_product(self_1474: Translator, other_1248: Motor) -> Motor {
    var self_1475: Translator;
    var other_1249: Motor;

    self_1475 = self_1474;
    other_1249 = other_1248;
    let _e4: Translator = self_1475;
    let _e8: Motor = other_1249;
    let _e11: Translator = self_1475;
    let _e15: Motor = other_1249;
    let _e26: Translator = self_1475;
    let _e29: Translator = self_1475;
    let _e32: Translator = self_1475;
    let _e35: Translator = self_1475;
    let _e39: Motor = other_1249;
    return Motor((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.z) * _e15.g0_.yyyx) * vec4<f32>(0.0, 0.0, 1.0, 1.0))) + ((vec4<f32>(_e26.g0_.x, _e29.g0_.x, _e32.g0_.y, _e35.g0_.y) * _e39.g0_.xxxy) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))));
}

fn translator_motor_regressive_product(self_1476: Translator, other_1250: Motor) -> Plane {
    var self_1477: Translator;
    var other_1251: Motor;

    self_1477 = self_1476;
    other_1251 = other_1250;
    let _e4: Translator = self_1477;
    let _e8: Motor = other_1251;
    let _e11: Motor = other_1251;
    let _e14: Motor = other_1251;
    let _e25: Translator = self_1477;
    let _e28: Motor = other_1251;
    let _e31: Motor = other_1251;
    let _e34: Motor = other_1251;
    return Plane((((vec3<f32>(_e4.g0_.z) * vec3<f32>(_e8.g0_.z, _e11.g0_.y, _e14.g0_.z)) * vec3<f32>(1.0, -(1.0), 0.0)) + ((_e25.g0_.yxy * vec3<f32>(_e28.g0_.w, _e31.g0_.x, _e34.g0_.y)) * vec3<f32>(-(1.0), 0.0, 1.0))));
}

fn translator_motor_outer_product(self_1478: Translator, other_1252: Motor) -> Motor {
    var self_1479: Translator;
    var other_1253: Motor;

    self_1479 = self_1478;
    other_1253 = other_1252;
    let _e4: Translator = self_1479;
    let _e8: Motor = other_1253;
    let _e11: Translator = self_1479;
    let _e14: Translator = self_1479;
    let _e17: Translator = self_1479;
    let _e20: Translator = self_1479;
    let _e24: Motor = other_1253;
    return Motor(((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.x, _e14.g0_.x, _e17.g0_.y, _e20.g0_.z) * vec4<f32>(_e24.g0_.x)) * vec4<f32>(0.0, 0.0, 1.0, 1.0))));
}

fn translator_motor_inner_product(self_1480: Translator, other_1254: Motor) -> Motor {
    var self_1481: Translator;
    var other_1255: Motor;

    self_1481 = self_1480;
    other_1255 = other_1254;
    let _e4: Translator = self_1481;
    let _e8: Motor = other_1255;
    let _e11: Translator = self_1481;
    let _e14: Translator = self_1481;
    let _e17: Translator = self_1481;
    let _e20: Translator = self_1481;
    let _e24: Motor = other_1255;
    return Motor(((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.x, _e14.g0_.x, _e17.g0_.y, _e20.g0_.z) * vec4<f32>(_e24.g0_.x)) * vec4<f32>(0.0, 0.0, 1.0, 1.0))));
}

fn translator_motor_geometric_anti_product(self_1482: Translator, other_1256: Motor) -> MotorDual {
    var self_1483: Translator;
    var other_1257: Motor;

    self_1483 = self_1482;
    other_1257 = other_1256;
    let _e4: Translator = self_1483;
    let _e8: Motor = other_1257;
    let _e19: Translator = self_1483;
    let _e23: Motor = other_1257;
    let _e35: Translator = self_1483;
    let _e39: Motor = other_1257;
    return MotorDual(((((vec4<f32>(_e4.g0_.y) * _e8.g0_.zwxy) * vec4<f32>(1.0, -(1.0), 1.0, 1.0)) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.wzyx) * vec4<f32>(1.0, 1.0, -(1.0), 1.0))) + ((vec4<f32>(_e35.g0_.x) * _e39.g0_.xxzw) * vec4<f32>(0.0, 0.0, 1.0, 1.0))));
}

fn translator_motor_left_contraction(self_1484: Translator, other_1258: Motor) -> Motor {
    var self_1485: Translator;
    var other_1259: Motor;

    self_1485 = self_1484;
    other_1259 = other_1258;
    let _e4: Translator = self_1485;
    let _e8: Motor = other_1259;
    return Motor((vec4<f32>(_e4.g0_.x) * _e8.g0_));
}

fn translator_motor_right_contraction(self_1486: Translator, other_1260: Motor) -> Translator {
    var self_1487: Translator;
    var other_1261: Motor;

    self_1487 = self_1486;
    other_1261 = other_1260;
    let _e4: Translator = self_1487;
    let _e6: Motor = other_1261;
    return Translator((_e4.g0_ * vec3<f32>(_e6.g0_.x)));
}

fn translator_motor_scalar_product(self_1488: Translator, other_1262: Motor) -> Scalar {
    var self_1489: Translator;
    var other_1263: Motor;

    self_1489 = self_1488;
    other_1263 = other_1262;
    let _e4: Translator = self_1489;
    let _e7: Motor = other_1263;
    return Scalar((_e4.g0_.x * _e7.g0_.x));
}

fn translator_motor_anti_scalar_product(self_1490: Translator, other_1264: Motor) -> AntiScalar {
    var self_1491: Translator;
    var other_1265: Motor;

    self_1491 = self_1490;
    other_1265 = other_1264;
    let _e4: Translator = self_1491;
    let _e7: Motor = other_1265;
    let _e11: Translator = self_1491;
    let _e14: Motor = other_1265;
    return AntiScalar(((_e4.g0_.y * _e7.g0_.z) + (_e11.g0_.z * _e14.g0_.w)));
}

fn translator_motor_dual_geometric_product(self_1492: Translator, other_1266: MotorDual) -> MotorDual {
    var self_1493: Translator;
    var other_1267: MotorDual;

    self_1493 = self_1492;
    other_1267 = other_1266;
    let _e4: Translator = self_1493;
    let _e8: MotorDual = other_1267;
    let _e11: Translator = self_1493;
    let _e15: MotorDual = other_1267;
    let _e27: Translator = self_1493;
    let _e30: Translator = self_1493;
    let _e33: Translator = self_1493;
    let _e36: Translator = self_1493;
    let _e40: MotorDual = other_1267;
    return MotorDual((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.z) * _e15.g0_.wzww) * vec4<f32>(1.0, -(1.0), 0.0, 0.0))) + ((vec4<f32>(_e27.g0_.y, _e30.g0_.y, _e33.g0_.x, _e36.g0_.x) * _e40.g0_.zwxx) * vec4<f32>(1.0, 1.0, 0.0, 0.0))));
}

fn translator_motor_dual_regressive_product(self_1494: Translator, other_1268: MotorDual) -> Translator {
    var self_1495: Translator;
    var other_1269: MotorDual;

    self_1495 = self_1494;
    other_1269 = other_1268;
    let _e4: Translator = self_1495;
    let _e8: MotorDual = other_1269;
    let _e11: MotorDual = other_1269;
    let _e14: MotorDual = other_1269;
    let _e24: Translator = self_1495;
    let _e28: MotorDual = other_1269;
    let _e31: MotorDual = other_1269;
    let _e34: MotorDual = other_1269;
    let _e45: Translator = self_1495;
    let _e49: MotorDual = other_1269;
    return Translator(((((vec3<f32>(_e4.g0_.y) * vec3<f32>(_e8.g0_.z, _e11.g0_.x, _e14.g0_.z)) * vec3<f32>(1.0, 1.0, 0.0)) + ((vec3<f32>(_e24.g0_.z) * vec3<f32>(_e28.g0_.w, _e31.g0_.w, _e34.g0_.x)) * vec3<f32>(1.0, 0.0, 1.0))) + ((vec3<f32>(_e45.g0_.x) * vec3<f32>(_e49.g0_.x)) * vec3<f32>(1.0, 0.0, 0.0))));
}

fn translator_motor_dual_outer_product(self_1496: Translator, other_1270: MotorDual) -> MotorDual {
    var self_1497: Translator;
    var other_1271: MotorDual;

    self_1497 = self_1496;
    other_1271 = other_1270;
    let _e4: Translator = self_1497;
    let _e8: MotorDual = other_1271;
    let _e11: Translator = self_1497;
    let _e15: MotorDual = other_1271;
    let _e27: Translator = self_1497;
    let _e30: Translator = self_1497;
    let _e33: Translator = self_1497;
    let _e36: Translator = self_1497;
    let _e40: MotorDual = other_1271;
    return MotorDual((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.z) * vec4<f32>(_e15.g0_.w)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e27.g0_.y, _e30.g0_.x, _e33.g0_.x, _e36.g0_.x) * _e40.g0_.zxxx) * vec4<f32>(1.0, 0.0, 0.0, 0.0))));
}

fn translator_motor_dual_inner_product(self_1498: Translator, other_1272: MotorDual) -> MotorDual {
    var self_1499: Translator;
    var other_1273: MotorDual;

    self_1499 = self_1498;
    other_1273 = other_1272;
    let _e4: Translator = self_1499;
    let _e8: MotorDual = other_1273;
    let _e11: Translator = self_1499;
    let _e15: MotorDual = other_1273;
    let _e28: Translator = self_1499;
    let _e31: Translator = self_1499;
    let _e34: Translator = self_1499;
    let _e37: Translator = self_1499;
    let _e41: MotorDual = other_1273;
    return MotorDual((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.z) * vec4<f32>(_e15.g0_.z)) * vec4<f32>(0.0, -(1.0), 0.0, 0.0))) + ((vec4<f32>(_e28.g0_.x, _e31.g0_.y, _e34.g0_.x, _e37.g0_.x) * _e41.g0_.xwxx) * vec4<f32>(0.0, 1.0, 0.0, 0.0))));
}

fn translator_motor_dual_geometric_anti_product(self_1500: Translator, other_1274: MotorDual) -> Motor {
    var self_1501: Translator;
    var other_1275: MotorDual;

    self_1501 = self_1500;
    other_1275 = other_1274;
    let _e4: Translator = self_1501;
    let _e8: MotorDual = other_1275;
    let _e19: Translator = self_1501;
    let _e23: MotorDual = other_1275;
    let _e35: Translator = self_1501;
    let _e39: MotorDual = other_1275;
    return Motor(((((vec4<f32>(_e4.g0_.y) * _e8.g0_.zwxy) * vec4<f32>(1.0, 1.0, 1.0, -(1.0))) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.wzyx) * vec4<f32>(1.0, -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e35.g0_.x) * _e39.g0_.xyxx) * vec4<f32>(1.0, -(1.0), 0.0, 0.0))));
}

fn translator_motor_dual_inner_anti_product(self_1502: Translator, other_1276: MotorDual) -> Motor {
    var self_1503: Translator;
    var other_1277: MotorDual;

    self_1503 = self_1502;
    other_1277 = other_1276;
    let _e4: Translator = self_1503;
    let _e8: MotorDual = other_1277;
    let _e19: Translator = self_1503;
    let _e23: MotorDual = other_1277;
    let _e35: Translator = self_1503;
    let _e39: MotorDual = other_1277;
    return Motor(((((vec4<f32>(_e4.g0_.y) * _e8.g0_.wwxy) * vec4<f32>(0.0, 1.0, 1.0, -(1.0))) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.zzyx) * vec4<f32>(0.0, -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e35.g0_.x) * _e39.g0_.xyxx) * vec4<f32>(1.0, -(1.0), 0.0, 0.0))));
}

fn translator_motor_dual_left_contraction(self_1504: Translator, other_1278: MotorDual) -> MotorDual {
    var self_1505: Translator;
    var other_1279: MotorDual;

    self_1505 = self_1504;
    other_1279 = other_1278;
    let _e4: Translator = self_1505;
    let _e8: MotorDual = other_1279;
    return MotorDual((vec4<f32>(_e4.g0_.x) * _e8.g0_));
}

fn translator_motor_dual_left_anti_contraction(self_1506: Translator, other_1280: MotorDual) -> Point {
    var self_1507: Translator;
    var other_1281: MotorDual;

    self_1507 = self_1506;
    other_1281 = other_1280;
    let _e4: Translator = self_1507;
    let _e8: MotorDual = other_1281;
    let _e11: MotorDual = other_1281;
    let _e14: MotorDual = other_1281;
    let _e25: Translator = self_1507;
    let _e28: MotorDual = other_1281;
    let _e31: MotorDual = other_1281;
    let _e34: MotorDual = other_1281;
    return Point((((vec3<f32>(_e4.g0_.z) * vec3<f32>(_e8.g0_.z, _e11.g0_.y, _e14.g0_.z)) * vec3<f32>(-(1.0), 1.0, 0.0)) + ((_e25.g0_.yxy * vec3<f32>(_e28.g0_.w, _e31.g0_.x, _e34.g0_.y)) * vec3<f32>(1.0, 0.0, -(1.0)))));
}

fn translator_motor_dual_right_anti_contraction(self_1508: Translator, other_1282: MotorDual) -> Motor {
    var self_1509: Translator;
    var other_1283: MotorDual;

    self_1509 = self_1508;
    other_1283 = other_1282;
    let _e4: Translator = self_1509;
    let _e7: Translator = self_1509;
    let _e10: Translator = self_1509;
    let _e13: Translator = self_1509;
    let _e17: MotorDual = other_1283;
    return Motor(((vec4<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.y, _e13.g0_.z) * _e17.g0_.xyxx) * vec4<f32>(1.0, -(1.0), 1.0, 1.0)));
}

fn translator_squared_magnitude(self_1510: Translator) -> Scalar {
    var self_1511: Translator;

    self_1511 = self_1510;
    let _e2: Translator = self_1511;
    let _e3: Translator = self_1511;
    let _e4: Translator = translator_reversal(_e3);
    let _e5: Scalar = translator_translator_scalar_product(_e2, _e4);
    return _e5;
}

fn translator_magnitude(self_1512: Translator) -> Scalar {
    var self_1513: Translator;

    self_1513 = self_1512;
    let _e2: Translator = self_1513;
    let _e3: Scalar = translator_squared_magnitude(_e2);
    return Scalar(sqrt(_e3.g0_));
}

fn translator_bulk_norm(self_1514: Translator) -> Scalar {
    var self_1515: Translator;

    self_1515 = self_1514;
    let _e2: Translator = self_1515;
    let _e3: Scalar = translator_squared_magnitude(_e2);
    return Scalar(sqrt(_e3.g0_));
}

fn translator_squared_anti_magnitude(self_1516: Translator) -> AntiScalar {
    var self_1517: Translator;

    self_1517 = self_1516;
    let _e2: Translator = self_1517;
    let _e3: Translator = self_1517;
    let _e4: Translator = translator_anti_reversal(_e3);
    let _e5: AntiScalar = translator_translator_anti_scalar_product(_e2, _e4);
    return _e5;
}

fn translator_weight_norm(self_1518: Translator) -> AntiScalar {
    var self_1519: Translator;

    self_1519 = self_1518;
    let _e2: Translator = self_1519;
    let _e3: AntiScalar = translator_squared_anti_magnitude(_e2);
    return AntiScalar(sqrt(_e3.g0_));
}

fn translator_scale(self_1520: Translator, other_1284: f32) -> Translator {
    var self_1521: Translator;
    var other_1285: f32;

    self_1521 = self_1520;
    other_1285 = other_1284;
    let _e4: Translator = self_1521;
    let _e5: f32 = other_1285;
    let _e7: Translator = translator_scalar_geometric_product(_e4, Scalar(_e5));
    return _e7;
}

fn translator_signum(self_1522: Translator) -> Translator {
    var self_1523: Translator;

    self_1523 = self_1522;
    let _e2: Translator = self_1523;
    let _e3: Translator = self_1523;
    let _e4: Scalar = translator_magnitude(_e3);
    let _e9: Translator = translator_scalar_geometric_product(_e2, Scalar((1.0 / _e4.g0_)));
    return _e9;
}

fn translator_inverse(self_1524: Translator) -> Translator {
    var self_1525: Translator;

    self_1525 = self_1524;
    let _e2: Translator = self_1525;
    let _e3: Translator = translator_reversal(_e2);
    let _e4: Translator = self_1525;
    let _e5: Scalar = translator_squared_magnitude(_e4);
    let _e10: Translator = translator_scalar_geometric_product(_e3, Scalar((1.0 / _e5.g0_)));
    return _e10;
}

fn translator_unitize(self_1526: Translator) -> Translator {
    var self_1527: Translator;

    self_1527 = self_1526;
    let _e2: Translator = self_1527;
    let _e3: Translator = self_1527;
    let _e4: AntiScalar = translator_weight_norm(_e3);
    let _e9: Translator = translator_scalar_geometric_product(_e2, Scalar((1.0 / _e4.g0_)));
    return _e9;
}

fn motor_zero() -> Motor {
    return Motor(vec4<f32>(0.0));
}

fn motor_one() -> Motor {
    return Motor(vec4<f32>(1.0, 0.0, 0.0, 0.0));
}

fn motor_neg(self_1528: Motor) -> Motor {
    var self_1529: Motor;

    self_1529 = self_1528;
    let _e2: Motor = self_1529;
    return Motor((_e2.g0_ * vec4<f32>(-(1.0))));
}

fn motor_automorphism(self_1530: Motor) -> Motor {
    var self_1531: Motor;

    self_1531 = self_1530;
    let _e2: Motor = self_1531;
    return Motor(_e2.g0_);
}

fn motor_reversal(self_1532: Motor) -> Motor {
    var self_1533: Motor;

    self_1533 = self_1532;
    let _e2: Motor = self_1533;
    return Motor((_e2.g0_ * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))));
}

fn motor_conjugation(self_1534: Motor) -> Motor {
    var self_1535: Motor;

    self_1535 = self_1534;
    let _e2: Motor = self_1535;
    return Motor((_e2.g0_ * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))));
}

fn motor_dual(self_1536: Motor) -> MotorDual {
    var self_1537: Motor;

    self_1537 = self_1536;
    let _e2: Motor = self_1537;
    return MotorDual(_e2.g0_);
}

fn motor_anti_reversal(self_1538: Motor) -> Motor {
    var self_1539: Motor;

    self_1539 = self_1538;
    let _e2: Motor = self_1539;
    return Motor((_e2.g0_ * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))));
}

fn motor_right_complement(self_1540: Motor) -> MotorDual {
    var self_1541: Motor;

    self_1541 = self_1540;
    let _e2: Motor = self_1541;
    return MotorDual(_e2.g0_);
}

fn motor_left_complement(self_1542: Motor) -> MotorDual {
    var self_1543: Motor;

    self_1543 = self_1542;
    let _e2: Motor = self_1543;
    return MotorDual(_e2.g0_);
}

fn motor_double_complement(self_1544: Motor) -> Motor {
    var self_1545: Motor;

    self_1545 = self_1544;
    let _e2: Motor = self_1545;
    return Motor(_e2.g0_);
}

fn motor_scalar_into(self_1546: Motor) -> Scalar {
    var self_1547: Motor;

    self_1547 = self_1546;
    let _e2: Motor = self_1547;
    return Scalar(_e2.g0_.x);
}

fn motor_scalar_add(self_1548: Motor, other_1286: Scalar) -> Motor {
    var self_1549: Motor;
    var other_1287: Scalar;

    self_1549 = self_1548;
    other_1287 = other_1286;
    let _e4: Motor = self_1549;
    let _e6: Scalar = other_1287;
    return Motor((_e4.g0_ + (vec4<f32>(_e6.g0_) * vec4<f32>(1.0, 0.0, 0.0, 0.0))));
}

fn motor_scalar_sub(self_1550: Motor, other_1288: Scalar) -> Motor {
    var self_1551: Motor;
    var other_1289: Scalar;

    self_1551 = self_1550;
    other_1289 = other_1288;
    let _e4: Motor = self_1551;
    let _e6: Scalar = other_1289;
    return Motor((_e4.g0_ - (vec4<f32>(_e6.g0_) * vec4<f32>(1.0, 0.0, 0.0, 0.0))));
}

fn motor_scalar_geometric_product(self_1552: Motor, other_1290: Scalar) -> Motor {
    var self_1553: Motor;
    var other_1291: Scalar;

    self_1553 = self_1552;
    other_1291 = other_1290;
    let _e4: Motor = self_1553;
    let _e6: Scalar = other_1291;
    return Motor((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn motor_scalar_outer_product(self_1554: Motor, other_1292: Scalar) -> Motor {
    var self_1555: Motor;
    var other_1293: Scalar;

    self_1555 = self_1554;
    other_1293 = other_1292;
    let _e4: Motor = self_1555;
    let _e6: Scalar = other_1293;
    return Motor((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn motor_scalar_inner_product(self_1556: Motor, other_1294: Scalar) -> Motor {
    var self_1557: Motor;
    var other_1295: Scalar;

    self_1557 = self_1556;
    other_1295 = other_1294;
    let _e4: Motor = self_1557;
    let _e6: Scalar = other_1295;
    return Motor((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn motor_scalar_left_contraction(self_1558: Motor, other_1296: Scalar) -> Scalar {
    var self_1559: Motor;
    var other_1297: Scalar;

    self_1559 = self_1558;
    other_1297 = other_1296;
    let _e4: Motor = self_1559;
    let _e7: Scalar = other_1297;
    return Scalar((_e4.g0_.x * _e7.g0_));
}

fn motor_scalar_right_contraction(self_1560: Motor, other_1298: Scalar) -> Motor {
    var self_1561: Motor;
    var other_1299: Scalar;

    self_1561 = self_1560;
    other_1299 = other_1298;
    let _e4: Motor = self_1561;
    let _e6: Scalar = other_1299;
    return Motor((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn motor_scalar_scalar_product(self_1562: Motor, other_1300: Scalar) -> Scalar {
    var self_1563: Motor;
    var other_1301: Scalar;

    self_1563 = self_1562;
    other_1301 = other_1300;
    let _e4: Motor = self_1563;
    let _e7: Scalar = other_1301;
    return Scalar((_e4.g0_.x * _e7.g0_));
}

fn motor_anti_scalar_regressive_product(self_1564: Motor, other_1302: AntiScalar) -> Motor {
    var self_1565: Motor;
    var other_1303: AntiScalar;

    self_1565 = self_1564;
    other_1303 = other_1302;
    let _e4: Motor = self_1565;
    let _e6: AntiScalar = other_1303;
    return Motor((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn motor_anti_scalar_outer_product(self_1566: Motor, other_1304: AntiScalar) -> AntiScalar {
    var self_1567: Motor;
    var other_1305: AntiScalar;

    self_1567 = self_1566;
    other_1305 = other_1304;
    let _e4: Motor = self_1567;
    let _e7: AntiScalar = other_1305;
    return AntiScalar((_e4.g0_.x * _e7.g0_));
}

fn motor_anti_scalar_geometric_anti_product(self_1568: Motor, other_1306: AntiScalar) -> Motor {
    var self_1569: Motor;
    var other_1307: AntiScalar;

    self_1569 = self_1568;
    other_1307 = other_1306;
    let _e4: Motor = self_1569;
    let _e6: AntiScalar = other_1307;
    return Motor((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn motor_anti_scalar_inner_anti_product(self_1570: Motor, other_1308: AntiScalar) -> Motor {
    var self_1571: Motor;
    var other_1309: AntiScalar;

    self_1571 = self_1570;
    other_1309 = other_1308;
    let _e4: Motor = self_1571;
    let _e6: AntiScalar = other_1309;
    return Motor((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn motor_anti_scalar_right_anti_contraction(self_1572: Motor, other_1310: AntiScalar) -> Motor {
    var self_1573: Motor;
    var other_1311: AntiScalar;

    self_1573 = self_1572;
    other_1311 = other_1310;
    let _e4: Motor = self_1573;
    let _e6: AntiScalar = other_1311;
    return Motor((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn motor_multi_vector_add(self_1574: Motor, other_1312: MultiVector) -> MultiVector {
    var self_1575: Motor;
    var other_1313: MultiVector;

    self_1575 = self_1574;
    other_1313 = other_1312;
    let _e4: Motor = self_1575;
    let _e13: MultiVector = other_1313;
    let _e16: Motor = self_1575;
    let _e25: MultiVector = other_1313;
    return MultiVector(((_e4.g0_.xyxx * vec4<f32>(1.0, 1.0, 0.0, 0.0)) + _e13.g0_), ((_e16.g0_.xxzw * vec4<f32>(0.0, 0.0, 1.0, 1.0)) + _e25.g1_));
}

fn motor_multi_vector_sub(self_1576: Motor, other_1314: MultiVector) -> MultiVector {
    var self_1577: Motor;
    var other_1315: MultiVector;

    self_1577 = self_1576;
    other_1315 = other_1314;
    let _e4: Motor = self_1577;
    let _e13: MultiVector = other_1315;
    let _e16: Motor = self_1577;
    let _e25: MultiVector = other_1315;
    return MultiVector(((_e4.g0_.xyxx * vec4<f32>(1.0, 1.0, 0.0, 0.0)) - _e13.g0_), ((_e16.g0_.xxzw * vec4<f32>(0.0, 0.0, 1.0, 1.0)) - _e25.g1_));
}

fn motor_multi_vector_geometric_product(self_1578: Motor, other_1316: MultiVector) -> MultiVector {
    var self_1579: Motor;
    var other_1317: MultiVector;

    self_1579 = self_1578;
    other_1317 = other_1316;
    let _e4: Motor = self_1579;
    let _e8: MultiVector = other_1317;
    let _e11: Motor = self_1579;
    let _e15: MultiVector = other_1317;
    let _e28: Motor = self_1579;
    let _e32: MultiVector = other_1317;
    let _e35: Motor = self_1579;
    let _e39: MultiVector = other_1317;
    let _e52: Motor = self_1579;
    let _e56: MultiVector = other_1317;
    let _e68: Motor = self_1579;
    let _e72: MultiVector = other_1317;
    return MultiVector(((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.y) * _e15.g0_.yxwz) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))), ((((vec4<f32>(_e28.g0_.x) * _e32.g1_) + ((vec4<f32>(_e35.g0_.y) * _e39.g1_.yxwz) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))) + ((vec4<f32>(_e52.g0_.z) * _e56.g0_.zwxy) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e68.g0_.w) * _e72.g0_.wzyx) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0))));
}

fn motor_multi_vector_outer_product(self_1580: Motor, other_1318: MultiVector) -> MultiVector {
    var self_1581: Motor;
    var other_1319: MultiVector;

    self_1581 = self_1580;
    other_1319 = other_1318;
    let _e4: Motor = self_1581;
    let _e8: MultiVector = other_1319;
    let _e11: Motor = self_1581;
    let _e14: MultiVector = other_1319;
    let _e26: Motor = self_1581;
    let _e30: MultiVector = other_1319;
    let _e33: Motor = self_1581;
    let _e37: MultiVector = other_1319;
    let _e48: Motor = self_1581;
    let _e52: MultiVector = other_1319;
    let _e63: Motor = self_1581;
    let _e66: MultiVector = other_1319;
    return MultiVector(((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((_e11.g0_.xyxx * vec4<f32>(_e14.g0_.x)) * vec4<f32>(0.0, 1.0, 0.0, 0.0))), ((((vec4<f32>(_e26.g0_.x) * _e30.g1_) + ((vec4<f32>(_e33.g0_.z) * _e37.g0_.wwxw) * vec4<f32>(0.0, 1.0, 1.0, 0.0))) + ((vec4<f32>(_e48.g0_.w) * _e52.g0_.zzzx) * vec4<f32>(0.0, 1.0, 0.0, 1.0))) + ((_e63.g0_.xyxx * vec4<f32>(_e66.g1_.x)) * vec4<f32>(0.0, 1.0, 0.0, 0.0))));
}

fn motor_multi_vector_inner_product(self_1582: Motor, other_1320: MultiVector) -> MultiVector {
    var self_1583: Motor;
    var other_1321: MultiVector;

    self_1583 = self_1582;
    other_1321 = other_1320;
    let _e4: Motor = self_1583;
    let _e8: MultiVector = other_1321;
    let _e11: Motor = self_1583;
    let _e15: MultiVector = other_1321;
    let _e28: Motor = self_1583;
    let _e32: MultiVector = other_1321;
    let _e35: Motor = self_1583;
    let _e39: MultiVector = other_1321;
    let _e50: Motor = self_1583;
    let _e54: MultiVector = other_1321;
    let _e66: Motor = self_1583;
    let _e69: MultiVector = other_1321;
    return MultiVector(((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.y) * _e15.g0_.yxwz) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))), ((((vec4<f32>(_e28.g0_.x) * _e32.g1_) + ((vec4<f32>(_e35.g0_.z) * _e39.g0_.zzxz) * vec4<f32>(1.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e50.g0_.w) * _e54.g0_.wwwx) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))) + ((_e66.g0_.yxxx * _e69.g1_.yxxx) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))));
}

fn motor_multi_vector_geometric_anti_product(self_1584: Motor, other_1322: MultiVector) -> MultiVector {
    var self_1585: Motor;
    var other_1323: MultiVector;

    self_1585 = self_1584;
    other_1323 = other_1322;
    let _e4: Motor = self_1585;
    let _e8: MultiVector = other_1323;
    let _e19: Motor = self_1585;
    let _e23: MultiVector = other_1323;
    let _e34: Motor = self_1585;
    let _e38: MultiVector = other_1323;
    let _e43: Motor = self_1585;
    let _e47: MultiVector = other_1323;
    let _e60: Motor = self_1585;
    let _e64: MultiVector = other_1323;
    let _e76: Motor = self_1585;
    let _e80: MultiVector = other_1323;
    return MultiVector((((((vec4<f32>(_e4.g0_.x) * _e8.g1_.yxwz) * vec4<f32>(1.0, -(1.0), 1.0, 1.0)) + ((vec4<f32>(_e19.g0_.y) * _e23.g1_) * vec4<f32>(1.0, 1.0, -(1.0), 1.0))) + (vec4<f32>(_e34.g0_.z) * _e38.g0_.wzyx)) + ((vec4<f32>(_e43.g0_.w) * _e47.g0_.zwxy) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))), (((vec4<f32>(_e60.g0_.z) * _e64.g1_.wzyx) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0))) + (vec4<f32>(_e76.g0_.w) * _e80.g1_.zwxy)));
}

fn motor_multi_vector_left_contraction(self_1586: Motor, other_1324: MultiVector) -> MultiVector {
    var self_1587: Motor;
    var other_1325: MultiVector;

    self_1587 = self_1586;
    other_1325 = other_1324;
    let _e4: Motor = self_1587;
    let _e8: MultiVector = other_1325;
    let _e11: Motor = self_1587;
    let _e14: MultiVector = other_1325;
    let _e26: Motor = self_1587;
    let _e30: MultiVector = other_1325;
    let _e33: Motor = self_1587;
    let _e36: MultiVector = other_1325;
    return MultiVector(((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((_e11.g0_.yxxx * _e14.g0_.yxxx) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))), ((vec4<f32>(_e26.g0_.x) * _e30.g1_) + ((_e33.g0_.yxxx * _e36.g1_.yxxx) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))));
}

fn motor_multi_vector_scalar_product(self_1588: Motor, other_1326: MultiVector) -> Scalar {
    var self_1589: Motor;
    var other_1327: MultiVector;

    self_1589 = self_1588;
    other_1327 = other_1326;
    let _e4: Motor = self_1589;
    let _e7: MultiVector = other_1327;
    let _e11: Motor = self_1589;
    let _e14: MultiVector = other_1327;
    return Scalar(((_e4.g0_.x * _e7.g0_.x) - (_e11.g0_.y * _e14.g0_.y)));
}

fn motor_multi_vector_anti_scalar_product(self_1590: Motor, other_1328: MultiVector) -> AntiScalar {
    var self_1591: Motor;
    var other_1329: MultiVector;

    self_1591 = self_1590;
    other_1329 = other_1328;
    let _e4: Motor = self_1591;
    let _e7: MultiVector = other_1329;
    let _e11: Motor = self_1591;
    let _e14: MultiVector = other_1329;
    return AntiScalar(((_e4.g0_.z * _e7.g1_.z) + (_e11.g0_.w * _e14.g1_.w)));
}

fn motor_rotor_into(self_1592: Motor) -> Rotor {
    var self_1593: Motor;

    self_1593 = self_1592;
    let _e2: Motor = self_1593;
    let _e5: Motor = self_1593;
    return Rotor(vec2<f32>(_e2.g0_.x, _e5.g0_.y));
}

fn motor_rotor_add(self_1594: Motor, other_1330: Rotor) -> Motor {
    var self_1595: Motor;
    var other_1331: Rotor;

    self_1595 = self_1594;
    other_1331 = other_1330;
    let _e4: Motor = self_1595;
    let _e6: Rotor = other_1331;
    let _e9: Rotor = other_1331;
    let _e12: Rotor = other_1331;
    let _e15: Rotor = other_1331;
    return Motor((_e4.g0_ + (vec4<f32>(_e6.g0_.x, _e9.g0_.y, _e12.g0_.x, _e15.g0_.x) * vec4<f32>(1.0, 1.0, 0.0, 0.0))));
}

fn motor_rotor_sub(self_1596: Motor, other_1332: Rotor) -> Motor {
    var self_1597: Motor;
    var other_1333: Rotor;

    self_1597 = self_1596;
    other_1333 = other_1332;
    let _e4: Motor = self_1597;
    let _e6: Rotor = other_1333;
    let _e9: Rotor = other_1333;
    let _e12: Rotor = other_1333;
    let _e15: Rotor = other_1333;
    return Motor((_e4.g0_ - (vec4<f32>(_e6.g0_.x, _e9.g0_.y, _e12.g0_.x, _e15.g0_.x) * vec4<f32>(1.0, 1.0, 0.0, 0.0))));
}

fn motor_rotor_geometric_product(self_1598: Motor, other_1334: Rotor) -> Motor {
    var self_1599: Motor;
    var other_1335: Rotor;

    self_1599 = self_1598;
    other_1335 = other_1334;
    let _e4: Motor = self_1599;
    let _e8: Rotor = other_1335;
    let _e11: Rotor = other_1335;
    let _e14: Rotor = other_1335;
    let _e17: Rotor = other_1335;
    let _e29: Motor = self_1599;
    let _e33: Rotor = other_1335;
    let _e36: Rotor = other_1335;
    let _e39: Rotor = other_1335;
    let _e42: Rotor = other_1335;
    let _e54: Motor = self_1599;
    let _e57: Rotor = other_1335;
    let _e60: Rotor = other_1335;
    let _e63: Rotor = other_1335;
    let _e66: Rotor = other_1335;
    return Motor(((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y, _e11.g0_.x, _e14.g0_.y, _e17.g0_.y)) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0)) + ((vec4<f32>(_e29.g0_.w) * vec4<f32>(_e33.g0_.y, _e36.g0_.y, _e39.g0_.y, _e42.g0_.x)) * vec4<f32>(0.0, 0.0, 1.0, 1.0))) + ((_e54.g0_.xxzz * vec4<f32>(_e57.g0_.x, _e60.g0_.y, _e63.g0_.x, _e66.g0_.y)) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))));
}

fn motor_rotor_outer_product(self_1600: Motor, other_1336: Rotor) -> Motor {
    var self_1601: Motor;
    var other_1337: Rotor;

    self_1601 = self_1600;
    other_1337 = other_1336;
    let _e4: Motor = self_1601;
    let _e8: Rotor = other_1337;
    let _e19: Motor = self_1601;
    let _e22: Rotor = other_1337;
    let _e25: Rotor = other_1337;
    let _e28: Rotor = other_1337;
    let _e31: Rotor = other_1337;
    return Motor((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.x)) * vec4<f32>(0.0, 1.0, 0.0, 0.0)) + (_e19.g0_.xxzw * vec4<f32>(_e22.g0_.x, _e25.g0_.y, _e28.g0_.x, _e31.g0_.x))));
}

fn motor_rotor_inner_product(self_1602: Motor, other_1338: Rotor) -> Motor {
    var self_1603: Motor;
    var other_1339: Rotor;

    self_1603 = self_1602;
    other_1339 = other_1338;
    let _e4: Motor = self_1603;
    let _e8: Rotor = other_1339;
    let _e11: Rotor = other_1339;
    let _e14: Rotor = other_1339;
    let _e17: Rotor = other_1339;
    let _e29: Motor = self_1603;
    let _e32: Rotor = other_1339;
    let _e35: Rotor = other_1339;
    let _e38: Rotor = other_1339;
    let _e41: Rotor = other_1339;
    return Motor((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y, _e11.g0_.x, _e14.g0_.y, _e17.g0_.y)) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0)) + (_e29.g0_.xxzw * vec4<f32>(_e32.g0_.x, _e35.g0_.y, _e38.g0_.x, _e41.g0_.x))));
}

fn motor_rotor_left_contraction(self_1604: Motor, other_1340: Rotor) -> Rotor {
    var self_1605: Motor;
    var other_1341: Rotor;

    self_1605 = self_1604;
    other_1341 = other_1340;
    let _e4: Motor = self_1605;
    let _e8: Rotor = other_1341;
    let _e11: Motor = self_1605;
    let _e14: Motor = self_1605;
    let _e18: Rotor = other_1341;
    return Rotor(((vec2<f32>(_e4.g0_.x) * _e8.g0_) + ((vec2<f32>(_e11.g0_.y, _e14.g0_.x) * _e18.g0_.yx) * vec2<f32>(-(1.0), 0.0))));
}

fn motor_rotor_right_contraction(self_1606: Motor, other_1342: Rotor) -> Motor {
    var self_1607: Motor;
    var other_1343: Rotor;

    self_1607 = self_1606;
    other_1343 = other_1342;
    let _e4: Motor = self_1607;
    let _e8: Rotor = other_1343;
    let _e11: Rotor = other_1343;
    let _e14: Rotor = other_1343;
    let _e17: Rotor = other_1343;
    let _e29: Motor = self_1607;
    let _e32: Rotor = other_1343;
    return Motor((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y, _e11.g0_.x, _e14.g0_.y, _e17.g0_.y)) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0)) + ((_e29.g0_.xxzw * vec4<f32>(_e32.g0_.x)) * vec4<f32>(1.0, 0.0, 1.0, 1.0))));
}

fn motor_rotor_scalar_product(self_1608: Motor, other_1344: Rotor) -> Scalar {
    var self_1609: Motor;
    var other_1345: Rotor;

    self_1609 = self_1608;
    other_1345 = other_1344;
    let _e4: Motor = self_1609;
    let _e7: Rotor = other_1345;
    let _e11: Motor = self_1609;
    let _e14: Rotor = other_1345;
    return Scalar(((_e4.g0_.x * _e7.g0_.x) - (_e11.g0_.y * _e14.g0_.y)));
}

fn motor_point_into(self_1610: Motor) -> Point {
    var self_1611: Motor;

    self_1611 = self_1610;
    let _e2: Motor = self_1611;
    let _e5: Motor = self_1611;
    let _e8: Motor = self_1611;
    return Point(vec3<f32>(_e2.g0_.y, _e5.g0_.z, _e8.g0_.w));
}

fn motor_point_add(self_1612: Motor, other_1346: Point) -> Motor {
    var self_1613: Motor;
    var other_1347: Point;

    self_1613 = self_1612;
    other_1347 = other_1346;
    let _e4: Motor = self_1613;
    let _e6: Point = other_1347;
    let _e9: Point = other_1347;
    let _e12: Point = other_1347;
    let _e15: Point = other_1347;
    return Motor((_e4.g0_ + (vec4<f32>(_e6.g0_.x, _e9.g0_.x, _e12.g0_.y, _e15.g0_.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn motor_point_sub(self_1614: Motor, other_1348: Point) -> Motor {
    var self_1615: Motor;
    var other_1349: Point;

    self_1615 = self_1614;
    other_1349 = other_1348;
    let _e4: Motor = self_1615;
    let _e6: Point = other_1349;
    let _e9: Point = other_1349;
    let _e12: Point = other_1349;
    let _e15: Point = other_1349;
    return Motor((_e4.g0_ - (vec4<f32>(_e6.g0_.x, _e9.g0_.x, _e12.g0_.y, _e15.g0_.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn motor_point_geometric_product(self_1616: Motor, other_1350: Point) -> Motor {
    var self_1617: Motor;
    var other_1351: Point;

    self_1617 = self_1616;
    other_1351 = other_1350;
    let _e4: Motor = self_1617;
    let _e8: Point = other_1351;
    let _e11: Point = other_1351;
    let _e14: Point = other_1351;
    let _e17: Point = other_1351;
    let _e30: Motor = self_1617;
    let _e34: Point = other_1351;
    let _e47: Motor = self_1617;
    let _e51: Point = other_1351;
    let _e63: Motor = self_1617;
    let _e67: Point = other_1351;
    let _e70: Point = other_1351;
    let _e73: Point = other_1351;
    let _e76: Point = other_1351;
    return Motor((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.x, _e11.g0_.x, _e14.g0_.z, _e17.g0_.y)) * vec4<f32>(-(1.0), 0.0, -(1.0), 1.0)) + ((vec4<f32>(_e30.g0_.z) * vec4<f32>(_e34.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e47.g0_.w) * vec4<f32>(_e51.g0_.x)) * vec4<f32>(0.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e63.g0_.x) * vec4<f32>(_e67.g0_.x, _e70.g0_.x, _e73.g0_.y, _e76.g0_.z)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn motor_point_regressive_product(self_1618: Motor, other_1352: Point) -> Plane {
    var self_1619: Motor;
    var other_1353: Point;

    self_1619 = self_1618;
    other_1353 = other_1352;
    let _e4: Motor = self_1619;
    let _e8: Point = other_1353;
    let _e18: Motor = self_1619;
    let _e22: Point = other_1353;
    let _e33: Motor = self_1619;
    let _e36: Motor = self_1619;
    let _e39: Motor = self_1619;
    let _e43: Point = other_1353;
    return Plane(((((vec3<f32>(_e4.g0_.z) * _e8.g0_.zzx) * vec3<f32>(-(1.0), 0.0, 1.0)) + ((vec3<f32>(_e18.g0_.w) * _e22.g0_.yxy) * vec3<f32>(1.0, -(1.0), 0.0))) + ((vec3<f32>(_e33.g0_.x, _e36.g0_.y, _e39.g0_.y) * _e43.g0_.xzy) * vec3<f32>(0.0, 1.0, -(1.0)))));
}

fn motor_point_outer_product(self_1620: Motor, other_1354: Point) -> Point {
    var self_1621: Motor;
    var other_1355: Point;

    self_1621 = self_1620;
    other_1355 = other_1354;
    let _e4: Motor = self_1621;
    let _e8: Point = other_1355;
    return Point((vec3<f32>(_e4.g0_.x) * _e8.g0_));
}

fn motor_point_inner_product(self_1622: Motor, other_1356: Point) -> Motor {
    var self_1623: Motor;
    var other_1357: Point;

    self_1623 = self_1622;
    other_1357 = other_1356;
    let _e4: Motor = self_1623;
    let _e7: Point = other_1357;
    let _e10: Point = other_1357;
    let _e13: Point = other_1357;
    let _e16: Point = other_1357;
    return Motor(((_e4.g0_.yxxx * vec4<f32>(_e7.g0_.x, _e10.g0_.x, _e13.g0_.y, _e16.g0_.z)) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0)));
}

fn motor_point_geometric_anti_product(self_1624: Motor, other_1358: Point) -> MotorDual {
    var self_1625: Motor;
    var other_1359: Point;

    self_1625 = self_1624;
    other_1359 = other_1358;
    let _e4: Motor = self_1625;
    let _e8: Point = other_1359;
    let _e11: Point = other_1359;
    let _e14: Point = other_1359;
    let _e17: Point = other_1359;
    let _e29: Motor = self_1625;
    let _e33: Point = other_1359;
    let _e36: Point = other_1359;
    let _e39: Point = other_1359;
    let _e42: Point = other_1359;
    let _e55: Motor = self_1625;
    let _e59: Point = other_1359;
    let _e62: Point = other_1359;
    let _e65: Point = other_1359;
    let _e68: Point = other_1359;
    let _e81: Motor = self_1625;
    let _e85: Point = other_1359;
    let _e88: Point = other_1359;
    let _e91: Point = other_1359;
    let _e94: Point = other_1359;
    return MotorDual((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.z, _e11.g0_.z, _e14.g0_.z, _e17.g0_.y)) * vec4<f32>(0.0, 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e29.g0_.z) * vec4<f32>(_e33.g0_.y, _e36.g0_.z, _e39.g0_.y, _e42.g0_.x)) * vec4<f32>(1.0, -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e55.g0_.w) * vec4<f32>(_e59.g0_.z, _e62.g0_.y, _e65.g0_.x, _e68.g0_.z)) * vec4<f32>(1.0, 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e81.g0_.x) * vec4<f32>(_e85.g0_.x, _e88.g0_.x, _e91.g0_.y, _e94.g0_.z)) * vec4<f32>(0.0, 0.0, 1.0, 1.0))));
}

fn motor_point_left_contraction(self_1626: Motor, other_1360: Point) -> Motor {
    var self_1627: Motor;
    var other_1361: Point;

    self_1627 = self_1626;
    other_1361 = other_1360;
    let _e4: Motor = self_1627;
    let _e7: Point = other_1361;
    let _e10: Point = other_1361;
    let _e13: Point = other_1361;
    let _e16: Point = other_1361;
    return Motor(((_e4.g0_.yxxx * vec4<f32>(_e7.g0_.x, _e10.g0_.x, _e13.g0_.y, _e16.g0_.z)) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0)));
}

fn motor_point_right_contraction(self_1628: Motor, other_1362: Point) -> Scalar {
    var self_1629: Motor;
    var other_1363: Point;

    self_1629 = self_1628;
    other_1363 = other_1362;
    let _e5: Motor = self_1629;
    let _e8: Point = other_1363;
    return Scalar((0.0 - (_e5.g0_.y * _e8.g0_.x)));
}

fn motor_point_left_anti_contraction(self_1630: Motor, other_1364: Point) -> AntiScalar {
    var self_1631: Motor;
    var other_1365: Point;

    self_1631 = self_1630;
    other_1365 = other_1364;
    let _e4: Motor = self_1631;
    let _e7: Point = other_1365;
    let _e11: Motor = self_1631;
    let _e14: Point = other_1365;
    return AntiScalar(((_e4.g0_.z * _e7.g0_.y) + (_e11.g0_.w * _e14.g0_.z)));
}

fn motor_point_scalar_product(self_1632: Motor, other_1366: Point) -> Scalar {
    var self_1633: Motor;
    var other_1367: Point;

    self_1633 = self_1632;
    other_1367 = other_1366;
    let _e5: Motor = self_1633;
    let _e8: Point = other_1367;
    return Scalar((0.0 - (_e5.g0_.y * _e8.g0_.x)));
}

fn motor_point_anti_scalar_product(self_1634: Motor, other_1368: Point) -> AntiScalar {
    var self_1635: Motor;
    var other_1369: Point;

    self_1635 = self_1634;
    other_1369 = other_1368;
    let _e4: Motor = self_1635;
    let _e7: Point = other_1369;
    let _e11: Motor = self_1635;
    let _e14: Point = other_1369;
    return AntiScalar(((_e4.g0_.z * _e7.g0_.y) + (_e11.g0_.w * _e14.g0_.z)));
}

fn motor_ideal_point_into(self_1636: Motor) -> IdealPoint {
    var self_1637: Motor;

    self_1637 = self_1636;
    let _e2: Motor = self_1637;
    let _e5: Motor = self_1637;
    return IdealPoint(vec2<f32>(_e2.g0_.z, _e5.g0_.w));
}

fn motor_ideal_point_add(self_1638: Motor, other_1370: IdealPoint) -> Motor {
    var self_1639: Motor;
    var other_1371: IdealPoint;

    self_1639 = self_1638;
    other_1371 = other_1370;
    let _e4: Motor = self_1639;
    let _e6: IdealPoint = other_1371;
    let _e9: IdealPoint = other_1371;
    let _e12: IdealPoint = other_1371;
    let _e15: IdealPoint = other_1371;
    return Motor((_e4.g0_ + (vec4<f32>(_e6.g0_.x, _e9.g0_.x, _e12.g0_.x, _e15.g0_.y) * vec4<f32>(0.0, 0.0, 1.0, 1.0))));
}

fn motor_ideal_point_sub(self_1640: Motor, other_1372: IdealPoint) -> Motor {
    var self_1641: Motor;
    var other_1373: IdealPoint;

    self_1641 = self_1640;
    other_1373 = other_1372;
    let _e4: Motor = self_1641;
    let _e6: IdealPoint = other_1373;
    let _e9: IdealPoint = other_1373;
    let _e12: IdealPoint = other_1373;
    let _e15: IdealPoint = other_1373;
    return Motor((_e4.g0_ - (vec4<f32>(_e6.g0_.x, _e9.g0_.x, _e12.g0_.x, _e15.g0_.y) * vec4<f32>(0.0, 0.0, 1.0, 1.0))));
}

fn motor_ideal_point_geometric_product(self_1642: Motor, other_1374: IdealPoint) -> IdealPoint {
    var self_1643: Motor;
    var other_1375: IdealPoint;

    self_1643 = self_1642;
    other_1375 = other_1374;
    let _e4: Motor = self_1643;
    let _e8: IdealPoint = other_1375;
    let _e11: Motor = self_1643;
    let _e15: IdealPoint = other_1375;
    return IdealPoint(((vec2<f32>(_e4.g0_.x) * _e8.g0_) + ((vec2<f32>(_e11.g0_.y) * _e15.g0_.yx) * vec2<f32>(-(1.0), 1.0))));
}

fn motor_ideal_point_regressive_product(self_1644: Motor, other_1376: IdealPoint) -> Plane {
    var self_1645: Motor;
    var other_1377: IdealPoint;

    self_1645 = self_1644;
    other_1377 = other_1376;
    let _e4: Motor = self_1645;
    let _e8: IdealPoint = other_1377;
    let _e18: Motor = self_1645;
    let _e21: Motor = self_1645;
    let _e24: Motor = self_1645;
    let _e28: IdealPoint = other_1377;
    let _e31: IdealPoint = other_1377;
    let _e34: IdealPoint = other_1377;
    return Plane((((vec3<f32>(_e4.g0_.w) * vec3<f32>(_e8.g0_.x)) * vec3<f32>(1.0, 0.0, 0.0)) + ((vec3<f32>(_e18.g0_.z, _e21.g0_.y, _e24.g0_.y) * vec3<f32>(_e28.g0_.y, _e31.g0_.y, _e34.g0_.x)) * vec3<f32>(-(1.0), 1.0, -(1.0)))));
}

fn motor_ideal_point_outer_product(self_1646: Motor, other_1378: IdealPoint) -> IdealPoint {
    var self_1647: Motor;
    var other_1379: IdealPoint;

    self_1647 = self_1646;
    other_1379 = other_1378;
    let _e4: Motor = self_1647;
    let _e8: IdealPoint = other_1379;
    return IdealPoint((vec2<f32>(_e4.g0_.x) * _e8.g0_));
}

fn motor_ideal_point_inner_product(self_1648: Motor, other_1380: IdealPoint) -> IdealPoint {
    var self_1649: Motor;
    var other_1381: IdealPoint;

    self_1649 = self_1648;
    other_1381 = other_1380;
    let _e4: Motor = self_1649;
    let _e8: IdealPoint = other_1381;
    return IdealPoint((vec2<f32>(_e4.g0_.x) * _e8.g0_));
}

fn motor_ideal_point_geometric_anti_product(self_1650: Motor, other_1382: IdealPoint) -> MotorDual {
    var self_1651: Motor;
    var other_1383: IdealPoint;

    self_1651 = self_1650;
    other_1383 = other_1382;
    let _e4: Motor = self_1651;
    let _e8: IdealPoint = other_1383;
    let _e11: IdealPoint = other_1383;
    let _e14: IdealPoint = other_1383;
    let _e17: IdealPoint = other_1383;
    let _e29: Motor = self_1651;
    let _e33: IdealPoint = other_1383;
    let _e36: IdealPoint = other_1383;
    let _e39: IdealPoint = other_1383;
    let _e42: IdealPoint = other_1383;
    let _e54: Motor = self_1651;
    let _e57: IdealPoint = other_1383;
    let _e60: IdealPoint = other_1383;
    let _e63: IdealPoint = other_1383;
    let _e66: IdealPoint = other_1383;
    return MotorDual(((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y, _e11.g0_.y, _e14.g0_.y, _e17.g0_.x)) * vec4<f32>(0.0, 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e29.g0_.w) * vec4<f32>(_e33.g0_.y, _e36.g0_.x, _e39.g0_.y, _e42.g0_.y)) * vec4<f32>(1.0, 1.0, 0.0, 0.0))) + ((_e54.g0_.zzxx * vec4<f32>(_e57.g0_.x, _e60.g0_.y, _e63.g0_.x, _e66.g0_.y)) * vec4<f32>(1.0, -(1.0), 1.0, 1.0))));
}

fn motor_ideal_point_left_contraction(self_1652: Motor, other_1384: IdealPoint) -> IdealPoint {
    var self_1653: Motor;
    var other_1385: IdealPoint;

    self_1653 = self_1652;
    other_1385 = other_1384;
    let _e4: Motor = self_1653;
    let _e8: IdealPoint = other_1385;
    return IdealPoint((vec2<f32>(_e4.g0_.x) * _e8.g0_));
}

fn motor_ideal_point_left_anti_contraction(self_1654: Motor, other_1386: IdealPoint) -> AntiScalar {
    var self_1655: Motor;
    var other_1387: IdealPoint;

    self_1655 = self_1654;
    other_1387 = other_1386;
    let _e4: Motor = self_1655;
    let _e7: IdealPoint = other_1387;
    let _e11: Motor = self_1655;
    let _e14: IdealPoint = other_1387;
    return AntiScalar(((_e4.g0_.z * _e7.g0_.x) + (_e11.g0_.w * _e14.g0_.y)));
}

fn motor_ideal_point_anti_scalar_product(self_1656: Motor, other_1388: IdealPoint) -> AntiScalar {
    var self_1657: Motor;
    var other_1389: IdealPoint;

    self_1657 = self_1656;
    other_1389 = other_1388;
    let _e4: Motor = self_1657;
    let _e7: IdealPoint = other_1389;
    let _e11: Motor = self_1657;
    let _e14: IdealPoint = other_1389;
    return AntiScalar(((_e4.g0_.z * _e7.g0_.x) + (_e11.g0_.w * _e14.g0_.y)));
}

fn motor_plane_geometric_product(self_1658: Motor, other_1390: Plane) -> MotorDual {
    var self_1659: Motor;
    var other_1391: Plane;

    self_1659 = self_1658;
    other_1391 = other_1390;
    let _e4: Motor = self_1659;
    let _e8: Plane = other_1391;
    let _e11: Plane = other_1391;
    let _e14: Plane = other_1391;
    let _e17: Plane = other_1391;
    let _e29: Motor = self_1659;
    let _e33: Plane = other_1391;
    let _e36: Plane = other_1391;
    let _e39: Plane = other_1391;
    let _e42: Plane = other_1391;
    let _e54: Motor = self_1659;
    let _e58: Plane = other_1391;
    let _e61: Plane = other_1391;
    let _e64: Plane = other_1391;
    let _e67: Plane = other_1391;
    let _e80: Motor = self_1659;
    let _e84: Plane = other_1391;
    let _e87: Plane = other_1391;
    let _e90: Plane = other_1391;
    let _e93: Plane = other_1391;
    return MotorDual((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.x, _e11.g0_.x, _e14.g0_.z, _e17.g0_.y)) * vec4<f32>(1.0, 0.0, -(1.0), 1.0)) + ((vec4<f32>(_e29.g0_.z) * vec4<f32>(_e33.g0_.y, _e36.g0_.z, _e39.g0_.y, _e42.g0_.y)) * vec4<f32>(1.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e54.g0_.w) * vec4<f32>(_e58.g0_.z, _e61.g0_.y, _e64.g0_.z, _e67.g0_.z)) * vec4<f32>(1.0, -(1.0), 0.0, 0.0))) + ((vec4<f32>(_e80.g0_.x) * vec4<f32>(_e84.g0_.x, _e87.g0_.x, _e90.g0_.y, _e93.g0_.z)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn motor_plane_regressive_product(self_1660: Motor, other_1392: Plane) -> Scalar {
    var self_1661: Motor;
    var other_1393: Plane;

    self_1661 = self_1660;
    other_1393 = other_1392;
    let _e4: Motor = self_1661;
    let _e7: Plane = other_1393;
    let _e11: Motor = self_1661;
    let _e14: Plane = other_1393;
    let _e19: Motor = self_1661;
    let _e22: Plane = other_1393;
    return Scalar((((_e4.g0_.y * _e7.g0_.x) + (_e11.g0_.z * _e14.g0_.y)) + (_e19.g0_.w * _e22.g0_.z)));
}

fn motor_plane_outer_product(self_1662: Motor, other_1394: Plane) -> MotorDual {
    var self_1663: Motor;
    var other_1395: Plane;

    self_1663 = self_1662;
    other_1395 = other_1394;
    let _e4: Motor = self_1663;
    let _e8: Plane = other_1395;
    let _e19: Motor = self_1663;
    let _e23: Plane = other_1395;
    let _e35: Motor = self_1663;
    let _e38: Plane = other_1395;
    let _e41: Plane = other_1395;
    let _e44: Plane = other_1395;
    let _e47: Plane = other_1395;
    return MotorDual(((((vec4<f32>(_e4.g0_.z) * vec4<f32>(_e8.g0_.y)) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) + ((vec4<f32>(_e19.g0_.w) * vec4<f32>(_e23.g0_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + (_e35.g0_.yxxx * vec4<f32>(_e38.g0_.x, _e41.g0_.x, _e44.g0_.y, _e47.g0_.z))));
}

fn motor_plane_inner_product(self_1664: Motor, other_1396: Plane) -> Plane {
    var self_1665: Motor;
    var other_1397: Plane;

    self_1665 = self_1664;
    other_1397 = other_1396;
    let _e4: Motor = self_1665;
    let _e8: Plane = other_1397;
    let _e11: Motor = self_1665;
    let _e15: Plane = other_1397;
    let _e27: Motor = self_1665;
    let _e30: Motor = self_1665;
    let _e33: Motor = self_1665;
    let _e37: Plane = other_1397;
    return Plane((((vec3<f32>(_e4.g0_.x) * _e8.g0_) + ((vec3<f32>(_e11.g0_.w) * vec3<f32>(_e15.g0_.y)) * vec3<f32>(-(1.0), 0.0, 0.0))) + ((vec3<f32>(_e27.g0_.z, _e30.g0_.y, _e33.g0_.y) * _e37.g0_.zzy) * vec3<f32>(1.0, -(1.0), 1.0))));
}

fn motor_plane_geometric_anti_product(self_1666: Motor, other_1398: Plane) -> Motor {
    var self_1667: Motor;
    var other_1399: Plane;

    self_1667 = self_1666;
    other_1399 = other_1398;
    let _e4: Motor = self_1667;
    let _e8: Plane = other_1399;
    let _e11: Plane = other_1399;
    let _e14: Plane = other_1399;
    let _e17: Plane = other_1399;
    let _e29: Motor = self_1667;
    let _e33: Plane = other_1399;
    let _e36: Plane = other_1399;
    let _e39: Plane = other_1399;
    let _e42: Plane = other_1399;
    let _e55: Motor = self_1667;
    let _e58: Plane = other_1399;
    return Motor(((((vec4<f32>(_e4.g0_.z) * vec4<f32>(_e8.g0_.y, _e11.g0_.z, _e14.g0_.y, _e17.g0_.x)) * vec4<f32>(1.0, 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e29.g0_.w) * vec4<f32>(_e33.g0_.z, _e36.g0_.y, _e39.g0_.x, _e42.g0_.z)) * vec4<f32>(1.0, -(1.0), 1.0, 0.0))) + ((_e55.g0_.yxxx * vec4<f32>(_e58.g0_.x)) * vec4<f32>(1.0, -(1.0), 0.0, 0.0))));
}

fn motor_plane_inner_anti_product(self_1668: Motor, other_1400: Plane) -> Point {
    var self_1669: Motor;
    var other_1401: Plane;

    self_1669 = self_1668;
    other_1401 = other_1400;
    let _e4: Motor = self_1669;
    let _e8: Plane = other_1401;
    let _e18: Motor = self_1669;
    let _e22: Plane = other_1401;
    let _e33: Motor = self_1669;
    let _e37: Plane = other_1401;
    return Point(((((vec3<f32>(_e4.g0_.z) * _e8.g0_.zzx) * vec3<f32>(1.0, 0.0, -(1.0))) + ((vec3<f32>(_e18.g0_.w) * _e22.g0_.yxy) * vec3<f32>(-(1.0), 1.0, 0.0))) + ((vec3<f32>(_e33.g0_.x) * vec3<f32>(_e37.g0_.x)) * vec3<f32>(-(1.0), 0.0, 0.0))));
}

fn motor_plane_left_contraction(self_1670: Motor, other_1402: Plane) -> Plane {
    var self_1671: Motor;
    var other_1403: Plane;

    self_1671 = self_1670;
    other_1403 = other_1402;
    let _e4: Motor = self_1671;
    let _e8: Plane = other_1403;
    return Plane((vec3<f32>(_e4.g0_.x) * _e8.g0_));
}

fn motor_plane_right_contraction(self_1672: Motor, other_1404: Plane) -> Plane {
    var self_1673: Motor;
    var other_1405: Plane;

    self_1673 = self_1672;
    other_1405 = other_1404;
    let _e4: Motor = self_1673;
    let _e8: Plane = other_1405;
    let _e19: Motor = self_1673;
    let _e22: Motor = self_1673;
    let _e25: Motor = self_1673;
    let _e29: Plane = other_1405;
    return Plane((((vec3<f32>(_e4.g0_.w) * vec3<f32>(_e8.g0_.y)) * vec3<f32>(-(1.0), 0.0, 0.0)) + ((vec3<f32>(_e19.g0_.z, _e22.g0_.y, _e25.g0_.y) * _e29.g0_.zzy) * vec3<f32>(1.0, -(1.0), 1.0))));
}

fn motor_plane_left_anti_contraction(self_1674: Motor, other_1406: Plane) -> Point {
    var self_1675: Motor;
    var other_1407: Plane;

    self_1675 = self_1674;
    other_1407 = other_1406;
    let _e4: Motor = self_1675;
    let _e8: Plane = other_1407;
    let _e18: Motor = self_1675;
    let _e21: Motor = self_1675;
    let _e24: Motor = self_1675;
    let _e28: Plane = other_1407;
    return Point((((vec3<f32>(_e4.g0_.w) * _e8.g0_.yxy) * vec3<f32>(-(1.0), 1.0, 0.0)) + ((vec3<f32>(_e18.g0_.z, _e21.g0_.x, _e24.g0_.z) * _e28.g0_.zxx) * vec3<f32>(1.0, 0.0, -(1.0)))));
}

fn motor_translator_into(self_1676: Motor) -> Translator {
    var self_1677: Motor;

    self_1677 = self_1676;
    let _e2: Motor = self_1677;
    let _e5: Motor = self_1677;
    let _e8: Motor = self_1677;
    return Translator(vec3<f32>(_e2.g0_.x, _e5.g0_.z, _e8.g0_.w));
}

fn motor_translator_add(self_1678: Motor, other_1408: Translator) -> Motor {
    var self_1679: Motor;
    var other_1409: Translator;

    self_1679 = self_1678;
    other_1409 = other_1408;
    let _e4: Motor = self_1679;
    let _e6: Translator = other_1409;
    let _e9: Translator = other_1409;
    let _e12: Translator = other_1409;
    let _e15: Translator = other_1409;
    return Motor((_e4.g0_ + (vec4<f32>(_e6.g0_.x, _e9.g0_.x, _e12.g0_.y, _e15.g0_.z) * vec4<f32>(1.0, 0.0, 1.0, 1.0))));
}

fn motor_translator_sub(self_1680: Motor, other_1410: Translator) -> Motor {
    var self_1681: Motor;
    var other_1411: Translator;

    self_1681 = self_1680;
    other_1411 = other_1410;
    let _e4: Motor = self_1681;
    let _e6: Translator = other_1411;
    let _e9: Translator = other_1411;
    let _e12: Translator = other_1411;
    let _e15: Translator = other_1411;
    return Motor((_e4.g0_ - (vec4<f32>(_e6.g0_.x, _e9.g0_.x, _e12.g0_.y, _e15.g0_.z) * vec4<f32>(1.0, 0.0, 1.0, 1.0))));
}

fn motor_translator_geometric_product(self_1682: Motor, other_1412: Translator) -> Motor {
    var self_1683: Motor;
    var other_1413: Translator;

    self_1683 = self_1682;
    other_1413 = other_1412;
    let _e4: Motor = self_1683;
    let _e8: Translator = other_1413;
    let _e11: Translator = other_1413;
    let _e14: Translator = other_1413;
    let _e17: Translator = other_1413;
    let _e29: Motor = self_1683;
    let _e33: Translator = other_1413;
    let _e45: Motor = self_1683;
    let _e49: Translator = other_1413;
    let _e61: Motor = self_1683;
    let _e65: Translator = other_1413;
    let _e68: Translator = other_1413;
    let _e71: Translator = other_1413;
    let _e74: Translator = other_1413;
    return Motor((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.x, _e11.g0_.x, _e14.g0_.z, _e17.g0_.y)) * vec4<f32>(0.0, 1.0, -(1.0), 1.0)) + ((vec4<f32>(_e29.g0_.z) * vec4<f32>(_e33.g0_.x)) * vec4<f32>(0.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e45.g0_.w) * vec4<f32>(_e49.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e61.g0_.x) * vec4<f32>(_e65.g0_.x, _e68.g0_.x, _e71.g0_.y, _e74.g0_.z)) * vec4<f32>(1.0, 0.0, 1.0, 1.0))));
}

fn motor_translator_regressive_product(self_1684: Motor, other_1414: Translator) -> Plane {
    var self_1685: Motor;
    var other_1415: Translator;

    self_1685 = self_1684;
    other_1415 = other_1414;
    let _e4: Motor = self_1685;
    let _e8: Translator = other_1415;
    let _e18: Motor = self_1685;
    let _e21: Motor = self_1685;
    let _e24: Motor = self_1685;
    let _e28: Translator = other_1415;
    return Plane((((vec3<f32>(_e4.g0_.w) * vec3<f32>(_e8.g0_.y)) * vec3<f32>(1.0, 0.0, 0.0)) + ((vec3<f32>(_e18.g0_.z, _e21.g0_.y, _e24.g0_.y) * _e28.g0_.zzy) * vec3<f32>(-(1.0), 1.0, -(1.0)))));
}

fn motor_translator_outer_product(self_1686: Motor, other_1416: Translator) -> Motor {
    var self_1687: Motor;
    var other_1417: Translator;

    self_1687 = self_1686;
    other_1417 = other_1416;
    let _e4: Motor = self_1687;
    let _e8: Translator = other_1417;
    let _e19: Motor = self_1687;
    let _e23: Translator = other_1417;
    let _e35: Motor = self_1687;
    let _e38: Translator = other_1417;
    let _e41: Translator = other_1417;
    let _e44: Translator = other_1417;
    let _e47: Translator = other_1417;
    return Motor(((((vec4<f32>(_e4.g0_.z) * vec4<f32>(_e8.g0_.x)) * vec4<f32>(0.0, 0.0, 1.0, 0.0)) + ((vec4<f32>(_e19.g0_.w) * vec4<f32>(_e23.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + (_e35.g0_.xyxx * vec4<f32>(_e38.g0_.x, _e41.g0_.x, _e44.g0_.y, _e47.g0_.z))));
}

fn motor_translator_inner_product(self_1688: Motor, other_1418: Translator) -> Motor {
    var self_1689: Motor;
    var other_1419: Translator;

    self_1689 = self_1688;
    other_1419 = other_1418;
    let _e4: Motor = self_1689;
    let _e8: Translator = other_1419;
    let _e19: Motor = self_1689;
    let _e23: Translator = other_1419;
    let _e35: Motor = self_1689;
    let _e38: Translator = other_1419;
    let _e41: Translator = other_1419;
    let _e44: Translator = other_1419;
    let _e47: Translator = other_1419;
    return Motor(((((vec4<f32>(_e4.g0_.z) * vec4<f32>(_e8.g0_.x)) * vec4<f32>(0.0, 0.0, 1.0, 0.0)) + ((vec4<f32>(_e19.g0_.w) * vec4<f32>(_e23.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + (_e35.g0_.xyxx * vec4<f32>(_e38.g0_.x, _e41.g0_.x, _e44.g0_.y, _e47.g0_.z))));
}

fn motor_translator_geometric_anti_product(self_1690: Motor, other_1420: Translator) -> MotorDual {
    var self_1691: Motor;
    var other_1421: Translator;

    self_1691 = self_1690;
    other_1421 = other_1420;
    let _e4: Motor = self_1691;
    let _e8: Translator = other_1421;
    let _e11: Translator = other_1421;
    let _e14: Translator = other_1421;
    let _e17: Translator = other_1421;
    let _e29: Motor = self_1691;
    let _e33: Translator = other_1421;
    let _e36: Translator = other_1421;
    let _e39: Translator = other_1421;
    let _e42: Translator = other_1421;
    let _e55: Motor = self_1691;
    let _e59: Translator = other_1421;
    let _e62: Translator = other_1421;
    let _e65: Translator = other_1421;
    let _e68: Translator = other_1421;
    let _e80: Motor = self_1691;
    let _e84: Translator = other_1421;
    let _e87: Translator = other_1421;
    let _e90: Translator = other_1421;
    let _e93: Translator = other_1421;
    return MotorDual((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.z, _e11.g0_.z, _e14.g0_.z, _e17.g0_.y)) * vec4<f32>(0.0, 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e29.g0_.z) * vec4<f32>(_e33.g0_.y, _e36.g0_.z, _e39.g0_.x, _e42.g0_.y)) * vec4<f32>(1.0, -(1.0), 1.0, 0.0))) + ((vec4<f32>(_e55.g0_.w) * vec4<f32>(_e59.g0_.z, _e62.g0_.y, _e65.g0_.z, _e68.g0_.x)) * vec4<f32>(1.0, 1.0, 0.0, 1.0))) + ((vec4<f32>(_e80.g0_.x) * vec4<f32>(_e84.g0_.x, _e87.g0_.x, _e90.g0_.y, _e93.g0_.z)) * vec4<f32>(0.0, 0.0, 1.0, 1.0))));
}

fn motor_translator_left_contraction(self_1692: Motor, other_1422: Translator) -> Translator {
    var self_1693: Motor;
    var other_1423: Translator;

    self_1693 = self_1692;
    other_1423 = other_1422;
    let _e4: Motor = self_1693;
    let _e8: Translator = other_1423;
    return Translator((vec3<f32>(_e4.g0_.x) * _e8.g0_));
}

fn motor_translator_right_contraction(self_1694: Motor, other_1424: Translator) -> Motor {
    var self_1695: Motor;
    var other_1425: Translator;

    self_1695 = self_1694;
    other_1425 = other_1424;
    let _e4: Motor = self_1695;
    let _e6: Translator = other_1425;
    return Motor((_e4.g0_ * vec4<f32>(_e6.g0_.x)));
}

fn motor_translator_scalar_product(self_1696: Motor, other_1426: Translator) -> Scalar {
    var self_1697: Motor;
    var other_1427: Translator;

    self_1697 = self_1696;
    other_1427 = other_1426;
    let _e4: Motor = self_1697;
    let _e7: Translator = other_1427;
    return Scalar((_e4.g0_.x * _e7.g0_.x));
}

fn motor_translator_anti_scalar_product(self_1698: Motor, other_1428: Translator) -> AntiScalar {
    var self_1699: Motor;
    var other_1429: Translator;

    self_1699 = self_1698;
    other_1429 = other_1428;
    let _e4: Motor = self_1699;
    let _e7: Translator = other_1429;
    let _e11: Motor = self_1699;
    let _e14: Translator = other_1429;
    return AntiScalar(((_e4.g0_.z * _e7.g0_.y) + (_e11.g0_.w * _e14.g0_.z)));
}

fn motor_motor_add(self_1700: Motor, other_1430: Motor) -> Motor {
    var self_1701: Motor;
    var other_1431: Motor;

    self_1701 = self_1700;
    other_1431 = other_1430;
    let _e4: Motor = self_1701;
    let _e6: Motor = other_1431;
    return Motor((_e4.g0_ + _e6.g0_));
}

fn motor_motor_sub(self_1702: Motor, other_1432: Motor) -> Motor {
    var self_1703: Motor;
    var other_1433: Motor;

    self_1703 = self_1702;
    other_1433 = other_1432;
    let _e4: Motor = self_1703;
    let _e6: Motor = other_1433;
    return Motor((_e4.g0_ - _e6.g0_));
}

fn motor_motor_mul(self_1704: Motor, other_1434: Motor) -> Motor {
    var self_1705: Motor;
    var other_1435: Motor;

    self_1705 = self_1704;
    other_1435 = other_1434;
    let _e4: Motor = self_1705;
    let _e6: Motor = other_1435;
    return Motor((_e4.g0_ * _e6.g0_));
}

fn motor_motor_div(self_1706: Motor, other_1436: Motor) -> Motor {
    var self_1707: Motor;
    var other_1437: Motor;

    self_1707 = self_1706;
    other_1437 = other_1436;
    let _e4: Motor = self_1707;
    let _e7: Motor = self_1707;
    let _e10: Motor = self_1707;
    let _e13: Motor = self_1707;
    let _e23: Motor = other_1437;
    let _e26: Motor = other_1437;
    let _e29: Motor = other_1437;
    let _e32: Motor = other_1437;
    return Motor((((vec4<f32>(_e4.g0_.x, _e7.g0_.y, _e10.g0_.z, _e13.g0_.w) * vec4<f32>(1.0, 1.0, 1.0, 1.0)) / vec4<f32>(_e23.g0_.x, _e26.g0_.y, _e29.g0_.z, _e32.g0_.w)) * vec4<f32>(1.0, 1.0, 1.0, 1.0)));
}

fn motor_motor_geometric_product(self_1708: Motor, other_1438: Motor) -> Motor {
    var self_1709: Motor;
    var other_1439: Motor;

    self_1709 = self_1708;
    other_1439 = other_1438;
    let _e4: Motor = self_1709;
    let _e8: Motor = other_1439;
    let _e11: Motor = self_1709;
    let _e15: Motor = other_1439;
    let _e28: Motor = self_1709;
    let _e32: Motor = other_1439;
    let _e43: Motor = self_1709;
    let _e46: Motor = other_1439;
    return Motor(((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.y) * _e15.g0_.yxwz) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))) + ((vec4<f32>(_e28.g0_.w) * _e32.g0_.yyyx) * vec4<f32>(0.0, 0.0, 1.0, 1.0))) + ((_e43.g0_.xxzz * _e46.g0_.xxxy) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))));
}

fn motor_motor_regressive_product(self_1710: Motor, other_1440: Motor) -> Plane {
    var self_1711: Motor;
    var other_1441: Motor;

    self_1711 = self_1710;
    other_1441 = other_1440;
    let _e4: Motor = self_1711;
    let _e8: Motor = other_1441;
    let _e11: Motor = other_1441;
    let _e14: Motor = other_1441;
    let _e25: Motor = self_1711;
    let _e29: Motor = other_1441;
    let _e32: Motor = other_1441;
    let _e35: Motor = other_1441;
    let _e47: Motor = self_1711;
    let _e50: Motor = self_1711;
    let _e53: Motor = self_1711;
    let _e57: Motor = other_1441;
    let _e60: Motor = other_1441;
    let _e63: Motor = other_1441;
    return Plane(((((vec3<f32>(_e4.g0_.z) * vec3<f32>(_e8.g0_.w, _e11.g0_.w, _e14.g0_.y)) * vec3<f32>(-(1.0), 0.0, 1.0)) + ((vec3<f32>(_e25.g0_.w) * vec3<f32>(_e29.g0_.z, _e32.g0_.y, _e35.g0_.z)) * vec3<f32>(1.0, -(1.0), 0.0))) + ((vec3<f32>(_e47.g0_.x, _e50.g0_.y, _e53.g0_.y) * vec3<f32>(_e57.g0_.x, _e60.g0_.w, _e63.g0_.z)) * vec3<f32>(0.0, 1.0, -(1.0)))));
}

fn motor_motor_outer_product(self_1712: Motor, other_1442: Motor) -> Motor {
    var self_1713: Motor;
    var other_1443: Motor;

    self_1713 = self_1712;
    other_1443 = other_1442;
    let _e4: Motor = self_1713;
    let _e8: Motor = other_1443;
    let _e11: Motor = self_1713;
    let _e13: Motor = other_1443;
    return Motor(((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((_e11.g0_ * vec4<f32>(_e13.g0_.x)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn motor_motor_inner_product(self_1714: Motor, other_1444: Motor) -> Motor {
    var self_1715: Motor;
    var other_1445: Motor;

    self_1715 = self_1714;
    other_1445 = other_1444;
    let _e4: Motor = self_1715;
    let _e8: Motor = other_1445;
    let _e11: Motor = self_1715;
    let _e14: Motor = other_1445;
    return Motor(((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((_e11.g0_.yyzw * _e14.g0_.yxxx) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0))));
}

fn motor_motor_geometric_anti_product(self_1716: Motor, other_1446: Motor) -> MotorDual {
    var self_1717: Motor;
    var other_1447: Motor;

    self_1717 = self_1716;
    other_1447 = other_1446;
    let _e4: Motor = self_1717;
    let _e8: Motor = other_1447;
    let _e19: Motor = self_1717;
    let _e23: Motor = other_1447;
    let _e35: Motor = self_1717;
    let _e39: Motor = other_1447;
    let _e51: Motor = self_1717;
    let _e55: Motor = other_1447;
    return MotorDual((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.zwxy) * vec4<f32>(1.0, -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e35.g0_.w) * _e39.g0_.wzyx) * vec4<f32>(1.0, 1.0, -(1.0), 1.0))) + ((vec4<f32>(_e51.g0_.x) * _e55.g0_.xxzw) * vec4<f32>(0.0, 0.0, 1.0, 1.0))));
}

fn motor_motor_left_contraction(self_1718: Motor, other_1448: Motor) -> Motor {
    var self_1719: Motor;
    var other_1449: Motor;

    self_1719 = self_1718;
    other_1449 = other_1448;
    let _e4: Motor = self_1719;
    let _e8: Motor = other_1449;
    let _e11: Motor = self_1719;
    let _e14: Motor = other_1449;
    return Motor(((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((_e11.g0_.yxxx * _e14.g0_.yxxx) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))));
}

fn motor_motor_right_contraction(self_1720: Motor, other_1450: Motor) -> Motor {
    var self_1721: Motor;
    var other_1451: Motor;

    self_1721 = self_1720;
    other_1451 = other_1450;
    let _e4: Motor = self_1721;
    let _e8: Motor = other_1451;
    let _e19: Motor = self_1721;
    let _e22: Motor = other_1451;
    return Motor((((vec4<f32>(_e4.g0_.y) * _e8.g0_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0)) + ((_e19.g0_.xxzw * vec4<f32>(_e22.g0_.x)) * vec4<f32>(1.0, 0.0, 1.0, 1.0))));
}

fn motor_motor_scalar_product(self_1722: Motor, other_1452: Motor) -> Scalar {
    var self_1723: Motor;
    var other_1453: Motor;

    self_1723 = self_1722;
    other_1453 = other_1452;
    let _e4: Motor = self_1723;
    let _e7: Motor = other_1453;
    let _e11: Motor = self_1723;
    let _e14: Motor = other_1453;
    return Scalar(((_e4.g0_.x * _e7.g0_.x) - (_e11.g0_.y * _e14.g0_.y)));
}

fn motor_motor_anti_scalar_product(self_1724: Motor, other_1454: Motor) -> AntiScalar {
    var self_1725: Motor;
    var other_1455: Motor;

    self_1725 = self_1724;
    other_1455 = other_1454;
    let _e4: Motor = self_1725;
    let _e7: Motor = other_1455;
    let _e11: Motor = self_1725;
    let _e14: Motor = other_1455;
    return AntiScalar(((_e4.g0_.z * _e7.g0_.z) + (_e11.g0_.w * _e14.g0_.w)));
}

fn motor_motor_dual_add(self_1726: Motor, other_1456: MotorDual) -> MultiVector {
    var self_1727: Motor;
    var other_1457: MotorDual;

    self_1727 = self_1726;
    other_1457 = other_1456;
    let _e4: Motor = self_1727;
    let _e13: MotorDual = other_1457;
    let _e23: Motor = self_1727;
    let _e32: MotorDual = other_1457;
    return MultiVector(((_e4.g0_.xyxx * vec4<f32>(1.0, 1.0, 0.0, 0.0)) + (_e13.g0_.xxwz * vec4<f32>(0.0, 0.0, 1.0, 1.0))), ((_e23.g0_.xxzw * vec4<f32>(0.0, 0.0, 1.0, 1.0)) + (_e32.g0_.yxxx * vec4<f32>(1.0, 1.0, 0.0, 0.0))));
}

fn motor_motor_dual_sub(self_1728: Motor, other_1458: MotorDual) -> MultiVector {
    var self_1729: Motor;
    var other_1459: MotorDual;

    self_1729 = self_1728;
    other_1459 = other_1458;
    let _e4: Motor = self_1729;
    let _e13: MotorDual = other_1459;
    let _e23: Motor = self_1729;
    let _e32: MotorDual = other_1459;
    return MultiVector(((_e4.g0_.xyxx * vec4<f32>(1.0, 1.0, 0.0, 0.0)) - (_e13.g0_.xxwz * vec4<f32>(0.0, 0.0, 1.0, 1.0))), ((_e23.g0_.xxzw * vec4<f32>(0.0, 0.0, 1.0, 1.0)) - (_e32.g0_.yxxx * vec4<f32>(1.0, 1.0, 0.0, 0.0))));
}

fn motor_motor_dual_geometric_product(self_1730: Motor, other_1460: MotorDual) -> MotorDual {
    var self_1731: Motor;
    var other_1461: MotorDual;

    self_1731 = self_1730;
    other_1461 = other_1460;
    let _e4: Motor = self_1731;
    let _e8: MotorDual = other_1461;
    let _e11: Motor = self_1731;
    let _e15: MotorDual = other_1461;
    let _e28: Motor = self_1731;
    let _e32: MotorDual = other_1461;
    let _e44: Motor = self_1731;
    let _e47: MotorDual = other_1461;
    return MotorDual(((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.y) * _e15.g0_.yxwz) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e28.g0_.w) * _e32.g0_.wzww) * vec4<f32>(1.0, -(1.0), 0.0, 0.0))) + ((_e44.g0_.zzxx * _e47.g0_.zwxx) * vec4<f32>(1.0, 1.0, 0.0, 0.0))));
}

fn motor_motor_dual_regressive_product(self_1732: Motor, other_1462: MotorDual) -> Motor {
    var self_1733: Motor;
    var other_1463: MotorDual;

    self_1733 = self_1732;
    other_1463 = other_1462;
    let _e4: Motor = self_1733;
    let _e8: MotorDual = other_1463;
    let _e18: Motor = self_1733;
    let _e22: MotorDual = other_1463;
    let _e33: Motor = self_1733;
    let _e37: MotorDual = other_1463;
    let _e48: Motor = self_1733;
    let _e52: MotorDual = other_1463;
    return Motor((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.yxyy) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) + ((vec4<f32>(_e18.g0_.z) * _e22.g0_.zzxz) * vec4<f32>(1.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e33.g0_.w) * _e37.g0_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e48.g0_.x) * vec4<f32>(_e52.g0_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))));
}

fn motor_motor_dual_outer_product(self_1734: Motor, other_1464: MotorDual) -> MotorDual {
    var self_1735: Motor;
    var other_1465: MotorDual;

    self_1735 = self_1734;
    other_1465 = other_1464;
    let _e4: Motor = self_1735;
    let _e8: MotorDual = other_1465;
    let _e11: Motor = self_1735;
    let _e15: MotorDual = other_1465;
    let _e27: Motor = self_1735;
    let _e31: MotorDual = other_1465;
    let _e43: Motor = self_1735;
    let _e46: MotorDual = other_1465;
    return MotorDual(((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.z) * vec4<f32>(_e15.g0_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e27.g0_.w) * vec4<f32>(_e31.g0_.w)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((_e43.g0_.yxxx * _e46.g0_.yxxx) * vec4<f32>(1.0, 0.0, 0.0, 0.0))));
}

fn motor_motor_dual_inner_product(self_1736: Motor, other_1466: MotorDual) -> MotorDual {
    var self_1737: Motor;
    var other_1467: MotorDual;

    self_1737 = self_1736;
    other_1467 = other_1466;
    let _e4: Motor = self_1737;
    let _e8: MotorDual = other_1467;
    let _e11: Motor = self_1737;
    let _e15: MotorDual = other_1467;
    let _e27: Motor = self_1737;
    let _e31: MotorDual = other_1467;
    let _e44: Motor = self_1737;
    let _e47: MotorDual = other_1467;
    return MotorDual(((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.z) * vec4<f32>(_e15.g0_.w)) * vec4<f32>(0.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e27.g0_.w) * vec4<f32>(_e31.g0_.z)) * vec4<f32>(0.0, -(1.0), 0.0, 0.0))) + ((_e44.g0_.xyyy * _e47.g0_.xxwz) * vec4<f32>(0.0, -(1.0), -(1.0), 1.0))));
}

fn motor_motor_dual_geometric_anti_product(self_1738: Motor, other_1468: MotorDual) -> Motor {
    var self_1739: Motor;
    var other_1469: MotorDual;

    self_1739 = self_1738;
    other_1469 = other_1468;
    let _e4: Motor = self_1739;
    let _e8: MotorDual = other_1469;
    let _e18: Motor = self_1739;
    let _e22: MotorDual = other_1469;
    let _e34: Motor = self_1739;
    let _e38: MotorDual = other_1469;
    let _e50: Motor = self_1739;
    let _e54: MotorDual = other_1469;
    return Motor((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.yxyy) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) + ((vec4<f32>(_e18.g0_.z) * _e22.g0_.zwxy) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e34.g0_.w) * _e38.g0_.wzyx) * vec4<f32>(1.0, -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e50.g0_.x) * _e54.g0_.xyxx) * vec4<f32>(1.0, -(1.0), 0.0, 0.0))));
}

fn motor_motor_dual_inner_anti_product(self_1740: Motor, other_1470: MotorDual) -> Motor {
    var self_1741: Motor;
    var other_1471: MotorDual;

    self_1741 = self_1740;
    other_1471 = other_1470;
    let _e4: Motor = self_1741;
    let _e8: MotorDual = other_1471;
    let _e19: Motor = self_1741;
    let _e23: MotorDual = other_1471;
    let _e35: Motor = self_1741;
    let _e39: MotorDual = other_1471;
    let _e51: Motor = self_1741;
    let _e55: MotorDual = other_1471;
    return Motor((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.x)) * vec4<f32>(0.0, 1.0, 0.0, 0.0)) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.wwxy) * vec4<f32>(0.0, 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e35.g0_.w) * _e39.g0_.zzyx) * vec4<f32>(0.0, -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e51.g0_.x) * _e55.g0_.xyxx) * vec4<f32>(1.0, -(1.0), 0.0, 0.0))));
}

fn motor_motor_dual_left_contraction(self_1742: Motor, other_1472: MotorDual) -> MotorDual {
    var self_1743: Motor;
    var other_1473: MotorDual;

    self_1743 = self_1742;
    other_1473 = other_1472;
    let _e4: Motor = self_1743;
    let _e8: MotorDual = other_1473;
    let _e11: Motor = self_1743;
    let _e14: MotorDual = other_1473;
    return MotorDual(((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((_e11.g0_.xyxx * vec4<f32>(_e14.g0_.x)) * vec4<f32>(0.0, -(1.0), 0.0, 0.0))));
}

fn motor_motor_dual_right_contraction(self_1744: Motor, other_1474: MotorDual) -> Plane {
    var self_1745: Motor;
    var other_1475: MotorDual;

    self_1745 = self_1744;
    other_1475 = other_1474;
    let _e4: Motor = self_1745;
    let _e8: MotorDual = other_1475;
    let _e19: Motor = self_1745;
    let _e22: Motor = self_1745;
    let _e25: Motor = self_1745;
    let _e29: MotorDual = other_1475;
    let _e32: MotorDual = other_1475;
    let _e35: MotorDual = other_1475;
    return Plane((((vec3<f32>(_e4.g0_.w) * vec3<f32>(_e8.g0_.z)) * vec3<f32>(-(1.0), 0.0, 0.0)) + ((vec3<f32>(_e19.g0_.z, _e22.g0_.y, _e25.g0_.y) * vec3<f32>(_e29.g0_.w, _e32.g0_.w, _e35.g0_.z)) * vec3<f32>(1.0, -(1.0), 1.0))));
}

fn motor_motor_dual_left_anti_contraction(self_1746: Motor, other_1476: MotorDual) -> Point {
    var self_1747: Motor;
    var other_1477: MotorDual;

    self_1747 = self_1746;
    other_1477 = other_1476;
    let _e4: Motor = self_1747;
    let _e8: MotorDual = other_1477;
    let _e11: MotorDual = other_1477;
    let _e14: MotorDual = other_1477;
    let _e25: Motor = self_1747;
    let _e28: Motor = self_1747;
    let _e31: Motor = self_1747;
    let _e35: MotorDual = other_1477;
    let _e38: MotorDual = other_1477;
    let _e41: MotorDual = other_1477;
    return Point((((vec3<f32>(_e4.g0_.w) * vec3<f32>(_e8.g0_.z, _e11.g0_.y, _e14.g0_.z)) * vec3<f32>(-(1.0), 1.0, 0.0)) + ((vec3<f32>(_e25.g0_.z, _e28.g0_.x, _e31.g0_.z) * vec3<f32>(_e35.g0_.w, _e38.g0_.x, _e41.g0_.y)) * vec3<f32>(1.0, 0.0, -(1.0)))));
}

fn motor_motor_dual_right_anti_contraction(self_1748: Motor, other_1478: MotorDual) -> Motor {
    var self_1749: Motor;
    var other_1479: MotorDual;

    self_1749 = self_1748;
    other_1479 = other_1478;
    let _e4: Motor = self_1749;
    let _e8: MotorDual = other_1479;
    let _e19: Motor = self_1749;
    let _e22: MotorDual = other_1479;
    return Motor((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.x)) * vec4<f32>(0.0, 1.0, 0.0, 0.0)) + ((_e19.g0_.xxzw * _e22.g0_.xyxx) * vec4<f32>(1.0, -(1.0), 1.0, 1.0))));
}

fn motor_squared_magnitude(self_1750: Motor) -> Scalar {
    var self_1751: Motor;

    self_1751 = self_1750;
    let _e2: Motor = self_1751;
    let _e3: Motor = self_1751;
    let _e4: Motor = motor_reversal(_e3);
    let _e5: Scalar = motor_motor_scalar_product(_e2, _e4);
    return _e5;
}

fn motor_magnitude(self_1752: Motor) -> Scalar {
    var self_1753: Motor;

    self_1753 = self_1752;
    let _e2: Motor = self_1753;
    let _e3: Scalar = motor_squared_magnitude(_e2);
    return Scalar(sqrt(_e3.g0_));
}

fn motor_bulk_norm(self_1754: Motor) -> Scalar {
    var self_1755: Motor;

    self_1755 = self_1754;
    let _e2: Motor = self_1755;
    let _e3: Scalar = motor_squared_magnitude(_e2);
    return Scalar(sqrt(_e3.g0_));
}

fn motor_squared_anti_magnitude(self_1756: Motor) -> AntiScalar {
    var self_1757: Motor;

    self_1757 = self_1756;
    let _e2: Motor = self_1757;
    let _e3: Motor = self_1757;
    let _e4: Motor = motor_anti_reversal(_e3);
    let _e5: AntiScalar = motor_motor_anti_scalar_product(_e2, _e4);
    return _e5;
}

fn motor_weight_norm(self_1758: Motor) -> AntiScalar {
    var self_1759: Motor;

    self_1759 = self_1758;
    let _e2: Motor = self_1759;
    let _e3: AntiScalar = motor_squared_anti_magnitude(_e2);
    return AntiScalar(sqrt(_e3.g0_));
}

fn motor_scale(self_1760: Motor, other_1480: f32) -> Motor {
    var self_1761: Motor;
    var other_1481: f32;

    self_1761 = self_1760;
    other_1481 = other_1480;
    let _e4: Motor = self_1761;
    let _e5: f32 = other_1481;
    let _e7: Motor = motor_scalar_geometric_product(_e4, Scalar(_e5));
    return _e7;
}

fn motor_signum(self_1762: Motor) -> Motor {
    var self_1763: Motor;

    self_1763 = self_1762;
    let _e2: Motor = self_1763;
    let _e3: Motor = self_1763;
    let _e4: Scalar = motor_magnitude(_e3);
    let _e9: Motor = motor_scalar_geometric_product(_e2, Scalar((1.0 / _e4.g0_)));
    return _e9;
}

fn motor_inverse(self_1764: Motor) -> Motor {
    var self_1765: Motor;

    self_1765 = self_1764;
    let _e2: Motor = self_1765;
    let _e3: Motor = motor_reversal(_e2);
    let _e4: Motor = self_1765;
    let _e5: Scalar = motor_squared_magnitude(_e4);
    let _e10: Motor = motor_scalar_geometric_product(_e3, Scalar((1.0 / _e5.g0_)));
    return _e10;
}

fn motor_unitize(self_1766: Motor) -> Motor {
    var self_1767: Motor;

    self_1767 = self_1766;
    let _e2: Motor = self_1767;
    let _e3: Motor = self_1767;
    let _e4: AntiScalar = motor_weight_norm(_e3);
    let _e9: Motor = motor_scalar_geometric_product(_e2, Scalar((1.0 / _e4.g0_)));
    return _e9;
}

fn motor_dual_zero() -> MotorDual {
    return MotorDual(vec4<f32>(0.0));
}

fn motor_dual_one() -> MotorDual {
    return MotorDual(vec4<f32>(0.0));
}

fn motor_dual_neg(self_1768: MotorDual) -> MotorDual {
    var self_1769: MotorDual;

    self_1769 = self_1768;
    let _e2: MotorDual = self_1769;
    return MotorDual((_e2.g0_ * vec4<f32>(-(1.0))));
}

fn motor_dual_automorphism(self_1770: MotorDual) -> MotorDual {
    var self_1771: MotorDual;

    self_1771 = self_1770;
    let _e2: MotorDual = self_1771;
    return MotorDual((_e2.g0_ * vec4<f32>(-(1.0))));
}

fn motor_dual_reversal(self_1772: MotorDual) -> MotorDual {
    var self_1773: MotorDual;

    self_1773 = self_1772;
    let _e2: MotorDual = self_1773;
    return MotorDual((_e2.g0_ * vec4<f32>(-(1.0), 1.0, 1.0, 1.0)));
}

fn motor_dual_conjugation(self_1774: MotorDual) -> MotorDual {
    var self_1775: MotorDual;

    self_1775 = self_1774;
    let _e2: MotorDual = self_1775;
    return MotorDual((_e2.g0_ * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))));
}

fn motor_dual_dual(self_1776: MotorDual) -> Motor {
    var self_1777: MotorDual;

    self_1777 = self_1776;
    let _e2: MotorDual = self_1777;
    return Motor(_e2.g0_);
}

fn motor_dual_anti_reversal(self_1778: MotorDual) -> MotorDual {
    var self_1779: MotorDual;

    self_1779 = self_1778;
    let _e2: MotorDual = self_1779;
    return MotorDual((_e2.g0_ * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))));
}

fn motor_dual_right_complement(self_1780: MotorDual) -> Motor {
    var self_1781: MotorDual;

    self_1781 = self_1780;
    let _e2: MotorDual = self_1781;
    return Motor(_e2.g0_);
}

fn motor_dual_left_complement(self_1782: MotorDual) -> Motor {
    var self_1783: MotorDual;

    self_1783 = self_1782;
    let _e2: MotorDual = self_1783;
    return Motor(_e2.g0_);
}

fn motor_dual_double_complement(self_1784: MotorDual) -> MotorDual {
    var self_1785: MotorDual;

    self_1785 = self_1784;
    let _e2: MotorDual = self_1785;
    return MotorDual(_e2.g0_);
}

fn motor_dual_scalar_geometric_product(self_1786: MotorDual, other_1482: Scalar) -> MotorDual {
    var self_1787: MotorDual;
    var other_1483: Scalar;

    self_1787 = self_1786;
    other_1483 = other_1482;
    let _e4: MotorDual = self_1787;
    let _e6: Scalar = other_1483;
    return MotorDual((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn motor_dual_scalar_regressive_product(self_1788: MotorDual, other_1484: Scalar) -> Scalar {
    var self_1789: MotorDual;
    var other_1485: Scalar;

    self_1789 = self_1788;
    other_1485 = other_1484;
    let _e4: MotorDual = self_1789;
    let _e7: Scalar = other_1485;
    return Scalar((_e4.g0_.x * _e7.g0_));
}

fn motor_dual_scalar_outer_product(self_1790: MotorDual, other_1486: Scalar) -> MotorDual {
    var self_1791: MotorDual;
    var other_1487: Scalar;

    self_1791 = self_1790;
    other_1487 = other_1486;
    let _e4: MotorDual = self_1791;
    let _e6: Scalar = other_1487;
    return MotorDual((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn motor_dual_scalar_inner_product(self_1792: MotorDual, other_1488: Scalar) -> MotorDual {
    var self_1793: MotorDual;
    var other_1489: Scalar;

    self_1793 = self_1792;
    other_1489 = other_1488;
    let _e4: MotorDual = self_1793;
    let _e6: Scalar = other_1489;
    return MotorDual((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn motor_dual_scalar_geometric_anti_product(self_1794: MotorDual, other_1490: Scalar) -> Rotor {
    var self_1795: MotorDual;
    var other_1491: Scalar;

    self_1795 = self_1794;
    other_1491 = other_1490;
    let _e4: MotorDual = self_1795;
    let _e7: MotorDual = self_1795;
    let _e11: Scalar = other_1491;
    return Rotor(((vec2<f32>(_e4.g0_.x, _e7.g0_.y) * vec2<f32>(_e11.g0_)) * vec2<f32>(1.0, -(1.0))));
}

fn motor_dual_scalar_inner_anti_product(self_1796: MotorDual, other_1492: Scalar) -> Rotor {
    var self_1797: MotorDual;
    var other_1493: Scalar;

    self_1797 = self_1796;
    other_1493 = other_1492;
    let _e4: MotorDual = self_1797;
    let _e7: MotorDual = self_1797;
    let _e11: Scalar = other_1493;
    return Rotor(((vec2<f32>(_e4.g0_.x, _e7.g0_.y) * vec2<f32>(_e11.g0_)) * vec2<f32>(1.0, -(1.0))));
}

fn motor_dual_scalar_right_contraction(self_1798: MotorDual, other_1494: Scalar) -> MotorDual {
    var self_1799: MotorDual;
    var other_1495: Scalar;

    self_1799 = self_1798;
    other_1495 = other_1494;
    let _e4: MotorDual = self_1799;
    let _e6: Scalar = other_1495;
    return MotorDual((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn motor_dual_scalar_left_anti_contraction(self_1800: MotorDual, other_1496: Scalar) -> Rotor {
    var self_1801: MotorDual;
    var other_1497: Scalar;

    self_1801 = self_1800;
    other_1497 = other_1496;
    let _e4: MotorDual = self_1801;
    let _e7: MotorDual = self_1801;
    let _e11: Scalar = other_1497;
    return Rotor(((vec2<f32>(_e4.g0_.x, _e7.g0_.y) * vec2<f32>(_e11.g0_)) * vec2<f32>(1.0, -(1.0))));
}

fn motor_dual_anti_scalar_into(self_1802: MotorDual) -> AntiScalar {
    var self_1803: MotorDual;

    self_1803 = self_1802;
    let _e2: MotorDual = self_1803;
    return AntiScalar(_e2.g0_.x);
}

fn motor_dual_anti_scalar_add(self_1804: MotorDual, other_1498: AntiScalar) -> MotorDual {
    var self_1805: MotorDual;
    var other_1499: AntiScalar;

    self_1805 = self_1804;
    other_1499 = other_1498;
    let _e4: MotorDual = self_1805;
    let _e6: AntiScalar = other_1499;
    return MotorDual((_e4.g0_ + (vec4<f32>(_e6.g0_) * vec4<f32>(1.0, 0.0, 0.0, 0.0))));
}

fn motor_dual_anti_scalar_sub(self_1806: MotorDual, other_1500: AntiScalar) -> MotorDual {
    var self_1807: MotorDual;
    var other_1501: AntiScalar;

    self_1807 = self_1806;
    other_1501 = other_1500;
    let _e4: MotorDual = self_1807;
    let _e6: AntiScalar = other_1501;
    return MotorDual((_e4.g0_ - (vec4<f32>(_e6.g0_) * vec4<f32>(1.0, 0.0, 0.0, 0.0))));
}

fn motor_dual_anti_scalar_geometric_product(self_1808: MotorDual, other_1502: AntiScalar) -> IdealPoint {
    var self_1809: MotorDual;
    var other_1503: AntiScalar;

    self_1809 = self_1808;
    other_1503 = other_1502;
    let _e4: MotorDual = self_1809;
    let _e7: MotorDual = self_1809;
    let _e11: AntiScalar = other_1503;
    return IdealPoint((vec2<f32>(_e4.g0_.z, _e7.g0_.w) * vec2<f32>(_e11.g0_)));
}

fn motor_dual_anti_scalar_regressive_product(self_1810: MotorDual, other_1504: AntiScalar) -> MotorDual {
    var self_1811: MotorDual;
    var other_1505: AntiScalar;

    self_1811 = self_1810;
    other_1505 = other_1504;
    let _e4: MotorDual = self_1811;
    let _e6: AntiScalar = other_1505;
    return MotorDual((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn motor_dual_anti_scalar_inner_product(self_1812: MotorDual, other_1506: AntiScalar) -> IdealPoint {
    var self_1813: MotorDual;
    var other_1507: AntiScalar;

    self_1813 = self_1812;
    other_1507 = other_1506;
    let _e4: MotorDual = self_1813;
    let _e7: MotorDual = self_1813;
    let _e11: AntiScalar = other_1507;
    return IdealPoint((vec2<f32>(_e4.g0_.z, _e7.g0_.w) * vec2<f32>(_e11.g0_)));
}

fn motor_dual_anti_scalar_geometric_anti_product(self_1814: MotorDual, other_1508: AntiScalar) -> MotorDual {
    var self_1815: MotorDual;
    var other_1509: AntiScalar;

    self_1815 = self_1814;
    other_1509 = other_1508;
    let _e4: MotorDual = self_1815;
    let _e6: AntiScalar = other_1509;
    return MotorDual((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn motor_dual_anti_scalar_inner_anti_product(self_1816: MotorDual, other_1510: AntiScalar) -> MotorDual {
    var self_1817: MotorDual;
    var other_1511: AntiScalar;

    self_1817 = self_1816;
    other_1511 = other_1510;
    let _e4: MotorDual = self_1817;
    let _e6: AntiScalar = other_1511;
    return MotorDual((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn motor_dual_anti_scalar_left_contraction(self_1818: MotorDual, other_1512: AntiScalar) -> IdealPoint {
    var self_1819: MotorDual;
    var other_1513: AntiScalar;

    self_1819 = self_1818;
    other_1513 = other_1512;
    let _e4: MotorDual = self_1819;
    let _e7: MotorDual = self_1819;
    let _e11: AntiScalar = other_1513;
    return IdealPoint((vec2<f32>(_e4.g0_.z, _e7.g0_.w) * vec2<f32>(_e11.g0_)));
}

fn motor_dual_anti_scalar_left_anti_contraction(self_1820: MotorDual, other_1514: AntiScalar) -> AntiScalar {
    var self_1821: MotorDual;
    var other_1515: AntiScalar;

    self_1821 = self_1820;
    other_1515 = other_1514;
    let _e4: MotorDual = self_1821;
    let _e7: AntiScalar = other_1515;
    return AntiScalar((_e4.g0_.x * _e7.g0_));
}

fn motor_dual_anti_scalar_right_anti_contraction(self_1822: MotorDual, other_1516: AntiScalar) -> MotorDual {
    var self_1823: MotorDual;
    var other_1517: AntiScalar;

    self_1823 = self_1822;
    other_1517 = other_1516;
    let _e4: MotorDual = self_1823;
    let _e6: AntiScalar = other_1517;
    return MotorDual((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn motor_dual_anti_scalar_anti_scalar_product(self_1824: MotorDual, other_1518: AntiScalar) -> AntiScalar {
    var self_1825: MotorDual;
    var other_1519: AntiScalar;

    self_1825 = self_1824;
    other_1519 = other_1518;
    let _e4: MotorDual = self_1825;
    let _e7: AntiScalar = other_1519;
    return AntiScalar((_e4.g0_.x * _e7.g0_));
}

fn motor_dual_multi_vector_add(self_1826: MotorDual, other_1520: MultiVector) -> MultiVector {
    var self_1827: MotorDual;
    var other_1521: MultiVector;

    self_1827 = self_1826;
    other_1521 = other_1520;
    let _e4: MotorDual = self_1827;
    let _e13: MultiVector = other_1521;
    let _e16: MotorDual = self_1827;
    let _e25: MultiVector = other_1521;
    return MultiVector(((_e4.g0_.xxwz * vec4<f32>(0.0, 0.0, 1.0, 1.0)) + _e13.g0_), ((_e16.g0_.yxxx * vec4<f32>(1.0, 1.0, 0.0, 0.0)) + _e25.g1_));
}

fn motor_dual_multi_vector_sub(self_1828: MotorDual, other_1522: MultiVector) -> MultiVector {
    var self_1829: MotorDual;
    var other_1523: MultiVector;

    self_1829 = self_1828;
    other_1523 = other_1522;
    let _e4: MotorDual = self_1829;
    let _e13: MultiVector = other_1523;
    let _e16: MotorDual = self_1829;
    let _e25: MultiVector = other_1523;
    return MultiVector(((_e4.g0_.xxwz * vec4<f32>(0.0, 0.0, 1.0, 1.0)) - _e13.g0_), ((_e16.g0_.yxxx * vec4<f32>(1.0, 1.0, 0.0, 0.0)) - _e25.g1_));
}

fn motor_dual_multi_vector_geometric_product(self_1830: MotorDual, other_1524: MultiVector) -> MultiVector {
    var self_1831: MotorDual;
    var other_1525: MultiVector;

    self_1831 = self_1830;
    other_1525 = other_1524;
    let _e4: MotorDual = self_1831;
    let _e8: MultiVector = other_1525;
    let _e20: MotorDual = self_1831;
    let _e24: MultiVector = other_1525;
    let _e29: MotorDual = self_1831;
    let _e33: MultiVector = other_1525;
    let _e44: MotorDual = self_1831;
    let _e48: MultiVector = other_1525;
    let _e59: MotorDual = self_1831;
    let _e63: MultiVector = other_1525;
    let _e68: MotorDual = self_1831;
    let _e72: MultiVector = other_1525;
    return MultiVector((((vec4<f32>(_e4.g0_.z) * _e8.g0_.wzyx) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0)) + (vec4<f32>(_e20.g0_.w) * _e24.g0_.zwxy)), (((((vec4<f32>(_e29.g0_.x) * _e33.g0_.yxwz) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0)) + ((vec4<f32>(_e44.g0_.y) * _e48.g0_) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))) + (vec4<f32>(_e59.g0_.z) * _e63.g1_.wzyx)) + ((vec4<f32>(_e68.g0_.w) * _e72.g1_.zwxy) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))));
}

fn motor_dual_multi_vector_regressive_product(self_1832: MotorDual, other_1526: MultiVector) -> MultiVector {
    var self_1833: MotorDual;
    var other_1527: MultiVector;

    self_1833 = self_1832;
    other_1527 = other_1526;
    let _e4: MotorDual = self_1833;
    let _e8: MultiVector = other_1527;
    let _e11: MotorDual = self_1833;
    let _e15: MultiVector = other_1527;
    let _e26: MotorDual = self_1833;
    let _e30: MultiVector = other_1527;
    let _e41: MotorDual = self_1833;
    let _e44: MultiVector = other_1527;
    let _e55: MotorDual = self_1833;
    let _e59: MultiVector = other_1527;
    let _e62: MotorDual = self_1833;
    let _e65: MultiVector = other_1527;
    return MultiVector(((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.z) * _e15.g1_.zzzy) * vec4<f32>(1.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e26.g0_.w) * _e30.g1_.wwyw) * vec4<f32>(1.0, 0.0, 1.0, 0.0))) + ((_e41.g0_.yxxx * _e44.g0_.yxxx) * vec4<f32>(1.0, 0.0, 0.0, 0.0))), ((vec4<f32>(_e55.g0_.x) * _e59.g1_) + ((_e62.g0_.yxxx * _e65.g1_.yxxx) * vec4<f32>(1.0, 0.0, 0.0, 0.0))));
}

fn motor_dual_multi_vector_geometric_anti_product(self_1834: MotorDual, other_1528: MultiVector) -> MultiVector {
    var self_1835: MotorDual;
    var other_1529: MultiVector;

    self_1835 = self_1834;
    other_1529 = other_1528;
    let _e4: MotorDual = self_1835;
    let _e8: MultiVector = other_1529;
    let _e11: MotorDual = self_1835;
    let _e15: MultiVector = other_1529;
    let _e28: MotorDual = self_1835;
    let _e32: MultiVector = other_1529;
    let _e44: MotorDual = self_1835;
    let _e48: MultiVector = other_1529;
    let _e60: MotorDual = self_1835;
    let _e64: MultiVector = other_1529;
    let _e67: MotorDual = self_1835;
    let _e71: MultiVector = other_1529;
    return MultiVector(((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.y) * _e15.g0_.yxwz) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e28.g0_.z) * _e32.g1_.zwxy) * vec4<f32>(1.0, 1.0, -(1.0), 1.0))) + ((vec4<f32>(_e44.g0_.w) * _e48.g1_.wzyx) * vec4<f32>(1.0, -(1.0), 1.0, 1.0))), ((vec4<f32>(_e60.g0_.x) * _e64.g1_) + ((vec4<f32>(_e67.g0_.y) * _e71.g1_.yxwz) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0))));
}

fn motor_dual_multi_vector_inner_anti_product(self_1836: MotorDual, other_1530: MultiVector) -> MultiVector {
    var self_1837: MotorDual;
    var other_1531: MultiVector;

    self_1837 = self_1836;
    other_1531 = other_1530;
    let _e4: MotorDual = self_1837;
    let _e8: MultiVector = other_1531;
    let _e11: MotorDual = self_1837;
    let _e15: MultiVector = other_1531;
    let _e26: MotorDual = self_1837;
    let _e30: MultiVector = other_1531;
    let _e42: MotorDual = self_1837;
    let _e45: MultiVector = other_1531;
    let _e58: MotorDual = self_1837;
    let _e62: MultiVector = other_1531;
    let _e65: MotorDual = self_1837;
    let _e69: MultiVector = other_1531;
    return MultiVector(((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.z) * _e15.g1_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, 1.0))) + ((vec4<f32>(_e26.g0_.w) * _e30.g1_.zzyz) * vec4<f32>(0.0, -(1.0), 1.0, 0.0))) + ((_e42.g0_.xyxx * vec4<f32>(_e45.g0_.x)) * vec4<f32>(0.0, -(1.0), 0.0, 0.0))), ((vec4<f32>(_e58.g0_.x) * _e62.g1_) + ((vec4<f32>(_e65.g0_.y) * _e69.g1_.yxwz) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0))));
}

fn motor_dual_multi_vector_left_anti_contraction(self_1838: MotorDual, other_1532: MultiVector) -> MultiVector {
    var self_1839: MotorDual;
    var other_1533: MultiVector;

    self_1839 = self_1838;
    other_1533 = other_1532;
    let _e4: MotorDual = self_1839;
    let _e8: MultiVector = other_1533;
    let _e11: MotorDual = self_1839;
    let _e14: MultiVector = other_1533;
    let _e27: MotorDual = self_1839;
    let _e31: MultiVector = other_1533;
    let _e34: MotorDual = self_1839;
    let _e37: MultiVector = other_1533;
    return MultiVector(((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((_e11.g0_.xyxx * vec4<f32>(_e14.g0_.x)) * vec4<f32>(0.0, -(1.0), 0.0, 0.0))), ((vec4<f32>(_e27.g0_.x) * _e31.g1_) + ((_e34.g0_.xyxx * vec4<f32>(_e37.g1_.x)) * vec4<f32>(0.0, -(1.0), 0.0, 0.0))));
}

fn motor_dual_multi_vector_scalar_product(self_1840: MotorDual, other_1534: MultiVector) -> Scalar {
    var self_1841: MotorDual;
    var other_1535: MultiVector;

    self_1841 = self_1840;
    other_1535 = other_1534;
    let _e4: MotorDual = self_1841;
    let _e7: MultiVector = other_1535;
    let _e11: MotorDual = self_1841;
    let _e14: MultiVector = other_1535;
    return Scalar(((_e4.g0_.z * _e7.g0_.w) + (_e11.g0_.w * _e14.g0_.z)));
}

fn motor_dual_multi_vector_anti_scalar_product(self_1842: MotorDual, other_1536: MultiVector) -> AntiScalar {
    var self_1843: MotorDual;
    var other_1537: MultiVector;

    self_1843 = self_1842;
    other_1537 = other_1536;
    let _e4: MotorDual = self_1843;
    let _e7: MultiVector = other_1537;
    let _e11: MotorDual = self_1843;
    let _e14: MultiVector = other_1537;
    return AntiScalar(((_e4.g0_.x * _e7.g1_.y) - (_e11.g0_.y * _e14.g1_.x)));
}

fn motor_dual_rotor_geometric_product(self_1844: MotorDual, other_1538: Rotor) -> MotorDual {
    var self_1845: MotorDual;
    var other_1539: Rotor;

    self_1845 = self_1844;
    other_1539 = other_1538;
    let _e4: MotorDual = self_1845;
    let _e8: Rotor = other_1539;
    let _e11: Rotor = other_1539;
    let _e14: Rotor = other_1539;
    let _e17: Rotor = other_1539;
    let _e28: MotorDual = self_1845;
    let _e32: Rotor = other_1539;
    let _e35: Rotor = other_1539;
    let _e38: Rotor = other_1539;
    let _e41: Rotor = other_1539;
    let _e53: MotorDual = self_1845;
    let _e56: Rotor = other_1539;
    let _e59: Rotor = other_1539;
    let _e62: Rotor = other_1539;
    let _e65: Rotor = other_1539;
    return MotorDual(((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y, _e11.g0_.x, _e14.g0_.y, _e17.g0_.y)) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) + ((vec4<f32>(_e28.g0_.w) * vec4<f32>(_e32.g0_.y, _e35.g0_.y, _e38.g0_.y, _e41.g0_.x)) * vec4<f32>(0.0, 0.0, 1.0, 1.0))) + ((_e53.g0_.xxzz * vec4<f32>(_e56.g0_.x, _e59.g0_.y, _e62.g0_.x, _e65.g0_.y)) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))));
}

fn motor_dual_rotor_regressive_product(self_1846: MotorDual, other_1540: Rotor) -> Rotor {
    var self_1847: MotorDual;
    var other_1541: Rotor;

    self_1847 = self_1846;
    other_1541 = other_1540;
    let _e4: MotorDual = self_1847;
    let _e8: Rotor = other_1541;
    let _e11: MotorDual = self_1847;
    let _e14: MotorDual = self_1847;
    let _e18: Rotor = other_1541;
    return Rotor(((vec2<f32>(_e4.g0_.x) * _e8.g0_) + ((vec2<f32>(_e11.g0_.y, _e14.g0_.x) * _e18.g0_.yx) * vec2<f32>(1.0, 0.0))));
}

fn motor_dual_rotor_outer_product(self_1848: MotorDual, other_1542: Rotor) -> MotorDual {
    var self_1849: MotorDual;
    var other_1543: Rotor;

    self_1849 = self_1848;
    other_1543 = other_1542;
    let _e4: MotorDual = self_1849;
    let _e8: Rotor = other_1543;
    let _e11: Rotor = other_1543;
    let _e14: Rotor = other_1543;
    let _e17: Rotor = other_1543;
    let _e28: MotorDual = self_1849;
    let _e31: Rotor = other_1543;
    return MotorDual((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y, _e11.g0_.x, _e14.g0_.y, _e17.g0_.y)) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) + ((_e28.g0_.xxzw * vec4<f32>(_e31.g0_.x)) * vec4<f32>(1.0, 0.0, 1.0, 1.0))));
}

fn motor_dual_rotor_inner_product(self_1850: MotorDual, other_1544: Rotor) -> MotorDual {
    var self_1851: MotorDual;
    var other_1545: Rotor;

    self_1851 = self_1850;
    other_1545 = other_1544;
    let _e4: MotorDual = self_1851;
    let _e8: Rotor = other_1545;
    let _e19: MotorDual = self_1851;
    let _e23: Rotor = other_1545;
    let _e26: Rotor = other_1545;
    let _e29: Rotor = other_1545;
    let _e32: Rotor = other_1545;
    let _e44: MotorDual = self_1851;
    let _e47: Rotor = other_1545;
    let _e50: Rotor = other_1545;
    let _e53: Rotor = other_1545;
    let _e56: Rotor = other_1545;
    return MotorDual(((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.x)) * vec4<f32>(0.0, 1.0, 0.0, 0.0)) + ((vec4<f32>(_e19.g0_.w) * vec4<f32>(_e23.g0_.y, _e26.g0_.y, _e29.g0_.y, _e32.g0_.x)) * vec4<f32>(0.0, 0.0, 1.0, 1.0))) + ((_e44.g0_.xxzz * vec4<f32>(_e47.g0_.x, _e50.g0_.y, _e53.g0_.x, _e56.g0_.y)) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))));
}

fn motor_dual_rotor_geometric_anti_product(self_1852: MotorDual, other_1546: Rotor) -> Rotor {
    var self_1853: MotorDual;
    var other_1547: Rotor;

    self_1853 = self_1852;
    other_1547 = other_1546;
    let _e4: MotorDual = self_1853;
    let _e8: Rotor = other_1547;
    let _e11: MotorDual = self_1853;
    let _e15: Rotor = other_1547;
    return Rotor(((vec2<f32>(_e4.g0_.x) * _e8.g0_) + ((vec2<f32>(_e11.g0_.y) * _e15.g0_.yx) * vec2<f32>(1.0, -(1.0)))));
}

fn motor_dual_rotor_inner_anti_product(self_1854: MotorDual, other_1548: Rotor) -> Rotor {
    var self_1855: MotorDual;
    var other_1549: Rotor;

    self_1855 = self_1854;
    other_1549 = other_1548;
    let _e4: MotorDual = self_1855;
    let _e8: Rotor = other_1549;
    let _e11: MotorDual = self_1855;
    let _e14: MotorDual = self_1855;
    let _e18: Rotor = other_1549;
    return Rotor(((vec2<f32>(_e4.g0_.x) * _e8.g0_) + ((vec2<f32>(_e11.g0_.x, _e14.g0_.y) * vec2<f32>(_e18.g0_.x)) * vec2<f32>(0.0, -(1.0)))));
}

fn motor_dual_rotor_right_contraction(self_1856: MotorDual, other_1550: Rotor) -> MotorDual {
    var self_1857: MotorDual;
    var other_1551: Rotor;

    self_1857 = self_1856;
    other_1551 = other_1550;
    let _e4: MotorDual = self_1857;
    let _e8: Rotor = other_1551;
    let _e19: MotorDual = self_1857;
    let _e22: Rotor = other_1551;
    let _e25: Rotor = other_1551;
    let _e28: Rotor = other_1551;
    let _e31: Rotor = other_1551;
    return MotorDual((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.x)) * vec4<f32>(0.0, 1.0, 0.0, 0.0)) + ((_e19.g0_.xxzw * vec4<f32>(_e22.g0_.x, _e25.g0_.y, _e28.g0_.x, _e31.g0_.x)) * vec4<f32>(1.0, -(1.0), 1.0, 1.0))));
}

fn motor_dual_rotor_left_anti_contraction(self_1858: MotorDual, other_1552: Rotor) -> Rotor {
    var self_1859: MotorDual;
    var other_1553: Rotor;

    self_1859 = self_1858;
    other_1553 = other_1552;
    let _e4: MotorDual = self_1859;
    let _e8: Rotor = other_1553;
    let _e11: MotorDual = self_1859;
    let _e14: MotorDual = self_1859;
    let _e18: Rotor = other_1553;
    return Rotor(((vec2<f32>(_e4.g0_.x) * _e8.g0_) + ((vec2<f32>(_e11.g0_.x, _e14.g0_.y) * vec2<f32>(_e18.g0_.x)) * vec2<f32>(0.0, -(1.0)))));
}

fn motor_dual_point_geometric_product(self_1860: MotorDual, other_1554: Point) -> MotorDual {
    var self_1861: MotorDual;
    var other_1555: Point;

    self_1861 = self_1860;
    other_1555 = other_1554;
    let _e4: MotorDual = self_1861;
    let _e8: Point = other_1555;
    let _e11: Point = other_1555;
    let _e14: Point = other_1555;
    let _e17: Point = other_1555;
    let _e29: MotorDual = self_1861;
    let _e33: Point = other_1555;
    let _e36: Point = other_1555;
    let _e39: Point = other_1555;
    let _e42: Point = other_1555;
    let _e55: MotorDual = self_1861;
    let _e58: Point = other_1555;
    return MotorDual(((((vec4<f32>(_e4.g0_.z) * vec4<f32>(_e8.g0_.y, _e11.g0_.z, _e14.g0_.y, _e17.g0_.x)) * vec4<f32>(1.0, 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e29.g0_.w) * vec4<f32>(_e33.g0_.z, _e36.g0_.y, _e39.g0_.x, _e42.g0_.z)) * vec4<f32>(1.0, -(1.0), 1.0, 0.0))) + ((_e55.g0_.yxxx * vec4<f32>(_e58.g0_.x)) * vec4<f32>(1.0, -(1.0), 0.0, 0.0))));
}

fn motor_dual_point_regressive_product(self_1862: MotorDual, other_1556: Point) -> Motor {
    var self_1863: MotorDual;
    var other_1557: Point;

    self_1863 = self_1862;
    other_1557 = other_1556;
    let _e4: MotorDual = self_1863;
    let _e8: Point = other_1557;
    let _e19: MotorDual = self_1863;
    let _e23: Point = other_1557;
    let _e35: MotorDual = self_1863;
    let _e38: Point = other_1557;
    let _e41: Point = other_1557;
    let _e44: Point = other_1557;
    let _e47: Point = other_1557;
    return Motor(((((vec4<f32>(_e4.g0_.z) * vec4<f32>(_e8.g0_.y)) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) + ((vec4<f32>(_e19.g0_.w) * vec4<f32>(_e23.g0_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + (_e35.g0_.yxxx * vec4<f32>(_e38.g0_.x, _e41.g0_.x, _e44.g0_.y, _e47.g0_.z))));
}

fn motor_dual_point_outer_product(self_1864: MotorDual, other_1558: Point) -> AntiScalar {
    var self_1865: MotorDual;
    var other_1559: Point;

    self_1865 = self_1864;
    other_1559 = other_1558;
    let _e4: MotorDual = self_1865;
    let _e7: Point = other_1559;
    let _e11: MotorDual = self_1865;
    let _e14: Point = other_1559;
    let _e19: MotorDual = self_1865;
    let _e22: Point = other_1559;
    return AntiScalar((((_e4.g0_.y * _e7.g0_.x) + (_e11.g0_.z * _e14.g0_.y)) + (_e19.g0_.w * _e22.g0_.z)));
}

fn motor_dual_point_inner_product(self_1866: MotorDual, other_1560: Point) -> Plane {
    var self_1867: MotorDual;
    var other_1561: Point;

    self_1867 = self_1866;
    other_1561 = other_1560;
    let _e4: MotorDual = self_1867;
    let _e8: Point = other_1561;
    let _e18: MotorDual = self_1867;
    let _e22: Point = other_1561;
    let _e33: MotorDual = self_1867;
    let _e37: Point = other_1561;
    return Plane(((((vec3<f32>(_e4.g0_.z) * _e8.g0_.zzx) * vec3<f32>(1.0, 0.0, -(1.0))) + ((vec3<f32>(_e18.g0_.w) * _e22.g0_.yxy) * vec3<f32>(-(1.0), 1.0, 0.0))) + ((vec3<f32>(_e33.g0_.x) * vec3<f32>(_e37.g0_.x)) * vec3<f32>(-(1.0), 0.0, 0.0))));
}

fn motor_dual_point_geometric_anti_product(self_1868: MotorDual, other_1562: Point) -> Motor {
    var self_1869: MotorDual;
    var other_1563: Point;

    self_1869 = self_1868;
    other_1563 = other_1562;
    let _e4: MotorDual = self_1869;
    let _e8: Point = other_1563;
    let _e11: Point = other_1563;
    let _e14: Point = other_1563;
    let _e17: Point = other_1563;
    let _e29: MotorDual = self_1869;
    let _e33: Point = other_1563;
    let _e36: Point = other_1563;
    let _e39: Point = other_1563;
    let _e42: Point = other_1563;
    let _e54: MotorDual = self_1869;
    let _e58: Point = other_1563;
    let _e61: Point = other_1563;
    let _e64: Point = other_1563;
    let _e67: Point = other_1563;
    let _e80: MotorDual = self_1869;
    let _e84: Point = other_1563;
    let _e87: Point = other_1563;
    let _e90: Point = other_1563;
    let _e93: Point = other_1563;
    return Motor((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.x, _e11.g0_.x, _e14.g0_.z, _e17.g0_.y)) * vec4<f32>(1.0, 0.0, -(1.0), 1.0)) + ((vec4<f32>(_e29.g0_.z) * vec4<f32>(_e33.g0_.y, _e36.g0_.z, _e39.g0_.y, _e42.g0_.y)) * vec4<f32>(1.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e54.g0_.w) * vec4<f32>(_e58.g0_.z, _e61.g0_.y, _e64.g0_.z, _e67.g0_.z)) * vec4<f32>(1.0, -(1.0), 0.0, 0.0))) + ((vec4<f32>(_e80.g0_.x) * vec4<f32>(_e84.g0_.x, _e87.g0_.x, _e90.g0_.y, _e93.g0_.z)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn motor_dual_point_inner_anti_product(self_1870: MotorDual, other_1564: Point) -> Point {
    var self_1871: MotorDual;
    var other_1565: Point;

    self_1871 = self_1870;
    other_1565 = other_1564;
    let _e4: MotorDual = self_1871;
    let _e8: Point = other_1565;
    let _e11: MotorDual = self_1871;
    let _e15: Point = other_1565;
    let _e27: MotorDual = self_1871;
    let _e30: MotorDual = self_1871;
    let _e33: MotorDual = self_1871;
    let _e37: Point = other_1565;
    return Point((((vec3<f32>(_e4.g0_.x) * _e8.g0_) + ((vec3<f32>(_e11.g0_.w) * vec3<f32>(_e15.g0_.y)) * vec3<f32>(-(1.0), 0.0, 0.0))) + ((vec3<f32>(_e27.g0_.z, _e30.g0_.y, _e33.g0_.y) * _e37.g0_.zzy) * vec3<f32>(1.0, -(1.0), 1.0))));
}

fn motor_dual_point_left_contraction(self_1872: MotorDual, other_1566: Point) -> Plane {
    var self_1873: MotorDual;
    var other_1567: Point;

    self_1873 = self_1872;
    other_1567 = other_1566;
    let _e4: MotorDual = self_1873;
    let _e8: Point = other_1567;
    let _e18: MotorDual = self_1873;
    let _e21: MotorDual = self_1873;
    let _e24: MotorDual = self_1873;
    let _e28: Point = other_1567;
    return Plane((((vec3<f32>(_e4.g0_.w) * _e8.g0_.yxy) * vec3<f32>(-(1.0), 1.0, 0.0)) + ((vec3<f32>(_e18.g0_.z, _e21.g0_.x, _e24.g0_.z) * _e28.g0_.zxx) * vec3<f32>(1.0, 0.0, -(1.0)))));
}

fn motor_dual_point_left_anti_contraction(self_1874: MotorDual, other_1568: Point) -> Point {
    var self_1875: MotorDual;
    var other_1569: Point;

    self_1875 = self_1874;
    other_1569 = other_1568;
    let _e4: MotorDual = self_1875;
    let _e8: Point = other_1569;
    return Point((vec3<f32>(_e4.g0_.x) * _e8.g0_));
}

fn motor_dual_point_right_anti_contraction(self_1876: MotorDual, other_1570: Point) -> Point {
    var self_1877: MotorDual;
    var other_1571: Point;

    self_1877 = self_1876;
    other_1571 = other_1570;
    let _e4: MotorDual = self_1877;
    let _e8: Point = other_1571;
    let _e19: MotorDual = self_1877;
    let _e22: MotorDual = self_1877;
    let _e25: MotorDual = self_1877;
    let _e29: Point = other_1571;
    return Point((((vec3<f32>(_e4.g0_.w) * vec3<f32>(_e8.g0_.y)) * vec3<f32>(-(1.0), 0.0, 0.0)) + ((vec3<f32>(_e19.g0_.z, _e22.g0_.y, _e25.g0_.y) * _e29.g0_.zzy) * vec3<f32>(1.0, -(1.0), 1.0))));
}

fn motor_dual_ideal_point_regressive_product(self_1878: MotorDual, other_1572: IdealPoint) -> Translator {
    var self_1879: MotorDual;
    var other_1573: IdealPoint;

    self_1879 = self_1878;
    other_1573 = other_1572;
    let _e4: MotorDual = self_1879;
    let _e8: IdealPoint = other_1573;
    let _e18: MotorDual = self_1879;
    let _e21: MotorDual = self_1879;
    let _e24: MotorDual = self_1879;
    let _e28: IdealPoint = other_1573;
    let _e31: IdealPoint = other_1573;
    let _e34: IdealPoint = other_1573;
    return Translator((((vec3<f32>(_e4.g0_.w) * vec3<f32>(_e8.g0_.y)) * vec3<f32>(1.0, 0.0, 0.0)) + (vec3<f32>(_e18.g0_.z, _e21.g0_.x, _e24.g0_.x) * vec3<f32>(_e28.g0_.x, _e31.g0_.x, _e34.g0_.y))));
}

fn motor_dual_ideal_point_outer_product(self_1880: MotorDual, other_1574: IdealPoint) -> AntiScalar {
    var self_1881: MotorDual;
    var other_1575: IdealPoint;

    self_1881 = self_1880;
    other_1575 = other_1574;
    let _e4: MotorDual = self_1881;
    let _e7: IdealPoint = other_1575;
    let _e11: MotorDual = self_1881;
    let _e14: IdealPoint = other_1575;
    return AntiScalar(((_e4.g0_.z * _e7.g0_.x) + (_e11.g0_.w * _e14.g0_.y)));
}

fn motor_dual_ideal_point_geometric_anti_product(self_1882: MotorDual, other_1576: IdealPoint) -> Motor {
    var self_1883: MotorDual;
    var other_1577: IdealPoint;

    self_1883 = self_1882;
    other_1577 = other_1576;
    let _e4: MotorDual = self_1883;
    let _e8: IdealPoint = other_1577;
    let _e11: IdealPoint = other_1577;
    let _e14: IdealPoint = other_1577;
    let _e17: IdealPoint = other_1577;
    let _e29: MotorDual = self_1883;
    let _e33: IdealPoint = other_1577;
    let _e36: IdealPoint = other_1577;
    let _e39: IdealPoint = other_1577;
    let _e42: IdealPoint = other_1577;
    let _e55: MotorDual = self_1883;
    let _e58: IdealPoint = other_1577;
    let _e61: IdealPoint = other_1577;
    let _e64: IdealPoint = other_1577;
    let _e67: IdealPoint = other_1577;
    return Motor(((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y, _e11.g0_.y, _e14.g0_.y, _e17.g0_.x)) * vec4<f32>(0.0, 0.0, -(1.0), 1.0)) + ((vec4<f32>(_e29.g0_.w) * vec4<f32>(_e33.g0_.y, _e36.g0_.x, _e39.g0_.y, _e42.g0_.y)) * vec4<f32>(1.0, -(1.0), 0.0, 0.0))) + (_e55.g0_.zzxx * vec4<f32>(_e58.g0_.x, _e61.g0_.y, _e64.g0_.x, _e67.g0_.y))));
}

fn motor_dual_ideal_point_inner_anti_product(self_1884: MotorDual, other_1578: IdealPoint) -> Point {
    var self_1885: MotorDual;
    var other_1579: IdealPoint;

    self_1885 = self_1884;
    other_1579 = other_1578;
    let _e4: MotorDual = self_1885;
    let _e8: IdealPoint = other_1579;
    let _e11: IdealPoint = other_1579;
    let _e14: IdealPoint = other_1579;
    let _e25: MotorDual = self_1885;
    let _e29: IdealPoint = other_1579;
    let _e41: MotorDual = self_1885;
    let _e44: MotorDual = self_1885;
    let _e47: MotorDual = self_1885;
    let _e51: IdealPoint = other_1579;
    let _e54: IdealPoint = other_1579;
    let _e57: IdealPoint = other_1579;
    return Point(((((vec3<f32>(_e4.g0_.y) * vec3<f32>(_e8.g0_.y, _e11.g0_.y, _e14.g0_.x)) * vec3<f32>(0.0, -(1.0), 1.0)) + ((vec3<f32>(_e25.g0_.w) * vec3<f32>(_e29.g0_.x)) * vec3<f32>(-(1.0), 0.0, 0.0))) + (vec3<f32>(_e41.g0_.z, _e44.g0_.x, _e47.g0_.x) * vec3<f32>(_e51.g0_.y, _e54.g0_.x, _e57.g0_.y))));
}

fn motor_dual_ideal_point_left_anti_contraction(self_1886: MotorDual, other_1580: IdealPoint) -> IdealPoint {
    var self_1887: MotorDual;
    var other_1581: IdealPoint;

    self_1887 = self_1886;
    other_1581 = other_1580;
    let _e4: MotorDual = self_1887;
    let _e8: IdealPoint = other_1581;
    return IdealPoint((vec2<f32>(_e4.g0_.x) * _e8.g0_));
}

fn motor_dual_ideal_point_right_anti_contraction(self_1888: MotorDual, other_1582: IdealPoint) -> Point {
    var self_1889: MotorDual;
    var other_1583: IdealPoint;

    self_1889 = self_1888;
    other_1583 = other_1582;
    let _e4: MotorDual = self_1889;
    let _e8: IdealPoint = other_1583;
    let _e19: MotorDual = self_1889;
    let _e22: MotorDual = self_1889;
    let _e25: MotorDual = self_1889;
    let _e29: IdealPoint = other_1583;
    let _e32: IdealPoint = other_1583;
    let _e35: IdealPoint = other_1583;
    return Point((((vec3<f32>(_e4.g0_.w) * vec3<f32>(_e8.g0_.x)) * vec3<f32>(-(1.0), 0.0, 0.0)) + ((vec3<f32>(_e19.g0_.z, _e22.g0_.y, _e25.g0_.y) * vec3<f32>(_e29.g0_.y, _e32.g0_.y, _e35.g0_.x)) * vec3<f32>(1.0, -(1.0), 1.0))));
}

fn motor_dual_plane_into(self_1890: MotorDual) -> Plane {
    var self_1891: MotorDual;

    self_1891 = self_1890;
    let _e2: MotorDual = self_1891;
    let _e5: MotorDual = self_1891;
    let _e8: MotorDual = self_1891;
    return Plane(vec3<f32>(_e2.g0_.y, _e5.g0_.z, _e8.g0_.w));
}

fn motor_dual_plane_add(self_1892: MotorDual, other_1584: Plane) -> MotorDual {
    var self_1893: MotorDual;
    var other_1585: Plane;

    self_1893 = self_1892;
    other_1585 = other_1584;
    let _e4: MotorDual = self_1893;
    let _e6: Plane = other_1585;
    let _e9: Plane = other_1585;
    let _e12: Plane = other_1585;
    let _e15: Plane = other_1585;
    return MotorDual((_e4.g0_ + (vec4<f32>(_e6.g0_.x, _e9.g0_.x, _e12.g0_.y, _e15.g0_.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn motor_dual_plane_sub(self_1894: MotorDual, other_1586: Plane) -> MotorDual {
    var self_1895: MotorDual;
    var other_1587: Plane;

    self_1895 = self_1894;
    other_1587 = other_1586;
    let _e4: MotorDual = self_1895;
    let _e6: Plane = other_1587;
    let _e9: Plane = other_1587;
    let _e12: Plane = other_1587;
    let _e15: Plane = other_1587;
    return MotorDual((_e4.g0_ - (vec4<f32>(_e6.g0_.x, _e9.g0_.x, _e12.g0_.y, _e15.g0_.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn motor_dual_plane_geometric_product(self_1896: MotorDual, other_1588: Plane) -> Motor {
    var self_1897: MotorDual;
    var other_1589: Plane;

    self_1897 = self_1896;
    other_1589 = other_1588;
    let _e4: MotorDual = self_1897;
    let _e8: Plane = other_1589;
    let _e11: Plane = other_1589;
    let _e14: Plane = other_1589;
    let _e17: Plane = other_1589;
    let _e29: MotorDual = self_1897;
    let _e33: Plane = other_1589;
    let _e36: Plane = other_1589;
    let _e39: Plane = other_1589;
    let _e42: Plane = other_1589;
    let _e55: MotorDual = self_1897;
    let _e59: Plane = other_1589;
    let _e62: Plane = other_1589;
    let _e65: Plane = other_1589;
    let _e68: Plane = other_1589;
    let _e81: MotorDual = self_1897;
    let _e85: Plane = other_1589;
    let _e88: Plane = other_1589;
    let _e91: Plane = other_1589;
    let _e94: Plane = other_1589;
    return Motor((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.z, _e11.g0_.z, _e14.g0_.z, _e17.g0_.y)) * vec4<f32>(0.0, 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e29.g0_.z) * vec4<f32>(_e33.g0_.y, _e36.g0_.z, _e39.g0_.y, _e42.g0_.x)) * vec4<f32>(1.0, -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e55.g0_.w) * vec4<f32>(_e59.g0_.z, _e62.g0_.y, _e65.g0_.x, _e68.g0_.z)) * vec4<f32>(1.0, 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e81.g0_.x) * vec4<f32>(_e85.g0_.x, _e88.g0_.x, _e91.g0_.y, _e94.g0_.z)) * vec4<f32>(0.0, 0.0, 1.0, 1.0))));
}

fn motor_dual_plane_regressive_product(self_1898: MotorDual, other_1590: Plane) -> Plane {
    var self_1899: MotorDual;
    var other_1591: Plane;

    self_1899 = self_1898;
    other_1591 = other_1590;
    let _e4: MotorDual = self_1899;
    let _e8: Plane = other_1591;
    return Plane((vec3<f32>(_e4.g0_.x) * _e8.g0_));
}

fn motor_dual_plane_outer_product(self_1900: MotorDual, other_1592: Plane) -> Point {
    var self_1901: MotorDual;
    var other_1593: Plane;

    self_1901 = self_1900;
    other_1593 = other_1592;
    let _e4: MotorDual = self_1901;
    let _e8: Plane = other_1593;
    let _e18: MotorDual = self_1901;
    let _e22: Plane = other_1593;
    let _e33: MotorDual = self_1901;
    let _e36: MotorDual = self_1901;
    let _e39: MotorDual = self_1901;
    let _e43: Plane = other_1593;
    return Point(((((vec3<f32>(_e4.g0_.z) * _e8.g0_.zzx) * vec3<f32>(-(1.0), 0.0, 1.0)) + ((vec3<f32>(_e18.g0_.w) * _e22.g0_.yxy) * vec3<f32>(1.0, -(1.0), 0.0))) + ((vec3<f32>(_e33.g0_.x, _e36.g0_.y, _e39.g0_.y) * _e43.g0_.xzy) * vec3<f32>(0.0, 1.0, -(1.0)))));
}

fn motor_dual_plane_inner_product(self_1902: MotorDual, other_1594: Plane) -> Translator {
    var self_1903: MotorDual;
    var other_1595: Plane;

    self_1903 = self_1902;
    other_1595 = other_1594;
    let _e4: MotorDual = self_1903;
    let _e8: Plane = other_1595;
    let _e18: MotorDual = self_1903;
    let _e21: MotorDual = self_1903;
    let _e24: MotorDual = self_1903;
    let _e28: Plane = other_1595;
    return Translator((((vec3<f32>(_e4.g0_.w) * vec3<f32>(_e8.g0_.z)) * vec3<f32>(1.0, 0.0, 0.0)) + (vec3<f32>(_e18.g0_.z, _e21.g0_.x, _e24.g0_.x) * _e28.g0_.yyz)));
}

fn motor_dual_plane_geometric_anti_product(self_1904: MotorDual, other_1596: Plane) -> MotorDual {
    var self_1905: MotorDual;
    var other_1597: Plane;

    self_1905 = self_1904;
    other_1597 = other_1596;
    let _e4: MotorDual = self_1905;
    let _e8: Plane = other_1597;
    let _e11: Plane = other_1597;
    let _e14: Plane = other_1597;
    let _e17: Plane = other_1597;
    let _e30: MotorDual = self_1905;
    let _e34: Plane = other_1597;
    let _e47: MotorDual = self_1905;
    let _e51: Plane = other_1597;
    let _e63: MotorDual = self_1905;
    let _e67: Plane = other_1597;
    let _e70: Plane = other_1597;
    let _e73: Plane = other_1597;
    let _e76: Plane = other_1597;
    return MotorDual((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.x, _e11.g0_.x, _e14.g0_.z, _e17.g0_.y)) * vec4<f32>(-(1.0), 0.0, -(1.0), 1.0)) + ((vec4<f32>(_e30.g0_.z) * vec4<f32>(_e34.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e47.g0_.w) * vec4<f32>(_e51.g0_.x)) * vec4<f32>(0.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e63.g0_.x) * vec4<f32>(_e67.g0_.x, _e70.g0_.x, _e73.g0_.y, _e76.g0_.z)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn motor_dual_plane_inner_anti_product(self_1906: MotorDual, other_1598: Plane) -> MotorDual {
    var self_1907: MotorDual;
    var other_1599: Plane;

    self_1907 = self_1906;
    other_1599 = other_1598;
    let _e4: MotorDual = self_1907;
    let _e7: Plane = other_1599;
    let _e10: Plane = other_1599;
    let _e13: Plane = other_1599;
    let _e16: Plane = other_1599;
    return MotorDual(((_e4.g0_.yxxx * vec4<f32>(_e7.g0_.x, _e10.g0_.x, _e13.g0_.y, _e16.g0_.z)) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0)));
}

fn motor_dual_plane_left_contraction(self_1908: MotorDual, other_1600: Plane) -> Scalar {
    var self_1909: MotorDual;
    var other_1601: Plane;

    self_1909 = self_1908;
    other_1601 = other_1600;
    let _e4: MotorDual = self_1909;
    let _e7: Plane = other_1601;
    let _e11: MotorDual = self_1909;
    let _e14: Plane = other_1601;
    return Scalar(((_e4.g0_.z * _e7.g0_.y) + (_e11.g0_.w * _e14.g0_.z)));
}

fn motor_dual_plane_right_contraction(self_1910: MotorDual, other_1602: Plane) -> Translator {
    var self_1911: MotorDual;
    var other_1603: Plane;

    self_1911 = self_1910;
    other_1603 = other_1602;
    let _e4: MotorDual = self_1911;
    let _e8: Plane = other_1603;
    let _e18: MotorDual = self_1911;
    let _e21: MotorDual = self_1911;
    let _e24: MotorDual = self_1911;
    let _e28: Plane = other_1603;
    return Translator((((vec3<f32>(_e4.g0_.w) * vec3<f32>(_e8.g0_.z)) * vec3<f32>(1.0, 0.0, 0.0)) + (vec3<f32>(_e18.g0_.z, _e21.g0_.x, _e24.g0_.x) * _e28.g0_.yyz)));
}

fn motor_dual_plane_left_anti_contraction(self_1912: MotorDual, other_1604: Plane) -> MotorDual {
    var self_1913: MotorDual;
    var other_1605: Plane;

    self_1913 = self_1912;
    other_1605 = other_1604;
    let _e4: MotorDual = self_1913;
    let _e7: Plane = other_1605;
    let _e10: Plane = other_1605;
    let _e13: Plane = other_1605;
    let _e16: Plane = other_1605;
    return MotorDual(((_e4.g0_.yxxx * vec4<f32>(_e7.g0_.x, _e10.g0_.x, _e13.g0_.y, _e16.g0_.z)) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0)));
}

fn motor_dual_plane_right_anti_contraction(self_1914: MotorDual, other_1606: Plane) -> AntiScalar {
    var self_1915: MotorDual;
    var other_1607: Plane;

    self_1915 = self_1914;
    other_1607 = other_1606;
    let _e5: MotorDual = self_1915;
    let _e8: Plane = other_1607;
    return AntiScalar((0.0 - (_e5.g0_.y * _e8.g0_.x)));
}

fn motor_dual_plane_scalar_product(self_1916: MotorDual, other_1608: Plane) -> Scalar {
    var self_1917: MotorDual;
    var other_1609: Plane;

    self_1917 = self_1916;
    other_1609 = other_1608;
    let _e4: MotorDual = self_1917;
    let _e7: Plane = other_1609;
    let _e11: MotorDual = self_1917;
    let _e14: Plane = other_1609;
    return Scalar(((_e4.g0_.z * _e7.g0_.y) + (_e11.g0_.w * _e14.g0_.z)));
}

fn motor_dual_plane_anti_scalar_product(self_1918: MotorDual, other_1610: Plane) -> AntiScalar {
    var self_1919: MotorDual;
    var other_1611: Plane;

    self_1919 = self_1918;
    other_1611 = other_1610;
    let _e5: MotorDual = self_1919;
    let _e8: Plane = other_1611;
    return AntiScalar((0.0 - (_e5.g0_.y * _e8.g0_.x)));
}

fn motor_dual_translator_geometric_product(self_1920: MotorDual, other_1612: Translator) -> MotorDual {
    var self_1921: MotorDual;
    var other_1613: Translator;

    self_1921 = self_1920;
    other_1613 = other_1612;
    let _e4: MotorDual = self_1921;
    let _e8: Translator = other_1613;
    let _e11: Translator = other_1613;
    let _e14: Translator = other_1613;
    let _e17: Translator = other_1613;
    let _e28: MotorDual = self_1921;
    let _e32: Translator = other_1613;
    let _e35: Translator = other_1613;
    let _e38: Translator = other_1613;
    let _e41: Translator = other_1613;
    let _e54: MotorDual = self_1921;
    let _e57: Translator = other_1613;
    return MotorDual(((((vec4<f32>(_e4.g0_.z) * vec4<f32>(_e8.g0_.y, _e11.g0_.z, _e14.g0_.x, _e17.g0_.y)) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) + ((vec4<f32>(_e28.g0_.w) * vec4<f32>(_e32.g0_.z, _e35.g0_.y, _e38.g0_.z, _e41.g0_.x)) * vec4<f32>(1.0, -(1.0), 0.0, 1.0))) + ((_e54.g0_.xyxx * vec4<f32>(_e57.g0_.x)) * vec4<f32>(1.0, 1.0, 0.0, 0.0))));
}

fn motor_dual_translator_regressive_product(self_1922: MotorDual, other_1614: Translator) -> Translator {
    var self_1923: MotorDual;
    var other_1615: Translator;

    self_1923 = self_1922;
    other_1615 = other_1614;
    let _e4: MotorDual = self_1923;
    let _e8: Translator = other_1615;
    let _e11: MotorDual = self_1923;
    let _e15: Translator = other_1615;
    let _e26: MotorDual = self_1923;
    let _e29: MotorDual = self_1923;
    let _e32: MotorDual = self_1923;
    let _e36: Translator = other_1615;
    return Translator((((vec3<f32>(_e4.g0_.x) * _e8.g0_) + ((vec3<f32>(_e11.g0_.w) * vec3<f32>(_e15.g0_.z)) * vec3<f32>(1.0, 0.0, 0.0))) + ((vec3<f32>(_e26.g0_.z, _e29.g0_.x, _e32.g0_.x) * _e36.g0_.yxx) * vec3<f32>(1.0, 0.0, 0.0))));
}

fn motor_dual_translator_outer_product(self_1924: MotorDual, other_1616: Translator) -> MotorDual {
    var self_1925: MotorDual;
    var other_1617: Translator;

    self_1925 = self_1924;
    other_1617 = other_1616;
    let _e4: MotorDual = self_1925;
    let _e8: Translator = other_1617;
    let _e11: Translator = other_1617;
    let _e14: Translator = other_1617;
    let _e17: Translator = other_1617;
    let _e28: MotorDual = self_1925;
    let _e32: Translator = other_1617;
    let _e35: Translator = other_1617;
    let _e38: Translator = other_1617;
    let _e41: Translator = other_1617;
    let _e53: MotorDual = self_1925;
    let _e56: Translator = other_1617;
    return MotorDual(((((vec4<f32>(_e4.g0_.z) * vec4<f32>(_e8.g0_.y, _e11.g0_.y, _e14.g0_.x, _e17.g0_.y)) * vec4<f32>(1.0, 0.0, 1.0, 0.0)) + ((vec4<f32>(_e28.g0_.w) * vec4<f32>(_e32.g0_.z, _e35.g0_.z, _e38.g0_.z, _e41.g0_.x)) * vec4<f32>(1.0, 0.0, 0.0, 1.0))) + ((_e53.g0_.xyxx * vec4<f32>(_e56.g0_.x)) * vec4<f32>(1.0, 1.0, 0.0, 0.0))));
}

fn motor_dual_translator_inner_product(self_1926: MotorDual, other_1618: Translator) -> MotorDual {
    var self_1927: MotorDual;
    var other_1619: Translator;

    self_1927 = self_1926;
    other_1619 = other_1618;
    let _e4: MotorDual = self_1927;
    let _e8: Translator = other_1619;
    let _e11: Translator = other_1619;
    let _e14: Translator = other_1619;
    let _e17: Translator = other_1619;
    let _e28: MotorDual = self_1927;
    let _e32: Translator = other_1619;
    let _e35: Translator = other_1619;
    let _e38: Translator = other_1619;
    let _e41: Translator = other_1619;
    let _e54: MotorDual = self_1927;
    let _e57: Translator = other_1619;
    return MotorDual(((((vec4<f32>(_e4.g0_.z) * vec4<f32>(_e8.g0_.z, _e11.g0_.z, _e14.g0_.x, _e17.g0_.z)) * vec4<f32>(0.0, 1.0, 1.0, 0.0)) + ((vec4<f32>(_e28.g0_.w) * vec4<f32>(_e32.g0_.y, _e35.g0_.y, _e38.g0_.y, _e41.g0_.x)) * vec4<f32>(0.0, -(1.0), 0.0, 1.0))) + ((_e54.g0_.xyxx * vec4<f32>(_e57.g0_.x)) * vec4<f32>(1.0, 1.0, 0.0, 0.0))));
}

fn motor_dual_translator_geometric_anti_product(self_1928: MotorDual, other_1620: Translator) -> Motor {
    var self_1929: MotorDual;
    var other_1621: Translator;

    self_1929 = self_1928;
    other_1621 = other_1620;
    let _e4: MotorDual = self_1929;
    let _e8: Translator = other_1621;
    let _e11: Translator = other_1621;
    let _e14: Translator = other_1621;
    let _e17: Translator = other_1621;
    let _e30: MotorDual = self_1929;
    let _e34: Translator = other_1621;
    let _e37: Translator = other_1621;
    let _e40: Translator = other_1621;
    let _e43: Translator = other_1621;
    let _e55: MotorDual = self_1929;
    let _e59: Translator = other_1621;
    let _e62: Translator = other_1621;
    let _e65: Translator = other_1621;
    let _e68: Translator = other_1621;
    let _e81: MotorDual = self_1929;
    let _e85: Translator = other_1621;
    let _e88: Translator = other_1621;
    let _e91: Translator = other_1621;
    let _e94: Translator = other_1621;
    return Motor((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.x, _e11.g0_.x, _e14.g0_.z, _e17.g0_.y)) * vec4<f32>(0.0, -(1.0), -(1.0), 1.0)) + ((vec4<f32>(_e30.g0_.z) * vec4<f32>(_e34.g0_.y, _e37.g0_.z, _e40.g0_.y, _e43.g0_.y)) * vec4<f32>(1.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e55.g0_.w) * vec4<f32>(_e59.g0_.z, _e62.g0_.y, _e65.g0_.z, _e68.g0_.z)) * vec4<f32>(1.0, -(1.0), 0.0, 0.0))) + ((vec4<f32>(_e81.g0_.x) * vec4<f32>(_e85.g0_.x, _e88.g0_.x, _e91.g0_.y, _e94.g0_.z)) * vec4<f32>(1.0, 0.0, 1.0, 1.0))));
}

fn motor_dual_translator_inner_anti_product(self_1930: MotorDual, other_1622: Translator) -> Motor {
    var self_1931: MotorDual;
    var other_1623: Translator;

    self_1931 = self_1930;
    other_1623 = other_1622;
    let _e4: MotorDual = self_1931;
    let _e8: Translator = other_1623;
    let _e11: Translator = other_1623;
    let _e14: Translator = other_1623;
    let _e17: Translator = other_1623;
    let _e30: MotorDual = self_1931;
    let _e34: Translator = other_1623;
    let _e47: MotorDual = self_1931;
    let _e50: Translator = other_1623;
    let _e53: Translator = other_1623;
    let _e56: Translator = other_1623;
    let _e59: Translator = other_1623;
    return Motor(((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.x, _e11.g0_.x, _e14.g0_.z, _e17.g0_.y)) * vec4<f32>(0.0, -(1.0), -(1.0), 1.0)) + ((vec4<f32>(_e30.g0_.w) * vec4<f32>(_e34.g0_.y)) * vec4<f32>(0.0, -(1.0), 0.0, 0.0))) + (_e47.g0_.xzxx * vec4<f32>(_e50.g0_.x, _e53.g0_.z, _e56.g0_.y, _e59.g0_.z))));
}

fn motor_dual_translator_right_contraction(self_1932: MotorDual, other_1624: Translator) -> MotorDual {
    var self_1933: MotorDual;
    var other_1625: Translator;

    self_1933 = self_1932;
    other_1625 = other_1624;
    let _e4: MotorDual = self_1933;
    let _e6: Translator = other_1625;
    return MotorDual((_e4.g0_ * vec4<f32>(_e6.g0_.x)));
}

fn motor_dual_translator_left_anti_contraction(self_1934: MotorDual, other_1626: Translator) -> Motor {
    var self_1935: MotorDual;
    var other_1627: Translator;

    self_1935 = self_1934;
    other_1627 = other_1626;
    let _e4: MotorDual = self_1935;
    let _e7: Translator = other_1627;
    let _e10: Translator = other_1627;
    let _e13: Translator = other_1627;
    let _e16: Translator = other_1627;
    return Motor(((_e4.g0_.xyxx * vec4<f32>(_e7.g0_.x, _e10.g0_.x, _e13.g0_.y, _e16.g0_.z)) * vec4<f32>(1.0, -(1.0), 1.0, 1.0)));
}

fn motor_dual_translator_right_anti_contraction(self_1936: MotorDual, other_1628: Translator) -> Point {
    var self_1937: MotorDual;
    var other_1629: Translator;

    self_1937 = self_1936;
    other_1629 = other_1628;
    let _e4: MotorDual = self_1937;
    let _e8: Translator = other_1629;
    let _e19: MotorDual = self_1937;
    let _e22: MotorDual = self_1937;
    let _e25: MotorDual = self_1937;
    let _e29: Translator = other_1629;
    return Point((((vec3<f32>(_e4.g0_.w) * vec3<f32>(_e8.g0_.y)) * vec3<f32>(-(1.0), 0.0, 0.0)) + ((vec3<f32>(_e19.g0_.z, _e22.g0_.y, _e25.g0_.y) * _e29.g0_.zzy) * vec3<f32>(1.0, -(1.0), 1.0))));
}

fn motor_dual_motor_add(self_1938: MotorDual, other_1630: Motor) -> MultiVector {
    var self_1939: MotorDual;
    var other_1631: Motor;

    self_1939 = self_1938;
    other_1631 = other_1630;
    let _e4: MotorDual = self_1939;
    let _e13: Motor = other_1631;
    let _e23: MotorDual = self_1939;
    let _e32: Motor = other_1631;
    return MultiVector(((_e4.g0_.xxwz * vec4<f32>(0.0, 0.0, 1.0, 1.0)) + (_e13.g0_.xyxx * vec4<f32>(1.0, 1.0, 0.0, 0.0))), ((_e23.g0_.yxxx * vec4<f32>(1.0, 1.0, 0.0, 0.0)) + (_e32.g0_.xxzw * vec4<f32>(0.0, 0.0, 1.0, 1.0))));
}

fn motor_dual_motor_sub(self_1940: MotorDual, other_1632: Motor) -> MultiVector {
    var self_1941: MotorDual;
    var other_1633: Motor;

    self_1941 = self_1940;
    other_1633 = other_1632;
    let _e4: MotorDual = self_1941;
    let _e13: Motor = other_1633;
    let _e23: MotorDual = self_1941;
    let _e32: Motor = other_1633;
    return MultiVector(((_e4.g0_.xxwz * vec4<f32>(0.0, 0.0, 1.0, 1.0)) - (_e13.g0_.xyxx * vec4<f32>(1.0, 1.0, 0.0, 0.0))), ((_e23.g0_.yxxx * vec4<f32>(1.0, 1.0, 0.0, 0.0)) - (_e32.g0_.xxzw * vec4<f32>(0.0, 0.0, 1.0, 1.0))));
}

fn motor_dual_motor_geometric_product(self_1942: MotorDual, other_1634: Motor) -> MotorDual {
    var self_1943: MotorDual;
    var other_1635: Motor;

    self_1943 = self_1942;
    other_1635 = other_1634;
    let _e4: MotorDual = self_1943;
    let _e8: Motor = other_1635;
    let _e18: MotorDual = self_1943;
    let _e22: Motor = other_1635;
    let _e34: MotorDual = self_1943;
    let _e38: Motor = other_1635;
    let _e50: MotorDual = self_1943;
    let _e54: Motor = other_1635;
    return MotorDual((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.yxyy) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) + ((vec4<f32>(_e18.g0_.z) * _e22.g0_.zwxy) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e34.g0_.w) * _e38.g0_.wzyx) * vec4<f32>(1.0, -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e50.g0_.x) * _e54.g0_.xyxx) * vec4<f32>(1.0, -(1.0), 0.0, 0.0))));
}

fn motor_dual_motor_regressive_product(self_1944: MotorDual, other_1636: Motor) -> Motor {
    var self_1945: MotorDual;
    var other_1637: Motor;

    self_1945 = self_1944;
    other_1637 = other_1636;
    let _e4: MotorDual = self_1945;
    let _e8: Motor = other_1637;
    let _e11: MotorDual = self_1945;
    let _e15: Motor = other_1637;
    let _e27: MotorDual = self_1945;
    let _e31: Motor = other_1637;
    let _e43: MotorDual = self_1945;
    let _e46: Motor = other_1637;
    return Motor(((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.z) * vec4<f32>(_e15.g0_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e27.g0_.w) * vec4<f32>(_e31.g0_.w)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((_e43.g0_.yxxx * _e46.g0_.yxxx) * vec4<f32>(1.0, 0.0, 0.0, 0.0))));
}

fn motor_dual_motor_outer_product(self_1946: MotorDual, other_1638: Motor) -> MotorDual {
    var self_1947: MotorDual;
    var other_1639: Motor;

    self_1947 = self_1946;
    other_1639 = other_1638;
    let _e4: MotorDual = self_1947;
    let _e8: Motor = other_1639;
    let _e18: MotorDual = self_1947;
    let _e22: Motor = other_1639;
    let _e33: MotorDual = self_1947;
    let _e37: Motor = other_1639;
    let _e48: MotorDual = self_1947;
    let _e52: Motor = other_1639;
    return MotorDual((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.yxyy) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) + ((vec4<f32>(_e18.g0_.z) * _e22.g0_.zzxz) * vec4<f32>(1.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e33.g0_.w) * _e37.g0_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e48.g0_.x) * vec4<f32>(_e52.g0_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))));
}

fn motor_dual_motor_inner_product(self_1948: MotorDual, other_1640: Motor) -> MotorDual {
    var self_1949: MotorDual;
    var other_1641: Motor;

    self_1949 = self_1948;
    other_1641 = other_1640;
    let _e4: MotorDual = self_1949;
    let _e8: Motor = other_1641;
    let _e19: MotorDual = self_1949;
    let _e23: Motor = other_1641;
    let _e35: MotorDual = self_1949;
    let _e39: Motor = other_1641;
    let _e51: MotorDual = self_1949;
    let _e55: Motor = other_1641;
    return MotorDual((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.x)) * vec4<f32>(0.0, 1.0, 0.0, 0.0)) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.wwxy) * vec4<f32>(0.0, 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e35.g0_.w) * _e39.g0_.zzyx) * vec4<f32>(0.0, -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e51.g0_.x) * _e55.g0_.xyxx) * vec4<f32>(1.0, -(1.0), 0.0, 0.0))));
}

fn motor_dual_motor_geometric_anti_product(self_1950: MotorDual, other_1642: Motor) -> Motor {
    var self_1951: MotorDual;
    var other_1643: Motor;

    self_1951 = self_1950;
    other_1643 = other_1642;
    let _e4: MotorDual = self_1951;
    let _e8: Motor = other_1643;
    let _e11: MotorDual = self_1951;
    let _e15: Motor = other_1643;
    let _e28: MotorDual = self_1951;
    let _e32: Motor = other_1643;
    let _e44: MotorDual = self_1951;
    let _e47: Motor = other_1643;
    return Motor(((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.y) * _e15.g0_.yxwz) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e28.g0_.w) * _e32.g0_.wzww) * vec4<f32>(1.0, -(1.0), 0.0, 0.0))) + ((_e44.g0_.zzxx * _e47.g0_.zwxx) * vec4<f32>(1.0, 1.0, 0.0, 0.0))));
}

fn motor_dual_motor_inner_anti_product(self_1952: MotorDual, other_1644: Motor) -> Motor {
    var self_1953: MotorDual;
    var other_1645: Motor;

    self_1953 = self_1952;
    other_1645 = other_1644;
    let _e4: MotorDual = self_1953;
    let _e8: Motor = other_1645;
    let _e11: MotorDual = self_1953;
    let _e15: Motor = other_1645;
    let _e27: MotorDual = self_1953;
    let _e31: Motor = other_1645;
    let _e44: MotorDual = self_1953;
    let _e47: Motor = other_1645;
    return Motor(((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.z) * vec4<f32>(_e15.g0_.w)) * vec4<f32>(0.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e27.g0_.w) * vec4<f32>(_e31.g0_.z)) * vec4<f32>(0.0, -(1.0), 0.0, 0.0))) + ((_e44.g0_.xyyy * _e47.g0_.xxwz) * vec4<f32>(0.0, -(1.0), -(1.0), 1.0))));
}

fn motor_dual_motor_left_contraction(self_1954: MotorDual, other_1646: Motor) -> Plane {
    var self_1955: MotorDual;
    var other_1647: Motor;

    self_1955 = self_1954;
    other_1647 = other_1646;
    let _e4: MotorDual = self_1955;
    let _e8: Motor = other_1647;
    let _e11: Motor = other_1647;
    let _e14: Motor = other_1647;
    let _e25: MotorDual = self_1955;
    let _e28: MotorDual = self_1955;
    let _e31: MotorDual = self_1955;
    let _e35: Motor = other_1647;
    let _e38: Motor = other_1647;
    let _e41: Motor = other_1647;
    return Plane((((vec3<f32>(_e4.g0_.w) * vec3<f32>(_e8.g0_.z, _e11.g0_.y, _e14.g0_.z)) * vec3<f32>(-(1.0), 1.0, 0.0)) + ((vec3<f32>(_e25.g0_.z, _e28.g0_.x, _e31.g0_.z) * vec3<f32>(_e35.g0_.w, _e38.g0_.x, _e41.g0_.y)) * vec3<f32>(1.0, 0.0, -(1.0)))));
}

fn motor_dual_motor_right_contraction(self_1956: MotorDual, other_1648: Motor) -> MotorDual {
    var self_1957: MotorDual;
    var other_1649: Motor;

    self_1957 = self_1956;
    other_1649 = other_1648;
    let _e4: MotorDual = self_1957;
    let _e8: Motor = other_1649;
    let _e19: MotorDual = self_1957;
    let _e22: Motor = other_1649;
    return MotorDual((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.x)) * vec4<f32>(0.0, 1.0, 0.0, 0.0)) + ((_e19.g0_.xxzw * _e22.g0_.xyxx) * vec4<f32>(1.0, -(1.0), 1.0, 1.0))));
}

fn motor_dual_motor_left_anti_contraction(self_1958: MotorDual, other_1650: Motor) -> Motor {
    var self_1959: MotorDual;
    var other_1651: Motor;

    self_1959 = self_1958;
    other_1651 = other_1650;
    let _e4: MotorDual = self_1959;
    let _e8: Motor = other_1651;
    let _e11: MotorDual = self_1959;
    let _e14: Motor = other_1651;
    return Motor(((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((_e11.g0_.xyxx * vec4<f32>(_e14.g0_.x)) * vec4<f32>(0.0, -(1.0), 0.0, 0.0))));
}

fn motor_dual_motor_right_anti_contraction(self_1960: MotorDual, other_1652: Motor) -> Point {
    var self_1961: MotorDual;
    var other_1653: Motor;

    self_1961 = self_1960;
    other_1653 = other_1652;
    let _e4: MotorDual = self_1961;
    let _e8: Motor = other_1653;
    let _e19: MotorDual = self_1961;
    let _e22: MotorDual = self_1961;
    let _e25: MotorDual = self_1961;
    let _e29: Motor = other_1653;
    let _e32: Motor = other_1653;
    let _e35: Motor = other_1653;
    return Point((((vec3<f32>(_e4.g0_.w) * vec3<f32>(_e8.g0_.z)) * vec3<f32>(-(1.0), 0.0, 0.0)) + ((vec3<f32>(_e19.g0_.z, _e22.g0_.y, _e25.g0_.y) * vec3<f32>(_e29.g0_.w, _e32.g0_.w, _e35.g0_.z)) * vec3<f32>(1.0, -(1.0), 1.0))));
}

fn motor_dual_motor_dual_add(self_1962: MotorDual, other_1654: MotorDual) -> MotorDual {
    var self_1963: MotorDual;
    var other_1655: MotorDual;

    self_1963 = self_1962;
    other_1655 = other_1654;
    let _e4: MotorDual = self_1963;
    let _e6: MotorDual = other_1655;
    return MotorDual((_e4.g0_ + _e6.g0_));
}

fn motor_dual_motor_dual_sub(self_1964: MotorDual, other_1656: MotorDual) -> MotorDual {
    var self_1965: MotorDual;
    var other_1657: MotorDual;

    self_1965 = self_1964;
    other_1657 = other_1656;
    let _e4: MotorDual = self_1965;
    let _e6: MotorDual = other_1657;
    return MotorDual((_e4.g0_ - _e6.g0_));
}

fn motor_dual_motor_dual_mul(self_1966: MotorDual, other_1658: MotorDual) -> MotorDual {
    var self_1967: MotorDual;
    var other_1659: MotorDual;

    self_1967 = self_1966;
    other_1659 = other_1658;
    let _e4: MotorDual = self_1967;
    let _e6: MotorDual = other_1659;
    return MotorDual((_e4.g0_ * _e6.g0_));
}

fn motor_dual_motor_dual_div(self_1968: MotorDual, other_1660: MotorDual) -> MotorDual {
    var self_1969: MotorDual;
    var other_1661: MotorDual;

    self_1969 = self_1968;
    other_1661 = other_1660;
    let _e4: MotorDual = self_1969;
    let _e7: MotorDual = self_1969;
    let _e10: MotorDual = self_1969;
    let _e13: MotorDual = self_1969;
    let _e23: MotorDual = other_1661;
    let _e26: MotorDual = other_1661;
    let _e29: MotorDual = other_1661;
    let _e32: MotorDual = other_1661;
    return MotorDual((((vec4<f32>(_e4.g0_.x, _e7.g0_.y, _e10.g0_.z, _e13.g0_.w) * vec4<f32>(1.0, 1.0, 1.0, 1.0)) / vec4<f32>(_e23.g0_.x, _e26.g0_.y, _e29.g0_.z, _e32.g0_.w)) * vec4<f32>(1.0, 1.0, 1.0, 1.0)));
}

fn motor_dual_motor_dual_geometric_product(self_1970: MotorDual, other_1662: MotorDual) -> Motor {
    var self_1971: MotorDual;
    var other_1663: MotorDual;

    self_1971 = self_1970;
    other_1663 = other_1662;
    let _e4: MotorDual = self_1971;
    let _e8: MotorDual = other_1663;
    let _e19: MotorDual = self_1971;
    let _e23: MotorDual = other_1663;
    let _e35: MotorDual = self_1971;
    let _e39: MotorDual = other_1663;
    let _e51: MotorDual = self_1971;
    let _e55: MotorDual = other_1663;
    return Motor((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.zwxy) * vec4<f32>(1.0, -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e35.g0_.w) * _e39.g0_.wzyx) * vec4<f32>(1.0, 1.0, -(1.0), 1.0))) + ((vec4<f32>(_e51.g0_.x) * _e55.g0_.xxzw) * vec4<f32>(0.0, 0.0, 1.0, 1.0))));
}

fn motor_dual_motor_dual_regressive_product(self_1972: MotorDual, other_1664: MotorDual) -> MotorDual {
    var self_1973: MotorDual;
    var other_1665: MotorDual;

    self_1973 = self_1972;
    other_1665 = other_1664;
    let _e4: MotorDual = self_1973;
    let _e8: MotorDual = other_1665;
    let _e11: MotorDual = self_1973;
    let _e13: MotorDual = other_1665;
    return MotorDual(((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((_e11.g0_ * vec4<f32>(_e13.g0_.x)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn motor_dual_motor_dual_outer_product(self_1974: MotorDual, other_1666: MotorDual) -> Point {
    var self_1975: MotorDual;
    var other_1667: MotorDual;

    self_1975 = self_1974;
    other_1667 = other_1666;
    let _e4: MotorDual = self_1975;
    let _e8: MotorDual = other_1667;
    let _e11: MotorDual = other_1667;
    let _e14: MotorDual = other_1667;
    let _e25: MotorDual = self_1975;
    let _e29: MotorDual = other_1667;
    let _e32: MotorDual = other_1667;
    let _e35: MotorDual = other_1667;
    let _e47: MotorDual = self_1975;
    let _e50: MotorDual = self_1975;
    let _e53: MotorDual = self_1975;
    let _e57: MotorDual = other_1667;
    let _e60: MotorDual = other_1667;
    let _e63: MotorDual = other_1667;
    return Point(((((vec3<f32>(_e4.g0_.z) * vec3<f32>(_e8.g0_.w, _e11.g0_.w, _e14.g0_.y)) * vec3<f32>(-(1.0), 0.0, 1.0)) + ((vec3<f32>(_e25.g0_.w) * vec3<f32>(_e29.g0_.z, _e32.g0_.y, _e35.g0_.z)) * vec3<f32>(1.0, -(1.0), 0.0))) + ((vec3<f32>(_e47.g0_.x, _e50.g0_.y, _e53.g0_.y) * vec3<f32>(_e57.g0_.x, _e60.g0_.w, _e63.g0_.z)) * vec3<f32>(0.0, 1.0, -(1.0)))));
}

fn motor_dual_motor_dual_inner_product(self_1976: MotorDual, other_1668: MotorDual) -> Translator {
    var self_1977: MotorDual;
    var other_1669: MotorDual;

    self_1977 = self_1976;
    other_1669 = other_1668;
    let _e4: MotorDual = self_1977;
    let _e8: MotorDual = other_1669;
    let _e11: MotorDual = other_1669;
    let _e14: MotorDual = other_1669;
    let _e24: MotorDual = self_1977;
    let _e28: MotorDual = other_1669;
    let _e31: MotorDual = other_1669;
    let _e34: MotorDual = other_1669;
    let _e45: MotorDual = self_1977;
    let _e49: MotorDual = other_1669;
    let _e52: MotorDual = other_1669;
    let _e55: MotorDual = other_1669;
    return Translator(((((vec3<f32>(_e4.g0_.z) * vec3<f32>(_e8.g0_.z, _e11.g0_.x, _e14.g0_.z)) * vec3<f32>(1.0, 1.0, 0.0)) + ((vec3<f32>(_e24.g0_.w) * vec3<f32>(_e28.g0_.w, _e31.g0_.w, _e34.g0_.x)) * vec3<f32>(1.0, 0.0, 1.0))) + ((vec3<f32>(_e45.g0_.x) * vec3<f32>(_e49.g0_.x, _e52.g0_.z, _e55.g0_.w)) * vec3<f32>(0.0, 1.0, 1.0))));
}

fn motor_dual_motor_dual_geometric_anti_product(self_1978: MotorDual, other_1670: MotorDual) -> MotorDual {
    var self_1979: MotorDual;
    var other_1671: MotorDual;

    self_1979 = self_1978;
    other_1671 = other_1670;
    let _e4: MotorDual = self_1979;
    let _e8: MotorDual = other_1671;
    let _e11: MotorDual = self_1979;
    let _e15: MotorDual = other_1671;
    let _e28: MotorDual = self_1979;
    let _e32: MotorDual = other_1671;
    let _e43: MotorDual = self_1979;
    let _e46: MotorDual = other_1671;
    return MotorDual(((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.y) * _e15.g0_.yxwz) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))) + ((vec4<f32>(_e28.g0_.w) * _e32.g0_.yyyx) * vec4<f32>(0.0, 0.0, 1.0, 1.0))) + ((_e43.g0_.xxzz * _e46.g0_.xxxy) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))));
}

fn motor_dual_motor_dual_inner_anti_product(self_1980: MotorDual, other_1672: MotorDual) -> MotorDual {
    var self_1981: MotorDual;
    var other_1673: MotorDual;

    self_1981 = self_1980;
    other_1673 = other_1672;
    let _e4: MotorDual = self_1981;
    let _e8: MotorDual = other_1673;
    let _e11: MotorDual = self_1981;
    let _e14: MotorDual = other_1673;
    return MotorDual(((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((_e11.g0_.yyzw * _e14.g0_.yxxx) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0))));
}

fn motor_dual_motor_dual_left_contraction(self_1982: MotorDual, other_1674: MotorDual) -> Translator {
    var self_1983: MotorDual;
    var other_1675: MotorDual;

    self_1983 = self_1982;
    other_1675 = other_1674;
    let _e4: MotorDual = self_1983;
    let _e8: MotorDual = other_1675;
    let _e11: MotorDual = other_1675;
    let _e14: MotorDual = other_1675;
    let _e24: MotorDual = self_1983;
    let _e27: MotorDual = self_1983;
    let _e30: MotorDual = self_1983;
    let _e34: MotorDual = other_1675;
    let _e37: MotorDual = other_1675;
    let _e40: MotorDual = other_1675;
    return Translator((((vec3<f32>(_e4.g0_.w) * vec3<f32>(_e8.g0_.w, _e11.g0_.w, _e14.g0_.x)) * vec3<f32>(1.0, 0.0, 1.0)) + ((vec3<f32>(_e24.g0_.z, _e27.g0_.z, _e30.g0_.x) * vec3<f32>(_e34.g0_.z, _e37.g0_.x, _e40.g0_.x)) * vec3<f32>(1.0, 1.0, 0.0))));
}

fn motor_dual_motor_dual_right_contraction(self_1984: MotorDual, other_1676: MotorDual) -> Translator {
    var self_1985: MotorDual;
    var other_1677: MotorDual;

    self_1985 = self_1984;
    other_1677 = other_1676;
    let _e4: MotorDual = self_1985;
    let _e8: MotorDual = other_1677;
    let _e18: MotorDual = self_1985;
    let _e21: MotorDual = self_1985;
    let _e24: MotorDual = self_1985;
    let _e28: MotorDual = other_1677;
    let _e31: MotorDual = other_1677;
    let _e34: MotorDual = other_1677;
    return Translator((((vec3<f32>(_e4.g0_.w) * vec3<f32>(_e8.g0_.w)) * vec3<f32>(1.0, 0.0, 0.0)) + (vec3<f32>(_e18.g0_.z, _e21.g0_.x, _e24.g0_.x) * vec3<f32>(_e28.g0_.z, _e31.g0_.z, _e34.g0_.w))));
}

fn motor_dual_motor_dual_left_anti_contraction(self_1986: MotorDual, other_1678: MotorDual) -> MotorDual {
    var self_1987: MotorDual;
    var other_1679: MotorDual;

    self_1987 = self_1986;
    other_1679 = other_1678;
    let _e4: MotorDual = self_1987;
    let _e8: MotorDual = other_1679;
    let _e11: MotorDual = self_1987;
    let _e14: MotorDual = other_1679;
    return MotorDual(((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((_e11.g0_.yxxx * _e14.g0_.yxxx) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))));
}

fn motor_dual_motor_dual_right_anti_contraction(self_1988: MotorDual, other_1680: MotorDual) -> MotorDual {
    var self_1989: MotorDual;
    var other_1681: MotorDual;

    self_1989 = self_1988;
    other_1681 = other_1680;
    let _e4: MotorDual = self_1989;
    let _e8: MotorDual = other_1681;
    let _e19: MotorDual = self_1989;
    let _e22: MotorDual = other_1681;
    return MotorDual((((vec4<f32>(_e4.g0_.y) * _e8.g0_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0)) + ((_e19.g0_.xxzw * vec4<f32>(_e22.g0_.x)) * vec4<f32>(1.0, 0.0, 1.0, 1.0))));
}

fn motor_dual_motor_dual_scalar_product(self_1990: MotorDual, other_1682: MotorDual) -> Scalar {
    var self_1991: MotorDual;
    var other_1683: MotorDual;

    self_1991 = self_1990;
    other_1683 = other_1682;
    let _e4: MotorDual = self_1991;
    let _e7: MotorDual = other_1683;
    let _e11: MotorDual = self_1991;
    let _e14: MotorDual = other_1683;
    return Scalar(((_e4.g0_.z * _e7.g0_.z) + (_e11.g0_.w * _e14.g0_.w)));
}

fn motor_dual_motor_dual_anti_scalar_product(self_1992: MotorDual, other_1684: MotorDual) -> AntiScalar {
    var self_1993: MotorDual;
    var other_1685: MotorDual;

    self_1993 = self_1992;
    other_1685 = other_1684;
    let _e4: MotorDual = self_1993;
    let _e7: MotorDual = other_1685;
    let _e11: MotorDual = self_1993;
    let _e14: MotorDual = other_1685;
    return AntiScalar(((_e4.g0_.x * _e7.g0_.x) - (_e11.g0_.y * _e14.g0_.y)));
}

fn motor_dual_squared_magnitude(self_1994: MotorDual) -> Scalar {
    var self_1995: MotorDual;

    self_1995 = self_1994;
    let _e2: MotorDual = self_1995;
    let _e3: MotorDual = self_1995;
    let _e4: MotorDual = motor_dual_reversal(_e3);
    let _e5: Scalar = motor_dual_motor_dual_scalar_product(_e2, _e4);
    return _e5;
}

fn motor_dual_magnitude(self_1996: MotorDual) -> Scalar {
    var self_1997: MotorDual;

    self_1997 = self_1996;
    let _e2: MotorDual = self_1997;
    let _e3: Scalar = motor_dual_squared_magnitude(_e2);
    return Scalar(sqrt(_e3.g0_));
}

fn motor_dual_bulk_norm(self_1998: MotorDual) -> Scalar {
    var self_1999: MotorDual;

    self_1999 = self_1998;
    let _e2: MotorDual = self_1999;
    let _e3: Scalar = motor_dual_squared_magnitude(_e2);
    return Scalar(sqrt(_e3.g0_));
}

fn motor_dual_squared_anti_magnitude(self_2000: MotorDual) -> AntiScalar {
    var self_2001: MotorDual;

    self_2001 = self_2000;
    let _e2: MotorDual = self_2001;
    let _e3: MotorDual = self_2001;
    let _e4: MotorDual = motor_dual_anti_reversal(_e3);
    let _e5: AntiScalar = motor_dual_motor_dual_anti_scalar_product(_e2, _e4);
    return _e5;
}

fn motor_dual_weight_norm(self_2002: MotorDual) -> AntiScalar {
    var self_2003: MotorDual;

    self_2003 = self_2002;
    let _e2: MotorDual = self_2003;
    let _e3: AntiScalar = motor_dual_squared_anti_magnitude(_e2);
    return AntiScalar(sqrt(_e3.g0_));
}

fn motor_dual_scale(self_2004: MotorDual, other_1686: f32) -> MotorDual {
    var self_2005: MotorDual;
    var other_1687: f32;

    self_2005 = self_2004;
    other_1687 = other_1686;
    let _e4: MotorDual = self_2005;
    let _e5: f32 = other_1687;
    let _e7: MotorDual = motor_dual_scalar_geometric_product(_e4, Scalar(_e5));
    return _e7;
}

fn motor_dual_signum(self_2006: MotorDual) -> MotorDual {
    var self_2007: MotorDual;

    self_2007 = self_2006;
    let _e2: MotorDual = self_2007;
    let _e3: MotorDual = self_2007;
    let _e4: Scalar = motor_dual_magnitude(_e3);
    let _e9: MotorDual = motor_dual_scalar_geometric_product(_e2, Scalar((1.0 / _e4.g0_)));
    return _e9;
}

fn motor_dual_inverse(self_2008: MotorDual) -> MotorDual {
    var self_2009: MotorDual;

    self_2009 = self_2008;
    let _e2: MotorDual = self_2009;
    let _e3: MotorDual = motor_dual_reversal(_e2);
    let _e4: MotorDual = self_2009;
    let _e5: Scalar = motor_dual_squared_magnitude(_e4);
    let _e10: MotorDual = motor_dual_scalar_geometric_product(_e3, Scalar((1.0 / _e5.g0_)));
    return _e10;
}

fn motor_dual_unitize(self_2010: MotorDual) -> MotorDual {
    var self_2011: MotorDual;

    self_2011 = self_2010;
    let _e2: MotorDual = self_2011;
    let _e3: MotorDual = self_2011;
    let _e4: AntiScalar = motor_dual_weight_norm(_e3);
    let _e9: MotorDual = motor_dual_scalar_geometric_product(_e2, Scalar((1.0 / _e4.g0_)));
    return _e9;
}

fn anti_scalar_motor_dual_geometric_quotient(self_2012: AntiScalar, other_1688: MotorDual) -> IdealPoint {
    var self_2013: AntiScalar;
    var other_1689: MotorDual;

    self_2013 = self_2012;
    other_1689 = other_1688;
    let _e4: AntiScalar = self_2013;
    let _e5: MotorDual = other_1689;
    let _e6: MotorDual = motor_dual_inverse(_e5);
    let _e7: IdealPoint = anti_scalar_motor_dual_geometric_product(_e4, _e6);
    return _e7;
}

fn anti_scalar_plane_geometric_quotient(self_2014: AntiScalar, other_1690: Plane) -> IdealPoint {
    var self_2015: AntiScalar;
    var other_1691: Plane;

    self_2015 = self_2014;
    other_1691 = other_1690;
    let _e4: AntiScalar = self_2015;
    let _e5: Plane = other_1691;
    let _e6: Plane = plane_inverse(_e5);
    let _e7: IdealPoint = anti_scalar_plane_geometric_product(_e4, _e6);
    return _e7;
}

fn anti_scalar_scalar_geometric_quotient(self_2016: AntiScalar, other_1692: Scalar) -> AntiScalar {
    var self_2017: AntiScalar;
    var other_1693: Scalar;

    self_2017 = self_2016;
    other_1693 = other_1692;
    let _e4: AntiScalar = self_2017;
    let _e5: Scalar = other_1693;
    let _e6: Scalar = scalar_inverse(_e5);
    let _e7: AntiScalar = anti_scalar_scalar_geometric_product(_e4, _e6);
    return _e7;
}

fn anti_scalar_translator_geometric_quotient(self_2018: AntiScalar, other_1694: Translator) -> AntiScalar {
    var self_2019: AntiScalar;
    var other_1695: Translator;

    self_2019 = self_2018;
    other_1695 = other_1694;
    let _e4: AntiScalar = self_2019;
    let _e5: Translator = other_1695;
    let _e6: Translator = translator_inverse(_e5);
    let _e7: AntiScalar = anti_scalar_translator_geometric_product(_e4, _e6);
    return _e7;
}

fn ideal_point_motor_geometric_quotient(self_2020: IdealPoint, other_1696: Motor) -> IdealPoint {
    var self_2021: IdealPoint;
    var other_1697: Motor;

    self_2021 = self_2020;
    other_1697 = other_1696;
    let _e4: IdealPoint = self_2021;
    let _e5: Motor = other_1697;
    let _e6: Motor = motor_inverse(_e5);
    let _e7: IdealPoint = ideal_point_motor_geometric_product(_e4, _e6);
    return _e7;
}

fn ideal_point_point_geometric_quotient(self_2022: IdealPoint, other_1698: Point) -> IdealPoint {
    var self_2023: IdealPoint;
    var other_1699: Point;

    self_2023 = self_2022;
    other_1699 = other_1698;
    let _e4: IdealPoint = self_2023;
    let _e5: Point = other_1699;
    let _e6: Point = point_inverse(_e5);
    let _e7: IdealPoint = ideal_point_point_geometric_product(_e4, _e6);
    return _e7;
}

fn ideal_point_rotor_geometric_quotient(self_2024: IdealPoint, other_1700: Rotor) -> IdealPoint {
    var self_2025: IdealPoint;
    var other_1701: Rotor;

    self_2025 = self_2024;
    other_1701 = other_1700;
    let _e4: IdealPoint = self_2025;
    let _e5: Rotor = other_1701;
    let _e6: Rotor = rotor_inverse(_e5);
    let _e7: IdealPoint = ideal_point_rotor_geometric_product(_e4, _e6);
    return _e7;
}

fn ideal_point_scalar_geometric_quotient(self_2026: IdealPoint, other_1702: Scalar) -> IdealPoint {
    var self_2027: IdealPoint;
    var other_1703: Scalar;

    self_2027 = self_2026;
    other_1703 = other_1702;
    let _e4: IdealPoint = self_2027;
    let _e5: Scalar = other_1703;
    let _e6: Scalar = scalar_inverse(_e5);
    let _e7: IdealPoint = ideal_point_scalar_geometric_product(_e4, _e6);
    return _e7;
}

fn ideal_point_translator_geometric_quotient(self_2028: IdealPoint, other_1704: Translator) -> IdealPoint {
    var self_2029: IdealPoint;
    var other_1705: Translator;

    self_2029 = self_2028;
    other_1705 = other_1704;
    let _e4: IdealPoint = self_2029;
    let _e5: Translator = other_1705;
    let _e6: Translator = translator_inverse(_e5);
    let _e7: IdealPoint = ideal_point_translator_geometric_product(_e4, _e6);
    return _e7;
}

fn motor_ideal_point_transformation(self_2030: Motor, other_1706: IdealPoint) -> IdealPoint {
    var self_2031: Motor;
    var other_1707: IdealPoint;

    self_2031 = self_2030;
    other_1707 = other_1706;
    let _e4: Motor = self_2031;
    let _e5: IdealPoint = other_1707;
    let _e6: IdealPoint = motor_ideal_point_geometric_product(_e4, _e5);
    let _e7: Motor = self_2031;
    let _e8: Motor = motor_reversal(_e7);
    let _e9: IdealPoint = ideal_point_motor_geometric_product(_e6, _e8);
    return _e9;
}

fn motor_powi(self_2032: Motor, exponent: i32) -> Motor {
    var self_2033: Motor;
    var exponent_1: i32;
    var local: Motor;
    var x: Motor;
    var y: Motor;
    var n: i32;

    self_2033 = self_2032;
    exponent_1 = exponent;
    let _e4: i32 = exponent_1;
    if (_e4 == 0) {
        {
            let _e7: Motor = motor_one();
            return _e7;
        }
    }
    let _e8: i32 = exponent_1;
    if (_e8 < 0) {
        let _e11: Motor = self_2033;
        let _e12: Motor = motor_inverse(_e11);
        local = _e12;
    } else {
        let _e14: Motor = self_2033;
        local = _e14;
    }
    let _e15: Motor = local;
    x = _e15;
    let _e17: Motor = motor_one();
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
                    let _e31: Motor = x;
                    let _e32: Motor = y;
                    let _e33: Motor = motor_motor_geometric_product(_e31, _e32);
                    y = _e33;
                }
            }
            let _e34: Motor = x;
            let _e35: Motor = x;
            let _e36: Motor = motor_motor_geometric_product(_e34, _e35);
            x = _e36;
            let _e37: i32 = n;
            n = (_e37 >> u32(1));
        }
    }
    let _e41: Motor = x;
    let _e42: Motor = y;
    let _e43: Motor = motor_motor_geometric_product(_e41, _e42);
    return _e43;
}

fn motor_motor_geometric_quotient(self_2034: Motor, other_1708: Motor) -> Motor {
    var self_2035: Motor;
    var other_1709: Motor;

    self_2035 = self_2034;
    other_1709 = other_1708;
    let _e4: Motor = self_2035;
    let _e5: Motor = other_1709;
    let _e6: Motor = motor_inverse(_e5);
    let _e7: Motor = motor_motor_geometric_product(_e4, _e6);
    return _e7;
}

fn motor_motor_transformation(self_2036: Motor, other_1710: Motor) -> Motor {
    var self_2037: Motor;
    var other_1711: Motor;

    self_2037 = self_2036;
    other_1711 = other_1710;
    let _e4: Motor = self_2037;
    let _e5: Motor = other_1711;
    let _e6: Motor = motor_motor_geometric_product(_e4, _e5);
    let _e7: Motor = self_2037;
    let _e8: Motor = motor_reversal(_e7);
    let _e9: Motor = motor_motor_geometric_product(_e6, _e8);
    return _e9;
}

fn motor_motor_dual_geometric_quotient(self_2038: Motor, other_1712: MotorDual) -> MotorDual {
    var self_2039: Motor;
    var other_1713: MotorDual;

    self_2039 = self_2038;
    other_1713 = other_1712;
    let _e4: Motor = self_2039;
    let _e5: MotorDual = other_1713;
    let _e6: MotorDual = motor_dual_inverse(_e5);
    let _e7: MotorDual = motor_motor_dual_geometric_product(_e4, _e6);
    return _e7;
}

fn motor_motor_dual_transformation(self_2040: Motor, other_1714: MotorDual) -> MotorDual {
    var self_2041: Motor;
    var other_1715: MotorDual;

    self_2041 = self_2040;
    other_1715 = other_1714;
    let _e4: Motor = self_2041;
    let _e5: MotorDual = other_1715;
    let _e6: MotorDual = motor_motor_dual_geometric_product(_e4, _e5);
    let _e7: Motor = self_2041;
    let _e8: Motor = motor_reversal(_e7);
    let _e9: MotorDual = motor_dual_motor_geometric_product(_e6, _e8);
    return _e9;
}

fn motor_multi_vector_geometric_quotient(self_2042: Motor, other_1716: MultiVector) -> MultiVector {
    var self_2043: Motor;
    var other_1717: MultiVector;

    self_2043 = self_2042;
    other_1717 = other_1716;
    let _e4: Motor = self_2043;
    let _e5: MultiVector = other_1717;
    let _e6: MultiVector = multi_vector_inverse(_e5);
    let _e7: MultiVector = motor_multi_vector_geometric_product(_e4, _e6);
    return _e7;
}

fn motor_multi_vector_transformation(self_2044: Motor, other_1718: MultiVector) -> MultiVector {
    var self_2045: Motor;
    var other_1719: MultiVector;

    self_2045 = self_2044;
    other_1719 = other_1718;
    let _e4: Motor = self_2045;
    let _e5: MultiVector = other_1719;
    let _e6: MultiVector = motor_multi_vector_geometric_product(_e4, _e5);
    let _e7: Motor = self_2045;
    let _e8: Motor = motor_reversal(_e7);
    let _e9: MultiVector = multi_vector_motor_geometric_product(_e6, _e8);
    return _e9;
}

fn motor_plane_geometric_quotient(self_2046: Motor, other_1720: Plane) -> MotorDual {
    var self_2047: Motor;
    var other_1721: Plane;

    self_2047 = self_2046;
    other_1721 = other_1720;
    let _e4: Motor = self_2047;
    let _e5: Plane = other_1721;
    let _e6: Plane = plane_inverse(_e5);
    let _e7: MotorDual = motor_plane_geometric_product(_e4, _e6);
    return _e7;
}

fn motor_plane_transformation(self_2048: Motor, other_1722: Plane) -> Plane {
    var self_2049: Motor;
    var other_1723: Plane;

    self_2049 = self_2048;
    other_1723 = other_1722;
    let _e4: Motor = self_2049;
    let _e5: Plane = other_1723;
    let _e6: MotorDual = motor_plane_geometric_product(_e4, _e5);
    let _e7: Motor = self_2049;
    let _e8: Motor = motor_reversal(_e7);
    let _e9: MotorDual = motor_dual_motor_geometric_product(_e6, _e8);
    let _e10: Plane = motor_dual_plane_into(_e9);
    return _e10;
}

fn motor_point_geometric_quotient(self_2050: Motor, other_1724: Point) -> Motor {
    var self_2051: Motor;
    var other_1725: Point;

    self_2051 = self_2050;
    other_1725 = other_1724;
    let _e4: Motor = self_2051;
    let _e5: Point = other_1725;
    let _e6: Point = point_inverse(_e5);
    let _e7: Motor = motor_point_geometric_product(_e4, _e6);
    return _e7;
}

fn motor_point_transformation(self_2052: Motor, other_1726: Point) -> Point {
    var self_2053: Motor;
    var other_1727: Point;

    self_2053 = self_2052;
    other_1727 = other_1726;
    let _e4: Motor = self_2053;
    let _e5: Point = other_1727;
    let _e6: Motor = motor_point_geometric_product(_e4, _e5);
    let _e7: Motor = self_2053;
    let _e8: Motor = motor_reversal(_e7);
    let _e9: Motor = motor_motor_geometric_product(_e6, _e8);
    let _e10: Point = motor_point_into(_e9);
    return _e10;
}

fn motor_rotor_geometric_quotient(self_2054: Motor, other_1728: Rotor) -> Motor {
    var self_2055: Motor;
    var other_1729: Rotor;

    self_2055 = self_2054;
    other_1729 = other_1728;
    let _e4: Motor = self_2055;
    let _e5: Rotor = other_1729;
    let _e6: Rotor = rotor_inverse(_e5);
    let _e7: Motor = motor_rotor_geometric_product(_e4, _e6);
    return _e7;
}

fn motor_rotor_transformation(self_2056: Motor, other_1730: Rotor) -> Rotor {
    var self_2057: Motor;
    var other_1731: Rotor;

    self_2057 = self_2056;
    other_1731 = other_1730;
    let _e4: Motor = self_2057;
    let _e5: Rotor = other_1731;
    let _e6: Motor = motor_rotor_geometric_product(_e4, _e5);
    let _e7: Motor = self_2057;
    let _e8: Motor = motor_reversal(_e7);
    let _e9: Motor = motor_motor_geometric_product(_e6, _e8);
    let _e10: Rotor = motor_rotor_into(_e9);
    return _e10;
}

fn motor_scalar_geometric_quotient(self_2058: Motor, other_1732: Scalar) -> Motor {
    var self_2059: Motor;
    var other_1733: Scalar;

    self_2059 = self_2058;
    other_1733 = other_1732;
    let _e4: Motor = self_2059;
    let _e5: Scalar = other_1733;
    let _e6: Scalar = scalar_inverse(_e5);
    let _e7: Motor = motor_scalar_geometric_product(_e4, _e6);
    return _e7;
}

fn motor_scalar_transformation(self_2060: Motor, other_1734: Scalar) -> Scalar {
    var self_2061: Motor;
    var other_1735: Scalar;

    self_2061 = self_2060;
    other_1735 = other_1734;
    let _e4: Motor = self_2061;
    let _e5: Scalar = other_1735;
    let _e6: Motor = motor_scalar_geometric_product(_e4, _e5);
    let _e7: Motor = self_2061;
    let _e8: Motor = motor_reversal(_e7);
    let _e9: Motor = motor_motor_geometric_product(_e6, _e8);
    let _e10: Scalar = motor_scalar_into(_e9);
    return _e10;
}

fn motor_translator_geometric_quotient(self_2062: Motor, other_1736: Translator) -> Motor {
    var self_2063: Motor;
    var other_1737: Translator;

    self_2063 = self_2062;
    other_1737 = other_1736;
    let _e4: Motor = self_2063;
    let _e5: Translator = other_1737;
    let _e6: Translator = translator_inverse(_e5);
    let _e7: Motor = motor_translator_geometric_product(_e4, _e6);
    return _e7;
}

fn motor_translator_transformation(self_2064: Motor, other_1738: Translator) -> Translator {
    var self_2065: Motor;
    var other_1739: Translator;

    self_2065 = self_2064;
    other_1739 = other_1738;
    let _e4: Motor = self_2065;
    let _e5: Translator = other_1739;
    let _e6: Motor = motor_translator_geometric_product(_e4, _e5);
    let _e7: Motor = self_2065;
    let _e8: Motor = motor_reversal(_e7);
    let _e9: Motor = motor_motor_geometric_product(_e6, _e8);
    let _e10: Translator = motor_translator_into(_e9);
    return _e10;
}

fn motor_dual_motor_geometric_quotient(self_2066: MotorDual, other_1740: Motor) -> MotorDual {
    var self_2067: MotorDual;
    var other_1741: Motor;

    self_2067 = self_2066;
    other_1741 = other_1740;
    let _e4: MotorDual = self_2067;
    let _e5: Motor = other_1741;
    let _e6: Motor = motor_inverse(_e5);
    let _e7: MotorDual = motor_dual_motor_geometric_product(_e4, _e6);
    return _e7;
}

fn motor_dual_motor_transformation(self_2068: MotorDual, other_1742: Motor) -> Motor {
    var self_2069: MotorDual;
    var other_1743: Motor;

    self_2069 = self_2068;
    other_1743 = other_1742;
    let _e4: MotorDual = self_2069;
    let _e5: Motor = other_1743;
    let _e6: MotorDual = motor_dual_motor_geometric_product(_e4, _e5);
    let _e7: MotorDual = self_2069;
    let _e8: MotorDual = motor_dual_reversal(_e7);
    let _e9: Motor = motor_dual_motor_dual_geometric_product(_e6, _e8);
    return _e9;
}

fn motor_dual_motor_dual_geometric_quotient(self_2070: MotorDual, other_1744: MotorDual) -> Motor {
    var self_2071: MotorDual;
    var other_1745: MotorDual;

    self_2071 = self_2070;
    other_1745 = other_1744;
    let _e4: MotorDual = self_2071;
    let _e5: MotorDual = other_1745;
    let _e6: MotorDual = motor_dual_inverse(_e5);
    let _e7: Motor = motor_dual_motor_dual_geometric_product(_e4, _e6);
    return _e7;
}

fn motor_dual_motor_dual_transformation(self_2072: MotorDual, other_1746: MotorDual) -> MotorDual {
    var self_2073: MotorDual;
    var other_1747: MotorDual;

    self_2073 = self_2072;
    other_1747 = other_1746;
    let _e4: MotorDual = self_2073;
    let _e5: MotorDual = other_1747;
    let _e6: Motor = motor_dual_motor_dual_geometric_product(_e4, _e5);
    let _e7: MotorDual = self_2073;
    let _e8: MotorDual = motor_dual_reversal(_e7);
    let _e9: MotorDual = motor_motor_dual_geometric_product(_e6, _e8);
    return _e9;
}

fn motor_dual_multi_vector_geometric_quotient(self_2074: MotorDual, other_1748: MultiVector) -> MultiVector {
    var self_2075: MotorDual;
    var other_1749: MultiVector;

    self_2075 = self_2074;
    other_1749 = other_1748;
    let _e4: MotorDual = self_2075;
    let _e5: MultiVector = other_1749;
    let _e6: MultiVector = multi_vector_inverse(_e5);
    let _e7: MultiVector = motor_dual_multi_vector_geometric_product(_e4, _e6);
    return _e7;
}

fn motor_dual_multi_vector_transformation(self_2076: MotorDual, other_1750: MultiVector) -> MultiVector {
    var self_2077: MotorDual;
    var other_1751: MultiVector;

    self_2077 = self_2076;
    other_1751 = other_1750;
    let _e4: MotorDual = self_2077;
    let _e5: MultiVector = other_1751;
    let _e6: MultiVector = motor_dual_multi_vector_geometric_product(_e4, _e5);
    let _e7: MotorDual = self_2077;
    let _e8: MotorDual = motor_dual_reversal(_e7);
    let _e9: MultiVector = multi_vector_motor_dual_geometric_product(_e6, _e8);
    return _e9;
}

fn motor_dual_plane_geometric_quotient(self_2078: MotorDual, other_1752: Plane) -> Motor {
    var self_2079: MotorDual;
    var other_1753: Plane;

    self_2079 = self_2078;
    other_1753 = other_1752;
    let _e4: MotorDual = self_2079;
    let _e5: Plane = other_1753;
    let _e6: Plane = plane_inverse(_e5);
    let _e7: Motor = motor_dual_plane_geometric_product(_e4, _e6);
    return _e7;
}

fn motor_dual_plane_transformation(self_2080: MotorDual, other_1754: Plane) -> Plane {
    var self_2081: MotorDual;
    var other_1755: Plane;

    self_2081 = self_2080;
    other_1755 = other_1754;
    let _e4: MotorDual = self_2081;
    let _e5: Plane = other_1755;
    let _e6: Motor = motor_dual_plane_geometric_product(_e4, _e5);
    let _e7: MotorDual = self_2081;
    let _e8: MotorDual = motor_dual_reversal(_e7);
    let _e9: MotorDual = motor_motor_dual_geometric_product(_e6, _e8);
    let _e10: Plane = motor_dual_plane_into(_e9);
    return _e10;
}

fn motor_dual_point_geometric_quotient(self_2082: MotorDual, other_1756: Point) -> MotorDual {
    var self_2083: MotorDual;
    var other_1757: Point;

    self_2083 = self_2082;
    other_1757 = other_1756;
    let _e4: MotorDual = self_2083;
    let _e5: Point = other_1757;
    let _e6: Point = point_inverse(_e5);
    let _e7: MotorDual = motor_dual_point_geometric_product(_e4, _e6);
    return _e7;
}

fn motor_dual_point_transformation(self_2084: MotorDual, other_1758: Point) -> Point {
    var self_2085: MotorDual;
    var other_1759: Point;

    self_2085 = self_2084;
    other_1759 = other_1758;
    let _e4: MotorDual = self_2085;
    let _e5: Point = other_1759;
    let _e6: MotorDual = motor_dual_point_geometric_product(_e4, _e5);
    let _e7: MotorDual = self_2085;
    let _e8: MotorDual = motor_dual_reversal(_e7);
    let _e9: Motor = motor_dual_motor_dual_geometric_product(_e6, _e8);
    let _e10: Point = motor_point_into(_e9);
    return _e10;
}

fn motor_dual_rotor_geometric_quotient(self_2086: MotorDual, other_1760: Rotor) -> MotorDual {
    var self_2087: MotorDual;
    var other_1761: Rotor;

    self_2087 = self_2086;
    other_1761 = other_1760;
    let _e4: MotorDual = self_2087;
    let _e5: Rotor = other_1761;
    let _e6: Rotor = rotor_inverse(_e5);
    let _e7: MotorDual = motor_dual_rotor_geometric_product(_e4, _e6);
    return _e7;
}

fn motor_dual_rotor_transformation(self_2088: MotorDual, other_1762: Rotor) -> Rotor {
    var self_2089: MotorDual;
    var other_1763: Rotor;

    self_2089 = self_2088;
    other_1763 = other_1762;
    let _e4: MotorDual = self_2089;
    let _e5: Rotor = other_1763;
    let _e6: MotorDual = motor_dual_rotor_geometric_product(_e4, _e5);
    let _e7: MotorDual = self_2089;
    let _e8: MotorDual = motor_dual_reversal(_e7);
    let _e9: Motor = motor_dual_motor_dual_geometric_product(_e6, _e8);
    let _e10: Rotor = motor_rotor_into(_e9);
    return _e10;
}

fn motor_dual_scalar_geometric_quotient(self_2090: MotorDual, other_1764: Scalar) -> MotorDual {
    var self_2091: MotorDual;
    var other_1765: Scalar;

    self_2091 = self_2090;
    other_1765 = other_1764;
    let _e4: MotorDual = self_2091;
    let _e5: Scalar = other_1765;
    let _e6: Scalar = scalar_inverse(_e5);
    let _e7: MotorDual = motor_dual_scalar_geometric_product(_e4, _e6);
    return _e7;
}

fn motor_dual_scalar_transformation(self_2092: MotorDual, other_1766: Scalar) -> Scalar {
    var self_2093: MotorDual;
    var other_1767: Scalar;

    self_2093 = self_2092;
    other_1767 = other_1766;
    let _e4: MotorDual = self_2093;
    let _e5: Scalar = other_1767;
    let _e6: MotorDual = motor_dual_scalar_geometric_product(_e4, _e5);
    let _e7: MotorDual = self_2093;
    let _e8: MotorDual = motor_dual_reversal(_e7);
    let _e9: Motor = motor_dual_motor_dual_geometric_product(_e6, _e8);
    let _e10: Scalar = motor_scalar_into(_e9);
    return _e10;
}

fn motor_dual_translator_geometric_quotient(self_2094: MotorDual, other_1768: Translator) -> MotorDual {
    var self_2095: MotorDual;
    var other_1769: Translator;

    self_2095 = self_2094;
    other_1769 = other_1768;
    let _e4: MotorDual = self_2095;
    let _e5: Translator = other_1769;
    let _e6: Translator = translator_inverse(_e5);
    let _e7: MotorDual = motor_dual_translator_geometric_product(_e4, _e6);
    return _e7;
}

fn motor_dual_translator_transformation(self_2096: MotorDual, other_1770: Translator) -> Translator {
    var self_2097: MotorDual;
    var other_1771: Translator;

    self_2097 = self_2096;
    other_1771 = other_1770;
    let _e4: MotorDual = self_2097;
    let _e5: Translator = other_1771;
    let _e6: MotorDual = motor_dual_translator_geometric_product(_e4, _e5);
    let _e7: MotorDual = self_2097;
    let _e8: MotorDual = motor_dual_reversal(_e7);
    let _e9: Motor = motor_dual_motor_dual_geometric_product(_e6, _e8);
    let _e10: Translator = motor_translator_into(_e9);
    return _e10;
}

fn multi_vector_motor_geometric_quotient(self_2098: MultiVector, other_1772: Motor) -> MultiVector {
    var self_2099: MultiVector;
    var other_1773: Motor;

    self_2099 = self_2098;
    other_1773 = other_1772;
    let _e4: MultiVector = self_2099;
    let _e5: Motor = other_1773;
    let _e6: Motor = motor_inverse(_e5);
    let _e7: MultiVector = multi_vector_motor_geometric_product(_e4, _e6);
    return _e7;
}

fn multi_vector_motor_transformation(self_2100: MultiVector, other_1774: Motor) -> Motor {
    var self_2101: MultiVector;
    var other_1775: Motor;

    self_2101 = self_2100;
    other_1775 = other_1774;
    let _e4: MultiVector = self_2101;
    let _e5: Motor = other_1775;
    let _e6: MultiVector = multi_vector_motor_geometric_product(_e4, _e5);
    let _e7: MultiVector = self_2101;
    let _e8: MultiVector = multi_vector_reversal(_e7);
    let _e9: MultiVector = multi_vector_multi_vector_geometric_product(_e6, _e8);
    let _e10: Motor = multi_vector_motor_into(_e9);
    return _e10;
}

fn multi_vector_motor_dual_geometric_quotient(self_2102: MultiVector, other_1776: MotorDual) -> MultiVector {
    var self_2103: MultiVector;
    var other_1777: MotorDual;

    self_2103 = self_2102;
    other_1777 = other_1776;
    let _e4: MultiVector = self_2103;
    let _e5: MotorDual = other_1777;
    let _e6: MotorDual = motor_dual_inverse(_e5);
    let _e7: MultiVector = multi_vector_motor_dual_geometric_product(_e4, _e6);
    return _e7;
}

fn multi_vector_motor_dual_transformation(self_2104: MultiVector, other_1778: MotorDual) -> MotorDual {
    var self_2105: MultiVector;
    var other_1779: MotorDual;

    self_2105 = self_2104;
    other_1779 = other_1778;
    let _e4: MultiVector = self_2105;
    let _e5: MotorDual = other_1779;
    let _e6: MultiVector = multi_vector_motor_dual_geometric_product(_e4, _e5);
    let _e7: MultiVector = self_2105;
    let _e8: MultiVector = multi_vector_reversal(_e7);
    let _e9: MultiVector = multi_vector_multi_vector_geometric_product(_e6, _e8);
    let _e10: MotorDual = multi_vector_motor_dual_into(_e9);
    return _e10;
}

fn multi_vector_powi(self_2106: MultiVector, exponent_2: i32) -> MultiVector {
    var self_2107: MultiVector;
    var exponent_3: i32;
    var local_1: MultiVector;
    var x_1: MultiVector;
    var y_1: MultiVector;
    var n_1: i32;

    self_2107 = self_2106;
    exponent_3 = exponent_2;
    let _e4: i32 = exponent_3;
    if (_e4 == 0) {
        {
            let _e7: MultiVector = multi_vector_one();
            return _e7;
        }
    }
    let _e8: i32 = exponent_3;
    if (_e8 < 0) {
        let _e11: MultiVector = self_2107;
        let _e12: MultiVector = multi_vector_inverse(_e11);
        local_1 = _e12;
    } else {
        let _e14: MultiVector = self_2107;
        local_1 = _e14;
    }
    let _e15: MultiVector = local_1;
    x_1 = _e15;
    let _e17: MultiVector = multi_vector_one();
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
                    let _e31: MultiVector = x_1;
                    let _e32: MultiVector = y_1;
                    let _e33: MultiVector = multi_vector_multi_vector_geometric_product(_e31, _e32);
                    y_1 = _e33;
                }
            }
            let _e34: MultiVector = x_1;
            let _e35: MultiVector = x_1;
            let _e36: MultiVector = multi_vector_multi_vector_geometric_product(_e34, _e35);
            x_1 = _e36;
            let _e37: i32 = n_1;
            n_1 = (_e37 >> u32(1));
        }
    }
    let _e41: MultiVector = x_1;
    let _e42: MultiVector = y_1;
    let _e43: MultiVector = multi_vector_multi_vector_geometric_product(_e41, _e42);
    return _e43;
}

fn multi_vector_multi_vector_geometric_quotient(self_2108: MultiVector, other_1780: MultiVector) -> MultiVector {
    var self_2109: MultiVector;
    var other_1781: MultiVector;

    self_2109 = self_2108;
    other_1781 = other_1780;
    let _e4: MultiVector = self_2109;
    let _e5: MultiVector = other_1781;
    let _e6: MultiVector = multi_vector_inverse(_e5);
    let _e7: MultiVector = multi_vector_multi_vector_geometric_product(_e4, _e6);
    return _e7;
}

fn multi_vector_multi_vector_transformation(self_2110: MultiVector, other_1782: MultiVector) -> MultiVector {
    var self_2111: MultiVector;
    var other_1783: MultiVector;

    self_2111 = self_2110;
    other_1783 = other_1782;
    let _e4: MultiVector = self_2111;
    let _e5: MultiVector = other_1783;
    let _e6: MultiVector = multi_vector_multi_vector_geometric_product(_e4, _e5);
    let _e7: MultiVector = self_2111;
    let _e8: MultiVector = multi_vector_reversal(_e7);
    let _e9: MultiVector = multi_vector_multi_vector_geometric_product(_e6, _e8);
    return _e9;
}

fn multi_vector_plane_geometric_quotient(self_2112: MultiVector, other_1784: Plane) -> MultiVector {
    var self_2113: MultiVector;
    var other_1785: Plane;

    self_2113 = self_2112;
    other_1785 = other_1784;
    let _e4: MultiVector = self_2113;
    let _e5: Plane = other_1785;
    let _e6: Plane = plane_inverse(_e5);
    let _e7: MultiVector = multi_vector_plane_geometric_product(_e4, _e6);
    return _e7;
}

fn multi_vector_plane_transformation(self_2114: MultiVector, other_1786: Plane) -> Plane {
    var self_2115: MultiVector;
    var other_1787: Plane;

    self_2115 = self_2114;
    other_1787 = other_1786;
    let _e4: MultiVector = self_2115;
    let _e5: Plane = other_1787;
    let _e6: MultiVector = multi_vector_plane_geometric_product(_e4, _e5);
    let _e7: MultiVector = self_2115;
    let _e8: MultiVector = multi_vector_reversal(_e7);
    let _e9: MultiVector = multi_vector_multi_vector_geometric_product(_e6, _e8);
    let _e10: Plane = multi_vector_plane_into(_e9);
    return _e10;
}

fn multi_vector_point_geometric_quotient(self_2116: MultiVector, other_1788: Point) -> MultiVector {
    var self_2117: MultiVector;
    var other_1789: Point;

    self_2117 = self_2116;
    other_1789 = other_1788;
    let _e4: MultiVector = self_2117;
    let _e5: Point = other_1789;
    let _e6: Point = point_inverse(_e5);
    let _e7: MultiVector = multi_vector_point_geometric_product(_e4, _e6);
    return _e7;
}

fn multi_vector_point_transformation(self_2118: MultiVector, other_1790: Point) -> Point {
    var self_2119: MultiVector;
    var other_1791: Point;

    self_2119 = self_2118;
    other_1791 = other_1790;
    let _e4: MultiVector = self_2119;
    let _e5: Point = other_1791;
    let _e6: MultiVector = multi_vector_point_geometric_product(_e4, _e5);
    let _e7: MultiVector = self_2119;
    let _e8: MultiVector = multi_vector_reversal(_e7);
    let _e9: MultiVector = multi_vector_multi_vector_geometric_product(_e6, _e8);
    let _e10: Point = multi_vector_point_into(_e9);
    return _e10;
}

fn multi_vector_rotor_geometric_quotient(self_2120: MultiVector, other_1792: Rotor) -> MultiVector {
    var self_2121: MultiVector;
    var other_1793: Rotor;

    self_2121 = self_2120;
    other_1793 = other_1792;
    let _e4: MultiVector = self_2121;
    let _e5: Rotor = other_1793;
    let _e6: Rotor = rotor_inverse(_e5);
    let _e7: MultiVector = multi_vector_rotor_geometric_product(_e4, _e6);
    return _e7;
}

fn multi_vector_rotor_transformation(self_2122: MultiVector, other_1794: Rotor) -> Rotor {
    var self_2123: MultiVector;
    var other_1795: Rotor;

    self_2123 = self_2122;
    other_1795 = other_1794;
    let _e4: MultiVector = self_2123;
    let _e5: Rotor = other_1795;
    let _e6: MultiVector = multi_vector_rotor_geometric_product(_e4, _e5);
    let _e7: MultiVector = self_2123;
    let _e8: MultiVector = multi_vector_reversal(_e7);
    let _e9: MultiVector = multi_vector_multi_vector_geometric_product(_e6, _e8);
    let _e10: Rotor = multi_vector_rotor_into(_e9);
    return _e10;
}

fn multi_vector_scalar_geometric_quotient(self_2124: MultiVector, other_1796: Scalar) -> MultiVector {
    var self_2125: MultiVector;
    var other_1797: Scalar;

    self_2125 = self_2124;
    other_1797 = other_1796;
    let _e4: MultiVector = self_2125;
    let _e5: Scalar = other_1797;
    let _e6: Scalar = scalar_inverse(_e5);
    let _e7: MultiVector = multi_vector_scalar_geometric_product(_e4, _e6);
    return _e7;
}

fn multi_vector_scalar_transformation(self_2126: MultiVector, other_1798: Scalar) -> Scalar {
    var self_2127: MultiVector;
    var other_1799: Scalar;

    self_2127 = self_2126;
    other_1799 = other_1798;
    let _e4: MultiVector = self_2127;
    let _e5: Scalar = other_1799;
    let _e6: MultiVector = multi_vector_scalar_geometric_product(_e4, _e5);
    let _e7: MultiVector = self_2127;
    let _e8: MultiVector = multi_vector_reversal(_e7);
    let _e9: MultiVector = multi_vector_multi_vector_geometric_product(_e6, _e8);
    let _e10: Scalar = multi_vector_scalar_into(_e9);
    return _e10;
}

fn multi_vector_translator_geometric_quotient(self_2128: MultiVector, other_1800: Translator) -> MultiVector {
    var self_2129: MultiVector;
    var other_1801: Translator;

    self_2129 = self_2128;
    other_1801 = other_1800;
    let _e4: MultiVector = self_2129;
    let _e5: Translator = other_1801;
    let _e6: Translator = translator_inverse(_e5);
    let _e7: MultiVector = multi_vector_translator_geometric_product(_e4, _e6);
    return _e7;
}

fn multi_vector_translator_transformation(self_2130: MultiVector, other_1802: Translator) -> Translator {
    var self_2131: MultiVector;
    var other_1803: Translator;

    self_2131 = self_2130;
    other_1803 = other_1802;
    let _e4: MultiVector = self_2131;
    let _e5: Translator = other_1803;
    let _e6: MultiVector = multi_vector_translator_geometric_product(_e4, _e5);
    let _e7: MultiVector = self_2131;
    let _e8: MultiVector = multi_vector_reversal(_e7);
    let _e9: MultiVector = multi_vector_multi_vector_geometric_product(_e6, _e8);
    let _e10: Translator = multi_vector_translator_into(_e9);
    return _e10;
}

fn plane_motor_geometric_quotient(self_2132: Plane, other_1804: Motor) -> MotorDual {
    var self_2133: Plane;
    var other_1805: Motor;

    self_2133 = self_2132;
    other_1805 = other_1804;
    let _e4: Plane = self_2133;
    let _e5: Motor = other_1805;
    let _e6: Motor = motor_inverse(_e5);
    let _e7: MotorDual = plane_motor_geometric_product(_e4, _e6);
    return _e7;
}

fn plane_motor_transformation(self_2134: Plane, other_1806: Motor) -> Motor {
    var self_2135: Plane;
    var other_1807: Motor;

    self_2135 = self_2134;
    other_1807 = other_1806;
    let _e4: Plane = self_2135;
    let _e5: Motor = other_1807;
    let _e6: MotorDual = plane_motor_geometric_product(_e4, _e5);
    let _e7: Plane = self_2135;
    let _e8: Plane = plane_reversal(_e7);
    let _e9: Motor = motor_dual_plane_geometric_product(_e6, _e8);
    return _e9;
}

fn plane_motor_dual_geometric_quotient(self_2136: Plane, other_1808: MotorDual) -> Motor {
    var self_2137: Plane;
    var other_1809: MotorDual;

    self_2137 = self_2136;
    other_1809 = other_1808;
    let _e4: Plane = self_2137;
    let _e5: MotorDual = other_1809;
    let _e6: MotorDual = motor_dual_inverse(_e5);
    let _e7: Motor = plane_motor_dual_geometric_product(_e4, _e6);
    return _e7;
}

fn plane_motor_dual_transformation(self_2138: Plane, other_1810: MotorDual) -> MotorDual {
    var self_2139: Plane;
    var other_1811: MotorDual;

    self_2139 = self_2138;
    other_1811 = other_1810;
    let _e4: Plane = self_2139;
    let _e5: MotorDual = other_1811;
    let _e6: Motor = plane_motor_dual_geometric_product(_e4, _e5);
    let _e7: Plane = self_2139;
    let _e8: Plane = plane_reversal(_e7);
    let _e9: MotorDual = motor_plane_geometric_product(_e6, _e8);
    return _e9;
}

fn plane_multi_vector_geometric_quotient(self_2140: Plane, other_1812: MultiVector) -> MultiVector {
    var self_2141: Plane;
    var other_1813: MultiVector;

    self_2141 = self_2140;
    other_1813 = other_1812;
    let _e4: Plane = self_2141;
    let _e5: MultiVector = other_1813;
    let _e6: MultiVector = multi_vector_inverse(_e5);
    let _e7: MultiVector = plane_multi_vector_geometric_product(_e4, _e6);
    return _e7;
}

fn plane_multi_vector_transformation(self_2142: Plane, other_1814: MultiVector) -> MultiVector {
    var self_2143: Plane;
    var other_1815: MultiVector;

    self_2143 = self_2142;
    other_1815 = other_1814;
    let _e4: Plane = self_2143;
    let _e5: MultiVector = other_1815;
    let _e6: MultiVector = plane_multi_vector_geometric_product(_e4, _e5);
    let _e7: Plane = self_2143;
    let _e8: Plane = plane_reversal(_e7);
    let _e9: MultiVector = multi_vector_plane_geometric_product(_e6, _e8);
    return _e9;
}

fn plane_plane_geometric_quotient(self_2144: Plane, other_1816: Plane) -> Motor {
    var self_2145: Plane;
    var other_1817: Plane;

    self_2145 = self_2144;
    other_1817 = other_1816;
    let _e4: Plane = self_2145;
    let _e5: Plane = other_1817;
    let _e6: Plane = plane_inverse(_e5);
    let _e7: Motor = plane_plane_geometric_product(_e4, _e6);
    return _e7;
}

fn plane_plane_transformation(self_2146: Plane, other_1818: Plane) -> Plane {
    var self_2147: Plane;
    var other_1819: Plane;

    self_2147 = self_2146;
    other_1819 = other_1818;
    let _e4: Plane = self_2147;
    let _e5: Plane = other_1819;
    let _e6: Motor = plane_plane_geometric_product(_e4, _e5);
    let _e7: Plane = self_2147;
    let _e8: Plane = plane_reversal(_e7);
    let _e9: MotorDual = motor_plane_geometric_product(_e6, _e8);
    let _e10: Plane = motor_dual_plane_into(_e9);
    return _e10;
}

fn plane_point_geometric_quotient(self_2148: Plane, other_1820: Point) -> MotorDual {
    var self_2149: Plane;
    var other_1821: Point;

    self_2149 = self_2148;
    other_1821 = other_1820;
    let _e4: Plane = self_2149;
    let _e5: Point = other_1821;
    let _e6: Point = point_inverse(_e5);
    let _e7: MotorDual = plane_point_geometric_product(_e4, _e6);
    return _e7;
}

fn plane_point_transformation(self_2150: Plane, other_1822: Point) -> Point {
    var self_2151: Plane;
    var other_1823: Point;

    self_2151 = self_2150;
    other_1823 = other_1822;
    let _e4: Plane = self_2151;
    let _e5: Point = other_1823;
    let _e6: MotorDual = plane_point_geometric_product(_e4, _e5);
    let _e7: Plane = self_2151;
    let _e8: Plane = plane_reversal(_e7);
    let _e9: Motor = motor_dual_plane_geometric_product(_e6, _e8);
    let _e10: Point = motor_point_into(_e9);
    return _e10;
}

fn plane_rotor_geometric_quotient(self_2152: Plane, other_1824: Rotor) -> MotorDual {
    var self_2153: Plane;
    var other_1825: Rotor;

    self_2153 = self_2152;
    other_1825 = other_1824;
    let _e4: Plane = self_2153;
    let _e5: Rotor = other_1825;
    let _e6: Rotor = rotor_inverse(_e5);
    let _e7: MotorDual = plane_rotor_geometric_product(_e4, _e6);
    return _e7;
}

fn plane_rotor_transformation(self_2154: Plane, other_1826: Rotor) -> Rotor {
    var self_2155: Plane;
    var other_1827: Rotor;

    self_2155 = self_2154;
    other_1827 = other_1826;
    let _e4: Plane = self_2155;
    let _e5: Rotor = other_1827;
    let _e6: MotorDual = plane_rotor_geometric_product(_e4, _e5);
    let _e7: Plane = self_2155;
    let _e8: Plane = plane_reversal(_e7);
    let _e9: Motor = motor_dual_plane_geometric_product(_e6, _e8);
    let _e10: Rotor = motor_rotor_into(_e9);
    return _e10;
}

fn plane_scalar_geometric_quotient(self_2156: Plane, other_1828: Scalar) -> Plane {
    var self_2157: Plane;
    var other_1829: Scalar;

    self_2157 = self_2156;
    other_1829 = other_1828;
    let _e4: Plane = self_2157;
    let _e5: Scalar = other_1829;
    let _e6: Scalar = scalar_inverse(_e5);
    let _e7: Plane = plane_scalar_geometric_product(_e4, _e6);
    return _e7;
}

fn plane_scalar_transformation(self_2158: Plane, other_1830: Scalar) -> Scalar {
    var self_2159: Plane;
    var other_1831: Scalar;

    self_2159 = self_2158;
    other_1831 = other_1830;
    let _e4: Plane = self_2159;
    let _e5: Scalar = other_1831;
    let _e6: Plane = plane_scalar_geometric_product(_e4, _e5);
    let _e7: Plane = self_2159;
    let _e8: Plane = plane_reversal(_e7);
    let _e9: Motor = plane_plane_geometric_product(_e6, _e8);
    let _e10: Scalar = motor_scalar_into(_e9);
    return _e10;
}

fn plane_translator_geometric_quotient(self_2160: Plane, other_1832: Translator) -> MotorDual {
    var self_2161: Plane;
    var other_1833: Translator;

    self_2161 = self_2160;
    other_1833 = other_1832;
    let _e4: Plane = self_2161;
    let _e5: Translator = other_1833;
    let _e6: Translator = translator_inverse(_e5);
    let _e7: MotorDual = plane_translator_geometric_product(_e4, _e6);
    return _e7;
}

fn plane_translator_transformation(self_2162: Plane, other_1834: Translator) -> Translator {
    var self_2163: Plane;
    var other_1835: Translator;

    self_2163 = self_2162;
    other_1835 = other_1834;
    let _e4: Plane = self_2163;
    let _e5: Translator = other_1835;
    let _e6: MotorDual = plane_translator_geometric_product(_e4, _e5);
    let _e7: Plane = self_2163;
    let _e8: Plane = plane_reversal(_e7);
    let _e9: Motor = motor_dual_plane_geometric_product(_e6, _e8);
    let _e10: Translator = motor_translator_into(_e9);
    return _e10;
}

fn point_ideal_point_transformation(self_2164: Point, other_1836: IdealPoint) -> IdealPoint {
    var self_2165: Point;
    var other_1837: IdealPoint;

    self_2165 = self_2164;
    other_1837 = other_1836;
    let _e4: Point = self_2165;
    let _e5: IdealPoint = other_1837;
    let _e6: IdealPoint = point_ideal_point_geometric_product(_e4, _e5);
    let _e7: Point = self_2165;
    let _e8: Point = point_reversal(_e7);
    let _e9: IdealPoint = ideal_point_point_geometric_product(_e6, _e8);
    return _e9;
}

fn point_motor_geometric_quotient(self_2166: Point, other_1838: Motor) -> Motor {
    var self_2167: Point;
    var other_1839: Motor;

    self_2167 = self_2166;
    other_1839 = other_1838;
    let _e4: Point = self_2167;
    let _e5: Motor = other_1839;
    let _e6: Motor = motor_inverse(_e5);
    let _e7: Motor = point_motor_geometric_product(_e4, _e6);
    return _e7;
}

fn point_motor_transformation(self_2168: Point, other_1840: Motor) -> Motor {
    var self_2169: Point;
    var other_1841: Motor;

    self_2169 = self_2168;
    other_1841 = other_1840;
    let _e4: Point = self_2169;
    let _e5: Motor = other_1841;
    let _e6: Motor = point_motor_geometric_product(_e4, _e5);
    let _e7: Point = self_2169;
    let _e8: Point = point_reversal(_e7);
    let _e9: Motor = motor_point_geometric_product(_e6, _e8);
    return _e9;
}

fn point_motor_dual_geometric_quotient(self_2170: Point, other_1842: MotorDual) -> MotorDual {
    var self_2171: Point;
    var other_1843: MotorDual;

    self_2171 = self_2170;
    other_1843 = other_1842;
    let _e4: Point = self_2171;
    let _e5: MotorDual = other_1843;
    let _e6: MotorDual = motor_dual_inverse(_e5);
    let _e7: MotorDual = point_motor_dual_geometric_product(_e4, _e6);
    return _e7;
}

fn point_motor_dual_transformation(self_2172: Point, other_1844: MotorDual) -> MotorDual {
    var self_2173: Point;
    var other_1845: MotorDual;

    self_2173 = self_2172;
    other_1845 = other_1844;
    let _e4: Point = self_2173;
    let _e5: MotorDual = other_1845;
    let _e6: MotorDual = point_motor_dual_geometric_product(_e4, _e5);
    let _e7: Point = self_2173;
    let _e8: Point = point_reversal(_e7);
    let _e9: MotorDual = motor_dual_point_geometric_product(_e6, _e8);
    return _e9;
}

fn point_multi_vector_geometric_quotient(self_2174: Point, other_1846: MultiVector) -> MultiVector {
    var self_2175: Point;
    var other_1847: MultiVector;

    self_2175 = self_2174;
    other_1847 = other_1846;
    let _e4: Point = self_2175;
    let _e5: MultiVector = other_1847;
    let _e6: MultiVector = multi_vector_inverse(_e5);
    let _e7: MultiVector = point_multi_vector_geometric_product(_e4, _e6);
    return _e7;
}

fn point_multi_vector_transformation(self_2176: Point, other_1848: MultiVector) -> MultiVector {
    var self_2177: Point;
    var other_1849: MultiVector;

    self_2177 = self_2176;
    other_1849 = other_1848;
    let _e4: Point = self_2177;
    let _e5: MultiVector = other_1849;
    let _e6: MultiVector = point_multi_vector_geometric_product(_e4, _e5);
    let _e7: Point = self_2177;
    let _e8: Point = point_reversal(_e7);
    let _e9: MultiVector = multi_vector_point_geometric_product(_e6, _e8);
    return _e9;
}

fn point_plane_geometric_quotient(self_2178: Point, other_1850: Plane) -> MotorDual {
    var self_2179: Point;
    var other_1851: Plane;

    self_2179 = self_2178;
    other_1851 = other_1850;
    let _e4: Point = self_2179;
    let _e5: Plane = other_1851;
    let _e6: Plane = plane_inverse(_e5);
    let _e7: MotorDual = point_plane_geometric_product(_e4, _e6);
    return _e7;
}

fn point_plane_transformation(self_2180: Point, other_1852: Plane) -> Plane {
    var self_2181: Point;
    var other_1853: Plane;

    self_2181 = self_2180;
    other_1853 = other_1852;
    let _e4: Point = self_2181;
    let _e5: Plane = other_1853;
    let _e6: MotorDual = point_plane_geometric_product(_e4, _e5);
    let _e7: Point = self_2181;
    let _e8: Point = point_reversal(_e7);
    let _e9: MotorDual = motor_dual_point_geometric_product(_e6, _e8);
    let _e10: Plane = motor_dual_plane_into(_e9);
    return _e10;
}

fn point_point_geometric_quotient(self_2182: Point, other_1854: Point) -> Translator {
    var self_2183: Point;
    var other_1855: Point;

    self_2183 = self_2182;
    other_1855 = other_1854;
    let _e4: Point = self_2183;
    let _e5: Point = other_1855;
    let _e6: Point = point_inverse(_e5);
    let _e7: Translator = point_point_geometric_product(_e4, _e6);
    return _e7;
}

fn point_point_transformation(self_2184: Point, other_1856: Point) -> Point {
    var self_2185: Point;
    var other_1857: Point;

    self_2185 = self_2184;
    other_1857 = other_1856;
    let _e4: Point = self_2185;
    let _e5: Point = other_1857;
    let _e6: Translator = point_point_geometric_product(_e4, _e5);
    let _e7: Point = self_2185;
    let _e8: Point = point_reversal(_e7);
    let _e9: Point = translator_point_geometric_product(_e6, _e8);
    return _e9;
}

fn point_rotor_geometric_quotient(self_2186: Point, other_1858: Rotor) -> Motor {
    var self_2187: Point;
    var other_1859: Rotor;

    self_2187 = self_2186;
    other_1859 = other_1858;
    let _e4: Point = self_2187;
    let _e5: Rotor = other_1859;
    let _e6: Rotor = rotor_inverse(_e5);
    let _e7: Motor = point_rotor_geometric_product(_e4, _e6);
    return _e7;
}

fn point_rotor_transformation(self_2188: Point, other_1860: Rotor) -> Rotor {
    var self_2189: Point;
    var other_1861: Rotor;

    self_2189 = self_2188;
    other_1861 = other_1860;
    let _e4: Point = self_2189;
    let _e5: Rotor = other_1861;
    let _e6: Motor = point_rotor_geometric_product(_e4, _e5);
    let _e7: Point = self_2189;
    let _e8: Point = point_reversal(_e7);
    let _e9: Motor = motor_point_geometric_product(_e6, _e8);
    let _e10: Rotor = motor_rotor_into(_e9);
    return _e10;
}

fn point_scalar_geometric_quotient(self_2190: Point, other_1862: Scalar) -> Point {
    var self_2191: Point;
    var other_1863: Scalar;

    self_2191 = self_2190;
    other_1863 = other_1862;
    let _e4: Point = self_2191;
    let _e5: Scalar = other_1863;
    let _e6: Scalar = scalar_inverse(_e5);
    let _e7: Point = point_scalar_geometric_product(_e4, _e6);
    return _e7;
}

fn point_scalar_transformation(self_2192: Point, other_1864: Scalar) -> Scalar {
    var self_2193: Point;
    var other_1865: Scalar;

    self_2193 = self_2192;
    other_1865 = other_1864;
    let _e4: Point = self_2193;
    let _e5: Scalar = other_1865;
    let _e6: Point = point_scalar_geometric_product(_e4, _e5);
    let _e7: Point = self_2193;
    let _e8: Point = point_reversal(_e7);
    let _e9: Translator = point_point_geometric_product(_e6, _e8);
    let _e10: Scalar = translator_scalar_into(_e9);
    return _e10;
}

fn point_translator_geometric_quotient(self_2194: Point, other_1866: Translator) -> Point {
    var self_2195: Point;
    var other_1867: Translator;

    self_2195 = self_2194;
    other_1867 = other_1866;
    let _e4: Point = self_2195;
    let _e5: Translator = other_1867;
    let _e6: Translator = translator_inverse(_e5);
    let _e7: Point = point_translator_geometric_product(_e4, _e6);
    return _e7;
}

fn point_translator_transformation(self_2196: Point, other_1868: Translator) -> Translator {
    var self_2197: Point;
    var other_1869: Translator;

    self_2197 = self_2196;
    other_1869 = other_1868;
    let _e4: Point = self_2197;
    let _e5: Translator = other_1869;
    let _e6: Point = point_translator_geometric_product(_e4, _e5);
    let _e7: Point = self_2197;
    let _e8: Point = point_reversal(_e7);
    let _e9: Translator = point_point_geometric_product(_e6, _e8);
    return _e9;
}

fn rotor_ideal_point_transformation(self_2198: Rotor, other_1870: IdealPoint) -> IdealPoint {
    var self_2199: Rotor;
    var other_1871: IdealPoint;

    self_2199 = self_2198;
    other_1871 = other_1870;
    let _e4: Rotor = self_2199;
    let _e5: IdealPoint = other_1871;
    let _e6: IdealPoint = rotor_ideal_point_geometric_product(_e4, _e5);
    let _e7: Rotor = self_2199;
    let _e8: Rotor = rotor_reversal(_e7);
    let _e9: IdealPoint = ideal_point_rotor_geometric_product(_e6, _e8);
    return _e9;
}

fn rotor_motor_geometric_quotient(self_2200: Rotor, other_1872: Motor) -> Motor {
    var self_2201: Rotor;
    var other_1873: Motor;

    self_2201 = self_2200;
    other_1873 = other_1872;
    let _e4: Rotor = self_2201;
    let _e5: Motor = other_1873;
    let _e6: Motor = motor_inverse(_e5);
    let _e7: Motor = rotor_motor_geometric_product(_e4, _e6);
    return _e7;
}

fn rotor_motor_transformation(self_2202: Rotor, other_1874: Motor) -> Motor {
    var self_2203: Rotor;
    var other_1875: Motor;

    self_2203 = self_2202;
    other_1875 = other_1874;
    let _e4: Rotor = self_2203;
    let _e5: Motor = other_1875;
    let _e6: Motor = rotor_motor_geometric_product(_e4, _e5);
    let _e7: Rotor = self_2203;
    let _e8: Rotor = rotor_reversal(_e7);
    let _e9: Motor = motor_rotor_geometric_product(_e6, _e8);
    return _e9;
}

fn rotor_motor_dual_geometric_quotient(self_2204: Rotor, other_1876: MotorDual) -> MotorDual {
    var self_2205: Rotor;
    var other_1877: MotorDual;

    self_2205 = self_2204;
    other_1877 = other_1876;
    let _e4: Rotor = self_2205;
    let _e5: MotorDual = other_1877;
    let _e6: MotorDual = motor_dual_inverse(_e5);
    let _e7: MotorDual = rotor_motor_dual_geometric_product(_e4, _e6);
    return _e7;
}

fn rotor_motor_dual_transformation(self_2206: Rotor, other_1878: MotorDual) -> MotorDual {
    var self_2207: Rotor;
    var other_1879: MotorDual;

    self_2207 = self_2206;
    other_1879 = other_1878;
    let _e4: Rotor = self_2207;
    let _e5: MotorDual = other_1879;
    let _e6: MotorDual = rotor_motor_dual_geometric_product(_e4, _e5);
    let _e7: Rotor = self_2207;
    let _e8: Rotor = rotor_reversal(_e7);
    let _e9: MotorDual = motor_dual_rotor_geometric_product(_e6, _e8);
    return _e9;
}

fn rotor_multi_vector_geometric_quotient(self_2208: Rotor, other_1880: MultiVector) -> MultiVector {
    var self_2209: Rotor;
    var other_1881: MultiVector;

    self_2209 = self_2208;
    other_1881 = other_1880;
    let _e4: Rotor = self_2209;
    let _e5: MultiVector = other_1881;
    let _e6: MultiVector = multi_vector_inverse(_e5);
    let _e7: MultiVector = rotor_multi_vector_geometric_product(_e4, _e6);
    return _e7;
}

fn rotor_multi_vector_transformation(self_2210: Rotor, other_1882: MultiVector) -> MultiVector {
    var self_2211: Rotor;
    var other_1883: MultiVector;

    self_2211 = self_2210;
    other_1883 = other_1882;
    let _e4: Rotor = self_2211;
    let _e5: MultiVector = other_1883;
    let _e6: MultiVector = rotor_multi_vector_geometric_product(_e4, _e5);
    let _e7: Rotor = self_2211;
    let _e8: Rotor = rotor_reversal(_e7);
    let _e9: MultiVector = multi_vector_rotor_geometric_product(_e6, _e8);
    return _e9;
}

fn rotor_plane_geometric_quotient(self_2212: Rotor, other_1884: Plane) -> MotorDual {
    var self_2213: Rotor;
    var other_1885: Plane;

    self_2213 = self_2212;
    other_1885 = other_1884;
    let _e4: Rotor = self_2213;
    let _e5: Plane = other_1885;
    let _e6: Plane = plane_inverse(_e5);
    let _e7: MotorDual = rotor_plane_geometric_product(_e4, _e6);
    return _e7;
}

fn rotor_plane_transformation(self_2214: Rotor, other_1886: Plane) -> Plane {
    var self_2215: Rotor;
    var other_1887: Plane;

    self_2215 = self_2214;
    other_1887 = other_1886;
    let _e4: Rotor = self_2215;
    let _e5: Plane = other_1887;
    let _e6: MotorDual = rotor_plane_geometric_product(_e4, _e5);
    let _e7: Rotor = self_2215;
    let _e8: Rotor = rotor_reversal(_e7);
    let _e9: MotorDual = motor_dual_rotor_geometric_product(_e6, _e8);
    let _e10: Plane = motor_dual_plane_into(_e9);
    return _e10;
}

fn rotor_point_geometric_quotient(self_2216: Rotor, other_1888: Point) -> Motor {
    var self_2217: Rotor;
    var other_1889: Point;

    self_2217 = self_2216;
    other_1889 = other_1888;
    let _e4: Rotor = self_2217;
    let _e5: Point = other_1889;
    let _e6: Point = point_inverse(_e5);
    let _e7: Motor = rotor_point_geometric_product(_e4, _e6);
    return _e7;
}

fn rotor_point_transformation(self_2218: Rotor, other_1890: Point) -> Point {
    var self_2219: Rotor;
    var other_1891: Point;

    self_2219 = self_2218;
    other_1891 = other_1890;
    let _e4: Rotor = self_2219;
    let _e5: Point = other_1891;
    let _e6: Motor = rotor_point_geometric_product(_e4, _e5);
    let _e7: Rotor = self_2219;
    let _e8: Rotor = rotor_reversal(_e7);
    let _e9: Motor = motor_rotor_geometric_product(_e6, _e8);
    let _e10: Point = motor_point_into(_e9);
    return _e10;
}

fn rotor_powi(self_2220: Rotor, exponent_4: i32) -> Rotor {
    var self_2221: Rotor;
    var exponent_5: i32;
    var local_2: Rotor;
    var x_2: Rotor;
    var y_2: Rotor;
    var n_2: i32;

    self_2221 = self_2220;
    exponent_5 = exponent_4;
    let _e4: i32 = exponent_5;
    if (_e4 == 0) {
        {
            let _e7: Rotor = rotor_one();
            return _e7;
        }
    }
    let _e8: i32 = exponent_5;
    if (_e8 < 0) {
        let _e11: Rotor = self_2221;
        let _e12: Rotor = rotor_inverse(_e11);
        local_2 = _e12;
    } else {
        let _e14: Rotor = self_2221;
        local_2 = _e14;
    }
    let _e15: Rotor = local_2;
    x_2 = _e15;
    let _e17: Rotor = rotor_one();
    y_2 = _e17;
    let _e19: i32 = exponent_5;
    n_2 = abs(_e19);
    loop {
        let _e23: i32 = n_2;
        if !((1 < _e23)) {
            break;
        }
        {
            let _e26: i32 = n_2;
            if ((_e26 & 1) == 1) {
                {
                    let _e31: Rotor = x_2;
                    let _e32: Rotor = y_2;
                    let _e33: Rotor = rotor_rotor_geometric_product(_e31, _e32);
                    y_2 = _e33;
                }
            }
            let _e34: Rotor = x_2;
            let _e35: Rotor = x_2;
            let _e36: Rotor = rotor_rotor_geometric_product(_e34, _e35);
            x_2 = _e36;
            let _e37: i32 = n_2;
            n_2 = (_e37 >> u32(1));
        }
    }
    let _e41: Rotor = x_2;
    let _e42: Rotor = y_2;
    let _e43: Rotor = rotor_rotor_geometric_product(_e41, _e42);
    return _e43;
}

fn rotor_rotor_geometric_quotient(self_2222: Rotor, other_1892: Rotor) -> Rotor {
    var self_2223: Rotor;
    var other_1893: Rotor;

    self_2223 = self_2222;
    other_1893 = other_1892;
    let _e4: Rotor = self_2223;
    let _e5: Rotor = other_1893;
    let _e6: Rotor = rotor_inverse(_e5);
    let _e7: Rotor = rotor_rotor_geometric_product(_e4, _e6);
    return _e7;
}

fn rotor_rotor_transformation(self_2224: Rotor, other_1894: Rotor) -> Rotor {
    var self_2225: Rotor;
    var other_1895: Rotor;

    self_2225 = self_2224;
    other_1895 = other_1894;
    let _e4: Rotor = self_2225;
    let _e5: Rotor = other_1895;
    let _e6: Rotor = rotor_rotor_geometric_product(_e4, _e5);
    let _e7: Rotor = self_2225;
    let _e8: Rotor = rotor_reversal(_e7);
    let _e9: Rotor = rotor_rotor_geometric_product(_e6, _e8);
    return _e9;
}

fn rotor_scalar_geometric_quotient(self_2226: Rotor, other_1896: Scalar) -> Rotor {
    var self_2227: Rotor;
    var other_1897: Scalar;

    self_2227 = self_2226;
    other_1897 = other_1896;
    let _e4: Rotor = self_2227;
    let _e5: Scalar = other_1897;
    let _e6: Scalar = scalar_inverse(_e5);
    let _e7: Rotor = rotor_scalar_geometric_product(_e4, _e6);
    return _e7;
}

fn rotor_scalar_transformation(self_2228: Rotor, other_1898: Scalar) -> Scalar {
    var self_2229: Rotor;
    var other_1899: Scalar;

    self_2229 = self_2228;
    other_1899 = other_1898;
    let _e4: Rotor = self_2229;
    let _e5: Scalar = other_1899;
    let _e6: Rotor = rotor_scalar_geometric_product(_e4, _e5);
    let _e7: Rotor = self_2229;
    let _e8: Rotor = rotor_reversal(_e7);
    let _e9: Rotor = rotor_rotor_geometric_product(_e6, _e8);
    let _e10: Scalar = rotor_scalar_into(_e9);
    return _e10;
}

fn rotor_translator_geometric_quotient(self_2230: Rotor, other_1900: Translator) -> Motor {
    var self_2231: Rotor;
    var other_1901: Translator;

    self_2231 = self_2230;
    other_1901 = other_1900;
    let _e4: Rotor = self_2231;
    let _e5: Translator = other_1901;
    let _e6: Translator = translator_inverse(_e5);
    let _e7: Motor = rotor_translator_geometric_product(_e4, _e6);
    return _e7;
}

fn rotor_translator_transformation(self_2232: Rotor, other_1902: Translator) -> Translator {
    var self_2233: Rotor;
    var other_1903: Translator;

    self_2233 = self_2232;
    other_1903 = other_1902;
    let _e4: Rotor = self_2233;
    let _e5: Translator = other_1903;
    let _e6: Motor = rotor_translator_geometric_product(_e4, _e5);
    let _e7: Rotor = self_2233;
    let _e8: Rotor = rotor_reversal(_e7);
    let _e9: Motor = motor_rotor_geometric_product(_e6, _e8);
    let _e10: Translator = motor_translator_into(_e9);
    return _e10;
}

fn scalar_anti_scalar_transformation(self_2234: Scalar, other_1904: AntiScalar) -> AntiScalar {
    var self_2235: Scalar;
    var other_1905: AntiScalar;

    self_2235 = self_2234;
    other_1905 = other_1904;
    let _e4: Scalar = self_2235;
    let _e5: AntiScalar = other_1905;
    let _e6: AntiScalar = scalar_anti_scalar_geometric_product(_e4, _e5);
    let _e7: Scalar = self_2235;
    let _e8: Scalar = scalar_reversal(_e7);
    let _e9: AntiScalar = anti_scalar_scalar_geometric_product(_e6, _e8);
    return _e9;
}

fn scalar_ideal_point_transformation(self_2236: Scalar, other_1906: IdealPoint) -> IdealPoint {
    var self_2237: Scalar;
    var other_1907: IdealPoint;

    self_2237 = self_2236;
    other_1907 = other_1906;
    let _e4: Scalar = self_2237;
    let _e5: IdealPoint = other_1907;
    let _e6: IdealPoint = scalar_ideal_point_geometric_product(_e4, _e5);
    let _e7: Scalar = self_2237;
    let _e8: Scalar = scalar_reversal(_e7);
    let _e9: IdealPoint = ideal_point_scalar_geometric_product(_e6, _e8);
    return _e9;
}

fn scalar_motor_geometric_quotient(self_2238: Scalar, other_1908: Motor) -> Motor {
    var self_2239: Scalar;
    var other_1909: Motor;

    self_2239 = self_2238;
    other_1909 = other_1908;
    let _e4: Scalar = self_2239;
    let _e5: Motor = other_1909;
    let _e6: Motor = motor_inverse(_e5);
    let _e7: Motor = scalar_motor_geometric_product(_e4, _e6);
    return _e7;
}

fn scalar_motor_transformation(self_2240: Scalar, other_1910: Motor) -> Motor {
    var self_2241: Scalar;
    var other_1911: Motor;

    self_2241 = self_2240;
    other_1911 = other_1910;
    let _e4: Scalar = self_2241;
    let _e5: Motor = other_1911;
    let _e6: Motor = scalar_motor_geometric_product(_e4, _e5);
    let _e7: Scalar = self_2241;
    let _e8: Scalar = scalar_reversal(_e7);
    let _e9: Motor = motor_scalar_geometric_product(_e6, _e8);
    return _e9;
}

fn scalar_motor_dual_geometric_quotient(self_2242: Scalar, other_1912: MotorDual) -> MotorDual {
    var self_2243: Scalar;
    var other_1913: MotorDual;

    self_2243 = self_2242;
    other_1913 = other_1912;
    let _e4: Scalar = self_2243;
    let _e5: MotorDual = other_1913;
    let _e6: MotorDual = motor_dual_inverse(_e5);
    let _e7: MotorDual = scalar_motor_dual_geometric_product(_e4, _e6);
    return _e7;
}

fn scalar_motor_dual_transformation(self_2244: Scalar, other_1914: MotorDual) -> MotorDual {
    var self_2245: Scalar;
    var other_1915: MotorDual;

    self_2245 = self_2244;
    other_1915 = other_1914;
    let _e4: Scalar = self_2245;
    let _e5: MotorDual = other_1915;
    let _e6: MotorDual = scalar_motor_dual_geometric_product(_e4, _e5);
    let _e7: Scalar = self_2245;
    let _e8: Scalar = scalar_reversal(_e7);
    let _e9: MotorDual = motor_dual_scalar_geometric_product(_e6, _e8);
    return _e9;
}

fn scalar_multi_vector_geometric_quotient(self_2246: Scalar, other_1916: MultiVector) -> MultiVector {
    var self_2247: Scalar;
    var other_1917: MultiVector;

    self_2247 = self_2246;
    other_1917 = other_1916;
    let _e4: Scalar = self_2247;
    let _e5: MultiVector = other_1917;
    let _e6: MultiVector = multi_vector_inverse(_e5);
    let _e7: MultiVector = scalar_multi_vector_geometric_product(_e4, _e6);
    return _e7;
}

fn scalar_multi_vector_transformation(self_2248: Scalar, other_1918: MultiVector) -> MultiVector {
    var self_2249: Scalar;
    var other_1919: MultiVector;

    self_2249 = self_2248;
    other_1919 = other_1918;
    let _e4: Scalar = self_2249;
    let _e5: MultiVector = other_1919;
    let _e6: MultiVector = scalar_multi_vector_geometric_product(_e4, _e5);
    let _e7: Scalar = self_2249;
    let _e8: Scalar = scalar_reversal(_e7);
    let _e9: MultiVector = multi_vector_scalar_geometric_product(_e6, _e8);
    return _e9;
}

fn scalar_plane_geometric_quotient(self_2250: Scalar, other_1920: Plane) -> Plane {
    var self_2251: Scalar;
    var other_1921: Plane;

    self_2251 = self_2250;
    other_1921 = other_1920;
    let _e4: Scalar = self_2251;
    let _e5: Plane = other_1921;
    let _e6: Plane = plane_inverse(_e5);
    let _e7: Plane = scalar_plane_geometric_product(_e4, _e6);
    return _e7;
}

fn scalar_plane_transformation(self_2252: Scalar, other_1922: Plane) -> Plane {
    var self_2253: Scalar;
    var other_1923: Plane;

    self_2253 = self_2252;
    other_1923 = other_1922;
    let _e4: Scalar = self_2253;
    let _e5: Plane = other_1923;
    let _e6: Plane = scalar_plane_geometric_product(_e4, _e5);
    let _e7: Scalar = self_2253;
    let _e8: Scalar = scalar_reversal(_e7);
    let _e9: Plane = plane_scalar_geometric_product(_e6, _e8);
    return _e9;
}

fn scalar_point_geometric_quotient(self_2254: Scalar, other_1924: Point) -> Point {
    var self_2255: Scalar;
    var other_1925: Point;

    self_2255 = self_2254;
    other_1925 = other_1924;
    let _e4: Scalar = self_2255;
    let _e5: Point = other_1925;
    let _e6: Point = point_inverse(_e5);
    let _e7: Point = scalar_point_geometric_product(_e4, _e6);
    return _e7;
}

fn scalar_point_transformation(self_2256: Scalar, other_1926: Point) -> Point {
    var self_2257: Scalar;
    var other_1927: Point;

    self_2257 = self_2256;
    other_1927 = other_1926;
    let _e4: Scalar = self_2257;
    let _e5: Point = other_1927;
    let _e6: Point = scalar_point_geometric_product(_e4, _e5);
    let _e7: Scalar = self_2257;
    let _e8: Scalar = scalar_reversal(_e7);
    let _e9: Point = point_scalar_geometric_product(_e6, _e8);
    return _e9;
}

fn scalar_rotor_geometric_quotient(self_2258: Scalar, other_1928: Rotor) -> Rotor {
    var self_2259: Scalar;
    var other_1929: Rotor;

    self_2259 = self_2258;
    other_1929 = other_1928;
    let _e4: Scalar = self_2259;
    let _e5: Rotor = other_1929;
    let _e6: Rotor = rotor_inverse(_e5);
    let _e7: Rotor = scalar_rotor_geometric_product(_e4, _e6);
    return _e7;
}

fn scalar_rotor_transformation(self_2260: Scalar, other_1930: Rotor) -> Rotor {
    var self_2261: Scalar;
    var other_1931: Rotor;

    self_2261 = self_2260;
    other_1931 = other_1930;
    let _e4: Scalar = self_2261;
    let _e5: Rotor = other_1931;
    let _e6: Rotor = scalar_rotor_geometric_product(_e4, _e5);
    let _e7: Scalar = self_2261;
    let _e8: Scalar = scalar_reversal(_e7);
    let _e9: Rotor = rotor_scalar_geometric_product(_e6, _e8);
    return _e9;
}

fn scalar_powi(self_2262: Scalar, exponent_6: i32) -> Scalar {
    var self_2263: Scalar;
    var exponent_7: i32;
    var local_3: Scalar;
    var x_3: Scalar;
    var y_3: Scalar;
    var n_3: i32;

    self_2263 = self_2262;
    exponent_7 = exponent_6;
    let _e4: i32 = exponent_7;
    if (_e4 == 0) {
        {
            let _e7: Scalar = scalar_one();
            return _e7;
        }
    }
    let _e8: i32 = exponent_7;
    if (_e8 < 0) {
        let _e11: Scalar = self_2263;
        let _e12: Scalar = scalar_inverse(_e11);
        local_3 = _e12;
    } else {
        let _e14: Scalar = self_2263;
        local_3 = _e14;
    }
    let _e15: Scalar = local_3;
    x_3 = _e15;
    let _e17: Scalar = scalar_one();
    y_3 = _e17;
    let _e19: i32 = exponent_7;
    n_3 = abs(_e19);
    loop {
        let _e23: i32 = n_3;
        if !((1 < _e23)) {
            break;
        }
        {
            let _e26: i32 = n_3;
            if ((_e26 & 1) == 1) {
                {
                    let _e31: Scalar = x_3;
                    let _e32: Scalar = y_3;
                    let _e33: Scalar = scalar_scalar_geometric_product(_e31, _e32);
                    y_3 = _e33;
                }
            }
            let _e34: Scalar = x_3;
            let _e35: Scalar = x_3;
            let _e36: Scalar = scalar_scalar_geometric_product(_e34, _e35);
            x_3 = _e36;
            let _e37: i32 = n_3;
            n_3 = (_e37 >> u32(1));
        }
    }
    let _e41: Scalar = x_3;
    let _e42: Scalar = y_3;
    let _e43: Scalar = scalar_scalar_geometric_product(_e41, _e42);
    return _e43;
}

fn scalar_scalar_geometric_quotient(self_2264: Scalar, other_1932: Scalar) -> Scalar {
    var self_2265: Scalar;
    var other_1933: Scalar;

    self_2265 = self_2264;
    other_1933 = other_1932;
    let _e4: Scalar = self_2265;
    let _e5: Scalar = other_1933;
    let _e6: Scalar = scalar_inverse(_e5);
    let _e7: Scalar = scalar_scalar_geometric_product(_e4, _e6);
    return _e7;
}

fn scalar_scalar_transformation(self_2266: Scalar, other_1934: Scalar) -> Scalar {
    var self_2267: Scalar;
    var other_1935: Scalar;

    self_2267 = self_2266;
    other_1935 = other_1934;
    let _e4: Scalar = self_2267;
    let _e5: Scalar = other_1935;
    let _e6: Scalar = scalar_scalar_geometric_product(_e4, _e5);
    let _e7: Scalar = self_2267;
    let _e8: Scalar = scalar_reversal(_e7);
    let _e9: Scalar = scalar_scalar_geometric_product(_e6, _e8);
    return _e9;
}

fn scalar_translator_geometric_quotient(self_2268: Scalar, other_1936: Translator) -> Translator {
    var self_2269: Scalar;
    var other_1937: Translator;

    self_2269 = self_2268;
    other_1937 = other_1936;
    let _e4: Scalar = self_2269;
    let _e5: Translator = other_1937;
    let _e6: Translator = translator_inverse(_e5);
    let _e7: Translator = scalar_translator_geometric_product(_e4, _e6);
    return _e7;
}

fn scalar_translator_transformation(self_2270: Scalar, other_1938: Translator) -> Translator {
    var self_2271: Scalar;
    var other_1939: Translator;

    self_2271 = self_2270;
    other_1939 = other_1938;
    let _e4: Scalar = self_2271;
    let _e5: Translator = other_1939;
    let _e6: Translator = scalar_translator_geometric_product(_e4, _e5);
    let _e7: Scalar = self_2271;
    let _e8: Scalar = scalar_reversal(_e7);
    let _e9: Translator = translator_scalar_geometric_product(_e6, _e8);
    return _e9;
}

fn translator_anti_scalar_transformation(self_2272: Translator, other_1940: AntiScalar) -> AntiScalar {
    var self_2273: Translator;
    var other_1941: AntiScalar;

    self_2273 = self_2272;
    other_1941 = other_1940;
    let _e4: Translator = self_2273;
    let _e5: AntiScalar = other_1941;
    let _e6: AntiScalar = translator_anti_scalar_geometric_product(_e4, _e5);
    let _e7: Translator = self_2273;
    let _e8: Translator = translator_reversal(_e7);
    let _e9: AntiScalar = anti_scalar_translator_geometric_product(_e6, _e8);
    return _e9;
}

fn translator_ideal_point_transformation(self_2274: Translator, other_1942: IdealPoint) -> IdealPoint {
    var self_2275: Translator;
    var other_1943: IdealPoint;

    self_2275 = self_2274;
    other_1943 = other_1942;
    let _e4: Translator = self_2275;
    let _e5: IdealPoint = other_1943;
    let _e6: IdealPoint = translator_ideal_point_geometric_product(_e4, _e5);
    let _e7: Translator = self_2275;
    let _e8: Translator = translator_reversal(_e7);
    let _e9: IdealPoint = ideal_point_translator_geometric_product(_e6, _e8);
    return _e9;
}

fn translator_motor_geometric_quotient(self_2276: Translator, other_1944: Motor) -> Motor {
    var self_2277: Translator;
    var other_1945: Motor;

    self_2277 = self_2276;
    other_1945 = other_1944;
    let _e4: Translator = self_2277;
    let _e5: Motor = other_1945;
    let _e6: Motor = motor_inverse(_e5);
    let _e7: Motor = translator_motor_geometric_product(_e4, _e6);
    return _e7;
}

fn translator_motor_transformation(self_2278: Translator, other_1946: Motor) -> Motor {
    var self_2279: Translator;
    var other_1947: Motor;

    self_2279 = self_2278;
    other_1947 = other_1946;
    let _e4: Translator = self_2279;
    let _e5: Motor = other_1947;
    let _e6: Motor = translator_motor_geometric_product(_e4, _e5);
    let _e7: Translator = self_2279;
    let _e8: Translator = translator_reversal(_e7);
    let _e9: Motor = motor_translator_geometric_product(_e6, _e8);
    return _e9;
}

fn translator_motor_dual_geometric_quotient(self_2280: Translator, other_1948: MotorDual) -> MotorDual {
    var self_2281: Translator;
    var other_1949: MotorDual;

    self_2281 = self_2280;
    other_1949 = other_1948;
    let _e4: Translator = self_2281;
    let _e5: MotorDual = other_1949;
    let _e6: MotorDual = motor_dual_inverse(_e5);
    let _e7: MotorDual = translator_motor_dual_geometric_product(_e4, _e6);
    return _e7;
}

fn translator_motor_dual_transformation(self_2282: Translator, other_1950: MotorDual) -> MotorDual {
    var self_2283: Translator;
    var other_1951: MotorDual;

    self_2283 = self_2282;
    other_1951 = other_1950;
    let _e4: Translator = self_2283;
    let _e5: MotorDual = other_1951;
    let _e6: MotorDual = translator_motor_dual_geometric_product(_e4, _e5);
    let _e7: Translator = self_2283;
    let _e8: Translator = translator_reversal(_e7);
    let _e9: MotorDual = motor_dual_translator_geometric_product(_e6, _e8);
    return _e9;
}

fn translator_multi_vector_geometric_quotient(self_2284: Translator, other_1952: MultiVector) -> MultiVector {
    var self_2285: Translator;
    var other_1953: MultiVector;

    self_2285 = self_2284;
    other_1953 = other_1952;
    let _e4: Translator = self_2285;
    let _e5: MultiVector = other_1953;
    let _e6: MultiVector = multi_vector_inverse(_e5);
    let _e7: MultiVector = translator_multi_vector_geometric_product(_e4, _e6);
    return _e7;
}

fn translator_multi_vector_transformation(self_2286: Translator, other_1954: MultiVector) -> MultiVector {
    var self_2287: Translator;
    var other_1955: MultiVector;

    self_2287 = self_2286;
    other_1955 = other_1954;
    let _e4: Translator = self_2287;
    let _e5: MultiVector = other_1955;
    let _e6: MultiVector = translator_multi_vector_geometric_product(_e4, _e5);
    let _e7: Translator = self_2287;
    let _e8: Translator = translator_reversal(_e7);
    let _e9: MultiVector = multi_vector_translator_geometric_product(_e6, _e8);
    return _e9;
}

fn translator_plane_geometric_quotient(self_2288: Translator, other_1956: Plane) -> MotorDual {
    var self_2289: Translator;
    var other_1957: Plane;

    self_2289 = self_2288;
    other_1957 = other_1956;
    let _e4: Translator = self_2289;
    let _e5: Plane = other_1957;
    let _e6: Plane = plane_inverse(_e5);
    let _e7: MotorDual = translator_plane_geometric_product(_e4, _e6);
    return _e7;
}

fn translator_plane_transformation(self_2290: Translator, other_1958: Plane) -> Plane {
    var self_2291: Translator;
    var other_1959: Plane;

    self_2291 = self_2290;
    other_1959 = other_1958;
    let _e4: Translator = self_2291;
    let _e5: Plane = other_1959;
    let _e6: MotorDual = translator_plane_geometric_product(_e4, _e5);
    let _e7: Translator = self_2291;
    let _e8: Translator = translator_reversal(_e7);
    let _e9: MotorDual = motor_dual_translator_geometric_product(_e6, _e8);
    let _e10: Plane = motor_dual_plane_into(_e9);
    return _e10;
}

fn translator_point_geometric_quotient(self_2292: Translator, other_1960: Point) -> Point {
    var self_2293: Translator;
    var other_1961: Point;

    self_2293 = self_2292;
    other_1961 = other_1960;
    let _e4: Translator = self_2293;
    let _e5: Point = other_1961;
    let _e6: Point = point_inverse(_e5);
    let _e7: Point = translator_point_geometric_product(_e4, _e6);
    return _e7;
}

fn translator_point_transformation(self_2294: Translator, other_1962: Point) -> Point {
    var self_2295: Translator;
    var other_1963: Point;

    self_2295 = self_2294;
    other_1963 = other_1962;
    let _e4: Translator = self_2295;
    let _e5: Point = other_1963;
    let _e6: Point = translator_point_geometric_product(_e4, _e5);
    let _e7: Translator = self_2295;
    let _e8: Translator = translator_reversal(_e7);
    let _e9: Point = point_translator_geometric_product(_e6, _e8);
    return _e9;
}

fn translator_rotor_geometric_quotient(self_2296: Translator, other_1964: Rotor) -> Motor {
    var self_2297: Translator;
    var other_1965: Rotor;

    self_2297 = self_2296;
    other_1965 = other_1964;
    let _e4: Translator = self_2297;
    let _e5: Rotor = other_1965;
    let _e6: Rotor = rotor_inverse(_e5);
    let _e7: Motor = translator_rotor_geometric_product(_e4, _e6);
    return _e7;
}

fn translator_rotor_transformation(self_2298: Translator, other_1966: Rotor) -> Rotor {
    var self_2299: Translator;
    var other_1967: Rotor;

    self_2299 = self_2298;
    other_1967 = other_1966;
    let _e4: Translator = self_2299;
    let _e5: Rotor = other_1967;
    let _e6: Motor = translator_rotor_geometric_product(_e4, _e5);
    let _e7: Translator = self_2299;
    let _e8: Translator = translator_reversal(_e7);
    let _e9: Motor = motor_translator_geometric_product(_e6, _e8);
    let _e10: Rotor = motor_rotor_into(_e9);
    return _e10;
}

fn translator_scalar_geometric_quotient(self_2300: Translator, other_1968: Scalar) -> Translator {
    var self_2301: Translator;
    var other_1969: Scalar;

    self_2301 = self_2300;
    other_1969 = other_1968;
    let _e4: Translator = self_2301;
    let _e5: Scalar = other_1969;
    let _e6: Scalar = scalar_inverse(_e5);
    let _e7: Translator = translator_scalar_geometric_product(_e4, _e6);
    return _e7;
}

fn translator_scalar_transformation(self_2302: Translator, other_1970: Scalar) -> Scalar {
    var self_2303: Translator;
    var other_1971: Scalar;

    self_2303 = self_2302;
    other_1971 = other_1970;
    let _e4: Translator = self_2303;
    let _e5: Scalar = other_1971;
    let _e6: Translator = translator_scalar_geometric_product(_e4, _e5);
    let _e7: Translator = self_2303;
    let _e8: Translator = translator_reversal(_e7);
    let _e9: Translator = translator_translator_geometric_product(_e6, _e8);
    let _e10: Scalar = translator_scalar_into(_e9);
    return _e10;
}

fn translator_powi(self_2304: Translator, exponent_8: i32) -> Translator {
    var self_2305: Translator;
    var exponent_9: i32;
    var local_4: Translator;
    var x_4: Translator;
    var y_4: Translator;
    var n_4: i32;

    self_2305 = self_2304;
    exponent_9 = exponent_8;
    let _e4: i32 = exponent_9;
    if (_e4 == 0) {
        {
            let _e7: Translator = translator_one();
            return _e7;
        }
    }
    let _e8: i32 = exponent_9;
    if (_e8 < 0) {
        let _e11: Translator = self_2305;
        let _e12: Translator = translator_inverse(_e11);
        local_4 = _e12;
    } else {
        let _e14: Translator = self_2305;
        local_4 = _e14;
    }
    let _e15: Translator = local_4;
    x_4 = _e15;
    let _e17: Translator = translator_one();
    y_4 = _e17;
    let _e19: i32 = exponent_9;
    n_4 = abs(_e19);
    loop {
        let _e23: i32 = n_4;
        if !((1 < _e23)) {
            break;
        }
        {
            let _e26: i32 = n_4;
            if ((_e26 & 1) == 1) {
                {
                    let _e31: Translator = x_4;
                    let _e32: Translator = y_4;
                    let _e33: Translator = translator_translator_geometric_product(_e31, _e32);
                    y_4 = _e33;
                }
            }
            let _e34: Translator = x_4;
            let _e35: Translator = x_4;
            let _e36: Translator = translator_translator_geometric_product(_e34, _e35);
            x_4 = _e36;
            let _e37: i32 = n_4;
            n_4 = (_e37 >> u32(1));
        }
    }
    let _e41: Translator = x_4;
    let _e42: Translator = y_4;
    let _e43: Translator = translator_translator_geometric_product(_e41, _e42);
    return _e43;
}

fn translator_translator_geometric_quotient(self_2306: Translator, other_1972: Translator) -> Translator {
    var self_2307: Translator;
    var other_1973: Translator;

    self_2307 = self_2306;
    other_1973 = other_1972;
    let _e4: Translator = self_2307;
    let _e5: Translator = other_1973;
    let _e6: Translator = translator_inverse(_e5);
    let _e7: Translator = translator_translator_geometric_product(_e4, _e6);
    return _e7;
}

fn translator_translator_transformation(self_2308: Translator, other_1974: Translator) -> Translator {
    var self_2309: Translator;
    var other_1975: Translator;

    self_2309 = self_2308;
    other_1975 = other_1974;
    let _e4: Translator = self_2309;
    let _e5: Translator = other_1975;
    let _e6: Translator = translator_translator_geometric_product(_e4, _e5);
    let _e7: Translator = self_2309;
    let _e8: Translator = translator_reversal(_e7);
    let _e9: Translator = translator_translator_geometric_product(_e6, _e8);
    return _e9;
}


struct Scalar {
    g0_: f32,
}

struct AntiScalar {
    g0_: f32,
}

struct MultiVector {
    g0_: vec4<f32>,
    g1_: vec4<f32>,
    g2_: vec4<f32>,
    g3_: vec4<f32>,
}

struct Rotor {
    g0_: vec4<f32>,
}

struct Point {
    g0_: vec4<f32>,
}

struct IdealPoint {
    g0_: vec3<f32>,
}

struct Translator {
    g0_: vec4<f32>,
}

struct Plane {
    g0_: vec4<f32>,
}

struct Line {
    g0_: vec3<f32>,
    g1_: vec3<f32>,
}

struct Motor {
    g0_: vec4<f32>,
    g1_: vec4<f32>,
}

struct PointAndPlane {
    g0_: vec4<f32>,
    g1_: vec4<f32>,
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
    let _e18: MultiVector = other_37;
    let _e20: MultiVector = other_37;
    return MultiVector(((vec4<f32>(_e4.g0_) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) + _e13.g0_), _e16.g1_, _e18.g2_, _e20.g3_);
}

fn scalar_multi_vector_sub(self_58: Scalar, other_38: MultiVector) -> MultiVector {
    var self_59: Scalar;
    var other_39: MultiVector;

    self_59 = self_58;
    other_39 = other_38;
    let _e4: Scalar = self_59;
    let _e13: MultiVector = other_39;
    let _e18: MultiVector = other_39;
    let _e23: MultiVector = other_39;
    let _e28: MultiVector = other_39;
    return MultiVector(((vec4<f32>(_e4.g0_) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) - _e13.g0_), (vec4<f32>(0.0) - _e18.g1_), (vec4<f32>(0.0) - _e23.g2_), (vec4<f32>(0.0) - _e28.g3_));
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
    let _e16: Scalar = self_61;
    let _e19: MultiVector = other_41;
    let _e22: Scalar = self_61;
    let _e25: MultiVector = other_41;
    return MultiVector((vec4<f32>(_e4.g0_) * _e7.g0_), (vec4<f32>(_e10.g0_) * _e13.g1_), (vec4<f32>(_e16.g0_) * _e19.g2_), (vec4<f32>(_e22.g0_) * _e25.g3_));
}

fn scalar_multi_vector_regressive_product(self_62: Scalar, other_42: MultiVector) -> Scalar {
    var self_63: Scalar;
    var other_43: MultiVector;

    self_63 = self_62;
    other_43 = other_42;
    let _e4: Scalar = self_63;
    let _e6: MultiVector = other_43;
    return Scalar((_e4.g0_ * _e6.g3_.x));
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
    let _e16: Scalar = self_65;
    let _e19: MultiVector = other_45;
    let _e22: Scalar = self_65;
    let _e25: MultiVector = other_45;
    return MultiVector((vec4<f32>(_e4.g0_) * _e7.g0_), (vec4<f32>(_e10.g0_) * _e13.g1_), (vec4<f32>(_e16.g0_) * _e19.g2_), (vec4<f32>(_e22.g0_) * _e25.g3_));
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
    let _e16: Scalar = self_67;
    let _e19: MultiVector = other_47;
    let _e22: Scalar = self_67;
    let _e25: MultiVector = other_47;
    return MultiVector((vec4<f32>(_e4.g0_) * _e7.g0_), (vec4<f32>(_e10.g0_) * _e13.g1_), (vec4<f32>(_e16.g0_) * _e19.g2_), (vec4<f32>(_e22.g0_) * _e25.g3_));
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
    let _e16: Scalar = self_69;
    let _e19: MultiVector = other_49;
    let _e22: Scalar = self_69;
    let _e25: MultiVector = other_49;
    return MultiVector((vec4<f32>(_e4.g0_) * _e7.g0_), (vec4<f32>(_e10.g0_) * _e13.g1_), (vec4<f32>(_e16.g0_) * _e19.g2_), (vec4<f32>(_e22.g0_) * _e25.g3_));
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
    let _e13: Rotor = other_55;
    return Rotor(((vec4<f32>(_e4.g0_) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) + _e13.g0_));
}

fn scalar_rotor_sub(self_76: Scalar, other_56: Rotor) -> Rotor {
    var self_77: Scalar;
    var other_57: Rotor;

    self_77 = self_76;
    other_57 = other_56;
    let _e4: Scalar = self_77;
    let _e13: Rotor = other_57;
    return Rotor(((vec4<f32>(_e4.g0_) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) - _e13.g0_));
}

fn scalar_rotor_geometric_product(self_78: Scalar, other_58: Rotor) -> Rotor {
    var self_79: Scalar;
    var other_59: Rotor;

    self_79 = self_78;
    other_59 = other_58;
    let _e4: Scalar = self_79;
    let _e7: Rotor = other_59;
    return Rotor((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_rotor_outer_product(self_80: Scalar, other_60: Rotor) -> Rotor {
    var self_81: Scalar;
    var other_61: Rotor;

    self_81 = self_80;
    other_61 = other_60;
    let _e4: Scalar = self_81;
    let _e7: Rotor = other_61;
    return Rotor((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_rotor_inner_product(self_82: Scalar, other_62: Rotor) -> Rotor {
    var self_83: Scalar;
    var other_63: Rotor;

    self_83 = self_82;
    other_63 = other_62;
    let _e4: Scalar = self_83;
    let _e7: Rotor = other_63;
    return Rotor((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_rotor_left_contraction(self_84: Scalar, other_64: Rotor) -> Rotor {
    var self_85: Scalar;
    var other_65: Rotor;

    self_85 = self_84;
    other_65 = other_64;
    let _e4: Scalar = self_85;
    let _e7: Rotor = other_65;
    return Rotor((vec4<f32>(_e4.g0_) * _e7.g0_));
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

fn scalar_point_geometric_product(self_90: Scalar, other_70: Point) -> Point {
    var self_91: Scalar;
    var other_71: Point;

    self_91 = self_90;
    other_71 = other_70;
    let _e4: Scalar = self_91;
    let _e7: Point = other_71;
    return Point((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_point_outer_product(self_92: Scalar, other_72: Point) -> Point {
    var self_93: Scalar;
    var other_73: Point;

    self_93 = self_92;
    other_73 = other_72;
    let _e4: Scalar = self_93;
    let _e7: Point = other_73;
    return Point((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_point_inner_product(self_94: Scalar, other_74: Point) -> Point {
    var self_95: Scalar;
    var other_75: Point;

    self_95 = self_94;
    other_75 = other_74;
    let _e4: Scalar = self_95;
    let _e7: Point = other_75;
    return Point((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_point_left_contraction(self_96: Scalar, other_76: Point) -> Point {
    var self_97: Scalar;
    var other_77: Point;

    self_97 = self_96;
    other_77 = other_76;
    let _e4: Scalar = self_97;
    let _e7: Point = other_77;
    return Point((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_ideal_point_add(self_98: Scalar, other_78: IdealPoint) -> Translator {
    var self_99: Scalar;
    var other_79: IdealPoint;

    self_99 = self_98;
    other_79 = other_78;
    let _e4: Scalar = self_99;
    let _e13: IdealPoint = other_79;
    let _e16: IdealPoint = other_79;
    let _e19: IdealPoint = other_79;
    let _e22: IdealPoint = other_79;
    return Translator(((vec4<f32>(_e4.g0_) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) + (vec4<f32>(_e13.g0_.x, _e16.g0_.x, _e19.g0_.y, _e22.g0_.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn scalar_ideal_point_sub(self_100: Scalar, other_80: IdealPoint) -> Translator {
    var self_101: Scalar;
    var other_81: IdealPoint;

    self_101 = self_100;
    other_81 = other_80;
    let _e4: Scalar = self_101;
    let _e13: IdealPoint = other_81;
    let _e16: IdealPoint = other_81;
    let _e19: IdealPoint = other_81;
    let _e22: IdealPoint = other_81;
    return Translator(((vec4<f32>(_e4.g0_) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) - (vec4<f32>(_e13.g0_.x, _e16.g0_.x, _e19.g0_.y, _e22.g0_.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn scalar_ideal_point_geometric_product(self_102: Scalar, other_82: IdealPoint) -> IdealPoint {
    var self_103: Scalar;
    var other_83: IdealPoint;

    self_103 = self_102;
    other_83 = other_82;
    let _e4: Scalar = self_103;
    let _e7: IdealPoint = other_83;
    return IdealPoint((vec3<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_ideal_point_outer_product(self_104: Scalar, other_84: IdealPoint) -> IdealPoint {
    var self_105: Scalar;
    var other_85: IdealPoint;

    self_105 = self_104;
    other_85 = other_84;
    let _e4: Scalar = self_105;
    let _e7: IdealPoint = other_85;
    return IdealPoint((vec3<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_ideal_point_inner_product(self_106: Scalar, other_86: IdealPoint) -> IdealPoint {
    var self_107: Scalar;
    var other_87: IdealPoint;

    self_107 = self_106;
    other_87 = other_86;
    let _e4: Scalar = self_107;
    let _e7: IdealPoint = other_87;
    return IdealPoint((vec3<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_ideal_point_left_contraction(self_108: Scalar, other_88: IdealPoint) -> IdealPoint {
    var self_109: Scalar;
    var other_89: IdealPoint;

    self_109 = self_108;
    other_89 = other_88;
    let _e4: Scalar = self_109;
    let _e7: IdealPoint = other_89;
    return IdealPoint((vec3<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_plane_geometric_product(self_110: Scalar, other_90: Plane) -> Plane {
    var self_111: Scalar;
    var other_91: Plane;

    self_111 = self_110;
    other_91 = other_90;
    let _e4: Scalar = self_111;
    let _e7: Plane = other_91;
    return Plane((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_plane_outer_product(self_112: Scalar, other_92: Plane) -> Plane {
    var self_113: Scalar;
    var other_93: Plane;

    self_113 = self_112;
    other_93 = other_92;
    let _e4: Scalar = self_113;
    let _e7: Plane = other_93;
    return Plane((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_plane_inner_product(self_114: Scalar, other_94: Plane) -> Plane {
    var self_115: Scalar;
    var other_95: Plane;

    self_115 = self_114;
    other_95 = other_94;
    let _e4: Scalar = self_115;
    let _e7: Plane = other_95;
    return Plane((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_plane_left_contraction(self_116: Scalar, other_96: Plane) -> Plane {
    var self_117: Scalar;
    var other_97: Plane;

    self_117 = self_116;
    other_97 = other_96;
    let _e4: Scalar = self_117;
    let _e7: Plane = other_97;
    return Plane((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_line_geometric_product(self_118: Scalar, other_98: Line) -> Line {
    var self_119: Scalar;
    var other_99: Line;

    self_119 = self_118;
    other_99 = other_98;
    let _e4: Scalar = self_119;
    let _e7: Line = other_99;
    let _e10: Scalar = self_119;
    let _e13: Line = other_99;
    return Line((vec3<f32>(_e4.g0_) * _e7.g0_), (vec3<f32>(_e10.g0_) * _e13.g1_));
}

fn scalar_line_outer_product(self_120: Scalar, other_100: Line) -> Line {
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

fn scalar_line_inner_product(self_122: Scalar, other_102: Line) -> Line {
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

fn scalar_line_left_contraction(self_124: Scalar, other_104: Line) -> Line {
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

fn scalar_translator_add(self_126: Scalar, other_106: Translator) -> Translator {
    var self_127: Scalar;
    var other_107: Translator;

    self_127 = self_126;
    other_107 = other_106;
    let _e4: Scalar = self_127;
    let _e13: Translator = other_107;
    return Translator(((vec4<f32>(_e4.g0_) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) + _e13.g0_));
}

fn scalar_translator_sub(self_128: Scalar, other_108: Translator) -> Translator {
    var self_129: Scalar;
    var other_109: Translator;

    self_129 = self_128;
    other_109 = other_108;
    let _e4: Scalar = self_129;
    let _e13: Translator = other_109;
    return Translator(((vec4<f32>(_e4.g0_) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) - _e13.g0_));
}

fn scalar_translator_geometric_product(self_130: Scalar, other_110: Translator) -> Translator {
    var self_131: Scalar;
    var other_111: Translator;

    self_131 = self_130;
    other_111 = other_110;
    let _e4: Scalar = self_131;
    let _e7: Translator = other_111;
    return Translator((vec4<f32>(_e4.g0_) * _e7.g0_));
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

fn scalar_translator_left_contraction(self_136: Scalar, other_116: Translator) -> Translator {
    var self_137: Scalar;
    var other_117: Translator;

    self_137 = self_136;
    other_117 = other_116;
    let _e4: Scalar = self_137;
    let _e7: Translator = other_117;
    return Translator((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_translator_right_contraction(self_138: Scalar, other_118: Translator) -> Scalar {
    var self_139: Scalar;
    var other_119: Translator;

    self_139 = self_138;
    other_119 = other_118;
    let _e4: Scalar = self_139;
    let _e6: Translator = other_119;
    return Scalar((_e4.g0_ * _e6.g0_.x));
}

fn scalar_translator_scalar_product(self_140: Scalar, other_120: Translator) -> Scalar {
    var self_141: Scalar;
    var other_121: Translator;

    self_141 = self_140;
    other_121 = other_120;
    let _e4: Scalar = self_141;
    let _e6: Translator = other_121;
    return Scalar((_e4.g0_ * _e6.g0_.x));
}

fn scalar_motor_add(self_142: Scalar, other_122: Motor) -> Motor {
    var self_143: Scalar;
    var other_123: Motor;

    self_143 = self_142;
    other_123 = other_122;
    let _e4: Scalar = self_143;
    let _e13: Motor = other_123;
    let _e16: Motor = other_123;
    return Motor(((vec4<f32>(_e4.g0_) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) + _e13.g0_), _e16.g1_);
}

fn scalar_motor_sub(self_144: Scalar, other_124: Motor) -> Motor {
    var self_145: Scalar;
    var other_125: Motor;

    self_145 = self_144;
    other_125 = other_124;
    let _e4: Scalar = self_145;
    let _e13: Motor = other_125;
    let _e18: Motor = other_125;
    return Motor(((vec4<f32>(_e4.g0_) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) - _e13.g0_), (vec4<f32>(0.0) - _e18.g1_));
}

fn scalar_motor_geometric_product(self_146: Scalar, other_126: Motor) -> Motor {
    var self_147: Scalar;
    var other_127: Motor;

    self_147 = self_146;
    other_127 = other_126;
    let _e4: Scalar = self_147;
    let _e7: Motor = other_127;
    let _e10: Scalar = self_147;
    let _e13: Motor = other_127;
    return Motor((vec4<f32>(_e4.g0_) * _e7.g0_), (vec4<f32>(_e10.g0_) * _e13.g1_));
}

fn scalar_motor_regressive_product(self_148: Scalar, other_128: Motor) -> Scalar {
    var self_149: Scalar;
    var other_129: Motor;

    self_149 = self_148;
    other_129 = other_128;
    let _e4: Scalar = self_149;
    let _e6: Motor = other_129;
    return Scalar((_e4.g0_ * _e6.g1_.x));
}

fn scalar_motor_outer_product(self_150: Scalar, other_130: Motor) -> Motor {
    var self_151: Scalar;
    var other_131: Motor;

    self_151 = self_150;
    other_131 = other_130;
    let _e4: Scalar = self_151;
    let _e7: Motor = other_131;
    let _e10: Scalar = self_151;
    let _e13: Motor = other_131;
    return Motor((vec4<f32>(_e4.g0_) * _e7.g0_), (vec4<f32>(_e10.g0_) * _e13.g1_));
}

fn scalar_motor_inner_product(self_152: Scalar, other_132: Motor) -> Motor {
    var self_153: Scalar;
    var other_133: Motor;

    self_153 = self_152;
    other_133 = other_132;
    let _e4: Scalar = self_153;
    let _e7: Motor = other_133;
    let _e10: Scalar = self_153;
    let _e13: Motor = other_133;
    return Motor((vec4<f32>(_e4.g0_) * _e7.g0_), (vec4<f32>(_e10.g0_) * _e13.g1_));
}

fn scalar_motor_geometric_anti_product(self_154: Scalar, other_134: Motor) -> Rotor {
    var self_155: Scalar;
    var other_135: Motor;

    self_155 = self_154;
    other_135 = other_134;
    let _e4: Scalar = self_155;
    let _e7: Motor = other_135;
    return Rotor(((vec4<f32>(_e4.g0_) * _e7.g1_) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))));
}

fn scalar_motor_inner_anti_product(self_156: Scalar, other_136: Motor) -> Rotor {
    var self_157: Scalar;
    var other_137: Motor;

    self_157 = self_156;
    other_137 = other_136;
    let _e4: Scalar = self_157;
    let _e7: Motor = other_137;
    return Rotor(((vec4<f32>(_e4.g0_) * _e7.g1_) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))));
}

fn scalar_motor_left_contraction(self_158: Scalar, other_138: Motor) -> Motor {
    var self_159: Scalar;
    var other_139: Motor;

    self_159 = self_158;
    other_139 = other_138;
    let _e4: Scalar = self_159;
    let _e7: Motor = other_139;
    let _e10: Scalar = self_159;
    let _e13: Motor = other_139;
    return Motor((vec4<f32>(_e4.g0_) * _e7.g0_), (vec4<f32>(_e10.g0_) * _e13.g1_));
}

fn scalar_motor_right_contraction(self_160: Scalar, other_140: Motor) -> Scalar {
    var self_161: Scalar;
    var other_141: Motor;

    self_161 = self_160;
    other_141 = other_140;
    let _e4: Scalar = self_161;
    let _e6: Motor = other_141;
    return Scalar((_e4.g0_ * _e6.g0_.x));
}

fn scalar_motor_right_anti_contraction(self_162: Scalar, other_142: Motor) -> Rotor {
    var self_163: Scalar;
    var other_143: Motor;

    self_163 = self_162;
    other_143 = other_142;
    let _e4: Scalar = self_163;
    let _e7: Motor = other_143;
    return Rotor(((vec4<f32>(_e4.g0_) * _e7.g1_) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))));
}

fn scalar_motor_scalar_product(self_164: Scalar, other_144: Motor) -> Scalar {
    var self_165: Scalar;
    var other_145: Motor;

    self_165 = self_164;
    other_145 = other_144;
    let _e4: Scalar = self_165;
    let _e6: Motor = other_145;
    return Scalar((_e4.g0_ * _e6.g0_.x));
}

fn scalar_point_and_plane_geometric_product(self_166: Scalar, other_146: PointAndPlane) -> PointAndPlane {
    var self_167: Scalar;
    var other_147: PointAndPlane;

    self_167 = self_166;
    other_147 = other_146;
    let _e4: Scalar = self_167;
    let _e7: PointAndPlane = other_147;
    let _e10: Scalar = self_167;
    let _e13: PointAndPlane = other_147;
    return PointAndPlane((vec4<f32>(_e4.g0_) * _e7.g0_), (vec4<f32>(_e10.g0_) * _e13.g1_));
}

fn scalar_point_and_plane_outer_product(self_168: Scalar, other_148: PointAndPlane) -> PointAndPlane {
    var self_169: Scalar;
    var other_149: PointAndPlane;

    self_169 = self_168;
    other_149 = other_148;
    let _e4: Scalar = self_169;
    let _e7: PointAndPlane = other_149;
    let _e10: Scalar = self_169;
    let _e13: PointAndPlane = other_149;
    return PointAndPlane((vec4<f32>(_e4.g0_) * _e7.g0_), (vec4<f32>(_e10.g0_) * _e13.g1_));
}

fn scalar_point_and_plane_inner_product(self_170: Scalar, other_150: PointAndPlane) -> PointAndPlane {
    var self_171: Scalar;
    var other_151: PointAndPlane;

    self_171 = self_170;
    other_151 = other_150;
    let _e4: Scalar = self_171;
    let _e7: PointAndPlane = other_151;
    let _e10: Scalar = self_171;
    let _e13: PointAndPlane = other_151;
    return PointAndPlane((vec4<f32>(_e4.g0_) * _e7.g0_), (vec4<f32>(_e10.g0_) * _e13.g1_));
}

fn scalar_point_and_plane_left_contraction(self_172: Scalar, other_152: PointAndPlane) -> PointAndPlane {
    var self_173: Scalar;
    var other_153: PointAndPlane;

    self_173 = self_172;
    other_153 = other_152;
    let _e4: Scalar = self_173;
    let _e7: PointAndPlane = other_153;
    let _e10: Scalar = self_173;
    let _e13: PointAndPlane = other_153;
    return PointAndPlane((vec4<f32>(_e4.g0_) * _e7.g0_), (vec4<f32>(_e10.g0_) * _e13.g1_));
}

fn scalar_squared_magnitude(self_174: Scalar) -> Scalar {
    var self_175: Scalar;

    self_175 = self_174;
    let _e2: Scalar = self_175;
    let _e3: Scalar = self_175;
    let _e4: Scalar = scalar_reversal(_e3);
    let _e5: Scalar = scalar_scalar_scalar_product(_e2, _e4);
    return _e5;
}

fn scalar_magnitude(self_176: Scalar) -> Scalar {
    var self_177: Scalar;

    self_177 = self_176;
    let _e2: Scalar = self_177;
    let _e3: Scalar = scalar_squared_magnitude(_e2);
    return Scalar(sqrt(_e3.g0_));
}

fn scalar_scale(self_178: Scalar, other_154: f32) -> Scalar {
    var self_179: Scalar;
    var other_155: f32;

    self_179 = self_178;
    other_155 = other_154;
    let _e4: Scalar = self_179;
    let _e5: f32 = other_155;
    let _e7: Scalar = scalar_scalar_geometric_product(_e4, Scalar(_e5));
    return _e7;
}

fn scalar_signum(self_180: Scalar) -> Scalar {
    var self_181: Scalar;

    self_181 = self_180;
    let _e2: Scalar = self_181;
    let _e3: Scalar = self_181;
    let _e4: Scalar = scalar_magnitude(_e3);
    let _e9: Scalar = scalar_scalar_geometric_product(_e2, Scalar((1.0 / _e4.g0_)));
    return _e9;
}

fn scalar_inverse(self_182: Scalar) -> Scalar {
    var self_183: Scalar;

    self_183 = self_182;
    let _e2: Scalar = self_183;
    let _e3: Scalar = scalar_reversal(_e2);
    let _e4: Scalar = self_183;
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

fn anti_scalar_grade(self_184: AntiScalar) -> i32 {
    return 4;
}

fn anti_scalar_anti_grade(self_185: AntiScalar) -> i32 {
    return 0;
}

fn anti_scalar_neg(self_186: AntiScalar) -> AntiScalar {
    var self_187: AntiScalar;

    self_187 = self_186;
    let _e2: AntiScalar = self_187;
    return AntiScalar((_e2.g0_ * -(1.0)));
}

fn anti_scalar_automorphism(self_188: AntiScalar) -> AntiScalar {
    var self_189: AntiScalar;

    self_189 = self_188;
    let _e2: AntiScalar = self_189;
    return AntiScalar(_e2.g0_);
}

fn anti_scalar_reversal(self_190: AntiScalar) -> AntiScalar {
    var self_191: AntiScalar;

    self_191 = self_190;
    let _e2: AntiScalar = self_191;
    return AntiScalar(_e2.g0_);
}

fn anti_scalar_conjugation(self_192: AntiScalar) -> AntiScalar {
    var self_193: AntiScalar;

    self_193 = self_192;
    let _e2: AntiScalar = self_193;
    return AntiScalar(_e2.g0_);
}

fn anti_scalar_dual(self_194: AntiScalar) -> Scalar {
    var self_195: AntiScalar;

    self_195 = self_194;
    let _e2: AntiScalar = self_195;
    return Scalar(_e2.g0_);
}

fn anti_scalar_anti_reversal(self_196: AntiScalar) -> AntiScalar {
    var self_197: AntiScalar;

    self_197 = self_196;
    let _e2: AntiScalar = self_197;
    return AntiScalar(_e2.g0_);
}

fn anti_scalar_right_complement(self_198: AntiScalar) -> Scalar {
    var self_199: AntiScalar;

    self_199 = self_198;
    let _e2: AntiScalar = self_199;
    return Scalar(_e2.g0_);
}

fn anti_scalar_left_complement(self_200: AntiScalar) -> Scalar {
    var self_201: AntiScalar;

    self_201 = self_200;
    let _e2: AntiScalar = self_201;
    return Scalar(_e2.g0_);
}

fn anti_scalar_double_complement(self_202: AntiScalar) -> AntiScalar {
    var self_203: AntiScalar;

    self_203 = self_202;
    let _e2: AntiScalar = self_203;
    return AntiScalar(_e2.g0_);
}

fn anti_scalar_scalar_geometric_product(self_204: AntiScalar, other_156: Scalar) -> AntiScalar {
    var self_205: AntiScalar;
    var other_157: Scalar;

    self_205 = self_204;
    other_157 = other_156;
    let _e4: AntiScalar = self_205;
    let _e6: Scalar = other_157;
    return AntiScalar((_e4.g0_ * _e6.g0_));
}

fn anti_scalar_scalar_regressive_product(self_206: AntiScalar, other_158: Scalar) -> Scalar {
    var self_207: AntiScalar;
    var other_159: Scalar;

    self_207 = self_206;
    other_159 = other_158;
    let _e4: AntiScalar = self_207;
    let _e6: Scalar = other_159;
    return Scalar((_e4.g0_ * _e6.g0_));
}

fn anti_scalar_scalar_outer_product(self_208: AntiScalar, other_160: Scalar) -> AntiScalar {
    var self_209: AntiScalar;
    var other_161: Scalar;

    self_209 = self_208;
    other_161 = other_160;
    let _e4: AntiScalar = self_209;
    let _e6: Scalar = other_161;
    return AntiScalar((_e4.g0_ * _e6.g0_));
}

fn anti_scalar_scalar_inner_product(self_210: AntiScalar, other_162: Scalar) -> AntiScalar {
    var self_211: AntiScalar;
    var other_163: Scalar;

    self_211 = self_210;
    other_163 = other_162;
    let _e4: AntiScalar = self_211;
    let _e6: Scalar = other_163;
    return AntiScalar((_e4.g0_ * _e6.g0_));
}

fn anti_scalar_scalar_geometric_anti_product(self_212: AntiScalar, other_164: Scalar) -> Scalar {
    var self_213: AntiScalar;
    var other_165: Scalar;

    self_213 = self_212;
    other_165 = other_164;
    let _e4: AntiScalar = self_213;
    let _e6: Scalar = other_165;
    return Scalar((_e4.g0_ * _e6.g0_));
}

fn anti_scalar_scalar_inner_anti_product(self_214: AntiScalar, other_166: Scalar) -> Scalar {
    var self_215: AntiScalar;
    var other_167: Scalar;

    self_215 = self_214;
    other_167 = other_166;
    let _e4: AntiScalar = self_215;
    let _e6: Scalar = other_167;
    return Scalar((_e4.g0_ * _e6.g0_));
}

fn anti_scalar_scalar_right_contraction(self_216: AntiScalar, other_168: Scalar) -> AntiScalar {
    var self_217: AntiScalar;
    var other_169: Scalar;

    self_217 = self_216;
    other_169 = other_168;
    let _e4: AntiScalar = self_217;
    let _e6: Scalar = other_169;
    return AntiScalar((_e4.g0_ * _e6.g0_));
}

fn anti_scalar_scalar_left_anti_contraction(self_218: AntiScalar, other_170: Scalar) -> Scalar {
    var self_219: AntiScalar;
    var other_171: Scalar;

    self_219 = self_218;
    other_171 = other_170;
    let _e4: AntiScalar = self_219;
    let _e6: Scalar = other_171;
    return Scalar((_e4.g0_ * _e6.g0_));
}

fn anti_scalar_anti_scalar_add(self_220: AntiScalar, other_172: AntiScalar) -> AntiScalar {
    var self_221: AntiScalar;
    var other_173: AntiScalar;

    self_221 = self_220;
    other_173 = other_172;
    let _e4: AntiScalar = self_221;
    let _e6: AntiScalar = other_173;
    return AntiScalar((_e4.g0_ + _e6.g0_));
}

fn anti_scalar_anti_scalar_sub(self_222: AntiScalar, other_174: AntiScalar) -> AntiScalar {
    var self_223: AntiScalar;
    var other_175: AntiScalar;

    self_223 = self_222;
    other_175 = other_174;
    let _e4: AntiScalar = self_223;
    let _e6: AntiScalar = other_175;
    return AntiScalar((_e4.g0_ - _e6.g0_));
}

fn anti_scalar_anti_scalar_mul(self_224: AntiScalar, other_176: AntiScalar) -> AntiScalar {
    var self_225: AntiScalar;
    var other_177: AntiScalar;

    self_225 = self_224;
    other_177 = other_176;
    let _e4: AntiScalar = self_225;
    let _e6: AntiScalar = other_177;
    return AntiScalar((_e4.g0_ * _e6.g0_));
}

fn anti_scalar_anti_scalar_div(self_226: AntiScalar, other_178: AntiScalar) -> AntiScalar {
    var self_227: AntiScalar;
    var other_179: AntiScalar;

    self_227 = self_226;
    other_179 = other_178;
    let _e4: AntiScalar = self_227;
    let _e8: AntiScalar = other_179;
    return AntiScalar((((_e4.g0_ * 1.0) / _e8.g0_) * 1.0));
}

fn anti_scalar_anti_scalar_regressive_product(self_228: AntiScalar, other_180: AntiScalar) -> AntiScalar {
    var self_229: AntiScalar;
    var other_181: AntiScalar;

    self_229 = self_228;
    other_181 = other_180;
    let _e4: AntiScalar = self_229;
    let _e6: AntiScalar = other_181;
    return AntiScalar((_e4.g0_ * _e6.g0_));
}

fn anti_scalar_anti_scalar_geometric_anti_product(self_230: AntiScalar, other_182: AntiScalar) -> AntiScalar {
    var self_231: AntiScalar;
    var other_183: AntiScalar;

    self_231 = self_230;
    other_183 = other_182;
    let _e4: AntiScalar = self_231;
    let _e6: AntiScalar = other_183;
    return AntiScalar((_e4.g0_ * _e6.g0_));
}

fn anti_scalar_anti_scalar_inner_anti_product(self_232: AntiScalar, other_184: AntiScalar) -> AntiScalar {
    var self_233: AntiScalar;
    var other_185: AntiScalar;

    self_233 = self_232;
    other_185 = other_184;
    let _e4: AntiScalar = self_233;
    let _e6: AntiScalar = other_185;
    return AntiScalar((_e4.g0_ * _e6.g0_));
}

fn anti_scalar_anti_scalar_left_anti_contraction(self_234: AntiScalar, other_186: AntiScalar) -> AntiScalar {
    var self_235: AntiScalar;
    var other_187: AntiScalar;

    self_235 = self_234;
    other_187 = other_186;
    let _e4: AntiScalar = self_235;
    let _e6: AntiScalar = other_187;
    return AntiScalar((_e4.g0_ * _e6.g0_));
}

fn anti_scalar_anti_scalar_right_anti_contraction(self_236: AntiScalar, other_188: AntiScalar) -> AntiScalar {
    var self_237: AntiScalar;
    var other_189: AntiScalar;

    self_237 = self_236;
    other_189 = other_188;
    let _e4: AntiScalar = self_237;
    let _e6: AntiScalar = other_189;
    return AntiScalar((_e4.g0_ * _e6.g0_));
}

fn anti_scalar_anti_scalar_anti_scalar_product(self_238: AntiScalar, other_190: AntiScalar) -> AntiScalar {
    var self_239: AntiScalar;
    var other_191: AntiScalar;

    self_239 = self_238;
    other_191 = other_190;
    let _e4: AntiScalar = self_239;
    let _e6: AntiScalar = other_191;
    return AntiScalar((_e4.g0_ * _e6.g0_));
}

fn anti_scalar_multi_vector_add(self_240: AntiScalar, other_192: MultiVector) -> MultiVector {
    var self_241: AntiScalar;
    var other_193: MultiVector;

    self_241 = self_240;
    other_193 = other_192;
    let _e4: MultiVector = other_193;
    let _e6: MultiVector = other_193;
    let _e8: MultiVector = other_193;
    let _e10: AntiScalar = self_241;
    let _e19: MultiVector = other_193;
    return MultiVector(_e4.g0_, _e6.g1_, _e8.g2_, ((vec4<f32>(_e10.g0_) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) + _e19.g3_));
}

fn anti_scalar_multi_vector_sub(self_242: AntiScalar, other_194: MultiVector) -> MultiVector {
    var self_243: AntiScalar;
    var other_195: MultiVector;

    self_243 = self_242;
    other_195 = other_194;
    let _e6: MultiVector = other_195;
    let _e11: MultiVector = other_195;
    let _e16: MultiVector = other_195;
    let _e19: AntiScalar = self_243;
    let _e28: MultiVector = other_195;
    return MultiVector((vec4<f32>(0.0) - _e6.g0_), (vec4<f32>(0.0) - _e11.g1_), (vec4<f32>(0.0) - _e16.g2_), ((vec4<f32>(_e19.g0_) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) - _e28.g3_));
}

fn anti_scalar_multi_vector_regressive_product(self_244: AntiScalar, other_196: MultiVector) -> MultiVector {
    var self_245: AntiScalar;
    var other_197: MultiVector;

    self_245 = self_244;
    other_197 = other_196;
    let _e4: AntiScalar = self_245;
    let _e7: MultiVector = other_197;
    let _e10: AntiScalar = self_245;
    let _e13: MultiVector = other_197;
    let _e16: AntiScalar = self_245;
    let _e19: MultiVector = other_197;
    let _e22: AntiScalar = self_245;
    let _e25: MultiVector = other_197;
    return MultiVector((vec4<f32>(_e4.g0_) * _e7.g0_), (vec4<f32>(_e10.g0_) * _e13.g1_), (vec4<f32>(_e16.g0_) * _e19.g2_), (vec4<f32>(_e22.g0_) * _e25.g3_));
}

fn anti_scalar_multi_vector_outer_product(self_246: AntiScalar, other_198: MultiVector) -> AntiScalar {
    var self_247: AntiScalar;
    var other_199: MultiVector;

    self_247 = self_246;
    other_199 = other_198;
    let _e4: AntiScalar = self_247;
    let _e6: MultiVector = other_199;
    return AntiScalar((_e4.g0_ * _e6.g0_.x));
}

fn anti_scalar_multi_vector_geometric_anti_product(self_248: AntiScalar, other_200: MultiVector) -> MultiVector {
    var self_249: AntiScalar;
    var other_201: MultiVector;

    self_249 = self_248;
    other_201 = other_200;
    let _e4: AntiScalar = self_249;
    let _e7: MultiVector = other_201;
    let _e10: AntiScalar = self_249;
    let _e13: MultiVector = other_201;
    let _e16: AntiScalar = self_249;
    let _e19: MultiVector = other_201;
    let _e22: AntiScalar = self_249;
    let _e25: MultiVector = other_201;
    return MultiVector((vec4<f32>(_e4.g0_) * _e7.g0_), (vec4<f32>(_e10.g0_) * _e13.g1_), (vec4<f32>(_e16.g0_) * _e19.g2_), (vec4<f32>(_e22.g0_) * _e25.g3_));
}

fn anti_scalar_multi_vector_inner_anti_product(self_250: AntiScalar, other_202: MultiVector) -> MultiVector {
    var self_251: AntiScalar;
    var other_203: MultiVector;

    self_251 = self_250;
    other_203 = other_202;
    let _e4: AntiScalar = self_251;
    let _e7: MultiVector = other_203;
    let _e10: AntiScalar = self_251;
    let _e13: MultiVector = other_203;
    let _e16: AntiScalar = self_251;
    let _e19: MultiVector = other_203;
    let _e22: AntiScalar = self_251;
    let _e25: MultiVector = other_203;
    return MultiVector((vec4<f32>(_e4.g0_) * _e7.g0_), (vec4<f32>(_e10.g0_) * _e13.g1_), (vec4<f32>(_e16.g0_) * _e19.g2_), (vec4<f32>(_e22.g0_) * _e25.g3_));
}

fn anti_scalar_multi_vector_left_anti_contraction(self_252: AntiScalar, other_204: MultiVector) -> MultiVector {
    var self_253: AntiScalar;
    var other_205: MultiVector;

    self_253 = self_252;
    other_205 = other_204;
    let _e4: AntiScalar = self_253;
    let _e7: MultiVector = other_205;
    let _e10: AntiScalar = self_253;
    let _e13: MultiVector = other_205;
    let _e16: AntiScalar = self_253;
    let _e19: MultiVector = other_205;
    let _e22: AntiScalar = self_253;
    let _e25: MultiVector = other_205;
    return MultiVector((vec4<f32>(_e4.g0_) * _e7.g0_), (vec4<f32>(_e10.g0_) * _e13.g1_), (vec4<f32>(_e16.g0_) * _e19.g2_), (vec4<f32>(_e22.g0_) * _e25.g3_));
}

fn anti_scalar_multi_vector_right_anti_contraction(self_254: AntiScalar, other_206: MultiVector) -> AntiScalar {
    var self_255: AntiScalar;
    var other_207: MultiVector;

    self_255 = self_254;
    other_207 = other_206;
    let _e4: AntiScalar = self_255;
    let _e6: MultiVector = other_207;
    return AntiScalar((_e4.g0_ * _e6.g3_.x));
}

fn anti_scalar_multi_vector_anti_scalar_product(self_256: AntiScalar, other_208: MultiVector) -> AntiScalar {
    var self_257: AntiScalar;
    var other_209: MultiVector;

    self_257 = self_256;
    other_209 = other_208;
    let _e4: AntiScalar = self_257;
    let _e6: MultiVector = other_209;
    return AntiScalar((_e4.g0_ * _e6.g3_.x));
}

fn anti_scalar_rotor_regressive_product(self_258: AntiScalar, other_210: Rotor) -> Rotor {
    var self_259: AntiScalar;
    var other_211: Rotor;

    self_259 = self_258;
    other_211 = other_210;
    let _e4: AntiScalar = self_259;
    let _e7: Rotor = other_211;
    return Rotor((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn anti_scalar_rotor_outer_product(self_260: AntiScalar, other_212: Rotor) -> AntiScalar {
    var self_261: AntiScalar;
    var other_213: Rotor;

    self_261 = self_260;
    other_213 = other_212;
    let _e4: AntiScalar = self_261;
    let _e6: Rotor = other_213;
    return AntiScalar((_e4.g0_ * _e6.g0_.x));
}

fn anti_scalar_rotor_geometric_anti_product(self_262: AntiScalar, other_214: Rotor) -> Rotor {
    var self_263: AntiScalar;
    var other_215: Rotor;

    self_263 = self_262;
    other_215 = other_214;
    let _e4: AntiScalar = self_263;
    let _e7: Rotor = other_215;
    return Rotor((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn anti_scalar_rotor_inner_anti_product(self_264: AntiScalar, other_216: Rotor) -> Rotor {
    var self_265: AntiScalar;
    var other_217: Rotor;

    self_265 = self_264;
    other_217 = other_216;
    let _e4: AntiScalar = self_265;
    let _e7: Rotor = other_217;
    return Rotor((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn anti_scalar_rotor_left_anti_contraction(self_266: AntiScalar, other_218: Rotor) -> Rotor {
    var self_267: AntiScalar;
    var other_219: Rotor;

    self_267 = self_266;
    other_219 = other_218;
    let _e4: AntiScalar = self_267;
    let _e7: Rotor = other_219;
    return Rotor((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn anti_scalar_point_regressive_product(self_268: AntiScalar, other_220: Point) -> Point {
    var self_269: AntiScalar;
    var other_221: Point;

    self_269 = self_268;
    other_221 = other_220;
    let _e4: AntiScalar = self_269;
    let _e7: Point = other_221;
    return Point((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn anti_scalar_point_geometric_anti_product(self_270: AntiScalar, other_222: Point) -> Point {
    var self_271: AntiScalar;
    var other_223: Point;

    self_271 = self_270;
    other_223 = other_222;
    let _e4: AntiScalar = self_271;
    let _e7: Point = other_223;
    return Point((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn anti_scalar_point_inner_anti_product(self_272: AntiScalar, other_224: Point) -> Point {
    var self_273: AntiScalar;
    var other_225: Point;

    self_273 = self_272;
    other_225 = other_224;
    let _e4: AntiScalar = self_273;
    let _e7: Point = other_225;
    return Point((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn anti_scalar_point_left_anti_contraction(self_274: AntiScalar, other_226: Point) -> Point {
    var self_275: AntiScalar;
    var other_227: Point;

    self_275 = self_274;
    other_227 = other_226;
    let _e4: AntiScalar = self_275;
    let _e7: Point = other_227;
    return Point((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn anti_scalar_ideal_point_regressive_product(self_276: AntiScalar, other_228: IdealPoint) -> IdealPoint {
    var self_277: AntiScalar;
    var other_229: IdealPoint;

    self_277 = self_276;
    other_229 = other_228;
    let _e4: AntiScalar = self_277;
    let _e7: IdealPoint = other_229;
    return IdealPoint((vec3<f32>(_e4.g0_) * _e7.g0_));
}

fn anti_scalar_ideal_point_geometric_anti_product(self_278: AntiScalar, other_230: IdealPoint) -> IdealPoint {
    var self_279: AntiScalar;
    var other_231: IdealPoint;

    self_279 = self_278;
    other_231 = other_230;
    let _e4: AntiScalar = self_279;
    let _e7: IdealPoint = other_231;
    return IdealPoint((vec3<f32>(_e4.g0_) * _e7.g0_));
}

fn anti_scalar_ideal_point_inner_anti_product(self_280: AntiScalar, other_232: IdealPoint) -> IdealPoint {
    var self_281: AntiScalar;
    var other_233: IdealPoint;

    self_281 = self_280;
    other_233 = other_232;
    let _e4: AntiScalar = self_281;
    let _e7: IdealPoint = other_233;
    return IdealPoint((vec3<f32>(_e4.g0_) * _e7.g0_));
}

fn anti_scalar_ideal_point_left_anti_contraction(self_282: AntiScalar, other_234: IdealPoint) -> IdealPoint {
    var self_283: AntiScalar;
    var other_235: IdealPoint;

    self_283 = self_282;
    other_235 = other_234;
    let _e4: AntiScalar = self_283;
    let _e7: IdealPoint = other_235;
    return IdealPoint((vec3<f32>(_e4.g0_) * _e7.g0_));
}

fn anti_scalar_plane_regressive_product(self_284: AntiScalar, other_236: Plane) -> Plane {
    var self_285: AntiScalar;
    var other_237: Plane;

    self_285 = self_284;
    other_237 = other_236;
    let _e4: AntiScalar = self_285;
    let _e7: Plane = other_237;
    return Plane((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn anti_scalar_plane_geometric_anti_product(self_286: AntiScalar, other_238: Plane) -> Plane {
    var self_287: AntiScalar;
    var other_239: Plane;

    self_287 = self_286;
    other_239 = other_238;
    let _e4: AntiScalar = self_287;
    let _e7: Plane = other_239;
    return Plane((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn anti_scalar_plane_inner_anti_product(self_288: AntiScalar, other_240: Plane) -> Plane {
    var self_289: AntiScalar;
    var other_241: Plane;

    self_289 = self_288;
    other_241 = other_240;
    let _e4: AntiScalar = self_289;
    let _e7: Plane = other_241;
    return Plane((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn anti_scalar_plane_left_anti_contraction(self_290: AntiScalar, other_242: Plane) -> Plane {
    var self_291: AntiScalar;
    var other_243: Plane;

    self_291 = self_290;
    other_243 = other_242;
    let _e4: AntiScalar = self_291;
    let _e7: Plane = other_243;
    return Plane((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn anti_scalar_line_geometric_product(self_292: AntiScalar, other_244: Line) -> IdealPoint {
    var self_293: AntiScalar;
    var other_245: Line;

    self_293 = self_292;
    other_245 = other_244;
    let _e6: AntiScalar = self_293;
    let _e9: Line = other_245;
    return IdealPoint((vec3<f32>(0.0) - (vec3<f32>(_e6.g0_) * _e9.g1_)));
}

fn anti_scalar_line_regressive_product(self_294: AntiScalar, other_246: Line) -> Line {
    var self_295: AntiScalar;
    var other_247: Line;

    self_295 = self_294;
    other_247 = other_246;
    let _e4: AntiScalar = self_295;
    let _e7: Line = other_247;
    let _e10: AntiScalar = self_295;
    let _e13: Line = other_247;
    return Line((vec3<f32>(_e4.g0_) * _e7.g0_), (vec3<f32>(_e10.g0_) * _e13.g1_));
}

fn anti_scalar_line_inner_product(self_296: AntiScalar, other_248: Line) -> IdealPoint {
    var self_297: AntiScalar;
    var other_249: Line;

    self_297 = self_296;
    other_249 = other_248;
    let _e6: AntiScalar = self_297;
    let _e9: Line = other_249;
    return IdealPoint((vec3<f32>(0.0) - (vec3<f32>(_e6.g0_) * _e9.g1_)));
}

fn anti_scalar_line_geometric_anti_product(self_298: AntiScalar, other_250: Line) -> Line {
    var self_299: AntiScalar;
    var other_251: Line;

    self_299 = self_298;
    other_251 = other_250;
    let _e4: AntiScalar = self_299;
    let _e7: Line = other_251;
    let _e10: AntiScalar = self_299;
    let _e13: Line = other_251;
    return Line((vec3<f32>(_e4.g0_) * _e7.g0_), (vec3<f32>(_e10.g0_) * _e13.g1_));
}

fn anti_scalar_line_inner_anti_product(self_300: AntiScalar, other_252: Line) -> Line {
    var self_301: AntiScalar;
    var other_253: Line;

    self_301 = self_300;
    other_253 = other_252;
    let _e4: AntiScalar = self_301;
    let _e7: Line = other_253;
    let _e10: AntiScalar = self_301;
    let _e13: Line = other_253;
    return Line((vec3<f32>(_e4.g0_) * _e7.g0_), (vec3<f32>(_e10.g0_) * _e13.g1_));
}

fn anti_scalar_line_right_contraction(self_302: AntiScalar, other_254: Line) -> IdealPoint {
    var self_303: AntiScalar;
    var other_255: Line;

    self_303 = self_302;
    other_255 = other_254;
    let _e6: AntiScalar = self_303;
    let _e9: Line = other_255;
    return IdealPoint((vec3<f32>(0.0) - (vec3<f32>(_e6.g0_) * _e9.g1_)));
}

fn anti_scalar_line_left_anti_contraction(self_304: AntiScalar, other_256: Line) -> Line {
    var self_305: AntiScalar;
    var other_257: Line;

    self_305 = self_304;
    other_257 = other_256;
    let _e4: AntiScalar = self_305;
    let _e7: Line = other_257;
    let _e10: AntiScalar = self_305;
    let _e13: Line = other_257;
    return Line((vec3<f32>(_e4.g0_) * _e7.g0_), (vec3<f32>(_e10.g0_) * _e13.g1_));
}

fn anti_scalar_translator_geometric_product(self_306: AntiScalar, other_258: Translator) -> AntiScalar {
    var self_307: AntiScalar;
    var other_259: Translator;

    self_307 = self_306;
    other_259 = other_258;
    let _e4: AntiScalar = self_307;
    let _e6: Translator = other_259;
    return AntiScalar((_e4.g0_ * _e6.g0_.x));
}

fn anti_scalar_translator_regressive_product(self_308: AntiScalar, other_260: Translator) -> Translator {
    var self_309: AntiScalar;
    var other_261: Translator;

    self_309 = self_308;
    other_261 = other_260;
    let _e4: AntiScalar = self_309;
    let _e7: Translator = other_261;
    return Translator((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn anti_scalar_translator_outer_product(self_310: AntiScalar, other_262: Translator) -> AntiScalar {
    var self_311: AntiScalar;
    var other_263: Translator;

    self_311 = self_310;
    other_263 = other_262;
    let _e4: AntiScalar = self_311;
    let _e6: Translator = other_263;
    return AntiScalar((_e4.g0_ * _e6.g0_.x));
}

fn anti_scalar_translator_inner_product(self_312: AntiScalar, other_264: Translator) -> AntiScalar {
    var self_313: AntiScalar;
    var other_265: Translator;

    self_313 = self_312;
    other_265 = other_264;
    let _e4: AntiScalar = self_313;
    let _e6: Translator = other_265;
    return AntiScalar((_e4.g0_ * _e6.g0_.x));
}

fn anti_scalar_translator_geometric_anti_product(self_314: AntiScalar, other_266: Translator) -> Translator {
    var self_315: AntiScalar;
    var other_267: Translator;

    self_315 = self_314;
    other_267 = other_266;
    let _e4: AntiScalar = self_315;
    let _e7: Translator = other_267;
    return Translator((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn anti_scalar_translator_inner_anti_product(self_316: AntiScalar, other_268: Translator) -> Translator {
    var self_317: AntiScalar;
    var other_269: Translator;

    self_317 = self_316;
    other_269 = other_268;
    let _e4: AntiScalar = self_317;
    let _e7: Translator = other_269;
    return Translator((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn anti_scalar_translator_right_contraction(self_318: AntiScalar, other_270: Translator) -> AntiScalar {
    var self_319: AntiScalar;
    var other_271: Translator;

    self_319 = self_318;
    other_271 = other_270;
    let _e4: AntiScalar = self_319;
    let _e6: Translator = other_271;
    return AntiScalar((_e4.g0_ * _e6.g0_.x));
}

fn anti_scalar_translator_left_anti_contraction(self_320: AntiScalar, other_272: Translator) -> Translator {
    var self_321: AntiScalar;
    var other_273: Translator;

    self_321 = self_320;
    other_273 = other_272;
    let _e4: AntiScalar = self_321;
    let _e7: Translator = other_273;
    return Translator((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn anti_scalar_motor_add(self_322: AntiScalar, other_274: Motor) -> Motor {
    var self_323: AntiScalar;
    var other_275: Motor;

    self_323 = self_322;
    other_275 = other_274;
    let _e4: Motor = other_275;
    let _e6: AntiScalar = self_323;
    let _e15: Motor = other_275;
    return Motor(_e4.g0_, ((vec4<f32>(_e6.g0_) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) + _e15.g1_));
}

fn anti_scalar_motor_sub(self_324: AntiScalar, other_276: Motor) -> Motor {
    var self_325: AntiScalar;
    var other_277: Motor;

    self_325 = self_324;
    other_277 = other_276;
    let _e6: Motor = other_277;
    let _e9: AntiScalar = self_325;
    let _e18: Motor = other_277;
    return Motor((vec4<f32>(0.0) - _e6.g0_), ((vec4<f32>(_e9.g0_) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) - _e18.g1_));
}

fn anti_scalar_motor_regressive_product(self_326: AntiScalar, other_278: Motor) -> Motor {
    var self_327: AntiScalar;
    var other_279: Motor;

    self_327 = self_326;
    other_279 = other_278;
    let _e4: AntiScalar = self_327;
    let _e7: Motor = other_279;
    let _e10: AntiScalar = self_327;
    let _e13: Motor = other_279;
    return Motor((vec4<f32>(_e4.g0_) * _e7.g0_), (vec4<f32>(_e10.g0_) * _e13.g1_));
}

fn anti_scalar_motor_outer_product(self_328: AntiScalar, other_280: Motor) -> AntiScalar {
    var self_329: AntiScalar;
    var other_281: Motor;

    self_329 = self_328;
    other_281 = other_280;
    let _e4: AntiScalar = self_329;
    let _e6: Motor = other_281;
    return AntiScalar((_e4.g0_ * _e6.g0_.x));
}

fn anti_scalar_motor_geometric_anti_product(self_330: AntiScalar, other_282: Motor) -> Motor {
    var self_331: AntiScalar;
    var other_283: Motor;

    self_331 = self_330;
    other_283 = other_282;
    let _e4: AntiScalar = self_331;
    let _e7: Motor = other_283;
    let _e10: AntiScalar = self_331;
    let _e13: Motor = other_283;
    return Motor((vec4<f32>(_e4.g0_) * _e7.g0_), (vec4<f32>(_e10.g0_) * _e13.g1_));
}

fn anti_scalar_motor_inner_anti_product(self_332: AntiScalar, other_284: Motor) -> Motor {
    var self_333: AntiScalar;
    var other_285: Motor;

    self_333 = self_332;
    other_285 = other_284;
    let _e4: AntiScalar = self_333;
    let _e7: Motor = other_285;
    let _e10: AntiScalar = self_333;
    let _e13: Motor = other_285;
    return Motor((vec4<f32>(_e4.g0_) * _e7.g0_), (vec4<f32>(_e10.g0_) * _e13.g1_));
}

fn anti_scalar_motor_left_anti_contraction(self_334: AntiScalar, other_286: Motor) -> Motor {
    var self_335: AntiScalar;
    var other_287: Motor;

    self_335 = self_334;
    other_287 = other_286;
    let _e4: AntiScalar = self_335;
    let _e7: Motor = other_287;
    let _e10: AntiScalar = self_335;
    let _e13: Motor = other_287;
    return Motor((vec4<f32>(_e4.g0_) * _e7.g0_), (vec4<f32>(_e10.g0_) * _e13.g1_));
}

fn anti_scalar_motor_right_anti_contraction(self_336: AntiScalar, other_288: Motor) -> AntiScalar {
    var self_337: AntiScalar;
    var other_289: Motor;

    self_337 = self_336;
    other_289 = other_288;
    let _e4: AntiScalar = self_337;
    let _e6: Motor = other_289;
    return AntiScalar((_e4.g0_ * _e6.g1_.x));
}

fn anti_scalar_motor_anti_scalar_product(self_338: AntiScalar, other_290: Motor) -> AntiScalar {
    var self_339: AntiScalar;
    var other_291: Motor;

    self_339 = self_338;
    other_291 = other_290;
    let _e4: AntiScalar = self_339;
    let _e6: Motor = other_291;
    return AntiScalar((_e4.g0_ * _e6.g1_.x));
}

fn anti_scalar_point_and_plane_regressive_product(self_340: AntiScalar, other_292: PointAndPlane) -> PointAndPlane {
    var self_341: AntiScalar;
    var other_293: PointAndPlane;

    self_341 = self_340;
    other_293 = other_292;
    let _e4: AntiScalar = self_341;
    let _e7: PointAndPlane = other_293;
    let _e10: AntiScalar = self_341;
    let _e13: PointAndPlane = other_293;
    return PointAndPlane((vec4<f32>(_e4.g0_) * _e7.g0_), (vec4<f32>(_e10.g0_) * _e13.g1_));
}

fn anti_scalar_point_and_plane_geometric_anti_product(self_342: AntiScalar, other_294: PointAndPlane) -> PointAndPlane {
    var self_343: AntiScalar;
    var other_295: PointAndPlane;

    self_343 = self_342;
    other_295 = other_294;
    let _e4: AntiScalar = self_343;
    let _e7: PointAndPlane = other_295;
    let _e10: AntiScalar = self_343;
    let _e13: PointAndPlane = other_295;
    return PointAndPlane((vec4<f32>(_e4.g0_) * _e7.g0_), (vec4<f32>(_e10.g0_) * _e13.g1_));
}

fn anti_scalar_point_and_plane_inner_anti_product(self_344: AntiScalar, other_296: PointAndPlane) -> PointAndPlane {
    var self_345: AntiScalar;
    var other_297: PointAndPlane;

    self_345 = self_344;
    other_297 = other_296;
    let _e4: AntiScalar = self_345;
    let _e7: PointAndPlane = other_297;
    let _e10: AntiScalar = self_345;
    let _e13: PointAndPlane = other_297;
    return PointAndPlane((vec4<f32>(_e4.g0_) * _e7.g0_), (vec4<f32>(_e10.g0_) * _e13.g1_));
}

fn anti_scalar_point_and_plane_left_anti_contraction(self_346: AntiScalar, other_298: PointAndPlane) -> PointAndPlane {
    var self_347: AntiScalar;
    var other_299: PointAndPlane;

    self_347 = self_346;
    other_299 = other_298;
    let _e4: AntiScalar = self_347;
    let _e7: PointAndPlane = other_299;
    let _e10: AntiScalar = self_347;
    let _e13: PointAndPlane = other_299;
    return PointAndPlane((vec4<f32>(_e4.g0_) * _e7.g0_), (vec4<f32>(_e10.g0_) * _e13.g1_));
}

fn anti_scalar_scale(self_348: AntiScalar, other_300: f32) -> AntiScalar {
    var self_349: AntiScalar;
    var other_301: f32;

    self_349 = self_348;
    other_301 = other_300;
    let _e4: AntiScalar = self_349;
    let _e5: f32 = other_301;
    let _e7: AntiScalar = anti_scalar_scalar_geometric_product(_e4, Scalar(_e5));
    return _e7;
}

fn multi_vector_zero() -> MultiVector {
    return MultiVector(vec4<f32>(0.0), vec4<f32>(0.0), vec4<f32>(0.0), vec4<f32>(0.0));
}

fn multi_vector_one() -> MultiVector {
    return MultiVector(vec4<f32>(1.0, 0.0, 0.0, 0.0), vec4<f32>(0.0), vec4<f32>(0.0), vec4<f32>(0.0));
}

fn multi_vector_neg(self_350: MultiVector) -> MultiVector {
    var self_351: MultiVector;

    self_351 = self_350;
    let _e2: MultiVector = self_351;
    let _e8: MultiVector = self_351;
    let _e14: MultiVector = self_351;
    let _e20: MultiVector = self_351;
    return MultiVector((_e2.g0_ * vec4<f32>(-(1.0))), (_e8.g1_ * vec4<f32>(-(1.0))), (_e14.g2_ * vec4<f32>(-(1.0))), (_e20.g3_ * vec4<f32>(-(1.0))));
}

fn multi_vector_automorphism(self_352: MultiVector) -> MultiVector {
    var self_353: MultiVector;

    self_353 = self_352;
    let _e2: MultiVector = self_353;
    let _e4: MultiVector = self_353;
    let _e10: MultiVector = self_353;
    let _e16: MultiVector = self_353;
    return MultiVector(_e2.g0_, (_e4.g1_ * vec4<f32>(-(1.0))), (_e10.g2_ * vec4<f32>(-(1.0))), _e16.g3_);
}

fn multi_vector_reversal(self_354: MultiVector) -> MultiVector {
    var self_355: MultiVector;

    self_355 = self_354;
    let _e2: MultiVector = self_355;
    let _e13: MultiVector = self_355;
    let _e24: MultiVector = self_355;
    let _e33: MultiVector = self_355;
    return MultiVector((_e2.g0_ * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))), (_e13.g1_ * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))), (_e24.g2_ * vec4<f32>(-(1.0), 1.0, 1.0, 1.0)), (_e33.g3_ * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))));
}

fn multi_vector_conjugation(self_356: MultiVector) -> MultiVector {
    var self_357: MultiVector;

    self_357 = self_356;
    let _e2: MultiVector = self_357;
    let _e13: MultiVector = self_357;
    let _e22: MultiVector = self_357;
    let _e33: MultiVector = self_357;
    return MultiVector((_e2.g0_ * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))), (_e13.g1_ * vec4<f32>(-(1.0), 1.0, 1.0, 1.0)), (_e22.g2_ * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))), (_e33.g3_ * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))));
}

fn multi_vector_dual(self_358: MultiVector) -> MultiVector {
    var self_359: MultiVector;

    self_359 = self_358;
    let _e2: MultiVector = self_359;
    let _e4: MultiVector = self_359;
    let _e13: MultiVector = self_359;
    let _e24: MultiVector = self_359;
    return MultiVector(_e2.g3_, (_e4.g2_ * vec4<f32>(-(1.0), 1.0, 1.0, 1.0)), (_e13.g1_ * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))), _e24.g0_);
}

fn multi_vector_anti_reversal(self_360: MultiVector) -> MultiVector {
    var self_361: MultiVector;

    self_361 = self_360;
    let _e2: MultiVector = self_361;
    let _e13: MultiVector = self_361;
    let _e22: MultiVector = self_361;
    let _e33: MultiVector = self_361;
    return MultiVector((_e2.g0_ * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))), (_e13.g1_ * vec4<f32>(-(1.0), 1.0, 1.0, 1.0)), (_e22.g2_ * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))), (_e33.g3_ * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))));
}

fn multi_vector_right_complement(self_362: MultiVector) -> MultiVector {
    var self_363: MultiVector;

    self_363 = self_362;
    let _e2: MultiVector = self_363;
    let _e4: MultiVector = self_363;
    let _e13: MultiVector = self_363;
    let _e24: MultiVector = self_363;
    return MultiVector(_e2.g3_, (_e4.g2_ * vec4<f32>(-(1.0), 1.0, 1.0, 1.0)), (_e13.g1_ * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))), _e24.g0_);
}

fn multi_vector_left_complement(self_364: MultiVector) -> MultiVector {
    var self_365: MultiVector;

    self_365 = self_364;
    let _e2: MultiVector = self_365;
    let _e4: MultiVector = self_365;
    let _e15: MultiVector = self_365;
    let _e24: MultiVector = self_365;
    return MultiVector(_e2.g3_, (_e4.g2_ * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))), (_e15.g1_ * vec4<f32>(-(1.0), 1.0, 1.0, 1.0)), _e24.g0_);
}

fn multi_vector_double_complement(self_366: MultiVector) -> MultiVector {
    var self_367: MultiVector;

    self_367 = self_366;
    let _e2: MultiVector = self_367;
    let _e4: MultiVector = self_367;
    let _e10: MultiVector = self_367;
    let _e16: MultiVector = self_367;
    return MultiVector(_e2.g0_, (_e4.g1_ * vec4<f32>(-(1.0))), (_e10.g2_ * vec4<f32>(-(1.0))), _e16.g3_);
}

fn multi_vector_scalar_into(self_368: MultiVector) -> Scalar {
    var self_369: MultiVector;

    self_369 = self_368;
    let _e2: MultiVector = self_369;
    return Scalar(_e2.g0_.x);
}

fn multi_vector_scalar_add(self_370: MultiVector, other_302: Scalar) -> MultiVector {
    var self_371: MultiVector;
    var other_303: Scalar;

    self_371 = self_370;
    other_303 = other_302;
    let _e4: MultiVector = self_371;
    let _e6: Scalar = other_303;
    let _e16: MultiVector = self_371;
    let _e18: MultiVector = self_371;
    let _e20: MultiVector = self_371;
    return MultiVector((_e4.g0_ + (vec4<f32>(_e6.g0_) * vec4<f32>(1.0, 0.0, 0.0, 0.0))), _e16.g1_, _e18.g2_, _e20.g3_);
}

fn multi_vector_scalar_sub(self_372: MultiVector, other_304: Scalar) -> MultiVector {
    var self_373: MultiVector;
    var other_305: Scalar;

    self_373 = self_372;
    other_305 = other_304;
    let _e4: MultiVector = self_373;
    let _e6: Scalar = other_305;
    let _e16: MultiVector = self_373;
    let _e18: MultiVector = self_373;
    let _e20: MultiVector = self_373;
    return MultiVector((_e4.g0_ - (vec4<f32>(_e6.g0_) * vec4<f32>(1.0, 0.0, 0.0, 0.0))), _e16.g1_, _e18.g2_, _e20.g3_);
}

fn multi_vector_scalar_geometric_product(self_374: MultiVector, other_306: Scalar) -> MultiVector {
    var self_375: MultiVector;
    var other_307: Scalar;

    self_375 = self_374;
    other_307 = other_306;
    let _e4: MultiVector = self_375;
    let _e6: Scalar = other_307;
    let _e10: MultiVector = self_375;
    let _e12: Scalar = other_307;
    let _e16: MultiVector = self_375;
    let _e18: Scalar = other_307;
    let _e22: MultiVector = self_375;
    let _e24: Scalar = other_307;
    return MultiVector((_e4.g0_ * vec4<f32>(_e6.g0_)), (_e10.g1_ * vec4<f32>(_e12.g0_)), (_e16.g2_ * vec4<f32>(_e18.g0_)), (_e22.g3_ * vec4<f32>(_e24.g0_)));
}

fn multi_vector_scalar_regressive_product(self_376: MultiVector, other_308: Scalar) -> Scalar {
    var self_377: MultiVector;
    var other_309: Scalar;

    self_377 = self_376;
    other_309 = other_308;
    let _e4: MultiVector = self_377;
    let _e7: Scalar = other_309;
    return Scalar((_e4.g3_.x * _e7.g0_));
}

fn multi_vector_scalar_outer_product(self_378: MultiVector, other_310: Scalar) -> MultiVector {
    var self_379: MultiVector;
    var other_311: Scalar;

    self_379 = self_378;
    other_311 = other_310;
    let _e4: MultiVector = self_379;
    let _e6: Scalar = other_311;
    let _e10: MultiVector = self_379;
    let _e12: Scalar = other_311;
    let _e16: MultiVector = self_379;
    let _e18: Scalar = other_311;
    let _e22: MultiVector = self_379;
    let _e24: Scalar = other_311;
    return MultiVector((_e4.g0_ * vec4<f32>(_e6.g0_)), (_e10.g1_ * vec4<f32>(_e12.g0_)), (_e16.g2_ * vec4<f32>(_e18.g0_)), (_e22.g3_ * vec4<f32>(_e24.g0_)));
}

fn multi_vector_scalar_inner_product(self_380: MultiVector, other_312: Scalar) -> MultiVector {
    var self_381: MultiVector;
    var other_313: Scalar;

    self_381 = self_380;
    other_313 = other_312;
    let _e4: MultiVector = self_381;
    let _e6: Scalar = other_313;
    let _e10: MultiVector = self_381;
    let _e12: Scalar = other_313;
    let _e16: MultiVector = self_381;
    let _e18: Scalar = other_313;
    let _e22: MultiVector = self_381;
    let _e24: Scalar = other_313;
    return MultiVector((_e4.g0_ * vec4<f32>(_e6.g0_)), (_e10.g1_ * vec4<f32>(_e12.g0_)), (_e16.g2_ * vec4<f32>(_e18.g0_)), (_e22.g3_ * vec4<f32>(_e24.g0_)));
}

fn multi_vector_scalar_left_contraction(self_382: MultiVector, other_314: Scalar) -> Scalar {
    var self_383: MultiVector;
    var other_315: Scalar;

    self_383 = self_382;
    other_315 = other_314;
    let _e4: MultiVector = self_383;
    let _e7: Scalar = other_315;
    return Scalar((_e4.g0_.x * _e7.g0_));
}

fn multi_vector_scalar_right_contraction(self_384: MultiVector, other_316: Scalar) -> MultiVector {
    var self_385: MultiVector;
    var other_317: Scalar;

    self_385 = self_384;
    other_317 = other_316;
    let _e4: MultiVector = self_385;
    let _e6: Scalar = other_317;
    let _e10: MultiVector = self_385;
    let _e12: Scalar = other_317;
    let _e16: MultiVector = self_385;
    let _e18: Scalar = other_317;
    let _e22: MultiVector = self_385;
    let _e24: Scalar = other_317;
    return MultiVector((_e4.g0_ * vec4<f32>(_e6.g0_)), (_e10.g1_ * vec4<f32>(_e12.g0_)), (_e16.g2_ * vec4<f32>(_e18.g0_)), (_e22.g3_ * vec4<f32>(_e24.g0_)));
}

fn multi_vector_scalar_scalar_product(self_386: MultiVector, other_318: Scalar) -> Scalar {
    var self_387: MultiVector;
    var other_319: Scalar;

    self_387 = self_386;
    other_319 = other_318;
    let _e4: MultiVector = self_387;
    let _e7: Scalar = other_319;
    return Scalar((_e4.g0_.x * _e7.g0_));
}

fn multi_vector_anti_scalar_into(self_388: MultiVector) -> AntiScalar {
    var self_389: MultiVector;

    self_389 = self_388;
    let _e2: MultiVector = self_389;
    return AntiScalar(_e2.g3_.x);
}

fn multi_vector_anti_scalar_add(self_390: MultiVector, other_320: AntiScalar) -> MultiVector {
    var self_391: MultiVector;
    var other_321: AntiScalar;

    self_391 = self_390;
    other_321 = other_320;
    let _e4: MultiVector = self_391;
    let _e6: MultiVector = self_391;
    let _e8: MultiVector = self_391;
    let _e10: MultiVector = self_391;
    let _e12: AntiScalar = other_321;
    return MultiVector(_e4.g0_, _e6.g1_, _e8.g2_, (_e10.g3_ + (vec4<f32>(_e12.g0_) * vec4<f32>(1.0, 0.0, 0.0, 0.0))));
}

fn multi_vector_anti_scalar_sub(self_392: MultiVector, other_322: AntiScalar) -> MultiVector {
    var self_393: MultiVector;
    var other_323: AntiScalar;

    self_393 = self_392;
    other_323 = other_322;
    let _e4: MultiVector = self_393;
    let _e6: MultiVector = self_393;
    let _e8: MultiVector = self_393;
    let _e10: MultiVector = self_393;
    let _e12: AntiScalar = other_323;
    return MultiVector(_e4.g0_, _e6.g1_, _e8.g2_, (_e10.g3_ - (vec4<f32>(_e12.g0_) * vec4<f32>(1.0, 0.0, 0.0, 0.0))));
}

fn multi_vector_anti_scalar_regressive_product(self_394: MultiVector, other_324: AntiScalar) -> MultiVector {
    var self_395: MultiVector;
    var other_325: AntiScalar;

    self_395 = self_394;
    other_325 = other_324;
    let _e4: MultiVector = self_395;
    let _e6: AntiScalar = other_325;
    let _e10: MultiVector = self_395;
    let _e12: AntiScalar = other_325;
    let _e16: MultiVector = self_395;
    let _e18: AntiScalar = other_325;
    let _e22: MultiVector = self_395;
    let _e24: AntiScalar = other_325;
    return MultiVector((_e4.g0_ * vec4<f32>(_e6.g0_)), (_e10.g1_ * vec4<f32>(_e12.g0_)), (_e16.g2_ * vec4<f32>(_e18.g0_)), (_e22.g3_ * vec4<f32>(_e24.g0_)));
}

fn multi_vector_anti_scalar_outer_product(self_396: MultiVector, other_326: AntiScalar) -> AntiScalar {
    var self_397: MultiVector;
    var other_327: AntiScalar;

    self_397 = self_396;
    other_327 = other_326;
    let _e4: MultiVector = self_397;
    let _e7: AntiScalar = other_327;
    return AntiScalar((_e4.g0_.x * _e7.g0_));
}

fn multi_vector_anti_scalar_geometric_anti_product(self_398: MultiVector, other_328: AntiScalar) -> MultiVector {
    var self_399: MultiVector;
    var other_329: AntiScalar;

    self_399 = self_398;
    other_329 = other_328;
    let _e4: MultiVector = self_399;
    let _e6: AntiScalar = other_329;
    let _e10: MultiVector = self_399;
    let _e12: AntiScalar = other_329;
    let _e16: MultiVector = self_399;
    let _e18: AntiScalar = other_329;
    let _e22: MultiVector = self_399;
    let _e24: AntiScalar = other_329;
    return MultiVector((_e4.g0_ * vec4<f32>(_e6.g0_)), (_e10.g1_ * vec4<f32>(_e12.g0_)), (_e16.g2_ * vec4<f32>(_e18.g0_)), (_e22.g3_ * vec4<f32>(_e24.g0_)));
}

fn multi_vector_anti_scalar_inner_anti_product(self_400: MultiVector, other_330: AntiScalar) -> MultiVector {
    var self_401: MultiVector;
    var other_331: AntiScalar;

    self_401 = self_400;
    other_331 = other_330;
    let _e4: MultiVector = self_401;
    let _e6: AntiScalar = other_331;
    let _e10: MultiVector = self_401;
    let _e12: AntiScalar = other_331;
    let _e16: MultiVector = self_401;
    let _e18: AntiScalar = other_331;
    let _e22: MultiVector = self_401;
    let _e24: AntiScalar = other_331;
    return MultiVector((_e4.g0_ * vec4<f32>(_e6.g0_)), (_e10.g1_ * vec4<f32>(_e12.g0_)), (_e16.g2_ * vec4<f32>(_e18.g0_)), (_e22.g3_ * vec4<f32>(_e24.g0_)));
}

fn multi_vector_anti_scalar_left_anti_contraction(self_402: MultiVector, other_332: AntiScalar) -> AntiScalar {
    var self_403: MultiVector;
    var other_333: AntiScalar;

    self_403 = self_402;
    other_333 = other_332;
    let _e4: MultiVector = self_403;
    let _e7: AntiScalar = other_333;
    return AntiScalar((_e4.g3_.x * _e7.g0_));
}

fn multi_vector_anti_scalar_right_anti_contraction(self_404: MultiVector, other_334: AntiScalar) -> MultiVector {
    var self_405: MultiVector;
    var other_335: AntiScalar;

    self_405 = self_404;
    other_335 = other_334;
    let _e4: MultiVector = self_405;
    let _e6: AntiScalar = other_335;
    let _e10: MultiVector = self_405;
    let _e12: AntiScalar = other_335;
    let _e16: MultiVector = self_405;
    let _e18: AntiScalar = other_335;
    let _e22: MultiVector = self_405;
    let _e24: AntiScalar = other_335;
    return MultiVector((_e4.g0_ * vec4<f32>(_e6.g0_)), (_e10.g1_ * vec4<f32>(_e12.g0_)), (_e16.g2_ * vec4<f32>(_e18.g0_)), (_e22.g3_ * vec4<f32>(_e24.g0_)));
}

fn multi_vector_anti_scalar_anti_scalar_product(self_406: MultiVector, other_336: AntiScalar) -> AntiScalar {
    var self_407: MultiVector;
    var other_337: AntiScalar;

    self_407 = self_406;
    other_337 = other_336;
    let _e4: MultiVector = self_407;
    let _e7: AntiScalar = other_337;
    return AntiScalar((_e4.g3_.x * _e7.g0_));
}

fn multi_vector_multi_vector_add(self_408: MultiVector, other_338: MultiVector) -> MultiVector {
    var self_409: MultiVector;
    var other_339: MultiVector;

    self_409 = self_408;
    other_339 = other_338;
    let _e4: MultiVector = self_409;
    let _e6: MultiVector = other_339;
    let _e9: MultiVector = self_409;
    let _e11: MultiVector = other_339;
    let _e14: MultiVector = self_409;
    let _e16: MultiVector = other_339;
    let _e19: MultiVector = self_409;
    let _e21: MultiVector = other_339;
    return MultiVector((_e4.g0_ + _e6.g0_), (_e9.g1_ + _e11.g1_), (_e14.g2_ + _e16.g2_), (_e19.g3_ + _e21.g3_));
}

fn multi_vector_multi_vector_sub(self_410: MultiVector, other_340: MultiVector) -> MultiVector {
    var self_411: MultiVector;
    var other_341: MultiVector;

    self_411 = self_410;
    other_341 = other_340;
    let _e4: MultiVector = self_411;
    let _e6: MultiVector = other_341;
    let _e9: MultiVector = self_411;
    let _e11: MultiVector = other_341;
    let _e14: MultiVector = self_411;
    let _e16: MultiVector = other_341;
    let _e19: MultiVector = self_411;
    let _e21: MultiVector = other_341;
    return MultiVector((_e4.g0_ - _e6.g0_), (_e9.g1_ - _e11.g1_), (_e14.g2_ - _e16.g2_), (_e19.g3_ - _e21.g3_));
}

fn multi_vector_multi_vector_mul(self_412: MultiVector, other_342: MultiVector) -> MultiVector {
    var self_413: MultiVector;
    var other_343: MultiVector;

    self_413 = self_412;
    other_343 = other_342;
    let _e4: MultiVector = self_413;
    let _e6: MultiVector = other_343;
    let _e9: MultiVector = self_413;
    let _e11: MultiVector = other_343;
    let _e14: MultiVector = self_413;
    let _e16: MultiVector = other_343;
    let _e19: MultiVector = self_413;
    let _e21: MultiVector = other_343;
    return MultiVector((_e4.g0_ * _e6.g0_), (_e9.g1_ * _e11.g1_), (_e14.g2_ * _e16.g2_), (_e19.g3_ * _e21.g3_));
}

fn multi_vector_multi_vector_div(self_414: MultiVector, other_344: MultiVector) -> MultiVector {
    var self_415: MultiVector;
    var other_345: MultiVector;

    self_415 = self_414;
    other_345 = other_344;
    let _e4: MultiVector = self_415;
    let _e7: MultiVector = self_415;
    let _e10: MultiVector = self_415;
    let _e13: MultiVector = self_415;
    let _e23: MultiVector = other_345;
    let _e26: MultiVector = other_345;
    let _e29: MultiVector = other_345;
    let _e32: MultiVector = other_345;
    let _e43: MultiVector = self_415;
    let _e46: MultiVector = self_415;
    let _e49: MultiVector = self_415;
    let _e52: MultiVector = self_415;
    let _e62: MultiVector = other_345;
    let _e65: MultiVector = other_345;
    let _e68: MultiVector = other_345;
    let _e71: MultiVector = other_345;
    let _e82: MultiVector = self_415;
    let _e85: MultiVector = self_415;
    let _e88: MultiVector = self_415;
    let _e91: MultiVector = self_415;
    let _e101: MultiVector = other_345;
    let _e104: MultiVector = other_345;
    let _e107: MultiVector = other_345;
    let _e110: MultiVector = other_345;
    let _e121: MultiVector = self_415;
    let _e124: MultiVector = self_415;
    let _e127: MultiVector = self_415;
    let _e130: MultiVector = self_415;
    let _e140: MultiVector = other_345;
    let _e143: MultiVector = other_345;
    let _e146: MultiVector = other_345;
    let _e149: MultiVector = other_345;
    return MultiVector((((vec4<f32>(_e4.g0_.x, _e7.g0_.y, _e10.g0_.z, _e13.g0_.w) * vec4<f32>(1.0, 1.0, 1.0, 1.0)) / vec4<f32>(_e23.g0_.x, _e26.g0_.y, _e29.g0_.z, _e32.g0_.w)) * vec4<f32>(1.0, 1.0, 1.0, 1.0)), (((vec4<f32>(_e43.g1_.x, _e46.g1_.y, _e49.g1_.z, _e52.g1_.w) * vec4<f32>(1.0, 1.0, 1.0, 1.0)) / vec4<f32>(_e62.g1_.x, _e65.g1_.y, _e68.g1_.z, _e71.g1_.w)) * vec4<f32>(1.0, 1.0, 1.0, 1.0)), (((vec4<f32>(_e82.g2_.x, _e85.g2_.y, _e88.g2_.z, _e91.g2_.w) * vec4<f32>(1.0, 1.0, 1.0, 1.0)) / vec4<f32>(_e101.g2_.x, _e104.g2_.y, _e107.g2_.z, _e110.g2_.w)) * vec4<f32>(1.0, 1.0, 1.0, 1.0)), (((vec4<f32>(_e121.g3_.x, _e124.g3_.y, _e127.g3_.z, _e130.g3_.w) * vec4<f32>(1.0, 1.0, 1.0, 1.0)) / vec4<f32>(_e140.g3_.x, _e143.g3_.y, _e146.g3_.z, _e149.g3_.w)) * vec4<f32>(1.0, 1.0, 1.0, 1.0)));
}

fn multi_vector_multi_vector_geometric_product(self_416: MultiVector, other_346: MultiVector) -> MultiVector {
    var self_417: MultiVector;
    var other_347: MultiVector;

    self_417 = self_416;
    other_347 = other_346;
    let _e4: MultiVector = self_417;
    let _e8: MultiVector = other_347;
    let _e11: MultiVector = self_417;
    let _e15: MultiVector = other_347;
    let _e28: MultiVector = self_417;
    let _e32: MultiVector = other_347;
    let _e45: MultiVector = self_417;
    let _e49: MultiVector = other_347;
    let _e62: MultiVector = self_417;
    let _e66: MultiVector = other_347;
    let _e77: MultiVector = self_417;
    let _e81: MultiVector = other_347;
    let _e93: MultiVector = self_417;
    let _e97: MultiVector = other_347;
    let _e109: MultiVector = self_417;
    let _e113: MultiVector = other_347;
    let _e125: MultiVector = self_417;
    let _e129: MultiVector = other_347;
    let _e132: MultiVector = self_417;
    let _e136: MultiVector = other_347;
    let _e149: MultiVector = self_417;
    let _e153: MultiVector = other_347;
    let _e166: MultiVector = self_417;
    let _e170: MultiVector = other_347;
    let _e183: MultiVector = self_417;
    let _e187: MultiVector = other_347;
    let _e200: MultiVector = self_417;
    let _e204: MultiVector = other_347;
    let _e216: MultiVector = self_417;
    let _e220: MultiVector = other_347;
    let _e232: MultiVector = self_417;
    let _e236: MultiVector = other_347;
    let _e248: MultiVector = self_417;
    let _e252: MultiVector = other_347;
    let _e256: MultiVector = self_417;
    let _e260: MultiVector = other_347;
    let _e273: MultiVector = self_417;
    let _e277: MultiVector = other_347;
    let _e290: MultiVector = self_417;
    let _e294: MultiVector = other_347;
    let _e307: MultiVector = self_417;
    let _e311: MultiVector = other_347;
    let _e315: MultiVector = self_417;
    let _e319: MultiVector = other_347;
    let _e332: MultiVector = self_417;
    let _e336: MultiVector = other_347;
    let _e349: MultiVector = self_417;
    let _e353: MultiVector = other_347;
    let _e366: MultiVector = self_417;
    let _e370: MultiVector = other_347;
    let _e373: MultiVector = self_417;
    let _e377: MultiVector = other_347;
    let _e390: MultiVector = self_417;
    let _e394: MultiVector = other_347;
    let _e407: MultiVector = self_417;
    let _e411: MultiVector = other_347;
    let _e424: MultiVector = self_417;
    let _e428: MultiVector = other_347;
    let _e441: MultiVector = self_417;
    let _e445: MultiVector = other_347;
    let _e457: MultiVector = self_417;
    let _e461: MultiVector = other_347;
    let _e473: MultiVector = self_417;
    let _e477: MultiVector = other_347;
    let _e489: MultiVector = self_417;
    let _e493: MultiVector = other_347;
    let _e496: MultiVector = self_417;
    let _e500: MultiVector = other_347;
    let _e513: MultiVector = self_417;
    let _e517: MultiVector = other_347;
    let _e530: MultiVector = self_417;
    let _e534: MultiVector = other_347;
    let _e547: MultiVector = self_417;
    let _e551: MultiVector = other_347;
    let _e555: MultiVector = self_417;
    let _e559: MultiVector = other_347;
    let _e572: MultiVector = self_417;
    let _e576: MultiVector = other_347;
    let _e589: MultiVector = self_417;
    let _e593: MultiVector = other_347;
    let _e606: MultiVector = self_417;
    let _e610: MultiVector = other_347;
    let _e614: MultiVector = self_417;
    let _e618: MultiVector = other_347;
    let _e631: MultiVector = self_417;
    let _e635: MultiVector = other_347;
    let _e648: MultiVector = self_417;
    let _e652: MultiVector = other_347;
    let _e665: MultiVector = self_417;
    let _e669: MultiVector = other_347;
    let _e682: MultiVector = self_417;
    let _e686: MultiVector = other_347;
    let _e698: MultiVector = self_417;
    let _e702: MultiVector = other_347;
    let _e714: MultiVector = self_417;
    let _e718: MultiVector = other_347;
    return MultiVector(((((((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.y) * _e15.g0_.yxwz) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e28.g0_.z) * _e32.g0_.zwxy) * vec4<f32>(-(1.0), -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e45.g0_.w) * _e49.g0_.wzyx) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))) + ((vec4<f32>(_e62.g2_.x) * _e66.g2_) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0))) + ((vec4<f32>(_e77.g2_.y) * _e81.g2_.yxwz) * vec4<f32>(1.0, 1.0, -(1.0), 1.0))) + ((vec4<f32>(_e93.g2_.z) * _e97.g2_.zwxy) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e109.g2_.w) * _e113.g2_.wzyx) * vec4<f32>(1.0, -(1.0), 1.0, 1.0))), ((((((((((((((((vec4<f32>(_e125.g0_.x) * _e129.g1_) + ((vec4<f32>(_e132.g0_.y) * _e136.g1_.yxwz) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e149.g0_.z) * _e153.g1_.zwxy) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e166.g0_.w) * _e170.g1_.wzyx) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e183.g1_.x) * _e187.g0_) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0)))) + ((vec4<f32>(_e200.g1_.y) * _e204.g0_.yxwz) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e216.g1_.z) * _e220.g0_.zwxy) * vec4<f32>(1.0, -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e232.g1_.w) * _e236.g0_.wzyx) * vec4<f32>(1.0, 1.0, -(1.0), 1.0))) + (vec4<f32>(_e248.g2_.x) * _e252.g3_)) + ((vec4<f32>(_e256.g2_.y) * _e260.g3_.yxwz) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))) + ((vec4<f32>(_e273.g2_.z) * _e277.g3_.zwxy) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e290.g2_.w) * _e294.g3_.wzyx) * vec4<f32>(-(1.0), -(1.0), 1.0, 1.0))) - (vec4<f32>(_e307.g3_.x) * _e311.g2_)) + ((vec4<f32>(_e315.g3_.y) * _e319.g2_.yxwz) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e332.g3_.z) * _e336.g2_.zwxy) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e349.g3_.w) * _e353.g2_.wzyx) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))), ((((((((vec4<f32>(_e366.g0_.x) * _e370.g2_) + ((vec4<f32>(_e373.g0_.y) * _e377.g2_.yxwz) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e390.g0_.z) * _e394.g2_.zwxy) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e407.g0_.w) * _e411.g2_.wzyx) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e424.g2_.x) * _e428.g0_) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0)))) + ((vec4<f32>(_e441.g2_.y) * _e445.g0_.yxwz) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e457.g2_.z) * _e461.g0_.zwxy) * vec4<f32>(1.0, -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e473.g2_.w) * _e477.g0_.wzyx) * vec4<f32>(1.0, 1.0, -(1.0), 1.0))), ((((((((((((((((vec4<f32>(_e489.g0_.x) * _e493.g3_) + ((vec4<f32>(_e496.g0_.y) * _e500.g3_.yxwz) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e513.g0_.z) * _e517.g3_.zwxy) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e530.g0_.w) * _e534.g3_.wzyx) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))) + (vec4<f32>(_e547.g1_.x) * _e551.g2_)) + ((vec4<f32>(_e555.g1_.y) * _e559.g2_.yxwz) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))) + ((vec4<f32>(_e572.g1_.z) * _e576.g2_.zwxy) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e589.g1_.w) * _e593.g2_.wzyx) * vec4<f32>(-(1.0), -(1.0), 1.0, 1.0))) - (vec4<f32>(_e606.g2_.x) * _e610.g1_)) + ((vec4<f32>(_e614.g2_.y) * _e618.g1_.yxwz) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e631.g2_.z) * _e635.g1_.zwxy) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e648.g2_.w) * _e652.g1_.wzyx) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e665.g3_.x) * _e669.g0_) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0)))) + ((vec4<f32>(_e682.g3_.y) * _e686.g0_.yxwz) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e698.g3_.z) * _e702.g0_.zwxy) * vec4<f32>(1.0, -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e714.g3_.w) * _e718.g0_.wzyx) * vec4<f32>(1.0, 1.0, -(1.0), 1.0))));
}

fn multi_vector_multi_vector_regressive_product(self_418: MultiVector, other_348: MultiVector) -> MultiVector {
    var self_419: MultiVector;
    var other_349: MultiVector;

    self_419 = self_418;
    other_349 = other_348;
    let _e4: MultiVector = self_419;
    let _e8: MultiVector = other_349;
    let _e18: MultiVector = self_419;
    let _e22: MultiVector = other_349;
    let _e33: MultiVector = self_419;
    let _e37: MultiVector = other_349;
    let _e48: MultiVector = self_419;
    let _e52: MultiVector = other_349;
    let _e64: MultiVector = self_419;
    let _e68: MultiVector = other_349;
    let _e81: MultiVector = self_419;
    let _e85: MultiVector = other_349;
    let _e98: MultiVector = self_419;
    let _e102: MultiVector = other_349;
    let _e115: MultiVector = self_419;
    let _e119: MultiVector = other_349;
    let _e130: MultiVector = self_419;
    let _e134: MultiVector = other_349;
    let _e146: MultiVector = self_419;
    let _e150: MultiVector = other_349;
    let _e162: MultiVector = self_419;
    let _e166: MultiVector = other_349;
    let _e178: MultiVector = self_419;
    let _e182: MultiVector = other_349;
    let _e186: MultiVector = self_419;
    let _e190: MultiVector = other_349;
    let _e202: MultiVector = self_419;
    let _e206: MultiVector = other_349;
    let _e218: MultiVector = self_419;
    let _e222: MultiVector = other_349;
    let _e234: MultiVector = self_419;
    let _e238: MultiVector = other_349;
    let _e250: MultiVector = self_419;
    let _e254: MultiVector = other_349;
    let _e265: MultiVector = self_419;
    let _e269: MultiVector = other_349;
    let _e281: MultiVector = self_419;
    let _e285: MultiVector = other_349;
    let _e297: MultiVector = self_419;
    let _e301: MultiVector = other_349;
    let _e305: MultiVector = self_419;
    let _e309: MultiVector = other_349;
    let _e322: MultiVector = self_419;
    let _e326: MultiVector = other_349;
    let _e339: MultiVector = self_419;
    let _e343: MultiVector = other_349;
    let _e356: MultiVector = self_419;
    let _e360: MultiVector = other_349;
    let _e372: MultiVector = self_419;
    let _e376: MultiVector = other_349;
    let _e387: MultiVector = self_419;
    let _e391: MultiVector = other_349;
    let _e403: MultiVector = self_419;
    let _e407: MultiVector = other_349;
    let _e419: MultiVector = self_419;
    let _e423: MultiVector = other_349;
    let _e435: MultiVector = self_419;
    let _e439: MultiVector = other_349;
    let _e451: MultiVector = self_419;
    let _e455: MultiVector = other_349;
    let _e459: MultiVector = self_419;
    let _e463: MultiVector = other_349;
    let _e475: MultiVector = self_419;
    let _e479: MultiVector = other_349;
    let _e491: MultiVector = self_419;
    let _e495: MultiVector = other_349;
    let _e499: MultiVector = self_419;
    let _e503: MultiVector = other_349;
    let _e515: MultiVector = self_419;
    let _e519: MultiVector = other_349;
    let _e531: MultiVector = self_419;
    let _e535: MultiVector = other_349;
    let _e547: MultiVector = self_419;
    let _e550: MultiVector = self_419;
    let _e553: MultiVector = self_419;
    let _e556: MultiVector = self_419;
    let _e560: MultiVector = other_349;
    let _e563: MultiVector = other_349;
    let _e566: MultiVector = other_349;
    let _e569: MultiVector = other_349;
    let _e582: MultiVector = self_419;
    let _e586: MultiVector = other_349;
    let _e597: MultiVector = self_419;
    let _e601: MultiVector = other_349;
    let _e613: MultiVector = self_419;
    let _e617: MultiVector = other_349;
    let _e621: MultiVector = self_419;
    let _e625: MultiVector = other_349;
    let _e637: MultiVector = self_419;
    let _e641: MultiVector = other_349;
    let _e653: MultiVector = self_419;
    let _e656: MultiVector = self_419;
    let _e659: MultiVector = self_419;
    let _e662: MultiVector = self_419;
    let _e666: MultiVector = other_349;
    let _e669: MultiVector = other_349;
    let _e672: MultiVector = other_349;
    let _e675: MultiVector = other_349;
    return MultiVector((((((((((((((((((vec4<f32>(_e4.g0_.y) * _e8.g3_.yxyy) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) + ((vec4<f32>(_e18.g0_.z) * _e22.g3_.zzxz) * vec4<f32>(1.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e33.g0_.w) * _e37.g3_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e48.g1_.x) * vec4<f32>(_e52.g2_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e64.g1_.y) * _e68.g2_.yxyy) * vec4<f32>(-(1.0), -(1.0), 0.0, 0.0))) + ((vec4<f32>(_e81.g1_.z) * _e85.g2_.zzxz) * vec4<f32>(-(1.0), 0.0, -(1.0), 0.0))) + ((vec4<f32>(_e98.g1_.w) * _e102.g2_.wwwx) * vec4<f32>(-(1.0), 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e115.g2_.x) * _e119.g1_) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0))) + ((vec4<f32>(_e130.g2_.y) * vec4<f32>(_e134.g1_.y)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e146.g2_.z) * vec4<f32>(_e150.g1_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e162.g2_.w) * vec4<f32>(_e166.g1_.w)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + (vec4<f32>(_e178.g3_.x) * _e182.g0_)) + ((vec4<f32>(_e186.g3_.y) * vec4<f32>(_e190.g0_.y)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e202.g3_.z) * vec4<f32>(_e206.g0_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e218.g3_.w) * vec4<f32>(_e222.g0_.w)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e234.g0_.x) * vec4<f32>(_e238.g3_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))), (((((((((vec4<f32>(_e250.g1_.y) * _e254.g3_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0)) + ((vec4<f32>(_e265.g1_.z) * _e269.g3_.zzxz) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0))) + ((vec4<f32>(_e281.g1_.w) * _e285.g3_.wwwx) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))) + (vec4<f32>(_e297.g3_.x) * _e301.g1_)) + ((vec4<f32>(_e305.g3_.y) * vec4<f32>(_e309.g1_.y)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((vec4<f32>(_e322.g3_.z) * vec4<f32>(_e326.g1_.z)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((vec4<f32>(_e339.g3_.w) * vec4<f32>(_e343.g1_.w)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((vec4<f32>(_e356.g1_.x) * vec4<f32>(_e360.g3_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))), ((((((((((((((vec4<f32>(_e372.g0_.z) * _e376.g1_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e387.g0_.w) * _e391.g1_.zzyz) * vec4<f32>(0.0, -(1.0), 1.0, 0.0))) + ((vec4<f32>(_e403.g1_.y) * _e407.g0_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e419.g1_.z) * _e423.g0_.wwwy) * vec4<f32>(0.0, -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e435.g1_.w) * _e439.g0_.zzyz) * vec4<f32>(0.0, 1.0, -(1.0), 0.0))) + (vec4<f32>(_e451.g2_.x) * _e455.g3_)) + ((vec4<f32>(_e459.g2_.z) * vec4<f32>(_e463.g3_.x)) * vec4<f32>(0.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e475.g2_.w) * vec4<f32>(_e479.g3_.x)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + (vec4<f32>(_e491.g3_.x) * _e495.g2_)) + ((vec4<f32>(_e499.g3_.y) * vec4<f32>(_e503.g2_.x)) * vec4<f32>(0.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e515.g3_.z) * vec4<f32>(_e519.g2_.x)) * vec4<f32>(0.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e531.g3_.w) * vec4<f32>(_e535.g2_.x)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e547.g0_.x, _e550.g2_.y, _e553.g0_.y, _e556.g0_.y) * vec4<f32>(_e560.g1_.x, _e563.g3_.x, _e566.g1_.w, _e569.g1_.z)) * vec4<f32>(0.0, 1.0, -(1.0), 1.0))), (((((((vec4<f32>(_e582.g1_.z) * _e586.g1_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e597.g1_.w) * _e601.g1_.zzyz) * vec4<f32>(0.0, -(1.0), 1.0, 0.0))) + (vec4<f32>(_e613.g3_.x) * _e617.g3_)) + ((vec4<f32>(_e621.g3_.z) * vec4<f32>(_e625.g3_.x)) * vec4<f32>(0.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e637.g3_.w) * vec4<f32>(_e641.g3_.x)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e653.g1_.x, _e656.g3_.y, _e659.g1_.y, _e662.g1_.y) * vec4<f32>(_e666.g1_.x, _e669.g3_.x, _e672.g1_.w, _e675.g1_.z)) * vec4<f32>(0.0, 1.0, -(1.0), 1.0))));
}

fn multi_vector_multi_vector_outer_product(self_420: MultiVector, other_350: MultiVector) -> MultiVector {
    var self_421: MultiVector;
    var other_351: MultiVector;

    self_421 = self_420;
    other_351 = other_350;
    let _e4: MultiVector = self_421;
    let _e8: MultiVector = other_351;
    let _e11: MultiVector = self_421;
    let _e15: MultiVector = other_351;
    let _e27: MultiVector = self_421;
    let _e31: MultiVector = other_351;
    let _e43: MultiVector = self_421;
    let _e47: MultiVector = other_351;
    let _e59: MultiVector = self_421;
    let _e61: MultiVector = other_351;
    let _e73: MultiVector = self_421;
    let _e77: MultiVector = other_351;
    let _e80: MultiVector = self_421;
    let _e84: MultiVector = other_351;
    let _e97: MultiVector = self_421;
    let _e101: MultiVector = other_351;
    let _e113: MultiVector = self_421;
    let _e117: MultiVector = other_351;
    let _e129: MultiVector = self_421;
    let _e133: MultiVector = other_351;
    let _e145: MultiVector = self_421;
    let _e149: MultiVector = other_351;
    let _e161: MultiVector = self_421;
    let _e165: MultiVector = other_351;
    let _e177: MultiVector = self_421;
    let _e181: MultiVector = other_351;
    let _e193: MultiVector = self_421;
    let _e197: MultiVector = other_351;
    let _e209: MultiVector = self_421;
    let _e213: MultiVector = other_351;
    let _e225: MultiVector = self_421;
    let _e229: MultiVector = other_351;
    let _e241: MultiVector = self_421;
    let _e243: MultiVector = other_351;
    let _e258: MultiVector = self_421;
    let _e262: MultiVector = other_351;
    let _e265: MultiVector = self_421;
    let _e269: MultiVector = other_351;
    let _e281: MultiVector = self_421;
    let _e285: MultiVector = other_351;
    let _e297: MultiVector = self_421;
    let _e301: MultiVector = other_351;
    let _e313: MultiVector = self_421;
    let _e317: MultiVector = other_351;
    let _e328: MultiVector = self_421;
    let _e332: MultiVector = other_351;
    let _e343: MultiVector = self_421;
    let _e347: MultiVector = other_351;
    let _e358: MultiVector = self_421;
    let _e361: MultiVector = other_351;
    let _e372: MultiVector = self_421;
    let _e376: MultiVector = other_351;
    let _e379: MultiVector = self_421;
    let _e383: MultiVector = other_351;
    let _e395: MultiVector = self_421;
    let _e399: MultiVector = other_351;
    let _e411: MultiVector = self_421;
    let _e415: MultiVector = other_351;
    let _e419: MultiVector = self_421;
    let _e423: MultiVector = other_351;
    let _e436: MultiVector = self_421;
    let _e440: MultiVector = other_351;
    let _e453: MultiVector = self_421;
    let _e457: MultiVector = other_351;
    let _e470: MultiVector = self_421;
    let _e474: MultiVector = other_351;
    let _e487: MultiVector = self_421;
    let _e491: MultiVector = other_351;
    let _e503: MultiVector = self_421;
    let _e507: MultiVector = other_351;
    let _e519: MultiVector = self_421;
    let _e523: MultiVector = other_351;
    let _e535: MultiVector = self_421;
    let _e539: MultiVector = other_351;
    let _e551: MultiVector = self_421;
    let _e555: MultiVector = other_351;
    let _e566: MultiVector = self_421;
    let _e570: MultiVector = other_351;
    let _e581: MultiVector = self_421;
    let _e585: MultiVector = other_351;
    let _e596: MultiVector = self_421;
    let _e599: MultiVector = other_351;
    return MultiVector((((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g2_.y) * _e15.g2_.wwwz) * vec4<f32>(0.0, 0.0, -(1.0), 1.0))) + ((vec4<f32>(_e27.g2_.z) * _e31.g2_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0)))) + ((vec4<f32>(_e43.g2_.w) * _e47.g2_.zzyz) * vec4<f32>(0.0, -(1.0), 1.0, 0.0))) + ((_e59.g0_ * vec4<f32>(_e61.g0_.x)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))), ((((((((((((vec4<f32>(_e73.g0_.x) * _e77.g1_) + ((vec4<f32>(_e80.g1_.x) * _e84.g0_) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0)))) + ((vec4<f32>(_e97.g1_.y) * vec4<f32>(_e101.g0_.x)) * vec4<f32>(0.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e113.g1_.z) * vec4<f32>(_e117.g0_.x)) * vec4<f32>(0.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e129.g1_.w) * vec4<f32>(_e133.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e145.g2_.y) * _e149.g3_.wwwz) * vec4<f32>(0.0, 0.0, -(1.0), 1.0))) + ((vec4<f32>(_e161.g2_.z) * _e165.g3_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0)))) + ((vec4<f32>(_e177.g2_.w) * _e181.g3_.zzyz) * vec4<f32>(0.0, -(1.0), 1.0, 0.0))) + ((vec4<f32>(_e193.g3_.y) * _e197.g2_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e209.g3_.z) * _e213.g2_.wwwy) * vec4<f32>(0.0, -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e225.g3_.w) * _e229.g2_.zzyz) * vec4<f32>(0.0, 1.0, -(1.0), 0.0))) + ((_e241.g0_ * vec4<f32>(_e243.g1_.x)) * vec4<f32>(0.0, -(1.0), -(1.0), -(1.0)))), ((((((((vec4<f32>(_e258.g0_.x) * _e262.g2_) + ((vec4<f32>(_e265.g0_.z) * vec4<f32>(_e269.g2_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e281.g0_.w) * vec4<f32>(_e285.g2_.w)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e297.g2_.x) * vec4<f32>(_e301.g0_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e313.g2_.y) * _e317.g0_.yxyy) * vec4<f32>(1.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e328.g2_.z) * _e332.g0_.zzxz) * vec4<f32>(1.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e343.g2_.w) * _e347.g0_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, 1.0))) + ((_e358.g0_.yxxx * _e361.g2_.yxxx) * vec4<f32>(1.0, 0.0, 0.0, 0.0))), ((((((((((((((((vec4<f32>(_e372.g0_.x) * _e376.g3_) + ((vec4<f32>(_e379.g0_.z) * vec4<f32>(_e383.g3_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e395.g0_.w) * vec4<f32>(_e399.g3_.w)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + (vec4<f32>(_e411.g1_.x) * _e415.g2_)) + ((vec4<f32>(_e419.g1_.y) * vec4<f32>(_e423.g2_.y)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((vec4<f32>(_e436.g1_.z) * vec4<f32>(_e440.g2_.z)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((vec4<f32>(_e453.g1_.w) * vec4<f32>(_e457.g2_.w)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((vec4<f32>(_e470.g2_.x) * vec4<f32>(_e474.g1_.x)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((vec4<f32>(_e487.g2_.y) * _e491.g1_.yxyy) * vec4<f32>(1.0, -(1.0), 0.0, 0.0))) + ((vec4<f32>(_e503.g2_.z) * _e507.g1_.zzxz) * vec4<f32>(1.0, 0.0, -(1.0), 0.0))) + ((vec4<f32>(_e519.g2_.w) * _e523.g1_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e535.g3_.x) * vec4<f32>(_e539.g0_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e551.g3_.y) * _e555.g0_.yxyy) * vec4<f32>(1.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e566.g3_.z) * _e570.g0_.zzxz) * vec4<f32>(1.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e581.g3_.w) * _e585.g0_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, 1.0))) + ((_e596.g0_.yxxx * _e599.g3_.yxxx) * vec4<f32>(1.0, 0.0, 0.0, 0.0))));
}

fn multi_vector_multi_vector_inner_product(self_422: MultiVector, other_352: MultiVector) -> MultiVector {
    var self_423: MultiVector;
    var other_353: MultiVector;

    self_423 = self_422;
    other_353 = other_352;
    let _e4: MultiVector = self_423;
    let _e8: MultiVector = other_353;
    let _e11: MultiVector = self_423;
    let _e15: MultiVector = other_353;
    let _e27: MultiVector = self_423;
    let _e31: MultiVector = other_353;
    let _e43: MultiVector = self_423;
    let _e47: MultiVector = other_353;
    let _e58: MultiVector = self_423;
    let _e62: MultiVector = other_353;
    let _e73: MultiVector = self_423;
    let _e77: MultiVector = other_353;
    let _e88: MultiVector = self_423;
    let _e92: MultiVector = other_353;
    let _e103: MultiVector = self_423;
    let _e106: MultiVector = other_353;
    let _e118: MultiVector = self_423;
    let _e122: MultiVector = other_353;
    let _e125: MultiVector = self_423;
    let _e129: MultiVector = other_353;
    let _e141: MultiVector = self_423;
    let _e145: MultiVector = other_353;
    let _e157: MultiVector = self_423;
    let _e161: MultiVector = other_353;
    let _e173: MultiVector = self_423;
    let _e177: MultiVector = other_353;
    let _e188: MultiVector = self_423;
    let _e192: MultiVector = other_353;
    let _e203: MultiVector = self_423;
    let _e207: MultiVector = other_353;
    let _e218: MultiVector = self_423;
    let _e222: MultiVector = other_353;
    let _e234: MultiVector = self_423;
    let _e238: MultiVector = other_353;
    let _e250: MultiVector = self_423;
    let _e254: MultiVector = other_353;
    let _e266: MultiVector = self_423;
    let _e270: MultiVector = other_353;
    let _e282: MultiVector = self_423;
    let _e286: MultiVector = other_353;
    let _e290: MultiVector = self_423;
    let _e294: MultiVector = other_353;
    let _e306: MultiVector = self_423;
    let _e310: MultiVector = other_353;
    let _e322: MultiVector = self_423;
    let _e326: MultiVector = other_353;
    let _e338: MultiVector = self_423;
    let _e341: MultiVector = other_353;
    let _e352: MultiVector = self_423;
    let _e356: MultiVector = other_353;
    let _e359: MultiVector = self_423;
    let _e363: MultiVector = other_353;
    let _e376: MultiVector = self_423;
    let _e380: MultiVector = other_353;
    let _e393: MultiVector = self_423;
    let _e397: MultiVector = other_353;
    let _e410: MultiVector = self_423;
    let _e414: MultiVector = other_353;
    let _e426: MultiVector = self_423;
    let _e430: MultiVector = other_353;
    let _e442: MultiVector = self_423;
    let _e446: MultiVector = other_353;
    let _e458: MultiVector = self_423;
    let _e461: MultiVector = other_353;
    let _e474: MultiVector = self_423;
    let _e478: MultiVector = other_353;
    let _e481: MultiVector = self_423;
    let _e485: MultiVector = other_353;
    let _e497: MultiVector = self_423;
    let _e501: MultiVector = other_353;
    let _e513: MultiVector = self_423;
    let _e517: MultiVector = other_353;
    let _e529: MultiVector = self_423;
    let _e533: MultiVector = other_353;
    let _e545: MultiVector = self_423;
    let _e549: MultiVector = other_353;
    let _e561: MultiVector = self_423;
    let _e565: MultiVector = other_353;
    let _e577: MultiVector = self_423;
    let _e581: MultiVector = other_353;
    let _e594: MultiVector = self_423;
    let _e598: MultiVector = other_353;
    let _e610: MultiVector = self_423;
    let _e614: MultiVector = other_353;
    let _e626: MultiVector = self_423;
    let _e630: MultiVector = other_353;
    let _e642: MultiVector = self_423;
    let _e644: MultiVector = other_353;
    return MultiVector(((((((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.z) * _e15.g0_.zzxz) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0))) + ((vec4<f32>(_e27.g0_.w) * _e31.g0_.wwwx) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))) + ((vec4<f32>(_e43.g2_.x) * _e47.g2_) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0))) + ((vec4<f32>(_e58.g2_.y) * _e62.g2_.yxyy) * vec4<f32>(1.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e73.g2_.z) * _e77.g2_.zzxz) * vec4<f32>(1.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e88.g2_.w) * _e92.g2_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, 1.0))) + ((_e103.g0_.yyxx * _e106.g0_.yxxx) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))), ((((((((((((((((vec4<f32>(_e118.g0_.x) * _e122.g1_) + ((vec4<f32>(_e125.g0_.z) * vec4<f32>(_e129.g1_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e141.g0_.w) * vec4<f32>(_e145.g1_.w)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e157.g1_.x) * vec4<f32>(_e161.g0_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e173.g1_.y) * _e177.g0_.yxyy) * vec4<f32>(1.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e188.g1_.z) * _e192.g0_.zzxz) * vec4<f32>(1.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e203.g1_.w) * _e207.g0_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e218.g2_.x) * vec4<f32>(_e222.g3_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e234.g2_.y) * _e238.g3_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))) + ((vec4<f32>(_e250.g2_.z) * _e254.g3_.zzxz) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0))) + ((vec4<f32>(_e266.g2_.w) * _e270.g3_.wwwx) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))) - (vec4<f32>(_e282.g3_.x) * _e286.g2_)) + ((vec4<f32>(_e290.g3_.y) * vec4<f32>(_e294.g2_.y)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e306.g3_.z) * vec4<f32>(_e310.g2_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e322.g3_.w) * vec4<f32>(_e326.g2_.w)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((_e338.g0_.yxxx * _e341.g1_.yxxx) * vec4<f32>(1.0, 0.0, 0.0, 0.0))), ((((((((vec4<f32>(_e352.g0_.x) * _e356.g2_) + ((vec4<f32>(_e359.g0_.z) * _e363.g2_.wwxy) * vec4<f32>(0.0, -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e376.g0_.w) * _e380.g2_.zzyx) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e393.g2_.x) * _e397.g0_) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0)))) + ((vec4<f32>(_e410.g2_.y) * _e414.g0_.xxwz) * vec4<f32>(0.0, 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e426.g2_.z) * _e430.g0_.wwxy) * vec4<f32>(0.0, -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e442.g2_.w) * _e446.g0_.zzyx) * vec4<f32>(0.0, 1.0, -(1.0), 1.0))) + ((_e458.g0_.xyyy * _e461.g2_.xxwz) * vec4<f32>(0.0, -(1.0), 1.0, -(1.0)))), ((((((((((((vec4<f32>(_e474.g0_.x) * _e478.g3_) + ((vec4<f32>(_e481.g1_.y) * _e485.g2_.wwwz) * vec4<f32>(0.0, 0.0, -(1.0), 1.0))) + ((vec4<f32>(_e497.g1_.z) * _e501.g2_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0)))) + ((vec4<f32>(_e513.g1_.w) * _e517.g2_.zzyz) * vec4<f32>(0.0, -(1.0), 1.0, 0.0))) + ((vec4<f32>(_e529.g2_.y) * _e533.g1_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e545.g2_.z) * _e549.g1_.wwwy) * vec4<f32>(0.0, -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e561.g2_.w) * _e565.g1_.zzyz) * vec4<f32>(0.0, 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e577.g3_.x) * _e581.g0_) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0)))) + ((vec4<f32>(_e594.g3_.y) * vec4<f32>(_e598.g0_.x)) * vec4<f32>(0.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e610.g3_.z) * vec4<f32>(_e614.g0_.x)) * vec4<f32>(0.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e626.g3_.w) * vec4<f32>(_e630.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((_e642.g0_ * vec4<f32>(_e644.g3_.x)) * vec4<f32>(0.0, -(1.0), -(1.0), -(1.0)))));
}

fn multi_vector_multi_vector_geometric_anti_product(self_424: MultiVector, other_354: MultiVector) -> MultiVector {
    var self_425: MultiVector;
    var other_355: MultiVector;

    self_425 = self_424;
    other_355 = other_354;
    let _e4: MultiVector = self_425;
    let _e8: MultiVector = other_355;
    let _e20: MultiVector = self_425;
    let _e24: MultiVector = other_355;
    let _e36: MultiVector = self_425;
    let _e40: MultiVector = other_355;
    let _e52: MultiVector = self_425;
    let _e56: MultiVector = other_355;
    let _e68: MultiVector = self_425;
    let _e72: MultiVector = other_355;
    let _e85: MultiVector = self_425;
    let _e89: MultiVector = other_355;
    let _e103: MultiVector = self_425;
    let _e107: MultiVector = other_355;
    let _e121: MultiVector = self_425;
    let _e125: MultiVector = other_355;
    let _e139: MultiVector = self_425;
    let _e143: MultiVector = other_355;
    let _e154: MultiVector = self_425;
    let _e158: MultiVector = other_355;
    let _e170: MultiVector = self_425;
    let _e174: MultiVector = other_355;
    let _e186: MultiVector = self_425;
    let _e190: MultiVector = other_355;
    let _e202: MultiVector = self_425;
    let _e206: MultiVector = other_355;
    let _e210: MultiVector = self_425;
    let _e214: MultiVector = other_355;
    let _e227: MultiVector = self_425;
    let _e231: MultiVector = other_355;
    let _e244: MultiVector = self_425;
    let _e248: MultiVector = other_355;
    let _e261: MultiVector = self_425;
    let _e265: MultiVector = other_355;
    let _e268: MultiVector = self_425;
    let _e272: MultiVector = other_355;
    let _e285: MultiVector = self_425;
    let _e289: MultiVector = other_355;
    let _e302: MultiVector = self_425;
    let _e306: MultiVector = other_355;
    let _e319: MultiVector = self_425;
    let _e323: MultiVector = other_355;
    let _e327: MultiVector = self_425;
    let _e331: MultiVector = other_355;
    let _e344: MultiVector = self_425;
    let _e348: MultiVector = other_355;
    let _e361: MultiVector = self_425;
    let _e365: MultiVector = other_355;
    let _e378: MultiVector = self_425;
    let _e382: MultiVector = other_355;
    let _e385: MultiVector = self_425;
    let _e389: MultiVector = other_355;
    let _e402: MultiVector = self_425;
    let _e406: MultiVector = other_355;
    let _e419: MultiVector = self_425;
    let _e423: MultiVector = other_355;
    let _e436: MultiVector = self_425;
    let _e440: MultiVector = other_355;
    let _e451: MultiVector = self_425;
    let _e455: MultiVector = other_355;
    let _e469: MultiVector = self_425;
    let _e473: MultiVector = other_355;
    let _e487: MultiVector = self_425;
    let _e491: MultiVector = other_355;
    let _e505: MultiVector = self_425;
    let _e509: MultiVector = other_355;
    let _e513: MultiVector = self_425;
    let _e517: MultiVector = other_355;
    let _e530: MultiVector = self_425;
    let _e534: MultiVector = other_355;
    let _e547: MultiVector = self_425;
    let _e551: MultiVector = other_355;
    let _e564: MultiVector = self_425;
    let _e568: MultiVector = other_355;
    let _e572: MultiVector = self_425;
    let _e576: MultiVector = other_355;
    let _e589: MultiVector = self_425;
    let _e593: MultiVector = other_355;
    let _e606: MultiVector = self_425;
    let _e610: MultiVector = other_355;
    let _e625: MultiVector = self_425;
    let _e629: MultiVector = other_355;
    let _e633: MultiVector = self_425;
    let _e637: MultiVector = other_355;
    let _e650: MultiVector = self_425;
    let _e654: MultiVector = other_355;
    let _e667: MultiVector = self_425;
    let _e671: MultiVector = other_355;
    let _e684: MultiVector = self_425;
    let _e688: MultiVector = other_355;
    let _e692: MultiVector = self_425;
    let _e696: MultiVector = other_355;
    let _e709: MultiVector = self_425;
    let _e713: MultiVector = other_355;
    let _e726: MultiVector = self_425;
    let _e730: MultiVector = other_355;
    return MultiVector((((((((((((((((((vec4<f32>(_e4.g0_.x) * _e8.g3_) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))) + ((vec4<f32>(_e20.g0_.y) * _e24.g3_.yxwz) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e36.g0_.z) * _e40.g3_.zwxy) * vec4<f32>(1.0, -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e52.g0_.w) * _e56.g3_.wzyx) * vec4<f32>(1.0, 1.0, -(1.0), 1.0))) + ((vec4<f32>(_e68.g1_.x) * _e72.g2_) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0)))) + ((vec4<f32>(_e85.g1_.y) * _e89.g2_.yxwz) * vec4<f32>(-(1.0), -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e103.g1_.z) * _e107.g2_.zwxy) * vec4<f32>(-(1.0), 1.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e121.g1_.w) * _e125.g2_.wzyx) * vec4<f32>(-(1.0), -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e139.g2_.x) * _e143.g1_) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0))) + ((vec4<f32>(_e154.g2_.y) * _e158.g1_.yxwz) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e170.g2_.z) * _e174.g1_.zwxy) * vec4<f32>(1.0, -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e186.g2_.w) * _e190.g1_.wzyx) * vec4<f32>(1.0, 1.0, -(1.0), 1.0))) + (vec4<f32>(_e202.g3_.x) * _e206.g0_)) + ((vec4<f32>(_e210.g3_.y) * _e214.g0_.yxwz) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e227.g3_.z) * _e231.g0_.zwxy) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e244.g3_.w) * _e248.g0_.wzyx) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))), ((((((((vec4<f32>(_e261.g1_.x) * _e265.g3_) + ((vec4<f32>(_e268.g1_.y) * _e272.g3_.yxwz) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e285.g1_.z) * _e289.g3_.zwxy) * vec4<f32>(-(1.0), -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e302.g1_.w) * _e306.g3_.wzyx) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))) + (vec4<f32>(_e319.g3_.x) * _e323.g1_)) + ((vec4<f32>(_e327.g3_.y) * _e331.g1_.yxwz) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e344.g3_.z) * _e348.g1_.zwxy) * vec4<f32>(-(1.0), -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e361.g3_.w) * _e365.g1_.wzyx) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))), ((((((((((((((((vec4<f32>(_e378.g0_.x) * _e382.g1_) + ((vec4<f32>(_e385.g0_.y) * _e389.g1_.yxwz) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e402.g0_.z) * _e406.g1_.zwxy) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e419.g0_.w) * _e423.g1_.wzyx) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e436.g1_.x) * _e440.g0_) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0))) + ((vec4<f32>(_e451.g1_.y) * _e455.g0_.yxwz) * vec4<f32>(-(1.0), -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e469.g1_.z) * _e473.g0_.zwxy) * vec4<f32>(-(1.0), -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e487.g1_.w) * _e491.g0_.wzyx) * vec4<f32>(-(1.0), 1.0, -(1.0), -(1.0)))) + (vec4<f32>(_e505.g2_.x) * _e509.g3_)) + ((vec4<f32>(_e513.g2_.y) * _e517.g3_.yxwz) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e530.g2_.z) * _e534.g3_.zwxy) * vec4<f32>(-(1.0), -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e547.g2_.w) * _e551.g3_.wzyx) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))) + (vec4<f32>(_e564.g3_.x) * _e568.g2_)) + ((vec4<f32>(_e572.g3_.y) * _e576.g2_.yxwz) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e589.g3_.z) * _e593.g2_.zwxy) * vec4<f32>(-(1.0), -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e606.g3_.w) * _e610.g2_.wzyx) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))), ((((((((vec4<f32>(0.0) - (vec4<f32>(_e625.g1_.x) * _e629.g1_)) + ((vec4<f32>(_e633.g1_.y) * _e637.g1_.yxwz) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e650.g1_.z) * _e654.g1_.zwxy) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e667.g1_.w) * _e671.g1_.wzyx) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))) + (vec4<f32>(_e684.g3_.x) * _e688.g3_)) + ((vec4<f32>(_e692.g3_.y) * _e696.g3_.yxwz) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e709.g3_.z) * _e713.g3_.zwxy) * vec4<f32>(-(1.0), -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e726.g3_.w) * _e730.g3_.wzyx) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))));
}

fn multi_vector_multi_vector_inner_anti_product(self_426: MultiVector, other_356: MultiVector) -> MultiVector {
    var self_427: MultiVector;
    var other_357: MultiVector;

    self_427 = self_426;
    other_357 = other_356;
    let _e4: MultiVector = self_427;
    let _e8: MultiVector = other_357;
    let _e20: MultiVector = self_427;
    let _e24: MultiVector = other_357;
    let _e36: MultiVector = self_427;
    let _e40: MultiVector = other_357;
    let _e52: MultiVector = self_427;
    let _e56: MultiVector = other_357;
    let _e68: MultiVector = self_427;
    let _e72: MultiVector = other_357;
    let _e84: MultiVector = self_427;
    let _e88: MultiVector = other_357;
    let _e100: MultiVector = self_427;
    let _e104: MultiVector = other_357;
    let _e116: MultiVector = self_427;
    let _e120: MultiVector = other_357;
    let _e124: MultiVector = self_427;
    let _e128: MultiVector = other_357;
    let _e141: MultiVector = self_427;
    let _e145: MultiVector = other_357;
    let _e158: MultiVector = self_427;
    let _e162: MultiVector = other_357;
    let _e175: MultiVector = self_427;
    let _e177: MultiVector = other_357;
    let _e189: MultiVector = self_427;
    let _e193: MultiVector = other_357;
    let _e196: MultiVector = self_427;
    let _e200: MultiVector = other_357;
    let _e212: MultiVector = self_427;
    let _e216: MultiVector = other_357;
    let _e228: MultiVector = self_427;
    let _e232: MultiVector = other_357;
    let _e236: MultiVector = self_427;
    let _e240: MultiVector = other_357;
    let _e252: MultiVector = self_427;
    let _e256: MultiVector = other_357;
    let _e268: MultiVector = self_427;
    let _e272: MultiVector = other_357;
    let _e284: MultiVector = self_427;
    let _e287: MultiVector = other_357;
    let _e299: MultiVector = self_427;
    let _e303: MultiVector = other_357;
    let _e306: MultiVector = self_427;
    let _e310: MultiVector = other_357;
    let _e322: MultiVector = self_427;
    let _e326: MultiVector = other_357;
    let _e338: MultiVector = self_427;
    let _e342: MultiVector = other_357;
    let _e355: MultiVector = self_427;
    let _e359: MultiVector = other_357;
    let _e372: MultiVector = self_427;
    let _e376: MultiVector = other_357;
    let _e389: MultiVector = self_427;
    let _e393: MultiVector = other_357;
    let _e406: MultiVector = self_427;
    let _e410: MultiVector = other_357;
    let _e422: MultiVector = self_427;
    let _e426: MultiVector = other_357;
    let _e438: MultiVector = self_427;
    let _e442: MultiVector = other_357;
    let _e454: MultiVector = self_427;
    let _e458: MultiVector = other_357;
    let _e470: MultiVector = self_427;
    let _e474: MultiVector = other_357;
    let _e478: MultiVector = self_427;
    let _e482: MultiVector = other_357;
    let _e495: MultiVector = self_427;
    let _e499: MultiVector = other_357;
    let _e512: MultiVector = self_427;
    let _e516: MultiVector = other_357;
    let _e529: MultiVector = self_427;
    let _e532: MultiVector = other_357;
    let _e545: MultiVector = self_427;
    let _e549: MultiVector = other_357;
    let _e553: MultiVector = self_427;
    let _e557: MultiVector = other_357;
    let _e569: MultiVector = self_427;
    let _e573: MultiVector = other_357;
    let _e585: MultiVector = self_427;
    let _e589: MultiVector = other_357;
    let _e593: MultiVector = self_427;
    let _e597: MultiVector = other_357;
    let _e609: MultiVector = self_427;
    let _e613: MultiVector = other_357;
    let _e625: MultiVector = self_427;
    let _e629: MultiVector = other_357;
    let _e641: MultiVector = self_427;
    let _e644: MultiVector = other_357;
    return MultiVector((((((((((((((vec4<f32>(_e4.g0_.x) * _e8.g3_) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))) + ((vec4<f32>(_e20.g1_.y) * _e24.g2_.wwwz) * vec4<f32>(0.0, 0.0, -(1.0), 1.0))) + ((vec4<f32>(_e36.g1_.z) * _e40.g2_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0)))) + ((vec4<f32>(_e52.g1_.w) * _e56.g2_.zzyz) * vec4<f32>(0.0, -(1.0), 1.0, 0.0))) + ((vec4<f32>(_e68.g2_.y) * _e72.g1_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e84.g2_.z) * _e88.g1_.wwwy) * vec4<f32>(0.0, -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e100.g2_.w) * _e104.g1_.zzyz) * vec4<f32>(0.0, 1.0, -(1.0), 0.0))) + (vec4<f32>(_e116.g3_.x) * _e120.g0_)) + ((vec4<f32>(_e124.g3_.y) * vec4<f32>(_e128.g0_.x)) * vec4<f32>(0.0, -(1.0), 0.0, 0.0))) + ((vec4<f32>(_e141.g3_.z) * vec4<f32>(_e145.g0_.x)) * vec4<f32>(0.0, 0.0, -(1.0), 0.0))) + ((vec4<f32>(_e158.g3_.w) * vec4<f32>(_e162.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((_e175.g0_ * vec4<f32>(_e177.g3_.x)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))), ((((((((vec4<f32>(_e189.g1_.x) * _e193.g3_) + ((vec4<f32>(_e196.g1_.z) * _e200.g3_.wwxy) * vec4<f32>(0.0, -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e212.g1_.w) * _e216.g3_.zzyx) * vec4<f32>(0.0, 1.0, -(1.0), 1.0))) + (vec4<f32>(_e228.g3_.x) * _e232.g1_)) + ((vec4<f32>(_e236.g3_.y) * _e240.g1_.xxwz) * vec4<f32>(0.0, 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e252.g3_.z) * _e256.g1_.wwxy) * vec4<f32>(0.0, -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e268.g3_.w) * _e272.g1_.zzyx) * vec4<f32>(0.0, 1.0, -(1.0), 1.0))) + ((_e284.g1_.xyyy * _e287.g3_.xxwz) * vec4<f32>(0.0, 1.0, 1.0, -(1.0)))), ((((((((((((((((vec4<f32>(_e299.g0_.x) * _e303.g1_) + ((vec4<f32>(_e306.g0_.z) * vec4<f32>(_e310.g1_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e322.g0_.w) * vec4<f32>(_e326.g1_.w)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e338.g1_.x) * vec4<f32>(_e342.g0_.x)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((vec4<f32>(_e355.g1_.y) * _e359.g0_.yxyy) * vec4<f32>(-(1.0), -(1.0), 0.0, 0.0))) + ((vec4<f32>(_e372.g1_.z) * _e376.g0_.zzxz) * vec4<f32>(-(1.0), 0.0, -(1.0), 0.0))) + ((vec4<f32>(_e389.g1_.w) * _e393.g0_.wwwx) * vec4<f32>(-(1.0), 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e406.g2_.x) * vec4<f32>(_e410.g3_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e422.g2_.y) * _e426.g3_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))) + ((vec4<f32>(_e438.g2_.z) * _e442.g3_.zzxz) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0))) + ((vec4<f32>(_e454.g2_.w) * _e458.g3_.wwwx) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))) + (vec4<f32>(_e470.g3_.x) * _e474.g2_)) + ((vec4<f32>(_e478.g3_.y) * vec4<f32>(_e482.g2_.y)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((vec4<f32>(_e495.g3_.z) * vec4<f32>(_e499.g2_.z)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((vec4<f32>(_e512.g3_.w) * vec4<f32>(_e516.g2_.w)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((_e529.g0_.yxxx * _e532.g1_.yxxx) * vec4<f32>(1.0, 0.0, 0.0, 0.0))), ((((((((vec4<f32>(0.0) - (vec4<f32>(_e545.g1_.x) * _e549.g1_)) + ((vec4<f32>(_e553.g1_.z) * _e557.g1_.zzxz) * vec4<f32>(1.0, 0.0, -(1.0), 0.0))) + ((vec4<f32>(_e569.g1_.w) * _e573.g1_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))) + (vec4<f32>(_e585.g3_.x) * _e589.g3_)) + ((vec4<f32>(_e593.g3_.y) * _e597.g3_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))) + ((vec4<f32>(_e609.g3_.z) * _e613.g3_.zzxz) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0))) + ((vec4<f32>(_e625.g3_.w) * _e629.g3_.wwwx) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))) + ((_e641.g1_.yyxx * _e644.g1_.yxxx) * vec4<f32>(1.0, -(1.0), 0.0, 0.0))));
}

fn multi_vector_multi_vector_left_contraction(self_428: MultiVector, other_358: MultiVector) -> MultiVector {
    var self_429: MultiVector;
    var other_359: MultiVector;

    self_429 = self_428;
    other_359 = other_358;
    let _e4: MultiVector = self_429;
    let _e8: MultiVector = other_359;
    let _e11: MultiVector = self_429;
    let _e15: MultiVector = other_359;
    let _e28: MultiVector = self_429;
    let _e32: MultiVector = other_359;
    let _e45: MultiVector = self_429;
    let _e49: MultiVector = other_359;
    let _e62: MultiVector = self_429;
    let _e66: MultiVector = other_359;
    let _e77: MultiVector = self_429;
    let _e81: MultiVector = other_359;
    let _e92: MultiVector = self_429;
    let _e96: MultiVector = other_359;
    let _e107: MultiVector = self_429;
    let _e110: MultiVector = other_359;
    let _e122: MultiVector = self_429;
    let _e126: MultiVector = other_359;
    let _e129: MultiVector = self_429;
    let _e133: MultiVector = other_359;
    let _e145: MultiVector = self_429;
    let _e149: MultiVector = other_359;
    let _e161: MultiVector = self_429;
    let _e165: MultiVector = other_359;
    let _e177: MultiVector = self_429;
    let _e181: MultiVector = other_359;
    let _e193: MultiVector = self_429;
    let _e197: MultiVector = other_359;
    let _e209: MultiVector = self_429;
    let _e213: MultiVector = other_359;
    let _e225: MultiVector = self_429;
    let _e228: MultiVector = other_359;
    let _e239: MultiVector = self_429;
    let _e243: MultiVector = other_359;
    let _e246: MultiVector = self_429;
    let _e250: MultiVector = other_359;
    let _e262: MultiVector = self_429;
    let _e266: MultiVector = other_359;
    let _e278: MultiVector = self_429;
    let _e282: MultiVector = other_359;
    let _e294: MultiVector = self_429;
    let _e296: MultiVector = other_359;
    let _e311: MultiVector = self_429;
    let _e315: MultiVector = other_359;
    let _e318: MultiVector = self_429;
    let _e322: MultiVector = other_359;
    let _e334: MultiVector = self_429;
    let _e338: MultiVector = other_359;
    let _e350: MultiVector = self_429;
    let _e354: MultiVector = other_359;
    let _e366: MultiVector = self_429;
    let _e368: MultiVector = other_359;
    return MultiVector(((((((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.z) * vec4<f32>(_e15.g0_.z)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((vec4<f32>(_e28.g0_.w) * vec4<f32>(_e32.g0_.w)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((vec4<f32>(_e45.g2_.x) * vec4<f32>(_e49.g2_.x)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((vec4<f32>(_e62.g2_.y) * _e66.g2_.yxyy) * vec4<f32>(1.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e77.g2_.z) * _e81.g2_.zzxz) * vec4<f32>(1.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e92.g2_.w) * _e96.g2_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, 1.0))) + ((_e107.g0_.yxxx * _e110.g0_.yxxx) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))), ((((((((vec4<f32>(_e122.g0_.x) * _e126.g1_) + ((vec4<f32>(_e129.g0_.z) * vec4<f32>(_e133.g1_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e145.g0_.w) * vec4<f32>(_e149.g1_.w)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e161.g2_.x) * vec4<f32>(_e165.g3_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e177.g2_.y) * _e181.g3_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))) + ((vec4<f32>(_e193.g2_.z) * _e197.g3_.zzxz) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0))) + ((vec4<f32>(_e209.g2_.w) * _e213.g3_.wwwx) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))) + ((_e225.g0_.yxxx * _e228.g1_.yxxx) * vec4<f32>(1.0, 0.0, 0.0, 0.0))), (((((vec4<f32>(_e239.g0_.x) * _e243.g2_) + ((vec4<f32>(_e246.g2_.y) * _e250.g0_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e262.g2_.z) * _e266.g0_.wwwy) * vec4<f32>(0.0, -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e278.g2_.w) * _e282.g0_.zzyz) * vec4<f32>(0.0, 1.0, -(1.0), 0.0))) + ((_e294.g0_ * vec4<f32>(_e296.g2_.x)) * vec4<f32>(0.0, -(1.0), -(1.0), -(1.0)))), (((((vec4<f32>(_e311.g0_.x) * _e315.g3_) + ((vec4<f32>(_e318.g2_.y) * _e322.g1_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e334.g2_.z) * _e338.g1_.wwwy) * vec4<f32>(0.0, -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e350.g2_.w) * _e354.g1_.zzyz) * vec4<f32>(0.0, 1.0, -(1.0), 0.0))) + ((_e366.g0_ * vec4<f32>(_e368.g3_.x)) * vec4<f32>(0.0, -(1.0), -(1.0), -(1.0)))));
}

fn multi_vector_multi_vector_right_contraction(self_430: MultiVector, other_360: MultiVector) -> MultiVector {
    var self_431: MultiVector;
    var other_361: MultiVector;

    self_431 = self_430;
    other_361 = other_360;
    let _e4: MultiVector = self_431;
    let _e8: MultiVector = other_361;
    let _e19: MultiVector = self_431;
    let _e23: MultiVector = other_361;
    let _e35: MultiVector = self_431;
    let _e39: MultiVector = other_361;
    let _e51: MultiVector = self_431;
    let _e55: MultiVector = other_361;
    let _e66: MultiVector = self_431;
    let _e70: MultiVector = other_361;
    let _e82: MultiVector = self_431;
    let _e86: MultiVector = other_361;
    let _e98: MultiVector = self_431;
    let _e102: MultiVector = other_361;
    let _e114: MultiVector = self_431;
    let _e118: MultiVector = other_361;
    let _e130: MultiVector = self_431;
    let _e134: MultiVector = other_361;
    let _e144: MultiVector = self_431;
    let _e148: MultiVector = other_361;
    let _e159: MultiVector = self_431;
    let _e163: MultiVector = other_361;
    let _e174: MultiVector = self_431;
    let _e178: MultiVector = other_361;
    let _e182: MultiVector = self_431;
    let _e186: MultiVector = other_361;
    let _e198: MultiVector = self_431;
    let _e202: MultiVector = other_361;
    let _e214: MultiVector = self_431;
    let _e218: MultiVector = other_361;
    let _e230: MultiVector = self_431;
    let _e234: MultiVector = other_361;
    let _e246: MultiVector = self_431;
    let _e250: MultiVector = other_361;
    let _e261: MultiVector = self_431;
    let _e265: MultiVector = other_361;
    let _e277: MultiVector = self_431;
    let _e281: MultiVector = other_361;
    let _e294: MultiVector = self_431;
    let _e298: MultiVector = other_361;
    let _e310: MultiVector = self_431;
    let _e314: MultiVector = other_361;
    let _e326: MultiVector = self_431;
    let _e329: MultiVector = self_431;
    let _e332: MultiVector = self_431;
    let _e335: MultiVector = self_431;
    let _e339: MultiVector = other_361;
    let _e342: MultiVector = other_361;
    let _e345: MultiVector = other_361;
    let _e348: MultiVector = other_361;
    let _e361: MultiVector = self_431;
    let _e365: MultiVector = other_361;
    let _e376: MultiVector = self_431;
    let _e380: MultiVector = other_361;
    let _e392: MultiVector = self_431;
    let _e396: MultiVector = other_361;
    let _e409: MultiVector = self_431;
    let _e413: MultiVector = other_361;
    let _e425: MultiVector = self_431;
    let _e429: MultiVector = other_361;
    let _e441: MultiVector = self_431;
    let _e444: MultiVector = self_431;
    let _e447: MultiVector = self_431;
    let _e450: MultiVector = self_431;
    let _e454: MultiVector = other_361;
    let _e457: MultiVector = other_361;
    let _e460: MultiVector = other_361;
    let _e463: MultiVector = other_361;
    return MultiVector((((((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0)) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.zzxz) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0))) + ((vec4<f32>(_e35.g0_.w) * _e39.g0_.wwwx) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))) + ((vec4<f32>(_e51.g2_.x) * _e55.g2_) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0))) + ((vec4<f32>(_e66.g2_.y) * vec4<f32>(_e70.g2_.y)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e82.g2_.z) * vec4<f32>(_e86.g2_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e98.g2_.w) * vec4<f32>(_e102.g2_.w)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e114.g0_.x) * vec4<f32>(_e118.g0_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))), (((((((((vec4<f32>(_e130.g1_.y) * _e134.g0_.yxyy) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) + ((vec4<f32>(_e144.g1_.z) * _e148.g0_.zzxz) * vec4<f32>(1.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e159.g1_.w) * _e163.g0_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, 1.0))) - (vec4<f32>(_e174.g3_.x) * _e178.g2_)) + ((vec4<f32>(_e182.g3_.y) * vec4<f32>(_e186.g2_.y)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e198.g3_.z) * vec4<f32>(_e202.g2_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e214.g3_.w) * vec4<f32>(_e218.g2_.w)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e230.g1_.x) * vec4<f32>(_e234.g0_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))), (((((((vec4<f32>(_e246.g0_.z) * _e250.g2_.wwwy) * vec4<f32>(0.0, -(1.0), 0.0, 1.0)) + ((vec4<f32>(_e261.g0_.w) * _e265.g2_.zzyz) * vec4<f32>(0.0, 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e277.g2_.x) * _e281.g0_) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0)))) + ((vec4<f32>(_e294.g2_.z) * vec4<f32>(_e298.g0_.x)) * vec4<f32>(0.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e310.g2_.w) * vec4<f32>(_e314.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e326.g0_.x, _e329.g2_.y, _e332.g0_.y, _e335.g0_.y) * vec4<f32>(_e339.g2_.x, _e342.g0_.x, _e345.g2_.w, _e348.g2_.z)) * vec4<f32>(0.0, 1.0, 1.0, -(1.0)))), (((((((vec4<f32>(_e361.g1_.z) * _e365.g2_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e376.g1_.w) * _e380.g2_.zzyz) * vec4<f32>(0.0, -(1.0), 1.0, 0.0))) + ((vec4<f32>(_e392.g3_.x) * _e396.g0_) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0)))) + ((vec4<f32>(_e409.g3_.z) * vec4<f32>(_e413.g0_.x)) * vec4<f32>(0.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e425.g3_.w) * vec4<f32>(_e429.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e441.g1_.x, _e444.g3_.y, _e447.g1_.y, _e450.g1_.y) * vec4<f32>(_e454.g2_.x, _e457.g0_.x, _e460.g2_.w, _e463.g2_.z)) * vec4<f32>(0.0, 1.0, -(1.0), 1.0))));
}

fn multi_vector_multi_vector_left_anti_contraction(self_432: MultiVector, other_362: MultiVector) -> MultiVector {
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
    let _e43: MultiVector = self_433;
    let _e47: MultiVector = other_363;
    let _e60: MultiVector = self_433;
    let _e64: MultiVector = other_363;
    let _e77: MultiVector = self_433;
    let _e80: MultiVector = self_433;
    let _e83: MultiVector = self_433;
    let _e86: MultiVector = self_433;
    let _e90: MultiVector = other_363;
    let _e93: MultiVector = other_363;
    let _e96: MultiVector = other_363;
    let _e99: MultiVector = other_363;
    let _e113: MultiVector = self_433;
    let _e117: MultiVector = other_363;
    let _e128: MultiVector = self_433;
    let _e132: MultiVector = other_363;
    let _e144: MultiVector = self_433;
    let _e148: MultiVector = other_363;
    let _e152: MultiVector = self_433;
    let _e156: MultiVector = other_363;
    let _e168: MultiVector = self_433;
    let _e172: MultiVector = other_363;
    let _e184: MultiVector = self_433;
    let _e187: MultiVector = self_433;
    let _e190: MultiVector = self_433;
    let _e193: MultiVector = self_433;
    let _e197: MultiVector = other_363;
    let _e200: MultiVector = other_363;
    let _e203: MultiVector = other_363;
    let _e206: MultiVector = other_363;
    let _e219: MultiVector = self_433;
    let _e223: MultiVector = other_363;
    let _e235: MultiVector = self_433;
    let _e239: MultiVector = other_363;
    let _e252: MultiVector = self_433;
    let _e256: MultiVector = other_363;
    let _e269: MultiVector = self_433;
    let _e273: MultiVector = other_363;
    let _e277: MultiVector = self_433;
    let _e281: MultiVector = other_363;
    let _e294: MultiVector = self_433;
    let _e298: MultiVector = other_363;
    let _e311: MultiVector = self_433;
    let _e315: MultiVector = other_363;
    let _e328: MultiVector = self_433;
    let _e332: MultiVector = other_363;
    let _e345: MultiVector = self_433;
    let _e349: MultiVector = other_363;
    let _e360: MultiVector = self_433;
    let _e364: MultiVector = other_363;
    let _e376: MultiVector = self_433;
    let _e380: MultiVector = other_363;
    let _e392: MultiVector = self_433;
    let _e396: MultiVector = other_363;
    let _e400: MultiVector = self_433;
    let _e404: MultiVector = other_363;
    let _e417: MultiVector = self_433;
    let _e421: MultiVector = other_363;
    let _e434: MultiVector = self_433;
    let _e438: MultiVector = other_363;
    let _e451: MultiVector = self_433;
    let _e455: MultiVector = other_363;
    return MultiVector((((((((vec4<f32>(_e4.g1_.z) * _e8.g2_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e19.g1_.w) * _e23.g2_.zzyz) * vec4<f32>(0.0, -(1.0), 1.0, 0.0))) + (vec4<f32>(_e35.g3_.x) * _e39.g0_)) + ((vec4<f32>(_e43.g3_.z) * vec4<f32>(_e47.g0_.x)) * vec4<f32>(0.0, 0.0, -(1.0), 0.0))) + ((vec4<f32>(_e60.g3_.w) * vec4<f32>(_e64.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e77.g1_.x, _e80.g3_.y, _e83.g1_.y, _e86.g1_.y) * vec4<f32>(_e90.g2_.x, _e93.g0_.x, _e96.g2_.w, _e99.g2_.z)) * vec4<f32>(0.0, -(1.0), -(1.0), 1.0))), (((((((vec4<f32>(_e113.g1_.z) * _e117.g3_.wwwy) * vec4<f32>(0.0, -(1.0), 0.0, 1.0)) + ((vec4<f32>(_e128.g1_.w) * _e132.g3_.zzyz) * vec4<f32>(0.0, 1.0, -(1.0), 0.0))) + (vec4<f32>(_e144.g3_.x) * _e148.g1_)) + ((vec4<f32>(_e152.g3_.z) * vec4<f32>(_e156.g1_.x)) * vec4<f32>(0.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e168.g3_.w) * vec4<f32>(_e172.g1_.x)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e184.g1_.x, _e187.g3_.y, _e190.g1_.y, _e193.g1_.y) * vec4<f32>(_e197.g3_.x, _e200.g1_.x, _e203.g3_.w, _e206.g3_.z)) * vec4<f32>(0.0, 1.0, 1.0, -(1.0)))), (((((((((vec4<f32>(_e219.g1_.y) * _e223.g0_.yxyy) * vec4<f32>(-(1.0), -(1.0), 0.0, 0.0)) + ((vec4<f32>(_e235.g1_.z) * _e239.g0_.zzxz) * vec4<f32>(-(1.0), 0.0, -(1.0), 0.0))) + ((vec4<f32>(_e252.g1_.w) * _e256.g0_.wwwx) * vec4<f32>(-(1.0), 0.0, 0.0, -(1.0)))) + (vec4<f32>(_e269.g3_.x) * _e273.g2_)) + ((vec4<f32>(_e277.g3_.y) * vec4<f32>(_e281.g2_.y)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((vec4<f32>(_e294.g3_.z) * vec4<f32>(_e298.g2_.z)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((vec4<f32>(_e311.g3_.w) * vec4<f32>(_e315.g2_.w)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((vec4<f32>(_e328.g1_.x) * vec4<f32>(_e332.g0_.x)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))), (((((((((vec4<f32>(_e345.g1_.y) * _e349.g1_.yxyy) * vec4<f32>(1.0, -(1.0), 0.0, 0.0)) + ((vec4<f32>(_e360.g1_.z) * _e364.g1_.zzxz) * vec4<f32>(1.0, 0.0, -(1.0), 0.0))) + ((vec4<f32>(_e376.g1_.w) * _e380.g1_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))) + (vec4<f32>(_e392.g3_.x) * _e396.g3_)) + ((vec4<f32>(_e400.g3_.y) * vec4<f32>(_e404.g3_.y)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((vec4<f32>(_e417.g3_.z) * vec4<f32>(_e421.g3_.z)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((vec4<f32>(_e434.g3_.w) * vec4<f32>(_e438.g3_.w)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((vec4<f32>(_e451.g1_.x) * vec4<f32>(_e455.g1_.x)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))));
}

fn multi_vector_multi_vector_right_anti_contraction(self_434: MultiVector, other_364: MultiVector) -> MultiVector {
    var self_435: MultiVector;
    var other_365: MultiVector;

    self_435 = self_434;
    other_365 = other_364;
    let _e4: MultiVector = self_435;
    let _e8: MultiVector = other_365;
    let _e20: MultiVector = self_435;
    let _e24: MultiVector = other_365;
    let _e36: MultiVector = self_435;
    let _e40: MultiVector = other_365;
    let _e52: MultiVector = self_435;
    let _e56: MultiVector = other_365;
    let _e68: MultiVector = self_435;
    let _e70: MultiVector = other_365;
    let _e82: MultiVector = self_435;
    let _e86: MultiVector = other_365;
    let _e89: MultiVector = self_435;
    let _e93: MultiVector = other_365;
    let _e105: MultiVector = self_435;
    let _e109: MultiVector = other_365;
    let _e121: MultiVector = self_435;
    let _e125: MultiVector = other_365;
    let _e137: MultiVector = self_435;
    let _e139: MultiVector = other_365;
    let _e151: MultiVector = self_435;
    let _e155: MultiVector = other_365;
    let _e158: MultiVector = self_435;
    let _e162: MultiVector = other_365;
    let _e174: MultiVector = self_435;
    let _e178: MultiVector = other_365;
    let _e190: MultiVector = self_435;
    let _e194: MultiVector = other_365;
    let _e206: MultiVector = self_435;
    let _e210: MultiVector = other_365;
    let _e222: MultiVector = self_435;
    let _e226: MultiVector = other_365;
    let _e238: MultiVector = self_435;
    let _e242: MultiVector = other_365;
    let _e254: MultiVector = self_435;
    let _e257: MultiVector = other_365;
    let _e270: MultiVector = self_435;
    let _e274: MultiVector = other_365;
    let _e278: MultiVector = self_435;
    let _e282: MultiVector = other_365;
    let _e294: MultiVector = self_435;
    let _e298: MultiVector = other_365;
    let _e310: MultiVector = self_435;
    let _e314: MultiVector = other_365;
    let _e326: MultiVector = self_435;
    let _e330: MultiVector = other_365;
    let _e342: MultiVector = self_435;
    let _e346: MultiVector = other_365;
    let _e358: MultiVector = self_435;
    let _e362: MultiVector = other_365;
    let _e374: MultiVector = self_435;
    let _e377: MultiVector = other_365;
    return MultiVector(((((((vec4<f32>(_e4.g0_.x) * _e8.g3_) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))) + ((vec4<f32>(_e20.g2_.y) * _e24.g1_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e36.g2_.z) * _e40.g1_.wwwy) * vec4<f32>(0.0, -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e52.g2_.w) * _e56.g1_.zzyz) * vec4<f32>(0.0, 1.0, -(1.0), 0.0))) + ((_e68.g0_ * vec4<f32>(_e70.g3_.x)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))), (((((vec4<f32>(_e82.g1_.x) * _e86.g3_) + ((vec4<f32>(_e89.g3_.y) * _e93.g1_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e105.g3_.z) * _e109.g1_.wwwy) * vec4<f32>(0.0, -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e121.g3_.w) * _e125.g1_.zzyz) * vec4<f32>(0.0, 1.0, -(1.0), 0.0))) + ((_e137.g1_ * vec4<f32>(_e139.g3_.x)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))), ((((((((vec4<f32>(_e151.g0_.x) * _e155.g1_) + ((vec4<f32>(_e158.g0_.z) * vec4<f32>(_e162.g1_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e174.g0_.w) * vec4<f32>(_e178.g1_.w)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e190.g2_.x) * vec4<f32>(_e194.g3_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e206.g2_.y) * _e210.g3_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))) + ((vec4<f32>(_e222.g2_.z) * _e226.g3_.zzxz) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0))) + ((vec4<f32>(_e238.g2_.w) * _e242.g3_.wwwx) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))) + ((_e254.g0_.yxxx * _e257.g1_.yxxx) * vec4<f32>(1.0, 0.0, 0.0, 0.0))), ((((((((vec4<f32>(0.0) - (vec4<f32>(_e270.g1_.x) * _e274.g1_)) + ((vec4<f32>(_e278.g1_.z) * vec4<f32>(_e282.g1_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e294.g1_.w) * vec4<f32>(_e298.g1_.w)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e310.g3_.x) * vec4<f32>(_e314.g3_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e326.g3_.y) * _e330.g3_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))) + ((vec4<f32>(_e342.g3_.z) * _e346.g3_.zzxz) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0))) + ((vec4<f32>(_e358.g3_.w) * _e362.g3_.wwwx) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))) + ((_e374.g1_.yxxx * _e377.g1_.yxxx) * vec4<f32>(1.0, 0.0, 0.0, 0.0))));
}

fn multi_vector_multi_vector_scalar_product(self_436: MultiVector, other_366: MultiVector) -> Scalar {
    var self_437: MultiVector;
    var other_367: MultiVector;

    self_437 = self_436;
    other_367 = other_366;
    let _e4: MultiVector = self_437;
    let _e7: MultiVector = other_367;
    let _e11: MultiVector = self_437;
    let _e14: MultiVector = other_367;
    let _e19: MultiVector = self_437;
    let _e22: MultiVector = other_367;
    let _e27: MultiVector = self_437;
    let _e30: MultiVector = other_367;
    let _e35: MultiVector = self_437;
    let _e38: MultiVector = other_367;
    let _e43: MultiVector = self_437;
    let _e46: MultiVector = other_367;
    let _e51: MultiVector = self_437;
    let _e54: MultiVector = other_367;
    let _e59: MultiVector = self_437;
    let _e62: MultiVector = other_367;
    return Scalar(((((((((_e4.g0_.x * _e7.g0_.x) - (_e11.g0_.y * _e14.g0_.y)) - (_e19.g0_.z * _e22.g0_.z)) - (_e27.g0_.w * _e30.g0_.w)) - (_e35.g2_.x * _e38.g2_.x)) + (_e43.g2_.y * _e46.g2_.y)) + (_e51.g2_.z * _e54.g2_.z)) + (_e59.g2_.w * _e62.g2_.w)));
}

fn multi_vector_multi_vector_anti_scalar_product(self_438: MultiVector, other_368: MultiVector) -> AntiScalar {
    var self_439: MultiVector;
    var other_369: MultiVector;

    self_439 = self_438;
    other_369 = other_368;
    let _e5: MultiVector = self_439;
    let _e8: MultiVector = other_369;
    let _e13: MultiVector = self_439;
    let _e16: MultiVector = other_369;
    let _e21: MultiVector = self_439;
    let _e24: MultiVector = other_369;
    let _e29: MultiVector = self_439;
    let _e32: MultiVector = other_369;
    let _e37: MultiVector = self_439;
    let _e40: MultiVector = other_369;
    let _e45: MultiVector = self_439;
    let _e48: MultiVector = other_369;
    let _e53: MultiVector = self_439;
    let _e56: MultiVector = other_369;
    let _e61: MultiVector = self_439;
    let _e64: MultiVector = other_369;
    return AntiScalar(((((((((0.0 - (_e5.g1_.x * _e8.g1_.x)) + (_e13.g1_.y * _e16.g1_.y)) + (_e21.g1_.z * _e24.g1_.z)) + (_e29.g1_.w * _e32.g1_.w)) + (_e37.g3_.x * _e40.g3_.x)) - (_e45.g3_.y * _e48.g3_.y)) - (_e53.g3_.z * _e56.g3_.z)) - (_e61.g3_.w * _e64.g3_.w)));
}

fn multi_vector_rotor_into(self_440: MultiVector) -> Rotor {
    var self_441: MultiVector;

    self_441 = self_440;
    let _e2: MultiVector = self_441;
    return Rotor(_e2.g0_);
}

fn multi_vector_rotor_add(self_442: MultiVector, other_370: Rotor) -> MultiVector {
    var self_443: MultiVector;
    var other_371: Rotor;

    self_443 = self_442;
    other_371 = other_370;
    let _e4: MultiVector = self_443;
    let _e6: Rotor = other_371;
    let _e9: MultiVector = self_443;
    let _e11: MultiVector = self_443;
    let _e13: MultiVector = self_443;
    return MultiVector((_e4.g0_ + _e6.g0_), _e9.g1_, _e11.g2_, _e13.g3_);
}

fn multi_vector_rotor_sub(self_444: MultiVector, other_372: Rotor) -> MultiVector {
    var self_445: MultiVector;
    var other_373: Rotor;

    self_445 = self_444;
    other_373 = other_372;
    let _e4: MultiVector = self_445;
    let _e6: Rotor = other_373;
    let _e9: MultiVector = self_445;
    let _e11: MultiVector = self_445;
    let _e13: MultiVector = self_445;
    return MultiVector((_e4.g0_ - _e6.g0_), _e9.g1_, _e11.g2_, _e13.g3_);
}

fn multi_vector_rotor_geometric_product(self_446: MultiVector, other_374: Rotor) -> MultiVector {
    var self_447: MultiVector;
    var other_375: Rotor;

    self_447 = self_446;
    other_375 = other_374;
    let _e4: MultiVector = self_447;
    let _e8: Rotor = other_375;
    let _e11: MultiVector = self_447;
    let _e15: Rotor = other_375;
    let _e28: MultiVector = self_447;
    let _e32: Rotor = other_375;
    let _e45: MultiVector = self_447;
    let _e49: Rotor = other_375;
    let _e62: MultiVector = self_447;
    let _e66: Rotor = other_375;
    let _e78: MultiVector = self_447;
    let _e82: Rotor = other_375;
    let _e94: MultiVector = self_447;
    let _e98: Rotor = other_375;
    let _e110: MultiVector = self_447;
    let _e114: Rotor = other_375;
    let _e126: MultiVector = self_447;
    let _e130: Rotor = other_375;
    let _e142: MultiVector = self_447;
    let _e146: Rotor = other_375;
    let _e158: MultiVector = self_447;
    let _e162: Rotor = other_375;
    let _e174: MultiVector = self_447;
    let _e178: Rotor = other_375;
    let _e190: MultiVector = self_447;
    let _e194: Rotor = other_375;
    let _e206: MultiVector = self_447;
    let _e210: Rotor = other_375;
    let _e222: MultiVector = self_447;
    let _e226: Rotor = other_375;
    let _e238: MultiVector = self_447;
    let _e242: Rotor = other_375;
    return MultiVector(((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.y) * _e15.g0_.yxwz) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e28.g0_.z) * _e32.g0_.zwxy) * vec4<f32>(-(1.0), -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e45.g0_.w) * _e49.g0_.wzyx) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))), (((((vec4<f32>(_e62.g1_.x) * _e66.g0_) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))) + ((vec4<f32>(_e78.g1_.y) * _e82.g0_.yxwz) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e94.g1_.z) * _e98.g0_.zwxy) * vec4<f32>(1.0, -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e110.g1_.w) * _e114.g0_.wzyx) * vec4<f32>(1.0, 1.0, -(1.0), 1.0))), (((((vec4<f32>(_e126.g2_.x) * _e130.g0_) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))) + ((vec4<f32>(_e142.g2_.y) * _e146.g0_.yxwz) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e158.g2_.z) * _e162.g0_.zwxy) * vec4<f32>(1.0, -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e174.g2_.w) * _e178.g0_.wzyx) * vec4<f32>(1.0, 1.0, -(1.0), 1.0))), (((((vec4<f32>(_e190.g3_.x) * _e194.g0_) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))) + ((vec4<f32>(_e206.g3_.y) * _e210.g0_.yxwz) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e222.g3_.z) * _e226.g0_.zwxy) * vec4<f32>(1.0, -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e238.g3_.w) * _e242.g0_.wzyx) * vec4<f32>(1.0, 1.0, -(1.0), 1.0))));
}

fn multi_vector_rotor_outer_product(self_448: MultiVector, other_376: Rotor) -> MultiVector {
    var self_449: MultiVector;
    var other_377: Rotor;

    self_449 = self_448;
    other_377 = other_376;
    let _e4: MultiVector = self_449;
    let _e8: Rotor = other_377;
    let _e11: MultiVector = self_449;
    let _e13: Rotor = other_377;
    let _e25: MultiVector = self_449;
    let _e29: Rotor = other_377;
    let _e41: MultiVector = self_449;
    let _e43: Rotor = other_377;
    let _e55: MultiVector = self_449;
    let _e59: Rotor = other_377;
    let _e69: MultiVector = self_449;
    let _e73: Rotor = other_377;
    let _e84: MultiVector = self_449;
    let _e88: Rotor = other_377;
    let _e99: MultiVector = self_449;
    let _e103: Rotor = other_377;
    let _e115: MultiVector = self_449;
    let _e119: Rotor = other_377;
    let _e129: MultiVector = self_449;
    let _e133: Rotor = other_377;
    let _e144: MultiVector = self_449;
    let _e148: Rotor = other_377;
    let _e159: MultiVector = self_449;
    let _e163: Rotor = other_377;
    return MultiVector(((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((_e11.g0_ * vec4<f32>(_e13.g0_.x)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))), (((vec4<f32>(_e25.g1_.x) * _e29.g0_) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))) + ((_e41.g1_ * vec4<f32>(_e43.g0_.x)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))), (((((vec4<f32>(_e55.g2_.y) * _e59.g0_.yxyy) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) + ((vec4<f32>(_e69.g2_.z) * _e73.g0_.zzxz) * vec4<f32>(1.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e84.g2_.w) * _e88.g0_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e99.g2_.x) * vec4<f32>(_e103.g0_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))), (((((vec4<f32>(_e115.g3_.y) * _e119.g0_.yxyy) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) + ((vec4<f32>(_e129.g3_.z) * _e133.g0_.zzxz) * vec4<f32>(1.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e144.g3_.w) * _e148.g0_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e159.g3_.x) * vec4<f32>(_e163.g0_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))));
}

fn multi_vector_rotor_inner_product(self_450: MultiVector, other_378: Rotor) -> MultiVector {
    var self_451: MultiVector;
    var other_379: Rotor;

    self_451 = self_450;
    other_379 = other_378;
    let _e4: MultiVector = self_451;
    let _e8: Rotor = other_379;
    let _e11: MultiVector = self_451;
    let _e15: Rotor = other_379;
    let _e27: MultiVector = self_451;
    let _e31: Rotor = other_379;
    let _e43: MultiVector = self_451;
    let _e46: Rotor = other_379;
    let _e58: MultiVector = self_451;
    let _e62: Rotor = other_379;
    let _e72: MultiVector = self_451;
    let _e76: Rotor = other_379;
    let _e87: MultiVector = self_451;
    let _e91: Rotor = other_379;
    let _e102: MultiVector = self_451;
    let _e106: Rotor = other_379;
    let _e118: MultiVector = self_451;
    let _e122: Rotor = other_379;
    let _e134: MultiVector = self_451;
    let _e138: Rotor = other_379;
    let _e150: MultiVector = self_451;
    let _e154: Rotor = other_379;
    let _e166: MultiVector = self_451;
    let _e169: Rotor = other_379;
    let _e181: MultiVector = self_451;
    let _e185: Rotor = other_379;
    let _e197: MultiVector = self_451;
    let _e199: Rotor = other_379;
    return MultiVector(((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.z) * _e15.g0_.zzxz) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0))) + ((vec4<f32>(_e27.g0_.w) * _e31.g0_.wwwx) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))) + ((_e43.g0_.yyxx * _e46.g0_.yxxx) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))), (((((vec4<f32>(_e58.g1_.y) * _e62.g0_.yxyy) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) + ((vec4<f32>(_e72.g1_.z) * _e76.g0_.zzxz) * vec4<f32>(1.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e87.g1_.w) * _e91.g0_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e102.g1_.x) * vec4<f32>(_e106.g0_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))), (((((vec4<f32>(_e118.g2_.x) * _e122.g0_) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))) + ((vec4<f32>(_e134.g2_.z) * _e138.g0_.wwxy) * vec4<f32>(0.0, -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e150.g2_.w) * _e154.g0_.zzyx) * vec4<f32>(0.0, 1.0, -(1.0), 1.0))) + ((_e166.g2_.xyyy * _e169.g0_.xxwz) * vec4<f32>(0.0, 1.0, 1.0, -(1.0)))), (((vec4<f32>(_e181.g3_.x) * _e185.g0_) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))) + ((_e197.g3_ * vec4<f32>(_e199.g0_.x)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn multi_vector_rotor_right_contraction(self_452: MultiVector, other_380: Rotor) -> MultiVector {
    var self_453: MultiVector;
    var other_381: Rotor;

    self_453 = self_452;
    other_381 = other_380;
    let _e4: MultiVector = self_453;
    let _e8: Rotor = other_381;
    let _e19: MultiVector = self_453;
    let _e23: Rotor = other_381;
    let _e35: MultiVector = self_453;
    let _e39: Rotor = other_381;
    let _e51: MultiVector = self_453;
    let _e55: Rotor = other_381;
    let _e67: MultiVector = self_453;
    let _e71: Rotor = other_381;
    let _e81: MultiVector = self_453;
    let _e85: Rotor = other_381;
    let _e96: MultiVector = self_453;
    let _e100: Rotor = other_381;
    let _e111: MultiVector = self_453;
    let _e115: Rotor = other_381;
    let _e127: MultiVector = self_453;
    let _e131: Rotor = other_381;
    let _e143: MultiVector = self_453;
    let _e145: Rotor = other_381;
    let _e157: MultiVector = self_453;
    let _e161: Rotor = other_381;
    let _e173: MultiVector = self_453;
    let _e175: Rotor = other_381;
    return MultiVector((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0)) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.zzxz) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0))) + ((vec4<f32>(_e35.g0_.w) * _e39.g0_.wwwx) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))) + ((vec4<f32>(_e51.g0_.x) * vec4<f32>(_e55.g0_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))), (((((vec4<f32>(_e67.g1_.y) * _e71.g0_.yxyy) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) + ((vec4<f32>(_e81.g1_.z) * _e85.g0_.zzxz) * vec4<f32>(1.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e96.g1_.w) * _e100.g0_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e111.g1_.x) * vec4<f32>(_e115.g0_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))), (((vec4<f32>(_e127.g2_.x) * _e131.g0_) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))) + ((_e143.g2_ * vec4<f32>(_e145.g0_.x)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))), (((vec4<f32>(_e157.g3_.x) * _e161.g0_) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))) + ((_e173.g3_ * vec4<f32>(_e175.g0_.x)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn multi_vector_rotor_scalar_product(self_454: MultiVector, other_382: Rotor) -> Scalar {
    var self_455: MultiVector;
    var other_383: Rotor;

    self_455 = self_454;
    other_383 = other_382;
    let _e4: MultiVector = self_455;
    let _e7: Rotor = other_383;
    let _e11: MultiVector = self_455;
    let _e14: Rotor = other_383;
    let _e19: MultiVector = self_455;
    let _e22: Rotor = other_383;
    let _e27: MultiVector = self_455;
    let _e30: Rotor = other_383;
    return Scalar(((((_e4.g0_.x * _e7.g0_.x) - (_e11.g0_.y * _e14.g0_.y)) - (_e19.g0_.z * _e22.g0_.z)) - (_e27.g0_.w * _e30.g0_.w)));
}

fn multi_vector_point_into(self_456: MultiVector) -> Point {
    var self_457: MultiVector;

    self_457 = self_456;
    let _e2: MultiVector = self_457;
    let _e5: MultiVector = self_457;
    let _e8: MultiVector = self_457;
    let _e11: MultiVector = self_457;
    return Point(vec4<f32>(_e2.g2_.x, _e5.g1_.y, _e8.g1_.z, _e11.g1_.w));
}

fn multi_vector_point_add(self_458: MultiVector, other_384: Point) -> MultiVector {
    var self_459: MultiVector;
    var other_385: Point;

    self_459 = self_458;
    other_385 = other_384;
    let _e4: MultiVector = self_459;
    let _e6: MultiVector = self_459;
    let _e8: Point = other_385;
    let _e17: MultiVector = self_459;
    let _e19: Point = other_385;
    let _e30: MultiVector = self_459;
    return MultiVector(_e4.g0_, (_e6.g1_ + (_e8.g0_ * vec4<f32>(0.0, 1.0, 1.0, 1.0))), (_e17.g2_ + (vec4<f32>(_e19.g0_.x) * vec4<f32>(1.0, 0.0, 0.0, 0.0))), _e30.g3_);
}

fn multi_vector_point_sub(self_460: MultiVector, other_386: Point) -> MultiVector {
    var self_461: MultiVector;
    var other_387: Point;

    self_461 = self_460;
    other_387 = other_386;
    let _e4: MultiVector = self_461;
    let _e6: MultiVector = self_461;
    let _e8: Point = other_387;
    let _e17: MultiVector = self_461;
    let _e19: Point = other_387;
    let _e30: MultiVector = self_461;
    return MultiVector(_e4.g0_, (_e6.g1_ - (_e8.g0_ * vec4<f32>(0.0, 1.0, 1.0, 1.0))), (_e17.g2_ - (vec4<f32>(_e19.g0_.x) * vec4<f32>(1.0, 0.0, 0.0, 0.0))), _e30.g3_);
}

fn multi_vector_point_geometric_product(self_462: MultiVector, other_388: Point) -> MultiVector {
    var self_463: MultiVector;
    var other_389: Point;

    self_463 = self_462;
    other_389 = other_388;
    let _e4: MultiVector = self_463;
    let _e6: Point = other_389;
    let _e18: MultiVector = self_463;
    let _e22: Point = other_389;
    let _e33: MultiVector = self_463;
    let _e37: Point = other_389;
    let _e49: MultiVector = self_463;
    let _e53: Point = other_389;
    let _e65: MultiVector = self_463;
    let _e69: Point = other_389;
    let _e82: MultiVector = self_463;
    let _e86: Point = other_389;
    let _e99: MultiVector = self_463;
    let _e103: Point = other_389;
    let _e116: MultiVector = self_463;
    let _e119: MultiVector = self_463;
    let _e122: MultiVector = self_463;
    let _e125: MultiVector = self_463;
    let _e129: Point = other_389;
    let _e140: MultiVector = self_463;
    let _e142: Point = other_389;
    let _e156: MultiVector = self_463;
    let _e160: Point = other_389;
    let _e173: MultiVector = self_463;
    let _e177: Point = other_389;
    let _e189: MultiVector = self_463;
    let _e193: Point = other_389;
    let _e205: MultiVector = self_463;
    let _e209: Point = other_389;
    let _e221: MultiVector = self_463;
    let _e223: Point = other_389;
    return MultiVector(((_e4.g2_ * vec4<f32>(_e6.g0_.x)) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0)), ((((((((vec4<f32>(_e18.g0_.y) * _e22.g0_.yywz) * vec4<f32>(1.0, 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e33.g0_.z) * _e37.g0_.zwzy) * vec4<f32>(1.0, -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e49.g0_.w) * _e53.g0_.wzyw) * vec4<f32>(1.0, 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e65.g3_.y) * vec4<f32>(_e69.g0_.x)) * vec4<f32>(0.0, -(1.0), 0.0, 0.0))) + ((vec4<f32>(_e82.g3_.z) * vec4<f32>(_e86.g0_.x)) * vec4<f32>(0.0, 0.0, -(1.0), 0.0))) + ((vec4<f32>(_e99.g3_.w) * vec4<f32>(_e103.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e116.g3_.x, _e119.g0_.x, _e122.g0_.x, _e125.g0_.x) * _e129.g0_) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0))), ((_e140.g0_ * vec4<f32>(_e142.g0_.x)) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))), ((((((vec4<f32>(_e156.g2_.x) * _e160.g0_.yyzw) * vec4<f32>(0.0, -(1.0), -(1.0), -(1.0))) + ((vec4<f32>(_e173.g2_.y) * _e177.g0_.yywz) * vec4<f32>(1.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e189.g2_.z) * _e193.g0_.zwzy) * vec4<f32>(1.0, -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e205.g2_.w) * _e209.g0_.wzyw) * vec4<f32>(1.0, 1.0, -(1.0), 0.0))) + (_e221.g1_ * vec4<f32>(_e223.g0_.x))));
}

fn multi_vector_point_geometric_anti_product(self_464: MultiVector, other_390: Point) -> MultiVector {
    var self_465: MultiVector;
    var other_391: Point;

    self_465 = self_464;
    other_391 = other_390;
    let _e4: MultiVector = self_465;
    let _e8: Point = other_391;
    let _e18: MultiVector = self_465;
    let _e22: Point = other_391;
    let _e34: MultiVector = self_465;
    let _e38: Point = other_391;
    let _e50: MultiVector = self_465;
    let _e54: Point = other_391;
    let _e66: MultiVector = self_465;
    let _e68: Point = other_391;
    let _e83: MultiVector = self_465;
    let _e87: Point = other_391;
    let _e99: MultiVector = self_465;
    let _e103: Point = other_391;
    let _e116: MultiVector = self_465;
    let _e120: Point = other_391;
    let _e133: MultiVector = self_465;
    let _e137: Point = other_391;
    let _e147: MultiVector = self_465;
    let _e151: Point = other_391;
    let _e162: MultiVector = self_465;
    let _e166: Point = other_391;
    let _e178: MultiVector = self_465;
    let _e182: Point = other_391;
    let _e194: MultiVector = self_465;
    let _e198: Point = other_391;
    let _e210: MultiVector = self_465;
    let _e214: Point = other_391;
    let _e226: MultiVector = self_465;
    let _e230: Point = other_391;
    let _e242: MultiVector = self_465;
    let _e245: MultiVector = self_465;
    let _e248: MultiVector = self_465;
    let _e251: MultiVector = self_465;
    let _e255: Point = other_391;
    let _e259: MultiVector = self_465;
    let _e263: Point = other_391;
    let _e274: MultiVector = self_465;
    let _e278: Point = other_391;
    let _e290: MultiVector = self_465;
    let _e294: Point = other_391;
    let _e306: MultiVector = self_465;
    let _e310: Point = other_391;
    return MultiVector(((((((vec4<f32>(_e4.g2_.x) * _e8.g0_.yyzw) * vec4<f32>(0.0, 1.0, 1.0, 1.0)) + ((vec4<f32>(_e18.g2_.y) * _e22.g0_.yywz) * vec4<f32>(1.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e34.g2_.z) * _e38.g0_.zwzy) * vec4<f32>(1.0, -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e50.g2_.w) * _e54.g0_.wzyw) * vec4<f32>(1.0, 1.0, -(1.0), 0.0))) + ((_e66.g1_ * vec4<f32>(_e68.g0_.x)) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0)))), (((((vec4<f32>(_e83.g3_.y) * _e87.g0_.yywz) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e99.g3_.z) * _e103.g0_.zwzy) * vec4<f32>(-(1.0), -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e116.g3_.w) * _e120.g0_.wzyw) * vec4<f32>(-(1.0), 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e133.g3_.x) * _e137.g0_) * vec4<f32>(0.0, 1.0, 1.0, 1.0))), ((((((((vec4<f32>(_e147.g0_.y) * _e151.g0_.yywz) * vec4<f32>(1.0, 0.0, -(1.0), 1.0)) + ((vec4<f32>(_e162.g0_.z) * _e166.g0_.zwzy) * vec4<f32>(1.0, 1.0, 0.0, -(1.0)))) + ((vec4<f32>(_e178.g0_.w) * _e182.g0_.wzyw) * vec4<f32>(1.0, -(1.0), 1.0, 0.0))) + ((vec4<f32>(_e194.g3_.y) * vec4<f32>(_e198.g0_.x)) * vec4<f32>(0.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e210.g3_.z) * vec4<f32>(_e214.g0_.x)) * vec4<f32>(0.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e226.g3_.w) * vec4<f32>(_e230.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + (vec4<f32>(_e242.g3_.x, _e245.g0_.x, _e248.g0_.x, _e251.g0_.x) * _e255.g0_)), (((((vec4<f32>(_e259.g1_.y) * _e263.g0_.yywz) * vec4<f32>(1.0, 0.0, -(1.0), 1.0)) + ((vec4<f32>(_e274.g1_.z) * _e278.g0_.zwzy) * vec4<f32>(1.0, 1.0, 0.0, -(1.0)))) + ((vec4<f32>(_e290.g1_.w) * _e294.g0_.wzyw) * vec4<f32>(1.0, -(1.0), 1.0, 0.0))) + ((vec4<f32>(_e306.g1_.x) * _e310.g0_) * vec4<f32>(0.0, -(1.0), -(1.0), -(1.0)))));
}

fn multi_vector_point_scalar_product(self_466: MultiVector, other_392: Point) -> Scalar {
    var self_467: MultiVector;
    var other_393: Point;

    self_467 = self_466;
    other_393 = other_392;
    let _e5: MultiVector = self_467;
    let _e8: Point = other_393;
    return Scalar((0.0 - (_e5.g2_.x * _e8.g0_.x)));
}

fn multi_vector_point_anti_scalar_product(self_468: MultiVector, other_394: Point) -> AntiScalar {
    var self_469: MultiVector;
    var other_395: Point;

    self_469 = self_468;
    other_395 = other_394;
    let _e4: MultiVector = self_469;
    let _e7: Point = other_395;
    let _e11: MultiVector = self_469;
    let _e14: Point = other_395;
    let _e19: MultiVector = self_469;
    let _e22: Point = other_395;
    return AntiScalar((((_e4.g1_.y * _e7.g0_.y) + (_e11.g1_.z * _e14.g0_.z)) + (_e19.g1_.w * _e22.g0_.w)));
}

fn multi_vector_ideal_point_into(self_470: MultiVector) -> IdealPoint {
    var self_471: MultiVector;

    self_471 = self_470;
    let _e2: MultiVector = self_471;
    let _e5: MultiVector = self_471;
    let _e8: MultiVector = self_471;
    return IdealPoint(vec3<f32>(_e2.g3_.y, _e5.g3_.z, _e8.g3_.w));
}

fn multi_vector_ideal_point_add(self_472: MultiVector, other_396: IdealPoint) -> MultiVector {
    var self_473: MultiVector;
    var other_397: IdealPoint;

    self_473 = self_472;
    other_397 = other_396;
    let _e4: MultiVector = self_473;
    let _e6: MultiVector = self_473;
    let _e8: MultiVector = self_473;
    let _e10: MultiVector = self_473;
    let _e12: IdealPoint = other_397;
    let _e15: IdealPoint = other_397;
    let _e18: IdealPoint = other_397;
    let _e21: IdealPoint = other_397;
    return MultiVector(_e4.g0_, _e6.g1_, _e8.g2_, (_e10.g3_ + (vec4<f32>(_e12.g0_.x, _e15.g0_.x, _e18.g0_.y, _e21.g0_.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn multi_vector_ideal_point_sub(self_474: MultiVector, other_398: IdealPoint) -> MultiVector {
    var self_475: MultiVector;
    var other_399: IdealPoint;

    self_475 = self_474;
    other_399 = other_398;
    let _e4: MultiVector = self_475;
    let _e6: MultiVector = self_475;
    let _e8: MultiVector = self_475;
    let _e10: MultiVector = self_475;
    let _e12: IdealPoint = other_399;
    let _e15: IdealPoint = other_399;
    let _e18: IdealPoint = other_399;
    let _e21: IdealPoint = other_399;
    return MultiVector(_e4.g0_, _e6.g1_, _e8.g2_, (_e10.g3_ - (vec4<f32>(_e12.g0_.x, _e15.g0_.x, _e18.g0_.y, _e21.g0_.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn multi_vector_ideal_point_geometric_anti_product(self_476: MultiVector, other_400: IdealPoint) -> MultiVector {
    var self_477: MultiVector;
    var other_401: IdealPoint;

    self_477 = self_476;
    other_401 = other_400;
    let _e4: MultiVector = self_477;
    let _e8: IdealPoint = other_401;
    let _e11: IdealPoint = other_401;
    let _e14: IdealPoint = other_401;
    let _e17: IdealPoint = other_401;
    let _e29: MultiVector = self_477;
    let _e33: IdealPoint = other_401;
    let _e36: IdealPoint = other_401;
    let _e39: IdealPoint = other_401;
    let _e42: IdealPoint = other_401;
    let _e55: MultiVector = self_477;
    let _e59: IdealPoint = other_401;
    let _e62: IdealPoint = other_401;
    let _e65: IdealPoint = other_401;
    let _e68: IdealPoint = other_401;
    let _e81: MultiVector = self_477;
    let _e85: IdealPoint = other_401;
    let _e88: IdealPoint = other_401;
    let _e91: IdealPoint = other_401;
    let _e94: IdealPoint = other_401;
    let _e109: MultiVector = self_477;
    let _e113: IdealPoint = other_401;
    let _e116: IdealPoint = other_401;
    let _e119: IdealPoint = other_401;
    let _e122: IdealPoint = other_401;
    let _e135: MultiVector = self_477;
    let _e139: IdealPoint = other_401;
    let _e142: IdealPoint = other_401;
    let _e145: IdealPoint = other_401;
    let _e148: IdealPoint = other_401;
    let _e162: MultiVector = self_477;
    let _e166: IdealPoint = other_401;
    let _e169: IdealPoint = other_401;
    let _e172: IdealPoint = other_401;
    let _e175: IdealPoint = other_401;
    let _e189: MultiVector = self_477;
    let _e193: IdealPoint = other_401;
    let _e196: IdealPoint = other_401;
    let _e199: IdealPoint = other_401;
    let _e202: IdealPoint = other_401;
    let _e214: MultiVector = self_477;
    let _e218: IdealPoint = other_401;
    let _e221: IdealPoint = other_401;
    let _e224: IdealPoint = other_401;
    let _e227: IdealPoint = other_401;
    let _e240: MultiVector = self_477;
    let _e244: IdealPoint = other_401;
    let _e247: IdealPoint = other_401;
    let _e250: IdealPoint = other_401;
    let _e253: IdealPoint = other_401;
    let _e267: MultiVector = self_477;
    let _e271: IdealPoint = other_401;
    let _e274: IdealPoint = other_401;
    let _e277: IdealPoint = other_401;
    let _e280: IdealPoint = other_401;
    let _e294: MultiVector = self_477;
    let _e298: IdealPoint = other_401;
    let _e301: IdealPoint = other_401;
    let _e304: IdealPoint = other_401;
    let _e307: IdealPoint = other_401;
    let _e319: MultiVector = self_477;
    let _e323: IdealPoint = other_401;
    let _e326: IdealPoint = other_401;
    let _e329: IdealPoint = other_401;
    let _e332: IdealPoint = other_401;
    let _e345: MultiVector = self_477;
    let _e349: IdealPoint = other_401;
    let _e352: IdealPoint = other_401;
    let _e355: IdealPoint = other_401;
    let _e358: IdealPoint = other_401;
    let _e372: MultiVector = self_477;
    let _e376: IdealPoint = other_401;
    let _e379: IdealPoint = other_401;
    let _e382: IdealPoint = other_401;
    let _e385: IdealPoint = other_401;
    let _e399: MultiVector = self_477;
    let _e403: IdealPoint = other_401;
    let _e406: IdealPoint = other_401;
    let _e409: IdealPoint = other_401;
    let _e412: IdealPoint = other_401;
    return MultiVector((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.x, _e11.g0_.x, _e14.g0_.z, _e17.g0_.y)) * vec4<f32>(1.0, 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e29.g0_.z) * vec4<f32>(_e33.g0_.y, _e36.g0_.z, _e39.g0_.y, _e42.g0_.x)) * vec4<f32>(1.0, -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e55.g0_.w) * vec4<f32>(_e59.g0_.z, _e62.g0_.y, _e65.g0_.x, _e68.g0_.z)) * vec4<f32>(1.0, 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e81.g0_.x) * vec4<f32>(_e85.g0_.x, _e88.g0_.x, _e91.g0_.y, _e94.g0_.z)) * vec4<f32>(0.0, -(1.0), -(1.0), -(1.0)))), (((((vec4<f32>(_e109.g1_.y) * vec4<f32>(_e113.g0_.x, _e116.g0_.x, _e119.g0_.z, _e122.g0_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e135.g1_.z) * vec4<f32>(_e139.g0_.y, _e142.g0_.z, _e145.g0_.y, _e148.g0_.x)) * vec4<f32>(-(1.0), -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e162.g1_.w) * vec4<f32>(_e166.g0_.z, _e169.g0_.y, _e172.g0_.x, _e175.g0_.z)) * vec4<f32>(-(1.0), 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e189.g1_.x) * vec4<f32>(_e193.g0_.x, _e196.g0_.x, _e199.g0_.y, _e202.g0_.z)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))), (((((vec4<f32>(_e214.g2_.y) * vec4<f32>(_e218.g0_.x, _e221.g0_.x, _e224.g0_.z, _e227.g0_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e240.g2_.z) * vec4<f32>(_e244.g0_.y, _e247.g0_.z, _e250.g0_.y, _e253.g0_.x)) * vec4<f32>(-(1.0), -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e267.g2_.w) * vec4<f32>(_e271.g0_.z, _e274.g0_.y, _e277.g0_.x, _e280.g0_.z)) * vec4<f32>(-(1.0), 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e294.g2_.x) * vec4<f32>(_e298.g0_.x, _e301.g0_.x, _e304.g0_.y, _e307.g0_.z)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))), (((((vec4<f32>(_e319.g3_.y) * vec4<f32>(_e323.g0_.x, _e326.g0_.x, _e329.g0_.z, _e332.g0_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e345.g3_.z) * vec4<f32>(_e349.g0_.y, _e352.g0_.z, _e355.g0_.y, _e358.g0_.x)) * vec4<f32>(-(1.0), -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e372.g3_.w) * vec4<f32>(_e376.g0_.z, _e379.g0_.y, _e382.g0_.x, _e385.g0_.z)) * vec4<f32>(-(1.0), 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e399.g3_.x) * vec4<f32>(_e403.g0_.x, _e406.g0_.x, _e409.g0_.y, _e412.g0_.z)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn multi_vector_ideal_point_anti_scalar_product(self_478: MultiVector, other_402: IdealPoint) -> AntiScalar {
    var self_479: MultiVector;
    var other_403: IdealPoint;

    self_479 = self_478;
    other_403 = other_402;
    let _e5: MultiVector = self_479;
    let _e8: IdealPoint = other_403;
    let _e13: MultiVector = self_479;
    let _e16: IdealPoint = other_403;
    let _e21: MultiVector = self_479;
    let _e24: IdealPoint = other_403;
    return AntiScalar((((0.0 - (_e5.g3_.y * _e8.g0_.x)) - (_e13.g3_.z * _e16.g0_.y)) - (_e21.g3_.w * _e24.g0_.z)));
}

fn multi_vector_plane_into(self_480: MultiVector) -> Plane {
    var self_481: MultiVector;

    self_481 = self_480;
    let _e2: MultiVector = self_481;
    let _e5: MultiVector = self_481;
    let _e8: MultiVector = self_481;
    let _e11: MultiVector = self_481;
    return Plane(vec4<f32>(_e2.g1_.x, _e5.g2_.y, _e8.g2_.z, _e11.g2_.w));
}

fn multi_vector_plane_add(self_482: MultiVector, other_404: Plane) -> MultiVector {
    var self_483: MultiVector;
    var other_405: Plane;

    self_483 = self_482;
    other_405 = other_404;
    let _e4: MultiVector = self_483;
    let _e6: MultiVector = self_483;
    let _e8: Plane = other_405;
    let _e19: MultiVector = self_483;
    let _e21: Plane = other_405;
    let _e30: MultiVector = self_483;
    return MultiVector(_e4.g0_, (_e6.g1_ + (vec4<f32>(_e8.g0_.x) * vec4<f32>(1.0, 0.0, 0.0, 0.0))), (_e19.g2_ + (_e21.g0_ * vec4<f32>(0.0, 1.0, 1.0, 1.0))), _e30.g3_);
}

fn multi_vector_plane_sub(self_484: MultiVector, other_406: Plane) -> MultiVector {
    var self_485: MultiVector;
    var other_407: Plane;

    self_485 = self_484;
    other_407 = other_406;
    let _e4: MultiVector = self_485;
    let _e6: MultiVector = self_485;
    let _e8: Plane = other_407;
    let _e19: MultiVector = self_485;
    let _e21: Plane = other_407;
    let _e30: MultiVector = self_485;
    return MultiVector(_e4.g0_, (_e6.g1_ - (vec4<f32>(_e8.g0_.x) * vec4<f32>(1.0, 0.0, 0.0, 0.0))), (_e19.g2_ - (_e21.g0_ * vec4<f32>(0.0, 1.0, 1.0, 1.0))), _e30.g3_);
}

fn multi_vector_plane_geometric_product(self_486: MultiVector, other_408: Plane) -> MultiVector {
    var self_487: MultiVector;
    var other_409: Plane;

    self_487 = self_486;
    other_409 = other_408;
    let _e4: MultiVector = self_487;
    let _e8: Plane = other_409;
    let _e19: MultiVector = self_487;
    let _e23: Plane = other_409;
    let _e35: MultiVector = self_487;
    let _e39: Plane = other_409;
    let _e51: MultiVector = self_487;
    let _e55: Plane = other_409;
    let _e65: MultiVector = self_487;
    let _e69: Plane = other_409;
    let _e82: MultiVector = self_487;
    let _e86: Plane = other_409;
    let _e98: MultiVector = self_487;
    let _e102: Plane = other_409;
    let _e114: MultiVector = self_487;
    let _e118: Plane = other_409;
    let _e130: MultiVector = self_487;
    let _e132: Plane = other_409;
    let _e147: MultiVector = self_487;
    let _e151: Plane = other_409;
    let _e162: MultiVector = self_487;
    let _e166: Plane = other_409;
    let _e178: MultiVector = self_487;
    let _e182: Plane = other_409;
    let _e194: MultiVector = self_487;
    let _e198: Plane = other_409;
    let _e208: MultiVector = self_487;
    let _e212: Plane = other_409;
    let _e224: MultiVector = self_487;
    let _e228: Plane = other_409;
    let _e241: MultiVector = self_487;
    let _e245: Plane = other_409;
    let _e258: MultiVector = self_487;
    let _e262: Plane = other_409;
    let _e275: MultiVector = self_487;
    let _e279: Plane = other_409;
    let _e292: MultiVector = self_487;
    let _e296: Plane = other_409;
    let _e309: MultiVector = self_487;
    let _e312: MultiVector = self_487;
    let _e315: MultiVector = self_487;
    let _e318: MultiVector = self_487;
    let _e322: Plane = other_409;
    return MultiVector((((((vec4<f32>(_e4.g2_.y) * _e8.g0_.yywz) * vec4<f32>(1.0, 0.0, -(1.0), 1.0)) + ((vec4<f32>(_e19.g2_.z) * _e23.g0_.zwzy) * vec4<f32>(1.0, 1.0, 0.0, -(1.0)))) + ((vec4<f32>(_e35.g2_.w) * _e39.g0_.wzyw) * vec4<f32>(1.0, -(1.0), 1.0, 0.0))) + ((vec4<f32>(_e51.g2_.x) * _e55.g0_) * vec4<f32>(0.0, 1.0, 1.0, 1.0))), ((((((vec4<f32>(_e65.g3_.x) * _e69.g0_.yyzw) * vec4<f32>(0.0, -(1.0), -(1.0), -(1.0))) + ((vec4<f32>(_e82.g3_.y) * _e86.g0_.yywz) * vec4<f32>(1.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e98.g3_.z) * _e102.g0_.zwzy) * vec4<f32>(1.0, -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e114.g3_.w) * _e118.g0_.wzyw) * vec4<f32>(1.0, 1.0, -(1.0), 0.0))) + ((_e130.g0_ * vec4<f32>(_e132.g0_.x)) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0)))), (((((vec4<f32>(_e147.g0_.y) * _e151.g0_.yywz) * vec4<f32>(1.0, 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e162.g0_.z) * _e166.g0_.zwzy) * vec4<f32>(1.0, -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e178.g0_.w) * _e182.g0_.wzyw) * vec4<f32>(1.0, 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e194.g0_.x) * _e198.g0_) * vec4<f32>(0.0, 1.0, 1.0, 1.0))), ((((((((vec4<f32>(_e208.g1_.y) * _e212.g0_.yywz) * vec4<f32>(-(1.0), 0.0, -(1.0), 1.0)) + ((vec4<f32>(_e224.g1_.z) * _e228.g0_.zwzy) * vec4<f32>(-(1.0), 1.0, 0.0, -(1.0)))) + ((vec4<f32>(_e241.g1_.w) * _e245.g0_.wzyw) * vec4<f32>(-(1.0), -(1.0), 1.0, 0.0))) + ((vec4<f32>(_e258.g2_.y) * vec4<f32>(_e262.g0_.x)) * vec4<f32>(0.0, -(1.0), 0.0, 0.0))) + ((vec4<f32>(_e275.g2_.z) * vec4<f32>(_e279.g0_.x)) * vec4<f32>(0.0, 0.0, -(1.0), 0.0))) + ((vec4<f32>(_e292.g2_.w) * vec4<f32>(_e296.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e309.g2_.x, _e312.g1_.x, _e315.g1_.x, _e318.g1_.x) * _e322.g0_) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0))));
}

fn multi_vector_plane_geometric_anti_product(self_488: MultiVector, other_410: Plane) -> MultiVector {
    var self_489: MultiVector;
    var other_411: Plane;

    self_489 = self_488;
    other_411 = other_410;
    let _e4: MultiVector = self_489;
    let _e8: Plane = other_411;
    let _e20: MultiVector = self_489;
    let _e24: Plane = other_411;
    let _e37: MultiVector = self_489;
    let _e41: Plane = other_411;
    let _e54: MultiVector = self_489;
    let _e58: Plane = other_411;
    let _e70: MultiVector = self_489;
    let _e74: Plane = other_411;
    let _e86: MultiVector = self_489;
    let _e90: Plane = other_411;
    let _e102: MultiVector = self_489;
    let _e105: MultiVector = self_489;
    let _e108: MultiVector = self_489;
    let _e111: MultiVector = self_489;
    let _e115: Plane = other_411;
    let _e123: MultiVector = self_489;
    let _e125: Plane = other_411;
    let _e130: MultiVector = self_489;
    let _e134: Plane = other_411;
    let _e144: MultiVector = self_489;
    let _e148: Plane = other_411;
    let _e161: MultiVector = self_489;
    let _e165: Plane = other_411;
    let _e178: MultiVector = self_489;
    let _e182: Plane = other_411;
    let _e195: MultiVector = self_489;
    let _e197: Plane = other_411;
    let _e212: MultiVector = self_489;
    let _e214: Plane = other_411;
    return MultiVector(((((((((vec4<f32>(_e4.g1_.y) * _e8.g0_.yywz) * vec4<f32>(-(1.0), 0.0, -(1.0), 1.0)) + ((vec4<f32>(_e20.g1_.z) * _e24.g0_.zwzy) * vec4<f32>(-(1.0), 1.0, 0.0, -(1.0)))) + ((vec4<f32>(_e37.g1_.w) * _e41.g0_.wzyw) * vec4<f32>(-(1.0), -(1.0), 1.0, 0.0))) + ((vec4<f32>(_e54.g2_.y) * vec4<f32>(_e58.g0_.x)) * vec4<f32>(0.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e70.g2_.z) * vec4<f32>(_e74.g0_.x)) * vec4<f32>(0.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e86.g2_.w) * vec4<f32>(_e90.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e102.g2_.x, _e105.g1_.x, _e108.g1_.x, _e111.g1_.x) * _e115.g0_) * vec4<f32>(-(1.0)))), (_e123.g3_ * vec4<f32>(_e125.g0_.x)), ((((((vec4<f32>(_e130.g3_.x) * _e134.g0_.yyzw) * vec4<f32>(0.0, 1.0, 1.0, 1.0)) + ((vec4<f32>(_e144.g3_.y) * _e148.g0_.yywz) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e161.g3_.z) * _e165.g0_.zwzy) * vec4<f32>(-(1.0), -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e178.g3_.w) * _e182.g0_.wzyw) * vec4<f32>(-(1.0), 1.0, -(1.0), 0.0))) + ((_e195.g0_ * vec4<f32>(_e197.g0_.x)) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0)))), ((_e212.g1_ * vec4<f32>(_e214.g0_.x)) * vec4<f32>(-(1.0))));
}

fn multi_vector_plane_scalar_product(self_490: MultiVector, other_412: Plane) -> Scalar {
    var self_491: MultiVector;
    var other_413: Plane;

    self_491 = self_490;
    other_413 = other_412;
    let _e4: MultiVector = self_491;
    let _e7: Plane = other_413;
    let _e11: MultiVector = self_491;
    let _e14: Plane = other_413;
    let _e19: MultiVector = self_491;
    let _e22: Plane = other_413;
    return Scalar((((_e4.g2_.y * _e7.g0_.y) + (_e11.g2_.z * _e14.g0_.z)) + (_e19.g2_.w * _e22.g0_.w)));
}

fn multi_vector_plane_anti_scalar_product(self_492: MultiVector, other_414: Plane) -> AntiScalar {
    var self_493: MultiVector;
    var other_415: Plane;

    self_493 = self_492;
    other_415 = other_414;
    let _e5: MultiVector = self_493;
    let _e8: Plane = other_415;
    return AntiScalar((0.0 - (_e5.g1_.x * _e8.g0_.x)));
}

fn multi_vector_line_into(self_494: MultiVector) -> Line {
    var self_495: MultiVector;

    self_495 = self_494;
    let _e2: MultiVector = self_495;
    let _e5: MultiVector = self_495;
    let _e8: MultiVector = self_495;
    let _e12: MultiVector = self_495;
    let _e15: MultiVector = self_495;
    let _e18: MultiVector = self_495;
    return Line(vec3<f32>(_e2.g3_.y, _e5.g3_.z, _e8.g3_.w), vec3<f32>(_e12.g0_.y, _e15.g0_.z, _e18.g0_.w));
}

fn multi_vector_line_add(self_496: MultiVector, other_416: Line) -> MultiVector {
    var self_497: MultiVector;
    var other_417: Line;

    self_497 = self_496;
    other_417 = other_416;
    let _e4: MultiVector = self_497;
    let _e6: Line = other_417;
    let _e9: Line = other_417;
    let _e12: Line = other_417;
    let _e15: Line = other_417;
    let _e26: MultiVector = self_497;
    let _e28: MultiVector = self_497;
    let _e30: MultiVector = self_497;
    let _e32: Line = other_417;
    let _e35: Line = other_417;
    let _e38: Line = other_417;
    let _e41: Line = other_417;
    return MultiVector((_e4.g0_ + (vec4<f32>(_e6.g0_.x, _e9.g1_.x, _e12.g1_.y, _e15.g1_.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0))), _e26.g1_, _e28.g2_, (_e30.g3_ + (vec4<f32>(_e32.g0_.x, _e35.g0_.x, _e38.g0_.y, _e41.g0_.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn multi_vector_line_sub(self_498: MultiVector, other_418: Line) -> MultiVector {
    var self_499: MultiVector;
    var other_419: Line;

    self_499 = self_498;
    other_419 = other_418;
    let _e4: MultiVector = self_499;
    let _e6: Line = other_419;
    let _e9: Line = other_419;
    let _e12: Line = other_419;
    let _e15: Line = other_419;
    let _e26: MultiVector = self_499;
    let _e28: MultiVector = self_499;
    let _e30: MultiVector = self_499;
    let _e32: Line = other_419;
    let _e35: Line = other_419;
    let _e38: Line = other_419;
    let _e41: Line = other_419;
    return MultiVector((_e4.g0_ - (vec4<f32>(_e6.g0_.x, _e9.g1_.x, _e12.g1_.y, _e15.g1_.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0))), _e26.g1_, _e28.g2_, (_e30.g3_ - (vec4<f32>(_e32.g0_.x, _e35.g0_.x, _e38.g0_.y, _e41.g0_.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn multi_vector_line_geometric_product(self_500: MultiVector, other_420: Line) -> MultiVector {
    var self_501: MultiVector;
    var other_421: Line;

    self_501 = self_500;
    other_421 = other_420;
    let _e4: MultiVector = self_501;
    let _e8: Line = other_421;
    let _e11: Line = other_421;
    let _e14: Line = other_421;
    let _e17: Line = other_421;
    let _e30: MultiVector = self_501;
    let _e34: Line = other_421;
    let _e37: Line = other_421;
    let _e40: Line = other_421;
    let _e43: Line = other_421;
    let _e57: MultiVector = self_501;
    let _e61: Line = other_421;
    let _e64: Line = other_421;
    let _e67: Line = other_421;
    let _e70: Line = other_421;
    let _e84: MultiVector = self_501;
    let _e88: Line = other_421;
    let _e91: Line = other_421;
    let _e94: Line = other_421;
    let _e97: Line = other_421;
    let _e109: MultiVector = self_501;
    let _e113: Line = other_421;
    let _e116: Line = other_421;
    let _e119: Line = other_421;
    let _e122: Line = other_421;
    let _e134: MultiVector = self_501;
    let _e138: Line = other_421;
    let _e141: Line = other_421;
    let _e144: Line = other_421;
    let _e147: Line = other_421;
    let _e160: MultiVector = self_501;
    let _e164: Line = other_421;
    let _e167: Line = other_421;
    let _e170: Line = other_421;
    let _e173: Line = other_421;
    let _e186: MultiVector = self_501;
    let _e190: Line = other_421;
    let _e193: Line = other_421;
    let _e196: Line = other_421;
    let _e199: Line = other_421;
    let _e211: MultiVector = self_501;
    let _e215: Line = other_421;
    let _e218: Line = other_421;
    let _e221: Line = other_421;
    let _e224: Line = other_421;
    let _e238: MultiVector = self_501;
    let _e242: Line = other_421;
    let _e245: Line = other_421;
    let _e248: Line = other_421;
    let _e251: Line = other_421;
    let _e265: MultiVector = self_501;
    let _e269: Line = other_421;
    let _e272: Line = other_421;
    let _e275: Line = other_421;
    let _e278: Line = other_421;
    let _e292: MultiVector = self_501;
    let _e296: Line = other_421;
    let _e299: Line = other_421;
    let _e302: Line = other_421;
    let _e305: Line = other_421;
    let _e320: MultiVector = self_501;
    let _e324: Line = other_421;
    let _e327: Line = other_421;
    let _e330: Line = other_421;
    let _e333: Line = other_421;
    let _e345: MultiVector = self_501;
    let _e349: Line = other_421;
    let _e352: Line = other_421;
    let _e355: Line = other_421;
    let _e358: Line = other_421;
    let _e371: MultiVector = self_501;
    let _e375: Line = other_421;
    let _e378: Line = other_421;
    let _e381: Line = other_421;
    let _e384: Line = other_421;
    let _e397: MultiVector = self_501;
    let _e401: Line = other_421;
    let _e404: Line = other_421;
    let _e407: Line = other_421;
    let _e410: Line = other_421;
    let _e425: MultiVector = self_501;
    let _e429: Line = other_421;
    let _e432: Line = other_421;
    let _e435: Line = other_421;
    let _e438: Line = other_421;
    let _e450: MultiVector = self_501;
    let _e454: Line = other_421;
    let _e457: Line = other_421;
    let _e460: Line = other_421;
    let _e463: Line = other_421;
    let _e476: MultiVector = self_501;
    let _e480: Line = other_421;
    let _e483: Line = other_421;
    let _e486: Line = other_421;
    let _e489: Line = other_421;
    let _e502: MultiVector = self_501;
    let _e506: Line = other_421;
    let _e509: Line = other_421;
    let _e512: Line = other_421;
    let _e515: Line = other_421;
    let _e530: MultiVector = self_501;
    let _e534: Line = other_421;
    let _e537: Line = other_421;
    let _e540: Line = other_421;
    let _e543: Line = other_421;
    let _e556: MultiVector = self_501;
    let _e560: Line = other_421;
    let _e563: Line = other_421;
    let _e566: Line = other_421;
    let _e569: Line = other_421;
    let _e582: MultiVector = self_501;
    let _e586: Line = other_421;
    let _e589: Line = other_421;
    let _e592: Line = other_421;
    let _e595: Line = other_421;
    let _e608: MultiVector = self_501;
    let _e612: Line = other_421;
    let _e615: Line = other_421;
    let _e618: Line = other_421;
    let _e621: Line = other_421;
    return MultiVector((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g1_.x, _e11.g1_.x, _e14.g1_.z, _e17.g1_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e30.g0_.z) * vec4<f32>(_e34.g1_.y, _e37.g1_.z, _e40.g1_.y, _e43.g1_.x)) * vec4<f32>(-(1.0), -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e57.g0_.w) * vec4<f32>(_e61.g1_.z, _e64.g1_.y, _e67.g1_.x, _e70.g1_.z)) * vec4<f32>(-(1.0), 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e84.g0_.x) * vec4<f32>(_e88.g1_.x, _e91.g1_.x, _e94.g1_.y, _e97.g1_.z)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))), (((((((((vec4<f32>(_e109.g1_.y) * vec4<f32>(_e113.g1_.x, _e116.g1_.x, _e119.g1_.z, _e122.g1_.y)) * vec4<f32>(1.0, 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e134.g1_.z) * vec4<f32>(_e138.g1_.y, _e141.g1_.z, _e144.g1_.y, _e147.g1_.x)) * vec4<f32>(1.0, -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e160.g1_.w) * vec4<f32>(_e164.g1_.z, _e167.g1_.y, _e170.g1_.x, _e173.g1_.z)) * vec4<f32>(1.0, 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e186.g2_.x) * vec4<f32>(_e190.g0_.x, _e193.g0_.x, _e196.g0_.y, _e199.g0_.z)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))) + ((vec4<f32>(_e211.g2_.y) * vec4<f32>(_e215.g0_.x, _e218.g0_.x, _e221.g0_.z, _e224.g0_.y)) * vec4<f32>(-(1.0), 0.0, -(1.0), 1.0))) + ((vec4<f32>(_e238.g2_.z) * vec4<f32>(_e242.g0_.y, _e245.g0_.z, _e248.g0_.y, _e251.g0_.x)) * vec4<f32>(-(1.0), 1.0, 0.0, -(1.0)))) + ((vec4<f32>(_e265.g2_.w) * vec4<f32>(_e269.g0_.z, _e272.g0_.y, _e275.g0_.x, _e278.g0_.z)) * vec4<f32>(-(1.0), -(1.0), 1.0, 0.0))) + ((vec4<f32>(_e292.g1_.x) * vec4<f32>(_e296.g1_.x, _e299.g1_.x, _e302.g1_.y, _e305.g1_.z)) * vec4<f32>(0.0, -(1.0), -(1.0), -(1.0)))), (((((vec4<f32>(_e320.g2_.y) * vec4<f32>(_e324.g1_.x, _e327.g1_.x, _e330.g1_.z, _e333.g1_.y)) * vec4<f32>(1.0, 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e345.g2_.z) * vec4<f32>(_e349.g1_.y, _e352.g1_.z, _e355.g1_.y, _e358.g1_.x)) * vec4<f32>(1.0, -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e371.g2_.w) * vec4<f32>(_e375.g1_.z, _e378.g1_.y, _e381.g1_.x, _e384.g1_.z)) * vec4<f32>(1.0, 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e397.g2_.x) * vec4<f32>(_e401.g1_.x, _e404.g1_.x, _e407.g1_.y, _e410.g1_.z)) * vec4<f32>(0.0, -(1.0), -(1.0), -(1.0)))), (((((((((vec4<f32>(_e425.g0_.y) * vec4<f32>(_e429.g0_.x, _e432.g0_.x, _e435.g0_.z, _e438.g0_.y)) * vec4<f32>(1.0, 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e450.g0_.z) * vec4<f32>(_e454.g0_.y, _e457.g0_.z, _e460.g0_.y, _e463.g0_.x)) * vec4<f32>(1.0, -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e476.g0_.w) * vec4<f32>(_e480.g0_.z, _e483.g0_.y, _e486.g0_.x, _e489.g0_.z)) * vec4<f32>(1.0, 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e502.g3_.x) * vec4<f32>(_e506.g1_.x, _e509.g1_.x, _e512.g1_.y, _e515.g1_.z)) * vec4<f32>(0.0, -(1.0), -(1.0), -(1.0)))) + ((vec4<f32>(_e530.g3_.y) * vec4<f32>(_e534.g1_.x, _e537.g1_.x, _e540.g1_.z, _e543.g1_.y)) * vec4<f32>(1.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e556.g3_.z) * vec4<f32>(_e560.g1_.y, _e563.g1_.z, _e566.g1_.y, _e569.g1_.x)) * vec4<f32>(1.0, -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e582.g3_.w) * vec4<f32>(_e586.g1_.z, _e589.g1_.y, _e592.g1_.x, _e595.g1_.z)) * vec4<f32>(1.0, 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e608.g0_.x) * vec4<f32>(_e612.g0_.x, _e615.g0_.x, _e618.g0_.y, _e621.g0_.z)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn multi_vector_line_geometric_anti_product(self_502: MultiVector, other_422: Line) -> MultiVector {
    var self_503: MultiVector;
    var other_423: Line;

    self_503 = self_502;
    other_423 = other_422;
    let _e4: MultiVector = self_503;
    let _e8: Line = other_423;
    let _e11: Line = other_423;
    let _e14: Line = other_423;
    let _e17: Line = other_423;
    let _e29: MultiVector = self_503;
    let _e33: Line = other_423;
    let _e36: Line = other_423;
    let _e39: Line = other_423;
    let _e42: Line = other_423;
    let _e55: MultiVector = self_503;
    let _e59: Line = other_423;
    let _e62: Line = other_423;
    let _e65: Line = other_423;
    let _e68: Line = other_423;
    let _e81: MultiVector = self_503;
    let _e85: Line = other_423;
    let _e88: Line = other_423;
    let _e91: Line = other_423;
    let _e94: Line = other_423;
    let _e106: MultiVector = self_503;
    let _e110: Line = other_423;
    let _e113: Line = other_423;
    let _e116: Line = other_423;
    let _e119: Line = other_423;
    let _e132: MultiVector = self_503;
    let _e136: Line = other_423;
    let _e139: Line = other_423;
    let _e142: Line = other_423;
    let _e145: Line = other_423;
    let _e158: MultiVector = self_503;
    let _e162: Line = other_423;
    let _e165: Line = other_423;
    let _e168: Line = other_423;
    let _e171: Line = other_423;
    let _e184: MultiVector = self_503;
    let _e188: Line = other_423;
    let _e191: Line = other_423;
    let _e194: Line = other_423;
    let _e197: Line = other_423;
    let _e212: MultiVector = self_503;
    let _e216: Line = other_423;
    let _e219: Line = other_423;
    let _e222: Line = other_423;
    let _e225: Line = other_423;
    let _e238: MultiVector = self_503;
    let _e242: Line = other_423;
    let _e245: Line = other_423;
    let _e248: Line = other_423;
    let _e251: Line = other_423;
    let _e265: MultiVector = self_503;
    let _e269: Line = other_423;
    let _e272: Line = other_423;
    let _e275: Line = other_423;
    let _e278: Line = other_423;
    let _e292: MultiVector = self_503;
    let _e296: Line = other_423;
    let _e299: Line = other_423;
    let _e302: Line = other_423;
    let _e305: Line = other_423;
    let _e317: MultiVector = self_503;
    let _e321: Line = other_423;
    let _e324: Line = other_423;
    let _e327: Line = other_423;
    let _e330: Line = other_423;
    let _e343: MultiVector = self_503;
    let _e347: Line = other_423;
    let _e350: Line = other_423;
    let _e353: Line = other_423;
    let _e356: Line = other_423;
    let _e370: MultiVector = self_503;
    let _e374: Line = other_423;
    let _e377: Line = other_423;
    let _e380: Line = other_423;
    let _e383: Line = other_423;
    let _e397: MultiVector = self_503;
    let _e401: Line = other_423;
    let _e404: Line = other_423;
    let _e407: Line = other_423;
    let _e410: Line = other_423;
    let _e422: MultiVector = self_503;
    let _e426: Line = other_423;
    let _e429: Line = other_423;
    let _e432: Line = other_423;
    let _e435: Line = other_423;
    let _e449: MultiVector = self_503;
    let _e453: Line = other_423;
    let _e456: Line = other_423;
    let _e459: Line = other_423;
    let _e462: Line = other_423;
    let _e476: MultiVector = self_503;
    let _e480: Line = other_423;
    let _e483: Line = other_423;
    let _e486: Line = other_423;
    let _e489: Line = other_423;
    let _e503: MultiVector = self_503;
    let _e507: Line = other_423;
    let _e510: Line = other_423;
    let _e513: Line = other_423;
    let _e516: Line = other_423;
    let _e528: MultiVector = self_503;
    let _e532: Line = other_423;
    let _e535: Line = other_423;
    let _e538: Line = other_423;
    let _e541: Line = other_423;
    let _e554: MultiVector = self_503;
    let _e558: Line = other_423;
    let _e561: Line = other_423;
    let _e564: Line = other_423;
    let _e567: Line = other_423;
    let _e581: MultiVector = self_503;
    let _e585: Line = other_423;
    let _e588: Line = other_423;
    let _e591: Line = other_423;
    let _e594: Line = other_423;
    let _e608: MultiVector = self_503;
    let _e612: Line = other_423;
    let _e615: Line = other_423;
    let _e618: Line = other_423;
    let _e621: Line = other_423;
    return MultiVector((((((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.x, _e11.g0_.x, _e14.g0_.z, _e17.g0_.y)) * vec4<f32>(1.0, 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e29.g0_.z) * vec4<f32>(_e33.g0_.y, _e36.g0_.z, _e39.g0_.y, _e42.g0_.x)) * vec4<f32>(1.0, -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e55.g0_.w) * vec4<f32>(_e59.g0_.z, _e62.g0_.y, _e65.g0_.x, _e68.g0_.z)) * vec4<f32>(1.0, 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e81.g3_.x) * vec4<f32>(_e85.g1_.x, _e88.g1_.x, _e91.g1_.y, _e94.g1_.z)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))) + ((vec4<f32>(_e106.g3_.y) * vec4<f32>(_e110.g1_.x, _e113.g1_.x, _e116.g1_.z, _e119.g1_.y)) * vec4<f32>(1.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e132.g3_.z) * vec4<f32>(_e136.g1_.y, _e139.g1_.z, _e142.g1_.y, _e145.g1_.x)) * vec4<f32>(1.0, -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e158.g3_.w) * vec4<f32>(_e162.g1_.z, _e165.g1_.y, _e168.g1_.x, _e171.g1_.z)) * vec4<f32>(1.0, 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e184.g0_.x) * vec4<f32>(_e188.g0_.x, _e191.g0_.x, _e194.g0_.y, _e197.g0_.z)) * vec4<f32>(0.0, -(1.0), -(1.0), -(1.0)))), (((((vec4<f32>(_e212.g1_.y) * vec4<f32>(_e216.g0_.x, _e219.g0_.x, _e222.g0_.z, _e225.g0_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e238.g1_.z) * vec4<f32>(_e242.g0_.y, _e245.g0_.z, _e248.g0_.y, _e251.g0_.x)) * vec4<f32>(-(1.0), -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e265.g1_.w) * vec4<f32>(_e269.g0_.z, _e272.g0_.y, _e275.g0_.x, _e278.g0_.z)) * vec4<f32>(-(1.0), 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e292.g1_.x) * vec4<f32>(_e296.g0_.x, _e299.g0_.x, _e302.g0_.y, _e305.g0_.z)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))), (((((((((vec4<f32>(_e317.g1_.y) * vec4<f32>(_e321.g1_.x, _e324.g1_.x, _e327.g1_.z, _e330.g1_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e343.g1_.z) * vec4<f32>(_e347.g1_.y, _e350.g1_.z, _e353.g1_.y, _e356.g1_.x)) * vec4<f32>(-(1.0), -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e370.g1_.w) * vec4<f32>(_e374.g1_.z, _e377.g1_.y, _e380.g1_.x, _e383.g1_.z)) * vec4<f32>(-(1.0), 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e397.g2_.x) * vec4<f32>(_e401.g0_.x, _e404.g0_.x, _e407.g0_.y, _e410.g0_.z)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))) + ((vec4<f32>(_e422.g2_.y) * vec4<f32>(_e426.g0_.x, _e429.g0_.x, _e432.g0_.z, _e435.g0_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e449.g2_.z) * vec4<f32>(_e453.g0_.y, _e456.g0_.z, _e459.g0_.y, _e462.g0_.x)) * vec4<f32>(-(1.0), -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e476.g2_.w) * vec4<f32>(_e480.g0_.z, _e483.g0_.y, _e486.g0_.x, _e489.g0_.z)) * vec4<f32>(-(1.0), 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e503.g1_.x) * vec4<f32>(_e507.g1_.x, _e510.g1_.x, _e513.g1_.y, _e516.g1_.z)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))), (((((vec4<f32>(_e528.g3_.y) * vec4<f32>(_e532.g0_.x, _e535.g0_.x, _e538.g0_.z, _e541.g0_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e554.g3_.z) * vec4<f32>(_e558.g0_.y, _e561.g0_.z, _e564.g0_.y, _e567.g0_.x)) * vec4<f32>(-(1.0), -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e581.g3_.w) * vec4<f32>(_e585.g0_.z, _e588.g0_.y, _e591.g0_.x, _e594.g0_.z)) * vec4<f32>(-(1.0), 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e608.g3_.x) * vec4<f32>(_e612.g0_.x, _e615.g0_.x, _e618.g0_.y, _e621.g0_.z)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn multi_vector_line_scalar_product(self_504: MultiVector, other_424: Line) -> Scalar {
    var self_505: MultiVector;
    var other_425: Line;

    self_505 = self_504;
    other_425 = other_424;
    let _e5: MultiVector = self_505;
    let _e8: Line = other_425;
    let _e13: MultiVector = self_505;
    let _e16: Line = other_425;
    let _e21: MultiVector = self_505;
    let _e24: Line = other_425;
    return Scalar((((0.0 - (_e5.g0_.y * _e8.g1_.x)) - (_e13.g0_.z * _e16.g1_.y)) - (_e21.g0_.w * _e24.g1_.z)));
}

fn multi_vector_line_anti_scalar_product(self_506: MultiVector, other_426: Line) -> AntiScalar {
    var self_507: MultiVector;
    var other_427: Line;

    self_507 = self_506;
    other_427 = other_426;
    let _e5: MultiVector = self_507;
    let _e8: Line = other_427;
    let _e13: MultiVector = self_507;
    let _e16: Line = other_427;
    let _e21: MultiVector = self_507;
    let _e24: Line = other_427;
    return AntiScalar((((0.0 - (_e5.g3_.y * _e8.g0_.x)) - (_e13.g3_.z * _e16.g0_.y)) - (_e21.g3_.w * _e24.g0_.z)));
}

fn multi_vector_translator_into(self_508: MultiVector) -> Translator {
    var self_509: MultiVector;

    self_509 = self_508;
    let _e2: MultiVector = self_509;
    let _e5: MultiVector = self_509;
    let _e8: MultiVector = self_509;
    let _e11: MultiVector = self_509;
    return Translator(vec4<f32>(_e2.g0_.x, _e5.g3_.y, _e8.g3_.z, _e11.g3_.w));
}

fn multi_vector_translator_add(self_510: MultiVector, other_428: Translator) -> MultiVector {
    var self_511: MultiVector;
    var other_429: Translator;

    self_511 = self_510;
    other_429 = other_428;
    let _e4: MultiVector = self_511;
    let _e6: Translator = other_429;
    let _e17: MultiVector = self_511;
    let _e19: MultiVector = self_511;
    let _e21: MultiVector = self_511;
    let _e23: Translator = other_429;
    return MultiVector((_e4.g0_ + (vec4<f32>(_e6.g0_.x) * vec4<f32>(1.0, 0.0, 0.0, 0.0))), _e17.g1_, _e19.g2_, (_e21.g3_ + (_e23.g0_ * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn multi_vector_translator_sub(self_512: MultiVector, other_430: Translator) -> MultiVector {
    var self_513: MultiVector;
    var other_431: Translator;

    self_513 = self_512;
    other_431 = other_430;
    let _e4: MultiVector = self_513;
    let _e6: Translator = other_431;
    let _e17: MultiVector = self_513;
    let _e19: MultiVector = self_513;
    let _e21: MultiVector = self_513;
    let _e23: Translator = other_431;
    return MultiVector((_e4.g0_ - (vec4<f32>(_e6.g0_.x) * vec4<f32>(1.0, 0.0, 0.0, 0.0))), _e17.g1_, _e19.g2_, (_e21.g3_ - (_e23.g0_ * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn multi_vector_translator_geometric_product(self_514: MultiVector, other_432: Translator) -> MultiVector {
    var self_515: MultiVector;
    var other_433: Translator;

    self_515 = self_514;
    other_433 = other_432;
    let _e4: MultiVector = self_515;
    let _e6: Translator = other_433;
    let _e11: MultiVector = self_515;
    let _e15: Translator = other_433;
    let _e25: MultiVector = self_515;
    let _e29: Translator = other_433;
    let _e42: MultiVector = self_515;
    let _e46: Translator = other_433;
    let _e59: MultiVector = self_515;
    let _e63: Translator = other_433;
    let _e76: MultiVector = self_515;
    let _e78: Translator = other_433;
    let _e84: MultiVector = self_515;
    let _e86: Translator = other_433;
    let _e91: MultiVector = self_515;
    let _e95: Translator = other_433;
    let _e106: MultiVector = self_515;
    let _e110: Translator = other_433;
    let _e122: MultiVector = self_515;
    let _e126: Translator = other_433;
    let _e138: MultiVector = self_515;
    let _e142: Translator = other_433;
    let _e154: MultiVector = self_515;
    let _e158: Translator = other_433;
    let _e170: MultiVector = self_515;
    let _e174: Translator = other_433;
    let _e186: MultiVector = self_515;
    let _e189: MultiVector = self_515;
    let _e192: MultiVector = self_515;
    let _e195: MultiVector = self_515;
    let _e199: Translator = other_433;
    return MultiVector((_e4.g0_ * vec4<f32>(_e6.g0_.x)), ((((((vec4<f32>(_e11.g2_.x) * _e15.g0_.yyzw) * vec4<f32>(0.0, 1.0, 1.0, 1.0)) + ((vec4<f32>(_e25.g2_.y) * _e29.g0_.yywz) * vec4<f32>(-(1.0), 0.0, -(1.0), 1.0))) + ((vec4<f32>(_e42.g2_.z) * _e46.g0_.zwzy) * vec4<f32>(-(1.0), 1.0, 0.0, -(1.0)))) + ((vec4<f32>(_e59.g2_.w) * _e63.g0_.wzyw) * vec4<f32>(-(1.0), -(1.0), 1.0, 0.0))) + (_e76.g1_ * vec4<f32>(_e78.g0_.x))), (_e84.g2_ * vec4<f32>(_e86.g0_.x)), ((((((((vec4<f32>(_e91.g0_.y) * _e95.g0_.yywz) * vec4<f32>(1.0, 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e106.g0_.z) * _e110.g0_.zwzy) * vec4<f32>(1.0, -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e122.g0_.w) * _e126.g0_.wzyw) * vec4<f32>(1.0, 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e138.g3_.y) * vec4<f32>(_e142.g0_.x)) * vec4<f32>(0.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e154.g3_.z) * vec4<f32>(_e158.g0_.x)) * vec4<f32>(0.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e170.g3_.w) * vec4<f32>(_e174.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + (vec4<f32>(_e186.g3_.x, _e189.g0_.x, _e192.g0_.x, _e195.g0_.x) * _e199.g0_)));
}

fn multi_vector_translator_outer_product(self_516: MultiVector, other_434: Translator) -> MultiVector {
    var self_517: MultiVector;
    var other_435: Translator;

    self_517 = self_516;
    other_435 = other_434;
    let _e4: MultiVector = self_517;
    let _e6: Translator = other_435;
    let _e11: MultiVector = self_517;
    let _e15: Translator = other_435;
    let _e26: MultiVector = self_517;
    let _e30: Translator = other_435;
    let _e42: MultiVector = self_517;
    let _e46: Translator = other_435;
    let _e58: MultiVector = self_517;
    let _e60: Translator = other_435;
    let _e66: MultiVector = self_517;
    let _e68: Translator = other_435;
    let _e73: MultiVector = self_517;
    let _e77: Translator = other_435;
    let _e88: MultiVector = self_517;
    let _e92: Translator = other_435;
    let _e104: MultiVector = self_517;
    let _e108: Translator = other_435;
    let _e120: MultiVector = self_517;
    let _e124: Translator = other_435;
    let _e136: MultiVector = self_517;
    let _e140: Translator = other_435;
    let _e152: MultiVector = self_517;
    let _e156: Translator = other_435;
    let _e168: MultiVector = self_517;
    let _e171: Translator = other_435;
    return MultiVector((_e4.g0_ * vec4<f32>(_e6.g0_.x)), (((((vec4<f32>(_e11.g2_.y) * _e15.g0_.wwwz) * vec4<f32>(0.0, 0.0, -(1.0), 1.0)) + ((vec4<f32>(_e26.g2_.z) * _e30.g0_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0)))) + ((vec4<f32>(_e42.g2_.w) * _e46.g0_.zzyz) * vec4<f32>(0.0, -(1.0), 1.0, 0.0))) + (_e58.g1_ * vec4<f32>(_e60.g0_.x))), (_e66.g2_ * vec4<f32>(_e68.g0_.x)), ((((((((vec4<f32>(_e73.g0_.z) * vec4<f32>(_e77.g0_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) + ((vec4<f32>(_e88.g0_.w) * vec4<f32>(_e92.g0_.w)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e104.g3_.x) * vec4<f32>(_e108.g0_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e120.g3_.y) * vec4<f32>(_e124.g0_.x)) * vec4<f32>(0.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e136.g3_.z) * vec4<f32>(_e140.g0_.x)) * vec4<f32>(0.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e152.g3_.w) * vec4<f32>(_e156.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + (_e168.g0_.yxxx * _e171.g0_.yyzw)));
}

fn multi_vector_translator_inner_product(self_518: MultiVector, other_436: Translator) -> MultiVector {
    var self_519: MultiVector;
    var other_437: Translator;

    self_519 = self_518;
    other_437 = other_436;
    let _e4: MultiVector = self_519;
    let _e6: Translator = other_437;
    let _e11: MultiVector = self_519;
    let _e15: Translator = other_437;
    let _e27: MultiVector = self_519;
    let _e31: Translator = other_437;
    let _e44: MultiVector = self_519;
    let _e48: Translator = other_437;
    let _e61: MultiVector = self_519;
    let _e63: Translator = other_437;
    let _e69: MultiVector = self_519;
    let _e71: Translator = other_437;
    let _e76: MultiVector = self_519;
    let _e80: Translator = other_437;
    let _e91: MultiVector = self_519;
    let _e95: Translator = other_437;
    let _e107: MultiVector = self_519;
    let _e111: Translator = other_437;
    let _e123: MultiVector = self_519;
    let _e126: MultiVector = self_519;
    let _e129: MultiVector = self_519;
    let _e132: MultiVector = self_519;
    let _e136: Translator = other_437;
    return MultiVector((_e4.g0_ * vec4<f32>(_e6.g0_.x)), (((((vec4<f32>(_e11.g2_.y) * vec4<f32>(_e15.g0_.y)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0)) + ((vec4<f32>(_e27.g2_.z) * vec4<f32>(_e31.g0_.z)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((vec4<f32>(_e44.g2_.w) * vec4<f32>(_e48.g0_.w)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + (_e61.g1_ * vec4<f32>(_e63.g0_.x))), (_e69.g2_ * vec4<f32>(_e71.g0_.x)), (((((vec4<f32>(_e76.g3_.y) * vec4<f32>(_e80.g0_.x)) * vec4<f32>(0.0, 1.0, 0.0, 0.0)) + ((vec4<f32>(_e91.g3_.z) * vec4<f32>(_e95.g0_.x)) * vec4<f32>(0.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e107.g3_.w) * vec4<f32>(_e111.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + (vec4<f32>(_e123.g3_.x, _e126.g0_.x, _e129.g0_.x, _e132.g0_.x) * _e136.g0_)));
}

fn multi_vector_translator_geometric_anti_product(self_520: MultiVector, other_438: Translator) -> MultiVector {
    var self_521: MultiVector;
    var other_439: Translator;

    self_521 = self_520;
    other_439 = other_438;
    let _e4: MultiVector = self_521;
    let _e8: Translator = other_439;
    let _e19: MultiVector = self_521;
    let _e23: Translator = other_439;
    let _e35: MultiVector = self_521;
    let _e39: Translator = other_439;
    let _e51: MultiVector = self_521;
    let _e55: Translator = other_439;
    let _e68: MultiVector = self_521;
    let _e72: Translator = other_439;
    let _e85: MultiVector = self_521;
    let _e89: Translator = other_439;
    let _e102: MultiVector = self_521;
    let _e105: MultiVector = self_521;
    let _e108: MultiVector = self_521;
    let _e111: MultiVector = self_521;
    let _e115: Translator = other_439;
    let _e128: MultiVector = self_521;
    let _e132: Translator = other_439;
    let _e144: MultiVector = self_521;
    let _e148: Translator = other_439;
    let _e161: MultiVector = self_521;
    let _e165: Translator = other_439;
    let _e178: MultiVector = self_521;
    let _e182: Translator = other_439;
    let _e192: MultiVector = self_521;
    let _e196: Translator = other_439;
    let _e206: MultiVector = self_521;
    let _e210: Translator = other_439;
    let _e223: MultiVector = self_521;
    let _e227: Translator = other_439;
    let _e240: MultiVector = self_521;
    let _e244: Translator = other_439;
    let _e257: MultiVector = self_521;
    let _e259: Translator = other_439;
    let _e269: MultiVector = self_521;
    let _e273: Translator = other_439;
    let _e285: MultiVector = self_521;
    let _e289: Translator = other_439;
    let _e302: MultiVector = self_521;
    let _e306: Translator = other_439;
    let _e319: MultiVector = self_521;
    let _e323: Translator = other_439;
    return MultiVector(((((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.yywz) * vec4<f32>(1.0, 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.zwzy) * vec4<f32>(1.0, -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e35.g0_.w) * _e39.g0_.wzyw) * vec4<f32>(1.0, 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e51.g3_.y) * vec4<f32>(_e55.g0_.x)) * vec4<f32>(0.0, -(1.0), 0.0, 0.0))) + ((vec4<f32>(_e68.g3_.z) * vec4<f32>(_e72.g0_.x)) * vec4<f32>(0.0, 0.0, -(1.0), 0.0))) + ((vec4<f32>(_e85.g3_.w) * vec4<f32>(_e89.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e102.g3_.x, _e105.g0_.x, _e108.g0_.x, _e111.g0_.x) * _e115.g0_) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0)))), (((((vec4<f32>(_e128.g1_.y) * _e132.g0_.yywz) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e144.g1_.z) * _e148.g0_.zwzy) * vec4<f32>(-(1.0), -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e161.g1_.w) * _e165.g0_.wzyw) * vec4<f32>(-(1.0), 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e178.g1_.x) * _e182.g0_) * vec4<f32>(0.0, 1.0, 1.0, 1.0))), ((((((vec4<f32>(_e192.g2_.x) * _e196.g0_.yyzw) * vec4<f32>(0.0, 1.0, 1.0, 1.0)) + ((vec4<f32>(_e206.g2_.y) * _e210.g0_.yywz) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e223.g2_.z) * _e227.g0_.zwzy) * vec4<f32>(-(1.0), -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e240.g2_.w) * _e244.g0_.wzyw) * vec4<f32>(-(1.0), 1.0, -(1.0), 0.0))) + ((_e257.g1_ * vec4<f32>(_e259.g0_.x)) * vec4<f32>(-(1.0)))), (((((vec4<f32>(_e269.g3_.y) * _e273.g0_.yywz) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e285.g3_.z) * _e289.g0_.zwzy) * vec4<f32>(-(1.0), -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e302.g3_.w) * _e306.g0_.wzyw) * vec4<f32>(-(1.0), 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e319.g3_.x) * _e323.g0_) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn multi_vector_translator_right_contraction(self_522: MultiVector, other_440: Translator) -> MultiVector {
    var self_523: MultiVector;
    var other_441: Translator;

    self_523 = self_522;
    other_441 = other_440;
    let _e4: MultiVector = self_523;
    let _e6: Translator = other_441;
    let _e11: MultiVector = self_523;
    let _e13: Translator = other_441;
    let _e18: MultiVector = self_523;
    let _e20: Translator = other_441;
    let _e25: MultiVector = self_523;
    let _e27: Translator = other_441;
    return MultiVector((_e4.g0_ * vec4<f32>(_e6.g0_.x)), (_e11.g1_ * vec4<f32>(_e13.g0_.x)), (_e18.g2_ * vec4<f32>(_e20.g0_.x)), (_e25.g3_ * vec4<f32>(_e27.g0_.x)));
}

fn multi_vector_translator_scalar_product(self_524: MultiVector, other_442: Translator) -> Scalar {
    var self_525: MultiVector;
    var other_443: Translator;

    self_525 = self_524;
    other_443 = other_442;
    let _e4: MultiVector = self_525;
    let _e7: Translator = other_443;
    return Scalar((_e4.g0_.x * _e7.g0_.x));
}

fn multi_vector_translator_anti_scalar_product(self_526: MultiVector, other_444: Translator) -> AntiScalar {
    var self_527: MultiVector;
    var other_445: Translator;

    self_527 = self_526;
    other_445 = other_444;
    let _e5: MultiVector = self_527;
    let _e8: Translator = other_445;
    let _e13: MultiVector = self_527;
    let _e16: Translator = other_445;
    let _e21: MultiVector = self_527;
    let _e24: Translator = other_445;
    return AntiScalar((((0.0 - (_e5.g3_.y * _e8.g0_.y)) - (_e13.g3_.z * _e16.g0_.z)) - (_e21.g3_.w * _e24.g0_.w)));
}

fn multi_vector_motor_into(self_528: MultiVector) -> Motor {
    var self_529: MultiVector;

    self_529 = self_528;
    let _e2: MultiVector = self_529;
    let _e4: MultiVector = self_529;
    return Motor(_e2.g0_, _e4.g3_);
}

fn multi_vector_motor_add(self_530: MultiVector, other_446: Motor) -> MultiVector {
    var self_531: MultiVector;
    var other_447: Motor;

    self_531 = self_530;
    other_447 = other_446;
    let _e4: MultiVector = self_531;
    let _e6: Motor = other_447;
    let _e9: MultiVector = self_531;
    let _e11: MultiVector = self_531;
    let _e13: MultiVector = self_531;
    let _e15: Motor = other_447;
    return MultiVector((_e4.g0_ + _e6.g0_), _e9.g1_, _e11.g2_, (_e13.g3_ + _e15.g1_));
}

fn multi_vector_motor_sub(self_532: MultiVector, other_448: Motor) -> MultiVector {
    var self_533: MultiVector;
    var other_449: Motor;

    self_533 = self_532;
    other_449 = other_448;
    let _e4: MultiVector = self_533;
    let _e6: Motor = other_449;
    let _e9: MultiVector = self_533;
    let _e11: MultiVector = self_533;
    let _e13: MultiVector = self_533;
    let _e15: Motor = other_449;
    return MultiVector((_e4.g0_ - _e6.g0_), _e9.g1_, _e11.g2_, (_e13.g3_ - _e15.g1_));
}

fn multi_vector_motor_geometric_product(self_534: MultiVector, other_450: Motor) -> MultiVector {
    var self_535: MultiVector;
    var other_451: Motor;

    self_535 = self_534;
    other_451 = other_450;
    let _e4: MultiVector = self_535;
    let _e8: Motor = other_451;
    let _e11: MultiVector = self_535;
    let _e15: Motor = other_451;
    let _e28: MultiVector = self_535;
    let _e32: Motor = other_451;
    let _e45: MultiVector = self_535;
    let _e49: Motor = other_451;
    let _e62: MultiVector = self_535;
    let _e66: Motor = other_451;
    let _e78: MultiVector = self_535;
    let _e82: Motor = other_451;
    let _e94: MultiVector = self_535;
    let _e98: Motor = other_451;
    let _e110: MultiVector = self_535;
    let _e114: Motor = other_451;
    let _e126: MultiVector = self_535;
    let _e130: Motor = other_451;
    let _e134: MultiVector = self_535;
    let _e138: Motor = other_451;
    let _e151: MultiVector = self_535;
    let _e155: Motor = other_451;
    let _e168: MultiVector = self_535;
    let _e172: Motor = other_451;
    let _e185: MultiVector = self_535;
    let _e189: Motor = other_451;
    let _e201: MultiVector = self_535;
    let _e205: Motor = other_451;
    let _e217: MultiVector = self_535;
    let _e221: Motor = other_451;
    let _e233: MultiVector = self_535;
    let _e237: Motor = other_451;
    let _e249: MultiVector = self_535;
    let _e253: Motor = other_451;
    let _e256: MultiVector = self_535;
    let _e260: Motor = other_451;
    let _e273: MultiVector = self_535;
    let _e277: Motor = other_451;
    let _e290: MultiVector = self_535;
    let _e294: Motor = other_451;
    let _e307: MultiVector = self_535;
    let _e311: Motor = other_451;
    let _e324: MultiVector = self_535;
    let _e328: Motor = other_451;
    let _e340: MultiVector = self_535;
    let _e344: Motor = other_451;
    let _e356: MultiVector = self_535;
    let _e360: Motor = other_451;
    return MultiVector(((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.y) * _e15.g0_.yxwz) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e28.g0_.z) * _e32.g0_.zwxy) * vec4<f32>(-(1.0), -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e45.g0_.w) * _e49.g0_.wzyx) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))), (((((((((vec4<f32>(_e62.g1_.x) * _e66.g0_) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))) + ((vec4<f32>(_e78.g1_.y) * _e82.g0_.yxwz) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e94.g1_.z) * _e98.g0_.zwxy) * vec4<f32>(1.0, -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e110.g1_.w) * _e114.g0_.wzyx) * vec4<f32>(1.0, 1.0, -(1.0), 1.0))) + (vec4<f32>(_e126.g2_.x) * _e130.g1_)) + ((vec4<f32>(_e134.g2_.y) * _e138.g1_.yxwz) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))) + ((vec4<f32>(_e151.g2_.z) * _e155.g1_.zwxy) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e168.g2_.w) * _e172.g1_.wzyx) * vec4<f32>(-(1.0), -(1.0), 1.0, 1.0))), (((((vec4<f32>(_e185.g2_.x) * _e189.g0_) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))) + ((vec4<f32>(_e201.g2_.y) * _e205.g0_.yxwz) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e217.g2_.z) * _e221.g0_.zwxy) * vec4<f32>(1.0, -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e233.g2_.w) * _e237.g0_.wzyx) * vec4<f32>(1.0, 1.0, -(1.0), 1.0))), ((((((((vec4<f32>(_e249.g0_.x) * _e253.g1_) + ((vec4<f32>(_e256.g0_.y) * _e260.g1_.yxwz) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e273.g0_.z) * _e277.g1_.zwxy) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e290.g0_.w) * _e294.g1_.wzyx) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e307.g3_.x) * _e311.g0_) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0)))) + ((vec4<f32>(_e324.g3_.y) * _e328.g0_.yxwz) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e340.g3_.z) * _e344.g0_.zwxy) * vec4<f32>(1.0, -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e356.g3_.w) * _e360.g0_.wzyx) * vec4<f32>(1.0, 1.0, -(1.0), 1.0))));
}

fn multi_vector_motor_regressive_product(self_536: MultiVector, other_452: Motor) -> MultiVector {
    var self_537: MultiVector;
    var other_453: Motor;

    self_537 = self_536;
    other_453 = other_452;
    let _e4: MultiVector = self_537;
    let _e8: Motor = other_453;
    let _e18: MultiVector = self_537;
    let _e22: Motor = other_453;
    let _e33: MultiVector = self_537;
    let _e37: Motor = other_453;
    let _e48: MultiVector = self_537;
    let _e52: Motor = other_453;
    let _e56: MultiVector = self_537;
    let _e60: Motor = other_453;
    let _e72: MultiVector = self_537;
    let _e76: Motor = other_453;
    let _e88: MultiVector = self_537;
    let _e92: Motor = other_453;
    let _e104: MultiVector = self_537;
    let _e108: Motor = other_453;
    let _e120: MultiVector = self_537;
    let _e124: Motor = other_453;
    let _e135: MultiVector = self_537;
    let _e139: Motor = other_453;
    let _e151: MultiVector = self_537;
    let _e155: Motor = other_453;
    let _e167: MultiVector = self_537;
    let _e171: Motor = other_453;
    let _e183: MultiVector = self_537;
    let _e187: Motor = other_453;
    let _e198: MultiVector = self_537;
    let _e202: Motor = other_453;
    let _e214: MultiVector = self_537;
    let _e218: Motor = other_453;
    let _e222: MultiVector = self_537;
    let _e226: Motor = other_453;
    let _e238: MultiVector = self_537;
    let _e242: Motor = other_453;
    let _e254: MultiVector = self_537;
    let _e257: MultiVector = self_537;
    let _e260: MultiVector = self_537;
    let _e263: MultiVector = self_537;
    let _e267: Motor = other_453;
    let _e270: Motor = other_453;
    let _e273: Motor = other_453;
    let _e276: Motor = other_453;
    let _e289: MultiVector = self_537;
    let _e293: Motor = other_453;
    let _e296: MultiVector = self_537;
    let _e298: Motor = other_453;
    return MultiVector((((((((((vec4<f32>(_e4.g0_.y) * _e8.g1_.yxyy) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) + ((vec4<f32>(_e18.g0_.z) * _e22.g1_.zzxz) * vec4<f32>(1.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e33.g0_.w) * _e37.g1_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, 1.0))) + (vec4<f32>(_e48.g3_.x) * _e52.g0_)) + ((vec4<f32>(_e56.g3_.y) * vec4<f32>(_e60.g0_.y)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e72.g3_.z) * vec4<f32>(_e76.g0_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e88.g3_.w) * vec4<f32>(_e92.g0_.w)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e104.g0_.x) * vec4<f32>(_e108.g1_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))), (((((vec4<f32>(_e120.g1_.y) * _e124.g1_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0)) + ((vec4<f32>(_e135.g1_.z) * _e139.g1_.zzxz) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0))) + ((vec4<f32>(_e151.g1_.w) * _e155.g1_.wwwx) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))) + ((vec4<f32>(_e167.g1_.x) * vec4<f32>(_e171.g1_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))), (((((((vec4<f32>(_e183.g1_.z) * _e187.g0_.wwwy) * vec4<f32>(0.0, -(1.0), 0.0, 1.0)) + ((vec4<f32>(_e198.g1_.w) * _e202.g0_.zzyz) * vec4<f32>(0.0, 1.0, -(1.0), 0.0))) + (vec4<f32>(_e214.g2_.x) * _e218.g1_)) + ((vec4<f32>(_e222.g2_.z) * vec4<f32>(_e226.g1_.x)) * vec4<f32>(0.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e238.g2_.w) * vec4<f32>(_e242.g1_.x)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e254.g1_.x, _e257.g2_.y, _e260.g1_.y, _e263.g1_.y) * vec4<f32>(_e267.g0_.x, _e270.g1_.x, _e273.g0_.w, _e276.g0_.z)) * vec4<f32>(0.0, 1.0, 1.0, -(1.0)))), ((vec4<f32>(_e289.g3_.x) * _e293.g1_) + ((_e296.g3_ * vec4<f32>(_e298.g1_.x)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn multi_vector_motor_outer_product(self_538: MultiVector, other_454: Motor) -> MultiVector {
    var self_539: MultiVector;
    var other_455: Motor;

    self_539 = self_538;
    other_455 = other_454;
    let _e4: MultiVector = self_539;
    let _e8: Motor = other_455;
    let _e11: MultiVector = self_539;
    let _e13: Motor = other_455;
    let _e25: MultiVector = self_539;
    let _e29: Motor = other_455;
    let _e41: MultiVector = self_539;
    let _e45: Motor = other_455;
    let _e57: MultiVector = self_539;
    let _e61: Motor = other_455;
    let _e73: MultiVector = self_539;
    let _e77: Motor = other_455;
    let _e89: MultiVector = self_539;
    let _e91: Motor = other_455;
    let _e103: MultiVector = self_539;
    let _e107: Motor = other_455;
    let _e117: MultiVector = self_539;
    let _e121: Motor = other_455;
    let _e132: MultiVector = self_539;
    let _e136: Motor = other_455;
    let _e147: MultiVector = self_539;
    let _e151: Motor = other_455;
    let _e163: MultiVector = self_539;
    let _e167: Motor = other_455;
    let _e170: MultiVector = self_539;
    let _e174: Motor = other_455;
    let _e186: MultiVector = self_539;
    let _e190: Motor = other_455;
    let _e202: MultiVector = self_539;
    let _e206: Motor = other_455;
    let _e218: MultiVector = self_539;
    let _e222: Motor = other_455;
    let _e233: MultiVector = self_539;
    let _e237: Motor = other_455;
    let _e248: MultiVector = self_539;
    let _e252: Motor = other_455;
    let _e263: MultiVector = self_539;
    let _e266: Motor = other_455;
    return MultiVector(((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((_e11.g0_ * vec4<f32>(_e13.g0_.x)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))), ((((((vec4<f32>(_e25.g1_.x) * _e29.g0_) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))) + ((vec4<f32>(_e41.g2_.y) * _e45.g1_.wwwz) * vec4<f32>(0.0, 0.0, -(1.0), 1.0))) + ((vec4<f32>(_e57.g2_.z) * _e61.g1_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0)))) + ((vec4<f32>(_e73.g2_.w) * _e77.g1_.zzyz) * vec4<f32>(0.0, -(1.0), 1.0, 0.0))) + ((_e89.g1_ * vec4<f32>(_e91.g0_.x)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))), (((((vec4<f32>(_e103.g2_.y) * _e107.g0_.yxyy) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) + ((vec4<f32>(_e117.g2_.z) * _e121.g0_.zzxz) * vec4<f32>(1.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e132.g2_.w) * _e136.g0_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e147.g2_.x) * vec4<f32>(_e151.g0_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))), ((((((((vec4<f32>(_e163.g0_.x) * _e167.g1_) + ((vec4<f32>(_e170.g0_.z) * vec4<f32>(_e174.g1_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e186.g0_.w) * vec4<f32>(_e190.g1_.w)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e202.g3_.x) * vec4<f32>(_e206.g0_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e218.g3_.y) * _e222.g0_.yxyy) * vec4<f32>(1.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e233.g3_.z) * _e237.g0_.zzxz) * vec4<f32>(1.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e248.g3_.w) * _e252.g0_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, 1.0))) + ((_e263.g0_.yxxx * _e266.g1_.yxxx) * vec4<f32>(1.0, 0.0, 0.0, 0.0))));
}

fn multi_vector_motor_inner_product(self_540: MultiVector, other_456: Motor) -> MultiVector {
    var self_541: MultiVector;
    var other_457: Motor;

    self_541 = self_540;
    other_457 = other_456;
    let _e4: MultiVector = self_541;
    let _e8: Motor = other_457;
    let _e11: MultiVector = self_541;
    let _e15: Motor = other_457;
    let _e27: MultiVector = self_541;
    let _e31: Motor = other_457;
    let _e43: MultiVector = self_541;
    let _e46: Motor = other_457;
    let _e58: MultiVector = self_541;
    let _e62: Motor = other_457;
    let _e72: MultiVector = self_541;
    let _e76: Motor = other_457;
    let _e87: MultiVector = self_541;
    let _e91: Motor = other_457;
    let _e102: MultiVector = self_541;
    let _e106: Motor = other_457;
    let _e118: MultiVector = self_541;
    let _e122: Motor = other_457;
    let _e134: MultiVector = self_541;
    let _e138: Motor = other_457;
    let _e150: MultiVector = self_541;
    let _e154: Motor = other_457;
    let _e166: MultiVector = self_541;
    let _e170: Motor = other_457;
    let _e182: MultiVector = self_541;
    let _e186: Motor = other_457;
    let _e198: MultiVector = self_541;
    let _e202: Motor = other_457;
    let _e214: MultiVector = self_541;
    let _e218: Motor = other_457;
    let _e230: MultiVector = self_541;
    let _e233: Motor = other_457;
    let _e245: MultiVector = self_541;
    let _e249: Motor = other_457;
    let _e252: MultiVector = self_541;
    let _e256: Motor = other_457;
    let _e269: MultiVector = self_541;
    let _e273: Motor = other_457;
    let _e285: MultiVector = self_541;
    let _e289: Motor = other_457;
    let _e301: MultiVector = self_541;
    let _e305: Motor = other_457;
    let _e317: MultiVector = self_541;
    let _e319: Motor = other_457;
    return MultiVector(((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.z) * _e15.g0_.zzxz) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0))) + ((vec4<f32>(_e27.g0_.w) * _e31.g0_.wwwx) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))) + ((_e43.g0_.yyxx * _e46.g0_.yxxx) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))), (((((((((vec4<f32>(_e58.g1_.y) * _e62.g0_.yxyy) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) + ((vec4<f32>(_e72.g1_.z) * _e76.g0_.zzxz) * vec4<f32>(1.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e87.g1_.w) * _e91.g0_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e102.g2_.x) * vec4<f32>(_e106.g1_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e118.g2_.y) * _e122.g1_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))) + ((vec4<f32>(_e134.g2_.z) * _e138.g1_.zzxz) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0))) + ((vec4<f32>(_e150.g2_.w) * _e154.g1_.wwwx) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))) + ((vec4<f32>(_e166.g1_.x) * vec4<f32>(_e170.g0_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))), (((((vec4<f32>(_e182.g2_.x) * _e186.g0_) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))) + ((vec4<f32>(_e198.g2_.z) * _e202.g0_.wwxy) * vec4<f32>(0.0, -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e214.g2_.w) * _e218.g0_.zzyx) * vec4<f32>(0.0, 1.0, -(1.0), 1.0))) + ((_e230.g2_.xyyy * _e233.g0_.xxwz) * vec4<f32>(0.0, 1.0, 1.0, -(1.0)))), ((((((vec4<f32>(_e245.g0_.x) * _e249.g1_) + ((vec4<f32>(_e252.g3_.x) * _e256.g0_) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0)))) + ((vec4<f32>(_e269.g3_.y) * vec4<f32>(_e273.g0_.x)) * vec4<f32>(0.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e285.g3_.z) * vec4<f32>(_e289.g0_.x)) * vec4<f32>(0.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e301.g3_.w) * vec4<f32>(_e305.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((_e317.g0_ * vec4<f32>(_e319.g1_.x)) * vec4<f32>(0.0, -(1.0), -(1.0), -(1.0)))));
}

fn multi_vector_motor_geometric_anti_product(self_542: MultiVector, other_458: Motor) -> MultiVector {
    var self_543: MultiVector;
    var other_459: Motor;

    self_543 = self_542;
    other_459 = other_458;
    let _e4: MultiVector = self_543;
    let _e8: Motor = other_459;
    let _e20: MultiVector = self_543;
    let _e24: Motor = other_459;
    let _e36: MultiVector = self_543;
    let _e40: Motor = other_459;
    let _e52: MultiVector = self_543;
    let _e56: Motor = other_459;
    let _e68: MultiVector = self_543;
    let _e72: Motor = other_459;
    let _e76: MultiVector = self_543;
    let _e80: Motor = other_459;
    let _e93: MultiVector = self_543;
    let _e97: Motor = other_459;
    let _e110: MultiVector = self_543;
    let _e114: Motor = other_459;
    let _e127: MultiVector = self_543;
    let _e131: Motor = other_459;
    let _e134: MultiVector = self_543;
    let _e138: Motor = other_459;
    let _e151: MultiVector = self_543;
    let _e155: Motor = other_459;
    let _e168: MultiVector = self_543;
    let _e172: Motor = other_459;
    let _e185: MultiVector = self_543;
    let _e189: Motor = other_459;
    let _e199: MultiVector = self_543;
    let _e203: Motor = other_459;
    let _e217: MultiVector = self_543;
    let _e221: Motor = other_459;
    let _e235: MultiVector = self_543;
    let _e239: Motor = other_459;
    let _e253: MultiVector = self_543;
    let _e257: Motor = other_459;
    let _e261: MultiVector = self_543;
    let _e265: Motor = other_459;
    let _e278: MultiVector = self_543;
    let _e282: Motor = other_459;
    let _e295: MultiVector = self_543;
    let _e299: Motor = other_459;
    let _e312: MultiVector = self_543;
    let _e316: Motor = other_459;
    let _e319: MultiVector = self_543;
    let _e323: Motor = other_459;
    let _e336: MultiVector = self_543;
    let _e340: Motor = other_459;
    let _e353: MultiVector = self_543;
    let _e357: Motor = other_459;
    return MultiVector((((((((((vec4<f32>(_e4.g0_.x) * _e8.g1_) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))) + ((vec4<f32>(_e20.g0_.y) * _e24.g1_.yxwz) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e36.g0_.z) * _e40.g1_.zwxy) * vec4<f32>(1.0, -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e52.g0_.w) * _e56.g1_.wzyx) * vec4<f32>(1.0, 1.0, -(1.0), 1.0))) + (vec4<f32>(_e68.g3_.x) * _e72.g0_)) + ((vec4<f32>(_e76.g3_.y) * _e80.g0_.yxwz) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e93.g3_.z) * _e97.g0_.zwxy) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e110.g3_.w) * _e114.g0_.wzyx) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))), ((((vec4<f32>(_e127.g1_.x) * _e131.g1_) + ((vec4<f32>(_e134.g1_.y) * _e138.g1_.yxwz) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e151.g1_.z) * _e155.g1_.zwxy) * vec4<f32>(-(1.0), -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e168.g1_.w) * _e172.g1_.wzyx) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))), (((((((((vec4<f32>(_e185.g1_.x) * _e189.g0_) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0)) + ((vec4<f32>(_e199.g1_.y) * _e203.g0_.yxwz) * vec4<f32>(-(1.0), -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e217.g1_.z) * _e221.g0_.zwxy) * vec4<f32>(-(1.0), -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e235.g1_.w) * _e239.g0_.wzyx) * vec4<f32>(-(1.0), 1.0, -(1.0), -(1.0)))) + (vec4<f32>(_e253.g2_.x) * _e257.g1_)) + ((vec4<f32>(_e261.g2_.y) * _e265.g1_.yxwz) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e278.g2_.z) * _e282.g1_.zwxy) * vec4<f32>(-(1.0), -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e295.g2_.w) * _e299.g1_.wzyx) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))), ((((vec4<f32>(_e312.g3_.x) * _e316.g1_) + ((vec4<f32>(_e319.g3_.y) * _e323.g1_.yxwz) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e336.g3_.z) * _e340.g1_.zwxy) * vec4<f32>(-(1.0), -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e353.g3_.w) * _e357.g1_.wzyx) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))));
}

fn multi_vector_motor_inner_anti_product(self_544: MultiVector, other_460: Motor) -> MultiVector {
    var self_545: MultiVector;
    var other_461: Motor;

    self_545 = self_544;
    other_461 = other_460;
    let _e4: MultiVector = self_545;
    let _e8: Motor = other_461;
    let _e20: MultiVector = self_545;
    let _e24: Motor = other_461;
    let _e28: MultiVector = self_545;
    let _e32: Motor = other_461;
    let _e45: MultiVector = self_545;
    let _e49: Motor = other_461;
    let _e62: MultiVector = self_545;
    let _e66: Motor = other_461;
    let _e79: MultiVector = self_545;
    let _e81: Motor = other_461;
    let _e93: MultiVector = self_545;
    let _e97: Motor = other_461;
    let _e100: MultiVector = self_545;
    let _e104: Motor = other_461;
    let _e116: MultiVector = self_545;
    let _e120: Motor = other_461;
    let _e132: MultiVector = self_545;
    let _e135: Motor = other_461;
    let _e147: MultiVector = self_545;
    let _e151: Motor = other_461;
    let _e163: MultiVector = self_545;
    let _e167: Motor = other_461;
    let _e180: MultiVector = self_545;
    let _e184: Motor = other_461;
    let _e197: MultiVector = self_545;
    let _e201: Motor = other_461;
    let _e213: MultiVector = self_545;
    let _e217: Motor = other_461;
    let _e229: MultiVector = self_545;
    let _e233: Motor = other_461;
    let _e245: MultiVector = self_545;
    let _e249: Motor = other_461;
    let _e261: MultiVector = self_545;
    let _e265: Motor = other_461;
    let _e278: MultiVector = self_545;
    let _e282: Motor = other_461;
    let _e285: MultiVector = self_545;
    let _e289: Motor = other_461;
    let _e301: MultiVector = self_545;
    let _e305: Motor = other_461;
    let _e317: MultiVector = self_545;
    let _e320: Motor = other_461;
    return MultiVector((((((((vec4<f32>(_e4.g0_.x) * _e8.g1_) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))) + (vec4<f32>(_e20.g3_.x) * _e24.g0_)) + ((vec4<f32>(_e28.g3_.y) * vec4<f32>(_e32.g0_.x)) * vec4<f32>(0.0, -(1.0), 0.0, 0.0))) + ((vec4<f32>(_e45.g3_.z) * vec4<f32>(_e49.g0_.x)) * vec4<f32>(0.0, 0.0, -(1.0), 0.0))) + ((vec4<f32>(_e62.g3_.w) * vec4<f32>(_e66.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((_e79.g0_ * vec4<f32>(_e81.g1_.x)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))), ((((vec4<f32>(_e93.g1_.x) * _e97.g1_) + ((vec4<f32>(_e100.g1_.z) * _e104.g1_.wwxy) * vec4<f32>(0.0, -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e116.g1_.w) * _e120.g1_.zzyx) * vec4<f32>(0.0, 1.0, -(1.0), 1.0))) + ((_e132.g1_.xyyy * _e135.g1_.xxwz) * vec4<f32>(0.0, 1.0, 1.0, -(1.0)))), (((((((((vec4<f32>(_e147.g1_.y) * _e151.g0_.yxyy) * vec4<f32>(-(1.0), -(1.0), 0.0, 0.0)) + ((vec4<f32>(_e163.g1_.z) * _e167.g0_.zzxz) * vec4<f32>(-(1.0), 0.0, -(1.0), 0.0))) + ((vec4<f32>(_e180.g1_.w) * _e184.g0_.wwwx) * vec4<f32>(-(1.0), 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e197.g2_.x) * vec4<f32>(_e201.g1_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e213.g2_.y) * _e217.g1_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))) + ((vec4<f32>(_e229.g2_.z) * _e233.g1_.zzxz) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0))) + ((vec4<f32>(_e245.g2_.w) * _e249.g1_.wwwx) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))) + ((vec4<f32>(_e261.g1_.x) * vec4<f32>(_e265.g0_.x)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))), ((((vec4<f32>(_e278.g3_.x) * _e282.g1_) + ((vec4<f32>(_e285.g3_.z) * _e289.g1_.zzxz) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0))) + ((vec4<f32>(_e301.g3_.w) * _e305.g1_.wwwx) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))) + ((_e317.g3_.yyxx * _e320.g1_.yxxx) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))));
}

fn multi_vector_motor_right_contraction(self_546: MultiVector, other_462: Motor) -> MultiVector {
    var self_547: MultiVector;
    var other_463: Motor;

    self_547 = self_546;
    other_463 = other_462;
    let _e4: MultiVector = self_547;
    let _e8: Motor = other_463;
    let _e19: MultiVector = self_547;
    let _e23: Motor = other_463;
    let _e35: MultiVector = self_547;
    let _e39: Motor = other_463;
    let _e51: MultiVector = self_547;
    let _e55: Motor = other_463;
    let _e67: MultiVector = self_547;
    let _e71: Motor = other_463;
    let _e81: MultiVector = self_547;
    let _e85: Motor = other_463;
    let _e96: MultiVector = self_547;
    let _e100: Motor = other_463;
    let _e111: MultiVector = self_547;
    let _e115: Motor = other_463;
    let _e127: MultiVector = self_547;
    let _e131: Motor = other_463;
    let _e143: MultiVector = self_547;
    let _e145: Motor = other_463;
    let _e157: MultiVector = self_547;
    let _e161: Motor = other_463;
    let _e173: MultiVector = self_547;
    let _e175: Motor = other_463;
    return MultiVector((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0)) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.zzxz) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0))) + ((vec4<f32>(_e35.g0_.w) * _e39.g0_.wwwx) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))) + ((vec4<f32>(_e51.g0_.x) * vec4<f32>(_e55.g0_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))), (((((vec4<f32>(_e67.g1_.y) * _e71.g0_.yxyy) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) + ((vec4<f32>(_e81.g1_.z) * _e85.g0_.zzxz) * vec4<f32>(1.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e96.g1_.w) * _e100.g0_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e111.g1_.x) * vec4<f32>(_e115.g0_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))), (((vec4<f32>(_e127.g2_.x) * _e131.g0_) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))) + ((_e143.g2_ * vec4<f32>(_e145.g0_.x)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))), (((vec4<f32>(_e157.g3_.x) * _e161.g0_) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))) + ((_e173.g3_ * vec4<f32>(_e175.g0_.x)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn multi_vector_motor_right_anti_contraction(self_548: MultiVector, other_464: Motor) -> MultiVector {
    var self_549: MultiVector;
    var other_465: Motor;

    self_549 = self_548;
    other_465 = other_464;
    let _e4: MultiVector = self_549;
    let _e8: Motor = other_465;
    let _e20: MultiVector = self_549;
    let _e22: Motor = other_465;
    let _e34: MultiVector = self_549;
    let _e38: Motor = other_465;
    let _e41: MultiVector = self_549;
    let _e43: Motor = other_465;
    let _e55: MultiVector = self_549;
    let _e59: Motor = other_465;
    let _e70: MultiVector = self_549;
    let _e74: Motor = other_465;
    let _e86: MultiVector = self_549;
    let _e90: Motor = other_465;
    let _e102: MultiVector = self_549;
    let _e106: Motor = other_465;
    let _e118: MultiVector = self_549;
    let _e122: Motor = other_465;
    let _e133: MultiVector = self_549;
    let _e137: Motor = other_465;
    let _e149: MultiVector = self_549;
    let _e153: Motor = other_465;
    let _e165: MultiVector = self_549;
    let _e169: Motor = other_465;
    return MultiVector((((vec4<f32>(_e4.g0_.x) * _e8.g1_) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))) + ((_e20.g0_ * vec4<f32>(_e22.g1_.x)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))), ((vec4<f32>(_e34.g1_.x) * _e38.g1_) + ((_e41.g1_ * vec4<f32>(_e43.g1_.x)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))), (((((vec4<f32>(_e55.g2_.y) * _e59.g1_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0)) + ((vec4<f32>(_e70.g2_.z) * _e74.g1_.zzxz) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0))) + ((vec4<f32>(_e86.g2_.w) * _e90.g1_.wwwx) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))) + ((vec4<f32>(_e102.g2_.x) * vec4<f32>(_e106.g1_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))), (((((vec4<f32>(_e118.g3_.y) * _e122.g1_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0)) + ((vec4<f32>(_e133.g3_.z) * _e137.g1_.zzxz) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0))) + ((vec4<f32>(_e149.g3_.w) * _e153.g1_.wwwx) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))) + ((vec4<f32>(_e165.g3_.x) * vec4<f32>(_e169.g1_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))));
}

fn multi_vector_motor_scalar_product(self_550: MultiVector, other_466: Motor) -> Scalar {
    var self_551: MultiVector;
    var other_467: Motor;

    self_551 = self_550;
    other_467 = other_466;
    let _e4: MultiVector = self_551;
    let _e7: Motor = other_467;
    let _e11: MultiVector = self_551;
    let _e14: Motor = other_467;
    let _e19: MultiVector = self_551;
    let _e22: Motor = other_467;
    let _e27: MultiVector = self_551;
    let _e30: Motor = other_467;
    return Scalar(((((_e4.g0_.x * _e7.g0_.x) - (_e11.g0_.y * _e14.g0_.y)) - (_e19.g0_.z * _e22.g0_.z)) - (_e27.g0_.w * _e30.g0_.w)));
}

fn multi_vector_motor_anti_scalar_product(self_552: MultiVector, other_468: Motor) -> AntiScalar {
    var self_553: MultiVector;
    var other_469: Motor;

    self_553 = self_552;
    other_469 = other_468;
    let _e4: MultiVector = self_553;
    let _e7: Motor = other_469;
    let _e11: MultiVector = self_553;
    let _e14: Motor = other_469;
    let _e19: MultiVector = self_553;
    let _e22: Motor = other_469;
    let _e27: MultiVector = self_553;
    let _e30: Motor = other_469;
    return AntiScalar(((((_e4.g3_.x * _e7.g1_.x) - (_e11.g3_.y * _e14.g1_.y)) - (_e19.g3_.z * _e22.g1_.z)) - (_e27.g3_.w * _e30.g1_.w)));
}

fn multi_vector_point_and_plane_into(self_554: MultiVector) -> PointAndPlane {
    var self_555: MultiVector;

    self_555 = self_554;
    let _e2: MultiVector = self_555;
    let _e5: MultiVector = self_555;
    let _e8: MultiVector = self_555;
    let _e11: MultiVector = self_555;
    let _e15: MultiVector = self_555;
    let _e18: MultiVector = self_555;
    let _e21: MultiVector = self_555;
    let _e24: MultiVector = self_555;
    return PointAndPlane(vec4<f32>(_e2.g2_.x, _e5.g1_.y, _e8.g1_.z, _e11.g1_.w), vec4<f32>(_e15.g1_.x, _e18.g2_.y, _e21.g2_.z, _e24.g2_.w));
}

fn multi_vector_point_and_plane_add(self_556: MultiVector, other_470: PointAndPlane) -> MultiVector {
    var self_557: MultiVector;
    var other_471: PointAndPlane;

    self_557 = self_556;
    other_471 = other_470;
    let _e4: MultiVector = self_557;
    let _e6: MultiVector = self_557;
    let _e8: PointAndPlane = other_471;
    let _e11: PointAndPlane = other_471;
    let _e14: PointAndPlane = other_471;
    let _e17: PointAndPlane = other_471;
    let _e22: MultiVector = self_557;
    let _e24: PointAndPlane = other_471;
    let _e27: PointAndPlane = other_471;
    let _e30: PointAndPlane = other_471;
    let _e33: PointAndPlane = other_471;
    let _e38: MultiVector = self_557;
    return MultiVector(_e4.g0_, (_e6.g1_ + vec4<f32>(_e8.g1_.x, _e11.g0_.y, _e14.g0_.z, _e17.g0_.w)), (_e22.g2_ + vec4<f32>(_e24.g0_.x, _e27.g1_.y, _e30.g1_.z, _e33.g1_.w)), _e38.g3_);
}

fn multi_vector_point_and_plane_sub(self_558: MultiVector, other_472: PointAndPlane) -> MultiVector {
    var self_559: MultiVector;
    var other_473: PointAndPlane;

    self_559 = self_558;
    other_473 = other_472;
    let _e4: MultiVector = self_559;
    let _e6: MultiVector = self_559;
    let _e8: PointAndPlane = other_473;
    let _e11: PointAndPlane = other_473;
    let _e14: PointAndPlane = other_473;
    let _e17: PointAndPlane = other_473;
    let _e22: MultiVector = self_559;
    let _e24: PointAndPlane = other_473;
    let _e27: PointAndPlane = other_473;
    let _e30: PointAndPlane = other_473;
    let _e33: PointAndPlane = other_473;
    let _e38: MultiVector = self_559;
    return MultiVector(_e4.g0_, (_e6.g1_ - vec4<f32>(_e8.g1_.x, _e11.g0_.y, _e14.g0_.z, _e17.g0_.w)), (_e22.g2_ - vec4<f32>(_e24.g0_.x, _e27.g1_.y, _e30.g1_.z, _e33.g1_.w)), _e38.g3_);
}

fn multi_vector_point_and_plane_geometric_product(self_560: MultiVector, other_474: PointAndPlane) -> MultiVector {
    var self_561: MultiVector;
    var other_475: PointAndPlane;

    self_561 = self_560;
    other_475 = other_474;
    let _e4: MultiVector = self_561;
    let _e8: PointAndPlane = other_475;
    let _e11: PointAndPlane = other_475;
    let _e14: PointAndPlane = other_475;
    let _e17: PointAndPlane = other_475;
    let _e29: MultiVector = self_561;
    let _e33: PointAndPlane = other_475;
    let _e36: PointAndPlane = other_475;
    let _e39: PointAndPlane = other_475;
    let _e42: PointAndPlane = other_475;
    let _e55: MultiVector = self_561;
    let _e59: PointAndPlane = other_475;
    let _e62: PointAndPlane = other_475;
    let _e65: PointAndPlane = other_475;
    let _e68: PointAndPlane = other_475;
    let _e81: MultiVector = self_561;
    let _e85: PointAndPlane = other_475;
    let _e88: PointAndPlane = other_475;
    let _e91: PointAndPlane = other_475;
    let _e94: PointAndPlane = other_475;
    let _e107: MultiVector = self_561;
    let _e111: PointAndPlane = other_475;
    let _e114: PointAndPlane = other_475;
    let _e117: PointAndPlane = other_475;
    let _e120: PointAndPlane = other_475;
    let _e125: MultiVector = self_561;
    let _e129: PointAndPlane = other_475;
    let _e132: PointAndPlane = other_475;
    let _e135: PointAndPlane = other_475;
    let _e138: PointAndPlane = other_475;
    let _e152: MultiVector = self_561;
    let _e156: PointAndPlane = other_475;
    let _e159: PointAndPlane = other_475;
    let _e162: PointAndPlane = other_475;
    let _e165: PointAndPlane = other_475;
    let _e179: MultiVector = self_561;
    let _e183: PointAndPlane = other_475;
    let _e186: PointAndPlane = other_475;
    let _e189: PointAndPlane = other_475;
    let _e192: PointAndPlane = other_475;
    let _e206: MultiVector = self_561;
    let _e210: PointAndPlane = other_475;
    let _e213: PointAndPlane = other_475;
    let _e216: PointAndPlane = other_475;
    let _e219: PointAndPlane = other_475;
    let _e225: MultiVector = self_561;
    let _e229: PointAndPlane = other_475;
    let _e232: PointAndPlane = other_475;
    let _e235: PointAndPlane = other_475;
    let _e238: PointAndPlane = other_475;
    let _e252: MultiVector = self_561;
    let _e256: PointAndPlane = other_475;
    let _e259: PointAndPlane = other_475;
    let _e262: PointAndPlane = other_475;
    let _e265: PointAndPlane = other_475;
    let _e279: MultiVector = self_561;
    let _e283: PointAndPlane = other_475;
    let _e286: PointAndPlane = other_475;
    let _e289: PointAndPlane = other_475;
    let _e292: PointAndPlane = other_475;
    let _e306: MultiVector = self_561;
    let _e310: PointAndPlane = other_475;
    let _e313: PointAndPlane = other_475;
    let _e316: PointAndPlane = other_475;
    let _e319: PointAndPlane = other_475;
    let _e324: MultiVector = self_561;
    let _e328: PointAndPlane = other_475;
    let _e331: PointAndPlane = other_475;
    let _e334: PointAndPlane = other_475;
    let _e337: PointAndPlane = other_475;
    let _e351: MultiVector = self_561;
    let _e355: PointAndPlane = other_475;
    let _e358: PointAndPlane = other_475;
    let _e361: PointAndPlane = other_475;
    let _e364: PointAndPlane = other_475;
    let _e378: MultiVector = self_561;
    let _e382: PointAndPlane = other_475;
    let _e385: PointAndPlane = other_475;
    let _e388: PointAndPlane = other_475;
    let _e391: PointAndPlane = other_475;
    let _e405: MultiVector = self_561;
    let _e409: PointAndPlane = other_475;
    let _e412: PointAndPlane = other_475;
    let _e415: PointAndPlane = other_475;
    let _e418: PointAndPlane = other_475;
    let _e423: MultiVector = self_561;
    let _e427: PointAndPlane = other_475;
    let _e430: PointAndPlane = other_475;
    let _e433: PointAndPlane = other_475;
    let _e436: PointAndPlane = other_475;
    let _e450: MultiVector = self_561;
    let _e454: PointAndPlane = other_475;
    let _e457: PointAndPlane = other_475;
    let _e460: PointAndPlane = other_475;
    let _e463: PointAndPlane = other_475;
    let _e477: MultiVector = self_561;
    let _e481: PointAndPlane = other_475;
    let _e484: PointAndPlane = other_475;
    let _e487: PointAndPlane = other_475;
    let _e490: PointAndPlane = other_475;
    let _e504: MultiVector = self_561;
    let _e508: PointAndPlane = other_475;
    let _e511: PointAndPlane = other_475;
    let _e514: PointAndPlane = other_475;
    let _e517: PointAndPlane = other_475;
    let _e523: MultiVector = self_561;
    let _e527: PointAndPlane = other_475;
    let _e530: PointAndPlane = other_475;
    let _e533: PointAndPlane = other_475;
    let _e536: PointAndPlane = other_475;
    let _e550: MultiVector = self_561;
    let _e554: PointAndPlane = other_475;
    let _e557: PointAndPlane = other_475;
    let _e560: PointAndPlane = other_475;
    let _e563: PointAndPlane = other_475;
    let _e577: MultiVector = self_561;
    let _e581: PointAndPlane = other_475;
    let _e584: PointAndPlane = other_475;
    let _e587: PointAndPlane = other_475;
    let _e590: PointAndPlane = other_475;
    return MultiVector((((((vec4<f32>(_e4.g2_.x) * vec4<f32>(_e8.g0_.x, _e11.g1_.y, _e14.g1_.z, _e17.g1_.w)) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0)) + ((vec4<f32>(_e29.g2_.y) * vec4<f32>(_e33.g1_.y, _e36.g0_.x, _e39.g1_.w, _e42.g1_.z)) * vec4<f32>(1.0, 1.0, -(1.0), 1.0))) + ((vec4<f32>(_e55.g2_.z) * vec4<f32>(_e59.g1_.z, _e62.g1_.w, _e65.g0_.x, _e68.g1_.y)) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e81.g2_.w) * vec4<f32>(_e85.g1_.w, _e88.g1_.z, _e91.g1_.y, _e94.g0_.x)) * vec4<f32>(1.0, -(1.0), 1.0, 1.0))), ((((((((vec4<f32>(_e107.g0_.x) * vec4<f32>(_e111.g1_.x, _e114.g0_.y, _e117.g0_.z, _e120.g0_.w)) + ((vec4<f32>(_e125.g0_.y) * vec4<f32>(_e129.g0_.y, _e132.g1_.x, _e135.g0_.w, _e138.g0_.z)) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e152.g0_.z) * vec4<f32>(_e156.g0_.z, _e159.g0_.w, _e162.g1_.x, _e165.g0_.y)) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e179.g0_.w) * vec4<f32>(_e183.g0_.w, _e186.g0_.z, _e189.g0_.y, _e192.g1_.x)) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))) - (vec4<f32>(_e206.g3_.x) * vec4<f32>(_e210.g0_.x, _e213.g1_.y, _e216.g1_.z, _e219.g1_.w))) + ((vec4<f32>(_e225.g3_.y) * vec4<f32>(_e229.g1_.y, _e232.g0_.x, _e235.g1_.w, _e238.g1_.z)) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e252.g3_.z) * vec4<f32>(_e256.g1_.z, _e259.g1_.w, _e262.g0_.x, _e265.g1_.y)) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e279.g3_.w) * vec4<f32>(_e283.g1_.w, _e286.g1_.z, _e289.g1_.y, _e292.g0_.x)) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))), ((((vec4<f32>(_e306.g0_.x) * vec4<f32>(_e310.g0_.x, _e313.g1_.y, _e316.g1_.z, _e319.g1_.w)) + ((vec4<f32>(_e324.g0_.y) * vec4<f32>(_e328.g1_.y, _e331.g0_.x, _e334.g1_.w, _e337.g1_.z)) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e351.g0_.z) * vec4<f32>(_e355.g1_.z, _e358.g1_.w, _e361.g0_.x, _e364.g1_.y)) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e378.g0_.w) * vec4<f32>(_e382.g1_.w, _e385.g1_.z, _e388.g1_.y, _e391.g0_.x)) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))), ((((((((vec4<f32>(_e405.g1_.x) * vec4<f32>(_e409.g0_.x, _e412.g1_.y, _e415.g1_.z, _e418.g1_.w)) + ((vec4<f32>(_e423.g1_.y) * vec4<f32>(_e427.g1_.y, _e430.g0_.x, _e433.g1_.w, _e436.g1_.z)) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))) + ((vec4<f32>(_e450.g1_.z) * vec4<f32>(_e454.g1_.z, _e457.g1_.w, _e460.g0_.x, _e463.g1_.y)) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e477.g1_.w) * vec4<f32>(_e481.g1_.w, _e484.g1_.z, _e487.g1_.y, _e490.g0_.x)) * vec4<f32>(-(1.0), -(1.0), 1.0, 1.0))) - (vec4<f32>(_e504.g2_.x) * vec4<f32>(_e508.g1_.x, _e511.g0_.y, _e514.g0_.z, _e517.g0_.w))) + ((vec4<f32>(_e523.g2_.y) * vec4<f32>(_e527.g0_.y, _e530.g1_.x, _e533.g0_.w, _e536.g0_.z)) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e550.g2_.z) * vec4<f32>(_e554.g0_.z, _e557.g0_.w, _e560.g1_.x, _e563.g0_.y)) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e577.g2_.w) * vec4<f32>(_e581.g0_.w, _e584.g0_.z, _e587.g0_.y, _e590.g1_.x)) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))));
}

fn multi_vector_point_and_plane_geometric_anti_product(self_562: MultiVector, other_476: PointAndPlane) -> MultiVector {
    var self_563: MultiVector;
    var other_477: PointAndPlane;

    self_563 = self_562;
    other_477 = other_476;
    let _e4: MultiVector = self_563;
    let _e8: PointAndPlane = other_477;
    let _e11: PointAndPlane = other_477;
    let _e14: PointAndPlane = other_477;
    let _e17: PointAndPlane = other_477;
    let _e31: MultiVector = self_563;
    let _e35: PointAndPlane = other_477;
    let _e38: PointAndPlane = other_477;
    let _e41: PointAndPlane = other_477;
    let _e44: PointAndPlane = other_477;
    let _e59: MultiVector = self_563;
    let _e63: PointAndPlane = other_477;
    let _e66: PointAndPlane = other_477;
    let _e69: PointAndPlane = other_477;
    let _e72: PointAndPlane = other_477;
    let _e87: MultiVector = self_563;
    let _e91: PointAndPlane = other_477;
    let _e94: PointAndPlane = other_477;
    let _e97: PointAndPlane = other_477;
    let _e100: PointAndPlane = other_477;
    let _e115: MultiVector = self_563;
    let _e119: PointAndPlane = other_477;
    let _e122: PointAndPlane = other_477;
    let _e125: PointAndPlane = other_477;
    let _e128: PointAndPlane = other_477;
    let _e141: MultiVector = self_563;
    let _e145: PointAndPlane = other_477;
    let _e148: PointAndPlane = other_477;
    let _e151: PointAndPlane = other_477;
    let _e154: PointAndPlane = other_477;
    let _e167: MultiVector = self_563;
    let _e171: PointAndPlane = other_477;
    let _e174: PointAndPlane = other_477;
    let _e177: PointAndPlane = other_477;
    let _e180: PointAndPlane = other_477;
    let _e193: MultiVector = self_563;
    let _e197: PointAndPlane = other_477;
    let _e200: PointAndPlane = other_477;
    let _e203: PointAndPlane = other_477;
    let _e206: PointAndPlane = other_477;
    let _e219: MultiVector = self_563;
    let _e223: PointAndPlane = other_477;
    let _e226: PointAndPlane = other_477;
    let _e229: PointAndPlane = other_477;
    let _e232: PointAndPlane = other_477;
    let _e237: MultiVector = self_563;
    let _e241: PointAndPlane = other_477;
    let _e244: PointAndPlane = other_477;
    let _e247: PointAndPlane = other_477;
    let _e250: PointAndPlane = other_477;
    let _e264: MultiVector = self_563;
    let _e268: PointAndPlane = other_477;
    let _e271: PointAndPlane = other_477;
    let _e274: PointAndPlane = other_477;
    let _e277: PointAndPlane = other_477;
    let _e291: MultiVector = self_563;
    let _e295: PointAndPlane = other_477;
    let _e298: PointAndPlane = other_477;
    let _e301: PointAndPlane = other_477;
    let _e304: PointAndPlane = other_477;
    let _e318: MultiVector = self_563;
    let _e322: PointAndPlane = other_477;
    let _e325: PointAndPlane = other_477;
    let _e328: PointAndPlane = other_477;
    let _e331: PointAndPlane = other_477;
    let _e336: MultiVector = self_563;
    let _e340: PointAndPlane = other_477;
    let _e343: PointAndPlane = other_477;
    let _e346: PointAndPlane = other_477;
    let _e349: PointAndPlane = other_477;
    let _e363: MultiVector = self_563;
    let _e367: PointAndPlane = other_477;
    let _e370: PointAndPlane = other_477;
    let _e373: PointAndPlane = other_477;
    let _e376: PointAndPlane = other_477;
    let _e390: MultiVector = self_563;
    let _e394: PointAndPlane = other_477;
    let _e397: PointAndPlane = other_477;
    let _e400: PointAndPlane = other_477;
    let _e403: PointAndPlane = other_477;
    let _e417: MultiVector = self_563;
    let _e421: PointAndPlane = other_477;
    let _e424: PointAndPlane = other_477;
    let _e427: PointAndPlane = other_477;
    let _e430: PointAndPlane = other_477;
    let _e436: MultiVector = self_563;
    let _e440: PointAndPlane = other_477;
    let _e443: PointAndPlane = other_477;
    let _e446: PointAndPlane = other_477;
    let _e449: PointAndPlane = other_477;
    let _e463: MultiVector = self_563;
    let _e467: PointAndPlane = other_477;
    let _e470: PointAndPlane = other_477;
    let _e473: PointAndPlane = other_477;
    let _e476: PointAndPlane = other_477;
    let _e490: MultiVector = self_563;
    let _e494: PointAndPlane = other_477;
    let _e497: PointAndPlane = other_477;
    let _e500: PointAndPlane = other_477;
    let _e503: PointAndPlane = other_477;
    let _e519: MultiVector = self_563;
    let _e523: PointAndPlane = other_477;
    let _e526: PointAndPlane = other_477;
    let _e529: PointAndPlane = other_477;
    let _e532: PointAndPlane = other_477;
    let _e538: MultiVector = self_563;
    let _e542: PointAndPlane = other_477;
    let _e545: PointAndPlane = other_477;
    let _e548: PointAndPlane = other_477;
    let _e551: PointAndPlane = other_477;
    let _e565: MultiVector = self_563;
    let _e569: PointAndPlane = other_477;
    let _e572: PointAndPlane = other_477;
    let _e575: PointAndPlane = other_477;
    let _e578: PointAndPlane = other_477;
    let _e592: MultiVector = self_563;
    let _e596: PointAndPlane = other_477;
    let _e599: PointAndPlane = other_477;
    let _e602: PointAndPlane = other_477;
    let _e605: PointAndPlane = other_477;
    return MultiVector((((((((((vec4<f32>(_e4.g1_.x) * vec4<f32>(_e8.g0_.x, _e11.g1_.y, _e14.g1_.z, _e17.g1_.w)) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))) + ((vec4<f32>(_e31.g1_.y) * vec4<f32>(_e35.g1_.y, _e38.g0_.x, _e41.g1_.w, _e44.g1_.z)) * vec4<f32>(-(1.0), -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e59.g1_.z) * vec4<f32>(_e63.g1_.z, _e66.g1_.w, _e69.g0_.x, _e72.g1_.y)) * vec4<f32>(-(1.0), 1.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e87.g1_.w) * vec4<f32>(_e91.g1_.w, _e94.g1_.z, _e97.g1_.y, _e100.g0_.x)) * vec4<f32>(-(1.0), -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e115.g2_.x) * vec4<f32>(_e119.g1_.x, _e122.g0_.y, _e125.g0_.z, _e128.g0_.w)) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0))) + ((vec4<f32>(_e141.g2_.y) * vec4<f32>(_e145.g0_.y, _e148.g1_.x, _e151.g0_.w, _e154.g0_.z)) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e167.g2_.z) * vec4<f32>(_e171.g0_.z, _e174.g0_.w, _e177.g1_.x, _e180.g0_.y)) * vec4<f32>(1.0, -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e193.g2_.w) * vec4<f32>(_e197.g0_.w, _e200.g0_.z, _e203.g0_.y, _e206.g1_.x)) * vec4<f32>(1.0, 1.0, -(1.0), 1.0))), ((((vec4<f32>(_e219.g3_.x) * vec4<f32>(_e223.g1_.x, _e226.g0_.y, _e229.g0_.z, _e232.g0_.w)) + ((vec4<f32>(_e237.g3_.y) * vec4<f32>(_e241.g0_.y, _e244.g1_.x, _e247.g0_.w, _e250.g0_.z)) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e264.g3_.z) * vec4<f32>(_e268.g0_.z, _e271.g0_.w, _e274.g1_.x, _e277.g0_.y)) * vec4<f32>(-(1.0), -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e291.g3_.w) * vec4<f32>(_e295.g0_.w, _e298.g0_.z, _e301.g0_.y, _e304.g1_.x)) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))), ((((((((vec4<f32>(_e318.g0_.x) * vec4<f32>(_e322.g1_.x, _e325.g0_.y, _e328.g0_.z, _e331.g0_.w)) + ((vec4<f32>(_e336.g0_.y) * vec4<f32>(_e340.g0_.y, _e343.g1_.x, _e346.g0_.w, _e349.g0_.z)) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e363.g0_.z) * vec4<f32>(_e367.g0_.z, _e370.g0_.w, _e373.g1_.x, _e376.g0_.y)) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e390.g0_.w) * vec4<f32>(_e394.g0_.w, _e397.g0_.z, _e400.g0_.y, _e403.g1_.x)) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))) + (vec4<f32>(_e417.g3_.x) * vec4<f32>(_e421.g0_.x, _e424.g1_.y, _e427.g1_.z, _e430.g1_.w))) + ((vec4<f32>(_e436.g3_.y) * vec4<f32>(_e440.g1_.y, _e443.g0_.x, _e446.g1_.w, _e449.g1_.z)) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e463.g3_.z) * vec4<f32>(_e467.g1_.z, _e470.g1_.w, _e473.g0_.x, _e476.g1_.y)) * vec4<f32>(-(1.0), -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e490.g3_.w) * vec4<f32>(_e494.g1_.w, _e497.g1_.z, _e500.g1_.y, _e503.g0_.x)) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))), ((((vec4<f32>(0.0) - (vec4<f32>(_e519.g1_.x) * vec4<f32>(_e523.g1_.x, _e526.g0_.y, _e529.g0_.z, _e532.g0_.w))) + ((vec4<f32>(_e538.g1_.y) * vec4<f32>(_e542.g0_.y, _e545.g1_.x, _e548.g0_.w, _e551.g0_.z)) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e565.g1_.z) * vec4<f32>(_e569.g0_.z, _e572.g0_.w, _e575.g1_.x, _e578.g0_.y)) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e592.g1_.w) * vec4<f32>(_e596.g0_.w, _e599.g0_.z, _e602.g0_.y, _e605.g1_.x)) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))));
}

fn multi_vector_point_and_plane_scalar_product(self_564: MultiVector, other_478: PointAndPlane) -> Scalar {
    var self_565: MultiVector;
    var other_479: PointAndPlane;

    self_565 = self_564;
    other_479 = other_478;
    let _e5: MultiVector = self_565;
    let _e8: PointAndPlane = other_479;
    let _e13: MultiVector = self_565;
    let _e16: PointAndPlane = other_479;
    let _e21: MultiVector = self_565;
    let _e24: PointAndPlane = other_479;
    let _e29: MultiVector = self_565;
    let _e32: PointAndPlane = other_479;
    return Scalar(((((0.0 - (_e5.g2_.x * _e8.g0_.x)) + (_e13.g2_.y * _e16.g1_.y)) + (_e21.g2_.z * _e24.g1_.z)) + (_e29.g2_.w * _e32.g1_.w)));
}

fn multi_vector_point_and_plane_anti_scalar_product(self_566: MultiVector, other_480: PointAndPlane) -> AntiScalar {
    var self_567: MultiVector;
    var other_481: PointAndPlane;

    self_567 = self_566;
    other_481 = other_480;
    let _e5: MultiVector = self_567;
    let _e8: PointAndPlane = other_481;
    let _e13: MultiVector = self_567;
    let _e16: PointAndPlane = other_481;
    let _e21: MultiVector = self_567;
    let _e24: PointAndPlane = other_481;
    let _e29: MultiVector = self_567;
    let _e32: PointAndPlane = other_481;
    return AntiScalar(((((0.0 - (_e5.g1_.x * _e8.g1_.x)) + (_e13.g1_.y * _e16.g0_.y)) + (_e21.g1_.z * _e24.g0_.z)) + (_e29.g1_.w * _e32.g0_.w)));
}

fn multi_vector_squared_magnitude(self_568: MultiVector) -> Scalar {
    var self_569: MultiVector;

    self_569 = self_568;
    let _e2: MultiVector = self_569;
    let _e3: MultiVector = self_569;
    let _e4: MultiVector = multi_vector_reversal(_e3);
    let _e5: Scalar = multi_vector_multi_vector_scalar_product(_e2, _e4);
    return _e5;
}

fn multi_vector_magnitude(self_570: MultiVector) -> Scalar {
    var self_571: MultiVector;

    self_571 = self_570;
    let _e2: MultiVector = self_571;
    let _e3: Scalar = multi_vector_squared_magnitude(_e2);
    return Scalar(sqrt(_e3.g0_));
}

fn multi_vector_bulk_norm(self_572: MultiVector) -> Scalar {
    var self_573: MultiVector;

    self_573 = self_572;
    let _e2: MultiVector = self_573;
    let _e3: Scalar = multi_vector_squared_magnitude(_e2);
    return Scalar(sqrt(_e3.g0_));
}

fn multi_vector_squared_anti_magnitude(self_574: MultiVector) -> AntiScalar {
    var self_575: MultiVector;

    self_575 = self_574;
    let _e2: MultiVector = self_575;
    let _e3: MultiVector = self_575;
    let _e4: MultiVector = multi_vector_anti_reversal(_e3);
    let _e5: AntiScalar = multi_vector_multi_vector_anti_scalar_product(_e2, _e4);
    return _e5;
}

fn multi_vector_weight_norm(self_576: MultiVector) -> AntiScalar {
    var self_577: MultiVector;

    self_577 = self_576;
    let _e2: MultiVector = self_577;
    let _e3: AntiScalar = multi_vector_squared_anti_magnitude(_e2);
    return AntiScalar(sqrt(_e3.g0_));
}

fn multi_vector_scale(self_578: MultiVector, other_482: f32) -> MultiVector {
    var self_579: MultiVector;
    var other_483: f32;

    self_579 = self_578;
    other_483 = other_482;
    let _e4: MultiVector = self_579;
    let _e5: f32 = other_483;
    let _e7: MultiVector = multi_vector_scalar_geometric_product(_e4, Scalar(_e5));
    return _e7;
}

fn multi_vector_signum(self_580: MultiVector) -> MultiVector {
    var self_581: MultiVector;

    self_581 = self_580;
    let _e2: MultiVector = self_581;
    let _e3: MultiVector = self_581;
    let _e4: Scalar = multi_vector_magnitude(_e3);
    let _e9: MultiVector = multi_vector_scalar_geometric_product(_e2, Scalar((1.0 / _e4.g0_)));
    return _e9;
}

fn multi_vector_inverse(self_582: MultiVector) -> MultiVector {
    var self_583: MultiVector;

    self_583 = self_582;
    let _e2: MultiVector = self_583;
    let _e3: MultiVector = multi_vector_reversal(_e2);
    let _e4: MultiVector = self_583;
    let _e5: Scalar = multi_vector_squared_magnitude(_e4);
    let _e10: MultiVector = multi_vector_scalar_geometric_product(_e3, Scalar((1.0 / _e5.g0_)));
    return _e10;
}

fn multi_vector_unitize(self_584: MultiVector) -> MultiVector {
    var self_585: MultiVector;

    self_585 = self_584;
    let _e2: MultiVector = self_585;
    let _e3: MultiVector = self_585;
    let _e4: AntiScalar = multi_vector_weight_norm(_e3);
    let _e9: MultiVector = multi_vector_scalar_geometric_product(_e2, Scalar((1.0 / _e4.g0_)));
    return _e9;
}

fn rotor_zero() -> Rotor {
    return Rotor(vec4<f32>(0.0));
}

fn rotor_one() -> Rotor {
    return Rotor(vec4<f32>(1.0, 0.0, 0.0, 0.0));
}

fn rotor_neg(self_586: Rotor) -> Rotor {
    var self_587: Rotor;

    self_587 = self_586;
    let _e2: Rotor = self_587;
    return Rotor((_e2.g0_ * vec4<f32>(-(1.0))));
}

fn rotor_automorphism(self_588: Rotor) -> Rotor {
    var self_589: Rotor;

    self_589 = self_588;
    let _e2: Rotor = self_589;
    return Rotor(_e2.g0_);
}

fn rotor_reversal(self_590: Rotor) -> Rotor {
    var self_591: Rotor;

    self_591 = self_590;
    let _e2: Rotor = self_591;
    return Rotor((_e2.g0_ * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))));
}

fn rotor_conjugation(self_592: Rotor) -> Rotor {
    var self_593: Rotor;

    self_593 = self_592;
    let _e2: Rotor = self_593;
    return Rotor((_e2.g0_ * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))));
}

fn rotor_anti_reversal(self_594: Rotor) -> Rotor {
    var self_595: Rotor;

    self_595 = self_594;
    let _e2: Rotor = self_595;
    return Rotor((_e2.g0_ * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))));
}

fn rotor_double_complement(self_596: Rotor) -> Rotor {
    var self_597: Rotor;

    self_597 = self_596;
    let _e2: Rotor = self_597;
    return Rotor(_e2.g0_);
}

fn rotor_scalar_into(self_598: Rotor) -> Scalar {
    var self_599: Rotor;

    self_599 = self_598;
    let _e2: Rotor = self_599;
    return Scalar(_e2.g0_.x);
}

fn rotor_scalar_add(self_600: Rotor, other_484: Scalar) -> Rotor {
    var self_601: Rotor;
    var other_485: Scalar;

    self_601 = self_600;
    other_485 = other_484;
    let _e4: Rotor = self_601;
    let _e6: Scalar = other_485;
    return Rotor((_e4.g0_ + (vec4<f32>(_e6.g0_) * vec4<f32>(1.0, 0.0, 0.0, 0.0))));
}

fn rotor_scalar_sub(self_602: Rotor, other_486: Scalar) -> Rotor {
    var self_603: Rotor;
    var other_487: Scalar;

    self_603 = self_602;
    other_487 = other_486;
    let _e4: Rotor = self_603;
    let _e6: Scalar = other_487;
    return Rotor((_e4.g0_ - (vec4<f32>(_e6.g0_) * vec4<f32>(1.0, 0.0, 0.0, 0.0))));
}

fn rotor_scalar_geometric_product(self_604: Rotor, other_488: Scalar) -> Rotor {
    var self_605: Rotor;
    var other_489: Scalar;

    self_605 = self_604;
    other_489 = other_488;
    let _e4: Rotor = self_605;
    let _e6: Scalar = other_489;
    return Rotor((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn rotor_scalar_outer_product(self_606: Rotor, other_490: Scalar) -> Rotor {
    var self_607: Rotor;
    var other_491: Scalar;

    self_607 = self_606;
    other_491 = other_490;
    let _e4: Rotor = self_607;
    let _e6: Scalar = other_491;
    return Rotor((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn rotor_scalar_inner_product(self_608: Rotor, other_492: Scalar) -> Rotor {
    var self_609: Rotor;
    var other_493: Scalar;

    self_609 = self_608;
    other_493 = other_492;
    let _e4: Rotor = self_609;
    let _e6: Scalar = other_493;
    return Rotor((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn rotor_scalar_left_contraction(self_610: Rotor, other_494: Scalar) -> Scalar {
    var self_611: Rotor;
    var other_495: Scalar;

    self_611 = self_610;
    other_495 = other_494;
    let _e4: Rotor = self_611;
    let _e7: Scalar = other_495;
    return Scalar((_e4.g0_.x * _e7.g0_));
}

fn rotor_scalar_right_contraction(self_612: Rotor, other_496: Scalar) -> Rotor {
    var self_613: Rotor;
    var other_497: Scalar;

    self_613 = self_612;
    other_497 = other_496;
    let _e4: Rotor = self_613;
    let _e6: Scalar = other_497;
    return Rotor((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn rotor_scalar_scalar_product(self_614: Rotor, other_498: Scalar) -> Scalar {
    var self_615: Rotor;
    var other_499: Scalar;

    self_615 = self_614;
    other_499 = other_498;
    let _e4: Rotor = self_615;
    let _e7: Scalar = other_499;
    return Scalar((_e4.g0_.x * _e7.g0_));
}

fn rotor_anti_scalar_regressive_product(self_616: Rotor, other_500: AntiScalar) -> Rotor {
    var self_617: Rotor;
    var other_501: AntiScalar;

    self_617 = self_616;
    other_501 = other_500;
    let _e4: Rotor = self_617;
    let _e6: AntiScalar = other_501;
    return Rotor((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn rotor_anti_scalar_outer_product(self_618: Rotor, other_502: AntiScalar) -> AntiScalar {
    var self_619: Rotor;
    var other_503: AntiScalar;

    self_619 = self_618;
    other_503 = other_502;
    let _e4: Rotor = self_619;
    let _e7: AntiScalar = other_503;
    return AntiScalar((_e4.g0_.x * _e7.g0_));
}

fn rotor_anti_scalar_geometric_anti_product(self_620: Rotor, other_504: AntiScalar) -> Rotor {
    var self_621: Rotor;
    var other_505: AntiScalar;

    self_621 = self_620;
    other_505 = other_504;
    let _e4: Rotor = self_621;
    let _e6: AntiScalar = other_505;
    return Rotor((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn rotor_anti_scalar_inner_anti_product(self_622: Rotor, other_506: AntiScalar) -> Rotor {
    var self_623: Rotor;
    var other_507: AntiScalar;

    self_623 = self_622;
    other_507 = other_506;
    let _e4: Rotor = self_623;
    let _e6: AntiScalar = other_507;
    return Rotor((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn rotor_anti_scalar_right_anti_contraction(self_624: Rotor, other_508: AntiScalar) -> Rotor {
    var self_625: Rotor;
    var other_509: AntiScalar;

    self_625 = self_624;
    other_509 = other_508;
    let _e4: Rotor = self_625;
    let _e6: AntiScalar = other_509;
    return Rotor((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn rotor_multi_vector_add(self_626: Rotor, other_510: MultiVector) -> MultiVector {
    var self_627: Rotor;
    var other_511: MultiVector;

    self_627 = self_626;
    other_511 = other_510;
    let _e4: Rotor = self_627;
    let _e6: MultiVector = other_511;
    let _e9: MultiVector = other_511;
    let _e11: MultiVector = other_511;
    let _e13: MultiVector = other_511;
    return MultiVector((_e4.g0_ + _e6.g0_), _e9.g1_, _e11.g2_, _e13.g3_);
}

fn rotor_multi_vector_sub(self_628: Rotor, other_512: MultiVector) -> MultiVector {
    var self_629: Rotor;
    var other_513: MultiVector;

    self_629 = self_628;
    other_513 = other_512;
    let _e4: Rotor = self_629;
    let _e6: MultiVector = other_513;
    let _e11: MultiVector = other_513;
    let _e16: MultiVector = other_513;
    let _e21: MultiVector = other_513;
    return MultiVector((_e4.g0_ - _e6.g0_), (vec4<f32>(0.0) - _e11.g1_), (vec4<f32>(0.0) - _e16.g2_), (vec4<f32>(0.0) - _e21.g3_));
}

fn rotor_multi_vector_geometric_product(self_630: Rotor, other_514: MultiVector) -> MultiVector {
    var self_631: Rotor;
    var other_515: MultiVector;

    self_631 = self_630;
    other_515 = other_514;
    let _e4: Rotor = self_631;
    let _e8: MultiVector = other_515;
    let _e11: Rotor = self_631;
    let _e15: MultiVector = other_515;
    let _e28: Rotor = self_631;
    let _e32: MultiVector = other_515;
    let _e45: Rotor = self_631;
    let _e49: MultiVector = other_515;
    let _e62: Rotor = self_631;
    let _e66: MultiVector = other_515;
    let _e69: Rotor = self_631;
    let _e73: MultiVector = other_515;
    let _e86: Rotor = self_631;
    let _e90: MultiVector = other_515;
    let _e103: Rotor = self_631;
    let _e107: MultiVector = other_515;
    let _e120: Rotor = self_631;
    let _e124: MultiVector = other_515;
    let _e127: Rotor = self_631;
    let _e131: MultiVector = other_515;
    let _e144: Rotor = self_631;
    let _e148: MultiVector = other_515;
    let _e161: Rotor = self_631;
    let _e165: MultiVector = other_515;
    let _e178: Rotor = self_631;
    let _e182: MultiVector = other_515;
    let _e185: Rotor = self_631;
    let _e189: MultiVector = other_515;
    let _e202: Rotor = self_631;
    let _e206: MultiVector = other_515;
    let _e219: Rotor = self_631;
    let _e223: MultiVector = other_515;
    return MultiVector(((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.y) * _e15.g0_.yxwz) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e28.g0_.z) * _e32.g0_.zwxy) * vec4<f32>(-(1.0), -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e45.g0_.w) * _e49.g0_.wzyx) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))), ((((vec4<f32>(_e62.g0_.x) * _e66.g1_) + ((vec4<f32>(_e69.g0_.y) * _e73.g1_.yxwz) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e86.g0_.z) * _e90.g1_.zwxy) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e103.g0_.w) * _e107.g1_.wzyx) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))), ((((vec4<f32>(_e120.g0_.x) * _e124.g2_) + ((vec4<f32>(_e127.g0_.y) * _e131.g2_.yxwz) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e144.g0_.z) * _e148.g2_.zwxy) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e161.g0_.w) * _e165.g2_.wzyx) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))), ((((vec4<f32>(_e178.g0_.x) * _e182.g3_) + ((vec4<f32>(_e185.g0_.y) * _e189.g3_.yxwz) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e202.g0_.z) * _e206.g3_.zwxy) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e219.g0_.w) * _e223.g3_.wzyx) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))));
}

fn rotor_multi_vector_outer_product(self_632: Rotor, other_516: MultiVector) -> MultiVector {
    var self_633: Rotor;
    var other_517: MultiVector;

    self_633 = self_632;
    other_517 = other_516;
    let _e4: Rotor = self_633;
    let _e8: MultiVector = other_517;
    let _e11: Rotor = self_633;
    let _e13: MultiVector = other_517;
    let _e25: Rotor = self_633;
    let _e29: MultiVector = other_517;
    let _e32: Rotor = self_633;
    let _e34: MultiVector = other_517;
    let _e49: Rotor = self_633;
    let _e53: MultiVector = other_517;
    let _e56: Rotor = self_633;
    let _e60: MultiVector = other_517;
    let _e72: Rotor = self_633;
    let _e76: MultiVector = other_517;
    let _e88: Rotor = self_633;
    let _e91: MultiVector = other_517;
    let _e102: Rotor = self_633;
    let _e106: MultiVector = other_517;
    let _e109: Rotor = self_633;
    let _e113: MultiVector = other_517;
    let _e125: Rotor = self_633;
    let _e129: MultiVector = other_517;
    let _e141: Rotor = self_633;
    let _e144: MultiVector = other_517;
    return MultiVector(((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((_e11.g0_ * vec4<f32>(_e13.g0_.x)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))), ((vec4<f32>(_e25.g0_.x) * _e29.g1_) + ((_e32.g0_ * vec4<f32>(_e34.g1_.x)) * vec4<f32>(0.0, -(1.0), -(1.0), -(1.0)))), ((((vec4<f32>(_e49.g0_.x) * _e53.g2_) + ((vec4<f32>(_e56.g0_.z) * vec4<f32>(_e60.g2_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e72.g0_.w) * vec4<f32>(_e76.g2_.w)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((_e88.g0_.yxxx * _e91.g2_.yxxx) * vec4<f32>(1.0, 0.0, 0.0, 0.0))), ((((vec4<f32>(_e102.g0_.x) * _e106.g3_) + ((vec4<f32>(_e109.g0_.z) * vec4<f32>(_e113.g3_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e125.g0_.w) * vec4<f32>(_e129.g3_.w)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((_e141.g0_.yxxx * _e144.g3_.yxxx) * vec4<f32>(1.0, 0.0, 0.0, 0.0))));
}

fn rotor_multi_vector_inner_product(self_634: Rotor, other_518: MultiVector) -> MultiVector {
    var self_635: Rotor;
    var other_519: MultiVector;

    self_635 = self_634;
    other_519 = other_518;
    let _e4: Rotor = self_635;
    let _e8: MultiVector = other_519;
    let _e11: Rotor = self_635;
    let _e15: MultiVector = other_519;
    let _e27: Rotor = self_635;
    let _e31: MultiVector = other_519;
    let _e43: Rotor = self_635;
    let _e46: MultiVector = other_519;
    let _e58: Rotor = self_635;
    let _e62: MultiVector = other_519;
    let _e65: Rotor = self_635;
    let _e69: MultiVector = other_519;
    let _e81: Rotor = self_635;
    let _e85: MultiVector = other_519;
    let _e97: Rotor = self_635;
    let _e100: MultiVector = other_519;
    let _e111: Rotor = self_635;
    let _e115: MultiVector = other_519;
    let _e118: Rotor = self_635;
    let _e122: MultiVector = other_519;
    let _e135: Rotor = self_635;
    let _e139: MultiVector = other_519;
    let _e152: Rotor = self_635;
    let _e155: MultiVector = other_519;
    let _e168: Rotor = self_635;
    let _e172: MultiVector = other_519;
    let _e175: Rotor = self_635;
    let _e177: MultiVector = other_519;
    return MultiVector(((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.z) * _e15.g0_.zzxz) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0))) + ((vec4<f32>(_e27.g0_.w) * _e31.g0_.wwwx) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))) + ((_e43.g0_.yyxx * _e46.g0_.yxxx) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))), ((((vec4<f32>(_e58.g0_.x) * _e62.g1_) + ((vec4<f32>(_e65.g0_.z) * vec4<f32>(_e69.g1_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e81.g0_.w) * vec4<f32>(_e85.g1_.w)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((_e97.g0_.yxxx * _e100.g1_.yxxx) * vec4<f32>(1.0, 0.0, 0.0, 0.0))), ((((vec4<f32>(_e111.g0_.x) * _e115.g2_) + ((vec4<f32>(_e118.g0_.z) * _e122.g2_.wwxy) * vec4<f32>(0.0, -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e135.g0_.w) * _e139.g2_.zzyx) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))) + ((_e152.g0_.xyyy * _e155.g2_.xxwz) * vec4<f32>(0.0, -(1.0), 1.0, -(1.0)))), ((vec4<f32>(_e168.g0_.x) * _e172.g3_) + ((_e175.g0_ * vec4<f32>(_e177.g3_.x)) * vec4<f32>(0.0, -(1.0), -(1.0), -(1.0)))));
}

fn rotor_multi_vector_left_contraction(self_636: Rotor, other_520: MultiVector) -> MultiVector {
    var self_637: Rotor;
    var other_521: MultiVector;

    self_637 = self_636;
    other_521 = other_520;
    let _e4: Rotor = self_637;
    let _e8: MultiVector = other_521;
    let _e11: Rotor = self_637;
    let _e15: MultiVector = other_521;
    let _e28: Rotor = self_637;
    let _e32: MultiVector = other_521;
    let _e45: Rotor = self_637;
    let _e48: MultiVector = other_521;
    let _e60: Rotor = self_637;
    let _e64: MultiVector = other_521;
    let _e67: Rotor = self_637;
    let _e71: MultiVector = other_521;
    let _e83: Rotor = self_637;
    let _e87: MultiVector = other_521;
    let _e99: Rotor = self_637;
    let _e102: MultiVector = other_521;
    let _e113: Rotor = self_637;
    let _e117: MultiVector = other_521;
    let _e120: Rotor = self_637;
    let _e122: MultiVector = other_521;
    let _e137: Rotor = self_637;
    let _e141: MultiVector = other_521;
    let _e144: Rotor = self_637;
    let _e146: MultiVector = other_521;
    return MultiVector(((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.z) * vec4<f32>(_e15.g0_.z)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((vec4<f32>(_e28.g0_.w) * vec4<f32>(_e32.g0_.w)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((_e45.g0_.yxxx * _e48.g0_.yxxx) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))), ((((vec4<f32>(_e60.g0_.x) * _e64.g1_) + ((vec4<f32>(_e67.g0_.z) * vec4<f32>(_e71.g1_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e83.g0_.w) * vec4<f32>(_e87.g1_.w)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((_e99.g0_.yxxx * _e102.g1_.yxxx) * vec4<f32>(1.0, 0.0, 0.0, 0.0))), ((vec4<f32>(_e113.g0_.x) * _e117.g2_) + ((_e120.g0_ * vec4<f32>(_e122.g2_.x)) * vec4<f32>(0.0, -(1.0), -(1.0), -(1.0)))), ((vec4<f32>(_e137.g0_.x) * _e141.g3_) + ((_e144.g0_ * vec4<f32>(_e146.g3_.x)) * vec4<f32>(0.0, -(1.0), -(1.0), -(1.0)))));
}

fn rotor_multi_vector_scalar_product(self_638: Rotor, other_522: MultiVector) -> Scalar {
    var self_639: Rotor;
    var other_523: MultiVector;

    self_639 = self_638;
    other_523 = other_522;
    let _e4: Rotor = self_639;
    let _e7: MultiVector = other_523;
    let _e11: Rotor = self_639;
    let _e14: MultiVector = other_523;
    let _e19: Rotor = self_639;
    let _e22: MultiVector = other_523;
    let _e27: Rotor = self_639;
    let _e30: MultiVector = other_523;
    return Scalar(((((_e4.g0_.x * _e7.g0_.x) - (_e11.g0_.y * _e14.g0_.y)) - (_e19.g0_.z * _e22.g0_.z)) - (_e27.g0_.w * _e30.g0_.w)));
}

fn rotor_rotor_add(self_640: Rotor, other_524: Rotor) -> Rotor {
    var self_641: Rotor;
    var other_525: Rotor;

    self_641 = self_640;
    other_525 = other_524;
    let _e4: Rotor = self_641;
    let _e6: Rotor = other_525;
    return Rotor((_e4.g0_ + _e6.g0_));
}

fn rotor_rotor_sub(self_642: Rotor, other_526: Rotor) -> Rotor {
    var self_643: Rotor;
    var other_527: Rotor;

    self_643 = self_642;
    other_527 = other_526;
    let _e4: Rotor = self_643;
    let _e6: Rotor = other_527;
    return Rotor((_e4.g0_ - _e6.g0_));
}

fn rotor_rotor_mul(self_644: Rotor, other_528: Rotor) -> Rotor {
    var self_645: Rotor;
    var other_529: Rotor;

    self_645 = self_644;
    other_529 = other_528;
    let _e4: Rotor = self_645;
    let _e6: Rotor = other_529;
    return Rotor((_e4.g0_ * _e6.g0_));
}

fn rotor_rotor_div(self_646: Rotor, other_530: Rotor) -> Rotor {
    var self_647: Rotor;
    var other_531: Rotor;

    self_647 = self_646;
    other_531 = other_530;
    let _e4: Rotor = self_647;
    let _e7: Rotor = self_647;
    let _e10: Rotor = self_647;
    let _e13: Rotor = self_647;
    let _e23: Rotor = other_531;
    let _e26: Rotor = other_531;
    let _e29: Rotor = other_531;
    let _e32: Rotor = other_531;
    return Rotor((((vec4<f32>(_e4.g0_.x, _e7.g0_.y, _e10.g0_.z, _e13.g0_.w) * vec4<f32>(1.0, 1.0, 1.0, 1.0)) / vec4<f32>(_e23.g0_.x, _e26.g0_.y, _e29.g0_.z, _e32.g0_.w)) * vec4<f32>(1.0, 1.0, 1.0, 1.0)));
}

fn rotor_rotor_geometric_product(self_648: Rotor, other_532: Rotor) -> Rotor {
    var self_649: Rotor;
    var other_533: Rotor;

    self_649 = self_648;
    other_533 = other_532;
    let _e4: Rotor = self_649;
    let _e8: Rotor = other_533;
    let _e11: Rotor = self_649;
    let _e15: Rotor = other_533;
    let _e28: Rotor = self_649;
    let _e32: Rotor = other_533;
    let _e45: Rotor = self_649;
    let _e49: Rotor = other_533;
    return Rotor(((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.y) * _e15.g0_.yxwz) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e28.g0_.z) * _e32.g0_.zwxy) * vec4<f32>(-(1.0), -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e45.g0_.w) * _e49.g0_.wzyx) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))));
}

fn rotor_rotor_outer_product(self_650: Rotor, other_534: Rotor) -> Rotor {
    var self_651: Rotor;
    var other_535: Rotor;

    self_651 = self_650;
    other_535 = other_534;
    let _e4: Rotor = self_651;
    let _e8: Rotor = other_535;
    let _e11: Rotor = self_651;
    let _e13: Rotor = other_535;
    return Rotor(((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((_e11.g0_ * vec4<f32>(_e13.g0_.x)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn rotor_rotor_inner_product(self_652: Rotor, other_536: Rotor) -> Rotor {
    var self_653: Rotor;
    var other_537: Rotor;

    self_653 = self_652;
    other_537 = other_536;
    let _e4: Rotor = self_653;
    let _e8: Rotor = other_537;
    let _e11: Rotor = self_653;
    let _e15: Rotor = other_537;
    let _e27: Rotor = self_653;
    let _e31: Rotor = other_537;
    let _e43: Rotor = self_653;
    let _e46: Rotor = other_537;
    return Rotor(((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.z) * _e15.g0_.zzxz) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0))) + ((vec4<f32>(_e27.g0_.w) * _e31.g0_.wwwx) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))) + ((_e43.g0_.yyxx * _e46.g0_.yxxx) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))));
}

fn rotor_rotor_left_contraction(self_654: Rotor, other_538: Rotor) -> Rotor {
    var self_655: Rotor;
    var other_539: Rotor;

    self_655 = self_654;
    other_539 = other_538;
    let _e4: Rotor = self_655;
    let _e8: Rotor = other_539;
    let _e11: Rotor = self_655;
    let _e15: Rotor = other_539;
    let _e28: Rotor = self_655;
    let _e32: Rotor = other_539;
    let _e45: Rotor = self_655;
    let _e48: Rotor = other_539;
    return Rotor(((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.z) * vec4<f32>(_e15.g0_.z)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((vec4<f32>(_e28.g0_.w) * vec4<f32>(_e32.g0_.w)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((_e45.g0_.yxxx * _e48.g0_.yxxx) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))));
}

fn rotor_rotor_right_contraction(self_656: Rotor, other_540: Rotor) -> Rotor {
    var self_657: Rotor;
    var other_541: Rotor;

    self_657 = self_656;
    other_541 = other_540;
    let _e4: Rotor = self_657;
    let _e8: Rotor = other_541;
    let _e19: Rotor = self_657;
    let _e23: Rotor = other_541;
    let _e35: Rotor = self_657;
    let _e39: Rotor = other_541;
    let _e51: Rotor = self_657;
    let _e55: Rotor = other_541;
    return Rotor((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0)) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.zzxz) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0))) + ((vec4<f32>(_e35.g0_.w) * _e39.g0_.wwwx) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))) + ((vec4<f32>(_e51.g0_.x) * vec4<f32>(_e55.g0_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))));
}

fn rotor_rotor_scalar_product(self_658: Rotor, other_542: Rotor) -> Scalar {
    var self_659: Rotor;
    var other_543: Rotor;

    self_659 = self_658;
    other_543 = other_542;
    let _e4: Rotor = self_659;
    let _e7: Rotor = other_543;
    let _e11: Rotor = self_659;
    let _e14: Rotor = other_543;
    let _e19: Rotor = self_659;
    let _e22: Rotor = other_543;
    let _e27: Rotor = self_659;
    let _e30: Rotor = other_543;
    return Scalar(((((_e4.g0_.x * _e7.g0_.x) - (_e11.g0_.y * _e14.g0_.y)) - (_e19.g0_.z * _e22.g0_.z)) - (_e27.g0_.w * _e30.g0_.w)));
}

fn rotor_point_geometric_product(self_660: Rotor, other_544: Point) -> PointAndPlane {
    var self_661: Rotor;
    var other_545: Point;

    self_661 = self_660;
    other_545 = other_544;
    let _e4: Rotor = self_661;
    let _e8: Point = other_545;
    let _e11: Rotor = self_661;
    let _e15: Point = other_545;
    let _e27: Rotor = self_661;
    let _e31: Point = other_545;
    let _e43: Rotor = self_661;
    let _e46: Point = other_545;
    let _e58: Rotor = self_661;
    let _e62: Point = other_545;
    let _e73: Rotor = self_661;
    let _e77: Point = other_545;
    let _e89: Rotor = self_661;
    let _e92: Point = other_545;
    return PointAndPlane(((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.z) * _e15.g0_.wwwy) * vec4<f32>(0.0, -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e27.g0_.w) * _e31.g0_.zzyz) * vec4<f32>(0.0, 1.0, -(1.0), 0.0))) + ((_e43.g0_.xxyy * _e46.g0_.xxwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))), ((((vec4<f32>(_e58.g0_.z) * _e62.g0_.zzxz) * vec4<f32>(1.0, 0.0, -(1.0), 0.0)) + ((vec4<f32>(_e73.g0_.w) * _e77.g0_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))) + ((_e89.g0_.yyxx * _e92.g0_.yxxx) * vec4<f32>(1.0, -(1.0), 0.0, 0.0))));
}

fn rotor_point_outer_product(self_662: Rotor, other_546: Point) -> Point {
    var self_663: Rotor;
    var other_547: Point;

    self_663 = self_662;
    other_547 = other_546;
    let _e4: Rotor = self_663;
    let _e8: Point = other_547;
    return Point((vec4<f32>(_e4.g0_.x) * _e8.g0_));
}

fn rotor_point_inner_product(self_664: Rotor, other_548: Point) -> PointAndPlane {
    var self_665: Rotor;
    var other_549: Point;

    self_665 = self_664;
    other_549 = other_548;
    let _e4: Rotor = self_665;
    let _e8: Point = other_549;
    let _e11: Rotor = self_665;
    let _e15: Point = other_549;
    let _e26: Rotor = self_665;
    let _e30: Point = other_549;
    let _e42: Rotor = self_665;
    let _e45: Point = other_549;
    return PointAndPlane((vec4<f32>(_e4.g0_.x) * _e8.g0_), ((((vec4<f32>(_e11.g0_.z) * _e15.g0_.zzxz) * vec4<f32>(1.0, 0.0, -(1.0), 0.0)) + ((vec4<f32>(_e26.g0_.w) * _e30.g0_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))) + ((_e42.g0_.yyxx * _e45.g0_.yxxx) * vec4<f32>(1.0, -(1.0), 0.0, 0.0))));
}

fn rotor_point_left_contraction(self_666: Rotor, other_550: Point) -> PointAndPlane {
    var self_667: Rotor;
    var other_551: Point;

    self_667 = self_666;
    other_551 = other_550;
    let _e4: Rotor = self_667;
    let _e8: Point = other_551;
    let _e11: Rotor = self_667;
    let _e15: Point = other_551;
    let _e26: Rotor = self_667;
    let _e30: Point = other_551;
    let _e42: Rotor = self_667;
    let _e45: Point = other_551;
    return PointAndPlane((vec4<f32>(_e4.g0_.x) * _e8.g0_), ((((vec4<f32>(_e11.g0_.z) * _e15.g0_.zzxz) * vec4<f32>(1.0, 0.0, -(1.0), 0.0)) + ((vec4<f32>(_e26.g0_.w) * _e30.g0_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))) + ((_e42.g0_.yyxx * _e45.g0_.yxxx) * vec4<f32>(1.0, -(1.0), 0.0, 0.0))));
}

fn rotor_ideal_point_regressive_product(self_668: Rotor, other_552: IdealPoint) -> Scalar {
    var self_669: Rotor;
    var other_553: IdealPoint;

    self_669 = self_668;
    other_553 = other_552;
    let _e4: Rotor = self_669;
    let _e7: IdealPoint = other_553;
    let _e11: Rotor = self_669;
    let _e14: IdealPoint = other_553;
    let _e19: Rotor = self_669;
    let _e22: IdealPoint = other_553;
    return Scalar((((_e4.g0_.y * _e7.g0_.x) + (_e11.g0_.z * _e14.g0_.y)) + (_e19.g0_.w * _e22.g0_.z)));
}

fn rotor_ideal_point_inner_product(self_670: Rotor, other_554: IdealPoint) -> IdealPoint {
    var self_671: Rotor;
    var other_555: IdealPoint;

    self_671 = self_670;
    other_555 = other_554;
    let _e4: Rotor = self_671;
    let _e8: IdealPoint = other_555;
    return IdealPoint((vec3<f32>(_e4.g0_.x) * _e8.g0_));
}

fn rotor_ideal_point_geometric_anti_product(self_672: Rotor, other_556: IdealPoint) -> Rotor {
    var self_673: Rotor;
    var other_557: IdealPoint;

    self_673 = self_672;
    other_557 = other_556;
    let _e4: Rotor = self_673;
    let _e8: IdealPoint = other_557;
    let _e11: IdealPoint = other_557;
    let _e14: IdealPoint = other_557;
    let _e17: IdealPoint = other_557;
    let _e29: Rotor = self_673;
    let _e33: IdealPoint = other_557;
    let _e36: IdealPoint = other_557;
    let _e39: IdealPoint = other_557;
    let _e42: IdealPoint = other_557;
    let _e55: Rotor = self_673;
    let _e59: IdealPoint = other_557;
    let _e62: IdealPoint = other_557;
    let _e65: IdealPoint = other_557;
    let _e68: IdealPoint = other_557;
    let _e81: Rotor = self_673;
    let _e85: IdealPoint = other_557;
    let _e88: IdealPoint = other_557;
    let _e91: IdealPoint = other_557;
    let _e94: IdealPoint = other_557;
    return Rotor((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.x, _e11.g0_.x, _e14.g0_.z, _e17.g0_.y)) * vec4<f32>(1.0, 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e29.g0_.z) * vec4<f32>(_e33.g0_.y, _e36.g0_.z, _e39.g0_.y, _e42.g0_.x)) * vec4<f32>(1.0, -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e55.g0_.w) * vec4<f32>(_e59.g0_.z, _e62.g0_.y, _e65.g0_.x, _e68.g0_.z)) * vec4<f32>(1.0, 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e81.g0_.x) * vec4<f32>(_e85.g0_.x, _e88.g0_.x, _e91.g0_.y, _e94.g0_.z)) * vec4<f32>(0.0, -(1.0), -(1.0), -(1.0)))));
}

fn rotor_ideal_point_left_contraction(self_674: Rotor, other_558: IdealPoint) -> IdealPoint {
    var self_675: Rotor;
    var other_559: IdealPoint;

    self_675 = self_674;
    other_559 = other_558;
    let _e4: Rotor = self_675;
    let _e8: IdealPoint = other_559;
    return IdealPoint((vec3<f32>(_e4.g0_.x) * _e8.g0_));
}

fn rotor_plane_geometric_product(self_676: Rotor, other_560: Plane) -> PointAndPlane {
    var self_677: Rotor;
    var other_561: Plane;

    self_677 = self_676;
    other_561 = other_560;
    let _e4: Rotor = self_677;
    let _e8: Plane = other_561;
    let _e19: Rotor = self_677;
    let _e23: Plane = other_561;
    let _e35: Rotor = self_677;
    let _e38: Plane = other_561;
    let _e50: Rotor = self_677;
    let _e54: Plane = other_561;
    let _e57: Rotor = self_677;
    let _e61: Plane = other_561;
    let _e73: Rotor = self_677;
    let _e77: Plane = other_561;
    let _e89: Rotor = self_677;
    let _e92: Plane = other_561;
    return PointAndPlane(((((vec4<f32>(_e4.g0_.z) * _e8.g0_.zzxz) * vec4<f32>(1.0, 0.0, -(1.0), 0.0)) + ((vec4<f32>(_e19.g0_.w) * _e23.g0_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))) + ((_e35.g0_.yyxx * _e38.g0_.yxxx) * vec4<f32>(1.0, -(1.0), 0.0, 0.0))), ((((vec4<f32>(_e50.g0_.x) * _e54.g0_) + ((vec4<f32>(_e57.g0_.z) * _e61.g0_.wwwy) * vec4<f32>(0.0, -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e73.g0_.w) * _e77.g0_.zzyz) * vec4<f32>(0.0, 1.0, -(1.0), 0.0))) + ((_e89.g0_.xxyy * _e92.g0_.xxwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))));
}

fn rotor_plane_outer_product(self_678: Rotor, other_562: Plane) -> PointAndPlane {
    var self_679: Rotor;
    var other_563: Plane;

    self_679 = self_678;
    other_563 = other_562;
    let _e4: Rotor = self_679;
    let _e8: Plane = other_563;
    let _e19: Rotor = self_679;
    let _e23: Plane = other_563;
    let _e35: Rotor = self_679;
    let _e38: Plane = other_563;
    let _e50: Rotor = self_679;
    let _e54: Plane = other_563;
    return PointAndPlane(((((vec4<f32>(_e4.g0_.z) * _e8.g0_.zzxz) * vec4<f32>(1.0, 0.0, -(1.0), 0.0)) + ((vec4<f32>(_e19.g0_.w) * _e23.g0_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))) + ((_e35.g0_.yyxx * _e38.g0_.yxxx) * vec4<f32>(1.0, -(1.0), 0.0, 0.0))), (vec4<f32>(_e50.g0_.x) * _e54.g0_));
}

fn rotor_plane_inner_product(self_680: Rotor, other_564: Plane) -> Plane {
    var self_681: Rotor;
    var other_565: Plane;

    self_681 = self_680;
    other_565 = other_564;
    let _e4: Rotor = self_681;
    let _e8: Plane = other_565;
    let _e11: Rotor = self_681;
    let _e15: Plane = other_565;
    let _e27: Rotor = self_681;
    let _e31: Plane = other_565;
    let _e43: Rotor = self_681;
    let _e46: Plane = other_565;
    return Plane(((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.z) * _e15.g0_.wwwy) * vec4<f32>(0.0, -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e27.g0_.w) * _e31.g0_.zzyz) * vec4<f32>(0.0, 1.0, -(1.0), 0.0))) + ((_e43.g0_.xxyy * _e46.g0_.xxwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))));
}

fn rotor_plane_left_contraction(self_682: Rotor, other_566: Plane) -> Plane {
    var self_683: Rotor;
    var other_567: Plane;

    self_683 = self_682;
    other_567 = other_566;
    let _e4: Rotor = self_683;
    let _e8: Plane = other_567;
    return Plane((vec4<f32>(_e4.g0_.x) * _e8.g0_));
}

fn rotor_line_geometric_product(self_684: Rotor, other_568: Line) -> Motor {
    var self_685: Rotor;
    var other_569: Line;

    self_685 = self_684;
    other_569 = other_568;
    let _e4: Rotor = self_685;
    let _e8: Line = other_569;
    let _e11: Line = other_569;
    let _e14: Line = other_569;
    let _e17: Line = other_569;
    let _e30: Rotor = self_685;
    let _e34: Line = other_569;
    let _e37: Line = other_569;
    let _e40: Line = other_569;
    let _e43: Line = other_569;
    let _e57: Rotor = self_685;
    let _e61: Line = other_569;
    let _e64: Line = other_569;
    let _e67: Line = other_569;
    let _e70: Line = other_569;
    let _e84: Rotor = self_685;
    let _e88: Line = other_569;
    let _e91: Line = other_569;
    let _e94: Line = other_569;
    let _e97: Line = other_569;
    let _e109: Rotor = self_685;
    let _e113: Line = other_569;
    let _e116: Line = other_569;
    let _e119: Line = other_569;
    let _e122: Line = other_569;
    let _e134: Rotor = self_685;
    let _e138: Line = other_569;
    let _e141: Line = other_569;
    let _e144: Line = other_569;
    let _e147: Line = other_569;
    let _e160: Rotor = self_685;
    let _e164: Line = other_569;
    let _e167: Line = other_569;
    let _e170: Line = other_569;
    let _e173: Line = other_569;
    let _e186: Rotor = self_685;
    let _e190: Line = other_569;
    let _e193: Line = other_569;
    let _e196: Line = other_569;
    let _e199: Line = other_569;
    return Motor((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g1_.x, _e11.g1_.x, _e14.g1_.z, _e17.g1_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e30.g0_.z) * vec4<f32>(_e34.g1_.y, _e37.g1_.z, _e40.g1_.y, _e43.g1_.x)) * vec4<f32>(-(1.0), -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e57.g0_.w) * vec4<f32>(_e61.g1_.z, _e64.g1_.y, _e67.g1_.x, _e70.g1_.z)) * vec4<f32>(-(1.0), 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e84.g0_.x) * vec4<f32>(_e88.g1_.x, _e91.g1_.x, _e94.g1_.y, _e97.g1_.z)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))), (((((vec4<f32>(_e109.g0_.y) * vec4<f32>(_e113.g0_.x, _e116.g0_.x, _e119.g0_.z, _e122.g0_.y)) * vec4<f32>(1.0, 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e134.g0_.z) * vec4<f32>(_e138.g0_.y, _e141.g0_.z, _e144.g0_.y, _e147.g0_.x)) * vec4<f32>(1.0, -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e160.g0_.w) * vec4<f32>(_e164.g0_.z, _e167.g0_.y, _e170.g0_.x, _e173.g0_.z)) * vec4<f32>(1.0, 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e186.g0_.x) * vec4<f32>(_e190.g0_.x, _e193.g0_.x, _e196.g0_.y, _e199.g0_.z)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn rotor_line_regressive_product(self_686: Rotor, other_570: Line) -> Scalar {
    var self_687: Rotor;
    var other_571: Line;

    self_687 = self_686;
    other_571 = other_570;
    let _e4: Rotor = self_687;
    let _e7: Line = other_571;
    let _e11: Rotor = self_687;
    let _e14: Line = other_571;
    let _e19: Rotor = self_687;
    let _e22: Line = other_571;
    return Scalar((((_e4.g0_.y * _e7.g0_.x) + (_e11.g0_.z * _e14.g0_.y)) + (_e19.g0_.w * _e22.g0_.z)));
}

fn rotor_line_geometric_anti_product(self_688: Rotor, other_572: Line) -> Rotor {
    var self_689: Rotor;
    var other_573: Line;

    self_689 = self_688;
    other_573 = other_572;
    let _e4: Rotor = self_689;
    let _e8: Line = other_573;
    let _e11: Line = other_573;
    let _e14: Line = other_573;
    let _e17: Line = other_573;
    let _e29: Rotor = self_689;
    let _e33: Line = other_573;
    let _e36: Line = other_573;
    let _e39: Line = other_573;
    let _e42: Line = other_573;
    let _e55: Rotor = self_689;
    let _e59: Line = other_573;
    let _e62: Line = other_573;
    let _e65: Line = other_573;
    let _e68: Line = other_573;
    let _e81: Rotor = self_689;
    let _e85: Line = other_573;
    let _e88: Line = other_573;
    let _e91: Line = other_573;
    let _e94: Line = other_573;
    return Rotor((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.x, _e11.g0_.x, _e14.g0_.z, _e17.g0_.y)) * vec4<f32>(1.0, 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e29.g0_.z) * vec4<f32>(_e33.g0_.y, _e36.g0_.z, _e39.g0_.y, _e42.g0_.x)) * vec4<f32>(1.0, -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e55.g0_.w) * vec4<f32>(_e59.g0_.z, _e62.g0_.y, _e65.g0_.x, _e68.g0_.z)) * vec4<f32>(1.0, 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e81.g0_.x) * vec4<f32>(_e85.g0_.x, _e88.g0_.x, _e91.g0_.y, _e94.g0_.z)) * vec4<f32>(0.0, -(1.0), -(1.0), -(1.0)))));
}

fn rotor_line_right_contraction(self_690: Rotor, other_574: Line) -> Scalar {
    var self_691: Rotor;
    var other_575: Line;

    self_691 = self_690;
    other_575 = other_574;
    let _e5: Rotor = self_691;
    let _e8: Line = other_575;
    let _e13: Rotor = self_691;
    let _e16: Line = other_575;
    let _e21: Rotor = self_691;
    let _e24: Line = other_575;
    return Scalar((((0.0 - (_e5.g0_.y * _e8.g1_.x)) - (_e13.g0_.z * _e16.g1_.y)) - (_e21.g0_.w * _e24.g1_.z)));
}

fn rotor_line_scalar_product(self_692: Rotor, other_576: Line) -> Scalar {
    var self_693: Rotor;
    var other_577: Line;

    self_693 = self_692;
    other_577 = other_576;
    let _e5: Rotor = self_693;
    let _e8: Line = other_577;
    let _e13: Rotor = self_693;
    let _e16: Line = other_577;
    let _e21: Rotor = self_693;
    let _e24: Line = other_577;
    return Scalar((((0.0 - (_e5.g0_.y * _e8.g1_.x)) - (_e13.g0_.z * _e16.g1_.y)) - (_e21.g0_.w * _e24.g1_.z)));
}

fn rotor_translator_geometric_product(self_694: Rotor, other_578: Translator) -> Motor {
    var self_695: Rotor;
    var other_579: Translator;

    self_695 = self_694;
    other_579 = other_578;
    let _e4: Rotor = self_695;
    let _e6: Translator = other_579;
    let _e11: Rotor = self_695;
    let _e15: Translator = other_579;
    let _e26: Rotor = self_695;
    let _e30: Translator = other_579;
    let _e42: Rotor = self_695;
    let _e46: Translator = other_579;
    let _e58: Rotor = self_695;
    let _e62: Translator = other_579;
    return Motor((_e4.g0_ * vec4<f32>(_e6.g0_.x)), (((((vec4<f32>(_e11.g0_.y) * _e15.g0_.yywz) * vec4<f32>(1.0, 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e26.g0_.z) * _e30.g0_.zwzy) * vec4<f32>(1.0, -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e42.g0_.w) * _e46.g0_.wzyw) * vec4<f32>(1.0, 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e58.g0_.x) * _e62.g0_) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn rotor_translator_regressive_product(self_696: Rotor, other_580: Translator) -> Scalar {
    var self_697: Rotor;
    var other_581: Translator;

    self_697 = self_696;
    other_581 = other_580;
    let _e4: Rotor = self_697;
    let _e7: Translator = other_581;
    let _e11: Rotor = self_697;
    let _e14: Translator = other_581;
    let _e19: Rotor = self_697;
    let _e22: Translator = other_581;
    return Scalar((((_e4.g0_.y * _e7.g0_.y) + (_e11.g0_.z * _e14.g0_.z)) + (_e19.g0_.w * _e22.g0_.w)));
}

fn rotor_translator_outer_product(self_698: Rotor, other_582: Translator) -> Motor {
    var self_699: Rotor;
    var other_583: Translator;

    self_699 = self_698;
    other_583 = other_582;
    let _e4: Rotor = self_699;
    let _e6: Translator = other_583;
    let _e11: Rotor = self_699;
    let _e15: Translator = other_583;
    let _e26: Rotor = self_699;
    let _e30: Translator = other_583;
    let _e42: Rotor = self_699;
    let _e45: Translator = other_583;
    return Motor((_e4.g0_ * vec4<f32>(_e6.g0_.x)), ((((vec4<f32>(_e11.g0_.z) * vec4<f32>(_e15.g0_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) + ((vec4<f32>(_e26.g0_.w) * vec4<f32>(_e30.g0_.w)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + (_e42.g0_.yxxx * _e45.g0_.yyzw)));
}

fn rotor_translator_geometric_anti_product(self_700: Rotor, other_584: Translator) -> Rotor {
    var self_701: Rotor;
    var other_585: Translator;

    self_701 = self_700;
    other_585 = other_584;
    let _e4: Rotor = self_701;
    let _e8: Translator = other_585;
    let _e19: Rotor = self_701;
    let _e23: Translator = other_585;
    let _e35: Rotor = self_701;
    let _e39: Translator = other_585;
    let _e51: Rotor = self_701;
    let _e55: Translator = other_585;
    return Rotor((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.yywz) * vec4<f32>(1.0, 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.zwzy) * vec4<f32>(1.0, -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e35.g0_.w) * _e39.g0_.wzyw) * vec4<f32>(1.0, 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e51.g0_.x) * _e55.g0_) * vec4<f32>(0.0, -(1.0), -(1.0), -(1.0)))));
}

fn rotor_translator_left_contraction(self_702: Rotor, other_586: Translator) -> Translator {
    var self_703: Rotor;
    var other_587: Translator;

    self_703 = self_702;
    other_587 = other_586;
    let _e4: Rotor = self_703;
    let _e8: Translator = other_587;
    return Translator((vec4<f32>(_e4.g0_.x) * _e8.g0_));
}

fn rotor_translator_right_contraction(self_704: Rotor, other_588: Translator) -> Rotor {
    var self_705: Rotor;
    var other_589: Translator;

    self_705 = self_704;
    other_589 = other_588;
    let _e4: Rotor = self_705;
    let _e6: Translator = other_589;
    return Rotor((_e4.g0_ * vec4<f32>(_e6.g0_.x)));
}

fn rotor_translator_scalar_product(self_706: Rotor, other_590: Translator) -> Scalar {
    var self_707: Rotor;
    var other_591: Translator;

    self_707 = self_706;
    other_591 = other_590;
    let _e4: Rotor = self_707;
    let _e7: Translator = other_591;
    return Scalar((_e4.g0_.x * _e7.g0_.x));
}

fn rotor_motor_add(self_708: Rotor, other_592: Motor) -> Motor {
    var self_709: Rotor;
    var other_593: Motor;

    self_709 = self_708;
    other_593 = other_592;
    let _e4: Rotor = self_709;
    let _e6: Motor = other_593;
    let _e9: Motor = other_593;
    return Motor((_e4.g0_ + _e6.g0_), _e9.g1_);
}

fn rotor_motor_sub(self_710: Rotor, other_594: Motor) -> Motor {
    var self_711: Rotor;
    var other_595: Motor;

    self_711 = self_710;
    other_595 = other_594;
    let _e4: Rotor = self_711;
    let _e6: Motor = other_595;
    let _e11: Motor = other_595;
    return Motor((_e4.g0_ - _e6.g0_), (vec4<f32>(0.0) - _e11.g1_));
}

fn rotor_motor_geometric_product(self_712: Rotor, other_596: Motor) -> Motor {
    var self_713: Rotor;
    var other_597: Motor;

    self_713 = self_712;
    other_597 = other_596;
    let _e4: Rotor = self_713;
    let _e8: Motor = other_597;
    let _e11: Rotor = self_713;
    let _e15: Motor = other_597;
    let _e28: Rotor = self_713;
    let _e32: Motor = other_597;
    let _e45: Rotor = self_713;
    let _e49: Motor = other_597;
    let _e62: Rotor = self_713;
    let _e66: Motor = other_597;
    let _e69: Rotor = self_713;
    let _e73: Motor = other_597;
    let _e86: Rotor = self_713;
    let _e90: Motor = other_597;
    let _e103: Rotor = self_713;
    let _e107: Motor = other_597;
    return Motor(((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.y) * _e15.g0_.yxwz) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e28.g0_.z) * _e32.g0_.zwxy) * vec4<f32>(-(1.0), -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e45.g0_.w) * _e49.g0_.wzyx) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))), ((((vec4<f32>(_e62.g0_.x) * _e66.g1_) + ((vec4<f32>(_e69.g0_.y) * _e73.g1_.yxwz) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e86.g0_.z) * _e90.g1_.zwxy) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e103.g0_.w) * _e107.g1_.wzyx) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))));
}

fn rotor_motor_regressive_product(self_714: Rotor, other_598: Motor) -> Rotor {
    var self_715: Rotor;
    var other_599: Motor;

    self_715 = self_714;
    other_599 = other_598;
    let _e4: Rotor = self_715;
    let _e8: Motor = other_599;
    let _e18: Rotor = self_715;
    let _e22: Motor = other_599;
    let _e33: Rotor = self_715;
    let _e37: Motor = other_599;
    let _e48: Rotor = self_715;
    let _e52: Motor = other_599;
    return Rotor((((((vec4<f32>(_e4.g0_.y) * _e8.g1_.yxyy) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) + ((vec4<f32>(_e18.g0_.z) * _e22.g1_.zzxz) * vec4<f32>(1.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e33.g0_.w) * _e37.g1_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e48.g0_.x) * vec4<f32>(_e52.g1_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))));
}

fn rotor_motor_outer_product(self_716: Rotor, other_600: Motor) -> Motor {
    var self_717: Rotor;
    var other_601: Motor;

    self_717 = self_716;
    other_601 = other_600;
    let _e4: Rotor = self_717;
    let _e8: Motor = other_601;
    let _e11: Rotor = self_717;
    let _e13: Motor = other_601;
    let _e25: Rotor = self_717;
    let _e29: Motor = other_601;
    let _e32: Rotor = self_717;
    let _e36: Motor = other_601;
    let _e48: Rotor = self_717;
    let _e52: Motor = other_601;
    let _e64: Rotor = self_717;
    let _e67: Motor = other_601;
    return Motor(((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((_e11.g0_ * vec4<f32>(_e13.g0_.x)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))), ((((vec4<f32>(_e25.g0_.x) * _e29.g1_) + ((vec4<f32>(_e32.g0_.z) * vec4<f32>(_e36.g1_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e48.g0_.w) * vec4<f32>(_e52.g1_.w)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((_e64.g0_.yxxx * _e67.g1_.yxxx) * vec4<f32>(1.0, 0.0, 0.0, 0.0))));
}

fn rotor_motor_inner_product(self_718: Rotor, other_602: Motor) -> Motor {
    var self_719: Rotor;
    var other_603: Motor;

    self_719 = self_718;
    other_603 = other_602;
    let _e4: Rotor = self_719;
    let _e8: Motor = other_603;
    let _e11: Rotor = self_719;
    let _e15: Motor = other_603;
    let _e27: Rotor = self_719;
    let _e31: Motor = other_603;
    let _e43: Rotor = self_719;
    let _e46: Motor = other_603;
    let _e58: Rotor = self_719;
    let _e62: Motor = other_603;
    let _e65: Rotor = self_719;
    let _e67: Motor = other_603;
    return Motor(((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.z) * _e15.g0_.zzxz) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0))) + ((vec4<f32>(_e27.g0_.w) * _e31.g0_.wwwx) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))) + ((_e43.g0_.yyxx * _e46.g0_.yxxx) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))), ((vec4<f32>(_e58.g0_.x) * _e62.g1_) + ((_e65.g0_ * vec4<f32>(_e67.g1_.x)) * vec4<f32>(0.0, -(1.0), -(1.0), -(1.0)))));
}

fn rotor_motor_geometric_anti_product(self_720: Rotor, other_604: Motor) -> Rotor {
    var self_721: Rotor;
    var other_605: Motor;

    self_721 = self_720;
    other_605 = other_604;
    let _e4: Rotor = self_721;
    let _e8: Motor = other_605;
    let _e20: Rotor = self_721;
    let _e24: Motor = other_605;
    let _e36: Rotor = self_721;
    let _e40: Motor = other_605;
    let _e52: Rotor = self_721;
    let _e56: Motor = other_605;
    return Rotor((((((vec4<f32>(_e4.g0_.x) * _e8.g1_) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))) + ((vec4<f32>(_e20.g0_.y) * _e24.g1_.yxwz) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e36.g0_.z) * _e40.g1_.zwxy) * vec4<f32>(1.0, -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e52.g0_.w) * _e56.g1_.wzyx) * vec4<f32>(1.0, 1.0, -(1.0), 1.0))));
}

fn rotor_motor_inner_anti_product(self_722: Rotor, other_606: Motor) -> Rotor {
    var self_723: Rotor;
    var other_607: Motor;

    self_723 = self_722;
    other_607 = other_606;
    let _e4: Rotor = self_723;
    let _e8: Motor = other_607;
    let _e20: Rotor = self_723;
    let _e22: Motor = other_607;
    return Rotor((((vec4<f32>(_e4.g0_.x) * _e8.g1_) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))) + ((_e20.g0_ * vec4<f32>(_e22.g1_.x)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn rotor_motor_left_contraction(self_724: Rotor, other_608: Motor) -> Motor {
    var self_725: Rotor;
    var other_609: Motor;

    self_725 = self_724;
    other_609 = other_608;
    let _e4: Rotor = self_725;
    let _e8: Motor = other_609;
    let _e11: Rotor = self_725;
    let _e15: Motor = other_609;
    let _e28: Rotor = self_725;
    let _e32: Motor = other_609;
    let _e45: Rotor = self_725;
    let _e48: Motor = other_609;
    let _e60: Rotor = self_725;
    let _e64: Motor = other_609;
    let _e67: Rotor = self_725;
    let _e69: Motor = other_609;
    return Motor(((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.z) * vec4<f32>(_e15.g0_.z)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((vec4<f32>(_e28.g0_.w) * vec4<f32>(_e32.g0_.w)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((_e45.g0_.yxxx * _e48.g0_.yxxx) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))), ((vec4<f32>(_e60.g0_.x) * _e64.g1_) + ((_e67.g0_ * vec4<f32>(_e69.g1_.x)) * vec4<f32>(0.0, -(1.0), -(1.0), -(1.0)))));
}

fn rotor_motor_right_contraction(self_726: Rotor, other_610: Motor) -> Rotor {
    var self_727: Rotor;
    var other_611: Motor;

    self_727 = self_726;
    other_611 = other_610;
    let _e4: Rotor = self_727;
    let _e8: Motor = other_611;
    let _e19: Rotor = self_727;
    let _e23: Motor = other_611;
    let _e35: Rotor = self_727;
    let _e39: Motor = other_611;
    let _e51: Rotor = self_727;
    let _e55: Motor = other_611;
    return Rotor((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0)) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.zzxz) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0))) + ((vec4<f32>(_e35.g0_.w) * _e39.g0_.wwwx) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))) + ((vec4<f32>(_e51.g0_.x) * vec4<f32>(_e55.g0_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))));
}

fn rotor_motor_right_anti_contraction(self_728: Rotor, other_612: Motor) -> Rotor {
    var self_729: Rotor;
    var other_613: Motor;

    self_729 = self_728;
    other_613 = other_612;
    let _e4: Rotor = self_729;
    let _e8: Motor = other_613;
    let _e20: Rotor = self_729;
    let _e22: Motor = other_613;
    return Rotor((((vec4<f32>(_e4.g0_.x) * _e8.g1_) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))) + ((_e20.g0_ * vec4<f32>(_e22.g1_.x)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn rotor_motor_scalar_product(self_730: Rotor, other_614: Motor) -> Scalar {
    var self_731: Rotor;
    var other_615: Motor;

    self_731 = self_730;
    other_615 = other_614;
    let _e4: Rotor = self_731;
    let _e7: Motor = other_615;
    let _e11: Rotor = self_731;
    let _e14: Motor = other_615;
    let _e19: Rotor = self_731;
    let _e22: Motor = other_615;
    let _e27: Rotor = self_731;
    let _e30: Motor = other_615;
    return Scalar(((((_e4.g0_.x * _e7.g0_.x) - (_e11.g0_.y * _e14.g0_.y)) - (_e19.g0_.z * _e22.g0_.z)) - (_e27.g0_.w * _e30.g0_.w)));
}

fn rotor_point_and_plane_geometric_product(self_732: Rotor, other_616: PointAndPlane) -> PointAndPlane {
    var self_733: Rotor;
    var other_617: PointAndPlane;

    self_733 = self_732;
    other_617 = other_616;
    let _e4: Rotor = self_733;
    let _e8: PointAndPlane = other_617;
    let _e11: Rotor = self_733;
    let _e15: PointAndPlane = other_617;
    let _e18: PointAndPlane = other_617;
    let _e21: PointAndPlane = other_617;
    let _e24: PointAndPlane = other_617;
    let _e38: Rotor = self_733;
    let _e42: PointAndPlane = other_617;
    let _e45: PointAndPlane = other_617;
    let _e48: PointAndPlane = other_617;
    let _e51: PointAndPlane = other_617;
    let _e65: Rotor = self_733;
    let _e69: PointAndPlane = other_617;
    let _e72: PointAndPlane = other_617;
    let _e75: PointAndPlane = other_617;
    let _e78: PointAndPlane = other_617;
    let _e92: Rotor = self_733;
    let _e96: PointAndPlane = other_617;
    let _e99: Rotor = self_733;
    let _e103: PointAndPlane = other_617;
    let _e106: PointAndPlane = other_617;
    let _e109: PointAndPlane = other_617;
    let _e112: PointAndPlane = other_617;
    let _e126: Rotor = self_733;
    let _e130: PointAndPlane = other_617;
    let _e133: PointAndPlane = other_617;
    let _e136: PointAndPlane = other_617;
    let _e139: PointAndPlane = other_617;
    let _e153: Rotor = self_733;
    let _e157: PointAndPlane = other_617;
    let _e160: PointAndPlane = other_617;
    let _e163: PointAndPlane = other_617;
    let _e166: PointAndPlane = other_617;
    return PointAndPlane(((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.y) * vec4<f32>(_e15.g1_.y, _e18.g1_.x, _e21.g0_.w, _e24.g0_.z)) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e38.g0_.z) * vec4<f32>(_e42.g1_.z, _e45.g0_.w, _e48.g1_.x, _e51.g0_.y)) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e65.g0_.w) * vec4<f32>(_e69.g1_.w, _e72.g0_.z, _e75.g0_.y, _e78.g1_.x)) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))), ((((vec4<f32>(_e92.g0_.x) * _e96.g1_) + ((vec4<f32>(_e99.g0_.y) * vec4<f32>(_e103.g0_.y, _e106.g0_.x, _e109.g1_.w, _e112.g1_.z)) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e126.g0_.z) * vec4<f32>(_e130.g0_.z, _e133.g1_.w, _e136.g0_.x, _e139.g1_.y)) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e153.g0_.w) * vec4<f32>(_e157.g0_.w, _e160.g1_.z, _e163.g1_.y, _e166.g0_.x)) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))));
}

fn rotor_point_and_plane_outer_product(self_734: Rotor, other_618: PointAndPlane) -> PointAndPlane {
    var self_735: Rotor;
    var other_619: PointAndPlane;

    self_735 = self_734;
    other_619 = other_618;
    let _e4: Rotor = self_735;
    let _e8: PointAndPlane = other_619;
    let _e11: Rotor = self_735;
    let _e15: PointAndPlane = other_619;
    let _e27: Rotor = self_735;
    let _e31: PointAndPlane = other_619;
    let _e43: Rotor = self_735;
    let _e46: PointAndPlane = other_619;
    let _e58: Rotor = self_735;
    let _e62: PointAndPlane = other_619;
    return PointAndPlane(((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.z) * _e15.g1_.zzxz) * vec4<f32>(1.0, 0.0, -(1.0), 0.0))) + ((vec4<f32>(_e27.g0_.w) * _e31.g1_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))) + ((_e43.g0_.yyxx * _e46.g1_.yxxx) * vec4<f32>(1.0, -(1.0), 0.0, 0.0))), (vec4<f32>(_e58.g0_.x) * _e62.g1_));
}

fn rotor_point_and_plane_inner_product(self_736: Rotor, other_620: PointAndPlane) -> PointAndPlane {
    var self_737: Rotor;
    var other_621: PointAndPlane;

    self_737 = self_736;
    other_621 = other_620;
    let _e4: Rotor = self_737;
    let _e8: PointAndPlane = other_621;
    let _e11: Rotor = self_737;
    let _e15: PointAndPlane = other_621;
    let _e18: Rotor = self_737;
    let _e22: PointAndPlane = other_621;
    let _e25: PointAndPlane = other_621;
    let _e28: PointAndPlane = other_621;
    let _e31: PointAndPlane = other_621;
    let _e45: Rotor = self_737;
    let _e49: PointAndPlane = other_621;
    let _e52: PointAndPlane = other_621;
    let _e55: PointAndPlane = other_621;
    let _e58: PointAndPlane = other_621;
    let _e72: Rotor = self_737;
    let _e76: PointAndPlane = other_621;
    let _e79: PointAndPlane = other_621;
    let _e82: PointAndPlane = other_621;
    let _e85: PointAndPlane = other_621;
    return PointAndPlane((vec4<f32>(_e4.g0_.x) * _e8.g0_), ((((vec4<f32>(_e11.g0_.x) * _e15.g1_) + ((vec4<f32>(_e18.g0_.y) * vec4<f32>(_e22.g0_.y, _e25.g0_.x, _e28.g1_.w, _e31.g1_.z)) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e45.g0_.z) * vec4<f32>(_e49.g0_.z, _e52.g1_.w, _e55.g0_.x, _e58.g1_.y)) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e72.g0_.w) * vec4<f32>(_e76.g0_.w, _e79.g1_.z, _e82.g1_.y, _e85.g0_.x)) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))));
}

fn rotor_point_and_plane_left_contraction(self_738: Rotor, other_622: PointAndPlane) -> PointAndPlane {
    var self_739: Rotor;
    var other_623: PointAndPlane;

    self_739 = self_738;
    other_623 = other_622;
    let _e4: Rotor = self_739;
    let _e8: PointAndPlane = other_623;
    let _e11: Rotor = self_739;
    let _e15: PointAndPlane = other_623;
    let _e18: Rotor = self_739;
    let _e22: PointAndPlane = other_623;
    let _e34: Rotor = self_739;
    let _e38: PointAndPlane = other_623;
    let _e50: Rotor = self_739;
    let _e53: PointAndPlane = other_623;
    return PointAndPlane((vec4<f32>(_e4.g0_.x) * _e8.g0_), ((((vec4<f32>(_e11.g0_.x) * _e15.g1_) + ((vec4<f32>(_e18.g0_.z) * _e22.g0_.zzxz) * vec4<f32>(1.0, 0.0, -(1.0), 0.0))) + ((vec4<f32>(_e34.g0_.w) * _e38.g0_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))) + ((_e50.g0_.yyxx * _e53.g0_.yxxx) * vec4<f32>(1.0, -(1.0), 0.0, 0.0))));
}

fn rotor_squared_magnitude(self_740: Rotor) -> Scalar {
    var self_741: Rotor;

    self_741 = self_740;
    let _e2: Rotor = self_741;
    let _e3: Rotor = self_741;
    let _e4: Rotor = rotor_reversal(_e3);
    let _e5: Scalar = rotor_rotor_scalar_product(_e2, _e4);
    return _e5;
}

fn rotor_magnitude(self_742: Rotor) -> Scalar {
    var self_743: Rotor;

    self_743 = self_742;
    let _e2: Rotor = self_743;
    let _e3: Scalar = rotor_squared_magnitude(_e2);
    return Scalar(sqrt(_e3.g0_));
}

fn rotor_scale(self_744: Rotor, other_624: f32) -> Rotor {
    var self_745: Rotor;
    var other_625: f32;

    self_745 = self_744;
    other_625 = other_624;
    let _e4: Rotor = self_745;
    let _e5: f32 = other_625;
    let _e7: Rotor = rotor_scalar_geometric_product(_e4, Scalar(_e5));
    return _e7;
}

fn rotor_signum(self_746: Rotor) -> Rotor {
    var self_747: Rotor;

    self_747 = self_746;
    let _e2: Rotor = self_747;
    let _e3: Rotor = self_747;
    let _e4: Scalar = rotor_magnitude(_e3);
    let _e9: Rotor = rotor_scalar_geometric_product(_e2, Scalar((1.0 / _e4.g0_)));
    return _e9;
}

fn rotor_inverse(self_748: Rotor) -> Rotor {
    var self_749: Rotor;

    self_749 = self_748;
    let _e2: Rotor = self_749;
    let _e3: Rotor = rotor_reversal(_e2);
    let _e4: Rotor = self_749;
    let _e5: Scalar = rotor_squared_magnitude(_e4);
    let _e10: Rotor = rotor_scalar_geometric_product(_e3, Scalar((1.0 / _e5.g0_)));
    return _e10;
}

fn point_zero() -> Point {
    return Point(vec4<f32>(0.0));
}

fn point_one() -> Point {
    return Point(vec4<f32>(0.0));
}

fn point_grade(self_750: Point) -> i32 {
    return 3;
}

fn point_anti_grade(self_751: Point) -> i32 {
    return 1;
}

fn point_neg(self_752: Point) -> Point {
    var self_753: Point;

    self_753 = self_752;
    let _e2: Point = self_753;
    return Point((_e2.g0_ * vec4<f32>(-(1.0))));
}

fn point_automorphism(self_754: Point) -> Point {
    var self_755: Point;

    self_755 = self_754;
    let _e2: Point = self_755;
    return Point((_e2.g0_ * vec4<f32>(-(1.0))));
}

fn point_reversal(self_756: Point) -> Point {
    var self_757: Point;

    self_757 = self_756;
    let _e2: Point = self_757;
    return Point((_e2.g0_ * vec4<f32>(-(1.0))));
}

fn point_conjugation(self_758: Point) -> Point {
    var self_759: Point;

    self_759 = self_758;
    let _e2: Point = self_759;
    return Point(_e2.g0_);
}

fn point_dual(self_760: Point) -> Plane {
    var self_761: Point;

    self_761 = self_760;
    let _e2: Point = self_761;
    return Plane((_e2.g0_ * vec4<f32>(-(1.0))));
}

fn point_anti_reversal(self_762: Point) -> Point {
    var self_763: Point;

    self_763 = self_762;
    let _e2: Point = self_763;
    return Point(_e2.g0_);
}

fn point_right_complement(self_764: Point) -> Plane {
    var self_765: Point;

    self_765 = self_764;
    let _e2: Point = self_765;
    return Plane((_e2.g0_ * vec4<f32>(-(1.0))));
}

fn point_left_complement(self_766: Point) -> Plane {
    var self_767: Point;

    self_767 = self_766;
    let _e2: Point = self_767;
    return Plane(_e2.g0_);
}

fn point_double_complement(self_768: Point) -> Point {
    var self_769: Point;

    self_769 = self_768;
    let _e2: Point = self_769;
    return Point((_e2.g0_ * vec4<f32>(-(1.0))));
}

fn point_scalar_geometric_product(self_770: Point, other_626: Scalar) -> Point {
    var self_771: Point;
    var other_627: Scalar;

    self_771 = self_770;
    other_627 = other_626;
    let _e4: Point = self_771;
    let _e6: Scalar = other_627;
    return Point((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn point_scalar_outer_product(self_772: Point, other_628: Scalar) -> Point {
    var self_773: Point;
    var other_629: Scalar;

    self_773 = self_772;
    other_629 = other_628;
    let _e4: Point = self_773;
    let _e6: Scalar = other_629;
    return Point((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn point_scalar_inner_product(self_774: Point, other_630: Scalar) -> Point {
    var self_775: Point;
    var other_631: Scalar;

    self_775 = self_774;
    other_631 = other_630;
    let _e4: Point = self_775;
    let _e6: Scalar = other_631;
    return Point((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn point_scalar_right_contraction(self_776: Point, other_632: Scalar) -> Point {
    var self_777: Point;
    var other_633: Scalar;

    self_777 = self_776;
    other_633 = other_632;
    let _e4: Point = self_777;
    let _e6: Scalar = other_633;
    return Point((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn point_anti_scalar_regressive_product(self_778: Point, other_634: AntiScalar) -> Point {
    var self_779: Point;
    var other_635: AntiScalar;

    self_779 = self_778;
    other_635 = other_634;
    let _e4: Point = self_779;
    let _e6: AntiScalar = other_635;
    return Point((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn point_anti_scalar_geometric_anti_product(self_780: Point, other_636: AntiScalar) -> Point {
    var self_781: Point;
    var other_637: AntiScalar;

    self_781 = self_780;
    other_637 = other_636;
    let _e4: Point = self_781;
    let _e6: AntiScalar = other_637;
    return Point((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn point_anti_scalar_inner_anti_product(self_782: Point, other_638: AntiScalar) -> Point {
    var self_783: Point;
    var other_639: AntiScalar;

    self_783 = self_782;
    other_639 = other_638;
    let _e4: Point = self_783;
    let _e6: AntiScalar = other_639;
    return Point((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn point_anti_scalar_right_anti_contraction(self_784: Point, other_640: AntiScalar) -> Point {
    var self_785: Point;
    var other_641: AntiScalar;

    self_785 = self_784;
    other_641 = other_640;
    let _e4: Point = self_785;
    let _e6: AntiScalar = other_641;
    return Point((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn point_multi_vector_add(self_786: Point, other_642: MultiVector) -> MultiVector {
    var self_787: Point;
    var other_643: MultiVector;

    self_787 = self_786;
    other_643 = other_642;
    let _e4: MultiVector = other_643;
    let _e6: Point = self_787;
    let _e14: MultiVector = other_643;
    let _e17: Point = self_787;
    let _e27: MultiVector = other_643;
    let _e30: MultiVector = other_643;
    return MultiVector(_e4.g0_, ((_e6.g0_ * vec4<f32>(0.0, 1.0, 1.0, 1.0)) + _e14.g1_), ((vec4<f32>(_e17.g0_.x) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) + _e27.g2_), _e30.g3_);
}

fn point_multi_vector_sub(self_788: Point, other_644: MultiVector) -> MultiVector {
    var self_789: Point;
    var other_645: MultiVector;

    self_789 = self_788;
    other_645 = other_644;
    let _e6: MultiVector = other_645;
    let _e9: Point = self_789;
    let _e17: MultiVector = other_645;
    let _e20: Point = self_789;
    let _e30: MultiVector = other_645;
    let _e35: MultiVector = other_645;
    return MultiVector((vec4<f32>(0.0) - _e6.g0_), ((_e9.g0_ * vec4<f32>(0.0, 1.0, 1.0, 1.0)) - _e17.g1_), ((vec4<f32>(_e20.g0_.x) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) - _e30.g2_), (vec4<f32>(0.0) - _e35.g3_));
}

fn point_multi_vector_geometric_product(self_790: Point, other_646: MultiVector) -> MultiVector {
    var self_791: Point;
    var other_647: MultiVector;

    self_791 = self_790;
    other_647 = other_646;
    let _e4: Point = self_791;
    let _e8: MultiVector = other_647;
    let _e18: Point = self_791;
    let _e22: MultiVector = other_647;
    let _e25: Point = self_791;
    let _e29: MultiVector = other_647;
    let _e41: Point = self_791;
    let _e45: MultiVector = other_647;
    let _e57: Point = self_791;
    let _e61: MultiVector = other_647;
    let _e73: Point = self_791;
    let _e77: MultiVector = other_647;
    let _e91: Point = self_791;
    let _e95: MultiVector = other_647;
    let _e99: Point = self_791;
    let _e103: MultiVector = other_647;
    let _e116: Point = self_791;
    let _e120: MultiVector = other_647;
    let _e133: Point = self_791;
    let _e137: MultiVector = other_647;
    return MultiVector(((vec4<f32>(_e4.g0_.x) * _e8.g2_) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0)), ((((vec4<f32>(_e18.g0_.x) * _e22.g3_) + ((vec4<f32>(_e25.g0_.y) * _e29.g0_.yxwz) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e41.g0_.z) * _e45.g0_.zwxy) * vec4<f32>(1.0, -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e57.g0_.w) * _e61.g0_.wzyx) * vec4<f32>(1.0, 1.0, -(1.0), 1.0))), ((vec4<f32>(_e73.g0_.x) * _e77.g0_) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))), ((((vec4<f32>(0.0) - (vec4<f32>(_e91.g0_.x) * _e95.g1_)) + ((vec4<f32>(_e99.g0_.y) * _e103.g2_.yxwz) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))) + ((vec4<f32>(_e116.g0_.z) * _e120.g2_.zwxy) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e133.g0_.w) * _e137.g2_.wzyx) * vec4<f32>(-(1.0), -(1.0), 1.0, 1.0))));
}

fn point_multi_vector_geometric_anti_product(self_792: Point, other_648: MultiVector) -> MultiVector {
    var self_793: Point;
    var other_649: MultiVector;

    self_793 = self_792;
    other_649 = other_648;
    let _e4: Point = self_793;
    let _e8: MultiVector = other_649;
    let _e18: Point = self_793;
    let _e22: MultiVector = other_649;
    let _e36: Point = self_793;
    let _e40: MultiVector = other_649;
    let _e54: Point = self_793;
    let _e58: MultiVector = other_649;
    let _e72: Point = self_793;
    let _e76: MultiVector = other_649;
    let _e88: Point = self_793;
    let _e92: MultiVector = other_649;
    let _e105: Point = self_793;
    let _e109: MultiVector = other_649;
    let _e122: Point = self_793;
    let _e126: MultiVector = other_649;
    let _e129: Point = self_793;
    let _e133: MultiVector = other_649;
    let _e147: Point = self_793;
    let _e151: MultiVector = other_649;
    let _e165: Point = self_793;
    let _e169: MultiVector = other_649;
    let _e183: Point = self_793;
    let _e187: MultiVector = other_649;
    let _e199: Point = self_793;
    let _e203: MultiVector = other_649;
    let _e216: Point = self_793;
    let _e220: MultiVector = other_649;
    return MultiVector((((((vec4<f32>(_e4.g0_.x) * _e8.g1_) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0)) + ((vec4<f32>(_e18.g0_.y) * _e22.g2_.yxwz) * vec4<f32>(-(1.0), -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e36.g0_.z) * _e40.g2_.zwxy) * vec4<f32>(-(1.0), 1.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e54.g0_.w) * _e58.g2_.wzyx) * vec4<f32>(-(1.0), -(1.0), 1.0, -(1.0)))), ((((vec4<f32>(_e72.g0_.y) * _e76.g3_.yxwz) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0))) + ((vec4<f32>(_e88.g0_.z) * _e92.g3_.zwxy) * vec4<f32>(-(1.0), -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e105.g0_.w) * _e109.g3_.wzyx) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))), ((((vec4<f32>(_e122.g0_.x) * _e126.g3_) + ((vec4<f32>(_e129.g0_.y) * _e133.g0_.yxwz) * vec4<f32>(-(1.0), -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e147.g0_.z) * _e151.g0_.zwxy) * vec4<f32>(-(1.0), -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e165.g0_.w) * _e169.g0_.wzyx) * vec4<f32>(-(1.0), 1.0, -(1.0), -(1.0)))), ((((vec4<f32>(_e183.g0_.y) * _e187.g1_.yxwz) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0)) + ((vec4<f32>(_e199.g0_.z) * _e203.g1_.zwxy) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e216.g0_.w) * _e220.g1_.wzyx) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))));
}

fn point_multi_vector_scalar_product(self_794: Point, other_650: MultiVector) -> Scalar {
    var self_795: Point;
    var other_651: MultiVector;

    self_795 = self_794;
    other_651 = other_650;
    let _e5: Point = self_795;
    let _e8: MultiVector = other_651;
    return Scalar((0.0 - (_e5.g0_.x * _e8.g2_.x)));
}

fn point_multi_vector_anti_scalar_product(self_796: Point, other_652: MultiVector) -> AntiScalar {
    var self_797: Point;
    var other_653: MultiVector;

    self_797 = self_796;
    other_653 = other_652;
    let _e4: Point = self_797;
    let _e7: MultiVector = other_653;
    let _e11: Point = self_797;
    let _e14: MultiVector = other_653;
    let _e19: Point = self_797;
    let _e22: MultiVector = other_653;
    return AntiScalar((((_e4.g0_.y * _e7.g1_.y) + (_e11.g0_.z * _e14.g1_.z)) + (_e19.g0_.w * _e22.g1_.w)));
}

fn point_rotor_geometric_product(self_798: Point, other_654: Rotor) -> PointAndPlane {
    var self_799: Point;
    var other_655: Rotor;

    self_799 = self_798;
    other_655 = other_654;
    let _e4: Point = self_799;
    let _e8: Rotor = other_655;
    let _e19: Point = self_799;
    let _e23: Rotor = other_655;
    let _e35: Point = self_799;
    let _e38: Rotor = other_655;
    let _e50: Point = self_799;
    let _e54: Rotor = other_655;
    let _e65: Point = self_799;
    let _e69: Rotor = other_655;
    let _e81: Point = self_799;
    let _e84: Rotor = other_655;
    return PointAndPlane(((((vec4<f32>(_e4.g0_.z) * _e8.g0_.wwxy) * vec4<f32>(0.0, -(1.0), 1.0, 1.0)) + ((vec4<f32>(_e19.g0_.w) * _e23.g0_.zzyx) * vec4<f32>(0.0, 1.0, -(1.0), 1.0))) + ((_e35.g0_.xyyy * _e38.g0_.xxwz) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))), ((((vec4<f32>(_e50.g0_.z) * vec4<f32>(_e54.g0_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) + ((vec4<f32>(_e65.g0_.w) * vec4<f32>(_e69.g0_.w)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((_e81.g0_.yxxx * _e84.g0_.yyzw) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0)))));
}

fn point_rotor_outer_product(self_800: Point, other_656: Rotor) -> Point {
    var self_801: Point;
    var other_657: Rotor;

    self_801 = self_800;
    other_657 = other_656;
    let _e4: Point = self_801;
    let _e6: Rotor = other_657;
    return Point((_e4.g0_ * vec4<f32>(_e6.g0_.x)));
}

fn point_rotor_inner_product(self_802: Point, other_658: Rotor) -> PointAndPlane {
    var self_803: Point;
    var other_659: Rotor;

    self_803 = self_802;
    other_659 = other_658;
    let _e4: Point = self_803;
    let _e6: Rotor = other_659;
    let _e11: Point = self_803;
    let _e15: Rotor = other_659;
    let _e26: Point = self_803;
    let _e30: Rotor = other_659;
    let _e42: Point = self_803;
    let _e45: Rotor = other_659;
    return PointAndPlane((_e4.g0_ * vec4<f32>(_e6.g0_.x)), ((((vec4<f32>(_e11.g0_.z) * vec4<f32>(_e15.g0_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) + ((vec4<f32>(_e26.g0_.w) * vec4<f32>(_e30.g0_.w)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((_e42.g0_.yxxx * _e45.g0_.yyzw) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0)))));
}

fn point_rotor_right_contraction(self_804: Point, other_660: Rotor) -> PointAndPlane {
    var self_805: Point;
    var other_661: Rotor;

    self_805 = self_804;
    other_661 = other_660;
    let _e4: Point = self_805;
    let _e6: Rotor = other_661;
    let _e11: Point = self_805;
    let _e15: Rotor = other_661;
    let _e26: Point = self_805;
    let _e30: Rotor = other_661;
    let _e42: Point = self_805;
    let _e45: Rotor = other_661;
    return PointAndPlane((_e4.g0_ * vec4<f32>(_e6.g0_.x)), ((((vec4<f32>(_e11.g0_.z) * vec4<f32>(_e15.g0_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) + ((vec4<f32>(_e26.g0_.w) * vec4<f32>(_e30.g0_.w)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((_e42.g0_.yxxx * _e45.g0_.yyzw) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0)))));
}

fn point_point_add(self_806: Point, other_662: Point) -> Point {
    var self_807: Point;
    var other_663: Point;

    self_807 = self_806;
    other_663 = other_662;
    let _e4: Point = self_807;
    let _e6: Point = other_663;
    return Point((_e4.g0_ + _e6.g0_));
}

fn point_point_sub(self_808: Point, other_664: Point) -> Point {
    var self_809: Point;
    var other_665: Point;

    self_809 = self_808;
    other_665 = other_664;
    let _e4: Point = self_809;
    let _e6: Point = other_665;
    return Point((_e4.g0_ - _e6.g0_));
}

fn point_point_mul(self_810: Point, other_666: Point) -> Point {
    var self_811: Point;
    var other_667: Point;

    self_811 = self_810;
    other_667 = other_666;
    let _e4: Point = self_811;
    let _e6: Point = other_667;
    return Point((_e4.g0_ * _e6.g0_));
}

fn point_point_div(self_812: Point, other_668: Point) -> Point {
    var self_813: Point;
    var other_669: Point;

    self_813 = self_812;
    other_669 = other_668;
    let _e4: Point = self_813;
    let _e7: Point = self_813;
    let _e10: Point = self_813;
    let _e13: Point = self_813;
    let _e23: Point = other_669;
    let _e26: Point = other_669;
    let _e29: Point = other_669;
    let _e32: Point = other_669;
    return Point((((vec4<f32>(_e4.g0_.x, _e7.g0_.y, _e10.g0_.z, _e13.g0_.w) * vec4<f32>(1.0, 1.0, 1.0, 1.0)) / vec4<f32>(_e23.g0_.x, _e26.g0_.y, _e29.g0_.z, _e32.g0_.w)) * vec4<f32>(1.0, 1.0, 1.0, 1.0)));
}

fn point_point_geometric_product(self_814: Point, other_670: Point) -> Translator {
    var self_815: Point;
    var other_671: Point;

    self_815 = self_814;
    other_671 = other_670;
    let _e6: Point = self_815;
    let _e10: Point = other_671;
    let _e14: Point = self_815;
    let _e16: Point = other_671;
    return Translator(((vec4<f32>(0.0) - (vec4<f32>(_e6.g0_.x) * _e10.g0_)) + ((_e14.g0_ * vec4<f32>(_e16.g0_.x)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn point_point_regressive_product(self_816: Point, other_672: Point) -> Line {
    var self_817: Point;
    var other_673: Point;

    self_817 = self_816;
    other_673 = other_672;
    let _e4: Point = self_817;
    let _e8: Point = other_673;
    let _e11: Point = other_673;
    let _e14: Point = other_673;
    let _e25: Point = self_817;
    let _e29: Point = other_673;
    let _e32: Point = other_673;
    let _e35: Point = other_673;
    let _e47: Point = self_817;
    let _e50: Point = self_817;
    let _e53: Point = self_817;
    let _e57: Point = other_673;
    let _e60: Point = other_673;
    let _e63: Point = other_673;
    let _e75: Point = self_817;
    let _e79: Point = other_673;
    let _e82: Point = other_673;
    let _e85: Point = other_673;
    let _e90: Point = self_817;
    let _e93: Point = self_817;
    let _e96: Point = self_817;
    let _e100: Point = other_673;
    return Line(((((vec3<f32>(_e4.g0_.z) * vec3<f32>(_e8.g0_.w, _e11.g0_.w, _e14.g0_.y)) * vec3<f32>(1.0, 0.0, -(1.0))) + ((vec3<f32>(_e25.g0_.w) * vec3<f32>(_e29.g0_.z, _e32.g0_.y, _e35.g0_.z)) * vec3<f32>(-(1.0), 1.0, 0.0))) + ((vec3<f32>(_e47.g0_.x, _e50.g0_.y, _e53.g0_.y) * vec3<f32>(_e57.g0_.x, _e60.g0_.w, _e63.g0_.z)) * vec3<f32>(0.0, -(1.0), 1.0))), ((vec3<f32>(_e75.g0_.x) * vec3<f32>(_e79.g0_.y, _e82.g0_.z, _e85.g0_.w)) + ((vec3<f32>(_e90.g0_.y, _e93.g0_.z, _e96.g0_.w) * vec3<f32>(_e100.g0_.x)) * vec3<f32>(-(1.0)))));
}

fn point_point_inner_product(self_818: Point, other_674: Point) -> Scalar {
    var self_819: Point;
    var other_675: Point;

    self_819 = self_818;
    other_675 = other_674;
    let _e5: Point = self_819;
    let _e8: Point = other_675;
    return Scalar((0.0 - (_e5.g0_.x * _e8.g0_.x)));
}

fn point_point_inner_anti_product(self_820: Point, other_676: Point) -> AntiScalar {
    var self_821: Point;
    var other_677: Point;

    self_821 = self_820;
    other_677 = other_676;
    let _e4: Point = self_821;
    let _e7: Point = other_677;
    let _e11: Point = self_821;
    let _e14: Point = other_677;
    let _e19: Point = self_821;
    let _e22: Point = other_677;
    return AntiScalar((((_e4.g0_.y * _e7.g0_.y) + (_e11.g0_.z * _e14.g0_.z)) + (_e19.g0_.w * _e22.g0_.w)));
}

fn point_point_left_contraction(self_822: Point, other_678: Point) -> Scalar {
    var self_823: Point;
    var other_679: Point;

    self_823 = self_822;
    other_679 = other_678;
    let _e5: Point = self_823;
    let _e8: Point = other_679;
    return Scalar((0.0 - (_e5.g0_.x * _e8.g0_.x)));
}

fn point_point_right_contraction(self_824: Point, other_680: Point) -> Scalar {
    var self_825: Point;
    var other_681: Point;

    self_825 = self_824;
    other_681 = other_680;
    let _e5: Point = self_825;
    let _e8: Point = other_681;
    return Scalar((0.0 - (_e5.g0_.x * _e8.g0_.x)));
}

fn point_point_left_anti_contraction(self_826: Point, other_682: Point) -> AntiScalar {
    var self_827: Point;
    var other_683: Point;

    self_827 = self_826;
    other_683 = other_682;
    let _e4: Point = self_827;
    let _e7: Point = other_683;
    let _e11: Point = self_827;
    let _e14: Point = other_683;
    let _e19: Point = self_827;
    let _e22: Point = other_683;
    return AntiScalar((((_e4.g0_.y * _e7.g0_.y) + (_e11.g0_.z * _e14.g0_.z)) + (_e19.g0_.w * _e22.g0_.w)));
}

fn point_point_right_anti_contraction(self_828: Point, other_684: Point) -> AntiScalar {
    var self_829: Point;
    var other_685: Point;

    self_829 = self_828;
    other_685 = other_684;
    let _e4: Point = self_829;
    let _e7: Point = other_685;
    let _e11: Point = self_829;
    let _e14: Point = other_685;
    let _e19: Point = self_829;
    let _e22: Point = other_685;
    return AntiScalar((((_e4.g0_.y * _e7.g0_.y) + (_e11.g0_.z * _e14.g0_.z)) + (_e19.g0_.w * _e22.g0_.w)));
}

fn point_point_scalar_product(self_830: Point, other_686: Point) -> Scalar {
    var self_831: Point;
    var other_687: Point;

    self_831 = self_830;
    other_687 = other_686;
    let _e5: Point = self_831;
    let _e8: Point = other_687;
    return Scalar((0.0 - (_e5.g0_.x * _e8.g0_.x)));
}

fn point_point_anti_scalar_product(self_832: Point, other_688: Point) -> AntiScalar {
    var self_833: Point;
    var other_689: Point;

    self_833 = self_832;
    other_689 = other_688;
    let _e4: Point = self_833;
    let _e7: Point = other_689;
    let _e11: Point = self_833;
    let _e14: Point = other_689;
    let _e19: Point = self_833;
    let _e22: Point = other_689;
    return AntiScalar((((_e4.g0_.y * _e7.g0_.y) + (_e11.g0_.z * _e14.g0_.z)) + (_e19.g0_.w * _e22.g0_.w)));
}

fn point_ideal_point_regressive_product(self_834: Point, other_690: IdealPoint) -> Plane {
    var self_835: Point;
    var other_691: IdealPoint;

    self_835 = self_834;
    other_691 = other_690;
    let _e4: Point = self_835;
    let _e8: IdealPoint = other_691;
    let _e20: Point = self_835;
    let _e24: IdealPoint = other_691;
    let _e37: Point = self_835;
    let _e40: IdealPoint = other_691;
    let _e43: IdealPoint = other_691;
    let _e46: IdealPoint = other_691;
    let _e49: IdealPoint = other_691;
    return Plane(((((vec4<f32>(_e4.g0_.z) * vec4<f32>(_e8.g0_.y)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0)) + ((vec4<f32>(_e20.g0_.w) * vec4<f32>(_e24.g0_.z)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((_e37.g0_.yxxx * vec4<f32>(_e40.g0_.x, _e43.g0_.x, _e46.g0_.y, _e49.g0_.z)) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0))));
}

fn point_plane_add(self_836: Point, other_692: Plane) -> PointAndPlane {
    var self_837: Point;
    var other_693: Plane;

    self_837 = self_836;
    other_693 = other_692;
    let _e4: Point = self_837;
    let _e6: Plane = other_693;
    return PointAndPlane(_e4.g0_, _e6.g0_);
}

fn point_plane_sub(self_838: Point, other_694: Plane) -> PointAndPlane {
    var self_839: Point;
    var other_695: Plane;

    self_839 = self_838;
    other_695 = other_694;
    let _e4: Point = self_839;
    let _e8: Plane = other_695;
    return PointAndPlane(_e4.g0_, (vec4<f32>(0.0) - _e8.g0_));
}

fn point_plane_regressive_product(self_840: Point, other_696: Plane) -> Scalar {
    var self_841: Point;
    var other_697: Plane;

    self_841 = self_840;
    other_697 = other_696;
    let _e5: Point = self_841;
    let _e8: Plane = other_697;
    let _e13: Point = self_841;
    let _e16: Plane = other_697;
    let _e21: Point = self_841;
    let _e24: Plane = other_697;
    let _e29: Point = self_841;
    let _e32: Plane = other_697;
    return Scalar(((((0.0 - (_e5.g0_.x * _e8.g0_.x)) - (_e13.g0_.y * _e16.g0_.y)) - (_e21.g0_.z * _e24.g0_.z)) - (_e29.g0_.w * _e32.g0_.w)));
}

fn point_plane_outer_product(self_842: Point, other_698: Plane) -> AntiScalar {
    var self_843: Point;
    var other_699: Plane;

    self_843 = self_842;
    other_699 = other_698;
    let _e5: Point = self_843;
    let _e8: Plane = other_699;
    let _e13: Point = self_843;
    let _e16: Plane = other_699;
    let _e21: Point = self_843;
    let _e24: Plane = other_699;
    let _e29: Point = self_843;
    let _e32: Plane = other_699;
    return AntiScalar(((((0.0 - (_e5.g0_.x * _e8.g0_.x)) - (_e13.g0_.y * _e16.g0_.y)) - (_e21.g0_.z * _e24.g0_.z)) - (_e29.g0_.w * _e32.g0_.w)));
}

fn point_plane_inner_product(self_844: Point, other_700: Plane) -> Line {
    var self_845: Point;
    var other_701: Plane;

    self_845 = self_844;
    other_701 = other_700;
    let _e4: Point = self_845;
    let _e8: Plane = other_701;
    let _e11: Plane = other_701;
    let _e14: Plane = other_701;
    let _e25: Point = self_845;
    let _e29: Plane = other_701;
    let _e32: Plane = other_701;
    let _e35: Plane = other_701;
    let _e47: Point = self_845;
    let _e50: Point = self_845;
    let _e53: Point = self_845;
    let _e57: Plane = other_701;
    let _e60: Plane = other_701;
    let _e63: Plane = other_701;
    let _e75: Point = self_845;
    let _e79: Plane = other_701;
    let _e82: Plane = other_701;
    let _e85: Plane = other_701;
    return Line(((((vec3<f32>(_e4.g0_.z) * vec3<f32>(_e8.g0_.w, _e11.g0_.w, _e14.g0_.y)) * vec3<f32>(1.0, 0.0, -(1.0))) + ((vec3<f32>(_e25.g0_.w) * vec3<f32>(_e29.g0_.z, _e32.g0_.y, _e35.g0_.z)) * vec3<f32>(-(1.0), 1.0, 0.0))) + ((vec3<f32>(_e47.g0_.x, _e50.g0_.y, _e53.g0_.y) * vec3<f32>(_e57.g0_.x, _e60.g0_.w, _e63.g0_.z)) * vec3<f32>(0.0, -(1.0), 1.0))), (vec3<f32>(_e75.g0_.x) * vec3<f32>(_e79.g0_.y, _e82.g0_.z, _e85.g0_.w)));
}

fn point_plane_inner_anti_product(self_846: Point, other_702: Plane) -> Line {
    var self_847: Point;
    var other_703: Plane;

    self_847 = self_846;
    other_703 = other_702;
    let _e4: Point = self_847;
    let _e7: Point = self_847;
    let _e10: Point = self_847;
    let _e14: Plane = other_703;
    let _e23: Point = self_847;
    let _e27: Plane = other_703;
    let _e30: Plane = other_703;
    let _e33: Plane = other_703;
    let _e44: Point = self_847;
    let _e48: Plane = other_703;
    let _e51: Plane = other_703;
    let _e54: Plane = other_703;
    let _e66: Point = self_847;
    let _e69: Point = self_847;
    let _e72: Point = self_847;
    let _e76: Plane = other_703;
    let _e79: Plane = other_703;
    let _e82: Plane = other_703;
    return Line(((vec3<f32>(_e4.g0_.y, _e7.g0_.z, _e10.g0_.w) * vec3<f32>(_e14.g0_.x)) * vec3<f32>(-(1.0))), ((((vec3<f32>(_e23.g0_.z) * vec3<f32>(_e27.g0_.w, _e30.g0_.w, _e33.g0_.y)) * vec3<f32>(1.0, 0.0, -(1.0))) + ((vec3<f32>(_e44.g0_.w) * vec3<f32>(_e48.g0_.z, _e51.g0_.y, _e54.g0_.z)) * vec3<f32>(-(1.0), 1.0, 0.0))) + ((vec3<f32>(_e66.g0_.x, _e69.g0_.y, _e72.g0_.y) * vec3<f32>(_e76.g0_.x, _e79.g0_.w, _e82.g0_.z)) * vec3<f32>(0.0, -(1.0), 1.0))));
}

fn point_plane_right_contraction(self_848: Point, other_704: Plane) -> Line {
    var self_849: Point;
    var other_705: Plane;

    self_849 = self_848;
    other_705 = other_704;
    let _e4: Point = self_849;
    let _e8: Plane = other_705;
    let _e11: Plane = other_705;
    let _e14: Plane = other_705;
    let _e25: Point = self_849;
    let _e29: Plane = other_705;
    let _e32: Plane = other_705;
    let _e35: Plane = other_705;
    let _e47: Point = self_849;
    let _e50: Point = self_849;
    let _e53: Point = self_849;
    let _e57: Plane = other_705;
    let _e60: Plane = other_705;
    let _e63: Plane = other_705;
    let _e75: Point = self_849;
    let _e79: Plane = other_705;
    let _e82: Plane = other_705;
    let _e85: Plane = other_705;
    return Line(((((vec3<f32>(_e4.g0_.z) * vec3<f32>(_e8.g0_.w, _e11.g0_.w, _e14.g0_.y)) * vec3<f32>(1.0, 0.0, -(1.0))) + ((vec3<f32>(_e25.g0_.w) * vec3<f32>(_e29.g0_.z, _e32.g0_.y, _e35.g0_.z)) * vec3<f32>(-(1.0), 1.0, 0.0))) + ((vec3<f32>(_e47.g0_.x, _e50.g0_.y, _e53.g0_.y) * vec3<f32>(_e57.g0_.x, _e60.g0_.w, _e63.g0_.z)) * vec3<f32>(0.0, -(1.0), 1.0))), (vec3<f32>(_e75.g0_.x) * vec3<f32>(_e79.g0_.y, _e82.g0_.z, _e85.g0_.w)));
}

fn point_plane_left_anti_contraction(self_850: Point, other_706: Plane) -> Line {
    var self_851: Point;
    var other_707: Plane;

    self_851 = self_850;
    other_707 = other_706;
    let _e4: Point = self_851;
    let _e7: Point = self_851;
    let _e10: Point = self_851;
    let _e14: Plane = other_707;
    let _e23: Point = self_851;
    let _e27: Plane = other_707;
    let _e30: Plane = other_707;
    let _e33: Plane = other_707;
    let _e44: Point = self_851;
    let _e48: Plane = other_707;
    let _e51: Plane = other_707;
    let _e54: Plane = other_707;
    let _e66: Point = self_851;
    let _e69: Point = self_851;
    let _e72: Point = self_851;
    let _e76: Plane = other_707;
    let _e79: Plane = other_707;
    let _e82: Plane = other_707;
    return Line(((vec3<f32>(_e4.g0_.y, _e7.g0_.z, _e10.g0_.w) * vec3<f32>(_e14.g0_.x)) * vec3<f32>(-(1.0))), ((((vec3<f32>(_e23.g0_.z) * vec3<f32>(_e27.g0_.w, _e30.g0_.w, _e33.g0_.y)) * vec3<f32>(1.0, 0.0, -(1.0))) + ((vec3<f32>(_e44.g0_.w) * vec3<f32>(_e48.g0_.z, _e51.g0_.y, _e54.g0_.z)) * vec3<f32>(-(1.0), 1.0, 0.0))) + ((vec3<f32>(_e66.g0_.x, _e69.g0_.y, _e72.g0_.y) * vec3<f32>(_e76.g0_.x, _e79.g0_.w, _e82.g0_.z)) * vec3<f32>(0.0, -(1.0), 1.0))));
}

fn point_line_regressive_product(self_852: Point, other_708: Line) -> Plane {
    var self_853: Point;
    var other_709: Line;

    self_853 = self_852;
    other_709 = other_708;
    let _e4: Point = self_853;
    let _e8: Line = other_709;
    let _e11: Line = other_709;
    let _e14: Line = other_709;
    let _e17: Line = other_709;
    let _e30: Point = self_853;
    let _e34: Line = other_709;
    let _e37: Line = other_709;
    let _e40: Line = other_709;
    let _e43: Line = other_709;
    let _e57: Point = self_853;
    let _e61: Line = other_709;
    let _e64: Line = other_709;
    let _e67: Line = other_709;
    let _e70: Line = other_709;
    let _e84: Point = self_853;
    let _e88: Line = other_709;
    let _e91: Line = other_709;
    let _e94: Line = other_709;
    let _e97: Line = other_709;
    return Plane((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.x, _e11.g0_.x, _e14.g1_.z, _e17.g1_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e30.g0_.z) * vec4<f32>(_e34.g0_.y, _e37.g1_.z, _e40.g0_.y, _e43.g1_.x)) * vec4<f32>(-(1.0), -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e57.g0_.w) * vec4<f32>(_e61.g0_.z, _e64.g1_.y, _e67.g1_.x, _e70.g0_.z)) * vec4<f32>(-(1.0), 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e84.g0_.x) * vec4<f32>(_e88.g0_.x, _e91.g0_.x, _e94.g0_.y, _e97.g0_.z)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn point_line_inner_product(self_854: Point, other_710: Line) -> Plane {
    var self_855: Point;
    var other_711: Line;

    self_855 = self_854;
    other_711 = other_710;
    let _e4: Point = self_855;
    let _e8: Line = other_711;
    let _e19: Point = self_855;
    let _e23: Line = other_711;
    let _e35: Point = self_855;
    let _e38: Line = other_711;
    let _e41: Line = other_711;
    let _e44: Line = other_711;
    let _e47: Line = other_711;
    return Plane(((((vec4<f32>(_e4.g0_.z) * vec4<f32>(_e8.g1_.y)) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) + ((vec4<f32>(_e19.g0_.w) * vec4<f32>(_e23.g1_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((_e35.g0_.yxxx * vec4<f32>(_e38.g1_.x, _e41.g1_.x, _e44.g1_.y, _e47.g1_.z)) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0)))));
}

fn point_line_geometric_anti_product(self_856: Point, other_712: Line) -> PointAndPlane {
    var self_857: Point;
    var other_713: Line;

    self_857 = self_856;
    other_713 = other_712;
    let _e4: Point = self_857;
    let _e8: Line = other_713;
    let _e11: Line = other_713;
    let _e14: Line = other_713;
    let _e17: Line = other_713;
    let _e30: Point = self_857;
    let _e34: Line = other_713;
    let _e37: Line = other_713;
    let _e40: Line = other_713;
    let _e43: Line = other_713;
    let _e57: Point = self_857;
    let _e60: Line = other_713;
    let _e63: Line = other_713;
    let _e66: Line = other_713;
    let _e69: Line = other_713;
    let _e83: Point = self_857;
    let _e87: Line = other_713;
    let _e90: Line = other_713;
    let _e93: Line = other_713;
    let _e96: Line = other_713;
    let _e109: Point = self_857;
    let _e113: Line = other_713;
    let _e116: Line = other_713;
    let _e119: Line = other_713;
    let _e122: Line = other_713;
    let _e136: Point = self_857;
    let _e140: Line = other_713;
    let _e143: Line = other_713;
    let _e146: Line = other_713;
    let _e149: Line = other_713;
    let _e163: Point = self_857;
    let _e167: Line = other_713;
    let _e170: Line = other_713;
    let _e173: Line = other_713;
    let _e176: Line = other_713;
    return PointAndPlane(((((vec4<f32>(_e4.g0_.z) * vec4<f32>(_e8.g1_.y, _e11.g0_.z, _e14.g1_.y, _e17.g0_.x)) * vec4<f32>(-(1.0), -(1.0), 0.0, 1.0)) + ((vec4<f32>(_e30.g0_.w) * vec4<f32>(_e34.g1_.z, _e37.g0_.y, _e40.g0_.x, _e43.g1_.z)) * vec4<f32>(-(1.0), 1.0, -(1.0), 0.0))) + ((_e57.g0_.yxyy * vec4<f32>(_e60.g1_.x, _e63.g1_.x, _e66.g0_.z, _e69.g0_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0)))), (((((vec4<f32>(_e83.g0_.y) * vec4<f32>(_e87.g0_.x, _e90.g0_.x, _e93.g1_.z, _e96.g1_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e109.g0_.z) * vec4<f32>(_e113.g0_.y, _e116.g1_.z, _e119.g0_.y, _e122.g1_.x)) * vec4<f32>(-(1.0), -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e136.g0_.w) * vec4<f32>(_e140.g0_.z, _e143.g1_.y, _e146.g1_.x, _e149.g0_.z)) * vec4<f32>(-(1.0), 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e163.g0_.x) * vec4<f32>(_e167.g0_.x, _e170.g0_.x, _e173.g0_.y, _e176.g0_.z)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn point_line_inner_anti_product(self_858: Point, other_714: Line) -> Point {
    var self_859: Point;
    var other_715: Line;

    self_859 = self_858;
    other_715 = other_714;
    let _e4: Point = self_859;
    let _e8: Line = other_715;
    let _e11: Line = other_715;
    let _e14: Line = other_715;
    let _e17: Line = other_715;
    let _e30: Point = self_859;
    let _e34: Line = other_715;
    let _e37: Line = other_715;
    let _e40: Line = other_715;
    let _e43: Line = other_715;
    let _e57: Point = self_859;
    let _e60: Line = other_715;
    let _e63: Line = other_715;
    let _e66: Line = other_715;
    let _e69: Line = other_715;
    return Point(((((vec4<f32>(_e4.g0_.z) * vec4<f32>(_e8.g1_.y, _e11.g0_.z, _e14.g1_.y, _e17.g0_.x)) * vec4<f32>(-(1.0), -(1.0), 0.0, 1.0)) + ((vec4<f32>(_e30.g0_.w) * vec4<f32>(_e34.g1_.z, _e37.g0_.y, _e40.g0_.x, _e43.g1_.z)) * vec4<f32>(-(1.0), 1.0, -(1.0), 0.0))) + ((_e57.g0_.yxyy * vec4<f32>(_e60.g1_.x, _e63.g1_.x, _e66.g0_.z, _e69.g0_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0)))));
}

fn point_line_right_contraction(self_860: Point, other_716: Line) -> Plane {
    var self_861: Point;
    var other_717: Line;

    self_861 = self_860;
    other_717 = other_716;
    let _e4: Point = self_861;
    let _e8: Line = other_717;
    let _e19: Point = self_861;
    let _e23: Line = other_717;
    let _e35: Point = self_861;
    let _e38: Line = other_717;
    let _e41: Line = other_717;
    let _e44: Line = other_717;
    let _e47: Line = other_717;
    return Plane(((((vec4<f32>(_e4.g0_.z) * vec4<f32>(_e8.g1_.y)) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) + ((vec4<f32>(_e19.g0_.w) * vec4<f32>(_e23.g1_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((_e35.g0_.yxxx * vec4<f32>(_e38.g1_.x, _e41.g1_.x, _e44.g1_.y, _e47.g1_.z)) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0)))));
}

fn point_line_left_anti_contraction(self_862: Point, other_718: Line) -> Point {
    var self_863: Point;
    var other_719: Line;

    self_863 = self_862;
    other_719 = other_718;
    let _e4: Point = self_863;
    let _e8: Line = other_719;
    let _e11: Line = other_719;
    let _e14: Line = other_719;
    let _e17: Line = other_719;
    let _e30: Point = self_863;
    let _e34: Line = other_719;
    let _e37: Line = other_719;
    let _e40: Line = other_719;
    let _e43: Line = other_719;
    let _e57: Point = self_863;
    let _e60: Line = other_719;
    let _e63: Line = other_719;
    let _e66: Line = other_719;
    let _e69: Line = other_719;
    return Point(((((vec4<f32>(_e4.g0_.z) * vec4<f32>(_e8.g1_.y, _e11.g0_.z, _e14.g1_.y, _e17.g0_.x)) * vec4<f32>(-(1.0), -(1.0), 0.0, 1.0)) + ((vec4<f32>(_e30.g0_.w) * vec4<f32>(_e34.g1_.z, _e37.g0_.y, _e40.g0_.x, _e43.g1_.z)) * vec4<f32>(-(1.0), 1.0, -(1.0), 0.0))) + ((_e57.g0_.yxyy * vec4<f32>(_e60.g1_.x, _e63.g1_.x, _e66.g0_.z, _e69.g0_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0)))));
}

fn point_translator_geometric_product(self_864: Point, other_720: Translator) -> Point {
    var self_865: Point;
    var other_721: Translator;

    self_865 = self_864;
    other_721 = other_720;
    let _e4: Point = self_865;
    let _e8: Translator = other_721;
    let _e11: Point = self_865;
    let _e13: Translator = other_721;
    return Point(((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((_e11.g0_ * vec4<f32>(_e13.g0_.x)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn point_translator_regressive_product(self_866: Point, other_722: Translator) -> Plane {
    var self_867: Point;
    var other_723: Translator;

    self_867 = self_866;
    other_723 = other_722;
    let _e4: Point = self_867;
    let _e8: Translator = other_723;
    let _e20: Point = self_867;
    let _e24: Translator = other_723;
    let _e37: Point = self_867;
    let _e40: Translator = other_723;
    return Plane(((((vec4<f32>(_e4.g0_.z) * vec4<f32>(_e8.g0_.z)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0)) + ((vec4<f32>(_e20.g0_.w) * vec4<f32>(_e24.g0_.w)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((_e37.g0_.yxxx * _e40.g0_.yyzw) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0))));
}

fn point_translator_outer_product(self_868: Point, other_724: Translator) -> Point {
    var self_869: Point;
    var other_725: Translator;

    self_869 = self_868;
    other_725 = other_724;
    let _e4: Point = self_869;
    let _e6: Translator = other_725;
    return Point((_e4.g0_ * vec4<f32>(_e6.g0_.x)));
}

fn point_translator_inner_product(self_870: Point, other_726: Translator) -> Point {
    var self_871: Point;
    var other_727: Translator;

    self_871 = self_870;
    other_727 = other_726;
    let _e4: Point = self_871;
    let _e6: Translator = other_727;
    return Point((_e4.g0_ * vec4<f32>(_e6.g0_.x)));
}

fn point_translator_right_contraction(self_872: Point, other_728: Translator) -> Point {
    var self_873: Point;
    var other_729: Translator;

    self_873 = self_872;
    other_729 = other_728;
    let _e4: Point = self_873;
    let _e6: Translator = other_729;
    return Point((_e4.g0_ * vec4<f32>(_e6.g0_.x)));
}

fn point_motor_geometric_product(self_874: Point, other_730: Motor) -> PointAndPlane {
    var self_875: Point;
    var other_731: Motor;

    self_875 = self_874;
    other_731 = other_730;
    let _e4: Point = self_875;
    let _e8: Motor = other_731;
    let _e11: Motor = other_731;
    let _e14: Motor = other_731;
    let _e17: Motor = other_731;
    let _e22: Point = self_875;
    let _e26: Motor = other_731;
    let _e38: Point = self_875;
    let _e42: Motor = other_731;
    let _e54: Point = self_875;
    let _e57: Motor = other_731;
    let _e69: Point = self_875;
    let _e73: Motor = other_731;
    let _e76: Motor = other_731;
    let _e79: Motor = other_731;
    let _e82: Motor = other_731;
    let _e96: Point = self_875;
    let _e100: Motor = other_731;
    let _e112: Point = self_875;
    let _e116: Motor = other_731;
    let _e128: Point = self_875;
    let _e131: Motor = other_731;
    return PointAndPlane(((((vec4<f32>(_e4.g0_.x) * vec4<f32>(_e8.g0_.x, _e11.g1_.y, _e14.g1_.z, _e17.g1_.w)) + ((vec4<f32>(_e22.g0_.z) * _e26.g0_.wwxy) * vec4<f32>(0.0, -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e38.g0_.w) * _e42.g0_.zzyx) * vec4<f32>(0.0, 1.0, -(1.0), 1.0))) + ((_e54.g0_.xyyy * _e57.g0_.xxwz) * vec4<f32>(0.0, 1.0, 1.0, -(1.0)))), (((((vec4<f32>(_e69.g0_.x) * vec4<f32>(_e73.g1_.x, _e76.g0_.y, _e79.g0_.z, _e82.g0_.w)) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))) + ((vec4<f32>(_e96.g0_.z) * vec4<f32>(_e100.g0_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e112.g0_.w) * vec4<f32>(_e116.g0_.w)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((_e128.g0_.yxxx * _e131.g0_.yxxx) * vec4<f32>(1.0, 0.0, 0.0, 0.0))));
}

fn point_motor_regressive_product(self_876: Point, other_732: Motor) -> PointAndPlane {
    var self_877: Point;
    var other_733: Motor;

    self_877 = self_876;
    other_733 = other_732;
    let _e4: Point = self_877;
    let _e6: Motor = other_733;
    let _e11: Point = self_877;
    let _e15: Motor = other_733;
    let _e18: Motor = other_733;
    let _e21: Motor = other_733;
    let _e24: Motor = other_733;
    let _e37: Point = self_877;
    let _e41: Motor = other_733;
    let _e44: Motor = other_733;
    let _e47: Motor = other_733;
    let _e50: Motor = other_733;
    let _e64: Point = self_877;
    let _e68: Motor = other_733;
    let _e71: Motor = other_733;
    let _e74: Motor = other_733;
    let _e77: Motor = other_733;
    let _e91: Point = self_877;
    let _e95: Motor = other_733;
    return PointAndPlane((_e4.g0_ * vec4<f32>(_e6.g1_.x)), (((((vec4<f32>(_e11.g0_.y) * vec4<f32>(_e15.g1_.y, _e18.g1_.y, _e21.g0_.w, _e24.g0_.z)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e37.g0_.z) * vec4<f32>(_e41.g1_.z, _e44.g0_.w, _e47.g1_.z, _e50.g0_.y)) * vec4<f32>(-(1.0), -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e64.g0_.w) * vec4<f32>(_e68.g1_.w, _e71.g0_.z, _e74.g0_.y, _e77.g1_.w)) * vec4<f32>(-(1.0), 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e91.g0_.x) * _e95.g1_) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn point_motor_outer_product(self_878: Point, other_734: Motor) -> Point {
    var self_879: Point;
    var other_735: Motor;

    self_879 = self_878;
    other_735 = other_734;
    let _e4: Point = self_879;
    let _e6: Motor = other_735;
    return Point((_e4.g0_ * vec4<f32>(_e6.g0_.x)));
}

fn point_motor_inner_product(self_880: Point, other_736: Motor) -> PointAndPlane {
    var self_881: Point;
    var other_737: Motor;

    self_881 = self_880;
    other_737 = other_736;
    let _e4: Point = self_881;
    let _e6: Motor = other_737;
    let _e11: Point = self_881;
    let _e15: Motor = other_737;
    let _e18: Motor = other_737;
    let _e21: Motor = other_737;
    let _e24: Motor = other_737;
    let _e38: Point = self_881;
    let _e42: Motor = other_737;
    let _e54: Point = self_881;
    let _e58: Motor = other_737;
    let _e70: Point = self_881;
    let _e73: Motor = other_737;
    return PointAndPlane((_e4.g0_ * vec4<f32>(_e6.g0_.x)), (((((vec4<f32>(_e11.g0_.x) * vec4<f32>(_e15.g1_.x, _e18.g0_.y, _e21.g0_.z, _e24.g0_.w)) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))) + ((vec4<f32>(_e38.g0_.z) * vec4<f32>(_e42.g0_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e54.g0_.w) * vec4<f32>(_e58.g0_.w)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((_e70.g0_.yxxx * _e73.g0_.yxxx) * vec4<f32>(1.0, 0.0, 0.0, 0.0))));
}

fn point_motor_geometric_anti_product(self_882: Point, other_738: Motor) -> PointAndPlane {
    var self_883: Point;
    var other_739: Motor;

    self_883 = self_882;
    other_739 = other_738;
    let _e4: Point = self_883;
    let _e8: Motor = other_739;
    let _e11: Motor = other_739;
    let _e14: Motor = other_739;
    let _e17: Motor = other_739;
    let _e30: Point = self_883;
    let _e34: Motor = other_739;
    let _e37: Motor = other_739;
    let _e40: Motor = other_739;
    let _e43: Motor = other_739;
    let _e57: Point = self_883;
    let _e61: Motor = other_739;
    let _e64: Motor = other_739;
    let _e67: Motor = other_739;
    let _e70: Motor = other_739;
    let _e84: Point = self_883;
    let _e88: Motor = other_739;
    let _e100: Point = self_883;
    let _e104: Motor = other_739;
    let _e107: Motor = other_739;
    let _e110: Motor = other_739;
    let _e113: Motor = other_739;
    let _e127: Point = self_883;
    let _e131: Motor = other_739;
    let _e134: Motor = other_739;
    let _e137: Motor = other_739;
    let _e140: Motor = other_739;
    let _e155: Point = self_883;
    let _e159: Motor = other_739;
    let _e162: Motor = other_739;
    let _e165: Motor = other_739;
    let _e168: Motor = other_739;
    let _e183: Point = self_883;
    let _e187: Motor = other_739;
    return PointAndPlane((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y, _e11.g1_.x, _e14.g1_.w, _e17.g1_.z)) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0))) + ((vec4<f32>(_e30.g0_.z) * vec4<f32>(_e34.g0_.z, _e37.g1_.w, _e40.g1_.x, _e43.g1_.y)) * vec4<f32>(-(1.0), -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e57.g0_.w) * vec4<f32>(_e61.g0_.w, _e64.g1_.z, _e67.g1_.y, _e70.g1_.x)) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))) + ((vec4<f32>(_e84.g0_.x) * vec4<f32>(_e88.g1_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))), (((((vec4<f32>(_e100.g0_.y) * vec4<f32>(_e104.g1_.y, _e107.g0_.x, _e110.g0_.w, _e113.g0_.z)) * vec4<f32>(-(1.0), -(1.0), 1.0, -(1.0))) + ((vec4<f32>(_e127.g0_.z) * vec4<f32>(_e131.g1_.z, _e134.g0_.w, _e137.g0_.x, _e140.g0_.y)) * vec4<f32>(-(1.0), -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e155.g0_.w) * vec4<f32>(_e159.g1_.w, _e162.g0_.z, _e165.g0_.y, _e168.g0_.x)) * vec4<f32>(-(1.0), 1.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e183.g0_.x) * _e187.g1_) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn point_motor_right_contraction(self_884: Point, other_740: Motor) -> PointAndPlane {
    var self_885: Point;
    var other_741: Motor;

    self_885 = self_884;
    other_741 = other_740;
    let _e4: Point = self_885;
    let _e6: Motor = other_741;
    let _e11: Point = self_885;
    let _e15: Motor = other_741;
    let _e26: Point = self_885;
    let _e30: Motor = other_741;
    let _e42: Point = self_885;
    let _e45: Motor = other_741;
    return PointAndPlane((_e4.g0_ * vec4<f32>(_e6.g0_.x)), ((((vec4<f32>(_e11.g0_.z) * vec4<f32>(_e15.g0_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) + ((vec4<f32>(_e26.g0_.w) * vec4<f32>(_e30.g0_.w)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((_e42.g0_.yxxx * _e45.g0_.yyzw) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0)))));
}

fn point_motor_right_anti_contraction(self_886: Point, other_742: Motor) -> Point {
    var self_887: Point;
    var other_743: Motor;

    self_887 = self_886;
    other_743 = other_742;
    let _e4: Point = self_887;
    let _e6: Motor = other_743;
    return Point((_e4.g0_ * vec4<f32>(_e6.g1_.x)));
}

fn point_point_and_plane_add(self_888: Point, other_744: PointAndPlane) -> PointAndPlane {
    var self_889: Point;
    var other_745: PointAndPlane;

    self_889 = self_888;
    other_745 = other_744;
    let _e4: Point = self_889;
    let _e6: PointAndPlane = other_745;
    let _e9: PointAndPlane = other_745;
    return PointAndPlane((_e4.g0_ + _e6.g0_), _e9.g1_);
}

fn point_point_and_plane_sub(self_890: Point, other_746: PointAndPlane) -> PointAndPlane {
    var self_891: Point;
    var other_747: PointAndPlane;

    self_891 = self_890;
    other_747 = other_746;
    let _e4: Point = self_891;
    let _e6: PointAndPlane = other_747;
    let _e11: PointAndPlane = other_747;
    return PointAndPlane((_e4.g0_ - _e6.g0_), (vec4<f32>(0.0) - _e11.g1_));
}

fn point_point_and_plane_geometric_product(self_892: Point, other_748: PointAndPlane) -> Motor {
    var self_893: Point;
    var other_749: PointAndPlane;

    self_893 = self_892;
    other_749 = other_748;
    let _e4: Point = self_893;
    let _e8: PointAndPlane = other_749;
    let _e11: PointAndPlane = other_749;
    let _e14: PointAndPlane = other_749;
    let _e17: PointAndPlane = other_749;
    let _e31: Point = self_893;
    let _e35: PointAndPlane = other_749;
    let _e38: PointAndPlane = other_749;
    let _e41: PointAndPlane = other_749;
    let _e44: PointAndPlane = other_749;
    let _e50: Point = self_893;
    let _e54: PointAndPlane = other_749;
    let _e57: PointAndPlane = other_749;
    let _e60: PointAndPlane = other_749;
    let _e63: PointAndPlane = other_749;
    let _e77: Point = self_893;
    let _e81: PointAndPlane = other_749;
    let _e84: PointAndPlane = other_749;
    let _e87: PointAndPlane = other_749;
    let _e90: PointAndPlane = other_749;
    let _e104: Point = self_893;
    let _e108: PointAndPlane = other_749;
    let _e111: PointAndPlane = other_749;
    let _e114: PointAndPlane = other_749;
    let _e117: PointAndPlane = other_749;
    return Motor(((vec4<f32>(_e4.g0_.x) * vec4<f32>(_e8.g0_.x, _e11.g1_.y, _e14.g1_.z, _e17.g1_.w)) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0)), ((((vec4<f32>(0.0) - (vec4<f32>(_e31.g0_.x) * vec4<f32>(_e35.g1_.x, _e38.g0_.y, _e41.g0_.z, _e44.g0_.w))) + ((vec4<f32>(_e50.g0_.y) * vec4<f32>(_e54.g1_.y, _e57.g0_.x, _e60.g1_.w, _e63.g1_.z)) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))) + ((vec4<f32>(_e77.g0_.z) * vec4<f32>(_e81.g1_.z, _e84.g1_.w, _e87.g0_.x, _e90.g1_.y)) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e104.g0_.w) * vec4<f32>(_e108.g1_.w, _e111.g1_.z, _e114.g1_.y, _e117.g0_.x)) * vec4<f32>(-(1.0), -(1.0), 1.0, 1.0))));
}

fn point_point_and_plane_outer_product(self_894: Point, other_750: PointAndPlane) -> AntiScalar {
    var self_895: Point;
    var other_751: PointAndPlane;

    self_895 = self_894;
    other_751 = other_750;
    let _e5: Point = self_895;
    let _e8: PointAndPlane = other_751;
    let _e13: Point = self_895;
    let _e16: PointAndPlane = other_751;
    let _e21: Point = self_895;
    let _e24: PointAndPlane = other_751;
    let _e29: Point = self_895;
    let _e32: PointAndPlane = other_751;
    return AntiScalar(((((0.0 - (_e5.g0_.x * _e8.g1_.x)) - (_e13.g0_.y * _e16.g1_.y)) - (_e21.g0_.z * _e24.g1_.z)) - (_e29.g0_.w * _e32.g1_.w)));
}

fn point_point_and_plane_geometric_anti_product(self_896: Point, other_752: PointAndPlane) -> Motor {
    var self_897: Point;
    var other_753: PointAndPlane;

    self_897 = self_896;
    other_753 = other_752;
    let _e4: Point = self_897;
    let _e8: PointAndPlane = other_753;
    let _e11: PointAndPlane = other_753;
    let _e14: PointAndPlane = other_753;
    let _e17: PointAndPlane = other_753;
    let _e29: Point = self_897;
    let _e33: PointAndPlane = other_753;
    let _e36: PointAndPlane = other_753;
    let _e39: PointAndPlane = other_753;
    let _e42: PointAndPlane = other_753;
    let _e57: Point = self_897;
    let _e61: PointAndPlane = other_753;
    let _e64: PointAndPlane = other_753;
    let _e67: PointAndPlane = other_753;
    let _e70: PointAndPlane = other_753;
    let _e85: Point = self_897;
    let _e89: PointAndPlane = other_753;
    let _e92: PointAndPlane = other_753;
    let _e95: PointAndPlane = other_753;
    let _e98: PointAndPlane = other_753;
    let _e113: Point = self_897;
    let _e117: PointAndPlane = other_753;
    let _e120: PointAndPlane = other_753;
    let _e123: PointAndPlane = other_753;
    let _e126: PointAndPlane = other_753;
    let _e139: Point = self_897;
    let _e143: PointAndPlane = other_753;
    let _e146: PointAndPlane = other_753;
    let _e149: PointAndPlane = other_753;
    let _e152: PointAndPlane = other_753;
    let _e166: Point = self_897;
    let _e170: PointAndPlane = other_753;
    let _e173: PointAndPlane = other_753;
    let _e176: PointAndPlane = other_753;
    let _e179: PointAndPlane = other_753;
    return Motor((((((vec4<f32>(_e4.g0_.x) * vec4<f32>(_e8.g1_.x, _e11.g0_.y, _e14.g0_.z, _e17.g0_.w)) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0)) + ((vec4<f32>(_e29.g0_.y) * vec4<f32>(_e33.g1_.y, _e36.g0_.x, _e39.g1_.w, _e42.g1_.z)) * vec4<f32>(-(1.0), -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e57.g0_.z) * vec4<f32>(_e61.g1_.z, _e64.g1_.w, _e67.g0_.x, _e70.g1_.y)) * vec4<f32>(-(1.0), 1.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e85.g0_.w) * vec4<f32>(_e89.g1_.w, _e92.g1_.z, _e95.g1_.y, _e98.g0_.x)) * vec4<f32>(-(1.0), -(1.0), 1.0, -(1.0)))), ((((vec4<f32>(_e113.g0_.y) * vec4<f32>(_e117.g0_.y, _e120.g1_.x, _e123.g0_.w, _e126.g0_.z)) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0)) + ((vec4<f32>(_e139.g0_.z) * vec4<f32>(_e143.g0_.z, _e146.g0_.w, _e149.g1_.x, _e152.g0_.y)) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e166.g0_.w) * vec4<f32>(_e170.g0_.w, _e173.g0_.z, _e176.g0_.y, _e179.g1_.x)) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))));
}

fn point_point_and_plane_left_contraction(self_898: Point, other_754: PointAndPlane) -> Scalar {
    var self_899: Point;
    var other_755: PointAndPlane;

    self_899 = self_898;
    other_755 = other_754;
    let _e5: Point = self_899;
    let _e8: PointAndPlane = other_755;
    return Scalar((0.0 - (_e5.g0_.x * _e8.g0_.x)));
}

fn point_point_and_plane_right_anti_contraction(self_900: Point, other_756: PointAndPlane) -> AntiScalar {
    var self_901: Point;
    var other_757: PointAndPlane;

    self_901 = self_900;
    other_757 = other_756;
    let _e4: Point = self_901;
    let _e7: PointAndPlane = other_757;
    let _e11: Point = self_901;
    let _e14: PointAndPlane = other_757;
    let _e19: Point = self_901;
    let _e22: PointAndPlane = other_757;
    return AntiScalar((((_e4.g0_.y * _e7.g0_.y) + (_e11.g0_.z * _e14.g0_.z)) + (_e19.g0_.w * _e22.g0_.w)));
}

fn point_point_and_plane_scalar_product(self_902: Point, other_758: PointAndPlane) -> Scalar {
    var self_903: Point;
    var other_759: PointAndPlane;

    self_903 = self_902;
    other_759 = other_758;
    let _e5: Point = self_903;
    let _e8: PointAndPlane = other_759;
    return Scalar((0.0 - (_e5.g0_.x * _e8.g0_.x)));
}

fn point_point_and_plane_anti_scalar_product(self_904: Point, other_760: PointAndPlane) -> AntiScalar {
    var self_905: Point;
    var other_761: PointAndPlane;

    self_905 = self_904;
    other_761 = other_760;
    let _e4: Point = self_905;
    let _e7: PointAndPlane = other_761;
    let _e11: Point = self_905;
    let _e14: PointAndPlane = other_761;
    let _e19: Point = self_905;
    let _e22: PointAndPlane = other_761;
    return AntiScalar((((_e4.g0_.y * _e7.g0_.y) + (_e11.g0_.z * _e14.g0_.z)) + (_e19.g0_.w * _e22.g0_.w)));
}

fn point_squared_magnitude(self_906: Point) -> Scalar {
    var self_907: Point;

    self_907 = self_906;
    let _e2: Point = self_907;
    let _e3: Point = self_907;
    let _e4: Point = point_reversal(_e3);
    let _e5: Scalar = point_point_scalar_product(_e2, _e4);
    return _e5;
}

fn point_magnitude(self_908: Point) -> Scalar {
    var self_909: Point;

    self_909 = self_908;
    let _e2: Point = self_909;
    let _e3: Scalar = point_squared_magnitude(_e2);
    return Scalar(sqrt(_e3.g0_));
}

fn point_bulk_norm(self_910: Point) -> Scalar {
    var self_911: Point;

    self_911 = self_910;
    let _e2: Point = self_911;
    let _e3: Scalar = point_squared_magnitude(_e2);
    return Scalar(sqrt(_e3.g0_));
}

fn point_squared_anti_magnitude(self_912: Point) -> AntiScalar {
    var self_913: Point;

    self_913 = self_912;
    let _e2: Point = self_913;
    let _e3: Point = self_913;
    let _e4: Point = point_anti_reversal(_e3);
    let _e5: AntiScalar = point_point_anti_scalar_product(_e2, _e4);
    return _e5;
}

fn point_weight_norm(self_914: Point) -> AntiScalar {
    var self_915: Point;

    self_915 = self_914;
    let _e2: Point = self_915;
    let _e3: AntiScalar = point_squared_anti_magnitude(_e2);
    return AntiScalar(sqrt(_e3.g0_));
}

fn point_scale(self_916: Point, other_762: f32) -> Point {
    var self_917: Point;
    var other_763: f32;

    self_917 = self_916;
    other_763 = other_762;
    let _e4: Point = self_917;
    let _e5: f32 = other_763;
    let _e7: Point = point_scalar_geometric_product(_e4, Scalar(_e5));
    return _e7;
}

fn point_signum(self_918: Point) -> Point {
    var self_919: Point;

    self_919 = self_918;
    let _e2: Point = self_919;
    let _e3: Point = self_919;
    let _e4: Scalar = point_magnitude(_e3);
    let _e9: Point = point_scalar_geometric_product(_e2, Scalar((1.0 / _e4.g0_)));
    return _e9;
}

fn point_inverse(self_920: Point) -> Point {
    var self_921: Point;

    self_921 = self_920;
    let _e2: Point = self_921;
    let _e3: Point = point_reversal(_e2);
    let _e4: Point = self_921;
    let _e5: Scalar = point_squared_magnitude(_e4);
    let _e10: Point = point_scalar_geometric_product(_e3, Scalar((1.0 / _e5.g0_)));
    return _e10;
}

fn point_unitize(self_922: Point) -> Point {
    var self_923: Point;

    self_923 = self_922;
    let _e2: Point = self_923;
    let _e3: Point = self_923;
    let _e4: AntiScalar = point_weight_norm(_e3);
    let _e9: Point = point_scalar_geometric_product(_e2, Scalar((1.0 / _e4.g0_)));
    return _e9;
}

fn ideal_point_zero() -> IdealPoint {
    return IdealPoint(vec3<f32>(0.0));
}

fn ideal_point_one() -> IdealPoint {
    return IdealPoint(vec3<f32>(0.0));
}

fn ideal_point_grade(self_924: IdealPoint) -> i32 {
    return 2;
}

fn ideal_point_anti_grade(self_925: IdealPoint) -> i32 {
    return 2;
}

fn ideal_point_neg(self_926: IdealPoint) -> IdealPoint {
    var self_927: IdealPoint;

    self_927 = self_926;
    let _e2: IdealPoint = self_927;
    return IdealPoint((_e2.g0_ * vec3<f32>(-(1.0))));
}

fn ideal_point_automorphism(self_928: IdealPoint) -> IdealPoint {
    var self_929: IdealPoint;

    self_929 = self_928;
    let _e2: IdealPoint = self_929;
    return IdealPoint(_e2.g0_);
}

fn ideal_point_reversal(self_930: IdealPoint) -> IdealPoint {
    var self_931: IdealPoint;

    self_931 = self_930;
    let _e2: IdealPoint = self_931;
    return IdealPoint((_e2.g0_ * vec3<f32>(-(1.0))));
}

fn ideal_point_conjugation(self_932: IdealPoint) -> IdealPoint {
    var self_933: IdealPoint;

    self_933 = self_932;
    let _e2: IdealPoint = self_933;
    return IdealPoint((_e2.g0_ * vec3<f32>(-(1.0))));
}

fn ideal_point_anti_reversal(self_934: IdealPoint) -> IdealPoint {
    var self_935: IdealPoint;

    self_935 = self_934;
    let _e2: IdealPoint = self_935;
    return IdealPoint((_e2.g0_ * vec3<f32>(-(1.0))));
}

fn ideal_point_double_complement(self_936: IdealPoint) -> IdealPoint {
    var self_937: IdealPoint;

    self_937 = self_936;
    let _e2: IdealPoint = self_937;
    return IdealPoint(_e2.g0_);
}

fn ideal_point_scalar_add(self_938: IdealPoint, other_764: Scalar) -> Translator {
    var self_939: IdealPoint;
    var other_765: Scalar;

    self_939 = self_938;
    other_765 = other_764;
    let _e4: IdealPoint = self_939;
    let _e7: IdealPoint = self_939;
    let _e10: IdealPoint = self_939;
    let _e13: IdealPoint = self_939;
    let _e23: Scalar = other_765;
    return Translator(((vec4<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.y, _e13.g0_.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0)) + (vec4<f32>(_e23.g0_) * vec4<f32>(1.0, 0.0, 0.0, 0.0))));
}

fn ideal_point_scalar_sub(self_940: IdealPoint, other_766: Scalar) -> Translator {
    var self_941: IdealPoint;
    var other_767: Scalar;

    self_941 = self_940;
    other_767 = other_766;
    let _e4: IdealPoint = self_941;
    let _e7: IdealPoint = self_941;
    let _e10: IdealPoint = self_941;
    let _e13: IdealPoint = self_941;
    let _e23: Scalar = other_767;
    return Translator(((vec4<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.y, _e13.g0_.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0)) - (vec4<f32>(_e23.g0_) * vec4<f32>(1.0, 0.0, 0.0, 0.0))));
}

fn ideal_point_scalar_geometric_product(self_942: IdealPoint, other_768: Scalar) -> IdealPoint {
    var self_943: IdealPoint;
    var other_769: Scalar;

    self_943 = self_942;
    other_769 = other_768;
    let _e4: IdealPoint = self_943;
    let _e6: Scalar = other_769;
    return IdealPoint((_e4.g0_ * vec3<f32>(_e6.g0_)));
}

fn ideal_point_scalar_outer_product(self_944: IdealPoint, other_770: Scalar) -> IdealPoint {
    var self_945: IdealPoint;
    var other_771: Scalar;

    self_945 = self_944;
    other_771 = other_770;
    let _e4: IdealPoint = self_945;
    let _e6: Scalar = other_771;
    return IdealPoint((_e4.g0_ * vec3<f32>(_e6.g0_)));
}

fn ideal_point_scalar_inner_product(self_946: IdealPoint, other_772: Scalar) -> IdealPoint {
    var self_947: IdealPoint;
    var other_773: Scalar;

    self_947 = self_946;
    other_773 = other_772;
    let _e4: IdealPoint = self_947;
    let _e6: Scalar = other_773;
    return IdealPoint((_e4.g0_ * vec3<f32>(_e6.g0_)));
}

fn ideal_point_scalar_right_contraction(self_948: IdealPoint, other_774: Scalar) -> IdealPoint {
    var self_949: IdealPoint;
    var other_775: Scalar;

    self_949 = self_948;
    other_775 = other_774;
    let _e4: IdealPoint = self_949;
    let _e6: Scalar = other_775;
    return IdealPoint((_e4.g0_ * vec3<f32>(_e6.g0_)));
}

fn ideal_point_anti_scalar_regressive_product(self_950: IdealPoint, other_776: AntiScalar) -> IdealPoint {
    var self_951: IdealPoint;
    var other_777: AntiScalar;

    self_951 = self_950;
    other_777 = other_776;
    let _e4: IdealPoint = self_951;
    let _e6: AntiScalar = other_777;
    return IdealPoint((_e4.g0_ * vec3<f32>(_e6.g0_)));
}

fn ideal_point_anti_scalar_geometric_anti_product(self_952: IdealPoint, other_778: AntiScalar) -> IdealPoint {
    var self_953: IdealPoint;
    var other_779: AntiScalar;

    self_953 = self_952;
    other_779 = other_778;
    let _e4: IdealPoint = self_953;
    let _e6: AntiScalar = other_779;
    return IdealPoint((_e4.g0_ * vec3<f32>(_e6.g0_)));
}

fn ideal_point_anti_scalar_inner_anti_product(self_954: IdealPoint, other_780: AntiScalar) -> IdealPoint {
    var self_955: IdealPoint;
    var other_781: AntiScalar;

    self_955 = self_954;
    other_781 = other_780;
    let _e4: IdealPoint = self_955;
    let _e6: AntiScalar = other_781;
    return IdealPoint((_e4.g0_ * vec3<f32>(_e6.g0_)));
}

fn ideal_point_anti_scalar_right_anti_contraction(self_956: IdealPoint, other_782: AntiScalar) -> IdealPoint {
    var self_957: IdealPoint;
    var other_783: AntiScalar;

    self_957 = self_956;
    other_783 = other_782;
    let _e4: IdealPoint = self_957;
    let _e6: AntiScalar = other_783;
    return IdealPoint((_e4.g0_ * vec3<f32>(_e6.g0_)));
}

fn ideal_point_multi_vector_add(self_958: IdealPoint, other_784: MultiVector) -> MultiVector {
    var self_959: IdealPoint;
    var other_785: MultiVector;

    self_959 = self_958;
    other_785 = other_784;
    let _e4: MultiVector = other_785;
    let _e6: MultiVector = other_785;
    let _e8: MultiVector = other_785;
    let _e10: IdealPoint = self_959;
    let _e13: IdealPoint = self_959;
    let _e16: IdealPoint = self_959;
    let _e19: IdealPoint = self_959;
    let _e29: MultiVector = other_785;
    return MultiVector(_e4.g0_, _e6.g1_, _e8.g2_, ((vec4<f32>(_e10.g0_.x, _e13.g0_.x, _e16.g0_.y, _e19.g0_.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0)) + _e29.g3_));
}

fn ideal_point_multi_vector_sub(self_960: IdealPoint, other_786: MultiVector) -> MultiVector {
    var self_961: IdealPoint;
    var other_787: MultiVector;

    self_961 = self_960;
    other_787 = other_786;
    let _e6: MultiVector = other_787;
    let _e11: MultiVector = other_787;
    let _e16: MultiVector = other_787;
    let _e19: IdealPoint = self_961;
    let _e22: IdealPoint = self_961;
    let _e25: IdealPoint = self_961;
    let _e28: IdealPoint = self_961;
    let _e38: MultiVector = other_787;
    return MultiVector((vec4<f32>(0.0) - _e6.g0_), (vec4<f32>(0.0) - _e11.g1_), (vec4<f32>(0.0) - _e16.g2_), ((vec4<f32>(_e19.g0_.x, _e22.g0_.x, _e25.g0_.y, _e28.g0_.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0)) - _e38.g3_));
}

fn ideal_point_multi_vector_geometric_anti_product(self_962: IdealPoint, other_788: MultiVector) -> MultiVector {
    var self_963: IdealPoint;
    var other_789: MultiVector;

    self_963 = self_962;
    other_789 = other_788;
    let _e4: IdealPoint = self_963;
    let _e8: MultiVector = other_789;
    let _e20: IdealPoint = self_963;
    let _e24: MultiVector = other_789;
    let _e37: IdealPoint = self_963;
    let _e41: MultiVector = other_789;
    let _e54: IdealPoint = self_963;
    let _e58: MultiVector = other_789;
    let _e70: IdealPoint = self_963;
    let _e74: MultiVector = other_789;
    let _e87: IdealPoint = self_963;
    let _e91: MultiVector = other_789;
    let _e104: IdealPoint = self_963;
    let _e108: MultiVector = other_789;
    let _e120: IdealPoint = self_963;
    let _e124: MultiVector = other_789;
    let _e137: IdealPoint = self_963;
    let _e141: MultiVector = other_789;
    let _e154: IdealPoint = self_963;
    let _e158: MultiVector = other_789;
    let _e170: IdealPoint = self_963;
    let _e174: MultiVector = other_789;
    let _e187: IdealPoint = self_963;
    let _e191: MultiVector = other_789;
    return MultiVector(((((vec4<f32>(_e4.g0_.x) * _e8.g0_.yxwz) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0))) + ((vec4<f32>(_e20.g0_.y) * _e24.g0_.zwxy) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e37.g0_.z) * _e41.g0_.wzyx) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))), ((((vec4<f32>(_e54.g0_.x) * _e58.g1_.yxwz) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0))) + ((vec4<f32>(_e70.g0_.y) * _e74.g1_.zwxy) * vec4<f32>(-(1.0), -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e87.g0_.z) * _e91.g1_.wzyx) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))), ((((vec4<f32>(_e104.g0_.x) * _e108.g2_.yxwz) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0))) + ((vec4<f32>(_e120.g0_.y) * _e124.g2_.zwxy) * vec4<f32>(-(1.0), -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e137.g0_.z) * _e141.g2_.wzyx) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))), ((((vec4<f32>(_e154.g0_.x) * _e158.g3_.yxwz) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0))) + ((vec4<f32>(_e170.g0_.y) * _e174.g3_.zwxy) * vec4<f32>(-(1.0), -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e187.g0_.z) * _e191.g3_.wzyx) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))));
}

fn ideal_point_multi_vector_anti_scalar_product(self_964: IdealPoint, other_790: MultiVector) -> AntiScalar {
    var self_965: IdealPoint;
    var other_791: MultiVector;

    self_965 = self_964;
    other_791 = other_790;
    let _e5: IdealPoint = self_965;
    let _e8: MultiVector = other_791;
    let _e13: IdealPoint = self_965;
    let _e16: MultiVector = other_791;
    let _e21: IdealPoint = self_965;
    let _e24: MultiVector = other_791;
    return AntiScalar((((0.0 - (_e5.g0_.x * _e8.g3_.y)) - (_e13.g0_.y * _e16.g3_.z)) - (_e21.g0_.z * _e24.g3_.w)));
}

fn ideal_point_rotor_regressive_product(self_966: IdealPoint, other_792: Rotor) -> Scalar {
    var self_967: IdealPoint;
    var other_793: Rotor;

    self_967 = self_966;
    other_793 = other_792;
    let _e4: IdealPoint = self_967;
    let _e7: Rotor = other_793;
    let _e11: IdealPoint = self_967;
    let _e14: Rotor = other_793;
    let _e19: IdealPoint = self_967;
    let _e22: Rotor = other_793;
    return Scalar((((_e4.g0_.x * _e7.g0_.y) + (_e11.g0_.y * _e14.g0_.z)) + (_e19.g0_.z * _e22.g0_.w)));
}

fn ideal_point_rotor_inner_product(self_968: IdealPoint, other_794: Rotor) -> IdealPoint {
    var self_969: IdealPoint;
    var other_795: Rotor;

    self_969 = self_968;
    other_795 = other_794;
    let _e4: IdealPoint = self_969;
    let _e6: Rotor = other_795;
    return IdealPoint((_e4.g0_ * vec3<f32>(_e6.g0_.x)));
}

fn ideal_point_rotor_geometric_anti_product(self_970: IdealPoint, other_796: Rotor) -> Rotor {
    var self_971: IdealPoint;
    var other_797: Rotor;

    self_971 = self_970;
    other_797 = other_796;
    let _e4: IdealPoint = self_971;
    let _e8: Rotor = other_797;
    let _e20: IdealPoint = self_971;
    let _e24: Rotor = other_797;
    let _e37: IdealPoint = self_971;
    let _e41: Rotor = other_797;
    return Rotor(((((vec4<f32>(_e4.g0_.x) * _e8.g0_.yxwz) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0))) + ((vec4<f32>(_e20.g0_.y) * _e24.g0_.zwxy) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e37.g0_.z) * _e41.g0_.wzyx) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))));
}

fn ideal_point_rotor_right_contraction(self_972: IdealPoint, other_798: Rotor) -> IdealPoint {
    var self_973: IdealPoint;
    var other_799: Rotor;

    self_973 = self_972;
    other_799 = other_798;
    let _e4: IdealPoint = self_973;
    let _e6: Rotor = other_799;
    return IdealPoint((_e4.g0_ * vec3<f32>(_e6.g0_.x)));
}

fn ideal_point_point_regressive_product(self_974: IdealPoint, other_800: Point) -> Plane {
    var self_975: IdealPoint;
    var other_801: Point;

    self_975 = self_974;
    other_801 = other_800;
    let _e4: IdealPoint = self_975;
    let _e8: Point = other_801;
    let _e19: IdealPoint = self_975;
    let _e23: Point = other_801;
    let _e35: IdealPoint = self_975;
    let _e39: Point = other_801;
    return Plane(((((vec4<f32>(_e4.g0_.y) * _e8.g0_.zzxz) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0)) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.wwwx) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))) + ((vec4<f32>(_e35.g0_.x) * _e39.g0_.yxxx) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))));
}

fn ideal_point_ideal_point_add(self_976: IdealPoint, other_802: IdealPoint) -> IdealPoint {
    var self_977: IdealPoint;
    var other_803: IdealPoint;

    self_977 = self_976;
    other_803 = other_802;
    let _e4: IdealPoint = self_977;
    let _e6: IdealPoint = other_803;
    return IdealPoint((_e4.g0_ + _e6.g0_));
}

fn ideal_point_ideal_point_sub(self_978: IdealPoint, other_804: IdealPoint) -> IdealPoint {
    var self_979: IdealPoint;
    var other_805: IdealPoint;

    self_979 = self_978;
    other_805 = other_804;
    let _e4: IdealPoint = self_979;
    let _e6: IdealPoint = other_805;
    return IdealPoint((_e4.g0_ - _e6.g0_));
}

fn ideal_point_ideal_point_mul(self_980: IdealPoint, other_806: IdealPoint) -> IdealPoint {
    var self_981: IdealPoint;
    var other_807: IdealPoint;

    self_981 = self_980;
    other_807 = other_806;
    let _e4: IdealPoint = self_981;
    let _e6: IdealPoint = other_807;
    return IdealPoint((_e4.g0_ * _e6.g0_));
}

fn ideal_point_ideal_point_div(self_982: IdealPoint, other_808: IdealPoint) -> IdealPoint {
    var self_983: IdealPoint;
    var other_809: IdealPoint;

    self_983 = self_982;
    other_809 = other_808;
    let _e4: IdealPoint = self_983;
    let _e7: IdealPoint = self_983;
    let _e10: IdealPoint = self_983;
    let _e19: IdealPoint = other_809;
    let _e22: IdealPoint = other_809;
    let _e25: IdealPoint = other_809;
    return IdealPoint((((vec3<f32>(_e4.g0_.x, _e7.g0_.y, _e10.g0_.z) * vec3<f32>(1.0, 1.0, 1.0)) / vec3<f32>(_e19.g0_.x, _e22.g0_.y, _e25.g0_.z)) * vec3<f32>(1.0, 1.0, 1.0)));
}

fn ideal_point_ideal_point_inner_anti_product(self_984: IdealPoint, other_810: IdealPoint) -> AntiScalar {
    var self_985: IdealPoint;
    var other_811: IdealPoint;

    self_985 = self_984;
    other_811 = other_810;
    let _e5: IdealPoint = self_985;
    let _e8: IdealPoint = other_811;
    let _e13: IdealPoint = self_985;
    let _e16: IdealPoint = other_811;
    let _e21: IdealPoint = self_985;
    let _e24: IdealPoint = other_811;
    return AntiScalar((((0.0 - (_e5.g0_.x * _e8.g0_.x)) - (_e13.g0_.y * _e16.g0_.y)) - (_e21.g0_.z * _e24.g0_.z)));
}

fn ideal_point_ideal_point_left_anti_contraction(self_986: IdealPoint, other_812: IdealPoint) -> AntiScalar {
    var self_987: IdealPoint;
    var other_813: IdealPoint;

    self_987 = self_986;
    other_813 = other_812;
    let _e5: IdealPoint = self_987;
    let _e8: IdealPoint = other_813;
    let _e13: IdealPoint = self_987;
    let _e16: IdealPoint = other_813;
    let _e21: IdealPoint = self_987;
    let _e24: IdealPoint = other_813;
    return AntiScalar((((0.0 - (_e5.g0_.x * _e8.g0_.x)) - (_e13.g0_.y * _e16.g0_.y)) - (_e21.g0_.z * _e24.g0_.z)));
}

fn ideal_point_ideal_point_right_anti_contraction(self_988: IdealPoint, other_814: IdealPoint) -> AntiScalar {
    var self_989: IdealPoint;
    var other_815: IdealPoint;

    self_989 = self_988;
    other_815 = other_814;
    let _e5: IdealPoint = self_989;
    let _e8: IdealPoint = other_815;
    let _e13: IdealPoint = self_989;
    let _e16: IdealPoint = other_815;
    let _e21: IdealPoint = self_989;
    let _e24: IdealPoint = other_815;
    return AntiScalar((((0.0 - (_e5.g0_.x * _e8.g0_.x)) - (_e13.g0_.y * _e16.g0_.y)) - (_e21.g0_.z * _e24.g0_.z)));
}

fn ideal_point_ideal_point_anti_scalar_product(self_990: IdealPoint, other_816: IdealPoint) -> AntiScalar {
    var self_991: IdealPoint;
    var other_817: IdealPoint;

    self_991 = self_990;
    other_817 = other_816;
    let _e5: IdealPoint = self_991;
    let _e8: IdealPoint = other_817;
    let _e13: IdealPoint = self_991;
    let _e16: IdealPoint = other_817;
    let _e21: IdealPoint = self_991;
    let _e24: IdealPoint = other_817;
    return AntiScalar((((0.0 - (_e5.g0_.x * _e8.g0_.x)) - (_e13.g0_.y * _e16.g0_.y)) - (_e21.g0_.z * _e24.g0_.z)));
}

fn ideal_point_plane_inner_anti_product(self_992: IdealPoint, other_818: Plane) -> Point {
    var self_993: IdealPoint;
    var other_819: Plane;

    self_993 = self_992;
    other_819 = other_818;
    let _e4: IdealPoint = self_993;
    let _e8: Plane = other_819;
    let _e19: IdealPoint = self_993;
    let _e23: Plane = other_819;
    let _e35: IdealPoint = self_993;
    let _e39: Plane = other_819;
    return Point(((((vec4<f32>(_e4.g0_.y) * _e8.g0_.zzxz) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0)) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.wwwx) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))) + ((vec4<f32>(_e35.g0_.x) * _e39.g0_.yxxx) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))));
}

fn ideal_point_plane_left_anti_contraction(self_994: IdealPoint, other_820: Plane) -> Point {
    var self_995: IdealPoint;
    var other_821: Plane;

    self_995 = self_994;
    other_821 = other_820;
    let _e4: IdealPoint = self_995;
    let _e8: Plane = other_821;
    let _e19: IdealPoint = self_995;
    let _e23: Plane = other_821;
    let _e35: IdealPoint = self_995;
    let _e39: Plane = other_821;
    return Point(((((vec4<f32>(_e4.g0_.y) * _e8.g0_.zzxz) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0)) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.wwwx) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))) + ((vec4<f32>(_e35.g0_.x) * _e39.g0_.yxxx) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))));
}

fn ideal_point_line_add(self_996: IdealPoint, other_822: Line) -> Line {
    var self_997: IdealPoint;
    var other_823: Line;

    self_997 = self_996;
    other_823 = other_822;
    let _e4: IdealPoint = self_997;
    let _e6: Line = other_823;
    let _e9: Line = other_823;
    return Line((_e4.g0_ + _e6.g0_), _e9.g1_);
}

fn ideal_point_line_sub(self_998: IdealPoint, other_824: Line) -> Line {
    var self_999: IdealPoint;
    var other_825: Line;

    self_999 = self_998;
    other_825 = other_824;
    let _e4: IdealPoint = self_999;
    let _e6: Line = other_825;
    let _e11: Line = other_825;
    return Line((_e4.g0_ - _e6.g0_), (vec3<f32>(0.0) - _e11.g1_));
}

fn ideal_point_line_regressive_product(self_1000: IdealPoint, other_826: Line) -> Scalar {
    var self_1001: IdealPoint;
    var other_827: Line;

    self_1001 = self_1000;
    other_827 = other_826;
    let _e4: IdealPoint = self_1001;
    let _e7: Line = other_827;
    let _e11: IdealPoint = self_1001;
    let _e14: Line = other_827;
    let _e19: IdealPoint = self_1001;
    let _e22: Line = other_827;
    return Scalar((((_e4.g0_.x * _e7.g1_.x) + (_e11.g0_.y * _e14.g1_.y)) + (_e19.g0_.z * _e22.g1_.z)));
}

fn ideal_point_line_outer_product(self_1002: IdealPoint, other_828: Line) -> AntiScalar {
    var self_1003: IdealPoint;
    var other_829: Line;

    self_1003 = self_1002;
    other_829 = other_828;
    let _e4: IdealPoint = self_1003;
    let _e7: Line = other_829;
    let _e11: IdealPoint = self_1003;
    let _e14: Line = other_829;
    let _e19: IdealPoint = self_1003;
    let _e22: Line = other_829;
    return AntiScalar((((_e4.g0_.x * _e7.g1_.x) + (_e11.g0_.y * _e14.g1_.y)) + (_e19.g0_.z * _e22.g1_.z)));
}

fn ideal_point_line_geometric_anti_product(self_1004: IdealPoint, other_830: Line) -> Motor {
    var self_1005: IdealPoint;
    var other_831: Line;

    self_1005 = self_1004;
    other_831 = other_830;
    let _e4: IdealPoint = self_1005;
    let _e8: Line = other_831;
    let _e11: Line = other_831;
    let _e14: Line = other_831;
    let _e17: Line = other_831;
    let _e29: IdealPoint = self_1005;
    let _e33: Line = other_831;
    let _e36: Line = other_831;
    let _e39: Line = other_831;
    let _e42: Line = other_831;
    let _e55: IdealPoint = self_1005;
    let _e59: Line = other_831;
    let _e62: Line = other_831;
    let _e65: Line = other_831;
    let _e68: Line = other_831;
    let _e81: IdealPoint = self_1005;
    let _e85: Line = other_831;
    let _e88: Line = other_831;
    let _e91: Line = other_831;
    let _e94: Line = other_831;
    let _e107: IdealPoint = self_1005;
    let _e111: Line = other_831;
    let _e114: Line = other_831;
    let _e117: Line = other_831;
    let _e120: Line = other_831;
    let _e134: IdealPoint = self_1005;
    let _e138: Line = other_831;
    let _e141: Line = other_831;
    let _e144: Line = other_831;
    let _e147: Line = other_831;
    return Motor(((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g1_.y, _e11.g1_.z, _e14.g1_.y, _e17.g1_.x)) * vec4<f32>(1.0, -(1.0), 0.0, 1.0)) + ((vec4<f32>(_e29.g0_.z) * vec4<f32>(_e33.g1_.z, _e36.g1_.y, _e39.g1_.x, _e42.g1_.z)) * vec4<f32>(1.0, 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e55.g0_.x) * vec4<f32>(_e59.g1_.x, _e62.g1_.x, _e65.g1_.z, _e68.g1_.y)) * vec4<f32>(1.0, 0.0, 1.0, -(1.0)))), ((((vec4<f32>(_e81.g0_.y) * vec4<f32>(_e85.g0_.y, _e88.g0_.z, _e91.g0_.y, _e94.g0_.x)) * vec4<f32>(-(1.0), -(1.0), 0.0, 1.0)) + ((vec4<f32>(_e107.g0_.z) * vec4<f32>(_e111.g0_.z, _e114.g0_.y, _e117.g0_.x, _e120.g0_.z)) * vec4<f32>(-(1.0), 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e134.g0_.x) * vec4<f32>(_e138.g0_.x, _e141.g0_.x, _e144.g0_.z, _e147.g0_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0)))));
}

fn ideal_point_line_inner_anti_product(self_1006: IdealPoint, other_832: Line) -> AntiScalar {
    var self_1007: IdealPoint;
    var other_833: Line;

    self_1007 = self_1006;
    other_833 = other_832;
    let _e5: IdealPoint = self_1007;
    let _e8: Line = other_833;
    let _e13: IdealPoint = self_1007;
    let _e16: Line = other_833;
    let _e21: IdealPoint = self_1007;
    let _e24: Line = other_833;
    return AntiScalar((((0.0 - (_e5.g0_.x * _e8.g0_.x)) - (_e13.g0_.y * _e16.g0_.y)) - (_e21.g0_.z * _e24.g0_.z)));
}

fn ideal_point_line_left_anti_contraction(self_1008: IdealPoint, other_834: Line) -> AntiScalar {
    var self_1009: IdealPoint;
    var other_835: Line;

    self_1009 = self_1008;
    other_835 = other_834;
    let _e5: IdealPoint = self_1009;
    let _e8: Line = other_835;
    let _e13: IdealPoint = self_1009;
    let _e16: Line = other_835;
    let _e21: IdealPoint = self_1009;
    let _e24: Line = other_835;
    return AntiScalar((((0.0 - (_e5.g0_.x * _e8.g0_.x)) - (_e13.g0_.y * _e16.g0_.y)) - (_e21.g0_.z * _e24.g0_.z)));
}

fn ideal_point_line_right_anti_contraction(self_1010: IdealPoint, other_836: Line) -> AntiScalar {
    var self_1011: IdealPoint;
    var other_837: Line;

    self_1011 = self_1010;
    other_837 = other_836;
    let _e5: IdealPoint = self_1011;
    let _e8: Line = other_837;
    let _e13: IdealPoint = self_1011;
    let _e16: Line = other_837;
    let _e21: IdealPoint = self_1011;
    let _e24: Line = other_837;
    return AntiScalar((((0.0 - (_e5.g0_.x * _e8.g0_.x)) - (_e13.g0_.y * _e16.g0_.y)) - (_e21.g0_.z * _e24.g0_.z)));
}

fn ideal_point_line_anti_scalar_product(self_1012: IdealPoint, other_838: Line) -> AntiScalar {
    var self_1013: IdealPoint;
    var other_839: Line;

    self_1013 = self_1012;
    other_839 = other_838;
    let _e5: IdealPoint = self_1013;
    let _e8: Line = other_839;
    let _e13: IdealPoint = self_1013;
    let _e16: Line = other_839;
    let _e21: IdealPoint = self_1013;
    let _e24: Line = other_839;
    return AntiScalar((((0.0 - (_e5.g0_.x * _e8.g0_.x)) - (_e13.g0_.y * _e16.g0_.y)) - (_e21.g0_.z * _e24.g0_.z)));
}

fn ideal_point_translator_add(self_1014: IdealPoint, other_840: Translator) -> Translator {
    var self_1015: IdealPoint;
    var other_841: Translator;

    self_1015 = self_1014;
    other_841 = other_840;
    let _e4: IdealPoint = self_1015;
    let _e7: IdealPoint = self_1015;
    let _e10: IdealPoint = self_1015;
    let _e13: IdealPoint = self_1015;
    let _e23: Translator = other_841;
    return Translator(((vec4<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.y, _e13.g0_.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0)) + _e23.g0_));
}

fn ideal_point_translator_sub(self_1016: IdealPoint, other_842: Translator) -> Translator {
    var self_1017: IdealPoint;
    var other_843: Translator;

    self_1017 = self_1016;
    other_843 = other_842;
    let _e4: IdealPoint = self_1017;
    let _e7: IdealPoint = self_1017;
    let _e10: IdealPoint = self_1017;
    let _e13: IdealPoint = self_1017;
    let _e23: Translator = other_843;
    return Translator(((vec4<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.y, _e13.g0_.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0)) - _e23.g0_));
}

fn ideal_point_translator_geometric_product(self_1018: IdealPoint, other_844: Translator) -> IdealPoint {
    var self_1019: IdealPoint;
    var other_845: Translator;

    self_1019 = self_1018;
    other_845 = other_844;
    let _e4: IdealPoint = self_1019;
    let _e6: Translator = other_845;
    return IdealPoint((_e4.g0_ * vec3<f32>(_e6.g0_.x)));
}

fn ideal_point_translator_outer_product(self_1020: IdealPoint, other_846: Translator) -> IdealPoint {
    var self_1021: IdealPoint;
    var other_847: Translator;

    self_1021 = self_1020;
    other_847 = other_846;
    let _e4: IdealPoint = self_1021;
    let _e6: Translator = other_847;
    return IdealPoint((_e4.g0_ * vec3<f32>(_e6.g0_.x)));
}

fn ideal_point_translator_inner_product(self_1022: IdealPoint, other_848: Translator) -> IdealPoint {
    var self_1023: IdealPoint;
    var other_849: Translator;

    self_1023 = self_1022;
    other_849 = other_848;
    let _e4: IdealPoint = self_1023;
    let _e6: Translator = other_849;
    return IdealPoint((_e4.g0_ * vec3<f32>(_e6.g0_.x)));
}

fn ideal_point_translator_right_contraction(self_1024: IdealPoint, other_850: Translator) -> IdealPoint {
    var self_1025: IdealPoint;
    var other_851: Translator;

    self_1025 = self_1024;
    other_851 = other_850;
    let _e4: IdealPoint = self_1025;
    let _e6: Translator = other_851;
    return IdealPoint((_e4.g0_ * vec3<f32>(_e6.g0_.x)));
}

fn ideal_point_translator_right_anti_contraction(self_1026: IdealPoint, other_852: Translator) -> AntiScalar {
    var self_1027: IdealPoint;
    var other_853: Translator;

    self_1027 = self_1026;
    other_853 = other_852;
    let _e5: IdealPoint = self_1027;
    let _e8: Translator = other_853;
    let _e13: IdealPoint = self_1027;
    let _e16: Translator = other_853;
    let _e21: IdealPoint = self_1027;
    let _e24: Translator = other_853;
    return AntiScalar((((0.0 - (_e5.g0_.x * _e8.g0_.y)) - (_e13.g0_.y * _e16.g0_.z)) - (_e21.g0_.z * _e24.g0_.w)));
}

fn ideal_point_translator_anti_scalar_product(self_1028: IdealPoint, other_854: Translator) -> AntiScalar {
    var self_1029: IdealPoint;
    var other_855: Translator;

    self_1029 = self_1028;
    other_855 = other_854;
    let _e5: IdealPoint = self_1029;
    let _e8: Translator = other_855;
    let _e13: IdealPoint = self_1029;
    let _e16: Translator = other_855;
    let _e21: IdealPoint = self_1029;
    let _e24: Translator = other_855;
    return AntiScalar((((0.0 - (_e5.g0_.x * _e8.g0_.y)) - (_e13.g0_.y * _e16.g0_.z)) - (_e21.g0_.z * _e24.g0_.w)));
}

fn ideal_point_motor_add(self_1030: IdealPoint, other_856: Motor) -> Motor {
    var self_1031: IdealPoint;
    var other_857: Motor;

    self_1031 = self_1030;
    other_857 = other_856;
    let _e4: Motor = other_857;
    let _e6: IdealPoint = self_1031;
    let _e9: IdealPoint = self_1031;
    let _e12: IdealPoint = self_1031;
    let _e15: IdealPoint = self_1031;
    let _e25: Motor = other_857;
    return Motor(_e4.g0_, ((vec4<f32>(_e6.g0_.x, _e9.g0_.x, _e12.g0_.y, _e15.g0_.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0)) + _e25.g1_));
}

fn ideal_point_motor_sub(self_1032: IdealPoint, other_858: Motor) -> Motor {
    var self_1033: IdealPoint;
    var other_859: Motor;

    self_1033 = self_1032;
    other_859 = other_858;
    let _e6: Motor = other_859;
    let _e9: IdealPoint = self_1033;
    let _e12: IdealPoint = self_1033;
    let _e15: IdealPoint = self_1033;
    let _e18: IdealPoint = self_1033;
    let _e28: Motor = other_859;
    return Motor((vec4<f32>(0.0) - _e6.g0_), ((vec4<f32>(_e9.g0_.x, _e12.g0_.x, _e15.g0_.y, _e18.g0_.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0)) - _e28.g1_));
}

fn ideal_point_motor_regressive_product(self_1034: IdealPoint, other_860: Motor) -> Translator {
    var self_1035: IdealPoint;
    var other_861: Motor;

    self_1035 = self_1034;
    other_861 = other_860;
    let _e4: IdealPoint = self_1035;
    let _e8: Motor = other_861;
    let _e11: Motor = other_861;
    let _e14: Motor = other_861;
    let _e17: Motor = other_861;
    let _e28: IdealPoint = self_1035;
    let _e32: Motor = other_861;
    let _e35: Motor = other_861;
    let _e38: Motor = other_861;
    let _e41: Motor = other_861;
    let _e53: IdealPoint = self_1035;
    let _e57: Motor = other_861;
    let _e60: Motor = other_861;
    let _e63: Motor = other_861;
    let _e66: Motor = other_861;
    return Translator(((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.z, _e11.g0_.z, _e14.g1_.x, _e17.g0_.z)) * vec4<f32>(1.0, 0.0, 1.0, 0.0)) + ((vec4<f32>(_e28.g0_.z) * vec4<f32>(_e32.g0_.w, _e35.g0_.w, _e38.g0_.w, _e41.g1_.x)) * vec4<f32>(1.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e53.g0_.x) * vec4<f32>(_e57.g0_.y, _e60.g1_.x, _e63.g0_.x, _e66.g0_.x)) * vec4<f32>(1.0, 1.0, 0.0, 0.0))));
}

fn ideal_point_motor_inner_product(self_1036: IdealPoint, other_862: Motor) -> IdealPoint {
    var self_1037: IdealPoint;
    var other_863: Motor;

    self_1037 = self_1036;
    other_863 = other_862;
    let _e4: IdealPoint = self_1037;
    let _e6: Motor = other_863;
    return IdealPoint((_e4.g0_ * vec3<f32>(_e6.g0_.x)));
}

fn ideal_point_motor_geometric_anti_product(self_1038: IdealPoint, other_864: Motor) -> Motor {
    var self_1039: IdealPoint;
    var other_865: Motor;

    self_1039 = self_1038;
    other_865 = other_864;
    let _e4: IdealPoint = self_1039;
    let _e8: Motor = other_865;
    let _e20: IdealPoint = self_1039;
    let _e24: Motor = other_865;
    let _e37: IdealPoint = self_1039;
    let _e41: Motor = other_865;
    let _e54: IdealPoint = self_1039;
    let _e58: Motor = other_865;
    let _e70: IdealPoint = self_1039;
    let _e74: Motor = other_865;
    let _e87: IdealPoint = self_1039;
    let _e91: Motor = other_865;
    return Motor(((((vec4<f32>(_e4.g0_.x) * _e8.g0_.yxwz) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0))) + ((vec4<f32>(_e20.g0_.y) * _e24.g0_.zwxy) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e37.g0_.z) * _e41.g0_.wzyx) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))), ((((vec4<f32>(_e54.g0_.x) * _e58.g1_.yxwz) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0))) + ((vec4<f32>(_e70.g0_.y) * _e74.g1_.zwxy) * vec4<f32>(-(1.0), -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e87.g0_.z) * _e91.g1_.wzyx) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))));
}

fn ideal_point_motor_right_contraction(self_1040: IdealPoint, other_866: Motor) -> IdealPoint {
    var self_1041: IdealPoint;
    var other_867: Motor;

    self_1041 = self_1040;
    other_867 = other_866;
    let _e4: IdealPoint = self_1041;
    let _e6: Motor = other_867;
    return IdealPoint((_e4.g0_ * vec3<f32>(_e6.g0_.x)));
}

fn ideal_point_motor_anti_scalar_product(self_1042: IdealPoint, other_868: Motor) -> AntiScalar {
    var self_1043: IdealPoint;
    var other_869: Motor;

    self_1043 = self_1042;
    other_869 = other_868;
    let _e5: IdealPoint = self_1043;
    let _e8: Motor = other_869;
    let _e13: IdealPoint = self_1043;
    let _e16: Motor = other_869;
    let _e21: IdealPoint = self_1043;
    let _e24: Motor = other_869;
    return AntiScalar((((0.0 - (_e5.g0_.x * _e8.g1_.y)) - (_e13.g0_.y * _e16.g1_.z)) - (_e21.g0_.z * _e24.g1_.w)));
}

fn ideal_point_point_and_plane_regressive_product(self_1044: IdealPoint, other_870: PointAndPlane) -> Plane {
    var self_1045: IdealPoint;
    var other_871: PointAndPlane;

    self_1045 = self_1044;
    other_871 = other_870;
    let _e4: IdealPoint = self_1045;
    let _e8: PointAndPlane = other_871;
    let _e19: IdealPoint = self_1045;
    let _e23: PointAndPlane = other_871;
    let _e35: IdealPoint = self_1045;
    let _e39: PointAndPlane = other_871;
    return Plane(((((vec4<f32>(_e4.g0_.y) * _e8.g0_.zzxz) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0)) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.wwwx) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))) + ((vec4<f32>(_e35.g0_.x) * _e39.g0_.yxxx) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))));
}

fn ideal_point_point_and_plane_geometric_anti_product(self_1046: IdealPoint, other_872: PointAndPlane) -> PointAndPlane {
    var self_1047: IdealPoint;
    var other_873: PointAndPlane;

    self_1047 = self_1046;
    other_873 = other_872;
    let _e4: IdealPoint = self_1047;
    let _e8: PointAndPlane = other_873;
    let _e11: PointAndPlane = other_873;
    let _e14: PointAndPlane = other_873;
    let _e17: PointAndPlane = other_873;
    let _e30: IdealPoint = self_1047;
    let _e34: PointAndPlane = other_873;
    let _e37: PointAndPlane = other_873;
    let _e40: PointAndPlane = other_873;
    let _e43: PointAndPlane = other_873;
    let _e57: IdealPoint = self_1047;
    let _e61: PointAndPlane = other_873;
    let _e64: PointAndPlane = other_873;
    let _e67: PointAndPlane = other_873;
    let _e70: PointAndPlane = other_873;
    let _e84: IdealPoint = self_1047;
    let _e88: PointAndPlane = other_873;
    let _e91: PointAndPlane = other_873;
    let _e94: PointAndPlane = other_873;
    let _e97: PointAndPlane = other_873;
    let _e110: IdealPoint = self_1047;
    let _e114: PointAndPlane = other_873;
    let _e117: PointAndPlane = other_873;
    let _e120: PointAndPlane = other_873;
    let _e123: PointAndPlane = other_873;
    let _e137: IdealPoint = self_1047;
    let _e141: PointAndPlane = other_873;
    let _e144: PointAndPlane = other_873;
    let _e147: PointAndPlane = other_873;
    let _e150: PointAndPlane = other_873;
    return PointAndPlane(((((vec4<f32>(_e4.g0_.x) * vec4<f32>(_e8.g1_.y, _e11.g1_.x, _e14.g0_.w, _e17.g0_.z)) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0))) + ((vec4<f32>(_e30.g0_.y) * vec4<f32>(_e34.g1_.z, _e37.g0_.w, _e40.g1_.x, _e43.g0_.y)) * vec4<f32>(-(1.0), -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e57.g0_.z) * vec4<f32>(_e61.g1_.w, _e64.g0_.z, _e67.g0_.y, _e70.g1_.x)) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))), ((((vec4<f32>(_e84.g0_.x) * vec4<f32>(_e88.g0_.y, _e91.g0_.x, _e94.g1_.w, _e97.g1_.z)) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0))) + ((vec4<f32>(_e110.g0_.y) * vec4<f32>(_e114.g0_.z, _e117.g1_.w, _e120.g0_.x, _e123.g1_.y)) * vec4<f32>(-(1.0), -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e137.g0_.z) * vec4<f32>(_e141.g0_.w, _e144.g1_.z, _e147.g1_.y, _e150.g0_.x)) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))));
}

fn ideal_point_point_and_plane_inner_anti_product(self_1048: IdealPoint, other_874: PointAndPlane) -> Point {
    var self_1049: IdealPoint;
    var other_875: PointAndPlane;

    self_1049 = self_1048;
    other_875 = other_874;
    let _e4: IdealPoint = self_1049;
    let _e8: PointAndPlane = other_875;
    let _e11: PointAndPlane = other_875;
    let _e14: PointAndPlane = other_875;
    let _e17: PointAndPlane = other_875;
    let _e30: IdealPoint = self_1049;
    let _e34: PointAndPlane = other_875;
    let _e37: PointAndPlane = other_875;
    let _e40: PointAndPlane = other_875;
    let _e43: PointAndPlane = other_875;
    let _e57: IdealPoint = self_1049;
    let _e61: PointAndPlane = other_875;
    let _e64: PointAndPlane = other_875;
    let _e67: PointAndPlane = other_875;
    let _e70: PointAndPlane = other_875;
    return Point(((((vec4<f32>(_e4.g0_.x) * vec4<f32>(_e8.g1_.y, _e11.g1_.x, _e14.g0_.w, _e17.g0_.z)) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0))) + ((vec4<f32>(_e30.g0_.y) * vec4<f32>(_e34.g1_.z, _e37.g0_.w, _e40.g1_.x, _e43.g0_.y)) * vec4<f32>(-(1.0), -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e57.g0_.z) * vec4<f32>(_e61.g1_.w, _e64.g0_.z, _e67.g0_.y, _e70.g1_.x)) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))));
}

fn ideal_point_point_and_plane_left_anti_contraction(self_1050: IdealPoint, other_876: PointAndPlane) -> Point {
    var self_1051: IdealPoint;
    var other_877: PointAndPlane;

    self_1051 = self_1050;
    other_877 = other_876;
    let _e4: IdealPoint = self_1051;
    let _e8: PointAndPlane = other_877;
    let _e19: IdealPoint = self_1051;
    let _e23: PointAndPlane = other_877;
    let _e35: IdealPoint = self_1051;
    let _e39: PointAndPlane = other_877;
    return Point(((((vec4<f32>(_e4.g0_.y) * _e8.g1_.zzxz) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0)) + ((vec4<f32>(_e19.g0_.z) * _e23.g1_.wwwx) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))) + ((vec4<f32>(_e35.g0_.x) * _e39.g1_.yxxx) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))));
}

fn ideal_point_scale(self_1052: IdealPoint, other_878: f32) -> IdealPoint {
    var self_1053: IdealPoint;
    var other_879: f32;

    self_1053 = self_1052;
    other_879 = other_878;
    let _e4: IdealPoint = self_1053;
    let _e5: f32 = other_879;
    let _e7: IdealPoint = ideal_point_scalar_geometric_product(_e4, Scalar(_e5));
    return _e7;
}

fn plane_zero() -> Plane {
    return Plane(vec4<f32>(0.0));
}

fn plane_one() -> Plane {
    return Plane(vec4<f32>(0.0));
}

fn plane_grade(self_1054: Plane) -> i32 {
    return 1;
}

fn plane_anti_grade(self_1055: Plane) -> i32 {
    return 3;
}

fn plane_neg(self_1056: Plane) -> Plane {
    var self_1057: Plane;

    self_1057 = self_1056;
    let _e2: Plane = self_1057;
    return Plane((_e2.g0_ * vec4<f32>(-(1.0))));
}

fn plane_automorphism(self_1058: Plane) -> Plane {
    var self_1059: Plane;

    self_1059 = self_1058;
    let _e2: Plane = self_1059;
    return Plane((_e2.g0_ * vec4<f32>(-(1.0))));
}

fn plane_reversal(self_1060: Plane) -> Plane {
    var self_1061: Plane;

    self_1061 = self_1060;
    let _e2: Plane = self_1061;
    return Plane(_e2.g0_);
}

fn plane_conjugation(self_1062: Plane) -> Plane {
    var self_1063: Plane;

    self_1063 = self_1062;
    let _e2: Plane = self_1063;
    return Plane((_e2.g0_ * vec4<f32>(-(1.0))));
}

fn plane_dual(self_1064: Plane) -> Point {
    var self_1065: Plane;

    self_1065 = self_1064;
    let _e2: Plane = self_1065;
    return Point(_e2.g0_);
}

fn plane_anti_reversal(self_1066: Plane) -> Plane {
    var self_1067: Plane;

    self_1067 = self_1066;
    let _e2: Plane = self_1067;
    return Plane((_e2.g0_ * vec4<f32>(-(1.0))));
}

fn plane_right_complement(self_1068: Plane) -> Point {
    var self_1069: Plane;

    self_1069 = self_1068;
    let _e2: Plane = self_1069;
    return Point(_e2.g0_);
}

fn plane_left_complement(self_1070: Plane) -> Point {
    var self_1071: Plane;

    self_1071 = self_1070;
    let _e2: Plane = self_1071;
    return Point((_e2.g0_ * vec4<f32>(-(1.0))));
}

fn plane_double_complement(self_1072: Plane) -> Plane {
    var self_1073: Plane;

    self_1073 = self_1072;
    let _e2: Plane = self_1073;
    return Plane((_e2.g0_ * vec4<f32>(-(1.0))));
}

fn plane_scalar_geometric_product(self_1074: Plane, other_880: Scalar) -> Plane {
    var self_1075: Plane;
    var other_881: Scalar;

    self_1075 = self_1074;
    other_881 = other_880;
    let _e4: Plane = self_1075;
    let _e6: Scalar = other_881;
    return Plane((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn plane_scalar_outer_product(self_1076: Plane, other_882: Scalar) -> Plane {
    var self_1077: Plane;
    var other_883: Scalar;

    self_1077 = self_1076;
    other_883 = other_882;
    let _e4: Plane = self_1077;
    let _e6: Scalar = other_883;
    return Plane((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn plane_scalar_inner_product(self_1078: Plane, other_884: Scalar) -> Plane {
    var self_1079: Plane;
    var other_885: Scalar;

    self_1079 = self_1078;
    other_885 = other_884;
    let _e4: Plane = self_1079;
    let _e6: Scalar = other_885;
    return Plane((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn plane_scalar_right_contraction(self_1080: Plane, other_886: Scalar) -> Plane {
    var self_1081: Plane;
    var other_887: Scalar;

    self_1081 = self_1080;
    other_887 = other_886;
    let _e4: Plane = self_1081;
    let _e6: Scalar = other_887;
    return Plane((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn plane_anti_scalar_regressive_product(self_1082: Plane, other_888: AntiScalar) -> Plane {
    var self_1083: Plane;
    var other_889: AntiScalar;

    self_1083 = self_1082;
    other_889 = other_888;
    let _e4: Plane = self_1083;
    let _e6: AntiScalar = other_889;
    return Plane((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn plane_anti_scalar_geometric_anti_product(self_1084: Plane, other_890: AntiScalar) -> Plane {
    var self_1085: Plane;
    var other_891: AntiScalar;

    self_1085 = self_1084;
    other_891 = other_890;
    let _e4: Plane = self_1085;
    let _e6: AntiScalar = other_891;
    return Plane((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn plane_anti_scalar_inner_anti_product(self_1086: Plane, other_892: AntiScalar) -> Plane {
    var self_1087: Plane;
    var other_893: AntiScalar;

    self_1087 = self_1086;
    other_893 = other_892;
    let _e4: Plane = self_1087;
    let _e6: AntiScalar = other_893;
    return Plane((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn plane_anti_scalar_right_anti_contraction(self_1088: Plane, other_894: AntiScalar) -> Plane {
    var self_1089: Plane;
    var other_895: AntiScalar;

    self_1089 = self_1088;
    other_895 = other_894;
    let _e4: Plane = self_1089;
    let _e6: AntiScalar = other_895;
    return Plane((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn plane_multi_vector_add(self_1090: Plane, other_896: MultiVector) -> MultiVector {
    var self_1091: Plane;
    var other_897: MultiVector;

    self_1091 = self_1090;
    other_897 = other_896;
    let _e4: MultiVector = other_897;
    let _e6: Plane = self_1091;
    let _e16: MultiVector = other_897;
    let _e19: Plane = self_1091;
    let _e27: MultiVector = other_897;
    let _e30: MultiVector = other_897;
    return MultiVector(_e4.g0_, ((vec4<f32>(_e6.g0_.x) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) + _e16.g1_), ((_e19.g0_ * vec4<f32>(0.0, 1.0, 1.0, 1.0)) + _e27.g2_), _e30.g3_);
}

fn plane_multi_vector_sub(self_1092: Plane, other_898: MultiVector) -> MultiVector {
    var self_1093: Plane;
    var other_899: MultiVector;

    self_1093 = self_1092;
    other_899 = other_898;
    let _e6: MultiVector = other_899;
    let _e9: Plane = self_1093;
    let _e19: MultiVector = other_899;
    let _e22: Plane = self_1093;
    let _e30: MultiVector = other_899;
    let _e35: MultiVector = other_899;
    return MultiVector((vec4<f32>(0.0) - _e6.g0_), ((vec4<f32>(_e9.g0_.x) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) - _e19.g1_), ((_e22.g0_ * vec4<f32>(0.0, 1.0, 1.0, 1.0)) - _e30.g2_), (vec4<f32>(0.0) - _e35.g3_));
}

fn plane_multi_vector_geometric_product(self_1094: Plane, other_900: MultiVector) -> MultiVector {
    var self_1095: Plane;
    var other_901: MultiVector;

    self_1095 = self_1094;
    other_901 = other_900;
    let _e4: Plane = self_1095;
    let _e8: MultiVector = other_901;
    let _e19: Plane = self_1095;
    let _e23: MultiVector = other_901;
    let _e35: Plane = self_1095;
    let _e39: MultiVector = other_901;
    let _e51: Plane = self_1095;
    let _e55: MultiVector = other_901;
    let _e67: Plane = self_1095;
    let _e71: MultiVector = other_901;
    let _e84: Plane = self_1095;
    let _e88: MultiVector = other_901;
    let _e101: Plane = self_1095;
    let _e105: MultiVector = other_901;
    let _e118: Plane = self_1095;
    let _e122: MultiVector = other_901;
    let _e133: Plane = self_1095;
    let _e137: MultiVector = other_901;
    let _e149: Plane = self_1095;
    let _e153: MultiVector = other_901;
    let _e165: Plane = self_1095;
    let _e169: MultiVector = other_901;
    let _e172: Plane = self_1095;
    let _e176: MultiVector = other_901;
    let _e189: Plane = self_1095;
    let _e193: MultiVector = other_901;
    let _e206: Plane = self_1095;
    let _e210: MultiVector = other_901;
    return MultiVector(((((vec4<f32>(_e4.g0_.y) * _e8.g2_.yxwz) * vec4<f32>(1.0, 1.0, -(1.0), 1.0)) + ((vec4<f32>(_e19.g0_.z) * _e23.g2_.zwxy) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e35.g0_.w) * _e39.g2_.wzyx) * vec4<f32>(1.0, -(1.0), 1.0, 1.0))), (((((vec4<f32>(_e51.g0_.x) * _e55.g0_) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))) + ((vec4<f32>(_e67.g0_.y) * _e71.g3_.yxwz) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))) + ((vec4<f32>(_e84.g0_.z) * _e88.g3_.zwxy) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e101.g0_.w) * _e105.g3_.wzyx) * vec4<f32>(-(1.0), -(1.0), 1.0, 1.0))), ((((vec4<f32>(_e118.g0_.y) * _e122.g0_.yxwz) * vec4<f32>(1.0, 1.0, 1.0, -(1.0))) + ((vec4<f32>(_e133.g0_.z) * _e137.g0_.zwxy) * vec4<f32>(1.0, -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e149.g0_.w) * _e153.g0_.wzyx) * vec4<f32>(1.0, 1.0, -(1.0), 1.0))), ((((vec4<f32>(_e165.g0_.x) * _e169.g2_) + ((vec4<f32>(_e172.g0_.y) * _e176.g1_.yxwz) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e189.g0_.z) * _e193.g1_.zwxy) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e206.g0_.w) * _e210.g1_.wzyx) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))));
}

fn plane_multi_vector_geometric_anti_product(self_1096: Plane, other_902: MultiVector) -> MultiVector {
    var self_1097: Plane;
    var other_903: MultiVector;

    self_1097 = self_1096;
    other_903 = other_902;
    let _e4: Plane = self_1097;
    let _e8: MultiVector = other_903;
    let _e20: Plane = self_1097;
    let _e24: MultiVector = other_903;
    let _e36: Plane = self_1097;
    let _e40: MultiVector = other_903;
    let _e52: Plane = self_1097;
    let _e56: MultiVector = other_903;
    let _e68: Plane = self_1097;
    let _e72: MultiVector = other_903;
    let _e75: Plane = self_1097;
    let _e79: MultiVector = other_903;
    let _e89: Plane = self_1097;
    let _e93: MultiVector = other_903;
    let _e106: Plane = self_1097;
    let _e110: MultiVector = other_903;
    let _e123: Plane = self_1097;
    let _e127: MultiVector = other_903;
    let _e142: Plane = self_1097;
    let _e146: MultiVector = other_903;
    return MultiVector((((((vec4<f32>(_e4.g0_.x) * _e8.g2_) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))) + ((vec4<f32>(_e20.g0_.y) * _e24.g1_.yxwz) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e36.g0_.z) * _e40.g1_.zwxy) * vec4<f32>(1.0, -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e52.g0_.w) * _e56.g1_.wzyx) * vec4<f32>(1.0, 1.0, -(1.0), 1.0))), (vec4<f32>(_e68.g0_.x) * _e72.g3_), (((((vec4<f32>(_e75.g0_.x) * _e79.g0_) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0)) + ((vec4<f32>(_e89.g0_.y) * _e93.g3_.yxwz) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e106.g0_.z) * _e110.g3_.zwxy) * vec4<f32>(-(1.0), -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e123.g0_.w) * _e127.g3_.wzyx) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))), (vec4<f32>(0.0) - (vec4<f32>(_e142.g0_.x) * _e146.g1_)));
}

fn plane_multi_vector_scalar_product(self_1098: Plane, other_904: MultiVector) -> Scalar {
    var self_1099: Plane;
    var other_905: MultiVector;

    self_1099 = self_1098;
    other_905 = other_904;
    let _e4: Plane = self_1099;
    let _e7: MultiVector = other_905;
    let _e11: Plane = self_1099;
    let _e14: MultiVector = other_905;
    let _e19: Plane = self_1099;
    let _e22: MultiVector = other_905;
    return Scalar((((_e4.g0_.y * _e7.g2_.y) + (_e11.g0_.z * _e14.g2_.z)) + (_e19.g0_.w * _e22.g2_.w)));
}

fn plane_multi_vector_anti_scalar_product(self_1100: Plane, other_906: MultiVector) -> AntiScalar {
    var self_1101: Plane;
    var other_907: MultiVector;

    self_1101 = self_1100;
    other_907 = other_906;
    let _e5: Plane = self_1101;
    let _e8: MultiVector = other_907;
    return AntiScalar((0.0 - (_e5.g0_.x * _e8.g1_.x)));
}

fn plane_rotor_geometric_product(self_1102: Plane, other_908: Rotor) -> PointAndPlane {
    var self_1103: Plane;
    var other_909: Rotor;

    self_1103 = self_1102;
    other_909 = other_908;
    let _e4: Plane = self_1103;
    let _e8: Rotor = other_909;
    let _e19: Plane = self_1103;
    let _e23: Rotor = other_909;
    let _e35: Plane = self_1103;
    let _e38: Rotor = other_909;
    let _e52: Plane = self_1103;
    let _e56: Rotor = other_909;
    let _e67: Plane = self_1103;
    let _e71: Rotor = other_909;
    let _e83: Plane = self_1103;
    let _e86: Rotor = other_909;
    return PointAndPlane(((((vec4<f32>(_e4.g0_.z) * vec4<f32>(_e8.g0_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) + ((vec4<f32>(_e19.g0_.w) * vec4<f32>(_e23.g0_.w)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((_e35.g0_.yxxx * _e38.g0_.yyzw) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0)))), ((((vec4<f32>(_e52.g0_.z) * _e56.g0_.wwxy) * vec4<f32>(0.0, -(1.0), 1.0, 1.0)) + ((vec4<f32>(_e67.g0_.w) * _e71.g0_.zzyx) * vec4<f32>(0.0, 1.0, -(1.0), 1.0))) + ((_e83.g0_.xyyy * _e86.g0_.xxwz) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))));
}

fn plane_rotor_outer_product(self_1104: Plane, other_910: Rotor) -> PointAndPlane {
    var self_1105: Plane;
    var other_911: Rotor;

    self_1105 = self_1104;
    other_911 = other_910;
    let _e4: Plane = self_1105;
    let _e8: Rotor = other_911;
    let _e19: Plane = self_1105;
    let _e23: Rotor = other_911;
    let _e35: Plane = self_1105;
    let _e38: Rotor = other_911;
    let _e52: Plane = self_1105;
    let _e54: Rotor = other_911;
    return PointAndPlane(((((vec4<f32>(_e4.g0_.z) * vec4<f32>(_e8.g0_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) + ((vec4<f32>(_e19.g0_.w) * vec4<f32>(_e23.g0_.w)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((_e35.g0_.yxxx * _e38.g0_.yyzw) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0)))), (_e52.g0_ * vec4<f32>(_e54.g0_.x)));
}

fn plane_rotor_inner_product(self_1106: Plane, other_912: Rotor) -> Plane {
    var self_1107: Plane;
    var other_913: Rotor;

    self_1107 = self_1106;
    other_913 = other_912;
    let _e4: Plane = self_1107;
    let _e8: Rotor = other_913;
    let _e19: Plane = self_1107;
    let _e23: Rotor = other_913;
    let _e35: Plane = self_1107;
    let _e38: Rotor = other_913;
    return Plane(((((vec4<f32>(_e4.g0_.z) * _e8.g0_.wwxy) * vec4<f32>(0.0, -(1.0), 1.0, 1.0)) + ((vec4<f32>(_e19.g0_.w) * _e23.g0_.zzyx) * vec4<f32>(0.0, 1.0, -(1.0), 1.0))) + ((_e35.g0_.xyyy * _e38.g0_.xxwz) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))));
}

fn plane_rotor_right_contraction(self_1108: Plane, other_914: Rotor) -> Plane {
    var self_1109: Plane;
    var other_915: Rotor;

    self_1109 = self_1108;
    other_915 = other_914;
    let _e4: Plane = self_1109;
    let _e6: Rotor = other_915;
    return Plane((_e4.g0_ * vec4<f32>(_e6.g0_.x)));
}

fn plane_point_add(self_1110: Plane, other_916: Point) -> PointAndPlane {
    var self_1111: Plane;
    var other_917: Point;

    self_1111 = self_1110;
    other_917 = other_916;
    let _e4: Point = other_917;
    let _e6: Plane = self_1111;
    return PointAndPlane(_e4.g0_, _e6.g0_);
}

fn plane_point_sub(self_1112: Plane, other_918: Point) -> PointAndPlane {
    var self_1113: Plane;
    var other_919: Point;

    self_1113 = self_1112;
    other_919 = other_918;
    let _e6: Point = other_919;
    let _e9: Plane = self_1113;
    return PointAndPlane((vec4<f32>(0.0) - _e6.g0_), _e9.g0_);
}

fn plane_point_regressive_product(self_1114: Plane, other_920: Point) -> Scalar {
    var self_1115: Plane;
    var other_921: Point;

    self_1115 = self_1114;
    other_921 = other_920;
    let _e4: Plane = self_1115;
    let _e7: Point = other_921;
    let _e11: Plane = self_1115;
    let _e14: Point = other_921;
    let _e19: Plane = self_1115;
    let _e22: Point = other_921;
    let _e27: Plane = self_1115;
    let _e30: Point = other_921;
    return Scalar(((((_e4.g0_.x * _e7.g0_.x) + (_e11.g0_.y * _e14.g0_.y)) + (_e19.g0_.z * _e22.g0_.z)) + (_e27.g0_.w * _e30.g0_.w)));
}

fn plane_point_outer_product(self_1116: Plane, other_922: Point) -> AntiScalar {
    var self_1117: Plane;
    var other_923: Point;

    self_1117 = self_1116;
    other_923 = other_922;
    let _e4: Plane = self_1117;
    let _e7: Point = other_923;
    let _e11: Plane = self_1117;
    let _e14: Point = other_923;
    let _e19: Plane = self_1117;
    let _e22: Point = other_923;
    let _e27: Plane = self_1117;
    let _e30: Point = other_923;
    return AntiScalar(((((_e4.g0_.x * _e7.g0_.x) + (_e11.g0_.y * _e14.g0_.y)) + (_e19.g0_.z * _e22.g0_.z)) + (_e27.g0_.w * _e30.g0_.w)));
}

fn plane_point_inner_product(self_1118: Plane, other_924: Point) -> Line {
    var self_1119: Plane;
    var other_925: Point;

    self_1119 = self_1118;
    other_925 = other_924;
    let _e4: Plane = self_1119;
    let _e8: Point = other_925;
    let _e11: Point = other_925;
    let _e14: Point = other_925;
    let _e25: Plane = self_1119;
    let _e29: Point = other_925;
    let _e32: Point = other_925;
    let _e35: Point = other_925;
    let _e47: Plane = self_1119;
    let _e50: Plane = self_1119;
    let _e53: Plane = self_1119;
    let _e57: Point = other_925;
    let _e60: Point = other_925;
    let _e63: Point = other_925;
    let _e75: Plane = self_1119;
    let _e78: Plane = self_1119;
    let _e81: Plane = self_1119;
    let _e85: Point = other_925;
    return Line(((((vec3<f32>(_e4.g0_.z) * vec3<f32>(_e8.g0_.w, _e11.g0_.w, _e14.g0_.y)) * vec3<f32>(-(1.0), 0.0, 1.0)) + ((vec3<f32>(_e25.g0_.w) * vec3<f32>(_e29.g0_.z, _e32.g0_.y, _e35.g0_.z)) * vec3<f32>(1.0, -(1.0), 0.0))) + ((vec3<f32>(_e47.g0_.x, _e50.g0_.y, _e53.g0_.y) * vec3<f32>(_e57.g0_.x, _e60.g0_.w, _e63.g0_.z)) * vec3<f32>(0.0, 1.0, -(1.0)))), (vec3<f32>(_e75.g0_.y, _e78.g0_.z, _e81.g0_.w) * vec3<f32>(_e85.g0_.x)));
}

fn plane_point_inner_anti_product(self_1120: Plane, other_926: Point) -> Line {
    var self_1121: Plane;
    var other_927: Point;

    self_1121 = self_1120;
    other_927 = other_926;
    let _e6: Plane = self_1121;
    let _e10: Point = other_927;
    let _e13: Point = other_927;
    let _e16: Point = other_927;
    let _e22: Plane = self_1121;
    let _e26: Point = other_927;
    let _e29: Point = other_927;
    let _e32: Point = other_927;
    let _e43: Plane = self_1121;
    let _e47: Point = other_927;
    let _e50: Point = other_927;
    let _e53: Point = other_927;
    let _e65: Plane = self_1121;
    let _e68: Plane = self_1121;
    let _e71: Plane = self_1121;
    let _e75: Point = other_927;
    let _e78: Point = other_927;
    let _e81: Point = other_927;
    return Line((vec3<f32>(0.0) - (vec3<f32>(_e6.g0_.x) * vec3<f32>(_e10.g0_.y, _e13.g0_.z, _e16.g0_.w))), ((((vec3<f32>(_e22.g0_.z) * vec3<f32>(_e26.g0_.w, _e29.g0_.w, _e32.g0_.y)) * vec3<f32>(-(1.0), 0.0, 1.0)) + ((vec3<f32>(_e43.g0_.w) * vec3<f32>(_e47.g0_.z, _e50.g0_.y, _e53.g0_.z)) * vec3<f32>(1.0, -(1.0), 0.0))) + ((vec3<f32>(_e65.g0_.x, _e68.g0_.y, _e71.g0_.y) * vec3<f32>(_e75.g0_.x, _e78.g0_.w, _e81.g0_.z)) * vec3<f32>(0.0, 1.0, -(1.0)))));
}

fn plane_point_left_contraction(self_1122: Plane, other_928: Point) -> Line {
    var self_1123: Plane;
    var other_929: Point;

    self_1123 = self_1122;
    other_929 = other_928;
    let _e4: Plane = self_1123;
    let _e8: Point = other_929;
    let _e11: Point = other_929;
    let _e14: Point = other_929;
    let _e25: Plane = self_1123;
    let _e29: Point = other_929;
    let _e32: Point = other_929;
    let _e35: Point = other_929;
    let _e47: Plane = self_1123;
    let _e50: Plane = self_1123;
    let _e53: Plane = self_1123;
    let _e57: Point = other_929;
    let _e60: Point = other_929;
    let _e63: Point = other_929;
    let _e75: Plane = self_1123;
    let _e78: Plane = self_1123;
    let _e81: Plane = self_1123;
    let _e85: Point = other_929;
    return Line(((((vec3<f32>(_e4.g0_.z) * vec3<f32>(_e8.g0_.w, _e11.g0_.w, _e14.g0_.y)) * vec3<f32>(-(1.0), 0.0, 1.0)) + ((vec3<f32>(_e25.g0_.w) * vec3<f32>(_e29.g0_.z, _e32.g0_.y, _e35.g0_.z)) * vec3<f32>(1.0, -(1.0), 0.0))) + ((vec3<f32>(_e47.g0_.x, _e50.g0_.y, _e53.g0_.y) * vec3<f32>(_e57.g0_.x, _e60.g0_.w, _e63.g0_.z)) * vec3<f32>(0.0, 1.0, -(1.0)))), (vec3<f32>(_e75.g0_.y, _e78.g0_.z, _e81.g0_.w) * vec3<f32>(_e85.g0_.x)));
}

fn plane_point_right_anti_contraction(self_1124: Plane, other_930: Point) -> Line {
    var self_1125: Plane;
    var other_931: Point;

    self_1125 = self_1124;
    other_931 = other_930;
    let _e6: Plane = self_1125;
    let _e10: Point = other_931;
    let _e13: Point = other_931;
    let _e16: Point = other_931;
    let _e22: Plane = self_1125;
    let _e26: Point = other_931;
    let _e29: Point = other_931;
    let _e32: Point = other_931;
    let _e43: Plane = self_1125;
    let _e47: Point = other_931;
    let _e50: Point = other_931;
    let _e53: Point = other_931;
    let _e65: Plane = self_1125;
    let _e68: Plane = self_1125;
    let _e71: Plane = self_1125;
    let _e75: Point = other_931;
    let _e78: Point = other_931;
    let _e81: Point = other_931;
    return Line((vec3<f32>(0.0) - (vec3<f32>(_e6.g0_.x) * vec3<f32>(_e10.g0_.y, _e13.g0_.z, _e16.g0_.w))), ((((vec3<f32>(_e22.g0_.z) * vec3<f32>(_e26.g0_.w, _e29.g0_.w, _e32.g0_.y)) * vec3<f32>(-(1.0), 0.0, 1.0)) + ((vec3<f32>(_e43.g0_.w) * vec3<f32>(_e47.g0_.z, _e50.g0_.y, _e53.g0_.z)) * vec3<f32>(1.0, -(1.0), 0.0))) + ((vec3<f32>(_e65.g0_.x, _e68.g0_.y, _e71.g0_.y) * vec3<f32>(_e75.g0_.x, _e78.g0_.w, _e81.g0_.z)) * vec3<f32>(0.0, 1.0, -(1.0)))));
}

fn plane_ideal_point_inner_anti_product(self_1126: Plane, other_932: IdealPoint) -> Point {
    var self_1127: Plane;
    var other_933: IdealPoint;

    self_1127 = self_1126;
    other_933 = other_932;
    let _e4: Plane = self_1127;
    let _e8: IdealPoint = other_933;
    let _e20: Plane = self_1127;
    let _e24: IdealPoint = other_933;
    let _e37: Plane = self_1127;
    let _e40: IdealPoint = other_933;
    let _e43: IdealPoint = other_933;
    let _e46: IdealPoint = other_933;
    let _e49: IdealPoint = other_933;
    return Point(((((vec4<f32>(_e4.g0_.z) * vec4<f32>(_e8.g0_.y)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0)) + ((vec4<f32>(_e20.g0_.w) * vec4<f32>(_e24.g0_.z)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((_e37.g0_.yxxx * vec4<f32>(_e40.g0_.x, _e43.g0_.x, _e46.g0_.y, _e49.g0_.z)) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0))));
}

fn plane_ideal_point_right_anti_contraction(self_1128: Plane, other_934: IdealPoint) -> Point {
    var self_1129: Plane;
    var other_935: IdealPoint;

    self_1129 = self_1128;
    other_935 = other_934;
    let _e4: Plane = self_1129;
    let _e8: IdealPoint = other_935;
    let _e20: Plane = self_1129;
    let _e24: IdealPoint = other_935;
    let _e37: Plane = self_1129;
    let _e40: IdealPoint = other_935;
    let _e43: IdealPoint = other_935;
    let _e46: IdealPoint = other_935;
    let _e49: IdealPoint = other_935;
    return Point(((((vec4<f32>(_e4.g0_.z) * vec4<f32>(_e8.g0_.y)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0)) + ((vec4<f32>(_e20.g0_.w) * vec4<f32>(_e24.g0_.z)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((_e37.g0_.yxxx * vec4<f32>(_e40.g0_.x, _e43.g0_.x, _e46.g0_.y, _e49.g0_.z)) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0))));
}

fn plane_plane_add(self_1130: Plane, other_936: Plane) -> Plane {
    var self_1131: Plane;
    var other_937: Plane;

    self_1131 = self_1130;
    other_937 = other_936;
    let _e4: Plane = self_1131;
    let _e6: Plane = other_937;
    return Plane((_e4.g0_ + _e6.g0_));
}

fn plane_plane_sub(self_1132: Plane, other_938: Plane) -> Plane {
    var self_1133: Plane;
    var other_939: Plane;

    self_1133 = self_1132;
    other_939 = other_938;
    let _e4: Plane = self_1133;
    let _e6: Plane = other_939;
    return Plane((_e4.g0_ - _e6.g0_));
}

fn plane_plane_mul(self_1134: Plane, other_940: Plane) -> Plane {
    var self_1135: Plane;
    var other_941: Plane;

    self_1135 = self_1134;
    other_941 = other_940;
    let _e4: Plane = self_1135;
    let _e6: Plane = other_941;
    return Plane((_e4.g0_ * _e6.g0_));
}

fn plane_plane_div(self_1136: Plane, other_942: Plane) -> Plane {
    var self_1137: Plane;
    var other_943: Plane;

    self_1137 = self_1136;
    other_943 = other_942;
    let _e4: Plane = self_1137;
    let _e7: Plane = self_1137;
    let _e10: Plane = self_1137;
    let _e13: Plane = self_1137;
    let _e23: Plane = other_943;
    let _e26: Plane = other_943;
    let _e29: Plane = other_943;
    let _e32: Plane = other_943;
    return Plane((((vec4<f32>(_e4.g0_.x, _e7.g0_.y, _e10.g0_.z, _e13.g0_.w) * vec4<f32>(1.0, 1.0, 1.0, 1.0)) / vec4<f32>(_e23.g0_.x, _e26.g0_.y, _e29.g0_.z, _e32.g0_.w)) * vec4<f32>(1.0, 1.0, 1.0, 1.0)));
}

fn plane_plane_outer_product(self_1138: Plane, other_944: Plane) -> Line {
    var self_1139: Plane;
    var other_945: Plane;

    self_1139 = self_1138;
    other_945 = other_944;
    let _e4: Plane = self_1139;
    let _e8: Plane = other_945;
    let _e11: Plane = other_945;
    let _e14: Plane = other_945;
    let _e19: Plane = self_1139;
    let _e22: Plane = self_1139;
    let _e25: Plane = self_1139;
    let _e29: Plane = other_945;
    let _e39: Plane = self_1139;
    let _e43: Plane = other_945;
    let _e46: Plane = other_945;
    let _e49: Plane = other_945;
    let _e60: Plane = self_1139;
    let _e64: Plane = other_945;
    let _e67: Plane = other_945;
    let _e70: Plane = other_945;
    let _e82: Plane = self_1139;
    let _e85: Plane = self_1139;
    let _e88: Plane = self_1139;
    let _e92: Plane = other_945;
    let _e95: Plane = other_945;
    let _e98: Plane = other_945;
    return Line(((vec3<f32>(_e4.g0_.x) * vec3<f32>(_e8.g0_.y, _e11.g0_.z, _e14.g0_.w)) + ((vec3<f32>(_e19.g0_.y, _e22.g0_.z, _e25.g0_.w) * vec3<f32>(_e29.g0_.x)) * vec3<f32>(-(1.0)))), ((((vec3<f32>(_e39.g0_.z) * vec3<f32>(_e43.g0_.w, _e46.g0_.w, _e49.g0_.y)) * vec3<f32>(1.0, 0.0, -(1.0))) + ((vec3<f32>(_e60.g0_.w) * vec3<f32>(_e64.g0_.z, _e67.g0_.y, _e70.g0_.z)) * vec3<f32>(-(1.0), 1.0, 0.0))) + ((vec3<f32>(_e82.g0_.x, _e85.g0_.y, _e88.g0_.y) * vec3<f32>(_e92.g0_.x, _e95.g0_.w, _e98.g0_.z)) * vec3<f32>(0.0, -(1.0), 1.0))));
}

fn plane_plane_inner_product(self_1140: Plane, other_946: Plane) -> Scalar {
    var self_1141: Plane;
    var other_947: Plane;

    self_1141 = self_1140;
    other_947 = other_946;
    let _e4: Plane = self_1141;
    let _e7: Plane = other_947;
    let _e11: Plane = self_1141;
    let _e14: Plane = other_947;
    let _e19: Plane = self_1141;
    let _e22: Plane = other_947;
    return Scalar((((_e4.g0_.y * _e7.g0_.y) + (_e11.g0_.z * _e14.g0_.z)) + (_e19.g0_.w * _e22.g0_.w)));
}

fn plane_plane_inner_anti_product(self_1142: Plane, other_948: Plane) -> AntiScalar {
    var self_1143: Plane;
    var other_949: Plane;

    self_1143 = self_1142;
    other_949 = other_948;
    let _e5: Plane = self_1143;
    let _e8: Plane = other_949;
    return AntiScalar((0.0 - (_e5.g0_.x * _e8.g0_.x)));
}

fn plane_plane_left_contraction(self_1144: Plane, other_950: Plane) -> Scalar {
    var self_1145: Plane;
    var other_951: Plane;

    self_1145 = self_1144;
    other_951 = other_950;
    let _e4: Plane = self_1145;
    let _e7: Plane = other_951;
    let _e11: Plane = self_1145;
    let _e14: Plane = other_951;
    let _e19: Plane = self_1145;
    let _e22: Plane = other_951;
    return Scalar((((_e4.g0_.y * _e7.g0_.y) + (_e11.g0_.z * _e14.g0_.z)) + (_e19.g0_.w * _e22.g0_.w)));
}

fn plane_plane_right_contraction(self_1146: Plane, other_952: Plane) -> Scalar {
    var self_1147: Plane;
    var other_953: Plane;

    self_1147 = self_1146;
    other_953 = other_952;
    let _e4: Plane = self_1147;
    let _e7: Plane = other_953;
    let _e11: Plane = self_1147;
    let _e14: Plane = other_953;
    let _e19: Plane = self_1147;
    let _e22: Plane = other_953;
    return Scalar((((_e4.g0_.y * _e7.g0_.y) + (_e11.g0_.z * _e14.g0_.z)) + (_e19.g0_.w * _e22.g0_.w)));
}

fn plane_plane_left_anti_contraction(self_1148: Plane, other_954: Plane) -> AntiScalar {
    var self_1149: Plane;
    var other_955: Plane;

    self_1149 = self_1148;
    other_955 = other_954;
    let _e5: Plane = self_1149;
    let _e8: Plane = other_955;
    return AntiScalar((0.0 - (_e5.g0_.x * _e8.g0_.x)));
}

fn plane_plane_right_anti_contraction(self_1150: Plane, other_956: Plane) -> AntiScalar {
    var self_1151: Plane;
    var other_957: Plane;

    self_1151 = self_1150;
    other_957 = other_956;
    let _e5: Plane = self_1151;
    let _e8: Plane = other_957;
    return AntiScalar((0.0 - (_e5.g0_.x * _e8.g0_.x)));
}

fn plane_plane_scalar_product(self_1152: Plane, other_958: Plane) -> Scalar {
    var self_1153: Plane;
    var other_959: Plane;

    self_1153 = self_1152;
    other_959 = other_958;
    let _e4: Plane = self_1153;
    let _e7: Plane = other_959;
    let _e11: Plane = self_1153;
    let _e14: Plane = other_959;
    let _e19: Plane = self_1153;
    let _e22: Plane = other_959;
    return Scalar((((_e4.g0_.y * _e7.g0_.y) + (_e11.g0_.z * _e14.g0_.z)) + (_e19.g0_.w * _e22.g0_.w)));
}

fn plane_plane_anti_scalar_product(self_1154: Plane, other_960: Plane) -> AntiScalar {
    var self_1155: Plane;
    var other_961: Plane;

    self_1155 = self_1154;
    other_961 = other_960;
    let _e5: Plane = self_1155;
    let _e8: Plane = other_961;
    return AntiScalar((0.0 - (_e5.g0_.x * _e8.g0_.x)));
}

fn plane_line_geometric_product(self_1156: Plane, other_962: Line) -> PointAndPlane {
    var self_1157: Plane;
    var other_963: Line;

    self_1157 = self_1156;
    other_963 = other_962;
    let _e4: Plane = self_1157;
    let _e8: Line = other_963;
    let _e11: Line = other_963;
    let _e14: Line = other_963;
    let _e17: Line = other_963;
    let _e29: Plane = self_1157;
    let _e33: Line = other_963;
    let _e36: Line = other_963;
    let _e39: Line = other_963;
    let _e42: Line = other_963;
    let _e55: Plane = self_1157;
    let _e59: Line = other_963;
    let _e62: Line = other_963;
    let _e65: Line = other_963;
    let _e68: Line = other_963;
    let _e81: Plane = self_1157;
    let _e85: Line = other_963;
    let _e88: Line = other_963;
    let _e91: Line = other_963;
    let _e94: Line = other_963;
    let _e109: Plane = self_1157;
    let _e113: Line = other_963;
    let _e116: Line = other_963;
    let _e119: Line = other_963;
    let _e122: Line = other_963;
    let _e135: Plane = self_1157;
    let _e139: Line = other_963;
    let _e142: Line = other_963;
    let _e145: Line = other_963;
    let _e148: Line = other_963;
    let _e162: Plane = self_1157;
    let _e165: Line = other_963;
    let _e168: Line = other_963;
    let _e171: Line = other_963;
    let _e174: Line = other_963;
    return PointAndPlane((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g1_.x, _e11.g1_.x, _e14.g0_.z, _e17.g0_.y)) * vec4<f32>(1.0, 0.0, -(1.0), 1.0)) + ((vec4<f32>(_e29.g0_.z) * vec4<f32>(_e33.g1_.y, _e36.g0_.z, _e39.g1_.y, _e42.g0_.x)) * vec4<f32>(1.0, 1.0, 0.0, -(1.0)))) + ((vec4<f32>(_e55.g0_.w) * vec4<f32>(_e59.g1_.z, _e62.g0_.y, _e65.g0_.x, _e68.g1_.z)) * vec4<f32>(1.0, -(1.0), 1.0, 0.0))) + ((vec4<f32>(_e81.g0_.x) * vec4<f32>(_e85.g1_.x, _e88.g1_.x, _e91.g1_.y, _e94.g1_.z)) * vec4<f32>(0.0, -(1.0), -(1.0), -(1.0)))), ((((vec4<f32>(_e109.g0_.z) * vec4<f32>(_e113.g0_.y, _e116.g1_.z, _e119.g0_.y, _e122.g1_.x)) * vec4<f32>(-(1.0), -(1.0), 0.0, 1.0)) + ((vec4<f32>(_e135.g0_.w) * vec4<f32>(_e139.g0_.z, _e142.g1_.y, _e145.g1_.x, _e148.g0_.z)) * vec4<f32>(-(1.0), 1.0, -(1.0), 0.0))) + ((_e162.g0_.yxyy * vec4<f32>(_e165.g0_.x, _e168.g0_.x, _e171.g1_.z, _e174.g1_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0)))));
}

fn plane_line_outer_product(self_1158: Plane, other_964: Line) -> Point {
    var self_1159: Plane;
    var other_965: Line;

    self_1159 = self_1158;
    other_965 = other_964;
    let _e4: Plane = self_1159;
    let _e8: Line = other_965;
    let _e11: Line = other_965;
    let _e14: Line = other_965;
    let _e17: Line = other_965;
    let _e29: Plane = self_1159;
    let _e33: Line = other_965;
    let _e36: Line = other_965;
    let _e39: Line = other_965;
    let _e42: Line = other_965;
    let _e55: Plane = self_1159;
    let _e59: Line = other_965;
    let _e62: Line = other_965;
    let _e65: Line = other_965;
    let _e68: Line = other_965;
    let _e81: Plane = self_1159;
    let _e85: Line = other_965;
    let _e88: Line = other_965;
    let _e91: Line = other_965;
    let _e94: Line = other_965;
    return Point((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g1_.x, _e11.g1_.x, _e14.g0_.z, _e17.g0_.y)) * vec4<f32>(1.0, 0.0, -(1.0), 1.0)) + ((vec4<f32>(_e29.g0_.z) * vec4<f32>(_e33.g1_.y, _e36.g0_.z, _e39.g1_.y, _e42.g0_.x)) * vec4<f32>(1.0, 1.0, 0.0, -(1.0)))) + ((vec4<f32>(_e55.g0_.w) * vec4<f32>(_e59.g1_.z, _e62.g0_.y, _e65.g0_.x, _e68.g1_.z)) * vec4<f32>(1.0, -(1.0), 1.0, 0.0))) + ((vec4<f32>(_e81.g0_.x) * vec4<f32>(_e85.g1_.x, _e88.g1_.x, _e91.g1_.y, _e94.g1_.z)) * vec4<f32>(0.0, -(1.0), -(1.0), -(1.0)))));
}

fn plane_line_inner_product(self_1160: Plane, other_966: Line) -> Plane {
    var self_1161: Plane;
    var other_967: Line;

    self_1161 = self_1160;
    other_967 = other_966;
    let _e4: Plane = self_1161;
    let _e8: Line = other_967;
    let _e11: Line = other_967;
    let _e14: Line = other_967;
    let _e17: Line = other_967;
    let _e30: Plane = self_1161;
    let _e34: Line = other_967;
    let _e37: Line = other_967;
    let _e40: Line = other_967;
    let _e43: Line = other_967;
    let _e57: Plane = self_1161;
    let _e60: Line = other_967;
    let _e63: Line = other_967;
    let _e66: Line = other_967;
    let _e69: Line = other_967;
    return Plane(((((vec4<f32>(_e4.g0_.z) * vec4<f32>(_e8.g0_.y, _e11.g1_.z, _e14.g0_.y, _e17.g1_.x)) * vec4<f32>(-(1.0), -(1.0), 0.0, 1.0)) + ((vec4<f32>(_e30.g0_.w) * vec4<f32>(_e34.g0_.z, _e37.g1_.y, _e40.g1_.x, _e43.g0_.z)) * vec4<f32>(-(1.0), 1.0, -(1.0), 0.0))) + ((_e57.g0_.yxyy * vec4<f32>(_e60.g0_.x, _e63.g0_.x, _e66.g1_.z, _e69.g1_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0)))));
}

fn plane_line_inner_anti_product(self_1162: Plane, other_968: Line) -> Point {
    var self_1163: Plane;
    var other_969: Line;

    self_1163 = self_1162;
    other_969 = other_968;
    let _e4: Plane = self_1163;
    let _e8: Line = other_969;
    let _e20: Plane = self_1163;
    let _e24: Line = other_969;
    let _e37: Plane = self_1163;
    let _e40: Line = other_969;
    let _e43: Line = other_969;
    let _e46: Line = other_969;
    let _e49: Line = other_969;
    return Point(((((vec4<f32>(_e4.g0_.z) * vec4<f32>(_e8.g0_.y)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0)) + ((vec4<f32>(_e20.g0_.w) * vec4<f32>(_e24.g0_.z)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((_e37.g0_.yxxx * vec4<f32>(_e40.g0_.x, _e43.g0_.x, _e46.g0_.y, _e49.g0_.z)) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0))));
}

fn plane_line_left_contraction(self_1164: Plane, other_970: Line) -> Plane {
    var self_1165: Plane;
    var other_971: Line;

    self_1165 = self_1164;
    other_971 = other_970;
    let _e4: Plane = self_1165;
    let _e8: Line = other_971;
    let _e11: Line = other_971;
    let _e14: Line = other_971;
    let _e17: Line = other_971;
    let _e30: Plane = self_1165;
    let _e34: Line = other_971;
    let _e37: Line = other_971;
    let _e40: Line = other_971;
    let _e43: Line = other_971;
    let _e57: Plane = self_1165;
    let _e60: Line = other_971;
    let _e63: Line = other_971;
    let _e66: Line = other_971;
    let _e69: Line = other_971;
    return Plane(((((vec4<f32>(_e4.g0_.z) * vec4<f32>(_e8.g0_.y, _e11.g1_.z, _e14.g0_.y, _e17.g1_.x)) * vec4<f32>(-(1.0), -(1.0), 0.0, 1.0)) + ((vec4<f32>(_e30.g0_.w) * vec4<f32>(_e34.g0_.z, _e37.g1_.y, _e40.g1_.x, _e43.g0_.z)) * vec4<f32>(-(1.0), 1.0, -(1.0), 0.0))) + ((_e57.g0_.yxyy * vec4<f32>(_e60.g0_.x, _e63.g0_.x, _e66.g1_.z, _e69.g1_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0)))));
}

fn plane_line_right_anti_contraction(self_1166: Plane, other_972: Line) -> Point {
    var self_1167: Plane;
    var other_973: Line;

    self_1167 = self_1166;
    other_973 = other_972;
    let _e4: Plane = self_1167;
    let _e8: Line = other_973;
    let _e20: Plane = self_1167;
    let _e24: Line = other_973;
    let _e37: Plane = self_1167;
    let _e40: Line = other_973;
    let _e43: Line = other_973;
    let _e46: Line = other_973;
    let _e49: Line = other_973;
    return Point(((((vec4<f32>(_e4.g0_.z) * vec4<f32>(_e8.g0_.y)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0)) + ((vec4<f32>(_e20.g0_.w) * vec4<f32>(_e24.g0_.z)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((_e37.g0_.yxxx * vec4<f32>(_e40.g0_.x, _e43.g0_.x, _e46.g0_.y, _e49.g0_.z)) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0))));
}

fn plane_translator_inner_product(self_1168: Plane, other_974: Translator) -> Plane {
    var self_1169: Plane;
    var other_975: Translator;

    self_1169 = self_1168;
    other_975 = other_974;
    let _e4: Plane = self_1169;
    let _e8: Translator = other_975;
    let _e19: Plane = self_1169;
    let _e23: Translator = other_975;
    let _e35: Plane = self_1169;
    let _e39: Translator = other_975;
    let _e51: Plane = self_1169;
    let _e55: Translator = other_975;
    return Plane((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0)) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.zzxz) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0))) + ((vec4<f32>(_e35.g0_.w) * _e39.g0_.wwwx) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))) + ((vec4<f32>(_e51.g0_.x) * vec4<f32>(_e55.g0_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))));
}

fn plane_translator_inner_anti_product(self_1170: Plane, other_976: Translator) -> Point {
    var self_1171: Plane;
    var other_977: Translator;

    self_1171 = self_1170;
    other_977 = other_976;
    let _e4: Plane = self_1171;
    let _e8: Translator = other_977;
    let _e18: Plane = self_1171;
    let _e22: Translator = other_977;
    let _e35: Plane = self_1171;
    let _e39: Translator = other_977;
    let _e52: Plane = self_1171;
    let _e55: Translator = other_977;
    return Point((((((vec4<f32>(_e4.g0_.x) * _e8.g0_) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0)) + ((vec4<f32>(_e18.g0_.z) * vec4<f32>(_e22.g0_.z)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((vec4<f32>(_e35.g0_.w) * vec4<f32>(_e39.g0_.w)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((_e52.g0_.yxxx * _e55.g0_.yxxx) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))));
}

fn plane_translator_right_contraction(self_1172: Plane, other_978: Translator) -> Plane {
    var self_1173: Plane;
    var other_979: Translator;

    self_1173 = self_1172;
    other_979 = other_978;
    let _e4: Plane = self_1173;
    let _e6: Translator = other_979;
    return Plane((_e4.g0_ * vec4<f32>(_e6.g0_.x)));
}

fn plane_translator_right_anti_contraction(self_1174: Plane, other_980: Translator) -> Point {
    var self_1175: Plane;
    var other_981: Translator;

    self_1175 = self_1174;
    other_981 = other_980;
    let _e4: Plane = self_1175;
    let _e8: Translator = other_981;
    let _e20: Plane = self_1175;
    let _e24: Translator = other_981;
    let _e37: Plane = self_1175;
    let _e40: Translator = other_981;
    return Point(((((vec4<f32>(_e4.g0_.z) * vec4<f32>(_e8.g0_.z)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0)) + ((vec4<f32>(_e20.g0_.w) * vec4<f32>(_e24.g0_.w)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((_e37.g0_.yxxx * _e40.g0_.yyzw) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0))));
}

fn plane_motor_geometric_product(self_1176: Plane, other_982: Motor) -> PointAndPlane {
    var self_1177: Plane;
    var other_983: Motor;

    self_1177 = self_1176;
    other_983 = other_982;
    let _e4: Plane = self_1177;
    let _e8: Motor = other_983;
    let _e11: Motor = other_983;
    let _e14: Motor = other_983;
    let _e17: Motor = other_983;
    let _e29: Plane = self_1177;
    let _e33: Motor = other_983;
    let _e36: Motor = other_983;
    let _e39: Motor = other_983;
    let _e42: Motor = other_983;
    let _e55: Plane = self_1177;
    let _e59: Motor = other_983;
    let _e62: Motor = other_983;
    let _e65: Motor = other_983;
    let _e68: Motor = other_983;
    let _e81: Plane = self_1177;
    let _e85: Motor = other_983;
    let _e98: Plane = self_1177;
    let _e102: Motor = other_983;
    let _e105: Motor = other_983;
    let _e108: Motor = other_983;
    let _e111: Motor = other_983;
    let _e124: Plane = self_1177;
    let _e128: Motor = other_983;
    let _e131: Motor = other_983;
    let _e134: Motor = other_983;
    let _e137: Motor = other_983;
    let _e151: Plane = self_1177;
    let _e155: Motor = other_983;
    let _e158: Motor = other_983;
    let _e161: Motor = other_983;
    let _e164: Motor = other_983;
    let _e178: Plane = self_1177;
    let _e182: Motor = other_983;
    return PointAndPlane((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y, _e11.g1_.x, _e14.g1_.w, _e17.g1_.z)) * vec4<f32>(1.0, 1.0, -(1.0), 1.0)) + ((vec4<f32>(_e29.g0_.z) * vec4<f32>(_e33.g0_.z, _e36.g1_.w, _e39.g1_.x, _e42.g1_.y)) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e55.g0_.w) * vec4<f32>(_e59.g0_.w, _e62.g1_.z, _e65.g1_.y, _e68.g1_.x)) * vec4<f32>(1.0, -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e81.g0_.x) * _e85.g0_) * vec4<f32>(0.0, -(1.0), -(1.0), -(1.0)))), (((((vec4<f32>(_e98.g0_.y) * vec4<f32>(_e102.g1_.y, _e105.g0_.x, _e108.g0_.w, _e111.g0_.z)) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0))) + ((vec4<f32>(_e124.g0_.z) * vec4<f32>(_e128.g1_.z, _e131.g0_.w, _e134.g0_.x, _e137.g0_.y)) * vec4<f32>(-(1.0), -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e151.g0_.w) * vec4<f32>(_e155.g1_.w, _e158.g0_.z, _e161.g0_.y, _e164.g0_.x)) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))) + ((vec4<f32>(_e178.g0_.x) * vec4<f32>(_e182.g0_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))));
}

fn plane_motor_regressive_product(self_1178: Plane, other_984: Motor) -> Plane {
    var self_1179: Plane;
    var other_985: Motor;

    self_1179 = self_1178;
    other_985 = other_984;
    let _e4: Plane = self_1179;
    let _e6: Motor = other_985;
    return Plane((_e4.g0_ * vec4<f32>(_e6.g1_.x)));
}

fn plane_motor_outer_product(self_1180: Plane, other_986: Motor) -> PointAndPlane {
    var self_1181: Plane;
    var other_987: Motor;

    self_1181 = self_1180;
    other_987 = other_986;
    let _e4: Plane = self_1181;
    let _e8: Motor = other_987;
    let _e11: Motor = other_987;
    let _e14: Motor = other_987;
    let _e17: Motor = other_987;
    let _e29: Plane = self_1181;
    let _e33: Motor = other_987;
    let _e36: Motor = other_987;
    let _e39: Motor = other_987;
    let _e42: Motor = other_987;
    let _e55: Plane = self_1181;
    let _e59: Motor = other_987;
    let _e62: Motor = other_987;
    let _e65: Motor = other_987;
    let _e68: Motor = other_987;
    let _e81: Plane = self_1181;
    let _e85: Motor = other_987;
    let _e98: Plane = self_1181;
    let _e100: Motor = other_987;
    return PointAndPlane((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y, _e11.g0_.y, _e14.g1_.w, _e17.g1_.z)) * vec4<f32>(1.0, 0.0, -(1.0), 1.0)) + ((vec4<f32>(_e29.g0_.z) * vec4<f32>(_e33.g0_.z, _e36.g1_.w, _e39.g0_.z, _e42.g1_.y)) * vec4<f32>(1.0, 1.0, 0.0, -(1.0)))) + ((vec4<f32>(_e55.g0_.w) * vec4<f32>(_e59.g0_.w, _e62.g1_.z, _e65.g1_.y, _e68.g0_.w)) * vec4<f32>(1.0, -(1.0), 1.0, 0.0))) + ((vec4<f32>(_e81.g0_.x) * _e85.g0_) * vec4<f32>(0.0, -(1.0), -(1.0), -(1.0)))), (_e98.g0_ * vec4<f32>(_e100.g0_.x)));
}

fn plane_motor_geometric_anti_product(self_1182: Plane, other_988: Motor) -> PointAndPlane {
    var self_1183: Plane;
    var other_989: Motor;

    self_1183 = self_1182;
    other_989 = other_988;
    let _e4: Plane = self_1183;
    let _e8: Motor = other_989;
    let _e11: Motor = other_989;
    let _e14: Motor = other_989;
    let _e17: Motor = other_989;
    let _e29: Plane = self_1183;
    let _e33: Motor = other_989;
    let _e46: Plane = self_1183;
    let _e50: Motor = other_989;
    let _e63: Plane = self_1183;
    let _e66: Motor = other_989;
    let _e78: Plane = self_1183;
    let _e82: Motor = other_989;
    let _e85: Motor = other_989;
    let _e88: Motor = other_989;
    let _e91: Motor = other_989;
    let _e96: Plane = self_1183;
    let _e100: Motor = other_989;
    let _e112: Plane = self_1183;
    let _e116: Motor = other_989;
    let _e128: Plane = self_1183;
    let _e131: Motor = other_989;
    return PointAndPlane((((((vec4<f32>(_e4.g0_.x) * vec4<f32>(_e8.g0_.x, _e11.g1_.y, _e14.g1_.z, _e17.g1_.w)) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0)) + ((vec4<f32>(_e29.g0_.z) * vec4<f32>(_e33.g1_.z)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((vec4<f32>(_e46.g0_.w) * vec4<f32>(_e50.g1_.w)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((_e63.g0_.yxxx * _e66.g1_.yxxx) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))), ((((vec4<f32>(_e78.g0_.x) * vec4<f32>(_e82.g1_.x, _e85.g0_.y, _e88.g0_.z, _e91.g0_.w)) + ((vec4<f32>(_e96.g0_.z) * _e100.g1_.wwxy) * vec4<f32>(0.0, -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e112.g0_.w) * _e116.g1_.zzyx) * vec4<f32>(0.0, 1.0, -(1.0), 1.0))) + ((_e128.g0_.xyyy * _e131.g1_.xxwz) * vec4<f32>(0.0, 1.0, 1.0, -(1.0)))));
}

fn plane_motor_inner_anti_product(self_1184: Plane, other_990: Motor) -> PointAndPlane {
    var self_1185: Plane;
    var other_991: Motor;

    self_1185 = self_1184;
    other_991 = other_990;
    let _e4: Plane = self_1185;
    let _e8: Motor = other_991;
    let _e11: Motor = other_991;
    let _e14: Motor = other_991;
    let _e17: Motor = other_991;
    let _e29: Plane = self_1185;
    let _e33: Motor = other_991;
    let _e46: Plane = self_1185;
    let _e50: Motor = other_991;
    let _e63: Plane = self_1185;
    let _e66: Motor = other_991;
    let _e78: Plane = self_1185;
    let _e80: Motor = other_991;
    return PointAndPlane((((((vec4<f32>(_e4.g0_.x) * vec4<f32>(_e8.g0_.x, _e11.g1_.y, _e14.g1_.z, _e17.g1_.w)) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0)) + ((vec4<f32>(_e29.g0_.z) * vec4<f32>(_e33.g1_.z)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((vec4<f32>(_e46.g0_.w) * vec4<f32>(_e50.g1_.w)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((_e63.g0_.yxxx * _e66.g1_.yxxx) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))), (_e78.g0_ * vec4<f32>(_e80.g1_.x)));
}

fn plane_motor_right_contraction(self_1186: Plane, other_992: Motor) -> Plane {
    var self_1187: Plane;
    var other_993: Motor;

    self_1187 = self_1186;
    other_993 = other_992;
    let _e4: Plane = self_1187;
    let _e6: Motor = other_993;
    return Plane((_e4.g0_ * vec4<f32>(_e6.g0_.x)));
}

fn plane_motor_right_anti_contraction(self_1188: Plane, other_994: Motor) -> PointAndPlane {
    var self_1189: Plane;
    var other_995: Motor;

    self_1189 = self_1188;
    other_995 = other_994;
    let _e4: Plane = self_1189;
    let _e8: Motor = other_995;
    let _e20: Plane = self_1189;
    let _e24: Motor = other_995;
    let _e37: Plane = self_1189;
    let _e40: Motor = other_995;
    let _e52: Plane = self_1189;
    let _e54: Motor = other_995;
    return PointAndPlane(((((vec4<f32>(_e4.g0_.z) * vec4<f32>(_e8.g1_.z)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0)) + ((vec4<f32>(_e20.g0_.w) * vec4<f32>(_e24.g1_.w)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((_e37.g0_.yxxx * _e40.g1_.yyzw) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0))), (_e52.g0_ * vec4<f32>(_e54.g1_.x)));
}

fn plane_point_and_plane_add(self_1190: Plane, other_996: PointAndPlane) -> PointAndPlane {
    var self_1191: Plane;
    var other_997: PointAndPlane;

    self_1191 = self_1190;
    other_997 = other_996;
    let _e4: PointAndPlane = other_997;
    let _e6: Plane = self_1191;
    let _e8: PointAndPlane = other_997;
    return PointAndPlane(_e4.g0_, (_e6.g0_ + _e8.g1_));
}

fn plane_point_and_plane_sub(self_1192: Plane, other_998: PointAndPlane) -> PointAndPlane {
    var self_1193: Plane;
    var other_999: PointAndPlane;

    self_1193 = self_1192;
    other_999 = other_998;
    let _e6: PointAndPlane = other_999;
    let _e9: Plane = self_1193;
    let _e11: PointAndPlane = other_999;
    return PointAndPlane((vec4<f32>(0.0) - _e6.g0_), (_e9.g0_ - _e11.g1_));
}

fn plane_point_and_plane_geometric_product(self_1194: Plane, other_1000: PointAndPlane) -> Motor {
    var self_1195: Plane;
    var other_1001: PointAndPlane;

    self_1195 = self_1194;
    other_1001 = other_1000;
    let _e4: Plane = self_1195;
    let _e8: PointAndPlane = other_1001;
    let _e11: PointAndPlane = other_1001;
    let _e14: PointAndPlane = other_1001;
    let _e17: PointAndPlane = other_1001;
    let _e29: Plane = self_1195;
    let _e33: PointAndPlane = other_1001;
    let _e36: PointAndPlane = other_1001;
    let _e39: PointAndPlane = other_1001;
    let _e42: PointAndPlane = other_1001;
    let _e55: Plane = self_1195;
    let _e59: PointAndPlane = other_1001;
    let _e62: PointAndPlane = other_1001;
    let _e65: PointAndPlane = other_1001;
    let _e68: PointAndPlane = other_1001;
    let _e81: Plane = self_1195;
    let _e85: PointAndPlane = other_1001;
    let _e88: PointAndPlane = other_1001;
    let _e91: PointAndPlane = other_1001;
    let _e94: PointAndPlane = other_1001;
    let _e99: Plane = self_1195;
    let _e103: PointAndPlane = other_1001;
    let _e106: PointAndPlane = other_1001;
    let _e109: PointAndPlane = other_1001;
    let _e112: PointAndPlane = other_1001;
    let _e126: Plane = self_1195;
    let _e130: PointAndPlane = other_1001;
    let _e133: PointAndPlane = other_1001;
    let _e136: PointAndPlane = other_1001;
    let _e139: PointAndPlane = other_1001;
    let _e153: Plane = self_1195;
    let _e157: PointAndPlane = other_1001;
    let _e160: PointAndPlane = other_1001;
    let _e163: PointAndPlane = other_1001;
    let _e166: PointAndPlane = other_1001;
    return Motor(((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g1_.y, _e11.g0_.x, _e14.g1_.w, _e17.g1_.z)) * vec4<f32>(1.0, 1.0, -(1.0), 1.0)) + ((vec4<f32>(_e29.g0_.z) * vec4<f32>(_e33.g1_.z, _e36.g1_.w, _e39.g0_.x, _e42.g1_.y)) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e55.g0_.w) * vec4<f32>(_e59.g1_.w, _e62.g1_.z, _e65.g1_.y, _e68.g0_.x)) * vec4<f32>(1.0, -(1.0), 1.0, 1.0))), ((((vec4<f32>(_e81.g0_.x) * vec4<f32>(_e85.g0_.x, _e88.g1_.y, _e91.g1_.z, _e94.g1_.w)) + ((vec4<f32>(_e99.g0_.y) * vec4<f32>(_e103.g0_.y, _e106.g1_.x, _e109.g0_.w, _e112.g0_.z)) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e126.g0_.z) * vec4<f32>(_e130.g0_.z, _e133.g0_.w, _e136.g1_.x, _e139.g0_.y)) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e153.g0_.w) * vec4<f32>(_e157.g0_.w, _e160.g0_.z, _e163.g0_.y, _e166.g1_.x)) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))));
}

fn plane_point_and_plane_regressive_product(self_1196: Plane, other_1002: PointAndPlane) -> Scalar {
    var self_1197: Plane;
    var other_1003: PointAndPlane;

    self_1197 = self_1196;
    other_1003 = other_1002;
    let _e4: Plane = self_1197;
    let _e7: PointAndPlane = other_1003;
    let _e11: Plane = self_1197;
    let _e14: PointAndPlane = other_1003;
    let _e19: Plane = self_1197;
    let _e22: PointAndPlane = other_1003;
    let _e27: Plane = self_1197;
    let _e30: PointAndPlane = other_1003;
    return Scalar(((((_e4.g0_.x * _e7.g0_.x) + (_e11.g0_.y * _e14.g0_.y)) + (_e19.g0_.z * _e22.g0_.z)) + (_e27.g0_.w * _e30.g0_.w)));
}

fn plane_point_and_plane_geometric_anti_product(self_1198: Plane, other_1004: PointAndPlane) -> Motor {
    var self_1199: Plane;
    var other_1005: PointAndPlane;

    self_1199 = self_1198;
    other_1005 = other_1004;
    let _e4: Plane = self_1199;
    let _e8: PointAndPlane = other_1005;
    let _e11: PointAndPlane = other_1005;
    let _e14: PointAndPlane = other_1005;
    let _e17: PointAndPlane = other_1005;
    let _e31: Plane = self_1199;
    let _e35: PointAndPlane = other_1005;
    let _e38: PointAndPlane = other_1005;
    let _e41: PointAndPlane = other_1005;
    let _e44: PointAndPlane = other_1005;
    let _e57: Plane = self_1199;
    let _e61: PointAndPlane = other_1005;
    let _e64: PointAndPlane = other_1005;
    let _e67: PointAndPlane = other_1005;
    let _e70: PointAndPlane = other_1005;
    let _e83: Plane = self_1199;
    let _e87: PointAndPlane = other_1005;
    let _e90: PointAndPlane = other_1005;
    let _e93: PointAndPlane = other_1005;
    let _e96: PointAndPlane = other_1005;
    let _e111: Plane = self_1199;
    let _e115: PointAndPlane = other_1005;
    let _e118: PointAndPlane = other_1005;
    let _e121: PointAndPlane = other_1005;
    let _e124: PointAndPlane = other_1005;
    return Motor((((((vec4<f32>(_e4.g0_.x) * vec4<f32>(_e8.g0_.x, _e11.g1_.y, _e14.g1_.z, _e17.g1_.w)) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))) + ((vec4<f32>(_e31.g0_.y) * vec4<f32>(_e35.g0_.y, _e38.g1_.x, _e41.g0_.w, _e44.g0_.z)) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e57.g0_.z) * vec4<f32>(_e61.g0_.z, _e64.g0_.w, _e67.g1_.x, _e70.g0_.y)) * vec4<f32>(1.0, -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e83.g0_.w) * vec4<f32>(_e87.g0_.w, _e90.g0_.z, _e93.g0_.y, _e96.g1_.x)) * vec4<f32>(1.0, 1.0, -(1.0), 1.0))), (vec4<f32>(0.0) - (vec4<f32>(_e111.g0_.x) * vec4<f32>(_e115.g1_.x, _e118.g0_.y, _e121.g0_.z, _e124.g0_.w))));
}

fn plane_point_and_plane_right_contraction(self_1200: Plane, other_1006: PointAndPlane) -> Scalar {
    var self_1201: Plane;
    var other_1007: PointAndPlane;

    self_1201 = self_1200;
    other_1007 = other_1006;
    let _e4: Plane = self_1201;
    let _e7: PointAndPlane = other_1007;
    let _e11: Plane = self_1201;
    let _e14: PointAndPlane = other_1007;
    let _e19: Plane = self_1201;
    let _e22: PointAndPlane = other_1007;
    return Scalar((((_e4.g0_.y * _e7.g1_.y) + (_e11.g0_.z * _e14.g1_.z)) + (_e19.g0_.w * _e22.g1_.w)));
}

fn plane_point_and_plane_left_anti_contraction(self_1202: Plane, other_1008: PointAndPlane) -> AntiScalar {
    var self_1203: Plane;
    var other_1009: PointAndPlane;

    self_1203 = self_1202;
    other_1009 = other_1008;
    let _e5: Plane = self_1203;
    let _e8: PointAndPlane = other_1009;
    return AntiScalar((0.0 - (_e5.g0_.x * _e8.g1_.x)));
}

fn plane_point_and_plane_scalar_product(self_1204: Plane, other_1010: PointAndPlane) -> Scalar {
    var self_1205: Plane;
    var other_1011: PointAndPlane;

    self_1205 = self_1204;
    other_1011 = other_1010;
    let _e4: Plane = self_1205;
    let _e7: PointAndPlane = other_1011;
    let _e11: Plane = self_1205;
    let _e14: PointAndPlane = other_1011;
    let _e19: Plane = self_1205;
    let _e22: PointAndPlane = other_1011;
    return Scalar((((_e4.g0_.y * _e7.g1_.y) + (_e11.g0_.z * _e14.g1_.z)) + (_e19.g0_.w * _e22.g1_.w)));
}

fn plane_point_and_plane_anti_scalar_product(self_1206: Plane, other_1012: PointAndPlane) -> AntiScalar {
    var self_1207: Plane;
    var other_1013: PointAndPlane;

    self_1207 = self_1206;
    other_1013 = other_1012;
    let _e5: Plane = self_1207;
    let _e8: PointAndPlane = other_1013;
    return AntiScalar((0.0 - (_e5.g0_.x * _e8.g1_.x)));
}

fn plane_squared_magnitude(self_1208: Plane) -> Scalar {
    var self_1209: Plane;

    self_1209 = self_1208;
    let _e2: Plane = self_1209;
    let _e3: Plane = self_1209;
    let _e4: Plane = plane_reversal(_e3);
    let _e5: Scalar = plane_plane_scalar_product(_e2, _e4);
    return _e5;
}

fn plane_magnitude(self_1210: Plane) -> Scalar {
    var self_1211: Plane;

    self_1211 = self_1210;
    let _e2: Plane = self_1211;
    let _e3: Scalar = plane_squared_magnitude(_e2);
    return Scalar(sqrt(_e3.g0_));
}

fn plane_bulk_norm(self_1212: Plane) -> Scalar {
    var self_1213: Plane;

    self_1213 = self_1212;
    let _e2: Plane = self_1213;
    let _e3: Scalar = plane_squared_magnitude(_e2);
    return Scalar(sqrt(_e3.g0_));
}

fn plane_squared_anti_magnitude(self_1214: Plane) -> AntiScalar {
    var self_1215: Plane;

    self_1215 = self_1214;
    let _e2: Plane = self_1215;
    let _e3: Plane = self_1215;
    let _e4: Plane = plane_anti_reversal(_e3);
    let _e5: AntiScalar = plane_plane_anti_scalar_product(_e2, _e4);
    return _e5;
}

fn plane_weight_norm(self_1216: Plane) -> AntiScalar {
    var self_1217: Plane;

    self_1217 = self_1216;
    let _e2: Plane = self_1217;
    let _e3: AntiScalar = plane_squared_anti_magnitude(_e2);
    return AntiScalar(sqrt(_e3.g0_));
}

fn plane_scale(self_1218: Plane, other_1014: f32) -> Plane {
    var self_1219: Plane;
    var other_1015: f32;

    self_1219 = self_1218;
    other_1015 = other_1014;
    let _e4: Plane = self_1219;
    let _e5: f32 = other_1015;
    let _e7: Plane = plane_scalar_geometric_product(_e4, Scalar(_e5));
    return _e7;
}

fn plane_signum(self_1220: Plane) -> Plane {
    var self_1221: Plane;

    self_1221 = self_1220;
    let _e2: Plane = self_1221;
    let _e3: Plane = self_1221;
    let _e4: Scalar = plane_magnitude(_e3);
    let _e9: Plane = plane_scalar_geometric_product(_e2, Scalar((1.0 / _e4.g0_)));
    return _e9;
}

fn plane_inverse(self_1222: Plane) -> Plane {
    var self_1223: Plane;

    self_1223 = self_1222;
    let _e2: Plane = self_1223;
    let _e3: Plane = plane_reversal(_e2);
    let _e4: Plane = self_1223;
    let _e5: Scalar = plane_squared_magnitude(_e4);
    let _e10: Plane = plane_scalar_geometric_product(_e3, Scalar((1.0 / _e5.g0_)));
    return _e10;
}

fn plane_unitize(self_1224: Plane) -> Plane {
    var self_1225: Plane;

    self_1225 = self_1224;
    let _e2: Plane = self_1225;
    let _e3: Plane = self_1225;
    let _e4: AntiScalar = plane_weight_norm(_e3);
    let _e9: Plane = plane_scalar_geometric_product(_e2, Scalar((1.0 / _e4.g0_)));
    return _e9;
}

fn line_zero() -> Line {
    return Line(vec3<f32>(0.0), vec3<f32>(0.0));
}

fn line_one() -> Line {
    return Line(vec3<f32>(0.0), vec3<f32>(0.0));
}

fn line_grade(self_1226: Line) -> i32 {
    return 2;
}

fn line_anti_grade(self_1227: Line) -> i32 {
    return 2;
}

fn line_neg(self_1228: Line) -> Line {
    var self_1229: Line;

    self_1229 = self_1228;
    let _e2: Line = self_1229;
    let _e8: Line = self_1229;
    return Line((_e2.g0_ * vec3<f32>(-(1.0))), (_e8.g1_ * vec3<f32>(-(1.0))));
}

fn line_automorphism(self_1230: Line) -> Line {
    var self_1231: Line;

    self_1231 = self_1230;
    let _e2: Line = self_1231;
    let _e4: Line = self_1231;
    return Line(_e2.g0_, _e4.g1_);
}

fn line_reversal(self_1232: Line) -> Line {
    var self_1233: Line;

    self_1233 = self_1232;
    let _e2: Line = self_1233;
    let _e8: Line = self_1233;
    return Line((_e2.g0_ * vec3<f32>(-(1.0))), (_e8.g1_ * vec3<f32>(-(1.0))));
}

fn line_conjugation(self_1234: Line) -> Line {
    var self_1235: Line;

    self_1235 = self_1234;
    let _e2: Line = self_1235;
    let _e8: Line = self_1235;
    return Line((_e2.g0_ * vec3<f32>(-(1.0))), (_e8.g1_ * vec3<f32>(-(1.0))));
}

fn line_dual(self_1236: Line) -> Line {
    var self_1237: Line;

    self_1237 = self_1236;
    let _e2: Line = self_1237;
    let _e4: Line = self_1237;
    return Line(_e2.g1_, _e4.g0_);
}

fn line_anti_reversal(self_1238: Line) -> Line {
    var self_1239: Line;

    self_1239 = self_1238;
    let _e2: Line = self_1239;
    let _e8: Line = self_1239;
    return Line((_e2.g0_ * vec3<f32>(-(1.0))), (_e8.g1_ * vec3<f32>(-(1.0))));
}

fn line_right_complement(self_1240: Line) -> Line {
    var self_1241: Line;

    self_1241 = self_1240;
    let _e2: Line = self_1241;
    let _e4: Line = self_1241;
    return Line(_e2.g1_, _e4.g0_);
}

fn line_left_complement(self_1242: Line) -> Line {
    var self_1243: Line;

    self_1243 = self_1242;
    let _e2: Line = self_1243;
    let _e4: Line = self_1243;
    return Line(_e2.g1_, _e4.g0_);
}

fn line_double_complement(self_1244: Line) -> Line {
    var self_1245: Line;

    self_1245 = self_1244;
    let _e2: Line = self_1245;
    let _e4: Line = self_1245;
    return Line(_e2.g0_, _e4.g1_);
}

fn line_scalar_geometric_product(self_1246: Line, other_1016: Scalar) -> Line {
    var self_1247: Line;
    var other_1017: Scalar;

    self_1247 = self_1246;
    other_1017 = other_1016;
    let _e4: Line = self_1247;
    let _e6: Scalar = other_1017;
    let _e10: Line = self_1247;
    let _e12: Scalar = other_1017;
    return Line((_e4.g0_ * vec3<f32>(_e6.g0_)), (_e10.g1_ * vec3<f32>(_e12.g0_)));
}

fn line_scalar_outer_product(self_1248: Line, other_1018: Scalar) -> Line {
    var self_1249: Line;
    var other_1019: Scalar;

    self_1249 = self_1248;
    other_1019 = other_1018;
    let _e4: Line = self_1249;
    let _e6: Scalar = other_1019;
    let _e10: Line = self_1249;
    let _e12: Scalar = other_1019;
    return Line((_e4.g0_ * vec3<f32>(_e6.g0_)), (_e10.g1_ * vec3<f32>(_e12.g0_)));
}

fn line_scalar_inner_product(self_1250: Line, other_1020: Scalar) -> Line {
    var self_1251: Line;
    var other_1021: Scalar;

    self_1251 = self_1250;
    other_1021 = other_1020;
    let _e4: Line = self_1251;
    let _e6: Scalar = other_1021;
    let _e10: Line = self_1251;
    let _e12: Scalar = other_1021;
    return Line((_e4.g0_ * vec3<f32>(_e6.g0_)), (_e10.g1_ * vec3<f32>(_e12.g0_)));
}

fn line_scalar_right_contraction(self_1252: Line, other_1022: Scalar) -> Line {
    var self_1253: Line;
    var other_1023: Scalar;

    self_1253 = self_1252;
    other_1023 = other_1022;
    let _e4: Line = self_1253;
    let _e6: Scalar = other_1023;
    let _e10: Line = self_1253;
    let _e12: Scalar = other_1023;
    return Line((_e4.g0_ * vec3<f32>(_e6.g0_)), (_e10.g1_ * vec3<f32>(_e12.g0_)));
}

fn line_anti_scalar_geometric_product(self_1254: Line, other_1024: AntiScalar) -> IdealPoint {
    var self_1255: Line;
    var other_1025: AntiScalar;

    self_1255 = self_1254;
    other_1025 = other_1024;
    let _e4: Line = self_1255;
    let _e6: AntiScalar = other_1025;
    return IdealPoint(((_e4.g1_ * vec3<f32>(_e6.g0_)) * vec3<f32>(-(1.0))));
}

fn line_anti_scalar_regressive_product(self_1256: Line, other_1026: AntiScalar) -> Line {
    var self_1257: Line;
    var other_1027: AntiScalar;

    self_1257 = self_1256;
    other_1027 = other_1026;
    let _e4: Line = self_1257;
    let _e6: AntiScalar = other_1027;
    let _e10: Line = self_1257;
    let _e12: AntiScalar = other_1027;
    return Line((_e4.g0_ * vec3<f32>(_e6.g0_)), (_e10.g1_ * vec3<f32>(_e12.g0_)));
}

fn line_anti_scalar_inner_product(self_1258: Line, other_1028: AntiScalar) -> IdealPoint {
    var self_1259: Line;
    var other_1029: AntiScalar;

    self_1259 = self_1258;
    other_1029 = other_1028;
    let _e4: Line = self_1259;
    let _e6: AntiScalar = other_1029;
    return IdealPoint(((_e4.g1_ * vec3<f32>(_e6.g0_)) * vec3<f32>(-(1.0))));
}

fn line_anti_scalar_geometric_anti_product(self_1260: Line, other_1030: AntiScalar) -> Line {
    var self_1261: Line;
    var other_1031: AntiScalar;

    self_1261 = self_1260;
    other_1031 = other_1030;
    let _e4: Line = self_1261;
    let _e6: AntiScalar = other_1031;
    let _e10: Line = self_1261;
    let _e12: AntiScalar = other_1031;
    return Line((_e4.g0_ * vec3<f32>(_e6.g0_)), (_e10.g1_ * vec3<f32>(_e12.g0_)));
}

fn line_anti_scalar_inner_anti_product(self_1262: Line, other_1032: AntiScalar) -> Line {
    var self_1263: Line;
    var other_1033: AntiScalar;

    self_1263 = self_1262;
    other_1033 = other_1032;
    let _e4: Line = self_1263;
    let _e6: AntiScalar = other_1033;
    let _e10: Line = self_1263;
    let _e12: AntiScalar = other_1033;
    return Line((_e4.g0_ * vec3<f32>(_e6.g0_)), (_e10.g1_ * vec3<f32>(_e12.g0_)));
}

fn line_anti_scalar_left_contraction(self_1264: Line, other_1034: AntiScalar) -> IdealPoint {
    var self_1265: Line;
    var other_1035: AntiScalar;

    self_1265 = self_1264;
    other_1035 = other_1034;
    let _e4: Line = self_1265;
    let _e6: AntiScalar = other_1035;
    return IdealPoint(((_e4.g1_ * vec3<f32>(_e6.g0_)) * vec3<f32>(-(1.0))));
}

fn line_anti_scalar_right_anti_contraction(self_1266: Line, other_1036: AntiScalar) -> Line {
    var self_1267: Line;
    var other_1037: AntiScalar;

    self_1267 = self_1266;
    other_1037 = other_1036;
    let _e4: Line = self_1267;
    let _e6: AntiScalar = other_1037;
    let _e10: Line = self_1267;
    let _e12: AntiScalar = other_1037;
    return Line((_e4.g0_ * vec3<f32>(_e6.g0_)), (_e10.g1_ * vec3<f32>(_e12.g0_)));
}

fn line_multi_vector_add(self_1268: Line, other_1038: MultiVector) -> MultiVector {
    var self_1269: Line;
    var other_1039: MultiVector;

    self_1269 = self_1268;
    other_1039 = other_1038;
    let _e4: Line = self_1269;
    let _e7: Line = self_1269;
    let _e10: Line = self_1269;
    let _e13: Line = self_1269;
    let _e23: MultiVector = other_1039;
    let _e26: MultiVector = other_1039;
    let _e28: MultiVector = other_1039;
    let _e30: Line = self_1269;
    let _e33: Line = self_1269;
    let _e36: Line = self_1269;
    let _e39: Line = self_1269;
    let _e49: MultiVector = other_1039;
    return MultiVector(((vec4<f32>(_e4.g0_.x, _e7.g1_.x, _e10.g1_.y, _e13.g1_.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0)) + _e23.g0_), _e26.g1_, _e28.g2_, ((vec4<f32>(_e30.g0_.x, _e33.g0_.x, _e36.g0_.y, _e39.g0_.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0)) + _e49.g3_));
}

fn line_multi_vector_sub(self_1270: Line, other_1040: MultiVector) -> MultiVector {
    var self_1271: Line;
    var other_1041: MultiVector;

    self_1271 = self_1270;
    other_1041 = other_1040;
    let _e4: Line = self_1271;
    let _e7: Line = self_1271;
    let _e10: Line = self_1271;
    let _e13: Line = self_1271;
    let _e23: MultiVector = other_1041;
    let _e28: MultiVector = other_1041;
    let _e33: MultiVector = other_1041;
    let _e36: Line = self_1271;
    let _e39: Line = self_1271;
    let _e42: Line = self_1271;
    let _e45: Line = self_1271;
    let _e55: MultiVector = other_1041;
    return MultiVector(((vec4<f32>(_e4.g0_.x, _e7.g1_.x, _e10.g1_.y, _e13.g1_.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0)) - _e23.g0_), (vec4<f32>(0.0) - _e28.g1_), (vec4<f32>(0.0) - _e33.g2_), ((vec4<f32>(_e36.g0_.x, _e39.g0_.x, _e42.g0_.y, _e45.g0_.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0)) - _e55.g3_));
}

fn line_multi_vector_geometric_product(self_1272: Line, other_1042: MultiVector) -> MultiVector {
    var self_1273: Line;
    var other_1043: MultiVector;

    self_1273 = self_1272;
    other_1043 = other_1042;
    let _e4: Line = self_1273;
    let _e8: MultiVector = other_1043;
    let _e20: Line = self_1273;
    let _e24: MultiVector = other_1043;
    let _e37: Line = self_1273;
    let _e41: MultiVector = other_1043;
    let _e54: Line = self_1273;
    let _e58: MultiVector = other_1043;
    let _e70: Line = self_1273;
    let _e74: MultiVector = other_1043;
    let _e87: Line = self_1273;
    let _e91: MultiVector = other_1043;
    let _e104: Line = self_1273;
    let _e108: MultiVector = other_1043;
    let _e121: Line = self_1273;
    let _e125: MultiVector = other_1043;
    let _e138: Line = self_1273;
    let _e142: MultiVector = other_1043;
    let _e155: Line = self_1273;
    let _e159: MultiVector = other_1043;
    let _e171: Line = self_1273;
    let _e175: MultiVector = other_1043;
    let _e188: Line = self_1273;
    let _e192: MultiVector = other_1043;
    let _e205: Line = self_1273;
    let _e209: MultiVector = other_1043;
    let _e220: Line = self_1273;
    let _e224: MultiVector = other_1043;
    let _e236: Line = self_1273;
    let _e240: MultiVector = other_1043;
    let _e252: Line = self_1273;
    let _e256: MultiVector = other_1043;
    let _e269: Line = self_1273;
    let _e273: MultiVector = other_1043;
    let _e286: Line = self_1273;
    let _e290: MultiVector = other_1043;
    return MultiVector(((((vec4<f32>(_e4.g1_.x) * _e8.g0_.yxwz) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0))) + ((vec4<f32>(_e20.g1_.y) * _e24.g0_.zwxy) * vec4<f32>(-(1.0), -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e37.g1_.z) * _e41.g0_.wzyx) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))), (((((((vec4<f32>(_e54.g0_.x) * _e58.g2_.yxwz) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0))) + ((vec4<f32>(_e70.g0_.y) * _e74.g2_.zwxy) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e87.g0_.z) * _e91.g2_.wzyx) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e104.g1_.x) * _e108.g1_.yxwz) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e121.g1_.y) * _e125.g1_.zwxy) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e138.g1_.z) * _e142.g1_.wzyx) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))), ((((vec4<f32>(_e155.g1_.x) * _e159.g2_.yxwz) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0))) + ((vec4<f32>(_e171.g1_.y) * _e175.g2_.zwxy) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e188.g1_.z) * _e192.g2_.wzyx) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))), (((((((vec4<f32>(_e205.g0_.x) * _e209.g0_.yxwz) * vec4<f32>(1.0, 1.0, 1.0, -(1.0))) + ((vec4<f32>(_e220.g0_.y) * _e224.g0_.zwxy) * vec4<f32>(1.0, -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e236.g0_.z) * _e240.g0_.wzyx) * vec4<f32>(1.0, 1.0, -(1.0), 1.0))) + ((vec4<f32>(_e252.g1_.x) * _e256.g3_.yxwz) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e269.g1_.y) * _e273.g3_.zwxy) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e286.g1_.z) * _e290.g3_.wzyx) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))));
}

fn line_multi_vector_geometric_anti_product(self_1274: Line, other_1044: MultiVector) -> MultiVector {
    var self_1275: Line;
    var other_1045: MultiVector;

    self_1275 = self_1274;
    other_1045 = other_1044;
    let _e4: Line = self_1275;
    let _e8: MultiVector = other_1045;
    let _e20: Line = self_1275;
    let _e24: MultiVector = other_1045;
    let _e37: Line = self_1275;
    let _e41: MultiVector = other_1045;
    let _e54: Line = self_1275;
    let _e58: MultiVector = other_1045;
    let _e70: Line = self_1275;
    let _e74: MultiVector = other_1045;
    let _e86: Line = self_1275;
    let _e90: MultiVector = other_1045;
    let _e102: Line = self_1275;
    let _e106: MultiVector = other_1045;
    let _e118: Line = self_1275;
    let _e122: MultiVector = other_1045;
    let _e135: Line = self_1275;
    let _e139: MultiVector = other_1045;
    let _e152: Line = self_1275;
    let _e156: MultiVector = other_1045;
    let _e168: Line = self_1275;
    let _e172: MultiVector = other_1045;
    let _e185: Line = self_1275;
    let _e189: MultiVector = other_1045;
    let _e202: Line = self_1275;
    let _e206: MultiVector = other_1045;
    let _e219: Line = self_1275;
    let _e223: MultiVector = other_1045;
    let _e236: Line = self_1275;
    let _e240: MultiVector = other_1045;
    let _e253: Line = self_1275;
    let _e257: MultiVector = other_1045;
    let _e269: Line = self_1275;
    let _e273: MultiVector = other_1045;
    let _e286: Line = self_1275;
    let _e290: MultiVector = other_1045;
    return MultiVector((((((((vec4<f32>(_e4.g0_.x) * _e8.g0_.yxwz) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0))) + ((vec4<f32>(_e20.g0_.y) * _e24.g0_.zwxy) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e37.g0_.z) * _e41.g0_.wzyx) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e54.g1_.x) * _e58.g3_.yxwz) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e70.g1_.y) * _e74.g3_.zwxy) * vec4<f32>(1.0, -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e86.g1_.z) * _e90.g3_.wzyx) * vec4<f32>(1.0, 1.0, -(1.0), 1.0))), ((((vec4<f32>(_e102.g0_.x) * _e106.g1_.yxwz) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0))) + ((vec4<f32>(_e118.g0_.y) * _e122.g1_.zwxy) * vec4<f32>(-(1.0), -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e135.g0_.z) * _e139.g1_.wzyx) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))), (((((((vec4<f32>(_e152.g0_.x) * _e156.g2_.yxwz) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0))) + ((vec4<f32>(_e168.g0_.y) * _e172.g2_.zwxy) * vec4<f32>(-(1.0), -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e185.g0_.z) * _e189.g2_.wzyx) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))) + ((vec4<f32>(_e202.g1_.x) * _e206.g1_.yxwz) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e219.g1_.y) * _e223.g1_.zwxy) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e236.g1_.z) * _e240.g1_.wzyx) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))), ((((vec4<f32>(_e253.g0_.x) * _e257.g3_.yxwz) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0))) + ((vec4<f32>(_e269.g0_.y) * _e273.g3_.zwxy) * vec4<f32>(-(1.0), -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e286.g0_.z) * _e290.g3_.wzyx) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))));
}

fn line_multi_vector_scalar_product(self_1276: Line, other_1046: MultiVector) -> Scalar {
    var self_1277: Line;
    var other_1047: MultiVector;

    self_1277 = self_1276;
    other_1047 = other_1046;
    let _e5: Line = self_1277;
    let _e8: MultiVector = other_1047;
    let _e13: Line = self_1277;
    let _e16: MultiVector = other_1047;
    let _e21: Line = self_1277;
    let _e24: MultiVector = other_1047;
    return Scalar((((0.0 - (_e5.g1_.x * _e8.g0_.y)) - (_e13.g1_.y * _e16.g0_.z)) - (_e21.g1_.z * _e24.g0_.w)));
}

fn line_multi_vector_anti_scalar_product(self_1278: Line, other_1048: MultiVector) -> AntiScalar {
    var self_1279: Line;
    var other_1049: MultiVector;

    self_1279 = self_1278;
    other_1049 = other_1048;
    let _e5: Line = self_1279;
    let _e8: MultiVector = other_1049;
    let _e13: Line = self_1279;
    let _e16: MultiVector = other_1049;
    let _e21: Line = self_1279;
    let _e24: MultiVector = other_1049;
    return AntiScalar((((0.0 - (_e5.g0_.x * _e8.g3_.y)) - (_e13.g0_.y * _e16.g3_.z)) - (_e21.g0_.z * _e24.g3_.w)));
}

fn line_rotor_geometric_product(self_1280: Line, other_1050: Rotor) -> Motor {
    var self_1281: Line;
    var other_1051: Rotor;

    self_1281 = self_1280;
    other_1051 = other_1050;
    let _e4: Line = self_1281;
    let _e8: Rotor = other_1051;
    let _e20: Line = self_1281;
    let _e24: Rotor = other_1051;
    let _e37: Line = self_1281;
    let _e41: Rotor = other_1051;
    let _e54: Line = self_1281;
    let _e58: Rotor = other_1051;
    let _e69: Line = self_1281;
    let _e73: Rotor = other_1051;
    let _e85: Line = self_1281;
    let _e89: Rotor = other_1051;
    return Motor(((((vec4<f32>(_e4.g1_.x) * _e8.g0_.yxwz) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0))) + ((vec4<f32>(_e20.g1_.y) * _e24.g0_.zwxy) * vec4<f32>(-(1.0), -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e37.g1_.z) * _e41.g0_.wzyx) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))), ((((vec4<f32>(_e54.g0_.x) * _e58.g0_.yxwz) * vec4<f32>(1.0, 1.0, 1.0, -(1.0))) + ((vec4<f32>(_e69.g0_.y) * _e73.g0_.zwxy) * vec4<f32>(1.0, -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e85.g0_.z) * _e89.g0_.wzyx) * vec4<f32>(1.0, 1.0, -(1.0), 1.0))));
}

fn line_rotor_regressive_product(self_1282: Line, other_1052: Rotor) -> Scalar {
    var self_1283: Line;
    var other_1053: Rotor;

    self_1283 = self_1282;
    other_1053 = other_1052;
    let _e4: Line = self_1283;
    let _e7: Rotor = other_1053;
    let _e11: Line = self_1283;
    let _e14: Rotor = other_1053;
    let _e19: Line = self_1283;
    let _e22: Rotor = other_1053;
    return Scalar((((_e4.g0_.x * _e7.g0_.y) + (_e11.g0_.y * _e14.g0_.z)) + (_e19.g0_.z * _e22.g0_.w)));
}

fn line_rotor_geometric_anti_product(self_1284: Line, other_1054: Rotor) -> Rotor {
    var self_1285: Line;
    var other_1055: Rotor;

    self_1285 = self_1284;
    other_1055 = other_1054;
    let _e4: Line = self_1285;
    let _e8: Rotor = other_1055;
    let _e20: Line = self_1285;
    let _e24: Rotor = other_1055;
    let _e37: Line = self_1285;
    let _e41: Rotor = other_1055;
    return Rotor(((((vec4<f32>(_e4.g0_.x) * _e8.g0_.yxwz) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0))) + ((vec4<f32>(_e20.g0_.y) * _e24.g0_.zwxy) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e37.g0_.z) * _e41.g0_.wzyx) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))));
}

fn line_rotor_left_contraction(self_1286: Line, other_1056: Rotor) -> Scalar {
    var self_1287: Line;
    var other_1057: Rotor;

    self_1287 = self_1286;
    other_1057 = other_1056;
    let _e5: Line = self_1287;
    let _e8: Rotor = other_1057;
    let _e13: Line = self_1287;
    let _e16: Rotor = other_1057;
    let _e21: Line = self_1287;
    let _e24: Rotor = other_1057;
    return Scalar((((0.0 - (_e5.g1_.x * _e8.g0_.y)) - (_e13.g1_.y * _e16.g0_.z)) - (_e21.g1_.z * _e24.g0_.w)));
}

fn line_rotor_scalar_product(self_1288: Line, other_1058: Rotor) -> Scalar {
    var self_1289: Line;
    var other_1059: Rotor;

    self_1289 = self_1288;
    other_1059 = other_1058;
    let _e5: Line = self_1289;
    let _e8: Rotor = other_1059;
    let _e13: Line = self_1289;
    let _e16: Rotor = other_1059;
    let _e21: Line = self_1289;
    let _e24: Rotor = other_1059;
    return Scalar((((0.0 - (_e5.g1_.x * _e8.g0_.y)) - (_e13.g1_.y * _e16.g0_.z)) - (_e21.g1_.z * _e24.g0_.w)));
}

fn line_point_regressive_product(self_1290: Line, other_1060: Point) -> Plane {
    var self_1291: Line;
    var other_1061: Point;

    self_1291 = self_1290;
    other_1061 = other_1060;
    let _e4: Line = self_1291;
    let _e8: Point = other_1061;
    let _e19: Line = self_1291;
    let _e23: Point = other_1061;
    let _e35: Line = self_1291;
    let _e39: Point = other_1061;
    let _e51: Line = self_1291;
    let _e55: Point = other_1061;
    let _e67: Line = self_1291;
    let _e70: Line = self_1291;
    let _e73: Line = self_1291;
    let _e76: Line = self_1291;
    let _e80: Point = other_1061;
    return Plane(((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.zzxz) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0)) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.wwwx) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))) + ((vec4<f32>(_e35.g1_.y) * _e39.g0_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0)))) + ((vec4<f32>(_e51.g1_.z) * _e55.g0_.zzyz) * vec4<f32>(0.0, -(1.0), 1.0, 0.0))) + ((vec4<f32>(_e67.g0_.x, _e70.g0_.x, _e73.g1_.x, _e76.g1_.x) * _e80.g0_.yxwz) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))));
}

fn line_point_inner_product(self_1292: Line, other_1062: Point) -> Plane {
    var self_1293: Line;
    var other_1063: Point;

    self_1293 = self_1292;
    other_1063 = other_1062;
    let _e4: Line = self_1293;
    let _e8: Point = other_1063;
    let _e19: Line = self_1293;
    let _e23: Point = other_1063;
    let _e35: Line = self_1293;
    let _e39: Point = other_1063;
    return Plane(((((vec4<f32>(_e4.g1_.y) * _e8.g0_.zzxz) * vec4<f32>(1.0, 0.0, -(1.0), 0.0)) + ((vec4<f32>(_e19.g1_.z) * _e23.g0_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e35.g1_.x) * _e39.g0_.yxxx) * vec4<f32>(1.0, -(1.0), 0.0, 0.0))));
}

fn line_point_geometric_anti_product(self_1294: Line, other_1064: Point) -> PointAndPlane {
    var self_1295: Line;
    var other_1065: Point;

    self_1295 = self_1294;
    other_1065 = other_1064;
    let _e4: Line = self_1295;
    let _e8: Point = other_1065;
    let _e19: Line = self_1295;
    let _e23: Point = other_1065;
    let _e35: Line = self_1295;
    let _e39: Point = other_1065;
    let _e51: Line = self_1295;
    let _e55: Point = other_1065;
    let _e67: Line = self_1295;
    let _e70: Line = self_1295;
    let _e73: Line = self_1295;
    let _e76: Line = self_1295;
    let _e80: Point = other_1065;
    let _e92: Line = self_1295;
    let _e96: Point = other_1065;
    let _e107: Line = self_1295;
    let _e111: Point = other_1065;
    let _e123: Line = self_1295;
    let _e127: Point = other_1065;
    let _e139: Line = self_1295;
    let _e143: Point = other_1065;
    let _e155: Line = self_1295;
    let _e158: Line = self_1295;
    let _e161: Line = self_1295;
    let _e164: Line = self_1295;
    let _e168: Point = other_1065;
    return PointAndPlane(((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.wwwy) * vec4<f32>(0.0, -(1.0), 0.0, 1.0)) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.zzyz) * vec4<f32>(0.0, 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e35.g1_.y) * vec4<f32>(_e39.g0_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e51.g1_.z) * vec4<f32>(_e55.g0_.w)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e67.g1_.x, _e70.g0_.x, _e73.g0_.x, _e76.g0_.x) * _e80.g0_.yxwz) * vec4<f32>(1.0, 0.0, 1.0, -(1.0)))), ((((((vec4<f32>(_e92.g0_.y) * _e96.g0_.zzxz) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0)) + ((vec4<f32>(_e107.g0_.z) * _e111.g0_.wwwx) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))) + ((vec4<f32>(_e123.g1_.y) * _e127.g0_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0)))) + ((vec4<f32>(_e139.g1_.z) * _e143.g0_.zzyz) * vec4<f32>(0.0, -(1.0), 1.0, 0.0))) + ((vec4<f32>(_e155.g0_.x, _e158.g0_.x, _e161.g1_.x, _e164.g1_.x) * _e168.g0_.yxwz) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))));
}

fn line_point_inner_anti_product(self_1296: Line, other_1066: Point) -> Point {
    var self_1297: Line;
    var other_1067: Point;

    self_1297 = self_1296;
    other_1067 = other_1066;
    let _e4: Line = self_1297;
    let _e8: Point = other_1067;
    let _e19: Line = self_1297;
    let _e23: Point = other_1067;
    let _e35: Line = self_1297;
    let _e39: Point = other_1067;
    let _e51: Line = self_1297;
    let _e55: Point = other_1067;
    let _e67: Line = self_1297;
    let _e70: Line = self_1297;
    let _e73: Line = self_1297;
    let _e76: Line = self_1297;
    let _e80: Point = other_1067;
    return Point(((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.wwwy) * vec4<f32>(0.0, -(1.0), 0.0, 1.0)) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.zzyz) * vec4<f32>(0.0, 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e35.g1_.y) * vec4<f32>(_e39.g0_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e51.g1_.z) * vec4<f32>(_e55.g0_.w)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e67.g1_.x, _e70.g0_.x, _e73.g0_.x, _e76.g0_.x) * _e80.g0_.yxwz) * vec4<f32>(1.0, 0.0, 1.0, -(1.0)))));
}

fn line_point_left_contraction(self_1298: Line, other_1068: Point) -> Plane {
    var self_1299: Line;
    var other_1069: Point;

    self_1299 = self_1298;
    other_1069 = other_1068;
    let _e4: Line = self_1299;
    let _e8: Point = other_1069;
    let _e19: Line = self_1299;
    let _e23: Point = other_1069;
    let _e35: Line = self_1299;
    let _e39: Point = other_1069;
    return Plane(((((vec4<f32>(_e4.g1_.y) * _e8.g0_.zzxz) * vec4<f32>(1.0, 0.0, -(1.0), 0.0)) + ((vec4<f32>(_e19.g1_.z) * _e23.g0_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e35.g1_.x) * _e39.g0_.yxxx) * vec4<f32>(1.0, -(1.0), 0.0, 0.0))));
}

fn line_point_right_anti_contraction(self_1300: Line, other_1070: Point) -> Point {
    var self_1301: Line;
    var other_1071: Point;

    self_1301 = self_1300;
    other_1071 = other_1070;
    let _e4: Line = self_1301;
    let _e8: Point = other_1071;
    let _e19: Line = self_1301;
    let _e23: Point = other_1071;
    let _e35: Line = self_1301;
    let _e39: Point = other_1071;
    let _e51: Line = self_1301;
    let _e55: Point = other_1071;
    let _e67: Line = self_1301;
    let _e70: Line = self_1301;
    let _e73: Line = self_1301;
    let _e76: Line = self_1301;
    let _e80: Point = other_1071;
    return Point(((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.wwwy) * vec4<f32>(0.0, -(1.0), 0.0, 1.0)) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.zzyz) * vec4<f32>(0.0, 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e35.g1_.y) * vec4<f32>(_e39.g0_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e51.g1_.z) * vec4<f32>(_e55.g0_.w)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e67.g1_.x, _e70.g0_.x, _e73.g0_.x, _e76.g0_.x) * _e80.g0_.yxwz) * vec4<f32>(1.0, 0.0, 1.0, -(1.0)))));
}

fn line_ideal_point_into(self_1302: Line) -> IdealPoint {
    var self_1303: Line;

    self_1303 = self_1302;
    let _e2: Line = self_1303;
    return IdealPoint(_e2.g0_);
}

fn line_ideal_point_add(self_1304: Line, other_1072: IdealPoint) -> Line {
    var self_1305: Line;
    var other_1073: IdealPoint;

    self_1305 = self_1304;
    other_1073 = other_1072;
    let _e4: Line = self_1305;
    let _e6: IdealPoint = other_1073;
    let _e9: Line = self_1305;
    return Line((_e4.g0_ + _e6.g0_), _e9.g1_);
}

fn line_ideal_point_sub(self_1306: Line, other_1074: IdealPoint) -> Line {
    var self_1307: Line;
    var other_1075: IdealPoint;

    self_1307 = self_1306;
    other_1075 = other_1074;
    let _e4: Line = self_1307;
    let _e6: IdealPoint = other_1075;
    let _e9: Line = self_1307;
    return Line((_e4.g0_ - _e6.g0_), _e9.g1_);
}

fn line_ideal_point_regressive_product(self_1308: Line, other_1076: IdealPoint) -> Scalar {
    var self_1309: Line;
    var other_1077: IdealPoint;

    self_1309 = self_1308;
    other_1077 = other_1076;
    let _e4: Line = self_1309;
    let _e7: IdealPoint = other_1077;
    let _e11: Line = self_1309;
    let _e14: IdealPoint = other_1077;
    let _e19: Line = self_1309;
    let _e22: IdealPoint = other_1077;
    return Scalar((((_e4.g1_.x * _e7.g0_.x) + (_e11.g1_.y * _e14.g0_.y)) + (_e19.g1_.z * _e22.g0_.z)));
}

fn line_ideal_point_outer_product(self_1310: Line, other_1078: IdealPoint) -> AntiScalar {
    var self_1311: Line;
    var other_1079: IdealPoint;

    self_1311 = self_1310;
    other_1079 = other_1078;
    let _e4: Line = self_1311;
    let _e7: IdealPoint = other_1079;
    let _e11: Line = self_1311;
    let _e14: IdealPoint = other_1079;
    let _e19: Line = self_1311;
    let _e22: IdealPoint = other_1079;
    return AntiScalar((((_e4.g1_.x * _e7.g0_.x) + (_e11.g1_.y * _e14.g0_.y)) + (_e19.g1_.z * _e22.g0_.z)));
}

fn line_ideal_point_geometric_anti_product(self_1312: Line, other_1080: IdealPoint) -> Motor {
    var self_1313: Line;
    var other_1081: IdealPoint;

    self_1313 = self_1312;
    other_1081 = other_1080;
    let _e4: Line = self_1313;
    let _e8: IdealPoint = other_1081;
    let _e11: IdealPoint = other_1081;
    let _e14: IdealPoint = other_1081;
    let _e17: IdealPoint = other_1081;
    let _e29: Line = self_1313;
    let _e33: IdealPoint = other_1081;
    let _e36: IdealPoint = other_1081;
    let _e39: IdealPoint = other_1081;
    let _e42: IdealPoint = other_1081;
    let _e55: Line = self_1313;
    let _e59: IdealPoint = other_1081;
    let _e62: IdealPoint = other_1081;
    let _e65: IdealPoint = other_1081;
    let _e68: IdealPoint = other_1081;
    let _e81: Line = self_1313;
    let _e85: IdealPoint = other_1081;
    let _e88: IdealPoint = other_1081;
    let _e91: IdealPoint = other_1081;
    let _e94: IdealPoint = other_1081;
    let _e107: Line = self_1313;
    let _e111: IdealPoint = other_1081;
    let _e114: IdealPoint = other_1081;
    let _e117: IdealPoint = other_1081;
    let _e120: IdealPoint = other_1081;
    let _e134: Line = self_1313;
    let _e138: IdealPoint = other_1081;
    let _e141: IdealPoint = other_1081;
    let _e144: IdealPoint = other_1081;
    let _e147: IdealPoint = other_1081;
    return Motor(((((vec4<f32>(_e4.g1_.y) * vec4<f32>(_e8.g0_.y, _e11.g0_.z, _e14.g0_.y, _e17.g0_.x)) * vec4<f32>(1.0, -(1.0), 0.0, 1.0)) + ((vec4<f32>(_e29.g1_.z) * vec4<f32>(_e33.g0_.z, _e36.g0_.y, _e39.g0_.x, _e42.g0_.z)) * vec4<f32>(1.0, 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e55.g1_.x) * vec4<f32>(_e59.g0_.x, _e62.g0_.x, _e65.g0_.z, _e68.g0_.y)) * vec4<f32>(1.0, 0.0, 1.0, -(1.0)))), ((((vec4<f32>(_e81.g0_.y) * vec4<f32>(_e85.g0_.y, _e88.g0_.z, _e91.g0_.y, _e94.g0_.x)) * vec4<f32>(-(1.0), -(1.0), 0.0, 1.0)) + ((vec4<f32>(_e107.g0_.z) * vec4<f32>(_e111.g0_.z, _e114.g0_.y, _e117.g0_.x, _e120.g0_.z)) * vec4<f32>(-(1.0), 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e134.g0_.x) * vec4<f32>(_e138.g0_.x, _e141.g0_.x, _e144.g0_.z, _e147.g0_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0)))));
}

fn line_ideal_point_inner_anti_product(self_1314: Line, other_1082: IdealPoint) -> AntiScalar {
    var self_1315: Line;
    var other_1083: IdealPoint;

    self_1315 = self_1314;
    other_1083 = other_1082;
    let _e5: Line = self_1315;
    let _e8: IdealPoint = other_1083;
    let _e13: Line = self_1315;
    let _e16: IdealPoint = other_1083;
    let _e21: Line = self_1315;
    let _e24: IdealPoint = other_1083;
    return AntiScalar((((0.0 - (_e5.g0_.x * _e8.g0_.x)) - (_e13.g0_.y * _e16.g0_.y)) - (_e21.g0_.z * _e24.g0_.z)));
}

fn line_ideal_point_left_anti_contraction(self_1316: Line, other_1084: IdealPoint) -> AntiScalar {
    var self_1317: Line;
    var other_1085: IdealPoint;

    self_1317 = self_1316;
    other_1085 = other_1084;
    let _e5: Line = self_1317;
    let _e8: IdealPoint = other_1085;
    let _e13: Line = self_1317;
    let _e16: IdealPoint = other_1085;
    let _e21: Line = self_1317;
    let _e24: IdealPoint = other_1085;
    return AntiScalar((((0.0 - (_e5.g0_.x * _e8.g0_.x)) - (_e13.g0_.y * _e16.g0_.y)) - (_e21.g0_.z * _e24.g0_.z)));
}

fn line_ideal_point_right_anti_contraction(self_1318: Line, other_1086: IdealPoint) -> AntiScalar {
    var self_1319: Line;
    var other_1087: IdealPoint;

    self_1319 = self_1318;
    other_1087 = other_1086;
    let _e5: Line = self_1319;
    let _e8: IdealPoint = other_1087;
    let _e13: Line = self_1319;
    let _e16: IdealPoint = other_1087;
    let _e21: Line = self_1319;
    let _e24: IdealPoint = other_1087;
    return AntiScalar((((0.0 - (_e5.g0_.x * _e8.g0_.x)) - (_e13.g0_.y * _e16.g0_.y)) - (_e21.g0_.z * _e24.g0_.z)));
}

fn line_ideal_point_anti_scalar_product(self_1320: Line, other_1088: IdealPoint) -> AntiScalar {
    var self_1321: Line;
    var other_1089: IdealPoint;

    self_1321 = self_1320;
    other_1089 = other_1088;
    let _e5: Line = self_1321;
    let _e8: IdealPoint = other_1089;
    let _e13: Line = self_1321;
    let _e16: IdealPoint = other_1089;
    let _e21: Line = self_1321;
    let _e24: IdealPoint = other_1089;
    return AntiScalar((((0.0 - (_e5.g0_.x * _e8.g0_.x)) - (_e13.g0_.y * _e16.g0_.y)) - (_e21.g0_.z * _e24.g0_.z)));
}

fn line_plane_geometric_product(self_1322: Line, other_1090: Plane) -> PointAndPlane {
    var self_1323: Line;
    var other_1091: Plane;

    self_1323 = self_1322;
    other_1091 = other_1090;
    let _e4: Line = self_1323;
    let _e8: Plane = other_1091;
    let _e19: Line = self_1323;
    let _e23: Plane = other_1091;
    let _e35: Line = self_1323;
    let _e39: Plane = other_1091;
    let _e51: Line = self_1323;
    let _e55: Plane = other_1091;
    let _e67: Line = self_1323;
    let _e70: Line = self_1323;
    let _e73: Line = self_1323;
    let _e76: Line = self_1323;
    let _e80: Plane = other_1091;
    let _e93: Line = self_1323;
    let _e97: Plane = other_1091;
    let _e108: Line = self_1323;
    let _e112: Plane = other_1091;
    let _e124: Line = self_1323;
    let _e128: Plane = other_1091;
    let _e140: Line = self_1323;
    let _e144: Plane = other_1091;
    let _e156: Line = self_1323;
    let _e159: Line = self_1323;
    let _e162: Line = self_1323;
    let _e165: Line = self_1323;
    let _e169: Plane = other_1091;
    return PointAndPlane(((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.wwwy) * vec4<f32>(0.0, -(1.0), 0.0, 1.0)) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.zzyz) * vec4<f32>(0.0, 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e35.g1_.y) * _e39.g0_.zzxz) * vec4<f32>(1.0, 0.0, -(1.0), 0.0))) + ((vec4<f32>(_e51.g1_.z) * _e55.g0_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e67.g1_.x, _e70.g1_.x, _e73.g0_.x, _e76.g0_.x) * _e80.g0_.yxwz) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))), ((((((vec4<f32>(_e93.g0_.y) * vec4<f32>(_e97.g0_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) + ((vec4<f32>(_e108.g0_.z) * vec4<f32>(_e112.g0_.w)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e124.g1_.y) * _e128.g0_.wwwy) * vec4<f32>(0.0, -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e140.g1_.z) * _e144.g0_.zzyz) * vec4<f32>(0.0, 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e156.g0_.x, _e159.g0_.x, _e162.g1_.x, _e165.g1_.x) * _e169.g0_.yxwz) * vec4<f32>(1.0, 0.0, 1.0, -(1.0)))));
}

fn line_plane_outer_product(self_1324: Line, other_1092: Plane) -> Point {
    var self_1325: Line;
    var other_1093: Plane;

    self_1325 = self_1324;
    other_1093 = other_1092;
    let _e4: Line = self_1325;
    let _e8: Plane = other_1093;
    let _e19: Line = self_1325;
    let _e23: Plane = other_1093;
    let _e35: Line = self_1325;
    let _e39: Plane = other_1093;
    let _e51: Line = self_1325;
    let _e55: Plane = other_1093;
    let _e67: Line = self_1325;
    let _e70: Line = self_1325;
    let _e73: Line = self_1325;
    let _e76: Line = self_1325;
    let _e80: Plane = other_1093;
    return Point(((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.wwwy) * vec4<f32>(0.0, -(1.0), 0.0, 1.0)) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.zzyz) * vec4<f32>(0.0, 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e35.g1_.y) * _e39.g0_.zzxz) * vec4<f32>(1.0, 0.0, -(1.0), 0.0))) + ((vec4<f32>(_e51.g1_.z) * _e55.g0_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e67.g1_.x, _e70.g1_.x, _e73.g0_.x, _e76.g0_.x) * _e80.g0_.yxwz) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))));
}

fn line_plane_inner_product(self_1326: Line, other_1094: Plane) -> Plane {
    var self_1327: Line;
    var other_1095: Plane;

    self_1327 = self_1326;
    other_1095 = other_1094;
    let _e4: Line = self_1327;
    let _e8: Plane = other_1095;
    let _e19: Line = self_1327;
    let _e23: Plane = other_1095;
    let _e35: Line = self_1327;
    let _e39: Plane = other_1095;
    let _e51: Line = self_1327;
    let _e55: Plane = other_1095;
    let _e67: Line = self_1327;
    let _e70: Line = self_1327;
    let _e73: Line = self_1327;
    let _e76: Line = self_1327;
    let _e80: Plane = other_1095;
    return Plane(((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) + ((vec4<f32>(_e19.g0_.z) * vec4<f32>(_e23.g0_.w)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e35.g1_.y) * _e39.g0_.wwwy) * vec4<f32>(0.0, -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e51.g1_.z) * _e55.g0_.zzyz) * vec4<f32>(0.0, 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e67.g0_.x, _e70.g0_.x, _e73.g1_.x, _e76.g1_.x) * _e80.g0_.yxwz) * vec4<f32>(1.0, 0.0, 1.0, -(1.0)))));
}

fn line_plane_inner_anti_product(self_1328: Line, other_1096: Plane) -> Point {
    var self_1329: Line;
    var other_1097: Plane;

    self_1329 = self_1328;
    other_1097 = other_1096;
    let _e4: Line = self_1329;
    let _e8: Plane = other_1097;
    let _e19: Line = self_1329;
    let _e23: Plane = other_1097;
    let _e35: Line = self_1329;
    let _e39: Plane = other_1097;
    return Point(((((vec4<f32>(_e4.g0_.y) * _e8.g0_.zzxz) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0)) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.wwwx) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))) + ((vec4<f32>(_e35.g0_.x) * _e39.g0_.yxxx) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))));
}

fn line_plane_right_contraction(self_1330: Line, other_1098: Plane) -> Plane {
    var self_1331: Line;
    var other_1099: Plane;

    self_1331 = self_1330;
    other_1099 = other_1098;
    let _e4: Line = self_1331;
    let _e8: Plane = other_1099;
    let _e19: Line = self_1331;
    let _e23: Plane = other_1099;
    let _e35: Line = self_1331;
    let _e39: Plane = other_1099;
    let _e51: Line = self_1331;
    let _e55: Plane = other_1099;
    let _e67: Line = self_1331;
    let _e70: Line = self_1331;
    let _e73: Line = self_1331;
    let _e76: Line = self_1331;
    let _e80: Plane = other_1099;
    return Plane(((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) + ((vec4<f32>(_e19.g0_.z) * vec4<f32>(_e23.g0_.w)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e35.g1_.y) * _e39.g0_.wwwy) * vec4<f32>(0.0, -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e51.g1_.z) * _e55.g0_.zzyz) * vec4<f32>(0.0, 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e67.g0_.x, _e70.g0_.x, _e73.g1_.x, _e76.g1_.x) * _e80.g0_.yxwz) * vec4<f32>(1.0, 0.0, 1.0, -(1.0)))));
}

fn line_plane_left_anti_contraction(self_1332: Line, other_1100: Plane) -> Point {
    var self_1333: Line;
    var other_1101: Plane;

    self_1333 = self_1332;
    other_1101 = other_1100;
    let _e4: Line = self_1333;
    let _e8: Plane = other_1101;
    let _e19: Line = self_1333;
    let _e23: Plane = other_1101;
    let _e35: Line = self_1333;
    let _e39: Plane = other_1101;
    return Point(((((vec4<f32>(_e4.g0_.y) * _e8.g0_.zzxz) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0)) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.wwwx) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))) + ((vec4<f32>(_e35.g0_.x) * _e39.g0_.yxxx) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))));
}

fn line_line_add(self_1334: Line, other_1102: Line) -> Line {
    var self_1335: Line;
    var other_1103: Line;

    self_1335 = self_1334;
    other_1103 = other_1102;
    let _e4: Line = self_1335;
    let _e6: Line = other_1103;
    let _e9: Line = self_1335;
    let _e11: Line = other_1103;
    return Line((_e4.g0_ + _e6.g0_), (_e9.g1_ + _e11.g1_));
}

fn line_line_sub(self_1336: Line, other_1104: Line) -> Line {
    var self_1337: Line;
    var other_1105: Line;

    self_1337 = self_1336;
    other_1105 = other_1104;
    let _e4: Line = self_1337;
    let _e6: Line = other_1105;
    let _e9: Line = self_1337;
    let _e11: Line = other_1105;
    return Line((_e4.g0_ - _e6.g0_), (_e9.g1_ - _e11.g1_));
}

fn line_line_mul(self_1338: Line, other_1106: Line) -> Line {
    var self_1339: Line;
    var other_1107: Line;

    self_1339 = self_1338;
    other_1107 = other_1106;
    let _e4: Line = self_1339;
    let _e6: Line = other_1107;
    let _e9: Line = self_1339;
    let _e11: Line = other_1107;
    return Line((_e4.g0_ * _e6.g0_), (_e9.g1_ * _e11.g1_));
}

fn line_line_div(self_1340: Line, other_1108: Line) -> Line {
    var self_1341: Line;
    var other_1109: Line;

    self_1341 = self_1340;
    other_1109 = other_1108;
    let _e4: Line = self_1341;
    let _e7: Line = self_1341;
    let _e10: Line = self_1341;
    let _e19: Line = other_1109;
    let _e22: Line = other_1109;
    let _e25: Line = other_1109;
    let _e35: Line = self_1341;
    let _e38: Line = self_1341;
    let _e41: Line = self_1341;
    let _e50: Line = other_1109;
    let _e53: Line = other_1109;
    let _e56: Line = other_1109;
    return Line((((vec3<f32>(_e4.g0_.x, _e7.g0_.y, _e10.g0_.z) * vec3<f32>(1.0, 1.0, 1.0)) / vec3<f32>(_e19.g0_.x, _e22.g0_.y, _e25.g0_.z)) * vec3<f32>(1.0, 1.0, 1.0)), (((vec3<f32>(_e35.g1_.x, _e38.g1_.y, _e41.g1_.z) * vec3<f32>(1.0, 1.0, 1.0)) / vec3<f32>(_e50.g1_.x, _e53.g1_.y, _e56.g1_.z)) * vec3<f32>(1.0, 1.0, 1.0)));
}

fn line_line_geometric_product(self_1342: Line, other_1110: Line) -> Motor {
    var self_1343: Line;
    var other_1111: Line;

    self_1343 = self_1342;
    other_1111 = other_1110;
    let _e4: Line = self_1343;
    let _e8: Line = other_1111;
    let _e11: Line = other_1111;
    let _e14: Line = other_1111;
    let _e17: Line = other_1111;
    let _e30: Line = self_1343;
    let _e34: Line = other_1111;
    let _e37: Line = other_1111;
    let _e40: Line = other_1111;
    let _e43: Line = other_1111;
    let _e57: Line = self_1343;
    let _e61: Line = other_1111;
    let _e64: Line = other_1111;
    let _e67: Line = other_1111;
    let _e70: Line = other_1111;
    let _e84: Line = self_1343;
    let _e88: Line = other_1111;
    let _e91: Line = other_1111;
    let _e94: Line = other_1111;
    let _e97: Line = other_1111;
    let _e109: Line = self_1343;
    let _e113: Line = other_1111;
    let _e116: Line = other_1111;
    let _e119: Line = other_1111;
    let _e122: Line = other_1111;
    let _e135: Line = self_1343;
    let _e139: Line = other_1111;
    let _e142: Line = other_1111;
    let _e145: Line = other_1111;
    let _e148: Line = other_1111;
    let _e161: Line = self_1343;
    let _e165: Line = other_1111;
    let _e168: Line = other_1111;
    let _e171: Line = other_1111;
    let _e174: Line = other_1111;
    let _e187: Line = self_1343;
    let _e191: Line = other_1111;
    let _e194: Line = other_1111;
    let _e197: Line = other_1111;
    let _e200: Line = other_1111;
    let _e213: Line = self_1343;
    let _e217: Line = other_1111;
    let _e220: Line = other_1111;
    let _e223: Line = other_1111;
    let _e226: Line = other_1111;
    return Motor(((((vec4<f32>(_e4.g1_.y) * vec4<f32>(_e8.g1_.y, _e11.g1_.z, _e14.g1_.y, _e17.g1_.x)) * vec4<f32>(-(1.0), -(1.0), 0.0, 1.0)) + ((vec4<f32>(_e30.g1_.z) * vec4<f32>(_e34.g1_.z, _e37.g1_.y, _e40.g1_.x, _e43.g1_.z)) * vec4<f32>(-(1.0), 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e57.g1_.x) * vec4<f32>(_e61.g1_.x, _e64.g1_.x, _e67.g1_.z, _e70.g1_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0)))), (((((((vec4<f32>(_e84.g0_.y) * vec4<f32>(_e88.g1_.y, _e91.g1_.z, _e94.g1_.y, _e97.g1_.x)) * vec4<f32>(1.0, -(1.0), 0.0, 1.0)) + ((vec4<f32>(_e109.g0_.z) * vec4<f32>(_e113.g1_.z, _e116.g1_.y, _e119.g1_.x, _e122.g1_.z)) * vec4<f32>(1.0, 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e135.g1_.x) * vec4<f32>(_e139.g0_.x, _e142.g0_.x, _e145.g0_.z, _e148.g0_.y)) * vec4<f32>(1.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e161.g1_.y) * vec4<f32>(_e165.g0_.y, _e168.g0_.z, _e171.g0_.y, _e174.g0_.x)) * vec4<f32>(1.0, -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e187.g1_.z) * vec4<f32>(_e191.g0_.z, _e194.g0_.y, _e197.g0_.x, _e200.g0_.z)) * vec4<f32>(1.0, 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e213.g0_.x) * vec4<f32>(_e217.g1_.x, _e220.g1_.x, _e223.g1_.z, _e226.g1_.y)) * vec4<f32>(1.0, 0.0, 1.0, -(1.0)))));
}

fn line_line_regressive_product(self_1344: Line, other_1112: Line) -> Scalar {
    var self_1345: Line;
    var other_1113: Line;

    self_1345 = self_1344;
    other_1113 = other_1112;
    let _e4: Line = self_1345;
    let _e7: Line = other_1113;
    let _e11: Line = self_1345;
    let _e14: Line = other_1113;
    let _e19: Line = self_1345;
    let _e22: Line = other_1113;
    let _e27: Line = self_1345;
    let _e30: Line = other_1113;
    let _e35: Line = self_1345;
    let _e38: Line = other_1113;
    let _e43: Line = self_1345;
    let _e46: Line = other_1113;
    return Scalar(((((((_e4.g0_.x * _e7.g1_.x) + (_e11.g0_.y * _e14.g1_.y)) + (_e19.g0_.z * _e22.g1_.z)) + (_e27.g1_.x * _e30.g0_.x)) + (_e35.g1_.y * _e38.g0_.y)) + (_e43.g1_.z * _e46.g0_.z)));
}

fn line_line_outer_product(self_1346: Line, other_1114: Line) -> AntiScalar {
    var self_1347: Line;
    var other_1115: Line;

    self_1347 = self_1346;
    other_1115 = other_1114;
    let _e4: Line = self_1347;
    let _e7: Line = other_1115;
    let _e11: Line = self_1347;
    let _e14: Line = other_1115;
    let _e19: Line = self_1347;
    let _e22: Line = other_1115;
    let _e27: Line = self_1347;
    let _e30: Line = other_1115;
    let _e35: Line = self_1347;
    let _e38: Line = other_1115;
    let _e43: Line = self_1347;
    let _e46: Line = other_1115;
    return AntiScalar(((((((_e4.g0_.x * _e7.g1_.x) + (_e11.g0_.y * _e14.g1_.y)) + (_e19.g0_.z * _e22.g1_.z)) + (_e27.g1_.x * _e30.g0_.x)) + (_e35.g1_.y * _e38.g0_.y)) + (_e43.g1_.z * _e46.g0_.z)));
}

fn line_line_inner_product(self_1348: Line, other_1116: Line) -> Scalar {
    var self_1349: Line;
    var other_1117: Line;

    self_1349 = self_1348;
    other_1117 = other_1116;
    let _e5: Line = self_1349;
    let _e8: Line = other_1117;
    let _e13: Line = self_1349;
    let _e16: Line = other_1117;
    let _e21: Line = self_1349;
    let _e24: Line = other_1117;
    return Scalar((((0.0 - (_e5.g1_.x * _e8.g1_.x)) - (_e13.g1_.y * _e16.g1_.y)) - (_e21.g1_.z * _e24.g1_.z)));
}

fn line_line_geometric_anti_product(self_1350: Line, other_1118: Line) -> Motor {
    var self_1351: Line;
    var other_1119: Line;

    self_1351 = self_1350;
    other_1119 = other_1118;
    let _e4: Line = self_1351;
    let _e8: Line = other_1119;
    let _e11: Line = other_1119;
    let _e14: Line = other_1119;
    let _e17: Line = other_1119;
    let _e29: Line = self_1351;
    let _e33: Line = other_1119;
    let _e36: Line = other_1119;
    let _e39: Line = other_1119;
    let _e42: Line = other_1119;
    let _e55: Line = self_1351;
    let _e59: Line = other_1119;
    let _e62: Line = other_1119;
    let _e65: Line = other_1119;
    let _e68: Line = other_1119;
    let _e81: Line = self_1351;
    let _e85: Line = other_1119;
    let _e88: Line = other_1119;
    let _e91: Line = other_1119;
    let _e94: Line = other_1119;
    let _e107: Line = self_1351;
    let _e111: Line = other_1119;
    let _e114: Line = other_1119;
    let _e117: Line = other_1119;
    let _e120: Line = other_1119;
    let _e133: Line = self_1351;
    let _e137: Line = other_1119;
    let _e140: Line = other_1119;
    let _e143: Line = other_1119;
    let _e146: Line = other_1119;
    let _e159: Line = self_1351;
    let _e163: Line = other_1119;
    let _e166: Line = other_1119;
    let _e169: Line = other_1119;
    let _e172: Line = other_1119;
    let _e185: Line = self_1351;
    let _e189: Line = other_1119;
    let _e192: Line = other_1119;
    let _e195: Line = other_1119;
    let _e198: Line = other_1119;
    let _e212: Line = self_1351;
    let _e216: Line = other_1119;
    let _e219: Line = other_1119;
    let _e222: Line = other_1119;
    let _e225: Line = other_1119;
    return Motor((((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g1_.y, _e11.g1_.z, _e14.g1_.y, _e17.g1_.x)) * vec4<f32>(1.0, -(1.0), 0.0, 1.0)) + ((vec4<f32>(_e29.g0_.z) * vec4<f32>(_e33.g1_.z, _e36.g1_.y, _e39.g1_.x, _e42.g1_.z)) * vec4<f32>(1.0, 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e55.g1_.x) * vec4<f32>(_e59.g0_.x, _e62.g0_.x, _e65.g0_.z, _e68.g0_.y)) * vec4<f32>(1.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e81.g1_.y) * vec4<f32>(_e85.g0_.y, _e88.g0_.z, _e91.g0_.y, _e94.g0_.x)) * vec4<f32>(1.0, -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e107.g1_.z) * vec4<f32>(_e111.g0_.z, _e114.g0_.y, _e117.g0_.x, _e120.g0_.z)) * vec4<f32>(1.0, 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e133.g0_.x) * vec4<f32>(_e137.g1_.x, _e140.g1_.x, _e143.g1_.z, _e146.g1_.y)) * vec4<f32>(1.0, 0.0, 1.0, -(1.0)))), ((((vec4<f32>(_e159.g0_.y) * vec4<f32>(_e163.g0_.y, _e166.g0_.z, _e169.g0_.y, _e172.g0_.x)) * vec4<f32>(-(1.0), -(1.0), 0.0, 1.0)) + ((vec4<f32>(_e185.g0_.z) * vec4<f32>(_e189.g0_.z, _e192.g0_.y, _e195.g0_.x, _e198.g0_.z)) * vec4<f32>(-(1.0), 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e212.g0_.x) * vec4<f32>(_e216.g0_.x, _e219.g0_.x, _e222.g0_.z, _e225.g0_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0)))));
}

fn line_line_inner_anti_product(self_1352: Line, other_1120: Line) -> AntiScalar {
    var self_1353: Line;
    var other_1121: Line;

    self_1353 = self_1352;
    other_1121 = other_1120;
    let _e5: Line = self_1353;
    let _e8: Line = other_1121;
    let _e13: Line = self_1353;
    let _e16: Line = other_1121;
    let _e21: Line = self_1353;
    let _e24: Line = other_1121;
    return AntiScalar((((0.0 - (_e5.g0_.x * _e8.g0_.x)) - (_e13.g0_.y * _e16.g0_.y)) - (_e21.g0_.z * _e24.g0_.z)));
}

fn line_line_left_contraction(self_1354: Line, other_1122: Line) -> Scalar {
    var self_1355: Line;
    var other_1123: Line;

    self_1355 = self_1354;
    other_1123 = other_1122;
    let _e5: Line = self_1355;
    let _e8: Line = other_1123;
    let _e13: Line = self_1355;
    let _e16: Line = other_1123;
    let _e21: Line = self_1355;
    let _e24: Line = other_1123;
    return Scalar((((0.0 - (_e5.g1_.x * _e8.g1_.x)) - (_e13.g1_.y * _e16.g1_.y)) - (_e21.g1_.z * _e24.g1_.z)));
}

fn line_line_right_contraction(self_1356: Line, other_1124: Line) -> Scalar {
    var self_1357: Line;
    var other_1125: Line;

    self_1357 = self_1356;
    other_1125 = other_1124;
    let _e5: Line = self_1357;
    let _e8: Line = other_1125;
    let _e13: Line = self_1357;
    let _e16: Line = other_1125;
    let _e21: Line = self_1357;
    let _e24: Line = other_1125;
    return Scalar((((0.0 - (_e5.g1_.x * _e8.g1_.x)) - (_e13.g1_.y * _e16.g1_.y)) - (_e21.g1_.z * _e24.g1_.z)));
}

fn line_line_left_anti_contraction(self_1358: Line, other_1126: Line) -> AntiScalar {
    var self_1359: Line;
    var other_1127: Line;

    self_1359 = self_1358;
    other_1127 = other_1126;
    let _e5: Line = self_1359;
    let _e8: Line = other_1127;
    let _e13: Line = self_1359;
    let _e16: Line = other_1127;
    let _e21: Line = self_1359;
    let _e24: Line = other_1127;
    return AntiScalar((((0.0 - (_e5.g0_.x * _e8.g0_.x)) - (_e13.g0_.y * _e16.g0_.y)) - (_e21.g0_.z * _e24.g0_.z)));
}

fn line_line_right_anti_contraction(self_1360: Line, other_1128: Line) -> AntiScalar {
    var self_1361: Line;
    var other_1129: Line;

    self_1361 = self_1360;
    other_1129 = other_1128;
    let _e5: Line = self_1361;
    let _e8: Line = other_1129;
    let _e13: Line = self_1361;
    let _e16: Line = other_1129;
    let _e21: Line = self_1361;
    let _e24: Line = other_1129;
    return AntiScalar((((0.0 - (_e5.g0_.x * _e8.g0_.x)) - (_e13.g0_.y * _e16.g0_.y)) - (_e21.g0_.z * _e24.g0_.z)));
}

fn line_line_scalar_product(self_1362: Line, other_1130: Line) -> Scalar {
    var self_1363: Line;
    var other_1131: Line;

    self_1363 = self_1362;
    other_1131 = other_1130;
    let _e5: Line = self_1363;
    let _e8: Line = other_1131;
    let _e13: Line = self_1363;
    let _e16: Line = other_1131;
    let _e21: Line = self_1363;
    let _e24: Line = other_1131;
    return Scalar((((0.0 - (_e5.g1_.x * _e8.g1_.x)) - (_e13.g1_.y * _e16.g1_.y)) - (_e21.g1_.z * _e24.g1_.z)));
}

fn line_line_anti_scalar_product(self_1364: Line, other_1132: Line) -> AntiScalar {
    var self_1365: Line;
    var other_1133: Line;

    self_1365 = self_1364;
    other_1133 = other_1132;
    let _e5: Line = self_1365;
    let _e8: Line = other_1133;
    let _e13: Line = self_1365;
    let _e16: Line = other_1133;
    let _e21: Line = self_1365;
    let _e24: Line = other_1133;
    return AntiScalar((((0.0 - (_e5.g0_.x * _e8.g0_.x)) - (_e13.g0_.y * _e16.g0_.y)) - (_e21.g0_.z * _e24.g0_.z)));
}

fn line_translator_regressive_product(self_1366: Line, other_1134: Translator) -> Scalar {
    var self_1367: Line;
    var other_1135: Translator;

    self_1367 = self_1366;
    other_1135 = other_1134;
    let _e4: Line = self_1367;
    let _e7: Translator = other_1135;
    let _e11: Line = self_1367;
    let _e14: Translator = other_1135;
    let _e19: Line = self_1367;
    let _e22: Translator = other_1135;
    return Scalar((((_e4.g1_.x * _e7.g0_.y) + (_e11.g1_.y * _e14.g0_.z)) + (_e19.g1_.z * _e22.g0_.w)));
}

fn line_translator_inner_product(self_1368: Line, other_1136: Translator) -> Line {
    var self_1369: Line;
    var other_1137: Translator;

    self_1369 = self_1368;
    other_1137 = other_1136;
    let _e4: Line = self_1369;
    let _e6: Translator = other_1137;
    let _e11: Line = self_1369;
    let _e13: Translator = other_1137;
    return Line((_e4.g0_ * vec3<f32>(_e6.g0_.x)), (_e11.g1_ * vec3<f32>(_e13.g0_.x)));
}

fn line_translator_geometric_anti_product(self_1370: Line, other_1138: Translator) -> Motor {
    var self_1371: Line;
    var other_1139: Translator;

    self_1371 = self_1370;
    other_1139 = other_1138;
    let _e4: Line = self_1371;
    let _e8: Translator = other_1139;
    let _e19: Line = self_1371;
    let _e23: Translator = other_1139;
    let _e35: Line = self_1371;
    let _e39: Translator = other_1139;
    let _e51: Line = self_1371;
    let _e54: Line = self_1371;
    let _e57: Line = self_1371;
    let _e60: Line = self_1371;
    let _e64: Translator = other_1139;
    let _e79: Line = self_1371;
    let _e83: Translator = other_1139;
    let _e95: Line = self_1371;
    let _e99: Translator = other_1139;
    let _e112: Line = self_1371;
    let _e116: Translator = other_1139;
    return Motor((((((vec4<f32>(_e4.g1_.x) * _e8.g0_.yywz) * vec4<f32>(1.0, 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e19.g1_.y) * _e23.g0_.zwzy) * vec4<f32>(1.0, -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e35.g1_.z) * _e39.g0_.wzyw) * vec4<f32>(1.0, 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e51.g0_.x, _e54.g0_.x, _e57.g0_.y, _e60.g0_.z) * vec4<f32>(_e64.g0_.x)) * vec4<f32>(0.0, -(1.0), -(1.0), -(1.0)))), ((((vec4<f32>(_e79.g0_.y) * _e83.g0_.zwzy) * vec4<f32>(-(1.0), -(1.0), 0.0, 1.0)) + ((vec4<f32>(_e95.g0_.z) * _e99.g0_.wzyw) * vec4<f32>(-(1.0), 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e112.g0_.x) * _e116.g0_.yxwz) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0)))));
}

fn line_translator_right_contraction(self_1372: Line, other_1140: Translator) -> Line {
    var self_1373: Line;
    var other_1141: Translator;

    self_1373 = self_1372;
    other_1141 = other_1140;
    let _e4: Line = self_1373;
    let _e6: Translator = other_1141;
    let _e11: Line = self_1373;
    let _e13: Translator = other_1141;
    return Line((_e4.g0_ * vec3<f32>(_e6.g0_.x)), (_e11.g1_ * vec3<f32>(_e13.g0_.x)));
}

fn line_translator_right_anti_contraction(self_1374: Line, other_1142: Translator) -> AntiScalar {
    var self_1375: Line;
    var other_1143: Translator;

    self_1375 = self_1374;
    other_1143 = other_1142;
    let _e5: Line = self_1375;
    let _e8: Translator = other_1143;
    let _e13: Line = self_1375;
    let _e16: Translator = other_1143;
    let _e21: Line = self_1375;
    let _e24: Translator = other_1143;
    return AntiScalar((((0.0 - (_e5.g0_.x * _e8.g0_.y)) - (_e13.g0_.y * _e16.g0_.z)) - (_e21.g0_.z * _e24.g0_.w)));
}

fn line_translator_anti_scalar_product(self_1376: Line, other_1144: Translator) -> AntiScalar {
    var self_1377: Line;
    var other_1145: Translator;

    self_1377 = self_1376;
    other_1145 = other_1144;
    let _e5: Line = self_1377;
    let _e8: Translator = other_1145;
    let _e13: Line = self_1377;
    let _e16: Translator = other_1145;
    let _e21: Line = self_1377;
    let _e24: Translator = other_1145;
    return AntiScalar((((0.0 - (_e5.g0_.x * _e8.g0_.y)) - (_e13.g0_.y * _e16.g0_.z)) - (_e21.g0_.z * _e24.g0_.w)));
}

fn line_motor_add(self_1378: Line, other_1146: Motor) -> Motor {
    var self_1379: Line;
    var other_1147: Motor;

    self_1379 = self_1378;
    other_1147 = other_1146;
    let _e4: Line = self_1379;
    let _e7: Line = self_1379;
    let _e10: Line = self_1379;
    let _e13: Line = self_1379;
    let _e23: Motor = other_1147;
    let _e26: Line = self_1379;
    let _e29: Line = self_1379;
    let _e32: Line = self_1379;
    let _e35: Line = self_1379;
    let _e45: Motor = other_1147;
    return Motor(((vec4<f32>(_e4.g0_.x, _e7.g1_.x, _e10.g1_.y, _e13.g1_.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0)) + _e23.g0_), ((vec4<f32>(_e26.g0_.x, _e29.g0_.x, _e32.g0_.y, _e35.g0_.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0)) + _e45.g1_));
}

fn line_motor_sub(self_1380: Line, other_1148: Motor) -> Motor {
    var self_1381: Line;
    var other_1149: Motor;

    self_1381 = self_1380;
    other_1149 = other_1148;
    let _e4: Line = self_1381;
    let _e7: Line = self_1381;
    let _e10: Line = self_1381;
    let _e13: Line = self_1381;
    let _e23: Motor = other_1149;
    let _e26: Line = self_1381;
    let _e29: Line = self_1381;
    let _e32: Line = self_1381;
    let _e35: Line = self_1381;
    let _e45: Motor = other_1149;
    return Motor(((vec4<f32>(_e4.g0_.x, _e7.g1_.x, _e10.g1_.y, _e13.g1_.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0)) - _e23.g0_), ((vec4<f32>(_e26.g0_.x, _e29.g0_.x, _e32.g0_.y, _e35.g0_.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0)) - _e45.g1_));
}

fn line_motor_geometric_product(self_1382: Line, other_1150: Motor) -> Motor {
    var self_1383: Line;
    var other_1151: Motor;

    self_1383 = self_1382;
    other_1151 = other_1150;
    let _e4: Line = self_1383;
    let _e8: Motor = other_1151;
    let _e20: Line = self_1383;
    let _e24: Motor = other_1151;
    let _e37: Line = self_1383;
    let _e41: Motor = other_1151;
    let _e54: Line = self_1383;
    let _e58: Motor = other_1151;
    let _e69: Line = self_1383;
    let _e73: Motor = other_1151;
    let _e85: Line = self_1383;
    let _e89: Motor = other_1151;
    let _e101: Line = self_1383;
    let _e105: Motor = other_1151;
    let _e118: Line = self_1383;
    let _e122: Motor = other_1151;
    let _e135: Line = self_1383;
    let _e139: Motor = other_1151;
    return Motor(((((vec4<f32>(_e4.g1_.x) * _e8.g0_.yxwz) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0))) + ((vec4<f32>(_e20.g1_.y) * _e24.g0_.zwxy) * vec4<f32>(-(1.0), -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e37.g1_.z) * _e41.g0_.wzyx) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))), (((((((vec4<f32>(_e54.g0_.x) * _e58.g0_.yxwz) * vec4<f32>(1.0, 1.0, 1.0, -(1.0))) + ((vec4<f32>(_e69.g0_.y) * _e73.g0_.zwxy) * vec4<f32>(1.0, -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e85.g0_.z) * _e89.g0_.wzyx) * vec4<f32>(1.0, 1.0, -(1.0), 1.0))) + ((vec4<f32>(_e101.g1_.x) * _e105.g1_.yxwz) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e118.g1_.y) * _e122.g1_.zwxy) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e135.g1_.z) * _e139.g1_.wzyx) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))));
}

fn line_motor_geometric_anti_product(self_1384: Line, other_1152: Motor) -> Motor {
    var self_1385: Line;
    var other_1153: Motor;

    self_1385 = self_1384;
    other_1153 = other_1152;
    let _e4: Line = self_1385;
    let _e8: Motor = other_1153;
    let _e20: Line = self_1385;
    let _e24: Motor = other_1153;
    let _e37: Line = self_1385;
    let _e41: Motor = other_1153;
    let _e54: Line = self_1385;
    let _e58: Motor = other_1153;
    let _e70: Line = self_1385;
    let _e74: Motor = other_1153;
    let _e86: Line = self_1385;
    let _e90: Motor = other_1153;
    let _e102: Line = self_1385;
    let _e106: Motor = other_1153;
    let _e118: Line = self_1385;
    let _e122: Motor = other_1153;
    let _e135: Line = self_1385;
    let _e139: Motor = other_1153;
    return Motor((((((((vec4<f32>(_e4.g0_.x) * _e8.g0_.yxwz) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0))) + ((vec4<f32>(_e20.g0_.y) * _e24.g0_.zwxy) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e37.g0_.z) * _e41.g0_.wzyx) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e54.g1_.x) * _e58.g1_.yxwz) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e70.g1_.y) * _e74.g1_.zwxy) * vec4<f32>(1.0, -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e86.g1_.z) * _e90.g1_.wzyx) * vec4<f32>(1.0, 1.0, -(1.0), 1.0))), ((((vec4<f32>(_e102.g0_.x) * _e106.g1_.yxwz) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0))) + ((vec4<f32>(_e118.g0_.y) * _e122.g1_.zwxy) * vec4<f32>(-(1.0), -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e135.g0_.z) * _e139.g1_.wzyx) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))));
}

fn line_motor_left_contraction(self_1386: Line, other_1154: Motor) -> Translator {
    var self_1387: Line;
    var other_1155: Motor;

    self_1387 = self_1386;
    other_1155 = other_1154;
    let _e4: Line = self_1387;
    let _e8: Motor = other_1155;
    let _e11: Motor = other_1155;
    let _e14: Motor = other_1155;
    let _e17: Motor = other_1155;
    let _e30: Line = self_1387;
    let _e34: Motor = other_1155;
    let _e37: Motor = other_1155;
    let _e40: Motor = other_1155;
    let _e43: Motor = other_1155;
    let _e57: Line = self_1387;
    let _e61: Motor = other_1155;
    let _e64: Motor = other_1155;
    let _e67: Motor = other_1155;
    let _e70: Motor = other_1155;
    return Translator(((((vec4<f32>(_e4.g1_.y) * vec4<f32>(_e8.g0_.z, _e11.g0_.z, _e14.g1_.x, _e17.g0_.z)) * vec4<f32>(-(1.0), 0.0, -(1.0), 0.0)) + ((vec4<f32>(_e30.g1_.z) * vec4<f32>(_e34.g0_.w, _e37.g0_.w, _e40.g0_.w, _e43.g1_.x)) * vec4<f32>(-(1.0), 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e57.g1_.x) * vec4<f32>(_e61.g0_.y, _e64.g1_.x, _e67.g0_.x, _e70.g0_.x)) * vec4<f32>(-(1.0), -(1.0), 0.0, 0.0))));
}

fn line_motor_scalar_product(self_1388: Line, other_1156: Motor) -> Scalar {
    var self_1389: Line;
    var other_1157: Motor;

    self_1389 = self_1388;
    other_1157 = other_1156;
    let _e5: Line = self_1389;
    let _e8: Motor = other_1157;
    let _e13: Line = self_1389;
    let _e16: Motor = other_1157;
    let _e21: Line = self_1389;
    let _e24: Motor = other_1157;
    return Scalar((((0.0 - (_e5.g1_.x * _e8.g0_.y)) - (_e13.g1_.y * _e16.g0_.z)) - (_e21.g1_.z * _e24.g0_.w)));
}

fn line_motor_anti_scalar_product(self_1390: Line, other_1158: Motor) -> AntiScalar {
    var self_1391: Line;
    var other_1159: Motor;

    self_1391 = self_1390;
    other_1159 = other_1158;
    let _e5: Line = self_1391;
    let _e8: Motor = other_1159;
    let _e13: Line = self_1391;
    let _e16: Motor = other_1159;
    let _e21: Line = self_1391;
    let _e24: Motor = other_1159;
    return AntiScalar((((0.0 - (_e5.g0_.x * _e8.g1_.y)) - (_e13.g0_.y * _e16.g1_.z)) - (_e21.g0_.z * _e24.g1_.w)));
}

fn line_point_and_plane_geometric_product(self_1392: Line, other_1160: PointAndPlane) -> PointAndPlane {
    var self_1393: Line;
    var other_1161: PointAndPlane;

    self_1393 = self_1392;
    other_1161 = other_1160;
    let _e4: Line = self_1393;
    let _e8: PointAndPlane = other_1161;
    let _e11: PointAndPlane = other_1161;
    let _e14: PointAndPlane = other_1161;
    let _e17: PointAndPlane = other_1161;
    let _e30: Line = self_1393;
    let _e34: PointAndPlane = other_1161;
    let _e37: PointAndPlane = other_1161;
    let _e40: PointAndPlane = other_1161;
    let _e43: PointAndPlane = other_1161;
    let _e57: Line = self_1393;
    let _e61: PointAndPlane = other_1161;
    let _e64: PointAndPlane = other_1161;
    let _e67: PointAndPlane = other_1161;
    let _e70: PointAndPlane = other_1161;
    let _e84: Line = self_1393;
    let _e88: PointAndPlane = other_1161;
    let _e91: PointAndPlane = other_1161;
    let _e94: PointAndPlane = other_1161;
    let _e97: PointAndPlane = other_1161;
    let _e111: Line = self_1393;
    let _e115: PointAndPlane = other_1161;
    let _e118: PointAndPlane = other_1161;
    let _e121: PointAndPlane = other_1161;
    let _e124: PointAndPlane = other_1161;
    let _e138: Line = self_1393;
    let _e142: PointAndPlane = other_1161;
    let _e145: PointAndPlane = other_1161;
    let _e148: PointAndPlane = other_1161;
    let _e151: PointAndPlane = other_1161;
    let _e165: Line = self_1393;
    let _e169: PointAndPlane = other_1161;
    let _e180: Line = self_1393;
    let _e184: PointAndPlane = other_1161;
    let _e196: Line = self_1393;
    let _e200: PointAndPlane = other_1161;
    let _e203: PointAndPlane = other_1161;
    let _e206: PointAndPlane = other_1161;
    let _e209: PointAndPlane = other_1161;
    let _e223: Line = self_1393;
    let _e227: PointAndPlane = other_1161;
    let _e230: PointAndPlane = other_1161;
    let _e233: PointAndPlane = other_1161;
    let _e236: PointAndPlane = other_1161;
    let _e250: Line = self_1393;
    let _e254: PointAndPlane = other_1161;
    let _e257: PointAndPlane = other_1161;
    let _e260: PointAndPlane = other_1161;
    let _e263: PointAndPlane = other_1161;
    let _e277: Line = self_1393;
    let _e281: PointAndPlane = other_1161;
    return PointAndPlane((((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g1_.w, _e11.g1_.w, _e14.g0_.x, _e17.g1_.y)) * vec4<f32>(0.0, -(1.0), -(1.0), 1.0)) + ((vec4<f32>(_e30.g0_.z) * vec4<f32>(_e34.g1_.z, _e37.g1_.z, _e40.g1_.y, _e43.g0_.x)) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e57.g1_.x) * vec4<f32>(_e61.g1_.y, _e64.g1_.x, _e67.g0_.w, _e70.g0_.z)) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e84.g1_.y) * vec4<f32>(_e88.g1_.z, _e91.g0_.w, _e94.g1_.x, _e97.g0_.y)) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e111.g1_.z) * vec4<f32>(_e115.g1_.w, _e118.g0_.z, _e121.g0_.y, _e124.g1_.x)) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e138.g0_.x) * vec4<f32>(_e142.g0_.x, _e145.g0_.x, _e148.g1_.w, _e151.g1_.z)) * vec4<f32>(0.0, -(1.0), 1.0, -(1.0)))), (((((((vec4<f32>(_e165.g0_.y) * vec4<f32>(_e169.g1_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) + ((vec4<f32>(_e180.g0_.z) * vec4<f32>(_e184.g1_.w)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e196.g1_.x) * vec4<f32>(_e200.g0_.y, _e203.g0_.x, _e206.g1_.w, _e209.g1_.z)) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e223.g1_.y) * vec4<f32>(_e227.g0_.z, _e230.g1_.w, _e233.g0_.x, _e236.g1_.y)) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e250.g1_.z) * vec4<f32>(_e254.g0_.w, _e257.g1_.z, _e260.g1_.y, _e263.g0_.x)) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e277.g0_.x) * _e281.g1_.yxxx) * vec4<f32>(1.0, 0.0, 0.0, 0.0))));
}

fn line_point_and_plane_regressive_product(self_1394: Line, other_1162: PointAndPlane) -> Plane {
    var self_1395: Line;
    var other_1163: PointAndPlane;

    self_1395 = self_1394;
    other_1163 = other_1162;
    let _e4: Line = self_1395;
    let _e8: PointAndPlane = other_1163;
    let _e19: Line = self_1395;
    let _e23: PointAndPlane = other_1163;
    let _e35: Line = self_1395;
    let _e39: PointAndPlane = other_1163;
    let _e51: Line = self_1395;
    let _e55: PointAndPlane = other_1163;
    let _e67: Line = self_1395;
    let _e70: Line = self_1395;
    let _e73: Line = self_1395;
    let _e76: Line = self_1395;
    let _e80: PointAndPlane = other_1163;
    return Plane(((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.zzxz) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0)) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.wwwx) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))) + ((vec4<f32>(_e35.g1_.y) * _e39.g0_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0)))) + ((vec4<f32>(_e51.g1_.z) * _e55.g0_.zzyz) * vec4<f32>(0.0, -(1.0), 1.0, 0.0))) + ((vec4<f32>(_e67.g0_.x, _e70.g0_.x, _e73.g1_.x, _e76.g1_.x) * _e80.g0_.yxwz) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))));
}

fn line_point_and_plane_outer_product(self_1396: Line, other_1164: PointAndPlane) -> Point {
    var self_1397: Line;
    var other_1165: PointAndPlane;

    self_1397 = self_1396;
    other_1165 = other_1164;
    let _e4: Line = self_1397;
    let _e8: PointAndPlane = other_1165;
    let _e19: Line = self_1397;
    let _e23: PointAndPlane = other_1165;
    let _e35: Line = self_1397;
    let _e39: PointAndPlane = other_1165;
    let _e51: Line = self_1397;
    let _e55: PointAndPlane = other_1165;
    let _e67: Line = self_1397;
    let _e70: Line = self_1397;
    let _e73: Line = self_1397;
    let _e76: Line = self_1397;
    let _e80: PointAndPlane = other_1165;
    return Point(((((((vec4<f32>(_e4.g0_.y) * _e8.g1_.wwwy) * vec4<f32>(0.0, -(1.0), 0.0, 1.0)) + ((vec4<f32>(_e19.g0_.z) * _e23.g1_.zzyz) * vec4<f32>(0.0, 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e35.g1_.y) * _e39.g1_.zzxz) * vec4<f32>(1.0, 0.0, -(1.0), 0.0))) + ((vec4<f32>(_e51.g1_.z) * _e55.g1_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e67.g1_.x, _e70.g1_.x, _e73.g0_.x, _e76.g0_.x) * _e80.g1_.yxwz) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))));
}

fn line_point_and_plane_inner_product(self_1398: Line, other_1166: PointAndPlane) -> Plane {
    var self_1399: Line;
    var other_1167: PointAndPlane;

    self_1399 = self_1398;
    other_1167 = other_1166;
    let _e4: Line = self_1399;
    let _e8: PointAndPlane = other_1167;
    let _e19: Line = self_1399;
    let _e23: PointAndPlane = other_1167;
    let _e35: Line = self_1399;
    let _e39: PointAndPlane = other_1167;
    let _e42: PointAndPlane = other_1167;
    let _e45: PointAndPlane = other_1167;
    let _e48: PointAndPlane = other_1167;
    let _e62: Line = self_1399;
    let _e66: PointAndPlane = other_1167;
    let _e69: PointAndPlane = other_1167;
    let _e72: PointAndPlane = other_1167;
    let _e75: PointAndPlane = other_1167;
    let _e89: Line = self_1399;
    let _e93: PointAndPlane = other_1167;
    let _e96: PointAndPlane = other_1167;
    let _e99: PointAndPlane = other_1167;
    let _e102: PointAndPlane = other_1167;
    let _e116: Line = self_1399;
    let _e120: PointAndPlane = other_1167;
    return Plane((((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g1_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) + ((vec4<f32>(_e19.g0_.z) * vec4<f32>(_e23.g1_.w)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e35.g1_.x) * vec4<f32>(_e39.g0_.y, _e42.g0_.x, _e45.g1_.w, _e48.g1_.z)) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e62.g1_.y) * vec4<f32>(_e66.g0_.z, _e69.g1_.w, _e72.g0_.x, _e75.g1_.y)) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e89.g1_.z) * vec4<f32>(_e93.g0_.w, _e96.g1_.z, _e99.g1_.y, _e102.g0_.x)) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e116.g0_.x) * _e120.g1_.yxxx) * vec4<f32>(1.0, 0.0, 0.0, 0.0))));
}

fn line_point_and_plane_geometric_anti_product(self_1400: Line, other_1168: PointAndPlane) -> PointAndPlane {
    var self_1401: Line;
    var other_1169: PointAndPlane;

    self_1401 = self_1400;
    other_1169 = other_1168;
    let _e4: Line = self_1401;
    let _e8: PointAndPlane = other_1169;
    let _e11: PointAndPlane = other_1169;
    let _e14: PointAndPlane = other_1169;
    let _e17: PointAndPlane = other_1169;
    let _e30: Line = self_1401;
    let _e34: PointAndPlane = other_1169;
    let _e37: PointAndPlane = other_1169;
    let _e40: PointAndPlane = other_1169;
    let _e43: PointAndPlane = other_1169;
    let _e57: Line = self_1401;
    let _e61: PointAndPlane = other_1169;
    let _e64: PointAndPlane = other_1169;
    let _e67: PointAndPlane = other_1169;
    let _e70: PointAndPlane = other_1169;
    let _e84: Line = self_1401;
    let _e88: PointAndPlane = other_1169;
    let _e100: Line = self_1401;
    let _e104: PointAndPlane = other_1169;
    let _e116: Line = self_1401;
    let _e120: PointAndPlane = other_1169;
    let _e131: Line = self_1401;
    let _e135: PointAndPlane = other_1169;
    let _e138: PointAndPlane = other_1169;
    let _e141: PointAndPlane = other_1169;
    let _e144: PointAndPlane = other_1169;
    let _e157: Line = self_1401;
    let _e161: PointAndPlane = other_1169;
    let _e164: PointAndPlane = other_1169;
    let _e167: PointAndPlane = other_1169;
    let _e170: PointAndPlane = other_1169;
    let _e184: Line = self_1401;
    let _e188: PointAndPlane = other_1169;
    let _e191: PointAndPlane = other_1169;
    let _e194: PointAndPlane = other_1169;
    let _e197: PointAndPlane = other_1169;
    let _e211: Line = self_1401;
    let _e215: PointAndPlane = other_1169;
    let _e218: PointAndPlane = other_1169;
    let _e221: PointAndPlane = other_1169;
    let _e224: PointAndPlane = other_1169;
    let _e238: Line = self_1401;
    let _e242: PointAndPlane = other_1169;
    let _e245: PointAndPlane = other_1169;
    let _e248: PointAndPlane = other_1169;
    let _e251: PointAndPlane = other_1169;
    let _e265: Line = self_1401;
    let _e269: PointAndPlane = other_1169;
    let _e272: PointAndPlane = other_1169;
    let _e275: PointAndPlane = other_1169;
    let _e278: PointAndPlane = other_1169;
    return PointAndPlane((((((((vec4<f32>(_e4.g0_.x) * vec4<f32>(_e8.g1_.y, _e11.g1_.x, _e14.g0_.w, _e17.g0_.z)) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0))) + ((vec4<f32>(_e30.g0_.y) * vec4<f32>(_e34.g1_.z, _e37.g0_.w, _e40.g1_.x, _e43.g0_.y)) * vec4<f32>(-(1.0), -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e57.g0_.z) * vec4<f32>(_e61.g1_.w, _e64.g0_.z, _e67.g0_.y, _e70.g1_.x)) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))) + ((vec4<f32>(_e84.g1_.y) * vec4<f32>(_e88.g0_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e100.g1_.z) * vec4<f32>(_e104.g0_.w)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e116.g1_.x) * _e120.g0_.yxxx) * vec4<f32>(1.0, 0.0, 0.0, 0.0))), (((((((vec4<f32>(_e131.g0_.x) * vec4<f32>(_e135.g0_.y, _e138.g0_.x, _e141.g1_.w, _e144.g1_.z)) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0))) + ((vec4<f32>(_e157.g0_.y) * vec4<f32>(_e161.g0_.z, _e164.g1_.w, _e167.g0_.x, _e170.g1_.y)) * vec4<f32>(-(1.0), -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e184.g0_.z) * vec4<f32>(_e188.g0_.w, _e191.g1_.z, _e194.g1_.y, _e197.g0_.x)) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))) + ((vec4<f32>(_e211.g1_.y) * vec4<f32>(_e215.g0_.w, _e218.g0_.w, _e221.g1_.x, _e224.g0_.y)) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e238.g1_.z) * vec4<f32>(_e242.g0_.z, _e245.g0_.z, _e248.g0_.y, _e251.g1_.x)) * vec4<f32>(0.0, -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e265.g1_.x) * vec4<f32>(_e269.g1_.x, _e272.g1_.x, _e275.g0_.w, _e278.g0_.z)) * vec4<f32>(0.0, -(1.0), -(1.0), 1.0))));
}

fn line_point_and_plane_inner_anti_product(self_1402: Line, other_1170: PointAndPlane) -> Point {
    var self_1403: Line;
    var other_1171: PointAndPlane;

    self_1403 = self_1402;
    other_1171 = other_1170;
    let _e4: Line = self_1403;
    let _e8: PointAndPlane = other_1171;
    let _e11: PointAndPlane = other_1171;
    let _e14: PointAndPlane = other_1171;
    let _e17: PointAndPlane = other_1171;
    let _e30: Line = self_1403;
    let _e34: PointAndPlane = other_1171;
    let _e37: PointAndPlane = other_1171;
    let _e40: PointAndPlane = other_1171;
    let _e43: PointAndPlane = other_1171;
    let _e57: Line = self_1403;
    let _e61: PointAndPlane = other_1171;
    let _e64: PointAndPlane = other_1171;
    let _e67: PointAndPlane = other_1171;
    let _e70: PointAndPlane = other_1171;
    let _e84: Line = self_1403;
    let _e88: PointAndPlane = other_1171;
    let _e100: Line = self_1403;
    let _e104: PointAndPlane = other_1171;
    let _e116: Line = self_1403;
    let _e120: PointAndPlane = other_1171;
    return Point((((((((vec4<f32>(_e4.g0_.x) * vec4<f32>(_e8.g1_.y, _e11.g1_.x, _e14.g0_.w, _e17.g0_.z)) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0))) + ((vec4<f32>(_e30.g0_.y) * vec4<f32>(_e34.g1_.z, _e37.g0_.w, _e40.g1_.x, _e43.g0_.y)) * vec4<f32>(-(1.0), -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e57.g0_.z) * vec4<f32>(_e61.g1_.w, _e64.g0_.z, _e67.g0_.y, _e70.g1_.x)) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))) + ((vec4<f32>(_e84.g1_.y) * vec4<f32>(_e88.g0_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e100.g1_.z) * vec4<f32>(_e104.g0_.w)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e116.g1_.x) * _e120.g0_.yxxx) * vec4<f32>(1.0, 0.0, 0.0, 0.0))));
}

fn line_point_and_plane_left_contraction(self_1404: Line, other_1172: PointAndPlane) -> Plane {
    var self_1405: Line;
    var other_1173: PointAndPlane;

    self_1405 = self_1404;
    other_1173 = other_1172;
    let _e4: Line = self_1405;
    let _e8: PointAndPlane = other_1173;
    let _e19: Line = self_1405;
    let _e23: PointAndPlane = other_1173;
    let _e35: Line = self_1405;
    let _e39: PointAndPlane = other_1173;
    return Plane(((((vec4<f32>(_e4.g1_.y) * _e8.g0_.zzxz) * vec4<f32>(1.0, 0.0, -(1.0), 0.0)) + ((vec4<f32>(_e19.g1_.z) * _e23.g0_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e35.g1_.x) * _e39.g0_.yxxx) * vec4<f32>(1.0, -(1.0), 0.0, 0.0))));
}

fn line_point_and_plane_right_contraction(self_1406: Line, other_1174: PointAndPlane) -> Plane {
    var self_1407: Line;
    var other_1175: PointAndPlane;

    self_1407 = self_1406;
    other_1175 = other_1174;
    let _e4: Line = self_1407;
    let _e8: PointAndPlane = other_1175;
    let _e19: Line = self_1407;
    let _e23: PointAndPlane = other_1175;
    let _e35: Line = self_1407;
    let _e39: PointAndPlane = other_1175;
    let _e51: Line = self_1407;
    let _e55: PointAndPlane = other_1175;
    let _e67: Line = self_1407;
    let _e70: Line = self_1407;
    let _e73: Line = self_1407;
    let _e76: Line = self_1407;
    let _e80: PointAndPlane = other_1175;
    return Plane(((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g1_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) + ((vec4<f32>(_e19.g0_.z) * vec4<f32>(_e23.g1_.w)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e35.g1_.y) * _e39.g1_.wwwy) * vec4<f32>(0.0, -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e51.g1_.z) * _e55.g1_.zzyz) * vec4<f32>(0.0, 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e67.g0_.x, _e70.g0_.x, _e73.g1_.x, _e76.g1_.x) * _e80.g1_.yxwz) * vec4<f32>(1.0, 0.0, 1.0, -(1.0)))));
}

fn line_point_and_plane_left_anti_contraction(self_1408: Line, other_1176: PointAndPlane) -> Point {
    var self_1409: Line;
    var other_1177: PointAndPlane;

    self_1409 = self_1408;
    other_1177 = other_1176;
    let _e4: Line = self_1409;
    let _e8: PointAndPlane = other_1177;
    let _e19: Line = self_1409;
    let _e23: PointAndPlane = other_1177;
    let _e35: Line = self_1409;
    let _e39: PointAndPlane = other_1177;
    return Point(((((vec4<f32>(_e4.g0_.y) * _e8.g1_.zzxz) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0)) + ((vec4<f32>(_e19.g0_.z) * _e23.g1_.wwwx) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))) + ((vec4<f32>(_e35.g0_.x) * _e39.g1_.yxxx) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))));
}

fn line_point_and_plane_right_anti_contraction(self_1410: Line, other_1178: PointAndPlane) -> Point {
    var self_1411: Line;
    var other_1179: PointAndPlane;

    self_1411 = self_1410;
    other_1179 = other_1178;
    let _e4: Line = self_1411;
    let _e8: PointAndPlane = other_1179;
    let _e19: Line = self_1411;
    let _e23: PointAndPlane = other_1179;
    let _e35: Line = self_1411;
    let _e39: PointAndPlane = other_1179;
    let _e51: Line = self_1411;
    let _e55: PointAndPlane = other_1179;
    let _e67: Line = self_1411;
    let _e70: Line = self_1411;
    let _e73: Line = self_1411;
    let _e76: Line = self_1411;
    let _e80: PointAndPlane = other_1179;
    return Point(((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.wwwy) * vec4<f32>(0.0, -(1.0), 0.0, 1.0)) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.zzyz) * vec4<f32>(0.0, 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e35.g1_.y) * vec4<f32>(_e39.g0_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e51.g1_.z) * vec4<f32>(_e55.g0_.w)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e67.g1_.x, _e70.g0_.x, _e73.g0_.x, _e76.g0_.x) * _e80.g0_.yxwz) * vec4<f32>(1.0, 0.0, 1.0, -(1.0)))));
}

fn line_squared_magnitude(self_1412: Line) -> Scalar {
    var self_1413: Line;

    self_1413 = self_1412;
    let _e2: Line = self_1413;
    let _e3: Line = self_1413;
    let _e4: Line = line_reversal(_e3);
    let _e5: Scalar = line_line_scalar_product(_e2, _e4);
    return _e5;
}

fn line_magnitude(self_1414: Line) -> Scalar {
    var self_1415: Line;

    self_1415 = self_1414;
    let _e2: Line = self_1415;
    let _e3: Scalar = line_squared_magnitude(_e2);
    return Scalar(sqrt(_e3.g0_));
}

fn line_bulk_norm(self_1416: Line) -> Scalar {
    var self_1417: Line;

    self_1417 = self_1416;
    let _e2: Line = self_1417;
    let _e3: Scalar = line_squared_magnitude(_e2);
    return Scalar(sqrt(_e3.g0_));
}

fn line_squared_anti_magnitude(self_1418: Line) -> AntiScalar {
    var self_1419: Line;

    self_1419 = self_1418;
    let _e2: Line = self_1419;
    let _e3: Line = self_1419;
    let _e4: Line = line_anti_reversal(_e3);
    let _e5: AntiScalar = line_line_anti_scalar_product(_e2, _e4);
    return _e5;
}

fn line_weight_norm(self_1420: Line) -> AntiScalar {
    var self_1421: Line;

    self_1421 = self_1420;
    let _e2: Line = self_1421;
    let _e3: AntiScalar = line_squared_anti_magnitude(_e2);
    return AntiScalar(sqrt(_e3.g0_));
}

fn line_scale(self_1422: Line, other_1180: f32) -> Line {
    var self_1423: Line;
    var other_1181: f32;

    self_1423 = self_1422;
    other_1181 = other_1180;
    let _e4: Line = self_1423;
    let _e5: f32 = other_1181;
    let _e7: Line = line_scalar_geometric_product(_e4, Scalar(_e5));
    return _e7;
}

fn line_signum(self_1424: Line) -> Line {
    var self_1425: Line;

    self_1425 = self_1424;
    let _e2: Line = self_1425;
    let _e3: Line = self_1425;
    let _e4: Scalar = line_magnitude(_e3);
    let _e9: Line = line_scalar_geometric_product(_e2, Scalar((1.0 / _e4.g0_)));
    return _e9;
}

fn line_inverse(self_1426: Line) -> Line {
    var self_1427: Line;

    self_1427 = self_1426;
    let _e2: Line = self_1427;
    let _e3: Line = line_reversal(_e2);
    let _e4: Line = self_1427;
    let _e5: Scalar = line_squared_magnitude(_e4);
    let _e10: Line = line_scalar_geometric_product(_e3, Scalar((1.0 / _e5.g0_)));
    return _e10;
}

fn line_unitize(self_1428: Line) -> Line {
    var self_1429: Line;

    self_1429 = self_1428;
    let _e2: Line = self_1429;
    let _e3: Line = self_1429;
    let _e4: AntiScalar = line_weight_norm(_e3);
    let _e9: Line = line_scalar_geometric_product(_e2, Scalar((1.0 / _e4.g0_)));
    return _e9;
}

fn translator_zero() -> Translator {
    return Translator(vec4<f32>(0.0));
}

fn translator_one() -> Translator {
    return Translator(vec4<f32>(1.0, 0.0, 0.0, 0.0));
}

fn translator_neg(self_1430: Translator) -> Translator {
    var self_1431: Translator;

    self_1431 = self_1430;
    let _e2: Translator = self_1431;
    return Translator((_e2.g0_ * vec4<f32>(-(1.0))));
}

fn translator_automorphism(self_1432: Translator) -> Translator {
    var self_1433: Translator;

    self_1433 = self_1432;
    let _e2: Translator = self_1433;
    return Translator(_e2.g0_);
}

fn translator_reversal(self_1434: Translator) -> Translator {
    var self_1435: Translator;

    self_1435 = self_1434;
    let _e2: Translator = self_1435;
    return Translator((_e2.g0_ * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))));
}

fn translator_conjugation(self_1436: Translator) -> Translator {
    var self_1437: Translator;

    self_1437 = self_1436;
    let _e2: Translator = self_1437;
    return Translator((_e2.g0_ * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))));
}

fn translator_anti_reversal(self_1438: Translator) -> Translator {
    var self_1439: Translator;

    self_1439 = self_1438;
    let _e2: Translator = self_1439;
    return Translator((_e2.g0_ * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))));
}

fn translator_double_complement(self_1440: Translator) -> Translator {
    var self_1441: Translator;

    self_1441 = self_1440;
    let _e2: Translator = self_1441;
    return Translator(_e2.g0_);
}

fn translator_scalar_into(self_1442: Translator) -> Scalar {
    var self_1443: Translator;

    self_1443 = self_1442;
    let _e2: Translator = self_1443;
    return Scalar(_e2.g0_.x);
}

fn translator_scalar_add(self_1444: Translator, other_1182: Scalar) -> Translator {
    var self_1445: Translator;
    var other_1183: Scalar;

    self_1445 = self_1444;
    other_1183 = other_1182;
    let _e4: Translator = self_1445;
    let _e6: Scalar = other_1183;
    return Translator((_e4.g0_ + (vec4<f32>(_e6.g0_) * vec4<f32>(1.0, 0.0, 0.0, 0.0))));
}

fn translator_scalar_sub(self_1446: Translator, other_1184: Scalar) -> Translator {
    var self_1447: Translator;
    var other_1185: Scalar;

    self_1447 = self_1446;
    other_1185 = other_1184;
    let _e4: Translator = self_1447;
    let _e6: Scalar = other_1185;
    return Translator((_e4.g0_ - (vec4<f32>(_e6.g0_) * vec4<f32>(1.0, 0.0, 0.0, 0.0))));
}

fn translator_scalar_geometric_product(self_1448: Translator, other_1186: Scalar) -> Translator {
    var self_1449: Translator;
    var other_1187: Scalar;

    self_1449 = self_1448;
    other_1187 = other_1186;
    let _e4: Translator = self_1449;
    let _e6: Scalar = other_1187;
    return Translator((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn translator_scalar_outer_product(self_1450: Translator, other_1188: Scalar) -> Translator {
    var self_1451: Translator;
    var other_1189: Scalar;

    self_1451 = self_1450;
    other_1189 = other_1188;
    let _e4: Translator = self_1451;
    let _e6: Scalar = other_1189;
    return Translator((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn translator_scalar_inner_product(self_1452: Translator, other_1190: Scalar) -> Translator {
    var self_1453: Translator;
    var other_1191: Scalar;

    self_1453 = self_1452;
    other_1191 = other_1190;
    let _e4: Translator = self_1453;
    let _e6: Scalar = other_1191;
    return Translator((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn translator_scalar_left_contraction(self_1454: Translator, other_1192: Scalar) -> Scalar {
    var self_1455: Translator;
    var other_1193: Scalar;

    self_1455 = self_1454;
    other_1193 = other_1192;
    let _e4: Translator = self_1455;
    let _e7: Scalar = other_1193;
    return Scalar((_e4.g0_.x * _e7.g0_));
}

fn translator_scalar_right_contraction(self_1456: Translator, other_1194: Scalar) -> Translator {
    var self_1457: Translator;
    var other_1195: Scalar;

    self_1457 = self_1456;
    other_1195 = other_1194;
    let _e4: Translator = self_1457;
    let _e6: Scalar = other_1195;
    return Translator((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn translator_scalar_scalar_product(self_1458: Translator, other_1196: Scalar) -> Scalar {
    var self_1459: Translator;
    var other_1197: Scalar;

    self_1459 = self_1458;
    other_1197 = other_1196;
    let _e4: Translator = self_1459;
    let _e7: Scalar = other_1197;
    return Scalar((_e4.g0_.x * _e7.g0_));
}

fn translator_anti_scalar_geometric_product(self_1460: Translator, other_1198: AntiScalar) -> AntiScalar {
    var self_1461: Translator;
    var other_1199: AntiScalar;

    self_1461 = self_1460;
    other_1199 = other_1198;
    let _e4: Translator = self_1461;
    let _e7: AntiScalar = other_1199;
    return AntiScalar((_e4.g0_.x * _e7.g0_));
}

fn translator_anti_scalar_regressive_product(self_1462: Translator, other_1200: AntiScalar) -> Translator {
    var self_1463: Translator;
    var other_1201: AntiScalar;

    self_1463 = self_1462;
    other_1201 = other_1200;
    let _e4: Translator = self_1463;
    let _e6: AntiScalar = other_1201;
    return Translator((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn translator_anti_scalar_outer_product(self_1464: Translator, other_1202: AntiScalar) -> AntiScalar {
    var self_1465: Translator;
    var other_1203: AntiScalar;

    self_1465 = self_1464;
    other_1203 = other_1202;
    let _e4: Translator = self_1465;
    let _e7: AntiScalar = other_1203;
    return AntiScalar((_e4.g0_.x * _e7.g0_));
}

fn translator_anti_scalar_inner_product(self_1466: Translator, other_1204: AntiScalar) -> AntiScalar {
    var self_1467: Translator;
    var other_1205: AntiScalar;

    self_1467 = self_1466;
    other_1205 = other_1204;
    let _e4: Translator = self_1467;
    let _e7: AntiScalar = other_1205;
    return AntiScalar((_e4.g0_.x * _e7.g0_));
}

fn translator_anti_scalar_geometric_anti_product(self_1468: Translator, other_1206: AntiScalar) -> Translator {
    var self_1469: Translator;
    var other_1207: AntiScalar;

    self_1469 = self_1468;
    other_1207 = other_1206;
    let _e4: Translator = self_1469;
    let _e6: AntiScalar = other_1207;
    return Translator((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn translator_anti_scalar_inner_anti_product(self_1470: Translator, other_1208: AntiScalar) -> Translator {
    var self_1471: Translator;
    var other_1209: AntiScalar;

    self_1471 = self_1470;
    other_1209 = other_1208;
    let _e4: Translator = self_1471;
    let _e6: AntiScalar = other_1209;
    return Translator((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn translator_anti_scalar_left_contraction(self_1472: Translator, other_1210: AntiScalar) -> AntiScalar {
    var self_1473: Translator;
    var other_1211: AntiScalar;

    self_1473 = self_1472;
    other_1211 = other_1210;
    let _e4: Translator = self_1473;
    let _e7: AntiScalar = other_1211;
    return AntiScalar((_e4.g0_.x * _e7.g0_));
}

fn translator_anti_scalar_right_anti_contraction(self_1474: Translator, other_1212: AntiScalar) -> Translator {
    var self_1475: Translator;
    var other_1213: AntiScalar;

    self_1475 = self_1474;
    other_1213 = other_1212;
    let _e4: Translator = self_1475;
    let _e6: AntiScalar = other_1213;
    return Translator((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn translator_multi_vector_add(self_1476: Translator, other_1214: MultiVector) -> MultiVector {
    var self_1477: Translator;
    var other_1215: MultiVector;

    self_1477 = self_1476;
    other_1215 = other_1214;
    let _e4: Translator = self_1477;
    let _e14: MultiVector = other_1215;
    let _e17: MultiVector = other_1215;
    let _e19: MultiVector = other_1215;
    let _e21: Translator = self_1477;
    let _e29: MultiVector = other_1215;
    return MultiVector(((vec4<f32>(_e4.g0_.x) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) + _e14.g0_), _e17.g1_, _e19.g2_, ((_e21.g0_ * vec4<f32>(0.0, 1.0, 1.0, 1.0)) + _e29.g3_));
}

fn translator_multi_vector_sub(self_1478: Translator, other_1216: MultiVector) -> MultiVector {
    var self_1479: Translator;
    var other_1217: MultiVector;

    self_1479 = self_1478;
    other_1217 = other_1216;
    let _e4: Translator = self_1479;
    let _e14: MultiVector = other_1217;
    let _e19: MultiVector = other_1217;
    let _e24: MultiVector = other_1217;
    let _e27: Translator = self_1479;
    let _e35: MultiVector = other_1217;
    return MultiVector(((vec4<f32>(_e4.g0_.x) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) - _e14.g0_), (vec4<f32>(0.0) - _e19.g1_), (vec4<f32>(0.0) - _e24.g2_), ((_e27.g0_ * vec4<f32>(0.0, 1.0, 1.0, 1.0)) - _e35.g3_));
}

fn translator_multi_vector_geometric_product(self_1480: Translator, other_1218: MultiVector) -> MultiVector {
    var self_1481: Translator;
    var other_1219: MultiVector;

    self_1481 = self_1480;
    other_1219 = other_1218;
    let _e4: Translator = self_1481;
    let _e8: MultiVector = other_1219;
    let _e11: Translator = self_1481;
    let _e15: MultiVector = other_1219;
    let _e18: Translator = self_1481;
    let _e22: MultiVector = other_1219;
    let _e35: Translator = self_1481;
    let _e39: MultiVector = other_1219;
    let _e52: Translator = self_1481;
    let _e56: MultiVector = other_1219;
    let _e69: Translator = self_1481;
    let _e73: MultiVector = other_1219;
    let _e76: Translator = self_1481;
    let _e80: MultiVector = other_1219;
    let _e83: Translator = self_1481;
    let _e87: MultiVector = other_1219;
    let _e99: Translator = self_1481;
    let _e103: MultiVector = other_1219;
    let _e115: Translator = self_1481;
    let _e119: MultiVector = other_1219;
    return MultiVector((vec4<f32>(_e4.g0_.x) * _e8.g0_), ((((vec4<f32>(_e11.g0_.x) * _e15.g1_) + ((vec4<f32>(_e18.g0_.y) * _e22.g2_.yxwz) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e35.g0_.z) * _e39.g2_.zwxy) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e52.g0_.w) * _e56.g2_.wzyx) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))), (vec4<f32>(_e69.g0_.x) * _e73.g2_), ((((vec4<f32>(_e76.g0_.x) * _e80.g3_) + ((vec4<f32>(_e83.g0_.y) * _e87.g0_.yxwz) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e99.g0_.z) * _e103.g0_.zwxy) * vec4<f32>(1.0, -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e115.g0_.w) * _e119.g0_.wzyx) * vec4<f32>(1.0, 1.0, -(1.0), 1.0))));
}

fn translator_multi_vector_outer_product(self_1482: Translator, other_1220: MultiVector) -> MultiVector {
    var self_1483: Translator;
    var other_1221: MultiVector;

    self_1483 = self_1482;
    other_1221 = other_1220;
    let _e4: Translator = self_1483;
    let _e8: MultiVector = other_1221;
    let _e11: Translator = self_1483;
    let _e15: MultiVector = other_1221;
    let _e18: Translator = self_1483;
    let _e22: MultiVector = other_1221;
    let _e34: Translator = self_1483;
    let _e38: MultiVector = other_1221;
    let _e50: Translator = self_1483;
    let _e53: MultiVector = other_1221;
    let _e65: Translator = self_1483;
    let _e69: MultiVector = other_1221;
    let _e72: Translator = self_1483;
    let _e76: MultiVector = other_1221;
    let _e79: Translator = self_1483;
    let _e83: MultiVector = other_1221;
    let _e94: Translator = self_1483;
    let _e98: MultiVector = other_1221;
    let _e109: Translator = self_1483;
    let _e112: MultiVector = other_1221;
    return MultiVector((vec4<f32>(_e4.g0_.x) * _e8.g0_), ((((vec4<f32>(_e11.g0_.x) * _e15.g1_) + ((vec4<f32>(_e18.g0_.z) * _e22.g2_.wwwy) * vec4<f32>(0.0, -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e34.g0_.w) * _e38.g2_.zzyz) * vec4<f32>(0.0, 1.0, -(1.0), 0.0))) + ((_e50.g0_.xxyy * _e53.g2_.xxwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))), (vec4<f32>(_e65.g0_.x) * _e69.g2_), ((((vec4<f32>(_e72.g0_.x) * _e76.g3_) + ((vec4<f32>(_e79.g0_.z) * _e83.g0_.zzxz) * vec4<f32>(1.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e94.g0_.w) * _e98.g0_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, 1.0))) + ((_e109.g0_.yyxx * _e112.g0_.yxxx) * vec4<f32>(1.0, 1.0, 0.0, 0.0))));
}

fn translator_multi_vector_inner_product(self_1484: Translator, other_1222: MultiVector) -> MultiVector {
    var self_1485: Translator;
    var other_1223: MultiVector;

    self_1485 = self_1484;
    other_1223 = other_1222;
    let _e4: Translator = self_1485;
    let _e8: MultiVector = other_1223;
    let _e11: Translator = self_1485;
    let _e15: MultiVector = other_1223;
    let _e18: Translator = self_1485;
    let _e22: MultiVector = other_1223;
    let _e34: Translator = self_1485;
    let _e38: MultiVector = other_1223;
    let _e50: Translator = self_1485;
    let _e53: MultiVector = other_1223;
    let _e64: Translator = self_1485;
    let _e68: MultiVector = other_1223;
    let _e71: Translator = self_1485;
    let _e75: MultiVector = other_1223;
    let _e78: Translator = self_1485;
    let _e80: MultiVector = other_1223;
    return MultiVector((vec4<f32>(_e4.g0_.x) * _e8.g0_), ((((vec4<f32>(_e11.g0_.x) * _e15.g1_) + ((vec4<f32>(_e18.g0_.z) * vec4<f32>(_e22.g2_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e34.g0_.w) * vec4<f32>(_e38.g2_.w)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((_e50.g0_.yxxx * _e53.g2_.yxxx) * vec4<f32>(1.0, 0.0, 0.0, 0.0))), (vec4<f32>(_e64.g0_.x) * _e68.g2_), ((vec4<f32>(_e71.g0_.x) * _e75.g3_) + ((_e78.g0_ * vec4<f32>(_e80.g0_.x)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn translator_multi_vector_geometric_anti_product(self_1486: Translator, other_1224: MultiVector) -> MultiVector {
    var self_1487: Translator;
    var other_1225: MultiVector;

    self_1487 = self_1486;
    other_1225 = other_1224;
    let _e4: Translator = self_1487;
    let _e8: MultiVector = other_1225;
    let _e20: Translator = self_1487;
    let _e24: MultiVector = other_1225;
    let _e37: Translator = self_1487;
    let _e41: MultiVector = other_1225;
    let _e54: Translator = self_1487;
    let _e58: MultiVector = other_1225;
    let _e71: Translator = self_1487;
    let _e75: MultiVector = other_1225;
    let _e87: Translator = self_1487;
    let _e91: MultiVector = other_1225;
    let _e104: Translator = self_1487;
    let _e108: MultiVector = other_1225;
    let _e121: Translator = self_1487;
    let _e125: MultiVector = other_1225;
    let _e128: Translator = self_1487;
    let _e132: MultiVector = other_1225;
    let _e145: Translator = self_1487;
    let _e149: MultiVector = other_1225;
    let _e162: Translator = self_1487;
    let _e166: MultiVector = other_1225;
    let _e179: Translator = self_1487;
    let _e183: MultiVector = other_1225;
    let _e195: Translator = self_1487;
    let _e199: MultiVector = other_1225;
    let _e212: Translator = self_1487;
    let _e216: MultiVector = other_1225;
    return MultiVector((((((vec4<f32>(_e4.g0_.x) * _e8.g3_) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))) + ((vec4<f32>(_e20.g0_.y) * _e24.g0_.yxwz) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e37.g0_.z) * _e41.g0_.zwxy) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e54.g0_.w) * _e58.g0_.wzyx) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))), ((((vec4<f32>(_e71.g0_.y) * _e75.g1_.yxwz) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0))) + ((vec4<f32>(_e87.g0_.z) * _e91.g1_.zwxy) * vec4<f32>(-(1.0), -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e104.g0_.w) * _e108.g1_.wzyx) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))), ((((vec4<f32>(_e121.g0_.x) * _e125.g1_) + ((vec4<f32>(_e128.g0_.y) * _e132.g2_.yxwz) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e145.g0_.z) * _e149.g2_.zwxy) * vec4<f32>(-(1.0), -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e162.g0_.w) * _e166.g2_.wzyx) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))), ((((vec4<f32>(_e179.g0_.y) * _e183.g3_.yxwz) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0))) + ((vec4<f32>(_e195.g0_.z) * _e199.g3_.zwxy) * vec4<f32>(-(1.0), -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e212.g0_.w) * _e216.g3_.wzyx) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))));
}

fn translator_multi_vector_left_contraction(self_1488: Translator, other_1226: MultiVector) -> MultiVector {
    var self_1489: Translator;
    var other_1227: MultiVector;

    self_1489 = self_1488;
    other_1227 = other_1226;
    let _e4: Translator = self_1489;
    let _e8: MultiVector = other_1227;
    let _e11: Translator = self_1489;
    let _e15: MultiVector = other_1227;
    let _e18: Translator = self_1489;
    let _e22: MultiVector = other_1227;
    let _e25: Translator = self_1489;
    let _e29: MultiVector = other_1227;
    return MultiVector((vec4<f32>(_e4.g0_.x) * _e8.g0_), (vec4<f32>(_e11.g0_.x) * _e15.g1_), (vec4<f32>(_e18.g0_.x) * _e22.g2_), (vec4<f32>(_e25.g0_.x) * _e29.g3_));
}

fn translator_multi_vector_scalar_product(self_1490: Translator, other_1228: MultiVector) -> Scalar {
    var self_1491: Translator;
    var other_1229: MultiVector;

    self_1491 = self_1490;
    other_1229 = other_1228;
    let _e4: Translator = self_1491;
    let _e7: MultiVector = other_1229;
    return Scalar((_e4.g0_.x * _e7.g0_.x));
}

fn translator_multi_vector_anti_scalar_product(self_1492: Translator, other_1230: MultiVector) -> AntiScalar {
    var self_1493: Translator;
    var other_1231: MultiVector;

    self_1493 = self_1492;
    other_1231 = other_1230;
    let _e5: Translator = self_1493;
    let _e8: MultiVector = other_1231;
    let _e13: Translator = self_1493;
    let _e16: MultiVector = other_1231;
    let _e21: Translator = self_1493;
    let _e24: MultiVector = other_1231;
    return AntiScalar((((0.0 - (_e5.g0_.y * _e8.g3_.y)) - (_e13.g0_.z * _e16.g3_.z)) - (_e21.g0_.w * _e24.g3_.w)));
}

fn translator_rotor_geometric_product(self_1494: Translator, other_1232: Rotor) -> Motor {
    var self_1495: Translator;
    var other_1233: Rotor;

    self_1495 = self_1494;
    other_1233 = other_1232;
    let _e4: Translator = self_1495;
    let _e8: Rotor = other_1233;
    let _e11: Translator = self_1495;
    let _e15: Rotor = other_1233;
    let _e26: Translator = self_1495;
    let _e30: Rotor = other_1233;
    let _e42: Translator = self_1495;
    let _e46: Rotor = other_1233;
    return Motor((vec4<f32>(_e4.g0_.x) * _e8.g0_), ((((vec4<f32>(_e11.g0_.y) * _e15.g0_.yxwz) * vec4<f32>(1.0, 1.0, 1.0, -(1.0))) + ((vec4<f32>(_e26.g0_.z) * _e30.g0_.zwxy) * vec4<f32>(1.0, -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e42.g0_.w) * _e46.g0_.wzyx) * vec4<f32>(1.0, 1.0, -(1.0), 1.0))));
}

fn translator_rotor_regressive_product(self_1496: Translator, other_1234: Rotor) -> Scalar {
    var self_1497: Translator;
    var other_1235: Rotor;

    self_1497 = self_1496;
    other_1235 = other_1234;
    let _e4: Translator = self_1497;
    let _e7: Rotor = other_1235;
    let _e11: Translator = self_1497;
    let _e14: Rotor = other_1235;
    let _e19: Translator = self_1497;
    let _e22: Rotor = other_1235;
    return Scalar((((_e4.g0_.y * _e7.g0_.y) + (_e11.g0_.z * _e14.g0_.z)) + (_e19.g0_.w * _e22.g0_.w)));
}

fn translator_rotor_outer_product(self_1498: Translator, other_1236: Rotor) -> Motor {
    var self_1499: Translator;
    var other_1237: Rotor;

    self_1499 = self_1498;
    other_1237 = other_1236;
    let _e4: Translator = self_1499;
    let _e8: Rotor = other_1237;
    let _e11: Translator = self_1499;
    let _e15: Rotor = other_1237;
    let _e25: Translator = self_1499;
    let _e29: Rotor = other_1237;
    let _e40: Translator = self_1499;
    let _e43: Rotor = other_1237;
    return Motor((vec4<f32>(_e4.g0_.x) * _e8.g0_), ((((vec4<f32>(_e11.g0_.z) * _e15.g0_.zzxz) * vec4<f32>(1.0, 0.0, 1.0, 0.0)) + ((vec4<f32>(_e25.g0_.w) * _e29.g0_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, 1.0))) + ((_e40.g0_.yyxx * _e43.g0_.yxxx) * vec4<f32>(1.0, 1.0, 0.0, 0.0))));
}

fn translator_rotor_geometric_anti_product(self_1500: Translator, other_1238: Rotor) -> Rotor {
    var self_1501: Translator;
    var other_1239: Rotor;

    self_1501 = self_1500;
    other_1239 = other_1238;
    let _e4: Translator = self_1501;
    let _e8: Rotor = other_1239;
    let _e20: Translator = self_1501;
    let _e24: Rotor = other_1239;
    let _e37: Translator = self_1501;
    let _e41: Rotor = other_1239;
    return Rotor(((((vec4<f32>(_e4.g0_.y) * _e8.g0_.yxwz) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0))) + ((vec4<f32>(_e20.g0_.z) * _e24.g0_.zwxy) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e37.g0_.w) * _e41.g0_.wzyx) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))));
}

fn translator_rotor_left_contraction(self_1502: Translator, other_1240: Rotor) -> Rotor {
    var self_1503: Translator;
    var other_1241: Rotor;

    self_1503 = self_1502;
    other_1241 = other_1240;
    let _e4: Translator = self_1503;
    let _e8: Rotor = other_1241;
    return Rotor((vec4<f32>(_e4.g0_.x) * _e8.g0_));
}

fn translator_rotor_right_contraction(self_1504: Translator, other_1242: Rotor) -> Translator {
    var self_1505: Translator;
    var other_1243: Rotor;

    self_1505 = self_1504;
    other_1243 = other_1242;
    let _e4: Translator = self_1505;
    let _e6: Rotor = other_1243;
    return Translator((_e4.g0_ * vec4<f32>(_e6.g0_.x)));
}

fn translator_rotor_scalar_product(self_1506: Translator, other_1244: Rotor) -> Scalar {
    var self_1507: Translator;
    var other_1245: Rotor;

    self_1507 = self_1506;
    other_1245 = other_1244;
    let _e4: Translator = self_1507;
    let _e7: Rotor = other_1245;
    return Scalar((_e4.g0_.x * _e7.g0_.x));
}

fn translator_point_geometric_product(self_1508: Translator, other_1246: Point) -> Point {
    var self_1509: Translator;
    var other_1247: Point;

    self_1509 = self_1508;
    other_1247 = other_1246;
    let _e4: Translator = self_1509;
    let _e8: Point = other_1247;
    let _e11: Translator = self_1509;
    let _e13: Point = other_1247;
    return Point(((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((_e11.g0_ * vec4<f32>(_e13.g0_.x)) * vec4<f32>(0.0, -(1.0), -(1.0), -(1.0)))));
}

fn translator_point_regressive_product(self_1510: Translator, other_1248: Point) -> Plane {
    var self_1511: Translator;
    var other_1249: Point;

    self_1511 = self_1510;
    other_1249 = other_1248;
    let _e4: Translator = self_1511;
    let _e8: Point = other_1249;
    let _e19: Translator = self_1511;
    let _e23: Point = other_1249;
    let _e35: Translator = self_1511;
    let _e38: Point = other_1249;
    return Plane(((((vec4<f32>(_e4.g0_.z) * _e8.g0_.zzxz) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0)) + ((vec4<f32>(_e19.g0_.w) * _e23.g0_.wwwx) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))) + ((_e35.g0_.yyxx * _e38.g0_.yxxx) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))));
}

fn translator_point_outer_product(self_1512: Translator, other_1250: Point) -> Point {
    var self_1513: Translator;
    var other_1251: Point;

    self_1513 = self_1512;
    other_1251 = other_1250;
    let _e4: Translator = self_1513;
    let _e8: Point = other_1251;
    return Point((vec4<f32>(_e4.g0_.x) * _e8.g0_));
}

fn translator_point_inner_product(self_1514: Translator, other_1252: Point) -> Point {
    var self_1515: Translator;
    var other_1253: Point;

    self_1515 = self_1514;
    other_1253 = other_1252;
    let _e4: Translator = self_1515;
    let _e8: Point = other_1253;
    return Point((vec4<f32>(_e4.g0_.x) * _e8.g0_));
}

fn translator_point_left_contraction(self_1516: Translator, other_1254: Point) -> Point {
    var self_1517: Translator;
    var other_1255: Point;

    self_1517 = self_1516;
    other_1255 = other_1254;
    let _e4: Translator = self_1517;
    let _e8: Point = other_1255;
    return Point((vec4<f32>(_e4.g0_.x) * _e8.g0_));
}

fn translator_ideal_point_into(self_1518: Translator) -> IdealPoint {
    var self_1519: Translator;

    self_1519 = self_1518;
    let _e2: Translator = self_1519;
    let _e5: Translator = self_1519;
    let _e8: Translator = self_1519;
    return IdealPoint(vec3<f32>(_e2.g0_.y, _e5.g0_.z, _e8.g0_.w));
}

fn translator_ideal_point_add(self_1520: Translator, other_1256: IdealPoint) -> Translator {
    var self_1521: Translator;
    var other_1257: IdealPoint;

    self_1521 = self_1520;
    other_1257 = other_1256;
    let _e4: Translator = self_1521;
    let _e6: IdealPoint = other_1257;
    let _e9: IdealPoint = other_1257;
    let _e12: IdealPoint = other_1257;
    let _e15: IdealPoint = other_1257;
    return Translator((_e4.g0_ + (vec4<f32>(_e6.g0_.x, _e9.g0_.x, _e12.g0_.y, _e15.g0_.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn translator_ideal_point_sub(self_1522: Translator, other_1258: IdealPoint) -> Translator {
    var self_1523: Translator;
    var other_1259: IdealPoint;

    self_1523 = self_1522;
    other_1259 = other_1258;
    let _e4: Translator = self_1523;
    let _e6: IdealPoint = other_1259;
    let _e9: IdealPoint = other_1259;
    let _e12: IdealPoint = other_1259;
    let _e15: IdealPoint = other_1259;
    return Translator((_e4.g0_ - (vec4<f32>(_e6.g0_.x, _e9.g0_.x, _e12.g0_.y, _e15.g0_.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn translator_ideal_point_geometric_product(self_1524: Translator, other_1260: IdealPoint) -> IdealPoint {
    var self_1525: Translator;
    var other_1261: IdealPoint;

    self_1525 = self_1524;
    other_1261 = other_1260;
    let _e4: Translator = self_1525;
    let _e8: IdealPoint = other_1261;
    return IdealPoint((vec3<f32>(_e4.g0_.x) * _e8.g0_));
}

fn translator_ideal_point_outer_product(self_1526: Translator, other_1262: IdealPoint) -> IdealPoint {
    var self_1527: Translator;
    var other_1263: IdealPoint;

    self_1527 = self_1526;
    other_1263 = other_1262;
    let _e4: Translator = self_1527;
    let _e8: IdealPoint = other_1263;
    return IdealPoint((vec3<f32>(_e4.g0_.x) * _e8.g0_));
}

fn translator_ideal_point_inner_product(self_1528: Translator, other_1264: IdealPoint) -> IdealPoint {
    var self_1529: Translator;
    var other_1265: IdealPoint;

    self_1529 = self_1528;
    other_1265 = other_1264;
    let _e4: Translator = self_1529;
    let _e8: IdealPoint = other_1265;
    return IdealPoint((vec3<f32>(_e4.g0_.x) * _e8.g0_));
}

fn translator_ideal_point_left_contraction(self_1530: Translator, other_1266: IdealPoint) -> IdealPoint {
    var self_1531: Translator;
    var other_1267: IdealPoint;

    self_1531 = self_1530;
    other_1267 = other_1266;
    let _e4: Translator = self_1531;
    let _e8: IdealPoint = other_1267;
    return IdealPoint((vec3<f32>(_e4.g0_.x) * _e8.g0_));
}

fn translator_ideal_point_left_anti_contraction(self_1532: Translator, other_1268: IdealPoint) -> AntiScalar {
    var self_1533: Translator;
    var other_1269: IdealPoint;

    self_1533 = self_1532;
    other_1269 = other_1268;
    let _e5: Translator = self_1533;
    let _e8: IdealPoint = other_1269;
    let _e13: Translator = self_1533;
    let _e16: IdealPoint = other_1269;
    let _e21: Translator = self_1533;
    let _e24: IdealPoint = other_1269;
    return AntiScalar((((0.0 - (_e5.g0_.y * _e8.g0_.x)) - (_e13.g0_.z * _e16.g0_.y)) - (_e21.g0_.w * _e24.g0_.z)));
}

fn translator_ideal_point_anti_scalar_product(self_1534: Translator, other_1270: IdealPoint) -> AntiScalar {
    var self_1535: Translator;
    var other_1271: IdealPoint;

    self_1535 = self_1534;
    other_1271 = other_1270;
    let _e5: Translator = self_1535;
    let _e8: IdealPoint = other_1271;
    let _e13: Translator = self_1535;
    let _e16: IdealPoint = other_1271;
    let _e21: Translator = self_1535;
    let _e24: IdealPoint = other_1271;
    return AntiScalar((((0.0 - (_e5.g0_.y * _e8.g0_.x)) - (_e13.g0_.z * _e16.g0_.y)) - (_e21.g0_.w * _e24.g0_.z)));
}

fn translator_plane_inner_product(self_1536: Translator, other_1272: Plane) -> Plane {
    var self_1537: Translator;
    var other_1273: Plane;

    self_1537 = self_1536;
    other_1273 = other_1272;
    let _e4: Translator = self_1537;
    let _e8: Plane = other_1273;
    let _e11: Translator = self_1537;
    let _e15: Plane = other_1273;
    let _e27: Translator = self_1537;
    let _e31: Plane = other_1273;
    let _e43: Translator = self_1537;
    let _e46: Plane = other_1273;
    return Plane(((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.z) * vec4<f32>(_e15.g0_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e27.g0_.w) * vec4<f32>(_e31.g0_.w)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((_e43.g0_.yxxx * _e46.g0_.yxxx) * vec4<f32>(1.0, 0.0, 0.0, 0.0))));
}

fn translator_plane_inner_anti_product(self_1538: Translator, other_1274: Plane) -> Point {
    var self_1539: Translator;
    var other_1275: Plane;

    self_1539 = self_1538;
    other_1275 = other_1274;
    let _e4: Translator = self_1539;
    let _e8: Plane = other_1275;
    let _e19: Translator = self_1539;
    let _e23: Plane = other_1275;
    let _e35: Translator = self_1539;
    let _e39: Plane = other_1275;
    let _e51: Translator = self_1539;
    let _e55: Plane = other_1275;
    return Point((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0)) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.zzxz) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0))) + ((vec4<f32>(_e35.g0_.w) * _e39.g0_.wwwx) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))) + ((vec4<f32>(_e51.g0_.x) * vec4<f32>(_e55.g0_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))));
}

fn translator_plane_left_contraction(self_1540: Translator, other_1276: Plane) -> Plane {
    var self_1541: Translator;
    var other_1277: Plane;

    self_1541 = self_1540;
    other_1277 = other_1276;
    let _e4: Translator = self_1541;
    let _e8: Plane = other_1277;
    return Plane((vec4<f32>(_e4.g0_.x) * _e8.g0_));
}

fn translator_plane_left_anti_contraction(self_1542: Translator, other_1278: Plane) -> Point {
    var self_1543: Translator;
    var other_1279: Plane;

    self_1543 = self_1542;
    other_1279 = other_1278;
    let _e4: Translator = self_1543;
    let _e8: Plane = other_1279;
    let _e19: Translator = self_1543;
    let _e23: Plane = other_1279;
    let _e35: Translator = self_1543;
    let _e38: Plane = other_1279;
    return Point(((((vec4<f32>(_e4.g0_.z) * _e8.g0_.zzxz) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0)) + ((vec4<f32>(_e19.g0_.w) * _e23.g0_.wwwx) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))) + ((_e35.g0_.yyxx * _e38.g0_.yxxx) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))));
}

fn translator_line_regressive_product(self_1544: Translator, other_1280: Line) -> Scalar {
    var self_1545: Translator;
    var other_1281: Line;

    self_1545 = self_1544;
    other_1281 = other_1280;
    let _e4: Translator = self_1545;
    let _e7: Line = other_1281;
    let _e11: Translator = self_1545;
    let _e14: Line = other_1281;
    let _e19: Translator = self_1545;
    let _e22: Line = other_1281;
    return Scalar((((_e4.g0_.y * _e7.g1_.x) + (_e11.g0_.z * _e14.g1_.y)) + (_e19.g0_.w * _e22.g1_.z)));
}

fn translator_line_inner_product(self_1546: Translator, other_1282: Line) -> Line {
    var self_1547: Translator;
    var other_1283: Line;

    self_1547 = self_1546;
    other_1283 = other_1282;
    let _e4: Translator = self_1547;
    let _e8: Line = other_1283;
    let _e11: Translator = self_1547;
    let _e15: Line = other_1283;
    return Line((vec3<f32>(_e4.g0_.x) * _e8.g0_), (vec3<f32>(_e11.g0_.x) * _e15.g1_));
}

fn translator_line_geometric_anti_product(self_1548: Translator, other_1284: Line) -> Motor {
    var self_1549: Translator;
    var other_1285: Line;

    self_1549 = self_1548;
    other_1285 = other_1284;
    let _e4: Translator = self_1549;
    let _e8: Line = other_1285;
    let _e11: Line = other_1285;
    let _e14: Line = other_1285;
    let _e17: Line = other_1285;
    let _e29: Translator = self_1549;
    let _e33: Line = other_1285;
    let _e36: Line = other_1285;
    let _e39: Line = other_1285;
    let _e42: Line = other_1285;
    let _e55: Translator = self_1549;
    let _e59: Line = other_1285;
    let _e62: Line = other_1285;
    let _e65: Line = other_1285;
    let _e68: Line = other_1285;
    let _e81: Translator = self_1549;
    let _e85: Line = other_1285;
    let _e88: Line = other_1285;
    let _e91: Line = other_1285;
    let _e94: Line = other_1285;
    let _e109: Translator = self_1549;
    let _e113: Line = other_1285;
    let _e116: Line = other_1285;
    let _e119: Line = other_1285;
    let _e122: Line = other_1285;
    let _e135: Translator = self_1549;
    let _e139: Line = other_1285;
    let _e142: Line = other_1285;
    let _e145: Line = other_1285;
    let _e148: Line = other_1285;
    let _e162: Translator = self_1549;
    let _e165: Line = other_1285;
    let _e168: Line = other_1285;
    let _e171: Line = other_1285;
    let _e174: Line = other_1285;
    return Motor((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g1_.x, _e11.g1_.x, _e14.g1_.z, _e17.g1_.y)) * vec4<f32>(1.0, 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e29.g0_.z) * vec4<f32>(_e33.g1_.y, _e36.g1_.z, _e39.g1_.y, _e42.g1_.x)) * vec4<f32>(1.0, -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e55.g0_.w) * vec4<f32>(_e59.g1_.z, _e62.g1_.y, _e65.g1_.x, _e68.g1_.z)) * vec4<f32>(1.0, 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e81.g0_.x) * vec4<f32>(_e85.g0_.x, _e88.g0_.x, _e91.g0_.y, _e94.g0_.z)) * vec4<f32>(0.0, -(1.0), -(1.0), -(1.0)))), ((((vec4<f32>(_e109.g0_.z) * vec4<f32>(_e113.g0_.y, _e116.g0_.z, _e119.g0_.y, _e122.g0_.x)) * vec4<f32>(-(1.0), -(1.0), 0.0, 1.0)) + ((vec4<f32>(_e135.g0_.w) * vec4<f32>(_e139.g0_.z, _e142.g0_.y, _e145.g0_.x, _e148.g0_.z)) * vec4<f32>(-(1.0), 1.0, -(1.0), 0.0))) + ((_e162.g0_.yxyy * vec4<f32>(_e165.g0_.x, _e168.g0_.x, _e171.g0_.z, _e174.g0_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0)))));
}

fn translator_line_left_contraction(self_1550: Translator, other_1286: Line) -> Line {
    var self_1551: Translator;
    var other_1287: Line;

    self_1551 = self_1550;
    other_1287 = other_1286;
    let _e4: Translator = self_1551;
    let _e8: Line = other_1287;
    let _e11: Translator = self_1551;
    let _e15: Line = other_1287;
    return Line((vec3<f32>(_e4.g0_.x) * _e8.g0_), (vec3<f32>(_e11.g0_.x) * _e15.g1_));
}

fn translator_line_left_anti_contraction(self_1552: Translator, other_1288: Line) -> AntiScalar {
    var self_1553: Translator;
    var other_1289: Line;

    self_1553 = self_1552;
    other_1289 = other_1288;
    let _e5: Translator = self_1553;
    let _e8: Line = other_1289;
    let _e13: Translator = self_1553;
    let _e16: Line = other_1289;
    let _e21: Translator = self_1553;
    let _e24: Line = other_1289;
    return AntiScalar((((0.0 - (_e5.g0_.y * _e8.g0_.x)) - (_e13.g0_.z * _e16.g0_.y)) - (_e21.g0_.w * _e24.g0_.z)));
}

fn translator_line_anti_scalar_product(self_1554: Translator, other_1290: Line) -> AntiScalar {
    var self_1555: Translator;
    var other_1291: Line;

    self_1555 = self_1554;
    other_1291 = other_1290;
    let _e5: Translator = self_1555;
    let _e8: Line = other_1291;
    let _e13: Translator = self_1555;
    let _e16: Line = other_1291;
    let _e21: Translator = self_1555;
    let _e24: Line = other_1291;
    return AntiScalar((((0.0 - (_e5.g0_.y * _e8.g0_.x)) - (_e13.g0_.z * _e16.g0_.y)) - (_e21.g0_.w * _e24.g0_.z)));
}

fn translator_translator_add(self_1556: Translator, other_1292: Translator) -> Translator {
    var self_1557: Translator;
    var other_1293: Translator;

    self_1557 = self_1556;
    other_1293 = other_1292;
    let _e4: Translator = self_1557;
    let _e6: Translator = other_1293;
    return Translator((_e4.g0_ + _e6.g0_));
}

fn translator_translator_sub(self_1558: Translator, other_1294: Translator) -> Translator {
    var self_1559: Translator;
    var other_1295: Translator;

    self_1559 = self_1558;
    other_1295 = other_1294;
    let _e4: Translator = self_1559;
    let _e6: Translator = other_1295;
    return Translator((_e4.g0_ - _e6.g0_));
}

fn translator_translator_mul(self_1560: Translator, other_1296: Translator) -> Translator {
    var self_1561: Translator;
    var other_1297: Translator;

    self_1561 = self_1560;
    other_1297 = other_1296;
    let _e4: Translator = self_1561;
    let _e6: Translator = other_1297;
    return Translator((_e4.g0_ * _e6.g0_));
}

fn translator_translator_div(self_1562: Translator, other_1298: Translator) -> Translator {
    var self_1563: Translator;
    var other_1299: Translator;

    self_1563 = self_1562;
    other_1299 = other_1298;
    let _e4: Translator = self_1563;
    let _e7: Translator = self_1563;
    let _e10: Translator = self_1563;
    let _e13: Translator = self_1563;
    let _e23: Translator = other_1299;
    let _e26: Translator = other_1299;
    let _e29: Translator = other_1299;
    let _e32: Translator = other_1299;
    return Translator((((vec4<f32>(_e4.g0_.x, _e7.g0_.y, _e10.g0_.z, _e13.g0_.w) * vec4<f32>(1.0, 1.0, 1.0, 1.0)) / vec4<f32>(_e23.g0_.x, _e26.g0_.y, _e29.g0_.z, _e32.g0_.w)) * vec4<f32>(1.0, 1.0, 1.0, 1.0)));
}

fn translator_translator_geometric_product(self_1564: Translator, other_1300: Translator) -> Translator {
    var self_1565: Translator;
    var other_1301: Translator;

    self_1565 = self_1564;
    other_1301 = other_1300;
    let _e4: Translator = self_1565;
    let _e8: Translator = other_1301;
    let _e11: Translator = self_1565;
    let _e13: Translator = other_1301;
    return Translator(((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((_e11.g0_ * vec4<f32>(_e13.g0_.x)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn translator_translator_outer_product(self_1566: Translator, other_1302: Translator) -> Translator {
    var self_1567: Translator;
    var other_1303: Translator;

    self_1567 = self_1566;
    other_1303 = other_1302;
    let _e4: Translator = self_1567;
    let _e8: Translator = other_1303;
    let _e11: Translator = self_1567;
    let _e13: Translator = other_1303;
    return Translator(((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((_e11.g0_ * vec4<f32>(_e13.g0_.x)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn translator_translator_inner_product(self_1568: Translator, other_1304: Translator) -> Translator {
    var self_1569: Translator;
    var other_1305: Translator;

    self_1569 = self_1568;
    other_1305 = other_1304;
    let _e4: Translator = self_1569;
    let _e8: Translator = other_1305;
    let _e11: Translator = self_1569;
    let _e13: Translator = other_1305;
    return Translator(((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((_e11.g0_ * vec4<f32>(_e13.g0_.x)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn translator_translator_left_contraction(self_1570: Translator, other_1306: Translator) -> Translator {
    var self_1571: Translator;
    var other_1307: Translator;

    self_1571 = self_1570;
    other_1307 = other_1306;
    let _e4: Translator = self_1571;
    let _e8: Translator = other_1307;
    return Translator((vec4<f32>(_e4.g0_.x) * _e8.g0_));
}

fn translator_translator_right_contraction(self_1572: Translator, other_1308: Translator) -> Translator {
    var self_1573: Translator;
    var other_1309: Translator;

    self_1573 = self_1572;
    other_1309 = other_1308;
    let _e4: Translator = self_1573;
    let _e6: Translator = other_1309;
    return Translator((_e4.g0_ * vec4<f32>(_e6.g0_.x)));
}

fn translator_translator_scalar_product(self_1574: Translator, other_1310: Translator) -> Scalar {
    var self_1575: Translator;
    var other_1311: Translator;

    self_1575 = self_1574;
    other_1311 = other_1310;
    let _e4: Translator = self_1575;
    let _e7: Translator = other_1311;
    return Scalar((_e4.g0_.x * _e7.g0_.x));
}

fn translator_translator_anti_scalar_product(self_1576: Translator, other_1312: Translator) -> AntiScalar {
    var self_1577: Translator;
    var other_1313: Translator;

    self_1577 = self_1576;
    other_1313 = other_1312;
    let _e5: Translator = self_1577;
    let _e8: Translator = other_1313;
    let _e13: Translator = self_1577;
    let _e16: Translator = other_1313;
    let _e21: Translator = self_1577;
    let _e24: Translator = other_1313;
    return AntiScalar((((0.0 - (_e5.g0_.y * _e8.g0_.y)) - (_e13.g0_.z * _e16.g0_.z)) - (_e21.g0_.w * _e24.g0_.w)));
}

fn translator_motor_add(self_1578: Translator, other_1314: Motor) -> Motor {
    var self_1579: Translator;
    var other_1315: Motor;

    self_1579 = self_1578;
    other_1315 = other_1314;
    let _e4: Translator = self_1579;
    let _e14: Motor = other_1315;
    let _e17: Translator = self_1579;
    let _e25: Motor = other_1315;
    return Motor(((vec4<f32>(_e4.g0_.x) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) + _e14.g0_), ((_e17.g0_ * vec4<f32>(0.0, 1.0, 1.0, 1.0)) + _e25.g1_));
}

fn translator_motor_sub(self_1580: Translator, other_1316: Motor) -> Motor {
    var self_1581: Translator;
    var other_1317: Motor;

    self_1581 = self_1580;
    other_1317 = other_1316;
    let _e4: Translator = self_1581;
    let _e14: Motor = other_1317;
    let _e17: Translator = self_1581;
    let _e25: Motor = other_1317;
    return Motor(((vec4<f32>(_e4.g0_.x) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) - _e14.g0_), ((_e17.g0_ * vec4<f32>(0.0, 1.0, 1.0, 1.0)) - _e25.g1_));
}

fn translator_motor_geometric_product(self_1582: Translator, other_1318: Motor) -> Motor {
    var self_1583: Translator;
    var other_1319: Motor;

    self_1583 = self_1582;
    other_1319 = other_1318;
    let _e4: Translator = self_1583;
    let _e8: Motor = other_1319;
    let _e11: Translator = self_1583;
    let _e15: Motor = other_1319;
    let _e18: Translator = self_1583;
    let _e22: Motor = other_1319;
    let _e34: Translator = self_1583;
    let _e38: Motor = other_1319;
    let _e50: Translator = self_1583;
    let _e54: Motor = other_1319;
    return Motor((vec4<f32>(_e4.g0_.x) * _e8.g0_), ((((vec4<f32>(_e11.g0_.x) * _e15.g1_) + ((vec4<f32>(_e18.g0_.y) * _e22.g0_.yxwz) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e34.g0_.z) * _e38.g0_.zwxy) * vec4<f32>(1.0, -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e50.g0_.w) * _e54.g0_.wzyx) * vec4<f32>(1.0, 1.0, -(1.0), 1.0))));
}

fn translator_motor_regressive_product(self_1584: Translator, other_1320: Motor) -> Translator {
    var self_1585: Translator;
    var other_1321: Motor;

    self_1585 = self_1584;
    other_1321 = other_1320;
    let _e4: Translator = self_1585;
    let _e8: Motor = other_1321;
    let _e11: Motor = other_1321;
    let _e14: Motor = other_1321;
    let _e17: Motor = other_1321;
    let _e28: Translator = self_1585;
    let _e32: Motor = other_1321;
    let _e35: Motor = other_1321;
    let _e38: Motor = other_1321;
    let _e41: Motor = other_1321;
    let _e53: Translator = self_1585;
    let _e57: Motor = other_1321;
    let _e60: Motor = other_1321;
    let _e63: Motor = other_1321;
    let _e66: Motor = other_1321;
    let _e78: Translator = self_1585;
    let _e82: Motor = other_1321;
    return Translator((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y, _e11.g1_.x, _e14.g0_.y, _e17.g0_.y)) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) + ((vec4<f32>(_e28.g0_.z) * vec4<f32>(_e32.g0_.z, _e35.g0_.z, _e38.g1_.x, _e41.g0_.z)) * vec4<f32>(1.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e53.g0_.w) * vec4<f32>(_e57.g0_.w, _e60.g0_.w, _e63.g0_.w, _e66.g1_.x)) * vec4<f32>(1.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e78.g0_.x) * vec4<f32>(_e82.g1_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))));
}

fn translator_motor_outer_product(self_1586: Translator, other_1322: Motor) -> Motor {
    var self_1587: Translator;
    var other_1323: Motor;

    self_1587 = self_1586;
    other_1323 = other_1322;
    let _e4: Translator = self_1587;
    let _e8: Motor = other_1323;
    let _e11: Translator = self_1587;
    let _e15: Motor = other_1323;
    let _e18: Translator = self_1587;
    let _e22: Motor = other_1323;
    let _e33: Translator = self_1587;
    let _e37: Motor = other_1323;
    let _e48: Translator = self_1587;
    let _e51: Motor = other_1323;
    return Motor((vec4<f32>(_e4.g0_.x) * _e8.g0_), ((((vec4<f32>(_e11.g0_.x) * _e15.g1_) + ((vec4<f32>(_e18.g0_.z) * _e22.g0_.zzxz) * vec4<f32>(1.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e33.g0_.w) * _e37.g0_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, 1.0))) + ((_e48.g0_.yyxx * _e51.g0_.yxxx) * vec4<f32>(1.0, 1.0, 0.0, 0.0))));
}

fn translator_motor_inner_product(self_1588: Translator, other_1324: Motor) -> Motor {
    var self_1589: Translator;
    var other_1325: Motor;

    self_1589 = self_1588;
    other_1325 = other_1324;
    let _e4: Translator = self_1589;
    let _e8: Motor = other_1325;
    let _e11: Translator = self_1589;
    let _e15: Motor = other_1325;
    let _e18: Translator = self_1589;
    let _e20: Motor = other_1325;
    return Motor((vec4<f32>(_e4.g0_.x) * _e8.g0_), ((vec4<f32>(_e11.g0_.x) * _e15.g1_) + ((_e18.g0_ * vec4<f32>(_e20.g0_.x)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn translator_motor_geometric_anti_product(self_1590: Translator, other_1326: Motor) -> Motor {
    var self_1591: Translator;
    var other_1327: Motor;

    self_1591 = self_1590;
    other_1327 = other_1326;
    let _e4: Translator = self_1591;
    let _e8: Motor = other_1327;
    let _e20: Translator = self_1591;
    let _e24: Motor = other_1327;
    let _e37: Translator = self_1591;
    let _e41: Motor = other_1327;
    let _e54: Translator = self_1591;
    let _e58: Motor = other_1327;
    let _e71: Translator = self_1591;
    let _e75: Motor = other_1327;
    let _e87: Translator = self_1591;
    let _e91: Motor = other_1327;
    let _e104: Translator = self_1591;
    let _e108: Motor = other_1327;
    return Motor((((((vec4<f32>(_e4.g0_.x) * _e8.g1_) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))) + ((vec4<f32>(_e20.g0_.y) * _e24.g0_.yxwz) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e37.g0_.z) * _e41.g0_.zwxy) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e54.g0_.w) * _e58.g0_.wzyx) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))), ((((vec4<f32>(_e71.g0_.y) * _e75.g1_.yxwz) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0))) + ((vec4<f32>(_e87.g0_.z) * _e91.g1_.zwxy) * vec4<f32>(-(1.0), -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e104.g0_.w) * _e108.g1_.wzyx) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))));
}

fn translator_motor_inner_anti_product(self_1592: Translator, other_1328: Motor) -> Motor {
    var self_1593: Translator;
    var other_1329: Motor;

    self_1593 = self_1592;
    other_1329 = other_1328;
    let _e4: Translator = self_1593;
    let _e8: Motor = other_1329;
    let _e20: Translator = self_1593;
    let _e22: Motor = other_1329;
    let _e37: Translator = self_1593;
    let _e41: Motor = other_1329;
    let _e52: Translator = self_1593;
    let _e56: Motor = other_1329;
    let _e68: Translator = self_1593;
    let _e71: Motor = other_1329;
    return Motor((((vec4<f32>(_e4.g0_.x) * _e8.g1_) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))) + ((_e20.g0_ * vec4<f32>(_e22.g0_.x)) * vec4<f32>(0.0, -(1.0), -(1.0), -(1.0)))), ((((vec4<f32>(_e37.g0_.z) * _e41.g1_.zzxz) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0)) + ((vec4<f32>(_e52.g0_.w) * _e56.g1_.wwwx) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))) + ((_e68.g0_.yyxx * _e71.g1_.yxxx) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))));
}

fn translator_motor_left_contraction(self_1594: Translator, other_1330: Motor) -> Motor {
    var self_1595: Translator;
    var other_1331: Motor;

    self_1595 = self_1594;
    other_1331 = other_1330;
    let _e4: Translator = self_1595;
    let _e8: Motor = other_1331;
    let _e11: Translator = self_1595;
    let _e15: Motor = other_1331;
    return Motor((vec4<f32>(_e4.g0_.x) * _e8.g0_), (vec4<f32>(_e11.g0_.x) * _e15.g1_));
}

fn translator_motor_right_contraction(self_1596: Translator, other_1332: Motor) -> Translator {
    var self_1597: Translator;
    var other_1333: Motor;

    self_1597 = self_1596;
    other_1333 = other_1332;
    let _e4: Translator = self_1597;
    let _e6: Motor = other_1333;
    return Translator((_e4.g0_ * vec4<f32>(_e6.g0_.x)));
}

fn translator_motor_right_anti_contraction(self_1598: Translator, other_1334: Motor) -> Motor {
    var self_1599: Translator;
    var other_1335: Motor;

    self_1599 = self_1598;
    other_1335 = other_1334;
    let _e4: Translator = self_1599;
    let _e8: Motor = other_1335;
    let _e20: Translator = self_1599;
    let _e24: Motor = other_1335;
    let _e35: Translator = self_1599;
    let _e39: Motor = other_1335;
    let _e51: Translator = self_1599;
    let _e54: Motor = other_1335;
    return Motor(((vec4<f32>(_e4.g0_.x) * _e8.g1_) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))), ((((vec4<f32>(_e20.g0_.z) * _e24.g1_.zzxz) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0)) + ((vec4<f32>(_e35.g0_.w) * _e39.g1_.wwwx) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))) + ((_e51.g0_.yyxx * _e54.g1_.yxxx) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))));
}

fn translator_motor_scalar_product(self_1600: Translator, other_1336: Motor) -> Scalar {
    var self_1601: Translator;
    var other_1337: Motor;

    self_1601 = self_1600;
    other_1337 = other_1336;
    let _e4: Translator = self_1601;
    let _e7: Motor = other_1337;
    return Scalar((_e4.g0_.x * _e7.g0_.x));
}

fn translator_motor_anti_scalar_product(self_1602: Translator, other_1338: Motor) -> AntiScalar {
    var self_1603: Translator;
    var other_1339: Motor;

    self_1603 = self_1602;
    other_1339 = other_1338;
    let _e5: Translator = self_1603;
    let _e8: Motor = other_1339;
    let _e13: Translator = self_1603;
    let _e16: Motor = other_1339;
    let _e21: Translator = self_1603;
    let _e24: Motor = other_1339;
    return AntiScalar((((0.0 - (_e5.g0_.y * _e8.g1_.y)) - (_e13.g0_.z * _e16.g1_.z)) - (_e21.g0_.w * _e24.g1_.w)));
}

fn translator_point_and_plane_geometric_product(self_1604: Translator, other_1340: PointAndPlane) -> PointAndPlane {
    var self_1605: Translator;
    var other_1341: PointAndPlane;

    self_1605 = self_1604;
    other_1341 = other_1340;
    let _e4: Translator = self_1605;
    let _e8: PointAndPlane = other_1341;
    let _e11: Translator = self_1605;
    let _e15: PointAndPlane = other_1341;
    let _e18: PointAndPlane = other_1341;
    let _e21: PointAndPlane = other_1341;
    let _e24: PointAndPlane = other_1341;
    let _e38: Translator = self_1605;
    let _e42: PointAndPlane = other_1341;
    let _e45: PointAndPlane = other_1341;
    let _e48: PointAndPlane = other_1341;
    let _e51: PointAndPlane = other_1341;
    let _e65: Translator = self_1605;
    let _e68: PointAndPlane = other_1341;
    let _e71: PointAndPlane = other_1341;
    let _e74: PointAndPlane = other_1341;
    let _e77: PointAndPlane = other_1341;
    let _e91: Translator = self_1605;
    let _e95: PointAndPlane = other_1341;
    let _e98: Translator = self_1605;
    let _e102: PointAndPlane = other_1341;
    let _e114: Translator = self_1605;
    let _e118: PointAndPlane = other_1341;
    let _e130: Translator = self_1605;
    let _e133: PointAndPlane = other_1341;
    return PointAndPlane(((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.z) * vec4<f32>(_e15.g1_.w, _e18.g1_.w, _e21.g0_.x, _e24.g1_.y)) * vec4<f32>(0.0, -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e38.g0_.w) * vec4<f32>(_e42.g1_.z, _e45.g1_.z, _e48.g1_.y, _e51.g0_.x)) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))) + ((_e65.g0_.xyyy * vec4<f32>(_e68.g0_.x, _e71.g0_.x, _e74.g1_.w, _e77.g1_.z)) * vec4<f32>(0.0, -(1.0), 1.0, -(1.0)))), ((((vec4<f32>(_e91.g0_.x) * _e95.g1_) + ((vec4<f32>(_e98.g0_.z) * vec4<f32>(_e102.g1_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e114.g0_.w) * vec4<f32>(_e118.g1_.w)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((_e130.g0_.yxxx * _e133.g1_.yxxx) * vec4<f32>(1.0, 0.0, 0.0, 0.0))));
}

fn translator_point_and_plane_regressive_product(self_1606: Translator, other_1342: PointAndPlane) -> Plane {
    var self_1607: Translator;
    var other_1343: PointAndPlane;

    self_1607 = self_1606;
    other_1343 = other_1342;
    let _e4: Translator = self_1607;
    let _e8: PointAndPlane = other_1343;
    let _e19: Translator = self_1607;
    let _e23: PointAndPlane = other_1343;
    let _e35: Translator = self_1607;
    let _e38: PointAndPlane = other_1343;
    return Plane(((((vec4<f32>(_e4.g0_.z) * _e8.g0_.zzxz) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0)) + ((vec4<f32>(_e19.g0_.w) * _e23.g0_.wwwx) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))) + ((_e35.g0_.yyxx * _e38.g0_.yxxx) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))));
}

fn translator_point_and_plane_outer_product(self_1608: Translator, other_1344: PointAndPlane) -> PointAndPlane {
    var self_1609: Translator;
    var other_1345: PointAndPlane;

    self_1609 = self_1608;
    other_1345 = other_1344;
    let _e4: Translator = self_1609;
    let _e8: PointAndPlane = other_1345;
    let _e11: Translator = self_1609;
    let _e15: PointAndPlane = other_1345;
    let _e27: Translator = self_1609;
    let _e31: PointAndPlane = other_1345;
    let _e43: Translator = self_1609;
    let _e46: PointAndPlane = other_1345;
    let _e58: Translator = self_1609;
    let _e62: PointAndPlane = other_1345;
    return PointAndPlane(((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.z) * _e15.g1_.wwwy) * vec4<f32>(0.0, -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e27.g0_.w) * _e31.g1_.zzyz) * vec4<f32>(0.0, 1.0, -(1.0), 0.0))) + ((_e43.g0_.xxyy * _e46.g1_.xxwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))), (vec4<f32>(_e58.g0_.x) * _e62.g1_));
}

fn translator_point_and_plane_inner_product(self_1610: Translator, other_1346: PointAndPlane) -> PointAndPlane {
    var self_1611: Translator;
    var other_1347: PointAndPlane;

    self_1611 = self_1610;
    other_1347 = other_1346;
    let _e4: Translator = self_1611;
    let _e8: PointAndPlane = other_1347;
    let _e11: Translator = self_1611;
    let _e15: PointAndPlane = other_1347;
    let _e18: Translator = self_1611;
    let _e22: PointAndPlane = other_1347;
    let _e34: Translator = self_1611;
    let _e38: PointAndPlane = other_1347;
    let _e50: Translator = self_1611;
    let _e53: PointAndPlane = other_1347;
    return PointAndPlane((vec4<f32>(_e4.g0_.x) * _e8.g0_), ((((vec4<f32>(_e11.g0_.x) * _e15.g1_) + ((vec4<f32>(_e18.g0_.z) * vec4<f32>(_e22.g1_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e34.g0_.w) * vec4<f32>(_e38.g1_.w)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((_e50.g0_.yxxx * _e53.g1_.yxxx) * vec4<f32>(1.0, 0.0, 0.0, 0.0))));
}

fn translator_point_and_plane_geometric_anti_product(self_1612: Translator, other_1348: PointAndPlane) -> PointAndPlane {
    var self_1613: Translator;
    var other_1349: PointAndPlane;

    self_1613 = self_1612;
    other_1349 = other_1348;
    let _e4: Translator = self_1613;
    let _e8: PointAndPlane = other_1349;
    let _e11: PointAndPlane = other_1349;
    let _e14: PointAndPlane = other_1349;
    let _e17: PointAndPlane = other_1349;
    let _e30: Translator = self_1613;
    let _e34: PointAndPlane = other_1349;
    let _e37: PointAndPlane = other_1349;
    let _e40: PointAndPlane = other_1349;
    let _e43: PointAndPlane = other_1349;
    let _e57: Translator = self_1613;
    let _e61: PointAndPlane = other_1349;
    let _e64: PointAndPlane = other_1349;
    let _e67: PointAndPlane = other_1349;
    let _e70: PointAndPlane = other_1349;
    let _e84: Translator = self_1613;
    let _e88: PointAndPlane = other_1349;
    let _e100: Translator = self_1613;
    let _e104: PointAndPlane = other_1349;
    let _e107: PointAndPlane = other_1349;
    let _e110: PointAndPlane = other_1349;
    let _e113: PointAndPlane = other_1349;
    let _e126: Translator = self_1613;
    let _e130: PointAndPlane = other_1349;
    let _e133: PointAndPlane = other_1349;
    let _e136: PointAndPlane = other_1349;
    let _e139: PointAndPlane = other_1349;
    let _e153: Translator = self_1613;
    let _e157: PointAndPlane = other_1349;
    let _e160: PointAndPlane = other_1349;
    let _e163: PointAndPlane = other_1349;
    let _e166: PointAndPlane = other_1349;
    let _e180: Translator = self_1613;
    let _e184: PointAndPlane = other_1349;
    return PointAndPlane((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g1_.y, _e11.g1_.x, _e14.g0_.w, _e17.g0_.z)) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0))) + ((vec4<f32>(_e30.g0_.z) * vec4<f32>(_e34.g1_.z, _e37.g0_.w, _e40.g1_.x, _e43.g0_.y)) * vec4<f32>(-(1.0), -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e57.g0_.w) * vec4<f32>(_e61.g1_.w, _e64.g0_.z, _e67.g0_.y, _e70.g1_.x)) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))) + ((vec4<f32>(_e84.g0_.x) * vec4<f32>(_e88.g1_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))), (((((vec4<f32>(_e100.g0_.y) * vec4<f32>(_e104.g0_.y, _e107.g0_.x, _e110.g1_.w, _e113.g1_.z)) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0))) + ((vec4<f32>(_e126.g0_.z) * vec4<f32>(_e130.g0_.z, _e133.g1_.w, _e136.g0_.x, _e139.g1_.y)) * vec4<f32>(-(1.0), -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e153.g0_.w) * vec4<f32>(_e157.g0_.w, _e160.g1_.z, _e163.g1_.y, _e166.g0_.x)) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))) + ((vec4<f32>(_e180.g0_.x) * _e184.g0_) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn translator_point_and_plane_left_contraction(self_1614: Translator, other_1350: PointAndPlane) -> PointAndPlane {
    var self_1615: Translator;
    var other_1351: PointAndPlane;

    self_1615 = self_1614;
    other_1351 = other_1350;
    let _e4: Translator = self_1615;
    let _e8: PointAndPlane = other_1351;
    let _e11: Translator = self_1615;
    let _e15: PointAndPlane = other_1351;
    return PointAndPlane((vec4<f32>(_e4.g0_.x) * _e8.g0_), (vec4<f32>(_e11.g0_.x) * _e15.g1_));
}

fn translator_point_and_plane_left_anti_contraction(self_1616: Translator, other_1352: PointAndPlane) -> Point {
    var self_1617: Translator;
    var other_1353: PointAndPlane;

    self_1617 = self_1616;
    other_1353 = other_1352;
    let _e4: Translator = self_1617;
    let _e8: PointAndPlane = other_1353;
    let _e19: Translator = self_1617;
    let _e23: PointAndPlane = other_1353;
    let _e35: Translator = self_1617;
    let _e38: PointAndPlane = other_1353;
    return Point(((((vec4<f32>(_e4.g0_.z) * _e8.g1_.zzxz) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0)) + ((vec4<f32>(_e19.g0_.w) * _e23.g1_.wwwx) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))) + ((_e35.g0_.yyxx * _e38.g1_.yxxx) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))));
}

fn translator_squared_magnitude(self_1618: Translator) -> Scalar {
    var self_1619: Translator;

    self_1619 = self_1618;
    let _e2: Translator = self_1619;
    let _e3: Translator = self_1619;
    let _e4: Translator = translator_reversal(_e3);
    let _e5: Scalar = translator_translator_scalar_product(_e2, _e4);
    return _e5;
}

fn translator_magnitude(self_1620: Translator) -> Scalar {
    var self_1621: Translator;

    self_1621 = self_1620;
    let _e2: Translator = self_1621;
    let _e3: Scalar = translator_squared_magnitude(_e2);
    return Scalar(sqrt(_e3.g0_));
}

fn translator_bulk_norm(self_1622: Translator) -> Scalar {
    var self_1623: Translator;

    self_1623 = self_1622;
    let _e2: Translator = self_1623;
    let _e3: Scalar = translator_squared_magnitude(_e2);
    return Scalar(sqrt(_e3.g0_));
}

fn translator_squared_anti_magnitude(self_1624: Translator) -> AntiScalar {
    var self_1625: Translator;

    self_1625 = self_1624;
    let _e2: Translator = self_1625;
    let _e3: Translator = self_1625;
    let _e4: Translator = translator_anti_reversal(_e3);
    let _e5: AntiScalar = translator_translator_anti_scalar_product(_e2, _e4);
    return _e5;
}

fn translator_weight_norm(self_1626: Translator) -> AntiScalar {
    var self_1627: Translator;

    self_1627 = self_1626;
    let _e2: Translator = self_1627;
    let _e3: AntiScalar = translator_squared_anti_magnitude(_e2);
    return AntiScalar(sqrt(_e3.g0_));
}

fn translator_scale(self_1628: Translator, other_1354: f32) -> Translator {
    var self_1629: Translator;
    var other_1355: f32;

    self_1629 = self_1628;
    other_1355 = other_1354;
    let _e4: Translator = self_1629;
    let _e5: f32 = other_1355;
    let _e7: Translator = translator_scalar_geometric_product(_e4, Scalar(_e5));
    return _e7;
}

fn translator_signum(self_1630: Translator) -> Translator {
    var self_1631: Translator;

    self_1631 = self_1630;
    let _e2: Translator = self_1631;
    let _e3: Translator = self_1631;
    let _e4: Scalar = translator_magnitude(_e3);
    let _e9: Translator = translator_scalar_geometric_product(_e2, Scalar((1.0 / _e4.g0_)));
    return _e9;
}

fn translator_inverse(self_1632: Translator) -> Translator {
    var self_1633: Translator;

    self_1633 = self_1632;
    let _e2: Translator = self_1633;
    let _e3: Translator = translator_reversal(_e2);
    let _e4: Translator = self_1633;
    let _e5: Scalar = translator_squared_magnitude(_e4);
    let _e10: Translator = translator_scalar_geometric_product(_e3, Scalar((1.0 / _e5.g0_)));
    return _e10;
}

fn translator_unitize(self_1634: Translator) -> Translator {
    var self_1635: Translator;

    self_1635 = self_1634;
    let _e2: Translator = self_1635;
    let _e3: Translator = self_1635;
    let _e4: AntiScalar = translator_weight_norm(_e3);
    let _e9: Translator = translator_scalar_geometric_product(_e2, Scalar((1.0 / _e4.g0_)));
    return _e9;
}

fn motor_zero() -> Motor {
    return Motor(vec4<f32>(0.0), vec4<f32>(0.0));
}

fn motor_one() -> Motor {
    return Motor(vec4<f32>(1.0, 0.0, 0.0, 0.0), vec4<f32>(0.0));
}

fn motor_neg(self_1636: Motor) -> Motor {
    var self_1637: Motor;

    self_1637 = self_1636;
    let _e2: Motor = self_1637;
    let _e8: Motor = self_1637;
    return Motor((_e2.g0_ * vec4<f32>(-(1.0))), (_e8.g1_ * vec4<f32>(-(1.0))));
}

fn motor_automorphism(self_1638: Motor) -> Motor {
    var self_1639: Motor;

    self_1639 = self_1638;
    let _e2: Motor = self_1639;
    let _e4: Motor = self_1639;
    return Motor(_e2.g0_, _e4.g1_);
}

fn motor_reversal(self_1640: Motor) -> Motor {
    var self_1641: Motor;

    self_1641 = self_1640;
    let _e2: Motor = self_1641;
    let _e13: Motor = self_1641;
    return Motor((_e2.g0_ * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))), (_e13.g1_ * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))));
}

fn motor_conjugation(self_1642: Motor) -> Motor {
    var self_1643: Motor;

    self_1643 = self_1642;
    let _e2: Motor = self_1643;
    let _e13: Motor = self_1643;
    return Motor((_e2.g0_ * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))), (_e13.g1_ * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))));
}

fn motor_dual(self_1644: Motor) -> Motor {
    var self_1645: Motor;

    self_1645 = self_1644;
    let _e2: Motor = self_1645;
    let _e4: Motor = self_1645;
    return Motor(_e2.g1_, _e4.g0_);
}

fn motor_anti_reversal(self_1646: Motor) -> Motor {
    var self_1647: Motor;

    self_1647 = self_1646;
    let _e2: Motor = self_1647;
    let _e13: Motor = self_1647;
    return Motor((_e2.g0_ * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))), (_e13.g1_ * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))));
}

fn motor_right_complement(self_1648: Motor) -> Motor {
    var self_1649: Motor;

    self_1649 = self_1648;
    let _e2: Motor = self_1649;
    let _e4: Motor = self_1649;
    return Motor(_e2.g1_, _e4.g0_);
}

fn motor_left_complement(self_1650: Motor) -> Motor {
    var self_1651: Motor;

    self_1651 = self_1650;
    let _e2: Motor = self_1651;
    let _e4: Motor = self_1651;
    return Motor(_e2.g1_, _e4.g0_);
}

fn motor_double_complement(self_1652: Motor) -> Motor {
    var self_1653: Motor;

    self_1653 = self_1652;
    let _e2: Motor = self_1653;
    let _e4: Motor = self_1653;
    return Motor(_e2.g0_, _e4.g1_);
}

fn motor_scalar_into(self_1654: Motor) -> Scalar {
    var self_1655: Motor;

    self_1655 = self_1654;
    let _e2: Motor = self_1655;
    return Scalar(_e2.g0_.x);
}

fn motor_scalar_add(self_1656: Motor, other_1356: Scalar) -> Motor {
    var self_1657: Motor;
    var other_1357: Scalar;

    self_1657 = self_1656;
    other_1357 = other_1356;
    let _e4: Motor = self_1657;
    let _e6: Scalar = other_1357;
    let _e16: Motor = self_1657;
    return Motor((_e4.g0_ + (vec4<f32>(_e6.g0_) * vec4<f32>(1.0, 0.0, 0.0, 0.0))), _e16.g1_);
}

fn motor_scalar_sub(self_1658: Motor, other_1358: Scalar) -> Motor {
    var self_1659: Motor;
    var other_1359: Scalar;

    self_1659 = self_1658;
    other_1359 = other_1358;
    let _e4: Motor = self_1659;
    let _e6: Scalar = other_1359;
    let _e16: Motor = self_1659;
    return Motor((_e4.g0_ - (vec4<f32>(_e6.g0_) * vec4<f32>(1.0, 0.0, 0.0, 0.0))), _e16.g1_);
}

fn motor_scalar_geometric_product(self_1660: Motor, other_1360: Scalar) -> Motor {
    var self_1661: Motor;
    var other_1361: Scalar;

    self_1661 = self_1660;
    other_1361 = other_1360;
    let _e4: Motor = self_1661;
    let _e6: Scalar = other_1361;
    let _e10: Motor = self_1661;
    let _e12: Scalar = other_1361;
    return Motor((_e4.g0_ * vec4<f32>(_e6.g0_)), (_e10.g1_ * vec4<f32>(_e12.g0_)));
}

fn motor_scalar_regressive_product(self_1662: Motor, other_1362: Scalar) -> Scalar {
    var self_1663: Motor;
    var other_1363: Scalar;

    self_1663 = self_1662;
    other_1363 = other_1362;
    let _e4: Motor = self_1663;
    let _e7: Scalar = other_1363;
    return Scalar((_e4.g1_.x * _e7.g0_));
}

fn motor_scalar_outer_product(self_1664: Motor, other_1364: Scalar) -> Motor {
    var self_1665: Motor;
    var other_1365: Scalar;

    self_1665 = self_1664;
    other_1365 = other_1364;
    let _e4: Motor = self_1665;
    let _e6: Scalar = other_1365;
    let _e10: Motor = self_1665;
    let _e12: Scalar = other_1365;
    return Motor((_e4.g0_ * vec4<f32>(_e6.g0_)), (_e10.g1_ * vec4<f32>(_e12.g0_)));
}

fn motor_scalar_inner_product(self_1666: Motor, other_1366: Scalar) -> Motor {
    var self_1667: Motor;
    var other_1367: Scalar;

    self_1667 = self_1666;
    other_1367 = other_1366;
    let _e4: Motor = self_1667;
    let _e6: Scalar = other_1367;
    let _e10: Motor = self_1667;
    let _e12: Scalar = other_1367;
    return Motor((_e4.g0_ * vec4<f32>(_e6.g0_)), (_e10.g1_ * vec4<f32>(_e12.g0_)));
}

fn motor_scalar_geometric_anti_product(self_1668: Motor, other_1368: Scalar) -> Rotor {
    var self_1669: Motor;
    var other_1369: Scalar;

    self_1669 = self_1668;
    other_1369 = other_1368;
    let _e4: Motor = self_1669;
    let _e6: Scalar = other_1369;
    return Rotor(((_e4.g1_ * vec4<f32>(_e6.g0_)) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))));
}

fn motor_scalar_inner_anti_product(self_1670: Motor, other_1370: Scalar) -> Rotor {
    var self_1671: Motor;
    var other_1371: Scalar;

    self_1671 = self_1670;
    other_1371 = other_1370;
    let _e4: Motor = self_1671;
    let _e6: Scalar = other_1371;
    return Rotor(((_e4.g1_ * vec4<f32>(_e6.g0_)) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))));
}

fn motor_scalar_left_contraction(self_1672: Motor, other_1372: Scalar) -> Scalar {
    var self_1673: Motor;
    var other_1373: Scalar;

    self_1673 = self_1672;
    other_1373 = other_1372;
    let _e4: Motor = self_1673;
    let _e7: Scalar = other_1373;
    return Scalar((_e4.g0_.x * _e7.g0_));
}

fn motor_scalar_right_contraction(self_1674: Motor, other_1374: Scalar) -> Motor {
    var self_1675: Motor;
    var other_1375: Scalar;

    self_1675 = self_1674;
    other_1375 = other_1374;
    let _e4: Motor = self_1675;
    let _e6: Scalar = other_1375;
    let _e10: Motor = self_1675;
    let _e12: Scalar = other_1375;
    return Motor((_e4.g0_ * vec4<f32>(_e6.g0_)), (_e10.g1_ * vec4<f32>(_e12.g0_)));
}

fn motor_scalar_left_anti_contraction(self_1676: Motor, other_1376: Scalar) -> Rotor {
    var self_1677: Motor;
    var other_1377: Scalar;

    self_1677 = self_1676;
    other_1377 = other_1376;
    let _e4: Motor = self_1677;
    let _e6: Scalar = other_1377;
    return Rotor(((_e4.g1_ * vec4<f32>(_e6.g0_)) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))));
}

fn motor_scalar_scalar_product(self_1678: Motor, other_1378: Scalar) -> Scalar {
    var self_1679: Motor;
    var other_1379: Scalar;

    self_1679 = self_1678;
    other_1379 = other_1378;
    let _e4: Motor = self_1679;
    let _e7: Scalar = other_1379;
    return Scalar((_e4.g0_.x * _e7.g0_));
}

fn motor_anti_scalar_into(self_1680: Motor) -> AntiScalar {
    var self_1681: Motor;

    self_1681 = self_1680;
    let _e2: Motor = self_1681;
    return AntiScalar(_e2.g1_.x);
}

fn motor_anti_scalar_add(self_1682: Motor, other_1380: AntiScalar) -> Motor {
    var self_1683: Motor;
    var other_1381: AntiScalar;

    self_1683 = self_1682;
    other_1381 = other_1380;
    let _e4: Motor = self_1683;
    let _e6: Motor = self_1683;
    let _e8: AntiScalar = other_1381;
    return Motor(_e4.g0_, (_e6.g1_ + (vec4<f32>(_e8.g0_) * vec4<f32>(1.0, 0.0, 0.0, 0.0))));
}

fn motor_anti_scalar_sub(self_1684: Motor, other_1382: AntiScalar) -> Motor {
    var self_1685: Motor;
    var other_1383: AntiScalar;

    self_1685 = self_1684;
    other_1383 = other_1382;
    let _e4: Motor = self_1685;
    let _e6: Motor = self_1685;
    let _e8: AntiScalar = other_1383;
    return Motor(_e4.g0_, (_e6.g1_ - (vec4<f32>(_e8.g0_) * vec4<f32>(1.0, 0.0, 0.0, 0.0))));
}

fn motor_anti_scalar_regressive_product(self_1686: Motor, other_1384: AntiScalar) -> Motor {
    var self_1687: Motor;
    var other_1385: AntiScalar;

    self_1687 = self_1686;
    other_1385 = other_1384;
    let _e4: Motor = self_1687;
    let _e6: AntiScalar = other_1385;
    let _e10: Motor = self_1687;
    let _e12: AntiScalar = other_1385;
    return Motor((_e4.g0_ * vec4<f32>(_e6.g0_)), (_e10.g1_ * vec4<f32>(_e12.g0_)));
}

fn motor_anti_scalar_outer_product(self_1688: Motor, other_1386: AntiScalar) -> AntiScalar {
    var self_1689: Motor;
    var other_1387: AntiScalar;

    self_1689 = self_1688;
    other_1387 = other_1386;
    let _e4: Motor = self_1689;
    let _e7: AntiScalar = other_1387;
    return AntiScalar((_e4.g0_.x * _e7.g0_));
}

fn motor_anti_scalar_geometric_anti_product(self_1690: Motor, other_1388: AntiScalar) -> Motor {
    var self_1691: Motor;
    var other_1389: AntiScalar;

    self_1691 = self_1690;
    other_1389 = other_1388;
    let _e4: Motor = self_1691;
    let _e6: AntiScalar = other_1389;
    let _e10: Motor = self_1691;
    let _e12: AntiScalar = other_1389;
    return Motor((_e4.g0_ * vec4<f32>(_e6.g0_)), (_e10.g1_ * vec4<f32>(_e12.g0_)));
}

fn motor_anti_scalar_inner_anti_product(self_1692: Motor, other_1390: AntiScalar) -> Motor {
    var self_1693: Motor;
    var other_1391: AntiScalar;

    self_1693 = self_1692;
    other_1391 = other_1390;
    let _e4: Motor = self_1693;
    let _e6: AntiScalar = other_1391;
    let _e10: Motor = self_1693;
    let _e12: AntiScalar = other_1391;
    return Motor((_e4.g0_ * vec4<f32>(_e6.g0_)), (_e10.g1_ * vec4<f32>(_e12.g0_)));
}

fn motor_anti_scalar_left_anti_contraction(self_1694: Motor, other_1392: AntiScalar) -> AntiScalar {
    var self_1695: Motor;
    var other_1393: AntiScalar;

    self_1695 = self_1694;
    other_1393 = other_1392;
    let _e4: Motor = self_1695;
    let _e7: AntiScalar = other_1393;
    return AntiScalar((_e4.g1_.x * _e7.g0_));
}

fn motor_anti_scalar_right_anti_contraction(self_1696: Motor, other_1394: AntiScalar) -> Motor {
    var self_1697: Motor;
    var other_1395: AntiScalar;

    self_1697 = self_1696;
    other_1395 = other_1394;
    let _e4: Motor = self_1697;
    let _e6: AntiScalar = other_1395;
    let _e10: Motor = self_1697;
    let _e12: AntiScalar = other_1395;
    return Motor((_e4.g0_ * vec4<f32>(_e6.g0_)), (_e10.g1_ * vec4<f32>(_e12.g0_)));
}

fn motor_anti_scalar_anti_scalar_product(self_1698: Motor, other_1396: AntiScalar) -> AntiScalar {
    var self_1699: Motor;
    var other_1397: AntiScalar;

    self_1699 = self_1698;
    other_1397 = other_1396;
    let _e4: Motor = self_1699;
    let _e7: AntiScalar = other_1397;
    return AntiScalar((_e4.g1_.x * _e7.g0_));
}

fn motor_multi_vector_add(self_1700: Motor, other_1398: MultiVector) -> MultiVector {
    var self_1701: Motor;
    var other_1399: MultiVector;

    self_1701 = self_1700;
    other_1399 = other_1398;
    let _e4: Motor = self_1701;
    let _e6: MultiVector = other_1399;
    let _e9: MultiVector = other_1399;
    let _e11: MultiVector = other_1399;
    let _e13: Motor = self_1701;
    let _e15: MultiVector = other_1399;
    return MultiVector((_e4.g0_ + _e6.g0_), _e9.g1_, _e11.g2_, (_e13.g1_ + _e15.g3_));
}

fn motor_multi_vector_sub(self_1702: Motor, other_1400: MultiVector) -> MultiVector {
    var self_1703: Motor;
    var other_1401: MultiVector;

    self_1703 = self_1702;
    other_1401 = other_1400;
    let _e4: Motor = self_1703;
    let _e6: MultiVector = other_1401;
    let _e11: MultiVector = other_1401;
    let _e16: MultiVector = other_1401;
    let _e19: Motor = self_1703;
    let _e21: MultiVector = other_1401;
    return MultiVector((_e4.g0_ - _e6.g0_), (vec4<f32>(0.0) - _e11.g1_), (vec4<f32>(0.0) - _e16.g2_), (_e19.g1_ - _e21.g3_));
}

fn motor_multi_vector_geometric_product(self_1704: Motor, other_1402: MultiVector) -> MultiVector {
    var self_1705: Motor;
    var other_1403: MultiVector;

    self_1705 = self_1704;
    other_1403 = other_1402;
    let _e4: Motor = self_1705;
    let _e8: MultiVector = other_1403;
    let _e11: Motor = self_1705;
    let _e15: MultiVector = other_1403;
    let _e28: Motor = self_1705;
    let _e32: MultiVector = other_1403;
    let _e45: Motor = self_1705;
    let _e49: MultiVector = other_1403;
    let _e62: Motor = self_1705;
    let _e66: MultiVector = other_1403;
    let _e69: Motor = self_1705;
    let _e73: MultiVector = other_1403;
    let _e86: Motor = self_1705;
    let _e90: MultiVector = other_1403;
    let _e103: Motor = self_1705;
    let _e107: MultiVector = other_1403;
    let _e120: Motor = self_1705;
    let _e124: MultiVector = other_1403;
    let _e128: Motor = self_1705;
    let _e132: MultiVector = other_1403;
    let _e145: Motor = self_1705;
    let _e149: MultiVector = other_1403;
    let _e162: Motor = self_1705;
    let _e166: MultiVector = other_1403;
    let _e179: Motor = self_1705;
    let _e183: MultiVector = other_1403;
    let _e186: Motor = self_1705;
    let _e190: MultiVector = other_1403;
    let _e203: Motor = self_1705;
    let _e207: MultiVector = other_1403;
    let _e220: Motor = self_1705;
    let _e224: MultiVector = other_1403;
    let _e237: Motor = self_1705;
    let _e241: MultiVector = other_1403;
    let _e244: Motor = self_1705;
    let _e248: MultiVector = other_1403;
    let _e261: Motor = self_1705;
    let _e265: MultiVector = other_1403;
    let _e278: Motor = self_1705;
    let _e282: MultiVector = other_1403;
    let _e295: Motor = self_1705;
    let _e299: MultiVector = other_1403;
    let _e312: Motor = self_1705;
    let _e316: MultiVector = other_1403;
    let _e328: Motor = self_1705;
    let _e332: MultiVector = other_1403;
    let _e344: Motor = self_1705;
    let _e348: MultiVector = other_1403;
    return MultiVector(((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.y) * _e15.g0_.yxwz) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e28.g0_.z) * _e32.g0_.zwxy) * vec4<f32>(-(1.0), -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e45.g0_.w) * _e49.g0_.wzyx) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))), ((((((((vec4<f32>(_e62.g0_.x) * _e66.g1_) + ((vec4<f32>(_e69.g0_.y) * _e73.g1_.yxwz) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e86.g0_.z) * _e90.g1_.zwxy) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e103.g0_.w) * _e107.g1_.wzyx) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))) - (vec4<f32>(_e120.g1_.x) * _e124.g2_)) + ((vec4<f32>(_e128.g1_.y) * _e132.g2_.yxwz) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e145.g1_.z) * _e149.g2_.zwxy) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e162.g1_.w) * _e166.g2_.wzyx) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))), ((((vec4<f32>(_e179.g0_.x) * _e183.g2_) + ((vec4<f32>(_e186.g0_.y) * _e190.g2_.yxwz) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e203.g0_.z) * _e207.g2_.zwxy) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e220.g0_.w) * _e224.g2_.wzyx) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))), ((((((((vec4<f32>(_e237.g0_.x) * _e241.g3_) + ((vec4<f32>(_e244.g0_.y) * _e248.g3_.yxwz) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e261.g0_.z) * _e265.g3_.zwxy) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e278.g0_.w) * _e282.g3_.wzyx) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e295.g1_.x) * _e299.g0_) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0)))) + ((vec4<f32>(_e312.g1_.y) * _e316.g0_.yxwz) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e328.g1_.z) * _e332.g0_.zwxy) * vec4<f32>(1.0, -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e344.g1_.w) * _e348.g0_.wzyx) * vec4<f32>(1.0, 1.0, -(1.0), 1.0))));
}

fn motor_multi_vector_regressive_product(self_1706: Motor, other_1404: MultiVector) -> MultiVector {
    var self_1707: Motor;
    var other_1405: MultiVector;

    self_1707 = self_1706;
    other_1405 = other_1404;
    let _e4: Motor = self_1707;
    let _e8: MultiVector = other_1405;
    let _e18: Motor = self_1707;
    let _e22: MultiVector = other_1405;
    let _e33: Motor = self_1707;
    let _e37: MultiVector = other_1405;
    let _e48: Motor = self_1707;
    let _e52: MultiVector = other_1405;
    let _e56: Motor = self_1707;
    let _e60: MultiVector = other_1405;
    let _e72: Motor = self_1707;
    let _e76: MultiVector = other_1405;
    let _e88: Motor = self_1707;
    let _e92: MultiVector = other_1405;
    let _e104: Motor = self_1707;
    let _e108: MultiVector = other_1405;
    let _e120: Motor = self_1707;
    let _e124: MultiVector = other_1405;
    let _e127: Motor = self_1707;
    let _e131: MultiVector = other_1405;
    let _e144: Motor = self_1707;
    let _e148: MultiVector = other_1405;
    let _e161: Motor = self_1707;
    let _e164: MultiVector = other_1405;
    let _e176: Motor = self_1707;
    let _e180: MultiVector = other_1405;
    let _e191: Motor = self_1707;
    let _e195: MultiVector = other_1405;
    let _e207: Motor = self_1707;
    let _e211: MultiVector = other_1405;
    let _e215: Motor = self_1707;
    let _e219: MultiVector = other_1405;
    let _e231: Motor = self_1707;
    let _e235: MultiVector = other_1405;
    let _e247: Motor = self_1707;
    let _e250: Motor = self_1707;
    let _e253: Motor = self_1707;
    let _e256: Motor = self_1707;
    let _e260: MultiVector = other_1405;
    let _e263: MultiVector = other_1405;
    let _e266: MultiVector = other_1405;
    let _e269: MultiVector = other_1405;
    let _e282: Motor = self_1707;
    let _e286: MultiVector = other_1405;
    let _e289: Motor = self_1707;
    let _e291: MultiVector = other_1405;
    return MultiVector((((((((((vec4<f32>(_e4.g0_.y) * _e8.g3_.yxyy) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) + ((vec4<f32>(_e18.g0_.z) * _e22.g3_.zzxz) * vec4<f32>(1.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e33.g0_.w) * _e37.g3_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, 1.0))) + (vec4<f32>(_e48.g1_.x) * _e52.g0_)) + ((vec4<f32>(_e56.g1_.y) * vec4<f32>(_e60.g0_.y)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e72.g1_.z) * vec4<f32>(_e76.g0_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e88.g1_.w) * vec4<f32>(_e92.g0_.w)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e104.g0_.x) * vec4<f32>(_e108.g3_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))), ((((vec4<f32>(_e120.g1_.x) * _e124.g1_) + ((vec4<f32>(_e127.g1_.z) * vec4<f32>(_e131.g1_.z)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((vec4<f32>(_e144.g1_.w) * vec4<f32>(_e148.g1_.w)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((_e161.g1_.yxxx * _e164.g1_.yxxx) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))), (((((((vec4<f32>(_e176.g0_.z) * _e180.g1_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e191.g0_.w) * _e195.g1_.zzyz) * vec4<f32>(0.0, -(1.0), 1.0, 0.0))) + (vec4<f32>(_e207.g1_.x) * _e211.g2_)) + ((vec4<f32>(_e215.g1_.z) * vec4<f32>(_e219.g2_.x)) * vec4<f32>(0.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e231.g1_.w) * vec4<f32>(_e235.g2_.x)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e247.g0_.x, _e250.g1_.y, _e253.g0_.y, _e256.g0_.y) * vec4<f32>(_e260.g1_.x, _e263.g2_.x, _e266.g1_.w, _e269.g1_.z)) * vec4<f32>(0.0, 1.0, -(1.0), 1.0))), ((vec4<f32>(_e282.g1_.x) * _e286.g3_) + ((_e289.g1_ * vec4<f32>(_e291.g3_.x)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn motor_multi_vector_outer_product(self_1708: Motor, other_1406: MultiVector) -> MultiVector {
    var self_1709: Motor;
    var other_1407: MultiVector;

    self_1709 = self_1708;
    other_1407 = other_1406;
    let _e4: Motor = self_1709;
    let _e8: MultiVector = other_1407;
    let _e11: Motor = self_1709;
    let _e13: MultiVector = other_1407;
    let _e25: Motor = self_1709;
    let _e29: MultiVector = other_1407;
    let _e32: Motor = self_1709;
    let _e36: MultiVector = other_1407;
    let _e48: Motor = self_1709;
    let _e52: MultiVector = other_1407;
    let _e64: Motor = self_1709;
    let _e68: MultiVector = other_1407;
    let _e80: Motor = self_1709;
    let _e82: MultiVector = other_1407;
    let _e97: Motor = self_1709;
    let _e101: MultiVector = other_1407;
    let _e104: Motor = self_1709;
    let _e108: MultiVector = other_1407;
    let _e120: Motor = self_1709;
    let _e124: MultiVector = other_1407;
    let _e136: Motor = self_1709;
    let _e139: MultiVector = other_1407;
    let _e150: Motor = self_1709;
    let _e154: MultiVector = other_1407;
    let _e157: Motor = self_1709;
    let _e161: MultiVector = other_1407;
    let _e173: Motor = self_1709;
    let _e177: MultiVector = other_1407;
    let _e189: Motor = self_1709;
    let _e193: MultiVector = other_1407;
    let _e205: Motor = self_1709;
    let _e209: MultiVector = other_1407;
    let _e220: Motor = self_1709;
    let _e224: MultiVector = other_1407;
    let _e235: Motor = self_1709;
    let _e239: MultiVector = other_1407;
    let _e250: Motor = self_1709;
    let _e253: MultiVector = other_1407;
    return MultiVector(((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((_e11.g0_ * vec4<f32>(_e13.g0_.x)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))), (((((vec4<f32>(_e25.g0_.x) * _e29.g1_) + ((vec4<f32>(_e32.g1_.y) * _e36.g2_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e48.g1_.z) * _e52.g2_.wwwy) * vec4<f32>(0.0, -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e64.g1_.w) * _e68.g2_.zzyz) * vec4<f32>(0.0, 1.0, -(1.0), 0.0))) + ((_e80.g0_ * vec4<f32>(_e82.g1_.x)) * vec4<f32>(0.0, -(1.0), -(1.0), -(1.0)))), ((((vec4<f32>(_e97.g0_.x) * _e101.g2_) + ((vec4<f32>(_e104.g0_.z) * vec4<f32>(_e108.g2_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e120.g0_.w) * vec4<f32>(_e124.g2_.w)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((_e136.g0_.yxxx * _e139.g2_.yxxx) * vec4<f32>(1.0, 0.0, 0.0, 0.0))), ((((((((vec4<f32>(_e150.g0_.x) * _e154.g3_) + ((vec4<f32>(_e157.g0_.z) * vec4<f32>(_e161.g3_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e173.g0_.w) * vec4<f32>(_e177.g3_.w)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e189.g1_.x) * vec4<f32>(_e193.g0_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e205.g1_.y) * _e209.g0_.yxyy) * vec4<f32>(1.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e220.g1_.z) * _e224.g0_.zzxz) * vec4<f32>(1.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e235.g1_.w) * _e239.g0_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, 1.0))) + ((_e250.g0_.yxxx * _e253.g3_.yxxx) * vec4<f32>(1.0, 0.0, 0.0, 0.0))));
}

fn motor_multi_vector_inner_product(self_1710: Motor, other_1408: MultiVector) -> MultiVector {
    var self_1711: Motor;
    var other_1409: MultiVector;

    self_1711 = self_1710;
    other_1409 = other_1408;
    let _e4: Motor = self_1711;
    let _e8: MultiVector = other_1409;
    let _e11: Motor = self_1711;
    let _e15: MultiVector = other_1409;
    let _e27: Motor = self_1711;
    let _e31: MultiVector = other_1409;
    let _e43: Motor = self_1711;
    let _e46: MultiVector = other_1409;
    let _e58: Motor = self_1711;
    let _e62: MultiVector = other_1409;
    let _e65: Motor = self_1711;
    let _e69: MultiVector = other_1409;
    let _e81: Motor = self_1711;
    let _e85: MultiVector = other_1409;
    let _e97: Motor = self_1711;
    let _e101: MultiVector = other_1409;
    let _e105: Motor = self_1711;
    let _e109: MultiVector = other_1409;
    let _e121: Motor = self_1711;
    let _e125: MultiVector = other_1409;
    let _e137: Motor = self_1711;
    let _e141: MultiVector = other_1409;
    let _e153: Motor = self_1711;
    let _e156: MultiVector = other_1409;
    let _e167: Motor = self_1711;
    let _e171: MultiVector = other_1409;
    let _e174: Motor = self_1711;
    let _e178: MultiVector = other_1409;
    let _e191: Motor = self_1711;
    let _e195: MultiVector = other_1409;
    let _e208: Motor = self_1711;
    let _e211: MultiVector = other_1409;
    let _e224: Motor = self_1711;
    let _e228: MultiVector = other_1409;
    let _e231: Motor = self_1711;
    let _e235: MultiVector = other_1409;
    let _e248: Motor = self_1711;
    let _e252: MultiVector = other_1409;
    let _e264: Motor = self_1711;
    let _e268: MultiVector = other_1409;
    let _e280: Motor = self_1711;
    let _e284: MultiVector = other_1409;
    let _e296: Motor = self_1711;
    let _e298: MultiVector = other_1409;
    return MultiVector(((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.z) * _e15.g0_.zzxz) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0))) + ((vec4<f32>(_e27.g0_.w) * _e31.g0_.wwwx) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))) + ((_e43.g0_.yyxx * _e46.g0_.yxxx) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))), ((((((((vec4<f32>(_e58.g0_.x) * _e62.g1_) + ((vec4<f32>(_e65.g0_.z) * vec4<f32>(_e69.g1_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e81.g0_.w) * vec4<f32>(_e85.g1_.w)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) - (vec4<f32>(_e97.g1_.x) * _e101.g2_)) + ((vec4<f32>(_e105.g1_.y) * vec4<f32>(_e109.g2_.y)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e121.g1_.z) * vec4<f32>(_e125.g2_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e137.g1_.w) * vec4<f32>(_e141.g2_.w)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((_e153.g0_.yxxx * _e156.g1_.yxxx) * vec4<f32>(1.0, 0.0, 0.0, 0.0))), ((((vec4<f32>(_e167.g0_.x) * _e171.g2_) + ((vec4<f32>(_e174.g0_.z) * _e178.g2_.wwxy) * vec4<f32>(0.0, -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e191.g0_.w) * _e195.g2_.zzyx) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))) + ((_e208.g0_.xyyy * _e211.g2_.xxwz) * vec4<f32>(0.0, -(1.0), 1.0, -(1.0)))), ((((((vec4<f32>(_e224.g0_.x) * _e228.g3_) + ((vec4<f32>(_e231.g1_.x) * _e235.g0_) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0)))) + ((vec4<f32>(_e248.g1_.y) * vec4<f32>(_e252.g0_.x)) * vec4<f32>(0.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e264.g1_.z) * vec4<f32>(_e268.g0_.x)) * vec4<f32>(0.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e280.g1_.w) * vec4<f32>(_e284.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((_e296.g0_ * vec4<f32>(_e298.g3_.x)) * vec4<f32>(0.0, -(1.0), -(1.0), -(1.0)))));
}

fn motor_multi_vector_geometric_anti_product(self_1712: Motor, other_1410: MultiVector) -> MultiVector {
    var self_1713: Motor;
    var other_1411: MultiVector;

    self_1713 = self_1712;
    other_1411 = other_1410;
    let _e4: Motor = self_1713;
    let _e8: MultiVector = other_1411;
    let _e20: Motor = self_1713;
    let _e24: MultiVector = other_1411;
    let _e36: Motor = self_1713;
    let _e40: MultiVector = other_1411;
    let _e52: Motor = self_1713;
    let _e56: MultiVector = other_1411;
    let _e68: Motor = self_1713;
    let _e72: MultiVector = other_1411;
    let _e76: Motor = self_1713;
    let _e80: MultiVector = other_1411;
    let _e93: Motor = self_1713;
    let _e97: MultiVector = other_1411;
    let _e110: Motor = self_1713;
    let _e114: MultiVector = other_1411;
    let _e127: Motor = self_1713;
    let _e131: MultiVector = other_1411;
    let _e134: Motor = self_1713;
    let _e138: MultiVector = other_1411;
    let _e151: Motor = self_1713;
    let _e155: MultiVector = other_1411;
    let _e168: Motor = self_1713;
    let _e172: MultiVector = other_1411;
    let _e185: Motor = self_1713;
    let _e189: MultiVector = other_1411;
    let _e192: Motor = self_1713;
    let _e196: MultiVector = other_1411;
    let _e209: Motor = self_1713;
    let _e213: MultiVector = other_1411;
    let _e226: Motor = self_1713;
    let _e230: MultiVector = other_1411;
    let _e243: Motor = self_1713;
    let _e247: MultiVector = other_1411;
    let _e251: Motor = self_1713;
    let _e255: MultiVector = other_1411;
    let _e268: Motor = self_1713;
    let _e272: MultiVector = other_1411;
    let _e285: Motor = self_1713;
    let _e289: MultiVector = other_1411;
    let _e302: Motor = self_1713;
    let _e306: MultiVector = other_1411;
    let _e309: Motor = self_1713;
    let _e313: MultiVector = other_1411;
    let _e326: Motor = self_1713;
    let _e330: MultiVector = other_1411;
    let _e343: Motor = self_1713;
    let _e347: MultiVector = other_1411;
    return MultiVector((((((((((vec4<f32>(_e4.g0_.x) * _e8.g3_) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))) + ((vec4<f32>(_e20.g0_.y) * _e24.g3_.yxwz) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e36.g0_.z) * _e40.g3_.zwxy) * vec4<f32>(1.0, -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e52.g0_.w) * _e56.g3_.wzyx) * vec4<f32>(1.0, 1.0, -(1.0), 1.0))) + (vec4<f32>(_e68.g1_.x) * _e72.g0_)) + ((vec4<f32>(_e76.g1_.y) * _e80.g0_.yxwz) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e93.g1_.z) * _e97.g0_.zwxy) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e110.g1_.w) * _e114.g0_.wzyx) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))), ((((vec4<f32>(_e127.g1_.x) * _e131.g1_) + ((vec4<f32>(_e134.g1_.y) * _e138.g1_.yxwz) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e151.g1_.z) * _e155.g1_.zwxy) * vec4<f32>(-(1.0), -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e168.g1_.w) * _e172.g1_.wzyx) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))), ((((((((vec4<f32>(_e185.g0_.x) * _e189.g1_) + ((vec4<f32>(_e192.g0_.y) * _e196.g1_.yxwz) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e209.g0_.z) * _e213.g1_.zwxy) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e226.g0_.w) * _e230.g1_.wzyx) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))) + (vec4<f32>(_e243.g1_.x) * _e247.g2_)) + ((vec4<f32>(_e251.g1_.y) * _e255.g2_.yxwz) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e268.g1_.z) * _e272.g2_.zwxy) * vec4<f32>(-(1.0), -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e285.g1_.w) * _e289.g2_.wzyx) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))), ((((vec4<f32>(_e302.g1_.x) * _e306.g3_) + ((vec4<f32>(_e309.g1_.y) * _e313.g3_.yxwz) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e326.g1_.z) * _e330.g3_.zwxy) * vec4<f32>(-(1.0), -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e343.g1_.w) * _e347.g3_.wzyx) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))));
}

fn motor_multi_vector_inner_anti_product(self_1714: Motor, other_1412: MultiVector) -> MultiVector {
    var self_1715: Motor;
    var other_1413: MultiVector;

    self_1715 = self_1714;
    other_1413 = other_1412;
    let _e4: Motor = self_1715;
    let _e8: MultiVector = other_1413;
    let _e20: Motor = self_1715;
    let _e24: MultiVector = other_1413;
    let _e28: Motor = self_1715;
    let _e32: MultiVector = other_1413;
    let _e45: Motor = self_1715;
    let _e49: MultiVector = other_1413;
    let _e62: Motor = self_1715;
    let _e66: MultiVector = other_1413;
    let _e79: Motor = self_1715;
    let _e81: MultiVector = other_1413;
    let _e93: Motor = self_1715;
    let _e97: MultiVector = other_1413;
    let _e100: Motor = self_1715;
    let _e104: MultiVector = other_1413;
    let _e116: Motor = self_1715;
    let _e120: MultiVector = other_1413;
    let _e132: Motor = self_1715;
    let _e135: MultiVector = other_1413;
    let _e147: Motor = self_1715;
    let _e151: MultiVector = other_1413;
    let _e154: Motor = self_1715;
    let _e158: MultiVector = other_1413;
    let _e170: Motor = self_1715;
    let _e174: MultiVector = other_1413;
    let _e186: Motor = self_1715;
    let _e190: MultiVector = other_1413;
    let _e194: Motor = self_1715;
    let _e198: MultiVector = other_1413;
    let _e211: Motor = self_1715;
    let _e215: MultiVector = other_1413;
    let _e228: Motor = self_1715;
    let _e232: MultiVector = other_1413;
    let _e245: Motor = self_1715;
    let _e248: MultiVector = other_1413;
    let _e259: Motor = self_1715;
    let _e263: MultiVector = other_1413;
    let _e266: Motor = self_1715;
    let _e270: MultiVector = other_1413;
    let _e282: Motor = self_1715;
    let _e286: MultiVector = other_1413;
    let _e298: Motor = self_1715;
    let _e301: MultiVector = other_1413;
    return MultiVector((((((((vec4<f32>(_e4.g0_.x) * _e8.g3_) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))) + (vec4<f32>(_e20.g1_.x) * _e24.g0_)) + ((vec4<f32>(_e28.g1_.y) * vec4<f32>(_e32.g0_.x)) * vec4<f32>(0.0, -(1.0), 0.0, 0.0))) + ((vec4<f32>(_e45.g1_.z) * vec4<f32>(_e49.g0_.x)) * vec4<f32>(0.0, 0.0, -(1.0), 0.0))) + ((vec4<f32>(_e62.g1_.w) * vec4<f32>(_e66.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((_e79.g0_ * vec4<f32>(_e81.g3_.x)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))), ((((vec4<f32>(_e93.g1_.x) * _e97.g1_) + ((vec4<f32>(_e100.g1_.z) * _e104.g1_.wwxy) * vec4<f32>(0.0, -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e116.g1_.w) * _e120.g1_.zzyx) * vec4<f32>(0.0, 1.0, -(1.0), 1.0))) + ((_e132.g1_.xyyy * _e135.g1_.xxwz) * vec4<f32>(0.0, 1.0, 1.0, -(1.0)))), ((((((((vec4<f32>(_e147.g0_.x) * _e151.g1_) + ((vec4<f32>(_e154.g0_.z) * vec4<f32>(_e158.g1_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e170.g0_.w) * vec4<f32>(_e174.g1_.w)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + (vec4<f32>(_e186.g1_.x) * _e190.g2_)) + ((vec4<f32>(_e194.g1_.y) * vec4<f32>(_e198.g2_.y)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((vec4<f32>(_e211.g1_.z) * vec4<f32>(_e215.g2_.z)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((vec4<f32>(_e228.g1_.w) * vec4<f32>(_e232.g2_.w)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((_e245.g0_.yxxx * _e248.g1_.yxxx) * vec4<f32>(1.0, 0.0, 0.0, 0.0))), ((((vec4<f32>(_e259.g1_.x) * _e263.g3_) + ((vec4<f32>(_e266.g1_.z) * _e270.g3_.zzxz) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0))) + ((vec4<f32>(_e282.g1_.w) * _e286.g3_.wwwx) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))) + ((_e298.g1_.yyxx * _e301.g3_.yxxx) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))));
}

fn motor_multi_vector_left_contraction(self_1716: Motor, other_1414: MultiVector) -> MultiVector {
    var self_1717: Motor;
    var other_1415: MultiVector;

    self_1717 = self_1716;
    other_1415 = other_1414;
    let _e4: Motor = self_1717;
    let _e8: MultiVector = other_1415;
    let _e11: Motor = self_1717;
    let _e15: MultiVector = other_1415;
    let _e28: Motor = self_1717;
    let _e32: MultiVector = other_1415;
    let _e45: Motor = self_1717;
    let _e48: MultiVector = other_1415;
    let _e60: Motor = self_1717;
    let _e64: MultiVector = other_1415;
    let _e67: Motor = self_1717;
    let _e71: MultiVector = other_1415;
    let _e83: Motor = self_1717;
    let _e87: MultiVector = other_1415;
    let _e99: Motor = self_1717;
    let _e102: MultiVector = other_1415;
    let _e113: Motor = self_1717;
    let _e117: MultiVector = other_1415;
    let _e120: Motor = self_1717;
    let _e122: MultiVector = other_1415;
    let _e137: Motor = self_1717;
    let _e141: MultiVector = other_1415;
    let _e144: Motor = self_1717;
    let _e146: MultiVector = other_1415;
    return MultiVector(((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.z) * vec4<f32>(_e15.g0_.z)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((vec4<f32>(_e28.g0_.w) * vec4<f32>(_e32.g0_.w)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((_e45.g0_.yxxx * _e48.g0_.yxxx) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))), ((((vec4<f32>(_e60.g0_.x) * _e64.g1_) + ((vec4<f32>(_e67.g0_.z) * vec4<f32>(_e71.g1_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e83.g0_.w) * vec4<f32>(_e87.g1_.w)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((_e99.g0_.yxxx * _e102.g1_.yxxx) * vec4<f32>(1.0, 0.0, 0.0, 0.0))), ((vec4<f32>(_e113.g0_.x) * _e117.g2_) + ((_e120.g0_ * vec4<f32>(_e122.g2_.x)) * vec4<f32>(0.0, -(1.0), -(1.0), -(1.0)))), ((vec4<f32>(_e137.g0_.x) * _e141.g3_) + ((_e144.g0_ * vec4<f32>(_e146.g3_.x)) * vec4<f32>(0.0, -(1.0), -(1.0), -(1.0)))));
}

fn motor_multi_vector_left_anti_contraction(self_1718: Motor, other_1416: MultiVector) -> MultiVector {
    var self_1719: Motor;
    var other_1417: MultiVector;

    self_1719 = self_1718;
    other_1417 = other_1416;
    let _e4: Motor = self_1719;
    let _e8: MultiVector = other_1417;
    let _e11: Motor = self_1719;
    let _e13: MultiVector = other_1417;
    let _e28: Motor = self_1719;
    let _e32: MultiVector = other_1417;
    let _e35: Motor = self_1719;
    let _e37: MultiVector = other_1417;
    let _e49: Motor = self_1719;
    let _e53: MultiVector = other_1417;
    let _e56: Motor = self_1719;
    let _e60: MultiVector = other_1417;
    let _e73: Motor = self_1719;
    let _e77: MultiVector = other_1417;
    let _e90: Motor = self_1719;
    let _e93: MultiVector = other_1417;
    let _e105: Motor = self_1719;
    let _e109: MultiVector = other_1417;
    let _e112: Motor = self_1719;
    let _e116: MultiVector = other_1417;
    let _e129: Motor = self_1719;
    let _e133: MultiVector = other_1417;
    let _e146: Motor = self_1719;
    let _e149: MultiVector = other_1417;
    return MultiVector(((vec4<f32>(_e4.g1_.x) * _e8.g0_) + ((_e11.g1_ * vec4<f32>(_e13.g0_.x)) * vec4<f32>(0.0, -(1.0), -(1.0), -(1.0)))), ((vec4<f32>(_e28.g1_.x) * _e32.g1_) + ((_e35.g1_ * vec4<f32>(_e37.g1_.x)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))), ((((vec4<f32>(_e49.g1_.x) * _e53.g2_) + ((vec4<f32>(_e56.g1_.z) * vec4<f32>(_e60.g2_.z)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((vec4<f32>(_e73.g1_.w) * vec4<f32>(_e77.g2_.w)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((_e90.g1_.yxxx * _e93.g2_.yxxx) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))), ((((vec4<f32>(_e105.g1_.x) * _e109.g3_) + ((vec4<f32>(_e112.g1_.z) * vec4<f32>(_e116.g3_.z)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((vec4<f32>(_e129.g1_.w) * vec4<f32>(_e133.g3_.w)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((_e146.g1_.yxxx * _e149.g3_.yxxx) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))));
}

fn motor_multi_vector_scalar_product(self_1720: Motor, other_1418: MultiVector) -> Scalar {
    var self_1721: Motor;
    var other_1419: MultiVector;

    self_1721 = self_1720;
    other_1419 = other_1418;
    let _e4: Motor = self_1721;
    let _e7: MultiVector = other_1419;
    let _e11: Motor = self_1721;
    let _e14: MultiVector = other_1419;
    let _e19: Motor = self_1721;
    let _e22: MultiVector = other_1419;
    let _e27: Motor = self_1721;
    let _e30: MultiVector = other_1419;
    return Scalar(((((_e4.g0_.x * _e7.g0_.x) - (_e11.g0_.y * _e14.g0_.y)) - (_e19.g0_.z * _e22.g0_.z)) - (_e27.g0_.w * _e30.g0_.w)));
}

fn motor_multi_vector_anti_scalar_product(self_1722: Motor, other_1420: MultiVector) -> AntiScalar {
    var self_1723: Motor;
    var other_1421: MultiVector;

    self_1723 = self_1722;
    other_1421 = other_1420;
    let _e4: Motor = self_1723;
    let _e7: MultiVector = other_1421;
    let _e11: Motor = self_1723;
    let _e14: MultiVector = other_1421;
    let _e19: Motor = self_1723;
    let _e22: MultiVector = other_1421;
    let _e27: Motor = self_1723;
    let _e30: MultiVector = other_1421;
    return AntiScalar(((((_e4.g1_.x * _e7.g3_.x) - (_e11.g1_.y * _e14.g3_.y)) - (_e19.g1_.z * _e22.g3_.z)) - (_e27.g1_.w * _e30.g3_.w)));
}

fn motor_rotor_into(self_1724: Motor) -> Rotor {
    var self_1725: Motor;

    self_1725 = self_1724;
    let _e2: Motor = self_1725;
    return Rotor(_e2.g0_);
}

fn motor_rotor_add(self_1726: Motor, other_1422: Rotor) -> Motor {
    var self_1727: Motor;
    var other_1423: Rotor;

    self_1727 = self_1726;
    other_1423 = other_1422;
    let _e4: Motor = self_1727;
    let _e6: Rotor = other_1423;
    let _e9: Motor = self_1727;
    return Motor((_e4.g0_ + _e6.g0_), _e9.g1_);
}

fn motor_rotor_sub(self_1728: Motor, other_1424: Rotor) -> Motor {
    var self_1729: Motor;
    var other_1425: Rotor;

    self_1729 = self_1728;
    other_1425 = other_1424;
    let _e4: Motor = self_1729;
    let _e6: Rotor = other_1425;
    let _e9: Motor = self_1729;
    return Motor((_e4.g0_ - _e6.g0_), _e9.g1_);
}

fn motor_rotor_geometric_product(self_1730: Motor, other_1426: Rotor) -> Motor {
    var self_1731: Motor;
    var other_1427: Rotor;

    self_1731 = self_1730;
    other_1427 = other_1426;
    let _e4: Motor = self_1731;
    let _e8: Rotor = other_1427;
    let _e11: Motor = self_1731;
    let _e15: Rotor = other_1427;
    let _e28: Motor = self_1731;
    let _e32: Rotor = other_1427;
    let _e45: Motor = self_1731;
    let _e49: Rotor = other_1427;
    let _e62: Motor = self_1731;
    let _e66: Rotor = other_1427;
    let _e78: Motor = self_1731;
    let _e82: Rotor = other_1427;
    let _e94: Motor = self_1731;
    let _e98: Rotor = other_1427;
    let _e110: Motor = self_1731;
    let _e114: Rotor = other_1427;
    return Motor(((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.y) * _e15.g0_.yxwz) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e28.g0_.z) * _e32.g0_.zwxy) * vec4<f32>(-(1.0), -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e45.g0_.w) * _e49.g0_.wzyx) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))), (((((vec4<f32>(_e62.g1_.x) * _e66.g0_) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))) + ((vec4<f32>(_e78.g1_.y) * _e82.g0_.yxwz) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e94.g1_.z) * _e98.g0_.zwxy) * vec4<f32>(1.0, -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e110.g1_.w) * _e114.g0_.wzyx) * vec4<f32>(1.0, 1.0, -(1.0), 1.0))));
}

fn motor_rotor_regressive_product(self_1732: Motor, other_1428: Rotor) -> Rotor {
    var self_1733: Motor;
    var other_1429: Rotor;

    self_1733 = self_1732;
    other_1429 = other_1428;
    let _e4: Motor = self_1733;
    let _e8: Rotor = other_1429;
    let _e11: Motor = self_1733;
    let _e15: Rotor = other_1429;
    let _e27: Motor = self_1733;
    let _e31: Rotor = other_1429;
    let _e43: Motor = self_1733;
    let _e46: Rotor = other_1429;
    return Rotor(((((vec4<f32>(_e4.g1_.x) * _e8.g0_) + ((vec4<f32>(_e11.g1_.z) * vec4<f32>(_e15.g0_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e27.g1_.w) * vec4<f32>(_e31.g0_.w)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((_e43.g1_.yxxx * _e46.g0_.yxxx) * vec4<f32>(1.0, 0.0, 0.0, 0.0))));
}

fn motor_rotor_outer_product(self_1734: Motor, other_1430: Rotor) -> Motor {
    var self_1735: Motor;
    var other_1431: Rotor;

    self_1735 = self_1734;
    other_1431 = other_1430;
    let _e4: Motor = self_1735;
    let _e8: Rotor = other_1431;
    let _e11: Motor = self_1735;
    let _e13: Rotor = other_1431;
    let _e25: Motor = self_1735;
    let _e29: Rotor = other_1431;
    let _e39: Motor = self_1735;
    let _e43: Rotor = other_1431;
    let _e54: Motor = self_1735;
    let _e58: Rotor = other_1431;
    let _e69: Motor = self_1735;
    let _e73: Rotor = other_1431;
    return Motor(((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((_e11.g0_ * vec4<f32>(_e13.g0_.x)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))), (((((vec4<f32>(_e25.g1_.y) * _e29.g0_.yxyy) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) + ((vec4<f32>(_e39.g1_.z) * _e43.g0_.zzxz) * vec4<f32>(1.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e54.g1_.w) * _e58.g0_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e69.g1_.x) * vec4<f32>(_e73.g0_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))));
}

fn motor_rotor_inner_product(self_1736: Motor, other_1432: Rotor) -> Motor {
    var self_1737: Motor;
    var other_1433: Rotor;

    self_1737 = self_1736;
    other_1433 = other_1432;
    let _e4: Motor = self_1737;
    let _e8: Rotor = other_1433;
    let _e11: Motor = self_1737;
    let _e15: Rotor = other_1433;
    let _e27: Motor = self_1737;
    let _e31: Rotor = other_1433;
    let _e43: Motor = self_1737;
    let _e46: Rotor = other_1433;
    let _e58: Motor = self_1737;
    let _e62: Rotor = other_1433;
    let _e74: Motor = self_1737;
    let _e76: Rotor = other_1433;
    return Motor(((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.z) * _e15.g0_.zzxz) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0))) + ((vec4<f32>(_e27.g0_.w) * _e31.g0_.wwwx) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))) + ((_e43.g0_.yyxx * _e46.g0_.yxxx) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))), (((vec4<f32>(_e58.g1_.x) * _e62.g0_) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))) + ((_e74.g1_ * vec4<f32>(_e76.g0_.x)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn motor_rotor_geometric_anti_product(self_1738: Motor, other_1434: Rotor) -> Rotor {
    var self_1739: Motor;
    var other_1435: Rotor;

    self_1739 = self_1738;
    other_1435 = other_1434;
    let _e4: Motor = self_1739;
    let _e8: Rotor = other_1435;
    let _e11: Motor = self_1739;
    let _e15: Rotor = other_1435;
    let _e28: Motor = self_1739;
    let _e32: Rotor = other_1435;
    let _e45: Motor = self_1739;
    let _e49: Rotor = other_1435;
    return Rotor(((((vec4<f32>(_e4.g1_.x) * _e8.g0_) + ((vec4<f32>(_e11.g1_.y) * _e15.g0_.yxwz) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e28.g1_.z) * _e32.g0_.zwxy) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e45.g1_.w) * _e49.g0_.wzyx) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))));
}

fn motor_rotor_inner_anti_product(self_1740: Motor, other_1436: Rotor) -> Rotor {
    var self_1741: Motor;
    var other_1437: Rotor;

    self_1741 = self_1740;
    other_1437 = other_1436;
    let _e4: Motor = self_1741;
    let _e8: Rotor = other_1437;
    let _e11: Motor = self_1741;
    let _e13: Rotor = other_1437;
    return Rotor(((vec4<f32>(_e4.g1_.x) * _e8.g0_) + ((_e11.g1_ * vec4<f32>(_e13.g0_.x)) * vec4<f32>(0.0, -(1.0), -(1.0), -(1.0)))));
}

fn motor_rotor_left_contraction(self_1742: Motor, other_1438: Rotor) -> Rotor {
    var self_1743: Motor;
    var other_1439: Rotor;

    self_1743 = self_1742;
    other_1439 = other_1438;
    let _e4: Motor = self_1743;
    let _e8: Rotor = other_1439;
    let _e11: Motor = self_1743;
    let _e15: Rotor = other_1439;
    let _e28: Motor = self_1743;
    let _e32: Rotor = other_1439;
    let _e45: Motor = self_1743;
    let _e48: Rotor = other_1439;
    return Rotor(((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.z) * vec4<f32>(_e15.g0_.z)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((vec4<f32>(_e28.g0_.w) * vec4<f32>(_e32.g0_.w)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((_e45.g0_.yxxx * _e48.g0_.yxxx) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))));
}

fn motor_rotor_right_contraction(self_1744: Motor, other_1440: Rotor) -> Motor {
    var self_1745: Motor;
    var other_1441: Rotor;

    self_1745 = self_1744;
    other_1441 = other_1440;
    let _e4: Motor = self_1745;
    let _e8: Rotor = other_1441;
    let _e19: Motor = self_1745;
    let _e23: Rotor = other_1441;
    let _e35: Motor = self_1745;
    let _e39: Rotor = other_1441;
    let _e51: Motor = self_1745;
    let _e55: Rotor = other_1441;
    let _e67: Motor = self_1745;
    let _e71: Rotor = other_1441;
    let _e83: Motor = self_1745;
    let _e85: Rotor = other_1441;
    return Motor((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0)) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.zzxz) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0))) + ((vec4<f32>(_e35.g0_.w) * _e39.g0_.wwwx) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))) + ((vec4<f32>(_e51.g0_.x) * vec4<f32>(_e55.g0_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))), (((vec4<f32>(_e67.g1_.x) * _e71.g0_) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))) + ((_e83.g1_ * vec4<f32>(_e85.g0_.x)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn motor_rotor_left_anti_contraction(self_1746: Motor, other_1442: Rotor) -> Rotor {
    var self_1747: Motor;
    var other_1443: Rotor;

    self_1747 = self_1746;
    other_1443 = other_1442;
    let _e4: Motor = self_1747;
    let _e8: Rotor = other_1443;
    let _e11: Motor = self_1747;
    let _e13: Rotor = other_1443;
    return Rotor(((vec4<f32>(_e4.g1_.x) * _e8.g0_) + ((_e11.g1_ * vec4<f32>(_e13.g0_.x)) * vec4<f32>(0.0, -(1.0), -(1.0), -(1.0)))));
}

fn motor_rotor_scalar_product(self_1748: Motor, other_1444: Rotor) -> Scalar {
    var self_1749: Motor;
    var other_1445: Rotor;

    self_1749 = self_1748;
    other_1445 = other_1444;
    let _e4: Motor = self_1749;
    let _e7: Rotor = other_1445;
    let _e11: Motor = self_1749;
    let _e14: Rotor = other_1445;
    let _e19: Motor = self_1749;
    let _e22: Rotor = other_1445;
    let _e27: Motor = self_1749;
    let _e30: Rotor = other_1445;
    return Scalar(((((_e4.g0_.x * _e7.g0_.x) - (_e11.g0_.y * _e14.g0_.y)) - (_e19.g0_.z * _e22.g0_.z)) - (_e27.g0_.w * _e30.g0_.w)));
}

fn motor_point_geometric_product(self_1750: Motor, other_1446: Point) -> PointAndPlane {
    var self_1751: Motor;
    var other_1447: Point;

    self_1751 = self_1750;
    other_1447 = other_1446;
    let _e4: Motor = self_1751;
    let _e8: Point = other_1447;
    let _e11: Motor = self_1751;
    let _e15: Point = other_1447;
    let _e27: Motor = self_1751;
    let _e31: Point = other_1447;
    let _e43: Motor = self_1751;
    let _e47: Point = other_1447;
    let _e60: Motor = self_1751;
    let _e64: Point = other_1447;
    let _e77: Motor = self_1751;
    let _e80: Motor = self_1751;
    let _e83: Motor = self_1751;
    let _e86: Motor = self_1751;
    let _e90: Point = other_1447;
    let _e103: Motor = self_1751;
    let _e107: Point = other_1447;
    let _e118: Motor = self_1751;
    let _e122: Point = other_1447;
    let _e134: Motor = self_1751;
    let _e138: Point = other_1447;
    let _e151: Motor = self_1751;
    let _e154: Point = other_1447;
    return PointAndPlane(((((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.z) * _e15.g0_.wwwy) * vec4<f32>(0.0, -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e27.g0_.w) * _e31.g0_.zzyz) * vec4<f32>(0.0, 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e43.g1_.z) * vec4<f32>(_e47.g0_.x)) * vec4<f32>(0.0, 0.0, -(1.0), 0.0))) + ((vec4<f32>(_e60.g1_.w) * vec4<f32>(_e64.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e77.g0_.x, _e80.g1_.y, _e83.g0_.y, _e86.g0_.y) * _e90.g0_.xxwz) * vec4<f32>(0.0, -(1.0), 1.0, -(1.0)))), (((((vec4<f32>(_e103.g0_.z) * _e107.g0_.zzxz) * vec4<f32>(1.0, 0.0, -(1.0), 0.0)) + ((vec4<f32>(_e118.g0_.w) * _e122.g0_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e134.g1_.x) * vec4<f32>(_e138.g0_.x)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((_e151.g0_.yyxx * _e154.g0_.yxxx) * vec4<f32>(1.0, -(1.0), 0.0, 0.0))));
}

fn motor_point_regressive_product(self_1752: Motor, other_1448: Point) -> PointAndPlane {
    var self_1753: Motor;
    var other_1449: Point;

    self_1753 = self_1752;
    other_1449 = other_1448;
    let _e4: Motor = self_1753;
    let _e8: Point = other_1449;
    let _e11: Motor = self_1753;
    let _e15: Point = other_1449;
    let _e26: Motor = self_1753;
    let _e30: Point = other_1449;
    let _e42: Motor = self_1753;
    let _e46: Point = other_1449;
    let _e58: Motor = self_1753;
    let _e62: Point = other_1449;
    let _e74: Motor = self_1753;
    let _e77: Motor = self_1753;
    let _e80: Motor = self_1753;
    let _e83: Motor = self_1753;
    let _e87: Point = other_1449;
    return PointAndPlane((vec4<f32>(_e4.g1_.x) * _e8.g0_), ((((((vec4<f32>(_e11.g0_.z) * _e15.g0_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e26.g0_.w) * _e30.g0_.zzyz) * vec4<f32>(0.0, -(1.0), 1.0, 0.0))) + ((vec4<f32>(_e42.g1_.z) * _e46.g0_.zzxz) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0))) + ((vec4<f32>(_e58.g1_.w) * _e62.g0_.wwwx) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))) + ((vec4<f32>(_e74.g1_.y, _e77.g1_.y, _e80.g0_.y, _e83.g0_.y) * _e87.g0_.yxwz) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))));
}

fn motor_point_outer_product(self_1754: Motor, other_1450: Point) -> Point {
    var self_1755: Motor;
    var other_1451: Point;

    self_1755 = self_1754;
    other_1451 = other_1450;
    let _e4: Motor = self_1755;
    let _e8: Point = other_1451;
    return Point((vec4<f32>(_e4.g0_.x) * _e8.g0_));
}

fn motor_point_inner_product(self_1756: Motor, other_1452: Point) -> PointAndPlane {
    var self_1757: Motor;
    var other_1453: Point;

    self_1757 = self_1756;
    other_1453 = other_1452;
    let _e4: Motor = self_1757;
    let _e8: Point = other_1453;
    let _e11: Motor = self_1757;
    let _e15: Point = other_1453;
    let _e26: Motor = self_1757;
    let _e30: Point = other_1453;
    let _e42: Motor = self_1757;
    let _e46: Point = other_1453;
    let _e59: Motor = self_1757;
    let _e62: Point = other_1453;
    return PointAndPlane((vec4<f32>(_e4.g0_.x) * _e8.g0_), (((((vec4<f32>(_e11.g0_.z) * _e15.g0_.zzxz) * vec4<f32>(1.0, 0.0, -(1.0), 0.0)) + ((vec4<f32>(_e26.g0_.w) * _e30.g0_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e42.g1_.x) * vec4<f32>(_e46.g0_.x)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((_e59.g0_.yyxx * _e62.g0_.yxxx) * vec4<f32>(1.0, -(1.0), 0.0, 0.0))));
}

fn motor_point_geometric_anti_product(self_1758: Motor, other_1454: Point) -> PointAndPlane {
    var self_1759: Motor;
    var other_1455: Point;

    self_1759 = self_1758;
    other_1455 = other_1454;
    let _e4: Motor = self_1759;
    let _e8: Point = other_1455;
    let _e19: Motor = self_1759;
    let _e23: Point = other_1455;
    let _e35: Motor = self_1759;
    let _e39: Point = other_1455;
    let _e43: Motor = self_1759;
    let _e47: Point = other_1455;
    let _e59: Motor = self_1759;
    let _e63: Point = other_1455;
    let _e75: Motor = self_1759;
    let _e78: Motor = self_1759;
    let _e81: Motor = self_1759;
    let _e84: Motor = self_1759;
    let _e88: Point = other_1455;
    let _e100: Motor = self_1759;
    let _e104: Point = other_1455;
    let _e115: Motor = self_1759;
    let _e119: Point = other_1455;
    let _e131: Motor = self_1759;
    let _e135: Point = other_1455;
    let _e147: Motor = self_1759;
    let _e151: Point = other_1455;
    let _e163: Motor = self_1759;
    let _e167: Point = other_1455;
    let _e179: Motor = self_1759;
    let _e183: Point = other_1455;
    let _e195: Motor = self_1759;
    let _e199: Point = other_1455;
    return PointAndPlane((((((((vec4<f32>(_e4.g0_.z) * vec4<f32>(_e8.g0_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) + ((vec4<f32>(_e19.g0_.w) * vec4<f32>(_e23.g0_.w)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + (vec4<f32>(_e35.g1_.x) * _e39.g0_)) + ((vec4<f32>(_e43.g1_.z) * _e47.g0_.wwwy) * vec4<f32>(0.0, -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e59.g1_.w) * _e63.g0_.zzyz) * vec4<f32>(0.0, 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e75.g0_.y, _e78.g0_.x, _e81.g1_.y, _e84.g1_.y) * _e88.g0_.yxwz) * vec4<f32>(1.0, 0.0, 1.0, -(1.0)))), ((((((((vec4<f32>(_e100.g0_.y) * _e104.g0_.wwwz) * vec4<f32>(0.0, 0.0, -(1.0), 1.0)) + ((vec4<f32>(_e115.g0_.z) * _e119.g0_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0)))) + ((vec4<f32>(_e131.g0_.w) * _e135.g0_.zzyz) * vec4<f32>(0.0, -(1.0), 1.0, 0.0))) + ((vec4<f32>(_e147.g1_.y) * _e151.g0_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))) + ((vec4<f32>(_e163.g1_.z) * _e167.g0_.zzxz) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0))) + ((vec4<f32>(_e179.g1_.w) * _e183.g0_.wwwx) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))) + ((vec4<f32>(_e195.g0_.x) * _e199.g0_) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn motor_point_left_contraction(self_1760: Motor, other_1456: Point) -> PointAndPlane {
    var self_1761: Motor;
    var other_1457: Point;

    self_1761 = self_1760;
    other_1457 = other_1456;
    let _e4: Motor = self_1761;
    let _e8: Point = other_1457;
    let _e11: Motor = self_1761;
    let _e15: Point = other_1457;
    let _e26: Motor = self_1761;
    let _e30: Point = other_1457;
    let _e42: Motor = self_1761;
    let _e45: Point = other_1457;
    return PointAndPlane((vec4<f32>(_e4.g0_.x) * _e8.g0_), ((((vec4<f32>(_e11.g0_.z) * _e15.g0_.zzxz) * vec4<f32>(1.0, 0.0, -(1.0), 0.0)) + ((vec4<f32>(_e26.g0_.w) * _e30.g0_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))) + ((_e42.g0_.yyxx * _e45.g0_.yxxx) * vec4<f32>(1.0, -(1.0), 0.0, 0.0))));
}

fn motor_point_left_anti_contraction(self_1762: Motor, other_1458: Point) -> Point {
    var self_1763: Motor;
    var other_1459: Point;

    self_1763 = self_1762;
    other_1459 = other_1458;
    let _e4: Motor = self_1763;
    let _e8: Point = other_1459;
    return Point((vec4<f32>(_e4.g1_.x) * _e8.g0_));
}

fn motor_ideal_point_into(self_1764: Motor) -> IdealPoint {
    var self_1765: Motor;

    self_1765 = self_1764;
    let _e2: Motor = self_1765;
    let _e5: Motor = self_1765;
    let _e8: Motor = self_1765;
    return IdealPoint(vec3<f32>(_e2.g1_.y, _e5.g1_.z, _e8.g1_.w));
}

fn motor_ideal_point_add(self_1766: Motor, other_1460: IdealPoint) -> Motor {
    var self_1767: Motor;
    var other_1461: IdealPoint;

    self_1767 = self_1766;
    other_1461 = other_1460;
    let _e4: Motor = self_1767;
    let _e6: Motor = self_1767;
    let _e8: IdealPoint = other_1461;
    let _e11: IdealPoint = other_1461;
    let _e14: IdealPoint = other_1461;
    let _e17: IdealPoint = other_1461;
    return Motor(_e4.g0_, (_e6.g1_ + (vec4<f32>(_e8.g0_.x, _e11.g0_.x, _e14.g0_.y, _e17.g0_.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn motor_ideal_point_sub(self_1768: Motor, other_1462: IdealPoint) -> Motor {
    var self_1769: Motor;
    var other_1463: IdealPoint;

    self_1769 = self_1768;
    other_1463 = other_1462;
    let _e4: Motor = self_1769;
    let _e6: Motor = self_1769;
    let _e8: IdealPoint = other_1463;
    let _e11: IdealPoint = other_1463;
    let _e14: IdealPoint = other_1463;
    let _e17: IdealPoint = other_1463;
    return Motor(_e4.g0_, (_e6.g1_ - (vec4<f32>(_e8.g0_.x, _e11.g0_.x, _e14.g0_.y, _e17.g0_.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn motor_ideal_point_regressive_product(self_1770: Motor, other_1464: IdealPoint) -> Translator {
    var self_1771: Motor;
    var other_1465: IdealPoint;

    self_1771 = self_1770;
    other_1465 = other_1464;
    let _e4: Motor = self_1771;
    let _e8: IdealPoint = other_1465;
    let _e19: Motor = self_1771;
    let _e23: IdealPoint = other_1465;
    let _e35: Motor = self_1771;
    let _e38: Motor = self_1771;
    let _e41: Motor = self_1771;
    let _e44: Motor = self_1771;
    let _e48: IdealPoint = other_1465;
    let _e51: IdealPoint = other_1465;
    let _e54: IdealPoint = other_1465;
    let _e57: IdealPoint = other_1465;
    return Translator(((((vec4<f32>(_e4.g0_.z) * vec4<f32>(_e8.g0_.y)) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) + ((vec4<f32>(_e19.g0_.w) * vec4<f32>(_e23.g0_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + (vec4<f32>(_e35.g0_.y, _e38.g1_.x, _e41.g1_.x, _e44.g1_.x) * vec4<f32>(_e48.g0_.x, _e51.g0_.x, _e54.g0_.y, _e57.g0_.z))));
}

fn motor_ideal_point_inner_product(self_1772: Motor, other_1466: IdealPoint) -> IdealPoint {
    var self_1773: Motor;
    var other_1467: IdealPoint;

    self_1773 = self_1772;
    other_1467 = other_1466;
    let _e4: Motor = self_1773;
    let _e8: IdealPoint = other_1467;
    return IdealPoint((vec3<f32>(_e4.g0_.x) * _e8.g0_));
}

fn motor_ideal_point_geometric_anti_product(self_1774: Motor, other_1468: IdealPoint) -> Motor {
    var self_1775: Motor;
    var other_1469: IdealPoint;

    self_1775 = self_1774;
    other_1469 = other_1468;
    let _e4: Motor = self_1775;
    let _e8: IdealPoint = other_1469;
    let _e11: IdealPoint = other_1469;
    let _e14: IdealPoint = other_1469;
    let _e17: IdealPoint = other_1469;
    let _e29: Motor = self_1775;
    let _e33: IdealPoint = other_1469;
    let _e36: IdealPoint = other_1469;
    let _e39: IdealPoint = other_1469;
    let _e42: IdealPoint = other_1469;
    let _e55: Motor = self_1775;
    let _e59: IdealPoint = other_1469;
    let _e62: IdealPoint = other_1469;
    let _e65: IdealPoint = other_1469;
    let _e68: IdealPoint = other_1469;
    let _e81: Motor = self_1775;
    let _e85: IdealPoint = other_1469;
    let _e88: IdealPoint = other_1469;
    let _e91: IdealPoint = other_1469;
    let _e94: IdealPoint = other_1469;
    let _e109: Motor = self_1775;
    let _e113: IdealPoint = other_1469;
    let _e116: IdealPoint = other_1469;
    let _e119: IdealPoint = other_1469;
    let _e122: IdealPoint = other_1469;
    let _e135: Motor = self_1775;
    let _e139: IdealPoint = other_1469;
    let _e142: IdealPoint = other_1469;
    let _e145: IdealPoint = other_1469;
    let _e148: IdealPoint = other_1469;
    let _e162: Motor = self_1775;
    let _e166: IdealPoint = other_1469;
    let _e169: IdealPoint = other_1469;
    let _e172: IdealPoint = other_1469;
    let _e175: IdealPoint = other_1469;
    let _e189: Motor = self_1775;
    let _e193: IdealPoint = other_1469;
    let _e196: IdealPoint = other_1469;
    let _e199: IdealPoint = other_1469;
    let _e202: IdealPoint = other_1469;
    return Motor((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.x, _e11.g0_.x, _e14.g0_.z, _e17.g0_.y)) * vec4<f32>(1.0, 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e29.g0_.z) * vec4<f32>(_e33.g0_.y, _e36.g0_.z, _e39.g0_.y, _e42.g0_.x)) * vec4<f32>(1.0, -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e55.g0_.w) * vec4<f32>(_e59.g0_.z, _e62.g0_.y, _e65.g0_.x, _e68.g0_.z)) * vec4<f32>(1.0, 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e81.g0_.x) * vec4<f32>(_e85.g0_.x, _e88.g0_.x, _e91.g0_.y, _e94.g0_.z)) * vec4<f32>(0.0, -(1.0), -(1.0), -(1.0)))), (((((vec4<f32>(_e109.g1_.y) * vec4<f32>(_e113.g0_.x, _e116.g0_.x, _e119.g0_.z, _e122.g0_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e135.g1_.z) * vec4<f32>(_e139.g0_.y, _e142.g0_.z, _e145.g0_.y, _e148.g0_.x)) * vec4<f32>(-(1.0), -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e162.g1_.w) * vec4<f32>(_e166.g0_.z, _e169.g0_.y, _e172.g0_.x, _e175.g0_.z)) * vec4<f32>(-(1.0), 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e189.g1_.x) * vec4<f32>(_e193.g0_.x, _e196.g0_.x, _e199.g0_.y, _e202.g0_.z)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn motor_ideal_point_left_contraction(self_1776: Motor, other_1470: IdealPoint) -> IdealPoint {
    var self_1777: Motor;
    var other_1471: IdealPoint;

    self_1777 = self_1776;
    other_1471 = other_1470;
    let _e4: Motor = self_1777;
    let _e8: IdealPoint = other_1471;
    return IdealPoint((vec3<f32>(_e4.g0_.x) * _e8.g0_));
}

fn motor_ideal_point_anti_scalar_product(self_1778: Motor, other_1472: IdealPoint) -> AntiScalar {
    var self_1779: Motor;
    var other_1473: IdealPoint;

    self_1779 = self_1778;
    other_1473 = other_1472;
    let _e5: Motor = self_1779;
    let _e8: IdealPoint = other_1473;
    let _e13: Motor = self_1779;
    let _e16: IdealPoint = other_1473;
    let _e21: Motor = self_1779;
    let _e24: IdealPoint = other_1473;
    return AntiScalar((((0.0 - (_e5.g1_.y * _e8.g0_.x)) - (_e13.g1_.z * _e16.g0_.y)) - (_e21.g1_.w * _e24.g0_.z)));
}

fn motor_plane_geometric_product(self_1780: Motor, other_1474: Plane) -> PointAndPlane {
    var self_1781: Motor;
    var other_1475: Plane;

    self_1781 = self_1780;
    other_1475 = other_1474;
    let _e4: Motor = self_1781;
    let _e8: Plane = other_1475;
    let _e19: Motor = self_1781;
    let _e23: Plane = other_1475;
    let _e35: Motor = self_1781;
    let _e39: Plane = other_1475;
    let _e53: Motor = self_1781;
    let _e57: Plane = other_1475;
    let _e69: Motor = self_1781;
    let _e73: Plane = other_1475;
    let _e85: Motor = self_1781;
    let _e88: Motor = self_1781;
    let _e91: Motor = self_1781;
    let _e94: Motor = self_1781;
    let _e98: Plane = other_1475;
    let _e111: Motor = self_1781;
    let _e115: Plane = other_1475;
    let _e118: Motor = self_1781;
    let _e122: Plane = other_1475;
    let _e134: Motor = self_1781;
    let _e138: Plane = other_1475;
    let _e150: Motor = self_1781;
    let _e154: Plane = other_1475;
    let _e166: Motor = self_1781;
    let _e170: Plane = other_1475;
    let _e182: Motor = self_1781;
    let _e185: Motor = self_1781;
    let _e188: Motor = self_1781;
    let _e191: Motor = self_1781;
    let _e195: Plane = other_1475;
    return PointAndPlane((((((((vec4<f32>(_e4.g0_.z) * _e8.g0_.zzxz) * vec4<f32>(1.0, 0.0, -(1.0), 0.0)) + ((vec4<f32>(_e19.g0_.w) * _e23.g0_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e35.g1_.x) * _e39.g0_.yyzw) * vec4<f32>(0.0, -(1.0), -(1.0), -(1.0)))) + ((vec4<f32>(_e53.g1_.z) * _e57.g0_.wwwy) * vec4<f32>(0.0, -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e69.g1_.w) * _e73.g0_.zzyz) * vec4<f32>(0.0, 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e85.g0_.y, _e88.g0_.y, _e91.g1_.y, _e94.g1_.y) * _e98.g0_.yxwz) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))), ((((((vec4<f32>(_e111.g0_.x) * _e115.g0_) + ((vec4<f32>(_e118.g0_.z) * _e122.g0_.wwwy) * vec4<f32>(0.0, -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e134.g0_.w) * _e138.g0_.zzyz) * vec4<f32>(0.0, 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e150.g1_.z) * vec4<f32>(_e154.g0_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e166.g1_.w) * vec4<f32>(_e170.g0_.w)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e182.g1_.y, _e185.g0_.x, _e188.g0_.y, _e191.g0_.y) * _e195.g0_.yxwz) * vec4<f32>(1.0, 0.0, 1.0, -(1.0)))));
}

fn motor_plane_regressive_product(self_1782: Motor, other_1476: Plane) -> Plane {
    var self_1783: Motor;
    var other_1477: Plane;

    self_1783 = self_1782;
    other_1477 = other_1476;
    let _e4: Motor = self_1783;
    let _e8: Plane = other_1477;
    return Plane((vec4<f32>(_e4.g1_.x) * _e8.g0_));
}

fn motor_plane_outer_product(self_1784: Motor, other_1478: Plane) -> PointAndPlane {
    var self_1785: Motor;
    var other_1479: Plane;

    self_1785 = self_1784;
    other_1479 = other_1478;
    let _e4: Motor = self_1785;
    let _e8: Plane = other_1479;
    let _e19: Motor = self_1785;
    let _e23: Plane = other_1479;
    let _e35: Motor = self_1785;
    let _e39: Plane = other_1479;
    let _e51: Motor = self_1785;
    let _e55: Plane = other_1479;
    let _e67: Motor = self_1785;
    let _e70: Motor = self_1785;
    let _e73: Motor = self_1785;
    let _e76: Motor = self_1785;
    let _e80: Plane = other_1479;
    let _e93: Motor = self_1785;
    let _e97: Plane = other_1479;
    return PointAndPlane(((((((vec4<f32>(_e4.g0_.z) * _e8.g0_.zzxz) * vec4<f32>(1.0, 0.0, -(1.0), 0.0)) + ((vec4<f32>(_e19.g0_.w) * _e23.g0_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e35.g1_.z) * _e39.g0_.wwwy) * vec4<f32>(0.0, -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e51.g1_.w) * _e55.g0_.zzyz) * vec4<f32>(0.0, 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e67.g0_.y, _e70.g0_.y, _e73.g1_.y, _e76.g1_.y) * _e80.g0_.yxwz) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))), (vec4<f32>(_e93.g0_.x) * _e97.g0_));
}

fn motor_plane_geometric_anti_product(self_1786: Motor, other_1480: Plane) -> PointAndPlane {
    var self_1787: Motor;
    var other_1481: Plane;

    self_1787 = self_1786;
    other_1481 = other_1480;
    let _e4: Motor = self_1787;
    let _e8: Plane = other_1481;
    let _e19: Motor = self_1787;
    let _e23: Plane = other_1481;
    let _e35: Motor = self_1787;
    let _e39: Plane = other_1481;
    let _e51: Motor = self_1787;
    let _e55: Plane = other_1481;
    let _e67: Motor = self_1787;
    let _e71: Plane = other_1481;
    let _e74: Motor = self_1787;
    let _e78: Plane = other_1481;
    let _e90: Motor = self_1787;
    let _e94: Plane = other_1481;
    let _e106: Motor = self_1787;
    let _e110: Plane = other_1481;
    let _e122: Motor = self_1787;
    let _e124: Plane = other_1481;
    return PointAndPlane((((((vec4<f32>(_e4.g1_.y) * _e8.g0_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0)) + ((vec4<f32>(_e19.g1_.z) * _e23.g0_.zzxz) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0))) + ((vec4<f32>(_e35.g1_.w) * _e39.g0_.wwwx) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))) + ((vec4<f32>(_e51.g0_.x) * vec4<f32>(_e55.g0_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))), (((((vec4<f32>(_e67.g1_.x) * _e71.g0_) + ((vec4<f32>(_e74.g1_.y) * _e78.g0_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e90.g1_.z) * _e94.g0_.wwwy) * vec4<f32>(0.0, -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e106.g1_.w) * _e110.g0_.zzyz) * vec4<f32>(0.0, 1.0, -(1.0), 0.0))) + ((_e122.g0_ * vec4<f32>(_e124.g0_.x)) * vec4<f32>(0.0, -(1.0), -(1.0), -(1.0)))));
}

fn motor_plane_inner_anti_product(self_1788: Motor, other_1482: Plane) -> PointAndPlane {
    var self_1789: Motor;
    var other_1483: Plane;

    self_1789 = self_1788;
    other_1483 = other_1482;
    let _e4: Motor = self_1789;
    let _e8: Plane = other_1483;
    let _e19: Motor = self_1789;
    let _e23: Plane = other_1483;
    let _e35: Motor = self_1789;
    let _e39: Plane = other_1483;
    let _e51: Motor = self_1789;
    let _e55: Plane = other_1483;
    let _e67: Motor = self_1789;
    let _e71: Plane = other_1483;
    return PointAndPlane((((((vec4<f32>(_e4.g1_.y) * _e8.g0_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0)) + ((vec4<f32>(_e19.g1_.z) * _e23.g0_.zzxz) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0))) + ((vec4<f32>(_e35.g1_.w) * _e39.g0_.wwwx) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))) + ((vec4<f32>(_e51.g0_.x) * vec4<f32>(_e55.g0_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))), (vec4<f32>(_e67.g1_.x) * _e71.g0_));
}

fn motor_plane_left_contraction(self_1790: Motor, other_1484: Plane) -> Plane {
    var self_1791: Motor;
    var other_1485: Plane;

    self_1791 = self_1790;
    other_1485 = other_1484;
    let _e4: Motor = self_1791;
    let _e8: Plane = other_1485;
    return Plane((vec4<f32>(_e4.g0_.x) * _e8.g0_));
}

fn motor_plane_left_anti_contraction(self_1792: Motor, other_1486: Plane) -> PointAndPlane {
    var self_1793: Motor;
    var other_1487: Plane;

    self_1793 = self_1792;
    other_1487 = other_1486;
    let _e4: Motor = self_1793;
    let _e8: Plane = other_1487;
    let _e19: Motor = self_1793;
    let _e23: Plane = other_1487;
    let _e35: Motor = self_1793;
    let _e38: Plane = other_1487;
    let _e50: Motor = self_1793;
    let _e54: Plane = other_1487;
    return PointAndPlane(((((vec4<f32>(_e4.g1_.z) * _e8.g0_.zzxz) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0)) + ((vec4<f32>(_e19.g1_.w) * _e23.g0_.wwwx) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))) + ((_e35.g1_.yyxx * _e38.g0_.yxxx) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))), (vec4<f32>(_e50.g1_.x) * _e54.g0_));
}

fn motor_line_into(self_1794: Motor) -> Line {
    var self_1795: Motor;

    self_1795 = self_1794;
    let _e2: Motor = self_1795;
    let _e5: Motor = self_1795;
    let _e8: Motor = self_1795;
    let _e12: Motor = self_1795;
    let _e15: Motor = self_1795;
    let _e18: Motor = self_1795;
    return Line(vec3<f32>(_e2.g1_.y, _e5.g1_.z, _e8.g1_.w), vec3<f32>(_e12.g0_.y, _e15.g0_.z, _e18.g0_.w));
}

fn motor_line_add(self_1796: Motor, other_1488: Line) -> Motor {
    var self_1797: Motor;
    var other_1489: Line;

    self_1797 = self_1796;
    other_1489 = other_1488;
    let _e4: Motor = self_1797;
    let _e6: Line = other_1489;
    let _e9: Line = other_1489;
    let _e12: Line = other_1489;
    let _e15: Line = other_1489;
    let _e26: Motor = self_1797;
    let _e28: Line = other_1489;
    let _e31: Line = other_1489;
    let _e34: Line = other_1489;
    let _e37: Line = other_1489;
    return Motor((_e4.g0_ + (vec4<f32>(_e6.g0_.x, _e9.g1_.x, _e12.g1_.y, _e15.g1_.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0))), (_e26.g1_ + (vec4<f32>(_e28.g0_.x, _e31.g0_.x, _e34.g0_.y, _e37.g0_.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn motor_line_sub(self_1798: Motor, other_1490: Line) -> Motor {
    var self_1799: Motor;
    var other_1491: Line;

    self_1799 = self_1798;
    other_1491 = other_1490;
    let _e4: Motor = self_1799;
    let _e6: Line = other_1491;
    let _e9: Line = other_1491;
    let _e12: Line = other_1491;
    let _e15: Line = other_1491;
    let _e26: Motor = self_1799;
    let _e28: Line = other_1491;
    let _e31: Line = other_1491;
    let _e34: Line = other_1491;
    let _e37: Line = other_1491;
    return Motor((_e4.g0_ - (vec4<f32>(_e6.g0_.x, _e9.g1_.x, _e12.g1_.y, _e15.g1_.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0))), (_e26.g1_ - (vec4<f32>(_e28.g0_.x, _e31.g0_.x, _e34.g0_.y, _e37.g0_.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn motor_line_geometric_product(self_1800: Motor, other_1492: Line) -> Motor {
    var self_1801: Motor;
    var other_1493: Line;

    self_1801 = self_1800;
    other_1493 = other_1492;
    let _e4: Motor = self_1801;
    let _e8: Line = other_1493;
    let _e11: Line = other_1493;
    let _e14: Line = other_1493;
    let _e17: Line = other_1493;
    let _e30: Motor = self_1801;
    let _e34: Line = other_1493;
    let _e37: Line = other_1493;
    let _e40: Line = other_1493;
    let _e43: Line = other_1493;
    let _e57: Motor = self_1801;
    let _e61: Line = other_1493;
    let _e64: Line = other_1493;
    let _e67: Line = other_1493;
    let _e70: Line = other_1493;
    let _e84: Motor = self_1801;
    let _e88: Line = other_1493;
    let _e91: Line = other_1493;
    let _e94: Line = other_1493;
    let _e97: Line = other_1493;
    let _e109: Motor = self_1801;
    let _e113: Line = other_1493;
    let _e116: Line = other_1493;
    let _e119: Line = other_1493;
    let _e122: Line = other_1493;
    let _e134: Motor = self_1801;
    let _e138: Line = other_1493;
    let _e141: Line = other_1493;
    let _e144: Line = other_1493;
    let _e147: Line = other_1493;
    let _e160: Motor = self_1801;
    let _e164: Line = other_1493;
    let _e167: Line = other_1493;
    let _e170: Line = other_1493;
    let _e173: Line = other_1493;
    let _e186: Motor = self_1801;
    let _e190: Line = other_1493;
    let _e193: Line = other_1493;
    let _e196: Line = other_1493;
    let _e199: Line = other_1493;
    let _e214: Motor = self_1801;
    let _e218: Line = other_1493;
    let _e221: Line = other_1493;
    let _e224: Line = other_1493;
    let _e227: Line = other_1493;
    let _e240: Motor = self_1801;
    let _e244: Line = other_1493;
    let _e247: Line = other_1493;
    let _e250: Line = other_1493;
    let _e253: Line = other_1493;
    let _e266: Motor = self_1801;
    let _e270: Line = other_1493;
    let _e273: Line = other_1493;
    let _e276: Line = other_1493;
    let _e279: Line = other_1493;
    let _e292: Motor = self_1801;
    let _e296: Line = other_1493;
    let _e299: Line = other_1493;
    let _e302: Line = other_1493;
    let _e305: Line = other_1493;
    return Motor((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g1_.x, _e11.g1_.x, _e14.g1_.z, _e17.g1_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e30.g0_.z) * vec4<f32>(_e34.g1_.y, _e37.g1_.z, _e40.g1_.y, _e43.g1_.x)) * vec4<f32>(-(1.0), -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e57.g0_.w) * vec4<f32>(_e61.g1_.z, _e64.g1_.y, _e67.g1_.x, _e70.g1_.z)) * vec4<f32>(-(1.0), 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e84.g0_.x) * vec4<f32>(_e88.g1_.x, _e91.g1_.x, _e94.g1_.y, _e97.g1_.z)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))), (((((((((vec4<f32>(_e109.g0_.y) * vec4<f32>(_e113.g0_.x, _e116.g0_.x, _e119.g0_.z, _e122.g0_.y)) * vec4<f32>(1.0, 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e134.g0_.z) * vec4<f32>(_e138.g0_.y, _e141.g0_.z, _e144.g0_.y, _e147.g0_.x)) * vec4<f32>(1.0, -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e160.g0_.w) * vec4<f32>(_e164.g0_.z, _e167.g0_.y, _e170.g0_.x, _e173.g0_.z)) * vec4<f32>(1.0, 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e186.g1_.x) * vec4<f32>(_e190.g1_.x, _e193.g1_.x, _e196.g1_.y, _e199.g1_.z)) * vec4<f32>(0.0, -(1.0), -(1.0), -(1.0)))) + ((vec4<f32>(_e214.g1_.y) * vec4<f32>(_e218.g1_.x, _e221.g1_.x, _e224.g1_.z, _e227.g1_.y)) * vec4<f32>(1.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e240.g1_.z) * vec4<f32>(_e244.g1_.y, _e247.g1_.z, _e250.g1_.y, _e253.g1_.x)) * vec4<f32>(1.0, -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e266.g1_.w) * vec4<f32>(_e270.g1_.z, _e273.g1_.y, _e276.g1_.x, _e279.g1_.z)) * vec4<f32>(1.0, 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e292.g0_.x) * vec4<f32>(_e296.g0_.x, _e299.g0_.x, _e302.g0_.y, _e305.g0_.z)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn motor_line_geometric_anti_product(self_1802: Motor, other_1494: Line) -> Motor {
    var self_1803: Motor;
    var other_1495: Line;

    self_1803 = self_1802;
    other_1495 = other_1494;
    let _e4: Motor = self_1803;
    let _e8: Line = other_1495;
    let _e11: Line = other_1495;
    let _e14: Line = other_1495;
    let _e17: Line = other_1495;
    let _e29: Motor = self_1803;
    let _e33: Line = other_1495;
    let _e36: Line = other_1495;
    let _e39: Line = other_1495;
    let _e42: Line = other_1495;
    let _e55: Motor = self_1803;
    let _e59: Line = other_1495;
    let _e62: Line = other_1495;
    let _e65: Line = other_1495;
    let _e68: Line = other_1495;
    let _e81: Motor = self_1803;
    let _e85: Line = other_1495;
    let _e88: Line = other_1495;
    let _e91: Line = other_1495;
    let _e94: Line = other_1495;
    let _e106: Motor = self_1803;
    let _e110: Line = other_1495;
    let _e113: Line = other_1495;
    let _e116: Line = other_1495;
    let _e119: Line = other_1495;
    let _e132: Motor = self_1803;
    let _e136: Line = other_1495;
    let _e139: Line = other_1495;
    let _e142: Line = other_1495;
    let _e145: Line = other_1495;
    let _e158: Motor = self_1803;
    let _e162: Line = other_1495;
    let _e165: Line = other_1495;
    let _e168: Line = other_1495;
    let _e171: Line = other_1495;
    let _e184: Motor = self_1803;
    let _e188: Line = other_1495;
    let _e191: Line = other_1495;
    let _e194: Line = other_1495;
    let _e197: Line = other_1495;
    let _e212: Motor = self_1803;
    let _e216: Line = other_1495;
    let _e219: Line = other_1495;
    let _e222: Line = other_1495;
    let _e225: Line = other_1495;
    let _e238: Motor = self_1803;
    let _e242: Line = other_1495;
    let _e245: Line = other_1495;
    let _e248: Line = other_1495;
    let _e251: Line = other_1495;
    let _e265: Motor = self_1803;
    let _e269: Line = other_1495;
    let _e272: Line = other_1495;
    let _e275: Line = other_1495;
    let _e278: Line = other_1495;
    let _e292: Motor = self_1803;
    let _e296: Line = other_1495;
    let _e299: Line = other_1495;
    let _e302: Line = other_1495;
    let _e305: Line = other_1495;
    return Motor((((((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.x, _e11.g0_.x, _e14.g0_.z, _e17.g0_.y)) * vec4<f32>(1.0, 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e29.g0_.z) * vec4<f32>(_e33.g0_.y, _e36.g0_.z, _e39.g0_.y, _e42.g0_.x)) * vec4<f32>(1.0, -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e55.g0_.w) * vec4<f32>(_e59.g0_.z, _e62.g0_.y, _e65.g0_.x, _e68.g0_.z)) * vec4<f32>(1.0, 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e81.g1_.x) * vec4<f32>(_e85.g1_.x, _e88.g1_.x, _e91.g1_.y, _e94.g1_.z)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))) + ((vec4<f32>(_e106.g1_.y) * vec4<f32>(_e110.g1_.x, _e113.g1_.x, _e116.g1_.z, _e119.g1_.y)) * vec4<f32>(1.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e132.g1_.z) * vec4<f32>(_e136.g1_.y, _e139.g1_.z, _e142.g1_.y, _e145.g1_.x)) * vec4<f32>(1.0, -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e158.g1_.w) * vec4<f32>(_e162.g1_.z, _e165.g1_.y, _e168.g1_.x, _e171.g1_.z)) * vec4<f32>(1.0, 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e184.g0_.x) * vec4<f32>(_e188.g0_.x, _e191.g0_.x, _e194.g0_.y, _e197.g0_.z)) * vec4<f32>(0.0, -(1.0), -(1.0), -(1.0)))), (((((vec4<f32>(_e212.g1_.y) * vec4<f32>(_e216.g0_.x, _e219.g0_.x, _e222.g0_.z, _e225.g0_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e238.g1_.z) * vec4<f32>(_e242.g0_.y, _e245.g0_.z, _e248.g0_.y, _e251.g0_.x)) * vec4<f32>(-(1.0), -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e265.g1_.w) * vec4<f32>(_e269.g0_.z, _e272.g0_.y, _e275.g0_.x, _e278.g0_.z)) * vec4<f32>(-(1.0), 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e292.g1_.x) * vec4<f32>(_e296.g0_.x, _e299.g0_.x, _e302.g0_.y, _e305.g0_.z)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn motor_line_right_contraction(self_1804: Motor, other_1496: Line) -> Translator {
    var self_1805: Motor;
    var other_1497: Line;

    self_1805 = self_1804;
    other_1497 = other_1496;
    let _e4: Motor = self_1805;
    let _e8: Line = other_1497;
    let _e20: Motor = self_1805;
    let _e24: Line = other_1497;
    let _e37: Motor = self_1805;
    let _e40: Motor = self_1805;
    let _e43: Motor = self_1805;
    let _e46: Motor = self_1805;
    let _e50: Line = other_1497;
    let _e53: Line = other_1497;
    let _e56: Line = other_1497;
    let _e59: Line = other_1497;
    return Translator(((((vec4<f32>(_e4.g0_.z) * vec4<f32>(_e8.g1_.y)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0)) + ((vec4<f32>(_e20.g0_.w) * vec4<f32>(_e24.g1_.z)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((vec4<f32>(_e37.g0_.y, _e40.g1_.x, _e43.g1_.x, _e46.g1_.x) * vec4<f32>(_e50.g1_.x, _e53.g1_.x, _e56.g1_.y, _e59.g1_.z)) * vec4<f32>(-(1.0)))));
}

fn motor_line_scalar_product(self_1806: Motor, other_1498: Line) -> Scalar {
    var self_1807: Motor;
    var other_1499: Line;

    self_1807 = self_1806;
    other_1499 = other_1498;
    let _e5: Motor = self_1807;
    let _e8: Line = other_1499;
    let _e13: Motor = self_1807;
    let _e16: Line = other_1499;
    let _e21: Motor = self_1807;
    let _e24: Line = other_1499;
    return Scalar((((0.0 - (_e5.g0_.y * _e8.g1_.x)) - (_e13.g0_.z * _e16.g1_.y)) - (_e21.g0_.w * _e24.g1_.z)));
}

fn motor_line_anti_scalar_product(self_1808: Motor, other_1500: Line) -> AntiScalar {
    var self_1809: Motor;
    var other_1501: Line;

    self_1809 = self_1808;
    other_1501 = other_1500;
    let _e5: Motor = self_1809;
    let _e8: Line = other_1501;
    let _e13: Motor = self_1809;
    let _e16: Line = other_1501;
    let _e21: Motor = self_1809;
    let _e24: Line = other_1501;
    return AntiScalar((((0.0 - (_e5.g1_.y * _e8.g0_.x)) - (_e13.g1_.z * _e16.g0_.y)) - (_e21.g1_.w * _e24.g0_.z)));
}

fn motor_translator_into(self_1810: Motor) -> Translator {
    var self_1811: Motor;

    self_1811 = self_1810;
    let _e2: Motor = self_1811;
    let _e5: Motor = self_1811;
    let _e8: Motor = self_1811;
    let _e11: Motor = self_1811;
    return Translator(vec4<f32>(_e2.g0_.x, _e5.g1_.y, _e8.g1_.z, _e11.g1_.w));
}

fn motor_translator_add(self_1812: Motor, other_1502: Translator) -> Motor {
    var self_1813: Motor;
    var other_1503: Translator;

    self_1813 = self_1812;
    other_1503 = other_1502;
    let _e4: Motor = self_1813;
    let _e6: Translator = other_1503;
    let _e17: Motor = self_1813;
    let _e19: Translator = other_1503;
    return Motor((_e4.g0_ + (vec4<f32>(_e6.g0_.x) * vec4<f32>(1.0, 0.0, 0.0, 0.0))), (_e17.g1_ + (_e19.g0_ * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn motor_translator_sub(self_1814: Motor, other_1504: Translator) -> Motor {
    var self_1815: Motor;
    var other_1505: Translator;

    self_1815 = self_1814;
    other_1505 = other_1504;
    let _e4: Motor = self_1815;
    let _e6: Translator = other_1505;
    let _e17: Motor = self_1815;
    let _e19: Translator = other_1505;
    return Motor((_e4.g0_ - (vec4<f32>(_e6.g0_.x) * vec4<f32>(1.0, 0.0, 0.0, 0.0))), (_e17.g1_ - (_e19.g0_ * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn motor_translator_geometric_product(self_1816: Motor, other_1506: Translator) -> Motor {
    var self_1817: Motor;
    var other_1507: Translator;

    self_1817 = self_1816;
    other_1507 = other_1506;
    let _e4: Motor = self_1817;
    let _e6: Translator = other_1507;
    let _e11: Motor = self_1817;
    let _e15: Translator = other_1507;
    let _e26: Motor = self_1817;
    let _e30: Translator = other_1507;
    let _e42: Motor = self_1817;
    let _e46: Translator = other_1507;
    let _e58: Motor = self_1817;
    let _e62: Translator = other_1507;
    let _e74: Motor = self_1817;
    let _e78: Translator = other_1507;
    let _e90: Motor = self_1817;
    let _e94: Translator = other_1507;
    let _e106: Motor = self_1817;
    let _e109: Motor = self_1817;
    let _e112: Motor = self_1817;
    let _e115: Motor = self_1817;
    let _e119: Translator = other_1507;
    return Motor((_e4.g0_ * vec4<f32>(_e6.g0_.x)), ((((((((vec4<f32>(_e11.g0_.y) * _e15.g0_.yywz) * vec4<f32>(1.0, 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e26.g0_.z) * _e30.g0_.zwzy) * vec4<f32>(1.0, -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e42.g0_.w) * _e46.g0_.wzyw) * vec4<f32>(1.0, 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e58.g1_.y) * vec4<f32>(_e62.g0_.x)) * vec4<f32>(0.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e74.g1_.z) * vec4<f32>(_e78.g0_.x)) * vec4<f32>(0.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e90.g1_.w) * vec4<f32>(_e94.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + (vec4<f32>(_e106.g1_.x, _e109.g0_.x, _e112.g0_.x, _e115.g0_.x) * _e119.g0_)));
}

fn motor_translator_regressive_product(self_1818: Motor, other_1508: Translator) -> Translator {
    var self_1819: Motor;
    var other_1509: Translator;

    self_1819 = self_1818;
    other_1509 = other_1508;
    let _e4: Motor = self_1819;
    let _e8: Translator = other_1509;
    let _e19: Motor = self_1819;
    let _e23: Translator = other_1509;
    let _e35: Motor = self_1819;
    let _e39: Translator = other_1509;
    let _e43: Motor = self_1819;
    let _e46: Translator = other_1509;
    return Translator((((((vec4<f32>(_e4.g0_.z) * vec4<f32>(_e8.g0_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) + ((vec4<f32>(_e19.g0_.w) * vec4<f32>(_e23.g0_.w)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + (vec4<f32>(_e35.g1_.x) * _e39.g0_)) + ((_e43.g0_.yxxx * _e46.g0_.yxxx) * vec4<f32>(1.0, 0.0, 0.0, 0.0))));
}

fn motor_translator_outer_product(self_1820: Motor, other_1510: Translator) -> Motor {
    var self_1821: Motor;
    var other_1511: Translator;

    self_1821 = self_1820;
    other_1511 = other_1510;
    let _e4: Motor = self_1821;
    let _e6: Translator = other_1511;
    let _e11: Motor = self_1821;
    let _e15: Translator = other_1511;
    let _e26: Motor = self_1821;
    let _e30: Translator = other_1511;
    let _e42: Motor = self_1821;
    let _e46: Translator = other_1511;
    let _e58: Motor = self_1821;
    let _e62: Translator = other_1511;
    let _e74: Motor = self_1821;
    let _e78: Translator = other_1511;
    let _e90: Motor = self_1821;
    let _e94: Translator = other_1511;
    let _e106: Motor = self_1821;
    let _e109: Translator = other_1511;
    return Motor((_e4.g0_ * vec4<f32>(_e6.g0_.x)), ((((((((vec4<f32>(_e11.g0_.z) * vec4<f32>(_e15.g0_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) + ((vec4<f32>(_e26.g0_.w) * vec4<f32>(_e30.g0_.w)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e42.g1_.x) * vec4<f32>(_e46.g0_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e58.g1_.y) * vec4<f32>(_e62.g0_.x)) * vec4<f32>(0.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e74.g1_.z) * vec4<f32>(_e78.g0_.x)) * vec4<f32>(0.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e90.g1_.w) * vec4<f32>(_e94.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + (_e106.g0_.yxxx * _e109.g0_.yyzw)));
}

fn motor_translator_inner_product(self_1822: Motor, other_1512: Translator) -> Motor {
    var self_1823: Motor;
    var other_1513: Translator;

    self_1823 = self_1822;
    other_1513 = other_1512;
    let _e4: Motor = self_1823;
    let _e6: Translator = other_1513;
    let _e11: Motor = self_1823;
    let _e15: Translator = other_1513;
    let _e26: Motor = self_1823;
    let _e30: Translator = other_1513;
    let _e42: Motor = self_1823;
    let _e46: Translator = other_1513;
    let _e58: Motor = self_1823;
    let _e61: Motor = self_1823;
    let _e64: Motor = self_1823;
    let _e67: Motor = self_1823;
    let _e71: Translator = other_1513;
    return Motor((_e4.g0_ * vec4<f32>(_e6.g0_.x)), (((((vec4<f32>(_e11.g1_.y) * vec4<f32>(_e15.g0_.x)) * vec4<f32>(0.0, 1.0, 0.0, 0.0)) + ((vec4<f32>(_e26.g1_.z) * vec4<f32>(_e30.g0_.x)) * vec4<f32>(0.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e42.g1_.w) * vec4<f32>(_e46.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + (vec4<f32>(_e58.g1_.x, _e61.g0_.x, _e64.g0_.x, _e67.g0_.x) * _e71.g0_)));
}

fn motor_translator_geometric_anti_product(self_1824: Motor, other_1514: Translator) -> Motor {
    var self_1825: Motor;
    var other_1515: Translator;

    self_1825 = self_1824;
    other_1515 = other_1514;
    let _e4: Motor = self_1825;
    let _e8: Translator = other_1515;
    let _e19: Motor = self_1825;
    let _e23: Translator = other_1515;
    let _e35: Motor = self_1825;
    let _e39: Translator = other_1515;
    let _e51: Motor = self_1825;
    let _e55: Translator = other_1515;
    let _e68: Motor = self_1825;
    let _e72: Translator = other_1515;
    let _e85: Motor = self_1825;
    let _e89: Translator = other_1515;
    let _e102: Motor = self_1825;
    let _e105: Motor = self_1825;
    let _e108: Motor = self_1825;
    let _e111: Motor = self_1825;
    let _e115: Translator = other_1515;
    let _e128: Motor = self_1825;
    let _e132: Translator = other_1515;
    let _e144: Motor = self_1825;
    let _e148: Translator = other_1515;
    let _e161: Motor = self_1825;
    let _e165: Translator = other_1515;
    let _e178: Motor = self_1825;
    let _e182: Translator = other_1515;
    return Motor(((((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.yywz) * vec4<f32>(1.0, 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.zwzy) * vec4<f32>(1.0, -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e35.g0_.w) * _e39.g0_.wzyw) * vec4<f32>(1.0, 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e51.g1_.y) * vec4<f32>(_e55.g0_.x)) * vec4<f32>(0.0, -(1.0), 0.0, 0.0))) + ((vec4<f32>(_e68.g1_.z) * vec4<f32>(_e72.g0_.x)) * vec4<f32>(0.0, 0.0, -(1.0), 0.0))) + ((vec4<f32>(_e85.g1_.w) * vec4<f32>(_e89.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e102.g1_.x, _e105.g0_.x, _e108.g0_.x, _e111.g0_.x) * _e115.g0_) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0)))), (((((vec4<f32>(_e128.g1_.y) * _e132.g0_.yywz) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e144.g1_.z) * _e148.g0_.zwzy) * vec4<f32>(-(1.0), -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e161.g1_.w) * _e165.g0_.wzyw) * vec4<f32>(-(1.0), 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e178.g1_.x) * _e182.g0_) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn motor_translator_inner_anti_product(self_1826: Motor, other_1516: Translator) -> Motor {
    var self_1827: Motor;
    var other_1517: Translator;

    self_1827 = self_1826;
    other_1517 = other_1516;
    let _e4: Motor = self_1827;
    let _e8: Translator = other_1517;
    let _e20: Motor = self_1827;
    let _e24: Translator = other_1517;
    let _e37: Motor = self_1827;
    let _e41: Translator = other_1517;
    let _e54: Motor = self_1827;
    let _e57: Motor = self_1827;
    let _e60: Motor = self_1827;
    let _e63: Motor = self_1827;
    let _e67: Translator = other_1517;
    let _e80: Motor = self_1827;
    let _e84: Translator = other_1517;
    let _e96: Motor = self_1827;
    let _e100: Translator = other_1517;
    let _e113: Motor = self_1827;
    let _e116: Translator = other_1517;
    return Motor((((((vec4<f32>(_e4.g1_.y) * vec4<f32>(_e8.g0_.x)) * vec4<f32>(0.0, -(1.0), 0.0, 0.0)) + ((vec4<f32>(_e20.g1_.z) * vec4<f32>(_e24.g0_.x)) * vec4<f32>(0.0, 0.0, -(1.0), 0.0))) + ((vec4<f32>(_e37.g1_.w) * vec4<f32>(_e41.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e54.g1_.x, _e57.g0_.x, _e60.g0_.x, _e63.g0_.x) * _e67.g0_) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0)))), ((((vec4<f32>(_e80.g1_.z) * vec4<f32>(_e84.g0_.z)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0)) + ((vec4<f32>(_e96.g1_.w) * vec4<f32>(_e100.g0_.w)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((_e113.g1_.yxxx * _e116.g0_.yyzw) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0))));
}

fn motor_translator_left_contraction(self_1828: Motor, other_1518: Translator) -> Translator {
    var self_1829: Motor;
    var other_1519: Translator;

    self_1829 = self_1828;
    other_1519 = other_1518;
    let _e4: Motor = self_1829;
    let _e8: Translator = other_1519;
    return Translator((vec4<f32>(_e4.g0_.x) * _e8.g0_));
}

fn motor_translator_right_contraction(self_1830: Motor, other_1520: Translator) -> Motor {
    var self_1831: Motor;
    var other_1521: Translator;

    self_1831 = self_1830;
    other_1521 = other_1520;
    let _e4: Motor = self_1831;
    let _e6: Translator = other_1521;
    let _e11: Motor = self_1831;
    let _e13: Translator = other_1521;
    return Motor((_e4.g0_ * vec4<f32>(_e6.g0_.x)), (_e11.g1_ * vec4<f32>(_e13.g0_.x)));
}

fn motor_translator_left_anti_contraction(self_1832: Motor, other_1522: Translator) -> Motor {
    var self_1833: Motor;
    var other_1523: Translator;

    self_1833 = self_1832;
    other_1523 = other_1522;
    let _e4: Motor = self_1833;
    let _e6: Translator = other_1523;
    let _e20: Motor = self_1833;
    let _e24: Translator = other_1523;
    let _e36: Motor = self_1833;
    let _e40: Translator = other_1523;
    let _e53: Motor = self_1833;
    let _e56: Translator = other_1523;
    return Motor(((_e4.g1_ * vec4<f32>(_e6.g0_.x)) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))), ((((vec4<f32>(_e20.g1_.z) * vec4<f32>(_e24.g0_.z)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0)) + ((vec4<f32>(_e36.g1_.w) * vec4<f32>(_e40.g0_.w)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((_e53.g1_.yxxx * _e56.g0_.yyzw) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0))));
}

fn motor_translator_scalar_product(self_1834: Motor, other_1524: Translator) -> Scalar {
    var self_1835: Motor;
    var other_1525: Translator;

    self_1835 = self_1834;
    other_1525 = other_1524;
    let _e4: Motor = self_1835;
    let _e7: Translator = other_1525;
    return Scalar((_e4.g0_.x * _e7.g0_.x));
}

fn motor_translator_anti_scalar_product(self_1836: Motor, other_1526: Translator) -> AntiScalar {
    var self_1837: Motor;
    var other_1527: Translator;

    self_1837 = self_1836;
    other_1527 = other_1526;
    let _e5: Motor = self_1837;
    let _e8: Translator = other_1527;
    let _e13: Motor = self_1837;
    let _e16: Translator = other_1527;
    let _e21: Motor = self_1837;
    let _e24: Translator = other_1527;
    return AntiScalar((((0.0 - (_e5.g1_.y * _e8.g0_.y)) - (_e13.g1_.z * _e16.g0_.z)) - (_e21.g1_.w * _e24.g0_.w)));
}

fn motor_motor_add(self_1838: Motor, other_1528: Motor) -> Motor {
    var self_1839: Motor;
    var other_1529: Motor;

    self_1839 = self_1838;
    other_1529 = other_1528;
    let _e4: Motor = self_1839;
    let _e6: Motor = other_1529;
    let _e9: Motor = self_1839;
    let _e11: Motor = other_1529;
    return Motor((_e4.g0_ + _e6.g0_), (_e9.g1_ + _e11.g1_));
}

fn motor_motor_sub(self_1840: Motor, other_1530: Motor) -> Motor {
    var self_1841: Motor;
    var other_1531: Motor;

    self_1841 = self_1840;
    other_1531 = other_1530;
    let _e4: Motor = self_1841;
    let _e6: Motor = other_1531;
    let _e9: Motor = self_1841;
    let _e11: Motor = other_1531;
    return Motor((_e4.g0_ - _e6.g0_), (_e9.g1_ - _e11.g1_));
}

fn motor_motor_mul(self_1842: Motor, other_1532: Motor) -> Motor {
    var self_1843: Motor;
    var other_1533: Motor;

    self_1843 = self_1842;
    other_1533 = other_1532;
    let _e4: Motor = self_1843;
    let _e6: Motor = other_1533;
    let _e9: Motor = self_1843;
    let _e11: Motor = other_1533;
    return Motor((_e4.g0_ * _e6.g0_), (_e9.g1_ * _e11.g1_));
}

fn motor_motor_div(self_1844: Motor, other_1534: Motor) -> Motor {
    var self_1845: Motor;
    var other_1535: Motor;

    self_1845 = self_1844;
    other_1535 = other_1534;
    let _e4: Motor = self_1845;
    let _e7: Motor = self_1845;
    let _e10: Motor = self_1845;
    let _e13: Motor = self_1845;
    let _e23: Motor = other_1535;
    let _e26: Motor = other_1535;
    let _e29: Motor = other_1535;
    let _e32: Motor = other_1535;
    let _e43: Motor = self_1845;
    let _e46: Motor = self_1845;
    let _e49: Motor = self_1845;
    let _e52: Motor = self_1845;
    let _e62: Motor = other_1535;
    let _e65: Motor = other_1535;
    let _e68: Motor = other_1535;
    let _e71: Motor = other_1535;
    return Motor((((vec4<f32>(_e4.g0_.x, _e7.g0_.y, _e10.g0_.z, _e13.g0_.w) * vec4<f32>(1.0, 1.0, 1.0, 1.0)) / vec4<f32>(_e23.g0_.x, _e26.g0_.y, _e29.g0_.z, _e32.g0_.w)) * vec4<f32>(1.0, 1.0, 1.0, 1.0)), (((vec4<f32>(_e43.g1_.x, _e46.g1_.y, _e49.g1_.z, _e52.g1_.w) * vec4<f32>(1.0, 1.0, 1.0, 1.0)) / vec4<f32>(_e62.g1_.x, _e65.g1_.y, _e68.g1_.z, _e71.g1_.w)) * vec4<f32>(1.0, 1.0, 1.0, 1.0)));
}

fn motor_motor_geometric_product(self_1846: Motor, other_1536: Motor) -> Motor {
    var self_1847: Motor;
    var other_1537: Motor;

    self_1847 = self_1846;
    other_1537 = other_1536;
    let _e4: Motor = self_1847;
    let _e8: Motor = other_1537;
    let _e11: Motor = self_1847;
    let _e15: Motor = other_1537;
    let _e28: Motor = self_1847;
    let _e32: Motor = other_1537;
    let _e45: Motor = self_1847;
    let _e49: Motor = other_1537;
    let _e62: Motor = self_1847;
    let _e66: Motor = other_1537;
    let _e69: Motor = self_1847;
    let _e73: Motor = other_1537;
    let _e86: Motor = self_1847;
    let _e90: Motor = other_1537;
    let _e103: Motor = self_1847;
    let _e107: Motor = other_1537;
    let _e120: Motor = self_1847;
    let _e124: Motor = other_1537;
    let _e137: Motor = self_1847;
    let _e141: Motor = other_1537;
    let _e153: Motor = self_1847;
    let _e157: Motor = other_1537;
    let _e169: Motor = self_1847;
    let _e173: Motor = other_1537;
    return Motor(((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.y) * _e15.g0_.yxwz) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e28.g0_.z) * _e32.g0_.zwxy) * vec4<f32>(-(1.0), -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e45.g0_.w) * _e49.g0_.wzyx) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))), ((((((((vec4<f32>(_e62.g0_.x) * _e66.g1_) + ((vec4<f32>(_e69.g0_.y) * _e73.g1_.yxwz) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e86.g0_.z) * _e90.g1_.zwxy) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e103.g0_.w) * _e107.g1_.wzyx) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e120.g1_.x) * _e124.g0_) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0)))) + ((vec4<f32>(_e137.g1_.y) * _e141.g0_.yxwz) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e153.g1_.z) * _e157.g0_.zwxy) * vec4<f32>(1.0, -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e169.g1_.w) * _e173.g0_.wzyx) * vec4<f32>(1.0, 1.0, -(1.0), 1.0))));
}

fn motor_motor_regressive_product(self_1848: Motor, other_1538: Motor) -> Motor {
    var self_1849: Motor;
    var other_1539: Motor;

    self_1849 = self_1848;
    other_1539 = other_1538;
    let _e4: Motor = self_1849;
    let _e8: Motor = other_1539;
    let _e18: Motor = self_1849;
    let _e22: Motor = other_1539;
    let _e33: Motor = self_1849;
    let _e37: Motor = other_1539;
    let _e48: Motor = self_1849;
    let _e52: Motor = other_1539;
    let _e56: Motor = self_1849;
    let _e60: Motor = other_1539;
    let _e72: Motor = self_1849;
    let _e76: Motor = other_1539;
    let _e88: Motor = self_1849;
    let _e92: Motor = other_1539;
    let _e104: Motor = self_1849;
    let _e108: Motor = other_1539;
    let _e120: Motor = self_1849;
    let _e124: Motor = other_1539;
    let _e127: Motor = self_1849;
    let _e129: Motor = other_1539;
    return Motor((((((((((vec4<f32>(_e4.g0_.y) * _e8.g1_.yxyy) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) + ((vec4<f32>(_e18.g0_.z) * _e22.g1_.zzxz) * vec4<f32>(1.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e33.g0_.w) * _e37.g1_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, 1.0))) + (vec4<f32>(_e48.g1_.x) * _e52.g0_)) + ((vec4<f32>(_e56.g1_.y) * vec4<f32>(_e60.g0_.y)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e72.g1_.z) * vec4<f32>(_e76.g0_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e88.g1_.w) * vec4<f32>(_e92.g0_.w)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e104.g0_.x) * vec4<f32>(_e108.g1_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))), ((vec4<f32>(_e120.g1_.x) * _e124.g1_) + ((_e127.g1_ * vec4<f32>(_e129.g1_.x)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn motor_motor_outer_product(self_1850: Motor, other_1540: Motor) -> Motor {
    var self_1851: Motor;
    var other_1541: Motor;

    self_1851 = self_1850;
    other_1541 = other_1540;
    let _e4: Motor = self_1851;
    let _e8: Motor = other_1541;
    let _e11: Motor = self_1851;
    let _e13: Motor = other_1541;
    let _e25: Motor = self_1851;
    let _e29: Motor = other_1541;
    let _e32: Motor = self_1851;
    let _e36: Motor = other_1541;
    let _e48: Motor = self_1851;
    let _e52: Motor = other_1541;
    let _e64: Motor = self_1851;
    let _e68: Motor = other_1541;
    let _e80: Motor = self_1851;
    let _e84: Motor = other_1541;
    let _e95: Motor = self_1851;
    let _e99: Motor = other_1541;
    let _e110: Motor = self_1851;
    let _e114: Motor = other_1541;
    let _e125: Motor = self_1851;
    let _e128: Motor = other_1541;
    return Motor(((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((_e11.g0_ * vec4<f32>(_e13.g0_.x)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))), ((((((((vec4<f32>(_e25.g0_.x) * _e29.g1_) + ((vec4<f32>(_e32.g0_.z) * vec4<f32>(_e36.g1_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e48.g0_.w) * vec4<f32>(_e52.g1_.w)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e64.g1_.x) * vec4<f32>(_e68.g0_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e80.g1_.y) * _e84.g0_.yxyy) * vec4<f32>(1.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e95.g1_.z) * _e99.g0_.zzxz) * vec4<f32>(1.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e110.g1_.w) * _e114.g0_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, 1.0))) + ((_e125.g0_.yxxx * _e128.g1_.yxxx) * vec4<f32>(1.0, 0.0, 0.0, 0.0))));
}

fn motor_motor_inner_product(self_1852: Motor, other_1542: Motor) -> Motor {
    var self_1853: Motor;
    var other_1543: Motor;

    self_1853 = self_1852;
    other_1543 = other_1542;
    let _e4: Motor = self_1853;
    let _e8: Motor = other_1543;
    let _e11: Motor = self_1853;
    let _e15: Motor = other_1543;
    let _e27: Motor = self_1853;
    let _e31: Motor = other_1543;
    let _e43: Motor = self_1853;
    let _e46: Motor = other_1543;
    let _e58: Motor = self_1853;
    let _e62: Motor = other_1543;
    let _e65: Motor = self_1853;
    let _e69: Motor = other_1543;
    let _e82: Motor = self_1853;
    let _e86: Motor = other_1543;
    let _e98: Motor = self_1853;
    let _e102: Motor = other_1543;
    let _e114: Motor = self_1853;
    let _e118: Motor = other_1543;
    let _e130: Motor = self_1853;
    let _e132: Motor = other_1543;
    return Motor(((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.z) * _e15.g0_.zzxz) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0))) + ((vec4<f32>(_e27.g0_.w) * _e31.g0_.wwwx) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))) + ((_e43.g0_.yyxx * _e46.g0_.yxxx) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))), ((((((vec4<f32>(_e58.g0_.x) * _e62.g1_) + ((vec4<f32>(_e65.g1_.x) * _e69.g0_) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0)))) + ((vec4<f32>(_e82.g1_.y) * vec4<f32>(_e86.g0_.x)) * vec4<f32>(0.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e98.g1_.z) * vec4<f32>(_e102.g0_.x)) * vec4<f32>(0.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e114.g1_.w) * vec4<f32>(_e118.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((_e130.g0_ * vec4<f32>(_e132.g1_.x)) * vec4<f32>(0.0, -(1.0), -(1.0), -(1.0)))));
}

fn motor_motor_geometric_anti_product(self_1854: Motor, other_1544: Motor) -> Motor {
    var self_1855: Motor;
    var other_1545: Motor;

    self_1855 = self_1854;
    other_1545 = other_1544;
    let _e4: Motor = self_1855;
    let _e8: Motor = other_1545;
    let _e20: Motor = self_1855;
    let _e24: Motor = other_1545;
    let _e36: Motor = self_1855;
    let _e40: Motor = other_1545;
    let _e52: Motor = self_1855;
    let _e56: Motor = other_1545;
    let _e68: Motor = self_1855;
    let _e72: Motor = other_1545;
    let _e76: Motor = self_1855;
    let _e80: Motor = other_1545;
    let _e93: Motor = self_1855;
    let _e97: Motor = other_1545;
    let _e110: Motor = self_1855;
    let _e114: Motor = other_1545;
    let _e127: Motor = self_1855;
    let _e131: Motor = other_1545;
    let _e134: Motor = self_1855;
    let _e138: Motor = other_1545;
    let _e151: Motor = self_1855;
    let _e155: Motor = other_1545;
    let _e168: Motor = self_1855;
    let _e172: Motor = other_1545;
    return Motor((((((((((vec4<f32>(_e4.g0_.x) * _e8.g1_) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))) + ((vec4<f32>(_e20.g0_.y) * _e24.g1_.yxwz) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e36.g0_.z) * _e40.g1_.zwxy) * vec4<f32>(1.0, -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e52.g0_.w) * _e56.g1_.wzyx) * vec4<f32>(1.0, 1.0, -(1.0), 1.0))) + (vec4<f32>(_e68.g1_.x) * _e72.g0_)) + ((vec4<f32>(_e76.g1_.y) * _e80.g0_.yxwz) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e93.g1_.z) * _e97.g0_.zwxy) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e110.g1_.w) * _e114.g0_.wzyx) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))), ((((vec4<f32>(_e127.g1_.x) * _e131.g1_) + ((vec4<f32>(_e134.g1_.y) * _e138.g1_.yxwz) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e151.g1_.z) * _e155.g1_.zwxy) * vec4<f32>(-(1.0), -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e168.g1_.w) * _e172.g1_.wzyx) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))));
}

fn motor_motor_inner_anti_product(self_1856: Motor, other_1546: Motor) -> Motor {
    var self_1857: Motor;
    var other_1547: Motor;

    self_1857 = self_1856;
    other_1547 = other_1546;
    let _e4: Motor = self_1857;
    let _e8: Motor = other_1547;
    let _e20: Motor = self_1857;
    let _e24: Motor = other_1547;
    let _e28: Motor = self_1857;
    let _e32: Motor = other_1547;
    let _e45: Motor = self_1857;
    let _e49: Motor = other_1547;
    let _e62: Motor = self_1857;
    let _e66: Motor = other_1547;
    let _e79: Motor = self_1857;
    let _e81: Motor = other_1547;
    let _e93: Motor = self_1857;
    let _e97: Motor = other_1547;
    let _e100: Motor = self_1857;
    let _e104: Motor = other_1547;
    let _e116: Motor = self_1857;
    let _e120: Motor = other_1547;
    let _e132: Motor = self_1857;
    let _e135: Motor = other_1547;
    return Motor((((((((vec4<f32>(_e4.g0_.x) * _e8.g1_) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))) + (vec4<f32>(_e20.g1_.x) * _e24.g0_)) + ((vec4<f32>(_e28.g1_.y) * vec4<f32>(_e32.g0_.x)) * vec4<f32>(0.0, -(1.0), 0.0, 0.0))) + ((vec4<f32>(_e45.g1_.z) * vec4<f32>(_e49.g0_.x)) * vec4<f32>(0.0, 0.0, -(1.0), 0.0))) + ((vec4<f32>(_e62.g1_.w) * vec4<f32>(_e66.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((_e79.g0_ * vec4<f32>(_e81.g1_.x)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))), ((((vec4<f32>(_e93.g1_.x) * _e97.g1_) + ((vec4<f32>(_e100.g1_.z) * _e104.g1_.zzxz) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0))) + ((vec4<f32>(_e116.g1_.w) * _e120.g1_.wwwx) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))) + ((_e132.g1_.yyxx * _e135.g1_.yxxx) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))));
}

fn motor_motor_left_contraction(self_1858: Motor, other_1548: Motor) -> Motor {
    var self_1859: Motor;
    var other_1549: Motor;

    self_1859 = self_1858;
    other_1549 = other_1548;
    let _e4: Motor = self_1859;
    let _e8: Motor = other_1549;
    let _e11: Motor = self_1859;
    let _e15: Motor = other_1549;
    let _e28: Motor = self_1859;
    let _e32: Motor = other_1549;
    let _e45: Motor = self_1859;
    let _e48: Motor = other_1549;
    let _e60: Motor = self_1859;
    let _e64: Motor = other_1549;
    let _e67: Motor = self_1859;
    let _e69: Motor = other_1549;
    return Motor(((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.z) * vec4<f32>(_e15.g0_.z)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((vec4<f32>(_e28.g0_.w) * vec4<f32>(_e32.g0_.w)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((_e45.g0_.yxxx * _e48.g0_.yxxx) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))), ((vec4<f32>(_e60.g0_.x) * _e64.g1_) + ((_e67.g0_ * vec4<f32>(_e69.g1_.x)) * vec4<f32>(0.0, -(1.0), -(1.0), -(1.0)))));
}

fn motor_motor_right_contraction(self_1860: Motor, other_1550: Motor) -> Motor {
    var self_1861: Motor;
    var other_1551: Motor;

    self_1861 = self_1860;
    other_1551 = other_1550;
    let _e4: Motor = self_1861;
    let _e8: Motor = other_1551;
    let _e19: Motor = self_1861;
    let _e23: Motor = other_1551;
    let _e35: Motor = self_1861;
    let _e39: Motor = other_1551;
    let _e51: Motor = self_1861;
    let _e55: Motor = other_1551;
    let _e67: Motor = self_1861;
    let _e71: Motor = other_1551;
    let _e83: Motor = self_1861;
    let _e85: Motor = other_1551;
    return Motor((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0)) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.zzxz) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0))) + ((vec4<f32>(_e35.g0_.w) * _e39.g0_.wwwx) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))) + ((vec4<f32>(_e51.g0_.x) * vec4<f32>(_e55.g0_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))), (((vec4<f32>(_e67.g1_.x) * _e71.g0_) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))) + ((_e83.g1_ * vec4<f32>(_e85.g0_.x)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn motor_motor_left_anti_contraction(self_1862: Motor, other_1552: Motor) -> Motor {
    var self_1863: Motor;
    var other_1553: Motor;

    self_1863 = self_1862;
    other_1553 = other_1552;
    let _e4: Motor = self_1863;
    let _e8: Motor = other_1553;
    let _e11: Motor = self_1863;
    let _e13: Motor = other_1553;
    let _e28: Motor = self_1863;
    let _e32: Motor = other_1553;
    let _e35: Motor = self_1863;
    let _e39: Motor = other_1553;
    let _e52: Motor = self_1863;
    let _e56: Motor = other_1553;
    let _e69: Motor = self_1863;
    let _e72: Motor = other_1553;
    return Motor(((vec4<f32>(_e4.g1_.x) * _e8.g0_) + ((_e11.g1_ * vec4<f32>(_e13.g0_.x)) * vec4<f32>(0.0, -(1.0), -(1.0), -(1.0)))), ((((vec4<f32>(_e28.g1_.x) * _e32.g1_) + ((vec4<f32>(_e35.g1_.z) * vec4<f32>(_e39.g1_.z)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((vec4<f32>(_e52.g1_.w) * vec4<f32>(_e56.g1_.w)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((_e69.g1_.yxxx * _e72.g1_.yxxx) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))));
}

fn motor_motor_right_anti_contraction(self_1864: Motor, other_1554: Motor) -> Motor {
    var self_1865: Motor;
    var other_1555: Motor;

    self_1865 = self_1864;
    other_1555 = other_1554;
    let _e4: Motor = self_1865;
    let _e8: Motor = other_1555;
    let _e20: Motor = self_1865;
    let _e22: Motor = other_1555;
    let _e34: Motor = self_1865;
    let _e38: Motor = other_1555;
    let _e49: Motor = self_1865;
    let _e53: Motor = other_1555;
    let _e65: Motor = self_1865;
    let _e69: Motor = other_1555;
    let _e81: Motor = self_1865;
    let _e85: Motor = other_1555;
    return Motor((((vec4<f32>(_e4.g0_.x) * _e8.g1_) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))) + ((_e20.g0_ * vec4<f32>(_e22.g1_.x)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))), (((((vec4<f32>(_e34.g1_.y) * _e38.g1_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0)) + ((vec4<f32>(_e49.g1_.z) * _e53.g1_.zzxz) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0))) + ((vec4<f32>(_e65.g1_.w) * _e69.g1_.wwwx) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))) + ((vec4<f32>(_e81.g1_.x) * vec4<f32>(_e85.g1_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))));
}

fn motor_motor_scalar_product(self_1866: Motor, other_1556: Motor) -> Scalar {
    var self_1867: Motor;
    var other_1557: Motor;

    self_1867 = self_1866;
    other_1557 = other_1556;
    let _e4: Motor = self_1867;
    let _e7: Motor = other_1557;
    let _e11: Motor = self_1867;
    let _e14: Motor = other_1557;
    let _e19: Motor = self_1867;
    let _e22: Motor = other_1557;
    let _e27: Motor = self_1867;
    let _e30: Motor = other_1557;
    return Scalar(((((_e4.g0_.x * _e7.g0_.x) - (_e11.g0_.y * _e14.g0_.y)) - (_e19.g0_.z * _e22.g0_.z)) - (_e27.g0_.w * _e30.g0_.w)));
}

fn motor_motor_anti_scalar_product(self_1868: Motor, other_1558: Motor) -> AntiScalar {
    var self_1869: Motor;
    var other_1559: Motor;

    self_1869 = self_1868;
    other_1559 = other_1558;
    let _e4: Motor = self_1869;
    let _e7: Motor = other_1559;
    let _e11: Motor = self_1869;
    let _e14: Motor = other_1559;
    let _e19: Motor = self_1869;
    let _e22: Motor = other_1559;
    let _e27: Motor = self_1869;
    let _e30: Motor = other_1559;
    return AntiScalar(((((_e4.g1_.x * _e7.g1_.x) - (_e11.g1_.y * _e14.g1_.y)) - (_e19.g1_.z * _e22.g1_.z)) - (_e27.g1_.w * _e30.g1_.w)));
}

fn motor_point_and_plane_add(self_1870: Motor, other_1560: PointAndPlane) -> MultiVector {
    var self_1871: Motor;
    var other_1561: PointAndPlane;

    self_1871 = self_1870;
    other_1561 = other_1560;
    let _e4: Motor = self_1871;
    let _e6: PointAndPlane = other_1561;
    let _e9: PointAndPlane = other_1561;
    let _e12: PointAndPlane = other_1561;
    let _e15: PointAndPlane = other_1561;
    let _e19: PointAndPlane = other_1561;
    let _e22: PointAndPlane = other_1561;
    let _e25: PointAndPlane = other_1561;
    let _e28: PointAndPlane = other_1561;
    let _e32: Motor = self_1871;
    return MultiVector(_e4.g0_, vec4<f32>(_e6.g1_.x, _e9.g0_.y, _e12.g0_.z, _e15.g0_.w), vec4<f32>(_e19.g0_.x, _e22.g1_.y, _e25.g1_.z, _e28.g1_.w), _e32.g1_);
}

fn motor_point_and_plane_sub(self_1872: Motor, other_1562: PointAndPlane) -> MultiVector {
    var self_1873: Motor;
    var other_1563: PointAndPlane;

    self_1873 = self_1872;
    other_1563 = other_1562;
    let _e4: Motor = self_1873;
    let _e8: PointAndPlane = other_1563;
    let _e11: PointAndPlane = other_1563;
    let _e14: PointAndPlane = other_1563;
    let _e17: PointAndPlane = other_1563;
    let _e24: PointAndPlane = other_1563;
    let _e27: PointAndPlane = other_1563;
    let _e30: PointAndPlane = other_1563;
    let _e33: PointAndPlane = other_1563;
    let _e38: Motor = self_1873;
    return MultiVector(_e4.g0_, (vec4<f32>(0.0) - vec4<f32>(_e8.g1_.x, _e11.g0_.y, _e14.g0_.z, _e17.g0_.w)), (vec4<f32>(0.0) - vec4<f32>(_e24.g0_.x, _e27.g1_.y, _e30.g1_.z, _e33.g1_.w)), _e38.g1_);
}

fn motor_point_and_plane_geometric_product(self_1874: Motor, other_1564: PointAndPlane) -> PointAndPlane {
    var self_1875: Motor;
    var other_1565: PointAndPlane;

    self_1875 = self_1874;
    other_1565 = other_1564;
    let _e4: Motor = self_1875;
    let _e8: PointAndPlane = other_1565;
    let _e11: Motor = self_1875;
    let _e15: PointAndPlane = other_1565;
    let _e18: PointAndPlane = other_1565;
    let _e21: PointAndPlane = other_1565;
    let _e24: PointAndPlane = other_1565;
    let _e38: Motor = self_1875;
    let _e42: PointAndPlane = other_1565;
    let _e45: PointAndPlane = other_1565;
    let _e48: PointAndPlane = other_1565;
    let _e51: PointAndPlane = other_1565;
    let _e65: Motor = self_1875;
    let _e69: PointAndPlane = other_1565;
    let _e72: PointAndPlane = other_1565;
    let _e75: PointAndPlane = other_1565;
    let _e78: PointAndPlane = other_1565;
    let _e92: Motor = self_1875;
    let _e96: PointAndPlane = other_1565;
    let _e99: PointAndPlane = other_1565;
    let _e102: PointAndPlane = other_1565;
    let _e105: PointAndPlane = other_1565;
    let _e119: Motor = self_1875;
    let _e123: PointAndPlane = other_1565;
    let _e126: PointAndPlane = other_1565;
    let _e129: PointAndPlane = other_1565;
    let _e132: PointAndPlane = other_1565;
    let _e146: Motor = self_1875;
    let _e150: PointAndPlane = other_1565;
    let _e153: PointAndPlane = other_1565;
    let _e156: PointAndPlane = other_1565;
    let _e159: PointAndPlane = other_1565;
    let _e173: Motor = self_1875;
    let _e177: PointAndPlane = other_1565;
    let _e190: Motor = self_1875;
    let _e194: PointAndPlane = other_1565;
    let _e197: Motor = self_1875;
    let _e201: PointAndPlane = other_1565;
    let _e204: PointAndPlane = other_1565;
    let _e207: PointAndPlane = other_1565;
    let _e210: PointAndPlane = other_1565;
    let _e224: Motor = self_1875;
    let _e228: PointAndPlane = other_1565;
    let _e231: PointAndPlane = other_1565;
    let _e234: PointAndPlane = other_1565;
    let _e237: PointAndPlane = other_1565;
    let _e251: Motor = self_1875;
    let _e255: PointAndPlane = other_1565;
    let _e258: PointAndPlane = other_1565;
    let _e261: PointAndPlane = other_1565;
    let _e264: PointAndPlane = other_1565;
    let _e278: Motor = self_1875;
    let _e282: PointAndPlane = other_1565;
    let _e294: Motor = self_1875;
    let _e298: PointAndPlane = other_1565;
    let _e310: Motor = self_1875;
    let _e314: PointAndPlane = other_1565;
    let _e326: Motor = self_1875;
    let _e330: PointAndPlane = other_1565;
    return PointAndPlane(((((((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.y) * vec4<f32>(_e15.g1_.y, _e18.g1_.x, _e21.g0_.w, _e24.g0_.z)) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e38.g0_.z) * vec4<f32>(_e42.g1_.z, _e45.g0_.w, _e48.g1_.x, _e51.g0_.y)) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e65.g0_.w) * vec4<f32>(_e69.g1_.w, _e72.g0_.z, _e75.g0_.y, _e78.g1_.x)) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e92.g1_.y) * vec4<f32>(_e96.g0_.x, _e99.g0_.x, _e102.g1_.w, _e105.g1_.z)) * vec4<f32>(0.0, -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e119.g1_.z) * vec4<f32>(_e123.g1_.w, _e126.g1_.w, _e129.g0_.x, _e132.g1_.y)) * vec4<f32>(0.0, -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e146.g1_.w) * vec4<f32>(_e150.g1_.z, _e153.g1_.z, _e156.g1_.y, _e159.g0_.x)) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e173.g1_.x) * _e177.g1_) * vec4<f32>(0.0, -(1.0), -(1.0), -(1.0)))), ((((((((vec4<f32>(_e190.g0_.x) * _e194.g1_) + ((vec4<f32>(_e197.g0_.y) * vec4<f32>(_e201.g0_.y, _e204.g0_.x, _e207.g1_.w, _e210.g1_.z)) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e224.g0_.z) * vec4<f32>(_e228.g0_.z, _e231.g1_.w, _e234.g0_.x, _e237.g1_.y)) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e251.g0_.w) * vec4<f32>(_e255.g0_.w, _e258.g1_.z, _e261.g1_.y, _e264.g0_.x)) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e278.g1_.y) * vec4<f32>(_e282.g1_.y)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e294.g1_.z) * vec4<f32>(_e298.g1_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e310.g1_.w) * vec4<f32>(_e314.g1_.w)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e326.g1_.x) * vec4<f32>(_e330.g0_.x)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))));
}

fn motor_point_and_plane_regressive_product(self_1876: Motor, other_1566: PointAndPlane) -> PointAndPlane {
    var self_1877: Motor;
    var other_1567: PointAndPlane;

    self_1877 = self_1876;
    other_1567 = other_1566;
    let _e4: Motor = self_1877;
    let _e8: PointAndPlane = other_1567;
    let _e11: Motor = self_1877;
    let _e15: PointAndPlane = other_1567;
    let _e26: Motor = self_1877;
    let _e30: PointAndPlane = other_1567;
    let _e42: Motor = self_1877;
    let _e46: PointAndPlane = other_1567;
    let _e50: Motor = self_1877;
    let _e54: PointAndPlane = other_1567;
    let _e66: Motor = self_1877;
    let _e70: PointAndPlane = other_1567;
    let _e82: Motor = self_1877;
    let _e85: Motor = self_1877;
    let _e88: Motor = self_1877;
    let _e91: Motor = self_1877;
    let _e95: PointAndPlane = other_1567;
    return PointAndPlane((vec4<f32>(_e4.g1_.x) * _e8.g0_), (((((((vec4<f32>(_e11.g0_.z) * _e15.g0_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e26.g0_.w) * _e30.g0_.zzyz) * vec4<f32>(0.0, -(1.0), 1.0, 0.0))) + (vec4<f32>(_e42.g1_.x) * _e46.g1_)) + ((vec4<f32>(_e50.g1_.z) * _e54.g0_.zzxz) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0))) + ((vec4<f32>(_e66.g1_.w) * _e70.g0_.wwwx) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))) + ((vec4<f32>(_e82.g1_.y, _e85.g1_.y, _e88.g0_.y, _e91.g0_.y) * _e95.g0_.yxwz) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))));
}

fn motor_point_and_plane_outer_product(self_1878: Motor, other_1568: PointAndPlane) -> PointAndPlane {
    var self_1879: Motor;
    var other_1569: PointAndPlane;

    self_1879 = self_1878;
    other_1569 = other_1568;
    let _e4: Motor = self_1879;
    let _e8: PointAndPlane = other_1569;
    let _e11: Motor = self_1879;
    let _e15: PointAndPlane = other_1569;
    let _e27: Motor = self_1879;
    let _e31: PointAndPlane = other_1569;
    let _e43: Motor = self_1879;
    let _e47: PointAndPlane = other_1569;
    let _e59: Motor = self_1879;
    let _e63: PointAndPlane = other_1569;
    let _e75: Motor = self_1879;
    let _e78: Motor = self_1879;
    let _e81: Motor = self_1879;
    let _e84: Motor = self_1879;
    let _e88: PointAndPlane = other_1569;
    let _e101: Motor = self_1879;
    let _e105: PointAndPlane = other_1569;
    return PointAndPlane(((((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.z) * _e15.g1_.zzxz) * vec4<f32>(1.0, 0.0, -(1.0), 0.0))) + ((vec4<f32>(_e27.g0_.w) * _e31.g1_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e43.g1_.z) * _e47.g1_.wwwy) * vec4<f32>(0.0, -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e59.g1_.w) * _e63.g1_.zzyz) * vec4<f32>(0.0, 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e75.g0_.y, _e78.g0_.y, _e81.g1_.y, _e84.g1_.y) * _e88.g1_.yxwz) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))), (vec4<f32>(_e101.g0_.x) * _e105.g1_));
}

fn motor_point_and_plane_inner_product(self_1880: Motor, other_1570: PointAndPlane) -> PointAndPlane {
    var self_1881: Motor;
    var other_1571: PointAndPlane;

    self_1881 = self_1880;
    other_1571 = other_1570;
    let _e4: Motor = self_1881;
    let _e8: PointAndPlane = other_1571;
    let _e11: Motor = self_1881;
    let _e15: PointAndPlane = other_1571;
    let _e28: Motor = self_1881;
    let _e32: PointAndPlane = other_1571;
    let _e35: Motor = self_1881;
    let _e39: PointAndPlane = other_1571;
    let _e42: PointAndPlane = other_1571;
    let _e45: PointAndPlane = other_1571;
    let _e48: PointAndPlane = other_1571;
    let _e62: Motor = self_1881;
    let _e66: PointAndPlane = other_1571;
    let _e69: PointAndPlane = other_1571;
    let _e72: PointAndPlane = other_1571;
    let _e75: PointAndPlane = other_1571;
    let _e89: Motor = self_1881;
    let _e93: PointAndPlane = other_1571;
    let _e96: PointAndPlane = other_1571;
    let _e99: PointAndPlane = other_1571;
    let _e102: PointAndPlane = other_1571;
    let _e116: Motor = self_1881;
    let _e120: PointAndPlane = other_1571;
    let _e132: Motor = self_1881;
    let _e136: PointAndPlane = other_1571;
    let _e148: Motor = self_1881;
    let _e152: PointAndPlane = other_1571;
    let _e164: Motor = self_1881;
    let _e168: PointAndPlane = other_1571;
    return PointAndPlane(((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g1_.x) * _e15.g1_) * vec4<f32>(0.0, -(1.0), -(1.0), -(1.0)))), ((((((((vec4<f32>(_e28.g0_.x) * _e32.g1_) + ((vec4<f32>(_e35.g0_.y) * vec4<f32>(_e39.g0_.y, _e42.g0_.x, _e45.g1_.w, _e48.g1_.z)) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e62.g0_.z) * vec4<f32>(_e66.g0_.z, _e69.g1_.w, _e72.g0_.x, _e75.g1_.y)) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e89.g0_.w) * vec4<f32>(_e93.g0_.w, _e96.g1_.z, _e99.g1_.y, _e102.g0_.x)) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e116.g1_.y) * vec4<f32>(_e120.g1_.y)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e132.g1_.z) * vec4<f32>(_e136.g1_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e148.g1_.w) * vec4<f32>(_e152.g1_.w)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e164.g1_.x) * vec4<f32>(_e168.g0_.x)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))));
}

fn motor_point_and_plane_geometric_anti_product(self_1882: Motor, other_1572: PointAndPlane) -> PointAndPlane {
    var self_1883: Motor;
    var other_1573: PointAndPlane;

    self_1883 = self_1882;
    other_1573 = other_1572;
    let _e4: Motor = self_1883;
    let _e8: PointAndPlane = other_1573;
    let _e19: Motor = self_1883;
    let _e23: PointAndPlane = other_1573;
    let _e35: Motor = self_1883;
    let _e39: PointAndPlane = other_1573;
    let _e51: Motor = self_1883;
    let _e55: PointAndPlane = other_1573;
    let _e59: Motor = self_1883;
    let _e63: PointAndPlane = other_1573;
    let _e66: PointAndPlane = other_1573;
    let _e69: PointAndPlane = other_1573;
    let _e72: PointAndPlane = other_1573;
    let _e86: Motor = self_1883;
    let _e90: PointAndPlane = other_1573;
    let _e93: PointAndPlane = other_1573;
    let _e96: PointAndPlane = other_1573;
    let _e99: PointAndPlane = other_1573;
    let _e113: Motor = self_1883;
    let _e117: PointAndPlane = other_1573;
    let _e120: PointAndPlane = other_1573;
    let _e123: PointAndPlane = other_1573;
    let _e126: PointAndPlane = other_1573;
    let _e140: Motor = self_1883;
    let _e144: PointAndPlane = other_1573;
    let _e156: Motor = self_1883;
    let _e160: PointAndPlane = other_1573;
    let _e163: PointAndPlane = other_1573;
    let _e166: PointAndPlane = other_1573;
    let _e169: PointAndPlane = other_1573;
    let _e182: Motor = self_1883;
    let _e186: PointAndPlane = other_1573;
    let _e189: PointAndPlane = other_1573;
    let _e192: PointAndPlane = other_1573;
    let _e195: PointAndPlane = other_1573;
    let _e209: Motor = self_1883;
    let _e213: PointAndPlane = other_1573;
    let _e216: PointAndPlane = other_1573;
    let _e219: PointAndPlane = other_1573;
    let _e222: PointAndPlane = other_1573;
    let _e236: Motor = self_1883;
    let _e240: PointAndPlane = other_1573;
    let _e244: Motor = self_1883;
    let _e248: PointAndPlane = other_1573;
    let _e251: PointAndPlane = other_1573;
    let _e254: PointAndPlane = other_1573;
    let _e257: PointAndPlane = other_1573;
    let _e271: Motor = self_1883;
    let _e275: PointAndPlane = other_1573;
    let _e278: PointAndPlane = other_1573;
    let _e281: PointAndPlane = other_1573;
    let _e284: PointAndPlane = other_1573;
    let _e298: Motor = self_1883;
    let _e302: PointAndPlane = other_1573;
    let _e305: PointAndPlane = other_1573;
    let _e308: PointAndPlane = other_1573;
    let _e311: PointAndPlane = other_1573;
    let _e325: Motor = self_1883;
    let _e329: PointAndPlane = other_1573;
    return PointAndPlane((((((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y)) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) + ((vec4<f32>(_e19.g0_.z) * vec4<f32>(_e23.g0_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e35.g0_.w) * vec4<f32>(_e39.g0_.w)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + (vec4<f32>(_e51.g1_.x) * _e55.g0_)) + ((vec4<f32>(_e59.g1_.y) * vec4<f32>(_e63.g1_.y, _e66.g1_.x, _e69.g0_.w, _e72.g0_.z)) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e86.g1_.z) * vec4<f32>(_e90.g1_.z, _e93.g0_.w, _e96.g1_.x, _e99.g0_.y)) * vec4<f32>(-(1.0), -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e113.g1_.w) * vec4<f32>(_e117.g1_.w, _e120.g0_.z, _e123.g0_.y, _e126.g1_.x)) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))) + ((vec4<f32>(_e140.g0_.x) * vec4<f32>(_e144.g1_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))), (((((((((vec4<f32>(_e156.g0_.y) * vec4<f32>(_e160.g1_.x, _e163.g1_.x, _e166.g0_.w, _e169.g0_.z)) * vec4<f32>(0.0, -(1.0), -(1.0), 1.0)) + ((vec4<f32>(_e182.g0_.z) * vec4<f32>(_e186.g0_.w, _e189.g0_.w, _e192.g1_.x, _e195.g0_.y)) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e209.g0_.w) * vec4<f32>(_e213.g0_.z, _e216.g0_.z, _e219.g0_.y, _e222.g1_.x)) * vec4<f32>(0.0, -(1.0), 1.0, -(1.0)))) + (vec4<f32>(_e236.g1_.x) * _e240.g1_)) + ((vec4<f32>(_e244.g1_.y) * vec4<f32>(_e248.g0_.y, _e251.g0_.x, _e254.g1_.w, _e257.g1_.z)) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e271.g1_.z) * vec4<f32>(_e275.g0_.z, _e278.g1_.w, _e281.g0_.x, _e284.g1_.y)) * vec4<f32>(-(1.0), -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e298.g1_.w) * vec4<f32>(_e302.g0_.w, _e305.g1_.z, _e308.g1_.y, _e311.g0_.x)) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))) + ((vec4<f32>(_e325.g0_.x) * _e329.g0_) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn motor_point_and_plane_inner_anti_product(self_1884: Motor, other_1574: PointAndPlane) -> PointAndPlane {
    var self_1885: Motor;
    var other_1575: PointAndPlane;

    self_1885 = self_1884;
    other_1575 = other_1574;
    let _e4: Motor = self_1885;
    let _e8: PointAndPlane = other_1575;
    let _e19: Motor = self_1885;
    let _e23: PointAndPlane = other_1575;
    let _e35: Motor = self_1885;
    let _e39: PointAndPlane = other_1575;
    let _e51: Motor = self_1885;
    let _e55: PointAndPlane = other_1575;
    let _e59: Motor = self_1885;
    let _e63: PointAndPlane = other_1575;
    let _e66: PointAndPlane = other_1575;
    let _e69: PointAndPlane = other_1575;
    let _e72: PointAndPlane = other_1575;
    let _e86: Motor = self_1885;
    let _e90: PointAndPlane = other_1575;
    let _e93: PointAndPlane = other_1575;
    let _e96: PointAndPlane = other_1575;
    let _e99: PointAndPlane = other_1575;
    let _e113: Motor = self_1885;
    let _e117: PointAndPlane = other_1575;
    let _e120: PointAndPlane = other_1575;
    let _e123: PointAndPlane = other_1575;
    let _e126: PointAndPlane = other_1575;
    let _e140: Motor = self_1885;
    let _e144: PointAndPlane = other_1575;
    let _e156: Motor = self_1885;
    let _e160: PointAndPlane = other_1575;
    let _e163: Motor = self_1885;
    let _e167: PointAndPlane = other_1575;
    return PointAndPlane((((((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y)) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) + ((vec4<f32>(_e19.g0_.z) * vec4<f32>(_e23.g0_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e35.g0_.w) * vec4<f32>(_e39.g0_.w)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + (vec4<f32>(_e51.g1_.x) * _e55.g0_)) + ((vec4<f32>(_e59.g1_.y) * vec4<f32>(_e63.g1_.y, _e66.g1_.x, _e69.g0_.w, _e72.g0_.z)) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e86.g1_.z) * vec4<f32>(_e90.g1_.z, _e93.g0_.w, _e96.g1_.x, _e99.g0_.y)) * vec4<f32>(-(1.0), -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e113.g1_.w) * vec4<f32>(_e117.g1_.w, _e120.g0_.z, _e123.g0_.y, _e126.g1_.x)) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))) + ((vec4<f32>(_e140.g0_.x) * vec4<f32>(_e144.g1_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))), ((vec4<f32>(_e156.g1_.x) * _e160.g1_) + ((vec4<f32>(_e163.g0_.x) * _e167.g0_) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn motor_point_and_plane_left_contraction(self_1886: Motor, other_1576: PointAndPlane) -> PointAndPlane {
    var self_1887: Motor;
    var other_1577: PointAndPlane;

    self_1887 = self_1886;
    other_1577 = other_1576;
    let _e4: Motor = self_1887;
    let _e8: PointAndPlane = other_1577;
    let _e11: Motor = self_1887;
    let _e15: PointAndPlane = other_1577;
    let _e18: Motor = self_1887;
    let _e22: PointAndPlane = other_1577;
    let _e34: Motor = self_1887;
    let _e38: PointAndPlane = other_1577;
    let _e50: Motor = self_1887;
    let _e53: PointAndPlane = other_1577;
    return PointAndPlane((vec4<f32>(_e4.g0_.x) * _e8.g0_), ((((vec4<f32>(_e11.g0_.x) * _e15.g1_) + ((vec4<f32>(_e18.g0_.z) * _e22.g0_.zzxz) * vec4<f32>(1.0, 0.0, -(1.0), 0.0))) + ((vec4<f32>(_e34.g0_.w) * _e38.g0_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))) + ((_e50.g0_.yyxx * _e53.g0_.yxxx) * vec4<f32>(1.0, -(1.0), 0.0, 0.0))));
}

fn motor_point_and_plane_left_anti_contraction(self_1888: Motor, other_1578: PointAndPlane) -> PointAndPlane {
    var self_1889: Motor;
    var other_1579: PointAndPlane;

    self_1889 = self_1888;
    other_1579 = other_1578;
    let _e4: Motor = self_1889;
    let _e8: PointAndPlane = other_1579;
    let _e11: Motor = self_1889;
    let _e15: PointAndPlane = other_1579;
    let _e27: Motor = self_1889;
    let _e31: PointAndPlane = other_1579;
    let _e43: Motor = self_1889;
    let _e46: PointAndPlane = other_1579;
    let _e58: Motor = self_1889;
    let _e62: PointAndPlane = other_1579;
    return PointAndPlane(((((vec4<f32>(_e4.g1_.x) * _e8.g0_) + ((vec4<f32>(_e11.g1_.z) * _e15.g1_.zzxz) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0))) + ((vec4<f32>(_e27.g1_.w) * _e31.g1_.wwwx) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))) + ((_e43.g1_.yyxx * _e46.g1_.yxxx) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))), (vec4<f32>(_e58.g1_.x) * _e62.g1_));
}

fn motor_squared_magnitude(self_1890: Motor) -> Scalar {
    var self_1891: Motor;

    self_1891 = self_1890;
    let _e2: Motor = self_1891;
    let _e3: Motor = self_1891;
    let _e4: Motor = motor_reversal(_e3);
    let _e5: Scalar = motor_motor_scalar_product(_e2, _e4);
    return _e5;
}

fn motor_magnitude(self_1892: Motor) -> Scalar {
    var self_1893: Motor;

    self_1893 = self_1892;
    let _e2: Motor = self_1893;
    let _e3: Scalar = motor_squared_magnitude(_e2);
    return Scalar(sqrt(_e3.g0_));
}

fn motor_bulk_norm(self_1894: Motor) -> Scalar {
    var self_1895: Motor;

    self_1895 = self_1894;
    let _e2: Motor = self_1895;
    let _e3: Scalar = motor_squared_magnitude(_e2);
    return Scalar(sqrt(_e3.g0_));
}

fn motor_squared_anti_magnitude(self_1896: Motor) -> AntiScalar {
    var self_1897: Motor;

    self_1897 = self_1896;
    let _e2: Motor = self_1897;
    let _e3: Motor = self_1897;
    let _e4: Motor = motor_anti_reversal(_e3);
    let _e5: AntiScalar = motor_motor_anti_scalar_product(_e2, _e4);
    return _e5;
}

fn motor_weight_norm(self_1898: Motor) -> AntiScalar {
    var self_1899: Motor;

    self_1899 = self_1898;
    let _e2: Motor = self_1899;
    let _e3: AntiScalar = motor_squared_anti_magnitude(_e2);
    return AntiScalar(sqrt(_e3.g0_));
}

fn motor_scale(self_1900: Motor, other_1580: f32) -> Motor {
    var self_1901: Motor;
    var other_1581: f32;

    self_1901 = self_1900;
    other_1581 = other_1580;
    let _e4: Motor = self_1901;
    let _e5: f32 = other_1581;
    let _e7: Motor = motor_scalar_geometric_product(_e4, Scalar(_e5));
    return _e7;
}

fn motor_signum(self_1902: Motor) -> Motor {
    var self_1903: Motor;

    self_1903 = self_1902;
    let _e2: Motor = self_1903;
    let _e3: Motor = self_1903;
    let _e4: Scalar = motor_magnitude(_e3);
    let _e9: Motor = motor_scalar_geometric_product(_e2, Scalar((1.0 / _e4.g0_)));
    return _e9;
}

fn motor_inverse(self_1904: Motor) -> Motor {
    var self_1905: Motor;

    self_1905 = self_1904;
    let _e2: Motor = self_1905;
    let _e3: Motor = motor_reversal(_e2);
    let _e4: Motor = self_1905;
    let _e5: Scalar = motor_squared_magnitude(_e4);
    let _e10: Motor = motor_scalar_geometric_product(_e3, Scalar((1.0 / _e5.g0_)));
    return _e10;
}

fn motor_unitize(self_1906: Motor) -> Motor {
    var self_1907: Motor;

    self_1907 = self_1906;
    let _e2: Motor = self_1907;
    let _e3: Motor = self_1907;
    let _e4: AntiScalar = motor_weight_norm(_e3);
    let _e9: Motor = motor_scalar_geometric_product(_e2, Scalar((1.0 / _e4.g0_)));
    return _e9;
}

fn point_and_plane_zero() -> PointAndPlane {
    return PointAndPlane(vec4<f32>(0.0), vec4<f32>(0.0));
}

fn point_and_plane_one() -> PointAndPlane {
    return PointAndPlane(vec4<f32>(0.0), vec4<f32>(0.0));
}

fn point_and_plane_neg(self_1908: PointAndPlane) -> PointAndPlane {
    var self_1909: PointAndPlane;

    self_1909 = self_1908;
    let _e2: PointAndPlane = self_1909;
    let _e8: PointAndPlane = self_1909;
    return PointAndPlane((_e2.g0_ * vec4<f32>(-(1.0))), (_e8.g1_ * vec4<f32>(-(1.0))));
}

fn point_and_plane_automorphism(self_1910: PointAndPlane) -> PointAndPlane {
    var self_1911: PointAndPlane;

    self_1911 = self_1910;
    let _e2: PointAndPlane = self_1911;
    let _e8: PointAndPlane = self_1911;
    return PointAndPlane((_e2.g0_ * vec4<f32>(-(1.0))), (_e8.g1_ * vec4<f32>(-(1.0))));
}

fn point_and_plane_reversal(self_1912: PointAndPlane) -> PointAndPlane {
    var self_1913: PointAndPlane;

    self_1913 = self_1912;
    let _e2: PointAndPlane = self_1913;
    let _e8: PointAndPlane = self_1913;
    return PointAndPlane((_e2.g0_ * vec4<f32>(-(1.0))), _e8.g1_);
}

fn point_and_plane_conjugation(self_1914: PointAndPlane) -> PointAndPlane {
    var self_1915: PointAndPlane;

    self_1915 = self_1914;
    let _e2: PointAndPlane = self_1915;
    let _e4: PointAndPlane = self_1915;
    return PointAndPlane(_e2.g0_, (_e4.g1_ * vec4<f32>(-(1.0))));
}

fn point_and_plane_dual(self_1916: PointAndPlane) -> PointAndPlane {
    var self_1917: PointAndPlane;

    self_1917 = self_1916;
    let _e2: PointAndPlane = self_1917;
    let _e4: PointAndPlane = self_1917;
    return PointAndPlane(_e2.g1_, (_e4.g0_ * vec4<f32>(-(1.0))));
}

fn point_and_plane_anti_reversal(self_1918: PointAndPlane) -> PointAndPlane {
    var self_1919: PointAndPlane;

    self_1919 = self_1918;
    let _e2: PointAndPlane = self_1919;
    let _e4: PointAndPlane = self_1919;
    return PointAndPlane(_e2.g0_, (_e4.g1_ * vec4<f32>(-(1.0))));
}

fn point_and_plane_right_complement(self_1920: PointAndPlane) -> PointAndPlane {
    var self_1921: PointAndPlane;

    self_1921 = self_1920;
    let _e2: PointAndPlane = self_1921;
    let _e4: PointAndPlane = self_1921;
    return PointAndPlane(_e2.g1_, (_e4.g0_ * vec4<f32>(-(1.0))));
}

fn point_and_plane_left_complement(self_1922: PointAndPlane) -> PointAndPlane {
    var self_1923: PointAndPlane;

    self_1923 = self_1922;
    let _e2: PointAndPlane = self_1923;
    let _e8: PointAndPlane = self_1923;
    return PointAndPlane((_e2.g1_ * vec4<f32>(-(1.0))), _e8.g0_);
}

fn point_and_plane_double_complement(self_1924: PointAndPlane) -> PointAndPlane {
    var self_1925: PointAndPlane;

    self_1925 = self_1924;
    let _e2: PointAndPlane = self_1925;
    let _e8: PointAndPlane = self_1925;
    return PointAndPlane((_e2.g0_ * vec4<f32>(-(1.0))), (_e8.g1_ * vec4<f32>(-(1.0))));
}

fn point_and_plane_scalar_geometric_product(self_1926: PointAndPlane, other_1582: Scalar) -> PointAndPlane {
    var self_1927: PointAndPlane;
    var other_1583: Scalar;

    self_1927 = self_1926;
    other_1583 = other_1582;
    let _e4: PointAndPlane = self_1927;
    let _e6: Scalar = other_1583;
    let _e10: PointAndPlane = self_1927;
    let _e12: Scalar = other_1583;
    return PointAndPlane((_e4.g0_ * vec4<f32>(_e6.g0_)), (_e10.g1_ * vec4<f32>(_e12.g0_)));
}

fn point_and_plane_scalar_outer_product(self_1928: PointAndPlane, other_1584: Scalar) -> PointAndPlane {
    var self_1929: PointAndPlane;
    var other_1585: Scalar;

    self_1929 = self_1928;
    other_1585 = other_1584;
    let _e4: PointAndPlane = self_1929;
    let _e6: Scalar = other_1585;
    let _e10: PointAndPlane = self_1929;
    let _e12: Scalar = other_1585;
    return PointAndPlane((_e4.g0_ * vec4<f32>(_e6.g0_)), (_e10.g1_ * vec4<f32>(_e12.g0_)));
}

fn point_and_plane_scalar_inner_product(self_1930: PointAndPlane, other_1586: Scalar) -> PointAndPlane {
    var self_1931: PointAndPlane;
    var other_1587: Scalar;

    self_1931 = self_1930;
    other_1587 = other_1586;
    let _e4: PointAndPlane = self_1931;
    let _e6: Scalar = other_1587;
    let _e10: PointAndPlane = self_1931;
    let _e12: Scalar = other_1587;
    return PointAndPlane((_e4.g0_ * vec4<f32>(_e6.g0_)), (_e10.g1_ * vec4<f32>(_e12.g0_)));
}

fn point_and_plane_scalar_right_contraction(self_1932: PointAndPlane, other_1588: Scalar) -> PointAndPlane {
    var self_1933: PointAndPlane;
    var other_1589: Scalar;

    self_1933 = self_1932;
    other_1589 = other_1588;
    let _e4: PointAndPlane = self_1933;
    let _e6: Scalar = other_1589;
    let _e10: PointAndPlane = self_1933;
    let _e12: Scalar = other_1589;
    return PointAndPlane((_e4.g0_ * vec4<f32>(_e6.g0_)), (_e10.g1_ * vec4<f32>(_e12.g0_)));
}

fn point_and_plane_anti_scalar_regressive_product(self_1934: PointAndPlane, other_1590: AntiScalar) -> PointAndPlane {
    var self_1935: PointAndPlane;
    var other_1591: AntiScalar;

    self_1935 = self_1934;
    other_1591 = other_1590;
    let _e4: PointAndPlane = self_1935;
    let _e6: AntiScalar = other_1591;
    let _e10: PointAndPlane = self_1935;
    let _e12: AntiScalar = other_1591;
    return PointAndPlane((_e4.g0_ * vec4<f32>(_e6.g0_)), (_e10.g1_ * vec4<f32>(_e12.g0_)));
}

fn point_and_plane_anti_scalar_geometric_anti_product(self_1936: PointAndPlane, other_1592: AntiScalar) -> PointAndPlane {
    var self_1937: PointAndPlane;
    var other_1593: AntiScalar;

    self_1937 = self_1936;
    other_1593 = other_1592;
    let _e4: PointAndPlane = self_1937;
    let _e6: AntiScalar = other_1593;
    let _e10: PointAndPlane = self_1937;
    let _e12: AntiScalar = other_1593;
    return PointAndPlane((_e4.g0_ * vec4<f32>(_e6.g0_)), (_e10.g1_ * vec4<f32>(_e12.g0_)));
}

fn point_and_plane_anti_scalar_inner_anti_product(self_1938: PointAndPlane, other_1594: AntiScalar) -> PointAndPlane {
    var self_1939: PointAndPlane;
    var other_1595: AntiScalar;

    self_1939 = self_1938;
    other_1595 = other_1594;
    let _e4: PointAndPlane = self_1939;
    let _e6: AntiScalar = other_1595;
    let _e10: PointAndPlane = self_1939;
    let _e12: AntiScalar = other_1595;
    return PointAndPlane((_e4.g0_ * vec4<f32>(_e6.g0_)), (_e10.g1_ * vec4<f32>(_e12.g0_)));
}

fn point_and_plane_anti_scalar_right_anti_contraction(self_1940: PointAndPlane, other_1596: AntiScalar) -> PointAndPlane {
    var self_1941: PointAndPlane;
    var other_1597: AntiScalar;

    self_1941 = self_1940;
    other_1597 = other_1596;
    let _e4: PointAndPlane = self_1941;
    let _e6: AntiScalar = other_1597;
    let _e10: PointAndPlane = self_1941;
    let _e12: AntiScalar = other_1597;
    return PointAndPlane((_e4.g0_ * vec4<f32>(_e6.g0_)), (_e10.g1_ * vec4<f32>(_e12.g0_)));
}

fn point_and_plane_multi_vector_add(self_1942: PointAndPlane, other_1598: MultiVector) -> MultiVector {
    var self_1943: PointAndPlane;
    var other_1599: MultiVector;

    self_1943 = self_1942;
    other_1599 = other_1598;
    let _e4: MultiVector = other_1599;
    let _e6: PointAndPlane = self_1943;
    let _e9: PointAndPlane = self_1943;
    let _e12: PointAndPlane = self_1943;
    let _e15: PointAndPlane = self_1943;
    let _e19: MultiVector = other_1599;
    let _e22: PointAndPlane = self_1943;
    let _e25: PointAndPlane = self_1943;
    let _e28: PointAndPlane = self_1943;
    let _e31: PointAndPlane = self_1943;
    let _e35: MultiVector = other_1599;
    let _e38: MultiVector = other_1599;
    return MultiVector(_e4.g0_, (vec4<f32>(_e6.g1_.x, _e9.g0_.y, _e12.g0_.z, _e15.g0_.w) + _e19.g1_), (vec4<f32>(_e22.g0_.x, _e25.g1_.y, _e28.g1_.z, _e31.g1_.w) + _e35.g2_), _e38.g3_);
}

fn point_and_plane_multi_vector_sub(self_1944: PointAndPlane, other_1600: MultiVector) -> MultiVector {
    var self_1945: PointAndPlane;
    var other_1601: MultiVector;

    self_1945 = self_1944;
    other_1601 = other_1600;
    let _e6: MultiVector = other_1601;
    let _e9: PointAndPlane = self_1945;
    let _e12: PointAndPlane = self_1945;
    let _e15: PointAndPlane = self_1945;
    let _e18: PointAndPlane = self_1945;
    let _e22: MultiVector = other_1601;
    let _e25: PointAndPlane = self_1945;
    let _e28: PointAndPlane = self_1945;
    let _e31: PointAndPlane = self_1945;
    let _e34: PointAndPlane = self_1945;
    let _e38: MultiVector = other_1601;
    let _e43: MultiVector = other_1601;
    return MultiVector((vec4<f32>(0.0) - _e6.g0_), (vec4<f32>(_e9.g1_.x, _e12.g0_.y, _e15.g0_.z, _e18.g0_.w) - _e22.g1_), (vec4<f32>(_e25.g0_.x, _e28.g1_.y, _e31.g1_.z, _e34.g1_.w) - _e38.g2_), (vec4<f32>(0.0) - _e43.g3_));
}

fn point_and_plane_multi_vector_geometric_product(self_1946: PointAndPlane, other_1602: MultiVector) -> MultiVector {
    var self_1947: PointAndPlane;
    var other_1603: MultiVector;

    self_1947 = self_1946;
    other_1603 = other_1602;
    let _e4: PointAndPlane = self_1947;
    let _e8: MultiVector = other_1603;
    let _e18: PointAndPlane = self_1947;
    let _e22: MultiVector = other_1603;
    let _e34: PointAndPlane = self_1947;
    let _e38: MultiVector = other_1603;
    let _e50: PointAndPlane = self_1947;
    let _e54: MultiVector = other_1603;
    let _e66: PointAndPlane = self_1947;
    let _e70: MultiVector = other_1603;
    let _e73: PointAndPlane = self_1947;
    let _e77: MultiVector = other_1603;
    let _e89: PointAndPlane = self_1947;
    let _e93: MultiVector = other_1603;
    let _e105: PointAndPlane = self_1947;
    let _e109: MultiVector = other_1603;
    let _e121: PointAndPlane = self_1947;
    let _e125: MultiVector = other_1603;
    let _e138: PointAndPlane = self_1947;
    let _e142: MultiVector = other_1603;
    let _e155: PointAndPlane = self_1947;
    let _e159: MultiVector = other_1603;
    let _e172: PointAndPlane = self_1947;
    let _e176: MultiVector = other_1603;
    let _e189: PointAndPlane = self_1947;
    let _e193: MultiVector = other_1603;
    let _e205: PointAndPlane = self_1947;
    let _e209: MultiVector = other_1603;
    let _e221: PointAndPlane = self_1947;
    let _e225: MultiVector = other_1603;
    let _e237: PointAndPlane = self_1947;
    let _e241: MultiVector = other_1603;
    let _e255: PointAndPlane = self_1947;
    let _e259: MultiVector = other_1603;
    let _e263: PointAndPlane = self_1947;
    let _e267: MultiVector = other_1603;
    let _e280: PointAndPlane = self_1947;
    let _e284: MultiVector = other_1603;
    let _e297: PointAndPlane = self_1947;
    let _e301: MultiVector = other_1603;
    let _e314: PointAndPlane = self_1947;
    let _e318: MultiVector = other_1603;
    let _e322: PointAndPlane = self_1947;
    let _e326: MultiVector = other_1603;
    let _e339: PointAndPlane = self_1947;
    let _e343: MultiVector = other_1603;
    let _e356: PointAndPlane = self_1947;
    let _e360: MultiVector = other_1603;
    return MultiVector((((((vec4<f32>(_e4.g0_.x) * _e8.g2_) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0)) + ((vec4<f32>(_e18.g1_.y) * _e22.g2_.yxwz) * vec4<f32>(1.0, 1.0, -(1.0), 1.0))) + ((vec4<f32>(_e34.g1_.z) * _e38.g2_.zwxy) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e50.g1_.w) * _e54.g2_.wzyx) * vec4<f32>(1.0, -(1.0), 1.0, 1.0))), ((((((((vec4<f32>(_e66.g0_.x) * _e70.g3_) + ((vec4<f32>(_e73.g0_.y) * _e77.g0_.yxwz) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e89.g0_.z) * _e93.g0_.zwxy) * vec4<f32>(1.0, -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e105.g0_.w) * _e109.g0_.wzyx) * vec4<f32>(1.0, 1.0, -(1.0), 1.0))) + ((vec4<f32>(_e121.g1_.x) * _e125.g0_) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0)))) + ((vec4<f32>(_e138.g1_.y) * _e142.g3_.yxwz) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))) + ((vec4<f32>(_e155.g1_.z) * _e159.g3_.zwxy) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e172.g1_.w) * _e176.g3_.wzyx) * vec4<f32>(-(1.0), -(1.0), 1.0, 1.0))), (((((vec4<f32>(_e189.g0_.x) * _e193.g0_) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))) + ((vec4<f32>(_e205.g1_.y) * _e209.g0_.yxwz) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e221.g1_.z) * _e225.g0_.zwxy) * vec4<f32>(1.0, -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e237.g1_.w) * _e241.g0_.wzyx) * vec4<f32>(1.0, 1.0, -(1.0), 1.0))), ((((((((vec4<f32>(0.0) - (vec4<f32>(_e255.g0_.x) * _e259.g1_)) + ((vec4<f32>(_e263.g0_.y) * _e267.g2_.yxwz) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))) + ((vec4<f32>(_e280.g0_.z) * _e284.g2_.zwxy) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e297.g0_.w) * _e301.g2_.wzyx) * vec4<f32>(-(1.0), -(1.0), 1.0, 1.0))) + (vec4<f32>(_e314.g1_.x) * _e318.g2_)) + ((vec4<f32>(_e322.g1_.y) * _e326.g1_.yxwz) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e339.g1_.z) * _e343.g1_.zwxy) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e356.g1_.w) * _e360.g1_.wzyx) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))));
}

fn point_and_plane_multi_vector_geometric_anti_product(self_1948: PointAndPlane, other_1604: MultiVector) -> MultiVector {
    var self_1949: PointAndPlane;
    var other_1605: MultiVector;

    self_1949 = self_1948;
    other_1605 = other_1604;
    let _e4: PointAndPlane = self_1949;
    let _e8: MultiVector = other_1605;
    let _e18: PointAndPlane = self_1949;
    let _e22: MultiVector = other_1605;
    let _e36: PointAndPlane = self_1949;
    let _e40: MultiVector = other_1605;
    let _e54: PointAndPlane = self_1949;
    let _e58: MultiVector = other_1605;
    let _e72: PointAndPlane = self_1949;
    let _e76: MultiVector = other_1605;
    let _e89: PointAndPlane = self_1949;
    let _e93: MultiVector = other_1605;
    let _e105: PointAndPlane = self_1949;
    let _e109: MultiVector = other_1605;
    let _e121: PointAndPlane = self_1949;
    let _e125: MultiVector = other_1605;
    let _e137: PointAndPlane = self_1949;
    let _e141: MultiVector = other_1605;
    let _e153: PointAndPlane = self_1949;
    let _e157: MultiVector = other_1605;
    let _e170: PointAndPlane = self_1949;
    let _e174: MultiVector = other_1605;
    let _e187: PointAndPlane = self_1949;
    let _e191: MultiVector = other_1605;
    let _e195: PointAndPlane = self_1949;
    let _e199: MultiVector = other_1605;
    let _e202: PointAndPlane = self_1949;
    let _e206: MultiVector = other_1605;
    let _e220: PointAndPlane = self_1949;
    let _e224: MultiVector = other_1605;
    let _e238: PointAndPlane = self_1949;
    let _e242: MultiVector = other_1605;
    let _e256: PointAndPlane = self_1949;
    let _e260: MultiVector = other_1605;
    let _e271: PointAndPlane = self_1949;
    let _e275: MultiVector = other_1605;
    let _e288: PointAndPlane = self_1949;
    let _e292: MultiVector = other_1605;
    let _e305: PointAndPlane = self_1949;
    let _e309: MultiVector = other_1605;
    let _e322: PointAndPlane = self_1949;
    let _e326: MultiVector = other_1605;
    let _e338: PointAndPlane = self_1949;
    let _e342: MultiVector = other_1605;
    let _e355: PointAndPlane = self_1949;
    let _e359: MultiVector = other_1605;
    let _e372: PointAndPlane = self_1949;
    let _e376: MultiVector = other_1605;
    return MultiVector((((((((((vec4<f32>(_e4.g0_.x) * _e8.g1_) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0)) + ((vec4<f32>(_e18.g0_.y) * _e22.g2_.yxwz) * vec4<f32>(-(1.0), -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e36.g0_.z) * _e40.g2_.zwxy) * vec4<f32>(-(1.0), 1.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e54.g0_.w) * _e58.g2_.wzyx) * vec4<f32>(-(1.0), -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e72.g1_.x) * _e76.g2_) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0)))) + ((vec4<f32>(_e89.g1_.y) * _e93.g1_.yxwz) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e105.g1_.z) * _e109.g1_.zwxy) * vec4<f32>(1.0, -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e121.g1_.w) * _e125.g1_.wzyx) * vec4<f32>(1.0, 1.0, -(1.0), 1.0))), (((((vec4<f32>(_e137.g0_.y) * _e141.g3_.yxwz) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0))) + ((vec4<f32>(_e153.g0_.z) * _e157.g3_.zwxy) * vec4<f32>(-(1.0), -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e170.g0_.w) * _e174.g3_.wzyx) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))) + (vec4<f32>(_e187.g1_.x) * _e191.g3_)), ((((((((vec4<f32>(_e195.g0_.x) * _e199.g3_) + ((vec4<f32>(_e202.g0_.y) * _e206.g0_.yxwz) * vec4<f32>(-(1.0), -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e220.g0_.z) * _e224.g0_.zwxy) * vec4<f32>(-(1.0), -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e238.g0_.w) * _e242.g0_.wzyx) * vec4<f32>(-(1.0), 1.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e256.g1_.x) * _e260.g0_) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0))) + ((vec4<f32>(_e271.g1_.y) * _e275.g3_.yxwz) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e288.g1_.z) * _e292.g3_.zwxy) * vec4<f32>(-(1.0), -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e305.g1_.w) * _e309.g3_.wzyx) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))), (((((vec4<f32>(_e322.g0_.y) * _e326.g1_.yxwz) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0)) + ((vec4<f32>(_e338.g0_.z) * _e342.g1_.zwxy) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e355.g0_.w) * _e359.g1_.wzyx) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))) - (vec4<f32>(_e372.g1_.x) * _e376.g1_)));
}

fn point_and_plane_multi_vector_scalar_product(self_1950: PointAndPlane, other_1606: MultiVector) -> Scalar {
    var self_1951: PointAndPlane;
    var other_1607: MultiVector;

    self_1951 = self_1950;
    other_1607 = other_1606;
    let _e5: PointAndPlane = self_1951;
    let _e8: MultiVector = other_1607;
    let _e13: PointAndPlane = self_1951;
    let _e16: MultiVector = other_1607;
    let _e21: PointAndPlane = self_1951;
    let _e24: MultiVector = other_1607;
    let _e29: PointAndPlane = self_1951;
    let _e32: MultiVector = other_1607;
    return Scalar(((((0.0 - (_e5.g0_.x * _e8.g2_.x)) + (_e13.g1_.y * _e16.g2_.y)) + (_e21.g1_.z * _e24.g2_.z)) + (_e29.g1_.w * _e32.g2_.w)));
}

fn point_and_plane_multi_vector_anti_scalar_product(self_1952: PointAndPlane, other_1608: MultiVector) -> AntiScalar {
    var self_1953: PointAndPlane;
    var other_1609: MultiVector;

    self_1953 = self_1952;
    other_1609 = other_1608;
    let _e4: PointAndPlane = self_1953;
    let _e7: MultiVector = other_1609;
    let _e11: PointAndPlane = self_1953;
    let _e14: MultiVector = other_1609;
    let _e19: PointAndPlane = self_1953;
    let _e22: MultiVector = other_1609;
    let _e27: PointAndPlane = self_1953;
    let _e30: MultiVector = other_1609;
    return AntiScalar(((((_e4.g0_.y * _e7.g1_.y) + (_e11.g0_.z * _e14.g1_.z)) + (_e19.g0_.w * _e22.g1_.w)) - (_e27.g1_.x * _e30.g1_.x)));
}

fn point_and_plane_rotor_geometric_product(self_1954: PointAndPlane, other_1610: Rotor) -> PointAndPlane {
    var self_1955: PointAndPlane;
    var other_1611: Rotor;

    self_1955 = self_1954;
    other_1611 = other_1610;
    let _e4: PointAndPlane = self_1955;
    let _e8: Rotor = other_1611;
    let _e19: PointAndPlane = self_1955;
    let _e23: Rotor = other_1611;
    let _e35: PointAndPlane = self_1955;
    let _e39: Rotor = other_1611;
    let _e53: PointAndPlane = self_1955;
    let _e57: Rotor = other_1611;
    let _e69: PointAndPlane = self_1955;
    let _e73: Rotor = other_1611;
    let _e85: PointAndPlane = self_1955;
    let _e89: Rotor = other_1611;
    let _e101: PointAndPlane = self_1955;
    let _e104: Rotor = other_1611;
    let _e116: PointAndPlane = self_1955;
    let _e120: Rotor = other_1611;
    let _e131: PointAndPlane = self_1955;
    let _e135: Rotor = other_1611;
    let _e147: PointAndPlane = self_1955;
    let _e151: Rotor = other_1611;
    let _e163: PointAndPlane = self_1955;
    let _e167: Rotor = other_1611;
    let _e179: PointAndPlane = self_1955;
    let _e183: Rotor = other_1611;
    let _e195: PointAndPlane = self_1955;
    let _e199: Rotor = other_1611;
    let _e211: PointAndPlane = self_1955;
    let _e214: Rotor = other_1611;
    return PointAndPlane(((((((((vec4<f32>(_e4.g0_.z) * _e8.g0_.wwxy) * vec4<f32>(0.0, -(1.0), 1.0, 1.0)) + ((vec4<f32>(_e19.g0_.w) * _e23.g0_.zzyx) * vec4<f32>(0.0, 1.0, -(1.0), 1.0))) + ((vec4<f32>(_e35.g1_.x) * _e39.g0_.yyzw) * vec4<f32>(0.0, -(1.0), -(1.0), -(1.0)))) + ((vec4<f32>(_e53.g1_.y) * vec4<f32>(_e57.g0_.y)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e69.g1_.z) * vec4<f32>(_e73.g0_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e85.g1_.w) * vec4<f32>(_e89.g0_.w)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((_e101.g0_.xyyy * _e104.g0_.xxwz) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))), ((((((((vec4<f32>(_e116.g0_.z) * vec4<f32>(_e120.g0_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) + ((vec4<f32>(_e131.g0_.w) * vec4<f32>(_e135.g0_.w)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e147.g1_.x) * vec4<f32>(_e151.g0_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e163.g1_.y) * _e167.g0_.xxwz) * vec4<f32>(0.0, 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e179.g1_.z) * _e183.g0_.wwxy) * vec4<f32>(0.0, -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e195.g1_.w) * _e199.g0_.zzyx) * vec4<f32>(0.0, 1.0, -(1.0), 1.0))) + ((_e211.g0_.yxxx * _e214.g0_.yyzw) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0)))));
}

fn point_and_plane_rotor_outer_product(self_1956: PointAndPlane, other_1612: Rotor) -> PointAndPlane {
    var self_1957: PointAndPlane;
    var other_1613: Rotor;

    self_1957 = self_1956;
    other_1613 = other_1612;
    let _e4: PointAndPlane = self_1957;
    let _e8: Rotor = other_1613;
    let _e21: PointAndPlane = self_1957;
    let _e25: Rotor = other_1613;
    let _e37: PointAndPlane = self_1957;
    let _e41: Rotor = other_1613;
    let _e53: PointAndPlane = self_1957;
    let _e57: Rotor = other_1613;
    let _e69: PointAndPlane = self_1957;
    let _e71: Rotor = other_1613;
    let _e77: PointAndPlane = self_1957;
    let _e79: Rotor = other_1613;
    return PointAndPlane(((((((vec4<f32>(_e4.g1_.x) * _e8.g0_.yyzw) * vec4<f32>(0.0, -(1.0), -(1.0), -(1.0))) + ((vec4<f32>(_e21.g1_.y) * vec4<f32>(_e25.g0_.y)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e37.g1_.z) * vec4<f32>(_e41.g0_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e53.g1_.w) * vec4<f32>(_e57.g0_.w)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + (_e69.g0_ * vec4<f32>(_e71.g0_.x))), (_e77.g1_ * vec4<f32>(_e79.g0_.x)));
}

fn point_and_plane_rotor_inner_product(self_1958: PointAndPlane, other_1614: Rotor) -> PointAndPlane {
    var self_1959: PointAndPlane;
    var other_1615: Rotor;

    self_1959 = self_1958;
    other_1615 = other_1614;
    let _e4: PointAndPlane = self_1959;
    let _e6: Rotor = other_1615;
    let _e11: PointAndPlane = self_1959;
    let _e15: Rotor = other_1615;
    let _e26: PointAndPlane = self_1959;
    let _e30: Rotor = other_1615;
    let _e42: PointAndPlane = self_1959;
    let _e46: Rotor = other_1615;
    let _e58: PointAndPlane = self_1959;
    let _e62: Rotor = other_1615;
    let _e74: PointAndPlane = self_1959;
    let _e78: Rotor = other_1615;
    let _e90: PointAndPlane = self_1959;
    let _e94: Rotor = other_1615;
    let _e106: PointAndPlane = self_1959;
    let _e109: Rotor = other_1615;
    return PointAndPlane((_e4.g0_ * vec4<f32>(_e6.g0_.x)), ((((((((vec4<f32>(_e11.g0_.z) * vec4<f32>(_e15.g0_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) + ((vec4<f32>(_e26.g0_.w) * vec4<f32>(_e30.g0_.w)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e42.g1_.x) * vec4<f32>(_e46.g0_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e58.g1_.y) * _e62.g0_.xxwz) * vec4<f32>(0.0, 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e74.g1_.z) * _e78.g0_.wwxy) * vec4<f32>(0.0, -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e90.g1_.w) * _e94.g0_.zzyx) * vec4<f32>(0.0, 1.0, -(1.0), 1.0))) + ((_e106.g0_.yxxx * _e109.g0_.yyzw) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0)))));
}

fn point_and_plane_rotor_right_contraction(self_1960: PointAndPlane, other_1616: Rotor) -> PointAndPlane {
    var self_1961: PointAndPlane;
    var other_1617: Rotor;

    self_1961 = self_1960;
    other_1617 = other_1616;
    let _e4: PointAndPlane = self_1961;
    let _e6: Rotor = other_1617;
    let _e11: PointAndPlane = self_1961;
    let _e15: Rotor = other_1617;
    let _e26: PointAndPlane = self_1961;
    let _e30: Rotor = other_1617;
    let _e42: PointAndPlane = self_1961;
    let _e46: Rotor = other_1617;
    let _e58: PointAndPlane = self_1961;
    let _e62: Rotor = other_1617;
    let _e74: PointAndPlane = self_1961;
    let _e78: Rotor = other_1617;
    let _e90: PointAndPlane = self_1961;
    let _e94: Rotor = other_1617;
    let _e106: PointAndPlane = self_1961;
    let _e109: Rotor = other_1617;
    return PointAndPlane((_e4.g0_ * vec4<f32>(_e6.g0_.x)), ((((((((vec4<f32>(_e11.g0_.z) * vec4<f32>(_e15.g0_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) + ((vec4<f32>(_e26.g0_.w) * vec4<f32>(_e30.g0_.w)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e42.g1_.x) * vec4<f32>(_e46.g0_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e58.g1_.y) * vec4<f32>(_e62.g0_.x)) * vec4<f32>(0.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e74.g1_.z) * vec4<f32>(_e78.g0_.x)) * vec4<f32>(0.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e90.g1_.w) * vec4<f32>(_e94.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((_e106.g0_.yxxx * _e109.g0_.yyzw) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0)))));
}

fn point_and_plane_point_into(self_1962: PointAndPlane) -> Point {
    var self_1963: PointAndPlane;

    self_1963 = self_1962;
    let _e2: PointAndPlane = self_1963;
    return Point(_e2.g0_);
}

fn point_and_plane_point_add(self_1964: PointAndPlane, other_1618: Point) -> PointAndPlane {
    var self_1965: PointAndPlane;
    var other_1619: Point;

    self_1965 = self_1964;
    other_1619 = other_1618;
    let _e4: PointAndPlane = self_1965;
    let _e6: Point = other_1619;
    let _e9: PointAndPlane = self_1965;
    return PointAndPlane((_e4.g0_ + _e6.g0_), _e9.g1_);
}

fn point_and_plane_point_sub(self_1966: PointAndPlane, other_1620: Point) -> PointAndPlane {
    var self_1967: PointAndPlane;
    var other_1621: Point;

    self_1967 = self_1966;
    other_1621 = other_1620;
    let _e4: PointAndPlane = self_1967;
    let _e6: Point = other_1621;
    let _e9: PointAndPlane = self_1967;
    return PointAndPlane((_e4.g0_ - _e6.g0_), _e9.g1_);
}

fn point_and_plane_point_geometric_product(self_1968: PointAndPlane, other_1622: Point) -> Motor {
    var self_1969: PointAndPlane;
    var other_1623: Point;

    self_1969 = self_1968;
    other_1623 = other_1622;
    let _e4: PointAndPlane = self_1969;
    let _e7: PointAndPlane = self_1969;
    let _e10: PointAndPlane = self_1969;
    let _e13: PointAndPlane = self_1969;
    let _e17: Point = other_1623;
    let _e29: PointAndPlane = self_1969;
    let _e33: Point = other_1623;
    let _e44: PointAndPlane = self_1969;
    let _e48: Point = other_1623;
    let _e60: PointAndPlane = self_1969;
    let _e64: Point = other_1623;
    let _e76: PointAndPlane = self_1969;
    let _e80: Point = other_1623;
    let _e92: PointAndPlane = self_1969;
    let _e96: Point = other_1623;
    let _e108: PointAndPlane = self_1969;
    let _e112: Point = other_1623;
    let _e124: PointAndPlane = self_1969;
    let _e127: PointAndPlane = self_1969;
    let _e130: PointAndPlane = self_1969;
    let _e133: PointAndPlane = self_1969;
    let _e137: Point = other_1623;
    return Motor(((vec4<f32>(_e4.g0_.x, _e7.g1_.y, _e10.g1_.z, _e13.g1_.w) * vec4<f32>(_e17.g0_.x)) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0)), ((((((((vec4<f32>(_e29.g0_.y) * vec4<f32>(_e33.g0_.x)) * vec4<f32>(0.0, 1.0, 0.0, 0.0)) + ((vec4<f32>(_e44.g0_.z) * vec4<f32>(_e48.g0_.x)) * vec4<f32>(0.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e60.g0_.w) * vec4<f32>(_e64.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e76.g1_.y) * _e80.g0_.yywz) * vec4<f32>(1.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e92.g1_.z) * _e96.g0_.zwzy) * vec4<f32>(1.0, -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e108.g1_.w) * _e112.g0_.wzyw) * vec4<f32>(1.0, 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e124.g1_.x, _e127.g0_.x, _e130.g0_.x, _e133.g0_.x) * _e137.g0_) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0)))));
}

fn point_and_plane_point_outer_product(self_1970: PointAndPlane, other_1624: Point) -> AntiScalar {
    var self_1971: PointAndPlane;
    var other_1625: Point;

    self_1971 = self_1970;
    other_1625 = other_1624;
    let _e4: PointAndPlane = self_1971;
    let _e7: Point = other_1625;
    let _e11: PointAndPlane = self_1971;
    let _e14: Point = other_1625;
    let _e19: PointAndPlane = self_1971;
    let _e22: Point = other_1625;
    let _e27: PointAndPlane = self_1971;
    let _e30: Point = other_1625;
    return AntiScalar(((((_e4.g1_.x * _e7.g0_.x) + (_e11.g1_.y * _e14.g0_.y)) + (_e19.g1_.z * _e22.g0_.z)) + (_e27.g1_.w * _e30.g0_.w)));
}

fn point_and_plane_point_geometric_anti_product(self_1972: PointAndPlane, other_1626: Point) -> Motor {
    var self_1973: PointAndPlane;
    var other_1627: Point;

    self_1973 = self_1972;
    other_1627 = other_1626;
    let _e4: PointAndPlane = self_1973;
    let _e8: Point = other_1627;
    let _e20: PointAndPlane = self_1973;
    let _e24: Point = other_1627;
    let _e37: PointAndPlane = self_1973;
    let _e41: Point = other_1627;
    let _e54: PointAndPlane = self_1973;
    let _e58: Point = other_1627;
    let _e70: PointAndPlane = self_1973;
    let _e74: Point = other_1627;
    let _e86: PointAndPlane = self_1973;
    let _e90: Point = other_1627;
    let _e102: PointAndPlane = self_1973;
    let _e105: PointAndPlane = self_1973;
    let _e108: PointAndPlane = self_1973;
    let _e111: PointAndPlane = self_1973;
    let _e115: Point = other_1627;
    let _e119: PointAndPlane = self_1973;
    let _e123: Point = other_1627;
    let _e134: PointAndPlane = self_1973;
    let _e138: Point = other_1627;
    let _e150: PointAndPlane = self_1973;
    let _e154: Point = other_1627;
    let _e168: PointAndPlane = self_1973;
    let _e171: Point = other_1627;
    return Motor(((((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.x)) * vec4<f32>(0.0, -(1.0), 0.0, 0.0)) + ((vec4<f32>(_e20.g0_.z) * vec4<f32>(_e24.g0_.x)) * vec4<f32>(0.0, 0.0, -(1.0), 0.0))) + ((vec4<f32>(_e37.g0_.w) * vec4<f32>(_e41.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e54.g1_.y) * _e58.g0_.yywz) * vec4<f32>(1.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e70.g1_.z) * _e74.g0_.zwzy) * vec4<f32>(1.0, -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e86.g1_.w) * _e90.g0_.wzyw) * vec4<f32>(1.0, 1.0, -(1.0), 0.0))) + (vec4<f32>(_e102.g1_.x, _e105.g0_.x, _e108.g0_.x, _e111.g0_.x) * _e115.g0_)), (((((vec4<f32>(_e119.g0_.z) * _e123.g0_.zwzy) * vec4<f32>(1.0, 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e134.g0_.w) * _e138.g0_.wzyw) * vec4<f32>(1.0, -(1.0), 1.0, 0.0))) + ((vec4<f32>(_e150.g1_.x) * _e154.g0_.yyzw) * vec4<f32>(0.0, -(1.0), -(1.0), -(1.0)))) + ((_e168.g0_.yxyy * _e171.g0_.yxwz) * vec4<f32>(1.0, 0.0, -(1.0), 1.0))));
}

fn point_and_plane_point_right_contraction(self_1974: PointAndPlane, other_1628: Point) -> Scalar {
    var self_1975: PointAndPlane;
    var other_1629: Point;

    self_1975 = self_1974;
    other_1629 = other_1628;
    let _e5: PointAndPlane = self_1975;
    let _e8: Point = other_1629;
    return Scalar((0.0 - (_e5.g0_.x * _e8.g0_.x)));
}

fn point_and_plane_point_left_anti_contraction(self_1976: PointAndPlane, other_1630: Point) -> AntiScalar {
    var self_1977: PointAndPlane;
    var other_1631: Point;

    self_1977 = self_1976;
    other_1631 = other_1630;
    let _e4: PointAndPlane = self_1977;
    let _e7: Point = other_1631;
    let _e11: PointAndPlane = self_1977;
    let _e14: Point = other_1631;
    let _e19: PointAndPlane = self_1977;
    let _e22: Point = other_1631;
    return AntiScalar((((_e4.g0_.y * _e7.g0_.y) + (_e11.g0_.z * _e14.g0_.z)) + (_e19.g0_.w * _e22.g0_.w)));
}

fn point_and_plane_point_scalar_product(self_1978: PointAndPlane, other_1632: Point) -> Scalar {
    var self_1979: PointAndPlane;
    var other_1633: Point;

    self_1979 = self_1978;
    other_1633 = other_1632;
    let _e5: PointAndPlane = self_1979;
    let _e8: Point = other_1633;
    return Scalar((0.0 - (_e5.g0_.x * _e8.g0_.x)));
}

fn point_and_plane_point_anti_scalar_product(self_1980: PointAndPlane, other_1634: Point) -> AntiScalar {
    var self_1981: PointAndPlane;
    var other_1635: Point;

    self_1981 = self_1980;
    other_1635 = other_1634;
    let _e4: PointAndPlane = self_1981;
    let _e7: Point = other_1635;
    let _e11: PointAndPlane = self_1981;
    let _e14: Point = other_1635;
    let _e19: PointAndPlane = self_1981;
    let _e22: Point = other_1635;
    return AntiScalar((((_e4.g0_.y * _e7.g0_.y) + (_e11.g0_.z * _e14.g0_.z)) + (_e19.g0_.w * _e22.g0_.w)));
}

fn point_and_plane_ideal_point_regressive_product(self_1982: PointAndPlane, other_1636: IdealPoint) -> Plane {
    var self_1983: PointAndPlane;
    var other_1637: IdealPoint;

    self_1983 = self_1982;
    other_1637 = other_1636;
    let _e4: PointAndPlane = self_1983;
    let _e8: IdealPoint = other_1637;
    let _e20: PointAndPlane = self_1983;
    let _e24: IdealPoint = other_1637;
    let _e37: PointAndPlane = self_1983;
    let _e40: IdealPoint = other_1637;
    let _e43: IdealPoint = other_1637;
    let _e46: IdealPoint = other_1637;
    let _e49: IdealPoint = other_1637;
    return Plane(((((vec4<f32>(_e4.g0_.z) * vec4<f32>(_e8.g0_.y)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0)) + ((vec4<f32>(_e20.g0_.w) * vec4<f32>(_e24.g0_.z)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((_e37.g0_.yxxx * vec4<f32>(_e40.g0_.x, _e43.g0_.x, _e46.g0_.y, _e49.g0_.z)) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0))));
}

fn point_and_plane_ideal_point_geometric_anti_product(self_1984: PointAndPlane, other_1638: IdealPoint) -> PointAndPlane {
    var self_1985: PointAndPlane;
    var other_1639: IdealPoint;

    self_1985 = self_1984;
    other_1639 = other_1638;
    let _e4: PointAndPlane = self_1985;
    let _e8: IdealPoint = other_1639;
    let _e11: IdealPoint = other_1639;
    let _e14: IdealPoint = other_1639;
    let _e17: IdealPoint = other_1639;
    let _e29: PointAndPlane = self_1985;
    let _e33: IdealPoint = other_1639;
    let _e36: IdealPoint = other_1639;
    let _e39: IdealPoint = other_1639;
    let _e42: IdealPoint = other_1639;
    let _e55: PointAndPlane = self_1985;
    let _e59: IdealPoint = other_1639;
    let _e62: IdealPoint = other_1639;
    let _e65: IdealPoint = other_1639;
    let _e68: IdealPoint = other_1639;
    let _e80: PointAndPlane = self_1985;
    let _e84: IdealPoint = other_1639;
    let _e97: PointAndPlane = self_1985;
    let _e101: IdealPoint = other_1639;
    let _e114: PointAndPlane = self_1985;
    let _e117: PointAndPlane = self_1985;
    let _e120: PointAndPlane = self_1985;
    let _e123: PointAndPlane = self_1985;
    let _e127: IdealPoint = other_1639;
    let _e130: IdealPoint = other_1639;
    let _e133: IdealPoint = other_1639;
    let _e136: IdealPoint = other_1639;
    let _e150: PointAndPlane = self_1985;
    let _e154: IdealPoint = other_1639;
    let _e166: PointAndPlane = self_1985;
    let _e170: IdealPoint = other_1639;
    let _e183: PointAndPlane = self_1985;
    let _e187: IdealPoint = other_1639;
    let _e190: IdealPoint = other_1639;
    let _e193: IdealPoint = other_1639;
    let _e196: IdealPoint = other_1639;
    let _e209: PointAndPlane = self_1985;
    let _e213: IdealPoint = other_1639;
    let _e216: IdealPoint = other_1639;
    let _e219: IdealPoint = other_1639;
    let _e222: IdealPoint = other_1639;
    let _e235: PointAndPlane = self_1985;
    let _e239: IdealPoint = other_1639;
    let _e242: IdealPoint = other_1639;
    let _e245: IdealPoint = other_1639;
    let _e248: IdealPoint = other_1639;
    let _e261: PointAndPlane = self_1985;
    let _e264: IdealPoint = other_1639;
    let _e267: IdealPoint = other_1639;
    let _e270: IdealPoint = other_1639;
    let _e273: IdealPoint = other_1639;
    return PointAndPlane((((((((vec4<f32>(_e4.g0_.z) * vec4<f32>(_e8.g0_.z, _e11.g0_.z, _e14.g0_.z, _e17.g0_.x)) * vec4<f32>(0.0, -(1.0), 0.0, 1.0)) + ((vec4<f32>(_e29.g0_.w) * vec4<f32>(_e33.g0_.y, _e36.g0_.y, _e39.g0_.x, _e42.g0_.y)) * vec4<f32>(0.0, 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e55.g1_.x) * vec4<f32>(_e59.g0_.x, _e62.g0_.x, _e65.g0_.y, _e68.g0_.z)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))) + ((vec4<f32>(_e80.g1_.z) * vec4<f32>(_e84.g0_.y)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((vec4<f32>(_e97.g1_.w) * vec4<f32>(_e101.g0_.z)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((vec4<f32>(_e114.g1_.y, _e117.g0_.x, _e120.g0_.y, _e123.g0_.y) * vec4<f32>(_e127.g0_.x, _e130.g0_.x, _e133.g0_.z, _e136.g0_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0)))), (((((((vec4<f32>(_e150.g0_.z) * vec4<f32>(_e154.g0_.y)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0)) + ((vec4<f32>(_e166.g0_.w) * vec4<f32>(_e170.g0_.z)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((vec4<f32>(_e183.g1_.y) * vec4<f32>(_e187.g0_.z, _e190.g0_.z, _e193.g0_.z, _e196.g0_.y)) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e209.g1_.z) * vec4<f32>(_e213.g0_.z, _e216.g0_.z, _e219.g0_.z, _e222.g0_.x)) * vec4<f32>(0.0, -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e235.g1_.w) * vec4<f32>(_e239.g0_.y, _e242.g0_.y, _e245.g0_.x, _e248.g0_.y)) * vec4<f32>(0.0, 1.0, -(1.0), 0.0))) + ((_e261.g0_.yxxx * vec4<f32>(_e264.g0_.x, _e267.g0_.x, _e270.g0_.y, _e273.g0_.z)) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0))));
}

fn point_and_plane_ideal_point_inner_anti_product(self_1986: PointAndPlane, other_1640: IdealPoint) -> Point {
    var self_1987: PointAndPlane;
    var other_1641: IdealPoint;

    self_1987 = self_1986;
    other_1641 = other_1640;
    let _e4: PointAndPlane = self_1987;
    let _e8: IdealPoint = other_1641;
    let _e11: IdealPoint = other_1641;
    let _e14: IdealPoint = other_1641;
    let _e17: IdealPoint = other_1641;
    let _e29: PointAndPlane = self_1987;
    let _e33: IdealPoint = other_1641;
    let _e36: IdealPoint = other_1641;
    let _e39: IdealPoint = other_1641;
    let _e42: IdealPoint = other_1641;
    let _e55: PointAndPlane = self_1987;
    let _e59: IdealPoint = other_1641;
    let _e62: IdealPoint = other_1641;
    let _e65: IdealPoint = other_1641;
    let _e68: IdealPoint = other_1641;
    let _e80: PointAndPlane = self_1987;
    let _e84: IdealPoint = other_1641;
    let _e97: PointAndPlane = self_1987;
    let _e101: IdealPoint = other_1641;
    let _e114: PointAndPlane = self_1987;
    let _e117: PointAndPlane = self_1987;
    let _e120: PointAndPlane = self_1987;
    let _e123: PointAndPlane = self_1987;
    let _e127: IdealPoint = other_1641;
    let _e130: IdealPoint = other_1641;
    let _e133: IdealPoint = other_1641;
    let _e136: IdealPoint = other_1641;
    return Point((((((((vec4<f32>(_e4.g0_.z) * vec4<f32>(_e8.g0_.z, _e11.g0_.z, _e14.g0_.z, _e17.g0_.x)) * vec4<f32>(0.0, -(1.0), 0.0, 1.0)) + ((vec4<f32>(_e29.g0_.w) * vec4<f32>(_e33.g0_.y, _e36.g0_.y, _e39.g0_.x, _e42.g0_.y)) * vec4<f32>(0.0, 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e55.g1_.x) * vec4<f32>(_e59.g0_.x, _e62.g0_.x, _e65.g0_.y, _e68.g0_.z)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))) + ((vec4<f32>(_e80.g1_.z) * vec4<f32>(_e84.g0_.y)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((vec4<f32>(_e97.g1_.w) * vec4<f32>(_e101.g0_.z)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((vec4<f32>(_e114.g1_.y, _e117.g0_.x, _e120.g0_.y, _e123.g0_.y) * vec4<f32>(_e127.g0_.x, _e130.g0_.x, _e133.g0_.z, _e136.g0_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0)))));
}

fn point_and_plane_ideal_point_right_anti_contraction(self_1988: PointAndPlane, other_1642: IdealPoint) -> Point {
    var self_1989: PointAndPlane;
    var other_1643: IdealPoint;

    self_1989 = self_1988;
    other_1643 = other_1642;
    let _e4: PointAndPlane = self_1989;
    let _e8: IdealPoint = other_1643;
    let _e20: PointAndPlane = self_1989;
    let _e24: IdealPoint = other_1643;
    let _e37: PointAndPlane = self_1989;
    let _e40: IdealPoint = other_1643;
    let _e43: IdealPoint = other_1643;
    let _e46: IdealPoint = other_1643;
    let _e49: IdealPoint = other_1643;
    return Point(((((vec4<f32>(_e4.g1_.z) * vec4<f32>(_e8.g0_.y)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0)) + ((vec4<f32>(_e20.g1_.w) * vec4<f32>(_e24.g0_.z)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((_e37.g1_.yxxx * vec4<f32>(_e40.g0_.x, _e43.g0_.x, _e46.g0_.y, _e49.g0_.z)) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0))));
}

fn point_and_plane_plane_into(self_1990: PointAndPlane) -> Plane {
    var self_1991: PointAndPlane;

    self_1991 = self_1990;
    let _e2: PointAndPlane = self_1991;
    return Plane(_e2.g1_);
}

fn point_and_plane_plane_add(self_1992: PointAndPlane, other_1644: Plane) -> PointAndPlane {
    var self_1993: PointAndPlane;
    var other_1645: Plane;

    self_1993 = self_1992;
    other_1645 = other_1644;
    let _e4: PointAndPlane = self_1993;
    let _e6: PointAndPlane = self_1993;
    let _e8: Plane = other_1645;
    return PointAndPlane(_e4.g0_, (_e6.g1_ + _e8.g0_));
}

fn point_and_plane_plane_sub(self_1994: PointAndPlane, other_1646: Plane) -> PointAndPlane {
    var self_1995: PointAndPlane;
    var other_1647: Plane;

    self_1995 = self_1994;
    other_1647 = other_1646;
    let _e4: PointAndPlane = self_1995;
    let _e6: PointAndPlane = self_1995;
    let _e8: Plane = other_1647;
    return PointAndPlane(_e4.g0_, (_e6.g1_ - _e8.g0_));
}

fn point_and_plane_plane_geometric_product(self_1996: PointAndPlane, other_1648: Plane) -> Motor {
    var self_1997: PointAndPlane;
    var other_1649: Plane;

    self_1997 = self_1996;
    other_1649 = other_1648;
    let _e4: PointAndPlane = self_1997;
    let _e8: Plane = other_1649;
    let _e19: PointAndPlane = self_1997;
    let _e23: Plane = other_1649;
    let _e35: PointAndPlane = self_1997;
    let _e39: Plane = other_1649;
    let _e51: PointAndPlane = self_1997;
    let _e55: Plane = other_1649;
    let _e65: PointAndPlane = self_1997;
    let _e69: Plane = other_1649;
    let _e81: PointAndPlane = self_1997;
    let _e85: Plane = other_1649;
    let _e98: PointAndPlane = self_1997;
    let _e102: Plane = other_1649;
    let _e115: PointAndPlane = self_1997;
    let _e119: Plane = other_1649;
    let _e132: PointAndPlane = self_1997;
    let _e136: Plane = other_1649;
    let _e149: PointAndPlane = self_1997;
    let _e153: Plane = other_1649;
    let _e166: PointAndPlane = self_1997;
    let _e169: PointAndPlane = self_1997;
    let _e172: PointAndPlane = self_1997;
    let _e175: PointAndPlane = self_1997;
    let _e179: Plane = other_1649;
    return Motor((((((vec4<f32>(_e4.g1_.y) * _e8.g0_.yywz) * vec4<f32>(1.0, 0.0, -(1.0), 1.0)) + ((vec4<f32>(_e19.g1_.z) * _e23.g0_.zwzy) * vec4<f32>(1.0, 1.0, 0.0, -(1.0)))) + ((vec4<f32>(_e35.g1_.w) * _e39.g0_.wzyw) * vec4<f32>(1.0, -(1.0), 1.0, 0.0))) + ((vec4<f32>(_e51.g0_.x) * _e55.g0_) * vec4<f32>(0.0, 1.0, 1.0, 1.0))), ((((((((vec4<f32>(_e65.g0_.y) * _e69.g0_.yywz) * vec4<f32>(-(1.0), 0.0, -(1.0), 1.0)) + ((vec4<f32>(_e81.g0_.z) * _e85.g0_.zwzy) * vec4<f32>(-(1.0), 1.0, 0.0, -(1.0)))) + ((vec4<f32>(_e98.g0_.w) * _e102.g0_.wzyw) * vec4<f32>(-(1.0), -(1.0), 1.0, 0.0))) + ((vec4<f32>(_e115.g1_.y) * vec4<f32>(_e119.g0_.x)) * vec4<f32>(0.0, -(1.0), 0.0, 0.0))) + ((vec4<f32>(_e132.g1_.z) * vec4<f32>(_e136.g0_.x)) * vec4<f32>(0.0, 0.0, -(1.0), 0.0))) + ((vec4<f32>(_e149.g1_.w) * vec4<f32>(_e153.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e166.g0_.x, _e169.g1_.x, _e172.g1_.x, _e175.g1_.x) * _e179.g0_) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0))));
}

fn point_and_plane_plane_regressive_product(self_1998: PointAndPlane, other_1650: Plane) -> Scalar {
    var self_1999: PointAndPlane;
    var other_1651: Plane;

    self_1999 = self_1998;
    other_1651 = other_1650;
    let _e5: PointAndPlane = self_1999;
    let _e8: Plane = other_1651;
    let _e13: PointAndPlane = self_1999;
    let _e16: Plane = other_1651;
    let _e21: PointAndPlane = self_1999;
    let _e24: Plane = other_1651;
    let _e29: PointAndPlane = self_1999;
    let _e32: Plane = other_1651;
    return Scalar(((((0.0 - (_e5.g0_.x * _e8.g0_.x)) - (_e13.g0_.y * _e16.g0_.y)) - (_e21.g0_.z * _e24.g0_.z)) - (_e29.g0_.w * _e32.g0_.w)));
}

fn point_and_plane_plane_geometric_anti_product(self_2000: PointAndPlane, other_1652: Plane) -> Motor {
    var self_2001: PointAndPlane;
    var other_1653: Plane;

    self_2001 = self_2000;
    other_1653 = other_1652;
    let _e4: PointAndPlane = self_2001;
    let _e8: Plane = other_1653;
    let _e20: PointAndPlane = self_2001;
    let _e24: Plane = other_1653;
    let _e37: PointAndPlane = self_2001;
    let _e41: Plane = other_1653;
    let _e54: PointAndPlane = self_2001;
    let _e58: Plane = other_1653;
    let _e70: PointAndPlane = self_2001;
    let _e74: Plane = other_1653;
    let _e86: PointAndPlane = self_2001;
    let _e90: Plane = other_1653;
    let _e102: PointAndPlane = self_2001;
    let _e105: PointAndPlane = self_2001;
    let _e108: PointAndPlane = self_2001;
    let _e111: PointAndPlane = self_2001;
    let _e115: Plane = other_1653;
    let _e123: PointAndPlane = self_2001;
    let _e126: PointAndPlane = self_2001;
    let _e129: PointAndPlane = self_2001;
    let _e132: PointAndPlane = self_2001;
    let _e136: Plane = other_1653;
    return Motor(((((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.yywz) * vec4<f32>(-(1.0), 0.0, -(1.0), 1.0)) + ((vec4<f32>(_e20.g0_.z) * _e24.g0_.zwzy) * vec4<f32>(-(1.0), 1.0, 0.0, -(1.0)))) + ((vec4<f32>(_e37.g0_.w) * _e41.g0_.wzyw) * vec4<f32>(-(1.0), -(1.0), 1.0, 0.0))) + ((vec4<f32>(_e54.g1_.y) * vec4<f32>(_e58.g0_.x)) * vec4<f32>(0.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e70.g1_.z) * vec4<f32>(_e74.g0_.x)) * vec4<f32>(0.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e86.g1_.w) * vec4<f32>(_e90.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e102.g0_.x, _e105.g1_.x, _e108.g1_.x, _e111.g1_.x) * _e115.g0_) * vec4<f32>(-(1.0)))), ((vec4<f32>(_e123.g1_.x, _e126.g0_.y, _e129.g0_.z, _e132.g0_.w) * vec4<f32>(_e136.g0_.x)) * vec4<f32>(-(1.0))));
}

fn point_and_plane_plane_left_contraction(self_2002: PointAndPlane, other_1654: Plane) -> Scalar {
    var self_2003: PointAndPlane;
    var other_1655: Plane;

    self_2003 = self_2002;
    other_1655 = other_1654;
    let _e4: PointAndPlane = self_2003;
    let _e7: Plane = other_1655;
    let _e11: PointAndPlane = self_2003;
    let _e14: Plane = other_1655;
    let _e19: PointAndPlane = self_2003;
    let _e22: Plane = other_1655;
    return Scalar((((_e4.g1_.y * _e7.g0_.y) + (_e11.g1_.z * _e14.g0_.z)) + (_e19.g1_.w * _e22.g0_.w)));
}

fn point_and_plane_plane_right_anti_contraction(self_2004: PointAndPlane, other_1656: Plane) -> AntiScalar {
    var self_2005: PointAndPlane;
    var other_1657: Plane;

    self_2005 = self_2004;
    other_1657 = other_1656;
    let _e5: PointAndPlane = self_2005;
    let _e8: Plane = other_1657;
    return AntiScalar((0.0 - (_e5.g1_.x * _e8.g0_.x)));
}

fn point_and_plane_plane_scalar_product(self_2006: PointAndPlane, other_1658: Plane) -> Scalar {
    var self_2007: PointAndPlane;
    var other_1659: Plane;

    self_2007 = self_2006;
    other_1659 = other_1658;
    let _e4: PointAndPlane = self_2007;
    let _e7: Plane = other_1659;
    let _e11: PointAndPlane = self_2007;
    let _e14: Plane = other_1659;
    let _e19: PointAndPlane = self_2007;
    let _e22: Plane = other_1659;
    return Scalar((((_e4.g1_.y * _e7.g0_.y) + (_e11.g1_.z * _e14.g0_.z)) + (_e19.g1_.w * _e22.g0_.w)));
}

fn point_and_plane_plane_anti_scalar_product(self_2008: PointAndPlane, other_1660: Plane) -> AntiScalar {
    var self_2009: PointAndPlane;
    var other_1661: Plane;

    self_2009 = self_2008;
    other_1661 = other_1660;
    let _e5: PointAndPlane = self_2009;
    let _e8: Plane = other_1661;
    return AntiScalar((0.0 - (_e5.g1_.x * _e8.g0_.x)));
}

fn point_and_plane_line_geometric_product(self_2010: PointAndPlane, other_1662: Line) -> PointAndPlane {
    var self_2011: PointAndPlane;
    var other_1663: Line;

    self_2011 = self_2010;
    other_1663 = other_1662;
    let _e4: PointAndPlane = self_2011;
    let _e8: Line = other_1663;
    let _e11: Line = other_1663;
    let _e14: Line = other_1663;
    let _e17: Line = other_1663;
    let _e29: PointAndPlane = self_2011;
    let _e33: Line = other_1663;
    let _e36: Line = other_1663;
    let _e39: Line = other_1663;
    let _e42: Line = other_1663;
    let _e55: PointAndPlane = self_2011;
    let _e59: Line = other_1663;
    let _e62: Line = other_1663;
    let _e65: Line = other_1663;
    let _e68: Line = other_1663;
    let _e81: PointAndPlane = self_2011;
    let _e85: Line = other_1663;
    let _e88: Line = other_1663;
    let _e91: Line = other_1663;
    let _e94: Line = other_1663;
    let _e109: PointAndPlane = self_2011;
    let _e113: Line = other_1663;
    let _e116: Line = other_1663;
    let _e119: Line = other_1663;
    let _e122: Line = other_1663;
    let _e135: PointAndPlane = self_2011;
    let _e139: Line = other_1663;
    let _e142: Line = other_1663;
    let _e145: Line = other_1663;
    let _e148: Line = other_1663;
    let _e161: PointAndPlane = self_2011;
    let _e165: Line = other_1663;
    let _e168: Line = other_1663;
    let _e171: Line = other_1663;
    let _e174: Line = other_1663;
    let _e187: PointAndPlane = self_2011;
    let _e191: Line = other_1663;
    let _e194: Line = other_1663;
    let _e197: Line = other_1663;
    let _e200: Line = other_1663;
    let _e212: PointAndPlane = self_2011;
    let _e216: Line = other_1663;
    let _e227: PointAndPlane = self_2011;
    let _e231: Line = other_1663;
    let _e243: PointAndPlane = self_2011;
    let _e247: Line = other_1663;
    let _e250: Line = other_1663;
    let _e253: Line = other_1663;
    let _e256: Line = other_1663;
    let _e270: PointAndPlane = self_2011;
    let _e274: Line = other_1663;
    let _e277: Line = other_1663;
    let _e280: Line = other_1663;
    let _e283: Line = other_1663;
    let _e297: PointAndPlane = self_2011;
    let _e301: Line = other_1663;
    let _e304: Line = other_1663;
    let _e307: Line = other_1663;
    let _e310: Line = other_1663;
    let _e324: PointAndPlane = self_2011;
    let _e327: Line = other_1663;
    let _e330: Line = other_1663;
    let _e333: Line = other_1663;
    let _e336: Line = other_1663;
    return PointAndPlane((((((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g1_.z, _e11.g1_.z, _e14.g1_.z, _e17.g1_.y)) * vec4<f32>(0.0, 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e29.g0_.z) * vec4<f32>(_e33.g1_.z, _e36.g1_.z, _e39.g1_.z, _e42.g1_.x)) * vec4<f32>(0.0, -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e55.g0_.w) * vec4<f32>(_e59.g1_.y, _e62.g1_.y, _e65.g1_.x, _e68.g1_.y)) * vec4<f32>(0.0, 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e81.g1_.x) * vec4<f32>(_e85.g1_.x, _e88.g1_.x, _e91.g1_.y, _e94.g1_.z)) * vec4<f32>(0.0, -(1.0), -(1.0), -(1.0)))) + ((vec4<f32>(_e109.g1_.y) * vec4<f32>(_e113.g1_.x, _e116.g1_.x, _e119.g0_.z, _e122.g0_.y)) * vec4<f32>(1.0, 0.0, -(1.0), 1.0))) + ((vec4<f32>(_e135.g1_.z) * vec4<f32>(_e139.g1_.y, _e142.g0_.z, _e145.g1_.y, _e148.g0_.x)) * vec4<f32>(1.0, 1.0, 0.0, -(1.0)))) + ((vec4<f32>(_e161.g1_.w) * vec4<f32>(_e165.g1_.z, _e168.g0_.y, _e171.g0_.x, _e174.g1_.z)) * vec4<f32>(1.0, -(1.0), 1.0, 0.0))) + ((vec4<f32>(_e187.g0_.x) * vec4<f32>(_e191.g0_.x, _e194.g0_.x, _e197.g0_.y, _e200.g0_.z)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))), (((((((vec4<f32>(_e212.g0_.z) * vec4<f32>(_e216.g1_.y)) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) + ((vec4<f32>(_e227.g0_.w) * vec4<f32>(_e231.g1_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e243.g1_.y) * vec4<f32>(_e247.g0_.x, _e250.g0_.x, _e253.g1_.z, _e256.g1_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e270.g1_.z) * vec4<f32>(_e274.g0_.y, _e277.g1_.z, _e280.g0_.y, _e283.g1_.x)) * vec4<f32>(-(1.0), -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e297.g1_.w) * vec4<f32>(_e301.g0_.z, _e304.g1_.y, _e307.g1_.x, _e310.g0_.z)) * vec4<f32>(-(1.0), 1.0, -(1.0), 0.0))) + ((_e324.g0_.yxxx * vec4<f32>(_e327.g1_.x, _e330.g1_.x, _e333.g1_.y, _e336.g1_.z)) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0)))));
}

fn point_and_plane_line_regressive_product(self_2012: PointAndPlane, other_1664: Line) -> Plane {
    var self_2013: PointAndPlane;
    var other_1665: Line;

    self_2013 = self_2012;
    other_1665 = other_1664;
    let _e4: PointAndPlane = self_2013;
    let _e8: Line = other_1665;
    let _e11: Line = other_1665;
    let _e14: Line = other_1665;
    let _e17: Line = other_1665;
    let _e30: PointAndPlane = self_2013;
    let _e34: Line = other_1665;
    let _e37: Line = other_1665;
    let _e40: Line = other_1665;
    let _e43: Line = other_1665;
    let _e57: PointAndPlane = self_2013;
    let _e61: Line = other_1665;
    let _e64: Line = other_1665;
    let _e67: Line = other_1665;
    let _e70: Line = other_1665;
    let _e84: PointAndPlane = self_2013;
    let _e88: Line = other_1665;
    let _e91: Line = other_1665;
    let _e94: Line = other_1665;
    let _e97: Line = other_1665;
    return Plane((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.x, _e11.g0_.x, _e14.g1_.z, _e17.g1_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e30.g0_.z) * vec4<f32>(_e34.g0_.y, _e37.g1_.z, _e40.g0_.y, _e43.g1_.x)) * vec4<f32>(-(1.0), -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e57.g0_.w) * vec4<f32>(_e61.g0_.z, _e64.g1_.y, _e67.g1_.x, _e70.g0_.z)) * vec4<f32>(-(1.0), 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e84.g0_.x) * vec4<f32>(_e88.g0_.x, _e91.g0_.x, _e94.g0_.y, _e97.g0_.z)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn point_and_plane_line_outer_product(self_2014: PointAndPlane, other_1666: Line) -> Point {
    var self_2015: PointAndPlane;
    var other_1667: Line;

    self_2015 = self_2014;
    other_1667 = other_1666;
    let _e4: PointAndPlane = self_2015;
    let _e8: Line = other_1667;
    let _e11: Line = other_1667;
    let _e14: Line = other_1667;
    let _e17: Line = other_1667;
    let _e29: PointAndPlane = self_2015;
    let _e33: Line = other_1667;
    let _e36: Line = other_1667;
    let _e39: Line = other_1667;
    let _e42: Line = other_1667;
    let _e55: PointAndPlane = self_2015;
    let _e59: Line = other_1667;
    let _e62: Line = other_1667;
    let _e65: Line = other_1667;
    let _e68: Line = other_1667;
    let _e81: PointAndPlane = self_2015;
    let _e85: Line = other_1667;
    let _e88: Line = other_1667;
    let _e91: Line = other_1667;
    let _e94: Line = other_1667;
    return Point((((((vec4<f32>(_e4.g1_.y) * vec4<f32>(_e8.g1_.x, _e11.g1_.x, _e14.g0_.z, _e17.g0_.y)) * vec4<f32>(1.0, 0.0, -(1.0), 1.0)) + ((vec4<f32>(_e29.g1_.z) * vec4<f32>(_e33.g1_.y, _e36.g0_.z, _e39.g1_.y, _e42.g0_.x)) * vec4<f32>(1.0, 1.0, 0.0, -(1.0)))) + ((vec4<f32>(_e55.g1_.w) * vec4<f32>(_e59.g1_.z, _e62.g0_.y, _e65.g0_.x, _e68.g1_.z)) * vec4<f32>(1.0, -(1.0), 1.0, 0.0))) + ((vec4<f32>(_e81.g1_.x) * vec4<f32>(_e85.g1_.x, _e88.g1_.x, _e91.g1_.y, _e94.g1_.z)) * vec4<f32>(0.0, -(1.0), -(1.0), -(1.0)))));
}

fn point_and_plane_line_inner_product(self_2016: PointAndPlane, other_1668: Line) -> Plane {
    var self_2017: PointAndPlane;
    var other_1669: Line;

    self_2017 = self_2016;
    other_1669 = other_1668;
    let _e4: PointAndPlane = self_2017;
    let _e8: Line = other_1669;
    let _e19: PointAndPlane = self_2017;
    let _e23: Line = other_1669;
    let _e35: PointAndPlane = self_2017;
    let _e39: Line = other_1669;
    let _e42: Line = other_1669;
    let _e45: Line = other_1669;
    let _e48: Line = other_1669;
    let _e62: PointAndPlane = self_2017;
    let _e66: Line = other_1669;
    let _e69: Line = other_1669;
    let _e72: Line = other_1669;
    let _e75: Line = other_1669;
    let _e89: PointAndPlane = self_2017;
    let _e93: Line = other_1669;
    let _e96: Line = other_1669;
    let _e99: Line = other_1669;
    let _e102: Line = other_1669;
    let _e116: PointAndPlane = self_2017;
    let _e119: Line = other_1669;
    let _e122: Line = other_1669;
    let _e125: Line = other_1669;
    let _e128: Line = other_1669;
    return Plane((((((((vec4<f32>(_e4.g0_.z) * vec4<f32>(_e8.g1_.y)) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) + ((vec4<f32>(_e19.g0_.w) * vec4<f32>(_e23.g1_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e35.g1_.y) * vec4<f32>(_e39.g0_.x, _e42.g0_.x, _e45.g1_.z, _e48.g1_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e62.g1_.z) * vec4<f32>(_e66.g0_.y, _e69.g1_.z, _e72.g0_.y, _e75.g1_.x)) * vec4<f32>(-(1.0), -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e89.g1_.w) * vec4<f32>(_e93.g0_.z, _e96.g1_.y, _e99.g1_.x, _e102.g0_.z)) * vec4<f32>(-(1.0), 1.0, -(1.0), 0.0))) + ((_e116.g0_.yxxx * vec4<f32>(_e119.g1_.x, _e122.g1_.x, _e125.g1_.y, _e128.g1_.z)) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0)))));
}

fn point_and_plane_line_geometric_anti_product(self_2018: PointAndPlane, other_1670: Line) -> PointAndPlane {
    var self_2019: PointAndPlane;
    var other_1671: Line;

    self_2019 = self_2018;
    other_1671 = other_1670;
    let _e4: PointAndPlane = self_2019;
    let _e8: Line = other_1671;
    let _e11: Line = other_1671;
    let _e14: Line = other_1671;
    let _e17: Line = other_1671;
    let _e30: PointAndPlane = self_2019;
    let _e34: Line = other_1671;
    let _e37: Line = other_1671;
    let _e40: Line = other_1671;
    let _e43: Line = other_1671;
    let _e57: PointAndPlane = self_2019;
    let _e61: Line = other_1671;
    let _e64: Line = other_1671;
    let _e67: Line = other_1671;
    let _e70: Line = other_1671;
    let _e82: PointAndPlane = self_2019;
    let _e86: Line = other_1671;
    let _e99: PointAndPlane = self_2019;
    let _e103: Line = other_1671;
    let _e116: PointAndPlane = self_2019;
    let _e120: Line = other_1671;
    let _e133: PointAndPlane = self_2019;
    let _e136: Line = other_1671;
    let _e139: Line = other_1671;
    let _e142: Line = other_1671;
    let _e145: Line = other_1671;
    let _e159: PointAndPlane = self_2019;
    let _e163: Line = other_1671;
    let _e166: Line = other_1671;
    let _e169: Line = other_1671;
    let _e172: Line = other_1671;
    let _e185: PointAndPlane = self_2019;
    let _e189: Line = other_1671;
    let _e192: Line = other_1671;
    let _e195: Line = other_1671;
    let _e198: Line = other_1671;
    let _e212: PointAndPlane = self_2019;
    let _e216: Line = other_1671;
    let _e219: Line = other_1671;
    let _e222: Line = other_1671;
    let _e225: Line = other_1671;
    let _e239: PointAndPlane = self_2019;
    let _e243: Line = other_1671;
    let _e246: Line = other_1671;
    let _e249: Line = other_1671;
    let _e252: Line = other_1671;
    let _e264: PointAndPlane = self_2019;
    let _e268: Line = other_1671;
    let _e271: Line = other_1671;
    let _e274: Line = other_1671;
    let _e277: Line = other_1671;
    let _e290: PointAndPlane = self_2019;
    let _e294: Line = other_1671;
    let _e297: Line = other_1671;
    let _e300: Line = other_1671;
    let _e303: Line = other_1671;
    let _e316: PointAndPlane = self_2019;
    let _e320: Line = other_1671;
    let _e323: Line = other_1671;
    let _e326: Line = other_1671;
    let _e329: Line = other_1671;
    let _e342: PointAndPlane = self_2019;
    let _e346: Line = other_1671;
    let _e349: Line = other_1671;
    let _e352: Line = other_1671;
    let _e355: Line = other_1671;
    return PointAndPlane(((((((((vec4<f32>(_e4.g0_.z) * vec4<f32>(_e8.g1_.y, _e11.g0_.z, _e14.g1_.y, _e17.g0_.x)) * vec4<f32>(-(1.0), -(1.0), 0.0, 1.0)) + ((vec4<f32>(_e30.g0_.w) * vec4<f32>(_e34.g1_.z, _e37.g0_.y, _e40.g0_.x, _e43.g1_.z)) * vec4<f32>(-(1.0), 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e57.g1_.x) * vec4<f32>(_e61.g0_.x, _e64.g0_.x, _e67.g0_.y, _e70.g0_.z)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))) + ((vec4<f32>(_e82.g1_.y) * vec4<f32>(_e86.g0_.x)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((vec4<f32>(_e99.g1_.z) * vec4<f32>(_e103.g0_.y)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((vec4<f32>(_e116.g1_.w) * vec4<f32>(_e120.g0_.z)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((_e133.g0_.yxyy * vec4<f32>(_e136.g1_.x, _e139.g1_.x, _e142.g0_.z, _e145.g0_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0)))), (((((((((vec4<f32>(_e159.g0_.y) * vec4<f32>(_e163.g0_.x, _e166.g0_.x, _e169.g1_.z, _e172.g1_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e185.g0_.z) * vec4<f32>(_e189.g0_.y, _e192.g1_.z, _e195.g0_.y, _e198.g1_.x)) * vec4<f32>(-(1.0), -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e212.g0_.w) * vec4<f32>(_e216.g0_.z, _e219.g1_.y, _e222.g1_.x, _e225.g0_.z)) * vec4<f32>(-(1.0), 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e239.g1_.x) * vec4<f32>(_e243.g1_.x, _e246.g1_.x, _e249.g1_.y, _e252.g1_.z)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))) + ((vec4<f32>(_e264.g1_.y) * vec4<f32>(_e268.g0_.z, _e271.g0_.z, _e274.g0_.z, _e277.g0_.y)) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e290.g1_.z) * vec4<f32>(_e294.g0_.z, _e297.g0_.z, _e300.g0_.z, _e303.g0_.x)) * vec4<f32>(0.0, -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e316.g1_.w) * vec4<f32>(_e320.g0_.y, _e323.g0_.y, _e326.g0_.x, _e329.g0_.y)) * vec4<f32>(0.0, 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e342.g0_.x) * vec4<f32>(_e346.g0_.x, _e349.g0_.x, _e352.g0_.y, _e355.g0_.z)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn point_and_plane_line_inner_anti_product(self_2020: PointAndPlane, other_1672: Line) -> Point {
    var self_2021: PointAndPlane;
    var other_1673: Line;

    self_2021 = self_2020;
    other_1673 = other_1672;
    let _e4: PointAndPlane = self_2021;
    let _e8: Line = other_1673;
    let _e11: Line = other_1673;
    let _e14: Line = other_1673;
    let _e17: Line = other_1673;
    let _e30: PointAndPlane = self_2021;
    let _e34: Line = other_1673;
    let _e37: Line = other_1673;
    let _e40: Line = other_1673;
    let _e43: Line = other_1673;
    let _e57: PointAndPlane = self_2021;
    let _e61: Line = other_1673;
    let _e64: Line = other_1673;
    let _e67: Line = other_1673;
    let _e70: Line = other_1673;
    let _e82: PointAndPlane = self_2021;
    let _e86: Line = other_1673;
    let _e99: PointAndPlane = self_2021;
    let _e103: Line = other_1673;
    let _e116: PointAndPlane = self_2021;
    let _e120: Line = other_1673;
    let _e133: PointAndPlane = self_2021;
    let _e136: Line = other_1673;
    let _e139: Line = other_1673;
    let _e142: Line = other_1673;
    let _e145: Line = other_1673;
    return Point(((((((((vec4<f32>(_e4.g0_.z) * vec4<f32>(_e8.g1_.y, _e11.g0_.z, _e14.g1_.y, _e17.g0_.x)) * vec4<f32>(-(1.0), -(1.0), 0.0, 1.0)) + ((vec4<f32>(_e30.g0_.w) * vec4<f32>(_e34.g1_.z, _e37.g0_.y, _e40.g0_.x, _e43.g1_.z)) * vec4<f32>(-(1.0), 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e57.g1_.x) * vec4<f32>(_e61.g0_.x, _e64.g0_.x, _e67.g0_.y, _e70.g0_.z)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))) + ((vec4<f32>(_e82.g1_.y) * vec4<f32>(_e86.g0_.x)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((vec4<f32>(_e99.g1_.z) * vec4<f32>(_e103.g0_.y)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((vec4<f32>(_e116.g1_.w) * vec4<f32>(_e120.g0_.z)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((_e133.g0_.yxyy * vec4<f32>(_e136.g1_.x, _e139.g1_.x, _e142.g0_.z, _e145.g0_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0)))));
}

fn point_and_plane_line_left_contraction(self_2022: PointAndPlane, other_1674: Line) -> Plane {
    var self_2023: PointAndPlane;
    var other_1675: Line;

    self_2023 = self_2022;
    other_1675 = other_1674;
    let _e4: PointAndPlane = self_2023;
    let _e8: Line = other_1675;
    let _e11: Line = other_1675;
    let _e14: Line = other_1675;
    let _e17: Line = other_1675;
    let _e30: PointAndPlane = self_2023;
    let _e34: Line = other_1675;
    let _e37: Line = other_1675;
    let _e40: Line = other_1675;
    let _e43: Line = other_1675;
    let _e57: PointAndPlane = self_2023;
    let _e60: Line = other_1675;
    let _e63: Line = other_1675;
    let _e66: Line = other_1675;
    let _e69: Line = other_1675;
    return Plane(((((vec4<f32>(_e4.g1_.z) * vec4<f32>(_e8.g0_.y, _e11.g1_.z, _e14.g0_.y, _e17.g1_.x)) * vec4<f32>(-(1.0), -(1.0), 0.0, 1.0)) + ((vec4<f32>(_e30.g1_.w) * vec4<f32>(_e34.g0_.z, _e37.g1_.y, _e40.g1_.x, _e43.g0_.z)) * vec4<f32>(-(1.0), 1.0, -(1.0), 0.0))) + ((_e57.g1_.yxyy * vec4<f32>(_e60.g0_.x, _e63.g0_.x, _e66.g1_.z, _e69.g1_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0)))));
}

fn point_and_plane_line_right_contraction(self_2024: PointAndPlane, other_1676: Line) -> Plane {
    var self_2025: PointAndPlane;
    var other_1677: Line;

    self_2025 = self_2024;
    other_1677 = other_1676;
    let _e4: PointAndPlane = self_2025;
    let _e8: Line = other_1677;
    let _e19: PointAndPlane = self_2025;
    let _e23: Line = other_1677;
    let _e35: PointAndPlane = self_2025;
    let _e38: Line = other_1677;
    let _e41: Line = other_1677;
    let _e44: Line = other_1677;
    let _e47: Line = other_1677;
    return Plane(((((vec4<f32>(_e4.g0_.z) * vec4<f32>(_e8.g1_.y)) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) + ((vec4<f32>(_e19.g0_.w) * vec4<f32>(_e23.g1_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((_e35.g0_.yxxx * vec4<f32>(_e38.g1_.x, _e41.g1_.x, _e44.g1_.y, _e47.g1_.z)) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0)))));
}

fn point_and_plane_line_left_anti_contraction(self_2026: PointAndPlane, other_1678: Line) -> Point {
    var self_2027: PointAndPlane;
    var other_1679: Line;

    self_2027 = self_2026;
    other_1679 = other_1678;
    let _e4: PointAndPlane = self_2027;
    let _e8: Line = other_1679;
    let _e11: Line = other_1679;
    let _e14: Line = other_1679;
    let _e17: Line = other_1679;
    let _e30: PointAndPlane = self_2027;
    let _e34: Line = other_1679;
    let _e37: Line = other_1679;
    let _e40: Line = other_1679;
    let _e43: Line = other_1679;
    let _e57: PointAndPlane = self_2027;
    let _e60: Line = other_1679;
    let _e63: Line = other_1679;
    let _e66: Line = other_1679;
    let _e69: Line = other_1679;
    return Point(((((vec4<f32>(_e4.g0_.z) * vec4<f32>(_e8.g1_.y, _e11.g0_.z, _e14.g1_.y, _e17.g0_.x)) * vec4<f32>(-(1.0), -(1.0), 0.0, 1.0)) + ((vec4<f32>(_e30.g0_.w) * vec4<f32>(_e34.g1_.z, _e37.g0_.y, _e40.g0_.x, _e43.g1_.z)) * vec4<f32>(-(1.0), 1.0, -(1.0), 0.0))) + ((_e57.g0_.yxyy * vec4<f32>(_e60.g1_.x, _e63.g1_.x, _e66.g0_.z, _e69.g0_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0)))));
}

fn point_and_plane_line_right_anti_contraction(self_2028: PointAndPlane, other_1680: Line) -> Point {
    var self_2029: PointAndPlane;
    var other_1681: Line;

    self_2029 = self_2028;
    other_1681 = other_1680;
    let _e4: PointAndPlane = self_2029;
    let _e8: Line = other_1681;
    let _e20: PointAndPlane = self_2029;
    let _e24: Line = other_1681;
    let _e37: PointAndPlane = self_2029;
    let _e40: Line = other_1681;
    let _e43: Line = other_1681;
    let _e46: Line = other_1681;
    let _e49: Line = other_1681;
    return Point(((((vec4<f32>(_e4.g1_.z) * vec4<f32>(_e8.g0_.y)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0)) + ((vec4<f32>(_e20.g1_.w) * vec4<f32>(_e24.g0_.z)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((_e37.g1_.yxxx * vec4<f32>(_e40.g0_.x, _e43.g0_.x, _e46.g0_.y, _e49.g0_.z)) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0))));
}

fn point_and_plane_translator_geometric_product(self_2030: PointAndPlane, other_1682: Translator) -> PointAndPlane {
    var self_2031: PointAndPlane;
    var other_1683: Translator;

    self_2031 = self_2030;
    other_1683 = other_1682;
    let _e4: PointAndPlane = self_2031;
    let _e8: Translator = other_1683;
    let _e11: PointAndPlane = self_2031;
    let _e15: Translator = other_1683;
    let _e27: PointAndPlane = self_2031;
    let _e31: Translator = other_1683;
    let _e43: PointAndPlane = self_2031;
    let _e47: Translator = other_1683;
    let _e59: PointAndPlane = self_2031;
    let _e61: Translator = other_1683;
    let _e73: PointAndPlane = self_2031;
    let _e77: Translator = other_1683;
    let _e88: PointAndPlane = self_2031;
    let _e92: Translator = other_1683;
    let _e104: PointAndPlane = self_2031;
    let _e108: Translator = other_1683;
    let _e120: PointAndPlane = self_2031;
    let _e124: Translator = other_1683;
    return PointAndPlane((((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g1_.y) * _e15.g0_.wwwz) * vec4<f32>(0.0, 0.0, -(1.0), 1.0))) + ((vec4<f32>(_e27.g1_.z) * _e31.g0_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0)))) + ((vec4<f32>(_e43.g1_.w) * _e47.g0_.zzyz) * vec4<f32>(0.0, -(1.0), 1.0, 0.0))) + ((_e59.g0_ * vec4<f32>(_e61.g0_.x)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))), (((((vec4<f32>(_e73.g1_.y) * _e77.g0_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0)) + ((vec4<f32>(_e88.g1_.z) * _e92.g0_.zzxz) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0))) + ((vec4<f32>(_e104.g1_.w) * _e108.g0_.wwwx) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))) + ((vec4<f32>(_e120.g1_.x) * vec4<f32>(_e124.g0_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))));
}

fn point_and_plane_translator_regressive_product(self_2032: PointAndPlane, other_1684: Translator) -> Plane {
    var self_2033: PointAndPlane;
    var other_1685: Translator;

    self_2033 = self_2032;
    other_1685 = other_1684;
    let _e4: PointAndPlane = self_2033;
    let _e8: Translator = other_1685;
    let _e20: PointAndPlane = self_2033;
    let _e24: Translator = other_1685;
    let _e37: PointAndPlane = self_2033;
    let _e40: Translator = other_1685;
    return Plane(((((vec4<f32>(_e4.g0_.z) * vec4<f32>(_e8.g0_.z)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0)) + ((vec4<f32>(_e20.g0_.w) * vec4<f32>(_e24.g0_.w)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((_e37.g0_.yxxx * _e40.g0_.yyzw) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0))));
}

fn point_and_plane_translator_outer_product(self_2034: PointAndPlane, other_1686: Translator) -> PointAndPlane {
    var self_2035: PointAndPlane;
    var other_1687: Translator;

    self_2035 = self_2034;
    other_1687 = other_1686;
    let _e4: PointAndPlane = self_2035;
    let _e8: Translator = other_1687;
    let _e19: PointAndPlane = self_2035;
    let _e23: Translator = other_1687;
    let _e35: PointAndPlane = self_2035;
    let _e39: Translator = other_1687;
    let _e51: PointAndPlane = self_2035;
    let _e53: Translator = other_1687;
    let _e59: PointAndPlane = self_2035;
    let _e61: Translator = other_1687;
    return PointAndPlane((((((vec4<f32>(_e4.g1_.y) * _e8.g0_.wwwz) * vec4<f32>(0.0, 0.0, -(1.0), 1.0)) + ((vec4<f32>(_e19.g1_.z) * _e23.g0_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0)))) + ((vec4<f32>(_e35.g1_.w) * _e39.g0_.zzyz) * vec4<f32>(0.0, -(1.0), 1.0, 0.0))) + (_e51.g0_ * vec4<f32>(_e53.g0_.x))), (_e59.g1_ * vec4<f32>(_e61.g0_.x)));
}

fn point_and_plane_translator_inner_product(self_2036: PointAndPlane, other_1688: Translator) -> PointAndPlane {
    var self_2037: PointAndPlane;
    var other_1689: Translator;

    self_2037 = self_2036;
    other_1689 = other_1688;
    let _e4: PointAndPlane = self_2037;
    let _e6: Translator = other_1689;
    let _e11: PointAndPlane = self_2037;
    let _e15: Translator = other_1689;
    let _e26: PointAndPlane = self_2037;
    let _e30: Translator = other_1689;
    let _e42: PointAndPlane = self_2037;
    let _e46: Translator = other_1689;
    let _e58: PointAndPlane = self_2037;
    let _e62: Translator = other_1689;
    return PointAndPlane((_e4.g0_ * vec4<f32>(_e6.g0_.x)), (((((vec4<f32>(_e11.g1_.y) * _e15.g0_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0)) + ((vec4<f32>(_e26.g1_.z) * _e30.g0_.zzxz) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0))) + ((vec4<f32>(_e42.g1_.w) * _e46.g0_.wwwx) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))) + ((vec4<f32>(_e58.g1_.x) * vec4<f32>(_e62.g0_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))));
}

fn point_and_plane_translator_geometric_anti_product(self_2038: PointAndPlane, other_1690: Translator) -> PointAndPlane {
    var self_2039: PointAndPlane;
    var other_1691: Translator;

    self_2039 = self_2038;
    other_1691 = other_1690;
    let _e4: PointAndPlane = self_2039;
    let _e8: Translator = other_1691;
    let _e19: PointAndPlane = self_2039;
    let _e23: Translator = other_1691;
    let _e35: PointAndPlane = self_2039;
    let _e39: Translator = other_1691;
    let _e50: PointAndPlane = self_2039;
    let _e54: Translator = other_1691;
    let _e67: PointAndPlane = self_2039;
    let _e71: Translator = other_1691;
    let _e84: PointAndPlane = self_2039;
    let _e87: PointAndPlane = self_2039;
    let _e90: PointAndPlane = self_2039;
    let _e93: PointAndPlane = self_2039;
    let _e97: Translator = other_1691;
    let _e110: PointAndPlane = self_2039;
    let _e114: Translator = other_1691;
    let _e126: PointAndPlane = self_2039;
    let _e130: Translator = other_1691;
    let _e143: PointAndPlane = self_2039;
    let _e147: Translator = other_1691;
    let _e160: PointAndPlane = self_2039;
    let _e164: Translator = other_1691;
    let _e176: PointAndPlane = self_2039;
    let _e180: Translator = other_1691;
    let _e192: PointAndPlane = self_2039;
    let _e196: Translator = other_1691;
    let _e208: PointAndPlane = self_2039;
    let _e212: Translator = other_1691;
    return PointAndPlane((((((((vec4<f32>(_e4.g0_.z) * _e8.g0_.wwwy) * vec4<f32>(0.0, -(1.0), 0.0, 1.0)) + ((vec4<f32>(_e19.g0_.w) * _e23.g0_.zzyz) * vec4<f32>(0.0, 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e35.g1_.x) * _e39.g0_) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0))) + ((vec4<f32>(_e50.g1_.z) * vec4<f32>(_e54.g0_.z)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((vec4<f32>(_e67.g1_.w) * vec4<f32>(_e71.g0_.w)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((vec4<f32>(_e84.g1_.y, _e87.g0_.x, _e90.g0_.y, _e93.g0_.y) * _e97.g0_.yxwz) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0)))), ((((((((vec4<f32>(_e110.g0_.y) * _e114.g0_.yxyy) * vec4<f32>(-(1.0), -(1.0), 0.0, 0.0)) + ((vec4<f32>(_e126.g0_.z) * _e130.g0_.zzxz) * vec4<f32>(-(1.0), 0.0, -(1.0), 0.0))) + ((vec4<f32>(_e143.g0_.w) * _e147.g0_.wwwx) * vec4<f32>(-(1.0), 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e160.g1_.y) * _e164.g0_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e176.g1_.z) * _e180.g0_.wwwy) * vec4<f32>(0.0, -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e192.g1_.w) * _e196.g0_.zzyz) * vec4<f32>(0.0, 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e208.g0_.x) * _e212.g0_) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn point_and_plane_translator_right_contraction(self_2040: PointAndPlane, other_1692: Translator) -> PointAndPlane {
    var self_2041: PointAndPlane;
    var other_1693: Translator;

    self_2041 = self_2040;
    other_1693 = other_1692;
    let _e4: PointAndPlane = self_2041;
    let _e6: Translator = other_1693;
    let _e11: PointAndPlane = self_2041;
    let _e13: Translator = other_1693;
    return PointAndPlane((_e4.g0_ * vec4<f32>(_e6.g0_.x)), (_e11.g1_ * vec4<f32>(_e13.g0_.x)));
}

fn point_and_plane_translator_right_anti_contraction(self_2042: PointAndPlane, other_1694: Translator) -> Point {
    var self_2043: PointAndPlane;
    var other_1695: Translator;

    self_2043 = self_2042;
    other_1695 = other_1694;
    let _e4: PointAndPlane = self_2043;
    let _e8: Translator = other_1695;
    let _e20: PointAndPlane = self_2043;
    let _e24: Translator = other_1695;
    let _e37: PointAndPlane = self_2043;
    let _e40: Translator = other_1695;
    return Point(((((vec4<f32>(_e4.g1_.z) * vec4<f32>(_e8.g0_.z)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0)) + ((vec4<f32>(_e20.g1_.w) * vec4<f32>(_e24.g0_.w)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((_e37.g1_.yxxx * _e40.g0_.yyzw) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0))));
}

fn point_and_plane_motor_add(self_2044: PointAndPlane, other_1696: Motor) -> MultiVector {
    var self_2045: PointAndPlane;
    var other_1697: Motor;

    self_2045 = self_2044;
    other_1697 = other_1696;
    let _e4: Motor = other_1697;
    let _e6: PointAndPlane = self_2045;
    let _e9: PointAndPlane = self_2045;
    let _e12: PointAndPlane = self_2045;
    let _e15: PointAndPlane = self_2045;
    let _e19: PointAndPlane = self_2045;
    let _e22: PointAndPlane = self_2045;
    let _e25: PointAndPlane = self_2045;
    let _e28: PointAndPlane = self_2045;
    let _e32: Motor = other_1697;
    return MultiVector(_e4.g0_, vec4<f32>(_e6.g1_.x, _e9.g0_.y, _e12.g0_.z, _e15.g0_.w), vec4<f32>(_e19.g0_.x, _e22.g1_.y, _e25.g1_.z, _e28.g1_.w), _e32.g1_);
}

fn point_and_plane_motor_sub(self_2046: PointAndPlane, other_1698: Motor) -> MultiVector {
    var self_2047: PointAndPlane;
    var other_1699: Motor;

    self_2047 = self_2046;
    other_1699 = other_1698;
    let _e6: Motor = other_1699;
    let _e9: PointAndPlane = self_2047;
    let _e12: PointAndPlane = self_2047;
    let _e15: PointAndPlane = self_2047;
    let _e18: PointAndPlane = self_2047;
    let _e22: PointAndPlane = self_2047;
    let _e25: PointAndPlane = self_2047;
    let _e28: PointAndPlane = self_2047;
    let _e31: PointAndPlane = self_2047;
    let _e37: Motor = other_1699;
    return MultiVector((vec4<f32>(0.0) - _e6.g0_), vec4<f32>(_e9.g1_.x, _e12.g0_.y, _e15.g0_.z, _e18.g0_.w), vec4<f32>(_e22.g0_.x, _e25.g1_.y, _e28.g1_.z, _e31.g1_.w), (vec4<f32>(0.0) - _e37.g1_));
}

fn point_and_plane_motor_geometric_product(self_2048: PointAndPlane, other_1700: Motor) -> PointAndPlane {
    var self_2049: PointAndPlane;
    var other_1701: Motor;

    self_2049 = self_2048;
    other_1701 = other_1700;
    let _e4: PointAndPlane = self_2049;
    let _e8: Motor = other_1701;
    let _e11: Motor = other_1701;
    let _e14: Motor = other_1701;
    let _e17: Motor = other_1701;
    let _e22: PointAndPlane = self_2049;
    let _e26: Motor = other_1701;
    let _e38: PointAndPlane = self_2049;
    let _e42: Motor = other_1701;
    let _e54: PointAndPlane = self_2049;
    let _e58: Motor = other_1701;
    let _e72: PointAndPlane = self_2049;
    let _e76: Motor = other_1701;
    let _e79: Motor = other_1701;
    let _e82: Motor = other_1701;
    let _e85: Motor = other_1701;
    let _e98: PointAndPlane = self_2049;
    let _e102: Motor = other_1701;
    let _e105: Motor = other_1701;
    let _e108: Motor = other_1701;
    let _e111: Motor = other_1701;
    let _e124: PointAndPlane = self_2049;
    let _e128: Motor = other_1701;
    let _e131: Motor = other_1701;
    let _e134: Motor = other_1701;
    let _e137: Motor = other_1701;
    let _e150: PointAndPlane = self_2049;
    let _e153: Motor = other_1701;
    let _e165: PointAndPlane = self_2049;
    let _e169: Motor = other_1701;
    let _e172: Motor = other_1701;
    let _e175: Motor = other_1701;
    let _e178: Motor = other_1701;
    let _e192: PointAndPlane = self_2049;
    let _e196: Motor = other_1701;
    let _e208: PointAndPlane = self_2049;
    let _e212: Motor = other_1701;
    let _e224: PointAndPlane = self_2049;
    let _e228: Motor = other_1701;
    let _e240: PointAndPlane = self_2049;
    let _e244: Motor = other_1701;
    let _e247: Motor = other_1701;
    let _e250: Motor = other_1701;
    let _e253: Motor = other_1701;
    let _e267: PointAndPlane = self_2049;
    let _e271: Motor = other_1701;
    let _e274: Motor = other_1701;
    let _e277: Motor = other_1701;
    let _e280: Motor = other_1701;
    let _e294: PointAndPlane = self_2049;
    let _e298: Motor = other_1701;
    let _e301: Motor = other_1701;
    let _e304: Motor = other_1701;
    let _e307: Motor = other_1701;
    let _e321: PointAndPlane = self_2049;
    let _e324: Motor = other_1701;
    return PointAndPlane(((((((((vec4<f32>(_e4.g0_.x) * vec4<f32>(_e8.g0_.x, _e11.g1_.y, _e14.g1_.z, _e17.g1_.w)) + ((vec4<f32>(_e22.g0_.z) * _e26.g0_.wwxy) * vec4<f32>(0.0, -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e38.g0_.w) * _e42.g0_.zzyx) * vec4<f32>(0.0, 1.0, -(1.0), 1.0))) + ((vec4<f32>(_e54.g1_.x) * _e58.g0_.yyzw) * vec4<f32>(0.0, -(1.0), -(1.0), -(1.0)))) + ((vec4<f32>(_e72.g1_.y) * vec4<f32>(_e76.g0_.y, _e79.g1_.x, _e82.g1_.w, _e85.g1_.z)) * vec4<f32>(1.0, 1.0, -(1.0), 1.0))) + ((vec4<f32>(_e98.g1_.z) * vec4<f32>(_e102.g0_.z, _e105.g1_.w, _e108.g1_.x, _e111.g1_.y)) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e124.g1_.w) * vec4<f32>(_e128.g0_.w, _e131.g1_.z, _e134.g1_.y, _e137.g1_.x)) * vec4<f32>(1.0, -(1.0), 1.0, 1.0))) + ((_e150.g0_.xyyy * _e153.g0_.xxwz) * vec4<f32>(0.0, 1.0, 1.0, -(1.0)))), (((((((((vec4<f32>(_e165.g0_.x) * vec4<f32>(_e169.g1_.x, _e172.g0_.y, _e175.g0_.z, _e178.g0_.w)) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))) + ((vec4<f32>(_e192.g0_.z) * vec4<f32>(_e196.g0_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e208.g0_.w) * vec4<f32>(_e212.g0_.w)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e224.g1_.x) * vec4<f32>(_e228.g0_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e240.g1_.y) * vec4<f32>(_e244.g1_.y, _e247.g0_.x, _e250.g0_.w, _e253.g0_.z)) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e267.g1_.z) * vec4<f32>(_e271.g1_.z, _e274.g0_.w, _e277.g0_.x, _e280.g0_.y)) * vec4<f32>(-(1.0), -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e294.g1_.w) * vec4<f32>(_e298.g1_.w, _e301.g0_.z, _e304.g0_.y, _e307.g0_.x)) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))) + ((_e321.g0_.yxxx * _e324.g0_.yxxx) * vec4<f32>(1.0, 0.0, 0.0, 0.0))));
}

fn point_and_plane_motor_regressive_product(self_2050: PointAndPlane, other_1702: Motor) -> PointAndPlane {
    var self_2051: PointAndPlane;
    var other_1703: Motor;

    self_2051 = self_2050;
    other_1703 = other_1702;
    let _e4: PointAndPlane = self_2051;
    let _e6: Motor = other_1703;
    let _e11: PointAndPlane = self_2051;
    let _e15: Motor = other_1703;
    let _e18: Motor = other_1703;
    let _e21: Motor = other_1703;
    let _e24: Motor = other_1703;
    let _e37: PointAndPlane = self_2051;
    let _e41: Motor = other_1703;
    let _e44: Motor = other_1703;
    let _e47: Motor = other_1703;
    let _e50: Motor = other_1703;
    let _e64: PointAndPlane = self_2051;
    let _e68: Motor = other_1703;
    let _e71: Motor = other_1703;
    let _e74: Motor = other_1703;
    let _e77: Motor = other_1703;
    let _e91: PointAndPlane = self_2051;
    let _e95: Motor = other_1703;
    let _e107: PointAndPlane = self_2051;
    let _e111: Motor = other_1703;
    let _e123: PointAndPlane = self_2051;
    let _e127: Motor = other_1703;
    let _e139: PointAndPlane = self_2051;
    let _e142: PointAndPlane = self_2051;
    let _e145: PointAndPlane = self_2051;
    let _e148: PointAndPlane = self_2051;
    let _e152: Motor = other_1703;
    return PointAndPlane((_e4.g0_ * vec4<f32>(_e6.g1_.x)), ((((((((vec4<f32>(_e11.g0_.y) * vec4<f32>(_e15.g1_.y, _e18.g1_.y, _e21.g0_.w, _e24.g0_.z)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e37.g0_.z) * vec4<f32>(_e41.g1_.z, _e44.g0_.w, _e47.g1_.z, _e50.g0_.y)) * vec4<f32>(-(1.0), -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e64.g0_.w) * vec4<f32>(_e68.g1_.w, _e71.g0_.z, _e74.g0_.y, _e77.g1_.w)) * vec4<f32>(-(1.0), 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e91.g1_.y) * vec4<f32>(_e95.g1_.x)) * vec4<f32>(0.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e107.g1_.z) * vec4<f32>(_e111.g1_.x)) * vec4<f32>(0.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e123.g1_.w) * vec4<f32>(_e127.g1_.x)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + (vec4<f32>(_e139.g1_.x, _e142.g0_.x, _e145.g0_.x, _e148.g0_.x) * _e152.g1_)));
}

fn point_and_plane_motor_outer_product(self_2052: PointAndPlane, other_1704: Motor) -> PointAndPlane {
    var self_2053: PointAndPlane;
    var other_1705: Motor;

    self_2053 = self_2052;
    other_1705 = other_1704;
    let _e4: PointAndPlane = self_2053;
    let _e8: Motor = other_1705;
    let _e21: PointAndPlane = self_2053;
    let _e25: Motor = other_1705;
    let _e28: Motor = other_1705;
    let _e31: Motor = other_1705;
    let _e34: Motor = other_1705;
    let _e47: PointAndPlane = self_2053;
    let _e51: Motor = other_1705;
    let _e54: Motor = other_1705;
    let _e57: Motor = other_1705;
    let _e60: Motor = other_1705;
    let _e73: PointAndPlane = self_2053;
    let _e77: Motor = other_1705;
    let _e80: Motor = other_1705;
    let _e83: Motor = other_1705;
    let _e86: Motor = other_1705;
    let _e99: PointAndPlane = self_2053;
    let _e101: Motor = other_1705;
    let _e107: PointAndPlane = self_2053;
    let _e109: Motor = other_1705;
    return PointAndPlane(((((((vec4<f32>(_e4.g1_.x) * _e8.g0_.yyzw) * vec4<f32>(0.0, -(1.0), -(1.0), -(1.0))) + ((vec4<f32>(_e21.g1_.y) * vec4<f32>(_e25.g0_.y, _e28.g0_.y, _e31.g1_.w, _e34.g1_.z)) * vec4<f32>(1.0, 0.0, -(1.0), 1.0))) + ((vec4<f32>(_e47.g1_.z) * vec4<f32>(_e51.g0_.z, _e54.g1_.w, _e57.g0_.z, _e60.g1_.y)) * vec4<f32>(1.0, 1.0, 0.0, -(1.0)))) + ((vec4<f32>(_e73.g1_.w) * vec4<f32>(_e77.g0_.w, _e80.g1_.z, _e83.g1_.y, _e86.g0_.w)) * vec4<f32>(1.0, -(1.0), 1.0, 0.0))) + (_e99.g0_ * vec4<f32>(_e101.g0_.x))), (_e107.g1_ * vec4<f32>(_e109.g0_.x)));
}

fn point_and_plane_motor_inner_product(self_2054: PointAndPlane, other_1706: Motor) -> PointAndPlane {
    var self_2055: PointAndPlane;
    var other_1707: Motor;

    self_2055 = self_2054;
    other_1707 = other_1706;
    let _e4: PointAndPlane = self_2055;
    let _e8: Motor = other_1707;
    let _e19: PointAndPlane = self_2055;
    let _e23: Motor = other_1707;
    let _e35: PointAndPlane = self_2055;
    let _e39: Motor = other_1707;
    let _e51: PointAndPlane = self_2055;
    let _e53: Motor = other_1707;
    let _e59: PointAndPlane = self_2055;
    let _e63: Motor = other_1707;
    let _e66: Motor = other_1707;
    let _e69: Motor = other_1707;
    let _e72: Motor = other_1707;
    let _e86: PointAndPlane = self_2055;
    let _e90: Motor = other_1707;
    let _e102: PointAndPlane = self_2055;
    let _e106: Motor = other_1707;
    let _e118: PointAndPlane = self_2055;
    let _e122: Motor = other_1707;
    let _e134: PointAndPlane = self_2055;
    let _e138: Motor = other_1707;
    let _e141: Motor = other_1707;
    let _e144: Motor = other_1707;
    let _e147: Motor = other_1707;
    let _e161: PointAndPlane = self_2055;
    let _e165: Motor = other_1707;
    let _e168: Motor = other_1707;
    let _e171: Motor = other_1707;
    let _e174: Motor = other_1707;
    let _e188: PointAndPlane = self_2055;
    let _e192: Motor = other_1707;
    let _e195: Motor = other_1707;
    let _e198: Motor = other_1707;
    let _e201: Motor = other_1707;
    let _e215: PointAndPlane = self_2055;
    let _e218: Motor = other_1707;
    return PointAndPlane((((((vec4<f32>(_e4.g1_.y) * vec4<f32>(_e8.g1_.x)) * vec4<f32>(0.0, 1.0, 0.0, 0.0)) + ((vec4<f32>(_e19.g1_.z) * vec4<f32>(_e23.g1_.x)) * vec4<f32>(0.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e35.g1_.w) * vec4<f32>(_e39.g1_.x)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + (_e51.g0_ * vec4<f32>(_e53.g0_.x))), (((((((((vec4<f32>(_e59.g0_.x) * vec4<f32>(_e63.g1_.x, _e66.g0_.y, _e69.g0_.z, _e72.g0_.w)) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))) + ((vec4<f32>(_e86.g0_.z) * vec4<f32>(_e90.g0_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e102.g0_.w) * vec4<f32>(_e106.g0_.w)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e118.g1_.x) * vec4<f32>(_e122.g0_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e134.g1_.y) * vec4<f32>(_e138.g1_.y, _e141.g0_.x, _e144.g0_.w, _e147.g0_.z)) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e161.g1_.z) * vec4<f32>(_e165.g1_.z, _e168.g0_.w, _e171.g0_.x, _e174.g0_.y)) * vec4<f32>(-(1.0), -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e188.g1_.w) * vec4<f32>(_e192.g1_.w, _e195.g0_.z, _e198.g0_.y, _e201.g0_.x)) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))) + ((_e215.g0_.yxxx * _e218.g0_.yxxx) * vec4<f32>(1.0, 0.0, 0.0, 0.0))));
}

fn point_and_plane_motor_geometric_anti_product(self_2056: PointAndPlane, other_1708: Motor) -> PointAndPlane {
    var self_2057: PointAndPlane;
    var other_1709: Motor;

    self_2057 = self_2056;
    other_1709 = other_1708;
    let _e4: PointAndPlane = self_2057;
    let _e8: Motor = other_1709;
    let _e11: Motor = other_1709;
    let _e14: Motor = other_1709;
    let _e17: Motor = other_1709;
    let _e30: PointAndPlane = self_2057;
    let _e34: Motor = other_1709;
    let _e37: Motor = other_1709;
    let _e40: Motor = other_1709;
    let _e43: Motor = other_1709;
    let _e57: PointAndPlane = self_2057;
    let _e61: Motor = other_1709;
    let _e64: Motor = other_1709;
    let _e67: Motor = other_1709;
    let _e70: Motor = other_1709;
    let _e84: PointAndPlane = self_2057;
    let _e88: Motor = other_1709;
    let _e91: Motor = other_1709;
    let _e94: Motor = other_1709;
    let _e97: Motor = other_1709;
    let _e110: PointAndPlane = self_2057;
    let _e114: Motor = other_1709;
    let _e127: PointAndPlane = self_2057;
    let _e131: Motor = other_1709;
    let _e144: PointAndPlane = self_2057;
    let _e148: Motor = other_1709;
    let _e161: PointAndPlane = self_2057;
    let _e165: Motor = other_1709;
    let _e177: PointAndPlane = self_2057;
    let _e181: Motor = other_1709;
    let _e184: Motor = other_1709;
    let _e187: Motor = other_1709;
    let _e190: Motor = other_1709;
    let _e204: PointAndPlane = self_2057;
    let _e208: Motor = other_1709;
    let _e211: Motor = other_1709;
    let _e214: Motor = other_1709;
    let _e217: Motor = other_1709;
    let _e232: PointAndPlane = self_2057;
    let _e236: Motor = other_1709;
    let _e239: Motor = other_1709;
    let _e242: Motor = other_1709;
    let _e245: Motor = other_1709;
    let _e260: PointAndPlane = self_2057;
    let _e264: Motor = other_1709;
    let _e267: Motor = other_1709;
    let _e270: Motor = other_1709;
    let _e273: Motor = other_1709;
    let _e279: PointAndPlane = self_2057;
    let _e283: Motor = other_1709;
    let _e295: PointAndPlane = self_2057;
    let _e299: Motor = other_1709;
    let _e311: PointAndPlane = self_2057;
    let _e315: Motor = other_1709;
    let _e327: PointAndPlane = self_2057;
    let _e331: Motor = other_1709;
    return PointAndPlane((((((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y, _e11.g1_.x, _e14.g1_.w, _e17.g1_.z)) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0))) + ((vec4<f32>(_e30.g0_.z) * vec4<f32>(_e34.g0_.z, _e37.g1_.w, _e40.g1_.x, _e43.g1_.y)) * vec4<f32>(-(1.0), -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e57.g0_.w) * vec4<f32>(_e61.g0_.w, _e64.g1_.z, _e67.g1_.y, _e70.g1_.x)) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))) + ((vec4<f32>(_e84.g1_.x) * vec4<f32>(_e88.g0_.x, _e91.g1_.y, _e94.g1_.z, _e97.g1_.w)) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0))) + ((vec4<f32>(_e110.g1_.y) * vec4<f32>(_e114.g1_.y)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((vec4<f32>(_e127.g1_.z) * vec4<f32>(_e131.g1_.z)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((vec4<f32>(_e144.g1_.w) * vec4<f32>(_e148.g1_.w)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((vec4<f32>(_e161.g0_.x) * vec4<f32>(_e165.g1_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))), (((((((((vec4<f32>(_e177.g0_.y) * vec4<f32>(_e181.g1_.y, _e184.g0_.x, _e187.g0_.w, _e190.g0_.z)) * vec4<f32>(-(1.0), -(1.0), 1.0, -(1.0))) + ((vec4<f32>(_e204.g0_.z) * vec4<f32>(_e208.g1_.z, _e211.g0_.w, _e214.g0_.x, _e217.g0_.y)) * vec4<f32>(-(1.0), -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e232.g0_.w) * vec4<f32>(_e236.g1_.w, _e239.g0_.z, _e242.g0_.y, _e245.g0_.x)) * vec4<f32>(-(1.0), 1.0, -(1.0), -(1.0)))) + (vec4<f32>(_e260.g1_.x) * vec4<f32>(_e264.g1_.x, _e267.g0_.y, _e270.g0_.z, _e273.g0_.w))) + ((vec4<f32>(_e279.g1_.y) * _e283.g1_.xxwz) * vec4<f32>(0.0, 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e295.g1_.z) * _e299.g1_.wwxy) * vec4<f32>(0.0, -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e311.g1_.w) * _e315.g1_.zzyx) * vec4<f32>(0.0, 1.0, -(1.0), 1.0))) + ((vec4<f32>(_e327.g0_.x) * _e331.g1_) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn point_and_plane_motor_inner_anti_product(self_2058: PointAndPlane, other_1710: Motor) -> PointAndPlane {
    var self_2059: PointAndPlane;
    var other_1711: Motor;

    self_2059 = self_2058;
    other_1711 = other_1710;
    let _e4: PointAndPlane = self_2059;
    let _e8: Motor = other_1711;
    let _e11: Motor = other_1711;
    let _e14: Motor = other_1711;
    let _e17: Motor = other_1711;
    let _e30: PointAndPlane = self_2059;
    let _e34: Motor = other_1711;
    let _e37: Motor = other_1711;
    let _e40: Motor = other_1711;
    let _e43: Motor = other_1711;
    let _e57: PointAndPlane = self_2059;
    let _e61: Motor = other_1711;
    let _e64: Motor = other_1711;
    let _e67: Motor = other_1711;
    let _e70: Motor = other_1711;
    let _e84: PointAndPlane = self_2059;
    let _e88: Motor = other_1711;
    let _e91: Motor = other_1711;
    let _e94: Motor = other_1711;
    let _e97: Motor = other_1711;
    let _e110: PointAndPlane = self_2059;
    let _e114: Motor = other_1711;
    let _e127: PointAndPlane = self_2059;
    let _e131: Motor = other_1711;
    let _e144: PointAndPlane = self_2059;
    let _e148: Motor = other_1711;
    let _e161: PointAndPlane = self_2059;
    let _e165: Motor = other_1711;
    let _e177: PointAndPlane = self_2059;
    let _e181: Motor = other_1711;
    let _e192: PointAndPlane = self_2059;
    let _e196: Motor = other_1711;
    let _e208: PointAndPlane = self_2059;
    let _e212: Motor = other_1711;
    let _e224: PointAndPlane = self_2059;
    let _e227: PointAndPlane = self_2059;
    let _e230: PointAndPlane = self_2059;
    let _e233: PointAndPlane = self_2059;
    let _e237: Motor = other_1711;
    let _e240: Motor = other_1711;
    let _e243: Motor = other_1711;
    let _e246: Motor = other_1711;
    return PointAndPlane((((((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y, _e11.g1_.x, _e14.g1_.w, _e17.g1_.z)) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0))) + ((vec4<f32>(_e30.g0_.z) * vec4<f32>(_e34.g0_.z, _e37.g1_.w, _e40.g1_.x, _e43.g1_.y)) * vec4<f32>(-(1.0), -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e57.g0_.w) * vec4<f32>(_e61.g0_.w, _e64.g1_.z, _e67.g1_.y, _e70.g1_.x)) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))) + ((vec4<f32>(_e84.g1_.x) * vec4<f32>(_e88.g0_.x, _e91.g1_.y, _e94.g1_.z, _e97.g1_.w)) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0))) + ((vec4<f32>(_e110.g1_.y) * vec4<f32>(_e114.g1_.y)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((vec4<f32>(_e127.g1_.z) * vec4<f32>(_e131.g1_.z)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((vec4<f32>(_e144.g1_.w) * vec4<f32>(_e148.g1_.w)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((vec4<f32>(_e161.g0_.x) * vec4<f32>(_e165.g1_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))), (((((vec4<f32>(_e177.g1_.y) * vec4<f32>(_e181.g1_.x)) * vec4<f32>(0.0, 1.0, 0.0, 0.0)) + ((vec4<f32>(_e192.g1_.z) * vec4<f32>(_e196.g1_.x)) * vec4<f32>(0.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e208.g1_.w) * vec4<f32>(_e212.g1_.x)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e224.g1_.x, _e227.g0_.y, _e230.g0_.z, _e233.g0_.w) * vec4<f32>(_e237.g1_.x, _e240.g0_.x, _e243.g0_.x, _e246.g0_.x)) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0)))));
}

fn point_and_plane_motor_right_contraction(self_2060: PointAndPlane, other_1712: Motor) -> PointAndPlane {
    var self_2061: PointAndPlane;
    var other_1713: Motor;

    self_2061 = self_2060;
    other_1713 = other_1712;
    let _e4: PointAndPlane = self_2061;
    let _e6: Motor = other_1713;
    let _e11: PointAndPlane = self_2061;
    let _e15: Motor = other_1713;
    let _e26: PointAndPlane = self_2061;
    let _e30: Motor = other_1713;
    let _e42: PointAndPlane = self_2061;
    let _e46: Motor = other_1713;
    let _e58: PointAndPlane = self_2061;
    let _e62: Motor = other_1713;
    let _e74: PointAndPlane = self_2061;
    let _e78: Motor = other_1713;
    let _e90: PointAndPlane = self_2061;
    let _e94: Motor = other_1713;
    let _e106: PointAndPlane = self_2061;
    let _e109: Motor = other_1713;
    return PointAndPlane((_e4.g0_ * vec4<f32>(_e6.g0_.x)), ((((((((vec4<f32>(_e11.g0_.z) * vec4<f32>(_e15.g0_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) + ((vec4<f32>(_e26.g0_.w) * vec4<f32>(_e30.g0_.w)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e42.g1_.x) * vec4<f32>(_e46.g0_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e58.g1_.y) * vec4<f32>(_e62.g0_.x)) * vec4<f32>(0.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e74.g1_.z) * vec4<f32>(_e78.g0_.x)) * vec4<f32>(0.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e90.g1_.w) * vec4<f32>(_e94.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((_e106.g0_.yxxx * _e109.g0_.yyzw) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0)))));
}

fn point_and_plane_motor_right_anti_contraction(self_2062: PointAndPlane, other_1714: Motor) -> PointAndPlane {
    var self_2063: PointAndPlane;
    var other_1715: Motor;

    self_2063 = self_2062;
    other_1715 = other_1714;
    let _e4: PointAndPlane = self_2063;
    let _e8: Motor = other_1715;
    let _e18: PointAndPlane = self_2063;
    let _e22: Motor = other_1715;
    let _e35: PointAndPlane = self_2063;
    let _e39: Motor = other_1715;
    let _e52: PointAndPlane = self_2063;
    let _e56: Motor = other_1715;
    let _e69: PointAndPlane = self_2063;
    let _e71: Motor = other_1715;
    let _e77: PointAndPlane = self_2063;
    let _e79: Motor = other_1715;
    return PointAndPlane(((((((vec4<f32>(_e4.g1_.x) * _e8.g1_.yyzw) * vec4<f32>(0.0, 1.0, 1.0, 1.0)) + ((vec4<f32>(_e18.g1_.y) * vec4<f32>(_e22.g1_.y)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((vec4<f32>(_e35.g1_.z) * vec4<f32>(_e39.g1_.z)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((vec4<f32>(_e52.g1_.w) * vec4<f32>(_e56.g1_.w)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + (_e69.g0_ * vec4<f32>(_e71.g1_.x))), (_e77.g1_ * vec4<f32>(_e79.g1_.x)));
}

fn point_and_plane_point_and_plane_add(self_2064: PointAndPlane, other_1716: PointAndPlane) -> PointAndPlane {
    var self_2065: PointAndPlane;
    var other_1717: PointAndPlane;

    self_2065 = self_2064;
    other_1717 = other_1716;
    let _e4: PointAndPlane = self_2065;
    let _e6: PointAndPlane = other_1717;
    let _e9: PointAndPlane = self_2065;
    let _e11: PointAndPlane = other_1717;
    return PointAndPlane((_e4.g0_ + _e6.g0_), (_e9.g1_ + _e11.g1_));
}

fn point_and_plane_point_and_plane_sub(self_2066: PointAndPlane, other_1718: PointAndPlane) -> PointAndPlane {
    var self_2067: PointAndPlane;
    var other_1719: PointAndPlane;

    self_2067 = self_2066;
    other_1719 = other_1718;
    let _e4: PointAndPlane = self_2067;
    let _e6: PointAndPlane = other_1719;
    let _e9: PointAndPlane = self_2067;
    let _e11: PointAndPlane = other_1719;
    return PointAndPlane((_e4.g0_ - _e6.g0_), (_e9.g1_ - _e11.g1_));
}

fn point_and_plane_point_and_plane_mul(self_2068: PointAndPlane, other_1720: PointAndPlane) -> PointAndPlane {
    var self_2069: PointAndPlane;
    var other_1721: PointAndPlane;

    self_2069 = self_2068;
    other_1721 = other_1720;
    let _e4: PointAndPlane = self_2069;
    let _e6: PointAndPlane = other_1721;
    let _e9: PointAndPlane = self_2069;
    let _e11: PointAndPlane = other_1721;
    return PointAndPlane((_e4.g0_ * _e6.g0_), (_e9.g1_ * _e11.g1_));
}

fn point_and_plane_point_and_plane_div(self_2070: PointAndPlane, other_1722: PointAndPlane) -> PointAndPlane {
    var self_2071: PointAndPlane;
    var other_1723: PointAndPlane;

    self_2071 = self_2070;
    other_1723 = other_1722;
    let _e4: PointAndPlane = self_2071;
    let _e7: PointAndPlane = self_2071;
    let _e10: PointAndPlane = self_2071;
    let _e13: PointAndPlane = self_2071;
    let _e23: PointAndPlane = other_1723;
    let _e26: PointAndPlane = other_1723;
    let _e29: PointAndPlane = other_1723;
    let _e32: PointAndPlane = other_1723;
    let _e43: PointAndPlane = self_2071;
    let _e46: PointAndPlane = self_2071;
    let _e49: PointAndPlane = self_2071;
    let _e52: PointAndPlane = self_2071;
    let _e62: PointAndPlane = other_1723;
    let _e65: PointAndPlane = other_1723;
    let _e68: PointAndPlane = other_1723;
    let _e71: PointAndPlane = other_1723;
    return PointAndPlane((((vec4<f32>(_e4.g0_.x, _e7.g0_.y, _e10.g0_.z, _e13.g0_.w) * vec4<f32>(1.0, 1.0, 1.0, 1.0)) / vec4<f32>(_e23.g0_.x, _e26.g0_.y, _e29.g0_.z, _e32.g0_.w)) * vec4<f32>(1.0, 1.0, 1.0, 1.0)), (((vec4<f32>(_e43.g1_.x, _e46.g1_.y, _e49.g1_.z, _e52.g1_.w) * vec4<f32>(1.0, 1.0, 1.0, 1.0)) / vec4<f32>(_e62.g1_.x, _e65.g1_.y, _e68.g1_.z, _e71.g1_.w)) * vec4<f32>(1.0, 1.0, 1.0, 1.0)));
}

fn point_and_plane_point_and_plane_geometric_product(self_2072: PointAndPlane, other_1724: PointAndPlane) -> Motor {
    var self_2073: PointAndPlane;
    var other_1725: PointAndPlane;

    self_2073 = self_2072;
    other_1725 = other_1724;
    let _e4: PointAndPlane = self_2073;
    let _e8: PointAndPlane = other_1725;
    let _e11: PointAndPlane = other_1725;
    let _e14: PointAndPlane = other_1725;
    let _e17: PointAndPlane = other_1725;
    let _e29: PointAndPlane = self_2073;
    let _e33: PointAndPlane = other_1725;
    let _e36: PointAndPlane = other_1725;
    let _e39: PointAndPlane = other_1725;
    let _e42: PointAndPlane = other_1725;
    let _e55: PointAndPlane = self_2073;
    let _e59: PointAndPlane = other_1725;
    let _e62: PointAndPlane = other_1725;
    let _e65: PointAndPlane = other_1725;
    let _e68: PointAndPlane = other_1725;
    let _e81: PointAndPlane = self_2073;
    let _e85: PointAndPlane = other_1725;
    let _e88: PointAndPlane = other_1725;
    let _e91: PointAndPlane = other_1725;
    let _e94: PointAndPlane = other_1725;
    let _e109: PointAndPlane = self_2073;
    let _e113: PointAndPlane = other_1725;
    let _e116: PointAndPlane = other_1725;
    let _e119: PointAndPlane = other_1725;
    let _e122: PointAndPlane = other_1725;
    let _e128: PointAndPlane = self_2073;
    let _e132: PointAndPlane = other_1725;
    let _e135: PointAndPlane = other_1725;
    let _e138: PointAndPlane = other_1725;
    let _e141: PointAndPlane = other_1725;
    let _e155: PointAndPlane = self_2073;
    let _e159: PointAndPlane = other_1725;
    let _e162: PointAndPlane = other_1725;
    let _e165: PointAndPlane = other_1725;
    let _e168: PointAndPlane = other_1725;
    let _e182: PointAndPlane = self_2073;
    let _e186: PointAndPlane = other_1725;
    let _e189: PointAndPlane = other_1725;
    let _e192: PointAndPlane = other_1725;
    let _e195: PointAndPlane = other_1725;
    let _e209: PointAndPlane = self_2073;
    let _e213: PointAndPlane = other_1725;
    let _e216: PointAndPlane = other_1725;
    let _e219: PointAndPlane = other_1725;
    let _e222: PointAndPlane = other_1725;
    let _e228: PointAndPlane = self_2073;
    let _e232: PointAndPlane = other_1725;
    let _e235: PointAndPlane = other_1725;
    let _e238: PointAndPlane = other_1725;
    let _e241: PointAndPlane = other_1725;
    let _e255: PointAndPlane = self_2073;
    let _e259: PointAndPlane = other_1725;
    let _e262: PointAndPlane = other_1725;
    let _e265: PointAndPlane = other_1725;
    let _e268: PointAndPlane = other_1725;
    let _e282: PointAndPlane = self_2073;
    let _e286: PointAndPlane = other_1725;
    let _e289: PointAndPlane = other_1725;
    let _e292: PointAndPlane = other_1725;
    let _e295: PointAndPlane = other_1725;
    return Motor((((((vec4<f32>(_e4.g0_.x) * vec4<f32>(_e8.g0_.x, _e11.g1_.y, _e14.g1_.z, _e17.g1_.w)) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0)) + ((vec4<f32>(_e29.g1_.y) * vec4<f32>(_e33.g1_.y, _e36.g0_.x, _e39.g1_.w, _e42.g1_.z)) * vec4<f32>(1.0, 1.0, -(1.0), 1.0))) + ((vec4<f32>(_e55.g1_.z) * vec4<f32>(_e59.g1_.z, _e62.g1_.w, _e65.g0_.x, _e68.g1_.y)) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e81.g1_.w) * vec4<f32>(_e85.g1_.w, _e88.g1_.z, _e91.g1_.y, _e94.g0_.x)) * vec4<f32>(1.0, -(1.0), 1.0, 1.0))), ((((((((vec4<f32>(0.0) - (vec4<f32>(_e109.g0_.x) * vec4<f32>(_e113.g1_.x, _e116.g0_.y, _e119.g0_.z, _e122.g0_.w))) + ((vec4<f32>(_e128.g0_.y) * vec4<f32>(_e132.g1_.y, _e135.g0_.x, _e138.g1_.w, _e141.g1_.z)) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))) + ((vec4<f32>(_e155.g0_.z) * vec4<f32>(_e159.g1_.z, _e162.g1_.w, _e165.g0_.x, _e168.g1_.y)) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e182.g0_.w) * vec4<f32>(_e186.g1_.w, _e189.g1_.z, _e192.g1_.y, _e195.g0_.x)) * vec4<f32>(-(1.0), -(1.0), 1.0, 1.0))) + (vec4<f32>(_e209.g1_.x) * vec4<f32>(_e213.g0_.x, _e216.g1_.y, _e219.g1_.z, _e222.g1_.w))) + ((vec4<f32>(_e228.g1_.y) * vec4<f32>(_e232.g0_.y, _e235.g1_.x, _e238.g0_.w, _e241.g0_.z)) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e255.g1_.z) * vec4<f32>(_e259.g0_.z, _e262.g0_.w, _e265.g1_.x, _e268.g0_.y)) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e282.g1_.w) * vec4<f32>(_e286.g0_.w, _e289.g0_.z, _e292.g0_.y, _e295.g1_.x)) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))));
}

fn point_and_plane_point_and_plane_geometric_anti_product(self_2074: PointAndPlane, other_1726: PointAndPlane) -> Motor {
    var self_2075: PointAndPlane;
    var other_1727: PointAndPlane;

    self_2075 = self_2074;
    other_1727 = other_1726;
    let _e4: PointAndPlane = self_2075;
    let _e8: PointAndPlane = other_1727;
    let _e11: PointAndPlane = other_1727;
    let _e14: PointAndPlane = other_1727;
    let _e17: PointAndPlane = other_1727;
    let _e29: PointAndPlane = self_2075;
    let _e33: PointAndPlane = other_1727;
    let _e36: PointAndPlane = other_1727;
    let _e39: PointAndPlane = other_1727;
    let _e42: PointAndPlane = other_1727;
    let _e57: PointAndPlane = self_2075;
    let _e61: PointAndPlane = other_1727;
    let _e64: PointAndPlane = other_1727;
    let _e67: PointAndPlane = other_1727;
    let _e70: PointAndPlane = other_1727;
    let _e85: PointAndPlane = self_2075;
    let _e89: PointAndPlane = other_1727;
    let _e92: PointAndPlane = other_1727;
    let _e95: PointAndPlane = other_1727;
    let _e98: PointAndPlane = other_1727;
    let _e113: PointAndPlane = self_2075;
    let _e117: PointAndPlane = other_1727;
    let _e120: PointAndPlane = other_1727;
    let _e123: PointAndPlane = other_1727;
    let _e126: PointAndPlane = other_1727;
    let _e141: PointAndPlane = self_2075;
    let _e145: PointAndPlane = other_1727;
    let _e148: PointAndPlane = other_1727;
    let _e151: PointAndPlane = other_1727;
    let _e154: PointAndPlane = other_1727;
    let _e167: PointAndPlane = self_2075;
    let _e171: PointAndPlane = other_1727;
    let _e174: PointAndPlane = other_1727;
    let _e177: PointAndPlane = other_1727;
    let _e180: PointAndPlane = other_1727;
    let _e193: PointAndPlane = self_2075;
    let _e197: PointAndPlane = other_1727;
    let _e200: PointAndPlane = other_1727;
    let _e203: PointAndPlane = other_1727;
    let _e206: PointAndPlane = other_1727;
    let _e219: PointAndPlane = self_2075;
    let _e223: PointAndPlane = other_1727;
    let _e226: PointAndPlane = other_1727;
    let _e229: PointAndPlane = other_1727;
    let _e232: PointAndPlane = other_1727;
    let _e245: PointAndPlane = self_2075;
    let _e249: PointAndPlane = other_1727;
    let _e252: PointAndPlane = other_1727;
    let _e255: PointAndPlane = other_1727;
    let _e258: PointAndPlane = other_1727;
    let _e272: PointAndPlane = self_2075;
    let _e276: PointAndPlane = other_1727;
    let _e279: PointAndPlane = other_1727;
    let _e282: PointAndPlane = other_1727;
    let _e285: PointAndPlane = other_1727;
    let _e299: PointAndPlane = self_2075;
    let _e303: PointAndPlane = other_1727;
    let _e306: PointAndPlane = other_1727;
    let _e309: PointAndPlane = other_1727;
    let _e312: PointAndPlane = other_1727;
    return Motor((((((((((vec4<f32>(_e4.g0_.x) * vec4<f32>(_e8.g1_.x, _e11.g0_.y, _e14.g0_.z, _e17.g0_.w)) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0)) + ((vec4<f32>(_e29.g0_.y) * vec4<f32>(_e33.g1_.y, _e36.g0_.x, _e39.g1_.w, _e42.g1_.z)) * vec4<f32>(-(1.0), -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e57.g0_.z) * vec4<f32>(_e61.g1_.z, _e64.g1_.w, _e67.g0_.x, _e70.g1_.y)) * vec4<f32>(-(1.0), 1.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e85.g0_.w) * vec4<f32>(_e89.g1_.w, _e92.g1_.z, _e95.g1_.y, _e98.g0_.x)) * vec4<f32>(-(1.0), -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e113.g1_.x) * vec4<f32>(_e117.g0_.x, _e120.g1_.y, _e123.g1_.z, _e126.g1_.w)) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0)))) + ((vec4<f32>(_e141.g1_.y) * vec4<f32>(_e145.g0_.y, _e148.g1_.x, _e151.g0_.w, _e154.g0_.z)) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e167.g1_.z) * vec4<f32>(_e171.g0_.z, _e174.g0_.w, _e177.g1_.x, _e180.g0_.y)) * vec4<f32>(1.0, -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e193.g1_.w) * vec4<f32>(_e197.g0_.w, _e200.g0_.z, _e203.g0_.y, _e206.g1_.x)) * vec4<f32>(1.0, 1.0, -(1.0), 1.0))), (((((vec4<f32>(_e219.g0_.y) * vec4<f32>(_e223.g0_.y, _e226.g1_.x, _e229.g0_.w, _e232.g0_.z)) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0)) + ((vec4<f32>(_e245.g0_.z) * vec4<f32>(_e249.g0_.z, _e252.g0_.w, _e255.g1_.x, _e258.g0_.y)) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e272.g0_.w) * vec4<f32>(_e276.g0_.w, _e279.g0_.z, _e282.g0_.y, _e285.g1_.x)) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))) - (vec4<f32>(_e299.g1_.x) * vec4<f32>(_e303.g1_.x, _e306.g0_.y, _e309.g0_.z, _e312.g0_.w))));
}

fn point_and_plane_point_and_plane_scalar_product(self_2076: PointAndPlane, other_1728: PointAndPlane) -> Scalar {
    var self_2077: PointAndPlane;
    var other_1729: PointAndPlane;

    self_2077 = self_2076;
    other_1729 = other_1728;
    let _e5: PointAndPlane = self_2077;
    let _e8: PointAndPlane = other_1729;
    let _e13: PointAndPlane = self_2077;
    let _e16: PointAndPlane = other_1729;
    let _e21: PointAndPlane = self_2077;
    let _e24: PointAndPlane = other_1729;
    let _e29: PointAndPlane = self_2077;
    let _e32: PointAndPlane = other_1729;
    return Scalar(((((0.0 - (_e5.g0_.x * _e8.g0_.x)) + (_e13.g1_.y * _e16.g1_.y)) + (_e21.g1_.z * _e24.g1_.z)) + (_e29.g1_.w * _e32.g1_.w)));
}

fn point_and_plane_point_and_plane_anti_scalar_product(self_2078: PointAndPlane, other_1730: PointAndPlane) -> AntiScalar {
    var self_2079: PointAndPlane;
    var other_1731: PointAndPlane;

    self_2079 = self_2078;
    other_1731 = other_1730;
    let _e4: PointAndPlane = self_2079;
    let _e7: PointAndPlane = other_1731;
    let _e11: PointAndPlane = self_2079;
    let _e14: PointAndPlane = other_1731;
    let _e19: PointAndPlane = self_2079;
    let _e22: PointAndPlane = other_1731;
    let _e27: PointAndPlane = self_2079;
    let _e30: PointAndPlane = other_1731;
    return AntiScalar(((((_e4.g0_.y * _e7.g0_.y) + (_e11.g0_.z * _e14.g0_.z)) + (_e19.g0_.w * _e22.g0_.w)) - (_e27.g1_.x * _e30.g1_.x)));
}

fn point_and_plane_squared_magnitude(self_2080: PointAndPlane) -> Scalar {
    var self_2081: PointAndPlane;

    self_2081 = self_2080;
    let _e2: PointAndPlane = self_2081;
    let _e3: PointAndPlane = self_2081;
    let _e4: PointAndPlane = point_and_plane_reversal(_e3);
    let _e5: Scalar = point_and_plane_point_and_plane_scalar_product(_e2, _e4);
    return _e5;
}

fn point_and_plane_magnitude(self_2082: PointAndPlane) -> Scalar {
    var self_2083: PointAndPlane;

    self_2083 = self_2082;
    let _e2: PointAndPlane = self_2083;
    let _e3: Scalar = point_and_plane_squared_magnitude(_e2);
    return Scalar(sqrt(_e3.g0_));
}

fn point_and_plane_bulk_norm(self_2084: PointAndPlane) -> Scalar {
    var self_2085: PointAndPlane;

    self_2085 = self_2084;
    let _e2: PointAndPlane = self_2085;
    let _e3: Scalar = point_and_plane_squared_magnitude(_e2);
    return Scalar(sqrt(_e3.g0_));
}

fn point_and_plane_squared_anti_magnitude(self_2086: PointAndPlane) -> AntiScalar {
    var self_2087: PointAndPlane;

    self_2087 = self_2086;
    let _e2: PointAndPlane = self_2087;
    let _e3: PointAndPlane = self_2087;
    let _e4: PointAndPlane = point_and_plane_anti_reversal(_e3);
    let _e5: AntiScalar = point_and_plane_point_and_plane_anti_scalar_product(_e2, _e4);
    return _e5;
}

fn point_and_plane_weight_norm(self_2088: PointAndPlane) -> AntiScalar {
    var self_2089: PointAndPlane;

    self_2089 = self_2088;
    let _e2: PointAndPlane = self_2089;
    let _e3: AntiScalar = point_and_plane_squared_anti_magnitude(_e2);
    return AntiScalar(sqrt(_e3.g0_));
}

fn point_and_plane_scale(self_2090: PointAndPlane, other_1732: f32) -> PointAndPlane {
    var self_2091: PointAndPlane;
    var other_1733: f32;

    self_2091 = self_2090;
    other_1733 = other_1732;
    let _e4: PointAndPlane = self_2091;
    let _e5: f32 = other_1733;
    let _e7: PointAndPlane = point_and_plane_scalar_geometric_product(_e4, Scalar(_e5));
    return _e7;
}

fn point_and_plane_signum(self_2092: PointAndPlane) -> PointAndPlane {
    var self_2093: PointAndPlane;

    self_2093 = self_2092;
    let _e2: PointAndPlane = self_2093;
    let _e3: PointAndPlane = self_2093;
    let _e4: Scalar = point_and_plane_magnitude(_e3);
    let _e9: PointAndPlane = point_and_plane_scalar_geometric_product(_e2, Scalar((1.0 / _e4.g0_)));
    return _e9;
}

fn point_and_plane_inverse(self_2094: PointAndPlane) -> PointAndPlane {
    var self_2095: PointAndPlane;

    self_2095 = self_2094;
    let _e2: PointAndPlane = self_2095;
    let _e3: PointAndPlane = point_and_plane_reversal(_e2);
    let _e4: PointAndPlane = self_2095;
    let _e5: Scalar = point_and_plane_squared_magnitude(_e4);
    let _e10: PointAndPlane = point_and_plane_scalar_geometric_product(_e3, Scalar((1.0 / _e5.g0_)));
    return _e10;
}

fn point_and_plane_unitize(self_2096: PointAndPlane) -> PointAndPlane {
    var self_2097: PointAndPlane;

    self_2097 = self_2096;
    let _e2: PointAndPlane = self_2097;
    let _e3: PointAndPlane = self_2097;
    let _e4: AntiScalar = point_and_plane_weight_norm(_e3);
    let _e9: PointAndPlane = point_and_plane_scalar_geometric_product(_e2, Scalar((1.0 / _e4.g0_)));
    return _e9;
}

fn anti_scalar_line_geometric_quotient(self_2098: AntiScalar, other_1734: Line) -> IdealPoint {
    var self_2099: AntiScalar;
    var other_1735: Line;

    self_2099 = self_2098;
    other_1735 = other_1734;
    let _e4: AntiScalar = self_2099;
    let _e5: Line = other_1735;
    let _e6: Line = line_inverse(_e5);
    let _e7: IdealPoint = anti_scalar_line_geometric_product(_e4, _e6);
    return _e7;
}

fn anti_scalar_scalar_geometric_quotient(self_2100: AntiScalar, other_1736: Scalar) -> AntiScalar {
    var self_2101: AntiScalar;
    var other_1737: Scalar;

    self_2101 = self_2100;
    other_1737 = other_1736;
    let _e4: AntiScalar = self_2101;
    let _e5: Scalar = other_1737;
    let _e6: Scalar = scalar_inverse(_e5);
    let _e7: AntiScalar = anti_scalar_scalar_geometric_product(_e4, _e6);
    return _e7;
}

fn anti_scalar_translator_geometric_quotient(self_2102: AntiScalar, other_1738: Translator) -> AntiScalar {
    var self_2103: AntiScalar;
    var other_1739: Translator;

    self_2103 = self_2102;
    other_1739 = other_1738;
    let _e4: AntiScalar = self_2103;
    let _e5: Translator = other_1739;
    let _e6: Translator = translator_inverse(_e5);
    let _e7: AntiScalar = anti_scalar_translator_geometric_product(_e4, _e6);
    return _e7;
}

fn ideal_point_scalar_geometric_quotient(self_2104: IdealPoint, other_1740: Scalar) -> IdealPoint {
    var self_2105: IdealPoint;
    var other_1741: Scalar;

    self_2105 = self_2104;
    other_1741 = other_1740;
    let _e4: IdealPoint = self_2105;
    let _e5: Scalar = other_1741;
    let _e6: Scalar = scalar_inverse(_e5);
    let _e7: IdealPoint = ideal_point_scalar_geometric_product(_e4, _e6);
    return _e7;
}

fn ideal_point_translator_geometric_quotient(self_2106: IdealPoint, other_1742: Translator) -> IdealPoint {
    var self_2107: IdealPoint;
    var other_1743: Translator;

    self_2107 = self_2106;
    other_1743 = other_1742;
    let _e4: IdealPoint = self_2107;
    let _e5: Translator = other_1743;
    let _e6: Translator = translator_inverse(_e5);
    let _e7: IdealPoint = ideal_point_translator_geometric_product(_e4, _e6);
    return _e7;
}

fn line_line_geometric_quotient(self_2108: Line, other_1744: Line) -> Motor {
    var self_2109: Line;
    var other_1745: Line;

    self_2109 = self_2108;
    other_1745 = other_1744;
    let _e4: Line = self_2109;
    let _e5: Line = other_1745;
    let _e6: Line = line_inverse(_e5);
    let _e7: Motor = line_line_geometric_product(_e4, _e6);
    return _e7;
}

fn line_line_transformation(self_2110: Line, other_1746: Line) -> Line {
    var self_2111: Line;
    var other_1747: Line;

    self_2111 = self_2110;
    other_1747 = other_1746;
    let _e4: Line = self_2111;
    let _e5: Line = other_1747;
    let _e6: Motor = line_line_geometric_product(_e4, _e5);
    let _e7: Line = self_2111;
    let _e8: Line = line_reversal(_e7);
    let _e9: Motor = motor_line_geometric_product(_e6, _e8);
    let _e10: Line = motor_line_into(_e9);
    return _e10;
}

fn line_motor_geometric_quotient(self_2112: Line, other_1748: Motor) -> Motor {
    var self_2113: Line;
    var other_1749: Motor;

    self_2113 = self_2112;
    other_1749 = other_1748;
    let _e4: Line = self_2113;
    let _e5: Motor = other_1749;
    let _e6: Motor = motor_inverse(_e5);
    let _e7: Motor = line_motor_geometric_product(_e4, _e6);
    return _e7;
}

fn line_motor_transformation(self_2114: Line, other_1750: Motor) -> Motor {
    var self_2115: Line;
    var other_1751: Motor;

    self_2115 = self_2114;
    other_1751 = other_1750;
    let _e4: Line = self_2115;
    let _e5: Motor = other_1751;
    let _e6: Motor = line_motor_geometric_product(_e4, _e5);
    let _e7: Line = self_2115;
    let _e8: Line = line_reversal(_e7);
    let _e9: Motor = motor_line_geometric_product(_e6, _e8);
    return _e9;
}

fn line_multi_vector_geometric_quotient(self_2116: Line, other_1752: MultiVector) -> MultiVector {
    var self_2117: Line;
    var other_1753: MultiVector;

    self_2117 = self_2116;
    other_1753 = other_1752;
    let _e4: Line = self_2117;
    let _e5: MultiVector = other_1753;
    let _e6: MultiVector = multi_vector_inverse(_e5);
    let _e7: MultiVector = line_multi_vector_geometric_product(_e4, _e6);
    return _e7;
}

fn line_multi_vector_transformation(self_2118: Line, other_1754: MultiVector) -> MultiVector {
    var self_2119: Line;
    var other_1755: MultiVector;

    self_2119 = self_2118;
    other_1755 = other_1754;
    let _e4: Line = self_2119;
    let _e5: MultiVector = other_1755;
    let _e6: MultiVector = line_multi_vector_geometric_product(_e4, _e5);
    let _e7: Line = self_2119;
    let _e8: Line = line_reversal(_e7);
    let _e9: MultiVector = multi_vector_line_geometric_product(_e6, _e8);
    return _e9;
}

fn line_plane_geometric_quotient(self_2120: Line, other_1756: Plane) -> PointAndPlane {
    var self_2121: Line;
    var other_1757: Plane;

    self_2121 = self_2120;
    other_1757 = other_1756;
    let _e4: Line = self_2121;
    let _e5: Plane = other_1757;
    let _e6: Plane = plane_inverse(_e5);
    let _e7: PointAndPlane = line_plane_geometric_product(_e4, _e6);
    return _e7;
}

fn line_plane_transformation(self_2122: Line, other_1758: Plane) -> Plane {
    var self_2123: Line;
    var other_1759: Plane;

    self_2123 = self_2122;
    other_1759 = other_1758;
    let _e4: Line = self_2123;
    let _e5: Plane = other_1759;
    let _e6: PointAndPlane = line_plane_geometric_product(_e4, _e5);
    let _e7: Line = self_2123;
    let _e8: Line = line_reversal(_e7);
    let _e9: PointAndPlane = point_and_plane_line_geometric_product(_e6, _e8);
    let _e10: Plane = point_and_plane_plane_into(_e9);
    return _e10;
}

fn line_point_and_plane_geometric_quotient(self_2124: Line, other_1760: PointAndPlane) -> PointAndPlane {
    var self_2125: Line;
    var other_1761: PointAndPlane;

    self_2125 = self_2124;
    other_1761 = other_1760;
    let _e4: Line = self_2125;
    let _e5: PointAndPlane = other_1761;
    let _e6: PointAndPlane = point_and_plane_inverse(_e5);
    let _e7: PointAndPlane = line_point_and_plane_geometric_product(_e4, _e6);
    return _e7;
}

fn line_point_and_plane_transformation(self_2126: Line, other_1762: PointAndPlane) -> PointAndPlane {
    var self_2127: Line;
    var other_1763: PointAndPlane;

    self_2127 = self_2126;
    other_1763 = other_1762;
    let _e4: Line = self_2127;
    let _e5: PointAndPlane = other_1763;
    let _e6: PointAndPlane = line_point_and_plane_geometric_product(_e4, _e5);
    let _e7: Line = self_2127;
    let _e8: Line = line_reversal(_e7);
    let _e9: PointAndPlane = point_and_plane_line_geometric_product(_e6, _e8);
    return _e9;
}

fn line_rotor_geometric_quotient(self_2128: Line, other_1764: Rotor) -> Motor {
    var self_2129: Line;
    var other_1765: Rotor;

    self_2129 = self_2128;
    other_1765 = other_1764;
    let _e4: Line = self_2129;
    let _e5: Rotor = other_1765;
    let _e6: Rotor = rotor_inverse(_e5);
    let _e7: Motor = line_rotor_geometric_product(_e4, _e6);
    return _e7;
}

fn line_rotor_transformation(self_2130: Line, other_1766: Rotor) -> Rotor {
    var self_2131: Line;
    var other_1767: Rotor;

    self_2131 = self_2130;
    other_1767 = other_1766;
    let _e4: Line = self_2131;
    let _e5: Rotor = other_1767;
    let _e6: Motor = line_rotor_geometric_product(_e4, _e5);
    let _e7: Line = self_2131;
    let _e8: Line = line_reversal(_e7);
    let _e9: Motor = motor_line_geometric_product(_e6, _e8);
    let _e10: Rotor = motor_rotor_into(_e9);
    return _e10;
}

fn line_scalar_geometric_quotient(self_2132: Line, other_1768: Scalar) -> Line {
    var self_2133: Line;
    var other_1769: Scalar;

    self_2133 = self_2132;
    other_1769 = other_1768;
    let _e4: Line = self_2133;
    let _e5: Scalar = other_1769;
    let _e6: Scalar = scalar_inverse(_e5);
    let _e7: Line = line_scalar_geometric_product(_e4, _e6);
    return _e7;
}

fn line_scalar_transformation(self_2134: Line, other_1770: Scalar) -> Scalar {
    var self_2135: Line;
    var other_1771: Scalar;

    self_2135 = self_2134;
    other_1771 = other_1770;
    let _e4: Line = self_2135;
    let _e5: Scalar = other_1771;
    let _e6: Line = line_scalar_geometric_product(_e4, _e5);
    let _e7: Line = self_2135;
    let _e8: Line = line_reversal(_e7);
    let _e9: Motor = line_line_geometric_product(_e6, _e8);
    let _e10: Scalar = motor_scalar_into(_e9);
    return _e10;
}

fn motor_line_geometric_quotient(self_2136: Motor, other_1772: Line) -> Motor {
    var self_2137: Motor;
    var other_1773: Line;

    self_2137 = self_2136;
    other_1773 = other_1772;
    let _e4: Motor = self_2137;
    let _e5: Line = other_1773;
    let _e6: Line = line_inverse(_e5);
    let _e7: Motor = motor_line_geometric_product(_e4, _e6);
    return _e7;
}

fn motor_line_transformation(self_2138: Motor, other_1774: Line) -> Line {
    var self_2139: Motor;
    var other_1775: Line;

    self_2139 = self_2138;
    other_1775 = other_1774;
    let _e4: Motor = self_2139;
    let _e5: Line = other_1775;
    let _e6: Motor = motor_line_geometric_product(_e4, _e5);
    let _e7: Motor = self_2139;
    let _e8: Motor = motor_reversal(_e7);
    let _e9: Motor = motor_motor_geometric_product(_e6, _e8);
    let _e10: Line = motor_line_into(_e9);
    return _e10;
}

fn motor_powi(self_2140: Motor, exponent: i32) -> Motor {
    var self_2141: Motor;
    var exponent_1: i32;
    var local: Motor;
    var x: Motor;
    var y: Motor;
    var n: i32;

    self_2141 = self_2140;
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
        let _e11: Motor = self_2141;
        let _e12: Motor = motor_inverse(_e11);
        local = _e12;
    } else {
        let _e14: Motor = self_2141;
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

fn motor_motor_geometric_quotient(self_2142: Motor, other_1776: Motor) -> Motor {
    var self_2143: Motor;
    var other_1777: Motor;

    self_2143 = self_2142;
    other_1777 = other_1776;
    let _e4: Motor = self_2143;
    let _e5: Motor = other_1777;
    let _e6: Motor = motor_inverse(_e5);
    let _e7: Motor = motor_motor_geometric_product(_e4, _e6);
    return _e7;
}

fn motor_motor_transformation(self_2144: Motor, other_1778: Motor) -> Motor {
    var self_2145: Motor;
    var other_1779: Motor;

    self_2145 = self_2144;
    other_1779 = other_1778;
    let _e4: Motor = self_2145;
    let _e5: Motor = other_1779;
    let _e6: Motor = motor_motor_geometric_product(_e4, _e5);
    let _e7: Motor = self_2145;
    let _e8: Motor = motor_reversal(_e7);
    let _e9: Motor = motor_motor_geometric_product(_e6, _e8);
    return _e9;
}

fn motor_multi_vector_geometric_quotient(self_2146: Motor, other_1780: MultiVector) -> MultiVector {
    var self_2147: Motor;
    var other_1781: MultiVector;

    self_2147 = self_2146;
    other_1781 = other_1780;
    let _e4: Motor = self_2147;
    let _e5: MultiVector = other_1781;
    let _e6: MultiVector = multi_vector_inverse(_e5);
    let _e7: MultiVector = motor_multi_vector_geometric_product(_e4, _e6);
    return _e7;
}

fn motor_multi_vector_transformation(self_2148: Motor, other_1782: MultiVector) -> MultiVector {
    var self_2149: Motor;
    var other_1783: MultiVector;

    self_2149 = self_2148;
    other_1783 = other_1782;
    let _e4: Motor = self_2149;
    let _e5: MultiVector = other_1783;
    let _e6: MultiVector = motor_multi_vector_geometric_product(_e4, _e5);
    let _e7: Motor = self_2149;
    let _e8: Motor = motor_reversal(_e7);
    let _e9: MultiVector = multi_vector_motor_geometric_product(_e6, _e8);
    return _e9;
}

fn motor_plane_geometric_quotient(self_2150: Motor, other_1784: Plane) -> PointAndPlane {
    var self_2151: Motor;
    var other_1785: Plane;

    self_2151 = self_2150;
    other_1785 = other_1784;
    let _e4: Motor = self_2151;
    let _e5: Plane = other_1785;
    let _e6: Plane = plane_inverse(_e5);
    let _e7: PointAndPlane = motor_plane_geometric_product(_e4, _e6);
    return _e7;
}

fn motor_plane_transformation(self_2152: Motor, other_1786: Plane) -> Plane {
    var self_2153: Motor;
    var other_1787: Plane;

    self_2153 = self_2152;
    other_1787 = other_1786;
    let _e4: Motor = self_2153;
    let _e5: Plane = other_1787;
    let _e6: PointAndPlane = motor_plane_geometric_product(_e4, _e5);
    let _e7: Motor = self_2153;
    let _e8: Motor = motor_reversal(_e7);
    let _e9: PointAndPlane = point_and_plane_motor_geometric_product(_e6, _e8);
    let _e10: Plane = point_and_plane_plane_into(_e9);
    return _e10;
}

fn motor_point_geometric_quotient(self_2154: Motor, other_1788: Point) -> PointAndPlane {
    var self_2155: Motor;
    var other_1789: Point;

    self_2155 = self_2154;
    other_1789 = other_1788;
    let _e4: Motor = self_2155;
    let _e5: Point = other_1789;
    let _e6: Point = point_inverse(_e5);
    let _e7: PointAndPlane = motor_point_geometric_product(_e4, _e6);
    return _e7;
}

fn motor_point_transformation(self_2156: Motor, other_1790: Point) -> Point {
    var self_2157: Motor;
    var other_1791: Point;

    self_2157 = self_2156;
    other_1791 = other_1790;
    let _e4: Motor = self_2157;
    let _e5: Point = other_1791;
    let _e6: PointAndPlane = motor_point_geometric_product(_e4, _e5);
    let _e7: Motor = self_2157;
    let _e8: Motor = motor_reversal(_e7);
    let _e9: PointAndPlane = point_and_plane_motor_geometric_product(_e6, _e8);
    let _e10: Point = point_and_plane_point_into(_e9);
    return _e10;
}

fn motor_point_and_plane_geometric_quotient(self_2158: Motor, other_1792: PointAndPlane) -> PointAndPlane {
    var self_2159: Motor;
    var other_1793: PointAndPlane;

    self_2159 = self_2158;
    other_1793 = other_1792;
    let _e4: Motor = self_2159;
    let _e5: PointAndPlane = other_1793;
    let _e6: PointAndPlane = point_and_plane_inverse(_e5);
    let _e7: PointAndPlane = motor_point_and_plane_geometric_product(_e4, _e6);
    return _e7;
}

fn motor_point_and_plane_transformation(self_2160: Motor, other_1794: PointAndPlane) -> PointAndPlane {
    var self_2161: Motor;
    var other_1795: PointAndPlane;

    self_2161 = self_2160;
    other_1795 = other_1794;
    let _e4: Motor = self_2161;
    let _e5: PointAndPlane = other_1795;
    let _e6: PointAndPlane = motor_point_and_plane_geometric_product(_e4, _e5);
    let _e7: Motor = self_2161;
    let _e8: Motor = motor_reversal(_e7);
    let _e9: PointAndPlane = point_and_plane_motor_geometric_product(_e6, _e8);
    return _e9;
}

fn motor_rotor_geometric_quotient(self_2162: Motor, other_1796: Rotor) -> Motor {
    var self_2163: Motor;
    var other_1797: Rotor;

    self_2163 = self_2162;
    other_1797 = other_1796;
    let _e4: Motor = self_2163;
    let _e5: Rotor = other_1797;
    let _e6: Rotor = rotor_inverse(_e5);
    let _e7: Motor = motor_rotor_geometric_product(_e4, _e6);
    return _e7;
}

fn motor_rotor_transformation(self_2164: Motor, other_1798: Rotor) -> Rotor {
    var self_2165: Motor;
    var other_1799: Rotor;

    self_2165 = self_2164;
    other_1799 = other_1798;
    let _e4: Motor = self_2165;
    let _e5: Rotor = other_1799;
    let _e6: Motor = motor_rotor_geometric_product(_e4, _e5);
    let _e7: Motor = self_2165;
    let _e8: Motor = motor_reversal(_e7);
    let _e9: Motor = motor_motor_geometric_product(_e6, _e8);
    let _e10: Rotor = motor_rotor_into(_e9);
    return _e10;
}

fn motor_scalar_geometric_quotient(self_2166: Motor, other_1800: Scalar) -> Motor {
    var self_2167: Motor;
    var other_1801: Scalar;

    self_2167 = self_2166;
    other_1801 = other_1800;
    let _e4: Motor = self_2167;
    let _e5: Scalar = other_1801;
    let _e6: Scalar = scalar_inverse(_e5);
    let _e7: Motor = motor_scalar_geometric_product(_e4, _e6);
    return _e7;
}

fn motor_scalar_transformation(self_2168: Motor, other_1802: Scalar) -> Scalar {
    var self_2169: Motor;
    var other_1803: Scalar;

    self_2169 = self_2168;
    other_1803 = other_1802;
    let _e4: Motor = self_2169;
    let _e5: Scalar = other_1803;
    let _e6: Motor = motor_scalar_geometric_product(_e4, _e5);
    let _e7: Motor = self_2169;
    let _e8: Motor = motor_reversal(_e7);
    let _e9: Motor = motor_motor_geometric_product(_e6, _e8);
    let _e10: Scalar = motor_scalar_into(_e9);
    return _e10;
}

fn motor_translator_geometric_quotient(self_2170: Motor, other_1804: Translator) -> Motor {
    var self_2171: Motor;
    var other_1805: Translator;

    self_2171 = self_2170;
    other_1805 = other_1804;
    let _e4: Motor = self_2171;
    let _e5: Translator = other_1805;
    let _e6: Translator = translator_inverse(_e5);
    let _e7: Motor = motor_translator_geometric_product(_e4, _e6);
    return _e7;
}

fn motor_translator_transformation(self_2172: Motor, other_1806: Translator) -> Translator {
    var self_2173: Motor;
    var other_1807: Translator;

    self_2173 = self_2172;
    other_1807 = other_1806;
    let _e4: Motor = self_2173;
    let _e5: Translator = other_1807;
    let _e6: Motor = motor_translator_geometric_product(_e4, _e5);
    let _e7: Motor = self_2173;
    let _e8: Motor = motor_reversal(_e7);
    let _e9: Motor = motor_motor_geometric_product(_e6, _e8);
    let _e10: Translator = motor_translator_into(_e9);
    return _e10;
}

fn multi_vector_line_geometric_quotient(self_2174: MultiVector, other_1808: Line) -> MultiVector {
    var self_2175: MultiVector;
    var other_1809: Line;

    self_2175 = self_2174;
    other_1809 = other_1808;
    let _e4: MultiVector = self_2175;
    let _e5: Line = other_1809;
    let _e6: Line = line_inverse(_e5);
    let _e7: MultiVector = multi_vector_line_geometric_product(_e4, _e6);
    return _e7;
}

fn multi_vector_line_transformation(self_2176: MultiVector, other_1810: Line) -> Line {
    var self_2177: MultiVector;
    var other_1811: Line;

    self_2177 = self_2176;
    other_1811 = other_1810;
    let _e4: MultiVector = self_2177;
    let _e5: Line = other_1811;
    let _e6: MultiVector = multi_vector_line_geometric_product(_e4, _e5);
    let _e7: MultiVector = self_2177;
    let _e8: MultiVector = multi_vector_reversal(_e7);
    let _e9: MultiVector = multi_vector_multi_vector_geometric_product(_e6, _e8);
    let _e10: Line = multi_vector_line_into(_e9);
    return _e10;
}

fn multi_vector_motor_geometric_quotient(self_2178: MultiVector, other_1812: Motor) -> MultiVector {
    var self_2179: MultiVector;
    var other_1813: Motor;

    self_2179 = self_2178;
    other_1813 = other_1812;
    let _e4: MultiVector = self_2179;
    let _e5: Motor = other_1813;
    let _e6: Motor = motor_inverse(_e5);
    let _e7: MultiVector = multi_vector_motor_geometric_product(_e4, _e6);
    return _e7;
}

fn multi_vector_motor_transformation(self_2180: MultiVector, other_1814: Motor) -> Motor {
    var self_2181: MultiVector;
    var other_1815: Motor;

    self_2181 = self_2180;
    other_1815 = other_1814;
    let _e4: MultiVector = self_2181;
    let _e5: Motor = other_1815;
    let _e6: MultiVector = multi_vector_motor_geometric_product(_e4, _e5);
    let _e7: MultiVector = self_2181;
    let _e8: MultiVector = multi_vector_reversal(_e7);
    let _e9: MultiVector = multi_vector_multi_vector_geometric_product(_e6, _e8);
    let _e10: Motor = multi_vector_motor_into(_e9);
    return _e10;
}

fn multi_vector_powi(self_2182: MultiVector, exponent_2: i32) -> MultiVector {
    var self_2183: MultiVector;
    var exponent_3: i32;
    var local_1: MultiVector;
    var x_1: MultiVector;
    var y_1: MultiVector;
    var n_1: i32;

    self_2183 = self_2182;
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
        let _e11: MultiVector = self_2183;
        let _e12: MultiVector = multi_vector_inverse(_e11);
        local_1 = _e12;
    } else {
        let _e14: MultiVector = self_2183;
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

fn multi_vector_multi_vector_geometric_quotient(self_2184: MultiVector, other_1816: MultiVector) -> MultiVector {
    var self_2185: MultiVector;
    var other_1817: MultiVector;

    self_2185 = self_2184;
    other_1817 = other_1816;
    let _e4: MultiVector = self_2185;
    let _e5: MultiVector = other_1817;
    let _e6: MultiVector = multi_vector_inverse(_e5);
    let _e7: MultiVector = multi_vector_multi_vector_geometric_product(_e4, _e6);
    return _e7;
}

fn multi_vector_multi_vector_transformation(self_2186: MultiVector, other_1818: MultiVector) -> MultiVector {
    var self_2187: MultiVector;
    var other_1819: MultiVector;

    self_2187 = self_2186;
    other_1819 = other_1818;
    let _e4: MultiVector = self_2187;
    let _e5: MultiVector = other_1819;
    let _e6: MultiVector = multi_vector_multi_vector_geometric_product(_e4, _e5);
    let _e7: MultiVector = self_2187;
    let _e8: MultiVector = multi_vector_reversal(_e7);
    let _e9: MultiVector = multi_vector_multi_vector_geometric_product(_e6, _e8);
    return _e9;
}

fn multi_vector_plane_geometric_quotient(self_2188: MultiVector, other_1820: Plane) -> MultiVector {
    var self_2189: MultiVector;
    var other_1821: Plane;

    self_2189 = self_2188;
    other_1821 = other_1820;
    let _e4: MultiVector = self_2189;
    let _e5: Plane = other_1821;
    let _e6: Plane = plane_inverse(_e5);
    let _e7: MultiVector = multi_vector_plane_geometric_product(_e4, _e6);
    return _e7;
}

fn multi_vector_plane_transformation(self_2190: MultiVector, other_1822: Plane) -> Plane {
    var self_2191: MultiVector;
    var other_1823: Plane;

    self_2191 = self_2190;
    other_1823 = other_1822;
    let _e4: MultiVector = self_2191;
    let _e5: Plane = other_1823;
    let _e6: MultiVector = multi_vector_plane_geometric_product(_e4, _e5);
    let _e7: MultiVector = self_2191;
    let _e8: MultiVector = multi_vector_reversal(_e7);
    let _e9: MultiVector = multi_vector_multi_vector_geometric_product(_e6, _e8);
    let _e10: Plane = multi_vector_plane_into(_e9);
    return _e10;
}

fn multi_vector_point_geometric_quotient(self_2192: MultiVector, other_1824: Point) -> MultiVector {
    var self_2193: MultiVector;
    var other_1825: Point;

    self_2193 = self_2192;
    other_1825 = other_1824;
    let _e4: MultiVector = self_2193;
    let _e5: Point = other_1825;
    let _e6: Point = point_inverse(_e5);
    let _e7: MultiVector = multi_vector_point_geometric_product(_e4, _e6);
    return _e7;
}

fn multi_vector_point_transformation(self_2194: MultiVector, other_1826: Point) -> Point {
    var self_2195: MultiVector;
    var other_1827: Point;

    self_2195 = self_2194;
    other_1827 = other_1826;
    let _e4: MultiVector = self_2195;
    let _e5: Point = other_1827;
    let _e6: MultiVector = multi_vector_point_geometric_product(_e4, _e5);
    let _e7: MultiVector = self_2195;
    let _e8: MultiVector = multi_vector_reversal(_e7);
    let _e9: MultiVector = multi_vector_multi_vector_geometric_product(_e6, _e8);
    let _e10: Point = multi_vector_point_into(_e9);
    return _e10;
}

fn multi_vector_point_and_plane_geometric_quotient(self_2196: MultiVector, other_1828: PointAndPlane) -> MultiVector {
    var self_2197: MultiVector;
    var other_1829: PointAndPlane;

    self_2197 = self_2196;
    other_1829 = other_1828;
    let _e4: MultiVector = self_2197;
    let _e5: PointAndPlane = other_1829;
    let _e6: PointAndPlane = point_and_plane_inverse(_e5);
    let _e7: MultiVector = multi_vector_point_and_plane_geometric_product(_e4, _e6);
    return _e7;
}

fn multi_vector_point_and_plane_transformation(self_2198: MultiVector, other_1830: PointAndPlane) -> PointAndPlane {
    var self_2199: MultiVector;
    var other_1831: PointAndPlane;

    self_2199 = self_2198;
    other_1831 = other_1830;
    let _e4: MultiVector = self_2199;
    let _e5: PointAndPlane = other_1831;
    let _e6: MultiVector = multi_vector_point_and_plane_geometric_product(_e4, _e5);
    let _e7: MultiVector = self_2199;
    let _e8: MultiVector = multi_vector_reversal(_e7);
    let _e9: MultiVector = multi_vector_multi_vector_geometric_product(_e6, _e8);
    let _e10: PointAndPlane = multi_vector_point_and_plane_into(_e9);
    return _e10;
}

fn multi_vector_rotor_geometric_quotient(self_2200: MultiVector, other_1832: Rotor) -> MultiVector {
    var self_2201: MultiVector;
    var other_1833: Rotor;

    self_2201 = self_2200;
    other_1833 = other_1832;
    let _e4: MultiVector = self_2201;
    let _e5: Rotor = other_1833;
    let _e6: Rotor = rotor_inverse(_e5);
    let _e7: MultiVector = multi_vector_rotor_geometric_product(_e4, _e6);
    return _e7;
}

fn multi_vector_rotor_transformation(self_2202: MultiVector, other_1834: Rotor) -> Rotor {
    var self_2203: MultiVector;
    var other_1835: Rotor;

    self_2203 = self_2202;
    other_1835 = other_1834;
    let _e4: MultiVector = self_2203;
    let _e5: Rotor = other_1835;
    let _e6: MultiVector = multi_vector_rotor_geometric_product(_e4, _e5);
    let _e7: MultiVector = self_2203;
    let _e8: MultiVector = multi_vector_reversal(_e7);
    let _e9: MultiVector = multi_vector_multi_vector_geometric_product(_e6, _e8);
    let _e10: Rotor = multi_vector_rotor_into(_e9);
    return _e10;
}

fn multi_vector_scalar_geometric_quotient(self_2204: MultiVector, other_1836: Scalar) -> MultiVector {
    var self_2205: MultiVector;
    var other_1837: Scalar;

    self_2205 = self_2204;
    other_1837 = other_1836;
    let _e4: MultiVector = self_2205;
    let _e5: Scalar = other_1837;
    let _e6: Scalar = scalar_inverse(_e5);
    let _e7: MultiVector = multi_vector_scalar_geometric_product(_e4, _e6);
    return _e7;
}

fn multi_vector_scalar_transformation(self_2206: MultiVector, other_1838: Scalar) -> Scalar {
    var self_2207: MultiVector;
    var other_1839: Scalar;

    self_2207 = self_2206;
    other_1839 = other_1838;
    let _e4: MultiVector = self_2207;
    let _e5: Scalar = other_1839;
    let _e6: MultiVector = multi_vector_scalar_geometric_product(_e4, _e5);
    let _e7: MultiVector = self_2207;
    let _e8: MultiVector = multi_vector_reversal(_e7);
    let _e9: MultiVector = multi_vector_multi_vector_geometric_product(_e6, _e8);
    let _e10: Scalar = multi_vector_scalar_into(_e9);
    return _e10;
}

fn multi_vector_translator_geometric_quotient(self_2208: MultiVector, other_1840: Translator) -> MultiVector {
    var self_2209: MultiVector;
    var other_1841: Translator;

    self_2209 = self_2208;
    other_1841 = other_1840;
    let _e4: MultiVector = self_2209;
    let _e5: Translator = other_1841;
    let _e6: Translator = translator_inverse(_e5);
    let _e7: MultiVector = multi_vector_translator_geometric_product(_e4, _e6);
    return _e7;
}

fn multi_vector_translator_transformation(self_2210: MultiVector, other_1842: Translator) -> Translator {
    var self_2211: MultiVector;
    var other_1843: Translator;

    self_2211 = self_2210;
    other_1843 = other_1842;
    let _e4: MultiVector = self_2211;
    let _e5: Translator = other_1843;
    let _e6: MultiVector = multi_vector_translator_geometric_product(_e4, _e5);
    let _e7: MultiVector = self_2211;
    let _e8: MultiVector = multi_vector_reversal(_e7);
    let _e9: MultiVector = multi_vector_multi_vector_geometric_product(_e6, _e8);
    let _e10: Translator = multi_vector_translator_into(_e9);
    return _e10;
}

fn plane_line_geometric_quotient(self_2212: Plane, other_1844: Line) -> PointAndPlane {
    var self_2213: Plane;
    var other_1845: Line;

    self_2213 = self_2212;
    other_1845 = other_1844;
    let _e4: Plane = self_2213;
    let _e5: Line = other_1845;
    let _e6: Line = line_inverse(_e5);
    let _e7: PointAndPlane = plane_line_geometric_product(_e4, _e6);
    return _e7;
}

fn plane_line_transformation(self_2214: Plane, other_1846: Line) -> Line {
    var self_2215: Plane;
    var other_1847: Line;

    self_2215 = self_2214;
    other_1847 = other_1846;
    let _e4: Plane = self_2215;
    let _e5: Line = other_1847;
    let _e6: PointAndPlane = plane_line_geometric_product(_e4, _e5);
    let _e7: Plane = self_2215;
    let _e8: Plane = plane_reversal(_e7);
    let _e9: Motor = point_and_plane_plane_geometric_product(_e6, _e8);
    let _e10: Line = motor_line_into(_e9);
    return _e10;
}

fn plane_motor_geometric_quotient(self_2216: Plane, other_1848: Motor) -> PointAndPlane {
    var self_2217: Plane;
    var other_1849: Motor;

    self_2217 = self_2216;
    other_1849 = other_1848;
    let _e4: Plane = self_2217;
    let _e5: Motor = other_1849;
    let _e6: Motor = motor_inverse(_e5);
    let _e7: PointAndPlane = plane_motor_geometric_product(_e4, _e6);
    return _e7;
}

fn plane_motor_transformation(self_2218: Plane, other_1850: Motor) -> Motor {
    var self_2219: Plane;
    var other_1851: Motor;

    self_2219 = self_2218;
    other_1851 = other_1850;
    let _e4: Plane = self_2219;
    let _e5: Motor = other_1851;
    let _e6: PointAndPlane = plane_motor_geometric_product(_e4, _e5);
    let _e7: Plane = self_2219;
    let _e8: Plane = plane_reversal(_e7);
    let _e9: Motor = point_and_plane_plane_geometric_product(_e6, _e8);
    return _e9;
}

fn plane_multi_vector_geometric_quotient(self_2220: Plane, other_1852: MultiVector) -> MultiVector {
    var self_2221: Plane;
    var other_1853: MultiVector;

    self_2221 = self_2220;
    other_1853 = other_1852;
    let _e4: Plane = self_2221;
    let _e5: MultiVector = other_1853;
    let _e6: MultiVector = multi_vector_inverse(_e5);
    let _e7: MultiVector = plane_multi_vector_geometric_product(_e4, _e6);
    return _e7;
}

fn plane_multi_vector_transformation(self_2222: Plane, other_1854: MultiVector) -> MultiVector {
    var self_2223: Plane;
    var other_1855: MultiVector;

    self_2223 = self_2222;
    other_1855 = other_1854;
    let _e4: Plane = self_2223;
    let _e5: MultiVector = other_1855;
    let _e6: MultiVector = plane_multi_vector_geometric_product(_e4, _e5);
    let _e7: Plane = self_2223;
    let _e8: Plane = plane_reversal(_e7);
    let _e9: MultiVector = multi_vector_plane_geometric_product(_e6, _e8);
    return _e9;
}

fn plane_point_and_plane_geometric_quotient(self_2224: Plane, other_1856: PointAndPlane) -> Motor {
    var self_2225: Plane;
    var other_1857: PointAndPlane;

    self_2225 = self_2224;
    other_1857 = other_1856;
    let _e4: Plane = self_2225;
    let _e5: PointAndPlane = other_1857;
    let _e6: PointAndPlane = point_and_plane_inverse(_e5);
    let _e7: Motor = plane_point_and_plane_geometric_product(_e4, _e6);
    return _e7;
}

fn plane_point_and_plane_transformation(self_2226: Plane, other_1858: PointAndPlane) -> PointAndPlane {
    var self_2227: Plane;
    var other_1859: PointAndPlane;

    self_2227 = self_2226;
    other_1859 = other_1858;
    let _e4: Plane = self_2227;
    let _e5: PointAndPlane = other_1859;
    let _e6: Motor = plane_point_and_plane_geometric_product(_e4, _e5);
    let _e7: Plane = self_2227;
    let _e8: Plane = plane_reversal(_e7);
    let _e9: PointAndPlane = motor_plane_geometric_product(_e6, _e8);
    return _e9;
}

fn plane_rotor_geometric_quotient(self_2228: Plane, other_1860: Rotor) -> PointAndPlane {
    var self_2229: Plane;
    var other_1861: Rotor;

    self_2229 = self_2228;
    other_1861 = other_1860;
    let _e4: Plane = self_2229;
    let _e5: Rotor = other_1861;
    let _e6: Rotor = rotor_inverse(_e5);
    let _e7: PointAndPlane = plane_rotor_geometric_product(_e4, _e6);
    return _e7;
}

fn plane_rotor_transformation(self_2230: Plane, other_1862: Rotor) -> Rotor {
    var self_2231: Plane;
    var other_1863: Rotor;

    self_2231 = self_2230;
    other_1863 = other_1862;
    let _e4: Plane = self_2231;
    let _e5: Rotor = other_1863;
    let _e6: PointAndPlane = plane_rotor_geometric_product(_e4, _e5);
    let _e7: Plane = self_2231;
    let _e8: Plane = plane_reversal(_e7);
    let _e9: Motor = point_and_plane_plane_geometric_product(_e6, _e8);
    let _e10: Rotor = motor_rotor_into(_e9);
    return _e10;
}

fn plane_scalar_geometric_quotient(self_2232: Plane, other_1864: Scalar) -> Plane {
    var self_2233: Plane;
    var other_1865: Scalar;

    self_2233 = self_2232;
    other_1865 = other_1864;
    let _e4: Plane = self_2233;
    let _e5: Scalar = other_1865;
    let _e6: Scalar = scalar_inverse(_e5);
    let _e7: Plane = plane_scalar_geometric_product(_e4, _e6);
    return _e7;
}

fn point_motor_geometric_quotient(self_2234: Point, other_1866: Motor) -> PointAndPlane {
    var self_2235: Point;
    var other_1867: Motor;

    self_2235 = self_2234;
    other_1867 = other_1866;
    let _e4: Point = self_2235;
    let _e5: Motor = other_1867;
    let _e6: Motor = motor_inverse(_e5);
    let _e7: PointAndPlane = point_motor_geometric_product(_e4, _e6);
    return _e7;
}

fn point_motor_transformation(self_2236: Point, other_1868: Motor) -> Motor {
    var self_2237: Point;
    var other_1869: Motor;

    self_2237 = self_2236;
    other_1869 = other_1868;
    let _e4: Point = self_2237;
    let _e5: Motor = other_1869;
    let _e6: PointAndPlane = point_motor_geometric_product(_e4, _e5);
    let _e7: Point = self_2237;
    let _e8: Point = point_reversal(_e7);
    let _e9: Motor = point_and_plane_point_geometric_product(_e6, _e8);
    return _e9;
}

fn point_multi_vector_geometric_quotient(self_2238: Point, other_1870: MultiVector) -> MultiVector {
    var self_2239: Point;
    var other_1871: MultiVector;

    self_2239 = self_2238;
    other_1871 = other_1870;
    let _e4: Point = self_2239;
    let _e5: MultiVector = other_1871;
    let _e6: MultiVector = multi_vector_inverse(_e5);
    let _e7: MultiVector = point_multi_vector_geometric_product(_e4, _e6);
    return _e7;
}

fn point_multi_vector_transformation(self_2240: Point, other_1872: MultiVector) -> MultiVector {
    var self_2241: Point;
    var other_1873: MultiVector;

    self_2241 = self_2240;
    other_1873 = other_1872;
    let _e4: Point = self_2241;
    let _e5: MultiVector = other_1873;
    let _e6: MultiVector = point_multi_vector_geometric_product(_e4, _e5);
    let _e7: Point = self_2241;
    let _e8: Point = point_reversal(_e7);
    let _e9: MultiVector = multi_vector_point_geometric_product(_e6, _e8);
    return _e9;
}

fn point_point_geometric_quotient(self_2242: Point, other_1874: Point) -> Translator {
    var self_2243: Point;
    var other_1875: Point;

    self_2243 = self_2242;
    other_1875 = other_1874;
    let _e4: Point = self_2243;
    let _e5: Point = other_1875;
    let _e6: Point = point_inverse(_e5);
    let _e7: Translator = point_point_geometric_product(_e4, _e6);
    return _e7;
}

fn point_point_transformation(self_2244: Point, other_1876: Point) -> Point {
    var self_2245: Point;
    var other_1877: Point;

    self_2245 = self_2244;
    other_1877 = other_1876;
    let _e4: Point = self_2245;
    let _e5: Point = other_1877;
    let _e6: Translator = point_point_geometric_product(_e4, _e5);
    let _e7: Point = self_2245;
    let _e8: Point = point_reversal(_e7);
    let _e9: Point = translator_point_geometric_product(_e6, _e8);
    return _e9;
}

fn point_point_and_plane_geometric_quotient(self_2246: Point, other_1878: PointAndPlane) -> Motor {
    var self_2247: Point;
    var other_1879: PointAndPlane;

    self_2247 = self_2246;
    other_1879 = other_1878;
    let _e4: Point = self_2247;
    let _e5: PointAndPlane = other_1879;
    let _e6: PointAndPlane = point_and_plane_inverse(_e5);
    let _e7: Motor = point_point_and_plane_geometric_product(_e4, _e6);
    return _e7;
}

fn point_point_and_plane_transformation(self_2248: Point, other_1880: PointAndPlane) -> PointAndPlane {
    var self_2249: Point;
    var other_1881: PointAndPlane;

    self_2249 = self_2248;
    other_1881 = other_1880;
    let _e4: Point = self_2249;
    let _e5: PointAndPlane = other_1881;
    let _e6: Motor = point_point_and_plane_geometric_product(_e4, _e5);
    let _e7: Point = self_2249;
    let _e8: Point = point_reversal(_e7);
    let _e9: PointAndPlane = motor_point_geometric_product(_e6, _e8);
    return _e9;
}

fn point_rotor_geometric_quotient(self_2250: Point, other_1882: Rotor) -> PointAndPlane {
    var self_2251: Point;
    var other_1883: Rotor;

    self_2251 = self_2250;
    other_1883 = other_1882;
    let _e4: Point = self_2251;
    let _e5: Rotor = other_1883;
    let _e6: Rotor = rotor_inverse(_e5);
    let _e7: PointAndPlane = point_rotor_geometric_product(_e4, _e6);
    return _e7;
}

fn point_rotor_transformation(self_2252: Point, other_1884: Rotor) -> Rotor {
    var self_2253: Point;
    var other_1885: Rotor;

    self_2253 = self_2252;
    other_1885 = other_1884;
    let _e4: Point = self_2253;
    let _e5: Rotor = other_1885;
    let _e6: PointAndPlane = point_rotor_geometric_product(_e4, _e5);
    let _e7: Point = self_2253;
    let _e8: Point = point_reversal(_e7);
    let _e9: Motor = point_and_plane_point_geometric_product(_e6, _e8);
    let _e10: Rotor = motor_rotor_into(_e9);
    return _e10;
}

fn point_scalar_geometric_quotient(self_2254: Point, other_1886: Scalar) -> Point {
    var self_2255: Point;
    var other_1887: Scalar;

    self_2255 = self_2254;
    other_1887 = other_1886;
    let _e4: Point = self_2255;
    let _e5: Scalar = other_1887;
    let _e6: Scalar = scalar_inverse(_e5);
    let _e7: Point = point_scalar_geometric_product(_e4, _e6);
    return _e7;
}

fn point_scalar_transformation(self_2256: Point, other_1888: Scalar) -> Scalar {
    var self_2257: Point;
    var other_1889: Scalar;

    self_2257 = self_2256;
    other_1889 = other_1888;
    let _e4: Point = self_2257;
    let _e5: Scalar = other_1889;
    let _e6: Point = point_scalar_geometric_product(_e4, _e5);
    let _e7: Point = self_2257;
    let _e8: Point = point_reversal(_e7);
    let _e9: Translator = point_point_geometric_product(_e6, _e8);
    let _e10: Scalar = translator_scalar_into(_e9);
    return _e10;
}

fn point_translator_geometric_quotient(self_2258: Point, other_1890: Translator) -> Point {
    var self_2259: Point;
    var other_1891: Translator;

    self_2259 = self_2258;
    other_1891 = other_1890;
    let _e4: Point = self_2259;
    let _e5: Translator = other_1891;
    let _e6: Translator = translator_inverse(_e5);
    let _e7: Point = point_translator_geometric_product(_e4, _e6);
    return _e7;
}

fn point_translator_transformation(self_2260: Point, other_1892: Translator) -> Translator {
    var self_2261: Point;
    var other_1893: Translator;

    self_2261 = self_2260;
    other_1893 = other_1892;
    let _e4: Point = self_2261;
    let _e5: Translator = other_1893;
    let _e6: Point = point_translator_geometric_product(_e4, _e5);
    let _e7: Point = self_2261;
    let _e8: Point = point_reversal(_e7);
    let _e9: Translator = point_point_geometric_product(_e6, _e8);
    return _e9;
}

fn point_and_plane_line_geometric_quotient(self_2262: PointAndPlane, other_1894: Line) -> PointAndPlane {
    var self_2263: PointAndPlane;
    var other_1895: Line;

    self_2263 = self_2262;
    other_1895 = other_1894;
    let _e4: PointAndPlane = self_2263;
    let _e5: Line = other_1895;
    let _e6: Line = line_inverse(_e5);
    let _e7: PointAndPlane = point_and_plane_line_geometric_product(_e4, _e6);
    return _e7;
}

fn point_and_plane_line_transformation(self_2264: PointAndPlane, other_1896: Line) -> Line {
    var self_2265: PointAndPlane;
    var other_1897: Line;

    self_2265 = self_2264;
    other_1897 = other_1896;
    let _e4: PointAndPlane = self_2265;
    let _e5: Line = other_1897;
    let _e6: PointAndPlane = point_and_plane_line_geometric_product(_e4, _e5);
    let _e7: PointAndPlane = self_2265;
    let _e8: PointAndPlane = point_and_plane_reversal(_e7);
    let _e9: Motor = point_and_plane_point_and_plane_geometric_product(_e6, _e8);
    let _e10: Line = motor_line_into(_e9);
    return _e10;
}

fn point_and_plane_motor_geometric_quotient(self_2266: PointAndPlane, other_1898: Motor) -> PointAndPlane {
    var self_2267: PointAndPlane;
    var other_1899: Motor;

    self_2267 = self_2266;
    other_1899 = other_1898;
    let _e4: PointAndPlane = self_2267;
    let _e5: Motor = other_1899;
    let _e6: Motor = motor_inverse(_e5);
    let _e7: PointAndPlane = point_and_plane_motor_geometric_product(_e4, _e6);
    return _e7;
}

fn point_and_plane_motor_transformation(self_2268: PointAndPlane, other_1900: Motor) -> Motor {
    var self_2269: PointAndPlane;
    var other_1901: Motor;

    self_2269 = self_2268;
    other_1901 = other_1900;
    let _e4: PointAndPlane = self_2269;
    let _e5: Motor = other_1901;
    let _e6: PointAndPlane = point_and_plane_motor_geometric_product(_e4, _e5);
    let _e7: PointAndPlane = self_2269;
    let _e8: PointAndPlane = point_and_plane_reversal(_e7);
    let _e9: Motor = point_and_plane_point_and_plane_geometric_product(_e6, _e8);
    return _e9;
}

fn point_and_plane_multi_vector_geometric_quotient(self_2270: PointAndPlane, other_1902: MultiVector) -> MultiVector {
    var self_2271: PointAndPlane;
    var other_1903: MultiVector;

    self_2271 = self_2270;
    other_1903 = other_1902;
    let _e4: PointAndPlane = self_2271;
    let _e5: MultiVector = other_1903;
    let _e6: MultiVector = multi_vector_inverse(_e5);
    let _e7: MultiVector = point_and_plane_multi_vector_geometric_product(_e4, _e6);
    return _e7;
}

fn point_and_plane_multi_vector_transformation(self_2272: PointAndPlane, other_1904: MultiVector) -> MultiVector {
    var self_2273: PointAndPlane;
    var other_1905: MultiVector;

    self_2273 = self_2272;
    other_1905 = other_1904;
    let _e4: PointAndPlane = self_2273;
    let _e5: MultiVector = other_1905;
    let _e6: MultiVector = point_and_plane_multi_vector_geometric_product(_e4, _e5);
    let _e7: PointAndPlane = self_2273;
    let _e8: PointAndPlane = point_and_plane_reversal(_e7);
    let _e9: MultiVector = multi_vector_point_and_plane_geometric_product(_e6, _e8);
    return _e9;
}

fn point_and_plane_plane_geometric_quotient(self_2274: PointAndPlane, other_1906: Plane) -> Motor {
    var self_2275: PointAndPlane;
    var other_1907: Plane;

    self_2275 = self_2274;
    other_1907 = other_1906;
    let _e4: PointAndPlane = self_2275;
    let _e5: Plane = other_1907;
    let _e6: Plane = plane_inverse(_e5);
    let _e7: Motor = point_and_plane_plane_geometric_product(_e4, _e6);
    return _e7;
}

fn point_and_plane_plane_transformation(self_2276: PointAndPlane, other_1908: Plane) -> Plane {
    var self_2277: PointAndPlane;
    var other_1909: Plane;

    self_2277 = self_2276;
    other_1909 = other_1908;
    let _e4: PointAndPlane = self_2277;
    let _e5: Plane = other_1909;
    let _e6: Motor = point_and_plane_plane_geometric_product(_e4, _e5);
    let _e7: PointAndPlane = self_2277;
    let _e8: PointAndPlane = point_and_plane_reversal(_e7);
    let _e9: PointAndPlane = motor_point_and_plane_geometric_product(_e6, _e8);
    let _e10: Plane = point_and_plane_plane_into(_e9);
    return _e10;
}

fn point_and_plane_point_geometric_quotient(self_2278: PointAndPlane, other_1910: Point) -> Motor {
    var self_2279: PointAndPlane;
    var other_1911: Point;

    self_2279 = self_2278;
    other_1911 = other_1910;
    let _e4: PointAndPlane = self_2279;
    let _e5: Point = other_1911;
    let _e6: Point = point_inverse(_e5);
    let _e7: Motor = point_and_plane_point_geometric_product(_e4, _e6);
    return _e7;
}

fn point_and_plane_point_transformation(self_2280: PointAndPlane, other_1912: Point) -> Point {
    var self_2281: PointAndPlane;
    var other_1913: Point;

    self_2281 = self_2280;
    other_1913 = other_1912;
    let _e4: PointAndPlane = self_2281;
    let _e5: Point = other_1913;
    let _e6: Motor = point_and_plane_point_geometric_product(_e4, _e5);
    let _e7: PointAndPlane = self_2281;
    let _e8: PointAndPlane = point_and_plane_reversal(_e7);
    let _e9: PointAndPlane = motor_point_and_plane_geometric_product(_e6, _e8);
    let _e10: Point = point_and_plane_point_into(_e9);
    return _e10;
}

fn point_and_plane_point_and_plane_geometric_quotient(self_2282: PointAndPlane, other_1914: PointAndPlane) -> Motor {
    var self_2283: PointAndPlane;
    var other_1915: PointAndPlane;

    self_2283 = self_2282;
    other_1915 = other_1914;
    let _e4: PointAndPlane = self_2283;
    let _e5: PointAndPlane = other_1915;
    let _e6: PointAndPlane = point_and_plane_inverse(_e5);
    let _e7: Motor = point_and_plane_point_and_plane_geometric_product(_e4, _e6);
    return _e7;
}

fn point_and_plane_point_and_plane_transformation(self_2284: PointAndPlane, other_1916: PointAndPlane) -> PointAndPlane {
    var self_2285: PointAndPlane;
    var other_1917: PointAndPlane;

    self_2285 = self_2284;
    other_1917 = other_1916;
    let _e4: PointAndPlane = self_2285;
    let _e5: PointAndPlane = other_1917;
    let _e6: Motor = point_and_plane_point_and_plane_geometric_product(_e4, _e5);
    let _e7: PointAndPlane = self_2285;
    let _e8: PointAndPlane = point_and_plane_reversal(_e7);
    let _e9: PointAndPlane = motor_point_and_plane_geometric_product(_e6, _e8);
    return _e9;
}

fn point_and_plane_rotor_geometric_quotient(self_2286: PointAndPlane, other_1918: Rotor) -> PointAndPlane {
    var self_2287: PointAndPlane;
    var other_1919: Rotor;

    self_2287 = self_2286;
    other_1919 = other_1918;
    let _e4: PointAndPlane = self_2287;
    let _e5: Rotor = other_1919;
    let _e6: Rotor = rotor_inverse(_e5);
    let _e7: PointAndPlane = point_and_plane_rotor_geometric_product(_e4, _e6);
    return _e7;
}

fn point_and_plane_rotor_transformation(self_2288: PointAndPlane, other_1920: Rotor) -> Rotor {
    var self_2289: PointAndPlane;
    var other_1921: Rotor;

    self_2289 = self_2288;
    other_1921 = other_1920;
    let _e4: PointAndPlane = self_2289;
    let _e5: Rotor = other_1921;
    let _e6: PointAndPlane = point_and_plane_rotor_geometric_product(_e4, _e5);
    let _e7: PointAndPlane = self_2289;
    let _e8: PointAndPlane = point_and_plane_reversal(_e7);
    let _e9: Motor = point_and_plane_point_and_plane_geometric_product(_e6, _e8);
    let _e10: Rotor = motor_rotor_into(_e9);
    return _e10;
}

fn point_and_plane_scalar_geometric_quotient(self_2290: PointAndPlane, other_1922: Scalar) -> PointAndPlane {
    var self_2291: PointAndPlane;
    var other_1923: Scalar;

    self_2291 = self_2290;
    other_1923 = other_1922;
    let _e4: PointAndPlane = self_2291;
    let _e5: Scalar = other_1923;
    let _e6: Scalar = scalar_inverse(_e5);
    let _e7: PointAndPlane = point_and_plane_scalar_geometric_product(_e4, _e6);
    return _e7;
}

fn point_and_plane_scalar_transformation(self_2292: PointAndPlane, other_1924: Scalar) -> Scalar {
    var self_2293: PointAndPlane;
    var other_1925: Scalar;

    self_2293 = self_2292;
    other_1925 = other_1924;
    let _e4: PointAndPlane = self_2293;
    let _e5: Scalar = other_1925;
    let _e6: PointAndPlane = point_and_plane_scalar_geometric_product(_e4, _e5);
    let _e7: PointAndPlane = self_2293;
    let _e8: PointAndPlane = point_and_plane_reversal(_e7);
    let _e9: Motor = point_and_plane_point_and_plane_geometric_product(_e6, _e8);
    let _e10: Scalar = motor_scalar_into(_e9);
    return _e10;
}

fn point_and_plane_translator_geometric_quotient(self_2294: PointAndPlane, other_1926: Translator) -> PointAndPlane {
    var self_2295: PointAndPlane;
    var other_1927: Translator;

    self_2295 = self_2294;
    other_1927 = other_1926;
    let _e4: PointAndPlane = self_2295;
    let _e5: Translator = other_1927;
    let _e6: Translator = translator_inverse(_e5);
    let _e7: PointAndPlane = point_and_plane_translator_geometric_product(_e4, _e6);
    return _e7;
}

fn point_and_plane_translator_transformation(self_2296: PointAndPlane, other_1928: Translator) -> Translator {
    var self_2297: PointAndPlane;
    var other_1929: Translator;

    self_2297 = self_2296;
    other_1929 = other_1928;
    let _e4: PointAndPlane = self_2297;
    let _e5: Translator = other_1929;
    let _e6: PointAndPlane = point_and_plane_translator_geometric_product(_e4, _e5);
    let _e7: PointAndPlane = self_2297;
    let _e8: PointAndPlane = point_and_plane_reversal(_e7);
    let _e9: Motor = point_and_plane_point_and_plane_geometric_product(_e6, _e8);
    let _e10: Translator = motor_translator_into(_e9);
    return _e10;
}

fn rotor_line_geometric_quotient(self_2298: Rotor, other_1930: Line) -> Motor {
    var self_2299: Rotor;
    var other_1931: Line;

    self_2299 = self_2298;
    other_1931 = other_1930;
    let _e4: Rotor = self_2299;
    let _e5: Line = other_1931;
    let _e6: Line = line_inverse(_e5);
    let _e7: Motor = rotor_line_geometric_product(_e4, _e6);
    return _e7;
}

fn rotor_line_transformation(self_2300: Rotor, other_1932: Line) -> Line {
    var self_2301: Rotor;
    var other_1933: Line;

    self_2301 = self_2300;
    other_1933 = other_1932;
    let _e4: Rotor = self_2301;
    let _e5: Line = other_1933;
    let _e6: Motor = rotor_line_geometric_product(_e4, _e5);
    let _e7: Rotor = self_2301;
    let _e8: Rotor = rotor_reversal(_e7);
    let _e9: Motor = motor_rotor_geometric_product(_e6, _e8);
    let _e10: Line = motor_line_into(_e9);
    return _e10;
}

fn rotor_motor_geometric_quotient(self_2302: Rotor, other_1934: Motor) -> Motor {
    var self_2303: Rotor;
    var other_1935: Motor;

    self_2303 = self_2302;
    other_1935 = other_1934;
    let _e4: Rotor = self_2303;
    let _e5: Motor = other_1935;
    let _e6: Motor = motor_inverse(_e5);
    let _e7: Motor = rotor_motor_geometric_product(_e4, _e6);
    return _e7;
}

fn rotor_motor_transformation(self_2304: Rotor, other_1936: Motor) -> Motor {
    var self_2305: Rotor;
    var other_1937: Motor;

    self_2305 = self_2304;
    other_1937 = other_1936;
    let _e4: Rotor = self_2305;
    let _e5: Motor = other_1937;
    let _e6: Motor = rotor_motor_geometric_product(_e4, _e5);
    let _e7: Rotor = self_2305;
    let _e8: Rotor = rotor_reversal(_e7);
    let _e9: Motor = motor_rotor_geometric_product(_e6, _e8);
    return _e9;
}

fn rotor_multi_vector_geometric_quotient(self_2306: Rotor, other_1938: MultiVector) -> MultiVector {
    var self_2307: Rotor;
    var other_1939: MultiVector;

    self_2307 = self_2306;
    other_1939 = other_1938;
    let _e4: Rotor = self_2307;
    let _e5: MultiVector = other_1939;
    let _e6: MultiVector = multi_vector_inverse(_e5);
    let _e7: MultiVector = rotor_multi_vector_geometric_product(_e4, _e6);
    return _e7;
}

fn rotor_multi_vector_transformation(self_2308: Rotor, other_1940: MultiVector) -> MultiVector {
    var self_2309: Rotor;
    var other_1941: MultiVector;

    self_2309 = self_2308;
    other_1941 = other_1940;
    let _e4: Rotor = self_2309;
    let _e5: MultiVector = other_1941;
    let _e6: MultiVector = rotor_multi_vector_geometric_product(_e4, _e5);
    let _e7: Rotor = self_2309;
    let _e8: Rotor = rotor_reversal(_e7);
    let _e9: MultiVector = multi_vector_rotor_geometric_product(_e6, _e8);
    return _e9;
}

fn rotor_plane_geometric_quotient(self_2310: Rotor, other_1942: Plane) -> PointAndPlane {
    var self_2311: Rotor;
    var other_1943: Plane;

    self_2311 = self_2310;
    other_1943 = other_1942;
    let _e4: Rotor = self_2311;
    let _e5: Plane = other_1943;
    let _e6: Plane = plane_inverse(_e5);
    let _e7: PointAndPlane = rotor_plane_geometric_product(_e4, _e6);
    return _e7;
}

fn rotor_plane_transformation(self_2312: Rotor, other_1944: Plane) -> Plane {
    var self_2313: Rotor;
    var other_1945: Plane;

    self_2313 = self_2312;
    other_1945 = other_1944;
    let _e4: Rotor = self_2313;
    let _e5: Plane = other_1945;
    let _e6: PointAndPlane = rotor_plane_geometric_product(_e4, _e5);
    let _e7: Rotor = self_2313;
    let _e8: Rotor = rotor_reversal(_e7);
    let _e9: PointAndPlane = point_and_plane_rotor_geometric_product(_e6, _e8);
    let _e10: Plane = point_and_plane_plane_into(_e9);
    return _e10;
}

fn rotor_point_geometric_quotient(self_2314: Rotor, other_1946: Point) -> PointAndPlane {
    var self_2315: Rotor;
    var other_1947: Point;

    self_2315 = self_2314;
    other_1947 = other_1946;
    let _e4: Rotor = self_2315;
    let _e5: Point = other_1947;
    let _e6: Point = point_inverse(_e5);
    let _e7: PointAndPlane = rotor_point_geometric_product(_e4, _e6);
    return _e7;
}

fn rotor_point_transformation(self_2316: Rotor, other_1948: Point) -> Point {
    var self_2317: Rotor;
    var other_1949: Point;

    self_2317 = self_2316;
    other_1949 = other_1948;
    let _e4: Rotor = self_2317;
    let _e5: Point = other_1949;
    let _e6: PointAndPlane = rotor_point_geometric_product(_e4, _e5);
    let _e7: Rotor = self_2317;
    let _e8: Rotor = rotor_reversal(_e7);
    let _e9: PointAndPlane = point_and_plane_rotor_geometric_product(_e6, _e8);
    let _e10: Point = point_and_plane_point_into(_e9);
    return _e10;
}

fn rotor_point_and_plane_geometric_quotient(self_2318: Rotor, other_1950: PointAndPlane) -> PointAndPlane {
    var self_2319: Rotor;
    var other_1951: PointAndPlane;

    self_2319 = self_2318;
    other_1951 = other_1950;
    let _e4: Rotor = self_2319;
    let _e5: PointAndPlane = other_1951;
    let _e6: PointAndPlane = point_and_plane_inverse(_e5);
    let _e7: PointAndPlane = rotor_point_and_plane_geometric_product(_e4, _e6);
    return _e7;
}

fn rotor_point_and_plane_transformation(self_2320: Rotor, other_1952: PointAndPlane) -> PointAndPlane {
    var self_2321: Rotor;
    var other_1953: PointAndPlane;

    self_2321 = self_2320;
    other_1953 = other_1952;
    let _e4: Rotor = self_2321;
    let _e5: PointAndPlane = other_1953;
    let _e6: PointAndPlane = rotor_point_and_plane_geometric_product(_e4, _e5);
    let _e7: Rotor = self_2321;
    let _e8: Rotor = rotor_reversal(_e7);
    let _e9: PointAndPlane = point_and_plane_rotor_geometric_product(_e6, _e8);
    return _e9;
}

fn rotor_powi(self_2322: Rotor, exponent_4: i32) -> Rotor {
    var self_2323: Rotor;
    var exponent_5: i32;
    var local_2: Rotor;
    var x_2: Rotor;
    var y_2: Rotor;
    var n_2: i32;

    self_2323 = self_2322;
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
        let _e11: Rotor = self_2323;
        let _e12: Rotor = rotor_inverse(_e11);
        local_2 = _e12;
    } else {
        let _e14: Rotor = self_2323;
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

fn rotor_rotor_geometric_quotient(self_2324: Rotor, other_1954: Rotor) -> Rotor {
    var self_2325: Rotor;
    var other_1955: Rotor;

    self_2325 = self_2324;
    other_1955 = other_1954;
    let _e4: Rotor = self_2325;
    let _e5: Rotor = other_1955;
    let _e6: Rotor = rotor_inverse(_e5);
    let _e7: Rotor = rotor_rotor_geometric_product(_e4, _e6);
    return _e7;
}

fn rotor_rotor_transformation(self_2326: Rotor, other_1956: Rotor) -> Rotor {
    var self_2327: Rotor;
    var other_1957: Rotor;

    self_2327 = self_2326;
    other_1957 = other_1956;
    let _e4: Rotor = self_2327;
    let _e5: Rotor = other_1957;
    let _e6: Rotor = rotor_rotor_geometric_product(_e4, _e5);
    let _e7: Rotor = self_2327;
    let _e8: Rotor = rotor_reversal(_e7);
    let _e9: Rotor = rotor_rotor_geometric_product(_e6, _e8);
    return _e9;
}

fn rotor_scalar_geometric_quotient(self_2328: Rotor, other_1958: Scalar) -> Rotor {
    var self_2329: Rotor;
    var other_1959: Scalar;

    self_2329 = self_2328;
    other_1959 = other_1958;
    let _e4: Rotor = self_2329;
    let _e5: Scalar = other_1959;
    let _e6: Scalar = scalar_inverse(_e5);
    let _e7: Rotor = rotor_scalar_geometric_product(_e4, _e6);
    return _e7;
}

fn rotor_scalar_transformation(self_2330: Rotor, other_1960: Scalar) -> Scalar {
    var self_2331: Rotor;
    var other_1961: Scalar;

    self_2331 = self_2330;
    other_1961 = other_1960;
    let _e4: Rotor = self_2331;
    let _e5: Scalar = other_1961;
    let _e6: Rotor = rotor_scalar_geometric_product(_e4, _e5);
    let _e7: Rotor = self_2331;
    let _e8: Rotor = rotor_reversal(_e7);
    let _e9: Rotor = rotor_rotor_geometric_product(_e6, _e8);
    let _e10: Scalar = rotor_scalar_into(_e9);
    return _e10;
}

fn rotor_translator_geometric_quotient(self_2332: Rotor, other_1962: Translator) -> Motor {
    var self_2333: Rotor;
    var other_1963: Translator;

    self_2333 = self_2332;
    other_1963 = other_1962;
    let _e4: Rotor = self_2333;
    let _e5: Translator = other_1963;
    let _e6: Translator = translator_inverse(_e5);
    let _e7: Motor = rotor_translator_geometric_product(_e4, _e6);
    return _e7;
}

fn rotor_translator_transformation(self_2334: Rotor, other_1964: Translator) -> Translator {
    var self_2335: Rotor;
    var other_1965: Translator;

    self_2335 = self_2334;
    other_1965 = other_1964;
    let _e4: Rotor = self_2335;
    let _e5: Translator = other_1965;
    let _e6: Motor = rotor_translator_geometric_product(_e4, _e5);
    let _e7: Rotor = self_2335;
    let _e8: Rotor = rotor_reversal(_e7);
    let _e9: Motor = motor_rotor_geometric_product(_e6, _e8);
    let _e10: Translator = motor_translator_into(_e9);
    return _e10;
}

fn scalar_anti_scalar_transformation(self_2336: Scalar, other_1966: AntiScalar) -> AntiScalar {
    var self_2337: Scalar;
    var other_1967: AntiScalar;

    self_2337 = self_2336;
    other_1967 = other_1966;
    let _e4: Scalar = self_2337;
    let _e5: AntiScalar = other_1967;
    let _e6: AntiScalar = scalar_anti_scalar_geometric_product(_e4, _e5);
    let _e7: Scalar = self_2337;
    let _e8: Scalar = scalar_reversal(_e7);
    let _e9: AntiScalar = anti_scalar_scalar_geometric_product(_e6, _e8);
    return _e9;
}

fn scalar_ideal_point_transformation(self_2338: Scalar, other_1968: IdealPoint) -> IdealPoint {
    var self_2339: Scalar;
    var other_1969: IdealPoint;

    self_2339 = self_2338;
    other_1969 = other_1968;
    let _e4: Scalar = self_2339;
    let _e5: IdealPoint = other_1969;
    let _e6: IdealPoint = scalar_ideal_point_geometric_product(_e4, _e5);
    let _e7: Scalar = self_2339;
    let _e8: Scalar = scalar_reversal(_e7);
    let _e9: IdealPoint = ideal_point_scalar_geometric_product(_e6, _e8);
    return _e9;
}

fn scalar_line_geometric_quotient(self_2340: Scalar, other_1970: Line) -> Line {
    var self_2341: Scalar;
    var other_1971: Line;

    self_2341 = self_2340;
    other_1971 = other_1970;
    let _e4: Scalar = self_2341;
    let _e5: Line = other_1971;
    let _e6: Line = line_inverse(_e5);
    let _e7: Line = scalar_line_geometric_product(_e4, _e6);
    return _e7;
}

fn scalar_line_transformation(self_2342: Scalar, other_1972: Line) -> Line {
    var self_2343: Scalar;
    var other_1973: Line;

    self_2343 = self_2342;
    other_1973 = other_1972;
    let _e4: Scalar = self_2343;
    let _e5: Line = other_1973;
    let _e6: Line = scalar_line_geometric_product(_e4, _e5);
    let _e7: Scalar = self_2343;
    let _e8: Scalar = scalar_reversal(_e7);
    let _e9: Line = line_scalar_geometric_product(_e6, _e8);
    return _e9;
}

fn scalar_motor_geometric_quotient(self_2344: Scalar, other_1974: Motor) -> Motor {
    var self_2345: Scalar;
    var other_1975: Motor;

    self_2345 = self_2344;
    other_1975 = other_1974;
    let _e4: Scalar = self_2345;
    let _e5: Motor = other_1975;
    let _e6: Motor = motor_inverse(_e5);
    let _e7: Motor = scalar_motor_geometric_product(_e4, _e6);
    return _e7;
}

fn scalar_motor_transformation(self_2346: Scalar, other_1976: Motor) -> Motor {
    var self_2347: Scalar;
    var other_1977: Motor;

    self_2347 = self_2346;
    other_1977 = other_1976;
    let _e4: Scalar = self_2347;
    let _e5: Motor = other_1977;
    let _e6: Motor = scalar_motor_geometric_product(_e4, _e5);
    let _e7: Scalar = self_2347;
    let _e8: Scalar = scalar_reversal(_e7);
    let _e9: Motor = motor_scalar_geometric_product(_e6, _e8);
    return _e9;
}

fn scalar_multi_vector_geometric_quotient(self_2348: Scalar, other_1978: MultiVector) -> MultiVector {
    var self_2349: Scalar;
    var other_1979: MultiVector;

    self_2349 = self_2348;
    other_1979 = other_1978;
    let _e4: Scalar = self_2349;
    let _e5: MultiVector = other_1979;
    let _e6: MultiVector = multi_vector_inverse(_e5);
    let _e7: MultiVector = scalar_multi_vector_geometric_product(_e4, _e6);
    return _e7;
}

fn scalar_multi_vector_transformation(self_2350: Scalar, other_1980: MultiVector) -> MultiVector {
    var self_2351: Scalar;
    var other_1981: MultiVector;

    self_2351 = self_2350;
    other_1981 = other_1980;
    let _e4: Scalar = self_2351;
    let _e5: MultiVector = other_1981;
    let _e6: MultiVector = scalar_multi_vector_geometric_product(_e4, _e5);
    let _e7: Scalar = self_2351;
    let _e8: Scalar = scalar_reversal(_e7);
    let _e9: MultiVector = multi_vector_scalar_geometric_product(_e6, _e8);
    return _e9;
}

fn scalar_plane_geometric_quotient(self_2352: Scalar, other_1982: Plane) -> Plane {
    var self_2353: Scalar;
    var other_1983: Plane;

    self_2353 = self_2352;
    other_1983 = other_1982;
    let _e4: Scalar = self_2353;
    let _e5: Plane = other_1983;
    let _e6: Plane = plane_inverse(_e5);
    let _e7: Plane = scalar_plane_geometric_product(_e4, _e6);
    return _e7;
}

fn scalar_plane_transformation(self_2354: Scalar, other_1984: Plane) -> Plane {
    var self_2355: Scalar;
    var other_1985: Plane;

    self_2355 = self_2354;
    other_1985 = other_1984;
    let _e4: Scalar = self_2355;
    let _e5: Plane = other_1985;
    let _e6: Plane = scalar_plane_geometric_product(_e4, _e5);
    let _e7: Scalar = self_2355;
    let _e8: Scalar = scalar_reversal(_e7);
    let _e9: Plane = plane_scalar_geometric_product(_e6, _e8);
    return _e9;
}

fn scalar_point_geometric_quotient(self_2356: Scalar, other_1986: Point) -> Point {
    var self_2357: Scalar;
    var other_1987: Point;

    self_2357 = self_2356;
    other_1987 = other_1986;
    let _e4: Scalar = self_2357;
    let _e5: Point = other_1987;
    let _e6: Point = point_inverse(_e5);
    let _e7: Point = scalar_point_geometric_product(_e4, _e6);
    return _e7;
}

fn scalar_point_transformation(self_2358: Scalar, other_1988: Point) -> Point {
    var self_2359: Scalar;
    var other_1989: Point;

    self_2359 = self_2358;
    other_1989 = other_1988;
    let _e4: Scalar = self_2359;
    let _e5: Point = other_1989;
    let _e6: Point = scalar_point_geometric_product(_e4, _e5);
    let _e7: Scalar = self_2359;
    let _e8: Scalar = scalar_reversal(_e7);
    let _e9: Point = point_scalar_geometric_product(_e6, _e8);
    return _e9;
}

fn scalar_point_and_plane_geometric_quotient(self_2360: Scalar, other_1990: PointAndPlane) -> PointAndPlane {
    var self_2361: Scalar;
    var other_1991: PointAndPlane;

    self_2361 = self_2360;
    other_1991 = other_1990;
    let _e4: Scalar = self_2361;
    let _e5: PointAndPlane = other_1991;
    let _e6: PointAndPlane = point_and_plane_inverse(_e5);
    let _e7: PointAndPlane = scalar_point_and_plane_geometric_product(_e4, _e6);
    return _e7;
}

fn scalar_point_and_plane_transformation(self_2362: Scalar, other_1992: PointAndPlane) -> PointAndPlane {
    var self_2363: Scalar;
    var other_1993: PointAndPlane;

    self_2363 = self_2362;
    other_1993 = other_1992;
    let _e4: Scalar = self_2363;
    let _e5: PointAndPlane = other_1993;
    let _e6: PointAndPlane = scalar_point_and_plane_geometric_product(_e4, _e5);
    let _e7: Scalar = self_2363;
    let _e8: Scalar = scalar_reversal(_e7);
    let _e9: PointAndPlane = point_and_plane_scalar_geometric_product(_e6, _e8);
    return _e9;
}

fn scalar_rotor_geometric_quotient(self_2364: Scalar, other_1994: Rotor) -> Rotor {
    var self_2365: Scalar;
    var other_1995: Rotor;

    self_2365 = self_2364;
    other_1995 = other_1994;
    let _e4: Scalar = self_2365;
    let _e5: Rotor = other_1995;
    let _e6: Rotor = rotor_inverse(_e5);
    let _e7: Rotor = scalar_rotor_geometric_product(_e4, _e6);
    return _e7;
}

fn scalar_rotor_transformation(self_2366: Scalar, other_1996: Rotor) -> Rotor {
    var self_2367: Scalar;
    var other_1997: Rotor;

    self_2367 = self_2366;
    other_1997 = other_1996;
    let _e4: Scalar = self_2367;
    let _e5: Rotor = other_1997;
    let _e6: Rotor = scalar_rotor_geometric_product(_e4, _e5);
    let _e7: Scalar = self_2367;
    let _e8: Scalar = scalar_reversal(_e7);
    let _e9: Rotor = rotor_scalar_geometric_product(_e6, _e8);
    return _e9;
}

fn scalar_powi(self_2368: Scalar, exponent_6: i32) -> Scalar {
    var self_2369: Scalar;
    var exponent_7: i32;
    var local_3: Scalar;
    var x_3: Scalar;
    var y_3: Scalar;
    var n_3: i32;

    self_2369 = self_2368;
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
        let _e11: Scalar = self_2369;
        let _e12: Scalar = scalar_inverse(_e11);
        local_3 = _e12;
    } else {
        let _e14: Scalar = self_2369;
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

fn scalar_scalar_geometric_quotient(self_2370: Scalar, other_1998: Scalar) -> Scalar {
    var self_2371: Scalar;
    var other_1999: Scalar;

    self_2371 = self_2370;
    other_1999 = other_1998;
    let _e4: Scalar = self_2371;
    let _e5: Scalar = other_1999;
    let _e6: Scalar = scalar_inverse(_e5);
    let _e7: Scalar = scalar_scalar_geometric_product(_e4, _e6);
    return _e7;
}

fn scalar_scalar_transformation(self_2372: Scalar, other_2000: Scalar) -> Scalar {
    var self_2373: Scalar;
    var other_2001: Scalar;

    self_2373 = self_2372;
    other_2001 = other_2000;
    let _e4: Scalar = self_2373;
    let _e5: Scalar = other_2001;
    let _e6: Scalar = scalar_scalar_geometric_product(_e4, _e5);
    let _e7: Scalar = self_2373;
    let _e8: Scalar = scalar_reversal(_e7);
    let _e9: Scalar = scalar_scalar_geometric_product(_e6, _e8);
    return _e9;
}

fn scalar_translator_geometric_quotient(self_2374: Scalar, other_2002: Translator) -> Translator {
    var self_2375: Scalar;
    var other_2003: Translator;

    self_2375 = self_2374;
    other_2003 = other_2002;
    let _e4: Scalar = self_2375;
    let _e5: Translator = other_2003;
    let _e6: Translator = translator_inverse(_e5);
    let _e7: Translator = scalar_translator_geometric_product(_e4, _e6);
    return _e7;
}

fn scalar_translator_transformation(self_2376: Scalar, other_2004: Translator) -> Translator {
    var self_2377: Scalar;
    var other_2005: Translator;

    self_2377 = self_2376;
    other_2005 = other_2004;
    let _e4: Scalar = self_2377;
    let _e5: Translator = other_2005;
    let _e6: Translator = scalar_translator_geometric_product(_e4, _e5);
    let _e7: Scalar = self_2377;
    let _e8: Scalar = scalar_reversal(_e7);
    let _e9: Translator = translator_scalar_geometric_product(_e6, _e8);
    return _e9;
}

fn translator_anti_scalar_transformation(self_2378: Translator, other_2006: AntiScalar) -> AntiScalar {
    var self_2379: Translator;
    var other_2007: AntiScalar;

    self_2379 = self_2378;
    other_2007 = other_2006;
    let _e4: Translator = self_2379;
    let _e5: AntiScalar = other_2007;
    let _e6: AntiScalar = translator_anti_scalar_geometric_product(_e4, _e5);
    let _e7: Translator = self_2379;
    let _e8: Translator = translator_reversal(_e7);
    let _e9: AntiScalar = anti_scalar_translator_geometric_product(_e6, _e8);
    return _e9;
}

fn translator_ideal_point_transformation(self_2380: Translator, other_2008: IdealPoint) -> IdealPoint {
    var self_2381: Translator;
    var other_2009: IdealPoint;

    self_2381 = self_2380;
    other_2009 = other_2008;
    let _e4: Translator = self_2381;
    let _e5: IdealPoint = other_2009;
    let _e6: IdealPoint = translator_ideal_point_geometric_product(_e4, _e5);
    let _e7: Translator = self_2381;
    let _e8: Translator = translator_reversal(_e7);
    let _e9: IdealPoint = ideal_point_translator_geometric_product(_e6, _e8);
    return _e9;
}

fn translator_motor_geometric_quotient(self_2382: Translator, other_2010: Motor) -> Motor {
    var self_2383: Translator;
    var other_2011: Motor;

    self_2383 = self_2382;
    other_2011 = other_2010;
    let _e4: Translator = self_2383;
    let _e5: Motor = other_2011;
    let _e6: Motor = motor_inverse(_e5);
    let _e7: Motor = translator_motor_geometric_product(_e4, _e6);
    return _e7;
}

fn translator_motor_transformation(self_2384: Translator, other_2012: Motor) -> Motor {
    var self_2385: Translator;
    var other_2013: Motor;

    self_2385 = self_2384;
    other_2013 = other_2012;
    let _e4: Translator = self_2385;
    let _e5: Motor = other_2013;
    let _e6: Motor = translator_motor_geometric_product(_e4, _e5);
    let _e7: Translator = self_2385;
    let _e8: Translator = translator_reversal(_e7);
    let _e9: Motor = motor_translator_geometric_product(_e6, _e8);
    return _e9;
}

fn translator_multi_vector_geometric_quotient(self_2386: Translator, other_2014: MultiVector) -> MultiVector {
    var self_2387: Translator;
    var other_2015: MultiVector;

    self_2387 = self_2386;
    other_2015 = other_2014;
    let _e4: Translator = self_2387;
    let _e5: MultiVector = other_2015;
    let _e6: MultiVector = multi_vector_inverse(_e5);
    let _e7: MultiVector = translator_multi_vector_geometric_product(_e4, _e6);
    return _e7;
}

fn translator_multi_vector_transformation(self_2388: Translator, other_2016: MultiVector) -> MultiVector {
    var self_2389: Translator;
    var other_2017: MultiVector;

    self_2389 = self_2388;
    other_2017 = other_2016;
    let _e4: Translator = self_2389;
    let _e5: MultiVector = other_2017;
    let _e6: MultiVector = translator_multi_vector_geometric_product(_e4, _e5);
    let _e7: Translator = self_2389;
    let _e8: Translator = translator_reversal(_e7);
    let _e9: MultiVector = multi_vector_translator_geometric_product(_e6, _e8);
    return _e9;
}

fn translator_point_geometric_quotient(self_2390: Translator, other_2018: Point) -> Point {
    var self_2391: Translator;
    var other_2019: Point;

    self_2391 = self_2390;
    other_2019 = other_2018;
    let _e4: Translator = self_2391;
    let _e5: Point = other_2019;
    let _e6: Point = point_inverse(_e5);
    let _e7: Point = translator_point_geometric_product(_e4, _e6);
    return _e7;
}

fn translator_point_transformation(self_2392: Translator, other_2020: Point) -> Point {
    var self_2393: Translator;
    var other_2021: Point;

    self_2393 = self_2392;
    other_2021 = other_2020;
    let _e4: Translator = self_2393;
    let _e5: Point = other_2021;
    let _e6: Point = translator_point_geometric_product(_e4, _e5);
    let _e7: Translator = self_2393;
    let _e8: Translator = translator_reversal(_e7);
    let _e9: Point = point_translator_geometric_product(_e6, _e8);
    return _e9;
}

fn translator_point_and_plane_geometric_quotient(self_2394: Translator, other_2022: PointAndPlane) -> PointAndPlane {
    var self_2395: Translator;
    var other_2023: PointAndPlane;

    self_2395 = self_2394;
    other_2023 = other_2022;
    let _e4: Translator = self_2395;
    let _e5: PointAndPlane = other_2023;
    let _e6: PointAndPlane = point_and_plane_inverse(_e5);
    let _e7: PointAndPlane = translator_point_and_plane_geometric_product(_e4, _e6);
    return _e7;
}

fn translator_point_and_plane_transformation(self_2396: Translator, other_2024: PointAndPlane) -> PointAndPlane {
    var self_2397: Translator;
    var other_2025: PointAndPlane;

    self_2397 = self_2396;
    other_2025 = other_2024;
    let _e4: Translator = self_2397;
    let _e5: PointAndPlane = other_2025;
    let _e6: PointAndPlane = translator_point_and_plane_geometric_product(_e4, _e5);
    let _e7: Translator = self_2397;
    let _e8: Translator = translator_reversal(_e7);
    let _e9: PointAndPlane = point_and_plane_translator_geometric_product(_e6, _e8);
    return _e9;
}

fn translator_rotor_geometric_quotient(self_2398: Translator, other_2026: Rotor) -> Motor {
    var self_2399: Translator;
    var other_2027: Rotor;

    self_2399 = self_2398;
    other_2027 = other_2026;
    let _e4: Translator = self_2399;
    let _e5: Rotor = other_2027;
    let _e6: Rotor = rotor_inverse(_e5);
    let _e7: Motor = translator_rotor_geometric_product(_e4, _e6);
    return _e7;
}

fn translator_rotor_transformation(self_2400: Translator, other_2028: Rotor) -> Rotor {
    var self_2401: Translator;
    var other_2029: Rotor;

    self_2401 = self_2400;
    other_2029 = other_2028;
    let _e4: Translator = self_2401;
    let _e5: Rotor = other_2029;
    let _e6: Motor = translator_rotor_geometric_product(_e4, _e5);
    let _e7: Translator = self_2401;
    let _e8: Translator = translator_reversal(_e7);
    let _e9: Motor = motor_translator_geometric_product(_e6, _e8);
    let _e10: Rotor = motor_rotor_into(_e9);
    return _e10;
}

fn translator_scalar_geometric_quotient(self_2402: Translator, other_2030: Scalar) -> Translator {
    var self_2403: Translator;
    var other_2031: Scalar;

    self_2403 = self_2402;
    other_2031 = other_2030;
    let _e4: Translator = self_2403;
    let _e5: Scalar = other_2031;
    let _e6: Scalar = scalar_inverse(_e5);
    let _e7: Translator = translator_scalar_geometric_product(_e4, _e6);
    return _e7;
}

fn translator_scalar_transformation(self_2404: Translator, other_2032: Scalar) -> Scalar {
    var self_2405: Translator;
    var other_2033: Scalar;

    self_2405 = self_2404;
    other_2033 = other_2032;
    let _e4: Translator = self_2405;
    let _e5: Scalar = other_2033;
    let _e6: Translator = translator_scalar_geometric_product(_e4, _e5);
    let _e7: Translator = self_2405;
    let _e8: Translator = translator_reversal(_e7);
    let _e9: Translator = translator_translator_geometric_product(_e6, _e8);
    let _e10: Scalar = translator_scalar_into(_e9);
    return _e10;
}

fn translator_powi(self_2406: Translator, exponent_8: i32) -> Translator {
    var self_2407: Translator;
    var exponent_9: i32;
    var local_4: Translator;
    var x_4: Translator;
    var y_4: Translator;
    var n_4: i32;

    self_2407 = self_2406;
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
        let _e11: Translator = self_2407;
        let _e12: Translator = translator_inverse(_e11);
        local_4 = _e12;
    } else {
        let _e14: Translator = self_2407;
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

fn translator_translator_geometric_quotient(self_2408: Translator, other_2034: Translator) -> Translator {
    var self_2409: Translator;
    var other_2035: Translator;

    self_2409 = self_2408;
    other_2035 = other_2034;
    let _e4: Translator = self_2409;
    let _e5: Translator = other_2035;
    let _e6: Translator = translator_inverse(_e5);
    let _e7: Translator = translator_translator_geometric_product(_e4, _e6);
    return _e7;
}

fn translator_translator_transformation(self_2410: Translator, other_2036: Translator) -> Translator {
    var self_2411: Translator;
    var other_2037: Translator;

    self_2411 = self_2410;
    other_2037 = other_2036;
    let _e4: Translator = self_2411;
    let _e5: Translator = other_2037;
    let _e6: Translator = translator_translator_geometric_product(_e4, _e5);
    let _e7: Translator = self_2411;
    let _e8: Translator = translator_reversal(_e7);
    let _e9: Translator = translator_translator_geometric_product(_e6, _e8);
    return _e9;
}


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

struct Plane {
    g0_: vec3<f32>,
}

struct IdealPoint {
    g0_: vec2<f32>,
}

struct Translator {
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

fn scalar_anti_reversal(self_10: Scalar) -> Scalar {
    var self_11: Scalar;

    self_11 = self_10;
    let _e2: Scalar = self_11;
    return Scalar(_e2.g0_);
}

fn scalar_scalar_add(self_12: Scalar, other: Scalar) -> Scalar {
    var self_13: Scalar;
    var other_1: Scalar;

    self_13 = self_12;
    other_1 = other;
    let _e4: Scalar = self_13;
    let _e6: Scalar = other_1;
    return Scalar((_e4.g0_ + _e6.g0_));
}

fn scalar_scalar_sub(self_14: Scalar, other_2: Scalar) -> Scalar {
    var self_15: Scalar;
    var other_3: Scalar;

    self_15 = self_14;
    other_3 = other_2;
    let _e4: Scalar = self_15;
    let _e6: Scalar = other_3;
    return Scalar((_e4.g0_ - _e6.g0_));
}

fn scalar_scalar_mul(self_16: Scalar, other_4: Scalar) -> Scalar {
    var self_17: Scalar;
    var other_5: Scalar;

    self_17 = self_16;
    other_5 = other_4;
    let _e4: Scalar = self_17;
    let _e6: Scalar = other_5;
    return Scalar((_e4.g0_ * _e6.g0_));
}

fn scalar_scalar_div(self_18: Scalar, other_6: Scalar) -> Scalar {
    var self_19: Scalar;
    var other_7: Scalar;

    self_19 = self_18;
    other_7 = other_6;
    let _e4: Scalar = self_19;
    let _e8: Scalar = other_7;
    return Scalar((((_e4.g0_ * 1.0) / _e8.g0_) * 1.0));
}

fn scalar_scalar_geometric_product(self_20: Scalar, other_8: Scalar) -> Scalar {
    var self_21: Scalar;
    var other_9: Scalar;

    self_21 = self_20;
    other_9 = other_8;
    let _e4: Scalar = self_21;
    let _e6: Scalar = other_9;
    return Scalar((_e4.g0_ * _e6.g0_));
}

fn scalar_scalar_outer_product(self_22: Scalar, other_10: Scalar) -> Scalar {
    var self_23: Scalar;
    var other_11: Scalar;

    self_23 = self_22;
    other_11 = other_10;
    let _e4: Scalar = self_23;
    let _e6: Scalar = other_11;
    return Scalar((_e4.g0_ * _e6.g0_));
}

fn scalar_scalar_inner_product(self_24: Scalar, other_12: Scalar) -> Scalar {
    var self_25: Scalar;
    var other_13: Scalar;

    self_25 = self_24;
    other_13 = other_12;
    let _e4: Scalar = self_25;
    let _e6: Scalar = other_13;
    return Scalar((_e4.g0_ * _e6.g0_));
}

fn scalar_scalar_geometric_anti_product(self_26: Scalar, other_14: Scalar) -> AntiScalar {
    var self_27: Scalar;
    var other_15: Scalar;

    self_27 = self_26;
    other_15 = other_14;
    let _e5: Scalar = self_27;
    let _e7: Scalar = other_15;
    return AntiScalar((0.0 - (_e5.g0_ * _e7.g0_)));
}

fn scalar_scalar_inner_anti_product(self_28: Scalar, other_16: Scalar) -> AntiScalar {
    var self_29: Scalar;
    var other_17: Scalar;

    self_29 = self_28;
    other_17 = other_16;
    let _e5: Scalar = self_29;
    let _e7: Scalar = other_17;
    return AntiScalar((0.0 - (_e5.g0_ * _e7.g0_)));
}

fn scalar_scalar_left_contraction(self_30: Scalar, other_18: Scalar) -> Scalar {
    var self_31: Scalar;
    var other_19: Scalar;

    self_31 = self_30;
    other_19 = other_18;
    let _e4: Scalar = self_31;
    let _e6: Scalar = other_19;
    return Scalar((_e4.g0_ * _e6.g0_));
}

fn scalar_scalar_right_contraction(self_32: Scalar, other_20: Scalar) -> Scalar {
    var self_33: Scalar;
    var other_21: Scalar;

    self_33 = self_32;
    other_21 = other_20;
    let _e4: Scalar = self_33;
    let _e6: Scalar = other_21;
    return Scalar((_e4.g0_ * _e6.g0_));
}

fn scalar_scalar_left_anti_contraction(self_34: Scalar, other_22: Scalar) -> AntiScalar {
    var self_35: Scalar;
    var other_23: Scalar;

    self_35 = self_34;
    other_23 = other_22;
    let _e5: Scalar = self_35;
    let _e7: Scalar = other_23;
    return AntiScalar((0.0 - (_e5.g0_ * _e7.g0_)));
}

fn scalar_scalar_right_anti_contraction(self_36: Scalar, other_24: Scalar) -> AntiScalar {
    var self_37: Scalar;
    var other_25: Scalar;

    self_37 = self_36;
    other_25 = other_24;
    let _e5: Scalar = self_37;
    let _e7: Scalar = other_25;
    return AntiScalar((0.0 - (_e5.g0_ * _e7.g0_)));
}

fn scalar_scalar_scalar_product(self_38: Scalar, other_26: Scalar) -> Scalar {
    var self_39: Scalar;
    var other_27: Scalar;

    self_39 = self_38;
    other_27 = other_26;
    let _e4: Scalar = self_39;
    let _e6: Scalar = other_27;
    return Scalar((_e4.g0_ * _e6.g0_));
}

fn scalar_scalar_anti_scalar_product(self_40: Scalar, other_28: Scalar) -> AntiScalar {
    var self_41: Scalar;
    var other_29: Scalar;

    self_41 = self_40;
    other_29 = other_28;
    let _e5: Scalar = self_41;
    let _e7: Scalar = other_29;
    return AntiScalar((0.0 - (_e5.g0_ * _e7.g0_)));
}

fn scalar_anti_scalar_geometric_product(self_42: Scalar, other_30: AntiScalar) -> AntiScalar {
    var self_43: Scalar;
    var other_31: AntiScalar;

    self_43 = self_42;
    other_31 = other_30;
    let _e4: Scalar = self_43;
    let _e6: AntiScalar = other_31;
    return AntiScalar((_e4.g0_ * _e6.g0_));
}

fn scalar_anti_scalar_regressive_product(self_44: Scalar, other_32: AntiScalar) -> Scalar {
    var self_45: Scalar;
    var other_33: AntiScalar;

    self_45 = self_44;
    other_33 = other_32;
    let _e4: Scalar = self_45;
    let _e6: AntiScalar = other_33;
    return Scalar((_e4.g0_ * _e6.g0_));
}

fn scalar_anti_scalar_outer_product(self_46: Scalar, other_34: AntiScalar) -> AntiScalar {
    var self_47: Scalar;
    var other_35: AntiScalar;

    self_47 = self_46;
    other_35 = other_34;
    let _e4: Scalar = self_47;
    let _e6: AntiScalar = other_35;
    return AntiScalar((_e4.g0_ * _e6.g0_));
}

fn scalar_anti_scalar_inner_product(self_48: Scalar, other_36: AntiScalar) -> AntiScalar {
    var self_49: Scalar;
    var other_37: AntiScalar;

    self_49 = self_48;
    other_37 = other_36;
    let _e4: Scalar = self_49;
    let _e6: AntiScalar = other_37;
    return AntiScalar((_e4.g0_ * _e6.g0_));
}

fn scalar_anti_scalar_geometric_anti_product(self_50: Scalar, other_38: AntiScalar) -> Scalar {
    var self_51: Scalar;
    var other_39: AntiScalar;

    self_51 = self_50;
    other_39 = other_38;
    let _e4: Scalar = self_51;
    let _e6: AntiScalar = other_39;
    return Scalar((_e4.g0_ * _e6.g0_));
}

fn scalar_anti_scalar_inner_anti_product(self_52: Scalar, other_40: AntiScalar) -> Scalar {
    var self_53: Scalar;
    var other_41: AntiScalar;

    self_53 = self_52;
    other_41 = other_40;
    let _e4: Scalar = self_53;
    let _e6: AntiScalar = other_41;
    return Scalar((_e4.g0_ * _e6.g0_));
}

fn scalar_anti_scalar_left_contraction(self_54: Scalar, other_42: AntiScalar) -> AntiScalar {
    var self_55: Scalar;
    var other_43: AntiScalar;

    self_55 = self_54;
    other_43 = other_42;
    let _e4: Scalar = self_55;
    let _e6: AntiScalar = other_43;
    return AntiScalar((_e4.g0_ * _e6.g0_));
}

fn scalar_anti_scalar_right_anti_contraction(self_56: Scalar, other_44: AntiScalar) -> Scalar {
    var self_57: Scalar;
    var other_45: AntiScalar;

    self_57 = self_56;
    other_45 = other_44;
    let _e4: Scalar = self_57;
    let _e6: AntiScalar = other_45;
    return Scalar((_e4.g0_ * _e6.g0_));
}

fn scalar_multi_vector_add(self_58: Scalar, other_46: MultiVector) -> MultiVector {
    var self_59: Scalar;
    var other_47: MultiVector;

    self_59 = self_58;
    other_47 = other_46;
    let _e4: Scalar = self_59;
    let _e13: MultiVector = other_47;
    let _e16: MultiVector = other_47;
    return MultiVector(((vec4<f32>(_e4.g0_) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) + _e13.g0_), _e16.g1_);
}

fn scalar_multi_vector_sub(self_60: Scalar, other_48: MultiVector) -> MultiVector {
    var self_61: Scalar;
    var other_49: MultiVector;

    self_61 = self_60;
    other_49 = other_48;
    let _e4: Scalar = self_61;
    let _e13: MultiVector = other_49;
    let _e18: MultiVector = other_49;
    return MultiVector(((vec4<f32>(_e4.g0_) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) - _e13.g0_), (vec4<f32>(0.0) - _e18.g1_));
}

fn scalar_multi_vector_geometric_product(self_62: Scalar, other_50: MultiVector) -> MultiVector {
    var self_63: Scalar;
    var other_51: MultiVector;

    self_63 = self_62;
    other_51 = other_50;
    let _e4: Scalar = self_63;
    let _e7: MultiVector = other_51;
    let _e10: Scalar = self_63;
    let _e13: MultiVector = other_51;
    return MultiVector((vec4<f32>(_e4.g0_) * _e7.g0_), (vec4<f32>(_e10.g0_) * _e13.g1_));
}

fn scalar_multi_vector_regressive_product(self_64: Scalar, other_52: MultiVector) -> Scalar {
    var self_65: Scalar;
    var other_53: MultiVector;

    self_65 = self_64;
    other_53 = other_52;
    let _e4: Scalar = self_65;
    let _e6: MultiVector = other_53;
    return Scalar((_e4.g0_ * _e6.g1_.y));
}

fn scalar_multi_vector_outer_product(self_66: Scalar, other_54: MultiVector) -> MultiVector {
    var self_67: Scalar;
    var other_55: MultiVector;

    self_67 = self_66;
    other_55 = other_54;
    let _e4: Scalar = self_67;
    let _e7: MultiVector = other_55;
    let _e10: Scalar = self_67;
    let _e13: MultiVector = other_55;
    return MultiVector((vec4<f32>(_e4.g0_) * _e7.g0_), (vec4<f32>(_e10.g0_) * _e13.g1_));
}

fn scalar_multi_vector_inner_product(self_68: Scalar, other_56: MultiVector) -> MultiVector {
    var self_69: Scalar;
    var other_57: MultiVector;

    self_69 = self_68;
    other_57 = other_56;
    let _e4: Scalar = self_69;
    let _e7: MultiVector = other_57;
    let _e10: Scalar = self_69;
    let _e13: MultiVector = other_57;
    return MultiVector((vec4<f32>(_e4.g0_) * _e7.g0_), (vec4<f32>(_e10.g0_) * _e13.g1_));
}

fn scalar_multi_vector_geometric_anti_product(self_70: Scalar, other_58: MultiVector) -> MultiVector {
    var self_71: Scalar;
    var other_59: MultiVector;

    self_71 = self_70;
    other_59 = other_58;
    let _e4: Scalar = self_71;
    let _e7: MultiVector = other_59;
    let _e18: Scalar = self_71;
    let _e21: MultiVector = other_59;
    return MultiVector(((vec4<f32>(_e4.g0_) * _e7.g1_.yxwz) * vec4<f32>(1.0, -(1.0), 1.0, 1.0)), ((vec4<f32>(_e18.g0_) * _e21.g0_.yxwz) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))));
}

fn scalar_multi_vector_inner_anti_product(self_72: Scalar, other_60: MultiVector) -> MultiVector {
    var self_73: Scalar;
    var other_61: MultiVector;

    self_73 = self_72;
    other_61 = other_60;
    let _e4: Scalar = self_73;
    let _e7: MultiVector = other_61;
    let _e18: Scalar = self_73;
    let _e21: MultiVector = other_61;
    return MultiVector(((vec4<f32>(_e4.g0_) * _e7.g1_.yxwz) * vec4<f32>(1.0, -(1.0), 1.0, 1.0)), ((vec4<f32>(_e18.g0_) * _e21.g0_.yxwz) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))));
}

fn scalar_multi_vector_left_contraction(self_74: Scalar, other_62: MultiVector) -> MultiVector {
    var self_75: Scalar;
    var other_63: MultiVector;

    self_75 = self_74;
    other_63 = other_62;
    let _e4: Scalar = self_75;
    let _e7: MultiVector = other_63;
    let _e10: Scalar = self_75;
    let _e13: MultiVector = other_63;
    return MultiVector((vec4<f32>(_e4.g0_) * _e7.g0_), (vec4<f32>(_e10.g0_) * _e13.g1_));
}

fn scalar_multi_vector_right_contraction(self_76: Scalar, other_64: MultiVector) -> Scalar {
    var self_77: Scalar;
    var other_65: MultiVector;

    self_77 = self_76;
    other_65 = other_64;
    let _e4: Scalar = self_77;
    let _e6: MultiVector = other_65;
    return Scalar((_e4.g0_ * _e6.g0_.x));
}

fn scalar_multi_vector_left_anti_contraction(self_78: Scalar, other_66: MultiVector) -> AntiScalar {
    var self_79: Scalar;
    var other_67: MultiVector;

    self_79 = self_78;
    other_67 = other_66;
    let _e5: Scalar = self_79;
    let _e7: MultiVector = other_67;
    return AntiScalar((0.0 - (_e5.g0_ * _e7.g0_.x)));
}

fn scalar_multi_vector_right_anti_contraction(self_80: Scalar, other_68: MultiVector) -> MultiVector {
    var self_81: Scalar;
    var other_69: MultiVector;

    self_81 = self_80;
    other_69 = other_68;
    let _e4: Scalar = self_81;
    let _e7: MultiVector = other_69;
    let _e18: Scalar = self_81;
    let _e21: MultiVector = other_69;
    return MultiVector(((vec4<f32>(_e4.g0_) * _e7.g1_.yxwz) * vec4<f32>(1.0, -(1.0), 1.0, 1.0)), ((vec4<f32>(_e18.g0_) * _e21.g0_.yxwz) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))));
}

fn scalar_multi_vector_scalar_product(self_82: Scalar, other_70: MultiVector) -> Scalar {
    var self_83: Scalar;
    var other_71: MultiVector;

    self_83 = self_82;
    other_71 = other_70;
    let _e4: Scalar = self_83;
    let _e6: MultiVector = other_71;
    return Scalar((_e4.g0_ * _e6.g0_.x));
}

fn scalar_multi_vector_anti_scalar_product(self_84: Scalar, other_72: MultiVector) -> AntiScalar {
    var self_85: Scalar;
    var other_73: MultiVector;

    self_85 = self_84;
    other_73 = other_72;
    let _e5: Scalar = self_85;
    let _e7: MultiVector = other_73;
    return AntiScalar((0.0 - (_e5.g0_ * _e7.g0_.x)));
}

fn scalar_rotor_add(self_86: Scalar, other_74: Rotor) -> Rotor {
    var self_87: Scalar;
    var other_75: Rotor;

    self_87 = self_86;
    other_75 = other_74;
    let _e4: Scalar = self_87;
    let _e11: Rotor = other_75;
    return Rotor(((vec2<f32>(_e4.g0_) * vec2<f32>(1.0, 0.0)) + _e11.g0_));
}

fn scalar_rotor_sub(self_88: Scalar, other_76: Rotor) -> Rotor {
    var self_89: Scalar;
    var other_77: Rotor;

    self_89 = self_88;
    other_77 = other_76;
    let _e4: Scalar = self_89;
    let _e11: Rotor = other_77;
    return Rotor(((vec2<f32>(_e4.g0_) * vec2<f32>(1.0, 0.0)) - _e11.g0_));
}

fn scalar_rotor_geometric_product(self_90: Scalar, other_78: Rotor) -> Rotor {
    var self_91: Scalar;
    var other_79: Rotor;

    self_91 = self_90;
    other_79 = other_78;
    let _e4: Scalar = self_91;
    let _e7: Rotor = other_79;
    return Rotor((vec2<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_rotor_outer_product(self_92: Scalar, other_80: Rotor) -> Rotor {
    var self_93: Scalar;
    var other_81: Rotor;

    self_93 = self_92;
    other_81 = other_80;
    let _e4: Scalar = self_93;
    let _e7: Rotor = other_81;
    return Rotor((vec2<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_rotor_inner_product(self_94: Scalar, other_82: Rotor) -> Rotor {
    var self_95: Scalar;
    var other_83: Rotor;

    self_95 = self_94;
    other_83 = other_82;
    let _e4: Scalar = self_95;
    let _e7: Rotor = other_83;
    return Rotor((vec2<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_rotor_left_contraction(self_96: Scalar, other_84: Rotor) -> Rotor {
    var self_97: Scalar;
    var other_85: Rotor;

    self_97 = self_96;
    other_85 = other_84;
    let _e4: Scalar = self_97;
    let _e7: Rotor = other_85;
    return Rotor((vec2<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_rotor_right_contraction(self_98: Scalar, other_86: Rotor) -> Scalar {
    var self_99: Scalar;
    var other_87: Rotor;

    self_99 = self_98;
    other_87 = other_86;
    let _e4: Scalar = self_99;
    let _e6: Rotor = other_87;
    return Scalar((_e4.g0_ * _e6.g0_.x));
}

fn scalar_rotor_left_anti_contraction(self_100: Scalar, other_88: Rotor) -> AntiScalar {
    var self_101: Scalar;
    var other_89: Rotor;

    self_101 = self_100;
    other_89 = other_88;
    let _e5: Scalar = self_101;
    let _e7: Rotor = other_89;
    return AntiScalar((0.0 - (_e5.g0_ * _e7.g0_.x)));
}

fn scalar_rotor_scalar_product(self_102: Scalar, other_90: Rotor) -> Scalar {
    var self_103: Scalar;
    var other_91: Rotor;

    self_103 = self_102;
    other_91 = other_90;
    let _e4: Scalar = self_103;
    let _e6: Rotor = other_91;
    return Scalar((_e4.g0_ * _e6.g0_.x));
}

fn scalar_rotor_anti_scalar_product(self_104: Scalar, other_92: Rotor) -> AntiScalar {
    var self_105: Scalar;
    var other_93: Rotor;

    self_105 = self_104;
    other_93 = other_92;
    let _e5: Scalar = self_105;
    let _e7: Rotor = other_93;
    return AntiScalar((0.0 - (_e5.g0_ * _e7.g0_.x)));
}

fn scalar_point_add(self_106: Scalar, other_94: Point) -> Motor {
    var self_107: Scalar;
    var other_95: Point;

    self_107 = self_106;
    other_95 = other_94;
    let _e4: Scalar = self_107;
    let _e13: Point = other_95;
    let _e16: Point = other_95;
    let _e19: Point = other_95;
    let _e22: Point = other_95;
    return Motor(((vec4<f32>(_e4.g0_) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) + (vec4<f32>(_e13.g0_.x, _e16.g0_.x, _e19.g0_.y, _e22.g0_.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn scalar_point_sub(self_108: Scalar, other_96: Point) -> Motor {
    var self_109: Scalar;
    var other_97: Point;

    self_109 = self_108;
    other_97 = other_96;
    let _e4: Scalar = self_109;
    let _e13: Point = other_97;
    let _e16: Point = other_97;
    let _e19: Point = other_97;
    let _e22: Point = other_97;
    return Motor(((vec4<f32>(_e4.g0_) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) - (vec4<f32>(_e13.g0_.x, _e16.g0_.x, _e19.g0_.y, _e22.g0_.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn scalar_point_geometric_product(self_110: Scalar, other_98: Point) -> Point {
    var self_111: Scalar;
    var other_99: Point;

    self_111 = self_110;
    other_99 = other_98;
    let _e4: Scalar = self_111;
    let _e7: Point = other_99;
    return Point((vec3<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_point_outer_product(self_112: Scalar, other_100: Point) -> Point {
    var self_113: Scalar;
    var other_101: Point;

    self_113 = self_112;
    other_101 = other_100;
    let _e4: Scalar = self_113;
    let _e7: Point = other_101;
    return Point((vec3<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_point_inner_product(self_114: Scalar, other_102: Point) -> Point {
    var self_115: Scalar;
    var other_103: Point;

    self_115 = self_114;
    other_103 = other_102;
    let _e4: Scalar = self_115;
    let _e7: Point = other_103;
    return Point((vec3<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_point_geometric_anti_product(self_116: Scalar, other_104: Point) -> Plane {
    var self_117: Scalar;
    var other_105: Point;

    self_117 = self_116;
    other_105 = other_104;
    let _e4: Scalar = self_117;
    let _e7: Point = other_105;
    return Plane((vec3<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_point_inner_anti_product(self_118: Scalar, other_106: Point) -> Plane {
    var self_119: Scalar;
    var other_107: Point;

    self_119 = self_118;
    other_107 = other_106;
    let _e4: Scalar = self_119;
    let _e7: Point = other_107;
    return Plane((vec3<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_point_left_contraction(self_120: Scalar, other_108: Point) -> Point {
    var self_121: Scalar;
    var other_109: Point;

    self_121 = self_120;
    other_109 = other_108;
    let _e4: Scalar = self_121;
    let _e7: Point = other_109;
    return Point((vec3<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_point_right_anti_contraction(self_122: Scalar, other_110: Point) -> Plane {
    var self_123: Scalar;
    var other_111: Point;

    self_123 = self_122;
    other_111 = other_110;
    let _e4: Scalar = self_123;
    let _e7: Point = other_111;
    return Plane((vec3<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_ideal_point_add(self_124: Scalar, other_112: IdealPoint) -> Translator {
    var self_125: Scalar;
    var other_113: IdealPoint;

    self_125 = self_124;
    other_113 = other_112;
    let _e4: Scalar = self_125;
    let _e12: IdealPoint = other_113;
    let _e15: IdealPoint = other_113;
    let _e18: IdealPoint = other_113;
    return Translator(((vec3<f32>(_e4.g0_) * vec3<f32>(1.0, 0.0, 0.0)) + (vec3<f32>(_e12.g0_.x, _e15.g0_.x, _e18.g0_.y) * vec3<f32>(0.0, 1.0, 1.0))));
}

fn scalar_ideal_point_sub(self_126: Scalar, other_114: IdealPoint) -> Translator {
    var self_127: Scalar;
    var other_115: IdealPoint;

    self_127 = self_126;
    other_115 = other_114;
    let _e4: Scalar = self_127;
    let _e12: IdealPoint = other_115;
    let _e15: IdealPoint = other_115;
    let _e18: IdealPoint = other_115;
    return Translator(((vec3<f32>(_e4.g0_) * vec3<f32>(1.0, 0.0, 0.0)) - (vec3<f32>(_e12.g0_.x, _e15.g0_.x, _e18.g0_.y) * vec3<f32>(0.0, 1.0, 1.0))));
}

fn scalar_ideal_point_geometric_product(self_128: Scalar, other_116: IdealPoint) -> IdealPoint {
    var self_129: Scalar;
    var other_117: IdealPoint;

    self_129 = self_128;
    other_117 = other_116;
    let _e4: Scalar = self_129;
    let _e7: IdealPoint = other_117;
    return IdealPoint((vec2<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_ideal_point_outer_product(self_130: Scalar, other_118: IdealPoint) -> IdealPoint {
    var self_131: Scalar;
    var other_119: IdealPoint;

    self_131 = self_130;
    other_119 = other_118;
    let _e4: Scalar = self_131;
    let _e7: IdealPoint = other_119;
    return IdealPoint((vec2<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_ideal_point_inner_product(self_132: Scalar, other_120: IdealPoint) -> IdealPoint {
    var self_133: Scalar;
    var other_121: IdealPoint;

    self_133 = self_132;
    other_121 = other_120;
    let _e4: Scalar = self_133;
    let _e7: IdealPoint = other_121;
    return IdealPoint((vec2<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_ideal_point_left_contraction(self_134: Scalar, other_122: IdealPoint) -> IdealPoint {
    var self_135: Scalar;
    var other_123: IdealPoint;

    self_135 = self_134;
    other_123 = other_122;
    let _e4: Scalar = self_135;
    let _e7: IdealPoint = other_123;
    return IdealPoint((vec2<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_plane_geometric_product(self_136: Scalar, other_124: Plane) -> Plane {
    var self_137: Scalar;
    var other_125: Plane;

    self_137 = self_136;
    other_125 = other_124;
    let _e4: Scalar = self_137;
    let _e7: Plane = other_125;
    return Plane((vec3<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_plane_outer_product(self_138: Scalar, other_126: Plane) -> Plane {
    var self_139: Scalar;
    var other_127: Plane;

    self_139 = self_138;
    other_127 = other_126;
    let _e4: Scalar = self_139;
    let _e7: Plane = other_127;
    return Plane((vec3<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_plane_inner_product(self_140: Scalar, other_128: Plane) -> Plane {
    var self_141: Scalar;
    var other_129: Plane;

    self_141 = self_140;
    other_129 = other_128;
    let _e4: Scalar = self_141;
    let _e7: Plane = other_129;
    return Plane((vec3<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_plane_geometric_anti_product(self_142: Scalar, other_130: Plane) -> Point {
    var self_143: Scalar;
    var other_131: Plane;

    self_143 = self_142;
    other_131 = other_130;
    let _e6: Scalar = self_143;
    let _e9: Plane = other_131;
    return Point((vec3<f32>(0.0) - (vec3<f32>(_e6.g0_) * _e9.g0_)));
}

fn scalar_plane_inner_anti_product(self_144: Scalar, other_132: Plane) -> Point {
    var self_145: Scalar;
    var other_133: Plane;

    self_145 = self_144;
    other_133 = other_132;
    let _e6: Scalar = self_145;
    let _e9: Plane = other_133;
    return Point((vec3<f32>(0.0) - (vec3<f32>(_e6.g0_) * _e9.g0_)));
}

fn scalar_plane_left_contraction(self_146: Scalar, other_134: Plane) -> Plane {
    var self_147: Scalar;
    var other_135: Plane;

    self_147 = self_146;
    other_135 = other_134;
    let _e4: Scalar = self_147;
    let _e7: Plane = other_135;
    return Plane((vec3<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_plane_right_anti_contraction(self_148: Scalar, other_136: Plane) -> Point {
    var self_149: Scalar;
    var other_137: Plane;

    self_149 = self_148;
    other_137 = other_136;
    let _e6: Scalar = self_149;
    let _e9: Plane = other_137;
    return Point((vec3<f32>(0.0) - (vec3<f32>(_e6.g0_) * _e9.g0_)));
}

fn scalar_translator_add(self_150: Scalar, other_138: Translator) -> Translator {
    var self_151: Scalar;
    var other_139: Translator;

    self_151 = self_150;
    other_139 = other_138;
    let _e4: Scalar = self_151;
    let _e12: Translator = other_139;
    return Translator(((vec3<f32>(_e4.g0_) * vec3<f32>(1.0, 0.0, 0.0)) + _e12.g0_));
}

fn scalar_translator_sub(self_152: Scalar, other_140: Translator) -> Translator {
    var self_153: Scalar;
    var other_141: Translator;

    self_153 = self_152;
    other_141 = other_140;
    let _e4: Scalar = self_153;
    let _e12: Translator = other_141;
    return Translator(((vec3<f32>(_e4.g0_) * vec3<f32>(1.0, 0.0, 0.0)) - _e12.g0_));
}

fn scalar_translator_geometric_product(self_154: Scalar, other_142: Translator) -> Translator {
    var self_155: Scalar;
    var other_143: Translator;

    self_155 = self_154;
    other_143 = other_142;
    let _e4: Scalar = self_155;
    let _e7: Translator = other_143;
    return Translator((vec3<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_translator_outer_product(self_156: Scalar, other_144: Translator) -> Translator {
    var self_157: Scalar;
    var other_145: Translator;

    self_157 = self_156;
    other_145 = other_144;
    let _e4: Scalar = self_157;
    let _e7: Translator = other_145;
    return Translator((vec3<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_translator_inner_product(self_158: Scalar, other_146: Translator) -> Translator {
    var self_159: Scalar;
    var other_147: Translator;

    self_159 = self_158;
    other_147 = other_146;
    let _e4: Scalar = self_159;
    let _e7: Translator = other_147;
    return Translator((vec3<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_translator_left_contraction(self_160: Scalar, other_148: Translator) -> Translator {
    var self_161: Scalar;
    var other_149: Translator;

    self_161 = self_160;
    other_149 = other_148;
    let _e4: Scalar = self_161;
    let _e7: Translator = other_149;
    return Translator((vec3<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_translator_right_contraction(self_162: Scalar, other_150: Translator) -> Scalar {
    var self_163: Scalar;
    var other_151: Translator;

    self_163 = self_162;
    other_151 = other_150;
    let _e4: Scalar = self_163;
    let _e6: Translator = other_151;
    return Scalar((_e4.g0_ * _e6.g0_.x));
}

fn scalar_translator_left_anti_contraction(self_164: Scalar, other_152: Translator) -> AntiScalar {
    var self_165: Scalar;
    var other_153: Translator;

    self_165 = self_164;
    other_153 = other_152;
    let _e5: Scalar = self_165;
    let _e7: Translator = other_153;
    return AntiScalar((0.0 - (_e5.g0_ * _e7.g0_.x)));
}

fn scalar_translator_scalar_product(self_166: Scalar, other_154: Translator) -> Scalar {
    var self_167: Scalar;
    var other_155: Translator;

    self_167 = self_166;
    other_155 = other_154;
    let _e4: Scalar = self_167;
    let _e6: Translator = other_155;
    return Scalar((_e4.g0_ * _e6.g0_.x));
}

fn scalar_translator_anti_scalar_product(self_168: Scalar, other_156: Translator) -> AntiScalar {
    var self_169: Scalar;
    var other_157: Translator;

    self_169 = self_168;
    other_157 = other_156;
    let _e5: Scalar = self_169;
    let _e7: Translator = other_157;
    return AntiScalar((0.0 - (_e5.g0_ * _e7.g0_.x)));
}

fn scalar_motor_add(self_170: Scalar, other_158: Motor) -> Motor {
    var self_171: Scalar;
    var other_159: Motor;

    self_171 = self_170;
    other_159 = other_158;
    let _e4: Scalar = self_171;
    let _e13: Motor = other_159;
    return Motor(((vec4<f32>(_e4.g0_) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) + _e13.g0_));
}

fn scalar_motor_sub(self_172: Scalar, other_160: Motor) -> Motor {
    var self_173: Scalar;
    var other_161: Motor;

    self_173 = self_172;
    other_161 = other_160;
    let _e4: Scalar = self_173;
    let _e13: Motor = other_161;
    return Motor(((vec4<f32>(_e4.g0_) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) - _e13.g0_));
}

fn scalar_motor_geometric_product(self_174: Scalar, other_162: Motor) -> Motor {
    var self_175: Scalar;
    var other_163: Motor;

    self_175 = self_174;
    other_163 = other_162;
    let _e4: Scalar = self_175;
    let _e7: Motor = other_163;
    return Motor((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_motor_outer_product(self_176: Scalar, other_164: Motor) -> Motor {
    var self_177: Scalar;
    var other_165: Motor;

    self_177 = self_176;
    other_165 = other_164;
    let _e4: Scalar = self_177;
    let _e7: Motor = other_165;
    return Motor((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_motor_inner_product(self_178: Scalar, other_166: Motor) -> Motor {
    var self_179: Scalar;
    var other_167: Motor;

    self_179 = self_178;
    other_167 = other_166;
    let _e4: Scalar = self_179;
    let _e7: Motor = other_167;
    return Motor((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_motor_geometric_anti_product(self_180: Scalar, other_168: Motor) -> MotorDual {
    var self_181: Scalar;
    var other_169: Motor;

    self_181 = self_180;
    other_169 = other_168;
    let _e4: Scalar = self_181;
    let _e7: Motor = other_169;
    return MotorDual(((vec4<f32>(_e4.g0_) * _e7.g0_) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0)));
}

fn scalar_motor_inner_anti_product(self_182: Scalar, other_170: Motor) -> MotorDual {
    var self_183: Scalar;
    var other_171: Motor;

    self_183 = self_182;
    other_171 = other_170;
    let _e4: Scalar = self_183;
    let _e7: Motor = other_171;
    return MotorDual(((vec4<f32>(_e4.g0_) * _e7.g0_) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0)));
}

fn scalar_motor_left_contraction(self_184: Scalar, other_172: Motor) -> Motor {
    var self_185: Scalar;
    var other_173: Motor;

    self_185 = self_184;
    other_173 = other_172;
    let _e4: Scalar = self_185;
    let _e7: Motor = other_173;
    return Motor((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_motor_right_contraction(self_186: Scalar, other_174: Motor) -> Scalar {
    var self_187: Scalar;
    var other_175: Motor;

    self_187 = self_186;
    other_175 = other_174;
    let _e4: Scalar = self_187;
    let _e6: Motor = other_175;
    return Scalar((_e4.g0_ * _e6.g0_.x));
}

fn scalar_motor_left_anti_contraction(self_188: Scalar, other_176: Motor) -> AntiScalar {
    var self_189: Scalar;
    var other_177: Motor;

    self_189 = self_188;
    other_177 = other_176;
    let _e5: Scalar = self_189;
    let _e7: Motor = other_177;
    return AntiScalar((0.0 - (_e5.g0_ * _e7.g0_.x)));
}

fn scalar_motor_right_anti_contraction(self_190: Scalar, other_178: Motor) -> MotorDual {
    var self_191: Scalar;
    var other_179: Motor;

    self_191 = self_190;
    other_179 = other_178;
    let _e4: Scalar = self_191;
    let _e7: Motor = other_179;
    return MotorDual(((vec4<f32>(_e4.g0_) * _e7.g0_) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0)));
}

fn scalar_motor_scalar_product(self_192: Scalar, other_180: Motor) -> Scalar {
    var self_193: Scalar;
    var other_181: Motor;

    self_193 = self_192;
    other_181 = other_180;
    let _e4: Scalar = self_193;
    let _e6: Motor = other_181;
    return Scalar((_e4.g0_ * _e6.g0_.x));
}

fn scalar_motor_anti_scalar_product(self_194: Scalar, other_182: Motor) -> AntiScalar {
    var self_195: Scalar;
    var other_183: Motor;

    self_195 = self_194;
    other_183 = other_182;
    let _e5: Scalar = self_195;
    let _e7: Motor = other_183;
    return AntiScalar((0.0 - (_e5.g0_ * _e7.g0_.x)));
}

fn scalar_motor_dual_geometric_product(self_196: Scalar, other_184: MotorDual) -> MotorDual {
    var self_197: Scalar;
    var other_185: MotorDual;

    self_197 = self_196;
    other_185 = other_184;
    let _e4: Scalar = self_197;
    let _e7: MotorDual = other_185;
    return MotorDual((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_motor_dual_regressive_product(self_198: Scalar, other_186: MotorDual) -> Scalar {
    var self_199: Scalar;
    var other_187: MotorDual;

    self_199 = self_198;
    other_187 = other_186;
    let _e4: Scalar = self_199;
    let _e6: MotorDual = other_187;
    return Scalar((_e4.g0_ * _e6.g0_.x));
}

fn scalar_motor_dual_outer_product(self_200: Scalar, other_188: MotorDual) -> MotorDual {
    var self_201: Scalar;
    var other_189: MotorDual;

    self_201 = self_200;
    other_189 = other_188;
    let _e4: Scalar = self_201;
    let _e7: MotorDual = other_189;
    return MotorDual((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_motor_dual_inner_product(self_202: Scalar, other_190: MotorDual) -> MotorDual {
    var self_203: Scalar;
    var other_191: MotorDual;

    self_203 = self_202;
    other_191 = other_190;
    let _e4: Scalar = self_203;
    let _e7: MotorDual = other_191;
    return MotorDual((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_motor_dual_geometric_anti_product(self_204: Scalar, other_192: MotorDual) -> Motor {
    var self_205: Scalar;
    var other_193: MotorDual;

    self_205 = self_204;
    other_193 = other_192;
    let _e4: Scalar = self_205;
    let _e7: MotorDual = other_193;
    return Motor(((vec4<f32>(_e4.g0_) * _e7.g0_) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))));
}

fn scalar_motor_dual_inner_anti_product(self_206: Scalar, other_194: MotorDual) -> Motor {
    var self_207: Scalar;
    var other_195: MotorDual;

    self_207 = self_206;
    other_195 = other_194;
    let _e4: Scalar = self_207;
    let _e7: MotorDual = other_195;
    return Motor(((vec4<f32>(_e4.g0_) * _e7.g0_) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))));
}

fn scalar_motor_dual_left_contraction(self_208: Scalar, other_196: MotorDual) -> MotorDual {
    var self_209: Scalar;
    var other_197: MotorDual;

    self_209 = self_208;
    other_197 = other_196;
    let _e4: Scalar = self_209;
    let _e7: MotorDual = other_197;
    return MotorDual((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_motor_dual_right_anti_contraction(self_210: Scalar, other_198: MotorDual) -> Motor {
    var self_211: Scalar;
    var other_199: MotorDual;

    self_211 = self_210;
    other_199 = other_198;
    let _e4: Scalar = self_211;
    let _e7: MotorDual = other_199;
    return Motor(((vec4<f32>(_e4.g0_) * _e7.g0_) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))));
}

fn scalar_squared_magnitude(self_212: Scalar) -> Scalar {
    var self_213: Scalar;

    self_213 = self_212;
    let _e2: Scalar = self_213;
    let _e3: Scalar = self_213;
    let _e4: Scalar = scalar_reversal(_e3);
    let _e5: Scalar = scalar_scalar_scalar_product(_e2, _e4);
    return _e5;
}

fn scalar_magnitude(self_214: Scalar) -> Scalar {
    var self_215: Scalar;

    self_215 = self_214;
    let _e2: Scalar = self_215;
    let _e3: Scalar = scalar_squared_magnitude(_e2);
    return Scalar(sqrt(_e3.g0_));
}

fn scalar_bulk_norm(self_216: Scalar) -> Scalar {
    var self_217: Scalar;

    self_217 = self_216;
    let _e2: Scalar = self_217;
    let _e3: Scalar = scalar_squared_magnitude(_e2);
    return Scalar(sqrt(_e3.g0_));
}

fn scalar_squared_anti_magnitude(self_218: Scalar) -> AntiScalar {
    var self_219: Scalar;

    self_219 = self_218;
    let _e2: Scalar = self_219;
    let _e3: Scalar = self_219;
    let _e4: Scalar = scalar_anti_reversal(_e3);
    let _e5: AntiScalar = scalar_scalar_anti_scalar_product(_e2, _e4);
    return _e5;
}

fn scalar_weight_norm(self_220: Scalar) -> AntiScalar {
    var self_221: Scalar;

    self_221 = self_220;
    let _e2: Scalar = self_221;
    let _e3: AntiScalar = scalar_squared_anti_magnitude(_e2);
    return AntiScalar(sqrt(_e3.g0_));
}

fn scalar_scale(self_222: Scalar, other_200: f32) -> Scalar {
    var self_223: Scalar;
    var other_201: f32;

    self_223 = self_222;
    other_201 = other_200;
    let _e4: Scalar = self_223;
    let _e5: f32 = other_201;
    let _e7: Scalar = scalar_scalar_geometric_product(_e4, Scalar(_e5));
    return _e7;
}

fn scalar_signum(self_224: Scalar) -> Scalar {
    var self_225: Scalar;

    self_225 = self_224;
    let _e2: Scalar = self_225;
    let _e3: Scalar = self_225;
    let _e4: Scalar = scalar_magnitude(_e3);
    let _e9: Scalar = scalar_scalar_geometric_product(_e2, Scalar((1.0 / _e4.g0_)));
    return _e9;
}

fn scalar_inverse(self_226: Scalar) -> Scalar {
    var self_227: Scalar;

    self_227 = self_226;
    let _e2: Scalar = self_227;
    let _e3: Scalar = scalar_reversal(_e2);
    let _e4: Scalar = self_227;
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

fn anti_scalar_neg(self_228: AntiScalar) -> AntiScalar {
    var self_229: AntiScalar;

    self_229 = self_228;
    let _e2: AntiScalar = self_229;
    return AntiScalar((_e2.g0_ * -(1.0)));
}

fn anti_scalar_automorphism(self_230: AntiScalar) -> AntiScalar {
    var self_231: AntiScalar;

    self_231 = self_230;
    let _e2: AntiScalar = self_231;
    return AntiScalar((_e2.g0_ * -(1.0)));
}

fn anti_scalar_reversal(self_232: AntiScalar) -> AntiScalar {
    var self_233: AntiScalar;

    self_233 = self_232;
    let _e2: AntiScalar = self_233;
    return AntiScalar((_e2.g0_ * -(1.0)));
}

fn anti_scalar_conjugation(self_234: AntiScalar) -> AntiScalar {
    var self_235: AntiScalar;

    self_235 = self_234;
    let _e2: AntiScalar = self_235;
    return AntiScalar(_e2.g0_);
}

fn anti_scalar_dual(self_236: AntiScalar) -> Scalar {
    var self_237: AntiScalar;

    self_237 = self_236;
    let _e2: AntiScalar = self_237;
    return Scalar(_e2.g0_);
}

fn anti_scalar_anti_reversal(self_238: AntiScalar) -> AntiScalar {
    var self_239: AntiScalar;

    self_239 = self_238;
    let _e2: AntiScalar = self_239;
    return AntiScalar(_e2.g0_);
}

fn anti_scalar_scalar_geometric_product(self_240: AntiScalar, other_202: Scalar) -> AntiScalar {
    var self_241: AntiScalar;
    var other_203: Scalar;

    self_241 = self_240;
    other_203 = other_202;
    let _e4: AntiScalar = self_241;
    let _e6: Scalar = other_203;
    return AntiScalar((_e4.g0_ * _e6.g0_));
}

fn anti_scalar_scalar_regressive_product(self_242: AntiScalar, other_204: Scalar) -> Scalar {
    var self_243: AntiScalar;
    var other_205: Scalar;

    self_243 = self_242;
    other_205 = other_204;
    let _e4: AntiScalar = self_243;
    let _e6: Scalar = other_205;
    return Scalar((_e4.g0_ * _e6.g0_));
}

fn anti_scalar_scalar_outer_product(self_244: AntiScalar, other_206: Scalar) -> AntiScalar {
    var self_245: AntiScalar;
    var other_207: Scalar;

    self_245 = self_244;
    other_207 = other_206;
    let _e4: AntiScalar = self_245;
    let _e6: Scalar = other_207;
    return AntiScalar((_e4.g0_ * _e6.g0_));
}

fn anti_scalar_scalar_inner_product(self_246: AntiScalar, other_208: Scalar) -> AntiScalar {
    var self_247: AntiScalar;
    var other_209: Scalar;

    self_247 = self_246;
    other_209 = other_208;
    let _e4: AntiScalar = self_247;
    let _e6: Scalar = other_209;
    return AntiScalar((_e4.g0_ * _e6.g0_));
}

fn anti_scalar_scalar_geometric_anti_product(self_248: AntiScalar, other_210: Scalar) -> Scalar {
    var self_249: AntiScalar;
    var other_211: Scalar;

    self_249 = self_248;
    other_211 = other_210;
    let _e4: AntiScalar = self_249;
    let _e6: Scalar = other_211;
    return Scalar((_e4.g0_ * _e6.g0_));
}

fn anti_scalar_scalar_inner_anti_product(self_250: AntiScalar, other_212: Scalar) -> Scalar {
    var self_251: AntiScalar;
    var other_213: Scalar;

    self_251 = self_250;
    other_213 = other_212;
    let _e4: AntiScalar = self_251;
    let _e6: Scalar = other_213;
    return Scalar((_e4.g0_ * _e6.g0_));
}

fn anti_scalar_scalar_right_contraction(self_252: AntiScalar, other_214: Scalar) -> AntiScalar {
    var self_253: AntiScalar;
    var other_215: Scalar;

    self_253 = self_252;
    other_215 = other_214;
    let _e4: AntiScalar = self_253;
    let _e6: Scalar = other_215;
    return AntiScalar((_e4.g0_ * _e6.g0_));
}

fn anti_scalar_scalar_left_anti_contraction(self_254: AntiScalar, other_216: Scalar) -> Scalar {
    var self_255: AntiScalar;
    var other_217: Scalar;

    self_255 = self_254;
    other_217 = other_216;
    let _e4: AntiScalar = self_255;
    let _e6: Scalar = other_217;
    return Scalar((_e4.g0_ * _e6.g0_));
}

fn anti_scalar_anti_scalar_add(self_256: AntiScalar, other_218: AntiScalar) -> AntiScalar {
    var self_257: AntiScalar;
    var other_219: AntiScalar;

    self_257 = self_256;
    other_219 = other_218;
    let _e4: AntiScalar = self_257;
    let _e6: AntiScalar = other_219;
    return AntiScalar((_e4.g0_ + _e6.g0_));
}

fn anti_scalar_anti_scalar_sub(self_258: AntiScalar, other_220: AntiScalar) -> AntiScalar {
    var self_259: AntiScalar;
    var other_221: AntiScalar;

    self_259 = self_258;
    other_221 = other_220;
    let _e4: AntiScalar = self_259;
    let _e6: AntiScalar = other_221;
    return AntiScalar((_e4.g0_ - _e6.g0_));
}

fn anti_scalar_anti_scalar_mul(self_260: AntiScalar, other_222: AntiScalar) -> AntiScalar {
    var self_261: AntiScalar;
    var other_223: AntiScalar;

    self_261 = self_260;
    other_223 = other_222;
    let _e4: AntiScalar = self_261;
    let _e6: AntiScalar = other_223;
    return AntiScalar((_e4.g0_ * _e6.g0_));
}

fn anti_scalar_anti_scalar_div(self_262: AntiScalar, other_224: AntiScalar) -> AntiScalar {
    var self_263: AntiScalar;
    var other_225: AntiScalar;

    self_263 = self_262;
    other_225 = other_224;
    let _e4: AntiScalar = self_263;
    let _e8: AntiScalar = other_225;
    return AntiScalar((((_e4.g0_ * 1.0) / _e8.g0_) * 1.0));
}

fn anti_scalar_anti_scalar_geometric_product(self_264: AntiScalar, other_226: AntiScalar) -> Scalar {
    var self_265: AntiScalar;
    var other_227: AntiScalar;

    self_265 = self_264;
    other_227 = other_226;
    let _e5: AntiScalar = self_265;
    let _e7: AntiScalar = other_227;
    return Scalar((0.0 - (_e5.g0_ * _e7.g0_)));
}

fn anti_scalar_anti_scalar_regressive_product(self_266: AntiScalar, other_228: AntiScalar) -> AntiScalar {
    var self_267: AntiScalar;
    var other_229: AntiScalar;

    self_267 = self_266;
    other_229 = other_228;
    let _e4: AntiScalar = self_267;
    let _e6: AntiScalar = other_229;
    return AntiScalar((_e4.g0_ * _e6.g0_));
}

fn anti_scalar_anti_scalar_inner_product(self_268: AntiScalar, other_230: AntiScalar) -> Scalar {
    var self_269: AntiScalar;
    var other_231: AntiScalar;

    self_269 = self_268;
    other_231 = other_230;
    let _e5: AntiScalar = self_269;
    let _e7: AntiScalar = other_231;
    return Scalar((0.0 - (_e5.g0_ * _e7.g0_)));
}

fn anti_scalar_anti_scalar_geometric_anti_product(self_270: AntiScalar, other_232: AntiScalar) -> AntiScalar {
    var self_271: AntiScalar;
    var other_233: AntiScalar;

    self_271 = self_270;
    other_233 = other_232;
    let _e4: AntiScalar = self_271;
    let _e6: AntiScalar = other_233;
    return AntiScalar((_e4.g0_ * _e6.g0_));
}

fn anti_scalar_anti_scalar_inner_anti_product(self_272: AntiScalar, other_234: AntiScalar) -> AntiScalar {
    var self_273: AntiScalar;
    var other_235: AntiScalar;

    self_273 = self_272;
    other_235 = other_234;
    let _e4: AntiScalar = self_273;
    let _e6: AntiScalar = other_235;
    return AntiScalar((_e4.g0_ * _e6.g0_));
}

fn anti_scalar_anti_scalar_left_contraction(self_274: AntiScalar, other_236: AntiScalar) -> Scalar {
    var self_275: AntiScalar;
    var other_237: AntiScalar;

    self_275 = self_274;
    other_237 = other_236;
    let _e5: AntiScalar = self_275;
    let _e7: AntiScalar = other_237;
    return Scalar((0.0 - (_e5.g0_ * _e7.g0_)));
}

fn anti_scalar_anti_scalar_right_contraction(self_276: AntiScalar, other_238: AntiScalar) -> Scalar {
    var self_277: AntiScalar;
    var other_239: AntiScalar;

    self_277 = self_276;
    other_239 = other_238;
    let _e5: AntiScalar = self_277;
    let _e7: AntiScalar = other_239;
    return Scalar((0.0 - (_e5.g0_ * _e7.g0_)));
}

fn anti_scalar_anti_scalar_left_anti_contraction(self_278: AntiScalar, other_240: AntiScalar) -> AntiScalar {
    var self_279: AntiScalar;
    var other_241: AntiScalar;

    self_279 = self_278;
    other_241 = other_240;
    let _e4: AntiScalar = self_279;
    let _e6: AntiScalar = other_241;
    return AntiScalar((_e4.g0_ * _e6.g0_));
}

fn anti_scalar_anti_scalar_right_anti_contraction(self_280: AntiScalar, other_242: AntiScalar) -> AntiScalar {
    var self_281: AntiScalar;
    var other_243: AntiScalar;

    self_281 = self_280;
    other_243 = other_242;
    let _e4: AntiScalar = self_281;
    let _e6: AntiScalar = other_243;
    return AntiScalar((_e4.g0_ * _e6.g0_));
}

fn anti_scalar_anti_scalar_scalar_product(self_282: AntiScalar, other_244: AntiScalar) -> Scalar {
    var self_283: AntiScalar;
    var other_245: AntiScalar;

    self_283 = self_282;
    other_245 = other_244;
    let _e5: AntiScalar = self_283;
    let _e7: AntiScalar = other_245;
    return Scalar((0.0 - (_e5.g0_ * _e7.g0_)));
}

fn anti_scalar_anti_scalar_anti_scalar_product(self_284: AntiScalar, other_246: AntiScalar) -> AntiScalar {
    var self_285: AntiScalar;
    var other_247: AntiScalar;

    self_285 = self_284;
    other_247 = other_246;
    let _e4: AntiScalar = self_285;
    let _e6: AntiScalar = other_247;
    return AntiScalar((_e4.g0_ * _e6.g0_));
}

fn anti_scalar_multi_vector_add(self_286: AntiScalar, other_248: MultiVector) -> MultiVector {
    var self_287: AntiScalar;
    var other_249: MultiVector;

    self_287 = self_286;
    other_249 = other_248;
    let _e4: MultiVector = other_249;
    let _e6: AntiScalar = self_287;
    let _e15: MultiVector = other_249;
    return MultiVector(_e4.g0_, ((vec4<f32>(_e6.g0_) * vec4<f32>(0.0, 1.0, 0.0, 0.0)) + _e15.g1_));
}

fn anti_scalar_multi_vector_sub(self_288: AntiScalar, other_250: MultiVector) -> MultiVector {
    var self_289: AntiScalar;
    var other_251: MultiVector;

    self_289 = self_288;
    other_251 = other_250;
    let _e6: MultiVector = other_251;
    let _e9: AntiScalar = self_289;
    let _e18: MultiVector = other_251;
    return MultiVector((vec4<f32>(0.0) - _e6.g0_), ((vec4<f32>(_e9.g0_) * vec4<f32>(0.0, 1.0, 0.0, 0.0)) - _e18.g1_));
}

fn anti_scalar_multi_vector_geometric_product(self_290: AntiScalar, other_252: MultiVector) -> MultiVector {
    var self_291: AntiScalar;
    var other_253: MultiVector;

    self_291 = self_290;
    other_253 = other_252;
    let _e4: AntiScalar = self_291;
    let _e7: MultiVector = other_253;
    let _e20: AntiScalar = self_291;
    let _e23: MultiVector = other_253;
    return MultiVector(((vec4<f32>(_e4.g0_) * _e7.g1_.yxwz) * vec4<f32>(-(1.0), 1.0, -(1.0), -(1.0))), ((vec4<f32>(_e20.g0_) * _e23.g0_.yxwz) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0)));
}

fn anti_scalar_multi_vector_regressive_product(self_292: AntiScalar, other_254: MultiVector) -> MultiVector {
    var self_293: AntiScalar;
    var other_255: MultiVector;

    self_293 = self_292;
    other_255 = other_254;
    let _e4: AntiScalar = self_293;
    let _e7: MultiVector = other_255;
    let _e10: AntiScalar = self_293;
    let _e13: MultiVector = other_255;
    return MultiVector((vec4<f32>(_e4.g0_) * _e7.g0_), (vec4<f32>(_e10.g0_) * _e13.g1_));
}

fn anti_scalar_multi_vector_outer_product(self_294: AntiScalar, other_256: MultiVector) -> AntiScalar {
    var self_295: AntiScalar;
    var other_257: MultiVector;

    self_295 = self_294;
    other_257 = other_256;
    let _e4: AntiScalar = self_295;
    let _e6: MultiVector = other_257;
    return AntiScalar((_e4.g0_ * _e6.g0_.x));
}

fn anti_scalar_multi_vector_inner_product(self_296: AntiScalar, other_258: MultiVector) -> MultiVector {
    var self_297: AntiScalar;
    var other_259: MultiVector;

    self_297 = self_296;
    other_259 = other_258;
    let _e4: AntiScalar = self_297;
    let _e7: MultiVector = other_259;
    let _e20: AntiScalar = self_297;
    let _e23: MultiVector = other_259;
    return MultiVector(((vec4<f32>(_e4.g0_) * _e7.g1_.yxwz) * vec4<f32>(-(1.0), 1.0, -(1.0), -(1.0))), ((vec4<f32>(_e20.g0_) * _e23.g0_.yxwz) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0)));
}

fn anti_scalar_multi_vector_geometric_anti_product(self_298: AntiScalar, other_260: MultiVector) -> MultiVector {
    var self_299: AntiScalar;
    var other_261: MultiVector;

    self_299 = self_298;
    other_261 = other_260;
    let _e4: AntiScalar = self_299;
    let _e7: MultiVector = other_261;
    let _e10: AntiScalar = self_299;
    let _e13: MultiVector = other_261;
    return MultiVector((vec4<f32>(_e4.g0_) * _e7.g0_), (vec4<f32>(_e10.g0_) * _e13.g1_));
}

fn anti_scalar_multi_vector_inner_anti_product(self_300: AntiScalar, other_262: MultiVector) -> MultiVector {
    var self_301: AntiScalar;
    var other_263: MultiVector;

    self_301 = self_300;
    other_263 = other_262;
    let _e4: AntiScalar = self_301;
    let _e7: MultiVector = other_263;
    let _e10: AntiScalar = self_301;
    let _e13: MultiVector = other_263;
    return MultiVector((vec4<f32>(_e4.g0_) * _e7.g0_), (vec4<f32>(_e10.g0_) * _e13.g1_));
}

fn anti_scalar_multi_vector_left_contraction(self_302: AntiScalar, other_264: MultiVector) -> Scalar {
    var self_303: AntiScalar;
    var other_265: MultiVector;

    self_303 = self_302;
    other_265 = other_264;
    let _e5: AntiScalar = self_303;
    let _e7: MultiVector = other_265;
    return Scalar((0.0 - (_e5.g0_ * _e7.g1_.y)));
}

fn anti_scalar_multi_vector_right_contraction(self_304: AntiScalar, other_266: MultiVector) -> MultiVector {
    var self_305: AntiScalar;
    var other_267: MultiVector;

    self_305 = self_304;
    other_267 = other_266;
    let _e4: AntiScalar = self_305;
    let _e7: MultiVector = other_267;
    let _e20: AntiScalar = self_305;
    let _e23: MultiVector = other_267;
    return MultiVector(((vec4<f32>(_e4.g0_) * _e7.g1_.yxwz) * vec4<f32>(-(1.0), 1.0, -(1.0), -(1.0))), ((vec4<f32>(_e20.g0_) * _e23.g0_.yxwz) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0)));
}

fn anti_scalar_multi_vector_left_anti_contraction(self_306: AntiScalar, other_268: MultiVector) -> MultiVector {
    var self_307: AntiScalar;
    var other_269: MultiVector;

    self_307 = self_306;
    other_269 = other_268;
    let _e4: AntiScalar = self_307;
    let _e7: MultiVector = other_269;
    let _e10: AntiScalar = self_307;
    let _e13: MultiVector = other_269;
    return MultiVector((vec4<f32>(_e4.g0_) * _e7.g0_), (vec4<f32>(_e10.g0_) * _e13.g1_));
}

fn anti_scalar_multi_vector_right_anti_contraction(self_308: AntiScalar, other_270: MultiVector) -> AntiScalar {
    var self_309: AntiScalar;
    var other_271: MultiVector;

    self_309 = self_308;
    other_271 = other_270;
    let _e4: AntiScalar = self_309;
    let _e6: MultiVector = other_271;
    return AntiScalar((_e4.g0_ * _e6.g1_.y));
}

fn anti_scalar_multi_vector_scalar_product(self_310: AntiScalar, other_272: MultiVector) -> Scalar {
    var self_311: AntiScalar;
    var other_273: MultiVector;

    self_311 = self_310;
    other_273 = other_272;
    let _e5: AntiScalar = self_311;
    let _e7: MultiVector = other_273;
    return Scalar((0.0 - (_e5.g0_ * _e7.g1_.y)));
}

fn anti_scalar_multi_vector_anti_scalar_product(self_312: AntiScalar, other_274: MultiVector) -> AntiScalar {
    var self_313: AntiScalar;
    var other_275: MultiVector;

    self_313 = self_312;
    other_275 = other_274;
    let _e4: AntiScalar = self_313;
    let _e6: MultiVector = other_275;
    return AntiScalar((_e4.g0_ * _e6.g1_.y));
}

fn anti_scalar_rotor_regressive_product(self_314: AntiScalar, other_276: Rotor) -> Rotor {
    var self_315: AntiScalar;
    var other_277: Rotor;

    self_315 = self_314;
    other_277 = other_276;
    let _e4: AntiScalar = self_315;
    let _e7: Rotor = other_277;
    return Rotor((vec2<f32>(_e4.g0_) * _e7.g0_));
}

fn anti_scalar_rotor_outer_product(self_316: AntiScalar, other_278: Rotor) -> AntiScalar {
    var self_317: AntiScalar;
    var other_279: Rotor;

    self_317 = self_316;
    other_279 = other_278;
    let _e4: AntiScalar = self_317;
    let _e6: Rotor = other_279;
    return AntiScalar((_e4.g0_ * _e6.g0_.x));
}

fn anti_scalar_rotor_geometric_anti_product(self_318: AntiScalar, other_280: Rotor) -> Rotor {
    var self_319: AntiScalar;
    var other_281: Rotor;

    self_319 = self_318;
    other_281 = other_280;
    let _e4: AntiScalar = self_319;
    let _e7: Rotor = other_281;
    return Rotor((vec2<f32>(_e4.g0_) * _e7.g0_));
}

fn anti_scalar_rotor_inner_anti_product(self_320: AntiScalar, other_282: Rotor) -> Rotor {
    var self_321: AntiScalar;
    var other_283: Rotor;

    self_321 = self_320;
    other_283 = other_282;
    let _e4: AntiScalar = self_321;
    let _e7: Rotor = other_283;
    return Rotor((vec2<f32>(_e4.g0_) * _e7.g0_));
}

fn anti_scalar_rotor_left_anti_contraction(self_322: AntiScalar, other_284: Rotor) -> Rotor {
    var self_323: AntiScalar;
    var other_285: Rotor;

    self_323 = self_322;
    other_285 = other_284;
    let _e4: AntiScalar = self_323;
    let _e7: Rotor = other_285;
    return Rotor((vec2<f32>(_e4.g0_) * _e7.g0_));
}

fn anti_scalar_point_geometric_product(self_324: AntiScalar, other_286: Point) -> Plane {
    var self_325: AntiScalar;
    var other_287: Point;

    self_325 = self_324;
    other_287 = other_286;
    let _e6: AntiScalar = self_325;
    let _e9: Point = other_287;
    return Plane((vec3<f32>(0.0) - (vec3<f32>(_e6.g0_) * _e9.g0_)));
}

fn anti_scalar_point_regressive_product(self_326: AntiScalar, other_288: Point) -> Point {
    var self_327: AntiScalar;
    var other_289: Point;

    self_327 = self_326;
    other_289 = other_288;
    let _e4: AntiScalar = self_327;
    let _e7: Point = other_289;
    return Point((vec3<f32>(_e4.g0_) * _e7.g0_));
}

fn anti_scalar_point_inner_product(self_328: AntiScalar, other_290: Point) -> Plane {
    var self_329: AntiScalar;
    var other_291: Point;

    self_329 = self_328;
    other_291 = other_290;
    let _e6: AntiScalar = self_329;
    let _e9: Point = other_291;
    return Plane((vec3<f32>(0.0) - (vec3<f32>(_e6.g0_) * _e9.g0_)));
}

fn anti_scalar_point_geometric_anti_product(self_330: AntiScalar, other_292: Point) -> Point {
    var self_331: AntiScalar;
    var other_293: Point;

    self_331 = self_330;
    other_293 = other_292;
    let _e4: AntiScalar = self_331;
    let _e7: Point = other_293;
    return Point((vec3<f32>(_e4.g0_) * _e7.g0_));
}

fn anti_scalar_point_inner_anti_product(self_332: AntiScalar, other_294: Point) -> Point {
    var self_333: AntiScalar;
    var other_295: Point;

    self_333 = self_332;
    other_295 = other_294;
    let _e4: AntiScalar = self_333;
    let _e7: Point = other_295;
    return Point((vec3<f32>(_e4.g0_) * _e7.g0_));
}

fn anti_scalar_point_right_contraction(self_334: AntiScalar, other_296: Point) -> Plane {
    var self_335: AntiScalar;
    var other_297: Point;

    self_335 = self_334;
    other_297 = other_296;
    let _e6: AntiScalar = self_335;
    let _e9: Point = other_297;
    return Plane((vec3<f32>(0.0) - (vec3<f32>(_e6.g0_) * _e9.g0_)));
}

fn anti_scalar_point_left_anti_contraction(self_336: AntiScalar, other_298: Point) -> Point {
    var self_337: AntiScalar;
    var other_299: Point;

    self_337 = self_336;
    other_299 = other_298;
    let _e4: AntiScalar = self_337;
    let _e7: Point = other_299;
    return Point((vec3<f32>(_e4.g0_) * _e7.g0_));
}

fn anti_scalar_ideal_point_regressive_product(self_338: AntiScalar, other_300: IdealPoint) -> IdealPoint {
    var self_339: AntiScalar;
    var other_301: IdealPoint;

    self_339 = self_338;
    other_301 = other_300;
    let _e4: AntiScalar = self_339;
    let _e7: IdealPoint = other_301;
    return IdealPoint((vec2<f32>(_e4.g0_) * _e7.g0_));
}

fn anti_scalar_ideal_point_geometric_anti_product(self_340: AntiScalar, other_302: IdealPoint) -> IdealPoint {
    var self_341: AntiScalar;
    var other_303: IdealPoint;

    self_341 = self_340;
    other_303 = other_302;
    let _e4: AntiScalar = self_341;
    let _e7: IdealPoint = other_303;
    return IdealPoint((vec2<f32>(_e4.g0_) * _e7.g0_));
}

fn anti_scalar_ideal_point_inner_anti_product(self_342: AntiScalar, other_304: IdealPoint) -> IdealPoint {
    var self_343: AntiScalar;
    var other_305: IdealPoint;

    self_343 = self_342;
    other_305 = other_304;
    let _e4: AntiScalar = self_343;
    let _e7: IdealPoint = other_305;
    return IdealPoint((vec2<f32>(_e4.g0_) * _e7.g0_));
}

fn anti_scalar_ideal_point_left_anti_contraction(self_344: AntiScalar, other_306: IdealPoint) -> IdealPoint {
    var self_345: AntiScalar;
    var other_307: IdealPoint;

    self_345 = self_344;
    other_307 = other_306;
    let _e4: AntiScalar = self_345;
    let _e7: IdealPoint = other_307;
    return IdealPoint((vec2<f32>(_e4.g0_) * _e7.g0_));
}

fn anti_scalar_plane_add(self_346: AntiScalar, other_308: Plane) -> MotorDual {
    var self_347: AntiScalar;
    var other_309: Plane;

    self_347 = self_346;
    other_309 = other_308;
    let _e4: AntiScalar = self_347;
    let _e13: Plane = other_309;
    let _e16: Plane = other_309;
    let _e19: Plane = other_309;
    let _e22: Plane = other_309;
    return MotorDual(((vec4<f32>(_e4.g0_) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) + (vec4<f32>(_e13.g0_.x, _e16.g0_.x, _e19.g0_.y, _e22.g0_.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn anti_scalar_plane_sub(self_348: AntiScalar, other_310: Plane) -> MotorDual {
    var self_349: AntiScalar;
    var other_311: Plane;

    self_349 = self_348;
    other_311 = other_310;
    let _e4: AntiScalar = self_349;
    let _e13: Plane = other_311;
    let _e16: Plane = other_311;
    let _e19: Plane = other_311;
    let _e22: Plane = other_311;
    return MotorDual(((vec4<f32>(_e4.g0_) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) - (vec4<f32>(_e13.g0_.x, _e16.g0_.x, _e19.g0_.y, _e22.g0_.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn anti_scalar_plane_geometric_product(self_350: AntiScalar, other_312: Plane) -> Point {
    var self_351: AntiScalar;
    var other_313: Plane;

    self_351 = self_350;
    other_313 = other_312;
    let _e4: AntiScalar = self_351;
    let _e7: Plane = other_313;
    return Point((vec3<f32>(_e4.g0_) * _e7.g0_));
}

fn anti_scalar_plane_regressive_product(self_352: AntiScalar, other_314: Plane) -> Plane {
    var self_353: AntiScalar;
    var other_315: Plane;

    self_353 = self_352;
    other_315 = other_314;
    let _e4: AntiScalar = self_353;
    let _e7: Plane = other_315;
    return Plane((vec3<f32>(_e4.g0_) * _e7.g0_));
}

fn anti_scalar_plane_inner_product(self_354: AntiScalar, other_316: Plane) -> Point {
    var self_355: AntiScalar;
    var other_317: Plane;

    self_355 = self_354;
    other_317 = other_316;
    let _e4: AntiScalar = self_355;
    let _e7: Plane = other_317;
    return Point((vec3<f32>(_e4.g0_) * _e7.g0_));
}

fn anti_scalar_plane_geometric_anti_product(self_356: AntiScalar, other_318: Plane) -> Plane {
    var self_357: AntiScalar;
    var other_319: Plane;

    self_357 = self_356;
    other_319 = other_318;
    let _e4: AntiScalar = self_357;
    let _e7: Plane = other_319;
    return Plane((vec3<f32>(_e4.g0_) * _e7.g0_));
}

fn anti_scalar_plane_inner_anti_product(self_358: AntiScalar, other_320: Plane) -> Plane {
    var self_359: AntiScalar;
    var other_321: Plane;

    self_359 = self_358;
    other_321 = other_320;
    let _e4: AntiScalar = self_359;
    let _e7: Plane = other_321;
    return Plane((vec3<f32>(_e4.g0_) * _e7.g0_));
}

fn anti_scalar_plane_right_contraction(self_360: AntiScalar, other_322: Plane) -> Point {
    var self_361: AntiScalar;
    var other_323: Plane;

    self_361 = self_360;
    other_323 = other_322;
    let _e4: AntiScalar = self_361;
    let _e7: Plane = other_323;
    return Point((vec3<f32>(_e4.g0_) * _e7.g0_));
}

fn anti_scalar_plane_left_anti_contraction(self_362: AntiScalar, other_324: Plane) -> Plane {
    var self_363: AntiScalar;
    var other_325: Plane;

    self_363 = self_362;
    other_325 = other_324;
    let _e4: AntiScalar = self_363;
    let _e7: Plane = other_325;
    return Plane((vec3<f32>(_e4.g0_) * _e7.g0_));
}

fn anti_scalar_translator_regressive_product(self_364: AntiScalar, other_326: Translator) -> Translator {
    var self_365: AntiScalar;
    var other_327: Translator;

    self_365 = self_364;
    other_327 = other_326;
    let _e4: AntiScalar = self_365;
    let _e7: Translator = other_327;
    return Translator((vec3<f32>(_e4.g0_) * _e7.g0_));
}

fn anti_scalar_translator_outer_product(self_366: AntiScalar, other_328: Translator) -> AntiScalar {
    var self_367: AntiScalar;
    var other_329: Translator;

    self_367 = self_366;
    other_329 = other_328;
    let _e4: AntiScalar = self_367;
    let _e6: Translator = other_329;
    return AntiScalar((_e4.g0_ * _e6.g0_.x));
}

fn anti_scalar_translator_geometric_anti_product(self_368: AntiScalar, other_330: Translator) -> Translator {
    var self_369: AntiScalar;
    var other_331: Translator;

    self_369 = self_368;
    other_331 = other_330;
    let _e4: AntiScalar = self_369;
    let _e7: Translator = other_331;
    return Translator((vec3<f32>(_e4.g0_) * _e7.g0_));
}

fn anti_scalar_translator_inner_anti_product(self_370: AntiScalar, other_332: Translator) -> Translator {
    var self_371: AntiScalar;
    var other_333: Translator;

    self_371 = self_370;
    other_333 = other_332;
    let _e4: AntiScalar = self_371;
    let _e7: Translator = other_333;
    return Translator((vec3<f32>(_e4.g0_) * _e7.g0_));
}

fn anti_scalar_translator_left_anti_contraction(self_372: AntiScalar, other_334: Translator) -> Translator {
    var self_373: AntiScalar;
    var other_335: Translator;

    self_373 = self_372;
    other_335 = other_334;
    let _e4: AntiScalar = self_373;
    let _e7: Translator = other_335;
    return Translator((vec3<f32>(_e4.g0_) * _e7.g0_));
}

fn anti_scalar_motor_geometric_product(self_374: AntiScalar, other_336: Motor) -> MotorDual {
    var self_375: AntiScalar;
    var other_337: Motor;

    self_375 = self_374;
    other_337 = other_336;
    let _e4: AntiScalar = self_375;
    let _e7: Motor = other_337;
    return MotorDual(((vec4<f32>(_e4.g0_) * _e7.g0_) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))));
}

fn anti_scalar_motor_regressive_product(self_376: AntiScalar, other_338: Motor) -> Motor {
    var self_377: AntiScalar;
    var other_339: Motor;

    self_377 = self_376;
    other_339 = other_338;
    let _e4: AntiScalar = self_377;
    let _e7: Motor = other_339;
    return Motor((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn anti_scalar_motor_outer_product(self_378: AntiScalar, other_340: Motor) -> AntiScalar {
    var self_379: AntiScalar;
    var other_341: Motor;

    self_379 = self_378;
    other_341 = other_340;
    let _e4: AntiScalar = self_379;
    let _e6: Motor = other_341;
    return AntiScalar((_e4.g0_ * _e6.g0_.x));
}

fn anti_scalar_motor_inner_product(self_380: AntiScalar, other_342: Motor) -> MotorDual {
    var self_381: AntiScalar;
    var other_343: Motor;

    self_381 = self_380;
    other_343 = other_342;
    let _e4: AntiScalar = self_381;
    let _e7: Motor = other_343;
    return MotorDual(((vec4<f32>(_e4.g0_) * _e7.g0_) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))));
}

fn anti_scalar_motor_geometric_anti_product(self_382: AntiScalar, other_344: Motor) -> Motor {
    var self_383: AntiScalar;
    var other_345: Motor;

    self_383 = self_382;
    other_345 = other_344;
    let _e4: AntiScalar = self_383;
    let _e7: Motor = other_345;
    return Motor((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn anti_scalar_motor_inner_anti_product(self_384: AntiScalar, other_346: Motor) -> Motor {
    var self_385: AntiScalar;
    var other_347: Motor;

    self_385 = self_384;
    other_347 = other_346;
    let _e4: AntiScalar = self_385;
    let _e7: Motor = other_347;
    return Motor((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn anti_scalar_motor_right_contraction(self_386: AntiScalar, other_348: Motor) -> MotorDual {
    var self_387: AntiScalar;
    var other_349: Motor;

    self_387 = self_386;
    other_349 = other_348;
    let _e4: AntiScalar = self_387;
    let _e7: Motor = other_349;
    return MotorDual(((vec4<f32>(_e4.g0_) * _e7.g0_) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))));
}

fn anti_scalar_motor_left_anti_contraction(self_388: AntiScalar, other_350: Motor) -> Motor {
    var self_389: AntiScalar;
    var other_351: Motor;

    self_389 = self_388;
    other_351 = other_350;
    let _e4: AntiScalar = self_389;
    let _e7: Motor = other_351;
    return Motor((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn anti_scalar_motor_dual_add(self_390: AntiScalar, other_352: MotorDual) -> MotorDual {
    var self_391: AntiScalar;
    var other_353: MotorDual;

    self_391 = self_390;
    other_353 = other_352;
    let _e4: AntiScalar = self_391;
    let _e13: MotorDual = other_353;
    return MotorDual(((vec4<f32>(_e4.g0_) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) + _e13.g0_));
}

fn anti_scalar_motor_dual_sub(self_392: AntiScalar, other_354: MotorDual) -> MotorDual {
    var self_393: AntiScalar;
    var other_355: MotorDual;

    self_393 = self_392;
    other_355 = other_354;
    let _e4: AntiScalar = self_393;
    let _e13: MotorDual = other_355;
    return MotorDual(((vec4<f32>(_e4.g0_) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) - _e13.g0_));
}

fn anti_scalar_motor_dual_geometric_product(self_394: AntiScalar, other_356: MotorDual) -> Motor {
    var self_395: AntiScalar;
    var other_357: MotorDual;

    self_395 = self_394;
    other_357 = other_356;
    let _e4: AntiScalar = self_395;
    let _e7: MotorDual = other_357;
    return Motor(((vec4<f32>(_e4.g0_) * _e7.g0_) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0)));
}

fn anti_scalar_motor_dual_regressive_product(self_396: AntiScalar, other_358: MotorDual) -> MotorDual {
    var self_397: AntiScalar;
    var other_359: MotorDual;

    self_397 = self_396;
    other_359 = other_358;
    let _e4: AntiScalar = self_397;
    let _e7: MotorDual = other_359;
    return MotorDual((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn anti_scalar_motor_dual_inner_product(self_398: AntiScalar, other_360: MotorDual) -> Motor {
    var self_399: AntiScalar;
    var other_361: MotorDual;

    self_399 = self_398;
    other_361 = other_360;
    let _e4: AntiScalar = self_399;
    let _e7: MotorDual = other_361;
    return Motor(((vec4<f32>(_e4.g0_) * _e7.g0_) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0)));
}

fn anti_scalar_motor_dual_geometric_anti_product(self_400: AntiScalar, other_362: MotorDual) -> MotorDual {
    var self_401: AntiScalar;
    var other_363: MotorDual;

    self_401 = self_400;
    other_363 = other_362;
    let _e4: AntiScalar = self_401;
    let _e7: MotorDual = other_363;
    return MotorDual((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn anti_scalar_motor_dual_inner_anti_product(self_402: AntiScalar, other_364: MotorDual) -> MotorDual {
    var self_403: AntiScalar;
    var other_365: MotorDual;

    self_403 = self_402;
    other_365 = other_364;
    let _e4: AntiScalar = self_403;
    let _e7: MotorDual = other_365;
    return MotorDual((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn anti_scalar_motor_dual_left_contraction(self_404: AntiScalar, other_366: MotorDual) -> Scalar {
    var self_405: AntiScalar;
    var other_367: MotorDual;

    self_405 = self_404;
    other_367 = other_366;
    let _e5: AntiScalar = self_405;
    let _e7: MotorDual = other_367;
    return Scalar((0.0 - (_e5.g0_ * _e7.g0_.x)));
}

fn anti_scalar_motor_dual_right_contraction(self_406: AntiScalar, other_368: MotorDual) -> Motor {
    var self_407: AntiScalar;
    var other_369: MotorDual;

    self_407 = self_406;
    other_369 = other_368;
    let _e4: AntiScalar = self_407;
    let _e7: MotorDual = other_369;
    return Motor(((vec4<f32>(_e4.g0_) * _e7.g0_) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0)));
}

fn anti_scalar_motor_dual_left_anti_contraction(self_408: AntiScalar, other_370: MotorDual) -> MotorDual {
    var self_409: AntiScalar;
    var other_371: MotorDual;

    self_409 = self_408;
    other_371 = other_370;
    let _e4: AntiScalar = self_409;
    let _e7: MotorDual = other_371;
    return MotorDual((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn anti_scalar_motor_dual_right_anti_contraction(self_410: AntiScalar, other_372: MotorDual) -> AntiScalar {
    var self_411: AntiScalar;
    var other_373: MotorDual;

    self_411 = self_410;
    other_373 = other_372;
    let _e4: AntiScalar = self_411;
    let _e6: MotorDual = other_373;
    return AntiScalar((_e4.g0_ * _e6.g0_.x));
}

fn anti_scalar_motor_dual_scalar_product(self_412: AntiScalar, other_374: MotorDual) -> Scalar {
    var self_413: AntiScalar;
    var other_375: MotorDual;

    self_413 = self_412;
    other_375 = other_374;
    let _e5: AntiScalar = self_413;
    let _e7: MotorDual = other_375;
    return Scalar((0.0 - (_e5.g0_ * _e7.g0_.x)));
}

fn anti_scalar_motor_dual_anti_scalar_product(self_414: AntiScalar, other_376: MotorDual) -> AntiScalar {
    var self_415: AntiScalar;
    var other_377: MotorDual;

    self_415 = self_414;
    other_377 = other_376;
    let _e4: AntiScalar = self_415;
    let _e6: MotorDual = other_377;
    return AntiScalar((_e4.g0_ * _e6.g0_.x));
}

fn anti_scalar_squared_magnitude(self_416: AntiScalar) -> Scalar {
    var self_417: AntiScalar;

    self_417 = self_416;
    let _e2: AntiScalar = self_417;
    let _e3: AntiScalar = self_417;
    let _e4: AntiScalar = anti_scalar_reversal(_e3);
    let _e5: Scalar = anti_scalar_anti_scalar_scalar_product(_e2, _e4);
    return _e5;
}

fn anti_scalar_magnitude(self_418: AntiScalar) -> Scalar {
    var self_419: AntiScalar;

    self_419 = self_418;
    let _e2: AntiScalar = self_419;
    let _e3: Scalar = anti_scalar_squared_magnitude(_e2);
    return Scalar(sqrt(_e3.g0_));
}

fn anti_scalar_bulk_norm(self_420: AntiScalar) -> Scalar {
    var self_421: AntiScalar;

    self_421 = self_420;
    let _e2: AntiScalar = self_421;
    let _e3: Scalar = anti_scalar_squared_magnitude(_e2);
    return Scalar(sqrt(_e3.g0_));
}

fn anti_scalar_squared_anti_magnitude(self_422: AntiScalar) -> AntiScalar {
    var self_423: AntiScalar;

    self_423 = self_422;
    let _e2: AntiScalar = self_423;
    let _e3: AntiScalar = self_423;
    let _e4: AntiScalar = anti_scalar_anti_reversal(_e3);
    let _e5: AntiScalar = anti_scalar_anti_scalar_anti_scalar_product(_e2, _e4);
    return _e5;
}

fn anti_scalar_weight_norm(self_424: AntiScalar) -> AntiScalar {
    var self_425: AntiScalar;

    self_425 = self_424;
    let _e2: AntiScalar = self_425;
    let _e3: AntiScalar = anti_scalar_squared_anti_magnitude(_e2);
    return AntiScalar(sqrt(_e3.g0_));
}

fn anti_scalar_scale(self_426: AntiScalar, other_378: f32) -> AntiScalar {
    var self_427: AntiScalar;
    var other_379: f32;

    self_427 = self_426;
    other_379 = other_378;
    let _e4: AntiScalar = self_427;
    let _e5: f32 = other_379;
    let _e7: AntiScalar = anti_scalar_scalar_geometric_product(_e4, Scalar(_e5));
    return _e7;
}

fn anti_scalar_signum(self_428: AntiScalar) -> AntiScalar {
    var self_429: AntiScalar;

    self_429 = self_428;
    let _e2: AntiScalar = self_429;
    let _e3: AntiScalar = self_429;
    let _e4: Scalar = anti_scalar_magnitude(_e3);
    let _e9: AntiScalar = anti_scalar_scalar_geometric_product(_e2, Scalar((1.0 / _e4.g0_)));
    return _e9;
}

fn anti_scalar_inverse(self_430: AntiScalar) -> AntiScalar {
    var self_431: AntiScalar;

    self_431 = self_430;
    let _e2: AntiScalar = self_431;
    let _e3: AntiScalar = anti_scalar_reversal(_e2);
    let _e4: AntiScalar = self_431;
    let _e5: Scalar = anti_scalar_squared_magnitude(_e4);
    let _e10: AntiScalar = anti_scalar_scalar_geometric_product(_e3, Scalar((1.0 / _e5.g0_)));
    return _e10;
}

fn multi_vector_zero() -> MultiVector {
    return MultiVector(vec4<f32>(0.0), vec4<f32>(0.0));
}

fn multi_vector_one() -> MultiVector {
    return MultiVector(vec4<f32>(1.0, 0.0, 0.0, 0.0), vec4<f32>(0.0));
}

fn multi_vector_neg(self_432: MultiVector) -> MultiVector {
    var self_433: MultiVector;

    self_433 = self_432;
    let _e2: MultiVector = self_433;
    let _e8: MultiVector = self_433;
    return MultiVector((_e2.g0_ * vec4<f32>(-(1.0))), (_e8.g1_ * vec4<f32>(-(1.0))));
}

fn multi_vector_automorphism(self_434: MultiVector) -> MultiVector {
    var self_435: MultiVector;

    self_435 = self_434;
    let _e2: MultiVector = self_435;
    let _e12: MultiVector = self_435;
    return MultiVector((_e2.g0_ * vec4<f32>(1.0, 1.0, -(1.0), -(1.0))), (_e12.g1_ * vec4<f32>(-(1.0), -(1.0), 1.0, 1.0)));
}

fn multi_vector_reversal(self_436: MultiVector) -> MultiVector {
    var self_437: MultiVector;

    self_437 = self_436;
    let _e2: MultiVector = self_437;
    let _e11: MultiVector = self_437;
    return MultiVector((_e2.g0_ * vec4<f32>(1.0, -(1.0), 1.0, 1.0)), (_e11.g1_ * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))));
}

fn multi_vector_conjugation(self_438: MultiVector) -> MultiVector {
    var self_439: MultiVector;

    self_439 = self_438;
    let _e2: MultiVector = self_439;
    let _e13: MultiVector = self_439;
    return MultiVector((_e2.g0_ * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))), (_e13.g1_ * vec4<f32>(-(1.0), 1.0, -(1.0), -(1.0))));
}

fn multi_vector_dual(self_440: MultiVector) -> MultiVector {
    var self_441: MultiVector;

    self_441 = self_440;
    let _e2: MultiVector = self_441;
    let _e5: MultiVector = self_441;
    return MultiVector(_e2.g1_.yxwz, _e5.g0_.yxwz);
}

fn multi_vector_anti_reversal(self_442: MultiVector) -> MultiVector {
    var self_443: MultiVector;

    self_443 = self_442;
    let _e2: MultiVector = self_443;
    let _e13: MultiVector = self_443;
    return MultiVector((_e2.g0_ * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))), (_e13.g1_ * vec4<f32>(-(1.0), 1.0, -(1.0), -(1.0))));
}

fn multi_vector_scalar_into(self_444: MultiVector) -> Scalar {
    var self_445: MultiVector;

    self_445 = self_444;
    let _e2: MultiVector = self_445;
    return Scalar(_e2.g0_.x);
}

fn multi_vector_scalar_add(self_446: MultiVector, other_380: Scalar) -> MultiVector {
    var self_447: MultiVector;
    var other_381: Scalar;

    self_447 = self_446;
    other_381 = other_380;
    let _e4: MultiVector = self_447;
    let _e6: Scalar = other_381;
    let _e16: MultiVector = self_447;
    return MultiVector((_e4.g0_ + (vec4<f32>(_e6.g0_) * vec4<f32>(1.0, 0.0, 0.0, 0.0))), _e16.g1_);
}

fn multi_vector_scalar_sub(self_448: MultiVector, other_382: Scalar) -> MultiVector {
    var self_449: MultiVector;
    var other_383: Scalar;

    self_449 = self_448;
    other_383 = other_382;
    let _e4: MultiVector = self_449;
    let _e6: Scalar = other_383;
    let _e16: MultiVector = self_449;
    return MultiVector((_e4.g0_ - (vec4<f32>(_e6.g0_) * vec4<f32>(1.0, 0.0, 0.0, 0.0))), _e16.g1_);
}

fn multi_vector_scalar_geometric_product(self_450: MultiVector, other_384: Scalar) -> MultiVector {
    var self_451: MultiVector;
    var other_385: Scalar;

    self_451 = self_450;
    other_385 = other_384;
    let _e4: MultiVector = self_451;
    let _e6: Scalar = other_385;
    let _e10: MultiVector = self_451;
    let _e12: Scalar = other_385;
    return MultiVector((_e4.g0_ * vec4<f32>(_e6.g0_)), (_e10.g1_ * vec4<f32>(_e12.g0_)));
}

fn multi_vector_scalar_regressive_product(self_452: MultiVector, other_386: Scalar) -> Scalar {
    var self_453: MultiVector;
    var other_387: Scalar;

    self_453 = self_452;
    other_387 = other_386;
    let _e4: MultiVector = self_453;
    let _e7: Scalar = other_387;
    return Scalar((_e4.g1_.y * _e7.g0_));
}

fn multi_vector_scalar_outer_product(self_454: MultiVector, other_388: Scalar) -> MultiVector {
    var self_455: MultiVector;
    var other_389: Scalar;

    self_455 = self_454;
    other_389 = other_388;
    let _e4: MultiVector = self_455;
    let _e6: Scalar = other_389;
    let _e10: MultiVector = self_455;
    let _e12: Scalar = other_389;
    return MultiVector((_e4.g0_ * vec4<f32>(_e6.g0_)), (_e10.g1_ * vec4<f32>(_e12.g0_)));
}

fn multi_vector_scalar_inner_product(self_456: MultiVector, other_390: Scalar) -> MultiVector {
    var self_457: MultiVector;
    var other_391: Scalar;

    self_457 = self_456;
    other_391 = other_390;
    let _e4: MultiVector = self_457;
    let _e6: Scalar = other_391;
    let _e10: MultiVector = self_457;
    let _e12: Scalar = other_391;
    return MultiVector((_e4.g0_ * vec4<f32>(_e6.g0_)), (_e10.g1_ * vec4<f32>(_e12.g0_)));
}

fn multi_vector_scalar_geometric_anti_product(self_458: MultiVector, other_392: Scalar) -> MultiVector {
    var self_459: MultiVector;
    var other_393: Scalar;

    self_459 = self_458;
    other_393 = other_392;
    let _e4: MultiVector = self_459;
    let _e7: Scalar = other_393;
    let _e18: MultiVector = self_459;
    let _e21: Scalar = other_393;
    return MultiVector(((_e4.g1_.yxwz * vec4<f32>(_e7.g0_)) * vec4<f32>(1.0, -(1.0), 1.0, 1.0)), ((_e18.g0_.yxwz * vec4<f32>(_e21.g0_)) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))));
}

fn multi_vector_scalar_inner_anti_product(self_460: MultiVector, other_394: Scalar) -> MultiVector {
    var self_461: MultiVector;
    var other_395: Scalar;

    self_461 = self_460;
    other_395 = other_394;
    let _e4: MultiVector = self_461;
    let _e7: Scalar = other_395;
    let _e18: MultiVector = self_461;
    let _e21: Scalar = other_395;
    return MultiVector(((_e4.g1_.yxwz * vec4<f32>(_e7.g0_)) * vec4<f32>(1.0, -(1.0), 1.0, 1.0)), ((_e18.g0_.yxwz * vec4<f32>(_e21.g0_)) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))));
}

fn multi_vector_scalar_left_contraction(self_462: MultiVector, other_396: Scalar) -> Scalar {
    var self_463: MultiVector;
    var other_397: Scalar;

    self_463 = self_462;
    other_397 = other_396;
    let _e4: MultiVector = self_463;
    let _e7: Scalar = other_397;
    return Scalar((_e4.g0_.x * _e7.g0_));
}

fn multi_vector_scalar_right_contraction(self_464: MultiVector, other_398: Scalar) -> MultiVector {
    var self_465: MultiVector;
    var other_399: Scalar;

    self_465 = self_464;
    other_399 = other_398;
    let _e4: MultiVector = self_465;
    let _e6: Scalar = other_399;
    let _e10: MultiVector = self_465;
    let _e12: Scalar = other_399;
    return MultiVector((_e4.g0_ * vec4<f32>(_e6.g0_)), (_e10.g1_ * vec4<f32>(_e12.g0_)));
}

fn multi_vector_scalar_left_anti_contraction(self_466: MultiVector, other_400: Scalar) -> MultiVector {
    var self_467: MultiVector;
    var other_401: Scalar;

    self_467 = self_466;
    other_401 = other_400;
    let _e4: MultiVector = self_467;
    let _e7: Scalar = other_401;
    let _e18: MultiVector = self_467;
    let _e21: Scalar = other_401;
    return MultiVector(((_e4.g1_.yxwz * vec4<f32>(_e7.g0_)) * vec4<f32>(1.0, -(1.0), 1.0, 1.0)), ((_e18.g0_.yxwz * vec4<f32>(_e21.g0_)) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))));
}

fn multi_vector_scalar_right_anti_contraction(self_468: MultiVector, other_402: Scalar) -> AntiScalar {
    var self_469: MultiVector;
    var other_403: Scalar;

    self_469 = self_468;
    other_403 = other_402;
    let _e5: MultiVector = self_469;
    let _e8: Scalar = other_403;
    return AntiScalar((0.0 - (_e5.g0_.x * _e8.g0_)));
}

fn multi_vector_scalar_scalar_product(self_470: MultiVector, other_404: Scalar) -> Scalar {
    var self_471: MultiVector;
    var other_405: Scalar;

    self_471 = self_470;
    other_405 = other_404;
    let _e4: MultiVector = self_471;
    let _e7: Scalar = other_405;
    return Scalar((_e4.g0_.x * _e7.g0_));
}

fn multi_vector_scalar_anti_scalar_product(self_472: MultiVector, other_406: Scalar) -> AntiScalar {
    var self_473: MultiVector;
    var other_407: Scalar;

    self_473 = self_472;
    other_407 = other_406;
    let _e5: MultiVector = self_473;
    let _e8: Scalar = other_407;
    return AntiScalar((0.0 - (_e5.g0_.x * _e8.g0_)));
}

fn multi_vector_anti_scalar_into(self_474: MultiVector) -> AntiScalar {
    var self_475: MultiVector;

    self_475 = self_474;
    let _e2: MultiVector = self_475;
    return AntiScalar(_e2.g1_.y);
}

fn multi_vector_anti_scalar_add(self_476: MultiVector, other_408: AntiScalar) -> MultiVector {
    var self_477: MultiVector;
    var other_409: AntiScalar;

    self_477 = self_476;
    other_409 = other_408;
    let _e4: MultiVector = self_477;
    let _e6: MultiVector = self_477;
    let _e8: AntiScalar = other_409;
    return MultiVector(_e4.g0_, (_e6.g1_ + (vec4<f32>(_e8.g0_) * vec4<f32>(0.0, 1.0, 0.0, 0.0))));
}

fn multi_vector_anti_scalar_sub(self_478: MultiVector, other_410: AntiScalar) -> MultiVector {
    var self_479: MultiVector;
    var other_411: AntiScalar;

    self_479 = self_478;
    other_411 = other_410;
    let _e4: MultiVector = self_479;
    let _e6: MultiVector = self_479;
    let _e8: AntiScalar = other_411;
    return MultiVector(_e4.g0_, (_e6.g1_ - (vec4<f32>(_e8.g0_) * vec4<f32>(0.0, 1.0, 0.0, 0.0))));
}

fn multi_vector_anti_scalar_geometric_product(self_480: MultiVector, other_412: AntiScalar) -> MultiVector {
    var self_481: MultiVector;
    var other_413: AntiScalar;

    self_481 = self_480;
    other_413 = other_412;
    let _e4: MultiVector = self_481;
    let _e7: AntiScalar = other_413;
    let _e20: MultiVector = self_481;
    let _e23: AntiScalar = other_413;
    return MultiVector(((_e4.g1_.yxwz * vec4<f32>(_e7.g0_)) * vec4<f32>(-(1.0), 1.0, -(1.0), -(1.0))), ((_e20.g0_.yxwz * vec4<f32>(_e23.g0_)) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0)));
}

fn multi_vector_anti_scalar_regressive_product(self_482: MultiVector, other_414: AntiScalar) -> MultiVector {
    var self_483: MultiVector;
    var other_415: AntiScalar;

    self_483 = self_482;
    other_415 = other_414;
    let _e4: MultiVector = self_483;
    let _e6: AntiScalar = other_415;
    let _e10: MultiVector = self_483;
    let _e12: AntiScalar = other_415;
    return MultiVector((_e4.g0_ * vec4<f32>(_e6.g0_)), (_e10.g1_ * vec4<f32>(_e12.g0_)));
}

fn multi_vector_anti_scalar_outer_product(self_484: MultiVector, other_416: AntiScalar) -> AntiScalar {
    var self_485: MultiVector;
    var other_417: AntiScalar;

    self_485 = self_484;
    other_417 = other_416;
    let _e4: MultiVector = self_485;
    let _e7: AntiScalar = other_417;
    return AntiScalar((_e4.g0_.x * _e7.g0_));
}

fn multi_vector_anti_scalar_inner_product(self_486: MultiVector, other_418: AntiScalar) -> MultiVector {
    var self_487: MultiVector;
    var other_419: AntiScalar;

    self_487 = self_486;
    other_419 = other_418;
    let _e4: MultiVector = self_487;
    let _e7: AntiScalar = other_419;
    let _e20: MultiVector = self_487;
    let _e23: AntiScalar = other_419;
    return MultiVector(((_e4.g1_.yxwz * vec4<f32>(_e7.g0_)) * vec4<f32>(-(1.0), 1.0, -(1.0), -(1.0))), ((_e20.g0_.yxwz * vec4<f32>(_e23.g0_)) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0)));
}

fn multi_vector_anti_scalar_geometric_anti_product(self_488: MultiVector, other_420: AntiScalar) -> MultiVector {
    var self_489: MultiVector;
    var other_421: AntiScalar;

    self_489 = self_488;
    other_421 = other_420;
    let _e4: MultiVector = self_489;
    let _e6: AntiScalar = other_421;
    let _e10: MultiVector = self_489;
    let _e12: AntiScalar = other_421;
    return MultiVector((_e4.g0_ * vec4<f32>(_e6.g0_)), (_e10.g1_ * vec4<f32>(_e12.g0_)));
}

fn multi_vector_anti_scalar_inner_anti_product(self_490: MultiVector, other_422: AntiScalar) -> MultiVector {
    var self_491: MultiVector;
    var other_423: AntiScalar;

    self_491 = self_490;
    other_423 = other_422;
    let _e4: MultiVector = self_491;
    let _e6: AntiScalar = other_423;
    let _e10: MultiVector = self_491;
    let _e12: AntiScalar = other_423;
    return MultiVector((_e4.g0_ * vec4<f32>(_e6.g0_)), (_e10.g1_ * vec4<f32>(_e12.g0_)));
}

fn multi_vector_anti_scalar_left_contraction(self_492: MultiVector, other_424: AntiScalar) -> MultiVector {
    var self_493: MultiVector;
    var other_425: AntiScalar;

    self_493 = self_492;
    other_425 = other_424;
    let _e4: MultiVector = self_493;
    let _e7: AntiScalar = other_425;
    let _e20: MultiVector = self_493;
    let _e23: AntiScalar = other_425;
    return MultiVector(((_e4.g1_.yxwz * vec4<f32>(_e7.g0_)) * vec4<f32>(-(1.0), 1.0, -(1.0), -(1.0))), ((_e20.g0_.yxwz * vec4<f32>(_e23.g0_)) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0)));
}

fn multi_vector_anti_scalar_right_contraction(self_494: MultiVector, other_426: AntiScalar) -> Scalar {
    var self_495: MultiVector;
    var other_427: AntiScalar;

    self_495 = self_494;
    other_427 = other_426;
    let _e5: MultiVector = self_495;
    let _e8: AntiScalar = other_427;
    return Scalar((0.0 - (_e5.g1_.y * _e8.g0_)));
}

fn multi_vector_anti_scalar_left_anti_contraction(self_496: MultiVector, other_428: AntiScalar) -> AntiScalar {
    var self_497: MultiVector;
    var other_429: AntiScalar;

    self_497 = self_496;
    other_429 = other_428;
    let _e4: MultiVector = self_497;
    let _e7: AntiScalar = other_429;
    return AntiScalar((_e4.g1_.y * _e7.g0_));
}

fn multi_vector_anti_scalar_right_anti_contraction(self_498: MultiVector, other_430: AntiScalar) -> MultiVector {
    var self_499: MultiVector;
    var other_431: AntiScalar;

    self_499 = self_498;
    other_431 = other_430;
    let _e4: MultiVector = self_499;
    let _e6: AntiScalar = other_431;
    let _e10: MultiVector = self_499;
    let _e12: AntiScalar = other_431;
    return MultiVector((_e4.g0_ * vec4<f32>(_e6.g0_)), (_e10.g1_ * vec4<f32>(_e12.g0_)));
}

fn multi_vector_anti_scalar_scalar_product(self_500: MultiVector, other_432: AntiScalar) -> Scalar {
    var self_501: MultiVector;
    var other_433: AntiScalar;

    self_501 = self_500;
    other_433 = other_432;
    let _e5: MultiVector = self_501;
    let _e8: AntiScalar = other_433;
    return Scalar((0.0 - (_e5.g1_.y * _e8.g0_)));
}

fn multi_vector_anti_scalar_anti_scalar_product(self_502: MultiVector, other_434: AntiScalar) -> AntiScalar {
    var self_503: MultiVector;
    var other_435: AntiScalar;

    self_503 = self_502;
    other_435 = other_434;
    let _e4: MultiVector = self_503;
    let _e7: AntiScalar = other_435;
    return AntiScalar((_e4.g1_.y * _e7.g0_));
}

fn multi_vector_multi_vector_add(self_504: MultiVector, other_436: MultiVector) -> MultiVector {
    var self_505: MultiVector;
    var other_437: MultiVector;

    self_505 = self_504;
    other_437 = other_436;
    let _e4: MultiVector = self_505;
    let _e6: MultiVector = other_437;
    let _e9: MultiVector = self_505;
    let _e11: MultiVector = other_437;
    return MultiVector((_e4.g0_ + _e6.g0_), (_e9.g1_ + _e11.g1_));
}

fn multi_vector_multi_vector_sub(self_506: MultiVector, other_438: MultiVector) -> MultiVector {
    var self_507: MultiVector;
    var other_439: MultiVector;

    self_507 = self_506;
    other_439 = other_438;
    let _e4: MultiVector = self_507;
    let _e6: MultiVector = other_439;
    let _e9: MultiVector = self_507;
    let _e11: MultiVector = other_439;
    return MultiVector((_e4.g0_ - _e6.g0_), (_e9.g1_ - _e11.g1_));
}

fn multi_vector_multi_vector_mul(self_508: MultiVector, other_440: MultiVector) -> MultiVector {
    var self_509: MultiVector;
    var other_441: MultiVector;

    self_509 = self_508;
    other_441 = other_440;
    let _e4: MultiVector = self_509;
    let _e6: MultiVector = other_441;
    let _e9: MultiVector = self_509;
    let _e11: MultiVector = other_441;
    return MultiVector((_e4.g0_ * _e6.g0_), (_e9.g1_ * _e11.g1_));
}

fn multi_vector_multi_vector_div(self_510: MultiVector, other_442: MultiVector) -> MultiVector {
    var self_511: MultiVector;
    var other_443: MultiVector;

    self_511 = self_510;
    other_443 = other_442;
    let _e4: MultiVector = self_511;
    let _e7: MultiVector = self_511;
    let _e10: MultiVector = self_511;
    let _e13: MultiVector = self_511;
    let _e23: MultiVector = other_443;
    let _e26: MultiVector = other_443;
    let _e29: MultiVector = other_443;
    let _e32: MultiVector = other_443;
    let _e43: MultiVector = self_511;
    let _e46: MultiVector = self_511;
    let _e49: MultiVector = self_511;
    let _e52: MultiVector = self_511;
    let _e62: MultiVector = other_443;
    let _e65: MultiVector = other_443;
    let _e68: MultiVector = other_443;
    let _e71: MultiVector = other_443;
    return MultiVector((((vec4<f32>(_e4.g0_.x, _e7.g0_.y, _e10.g0_.z, _e13.g0_.w) * vec4<f32>(1.0, 1.0, 1.0, 1.0)) / vec4<f32>(_e23.g0_.x, _e26.g0_.y, _e29.g0_.z, _e32.g0_.w)) * vec4<f32>(1.0, 1.0, 1.0, 1.0)), (((vec4<f32>(_e43.g1_.x, _e46.g1_.y, _e49.g1_.z, _e52.g1_.w) * vec4<f32>(1.0, 1.0, 1.0, 1.0)) / vec4<f32>(_e62.g1_.x, _e65.g1_.y, _e68.g1_.z, _e71.g1_.w)) * vec4<f32>(1.0, 1.0, 1.0, 1.0)));
}

fn multi_vector_multi_vector_geometric_product(self_512: MultiVector, other_444: MultiVector) -> MultiVector {
    var self_513: MultiVector;
    var other_445: MultiVector;

    self_513 = self_512;
    other_445 = other_444;
    let _e4: MultiVector = self_513;
    let _e8: MultiVector = other_445;
    let _e11: MultiVector = self_513;
    let _e15: MultiVector = other_445;
    let _e28: MultiVector = self_513;
    let _e32: MultiVector = other_445;
    let _e37: MultiVector = self_513;
    let _e41: MultiVector = other_445;
    let _e54: MultiVector = self_513;
    let _e58: MultiVector = other_445;
    let _e69: MultiVector = self_513;
    let _e73: MultiVector = other_445;
    let _e87: MultiVector = self_513;
    let _e91: MultiVector = other_445;
    let _e105: MultiVector = self_513;
    let _e109: MultiVector = other_445;
    let _e123: MultiVector = self_513;
    let _e127: MultiVector = other_445;
    let _e130: MultiVector = self_513;
    let _e134: MultiVector = other_445;
    let _e147: MultiVector = self_513;
    let _e151: MultiVector = other_445;
    let _e164: MultiVector = self_513;
    let _e168: MultiVector = other_445;
    let _e173: MultiVector = self_513;
    let _e177: MultiVector = other_445;
    let _e188: MultiVector = self_513;
    let _e192: MultiVector = other_445;
    let _e204: MultiVector = self_513;
    let _e208: MultiVector = other_445;
    let _e220: MultiVector = self_513;
    let _e224: MultiVector = other_445;
    return MultiVector(((((((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.y) * _e15.g0_.yxwz) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + (vec4<f32>(_e28.g0_.z) * _e32.g0_.zwxy)) + ((vec4<f32>(_e37.g0_.w) * _e41.g0_.wzyx) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e54.g1_.x) * _e58.g1_) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e69.g1_.y) * _e73.g1_.yxwz) * vec4<f32>(-(1.0), 1.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e87.g1_.z) * _e91.g1_.zwxy) * vec4<f32>(-(1.0), 1.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e105.g1_.w) * _e109.g1_.wzyx) * vec4<f32>(-(1.0), -(1.0), -(1.0), 1.0))), ((((((((vec4<f32>(_e123.g0_.x) * _e127.g1_) + ((vec4<f32>(_e130.g0_.y) * _e134.g1_.yxwz) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))) + ((vec4<f32>(_e147.g0_.z) * _e151.g1_.zwxy) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))) + (vec4<f32>(_e164.g0_.w) * _e168.g1_.wzyx)) + ((vec4<f32>(_e173.g1_.x) * _e177.g0_) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e188.g1_.y) * _e192.g0_.yxwz) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0))) + ((vec4<f32>(_e204.g1_.z) * _e208.g0_.zwxy) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e220.g1_.w) * _e224.g0_.wzyx) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0))));
}

fn multi_vector_multi_vector_regressive_product(self_514: MultiVector, other_446: MultiVector) -> MultiVector {
    var self_515: MultiVector;
    var other_447: MultiVector;

    self_515 = self_514;
    other_447 = other_446;
    let _e4: MultiVector = self_515;
    let _e8: MultiVector = other_447;
    let _e18: MultiVector = self_515;
    let _e22: MultiVector = other_447;
    let _e33: MultiVector = self_515;
    let _e37: MultiVector = other_447;
    let _e48: MultiVector = self_515;
    let _e52: MultiVector = other_447;
    let _e64: MultiVector = self_515;
    let _e68: MultiVector = other_447;
    let _e72: MultiVector = self_515;
    let _e76: MultiVector = other_447;
    let _e87: MultiVector = self_515;
    let _e91: MultiVector = other_447;
    let _e103: MultiVector = self_515;
    let _e107: MultiVector = other_447;
    let _e118: MultiVector = self_515;
    let _e122: MultiVector = other_447;
    let _e125: MultiVector = self_515;
    let _e129: MultiVector = other_447;
    let _e141: MultiVector = self_515;
    let _e145: MultiVector = other_447;
    let _e156: MultiVector = self_515;
    let _e160: MultiVector = other_447;
    return MultiVector((((((((((vec4<f32>(_e4.g0_.y) * _e8.g1_) * vec4<f32>(1.0, 1.0, -(1.0), 1.0)) + ((vec4<f32>(_e18.g0_.z) * _e22.g1_.wwyw) * vec4<f32>(1.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e33.g0_.w) * _e37.g1_.zzzy) * vec4<f32>(1.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e48.g1_.x) * vec4<f32>(_e52.g0_.y)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + (vec4<f32>(_e64.g1_.y) * _e68.g0_)) + ((vec4<f32>(_e72.g1_.z) * _e76.g0_.wwyw) * vec4<f32>(1.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e87.g1_.w) * _e91.g0_.zzzy) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e103.g0_.x) * _e107.g1_.yxxx) * vec4<f32>(1.0, 0.0, 0.0, 0.0))), ((((vec4<f32>(_e118.g1_.y) * _e122.g1_) + ((vec4<f32>(_e125.g1_.z) * _e129.g1_.wwyw) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0))) + ((vec4<f32>(_e141.g1_.w) * _e145.g1_.zzzy) * vec4<f32>(1.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e156.g1_.x) * _e160.g1_.yxxx) * vec4<f32>(1.0, 0.0, 0.0, 0.0))));
}

fn multi_vector_multi_vector_outer_product(self_516: MultiVector, other_448: MultiVector) -> MultiVector {
    var self_517: MultiVector;
    var other_449: MultiVector;

    self_517 = self_516;
    other_449 = other_448;
    let _e4: MultiVector = self_517;
    let _e8: MultiVector = other_449;
    let _e11: MultiVector = self_517;
    let _e15: MultiVector = other_449;
    let _e26: MultiVector = self_517;
    let _e30: MultiVector = other_449;
    let _e42: MultiVector = self_517;
    let _e45: MultiVector = other_449;
    let _e57: MultiVector = self_517;
    let _e61: MultiVector = other_449;
    let _e64: MultiVector = self_517;
    let _e68: MultiVector = other_449;
    let _e80: MultiVector = self_517;
    let _e84: MultiVector = other_449;
    let _e95: MultiVector = self_517;
    let _e99: MultiVector = other_449;
    let _e110: MultiVector = self_517;
    let _e114: MultiVector = other_449;
    let _e126: MultiVector = self_517;
    let _e130: MultiVector = other_449;
    let _e141: MultiVector = self_517;
    let _e145: MultiVector = other_449;
    let _e156: MultiVector = self_517;
    let _e159: MultiVector = other_449;
    return MultiVector(((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.z) * _e15.g0_.wwxw) * vec4<f32>(0.0, 1.0, 1.0, 0.0))) + ((vec4<f32>(_e26.g0_.w) * _e30.g0_.zzzx) * vec4<f32>(0.0, -(1.0), 0.0, 1.0))) + ((_e42.g0_.xyxx * vec4<f32>(_e45.g0_.x)) * vec4<f32>(0.0, 1.0, 0.0, 0.0))), ((((((((vec4<f32>(_e57.g0_.x) * _e61.g1_) + ((vec4<f32>(_e64.g0_.z) * _e68.g1_.wwxw) * vec4<f32>(0.0, 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e80.g0_.w) * _e84.g1_.zzzx) * vec4<f32>(0.0, 1.0, 0.0, 1.0))) + ((vec4<f32>(_e95.g1_.x) * _e99.g0_) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e110.g1_.y) * vec4<f32>(_e114.g0_.x)) * vec4<f32>(0.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e126.g1_.z) * _e130.g0_.wwxw) * vec4<f32>(0.0, 1.0, 1.0, 0.0))) + ((vec4<f32>(_e141.g1_.w) * _e145.g0_.zzzx) * vec4<f32>(0.0, 1.0, 0.0, 1.0))) + ((_e156.g0_.xyxx * vec4<f32>(_e159.g1_.x)) * vec4<f32>(0.0, 1.0, 0.0, 0.0))));
}

fn multi_vector_multi_vector_inner_product(self_518: MultiVector, other_450: MultiVector) -> MultiVector {
    var self_519: MultiVector;
    var other_451: MultiVector;

    self_519 = self_518;
    other_451 = other_450;
    let _e4: MultiVector = self_519;
    let _e8: MultiVector = other_451;
    let _e11: MultiVector = self_519;
    let _e15: MultiVector = other_451;
    let _e28: MultiVector = self_519;
    let _e32: MultiVector = other_451;
    let _e44: MultiVector = self_519;
    let _e48: MultiVector = other_451;
    let _e59: MultiVector = self_519;
    let _e63: MultiVector = other_451;
    let _e77: MultiVector = self_519;
    let _e81: MultiVector = other_451;
    let _e95: MultiVector = self_519;
    let _e99: MultiVector = other_451;
    let _e112: MultiVector = self_519;
    let _e115: MultiVector = other_451;
    let _e126: MultiVector = self_519;
    let _e130: MultiVector = other_451;
    let _e133: MultiVector = self_519;
    let _e137: MultiVector = other_451;
    let _e149: MultiVector = self_519;
    let _e153: MultiVector = other_451;
    let _e164: MultiVector = self_519;
    let _e168: MultiVector = other_451;
    let _e180: MultiVector = self_519;
    let _e184: MultiVector = other_451;
    let _e196: MultiVector = self_519;
    let _e200: MultiVector = other_451;
    let _e211: MultiVector = self_519;
    let _e215: MultiVector = other_451;
    let _e227: MultiVector = self_519;
    let _e230: MultiVector = other_451;
    return MultiVector(((((((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.y) * _e15.g0_.yxwz) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e28.g0_.w) * _e32.g0_.wwyx) * vec4<f32>(1.0, 0.0, -(1.0), 1.0))) + ((vec4<f32>(_e44.g1_.x) * _e48.g1_) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e59.g1_.y) * _e63.g1_.yxwz) * vec4<f32>(-(1.0), 1.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e77.g1_.z) * _e81.g1_.zzxy) * vec4<f32>(-(1.0), 0.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e95.g1_.w) * _e99.g1_.wwyx) * vec4<f32>(-(1.0), 0.0, -(1.0), 1.0))) + ((_e112.g0_.zxzz * _e115.g0_.zxxy) * vec4<f32>(1.0, 0.0, 1.0, 1.0))), ((((((((vec4<f32>(_e126.g0_.x) * _e130.g1_) + ((vec4<f32>(_e133.g0_.z) * _e137.g1_.zzzy) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))) + ((vec4<f32>(_e149.g0_.w) * _e153.g1_.wwyw) * vec4<f32>(1.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e164.g1_.x) * vec4<f32>(_e168.g0_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e180.g1_.y) * _e184.g0_.yxwz) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0))) + ((vec4<f32>(_e196.g1_.z) * _e200.g0_.zzxz) * vec4<f32>(1.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e211.g1_.w) * _e215.g0_.wwwx) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))) + ((_e227.g0_.yxxx * _e230.g1_.yxxx) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))));
}

fn multi_vector_multi_vector_geometric_anti_product(self_520: MultiVector, other_452: MultiVector) -> MultiVector {
    var self_521: MultiVector;
    var other_453: MultiVector;

    self_521 = self_520;
    other_453 = other_452;
    let _e4: MultiVector = self_521;
    let _e8: MultiVector = other_453;
    let _e19: MultiVector = self_521;
    let _e23: MultiVector = other_453;
    let _e34: MultiVector = self_521;
    let _e38: MultiVector = other_453;
    let _e50: MultiVector = self_521;
    let _e54: MultiVector = other_453;
    let _e66: MultiVector = self_521;
    let _e70: MultiVector = other_453;
    let _e83: MultiVector = self_521;
    let _e87: MultiVector = other_453;
    let _e91: MultiVector = self_521;
    let _e95: MultiVector = other_453;
    let _e100: MultiVector = self_521;
    let _e104: MultiVector = other_453;
    let _e117: MultiVector = self_521;
    let _e121: MultiVector = other_453;
    let _e134: MultiVector = self_521;
    let _e138: MultiVector = other_453;
    let _e149: MultiVector = self_521;
    let _e153: MultiVector = other_453;
    let _e167: MultiVector = self_521;
    let _e171: MultiVector = other_453;
    let _e185: MultiVector = self_521;
    let _e189: MultiVector = other_453;
    let _e202: MultiVector = self_521;
    let _e206: MultiVector = other_453;
    let _e210: MultiVector = self_521;
    let _e214: MultiVector = other_453;
    let _e227: MultiVector = self_521;
    let _e231: MultiVector = other_453;
    return MultiVector((((((((((vec4<f32>(_e4.g0_.x) * _e8.g1_.yxwz) * vec4<f32>(1.0, -(1.0), 1.0, 1.0)) + ((vec4<f32>(_e19.g0_.y) * _e23.g1_) * vec4<f32>(1.0, 1.0, -(1.0), 1.0))) + ((vec4<f32>(_e34.g0_.z) * _e38.g1_.wzyx) * vec4<f32>(1.0, -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e50.g0_.w) * _e54.g1_.zwxy) * vec4<f32>(1.0, 1.0, -(1.0), 1.0))) + ((vec4<f32>(_e66.g1_.x) * _e70.g0_.yxwz) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))) + (vec4<f32>(_e83.g1_.y) * _e87.g0_)) + (vec4<f32>(_e91.g1_.z) * _e95.g0_.wzyx)) + ((vec4<f32>(_e100.g1_.w) * _e104.g0_.zwxy) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))), (((((((((vec4<f32>(_e117.g0_.x) * _e121.g0_.yxwz) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))) + ((vec4<f32>(_e134.g0_.y) * _e138.g0_) * vec4<f32>(1.0, 1.0, -(1.0), 1.0))) + ((vec4<f32>(_e149.g0_.z) * _e153.g0_.wzyx) * vec4<f32>(-(1.0), -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e167.g0_.w) * _e171.g0_.zwxy) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0)))) + ((vec4<f32>(_e185.g1_.x) * _e189.g1_.yxwz) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0))) + (vec4<f32>(_e202.g1_.y) * _e206.g1_)) + ((vec4<f32>(_e210.g1_.z) * _e214.g1_.wzyx) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + (vec4<f32>(_e227.g1_.w) * _e231.g1_.zwxy)));
}

fn multi_vector_multi_vector_inner_anti_product(self_522: MultiVector, other_454: MultiVector) -> MultiVector {
    var self_523: MultiVector;
    var other_455: MultiVector;

    self_523 = self_522;
    other_455 = other_454;
    let _e4: MultiVector = self_523;
    let _e8: MultiVector = other_455;
    let _e19: MultiVector = self_523;
    let _e23: MultiVector = other_455;
    let _e35: MultiVector = self_523;
    let _e39: MultiVector = other_455;
    let _e50: MultiVector = self_523;
    let _e54: MultiVector = other_455;
    let _e67: MultiVector = self_523;
    let _e71: MultiVector = other_455;
    let _e75: MultiVector = self_523;
    let _e79: MultiVector = other_455;
    let _e90: MultiVector = self_523;
    let _e94: MultiVector = other_455;
    let _e106: MultiVector = self_523;
    let _e109: MultiVector = other_455;
    let _e120: MultiVector = self_523;
    let _e124: MultiVector = other_455;
    let _e137: MultiVector = self_523;
    let _e141: MultiVector = other_455;
    let _e152: MultiVector = self_523;
    let _e156: MultiVector = other_455;
    let _e170: MultiVector = self_523;
    let _e174: MultiVector = other_455;
    let _e187: MultiVector = self_523;
    let _e191: MultiVector = other_455;
    let _e195: MultiVector = self_523;
    let _e199: MultiVector = other_455;
    let _e211: MultiVector = self_523;
    let _e215: MultiVector = other_455;
    let _e226: MultiVector = self_523;
    let _e229: MultiVector = other_455;
    return MultiVector((((((((((vec4<f32>(_e4.g0_.x) * _e8.g1_.yxwz) * vec4<f32>(1.0, -(1.0), 1.0, 1.0)) + ((vec4<f32>(_e19.g0_.z) * _e23.g1_.zzyz) * vec4<f32>(0.0, -(1.0), 1.0, 0.0))) + ((vec4<f32>(_e35.g0_.w) * _e39.g1_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, 1.0))) + ((vec4<f32>(_e50.g1_.x) * vec4<f32>(_e54.g0_.x)) * vec4<f32>(0.0, -(1.0), 0.0, 0.0))) + (vec4<f32>(_e67.g1_.y) * _e71.g0_)) + ((vec4<f32>(_e75.g1_.z) * _e79.g0_.zzzx) * vec4<f32>(0.0, 1.0, 0.0, 1.0))) + ((vec4<f32>(_e90.g1_.w) * _e94.g0_.wwxw) * vec4<f32>(0.0, -(1.0), 1.0, 0.0))) + ((_e106.g0_.xyxx * _e109.g1_.xyxx) * vec4<f32>(0.0, 1.0, 0.0, 0.0))), (((((((((vec4<f32>(_e120.g0_.x) * _e124.g0_.yxwz) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))) + ((vec4<f32>(_e137.g0_.y) * _e141.g0_) * vec4<f32>(1.0, 1.0, -(1.0), 1.0))) + ((vec4<f32>(_e152.g0_.w) * _e156.g0_.wwxy) * vec4<f32>(0.0, -(1.0), -(1.0), -(1.0)))) + ((vec4<f32>(_e170.g1_.x) * _e174.g1_.yxwz) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0))) + (vec4<f32>(_e187.g1_.y) * _e191.g1_)) + ((vec4<f32>(_e195.g1_.z) * _e199.g1_.zzyx) * vec4<f32>(0.0, 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e211.g1_.w) * _e215.g1_.wwxy) * vec4<f32>(0.0, 1.0, 1.0, 1.0))) + ((_e226.g0_.xzzz * _e229.g0_.xzyx) * vec4<f32>(0.0, -(1.0), 1.0, -(1.0)))));
}

fn multi_vector_multi_vector_left_contraction(self_524: MultiVector, other_456: MultiVector) -> MultiVector {
    var self_525: MultiVector;
    var other_457: MultiVector;

    self_525 = self_524;
    other_457 = other_456;
    let _e4: MultiVector = self_525;
    let _e8: MultiVector = other_457;
    let _e11: MultiVector = self_525;
    let _e15: MultiVector = other_457;
    let _e26: MultiVector = self_525;
    let _e30: MultiVector = other_457;
    let _e42: MultiVector = self_525;
    let _e46: MultiVector = other_457;
    let _e57: MultiVector = self_525;
    let _e61: MultiVector = other_457;
    let _e74: MultiVector = self_525;
    let _e78: MultiVector = other_457;
    let _e91: MultiVector = self_525;
    let _e95: MultiVector = other_457;
    let _e108: MultiVector = self_525;
    let _e111: MultiVector = other_457;
    let _e123: MultiVector = self_525;
    let _e127: MultiVector = other_457;
    let _e130: MultiVector = self_525;
    let _e134: MultiVector = other_457;
    let _e146: MultiVector = self_525;
    let _e150: MultiVector = other_457;
    let _e161: MultiVector = self_525;
    let _e164: MultiVector = other_457;
    return MultiVector(((((((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.z) * _e15.g0_.zzzy) * vec4<f32>(1.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e26.g0_.w) * _e30.g0_.wwyw) * vec4<f32>(1.0, 0.0, -(1.0), 0.0))) + ((vec4<f32>(_e42.g1_.x) * _e46.g1_) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e57.g1_.y) * vec4<f32>(_e61.g1_.y)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((vec4<f32>(_e74.g1_.z) * _e78.g1_.zzzy) * vec4<f32>(-(1.0), 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e91.g1_.w) * _e95.g1_.wwyw) * vec4<f32>(-(1.0), 0.0, -(1.0), 0.0))) + ((_e108.g0_.yxxx * _e111.g0_.yxxx) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))), ((((vec4<f32>(_e123.g0_.x) * _e127.g1_) + ((vec4<f32>(_e130.g0_.z) * _e134.g1_.zzzy) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))) + ((vec4<f32>(_e146.g0_.w) * _e150.g1_.wwyw) * vec4<f32>(1.0, 0.0, 1.0, 0.0))) + ((_e161.g0_.yxxx * _e164.g1_.yxxx) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))));
}

fn multi_vector_multi_vector_right_contraction(self_526: MultiVector, other_458: MultiVector) -> MultiVector {
    var self_527: MultiVector;
    var other_459: MultiVector;

    self_527 = self_526;
    other_459 = other_458;
    let _e4: MultiVector = self_527;
    let _e8: MultiVector = other_459;
    let _e20: MultiVector = self_527;
    let _e24: MultiVector = other_459;
    let _e35: MultiVector = self_527;
    let _e39: MultiVector = other_459;
    let _e50: MultiVector = self_527;
    let _e54: MultiVector = other_459;
    let _e66: MultiVector = self_527;
    let _e70: MultiVector = other_459;
    let _e84: MultiVector = self_527;
    let _e88: MultiVector = other_459;
    let _e101: MultiVector = self_527;
    let _e105: MultiVector = other_459;
    let _e117: MultiVector = self_527;
    let _e121: MultiVector = other_459;
    let _e133: MultiVector = self_527;
    let _e137: MultiVector = other_459;
    let _e148: MultiVector = self_527;
    let _e152: MultiVector = other_459;
    let _e163: MultiVector = self_527;
    let _e167: MultiVector = other_459;
    let _e179: MultiVector = self_527;
    let _e183: MultiVector = other_459;
    return MultiVector((((((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.yxwz) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0))) + ((vec4<f32>(_e20.g0_.z) * _e24.g0_.zzxz) * vec4<f32>(1.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e35.g0_.w) * _e39.g0_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e50.g1_.x) * vec4<f32>(_e54.g1_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e66.g1_.y) * _e70.g1_.yxwz) * vec4<f32>(-(1.0), 1.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e84.g1_.z) * _e88.g1_.zzxz) * vec4<f32>(-(1.0), 0.0, -(1.0), 0.0))) + ((vec4<f32>(_e101.g1_.w) * _e105.g1_.wwwx) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))) + ((vec4<f32>(_e117.g0_.x) * vec4<f32>(_e121.g0_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))), (((((vec4<f32>(_e133.g1_.y) * _e137.g0_.yxwz) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0)) + ((vec4<f32>(_e148.g1_.z) * _e152.g0_.zzxz) * vec4<f32>(1.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e163.g1_.w) * _e167.g0_.wwwx) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))) + ((vec4<f32>(_e179.g1_.x) * vec4<f32>(_e183.g0_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))));
}

fn multi_vector_multi_vector_left_anti_contraction(self_528: MultiVector, other_460: MultiVector) -> MultiVector {
    var self_529: MultiVector;
    var other_461: MultiVector;

    self_529 = self_528;
    other_461 = other_460;
    let _e4: MultiVector = self_529;
    let _e8: MultiVector = other_461;
    let _e11: MultiVector = self_529;
    let _e15: MultiVector = other_461;
    let _e26: MultiVector = self_529;
    let _e30: MultiVector = other_461;
    let _e42: MultiVector = self_529;
    let _e46: MultiVector = other_461;
    let _e59: MultiVector = self_529;
    let _e63: MultiVector = other_461;
    let _e73: MultiVector = self_529;
    let _e77: MultiVector = other_461;
    let _e90: MultiVector = self_529;
    let _e94: MultiVector = other_461;
    let _e107: MultiVector = self_529;
    let _e111: MultiVector = other_461;
    let _e124: MultiVector = self_529;
    let _e128: MultiVector = other_461;
    let _e132: MultiVector = self_529;
    let _e136: MultiVector = other_461;
    let _e148: MultiVector = self_529;
    let _e152: MultiVector = other_461;
    let _e163: MultiVector = self_529;
    let _e167: MultiVector = other_461;
    return MultiVector(((((vec4<f32>(_e4.g1_.y) * _e8.g0_) + ((vec4<f32>(_e11.g1_.z) * _e15.g0_.zzzx) * vec4<f32>(0.0, 1.0, 0.0, 1.0))) + ((vec4<f32>(_e26.g1_.w) * _e30.g0_.wwxw) * vec4<f32>(0.0, -(1.0), 1.0, 0.0))) + ((vec4<f32>(_e42.g1_.x) * vec4<f32>(_e46.g0_.x)) * vec4<f32>(0.0, -(1.0), 0.0, 0.0))), (((((((((vec4<f32>(_e59.g0_.y) * _e63.g0_) * vec4<f32>(1.0, 1.0, -(1.0), 1.0)) + ((vec4<f32>(_e73.g0_.z) * _e77.g0_.zzzx) * vec4<f32>(0.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e90.g0_.w) * _e94.g0_.wwxw) * vec4<f32>(0.0, -(1.0), -(1.0), 0.0))) + ((vec4<f32>(_e107.g1_.x) * vec4<f32>(_e111.g1_.x)) * vec4<f32>(0.0, -(1.0), 0.0, 0.0))) + (vec4<f32>(_e124.g1_.y) * _e128.g1_)) + ((vec4<f32>(_e132.g1_.z) * _e136.g1_.zzzx) * vec4<f32>(0.0, 1.0, 0.0, -(1.0)))) + ((vec4<f32>(_e148.g1_.w) * _e152.g1_.wwxw) * vec4<f32>(0.0, 1.0, 1.0, 0.0))) + ((vec4<f32>(_e163.g0_.x) * vec4<f32>(_e167.g0_.x)) * vec4<f32>(0.0, -(1.0), 0.0, 0.0))));
}

fn multi_vector_multi_vector_right_anti_contraction(self_530: MultiVector, other_462: MultiVector) -> MultiVector {
    var self_531: MultiVector;
    var other_463: MultiVector;

    self_531 = self_530;
    other_463 = other_462;
    let _e4: MultiVector = self_531;
    let _e8: MultiVector = other_463;
    let _e19: MultiVector = self_531;
    let _e23: MultiVector = other_463;
    let _e35: MultiVector = self_531;
    let _e39: MultiVector = other_463;
    let _e50: MultiVector = self_531;
    let _e53: MultiVector = other_463;
    let _e64: MultiVector = self_531;
    let _e68: MultiVector = other_463;
    let _e81: MultiVector = self_531;
    let _e85: MultiVector = other_463;
    let _e97: MultiVector = self_531;
    let _e101: MultiVector = other_463;
    let _e114: MultiVector = self_531;
    let _e118: MultiVector = other_463;
    let _e131: MultiVector = self_531;
    let _e135: MultiVector = other_463;
    let _e147: MultiVector = self_531;
    let _e151: MultiVector = other_463;
    let _e162: MultiVector = self_531;
    let _e166: MultiVector = other_463;
    let _e177: MultiVector = self_531;
    let _e180: MultiVector = other_463;
    return MultiVector((((((vec4<f32>(_e4.g0_.x) * _e8.g1_.yxwz) * vec4<f32>(1.0, -(1.0), 1.0, 1.0)) + ((vec4<f32>(_e19.g0_.z) * _e23.g1_.zzyz) * vec4<f32>(0.0, -(1.0), 1.0, 0.0))) + ((vec4<f32>(_e35.g0_.w) * _e39.g1_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, 1.0))) + ((_e50.g0_.xyxx * _e53.g1_.xyxx) * vec4<f32>(0.0, 1.0, 0.0, 0.0))), (((((((((vec4<f32>(_e64.g0_.x) * _e68.g0_.yxwz) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))) + ((vec4<f32>(_e81.g0_.z) * _e85.g0_.zzyz) * vec4<f32>(0.0, -(1.0), 1.0, 0.0))) + ((vec4<f32>(_e97.g0_.w) * _e101.g0_.wwwy) * vec4<f32>(0.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e114.g1_.x) * _e118.g1_.yxwz) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e131.g1_.y) * vec4<f32>(_e135.g1_.y)) * vec4<f32>(0.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e147.g1_.z) * _e151.g1_.zzyz) * vec4<f32>(0.0, 1.0, 1.0, 0.0))) + ((vec4<f32>(_e162.g1_.w) * _e166.g1_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, 1.0))) + ((_e177.g0_.xyxx * _e180.g0_.xyxx) * vec4<f32>(0.0, 1.0, 0.0, 0.0))));
}

fn multi_vector_multi_vector_scalar_product(self_532: MultiVector, other_464: MultiVector) -> Scalar {
    var self_533: MultiVector;
    var other_465: MultiVector;

    self_533 = self_532;
    other_465 = other_464;
    let _e4: MultiVector = self_533;
    let _e7: MultiVector = other_465;
    let _e11: MultiVector = self_533;
    let _e14: MultiVector = other_465;
    let _e19: MultiVector = self_533;
    let _e22: MultiVector = other_465;
    let _e27: MultiVector = self_533;
    let _e30: MultiVector = other_465;
    let _e35: MultiVector = self_533;
    let _e38: MultiVector = other_465;
    let _e43: MultiVector = self_533;
    let _e46: MultiVector = other_465;
    let _e51: MultiVector = self_533;
    let _e54: MultiVector = other_465;
    let _e59: MultiVector = self_533;
    let _e62: MultiVector = other_465;
    return Scalar(((((((((_e4.g0_.x * _e7.g0_.x) - (_e11.g0_.y * _e14.g0_.y)) + (_e19.g0_.z * _e22.g0_.z)) + (_e27.g0_.w * _e30.g0_.w)) + (_e35.g1_.x * _e38.g1_.x)) - (_e43.g1_.y * _e46.g1_.y)) - (_e51.g1_.z * _e54.g1_.z)) - (_e59.g1_.w * _e62.g1_.w)));
}

fn multi_vector_multi_vector_anti_scalar_product(self_534: MultiVector, other_466: MultiVector) -> AntiScalar {
    var self_535: MultiVector;
    var other_467: MultiVector;

    self_535 = self_534;
    other_467 = other_466;
    let _e5: MultiVector = self_535;
    let _e8: MultiVector = other_467;
    let _e13: MultiVector = self_535;
    let _e16: MultiVector = other_467;
    let _e21: MultiVector = self_535;
    let _e24: MultiVector = other_467;
    let _e29: MultiVector = self_535;
    let _e32: MultiVector = other_467;
    let _e37: MultiVector = self_535;
    let _e40: MultiVector = other_467;
    let _e45: MultiVector = self_535;
    let _e48: MultiVector = other_467;
    let _e53: MultiVector = self_535;
    let _e56: MultiVector = other_467;
    let _e61: MultiVector = self_535;
    let _e64: MultiVector = other_467;
    return AntiScalar(((((((((0.0 - (_e5.g0_.x * _e8.g0_.x)) + (_e13.g0_.y * _e16.g0_.y)) - (_e21.g0_.z * _e24.g0_.z)) - (_e29.g0_.w * _e32.g0_.w)) - (_e37.g1_.x * _e40.g1_.x)) + (_e45.g1_.y * _e48.g1_.y)) + (_e53.g1_.z * _e56.g1_.z)) + (_e61.g1_.w * _e64.g1_.w)));
}

fn multi_vector_rotor_into(self_536: MultiVector) -> Rotor {
    var self_537: MultiVector;

    self_537 = self_536;
    let _e2: MultiVector = self_537;
    let _e5: MultiVector = self_537;
    return Rotor(vec2<f32>(_e2.g0_.x, _e5.g0_.y));
}

fn multi_vector_rotor_add(self_538: MultiVector, other_468: Rotor) -> MultiVector {
    var self_539: MultiVector;
    var other_469: Rotor;

    self_539 = self_538;
    other_469 = other_468;
    let _e4: MultiVector = self_539;
    let _e6: Rotor = other_469;
    let _e9: Rotor = other_469;
    let _e12: Rotor = other_469;
    let _e15: Rotor = other_469;
    let _e26: MultiVector = self_539;
    return MultiVector((_e4.g0_ + (vec4<f32>(_e6.g0_.x, _e9.g0_.y, _e12.g0_.x, _e15.g0_.x) * vec4<f32>(1.0, 1.0, 0.0, 0.0))), _e26.g1_);
}

fn multi_vector_rotor_sub(self_540: MultiVector, other_470: Rotor) -> MultiVector {
    var self_541: MultiVector;
    var other_471: Rotor;

    self_541 = self_540;
    other_471 = other_470;
    let _e4: MultiVector = self_541;
    let _e6: Rotor = other_471;
    let _e9: Rotor = other_471;
    let _e12: Rotor = other_471;
    let _e15: Rotor = other_471;
    let _e26: MultiVector = self_541;
    return MultiVector((_e4.g0_ - (vec4<f32>(_e6.g0_.x, _e9.g0_.y, _e12.g0_.x, _e15.g0_.x) * vec4<f32>(1.0, 1.0, 0.0, 0.0))), _e26.g1_);
}

fn multi_vector_rotor_geometric_product(self_542: MultiVector, other_472: Rotor) -> MultiVector {
    var self_543: MultiVector;
    var other_473: Rotor;

    self_543 = self_542;
    other_473 = other_472;
    let _e4: MultiVector = self_543;
    let _e8: Rotor = other_473;
    let _e11: Rotor = other_473;
    let _e14: Rotor = other_473;
    let _e17: Rotor = other_473;
    let _e29: MultiVector = self_543;
    let _e33: Rotor = other_473;
    let _e36: Rotor = other_473;
    let _e39: Rotor = other_473;
    let _e42: Rotor = other_473;
    let _e55: MultiVector = self_543;
    let _e58: Rotor = other_473;
    let _e61: Rotor = other_473;
    let _e64: Rotor = other_473;
    let _e67: Rotor = other_473;
    let _e73: MultiVector = self_543;
    let _e77: Rotor = other_473;
    let _e80: Rotor = other_473;
    let _e83: Rotor = other_473;
    let _e86: Rotor = other_473;
    let _e98: MultiVector = self_543;
    let _e102: Rotor = other_473;
    let _e105: Rotor = other_473;
    let _e108: Rotor = other_473;
    let _e111: Rotor = other_473;
    let _e123: MultiVector = self_543;
    let _e126: Rotor = other_473;
    let _e129: Rotor = other_473;
    let _e132: Rotor = other_473;
    let _e135: Rotor = other_473;
    return MultiVector(((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y, _e11.g0_.x, _e14.g0_.y, _e17.g0_.y)) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0)) + ((vec4<f32>(_e29.g0_.w) * vec4<f32>(_e33.g0_.y, _e36.g0_.y, _e39.g0_.y, _e42.g0_.x)) * vec4<f32>(0.0, 0.0, -(1.0), 1.0))) + (_e55.g0_.xxzz * vec4<f32>(_e58.g0_.x, _e61.g0_.y, _e64.g0_.x, _e67.g0_.y))), ((((vec4<f32>(_e73.g1_.y) * vec4<f32>(_e77.g0_.y, _e80.g0_.x, _e83.g0_.y, _e86.g0_.y)) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0)) + ((vec4<f32>(_e98.g1_.w) * vec4<f32>(_e102.g0_.y, _e105.g0_.y, _e108.g0_.y, _e111.g0_.x)) * vec4<f32>(0.0, 0.0, 1.0, 1.0))) + ((_e123.g1_.xxzz * vec4<f32>(_e126.g0_.x, _e129.g0_.y, _e132.g0_.x, _e135.g0_.y)) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))));
}

fn multi_vector_rotor_outer_product(self_544: MultiVector, other_474: Rotor) -> MultiVector {
    var self_545: MultiVector;
    var other_475: Rotor;

    self_545 = self_544;
    other_475 = other_474;
    let _e4: MultiVector = self_545;
    let _e8: Rotor = other_475;
    let _e19: MultiVector = self_545;
    let _e22: Rotor = other_475;
    let _e25: Rotor = other_475;
    let _e28: Rotor = other_475;
    let _e31: Rotor = other_475;
    let _e37: MultiVector = self_545;
    let _e41: Rotor = other_475;
    let _e52: MultiVector = self_545;
    let _e55: Rotor = other_475;
    let _e58: Rotor = other_475;
    let _e61: Rotor = other_475;
    let _e64: Rotor = other_475;
    return MultiVector((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.x)) * vec4<f32>(0.0, 1.0, 0.0, 0.0)) + (_e19.g0_.xxzw * vec4<f32>(_e22.g0_.x, _e25.g0_.y, _e28.g0_.x, _e31.g0_.x))), (((vec4<f32>(_e37.g1_.y) * vec4<f32>(_e41.g0_.x)) * vec4<f32>(0.0, 1.0, 0.0, 0.0)) + (_e52.g1_.xxzw * vec4<f32>(_e55.g0_.x, _e58.g0_.y, _e61.g0_.x, _e64.g0_.x))));
}

fn multi_vector_rotor_inner_product(self_546: MultiVector, other_476: Rotor) -> MultiVector {
    var self_547: MultiVector;
    var other_477: Rotor;

    self_547 = self_546;
    other_477 = other_476;
    let _e4: MultiVector = self_547;
    let _e8: Rotor = other_477;
    let _e11: Rotor = other_477;
    let _e14: Rotor = other_477;
    let _e17: Rotor = other_477;
    let _e29: MultiVector = self_547;
    let _e33: Rotor = other_477;
    let _e36: Rotor = other_477;
    let _e39: Rotor = other_477;
    let _e42: Rotor = other_477;
    let _e55: MultiVector = self_547;
    let _e58: Rotor = other_477;
    let _e61: Rotor = other_477;
    let _e64: Rotor = other_477;
    let _e67: Rotor = other_477;
    let _e73: MultiVector = self_547;
    let _e77: Rotor = other_477;
    let _e80: Rotor = other_477;
    let _e83: Rotor = other_477;
    let _e86: Rotor = other_477;
    let _e98: MultiVector = self_547;
    let _e101: Rotor = other_477;
    return MultiVector(((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y, _e11.g0_.x, _e14.g0_.y, _e17.g0_.y)) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0)) + ((vec4<f32>(_e29.g0_.w) * vec4<f32>(_e33.g0_.y, _e36.g0_.y, _e39.g0_.y, _e42.g0_.x)) * vec4<f32>(0.0, 0.0, -(1.0), 1.0))) + (_e55.g0_.xxzz * vec4<f32>(_e58.g0_.x, _e61.g0_.y, _e64.g0_.x, _e67.g0_.y))), (((vec4<f32>(_e73.g1_.y) * vec4<f32>(_e77.g0_.y, _e80.g0_.x, _e83.g0_.y, _e86.g0_.y)) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0)) + ((_e98.g1_.xxzw * vec4<f32>(_e101.g0_.x)) * vec4<f32>(1.0, 0.0, 1.0, 1.0))));
}

fn multi_vector_rotor_geometric_anti_product(self_548: MultiVector, other_478: Rotor) -> MultiVector {
    var self_549: MultiVector;
    var other_479: Rotor;

    self_549 = self_548;
    other_479 = other_478;
    let _e4: MultiVector = self_549;
    let _e8: Rotor = other_479;
    let _e11: Rotor = other_479;
    let _e14: Rotor = other_479;
    let _e17: Rotor = other_479;
    let _e28: MultiVector = self_549;
    let _e32: Rotor = other_479;
    let _e35: Rotor = other_479;
    let _e38: Rotor = other_479;
    let _e41: Rotor = other_479;
    let _e54: MultiVector = self_549;
    let _e57: Rotor = other_479;
    let _e60: Rotor = other_479;
    let _e63: Rotor = other_479;
    let _e66: Rotor = other_479;
    let _e79: MultiVector = self_549;
    let _e83: Rotor = other_479;
    let _e86: Rotor = other_479;
    let _e89: Rotor = other_479;
    let _e92: Rotor = other_479;
    let _e103: MultiVector = self_549;
    let _e107: Rotor = other_479;
    let _e110: Rotor = other_479;
    let _e113: Rotor = other_479;
    let _e116: Rotor = other_479;
    let _e130: MultiVector = self_549;
    let _e133: Rotor = other_479;
    let _e136: Rotor = other_479;
    let _e139: Rotor = other_479;
    let _e142: Rotor = other_479;
    return MultiVector(((((vec4<f32>(_e4.g1_.y) * vec4<f32>(_e8.g0_.x, _e11.g0_.y, _e14.g0_.x, _e17.g0_.x)) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) + ((vec4<f32>(_e28.g1_.w) * vec4<f32>(_e32.g0_.x, _e35.g0_.x, _e38.g0_.x, _e41.g0_.y)) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((_e54.g1_.xxzz * vec4<f32>(_e57.g0_.y, _e60.g0_.x, _e63.g0_.y, _e66.g0_.x)) * vec4<f32>(1.0, -(1.0), 1.0, 1.0))), ((((vec4<f32>(_e79.g0_.y) * vec4<f32>(_e83.g0_.x, _e86.g0_.y, _e89.g0_.x, _e92.g0_.x)) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) + ((vec4<f32>(_e103.g0_.w) * vec4<f32>(_e107.g0_.x, _e110.g0_.x, _e113.g0_.x, _e116.g0_.y)) * vec4<f32>(0.0, 0.0, -(1.0), -(1.0)))) + ((_e130.g0_.xxzz * vec4<f32>(_e133.g0_.y, _e136.g0_.x, _e139.g0_.y, _e142.g0_.x)) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))));
}

fn multi_vector_rotor_inner_anti_product(self_550: MultiVector, other_480: Rotor) -> MultiVector {
    var self_551: MultiVector;
    var other_481: Rotor;

    self_551 = self_550;
    other_481 = other_480;
    let _e4: MultiVector = self_551;
    let _e8: Rotor = other_481;
    let _e11: Rotor = other_481;
    let _e14: Rotor = other_481;
    let _e17: Rotor = other_481;
    let _e28: MultiVector = self_551;
    let _e31: Rotor = other_481;
    let _e44: MultiVector = self_551;
    let _e48: Rotor = other_481;
    let _e51: Rotor = other_481;
    let _e54: Rotor = other_481;
    let _e57: Rotor = other_481;
    let _e68: MultiVector = self_551;
    let _e72: Rotor = other_481;
    let _e75: Rotor = other_481;
    let _e78: Rotor = other_481;
    let _e81: Rotor = other_481;
    let _e95: MultiVector = self_551;
    let _e98: Rotor = other_481;
    let _e101: Rotor = other_481;
    let _e104: Rotor = other_481;
    let _e107: Rotor = other_481;
    return MultiVector((((vec4<f32>(_e4.g1_.y) * vec4<f32>(_e8.g0_.x, _e11.g0_.y, _e14.g0_.x, _e17.g0_.x)) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) + ((_e28.g1_.xxwz * vec4<f32>(_e31.g0_.x)) * vec4<f32>(0.0, -(1.0), 1.0, 1.0))), ((((vec4<f32>(_e44.g0_.y) * vec4<f32>(_e48.g0_.x, _e51.g0_.y, _e54.g0_.x, _e57.g0_.x)) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) + ((vec4<f32>(_e68.g0_.w) * vec4<f32>(_e72.g0_.x, _e75.g0_.x, _e78.g0_.x, _e81.g0_.y)) * vec4<f32>(0.0, 0.0, -(1.0), -(1.0)))) + ((_e95.g0_.xxzz * vec4<f32>(_e98.g0_.y, _e101.g0_.x, _e104.g0_.y, _e107.g0_.x)) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))));
}

fn multi_vector_rotor_right_contraction(self_552: MultiVector, other_482: Rotor) -> MultiVector {
    var self_553: MultiVector;
    var other_483: Rotor;

    self_553 = self_552;
    other_483 = other_482;
    let _e4: MultiVector = self_553;
    let _e8: Rotor = other_483;
    let _e11: Rotor = other_483;
    let _e14: Rotor = other_483;
    let _e17: Rotor = other_483;
    let _e29: MultiVector = self_553;
    let _e32: Rotor = other_483;
    let _e44: MultiVector = self_553;
    let _e48: Rotor = other_483;
    let _e51: Rotor = other_483;
    let _e54: Rotor = other_483;
    let _e57: Rotor = other_483;
    let _e69: MultiVector = self_553;
    let _e72: Rotor = other_483;
    return MultiVector((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y, _e11.g0_.x, _e14.g0_.y, _e17.g0_.y)) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0)) + ((_e29.g0_.xxzw * vec4<f32>(_e32.g0_.x)) * vec4<f32>(1.0, 0.0, 1.0, 1.0))), (((vec4<f32>(_e44.g1_.y) * vec4<f32>(_e48.g0_.y, _e51.g0_.x, _e54.g0_.y, _e57.g0_.y)) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0)) + ((_e69.g1_.xxzw * vec4<f32>(_e72.g0_.x)) * vec4<f32>(1.0, 0.0, 1.0, 1.0))));
}

fn multi_vector_rotor_left_anti_contraction(self_554: MultiVector, other_484: Rotor) -> MultiVector {
    var self_555: MultiVector;
    var other_485: Rotor;

    self_555 = self_554;
    other_485 = other_484;
    let _e4: MultiVector = self_555;
    let _e8: Rotor = other_485;
    let _e11: Rotor = other_485;
    let _e14: Rotor = other_485;
    let _e17: Rotor = other_485;
    let _e28: MultiVector = self_555;
    let _e31: Rotor = other_485;
    let _e44: MultiVector = self_555;
    let _e48: Rotor = other_485;
    let _e51: Rotor = other_485;
    let _e54: Rotor = other_485;
    let _e57: Rotor = other_485;
    let _e68: MultiVector = self_555;
    let _e71: Rotor = other_485;
    return MultiVector((((vec4<f32>(_e4.g1_.y) * vec4<f32>(_e8.g0_.x, _e11.g0_.y, _e14.g0_.x, _e17.g0_.x)) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) + ((_e28.g1_.xxwz * vec4<f32>(_e31.g0_.x)) * vec4<f32>(0.0, -(1.0), 1.0, 1.0))), (((vec4<f32>(_e44.g0_.y) * vec4<f32>(_e48.g0_.x, _e51.g0_.y, _e54.g0_.x, _e57.g0_.x)) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) + ((_e68.g0_.xxwz * vec4<f32>(_e71.g0_.x)) * vec4<f32>(0.0, -(1.0), -(1.0), -(1.0)))));
}

fn multi_vector_rotor_scalar_product(self_556: MultiVector, other_486: Rotor) -> Scalar {
    var self_557: MultiVector;
    var other_487: Rotor;

    self_557 = self_556;
    other_487 = other_486;
    let _e4: MultiVector = self_557;
    let _e7: Rotor = other_487;
    let _e11: MultiVector = self_557;
    let _e14: Rotor = other_487;
    return Scalar(((_e4.g0_.x * _e7.g0_.x) - (_e11.g0_.y * _e14.g0_.y)));
}

fn multi_vector_rotor_anti_scalar_product(self_558: MultiVector, other_488: Rotor) -> AntiScalar {
    var self_559: MultiVector;
    var other_489: Rotor;

    self_559 = self_558;
    other_489 = other_488;
    let _e5: MultiVector = self_559;
    let _e8: Rotor = other_489;
    let _e13: MultiVector = self_559;
    let _e16: Rotor = other_489;
    return AntiScalar(((0.0 - (_e5.g0_.x * _e8.g0_.x)) + (_e13.g0_.y * _e16.g0_.y)));
}

fn multi_vector_point_into(self_560: MultiVector) -> Point {
    var self_561: MultiVector;

    self_561 = self_560;
    let _e2: MultiVector = self_561;
    let _e5: MultiVector = self_561;
    let _e8: MultiVector = self_561;
    return Point(vec3<f32>(_e2.g0_.y, _e5.g1_.z, _e8.g1_.w));
}

fn multi_vector_point_add(self_562: MultiVector, other_490: Point) -> MultiVector {
    var self_563: MultiVector;
    var other_491: Point;

    self_563 = self_562;
    other_491 = other_490;
    let _e4: MultiVector = self_563;
    let _e6: Point = other_491;
    let _e17: MultiVector = self_563;
    let _e19: Point = other_491;
    let _e22: Point = other_491;
    let _e25: Point = other_491;
    let _e28: Point = other_491;
    return MultiVector((_e4.g0_ + (vec4<f32>(_e6.g0_.x) * vec4<f32>(0.0, 1.0, 0.0, 0.0))), (_e17.g1_ + (vec4<f32>(_e19.g0_.x, _e22.g0_.x, _e25.g0_.y, _e28.g0_.z) * vec4<f32>(0.0, 0.0, 1.0, 1.0))));
}

fn multi_vector_point_sub(self_564: MultiVector, other_492: Point) -> MultiVector {
    var self_565: MultiVector;
    var other_493: Point;

    self_565 = self_564;
    other_493 = other_492;
    let _e4: MultiVector = self_565;
    let _e6: Point = other_493;
    let _e17: MultiVector = self_565;
    let _e19: Point = other_493;
    let _e22: Point = other_493;
    let _e25: Point = other_493;
    let _e28: Point = other_493;
    return MultiVector((_e4.g0_ - (vec4<f32>(_e6.g0_.x) * vec4<f32>(0.0, 1.0, 0.0, 0.0))), (_e17.g1_ - (vec4<f32>(_e19.g0_.x, _e22.g0_.x, _e25.g0_.y, _e28.g0_.z) * vec4<f32>(0.0, 0.0, 1.0, 1.0))));
}

fn multi_vector_point_geometric_product(self_566: MultiVector, other_494: Point) -> MultiVector {
    var self_567: MultiVector;
    var other_495: Point;

    self_567 = self_566;
    other_495 = other_494;
    let _e4: MultiVector = self_567;
    let _e8: Point = other_495;
    let _e11: Point = other_495;
    let _e14: Point = other_495;
    let _e17: Point = other_495;
    let _e29: MultiVector = self_567;
    let _e33: Point = other_495;
    let _e36: Point = other_495;
    let _e39: Point = other_495;
    let _e42: Point = other_495;
    let _e56: MultiVector = self_567;
    let _e60: Point = other_495;
    let _e63: Point = other_495;
    let _e66: Point = other_495;
    let _e69: Point = other_495;
    let _e82: MultiVector = self_567;
    let _e86: Point = other_495;
    let _e89: Point = other_495;
    let _e92: Point = other_495;
    let _e95: Point = other_495;
    let _e109: MultiVector = self_567;
    let _e112: Point = other_495;
    let _e126: MultiVector = self_567;
    let _e130: Point = other_495;
    let _e133: Point = other_495;
    let _e136: Point = other_495;
    let _e139: Point = other_495;
    let _e151: MultiVector = self_567;
    let _e155: Point = other_495;
    let _e158: Point = other_495;
    let _e161: Point = other_495;
    let _e164: Point = other_495;
    let _e176: MultiVector = self_567;
    let _e180: Point = other_495;
    let _e192: MultiVector = self_567;
    let _e196: Point = other_495;
    let _e209: MultiVector = self_567;
    let _e213: Point = other_495;
    let _e226: MultiVector = self_567;
    let _e230: Point = other_495;
    let _e242: MultiVector = self_567;
    let _e245: Point = other_495;
    let _e248: Point = other_495;
    let _e251: Point = other_495;
    let _e254: Point = other_495;
    return MultiVector(((((((vec4<f32>(_e4.g1_.x) * vec4<f32>(_e8.g0_.y, _e11.g0_.y, _e14.g0_.y, _e17.g0_.z)) * vec4<f32>(0.0, 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e29.g1_.y) * vec4<f32>(_e33.g0_.z, _e36.g0_.z, _e39.g0_.z, _e42.g0_.y)) * vec4<f32>(0.0, 0.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e56.g1_.z) * vec4<f32>(_e60.g0_.y, _e63.g0_.z, _e66.g0_.y, _e69.g0_.y)) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))) + ((vec4<f32>(_e82.g1_.w) * vec4<f32>(_e86.g0_.z, _e89.g0_.y, _e92.g0_.z, _e95.g0_.z)) * vec4<f32>(-(1.0), -(1.0), 0.0, 0.0))) + ((_e109.g0_.yxwz * vec4<f32>(_e112.g0_.x)) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))), ((((((((vec4<f32>(_e126.g0_.y) * vec4<f32>(_e130.g0_.z, _e133.g0_.z, _e136.g0_.z, _e139.g0_.y)) * vec4<f32>(0.0, 0.0, -(1.0), 1.0)) + ((vec4<f32>(_e151.g0_.w) * vec4<f32>(_e155.g0_.z, _e158.g0_.y, _e161.g0_.z, _e164.g0_.z)) * vec4<f32>(1.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e176.g1_.x) * vec4<f32>(_e180.g0_.x)) * vec4<f32>(0.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e192.g1_.y) * vec4<f32>(_e196.g0_.x)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((vec4<f32>(_e209.g1_.z) * vec4<f32>(_e213.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e226.g1_.w) * vec4<f32>(_e230.g0_.x)) * vec4<f32>(0.0, 0.0, 1.0, 0.0))) + ((_e242.g0_.zzxx * vec4<f32>(_e245.g0_.y, _e248.g0_.z, _e251.g0_.y, _e254.g0_.z)) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0))));
}

fn multi_vector_point_geometric_anti_product(self_568: MultiVector, other_496: Point) -> MultiVector {
    var self_569: MultiVector;
    var other_497: Point;

    self_569 = self_568;
    other_497 = other_496;
    let _e4: MultiVector = self_569;
    let _e8: Point = other_497;
    let _e11: Point = other_497;
    let _e14: Point = other_497;
    let _e17: Point = other_497;
    let _e29: MultiVector = self_569;
    let _e33: Point = other_497;
    let _e36: Point = other_497;
    let _e39: Point = other_497;
    let _e42: Point = other_497;
    let _e54: MultiVector = self_569;
    let _e58: Point = other_497;
    let _e70: MultiVector = self_569;
    let _e74: Point = other_497;
    let _e86: MultiVector = self_569;
    let _e90: Point = other_497;
    let _e102: MultiVector = self_569;
    let _e106: Point = other_497;
    let _e119: MultiVector = self_569;
    let _e122: Point = other_497;
    let _e125: Point = other_497;
    let _e128: Point = other_497;
    let _e131: Point = other_497;
    let _e144: MultiVector = self_569;
    let _e148: Point = other_497;
    let _e151: Point = other_497;
    let _e154: Point = other_497;
    let _e157: Point = other_497;
    let _e169: MultiVector = self_569;
    let _e173: Point = other_497;
    let _e176: Point = other_497;
    let _e179: Point = other_497;
    let _e182: Point = other_497;
    let _e194: MultiVector = self_569;
    let _e198: Point = other_497;
    let _e201: Point = other_497;
    let _e204: Point = other_497;
    let _e207: Point = other_497;
    let _e220: MultiVector = self_569;
    let _e224: Point = other_497;
    let _e227: Point = other_497;
    let _e230: Point = other_497;
    let _e233: Point = other_497;
    let _e245: MultiVector = self_569;
    let _e247: Point = other_497;
    return MultiVector(((((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y, _e11.g0_.y, _e14.g0_.y, _e17.g0_.z)) * vec4<f32>(0.0, 0.0, -(1.0), 1.0)) + ((vec4<f32>(_e29.g0_.w) * vec4<f32>(_e33.g0_.y, _e36.g0_.z, _e39.g0_.y, _e42.g0_.y)) * vec4<f32>(1.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e54.g1_.x) * vec4<f32>(_e58.g0_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e70.g1_.y) * vec4<f32>(_e74.g0_.x)) * vec4<f32>(0.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e86.g1_.z) * vec4<f32>(_e90.g0_.x)) * vec4<f32>(0.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e102.g1_.w) * vec4<f32>(_e106.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((_e119.g0_.zzxx * vec4<f32>(_e122.g0_.z, _e125.g0_.y, _e128.g0_.z, _e131.g0_.y)) * vec4<f32>(1.0, -(1.0), 1.0, 1.0))), ((((((vec4<f32>(_e144.g1_.x) * vec4<f32>(_e148.g0_.z, _e151.g0_.z, _e154.g0_.z, _e157.g0_.y)) * vec4<f32>(0.0, 0.0, -(1.0), 1.0)) + ((vec4<f32>(_e169.g1_.y) * vec4<f32>(_e173.g0_.y, _e176.g0_.y, _e179.g0_.y, _e182.g0_.z)) * vec4<f32>(0.0, 0.0, 1.0, 1.0))) + ((vec4<f32>(_e194.g1_.z) * vec4<f32>(_e198.g0_.z, _e201.g0_.y, _e204.g0_.z, _e207.g0_.z)) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))) + ((vec4<f32>(_e220.g1_.w) * vec4<f32>(_e224.g0_.y, _e227.g0_.z, _e230.g0_.y, _e233.g0_.y)) * vec4<f32>(1.0, 1.0, 0.0, 0.0))) + ((_e245.g0_ * vec4<f32>(_e247.g0_.x)) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))));
}

fn multi_vector_point_scalar_product(self_570: MultiVector, other_498: Point) -> Scalar {
    var self_571: MultiVector;
    var other_499: Point;

    self_571 = self_570;
    other_499 = other_498;
    let _e5: MultiVector = self_571;
    let _e8: Point = other_499;
    let _e13: MultiVector = self_571;
    let _e16: Point = other_499;
    let _e21: MultiVector = self_571;
    let _e24: Point = other_499;
    return Scalar((((0.0 - (_e5.g0_.y * _e8.g0_.x)) - (_e13.g1_.z * _e16.g0_.y)) - (_e21.g1_.w * _e24.g0_.z)));
}

fn multi_vector_point_anti_scalar_product(self_572: MultiVector, other_500: Point) -> AntiScalar {
    var self_573: MultiVector;
    var other_501: Point;

    self_573 = self_572;
    other_501 = other_500;
    let _e4: MultiVector = self_573;
    let _e7: Point = other_501;
    let _e11: MultiVector = self_573;
    let _e14: Point = other_501;
    let _e19: MultiVector = self_573;
    let _e22: Point = other_501;
    return AntiScalar((((_e4.g0_.y * _e7.g0_.x) + (_e11.g1_.z * _e14.g0_.y)) + (_e19.g1_.w * _e22.g0_.z)));
}

fn multi_vector_ideal_point_into(self_574: MultiVector) -> IdealPoint {
    var self_575: MultiVector;

    self_575 = self_574;
    let _e2: MultiVector = self_575;
    let _e5: MultiVector = self_575;
    return IdealPoint(vec2<f32>(_e2.g1_.z, _e5.g1_.w));
}

fn multi_vector_ideal_point_add(self_576: MultiVector, other_502: IdealPoint) -> MultiVector {
    var self_577: MultiVector;
    var other_503: IdealPoint;

    self_577 = self_576;
    other_503 = other_502;
    let _e4: MultiVector = self_577;
    let _e6: MultiVector = self_577;
    let _e8: IdealPoint = other_503;
    let _e11: IdealPoint = other_503;
    let _e14: IdealPoint = other_503;
    let _e17: IdealPoint = other_503;
    return MultiVector(_e4.g0_, (_e6.g1_ + (vec4<f32>(_e8.g0_.x, _e11.g0_.x, _e14.g0_.x, _e17.g0_.y) * vec4<f32>(0.0, 0.0, 1.0, 1.0))));
}

fn multi_vector_ideal_point_sub(self_578: MultiVector, other_504: IdealPoint) -> MultiVector {
    var self_579: MultiVector;
    var other_505: IdealPoint;

    self_579 = self_578;
    other_505 = other_504;
    let _e4: MultiVector = self_579;
    let _e6: MultiVector = self_579;
    let _e8: IdealPoint = other_505;
    let _e11: IdealPoint = other_505;
    let _e14: IdealPoint = other_505;
    let _e17: IdealPoint = other_505;
    return MultiVector(_e4.g0_, (_e6.g1_ - (vec4<f32>(_e8.g0_.x, _e11.g0_.x, _e14.g0_.x, _e17.g0_.y) * vec4<f32>(0.0, 0.0, 1.0, 1.0))));
}

fn multi_vector_ideal_point_geometric_product(self_580: MultiVector, other_506: IdealPoint) -> MultiVector {
    var self_581: MultiVector;
    var other_507: IdealPoint;

    self_581 = self_580;
    other_507 = other_506;
    let _e4: MultiVector = self_581;
    let _e8: IdealPoint = other_507;
    let _e11: IdealPoint = other_507;
    let _e14: IdealPoint = other_507;
    let _e17: IdealPoint = other_507;
    let _e30: MultiVector = self_581;
    let _e34: IdealPoint = other_507;
    let _e37: IdealPoint = other_507;
    let _e40: IdealPoint = other_507;
    let _e43: IdealPoint = other_507;
    let _e57: MultiVector = self_581;
    let _e60: IdealPoint = other_507;
    let _e63: IdealPoint = other_507;
    let _e66: IdealPoint = other_507;
    let _e69: IdealPoint = other_507;
    let _e83: MultiVector = self_581;
    let _e87: IdealPoint = other_507;
    let _e90: IdealPoint = other_507;
    let _e93: IdealPoint = other_507;
    let _e96: IdealPoint = other_507;
    let _e108: MultiVector = self_581;
    let _e112: IdealPoint = other_507;
    let _e115: IdealPoint = other_507;
    let _e118: IdealPoint = other_507;
    let _e121: IdealPoint = other_507;
    let _e133: MultiVector = self_581;
    let _e136: IdealPoint = other_507;
    let _e139: IdealPoint = other_507;
    let _e142: IdealPoint = other_507;
    let _e145: IdealPoint = other_507;
    return MultiVector(((((vec4<f32>(_e4.g1_.y) * vec4<f32>(_e8.g0_.y, _e11.g0_.y, _e14.g0_.y, _e17.g0_.x)) * vec4<f32>(0.0, 0.0, -(1.0), -(1.0))) + ((vec4<f32>(_e30.g1_.w) * vec4<f32>(_e34.g0_.y, _e37.g0_.x, _e40.g0_.y, _e43.g0_.y)) * vec4<f32>(-(1.0), -(1.0), 0.0, 0.0))) + ((_e57.g1_.zzxx * vec4<f32>(_e60.g0_.x, _e63.g0_.y, _e66.g0_.x, _e69.g0_.y)) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))), ((((vec4<f32>(_e83.g0_.y) * vec4<f32>(_e87.g0_.y, _e90.g0_.y, _e93.g0_.y, _e96.g0_.x)) * vec4<f32>(0.0, 0.0, -(1.0), 1.0)) + ((vec4<f32>(_e108.g0_.w) * vec4<f32>(_e112.g0_.y, _e115.g0_.x, _e118.g0_.y, _e121.g0_.y)) * vec4<f32>(1.0, 1.0, 0.0, 0.0))) + ((_e133.g0_.zzxx * vec4<f32>(_e136.g0_.x, _e139.g0_.y, _e142.g0_.x, _e145.g0_.y)) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0))));
}

fn multi_vector_ideal_point_geometric_anti_product(self_582: MultiVector, other_508: IdealPoint) -> MultiVector {
    var self_583: MultiVector;
    var other_509: IdealPoint;

    self_583 = self_582;
    other_509 = other_508;
    let _e4: MultiVector = self_583;
    let _e8: IdealPoint = other_509;
    let _e11: IdealPoint = other_509;
    let _e14: IdealPoint = other_509;
    let _e17: IdealPoint = other_509;
    let _e29: MultiVector = self_583;
    let _e33: IdealPoint = other_509;
    let _e36: IdealPoint = other_509;
    let _e39: IdealPoint = other_509;
    let _e42: IdealPoint = other_509;
    let _e54: MultiVector = self_583;
    let _e57: IdealPoint = other_509;
    let _e60: IdealPoint = other_509;
    let _e63: IdealPoint = other_509;
    let _e66: IdealPoint = other_509;
    let _e79: MultiVector = self_583;
    let _e83: IdealPoint = other_509;
    let _e86: IdealPoint = other_509;
    let _e89: IdealPoint = other_509;
    let _e92: IdealPoint = other_509;
    let _e103: MultiVector = self_583;
    let _e107: IdealPoint = other_509;
    let _e110: IdealPoint = other_509;
    let _e113: IdealPoint = other_509;
    let _e116: IdealPoint = other_509;
    let _e128: MultiVector = self_583;
    let _e131: IdealPoint = other_509;
    let _e134: IdealPoint = other_509;
    let _e137: IdealPoint = other_509;
    let _e140: IdealPoint = other_509;
    return MultiVector(((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.x, _e11.g0_.x, _e14.g0_.x, _e17.g0_.y)) * vec4<f32>(0.0, 0.0, -(1.0), 1.0)) + ((vec4<f32>(_e29.g0_.w) * vec4<f32>(_e33.g0_.x, _e36.g0_.y, _e39.g0_.x, _e42.g0_.x)) * vec4<f32>(1.0, 1.0, 0.0, 0.0))) + ((_e54.g0_.zzxx * vec4<f32>(_e57.g0_.y, _e60.g0_.x, _e63.g0_.y, _e66.g0_.x)) * vec4<f32>(1.0, -(1.0), 1.0, 1.0))), ((((vec4<f32>(_e79.g1_.y) * vec4<f32>(_e83.g0_.x, _e86.g0_.x, _e89.g0_.x, _e92.g0_.y)) * vec4<f32>(0.0, 0.0, 1.0, 1.0)) + ((vec4<f32>(_e103.g1_.w) * vec4<f32>(_e107.g0_.x, _e110.g0_.y, _e113.g0_.x, _e116.g0_.x)) * vec4<f32>(1.0, 1.0, 0.0, 0.0))) + ((_e128.g1_.zzxx * vec4<f32>(_e131.g0_.y, _e134.g0_.x, _e137.g0_.y, _e140.g0_.x)) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))));
}

fn multi_vector_ideal_point_scalar_product(self_584: MultiVector, other_510: IdealPoint) -> Scalar {
    var self_585: MultiVector;
    var other_511: IdealPoint;

    self_585 = self_584;
    other_511 = other_510;
    let _e5: MultiVector = self_585;
    let _e8: IdealPoint = other_511;
    let _e13: MultiVector = self_585;
    let _e16: IdealPoint = other_511;
    return Scalar(((0.0 - (_e5.g1_.z * _e8.g0_.x)) - (_e13.g1_.w * _e16.g0_.y)));
}

fn multi_vector_ideal_point_anti_scalar_product(self_586: MultiVector, other_512: IdealPoint) -> AntiScalar {
    var self_587: MultiVector;
    var other_513: IdealPoint;

    self_587 = self_586;
    other_513 = other_512;
    let _e4: MultiVector = self_587;
    let _e7: IdealPoint = other_513;
    let _e11: MultiVector = self_587;
    let _e14: IdealPoint = other_513;
    return AntiScalar(((_e4.g1_.z * _e7.g0_.x) + (_e11.g1_.w * _e14.g0_.y)));
}

fn multi_vector_plane_into(self_588: MultiVector) -> Plane {
    var self_589: MultiVector;

    self_589 = self_588;
    let _e2: MultiVector = self_589;
    let _e5: MultiVector = self_589;
    let _e8: MultiVector = self_589;
    return Plane(vec3<f32>(_e2.g1_.x, _e5.g0_.w, _e8.g0_.z));
}

fn multi_vector_plane_add(self_590: MultiVector, other_514: Plane) -> MultiVector {
    var self_591: MultiVector;
    var other_515: Plane;

    self_591 = self_590;
    other_515 = other_514;
    let _e4: MultiVector = self_591;
    let _e6: Plane = other_515;
    let _e9: Plane = other_515;
    let _e12: Plane = other_515;
    let _e15: Plane = other_515;
    let _e26: MultiVector = self_591;
    let _e28: Plane = other_515;
    return MultiVector((_e4.g0_ + (vec4<f32>(_e6.g0_.x, _e9.g0_.x, _e12.g0_.z, _e15.g0_.y) * vec4<f32>(0.0, 0.0, 1.0, 1.0))), (_e26.g1_ + (vec4<f32>(_e28.g0_.x) * vec4<f32>(1.0, 0.0, 0.0, 0.0))));
}

fn multi_vector_plane_sub(self_592: MultiVector, other_516: Plane) -> MultiVector {
    var self_593: MultiVector;
    var other_517: Plane;

    self_593 = self_592;
    other_517 = other_516;
    let _e4: MultiVector = self_593;
    let _e6: Plane = other_517;
    let _e9: Plane = other_517;
    let _e12: Plane = other_517;
    let _e15: Plane = other_517;
    let _e26: MultiVector = self_593;
    let _e28: Plane = other_517;
    return MultiVector((_e4.g0_ - (vec4<f32>(_e6.g0_.x, _e9.g0_.x, _e12.g0_.z, _e15.g0_.y) * vec4<f32>(0.0, 0.0, 1.0, 1.0))), (_e26.g1_ - (vec4<f32>(_e28.g0_.x) * vec4<f32>(1.0, 0.0, 0.0, 0.0))));
}

fn multi_vector_plane_geometric_product(self_594: MultiVector, other_518: Plane) -> MultiVector {
    var self_595: MultiVector;
    var other_519: Plane;

    self_595 = self_594;
    other_519 = other_518;
    let _e4: MultiVector = self_595;
    let _e8: Plane = other_519;
    let _e11: Plane = other_519;
    let _e14: Plane = other_519;
    let _e17: Plane = other_519;
    let _e29: MultiVector = self_595;
    let _e33: Plane = other_519;
    let _e36: Plane = other_519;
    let _e39: Plane = other_519;
    let _e42: Plane = other_519;
    let _e55: MultiVector = self_595;
    let _e59: Plane = other_519;
    let _e71: MultiVector = self_595;
    let _e75: Plane = other_519;
    let _e87: MultiVector = self_595;
    let _e91: Plane = other_519;
    let _e104: MultiVector = self_595;
    let _e108: Plane = other_519;
    let _e120: MultiVector = self_595;
    let _e123: Plane = other_519;
    let _e126: Plane = other_519;
    let _e129: Plane = other_519;
    let _e132: Plane = other_519;
    let _e138: MultiVector = self_595;
    let _e142: Plane = other_519;
    let _e145: Plane = other_519;
    let _e148: Plane = other_519;
    let _e151: Plane = other_519;
    let _e163: MultiVector = self_595;
    let _e167: Plane = other_519;
    let _e170: Plane = other_519;
    let _e173: Plane = other_519;
    let _e176: Plane = other_519;
    let _e188: MultiVector = self_595;
    let _e192: Plane = other_519;
    let _e195: Plane = other_519;
    let _e198: Plane = other_519;
    let _e201: Plane = other_519;
    let _e213: MultiVector = self_595;
    let _e217: Plane = other_519;
    let _e220: Plane = other_519;
    let _e223: Plane = other_519;
    let _e226: Plane = other_519;
    let _e239: MultiVector = self_595;
    let _e241: Plane = other_519;
    return MultiVector(((((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y, _e11.g0_.y, _e14.g0_.y, _e17.g0_.z)) * vec4<f32>(0.0, 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e29.g0_.w) * vec4<f32>(_e33.g0_.y, _e36.g0_.z, _e39.g0_.y, _e42.g0_.y)) * vec4<f32>(1.0, -(1.0), 0.0, 0.0))) + ((vec4<f32>(_e55.g1_.x) * vec4<f32>(_e59.g0_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e71.g1_.y) * vec4<f32>(_e75.g0_.x)) * vec4<f32>(0.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e87.g1_.z) * vec4<f32>(_e91.g0_.x)) * vec4<f32>(0.0, 0.0, -(1.0), 0.0))) + ((vec4<f32>(_e104.g1_.w) * vec4<f32>(_e108.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + (_e120.g0_.zzxx * vec4<f32>(_e123.g0_.z, _e126.g0_.y, _e129.g0_.z, _e132.g0_.y))), ((((((vec4<f32>(_e138.g1_.x) * vec4<f32>(_e142.g0_.z, _e145.g0_.z, _e148.g0_.z, _e151.g0_.y)) * vec4<f32>(0.0, 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e163.g1_.y) * vec4<f32>(_e167.g0_.y, _e170.g0_.y, _e173.g0_.y, _e176.g0_.z)) * vec4<f32>(0.0, 0.0, 1.0, 1.0))) + ((vec4<f32>(_e188.g1_.z) * vec4<f32>(_e192.g0_.z, _e195.g0_.y, _e198.g0_.z, _e201.g0_.z)) * vec4<f32>(1.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e213.g1_.w) * vec4<f32>(_e217.g0_.y, _e220.g0_.z, _e223.g0_.y, _e226.g0_.y)) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))) + ((_e239.g0_ * vec4<f32>(_e241.g0_.x)) * vec4<f32>(1.0, 1.0, -(1.0), 1.0))));
}

fn multi_vector_plane_geometric_anti_product(self_596: MultiVector, other_520: Plane) -> MultiVector {
    var self_597: MultiVector;
    var other_521: Plane;

    self_597 = self_596;
    other_521 = other_520;
    let _e4: MultiVector = self_597;
    let _e8: Plane = other_521;
    let _e11: Plane = other_521;
    let _e14: Plane = other_521;
    let _e17: Plane = other_521;
    let _e29: MultiVector = self_597;
    let _e33: Plane = other_521;
    let _e36: Plane = other_521;
    let _e39: Plane = other_521;
    let _e42: Plane = other_521;
    let _e54: MultiVector = self_597;
    let _e58: Plane = other_521;
    let _e61: Plane = other_521;
    let _e64: Plane = other_521;
    let _e67: Plane = other_521;
    let _e79: MultiVector = self_597;
    let _e83: Plane = other_521;
    let _e86: Plane = other_521;
    let _e89: Plane = other_521;
    let _e92: Plane = other_521;
    let _e105: MultiVector = self_597;
    let _e108: Plane = other_521;
    let _e122: MultiVector = self_597;
    let _e126: Plane = other_521;
    let _e129: Plane = other_521;
    let _e132: Plane = other_521;
    let _e135: Plane = other_521;
    let _e147: MultiVector = self_597;
    let _e151: Plane = other_521;
    let _e154: Plane = other_521;
    let _e157: Plane = other_521;
    let _e160: Plane = other_521;
    let _e173: MultiVector = self_597;
    let _e177: Plane = other_521;
    let _e190: MultiVector = self_597;
    let _e194: Plane = other_521;
    let _e206: MultiVector = self_597;
    let _e210: Plane = other_521;
    let _e223: MultiVector = self_597;
    let _e227: Plane = other_521;
    let _e239: MultiVector = self_597;
    let _e242: Plane = other_521;
    let _e245: Plane = other_521;
    let _e248: Plane = other_521;
    let _e251: Plane = other_521;
    return MultiVector(((((((vec4<f32>(_e4.g1_.x) * vec4<f32>(_e8.g0_.y, _e11.g0_.y, _e14.g0_.y, _e17.g0_.z)) * vec4<f32>(0.0, 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e29.g1_.y) * vec4<f32>(_e33.g0_.z, _e36.g0_.z, _e39.g0_.z, _e42.g0_.y)) * vec4<f32>(0.0, 0.0, 1.0, 1.0))) + ((vec4<f32>(_e54.g1_.z) * vec4<f32>(_e58.g0_.y, _e61.g0_.z, _e64.g0_.y, _e67.g0_.y)) * vec4<f32>(1.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e79.g1_.w) * vec4<f32>(_e83.g0_.z, _e86.g0_.y, _e89.g0_.z, _e92.g0_.z)) * vec4<f32>(1.0, -(1.0), 0.0, 0.0))) + ((_e105.g0_.yxwz * vec4<f32>(_e108.g0_.x)) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0))), ((((((((vec4<f32>(_e122.g0_.y) * vec4<f32>(_e126.g0_.z, _e129.g0_.z, _e132.g0_.z, _e135.g0_.y)) * vec4<f32>(0.0, 0.0, -(1.0), 1.0)) + ((vec4<f32>(_e147.g0_.w) * vec4<f32>(_e151.g0_.z, _e154.g0_.y, _e157.g0_.z, _e160.g0_.z)) * vec4<f32>(1.0, -(1.0), 0.0, 0.0))) + ((vec4<f32>(_e173.g1_.x) * vec4<f32>(_e177.g0_.x)) * vec4<f32>(0.0, -(1.0), 0.0, 0.0))) + ((vec4<f32>(_e190.g1_.y) * vec4<f32>(_e194.g0_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e206.g1_.z) * vec4<f32>(_e210.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e223.g1_.w) * vec4<f32>(_e227.g0_.x)) * vec4<f32>(0.0, 0.0, 1.0, 0.0))) + ((_e239.g0_.zzxx * vec4<f32>(_e242.g0_.y, _e245.g0_.z, _e248.g0_.y, _e251.g0_.z)) * vec4<f32>(-(1.0)))));
}

fn multi_vector_plane_scalar_product(self_598: MultiVector, other_522: Plane) -> Scalar {
    var self_599: MultiVector;
    var other_523: Plane;

    self_599 = self_598;
    other_523 = other_522;
    let _e4: MultiVector = self_599;
    let _e7: Plane = other_523;
    let _e11: MultiVector = self_599;
    let _e14: Plane = other_523;
    let _e19: MultiVector = self_599;
    let _e22: Plane = other_523;
    return Scalar((((_e4.g0_.z * _e7.g0_.z) + (_e11.g0_.w * _e14.g0_.y)) + (_e19.g1_.x * _e22.g0_.x)));
}

fn multi_vector_plane_anti_scalar_product(self_600: MultiVector, other_524: Plane) -> AntiScalar {
    var self_601: MultiVector;
    var other_525: Plane;

    self_601 = self_600;
    other_525 = other_524;
    let _e5: MultiVector = self_601;
    let _e8: Plane = other_525;
    let _e13: MultiVector = self_601;
    let _e16: Plane = other_525;
    let _e21: MultiVector = self_601;
    let _e24: Plane = other_525;
    return AntiScalar((((0.0 - (_e5.g0_.z * _e8.g0_.z)) - (_e13.g0_.w * _e16.g0_.y)) - (_e21.g1_.x * _e24.g0_.x)));
}

fn multi_vector_translator_into(self_602: MultiVector) -> Translator {
    var self_603: MultiVector;

    self_603 = self_602;
    let _e2: MultiVector = self_603;
    let _e5: MultiVector = self_603;
    let _e8: MultiVector = self_603;
    return Translator(vec3<f32>(_e2.g0_.x, _e5.g1_.z, _e8.g1_.w));
}

fn multi_vector_translator_add(self_604: MultiVector, other_526: Translator) -> MultiVector {
    var self_605: MultiVector;
    var other_527: Translator;

    self_605 = self_604;
    other_527 = other_526;
    let _e4: MultiVector = self_605;
    let _e6: Translator = other_527;
    let _e17: MultiVector = self_605;
    let _e19: Translator = other_527;
    let _e22: Translator = other_527;
    let _e25: Translator = other_527;
    let _e28: Translator = other_527;
    return MultiVector((_e4.g0_ + (vec4<f32>(_e6.g0_.x) * vec4<f32>(1.0, 0.0, 0.0, 0.0))), (_e17.g1_ + (vec4<f32>(_e19.g0_.x, _e22.g0_.x, _e25.g0_.y, _e28.g0_.z) * vec4<f32>(0.0, 0.0, 1.0, 1.0))));
}

fn multi_vector_translator_sub(self_606: MultiVector, other_528: Translator) -> MultiVector {
    var self_607: MultiVector;
    var other_529: Translator;

    self_607 = self_606;
    other_529 = other_528;
    let _e4: MultiVector = self_607;
    let _e6: Translator = other_529;
    let _e17: MultiVector = self_607;
    let _e19: Translator = other_529;
    let _e22: Translator = other_529;
    let _e25: Translator = other_529;
    let _e28: Translator = other_529;
    return MultiVector((_e4.g0_ - (vec4<f32>(_e6.g0_.x) * vec4<f32>(1.0, 0.0, 0.0, 0.0))), (_e17.g1_ - (vec4<f32>(_e19.g0_.x, _e22.g0_.x, _e25.g0_.y, _e28.g0_.z) * vec4<f32>(0.0, 0.0, 1.0, 1.0))));
}

fn multi_vector_translator_geometric_product(self_608: MultiVector, other_530: Translator) -> MultiVector {
    var self_609: MultiVector;
    var other_531: Translator;

    self_609 = self_608;
    other_531 = other_530;
    let _e4: MultiVector = self_609;
    let _e8: Translator = other_531;
    let _e11: Translator = other_531;
    let _e14: Translator = other_531;
    let _e17: Translator = other_531;
    let _e29: MultiVector = self_609;
    let _e33: Translator = other_531;
    let _e36: Translator = other_531;
    let _e39: Translator = other_531;
    let _e42: Translator = other_531;
    let _e56: MultiVector = self_609;
    let _e60: Translator = other_531;
    let _e63: Translator = other_531;
    let _e66: Translator = other_531;
    let _e69: Translator = other_531;
    let _e82: MultiVector = self_609;
    let _e86: Translator = other_531;
    let _e89: Translator = other_531;
    let _e92: Translator = other_531;
    let _e95: Translator = other_531;
    let _e109: MultiVector = self_609;
    let _e111: Translator = other_531;
    let _e117: MultiVector = self_609;
    let _e121: Translator = other_531;
    let _e124: Translator = other_531;
    let _e127: Translator = other_531;
    let _e130: Translator = other_531;
    let _e142: MultiVector = self_609;
    let _e146: Translator = other_531;
    let _e149: Translator = other_531;
    let _e152: Translator = other_531;
    let _e155: Translator = other_531;
    let _e167: MultiVector = self_609;
    let _e171: Translator = other_531;
    let _e183: MultiVector = self_609;
    let _e187: Translator = other_531;
    let _e199: MultiVector = self_609;
    let _e203: Translator = other_531;
    let _e215: MultiVector = self_609;
    let _e219: Translator = other_531;
    let _e231: MultiVector = self_609;
    let _e234: Translator = other_531;
    let _e237: Translator = other_531;
    let _e240: Translator = other_531;
    let _e243: Translator = other_531;
    return MultiVector(((((((vec4<f32>(_e4.g1_.x) * vec4<f32>(_e8.g0_.y, _e11.g0_.y, _e14.g0_.y, _e17.g0_.z)) * vec4<f32>(0.0, 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e29.g1_.y) * vec4<f32>(_e33.g0_.z, _e36.g0_.z, _e39.g0_.z, _e42.g0_.y)) * vec4<f32>(0.0, 0.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e56.g1_.z) * vec4<f32>(_e60.g0_.y, _e63.g0_.z, _e66.g0_.y, _e69.g0_.y)) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))) + ((vec4<f32>(_e82.g1_.w) * vec4<f32>(_e86.g0_.z, _e89.g0_.y, _e92.g0_.z, _e95.g0_.z)) * vec4<f32>(-(1.0), -(1.0), 0.0, 0.0))) + (_e109.g0_ * vec4<f32>(_e111.g0_.x))), ((((((((vec4<f32>(_e117.g0_.y) * vec4<f32>(_e121.g0_.z, _e124.g0_.z, _e127.g0_.z, _e130.g0_.y)) * vec4<f32>(0.0, 0.0, -(1.0), 1.0)) + ((vec4<f32>(_e142.g0_.w) * vec4<f32>(_e146.g0_.z, _e149.g0_.y, _e152.g0_.z, _e155.g0_.z)) * vec4<f32>(1.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e167.g1_.x) * vec4<f32>(_e171.g0_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e183.g1_.y) * vec4<f32>(_e187.g0_.x)) * vec4<f32>(0.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e199.g1_.z) * vec4<f32>(_e203.g0_.x)) * vec4<f32>(0.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e215.g1_.w) * vec4<f32>(_e219.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((_e231.g0_.zzxx * vec4<f32>(_e234.g0_.y, _e237.g0_.z, _e240.g0_.y, _e243.g0_.z)) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0))));
}

fn multi_vector_translator_outer_product(self_610: MultiVector, other_532: Translator) -> MultiVector {
    var self_611: MultiVector;
    var other_533: Translator;

    self_611 = self_610;
    other_533 = other_532;
    let _e4: MultiVector = self_611;
    let _e6: Translator = other_533;
    let _e11: MultiVector = self_611;
    let _e15: Translator = other_533;
    let _e26: MultiVector = self_611;
    let _e30: Translator = other_533;
    let _e42: MultiVector = self_611;
    let _e46: Translator = other_533;
    let _e58: MultiVector = self_611;
    let _e62: Translator = other_533;
    let _e74: MultiVector = self_611;
    let _e77: MultiVector = self_611;
    let _e80: MultiVector = self_611;
    let _e83: MultiVector = self_611;
    let _e87: Translator = other_533;
    let _e90: Translator = other_533;
    let _e93: Translator = other_533;
    let _e96: Translator = other_533;
    return MultiVector((_e4.g0_ * vec4<f32>(_e6.g0_.x)), ((((((vec4<f32>(_e11.g0_.w) * vec4<f32>(_e15.g0_.y)) * vec4<f32>(0.0, 1.0, 0.0, 0.0)) + ((vec4<f32>(_e26.g1_.y) * vec4<f32>(_e30.g0_.x)) * vec4<f32>(0.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e42.g1_.z) * vec4<f32>(_e46.g0_.x)) * vec4<f32>(0.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e58.g1_.w) * vec4<f32>(_e62.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + (vec4<f32>(_e74.g1_.x, _e77.g0_.z, _e80.g0_.x, _e83.g0_.x) * vec4<f32>(_e87.g0_.x, _e90.g0_.z, _e93.g0_.y, _e96.g0_.z))));
}

fn multi_vector_translator_inner_product(self_612: MultiVector, other_534: Translator) -> MultiVector {
    var self_613: MultiVector;
    var other_535: Translator;

    self_613 = self_612;
    other_535 = other_534;
    let _e4: MultiVector = self_613;
    let _e8: Translator = other_535;
    let _e11: Translator = other_535;
    let _e14: Translator = other_535;
    let _e17: Translator = other_535;
    let _e29: MultiVector = self_613;
    let _e33: Translator = other_535;
    let _e36: Translator = other_535;
    let _e39: Translator = other_535;
    let _e42: Translator = other_535;
    let _e56: MultiVector = self_613;
    let _e60: Translator = other_535;
    let _e73: MultiVector = self_613;
    let _e77: Translator = other_535;
    let _e90: MultiVector = self_613;
    let _e92: Translator = other_535;
    let _e98: MultiVector = self_613;
    let _e102: Translator = other_535;
    let _e113: MultiVector = self_613;
    let _e117: Translator = other_535;
    let _e129: MultiVector = self_613;
    let _e133: Translator = other_535;
    let _e145: MultiVector = self_613;
    let _e149: Translator = other_535;
    let _e161: MultiVector = self_613;
    let _e164: MultiVector = self_613;
    let _e167: MultiVector = self_613;
    let _e170: MultiVector = self_613;
    let _e174: Translator = other_535;
    let _e177: Translator = other_535;
    let _e180: Translator = other_535;
    let _e183: Translator = other_535;
    return MultiVector(((((((vec4<f32>(_e4.g1_.x) * vec4<f32>(_e8.g0_.y, _e11.g0_.y, _e14.g0_.y, _e17.g0_.z)) * vec4<f32>(0.0, 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e29.g1_.y) * vec4<f32>(_e33.g0_.z, _e36.g0_.z, _e39.g0_.z, _e42.g0_.y)) * vec4<f32>(0.0, 0.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e56.g1_.z) * vec4<f32>(_e60.g0_.y)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((vec4<f32>(_e73.g1_.w) * vec4<f32>(_e77.g0_.z)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + (_e90.g0_ * vec4<f32>(_e92.g0_.x))), ((((((vec4<f32>(_e98.g0_.w) * vec4<f32>(_e102.g0_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) + ((vec4<f32>(_e113.g1_.x) * vec4<f32>(_e117.g0_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e129.g1_.z) * vec4<f32>(_e133.g0_.x)) * vec4<f32>(0.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e145.g1_.w) * vec4<f32>(_e149.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e161.g0_.z, _e164.g1_.y, _e167.g0_.x, _e170.g0_.x) * vec4<f32>(_e174.g0_.y, _e177.g0_.x, _e180.g0_.y, _e183.g0_.z)) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0))));
}

fn multi_vector_translator_geometric_anti_product(self_614: MultiVector, other_536: Translator) -> MultiVector {
    var self_615: MultiVector;
    var other_537: Translator;

    self_615 = self_614;
    other_537 = other_536;
    let _e4: MultiVector = self_615;
    let _e8: Translator = other_537;
    let _e11: Translator = other_537;
    let _e14: Translator = other_537;
    let _e17: Translator = other_537;
    let _e29: MultiVector = self_615;
    let _e33: Translator = other_537;
    let _e36: Translator = other_537;
    let _e39: Translator = other_537;
    let _e42: Translator = other_537;
    let _e54: MultiVector = self_615;
    let _e58: Translator = other_537;
    let _e71: MultiVector = self_615;
    let _e75: Translator = other_537;
    let _e87: MultiVector = self_615;
    let _e91: Translator = other_537;
    let _e103: MultiVector = self_615;
    let _e107: Translator = other_537;
    let _e119: MultiVector = self_615;
    let _e122: Translator = other_537;
    let _e125: Translator = other_537;
    let _e128: Translator = other_537;
    let _e131: Translator = other_537;
    let _e144: MultiVector = self_615;
    let _e148: Translator = other_537;
    let _e151: Translator = other_537;
    let _e154: Translator = other_537;
    let _e157: Translator = other_537;
    let _e169: MultiVector = self_615;
    let _e173: Translator = other_537;
    let _e176: Translator = other_537;
    let _e179: Translator = other_537;
    let _e182: Translator = other_537;
    let _e194: MultiVector = self_615;
    let _e198: Translator = other_537;
    let _e201: Translator = other_537;
    let _e204: Translator = other_537;
    let _e207: Translator = other_537;
    let _e220: MultiVector = self_615;
    let _e224: Translator = other_537;
    let _e227: Translator = other_537;
    let _e230: Translator = other_537;
    let _e233: Translator = other_537;
    let _e245: MultiVector = self_615;
    let _e248: Translator = other_537;
    return MultiVector(((((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y, _e11.g0_.y, _e14.g0_.y, _e17.g0_.z)) * vec4<f32>(0.0, 0.0, -(1.0), 1.0)) + ((vec4<f32>(_e29.g0_.w) * vec4<f32>(_e33.g0_.y, _e36.g0_.z, _e39.g0_.y, _e42.g0_.y)) * vec4<f32>(1.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e54.g1_.x) * vec4<f32>(_e58.g0_.x)) * vec4<f32>(0.0, -(1.0), 0.0, 0.0))) + ((vec4<f32>(_e71.g1_.y) * vec4<f32>(_e75.g0_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e87.g1_.z) * vec4<f32>(_e91.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e103.g1_.w) * vec4<f32>(_e107.g0_.x)) * vec4<f32>(0.0, 0.0, 1.0, 0.0))) + ((_e119.g0_.zzxx * vec4<f32>(_e122.g0_.z, _e125.g0_.y, _e128.g0_.z, _e131.g0_.y)) * vec4<f32>(1.0, -(1.0), 1.0, 1.0))), ((((((vec4<f32>(_e144.g1_.x) * vec4<f32>(_e148.g0_.z, _e151.g0_.z, _e154.g0_.z, _e157.g0_.y)) * vec4<f32>(0.0, 0.0, -(1.0), 1.0)) + ((vec4<f32>(_e169.g1_.y) * vec4<f32>(_e173.g0_.y, _e176.g0_.y, _e179.g0_.y, _e182.g0_.z)) * vec4<f32>(0.0, 0.0, 1.0, 1.0))) + ((vec4<f32>(_e194.g1_.z) * vec4<f32>(_e198.g0_.z, _e201.g0_.y, _e204.g0_.z, _e207.g0_.z)) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))) + ((vec4<f32>(_e220.g1_.w) * vec4<f32>(_e224.g0_.y, _e227.g0_.z, _e230.g0_.y, _e233.g0_.y)) * vec4<f32>(1.0, 1.0, 0.0, 0.0))) + ((_e245.g0_.yxwz * vec4<f32>(_e248.g0_.x)) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0)))));
}

fn multi_vector_translator_inner_anti_product(self_616: MultiVector, other_538: Translator) -> MultiVector {
    var self_617: MultiVector;
    var other_539: Translator;

    self_617 = self_616;
    other_539 = other_538;
    let _e4: MultiVector = self_617;
    let _e8: Translator = other_539;
    let _e19: MultiVector = self_617;
    let _e23: Translator = other_539;
    let _e36: MultiVector = self_617;
    let _e40: Translator = other_539;
    let _e52: MultiVector = self_617;
    let _e56: Translator = other_539;
    let _e68: MultiVector = self_617;
    let _e71: MultiVector = self_617;
    let _e74: MultiVector = self_617;
    let _e77: MultiVector = self_617;
    let _e81: Translator = other_539;
    let _e84: Translator = other_539;
    let _e87: Translator = other_539;
    let _e90: Translator = other_539;
    let _e103: MultiVector = self_617;
    let _e107: Translator = other_539;
    let _e110: Translator = other_539;
    let _e113: Translator = other_539;
    let _e116: Translator = other_539;
    let _e128: MultiVector = self_617;
    let _e132: Translator = other_539;
    let _e135: Translator = other_539;
    let _e138: Translator = other_539;
    let _e141: Translator = other_539;
    let _e153: MultiVector = self_617;
    let _e157: Translator = other_539;
    let _e169: MultiVector = self_617;
    let _e173: Translator = other_539;
    let _e185: MultiVector = self_617;
    let _e188: Translator = other_539;
    return MultiVector(((((((vec4<f32>(_e4.g0_.w) * vec4<f32>(_e8.g0_.z)) * vec4<f32>(0.0, 1.0, 0.0, 0.0)) + ((vec4<f32>(_e19.g1_.x) * vec4<f32>(_e23.g0_.x)) * vec4<f32>(0.0, -(1.0), 0.0, 0.0))) + ((vec4<f32>(_e36.g1_.z) * vec4<f32>(_e40.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e52.g1_.w) * vec4<f32>(_e56.g0_.x)) * vec4<f32>(0.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e68.g1_.y, _e71.g0_.z, _e74.g0_.x, _e77.g0_.x) * vec4<f32>(_e81.g0_.x, _e84.g0_.y, _e87.g0_.z, _e90.g0_.y)) * vec4<f32>(1.0, -(1.0), 1.0, 1.0))), ((((((vec4<f32>(_e103.g1_.x) * vec4<f32>(_e107.g0_.z, _e110.g0_.z, _e113.g0_.z, _e116.g0_.y)) * vec4<f32>(0.0, 0.0, -(1.0), 1.0)) + ((vec4<f32>(_e128.g1_.y) * vec4<f32>(_e132.g0_.y, _e135.g0_.y, _e138.g0_.y, _e141.g0_.z)) * vec4<f32>(0.0, 0.0, 1.0, 1.0))) + ((vec4<f32>(_e153.g1_.z) * vec4<f32>(_e157.g0_.y)) * vec4<f32>(0.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e169.g1_.w) * vec4<f32>(_e173.g0_.z)) * vec4<f32>(0.0, 1.0, 0.0, 0.0))) + ((_e185.g0_.yxwz * vec4<f32>(_e188.g0_.x)) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0)))));
}

fn multi_vector_translator_right_contraction(self_618: MultiVector, other_540: Translator) -> MultiVector {
    var self_619: MultiVector;
    var other_541: Translator;

    self_619 = self_618;
    other_541 = other_540;
    let _e4: MultiVector = self_619;
    let _e8: Translator = other_541;
    let _e11: Translator = other_541;
    let _e14: Translator = other_541;
    let _e17: Translator = other_541;
    let _e30: MultiVector = self_619;
    let _e34: Translator = other_541;
    let _e47: MultiVector = self_619;
    let _e51: Translator = other_541;
    let _e64: MultiVector = self_619;
    let _e66: Translator = other_541;
    let _e72: MultiVector = self_619;
    let _e74: Translator = other_541;
    return MultiVector((((((vec4<f32>(_e4.g1_.y) * vec4<f32>(_e8.g0_.z, _e11.g0_.z, _e14.g0_.z, _e17.g0_.y)) * vec4<f32>(0.0, 0.0, -(1.0), -(1.0))) + ((vec4<f32>(_e30.g1_.z) * vec4<f32>(_e34.g0_.y)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((vec4<f32>(_e47.g1_.w) * vec4<f32>(_e51.g0_.z)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + (_e64.g0_ * vec4<f32>(_e66.g0_.x))), (_e72.g1_ * vec4<f32>(_e74.g0_.x)));
}

fn multi_vector_translator_left_anti_contraction(self_620: MultiVector, other_542: Translator) -> MultiVector {
    var self_621: MultiVector;
    var other_543: Translator;

    self_621 = self_620;
    other_543 = other_542;
    let _e4: MultiVector = self_621;
    let _e7: Translator = other_543;
    let _e19: MultiVector = self_621;
    let _e23: Translator = other_543;
    let _e26: Translator = other_543;
    let _e29: Translator = other_543;
    let _e32: Translator = other_543;
    let _e43: MultiVector = self_621;
    let _e47: Translator = other_543;
    let _e59: MultiVector = self_621;
    let _e63: Translator = other_543;
    let _e75: MultiVector = self_621;
    let _e78: Translator = other_543;
    return MultiVector(((_e4.g1_.yxwz * vec4<f32>(_e7.g0_.x)) * vec4<f32>(1.0, -(1.0), 1.0, 1.0)), (((((vec4<f32>(_e19.g1_.y) * vec4<f32>(_e23.g0_.y, _e26.g0_.y, _e29.g0_.y, _e32.g0_.z)) * vec4<f32>(0.0, 0.0, 1.0, 1.0)) + ((vec4<f32>(_e43.g1_.z) * vec4<f32>(_e47.g0_.y)) * vec4<f32>(0.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e59.g1_.w) * vec4<f32>(_e63.g0_.z)) * vec4<f32>(0.0, 1.0, 0.0, 0.0))) + ((_e75.g0_.yxwz * vec4<f32>(_e78.g0_.x)) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0)))));
}

fn multi_vector_translator_scalar_product(self_622: MultiVector, other_544: Translator) -> Scalar {
    var self_623: MultiVector;
    var other_545: Translator;

    self_623 = self_622;
    other_545 = other_544;
    let _e4: MultiVector = self_623;
    let _e7: Translator = other_545;
    let _e11: MultiVector = self_623;
    let _e14: Translator = other_545;
    let _e19: MultiVector = self_623;
    let _e22: Translator = other_545;
    return Scalar((((_e4.g0_.x * _e7.g0_.x) - (_e11.g1_.z * _e14.g0_.y)) - (_e19.g1_.w * _e22.g0_.z)));
}

fn multi_vector_translator_anti_scalar_product(self_624: MultiVector, other_546: Translator) -> AntiScalar {
    var self_625: MultiVector;
    var other_547: Translator;

    self_625 = self_624;
    other_547 = other_546;
    let _e5: MultiVector = self_625;
    let _e8: Translator = other_547;
    let _e13: MultiVector = self_625;
    let _e16: Translator = other_547;
    let _e21: MultiVector = self_625;
    let _e24: Translator = other_547;
    return AntiScalar((((0.0 - (_e5.g0_.x * _e8.g0_.x)) + (_e13.g1_.z * _e16.g0_.y)) + (_e21.g1_.w * _e24.g0_.z)));
}

fn multi_vector_motor_into(self_626: MultiVector) -> Motor {
    var self_627: MultiVector;

    self_627 = self_626;
    let _e2: MultiVector = self_627;
    let _e5: MultiVector = self_627;
    let _e8: MultiVector = self_627;
    let _e11: MultiVector = self_627;
    return Motor(vec4<f32>(_e2.g0_.x, _e5.g0_.y, _e8.g1_.z, _e11.g1_.w));
}

fn multi_vector_motor_add(self_628: MultiVector, other_548: Motor) -> MultiVector {
    var self_629: MultiVector;
    var other_549: Motor;

    self_629 = self_628;
    other_549 = other_548;
    let _e4: MultiVector = self_629;
    let _e6: Motor = other_549;
    let _e16: MultiVector = self_629;
    let _e18: Motor = other_549;
    return MultiVector((_e4.g0_ + (_e6.g0_.xyxx * vec4<f32>(1.0, 1.0, 0.0, 0.0))), (_e16.g1_ + (_e18.g0_.xxzw * vec4<f32>(0.0, 0.0, 1.0, 1.0))));
}

fn multi_vector_motor_sub(self_630: MultiVector, other_550: Motor) -> MultiVector {
    var self_631: MultiVector;
    var other_551: Motor;

    self_631 = self_630;
    other_551 = other_550;
    let _e4: MultiVector = self_631;
    let _e6: Motor = other_551;
    let _e16: MultiVector = self_631;
    let _e18: Motor = other_551;
    return MultiVector((_e4.g0_ - (_e6.g0_.xyxx * vec4<f32>(1.0, 1.0, 0.0, 0.0))), (_e16.g1_ - (_e18.g0_.xxzw * vec4<f32>(0.0, 0.0, 1.0, 1.0))));
}

fn multi_vector_motor_geometric_product(self_632: MultiVector, other_552: Motor) -> MultiVector {
    var self_633: MultiVector;
    var other_553: Motor;

    self_633 = self_632;
    other_553 = other_552;
    let _e4: MultiVector = self_633;
    let _e8: Motor = other_553;
    let _e19: MultiVector = self_633;
    let _e23: Motor = other_553;
    let _e35: MultiVector = self_633;
    let _e39: Motor = other_553;
    let _e51: MultiVector = self_633;
    let _e55: Motor = other_553;
    let _e68: MultiVector = self_633;
    let _e72: Motor = other_553;
    let _e84: MultiVector = self_633;
    let _e88: Motor = other_553;
    let _e101: MultiVector = self_633;
    let _e104: Motor = other_553;
    let _e109: MultiVector = self_633;
    let _e113: Motor = other_553;
    let _e124: MultiVector = self_633;
    let _e128: Motor = other_553;
    let _e139: MultiVector = self_633;
    let _e143: Motor = other_553;
    let _e154: MultiVector = self_633;
    let _e158: Motor = other_553;
    let _e170: MultiVector = self_633;
    let _e174: Motor = other_553;
    let _e186: MultiVector = self_633;
    let _e190: Motor = other_553;
    let _e201: MultiVector = self_633;
    let _e204: Motor = other_553;
    return MultiVector(((((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0)) + ((vec4<f32>(_e19.g0_.w) * _e23.g0_.yyyx) * vec4<f32>(0.0, 0.0, -(1.0), 1.0))) + ((vec4<f32>(_e35.g1_.x) * _e39.g0_.zzzw) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e51.g1_.y) * _e55.g0_.wwwz) * vec4<f32>(0.0, 0.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e68.g1_.z) * _e72.g0_.zwzz) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))) + ((vec4<f32>(_e84.g1_.w) * _e88.g0_.wzww) * vec4<f32>(-(1.0), -(1.0), 0.0, 0.0))) + (_e101.g0_.xxzz * _e104.g0_.xyxy)), ((((((((vec4<f32>(_e109.g0_.y) * _e113.g0_.wwwz) * vec4<f32>(0.0, 0.0, -(1.0), 1.0)) + ((vec4<f32>(_e124.g0_.w) * _e128.g0_.wzww) * vec4<f32>(1.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e139.g1_.x) * _e143.g0_.xyxx) * vec4<f32>(1.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e154.g1_.y) * _e158.g0_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))) + ((vec4<f32>(_e170.g1_.z) * _e174.g0_.xxxy) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e186.g1_.w) * _e190.g0_.yyyx) * vec4<f32>(0.0, 0.0, 1.0, 1.0))) + ((_e201.g0_.zzxx * _e204.g0_.zwzw) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0))));
}

fn multi_vector_motor_outer_product(self_634: MultiVector, other_554: Motor) -> MultiVector {
    var self_635: MultiVector;
    var other_555: Motor;

    self_635 = self_634;
    other_555 = other_554;
    let _e4: MultiVector = self_635;
    let _e8: Motor = other_555;
    let _e19: MultiVector = self_635;
    let _e22: Motor = other_555;
    let _e27: MultiVector = self_635;
    let _e31: Motor = other_555;
    let _e42: MultiVector = self_635;
    let _e46: Motor = other_555;
    let _e57: MultiVector = self_635;
    let _e61: Motor = other_555;
    let _e73: MultiVector = self_635;
    let _e77: Motor = other_555;
    let _e89: MultiVector = self_635;
    let _e93: Motor = other_555;
    let _e105: MultiVector = self_635;
    let _e108: Motor = other_555;
    return MultiVector((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.x)) * vec4<f32>(0.0, 1.0, 0.0, 0.0)) + (_e19.g0_.xxzw * _e22.g0_.xyxx)), (((((((vec4<f32>(_e27.g0_.w) * vec4<f32>(_e31.g0_.z)) * vec4<f32>(0.0, 1.0, 0.0, 0.0)) + ((vec4<f32>(_e42.g1_.x) * _e46.g0_.xyxx) * vec4<f32>(1.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e57.g1_.y) * vec4<f32>(_e61.g0_.x)) * vec4<f32>(0.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e73.g1_.z) * vec4<f32>(_e77.g0_.x)) * vec4<f32>(0.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e89.g1_.w) * vec4<f32>(_e93.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((_e105.g0_.xzxx * _e108.g0_.xwzw) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn multi_vector_motor_inner_product(self_636: MultiVector, other_556: Motor) -> MultiVector {
    var self_637: MultiVector;
    var other_557: Motor;

    self_637 = self_636;
    other_557 = other_556;
    let _e4: MultiVector = self_637;
    let _e8: Motor = other_557;
    let _e19: MultiVector = self_637;
    let _e23: Motor = other_557;
    let _e35: MultiVector = self_637;
    let _e39: Motor = other_557;
    let _e51: MultiVector = self_637;
    let _e55: Motor = other_557;
    let _e68: MultiVector = self_637;
    let _e72: Motor = other_557;
    let _e85: MultiVector = self_637;
    let _e89: Motor = other_557;
    let _e102: MultiVector = self_637;
    let _e105: Motor = other_557;
    let _e110: MultiVector = self_637;
    let _e114: Motor = other_557;
    let _e125: MultiVector = self_637;
    let _e129: Motor = other_557;
    let _e141: MultiVector = self_637;
    let _e145: Motor = other_557;
    let _e157: MultiVector = self_637;
    let _e161: Motor = other_557;
    let _e173: MultiVector = self_637;
    let _e177: Motor = other_557;
    let _e189: MultiVector = self_637;
    let _e192: Motor = other_557;
    return MultiVector(((((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0)) + ((vec4<f32>(_e19.g0_.w) * _e23.g0_.yyyx) * vec4<f32>(0.0, 0.0, -(1.0), 1.0))) + ((vec4<f32>(_e35.g1_.x) * _e39.g0_.zzzw) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e51.g1_.y) * _e55.g0_.wwwz) * vec4<f32>(0.0, 0.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e68.g1_.z) * vec4<f32>(_e72.g0_.z)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((vec4<f32>(_e85.g1_.w) * vec4<f32>(_e89.g0_.w)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + (_e102.g0_.xxzz * _e105.g0_.xyxy)), (((((((vec4<f32>(_e110.g0_.w) * vec4<f32>(_e114.g0_.w)) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) + ((vec4<f32>(_e125.g1_.x) * vec4<f32>(_e129.g0_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e141.g1_.y) * _e145.g0_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))) + ((vec4<f32>(_e157.g1_.z) * vec4<f32>(_e161.g0_.x)) * vec4<f32>(0.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e173.g1_.w) * vec4<f32>(_e177.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((_e189.g0_.zxxx * _e192.g0_.zxzw) * vec4<f32>(-(1.0), 0.0, 1.0, 1.0))));
}

fn multi_vector_motor_geometric_anti_product(self_638: MultiVector, other_558: Motor) -> MultiVector {
    var self_639: MultiVector;
    var other_559: Motor;

    self_639 = self_638;
    other_559 = other_558;
    let _e4: MultiVector = self_639;
    let _e8: Motor = other_559;
    let _e19: MultiVector = self_639;
    let _e23: Motor = other_559;
    let _e34: MultiVector = self_639;
    let _e38: Motor = other_559;
    let _e50: MultiVector = self_639;
    let _e54: Motor = other_559;
    let _e65: MultiVector = self_639;
    let _e69: Motor = other_559;
    let _e80: MultiVector = self_639;
    let _e84: Motor = other_559;
    let _e96: MultiVector = self_639;
    let _e99: Motor = other_559;
    let _e111: MultiVector = self_639;
    let _e115: Motor = other_559;
    let _e125: MultiVector = self_639;
    let _e129: Motor = other_559;
    let _e142: MultiVector = self_639;
    let _e146: Motor = other_559;
    let _e158: MultiVector = self_639;
    let _e162: Motor = other_559;
    let _e173: MultiVector = self_639;
    let _e177: Motor = other_559;
    let _e189: MultiVector = self_639;
    let _e193: Motor = other_559;
    let _e204: MultiVector = self_639;
    let _e207: Motor = other_559;
    return MultiVector(((((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.zzzw) * vec4<f32>(0.0, 0.0, -(1.0), 1.0)) + ((vec4<f32>(_e19.g0_.w) * _e23.g0_.zwzz) * vec4<f32>(1.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e34.g1_.x) * _e38.g0_.yxyy) * vec4<f32>(1.0, -(1.0), 0.0, 0.0))) + ((vec4<f32>(_e50.g1_.y) * _e54.g0_.xyxx) * vec4<f32>(1.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e65.g1_.z) * _e69.g0_.yyyx) * vec4<f32>(0.0, 0.0, 1.0, 1.0))) + ((vec4<f32>(_e80.g1_.w) * _e84.g0_.xxxy) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((_e96.g0_.zzxx * _e99.g0_.wzwz) * vec4<f32>(1.0, -(1.0), 1.0, 1.0))), ((((((((vec4<f32>(_e111.g0_.y) * _e115.g0_.xyxx) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) + ((vec4<f32>(_e125.g0_.w) * _e129.g0_.xxxy) * vec4<f32>(0.0, 0.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e142.g1_.x) * _e146.g0_.wwwz) * vec4<f32>(0.0, 0.0, -(1.0), 1.0))) + ((vec4<f32>(_e158.g1_.y) * _e162.g0_.zzzw) * vec4<f32>(0.0, 0.0, 1.0, 1.0))) + ((vec4<f32>(_e173.g1_.z) * _e177.g0_.wzww) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))) + ((vec4<f32>(_e189.g1_.w) * _e193.g0_.zwzz) * vec4<f32>(1.0, 1.0, 0.0, 0.0))) + ((_e204.g0_.xxzz * _e207.g0_.yxyx) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))));
}

fn multi_vector_motor_inner_anti_product(self_640: MultiVector, other_560: Motor) -> MultiVector {
    var self_641: MultiVector;
    var other_561: Motor;

    self_641 = self_640;
    other_561 = other_560;
    let _e4: MultiVector = self_641;
    let _e8: Motor = other_561;
    let _e19: MultiVector = self_641;
    let _e23: Motor = other_561;
    let _e36: MultiVector = self_641;
    let _e40: Motor = other_561;
    let _e51: MultiVector = self_641;
    let _e55: Motor = other_561;
    let _e67: MultiVector = self_641;
    let _e71: Motor = other_561;
    let _e83: MultiVector = self_641;
    let _e86: Motor = other_561;
    let _e98: MultiVector = self_641;
    let _e102: Motor = other_561;
    let _e112: MultiVector = self_641;
    let _e116: Motor = other_561;
    let _e129: MultiVector = self_641;
    let _e133: Motor = other_561;
    let _e145: MultiVector = self_641;
    let _e149: Motor = other_561;
    let _e160: MultiVector = self_641;
    let _e164: Motor = other_561;
    let _e176: MultiVector = self_641;
    let _e180: Motor = other_561;
    let _e192: MultiVector = self_641;
    let _e195: Motor = other_561;
    return MultiVector((((((((vec4<f32>(_e4.g0_.w) * vec4<f32>(_e8.g0_.w)) * vec4<f32>(0.0, 1.0, 0.0, 0.0)) + ((vec4<f32>(_e19.g1_.x) * vec4<f32>(_e23.g0_.x)) * vec4<f32>(0.0, -(1.0), 0.0, 0.0))) + ((vec4<f32>(_e36.g1_.y) * _e40.g0_.xyxx) * vec4<f32>(1.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e51.g1_.z) * vec4<f32>(_e55.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e67.g1_.w) * vec4<f32>(_e71.g0_.x)) * vec4<f32>(0.0, 0.0, 1.0, 0.0))) + ((_e83.g0_.xzxx * _e86.g0_.xzwz) * vec4<f32>(0.0, -(1.0), 1.0, 1.0))), ((((((((vec4<f32>(_e98.g0_.y) * _e102.g0_.xyxx) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) + ((vec4<f32>(_e112.g0_.w) * _e116.g0_.xxxy) * vec4<f32>(0.0, 0.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e129.g1_.x) * _e133.g0_.wwwz) * vec4<f32>(0.0, 0.0, -(1.0), 1.0))) + ((vec4<f32>(_e145.g1_.y) * _e149.g0_.zzzw) * vec4<f32>(0.0, 0.0, 1.0, 1.0))) + ((vec4<f32>(_e160.g1_.z) * vec4<f32>(_e164.g0_.z)) * vec4<f32>(0.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e176.g1_.w) * vec4<f32>(_e180.g0_.w)) * vec4<f32>(0.0, 1.0, 0.0, 0.0))) + ((_e192.g0_.xxzz * _e195.g0_.yxyx) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))));
}

fn multi_vector_motor_right_contraction(self_642: MultiVector, other_562: Motor) -> MultiVector {
    var self_643: MultiVector;
    var other_563: Motor;

    self_643 = self_642;
    other_563 = other_562;
    let _e4: MultiVector = self_643;
    let _e8: Motor = other_563;
    let _e19: MultiVector = self_643;
    let _e23: Motor = other_563;
    let _e36: MultiVector = self_643;
    let _e40: Motor = other_563;
    let _e53: MultiVector = self_643;
    let _e57: Motor = other_563;
    let _e70: MultiVector = self_643;
    let _e73: Motor = other_563;
    let _e85: MultiVector = self_643;
    let _e89: Motor = other_563;
    let _e100: MultiVector = self_643;
    let _e103: Motor = other_563;
    return MultiVector(((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0)) + ((vec4<f32>(_e19.g1_.y) * _e23.g0_.wwwz) * vec4<f32>(0.0, 0.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e36.g1_.z) * vec4<f32>(_e40.g0_.z)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((vec4<f32>(_e53.g1_.w) * vec4<f32>(_e57.g0_.w)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((_e70.g0_.xxzw * vec4<f32>(_e73.g0_.x)) * vec4<f32>(1.0, 0.0, 1.0, 1.0))), (((vec4<f32>(_e85.g1_.y) * _e89.g0_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0)) + ((_e100.g1_.xxzw * vec4<f32>(_e103.g0_.x)) * vec4<f32>(1.0, 0.0, 1.0, 1.0))));
}

fn multi_vector_motor_left_anti_contraction(self_644: MultiVector, other_564: Motor) -> MultiVector {
    var self_645: MultiVector;
    var other_565: Motor;

    self_645 = self_644;
    other_565 = other_564;
    let _e4: MultiVector = self_645;
    let _e8: Motor = other_565;
    let _e18: MultiVector = self_645;
    let _e21: Motor = other_565;
    let _e34: MultiVector = self_645;
    let _e38: Motor = other_565;
    let _e48: MultiVector = self_645;
    let _e52: Motor = other_565;
    let _e63: MultiVector = self_645;
    let _e67: Motor = other_565;
    let _e79: MultiVector = self_645;
    let _e83: Motor = other_565;
    let _e95: MultiVector = self_645;
    let _e98: Motor = other_565;
    return MultiVector((((vec4<f32>(_e4.g1_.y) * _e8.g0_.xyxx) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) + ((_e18.g1_.xxwz * vec4<f32>(_e21.g0_.x)) * vec4<f32>(0.0, -(1.0), 1.0, 1.0))), ((((((vec4<f32>(_e34.g0_.y) * _e38.g0_.xyxx) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) + ((vec4<f32>(_e48.g1_.y) * _e52.g0_.zzzw) * vec4<f32>(0.0, 0.0, 1.0, 1.0))) + ((vec4<f32>(_e63.g1_.z) * vec4<f32>(_e67.g0_.z)) * vec4<f32>(0.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e79.g1_.w) * vec4<f32>(_e83.g0_.w)) * vec4<f32>(0.0, 1.0, 0.0, 0.0))) + ((_e95.g0_.xxwz * vec4<f32>(_e98.g0_.x)) * vec4<f32>(0.0, -(1.0), -(1.0), -(1.0)))));
}

fn multi_vector_motor_scalar_product(self_646: MultiVector, other_566: Motor) -> Scalar {
    var self_647: MultiVector;
    var other_567: Motor;

    self_647 = self_646;
    other_567 = other_566;
    let _e4: MultiVector = self_647;
    let _e7: Motor = other_567;
    let _e11: MultiVector = self_647;
    let _e14: Motor = other_567;
    let _e19: MultiVector = self_647;
    let _e22: Motor = other_567;
    let _e27: MultiVector = self_647;
    let _e30: Motor = other_567;
    return Scalar(((((_e4.g0_.x * _e7.g0_.x) - (_e11.g0_.y * _e14.g0_.y)) - (_e19.g1_.z * _e22.g0_.z)) - (_e27.g1_.w * _e30.g0_.w)));
}

fn multi_vector_motor_anti_scalar_product(self_648: MultiVector, other_568: Motor) -> AntiScalar {
    var self_649: MultiVector;
    var other_569: Motor;

    self_649 = self_648;
    other_569 = other_568;
    let _e5: MultiVector = self_649;
    let _e8: Motor = other_569;
    let _e13: MultiVector = self_649;
    let _e16: Motor = other_569;
    let _e21: MultiVector = self_649;
    let _e24: Motor = other_569;
    let _e29: MultiVector = self_649;
    let _e32: Motor = other_569;
    return AntiScalar(((((0.0 - (_e5.g0_.x * _e8.g0_.x)) + (_e13.g0_.y * _e16.g0_.y)) + (_e21.g1_.z * _e24.g0_.z)) + (_e29.g1_.w * _e32.g0_.w)));
}

fn multi_vector_motor_dual_into(self_650: MultiVector) -> MotorDual {
    var self_651: MultiVector;

    self_651 = self_650;
    let _e2: MultiVector = self_651;
    let _e5: MultiVector = self_651;
    let _e8: MultiVector = self_651;
    let _e11: MultiVector = self_651;
    return MotorDual(vec4<f32>(_e2.g1_.y, _e5.g1_.x, _e8.g0_.w, _e11.g0_.z));
}

fn multi_vector_motor_dual_add(self_652: MultiVector, other_570: MotorDual) -> MultiVector {
    var self_653: MultiVector;
    var other_571: MotorDual;

    self_653 = self_652;
    other_571 = other_570;
    let _e4: MultiVector = self_653;
    let _e6: MotorDual = other_571;
    let _e16: MultiVector = self_653;
    let _e18: MotorDual = other_571;
    return MultiVector((_e4.g0_ + (_e6.g0_.xxwz * vec4<f32>(0.0, 0.0, 1.0, 1.0))), (_e16.g1_ + (_e18.g0_.yxxx * vec4<f32>(1.0, 1.0, 0.0, 0.0))));
}

fn multi_vector_motor_dual_sub(self_654: MultiVector, other_572: MotorDual) -> MultiVector {
    var self_655: MultiVector;
    var other_573: MotorDual;

    self_655 = self_654;
    other_573 = other_572;
    let _e4: MultiVector = self_655;
    let _e6: MotorDual = other_573;
    let _e16: MultiVector = self_655;
    let _e18: MotorDual = other_573;
    return MultiVector((_e4.g0_ - (_e6.g0_.xxwz * vec4<f32>(0.0, 0.0, 1.0, 1.0))), (_e16.g1_ - (_e18.g0_.yxxx * vec4<f32>(1.0, 1.0, 0.0, 0.0))));
}

fn multi_vector_motor_dual_geometric_product(self_656: MultiVector, other_574: MotorDual) -> MultiVector {
    var self_657: MultiVector;
    var other_575: MotorDual;

    self_657 = self_656;
    other_575 = other_574;
    let _e4: MultiVector = self_657;
    let _e8: MotorDual = other_575;
    let _e19: MultiVector = self_657;
    let _e23: MotorDual = other_575;
    let _e35: MultiVector = self_657;
    let _e39: MotorDual = other_575;
    let _e50: MultiVector = self_657;
    let _e54: MotorDual = other_575;
    let _e66: MultiVector = self_657;
    let _e70: MotorDual = other_575;
    let _e83: MultiVector = self_657;
    let _e87: MotorDual = other_575;
    let _e99: MultiVector = self_657;
    let _e102: MotorDual = other_575;
    let _e107: MultiVector = self_657;
    let _e111: MotorDual = other_575;
    let _e122: MultiVector = self_657;
    let _e126: MotorDual = other_575;
    let _e137: MultiVector = self_657;
    let _e141: MotorDual = other_575;
    let _e153: MultiVector = self_657;
    let _e157: MotorDual = other_575;
    let _e168: MultiVector = self_657;
    let _e172: MotorDual = other_575;
    let _e183: MultiVector = self_657;
    let _e187: MotorDual = other_575;
    let _e199: MultiVector = self_657;
    let _e202: MotorDual = other_575;
    return MultiVector(((((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.zzzw) * vec4<f32>(0.0, 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e19.g0_.w) * _e23.g0_.zwzz) * vec4<f32>(1.0, -(1.0), 0.0, 0.0))) + ((vec4<f32>(_e35.g1_.x) * _e39.g0_.yxyy) * vec4<f32>(1.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e50.g1_.y) * _e54.g0_.xyxx) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))) + ((vec4<f32>(_e66.g1_.z) * _e70.g0_.yyyx) * vec4<f32>(0.0, 0.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e83.g1_.w) * _e87.g0_.xxxy) * vec4<f32>(0.0, 0.0, -(1.0), 1.0))) + (_e99.g0_.zzxx * _e102.g0_.wzwz)), ((((((((vec4<f32>(_e107.g0_.y) * _e111.g0_.xyxx) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0)) + ((vec4<f32>(_e122.g0_.w) * _e126.g0_.xxxy) * vec4<f32>(0.0, 0.0, 1.0, 1.0))) + ((vec4<f32>(_e137.g1_.x) * _e141.g0_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e153.g1_.y) * _e157.g0_.zzzw) * vec4<f32>(0.0, 0.0, 1.0, 1.0))) + ((vec4<f32>(_e168.g1_.z) * _e172.g0_.wzww) * vec4<f32>(1.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e183.g1_.w) * _e187.g0_.zwzz) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))) + ((_e199.g0_.xxzz * _e202.g0_.yxyx) * vec4<f32>(1.0, 1.0, -(1.0), 1.0))));
}

fn multi_vector_motor_dual_regressive_product(self_658: MultiVector, other_576: MotorDual) -> MultiVector {
    var self_659: MultiVector;
    var other_577: MotorDual;

    self_659 = self_658;
    other_577 = other_576;
    let _e4: MultiVector = self_659;
    let _e8: MotorDual = other_577;
    let _e18: MultiVector = self_659;
    let _e22: MotorDual = other_577;
    let _e33: MultiVector = self_659;
    let _e37: MotorDual = other_577;
    let _e49: MultiVector = self_659;
    let _e53: MotorDual = other_577;
    let _e65: MultiVector = self_659;
    let _e68: MotorDual = other_577;
    let _e80: MultiVector = self_659;
    let _e84: MotorDual = other_577;
    let _e94: MultiVector = self_659;
    let _e97: MotorDual = other_577;
    return MultiVector(((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.yxyy) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) + ((vec4<f32>(_e18.g1_.y) * _e22.g0_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, 1.0))) + ((vec4<f32>(_e33.g1_.z) * vec4<f32>(_e37.g0_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e49.g1_.w) * vec4<f32>(_e53.g0_.w)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((_e65.g0_.xxzw * vec4<f32>(_e68.g0_.x)) * vec4<f32>(1.0, 0.0, 1.0, 1.0))), (((vec4<f32>(_e80.g1_.y) * _e84.g0_.yxyy) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) + ((_e94.g1_.xxzw * vec4<f32>(_e97.g0_.x)) * vec4<f32>(1.0, 0.0, 1.0, 1.0))));
}

fn multi_vector_motor_dual_inner_product(self_660: MultiVector, other_578: MotorDual) -> MultiVector {
    var self_661: MultiVector;
    var other_579: MotorDual;

    self_661 = self_660;
    other_579 = other_578;
    let _e4: MultiVector = self_661;
    let _e8: MotorDual = other_579;
    let _e19: MultiVector = self_661;
    let _e23: MotorDual = other_579;
    let _e35: MultiVector = self_661;
    let _e39: MotorDual = other_579;
    let _e50: MultiVector = self_661;
    let _e54: MotorDual = other_579;
    let _e66: MultiVector = self_661;
    let _e70: MotorDual = other_579;
    let _e83: MultiVector = self_661;
    let _e87: MotorDual = other_579;
    let _e99: MultiVector = self_661;
    let _e102: MotorDual = other_579;
    let _e113: MultiVector = self_661;
    let _e117: MotorDual = other_579;
    let _e129: MultiVector = self_661;
    let _e133: MotorDual = other_579;
    let _e144: MultiVector = self_661;
    let _e148: MotorDual = other_579;
    let _e160: MultiVector = self_661;
    let _e164: MotorDual = other_579;
    let _e177: MultiVector = self_661;
    let _e180: MotorDual = other_579;
    return MultiVector(((((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.zzzw) * vec4<f32>(0.0, 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e19.g0_.w) * vec4<f32>(_e23.g0_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e35.g1_.x) * _e39.g0_.yxyy) * vec4<f32>(1.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e50.g1_.y) * _e54.g0_.xyxx) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))) + ((vec4<f32>(_e66.g1_.z) * _e70.g0_.yyyx) * vec4<f32>(0.0, 0.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e83.g1_.w) * _e87.g0_.xxxy) * vec4<f32>(0.0, 0.0, -(1.0), 1.0))) + ((_e99.g0_.zxxx * _e102.g0_.wxwz) * vec4<f32>(1.0, 0.0, 1.0, 1.0))), ((((((vec4<f32>(_e113.g0_.y) * vec4<f32>(_e117.g0_.x)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0)) + ((vec4<f32>(_e129.g1_.y) * _e133.g0_.zzzw) * vec4<f32>(0.0, 0.0, 1.0, 1.0))) + ((vec4<f32>(_e144.g1_.z) * vec4<f32>(_e148.g0_.w)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e160.g1_.w) * vec4<f32>(_e164.g0_.z)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + (_e177.g0_.xxwz * _e180.g0_.yxxx)));
}

fn multi_vector_motor_dual_geometric_anti_product(self_662: MultiVector, other_580: MotorDual) -> MultiVector {
    var self_663: MultiVector;
    var other_581: MotorDual;

    self_663 = self_662;
    other_581 = other_580;
    let _e4: MultiVector = self_663;
    let _e8: MotorDual = other_581;
    let _e18: MultiVector = self_663;
    let _e22: MotorDual = other_581;
    let _e34: MultiVector = self_663;
    let _e38: MotorDual = other_581;
    let _e50: MultiVector = self_663;
    let _e54: MotorDual = other_581;
    let _e65: MultiVector = self_663;
    let _e69: MotorDual = other_581;
    let _e80: MultiVector = self_663;
    let _e84: MotorDual = other_581;
    let _e96: MultiVector = self_663;
    let _e99: MotorDual = other_581;
    let _e111: MultiVector = self_663;
    let _e115: MotorDual = other_581;
    let _e126: MultiVector = self_663;
    let _e130: MotorDual = other_581;
    let _e142: MultiVector = self_663;
    let _e146: MotorDual = other_581;
    let _e158: MultiVector = self_663;
    let _e162: MotorDual = other_581;
    let _e173: MultiVector = self_663;
    let _e177: MotorDual = other_581;
    let _e189: MultiVector = self_663;
    let _e193: MotorDual = other_581;
    let _e204: MultiVector = self_663;
    let _e207: MotorDual = other_581;
    return MultiVector(((((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.yxyy) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) + ((vec4<f32>(_e18.g0_.w) * _e22.g0_.yyyx) * vec4<f32>(0.0, 0.0, -(1.0), 1.0))) + ((vec4<f32>(_e34.g1_.x) * _e38.g0_.zzzw) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e50.g1_.y) * _e54.g0_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, 1.0))) + ((vec4<f32>(_e65.g1_.z) * _e69.g0_.zwzz) * vec4<f32>(1.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e80.g1_.w) * _e84.g0_.wzww) * vec4<f32>(1.0, -(1.0), 0.0, 0.0))) + ((_e96.g0_.xxzz * _e99.g0_.xyxy) * vec4<f32>(1.0, -(1.0), 1.0, 1.0))), ((((((((vec4<f32>(_e111.g0_.y) * _e115.g0_.wwwz) * vec4<f32>(0.0, 0.0, -(1.0), 1.0)) + ((vec4<f32>(_e126.g0_.w) * _e130.g0_.wzww) * vec4<f32>(1.0, -(1.0), 0.0, 0.0))) + ((vec4<f32>(_e142.g1_.x) * _e146.g0_.xyxx) * vec4<f32>(1.0, -(1.0), 0.0, 0.0))) + ((vec4<f32>(_e158.g1_.y) * _e162.g0_.yxyy) * vec4<f32>(1.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e173.g1_.z) * _e177.g0_.xxxy) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e189.g1_.w) * _e193.g0_.yyyx) * vec4<f32>(0.0, 0.0, 1.0, 1.0))) + ((_e204.g0_.zzxx * _e207.g0_.zwzw) * vec4<f32>(-(1.0)))));
}

fn multi_vector_motor_dual_inner_anti_product(self_664: MultiVector, other_582: MotorDual) -> MultiVector {
    var self_665: MultiVector;
    var other_583: MotorDual;

    self_665 = self_664;
    other_583 = other_582;
    let _e4: MultiVector = self_665;
    let _e8: MotorDual = other_583;
    let _e19: MultiVector = self_665;
    let _e23: MotorDual = other_583;
    let _e34: MultiVector = self_665;
    let _e38: MotorDual = other_583;
    let _e50: MultiVector = self_665;
    let _e54: MotorDual = other_583;
    let _e67: MultiVector = self_665;
    let _e70: MotorDual = other_583;
    let _e82: MultiVector = self_665;
    let _e86: MotorDual = other_583;
    let _e97: MultiVector = self_665;
    let _e101: MotorDual = other_583;
    let _e114: MultiVector = self_665;
    let _e118: MotorDual = other_583;
    let _e130: MultiVector = self_665;
    let _e134: MotorDual = other_583;
    let _e145: MultiVector = self_665;
    let _e149: MotorDual = other_583;
    let _e161: MultiVector = self_665;
    let _e165: MotorDual = other_583;
    let _e176: MultiVector = self_665;
    let _e179: MotorDual = other_583;
    return MultiVector(((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.x)) * vec4<f32>(0.0, 1.0, 0.0, 0.0)) + ((vec4<f32>(_e19.g1_.y) * _e23.g0_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, 1.0))) + ((vec4<f32>(_e34.g1_.z) * vec4<f32>(_e38.g0_.w)) * vec4<f32>(0.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e50.g1_.w) * vec4<f32>(_e54.g0_.z)) * vec4<f32>(0.0, -(1.0), 0.0, 0.0))) + ((_e67.g0_.xxzw * _e70.g0_.xyxx) * vec4<f32>(1.0, -(1.0), 1.0, 1.0))), ((((((((vec4<f32>(_e82.g0_.y) * _e86.g0_.wwwz) * vec4<f32>(0.0, 0.0, -(1.0), 1.0)) + ((vec4<f32>(_e97.g0_.w) * vec4<f32>(_e101.g0_.z)) * vec4<f32>(0.0, -(1.0), 0.0, 0.0))) + ((vec4<f32>(_e114.g1_.x) * _e118.g0_.xyxx) * vec4<f32>(1.0, -(1.0), 0.0, 0.0))) + ((vec4<f32>(_e130.g1_.y) * _e134.g0_.yxyy) * vec4<f32>(1.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e145.g1_.z) * _e149.g0_.xxxy) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e161.g1_.w) * _e165.g0_.yyyx) * vec4<f32>(0.0, 0.0, 1.0, 1.0))) + ((_e176.g0_.xzxx * _e179.g0_.xwzw) * vec4<f32>(0.0, -(1.0), -(1.0), -(1.0)))));
}

fn multi_vector_motor_dual_left_contraction(self_666: MultiVector, other_584: MotorDual) -> MultiVector {
    var self_667: MultiVector;
    var other_585: MotorDual;

    self_667 = self_666;
    other_585 = other_584;
    let _e4: MultiVector = self_667;
    let _e8: MotorDual = other_585;
    let _e19: MultiVector = self_667;
    let _e23: MotorDual = other_585;
    let _e34: MultiVector = self_667;
    let _e38: MotorDual = other_585;
    let _e51: MultiVector = self_667;
    let _e55: MotorDual = other_585;
    let _e68: MultiVector = self_667;
    let _e72: MotorDual = other_585;
    let _e85: MultiVector = self_667;
    let _e88: MotorDual = other_585;
    let _e99: MultiVector = self_667;
    let _e103: MotorDual = other_585;
    let _e115: MultiVector = self_667;
    let _e118: MotorDual = other_585;
    return MultiVector((((((((vec4<f32>(_e4.g0_.w) * vec4<f32>(_e8.g0_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) + ((vec4<f32>(_e19.g1_.x) * _e23.g0_.yxyy) * vec4<f32>(1.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e34.g1_.y) * vec4<f32>(_e38.g0_.x)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((vec4<f32>(_e51.g1_.z) * vec4<f32>(_e55.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e68.g1_.w) * vec4<f32>(_e72.g0_.x)) * vec4<f32>(0.0, 0.0, -(1.0), 0.0))) + ((_e85.g0_.zxxx * _e88.g0_.wxwz) * vec4<f32>(1.0, 0.0, 1.0, 1.0))), (((vec4<f32>(_e99.g0_.y) * vec4<f32>(_e103.g0_.x)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0)) + (_e115.g0_.xxwz * _e118.g0_.yxxx)));
}

fn multi_vector_motor_dual_right_anti_contraction(self_668: MultiVector, other_586: MotorDual) -> MultiVector {
    var self_669: MultiVector;
    var other_587: MotorDual;

    self_669 = self_668;
    other_587 = other_586;
    let _e4: MultiVector = self_669;
    let _e8: MotorDual = other_587;
    let _e19: MultiVector = self_669;
    let _e22: MotorDual = other_587;
    let _e34: MultiVector = self_669;
    let _e38: MotorDual = other_587;
    let _e50: MultiVector = self_669;
    let _e54: MotorDual = other_587;
    let _e66: MultiVector = self_669;
    let _e70: MotorDual = other_587;
    let _e82: MultiVector = self_669;
    let _e86: MotorDual = other_587;
    let _e98: MultiVector = self_669;
    let _e102: MotorDual = other_587;
    let _e114: MultiVector = self_669;
    let _e117: MotorDual = other_587;
    return MultiVector((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.x)) * vec4<f32>(0.0, 1.0, 0.0, 0.0)) + ((_e19.g0_.xxzw * _e22.g0_.xyxx) * vec4<f32>(1.0, -(1.0), 1.0, 1.0))), (((((((vec4<f32>(_e34.g0_.w) * vec4<f32>(_e38.g0_.z)) * vec4<f32>(0.0, -(1.0), 0.0, 0.0)) + ((vec4<f32>(_e50.g1_.x) * _e54.g0_.xyxx) * vec4<f32>(1.0, -(1.0), 0.0, 0.0))) + ((vec4<f32>(_e66.g1_.y) * vec4<f32>(_e70.g0_.x)) * vec4<f32>(0.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e82.g1_.z) * vec4<f32>(_e86.g0_.x)) * vec4<f32>(0.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e98.g1_.w) * vec4<f32>(_e102.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((_e114.g0_.xzxx * _e117.g0_.xwzw) * vec4<f32>(0.0, -(1.0), -(1.0), -(1.0)))));
}

fn multi_vector_motor_dual_scalar_product(self_670: MultiVector, other_588: MotorDual) -> Scalar {
    var self_671: MultiVector;
    var other_589: MotorDual;

    self_671 = self_670;
    other_589 = other_588;
    let _e4: MultiVector = self_671;
    let _e7: MotorDual = other_589;
    let _e11: MultiVector = self_671;
    let _e14: MotorDual = other_589;
    let _e19: MultiVector = self_671;
    let _e22: MotorDual = other_589;
    let _e27: MultiVector = self_671;
    let _e30: MotorDual = other_589;
    return Scalar(((((_e4.g0_.z * _e7.g0_.w) + (_e11.g0_.w * _e14.g0_.z)) + (_e19.g1_.x * _e22.g0_.y)) - (_e27.g1_.y * _e30.g0_.x)));
}

fn multi_vector_motor_dual_anti_scalar_product(self_672: MultiVector, other_590: MotorDual) -> AntiScalar {
    var self_673: MultiVector;
    var other_591: MotorDual;

    self_673 = self_672;
    other_591 = other_590;
    let _e5: MultiVector = self_673;
    let _e8: MotorDual = other_591;
    let _e13: MultiVector = self_673;
    let _e16: MotorDual = other_591;
    let _e21: MultiVector = self_673;
    let _e24: MotorDual = other_591;
    let _e29: MultiVector = self_673;
    let _e32: MotorDual = other_591;
    return AntiScalar(((((0.0 - (_e5.g0_.z * _e8.g0_.w)) - (_e13.g0_.w * _e16.g0_.z)) - (_e21.g1_.x * _e24.g0_.y)) + (_e29.g1_.y * _e32.g0_.x)));
}

fn multi_vector_squared_magnitude(self_674: MultiVector) -> Scalar {
    var self_675: MultiVector;

    self_675 = self_674;
    let _e2: MultiVector = self_675;
    let _e3: MultiVector = self_675;
    let _e4: MultiVector = multi_vector_reversal(_e3);
    let _e5: Scalar = multi_vector_multi_vector_scalar_product(_e2, _e4);
    return _e5;
}

fn multi_vector_magnitude(self_676: MultiVector) -> Scalar {
    var self_677: MultiVector;

    self_677 = self_676;
    let _e2: MultiVector = self_677;
    let _e3: Scalar = multi_vector_squared_magnitude(_e2);
    return Scalar(sqrt(_e3.g0_));
}

fn multi_vector_bulk_norm(self_678: MultiVector) -> Scalar {
    var self_679: MultiVector;

    self_679 = self_678;
    let _e2: MultiVector = self_679;
    let _e3: Scalar = multi_vector_squared_magnitude(_e2);
    return Scalar(sqrt(_e3.g0_));
}

fn multi_vector_squared_anti_magnitude(self_680: MultiVector) -> AntiScalar {
    var self_681: MultiVector;

    self_681 = self_680;
    let _e2: MultiVector = self_681;
    let _e3: MultiVector = self_681;
    let _e4: MultiVector = multi_vector_anti_reversal(_e3);
    let _e5: AntiScalar = multi_vector_multi_vector_anti_scalar_product(_e2, _e4);
    return _e5;
}

fn multi_vector_weight_norm(self_682: MultiVector) -> AntiScalar {
    var self_683: MultiVector;

    self_683 = self_682;
    let _e2: MultiVector = self_683;
    let _e3: AntiScalar = multi_vector_squared_anti_magnitude(_e2);
    return AntiScalar(sqrt(_e3.g0_));
}

fn multi_vector_scale(self_684: MultiVector, other_592: f32) -> MultiVector {
    var self_685: MultiVector;
    var other_593: f32;

    self_685 = self_684;
    other_593 = other_592;
    let _e4: MultiVector = self_685;
    let _e5: f32 = other_593;
    let _e7: MultiVector = multi_vector_scalar_geometric_product(_e4, Scalar(_e5));
    return _e7;
}

fn multi_vector_signum(self_686: MultiVector) -> MultiVector {
    var self_687: MultiVector;

    self_687 = self_686;
    let _e2: MultiVector = self_687;
    let _e3: MultiVector = self_687;
    let _e4: Scalar = multi_vector_magnitude(_e3);
    let _e9: MultiVector = multi_vector_scalar_geometric_product(_e2, Scalar((1.0 / _e4.g0_)));
    return _e9;
}

fn multi_vector_inverse(self_688: MultiVector) -> MultiVector {
    var self_689: MultiVector;

    self_689 = self_688;
    let _e2: MultiVector = self_689;
    let _e3: MultiVector = multi_vector_reversal(_e2);
    let _e4: MultiVector = self_689;
    let _e5: Scalar = multi_vector_squared_magnitude(_e4);
    let _e10: MultiVector = multi_vector_scalar_geometric_product(_e3, Scalar((1.0 / _e5.g0_)));
    return _e10;
}

fn rotor_zero() -> Rotor {
    return Rotor(vec2<f32>(0.0));
}

fn rotor_one() -> Rotor {
    return Rotor(vec2<f32>(1.0, 0.0));
}

fn rotor_neg(self_690: Rotor) -> Rotor {
    var self_691: Rotor;

    self_691 = self_690;
    let _e2: Rotor = self_691;
    return Rotor((_e2.g0_ * vec2<f32>(-(1.0))));
}

fn rotor_automorphism(self_692: Rotor) -> Rotor {
    var self_693: Rotor;

    self_693 = self_692;
    let _e2: Rotor = self_693;
    return Rotor(_e2.g0_);
}

fn rotor_reversal(self_694: Rotor) -> Rotor {
    var self_695: Rotor;

    self_695 = self_694;
    let _e2: Rotor = self_695;
    return Rotor((_e2.g0_ * vec2<f32>(1.0, -(1.0))));
}

fn rotor_conjugation(self_696: Rotor) -> Rotor {
    var self_697: Rotor;

    self_697 = self_696;
    let _e2: Rotor = self_697;
    return Rotor((_e2.g0_ * vec2<f32>(1.0, -(1.0))));
}

fn rotor_anti_reversal(self_698: Rotor) -> Rotor {
    var self_699: Rotor;

    self_699 = self_698;
    let _e2: Rotor = self_699;
    return Rotor((_e2.g0_ * vec2<f32>(1.0, -(1.0))));
}

fn rotor_scalar_into(self_700: Rotor) -> Scalar {
    var self_701: Rotor;

    self_701 = self_700;
    let _e2: Rotor = self_701;
    return Scalar(_e2.g0_.x);
}

fn rotor_scalar_add(self_702: Rotor, other_594: Scalar) -> Rotor {
    var self_703: Rotor;
    var other_595: Scalar;

    self_703 = self_702;
    other_595 = other_594;
    let _e4: Rotor = self_703;
    let _e6: Scalar = other_595;
    return Rotor((_e4.g0_ + (vec2<f32>(_e6.g0_) * vec2<f32>(1.0, 0.0))));
}

fn rotor_scalar_sub(self_704: Rotor, other_596: Scalar) -> Rotor {
    var self_705: Rotor;
    var other_597: Scalar;

    self_705 = self_704;
    other_597 = other_596;
    let _e4: Rotor = self_705;
    let _e6: Scalar = other_597;
    return Rotor((_e4.g0_ - (vec2<f32>(_e6.g0_) * vec2<f32>(1.0, 0.0))));
}

fn rotor_scalar_geometric_product(self_706: Rotor, other_598: Scalar) -> Rotor {
    var self_707: Rotor;
    var other_599: Scalar;

    self_707 = self_706;
    other_599 = other_598;
    let _e4: Rotor = self_707;
    let _e6: Scalar = other_599;
    return Rotor((_e4.g0_ * vec2<f32>(_e6.g0_)));
}

fn rotor_scalar_outer_product(self_708: Rotor, other_600: Scalar) -> Rotor {
    var self_709: Rotor;
    var other_601: Scalar;

    self_709 = self_708;
    other_601 = other_600;
    let _e4: Rotor = self_709;
    let _e6: Scalar = other_601;
    return Rotor((_e4.g0_ * vec2<f32>(_e6.g0_)));
}

fn rotor_scalar_inner_product(self_710: Rotor, other_602: Scalar) -> Rotor {
    var self_711: Rotor;
    var other_603: Scalar;

    self_711 = self_710;
    other_603 = other_602;
    let _e4: Rotor = self_711;
    let _e6: Scalar = other_603;
    return Rotor((_e4.g0_ * vec2<f32>(_e6.g0_)));
}

fn rotor_scalar_left_contraction(self_712: Rotor, other_604: Scalar) -> Scalar {
    var self_713: Rotor;
    var other_605: Scalar;

    self_713 = self_712;
    other_605 = other_604;
    let _e4: Rotor = self_713;
    let _e7: Scalar = other_605;
    return Scalar((_e4.g0_.x * _e7.g0_));
}

fn rotor_scalar_right_contraction(self_714: Rotor, other_606: Scalar) -> Rotor {
    var self_715: Rotor;
    var other_607: Scalar;

    self_715 = self_714;
    other_607 = other_606;
    let _e4: Rotor = self_715;
    let _e6: Scalar = other_607;
    return Rotor((_e4.g0_ * vec2<f32>(_e6.g0_)));
}

fn rotor_scalar_right_anti_contraction(self_716: Rotor, other_608: Scalar) -> AntiScalar {
    var self_717: Rotor;
    var other_609: Scalar;

    self_717 = self_716;
    other_609 = other_608;
    let _e5: Rotor = self_717;
    let _e8: Scalar = other_609;
    return AntiScalar((0.0 - (_e5.g0_.x * _e8.g0_)));
}

fn rotor_scalar_scalar_product(self_718: Rotor, other_610: Scalar) -> Scalar {
    var self_719: Rotor;
    var other_611: Scalar;

    self_719 = self_718;
    other_611 = other_610;
    let _e4: Rotor = self_719;
    let _e7: Scalar = other_611;
    return Scalar((_e4.g0_.x * _e7.g0_));
}

fn rotor_scalar_anti_scalar_product(self_720: Rotor, other_612: Scalar) -> AntiScalar {
    var self_721: Rotor;
    var other_613: Scalar;

    self_721 = self_720;
    other_613 = other_612;
    let _e5: Rotor = self_721;
    let _e8: Scalar = other_613;
    return AntiScalar((0.0 - (_e5.g0_.x * _e8.g0_)));
}

fn rotor_anti_scalar_regressive_product(self_722: Rotor, other_614: AntiScalar) -> Rotor {
    var self_723: Rotor;
    var other_615: AntiScalar;

    self_723 = self_722;
    other_615 = other_614;
    let _e4: Rotor = self_723;
    let _e6: AntiScalar = other_615;
    return Rotor((_e4.g0_ * vec2<f32>(_e6.g0_)));
}

fn rotor_anti_scalar_outer_product(self_724: Rotor, other_616: AntiScalar) -> AntiScalar {
    var self_725: Rotor;
    var other_617: AntiScalar;

    self_725 = self_724;
    other_617 = other_616;
    let _e4: Rotor = self_725;
    let _e7: AntiScalar = other_617;
    return AntiScalar((_e4.g0_.x * _e7.g0_));
}

fn rotor_anti_scalar_geometric_anti_product(self_726: Rotor, other_618: AntiScalar) -> Rotor {
    var self_727: Rotor;
    var other_619: AntiScalar;

    self_727 = self_726;
    other_619 = other_618;
    let _e4: Rotor = self_727;
    let _e6: AntiScalar = other_619;
    return Rotor((_e4.g0_ * vec2<f32>(_e6.g0_)));
}

fn rotor_anti_scalar_inner_anti_product(self_728: Rotor, other_620: AntiScalar) -> Rotor {
    var self_729: Rotor;
    var other_621: AntiScalar;

    self_729 = self_728;
    other_621 = other_620;
    let _e4: Rotor = self_729;
    let _e6: AntiScalar = other_621;
    return Rotor((_e4.g0_ * vec2<f32>(_e6.g0_)));
}

fn rotor_anti_scalar_right_anti_contraction(self_730: Rotor, other_622: AntiScalar) -> Rotor {
    var self_731: Rotor;
    var other_623: AntiScalar;

    self_731 = self_730;
    other_623 = other_622;
    let _e4: Rotor = self_731;
    let _e6: AntiScalar = other_623;
    return Rotor((_e4.g0_ * vec2<f32>(_e6.g0_)));
}

fn rotor_multi_vector_add(self_732: Rotor, other_624: MultiVector) -> MultiVector {
    var self_733: Rotor;
    var other_625: MultiVector;

    self_733 = self_732;
    other_625 = other_624;
    let _e4: Rotor = self_733;
    let _e7: Rotor = self_733;
    let _e10: Rotor = self_733;
    let _e13: Rotor = self_733;
    let _e23: MultiVector = other_625;
    let _e26: MultiVector = other_625;
    return MultiVector(((vec4<f32>(_e4.g0_.x, _e7.g0_.y, _e10.g0_.x, _e13.g0_.x) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) + _e23.g0_), _e26.g1_);
}

fn rotor_multi_vector_sub(self_734: Rotor, other_626: MultiVector) -> MultiVector {
    var self_735: Rotor;
    var other_627: MultiVector;

    self_735 = self_734;
    other_627 = other_626;
    let _e4: Rotor = self_735;
    let _e7: Rotor = self_735;
    let _e10: Rotor = self_735;
    let _e13: Rotor = self_735;
    let _e23: MultiVector = other_627;
    let _e28: MultiVector = other_627;
    return MultiVector(((vec4<f32>(_e4.g0_.x, _e7.g0_.y, _e10.g0_.x, _e13.g0_.x) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) - _e23.g0_), (vec4<f32>(0.0) - _e28.g1_));
}

fn rotor_multi_vector_geometric_product(self_736: Rotor, other_628: MultiVector) -> MultiVector {
    var self_737: Rotor;
    var other_629: MultiVector;

    self_737 = self_736;
    other_629 = other_628;
    let _e4: Rotor = self_737;
    let _e8: MultiVector = other_629;
    let _e11: Rotor = self_737;
    let _e15: MultiVector = other_629;
    let _e28: Rotor = self_737;
    let _e32: MultiVector = other_629;
    let _e35: Rotor = self_737;
    let _e39: MultiVector = other_629;
    return MultiVector(((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.y) * _e15.g0_.yxwz) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))), ((vec4<f32>(_e28.g0_.x) * _e32.g1_) + ((vec4<f32>(_e35.g0_.y) * _e39.g1_.yxwz) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))));
}

fn rotor_multi_vector_outer_product(self_738: Rotor, other_630: MultiVector) -> MultiVector {
    var self_739: Rotor;
    var other_631: MultiVector;

    self_739 = self_738;
    other_631 = other_630;
    let _e4: Rotor = self_739;
    let _e8: MultiVector = other_631;
    let _e11: Rotor = self_739;
    let _e14: Rotor = self_739;
    let _e17: Rotor = self_739;
    let _e20: Rotor = self_739;
    let _e24: MultiVector = other_631;
    let _e36: Rotor = self_739;
    let _e40: MultiVector = other_631;
    let _e43: Rotor = self_739;
    let _e46: Rotor = self_739;
    let _e49: Rotor = self_739;
    let _e52: Rotor = self_739;
    let _e56: MultiVector = other_631;
    return MultiVector(((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.x, _e14.g0_.y, _e17.g0_.x, _e20.g0_.x) * vec4<f32>(_e24.g0_.x)) * vec4<f32>(0.0, 1.0, 0.0, 0.0))), ((vec4<f32>(_e36.g0_.x) * _e40.g1_) + ((vec4<f32>(_e43.g0_.x, _e46.g0_.y, _e49.g0_.x, _e52.g0_.x) * vec4<f32>(_e56.g1_.x)) * vec4<f32>(0.0, 1.0, 0.0, 0.0))));
}

fn rotor_multi_vector_inner_product(self_740: Rotor, other_632: MultiVector) -> MultiVector {
    var self_741: Rotor;
    var other_633: MultiVector;

    self_741 = self_740;
    other_633 = other_632;
    let _e4: Rotor = self_741;
    let _e8: MultiVector = other_633;
    let _e11: Rotor = self_741;
    let _e15: MultiVector = other_633;
    let _e28: Rotor = self_741;
    let _e32: MultiVector = other_633;
    let _e35: Rotor = self_741;
    let _e38: Rotor = self_741;
    let _e41: Rotor = self_741;
    let _e44: Rotor = self_741;
    let _e48: MultiVector = other_633;
    return MultiVector(((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.y) * _e15.g0_.yxwz) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))), ((vec4<f32>(_e28.g0_.x) * _e32.g1_) + ((vec4<f32>(_e35.g0_.y, _e38.g0_.x, _e41.g0_.x, _e44.g0_.x) * _e48.g1_.yxxx) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))));
}

fn rotor_multi_vector_geometric_anti_product(self_742: Rotor, other_634: MultiVector) -> MultiVector {
    var self_743: Rotor;
    var other_635: MultiVector;

    self_743 = self_742;
    other_635 = other_634;
    let _e4: Rotor = self_743;
    let _e8: MultiVector = other_635;
    let _e19: Rotor = self_743;
    let _e23: MultiVector = other_635;
    let _e34: Rotor = self_743;
    let _e38: MultiVector = other_635;
    let _e51: Rotor = self_743;
    let _e55: MultiVector = other_635;
    return MultiVector((((vec4<f32>(_e4.g0_.x) * _e8.g1_.yxwz) * vec4<f32>(1.0, -(1.0), 1.0, 1.0)) + ((vec4<f32>(_e19.g0_.y) * _e23.g1_) * vec4<f32>(1.0, 1.0, -(1.0), 1.0))), (((vec4<f32>(_e34.g0_.x) * _e38.g0_.yxwz) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))) + ((vec4<f32>(_e51.g0_.y) * _e55.g0_) * vec4<f32>(1.0, 1.0, -(1.0), 1.0))));
}

fn rotor_multi_vector_inner_anti_product(self_744: Rotor, other_636: MultiVector) -> MultiVector {
    var self_745: Rotor;
    var other_637: MultiVector;

    self_745 = self_744;
    other_637 = other_636;
    let _e4: Rotor = self_745;
    let _e8: MultiVector = other_637;
    let _e19: Rotor = self_745;
    let _e22: Rotor = self_745;
    let _e25: Rotor = self_745;
    let _e28: Rotor = self_745;
    let _e32: MultiVector = other_637;
    let _e43: Rotor = self_745;
    let _e47: MultiVector = other_637;
    let _e60: Rotor = self_745;
    let _e64: MultiVector = other_637;
    return MultiVector((((vec4<f32>(_e4.g0_.x) * _e8.g1_.yxwz) * vec4<f32>(1.0, -(1.0), 1.0, 1.0)) + ((vec4<f32>(_e19.g0_.x, _e22.g0_.y, _e25.g0_.x, _e28.g0_.x) * _e32.g1_.xyxx) * vec4<f32>(0.0, 1.0, 0.0, 0.0))), (((vec4<f32>(_e43.g0_.x) * _e47.g0_.yxwz) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))) + ((vec4<f32>(_e60.g0_.y) * _e64.g0_) * vec4<f32>(1.0, 1.0, -(1.0), 1.0))));
}

fn rotor_multi_vector_left_contraction(self_746: Rotor, other_638: MultiVector) -> MultiVector {
    var self_747: Rotor;
    var other_639: MultiVector;

    self_747 = self_746;
    other_639 = other_638;
    let _e4: Rotor = self_747;
    let _e8: MultiVector = other_639;
    let _e11: Rotor = self_747;
    let _e14: Rotor = self_747;
    let _e17: Rotor = self_747;
    let _e20: Rotor = self_747;
    let _e24: MultiVector = other_639;
    let _e36: Rotor = self_747;
    let _e40: MultiVector = other_639;
    let _e43: Rotor = self_747;
    let _e46: Rotor = self_747;
    let _e49: Rotor = self_747;
    let _e52: Rotor = self_747;
    let _e56: MultiVector = other_639;
    return MultiVector(((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.y, _e14.g0_.x, _e17.g0_.x, _e20.g0_.x) * _e24.g0_.yxxx) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))), ((vec4<f32>(_e36.g0_.x) * _e40.g1_) + ((vec4<f32>(_e43.g0_.y, _e46.g0_.x, _e49.g0_.x, _e52.g0_.x) * _e56.g1_.yxxx) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))));
}

fn rotor_multi_vector_right_anti_contraction(self_748: Rotor, other_640: MultiVector) -> MultiVector {
    var self_749: Rotor;
    var other_641: MultiVector;

    self_749 = self_748;
    other_641 = other_640;
    let _e4: Rotor = self_749;
    let _e8: MultiVector = other_641;
    let _e19: Rotor = self_749;
    let _e22: Rotor = self_749;
    let _e25: Rotor = self_749;
    let _e28: Rotor = self_749;
    let _e32: MultiVector = other_641;
    let _e43: Rotor = self_749;
    let _e47: MultiVector = other_641;
    let _e60: Rotor = self_749;
    let _e63: Rotor = self_749;
    let _e66: Rotor = self_749;
    let _e69: Rotor = self_749;
    let _e73: MultiVector = other_641;
    return MultiVector((((vec4<f32>(_e4.g0_.x) * _e8.g1_.yxwz) * vec4<f32>(1.0, -(1.0), 1.0, 1.0)) + ((vec4<f32>(_e19.g0_.x, _e22.g0_.y, _e25.g0_.x, _e28.g0_.x) * _e32.g1_.xyxx) * vec4<f32>(0.0, 1.0, 0.0, 0.0))), (((vec4<f32>(_e43.g0_.x) * _e47.g0_.yxwz) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))) + ((vec4<f32>(_e60.g0_.x, _e63.g0_.y, _e66.g0_.x, _e69.g0_.x) * _e73.g0_.xyxx) * vec4<f32>(0.0, 1.0, 0.0, 0.0))));
}

fn rotor_multi_vector_scalar_product(self_750: Rotor, other_642: MultiVector) -> Scalar {
    var self_751: Rotor;
    var other_643: MultiVector;

    self_751 = self_750;
    other_643 = other_642;
    let _e4: Rotor = self_751;
    let _e7: MultiVector = other_643;
    let _e11: Rotor = self_751;
    let _e14: MultiVector = other_643;
    return Scalar(((_e4.g0_.x * _e7.g0_.x) - (_e11.g0_.y * _e14.g0_.y)));
}

fn rotor_multi_vector_anti_scalar_product(self_752: Rotor, other_644: MultiVector) -> AntiScalar {
    var self_753: Rotor;
    var other_645: MultiVector;

    self_753 = self_752;
    other_645 = other_644;
    let _e5: Rotor = self_753;
    let _e8: MultiVector = other_645;
    let _e13: Rotor = self_753;
    let _e16: MultiVector = other_645;
    return AntiScalar(((0.0 - (_e5.g0_.x * _e8.g0_.x)) + (_e13.g0_.y * _e16.g0_.y)));
}

fn rotor_rotor_add(self_754: Rotor, other_646: Rotor) -> Rotor {
    var self_755: Rotor;
    var other_647: Rotor;

    self_755 = self_754;
    other_647 = other_646;
    let _e4: Rotor = self_755;
    let _e6: Rotor = other_647;
    return Rotor((_e4.g0_ + _e6.g0_));
}

fn rotor_rotor_sub(self_756: Rotor, other_648: Rotor) -> Rotor {
    var self_757: Rotor;
    var other_649: Rotor;

    self_757 = self_756;
    other_649 = other_648;
    let _e4: Rotor = self_757;
    let _e6: Rotor = other_649;
    return Rotor((_e4.g0_ - _e6.g0_));
}

fn rotor_rotor_mul(self_758: Rotor, other_650: Rotor) -> Rotor {
    var self_759: Rotor;
    var other_651: Rotor;

    self_759 = self_758;
    other_651 = other_650;
    let _e4: Rotor = self_759;
    let _e6: Rotor = other_651;
    return Rotor((_e4.g0_ * _e6.g0_));
}

fn rotor_rotor_div(self_760: Rotor, other_652: Rotor) -> Rotor {
    var self_761: Rotor;
    var other_653: Rotor;

    self_761 = self_760;
    other_653 = other_652;
    let _e4: Rotor = self_761;
    let _e7: Rotor = self_761;
    let _e15: Rotor = other_653;
    let _e18: Rotor = other_653;
    return Rotor((((vec2<f32>(_e4.g0_.x, _e7.g0_.y) * vec2<f32>(1.0, 1.0)) / vec2<f32>(_e15.g0_.x, _e18.g0_.y)) * vec2<f32>(1.0, 1.0)));
}

fn rotor_rotor_geometric_product(self_762: Rotor, other_654: Rotor) -> Rotor {
    var self_763: Rotor;
    var other_655: Rotor;

    self_763 = self_762;
    other_655 = other_654;
    let _e4: Rotor = self_763;
    let _e8: Rotor = other_655;
    let _e11: Rotor = self_763;
    let _e15: Rotor = other_655;
    return Rotor(((vec2<f32>(_e4.g0_.x) * _e8.g0_) + ((vec2<f32>(_e11.g0_.y) * _e15.g0_.yx) * vec2<f32>(-(1.0), 1.0))));
}

fn rotor_rotor_outer_product(self_764: Rotor, other_656: Rotor) -> Rotor {
    var self_765: Rotor;
    var other_657: Rotor;

    self_765 = self_764;
    other_657 = other_656;
    let _e4: Rotor = self_765;
    let _e8: Rotor = other_657;
    let _e11: Rotor = self_765;
    let _e13: Rotor = other_657;
    return Rotor(((vec2<f32>(_e4.g0_.x) * _e8.g0_) + ((_e11.g0_ * vec2<f32>(_e13.g0_.x)) * vec2<f32>(0.0, 1.0))));
}

fn rotor_rotor_inner_product(self_766: Rotor, other_658: Rotor) -> Rotor {
    var self_767: Rotor;
    var other_659: Rotor;

    self_767 = self_766;
    other_659 = other_658;
    let _e4: Rotor = self_767;
    let _e8: Rotor = other_659;
    let _e11: Rotor = self_767;
    let _e15: Rotor = other_659;
    return Rotor(((vec2<f32>(_e4.g0_.x) * _e8.g0_) + ((vec2<f32>(_e11.g0_.y) * _e15.g0_.yx) * vec2<f32>(-(1.0), 1.0))));
}

fn rotor_rotor_left_contraction(self_768: Rotor, other_660: Rotor) -> Rotor {
    var self_769: Rotor;
    var other_661: Rotor;

    self_769 = self_768;
    other_661 = other_660;
    let _e4: Rotor = self_769;
    let _e8: Rotor = other_661;
    let _e11: Rotor = self_769;
    let _e14: Rotor = other_661;
    return Rotor(((vec2<f32>(_e4.g0_.x) * _e8.g0_) + ((_e11.g0_.yx * _e14.g0_.yx) * vec2<f32>(-(1.0), 0.0))));
}

fn rotor_rotor_right_contraction(self_770: Rotor, other_662: Rotor) -> Rotor {
    var self_771: Rotor;
    var other_663: Rotor;

    self_771 = self_770;
    other_663 = other_662;
    let _e4: Rotor = self_771;
    let _e8: Rotor = other_663;
    let _e17: Rotor = self_771;
    let _e21: Rotor = other_663;
    return Rotor((((vec2<f32>(_e4.g0_.y) * _e8.g0_.yx) * vec2<f32>(-(1.0), 1.0)) + ((vec2<f32>(_e17.g0_.x) * vec2<f32>(_e21.g0_.x)) * vec2<f32>(1.0, 0.0))));
}

fn rotor_rotor_scalar_product(self_772: Rotor, other_664: Rotor) -> Scalar {
    var self_773: Rotor;
    var other_665: Rotor;

    self_773 = self_772;
    other_665 = other_664;
    let _e4: Rotor = self_773;
    let _e7: Rotor = other_665;
    let _e11: Rotor = self_773;
    let _e14: Rotor = other_665;
    return Scalar(((_e4.g0_.x * _e7.g0_.x) - (_e11.g0_.y * _e14.g0_.y)));
}

fn rotor_rotor_anti_scalar_product(self_774: Rotor, other_666: Rotor) -> AntiScalar {
    var self_775: Rotor;
    var other_667: Rotor;

    self_775 = self_774;
    other_667 = other_666;
    let _e5: Rotor = self_775;
    let _e8: Rotor = other_667;
    let _e13: Rotor = self_775;
    let _e16: Rotor = other_667;
    return AntiScalar(((0.0 - (_e5.g0_.x * _e8.g0_.x)) + (_e13.g0_.y * _e16.g0_.y)));
}

fn rotor_point_add(self_776: Rotor, other_668: Point) -> Motor {
    var self_777: Rotor;
    var other_669: Point;

    self_777 = self_776;
    other_669 = other_668;
    let _e4: Rotor = self_777;
    let _e7: Rotor = self_777;
    let _e10: Rotor = self_777;
    let _e13: Rotor = self_777;
    let _e23: Point = other_669;
    let _e26: Point = other_669;
    let _e29: Point = other_669;
    let _e32: Point = other_669;
    return Motor(((vec4<f32>(_e4.g0_.x, _e7.g0_.y, _e10.g0_.x, _e13.g0_.x) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) + (vec4<f32>(_e23.g0_.x, _e26.g0_.x, _e29.g0_.y, _e32.g0_.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn rotor_point_sub(self_778: Rotor, other_670: Point) -> Motor {
    var self_779: Rotor;
    var other_671: Point;

    self_779 = self_778;
    other_671 = other_670;
    let _e4: Rotor = self_779;
    let _e7: Rotor = self_779;
    let _e10: Rotor = self_779;
    let _e13: Rotor = self_779;
    let _e23: Point = other_671;
    let _e26: Point = other_671;
    let _e29: Point = other_671;
    let _e32: Point = other_671;
    return Motor(((vec4<f32>(_e4.g0_.x, _e7.g0_.y, _e10.g0_.x, _e13.g0_.x) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) - (vec4<f32>(_e23.g0_.x, _e26.g0_.x, _e29.g0_.y, _e32.g0_.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn rotor_point_geometric_product(self_780: Rotor, other_672: Point) -> Motor {
    var self_781: Rotor;
    var other_673: Point;

    self_781 = self_780;
    other_673 = other_672;
    let _e4: Rotor = self_781;
    let _e8: Point = other_673;
    let _e11: Point = other_673;
    let _e14: Point = other_673;
    let _e17: Point = other_673;
    let _e30: Rotor = self_781;
    let _e34: Point = other_673;
    let _e37: Point = other_673;
    let _e40: Point = other_673;
    let _e43: Point = other_673;
    return Motor((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.x, _e11.g0_.x, _e14.g0_.z, _e17.g0_.y)) * vec4<f32>(-(1.0), 0.0, -(1.0), 1.0)) + ((vec4<f32>(_e30.g0_.x) * vec4<f32>(_e34.g0_.x, _e37.g0_.x, _e40.g0_.y, _e43.g0_.z)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn rotor_point_outer_product(self_782: Rotor, other_674: Point) -> Point {
    var self_783: Rotor;
    var other_675: Point;

    self_783 = self_782;
    other_675 = other_674;
    let _e4: Rotor = self_783;
    let _e8: Point = other_675;
    return Point((vec3<f32>(_e4.g0_.x) * _e8.g0_));
}

fn rotor_point_inner_product(self_784: Rotor, other_676: Point) -> Motor {
    var self_785: Rotor;
    var other_677: Point;

    self_785 = self_784;
    other_677 = other_676;
    let _e4: Rotor = self_785;
    let _e7: Rotor = self_785;
    let _e10: Rotor = self_785;
    let _e13: Rotor = self_785;
    let _e17: Point = other_677;
    let _e20: Point = other_677;
    let _e23: Point = other_677;
    let _e26: Point = other_677;
    return Motor(((vec4<f32>(_e4.g0_.y, _e7.g0_.x, _e10.g0_.x, _e13.g0_.x) * vec4<f32>(_e17.g0_.x, _e20.g0_.x, _e23.g0_.y, _e26.g0_.z)) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0)));
}

fn rotor_point_geometric_anti_product(self_786: Rotor, other_678: Point) -> MotorDual {
    var self_787: Rotor;
    var other_679: Point;

    self_787 = self_786;
    other_679 = other_678;
    let _e4: Rotor = self_787;
    let _e8: Point = other_679;
    let _e11: Point = other_679;
    let _e14: Point = other_679;
    let _e17: Point = other_679;
    let _e29: Rotor = self_787;
    let _e33: Point = other_679;
    let _e36: Point = other_679;
    let _e39: Point = other_679;
    let _e42: Point = other_679;
    return MotorDual((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.x, _e11.g0_.x, _e14.g0_.z, _e17.g0_.y)) * vec4<f32>(1.0, 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e29.g0_.x) * vec4<f32>(_e33.g0_.x, _e36.g0_.x, _e39.g0_.y, _e42.g0_.z)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn rotor_point_inner_anti_product(self_788: Rotor, other_680: Point) -> MotorDual {
    var self_789: Rotor;
    var other_681: Point;

    self_789 = self_788;
    other_681 = other_680;
    let _e4: Rotor = self_789;
    let _e7: Rotor = self_789;
    let _e10: Rotor = self_789;
    let _e13: Rotor = self_789;
    let _e17: Point = other_681;
    let _e20: Point = other_681;
    let _e23: Point = other_681;
    let _e26: Point = other_681;
    return MotorDual((vec4<f32>(_e4.g0_.y, _e7.g0_.x, _e10.g0_.x, _e13.g0_.x) * vec4<f32>(_e17.g0_.x, _e20.g0_.x, _e23.g0_.y, _e26.g0_.z)));
}

fn rotor_point_left_contraction(self_790: Rotor, other_682: Point) -> Motor {
    var self_791: Rotor;
    var other_683: Point;

    self_791 = self_790;
    other_683 = other_682;
    let _e4: Rotor = self_791;
    let _e7: Rotor = self_791;
    let _e10: Rotor = self_791;
    let _e13: Rotor = self_791;
    let _e17: Point = other_683;
    let _e20: Point = other_683;
    let _e23: Point = other_683;
    let _e26: Point = other_683;
    return Motor(((vec4<f32>(_e4.g0_.y, _e7.g0_.x, _e10.g0_.x, _e13.g0_.x) * vec4<f32>(_e17.g0_.x, _e20.g0_.x, _e23.g0_.y, _e26.g0_.z)) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0)));
}

fn rotor_point_right_contraction(self_792: Rotor, other_684: Point) -> Scalar {
    var self_793: Rotor;
    var other_685: Point;

    self_793 = self_792;
    other_685 = other_684;
    let _e5: Rotor = self_793;
    let _e8: Point = other_685;
    return Scalar((0.0 - (_e5.g0_.y * _e8.g0_.x)));
}

fn rotor_point_left_anti_contraction(self_794: Rotor, other_686: Point) -> AntiScalar {
    var self_795: Rotor;
    var other_687: Point;

    self_795 = self_794;
    other_687 = other_686;
    let _e4: Rotor = self_795;
    let _e7: Point = other_687;
    return AntiScalar((_e4.g0_.y * _e7.g0_.x));
}

fn rotor_point_right_anti_contraction(self_796: Rotor, other_688: Point) -> MotorDual {
    var self_797: Rotor;
    var other_689: Point;

    self_797 = self_796;
    other_689 = other_688;
    let _e4: Rotor = self_797;
    let _e7: Rotor = self_797;
    let _e10: Rotor = self_797;
    let _e13: Rotor = self_797;
    let _e17: Point = other_689;
    let _e20: Point = other_689;
    let _e23: Point = other_689;
    let _e26: Point = other_689;
    return MotorDual((vec4<f32>(_e4.g0_.y, _e7.g0_.x, _e10.g0_.x, _e13.g0_.x) * vec4<f32>(_e17.g0_.x, _e20.g0_.x, _e23.g0_.y, _e26.g0_.z)));
}

fn rotor_point_scalar_product(self_798: Rotor, other_690: Point) -> Scalar {
    var self_799: Rotor;
    var other_691: Point;

    self_799 = self_798;
    other_691 = other_690;
    let _e5: Rotor = self_799;
    let _e8: Point = other_691;
    return Scalar((0.0 - (_e5.g0_.y * _e8.g0_.x)));
}

fn rotor_point_anti_scalar_product(self_800: Rotor, other_692: Point) -> AntiScalar {
    var self_801: Rotor;
    var other_693: Point;

    self_801 = self_800;
    other_693 = other_692;
    let _e4: Rotor = self_801;
    let _e7: Point = other_693;
    return AntiScalar((_e4.g0_.y * _e7.g0_.x));
}

fn rotor_ideal_point_add(self_802: Rotor, other_694: IdealPoint) -> Motor {
    var self_803: Rotor;
    var other_695: IdealPoint;

    self_803 = self_802;
    other_695 = other_694;
    let _e4: Rotor = self_803;
    let _e7: Rotor = self_803;
    let _e10: Rotor = self_803;
    let _e13: Rotor = self_803;
    let _e23: IdealPoint = other_695;
    let _e26: IdealPoint = other_695;
    let _e29: IdealPoint = other_695;
    let _e32: IdealPoint = other_695;
    return Motor(((vec4<f32>(_e4.g0_.x, _e7.g0_.y, _e10.g0_.x, _e13.g0_.x) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) + (vec4<f32>(_e23.g0_.x, _e26.g0_.x, _e29.g0_.x, _e32.g0_.y) * vec4<f32>(0.0, 0.0, 1.0, 1.0))));
}

fn rotor_ideal_point_sub(self_804: Rotor, other_696: IdealPoint) -> Motor {
    var self_805: Rotor;
    var other_697: IdealPoint;

    self_805 = self_804;
    other_697 = other_696;
    let _e4: Rotor = self_805;
    let _e7: Rotor = self_805;
    let _e10: Rotor = self_805;
    let _e13: Rotor = self_805;
    let _e23: IdealPoint = other_697;
    let _e26: IdealPoint = other_697;
    let _e29: IdealPoint = other_697;
    let _e32: IdealPoint = other_697;
    return Motor(((vec4<f32>(_e4.g0_.x, _e7.g0_.y, _e10.g0_.x, _e13.g0_.x) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) - (vec4<f32>(_e23.g0_.x, _e26.g0_.x, _e29.g0_.x, _e32.g0_.y) * vec4<f32>(0.0, 0.0, 1.0, 1.0))));
}

fn rotor_ideal_point_geometric_product(self_806: Rotor, other_698: IdealPoint) -> IdealPoint {
    var self_807: Rotor;
    var other_699: IdealPoint;

    self_807 = self_806;
    other_699 = other_698;
    let _e4: Rotor = self_807;
    let _e8: IdealPoint = other_699;
    let _e11: Rotor = self_807;
    let _e15: IdealPoint = other_699;
    return IdealPoint(((vec2<f32>(_e4.g0_.x) * _e8.g0_) + ((vec2<f32>(_e11.g0_.y) * _e15.g0_.yx) * vec2<f32>(-(1.0), 1.0))));
}

fn rotor_ideal_point_outer_product(self_808: Rotor, other_700: IdealPoint) -> IdealPoint {
    var self_809: Rotor;
    var other_701: IdealPoint;

    self_809 = self_808;
    other_701 = other_700;
    let _e4: Rotor = self_809;
    let _e8: IdealPoint = other_701;
    return IdealPoint((vec2<f32>(_e4.g0_.x) * _e8.g0_));
}

fn rotor_ideal_point_inner_product(self_810: Rotor, other_702: IdealPoint) -> IdealPoint {
    var self_811: Rotor;
    var other_703: IdealPoint;

    self_811 = self_810;
    other_703 = other_702;
    let _e4: Rotor = self_811;
    let _e8: IdealPoint = other_703;
    return IdealPoint((vec2<f32>(_e4.g0_.x) * _e8.g0_));
}

fn rotor_ideal_point_left_contraction(self_812: Rotor, other_704: IdealPoint) -> IdealPoint {
    var self_813: Rotor;
    var other_705: IdealPoint;

    self_813 = self_812;
    other_705 = other_704;
    let _e4: Rotor = self_813;
    let _e8: IdealPoint = other_705;
    return IdealPoint((vec2<f32>(_e4.g0_.x) * _e8.g0_));
}

fn rotor_plane_geometric_product(self_814: Rotor, other_706: Plane) -> MotorDual {
    var self_815: Rotor;
    var other_707: Plane;

    self_815 = self_814;
    other_707 = other_706;
    let _e4: Rotor = self_815;
    let _e8: Plane = other_707;
    let _e11: Plane = other_707;
    let _e14: Plane = other_707;
    let _e17: Plane = other_707;
    let _e29: Rotor = self_815;
    let _e33: Plane = other_707;
    let _e36: Plane = other_707;
    let _e39: Plane = other_707;
    let _e42: Plane = other_707;
    return MotorDual((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.x, _e11.g0_.x, _e14.g0_.z, _e17.g0_.y)) * vec4<f32>(1.0, 0.0, -(1.0), 1.0)) + ((vec4<f32>(_e29.g0_.x) * vec4<f32>(_e33.g0_.x, _e36.g0_.x, _e39.g0_.y, _e42.g0_.z)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn rotor_plane_regressive_product(self_816: Rotor, other_708: Plane) -> Scalar {
    var self_817: Rotor;
    var other_709: Plane;

    self_817 = self_816;
    other_709 = other_708;
    let _e4: Rotor = self_817;
    let _e7: Plane = other_709;
    return Scalar((_e4.g0_.y * _e7.g0_.x));
}

fn rotor_plane_outer_product(self_818: Rotor, other_710: Plane) -> MotorDual {
    var self_819: Rotor;
    var other_711: Plane;

    self_819 = self_818;
    other_711 = other_710;
    let _e4: Rotor = self_819;
    let _e7: Rotor = self_819;
    let _e10: Rotor = self_819;
    let _e13: Rotor = self_819;
    let _e17: Plane = other_711;
    let _e20: Plane = other_711;
    let _e23: Plane = other_711;
    let _e26: Plane = other_711;
    return MotorDual((vec4<f32>(_e4.g0_.y, _e7.g0_.x, _e10.g0_.x, _e13.g0_.x) * vec4<f32>(_e17.g0_.x, _e20.g0_.x, _e23.g0_.y, _e26.g0_.z)));
}

fn rotor_plane_inner_product(self_820: Rotor, other_712: Plane) -> Plane {
    var self_821: Rotor;
    var other_713: Plane;

    self_821 = self_820;
    other_713 = other_712;
    let _e4: Rotor = self_821;
    let _e8: Plane = other_713;
    let _e11: Rotor = self_821;
    let _e14: Rotor = self_821;
    let _e17: Rotor = self_821;
    let _e21: Plane = other_713;
    return Plane(((vec3<f32>(_e4.g0_.x) * _e8.g0_) + ((vec3<f32>(_e11.g0_.x, _e14.g0_.y, _e17.g0_.y) * _e21.g0_.xzy) * vec3<f32>(0.0, -(1.0), 1.0))));
}

fn rotor_plane_geometric_anti_product(self_822: Rotor, other_714: Plane) -> Motor {
    var self_823: Rotor;
    var other_715: Plane;

    self_823 = self_822;
    other_715 = other_714;
    let _e4: Rotor = self_823;
    let _e8: Plane = other_715;
    let _e11: Plane = other_715;
    let _e14: Plane = other_715;
    let _e17: Plane = other_715;
    let _e29: Rotor = self_823;
    let _e33: Plane = other_715;
    let _e36: Plane = other_715;
    let _e39: Plane = other_715;
    let _e42: Plane = other_715;
    return Motor((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.x, _e11.g0_.x, _e14.g0_.z, _e17.g0_.y)) * vec4<f32>(1.0, 0.0, -(1.0), 1.0)) + ((vec4<f32>(_e29.g0_.x) * vec4<f32>(_e33.g0_.x, _e36.g0_.x, _e39.g0_.y, _e42.g0_.z)) * vec4<f32>(0.0, -(1.0), -(1.0), -(1.0)))));
}

fn rotor_plane_inner_anti_product(self_824: Rotor, other_716: Plane) -> Point {
    var self_825: Rotor;
    var other_717: Plane;

    self_825 = self_824;
    other_717 = other_716;
    let _e6: Rotor = self_825;
    let _e10: Plane = other_717;
    let _e14: Rotor = self_825;
    let _e17: Rotor = self_825;
    let _e20: Rotor = self_825;
    let _e24: Plane = other_717;
    return Point(((vec3<f32>(0.0) - (vec3<f32>(_e6.g0_.x) * _e10.g0_)) + ((vec3<f32>(_e14.g0_.x, _e17.g0_.y, _e20.g0_.y) * _e24.g0_.xzy) * vec3<f32>(0.0, -(1.0), 1.0))));
}

fn rotor_plane_left_contraction(self_826: Rotor, other_718: Plane) -> Plane {
    var self_827: Rotor;
    var other_719: Plane;

    self_827 = self_826;
    other_719 = other_718;
    let _e4: Rotor = self_827;
    let _e8: Plane = other_719;
    return Plane((vec3<f32>(_e4.g0_.x) * _e8.g0_));
}

fn rotor_plane_left_anti_contraction(self_828: Rotor, other_720: Plane) -> IdealPoint {
    var self_829: Rotor;
    var other_721: Plane;

    self_829 = self_828;
    other_721 = other_720;
    let _e4: Rotor = self_829;
    let _e8: Plane = other_721;
    let _e11: Plane = other_721;
    return IdealPoint(((vec2<f32>(_e4.g0_.y) * vec2<f32>(_e8.g0_.z, _e11.g0_.y)) * vec2<f32>(-(1.0), 1.0)));
}

fn rotor_plane_right_anti_contraction(self_830: Rotor, other_722: Plane) -> Point {
    var self_831: Rotor;
    var other_723: Plane;

    self_831 = self_830;
    other_723 = other_722;
    let _e6: Rotor = self_831;
    let _e10: Plane = other_723;
    return Point((vec3<f32>(0.0) - (vec3<f32>(_e6.g0_.x) * _e10.g0_)));
}

fn rotor_translator_add(self_832: Rotor, other_724: Translator) -> Motor {
    var self_833: Rotor;
    var other_725: Translator;

    self_833 = self_832;
    other_725 = other_724;
    let _e4: Rotor = self_833;
    let _e7: Rotor = self_833;
    let _e10: Rotor = self_833;
    let _e13: Rotor = self_833;
    let _e23: Translator = other_725;
    let _e26: Translator = other_725;
    let _e29: Translator = other_725;
    let _e32: Translator = other_725;
    return Motor(((vec4<f32>(_e4.g0_.x, _e7.g0_.y, _e10.g0_.x, _e13.g0_.x) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) + (vec4<f32>(_e23.g0_.x, _e26.g0_.x, _e29.g0_.y, _e32.g0_.z) * vec4<f32>(1.0, 0.0, 1.0, 1.0))));
}

fn rotor_translator_sub(self_834: Rotor, other_726: Translator) -> Motor {
    var self_835: Rotor;
    var other_727: Translator;

    self_835 = self_834;
    other_727 = other_726;
    let _e4: Rotor = self_835;
    let _e7: Rotor = self_835;
    let _e10: Rotor = self_835;
    let _e13: Rotor = self_835;
    let _e23: Translator = other_727;
    let _e26: Translator = other_727;
    let _e29: Translator = other_727;
    let _e32: Translator = other_727;
    return Motor(((vec4<f32>(_e4.g0_.x, _e7.g0_.y, _e10.g0_.x, _e13.g0_.x) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) - (vec4<f32>(_e23.g0_.x, _e26.g0_.x, _e29.g0_.y, _e32.g0_.z) * vec4<f32>(1.0, 0.0, 1.0, 1.0))));
}

fn rotor_translator_geometric_product(self_836: Rotor, other_728: Translator) -> Motor {
    var self_837: Rotor;
    var other_729: Translator;

    self_837 = self_836;
    other_729 = other_728;
    let _e4: Rotor = self_837;
    let _e8: Translator = other_729;
    let _e11: Translator = other_729;
    let _e14: Translator = other_729;
    let _e17: Translator = other_729;
    let _e29: Rotor = self_837;
    let _e33: Translator = other_729;
    let _e36: Translator = other_729;
    let _e39: Translator = other_729;
    let _e42: Translator = other_729;
    return Motor((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.x, _e11.g0_.x, _e14.g0_.z, _e17.g0_.y)) * vec4<f32>(0.0, 1.0, -(1.0), 1.0)) + ((vec4<f32>(_e29.g0_.x) * vec4<f32>(_e33.g0_.x, _e36.g0_.x, _e39.g0_.y, _e42.g0_.z)) * vec4<f32>(1.0, 0.0, 1.0, 1.0))));
}

fn rotor_translator_outer_product(self_838: Rotor, other_730: Translator) -> Motor {
    var self_839: Rotor;
    var other_731: Translator;

    self_839 = self_838;
    other_731 = other_730;
    let _e4: Rotor = self_839;
    let _e7: Rotor = self_839;
    let _e10: Rotor = self_839;
    let _e13: Rotor = self_839;
    let _e17: Translator = other_731;
    let _e20: Translator = other_731;
    let _e23: Translator = other_731;
    let _e26: Translator = other_731;
    return Motor((vec4<f32>(_e4.g0_.x, _e7.g0_.y, _e10.g0_.x, _e13.g0_.x) * vec4<f32>(_e17.g0_.x, _e20.g0_.x, _e23.g0_.y, _e26.g0_.z)));
}

fn rotor_translator_inner_product(self_840: Rotor, other_732: Translator) -> Motor {
    var self_841: Rotor;
    var other_733: Translator;

    self_841 = self_840;
    other_733 = other_732;
    let _e4: Rotor = self_841;
    let _e7: Rotor = self_841;
    let _e10: Rotor = self_841;
    let _e13: Rotor = self_841;
    let _e17: Translator = other_733;
    let _e20: Translator = other_733;
    let _e23: Translator = other_733;
    let _e26: Translator = other_733;
    return Motor((vec4<f32>(_e4.g0_.x, _e7.g0_.y, _e10.g0_.x, _e13.g0_.x) * vec4<f32>(_e17.g0_.x, _e20.g0_.x, _e23.g0_.y, _e26.g0_.z)));
}

fn rotor_translator_geometric_anti_product(self_842: Rotor, other_734: Translator) -> MotorDual {
    var self_843: Rotor;
    var other_735: Translator;

    self_843 = self_842;
    other_735 = other_734;
    let _e4: Rotor = self_843;
    let _e8: Translator = other_735;
    let _e11: Translator = other_735;
    let _e14: Translator = other_735;
    let _e17: Translator = other_735;
    let _e29: Rotor = self_843;
    let _e33: Translator = other_735;
    let _e36: Translator = other_735;
    let _e39: Translator = other_735;
    let _e42: Translator = other_735;
    return MotorDual((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.x, _e11.g0_.x, _e14.g0_.z, _e17.g0_.y)) * vec4<f32>(0.0, 1.0, 1.0, -(1.0))) + ((vec4<f32>(_e29.g0_.x) * vec4<f32>(_e33.g0_.x, _e36.g0_.x, _e39.g0_.y, _e42.g0_.z)) * vec4<f32>(-(1.0), 0.0, 1.0, 1.0))));
}

fn rotor_translator_inner_anti_product(self_844: Rotor, other_736: Translator) -> MotorDual {
    var self_845: Rotor;
    var other_737: Translator;

    self_845 = self_844;
    other_737 = other_736;
    let _e4: Rotor = self_845;
    let _e7: Rotor = self_845;
    let _e10: Rotor = self_845;
    let _e13: Rotor = self_845;
    let _e17: Translator = other_737;
    let _e20: Translator = other_737;
    let _e23: Translator = other_737;
    let _e26: Translator = other_737;
    return MotorDual(((vec4<f32>(_e4.g0_.x, _e7.g0_.y, _e10.g0_.x, _e13.g0_.x) * vec4<f32>(_e17.g0_.x, _e20.g0_.x, _e23.g0_.y, _e26.g0_.z)) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0)));
}

fn rotor_translator_left_contraction(self_846: Rotor, other_738: Translator) -> Translator {
    var self_847: Rotor;
    var other_739: Translator;

    self_847 = self_846;
    other_739 = other_738;
    let _e4: Rotor = self_847;
    let _e8: Translator = other_739;
    return Translator((vec3<f32>(_e4.g0_.x) * _e8.g0_));
}

fn rotor_translator_right_contraction(self_848: Rotor, other_740: Translator) -> Rotor {
    var self_849: Rotor;
    var other_741: Translator;

    self_849 = self_848;
    other_741 = other_740;
    let _e4: Rotor = self_849;
    let _e6: Translator = other_741;
    return Rotor((_e4.g0_ * vec2<f32>(_e6.g0_.x)));
}

fn rotor_translator_scalar_product(self_850: Rotor, other_742: Translator) -> Scalar {
    var self_851: Rotor;
    var other_743: Translator;

    self_851 = self_850;
    other_743 = other_742;
    let _e4: Rotor = self_851;
    let _e7: Translator = other_743;
    return Scalar((_e4.g0_.x * _e7.g0_.x));
}

fn rotor_translator_anti_scalar_product(self_852: Rotor, other_744: Translator) -> AntiScalar {
    var self_853: Rotor;
    var other_745: Translator;

    self_853 = self_852;
    other_745 = other_744;
    let _e5: Rotor = self_853;
    let _e8: Translator = other_745;
    return AntiScalar((0.0 - (_e5.g0_.x * _e8.g0_.x)));
}

fn rotor_motor_add(self_854: Rotor, other_746: Motor) -> Motor {
    var self_855: Rotor;
    var other_747: Motor;

    self_855 = self_854;
    other_747 = other_746;
    let _e4: Rotor = self_855;
    let _e7: Rotor = self_855;
    let _e10: Rotor = self_855;
    let _e13: Rotor = self_855;
    let _e23: Motor = other_747;
    return Motor(((vec4<f32>(_e4.g0_.x, _e7.g0_.y, _e10.g0_.x, _e13.g0_.x) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) + _e23.g0_));
}

fn rotor_motor_sub(self_856: Rotor, other_748: Motor) -> Motor {
    var self_857: Rotor;
    var other_749: Motor;

    self_857 = self_856;
    other_749 = other_748;
    let _e4: Rotor = self_857;
    let _e7: Rotor = self_857;
    let _e10: Rotor = self_857;
    let _e13: Rotor = self_857;
    let _e23: Motor = other_749;
    return Motor(((vec4<f32>(_e4.g0_.x, _e7.g0_.y, _e10.g0_.x, _e13.g0_.x) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) - _e23.g0_));
}

fn rotor_motor_geometric_product(self_858: Rotor, other_750: Motor) -> Motor {
    var self_859: Rotor;
    var other_751: Motor;

    self_859 = self_858;
    other_751 = other_750;
    let _e4: Rotor = self_859;
    let _e8: Motor = other_751;
    let _e11: Rotor = self_859;
    let _e15: Motor = other_751;
    return Motor(((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.y) * _e15.g0_.yxwz) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))));
}

fn rotor_motor_outer_product(self_860: Rotor, other_752: Motor) -> Motor {
    var self_861: Rotor;
    var other_753: Motor;

    self_861 = self_860;
    other_753 = other_752;
    let _e4: Rotor = self_861;
    let _e8: Motor = other_753;
    let _e11: Rotor = self_861;
    let _e14: Rotor = self_861;
    let _e17: Rotor = self_861;
    let _e20: Rotor = self_861;
    let _e24: Motor = other_753;
    return Motor(((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.x, _e14.g0_.y, _e17.g0_.x, _e20.g0_.x) * vec4<f32>(_e24.g0_.x)) * vec4<f32>(0.0, 1.0, 0.0, 0.0))));
}

fn rotor_motor_inner_product(self_862: Rotor, other_754: Motor) -> Motor {
    var self_863: Rotor;
    var other_755: Motor;

    self_863 = self_862;
    other_755 = other_754;
    let _e4: Rotor = self_863;
    let _e8: Motor = other_755;
    let _e11: Rotor = self_863;
    let _e14: Rotor = self_863;
    let _e17: Rotor = self_863;
    let _e20: Rotor = self_863;
    let _e24: Motor = other_755;
    return Motor(((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.y, _e14.g0_.y, _e17.g0_.x, _e20.g0_.x) * _e24.g0_.yxxx) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))));
}

fn rotor_motor_geometric_anti_product(self_864: Rotor, other_756: Motor) -> MotorDual {
    var self_865: Rotor;
    var other_757: Motor;

    self_865 = self_864;
    other_757 = other_756;
    let _e4: Rotor = self_865;
    let _e8: Motor = other_757;
    let _e18: Rotor = self_865;
    let _e22: Motor = other_757;
    return MotorDual((((vec4<f32>(_e4.g0_.x) * _e8.g0_) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0)) + ((vec4<f32>(_e18.g0_.y) * _e22.g0_.yxwz) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))));
}

fn rotor_motor_inner_anti_product(self_866: Rotor, other_758: Motor) -> MotorDual {
    var self_867: Rotor;
    var other_759: Motor;

    self_867 = self_866;
    other_759 = other_758;
    let _e4: Rotor = self_867;
    let _e8: Motor = other_759;
    let _e18: Rotor = self_867;
    let _e21: Rotor = self_867;
    let _e24: Rotor = self_867;
    let _e27: Rotor = self_867;
    let _e31: Motor = other_759;
    return MotorDual((((vec4<f32>(_e4.g0_.x) * _e8.g0_) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0)) + ((vec4<f32>(_e18.g0_.y, _e21.g0_.y, _e24.g0_.x, _e27.g0_.x) * _e31.g0_.yxxx) * vec4<f32>(1.0, 1.0, 0.0, 0.0))));
}

fn rotor_motor_left_contraction(self_868: Rotor, other_760: Motor) -> Motor {
    var self_869: Rotor;
    var other_761: Motor;

    self_869 = self_868;
    other_761 = other_760;
    let _e4: Rotor = self_869;
    let _e8: Motor = other_761;
    let _e11: Rotor = self_869;
    let _e14: Rotor = self_869;
    let _e17: Rotor = self_869;
    let _e20: Rotor = self_869;
    let _e24: Motor = other_761;
    return Motor(((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.y, _e14.g0_.x, _e17.g0_.x, _e20.g0_.x) * _e24.g0_.yxxx) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))));
}

fn rotor_motor_right_contraction(self_870: Rotor, other_762: Motor) -> Rotor {
    var self_871: Rotor;
    var other_763: Motor;

    self_871 = self_870;
    other_763 = other_762;
    let _e4: Rotor = self_871;
    let _e8: Motor = other_763;
    let _e11: Motor = other_763;
    let _e21: Rotor = self_871;
    let _e25: Motor = other_763;
    return Rotor((((vec2<f32>(_e4.g0_.y) * vec2<f32>(_e8.g0_.y, _e11.g0_.x)) * vec2<f32>(-(1.0), 1.0)) + ((vec2<f32>(_e21.g0_.x) * vec2<f32>(_e25.g0_.x)) * vec2<f32>(1.0, 0.0))));
}

fn rotor_motor_right_anti_contraction(self_872: Rotor, other_764: Motor) -> MotorDual {
    var self_873: Rotor;
    var other_765: Motor;

    self_873 = self_872;
    other_765 = other_764;
    let _e4: Rotor = self_873;
    let _e8: Motor = other_765;
    let _e18: Rotor = self_873;
    let _e21: Rotor = self_873;
    let _e24: Rotor = self_873;
    let _e27: Rotor = self_873;
    let _e31: Motor = other_765;
    return MotorDual((((vec4<f32>(_e4.g0_.x) * _e8.g0_) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0)) + ((vec4<f32>(_e18.g0_.y, _e21.g0_.x, _e24.g0_.x, _e27.g0_.x) * _e31.g0_.yxxx) * vec4<f32>(1.0, 0.0, 0.0, 0.0))));
}

fn rotor_motor_scalar_product(self_874: Rotor, other_766: Motor) -> Scalar {
    var self_875: Rotor;
    var other_767: Motor;

    self_875 = self_874;
    other_767 = other_766;
    let _e4: Rotor = self_875;
    let _e7: Motor = other_767;
    let _e11: Rotor = self_875;
    let _e14: Motor = other_767;
    return Scalar(((_e4.g0_.x * _e7.g0_.x) - (_e11.g0_.y * _e14.g0_.y)));
}

fn rotor_motor_anti_scalar_product(self_876: Rotor, other_768: Motor) -> AntiScalar {
    var self_877: Rotor;
    var other_769: Motor;

    self_877 = self_876;
    other_769 = other_768;
    let _e5: Rotor = self_877;
    let _e8: Motor = other_769;
    let _e13: Rotor = self_877;
    let _e16: Motor = other_769;
    return AntiScalar(((0.0 - (_e5.g0_.x * _e8.g0_.x)) + (_e13.g0_.y * _e16.g0_.y)));
}

fn rotor_motor_dual_geometric_product(self_878: Rotor, other_770: MotorDual) -> MotorDual {
    var self_879: Rotor;
    var other_771: MotorDual;

    self_879 = self_878;
    other_771 = other_770;
    let _e4: Rotor = self_879;
    let _e8: MotorDual = other_771;
    let _e11: Rotor = self_879;
    let _e15: MotorDual = other_771;
    return MotorDual(((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.y) * _e15.g0_.yxwz) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0))));
}

fn rotor_motor_dual_regressive_product(self_880: Rotor, other_772: MotorDual) -> Rotor {
    var self_881: Rotor;
    var other_773: MotorDual;

    self_881 = self_880;
    other_773 = other_772;
    let _e4: Rotor = self_881;
    let _e8: MotorDual = other_773;
    let _e11: MotorDual = other_773;
    let _e16: Rotor = self_881;
    let _e20: MotorDual = other_773;
    return Rotor(((vec2<f32>(_e4.g0_.y) * vec2<f32>(_e8.g0_.y, _e11.g0_.x)) + ((vec2<f32>(_e16.g0_.x) * vec2<f32>(_e20.g0_.x)) * vec2<f32>(1.0, 0.0))));
}

fn rotor_motor_dual_outer_product(self_882: Rotor, other_774: MotorDual) -> MotorDual {
    var self_883: Rotor;
    var other_775: MotorDual;

    self_883 = self_882;
    other_775 = other_774;
    let _e4: Rotor = self_883;
    let _e8: MotorDual = other_775;
    let _e11: Rotor = self_883;
    let _e14: Rotor = self_883;
    let _e17: Rotor = self_883;
    let _e20: Rotor = self_883;
    let _e24: MotorDual = other_775;
    return MotorDual(((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.y, _e14.g0_.x, _e17.g0_.x, _e20.g0_.x) * _e24.g0_.yxxx) * vec4<f32>(1.0, 0.0, 0.0, 0.0))));
}

fn rotor_motor_dual_inner_product(self_884: Rotor, other_776: MotorDual) -> MotorDual {
    var self_885: Rotor;
    var other_777: MotorDual;

    self_885 = self_884;
    other_777 = other_776;
    let _e4: Rotor = self_885;
    let _e8: MotorDual = other_777;
    let _e11: Rotor = self_885;
    let _e14: Rotor = self_885;
    let _e17: Rotor = self_885;
    let _e20: Rotor = self_885;
    let _e24: MotorDual = other_777;
    return MotorDual(((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.x, _e14.g0_.y, _e17.g0_.y, _e20.g0_.y) * _e24.g0_.xxwz) * vec4<f32>(0.0, -(1.0), -(1.0), 1.0))));
}

fn rotor_motor_dual_geometric_anti_product(self_886: Rotor, other_778: MotorDual) -> Motor {
    var self_887: Rotor;
    var other_779: MotorDual;

    self_887 = self_886;
    other_779 = other_778;
    let _e4: Rotor = self_887;
    let _e8: MotorDual = other_779;
    let _e20: Rotor = self_887;
    let _e24: MotorDual = other_779;
    return Motor((((vec4<f32>(_e4.g0_.x) * _e8.g0_) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))) + ((vec4<f32>(_e20.g0_.y) * _e24.g0_.yxwz) * vec4<f32>(1.0, 1.0, -(1.0), 1.0))));
}

fn rotor_motor_dual_inner_anti_product(self_888: Rotor, other_780: MotorDual) -> Motor {
    var self_889: Rotor;
    var other_781: MotorDual;

    self_889 = self_888;
    other_781 = other_780;
    let _e4: Rotor = self_889;
    let _e8: MotorDual = other_781;
    let _e20: Rotor = self_889;
    let _e23: Rotor = self_889;
    let _e26: Rotor = self_889;
    let _e29: Rotor = self_889;
    let _e33: MotorDual = other_781;
    return Motor((((vec4<f32>(_e4.g0_.x) * _e8.g0_) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))) + ((vec4<f32>(_e20.g0_.x, _e23.g0_.y, _e26.g0_.y, _e29.g0_.y) * _e33.g0_.xxwz) * vec4<f32>(0.0, 1.0, -(1.0), 1.0))));
}

fn rotor_motor_dual_left_contraction(self_890: Rotor, other_782: MotorDual) -> MotorDual {
    var self_891: Rotor;
    var other_783: MotorDual;

    self_891 = self_890;
    other_783 = other_782;
    let _e4: Rotor = self_891;
    let _e8: MotorDual = other_783;
    let _e11: Rotor = self_891;
    let _e14: Rotor = self_891;
    let _e17: Rotor = self_891;
    let _e20: Rotor = self_891;
    let _e24: MotorDual = other_783;
    return MotorDual(((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.x, _e14.g0_.y, _e17.g0_.x, _e20.g0_.x) * vec4<f32>(_e24.g0_.x)) * vec4<f32>(0.0, -(1.0), 0.0, 0.0))));
}

fn rotor_motor_dual_left_anti_contraction(self_892: Rotor, other_784: MotorDual) -> IdealPoint {
    var self_893: Rotor;
    var other_785: MotorDual;

    self_893 = self_892;
    other_785 = other_784;
    let _e4: Rotor = self_893;
    let _e8: MotorDual = other_785;
    let _e11: MotorDual = other_785;
    return IdealPoint(((vec2<f32>(_e4.g0_.y) * vec2<f32>(_e8.g0_.w, _e11.g0_.z)) * vec2<f32>(-(1.0), 1.0)));
}

fn rotor_motor_dual_right_anti_contraction(self_894: Rotor, other_786: MotorDual) -> Motor {
    var self_895: Rotor;
    var other_787: MotorDual;

    self_895 = self_894;
    other_787 = other_786;
    let _e4: Rotor = self_895;
    let _e8: MotorDual = other_787;
    let _e20: Rotor = self_895;
    let _e23: Rotor = self_895;
    let _e26: Rotor = self_895;
    let _e29: Rotor = self_895;
    let _e33: MotorDual = other_787;
    return Motor((((vec4<f32>(_e4.g0_.x) * _e8.g0_) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))) + ((vec4<f32>(_e20.g0_.x, _e23.g0_.y, _e26.g0_.x, _e29.g0_.x) * vec4<f32>(_e33.g0_.x)) * vec4<f32>(0.0, 1.0, 0.0, 0.0))));
}

fn rotor_squared_magnitude(self_896: Rotor) -> Scalar {
    var self_897: Rotor;

    self_897 = self_896;
    let _e2: Rotor = self_897;
    let _e3: Rotor = self_897;
    let _e4: Rotor = rotor_reversal(_e3);
    let _e5: Scalar = rotor_rotor_scalar_product(_e2, _e4);
    return _e5;
}

fn rotor_magnitude(self_898: Rotor) -> Scalar {
    var self_899: Rotor;

    self_899 = self_898;
    let _e2: Rotor = self_899;
    let _e3: Scalar = rotor_squared_magnitude(_e2);
    return Scalar(sqrt(_e3.g0_));
}

fn rotor_bulk_norm(self_900: Rotor) -> Scalar {
    var self_901: Rotor;

    self_901 = self_900;
    let _e2: Rotor = self_901;
    let _e3: Scalar = rotor_squared_magnitude(_e2);
    return Scalar(sqrt(_e3.g0_));
}

fn rotor_squared_anti_magnitude(self_902: Rotor) -> AntiScalar {
    var self_903: Rotor;

    self_903 = self_902;
    let _e2: Rotor = self_903;
    let _e3: Rotor = self_903;
    let _e4: Rotor = rotor_anti_reversal(_e3);
    let _e5: AntiScalar = rotor_rotor_anti_scalar_product(_e2, _e4);
    return _e5;
}

fn rotor_weight_norm(self_904: Rotor) -> AntiScalar {
    var self_905: Rotor;

    self_905 = self_904;
    let _e2: Rotor = self_905;
    let _e3: AntiScalar = rotor_squared_anti_magnitude(_e2);
    return AntiScalar(sqrt(_e3.g0_));
}

fn rotor_scale(self_906: Rotor, other_788: f32) -> Rotor {
    var self_907: Rotor;
    var other_789: f32;

    self_907 = self_906;
    other_789 = other_788;
    let _e4: Rotor = self_907;
    let _e5: f32 = other_789;
    let _e7: Rotor = rotor_scalar_geometric_product(_e4, Scalar(_e5));
    return _e7;
}

fn rotor_signum(self_908: Rotor) -> Rotor {
    var self_909: Rotor;

    self_909 = self_908;
    let _e2: Rotor = self_909;
    let _e3: Rotor = self_909;
    let _e4: Scalar = rotor_magnitude(_e3);
    let _e9: Rotor = rotor_scalar_geometric_product(_e2, Scalar((1.0 / _e4.g0_)));
    return _e9;
}

fn rotor_inverse(self_910: Rotor) -> Rotor {
    var self_911: Rotor;

    self_911 = self_910;
    let _e2: Rotor = self_911;
    let _e3: Rotor = rotor_reversal(_e2);
    let _e4: Rotor = self_911;
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

fn point_neg(self_912: Point) -> Point {
    var self_913: Point;

    self_913 = self_912;
    let _e2: Point = self_913;
    return Point((_e2.g0_ * vec3<f32>(-(1.0))));
}

fn point_automorphism(self_914: Point) -> Point {
    var self_915: Point;

    self_915 = self_914;
    let _e2: Point = self_915;
    return Point(_e2.g0_);
}

fn point_reversal(self_916: Point) -> Point {
    var self_917: Point;

    self_917 = self_916;
    let _e2: Point = self_917;
    return Point((_e2.g0_ * vec3<f32>(-(1.0))));
}

fn point_conjugation(self_918: Point) -> Point {
    var self_919: Point;

    self_919 = self_918;
    let _e2: Point = self_919;
    return Point((_e2.g0_ * vec3<f32>(-(1.0))));
}

fn point_dual(self_920: Point) -> Plane {
    var self_921: Point;

    self_921 = self_920;
    let _e2: Point = self_921;
    return Plane(_e2.g0_);
}

fn point_anti_reversal(self_922: Point) -> Point {
    var self_923: Point;

    self_923 = self_922;
    let _e2: Point = self_923;
    return Point((_e2.g0_ * vec3<f32>(-(1.0))));
}

fn point_scalar_add(self_924: Point, other_790: Scalar) -> Motor {
    var self_925: Point;
    var other_791: Scalar;

    self_925 = self_924;
    other_791 = other_790;
    let _e4: Point = self_925;
    let _e7: Point = self_925;
    let _e10: Point = self_925;
    let _e13: Point = self_925;
    let _e23: Scalar = other_791;
    return Motor(((vec4<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.y, _e13.g0_.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0)) + (vec4<f32>(_e23.g0_) * vec4<f32>(1.0, 0.0, 0.0, 0.0))));
}

fn point_scalar_sub(self_926: Point, other_792: Scalar) -> Motor {
    var self_927: Point;
    var other_793: Scalar;

    self_927 = self_926;
    other_793 = other_792;
    let _e4: Point = self_927;
    let _e7: Point = self_927;
    let _e10: Point = self_927;
    let _e13: Point = self_927;
    let _e23: Scalar = other_793;
    return Motor(((vec4<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.y, _e13.g0_.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0)) - (vec4<f32>(_e23.g0_) * vec4<f32>(1.0, 0.0, 0.0, 0.0))));
}

fn point_scalar_geometric_product(self_928: Point, other_794: Scalar) -> Point {
    var self_929: Point;
    var other_795: Scalar;

    self_929 = self_928;
    other_795 = other_794;
    let _e4: Point = self_929;
    let _e6: Scalar = other_795;
    return Point((_e4.g0_ * vec3<f32>(_e6.g0_)));
}

fn point_scalar_outer_product(self_930: Point, other_796: Scalar) -> Point {
    var self_931: Point;
    var other_797: Scalar;

    self_931 = self_930;
    other_797 = other_796;
    let _e4: Point = self_931;
    let _e6: Scalar = other_797;
    return Point((_e4.g0_ * vec3<f32>(_e6.g0_)));
}

fn point_scalar_inner_product(self_932: Point, other_798: Scalar) -> Point {
    var self_933: Point;
    var other_799: Scalar;

    self_933 = self_932;
    other_799 = other_798;
    let _e4: Point = self_933;
    let _e6: Scalar = other_799;
    return Point((_e4.g0_ * vec3<f32>(_e6.g0_)));
}

fn point_scalar_geometric_anti_product(self_934: Point, other_800: Scalar) -> Plane {
    var self_935: Point;
    var other_801: Scalar;

    self_935 = self_934;
    other_801 = other_800;
    let _e4: Point = self_935;
    let _e6: Scalar = other_801;
    return Plane((_e4.g0_ * vec3<f32>(_e6.g0_)));
}

fn point_scalar_inner_anti_product(self_936: Point, other_802: Scalar) -> Plane {
    var self_937: Point;
    var other_803: Scalar;

    self_937 = self_936;
    other_803 = other_802;
    let _e4: Point = self_937;
    let _e6: Scalar = other_803;
    return Plane((_e4.g0_ * vec3<f32>(_e6.g0_)));
}

fn point_scalar_right_contraction(self_938: Point, other_804: Scalar) -> Point {
    var self_939: Point;
    var other_805: Scalar;

    self_939 = self_938;
    other_805 = other_804;
    let _e4: Point = self_939;
    let _e6: Scalar = other_805;
    return Point((_e4.g0_ * vec3<f32>(_e6.g0_)));
}

fn point_scalar_left_anti_contraction(self_940: Point, other_806: Scalar) -> Plane {
    var self_941: Point;
    var other_807: Scalar;

    self_941 = self_940;
    other_807 = other_806;
    let _e4: Point = self_941;
    let _e6: Scalar = other_807;
    return Plane((_e4.g0_ * vec3<f32>(_e6.g0_)));
}

fn point_anti_scalar_geometric_product(self_942: Point, other_808: AntiScalar) -> Plane {
    var self_943: Point;
    var other_809: AntiScalar;

    self_943 = self_942;
    other_809 = other_808;
    let _e4: Point = self_943;
    let _e6: AntiScalar = other_809;
    return Plane(((_e4.g0_ * vec3<f32>(_e6.g0_)) * vec3<f32>(-(1.0))));
}

fn point_anti_scalar_regressive_product(self_944: Point, other_810: AntiScalar) -> Point {
    var self_945: Point;
    var other_811: AntiScalar;

    self_945 = self_944;
    other_811 = other_810;
    let _e4: Point = self_945;
    let _e6: AntiScalar = other_811;
    return Point((_e4.g0_ * vec3<f32>(_e6.g0_)));
}

fn point_anti_scalar_inner_product(self_946: Point, other_812: AntiScalar) -> Plane {
    var self_947: Point;
    var other_813: AntiScalar;

    self_947 = self_946;
    other_813 = other_812;
    let _e4: Point = self_947;
    let _e6: AntiScalar = other_813;
    return Plane(((_e4.g0_ * vec3<f32>(_e6.g0_)) * vec3<f32>(-(1.0))));
}

fn point_anti_scalar_geometric_anti_product(self_948: Point, other_814: AntiScalar) -> Point {
    var self_949: Point;
    var other_815: AntiScalar;

    self_949 = self_948;
    other_815 = other_814;
    let _e4: Point = self_949;
    let _e6: AntiScalar = other_815;
    return Point((_e4.g0_ * vec3<f32>(_e6.g0_)));
}

fn point_anti_scalar_inner_anti_product(self_950: Point, other_816: AntiScalar) -> Point {
    var self_951: Point;
    var other_817: AntiScalar;

    self_951 = self_950;
    other_817 = other_816;
    let _e4: Point = self_951;
    let _e6: AntiScalar = other_817;
    return Point((_e4.g0_ * vec3<f32>(_e6.g0_)));
}

fn point_anti_scalar_left_contraction(self_952: Point, other_818: AntiScalar) -> Plane {
    var self_953: Point;
    var other_819: AntiScalar;

    self_953 = self_952;
    other_819 = other_818;
    let _e4: Point = self_953;
    let _e6: AntiScalar = other_819;
    return Plane(((_e4.g0_ * vec3<f32>(_e6.g0_)) * vec3<f32>(-(1.0))));
}

fn point_anti_scalar_right_anti_contraction(self_954: Point, other_820: AntiScalar) -> Point {
    var self_955: Point;
    var other_821: AntiScalar;

    self_955 = self_954;
    other_821 = other_820;
    let _e4: Point = self_955;
    let _e6: AntiScalar = other_821;
    return Point((_e4.g0_ * vec3<f32>(_e6.g0_)));
}

fn point_multi_vector_add(self_956: Point, other_822: MultiVector) -> MultiVector {
    var self_957: Point;
    var other_823: MultiVector;

    self_957 = self_956;
    other_823 = other_822;
    let _e4: Point = self_957;
    let _e14: MultiVector = other_823;
    let _e17: Point = self_957;
    let _e20: Point = self_957;
    let _e23: Point = self_957;
    let _e26: Point = self_957;
    let _e36: MultiVector = other_823;
    return MultiVector(((vec4<f32>(_e4.g0_.x) * vec4<f32>(0.0, 1.0, 0.0, 0.0)) + _e14.g0_), ((vec4<f32>(_e17.g0_.x, _e20.g0_.x, _e23.g0_.y, _e26.g0_.z) * vec4<f32>(0.0, 0.0, 1.0, 1.0)) + _e36.g1_));
}

fn point_multi_vector_sub(self_958: Point, other_824: MultiVector) -> MultiVector {
    var self_959: Point;
    var other_825: MultiVector;

    self_959 = self_958;
    other_825 = other_824;
    let _e4: Point = self_959;
    let _e14: MultiVector = other_825;
    let _e17: Point = self_959;
    let _e20: Point = self_959;
    let _e23: Point = self_959;
    let _e26: Point = self_959;
    let _e36: MultiVector = other_825;
    return MultiVector(((vec4<f32>(_e4.g0_.x) * vec4<f32>(0.0, 1.0, 0.0, 0.0)) - _e14.g0_), ((vec4<f32>(_e17.g0_.x, _e20.g0_.x, _e23.g0_.y, _e26.g0_.z) * vec4<f32>(0.0, 0.0, 1.0, 1.0)) - _e36.g1_));
}

fn point_multi_vector_geometric_product(self_960: Point, other_826: MultiVector) -> MultiVector {
    var self_961: Point;
    var other_827: MultiVector;

    self_961 = self_960;
    other_827 = other_826;
    let _e4: Point = self_961;
    let _e8: MultiVector = other_827;
    let _e20: Point = self_961;
    let _e24: MultiVector = other_827;
    let _e38: Point = self_961;
    let _e42: MultiVector = other_827;
    let _e56: Point = self_961;
    let _e60: MultiVector = other_827;
    let _e72: Point = self_961;
    let _e76: MultiVector = other_827;
    let _e88: Point = self_961;
    let _e92: MultiVector = other_827;
    return MultiVector(((((vec4<f32>(_e4.g0_.x) * _e8.g0_.yxwz) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0))) + ((vec4<f32>(_e20.g0_.y) * _e24.g1_.zwxy) * vec4<f32>(-(1.0), 1.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e38.g0_.z) * _e42.g1_.wzyx) * vec4<f32>(-(1.0), -(1.0), -(1.0), 1.0))), ((((vec4<f32>(_e56.g0_.x) * _e60.g1_.yxwz) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0)) + ((vec4<f32>(_e72.g0_.y) * _e76.g0_.zwxy) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e88.g0_.z) * _e92.g0_.wzyx) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0))));
}

fn point_multi_vector_geometric_anti_product(self_962: Point, other_828: MultiVector) -> MultiVector {
    var self_963: Point;
    var other_829: MultiVector;

    self_963 = self_962;
    other_829 = other_828;
    let _e4: Point = self_963;
    let _e8: MultiVector = other_829;
    let _e18: Point = self_963;
    let _e22: MultiVector = other_829;
    let _e27: Point = self_963;
    let _e31: MultiVector = other_829;
    let _e44: Point = self_963;
    let _e48: MultiVector = other_829;
    let _e58: Point = self_963;
    let _e62: MultiVector = other_829;
    let _e75: Point = self_963;
    let _e79: MultiVector = other_829;
    return MultiVector(((((vec4<f32>(_e4.g0_.x) * _e8.g1_) * vec4<f32>(1.0, 1.0, -(1.0), 1.0)) + (vec4<f32>(_e18.g0_.y) * _e22.g0_.wzyx)) + ((vec4<f32>(_e27.g0_.z) * _e31.g0_.zwxy) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))), ((((vec4<f32>(_e44.g0_.x) * _e48.g0_) * vec4<f32>(1.0, 1.0, -(1.0), 1.0)) + ((vec4<f32>(_e58.g0_.y) * _e62.g1_.wzyx) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + (vec4<f32>(_e75.g0_.z) * _e79.g1_.zwxy)));
}

fn point_multi_vector_scalar_product(self_964: Point, other_830: MultiVector) -> Scalar {
    var self_965: Point;
    var other_831: MultiVector;

    self_965 = self_964;
    other_831 = other_830;
    let _e5: Point = self_965;
    let _e8: MultiVector = other_831;
    let _e13: Point = self_965;
    let _e16: MultiVector = other_831;
    let _e21: Point = self_965;
    let _e24: MultiVector = other_831;
    return Scalar((((0.0 - (_e5.g0_.x * _e8.g0_.y)) - (_e13.g0_.y * _e16.g1_.z)) - (_e21.g0_.z * _e24.g1_.w)));
}

fn point_multi_vector_anti_scalar_product(self_966: Point, other_832: MultiVector) -> AntiScalar {
    var self_967: Point;
    var other_833: MultiVector;

    self_967 = self_966;
    other_833 = other_832;
    let _e4: Point = self_967;
    let _e7: MultiVector = other_833;
    let _e11: Point = self_967;
    let _e14: MultiVector = other_833;
    let _e19: Point = self_967;
    let _e22: MultiVector = other_833;
    return AntiScalar((((_e4.g0_.x * _e7.g0_.y) + (_e11.g0_.y * _e14.g1_.z)) + (_e19.g0_.z * _e22.g1_.w)));
}

fn point_rotor_add(self_968: Point, other_834: Rotor) -> Motor {
    var self_969: Point;
    var other_835: Rotor;

    self_969 = self_968;
    other_835 = other_834;
    let _e4: Point = self_969;
    let _e7: Point = self_969;
    let _e10: Point = self_969;
    let _e13: Point = self_969;
    let _e23: Rotor = other_835;
    let _e26: Rotor = other_835;
    let _e29: Rotor = other_835;
    let _e32: Rotor = other_835;
    return Motor(((vec4<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.y, _e13.g0_.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0)) + (vec4<f32>(_e23.g0_.x, _e26.g0_.y, _e29.g0_.x, _e32.g0_.x) * vec4<f32>(1.0, 1.0, 0.0, 0.0))));
}

fn point_rotor_sub(self_970: Point, other_836: Rotor) -> Motor {
    var self_971: Point;
    var other_837: Rotor;

    self_971 = self_970;
    other_837 = other_836;
    let _e4: Point = self_971;
    let _e7: Point = self_971;
    let _e10: Point = self_971;
    let _e13: Point = self_971;
    let _e23: Rotor = other_837;
    let _e26: Rotor = other_837;
    let _e29: Rotor = other_837;
    let _e32: Rotor = other_837;
    return Motor(((vec4<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.y, _e13.g0_.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0)) - (vec4<f32>(_e23.g0_.x, _e26.g0_.y, _e29.g0_.x, _e32.g0_.x) * vec4<f32>(1.0, 1.0, 0.0, 0.0))));
}

fn point_rotor_geometric_product(self_972: Point, other_838: Rotor) -> Motor {
    var self_973: Point;
    var other_839: Rotor;

    self_973 = self_972;
    other_839 = other_838;
    let _e4: Point = self_973;
    let _e8: Rotor = other_839;
    let _e11: Rotor = other_839;
    let _e14: Rotor = other_839;
    let _e17: Rotor = other_839;
    let _e28: Point = self_973;
    let _e31: Point = self_973;
    let _e34: Point = self_973;
    let _e37: Point = self_973;
    let _e41: Rotor = other_839;
    let _e44: Rotor = other_839;
    let _e47: Rotor = other_839;
    let _e50: Rotor = other_839;
    return Motor((((vec4<f32>(_e4.g0_.z) * vec4<f32>(_e8.g0_.y, _e11.g0_.y, _e14.g0_.y, _e17.g0_.x)) * vec4<f32>(0.0, 0.0, 1.0, 1.0)) + ((vec4<f32>(_e28.g0_.x, _e31.g0_.x, _e34.g0_.y, _e37.g0_.y) * vec4<f32>(_e41.g0_.y, _e44.g0_.x, _e47.g0_.x, _e50.g0_.y)) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))));
}

fn point_rotor_outer_product(self_974: Point, other_840: Rotor) -> Point {
    var self_975: Point;
    var other_841: Rotor;

    self_975 = self_974;
    other_841 = other_840;
    let _e4: Point = self_975;
    let _e6: Rotor = other_841;
    return Point((_e4.g0_ * vec3<f32>(_e6.g0_.x)));
}

fn point_rotor_inner_product(self_976: Point, other_842: Rotor) -> Motor {
    var self_977: Point;
    var other_843: Rotor;

    self_977 = self_976;
    other_843 = other_842;
    let _e4: Point = self_977;
    let _e7: Point = self_977;
    let _e10: Point = self_977;
    let _e13: Point = self_977;
    let _e17: Rotor = other_843;
    let _e20: Rotor = other_843;
    let _e23: Rotor = other_843;
    let _e26: Rotor = other_843;
    return Motor(((vec4<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.y, _e13.g0_.z) * vec4<f32>(_e17.g0_.y, _e20.g0_.x, _e23.g0_.x, _e26.g0_.x)) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0)));
}

fn point_rotor_geometric_anti_product(self_978: Point, other_844: Rotor) -> MotorDual {
    var self_979: Point;
    var other_845: Rotor;

    self_979 = self_978;
    other_845 = other_844;
    let _e4: Point = self_979;
    let _e8: Rotor = other_845;
    let _e11: Rotor = other_845;
    let _e14: Rotor = other_845;
    let _e17: Rotor = other_845;
    let _e29: Point = self_979;
    let _e32: Point = self_979;
    let _e35: Point = self_979;
    let _e38: Point = self_979;
    let _e42: Rotor = other_845;
    let _e45: Rotor = other_845;
    let _e48: Rotor = other_845;
    let _e51: Rotor = other_845;
    return MotorDual((((vec4<f32>(_e4.g0_.z) * vec4<f32>(_e8.g0_.y, _e11.g0_.y, _e14.g0_.y, _e17.g0_.x)) * vec4<f32>(0.0, 0.0, -(1.0), 1.0)) + (vec4<f32>(_e29.g0_.x, _e32.g0_.x, _e35.g0_.y, _e38.g0_.y) * vec4<f32>(_e42.g0_.y, _e45.g0_.x, _e48.g0_.x, _e51.g0_.y))));
}

fn point_rotor_inner_anti_product(self_980: Point, other_846: Rotor) -> MotorDual {
    var self_981: Point;
    var other_847: Rotor;

    self_981 = self_980;
    other_847 = other_846;
    let _e4: Point = self_981;
    let _e7: Point = self_981;
    let _e10: Point = self_981;
    let _e13: Point = self_981;
    let _e17: Rotor = other_847;
    let _e20: Rotor = other_847;
    let _e23: Rotor = other_847;
    let _e26: Rotor = other_847;
    return MotorDual((vec4<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.y, _e13.g0_.z) * vec4<f32>(_e17.g0_.y, _e20.g0_.x, _e23.g0_.x, _e26.g0_.x)));
}

fn point_rotor_left_contraction(self_982: Point, other_848: Rotor) -> Scalar {
    var self_983: Point;
    var other_849: Rotor;

    self_983 = self_982;
    other_849 = other_848;
    let _e5: Point = self_983;
    let _e8: Rotor = other_849;
    return Scalar((0.0 - (_e5.g0_.x * _e8.g0_.y)));
}

fn point_rotor_right_contraction(self_984: Point, other_850: Rotor) -> Motor {
    var self_985: Point;
    var other_851: Rotor;

    self_985 = self_984;
    other_851 = other_850;
    let _e4: Point = self_985;
    let _e7: Point = self_985;
    let _e10: Point = self_985;
    let _e13: Point = self_985;
    let _e17: Rotor = other_851;
    let _e20: Rotor = other_851;
    let _e23: Rotor = other_851;
    let _e26: Rotor = other_851;
    return Motor(((vec4<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.y, _e13.g0_.z) * vec4<f32>(_e17.g0_.y, _e20.g0_.x, _e23.g0_.x, _e26.g0_.x)) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0)));
}

fn point_rotor_left_anti_contraction(self_986: Point, other_852: Rotor) -> MotorDual {
    var self_987: Point;
    var other_853: Rotor;

    self_987 = self_986;
    other_853 = other_852;
    let _e4: Point = self_987;
    let _e7: Point = self_987;
    let _e10: Point = self_987;
    let _e13: Point = self_987;
    let _e17: Rotor = other_853;
    let _e20: Rotor = other_853;
    let _e23: Rotor = other_853;
    let _e26: Rotor = other_853;
    return MotorDual((vec4<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.y, _e13.g0_.z) * vec4<f32>(_e17.g0_.y, _e20.g0_.x, _e23.g0_.x, _e26.g0_.x)));
}

fn point_rotor_right_anti_contraction(self_988: Point, other_854: Rotor) -> AntiScalar {
    var self_989: Point;
    var other_855: Rotor;

    self_989 = self_988;
    other_855 = other_854;
    let _e4: Point = self_989;
    let _e7: Rotor = other_855;
    return AntiScalar((_e4.g0_.x * _e7.g0_.y));
}

fn point_rotor_scalar_product(self_990: Point, other_856: Rotor) -> Scalar {
    var self_991: Point;
    var other_857: Rotor;

    self_991 = self_990;
    other_857 = other_856;
    let _e5: Point = self_991;
    let _e8: Rotor = other_857;
    return Scalar((0.0 - (_e5.g0_.x * _e8.g0_.y)));
}

fn point_rotor_anti_scalar_product(self_992: Point, other_858: Rotor) -> AntiScalar {
    var self_993: Point;
    var other_859: Rotor;

    self_993 = self_992;
    other_859 = other_858;
    let _e4: Point = self_993;
    let _e7: Rotor = other_859;
    return AntiScalar((_e4.g0_.x * _e7.g0_.y));
}

fn point_point_add(self_994: Point, other_860: Point) -> Point {
    var self_995: Point;
    var other_861: Point;

    self_995 = self_994;
    other_861 = other_860;
    let _e4: Point = self_995;
    let _e6: Point = other_861;
    return Point((_e4.g0_ + _e6.g0_));
}

fn point_point_sub(self_996: Point, other_862: Point) -> Point {
    var self_997: Point;
    var other_863: Point;

    self_997 = self_996;
    other_863 = other_862;
    let _e4: Point = self_997;
    let _e6: Point = other_863;
    return Point((_e4.g0_ - _e6.g0_));
}

fn point_point_mul(self_998: Point, other_864: Point) -> Point {
    var self_999: Point;
    var other_865: Point;

    self_999 = self_998;
    other_865 = other_864;
    let _e4: Point = self_999;
    let _e6: Point = other_865;
    return Point((_e4.g0_ * _e6.g0_));
}

fn point_point_div(self_1000: Point, other_866: Point) -> Point {
    var self_1001: Point;
    var other_867: Point;

    self_1001 = self_1000;
    other_867 = other_866;
    let _e4: Point = self_1001;
    let _e7: Point = self_1001;
    let _e10: Point = self_1001;
    let _e19: Point = other_867;
    let _e22: Point = other_867;
    let _e25: Point = other_867;
    return Point((((vec3<f32>(_e4.g0_.x, _e7.g0_.y, _e10.g0_.z) * vec3<f32>(1.0, 1.0, 1.0)) / vec3<f32>(_e19.g0_.x, _e22.g0_.y, _e25.g0_.z)) * vec3<f32>(1.0, 1.0, 1.0)));
}

fn point_point_geometric_product(self_1002: Point, other_868: Point) -> Motor {
    var self_1003: Point;
    var other_869: Point;

    self_1003 = self_1002;
    other_869 = other_868;
    let _e4: Point = self_1003;
    let _e8: Point = other_869;
    let _e11: Point = other_869;
    let _e14: Point = other_869;
    let _e17: Point = other_869;
    let _e30: Point = self_1003;
    let _e34: Point = other_869;
    let _e37: Point = other_869;
    let _e40: Point = other_869;
    let _e43: Point = other_869;
    let _e57: Point = self_1003;
    let _e61: Point = other_869;
    let _e64: Point = other_869;
    let _e67: Point = other_869;
    let _e70: Point = other_869;
    return Motor(((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y, _e11.g0_.z, _e14.g0_.y, _e17.g0_.x)) * vec4<f32>(-(1.0), 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e30.g0_.z) * vec4<f32>(_e34.g0_.z, _e37.g0_.y, _e40.g0_.x, _e43.g0_.z)) * vec4<f32>(-(1.0), -(1.0), 1.0, 0.0))) + ((vec4<f32>(_e57.g0_.x) * vec4<f32>(_e61.g0_.x, _e64.g0_.x, _e67.g0_.z, _e70.g0_.y)) * vec4<f32>(-(1.0), 0.0, -(1.0), 1.0))));
}

fn point_point_regressive_product(self_1004: Point, other_870: Point) -> Plane {
    var self_1005: Point;
    var other_871: Point;

    self_1005 = self_1004;
    other_871 = other_870;
    let _e4: Point = self_1005;
    let _e8: Point = other_871;
    let _e18: Point = self_1005;
    let _e22: Point = other_871;
    let _e33: Point = self_1005;
    let _e37: Point = other_871;
    return Plane(((((vec3<f32>(_e4.g0_.y) * _e8.g0_.zzx) * vec3<f32>(-(1.0), 0.0, 1.0)) + ((vec3<f32>(_e18.g0_.z) * _e22.g0_.yxy) * vec3<f32>(1.0, -(1.0), 0.0))) + ((vec3<f32>(_e33.g0_.x) * _e37.g0_.xzy) * vec3<f32>(0.0, 1.0, -(1.0)))));
}

fn point_point_inner_product(self_1006: Point, other_872: Point) -> Scalar {
    var self_1007: Point;
    var other_873: Point;

    self_1007 = self_1006;
    other_873 = other_872;
    let _e5: Point = self_1007;
    let _e8: Point = other_873;
    let _e13: Point = self_1007;
    let _e16: Point = other_873;
    let _e21: Point = self_1007;
    let _e24: Point = other_873;
    return Scalar((((0.0 - (_e5.g0_.x * _e8.g0_.x)) - (_e13.g0_.y * _e16.g0_.y)) - (_e21.g0_.z * _e24.g0_.z)));
}

fn point_point_geometric_anti_product(self_1008: Point, other_874: Point) -> MotorDual {
    var self_1009: Point;
    var other_875: Point;

    self_1009 = self_1008;
    other_875 = other_874;
    let _e4: Point = self_1009;
    let _e8: Point = other_875;
    let _e11: Point = other_875;
    let _e14: Point = other_875;
    let _e17: Point = other_875;
    let _e29: Point = self_1009;
    let _e33: Point = other_875;
    let _e36: Point = other_875;
    let _e39: Point = other_875;
    let _e42: Point = other_875;
    let _e55: Point = self_1009;
    let _e59: Point = other_875;
    let _e62: Point = other_875;
    let _e65: Point = other_875;
    let _e68: Point = other_875;
    return MotorDual(((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y, _e11.g0_.z, _e14.g0_.y, _e17.g0_.x)) * vec4<f32>(1.0, -(1.0), 0.0, 1.0)) + ((vec4<f32>(_e29.g0_.z) * vec4<f32>(_e33.g0_.z, _e36.g0_.y, _e39.g0_.x, _e42.g0_.z)) * vec4<f32>(1.0, 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e55.g0_.x) * vec4<f32>(_e59.g0_.x, _e62.g0_.x, _e65.g0_.z, _e68.g0_.y)) * vec4<f32>(1.0, 0.0, 1.0, -(1.0)))));
}

fn point_point_inner_anti_product(self_1010: Point, other_876: Point) -> AntiScalar {
    var self_1011: Point;
    var other_877: Point;

    self_1011 = self_1010;
    other_877 = other_876;
    let _e4: Point = self_1011;
    let _e7: Point = other_877;
    let _e11: Point = self_1011;
    let _e14: Point = other_877;
    let _e19: Point = self_1011;
    let _e22: Point = other_877;
    return AntiScalar((((_e4.g0_.x * _e7.g0_.x) + (_e11.g0_.y * _e14.g0_.y)) + (_e19.g0_.z * _e22.g0_.z)));
}

fn point_point_left_contraction(self_1012: Point, other_878: Point) -> Scalar {
    var self_1013: Point;
    var other_879: Point;

    self_1013 = self_1012;
    other_879 = other_878;
    let _e5: Point = self_1013;
    let _e8: Point = other_879;
    let _e13: Point = self_1013;
    let _e16: Point = other_879;
    let _e21: Point = self_1013;
    let _e24: Point = other_879;
    return Scalar((((0.0 - (_e5.g0_.x * _e8.g0_.x)) - (_e13.g0_.y * _e16.g0_.y)) - (_e21.g0_.z * _e24.g0_.z)));
}

fn point_point_right_contraction(self_1014: Point, other_880: Point) -> Scalar {
    var self_1015: Point;
    var other_881: Point;

    self_1015 = self_1014;
    other_881 = other_880;
    let _e5: Point = self_1015;
    let _e8: Point = other_881;
    let _e13: Point = self_1015;
    let _e16: Point = other_881;
    let _e21: Point = self_1015;
    let _e24: Point = other_881;
    return Scalar((((0.0 - (_e5.g0_.x * _e8.g0_.x)) - (_e13.g0_.y * _e16.g0_.y)) - (_e21.g0_.z * _e24.g0_.z)));
}

fn point_point_left_anti_contraction(self_1016: Point, other_882: Point) -> AntiScalar {
    var self_1017: Point;
    var other_883: Point;

    self_1017 = self_1016;
    other_883 = other_882;
    let _e4: Point = self_1017;
    let _e7: Point = other_883;
    let _e11: Point = self_1017;
    let _e14: Point = other_883;
    let _e19: Point = self_1017;
    let _e22: Point = other_883;
    return AntiScalar((((_e4.g0_.x * _e7.g0_.x) + (_e11.g0_.y * _e14.g0_.y)) + (_e19.g0_.z * _e22.g0_.z)));
}

fn point_point_right_anti_contraction(self_1018: Point, other_884: Point) -> AntiScalar {
    var self_1019: Point;
    var other_885: Point;

    self_1019 = self_1018;
    other_885 = other_884;
    let _e4: Point = self_1019;
    let _e7: Point = other_885;
    let _e11: Point = self_1019;
    let _e14: Point = other_885;
    let _e19: Point = self_1019;
    let _e22: Point = other_885;
    return AntiScalar((((_e4.g0_.x * _e7.g0_.x) + (_e11.g0_.y * _e14.g0_.y)) + (_e19.g0_.z * _e22.g0_.z)));
}

fn point_point_scalar_product(self_1020: Point, other_886: Point) -> Scalar {
    var self_1021: Point;
    var other_887: Point;

    self_1021 = self_1020;
    other_887 = other_886;
    let _e5: Point = self_1021;
    let _e8: Point = other_887;
    let _e13: Point = self_1021;
    let _e16: Point = other_887;
    let _e21: Point = self_1021;
    let _e24: Point = other_887;
    return Scalar((((0.0 - (_e5.g0_.x * _e8.g0_.x)) - (_e13.g0_.y * _e16.g0_.y)) - (_e21.g0_.z * _e24.g0_.z)));
}

fn point_point_anti_scalar_product(self_1022: Point, other_888: Point) -> AntiScalar {
    var self_1023: Point;
    var other_889: Point;

    self_1023 = self_1022;
    other_889 = other_888;
    let _e4: Point = self_1023;
    let _e7: Point = other_889;
    let _e11: Point = self_1023;
    let _e14: Point = other_889;
    let _e19: Point = self_1023;
    let _e22: Point = other_889;
    return AntiScalar((((_e4.g0_.x * _e7.g0_.x) + (_e11.g0_.y * _e14.g0_.y)) + (_e19.g0_.z * _e22.g0_.z)));
}

fn point_ideal_point_into(self_1024: Point) -> IdealPoint {
    var self_1025: Point;

    self_1025 = self_1024;
    let _e2: Point = self_1025;
    let _e5: Point = self_1025;
    return IdealPoint(vec2<f32>(_e2.g0_.y, _e5.g0_.z));
}

fn point_ideal_point_add(self_1026: Point, other_890: IdealPoint) -> Point {
    var self_1027: Point;
    var other_891: IdealPoint;

    self_1027 = self_1026;
    other_891 = other_890;
    let _e4: Point = self_1027;
    let _e6: IdealPoint = other_891;
    let _e9: IdealPoint = other_891;
    let _e12: IdealPoint = other_891;
    return Point((_e4.g0_ + (vec3<f32>(_e6.g0_.x, _e9.g0_.x, _e12.g0_.y) * vec3<f32>(0.0, 1.0, 1.0))));
}

fn point_ideal_point_sub(self_1028: Point, other_892: IdealPoint) -> Point {
    var self_1029: Point;
    var other_893: IdealPoint;

    self_1029 = self_1028;
    other_893 = other_892;
    let _e4: Point = self_1029;
    let _e6: IdealPoint = other_893;
    let _e9: IdealPoint = other_893;
    let _e12: IdealPoint = other_893;
    return Point((_e4.g0_ - (vec3<f32>(_e6.g0_.x, _e9.g0_.x, _e12.g0_.y) * vec3<f32>(0.0, 1.0, 1.0))));
}

fn point_ideal_point_geometric_product(self_1030: Point, other_894: IdealPoint) -> Motor {
    var self_1031: Point;
    var other_895: IdealPoint;

    self_1031 = self_1030;
    other_895 = other_894;
    let _e4: Point = self_1031;
    let _e8: IdealPoint = other_895;
    let _e11: IdealPoint = other_895;
    let _e14: IdealPoint = other_895;
    let _e17: IdealPoint = other_895;
    let _e30: Point = self_1031;
    let _e33: Point = self_1031;
    let _e36: Point = self_1031;
    let _e39: Point = self_1031;
    let _e43: IdealPoint = other_895;
    let _e46: IdealPoint = other_895;
    let _e49: IdealPoint = other_895;
    let _e52: IdealPoint = other_895;
    return Motor((((vec4<f32>(_e4.g0_.z) * vec4<f32>(_e8.g0_.y, _e11.g0_.x, _e14.g0_.y, _e17.g0_.y)) * vec4<f32>(-(1.0), -(1.0), 0.0, 0.0)) + ((vec4<f32>(_e30.g0_.y, _e33.g0_.y, _e36.g0_.x, _e39.g0_.x) * vec4<f32>(_e43.g0_.x, _e46.g0_.y, _e49.g0_.y, _e52.g0_.x)) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))));
}

fn point_ideal_point_regressive_product(self_1032: Point, other_896: IdealPoint) -> Plane {
    var self_1033: Point;
    var other_897: IdealPoint;

    self_1033 = self_1032;
    other_897 = other_896;
    let _e4: Point = self_1033;
    let _e8: IdealPoint = other_897;
    let _e18: Point = self_1033;
    let _e21: IdealPoint = other_897;
    let _e24: IdealPoint = other_897;
    let _e27: IdealPoint = other_897;
    return Plane((((vec3<f32>(_e4.g0_.z) * vec3<f32>(_e8.g0_.x)) * vec3<f32>(1.0, 0.0, 0.0)) + ((_e18.g0_.yxx * vec3<f32>(_e21.g0_.y, _e24.g0_.y, _e27.g0_.x)) * vec3<f32>(-(1.0), 1.0, -(1.0)))));
}

fn point_ideal_point_inner_product(self_1034: Point, other_898: IdealPoint) -> Scalar {
    var self_1035: Point;
    var other_899: IdealPoint;

    self_1035 = self_1034;
    other_899 = other_898;
    let _e5: Point = self_1035;
    let _e8: IdealPoint = other_899;
    let _e13: Point = self_1035;
    let _e16: IdealPoint = other_899;
    return Scalar(((0.0 - (_e5.g0_.y * _e8.g0_.x)) - (_e13.g0_.z * _e16.g0_.y)));
}

fn point_ideal_point_geometric_anti_product(self_1036: Point, other_900: IdealPoint) -> MotorDual {
    var self_1037: Point;
    var other_901: IdealPoint;

    self_1037 = self_1036;
    other_901 = other_900;
    let _e4: Point = self_1037;
    let _e8: IdealPoint = other_901;
    let _e11: IdealPoint = other_901;
    let _e14: IdealPoint = other_901;
    let _e17: IdealPoint = other_901;
    let _e28: Point = self_1037;
    let _e31: Point = self_1037;
    let _e34: Point = self_1037;
    let _e37: Point = self_1037;
    let _e41: IdealPoint = other_901;
    let _e44: IdealPoint = other_901;
    let _e47: IdealPoint = other_901;
    let _e50: IdealPoint = other_901;
    return MotorDual((((vec4<f32>(_e4.g0_.z) * vec4<f32>(_e8.g0_.y, _e11.g0_.x, _e14.g0_.y, _e17.g0_.y)) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) + ((vec4<f32>(_e28.g0_.y, _e31.g0_.y, _e34.g0_.x, _e37.g0_.x) * vec4<f32>(_e41.g0_.x, _e44.g0_.y, _e47.g0_.y, _e50.g0_.x)) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))));
}

fn point_ideal_point_inner_anti_product(self_1038: Point, other_902: IdealPoint) -> AntiScalar {
    var self_1039: Point;
    var other_903: IdealPoint;

    self_1039 = self_1038;
    other_903 = other_902;
    let _e4: Point = self_1039;
    let _e7: IdealPoint = other_903;
    let _e11: Point = self_1039;
    let _e14: IdealPoint = other_903;
    return AntiScalar(((_e4.g0_.y * _e7.g0_.x) + (_e11.g0_.z * _e14.g0_.y)));
}

fn point_ideal_point_left_contraction(self_1040: Point, other_904: IdealPoint) -> Scalar {
    var self_1041: Point;
    var other_905: IdealPoint;

    self_1041 = self_1040;
    other_905 = other_904;
    let _e5: Point = self_1041;
    let _e8: IdealPoint = other_905;
    let _e13: Point = self_1041;
    let _e16: IdealPoint = other_905;
    return Scalar(((0.0 - (_e5.g0_.y * _e8.g0_.x)) - (_e13.g0_.z * _e16.g0_.y)));
}

fn point_ideal_point_right_contraction(self_1042: Point, other_906: IdealPoint) -> Scalar {
    var self_1043: Point;
    var other_907: IdealPoint;

    self_1043 = self_1042;
    other_907 = other_906;
    let _e5: Point = self_1043;
    let _e8: IdealPoint = other_907;
    let _e13: Point = self_1043;
    let _e16: IdealPoint = other_907;
    return Scalar(((0.0 - (_e5.g0_.y * _e8.g0_.x)) - (_e13.g0_.z * _e16.g0_.y)));
}

fn point_ideal_point_left_anti_contraction(self_1044: Point, other_908: IdealPoint) -> AntiScalar {
    var self_1045: Point;
    var other_909: IdealPoint;

    self_1045 = self_1044;
    other_909 = other_908;
    let _e4: Point = self_1045;
    let _e7: IdealPoint = other_909;
    let _e11: Point = self_1045;
    let _e14: IdealPoint = other_909;
    return AntiScalar(((_e4.g0_.y * _e7.g0_.x) + (_e11.g0_.z * _e14.g0_.y)));
}

fn point_ideal_point_right_anti_contraction(self_1046: Point, other_910: IdealPoint) -> AntiScalar {
    var self_1047: Point;
    var other_911: IdealPoint;

    self_1047 = self_1046;
    other_911 = other_910;
    let _e4: Point = self_1047;
    let _e7: IdealPoint = other_911;
    let _e11: Point = self_1047;
    let _e14: IdealPoint = other_911;
    return AntiScalar(((_e4.g0_.y * _e7.g0_.x) + (_e11.g0_.z * _e14.g0_.y)));
}

fn point_ideal_point_scalar_product(self_1048: Point, other_912: IdealPoint) -> Scalar {
    var self_1049: Point;
    var other_913: IdealPoint;

    self_1049 = self_1048;
    other_913 = other_912;
    let _e5: Point = self_1049;
    let _e8: IdealPoint = other_913;
    let _e13: Point = self_1049;
    let _e16: IdealPoint = other_913;
    return Scalar(((0.0 - (_e5.g0_.y * _e8.g0_.x)) - (_e13.g0_.z * _e16.g0_.y)));
}

fn point_ideal_point_anti_scalar_product(self_1050: Point, other_914: IdealPoint) -> AntiScalar {
    var self_1051: Point;
    var other_915: IdealPoint;

    self_1051 = self_1050;
    other_915 = other_914;
    let _e4: Point = self_1051;
    let _e7: IdealPoint = other_915;
    let _e11: Point = self_1051;
    let _e14: IdealPoint = other_915;
    return AntiScalar(((_e4.g0_.y * _e7.g0_.x) + (_e11.g0_.z * _e14.g0_.y)));
}

fn point_plane_geometric_product(self_1052: Point, other_916: Plane) -> MotorDual {
    var self_1053: Point;
    var other_917: Plane;

    self_1053 = self_1052;
    other_917 = other_916;
    let _e4: Point = self_1053;
    let _e8: Plane = other_917;
    let _e11: Plane = other_917;
    let _e14: Plane = other_917;
    let _e17: Plane = other_917;
    let _e29: Point = self_1053;
    let _e33: Plane = other_917;
    let _e36: Plane = other_917;
    let _e39: Plane = other_917;
    let _e42: Plane = other_917;
    let _e55: Point = self_1053;
    let _e59: Plane = other_917;
    let _e62: Plane = other_917;
    let _e65: Plane = other_917;
    let _e68: Plane = other_917;
    return MotorDual(((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y, _e11.g0_.z, _e14.g0_.y, _e17.g0_.x)) * vec4<f32>(1.0, 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e29.g0_.z) * vec4<f32>(_e33.g0_.z, _e36.g0_.y, _e39.g0_.x, _e42.g0_.z)) * vec4<f32>(1.0, -(1.0), 1.0, 0.0))) + ((vec4<f32>(_e55.g0_.x) * vec4<f32>(_e59.g0_.x, _e62.g0_.x, _e65.g0_.z, _e68.g0_.y)) * vec4<f32>(1.0, 0.0, -(1.0), 1.0))));
}

fn point_plane_regressive_product(self_1054: Point, other_918: Plane) -> Scalar {
    var self_1055: Point;
    var other_919: Plane;

    self_1055 = self_1054;
    other_919 = other_918;
    let _e4: Point = self_1055;
    let _e7: Plane = other_919;
    let _e11: Point = self_1055;
    let _e14: Plane = other_919;
    let _e19: Point = self_1055;
    let _e22: Plane = other_919;
    return Scalar((((_e4.g0_.x * _e7.g0_.x) + (_e11.g0_.y * _e14.g0_.y)) + (_e19.g0_.z * _e22.g0_.z)));
}

fn point_plane_outer_product(self_1056: Point, other_920: Plane) -> AntiScalar {
    var self_1057: Point;
    var other_921: Plane;

    self_1057 = self_1056;
    other_921 = other_920;
    let _e4: Point = self_1057;
    let _e7: Plane = other_921;
    let _e11: Point = self_1057;
    let _e14: Plane = other_921;
    let _e19: Point = self_1057;
    let _e22: Plane = other_921;
    return AntiScalar((((_e4.g0_.x * _e7.g0_.x) + (_e11.g0_.y * _e14.g0_.y)) + (_e19.g0_.z * _e22.g0_.z)));
}

fn point_plane_inner_product(self_1058: Point, other_922: Plane) -> Plane {
    var self_1059: Point;
    var other_923: Plane;

    self_1059 = self_1058;
    other_923 = other_922;
    let _e4: Point = self_1059;
    let _e8: Plane = other_923;
    let _e18: Point = self_1059;
    let _e22: Plane = other_923;
    let _e33: Point = self_1059;
    let _e37: Plane = other_923;
    return Plane(((((vec3<f32>(_e4.g0_.y) * _e8.g0_.zzx) * vec3<f32>(1.0, 0.0, -(1.0))) + ((vec3<f32>(_e18.g0_.z) * _e22.g0_.yxy) * vec3<f32>(-(1.0), 1.0, 0.0))) + ((vec3<f32>(_e33.g0_.x) * _e37.g0_.xzy) * vec3<f32>(0.0, -(1.0), 1.0))));
}

fn point_plane_geometric_anti_product(self_1060: Point, other_924: Plane) -> Motor {
    var self_1061: Point;
    var other_925: Plane;

    self_1061 = self_1060;
    other_925 = other_924;
    let _e4: Point = self_1061;
    let _e8: Plane = other_925;
    let _e11: Plane = other_925;
    let _e14: Plane = other_925;
    let _e17: Plane = other_925;
    let _e29: Point = self_1061;
    let _e33: Plane = other_925;
    let _e36: Plane = other_925;
    let _e39: Plane = other_925;
    let _e42: Plane = other_925;
    let _e55: Point = self_1061;
    let _e59: Plane = other_925;
    let _e62: Plane = other_925;
    let _e65: Plane = other_925;
    let _e68: Plane = other_925;
    return Motor(((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y, _e11.g0_.z, _e14.g0_.y, _e17.g0_.x)) * vec4<f32>(1.0, 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e29.g0_.z) * vec4<f32>(_e33.g0_.z, _e36.g0_.y, _e39.g0_.x, _e42.g0_.z)) * vec4<f32>(1.0, -(1.0), 1.0, 0.0))) + ((vec4<f32>(_e55.g0_.x) * vec4<f32>(_e59.g0_.x, _e62.g0_.x, _e65.g0_.z, _e68.g0_.y)) * vec4<f32>(1.0, 0.0, -(1.0), 1.0))));
}

fn point_plane_inner_anti_product(self_1062: Point, other_926: Plane) -> Point {
    var self_1063: Point;
    var other_927: Plane;

    self_1063 = self_1062;
    other_927 = other_926;
    let _e4: Point = self_1063;
    let _e8: Plane = other_927;
    let _e18: Point = self_1063;
    let _e22: Plane = other_927;
    let _e33: Point = self_1063;
    let _e37: Plane = other_927;
    return Point(((((vec3<f32>(_e4.g0_.y) * _e8.g0_.zzx) * vec3<f32>(1.0, 0.0, -(1.0))) + ((vec3<f32>(_e18.g0_.z) * _e22.g0_.yxy) * vec3<f32>(-(1.0), 1.0, 0.0))) + ((vec3<f32>(_e33.g0_.x) * _e37.g0_.xzy) * vec3<f32>(0.0, -(1.0), 1.0))));
}

fn point_plane_right_contraction(self_1064: Point, other_928: Plane) -> Plane {
    var self_1065: Point;
    var other_929: Plane;

    self_1065 = self_1064;
    other_929 = other_928;
    let _e4: Point = self_1065;
    let _e8: Plane = other_929;
    let _e18: Point = self_1065;
    let _e22: Plane = other_929;
    let _e33: Point = self_1065;
    let _e37: Plane = other_929;
    return Plane(((((vec3<f32>(_e4.g0_.y) * _e8.g0_.zzx) * vec3<f32>(1.0, 0.0, -(1.0))) + ((vec3<f32>(_e18.g0_.z) * _e22.g0_.yxy) * vec3<f32>(-(1.0), 1.0, 0.0))) + ((vec3<f32>(_e33.g0_.x) * _e37.g0_.xzy) * vec3<f32>(0.0, -(1.0), 1.0))));
}

fn point_plane_left_anti_contraction(self_1066: Point, other_930: Plane) -> Point {
    var self_1067: Point;
    var other_931: Plane;

    self_1067 = self_1066;
    other_931 = other_930;
    let _e4: Point = self_1067;
    let _e8: Plane = other_931;
    let _e18: Point = self_1067;
    let _e22: Plane = other_931;
    let _e33: Point = self_1067;
    let _e37: Plane = other_931;
    return Point(((((vec3<f32>(_e4.g0_.y) * _e8.g0_.zzx) * vec3<f32>(1.0, 0.0, -(1.0))) + ((vec3<f32>(_e18.g0_.z) * _e22.g0_.yxy) * vec3<f32>(-(1.0), 1.0, 0.0))) + ((vec3<f32>(_e33.g0_.x) * _e37.g0_.xzy) * vec3<f32>(0.0, -(1.0), 1.0))));
}

fn point_translator_add(self_1068: Point, other_932: Translator) -> Motor {
    var self_1069: Point;
    var other_933: Translator;

    self_1069 = self_1068;
    other_933 = other_932;
    let _e4: Point = self_1069;
    let _e7: Point = self_1069;
    let _e10: Point = self_1069;
    let _e13: Point = self_1069;
    let _e23: Translator = other_933;
    let _e26: Translator = other_933;
    let _e29: Translator = other_933;
    let _e32: Translator = other_933;
    return Motor(((vec4<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.y, _e13.g0_.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0)) + (vec4<f32>(_e23.g0_.x, _e26.g0_.x, _e29.g0_.y, _e32.g0_.z) * vec4<f32>(1.0, 0.0, 1.0, 1.0))));
}

fn point_translator_sub(self_1070: Point, other_934: Translator) -> Motor {
    var self_1071: Point;
    var other_935: Translator;

    self_1071 = self_1070;
    other_935 = other_934;
    let _e4: Point = self_1071;
    let _e7: Point = self_1071;
    let _e10: Point = self_1071;
    let _e13: Point = self_1071;
    let _e23: Translator = other_935;
    let _e26: Translator = other_935;
    let _e29: Translator = other_935;
    let _e32: Translator = other_935;
    return Motor(((vec4<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.y, _e13.g0_.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0)) - (vec4<f32>(_e23.g0_.x, _e26.g0_.x, _e29.g0_.y, _e32.g0_.z) * vec4<f32>(1.0, 0.0, 1.0, 1.0))));
}

fn point_translator_geometric_product(self_1072: Point, other_936: Translator) -> Motor {
    var self_1073: Point;
    var other_937: Translator;

    self_1073 = self_1072;
    other_937 = other_936;
    let _e4: Point = self_1073;
    let _e8: Translator = other_937;
    let _e11: Translator = other_937;
    let _e14: Translator = other_937;
    let _e17: Translator = other_937;
    let _e29: Point = self_1073;
    let _e33: Translator = other_937;
    let _e36: Translator = other_937;
    let _e39: Translator = other_937;
    let _e42: Translator = other_937;
    let _e56: Point = self_1073;
    let _e60: Translator = other_937;
    let _e63: Translator = other_937;
    let _e66: Translator = other_937;
    let _e69: Translator = other_937;
    return Motor(((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y, _e11.g0_.z, _e14.g0_.x, _e17.g0_.y)) * vec4<f32>(-(1.0), 1.0, 1.0, 0.0)) + ((vec4<f32>(_e29.g0_.z) * vec4<f32>(_e33.g0_.z, _e36.g0_.y, _e39.g0_.z, _e42.g0_.x)) * vec4<f32>(-(1.0), -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e56.g0_.x) * vec4<f32>(_e60.g0_.x, _e63.g0_.x, _e66.g0_.z, _e69.g0_.y)) * vec4<f32>(0.0, 1.0, -(1.0), 1.0))));
}

fn point_translator_regressive_product(self_1074: Point, other_938: Translator) -> Plane {
    var self_1075: Point;
    var other_939: Translator;

    self_1075 = self_1074;
    other_939 = other_938;
    let _e4: Point = self_1075;
    let _e8: Translator = other_939;
    let _e18: Point = self_1075;
    let _e21: Translator = other_939;
    return Plane((((vec3<f32>(_e4.g0_.z) * vec3<f32>(_e8.g0_.y)) * vec3<f32>(1.0, 0.0, 0.0)) + ((_e18.g0_.yxx * _e21.g0_.zzy) * vec3<f32>(-(1.0), 1.0, -(1.0)))));
}

fn point_translator_outer_product(self_1076: Point, other_940: Translator) -> Point {
    var self_1077: Point;
    var other_941: Translator;

    self_1077 = self_1076;
    other_941 = other_940;
    let _e4: Point = self_1077;
    let _e6: Translator = other_941;
    return Point((_e4.g0_ * vec3<f32>(_e6.g0_.x)));
}

fn point_translator_inner_product(self_1078: Point, other_942: Translator) -> Motor {
    var self_1079: Point;
    var other_943: Translator;

    self_1079 = self_1078;
    other_943 = other_942;
    let _e4: Point = self_1079;
    let _e8: Translator = other_943;
    let _e11: Translator = other_943;
    let _e14: Translator = other_943;
    let _e17: Translator = other_943;
    let _e29: Point = self_1079;
    let _e32: Point = self_1079;
    let _e35: Point = self_1079;
    let _e38: Point = self_1079;
    let _e42: Translator = other_943;
    let _e45: Translator = other_943;
    let _e48: Translator = other_943;
    let _e51: Translator = other_943;
    return Motor((((vec4<f32>(_e4.g0_.z) * vec4<f32>(_e8.g0_.z, _e11.g0_.z, _e14.g0_.z, _e17.g0_.x)) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0)) + ((vec4<f32>(_e29.g0_.y, _e32.g0_.x, _e35.g0_.y, _e38.g0_.x) * vec4<f32>(_e42.g0_.y, _e45.g0_.x, _e48.g0_.x, _e51.g0_.x)) * vec4<f32>(-(1.0), 1.0, 1.0, 0.0))));
}

fn point_translator_geometric_anti_product(self_1080: Point, other_944: Translator) -> MotorDual {
    var self_1081: Point;
    var other_945: Translator;

    self_1081 = self_1080;
    other_945 = other_944;
    let _e4: Point = self_1081;
    let _e8: Translator = other_945;
    let _e11: Translator = other_945;
    let _e14: Translator = other_945;
    let _e17: Translator = other_945;
    let _e29: Point = self_1081;
    let _e33: Translator = other_945;
    let _e36: Translator = other_945;
    let _e39: Translator = other_945;
    let _e42: Translator = other_945;
    let _e54: Point = self_1081;
    let _e58: Translator = other_945;
    let _e61: Translator = other_945;
    let _e64: Translator = other_945;
    let _e67: Translator = other_945;
    return MotorDual(((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y, _e11.g0_.z, _e14.g0_.x, _e17.g0_.y)) * vec4<f32>(1.0, -(1.0), 1.0, 0.0)) + ((vec4<f32>(_e29.g0_.z) * vec4<f32>(_e33.g0_.z, _e36.g0_.y, _e39.g0_.z, _e42.g0_.x)) * vec4<f32>(1.0, 1.0, 0.0, 1.0))) + ((vec4<f32>(_e54.g0_.x) * vec4<f32>(_e58.g0_.x, _e61.g0_.x, _e64.g0_.z, _e67.g0_.y)) * vec4<f32>(0.0, 1.0, 1.0, -(1.0)))));
}

fn point_translator_inner_anti_product(self_1082: Point, other_946: Translator) -> MotorDual {
    var self_1083: Point;
    var other_947: Translator;

    self_1083 = self_1082;
    other_947 = other_946;
    let _e4: Point = self_1083;
    let _e8: Translator = other_947;
    let _e11: Translator = other_947;
    let _e14: Translator = other_947;
    let _e17: Translator = other_947;
    let _e28: Point = self_1083;
    let _e31: Point = self_1083;
    let _e34: Point = self_1083;
    let _e37: Point = self_1083;
    let _e41: Translator = other_947;
    let _e44: Translator = other_947;
    let _e47: Translator = other_947;
    let _e50: Translator = other_947;
    return MotorDual((((vec4<f32>(_e4.g0_.z) * vec4<f32>(_e8.g0_.z, _e11.g0_.z, _e14.g0_.z, _e17.g0_.x)) * vec4<f32>(1.0, 0.0, 0.0, 1.0)) + ((vec4<f32>(_e28.g0_.y, _e31.g0_.x, _e34.g0_.y, _e37.g0_.x) * vec4<f32>(_e41.g0_.y, _e44.g0_.x, _e47.g0_.x, _e50.g0_.x)) * vec4<f32>(1.0, 1.0, 1.0, 0.0))));
}

fn point_translator_left_contraction(self_1084: Point, other_948: Translator) -> Scalar {
    var self_1085: Point;
    var other_949: Translator;

    self_1085 = self_1084;
    other_949 = other_948;
    let _e5: Point = self_1085;
    let _e8: Translator = other_949;
    let _e13: Point = self_1085;
    let _e16: Translator = other_949;
    return Scalar(((0.0 - (_e5.g0_.y * _e8.g0_.y)) - (_e13.g0_.z * _e16.g0_.z)));
}

fn point_translator_right_contraction(self_1086: Point, other_950: Translator) -> Motor {
    var self_1087: Point;
    var other_951: Translator;

    self_1087 = self_1086;
    other_951 = other_950;
    let _e4: Point = self_1087;
    let _e8: Translator = other_951;
    let _e11: Translator = other_951;
    let _e14: Translator = other_951;
    let _e17: Translator = other_951;
    let _e29: Point = self_1087;
    let _e32: Point = self_1087;
    let _e35: Point = self_1087;
    let _e38: Point = self_1087;
    let _e42: Translator = other_951;
    let _e45: Translator = other_951;
    let _e48: Translator = other_951;
    let _e51: Translator = other_951;
    return Motor((((vec4<f32>(_e4.g0_.z) * vec4<f32>(_e8.g0_.z, _e11.g0_.z, _e14.g0_.z, _e17.g0_.x)) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0)) + ((vec4<f32>(_e29.g0_.y, _e32.g0_.x, _e35.g0_.y, _e38.g0_.x) * vec4<f32>(_e42.g0_.y, _e45.g0_.x, _e48.g0_.x, _e51.g0_.x)) * vec4<f32>(-(1.0), 1.0, 1.0, 0.0))));
}

fn point_translator_left_anti_contraction(self_1088: Point, other_952: Translator) -> MotorDual {
    var self_1089: Point;
    var other_953: Translator;

    self_1089 = self_1088;
    other_953 = other_952;
    let _e4: Point = self_1089;
    let _e8: Translator = other_953;
    let _e11: Translator = other_953;
    let _e14: Translator = other_953;
    let _e17: Translator = other_953;
    let _e28: Point = self_1089;
    let _e31: Point = self_1089;
    let _e34: Point = self_1089;
    let _e37: Point = self_1089;
    let _e41: Translator = other_953;
    let _e44: Translator = other_953;
    let _e47: Translator = other_953;
    let _e50: Translator = other_953;
    return MotorDual((((vec4<f32>(_e4.g0_.z) * vec4<f32>(_e8.g0_.z, _e11.g0_.z, _e14.g0_.z, _e17.g0_.x)) * vec4<f32>(1.0, 0.0, 0.0, 1.0)) + ((vec4<f32>(_e28.g0_.y, _e31.g0_.x, _e34.g0_.y, _e37.g0_.x) * vec4<f32>(_e41.g0_.y, _e44.g0_.x, _e47.g0_.x, _e50.g0_.x)) * vec4<f32>(1.0, 1.0, 1.0, 0.0))));
}

fn point_translator_right_anti_contraction(self_1090: Point, other_954: Translator) -> AntiScalar {
    var self_1091: Point;
    var other_955: Translator;

    self_1091 = self_1090;
    other_955 = other_954;
    let _e4: Point = self_1091;
    let _e7: Translator = other_955;
    let _e11: Point = self_1091;
    let _e14: Translator = other_955;
    return AntiScalar(((_e4.g0_.y * _e7.g0_.y) + (_e11.g0_.z * _e14.g0_.z)));
}

fn point_translator_scalar_product(self_1092: Point, other_956: Translator) -> Scalar {
    var self_1093: Point;
    var other_957: Translator;

    self_1093 = self_1092;
    other_957 = other_956;
    let _e5: Point = self_1093;
    let _e8: Translator = other_957;
    let _e13: Point = self_1093;
    let _e16: Translator = other_957;
    return Scalar(((0.0 - (_e5.g0_.y * _e8.g0_.y)) - (_e13.g0_.z * _e16.g0_.z)));
}

fn point_translator_anti_scalar_product(self_1094: Point, other_958: Translator) -> AntiScalar {
    var self_1095: Point;
    var other_959: Translator;

    self_1095 = self_1094;
    other_959 = other_958;
    let _e4: Point = self_1095;
    let _e7: Translator = other_959;
    let _e11: Point = self_1095;
    let _e14: Translator = other_959;
    return AntiScalar(((_e4.g0_.y * _e7.g0_.y) + (_e11.g0_.z * _e14.g0_.z)));
}

fn point_motor_add(self_1096: Point, other_960: Motor) -> Motor {
    var self_1097: Point;
    var other_961: Motor;

    self_1097 = self_1096;
    other_961 = other_960;
    let _e4: Point = self_1097;
    let _e7: Point = self_1097;
    let _e10: Point = self_1097;
    let _e13: Point = self_1097;
    let _e23: Motor = other_961;
    return Motor(((vec4<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.y, _e13.g0_.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0)) + _e23.g0_));
}

fn point_motor_sub(self_1098: Point, other_962: Motor) -> Motor {
    var self_1099: Point;
    var other_963: Motor;

    self_1099 = self_1098;
    other_963 = other_962;
    let _e4: Point = self_1099;
    let _e7: Point = self_1099;
    let _e10: Point = self_1099;
    let _e13: Point = self_1099;
    let _e23: Motor = other_963;
    return Motor(((vec4<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.y, _e13.g0_.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0)) - _e23.g0_));
}

fn point_motor_geometric_product(self_1100: Point, other_964: Motor) -> Motor {
    var self_1101: Point;
    var other_965: Motor;

    self_1101 = self_1100;
    other_965 = other_964;
    let _e4: Point = self_1101;
    let _e8: Motor = other_965;
    let _e20: Point = self_1101;
    let _e24: Motor = other_965;
    let _e37: Point = self_1101;
    let _e41: Motor = other_965;
    return Motor(((((vec4<f32>(_e4.g0_.x) * _e8.g0_.yxwz) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0)) + ((vec4<f32>(_e20.g0_.y) * _e24.g0_.zwxy) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e37.g0_.z) * _e41.g0_.wzyx) * vec4<f32>(-(1.0), -(1.0), 1.0, 1.0))));
}

fn point_motor_regressive_product(self_1102: Point, other_966: Motor) -> Plane {
    var self_1103: Point;
    var other_967: Motor;

    self_1103 = self_1102;
    other_967 = other_966;
    let _e4: Point = self_1103;
    let _e8: Motor = other_967;
    let _e11: Motor = other_967;
    let _e14: Motor = other_967;
    let _e25: Point = self_1103;
    let _e29: Motor = other_967;
    let _e32: Motor = other_967;
    let _e35: Motor = other_967;
    let _e47: Point = self_1103;
    let _e51: Motor = other_967;
    let _e54: Motor = other_967;
    let _e57: Motor = other_967;
    return Plane(((((vec3<f32>(_e4.g0_.y) * vec3<f32>(_e8.g0_.w, _e11.g0_.w, _e14.g0_.y)) * vec3<f32>(-(1.0), 0.0, 1.0)) + ((vec3<f32>(_e25.g0_.z) * vec3<f32>(_e29.g0_.z, _e32.g0_.y, _e35.g0_.z)) * vec3<f32>(1.0, -(1.0), 0.0))) + ((vec3<f32>(_e47.g0_.x) * vec3<f32>(_e51.g0_.x, _e54.g0_.w, _e57.g0_.z)) * vec3<f32>(0.0, 1.0, -(1.0)))));
}

fn point_motor_outer_product(self_1104: Point, other_968: Motor) -> Point {
    var self_1105: Point;
    var other_969: Motor;

    self_1105 = self_1104;
    other_969 = other_968;
    let _e4: Point = self_1105;
    let _e6: Motor = other_969;
    return Point((_e4.g0_ * vec3<f32>(_e6.g0_.x)));
}

fn point_motor_inner_product(self_1106: Point, other_970: Motor) -> Motor {
    var self_1107: Point;
    var other_971: Motor;

    self_1107 = self_1106;
    other_971 = other_970;
    let _e4: Point = self_1107;
    let _e8: Motor = other_971;
    let _e19: Point = self_1107;
    let _e23: Motor = other_971;
    let _e35: Point = self_1107;
    let _e39: Motor = other_971;
    return Motor(((((vec4<f32>(_e4.g0_.y) * _e8.g0_.zzxz) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0)) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.wwwx) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))) + ((vec4<f32>(_e35.g0_.x) * _e39.g0_.yxxx) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))));
}

fn point_motor_geometric_anti_product(self_1108: Point, other_972: Motor) -> MotorDual {
    var self_1109: Point;
    var other_973: Motor;

    self_1109 = self_1108;
    other_973 = other_972;
    let _e4: Point = self_1109;
    let _e8: Motor = other_973;
    let _e19: Point = self_1109;
    let _e23: Motor = other_973;
    let _e35: Point = self_1109;
    let _e39: Motor = other_973;
    return MotorDual(((((vec4<f32>(_e4.g0_.x) * _e8.g0_.yxwz) * vec4<f32>(1.0, 1.0, 1.0, -(1.0))) + ((vec4<f32>(_e19.g0_.y) * _e23.g0_.zwxy) * vec4<f32>(1.0, -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e35.g0_.z) * _e39.g0_.wzyx) * vec4<f32>(1.0, 1.0, -(1.0), 1.0))));
}

fn point_motor_inner_anti_product(self_1110: Point, other_974: Motor) -> MotorDual {
    var self_1111: Point;
    var other_975: Motor;

    self_1111 = self_1110;
    other_975 = other_974;
    let _e4: Point = self_1111;
    let _e8: Motor = other_975;
    let _e18: Point = self_1111;
    let _e22: Motor = other_975;
    let _e33: Point = self_1111;
    let _e37: Motor = other_975;
    return MotorDual(((((vec4<f32>(_e4.g0_.y) * _e8.g0_.zzxz) * vec4<f32>(1.0, 0.0, 1.0, 0.0)) + ((vec4<f32>(_e18.g0_.z) * _e22.g0_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e33.g0_.x) * _e37.g0_.yxxx) * vec4<f32>(1.0, 1.0, 0.0, 0.0))));
}

fn point_motor_left_contraction(self_1112: Point, other_976: Motor) -> Scalar {
    var self_1113: Point;
    var other_977: Motor;

    self_1113 = self_1112;
    other_977 = other_976;
    let _e5: Point = self_1113;
    let _e8: Motor = other_977;
    let _e13: Point = self_1113;
    let _e16: Motor = other_977;
    let _e21: Point = self_1113;
    let _e24: Motor = other_977;
    return Scalar((((0.0 - (_e5.g0_.x * _e8.g0_.y)) - (_e13.g0_.y * _e16.g0_.z)) - (_e21.g0_.z * _e24.g0_.w)));
}

fn point_motor_right_contraction(self_1114: Point, other_978: Motor) -> Motor {
    var self_1115: Point;
    var other_979: Motor;

    self_1115 = self_1114;
    other_979 = other_978;
    let _e4: Point = self_1115;
    let _e8: Motor = other_979;
    let _e19: Point = self_1115;
    let _e23: Motor = other_979;
    let _e35: Point = self_1115;
    let _e39: Motor = other_979;
    return Motor(((((vec4<f32>(_e4.g0_.y) * _e8.g0_.zzxz) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0)) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.wwwx) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))) + ((vec4<f32>(_e35.g0_.x) * _e39.g0_.yxxx) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))));
}

fn point_motor_left_anti_contraction(self_1116: Point, other_980: Motor) -> MotorDual {
    var self_1117: Point;
    var other_981: Motor;

    self_1117 = self_1116;
    other_981 = other_980;
    let _e4: Point = self_1117;
    let _e8: Motor = other_981;
    let _e18: Point = self_1117;
    let _e22: Motor = other_981;
    let _e33: Point = self_1117;
    let _e37: Motor = other_981;
    return MotorDual(((((vec4<f32>(_e4.g0_.y) * _e8.g0_.zzxz) * vec4<f32>(1.0, 0.0, 1.0, 0.0)) + ((vec4<f32>(_e18.g0_.z) * _e22.g0_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e33.g0_.x) * _e37.g0_.yxxx) * vec4<f32>(1.0, 1.0, 0.0, 0.0))));
}

fn point_motor_right_anti_contraction(self_1118: Point, other_982: Motor) -> AntiScalar {
    var self_1119: Point;
    var other_983: Motor;

    self_1119 = self_1118;
    other_983 = other_982;
    let _e4: Point = self_1119;
    let _e7: Motor = other_983;
    let _e11: Point = self_1119;
    let _e14: Motor = other_983;
    let _e19: Point = self_1119;
    let _e22: Motor = other_983;
    return AntiScalar((((_e4.g0_.x * _e7.g0_.y) + (_e11.g0_.y * _e14.g0_.z)) + (_e19.g0_.z * _e22.g0_.w)));
}

fn point_motor_scalar_product(self_1120: Point, other_984: Motor) -> Scalar {
    var self_1121: Point;
    var other_985: Motor;

    self_1121 = self_1120;
    other_985 = other_984;
    let _e5: Point = self_1121;
    let _e8: Motor = other_985;
    let _e13: Point = self_1121;
    let _e16: Motor = other_985;
    let _e21: Point = self_1121;
    let _e24: Motor = other_985;
    return Scalar((((0.0 - (_e5.g0_.x * _e8.g0_.y)) - (_e13.g0_.y * _e16.g0_.z)) - (_e21.g0_.z * _e24.g0_.w)));
}

fn point_motor_anti_scalar_product(self_1122: Point, other_986: Motor) -> AntiScalar {
    var self_1123: Point;
    var other_987: Motor;

    self_1123 = self_1122;
    other_987 = other_986;
    let _e4: Point = self_1123;
    let _e7: Motor = other_987;
    let _e11: Point = self_1123;
    let _e14: Motor = other_987;
    let _e19: Point = self_1123;
    let _e22: Motor = other_987;
    return AntiScalar((((_e4.g0_.x * _e7.g0_.y) + (_e11.g0_.y * _e14.g0_.z)) + (_e19.g0_.z * _e22.g0_.w)));
}

fn point_motor_dual_geometric_product(self_1124: Point, other_988: MotorDual) -> MotorDual {
    var self_1125: Point;
    var other_989: MotorDual;

    self_1125 = self_1124;
    other_989 = other_988;
    let _e4: Point = self_1125;
    let _e8: MotorDual = other_989;
    let _e20: Point = self_1125;
    let _e24: MotorDual = other_989;
    let _e37: Point = self_1125;
    let _e41: MotorDual = other_989;
    return MotorDual(((((vec4<f32>(_e4.g0_.x) * _e8.g0_.yxwz) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0)) + ((vec4<f32>(_e20.g0_.y) * _e24.g0_.zwxy) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e37.g0_.z) * _e41.g0_.wzyx) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))));
}

fn point_motor_dual_regressive_product(self_1126: Point, other_990: MotorDual) -> Motor {
    var self_1127: Point;
    var other_991: MotorDual;

    self_1127 = self_1126;
    other_991 = other_990;
    let _e4: Point = self_1127;
    let _e8: MotorDual = other_991;
    let _e18: Point = self_1127;
    let _e22: MotorDual = other_991;
    let _e33: Point = self_1127;
    let _e37: MotorDual = other_991;
    return Motor(((((vec4<f32>(_e4.g0_.y) * _e8.g0_.zzxz) * vec4<f32>(1.0, 0.0, 1.0, 0.0)) + ((vec4<f32>(_e18.g0_.z) * _e22.g0_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e33.g0_.x) * _e37.g0_.yxxx) * vec4<f32>(1.0, 1.0, 0.0, 0.0))));
}

fn point_motor_dual_outer_product(self_1128: Point, other_992: MotorDual) -> AntiScalar {
    var self_1129: Point;
    var other_993: MotorDual;

    self_1129 = self_1128;
    other_993 = other_992;
    let _e4: Point = self_1129;
    let _e7: MotorDual = other_993;
    let _e11: Point = self_1129;
    let _e14: MotorDual = other_993;
    let _e19: Point = self_1129;
    let _e22: MotorDual = other_993;
    return AntiScalar((((_e4.g0_.x * _e7.g0_.y) + (_e11.g0_.y * _e14.g0_.z)) + (_e19.g0_.z * _e22.g0_.w)));
}

fn point_motor_dual_inner_product(self_1130: Point, other_994: MotorDual) -> Plane {
    var self_1131: Point;
    var other_995: MotorDual;

    self_1131 = self_1130;
    other_995 = other_994;
    let _e4: Point = self_1131;
    let _e8: MotorDual = other_995;
    let _e11: MotorDual = other_995;
    let _e14: MotorDual = other_995;
    let _e26: Point = self_1131;
    let _e30: MotorDual = other_995;
    let _e33: MotorDual = other_995;
    let _e36: MotorDual = other_995;
    let _e49: Point = self_1131;
    let _e53: MotorDual = other_995;
    let _e56: MotorDual = other_995;
    let _e59: MotorDual = other_995;
    return Plane(((((vec3<f32>(_e4.g0_.x) * vec3<f32>(_e8.g0_.x, _e11.g0_.w, _e14.g0_.z)) * vec3<f32>(-(1.0), -(1.0), 1.0)) + ((vec3<f32>(_e26.g0_.y) * vec3<f32>(_e30.g0_.w, _e33.g0_.x, _e36.g0_.y)) * vec3<f32>(1.0, -(1.0), -(1.0)))) + ((vec3<f32>(_e49.g0_.z) * vec3<f32>(_e53.g0_.z, _e56.g0_.y, _e59.g0_.x)) * vec3<f32>(-(1.0), 1.0, -(1.0)))));
}

fn point_motor_dual_geometric_anti_product(self_1132: Point, other_996: MotorDual) -> Motor {
    var self_1133: Point;
    var other_997: MotorDual;

    self_1133 = self_1132;
    other_997 = other_996;
    let _e4: Point = self_1133;
    let _e8: MotorDual = other_997;
    let _e19: Point = self_1133;
    let _e23: MotorDual = other_997;
    let _e35: Point = self_1133;
    let _e39: MotorDual = other_997;
    return Motor(((((vec4<f32>(_e4.g0_.x) * _e8.g0_.yxwz) * vec4<f32>(1.0, 1.0, -(1.0), 1.0)) + ((vec4<f32>(_e19.g0_.y) * _e23.g0_.zwxy) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e35.g0_.z) * _e39.g0_.wzyx) * vec4<f32>(1.0, -(1.0), 1.0, 1.0))));
}

fn point_motor_dual_inner_anti_product(self_1134: Point, other_998: MotorDual) -> Point {
    var self_1135: Point;
    var other_999: MotorDual;

    self_1135 = self_1134;
    other_999 = other_998;
    let _e4: Point = self_1135;
    let _e8: MotorDual = other_999;
    let _e11: MotorDual = other_999;
    let _e14: MotorDual = other_999;
    let _e25: Point = self_1135;
    let _e29: MotorDual = other_999;
    let _e32: MotorDual = other_999;
    let _e35: MotorDual = other_999;
    let _e47: Point = self_1135;
    let _e51: MotorDual = other_999;
    let _e54: MotorDual = other_999;
    let _e57: MotorDual = other_999;
    return Point(((((vec3<f32>(_e4.g0_.x) * vec3<f32>(_e8.g0_.x, _e11.g0_.w, _e14.g0_.z)) * vec3<f32>(1.0, -(1.0), 1.0)) + ((vec3<f32>(_e25.g0_.y) * vec3<f32>(_e29.g0_.w, _e32.g0_.x, _e35.g0_.y)) * vec3<f32>(1.0, 1.0, -(1.0)))) + ((vec3<f32>(_e47.g0_.z) * vec3<f32>(_e51.g0_.z, _e54.g0_.y, _e57.g0_.x)) * vec3<f32>(-(1.0), 1.0, 1.0))));
}

fn point_motor_dual_left_contraction(self_1136: Point, other_1000: MotorDual) -> Plane {
    var self_1137: Point;
    var other_1001: MotorDual;

    self_1137 = self_1136;
    other_1001 = other_1000;
    let _e4: Point = self_1137;
    let _e6: MotorDual = other_1001;
    return Plane(((_e4.g0_ * vec3<f32>(_e6.g0_.x)) * vec3<f32>(-(1.0))));
}

fn point_motor_dual_right_contraction(self_1138: Point, other_1002: MotorDual) -> Plane {
    var self_1139: Point;
    var other_1003: MotorDual;

    self_1139 = self_1138;
    other_1003 = other_1002;
    let _e4: Point = self_1139;
    let _e8: MotorDual = other_1003;
    let _e11: MotorDual = other_1003;
    let _e14: MotorDual = other_1003;
    let _e25: Point = self_1139;
    let _e29: MotorDual = other_1003;
    let _e32: MotorDual = other_1003;
    let _e35: MotorDual = other_1003;
    let _e47: Point = self_1139;
    let _e51: MotorDual = other_1003;
    let _e54: MotorDual = other_1003;
    let _e57: MotorDual = other_1003;
    return Plane(((((vec3<f32>(_e4.g0_.y) * vec3<f32>(_e8.g0_.w, _e11.g0_.w, _e14.g0_.y)) * vec3<f32>(1.0, 0.0, -(1.0))) + ((vec3<f32>(_e25.g0_.z) * vec3<f32>(_e29.g0_.z, _e32.g0_.y, _e35.g0_.z)) * vec3<f32>(-(1.0), 1.0, 0.0))) + ((vec3<f32>(_e47.g0_.x) * vec3<f32>(_e51.g0_.x, _e54.g0_.w, _e57.g0_.z)) * vec3<f32>(0.0, -(1.0), 1.0))));
}

fn point_motor_dual_left_anti_contraction(self_1140: Point, other_1004: MotorDual) -> Point {
    var self_1141: Point;
    var other_1005: MotorDual;

    self_1141 = self_1140;
    other_1005 = other_1004;
    let _e4: Point = self_1141;
    let _e8: MotorDual = other_1005;
    let _e11: MotorDual = other_1005;
    let _e14: MotorDual = other_1005;
    let _e25: Point = self_1141;
    let _e29: MotorDual = other_1005;
    let _e32: MotorDual = other_1005;
    let _e35: MotorDual = other_1005;
    let _e47: Point = self_1141;
    let _e51: MotorDual = other_1005;
    let _e54: MotorDual = other_1005;
    let _e57: MotorDual = other_1005;
    return Point(((((vec3<f32>(_e4.g0_.y) * vec3<f32>(_e8.g0_.w, _e11.g0_.w, _e14.g0_.y)) * vec3<f32>(1.0, 0.0, -(1.0))) + ((vec3<f32>(_e25.g0_.z) * vec3<f32>(_e29.g0_.z, _e32.g0_.y, _e35.g0_.z)) * vec3<f32>(-(1.0), 1.0, 0.0))) + ((vec3<f32>(_e47.g0_.x) * vec3<f32>(_e51.g0_.x, _e54.g0_.w, _e57.g0_.z)) * vec3<f32>(0.0, -(1.0), 1.0))));
}

fn point_motor_dual_right_anti_contraction(self_1142: Point, other_1006: MotorDual) -> Point {
    var self_1143: Point;
    var other_1007: MotorDual;

    self_1143 = self_1142;
    other_1007 = other_1006;
    let _e4: Point = self_1143;
    let _e6: MotorDual = other_1007;
    return Point((_e4.g0_ * vec3<f32>(_e6.g0_.x)));
}

fn point_squared_magnitude(self_1144: Point) -> Scalar {
    var self_1145: Point;

    self_1145 = self_1144;
    let _e2: Point = self_1145;
    let _e3: Point = self_1145;
    let _e4: Point = point_reversal(_e3);
    let _e5: Scalar = point_point_scalar_product(_e2, _e4);
    return _e5;
}

fn point_magnitude(self_1146: Point) -> Scalar {
    var self_1147: Point;

    self_1147 = self_1146;
    let _e2: Point = self_1147;
    let _e3: Scalar = point_squared_magnitude(_e2);
    return Scalar(sqrt(_e3.g0_));
}

fn point_bulk_norm(self_1148: Point) -> Scalar {
    var self_1149: Point;

    self_1149 = self_1148;
    let _e2: Point = self_1149;
    let _e3: Scalar = point_squared_magnitude(_e2);
    return Scalar(sqrt(_e3.g0_));
}

fn point_squared_anti_magnitude(self_1150: Point) -> AntiScalar {
    var self_1151: Point;

    self_1151 = self_1150;
    let _e2: Point = self_1151;
    let _e3: Point = self_1151;
    let _e4: Point = point_anti_reversal(_e3);
    let _e5: AntiScalar = point_point_anti_scalar_product(_e2, _e4);
    return _e5;
}

fn point_weight_norm(self_1152: Point) -> AntiScalar {
    var self_1153: Point;

    self_1153 = self_1152;
    let _e2: Point = self_1153;
    let _e3: AntiScalar = point_squared_anti_magnitude(_e2);
    return AntiScalar(sqrt(_e3.g0_));
}

fn point_scale(self_1154: Point, other_1008: f32) -> Point {
    var self_1155: Point;
    var other_1009: f32;

    self_1155 = self_1154;
    other_1009 = other_1008;
    let _e4: Point = self_1155;
    let _e5: f32 = other_1009;
    let _e7: Point = point_scalar_geometric_product(_e4, Scalar(_e5));
    return _e7;
}

fn point_signum(self_1156: Point) -> Point {
    var self_1157: Point;

    self_1157 = self_1156;
    let _e2: Point = self_1157;
    let _e3: Point = self_1157;
    let _e4: Scalar = point_magnitude(_e3);
    let _e9: Point = point_scalar_geometric_product(_e2, Scalar((1.0 / _e4.g0_)));
    return _e9;
}

fn point_inverse(self_1158: Point) -> Point {
    var self_1159: Point;

    self_1159 = self_1158;
    let _e2: Point = self_1159;
    let _e3: Point = point_reversal(_e2);
    let _e4: Point = self_1159;
    let _e5: Scalar = point_squared_magnitude(_e4);
    let _e10: Point = point_scalar_geometric_product(_e3, Scalar((1.0 / _e5.g0_)));
    return _e10;
}

fn ideal_point_zero() -> IdealPoint {
    return IdealPoint(vec2<f32>(0.0));
}

fn ideal_point_one() -> IdealPoint {
    return IdealPoint(vec2<f32>(0.0));
}

fn ideal_point_neg(self_1160: IdealPoint) -> IdealPoint {
    var self_1161: IdealPoint;

    self_1161 = self_1160;
    let _e2: IdealPoint = self_1161;
    return IdealPoint((_e2.g0_ * vec2<f32>(-(1.0))));
}

fn ideal_point_automorphism(self_1162: IdealPoint) -> IdealPoint {
    var self_1163: IdealPoint;

    self_1163 = self_1162;
    let _e2: IdealPoint = self_1163;
    return IdealPoint(_e2.g0_);
}

fn ideal_point_reversal(self_1164: IdealPoint) -> IdealPoint {
    var self_1165: IdealPoint;

    self_1165 = self_1164;
    let _e2: IdealPoint = self_1165;
    return IdealPoint((_e2.g0_ * vec2<f32>(-(1.0))));
}

fn ideal_point_conjugation(self_1166: IdealPoint) -> IdealPoint {
    var self_1167: IdealPoint;

    self_1167 = self_1166;
    let _e2: IdealPoint = self_1167;
    return IdealPoint((_e2.g0_ * vec2<f32>(-(1.0))));
}

fn ideal_point_anti_reversal(self_1168: IdealPoint) -> IdealPoint {
    var self_1169: IdealPoint;

    self_1169 = self_1168;
    let _e2: IdealPoint = self_1169;
    return IdealPoint((_e2.g0_ * vec2<f32>(-(1.0))));
}

fn ideal_point_scalar_add(self_1170: IdealPoint, other_1010: Scalar) -> Translator {
    var self_1171: IdealPoint;
    var other_1011: Scalar;

    self_1171 = self_1170;
    other_1011 = other_1010;
    let _e4: IdealPoint = self_1171;
    let _e7: IdealPoint = self_1171;
    let _e10: IdealPoint = self_1171;
    let _e19: Scalar = other_1011;
    return Translator(((vec3<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.y) * vec3<f32>(0.0, 1.0, 1.0)) + (vec3<f32>(_e19.g0_) * vec3<f32>(1.0, 0.0, 0.0))));
}

fn ideal_point_scalar_sub(self_1172: IdealPoint, other_1012: Scalar) -> Translator {
    var self_1173: IdealPoint;
    var other_1013: Scalar;

    self_1173 = self_1172;
    other_1013 = other_1012;
    let _e4: IdealPoint = self_1173;
    let _e7: IdealPoint = self_1173;
    let _e10: IdealPoint = self_1173;
    let _e19: Scalar = other_1013;
    return Translator(((vec3<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.y) * vec3<f32>(0.0, 1.0, 1.0)) - (vec3<f32>(_e19.g0_) * vec3<f32>(1.0, 0.0, 0.0))));
}

fn ideal_point_scalar_geometric_product(self_1174: IdealPoint, other_1014: Scalar) -> IdealPoint {
    var self_1175: IdealPoint;
    var other_1015: Scalar;

    self_1175 = self_1174;
    other_1015 = other_1014;
    let _e4: IdealPoint = self_1175;
    let _e6: Scalar = other_1015;
    return IdealPoint((_e4.g0_ * vec2<f32>(_e6.g0_)));
}

fn ideal_point_scalar_outer_product(self_1176: IdealPoint, other_1016: Scalar) -> IdealPoint {
    var self_1177: IdealPoint;
    var other_1017: Scalar;

    self_1177 = self_1176;
    other_1017 = other_1016;
    let _e4: IdealPoint = self_1177;
    let _e6: Scalar = other_1017;
    return IdealPoint((_e4.g0_ * vec2<f32>(_e6.g0_)));
}

fn ideal_point_scalar_inner_product(self_1178: IdealPoint, other_1018: Scalar) -> IdealPoint {
    var self_1179: IdealPoint;
    var other_1019: Scalar;

    self_1179 = self_1178;
    other_1019 = other_1018;
    let _e4: IdealPoint = self_1179;
    let _e6: Scalar = other_1019;
    return IdealPoint((_e4.g0_ * vec2<f32>(_e6.g0_)));
}

fn ideal_point_scalar_right_contraction(self_1180: IdealPoint, other_1020: Scalar) -> IdealPoint {
    var self_1181: IdealPoint;
    var other_1021: Scalar;

    self_1181 = self_1180;
    other_1021 = other_1020;
    let _e4: IdealPoint = self_1181;
    let _e6: Scalar = other_1021;
    return IdealPoint((_e4.g0_ * vec2<f32>(_e6.g0_)));
}

fn ideal_point_anti_scalar_regressive_product(self_1182: IdealPoint, other_1022: AntiScalar) -> IdealPoint {
    var self_1183: IdealPoint;
    var other_1023: AntiScalar;

    self_1183 = self_1182;
    other_1023 = other_1022;
    let _e4: IdealPoint = self_1183;
    let _e6: AntiScalar = other_1023;
    return IdealPoint((_e4.g0_ * vec2<f32>(_e6.g0_)));
}

fn ideal_point_anti_scalar_geometric_anti_product(self_1184: IdealPoint, other_1024: AntiScalar) -> IdealPoint {
    var self_1185: IdealPoint;
    var other_1025: AntiScalar;

    self_1185 = self_1184;
    other_1025 = other_1024;
    let _e4: IdealPoint = self_1185;
    let _e6: AntiScalar = other_1025;
    return IdealPoint((_e4.g0_ * vec2<f32>(_e6.g0_)));
}

fn ideal_point_anti_scalar_inner_anti_product(self_1186: IdealPoint, other_1026: AntiScalar) -> IdealPoint {
    var self_1187: IdealPoint;
    var other_1027: AntiScalar;

    self_1187 = self_1186;
    other_1027 = other_1026;
    let _e4: IdealPoint = self_1187;
    let _e6: AntiScalar = other_1027;
    return IdealPoint((_e4.g0_ * vec2<f32>(_e6.g0_)));
}

fn ideal_point_anti_scalar_right_anti_contraction(self_1188: IdealPoint, other_1028: AntiScalar) -> IdealPoint {
    var self_1189: IdealPoint;
    var other_1029: AntiScalar;

    self_1189 = self_1188;
    other_1029 = other_1028;
    let _e4: IdealPoint = self_1189;
    let _e6: AntiScalar = other_1029;
    return IdealPoint((_e4.g0_ * vec2<f32>(_e6.g0_)));
}

fn ideal_point_multi_vector_add(self_1190: IdealPoint, other_1030: MultiVector) -> MultiVector {
    var self_1191: IdealPoint;
    var other_1031: MultiVector;

    self_1191 = self_1190;
    other_1031 = other_1030;
    let _e4: MultiVector = other_1031;
    let _e6: IdealPoint = self_1191;
    let _e9: IdealPoint = self_1191;
    let _e12: IdealPoint = self_1191;
    let _e15: IdealPoint = self_1191;
    let _e25: MultiVector = other_1031;
    return MultiVector(_e4.g0_, ((vec4<f32>(_e6.g0_.x, _e9.g0_.x, _e12.g0_.x, _e15.g0_.y) * vec4<f32>(0.0, 0.0, 1.0, 1.0)) + _e25.g1_));
}

fn ideal_point_multi_vector_sub(self_1192: IdealPoint, other_1032: MultiVector) -> MultiVector {
    var self_1193: IdealPoint;
    var other_1033: MultiVector;

    self_1193 = self_1192;
    other_1033 = other_1032;
    let _e6: MultiVector = other_1033;
    let _e9: IdealPoint = self_1193;
    let _e12: IdealPoint = self_1193;
    let _e15: IdealPoint = self_1193;
    let _e18: IdealPoint = self_1193;
    let _e28: MultiVector = other_1033;
    return MultiVector((vec4<f32>(0.0) - _e6.g0_), ((vec4<f32>(_e9.g0_.x, _e12.g0_.x, _e15.g0_.x, _e18.g0_.y) * vec4<f32>(0.0, 0.0, 1.0, 1.0)) - _e28.g1_));
}

fn ideal_point_multi_vector_geometric_product(self_1194: IdealPoint, other_1034: MultiVector) -> MultiVector {
    var self_1195: IdealPoint;
    var other_1035: MultiVector;

    self_1195 = self_1194;
    other_1035 = other_1034;
    let _e4: IdealPoint = self_1195;
    let _e8: MultiVector = other_1035;
    let _e21: IdealPoint = self_1195;
    let _e25: MultiVector = other_1035;
    let _e39: IdealPoint = self_1195;
    let _e43: MultiVector = other_1035;
    let _e54: IdealPoint = self_1195;
    let _e58: MultiVector = other_1035;
    return MultiVector((((vec4<f32>(_e4.g0_.x) * _e8.g1_.zwxy) * vec4<f32>(-(1.0), 1.0, -(1.0), -(1.0))) + ((vec4<f32>(_e21.g0_.y) * _e25.g1_.wzyx) * vec4<f32>(-(1.0), -(1.0), -(1.0), 1.0))), (((vec4<f32>(_e39.g0_.x) * _e43.g0_.zwxy) * vec4<f32>(1.0, 1.0, 1.0, -(1.0))) + ((vec4<f32>(_e54.g0_.y) * _e58.g0_.wzyx) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0))));
}

fn ideal_point_multi_vector_geometric_anti_product(self_1196: IdealPoint, other_1036: MultiVector) -> MultiVector {
    var self_1197: IdealPoint;
    var other_1037: MultiVector;

    self_1197 = self_1196;
    other_1037 = other_1036;
    let _e4: IdealPoint = self_1197;
    let _e8: MultiVector = other_1037;
    let _e12: IdealPoint = self_1197;
    let _e16: MultiVector = other_1037;
    let _e29: IdealPoint = self_1197;
    let _e33: MultiVector = other_1037;
    let _e45: IdealPoint = self_1197;
    let _e49: MultiVector = other_1037;
    return MultiVector(((vec4<f32>(_e4.g0_.x) * _e8.g0_.wzyx) + ((vec4<f32>(_e12.g0_.y) * _e16.g0_.zwxy) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))), (((vec4<f32>(_e29.g0_.x) * _e33.g1_.wzyx) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0))) + (vec4<f32>(_e45.g0_.y) * _e49.g1_.zwxy)));
}

fn ideal_point_multi_vector_scalar_product(self_1198: IdealPoint, other_1038: MultiVector) -> Scalar {
    var self_1199: IdealPoint;
    var other_1039: MultiVector;

    self_1199 = self_1198;
    other_1039 = other_1038;
    let _e5: IdealPoint = self_1199;
    let _e8: MultiVector = other_1039;
    let _e13: IdealPoint = self_1199;
    let _e16: MultiVector = other_1039;
    return Scalar(((0.0 - (_e5.g0_.x * _e8.g1_.z)) - (_e13.g0_.y * _e16.g1_.w)));
}

fn ideal_point_multi_vector_anti_scalar_product(self_1200: IdealPoint, other_1040: MultiVector) -> AntiScalar {
    var self_1201: IdealPoint;
    var other_1041: MultiVector;

    self_1201 = self_1200;
    other_1041 = other_1040;
    let _e4: IdealPoint = self_1201;
    let _e7: MultiVector = other_1041;
    let _e11: IdealPoint = self_1201;
    let _e14: MultiVector = other_1041;
    return AntiScalar(((_e4.g0_.x * _e7.g1_.z) + (_e11.g0_.y * _e14.g1_.w)));
}

fn ideal_point_rotor_add(self_1202: IdealPoint, other_1042: Rotor) -> Motor {
    var self_1203: IdealPoint;
    var other_1043: Rotor;

    self_1203 = self_1202;
    other_1043 = other_1042;
    let _e4: IdealPoint = self_1203;
    let _e7: IdealPoint = self_1203;
    let _e10: IdealPoint = self_1203;
    let _e13: IdealPoint = self_1203;
    let _e23: Rotor = other_1043;
    let _e26: Rotor = other_1043;
    let _e29: Rotor = other_1043;
    let _e32: Rotor = other_1043;
    return Motor(((vec4<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.x, _e13.g0_.y) * vec4<f32>(0.0, 0.0, 1.0, 1.0)) + (vec4<f32>(_e23.g0_.x, _e26.g0_.y, _e29.g0_.x, _e32.g0_.x) * vec4<f32>(1.0, 1.0, 0.0, 0.0))));
}

fn ideal_point_rotor_sub(self_1204: IdealPoint, other_1044: Rotor) -> Motor {
    var self_1205: IdealPoint;
    var other_1045: Rotor;

    self_1205 = self_1204;
    other_1045 = other_1044;
    let _e4: IdealPoint = self_1205;
    let _e7: IdealPoint = self_1205;
    let _e10: IdealPoint = self_1205;
    let _e13: IdealPoint = self_1205;
    let _e23: Rotor = other_1045;
    let _e26: Rotor = other_1045;
    let _e29: Rotor = other_1045;
    let _e32: Rotor = other_1045;
    return Motor(((vec4<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.x, _e13.g0_.y) * vec4<f32>(0.0, 0.0, 1.0, 1.0)) - (vec4<f32>(_e23.g0_.x, _e26.g0_.y, _e29.g0_.x, _e32.g0_.x) * vec4<f32>(1.0, 1.0, 0.0, 0.0))));
}

fn ideal_point_rotor_geometric_product(self_1206: IdealPoint, other_1046: Rotor) -> IdealPoint {
    var self_1207: IdealPoint;
    var other_1047: Rotor;

    self_1207 = self_1206;
    other_1047 = other_1046;
    let _e4: IdealPoint = self_1207;
    let _e8: Rotor = other_1047;
    let _e16: IdealPoint = self_1207;
    let _e20: Rotor = other_1047;
    return IdealPoint((((vec2<f32>(_e4.g0_.x) * _e8.g0_) * vec2<f32>(1.0, -(1.0))) + (vec2<f32>(_e16.g0_.y) * _e20.g0_.yx)));
}

fn ideal_point_rotor_outer_product(self_1208: IdealPoint, other_1048: Rotor) -> IdealPoint {
    var self_1209: IdealPoint;
    var other_1049: Rotor;

    self_1209 = self_1208;
    other_1049 = other_1048;
    let _e4: IdealPoint = self_1209;
    let _e6: Rotor = other_1049;
    return IdealPoint((_e4.g0_ * vec2<f32>(_e6.g0_.x)));
}

fn ideal_point_rotor_inner_product(self_1210: IdealPoint, other_1050: Rotor) -> IdealPoint {
    var self_1211: IdealPoint;
    var other_1051: Rotor;

    self_1211 = self_1210;
    other_1051 = other_1050;
    let _e4: IdealPoint = self_1211;
    let _e6: Rotor = other_1051;
    return IdealPoint((_e4.g0_ * vec2<f32>(_e6.g0_.x)));
}

fn ideal_point_rotor_right_contraction(self_1212: IdealPoint, other_1052: Rotor) -> IdealPoint {
    var self_1213: IdealPoint;
    var other_1053: Rotor;

    self_1213 = self_1212;
    other_1053 = other_1052;
    let _e4: IdealPoint = self_1213;
    let _e6: Rotor = other_1053;
    return IdealPoint((_e4.g0_ * vec2<f32>(_e6.g0_.x)));
}

fn ideal_point_point_add(self_1214: IdealPoint, other_1054: Point) -> Point {
    var self_1215: IdealPoint;
    var other_1055: Point;

    self_1215 = self_1214;
    other_1055 = other_1054;
    let _e4: IdealPoint = self_1215;
    let _e7: IdealPoint = self_1215;
    let _e10: IdealPoint = self_1215;
    let _e19: Point = other_1055;
    return Point(((vec3<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.y) * vec3<f32>(0.0, 1.0, 1.0)) + _e19.g0_));
}

fn ideal_point_point_sub(self_1216: IdealPoint, other_1056: Point) -> Point {
    var self_1217: IdealPoint;
    var other_1057: Point;

    self_1217 = self_1216;
    other_1057 = other_1056;
    let _e4: IdealPoint = self_1217;
    let _e7: IdealPoint = self_1217;
    let _e10: IdealPoint = self_1217;
    let _e19: Point = other_1057;
    return Point(((vec3<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.y) * vec3<f32>(0.0, 1.0, 1.0)) - _e19.g0_));
}

fn ideal_point_point_geometric_product(self_1218: IdealPoint, other_1058: Point) -> Motor {
    var self_1219: IdealPoint;
    var other_1059: Point;

    self_1219 = self_1218;
    other_1059 = other_1058;
    let _e4: IdealPoint = self_1219;
    let _e8: Point = other_1059;
    let _e11: Point = other_1059;
    let _e14: Point = other_1059;
    let _e17: Point = other_1059;
    let _e30: IdealPoint = self_1219;
    let _e34: Point = other_1059;
    let _e37: Point = other_1059;
    let _e40: Point = other_1059;
    let _e43: Point = other_1059;
    return Motor((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.z, _e11.g0_.y, _e14.g0_.x, _e17.g0_.z)) * vec4<f32>(-(1.0), -(1.0), 1.0, 0.0)) + ((vec4<f32>(_e30.g0_.x) * vec4<f32>(_e34.g0_.y, _e37.g0_.z, _e40.g0_.x, _e43.g0_.x)) * vec4<f32>(-(1.0), 1.0, 0.0, -(1.0)))));
}

fn ideal_point_point_regressive_product(self_1220: IdealPoint, other_1060: Point) -> Plane {
    var self_1221: IdealPoint;
    var other_1061: Point;

    self_1221 = self_1220;
    other_1061 = other_1060;
    let _e4: IdealPoint = self_1221;
    let _e8: Point = other_1061;
    let _e18: IdealPoint = self_1221;
    let _e22: Point = other_1061;
    return Plane((((vec3<f32>(_e4.g0_.y) * _e8.g0_.yxy) * vec3<f32>(1.0, -(1.0), 0.0)) + ((vec3<f32>(_e18.g0_.x) * _e22.g0_.zxx) * vec3<f32>(-(1.0), 0.0, 1.0))));
}

fn ideal_point_point_inner_product(self_1222: IdealPoint, other_1062: Point) -> Scalar {
    var self_1223: IdealPoint;
    var other_1063: Point;

    self_1223 = self_1222;
    other_1063 = other_1062;
    let _e5: IdealPoint = self_1223;
    let _e8: Point = other_1063;
    let _e13: IdealPoint = self_1223;
    let _e16: Point = other_1063;
    return Scalar(((0.0 - (_e5.g0_.x * _e8.g0_.y)) - (_e13.g0_.y * _e16.g0_.z)));
}

fn ideal_point_point_geometric_anti_product(self_1224: IdealPoint, other_1064: Point) -> MotorDual {
    var self_1225: IdealPoint;
    var other_1065: Point;

    self_1225 = self_1224;
    other_1065 = other_1064;
    let _e4: IdealPoint = self_1225;
    let _e8: Point = other_1065;
    let _e11: Point = other_1065;
    let _e14: Point = other_1065;
    let _e17: Point = other_1065;
    let _e29: IdealPoint = self_1225;
    let _e33: Point = other_1065;
    let _e36: Point = other_1065;
    let _e39: Point = other_1065;
    let _e42: Point = other_1065;
    return MotorDual((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.z, _e11.g0_.y, _e14.g0_.x, _e17.g0_.z)) * vec4<f32>(1.0, 1.0, -(1.0), 0.0)) + ((vec4<f32>(_e29.g0_.x) * vec4<f32>(_e33.g0_.y, _e36.g0_.z, _e39.g0_.x, _e42.g0_.x)) * vec4<f32>(1.0, -(1.0), 0.0, 1.0))));
}

fn ideal_point_point_inner_anti_product(self_1226: IdealPoint, other_1066: Point) -> AntiScalar {
    var self_1227: IdealPoint;
    var other_1067: Point;

    self_1227 = self_1226;
    other_1067 = other_1066;
    let _e4: IdealPoint = self_1227;
    let _e7: Point = other_1067;
    let _e11: IdealPoint = self_1227;
    let _e14: Point = other_1067;
    return AntiScalar(((_e4.g0_.x * _e7.g0_.y) + (_e11.g0_.y * _e14.g0_.z)));
}

fn ideal_point_point_left_contraction(self_1228: IdealPoint, other_1068: Point) -> Scalar {
    var self_1229: IdealPoint;
    var other_1069: Point;

    self_1229 = self_1228;
    other_1069 = other_1068;
    let _e5: IdealPoint = self_1229;
    let _e8: Point = other_1069;
    let _e13: IdealPoint = self_1229;
    let _e16: Point = other_1069;
    return Scalar(((0.0 - (_e5.g0_.x * _e8.g0_.y)) - (_e13.g0_.y * _e16.g0_.z)));
}

fn ideal_point_point_right_contraction(self_1230: IdealPoint, other_1070: Point) -> Scalar {
    var self_1231: IdealPoint;
    var other_1071: Point;

    self_1231 = self_1230;
    other_1071 = other_1070;
    let _e5: IdealPoint = self_1231;
    let _e8: Point = other_1071;
    let _e13: IdealPoint = self_1231;
    let _e16: Point = other_1071;
    return Scalar(((0.0 - (_e5.g0_.x * _e8.g0_.y)) - (_e13.g0_.y * _e16.g0_.z)));
}

fn ideal_point_point_left_anti_contraction(self_1232: IdealPoint, other_1072: Point) -> AntiScalar {
    var self_1233: IdealPoint;
    var other_1073: Point;

    self_1233 = self_1232;
    other_1073 = other_1072;
    let _e4: IdealPoint = self_1233;
    let _e7: Point = other_1073;
    let _e11: IdealPoint = self_1233;
    let _e14: Point = other_1073;
    return AntiScalar(((_e4.g0_.x * _e7.g0_.y) + (_e11.g0_.y * _e14.g0_.z)));
}

fn ideal_point_point_right_anti_contraction(self_1234: IdealPoint, other_1074: Point) -> AntiScalar {
    var self_1235: IdealPoint;
    var other_1075: Point;

    self_1235 = self_1234;
    other_1075 = other_1074;
    let _e4: IdealPoint = self_1235;
    let _e7: Point = other_1075;
    let _e11: IdealPoint = self_1235;
    let _e14: Point = other_1075;
    return AntiScalar(((_e4.g0_.x * _e7.g0_.y) + (_e11.g0_.y * _e14.g0_.z)));
}

fn ideal_point_point_scalar_product(self_1236: IdealPoint, other_1076: Point) -> Scalar {
    var self_1237: IdealPoint;
    var other_1077: Point;

    self_1237 = self_1236;
    other_1077 = other_1076;
    let _e5: IdealPoint = self_1237;
    let _e8: Point = other_1077;
    let _e13: IdealPoint = self_1237;
    let _e16: Point = other_1077;
    return Scalar(((0.0 - (_e5.g0_.x * _e8.g0_.y)) - (_e13.g0_.y * _e16.g0_.z)));
}

fn ideal_point_point_anti_scalar_product(self_1238: IdealPoint, other_1078: Point) -> AntiScalar {
    var self_1239: IdealPoint;
    var other_1079: Point;

    self_1239 = self_1238;
    other_1079 = other_1078;
    let _e4: IdealPoint = self_1239;
    let _e7: Point = other_1079;
    let _e11: IdealPoint = self_1239;
    let _e14: Point = other_1079;
    return AntiScalar(((_e4.g0_.x * _e7.g0_.y) + (_e11.g0_.y * _e14.g0_.z)));
}

fn ideal_point_ideal_point_add(self_1240: IdealPoint, other_1080: IdealPoint) -> IdealPoint {
    var self_1241: IdealPoint;
    var other_1081: IdealPoint;

    self_1241 = self_1240;
    other_1081 = other_1080;
    let _e4: IdealPoint = self_1241;
    let _e6: IdealPoint = other_1081;
    return IdealPoint((_e4.g0_ + _e6.g0_));
}

fn ideal_point_ideal_point_sub(self_1242: IdealPoint, other_1082: IdealPoint) -> IdealPoint {
    var self_1243: IdealPoint;
    var other_1083: IdealPoint;

    self_1243 = self_1242;
    other_1083 = other_1082;
    let _e4: IdealPoint = self_1243;
    let _e6: IdealPoint = other_1083;
    return IdealPoint((_e4.g0_ - _e6.g0_));
}

fn ideal_point_ideal_point_mul(self_1244: IdealPoint, other_1084: IdealPoint) -> IdealPoint {
    var self_1245: IdealPoint;
    var other_1085: IdealPoint;

    self_1245 = self_1244;
    other_1085 = other_1084;
    let _e4: IdealPoint = self_1245;
    let _e6: IdealPoint = other_1085;
    return IdealPoint((_e4.g0_ * _e6.g0_));
}

fn ideal_point_ideal_point_div(self_1246: IdealPoint, other_1086: IdealPoint) -> IdealPoint {
    var self_1247: IdealPoint;
    var other_1087: IdealPoint;

    self_1247 = self_1246;
    other_1087 = other_1086;
    let _e4: IdealPoint = self_1247;
    let _e7: IdealPoint = self_1247;
    let _e15: IdealPoint = other_1087;
    let _e18: IdealPoint = other_1087;
    return IdealPoint((((vec2<f32>(_e4.g0_.x, _e7.g0_.y) * vec2<f32>(1.0, 1.0)) / vec2<f32>(_e15.g0_.x, _e18.g0_.y)) * vec2<f32>(1.0, 1.0)));
}

fn ideal_point_ideal_point_geometric_product(self_1248: IdealPoint, other_1088: IdealPoint) -> Rotor {
    var self_1249: IdealPoint;
    var other_1089: IdealPoint;

    self_1249 = self_1248;
    other_1089 = other_1088;
    let _e4: IdealPoint = self_1249;
    let _e8: IdealPoint = other_1089;
    let _e16: IdealPoint = self_1249;
    let _e20: IdealPoint = other_1089;
    return Rotor((((vec2<f32>(_e4.g0_.x) * _e8.g0_) * vec2<f32>(-(1.0), 1.0)) - (vec2<f32>(_e16.g0_.y) * _e20.g0_.yx)));
}

fn ideal_point_ideal_point_inner_product(self_1250: IdealPoint, other_1090: IdealPoint) -> Scalar {
    var self_1251: IdealPoint;
    var other_1091: IdealPoint;

    self_1251 = self_1250;
    other_1091 = other_1090;
    let _e5: IdealPoint = self_1251;
    let _e8: IdealPoint = other_1091;
    let _e13: IdealPoint = self_1251;
    let _e16: IdealPoint = other_1091;
    return Scalar(((0.0 - (_e5.g0_.x * _e8.g0_.x)) - (_e13.g0_.y * _e16.g0_.y)));
}

fn ideal_point_ideal_point_inner_anti_product(self_1252: IdealPoint, other_1092: IdealPoint) -> AntiScalar {
    var self_1253: IdealPoint;
    var other_1093: IdealPoint;

    self_1253 = self_1252;
    other_1093 = other_1092;
    let _e4: IdealPoint = self_1253;
    let _e7: IdealPoint = other_1093;
    let _e11: IdealPoint = self_1253;
    let _e14: IdealPoint = other_1093;
    return AntiScalar(((_e4.g0_.x * _e7.g0_.x) + (_e11.g0_.y * _e14.g0_.y)));
}

fn ideal_point_ideal_point_left_contraction(self_1254: IdealPoint, other_1094: IdealPoint) -> Scalar {
    var self_1255: IdealPoint;
    var other_1095: IdealPoint;

    self_1255 = self_1254;
    other_1095 = other_1094;
    let _e5: IdealPoint = self_1255;
    let _e8: IdealPoint = other_1095;
    let _e13: IdealPoint = self_1255;
    let _e16: IdealPoint = other_1095;
    return Scalar(((0.0 - (_e5.g0_.x * _e8.g0_.x)) - (_e13.g0_.y * _e16.g0_.y)));
}

fn ideal_point_ideal_point_right_contraction(self_1256: IdealPoint, other_1096: IdealPoint) -> Scalar {
    var self_1257: IdealPoint;
    var other_1097: IdealPoint;

    self_1257 = self_1256;
    other_1097 = other_1096;
    let _e5: IdealPoint = self_1257;
    let _e8: IdealPoint = other_1097;
    let _e13: IdealPoint = self_1257;
    let _e16: IdealPoint = other_1097;
    return Scalar(((0.0 - (_e5.g0_.x * _e8.g0_.x)) - (_e13.g0_.y * _e16.g0_.y)));
}

fn ideal_point_ideal_point_left_anti_contraction(self_1258: IdealPoint, other_1098: IdealPoint) -> AntiScalar {
    var self_1259: IdealPoint;
    var other_1099: IdealPoint;

    self_1259 = self_1258;
    other_1099 = other_1098;
    let _e4: IdealPoint = self_1259;
    let _e7: IdealPoint = other_1099;
    let _e11: IdealPoint = self_1259;
    let _e14: IdealPoint = other_1099;
    return AntiScalar(((_e4.g0_.x * _e7.g0_.x) + (_e11.g0_.y * _e14.g0_.y)));
}

fn ideal_point_ideal_point_right_anti_contraction(self_1260: IdealPoint, other_1100: IdealPoint) -> AntiScalar {
    var self_1261: IdealPoint;
    var other_1101: IdealPoint;

    self_1261 = self_1260;
    other_1101 = other_1100;
    let _e4: IdealPoint = self_1261;
    let _e7: IdealPoint = other_1101;
    let _e11: IdealPoint = self_1261;
    let _e14: IdealPoint = other_1101;
    return AntiScalar(((_e4.g0_.x * _e7.g0_.x) + (_e11.g0_.y * _e14.g0_.y)));
}

fn ideal_point_ideal_point_scalar_product(self_1262: IdealPoint, other_1102: IdealPoint) -> Scalar {
    var self_1263: IdealPoint;
    var other_1103: IdealPoint;

    self_1263 = self_1262;
    other_1103 = other_1102;
    let _e5: IdealPoint = self_1263;
    let _e8: IdealPoint = other_1103;
    let _e13: IdealPoint = self_1263;
    let _e16: IdealPoint = other_1103;
    return Scalar(((0.0 - (_e5.g0_.x * _e8.g0_.x)) - (_e13.g0_.y * _e16.g0_.y)));
}

fn ideal_point_ideal_point_anti_scalar_product(self_1264: IdealPoint, other_1104: IdealPoint) -> AntiScalar {
    var self_1265: IdealPoint;
    var other_1105: IdealPoint;

    self_1265 = self_1264;
    other_1105 = other_1104;
    let _e4: IdealPoint = self_1265;
    let _e7: IdealPoint = other_1105;
    let _e11: IdealPoint = self_1265;
    let _e14: IdealPoint = other_1105;
    return AntiScalar(((_e4.g0_.x * _e7.g0_.x) + (_e11.g0_.y * _e14.g0_.y)));
}

fn ideal_point_plane_geometric_product(self_1266: IdealPoint, other_1106: Plane) -> MotorDual {
    var self_1267: IdealPoint;
    var other_1107: Plane;

    self_1267 = self_1266;
    other_1107 = other_1106;
    let _e4: IdealPoint = self_1267;
    let _e8: Plane = other_1107;
    let _e11: Plane = other_1107;
    let _e14: Plane = other_1107;
    let _e17: Plane = other_1107;
    let _e29: IdealPoint = self_1267;
    let _e33: Plane = other_1107;
    let _e36: Plane = other_1107;
    let _e39: Plane = other_1107;
    let _e42: Plane = other_1107;
    return MotorDual((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.z, _e11.g0_.y, _e14.g0_.x, _e17.g0_.z)) * vec4<f32>(1.0, -(1.0), 1.0, 0.0)) + ((vec4<f32>(_e29.g0_.x) * vec4<f32>(_e33.g0_.y, _e36.g0_.z, _e39.g0_.x, _e42.g0_.x)) * vec4<f32>(1.0, 1.0, 0.0, -(1.0)))));
}

fn ideal_point_plane_regressive_product(self_1268: IdealPoint, other_1108: Plane) -> Scalar {
    var self_1269: IdealPoint;
    var other_1109: Plane;

    self_1269 = self_1268;
    other_1109 = other_1108;
    let _e4: IdealPoint = self_1269;
    let _e7: Plane = other_1109;
    let _e11: IdealPoint = self_1269;
    let _e14: Plane = other_1109;
    return Scalar(((_e4.g0_.x * _e7.g0_.y) + (_e11.g0_.y * _e14.g0_.z)));
}

fn ideal_point_plane_outer_product(self_1270: IdealPoint, other_1110: Plane) -> AntiScalar {
    var self_1271: IdealPoint;
    var other_1111: Plane;

    self_1271 = self_1270;
    other_1111 = other_1110;
    let _e4: IdealPoint = self_1271;
    let _e7: Plane = other_1111;
    let _e11: IdealPoint = self_1271;
    let _e14: Plane = other_1111;
    return AntiScalar(((_e4.g0_.x * _e7.g0_.y) + (_e11.g0_.y * _e14.g0_.z)));
}

fn ideal_point_plane_inner_product(self_1272: IdealPoint, other_1112: Plane) -> Plane {
    var self_1273: IdealPoint;
    var other_1113: Plane;

    self_1273 = self_1272;
    other_1113 = other_1112;
    let _e4: IdealPoint = self_1273;
    let _e8: Plane = other_1113;
    let _e18: IdealPoint = self_1273;
    let _e22: Plane = other_1113;
    return Plane((((vec3<f32>(_e4.g0_.y) * _e8.g0_.yxy) * vec3<f32>(-(1.0), 1.0, 0.0)) + ((vec3<f32>(_e18.g0_.x) * _e22.g0_.zxx) * vec3<f32>(1.0, 0.0, -(1.0)))));
}

fn ideal_point_plane_geometric_anti_product(self_1274: IdealPoint, other_1114: Plane) -> Motor {
    var self_1275: IdealPoint;
    var other_1115: Plane;

    self_1275 = self_1274;
    other_1115 = other_1114;
    let _e4: IdealPoint = self_1275;
    let _e8: Plane = other_1115;
    let _e11: Plane = other_1115;
    let _e14: Plane = other_1115;
    let _e17: Plane = other_1115;
    let _e29: IdealPoint = self_1275;
    let _e33: Plane = other_1115;
    let _e36: Plane = other_1115;
    let _e39: Plane = other_1115;
    let _e42: Plane = other_1115;
    return Motor((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.z, _e11.g0_.y, _e14.g0_.x, _e17.g0_.z)) * vec4<f32>(1.0, -(1.0), 1.0, 0.0)) + ((vec4<f32>(_e29.g0_.x) * vec4<f32>(_e33.g0_.y, _e36.g0_.z, _e39.g0_.x, _e42.g0_.x)) * vec4<f32>(1.0, 1.0, 0.0, -(1.0)))));
}

fn ideal_point_plane_inner_anti_product(self_1276: IdealPoint, other_1116: Plane) -> Point {
    var self_1277: IdealPoint;
    var other_1117: Plane;

    self_1277 = self_1276;
    other_1117 = other_1116;
    let _e4: IdealPoint = self_1277;
    let _e8: Plane = other_1117;
    let _e18: IdealPoint = self_1277;
    let _e22: Plane = other_1117;
    return Point((((vec3<f32>(_e4.g0_.y) * _e8.g0_.yxy) * vec3<f32>(-(1.0), 1.0, 0.0)) + ((vec3<f32>(_e18.g0_.x) * _e22.g0_.zxx) * vec3<f32>(1.0, 0.0, -(1.0)))));
}

fn ideal_point_plane_right_contraction(self_1278: IdealPoint, other_1118: Plane) -> Plane {
    var self_1279: IdealPoint;
    var other_1119: Plane;

    self_1279 = self_1278;
    other_1119 = other_1118;
    let _e4: IdealPoint = self_1279;
    let _e8: Plane = other_1119;
    let _e18: IdealPoint = self_1279;
    let _e22: Plane = other_1119;
    return Plane((((vec3<f32>(_e4.g0_.y) * _e8.g0_.yxy) * vec3<f32>(-(1.0), 1.0, 0.0)) + ((vec3<f32>(_e18.g0_.x) * _e22.g0_.zxx) * vec3<f32>(1.0, 0.0, -(1.0)))));
}

fn ideal_point_plane_left_anti_contraction(self_1280: IdealPoint, other_1120: Plane) -> Point {
    var self_1281: IdealPoint;
    var other_1121: Plane;

    self_1281 = self_1280;
    other_1121 = other_1120;
    let _e4: IdealPoint = self_1281;
    let _e8: Plane = other_1121;
    let _e18: IdealPoint = self_1281;
    let _e22: Plane = other_1121;
    return Point((((vec3<f32>(_e4.g0_.y) * _e8.g0_.yxy) * vec3<f32>(-(1.0), 1.0, 0.0)) + ((vec3<f32>(_e18.g0_.x) * _e22.g0_.zxx) * vec3<f32>(1.0, 0.0, -(1.0)))));
}

fn ideal_point_translator_add(self_1282: IdealPoint, other_1122: Translator) -> Translator {
    var self_1283: IdealPoint;
    var other_1123: Translator;

    self_1283 = self_1282;
    other_1123 = other_1122;
    let _e4: IdealPoint = self_1283;
    let _e7: IdealPoint = self_1283;
    let _e10: IdealPoint = self_1283;
    let _e19: Translator = other_1123;
    return Translator(((vec3<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.y) * vec3<f32>(0.0, 1.0, 1.0)) + _e19.g0_));
}

fn ideal_point_translator_sub(self_1284: IdealPoint, other_1124: Translator) -> Translator {
    var self_1285: IdealPoint;
    var other_1125: Translator;

    self_1285 = self_1284;
    other_1125 = other_1124;
    let _e4: IdealPoint = self_1285;
    let _e7: IdealPoint = self_1285;
    let _e10: IdealPoint = self_1285;
    let _e19: Translator = other_1125;
    return Translator(((vec3<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.y) * vec3<f32>(0.0, 1.0, 1.0)) - _e19.g0_));
}

fn ideal_point_translator_geometric_product(self_1286: IdealPoint, other_1126: Translator) -> Motor {
    var self_1287: IdealPoint;
    var other_1127: Translator;

    self_1287 = self_1286;
    other_1127 = other_1126;
    let _e4: IdealPoint = self_1287;
    let _e8: Translator = other_1127;
    let _e11: Translator = other_1127;
    let _e14: Translator = other_1127;
    let _e17: Translator = other_1127;
    let _e30: IdealPoint = self_1287;
    let _e34: Translator = other_1127;
    let _e37: Translator = other_1127;
    let _e40: Translator = other_1127;
    let _e43: Translator = other_1127;
    return Motor((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.z, _e11.g0_.y, _e14.g0_.z, _e17.g0_.x)) * vec4<f32>(-(1.0), -(1.0), 0.0, 1.0)) + ((vec4<f32>(_e30.g0_.x) * vec4<f32>(_e34.g0_.y, _e37.g0_.z, _e40.g0_.x, _e43.g0_.x)) * vec4<f32>(-(1.0), 1.0, 1.0, 0.0))));
}

fn ideal_point_translator_outer_product(self_1288: IdealPoint, other_1128: Translator) -> IdealPoint {
    var self_1289: IdealPoint;
    var other_1129: Translator;

    self_1289 = self_1288;
    other_1129 = other_1128;
    let _e4: IdealPoint = self_1289;
    let _e6: Translator = other_1129;
    return IdealPoint((_e4.g0_ * vec2<f32>(_e6.g0_.x)));
}

fn ideal_point_translator_inner_product(self_1290: IdealPoint, other_1130: Translator) -> Translator {
    var self_1291: IdealPoint;
    var other_1131: Translator;

    self_1291 = self_1290;
    other_1131 = other_1130;
    let _e4: IdealPoint = self_1291;
    let _e8: Translator = other_1131;
    let _e18: IdealPoint = self_1291;
    let _e22: Translator = other_1131;
    return Translator((((vec3<f32>(_e4.g0_.y) * _e8.g0_.zzx) * vec3<f32>(-(1.0), 0.0, 1.0)) + ((vec3<f32>(_e18.g0_.x) * _e22.g0_.yxx) * vec3<f32>(-(1.0), 1.0, 0.0))));
}

fn ideal_point_translator_geometric_anti_product(self_1292: IdealPoint, other_1132: Translator) -> MotorDual {
    var self_1293: IdealPoint;
    var other_1133: Translator;

    self_1293 = self_1292;
    other_1133 = other_1132;
    let _e4: IdealPoint = self_1293;
    let _e8: Translator = other_1133;
    let _e11: Translator = other_1133;
    let _e14: Translator = other_1133;
    let _e17: Translator = other_1133;
    let _e28: IdealPoint = self_1293;
    let _e32: Translator = other_1133;
    let _e35: Translator = other_1133;
    let _e38: Translator = other_1133;
    let _e41: Translator = other_1133;
    return MotorDual((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.z, _e11.g0_.y, _e14.g0_.z, _e17.g0_.x)) * vec4<f32>(1.0, 1.0, 0.0, 1.0)) + ((vec4<f32>(_e28.g0_.x) * vec4<f32>(_e32.g0_.y, _e35.g0_.z, _e38.g0_.x, _e41.g0_.x)) * vec4<f32>(1.0, -(1.0), 1.0, 0.0))));
}

fn ideal_point_translator_left_contraction(self_1294: IdealPoint, other_1134: Translator) -> Scalar {
    var self_1295: IdealPoint;
    var other_1135: Translator;

    self_1295 = self_1294;
    other_1135 = other_1134;
    let _e5: IdealPoint = self_1295;
    let _e8: Translator = other_1135;
    let _e13: IdealPoint = self_1295;
    let _e16: Translator = other_1135;
    return Scalar(((0.0 - (_e5.g0_.x * _e8.g0_.y)) - (_e13.g0_.y * _e16.g0_.z)));
}

fn ideal_point_translator_right_contraction(self_1296: IdealPoint, other_1136: Translator) -> Translator {
    var self_1297: IdealPoint;
    var other_1137: Translator;

    self_1297 = self_1296;
    other_1137 = other_1136;
    let _e4: IdealPoint = self_1297;
    let _e8: Translator = other_1137;
    let _e18: IdealPoint = self_1297;
    let _e22: Translator = other_1137;
    return Translator((((vec3<f32>(_e4.g0_.y) * _e8.g0_.zzx) * vec3<f32>(-(1.0), 0.0, 1.0)) + ((vec3<f32>(_e18.g0_.x) * _e22.g0_.yxx) * vec3<f32>(-(1.0), 1.0, 0.0))));
}

fn ideal_point_translator_right_anti_contraction(self_1298: IdealPoint, other_1138: Translator) -> AntiScalar {
    var self_1299: IdealPoint;
    var other_1139: Translator;

    self_1299 = self_1298;
    other_1139 = other_1138;
    let _e4: IdealPoint = self_1299;
    let _e7: Translator = other_1139;
    let _e11: IdealPoint = self_1299;
    let _e14: Translator = other_1139;
    return AntiScalar(((_e4.g0_.x * _e7.g0_.y) + (_e11.g0_.y * _e14.g0_.z)));
}

fn ideal_point_translator_scalar_product(self_1300: IdealPoint, other_1140: Translator) -> Scalar {
    var self_1301: IdealPoint;
    var other_1141: Translator;

    self_1301 = self_1300;
    other_1141 = other_1140;
    let _e5: IdealPoint = self_1301;
    let _e8: Translator = other_1141;
    let _e13: IdealPoint = self_1301;
    let _e16: Translator = other_1141;
    return Scalar(((0.0 - (_e5.g0_.x * _e8.g0_.y)) - (_e13.g0_.y * _e16.g0_.z)));
}

fn ideal_point_translator_anti_scalar_product(self_1302: IdealPoint, other_1142: Translator) -> AntiScalar {
    var self_1303: IdealPoint;
    var other_1143: Translator;

    self_1303 = self_1302;
    other_1143 = other_1142;
    let _e4: IdealPoint = self_1303;
    let _e7: Translator = other_1143;
    let _e11: IdealPoint = self_1303;
    let _e14: Translator = other_1143;
    return AntiScalar(((_e4.g0_.x * _e7.g0_.y) + (_e11.g0_.y * _e14.g0_.z)));
}

fn ideal_point_motor_add(self_1304: IdealPoint, other_1144: Motor) -> Motor {
    var self_1305: IdealPoint;
    var other_1145: Motor;

    self_1305 = self_1304;
    other_1145 = other_1144;
    let _e4: IdealPoint = self_1305;
    let _e7: IdealPoint = self_1305;
    let _e10: IdealPoint = self_1305;
    let _e13: IdealPoint = self_1305;
    let _e23: Motor = other_1145;
    return Motor(((vec4<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.x, _e13.g0_.y) * vec4<f32>(0.0, 0.0, 1.0, 1.0)) + _e23.g0_));
}

fn ideal_point_motor_sub(self_1306: IdealPoint, other_1146: Motor) -> Motor {
    var self_1307: IdealPoint;
    var other_1147: Motor;

    self_1307 = self_1306;
    other_1147 = other_1146;
    let _e4: IdealPoint = self_1307;
    let _e7: IdealPoint = self_1307;
    let _e10: IdealPoint = self_1307;
    let _e13: IdealPoint = self_1307;
    let _e23: Motor = other_1147;
    return Motor(((vec4<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.x, _e13.g0_.y) * vec4<f32>(0.0, 0.0, 1.0, 1.0)) - _e23.g0_));
}

fn ideal_point_motor_geometric_product(self_1308: IdealPoint, other_1148: Motor) -> Motor {
    var self_1309: IdealPoint;
    var other_1149: Motor;

    self_1309 = self_1308;
    other_1149 = other_1148;
    let _e4: IdealPoint = self_1309;
    let _e8: Motor = other_1149;
    let _e20: IdealPoint = self_1309;
    let _e24: Motor = other_1149;
    return Motor((((vec4<f32>(_e4.g0_.x) * _e8.g0_.zwxy) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0))) + ((vec4<f32>(_e20.g0_.y) * _e24.g0_.wzyx) * vec4<f32>(-(1.0), -(1.0), 1.0, 1.0))));
}

fn ideal_point_motor_regressive_product(self_1310: IdealPoint, other_1150: Motor) -> Plane {
    var self_1311: IdealPoint;
    var other_1151: Motor;

    self_1311 = self_1310;
    other_1151 = other_1150;
    let _e4: IdealPoint = self_1311;
    let _e8: Motor = other_1151;
    let _e11: Motor = other_1151;
    let _e14: Motor = other_1151;
    let _e25: IdealPoint = self_1311;
    let _e29: Motor = other_1151;
    let _e32: Motor = other_1151;
    let _e35: Motor = other_1151;
    return Plane((((vec3<f32>(_e4.g0_.y) * vec3<f32>(_e8.g0_.z, _e11.g0_.y, _e14.g0_.z)) * vec3<f32>(1.0, -(1.0), 0.0)) + ((vec3<f32>(_e25.g0_.x) * vec3<f32>(_e29.g0_.w, _e32.g0_.x, _e35.g0_.y)) * vec3<f32>(-(1.0), 0.0, 1.0))));
}

fn ideal_point_motor_outer_product(self_1312: IdealPoint, other_1152: Motor) -> IdealPoint {
    var self_1313: IdealPoint;
    var other_1153: Motor;

    self_1313 = self_1312;
    other_1153 = other_1152;
    let _e4: IdealPoint = self_1313;
    let _e6: Motor = other_1153;
    return IdealPoint((_e4.g0_ * vec2<f32>(_e6.g0_.x)));
}

fn ideal_point_motor_inner_product(self_1314: IdealPoint, other_1154: Motor) -> Translator {
    var self_1315: IdealPoint;
    var other_1155: Motor;

    self_1315 = self_1314;
    other_1155 = other_1154;
    let _e4: IdealPoint = self_1315;
    let _e8: Motor = other_1155;
    let _e11: Motor = other_1155;
    let _e14: Motor = other_1155;
    let _e25: IdealPoint = self_1315;
    let _e29: Motor = other_1155;
    let _e32: Motor = other_1155;
    let _e35: Motor = other_1155;
    return Translator((((vec3<f32>(_e4.g0_.y) * vec3<f32>(_e8.g0_.w, _e11.g0_.w, _e14.g0_.x)) * vec3<f32>(-(1.0), 0.0, 1.0)) + ((vec3<f32>(_e25.g0_.x) * vec3<f32>(_e29.g0_.z, _e32.g0_.x, _e35.g0_.x)) * vec3<f32>(-(1.0), 1.0, 0.0))));
}

fn ideal_point_motor_geometric_anti_product(self_1316: IdealPoint, other_1156: Motor) -> MotorDual {
    var self_1317: IdealPoint;
    var other_1157: Motor;

    self_1317 = self_1316;
    other_1157 = other_1156;
    let _e4: IdealPoint = self_1317;
    let _e8: Motor = other_1157;
    let _e19: IdealPoint = self_1317;
    let _e23: Motor = other_1157;
    return MotorDual((((vec4<f32>(_e4.g0_.x) * _e8.g0_.zwxy) * vec4<f32>(1.0, -(1.0), 1.0, 1.0)) + ((vec4<f32>(_e19.g0_.y) * _e23.g0_.wzyx) * vec4<f32>(1.0, 1.0, -(1.0), 1.0))));
}

fn ideal_point_motor_left_contraction(self_1318: IdealPoint, other_1158: Motor) -> Scalar {
    var self_1319: IdealPoint;
    var other_1159: Motor;

    self_1319 = self_1318;
    other_1159 = other_1158;
    let _e5: IdealPoint = self_1319;
    let _e8: Motor = other_1159;
    let _e13: IdealPoint = self_1319;
    let _e16: Motor = other_1159;
    return Scalar(((0.0 - (_e5.g0_.x * _e8.g0_.z)) - (_e13.g0_.y * _e16.g0_.w)));
}

fn ideal_point_motor_right_contraction(self_1320: IdealPoint, other_1160: Motor) -> Translator {
    var self_1321: IdealPoint;
    var other_1161: Motor;

    self_1321 = self_1320;
    other_1161 = other_1160;
    let _e4: IdealPoint = self_1321;
    let _e8: Motor = other_1161;
    let _e11: Motor = other_1161;
    let _e14: Motor = other_1161;
    let _e25: IdealPoint = self_1321;
    let _e29: Motor = other_1161;
    let _e32: Motor = other_1161;
    let _e35: Motor = other_1161;
    return Translator((((vec3<f32>(_e4.g0_.y) * vec3<f32>(_e8.g0_.w, _e11.g0_.w, _e14.g0_.x)) * vec3<f32>(-(1.0), 0.0, 1.0)) + ((vec3<f32>(_e25.g0_.x) * vec3<f32>(_e29.g0_.z, _e32.g0_.x, _e35.g0_.x)) * vec3<f32>(-(1.0), 1.0, 0.0))));
}

fn ideal_point_motor_right_anti_contraction(self_1322: IdealPoint, other_1162: Motor) -> AntiScalar {
    var self_1323: IdealPoint;
    var other_1163: Motor;

    self_1323 = self_1322;
    other_1163 = other_1162;
    let _e4: IdealPoint = self_1323;
    let _e7: Motor = other_1163;
    let _e11: IdealPoint = self_1323;
    let _e14: Motor = other_1163;
    return AntiScalar(((_e4.g0_.x * _e7.g0_.z) + (_e11.g0_.y * _e14.g0_.w)));
}

fn ideal_point_motor_scalar_product(self_1324: IdealPoint, other_1164: Motor) -> Scalar {
    var self_1325: IdealPoint;
    var other_1165: Motor;

    self_1325 = self_1324;
    other_1165 = other_1164;
    let _e5: IdealPoint = self_1325;
    let _e8: Motor = other_1165;
    let _e13: IdealPoint = self_1325;
    let _e16: Motor = other_1165;
    return Scalar(((0.0 - (_e5.g0_.x * _e8.g0_.z)) - (_e13.g0_.y * _e16.g0_.w)));
}

fn ideal_point_motor_anti_scalar_product(self_1326: IdealPoint, other_1166: Motor) -> AntiScalar {
    var self_1327: IdealPoint;
    var other_1167: Motor;

    self_1327 = self_1326;
    other_1167 = other_1166;
    let _e4: IdealPoint = self_1327;
    let _e7: Motor = other_1167;
    let _e11: IdealPoint = self_1327;
    let _e14: Motor = other_1167;
    return AntiScalar(((_e4.g0_.x * _e7.g0_.z) + (_e11.g0_.y * _e14.g0_.w)));
}

fn ideal_point_motor_dual_geometric_product(self_1328: IdealPoint, other_1168: MotorDual) -> MotorDual {
    var self_1329: IdealPoint;
    var other_1169: MotorDual;

    self_1329 = self_1328;
    other_1169 = other_1168;
    let _e4: IdealPoint = self_1329;
    let _e8: MotorDual = other_1169;
    let _e20: IdealPoint = self_1329;
    let _e24: MotorDual = other_1169;
    return MotorDual((((vec4<f32>(_e4.g0_.x) * _e8.g0_.zwxy) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0))) + ((vec4<f32>(_e20.g0_.y) * _e24.g0_.wzyx) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))));
}

fn ideal_point_motor_dual_regressive_product(self_1330: IdealPoint, other_1170: MotorDual) -> Translator {
    var self_1331: IdealPoint;
    var other_1171: MotorDual;

    self_1331 = self_1330;
    other_1171 = other_1170;
    let _e4: IdealPoint = self_1331;
    let _e8: MotorDual = other_1171;
    let _e11: MotorDual = other_1171;
    let _e14: MotorDual = other_1171;
    let _e24: IdealPoint = self_1331;
    let _e28: MotorDual = other_1171;
    let _e31: MotorDual = other_1171;
    let _e34: MotorDual = other_1171;
    return Translator((((vec3<f32>(_e4.g0_.y) * vec3<f32>(_e8.g0_.w, _e11.g0_.w, _e14.g0_.x)) * vec3<f32>(1.0, 0.0, 1.0)) + ((vec3<f32>(_e24.g0_.x) * vec3<f32>(_e28.g0_.z, _e31.g0_.x, _e34.g0_.x)) * vec3<f32>(1.0, 1.0, 0.0))));
}

fn ideal_point_motor_dual_outer_product(self_1332: IdealPoint, other_1172: MotorDual) -> AntiScalar {
    var self_1333: IdealPoint;
    var other_1173: MotorDual;

    self_1333 = self_1332;
    other_1173 = other_1172;
    let _e4: IdealPoint = self_1333;
    let _e7: MotorDual = other_1173;
    let _e11: IdealPoint = self_1333;
    let _e14: MotorDual = other_1173;
    return AntiScalar(((_e4.g0_.x * _e7.g0_.z) + (_e11.g0_.y * _e14.g0_.w)));
}

fn ideal_point_motor_dual_inner_product(self_1334: IdealPoint, other_1174: MotorDual) -> Plane {
    var self_1335: IdealPoint;
    var other_1175: MotorDual;

    self_1335 = self_1334;
    other_1175 = other_1174;
    let _e4: IdealPoint = self_1335;
    let _e8: MotorDual = other_1175;
    let _e11: MotorDual = other_1175;
    let _e14: MotorDual = other_1175;
    let _e26: IdealPoint = self_1335;
    let _e30: MotorDual = other_1175;
    let _e33: MotorDual = other_1175;
    let _e36: MotorDual = other_1175;
    return Plane((((vec3<f32>(_e4.g0_.x) * vec3<f32>(_e8.g0_.w, _e11.g0_.x, _e14.g0_.y)) * vec3<f32>(1.0, -(1.0), -(1.0))) + ((vec3<f32>(_e26.g0_.y) * vec3<f32>(_e30.g0_.z, _e33.g0_.y, _e36.g0_.x)) * vec3<f32>(-(1.0), 1.0, -(1.0)))));
}

fn ideal_point_motor_dual_geometric_anti_product(self_1336: IdealPoint, other_1176: MotorDual) -> Motor {
    var self_1337: IdealPoint;
    var other_1177: MotorDual;

    self_1337 = self_1336;
    other_1177 = other_1176;
    let _e4: IdealPoint = self_1337;
    let _e8: MotorDual = other_1177;
    let _e19: IdealPoint = self_1337;
    let _e23: MotorDual = other_1177;
    return Motor((((vec4<f32>(_e4.g0_.x) * _e8.g0_.zwxy) * vec4<f32>(1.0, 1.0, 1.0, -(1.0))) + ((vec4<f32>(_e19.g0_.y) * _e23.g0_.wzyx) * vec4<f32>(1.0, -(1.0), 1.0, 1.0))));
}

fn ideal_point_motor_dual_inner_anti_product(self_1338: IdealPoint, other_1178: MotorDual) -> Point {
    var self_1339: IdealPoint;
    var other_1179: MotorDual;

    self_1339 = self_1338;
    other_1179 = other_1178;
    let _e4: IdealPoint = self_1339;
    let _e8: MotorDual = other_1179;
    let _e11: MotorDual = other_1179;
    let _e14: MotorDual = other_1179;
    let _e25: IdealPoint = self_1339;
    let _e29: MotorDual = other_1179;
    let _e32: MotorDual = other_1179;
    let _e35: MotorDual = other_1179;
    return Point((((vec3<f32>(_e4.g0_.x) * vec3<f32>(_e8.g0_.w, _e11.g0_.x, _e14.g0_.y)) * vec3<f32>(1.0, 1.0, -(1.0))) + ((vec3<f32>(_e25.g0_.y) * vec3<f32>(_e29.g0_.z, _e32.g0_.y, _e35.g0_.x)) * vec3<f32>(-(1.0), 1.0, 1.0))));
}

fn ideal_point_motor_dual_right_contraction(self_1340: IdealPoint, other_1180: MotorDual) -> Plane {
    var self_1341: IdealPoint;
    var other_1181: MotorDual;

    self_1341 = self_1340;
    other_1181 = other_1180;
    let _e4: IdealPoint = self_1341;
    let _e8: MotorDual = other_1181;
    let _e11: MotorDual = other_1181;
    let _e14: MotorDual = other_1181;
    let _e25: IdealPoint = self_1341;
    let _e29: MotorDual = other_1181;
    let _e32: MotorDual = other_1181;
    let _e35: MotorDual = other_1181;
    return Plane((((vec3<f32>(_e4.g0_.y) * vec3<f32>(_e8.g0_.z, _e11.g0_.y, _e14.g0_.z)) * vec3<f32>(-(1.0), 1.0, 0.0)) + ((vec3<f32>(_e25.g0_.x) * vec3<f32>(_e29.g0_.w, _e32.g0_.x, _e35.g0_.y)) * vec3<f32>(1.0, 0.0, -(1.0)))));
}

fn ideal_point_motor_dual_left_anti_contraction(self_1342: IdealPoint, other_1182: MotorDual) -> Point {
    var self_1343: IdealPoint;
    var other_1183: MotorDual;

    self_1343 = self_1342;
    other_1183 = other_1182;
    let _e4: IdealPoint = self_1343;
    let _e8: MotorDual = other_1183;
    let _e11: MotorDual = other_1183;
    let _e14: MotorDual = other_1183;
    let _e25: IdealPoint = self_1343;
    let _e29: MotorDual = other_1183;
    let _e32: MotorDual = other_1183;
    let _e35: MotorDual = other_1183;
    return Point((((vec3<f32>(_e4.g0_.y) * vec3<f32>(_e8.g0_.z, _e11.g0_.y, _e14.g0_.z)) * vec3<f32>(-(1.0), 1.0, 0.0)) + ((vec3<f32>(_e25.g0_.x) * vec3<f32>(_e29.g0_.w, _e32.g0_.x, _e35.g0_.y)) * vec3<f32>(1.0, 0.0, -(1.0)))));
}

fn ideal_point_motor_dual_right_anti_contraction(self_1344: IdealPoint, other_1184: MotorDual) -> IdealPoint {
    var self_1345: IdealPoint;
    var other_1185: MotorDual;

    self_1345 = self_1344;
    other_1185 = other_1184;
    let _e4: IdealPoint = self_1345;
    let _e6: MotorDual = other_1185;
    return IdealPoint((_e4.g0_ * vec2<f32>(_e6.g0_.x)));
}

fn ideal_point_squared_magnitude(self_1346: IdealPoint) -> Scalar {
    var self_1347: IdealPoint;

    self_1347 = self_1346;
    let _e2: IdealPoint = self_1347;
    let _e3: IdealPoint = self_1347;
    let _e4: IdealPoint = ideal_point_reversal(_e3);
    let _e5: Scalar = ideal_point_ideal_point_scalar_product(_e2, _e4);
    return _e5;
}

fn ideal_point_magnitude(self_1348: IdealPoint) -> Scalar {
    var self_1349: IdealPoint;

    self_1349 = self_1348;
    let _e2: IdealPoint = self_1349;
    let _e3: Scalar = ideal_point_squared_magnitude(_e2);
    return Scalar(sqrt(_e3.g0_));
}

fn ideal_point_bulk_norm(self_1350: IdealPoint) -> Scalar {
    var self_1351: IdealPoint;

    self_1351 = self_1350;
    let _e2: IdealPoint = self_1351;
    let _e3: Scalar = ideal_point_squared_magnitude(_e2);
    return Scalar(sqrt(_e3.g0_));
}

fn ideal_point_squared_anti_magnitude(self_1352: IdealPoint) -> AntiScalar {
    var self_1353: IdealPoint;

    self_1353 = self_1352;
    let _e2: IdealPoint = self_1353;
    let _e3: IdealPoint = self_1353;
    let _e4: IdealPoint = ideal_point_anti_reversal(_e3);
    let _e5: AntiScalar = ideal_point_ideal_point_anti_scalar_product(_e2, _e4);
    return _e5;
}

fn ideal_point_weight_norm(self_1354: IdealPoint) -> AntiScalar {
    var self_1355: IdealPoint;

    self_1355 = self_1354;
    let _e2: IdealPoint = self_1355;
    let _e3: AntiScalar = ideal_point_squared_anti_magnitude(_e2);
    return AntiScalar(sqrt(_e3.g0_));
}

fn ideal_point_scale(self_1356: IdealPoint, other_1186: f32) -> IdealPoint {
    var self_1357: IdealPoint;
    var other_1187: f32;

    self_1357 = self_1356;
    other_1187 = other_1186;
    let _e4: IdealPoint = self_1357;
    let _e5: f32 = other_1187;
    let _e7: IdealPoint = ideal_point_scalar_geometric_product(_e4, Scalar(_e5));
    return _e7;
}

fn ideal_point_signum(self_1358: IdealPoint) -> IdealPoint {
    var self_1359: IdealPoint;

    self_1359 = self_1358;
    let _e2: IdealPoint = self_1359;
    let _e3: IdealPoint = self_1359;
    let _e4: Scalar = ideal_point_magnitude(_e3);
    let _e9: IdealPoint = ideal_point_scalar_geometric_product(_e2, Scalar((1.0 / _e4.g0_)));
    return _e9;
}

fn ideal_point_inverse(self_1360: IdealPoint) -> IdealPoint {
    var self_1361: IdealPoint;

    self_1361 = self_1360;
    let _e2: IdealPoint = self_1361;
    let _e3: IdealPoint = ideal_point_reversal(_e2);
    let _e4: IdealPoint = self_1361;
    let _e5: Scalar = ideal_point_squared_magnitude(_e4);
    let _e10: IdealPoint = ideal_point_scalar_geometric_product(_e3, Scalar((1.0 / _e5.g0_)));
    return _e10;
}

fn plane_zero() -> Plane {
    return Plane(vec3<f32>(0.0));
}

fn plane_one() -> Plane {
    return Plane(vec3<f32>(0.0));
}

fn plane_neg(self_1362: Plane) -> Plane {
    var self_1363: Plane;

    self_1363 = self_1362;
    let _e2: Plane = self_1363;
    return Plane((_e2.g0_ * vec3<f32>(-(1.0))));
}

fn plane_automorphism(self_1364: Plane) -> Plane {
    var self_1365: Plane;

    self_1365 = self_1364;
    let _e2: Plane = self_1365;
    return Plane((_e2.g0_ * vec3<f32>(-(1.0))));
}

fn plane_reversal(self_1366: Plane) -> Plane {
    var self_1367: Plane;

    self_1367 = self_1366;
    let _e2: Plane = self_1367;
    return Plane(_e2.g0_);
}

fn plane_conjugation(self_1368: Plane) -> Plane {
    var self_1369: Plane;

    self_1369 = self_1368;
    let _e2: Plane = self_1369;
    return Plane((_e2.g0_ * vec3<f32>(-(1.0))));
}

fn plane_dual(self_1370: Plane) -> Point {
    var self_1371: Plane;

    self_1371 = self_1370;
    let _e2: Plane = self_1371;
    return Point(_e2.g0_);
}

fn plane_anti_reversal(self_1372: Plane) -> Plane {
    var self_1373: Plane;

    self_1373 = self_1372;
    let _e2: Plane = self_1373;
    return Plane((_e2.g0_ * vec3<f32>(-(1.0))));
}

fn plane_scalar_geometric_product(self_1374: Plane, other_1188: Scalar) -> Plane {
    var self_1375: Plane;
    var other_1189: Scalar;

    self_1375 = self_1374;
    other_1189 = other_1188;
    let _e4: Plane = self_1375;
    let _e6: Scalar = other_1189;
    return Plane((_e4.g0_ * vec3<f32>(_e6.g0_)));
}

fn plane_scalar_outer_product(self_1376: Plane, other_1190: Scalar) -> Plane {
    var self_1377: Plane;
    var other_1191: Scalar;

    self_1377 = self_1376;
    other_1191 = other_1190;
    let _e4: Plane = self_1377;
    let _e6: Scalar = other_1191;
    return Plane((_e4.g0_ * vec3<f32>(_e6.g0_)));
}

fn plane_scalar_inner_product(self_1378: Plane, other_1192: Scalar) -> Plane {
    var self_1379: Plane;
    var other_1193: Scalar;

    self_1379 = self_1378;
    other_1193 = other_1192;
    let _e4: Plane = self_1379;
    let _e6: Scalar = other_1193;
    return Plane((_e4.g0_ * vec3<f32>(_e6.g0_)));
}

fn plane_scalar_geometric_anti_product(self_1380: Plane, other_1194: Scalar) -> Point {
    var self_1381: Plane;
    var other_1195: Scalar;

    self_1381 = self_1380;
    other_1195 = other_1194;
    let _e4: Plane = self_1381;
    let _e6: Scalar = other_1195;
    return Point(((_e4.g0_ * vec3<f32>(_e6.g0_)) * vec3<f32>(-(1.0))));
}

fn plane_scalar_inner_anti_product(self_1382: Plane, other_1196: Scalar) -> Point {
    var self_1383: Plane;
    var other_1197: Scalar;

    self_1383 = self_1382;
    other_1197 = other_1196;
    let _e4: Plane = self_1383;
    let _e6: Scalar = other_1197;
    return Point(((_e4.g0_ * vec3<f32>(_e6.g0_)) * vec3<f32>(-(1.0))));
}

fn plane_scalar_right_contraction(self_1384: Plane, other_1198: Scalar) -> Plane {
    var self_1385: Plane;
    var other_1199: Scalar;

    self_1385 = self_1384;
    other_1199 = other_1198;
    let _e4: Plane = self_1385;
    let _e6: Scalar = other_1199;
    return Plane((_e4.g0_ * vec3<f32>(_e6.g0_)));
}

fn plane_scalar_left_anti_contraction(self_1386: Plane, other_1200: Scalar) -> Point {
    var self_1387: Plane;
    var other_1201: Scalar;

    self_1387 = self_1386;
    other_1201 = other_1200;
    let _e4: Plane = self_1387;
    let _e6: Scalar = other_1201;
    return Point(((_e4.g0_ * vec3<f32>(_e6.g0_)) * vec3<f32>(-(1.0))));
}

fn plane_anti_scalar_add(self_1388: Plane, other_1202: AntiScalar) -> MotorDual {
    var self_1389: Plane;
    var other_1203: AntiScalar;

    self_1389 = self_1388;
    other_1203 = other_1202;
    let _e4: Plane = self_1389;
    let _e7: Plane = self_1389;
    let _e10: Plane = self_1389;
    let _e13: Plane = self_1389;
    let _e23: AntiScalar = other_1203;
    return MotorDual(((vec4<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.y, _e13.g0_.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0)) + (vec4<f32>(_e23.g0_) * vec4<f32>(1.0, 0.0, 0.0, 0.0))));
}

fn plane_anti_scalar_sub(self_1390: Plane, other_1204: AntiScalar) -> MotorDual {
    var self_1391: Plane;
    var other_1205: AntiScalar;

    self_1391 = self_1390;
    other_1205 = other_1204;
    let _e4: Plane = self_1391;
    let _e7: Plane = self_1391;
    let _e10: Plane = self_1391;
    let _e13: Plane = self_1391;
    let _e23: AntiScalar = other_1205;
    return MotorDual(((vec4<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.y, _e13.g0_.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0)) - (vec4<f32>(_e23.g0_) * vec4<f32>(1.0, 0.0, 0.0, 0.0))));
}

fn plane_anti_scalar_geometric_product(self_1392: Plane, other_1206: AntiScalar) -> Point {
    var self_1393: Plane;
    var other_1207: AntiScalar;

    self_1393 = self_1392;
    other_1207 = other_1206;
    let _e4: Plane = self_1393;
    let _e6: AntiScalar = other_1207;
    return Point((_e4.g0_ * vec3<f32>(_e6.g0_)));
}

fn plane_anti_scalar_regressive_product(self_1394: Plane, other_1208: AntiScalar) -> Plane {
    var self_1395: Plane;
    var other_1209: AntiScalar;

    self_1395 = self_1394;
    other_1209 = other_1208;
    let _e4: Plane = self_1395;
    let _e6: AntiScalar = other_1209;
    return Plane((_e4.g0_ * vec3<f32>(_e6.g0_)));
}

fn plane_anti_scalar_inner_product(self_1396: Plane, other_1210: AntiScalar) -> Point {
    var self_1397: Plane;
    var other_1211: AntiScalar;

    self_1397 = self_1396;
    other_1211 = other_1210;
    let _e4: Plane = self_1397;
    let _e6: AntiScalar = other_1211;
    return Point((_e4.g0_ * vec3<f32>(_e6.g0_)));
}

fn plane_anti_scalar_geometric_anti_product(self_1398: Plane, other_1212: AntiScalar) -> Plane {
    var self_1399: Plane;
    var other_1213: AntiScalar;

    self_1399 = self_1398;
    other_1213 = other_1212;
    let _e4: Plane = self_1399;
    let _e6: AntiScalar = other_1213;
    return Plane((_e4.g0_ * vec3<f32>(_e6.g0_)));
}

fn plane_anti_scalar_inner_anti_product(self_1400: Plane, other_1214: AntiScalar) -> Plane {
    var self_1401: Plane;
    var other_1215: AntiScalar;

    self_1401 = self_1400;
    other_1215 = other_1214;
    let _e4: Plane = self_1401;
    let _e6: AntiScalar = other_1215;
    return Plane((_e4.g0_ * vec3<f32>(_e6.g0_)));
}

fn plane_anti_scalar_left_contraction(self_1402: Plane, other_1216: AntiScalar) -> Point {
    var self_1403: Plane;
    var other_1217: AntiScalar;

    self_1403 = self_1402;
    other_1217 = other_1216;
    let _e4: Plane = self_1403;
    let _e6: AntiScalar = other_1217;
    return Point((_e4.g0_ * vec3<f32>(_e6.g0_)));
}

fn plane_anti_scalar_right_anti_contraction(self_1404: Plane, other_1218: AntiScalar) -> Plane {
    var self_1405: Plane;
    var other_1219: AntiScalar;

    self_1405 = self_1404;
    other_1219 = other_1218;
    let _e4: Plane = self_1405;
    let _e6: AntiScalar = other_1219;
    return Plane((_e4.g0_ * vec3<f32>(_e6.g0_)));
}

fn plane_multi_vector_add(self_1406: Plane, other_1220: MultiVector) -> MultiVector {
    var self_1407: Plane;
    var other_1221: MultiVector;

    self_1407 = self_1406;
    other_1221 = other_1220;
    let _e4: Plane = self_1407;
    let _e7: Plane = self_1407;
    let _e10: Plane = self_1407;
    let _e13: Plane = self_1407;
    let _e23: MultiVector = other_1221;
    let _e26: Plane = self_1407;
    let _e36: MultiVector = other_1221;
    return MultiVector(((vec4<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.z, _e13.g0_.y) * vec4<f32>(0.0, 0.0, 1.0, 1.0)) + _e23.g0_), ((vec4<f32>(_e26.g0_.x) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) + _e36.g1_));
}

fn plane_multi_vector_sub(self_1408: Plane, other_1222: MultiVector) -> MultiVector {
    var self_1409: Plane;
    var other_1223: MultiVector;

    self_1409 = self_1408;
    other_1223 = other_1222;
    let _e4: Plane = self_1409;
    let _e7: Plane = self_1409;
    let _e10: Plane = self_1409;
    let _e13: Plane = self_1409;
    let _e23: MultiVector = other_1223;
    let _e26: Plane = self_1409;
    let _e36: MultiVector = other_1223;
    return MultiVector(((vec4<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.z, _e13.g0_.y) * vec4<f32>(0.0, 0.0, 1.0, 1.0)) - _e23.g0_), ((vec4<f32>(_e26.g0_.x) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) - _e36.g1_));
}

fn plane_multi_vector_geometric_product(self_1410: Plane, other_1224: MultiVector) -> MultiVector {
    var self_1411: Plane;
    var other_1225: MultiVector;

    self_1411 = self_1410;
    other_1225 = other_1224;
    let _e4: Plane = self_1411;
    let _e8: MultiVector = other_1225;
    let _e18: Plane = self_1411;
    let _e22: MultiVector = other_1225;
    let _e35: Plane = self_1411;
    let _e39: MultiVector = other_1225;
    let _e44: Plane = self_1411;
    let _e48: MultiVector = other_1225;
    let _e58: Plane = self_1411;
    let _e62: MultiVector = other_1225;
    let _e67: Plane = self_1411;
    let _e71: MultiVector = other_1225;
    return MultiVector(((((vec4<f32>(_e4.g0_.x) * _e8.g1_) * vec4<f32>(1.0, 1.0, 1.0, -(1.0))) + ((vec4<f32>(_e18.g0_.y) * _e22.g0_.wzyx) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0))) + (vec4<f32>(_e35.g0_.z) * _e39.g0_.zwxy)), ((((vec4<f32>(_e44.g0_.x) * _e48.g0_) * vec4<f32>(1.0, 1.0, 1.0, -(1.0))) + (vec4<f32>(_e58.g0_.y) * _e62.g1_.wzyx)) + ((vec4<f32>(_e67.g0_.z) * _e71.g1_.zwxy) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))));
}

fn plane_multi_vector_geometric_anti_product(self_1412: Plane, other_1226: MultiVector) -> MultiVector {
    var self_1413: Plane;
    var other_1227: MultiVector;

    self_1413 = self_1412;
    other_1227 = other_1226;
    let _e4: Plane = self_1413;
    let _e8: MultiVector = other_1227;
    let _e20: Plane = self_1413;
    let _e24: MultiVector = other_1227;
    let _e36: Plane = self_1413;
    let _e40: MultiVector = other_1227;
    let _e52: Plane = self_1413;
    let _e56: MultiVector = other_1227;
    let _e68: Plane = self_1413;
    let _e72: MultiVector = other_1227;
    let _e86: Plane = self_1413;
    let _e90: MultiVector = other_1227;
    return MultiVector(((((vec4<f32>(_e4.g0_.x) * _e8.g0_.yxwz) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0))) + ((vec4<f32>(_e20.g0_.y) * _e24.g1_.zwxy) * vec4<f32>(1.0, 1.0, -(1.0), 1.0))) + ((vec4<f32>(_e36.g0_.z) * _e40.g1_.wzyx) * vec4<f32>(1.0, -(1.0), 1.0, 1.0))), ((((vec4<f32>(_e52.g0_.x) * _e56.g1_.yxwz) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0)) + ((vec4<f32>(_e68.g0_.y) * _e72.g0_.zwxy) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0)))) + ((vec4<f32>(_e86.g0_.z) * _e90.g0_.wzyx) * vec4<f32>(-(1.0), -(1.0), 1.0, -(1.0)))));
}

fn plane_multi_vector_scalar_product(self_1414: Plane, other_1228: MultiVector) -> Scalar {
    var self_1415: Plane;
    var other_1229: MultiVector;

    self_1415 = self_1414;
    other_1229 = other_1228;
    let _e4: Plane = self_1415;
    let _e7: MultiVector = other_1229;
    let _e11: Plane = self_1415;
    let _e14: MultiVector = other_1229;
    let _e19: Plane = self_1415;
    let _e22: MultiVector = other_1229;
    return Scalar((((_e4.g0_.x * _e7.g1_.x) + (_e11.g0_.y * _e14.g0_.w)) + (_e19.g0_.z * _e22.g0_.z)));
}

fn plane_multi_vector_anti_scalar_product(self_1416: Plane, other_1230: MultiVector) -> AntiScalar {
    var self_1417: Plane;
    var other_1231: MultiVector;

    self_1417 = self_1416;
    other_1231 = other_1230;
    let _e5: Plane = self_1417;
    let _e8: MultiVector = other_1231;
    let _e13: Plane = self_1417;
    let _e16: MultiVector = other_1231;
    let _e21: Plane = self_1417;
    let _e24: MultiVector = other_1231;
    return AntiScalar((((0.0 - (_e5.g0_.x * _e8.g1_.x)) - (_e13.g0_.y * _e16.g0_.w)) - (_e21.g0_.z * _e24.g0_.z)));
}

fn plane_rotor_geometric_product(self_1418: Plane, other_1232: Rotor) -> MotorDual {
    var self_1419: Plane;
    var other_1233: Rotor;

    self_1419 = self_1418;
    other_1233 = other_1232;
    let _e4: Plane = self_1419;
    let _e8: Rotor = other_1233;
    let _e11: Rotor = other_1233;
    let _e14: Rotor = other_1233;
    let _e17: Rotor = other_1233;
    let _e28: Plane = self_1419;
    let _e31: Plane = self_1419;
    let _e34: Plane = self_1419;
    let _e37: Plane = self_1419;
    let _e41: Rotor = other_1233;
    let _e44: Rotor = other_1233;
    let _e47: Rotor = other_1233;
    let _e50: Rotor = other_1233;
    return MotorDual((((vec4<f32>(_e4.g0_.z) * vec4<f32>(_e8.g0_.y, _e11.g0_.y, _e14.g0_.y, _e17.g0_.x)) * vec4<f32>(0.0, 0.0, 1.0, 1.0)) + ((vec4<f32>(_e28.g0_.x, _e31.g0_.x, _e34.g0_.y, _e37.g0_.y) * vec4<f32>(_e41.g0_.y, _e44.g0_.x, _e47.g0_.x, _e50.g0_.y)) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))));
}

fn plane_rotor_regressive_product(self_1420: Plane, other_1234: Rotor) -> Scalar {
    var self_1421: Plane;
    var other_1235: Rotor;

    self_1421 = self_1420;
    other_1235 = other_1234;
    let _e4: Plane = self_1421;
    let _e7: Rotor = other_1235;
    return Scalar((_e4.g0_.x * _e7.g0_.y));
}

fn plane_rotor_outer_product(self_1422: Plane, other_1236: Rotor) -> MotorDual {
    var self_1423: Plane;
    var other_1237: Rotor;

    self_1423 = self_1422;
    other_1237 = other_1236;
    let _e4: Plane = self_1423;
    let _e7: Plane = self_1423;
    let _e10: Plane = self_1423;
    let _e13: Plane = self_1423;
    let _e17: Rotor = other_1237;
    let _e20: Rotor = other_1237;
    let _e23: Rotor = other_1237;
    let _e26: Rotor = other_1237;
    return MotorDual((vec4<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.y, _e13.g0_.z) * vec4<f32>(_e17.g0_.y, _e20.g0_.x, _e23.g0_.x, _e26.g0_.x)));
}

fn plane_rotor_inner_product(self_1424: Plane, other_1238: Rotor) -> Plane {
    var self_1425: Plane;
    var other_1239: Rotor;

    self_1425 = self_1424;
    other_1239 = other_1238;
    let _e4: Plane = self_1425;
    let _e8: Rotor = other_1239;
    let _e11: Rotor = other_1239;
    let _e14: Rotor = other_1239;
    let _e24: Plane = self_1425;
    let _e27: Rotor = other_1239;
    let _e30: Rotor = other_1239;
    let _e33: Rotor = other_1239;
    return Plane((((vec3<f32>(_e4.g0_.z) * vec3<f32>(_e8.g0_.y, _e11.g0_.y, _e14.g0_.x)) * vec3<f32>(0.0, 1.0, 1.0)) + ((_e24.g0_.xyy * vec3<f32>(_e27.g0_.x, _e30.g0_.x, _e33.g0_.y)) * vec3<f32>(1.0, 1.0, -(1.0)))));
}

fn plane_rotor_geometric_anti_product(self_1426: Plane, other_1240: Rotor) -> Motor {
    var self_1427: Plane;
    var other_1241: Rotor;

    self_1427 = self_1426;
    other_1241 = other_1240;
    let _e4: Plane = self_1427;
    let _e8: Rotor = other_1241;
    let _e11: Rotor = other_1241;
    let _e14: Rotor = other_1241;
    let _e17: Rotor = other_1241;
    let _e29: Plane = self_1427;
    let _e32: Plane = self_1427;
    let _e35: Plane = self_1427;
    let _e38: Plane = self_1427;
    let _e42: Rotor = other_1241;
    let _e45: Rotor = other_1241;
    let _e48: Rotor = other_1241;
    let _e51: Rotor = other_1241;
    return Motor((((vec4<f32>(_e4.g0_.z) * vec4<f32>(_e8.g0_.y, _e11.g0_.y, _e14.g0_.y, _e17.g0_.x)) * vec4<f32>(0.0, 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e29.g0_.x, _e32.g0_.x, _e35.g0_.y, _e38.g0_.y) * vec4<f32>(_e42.g0_.y, _e45.g0_.x, _e48.g0_.x, _e51.g0_.y)) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0)))));
}

fn plane_rotor_inner_anti_product(self_1428: Plane, other_1242: Rotor) -> Point {
    var self_1429: Plane;
    var other_1243: Rotor;

    self_1429 = self_1428;
    other_1243 = other_1242;
    let _e4: Plane = self_1429;
    let _e8: Rotor = other_1243;
    let _e11: Rotor = other_1243;
    let _e14: Rotor = other_1243;
    let _e25: Plane = self_1429;
    let _e28: Rotor = other_1243;
    let _e31: Rotor = other_1243;
    let _e34: Rotor = other_1243;
    return Point((((vec3<f32>(_e4.g0_.z) * vec3<f32>(_e8.g0_.y, _e11.g0_.y, _e14.g0_.x)) * vec3<f32>(0.0, 1.0, -(1.0))) + ((_e25.g0_.xyy * vec3<f32>(_e28.g0_.x, _e31.g0_.x, _e34.g0_.y)) * vec3<f32>(-(1.0)))));
}

fn plane_rotor_right_contraction(self_1430: Plane, other_1244: Rotor) -> Plane {
    var self_1431: Plane;
    var other_1245: Rotor;

    self_1431 = self_1430;
    other_1245 = other_1244;
    let _e4: Plane = self_1431;
    let _e6: Rotor = other_1245;
    return Plane((_e4.g0_ * vec3<f32>(_e6.g0_.x)));
}

fn plane_rotor_left_anti_contraction(self_1432: Plane, other_1246: Rotor) -> Point {
    var self_1433: Plane;
    var other_1247: Rotor;

    self_1433 = self_1432;
    other_1247 = other_1246;
    let _e4: Plane = self_1433;
    let _e6: Rotor = other_1247;
    return Point(((_e4.g0_ * vec3<f32>(_e6.g0_.x)) * vec3<f32>(-(1.0))));
}

fn plane_rotor_right_anti_contraction(self_1434: Plane, other_1248: Rotor) -> IdealPoint {
    var self_1435: Plane;
    var other_1249: Rotor;

    self_1435 = self_1434;
    other_1249 = other_1248;
    let _e4: Plane = self_1435;
    let _e7: Plane = self_1435;
    let _e11: Rotor = other_1249;
    return IdealPoint(((vec2<f32>(_e4.g0_.z, _e7.g0_.y) * vec2<f32>(_e11.g0_.y)) * vec2<f32>(1.0, -(1.0))));
}

fn plane_point_geometric_product(self_1436: Plane, other_1250: Point) -> MotorDual {
    var self_1437: Plane;
    var other_1251: Point;

    self_1437 = self_1436;
    other_1251 = other_1250;
    let _e4: Plane = self_1437;
    let _e8: Point = other_1251;
    let _e11: Point = other_1251;
    let _e14: Point = other_1251;
    let _e17: Point = other_1251;
    let _e29: Plane = self_1437;
    let _e33: Point = other_1251;
    let _e36: Point = other_1251;
    let _e39: Point = other_1251;
    let _e42: Point = other_1251;
    let _e55: Plane = self_1437;
    let _e59: Point = other_1251;
    let _e62: Point = other_1251;
    let _e65: Point = other_1251;
    let _e68: Point = other_1251;
    return MotorDual(((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y, _e11.g0_.z, _e14.g0_.y, _e17.g0_.x)) * vec4<f32>(1.0, 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e29.g0_.z) * vec4<f32>(_e33.g0_.z, _e36.g0_.y, _e39.g0_.x, _e42.g0_.z)) * vec4<f32>(1.0, -(1.0), 1.0, 0.0))) + ((vec4<f32>(_e55.g0_.x) * vec4<f32>(_e59.g0_.x, _e62.g0_.x, _e65.g0_.z, _e68.g0_.y)) * vec4<f32>(1.0, 0.0, -(1.0), 1.0))));
}

fn plane_point_regressive_product(self_1438: Plane, other_1252: Point) -> Scalar {
    var self_1439: Plane;
    var other_1253: Point;

    self_1439 = self_1438;
    other_1253 = other_1252;
    let _e4: Plane = self_1439;
    let _e7: Point = other_1253;
    let _e11: Plane = self_1439;
    let _e14: Point = other_1253;
    let _e19: Plane = self_1439;
    let _e22: Point = other_1253;
    return Scalar((((_e4.g0_.x * _e7.g0_.x) + (_e11.g0_.y * _e14.g0_.y)) + (_e19.g0_.z * _e22.g0_.z)));
}

fn plane_point_outer_product(self_1440: Plane, other_1254: Point) -> AntiScalar {
    var self_1441: Plane;
    var other_1255: Point;

    self_1441 = self_1440;
    other_1255 = other_1254;
    let _e4: Plane = self_1441;
    let _e7: Point = other_1255;
    let _e11: Plane = self_1441;
    let _e14: Point = other_1255;
    let _e19: Plane = self_1441;
    let _e22: Point = other_1255;
    return AntiScalar((((_e4.g0_.x * _e7.g0_.x) + (_e11.g0_.y * _e14.g0_.y)) + (_e19.g0_.z * _e22.g0_.z)));
}

fn plane_point_inner_product(self_1442: Plane, other_1256: Point) -> Plane {
    var self_1443: Plane;
    var other_1257: Point;

    self_1443 = self_1442;
    other_1257 = other_1256;
    let _e4: Plane = self_1443;
    let _e8: Point = other_1257;
    let _e18: Plane = self_1443;
    let _e22: Point = other_1257;
    let _e33: Plane = self_1443;
    let _e37: Point = other_1257;
    return Plane(((((vec3<f32>(_e4.g0_.y) * _e8.g0_.zzx) * vec3<f32>(1.0, 0.0, -(1.0))) + ((vec3<f32>(_e18.g0_.z) * _e22.g0_.yxy) * vec3<f32>(-(1.0), 1.0, 0.0))) + ((vec3<f32>(_e33.g0_.x) * _e37.g0_.xzy) * vec3<f32>(0.0, -(1.0), 1.0))));
}

fn plane_point_geometric_anti_product(self_1444: Plane, other_1258: Point) -> Motor {
    var self_1445: Plane;
    var other_1259: Point;

    self_1445 = self_1444;
    other_1259 = other_1258;
    let _e4: Plane = self_1445;
    let _e8: Point = other_1259;
    let _e11: Point = other_1259;
    let _e14: Point = other_1259;
    let _e17: Point = other_1259;
    let _e29: Plane = self_1445;
    let _e33: Point = other_1259;
    let _e36: Point = other_1259;
    let _e39: Point = other_1259;
    let _e42: Point = other_1259;
    let _e55: Plane = self_1445;
    let _e59: Point = other_1259;
    let _e62: Point = other_1259;
    let _e65: Point = other_1259;
    let _e68: Point = other_1259;
    return Motor(((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y, _e11.g0_.z, _e14.g0_.y, _e17.g0_.x)) * vec4<f32>(1.0, 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e29.g0_.z) * vec4<f32>(_e33.g0_.z, _e36.g0_.y, _e39.g0_.x, _e42.g0_.z)) * vec4<f32>(1.0, -(1.0), 1.0, 0.0))) + ((vec4<f32>(_e55.g0_.x) * vec4<f32>(_e59.g0_.x, _e62.g0_.x, _e65.g0_.z, _e68.g0_.y)) * vec4<f32>(1.0, 0.0, -(1.0), 1.0))));
}

fn plane_point_inner_anti_product(self_1446: Plane, other_1260: Point) -> Point {
    var self_1447: Plane;
    var other_1261: Point;

    self_1447 = self_1446;
    other_1261 = other_1260;
    let _e4: Plane = self_1447;
    let _e8: Point = other_1261;
    let _e18: Plane = self_1447;
    let _e22: Point = other_1261;
    let _e33: Plane = self_1447;
    let _e37: Point = other_1261;
    return Point(((((vec3<f32>(_e4.g0_.y) * _e8.g0_.zzx) * vec3<f32>(1.0, 0.0, -(1.0))) + ((vec3<f32>(_e18.g0_.z) * _e22.g0_.yxy) * vec3<f32>(-(1.0), 1.0, 0.0))) + ((vec3<f32>(_e33.g0_.x) * _e37.g0_.xzy) * vec3<f32>(0.0, -(1.0), 1.0))));
}

fn plane_point_left_contraction(self_1448: Plane, other_1262: Point) -> Plane {
    var self_1449: Plane;
    var other_1263: Point;

    self_1449 = self_1448;
    other_1263 = other_1262;
    let _e4: Plane = self_1449;
    let _e8: Point = other_1263;
    let _e18: Plane = self_1449;
    let _e22: Point = other_1263;
    let _e33: Plane = self_1449;
    let _e37: Point = other_1263;
    return Plane(((((vec3<f32>(_e4.g0_.y) * _e8.g0_.zzx) * vec3<f32>(1.0, 0.0, -(1.0))) + ((vec3<f32>(_e18.g0_.z) * _e22.g0_.yxy) * vec3<f32>(-(1.0), 1.0, 0.0))) + ((vec3<f32>(_e33.g0_.x) * _e37.g0_.xzy) * vec3<f32>(0.0, -(1.0), 1.0))));
}

fn plane_point_right_anti_contraction(self_1450: Plane, other_1264: Point) -> Point {
    var self_1451: Plane;
    var other_1265: Point;

    self_1451 = self_1450;
    other_1265 = other_1264;
    let _e4: Plane = self_1451;
    let _e8: Point = other_1265;
    let _e18: Plane = self_1451;
    let _e22: Point = other_1265;
    let _e33: Plane = self_1451;
    let _e37: Point = other_1265;
    return Point(((((vec3<f32>(_e4.g0_.y) * _e8.g0_.zzx) * vec3<f32>(1.0, 0.0, -(1.0))) + ((vec3<f32>(_e18.g0_.z) * _e22.g0_.yxy) * vec3<f32>(-(1.0), 1.0, 0.0))) + ((vec3<f32>(_e33.g0_.x) * _e37.g0_.xzy) * vec3<f32>(0.0, -(1.0), 1.0))));
}

fn plane_ideal_point_geometric_product(self_1452: Plane, other_1266: IdealPoint) -> MotorDual {
    var self_1453: Plane;
    var other_1267: IdealPoint;

    self_1453 = self_1452;
    other_1267 = other_1266;
    let _e4: Plane = self_1453;
    let _e8: IdealPoint = other_1267;
    let _e11: IdealPoint = other_1267;
    let _e14: IdealPoint = other_1267;
    let _e17: IdealPoint = other_1267;
    let _e29: Plane = self_1453;
    let _e32: Plane = self_1453;
    let _e35: Plane = self_1453;
    let _e38: Plane = self_1453;
    let _e42: IdealPoint = other_1267;
    let _e45: IdealPoint = other_1267;
    let _e48: IdealPoint = other_1267;
    let _e51: IdealPoint = other_1267;
    return MotorDual((((vec4<f32>(_e4.g0_.z) * vec4<f32>(_e8.g0_.y, _e11.g0_.x, _e14.g0_.y, _e17.g0_.y)) * vec4<f32>(1.0, -(1.0), 0.0, 0.0)) + ((vec4<f32>(_e29.g0_.y, _e32.g0_.y, _e35.g0_.x, _e38.g0_.x) * vec4<f32>(_e42.g0_.x, _e45.g0_.y, _e48.g0_.y, _e51.g0_.x)) * vec4<f32>(1.0, 1.0, -(1.0), 1.0))));
}

fn plane_ideal_point_regressive_product(self_1454: Plane, other_1268: IdealPoint) -> Scalar {
    var self_1455: Plane;
    var other_1269: IdealPoint;

    self_1455 = self_1454;
    other_1269 = other_1268;
    let _e4: Plane = self_1455;
    let _e7: IdealPoint = other_1269;
    let _e11: Plane = self_1455;
    let _e14: IdealPoint = other_1269;
    return Scalar(((_e4.g0_.y * _e7.g0_.x) + (_e11.g0_.z * _e14.g0_.y)));
}

fn plane_ideal_point_outer_product(self_1456: Plane, other_1270: IdealPoint) -> AntiScalar {
    var self_1457: Plane;
    var other_1271: IdealPoint;

    self_1457 = self_1456;
    other_1271 = other_1270;
    let _e4: Plane = self_1457;
    let _e7: IdealPoint = other_1271;
    let _e11: Plane = self_1457;
    let _e14: IdealPoint = other_1271;
    return AntiScalar(((_e4.g0_.y * _e7.g0_.x) + (_e11.g0_.z * _e14.g0_.y)));
}

fn plane_ideal_point_inner_product(self_1458: Plane, other_1272: IdealPoint) -> Plane {
    var self_1459: Plane;
    var other_1273: IdealPoint;

    self_1459 = self_1458;
    other_1273 = other_1272;
    let _e4: Plane = self_1459;
    let _e8: IdealPoint = other_1273;
    let _e19: Plane = self_1459;
    let _e22: IdealPoint = other_1273;
    let _e25: IdealPoint = other_1273;
    let _e28: IdealPoint = other_1273;
    return Plane((((vec3<f32>(_e4.g0_.z) * vec3<f32>(_e8.g0_.x)) * vec3<f32>(-(1.0), 0.0, 0.0)) + ((_e19.g0_.yxx * vec3<f32>(_e22.g0_.y, _e25.g0_.y, _e28.g0_.x)) * vec3<f32>(1.0, -(1.0), 1.0))));
}

fn plane_ideal_point_geometric_anti_product(self_1460: Plane, other_1274: IdealPoint) -> Motor {
    var self_1461: Plane;
    var other_1275: IdealPoint;

    self_1461 = self_1460;
    other_1275 = other_1274;
    let _e4: Plane = self_1461;
    let _e8: IdealPoint = other_1275;
    let _e11: IdealPoint = other_1275;
    let _e14: IdealPoint = other_1275;
    let _e17: IdealPoint = other_1275;
    let _e29: Plane = self_1461;
    let _e32: Plane = self_1461;
    let _e35: Plane = self_1461;
    let _e38: Plane = self_1461;
    let _e42: IdealPoint = other_1275;
    let _e45: IdealPoint = other_1275;
    let _e48: IdealPoint = other_1275;
    let _e51: IdealPoint = other_1275;
    return Motor((((vec4<f32>(_e4.g0_.z) * vec4<f32>(_e8.g0_.y, _e11.g0_.x, _e14.g0_.y, _e17.g0_.y)) * vec4<f32>(1.0, -(1.0), 0.0, 0.0)) + ((vec4<f32>(_e29.g0_.y, _e32.g0_.y, _e35.g0_.x, _e38.g0_.x) * vec4<f32>(_e42.g0_.x, _e45.g0_.y, _e48.g0_.y, _e51.g0_.x)) * vec4<f32>(1.0, 1.0, -(1.0), 1.0))));
}

fn plane_ideal_point_inner_anti_product(self_1462: Plane, other_1276: IdealPoint) -> Point {
    var self_1463: Plane;
    var other_1277: IdealPoint;

    self_1463 = self_1462;
    other_1277 = other_1276;
    let _e4: Plane = self_1463;
    let _e8: IdealPoint = other_1277;
    let _e19: Plane = self_1463;
    let _e22: IdealPoint = other_1277;
    let _e25: IdealPoint = other_1277;
    let _e28: IdealPoint = other_1277;
    return Point((((vec3<f32>(_e4.g0_.z) * vec3<f32>(_e8.g0_.x)) * vec3<f32>(-(1.0), 0.0, 0.0)) + ((_e19.g0_.yxx * vec3<f32>(_e22.g0_.y, _e25.g0_.y, _e28.g0_.x)) * vec3<f32>(1.0, -(1.0), 1.0))));
}

fn plane_ideal_point_left_contraction(self_1464: Plane, other_1278: IdealPoint) -> Plane {
    var self_1465: Plane;
    var other_1279: IdealPoint;

    self_1465 = self_1464;
    other_1279 = other_1278;
    let _e4: Plane = self_1465;
    let _e8: IdealPoint = other_1279;
    let _e19: Plane = self_1465;
    let _e22: IdealPoint = other_1279;
    let _e25: IdealPoint = other_1279;
    let _e28: IdealPoint = other_1279;
    return Plane((((vec3<f32>(_e4.g0_.z) * vec3<f32>(_e8.g0_.x)) * vec3<f32>(-(1.0), 0.0, 0.0)) + ((_e19.g0_.yxx * vec3<f32>(_e22.g0_.y, _e25.g0_.y, _e28.g0_.x)) * vec3<f32>(1.0, -(1.0), 1.0))));
}

fn plane_ideal_point_right_anti_contraction(self_1466: Plane, other_1280: IdealPoint) -> Point {
    var self_1467: Plane;
    var other_1281: IdealPoint;

    self_1467 = self_1466;
    other_1281 = other_1280;
    let _e4: Plane = self_1467;
    let _e8: IdealPoint = other_1281;
    let _e19: Plane = self_1467;
    let _e22: IdealPoint = other_1281;
    let _e25: IdealPoint = other_1281;
    let _e28: IdealPoint = other_1281;
    return Point((((vec3<f32>(_e4.g0_.z) * vec3<f32>(_e8.g0_.x)) * vec3<f32>(-(1.0), 0.0, 0.0)) + ((_e19.g0_.yxx * vec3<f32>(_e22.g0_.y, _e25.g0_.y, _e28.g0_.x)) * vec3<f32>(1.0, -(1.0), 1.0))));
}

fn plane_plane_add(self_1468: Plane, other_1282: Plane) -> Plane {
    var self_1469: Plane;
    var other_1283: Plane;

    self_1469 = self_1468;
    other_1283 = other_1282;
    let _e4: Plane = self_1469;
    let _e6: Plane = other_1283;
    return Plane((_e4.g0_ + _e6.g0_));
}

fn plane_plane_sub(self_1470: Plane, other_1284: Plane) -> Plane {
    var self_1471: Plane;
    var other_1285: Plane;

    self_1471 = self_1470;
    other_1285 = other_1284;
    let _e4: Plane = self_1471;
    let _e6: Plane = other_1285;
    return Plane((_e4.g0_ - _e6.g0_));
}

fn plane_plane_mul(self_1472: Plane, other_1286: Plane) -> Plane {
    var self_1473: Plane;
    var other_1287: Plane;

    self_1473 = self_1472;
    other_1287 = other_1286;
    let _e4: Plane = self_1473;
    let _e6: Plane = other_1287;
    return Plane((_e4.g0_ * _e6.g0_));
}

fn plane_plane_div(self_1474: Plane, other_1288: Plane) -> Plane {
    var self_1475: Plane;
    var other_1289: Plane;

    self_1475 = self_1474;
    other_1289 = other_1288;
    let _e4: Plane = self_1475;
    let _e7: Plane = self_1475;
    let _e10: Plane = self_1475;
    let _e19: Plane = other_1289;
    let _e22: Plane = other_1289;
    let _e25: Plane = other_1289;
    return Plane((((vec3<f32>(_e4.g0_.x, _e7.g0_.y, _e10.g0_.z) * vec3<f32>(1.0, 1.0, 1.0)) / vec3<f32>(_e19.g0_.x, _e22.g0_.y, _e25.g0_.z)) * vec3<f32>(1.0, 1.0, 1.0)));
}

fn plane_plane_geometric_product(self_1476: Plane, other_1290: Plane) -> Motor {
    var self_1477: Plane;
    var other_1291: Plane;

    self_1477 = self_1476;
    other_1291 = other_1290;
    let _e4: Plane = self_1477;
    let _e8: Plane = other_1291;
    let _e11: Plane = other_1291;
    let _e14: Plane = other_1291;
    let _e17: Plane = other_1291;
    let _e29: Plane = self_1477;
    let _e33: Plane = other_1291;
    let _e36: Plane = other_1291;
    let _e39: Plane = other_1291;
    let _e42: Plane = other_1291;
    let _e55: Plane = self_1477;
    let _e59: Plane = other_1291;
    let _e62: Plane = other_1291;
    let _e65: Plane = other_1291;
    let _e68: Plane = other_1291;
    return Motor(((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y, _e11.g0_.z, _e14.g0_.y, _e17.g0_.x)) * vec4<f32>(1.0, -(1.0), 0.0, 1.0)) + ((vec4<f32>(_e29.g0_.z) * vec4<f32>(_e33.g0_.z, _e36.g0_.y, _e39.g0_.x, _e42.g0_.z)) * vec4<f32>(1.0, 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e55.g0_.x) * vec4<f32>(_e59.g0_.x, _e62.g0_.x, _e65.g0_.z, _e68.g0_.y)) * vec4<f32>(1.0, 0.0, 1.0, -(1.0)))));
}

fn plane_plane_outer_product(self_1478: Plane, other_1292: Plane) -> Point {
    var self_1479: Plane;
    var other_1293: Plane;

    self_1479 = self_1478;
    other_1293 = other_1292;
    let _e4: Plane = self_1479;
    let _e8: Plane = other_1293;
    let _e18: Plane = self_1479;
    let _e22: Plane = other_1293;
    let _e33: Plane = self_1479;
    let _e37: Plane = other_1293;
    return Point(((((vec3<f32>(_e4.g0_.y) * _e8.g0_.zzx) * vec3<f32>(-(1.0), 0.0, 1.0)) + ((vec3<f32>(_e18.g0_.z) * _e22.g0_.yxy) * vec3<f32>(1.0, -(1.0), 0.0))) + ((vec3<f32>(_e33.g0_.x) * _e37.g0_.xzy) * vec3<f32>(0.0, 1.0, -(1.0)))));
}

fn plane_plane_inner_product(self_1480: Plane, other_1294: Plane) -> Scalar {
    var self_1481: Plane;
    var other_1295: Plane;

    self_1481 = self_1480;
    other_1295 = other_1294;
    let _e4: Plane = self_1481;
    let _e7: Plane = other_1295;
    let _e11: Plane = self_1481;
    let _e14: Plane = other_1295;
    let _e19: Plane = self_1481;
    let _e22: Plane = other_1295;
    return Scalar((((_e4.g0_.x * _e7.g0_.x) + (_e11.g0_.y * _e14.g0_.y)) + (_e19.g0_.z * _e22.g0_.z)));
}

fn plane_plane_geometric_anti_product(self_1482: Plane, other_1296: Plane) -> MotorDual {
    var self_1483: Plane;
    var other_1297: Plane;

    self_1483 = self_1482;
    other_1297 = other_1296;
    let _e4: Plane = self_1483;
    let _e8: Plane = other_1297;
    let _e11: Plane = other_1297;
    let _e14: Plane = other_1297;
    let _e17: Plane = other_1297;
    let _e30: Plane = self_1483;
    let _e34: Plane = other_1297;
    let _e37: Plane = other_1297;
    let _e40: Plane = other_1297;
    let _e43: Plane = other_1297;
    let _e57: Plane = self_1483;
    let _e61: Plane = other_1297;
    let _e64: Plane = other_1297;
    let _e67: Plane = other_1297;
    let _e70: Plane = other_1297;
    return MotorDual(((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y, _e11.g0_.z, _e14.g0_.y, _e17.g0_.x)) * vec4<f32>(-(1.0), 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e30.g0_.z) * vec4<f32>(_e34.g0_.z, _e37.g0_.y, _e40.g0_.x, _e43.g0_.z)) * vec4<f32>(-(1.0), -(1.0), 1.0, 0.0))) + ((vec4<f32>(_e57.g0_.x) * vec4<f32>(_e61.g0_.x, _e64.g0_.x, _e67.g0_.z, _e70.g0_.y)) * vec4<f32>(-(1.0), 0.0, -(1.0), 1.0))));
}

fn plane_plane_inner_anti_product(self_1484: Plane, other_1298: Plane) -> AntiScalar {
    var self_1485: Plane;
    var other_1299: Plane;

    self_1485 = self_1484;
    other_1299 = other_1298;
    let _e5: Plane = self_1485;
    let _e8: Plane = other_1299;
    let _e13: Plane = self_1485;
    let _e16: Plane = other_1299;
    let _e21: Plane = self_1485;
    let _e24: Plane = other_1299;
    return AntiScalar((((0.0 - (_e5.g0_.x * _e8.g0_.x)) - (_e13.g0_.y * _e16.g0_.y)) - (_e21.g0_.z * _e24.g0_.z)));
}

fn plane_plane_left_contraction(self_1486: Plane, other_1300: Plane) -> Scalar {
    var self_1487: Plane;
    var other_1301: Plane;

    self_1487 = self_1486;
    other_1301 = other_1300;
    let _e4: Plane = self_1487;
    let _e7: Plane = other_1301;
    let _e11: Plane = self_1487;
    let _e14: Plane = other_1301;
    let _e19: Plane = self_1487;
    let _e22: Plane = other_1301;
    return Scalar((((_e4.g0_.x * _e7.g0_.x) + (_e11.g0_.y * _e14.g0_.y)) + (_e19.g0_.z * _e22.g0_.z)));
}

fn plane_plane_right_contraction(self_1488: Plane, other_1302: Plane) -> Scalar {
    var self_1489: Plane;
    var other_1303: Plane;

    self_1489 = self_1488;
    other_1303 = other_1302;
    let _e4: Plane = self_1489;
    let _e7: Plane = other_1303;
    let _e11: Plane = self_1489;
    let _e14: Plane = other_1303;
    let _e19: Plane = self_1489;
    let _e22: Plane = other_1303;
    return Scalar((((_e4.g0_.x * _e7.g0_.x) + (_e11.g0_.y * _e14.g0_.y)) + (_e19.g0_.z * _e22.g0_.z)));
}

fn plane_plane_left_anti_contraction(self_1490: Plane, other_1304: Plane) -> AntiScalar {
    var self_1491: Plane;
    var other_1305: Plane;

    self_1491 = self_1490;
    other_1305 = other_1304;
    let _e5: Plane = self_1491;
    let _e8: Plane = other_1305;
    let _e13: Plane = self_1491;
    let _e16: Plane = other_1305;
    let _e21: Plane = self_1491;
    let _e24: Plane = other_1305;
    return AntiScalar((((0.0 - (_e5.g0_.x * _e8.g0_.x)) - (_e13.g0_.y * _e16.g0_.y)) - (_e21.g0_.z * _e24.g0_.z)));
}

fn plane_plane_right_anti_contraction(self_1492: Plane, other_1306: Plane) -> AntiScalar {
    var self_1493: Plane;
    var other_1307: Plane;

    self_1493 = self_1492;
    other_1307 = other_1306;
    let _e5: Plane = self_1493;
    let _e8: Plane = other_1307;
    let _e13: Plane = self_1493;
    let _e16: Plane = other_1307;
    let _e21: Plane = self_1493;
    let _e24: Plane = other_1307;
    return AntiScalar((((0.0 - (_e5.g0_.x * _e8.g0_.x)) - (_e13.g0_.y * _e16.g0_.y)) - (_e21.g0_.z * _e24.g0_.z)));
}

fn plane_plane_scalar_product(self_1494: Plane, other_1308: Plane) -> Scalar {
    var self_1495: Plane;
    var other_1309: Plane;

    self_1495 = self_1494;
    other_1309 = other_1308;
    let _e4: Plane = self_1495;
    let _e7: Plane = other_1309;
    let _e11: Plane = self_1495;
    let _e14: Plane = other_1309;
    let _e19: Plane = self_1495;
    let _e22: Plane = other_1309;
    return Scalar((((_e4.g0_.x * _e7.g0_.x) + (_e11.g0_.y * _e14.g0_.y)) + (_e19.g0_.z * _e22.g0_.z)));
}

fn plane_plane_anti_scalar_product(self_1496: Plane, other_1310: Plane) -> AntiScalar {
    var self_1497: Plane;
    var other_1311: Plane;

    self_1497 = self_1496;
    other_1311 = other_1310;
    let _e5: Plane = self_1497;
    let _e8: Plane = other_1311;
    let _e13: Plane = self_1497;
    let _e16: Plane = other_1311;
    let _e21: Plane = self_1497;
    let _e24: Plane = other_1311;
    return AntiScalar((((0.0 - (_e5.g0_.x * _e8.g0_.x)) - (_e13.g0_.y * _e16.g0_.y)) - (_e21.g0_.z * _e24.g0_.z)));
}

fn plane_translator_geometric_product(self_1498: Plane, other_1312: Translator) -> MotorDual {
    var self_1499: Plane;
    var other_1313: Translator;

    self_1499 = self_1498;
    other_1313 = other_1312;
    let _e4: Plane = self_1499;
    let _e8: Translator = other_1313;
    let _e11: Translator = other_1313;
    let _e14: Translator = other_1313;
    let _e17: Translator = other_1313;
    let _e28: Plane = self_1499;
    let _e32: Translator = other_1313;
    let _e35: Translator = other_1313;
    let _e38: Translator = other_1313;
    let _e41: Translator = other_1313;
    let _e54: Plane = self_1499;
    let _e58: Translator = other_1313;
    let _e61: Translator = other_1313;
    let _e64: Translator = other_1313;
    let _e67: Translator = other_1313;
    return MotorDual(((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y, _e11.g0_.z, _e14.g0_.x, _e17.g0_.y)) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) + ((vec4<f32>(_e28.g0_.z) * vec4<f32>(_e32.g0_.z, _e35.g0_.y, _e38.g0_.z, _e41.g0_.x)) * vec4<f32>(1.0, -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e54.g0_.x) * vec4<f32>(_e58.g0_.x, _e61.g0_.x, _e64.g0_.z, _e67.g0_.y)) * vec4<f32>(0.0, 1.0, -(1.0), 1.0))));
}

fn plane_translator_regressive_product(self_1500: Plane, other_1314: Translator) -> Scalar {
    var self_1501: Plane;
    var other_1315: Translator;

    self_1501 = self_1500;
    other_1315 = other_1314;
    let _e4: Plane = self_1501;
    let _e7: Translator = other_1315;
    let _e11: Plane = self_1501;
    let _e14: Translator = other_1315;
    return Scalar(((_e4.g0_.y * _e7.g0_.y) + (_e11.g0_.z * _e14.g0_.z)));
}

fn plane_translator_outer_product(self_1502: Plane, other_1316: Translator) -> MotorDual {
    var self_1503: Plane;
    var other_1317: Translator;

    self_1503 = self_1502;
    other_1317 = other_1316;
    let _e4: Plane = self_1503;
    let _e8: Translator = other_1317;
    let _e11: Translator = other_1317;
    let _e14: Translator = other_1317;
    let _e17: Translator = other_1317;
    let _e28: Plane = self_1503;
    let _e31: Plane = self_1503;
    let _e34: Plane = self_1503;
    let _e37: Plane = self_1503;
    let _e41: Translator = other_1317;
    let _e44: Translator = other_1317;
    let _e47: Translator = other_1317;
    let _e50: Translator = other_1317;
    return MotorDual((((vec4<f32>(_e4.g0_.z) * vec4<f32>(_e8.g0_.z, _e11.g0_.z, _e14.g0_.z, _e17.g0_.x)) * vec4<f32>(1.0, 0.0, 0.0, 1.0)) + ((vec4<f32>(_e28.g0_.y, _e31.g0_.x, _e34.g0_.y, _e37.g0_.x) * vec4<f32>(_e41.g0_.y, _e44.g0_.x, _e47.g0_.x, _e50.g0_.x)) * vec4<f32>(1.0, 1.0, 1.0, 0.0))));
}

fn plane_translator_inner_product(self_1504: Plane, other_1318: Translator) -> Plane {
    var self_1505: Plane;
    var other_1319: Translator;

    self_1505 = self_1504;
    other_1319 = other_1318;
    let _e4: Plane = self_1505;
    let _e8: Translator = other_1319;
    let _e18: Plane = self_1505;
    let _e22: Translator = other_1319;
    let _e33: Plane = self_1505;
    let _e36: Translator = other_1319;
    return Plane(((((vec3<f32>(_e4.g0_.x) * _e8.g0_.xzy) * vec3<f32>(1.0, -(1.0), 1.0)) + ((vec3<f32>(_e18.g0_.z) * _e22.g0_.yyx) * vec3<f32>(-(1.0), 0.0, 1.0))) + ((_e33.g0_.yyx * _e36.g0_.zxx) * vec3<f32>(1.0, 1.0, 0.0))));
}

fn plane_translator_geometric_anti_product(self_1506: Plane, other_1320: Translator) -> Motor {
    var self_1507: Plane;
    var other_1321: Translator;

    self_1507 = self_1506;
    other_1321 = other_1320;
    let _e4: Plane = self_1507;
    let _e8: Translator = other_1321;
    let _e11: Translator = other_1321;
    let _e14: Translator = other_1321;
    let _e17: Translator = other_1321;
    let _e29: Plane = self_1507;
    let _e33: Translator = other_1321;
    let _e36: Translator = other_1321;
    let _e39: Translator = other_1321;
    let _e42: Translator = other_1321;
    let _e56: Plane = self_1507;
    let _e60: Translator = other_1321;
    let _e63: Translator = other_1321;
    let _e66: Translator = other_1321;
    let _e69: Translator = other_1321;
    return Motor(((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y, _e11.g0_.z, _e14.g0_.x, _e17.g0_.y)) * vec4<f32>(1.0, 1.0, -(1.0), 0.0)) + ((vec4<f32>(_e29.g0_.z) * vec4<f32>(_e33.g0_.z, _e36.g0_.y, _e39.g0_.z, _e42.g0_.x)) * vec4<f32>(1.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e56.g0_.x) * vec4<f32>(_e60.g0_.x, _e63.g0_.x, _e66.g0_.z, _e69.g0_.y)) * vec4<f32>(0.0, -(1.0), -(1.0), 1.0))));
}

fn plane_translator_inner_anti_product(self_1508: Plane, other_1322: Translator) -> Point {
    var self_1509: Plane;
    var other_1323: Translator;

    self_1509 = self_1508;
    other_1323 = other_1322;
    let _e4: Plane = self_1509;
    let _e8: Translator = other_1323;
    let _e19: Plane = self_1509;
    let _e23: Translator = other_1323;
    let _e35: Plane = self_1509;
    let _e38: Translator = other_1323;
    return Point(((((vec3<f32>(_e4.g0_.x) * _e8.g0_.xzy) * vec3<f32>(-(1.0), -(1.0), 1.0)) + ((vec3<f32>(_e19.g0_.z) * _e23.g0_.yyx) * vec3<f32>(-(1.0), 0.0, -(1.0)))) + ((_e35.g0_.yyx * _e38.g0_.zxx) * vec3<f32>(1.0, -(1.0), 0.0))));
}

fn plane_translator_left_contraction(self_1510: Plane, other_1324: Translator) -> Plane {
    var self_1511: Plane;
    var other_1325: Translator;

    self_1511 = self_1510;
    other_1325 = other_1324;
    let _e4: Plane = self_1511;
    let _e8: Translator = other_1325;
    let _e19: Plane = self_1511;
    let _e22: Translator = other_1325;
    return Plane((((vec3<f32>(_e4.g0_.z) * vec3<f32>(_e8.g0_.y)) * vec3<f32>(-(1.0), 0.0, 0.0)) + ((_e19.g0_.yxx * _e22.g0_.zzy) * vec3<f32>(1.0, -(1.0), 1.0))));
}

fn plane_translator_right_contraction(self_1512: Plane, other_1326: Translator) -> Plane {
    var self_1513: Plane;
    var other_1327: Translator;

    self_1513 = self_1512;
    other_1327 = other_1326;
    let _e4: Plane = self_1513;
    let _e6: Translator = other_1327;
    return Plane((_e4.g0_ * vec3<f32>(_e6.g0_.x)));
}

fn plane_translator_left_anti_contraction(self_1514: Plane, other_1328: Translator) -> Point {
    var self_1515: Plane;
    var other_1329: Translator;

    self_1515 = self_1514;
    other_1329 = other_1328;
    let _e4: Plane = self_1515;
    let _e6: Translator = other_1329;
    return Point(((_e4.g0_ * vec3<f32>(_e6.g0_.x)) * vec3<f32>(-(1.0))));
}

fn plane_translator_right_anti_contraction(self_1516: Plane, other_1330: Translator) -> Point {
    var self_1517: Plane;
    var other_1331: Translator;

    self_1517 = self_1516;
    other_1331 = other_1330;
    let _e4: Plane = self_1517;
    let _e8: Translator = other_1331;
    let _e19: Plane = self_1517;
    let _e22: Translator = other_1331;
    return Point((((vec3<f32>(_e4.g0_.z) * vec3<f32>(_e8.g0_.y)) * vec3<f32>(-(1.0), 0.0, 0.0)) + ((_e19.g0_.yxx * _e22.g0_.zzy) * vec3<f32>(1.0, -(1.0), 1.0))));
}

fn plane_motor_geometric_product(self_1518: Plane, other_1332: Motor) -> MotorDual {
    var self_1519: Plane;
    var other_1333: Motor;

    self_1519 = self_1518;
    other_1333 = other_1332;
    let _e4: Plane = self_1519;
    let _e8: Motor = other_1333;
    let _e19: Plane = self_1519;
    let _e23: Motor = other_1333;
    let _e35: Plane = self_1519;
    let _e39: Motor = other_1333;
    return MotorDual(((((vec4<f32>(_e4.g0_.x) * _e8.g0_.yxwz) * vec4<f32>(1.0, 1.0, -(1.0), 1.0)) + ((vec4<f32>(_e19.g0_.y) * _e23.g0_.zwxy) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e35.g0_.z) * _e39.g0_.wzyx) * vec4<f32>(1.0, -(1.0), 1.0, 1.0))));
}

fn plane_motor_regressive_product(self_1520: Plane, other_1334: Motor) -> Scalar {
    var self_1521: Plane;
    var other_1335: Motor;

    self_1521 = self_1520;
    other_1335 = other_1334;
    let _e4: Plane = self_1521;
    let _e7: Motor = other_1335;
    let _e11: Plane = self_1521;
    let _e14: Motor = other_1335;
    let _e19: Plane = self_1521;
    let _e22: Motor = other_1335;
    return Scalar((((_e4.g0_.x * _e7.g0_.y) + (_e11.g0_.y * _e14.g0_.z)) + (_e19.g0_.z * _e22.g0_.w)));
}

fn plane_motor_outer_product(self_1522: Plane, other_1336: Motor) -> MotorDual {
    var self_1523: Plane;
    var other_1337: Motor;

    self_1523 = self_1522;
    other_1337 = other_1336;
    let _e4: Plane = self_1523;
    let _e8: Motor = other_1337;
    let _e18: Plane = self_1523;
    let _e22: Motor = other_1337;
    let _e33: Plane = self_1523;
    let _e37: Motor = other_1337;
    return MotorDual(((((vec4<f32>(_e4.g0_.y) * _e8.g0_.zzxz) * vec4<f32>(1.0, 0.0, 1.0, 0.0)) + ((vec4<f32>(_e18.g0_.z) * _e22.g0_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e33.g0_.x) * _e37.g0_.yxxx) * vec4<f32>(1.0, 1.0, 0.0, 0.0))));
}

fn plane_motor_inner_product(self_1524: Plane, other_1338: Motor) -> Plane {
    var self_1525: Plane;
    var other_1339: Motor;

    self_1525 = self_1524;
    other_1339 = other_1338;
    let _e4: Plane = self_1525;
    let _e8: Motor = other_1339;
    let _e11: Motor = other_1339;
    let _e14: Motor = other_1339;
    let _e25: Plane = self_1525;
    let _e29: Motor = other_1339;
    let _e32: Motor = other_1339;
    let _e35: Motor = other_1339;
    let _e47: Plane = self_1525;
    let _e51: Motor = other_1339;
    let _e54: Motor = other_1339;
    let _e57: Motor = other_1339;
    return Plane(((((vec3<f32>(_e4.g0_.x) * vec3<f32>(_e8.g0_.x, _e11.g0_.w, _e14.g0_.z)) * vec3<f32>(1.0, -(1.0), 1.0)) + ((vec3<f32>(_e25.g0_.y) * vec3<f32>(_e29.g0_.w, _e32.g0_.x, _e35.g0_.y)) * vec3<f32>(1.0, 1.0, -(1.0)))) + ((vec3<f32>(_e47.g0_.z) * vec3<f32>(_e51.g0_.z, _e54.g0_.y, _e57.g0_.x)) * vec3<f32>(-(1.0), 1.0, 1.0))));
}

fn plane_motor_geometric_anti_product(self_1526: Plane, other_1340: Motor) -> Motor {
    var self_1527: Plane;
    var other_1341: Motor;

    self_1527 = self_1526;
    other_1341 = other_1340;
    let _e4: Plane = self_1527;
    let _e8: Motor = other_1341;
    let _e20: Plane = self_1527;
    let _e24: Motor = other_1341;
    let _e37: Plane = self_1527;
    let _e41: Motor = other_1341;
    return Motor(((((vec4<f32>(_e4.g0_.x) * _e8.g0_.yxwz) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0)) + ((vec4<f32>(_e20.g0_.y) * _e24.g0_.zwxy) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e37.g0_.z) * _e41.g0_.wzyx) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))));
}

fn plane_motor_inner_anti_product(self_1528: Plane, other_1342: Motor) -> Point {
    var self_1529: Plane;
    var other_1343: Motor;

    self_1529 = self_1528;
    other_1343 = other_1342;
    let _e4: Plane = self_1529;
    let _e8: Motor = other_1343;
    let _e11: Motor = other_1343;
    let _e14: Motor = other_1343;
    let _e26: Plane = self_1529;
    let _e30: Motor = other_1343;
    let _e33: Motor = other_1343;
    let _e36: Motor = other_1343;
    let _e49: Plane = self_1529;
    let _e53: Motor = other_1343;
    let _e56: Motor = other_1343;
    let _e59: Motor = other_1343;
    return Point(((((vec3<f32>(_e4.g0_.x) * vec3<f32>(_e8.g0_.x, _e11.g0_.w, _e14.g0_.z)) * vec3<f32>(-(1.0), -(1.0), 1.0)) + ((vec3<f32>(_e26.g0_.y) * vec3<f32>(_e30.g0_.w, _e33.g0_.x, _e36.g0_.y)) * vec3<f32>(1.0, -(1.0), -(1.0)))) + ((vec3<f32>(_e49.g0_.z) * vec3<f32>(_e53.g0_.z, _e56.g0_.y, _e59.g0_.x)) * vec3<f32>(-(1.0), 1.0, -(1.0)))));
}

fn plane_motor_left_contraction(self_1530: Plane, other_1344: Motor) -> Plane {
    var self_1531: Plane;
    var other_1345: Motor;

    self_1531 = self_1530;
    other_1345 = other_1344;
    let _e4: Plane = self_1531;
    let _e8: Motor = other_1345;
    let _e11: Motor = other_1345;
    let _e14: Motor = other_1345;
    let _e25: Plane = self_1531;
    let _e29: Motor = other_1345;
    let _e32: Motor = other_1345;
    let _e35: Motor = other_1345;
    let _e47: Plane = self_1531;
    let _e51: Motor = other_1345;
    let _e54: Motor = other_1345;
    let _e57: Motor = other_1345;
    return Plane(((((vec3<f32>(_e4.g0_.y) * vec3<f32>(_e8.g0_.w, _e11.g0_.w, _e14.g0_.y)) * vec3<f32>(1.0, 0.0, -(1.0))) + ((vec3<f32>(_e25.g0_.z) * vec3<f32>(_e29.g0_.z, _e32.g0_.y, _e35.g0_.z)) * vec3<f32>(-(1.0), 1.0, 0.0))) + ((vec3<f32>(_e47.g0_.x) * vec3<f32>(_e51.g0_.x, _e54.g0_.w, _e57.g0_.z)) * vec3<f32>(0.0, -(1.0), 1.0))));
}

fn plane_motor_right_contraction(self_1532: Plane, other_1346: Motor) -> Plane {
    var self_1533: Plane;
    var other_1347: Motor;

    self_1533 = self_1532;
    other_1347 = other_1346;
    let _e4: Plane = self_1533;
    let _e6: Motor = other_1347;
    return Plane((_e4.g0_ * vec3<f32>(_e6.g0_.x)));
}

fn plane_motor_left_anti_contraction(self_1534: Plane, other_1348: Motor) -> Point {
    var self_1535: Plane;
    var other_1349: Motor;

    self_1535 = self_1534;
    other_1349 = other_1348;
    let _e4: Plane = self_1535;
    let _e6: Motor = other_1349;
    return Point(((_e4.g0_ * vec3<f32>(_e6.g0_.x)) * vec3<f32>(-(1.0))));
}

fn plane_motor_right_anti_contraction(self_1536: Plane, other_1350: Motor) -> Point {
    var self_1537: Plane;
    var other_1351: Motor;

    self_1537 = self_1536;
    other_1351 = other_1350;
    let _e4: Plane = self_1537;
    let _e8: Motor = other_1351;
    let _e11: Motor = other_1351;
    let _e14: Motor = other_1351;
    let _e25: Plane = self_1537;
    let _e29: Motor = other_1351;
    let _e32: Motor = other_1351;
    let _e35: Motor = other_1351;
    let _e47: Plane = self_1537;
    let _e51: Motor = other_1351;
    let _e54: Motor = other_1351;
    let _e57: Motor = other_1351;
    return Point(((((vec3<f32>(_e4.g0_.y) * vec3<f32>(_e8.g0_.w, _e11.g0_.w, _e14.g0_.y)) * vec3<f32>(1.0, 0.0, -(1.0))) + ((vec3<f32>(_e25.g0_.z) * vec3<f32>(_e29.g0_.z, _e32.g0_.y, _e35.g0_.z)) * vec3<f32>(-(1.0), 1.0, 0.0))) + ((vec3<f32>(_e47.g0_.x) * vec3<f32>(_e51.g0_.x, _e54.g0_.w, _e57.g0_.z)) * vec3<f32>(0.0, -(1.0), 1.0))));
}

fn plane_motor_dual_add(self_1538: Plane, other_1352: MotorDual) -> MotorDual {
    var self_1539: Plane;
    var other_1353: MotorDual;

    self_1539 = self_1538;
    other_1353 = other_1352;
    let _e4: Plane = self_1539;
    let _e7: Plane = self_1539;
    let _e10: Plane = self_1539;
    let _e13: Plane = self_1539;
    let _e23: MotorDual = other_1353;
    return MotorDual(((vec4<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.y, _e13.g0_.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0)) + _e23.g0_));
}

fn plane_motor_dual_sub(self_1540: Plane, other_1354: MotorDual) -> MotorDual {
    var self_1541: Plane;
    var other_1355: MotorDual;

    self_1541 = self_1540;
    other_1355 = other_1354;
    let _e4: Plane = self_1541;
    let _e7: Plane = self_1541;
    let _e10: Plane = self_1541;
    let _e13: Plane = self_1541;
    let _e23: MotorDual = other_1355;
    return MotorDual(((vec4<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.y, _e13.g0_.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0)) - _e23.g0_));
}

fn plane_motor_dual_geometric_product(self_1542: Plane, other_1356: MotorDual) -> Motor {
    var self_1543: Plane;
    var other_1357: MotorDual;

    self_1543 = self_1542;
    other_1357 = other_1356;
    let _e4: Plane = self_1543;
    let _e8: MotorDual = other_1357;
    let _e19: Plane = self_1543;
    let _e23: MotorDual = other_1357;
    let _e35: Plane = self_1543;
    let _e39: MotorDual = other_1357;
    return Motor(((((vec4<f32>(_e4.g0_.x) * _e8.g0_.yxwz) * vec4<f32>(1.0, 1.0, 1.0, -(1.0))) + ((vec4<f32>(_e19.g0_.y) * _e23.g0_.zwxy) * vec4<f32>(1.0, -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e35.g0_.z) * _e39.g0_.wzyx) * vec4<f32>(1.0, 1.0, -(1.0), 1.0))));
}

fn plane_motor_dual_regressive_product(self_1544: Plane, other_1358: MotorDual) -> Plane {
    var self_1545: Plane;
    var other_1359: MotorDual;

    self_1545 = self_1544;
    other_1359 = other_1358;
    let _e4: Plane = self_1545;
    let _e6: MotorDual = other_1359;
    return Plane((_e4.g0_ * vec3<f32>(_e6.g0_.x)));
}

fn plane_motor_dual_outer_product(self_1546: Plane, other_1360: MotorDual) -> Point {
    var self_1547: Plane;
    var other_1361: MotorDual;

    self_1547 = self_1546;
    other_1361 = other_1360;
    let _e4: Plane = self_1547;
    let _e8: MotorDual = other_1361;
    let _e11: MotorDual = other_1361;
    let _e14: MotorDual = other_1361;
    let _e25: Plane = self_1547;
    let _e29: MotorDual = other_1361;
    let _e32: MotorDual = other_1361;
    let _e35: MotorDual = other_1361;
    let _e47: Plane = self_1547;
    let _e51: MotorDual = other_1361;
    let _e54: MotorDual = other_1361;
    let _e57: MotorDual = other_1361;
    return Point(((((vec3<f32>(_e4.g0_.y) * vec3<f32>(_e8.g0_.w, _e11.g0_.w, _e14.g0_.y)) * vec3<f32>(-(1.0), 0.0, 1.0)) + ((vec3<f32>(_e25.g0_.z) * vec3<f32>(_e29.g0_.z, _e32.g0_.y, _e35.g0_.z)) * vec3<f32>(1.0, -(1.0), 0.0))) + ((vec3<f32>(_e47.g0_.x) * vec3<f32>(_e51.g0_.x, _e54.g0_.w, _e57.g0_.z)) * vec3<f32>(0.0, 1.0, -(1.0)))));
}

fn plane_motor_dual_inner_product(self_1548: Plane, other_1362: MotorDual) -> Motor {
    var self_1549: Plane;
    var other_1363: MotorDual;

    self_1549 = self_1548;
    other_1363 = other_1362;
    let _e4: Plane = self_1549;
    let _e8: MotorDual = other_1363;
    let _e18: Plane = self_1549;
    let _e22: MotorDual = other_1363;
    let _e33: Plane = self_1549;
    let _e37: MotorDual = other_1363;
    return Motor(((((vec4<f32>(_e4.g0_.y) * _e8.g0_.zzxz) * vec4<f32>(1.0, 0.0, 1.0, 0.0)) + ((vec4<f32>(_e18.g0_.z) * _e22.g0_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e33.g0_.x) * _e37.g0_.yxxx) * vec4<f32>(1.0, 1.0, 0.0, 0.0))));
}

fn plane_motor_dual_geometric_anti_product(self_1550: Plane, other_1364: MotorDual) -> MotorDual {
    var self_1551: Plane;
    var other_1365: MotorDual;

    self_1551 = self_1550;
    other_1365 = other_1364;
    let _e4: Plane = self_1551;
    let _e8: MotorDual = other_1365;
    let _e20: Plane = self_1551;
    let _e24: MotorDual = other_1365;
    let _e37: Plane = self_1551;
    let _e41: MotorDual = other_1365;
    return MotorDual(((((vec4<f32>(_e4.g0_.x) * _e8.g0_.yxwz) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0)) + ((vec4<f32>(_e20.g0_.y) * _e24.g0_.zwxy) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e37.g0_.z) * _e41.g0_.wzyx) * vec4<f32>(-(1.0), -(1.0), 1.0, 1.0))));
}

fn plane_motor_dual_inner_anti_product(self_1552: Plane, other_1366: MotorDual) -> MotorDual {
    var self_1553: Plane;
    var other_1367: MotorDual;

    self_1553 = self_1552;
    other_1367 = other_1366;
    let _e4: Plane = self_1553;
    let _e8: MotorDual = other_1367;
    let _e19: Plane = self_1553;
    let _e23: MotorDual = other_1367;
    let _e35: Plane = self_1553;
    let _e39: MotorDual = other_1367;
    return MotorDual(((((vec4<f32>(_e4.g0_.y) * _e8.g0_.zzxz) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0)) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.wwwx) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))) + ((vec4<f32>(_e35.g0_.x) * _e39.g0_.yxxx) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))));
}

fn plane_motor_dual_left_contraction(self_1554: Plane, other_1368: MotorDual) -> Motor {
    var self_1555: Plane;
    var other_1369: MotorDual;

    self_1555 = self_1554;
    other_1369 = other_1368;
    let _e4: Plane = self_1555;
    let _e8: MotorDual = other_1369;
    let _e18: Plane = self_1555;
    let _e22: MotorDual = other_1369;
    let _e33: Plane = self_1555;
    let _e37: MotorDual = other_1369;
    return Motor(((((vec4<f32>(_e4.g0_.y) * _e8.g0_.zzxz) * vec4<f32>(1.0, 0.0, 1.0, 0.0)) + ((vec4<f32>(_e18.g0_.z) * _e22.g0_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e33.g0_.x) * _e37.g0_.yxxx) * vec4<f32>(1.0, 1.0, 0.0, 0.0))));
}

fn plane_motor_dual_right_contraction(self_1556: Plane, other_1370: MotorDual) -> Scalar {
    var self_1557: Plane;
    var other_1371: MotorDual;

    self_1557 = self_1556;
    other_1371 = other_1370;
    let _e4: Plane = self_1557;
    let _e7: MotorDual = other_1371;
    let _e11: Plane = self_1557;
    let _e14: MotorDual = other_1371;
    let _e19: Plane = self_1557;
    let _e22: MotorDual = other_1371;
    return Scalar((((_e4.g0_.x * _e7.g0_.y) + (_e11.g0_.y * _e14.g0_.z)) + (_e19.g0_.z * _e22.g0_.w)));
}

fn plane_motor_dual_left_anti_contraction(self_1558: Plane, other_1372: MotorDual) -> AntiScalar {
    var self_1559: Plane;
    var other_1373: MotorDual;

    self_1559 = self_1558;
    other_1373 = other_1372;
    let _e5: Plane = self_1559;
    let _e8: MotorDual = other_1373;
    let _e13: Plane = self_1559;
    let _e16: MotorDual = other_1373;
    let _e21: Plane = self_1559;
    let _e24: MotorDual = other_1373;
    return AntiScalar((((0.0 - (_e5.g0_.x * _e8.g0_.y)) - (_e13.g0_.y * _e16.g0_.z)) - (_e21.g0_.z * _e24.g0_.w)));
}

fn plane_motor_dual_right_anti_contraction(self_1560: Plane, other_1374: MotorDual) -> MotorDual {
    var self_1561: Plane;
    var other_1375: MotorDual;

    self_1561 = self_1560;
    other_1375 = other_1374;
    let _e4: Plane = self_1561;
    let _e8: MotorDual = other_1375;
    let _e19: Plane = self_1561;
    let _e23: MotorDual = other_1375;
    let _e35: Plane = self_1561;
    let _e39: MotorDual = other_1375;
    return MotorDual(((((vec4<f32>(_e4.g0_.y) * _e8.g0_.zzxz) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0)) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.wwwx) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))) + ((vec4<f32>(_e35.g0_.x) * _e39.g0_.yxxx) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))));
}

fn plane_motor_dual_scalar_product(self_1562: Plane, other_1376: MotorDual) -> Scalar {
    var self_1563: Plane;
    var other_1377: MotorDual;

    self_1563 = self_1562;
    other_1377 = other_1376;
    let _e4: Plane = self_1563;
    let _e7: MotorDual = other_1377;
    let _e11: Plane = self_1563;
    let _e14: MotorDual = other_1377;
    let _e19: Plane = self_1563;
    let _e22: MotorDual = other_1377;
    return Scalar((((_e4.g0_.x * _e7.g0_.y) + (_e11.g0_.y * _e14.g0_.z)) + (_e19.g0_.z * _e22.g0_.w)));
}

fn plane_motor_dual_anti_scalar_product(self_1564: Plane, other_1378: MotorDual) -> AntiScalar {
    var self_1565: Plane;
    var other_1379: MotorDual;

    self_1565 = self_1564;
    other_1379 = other_1378;
    let _e5: Plane = self_1565;
    let _e8: MotorDual = other_1379;
    let _e13: Plane = self_1565;
    let _e16: MotorDual = other_1379;
    let _e21: Plane = self_1565;
    let _e24: MotorDual = other_1379;
    return AntiScalar((((0.0 - (_e5.g0_.x * _e8.g0_.y)) - (_e13.g0_.y * _e16.g0_.z)) - (_e21.g0_.z * _e24.g0_.w)));
}

fn plane_squared_magnitude(self_1566: Plane) -> Scalar {
    var self_1567: Plane;

    self_1567 = self_1566;
    let _e2: Plane = self_1567;
    let _e3: Plane = self_1567;
    let _e4: Plane = plane_reversal(_e3);
    let _e5: Scalar = plane_plane_scalar_product(_e2, _e4);
    return _e5;
}

fn plane_magnitude(self_1568: Plane) -> Scalar {
    var self_1569: Plane;

    self_1569 = self_1568;
    let _e2: Plane = self_1569;
    let _e3: Scalar = plane_squared_magnitude(_e2);
    return Scalar(sqrt(_e3.g0_));
}

fn plane_bulk_norm(self_1570: Plane) -> Scalar {
    var self_1571: Plane;

    self_1571 = self_1570;
    let _e2: Plane = self_1571;
    let _e3: Scalar = plane_squared_magnitude(_e2);
    return Scalar(sqrt(_e3.g0_));
}

fn plane_squared_anti_magnitude(self_1572: Plane) -> AntiScalar {
    var self_1573: Plane;

    self_1573 = self_1572;
    let _e2: Plane = self_1573;
    let _e3: Plane = self_1573;
    let _e4: Plane = plane_anti_reversal(_e3);
    let _e5: AntiScalar = plane_plane_anti_scalar_product(_e2, _e4);
    return _e5;
}

fn plane_weight_norm(self_1574: Plane) -> AntiScalar {
    var self_1575: Plane;

    self_1575 = self_1574;
    let _e2: Plane = self_1575;
    let _e3: AntiScalar = plane_squared_anti_magnitude(_e2);
    return AntiScalar(sqrt(_e3.g0_));
}

fn plane_scale(self_1576: Plane, other_1380: f32) -> Plane {
    var self_1577: Plane;
    var other_1381: f32;

    self_1577 = self_1576;
    other_1381 = other_1380;
    let _e4: Plane = self_1577;
    let _e5: f32 = other_1381;
    let _e7: Plane = plane_scalar_geometric_product(_e4, Scalar(_e5));
    return _e7;
}

fn plane_signum(self_1578: Plane) -> Plane {
    var self_1579: Plane;

    self_1579 = self_1578;
    let _e2: Plane = self_1579;
    let _e3: Plane = self_1579;
    let _e4: Scalar = plane_magnitude(_e3);
    let _e9: Plane = plane_scalar_geometric_product(_e2, Scalar((1.0 / _e4.g0_)));
    return _e9;
}

fn plane_inverse(self_1580: Plane) -> Plane {
    var self_1581: Plane;

    self_1581 = self_1580;
    let _e2: Plane = self_1581;
    let _e3: Plane = plane_reversal(_e2);
    let _e4: Plane = self_1581;
    let _e5: Scalar = plane_squared_magnitude(_e4);
    let _e10: Plane = plane_scalar_geometric_product(_e3, Scalar((1.0 / _e5.g0_)));
    return _e10;
}

fn translator_zero() -> Translator {
    return Translator(vec3<f32>(0.0));
}

fn translator_one() -> Translator {
    return Translator(vec3<f32>(1.0, 0.0, 0.0));
}

fn translator_neg(self_1582: Translator) -> Translator {
    var self_1583: Translator;

    self_1583 = self_1582;
    let _e2: Translator = self_1583;
    return Translator((_e2.g0_ * vec3<f32>(-(1.0))));
}

fn translator_automorphism(self_1584: Translator) -> Translator {
    var self_1585: Translator;

    self_1585 = self_1584;
    let _e2: Translator = self_1585;
    return Translator(_e2.g0_);
}

fn translator_reversal(self_1586: Translator) -> Translator {
    var self_1587: Translator;

    self_1587 = self_1586;
    let _e2: Translator = self_1587;
    return Translator((_e2.g0_ * vec3<f32>(1.0, -(1.0), -(1.0))));
}

fn translator_conjugation(self_1588: Translator) -> Translator {
    var self_1589: Translator;

    self_1589 = self_1588;
    let _e2: Translator = self_1589;
    return Translator((_e2.g0_ * vec3<f32>(1.0, -(1.0), -(1.0))));
}

fn translator_anti_reversal(self_1590: Translator) -> Translator {
    var self_1591: Translator;

    self_1591 = self_1590;
    let _e2: Translator = self_1591;
    return Translator((_e2.g0_ * vec3<f32>(1.0, -(1.0), -(1.0))));
}

fn translator_scalar_into(self_1592: Translator) -> Scalar {
    var self_1593: Translator;

    self_1593 = self_1592;
    let _e2: Translator = self_1593;
    return Scalar(_e2.g0_.x);
}

fn translator_scalar_add(self_1594: Translator, other_1382: Scalar) -> Translator {
    var self_1595: Translator;
    var other_1383: Scalar;

    self_1595 = self_1594;
    other_1383 = other_1382;
    let _e4: Translator = self_1595;
    let _e6: Scalar = other_1383;
    return Translator((_e4.g0_ + (vec3<f32>(_e6.g0_) * vec3<f32>(1.0, 0.0, 0.0))));
}

fn translator_scalar_sub(self_1596: Translator, other_1384: Scalar) -> Translator {
    var self_1597: Translator;
    var other_1385: Scalar;

    self_1597 = self_1596;
    other_1385 = other_1384;
    let _e4: Translator = self_1597;
    let _e6: Scalar = other_1385;
    return Translator((_e4.g0_ - (vec3<f32>(_e6.g0_) * vec3<f32>(1.0, 0.0, 0.0))));
}

fn translator_scalar_geometric_product(self_1598: Translator, other_1386: Scalar) -> Translator {
    var self_1599: Translator;
    var other_1387: Scalar;

    self_1599 = self_1598;
    other_1387 = other_1386;
    let _e4: Translator = self_1599;
    let _e6: Scalar = other_1387;
    return Translator((_e4.g0_ * vec3<f32>(_e6.g0_)));
}

fn translator_scalar_outer_product(self_1600: Translator, other_1388: Scalar) -> Translator {
    var self_1601: Translator;
    var other_1389: Scalar;

    self_1601 = self_1600;
    other_1389 = other_1388;
    let _e4: Translator = self_1601;
    let _e6: Scalar = other_1389;
    return Translator((_e4.g0_ * vec3<f32>(_e6.g0_)));
}

fn translator_scalar_inner_product(self_1602: Translator, other_1390: Scalar) -> Translator {
    var self_1603: Translator;
    var other_1391: Scalar;

    self_1603 = self_1602;
    other_1391 = other_1390;
    let _e4: Translator = self_1603;
    let _e6: Scalar = other_1391;
    return Translator((_e4.g0_ * vec3<f32>(_e6.g0_)));
}

fn translator_scalar_left_contraction(self_1604: Translator, other_1392: Scalar) -> Scalar {
    var self_1605: Translator;
    var other_1393: Scalar;

    self_1605 = self_1604;
    other_1393 = other_1392;
    let _e4: Translator = self_1605;
    let _e7: Scalar = other_1393;
    return Scalar((_e4.g0_.x * _e7.g0_));
}

fn translator_scalar_right_contraction(self_1606: Translator, other_1394: Scalar) -> Translator {
    var self_1607: Translator;
    var other_1395: Scalar;

    self_1607 = self_1606;
    other_1395 = other_1394;
    let _e4: Translator = self_1607;
    let _e6: Scalar = other_1395;
    return Translator((_e4.g0_ * vec3<f32>(_e6.g0_)));
}

fn translator_scalar_right_anti_contraction(self_1608: Translator, other_1396: Scalar) -> AntiScalar {
    var self_1609: Translator;
    var other_1397: Scalar;

    self_1609 = self_1608;
    other_1397 = other_1396;
    let _e5: Translator = self_1609;
    let _e8: Scalar = other_1397;
    return AntiScalar((0.0 - (_e5.g0_.x * _e8.g0_)));
}

fn translator_scalar_scalar_product(self_1610: Translator, other_1398: Scalar) -> Scalar {
    var self_1611: Translator;
    var other_1399: Scalar;

    self_1611 = self_1610;
    other_1399 = other_1398;
    let _e4: Translator = self_1611;
    let _e7: Scalar = other_1399;
    return Scalar((_e4.g0_.x * _e7.g0_));
}

fn translator_scalar_anti_scalar_product(self_1612: Translator, other_1400: Scalar) -> AntiScalar {
    var self_1613: Translator;
    var other_1401: Scalar;

    self_1613 = self_1612;
    other_1401 = other_1400;
    let _e5: Translator = self_1613;
    let _e8: Scalar = other_1401;
    return AntiScalar((0.0 - (_e5.g0_.x * _e8.g0_)));
}

fn translator_anti_scalar_regressive_product(self_1614: Translator, other_1402: AntiScalar) -> Translator {
    var self_1615: Translator;
    var other_1403: AntiScalar;

    self_1615 = self_1614;
    other_1403 = other_1402;
    let _e4: Translator = self_1615;
    let _e6: AntiScalar = other_1403;
    return Translator((_e4.g0_ * vec3<f32>(_e6.g0_)));
}

fn translator_anti_scalar_outer_product(self_1616: Translator, other_1404: AntiScalar) -> AntiScalar {
    var self_1617: Translator;
    var other_1405: AntiScalar;

    self_1617 = self_1616;
    other_1405 = other_1404;
    let _e4: Translator = self_1617;
    let _e7: AntiScalar = other_1405;
    return AntiScalar((_e4.g0_.x * _e7.g0_));
}

fn translator_anti_scalar_geometric_anti_product(self_1618: Translator, other_1406: AntiScalar) -> Translator {
    var self_1619: Translator;
    var other_1407: AntiScalar;

    self_1619 = self_1618;
    other_1407 = other_1406;
    let _e4: Translator = self_1619;
    let _e6: AntiScalar = other_1407;
    return Translator((_e4.g0_ * vec3<f32>(_e6.g0_)));
}

fn translator_anti_scalar_inner_anti_product(self_1620: Translator, other_1408: AntiScalar) -> Translator {
    var self_1621: Translator;
    var other_1409: AntiScalar;

    self_1621 = self_1620;
    other_1409 = other_1408;
    let _e4: Translator = self_1621;
    let _e6: AntiScalar = other_1409;
    return Translator((_e4.g0_ * vec3<f32>(_e6.g0_)));
}

fn translator_anti_scalar_right_anti_contraction(self_1622: Translator, other_1410: AntiScalar) -> Translator {
    var self_1623: Translator;
    var other_1411: AntiScalar;

    self_1623 = self_1622;
    other_1411 = other_1410;
    let _e4: Translator = self_1623;
    let _e6: AntiScalar = other_1411;
    return Translator((_e4.g0_ * vec3<f32>(_e6.g0_)));
}

fn translator_multi_vector_add(self_1624: Translator, other_1412: MultiVector) -> MultiVector {
    var self_1625: Translator;
    var other_1413: MultiVector;

    self_1625 = self_1624;
    other_1413 = other_1412;
    let _e4: Translator = self_1625;
    let _e14: MultiVector = other_1413;
    let _e17: Translator = self_1625;
    let _e20: Translator = self_1625;
    let _e23: Translator = self_1625;
    let _e26: Translator = self_1625;
    let _e36: MultiVector = other_1413;
    return MultiVector(((vec4<f32>(_e4.g0_.x) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) + _e14.g0_), ((vec4<f32>(_e17.g0_.x, _e20.g0_.x, _e23.g0_.y, _e26.g0_.z) * vec4<f32>(0.0, 0.0, 1.0, 1.0)) + _e36.g1_));
}

fn translator_multi_vector_sub(self_1626: Translator, other_1414: MultiVector) -> MultiVector {
    var self_1627: Translator;
    var other_1415: MultiVector;

    self_1627 = self_1626;
    other_1415 = other_1414;
    let _e4: Translator = self_1627;
    let _e14: MultiVector = other_1415;
    let _e17: Translator = self_1627;
    let _e20: Translator = self_1627;
    let _e23: Translator = self_1627;
    let _e26: Translator = self_1627;
    let _e36: MultiVector = other_1415;
    return MultiVector(((vec4<f32>(_e4.g0_.x) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) - _e14.g0_), ((vec4<f32>(_e17.g0_.x, _e20.g0_.x, _e23.g0_.y, _e26.g0_.z) * vec4<f32>(0.0, 0.0, 1.0, 1.0)) - _e36.g1_));
}

fn translator_multi_vector_geometric_product(self_1628: Translator, other_1416: MultiVector) -> MultiVector {
    var self_1629: Translator;
    var other_1417: MultiVector;

    self_1629 = self_1628;
    other_1417 = other_1416;
    let _e4: Translator = self_1629;
    let _e8: MultiVector = other_1417;
    let _e11: Translator = self_1629;
    let _e15: MultiVector = other_1417;
    let _e29: Translator = self_1629;
    let _e33: MultiVector = other_1417;
    let _e47: Translator = self_1629;
    let _e51: MultiVector = other_1417;
    let _e54: Translator = self_1629;
    let _e58: MultiVector = other_1417;
    let _e70: Translator = self_1629;
    let _e74: MultiVector = other_1417;
    return MultiVector((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.y) * _e15.g1_.zwxy) * vec4<f32>(-(1.0), 1.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e29.g0_.z) * _e33.g1_.wzyx) * vec4<f32>(-(1.0), -(1.0), -(1.0), 1.0))), (((vec4<f32>(_e47.g0_.x) * _e51.g1_) + ((vec4<f32>(_e54.g0_.y) * _e58.g0_.zwxy) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e70.g0_.z) * _e74.g0_.wzyx) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0))));
}

fn translator_multi_vector_outer_product(self_1630: Translator, other_1418: MultiVector) -> MultiVector {
    var self_1631: Translator;
    var other_1419: MultiVector;

    self_1631 = self_1630;
    other_1419 = other_1418;
    let _e4: Translator = self_1631;
    let _e8: MultiVector = other_1419;
    let _e11: Translator = self_1631;
    let _e15: MultiVector = other_1419;
    let _e18: Translator = self_1631;
    let _e22: MultiVector = other_1419;
    let _e33: Translator = self_1631;
    let _e36: Translator = self_1631;
    let _e39: Translator = self_1631;
    let _e42: Translator = self_1631;
    let _e46: MultiVector = other_1419;
    return MultiVector((vec4<f32>(_e4.g0_.x) * _e8.g0_), (((vec4<f32>(_e11.g0_.x) * _e15.g1_) + ((vec4<f32>(_e18.g0_.z) * _e22.g0_.zzzx) * vec4<f32>(0.0, 1.0, 0.0, 1.0))) + ((vec4<f32>(_e33.g0_.x, _e36.g0_.y, _e39.g0_.y, _e42.g0_.x) * _e46.g0_.xwxx) * vec4<f32>(0.0, 1.0, 1.0, 0.0))));
}

fn translator_multi_vector_inner_product(self_1632: Translator, other_1420: MultiVector) -> MultiVector {
    var self_1633: Translator;
    var other_1421: MultiVector;

    self_1633 = self_1632;
    other_1421 = other_1420;
    let _e4: Translator = self_1633;
    let _e8: MultiVector = other_1421;
    let _e11: Translator = self_1633;
    let _e15: MultiVector = other_1421;
    let _e28: Translator = self_1633;
    let _e31: Translator = self_1633;
    let _e34: Translator = self_1633;
    let _e37: Translator = self_1633;
    let _e41: MultiVector = other_1421;
    let _e55: Translator = self_1633;
    let _e59: MultiVector = other_1421;
    let _e62: Translator = self_1633;
    let _e66: MultiVector = other_1421;
    let _e78: Translator = self_1633;
    let _e81: Translator = self_1633;
    let _e84: Translator = self_1633;
    let _e87: Translator = self_1633;
    let _e91: MultiVector = other_1421;
    return MultiVector((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.z) * _e15.g1_.wwyx) * vec4<f32>(-(1.0), 0.0, -(1.0), 1.0))) + ((vec4<f32>(_e28.g0_.y, _e31.g0_.x, _e34.g0_.y, _e37.g0_.y) * _e41.g1_.zxxy) * vec4<f32>(-(1.0), 0.0, -(1.0), -(1.0)))), (((vec4<f32>(_e55.g0_.x) * _e59.g1_) + ((vec4<f32>(_e62.g0_.z) * _e66.g0_.wwwx) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))) + ((vec4<f32>(_e78.g0_.y, _e81.g0_.x, _e84.g0_.y, _e87.g0_.x) * _e91.g0_.zxxx) * vec4<f32>(1.0, 0.0, 1.0, 0.0))));
}

fn translator_multi_vector_geometric_anti_product(self_1634: Translator, other_1422: MultiVector) -> MultiVector {
    var self_1635: Translator;
    var other_1423: MultiVector;

    self_1635 = self_1634;
    other_1423 = other_1422;
    let _e4: Translator = self_1635;
    let _e8: MultiVector = other_1423;
    let _e19: Translator = self_1635;
    let _e23: MultiVector = other_1423;
    let _e28: Translator = self_1635;
    let _e32: MultiVector = other_1423;
    let _e45: Translator = self_1635;
    let _e49: MultiVector = other_1423;
    let _e62: Translator = self_1635;
    let _e66: MultiVector = other_1423;
    let _e79: Translator = self_1635;
    let _e83: MultiVector = other_1423;
    return MultiVector(((((vec4<f32>(_e4.g0_.x) * _e8.g1_.yxwz) * vec4<f32>(1.0, -(1.0), 1.0, 1.0)) + (vec4<f32>(_e19.g0_.y) * _e23.g0_.wzyx)) + ((vec4<f32>(_e28.g0_.z) * _e32.g0_.zwxy) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))), ((((vec4<f32>(_e45.g0_.x) * _e49.g0_.yxwz) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))) + ((vec4<f32>(_e62.g0_.y) * _e66.g1_.wzyx) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + (vec4<f32>(_e79.g0_.z) * _e83.g1_.zwxy)));
}

fn translator_multi_vector_inner_anti_product(self_1636: Translator, other_1424: MultiVector) -> MultiVector {
    var self_1637: Translator;
    var other_1425: MultiVector;

    self_1637 = self_1636;
    other_1425 = other_1424;
    let _e4: Translator = self_1637;
    let _e8: MultiVector = other_1425;
    let _e19: Translator = self_1637;
    let _e23: MultiVector = other_1425;
    let _e35: Translator = self_1637;
    let _e38: Translator = self_1637;
    let _e41: Translator = self_1637;
    let _e44: Translator = self_1637;
    let _e48: MultiVector = other_1425;
    let _e59: Translator = self_1637;
    let _e63: MultiVector = other_1425;
    let _e76: Translator = self_1637;
    let _e80: MultiVector = other_1425;
    let _e91: Translator = self_1637;
    let _e94: Translator = self_1637;
    let _e97: Translator = self_1637;
    let _e100: Translator = self_1637;
    let _e104: MultiVector = other_1425;
    return MultiVector(((((vec4<f32>(_e4.g0_.x) * _e8.g1_.yxwz) * vec4<f32>(1.0, -(1.0), 1.0, 1.0)) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.wwxw) * vec4<f32>(0.0, -(1.0), 1.0, 0.0))) + ((vec4<f32>(_e35.g0_.x, _e38.g0_.y, _e41.g0_.x, _e44.g0_.y) * _e48.g0_.xzxx) * vec4<f32>(0.0, 1.0, 0.0, 1.0))), ((((vec4<f32>(_e59.g0_.x) * _e63.g0_.yxwz) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))) + ((vec4<f32>(_e76.g0_.z) * _e80.g1_.wwxy) * vec4<f32>(0.0, 1.0, 1.0, 1.0))) + ((vec4<f32>(_e91.g0_.x, _e94.g0_.y, _e97.g0_.y, _e100.g0_.y) * _e104.g1_.xzyx) * vec4<f32>(0.0, 1.0, 1.0, -(1.0)))));
}

fn translator_multi_vector_left_contraction(self_1638: Translator, other_1426: MultiVector) -> MultiVector {
    var self_1639: Translator;
    var other_1427: MultiVector;

    self_1639 = self_1638;
    other_1427 = other_1426;
    let _e4: Translator = self_1639;
    let _e8: MultiVector = other_1427;
    let _e11: Translator = self_1639;
    let _e15: MultiVector = other_1427;
    let _e28: Translator = self_1639;
    let _e31: Translator = self_1639;
    let _e34: Translator = self_1639;
    let _e37: Translator = self_1639;
    let _e41: MultiVector = other_1427;
    let _e54: Translator = self_1639;
    let _e58: MultiVector = other_1427;
    return MultiVector((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.z) * _e15.g1_.wwyw) * vec4<f32>(-(1.0), 0.0, -(1.0), 0.0))) + ((vec4<f32>(_e28.g0_.y, _e31.g0_.x, _e34.g0_.x, _e37.g0_.y) * _e41.g1_.zxxy) * vec4<f32>(-(1.0), 0.0, 0.0, -(1.0)))), (vec4<f32>(_e54.g0_.x) * _e58.g1_));
}

fn translator_multi_vector_right_anti_contraction(self_1640: Translator, other_1428: MultiVector) -> MultiVector {
    var self_1641: Translator;
    var other_1429: MultiVector;

    self_1641 = self_1640;
    other_1429 = other_1428;
    let _e4: Translator = self_1641;
    let _e8: MultiVector = other_1429;
    let _e19: Translator = self_1641;
    let _e23: MultiVector = other_1429;
    let _e36: Translator = self_1641;
    let _e40: MultiVector = other_1429;
    let _e51: Translator = self_1641;
    let _e54: Translator = self_1641;
    let _e57: Translator = self_1641;
    let _e60: Translator = self_1641;
    let _e64: MultiVector = other_1429;
    return MultiVector(((vec4<f32>(_e4.g0_.x) * _e8.g1_.yxwz) * vec4<f32>(1.0, -(1.0), 1.0, 1.0)), ((((vec4<f32>(_e19.g0_.x) * _e23.g0_.yxwz) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))) + ((vec4<f32>(_e36.g0_.z) * _e40.g1_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, 1.0))) + ((vec4<f32>(_e51.g0_.x, _e54.g0_.y, _e57.g0_.y, _e60.g0_.x) * _e64.g1_.xzyx) * vec4<f32>(0.0, 1.0, 1.0, 0.0))));
}

fn translator_multi_vector_scalar_product(self_1642: Translator, other_1430: MultiVector) -> Scalar {
    var self_1643: Translator;
    var other_1431: MultiVector;

    self_1643 = self_1642;
    other_1431 = other_1430;
    let _e4: Translator = self_1643;
    let _e7: MultiVector = other_1431;
    let _e11: Translator = self_1643;
    let _e14: MultiVector = other_1431;
    let _e19: Translator = self_1643;
    let _e22: MultiVector = other_1431;
    return Scalar((((_e4.g0_.x * _e7.g0_.x) - (_e11.g0_.y * _e14.g1_.z)) - (_e19.g0_.z * _e22.g1_.w)));
}

fn translator_multi_vector_anti_scalar_product(self_1644: Translator, other_1432: MultiVector) -> AntiScalar {
    var self_1645: Translator;
    var other_1433: MultiVector;

    self_1645 = self_1644;
    other_1433 = other_1432;
    let _e5: Translator = self_1645;
    let _e8: MultiVector = other_1433;
    let _e13: Translator = self_1645;
    let _e16: MultiVector = other_1433;
    let _e21: Translator = self_1645;
    let _e24: MultiVector = other_1433;
    return AntiScalar((((0.0 - (_e5.g0_.x * _e8.g0_.x)) + (_e13.g0_.y * _e16.g1_.z)) + (_e21.g0_.z * _e24.g1_.w)));
}

fn translator_rotor_add(self_1646: Translator, other_1434: Rotor) -> Motor {
    var self_1647: Translator;
    var other_1435: Rotor;

    self_1647 = self_1646;
    other_1435 = other_1434;
    let _e4: Translator = self_1647;
    let _e7: Translator = self_1647;
    let _e10: Translator = self_1647;
    let _e13: Translator = self_1647;
    let _e23: Rotor = other_1435;
    let _e26: Rotor = other_1435;
    let _e29: Rotor = other_1435;
    let _e32: Rotor = other_1435;
    return Motor(((vec4<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.y, _e13.g0_.z) * vec4<f32>(1.0, 0.0, 1.0, 1.0)) + (vec4<f32>(_e23.g0_.x, _e26.g0_.y, _e29.g0_.x, _e32.g0_.x) * vec4<f32>(1.0, 1.0, 0.0, 0.0))));
}

fn translator_rotor_sub(self_1648: Translator, other_1436: Rotor) -> Motor {
    var self_1649: Translator;
    var other_1437: Rotor;

    self_1649 = self_1648;
    other_1437 = other_1436;
    let _e4: Translator = self_1649;
    let _e7: Translator = self_1649;
    let _e10: Translator = self_1649;
    let _e13: Translator = self_1649;
    let _e23: Rotor = other_1437;
    let _e26: Rotor = other_1437;
    let _e29: Rotor = other_1437;
    let _e32: Rotor = other_1437;
    return Motor(((vec4<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.y, _e13.g0_.z) * vec4<f32>(1.0, 0.0, 1.0, 1.0)) - (vec4<f32>(_e23.g0_.x, _e26.g0_.y, _e29.g0_.x, _e32.g0_.x) * vec4<f32>(1.0, 1.0, 0.0, 0.0))));
}

fn translator_rotor_geometric_product(self_1650: Translator, other_1438: Rotor) -> Motor {
    var self_1651: Translator;
    var other_1439: Rotor;

    self_1651 = self_1650;
    other_1439 = other_1438;
    let _e4: Translator = self_1651;
    let _e8: Rotor = other_1439;
    let _e11: Rotor = other_1439;
    let _e14: Rotor = other_1439;
    let _e17: Rotor = other_1439;
    let _e28: Translator = self_1651;
    let _e31: Translator = self_1651;
    let _e34: Translator = self_1651;
    let _e37: Translator = self_1651;
    let _e41: Rotor = other_1439;
    let _e44: Rotor = other_1439;
    let _e47: Rotor = other_1439;
    let _e50: Rotor = other_1439;
    return Motor((((vec4<f32>(_e4.g0_.z) * vec4<f32>(_e8.g0_.y, _e11.g0_.y, _e14.g0_.y, _e17.g0_.x)) * vec4<f32>(0.0, 0.0, 1.0, 1.0)) + ((vec4<f32>(_e28.g0_.x, _e31.g0_.x, _e34.g0_.y, _e37.g0_.y) * vec4<f32>(_e41.g0_.x, _e44.g0_.y, _e47.g0_.x, _e50.g0_.y)) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))));
}

fn translator_rotor_outer_product(self_1652: Translator, other_1440: Rotor) -> Motor {
    var self_1653: Translator;
    var other_1441: Rotor;

    self_1653 = self_1652;
    other_1441 = other_1440;
    let _e4: Translator = self_1653;
    let _e7: Translator = self_1653;
    let _e10: Translator = self_1653;
    let _e13: Translator = self_1653;
    let _e17: Rotor = other_1441;
    let _e20: Rotor = other_1441;
    let _e23: Rotor = other_1441;
    let _e26: Rotor = other_1441;
    return Motor((vec4<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.y, _e13.g0_.z) * vec4<f32>(_e17.g0_.x, _e20.g0_.y, _e23.g0_.x, _e26.g0_.x)));
}

fn translator_rotor_inner_product(self_1654: Translator, other_1442: Rotor) -> Motor {
    var self_1655: Translator;
    var other_1443: Rotor;

    self_1655 = self_1654;
    other_1443 = other_1442;
    let _e4: Translator = self_1655;
    let _e7: Translator = self_1655;
    let _e10: Translator = self_1655;
    let _e13: Translator = self_1655;
    let _e17: Rotor = other_1443;
    let _e20: Rotor = other_1443;
    let _e23: Rotor = other_1443;
    let _e26: Rotor = other_1443;
    return Motor((vec4<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.y, _e13.g0_.z) * vec4<f32>(_e17.g0_.x, _e20.g0_.y, _e23.g0_.x, _e26.g0_.x)));
}

fn translator_rotor_geometric_anti_product(self_1656: Translator, other_1444: Rotor) -> MotorDual {
    var self_1657: Translator;
    var other_1445: Rotor;

    self_1657 = self_1656;
    other_1445 = other_1444;
    let _e4: Translator = self_1657;
    let _e8: Rotor = other_1445;
    let _e11: Rotor = other_1445;
    let _e14: Rotor = other_1445;
    let _e17: Rotor = other_1445;
    let _e29: Translator = self_1657;
    let _e32: Translator = self_1657;
    let _e35: Translator = self_1657;
    let _e38: Translator = self_1657;
    let _e42: Rotor = other_1445;
    let _e45: Rotor = other_1445;
    let _e48: Rotor = other_1445;
    let _e51: Rotor = other_1445;
    return MotorDual((((vec4<f32>(_e4.g0_.z) * vec4<f32>(_e8.g0_.y, _e11.g0_.y, _e14.g0_.y, _e17.g0_.x)) * vec4<f32>(0.0, 0.0, -(1.0), 1.0)) + ((vec4<f32>(_e29.g0_.x, _e32.g0_.x, _e35.g0_.y, _e38.g0_.y) * vec4<f32>(_e42.g0_.x, _e45.g0_.y, _e48.g0_.x, _e51.g0_.y)) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0))));
}

fn translator_rotor_inner_anti_product(self_1658: Translator, other_1446: Rotor) -> MotorDual {
    var self_1659: Translator;
    var other_1447: Rotor;

    self_1659 = self_1658;
    other_1447 = other_1446;
    let _e4: Translator = self_1659;
    let _e7: Translator = self_1659;
    let _e10: Translator = self_1659;
    let _e13: Translator = self_1659;
    let _e17: Rotor = other_1447;
    let _e20: Rotor = other_1447;
    let _e23: Rotor = other_1447;
    let _e26: Rotor = other_1447;
    return MotorDual(((vec4<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.y, _e13.g0_.z) * vec4<f32>(_e17.g0_.x, _e20.g0_.y, _e23.g0_.x, _e26.g0_.x)) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0)));
}

fn translator_rotor_left_contraction(self_1660: Translator, other_1448: Rotor) -> Rotor {
    var self_1661: Translator;
    var other_1449: Rotor;

    self_1661 = self_1660;
    other_1449 = other_1448;
    let _e4: Translator = self_1661;
    let _e8: Rotor = other_1449;
    return Rotor((vec2<f32>(_e4.g0_.x) * _e8.g0_));
}

fn translator_rotor_right_contraction(self_1662: Translator, other_1450: Rotor) -> Translator {
    var self_1663: Translator;
    var other_1451: Rotor;

    self_1663 = self_1662;
    other_1451 = other_1450;
    let _e4: Translator = self_1663;
    let _e6: Rotor = other_1451;
    return Translator((_e4.g0_ * vec3<f32>(_e6.g0_.x)));
}

fn translator_rotor_scalar_product(self_1664: Translator, other_1452: Rotor) -> Scalar {
    var self_1665: Translator;
    var other_1453: Rotor;

    self_1665 = self_1664;
    other_1453 = other_1452;
    let _e4: Translator = self_1665;
    let _e7: Rotor = other_1453;
    return Scalar((_e4.g0_.x * _e7.g0_.x));
}

fn translator_rotor_anti_scalar_product(self_1666: Translator, other_1454: Rotor) -> AntiScalar {
    var self_1667: Translator;
    var other_1455: Rotor;

    self_1667 = self_1666;
    other_1455 = other_1454;
    let _e5: Translator = self_1667;
    let _e8: Rotor = other_1455;
    return AntiScalar((0.0 - (_e5.g0_.x * _e8.g0_.x)));
}

fn translator_point_add(self_1668: Translator, other_1456: Point) -> Motor {
    var self_1669: Translator;
    var other_1457: Point;

    self_1669 = self_1668;
    other_1457 = other_1456;
    let _e4: Translator = self_1669;
    let _e7: Translator = self_1669;
    let _e10: Translator = self_1669;
    let _e13: Translator = self_1669;
    let _e23: Point = other_1457;
    let _e26: Point = other_1457;
    let _e29: Point = other_1457;
    let _e32: Point = other_1457;
    return Motor(((vec4<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.y, _e13.g0_.z) * vec4<f32>(1.0, 0.0, 1.0, 1.0)) + (vec4<f32>(_e23.g0_.x, _e26.g0_.x, _e29.g0_.y, _e32.g0_.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn translator_point_sub(self_1670: Translator, other_1458: Point) -> Motor {
    var self_1671: Translator;
    var other_1459: Point;

    self_1671 = self_1670;
    other_1459 = other_1458;
    let _e4: Translator = self_1671;
    let _e7: Translator = self_1671;
    let _e10: Translator = self_1671;
    let _e13: Translator = self_1671;
    let _e23: Point = other_1459;
    let _e26: Point = other_1459;
    let _e29: Point = other_1459;
    let _e32: Point = other_1459;
    return Motor(((vec4<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.y, _e13.g0_.z) * vec4<f32>(1.0, 0.0, 1.0, 1.0)) - (vec4<f32>(_e23.g0_.x, _e26.g0_.x, _e29.g0_.y, _e32.g0_.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn translator_point_geometric_product(self_1672: Translator, other_1460: Point) -> Motor {
    var self_1673: Translator;
    var other_1461: Point;

    self_1673 = self_1672;
    other_1461 = other_1460;
    let _e4: Translator = self_1673;
    let _e8: Point = other_1461;
    let _e11: Point = other_1461;
    let _e14: Point = other_1461;
    let _e17: Point = other_1461;
    let _e30: Translator = self_1673;
    let _e34: Point = other_1461;
    let _e37: Point = other_1461;
    let _e40: Point = other_1461;
    let _e43: Point = other_1461;
    let _e57: Translator = self_1673;
    let _e61: Point = other_1461;
    let _e64: Point = other_1461;
    let _e67: Point = other_1461;
    let _e70: Point = other_1461;
    return Motor(((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y, _e11.g0_.z, _e14.g0_.y, _e17.g0_.x)) * vec4<f32>(-(1.0), 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e30.g0_.z) * vec4<f32>(_e34.g0_.z, _e37.g0_.y, _e40.g0_.x, _e43.g0_.z)) * vec4<f32>(-(1.0), -(1.0), 1.0, 0.0))) + ((vec4<f32>(_e57.g0_.x) * vec4<f32>(_e61.g0_.x, _e64.g0_.x, _e67.g0_.y, _e70.g0_.z)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn translator_point_regressive_product(self_1674: Translator, other_1462: Point) -> Plane {
    var self_1675: Translator;
    var other_1463: Point;

    self_1675 = self_1674;
    other_1463 = other_1462;
    let _e4: Translator = self_1675;
    let _e8: Point = other_1463;
    let _e18: Translator = self_1675;
    let _e21: Point = other_1463;
    return Plane((((vec3<f32>(_e4.g0_.z) * _e8.g0_.yxy) * vec3<f32>(1.0, -(1.0), 0.0)) + ((_e18.g0_.yxy * _e21.g0_.zxx) * vec3<f32>(-(1.0), 0.0, 1.0))));
}

fn translator_point_outer_product(self_1676: Translator, other_1464: Point) -> Point {
    var self_1677: Translator;
    var other_1465: Point;

    self_1677 = self_1676;
    other_1465 = other_1464;
    let _e4: Translator = self_1677;
    let _e8: Point = other_1465;
    return Point((vec3<f32>(_e4.g0_.x) * _e8.g0_));
}

fn translator_point_inner_product(self_1678: Translator, other_1466: Point) -> Motor {
    var self_1679: Translator;
    var other_1467: Point;

    self_1679 = self_1678;
    other_1467 = other_1466;
    let _e4: Translator = self_1679;
    let _e8: Point = other_1467;
    let _e20: Translator = self_1679;
    let _e23: Translator = self_1679;
    let _e26: Translator = self_1679;
    let _e29: Translator = self_1679;
    let _e33: Point = other_1467;
    let _e36: Point = other_1467;
    let _e39: Point = other_1467;
    let _e42: Point = other_1467;
    return Motor((((vec4<f32>(_e4.g0_.z) * vec4<f32>(_e8.g0_.z)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0)) + ((vec4<f32>(_e20.g0_.y, _e23.g0_.x, _e26.g0_.x, _e29.g0_.x) * vec4<f32>(_e33.g0_.y, _e36.g0_.x, _e39.g0_.y, _e42.g0_.z)) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0))));
}

fn translator_point_geometric_anti_product(self_1680: Translator, other_1468: Point) -> MotorDual {
    var self_1681: Translator;
    var other_1469: Point;

    self_1681 = self_1680;
    other_1469 = other_1468;
    let _e4: Translator = self_1681;
    let _e8: Point = other_1469;
    let _e11: Point = other_1469;
    let _e14: Point = other_1469;
    let _e17: Point = other_1469;
    let _e29: Translator = self_1681;
    let _e33: Point = other_1469;
    let _e36: Point = other_1469;
    let _e39: Point = other_1469;
    let _e42: Point = other_1469;
    let _e55: Translator = self_1681;
    let _e59: Point = other_1469;
    let _e62: Point = other_1469;
    let _e65: Point = other_1469;
    let _e68: Point = other_1469;
    return MotorDual(((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y, _e11.g0_.z, _e14.g0_.y, _e17.g0_.x)) * vec4<f32>(1.0, -(1.0), 0.0, 1.0)) + ((vec4<f32>(_e29.g0_.z) * vec4<f32>(_e33.g0_.z, _e36.g0_.y, _e39.g0_.x, _e42.g0_.z)) * vec4<f32>(1.0, 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e55.g0_.x) * vec4<f32>(_e59.g0_.x, _e62.g0_.x, _e65.g0_.y, _e68.g0_.z)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn translator_point_inner_anti_product(self_1682: Translator, other_1470: Point) -> MotorDual {
    var self_1683: Translator;
    var other_1471: Point;

    self_1683 = self_1682;
    other_1471 = other_1470;
    let _e4: Translator = self_1683;
    let _e8: Point = other_1471;
    let _e19: Translator = self_1683;
    let _e22: Translator = self_1683;
    let _e25: Translator = self_1683;
    let _e28: Translator = self_1683;
    let _e32: Point = other_1471;
    let _e35: Point = other_1471;
    let _e38: Point = other_1471;
    let _e41: Point = other_1471;
    return MotorDual((((vec4<f32>(_e4.g0_.z) * vec4<f32>(_e8.g0_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) + (vec4<f32>(_e19.g0_.y, _e22.g0_.x, _e25.g0_.x, _e28.g0_.x) * vec4<f32>(_e32.g0_.y, _e35.g0_.x, _e38.g0_.y, _e41.g0_.z))));
}

fn translator_point_left_contraction(self_1684: Translator, other_1472: Point) -> Motor {
    var self_1685: Translator;
    var other_1473: Point;

    self_1685 = self_1684;
    other_1473 = other_1472;
    let _e4: Translator = self_1685;
    let _e8: Point = other_1473;
    let _e20: Translator = self_1685;
    let _e23: Translator = self_1685;
    let _e26: Translator = self_1685;
    let _e29: Translator = self_1685;
    let _e33: Point = other_1473;
    let _e36: Point = other_1473;
    let _e39: Point = other_1473;
    let _e42: Point = other_1473;
    return Motor((((vec4<f32>(_e4.g0_.z) * vec4<f32>(_e8.g0_.z)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0)) + ((vec4<f32>(_e20.g0_.y, _e23.g0_.x, _e26.g0_.x, _e29.g0_.x) * vec4<f32>(_e33.g0_.y, _e36.g0_.x, _e39.g0_.y, _e42.g0_.z)) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0))));
}

fn translator_point_right_contraction(self_1686: Translator, other_1474: Point) -> Scalar {
    var self_1687: Translator;
    var other_1475: Point;

    self_1687 = self_1686;
    other_1475 = other_1474;
    let _e5: Translator = self_1687;
    let _e8: Point = other_1475;
    let _e13: Translator = self_1687;
    let _e16: Point = other_1475;
    return Scalar(((0.0 - (_e5.g0_.y * _e8.g0_.y)) - (_e13.g0_.z * _e16.g0_.z)));
}

fn translator_point_left_anti_contraction(self_1688: Translator, other_1476: Point) -> AntiScalar {
    var self_1689: Translator;
    var other_1477: Point;

    self_1689 = self_1688;
    other_1477 = other_1476;
    let _e4: Translator = self_1689;
    let _e7: Point = other_1477;
    let _e11: Translator = self_1689;
    let _e14: Point = other_1477;
    return AntiScalar(((_e4.g0_.y * _e7.g0_.y) + (_e11.g0_.z * _e14.g0_.z)));
}

fn translator_point_right_anti_contraction(self_1690: Translator, other_1478: Point) -> MotorDual {
    var self_1691: Translator;
    var other_1479: Point;

    self_1691 = self_1690;
    other_1479 = other_1478;
    let _e4: Translator = self_1691;
    let _e8: Point = other_1479;
    let _e19: Translator = self_1691;
    let _e22: Translator = self_1691;
    let _e25: Translator = self_1691;
    let _e28: Translator = self_1691;
    let _e32: Point = other_1479;
    let _e35: Point = other_1479;
    let _e38: Point = other_1479;
    let _e41: Point = other_1479;
    return MotorDual((((vec4<f32>(_e4.g0_.z) * vec4<f32>(_e8.g0_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) + (vec4<f32>(_e19.g0_.y, _e22.g0_.x, _e25.g0_.x, _e28.g0_.x) * vec4<f32>(_e32.g0_.y, _e35.g0_.x, _e38.g0_.y, _e41.g0_.z))));
}

fn translator_point_scalar_product(self_1692: Translator, other_1480: Point) -> Scalar {
    var self_1693: Translator;
    var other_1481: Point;

    self_1693 = self_1692;
    other_1481 = other_1480;
    let _e5: Translator = self_1693;
    let _e8: Point = other_1481;
    let _e13: Translator = self_1693;
    let _e16: Point = other_1481;
    return Scalar(((0.0 - (_e5.g0_.y * _e8.g0_.y)) - (_e13.g0_.z * _e16.g0_.z)));
}

fn translator_point_anti_scalar_product(self_1694: Translator, other_1482: Point) -> AntiScalar {
    var self_1695: Translator;
    var other_1483: Point;

    self_1695 = self_1694;
    other_1483 = other_1482;
    let _e4: Translator = self_1695;
    let _e7: Point = other_1483;
    let _e11: Translator = self_1695;
    let _e14: Point = other_1483;
    return AntiScalar(((_e4.g0_.y * _e7.g0_.y) + (_e11.g0_.z * _e14.g0_.z)));
}

fn translator_ideal_point_into(self_1696: Translator) -> IdealPoint {
    var self_1697: Translator;

    self_1697 = self_1696;
    let _e2: Translator = self_1697;
    let _e5: Translator = self_1697;
    return IdealPoint(vec2<f32>(_e2.g0_.y, _e5.g0_.z));
}

fn translator_ideal_point_add(self_1698: Translator, other_1484: IdealPoint) -> Translator {
    var self_1699: Translator;
    var other_1485: IdealPoint;

    self_1699 = self_1698;
    other_1485 = other_1484;
    let _e4: Translator = self_1699;
    let _e6: IdealPoint = other_1485;
    let _e9: IdealPoint = other_1485;
    let _e12: IdealPoint = other_1485;
    return Translator((_e4.g0_ + (vec3<f32>(_e6.g0_.x, _e9.g0_.x, _e12.g0_.y) * vec3<f32>(0.0, 1.0, 1.0))));
}

fn translator_ideal_point_sub(self_1700: Translator, other_1486: IdealPoint) -> Translator {
    var self_1701: Translator;
    var other_1487: IdealPoint;

    self_1701 = self_1700;
    other_1487 = other_1486;
    let _e4: Translator = self_1701;
    let _e6: IdealPoint = other_1487;
    let _e9: IdealPoint = other_1487;
    let _e12: IdealPoint = other_1487;
    return Translator((_e4.g0_ - (vec3<f32>(_e6.g0_.x, _e9.g0_.x, _e12.g0_.y) * vec3<f32>(0.0, 1.0, 1.0))));
}

fn translator_ideal_point_geometric_product(self_1702: Translator, other_1488: IdealPoint) -> Motor {
    var self_1703: Translator;
    var other_1489: IdealPoint;

    self_1703 = self_1702;
    other_1489 = other_1488;
    let _e4: Translator = self_1703;
    let _e8: IdealPoint = other_1489;
    let _e11: IdealPoint = other_1489;
    let _e14: IdealPoint = other_1489;
    let _e17: IdealPoint = other_1489;
    let _e30: Translator = self_1703;
    let _e33: Translator = self_1703;
    let _e36: Translator = self_1703;
    let _e39: Translator = self_1703;
    let _e43: IdealPoint = other_1489;
    let _e46: IdealPoint = other_1489;
    let _e49: IdealPoint = other_1489;
    let _e52: IdealPoint = other_1489;
    return Motor((((vec4<f32>(_e4.g0_.z) * vec4<f32>(_e8.g0_.y, _e11.g0_.x, _e14.g0_.y, _e17.g0_.y)) * vec4<f32>(-(1.0), -(1.0), 0.0, 0.0)) + ((vec4<f32>(_e30.g0_.y, _e33.g0_.y, _e36.g0_.x, _e39.g0_.x) * vec4<f32>(_e43.g0_.x, _e46.g0_.y, _e49.g0_.x, _e52.g0_.y)) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0))));
}

fn translator_ideal_point_outer_product(self_1704: Translator, other_1490: IdealPoint) -> IdealPoint {
    var self_1705: Translator;
    var other_1491: IdealPoint;

    self_1705 = self_1704;
    other_1491 = other_1490;
    let _e4: Translator = self_1705;
    let _e8: IdealPoint = other_1491;
    return IdealPoint((vec2<f32>(_e4.g0_.x) * _e8.g0_));
}

fn translator_ideal_point_inner_product(self_1706: Translator, other_1492: IdealPoint) -> Translator {
    var self_1707: Translator;
    var other_1493: IdealPoint;

    self_1707 = self_1706;
    other_1493 = other_1492;
    let _e4: Translator = self_1707;
    let _e8: IdealPoint = other_1493;
    let _e19: Translator = self_1707;
    let _e22: IdealPoint = other_1493;
    let _e25: IdealPoint = other_1493;
    let _e28: IdealPoint = other_1493;
    return Translator((((vec3<f32>(_e4.g0_.z) * vec3<f32>(_e8.g0_.y)) * vec3<f32>(-(1.0), 0.0, 0.0)) + ((_e19.g0_.yxx * vec3<f32>(_e22.g0_.x, _e25.g0_.x, _e28.g0_.y)) * vec3<f32>(-(1.0), 1.0, 1.0))));
}

fn translator_ideal_point_geometric_anti_product(self_1708: Translator, other_1494: IdealPoint) -> MotorDual {
    var self_1709: Translator;
    var other_1495: IdealPoint;

    self_1709 = self_1708;
    other_1495 = other_1494;
    let _e4: Translator = self_1709;
    let _e8: IdealPoint = other_1495;
    let _e11: IdealPoint = other_1495;
    let _e14: IdealPoint = other_1495;
    let _e17: IdealPoint = other_1495;
    let _e28: Translator = self_1709;
    let _e31: Translator = self_1709;
    let _e34: Translator = self_1709;
    let _e37: Translator = self_1709;
    let _e41: IdealPoint = other_1495;
    let _e44: IdealPoint = other_1495;
    let _e47: IdealPoint = other_1495;
    let _e50: IdealPoint = other_1495;
    return MotorDual((((vec4<f32>(_e4.g0_.z) * vec4<f32>(_e8.g0_.y, _e11.g0_.x, _e14.g0_.y, _e17.g0_.y)) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) + ((vec4<f32>(_e28.g0_.y, _e31.g0_.y, _e34.g0_.x, _e37.g0_.x) * vec4<f32>(_e41.g0_.x, _e44.g0_.y, _e47.g0_.x, _e50.g0_.y)) * vec4<f32>(1.0, -(1.0), 1.0, 1.0))));
}

fn translator_ideal_point_left_contraction(self_1710: Translator, other_1496: IdealPoint) -> Translator {
    var self_1711: Translator;
    var other_1497: IdealPoint;

    self_1711 = self_1710;
    other_1497 = other_1496;
    let _e4: Translator = self_1711;
    let _e8: IdealPoint = other_1497;
    let _e19: Translator = self_1711;
    let _e22: IdealPoint = other_1497;
    let _e25: IdealPoint = other_1497;
    let _e28: IdealPoint = other_1497;
    return Translator((((vec3<f32>(_e4.g0_.z) * vec3<f32>(_e8.g0_.y)) * vec3<f32>(-(1.0), 0.0, 0.0)) + ((_e19.g0_.yxx * vec3<f32>(_e22.g0_.x, _e25.g0_.x, _e28.g0_.y)) * vec3<f32>(-(1.0), 1.0, 1.0))));
}

fn translator_ideal_point_right_contraction(self_1712: Translator, other_1498: IdealPoint) -> Scalar {
    var self_1713: Translator;
    var other_1499: IdealPoint;

    self_1713 = self_1712;
    other_1499 = other_1498;
    let _e5: Translator = self_1713;
    let _e8: IdealPoint = other_1499;
    let _e13: Translator = self_1713;
    let _e16: IdealPoint = other_1499;
    return Scalar(((0.0 - (_e5.g0_.y * _e8.g0_.x)) - (_e13.g0_.z * _e16.g0_.y)));
}

fn translator_ideal_point_left_anti_contraction(self_1714: Translator, other_1500: IdealPoint) -> AntiScalar {
    var self_1715: Translator;
    var other_1501: IdealPoint;

    self_1715 = self_1714;
    other_1501 = other_1500;
    let _e4: Translator = self_1715;
    let _e7: IdealPoint = other_1501;
    let _e11: Translator = self_1715;
    let _e14: IdealPoint = other_1501;
    return AntiScalar(((_e4.g0_.y * _e7.g0_.x) + (_e11.g0_.z * _e14.g0_.y)));
}

fn translator_ideal_point_scalar_product(self_1716: Translator, other_1502: IdealPoint) -> Scalar {
    var self_1717: Translator;
    var other_1503: IdealPoint;

    self_1717 = self_1716;
    other_1503 = other_1502;
    let _e5: Translator = self_1717;
    let _e8: IdealPoint = other_1503;
    let _e13: Translator = self_1717;
    let _e16: IdealPoint = other_1503;
    return Scalar(((0.0 - (_e5.g0_.y * _e8.g0_.x)) - (_e13.g0_.z * _e16.g0_.y)));
}

fn translator_ideal_point_anti_scalar_product(self_1718: Translator, other_1504: IdealPoint) -> AntiScalar {
    var self_1719: Translator;
    var other_1505: IdealPoint;

    self_1719 = self_1718;
    other_1505 = other_1504;
    let _e4: Translator = self_1719;
    let _e7: IdealPoint = other_1505;
    let _e11: Translator = self_1719;
    let _e14: IdealPoint = other_1505;
    return AntiScalar(((_e4.g0_.y * _e7.g0_.x) + (_e11.g0_.z * _e14.g0_.y)));
}

fn translator_plane_geometric_product(self_1720: Translator, other_1506: Plane) -> MotorDual {
    var self_1721: Translator;
    var other_1507: Plane;

    self_1721 = self_1720;
    other_1507 = other_1506;
    let _e4: Translator = self_1721;
    let _e8: Plane = other_1507;
    let _e11: Plane = other_1507;
    let _e14: Plane = other_1507;
    let _e17: Plane = other_1507;
    let _e29: Translator = self_1721;
    let _e33: Plane = other_1507;
    let _e36: Plane = other_1507;
    let _e39: Plane = other_1507;
    let _e42: Plane = other_1507;
    let _e55: Translator = self_1721;
    let _e59: Plane = other_1507;
    let _e62: Plane = other_1507;
    let _e65: Plane = other_1507;
    let _e68: Plane = other_1507;
    return MotorDual(((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y, _e11.g0_.z, _e14.g0_.y, _e17.g0_.x)) * vec4<f32>(1.0, 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e29.g0_.z) * vec4<f32>(_e33.g0_.z, _e36.g0_.y, _e39.g0_.x, _e42.g0_.z)) * vec4<f32>(1.0, -(1.0), 1.0, 0.0))) + ((vec4<f32>(_e55.g0_.x) * vec4<f32>(_e59.g0_.x, _e62.g0_.x, _e65.g0_.y, _e68.g0_.z)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn translator_plane_regressive_product(self_1722: Translator, other_1508: Plane) -> Scalar {
    var self_1723: Translator;
    var other_1509: Plane;

    self_1723 = self_1722;
    other_1509 = other_1508;
    let _e4: Translator = self_1723;
    let _e7: Plane = other_1509;
    let _e11: Translator = self_1723;
    let _e14: Plane = other_1509;
    return Scalar(((_e4.g0_.y * _e7.g0_.y) + (_e11.g0_.z * _e14.g0_.z)));
}

fn translator_plane_outer_product(self_1724: Translator, other_1510: Plane) -> MotorDual {
    var self_1725: Translator;
    var other_1511: Plane;

    self_1725 = self_1724;
    other_1511 = other_1510;
    let _e4: Translator = self_1725;
    let _e8: Plane = other_1511;
    let _e19: Translator = self_1725;
    let _e22: Translator = self_1725;
    let _e25: Translator = self_1725;
    let _e28: Translator = self_1725;
    let _e32: Plane = other_1511;
    let _e35: Plane = other_1511;
    let _e38: Plane = other_1511;
    let _e41: Plane = other_1511;
    return MotorDual((((vec4<f32>(_e4.g0_.z) * vec4<f32>(_e8.g0_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) + (vec4<f32>(_e19.g0_.y, _e22.g0_.x, _e25.g0_.x, _e28.g0_.x) * vec4<f32>(_e32.g0_.y, _e35.g0_.x, _e38.g0_.y, _e41.g0_.z))));
}

fn translator_plane_inner_product(self_1726: Translator, other_1512: Plane) -> Plane {
    var self_1727: Translator;
    var other_1513: Plane;

    self_1727 = self_1726;
    other_1513 = other_1512;
    let _e4: Translator = self_1727;
    let _e8: Plane = other_1513;
    let _e11: Translator = self_1727;
    let _e15: Plane = other_1513;
    let _e26: Translator = self_1727;
    let _e29: Plane = other_1513;
    return Plane((((vec3<f32>(_e4.g0_.x) * _e8.g0_) + ((vec3<f32>(_e11.g0_.z) * _e15.g0_.yxy) * vec3<f32>(-(1.0), 1.0, 0.0))) + ((_e26.g0_.yxy * _e29.g0_.zxx) * vec3<f32>(1.0, 0.0, -(1.0)))));
}

fn translator_plane_geometric_anti_product(self_1728: Translator, other_1514: Plane) -> Motor {
    var self_1729: Translator;
    var other_1515: Plane;

    self_1729 = self_1728;
    other_1515 = other_1514;
    let _e4: Translator = self_1729;
    let _e8: Plane = other_1515;
    let _e11: Plane = other_1515;
    let _e14: Plane = other_1515;
    let _e17: Plane = other_1515;
    let _e29: Translator = self_1729;
    let _e33: Plane = other_1515;
    let _e36: Plane = other_1515;
    let _e39: Plane = other_1515;
    let _e42: Plane = other_1515;
    let _e55: Translator = self_1729;
    let _e59: Plane = other_1515;
    let _e62: Plane = other_1515;
    let _e65: Plane = other_1515;
    let _e68: Plane = other_1515;
    return Motor(((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y, _e11.g0_.z, _e14.g0_.y, _e17.g0_.x)) * vec4<f32>(1.0, 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e29.g0_.z) * vec4<f32>(_e33.g0_.z, _e36.g0_.y, _e39.g0_.x, _e42.g0_.z)) * vec4<f32>(1.0, -(1.0), 1.0, 0.0))) + ((vec4<f32>(_e55.g0_.x) * vec4<f32>(_e59.g0_.x, _e62.g0_.x, _e65.g0_.y, _e68.g0_.z)) * vec4<f32>(0.0, -(1.0), -(1.0), -(1.0)))));
}

fn translator_plane_inner_anti_product(self_1730: Translator, other_1516: Plane) -> Point {
    var self_1731: Translator;
    var other_1517: Plane;

    self_1731 = self_1730;
    other_1517 = other_1516;
    let _e6: Translator = self_1731;
    let _e10: Plane = other_1517;
    let _e14: Translator = self_1731;
    let _e18: Plane = other_1517;
    let _e29: Translator = self_1731;
    let _e32: Plane = other_1517;
    return Point((((vec3<f32>(0.0) - (vec3<f32>(_e6.g0_.x) * _e10.g0_)) + ((vec3<f32>(_e14.g0_.z) * _e18.g0_.yxy) * vec3<f32>(-(1.0), 1.0, 0.0))) + ((_e29.g0_.yxy * _e32.g0_.zxx) * vec3<f32>(1.0, 0.0, -(1.0)))));
}

fn translator_plane_left_contraction(self_1732: Translator, other_1518: Plane) -> Plane {
    var self_1733: Translator;
    var other_1519: Plane;

    self_1733 = self_1732;
    other_1519 = other_1518;
    let _e4: Translator = self_1733;
    let _e8: Plane = other_1519;
    return Plane((vec3<f32>(_e4.g0_.x) * _e8.g0_));
}

fn translator_plane_right_contraction(self_1734: Translator, other_1520: Plane) -> Plane {
    var self_1735: Translator;
    var other_1521: Plane;

    self_1735 = self_1734;
    other_1521 = other_1520;
    let _e4: Translator = self_1735;
    let _e8: Plane = other_1521;
    let _e18: Translator = self_1735;
    let _e21: Plane = other_1521;
    return Plane((((vec3<f32>(_e4.g0_.z) * _e8.g0_.yxy) * vec3<f32>(-(1.0), 1.0, 0.0)) + ((_e18.g0_.yxy * _e21.g0_.zxx) * vec3<f32>(1.0, 0.0, -(1.0)))));
}

fn translator_plane_left_anti_contraction(self_1736: Translator, other_1522: Plane) -> Point {
    var self_1737: Translator;
    var other_1523: Plane;

    self_1737 = self_1736;
    other_1523 = other_1522;
    let _e4: Translator = self_1737;
    let _e8: Plane = other_1523;
    let _e18: Translator = self_1737;
    let _e21: Plane = other_1523;
    return Point((((vec3<f32>(_e4.g0_.z) * _e8.g0_.yxy) * vec3<f32>(-(1.0), 1.0, 0.0)) + ((_e18.g0_.yxy * _e21.g0_.zxx) * vec3<f32>(1.0, 0.0, -(1.0)))));
}

fn translator_plane_right_anti_contraction(self_1738: Translator, other_1524: Plane) -> Point {
    var self_1739: Translator;
    var other_1525: Plane;

    self_1739 = self_1738;
    other_1525 = other_1524;
    let _e6: Translator = self_1739;
    let _e10: Plane = other_1525;
    return Point((vec3<f32>(0.0) - (vec3<f32>(_e6.g0_.x) * _e10.g0_)));
}

fn translator_translator_add(self_1740: Translator, other_1526: Translator) -> Translator {
    var self_1741: Translator;
    var other_1527: Translator;

    self_1741 = self_1740;
    other_1527 = other_1526;
    let _e4: Translator = self_1741;
    let _e6: Translator = other_1527;
    return Translator((_e4.g0_ + _e6.g0_));
}

fn translator_translator_sub(self_1742: Translator, other_1528: Translator) -> Translator {
    var self_1743: Translator;
    var other_1529: Translator;

    self_1743 = self_1742;
    other_1529 = other_1528;
    let _e4: Translator = self_1743;
    let _e6: Translator = other_1529;
    return Translator((_e4.g0_ - _e6.g0_));
}

fn translator_translator_mul(self_1744: Translator, other_1530: Translator) -> Translator {
    var self_1745: Translator;
    var other_1531: Translator;

    self_1745 = self_1744;
    other_1531 = other_1530;
    let _e4: Translator = self_1745;
    let _e6: Translator = other_1531;
    return Translator((_e4.g0_ * _e6.g0_));
}

fn translator_translator_div(self_1746: Translator, other_1532: Translator) -> Translator {
    var self_1747: Translator;
    var other_1533: Translator;

    self_1747 = self_1746;
    other_1533 = other_1532;
    let _e4: Translator = self_1747;
    let _e7: Translator = self_1747;
    let _e10: Translator = self_1747;
    let _e19: Translator = other_1533;
    let _e22: Translator = other_1533;
    let _e25: Translator = other_1533;
    return Translator((((vec3<f32>(_e4.g0_.x, _e7.g0_.y, _e10.g0_.z) * vec3<f32>(1.0, 1.0, 1.0)) / vec3<f32>(_e19.g0_.x, _e22.g0_.y, _e25.g0_.z)) * vec3<f32>(1.0, 1.0, 1.0)));
}

fn translator_translator_geometric_product(self_1748: Translator, other_1534: Translator) -> Motor {
    var self_1749: Translator;
    var other_1535: Translator;

    self_1749 = self_1748;
    other_1535 = other_1534;
    let _e4: Translator = self_1749;
    let _e8: Translator = other_1535;
    let _e11: Translator = other_1535;
    let _e14: Translator = other_1535;
    let _e17: Translator = other_1535;
    let _e29: Translator = self_1749;
    let _e33: Translator = other_1535;
    let _e36: Translator = other_1535;
    let _e39: Translator = other_1535;
    let _e42: Translator = other_1535;
    let _e56: Translator = self_1749;
    let _e60: Translator = other_1535;
    let _e63: Translator = other_1535;
    let _e66: Translator = other_1535;
    let _e69: Translator = other_1535;
    return Motor(((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y, _e11.g0_.z, _e14.g0_.x, _e17.g0_.y)) * vec4<f32>(-(1.0), 1.0, 1.0, 0.0)) + ((vec4<f32>(_e29.g0_.z) * vec4<f32>(_e33.g0_.z, _e36.g0_.y, _e39.g0_.z, _e42.g0_.x)) * vec4<f32>(-(1.0), -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e56.g0_.x) * vec4<f32>(_e60.g0_.x, _e63.g0_.x, _e66.g0_.y, _e69.g0_.z)) * vec4<f32>(1.0, 0.0, 1.0, 1.0))));
}

fn translator_translator_outer_product(self_1750: Translator, other_1536: Translator) -> Translator {
    var self_1751: Translator;
    var other_1537: Translator;

    self_1751 = self_1750;
    other_1537 = other_1536;
    let _e4: Translator = self_1751;
    let _e8: Translator = other_1537;
    let _e11: Translator = self_1751;
    let _e13: Translator = other_1537;
    return Translator(((vec3<f32>(_e4.g0_.x) * _e8.g0_) + ((_e11.g0_ * vec3<f32>(_e13.g0_.x)) * vec3<f32>(0.0, 1.0, 1.0))));
}

fn translator_translator_inner_product(self_1752: Translator, other_1538: Translator) -> Translator {
    var self_1753: Translator;
    var other_1539: Translator;

    self_1753 = self_1752;
    other_1539 = other_1538;
    let _e4: Translator = self_1753;
    let _e8: Translator = other_1539;
    let _e11: Translator = self_1753;
    let _e15: Translator = other_1539;
    let _e26: Translator = self_1753;
    let _e29: Translator = other_1539;
    return Translator((((vec3<f32>(_e4.g0_.x) * _e8.g0_) + ((vec3<f32>(_e11.g0_.z) * _e15.g0_.zzx) * vec3<f32>(-(1.0), 0.0, 1.0))) + ((_e26.g0_.yyx * _e29.g0_.yxx) * vec3<f32>(-(1.0), 1.0, 0.0))));
}

fn translator_translator_geometric_anti_product(self_1754: Translator, other_1540: Translator) -> MotorDual {
    var self_1755: Translator;
    var other_1541: Translator;

    self_1755 = self_1754;
    other_1541 = other_1540;
    let _e4: Translator = self_1755;
    let _e8: Translator = other_1541;
    let _e11: Translator = other_1541;
    let _e14: Translator = other_1541;
    let _e17: Translator = other_1541;
    let _e29: Translator = self_1755;
    let _e33: Translator = other_1541;
    let _e36: Translator = other_1541;
    let _e39: Translator = other_1541;
    let _e42: Translator = other_1541;
    let _e54: Translator = self_1755;
    let _e58: Translator = other_1541;
    let _e61: Translator = other_1541;
    let _e64: Translator = other_1541;
    let _e67: Translator = other_1541;
    return MotorDual(((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y, _e11.g0_.z, _e14.g0_.x, _e17.g0_.y)) * vec4<f32>(1.0, -(1.0), 1.0, 0.0)) + ((vec4<f32>(_e29.g0_.z) * vec4<f32>(_e33.g0_.z, _e36.g0_.y, _e39.g0_.z, _e42.g0_.x)) * vec4<f32>(1.0, 1.0, 0.0, 1.0))) + ((vec4<f32>(_e54.g0_.x) * vec4<f32>(_e58.g0_.x, _e61.g0_.x, _e64.g0_.y, _e67.g0_.z)) * vec4<f32>(-(1.0), 0.0, 1.0, 1.0))));
}

fn translator_translator_left_contraction(self_1756: Translator, other_1542: Translator) -> Translator {
    var self_1757: Translator;
    var other_1543: Translator;

    self_1757 = self_1756;
    other_1543 = other_1542;
    let _e4: Translator = self_1757;
    let _e8: Translator = other_1543;
    let _e11: Translator = self_1757;
    let _e15: Translator = other_1543;
    let _e27: Translator = self_1757;
    let _e30: Translator = other_1543;
    return Translator((((vec3<f32>(_e4.g0_.x) * _e8.g0_) + ((vec3<f32>(_e11.g0_.z) * vec3<f32>(_e15.g0_.z)) * vec3<f32>(-(1.0), 0.0, 0.0))) + ((_e27.g0_.yxx * _e30.g0_.yxx) * vec3<f32>(-(1.0), 0.0, 0.0))));
}

fn translator_translator_right_contraction(self_1758: Translator, other_1544: Translator) -> Translator {
    var self_1759: Translator;
    var other_1545: Translator;

    self_1759 = self_1758;
    other_1545 = other_1544;
    let _e4: Translator = self_1759;
    let _e8: Translator = other_1545;
    let _e18: Translator = self_1759;
    let _e22: Translator = other_1545;
    let _e33: Translator = self_1759;
    let _e37: Translator = other_1545;
    return Translator(((((vec3<f32>(_e4.g0_.y) * _e8.g0_.yxy) * vec3<f32>(-(1.0), 1.0, 0.0)) + ((vec3<f32>(_e18.g0_.z) * _e22.g0_.zzx) * vec3<f32>(-(1.0), 0.0, 1.0))) + ((vec3<f32>(_e33.g0_.x) * vec3<f32>(_e37.g0_.x)) * vec3<f32>(1.0, 0.0, 0.0))));
}

fn translator_translator_scalar_product(self_1760: Translator, other_1546: Translator) -> Scalar {
    var self_1761: Translator;
    var other_1547: Translator;

    self_1761 = self_1760;
    other_1547 = other_1546;
    let _e4: Translator = self_1761;
    let _e7: Translator = other_1547;
    let _e11: Translator = self_1761;
    let _e14: Translator = other_1547;
    let _e19: Translator = self_1761;
    let _e22: Translator = other_1547;
    return Scalar((((_e4.g0_.x * _e7.g0_.x) - (_e11.g0_.y * _e14.g0_.y)) - (_e19.g0_.z * _e22.g0_.z)));
}

fn translator_translator_anti_scalar_product(self_1762: Translator, other_1548: Translator) -> AntiScalar {
    var self_1763: Translator;
    var other_1549: Translator;

    self_1763 = self_1762;
    other_1549 = other_1548;
    let _e5: Translator = self_1763;
    let _e8: Translator = other_1549;
    let _e13: Translator = self_1763;
    let _e16: Translator = other_1549;
    let _e21: Translator = self_1763;
    let _e24: Translator = other_1549;
    return AntiScalar((((0.0 - (_e5.g0_.x * _e8.g0_.x)) + (_e13.g0_.y * _e16.g0_.y)) + (_e21.g0_.z * _e24.g0_.z)));
}

fn translator_motor_add(self_1764: Translator, other_1550: Motor) -> Motor {
    var self_1765: Translator;
    var other_1551: Motor;

    self_1765 = self_1764;
    other_1551 = other_1550;
    let _e4: Translator = self_1765;
    let _e7: Translator = self_1765;
    let _e10: Translator = self_1765;
    let _e13: Translator = self_1765;
    let _e23: Motor = other_1551;
    return Motor(((vec4<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.y, _e13.g0_.z) * vec4<f32>(1.0, 0.0, 1.0, 1.0)) + _e23.g0_));
}

fn translator_motor_sub(self_1766: Translator, other_1552: Motor) -> Motor {
    var self_1767: Translator;
    var other_1553: Motor;

    self_1767 = self_1766;
    other_1553 = other_1552;
    let _e4: Translator = self_1767;
    let _e7: Translator = self_1767;
    let _e10: Translator = self_1767;
    let _e13: Translator = self_1767;
    let _e23: Motor = other_1553;
    return Motor(((vec4<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.y, _e13.g0_.z) * vec4<f32>(1.0, 0.0, 1.0, 1.0)) - _e23.g0_));
}

fn translator_motor_geometric_product(self_1768: Translator, other_1554: Motor) -> Motor {
    var self_1769: Translator;
    var other_1555: Motor;

    self_1769 = self_1768;
    other_1555 = other_1554;
    let _e4: Translator = self_1769;
    let _e8: Motor = other_1555;
    let _e11: Translator = self_1769;
    let _e15: Motor = other_1555;
    let _e28: Translator = self_1769;
    let _e32: Motor = other_1555;
    return Motor((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.y) * _e15.g0_.zwxy) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e28.g0_.z) * _e32.g0_.wzyx) * vec4<f32>(-(1.0), -(1.0), 1.0, 1.0))));
}

fn translator_motor_regressive_product(self_1770: Translator, other_1556: Motor) -> Plane {
    var self_1771: Translator;
    var other_1557: Motor;

    self_1771 = self_1770;
    other_1557 = other_1556;
    let _e4: Translator = self_1771;
    let _e8: Motor = other_1557;
    let _e11: Motor = other_1557;
    let _e14: Motor = other_1557;
    let _e25: Translator = self_1771;
    let _e28: Motor = other_1557;
    let _e31: Motor = other_1557;
    let _e34: Motor = other_1557;
    return Plane((((vec3<f32>(_e4.g0_.z) * vec3<f32>(_e8.g0_.z, _e11.g0_.y, _e14.g0_.z)) * vec3<f32>(1.0, -(1.0), 0.0)) + ((_e25.g0_.yxy * vec3<f32>(_e28.g0_.w, _e31.g0_.x, _e34.g0_.y)) * vec3<f32>(-(1.0), 0.0, 1.0))));
}

fn translator_motor_outer_product(self_1772: Translator, other_1558: Motor) -> Motor {
    var self_1773: Translator;
    var other_1559: Motor;

    self_1773 = self_1772;
    other_1559 = other_1558;
    let _e4: Translator = self_1773;
    let _e8: Motor = other_1559;
    let _e11: Translator = self_1773;
    let _e14: Translator = self_1773;
    let _e17: Translator = self_1773;
    let _e20: Translator = self_1773;
    let _e24: Motor = other_1559;
    return Motor(((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.x, _e14.g0_.x, _e17.g0_.y, _e20.g0_.z) * vec4<f32>(_e24.g0_.x)) * vec4<f32>(0.0, 0.0, 1.0, 1.0))));
}

fn translator_motor_inner_product(self_1774: Translator, other_1560: Motor) -> Motor {
    var self_1775: Translator;
    var other_1561: Motor;

    self_1775 = self_1774;
    other_1561 = other_1560;
    let _e4: Translator = self_1775;
    let _e8: Motor = other_1561;
    let _e11: Translator = self_1775;
    let _e15: Motor = other_1561;
    let _e27: Translator = self_1775;
    let _e30: Translator = self_1775;
    let _e33: Translator = self_1775;
    let _e36: Translator = self_1775;
    let _e40: Motor = other_1561;
    return Motor((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.z) * _e15.g0_.wwwx) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))) + ((vec4<f32>(_e27.g0_.y, _e30.g0_.x, _e33.g0_.y, _e36.g0_.x) * _e40.g0_.zxxx) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0))));
}

fn translator_motor_geometric_anti_product(self_1776: Translator, other_1562: Motor) -> MotorDual {
    var self_1777: Translator;
    var other_1563: Motor;

    self_1777 = self_1776;
    other_1563 = other_1562;
    let _e4: Translator = self_1777;
    let _e8: Motor = other_1563;
    let _e18: Translator = self_1777;
    let _e22: Motor = other_1563;
    let _e34: Translator = self_1777;
    let _e38: Motor = other_1563;
    return MotorDual(((((vec4<f32>(_e4.g0_.x) * _e8.g0_) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0)) + ((vec4<f32>(_e18.g0_.y) * _e22.g0_.zwxy) * vec4<f32>(1.0, -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e34.g0_.z) * _e38.g0_.wzyx) * vec4<f32>(1.0, 1.0, -(1.0), 1.0))));
}

fn translator_motor_inner_anti_product(self_1778: Translator, other_1564: Motor) -> MotorDual {
    var self_1779: Translator;
    var other_1565: Motor;

    self_1779 = self_1778;
    other_1565 = other_1564;
    let _e4: Translator = self_1779;
    let _e8: Motor = other_1565;
    let _e18: Translator = self_1779;
    let _e22: Motor = other_1565;
    let _e33: Translator = self_1779;
    let _e36: Translator = self_1779;
    let _e39: Translator = self_1779;
    let _e42: Translator = self_1779;
    let _e46: Motor = other_1565;
    return MotorDual(((((vec4<f32>(_e4.g0_.x) * _e8.g0_) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0)) + ((vec4<f32>(_e18.g0_.z) * _e22.g0_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e33.g0_.y, _e36.g0_.x, _e39.g0_.y, _e42.g0_.x) * _e46.g0_.zxxx) * vec4<f32>(1.0, 0.0, 1.0, 0.0))));
}

fn translator_motor_left_contraction(self_1780: Translator, other_1566: Motor) -> Motor {
    var self_1781: Translator;
    var other_1567: Motor;

    self_1781 = self_1780;
    other_1567 = other_1566;
    let _e4: Translator = self_1781;
    let _e8: Motor = other_1567;
    let _e11: Translator = self_1781;
    let _e15: Motor = other_1567;
    let _e28: Translator = self_1781;
    let _e31: Translator = self_1781;
    let _e34: Translator = self_1781;
    let _e37: Translator = self_1781;
    let _e41: Motor = other_1567;
    return Motor((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.z) * vec4<f32>(_e15.g0_.w)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((vec4<f32>(_e28.g0_.y, _e31.g0_.x, _e34.g0_.x, _e37.g0_.x) * _e41.g0_.zxxx) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))));
}

fn translator_motor_right_contraction(self_1782: Translator, other_1568: Motor) -> Translator {
    var self_1783: Translator;
    var other_1569: Motor;

    self_1783 = self_1782;
    other_1569 = other_1568;
    let _e4: Translator = self_1783;
    let _e8: Motor = other_1569;
    let _e11: Motor = other_1569;
    let _e14: Motor = other_1569;
    let _e25: Translator = self_1783;
    let _e29: Motor = other_1569;
    let _e32: Motor = other_1569;
    let _e35: Motor = other_1569;
    let _e47: Translator = self_1783;
    let _e51: Motor = other_1569;
    return Translator(((((vec3<f32>(_e4.g0_.y) * vec3<f32>(_e8.g0_.z, _e11.g0_.x, _e14.g0_.z)) * vec3<f32>(-(1.0), 1.0, 0.0)) + ((vec3<f32>(_e25.g0_.z) * vec3<f32>(_e29.g0_.w, _e32.g0_.w, _e35.g0_.x)) * vec3<f32>(-(1.0), 0.0, 1.0))) + ((vec3<f32>(_e47.g0_.x) * vec3<f32>(_e51.g0_.x)) * vec3<f32>(1.0, 0.0, 0.0))));
}

fn translator_motor_right_anti_contraction(self_1784: Translator, other_1570: Motor) -> MotorDual {
    var self_1785: Translator;
    var other_1571: Motor;

    self_1785 = self_1784;
    other_1571 = other_1570;
    let _e4: Translator = self_1785;
    let _e8: Motor = other_1571;
    let _e18: Translator = self_1785;
    let _e22: Motor = other_1571;
    let _e34: Translator = self_1785;
    let _e37: Translator = self_1785;
    let _e40: Translator = self_1785;
    let _e43: Translator = self_1785;
    let _e47: Motor = other_1571;
    return MotorDual(((((vec4<f32>(_e4.g0_.x) * _e8.g0_) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0)) + ((vec4<f32>(_e18.g0_.z) * vec4<f32>(_e22.g0_.w)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e34.g0_.y, _e37.g0_.x, _e40.g0_.x, _e43.g0_.x) * _e47.g0_.zxxx) * vec4<f32>(1.0, 0.0, 0.0, 0.0))));
}

fn translator_motor_scalar_product(self_1786: Translator, other_1572: Motor) -> Scalar {
    var self_1787: Translator;
    var other_1573: Motor;

    self_1787 = self_1786;
    other_1573 = other_1572;
    let _e4: Translator = self_1787;
    let _e7: Motor = other_1573;
    let _e11: Translator = self_1787;
    let _e14: Motor = other_1573;
    let _e19: Translator = self_1787;
    let _e22: Motor = other_1573;
    return Scalar((((_e4.g0_.x * _e7.g0_.x) - (_e11.g0_.y * _e14.g0_.z)) - (_e19.g0_.z * _e22.g0_.w)));
}

fn translator_motor_anti_scalar_product(self_1788: Translator, other_1574: Motor) -> AntiScalar {
    var self_1789: Translator;
    var other_1575: Motor;

    self_1789 = self_1788;
    other_1575 = other_1574;
    let _e5: Translator = self_1789;
    let _e8: Motor = other_1575;
    let _e13: Translator = self_1789;
    let _e16: Motor = other_1575;
    let _e21: Translator = self_1789;
    let _e24: Motor = other_1575;
    return AntiScalar((((0.0 - (_e5.g0_.x * _e8.g0_.x)) + (_e13.g0_.y * _e16.g0_.z)) + (_e21.g0_.z * _e24.g0_.w)));
}

fn translator_motor_dual_geometric_product(self_1790: Translator, other_1576: MotorDual) -> MotorDual {
    var self_1791: Translator;
    var other_1577: MotorDual;

    self_1791 = self_1790;
    other_1577 = other_1576;
    let _e4: Translator = self_1791;
    let _e8: MotorDual = other_1577;
    let _e11: Translator = self_1791;
    let _e15: MotorDual = other_1577;
    let _e28: Translator = self_1791;
    let _e32: MotorDual = other_1577;
    return MotorDual((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.y) * _e15.g0_.zwxy) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e28.g0_.z) * _e32.g0_.wzyx) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))));
}

fn translator_motor_dual_regressive_product(self_1792: Translator, other_1578: MotorDual) -> Translator {
    var self_1793: Translator;
    var other_1579: MotorDual;

    self_1793 = self_1792;
    other_1579 = other_1578;
    let _e4: Translator = self_1793;
    let _e8: MotorDual = other_1579;
    let _e11: MotorDual = other_1579;
    let _e14: MotorDual = other_1579;
    let _e24: Translator = self_1793;
    let _e28: MotorDual = other_1579;
    let _e31: MotorDual = other_1579;
    let _e34: MotorDual = other_1579;
    let _e45: Translator = self_1793;
    let _e49: MotorDual = other_1579;
    return Translator(((((vec3<f32>(_e4.g0_.y) * vec3<f32>(_e8.g0_.z, _e11.g0_.x, _e14.g0_.z)) * vec3<f32>(1.0, 1.0, 0.0)) + ((vec3<f32>(_e24.g0_.z) * vec3<f32>(_e28.g0_.w, _e31.g0_.w, _e34.g0_.x)) * vec3<f32>(1.0, 0.0, 1.0))) + ((vec3<f32>(_e45.g0_.x) * vec3<f32>(_e49.g0_.x)) * vec3<f32>(1.0, 0.0, 0.0))));
}

fn translator_motor_dual_outer_product(self_1794: Translator, other_1580: MotorDual) -> MotorDual {
    var self_1795: Translator;
    var other_1581: MotorDual;

    self_1795 = self_1794;
    other_1581 = other_1580;
    let _e4: Translator = self_1795;
    let _e8: MotorDual = other_1581;
    let _e11: Translator = self_1795;
    let _e15: MotorDual = other_1581;
    let _e27: Translator = self_1795;
    let _e30: Translator = self_1795;
    let _e33: Translator = self_1795;
    let _e36: Translator = self_1795;
    let _e40: MotorDual = other_1581;
    return MotorDual((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.z) * vec4<f32>(_e15.g0_.w)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e27.g0_.y, _e30.g0_.x, _e33.g0_.x, _e36.g0_.x) * _e40.g0_.zxxx) * vec4<f32>(1.0, 0.0, 0.0, 0.0))));
}

fn translator_motor_dual_inner_product(self_1796: Translator, other_1582: MotorDual) -> MotorDual {
    var self_1797: Translator;
    var other_1583: MotorDual;

    self_1797 = self_1796;
    other_1583 = other_1582;
    let _e4: Translator = self_1797;
    let _e8: MotorDual = other_1583;
    let _e11: Translator = self_1797;
    let _e15: MotorDual = other_1583;
    let _e28: Translator = self_1797;
    let _e31: Translator = self_1797;
    let _e34: Translator = self_1797;
    let _e37: Translator = self_1797;
    let _e41: MotorDual = other_1583;
    return MotorDual((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.z) * _e15.g0_.zzyx) * vec4<f32>(0.0, -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e28.g0_.x, _e31.g0_.y, _e34.g0_.y, _e37.g0_.y) * _e41.g0_.xwxy) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))));
}

fn translator_motor_dual_geometric_anti_product(self_1798: Translator, other_1584: MotorDual) -> Motor {
    var self_1799: Translator;
    var other_1585: MotorDual;

    self_1799 = self_1798;
    other_1585 = other_1584;
    let _e4: Translator = self_1799;
    let _e8: MotorDual = other_1585;
    let _e20: Translator = self_1799;
    let _e24: MotorDual = other_1585;
    let _e36: Translator = self_1799;
    let _e40: MotorDual = other_1585;
    return Motor(((((vec4<f32>(_e4.g0_.x) * _e8.g0_) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))) + ((vec4<f32>(_e20.g0_.y) * _e24.g0_.zwxy) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e36.g0_.z) * _e40.g0_.wzyx) * vec4<f32>(1.0, -(1.0), 1.0, 1.0))));
}

fn translator_motor_dual_inner_anti_product(self_1800: Translator, other_1586: MotorDual) -> Motor {
    var self_1801: Translator;
    var other_1587: MotorDual;

    self_1801 = self_1800;
    other_1587 = other_1586;
    let _e4: Translator = self_1801;
    let _e8: MotorDual = other_1587;
    let _e20: Translator = self_1801;
    let _e24: MotorDual = other_1587;
    let _e36: Translator = self_1801;
    let _e39: Translator = self_1801;
    let _e42: Translator = self_1801;
    let _e45: Translator = self_1801;
    let _e49: MotorDual = other_1587;
    return Motor(((((vec4<f32>(_e4.g0_.x) * _e8.g0_) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))) + ((vec4<f32>(_e20.g0_.z) * _e24.g0_.zzyx) * vec4<f32>(0.0, -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e36.g0_.x, _e39.g0_.y, _e42.g0_.y, _e45.g0_.y) * _e49.g0_.xwxy) * vec4<f32>(0.0, 1.0, 1.0, -(1.0)))));
}

fn translator_motor_dual_left_contraction(self_1802: Translator, other_1588: MotorDual) -> MotorDual {
    var self_1803: Translator;
    var other_1589: MotorDual;

    self_1803 = self_1802;
    other_1589 = other_1588;
    let _e4: Translator = self_1803;
    let _e8: MotorDual = other_1589;
    let _e11: Translator = self_1803;
    let _e14: Translator = self_1803;
    let _e17: Translator = self_1803;
    let _e20: Translator = self_1803;
    let _e24: MotorDual = other_1589;
    return MotorDual(((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.x, _e14.g0_.x, _e17.g0_.y, _e20.g0_.z) * vec4<f32>(_e24.g0_.x)) * vec4<f32>(0.0, 0.0, -(1.0), -(1.0)))));
}

fn translator_motor_dual_right_contraction(self_1804: Translator, other_1590: MotorDual) -> Plane {
    var self_1805: Translator;
    var other_1591: MotorDual;

    self_1805 = self_1804;
    other_1591 = other_1590;
    let _e4: Translator = self_1805;
    let _e8: MotorDual = other_1591;
    let _e11: MotorDual = other_1591;
    let _e14: MotorDual = other_1591;
    let _e25: Translator = self_1805;
    let _e28: MotorDual = other_1591;
    let _e31: MotorDual = other_1591;
    let _e34: MotorDual = other_1591;
    return Plane((((vec3<f32>(_e4.g0_.z) * vec3<f32>(_e8.g0_.z, _e11.g0_.y, _e14.g0_.z)) * vec3<f32>(-(1.0), 1.0, 0.0)) + ((_e25.g0_.yxy * vec3<f32>(_e28.g0_.w, _e31.g0_.x, _e34.g0_.y)) * vec3<f32>(1.0, 0.0, -(1.0)))));
}

fn translator_motor_dual_left_anti_contraction(self_1806: Translator, other_1592: MotorDual) -> Point {
    var self_1807: Translator;
    var other_1593: MotorDual;

    self_1807 = self_1806;
    other_1593 = other_1592;
    let _e4: Translator = self_1807;
    let _e8: MotorDual = other_1593;
    let _e11: MotorDual = other_1593;
    let _e14: MotorDual = other_1593;
    let _e25: Translator = self_1807;
    let _e28: MotorDual = other_1593;
    let _e31: MotorDual = other_1593;
    let _e34: MotorDual = other_1593;
    return Point((((vec3<f32>(_e4.g0_.z) * vec3<f32>(_e8.g0_.z, _e11.g0_.y, _e14.g0_.z)) * vec3<f32>(-(1.0), 1.0, 0.0)) + ((_e25.g0_.yxy * vec3<f32>(_e28.g0_.w, _e31.g0_.x, _e34.g0_.y)) * vec3<f32>(1.0, 0.0, -(1.0)))));
}

fn translator_motor_dual_right_anti_contraction(self_1808: Translator, other_1594: MotorDual) -> Motor {
    var self_1809: Translator;
    var other_1595: MotorDual;

    self_1809 = self_1808;
    other_1595 = other_1594;
    let _e4: Translator = self_1809;
    let _e8: MotorDual = other_1595;
    let _e20: Translator = self_1809;
    let _e23: Translator = self_1809;
    let _e26: Translator = self_1809;
    let _e29: Translator = self_1809;
    let _e33: MotorDual = other_1595;
    return Motor((((vec4<f32>(_e4.g0_.x) * _e8.g0_) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))) + ((vec4<f32>(_e20.g0_.x, _e23.g0_.x, _e26.g0_.y, _e29.g0_.z) * vec4<f32>(_e33.g0_.x)) * vec4<f32>(0.0, 0.0, 1.0, 1.0))));
}

fn translator_squared_magnitude(self_1810: Translator) -> Scalar {
    var self_1811: Translator;

    self_1811 = self_1810;
    let _e2: Translator = self_1811;
    let _e3: Translator = self_1811;
    let _e4: Translator = translator_reversal(_e3);
    let _e5: Scalar = translator_translator_scalar_product(_e2, _e4);
    return _e5;
}

fn translator_magnitude(self_1812: Translator) -> Scalar {
    var self_1813: Translator;

    self_1813 = self_1812;
    let _e2: Translator = self_1813;
    let _e3: Scalar = translator_squared_magnitude(_e2);
    return Scalar(sqrt(_e3.g0_));
}

fn translator_bulk_norm(self_1814: Translator) -> Scalar {
    var self_1815: Translator;

    self_1815 = self_1814;
    let _e2: Translator = self_1815;
    let _e3: Scalar = translator_squared_magnitude(_e2);
    return Scalar(sqrt(_e3.g0_));
}

fn translator_squared_anti_magnitude(self_1816: Translator) -> AntiScalar {
    var self_1817: Translator;

    self_1817 = self_1816;
    let _e2: Translator = self_1817;
    let _e3: Translator = self_1817;
    let _e4: Translator = translator_anti_reversal(_e3);
    let _e5: AntiScalar = translator_translator_anti_scalar_product(_e2, _e4);
    return _e5;
}

fn translator_weight_norm(self_1818: Translator) -> AntiScalar {
    var self_1819: Translator;

    self_1819 = self_1818;
    let _e2: Translator = self_1819;
    let _e3: AntiScalar = translator_squared_anti_magnitude(_e2);
    return AntiScalar(sqrt(_e3.g0_));
}

fn translator_scale(self_1820: Translator, other_1596: f32) -> Translator {
    var self_1821: Translator;
    var other_1597: f32;

    self_1821 = self_1820;
    other_1597 = other_1596;
    let _e4: Translator = self_1821;
    let _e5: f32 = other_1597;
    let _e7: Translator = translator_scalar_geometric_product(_e4, Scalar(_e5));
    return _e7;
}

fn translator_signum(self_1822: Translator) -> Translator {
    var self_1823: Translator;

    self_1823 = self_1822;
    let _e2: Translator = self_1823;
    let _e3: Translator = self_1823;
    let _e4: Scalar = translator_magnitude(_e3);
    let _e9: Translator = translator_scalar_geometric_product(_e2, Scalar((1.0 / _e4.g0_)));
    return _e9;
}

fn translator_inverse(self_1824: Translator) -> Translator {
    var self_1825: Translator;

    self_1825 = self_1824;
    let _e2: Translator = self_1825;
    let _e3: Translator = translator_reversal(_e2);
    let _e4: Translator = self_1825;
    let _e5: Scalar = translator_squared_magnitude(_e4);
    let _e10: Translator = translator_scalar_geometric_product(_e3, Scalar((1.0 / _e5.g0_)));
    return _e10;
}

fn motor_zero() -> Motor {
    return Motor(vec4<f32>(0.0));
}

fn motor_one() -> Motor {
    return Motor(vec4<f32>(1.0, 0.0, 0.0, 0.0));
}

fn motor_neg(self_1826: Motor) -> Motor {
    var self_1827: Motor;

    self_1827 = self_1826;
    let _e2: Motor = self_1827;
    return Motor((_e2.g0_ * vec4<f32>(-(1.0))));
}

fn motor_automorphism(self_1828: Motor) -> Motor {
    var self_1829: Motor;

    self_1829 = self_1828;
    let _e2: Motor = self_1829;
    return Motor(_e2.g0_);
}

fn motor_reversal(self_1830: Motor) -> Motor {
    var self_1831: Motor;

    self_1831 = self_1830;
    let _e2: Motor = self_1831;
    return Motor((_e2.g0_ * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))));
}

fn motor_conjugation(self_1832: Motor) -> Motor {
    var self_1833: Motor;

    self_1833 = self_1832;
    let _e2: Motor = self_1833;
    return Motor((_e2.g0_ * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))));
}

fn motor_dual(self_1834: Motor) -> MotorDual {
    var self_1835: Motor;

    self_1835 = self_1834;
    let _e2: Motor = self_1835;
    return MotorDual(_e2.g0_);
}

fn motor_anti_reversal(self_1836: Motor) -> Motor {
    var self_1837: Motor;

    self_1837 = self_1836;
    let _e2: Motor = self_1837;
    return Motor((_e2.g0_ * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))));
}

fn motor_scalar_into(self_1838: Motor) -> Scalar {
    var self_1839: Motor;

    self_1839 = self_1838;
    let _e2: Motor = self_1839;
    return Scalar(_e2.g0_.x);
}

fn motor_scalar_add(self_1840: Motor, other_1598: Scalar) -> Motor {
    var self_1841: Motor;
    var other_1599: Scalar;

    self_1841 = self_1840;
    other_1599 = other_1598;
    let _e4: Motor = self_1841;
    let _e6: Scalar = other_1599;
    return Motor((_e4.g0_ + (vec4<f32>(_e6.g0_) * vec4<f32>(1.0, 0.0, 0.0, 0.0))));
}

fn motor_scalar_sub(self_1842: Motor, other_1600: Scalar) -> Motor {
    var self_1843: Motor;
    var other_1601: Scalar;

    self_1843 = self_1842;
    other_1601 = other_1600;
    let _e4: Motor = self_1843;
    let _e6: Scalar = other_1601;
    return Motor((_e4.g0_ - (vec4<f32>(_e6.g0_) * vec4<f32>(1.0, 0.0, 0.0, 0.0))));
}

fn motor_scalar_geometric_product(self_1844: Motor, other_1602: Scalar) -> Motor {
    var self_1845: Motor;
    var other_1603: Scalar;

    self_1845 = self_1844;
    other_1603 = other_1602;
    let _e4: Motor = self_1845;
    let _e6: Scalar = other_1603;
    return Motor((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn motor_scalar_outer_product(self_1846: Motor, other_1604: Scalar) -> Motor {
    var self_1847: Motor;
    var other_1605: Scalar;

    self_1847 = self_1846;
    other_1605 = other_1604;
    let _e4: Motor = self_1847;
    let _e6: Scalar = other_1605;
    return Motor((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn motor_scalar_inner_product(self_1848: Motor, other_1606: Scalar) -> Motor {
    var self_1849: Motor;
    var other_1607: Scalar;

    self_1849 = self_1848;
    other_1607 = other_1606;
    let _e4: Motor = self_1849;
    let _e6: Scalar = other_1607;
    return Motor((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn motor_scalar_geometric_anti_product(self_1850: Motor, other_1608: Scalar) -> MotorDual {
    var self_1851: Motor;
    var other_1609: Scalar;

    self_1851 = self_1850;
    other_1609 = other_1608;
    let _e4: Motor = self_1851;
    let _e6: Scalar = other_1609;
    return MotorDual(((_e4.g0_ * vec4<f32>(_e6.g0_)) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0)));
}

fn motor_scalar_inner_anti_product(self_1852: Motor, other_1610: Scalar) -> MotorDual {
    var self_1853: Motor;
    var other_1611: Scalar;

    self_1853 = self_1852;
    other_1611 = other_1610;
    let _e4: Motor = self_1853;
    let _e6: Scalar = other_1611;
    return MotorDual(((_e4.g0_ * vec4<f32>(_e6.g0_)) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0)));
}

fn motor_scalar_left_contraction(self_1854: Motor, other_1612: Scalar) -> Scalar {
    var self_1855: Motor;
    var other_1613: Scalar;

    self_1855 = self_1854;
    other_1613 = other_1612;
    let _e4: Motor = self_1855;
    let _e7: Scalar = other_1613;
    return Scalar((_e4.g0_.x * _e7.g0_));
}

fn motor_scalar_right_contraction(self_1856: Motor, other_1614: Scalar) -> Motor {
    var self_1857: Motor;
    var other_1615: Scalar;

    self_1857 = self_1856;
    other_1615 = other_1614;
    let _e4: Motor = self_1857;
    let _e6: Scalar = other_1615;
    return Motor((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn motor_scalar_left_anti_contraction(self_1858: Motor, other_1616: Scalar) -> MotorDual {
    var self_1859: Motor;
    var other_1617: Scalar;

    self_1859 = self_1858;
    other_1617 = other_1616;
    let _e4: Motor = self_1859;
    let _e6: Scalar = other_1617;
    return MotorDual(((_e4.g0_ * vec4<f32>(_e6.g0_)) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0)));
}

fn motor_scalar_right_anti_contraction(self_1860: Motor, other_1618: Scalar) -> AntiScalar {
    var self_1861: Motor;
    var other_1619: Scalar;

    self_1861 = self_1860;
    other_1619 = other_1618;
    let _e5: Motor = self_1861;
    let _e8: Scalar = other_1619;
    return AntiScalar((0.0 - (_e5.g0_.x * _e8.g0_)));
}

fn motor_scalar_scalar_product(self_1862: Motor, other_1620: Scalar) -> Scalar {
    var self_1863: Motor;
    var other_1621: Scalar;

    self_1863 = self_1862;
    other_1621 = other_1620;
    let _e4: Motor = self_1863;
    let _e7: Scalar = other_1621;
    return Scalar((_e4.g0_.x * _e7.g0_));
}

fn motor_scalar_anti_scalar_product(self_1864: Motor, other_1622: Scalar) -> AntiScalar {
    var self_1865: Motor;
    var other_1623: Scalar;

    self_1865 = self_1864;
    other_1623 = other_1622;
    let _e5: Motor = self_1865;
    let _e8: Scalar = other_1623;
    return AntiScalar((0.0 - (_e5.g0_.x * _e8.g0_)));
}

fn motor_anti_scalar_geometric_product(self_1866: Motor, other_1624: AntiScalar) -> MotorDual {
    var self_1867: Motor;
    var other_1625: AntiScalar;

    self_1867 = self_1866;
    other_1625 = other_1624;
    let _e4: Motor = self_1867;
    let _e6: AntiScalar = other_1625;
    return MotorDual(((_e4.g0_ * vec4<f32>(_e6.g0_)) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))));
}

fn motor_anti_scalar_regressive_product(self_1868: Motor, other_1626: AntiScalar) -> Motor {
    var self_1869: Motor;
    var other_1627: AntiScalar;

    self_1869 = self_1868;
    other_1627 = other_1626;
    let _e4: Motor = self_1869;
    let _e6: AntiScalar = other_1627;
    return Motor((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn motor_anti_scalar_outer_product(self_1870: Motor, other_1628: AntiScalar) -> AntiScalar {
    var self_1871: Motor;
    var other_1629: AntiScalar;

    self_1871 = self_1870;
    other_1629 = other_1628;
    let _e4: Motor = self_1871;
    let _e7: AntiScalar = other_1629;
    return AntiScalar((_e4.g0_.x * _e7.g0_));
}

fn motor_anti_scalar_inner_product(self_1872: Motor, other_1630: AntiScalar) -> MotorDual {
    var self_1873: Motor;
    var other_1631: AntiScalar;

    self_1873 = self_1872;
    other_1631 = other_1630;
    let _e4: Motor = self_1873;
    let _e6: AntiScalar = other_1631;
    return MotorDual(((_e4.g0_ * vec4<f32>(_e6.g0_)) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))));
}

fn motor_anti_scalar_geometric_anti_product(self_1874: Motor, other_1632: AntiScalar) -> Motor {
    var self_1875: Motor;
    var other_1633: AntiScalar;

    self_1875 = self_1874;
    other_1633 = other_1632;
    let _e4: Motor = self_1875;
    let _e6: AntiScalar = other_1633;
    return Motor((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn motor_anti_scalar_inner_anti_product(self_1876: Motor, other_1634: AntiScalar) -> Motor {
    var self_1877: Motor;
    var other_1635: AntiScalar;

    self_1877 = self_1876;
    other_1635 = other_1634;
    let _e4: Motor = self_1877;
    let _e6: AntiScalar = other_1635;
    return Motor((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn motor_anti_scalar_left_contraction(self_1878: Motor, other_1636: AntiScalar) -> MotorDual {
    var self_1879: Motor;
    var other_1637: AntiScalar;

    self_1879 = self_1878;
    other_1637 = other_1636;
    let _e4: Motor = self_1879;
    let _e6: AntiScalar = other_1637;
    return MotorDual(((_e4.g0_ * vec4<f32>(_e6.g0_)) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))));
}

fn motor_anti_scalar_right_anti_contraction(self_1880: Motor, other_1638: AntiScalar) -> Motor {
    var self_1881: Motor;
    var other_1639: AntiScalar;

    self_1881 = self_1880;
    other_1639 = other_1638;
    let _e4: Motor = self_1881;
    let _e6: AntiScalar = other_1639;
    return Motor((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn motor_multi_vector_add(self_1882: Motor, other_1640: MultiVector) -> MultiVector {
    var self_1883: Motor;
    var other_1641: MultiVector;

    self_1883 = self_1882;
    other_1641 = other_1640;
    let _e4: Motor = self_1883;
    let _e13: MultiVector = other_1641;
    let _e16: Motor = self_1883;
    let _e25: MultiVector = other_1641;
    return MultiVector(((_e4.g0_.xyxx * vec4<f32>(1.0, 1.0, 0.0, 0.0)) + _e13.g0_), ((_e16.g0_.xxzw * vec4<f32>(0.0, 0.0, 1.0, 1.0)) + _e25.g1_));
}

fn motor_multi_vector_sub(self_1884: Motor, other_1642: MultiVector) -> MultiVector {
    var self_1885: Motor;
    var other_1643: MultiVector;

    self_1885 = self_1884;
    other_1643 = other_1642;
    let _e4: Motor = self_1885;
    let _e13: MultiVector = other_1643;
    let _e16: Motor = self_1885;
    let _e25: MultiVector = other_1643;
    return MultiVector(((_e4.g0_.xyxx * vec4<f32>(1.0, 1.0, 0.0, 0.0)) - _e13.g0_), ((_e16.g0_.xxzw * vec4<f32>(0.0, 0.0, 1.0, 1.0)) - _e25.g1_));
}

fn motor_multi_vector_geometric_product(self_1886: Motor, other_1644: MultiVector) -> MultiVector {
    var self_1887: Motor;
    var other_1645: MultiVector;

    self_1887 = self_1886;
    other_1645 = other_1644;
    let _e4: Motor = self_1887;
    let _e8: MultiVector = other_1645;
    let _e11: Motor = self_1887;
    let _e15: MultiVector = other_1645;
    let _e28: Motor = self_1887;
    let _e32: MultiVector = other_1645;
    let _e46: Motor = self_1887;
    let _e50: MultiVector = other_1645;
    let _e64: Motor = self_1887;
    let _e68: MultiVector = other_1645;
    let _e71: Motor = self_1887;
    let _e75: MultiVector = other_1645;
    let _e88: Motor = self_1887;
    let _e92: MultiVector = other_1645;
    let _e104: Motor = self_1887;
    let _e108: MultiVector = other_1645;
    return MultiVector(((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.y) * _e15.g0_.yxwz) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e28.g0_.z) * _e32.g1_.zwxy) * vec4<f32>(-(1.0), 1.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e46.g0_.w) * _e50.g1_.wzyx) * vec4<f32>(-(1.0), -(1.0), -(1.0), 1.0))), ((((vec4<f32>(_e64.g0_.x) * _e68.g1_) + ((vec4<f32>(_e71.g0_.y) * _e75.g1_.yxwz) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))) + ((vec4<f32>(_e88.g0_.z) * _e92.g0_.zwxy) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e104.g0_.w) * _e108.g0_.wzyx) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0))));
}

fn motor_multi_vector_outer_product(self_1888: Motor, other_1646: MultiVector) -> MultiVector {
    var self_1889: Motor;
    var other_1647: MultiVector;

    self_1889 = self_1888;
    other_1647 = other_1646;
    let _e4: Motor = self_1889;
    let _e8: MultiVector = other_1647;
    let _e11: Motor = self_1889;
    let _e14: MultiVector = other_1647;
    let _e26: Motor = self_1889;
    let _e30: MultiVector = other_1647;
    let _e33: Motor = self_1889;
    let _e37: MultiVector = other_1647;
    let _e48: Motor = self_1889;
    let _e52: MultiVector = other_1647;
    let _e63: Motor = self_1889;
    let _e66: MultiVector = other_1647;
    return MultiVector(((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((_e11.g0_.xyxx * vec4<f32>(_e14.g0_.x)) * vec4<f32>(0.0, 1.0, 0.0, 0.0))), ((((vec4<f32>(_e26.g0_.x) * _e30.g1_) + ((vec4<f32>(_e33.g0_.z) * _e37.g0_.wwxw) * vec4<f32>(0.0, 1.0, 1.0, 0.0))) + ((vec4<f32>(_e48.g0_.w) * _e52.g0_.zzzx) * vec4<f32>(0.0, 1.0, 0.0, 1.0))) + ((_e63.g0_.xyxx * vec4<f32>(_e66.g1_.x)) * vec4<f32>(0.0, 1.0, 0.0, 0.0))));
}

fn motor_multi_vector_inner_product(self_1890: Motor, other_1648: MultiVector) -> MultiVector {
    var self_1891: Motor;
    var other_1649: MultiVector;

    self_1891 = self_1890;
    other_1649 = other_1648;
    let _e4: Motor = self_1891;
    let _e8: MultiVector = other_1649;
    let _e11: Motor = self_1891;
    let _e15: MultiVector = other_1649;
    let _e28: Motor = self_1891;
    let _e32: MultiVector = other_1649;
    let _e45: Motor = self_1891;
    let _e48: MultiVector = other_1649;
    let _e62: Motor = self_1891;
    let _e66: MultiVector = other_1649;
    let _e69: Motor = self_1891;
    let _e73: MultiVector = other_1649;
    let _e84: Motor = self_1891;
    let _e88: MultiVector = other_1649;
    let _e100: Motor = self_1891;
    let _e103: MultiVector = other_1649;
    return MultiVector(((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.y) * _e15.g0_.yxwz) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e28.g0_.w) * _e32.g1_.wwyx) * vec4<f32>(-(1.0), 0.0, -(1.0), 1.0))) + ((_e45.g0_.zxzz * _e48.g1_.zxxy) * vec4<f32>(-(1.0), 0.0, -(1.0), -(1.0)))), ((((vec4<f32>(_e62.g0_.x) * _e66.g1_) + ((vec4<f32>(_e69.g0_.z) * _e73.g0_.zzxz) * vec4<f32>(1.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e84.g0_.w) * _e88.g0_.wwwx) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))) + ((_e100.g0_.yxxx * _e103.g1_.yxxx) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))));
}

fn motor_multi_vector_geometric_anti_product(self_1892: Motor, other_1650: MultiVector) -> MultiVector {
    var self_1893: Motor;
    var other_1651: MultiVector;

    self_1893 = self_1892;
    other_1651 = other_1650;
    let _e4: Motor = self_1893;
    let _e8: MultiVector = other_1651;
    let _e19: Motor = self_1893;
    let _e23: MultiVector = other_1651;
    let _e34: Motor = self_1893;
    let _e38: MultiVector = other_1651;
    let _e43: Motor = self_1893;
    let _e47: MultiVector = other_1651;
    let _e60: Motor = self_1893;
    let _e64: MultiVector = other_1651;
    let _e77: Motor = self_1893;
    let _e81: MultiVector = other_1651;
    let _e92: Motor = self_1893;
    let _e96: MultiVector = other_1651;
    let _e109: Motor = self_1893;
    let _e113: MultiVector = other_1651;
    return MultiVector((((((vec4<f32>(_e4.g0_.x) * _e8.g1_.yxwz) * vec4<f32>(1.0, -(1.0), 1.0, 1.0)) + ((vec4<f32>(_e19.g0_.y) * _e23.g1_) * vec4<f32>(1.0, 1.0, -(1.0), 1.0))) + (vec4<f32>(_e34.g0_.z) * _e38.g0_.wzyx)) + ((vec4<f32>(_e43.g0_.w) * _e47.g0_.zwxy) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))), (((((vec4<f32>(_e60.g0_.x) * _e64.g0_.yxwz) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))) + ((vec4<f32>(_e77.g0_.y) * _e81.g0_) * vec4<f32>(1.0, 1.0, -(1.0), 1.0))) + ((vec4<f32>(_e92.g0_.z) * _e96.g1_.wzyx) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + (vec4<f32>(_e109.g0_.w) * _e113.g1_.zwxy)));
}

fn motor_multi_vector_inner_anti_product(self_1894: Motor, other_1652: MultiVector) -> MultiVector {
    var self_1895: Motor;
    var other_1653: MultiVector;

    self_1895 = self_1894;
    other_1653 = other_1652;
    let _e4: Motor = self_1895;
    let _e8: MultiVector = other_1653;
    let _e19: Motor = self_1895;
    let _e23: MultiVector = other_1653;
    let _e34: Motor = self_1895;
    let _e38: MultiVector = other_1653;
    let _e50: Motor = self_1895;
    let _e53: MultiVector = other_1653;
    let _e64: Motor = self_1895;
    let _e68: MultiVector = other_1653;
    let _e81: Motor = self_1895;
    let _e85: MultiVector = other_1653;
    let _e96: Motor = self_1895;
    let _e100: MultiVector = other_1653;
    let _e111: Motor = self_1895;
    let _e114: MultiVector = other_1653;
    return MultiVector((((((vec4<f32>(_e4.g0_.x) * _e8.g1_.yxwz) * vec4<f32>(1.0, -(1.0), 1.0, 1.0)) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.zzzx) * vec4<f32>(0.0, 1.0, 0.0, 1.0))) + ((vec4<f32>(_e34.g0_.w) * _e38.g0_.wwxw) * vec4<f32>(0.0, -(1.0), 1.0, 0.0))) + ((_e50.g0_.xyxx * _e53.g1_.xyxx) * vec4<f32>(0.0, 1.0, 0.0, 0.0))), (((((vec4<f32>(_e64.g0_.x) * _e68.g0_.yxwz) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))) + ((vec4<f32>(_e81.g0_.y) * _e85.g0_) * vec4<f32>(1.0, 1.0, -(1.0), 1.0))) + ((vec4<f32>(_e96.g0_.w) * _e100.g1_.wwxy) * vec4<f32>(0.0, 1.0, 1.0, 1.0))) + ((_e111.g0_.xzzz * _e114.g1_.xzyx) * vec4<f32>(0.0, 1.0, 1.0, -(1.0)))));
}

fn motor_multi_vector_left_contraction(self_1896: Motor, other_1654: MultiVector) -> MultiVector {
    var self_1897: Motor;
    var other_1655: MultiVector;

    self_1897 = self_1896;
    other_1655 = other_1654;
    let _e4: Motor = self_1897;
    let _e8: MultiVector = other_1655;
    let _e11: Motor = self_1897;
    let _e15: MultiVector = other_1655;
    let _e28: Motor = self_1897;
    let _e32: MultiVector = other_1655;
    let _e45: Motor = self_1897;
    let _e48: MultiVector = other_1655;
    let _e60: Motor = self_1897;
    let _e64: MultiVector = other_1655;
    let _e67: Motor = self_1897;
    let _e70: MultiVector = other_1655;
    return MultiVector(((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.z) * _e15.g1_.zzzy) * vec4<f32>(-(1.0), 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e28.g0_.w) * _e32.g1_.wwyw) * vec4<f32>(-(1.0), 0.0, -(1.0), 0.0))) + ((_e45.g0_.yxxx * _e48.g0_.yxxx) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))), ((vec4<f32>(_e60.g0_.x) * _e64.g1_) + ((_e67.g0_.yxxx * _e70.g1_.yxxx) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))));
}

fn motor_multi_vector_right_anti_contraction(self_1898: Motor, other_1656: MultiVector) -> MultiVector {
    var self_1899: Motor;
    var other_1657: MultiVector;

    self_1899 = self_1898;
    other_1657 = other_1656;
    let _e4: Motor = self_1899;
    let _e8: MultiVector = other_1657;
    let _e19: Motor = self_1899;
    let _e22: MultiVector = other_1657;
    let _e33: Motor = self_1899;
    let _e37: MultiVector = other_1657;
    let _e50: Motor = self_1899;
    let _e54: MultiVector = other_1657;
    let _e65: Motor = self_1899;
    let _e69: MultiVector = other_1657;
    let _e80: Motor = self_1899;
    let _e83: MultiVector = other_1657;
    return MultiVector((((vec4<f32>(_e4.g0_.x) * _e8.g1_.yxwz) * vec4<f32>(1.0, -(1.0), 1.0, 1.0)) + ((_e19.g0_.xyxx * _e22.g1_.xyxx) * vec4<f32>(0.0, 1.0, 0.0, 0.0))), (((((vec4<f32>(_e33.g0_.x) * _e37.g0_.yxwz) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))) + ((vec4<f32>(_e50.g0_.z) * _e54.g1_.zzyz) * vec4<f32>(0.0, 1.0, 1.0, 0.0))) + ((vec4<f32>(_e65.g0_.w) * _e69.g1_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, 1.0))) + ((_e80.g0_.xyxx * _e83.g0_.xyxx) * vec4<f32>(0.0, 1.0, 0.0, 0.0))));
}

fn motor_multi_vector_scalar_product(self_1900: Motor, other_1658: MultiVector) -> Scalar {
    var self_1901: Motor;
    var other_1659: MultiVector;

    self_1901 = self_1900;
    other_1659 = other_1658;
    let _e4: Motor = self_1901;
    let _e7: MultiVector = other_1659;
    let _e11: Motor = self_1901;
    let _e14: MultiVector = other_1659;
    let _e19: Motor = self_1901;
    let _e22: MultiVector = other_1659;
    let _e27: Motor = self_1901;
    let _e30: MultiVector = other_1659;
    return Scalar(((((_e4.g0_.x * _e7.g0_.x) - (_e11.g0_.y * _e14.g0_.y)) - (_e19.g0_.z * _e22.g1_.z)) - (_e27.g0_.w * _e30.g1_.w)));
}

fn motor_multi_vector_anti_scalar_product(self_1902: Motor, other_1660: MultiVector) -> AntiScalar {
    var self_1903: Motor;
    var other_1661: MultiVector;

    self_1903 = self_1902;
    other_1661 = other_1660;
    let _e5: Motor = self_1903;
    let _e8: MultiVector = other_1661;
    let _e13: Motor = self_1903;
    let _e16: MultiVector = other_1661;
    let _e21: Motor = self_1903;
    let _e24: MultiVector = other_1661;
    let _e29: Motor = self_1903;
    let _e32: MultiVector = other_1661;
    return AntiScalar(((((0.0 - (_e5.g0_.x * _e8.g0_.x)) + (_e13.g0_.y * _e16.g0_.y)) + (_e21.g0_.z * _e24.g1_.z)) + (_e29.g0_.w * _e32.g1_.w)));
}

fn motor_rotor_into(self_1904: Motor) -> Rotor {
    var self_1905: Motor;

    self_1905 = self_1904;
    let _e2: Motor = self_1905;
    let _e5: Motor = self_1905;
    return Rotor(vec2<f32>(_e2.g0_.x, _e5.g0_.y));
}

fn motor_rotor_add(self_1906: Motor, other_1662: Rotor) -> Motor {
    var self_1907: Motor;
    var other_1663: Rotor;

    self_1907 = self_1906;
    other_1663 = other_1662;
    let _e4: Motor = self_1907;
    let _e6: Rotor = other_1663;
    let _e9: Rotor = other_1663;
    let _e12: Rotor = other_1663;
    let _e15: Rotor = other_1663;
    return Motor((_e4.g0_ + (vec4<f32>(_e6.g0_.x, _e9.g0_.y, _e12.g0_.x, _e15.g0_.x) * vec4<f32>(1.0, 1.0, 0.0, 0.0))));
}

fn motor_rotor_sub(self_1908: Motor, other_1664: Rotor) -> Motor {
    var self_1909: Motor;
    var other_1665: Rotor;

    self_1909 = self_1908;
    other_1665 = other_1664;
    let _e4: Motor = self_1909;
    let _e6: Rotor = other_1665;
    let _e9: Rotor = other_1665;
    let _e12: Rotor = other_1665;
    let _e15: Rotor = other_1665;
    return Motor((_e4.g0_ - (vec4<f32>(_e6.g0_.x, _e9.g0_.y, _e12.g0_.x, _e15.g0_.x) * vec4<f32>(1.0, 1.0, 0.0, 0.0))));
}

fn motor_rotor_geometric_product(self_1910: Motor, other_1666: Rotor) -> Motor {
    var self_1911: Motor;
    var other_1667: Rotor;

    self_1911 = self_1910;
    other_1667 = other_1666;
    let _e4: Motor = self_1911;
    let _e8: Rotor = other_1667;
    let _e11: Rotor = other_1667;
    let _e14: Rotor = other_1667;
    let _e17: Rotor = other_1667;
    let _e29: Motor = self_1911;
    let _e33: Rotor = other_1667;
    let _e36: Rotor = other_1667;
    let _e39: Rotor = other_1667;
    let _e42: Rotor = other_1667;
    let _e54: Motor = self_1911;
    let _e57: Rotor = other_1667;
    let _e60: Rotor = other_1667;
    let _e63: Rotor = other_1667;
    let _e66: Rotor = other_1667;
    return Motor(((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y, _e11.g0_.x, _e14.g0_.y, _e17.g0_.y)) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0)) + ((vec4<f32>(_e29.g0_.w) * vec4<f32>(_e33.g0_.y, _e36.g0_.y, _e39.g0_.y, _e42.g0_.x)) * vec4<f32>(0.0, 0.0, 1.0, 1.0))) + ((_e54.g0_.xxzz * vec4<f32>(_e57.g0_.x, _e60.g0_.y, _e63.g0_.x, _e66.g0_.y)) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))));
}

fn motor_rotor_outer_product(self_1912: Motor, other_1668: Rotor) -> Motor {
    var self_1913: Motor;
    var other_1669: Rotor;

    self_1913 = self_1912;
    other_1669 = other_1668;
    let _e4: Motor = self_1913;
    let _e8: Rotor = other_1669;
    let _e19: Motor = self_1913;
    let _e22: Rotor = other_1669;
    let _e25: Rotor = other_1669;
    let _e28: Rotor = other_1669;
    let _e31: Rotor = other_1669;
    return Motor((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.x)) * vec4<f32>(0.0, 1.0, 0.0, 0.0)) + (_e19.g0_.xxzw * vec4<f32>(_e22.g0_.x, _e25.g0_.y, _e28.g0_.x, _e31.g0_.x))));
}

fn motor_rotor_inner_product(self_1914: Motor, other_1670: Rotor) -> Motor {
    var self_1915: Motor;
    var other_1671: Rotor;

    self_1915 = self_1914;
    other_1671 = other_1670;
    let _e4: Motor = self_1915;
    let _e8: Rotor = other_1671;
    let _e11: Rotor = other_1671;
    let _e14: Rotor = other_1671;
    let _e17: Rotor = other_1671;
    let _e29: Motor = self_1915;
    let _e32: Rotor = other_1671;
    let _e35: Rotor = other_1671;
    let _e38: Rotor = other_1671;
    let _e41: Rotor = other_1671;
    return Motor((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y, _e11.g0_.x, _e14.g0_.y, _e17.g0_.y)) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0)) + (_e29.g0_.xxzw * vec4<f32>(_e32.g0_.x, _e35.g0_.y, _e38.g0_.x, _e41.g0_.x))));
}

fn motor_rotor_geometric_anti_product(self_1916: Motor, other_1672: Rotor) -> MotorDual {
    var self_1917: Motor;
    var other_1673: Rotor;

    self_1917 = self_1916;
    other_1673 = other_1672;
    let _e4: Motor = self_1917;
    let _e8: Rotor = other_1673;
    let _e11: Rotor = other_1673;
    let _e14: Rotor = other_1673;
    let _e17: Rotor = other_1673;
    let _e28: Motor = self_1917;
    let _e32: Rotor = other_1673;
    let _e35: Rotor = other_1673;
    let _e38: Rotor = other_1673;
    let _e41: Rotor = other_1673;
    let _e54: Motor = self_1917;
    let _e57: Rotor = other_1673;
    let _e60: Rotor = other_1673;
    let _e63: Rotor = other_1673;
    let _e66: Rotor = other_1673;
    return MotorDual(((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y, _e11.g0_.x, _e14.g0_.y, _e17.g0_.y)) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) + ((vec4<f32>(_e28.g0_.w) * vec4<f32>(_e32.g0_.y, _e35.g0_.y, _e38.g0_.y, _e41.g0_.x)) * vec4<f32>(0.0, 0.0, -(1.0), 1.0))) + ((_e54.g0_.xxzz * vec4<f32>(_e57.g0_.x, _e60.g0_.y, _e63.g0_.x, _e66.g0_.y)) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0))));
}

fn motor_rotor_inner_anti_product(self_1918: Motor, other_1674: Rotor) -> MotorDual {
    var self_1919: Motor;
    var other_1675: Rotor;

    self_1919 = self_1918;
    other_1675 = other_1674;
    let _e4: Motor = self_1919;
    let _e8: Rotor = other_1675;
    let _e11: Rotor = other_1675;
    let _e14: Rotor = other_1675;
    let _e17: Rotor = other_1675;
    let _e28: Motor = self_1919;
    let _e31: Rotor = other_1675;
    let _e34: Rotor = other_1675;
    let _e37: Rotor = other_1675;
    let _e40: Rotor = other_1675;
    return MotorDual((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y, _e11.g0_.x, _e14.g0_.y, _e17.g0_.y)) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) + ((_e28.g0_.xxzw * vec4<f32>(_e31.g0_.x, _e34.g0_.y, _e37.g0_.x, _e40.g0_.x)) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0))));
}

fn motor_rotor_left_contraction(self_1920: Motor, other_1676: Rotor) -> Rotor {
    var self_1921: Motor;
    var other_1677: Rotor;

    self_1921 = self_1920;
    other_1677 = other_1676;
    let _e4: Motor = self_1921;
    let _e8: Rotor = other_1677;
    let _e11: Motor = self_1921;
    let _e14: Motor = self_1921;
    let _e18: Rotor = other_1677;
    return Rotor(((vec2<f32>(_e4.g0_.x) * _e8.g0_) + ((vec2<f32>(_e11.g0_.y, _e14.g0_.x) * _e18.g0_.yx) * vec2<f32>(-(1.0), 0.0))));
}

fn motor_rotor_right_contraction(self_1922: Motor, other_1678: Rotor) -> Motor {
    var self_1923: Motor;
    var other_1679: Rotor;

    self_1923 = self_1922;
    other_1679 = other_1678;
    let _e4: Motor = self_1923;
    let _e8: Rotor = other_1679;
    let _e11: Rotor = other_1679;
    let _e14: Rotor = other_1679;
    let _e17: Rotor = other_1679;
    let _e29: Motor = self_1923;
    let _e32: Rotor = other_1679;
    return Motor((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y, _e11.g0_.x, _e14.g0_.y, _e17.g0_.y)) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0)) + ((_e29.g0_.xxzw * vec4<f32>(_e32.g0_.x)) * vec4<f32>(1.0, 0.0, 1.0, 1.0))));
}

fn motor_rotor_left_anti_contraction(self_1924: Motor, other_1680: Rotor) -> MotorDual {
    var self_1925: Motor;
    var other_1681: Rotor;

    self_1925 = self_1924;
    other_1681 = other_1680;
    let _e4: Motor = self_1925;
    let _e8: Rotor = other_1681;
    let _e11: Rotor = other_1681;
    let _e14: Rotor = other_1681;
    let _e17: Rotor = other_1681;
    let _e28: Motor = self_1925;
    let _e31: Rotor = other_1681;
    return MotorDual((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y, _e11.g0_.x, _e14.g0_.y, _e17.g0_.y)) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) + ((_e28.g0_.xxzw * vec4<f32>(_e31.g0_.x)) * vec4<f32>(-(1.0), 0.0, 1.0, 1.0))));
}

fn motor_rotor_scalar_product(self_1926: Motor, other_1682: Rotor) -> Scalar {
    var self_1927: Motor;
    var other_1683: Rotor;

    self_1927 = self_1926;
    other_1683 = other_1682;
    let _e4: Motor = self_1927;
    let _e7: Rotor = other_1683;
    let _e11: Motor = self_1927;
    let _e14: Rotor = other_1683;
    return Scalar(((_e4.g0_.x * _e7.g0_.x) - (_e11.g0_.y * _e14.g0_.y)));
}

fn motor_rotor_anti_scalar_product(self_1928: Motor, other_1684: Rotor) -> AntiScalar {
    var self_1929: Motor;
    var other_1685: Rotor;

    self_1929 = self_1928;
    other_1685 = other_1684;
    let _e5: Motor = self_1929;
    let _e8: Rotor = other_1685;
    let _e13: Motor = self_1929;
    let _e16: Rotor = other_1685;
    return AntiScalar(((0.0 - (_e5.g0_.x * _e8.g0_.x)) + (_e13.g0_.y * _e16.g0_.y)));
}

fn motor_point_into(self_1930: Motor) -> Point {
    var self_1931: Motor;

    self_1931 = self_1930;
    let _e2: Motor = self_1931;
    let _e5: Motor = self_1931;
    let _e8: Motor = self_1931;
    return Point(vec3<f32>(_e2.g0_.y, _e5.g0_.z, _e8.g0_.w));
}

fn motor_point_add(self_1932: Motor, other_1686: Point) -> Motor {
    var self_1933: Motor;
    var other_1687: Point;

    self_1933 = self_1932;
    other_1687 = other_1686;
    let _e4: Motor = self_1933;
    let _e6: Point = other_1687;
    let _e9: Point = other_1687;
    let _e12: Point = other_1687;
    let _e15: Point = other_1687;
    return Motor((_e4.g0_ + (vec4<f32>(_e6.g0_.x, _e9.g0_.x, _e12.g0_.y, _e15.g0_.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn motor_point_sub(self_1934: Motor, other_1688: Point) -> Motor {
    var self_1935: Motor;
    var other_1689: Point;

    self_1935 = self_1934;
    other_1689 = other_1688;
    let _e4: Motor = self_1935;
    let _e6: Point = other_1689;
    let _e9: Point = other_1689;
    let _e12: Point = other_1689;
    let _e15: Point = other_1689;
    return Motor((_e4.g0_ - (vec4<f32>(_e6.g0_.x, _e9.g0_.x, _e12.g0_.y, _e15.g0_.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn motor_point_geometric_product(self_1936: Motor, other_1690: Point) -> Motor {
    var self_1937: Motor;
    var other_1691: Point;

    self_1937 = self_1936;
    other_1691 = other_1690;
    let _e4: Motor = self_1937;
    let _e8: Point = other_1691;
    let _e11: Point = other_1691;
    let _e14: Point = other_1691;
    let _e17: Point = other_1691;
    let _e30: Motor = self_1937;
    let _e34: Point = other_1691;
    let _e37: Point = other_1691;
    let _e40: Point = other_1691;
    let _e43: Point = other_1691;
    let _e57: Motor = self_1937;
    let _e61: Point = other_1691;
    let _e64: Point = other_1691;
    let _e67: Point = other_1691;
    let _e70: Point = other_1691;
    let _e84: Motor = self_1937;
    let _e88: Point = other_1691;
    let _e91: Point = other_1691;
    let _e94: Point = other_1691;
    let _e97: Point = other_1691;
    return Motor((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.x, _e11.g0_.x, _e14.g0_.z, _e17.g0_.y)) * vec4<f32>(-(1.0), 0.0, -(1.0), 1.0)) + ((vec4<f32>(_e30.g0_.z) * vec4<f32>(_e34.g0_.y, _e37.g0_.z, _e40.g0_.y, _e43.g0_.x)) * vec4<f32>(-(1.0), 1.0, 0.0, -(1.0)))) + ((vec4<f32>(_e57.g0_.w) * vec4<f32>(_e61.g0_.z, _e64.g0_.y, _e67.g0_.x, _e70.g0_.z)) * vec4<f32>(-(1.0), -(1.0), 1.0, 0.0))) + ((vec4<f32>(_e84.g0_.x) * vec4<f32>(_e88.g0_.x, _e91.g0_.x, _e94.g0_.y, _e97.g0_.z)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn motor_point_regressive_product(self_1938: Motor, other_1692: Point) -> Plane {
    var self_1939: Motor;
    var other_1693: Point;

    self_1939 = self_1938;
    other_1693 = other_1692;
    let _e4: Motor = self_1939;
    let _e8: Point = other_1693;
    let _e18: Motor = self_1939;
    let _e22: Point = other_1693;
    let _e33: Motor = self_1939;
    let _e36: Motor = self_1939;
    let _e39: Motor = self_1939;
    let _e43: Point = other_1693;
    return Plane(((((vec3<f32>(_e4.g0_.z) * _e8.g0_.zzx) * vec3<f32>(-(1.0), 0.0, 1.0)) + ((vec3<f32>(_e18.g0_.w) * _e22.g0_.yxy) * vec3<f32>(1.0, -(1.0), 0.0))) + ((vec3<f32>(_e33.g0_.x, _e36.g0_.y, _e39.g0_.y) * _e43.g0_.xzy) * vec3<f32>(0.0, 1.0, -(1.0)))));
}

fn motor_point_outer_product(self_1940: Motor, other_1694: Point) -> Point {
    var self_1941: Motor;
    var other_1695: Point;

    self_1941 = self_1940;
    other_1695 = other_1694;
    let _e4: Motor = self_1941;
    let _e8: Point = other_1695;
    return Point((vec3<f32>(_e4.g0_.x) * _e8.g0_));
}

fn motor_point_inner_product(self_1942: Motor, other_1696: Point) -> Motor {
    var self_1943: Motor;
    var other_1697: Point;

    self_1943 = self_1942;
    other_1697 = other_1696;
    let _e4: Motor = self_1943;
    let _e8: Point = other_1697;
    let _e20: Motor = self_1943;
    let _e24: Point = other_1697;
    let _e37: Motor = self_1943;
    let _e40: Point = other_1697;
    let _e43: Point = other_1697;
    let _e46: Point = other_1697;
    let _e49: Point = other_1697;
    return Motor(((((vec4<f32>(_e4.g0_.z) * vec4<f32>(_e8.g0_.y)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0)) + ((vec4<f32>(_e20.g0_.w) * vec4<f32>(_e24.g0_.z)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((_e37.g0_.yxxx * vec4<f32>(_e40.g0_.x, _e43.g0_.x, _e46.g0_.y, _e49.g0_.z)) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0))));
}

fn motor_point_geometric_anti_product(self_1944: Motor, other_1698: Point) -> MotorDual {
    var self_1945: Motor;
    var other_1699: Point;

    self_1945 = self_1944;
    other_1699 = other_1698;
    let _e4: Motor = self_1945;
    let _e8: Point = other_1699;
    let _e11: Point = other_1699;
    let _e14: Point = other_1699;
    let _e17: Point = other_1699;
    let _e29: Motor = self_1945;
    let _e33: Point = other_1699;
    let _e36: Point = other_1699;
    let _e39: Point = other_1699;
    let _e42: Point = other_1699;
    let _e55: Motor = self_1945;
    let _e59: Point = other_1699;
    let _e62: Point = other_1699;
    let _e65: Point = other_1699;
    let _e68: Point = other_1699;
    let _e81: Motor = self_1945;
    let _e85: Point = other_1699;
    let _e88: Point = other_1699;
    let _e91: Point = other_1699;
    let _e94: Point = other_1699;
    return MotorDual((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.x, _e11.g0_.x, _e14.g0_.z, _e17.g0_.y)) * vec4<f32>(1.0, 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e29.g0_.z) * vec4<f32>(_e33.g0_.y, _e36.g0_.z, _e39.g0_.y, _e42.g0_.x)) * vec4<f32>(1.0, -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e55.g0_.w) * vec4<f32>(_e59.g0_.z, _e62.g0_.y, _e65.g0_.x, _e68.g0_.z)) * vec4<f32>(1.0, 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e81.g0_.x) * vec4<f32>(_e85.g0_.x, _e88.g0_.x, _e91.g0_.y, _e94.g0_.z)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn motor_point_inner_anti_product(self_1946: Motor, other_1700: Point) -> MotorDual {
    var self_1947: Motor;
    var other_1701: Point;

    self_1947 = self_1946;
    other_1701 = other_1700;
    let _e4: Motor = self_1947;
    let _e8: Point = other_1701;
    let _e19: Motor = self_1947;
    let _e23: Point = other_1701;
    let _e35: Motor = self_1947;
    let _e38: Point = other_1701;
    let _e41: Point = other_1701;
    let _e44: Point = other_1701;
    let _e47: Point = other_1701;
    return MotorDual(((((vec4<f32>(_e4.g0_.z) * vec4<f32>(_e8.g0_.y)) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) + ((vec4<f32>(_e19.g0_.w) * vec4<f32>(_e23.g0_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + (_e35.g0_.yxxx * vec4<f32>(_e38.g0_.x, _e41.g0_.x, _e44.g0_.y, _e47.g0_.z))));
}

fn motor_point_left_contraction(self_1948: Motor, other_1702: Point) -> Motor {
    var self_1949: Motor;
    var other_1703: Point;

    self_1949 = self_1948;
    other_1703 = other_1702;
    let _e4: Motor = self_1949;
    let _e8: Point = other_1703;
    let _e20: Motor = self_1949;
    let _e24: Point = other_1703;
    let _e37: Motor = self_1949;
    let _e40: Point = other_1703;
    let _e43: Point = other_1703;
    let _e46: Point = other_1703;
    let _e49: Point = other_1703;
    return Motor(((((vec4<f32>(_e4.g0_.z) * vec4<f32>(_e8.g0_.y)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0)) + ((vec4<f32>(_e20.g0_.w) * vec4<f32>(_e24.g0_.z)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((_e37.g0_.yxxx * vec4<f32>(_e40.g0_.x, _e43.g0_.x, _e46.g0_.y, _e49.g0_.z)) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0))));
}

fn motor_point_right_contraction(self_1950: Motor, other_1704: Point) -> Scalar {
    var self_1951: Motor;
    var other_1705: Point;

    self_1951 = self_1950;
    other_1705 = other_1704;
    let _e5: Motor = self_1951;
    let _e8: Point = other_1705;
    let _e13: Motor = self_1951;
    let _e16: Point = other_1705;
    let _e21: Motor = self_1951;
    let _e24: Point = other_1705;
    return Scalar((((0.0 - (_e5.g0_.y * _e8.g0_.x)) - (_e13.g0_.z * _e16.g0_.y)) - (_e21.g0_.w * _e24.g0_.z)));
}

fn motor_point_left_anti_contraction(self_1952: Motor, other_1706: Point) -> AntiScalar {
    var self_1953: Motor;
    var other_1707: Point;

    self_1953 = self_1952;
    other_1707 = other_1706;
    let _e4: Motor = self_1953;
    let _e7: Point = other_1707;
    let _e11: Motor = self_1953;
    let _e14: Point = other_1707;
    let _e19: Motor = self_1953;
    let _e22: Point = other_1707;
    return AntiScalar((((_e4.g0_.y * _e7.g0_.x) + (_e11.g0_.z * _e14.g0_.y)) + (_e19.g0_.w * _e22.g0_.z)));
}

fn motor_point_right_anti_contraction(self_1954: Motor, other_1708: Point) -> MotorDual {
    var self_1955: Motor;
    var other_1709: Point;

    self_1955 = self_1954;
    other_1709 = other_1708;
    let _e4: Motor = self_1955;
    let _e8: Point = other_1709;
    let _e19: Motor = self_1955;
    let _e23: Point = other_1709;
    let _e35: Motor = self_1955;
    let _e38: Point = other_1709;
    let _e41: Point = other_1709;
    let _e44: Point = other_1709;
    let _e47: Point = other_1709;
    return MotorDual(((((vec4<f32>(_e4.g0_.z) * vec4<f32>(_e8.g0_.y)) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) + ((vec4<f32>(_e19.g0_.w) * vec4<f32>(_e23.g0_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + (_e35.g0_.yxxx * vec4<f32>(_e38.g0_.x, _e41.g0_.x, _e44.g0_.y, _e47.g0_.z))));
}

fn motor_point_scalar_product(self_1956: Motor, other_1710: Point) -> Scalar {
    var self_1957: Motor;
    var other_1711: Point;

    self_1957 = self_1956;
    other_1711 = other_1710;
    let _e5: Motor = self_1957;
    let _e8: Point = other_1711;
    let _e13: Motor = self_1957;
    let _e16: Point = other_1711;
    let _e21: Motor = self_1957;
    let _e24: Point = other_1711;
    return Scalar((((0.0 - (_e5.g0_.y * _e8.g0_.x)) - (_e13.g0_.z * _e16.g0_.y)) - (_e21.g0_.w * _e24.g0_.z)));
}

fn motor_point_anti_scalar_product(self_1958: Motor, other_1712: Point) -> AntiScalar {
    var self_1959: Motor;
    var other_1713: Point;

    self_1959 = self_1958;
    other_1713 = other_1712;
    let _e4: Motor = self_1959;
    let _e7: Point = other_1713;
    let _e11: Motor = self_1959;
    let _e14: Point = other_1713;
    let _e19: Motor = self_1959;
    let _e22: Point = other_1713;
    return AntiScalar((((_e4.g0_.y * _e7.g0_.x) + (_e11.g0_.z * _e14.g0_.y)) + (_e19.g0_.w * _e22.g0_.z)));
}

fn motor_ideal_point_into(self_1960: Motor) -> IdealPoint {
    var self_1961: Motor;

    self_1961 = self_1960;
    let _e2: Motor = self_1961;
    let _e5: Motor = self_1961;
    return IdealPoint(vec2<f32>(_e2.g0_.z, _e5.g0_.w));
}

fn motor_ideal_point_add(self_1962: Motor, other_1714: IdealPoint) -> Motor {
    var self_1963: Motor;
    var other_1715: IdealPoint;

    self_1963 = self_1962;
    other_1715 = other_1714;
    let _e4: Motor = self_1963;
    let _e6: IdealPoint = other_1715;
    let _e9: IdealPoint = other_1715;
    let _e12: IdealPoint = other_1715;
    let _e15: IdealPoint = other_1715;
    return Motor((_e4.g0_ + (vec4<f32>(_e6.g0_.x, _e9.g0_.x, _e12.g0_.x, _e15.g0_.y) * vec4<f32>(0.0, 0.0, 1.0, 1.0))));
}

fn motor_ideal_point_sub(self_1964: Motor, other_1716: IdealPoint) -> Motor {
    var self_1965: Motor;
    var other_1717: IdealPoint;

    self_1965 = self_1964;
    other_1717 = other_1716;
    let _e4: Motor = self_1965;
    let _e6: IdealPoint = other_1717;
    let _e9: IdealPoint = other_1717;
    let _e12: IdealPoint = other_1717;
    let _e15: IdealPoint = other_1717;
    return Motor((_e4.g0_ - (vec4<f32>(_e6.g0_.x, _e9.g0_.x, _e12.g0_.x, _e15.g0_.y) * vec4<f32>(0.0, 0.0, 1.0, 1.0))));
}

fn motor_ideal_point_geometric_product(self_1966: Motor, other_1718: IdealPoint) -> Motor {
    var self_1967: Motor;
    var other_1719: IdealPoint;

    self_1967 = self_1966;
    other_1719 = other_1718;
    let _e4: Motor = self_1967;
    let _e8: IdealPoint = other_1719;
    let _e11: IdealPoint = other_1719;
    let _e14: IdealPoint = other_1719;
    let _e17: IdealPoint = other_1719;
    let _e29: Motor = self_1967;
    let _e33: IdealPoint = other_1719;
    let _e36: IdealPoint = other_1719;
    let _e39: IdealPoint = other_1719;
    let _e42: IdealPoint = other_1719;
    let _e56: Motor = self_1967;
    let _e59: IdealPoint = other_1719;
    let _e62: IdealPoint = other_1719;
    let _e65: IdealPoint = other_1719;
    let _e68: IdealPoint = other_1719;
    return Motor(((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y, _e11.g0_.y, _e14.g0_.y, _e17.g0_.x)) * vec4<f32>(0.0, 0.0, -(1.0), 1.0)) + ((vec4<f32>(_e29.g0_.w) * vec4<f32>(_e33.g0_.y, _e36.g0_.x, _e39.g0_.y, _e42.g0_.y)) * vec4<f32>(-(1.0), -(1.0), 0.0, 0.0))) + ((_e56.g0_.zzxx * vec4<f32>(_e59.g0_.x, _e62.g0_.y, _e65.g0_.x, _e68.g0_.y)) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0))));
}

fn motor_ideal_point_regressive_product(self_1968: Motor, other_1720: IdealPoint) -> Plane {
    var self_1969: Motor;
    var other_1721: IdealPoint;

    self_1969 = self_1968;
    other_1721 = other_1720;
    let _e4: Motor = self_1969;
    let _e8: IdealPoint = other_1721;
    let _e18: Motor = self_1969;
    let _e21: Motor = self_1969;
    let _e24: Motor = self_1969;
    let _e28: IdealPoint = other_1721;
    let _e31: IdealPoint = other_1721;
    let _e34: IdealPoint = other_1721;
    return Plane((((vec3<f32>(_e4.g0_.w) * vec3<f32>(_e8.g0_.x)) * vec3<f32>(1.0, 0.0, 0.0)) + ((vec3<f32>(_e18.g0_.z, _e21.g0_.y, _e24.g0_.y) * vec3<f32>(_e28.g0_.y, _e31.g0_.y, _e34.g0_.x)) * vec3<f32>(-(1.0), 1.0, -(1.0)))));
}

fn motor_ideal_point_outer_product(self_1970: Motor, other_1722: IdealPoint) -> IdealPoint {
    var self_1971: Motor;
    var other_1723: IdealPoint;

    self_1971 = self_1970;
    other_1723 = other_1722;
    let _e4: Motor = self_1971;
    let _e8: IdealPoint = other_1723;
    return IdealPoint((vec2<f32>(_e4.g0_.x) * _e8.g0_));
}

fn motor_ideal_point_inner_product(self_1972: Motor, other_1724: IdealPoint) -> Translator {
    var self_1973: Motor;
    var other_1725: IdealPoint;

    self_1973 = self_1972;
    other_1725 = other_1724;
    let _e4: Motor = self_1973;
    let _e8: IdealPoint = other_1725;
    let _e19: Motor = self_1973;
    let _e22: Motor = self_1973;
    let _e25: Motor = self_1973;
    let _e29: IdealPoint = other_1725;
    let _e32: IdealPoint = other_1725;
    let _e35: IdealPoint = other_1725;
    return Translator((((vec3<f32>(_e4.g0_.w) * vec3<f32>(_e8.g0_.y)) * vec3<f32>(-(1.0), 0.0, 0.0)) + ((vec3<f32>(_e19.g0_.z, _e22.g0_.x, _e25.g0_.x) * vec3<f32>(_e29.g0_.x, _e32.g0_.x, _e35.g0_.y)) * vec3<f32>(-(1.0), 1.0, 1.0))));
}

fn motor_ideal_point_geometric_anti_product(self_1974: Motor, other_1726: IdealPoint) -> MotorDual {
    var self_1975: Motor;
    var other_1727: IdealPoint;

    self_1975 = self_1974;
    other_1727 = other_1726;
    let _e4: Motor = self_1975;
    let _e8: IdealPoint = other_1727;
    let _e11: IdealPoint = other_1727;
    let _e14: IdealPoint = other_1727;
    let _e17: IdealPoint = other_1727;
    let _e29: Motor = self_1975;
    let _e33: IdealPoint = other_1727;
    let _e36: IdealPoint = other_1727;
    let _e39: IdealPoint = other_1727;
    let _e42: IdealPoint = other_1727;
    let _e54: Motor = self_1975;
    let _e57: IdealPoint = other_1727;
    let _e60: IdealPoint = other_1727;
    let _e63: IdealPoint = other_1727;
    let _e66: IdealPoint = other_1727;
    return MotorDual(((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y, _e11.g0_.y, _e14.g0_.y, _e17.g0_.x)) * vec4<f32>(0.0, 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e29.g0_.w) * vec4<f32>(_e33.g0_.y, _e36.g0_.x, _e39.g0_.y, _e42.g0_.y)) * vec4<f32>(1.0, 1.0, 0.0, 0.0))) + ((_e54.g0_.zzxx * vec4<f32>(_e57.g0_.x, _e60.g0_.y, _e63.g0_.x, _e66.g0_.y)) * vec4<f32>(1.0, -(1.0), 1.0, 1.0))));
}

fn motor_ideal_point_left_contraction(self_1976: Motor, other_1728: IdealPoint) -> Translator {
    var self_1977: Motor;
    var other_1729: IdealPoint;

    self_1977 = self_1976;
    other_1729 = other_1728;
    let _e4: Motor = self_1977;
    let _e8: IdealPoint = other_1729;
    let _e19: Motor = self_1977;
    let _e22: Motor = self_1977;
    let _e25: Motor = self_1977;
    let _e29: IdealPoint = other_1729;
    let _e32: IdealPoint = other_1729;
    let _e35: IdealPoint = other_1729;
    return Translator((((vec3<f32>(_e4.g0_.w) * vec3<f32>(_e8.g0_.y)) * vec3<f32>(-(1.0), 0.0, 0.0)) + ((vec3<f32>(_e19.g0_.z, _e22.g0_.x, _e25.g0_.x) * vec3<f32>(_e29.g0_.x, _e32.g0_.x, _e35.g0_.y)) * vec3<f32>(-(1.0), 1.0, 1.0))));
}

fn motor_ideal_point_right_contraction(self_1978: Motor, other_1730: IdealPoint) -> Scalar {
    var self_1979: Motor;
    var other_1731: IdealPoint;

    self_1979 = self_1978;
    other_1731 = other_1730;
    let _e5: Motor = self_1979;
    let _e8: IdealPoint = other_1731;
    let _e13: Motor = self_1979;
    let _e16: IdealPoint = other_1731;
    return Scalar(((0.0 - (_e5.g0_.z * _e8.g0_.x)) - (_e13.g0_.w * _e16.g0_.y)));
}

fn motor_ideal_point_left_anti_contraction(self_1980: Motor, other_1732: IdealPoint) -> AntiScalar {
    var self_1981: Motor;
    var other_1733: IdealPoint;

    self_1981 = self_1980;
    other_1733 = other_1732;
    let _e4: Motor = self_1981;
    let _e7: IdealPoint = other_1733;
    let _e11: Motor = self_1981;
    let _e14: IdealPoint = other_1733;
    return AntiScalar(((_e4.g0_.z * _e7.g0_.x) + (_e11.g0_.w * _e14.g0_.y)));
}

fn motor_ideal_point_scalar_product(self_1982: Motor, other_1734: IdealPoint) -> Scalar {
    var self_1983: Motor;
    var other_1735: IdealPoint;

    self_1983 = self_1982;
    other_1735 = other_1734;
    let _e5: Motor = self_1983;
    let _e8: IdealPoint = other_1735;
    let _e13: Motor = self_1983;
    let _e16: IdealPoint = other_1735;
    return Scalar(((0.0 - (_e5.g0_.z * _e8.g0_.x)) - (_e13.g0_.w * _e16.g0_.y)));
}

fn motor_ideal_point_anti_scalar_product(self_1984: Motor, other_1736: IdealPoint) -> AntiScalar {
    var self_1985: Motor;
    var other_1737: IdealPoint;

    self_1985 = self_1984;
    other_1737 = other_1736;
    let _e4: Motor = self_1985;
    let _e7: IdealPoint = other_1737;
    let _e11: Motor = self_1985;
    let _e14: IdealPoint = other_1737;
    return AntiScalar(((_e4.g0_.z * _e7.g0_.x) + (_e11.g0_.w * _e14.g0_.y)));
}

fn motor_plane_geometric_product(self_1986: Motor, other_1738: Plane) -> MotorDual {
    var self_1987: Motor;
    var other_1739: Plane;

    self_1987 = self_1986;
    other_1739 = other_1738;
    let _e4: Motor = self_1987;
    let _e8: Plane = other_1739;
    let _e11: Plane = other_1739;
    let _e14: Plane = other_1739;
    let _e17: Plane = other_1739;
    let _e29: Motor = self_1987;
    let _e33: Plane = other_1739;
    let _e36: Plane = other_1739;
    let _e39: Plane = other_1739;
    let _e42: Plane = other_1739;
    let _e55: Motor = self_1987;
    let _e59: Plane = other_1739;
    let _e62: Plane = other_1739;
    let _e65: Plane = other_1739;
    let _e68: Plane = other_1739;
    let _e81: Motor = self_1987;
    let _e85: Plane = other_1739;
    let _e88: Plane = other_1739;
    let _e91: Plane = other_1739;
    let _e94: Plane = other_1739;
    return MotorDual((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.x, _e11.g0_.x, _e14.g0_.z, _e17.g0_.y)) * vec4<f32>(1.0, 0.0, -(1.0), 1.0)) + ((vec4<f32>(_e29.g0_.z) * vec4<f32>(_e33.g0_.y, _e36.g0_.z, _e39.g0_.y, _e42.g0_.x)) * vec4<f32>(1.0, 1.0, 0.0, -(1.0)))) + ((vec4<f32>(_e55.g0_.w) * vec4<f32>(_e59.g0_.z, _e62.g0_.y, _e65.g0_.x, _e68.g0_.z)) * vec4<f32>(1.0, -(1.0), 1.0, 0.0))) + ((vec4<f32>(_e81.g0_.x) * vec4<f32>(_e85.g0_.x, _e88.g0_.x, _e91.g0_.y, _e94.g0_.z)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn motor_plane_regressive_product(self_1988: Motor, other_1740: Plane) -> Scalar {
    var self_1989: Motor;
    var other_1741: Plane;

    self_1989 = self_1988;
    other_1741 = other_1740;
    let _e4: Motor = self_1989;
    let _e7: Plane = other_1741;
    let _e11: Motor = self_1989;
    let _e14: Plane = other_1741;
    let _e19: Motor = self_1989;
    let _e22: Plane = other_1741;
    return Scalar((((_e4.g0_.y * _e7.g0_.x) + (_e11.g0_.z * _e14.g0_.y)) + (_e19.g0_.w * _e22.g0_.z)));
}

fn motor_plane_outer_product(self_1990: Motor, other_1742: Plane) -> MotorDual {
    var self_1991: Motor;
    var other_1743: Plane;

    self_1991 = self_1990;
    other_1743 = other_1742;
    let _e4: Motor = self_1991;
    let _e8: Plane = other_1743;
    let _e19: Motor = self_1991;
    let _e23: Plane = other_1743;
    let _e35: Motor = self_1991;
    let _e38: Plane = other_1743;
    let _e41: Plane = other_1743;
    let _e44: Plane = other_1743;
    let _e47: Plane = other_1743;
    return MotorDual(((((vec4<f32>(_e4.g0_.z) * vec4<f32>(_e8.g0_.y)) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) + ((vec4<f32>(_e19.g0_.w) * vec4<f32>(_e23.g0_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + (_e35.g0_.yxxx * vec4<f32>(_e38.g0_.x, _e41.g0_.x, _e44.g0_.y, _e47.g0_.z))));
}

fn motor_plane_inner_product(self_1992: Motor, other_1744: Plane) -> Plane {
    var self_1993: Motor;
    var other_1745: Plane;

    self_1993 = self_1992;
    other_1745 = other_1744;
    let _e4: Motor = self_1993;
    let _e8: Plane = other_1745;
    let _e11: Motor = self_1993;
    let _e15: Plane = other_1745;
    let _e26: Motor = self_1993;
    let _e30: Plane = other_1745;
    let _e41: Motor = self_1993;
    let _e44: Motor = self_1993;
    let _e47: Motor = self_1993;
    let _e51: Plane = other_1745;
    return Plane(((((vec3<f32>(_e4.g0_.x) * _e8.g0_) + ((vec3<f32>(_e11.g0_.z) * _e15.g0_.zzx) * vec3<f32>(1.0, 0.0, -(1.0)))) + ((vec3<f32>(_e26.g0_.w) * _e30.g0_.yxy) * vec3<f32>(-(1.0), 1.0, 0.0))) + ((vec3<f32>(_e41.g0_.x, _e44.g0_.y, _e47.g0_.y) * _e51.g0_.xzy) * vec3<f32>(0.0, -(1.0), 1.0))));
}

fn motor_plane_geometric_anti_product(self_1994: Motor, other_1746: Plane) -> Motor {
    var self_1995: Motor;
    var other_1747: Plane;

    self_1995 = self_1994;
    other_1747 = other_1746;
    let _e4: Motor = self_1995;
    let _e8: Plane = other_1747;
    let _e11: Plane = other_1747;
    let _e14: Plane = other_1747;
    let _e17: Plane = other_1747;
    let _e29: Motor = self_1995;
    let _e33: Plane = other_1747;
    let _e36: Plane = other_1747;
    let _e39: Plane = other_1747;
    let _e42: Plane = other_1747;
    let _e55: Motor = self_1995;
    let _e59: Plane = other_1747;
    let _e62: Plane = other_1747;
    let _e65: Plane = other_1747;
    let _e68: Plane = other_1747;
    let _e81: Motor = self_1995;
    let _e85: Plane = other_1747;
    let _e88: Plane = other_1747;
    let _e91: Plane = other_1747;
    let _e94: Plane = other_1747;
    return Motor((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.x, _e11.g0_.x, _e14.g0_.z, _e17.g0_.y)) * vec4<f32>(1.0, 0.0, -(1.0), 1.0)) + ((vec4<f32>(_e29.g0_.z) * vec4<f32>(_e33.g0_.y, _e36.g0_.z, _e39.g0_.y, _e42.g0_.x)) * vec4<f32>(1.0, 1.0, 0.0, -(1.0)))) + ((vec4<f32>(_e55.g0_.w) * vec4<f32>(_e59.g0_.z, _e62.g0_.y, _e65.g0_.x, _e68.g0_.z)) * vec4<f32>(1.0, -(1.0), 1.0, 0.0))) + ((vec4<f32>(_e81.g0_.x) * vec4<f32>(_e85.g0_.x, _e88.g0_.x, _e91.g0_.y, _e94.g0_.z)) * vec4<f32>(0.0, -(1.0), -(1.0), -(1.0)))));
}

fn motor_plane_inner_anti_product(self_1996: Motor, other_1748: Plane) -> Point {
    var self_1997: Motor;
    var other_1749: Plane;

    self_1997 = self_1996;
    other_1749 = other_1748;
    let _e6: Motor = self_1997;
    let _e10: Plane = other_1749;
    let _e14: Motor = self_1997;
    let _e18: Plane = other_1749;
    let _e29: Motor = self_1997;
    let _e33: Plane = other_1749;
    let _e44: Motor = self_1997;
    let _e47: Motor = self_1997;
    let _e50: Motor = self_1997;
    let _e54: Plane = other_1749;
    return Point(((((vec3<f32>(0.0) - (vec3<f32>(_e6.g0_.x) * _e10.g0_)) + ((vec3<f32>(_e14.g0_.z) * _e18.g0_.zzx) * vec3<f32>(1.0, 0.0, -(1.0)))) + ((vec3<f32>(_e29.g0_.w) * _e33.g0_.yxy) * vec3<f32>(-(1.0), 1.0, 0.0))) + ((vec3<f32>(_e44.g0_.x, _e47.g0_.y, _e50.g0_.y) * _e54.g0_.xzy) * vec3<f32>(0.0, -(1.0), 1.0))));
}

fn motor_plane_left_contraction(self_1998: Motor, other_1750: Plane) -> Plane {
    var self_1999: Motor;
    var other_1751: Plane;

    self_1999 = self_1998;
    other_1751 = other_1750;
    let _e4: Motor = self_1999;
    let _e8: Plane = other_1751;
    return Plane((vec3<f32>(_e4.g0_.x) * _e8.g0_));
}

fn motor_plane_right_contraction(self_2000: Motor, other_1752: Plane) -> Plane {
    var self_2001: Motor;
    var other_1753: Plane;

    self_2001 = self_2000;
    other_1753 = other_1752;
    let _e4: Motor = self_2001;
    let _e8: Plane = other_1753;
    let _e18: Motor = self_2001;
    let _e22: Plane = other_1753;
    let _e33: Motor = self_2001;
    let _e36: Motor = self_2001;
    let _e39: Motor = self_2001;
    let _e43: Plane = other_1753;
    return Plane(((((vec3<f32>(_e4.g0_.z) * _e8.g0_.zzx) * vec3<f32>(1.0, 0.0, -(1.0))) + ((vec3<f32>(_e18.g0_.w) * _e22.g0_.yxy) * vec3<f32>(-(1.0), 1.0, 0.0))) + ((vec3<f32>(_e33.g0_.x, _e36.g0_.y, _e39.g0_.y) * _e43.g0_.xzy) * vec3<f32>(0.0, -(1.0), 1.0))));
}

fn motor_plane_left_anti_contraction(self_2002: Motor, other_1754: Plane) -> Point {
    var self_2003: Motor;
    var other_1755: Plane;

    self_2003 = self_2002;
    other_1755 = other_1754;
    let _e4: Motor = self_2003;
    let _e8: Plane = other_1755;
    let _e18: Motor = self_2003;
    let _e22: Plane = other_1755;
    let _e33: Motor = self_2003;
    let _e36: Motor = self_2003;
    let _e39: Motor = self_2003;
    let _e43: Plane = other_1755;
    return Point(((((vec3<f32>(_e4.g0_.z) * _e8.g0_.zzx) * vec3<f32>(1.0, 0.0, -(1.0))) + ((vec3<f32>(_e18.g0_.w) * _e22.g0_.yxy) * vec3<f32>(-(1.0), 1.0, 0.0))) + ((vec3<f32>(_e33.g0_.x, _e36.g0_.y, _e39.g0_.y) * _e43.g0_.xzy) * vec3<f32>(0.0, -(1.0), 1.0))));
}

fn motor_plane_right_anti_contraction(self_2004: Motor, other_1756: Plane) -> Point {
    var self_2005: Motor;
    var other_1757: Plane;

    self_2005 = self_2004;
    other_1757 = other_1756;
    let _e6: Motor = self_2005;
    let _e10: Plane = other_1757;
    return Point((vec3<f32>(0.0) - (vec3<f32>(_e6.g0_.x) * _e10.g0_)));
}

fn motor_translator_into(self_2006: Motor) -> Translator {
    var self_2007: Motor;

    self_2007 = self_2006;
    let _e2: Motor = self_2007;
    let _e5: Motor = self_2007;
    let _e8: Motor = self_2007;
    return Translator(vec3<f32>(_e2.g0_.x, _e5.g0_.z, _e8.g0_.w));
}

fn motor_translator_add(self_2008: Motor, other_1758: Translator) -> Motor {
    var self_2009: Motor;
    var other_1759: Translator;

    self_2009 = self_2008;
    other_1759 = other_1758;
    let _e4: Motor = self_2009;
    let _e6: Translator = other_1759;
    let _e9: Translator = other_1759;
    let _e12: Translator = other_1759;
    let _e15: Translator = other_1759;
    return Motor((_e4.g0_ + (vec4<f32>(_e6.g0_.x, _e9.g0_.x, _e12.g0_.y, _e15.g0_.z) * vec4<f32>(1.0, 0.0, 1.0, 1.0))));
}

fn motor_translator_sub(self_2010: Motor, other_1760: Translator) -> Motor {
    var self_2011: Motor;
    var other_1761: Translator;

    self_2011 = self_2010;
    other_1761 = other_1760;
    let _e4: Motor = self_2011;
    let _e6: Translator = other_1761;
    let _e9: Translator = other_1761;
    let _e12: Translator = other_1761;
    let _e15: Translator = other_1761;
    return Motor((_e4.g0_ - (vec4<f32>(_e6.g0_.x, _e9.g0_.x, _e12.g0_.y, _e15.g0_.z) * vec4<f32>(1.0, 0.0, 1.0, 1.0))));
}

fn motor_translator_geometric_product(self_2012: Motor, other_1762: Translator) -> Motor {
    var self_2013: Motor;
    var other_1763: Translator;

    self_2013 = self_2012;
    other_1763 = other_1762;
    let _e4: Motor = self_2013;
    let _e8: Translator = other_1763;
    let _e11: Translator = other_1763;
    let _e14: Translator = other_1763;
    let _e17: Translator = other_1763;
    let _e29: Motor = self_2013;
    let _e33: Translator = other_1763;
    let _e36: Translator = other_1763;
    let _e39: Translator = other_1763;
    let _e42: Translator = other_1763;
    let _e55: Motor = self_2013;
    let _e59: Translator = other_1763;
    let _e62: Translator = other_1763;
    let _e65: Translator = other_1763;
    let _e68: Translator = other_1763;
    let _e82: Motor = self_2013;
    let _e86: Translator = other_1763;
    let _e89: Translator = other_1763;
    let _e92: Translator = other_1763;
    let _e95: Translator = other_1763;
    return Motor((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.x, _e11.g0_.x, _e14.g0_.z, _e17.g0_.y)) * vec4<f32>(0.0, 1.0, -(1.0), 1.0)) + ((vec4<f32>(_e29.g0_.z) * vec4<f32>(_e33.g0_.y, _e36.g0_.z, _e39.g0_.x, _e42.g0_.y)) * vec4<f32>(-(1.0), 1.0, 1.0, 0.0))) + ((vec4<f32>(_e55.g0_.w) * vec4<f32>(_e59.g0_.z, _e62.g0_.y, _e65.g0_.z, _e68.g0_.x)) * vec4<f32>(-(1.0), -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e82.g0_.x) * vec4<f32>(_e86.g0_.x, _e89.g0_.x, _e92.g0_.y, _e95.g0_.z)) * vec4<f32>(1.0, 0.0, 1.0, 1.0))));
}

fn motor_translator_regressive_product(self_2014: Motor, other_1764: Translator) -> Plane {
    var self_2015: Motor;
    var other_1765: Translator;

    self_2015 = self_2014;
    other_1765 = other_1764;
    let _e4: Motor = self_2015;
    let _e8: Translator = other_1765;
    let _e18: Motor = self_2015;
    let _e21: Motor = self_2015;
    let _e24: Motor = self_2015;
    let _e28: Translator = other_1765;
    return Plane((((vec3<f32>(_e4.g0_.w) * vec3<f32>(_e8.g0_.y)) * vec3<f32>(1.0, 0.0, 0.0)) + ((vec3<f32>(_e18.g0_.z, _e21.g0_.y, _e24.g0_.y) * _e28.g0_.zzy) * vec3<f32>(-(1.0), 1.0, -(1.0)))));
}

fn motor_translator_outer_product(self_2016: Motor, other_1766: Translator) -> Motor {
    var self_2017: Motor;
    var other_1767: Translator;

    self_2017 = self_2016;
    other_1767 = other_1766;
    let _e4: Motor = self_2017;
    let _e8: Translator = other_1767;
    let _e19: Motor = self_2017;
    let _e23: Translator = other_1767;
    let _e35: Motor = self_2017;
    let _e38: Translator = other_1767;
    let _e41: Translator = other_1767;
    let _e44: Translator = other_1767;
    let _e47: Translator = other_1767;
    return Motor(((((vec4<f32>(_e4.g0_.z) * vec4<f32>(_e8.g0_.x)) * vec4<f32>(0.0, 0.0, 1.0, 0.0)) + ((vec4<f32>(_e19.g0_.w) * vec4<f32>(_e23.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + (_e35.g0_.xyxx * vec4<f32>(_e38.g0_.x, _e41.g0_.x, _e44.g0_.y, _e47.g0_.z))));
}

fn motor_translator_inner_product(self_2018: Motor, other_1768: Translator) -> Motor {
    var self_2019: Motor;
    var other_1769: Translator;

    self_2019 = self_2018;
    other_1769 = other_1768;
    let _e4: Motor = self_2019;
    let _e8: Translator = other_1769;
    let _e11: Translator = other_1769;
    let _e14: Translator = other_1769;
    let _e17: Translator = other_1769;
    let _e29: Motor = self_2019;
    let _e33: Translator = other_1769;
    let _e36: Translator = other_1769;
    let _e39: Translator = other_1769;
    let _e42: Translator = other_1769;
    let _e55: Motor = self_2019;
    let _e58: Translator = other_1769;
    let _e61: Translator = other_1769;
    let _e64: Translator = other_1769;
    let _e67: Translator = other_1769;
    return Motor(((((vec4<f32>(_e4.g0_.z) * vec4<f32>(_e8.g0_.y, _e11.g0_.y, _e14.g0_.x, _e17.g0_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0)) + ((vec4<f32>(_e29.g0_.w) * vec4<f32>(_e33.g0_.z, _e36.g0_.z, _e39.g0_.z, _e42.g0_.x)) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))) + (_e55.g0_.xyxx * vec4<f32>(_e58.g0_.x, _e61.g0_.x, _e64.g0_.y, _e67.g0_.z))));
}

fn motor_translator_geometric_anti_product(self_2020: Motor, other_1770: Translator) -> MotorDual {
    var self_2021: Motor;
    var other_1771: Translator;

    self_2021 = self_2020;
    other_1771 = other_1770;
    let _e4: Motor = self_2021;
    let _e8: Translator = other_1771;
    let _e11: Translator = other_1771;
    let _e14: Translator = other_1771;
    let _e17: Translator = other_1771;
    let _e29: Motor = self_2021;
    let _e33: Translator = other_1771;
    let _e36: Translator = other_1771;
    let _e39: Translator = other_1771;
    let _e42: Translator = other_1771;
    let _e55: Motor = self_2021;
    let _e59: Translator = other_1771;
    let _e62: Translator = other_1771;
    let _e65: Translator = other_1771;
    let _e68: Translator = other_1771;
    let _e80: Motor = self_2021;
    let _e84: Translator = other_1771;
    let _e87: Translator = other_1771;
    let _e90: Translator = other_1771;
    let _e93: Translator = other_1771;
    return MotorDual((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.x, _e11.g0_.x, _e14.g0_.z, _e17.g0_.y)) * vec4<f32>(0.0, 1.0, 1.0, -(1.0))) + ((vec4<f32>(_e29.g0_.z) * vec4<f32>(_e33.g0_.y, _e36.g0_.z, _e39.g0_.x, _e42.g0_.y)) * vec4<f32>(1.0, -(1.0), 1.0, 0.0))) + ((vec4<f32>(_e55.g0_.w) * vec4<f32>(_e59.g0_.z, _e62.g0_.y, _e65.g0_.z, _e68.g0_.x)) * vec4<f32>(1.0, 1.0, 0.0, 1.0))) + ((vec4<f32>(_e80.g0_.x) * vec4<f32>(_e84.g0_.x, _e87.g0_.x, _e90.g0_.y, _e93.g0_.z)) * vec4<f32>(-(1.0), 0.0, 1.0, 1.0))));
}

fn motor_translator_inner_anti_product(self_2022: Motor, other_1772: Translator) -> MotorDual {
    var self_2023: Motor;
    var other_1773: Translator;

    self_2023 = self_2022;
    other_1773 = other_1772;
    let _e4: Motor = self_2023;
    let _e8: Translator = other_1773;
    let _e11: Translator = other_1773;
    let _e14: Translator = other_1773;
    let _e17: Translator = other_1773;
    let _e28: Motor = self_2023;
    let _e32: Translator = other_1773;
    let _e35: Translator = other_1773;
    let _e38: Translator = other_1773;
    let _e41: Translator = other_1773;
    let _e53: Motor = self_2023;
    let _e56: Translator = other_1773;
    let _e59: Translator = other_1773;
    let _e62: Translator = other_1773;
    let _e65: Translator = other_1773;
    return MotorDual(((((vec4<f32>(_e4.g0_.z) * vec4<f32>(_e8.g0_.y, _e11.g0_.y, _e14.g0_.x, _e17.g0_.y)) * vec4<f32>(1.0, 0.0, 1.0, 0.0)) + ((vec4<f32>(_e28.g0_.w) * vec4<f32>(_e32.g0_.z, _e35.g0_.z, _e38.g0_.z, _e41.g0_.x)) * vec4<f32>(1.0, 0.0, 0.0, 1.0))) + ((_e53.g0_.xyxx * vec4<f32>(_e56.g0_.x, _e59.g0_.x, _e62.g0_.y, _e65.g0_.z)) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0))));
}

fn motor_translator_left_contraction(self_2024: Motor, other_1774: Translator) -> Translator {
    var self_2025: Motor;
    var other_1775: Translator;

    self_2025 = self_2024;
    other_1775 = other_1774;
    let _e4: Motor = self_2025;
    let _e8: Translator = other_1775;
    let _e11: Motor = self_2025;
    let _e15: Translator = other_1775;
    let _e27: Motor = self_2025;
    let _e30: Motor = self_2025;
    let _e33: Motor = self_2025;
    let _e37: Translator = other_1775;
    return Translator((((vec3<f32>(_e4.g0_.x) * _e8.g0_) + ((vec3<f32>(_e11.g0_.w) * vec3<f32>(_e15.g0_.z)) * vec3<f32>(-(1.0), 0.0, 0.0))) + ((vec3<f32>(_e27.g0_.z, _e30.g0_.x, _e33.g0_.x) * _e37.g0_.yxx) * vec3<f32>(-(1.0), 0.0, 0.0))));
}

fn motor_translator_right_contraction(self_2026: Motor, other_1776: Translator) -> Motor {
    var self_2027: Motor;
    var other_1777: Translator;

    self_2027 = self_2026;
    other_1777 = other_1776;
    let _e4: Motor = self_2027;
    let _e8: Translator = other_1777;
    let _e11: Translator = other_1777;
    let _e14: Translator = other_1777;
    let _e17: Translator = other_1777;
    let _e29: Motor = self_2027;
    let _e33: Translator = other_1777;
    let _e36: Translator = other_1777;
    let _e39: Translator = other_1777;
    let _e42: Translator = other_1777;
    let _e55: Motor = self_2027;
    let _e58: Translator = other_1777;
    return Motor(((((vec4<f32>(_e4.g0_.z) * vec4<f32>(_e8.g0_.y, _e11.g0_.y, _e14.g0_.x, _e17.g0_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0)) + ((vec4<f32>(_e29.g0_.w) * vec4<f32>(_e33.g0_.z, _e36.g0_.z, _e39.g0_.z, _e42.g0_.x)) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))) + ((_e55.g0_.xyxx * vec4<f32>(_e58.g0_.x)) * vec4<f32>(1.0, 1.0, 0.0, 0.0))));
}

fn motor_translator_left_anti_contraction(self_2028: Motor, other_1778: Translator) -> MotorDual {
    var self_2029: Motor;
    var other_1779: Translator;

    self_2029 = self_2028;
    other_1779 = other_1778;
    let _e4: Motor = self_2029;
    let _e8: Translator = other_1779;
    let _e11: Translator = other_1779;
    let _e14: Translator = other_1779;
    let _e17: Translator = other_1779;
    let _e28: Motor = self_2029;
    let _e32: Translator = other_1779;
    let _e35: Translator = other_1779;
    let _e38: Translator = other_1779;
    let _e41: Translator = other_1779;
    let _e53: Motor = self_2029;
    let _e56: Translator = other_1779;
    return MotorDual(((((vec4<f32>(_e4.g0_.z) * vec4<f32>(_e8.g0_.y, _e11.g0_.y, _e14.g0_.x, _e17.g0_.y)) * vec4<f32>(1.0, 0.0, 1.0, 0.0)) + ((vec4<f32>(_e28.g0_.w) * vec4<f32>(_e32.g0_.z, _e35.g0_.z, _e38.g0_.z, _e41.g0_.x)) * vec4<f32>(1.0, 0.0, 0.0, 1.0))) + ((_e53.g0_.xyxx * vec4<f32>(_e56.g0_.x)) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))));
}

fn motor_translator_scalar_product(self_2030: Motor, other_1780: Translator) -> Scalar {
    var self_2031: Motor;
    var other_1781: Translator;

    self_2031 = self_2030;
    other_1781 = other_1780;
    let _e4: Motor = self_2031;
    let _e7: Translator = other_1781;
    let _e11: Motor = self_2031;
    let _e14: Translator = other_1781;
    let _e19: Motor = self_2031;
    let _e22: Translator = other_1781;
    return Scalar((((_e4.g0_.x * _e7.g0_.x) - (_e11.g0_.z * _e14.g0_.y)) - (_e19.g0_.w * _e22.g0_.z)));
}

fn motor_translator_anti_scalar_product(self_2032: Motor, other_1782: Translator) -> AntiScalar {
    var self_2033: Motor;
    var other_1783: Translator;

    self_2033 = self_2032;
    other_1783 = other_1782;
    let _e5: Motor = self_2033;
    let _e8: Translator = other_1783;
    let _e13: Motor = self_2033;
    let _e16: Translator = other_1783;
    let _e21: Motor = self_2033;
    let _e24: Translator = other_1783;
    return AntiScalar((((0.0 - (_e5.g0_.x * _e8.g0_.x)) + (_e13.g0_.z * _e16.g0_.y)) + (_e21.g0_.w * _e24.g0_.z)));
}

fn motor_motor_add(self_2034: Motor, other_1784: Motor) -> Motor {
    var self_2035: Motor;
    var other_1785: Motor;

    self_2035 = self_2034;
    other_1785 = other_1784;
    let _e4: Motor = self_2035;
    let _e6: Motor = other_1785;
    return Motor((_e4.g0_ + _e6.g0_));
}

fn motor_motor_sub(self_2036: Motor, other_1786: Motor) -> Motor {
    var self_2037: Motor;
    var other_1787: Motor;

    self_2037 = self_2036;
    other_1787 = other_1786;
    let _e4: Motor = self_2037;
    let _e6: Motor = other_1787;
    return Motor((_e4.g0_ - _e6.g0_));
}

fn motor_motor_mul(self_2038: Motor, other_1788: Motor) -> Motor {
    var self_2039: Motor;
    var other_1789: Motor;

    self_2039 = self_2038;
    other_1789 = other_1788;
    let _e4: Motor = self_2039;
    let _e6: Motor = other_1789;
    return Motor((_e4.g0_ * _e6.g0_));
}

fn motor_motor_div(self_2040: Motor, other_1790: Motor) -> Motor {
    var self_2041: Motor;
    var other_1791: Motor;

    self_2041 = self_2040;
    other_1791 = other_1790;
    let _e4: Motor = self_2041;
    let _e7: Motor = self_2041;
    let _e10: Motor = self_2041;
    let _e13: Motor = self_2041;
    let _e23: Motor = other_1791;
    let _e26: Motor = other_1791;
    let _e29: Motor = other_1791;
    let _e32: Motor = other_1791;
    return Motor((((vec4<f32>(_e4.g0_.x, _e7.g0_.y, _e10.g0_.z, _e13.g0_.w) * vec4<f32>(1.0, 1.0, 1.0, 1.0)) / vec4<f32>(_e23.g0_.x, _e26.g0_.y, _e29.g0_.z, _e32.g0_.w)) * vec4<f32>(1.0, 1.0, 1.0, 1.0)));
}

fn motor_motor_geometric_product(self_2042: Motor, other_1792: Motor) -> Motor {
    var self_2043: Motor;
    var other_1793: Motor;

    self_2043 = self_2042;
    other_1793 = other_1792;
    let _e4: Motor = self_2043;
    let _e8: Motor = other_1793;
    let _e11: Motor = self_2043;
    let _e15: Motor = other_1793;
    let _e28: Motor = self_2043;
    let _e32: Motor = other_1793;
    let _e45: Motor = self_2043;
    let _e49: Motor = other_1793;
    return Motor(((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.y) * _e15.g0_.yxwz) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))) + ((vec4<f32>(_e28.g0_.z) * _e32.g0_.zwxy) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e45.g0_.w) * _e49.g0_.wzyx) * vec4<f32>(-(1.0), -(1.0), 1.0, 1.0))));
}

fn motor_motor_regressive_product(self_2044: Motor, other_1794: Motor) -> Plane {
    var self_2045: Motor;
    var other_1795: Motor;

    self_2045 = self_2044;
    other_1795 = other_1794;
    let _e4: Motor = self_2045;
    let _e8: Motor = other_1795;
    let _e11: Motor = other_1795;
    let _e14: Motor = other_1795;
    let _e25: Motor = self_2045;
    let _e29: Motor = other_1795;
    let _e32: Motor = other_1795;
    let _e35: Motor = other_1795;
    let _e47: Motor = self_2045;
    let _e50: Motor = self_2045;
    let _e53: Motor = self_2045;
    let _e57: Motor = other_1795;
    let _e60: Motor = other_1795;
    let _e63: Motor = other_1795;
    return Plane(((((vec3<f32>(_e4.g0_.z) * vec3<f32>(_e8.g0_.w, _e11.g0_.w, _e14.g0_.y)) * vec3<f32>(-(1.0), 0.0, 1.0)) + ((vec3<f32>(_e25.g0_.w) * vec3<f32>(_e29.g0_.z, _e32.g0_.y, _e35.g0_.z)) * vec3<f32>(1.0, -(1.0), 0.0))) + ((vec3<f32>(_e47.g0_.x, _e50.g0_.y, _e53.g0_.y) * vec3<f32>(_e57.g0_.x, _e60.g0_.w, _e63.g0_.z)) * vec3<f32>(0.0, 1.0, -(1.0)))));
}

fn motor_motor_outer_product(self_2046: Motor, other_1796: Motor) -> Motor {
    var self_2047: Motor;
    var other_1797: Motor;

    self_2047 = self_2046;
    other_1797 = other_1796;
    let _e4: Motor = self_2047;
    let _e8: Motor = other_1797;
    let _e11: Motor = self_2047;
    let _e13: Motor = other_1797;
    return Motor(((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((_e11.g0_ * vec4<f32>(_e13.g0_.x)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn motor_motor_inner_product(self_2048: Motor, other_1798: Motor) -> Motor {
    var self_2049: Motor;
    var other_1799: Motor;

    self_2049 = self_2048;
    other_1799 = other_1798;
    let _e4: Motor = self_2049;
    let _e8: Motor = other_1799;
    let _e11: Motor = self_2049;
    let _e15: Motor = other_1799;
    let _e27: Motor = self_2049;
    let _e31: Motor = other_1799;
    let _e43: Motor = self_2049;
    let _e46: Motor = other_1799;
    return Motor(((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.z) * _e15.g0_.zzxz) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0))) + ((vec4<f32>(_e27.g0_.w) * _e31.g0_.wwwx) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))) + ((_e43.g0_.yyxx * _e46.g0_.yxxx) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))));
}

fn motor_motor_geometric_anti_product(self_2050: Motor, other_1800: Motor) -> MotorDual {
    var self_2051: Motor;
    var other_1801: Motor;

    self_2051 = self_2050;
    other_1801 = other_1800;
    let _e4: Motor = self_2051;
    let _e8: Motor = other_1801;
    let _e18: Motor = self_2051;
    let _e22: Motor = other_1801;
    let _e34: Motor = self_2051;
    let _e38: Motor = other_1801;
    let _e50: Motor = self_2051;
    let _e54: Motor = other_1801;
    return MotorDual((((((vec4<f32>(_e4.g0_.x) * _e8.g0_) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0)) + ((vec4<f32>(_e18.g0_.y) * _e22.g0_.yxwz) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e34.g0_.z) * _e38.g0_.zwxy) * vec4<f32>(1.0, -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e50.g0_.w) * _e54.g0_.wzyx) * vec4<f32>(1.0, 1.0, -(1.0), 1.0))));
}

fn motor_motor_inner_anti_product(self_2052: Motor, other_1802: Motor) -> MotorDual {
    var self_2053: Motor;
    var other_1803: Motor;

    self_2053 = self_2052;
    other_1803 = other_1802;
    let _e4: Motor = self_2053;
    let _e8: Motor = other_1803;
    let _e18: Motor = self_2053;
    let _e22: Motor = other_1803;
    let _e33: Motor = self_2053;
    let _e37: Motor = other_1803;
    let _e48: Motor = self_2053;
    let _e51: Motor = other_1803;
    return MotorDual((((((vec4<f32>(_e4.g0_.x) * _e8.g0_) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0)) + ((vec4<f32>(_e18.g0_.z) * _e22.g0_.zzxz) * vec4<f32>(1.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e33.g0_.w) * _e37.g0_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, 1.0))) + ((_e48.g0_.yyxx * _e51.g0_.yxxx) * vec4<f32>(1.0, 1.0, 0.0, 0.0))));
}

fn motor_motor_left_contraction(self_2054: Motor, other_1804: Motor) -> Motor {
    var self_2055: Motor;
    var other_1805: Motor;

    self_2055 = self_2054;
    other_1805 = other_1804;
    let _e4: Motor = self_2055;
    let _e8: Motor = other_1805;
    let _e11: Motor = self_2055;
    let _e15: Motor = other_1805;
    let _e28: Motor = self_2055;
    let _e32: Motor = other_1805;
    let _e45: Motor = self_2055;
    let _e48: Motor = other_1805;
    return Motor(((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.z) * vec4<f32>(_e15.g0_.z)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((vec4<f32>(_e28.g0_.w) * vec4<f32>(_e32.g0_.w)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((_e45.g0_.yxxx * _e48.g0_.yxxx) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))));
}

fn motor_motor_right_contraction(self_2056: Motor, other_1806: Motor) -> Motor {
    var self_2057: Motor;
    var other_1807: Motor;

    self_2057 = self_2056;
    other_1807 = other_1806;
    let _e4: Motor = self_2057;
    let _e8: Motor = other_1807;
    let _e19: Motor = self_2057;
    let _e23: Motor = other_1807;
    let _e35: Motor = self_2057;
    let _e39: Motor = other_1807;
    let _e51: Motor = self_2057;
    let _e55: Motor = other_1807;
    return Motor((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0)) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.zzxz) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0))) + ((vec4<f32>(_e35.g0_.w) * _e39.g0_.wwwx) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))) + ((vec4<f32>(_e51.g0_.x) * vec4<f32>(_e55.g0_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))));
}

fn motor_motor_left_anti_contraction(self_2058: Motor, other_1808: Motor) -> MotorDual {
    var self_2059: Motor;
    var other_1809: Motor;

    self_2059 = self_2058;
    other_1809 = other_1808;
    let _e4: Motor = self_2059;
    let _e8: Motor = other_1809;
    let _e18: Motor = self_2059;
    let _e22: Motor = other_1809;
    let _e33: Motor = self_2059;
    let _e37: Motor = other_1809;
    let _e48: Motor = self_2059;
    let _e52: Motor = other_1809;
    return MotorDual((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.yxyy) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) + ((vec4<f32>(_e18.g0_.z) * _e22.g0_.zzxz) * vec4<f32>(1.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e33.g0_.w) * _e37.g0_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e48.g0_.x) * vec4<f32>(_e52.g0_.x)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))));
}

fn motor_motor_right_anti_contraction(self_2060: Motor, other_1810: Motor) -> MotorDual {
    var self_2061: Motor;
    var other_1811: Motor;

    self_2061 = self_2060;
    other_1811 = other_1810;
    let _e4: Motor = self_2061;
    let _e8: Motor = other_1811;
    let _e18: Motor = self_2061;
    let _e22: Motor = other_1811;
    let _e34: Motor = self_2061;
    let _e38: Motor = other_1811;
    let _e50: Motor = self_2061;
    let _e53: Motor = other_1811;
    return MotorDual((((((vec4<f32>(_e4.g0_.x) * _e8.g0_) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0)) + ((vec4<f32>(_e18.g0_.z) * vec4<f32>(_e22.g0_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e34.g0_.w) * vec4<f32>(_e38.g0_.w)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((_e50.g0_.yxxx * _e53.g0_.yxxx) * vec4<f32>(1.0, 0.0, 0.0, 0.0))));
}

fn motor_motor_scalar_product(self_2062: Motor, other_1812: Motor) -> Scalar {
    var self_2063: Motor;
    var other_1813: Motor;

    self_2063 = self_2062;
    other_1813 = other_1812;
    let _e4: Motor = self_2063;
    let _e7: Motor = other_1813;
    let _e11: Motor = self_2063;
    let _e14: Motor = other_1813;
    let _e19: Motor = self_2063;
    let _e22: Motor = other_1813;
    let _e27: Motor = self_2063;
    let _e30: Motor = other_1813;
    return Scalar(((((_e4.g0_.x * _e7.g0_.x) - (_e11.g0_.y * _e14.g0_.y)) - (_e19.g0_.z * _e22.g0_.z)) - (_e27.g0_.w * _e30.g0_.w)));
}

fn motor_motor_anti_scalar_product(self_2064: Motor, other_1814: Motor) -> AntiScalar {
    var self_2065: Motor;
    var other_1815: Motor;

    self_2065 = self_2064;
    other_1815 = other_1814;
    let _e5: Motor = self_2065;
    let _e8: Motor = other_1815;
    let _e13: Motor = self_2065;
    let _e16: Motor = other_1815;
    let _e21: Motor = self_2065;
    let _e24: Motor = other_1815;
    let _e29: Motor = self_2065;
    let _e32: Motor = other_1815;
    return AntiScalar(((((0.0 - (_e5.g0_.x * _e8.g0_.x)) + (_e13.g0_.y * _e16.g0_.y)) + (_e21.g0_.z * _e24.g0_.z)) + (_e29.g0_.w * _e32.g0_.w)));
}

fn motor_motor_dual_add(self_2066: Motor, other_1816: MotorDual) -> MultiVector {
    var self_2067: Motor;
    var other_1817: MotorDual;

    self_2067 = self_2066;
    other_1817 = other_1816;
    let _e4: Motor = self_2067;
    let _e13: MotorDual = other_1817;
    let _e23: Motor = self_2067;
    let _e32: MotorDual = other_1817;
    return MultiVector(((_e4.g0_.xyxx * vec4<f32>(1.0, 1.0, 0.0, 0.0)) + (_e13.g0_.xxwz * vec4<f32>(0.0, 0.0, 1.0, 1.0))), ((_e23.g0_.xxzw * vec4<f32>(0.0, 0.0, 1.0, 1.0)) + (_e32.g0_.yxxx * vec4<f32>(1.0, 1.0, 0.0, 0.0))));
}

fn motor_motor_dual_sub(self_2068: Motor, other_1818: MotorDual) -> MultiVector {
    var self_2069: Motor;
    var other_1819: MotorDual;

    self_2069 = self_2068;
    other_1819 = other_1818;
    let _e4: Motor = self_2069;
    let _e13: MotorDual = other_1819;
    let _e23: Motor = self_2069;
    let _e32: MotorDual = other_1819;
    return MultiVector(((_e4.g0_.xyxx * vec4<f32>(1.0, 1.0, 0.0, 0.0)) - (_e13.g0_.xxwz * vec4<f32>(0.0, 0.0, 1.0, 1.0))), ((_e23.g0_.xxzw * vec4<f32>(0.0, 0.0, 1.0, 1.0)) - (_e32.g0_.yxxx * vec4<f32>(1.0, 1.0, 0.0, 0.0))));
}

fn motor_motor_dual_geometric_product(self_2070: Motor, other_1820: MotorDual) -> MotorDual {
    var self_2071: Motor;
    var other_1821: MotorDual;

    self_2071 = self_2070;
    other_1821 = other_1820;
    let _e4: Motor = self_2071;
    let _e8: MotorDual = other_1821;
    let _e11: Motor = self_2071;
    let _e15: MotorDual = other_1821;
    let _e28: Motor = self_2071;
    let _e32: MotorDual = other_1821;
    let _e45: Motor = self_2071;
    let _e49: MotorDual = other_1821;
    return MotorDual(((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.y) * _e15.g0_.yxwz) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e28.g0_.z) * _e32.g0_.zwxy) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e45.g0_.w) * _e49.g0_.wzyx) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))));
}

fn motor_motor_dual_regressive_product(self_2072: Motor, other_1822: MotorDual) -> Motor {
    var self_2073: Motor;
    var other_1823: MotorDual;

    self_2073 = self_2072;
    other_1823 = other_1822;
    let _e4: Motor = self_2073;
    let _e8: MotorDual = other_1823;
    let _e18: Motor = self_2073;
    let _e22: MotorDual = other_1823;
    let _e33: Motor = self_2073;
    let _e37: MotorDual = other_1823;
    let _e48: Motor = self_2073;
    let _e52: MotorDual = other_1823;
    return Motor((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.yxyy) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) + ((vec4<f32>(_e18.g0_.z) * _e22.g0_.zzxz) * vec4<f32>(1.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e33.g0_.w) * _e37.g0_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e48.g0_.x) * vec4<f32>(_e52.g0_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))));
}

fn motor_motor_dual_outer_product(self_2074: Motor, other_1824: MotorDual) -> MotorDual {
    var self_2075: Motor;
    var other_1825: MotorDual;

    self_2075 = self_2074;
    other_1825 = other_1824;
    let _e4: Motor = self_2075;
    let _e8: MotorDual = other_1825;
    let _e11: Motor = self_2075;
    let _e15: MotorDual = other_1825;
    let _e27: Motor = self_2075;
    let _e31: MotorDual = other_1825;
    let _e43: Motor = self_2075;
    let _e46: MotorDual = other_1825;
    return MotorDual(((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.z) * vec4<f32>(_e15.g0_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e27.g0_.w) * vec4<f32>(_e31.g0_.w)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((_e43.g0_.yxxx * _e46.g0_.yxxx) * vec4<f32>(1.0, 0.0, 0.0, 0.0))));
}

fn motor_motor_dual_inner_product(self_2076: Motor, other_1826: MotorDual) -> MotorDual {
    var self_2077: Motor;
    var other_1827: MotorDual;

    self_2077 = self_2076;
    other_1827 = other_1826;
    let _e4: Motor = self_2077;
    let _e8: MotorDual = other_1827;
    let _e11: Motor = self_2077;
    let _e15: MotorDual = other_1827;
    let _e28: Motor = self_2077;
    let _e32: MotorDual = other_1827;
    let _e45: Motor = self_2077;
    let _e48: MotorDual = other_1827;
    return MotorDual(((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.z) * _e15.g0_.wwxy) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e28.g0_.w) * _e32.g0_.zzyx) * vec4<f32>(0.0, -(1.0), 1.0, -(1.0)))) + ((_e45.g0_.xyyy * _e48.g0_.xxwz) * vec4<f32>(0.0, -(1.0), -(1.0), 1.0))));
}

fn motor_motor_dual_geometric_anti_product(self_2078: Motor, other_1828: MotorDual) -> Motor {
    var self_2079: Motor;
    var other_1829: MotorDual;

    self_2079 = self_2078;
    other_1829 = other_1828;
    let _e4: Motor = self_2079;
    let _e8: MotorDual = other_1829;
    let _e20: Motor = self_2079;
    let _e24: MotorDual = other_1829;
    let _e36: Motor = self_2079;
    let _e40: MotorDual = other_1829;
    let _e52: Motor = self_2079;
    let _e56: MotorDual = other_1829;
    return Motor((((((vec4<f32>(_e4.g0_.x) * _e8.g0_) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))) + ((vec4<f32>(_e20.g0_.y) * _e24.g0_.yxwz) * vec4<f32>(1.0, 1.0, -(1.0), 1.0))) + ((vec4<f32>(_e36.g0_.z) * _e40.g0_.zwxy) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e52.g0_.w) * _e56.g0_.wzyx) * vec4<f32>(1.0, -(1.0), 1.0, 1.0))));
}

fn motor_motor_dual_inner_anti_product(self_2080: Motor, other_1830: MotorDual) -> Motor {
    var self_2081: Motor;
    var other_1831: MotorDual;

    self_2081 = self_2080;
    other_1831 = other_1830;
    let _e4: Motor = self_2081;
    let _e8: MotorDual = other_1831;
    let _e20: Motor = self_2081;
    let _e24: MotorDual = other_1831;
    let _e36: Motor = self_2081;
    let _e40: MotorDual = other_1831;
    let _e52: Motor = self_2081;
    let _e55: MotorDual = other_1831;
    return Motor((((((vec4<f32>(_e4.g0_.x) * _e8.g0_) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))) + ((vec4<f32>(_e20.g0_.z) * _e24.g0_.wwxy) * vec4<f32>(0.0, 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e36.g0_.w) * _e40.g0_.zzyx) * vec4<f32>(0.0, -(1.0), 1.0, 1.0))) + ((_e52.g0_.xyyy * _e55.g0_.xxwz) * vec4<f32>(0.0, 1.0, -(1.0), 1.0))));
}

fn motor_motor_dual_left_contraction(self_2082: Motor, other_1832: MotorDual) -> MotorDual {
    var self_2083: Motor;
    var other_1833: MotorDual;

    self_2083 = self_2082;
    other_1833 = other_1832;
    let _e4: Motor = self_2083;
    let _e8: MotorDual = other_1833;
    let _e11: Motor = self_2083;
    let _e13: MotorDual = other_1833;
    return MotorDual(((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((_e11.g0_ * vec4<f32>(_e13.g0_.x)) * vec4<f32>(0.0, -(1.0), -(1.0), -(1.0)))));
}

fn motor_motor_dual_right_contraction(self_2084: Motor, other_1834: MotorDual) -> Plane {
    var self_2085: Motor;
    var other_1835: MotorDual;

    self_2085 = self_2084;
    other_1835 = other_1834;
    let _e4: Motor = self_2085;
    let _e8: MotorDual = other_1835;
    let _e11: MotorDual = other_1835;
    let _e14: MotorDual = other_1835;
    let _e25: Motor = self_2085;
    let _e29: MotorDual = other_1835;
    let _e32: MotorDual = other_1835;
    let _e35: MotorDual = other_1835;
    let _e47: Motor = self_2085;
    let _e50: Motor = self_2085;
    let _e53: Motor = self_2085;
    let _e57: MotorDual = other_1835;
    let _e60: MotorDual = other_1835;
    let _e63: MotorDual = other_1835;
    return Plane(((((vec3<f32>(_e4.g0_.z) * vec3<f32>(_e8.g0_.w, _e11.g0_.w, _e14.g0_.y)) * vec3<f32>(1.0, 0.0, -(1.0))) + ((vec3<f32>(_e25.g0_.w) * vec3<f32>(_e29.g0_.z, _e32.g0_.y, _e35.g0_.z)) * vec3<f32>(-(1.0), 1.0, 0.0))) + ((vec3<f32>(_e47.g0_.x, _e50.g0_.y, _e53.g0_.y) * vec3<f32>(_e57.g0_.x, _e60.g0_.w, _e63.g0_.z)) * vec3<f32>(0.0, -(1.0), 1.0))));
}

fn motor_motor_dual_left_anti_contraction(self_2086: Motor, other_1836: MotorDual) -> Point {
    var self_2087: Motor;
    var other_1837: MotorDual;

    self_2087 = self_2086;
    other_1837 = other_1836;
    let _e4: Motor = self_2087;
    let _e8: MotorDual = other_1837;
    let _e11: MotorDual = other_1837;
    let _e14: MotorDual = other_1837;
    let _e25: Motor = self_2087;
    let _e29: MotorDual = other_1837;
    let _e32: MotorDual = other_1837;
    let _e35: MotorDual = other_1837;
    let _e47: Motor = self_2087;
    let _e50: Motor = self_2087;
    let _e53: Motor = self_2087;
    let _e57: MotorDual = other_1837;
    let _e60: MotorDual = other_1837;
    let _e63: MotorDual = other_1837;
    return Point(((((vec3<f32>(_e4.g0_.z) * vec3<f32>(_e8.g0_.w, _e11.g0_.w, _e14.g0_.y)) * vec3<f32>(1.0, 0.0, -(1.0))) + ((vec3<f32>(_e25.g0_.w) * vec3<f32>(_e29.g0_.z, _e32.g0_.y, _e35.g0_.z)) * vec3<f32>(-(1.0), 1.0, 0.0))) + ((vec3<f32>(_e47.g0_.x, _e50.g0_.y, _e53.g0_.y) * vec3<f32>(_e57.g0_.x, _e60.g0_.w, _e63.g0_.z)) * vec3<f32>(0.0, -(1.0), 1.0))));
}

fn motor_motor_dual_right_anti_contraction(self_2088: Motor, other_1838: MotorDual) -> Motor {
    var self_2089: Motor;
    var other_1839: MotorDual;

    self_2089 = self_2088;
    other_1839 = other_1838;
    let _e4: Motor = self_2089;
    let _e8: MotorDual = other_1839;
    let _e20: Motor = self_2089;
    let _e22: MotorDual = other_1839;
    return Motor((((vec4<f32>(_e4.g0_.x) * _e8.g0_) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))) + ((_e20.g0_ * vec4<f32>(_e22.g0_.x)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn motor_squared_magnitude(self_2090: Motor) -> Scalar {
    var self_2091: Motor;

    self_2091 = self_2090;
    let _e2: Motor = self_2091;
    let _e3: Motor = self_2091;
    let _e4: Motor = motor_reversal(_e3);
    let _e5: Scalar = motor_motor_scalar_product(_e2, _e4);
    return _e5;
}

fn motor_magnitude(self_2092: Motor) -> Scalar {
    var self_2093: Motor;

    self_2093 = self_2092;
    let _e2: Motor = self_2093;
    let _e3: Scalar = motor_squared_magnitude(_e2);
    return Scalar(sqrt(_e3.g0_));
}

fn motor_bulk_norm(self_2094: Motor) -> Scalar {
    var self_2095: Motor;

    self_2095 = self_2094;
    let _e2: Motor = self_2095;
    let _e3: Scalar = motor_squared_magnitude(_e2);
    return Scalar(sqrt(_e3.g0_));
}

fn motor_squared_anti_magnitude(self_2096: Motor) -> AntiScalar {
    var self_2097: Motor;

    self_2097 = self_2096;
    let _e2: Motor = self_2097;
    let _e3: Motor = self_2097;
    let _e4: Motor = motor_anti_reversal(_e3);
    let _e5: AntiScalar = motor_motor_anti_scalar_product(_e2, _e4);
    return _e5;
}

fn motor_weight_norm(self_2098: Motor) -> AntiScalar {
    var self_2099: Motor;

    self_2099 = self_2098;
    let _e2: Motor = self_2099;
    let _e3: AntiScalar = motor_squared_anti_magnitude(_e2);
    return AntiScalar(sqrt(_e3.g0_));
}

fn motor_scale(self_2100: Motor, other_1840: f32) -> Motor {
    var self_2101: Motor;
    var other_1841: f32;

    self_2101 = self_2100;
    other_1841 = other_1840;
    let _e4: Motor = self_2101;
    let _e5: f32 = other_1841;
    let _e7: Motor = motor_scalar_geometric_product(_e4, Scalar(_e5));
    return _e7;
}

fn motor_signum(self_2102: Motor) -> Motor {
    var self_2103: Motor;

    self_2103 = self_2102;
    let _e2: Motor = self_2103;
    let _e3: Motor = self_2103;
    let _e4: Scalar = motor_magnitude(_e3);
    let _e9: Motor = motor_scalar_geometric_product(_e2, Scalar((1.0 / _e4.g0_)));
    return _e9;
}

fn motor_inverse(self_2104: Motor) -> Motor {
    var self_2105: Motor;

    self_2105 = self_2104;
    let _e2: Motor = self_2105;
    let _e3: Motor = motor_reversal(_e2);
    let _e4: Motor = self_2105;
    let _e5: Scalar = motor_squared_magnitude(_e4);
    let _e10: Motor = motor_scalar_geometric_product(_e3, Scalar((1.0 / _e5.g0_)));
    return _e10;
}

fn motor_dual_zero() -> MotorDual {
    return MotorDual(vec4<f32>(0.0));
}

fn motor_dual_one() -> MotorDual {
    return MotorDual(vec4<f32>(0.0));
}

fn motor_dual_neg(self_2106: MotorDual) -> MotorDual {
    var self_2107: MotorDual;

    self_2107 = self_2106;
    let _e2: MotorDual = self_2107;
    return MotorDual((_e2.g0_ * vec4<f32>(-(1.0))));
}

fn motor_dual_automorphism(self_2108: MotorDual) -> MotorDual {
    var self_2109: MotorDual;

    self_2109 = self_2108;
    let _e2: MotorDual = self_2109;
    return MotorDual((_e2.g0_ * vec4<f32>(-(1.0))));
}

fn motor_dual_reversal(self_2110: MotorDual) -> MotorDual {
    var self_2111: MotorDual;

    self_2111 = self_2110;
    let _e2: MotorDual = self_2111;
    return MotorDual((_e2.g0_ * vec4<f32>(-(1.0), 1.0, 1.0, 1.0)));
}

fn motor_dual_conjugation(self_2112: MotorDual) -> MotorDual {
    var self_2113: MotorDual;

    self_2113 = self_2112;
    let _e2: MotorDual = self_2113;
    return MotorDual((_e2.g0_ * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))));
}

fn motor_dual_dual(self_2114: MotorDual) -> Motor {
    var self_2115: MotorDual;

    self_2115 = self_2114;
    let _e2: MotorDual = self_2115;
    return Motor(_e2.g0_);
}

fn motor_dual_anti_reversal(self_2116: MotorDual) -> MotorDual {
    var self_2117: MotorDual;

    self_2117 = self_2116;
    let _e2: MotorDual = self_2117;
    return MotorDual((_e2.g0_ * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))));
}

fn motor_dual_scalar_geometric_product(self_2118: MotorDual, other_1842: Scalar) -> MotorDual {
    var self_2119: MotorDual;
    var other_1843: Scalar;

    self_2119 = self_2118;
    other_1843 = other_1842;
    let _e4: MotorDual = self_2119;
    let _e6: Scalar = other_1843;
    return MotorDual((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn motor_dual_scalar_regressive_product(self_2120: MotorDual, other_1844: Scalar) -> Scalar {
    var self_2121: MotorDual;
    var other_1845: Scalar;

    self_2121 = self_2120;
    other_1845 = other_1844;
    let _e4: MotorDual = self_2121;
    let _e7: Scalar = other_1845;
    return Scalar((_e4.g0_.x * _e7.g0_));
}

fn motor_dual_scalar_outer_product(self_2122: MotorDual, other_1846: Scalar) -> MotorDual {
    var self_2123: MotorDual;
    var other_1847: Scalar;

    self_2123 = self_2122;
    other_1847 = other_1846;
    let _e4: MotorDual = self_2123;
    let _e6: Scalar = other_1847;
    return MotorDual((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn motor_dual_scalar_inner_product(self_2124: MotorDual, other_1848: Scalar) -> MotorDual {
    var self_2125: MotorDual;
    var other_1849: Scalar;

    self_2125 = self_2124;
    other_1849 = other_1848;
    let _e4: MotorDual = self_2125;
    let _e6: Scalar = other_1849;
    return MotorDual((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn motor_dual_scalar_geometric_anti_product(self_2126: MotorDual, other_1850: Scalar) -> Motor {
    var self_2127: MotorDual;
    var other_1851: Scalar;

    self_2127 = self_2126;
    other_1851 = other_1850;
    let _e4: MotorDual = self_2127;
    let _e6: Scalar = other_1851;
    return Motor(((_e4.g0_ * vec4<f32>(_e6.g0_)) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))));
}

fn motor_dual_scalar_inner_anti_product(self_2128: MotorDual, other_1852: Scalar) -> Motor {
    var self_2129: MotorDual;
    var other_1853: Scalar;

    self_2129 = self_2128;
    other_1853 = other_1852;
    let _e4: MotorDual = self_2129;
    let _e6: Scalar = other_1853;
    return Motor(((_e4.g0_ * vec4<f32>(_e6.g0_)) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))));
}

fn motor_dual_scalar_right_contraction(self_2130: MotorDual, other_1854: Scalar) -> MotorDual {
    var self_2131: MotorDual;
    var other_1855: Scalar;

    self_2131 = self_2130;
    other_1855 = other_1854;
    let _e4: MotorDual = self_2131;
    let _e6: Scalar = other_1855;
    return MotorDual((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn motor_dual_scalar_left_anti_contraction(self_2132: MotorDual, other_1856: Scalar) -> Motor {
    var self_2133: MotorDual;
    var other_1857: Scalar;

    self_2133 = self_2132;
    other_1857 = other_1856;
    let _e4: MotorDual = self_2133;
    let _e6: Scalar = other_1857;
    return Motor(((_e4.g0_ * vec4<f32>(_e6.g0_)) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))));
}

fn motor_dual_anti_scalar_into(self_2134: MotorDual) -> AntiScalar {
    var self_2135: MotorDual;

    self_2135 = self_2134;
    let _e2: MotorDual = self_2135;
    return AntiScalar(_e2.g0_.x);
}

fn motor_dual_anti_scalar_add(self_2136: MotorDual, other_1858: AntiScalar) -> MotorDual {
    var self_2137: MotorDual;
    var other_1859: AntiScalar;

    self_2137 = self_2136;
    other_1859 = other_1858;
    let _e4: MotorDual = self_2137;
    let _e6: AntiScalar = other_1859;
    return MotorDual((_e4.g0_ + (vec4<f32>(_e6.g0_) * vec4<f32>(1.0, 0.0, 0.0, 0.0))));
}

fn motor_dual_anti_scalar_sub(self_2138: MotorDual, other_1860: AntiScalar) -> MotorDual {
    var self_2139: MotorDual;
    var other_1861: AntiScalar;

    self_2139 = self_2138;
    other_1861 = other_1860;
    let _e4: MotorDual = self_2139;
    let _e6: AntiScalar = other_1861;
    return MotorDual((_e4.g0_ - (vec4<f32>(_e6.g0_) * vec4<f32>(1.0, 0.0, 0.0, 0.0))));
}

fn motor_dual_anti_scalar_geometric_product(self_2140: MotorDual, other_1862: AntiScalar) -> Motor {
    var self_2141: MotorDual;
    var other_1863: AntiScalar;

    self_2141 = self_2140;
    other_1863 = other_1862;
    let _e4: MotorDual = self_2141;
    let _e6: AntiScalar = other_1863;
    return Motor(((_e4.g0_ * vec4<f32>(_e6.g0_)) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0)));
}

fn motor_dual_anti_scalar_regressive_product(self_2142: MotorDual, other_1864: AntiScalar) -> MotorDual {
    var self_2143: MotorDual;
    var other_1865: AntiScalar;

    self_2143 = self_2142;
    other_1865 = other_1864;
    let _e4: MotorDual = self_2143;
    let _e6: AntiScalar = other_1865;
    return MotorDual((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn motor_dual_anti_scalar_inner_product(self_2144: MotorDual, other_1866: AntiScalar) -> Motor {
    var self_2145: MotorDual;
    var other_1867: AntiScalar;

    self_2145 = self_2144;
    other_1867 = other_1866;
    let _e4: MotorDual = self_2145;
    let _e6: AntiScalar = other_1867;
    return Motor(((_e4.g0_ * vec4<f32>(_e6.g0_)) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0)));
}

fn motor_dual_anti_scalar_geometric_anti_product(self_2146: MotorDual, other_1868: AntiScalar) -> MotorDual {
    var self_2147: MotorDual;
    var other_1869: AntiScalar;

    self_2147 = self_2146;
    other_1869 = other_1868;
    let _e4: MotorDual = self_2147;
    let _e6: AntiScalar = other_1869;
    return MotorDual((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn motor_dual_anti_scalar_inner_anti_product(self_2148: MotorDual, other_1870: AntiScalar) -> MotorDual {
    var self_2149: MotorDual;
    var other_1871: AntiScalar;

    self_2149 = self_2148;
    other_1871 = other_1870;
    let _e4: MotorDual = self_2149;
    let _e6: AntiScalar = other_1871;
    return MotorDual((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn motor_dual_anti_scalar_left_contraction(self_2150: MotorDual, other_1872: AntiScalar) -> Motor {
    var self_2151: MotorDual;
    var other_1873: AntiScalar;

    self_2151 = self_2150;
    other_1873 = other_1872;
    let _e4: MotorDual = self_2151;
    let _e6: AntiScalar = other_1873;
    return Motor(((_e4.g0_ * vec4<f32>(_e6.g0_)) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0)));
}

fn motor_dual_anti_scalar_right_contraction(self_2152: MotorDual, other_1874: AntiScalar) -> Scalar {
    var self_2153: MotorDual;
    var other_1875: AntiScalar;

    self_2153 = self_2152;
    other_1875 = other_1874;
    let _e5: MotorDual = self_2153;
    let _e8: AntiScalar = other_1875;
    return Scalar((0.0 - (_e5.g0_.x * _e8.g0_)));
}

fn motor_dual_anti_scalar_left_anti_contraction(self_2154: MotorDual, other_1876: AntiScalar) -> AntiScalar {
    var self_2155: MotorDual;
    var other_1877: AntiScalar;

    self_2155 = self_2154;
    other_1877 = other_1876;
    let _e4: MotorDual = self_2155;
    let _e7: AntiScalar = other_1877;
    return AntiScalar((_e4.g0_.x * _e7.g0_));
}

fn motor_dual_anti_scalar_right_anti_contraction(self_2156: MotorDual, other_1878: AntiScalar) -> MotorDual {
    var self_2157: MotorDual;
    var other_1879: AntiScalar;

    self_2157 = self_2156;
    other_1879 = other_1878;
    let _e4: MotorDual = self_2157;
    let _e6: AntiScalar = other_1879;
    return MotorDual((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn motor_dual_anti_scalar_scalar_product(self_2158: MotorDual, other_1880: AntiScalar) -> Scalar {
    var self_2159: MotorDual;
    var other_1881: AntiScalar;

    self_2159 = self_2158;
    other_1881 = other_1880;
    let _e5: MotorDual = self_2159;
    let _e8: AntiScalar = other_1881;
    return Scalar((0.0 - (_e5.g0_.x * _e8.g0_)));
}

fn motor_dual_anti_scalar_anti_scalar_product(self_2160: MotorDual, other_1882: AntiScalar) -> AntiScalar {
    var self_2161: MotorDual;
    var other_1883: AntiScalar;

    self_2161 = self_2160;
    other_1883 = other_1882;
    let _e4: MotorDual = self_2161;
    let _e7: AntiScalar = other_1883;
    return AntiScalar((_e4.g0_.x * _e7.g0_));
}

fn motor_dual_multi_vector_add(self_2162: MotorDual, other_1884: MultiVector) -> MultiVector {
    var self_2163: MotorDual;
    var other_1885: MultiVector;

    self_2163 = self_2162;
    other_1885 = other_1884;
    let _e4: MotorDual = self_2163;
    let _e13: MultiVector = other_1885;
    let _e16: MotorDual = self_2163;
    let _e25: MultiVector = other_1885;
    return MultiVector(((_e4.g0_.xxwz * vec4<f32>(0.0, 0.0, 1.0, 1.0)) + _e13.g0_), ((_e16.g0_.yxxx * vec4<f32>(1.0, 1.0, 0.0, 0.0)) + _e25.g1_));
}

fn motor_dual_multi_vector_sub(self_2164: MotorDual, other_1886: MultiVector) -> MultiVector {
    var self_2165: MotorDual;
    var other_1887: MultiVector;

    self_2165 = self_2164;
    other_1887 = other_1886;
    let _e4: MotorDual = self_2165;
    let _e13: MultiVector = other_1887;
    let _e16: MotorDual = self_2165;
    let _e25: MultiVector = other_1887;
    return MultiVector(((_e4.g0_.xxwz * vec4<f32>(0.0, 0.0, 1.0, 1.0)) - _e13.g0_), ((_e16.g0_.yxxx * vec4<f32>(1.0, 1.0, 0.0, 0.0)) - _e25.g1_));
}

fn motor_dual_multi_vector_geometric_product(self_2166: MotorDual, other_1888: MultiVector) -> MultiVector {
    var self_2167: MotorDual;
    var other_1889: MultiVector;

    self_2167 = self_2166;
    other_1889 = other_1888;
    let _e4: MotorDual = self_2167;
    let _e8: MultiVector = other_1889;
    let _e21: MotorDual = self_2167;
    let _e25: MultiVector = other_1889;
    let _e36: MotorDual = self_2167;
    let _e40: MultiVector = other_1889;
    let _e53: MotorDual = self_2167;
    let _e57: MultiVector = other_1889;
    let _e62: MotorDual = self_2167;
    let _e66: MultiVector = other_1889;
    let _e77: MotorDual = self_2167;
    let _e81: MultiVector = other_1889;
    let _e92: MotorDual = self_2167;
    let _e96: MultiVector = other_1889;
    let _e101: MotorDual = self_2167;
    let _e105: MultiVector = other_1889;
    return MultiVector((((((vec4<f32>(_e4.g0_.x) * _e8.g1_.yxwz) * vec4<f32>(-(1.0), 1.0, -(1.0), -(1.0))) + ((vec4<f32>(_e21.g0_.y) * _e25.g1_) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e36.g0_.z) * _e40.g0_.wzyx) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0))) + (vec4<f32>(_e53.g0_.w) * _e57.g0_.zwxy)), (((((vec4<f32>(_e62.g0_.x) * _e66.g0_.yxwz) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0)) + ((vec4<f32>(_e77.g0_.y) * _e81.g0_) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))) + (vec4<f32>(_e92.g0_.z) * _e96.g1_.wzyx)) + ((vec4<f32>(_e101.g0_.w) * _e105.g1_.zwxy) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))));
}

fn motor_dual_multi_vector_regressive_product(self_2168: MotorDual, other_1890: MultiVector) -> MultiVector {
    var self_2169: MotorDual;
    var other_1891: MultiVector;

    self_2169 = self_2168;
    other_1891 = other_1890;
    let _e4: MotorDual = self_2169;
    let _e8: MultiVector = other_1891;
    let _e11: MotorDual = self_2169;
    let _e15: MultiVector = other_1891;
    let _e26: MotorDual = self_2169;
    let _e30: MultiVector = other_1891;
    let _e41: MotorDual = self_2169;
    let _e44: MultiVector = other_1891;
    let _e55: MotorDual = self_2169;
    let _e59: MultiVector = other_1891;
    let _e62: MotorDual = self_2169;
    let _e65: MultiVector = other_1891;
    return MultiVector(((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.z) * _e15.g1_.zzzy) * vec4<f32>(1.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e26.g0_.w) * _e30.g1_.wwyw) * vec4<f32>(1.0, 0.0, 1.0, 0.0))) + ((_e41.g0_.yxxx * _e44.g0_.yxxx) * vec4<f32>(1.0, 0.0, 0.0, 0.0))), ((vec4<f32>(_e55.g0_.x) * _e59.g1_) + ((_e62.g0_.yxxx * _e65.g1_.yxxx) * vec4<f32>(1.0, 0.0, 0.0, 0.0))));
}

fn motor_dual_multi_vector_inner_product(self_2170: MotorDual, other_1892: MultiVector) -> MultiVector {
    var self_2171: MotorDual;
    var other_1893: MultiVector;

    self_2171 = self_2170;
    other_1893 = other_1892;
    let _e4: MotorDual = self_2171;
    let _e8: MultiVector = other_1893;
    let _e21: MotorDual = self_2171;
    let _e25: MultiVector = other_1893;
    let _e36: MotorDual = self_2171;
    let _e40: MultiVector = other_1893;
    let _e51: MotorDual = self_2171;
    let _e54: MultiVector = other_1893;
    let _e66: MotorDual = self_2171;
    let _e70: MultiVector = other_1893;
    let _e81: MotorDual = self_2171;
    let _e85: MultiVector = other_1893;
    let _e96: MotorDual = self_2171;
    let _e100: MultiVector = other_1893;
    let _e112: MotorDual = self_2171;
    let _e115: MultiVector = other_1893;
    return MultiVector((((((vec4<f32>(_e4.g0_.x) * _e8.g1_.yxwz) * vec4<f32>(-(1.0), 1.0, -(1.0), -(1.0))) + ((vec4<f32>(_e21.g0_.y) * _e25.g1_) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e36.g0_.w) * _e40.g0_.zzxy) * vec4<f32>(1.0, 0.0, 1.0, 1.0))) + ((_e51.g0_.zxzz * _e54.g0_.wxyx) * vec4<f32>(1.0, 0.0, -(1.0), 1.0))), (((((vec4<f32>(_e66.g0_.x) * _e70.g0_.yxwz) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0)) + ((vec4<f32>(_e81.g0_.z) * _e85.g1_.wwyw) * vec4<f32>(1.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e96.g0_.w) * _e100.g1_.zzzy) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))) + ((_e112.g0_.yxxx * vec4<f32>(_e115.g0_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))));
}

fn motor_dual_multi_vector_geometric_anti_product(self_2172: MotorDual, other_1894: MultiVector) -> MultiVector {
    var self_2173: MotorDual;
    var other_1895: MultiVector;

    self_2173 = self_2172;
    other_1895 = other_1894;
    let _e4: MotorDual = self_2173;
    let _e8: MultiVector = other_1895;
    let _e11: MotorDual = self_2173;
    let _e15: MultiVector = other_1895;
    let _e28: MotorDual = self_2173;
    let _e32: MultiVector = other_1895;
    let _e44: MotorDual = self_2173;
    let _e48: MultiVector = other_1895;
    let _e60: MotorDual = self_2173;
    let _e64: MultiVector = other_1895;
    let _e67: MotorDual = self_2173;
    let _e71: MultiVector = other_1895;
    let _e84: MotorDual = self_2173;
    let _e88: MultiVector = other_1895;
    let _e102: MotorDual = self_2173;
    let _e106: MultiVector = other_1895;
    return MultiVector(((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.y) * _e15.g0_.yxwz) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e28.g0_.z) * _e32.g1_.zwxy) * vec4<f32>(1.0, 1.0, -(1.0), 1.0))) + ((vec4<f32>(_e44.g0_.w) * _e48.g1_.wzyx) * vec4<f32>(1.0, -(1.0), 1.0, 1.0))), ((((vec4<f32>(_e60.g0_.x) * _e64.g1_) + ((vec4<f32>(_e67.g0_.y) * _e71.g1_.yxwz) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e84.g0_.z) * _e88.g0_.zwxy) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0)))) + ((vec4<f32>(_e102.g0_.w) * _e106.g0_.wzyx) * vec4<f32>(-(1.0), -(1.0), 1.0, -(1.0)))));
}

fn motor_dual_multi_vector_inner_anti_product(self_2174: MotorDual, other_1896: MultiVector) -> MultiVector {
    var self_2175: MotorDual;
    var other_1897: MultiVector;

    self_2175 = self_2174;
    other_1897 = other_1896;
    let _e4: MotorDual = self_2175;
    let _e8: MultiVector = other_1897;
    let _e11: MotorDual = self_2175;
    let _e15: MultiVector = other_1897;
    let _e26: MotorDual = self_2175;
    let _e30: MultiVector = other_1897;
    let _e42: MotorDual = self_2175;
    let _e45: MultiVector = other_1897;
    let _e58: MotorDual = self_2175;
    let _e62: MultiVector = other_1897;
    let _e65: MotorDual = self_2175;
    let _e69: MultiVector = other_1897;
    let _e82: MotorDual = self_2175;
    let _e86: MultiVector = other_1897;
    let _e99: MotorDual = self_2175;
    let _e102: MultiVector = other_1897;
    return MultiVector(((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.z) * _e15.g1_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, 1.0))) + ((vec4<f32>(_e26.g0_.w) * _e30.g1_.zzyz) * vec4<f32>(0.0, -(1.0), 1.0, 0.0))) + ((_e42.g0_.xyxx * vec4<f32>(_e45.g0_.x)) * vec4<f32>(0.0, -(1.0), 0.0, 0.0))), ((((vec4<f32>(_e58.g0_.x) * _e62.g1_) + ((vec4<f32>(_e65.g0_.y) * _e69.g1_.yxwz) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e82.g0_.w) * _e86.g0_.zzyx) * vec4<f32>(0.0, -(1.0), 1.0, -(1.0)))) + ((_e99.g0_.xzzz * _e102.g0_.xwxy) * vec4<f32>(0.0, -(1.0), -(1.0), -(1.0)))));
}

fn motor_dual_multi_vector_right_contraction(self_2176: MotorDual, other_1898: MultiVector) -> MultiVector {
    var self_2177: MotorDual;
    var other_1899: MultiVector;

    self_2177 = self_2176;
    other_1899 = other_1898;
    let _e4: MotorDual = self_2177;
    let _e8: MultiVector = other_1899;
    let _e21: MotorDual = self_2177;
    let _e25: MultiVector = other_1899;
    let _e36: MotorDual = self_2177;
    let _e40: MultiVector = other_1899;
    let _e51: MotorDual = self_2177;
    let _e54: MultiVector = other_1899;
    let _e66: MotorDual = self_2177;
    let _e70: MultiVector = other_1899;
    let _e81: MotorDual = self_2177;
    let _e84: MultiVector = other_1899;
    return MultiVector((((((vec4<f32>(_e4.g0_.x) * _e8.g1_.yxwz) * vec4<f32>(-(1.0), 1.0, -(1.0), -(1.0))) + ((vec4<f32>(_e21.g0_.z) * _e25.g0_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e36.g0_.w) * _e40.g0_.zzxz) * vec4<f32>(1.0, 0.0, 1.0, 0.0))) + ((_e51.g0_.yxxx * vec4<f32>(_e54.g1_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))), (((vec4<f32>(_e66.g0_.x) * _e70.g0_.yxwz) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0)) + ((_e81.g0_.yxxx * vec4<f32>(_e84.g0_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))));
}

fn motor_dual_multi_vector_left_anti_contraction(self_2178: MotorDual, other_1900: MultiVector) -> MultiVector {
    var self_2179: MotorDual;
    var other_1901: MultiVector;

    self_2179 = self_2178;
    other_1901 = other_1900;
    let _e4: MotorDual = self_2179;
    let _e8: MultiVector = other_1901;
    let _e11: MotorDual = self_2179;
    let _e14: MultiVector = other_1901;
    let _e27: MotorDual = self_2179;
    let _e31: MultiVector = other_1901;
    let _e34: MotorDual = self_2179;
    let _e38: MultiVector = other_1901;
    let _e51: MotorDual = self_2179;
    let _e55: MultiVector = other_1901;
    let _e68: MotorDual = self_2179;
    let _e71: MultiVector = other_1901;
    return MultiVector(((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((_e11.g0_.xyxx * vec4<f32>(_e14.g0_.x)) * vec4<f32>(0.0, -(1.0), 0.0, 0.0))), ((((vec4<f32>(_e27.g0_.x) * _e31.g1_) + ((vec4<f32>(_e34.g0_.z) * _e38.g0_.wwxw) * vec4<f32>(0.0, -(1.0), -(1.0), 0.0))) + ((vec4<f32>(_e51.g0_.w) * _e55.g0_.zzzx) * vec4<f32>(0.0, -(1.0), 0.0, -(1.0)))) + ((_e68.g0_.xyxx * vec4<f32>(_e71.g1_.x)) * vec4<f32>(0.0, -(1.0), 0.0, 0.0))));
}

fn motor_dual_multi_vector_scalar_product(self_2180: MotorDual, other_1902: MultiVector) -> Scalar {
    var self_2181: MotorDual;
    var other_1903: MultiVector;

    self_2181 = self_2180;
    other_1903 = other_1902;
    let _e5: MotorDual = self_2181;
    let _e8: MultiVector = other_1903;
    let _e13: MotorDual = self_2181;
    let _e16: MultiVector = other_1903;
    let _e21: MotorDual = self_2181;
    let _e24: MultiVector = other_1903;
    let _e29: MotorDual = self_2181;
    let _e32: MultiVector = other_1903;
    return Scalar(((((0.0 - (_e5.g0_.x * _e8.g1_.y)) + (_e13.g0_.y * _e16.g1_.x)) + (_e21.g0_.z * _e24.g0_.w)) + (_e29.g0_.w * _e32.g0_.z)));
}

fn motor_dual_multi_vector_anti_scalar_product(self_2182: MotorDual, other_1904: MultiVector) -> AntiScalar {
    var self_2183: MotorDual;
    var other_1905: MultiVector;

    self_2183 = self_2182;
    other_1905 = other_1904;
    let _e4: MotorDual = self_2183;
    let _e7: MultiVector = other_1905;
    let _e11: MotorDual = self_2183;
    let _e14: MultiVector = other_1905;
    let _e19: MotorDual = self_2183;
    let _e22: MultiVector = other_1905;
    let _e27: MotorDual = self_2183;
    let _e30: MultiVector = other_1905;
    return AntiScalar(((((_e4.g0_.x * _e7.g1_.y) - (_e11.g0_.y * _e14.g1_.x)) - (_e19.g0_.z * _e22.g0_.w)) - (_e27.g0_.w * _e30.g0_.z)));
}

fn motor_dual_rotor_geometric_product(self_2184: MotorDual, other_1906: Rotor) -> MotorDual {
    var self_2185: MotorDual;
    var other_1907: Rotor;

    self_2185 = self_2184;
    other_1907 = other_1906;
    let _e4: MotorDual = self_2185;
    let _e8: Rotor = other_1907;
    let _e11: Rotor = other_1907;
    let _e14: Rotor = other_1907;
    let _e17: Rotor = other_1907;
    let _e28: MotorDual = self_2185;
    let _e32: Rotor = other_1907;
    let _e35: Rotor = other_1907;
    let _e38: Rotor = other_1907;
    let _e41: Rotor = other_1907;
    let _e53: MotorDual = self_2185;
    let _e56: Rotor = other_1907;
    let _e59: Rotor = other_1907;
    let _e62: Rotor = other_1907;
    let _e65: Rotor = other_1907;
    return MotorDual(((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y, _e11.g0_.x, _e14.g0_.y, _e17.g0_.y)) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) + ((vec4<f32>(_e28.g0_.w) * vec4<f32>(_e32.g0_.y, _e35.g0_.y, _e38.g0_.y, _e41.g0_.x)) * vec4<f32>(0.0, 0.0, 1.0, 1.0))) + ((_e53.g0_.xxzz * vec4<f32>(_e56.g0_.x, _e59.g0_.y, _e62.g0_.x, _e65.g0_.y)) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))));
}

fn motor_dual_rotor_regressive_product(self_2186: MotorDual, other_1908: Rotor) -> Rotor {
    var self_2187: MotorDual;
    var other_1909: Rotor;

    self_2187 = self_2186;
    other_1909 = other_1908;
    let _e4: MotorDual = self_2187;
    let _e8: Rotor = other_1909;
    let _e11: MotorDual = self_2187;
    let _e14: MotorDual = self_2187;
    let _e18: Rotor = other_1909;
    return Rotor(((vec2<f32>(_e4.g0_.x) * _e8.g0_) + ((vec2<f32>(_e11.g0_.y, _e14.g0_.x) * _e18.g0_.yx) * vec2<f32>(1.0, 0.0))));
}

fn motor_dual_rotor_outer_product(self_2188: MotorDual, other_1910: Rotor) -> MotorDual {
    var self_2189: MotorDual;
    var other_1911: Rotor;

    self_2189 = self_2188;
    other_1911 = other_1910;
    let _e4: MotorDual = self_2189;
    let _e8: Rotor = other_1911;
    let _e11: Rotor = other_1911;
    let _e14: Rotor = other_1911;
    let _e17: Rotor = other_1911;
    let _e28: MotorDual = self_2189;
    let _e31: Rotor = other_1911;
    return MotorDual((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y, _e11.g0_.x, _e14.g0_.y, _e17.g0_.y)) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) + ((_e28.g0_.xxzw * vec4<f32>(_e31.g0_.x)) * vec4<f32>(1.0, 0.0, 1.0, 1.0))));
}

fn motor_dual_rotor_inner_product(self_2190: MotorDual, other_1912: Rotor) -> MotorDual {
    var self_2191: MotorDual;
    var other_1913: Rotor;

    self_2191 = self_2190;
    other_1913 = other_1912;
    let _e4: MotorDual = self_2191;
    let _e8: Rotor = other_1913;
    let _e19: MotorDual = self_2191;
    let _e23: Rotor = other_1913;
    let _e26: Rotor = other_1913;
    let _e29: Rotor = other_1913;
    let _e32: Rotor = other_1913;
    let _e44: MotorDual = self_2191;
    let _e47: Rotor = other_1913;
    let _e50: Rotor = other_1913;
    let _e53: Rotor = other_1913;
    let _e56: Rotor = other_1913;
    return MotorDual(((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.x)) * vec4<f32>(0.0, 1.0, 0.0, 0.0)) + ((vec4<f32>(_e19.g0_.w) * vec4<f32>(_e23.g0_.y, _e26.g0_.y, _e29.g0_.y, _e32.g0_.x)) * vec4<f32>(0.0, 0.0, 1.0, 1.0))) + ((_e44.g0_.xxzz * vec4<f32>(_e47.g0_.x, _e50.g0_.y, _e53.g0_.x, _e56.g0_.y)) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))));
}

fn motor_dual_rotor_geometric_anti_product(self_2192: MotorDual, other_1914: Rotor) -> Motor {
    var self_2193: MotorDual;
    var other_1915: Rotor;

    self_2193 = self_2192;
    other_1915 = other_1914;
    let _e4: MotorDual = self_2193;
    let _e8: Rotor = other_1915;
    let _e11: Rotor = other_1915;
    let _e14: Rotor = other_1915;
    let _e17: Rotor = other_1915;
    let _e29: MotorDual = self_2193;
    let _e33: Rotor = other_1915;
    let _e36: Rotor = other_1915;
    let _e39: Rotor = other_1915;
    let _e42: Rotor = other_1915;
    let _e55: MotorDual = self_2193;
    let _e58: Rotor = other_1915;
    let _e61: Rotor = other_1915;
    let _e64: Rotor = other_1915;
    let _e67: Rotor = other_1915;
    return Motor(((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y, _e11.g0_.x, _e14.g0_.y, _e17.g0_.y)) * vec4<f32>(1.0, -(1.0), 0.0, 0.0)) + ((vec4<f32>(_e29.g0_.w) * vec4<f32>(_e33.g0_.y, _e36.g0_.y, _e39.g0_.y, _e42.g0_.x)) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((_e55.g0_.xxzz * vec4<f32>(_e58.g0_.x, _e61.g0_.y, _e64.g0_.x, _e67.g0_.y)) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))));
}

fn motor_dual_rotor_inner_anti_product(self_2194: MotorDual, other_1916: Rotor) -> Motor {
    var self_2195: MotorDual;
    var other_1917: Rotor;

    self_2195 = self_2194;
    other_1917 = other_1916;
    let _e4: MotorDual = self_2195;
    let _e8: Rotor = other_1917;
    let _e20: MotorDual = self_2195;
    let _e24: Rotor = other_1917;
    let _e27: Rotor = other_1917;
    let _e30: Rotor = other_1917;
    let _e33: Rotor = other_1917;
    let _e46: MotorDual = self_2195;
    let _e49: Rotor = other_1917;
    let _e52: Rotor = other_1917;
    let _e55: Rotor = other_1917;
    let _e58: Rotor = other_1917;
    return Motor(((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.x)) * vec4<f32>(0.0, -(1.0), 0.0, 0.0)) + ((vec4<f32>(_e20.g0_.w) * vec4<f32>(_e24.g0_.y, _e27.g0_.y, _e30.g0_.y, _e33.g0_.x)) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((_e46.g0_.xxzz * vec4<f32>(_e49.g0_.x, _e52.g0_.y, _e55.g0_.x, _e58.g0_.y)) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))));
}

fn motor_dual_rotor_right_contraction(self_2196: MotorDual, other_1918: Rotor) -> MotorDual {
    var self_2197: MotorDual;
    var other_1919: Rotor;

    self_2197 = self_2196;
    other_1919 = other_1918;
    let _e4: MotorDual = self_2197;
    let _e8: Rotor = other_1919;
    let _e19: MotorDual = self_2197;
    let _e22: Rotor = other_1919;
    let _e25: Rotor = other_1919;
    let _e28: Rotor = other_1919;
    let _e31: Rotor = other_1919;
    return MotorDual((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.x)) * vec4<f32>(0.0, 1.0, 0.0, 0.0)) + ((_e19.g0_.xxzw * vec4<f32>(_e22.g0_.x, _e25.g0_.y, _e28.g0_.x, _e31.g0_.x)) * vec4<f32>(1.0, -(1.0), 1.0, 1.0))));
}

fn motor_dual_rotor_left_anti_contraction(self_2198: MotorDual, other_1920: Rotor) -> Motor {
    var self_2199: MotorDual;
    var other_1921: Rotor;

    self_2199 = self_2198;
    other_1921 = other_1920;
    let _e4: MotorDual = self_2199;
    let _e8: Rotor = other_1921;
    let _e20: MotorDual = self_2199;
    let _e23: Rotor = other_1921;
    let _e26: Rotor = other_1921;
    let _e29: Rotor = other_1921;
    let _e32: Rotor = other_1921;
    return Motor((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.x)) * vec4<f32>(0.0, -(1.0), 0.0, 0.0)) + ((_e20.g0_.xxzw * vec4<f32>(_e23.g0_.x, _e26.g0_.y, _e29.g0_.x, _e32.g0_.x)) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))));
}

fn motor_dual_rotor_right_anti_contraction(self_2200: MotorDual, other_1922: Rotor) -> IdealPoint {
    var self_2201: MotorDual;
    var other_1923: Rotor;

    self_2201 = self_2200;
    other_1923 = other_1922;
    let _e4: MotorDual = self_2201;
    let _e7: MotorDual = self_2201;
    let _e11: Rotor = other_1923;
    return IdealPoint(((vec2<f32>(_e4.g0_.w, _e7.g0_.z) * vec2<f32>(_e11.g0_.y)) * vec2<f32>(1.0, -(1.0))));
}

fn motor_dual_point_geometric_product(self_2202: MotorDual, other_1924: Point) -> MotorDual {
    var self_2203: MotorDual;
    var other_1925: Point;

    self_2203 = self_2202;
    other_1925 = other_1924;
    let _e4: MotorDual = self_2203;
    let _e8: Point = other_1925;
    let _e11: Point = other_1925;
    let _e14: Point = other_1925;
    let _e17: Point = other_1925;
    let _e29: MotorDual = self_2203;
    let _e33: Point = other_1925;
    let _e36: Point = other_1925;
    let _e39: Point = other_1925;
    let _e42: Point = other_1925;
    let _e55: MotorDual = self_2203;
    let _e59: Point = other_1925;
    let _e62: Point = other_1925;
    let _e65: Point = other_1925;
    let _e68: Point = other_1925;
    let _e81: MotorDual = self_2203;
    let _e85: Point = other_1925;
    let _e88: Point = other_1925;
    let _e91: Point = other_1925;
    let _e94: Point = other_1925;
    return MotorDual((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.x, _e11.g0_.x, _e14.g0_.z, _e17.g0_.y)) * vec4<f32>(1.0, 0.0, -(1.0), 1.0)) + ((vec4<f32>(_e29.g0_.z) * vec4<f32>(_e33.g0_.y, _e36.g0_.z, _e39.g0_.y, _e42.g0_.x)) * vec4<f32>(1.0, 1.0, 0.0, -(1.0)))) + ((vec4<f32>(_e55.g0_.w) * vec4<f32>(_e59.g0_.z, _e62.g0_.y, _e65.g0_.x, _e68.g0_.z)) * vec4<f32>(1.0, -(1.0), 1.0, 0.0))) + ((vec4<f32>(_e81.g0_.x) * vec4<f32>(_e85.g0_.x, _e88.g0_.x, _e91.g0_.y, _e94.g0_.z)) * vec4<f32>(0.0, -(1.0), -(1.0), -(1.0)))));
}

fn motor_dual_point_regressive_product(self_2204: MotorDual, other_1926: Point) -> Motor {
    var self_2205: MotorDual;
    var other_1927: Point;

    self_2205 = self_2204;
    other_1927 = other_1926;
    let _e4: MotorDual = self_2205;
    let _e8: Point = other_1927;
    let _e19: MotorDual = self_2205;
    let _e23: Point = other_1927;
    let _e35: MotorDual = self_2205;
    let _e38: Point = other_1927;
    let _e41: Point = other_1927;
    let _e44: Point = other_1927;
    let _e47: Point = other_1927;
    return Motor(((((vec4<f32>(_e4.g0_.z) * vec4<f32>(_e8.g0_.y)) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) + ((vec4<f32>(_e19.g0_.w) * vec4<f32>(_e23.g0_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + (_e35.g0_.yxxx * vec4<f32>(_e38.g0_.x, _e41.g0_.x, _e44.g0_.y, _e47.g0_.z))));
}

fn motor_dual_point_outer_product(self_2206: MotorDual, other_1928: Point) -> AntiScalar {
    var self_2207: MotorDual;
    var other_1929: Point;

    self_2207 = self_2206;
    other_1929 = other_1928;
    let _e4: MotorDual = self_2207;
    let _e7: Point = other_1929;
    let _e11: MotorDual = self_2207;
    let _e14: Point = other_1929;
    let _e19: MotorDual = self_2207;
    let _e22: Point = other_1929;
    return AntiScalar((((_e4.g0_.y * _e7.g0_.x) + (_e11.g0_.z * _e14.g0_.y)) + (_e19.g0_.w * _e22.g0_.z)));
}

fn motor_dual_point_inner_product(self_2208: MotorDual, other_1930: Point) -> Plane {
    var self_2209: MotorDual;
    var other_1931: Point;

    self_2209 = self_2208;
    other_1931 = other_1930;
    let _e6: MotorDual = self_2209;
    let _e10: Point = other_1931;
    let _e14: MotorDual = self_2209;
    let _e18: Point = other_1931;
    let _e29: MotorDual = self_2209;
    let _e33: Point = other_1931;
    let _e44: MotorDual = self_2209;
    let _e47: MotorDual = self_2209;
    let _e50: MotorDual = self_2209;
    let _e54: Point = other_1931;
    return Plane(((((vec3<f32>(0.0) - (vec3<f32>(_e6.g0_.x) * _e10.g0_)) + ((vec3<f32>(_e14.g0_.z) * _e18.g0_.zzx) * vec3<f32>(1.0, 0.0, -(1.0)))) + ((vec3<f32>(_e29.g0_.w) * _e33.g0_.yxy) * vec3<f32>(-(1.0), 1.0, 0.0))) + ((vec3<f32>(_e44.g0_.x, _e47.g0_.y, _e50.g0_.y) * _e54.g0_.xzy) * vec3<f32>(0.0, -(1.0), 1.0))));
}

fn motor_dual_point_geometric_anti_product(self_2210: MotorDual, other_1932: Point) -> Motor {
    var self_2211: MotorDual;
    var other_1933: Point;

    self_2211 = self_2210;
    other_1933 = other_1932;
    let _e4: MotorDual = self_2211;
    let _e8: Point = other_1933;
    let _e11: Point = other_1933;
    let _e14: Point = other_1933;
    let _e17: Point = other_1933;
    let _e29: MotorDual = self_2211;
    let _e33: Point = other_1933;
    let _e36: Point = other_1933;
    let _e39: Point = other_1933;
    let _e42: Point = other_1933;
    let _e55: MotorDual = self_2211;
    let _e59: Point = other_1933;
    let _e62: Point = other_1933;
    let _e65: Point = other_1933;
    let _e68: Point = other_1933;
    let _e81: MotorDual = self_2211;
    let _e85: Point = other_1933;
    let _e88: Point = other_1933;
    let _e91: Point = other_1933;
    let _e94: Point = other_1933;
    return Motor((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.x, _e11.g0_.x, _e14.g0_.z, _e17.g0_.y)) * vec4<f32>(1.0, 0.0, -(1.0), 1.0)) + ((vec4<f32>(_e29.g0_.z) * vec4<f32>(_e33.g0_.y, _e36.g0_.z, _e39.g0_.y, _e42.g0_.x)) * vec4<f32>(1.0, 1.0, 0.0, -(1.0)))) + ((vec4<f32>(_e55.g0_.w) * vec4<f32>(_e59.g0_.z, _e62.g0_.y, _e65.g0_.x, _e68.g0_.z)) * vec4<f32>(1.0, -(1.0), 1.0, 0.0))) + ((vec4<f32>(_e81.g0_.x) * vec4<f32>(_e85.g0_.x, _e88.g0_.x, _e91.g0_.y, _e94.g0_.z)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn motor_dual_point_inner_anti_product(self_2212: MotorDual, other_1934: Point) -> Point {
    var self_2213: MotorDual;
    var other_1935: Point;

    self_2213 = self_2212;
    other_1935 = other_1934;
    let _e4: MotorDual = self_2213;
    let _e8: Point = other_1935;
    let _e11: MotorDual = self_2213;
    let _e15: Point = other_1935;
    let _e26: MotorDual = self_2213;
    let _e30: Point = other_1935;
    let _e41: MotorDual = self_2213;
    let _e44: MotorDual = self_2213;
    let _e47: MotorDual = self_2213;
    let _e51: Point = other_1935;
    return Point(((((vec3<f32>(_e4.g0_.x) * _e8.g0_) + ((vec3<f32>(_e11.g0_.z) * _e15.g0_.zzx) * vec3<f32>(1.0, 0.0, -(1.0)))) + ((vec3<f32>(_e26.g0_.w) * _e30.g0_.yxy) * vec3<f32>(-(1.0), 1.0, 0.0))) + ((vec3<f32>(_e41.g0_.x, _e44.g0_.y, _e47.g0_.y) * _e51.g0_.xzy) * vec3<f32>(0.0, -(1.0), 1.0))));
}

fn motor_dual_point_left_contraction(self_2214: MotorDual, other_1936: Point) -> Plane {
    var self_2215: MotorDual;
    var other_1937: Point;

    self_2215 = self_2214;
    other_1937 = other_1936;
    let _e4: MotorDual = self_2215;
    let _e8: Point = other_1937;
    let _e18: MotorDual = self_2215;
    let _e22: Point = other_1937;
    let _e33: MotorDual = self_2215;
    let _e36: MotorDual = self_2215;
    let _e39: MotorDual = self_2215;
    let _e43: Point = other_1937;
    return Plane(((((vec3<f32>(_e4.g0_.z) * _e8.g0_.zzx) * vec3<f32>(1.0, 0.0, -(1.0))) + ((vec3<f32>(_e18.g0_.w) * _e22.g0_.yxy) * vec3<f32>(-(1.0), 1.0, 0.0))) + ((vec3<f32>(_e33.g0_.x, _e36.g0_.y, _e39.g0_.y) * _e43.g0_.xzy) * vec3<f32>(0.0, -(1.0), 1.0))));
}

fn motor_dual_point_right_contraction(self_2216: MotorDual, other_1938: Point) -> Plane {
    var self_2217: MotorDual;
    var other_1939: Point;

    self_2217 = self_2216;
    other_1939 = other_1938;
    let _e6: MotorDual = self_2217;
    let _e10: Point = other_1939;
    return Plane((vec3<f32>(0.0) - (vec3<f32>(_e6.g0_.x) * _e10.g0_)));
}

fn motor_dual_point_left_anti_contraction(self_2218: MotorDual, other_1940: Point) -> Point {
    var self_2219: MotorDual;
    var other_1941: Point;

    self_2219 = self_2218;
    other_1941 = other_1940;
    let _e4: MotorDual = self_2219;
    let _e8: Point = other_1941;
    return Point((vec3<f32>(_e4.g0_.x) * _e8.g0_));
}

fn motor_dual_point_right_anti_contraction(self_2220: MotorDual, other_1942: Point) -> Point {
    var self_2221: MotorDual;
    var other_1943: Point;

    self_2221 = self_2220;
    other_1943 = other_1942;
    let _e4: MotorDual = self_2221;
    let _e8: Point = other_1943;
    let _e18: MotorDual = self_2221;
    let _e22: Point = other_1943;
    let _e33: MotorDual = self_2221;
    let _e36: MotorDual = self_2221;
    let _e39: MotorDual = self_2221;
    let _e43: Point = other_1943;
    return Point(((((vec3<f32>(_e4.g0_.z) * _e8.g0_.zzx) * vec3<f32>(1.0, 0.0, -(1.0))) + ((vec3<f32>(_e18.g0_.w) * _e22.g0_.yxy) * vec3<f32>(-(1.0), 1.0, 0.0))) + ((vec3<f32>(_e33.g0_.x, _e36.g0_.y, _e39.g0_.y) * _e43.g0_.xzy) * vec3<f32>(0.0, -(1.0), 1.0))));
}

fn motor_dual_ideal_point_geometric_product(self_2222: MotorDual, other_1944: IdealPoint) -> MotorDual {
    var self_2223: MotorDual;
    var other_1945: IdealPoint;

    self_2223 = self_2222;
    other_1945 = other_1944;
    let _e4: MotorDual = self_2223;
    let _e8: IdealPoint = other_1945;
    let _e11: IdealPoint = other_1945;
    let _e14: IdealPoint = other_1945;
    let _e17: IdealPoint = other_1945;
    let _e29: MotorDual = self_2223;
    let _e33: IdealPoint = other_1945;
    let _e36: IdealPoint = other_1945;
    let _e39: IdealPoint = other_1945;
    let _e42: IdealPoint = other_1945;
    let _e55: MotorDual = self_2223;
    let _e58: IdealPoint = other_1945;
    let _e61: IdealPoint = other_1945;
    let _e64: IdealPoint = other_1945;
    let _e67: IdealPoint = other_1945;
    return MotorDual(((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y, _e11.g0_.y, _e14.g0_.y, _e17.g0_.x)) * vec4<f32>(0.0, 0.0, -(1.0), 1.0)) + ((vec4<f32>(_e29.g0_.w) * vec4<f32>(_e33.g0_.y, _e36.g0_.x, _e39.g0_.y, _e42.g0_.y)) * vec4<f32>(1.0, -(1.0), 0.0, 0.0))) + ((_e55.g0_.zzxx * vec4<f32>(_e58.g0_.x, _e61.g0_.y, _e64.g0_.x, _e67.g0_.y)) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))));
}

fn motor_dual_ideal_point_regressive_product(self_2224: MotorDual, other_1946: IdealPoint) -> Translator {
    var self_2225: MotorDual;
    var other_1947: IdealPoint;

    self_2225 = self_2224;
    other_1947 = other_1946;
    let _e4: MotorDual = self_2225;
    let _e8: IdealPoint = other_1947;
    let _e18: MotorDual = self_2225;
    let _e21: MotorDual = self_2225;
    let _e24: MotorDual = self_2225;
    let _e28: IdealPoint = other_1947;
    let _e31: IdealPoint = other_1947;
    let _e34: IdealPoint = other_1947;
    return Translator((((vec3<f32>(_e4.g0_.w) * vec3<f32>(_e8.g0_.y)) * vec3<f32>(1.0, 0.0, 0.0)) + (vec3<f32>(_e18.g0_.z, _e21.g0_.x, _e24.g0_.x) * vec3<f32>(_e28.g0_.x, _e31.g0_.x, _e34.g0_.y))));
}

fn motor_dual_ideal_point_outer_product(self_2226: MotorDual, other_1948: IdealPoint) -> AntiScalar {
    var self_2227: MotorDual;
    var other_1949: IdealPoint;

    self_2227 = self_2226;
    other_1949 = other_1948;
    let _e4: MotorDual = self_2227;
    let _e7: IdealPoint = other_1949;
    let _e11: MotorDual = self_2227;
    let _e14: IdealPoint = other_1949;
    return AntiScalar(((_e4.g0_.z * _e7.g0_.x) + (_e11.g0_.w * _e14.g0_.y)));
}

fn motor_dual_ideal_point_inner_product(self_2228: MotorDual, other_1950: IdealPoint) -> Plane {
    var self_2229: MotorDual;
    var other_1951: IdealPoint;

    self_2229 = self_2228;
    other_1951 = other_1950;
    let _e4: MotorDual = self_2229;
    let _e8: IdealPoint = other_1951;
    let _e11: IdealPoint = other_1951;
    let _e14: IdealPoint = other_1951;
    let _e25: MotorDual = self_2229;
    let _e29: IdealPoint = other_1951;
    let _e41: MotorDual = self_2229;
    let _e44: MotorDual = self_2229;
    let _e47: MotorDual = self_2229;
    let _e51: IdealPoint = other_1951;
    let _e54: IdealPoint = other_1951;
    let _e57: IdealPoint = other_1951;
    return Plane(((((vec3<f32>(_e4.g0_.y) * vec3<f32>(_e8.g0_.y, _e11.g0_.y, _e14.g0_.x)) * vec3<f32>(0.0, -(1.0), 1.0)) + ((vec3<f32>(_e25.g0_.w) * vec3<f32>(_e29.g0_.x)) * vec3<f32>(-(1.0), 0.0, 0.0))) + ((vec3<f32>(_e41.g0_.z, _e44.g0_.x, _e47.g0_.x) * vec3<f32>(_e51.g0_.y, _e54.g0_.x, _e57.g0_.y)) * vec3<f32>(1.0, -(1.0), -(1.0)))));
}

fn motor_dual_ideal_point_geometric_anti_product(self_2230: MotorDual, other_1952: IdealPoint) -> Motor {
    var self_2231: MotorDual;
    var other_1953: IdealPoint;

    self_2231 = self_2230;
    other_1953 = other_1952;
    let _e4: MotorDual = self_2231;
    let _e8: IdealPoint = other_1953;
    let _e11: IdealPoint = other_1953;
    let _e14: IdealPoint = other_1953;
    let _e17: IdealPoint = other_1953;
    let _e29: MotorDual = self_2231;
    let _e33: IdealPoint = other_1953;
    let _e36: IdealPoint = other_1953;
    let _e39: IdealPoint = other_1953;
    let _e42: IdealPoint = other_1953;
    let _e55: MotorDual = self_2231;
    let _e58: IdealPoint = other_1953;
    let _e61: IdealPoint = other_1953;
    let _e64: IdealPoint = other_1953;
    let _e67: IdealPoint = other_1953;
    return Motor(((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y, _e11.g0_.y, _e14.g0_.y, _e17.g0_.x)) * vec4<f32>(0.0, 0.0, -(1.0), 1.0)) + ((vec4<f32>(_e29.g0_.w) * vec4<f32>(_e33.g0_.y, _e36.g0_.x, _e39.g0_.y, _e42.g0_.y)) * vec4<f32>(1.0, -(1.0), 0.0, 0.0))) + (_e55.g0_.zzxx * vec4<f32>(_e58.g0_.x, _e61.g0_.y, _e64.g0_.x, _e67.g0_.y))));
}

fn motor_dual_ideal_point_inner_anti_product(self_2232: MotorDual, other_1954: IdealPoint) -> Point {
    var self_2233: MotorDual;
    var other_1955: IdealPoint;

    self_2233 = self_2232;
    other_1955 = other_1954;
    let _e4: MotorDual = self_2233;
    let _e8: IdealPoint = other_1955;
    let _e11: IdealPoint = other_1955;
    let _e14: IdealPoint = other_1955;
    let _e25: MotorDual = self_2233;
    let _e29: IdealPoint = other_1955;
    let _e41: MotorDual = self_2233;
    let _e44: MotorDual = self_2233;
    let _e47: MotorDual = self_2233;
    let _e51: IdealPoint = other_1955;
    let _e54: IdealPoint = other_1955;
    let _e57: IdealPoint = other_1955;
    return Point(((((vec3<f32>(_e4.g0_.y) * vec3<f32>(_e8.g0_.y, _e11.g0_.y, _e14.g0_.x)) * vec3<f32>(0.0, -(1.0), 1.0)) + ((vec3<f32>(_e25.g0_.w) * vec3<f32>(_e29.g0_.x)) * vec3<f32>(-(1.0), 0.0, 0.0))) + (vec3<f32>(_e41.g0_.z, _e44.g0_.x, _e47.g0_.x) * vec3<f32>(_e51.g0_.y, _e54.g0_.x, _e57.g0_.y))));
}

fn motor_dual_ideal_point_left_contraction(self_2234: MotorDual, other_1956: IdealPoint) -> Plane {
    var self_2235: MotorDual;
    var other_1957: IdealPoint;

    self_2235 = self_2234;
    other_1957 = other_1956;
    let _e4: MotorDual = self_2235;
    let _e8: IdealPoint = other_1957;
    let _e19: MotorDual = self_2235;
    let _e22: MotorDual = self_2235;
    let _e25: MotorDual = self_2235;
    let _e29: IdealPoint = other_1957;
    let _e32: IdealPoint = other_1957;
    let _e35: IdealPoint = other_1957;
    return Plane((((vec3<f32>(_e4.g0_.w) * vec3<f32>(_e8.g0_.x)) * vec3<f32>(-(1.0), 0.0, 0.0)) + ((vec3<f32>(_e19.g0_.z, _e22.g0_.y, _e25.g0_.y) * vec3<f32>(_e29.g0_.y, _e32.g0_.y, _e35.g0_.x)) * vec3<f32>(1.0, -(1.0), 1.0))));
}

fn motor_dual_ideal_point_left_anti_contraction(self_2236: MotorDual, other_1958: IdealPoint) -> IdealPoint {
    var self_2237: MotorDual;
    var other_1959: IdealPoint;

    self_2237 = self_2236;
    other_1959 = other_1958;
    let _e4: MotorDual = self_2237;
    let _e8: IdealPoint = other_1959;
    return IdealPoint((vec2<f32>(_e4.g0_.x) * _e8.g0_));
}

fn motor_dual_ideal_point_right_anti_contraction(self_2238: MotorDual, other_1960: IdealPoint) -> Point {
    var self_2239: MotorDual;
    var other_1961: IdealPoint;

    self_2239 = self_2238;
    other_1961 = other_1960;
    let _e4: MotorDual = self_2239;
    let _e8: IdealPoint = other_1961;
    let _e19: MotorDual = self_2239;
    let _e22: MotorDual = self_2239;
    let _e25: MotorDual = self_2239;
    let _e29: IdealPoint = other_1961;
    let _e32: IdealPoint = other_1961;
    let _e35: IdealPoint = other_1961;
    return Point((((vec3<f32>(_e4.g0_.w) * vec3<f32>(_e8.g0_.x)) * vec3<f32>(-(1.0), 0.0, 0.0)) + ((vec3<f32>(_e19.g0_.z, _e22.g0_.y, _e25.g0_.y) * vec3<f32>(_e29.g0_.y, _e32.g0_.y, _e35.g0_.x)) * vec3<f32>(1.0, -(1.0), 1.0))));
}

fn motor_dual_plane_into(self_2240: MotorDual) -> Plane {
    var self_2241: MotorDual;

    self_2241 = self_2240;
    let _e2: MotorDual = self_2241;
    let _e5: MotorDual = self_2241;
    let _e8: MotorDual = self_2241;
    return Plane(vec3<f32>(_e2.g0_.y, _e5.g0_.z, _e8.g0_.w));
}

fn motor_dual_plane_add(self_2242: MotorDual, other_1962: Plane) -> MotorDual {
    var self_2243: MotorDual;
    var other_1963: Plane;

    self_2243 = self_2242;
    other_1963 = other_1962;
    let _e4: MotorDual = self_2243;
    let _e6: Plane = other_1963;
    let _e9: Plane = other_1963;
    let _e12: Plane = other_1963;
    let _e15: Plane = other_1963;
    return MotorDual((_e4.g0_ + (vec4<f32>(_e6.g0_.x, _e9.g0_.x, _e12.g0_.y, _e15.g0_.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn motor_dual_plane_sub(self_2244: MotorDual, other_1964: Plane) -> MotorDual {
    var self_2245: MotorDual;
    var other_1965: Plane;

    self_2245 = self_2244;
    other_1965 = other_1964;
    let _e4: MotorDual = self_2245;
    let _e6: Plane = other_1965;
    let _e9: Plane = other_1965;
    let _e12: Plane = other_1965;
    let _e15: Plane = other_1965;
    return MotorDual((_e4.g0_ - (vec4<f32>(_e6.g0_.x, _e9.g0_.x, _e12.g0_.y, _e15.g0_.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn motor_dual_plane_geometric_product(self_2246: MotorDual, other_1966: Plane) -> Motor {
    var self_2247: MotorDual;
    var other_1967: Plane;

    self_2247 = self_2246;
    other_1967 = other_1966;
    let _e4: MotorDual = self_2247;
    let _e8: Plane = other_1967;
    let _e11: Plane = other_1967;
    let _e14: Plane = other_1967;
    let _e17: Plane = other_1967;
    let _e29: MotorDual = self_2247;
    let _e33: Plane = other_1967;
    let _e36: Plane = other_1967;
    let _e39: Plane = other_1967;
    let _e42: Plane = other_1967;
    let _e55: MotorDual = self_2247;
    let _e59: Plane = other_1967;
    let _e62: Plane = other_1967;
    let _e65: Plane = other_1967;
    let _e68: Plane = other_1967;
    let _e81: MotorDual = self_2247;
    let _e85: Plane = other_1967;
    let _e88: Plane = other_1967;
    let _e91: Plane = other_1967;
    let _e94: Plane = other_1967;
    return Motor((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.x, _e11.g0_.x, _e14.g0_.z, _e17.g0_.y)) * vec4<f32>(1.0, 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e29.g0_.z) * vec4<f32>(_e33.g0_.y, _e36.g0_.z, _e39.g0_.y, _e42.g0_.x)) * vec4<f32>(1.0, -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e55.g0_.w) * vec4<f32>(_e59.g0_.z, _e62.g0_.y, _e65.g0_.x, _e68.g0_.z)) * vec4<f32>(1.0, 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e81.g0_.x) * vec4<f32>(_e85.g0_.x, _e88.g0_.x, _e91.g0_.y, _e94.g0_.z)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn motor_dual_plane_regressive_product(self_2248: MotorDual, other_1968: Plane) -> Plane {
    var self_2249: MotorDual;
    var other_1969: Plane;

    self_2249 = self_2248;
    other_1969 = other_1968;
    let _e4: MotorDual = self_2249;
    let _e8: Plane = other_1969;
    return Plane((vec3<f32>(_e4.g0_.x) * _e8.g0_));
}

fn motor_dual_plane_outer_product(self_2250: MotorDual, other_1970: Plane) -> Point {
    var self_2251: MotorDual;
    var other_1971: Plane;

    self_2251 = self_2250;
    other_1971 = other_1970;
    let _e4: MotorDual = self_2251;
    let _e8: Plane = other_1971;
    let _e18: MotorDual = self_2251;
    let _e22: Plane = other_1971;
    let _e33: MotorDual = self_2251;
    let _e36: MotorDual = self_2251;
    let _e39: MotorDual = self_2251;
    let _e43: Plane = other_1971;
    return Point(((((vec3<f32>(_e4.g0_.z) * _e8.g0_.zzx) * vec3<f32>(-(1.0), 0.0, 1.0)) + ((vec3<f32>(_e18.g0_.w) * _e22.g0_.yxy) * vec3<f32>(1.0, -(1.0), 0.0))) + ((vec3<f32>(_e33.g0_.x, _e36.g0_.y, _e39.g0_.y) * _e43.g0_.xzy) * vec3<f32>(0.0, 1.0, -(1.0)))));
}

fn motor_dual_plane_inner_product(self_2252: MotorDual, other_1972: Plane) -> Motor {
    var self_2253: MotorDual;
    var other_1973: Plane;

    self_2253 = self_2252;
    other_1973 = other_1972;
    let _e4: MotorDual = self_2253;
    let _e8: Plane = other_1973;
    let _e19: MotorDual = self_2253;
    let _e23: Plane = other_1973;
    let _e35: MotorDual = self_2253;
    let _e38: Plane = other_1973;
    let _e41: Plane = other_1973;
    let _e44: Plane = other_1973;
    let _e47: Plane = other_1973;
    return Motor(((((vec4<f32>(_e4.g0_.z) * vec4<f32>(_e8.g0_.y)) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) + ((vec4<f32>(_e19.g0_.w) * vec4<f32>(_e23.g0_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + (_e35.g0_.yxxx * vec4<f32>(_e38.g0_.x, _e41.g0_.x, _e44.g0_.y, _e47.g0_.z))));
}

fn motor_dual_plane_geometric_anti_product(self_2254: MotorDual, other_1974: Plane) -> MotorDual {
    var self_2255: MotorDual;
    var other_1975: Plane;

    self_2255 = self_2254;
    other_1975 = other_1974;
    let _e4: MotorDual = self_2255;
    let _e8: Plane = other_1975;
    let _e11: Plane = other_1975;
    let _e14: Plane = other_1975;
    let _e17: Plane = other_1975;
    let _e30: MotorDual = self_2255;
    let _e34: Plane = other_1975;
    let _e37: Plane = other_1975;
    let _e40: Plane = other_1975;
    let _e43: Plane = other_1975;
    let _e57: MotorDual = self_2255;
    let _e61: Plane = other_1975;
    let _e64: Plane = other_1975;
    let _e67: Plane = other_1975;
    let _e70: Plane = other_1975;
    let _e84: MotorDual = self_2255;
    let _e88: Plane = other_1975;
    let _e91: Plane = other_1975;
    let _e94: Plane = other_1975;
    let _e97: Plane = other_1975;
    return MotorDual((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.x, _e11.g0_.x, _e14.g0_.z, _e17.g0_.y)) * vec4<f32>(-(1.0), 0.0, -(1.0), 1.0)) + ((vec4<f32>(_e30.g0_.z) * vec4<f32>(_e34.g0_.y, _e37.g0_.z, _e40.g0_.y, _e43.g0_.x)) * vec4<f32>(-(1.0), 1.0, 0.0, -(1.0)))) + ((vec4<f32>(_e57.g0_.w) * vec4<f32>(_e61.g0_.z, _e64.g0_.y, _e67.g0_.x, _e70.g0_.z)) * vec4<f32>(-(1.0), -(1.0), 1.0, 0.0))) + ((vec4<f32>(_e84.g0_.x) * vec4<f32>(_e88.g0_.x, _e91.g0_.x, _e94.g0_.y, _e97.g0_.z)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn motor_dual_plane_inner_anti_product(self_2256: MotorDual, other_1976: Plane) -> MotorDual {
    var self_2257: MotorDual;
    var other_1977: Plane;

    self_2257 = self_2256;
    other_1977 = other_1976;
    let _e4: MotorDual = self_2257;
    let _e8: Plane = other_1977;
    let _e20: MotorDual = self_2257;
    let _e24: Plane = other_1977;
    let _e37: MotorDual = self_2257;
    let _e40: Plane = other_1977;
    let _e43: Plane = other_1977;
    let _e46: Plane = other_1977;
    let _e49: Plane = other_1977;
    return MotorDual(((((vec4<f32>(_e4.g0_.z) * vec4<f32>(_e8.g0_.y)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0)) + ((vec4<f32>(_e20.g0_.w) * vec4<f32>(_e24.g0_.z)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((_e37.g0_.yxxx * vec4<f32>(_e40.g0_.x, _e43.g0_.x, _e46.g0_.y, _e49.g0_.z)) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0))));
}

fn motor_dual_plane_left_contraction(self_2258: MotorDual, other_1978: Plane) -> Scalar {
    var self_2259: MotorDual;
    var other_1979: Plane;

    self_2259 = self_2258;
    other_1979 = other_1978;
    let _e4: MotorDual = self_2259;
    let _e7: Plane = other_1979;
    let _e11: MotorDual = self_2259;
    let _e14: Plane = other_1979;
    let _e19: MotorDual = self_2259;
    let _e22: Plane = other_1979;
    return Scalar((((_e4.g0_.y * _e7.g0_.x) + (_e11.g0_.z * _e14.g0_.y)) + (_e19.g0_.w * _e22.g0_.z)));
}

fn motor_dual_plane_right_contraction(self_2260: MotorDual, other_1980: Plane) -> Motor {
    var self_2261: MotorDual;
    var other_1981: Plane;

    self_2261 = self_2260;
    other_1981 = other_1980;
    let _e4: MotorDual = self_2261;
    let _e8: Plane = other_1981;
    let _e19: MotorDual = self_2261;
    let _e23: Plane = other_1981;
    let _e35: MotorDual = self_2261;
    let _e38: Plane = other_1981;
    let _e41: Plane = other_1981;
    let _e44: Plane = other_1981;
    let _e47: Plane = other_1981;
    return Motor(((((vec4<f32>(_e4.g0_.z) * vec4<f32>(_e8.g0_.y)) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) + ((vec4<f32>(_e19.g0_.w) * vec4<f32>(_e23.g0_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + (_e35.g0_.yxxx * vec4<f32>(_e38.g0_.x, _e41.g0_.x, _e44.g0_.y, _e47.g0_.z))));
}

fn motor_dual_plane_left_anti_contraction(self_2262: MotorDual, other_1982: Plane) -> MotorDual {
    var self_2263: MotorDual;
    var other_1983: Plane;

    self_2263 = self_2262;
    other_1983 = other_1982;
    let _e4: MotorDual = self_2263;
    let _e8: Plane = other_1983;
    let _e20: MotorDual = self_2263;
    let _e24: Plane = other_1983;
    let _e37: MotorDual = self_2263;
    let _e40: Plane = other_1983;
    let _e43: Plane = other_1983;
    let _e46: Plane = other_1983;
    let _e49: Plane = other_1983;
    return MotorDual(((((vec4<f32>(_e4.g0_.z) * vec4<f32>(_e8.g0_.y)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0)) + ((vec4<f32>(_e20.g0_.w) * vec4<f32>(_e24.g0_.z)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((_e37.g0_.yxxx * vec4<f32>(_e40.g0_.x, _e43.g0_.x, _e46.g0_.y, _e49.g0_.z)) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0))));
}

fn motor_dual_plane_right_anti_contraction(self_2264: MotorDual, other_1984: Plane) -> AntiScalar {
    var self_2265: MotorDual;
    var other_1985: Plane;

    self_2265 = self_2264;
    other_1985 = other_1984;
    let _e5: MotorDual = self_2265;
    let _e8: Plane = other_1985;
    let _e13: MotorDual = self_2265;
    let _e16: Plane = other_1985;
    let _e21: MotorDual = self_2265;
    let _e24: Plane = other_1985;
    return AntiScalar((((0.0 - (_e5.g0_.y * _e8.g0_.x)) - (_e13.g0_.z * _e16.g0_.y)) - (_e21.g0_.w * _e24.g0_.z)));
}

fn motor_dual_plane_scalar_product(self_2266: MotorDual, other_1986: Plane) -> Scalar {
    var self_2267: MotorDual;
    var other_1987: Plane;

    self_2267 = self_2266;
    other_1987 = other_1986;
    let _e4: MotorDual = self_2267;
    let _e7: Plane = other_1987;
    let _e11: MotorDual = self_2267;
    let _e14: Plane = other_1987;
    let _e19: MotorDual = self_2267;
    let _e22: Plane = other_1987;
    return Scalar((((_e4.g0_.y * _e7.g0_.x) + (_e11.g0_.z * _e14.g0_.y)) + (_e19.g0_.w * _e22.g0_.z)));
}

fn motor_dual_plane_anti_scalar_product(self_2268: MotorDual, other_1988: Plane) -> AntiScalar {
    var self_2269: MotorDual;
    var other_1989: Plane;

    self_2269 = self_2268;
    other_1989 = other_1988;
    let _e5: MotorDual = self_2269;
    let _e8: Plane = other_1989;
    let _e13: MotorDual = self_2269;
    let _e16: Plane = other_1989;
    let _e21: MotorDual = self_2269;
    let _e24: Plane = other_1989;
    return AntiScalar((((0.0 - (_e5.g0_.y * _e8.g0_.x)) - (_e13.g0_.z * _e16.g0_.y)) - (_e21.g0_.w * _e24.g0_.z)));
}

fn motor_dual_translator_geometric_product(self_2270: MotorDual, other_1990: Translator) -> MotorDual {
    var self_2271: MotorDual;
    var other_1991: Translator;

    self_2271 = self_2270;
    other_1991 = other_1990;
    let _e4: MotorDual = self_2271;
    let _e8: Translator = other_1991;
    let _e11: Translator = other_1991;
    let _e14: Translator = other_1991;
    let _e17: Translator = other_1991;
    let _e29: MotorDual = self_2271;
    let _e33: Translator = other_1991;
    let _e36: Translator = other_1991;
    let _e39: Translator = other_1991;
    let _e42: Translator = other_1991;
    let _e54: MotorDual = self_2271;
    let _e58: Translator = other_1991;
    let _e61: Translator = other_1991;
    let _e64: Translator = other_1991;
    let _e67: Translator = other_1991;
    let _e80: MotorDual = self_2271;
    let _e84: Translator = other_1991;
    let _e87: Translator = other_1991;
    let _e90: Translator = other_1991;
    let _e93: Translator = other_1991;
    return MotorDual((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.x, _e11.g0_.x, _e14.g0_.z, _e17.g0_.y)) * vec4<f32>(0.0, 1.0, -(1.0), 1.0)) + ((vec4<f32>(_e29.g0_.z) * vec4<f32>(_e33.g0_.y, _e36.g0_.z, _e39.g0_.x, _e42.g0_.y)) * vec4<f32>(1.0, 1.0, 1.0, 0.0))) + ((vec4<f32>(_e54.g0_.w) * vec4<f32>(_e58.g0_.z, _e61.g0_.y, _e64.g0_.z, _e67.g0_.x)) * vec4<f32>(1.0, -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e80.g0_.x) * vec4<f32>(_e84.g0_.x, _e87.g0_.x, _e90.g0_.y, _e93.g0_.z)) * vec4<f32>(1.0, 0.0, -(1.0), -(1.0)))));
}

fn motor_dual_translator_regressive_product(self_2272: MotorDual, other_1992: Translator) -> Translator {
    var self_2273: MotorDual;
    var other_1993: Translator;

    self_2273 = self_2272;
    other_1993 = other_1992;
    let _e4: MotorDual = self_2273;
    let _e8: Translator = other_1993;
    let _e11: MotorDual = self_2273;
    let _e15: Translator = other_1993;
    let _e26: MotorDual = self_2273;
    let _e29: MotorDual = self_2273;
    let _e32: MotorDual = self_2273;
    let _e36: Translator = other_1993;
    return Translator((((vec3<f32>(_e4.g0_.x) * _e8.g0_) + ((vec3<f32>(_e11.g0_.w) * vec3<f32>(_e15.g0_.z)) * vec3<f32>(1.0, 0.0, 0.0))) + ((vec3<f32>(_e26.g0_.z, _e29.g0_.x, _e32.g0_.x) * _e36.g0_.yxx) * vec3<f32>(1.0, 0.0, 0.0))));
}

fn motor_dual_translator_outer_product(self_2274: MotorDual, other_1994: Translator) -> MotorDual {
    var self_2275: MotorDual;
    var other_1995: Translator;

    self_2275 = self_2274;
    other_1995 = other_1994;
    let _e4: MotorDual = self_2275;
    let _e8: Translator = other_1995;
    let _e11: Translator = other_1995;
    let _e14: Translator = other_1995;
    let _e17: Translator = other_1995;
    let _e28: MotorDual = self_2275;
    let _e32: Translator = other_1995;
    let _e35: Translator = other_1995;
    let _e38: Translator = other_1995;
    let _e41: Translator = other_1995;
    let _e53: MotorDual = self_2275;
    let _e56: Translator = other_1995;
    return MotorDual(((((vec4<f32>(_e4.g0_.z) * vec4<f32>(_e8.g0_.y, _e11.g0_.y, _e14.g0_.x, _e17.g0_.y)) * vec4<f32>(1.0, 0.0, 1.0, 0.0)) + ((vec4<f32>(_e28.g0_.w) * vec4<f32>(_e32.g0_.z, _e35.g0_.z, _e38.g0_.z, _e41.g0_.x)) * vec4<f32>(1.0, 0.0, 0.0, 1.0))) + ((_e53.g0_.xyxx * vec4<f32>(_e56.g0_.x)) * vec4<f32>(1.0, 1.0, 0.0, 0.0))));
}

fn motor_dual_translator_inner_product(self_2276: MotorDual, other_1996: Translator) -> MotorDual {
    var self_2277: MotorDual;
    var other_1997: Translator;

    self_2277 = self_2276;
    other_1997 = other_1996;
    let _e4: MotorDual = self_2277;
    let _e8: Translator = other_1997;
    let _e11: Translator = other_1997;
    let _e14: Translator = other_1997;
    let _e17: Translator = other_1997;
    let _e29: MotorDual = self_2277;
    let _e33: Translator = other_1997;
    let _e36: Translator = other_1997;
    let _e39: Translator = other_1997;
    let _e42: Translator = other_1997;
    let _e54: MotorDual = self_2277;
    let _e58: Translator = other_1997;
    let _e61: Translator = other_1997;
    let _e64: Translator = other_1997;
    let _e67: Translator = other_1997;
    let _e80: MotorDual = self_2277;
    let _e84: Translator = other_1997;
    let _e87: Translator = other_1997;
    let _e90: Translator = other_1997;
    let _e93: Translator = other_1997;
    return MotorDual((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.x, _e11.g0_.x, _e14.g0_.z, _e17.g0_.y)) * vec4<f32>(0.0, 1.0, -(1.0), 1.0)) + ((vec4<f32>(_e29.g0_.z) * vec4<f32>(_e33.g0_.z, _e36.g0_.z, _e39.g0_.x, _e42.g0_.z)) * vec4<f32>(0.0, 1.0, 1.0, 0.0))) + ((vec4<f32>(_e54.g0_.w) * vec4<f32>(_e58.g0_.y, _e61.g0_.y, _e64.g0_.y, _e67.g0_.x)) * vec4<f32>(0.0, -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e80.g0_.x) * vec4<f32>(_e84.g0_.x, _e87.g0_.x, _e90.g0_.y, _e93.g0_.z)) * vec4<f32>(1.0, 0.0, -(1.0), -(1.0)))));
}

fn motor_dual_translator_geometric_anti_product(self_2278: MotorDual, other_1998: Translator) -> Motor {
    var self_2279: MotorDual;
    var other_1999: Translator;

    self_2279 = self_2278;
    other_1999 = other_1998;
    let _e4: MotorDual = self_2279;
    let _e8: Translator = other_1999;
    let _e11: Translator = other_1999;
    let _e14: Translator = other_1999;
    let _e17: Translator = other_1999;
    let _e30: MotorDual = self_2279;
    let _e34: Translator = other_1999;
    let _e37: Translator = other_1999;
    let _e40: Translator = other_1999;
    let _e43: Translator = other_1999;
    let _e56: MotorDual = self_2279;
    let _e60: Translator = other_1999;
    let _e63: Translator = other_1999;
    let _e66: Translator = other_1999;
    let _e69: Translator = other_1999;
    let _e83: MotorDual = self_2279;
    let _e87: Translator = other_1999;
    let _e90: Translator = other_1999;
    let _e93: Translator = other_1999;
    let _e96: Translator = other_1999;
    return Motor((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.x, _e11.g0_.x, _e14.g0_.z, _e17.g0_.y)) * vec4<f32>(0.0, -(1.0), -(1.0), 1.0)) + ((vec4<f32>(_e30.g0_.z) * vec4<f32>(_e34.g0_.y, _e37.g0_.z, _e40.g0_.x, _e43.g0_.y)) * vec4<f32>(1.0, 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e56.g0_.w) * vec4<f32>(_e60.g0_.z, _e63.g0_.y, _e66.g0_.z, _e69.g0_.x)) * vec4<f32>(1.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e83.g0_.x) * vec4<f32>(_e87.g0_.x, _e90.g0_.x, _e93.g0_.y, _e96.g0_.z)) * vec4<f32>(1.0, 0.0, 1.0, 1.0))));
}

fn motor_dual_translator_inner_anti_product(self_2280: MotorDual, other_2000: Translator) -> Motor {
    var self_2281: MotorDual;
    var other_2001: Translator;

    self_2281 = self_2280;
    other_2001 = other_2000;
    let _e4: MotorDual = self_2281;
    let _e8: Translator = other_2001;
    let _e11: Translator = other_2001;
    let _e14: Translator = other_2001;
    let _e17: Translator = other_2001;
    let _e30: MotorDual = self_2281;
    let _e34: Translator = other_2001;
    let _e37: Translator = other_2001;
    let _e40: Translator = other_2001;
    let _e43: Translator = other_2001;
    let _e56: MotorDual = self_2281;
    let _e60: Translator = other_2001;
    let _e63: Translator = other_2001;
    let _e66: Translator = other_2001;
    let _e69: Translator = other_2001;
    let _e83: MotorDual = self_2281;
    let _e87: Translator = other_2001;
    let _e90: Translator = other_2001;
    let _e93: Translator = other_2001;
    let _e96: Translator = other_2001;
    return Motor((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.x, _e11.g0_.x, _e14.g0_.z, _e17.g0_.y)) * vec4<f32>(0.0, -(1.0), -(1.0), 1.0)) + ((vec4<f32>(_e30.g0_.z) * vec4<f32>(_e34.g0_.z, _e37.g0_.z, _e40.g0_.x, _e43.g0_.z)) * vec4<f32>(0.0, 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e56.g0_.w) * vec4<f32>(_e60.g0_.y, _e63.g0_.y, _e66.g0_.y, _e69.g0_.x)) * vec4<f32>(0.0, -(1.0), 0.0, -(1.0)))) + ((vec4<f32>(_e83.g0_.x) * vec4<f32>(_e87.g0_.x, _e90.g0_.x, _e93.g0_.y, _e96.g0_.z)) * vec4<f32>(1.0, 0.0, 1.0, 1.0))));
}

fn motor_dual_translator_left_contraction(self_2282: MotorDual, other_2002: Translator) -> Plane {
    var self_2283: MotorDual;
    var other_2003: Translator;

    self_2283 = self_2282;
    other_2003 = other_2002;
    let _e4: MotorDual = self_2283;
    let _e8: Translator = other_2003;
    let _e19: MotorDual = self_2283;
    let _e22: MotorDual = self_2283;
    let _e25: MotorDual = self_2283;
    let _e29: Translator = other_2003;
    return Plane((((vec3<f32>(_e4.g0_.w) * vec3<f32>(_e8.g0_.y)) * vec3<f32>(-(1.0), 0.0, 0.0)) + ((vec3<f32>(_e19.g0_.z, _e22.g0_.y, _e25.g0_.y) * _e29.g0_.zzy) * vec3<f32>(1.0, -(1.0), 1.0))));
}

fn motor_dual_translator_right_contraction(self_2284: MotorDual, other_2004: Translator) -> MotorDual {
    var self_2285: MotorDual;
    var other_2005: Translator;

    self_2285 = self_2284;
    other_2005 = other_2004;
    let _e4: MotorDual = self_2285;
    let _e8: Translator = other_2005;
    let _e19: MotorDual = self_2285;
    let _e23: Translator = other_2005;
    let _e35: MotorDual = self_2285;
    let _e38: Translator = other_2005;
    let _e41: Translator = other_2005;
    let _e44: Translator = other_2005;
    let _e47: Translator = other_2005;
    return MotorDual(((((vec4<f32>(_e4.g0_.z) * vec4<f32>(_e8.g0_.x)) * vec4<f32>(0.0, 0.0, 1.0, 0.0)) + ((vec4<f32>(_e19.g0_.w) * vec4<f32>(_e23.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((_e35.g0_.xyxx * vec4<f32>(_e38.g0_.x, _e41.g0_.x, _e44.g0_.y, _e47.g0_.z)) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))));
}

fn motor_dual_translator_left_anti_contraction(self_2286: MotorDual, other_2006: Translator) -> Motor {
    var self_2287: MotorDual;
    var other_2007: Translator;

    self_2287 = self_2286;
    other_2007 = other_2006;
    let _e4: MotorDual = self_2287;
    let _e8: Translator = other_2007;
    let _e20: MotorDual = self_2287;
    let _e24: Translator = other_2007;
    let _e37: MotorDual = self_2287;
    let _e40: Translator = other_2007;
    let _e43: Translator = other_2007;
    let _e46: Translator = other_2007;
    let _e49: Translator = other_2007;
    return Motor(((((vec4<f32>(_e4.g0_.z) * vec4<f32>(_e8.g0_.x)) * vec4<f32>(0.0, 0.0, -(1.0), 0.0)) + ((vec4<f32>(_e20.g0_.w) * vec4<f32>(_e24.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((_e37.g0_.xyxx * vec4<f32>(_e40.g0_.x, _e43.g0_.x, _e46.g0_.y, _e49.g0_.z)) * vec4<f32>(1.0, -(1.0), 1.0, 1.0))));
}

fn motor_dual_translator_right_anti_contraction(self_2288: MotorDual, other_2008: Translator) -> Point {
    var self_2289: MotorDual;
    var other_2009: Translator;

    self_2289 = self_2288;
    other_2009 = other_2008;
    let _e4: MotorDual = self_2289;
    let _e8: Translator = other_2009;
    let _e19: MotorDual = self_2289;
    let _e22: MotorDual = self_2289;
    let _e25: MotorDual = self_2289;
    let _e29: Translator = other_2009;
    return Point((((vec3<f32>(_e4.g0_.w) * vec3<f32>(_e8.g0_.y)) * vec3<f32>(-(1.0), 0.0, 0.0)) + ((vec3<f32>(_e19.g0_.z, _e22.g0_.y, _e25.g0_.y) * _e29.g0_.zzy) * vec3<f32>(1.0, -(1.0), 1.0))));
}

fn motor_dual_motor_add(self_2290: MotorDual, other_2010: Motor) -> MultiVector {
    var self_2291: MotorDual;
    var other_2011: Motor;

    self_2291 = self_2290;
    other_2011 = other_2010;
    let _e4: MotorDual = self_2291;
    let _e13: Motor = other_2011;
    let _e23: MotorDual = self_2291;
    let _e32: Motor = other_2011;
    return MultiVector(((_e4.g0_.xxwz * vec4<f32>(0.0, 0.0, 1.0, 1.0)) + (_e13.g0_.xyxx * vec4<f32>(1.0, 1.0, 0.0, 0.0))), ((_e23.g0_.yxxx * vec4<f32>(1.0, 1.0, 0.0, 0.0)) + (_e32.g0_.xxzw * vec4<f32>(0.0, 0.0, 1.0, 1.0))));
}

fn motor_dual_motor_sub(self_2292: MotorDual, other_2012: Motor) -> MultiVector {
    var self_2293: MotorDual;
    var other_2013: Motor;

    self_2293 = self_2292;
    other_2013 = other_2012;
    let _e4: MotorDual = self_2293;
    let _e13: Motor = other_2013;
    let _e23: MotorDual = self_2293;
    let _e32: Motor = other_2013;
    return MultiVector(((_e4.g0_.xxwz * vec4<f32>(0.0, 0.0, 1.0, 1.0)) - (_e13.g0_.xyxx * vec4<f32>(1.0, 1.0, 0.0, 0.0))), ((_e23.g0_.yxxx * vec4<f32>(1.0, 1.0, 0.0, 0.0)) - (_e32.g0_.xxzw * vec4<f32>(0.0, 0.0, 1.0, 1.0))));
}

fn motor_dual_motor_geometric_product(self_2294: MotorDual, other_2014: Motor) -> MotorDual {
    var self_2295: MotorDual;
    var other_2015: Motor;

    self_2295 = self_2294;
    other_2015 = other_2014;
    let _e4: MotorDual = self_2295;
    let _e8: Motor = other_2015;
    let _e20: MotorDual = self_2295;
    let _e24: Motor = other_2015;
    let _e36: MotorDual = self_2295;
    let _e40: Motor = other_2015;
    let _e52: MotorDual = self_2295;
    let _e56: Motor = other_2015;
    return MotorDual((((((vec4<f32>(_e4.g0_.x) * _e8.g0_) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))) + ((vec4<f32>(_e20.g0_.y) * _e24.g0_.yxwz) * vec4<f32>(1.0, 1.0, -(1.0), 1.0))) + ((vec4<f32>(_e36.g0_.z) * _e40.g0_.zwxy) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e52.g0_.w) * _e56.g0_.wzyx) * vec4<f32>(1.0, -(1.0), 1.0, 1.0))));
}

fn motor_dual_motor_regressive_product(self_2296: MotorDual, other_2016: Motor) -> Motor {
    var self_2297: MotorDual;
    var other_2017: Motor;

    self_2297 = self_2296;
    other_2017 = other_2016;
    let _e4: MotorDual = self_2297;
    let _e8: Motor = other_2017;
    let _e11: MotorDual = self_2297;
    let _e15: Motor = other_2017;
    let _e27: MotorDual = self_2297;
    let _e31: Motor = other_2017;
    let _e43: MotorDual = self_2297;
    let _e46: Motor = other_2017;
    return Motor(((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.z) * vec4<f32>(_e15.g0_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e27.g0_.w) * vec4<f32>(_e31.g0_.w)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((_e43.g0_.yxxx * _e46.g0_.yxxx) * vec4<f32>(1.0, 0.0, 0.0, 0.0))));
}

fn motor_dual_motor_outer_product(self_2298: MotorDual, other_2018: Motor) -> MotorDual {
    var self_2299: MotorDual;
    var other_2019: Motor;

    self_2299 = self_2298;
    other_2019 = other_2018;
    let _e4: MotorDual = self_2299;
    let _e8: Motor = other_2019;
    let _e18: MotorDual = self_2299;
    let _e22: Motor = other_2019;
    let _e33: MotorDual = self_2299;
    let _e37: Motor = other_2019;
    let _e48: MotorDual = self_2299;
    let _e52: Motor = other_2019;
    return MotorDual((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.yxyy) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) + ((vec4<f32>(_e18.g0_.z) * _e22.g0_.zzxz) * vec4<f32>(1.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e33.g0_.w) * _e37.g0_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e48.g0_.x) * vec4<f32>(_e52.g0_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))));
}

fn motor_dual_motor_inner_product(self_2300: MotorDual, other_2020: Motor) -> MotorDual {
    var self_2301: MotorDual;
    var other_2021: Motor;

    self_2301 = self_2300;
    other_2021 = other_2020;
    let _e4: MotorDual = self_2301;
    let _e8: Motor = other_2021;
    let _e20: MotorDual = self_2301;
    let _e24: Motor = other_2021;
    let _e36: MotorDual = self_2301;
    let _e40: Motor = other_2021;
    let _e52: MotorDual = self_2301;
    let _e55: Motor = other_2021;
    return MotorDual((((((vec4<f32>(_e4.g0_.x) * _e8.g0_) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))) + ((vec4<f32>(_e20.g0_.z) * _e24.g0_.wwxy) * vec4<f32>(0.0, 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e36.g0_.w) * _e40.g0_.zzyx) * vec4<f32>(0.0, -(1.0), 1.0, 1.0))) + ((_e52.g0_.xyyy * _e55.g0_.xxwz) * vec4<f32>(0.0, 1.0, -(1.0), 1.0))));
}

fn motor_dual_motor_geometric_anti_product(self_2302: MotorDual, other_2022: Motor) -> Motor {
    var self_2303: MotorDual;
    var other_2023: Motor;

    self_2303 = self_2302;
    other_2023 = other_2022;
    let _e4: MotorDual = self_2303;
    let _e8: Motor = other_2023;
    let _e11: MotorDual = self_2303;
    let _e15: Motor = other_2023;
    let _e28: MotorDual = self_2303;
    let _e32: Motor = other_2023;
    let _e45: MotorDual = self_2303;
    let _e49: Motor = other_2023;
    return Motor(((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.y) * _e15.g0_.yxwz) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e28.g0_.z) * _e32.g0_.zwxy) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e45.g0_.w) * _e49.g0_.wzyx) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))));
}

fn motor_dual_motor_inner_anti_product(self_2304: MotorDual, other_2024: Motor) -> Motor {
    var self_2305: MotorDual;
    var other_2025: Motor;

    self_2305 = self_2304;
    other_2025 = other_2024;
    let _e4: MotorDual = self_2305;
    let _e8: Motor = other_2025;
    let _e11: MotorDual = self_2305;
    let _e15: Motor = other_2025;
    let _e28: MotorDual = self_2305;
    let _e32: Motor = other_2025;
    let _e45: MotorDual = self_2305;
    let _e48: Motor = other_2025;
    return Motor(((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.z) * _e15.g0_.wwxy) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e28.g0_.w) * _e32.g0_.zzyx) * vec4<f32>(0.0, -(1.0), 1.0, -(1.0)))) + ((_e45.g0_.xyyy * _e48.g0_.xxwz) * vec4<f32>(0.0, -(1.0), -(1.0), 1.0))));
}

fn motor_dual_motor_left_contraction(self_2306: MotorDual, other_2026: Motor) -> Plane {
    var self_2307: MotorDual;
    var other_2027: Motor;

    self_2307 = self_2306;
    other_2027 = other_2026;
    let _e4: MotorDual = self_2307;
    let _e8: Motor = other_2027;
    let _e11: Motor = other_2027;
    let _e14: Motor = other_2027;
    let _e25: MotorDual = self_2307;
    let _e29: Motor = other_2027;
    let _e32: Motor = other_2027;
    let _e35: Motor = other_2027;
    let _e47: MotorDual = self_2307;
    let _e50: MotorDual = self_2307;
    let _e53: MotorDual = self_2307;
    let _e57: Motor = other_2027;
    let _e60: Motor = other_2027;
    let _e63: Motor = other_2027;
    return Plane(((((vec3<f32>(_e4.g0_.z) * vec3<f32>(_e8.g0_.w, _e11.g0_.w, _e14.g0_.y)) * vec3<f32>(1.0, 0.0, -(1.0))) + ((vec3<f32>(_e25.g0_.w) * vec3<f32>(_e29.g0_.z, _e32.g0_.y, _e35.g0_.z)) * vec3<f32>(-(1.0), 1.0, 0.0))) + ((vec3<f32>(_e47.g0_.x, _e50.g0_.y, _e53.g0_.y) * vec3<f32>(_e57.g0_.x, _e60.g0_.w, _e63.g0_.z)) * vec3<f32>(0.0, -(1.0), 1.0))));
}

fn motor_dual_motor_right_contraction(self_2308: MotorDual, other_2028: Motor) -> MotorDual {
    var self_2309: MotorDual;
    var other_2029: Motor;

    self_2309 = self_2308;
    other_2029 = other_2028;
    let _e4: MotorDual = self_2309;
    let _e8: Motor = other_2029;
    let _e20: MotorDual = self_2309;
    let _e22: Motor = other_2029;
    return MotorDual((((vec4<f32>(_e4.g0_.x) * _e8.g0_) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))) + ((_e20.g0_ * vec4<f32>(_e22.g0_.x)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn motor_dual_motor_left_anti_contraction(self_2310: MotorDual, other_2030: Motor) -> Motor {
    var self_2311: MotorDual;
    var other_2031: Motor;

    self_2311 = self_2310;
    other_2031 = other_2030;
    let _e4: MotorDual = self_2311;
    let _e8: Motor = other_2031;
    let _e11: MotorDual = self_2311;
    let _e13: Motor = other_2031;
    return Motor(((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((_e11.g0_ * vec4<f32>(_e13.g0_.x)) * vec4<f32>(0.0, -(1.0), -(1.0), -(1.0)))));
}

fn motor_dual_motor_right_anti_contraction(self_2312: MotorDual, other_2032: Motor) -> Point {
    var self_2313: MotorDual;
    var other_2033: Motor;

    self_2313 = self_2312;
    other_2033 = other_2032;
    let _e4: MotorDual = self_2313;
    let _e8: Motor = other_2033;
    let _e11: Motor = other_2033;
    let _e14: Motor = other_2033;
    let _e25: MotorDual = self_2313;
    let _e29: Motor = other_2033;
    let _e32: Motor = other_2033;
    let _e35: Motor = other_2033;
    let _e47: MotorDual = self_2313;
    let _e50: MotorDual = self_2313;
    let _e53: MotorDual = self_2313;
    let _e57: Motor = other_2033;
    let _e60: Motor = other_2033;
    let _e63: Motor = other_2033;
    return Point(((((vec3<f32>(_e4.g0_.z) * vec3<f32>(_e8.g0_.w, _e11.g0_.w, _e14.g0_.y)) * vec3<f32>(1.0, 0.0, -(1.0))) + ((vec3<f32>(_e25.g0_.w) * vec3<f32>(_e29.g0_.z, _e32.g0_.y, _e35.g0_.z)) * vec3<f32>(-(1.0), 1.0, 0.0))) + ((vec3<f32>(_e47.g0_.x, _e50.g0_.y, _e53.g0_.y) * vec3<f32>(_e57.g0_.x, _e60.g0_.w, _e63.g0_.z)) * vec3<f32>(0.0, -(1.0), 1.0))));
}

fn motor_dual_motor_dual_add(self_2314: MotorDual, other_2034: MotorDual) -> MotorDual {
    var self_2315: MotorDual;
    var other_2035: MotorDual;

    self_2315 = self_2314;
    other_2035 = other_2034;
    let _e4: MotorDual = self_2315;
    let _e6: MotorDual = other_2035;
    return MotorDual((_e4.g0_ + _e6.g0_));
}

fn motor_dual_motor_dual_sub(self_2316: MotorDual, other_2036: MotorDual) -> MotorDual {
    var self_2317: MotorDual;
    var other_2037: MotorDual;

    self_2317 = self_2316;
    other_2037 = other_2036;
    let _e4: MotorDual = self_2317;
    let _e6: MotorDual = other_2037;
    return MotorDual((_e4.g0_ - _e6.g0_));
}

fn motor_dual_motor_dual_mul(self_2318: MotorDual, other_2038: MotorDual) -> MotorDual {
    var self_2319: MotorDual;
    var other_2039: MotorDual;

    self_2319 = self_2318;
    other_2039 = other_2038;
    let _e4: MotorDual = self_2319;
    let _e6: MotorDual = other_2039;
    return MotorDual((_e4.g0_ * _e6.g0_));
}

fn motor_dual_motor_dual_div(self_2320: MotorDual, other_2040: MotorDual) -> MotorDual {
    var self_2321: MotorDual;
    var other_2041: MotorDual;

    self_2321 = self_2320;
    other_2041 = other_2040;
    let _e4: MotorDual = self_2321;
    let _e7: MotorDual = self_2321;
    let _e10: MotorDual = self_2321;
    let _e13: MotorDual = self_2321;
    let _e23: MotorDual = other_2041;
    let _e26: MotorDual = other_2041;
    let _e29: MotorDual = other_2041;
    let _e32: MotorDual = other_2041;
    return MotorDual((((vec4<f32>(_e4.g0_.x, _e7.g0_.y, _e10.g0_.z, _e13.g0_.w) * vec4<f32>(1.0, 1.0, 1.0, 1.0)) / vec4<f32>(_e23.g0_.x, _e26.g0_.y, _e29.g0_.z, _e32.g0_.w)) * vec4<f32>(1.0, 1.0, 1.0, 1.0)));
}

fn motor_dual_motor_dual_geometric_product(self_2322: MotorDual, other_2042: MotorDual) -> Motor {
    var self_2323: MotorDual;
    var other_2043: MotorDual;

    self_2323 = self_2322;
    other_2043 = other_2042;
    let _e4: MotorDual = self_2323;
    let _e8: MotorDual = other_2043;
    let _e18: MotorDual = self_2323;
    let _e22: MotorDual = other_2043;
    let _e34: MotorDual = self_2323;
    let _e38: MotorDual = other_2043;
    let _e50: MotorDual = self_2323;
    let _e54: MotorDual = other_2043;
    return Motor((((((vec4<f32>(_e4.g0_.x) * _e8.g0_) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0)) + ((vec4<f32>(_e18.g0_.y) * _e22.g0_.yxwz) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e34.g0_.z) * _e38.g0_.zwxy) * vec4<f32>(1.0, -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e50.g0_.w) * _e54.g0_.wzyx) * vec4<f32>(1.0, 1.0, -(1.0), 1.0))));
}

fn motor_dual_motor_dual_regressive_product(self_2324: MotorDual, other_2044: MotorDual) -> MotorDual {
    var self_2325: MotorDual;
    var other_2045: MotorDual;

    self_2325 = self_2324;
    other_2045 = other_2044;
    let _e4: MotorDual = self_2325;
    let _e8: MotorDual = other_2045;
    let _e11: MotorDual = self_2325;
    let _e13: MotorDual = other_2045;
    return MotorDual(((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((_e11.g0_ * vec4<f32>(_e13.g0_.x)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn motor_dual_motor_dual_outer_product(self_2326: MotorDual, other_2046: MotorDual) -> Point {
    var self_2327: MotorDual;
    var other_2047: MotorDual;

    self_2327 = self_2326;
    other_2047 = other_2046;
    let _e4: MotorDual = self_2327;
    let _e8: MotorDual = other_2047;
    let _e11: MotorDual = other_2047;
    let _e14: MotorDual = other_2047;
    let _e25: MotorDual = self_2327;
    let _e29: MotorDual = other_2047;
    let _e32: MotorDual = other_2047;
    let _e35: MotorDual = other_2047;
    let _e47: MotorDual = self_2327;
    let _e50: MotorDual = self_2327;
    let _e53: MotorDual = self_2327;
    let _e57: MotorDual = other_2047;
    let _e60: MotorDual = other_2047;
    let _e63: MotorDual = other_2047;
    return Point(((((vec3<f32>(_e4.g0_.z) * vec3<f32>(_e8.g0_.w, _e11.g0_.w, _e14.g0_.y)) * vec3<f32>(-(1.0), 0.0, 1.0)) + ((vec3<f32>(_e25.g0_.w) * vec3<f32>(_e29.g0_.z, _e32.g0_.y, _e35.g0_.z)) * vec3<f32>(1.0, -(1.0), 0.0))) + ((vec3<f32>(_e47.g0_.x, _e50.g0_.y, _e53.g0_.y) * vec3<f32>(_e57.g0_.x, _e60.g0_.w, _e63.g0_.z)) * vec3<f32>(0.0, 1.0, -(1.0)))));
}

fn motor_dual_motor_dual_inner_product(self_2328: MotorDual, other_2048: MotorDual) -> Motor {
    var self_2329: MotorDual;
    var other_2049: MotorDual;

    self_2329 = self_2328;
    other_2049 = other_2048;
    let _e4: MotorDual = self_2329;
    let _e8: MotorDual = other_2049;
    let _e18: MotorDual = self_2329;
    let _e22: MotorDual = other_2049;
    let _e33: MotorDual = self_2329;
    let _e37: MotorDual = other_2049;
    let _e48: MotorDual = self_2329;
    let _e51: MotorDual = other_2049;
    return Motor((((((vec4<f32>(_e4.g0_.x) * _e8.g0_) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0)) + ((vec4<f32>(_e18.g0_.z) * _e22.g0_.zzxz) * vec4<f32>(1.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e33.g0_.w) * _e37.g0_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, 1.0))) + ((_e48.g0_.yyxx * _e51.g0_.yxxx) * vec4<f32>(1.0, 1.0, 0.0, 0.0))));
}

fn motor_dual_motor_dual_geometric_anti_product(self_2330: MotorDual, other_2050: MotorDual) -> MotorDual {
    var self_2331: MotorDual;
    var other_2051: MotorDual;

    self_2331 = self_2330;
    other_2051 = other_2050;
    let _e4: MotorDual = self_2331;
    let _e8: MotorDual = other_2051;
    let _e11: MotorDual = self_2331;
    let _e15: MotorDual = other_2051;
    let _e28: MotorDual = self_2331;
    let _e32: MotorDual = other_2051;
    let _e45: MotorDual = self_2331;
    let _e49: MotorDual = other_2051;
    return MotorDual(((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.y) * _e15.g0_.yxwz) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))) + ((vec4<f32>(_e28.g0_.z) * _e32.g0_.zwxy) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e45.g0_.w) * _e49.g0_.wzyx) * vec4<f32>(-(1.0), -(1.0), 1.0, 1.0))));
}

fn motor_dual_motor_dual_inner_anti_product(self_2332: MotorDual, other_2052: MotorDual) -> MotorDual {
    var self_2333: MotorDual;
    var other_2053: MotorDual;

    self_2333 = self_2332;
    other_2053 = other_2052;
    let _e4: MotorDual = self_2333;
    let _e8: MotorDual = other_2053;
    let _e11: MotorDual = self_2333;
    let _e15: MotorDual = other_2053;
    let _e27: MotorDual = self_2333;
    let _e31: MotorDual = other_2053;
    let _e43: MotorDual = self_2333;
    let _e46: MotorDual = other_2053;
    return MotorDual(((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.z) * _e15.g0_.zzxz) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0))) + ((vec4<f32>(_e27.g0_.w) * _e31.g0_.wwwx) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))) + ((_e43.g0_.yyxx * _e46.g0_.yxxx) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))));
}

fn motor_dual_motor_dual_left_contraction(self_2334: MotorDual, other_2054: MotorDual) -> Motor {
    var self_2335: MotorDual;
    var other_2055: MotorDual;

    self_2335 = self_2334;
    other_2055 = other_2054;
    let _e4: MotorDual = self_2335;
    let _e8: MotorDual = other_2055;
    let _e18: MotorDual = self_2335;
    let _e22: MotorDual = other_2055;
    let _e33: MotorDual = self_2335;
    let _e37: MotorDual = other_2055;
    let _e48: MotorDual = self_2335;
    let _e52: MotorDual = other_2055;
    return Motor((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.yxyy) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) + ((vec4<f32>(_e18.g0_.z) * _e22.g0_.zzxz) * vec4<f32>(1.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e33.g0_.w) * _e37.g0_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e48.g0_.x) * vec4<f32>(_e52.g0_.x)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))));
}

fn motor_dual_motor_dual_right_contraction(self_2336: MotorDual, other_2056: MotorDual) -> Motor {
    var self_2337: MotorDual;
    var other_2057: MotorDual;

    self_2337 = self_2336;
    other_2057 = other_2056;
    let _e4: MotorDual = self_2337;
    let _e8: MotorDual = other_2057;
    let _e18: MotorDual = self_2337;
    let _e22: MotorDual = other_2057;
    let _e34: MotorDual = self_2337;
    let _e38: MotorDual = other_2057;
    let _e50: MotorDual = self_2337;
    let _e53: MotorDual = other_2057;
    return Motor((((((vec4<f32>(_e4.g0_.x) * _e8.g0_) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0)) + ((vec4<f32>(_e18.g0_.z) * vec4<f32>(_e22.g0_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e34.g0_.w) * vec4<f32>(_e38.g0_.w)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((_e50.g0_.yxxx * _e53.g0_.yxxx) * vec4<f32>(1.0, 0.0, 0.0, 0.0))));
}

fn motor_dual_motor_dual_left_anti_contraction(self_2338: MotorDual, other_2058: MotorDual) -> MotorDual {
    var self_2339: MotorDual;
    var other_2059: MotorDual;

    self_2339 = self_2338;
    other_2059 = other_2058;
    let _e4: MotorDual = self_2339;
    let _e8: MotorDual = other_2059;
    let _e11: MotorDual = self_2339;
    let _e15: MotorDual = other_2059;
    let _e28: MotorDual = self_2339;
    let _e32: MotorDual = other_2059;
    let _e45: MotorDual = self_2339;
    let _e48: MotorDual = other_2059;
    return MotorDual(((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.z) * vec4<f32>(_e15.g0_.z)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((vec4<f32>(_e28.g0_.w) * vec4<f32>(_e32.g0_.w)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((_e45.g0_.yxxx * _e48.g0_.yxxx) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))));
}

fn motor_dual_motor_dual_right_anti_contraction(self_2340: MotorDual, other_2060: MotorDual) -> MotorDual {
    var self_2341: MotorDual;
    var other_2061: MotorDual;

    self_2341 = self_2340;
    other_2061 = other_2060;
    let _e4: MotorDual = self_2341;
    let _e8: MotorDual = other_2061;
    let _e19: MotorDual = self_2341;
    let _e23: MotorDual = other_2061;
    let _e35: MotorDual = self_2341;
    let _e39: MotorDual = other_2061;
    let _e51: MotorDual = self_2341;
    let _e55: MotorDual = other_2061;
    return MotorDual((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0)) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.zzxz) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0))) + ((vec4<f32>(_e35.g0_.w) * _e39.g0_.wwwx) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))) + ((vec4<f32>(_e51.g0_.x) * vec4<f32>(_e55.g0_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))));
}

fn motor_dual_motor_dual_scalar_product(self_2342: MotorDual, other_2062: MotorDual) -> Scalar {
    var self_2343: MotorDual;
    var other_2063: MotorDual;

    self_2343 = self_2342;
    other_2063 = other_2062;
    let _e5: MotorDual = self_2343;
    let _e8: MotorDual = other_2063;
    let _e13: MotorDual = self_2343;
    let _e16: MotorDual = other_2063;
    let _e21: MotorDual = self_2343;
    let _e24: MotorDual = other_2063;
    let _e29: MotorDual = self_2343;
    let _e32: MotorDual = other_2063;
    return Scalar(((((0.0 - (_e5.g0_.x * _e8.g0_.x)) + (_e13.g0_.y * _e16.g0_.y)) + (_e21.g0_.z * _e24.g0_.z)) + (_e29.g0_.w * _e32.g0_.w)));
}

fn motor_dual_motor_dual_anti_scalar_product(self_2344: MotorDual, other_2064: MotorDual) -> AntiScalar {
    var self_2345: MotorDual;
    var other_2065: MotorDual;

    self_2345 = self_2344;
    other_2065 = other_2064;
    let _e4: MotorDual = self_2345;
    let _e7: MotorDual = other_2065;
    let _e11: MotorDual = self_2345;
    let _e14: MotorDual = other_2065;
    let _e19: MotorDual = self_2345;
    let _e22: MotorDual = other_2065;
    let _e27: MotorDual = self_2345;
    let _e30: MotorDual = other_2065;
    return AntiScalar(((((_e4.g0_.x * _e7.g0_.x) - (_e11.g0_.y * _e14.g0_.y)) - (_e19.g0_.z * _e22.g0_.z)) - (_e27.g0_.w * _e30.g0_.w)));
}

fn motor_dual_squared_magnitude(self_2346: MotorDual) -> Scalar {
    var self_2347: MotorDual;

    self_2347 = self_2346;
    let _e2: MotorDual = self_2347;
    let _e3: MotorDual = self_2347;
    let _e4: MotorDual = motor_dual_reversal(_e3);
    let _e5: Scalar = motor_dual_motor_dual_scalar_product(_e2, _e4);
    return _e5;
}

fn motor_dual_magnitude(self_2348: MotorDual) -> Scalar {
    var self_2349: MotorDual;

    self_2349 = self_2348;
    let _e2: MotorDual = self_2349;
    let _e3: Scalar = motor_dual_squared_magnitude(_e2);
    return Scalar(sqrt(_e3.g0_));
}

fn motor_dual_bulk_norm(self_2350: MotorDual) -> Scalar {
    var self_2351: MotorDual;

    self_2351 = self_2350;
    let _e2: MotorDual = self_2351;
    let _e3: Scalar = motor_dual_squared_magnitude(_e2);
    return Scalar(sqrt(_e3.g0_));
}

fn motor_dual_squared_anti_magnitude(self_2352: MotorDual) -> AntiScalar {
    var self_2353: MotorDual;

    self_2353 = self_2352;
    let _e2: MotorDual = self_2353;
    let _e3: MotorDual = self_2353;
    let _e4: MotorDual = motor_dual_anti_reversal(_e3);
    let _e5: AntiScalar = motor_dual_motor_dual_anti_scalar_product(_e2, _e4);
    return _e5;
}

fn motor_dual_weight_norm(self_2354: MotorDual) -> AntiScalar {
    var self_2355: MotorDual;

    self_2355 = self_2354;
    let _e2: MotorDual = self_2355;
    let _e3: AntiScalar = motor_dual_squared_anti_magnitude(_e2);
    return AntiScalar(sqrt(_e3.g0_));
}

fn motor_dual_scale(self_2356: MotorDual, other_2066: f32) -> MotorDual {
    var self_2357: MotorDual;
    var other_2067: f32;

    self_2357 = self_2356;
    other_2067 = other_2066;
    let _e4: MotorDual = self_2357;
    let _e5: f32 = other_2067;
    let _e7: MotorDual = motor_dual_scalar_geometric_product(_e4, Scalar(_e5));
    return _e7;
}

fn motor_dual_signum(self_2358: MotorDual) -> MotorDual {
    var self_2359: MotorDual;

    self_2359 = self_2358;
    let _e2: MotorDual = self_2359;
    let _e3: MotorDual = self_2359;
    let _e4: Scalar = motor_dual_magnitude(_e3);
    let _e9: MotorDual = motor_dual_scalar_geometric_product(_e2, Scalar((1.0 / _e4.g0_)));
    return _e9;
}

fn motor_dual_inverse(self_2360: MotorDual) -> MotorDual {
    var self_2361: MotorDual;

    self_2361 = self_2360;
    let _e2: MotorDual = self_2361;
    let _e3: MotorDual = motor_dual_reversal(_e2);
    let _e4: MotorDual = self_2361;
    let _e5: Scalar = motor_dual_squared_magnitude(_e4);
    let _e10: MotorDual = motor_dual_scalar_geometric_product(_e3, Scalar((1.0 / _e5.g0_)));
    return _e10;
}

fn anti_scalar_anti_scalar_geometric_quotient(self_2362: AntiScalar, other_2068: AntiScalar) -> Scalar {
    var self_2363: AntiScalar;
    var other_2069: AntiScalar;

    self_2363 = self_2362;
    other_2069 = other_2068;
    let _e4: AntiScalar = self_2363;
    let _e5: AntiScalar = other_2069;
    let _e6: AntiScalar = anti_scalar_inverse(_e5);
    let _e7: Scalar = anti_scalar_anti_scalar_geometric_product(_e4, _e6);
    return _e7;
}

fn anti_scalar_anti_scalar_transformation(self_2364: AntiScalar, other_2070: AntiScalar) -> AntiScalar {
    var self_2365: AntiScalar;
    var other_2071: AntiScalar;

    self_2365 = self_2364;
    other_2071 = other_2070;
    let _e4: AntiScalar = self_2365;
    let _e5: AntiScalar = other_2071;
    let _e6: Scalar = anti_scalar_anti_scalar_geometric_product(_e4, _e5);
    let _e7: AntiScalar = self_2365;
    let _e8: AntiScalar = anti_scalar_reversal(_e7);
    let _e9: AntiScalar = scalar_anti_scalar_geometric_product(_e6, _e8);
    return _e9;
}

fn anti_scalar_motor_geometric_quotient(self_2366: AntiScalar, other_2072: Motor) -> MotorDual {
    var self_2367: AntiScalar;
    var other_2073: Motor;

    self_2367 = self_2366;
    other_2073 = other_2072;
    let _e4: AntiScalar = self_2367;
    let _e5: Motor = other_2073;
    let _e6: Motor = motor_inverse(_e5);
    let _e7: MotorDual = anti_scalar_motor_geometric_product(_e4, _e6);
    return _e7;
}

fn anti_scalar_motor_transformation(self_2368: AntiScalar, other_2074: Motor) -> Motor {
    var self_2369: AntiScalar;
    var other_2075: Motor;

    self_2369 = self_2368;
    other_2075 = other_2074;
    let _e4: AntiScalar = self_2369;
    let _e5: Motor = other_2075;
    let _e6: MotorDual = anti_scalar_motor_geometric_product(_e4, _e5);
    let _e7: AntiScalar = self_2369;
    let _e8: AntiScalar = anti_scalar_reversal(_e7);
    let _e9: Motor = motor_dual_anti_scalar_geometric_product(_e6, _e8);
    return _e9;
}

fn anti_scalar_motor_dual_geometric_quotient(self_2370: AntiScalar, other_2076: MotorDual) -> Motor {
    var self_2371: AntiScalar;
    var other_2077: MotorDual;

    self_2371 = self_2370;
    other_2077 = other_2076;
    let _e4: AntiScalar = self_2371;
    let _e5: MotorDual = other_2077;
    let _e6: MotorDual = motor_dual_inverse(_e5);
    let _e7: Motor = anti_scalar_motor_dual_geometric_product(_e4, _e6);
    return _e7;
}

fn anti_scalar_motor_dual_transformation(self_2372: AntiScalar, other_2078: MotorDual) -> MotorDual {
    var self_2373: AntiScalar;
    var other_2079: MotorDual;

    self_2373 = self_2372;
    other_2079 = other_2078;
    let _e4: AntiScalar = self_2373;
    let _e5: MotorDual = other_2079;
    let _e6: Motor = anti_scalar_motor_dual_geometric_product(_e4, _e5);
    let _e7: AntiScalar = self_2373;
    let _e8: AntiScalar = anti_scalar_reversal(_e7);
    let _e9: MotorDual = motor_anti_scalar_geometric_product(_e6, _e8);
    return _e9;
}

fn anti_scalar_multi_vector_geometric_quotient(self_2374: AntiScalar, other_2080: MultiVector) -> MultiVector {
    var self_2375: AntiScalar;
    var other_2081: MultiVector;

    self_2375 = self_2374;
    other_2081 = other_2080;
    let _e4: AntiScalar = self_2375;
    let _e5: MultiVector = other_2081;
    let _e6: MultiVector = multi_vector_inverse(_e5);
    let _e7: MultiVector = anti_scalar_multi_vector_geometric_product(_e4, _e6);
    return _e7;
}

fn anti_scalar_multi_vector_transformation(self_2376: AntiScalar, other_2082: MultiVector) -> MultiVector {
    var self_2377: AntiScalar;
    var other_2083: MultiVector;

    self_2377 = self_2376;
    other_2083 = other_2082;
    let _e4: AntiScalar = self_2377;
    let _e5: MultiVector = other_2083;
    let _e6: MultiVector = anti_scalar_multi_vector_geometric_product(_e4, _e5);
    let _e7: AntiScalar = self_2377;
    let _e8: AntiScalar = anti_scalar_reversal(_e7);
    let _e9: MultiVector = multi_vector_anti_scalar_geometric_product(_e6, _e8);
    return _e9;
}

fn anti_scalar_plane_geometric_quotient(self_2378: AntiScalar, other_2084: Plane) -> Point {
    var self_2379: AntiScalar;
    var other_2085: Plane;

    self_2379 = self_2378;
    other_2085 = other_2084;
    let _e4: AntiScalar = self_2379;
    let _e5: Plane = other_2085;
    let _e6: Plane = plane_inverse(_e5);
    let _e7: Point = anti_scalar_plane_geometric_product(_e4, _e6);
    return _e7;
}

fn anti_scalar_plane_transformation(self_2380: AntiScalar, other_2086: Plane) -> Plane {
    var self_2381: AntiScalar;
    var other_2087: Plane;

    self_2381 = self_2380;
    other_2087 = other_2086;
    let _e4: AntiScalar = self_2381;
    let _e5: Plane = other_2087;
    let _e6: Point = anti_scalar_plane_geometric_product(_e4, _e5);
    let _e7: AntiScalar = self_2381;
    let _e8: AntiScalar = anti_scalar_reversal(_e7);
    let _e9: Plane = point_anti_scalar_geometric_product(_e6, _e8);
    return _e9;
}

fn anti_scalar_point_geometric_quotient(self_2382: AntiScalar, other_2088: Point) -> Plane {
    var self_2383: AntiScalar;
    var other_2089: Point;

    self_2383 = self_2382;
    other_2089 = other_2088;
    let _e4: AntiScalar = self_2383;
    let _e5: Point = other_2089;
    let _e6: Point = point_inverse(_e5);
    let _e7: Plane = anti_scalar_point_geometric_product(_e4, _e6);
    return _e7;
}

fn anti_scalar_point_transformation(self_2384: AntiScalar, other_2090: Point) -> Point {
    var self_2385: AntiScalar;
    var other_2091: Point;

    self_2385 = self_2384;
    other_2091 = other_2090;
    let _e4: AntiScalar = self_2385;
    let _e5: Point = other_2091;
    let _e6: Plane = anti_scalar_point_geometric_product(_e4, _e5);
    let _e7: AntiScalar = self_2385;
    let _e8: AntiScalar = anti_scalar_reversal(_e7);
    let _e9: Point = plane_anti_scalar_geometric_product(_e6, _e8);
    return _e9;
}

fn anti_scalar_scalar_geometric_quotient(self_2386: AntiScalar, other_2092: Scalar) -> AntiScalar {
    var self_2387: AntiScalar;
    var other_2093: Scalar;

    self_2387 = self_2386;
    other_2093 = other_2092;
    let _e4: AntiScalar = self_2387;
    let _e5: Scalar = other_2093;
    let _e6: Scalar = scalar_inverse(_e5);
    let _e7: AntiScalar = anti_scalar_scalar_geometric_product(_e4, _e6);
    return _e7;
}

fn anti_scalar_scalar_transformation(self_2388: AntiScalar, other_2094: Scalar) -> Scalar {
    var self_2389: AntiScalar;
    var other_2095: Scalar;

    self_2389 = self_2388;
    other_2095 = other_2094;
    let _e4: AntiScalar = self_2389;
    let _e5: Scalar = other_2095;
    let _e6: AntiScalar = anti_scalar_scalar_geometric_product(_e4, _e5);
    let _e7: AntiScalar = self_2389;
    let _e8: AntiScalar = anti_scalar_reversal(_e7);
    let _e9: Scalar = anti_scalar_anti_scalar_geometric_product(_e6, _e8);
    return _e9;
}

fn ideal_point_ideal_point_geometric_quotient(self_2390: IdealPoint, other_2096: IdealPoint) -> Rotor {
    var self_2391: IdealPoint;
    var other_2097: IdealPoint;

    self_2391 = self_2390;
    other_2097 = other_2096;
    let _e4: IdealPoint = self_2391;
    let _e5: IdealPoint = other_2097;
    let _e6: IdealPoint = ideal_point_inverse(_e5);
    let _e7: Rotor = ideal_point_ideal_point_geometric_product(_e4, _e6);
    return _e7;
}

fn ideal_point_ideal_point_transformation(self_2392: IdealPoint, other_2098: IdealPoint) -> IdealPoint {
    var self_2393: IdealPoint;
    var other_2099: IdealPoint;

    self_2393 = self_2392;
    other_2099 = other_2098;
    let _e4: IdealPoint = self_2393;
    let _e5: IdealPoint = other_2099;
    let _e6: Rotor = ideal_point_ideal_point_geometric_product(_e4, _e5);
    let _e7: IdealPoint = self_2393;
    let _e8: IdealPoint = ideal_point_reversal(_e7);
    let _e9: IdealPoint = rotor_ideal_point_geometric_product(_e6, _e8);
    return _e9;
}

fn ideal_point_motor_geometric_quotient(self_2394: IdealPoint, other_2100: Motor) -> Motor {
    var self_2395: IdealPoint;
    var other_2101: Motor;

    self_2395 = self_2394;
    other_2101 = other_2100;
    let _e4: IdealPoint = self_2395;
    let _e5: Motor = other_2101;
    let _e6: Motor = motor_inverse(_e5);
    let _e7: Motor = ideal_point_motor_geometric_product(_e4, _e6);
    return _e7;
}

fn ideal_point_motor_transformation(self_2396: IdealPoint, other_2102: Motor) -> Motor {
    var self_2397: IdealPoint;
    var other_2103: Motor;

    self_2397 = self_2396;
    other_2103 = other_2102;
    let _e4: IdealPoint = self_2397;
    let _e5: Motor = other_2103;
    let _e6: Motor = ideal_point_motor_geometric_product(_e4, _e5);
    let _e7: IdealPoint = self_2397;
    let _e8: IdealPoint = ideal_point_reversal(_e7);
    let _e9: Motor = motor_ideal_point_geometric_product(_e6, _e8);
    return _e9;
}

fn ideal_point_motor_dual_geometric_quotient(self_2398: IdealPoint, other_2104: MotorDual) -> MotorDual {
    var self_2399: IdealPoint;
    var other_2105: MotorDual;

    self_2399 = self_2398;
    other_2105 = other_2104;
    let _e4: IdealPoint = self_2399;
    let _e5: MotorDual = other_2105;
    let _e6: MotorDual = motor_dual_inverse(_e5);
    let _e7: MotorDual = ideal_point_motor_dual_geometric_product(_e4, _e6);
    return _e7;
}

fn ideal_point_motor_dual_transformation(self_2400: IdealPoint, other_2106: MotorDual) -> MotorDual {
    var self_2401: IdealPoint;
    var other_2107: MotorDual;

    self_2401 = self_2400;
    other_2107 = other_2106;
    let _e4: IdealPoint = self_2401;
    let _e5: MotorDual = other_2107;
    let _e6: MotorDual = ideal_point_motor_dual_geometric_product(_e4, _e5);
    let _e7: IdealPoint = self_2401;
    let _e8: IdealPoint = ideal_point_reversal(_e7);
    let _e9: MotorDual = motor_dual_ideal_point_geometric_product(_e6, _e8);
    return _e9;
}

fn ideal_point_multi_vector_geometric_quotient(self_2402: IdealPoint, other_2108: MultiVector) -> MultiVector {
    var self_2403: IdealPoint;
    var other_2109: MultiVector;

    self_2403 = self_2402;
    other_2109 = other_2108;
    let _e4: IdealPoint = self_2403;
    let _e5: MultiVector = other_2109;
    let _e6: MultiVector = multi_vector_inverse(_e5);
    let _e7: MultiVector = ideal_point_multi_vector_geometric_product(_e4, _e6);
    return _e7;
}

fn ideal_point_multi_vector_transformation(self_2404: IdealPoint, other_2110: MultiVector) -> MultiVector {
    var self_2405: IdealPoint;
    var other_2111: MultiVector;

    self_2405 = self_2404;
    other_2111 = other_2110;
    let _e4: IdealPoint = self_2405;
    let _e5: MultiVector = other_2111;
    let _e6: MultiVector = ideal_point_multi_vector_geometric_product(_e4, _e5);
    let _e7: IdealPoint = self_2405;
    let _e8: IdealPoint = ideal_point_reversal(_e7);
    let _e9: MultiVector = multi_vector_ideal_point_geometric_product(_e6, _e8);
    return _e9;
}

fn ideal_point_plane_geometric_quotient(self_2406: IdealPoint, other_2112: Plane) -> MotorDual {
    var self_2407: IdealPoint;
    var other_2113: Plane;

    self_2407 = self_2406;
    other_2113 = other_2112;
    let _e4: IdealPoint = self_2407;
    let _e5: Plane = other_2113;
    let _e6: Plane = plane_inverse(_e5);
    let _e7: MotorDual = ideal_point_plane_geometric_product(_e4, _e6);
    return _e7;
}

fn ideal_point_plane_transformation(self_2408: IdealPoint, other_2114: Plane) -> Plane {
    var self_2409: IdealPoint;
    var other_2115: Plane;

    self_2409 = self_2408;
    other_2115 = other_2114;
    let _e4: IdealPoint = self_2409;
    let _e5: Plane = other_2115;
    let _e6: MotorDual = ideal_point_plane_geometric_product(_e4, _e5);
    let _e7: IdealPoint = self_2409;
    let _e8: IdealPoint = ideal_point_reversal(_e7);
    let _e9: MotorDual = motor_dual_ideal_point_geometric_product(_e6, _e8);
    let _e10: Plane = motor_dual_plane_into(_e9);
    return _e10;
}

fn ideal_point_point_geometric_quotient(self_2410: IdealPoint, other_2116: Point) -> Motor {
    var self_2411: IdealPoint;
    var other_2117: Point;

    self_2411 = self_2410;
    other_2117 = other_2116;
    let _e4: IdealPoint = self_2411;
    let _e5: Point = other_2117;
    let _e6: Point = point_inverse(_e5);
    let _e7: Motor = ideal_point_point_geometric_product(_e4, _e6);
    return _e7;
}

fn ideal_point_point_transformation(self_2412: IdealPoint, other_2118: Point) -> Point {
    var self_2413: IdealPoint;
    var other_2119: Point;

    self_2413 = self_2412;
    other_2119 = other_2118;
    let _e4: IdealPoint = self_2413;
    let _e5: Point = other_2119;
    let _e6: Motor = ideal_point_point_geometric_product(_e4, _e5);
    let _e7: IdealPoint = self_2413;
    let _e8: IdealPoint = ideal_point_reversal(_e7);
    let _e9: Motor = motor_ideal_point_geometric_product(_e6, _e8);
    let _e10: Point = motor_point_into(_e9);
    return _e10;
}

fn ideal_point_rotor_geometric_quotient(self_2414: IdealPoint, other_2120: Rotor) -> IdealPoint {
    var self_2415: IdealPoint;
    var other_2121: Rotor;

    self_2415 = self_2414;
    other_2121 = other_2120;
    let _e4: IdealPoint = self_2415;
    let _e5: Rotor = other_2121;
    let _e6: Rotor = rotor_inverse(_e5);
    let _e7: IdealPoint = ideal_point_rotor_geometric_product(_e4, _e6);
    return _e7;
}

fn ideal_point_rotor_transformation(self_2416: IdealPoint, other_2122: Rotor) -> Rotor {
    var self_2417: IdealPoint;
    var other_2123: Rotor;

    self_2417 = self_2416;
    other_2123 = other_2122;
    let _e4: IdealPoint = self_2417;
    let _e5: Rotor = other_2123;
    let _e6: IdealPoint = ideal_point_rotor_geometric_product(_e4, _e5);
    let _e7: IdealPoint = self_2417;
    let _e8: IdealPoint = ideal_point_reversal(_e7);
    let _e9: Rotor = ideal_point_ideal_point_geometric_product(_e6, _e8);
    return _e9;
}

fn ideal_point_scalar_geometric_quotient(self_2418: IdealPoint, other_2124: Scalar) -> IdealPoint {
    var self_2419: IdealPoint;
    var other_2125: Scalar;

    self_2419 = self_2418;
    other_2125 = other_2124;
    let _e4: IdealPoint = self_2419;
    let _e5: Scalar = other_2125;
    let _e6: Scalar = scalar_inverse(_e5);
    let _e7: IdealPoint = ideal_point_scalar_geometric_product(_e4, _e6);
    return _e7;
}

fn ideal_point_scalar_transformation(self_2420: IdealPoint, other_2126: Scalar) -> Scalar {
    var self_2421: IdealPoint;
    var other_2127: Scalar;

    self_2421 = self_2420;
    other_2127 = other_2126;
    let _e4: IdealPoint = self_2421;
    let _e5: Scalar = other_2127;
    let _e6: IdealPoint = ideal_point_scalar_geometric_product(_e4, _e5);
    let _e7: IdealPoint = self_2421;
    let _e8: IdealPoint = ideal_point_reversal(_e7);
    let _e9: Rotor = ideal_point_ideal_point_geometric_product(_e6, _e8);
    let _e10: Scalar = rotor_scalar_into(_e9);
    return _e10;
}

fn ideal_point_translator_geometric_quotient(self_2422: IdealPoint, other_2128: Translator) -> Motor {
    var self_2423: IdealPoint;
    var other_2129: Translator;

    self_2423 = self_2422;
    other_2129 = other_2128;
    let _e4: IdealPoint = self_2423;
    let _e5: Translator = other_2129;
    let _e6: Translator = translator_inverse(_e5);
    let _e7: Motor = ideal_point_translator_geometric_product(_e4, _e6);
    return _e7;
}

fn ideal_point_translator_transformation(self_2424: IdealPoint, other_2130: Translator) -> Translator {
    var self_2425: IdealPoint;
    var other_2131: Translator;

    self_2425 = self_2424;
    other_2131 = other_2130;
    let _e4: IdealPoint = self_2425;
    let _e5: Translator = other_2131;
    let _e6: Motor = ideal_point_translator_geometric_product(_e4, _e5);
    let _e7: IdealPoint = self_2425;
    let _e8: IdealPoint = ideal_point_reversal(_e7);
    let _e9: Motor = motor_ideal_point_geometric_product(_e6, _e8);
    let _e10: Translator = motor_translator_into(_e9);
    return _e10;
}

fn motor_anti_scalar_geometric_quotient(self_2426: Motor, other_2132: AntiScalar) -> MotorDual {
    var self_2427: Motor;
    var other_2133: AntiScalar;

    self_2427 = self_2426;
    other_2133 = other_2132;
    let _e4: Motor = self_2427;
    let _e5: AntiScalar = other_2133;
    let _e6: AntiScalar = anti_scalar_inverse(_e5);
    let _e7: MotorDual = motor_anti_scalar_geometric_product(_e4, _e6);
    return _e7;
}

fn motor_anti_scalar_transformation(self_2428: Motor, other_2134: AntiScalar) -> AntiScalar {
    var self_2429: Motor;
    var other_2135: AntiScalar;

    self_2429 = self_2428;
    other_2135 = other_2134;
    let _e4: Motor = self_2429;
    let _e5: AntiScalar = other_2135;
    let _e6: MotorDual = motor_anti_scalar_geometric_product(_e4, _e5);
    let _e7: Motor = self_2429;
    let _e8: Motor = motor_reversal(_e7);
    let _e9: MotorDual = motor_dual_motor_geometric_product(_e6, _e8);
    let _e10: AntiScalar = motor_dual_anti_scalar_into(_e9);
    return _e10;
}

fn motor_ideal_point_geometric_quotient(self_2430: Motor, other_2136: IdealPoint) -> Motor {
    var self_2431: Motor;
    var other_2137: IdealPoint;

    self_2431 = self_2430;
    other_2137 = other_2136;
    let _e4: Motor = self_2431;
    let _e5: IdealPoint = other_2137;
    let _e6: IdealPoint = ideal_point_inverse(_e5);
    let _e7: Motor = motor_ideal_point_geometric_product(_e4, _e6);
    return _e7;
}

fn motor_ideal_point_transformation(self_2432: Motor, other_2138: IdealPoint) -> IdealPoint {
    var self_2433: Motor;
    var other_2139: IdealPoint;

    self_2433 = self_2432;
    other_2139 = other_2138;
    let _e4: Motor = self_2433;
    let _e5: IdealPoint = other_2139;
    let _e6: Motor = motor_ideal_point_geometric_product(_e4, _e5);
    let _e7: Motor = self_2433;
    let _e8: Motor = motor_reversal(_e7);
    let _e9: Motor = motor_motor_geometric_product(_e6, _e8);
    let _e10: IdealPoint = motor_ideal_point_into(_e9);
    return _e10;
}

fn motor_powi(self_2434: Motor, exponent: i32) -> Motor {
    var self_2435: Motor;
    var exponent_1: i32;
    var local: Motor;
    var x: Motor;
    var y: Motor;
    var n: i32;

    self_2435 = self_2434;
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
        let _e11: Motor = self_2435;
        let _e12: Motor = motor_inverse(_e11);
        local = _e12;
    } else {
        let _e14: Motor = self_2435;
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

fn motor_motor_geometric_quotient(self_2436: Motor, other_2140: Motor) -> Motor {
    var self_2437: Motor;
    var other_2141: Motor;

    self_2437 = self_2436;
    other_2141 = other_2140;
    let _e4: Motor = self_2437;
    let _e5: Motor = other_2141;
    let _e6: Motor = motor_inverse(_e5);
    let _e7: Motor = motor_motor_geometric_product(_e4, _e6);
    return _e7;
}

fn motor_motor_transformation(self_2438: Motor, other_2142: Motor) -> Motor {
    var self_2439: Motor;
    var other_2143: Motor;

    self_2439 = self_2438;
    other_2143 = other_2142;
    let _e4: Motor = self_2439;
    let _e5: Motor = other_2143;
    let _e6: Motor = motor_motor_geometric_product(_e4, _e5);
    let _e7: Motor = self_2439;
    let _e8: Motor = motor_reversal(_e7);
    let _e9: Motor = motor_motor_geometric_product(_e6, _e8);
    return _e9;
}

fn motor_motor_dual_geometric_quotient(self_2440: Motor, other_2144: MotorDual) -> MotorDual {
    var self_2441: Motor;
    var other_2145: MotorDual;

    self_2441 = self_2440;
    other_2145 = other_2144;
    let _e4: Motor = self_2441;
    let _e5: MotorDual = other_2145;
    let _e6: MotorDual = motor_dual_inverse(_e5);
    let _e7: MotorDual = motor_motor_dual_geometric_product(_e4, _e6);
    return _e7;
}

fn motor_motor_dual_transformation(self_2442: Motor, other_2146: MotorDual) -> MotorDual {
    var self_2443: Motor;
    var other_2147: MotorDual;

    self_2443 = self_2442;
    other_2147 = other_2146;
    let _e4: Motor = self_2443;
    let _e5: MotorDual = other_2147;
    let _e6: MotorDual = motor_motor_dual_geometric_product(_e4, _e5);
    let _e7: Motor = self_2443;
    let _e8: Motor = motor_reversal(_e7);
    let _e9: MotorDual = motor_dual_motor_geometric_product(_e6, _e8);
    return _e9;
}

fn motor_multi_vector_geometric_quotient(self_2444: Motor, other_2148: MultiVector) -> MultiVector {
    var self_2445: Motor;
    var other_2149: MultiVector;

    self_2445 = self_2444;
    other_2149 = other_2148;
    let _e4: Motor = self_2445;
    let _e5: MultiVector = other_2149;
    let _e6: MultiVector = multi_vector_inverse(_e5);
    let _e7: MultiVector = motor_multi_vector_geometric_product(_e4, _e6);
    return _e7;
}

fn motor_multi_vector_transformation(self_2446: Motor, other_2150: MultiVector) -> MultiVector {
    var self_2447: Motor;
    var other_2151: MultiVector;

    self_2447 = self_2446;
    other_2151 = other_2150;
    let _e4: Motor = self_2447;
    let _e5: MultiVector = other_2151;
    let _e6: MultiVector = motor_multi_vector_geometric_product(_e4, _e5);
    let _e7: Motor = self_2447;
    let _e8: Motor = motor_reversal(_e7);
    let _e9: MultiVector = multi_vector_motor_geometric_product(_e6, _e8);
    return _e9;
}

fn motor_plane_geometric_quotient(self_2448: Motor, other_2152: Plane) -> MotorDual {
    var self_2449: Motor;
    var other_2153: Plane;

    self_2449 = self_2448;
    other_2153 = other_2152;
    let _e4: Motor = self_2449;
    let _e5: Plane = other_2153;
    let _e6: Plane = plane_inverse(_e5);
    let _e7: MotorDual = motor_plane_geometric_product(_e4, _e6);
    return _e7;
}

fn motor_plane_transformation(self_2450: Motor, other_2154: Plane) -> Plane {
    var self_2451: Motor;
    var other_2155: Plane;

    self_2451 = self_2450;
    other_2155 = other_2154;
    let _e4: Motor = self_2451;
    let _e5: Plane = other_2155;
    let _e6: MotorDual = motor_plane_geometric_product(_e4, _e5);
    let _e7: Motor = self_2451;
    let _e8: Motor = motor_reversal(_e7);
    let _e9: MotorDual = motor_dual_motor_geometric_product(_e6, _e8);
    let _e10: Plane = motor_dual_plane_into(_e9);
    return _e10;
}

fn motor_point_geometric_quotient(self_2452: Motor, other_2156: Point) -> Motor {
    var self_2453: Motor;
    var other_2157: Point;

    self_2453 = self_2452;
    other_2157 = other_2156;
    let _e4: Motor = self_2453;
    let _e5: Point = other_2157;
    let _e6: Point = point_inverse(_e5);
    let _e7: Motor = motor_point_geometric_product(_e4, _e6);
    return _e7;
}

fn motor_point_transformation(self_2454: Motor, other_2158: Point) -> Point {
    var self_2455: Motor;
    var other_2159: Point;

    self_2455 = self_2454;
    other_2159 = other_2158;
    let _e4: Motor = self_2455;
    let _e5: Point = other_2159;
    let _e6: Motor = motor_point_geometric_product(_e4, _e5);
    let _e7: Motor = self_2455;
    let _e8: Motor = motor_reversal(_e7);
    let _e9: Motor = motor_motor_geometric_product(_e6, _e8);
    let _e10: Point = motor_point_into(_e9);
    return _e10;
}

fn motor_rotor_geometric_quotient(self_2456: Motor, other_2160: Rotor) -> Motor {
    var self_2457: Motor;
    var other_2161: Rotor;

    self_2457 = self_2456;
    other_2161 = other_2160;
    let _e4: Motor = self_2457;
    let _e5: Rotor = other_2161;
    let _e6: Rotor = rotor_inverse(_e5);
    let _e7: Motor = motor_rotor_geometric_product(_e4, _e6);
    return _e7;
}

fn motor_rotor_transformation(self_2458: Motor, other_2162: Rotor) -> Rotor {
    var self_2459: Motor;
    var other_2163: Rotor;

    self_2459 = self_2458;
    other_2163 = other_2162;
    let _e4: Motor = self_2459;
    let _e5: Rotor = other_2163;
    let _e6: Motor = motor_rotor_geometric_product(_e4, _e5);
    let _e7: Motor = self_2459;
    let _e8: Motor = motor_reversal(_e7);
    let _e9: Motor = motor_motor_geometric_product(_e6, _e8);
    let _e10: Rotor = motor_rotor_into(_e9);
    return _e10;
}

fn motor_scalar_geometric_quotient(self_2460: Motor, other_2164: Scalar) -> Motor {
    var self_2461: Motor;
    var other_2165: Scalar;

    self_2461 = self_2460;
    other_2165 = other_2164;
    let _e4: Motor = self_2461;
    let _e5: Scalar = other_2165;
    let _e6: Scalar = scalar_inverse(_e5);
    let _e7: Motor = motor_scalar_geometric_product(_e4, _e6);
    return _e7;
}

fn motor_scalar_transformation(self_2462: Motor, other_2166: Scalar) -> Scalar {
    var self_2463: Motor;
    var other_2167: Scalar;

    self_2463 = self_2462;
    other_2167 = other_2166;
    let _e4: Motor = self_2463;
    let _e5: Scalar = other_2167;
    let _e6: Motor = motor_scalar_geometric_product(_e4, _e5);
    let _e7: Motor = self_2463;
    let _e8: Motor = motor_reversal(_e7);
    let _e9: Motor = motor_motor_geometric_product(_e6, _e8);
    let _e10: Scalar = motor_scalar_into(_e9);
    return _e10;
}

fn motor_translator_geometric_quotient(self_2464: Motor, other_2168: Translator) -> Motor {
    var self_2465: Motor;
    var other_2169: Translator;

    self_2465 = self_2464;
    other_2169 = other_2168;
    let _e4: Motor = self_2465;
    let _e5: Translator = other_2169;
    let _e6: Translator = translator_inverse(_e5);
    let _e7: Motor = motor_translator_geometric_product(_e4, _e6);
    return _e7;
}

fn motor_translator_transformation(self_2466: Motor, other_2170: Translator) -> Translator {
    var self_2467: Motor;
    var other_2171: Translator;

    self_2467 = self_2466;
    other_2171 = other_2170;
    let _e4: Motor = self_2467;
    let _e5: Translator = other_2171;
    let _e6: Motor = motor_translator_geometric_product(_e4, _e5);
    let _e7: Motor = self_2467;
    let _e8: Motor = motor_reversal(_e7);
    let _e9: Motor = motor_motor_geometric_product(_e6, _e8);
    let _e10: Translator = motor_translator_into(_e9);
    return _e10;
}

fn motor_dual_anti_scalar_geometric_quotient(self_2468: MotorDual, other_2172: AntiScalar) -> Motor {
    var self_2469: MotorDual;
    var other_2173: AntiScalar;

    self_2469 = self_2468;
    other_2173 = other_2172;
    let _e4: MotorDual = self_2469;
    let _e5: AntiScalar = other_2173;
    let _e6: AntiScalar = anti_scalar_inverse(_e5);
    let _e7: Motor = motor_dual_anti_scalar_geometric_product(_e4, _e6);
    return _e7;
}

fn motor_dual_anti_scalar_transformation(self_2470: MotorDual, other_2174: AntiScalar) -> AntiScalar {
    var self_2471: MotorDual;
    var other_2175: AntiScalar;

    self_2471 = self_2470;
    other_2175 = other_2174;
    let _e4: MotorDual = self_2471;
    let _e5: AntiScalar = other_2175;
    let _e6: Motor = motor_dual_anti_scalar_geometric_product(_e4, _e5);
    let _e7: MotorDual = self_2471;
    let _e8: MotorDual = motor_dual_reversal(_e7);
    let _e9: MotorDual = motor_motor_dual_geometric_product(_e6, _e8);
    let _e10: AntiScalar = motor_dual_anti_scalar_into(_e9);
    return _e10;
}

fn motor_dual_ideal_point_geometric_quotient(self_2472: MotorDual, other_2176: IdealPoint) -> MotorDual {
    var self_2473: MotorDual;
    var other_2177: IdealPoint;

    self_2473 = self_2472;
    other_2177 = other_2176;
    let _e4: MotorDual = self_2473;
    let _e5: IdealPoint = other_2177;
    let _e6: IdealPoint = ideal_point_inverse(_e5);
    let _e7: MotorDual = motor_dual_ideal_point_geometric_product(_e4, _e6);
    return _e7;
}

fn motor_dual_ideal_point_transformation(self_2474: MotorDual, other_2178: IdealPoint) -> IdealPoint {
    var self_2475: MotorDual;
    var other_2179: IdealPoint;

    self_2475 = self_2474;
    other_2179 = other_2178;
    let _e4: MotorDual = self_2475;
    let _e5: IdealPoint = other_2179;
    let _e6: MotorDual = motor_dual_ideal_point_geometric_product(_e4, _e5);
    let _e7: MotorDual = self_2475;
    let _e8: MotorDual = motor_dual_reversal(_e7);
    let _e9: Motor = motor_dual_motor_dual_geometric_product(_e6, _e8);
    let _e10: IdealPoint = motor_ideal_point_into(_e9);
    return _e10;
}

fn motor_dual_motor_geometric_quotient(self_2476: MotorDual, other_2180: Motor) -> MotorDual {
    var self_2477: MotorDual;
    var other_2181: Motor;

    self_2477 = self_2476;
    other_2181 = other_2180;
    let _e4: MotorDual = self_2477;
    let _e5: Motor = other_2181;
    let _e6: Motor = motor_inverse(_e5);
    let _e7: MotorDual = motor_dual_motor_geometric_product(_e4, _e6);
    return _e7;
}

fn motor_dual_motor_transformation(self_2478: MotorDual, other_2182: Motor) -> Motor {
    var self_2479: MotorDual;
    var other_2183: Motor;

    self_2479 = self_2478;
    other_2183 = other_2182;
    let _e4: MotorDual = self_2479;
    let _e5: Motor = other_2183;
    let _e6: MotorDual = motor_dual_motor_geometric_product(_e4, _e5);
    let _e7: MotorDual = self_2479;
    let _e8: MotorDual = motor_dual_reversal(_e7);
    let _e9: Motor = motor_dual_motor_dual_geometric_product(_e6, _e8);
    return _e9;
}

fn motor_dual_motor_dual_geometric_quotient(self_2480: MotorDual, other_2184: MotorDual) -> Motor {
    var self_2481: MotorDual;
    var other_2185: MotorDual;

    self_2481 = self_2480;
    other_2185 = other_2184;
    let _e4: MotorDual = self_2481;
    let _e5: MotorDual = other_2185;
    let _e6: MotorDual = motor_dual_inverse(_e5);
    let _e7: Motor = motor_dual_motor_dual_geometric_product(_e4, _e6);
    return _e7;
}

fn motor_dual_motor_dual_transformation(self_2482: MotorDual, other_2186: MotorDual) -> MotorDual {
    var self_2483: MotorDual;
    var other_2187: MotorDual;

    self_2483 = self_2482;
    other_2187 = other_2186;
    let _e4: MotorDual = self_2483;
    let _e5: MotorDual = other_2187;
    let _e6: Motor = motor_dual_motor_dual_geometric_product(_e4, _e5);
    let _e7: MotorDual = self_2483;
    let _e8: MotorDual = motor_dual_reversal(_e7);
    let _e9: MotorDual = motor_motor_dual_geometric_product(_e6, _e8);
    return _e9;
}

fn motor_dual_multi_vector_geometric_quotient(self_2484: MotorDual, other_2188: MultiVector) -> MultiVector {
    var self_2485: MotorDual;
    var other_2189: MultiVector;

    self_2485 = self_2484;
    other_2189 = other_2188;
    let _e4: MotorDual = self_2485;
    let _e5: MultiVector = other_2189;
    let _e6: MultiVector = multi_vector_inverse(_e5);
    let _e7: MultiVector = motor_dual_multi_vector_geometric_product(_e4, _e6);
    return _e7;
}

fn motor_dual_multi_vector_transformation(self_2486: MotorDual, other_2190: MultiVector) -> MultiVector {
    var self_2487: MotorDual;
    var other_2191: MultiVector;

    self_2487 = self_2486;
    other_2191 = other_2190;
    let _e4: MotorDual = self_2487;
    let _e5: MultiVector = other_2191;
    let _e6: MultiVector = motor_dual_multi_vector_geometric_product(_e4, _e5);
    let _e7: MotorDual = self_2487;
    let _e8: MotorDual = motor_dual_reversal(_e7);
    let _e9: MultiVector = multi_vector_motor_dual_geometric_product(_e6, _e8);
    return _e9;
}

fn motor_dual_plane_geometric_quotient(self_2488: MotorDual, other_2192: Plane) -> Motor {
    var self_2489: MotorDual;
    var other_2193: Plane;

    self_2489 = self_2488;
    other_2193 = other_2192;
    let _e4: MotorDual = self_2489;
    let _e5: Plane = other_2193;
    let _e6: Plane = plane_inverse(_e5);
    let _e7: Motor = motor_dual_plane_geometric_product(_e4, _e6);
    return _e7;
}

fn motor_dual_plane_transformation(self_2490: MotorDual, other_2194: Plane) -> Plane {
    var self_2491: MotorDual;
    var other_2195: Plane;

    self_2491 = self_2490;
    other_2195 = other_2194;
    let _e4: MotorDual = self_2491;
    let _e5: Plane = other_2195;
    let _e6: Motor = motor_dual_plane_geometric_product(_e4, _e5);
    let _e7: MotorDual = self_2491;
    let _e8: MotorDual = motor_dual_reversal(_e7);
    let _e9: MotorDual = motor_motor_dual_geometric_product(_e6, _e8);
    let _e10: Plane = motor_dual_plane_into(_e9);
    return _e10;
}

fn motor_dual_point_geometric_quotient(self_2492: MotorDual, other_2196: Point) -> MotorDual {
    var self_2493: MotorDual;
    var other_2197: Point;

    self_2493 = self_2492;
    other_2197 = other_2196;
    let _e4: MotorDual = self_2493;
    let _e5: Point = other_2197;
    let _e6: Point = point_inverse(_e5);
    let _e7: MotorDual = motor_dual_point_geometric_product(_e4, _e6);
    return _e7;
}

fn motor_dual_point_transformation(self_2494: MotorDual, other_2198: Point) -> Point {
    var self_2495: MotorDual;
    var other_2199: Point;

    self_2495 = self_2494;
    other_2199 = other_2198;
    let _e4: MotorDual = self_2495;
    let _e5: Point = other_2199;
    let _e6: MotorDual = motor_dual_point_geometric_product(_e4, _e5);
    let _e7: MotorDual = self_2495;
    let _e8: MotorDual = motor_dual_reversal(_e7);
    let _e9: Motor = motor_dual_motor_dual_geometric_product(_e6, _e8);
    let _e10: Point = motor_point_into(_e9);
    return _e10;
}

fn motor_dual_rotor_geometric_quotient(self_2496: MotorDual, other_2200: Rotor) -> MotorDual {
    var self_2497: MotorDual;
    var other_2201: Rotor;

    self_2497 = self_2496;
    other_2201 = other_2200;
    let _e4: MotorDual = self_2497;
    let _e5: Rotor = other_2201;
    let _e6: Rotor = rotor_inverse(_e5);
    let _e7: MotorDual = motor_dual_rotor_geometric_product(_e4, _e6);
    return _e7;
}

fn motor_dual_rotor_transformation(self_2498: MotorDual, other_2202: Rotor) -> Rotor {
    var self_2499: MotorDual;
    var other_2203: Rotor;

    self_2499 = self_2498;
    other_2203 = other_2202;
    let _e4: MotorDual = self_2499;
    let _e5: Rotor = other_2203;
    let _e6: MotorDual = motor_dual_rotor_geometric_product(_e4, _e5);
    let _e7: MotorDual = self_2499;
    let _e8: MotorDual = motor_dual_reversal(_e7);
    let _e9: Motor = motor_dual_motor_dual_geometric_product(_e6, _e8);
    let _e10: Rotor = motor_rotor_into(_e9);
    return _e10;
}

fn motor_dual_scalar_geometric_quotient(self_2500: MotorDual, other_2204: Scalar) -> MotorDual {
    var self_2501: MotorDual;
    var other_2205: Scalar;

    self_2501 = self_2500;
    other_2205 = other_2204;
    let _e4: MotorDual = self_2501;
    let _e5: Scalar = other_2205;
    let _e6: Scalar = scalar_inverse(_e5);
    let _e7: MotorDual = motor_dual_scalar_geometric_product(_e4, _e6);
    return _e7;
}

fn motor_dual_scalar_transformation(self_2502: MotorDual, other_2206: Scalar) -> Scalar {
    var self_2503: MotorDual;
    var other_2207: Scalar;

    self_2503 = self_2502;
    other_2207 = other_2206;
    let _e4: MotorDual = self_2503;
    let _e5: Scalar = other_2207;
    let _e6: MotorDual = motor_dual_scalar_geometric_product(_e4, _e5);
    let _e7: MotorDual = self_2503;
    let _e8: MotorDual = motor_dual_reversal(_e7);
    let _e9: Motor = motor_dual_motor_dual_geometric_product(_e6, _e8);
    let _e10: Scalar = motor_scalar_into(_e9);
    return _e10;
}

fn motor_dual_translator_geometric_quotient(self_2504: MotorDual, other_2208: Translator) -> MotorDual {
    var self_2505: MotorDual;
    var other_2209: Translator;

    self_2505 = self_2504;
    other_2209 = other_2208;
    let _e4: MotorDual = self_2505;
    let _e5: Translator = other_2209;
    let _e6: Translator = translator_inverse(_e5);
    let _e7: MotorDual = motor_dual_translator_geometric_product(_e4, _e6);
    return _e7;
}

fn motor_dual_translator_transformation(self_2506: MotorDual, other_2210: Translator) -> Translator {
    var self_2507: MotorDual;
    var other_2211: Translator;

    self_2507 = self_2506;
    other_2211 = other_2210;
    let _e4: MotorDual = self_2507;
    let _e5: Translator = other_2211;
    let _e6: MotorDual = motor_dual_translator_geometric_product(_e4, _e5);
    let _e7: MotorDual = self_2507;
    let _e8: MotorDual = motor_dual_reversal(_e7);
    let _e9: Motor = motor_dual_motor_dual_geometric_product(_e6, _e8);
    let _e10: Translator = motor_translator_into(_e9);
    return _e10;
}

fn multi_vector_anti_scalar_geometric_quotient(self_2508: MultiVector, other_2212: AntiScalar) -> MultiVector {
    var self_2509: MultiVector;
    var other_2213: AntiScalar;

    self_2509 = self_2508;
    other_2213 = other_2212;
    let _e4: MultiVector = self_2509;
    let _e5: AntiScalar = other_2213;
    let _e6: AntiScalar = anti_scalar_inverse(_e5);
    let _e7: MultiVector = multi_vector_anti_scalar_geometric_product(_e4, _e6);
    return _e7;
}

fn multi_vector_anti_scalar_transformation(self_2510: MultiVector, other_2214: AntiScalar) -> AntiScalar {
    var self_2511: MultiVector;
    var other_2215: AntiScalar;

    self_2511 = self_2510;
    other_2215 = other_2214;
    let _e4: MultiVector = self_2511;
    let _e5: AntiScalar = other_2215;
    let _e6: MultiVector = multi_vector_anti_scalar_geometric_product(_e4, _e5);
    let _e7: MultiVector = self_2511;
    let _e8: MultiVector = multi_vector_reversal(_e7);
    let _e9: MultiVector = multi_vector_multi_vector_geometric_product(_e6, _e8);
    let _e10: AntiScalar = multi_vector_anti_scalar_into(_e9);
    return _e10;
}

fn multi_vector_ideal_point_geometric_quotient(self_2512: MultiVector, other_2216: IdealPoint) -> MultiVector {
    var self_2513: MultiVector;
    var other_2217: IdealPoint;

    self_2513 = self_2512;
    other_2217 = other_2216;
    let _e4: MultiVector = self_2513;
    let _e5: IdealPoint = other_2217;
    let _e6: IdealPoint = ideal_point_inverse(_e5);
    let _e7: MultiVector = multi_vector_ideal_point_geometric_product(_e4, _e6);
    return _e7;
}

fn multi_vector_ideal_point_transformation(self_2514: MultiVector, other_2218: IdealPoint) -> IdealPoint {
    var self_2515: MultiVector;
    var other_2219: IdealPoint;

    self_2515 = self_2514;
    other_2219 = other_2218;
    let _e4: MultiVector = self_2515;
    let _e5: IdealPoint = other_2219;
    let _e6: MultiVector = multi_vector_ideal_point_geometric_product(_e4, _e5);
    let _e7: MultiVector = self_2515;
    let _e8: MultiVector = multi_vector_reversal(_e7);
    let _e9: MultiVector = multi_vector_multi_vector_geometric_product(_e6, _e8);
    let _e10: IdealPoint = multi_vector_ideal_point_into(_e9);
    return _e10;
}

fn multi_vector_motor_geometric_quotient(self_2516: MultiVector, other_2220: Motor) -> MultiVector {
    var self_2517: MultiVector;
    var other_2221: Motor;

    self_2517 = self_2516;
    other_2221 = other_2220;
    let _e4: MultiVector = self_2517;
    let _e5: Motor = other_2221;
    let _e6: Motor = motor_inverse(_e5);
    let _e7: MultiVector = multi_vector_motor_geometric_product(_e4, _e6);
    return _e7;
}

fn multi_vector_motor_transformation(self_2518: MultiVector, other_2222: Motor) -> Motor {
    var self_2519: MultiVector;
    var other_2223: Motor;

    self_2519 = self_2518;
    other_2223 = other_2222;
    let _e4: MultiVector = self_2519;
    let _e5: Motor = other_2223;
    let _e6: MultiVector = multi_vector_motor_geometric_product(_e4, _e5);
    let _e7: MultiVector = self_2519;
    let _e8: MultiVector = multi_vector_reversal(_e7);
    let _e9: MultiVector = multi_vector_multi_vector_geometric_product(_e6, _e8);
    let _e10: Motor = multi_vector_motor_into(_e9);
    return _e10;
}

fn multi_vector_motor_dual_geometric_quotient(self_2520: MultiVector, other_2224: MotorDual) -> MultiVector {
    var self_2521: MultiVector;
    var other_2225: MotorDual;

    self_2521 = self_2520;
    other_2225 = other_2224;
    let _e4: MultiVector = self_2521;
    let _e5: MotorDual = other_2225;
    let _e6: MotorDual = motor_dual_inverse(_e5);
    let _e7: MultiVector = multi_vector_motor_dual_geometric_product(_e4, _e6);
    return _e7;
}

fn multi_vector_motor_dual_transformation(self_2522: MultiVector, other_2226: MotorDual) -> MotorDual {
    var self_2523: MultiVector;
    var other_2227: MotorDual;

    self_2523 = self_2522;
    other_2227 = other_2226;
    let _e4: MultiVector = self_2523;
    let _e5: MotorDual = other_2227;
    let _e6: MultiVector = multi_vector_motor_dual_geometric_product(_e4, _e5);
    let _e7: MultiVector = self_2523;
    let _e8: MultiVector = multi_vector_reversal(_e7);
    let _e9: MultiVector = multi_vector_multi_vector_geometric_product(_e6, _e8);
    let _e10: MotorDual = multi_vector_motor_dual_into(_e9);
    return _e10;
}

fn multi_vector_powi(self_2524: MultiVector, exponent_2: i32) -> MultiVector {
    var self_2525: MultiVector;
    var exponent_3: i32;
    var local_1: MultiVector;
    var x_1: MultiVector;
    var y_1: MultiVector;
    var n_1: i32;

    self_2525 = self_2524;
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
        let _e11: MultiVector = self_2525;
        let _e12: MultiVector = multi_vector_inverse(_e11);
        local_1 = _e12;
    } else {
        let _e14: MultiVector = self_2525;
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

fn multi_vector_multi_vector_geometric_quotient(self_2526: MultiVector, other_2228: MultiVector) -> MultiVector {
    var self_2527: MultiVector;
    var other_2229: MultiVector;

    self_2527 = self_2526;
    other_2229 = other_2228;
    let _e4: MultiVector = self_2527;
    let _e5: MultiVector = other_2229;
    let _e6: MultiVector = multi_vector_inverse(_e5);
    let _e7: MultiVector = multi_vector_multi_vector_geometric_product(_e4, _e6);
    return _e7;
}

fn multi_vector_multi_vector_transformation(self_2528: MultiVector, other_2230: MultiVector) -> MultiVector {
    var self_2529: MultiVector;
    var other_2231: MultiVector;

    self_2529 = self_2528;
    other_2231 = other_2230;
    let _e4: MultiVector = self_2529;
    let _e5: MultiVector = other_2231;
    let _e6: MultiVector = multi_vector_multi_vector_geometric_product(_e4, _e5);
    let _e7: MultiVector = self_2529;
    let _e8: MultiVector = multi_vector_reversal(_e7);
    let _e9: MultiVector = multi_vector_multi_vector_geometric_product(_e6, _e8);
    return _e9;
}

fn multi_vector_plane_geometric_quotient(self_2530: MultiVector, other_2232: Plane) -> MultiVector {
    var self_2531: MultiVector;
    var other_2233: Plane;

    self_2531 = self_2530;
    other_2233 = other_2232;
    let _e4: MultiVector = self_2531;
    let _e5: Plane = other_2233;
    let _e6: Plane = plane_inverse(_e5);
    let _e7: MultiVector = multi_vector_plane_geometric_product(_e4, _e6);
    return _e7;
}

fn multi_vector_plane_transformation(self_2532: MultiVector, other_2234: Plane) -> Plane {
    var self_2533: MultiVector;
    var other_2235: Plane;

    self_2533 = self_2532;
    other_2235 = other_2234;
    let _e4: MultiVector = self_2533;
    let _e5: Plane = other_2235;
    let _e6: MultiVector = multi_vector_plane_geometric_product(_e4, _e5);
    let _e7: MultiVector = self_2533;
    let _e8: MultiVector = multi_vector_reversal(_e7);
    let _e9: MultiVector = multi_vector_multi_vector_geometric_product(_e6, _e8);
    let _e10: Plane = multi_vector_plane_into(_e9);
    return _e10;
}

fn multi_vector_point_geometric_quotient(self_2534: MultiVector, other_2236: Point) -> MultiVector {
    var self_2535: MultiVector;
    var other_2237: Point;

    self_2535 = self_2534;
    other_2237 = other_2236;
    let _e4: MultiVector = self_2535;
    let _e5: Point = other_2237;
    let _e6: Point = point_inverse(_e5);
    let _e7: MultiVector = multi_vector_point_geometric_product(_e4, _e6);
    return _e7;
}

fn multi_vector_point_transformation(self_2536: MultiVector, other_2238: Point) -> Point {
    var self_2537: MultiVector;
    var other_2239: Point;

    self_2537 = self_2536;
    other_2239 = other_2238;
    let _e4: MultiVector = self_2537;
    let _e5: Point = other_2239;
    let _e6: MultiVector = multi_vector_point_geometric_product(_e4, _e5);
    let _e7: MultiVector = self_2537;
    let _e8: MultiVector = multi_vector_reversal(_e7);
    let _e9: MultiVector = multi_vector_multi_vector_geometric_product(_e6, _e8);
    let _e10: Point = multi_vector_point_into(_e9);
    return _e10;
}

fn multi_vector_rotor_geometric_quotient(self_2538: MultiVector, other_2240: Rotor) -> MultiVector {
    var self_2539: MultiVector;
    var other_2241: Rotor;

    self_2539 = self_2538;
    other_2241 = other_2240;
    let _e4: MultiVector = self_2539;
    let _e5: Rotor = other_2241;
    let _e6: Rotor = rotor_inverse(_e5);
    let _e7: MultiVector = multi_vector_rotor_geometric_product(_e4, _e6);
    return _e7;
}

fn multi_vector_rotor_transformation(self_2540: MultiVector, other_2242: Rotor) -> Rotor {
    var self_2541: MultiVector;
    var other_2243: Rotor;

    self_2541 = self_2540;
    other_2243 = other_2242;
    let _e4: MultiVector = self_2541;
    let _e5: Rotor = other_2243;
    let _e6: MultiVector = multi_vector_rotor_geometric_product(_e4, _e5);
    let _e7: MultiVector = self_2541;
    let _e8: MultiVector = multi_vector_reversal(_e7);
    let _e9: MultiVector = multi_vector_multi_vector_geometric_product(_e6, _e8);
    let _e10: Rotor = multi_vector_rotor_into(_e9);
    return _e10;
}

fn multi_vector_scalar_geometric_quotient(self_2542: MultiVector, other_2244: Scalar) -> MultiVector {
    var self_2543: MultiVector;
    var other_2245: Scalar;

    self_2543 = self_2542;
    other_2245 = other_2244;
    let _e4: MultiVector = self_2543;
    let _e5: Scalar = other_2245;
    let _e6: Scalar = scalar_inverse(_e5);
    let _e7: MultiVector = multi_vector_scalar_geometric_product(_e4, _e6);
    return _e7;
}

fn multi_vector_scalar_transformation(self_2544: MultiVector, other_2246: Scalar) -> Scalar {
    var self_2545: MultiVector;
    var other_2247: Scalar;

    self_2545 = self_2544;
    other_2247 = other_2246;
    let _e4: MultiVector = self_2545;
    let _e5: Scalar = other_2247;
    let _e6: MultiVector = multi_vector_scalar_geometric_product(_e4, _e5);
    let _e7: MultiVector = self_2545;
    let _e8: MultiVector = multi_vector_reversal(_e7);
    let _e9: MultiVector = multi_vector_multi_vector_geometric_product(_e6, _e8);
    let _e10: Scalar = multi_vector_scalar_into(_e9);
    return _e10;
}

fn multi_vector_translator_geometric_quotient(self_2546: MultiVector, other_2248: Translator) -> MultiVector {
    var self_2547: MultiVector;
    var other_2249: Translator;

    self_2547 = self_2546;
    other_2249 = other_2248;
    let _e4: MultiVector = self_2547;
    let _e5: Translator = other_2249;
    let _e6: Translator = translator_inverse(_e5);
    let _e7: MultiVector = multi_vector_translator_geometric_product(_e4, _e6);
    return _e7;
}

fn multi_vector_translator_transformation(self_2548: MultiVector, other_2250: Translator) -> Translator {
    var self_2549: MultiVector;
    var other_2251: Translator;

    self_2549 = self_2548;
    other_2251 = other_2250;
    let _e4: MultiVector = self_2549;
    let _e5: Translator = other_2251;
    let _e6: MultiVector = multi_vector_translator_geometric_product(_e4, _e5);
    let _e7: MultiVector = self_2549;
    let _e8: MultiVector = multi_vector_reversal(_e7);
    let _e9: MultiVector = multi_vector_multi_vector_geometric_product(_e6, _e8);
    let _e10: Translator = multi_vector_translator_into(_e9);
    return _e10;
}

fn plane_anti_scalar_geometric_quotient(self_2550: Plane, other_2252: AntiScalar) -> Point {
    var self_2551: Plane;
    var other_2253: AntiScalar;

    self_2551 = self_2550;
    other_2253 = other_2252;
    let _e4: Plane = self_2551;
    let _e5: AntiScalar = other_2253;
    let _e6: AntiScalar = anti_scalar_inverse(_e5);
    let _e7: Point = plane_anti_scalar_geometric_product(_e4, _e6);
    return _e7;
}

fn plane_anti_scalar_transformation(self_2552: Plane, other_2254: AntiScalar) -> AntiScalar {
    var self_2553: Plane;
    var other_2255: AntiScalar;

    self_2553 = self_2552;
    other_2255 = other_2254;
    let _e4: Plane = self_2553;
    let _e5: AntiScalar = other_2255;
    let _e6: Point = plane_anti_scalar_geometric_product(_e4, _e5);
    let _e7: Plane = self_2553;
    let _e8: Plane = plane_reversal(_e7);
    let _e9: MotorDual = point_plane_geometric_product(_e6, _e8);
    let _e10: AntiScalar = motor_dual_anti_scalar_into(_e9);
    return _e10;
}

fn plane_ideal_point_geometric_quotient(self_2554: Plane, other_2256: IdealPoint) -> MotorDual {
    var self_2555: Plane;
    var other_2257: IdealPoint;

    self_2555 = self_2554;
    other_2257 = other_2256;
    let _e4: Plane = self_2555;
    let _e5: IdealPoint = other_2257;
    let _e6: IdealPoint = ideal_point_inverse(_e5);
    let _e7: MotorDual = plane_ideal_point_geometric_product(_e4, _e6);
    return _e7;
}

fn plane_ideal_point_transformation(self_2556: Plane, other_2258: IdealPoint) -> IdealPoint {
    var self_2557: Plane;
    var other_2259: IdealPoint;

    self_2557 = self_2556;
    other_2259 = other_2258;
    let _e4: Plane = self_2557;
    let _e5: IdealPoint = other_2259;
    let _e6: MotorDual = plane_ideal_point_geometric_product(_e4, _e5);
    let _e7: Plane = self_2557;
    let _e8: Plane = plane_reversal(_e7);
    let _e9: Motor = motor_dual_plane_geometric_product(_e6, _e8);
    let _e10: IdealPoint = motor_ideal_point_into(_e9);
    return _e10;
}

fn plane_motor_geometric_quotient(self_2558: Plane, other_2260: Motor) -> MotorDual {
    var self_2559: Plane;
    var other_2261: Motor;

    self_2559 = self_2558;
    other_2261 = other_2260;
    let _e4: Plane = self_2559;
    let _e5: Motor = other_2261;
    let _e6: Motor = motor_inverse(_e5);
    let _e7: MotorDual = plane_motor_geometric_product(_e4, _e6);
    return _e7;
}

fn plane_motor_transformation(self_2560: Plane, other_2262: Motor) -> Motor {
    var self_2561: Plane;
    var other_2263: Motor;

    self_2561 = self_2560;
    other_2263 = other_2262;
    let _e4: Plane = self_2561;
    let _e5: Motor = other_2263;
    let _e6: MotorDual = plane_motor_geometric_product(_e4, _e5);
    let _e7: Plane = self_2561;
    let _e8: Plane = plane_reversal(_e7);
    let _e9: Motor = motor_dual_plane_geometric_product(_e6, _e8);
    return _e9;
}

fn plane_motor_dual_geometric_quotient(self_2562: Plane, other_2264: MotorDual) -> Motor {
    var self_2563: Plane;
    var other_2265: MotorDual;

    self_2563 = self_2562;
    other_2265 = other_2264;
    let _e4: Plane = self_2563;
    let _e5: MotorDual = other_2265;
    let _e6: MotorDual = motor_dual_inverse(_e5);
    let _e7: Motor = plane_motor_dual_geometric_product(_e4, _e6);
    return _e7;
}

fn plane_motor_dual_transformation(self_2564: Plane, other_2266: MotorDual) -> MotorDual {
    var self_2565: Plane;
    var other_2267: MotorDual;

    self_2565 = self_2564;
    other_2267 = other_2266;
    let _e4: Plane = self_2565;
    let _e5: MotorDual = other_2267;
    let _e6: Motor = plane_motor_dual_geometric_product(_e4, _e5);
    let _e7: Plane = self_2565;
    let _e8: Plane = plane_reversal(_e7);
    let _e9: MotorDual = motor_plane_geometric_product(_e6, _e8);
    return _e9;
}

fn plane_multi_vector_geometric_quotient(self_2566: Plane, other_2268: MultiVector) -> MultiVector {
    var self_2567: Plane;
    var other_2269: MultiVector;

    self_2567 = self_2566;
    other_2269 = other_2268;
    let _e4: Plane = self_2567;
    let _e5: MultiVector = other_2269;
    let _e6: MultiVector = multi_vector_inverse(_e5);
    let _e7: MultiVector = plane_multi_vector_geometric_product(_e4, _e6);
    return _e7;
}

fn plane_multi_vector_transformation(self_2568: Plane, other_2270: MultiVector) -> MultiVector {
    var self_2569: Plane;
    var other_2271: MultiVector;

    self_2569 = self_2568;
    other_2271 = other_2270;
    let _e4: Plane = self_2569;
    let _e5: MultiVector = other_2271;
    let _e6: MultiVector = plane_multi_vector_geometric_product(_e4, _e5);
    let _e7: Plane = self_2569;
    let _e8: Plane = plane_reversal(_e7);
    let _e9: MultiVector = multi_vector_plane_geometric_product(_e6, _e8);
    return _e9;
}

fn plane_plane_geometric_quotient(self_2570: Plane, other_2272: Plane) -> Motor {
    var self_2571: Plane;
    var other_2273: Plane;

    self_2571 = self_2570;
    other_2273 = other_2272;
    let _e4: Plane = self_2571;
    let _e5: Plane = other_2273;
    let _e6: Plane = plane_inverse(_e5);
    let _e7: Motor = plane_plane_geometric_product(_e4, _e6);
    return _e7;
}

fn plane_plane_transformation(self_2572: Plane, other_2274: Plane) -> Plane {
    var self_2573: Plane;
    var other_2275: Plane;

    self_2573 = self_2572;
    other_2275 = other_2274;
    let _e4: Plane = self_2573;
    let _e5: Plane = other_2275;
    let _e6: Motor = plane_plane_geometric_product(_e4, _e5);
    let _e7: Plane = self_2573;
    let _e8: Plane = plane_reversal(_e7);
    let _e9: MotorDual = motor_plane_geometric_product(_e6, _e8);
    let _e10: Plane = motor_dual_plane_into(_e9);
    return _e10;
}

fn plane_point_geometric_quotient(self_2574: Plane, other_2276: Point) -> MotorDual {
    var self_2575: Plane;
    var other_2277: Point;

    self_2575 = self_2574;
    other_2277 = other_2276;
    let _e4: Plane = self_2575;
    let _e5: Point = other_2277;
    let _e6: Point = point_inverse(_e5);
    let _e7: MotorDual = plane_point_geometric_product(_e4, _e6);
    return _e7;
}

fn plane_point_transformation(self_2576: Plane, other_2278: Point) -> Point {
    var self_2577: Plane;
    var other_2279: Point;

    self_2577 = self_2576;
    other_2279 = other_2278;
    let _e4: Plane = self_2577;
    let _e5: Point = other_2279;
    let _e6: MotorDual = plane_point_geometric_product(_e4, _e5);
    let _e7: Plane = self_2577;
    let _e8: Plane = plane_reversal(_e7);
    let _e9: Motor = motor_dual_plane_geometric_product(_e6, _e8);
    let _e10: Point = motor_point_into(_e9);
    return _e10;
}

fn plane_rotor_geometric_quotient(self_2578: Plane, other_2280: Rotor) -> MotorDual {
    var self_2579: Plane;
    var other_2281: Rotor;

    self_2579 = self_2578;
    other_2281 = other_2280;
    let _e4: Plane = self_2579;
    let _e5: Rotor = other_2281;
    let _e6: Rotor = rotor_inverse(_e5);
    let _e7: MotorDual = plane_rotor_geometric_product(_e4, _e6);
    return _e7;
}

fn plane_rotor_transformation(self_2580: Plane, other_2282: Rotor) -> Rotor {
    var self_2581: Plane;
    var other_2283: Rotor;

    self_2581 = self_2580;
    other_2283 = other_2282;
    let _e4: Plane = self_2581;
    let _e5: Rotor = other_2283;
    let _e6: MotorDual = plane_rotor_geometric_product(_e4, _e5);
    let _e7: Plane = self_2581;
    let _e8: Plane = plane_reversal(_e7);
    let _e9: Motor = motor_dual_plane_geometric_product(_e6, _e8);
    let _e10: Rotor = motor_rotor_into(_e9);
    return _e10;
}

fn plane_scalar_geometric_quotient(self_2582: Plane, other_2284: Scalar) -> Plane {
    var self_2583: Plane;
    var other_2285: Scalar;

    self_2583 = self_2582;
    other_2285 = other_2284;
    let _e4: Plane = self_2583;
    let _e5: Scalar = other_2285;
    let _e6: Scalar = scalar_inverse(_e5);
    let _e7: Plane = plane_scalar_geometric_product(_e4, _e6);
    return _e7;
}

fn plane_scalar_transformation(self_2584: Plane, other_2286: Scalar) -> Scalar {
    var self_2585: Plane;
    var other_2287: Scalar;

    self_2585 = self_2584;
    other_2287 = other_2286;
    let _e4: Plane = self_2585;
    let _e5: Scalar = other_2287;
    let _e6: Plane = plane_scalar_geometric_product(_e4, _e5);
    let _e7: Plane = self_2585;
    let _e8: Plane = plane_reversal(_e7);
    let _e9: Motor = plane_plane_geometric_product(_e6, _e8);
    let _e10: Scalar = motor_scalar_into(_e9);
    return _e10;
}

fn plane_translator_geometric_quotient(self_2586: Plane, other_2288: Translator) -> MotorDual {
    var self_2587: Plane;
    var other_2289: Translator;

    self_2587 = self_2586;
    other_2289 = other_2288;
    let _e4: Plane = self_2587;
    let _e5: Translator = other_2289;
    let _e6: Translator = translator_inverse(_e5);
    let _e7: MotorDual = plane_translator_geometric_product(_e4, _e6);
    return _e7;
}

fn plane_translator_transformation(self_2588: Plane, other_2290: Translator) -> Translator {
    var self_2589: Plane;
    var other_2291: Translator;

    self_2589 = self_2588;
    other_2291 = other_2290;
    let _e4: Plane = self_2589;
    let _e5: Translator = other_2291;
    let _e6: MotorDual = plane_translator_geometric_product(_e4, _e5);
    let _e7: Plane = self_2589;
    let _e8: Plane = plane_reversal(_e7);
    let _e9: Motor = motor_dual_plane_geometric_product(_e6, _e8);
    let _e10: Translator = motor_translator_into(_e9);
    return _e10;
}

fn point_anti_scalar_geometric_quotient(self_2590: Point, other_2292: AntiScalar) -> Plane {
    var self_2591: Point;
    var other_2293: AntiScalar;

    self_2591 = self_2590;
    other_2293 = other_2292;
    let _e4: Point = self_2591;
    let _e5: AntiScalar = other_2293;
    let _e6: AntiScalar = anti_scalar_inverse(_e5);
    let _e7: Plane = point_anti_scalar_geometric_product(_e4, _e6);
    return _e7;
}

fn point_anti_scalar_transformation(self_2592: Point, other_2294: AntiScalar) -> AntiScalar {
    var self_2593: Point;
    var other_2295: AntiScalar;

    self_2593 = self_2592;
    other_2295 = other_2294;
    let _e4: Point = self_2593;
    let _e5: AntiScalar = other_2295;
    let _e6: Plane = point_anti_scalar_geometric_product(_e4, _e5);
    let _e7: Point = self_2593;
    let _e8: Point = point_reversal(_e7);
    let _e9: MotorDual = plane_point_geometric_product(_e6, _e8);
    let _e10: AntiScalar = motor_dual_anti_scalar_into(_e9);
    return _e10;
}

fn point_ideal_point_geometric_quotient(self_2594: Point, other_2296: IdealPoint) -> Motor {
    var self_2595: Point;
    var other_2297: IdealPoint;

    self_2595 = self_2594;
    other_2297 = other_2296;
    let _e4: Point = self_2595;
    let _e5: IdealPoint = other_2297;
    let _e6: IdealPoint = ideal_point_inverse(_e5);
    let _e7: Motor = point_ideal_point_geometric_product(_e4, _e6);
    return _e7;
}

fn point_ideal_point_transformation(self_2596: Point, other_2298: IdealPoint) -> IdealPoint {
    var self_2597: Point;
    var other_2299: IdealPoint;

    self_2597 = self_2596;
    other_2299 = other_2298;
    let _e4: Point = self_2597;
    let _e5: IdealPoint = other_2299;
    let _e6: Motor = point_ideal_point_geometric_product(_e4, _e5);
    let _e7: Point = self_2597;
    let _e8: Point = point_reversal(_e7);
    let _e9: Motor = motor_point_geometric_product(_e6, _e8);
    let _e10: IdealPoint = motor_ideal_point_into(_e9);
    return _e10;
}

fn point_motor_geometric_quotient(self_2598: Point, other_2300: Motor) -> Motor {
    var self_2599: Point;
    var other_2301: Motor;

    self_2599 = self_2598;
    other_2301 = other_2300;
    let _e4: Point = self_2599;
    let _e5: Motor = other_2301;
    let _e6: Motor = motor_inverse(_e5);
    let _e7: Motor = point_motor_geometric_product(_e4, _e6);
    return _e7;
}

fn point_motor_transformation(self_2600: Point, other_2302: Motor) -> Motor {
    var self_2601: Point;
    var other_2303: Motor;

    self_2601 = self_2600;
    other_2303 = other_2302;
    let _e4: Point = self_2601;
    let _e5: Motor = other_2303;
    let _e6: Motor = point_motor_geometric_product(_e4, _e5);
    let _e7: Point = self_2601;
    let _e8: Point = point_reversal(_e7);
    let _e9: Motor = motor_point_geometric_product(_e6, _e8);
    return _e9;
}

fn point_motor_dual_geometric_quotient(self_2602: Point, other_2304: MotorDual) -> MotorDual {
    var self_2603: Point;
    var other_2305: MotorDual;

    self_2603 = self_2602;
    other_2305 = other_2304;
    let _e4: Point = self_2603;
    let _e5: MotorDual = other_2305;
    let _e6: MotorDual = motor_dual_inverse(_e5);
    let _e7: MotorDual = point_motor_dual_geometric_product(_e4, _e6);
    return _e7;
}

fn point_motor_dual_transformation(self_2604: Point, other_2306: MotorDual) -> MotorDual {
    var self_2605: Point;
    var other_2307: MotorDual;

    self_2605 = self_2604;
    other_2307 = other_2306;
    let _e4: Point = self_2605;
    let _e5: MotorDual = other_2307;
    let _e6: MotorDual = point_motor_dual_geometric_product(_e4, _e5);
    let _e7: Point = self_2605;
    let _e8: Point = point_reversal(_e7);
    let _e9: MotorDual = motor_dual_point_geometric_product(_e6, _e8);
    return _e9;
}

fn point_multi_vector_geometric_quotient(self_2606: Point, other_2308: MultiVector) -> MultiVector {
    var self_2607: Point;
    var other_2309: MultiVector;

    self_2607 = self_2606;
    other_2309 = other_2308;
    let _e4: Point = self_2607;
    let _e5: MultiVector = other_2309;
    let _e6: MultiVector = multi_vector_inverse(_e5);
    let _e7: MultiVector = point_multi_vector_geometric_product(_e4, _e6);
    return _e7;
}

fn point_multi_vector_transformation(self_2608: Point, other_2310: MultiVector) -> MultiVector {
    var self_2609: Point;
    var other_2311: MultiVector;

    self_2609 = self_2608;
    other_2311 = other_2310;
    let _e4: Point = self_2609;
    let _e5: MultiVector = other_2311;
    let _e6: MultiVector = point_multi_vector_geometric_product(_e4, _e5);
    let _e7: Point = self_2609;
    let _e8: Point = point_reversal(_e7);
    let _e9: MultiVector = multi_vector_point_geometric_product(_e6, _e8);
    return _e9;
}

fn point_plane_geometric_quotient(self_2610: Point, other_2312: Plane) -> MotorDual {
    var self_2611: Point;
    var other_2313: Plane;

    self_2611 = self_2610;
    other_2313 = other_2312;
    let _e4: Point = self_2611;
    let _e5: Plane = other_2313;
    let _e6: Plane = plane_inverse(_e5);
    let _e7: MotorDual = point_plane_geometric_product(_e4, _e6);
    return _e7;
}

fn point_plane_transformation(self_2612: Point, other_2314: Plane) -> Plane {
    var self_2613: Point;
    var other_2315: Plane;

    self_2613 = self_2612;
    other_2315 = other_2314;
    let _e4: Point = self_2613;
    let _e5: Plane = other_2315;
    let _e6: MotorDual = point_plane_geometric_product(_e4, _e5);
    let _e7: Point = self_2613;
    let _e8: Point = point_reversal(_e7);
    let _e9: MotorDual = motor_dual_point_geometric_product(_e6, _e8);
    let _e10: Plane = motor_dual_plane_into(_e9);
    return _e10;
}

fn point_point_geometric_quotient(self_2614: Point, other_2316: Point) -> Motor {
    var self_2615: Point;
    var other_2317: Point;

    self_2615 = self_2614;
    other_2317 = other_2316;
    let _e4: Point = self_2615;
    let _e5: Point = other_2317;
    let _e6: Point = point_inverse(_e5);
    let _e7: Motor = point_point_geometric_product(_e4, _e6);
    return _e7;
}

fn point_point_transformation(self_2616: Point, other_2318: Point) -> Point {
    var self_2617: Point;
    var other_2319: Point;

    self_2617 = self_2616;
    other_2319 = other_2318;
    let _e4: Point = self_2617;
    let _e5: Point = other_2319;
    let _e6: Motor = point_point_geometric_product(_e4, _e5);
    let _e7: Point = self_2617;
    let _e8: Point = point_reversal(_e7);
    let _e9: Motor = motor_point_geometric_product(_e6, _e8);
    let _e10: Point = motor_point_into(_e9);
    return _e10;
}

fn point_rotor_geometric_quotient(self_2618: Point, other_2320: Rotor) -> Motor {
    var self_2619: Point;
    var other_2321: Rotor;

    self_2619 = self_2618;
    other_2321 = other_2320;
    let _e4: Point = self_2619;
    let _e5: Rotor = other_2321;
    let _e6: Rotor = rotor_inverse(_e5);
    let _e7: Motor = point_rotor_geometric_product(_e4, _e6);
    return _e7;
}

fn point_rotor_transformation(self_2620: Point, other_2322: Rotor) -> Rotor {
    var self_2621: Point;
    var other_2323: Rotor;

    self_2621 = self_2620;
    other_2323 = other_2322;
    let _e4: Point = self_2621;
    let _e5: Rotor = other_2323;
    let _e6: Motor = point_rotor_geometric_product(_e4, _e5);
    let _e7: Point = self_2621;
    let _e8: Point = point_reversal(_e7);
    let _e9: Motor = motor_point_geometric_product(_e6, _e8);
    let _e10: Rotor = motor_rotor_into(_e9);
    return _e10;
}

fn point_scalar_geometric_quotient(self_2622: Point, other_2324: Scalar) -> Point {
    var self_2623: Point;
    var other_2325: Scalar;

    self_2623 = self_2622;
    other_2325 = other_2324;
    let _e4: Point = self_2623;
    let _e5: Scalar = other_2325;
    let _e6: Scalar = scalar_inverse(_e5);
    let _e7: Point = point_scalar_geometric_product(_e4, _e6);
    return _e7;
}

fn point_scalar_transformation(self_2624: Point, other_2326: Scalar) -> Scalar {
    var self_2625: Point;
    var other_2327: Scalar;

    self_2625 = self_2624;
    other_2327 = other_2326;
    let _e4: Point = self_2625;
    let _e5: Scalar = other_2327;
    let _e6: Point = point_scalar_geometric_product(_e4, _e5);
    let _e7: Point = self_2625;
    let _e8: Point = point_reversal(_e7);
    let _e9: Motor = point_point_geometric_product(_e6, _e8);
    let _e10: Scalar = motor_scalar_into(_e9);
    return _e10;
}

fn point_translator_geometric_quotient(self_2626: Point, other_2328: Translator) -> Motor {
    var self_2627: Point;
    var other_2329: Translator;

    self_2627 = self_2626;
    other_2329 = other_2328;
    let _e4: Point = self_2627;
    let _e5: Translator = other_2329;
    let _e6: Translator = translator_inverse(_e5);
    let _e7: Motor = point_translator_geometric_product(_e4, _e6);
    return _e7;
}

fn point_translator_transformation(self_2628: Point, other_2330: Translator) -> Translator {
    var self_2629: Point;
    var other_2331: Translator;

    self_2629 = self_2628;
    other_2331 = other_2330;
    let _e4: Point = self_2629;
    let _e5: Translator = other_2331;
    let _e6: Motor = point_translator_geometric_product(_e4, _e5);
    let _e7: Point = self_2629;
    let _e8: Point = point_reversal(_e7);
    let _e9: Motor = motor_point_geometric_product(_e6, _e8);
    let _e10: Translator = motor_translator_into(_e9);
    return _e10;
}

fn rotor_ideal_point_geometric_quotient(self_2630: Rotor, other_2332: IdealPoint) -> IdealPoint {
    var self_2631: Rotor;
    var other_2333: IdealPoint;

    self_2631 = self_2630;
    other_2333 = other_2332;
    let _e4: Rotor = self_2631;
    let _e5: IdealPoint = other_2333;
    let _e6: IdealPoint = ideal_point_inverse(_e5);
    let _e7: IdealPoint = rotor_ideal_point_geometric_product(_e4, _e6);
    return _e7;
}

fn rotor_ideal_point_transformation(self_2632: Rotor, other_2334: IdealPoint) -> IdealPoint {
    var self_2633: Rotor;
    var other_2335: IdealPoint;

    self_2633 = self_2632;
    other_2335 = other_2334;
    let _e4: Rotor = self_2633;
    let _e5: IdealPoint = other_2335;
    let _e6: IdealPoint = rotor_ideal_point_geometric_product(_e4, _e5);
    let _e7: Rotor = self_2633;
    let _e8: Rotor = rotor_reversal(_e7);
    let _e9: IdealPoint = ideal_point_rotor_geometric_product(_e6, _e8);
    return _e9;
}

fn rotor_motor_geometric_quotient(self_2634: Rotor, other_2336: Motor) -> Motor {
    var self_2635: Rotor;
    var other_2337: Motor;

    self_2635 = self_2634;
    other_2337 = other_2336;
    let _e4: Rotor = self_2635;
    let _e5: Motor = other_2337;
    let _e6: Motor = motor_inverse(_e5);
    let _e7: Motor = rotor_motor_geometric_product(_e4, _e6);
    return _e7;
}

fn rotor_motor_transformation(self_2636: Rotor, other_2338: Motor) -> Motor {
    var self_2637: Rotor;
    var other_2339: Motor;

    self_2637 = self_2636;
    other_2339 = other_2338;
    let _e4: Rotor = self_2637;
    let _e5: Motor = other_2339;
    let _e6: Motor = rotor_motor_geometric_product(_e4, _e5);
    let _e7: Rotor = self_2637;
    let _e8: Rotor = rotor_reversal(_e7);
    let _e9: Motor = motor_rotor_geometric_product(_e6, _e8);
    return _e9;
}

fn rotor_motor_dual_geometric_quotient(self_2638: Rotor, other_2340: MotorDual) -> MotorDual {
    var self_2639: Rotor;
    var other_2341: MotorDual;

    self_2639 = self_2638;
    other_2341 = other_2340;
    let _e4: Rotor = self_2639;
    let _e5: MotorDual = other_2341;
    let _e6: MotorDual = motor_dual_inverse(_e5);
    let _e7: MotorDual = rotor_motor_dual_geometric_product(_e4, _e6);
    return _e7;
}

fn rotor_motor_dual_transformation(self_2640: Rotor, other_2342: MotorDual) -> MotorDual {
    var self_2641: Rotor;
    var other_2343: MotorDual;

    self_2641 = self_2640;
    other_2343 = other_2342;
    let _e4: Rotor = self_2641;
    let _e5: MotorDual = other_2343;
    let _e6: MotorDual = rotor_motor_dual_geometric_product(_e4, _e5);
    let _e7: Rotor = self_2641;
    let _e8: Rotor = rotor_reversal(_e7);
    let _e9: MotorDual = motor_dual_rotor_geometric_product(_e6, _e8);
    return _e9;
}

fn rotor_multi_vector_geometric_quotient(self_2642: Rotor, other_2344: MultiVector) -> MultiVector {
    var self_2643: Rotor;
    var other_2345: MultiVector;

    self_2643 = self_2642;
    other_2345 = other_2344;
    let _e4: Rotor = self_2643;
    let _e5: MultiVector = other_2345;
    let _e6: MultiVector = multi_vector_inverse(_e5);
    let _e7: MultiVector = rotor_multi_vector_geometric_product(_e4, _e6);
    return _e7;
}

fn rotor_multi_vector_transformation(self_2644: Rotor, other_2346: MultiVector) -> MultiVector {
    var self_2645: Rotor;
    var other_2347: MultiVector;

    self_2645 = self_2644;
    other_2347 = other_2346;
    let _e4: Rotor = self_2645;
    let _e5: MultiVector = other_2347;
    let _e6: MultiVector = rotor_multi_vector_geometric_product(_e4, _e5);
    let _e7: Rotor = self_2645;
    let _e8: Rotor = rotor_reversal(_e7);
    let _e9: MultiVector = multi_vector_rotor_geometric_product(_e6, _e8);
    return _e9;
}

fn rotor_plane_geometric_quotient(self_2646: Rotor, other_2348: Plane) -> MotorDual {
    var self_2647: Rotor;
    var other_2349: Plane;

    self_2647 = self_2646;
    other_2349 = other_2348;
    let _e4: Rotor = self_2647;
    let _e5: Plane = other_2349;
    let _e6: Plane = plane_inverse(_e5);
    let _e7: MotorDual = rotor_plane_geometric_product(_e4, _e6);
    return _e7;
}

fn rotor_plane_transformation(self_2648: Rotor, other_2350: Plane) -> Plane {
    var self_2649: Rotor;
    var other_2351: Plane;

    self_2649 = self_2648;
    other_2351 = other_2350;
    let _e4: Rotor = self_2649;
    let _e5: Plane = other_2351;
    let _e6: MotorDual = rotor_plane_geometric_product(_e4, _e5);
    let _e7: Rotor = self_2649;
    let _e8: Rotor = rotor_reversal(_e7);
    let _e9: MotorDual = motor_dual_rotor_geometric_product(_e6, _e8);
    let _e10: Plane = motor_dual_plane_into(_e9);
    return _e10;
}

fn rotor_point_geometric_quotient(self_2650: Rotor, other_2352: Point) -> Motor {
    var self_2651: Rotor;
    var other_2353: Point;

    self_2651 = self_2650;
    other_2353 = other_2352;
    let _e4: Rotor = self_2651;
    let _e5: Point = other_2353;
    let _e6: Point = point_inverse(_e5);
    let _e7: Motor = rotor_point_geometric_product(_e4, _e6);
    return _e7;
}

fn rotor_point_transformation(self_2652: Rotor, other_2354: Point) -> Point {
    var self_2653: Rotor;
    var other_2355: Point;

    self_2653 = self_2652;
    other_2355 = other_2354;
    let _e4: Rotor = self_2653;
    let _e5: Point = other_2355;
    let _e6: Motor = rotor_point_geometric_product(_e4, _e5);
    let _e7: Rotor = self_2653;
    let _e8: Rotor = rotor_reversal(_e7);
    let _e9: Motor = motor_rotor_geometric_product(_e6, _e8);
    let _e10: Point = motor_point_into(_e9);
    return _e10;
}

fn rotor_powi(self_2654: Rotor, exponent_4: i32) -> Rotor {
    var self_2655: Rotor;
    var exponent_5: i32;
    var local_2: Rotor;
    var x_2: Rotor;
    var y_2: Rotor;
    var n_2: i32;

    self_2655 = self_2654;
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
        let _e11: Rotor = self_2655;
        let _e12: Rotor = rotor_inverse(_e11);
        local_2 = _e12;
    } else {
        let _e14: Rotor = self_2655;
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

fn rotor_rotor_geometric_quotient(self_2656: Rotor, other_2356: Rotor) -> Rotor {
    var self_2657: Rotor;
    var other_2357: Rotor;

    self_2657 = self_2656;
    other_2357 = other_2356;
    let _e4: Rotor = self_2657;
    let _e5: Rotor = other_2357;
    let _e6: Rotor = rotor_inverse(_e5);
    let _e7: Rotor = rotor_rotor_geometric_product(_e4, _e6);
    return _e7;
}

fn rotor_rotor_transformation(self_2658: Rotor, other_2358: Rotor) -> Rotor {
    var self_2659: Rotor;
    var other_2359: Rotor;

    self_2659 = self_2658;
    other_2359 = other_2358;
    let _e4: Rotor = self_2659;
    let _e5: Rotor = other_2359;
    let _e6: Rotor = rotor_rotor_geometric_product(_e4, _e5);
    let _e7: Rotor = self_2659;
    let _e8: Rotor = rotor_reversal(_e7);
    let _e9: Rotor = rotor_rotor_geometric_product(_e6, _e8);
    return _e9;
}

fn rotor_scalar_geometric_quotient(self_2660: Rotor, other_2360: Scalar) -> Rotor {
    var self_2661: Rotor;
    var other_2361: Scalar;

    self_2661 = self_2660;
    other_2361 = other_2360;
    let _e4: Rotor = self_2661;
    let _e5: Scalar = other_2361;
    let _e6: Scalar = scalar_inverse(_e5);
    let _e7: Rotor = rotor_scalar_geometric_product(_e4, _e6);
    return _e7;
}

fn rotor_scalar_transformation(self_2662: Rotor, other_2362: Scalar) -> Scalar {
    var self_2663: Rotor;
    var other_2363: Scalar;

    self_2663 = self_2662;
    other_2363 = other_2362;
    let _e4: Rotor = self_2663;
    let _e5: Scalar = other_2363;
    let _e6: Rotor = rotor_scalar_geometric_product(_e4, _e5);
    let _e7: Rotor = self_2663;
    let _e8: Rotor = rotor_reversal(_e7);
    let _e9: Rotor = rotor_rotor_geometric_product(_e6, _e8);
    let _e10: Scalar = rotor_scalar_into(_e9);
    return _e10;
}

fn rotor_translator_geometric_quotient(self_2664: Rotor, other_2364: Translator) -> Motor {
    var self_2665: Rotor;
    var other_2365: Translator;

    self_2665 = self_2664;
    other_2365 = other_2364;
    let _e4: Rotor = self_2665;
    let _e5: Translator = other_2365;
    let _e6: Translator = translator_inverse(_e5);
    let _e7: Motor = rotor_translator_geometric_product(_e4, _e6);
    return _e7;
}

fn rotor_translator_transformation(self_2666: Rotor, other_2366: Translator) -> Translator {
    var self_2667: Rotor;
    var other_2367: Translator;

    self_2667 = self_2666;
    other_2367 = other_2366;
    let _e4: Rotor = self_2667;
    let _e5: Translator = other_2367;
    let _e6: Motor = rotor_translator_geometric_product(_e4, _e5);
    let _e7: Rotor = self_2667;
    let _e8: Rotor = rotor_reversal(_e7);
    let _e9: Motor = motor_rotor_geometric_product(_e6, _e8);
    let _e10: Translator = motor_translator_into(_e9);
    return _e10;
}

fn scalar_anti_scalar_geometric_quotient(self_2668: Scalar, other_2368: AntiScalar) -> AntiScalar {
    var self_2669: Scalar;
    var other_2369: AntiScalar;

    self_2669 = self_2668;
    other_2369 = other_2368;
    let _e4: Scalar = self_2669;
    let _e5: AntiScalar = other_2369;
    let _e6: AntiScalar = anti_scalar_inverse(_e5);
    let _e7: AntiScalar = scalar_anti_scalar_geometric_product(_e4, _e6);
    return _e7;
}

fn scalar_anti_scalar_transformation(self_2670: Scalar, other_2370: AntiScalar) -> AntiScalar {
    var self_2671: Scalar;
    var other_2371: AntiScalar;

    self_2671 = self_2670;
    other_2371 = other_2370;
    let _e4: Scalar = self_2671;
    let _e5: AntiScalar = other_2371;
    let _e6: AntiScalar = scalar_anti_scalar_geometric_product(_e4, _e5);
    let _e7: Scalar = self_2671;
    let _e8: Scalar = scalar_reversal(_e7);
    let _e9: AntiScalar = anti_scalar_scalar_geometric_product(_e6, _e8);
    return _e9;
}

fn scalar_ideal_point_geometric_quotient(self_2672: Scalar, other_2372: IdealPoint) -> IdealPoint {
    var self_2673: Scalar;
    var other_2373: IdealPoint;

    self_2673 = self_2672;
    other_2373 = other_2372;
    let _e4: Scalar = self_2673;
    let _e5: IdealPoint = other_2373;
    let _e6: IdealPoint = ideal_point_inverse(_e5);
    let _e7: IdealPoint = scalar_ideal_point_geometric_product(_e4, _e6);
    return _e7;
}

fn scalar_ideal_point_transformation(self_2674: Scalar, other_2374: IdealPoint) -> IdealPoint {
    var self_2675: Scalar;
    var other_2375: IdealPoint;

    self_2675 = self_2674;
    other_2375 = other_2374;
    let _e4: Scalar = self_2675;
    let _e5: IdealPoint = other_2375;
    let _e6: IdealPoint = scalar_ideal_point_geometric_product(_e4, _e5);
    let _e7: Scalar = self_2675;
    let _e8: Scalar = scalar_reversal(_e7);
    let _e9: IdealPoint = ideal_point_scalar_geometric_product(_e6, _e8);
    return _e9;
}

fn scalar_motor_geometric_quotient(self_2676: Scalar, other_2376: Motor) -> Motor {
    var self_2677: Scalar;
    var other_2377: Motor;

    self_2677 = self_2676;
    other_2377 = other_2376;
    let _e4: Scalar = self_2677;
    let _e5: Motor = other_2377;
    let _e6: Motor = motor_inverse(_e5);
    let _e7: Motor = scalar_motor_geometric_product(_e4, _e6);
    return _e7;
}

fn scalar_motor_transformation(self_2678: Scalar, other_2378: Motor) -> Motor {
    var self_2679: Scalar;
    var other_2379: Motor;

    self_2679 = self_2678;
    other_2379 = other_2378;
    let _e4: Scalar = self_2679;
    let _e5: Motor = other_2379;
    let _e6: Motor = scalar_motor_geometric_product(_e4, _e5);
    let _e7: Scalar = self_2679;
    let _e8: Scalar = scalar_reversal(_e7);
    let _e9: Motor = motor_scalar_geometric_product(_e6, _e8);
    return _e9;
}

fn scalar_motor_dual_geometric_quotient(self_2680: Scalar, other_2380: MotorDual) -> MotorDual {
    var self_2681: Scalar;
    var other_2381: MotorDual;

    self_2681 = self_2680;
    other_2381 = other_2380;
    let _e4: Scalar = self_2681;
    let _e5: MotorDual = other_2381;
    let _e6: MotorDual = motor_dual_inverse(_e5);
    let _e7: MotorDual = scalar_motor_dual_geometric_product(_e4, _e6);
    return _e7;
}

fn scalar_motor_dual_transformation(self_2682: Scalar, other_2382: MotorDual) -> MotorDual {
    var self_2683: Scalar;
    var other_2383: MotorDual;

    self_2683 = self_2682;
    other_2383 = other_2382;
    let _e4: Scalar = self_2683;
    let _e5: MotorDual = other_2383;
    let _e6: MotorDual = scalar_motor_dual_geometric_product(_e4, _e5);
    let _e7: Scalar = self_2683;
    let _e8: Scalar = scalar_reversal(_e7);
    let _e9: MotorDual = motor_dual_scalar_geometric_product(_e6, _e8);
    return _e9;
}

fn scalar_multi_vector_geometric_quotient(self_2684: Scalar, other_2384: MultiVector) -> MultiVector {
    var self_2685: Scalar;
    var other_2385: MultiVector;

    self_2685 = self_2684;
    other_2385 = other_2384;
    let _e4: Scalar = self_2685;
    let _e5: MultiVector = other_2385;
    let _e6: MultiVector = multi_vector_inverse(_e5);
    let _e7: MultiVector = scalar_multi_vector_geometric_product(_e4, _e6);
    return _e7;
}

fn scalar_multi_vector_transformation(self_2686: Scalar, other_2386: MultiVector) -> MultiVector {
    var self_2687: Scalar;
    var other_2387: MultiVector;

    self_2687 = self_2686;
    other_2387 = other_2386;
    let _e4: Scalar = self_2687;
    let _e5: MultiVector = other_2387;
    let _e6: MultiVector = scalar_multi_vector_geometric_product(_e4, _e5);
    let _e7: Scalar = self_2687;
    let _e8: Scalar = scalar_reversal(_e7);
    let _e9: MultiVector = multi_vector_scalar_geometric_product(_e6, _e8);
    return _e9;
}

fn scalar_plane_geometric_quotient(self_2688: Scalar, other_2388: Plane) -> Plane {
    var self_2689: Scalar;
    var other_2389: Plane;

    self_2689 = self_2688;
    other_2389 = other_2388;
    let _e4: Scalar = self_2689;
    let _e5: Plane = other_2389;
    let _e6: Plane = plane_inverse(_e5);
    let _e7: Plane = scalar_plane_geometric_product(_e4, _e6);
    return _e7;
}

fn scalar_plane_transformation(self_2690: Scalar, other_2390: Plane) -> Plane {
    var self_2691: Scalar;
    var other_2391: Plane;

    self_2691 = self_2690;
    other_2391 = other_2390;
    let _e4: Scalar = self_2691;
    let _e5: Plane = other_2391;
    let _e6: Plane = scalar_plane_geometric_product(_e4, _e5);
    let _e7: Scalar = self_2691;
    let _e8: Scalar = scalar_reversal(_e7);
    let _e9: Plane = plane_scalar_geometric_product(_e6, _e8);
    return _e9;
}

fn scalar_point_geometric_quotient(self_2692: Scalar, other_2392: Point) -> Point {
    var self_2693: Scalar;
    var other_2393: Point;

    self_2693 = self_2692;
    other_2393 = other_2392;
    let _e4: Scalar = self_2693;
    let _e5: Point = other_2393;
    let _e6: Point = point_inverse(_e5);
    let _e7: Point = scalar_point_geometric_product(_e4, _e6);
    return _e7;
}

fn scalar_point_transformation(self_2694: Scalar, other_2394: Point) -> Point {
    var self_2695: Scalar;
    var other_2395: Point;

    self_2695 = self_2694;
    other_2395 = other_2394;
    let _e4: Scalar = self_2695;
    let _e5: Point = other_2395;
    let _e6: Point = scalar_point_geometric_product(_e4, _e5);
    let _e7: Scalar = self_2695;
    let _e8: Scalar = scalar_reversal(_e7);
    let _e9: Point = point_scalar_geometric_product(_e6, _e8);
    return _e9;
}

fn scalar_rotor_geometric_quotient(self_2696: Scalar, other_2396: Rotor) -> Rotor {
    var self_2697: Scalar;
    var other_2397: Rotor;

    self_2697 = self_2696;
    other_2397 = other_2396;
    let _e4: Scalar = self_2697;
    let _e5: Rotor = other_2397;
    let _e6: Rotor = rotor_inverse(_e5);
    let _e7: Rotor = scalar_rotor_geometric_product(_e4, _e6);
    return _e7;
}

fn scalar_rotor_transformation(self_2698: Scalar, other_2398: Rotor) -> Rotor {
    var self_2699: Scalar;
    var other_2399: Rotor;

    self_2699 = self_2698;
    other_2399 = other_2398;
    let _e4: Scalar = self_2699;
    let _e5: Rotor = other_2399;
    let _e6: Rotor = scalar_rotor_geometric_product(_e4, _e5);
    let _e7: Scalar = self_2699;
    let _e8: Scalar = scalar_reversal(_e7);
    let _e9: Rotor = rotor_scalar_geometric_product(_e6, _e8);
    return _e9;
}

fn scalar_powi(self_2700: Scalar, exponent_6: i32) -> Scalar {
    var self_2701: Scalar;
    var exponent_7: i32;
    var local_3: Scalar;
    var x_3: Scalar;
    var y_3: Scalar;
    var n_3: i32;

    self_2701 = self_2700;
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
        let _e11: Scalar = self_2701;
        let _e12: Scalar = scalar_inverse(_e11);
        local_3 = _e12;
    } else {
        let _e14: Scalar = self_2701;
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

fn scalar_scalar_geometric_quotient(self_2702: Scalar, other_2400: Scalar) -> Scalar {
    var self_2703: Scalar;
    var other_2401: Scalar;

    self_2703 = self_2702;
    other_2401 = other_2400;
    let _e4: Scalar = self_2703;
    let _e5: Scalar = other_2401;
    let _e6: Scalar = scalar_inverse(_e5);
    let _e7: Scalar = scalar_scalar_geometric_product(_e4, _e6);
    return _e7;
}

fn scalar_scalar_transformation(self_2704: Scalar, other_2402: Scalar) -> Scalar {
    var self_2705: Scalar;
    var other_2403: Scalar;

    self_2705 = self_2704;
    other_2403 = other_2402;
    let _e4: Scalar = self_2705;
    let _e5: Scalar = other_2403;
    let _e6: Scalar = scalar_scalar_geometric_product(_e4, _e5);
    let _e7: Scalar = self_2705;
    let _e8: Scalar = scalar_reversal(_e7);
    let _e9: Scalar = scalar_scalar_geometric_product(_e6, _e8);
    return _e9;
}

fn scalar_translator_geometric_quotient(self_2706: Scalar, other_2404: Translator) -> Translator {
    var self_2707: Scalar;
    var other_2405: Translator;

    self_2707 = self_2706;
    other_2405 = other_2404;
    let _e4: Scalar = self_2707;
    let _e5: Translator = other_2405;
    let _e6: Translator = translator_inverse(_e5);
    let _e7: Translator = scalar_translator_geometric_product(_e4, _e6);
    return _e7;
}

fn scalar_translator_transformation(self_2708: Scalar, other_2406: Translator) -> Translator {
    var self_2709: Scalar;
    var other_2407: Translator;

    self_2709 = self_2708;
    other_2407 = other_2406;
    let _e4: Scalar = self_2709;
    let _e5: Translator = other_2407;
    let _e6: Translator = scalar_translator_geometric_product(_e4, _e5);
    let _e7: Scalar = self_2709;
    let _e8: Scalar = scalar_reversal(_e7);
    let _e9: Translator = translator_scalar_geometric_product(_e6, _e8);
    return _e9;
}

fn translator_ideal_point_geometric_quotient(self_2710: Translator, other_2408: IdealPoint) -> Motor {
    var self_2711: Translator;
    var other_2409: IdealPoint;

    self_2711 = self_2710;
    other_2409 = other_2408;
    let _e4: Translator = self_2711;
    let _e5: IdealPoint = other_2409;
    let _e6: IdealPoint = ideal_point_inverse(_e5);
    let _e7: Motor = translator_ideal_point_geometric_product(_e4, _e6);
    return _e7;
}

fn translator_ideal_point_transformation(self_2712: Translator, other_2410: IdealPoint) -> IdealPoint {
    var self_2713: Translator;
    var other_2411: IdealPoint;

    self_2713 = self_2712;
    other_2411 = other_2410;
    let _e4: Translator = self_2713;
    let _e5: IdealPoint = other_2411;
    let _e6: Motor = translator_ideal_point_geometric_product(_e4, _e5);
    let _e7: Translator = self_2713;
    let _e8: Translator = translator_reversal(_e7);
    let _e9: Motor = motor_translator_geometric_product(_e6, _e8);
    let _e10: IdealPoint = motor_ideal_point_into(_e9);
    return _e10;
}

fn translator_motor_geometric_quotient(self_2714: Translator, other_2412: Motor) -> Motor {
    var self_2715: Translator;
    var other_2413: Motor;

    self_2715 = self_2714;
    other_2413 = other_2412;
    let _e4: Translator = self_2715;
    let _e5: Motor = other_2413;
    let _e6: Motor = motor_inverse(_e5);
    let _e7: Motor = translator_motor_geometric_product(_e4, _e6);
    return _e7;
}

fn translator_motor_transformation(self_2716: Translator, other_2414: Motor) -> Motor {
    var self_2717: Translator;
    var other_2415: Motor;

    self_2717 = self_2716;
    other_2415 = other_2414;
    let _e4: Translator = self_2717;
    let _e5: Motor = other_2415;
    let _e6: Motor = translator_motor_geometric_product(_e4, _e5);
    let _e7: Translator = self_2717;
    let _e8: Translator = translator_reversal(_e7);
    let _e9: Motor = motor_translator_geometric_product(_e6, _e8);
    return _e9;
}

fn translator_motor_dual_geometric_quotient(self_2718: Translator, other_2416: MotorDual) -> MotorDual {
    var self_2719: Translator;
    var other_2417: MotorDual;

    self_2719 = self_2718;
    other_2417 = other_2416;
    let _e4: Translator = self_2719;
    let _e5: MotorDual = other_2417;
    let _e6: MotorDual = motor_dual_inverse(_e5);
    let _e7: MotorDual = translator_motor_dual_geometric_product(_e4, _e6);
    return _e7;
}

fn translator_motor_dual_transformation(self_2720: Translator, other_2418: MotorDual) -> MotorDual {
    var self_2721: Translator;
    var other_2419: MotorDual;

    self_2721 = self_2720;
    other_2419 = other_2418;
    let _e4: Translator = self_2721;
    let _e5: MotorDual = other_2419;
    let _e6: MotorDual = translator_motor_dual_geometric_product(_e4, _e5);
    let _e7: Translator = self_2721;
    let _e8: Translator = translator_reversal(_e7);
    let _e9: MotorDual = motor_dual_translator_geometric_product(_e6, _e8);
    return _e9;
}

fn translator_multi_vector_geometric_quotient(self_2722: Translator, other_2420: MultiVector) -> MultiVector {
    var self_2723: Translator;
    var other_2421: MultiVector;

    self_2723 = self_2722;
    other_2421 = other_2420;
    let _e4: Translator = self_2723;
    let _e5: MultiVector = other_2421;
    let _e6: MultiVector = multi_vector_inverse(_e5);
    let _e7: MultiVector = translator_multi_vector_geometric_product(_e4, _e6);
    return _e7;
}

fn translator_multi_vector_transformation(self_2724: Translator, other_2422: MultiVector) -> MultiVector {
    var self_2725: Translator;
    var other_2423: MultiVector;

    self_2725 = self_2724;
    other_2423 = other_2422;
    let _e4: Translator = self_2725;
    let _e5: MultiVector = other_2423;
    let _e6: MultiVector = translator_multi_vector_geometric_product(_e4, _e5);
    let _e7: Translator = self_2725;
    let _e8: Translator = translator_reversal(_e7);
    let _e9: MultiVector = multi_vector_translator_geometric_product(_e6, _e8);
    return _e9;
}

fn translator_plane_geometric_quotient(self_2726: Translator, other_2424: Plane) -> MotorDual {
    var self_2727: Translator;
    var other_2425: Plane;

    self_2727 = self_2726;
    other_2425 = other_2424;
    let _e4: Translator = self_2727;
    let _e5: Plane = other_2425;
    let _e6: Plane = plane_inverse(_e5);
    let _e7: MotorDual = translator_plane_geometric_product(_e4, _e6);
    return _e7;
}

fn translator_plane_transformation(self_2728: Translator, other_2426: Plane) -> Plane {
    var self_2729: Translator;
    var other_2427: Plane;

    self_2729 = self_2728;
    other_2427 = other_2426;
    let _e4: Translator = self_2729;
    let _e5: Plane = other_2427;
    let _e6: MotorDual = translator_plane_geometric_product(_e4, _e5);
    let _e7: Translator = self_2729;
    let _e8: Translator = translator_reversal(_e7);
    let _e9: MotorDual = motor_dual_translator_geometric_product(_e6, _e8);
    let _e10: Plane = motor_dual_plane_into(_e9);
    return _e10;
}

fn translator_point_geometric_quotient(self_2730: Translator, other_2428: Point) -> Motor {
    var self_2731: Translator;
    var other_2429: Point;

    self_2731 = self_2730;
    other_2429 = other_2428;
    let _e4: Translator = self_2731;
    let _e5: Point = other_2429;
    let _e6: Point = point_inverse(_e5);
    let _e7: Motor = translator_point_geometric_product(_e4, _e6);
    return _e7;
}

fn translator_point_transformation(self_2732: Translator, other_2430: Point) -> Point {
    var self_2733: Translator;
    var other_2431: Point;

    self_2733 = self_2732;
    other_2431 = other_2430;
    let _e4: Translator = self_2733;
    let _e5: Point = other_2431;
    let _e6: Motor = translator_point_geometric_product(_e4, _e5);
    let _e7: Translator = self_2733;
    let _e8: Translator = translator_reversal(_e7);
    let _e9: Motor = motor_translator_geometric_product(_e6, _e8);
    let _e10: Point = motor_point_into(_e9);
    return _e10;
}

fn translator_rotor_geometric_quotient(self_2734: Translator, other_2432: Rotor) -> Motor {
    var self_2735: Translator;
    var other_2433: Rotor;

    self_2735 = self_2734;
    other_2433 = other_2432;
    let _e4: Translator = self_2735;
    let _e5: Rotor = other_2433;
    let _e6: Rotor = rotor_inverse(_e5);
    let _e7: Motor = translator_rotor_geometric_product(_e4, _e6);
    return _e7;
}

fn translator_rotor_transformation(self_2736: Translator, other_2434: Rotor) -> Rotor {
    var self_2737: Translator;
    var other_2435: Rotor;

    self_2737 = self_2736;
    other_2435 = other_2434;
    let _e4: Translator = self_2737;
    let _e5: Rotor = other_2435;
    let _e6: Motor = translator_rotor_geometric_product(_e4, _e5);
    let _e7: Translator = self_2737;
    let _e8: Translator = translator_reversal(_e7);
    let _e9: Motor = motor_translator_geometric_product(_e6, _e8);
    let _e10: Rotor = motor_rotor_into(_e9);
    return _e10;
}

fn translator_scalar_geometric_quotient(self_2738: Translator, other_2436: Scalar) -> Translator {
    var self_2739: Translator;
    var other_2437: Scalar;

    self_2739 = self_2738;
    other_2437 = other_2436;
    let _e4: Translator = self_2739;
    let _e5: Scalar = other_2437;
    let _e6: Scalar = scalar_inverse(_e5);
    let _e7: Translator = translator_scalar_geometric_product(_e4, _e6);
    return _e7;
}

fn translator_scalar_transformation(self_2740: Translator, other_2438: Scalar) -> Scalar {
    var self_2741: Translator;
    var other_2439: Scalar;

    self_2741 = self_2740;
    other_2439 = other_2438;
    let _e4: Translator = self_2741;
    let _e5: Scalar = other_2439;
    let _e6: Translator = translator_scalar_geometric_product(_e4, _e5);
    let _e7: Translator = self_2741;
    let _e8: Translator = translator_reversal(_e7);
    let _e9: Motor = translator_translator_geometric_product(_e6, _e8);
    let _e10: Scalar = motor_scalar_into(_e9);
    return _e10;
}

fn translator_translator_geometric_quotient(self_2742: Translator, other_2440: Translator) -> Motor {
    var self_2743: Translator;
    var other_2441: Translator;

    self_2743 = self_2742;
    other_2441 = other_2440;
    let _e4: Translator = self_2743;
    let _e5: Translator = other_2441;
    let _e6: Translator = translator_inverse(_e5);
    let _e7: Motor = translator_translator_geometric_product(_e4, _e6);
    return _e7;
}

fn translator_translator_transformation(self_2744: Translator, other_2442: Translator) -> Translator {
    var self_2745: Translator;
    var other_2443: Translator;

    self_2745 = self_2744;
    other_2443 = other_2442;
    let _e4: Translator = self_2745;
    let _e5: Translator = other_2443;
    let _e6: Motor = translator_translator_geometric_product(_e4, _e5);
    let _e7: Translator = self_2745;
    let _e8: Translator = translator_reversal(_e7);
    let _e9: Motor = motor_translator_geometric_product(_e6, _e8);
    let _e10: Translator = motor_translator_into(_e9);
    return _e10;
}


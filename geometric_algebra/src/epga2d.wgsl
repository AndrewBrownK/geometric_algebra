struct Scalar {
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

fn scalar_multi_vector_add(self_28: Scalar, other_20: MultiVector) -> MultiVector {
    var self_29: Scalar;
    var other_21: MultiVector;

    self_29 = self_28;
    other_21 = other_20;
    let _e4: Scalar = self_29;
    let _e13: MultiVector = other_21;
    let _e16: MultiVector = other_21;
    return MultiVector(((vec4<f32>(_e4.g0_) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) + _e13.g0_), _e16.g1_);
}

fn scalar_multi_vector_sub(self_30: Scalar, other_22: MultiVector) -> MultiVector {
    var self_31: Scalar;
    var other_23: MultiVector;

    self_31 = self_30;
    other_23 = other_22;
    let _e4: Scalar = self_31;
    let _e13: MultiVector = other_23;
    let _e18: MultiVector = other_23;
    return MultiVector(((vec4<f32>(_e4.g0_) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) - _e13.g0_), (vec4<f32>(0.0) - _e18.g1_));
}

fn scalar_multi_vector_geometric_product(self_32: Scalar, other_24: MultiVector) -> MultiVector {
    var self_33: Scalar;
    var other_25: MultiVector;

    self_33 = self_32;
    other_25 = other_24;
    let _e4: Scalar = self_33;
    let _e7: MultiVector = other_25;
    let _e10: Scalar = self_33;
    let _e13: MultiVector = other_25;
    return MultiVector((vec4<f32>(_e4.g0_) * _e7.g0_), (vec4<f32>(_e10.g0_) * _e13.g1_));
}

fn scalar_multi_vector_regressive_product(self_34: Scalar, other_26: MultiVector) -> Scalar {
    var self_35: Scalar;
    var other_27: MultiVector;

    self_35 = self_34;
    other_27 = other_26;
    let _e4: Scalar = self_35;
    let _e6: MultiVector = other_27;
    return Scalar((_e4.g0_ * _e6.g1_.y));
}

fn scalar_multi_vector_outer_product(self_36: Scalar, other_28: MultiVector) -> MultiVector {
    var self_37: Scalar;
    var other_29: MultiVector;

    self_37 = self_36;
    other_29 = other_28;
    let _e4: Scalar = self_37;
    let _e7: MultiVector = other_29;
    let _e10: Scalar = self_37;
    let _e13: MultiVector = other_29;
    return MultiVector((vec4<f32>(_e4.g0_) * _e7.g0_), (vec4<f32>(_e10.g0_) * _e13.g1_));
}

fn scalar_multi_vector_inner_product(self_38: Scalar, other_30: MultiVector) -> MultiVector {
    var self_39: Scalar;
    var other_31: MultiVector;

    self_39 = self_38;
    other_31 = other_30;
    let _e4: Scalar = self_39;
    let _e7: MultiVector = other_31;
    let _e10: Scalar = self_39;
    let _e13: MultiVector = other_31;
    return MultiVector((vec4<f32>(_e4.g0_) * _e7.g0_), (vec4<f32>(_e10.g0_) * _e13.g1_));
}

fn scalar_multi_vector_left_contraction(self_40: Scalar, other_32: MultiVector) -> MultiVector {
    var self_41: Scalar;
    var other_33: MultiVector;

    self_41 = self_40;
    other_33 = other_32;
    let _e4: Scalar = self_41;
    let _e7: MultiVector = other_33;
    let _e10: Scalar = self_41;
    let _e13: MultiVector = other_33;
    return MultiVector((vec4<f32>(_e4.g0_) * _e7.g0_), (vec4<f32>(_e10.g0_) * _e13.g1_));
}

fn scalar_multi_vector_right_contraction(self_42: Scalar, other_34: MultiVector) -> Scalar {
    var self_43: Scalar;
    var other_35: MultiVector;

    self_43 = self_42;
    other_35 = other_34;
    let _e4: Scalar = self_43;
    let _e6: MultiVector = other_35;
    return Scalar((_e4.g0_ * _e6.g0_.x));
}

fn scalar_multi_vector_scalar_product(self_44: Scalar, other_36: MultiVector) -> Scalar {
    var self_45: Scalar;
    var other_37: MultiVector;

    self_45 = self_44;
    other_37 = other_36;
    let _e4: Scalar = self_45;
    let _e6: MultiVector = other_37;
    return Scalar((_e4.g0_ * _e6.g0_.x));
}

fn scalar_rotor_add(self_46: Scalar, other_38: Rotor) -> Rotor {
    var self_47: Scalar;
    var other_39: Rotor;

    self_47 = self_46;
    other_39 = other_38;
    let _e4: Scalar = self_47;
    let _e11: Rotor = other_39;
    return Rotor(((vec2<f32>(_e4.g0_) * vec2<f32>(1.0, 0.0)) + _e11.g0_));
}

fn scalar_rotor_sub(self_48: Scalar, other_40: Rotor) -> Rotor {
    var self_49: Scalar;
    var other_41: Rotor;

    self_49 = self_48;
    other_41 = other_40;
    let _e4: Scalar = self_49;
    let _e11: Rotor = other_41;
    return Rotor(((vec2<f32>(_e4.g0_) * vec2<f32>(1.0, 0.0)) - _e11.g0_));
}

fn scalar_rotor_geometric_product(self_50: Scalar, other_42: Rotor) -> Rotor {
    var self_51: Scalar;
    var other_43: Rotor;

    self_51 = self_50;
    other_43 = other_42;
    let _e4: Scalar = self_51;
    let _e7: Rotor = other_43;
    return Rotor((vec2<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_rotor_outer_product(self_52: Scalar, other_44: Rotor) -> Rotor {
    var self_53: Scalar;
    var other_45: Rotor;

    self_53 = self_52;
    other_45 = other_44;
    let _e4: Scalar = self_53;
    let _e7: Rotor = other_45;
    return Rotor((vec2<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_rotor_inner_product(self_54: Scalar, other_46: Rotor) -> Rotor {
    var self_55: Scalar;
    var other_47: Rotor;

    self_55 = self_54;
    other_47 = other_46;
    let _e4: Scalar = self_55;
    let _e7: Rotor = other_47;
    return Rotor((vec2<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_rotor_left_contraction(self_56: Scalar, other_48: Rotor) -> Rotor {
    var self_57: Scalar;
    var other_49: Rotor;

    self_57 = self_56;
    other_49 = other_48;
    let _e4: Scalar = self_57;
    let _e7: Rotor = other_49;
    return Rotor((vec2<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_rotor_right_contraction(self_58: Scalar, other_50: Rotor) -> Scalar {
    var self_59: Scalar;
    var other_51: Rotor;

    self_59 = self_58;
    other_51 = other_50;
    let _e4: Scalar = self_59;
    let _e6: Rotor = other_51;
    return Scalar((_e4.g0_ * _e6.g0_.x));
}

fn scalar_rotor_scalar_product(self_60: Scalar, other_52: Rotor) -> Scalar {
    var self_61: Scalar;
    var other_53: Rotor;

    self_61 = self_60;
    other_53 = other_52;
    let _e4: Scalar = self_61;
    let _e6: Rotor = other_53;
    return Scalar((_e4.g0_ * _e6.g0_.x));
}

fn scalar_point_add(self_62: Scalar, other_54: Point) -> Motor {
    var self_63: Scalar;
    var other_55: Point;

    self_63 = self_62;
    other_55 = other_54;
    let _e4: Scalar = self_63;
    let _e13: Point = other_55;
    let _e16: Point = other_55;
    let _e19: Point = other_55;
    let _e22: Point = other_55;
    return Motor(((vec4<f32>(_e4.g0_) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) + (vec4<f32>(_e13.g0_.x, _e16.g0_.x, _e19.g0_.y, _e22.g0_.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn scalar_point_sub(self_64: Scalar, other_56: Point) -> Motor {
    var self_65: Scalar;
    var other_57: Point;

    self_65 = self_64;
    other_57 = other_56;
    let _e4: Scalar = self_65;
    let _e13: Point = other_57;
    let _e16: Point = other_57;
    let _e19: Point = other_57;
    let _e22: Point = other_57;
    return Motor(((vec4<f32>(_e4.g0_) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) - (vec4<f32>(_e13.g0_.x, _e16.g0_.x, _e19.g0_.y, _e22.g0_.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn scalar_point_geometric_product(self_66: Scalar, other_58: Point) -> Point {
    var self_67: Scalar;
    var other_59: Point;

    self_67 = self_66;
    other_59 = other_58;
    let _e4: Scalar = self_67;
    let _e7: Point = other_59;
    return Point((vec3<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_point_outer_product(self_68: Scalar, other_60: Point) -> Point {
    var self_69: Scalar;
    var other_61: Point;

    self_69 = self_68;
    other_61 = other_60;
    let _e4: Scalar = self_69;
    let _e7: Point = other_61;
    return Point((vec3<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_point_inner_product(self_70: Scalar, other_62: Point) -> Point {
    var self_71: Scalar;
    var other_63: Point;

    self_71 = self_70;
    other_63 = other_62;
    let _e4: Scalar = self_71;
    let _e7: Point = other_63;
    return Point((vec3<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_point_left_contraction(self_72: Scalar, other_64: Point) -> Point {
    var self_73: Scalar;
    var other_65: Point;

    self_73 = self_72;
    other_65 = other_64;
    let _e4: Scalar = self_73;
    let _e7: Point = other_65;
    return Point((vec3<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_ideal_point_add(self_74: Scalar, other_66: IdealPoint) -> Translator {
    var self_75: Scalar;
    var other_67: IdealPoint;

    self_75 = self_74;
    other_67 = other_66;
    let _e4: Scalar = self_75;
    let _e12: IdealPoint = other_67;
    let _e15: IdealPoint = other_67;
    let _e18: IdealPoint = other_67;
    return Translator(((vec3<f32>(_e4.g0_) * vec3<f32>(1.0, 0.0, 0.0)) + (vec3<f32>(_e12.g0_.x, _e15.g0_.x, _e18.g0_.y) * vec3<f32>(0.0, 1.0, 1.0))));
}

fn scalar_ideal_point_sub(self_76: Scalar, other_68: IdealPoint) -> Translator {
    var self_77: Scalar;
    var other_69: IdealPoint;

    self_77 = self_76;
    other_69 = other_68;
    let _e4: Scalar = self_77;
    let _e12: IdealPoint = other_69;
    let _e15: IdealPoint = other_69;
    let _e18: IdealPoint = other_69;
    return Translator(((vec3<f32>(_e4.g0_) * vec3<f32>(1.0, 0.0, 0.0)) - (vec3<f32>(_e12.g0_.x, _e15.g0_.x, _e18.g0_.y) * vec3<f32>(0.0, 1.0, 1.0))));
}

fn scalar_ideal_point_geometric_product(self_78: Scalar, other_70: IdealPoint) -> IdealPoint {
    var self_79: Scalar;
    var other_71: IdealPoint;

    self_79 = self_78;
    other_71 = other_70;
    let _e4: Scalar = self_79;
    let _e7: IdealPoint = other_71;
    return IdealPoint((vec2<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_ideal_point_outer_product(self_80: Scalar, other_72: IdealPoint) -> IdealPoint {
    var self_81: Scalar;
    var other_73: IdealPoint;

    self_81 = self_80;
    other_73 = other_72;
    let _e4: Scalar = self_81;
    let _e7: IdealPoint = other_73;
    return IdealPoint((vec2<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_ideal_point_inner_product(self_82: Scalar, other_74: IdealPoint) -> IdealPoint {
    var self_83: Scalar;
    var other_75: IdealPoint;

    self_83 = self_82;
    other_75 = other_74;
    let _e4: Scalar = self_83;
    let _e7: IdealPoint = other_75;
    return IdealPoint((vec2<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_ideal_point_left_contraction(self_84: Scalar, other_76: IdealPoint) -> IdealPoint {
    var self_85: Scalar;
    var other_77: IdealPoint;

    self_85 = self_84;
    other_77 = other_76;
    let _e4: Scalar = self_85;
    let _e7: IdealPoint = other_77;
    return IdealPoint((vec2<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_plane_geometric_product(self_86: Scalar, other_78: Plane) -> Plane {
    var self_87: Scalar;
    var other_79: Plane;

    self_87 = self_86;
    other_79 = other_78;
    let _e4: Scalar = self_87;
    let _e7: Plane = other_79;
    return Plane((vec3<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_plane_outer_product(self_88: Scalar, other_80: Plane) -> Plane {
    var self_89: Scalar;
    var other_81: Plane;

    self_89 = self_88;
    other_81 = other_80;
    let _e4: Scalar = self_89;
    let _e7: Plane = other_81;
    return Plane((vec3<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_plane_inner_product(self_90: Scalar, other_82: Plane) -> Plane {
    var self_91: Scalar;
    var other_83: Plane;

    self_91 = self_90;
    other_83 = other_82;
    let _e4: Scalar = self_91;
    let _e7: Plane = other_83;
    return Plane((vec3<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_plane_left_contraction(self_92: Scalar, other_84: Plane) -> Plane {
    var self_93: Scalar;
    var other_85: Plane;

    self_93 = self_92;
    other_85 = other_84;
    let _e4: Scalar = self_93;
    let _e7: Plane = other_85;
    return Plane((vec3<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_translator_add(self_94: Scalar, other_86: Translator) -> Translator {
    var self_95: Scalar;
    var other_87: Translator;

    self_95 = self_94;
    other_87 = other_86;
    let _e4: Scalar = self_95;
    let _e12: Translator = other_87;
    return Translator(((vec3<f32>(_e4.g0_) * vec3<f32>(1.0, 0.0, 0.0)) + _e12.g0_));
}

fn scalar_translator_sub(self_96: Scalar, other_88: Translator) -> Translator {
    var self_97: Scalar;
    var other_89: Translator;

    self_97 = self_96;
    other_89 = other_88;
    let _e4: Scalar = self_97;
    let _e12: Translator = other_89;
    return Translator(((vec3<f32>(_e4.g0_) * vec3<f32>(1.0, 0.0, 0.0)) - _e12.g0_));
}

fn scalar_translator_geometric_product(self_98: Scalar, other_90: Translator) -> Translator {
    var self_99: Scalar;
    var other_91: Translator;

    self_99 = self_98;
    other_91 = other_90;
    let _e4: Scalar = self_99;
    let _e7: Translator = other_91;
    return Translator((vec3<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_translator_outer_product(self_100: Scalar, other_92: Translator) -> Translator {
    var self_101: Scalar;
    var other_93: Translator;

    self_101 = self_100;
    other_93 = other_92;
    let _e4: Scalar = self_101;
    let _e7: Translator = other_93;
    return Translator((vec3<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_translator_inner_product(self_102: Scalar, other_94: Translator) -> Translator {
    var self_103: Scalar;
    var other_95: Translator;

    self_103 = self_102;
    other_95 = other_94;
    let _e4: Scalar = self_103;
    let _e7: Translator = other_95;
    return Translator((vec3<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_translator_left_contraction(self_104: Scalar, other_96: Translator) -> Translator {
    var self_105: Scalar;
    var other_97: Translator;

    self_105 = self_104;
    other_97 = other_96;
    let _e4: Scalar = self_105;
    let _e7: Translator = other_97;
    return Translator((vec3<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_translator_right_contraction(self_106: Scalar, other_98: Translator) -> Scalar {
    var self_107: Scalar;
    var other_99: Translator;

    self_107 = self_106;
    other_99 = other_98;
    let _e4: Scalar = self_107;
    let _e6: Translator = other_99;
    return Scalar((_e4.g0_ * _e6.g0_.x));
}

fn scalar_translator_scalar_product(self_108: Scalar, other_100: Translator) -> Scalar {
    var self_109: Scalar;
    var other_101: Translator;

    self_109 = self_108;
    other_101 = other_100;
    let _e4: Scalar = self_109;
    let _e6: Translator = other_101;
    return Scalar((_e4.g0_ * _e6.g0_.x));
}

fn scalar_motor_add(self_110: Scalar, other_102: Motor) -> Motor {
    var self_111: Scalar;
    var other_103: Motor;

    self_111 = self_110;
    other_103 = other_102;
    let _e4: Scalar = self_111;
    let _e13: Motor = other_103;
    return Motor(((vec4<f32>(_e4.g0_) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) + _e13.g0_));
}

fn scalar_motor_sub(self_112: Scalar, other_104: Motor) -> Motor {
    var self_113: Scalar;
    var other_105: Motor;

    self_113 = self_112;
    other_105 = other_104;
    let _e4: Scalar = self_113;
    let _e13: Motor = other_105;
    return Motor(((vec4<f32>(_e4.g0_) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) - _e13.g0_));
}

fn scalar_motor_geometric_product(self_114: Scalar, other_106: Motor) -> Motor {
    var self_115: Scalar;
    var other_107: Motor;

    self_115 = self_114;
    other_107 = other_106;
    let _e4: Scalar = self_115;
    let _e7: Motor = other_107;
    return Motor((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_motor_outer_product(self_116: Scalar, other_108: Motor) -> Motor {
    var self_117: Scalar;
    var other_109: Motor;

    self_117 = self_116;
    other_109 = other_108;
    let _e4: Scalar = self_117;
    let _e7: Motor = other_109;
    return Motor((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_motor_inner_product(self_118: Scalar, other_110: Motor) -> Motor {
    var self_119: Scalar;
    var other_111: Motor;

    self_119 = self_118;
    other_111 = other_110;
    let _e4: Scalar = self_119;
    let _e7: Motor = other_111;
    return Motor((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_motor_left_contraction(self_120: Scalar, other_112: Motor) -> Motor {
    var self_121: Scalar;
    var other_113: Motor;

    self_121 = self_120;
    other_113 = other_112;
    let _e4: Scalar = self_121;
    let _e7: Motor = other_113;
    return Motor((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_motor_right_contraction(self_122: Scalar, other_114: Motor) -> Scalar {
    var self_123: Scalar;
    var other_115: Motor;

    self_123 = self_122;
    other_115 = other_114;
    let _e4: Scalar = self_123;
    let _e6: Motor = other_115;
    return Scalar((_e4.g0_ * _e6.g0_.x));
}

fn scalar_motor_scalar_product(self_124: Scalar, other_116: Motor) -> Scalar {
    var self_125: Scalar;
    var other_117: Motor;

    self_125 = self_124;
    other_117 = other_116;
    let _e4: Scalar = self_125;
    let _e6: Motor = other_117;
    return Scalar((_e4.g0_ * _e6.g0_.x));
}

fn scalar_motor_dual_geometric_product(self_126: Scalar, other_118: MotorDual) -> MotorDual {
    var self_127: Scalar;
    var other_119: MotorDual;

    self_127 = self_126;
    other_119 = other_118;
    let _e4: Scalar = self_127;
    let _e7: MotorDual = other_119;
    return MotorDual((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_motor_dual_regressive_product(self_128: Scalar, other_120: MotorDual) -> Scalar {
    var self_129: Scalar;
    var other_121: MotorDual;

    self_129 = self_128;
    other_121 = other_120;
    let _e4: Scalar = self_129;
    let _e6: MotorDual = other_121;
    return Scalar((_e4.g0_ * _e6.g0_.x));
}

fn scalar_motor_dual_outer_product(self_130: Scalar, other_122: MotorDual) -> MotorDual {
    var self_131: Scalar;
    var other_123: MotorDual;

    self_131 = self_130;
    other_123 = other_122;
    let _e4: Scalar = self_131;
    let _e7: MotorDual = other_123;
    return MotorDual((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_motor_dual_inner_product(self_132: Scalar, other_124: MotorDual) -> MotorDual {
    var self_133: Scalar;
    var other_125: MotorDual;

    self_133 = self_132;
    other_125 = other_124;
    let _e4: Scalar = self_133;
    let _e7: MotorDual = other_125;
    return MotorDual((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_motor_dual_left_contraction(self_134: Scalar, other_126: MotorDual) -> MotorDual {
    var self_135: Scalar;
    var other_127: MotorDual;

    self_135 = self_134;
    other_127 = other_126;
    let _e4: Scalar = self_135;
    let _e7: MotorDual = other_127;
    return MotorDual((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_squared_magnitude(self_136: Scalar) -> Scalar {
    var self_137: Scalar;

    self_137 = self_136;
    let _e2: Scalar = self_137;
    let _e3: Scalar = self_137;
    let _e4: Scalar = scalar_reversal(_e3);
    let _e5: Scalar = scalar_scalar_scalar_product(_e2, _e4);
    return _e5;
}

fn scalar_magnitude(self_138: Scalar) -> Scalar {
    var self_139: Scalar;

    self_139 = self_138;
    let _e2: Scalar = self_139;
    let _e3: Scalar = scalar_squared_magnitude(_e2);
    return Scalar(sqrt(_e3.g0_));
}

fn scalar_scale(self_140: Scalar, other_128: f32) -> Scalar {
    var self_141: Scalar;
    var other_129: f32;

    self_141 = self_140;
    other_129 = other_128;
    let _e4: Scalar = self_141;
    let _e5: f32 = other_129;
    let _e7: Scalar = scalar_scalar_geometric_product(_e4, Scalar(_e5));
    return _e7;
}

fn scalar_signum(self_142: Scalar) -> Scalar {
    var self_143: Scalar;

    self_143 = self_142;
    let _e2: Scalar = self_143;
    let _e3: Scalar = self_143;
    let _e4: Scalar = scalar_magnitude(_e3);
    let _e9: Scalar = scalar_scalar_geometric_product(_e2, Scalar((1.0 / _e4.g0_)));
    return _e9;
}

fn scalar_inverse(self_144: Scalar) -> Scalar {
    var self_145: Scalar;

    self_145 = self_144;
    let _e2: Scalar = self_145;
    let _e3: Scalar = scalar_reversal(_e2);
    let _e4: Scalar = self_145;
    let _e5: Scalar = scalar_squared_magnitude(_e4);
    let _e10: Scalar = scalar_scalar_geometric_product(_e3, Scalar((1.0 / _e5.g0_)));
    return _e10;
}

fn multi_vector_zero() -> MultiVector {
    return MultiVector(vec4<f32>(0.0), vec4<f32>(0.0));
}

fn multi_vector_one() -> MultiVector {
    return MultiVector(vec4<f32>(1.0, 0.0, 0.0, 0.0), vec4<f32>(0.0));
}

fn multi_vector_neg(self_146: MultiVector) -> MultiVector {
    var self_147: MultiVector;

    self_147 = self_146;
    let _e2: MultiVector = self_147;
    let _e8: MultiVector = self_147;
    return MultiVector((_e2.g0_ * vec4<f32>(-(1.0))), (_e8.g1_ * vec4<f32>(-(1.0))));
}

fn multi_vector_automorphism(self_148: MultiVector) -> MultiVector {
    var self_149: MultiVector;

    self_149 = self_148;
    let _e2: MultiVector = self_149;
    let _e12: MultiVector = self_149;
    return MultiVector((_e2.g0_ * vec4<f32>(1.0, 1.0, -(1.0), -(1.0))), (_e12.g1_ * vec4<f32>(-(1.0), -(1.0), 1.0, 1.0)));
}

fn multi_vector_reversal(self_150: MultiVector) -> MultiVector {
    var self_151: MultiVector;

    self_151 = self_150;
    let _e2: MultiVector = self_151;
    let _e11: MultiVector = self_151;
    return MultiVector((_e2.g0_ * vec4<f32>(1.0, -(1.0), 1.0, 1.0)), (_e11.g1_ * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))));
}

fn multi_vector_conjugation(self_152: MultiVector) -> MultiVector {
    var self_153: MultiVector;

    self_153 = self_152;
    let _e2: MultiVector = self_153;
    let _e13: MultiVector = self_153;
    return MultiVector((_e2.g0_ * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))), (_e13.g1_ * vec4<f32>(-(1.0), 1.0, -(1.0), -(1.0))));
}

fn multi_vector_dual(self_154: MultiVector) -> MultiVector {
    var self_155: MultiVector;

    self_155 = self_154;
    let _e2: MultiVector = self_155;
    let _e5: MultiVector = self_155;
    return MultiVector(_e2.g1_.yxwz, _e5.g0_.yxwz);
}

fn multi_vector_scalar_into(self_156: MultiVector) -> Scalar {
    var self_157: MultiVector;

    self_157 = self_156;
    let _e2: MultiVector = self_157;
    return Scalar(_e2.g0_.x);
}

fn multi_vector_scalar_add(self_158: MultiVector, other_130: Scalar) -> MultiVector {
    var self_159: MultiVector;
    var other_131: Scalar;

    self_159 = self_158;
    other_131 = other_130;
    let _e4: MultiVector = self_159;
    let _e6: Scalar = other_131;
    let _e16: MultiVector = self_159;
    return MultiVector((_e4.g0_ + (vec4<f32>(_e6.g0_) * vec4<f32>(1.0, 0.0, 0.0, 0.0))), _e16.g1_);
}

fn multi_vector_scalar_sub(self_160: MultiVector, other_132: Scalar) -> MultiVector {
    var self_161: MultiVector;
    var other_133: Scalar;

    self_161 = self_160;
    other_133 = other_132;
    let _e4: MultiVector = self_161;
    let _e6: Scalar = other_133;
    let _e16: MultiVector = self_161;
    return MultiVector((_e4.g0_ - (vec4<f32>(_e6.g0_) * vec4<f32>(1.0, 0.0, 0.0, 0.0))), _e16.g1_);
}

fn multi_vector_scalar_geometric_product(self_162: MultiVector, other_134: Scalar) -> MultiVector {
    var self_163: MultiVector;
    var other_135: Scalar;

    self_163 = self_162;
    other_135 = other_134;
    let _e4: MultiVector = self_163;
    let _e6: Scalar = other_135;
    let _e10: MultiVector = self_163;
    let _e12: Scalar = other_135;
    return MultiVector((_e4.g0_ * vec4<f32>(_e6.g0_)), (_e10.g1_ * vec4<f32>(_e12.g0_)));
}

fn multi_vector_scalar_regressive_product(self_164: MultiVector, other_136: Scalar) -> Scalar {
    var self_165: MultiVector;
    var other_137: Scalar;

    self_165 = self_164;
    other_137 = other_136;
    let _e4: MultiVector = self_165;
    let _e7: Scalar = other_137;
    return Scalar((_e4.g1_.y * _e7.g0_));
}

fn multi_vector_scalar_outer_product(self_166: MultiVector, other_138: Scalar) -> MultiVector {
    var self_167: MultiVector;
    var other_139: Scalar;

    self_167 = self_166;
    other_139 = other_138;
    let _e4: MultiVector = self_167;
    let _e6: Scalar = other_139;
    let _e10: MultiVector = self_167;
    let _e12: Scalar = other_139;
    return MultiVector((_e4.g0_ * vec4<f32>(_e6.g0_)), (_e10.g1_ * vec4<f32>(_e12.g0_)));
}

fn multi_vector_scalar_inner_product(self_168: MultiVector, other_140: Scalar) -> MultiVector {
    var self_169: MultiVector;
    var other_141: Scalar;

    self_169 = self_168;
    other_141 = other_140;
    let _e4: MultiVector = self_169;
    let _e6: Scalar = other_141;
    let _e10: MultiVector = self_169;
    let _e12: Scalar = other_141;
    return MultiVector((_e4.g0_ * vec4<f32>(_e6.g0_)), (_e10.g1_ * vec4<f32>(_e12.g0_)));
}

fn multi_vector_scalar_left_contraction(self_170: MultiVector, other_142: Scalar) -> Scalar {
    var self_171: MultiVector;
    var other_143: Scalar;

    self_171 = self_170;
    other_143 = other_142;
    let _e4: MultiVector = self_171;
    let _e7: Scalar = other_143;
    return Scalar((_e4.g0_.x * _e7.g0_));
}

fn multi_vector_scalar_right_contraction(self_172: MultiVector, other_144: Scalar) -> MultiVector {
    var self_173: MultiVector;
    var other_145: Scalar;

    self_173 = self_172;
    other_145 = other_144;
    let _e4: MultiVector = self_173;
    let _e6: Scalar = other_145;
    let _e10: MultiVector = self_173;
    let _e12: Scalar = other_145;
    return MultiVector((_e4.g0_ * vec4<f32>(_e6.g0_)), (_e10.g1_ * vec4<f32>(_e12.g0_)));
}

fn multi_vector_scalar_scalar_product(self_174: MultiVector, other_146: Scalar) -> Scalar {
    var self_175: MultiVector;
    var other_147: Scalar;

    self_175 = self_174;
    other_147 = other_146;
    let _e4: MultiVector = self_175;
    let _e7: Scalar = other_147;
    return Scalar((_e4.g0_.x * _e7.g0_));
}

fn multi_vector_multi_vector_add(self_176: MultiVector, other_148: MultiVector) -> MultiVector {
    var self_177: MultiVector;
    var other_149: MultiVector;

    self_177 = self_176;
    other_149 = other_148;
    let _e4: MultiVector = self_177;
    let _e6: MultiVector = other_149;
    let _e9: MultiVector = self_177;
    let _e11: MultiVector = other_149;
    return MultiVector((_e4.g0_ + _e6.g0_), (_e9.g1_ + _e11.g1_));
}

fn multi_vector_multi_vector_sub(self_178: MultiVector, other_150: MultiVector) -> MultiVector {
    var self_179: MultiVector;
    var other_151: MultiVector;

    self_179 = self_178;
    other_151 = other_150;
    let _e4: MultiVector = self_179;
    let _e6: MultiVector = other_151;
    let _e9: MultiVector = self_179;
    let _e11: MultiVector = other_151;
    return MultiVector((_e4.g0_ - _e6.g0_), (_e9.g1_ - _e11.g1_));
}

fn multi_vector_multi_vector_mul(self_180: MultiVector, other_152: MultiVector) -> MultiVector {
    var self_181: MultiVector;
    var other_153: MultiVector;

    self_181 = self_180;
    other_153 = other_152;
    let _e4: MultiVector = self_181;
    let _e6: MultiVector = other_153;
    let _e9: MultiVector = self_181;
    let _e11: MultiVector = other_153;
    return MultiVector((_e4.g0_ * _e6.g0_), (_e9.g1_ * _e11.g1_));
}

fn multi_vector_multi_vector_div(self_182: MultiVector, other_154: MultiVector) -> MultiVector {
    var self_183: MultiVector;
    var other_155: MultiVector;

    self_183 = self_182;
    other_155 = other_154;
    let _e4: MultiVector = self_183;
    let _e7: MultiVector = self_183;
    let _e10: MultiVector = self_183;
    let _e13: MultiVector = self_183;
    let _e23: MultiVector = other_155;
    let _e26: MultiVector = other_155;
    let _e29: MultiVector = other_155;
    let _e32: MultiVector = other_155;
    let _e43: MultiVector = self_183;
    let _e46: MultiVector = self_183;
    let _e49: MultiVector = self_183;
    let _e52: MultiVector = self_183;
    let _e62: MultiVector = other_155;
    let _e65: MultiVector = other_155;
    let _e68: MultiVector = other_155;
    let _e71: MultiVector = other_155;
    return MultiVector((((vec4<f32>(_e4.g0_.x, _e7.g0_.y, _e10.g0_.z, _e13.g0_.w) * vec4<f32>(1.0, 1.0, 1.0, 1.0)) / vec4<f32>(_e23.g0_.x, _e26.g0_.y, _e29.g0_.z, _e32.g0_.w)) * vec4<f32>(1.0, 1.0, 1.0, 1.0)), (((vec4<f32>(_e43.g1_.x, _e46.g1_.y, _e49.g1_.z, _e52.g1_.w) * vec4<f32>(1.0, 1.0, 1.0, 1.0)) / vec4<f32>(_e62.g1_.x, _e65.g1_.y, _e68.g1_.z, _e71.g1_.w)) * vec4<f32>(1.0, 1.0, 1.0, 1.0)));
}

fn multi_vector_multi_vector_geometric_product(self_184: MultiVector, other_156: MultiVector) -> MultiVector {
    var self_185: MultiVector;
    var other_157: MultiVector;

    self_185 = self_184;
    other_157 = other_156;
    let _e4: MultiVector = self_185;
    let _e8: MultiVector = other_157;
    let _e11: MultiVector = self_185;
    let _e15: MultiVector = other_157;
    let _e28: MultiVector = self_185;
    let _e32: MultiVector = other_157;
    let _e37: MultiVector = self_185;
    let _e41: MultiVector = other_157;
    let _e54: MultiVector = self_185;
    let _e58: MultiVector = other_157;
    let _e69: MultiVector = self_185;
    let _e73: MultiVector = other_157;
    let _e87: MultiVector = self_185;
    let _e91: MultiVector = other_157;
    let _e105: MultiVector = self_185;
    let _e109: MultiVector = other_157;
    let _e123: MultiVector = self_185;
    let _e127: MultiVector = other_157;
    let _e130: MultiVector = self_185;
    let _e134: MultiVector = other_157;
    let _e147: MultiVector = self_185;
    let _e151: MultiVector = other_157;
    let _e164: MultiVector = self_185;
    let _e168: MultiVector = other_157;
    let _e173: MultiVector = self_185;
    let _e177: MultiVector = other_157;
    let _e188: MultiVector = self_185;
    let _e192: MultiVector = other_157;
    let _e204: MultiVector = self_185;
    let _e208: MultiVector = other_157;
    let _e220: MultiVector = self_185;
    let _e224: MultiVector = other_157;
    return MultiVector(((((((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.y) * _e15.g0_.yxwz) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + (vec4<f32>(_e28.g0_.z) * _e32.g0_.zwxy)) + ((vec4<f32>(_e37.g0_.w) * _e41.g0_.wzyx) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e54.g1_.x) * _e58.g1_) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e69.g1_.y) * _e73.g1_.yxwz) * vec4<f32>(-(1.0), 1.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e87.g1_.z) * _e91.g1_.zwxy) * vec4<f32>(-(1.0), 1.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e105.g1_.w) * _e109.g1_.wzyx) * vec4<f32>(-(1.0), -(1.0), -(1.0), 1.0))), ((((((((vec4<f32>(_e123.g0_.x) * _e127.g1_) + ((vec4<f32>(_e130.g0_.y) * _e134.g1_.yxwz) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))) + ((vec4<f32>(_e147.g0_.z) * _e151.g1_.zwxy) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))) + (vec4<f32>(_e164.g0_.w) * _e168.g1_.wzyx)) + ((vec4<f32>(_e173.g1_.x) * _e177.g0_) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e188.g1_.y) * _e192.g0_.yxwz) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0))) + ((vec4<f32>(_e204.g1_.z) * _e208.g0_.zwxy) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e220.g1_.w) * _e224.g0_.wzyx) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0))));
}

fn multi_vector_multi_vector_regressive_product(self_186: MultiVector, other_158: MultiVector) -> MultiVector {
    var self_187: MultiVector;
    var other_159: MultiVector;

    self_187 = self_186;
    other_159 = other_158;
    let _e4: MultiVector = self_187;
    let _e8: MultiVector = other_159;
    let _e18: MultiVector = self_187;
    let _e22: MultiVector = other_159;
    let _e33: MultiVector = self_187;
    let _e37: MultiVector = other_159;
    let _e48: MultiVector = self_187;
    let _e52: MultiVector = other_159;
    let _e64: MultiVector = self_187;
    let _e68: MultiVector = other_159;
    let _e72: MultiVector = self_187;
    let _e76: MultiVector = other_159;
    let _e87: MultiVector = self_187;
    let _e91: MultiVector = other_159;
    let _e103: MultiVector = self_187;
    let _e107: MultiVector = other_159;
    let _e118: MultiVector = self_187;
    let _e122: MultiVector = other_159;
    let _e125: MultiVector = self_187;
    let _e129: MultiVector = other_159;
    let _e141: MultiVector = self_187;
    let _e145: MultiVector = other_159;
    let _e156: MultiVector = self_187;
    let _e160: MultiVector = other_159;
    return MultiVector((((((((((vec4<f32>(_e4.g0_.y) * _e8.g1_) * vec4<f32>(1.0, 1.0, -(1.0), 1.0)) + ((vec4<f32>(_e18.g0_.z) * _e22.g1_.wwyw) * vec4<f32>(1.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e33.g0_.w) * _e37.g1_.zzzy) * vec4<f32>(1.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e48.g1_.x) * vec4<f32>(_e52.g0_.y)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + (vec4<f32>(_e64.g1_.y) * _e68.g0_)) + ((vec4<f32>(_e72.g1_.z) * _e76.g0_.wwyw) * vec4<f32>(1.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e87.g1_.w) * _e91.g0_.zzzy) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e103.g0_.x) * _e107.g1_.yxxx) * vec4<f32>(1.0, 0.0, 0.0, 0.0))), ((((vec4<f32>(_e118.g1_.y) * _e122.g1_) + ((vec4<f32>(_e125.g1_.z) * _e129.g1_.wwyw) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0))) + ((vec4<f32>(_e141.g1_.w) * _e145.g1_.zzzy) * vec4<f32>(1.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e156.g1_.x) * _e160.g1_.yxxx) * vec4<f32>(1.0, 0.0, 0.0, 0.0))));
}

fn multi_vector_multi_vector_outer_product(self_188: MultiVector, other_160: MultiVector) -> MultiVector {
    var self_189: MultiVector;
    var other_161: MultiVector;

    self_189 = self_188;
    other_161 = other_160;
    let _e4: MultiVector = self_189;
    let _e8: MultiVector = other_161;
    let _e11: MultiVector = self_189;
    let _e15: MultiVector = other_161;
    let _e26: MultiVector = self_189;
    let _e30: MultiVector = other_161;
    let _e42: MultiVector = self_189;
    let _e45: MultiVector = other_161;
    let _e57: MultiVector = self_189;
    let _e61: MultiVector = other_161;
    let _e64: MultiVector = self_189;
    let _e68: MultiVector = other_161;
    let _e80: MultiVector = self_189;
    let _e84: MultiVector = other_161;
    let _e95: MultiVector = self_189;
    let _e99: MultiVector = other_161;
    let _e110: MultiVector = self_189;
    let _e114: MultiVector = other_161;
    let _e126: MultiVector = self_189;
    let _e130: MultiVector = other_161;
    let _e141: MultiVector = self_189;
    let _e145: MultiVector = other_161;
    let _e156: MultiVector = self_189;
    let _e159: MultiVector = other_161;
    return MultiVector(((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.z) * _e15.g0_.wwxw) * vec4<f32>(0.0, 1.0, 1.0, 0.0))) + ((vec4<f32>(_e26.g0_.w) * _e30.g0_.zzzx) * vec4<f32>(0.0, -(1.0), 0.0, 1.0))) + ((_e42.g0_.xyxx * vec4<f32>(_e45.g0_.x)) * vec4<f32>(0.0, 1.0, 0.0, 0.0))), ((((((((vec4<f32>(_e57.g0_.x) * _e61.g1_) + ((vec4<f32>(_e64.g0_.z) * _e68.g1_.wwxw) * vec4<f32>(0.0, 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e80.g0_.w) * _e84.g1_.zzzx) * vec4<f32>(0.0, 1.0, 0.0, 1.0))) + ((vec4<f32>(_e95.g1_.x) * _e99.g0_) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e110.g1_.y) * vec4<f32>(_e114.g0_.x)) * vec4<f32>(0.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e126.g1_.z) * _e130.g0_.wwxw) * vec4<f32>(0.0, 1.0, 1.0, 0.0))) + ((vec4<f32>(_e141.g1_.w) * _e145.g0_.zzzx) * vec4<f32>(0.0, 1.0, 0.0, 1.0))) + ((_e156.g0_.xyxx * vec4<f32>(_e159.g1_.x)) * vec4<f32>(0.0, 1.0, 0.0, 0.0))));
}

fn multi_vector_multi_vector_inner_product(self_190: MultiVector, other_162: MultiVector) -> MultiVector {
    var self_191: MultiVector;
    var other_163: MultiVector;

    self_191 = self_190;
    other_163 = other_162;
    let _e4: MultiVector = self_191;
    let _e8: MultiVector = other_163;
    let _e11: MultiVector = self_191;
    let _e15: MultiVector = other_163;
    let _e28: MultiVector = self_191;
    let _e32: MultiVector = other_163;
    let _e44: MultiVector = self_191;
    let _e48: MultiVector = other_163;
    let _e59: MultiVector = self_191;
    let _e63: MultiVector = other_163;
    let _e77: MultiVector = self_191;
    let _e81: MultiVector = other_163;
    let _e95: MultiVector = self_191;
    let _e99: MultiVector = other_163;
    let _e112: MultiVector = self_191;
    let _e115: MultiVector = other_163;
    let _e126: MultiVector = self_191;
    let _e130: MultiVector = other_163;
    let _e133: MultiVector = self_191;
    let _e137: MultiVector = other_163;
    let _e149: MultiVector = self_191;
    let _e153: MultiVector = other_163;
    let _e164: MultiVector = self_191;
    let _e168: MultiVector = other_163;
    let _e180: MultiVector = self_191;
    let _e184: MultiVector = other_163;
    let _e196: MultiVector = self_191;
    let _e200: MultiVector = other_163;
    let _e211: MultiVector = self_191;
    let _e215: MultiVector = other_163;
    let _e227: MultiVector = self_191;
    let _e230: MultiVector = other_163;
    return MultiVector(((((((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.y) * _e15.g0_.yxwz) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e28.g0_.w) * _e32.g0_.wwyx) * vec4<f32>(1.0, 0.0, -(1.0), 1.0))) + ((vec4<f32>(_e44.g1_.x) * _e48.g1_) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e59.g1_.y) * _e63.g1_.yxwz) * vec4<f32>(-(1.0), 1.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e77.g1_.z) * _e81.g1_.zzxy) * vec4<f32>(-(1.0), 0.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e95.g1_.w) * _e99.g1_.wwyx) * vec4<f32>(-(1.0), 0.0, -(1.0), 1.0))) + ((_e112.g0_.zxzz * _e115.g0_.zxxy) * vec4<f32>(1.0, 0.0, 1.0, 1.0))), ((((((((vec4<f32>(_e126.g0_.x) * _e130.g1_) + ((vec4<f32>(_e133.g0_.z) * _e137.g1_.zzzy) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))) + ((vec4<f32>(_e149.g0_.w) * _e153.g1_.wwyw) * vec4<f32>(1.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e164.g1_.x) * vec4<f32>(_e168.g0_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e180.g1_.y) * _e184.g0_.yxwz) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0))) + ((vec4<f32>(_e196.g1_.z) * _e200.g0_.zzxz) * vec4<f32>(1.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e211.g1_.w) * _e215.g0_.wwwx) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))) + ((_e227.g0_.yxxx * _e230.g1_.yxxx) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))));
}

fn multi_vector_multi_vector_left_contraction(self_192: MultiVector, other_164: MultiVector) -> MultiVector {
    var self_193: MultiVector;
    var other_165: MultiVector;

    self_193 = self_192;
    other_165 = other_164;
    let _e4: MultiVector = self_193;
    let _e8: MultiVector = other_165;
    let _e11: MultiVector = self_193;
    let _e15: MultiVector = other_165;
    let _e26: MultiVector = self_193;
    let _e30: MultiVector = other_165;
    let _e42: MultiVector = self_193;
    let _e46: MultiVector = other_165;
    let _e57: MultiVector = self_193;
    let _e61: MultiVector = other_165;
    let _e74: MultiVector = self_193;
    let _e78: MultiVector = other_165;
    let _e91: MultiVector = self_193;
    let _e95: MultiVector = other_165;
    let _e108: MultiVector = self_193;
    let _e111: MultiVector = other_165;
    let _e123: MultiVector = self_193;
    let _e127: MultiVector = other_165;
    let _e130: MultiVector = self_193;
    let _e134: MultiVector = other_165;
    let _e146: MultiVector = self_193;
    let _e150: MultiVector = other_165;
    let _e161: MultiVector = self_193;
    let _e164: MultiVector = other_165;
    return MultiVector(((((((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.z) * _e15.g0_.zzzy) * vec4<f32>(1.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e26.g0_.w) * _e30.g0_.wwyw) * vec4<f32>(1.0, 0.0, -(1.0), 0.0))) + ((vec4<f32>(_e42.g1_.x) * _e46.g1_) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e57.g1_.y) * vec4<f32>(_e61.g1_.y)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((vec4<f32>(_e74.g1_.z) * _e78.g1_.zzzy) * vec4<f32>(-(1.0), 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e91.g1_.w) * _e95.g1_.wwyw) * vec4<f32>(-(1.0), 0.0, -(1.0), 0.0))) + ((_e108.g0_.yxxx * _e111.g0_.yxxx) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))), ((((vec4<f32>(_e123.g0_.x) * _e127.g1_) + ((vec4<f32>(_e130.g0_.z) * _e134.g1_.zzzy) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))) + ((vec4<f32>(_e146.g0_.w) * _e150.g1_.wwyw) * vec4<f32>(1.0, 0.0, 1.0, 0.0))) + ((_e161.g0_.yxxx * _e164.g1_.yxxx) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))));
}

fn multi_vector_multi_vector_right_contraction(self_194: MultiVector, other_166: MultiVector) -> MultiVector {
    var self_195: MultiVector;
    var other_167: MultiVector;

    self_195 = self_194;
    other_167 = other_166;
    let _e4: MultiVector = self_195;
    let _e8: MultiVector = other_167;
    let _e20: MultiVector = self_195;
    let _e24: MultiVector = other_167;
    let _e35: MultiVector = self_195;
    let _e39: MultiVector = other_167;
    let _e50: MultiVector = self_195;
    let _e54: MultiVector = other_167;
    let _e66: MultiVector = self_195;
    let _e70: MultiVector = other_167;
    let _e84: MultiVector = self_195;
    let _e88: MultiVector = other_167;
    let _e101: MultiVector = self_195;
    let _e105: MultiVector = other_167;
    let _e117: MultiVector = self_195;
    let _e121: MultiVector = other_167;
    let _e133: MultiVector = self_195;
    let _e137: MultiVector = other_167;
    let _e148: MultiVector = self_195;
    let _e152: MultiVector = other_167;
    let _e163: MultiVector = self_195;
    let _e167: MultiVector = other_167;
    let _e179: MultiVector = self_195;
    let _e183: MultiVector = other_167;
    return MultiVector((((((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.yxwz) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0))) + ((vec4<f32>(_e20.g0_.z) * _e24.g0_.zzxz) * vec4<f32>(1.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e35.g0_.w) * _e39.g0_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e50.g1_.x) * vec4<f32>(_e54.g1_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e66.g1_.y) * _e70.g1_.yxwz) * vec4<f32>(-(1.0), 1.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e84.g1_.z) * _e88.g1_.zzxz) * vec4<f32>(-(1.0), 0.0, -(1.0), 0.0))) + ((vec4<f32>(_e101.g1_.w) * _e105.g1_.wwwx) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))) + ((vec4<f32>(_e117.g0_.x) * vec4<f32>(_e121.g0_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))), (((((vec4<f32>(_e133.g1_.y) * _e137.g0_.yxwz) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0)) + ((vec4<f32>(_e148.g1_.z) * _e152.g0_.zzxz) * vec4<f32>(1.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e163.g1_.w) * _e167.g0_.wwwx) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))) + ((vec4<f32>(_e179.g1_.x) * vec4<f32>(_e183.g0_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))));
}

fn multi_vector_multi_vector_scalar_product(self_196: MultiVector, other_168: MultiVector) -> Scalar {
    var self_197: MultiVector;
    var other_169: MultiVector;

    self_197 = self_196;
    other_169 = other_168;
    let _e4: MultiVector = self_197;
    let _e7: MultiVector = other_169;
    let _e11: MultiVector = self_197;
    let _e14: MultiVector = other_169;
    let _e19: MultiVector = self_197;
    let _e22: MultiVector = other_169;
    let _e27: MultiVector = self_197;
    let _e30: MultiVector = other_169;
    let _e35: MultiVector = self_197;
    let _e38: MultiVector = other_169;
    let _e43: MultiVector = self_197;
    let _e46: MultiVector = other_169;
    let _e51: MultiVector = self_197;
    let _e54: MultiVector = other_169;
    let _e59: MultiVector = self_197;
    let _e62: MultiVector = other_169;
    return Scalar(((((((((_e4.g0_.x * _e7.g0_.x) - (_e11.g0_.y * _e14.g0_.y)) + (_e19.g0_.z * _e22.g0_.z)) + (_e27.g0_.w * _e30.g0_.w)) + (_e35.g1_.x * _e38.g1_.x)) - (_e43.g1_.y * _e46.g1_.y)) - (_e51.g1_.z * _e54.g1_.z)) - (_e59.g1_.w * _e62.g1_.w)));
}

fn multi_vector_rotor_into(self_198: MultiVector) -> Rotor {
    var self_199: MultiVector;

    self_199 = self_198;
    let _e2: MultiVector = self_199;
    let _e5: MultiVector = self_199;
    return Rotor(vec2<f32>(_e2.g0_.x, _e5.g0_.y));
}

fn multi_vector_rotor_add(self_200: MultiVector, other_170: Rotor) -> MultiVector {
    var self_201: MultiVector;
    var other_171: Rotor;

    self_201 = self_200;
    other_171 = other_170;
    let _e4: MultiVector = self_201;
    let _e6: Rotor = other_171;
    let _e9: Rotor = other_171;
    let _e12: Rotor = other_171;
    let _e15: Rotor = other_171;
    let _e26: MultiVector = self_201;
    return MultiVector((_e4.g0_ + (vec4<f32>(_e6.g0_.x, _e9.g0_.y, _e12.g0_.x, _e15.g0_.x) * vec4<f32>(1.0, 1.0, 0.0, 0.0))), _e26.g1_);
}

fn multi_vector_rotor_sub(self_202: MultiVector, other_172: Rotor) -> MultiVector {
    var self_203: MultiVector;
    var other_173: Rotor;

    self_203 = self_202;
    other_173 = other_172;
    let _e4: MultiVector = self_203;
    let _e6: Rotor = other_173;
    let _e9: Rotor = other_173;
    let _e12: Rotor = other_173;
    let _e15: Rotor = other_173;
    let _e26: MultiVector = self_203;
    return MultiVector((_e4.g0_ - (vec4<f32>(_e6.g0_.x, _e9.g0_.y, _e12.g0_.x, _e15.g0_.x) * vec4<f32>(1.0, 1.0, 0.0, 0.0))), _e26.g1_);
}

fn multi_vector_rotor_geometric_product(self_204: MultiVector, other_174: Rotor) -> MultiVector {
    var self_205: MultiVector;
    var other_175: Rotor;

    self_205 = self_204;
    other_175 = other_174;
    let _e4: MultiVector = self_205;
    let _e8: Rotor = other_175;
    let _e11: Rotor = other_175;
    let _e14: Rotor = other_175;
    let _e17: Rotor = other_175;
    let _e29: MultiVector = self_205;
    let _e33: Rotor = other_175;
    let _e36: Rotor = other_175;
    let _e39: Rotor = other_175;
    let _e42: Rotor = other_175;
    let _e55: MultiVector = self_205;
    let _e58: Rotor = other_175;
    let _e61: Rotor = other_175;
    let _e64: Rotor = other_175;
    let _e67: Rotor = other_175;
    let _e73: MultiVector = self_205;
    let _e77: Rotor = other_175;
    let _e80: Rotor = other_175;
    let _e83: Rotor = other_175;
    let _e86: Rotor = other_175;
    let _e98: MultiVector = self_205;
    let _e102: Rotor = other_175;
    let _e105: Rotor = other_175;
    let _e108: Rotor = other_175;
    let _e111: Rotor = other_175;
    let _e123: MultiVector = self_205;
    let _e126: Rotor = other_175;
    let _e129: Rotor = other_175;
    let _e132: Rotor = other_175;
    let _e135: Rotor = other_175;
    return MultiVector(((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y, _e11.g0_.x, _e14.g0_.y, _e17.g0_.y)) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0)) + ((vec4<f32>(_e29.g0_.w) * vec4<f32>(_e33.g0_.y, _e36.g0_.y, _e39.g0_.y, _e42.g0_.x)) * vec4<f32>(0.0, 0.0, -(1.0), 1.0))) + (_e55.g0_.xxzz * vec4<f32>(_e58.g0_.x, _e61.g0_.y, _e64.g0_.x, _e67.g0_.y))), ((((vec4<f32>(_e73.g1_.y) * vec4<f32>(_e77.g0_.y, _e80.g0_.x, _e83.g0_.y, _e86.g0_.y)) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0)) + ((vec4<f32>(_e98.g1_.w) * vec4<f32>(_e102.g0_.y, _e105.g0_.y, _e108.g0_.y, _e111.g0_.x)) * vec4<f32>(0.0, 0.0, 1.0, 1.0))) + ((_e123.g1_.xxzz * vec4<f32>(_e126.g0_.x, _e129.g0_.y, _e132.g0_.x, _e135.g0_.y)) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))));
}

fn multi_vector_rotor_outer_product(self_206: MultiVector, other_176: Rotor) -> MultiVector {
    var self_207: MultiVector;
    var other_177: Rotor;

    self_207 = self_206;
    other_177 = other_176;
    let _e4: MultiVector = self_207;
    let _e8: Rotor = other_177;
    let _e19: MultiVector = self_207;
    let _e22: Rotor = other_177;
    let _e25: Rotor = other_177;
    let _e28: Rotor = other_177;
    let _e31: Rotor = other_177;
    let _e37: MultiVector = self_207;
    let _e41: Rotor = other_177;
    let _e52: MultiVector = self_207;
    let _e55: Rotor = other_177;
    let _e58: Rotor = other_177;
    let _e61: Rotor = other_177;
    let _e64: Rotor = other_177;
    return MultiVector((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.x)) * vec4<f32>(0.0, 1.0, 0.0, 0.0)) + (_e19.g0_.xxzw * vec4<f32>(_e22.g0_.x, _e25.g0_.y, _e28.g0_.x, _e31.g0_.x))), (((vec4<f32>(_e37.g1_.y) * vec4<f32>(_e41.g0_.x)) * vec4<f32>(0.0, 1.0, 0.0, 0.0)) + (_e52.g1_.xxzw * vec4<f32>(_e55.g0_.x, _e58.g0_.y, _e61.g0_.x, _e64.g0_.x))));
}

fn multi_vector_rotor_inner_product(self_208: MultiVector, other_178: Rotor) -> MultiVector {
    var self_209: MultiVector;
    var other_179: Rotor;

    self_209 = self_208;
    other_179 = other_178;
    let _e4: MultiVector = self_209;
    let _e8: Rotor = other_179;
    let _e11: Rotor = other_179;
    let _e14: Rotor = other_179;
    let _e17: Rotor = other_179;
    let _e29: MultiVector = self_209;
    let _e33: Rotor = other_179;
    let _e36: Rotor = other_179;
    let _e39: Rotor = other_179;
    let _e42: Rotor = other_179;
    let _e55: MultiVector = self_209;
    let _e58: Rotor = other_179;
    let _e61: Rotor = other_179;
    let _e64: Rotor = other_179;
    let _e67: Rotor = other_179;
    let _e73: MultiVector = self_209;
    let _e77: Rotor = other_179;
    let _e80: Rotor = other_179;
    let _e83: Rotor = other_179;
    let _e86: Rotor = other_179;
    let _e98: MultiVector = self_209;
    let _e101: Rotor = other_179;
    return MultiVector(((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y, _e11.g0_.x, _e14.g0_.y, _e17.g0_.y)) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0)) + ((vec4<f32>(_e29.g0_.w) * vec4<f32>(_e33.g0_.y, _e36.g0_.y, _e39.g0_.y, _e42.g0_.x)) * vec4<f32>(0.0, 0.0, -(1.0), 1.0))) + (_e55.g0_.xxzz * vec4<f32>(_e58.g0_.x, _e61.g0_.y, _e64.g0_.x, _e67.g0_.y))), (((vec4<f32>(_e73.g1_.y) * vec4<f32>(_e77.g0_.y, _e80.g0_.x, _e83.g0_.y, _e86.g0_.y)) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0)) + ((_e98.g1_.xxzw * vec4<f32>(_e101.g0_.x)) * vec4<f32>(1.0, 0.0, 1.0, 1.0))));
}

fn multi_vector_rotor_right_contraction(self_210: MultiVector, other_180: Rotor) -> MultiVector {
    var self_211: MultiVector;
    var other_181: Rotor;

    self_211 = self_210;
    other_181 = other_180;
    let _e4: MultiVector = self_211;
    let _e8: Rotor = other_181;
    let _e11: Rotor = other_181;
    let _e14: Rotor = other_181;
    let _e17: Rotor = other_181;
    let _e29: MultiVector = self_211;
    let _e32: Rotor = other_181;
    let _e44: MultiVector = self_211;
    let _e48: Rotor = other_181;
    let _e51: Rotor = other_181;
    let _e54: Rotor = other_181;
    let _e57: Rotor = other_181;
    let _e69: MultiVector = self_211;
    let _e72: Rotor = other_181;
    return MultiVector((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y, _e11.g0_.x, _e14.g0_.y, _e17.g0_.y)) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0)) + ((_e29.g0_.xxzw * vec4<f32>(_e32.g0_.x)) * vec4<f32>(1.0, 0.0, 1.0, 1.0))), (((vec4<f32>(_e44.g1_.y) * vec4<f32>(_e48.g0_.y, _e51.g0_.x, _e54.g0_.y, _e57.g0_.y)) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0)) + ((_e69.g1_.xxzw * vec4<f32>(_e72.g0_.x)) * vec4<f32>(1.0, 0.0, 1.0, 1.0))));
}

fn multi_vector_rotor_scalar_product(self_212: MultiVector, other_182: Rotor) -> Scalar {
    var self_213: MultiVector;
    var other_183: Rotor;

    self_213 = self_212;
    other_183 = other_182;
    let _e4: MultiVector = self_213;
    let _e7: Rotor = other_183;
    let _e11: MultiVector = self_213;
    let _e14: Rotor = other_183;
    return Scalar(((_e4.g0_.x * _e7.g0_.x) - (_e11.g0_.y * _e14.g0_.y)));
}

fn multi_vector_point_into(self_214: MultiVector) -> Point {
    var self_215: MultiVector;

    self_215 = self_214;
    let _e2: MultiVector = self_215;
    let _e5: MultiVector = self_215;
    let _e8: MultiVector = self_215;
    return Point(vec3<f32>(_e2.g0_.y, _e5.g1_.z, _e8.g1_.w));
}

fn multi_vector_point_add(self_216: MultiVector, other_184: Point) -> MultiVector {
    var self_217: MultiVector;
    var other_185: Point;

    self_217 = self_216;
    other_185 = other_184;
    let _e4: MultiVector = self_217;
    let _e6: Point = other_185;
    let _e17: MultiVector = self_217;
    let _e19: Point = other_185;
    let _e22: Point = other_185;
    let _e25: Point = other_185;
    let _e28: Point = other_185;
    return MultiVector((_e4.g0_ + (vec4<f32>(_e6.g0_.x) * vec4<f32>(0.0, 1.0, 0.0, 0.0))), (_e17.g1_ + (vec4<f32>(_e19.g0_.x, _e22.g0_.x, _e25.g0_.y, _e28.g0_.z) * vec4<f32>(0.0, 0.0, 1.0, 1.0))));
}

fn multi_vector_point_sub(self_218: MultiVector, other_186: Point) -> MultiVector {
    var self_219: MultiVector;
    var other_187: Point;

    self_219 = self_218;
    other_187 = other_186;
    let _e4: MultiVector = self_219;
    let _e6: Point = other_187;
    let _e17: MultiVector = self_219;
    let _e19: Point = other_187;
    let _e22: Point = other_187;
    let _e25: Point = other_187;
    let _e28: Point = other_187;
    return MultiVector((_e4.g0_ - (vec4<f32>(_e6.g0_.x) * vec4<f32>(0.0, 1.0, 0.0, 0.0))), (_e17.g1_ - (vec4<f32>(_e19.g0_.x, _e22.g0_.x, _e25.g0_.y, _e28.g0_.z) * vec4<f32>(0.0, 0.0, 1.0, 1.0))));
}

fn multi_vector_point_geometric_product(self_220: MultiVector, other_188: Point) -> MultiVector {
    var self_221: MultiVector;
    var other_189: Point;

    self_221 = self_220;
    other_189 = other_188;
    let _e4: MultiVector = self_221;
    let _e8: Point = other_189;
    let _e11: Point = other_189;
    let _e14: Point = other_189;
    let _e17: Point = other_189;
    let _e29: MultiVector = self_221;
    let _e33: Point = other_189;
    let _e36: Point = other_189;
    let _e39: Point = other_189;
    let _e42: Point = other_189;
    let _e56: MultiVector = self_221;
    let _e60: Point = other_189;
    let _e63: Point = other_189;
    let _e66: Point = other_189;
    let _e69: Point = other_189;
    let _e82: MultiVector = self_221;
    let _e86: Point = other_189;
    let _e89: Point = other_189;
    let _e92: Point = other_189;
    let _e95: Point = other_189;
    let _e109: MultiVector = self_221;
    let _e112: Point = other_189;
    let _e126: MultiVector = self_221;
    let _e130: Point = other_189;
    let _e133: Point = other_189;
    let _e136: Point = other_189;
    let _e139: Point = other_189;
    let _e151: MultiVector = self_221;
    let _e155: Point = other_189;
    let _e158: Point = other_189;
    let _e161: Point = other_189;
    let _e164: Point = other_189;
    let _e176: MultiVector = self_221;
    let _e180: Point = other_189;
    let _e192: MultiVector = self_221;
    let _e196: Point = other_189;
    let _e209: MultiVector = self_221;
    let _e213: Point = other_189;
    let _e226: MultiVector = self_221;
    let _e230: Point = other_189;
    let _e242: MultiVector = self_221;
    let _e245: Point = other_189;
    let _e248: Point = other_189;
    let _e251: Point = other_189;
    let _e254: Point = other_189;
    return MultiVector(((((((vec4<f32>(_e4.g1_.x) * vec4<f32>(_e8.g0_.y, _e11.g0_.y, _e14.g0_.y, _e17.g0_.z)) * vec4<f32>(0.0, 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e29.g1_.y) * vec4<f32>(_e33.g0_.z, _e36.g0_.z, _e39.g0_.z, _e42.g0_.y)) * vec4<f32>(0.0, 0.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e56.g1_.z) * vec4<f32>(_e60.g0_.y, _e63.g0_.z, _e66.g0_.y, _e69.g0_.y)) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))) + ((vec4<f32>(_e82.g1_.w) * vec4<f32>(_e86.g0_.z, _e89.g0_.y, _e92.g0_.z, _e95.g0_.z)) * vec4<f32>(-(1.0), -(1.0), 0.0, 0.0))) + ((_e109.g0_.yxwz * vec4<f32>(_e112.g0_.x)) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))), ((((((((vec4<f32>(_e126.g0_.y) * vec4<f32>(_e130.g0_.z, _e133.g0_.z, _e136.g0_.z, _e139.g0_.y)) * vec4<f32>(0.0, 0.0, -(1.0), 1.0)) + ((vec4<f32>(_e151.g0_.w) * vec4<f32>(_e155.g0_.z, _e158.g0_.y, _e161.g0_.z, _e164.g0_.z)) * vec4<f32>(1.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e176.g1_.x) * vec4<f32>(_e180.g0_.x)) * vec4<f32>(0.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e192.g1_.y) * vec4<f32>(_e196.g0_.x)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((vec4<f32>(_e209.g1_.z) * vec4<f32>(_e213.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e226.g1_.w) * vec4<f32>(_e230.g0_.x)) * vec4<f32>(0.0, 0.0, 1.0, 0.0))) + ((_e242.g0_.zzxx * vec4<f32>(_e245.g0_.y, _e248.g0_.z, _e251.g0_.y, _e254.g0_.z)) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0))));
}

fn multi_vector_point_scalar_product(self_222: MultiVector, other_190: Point) -> Scalar {
    var self_223: MultiVector;
    var other_191: Point;

    self_223 = self_222;
    other_191 = other_190;
    let _e5: MultiVector = self_223;
    let _e8: Point = other_191;
    let _e13: MultiVector = self_223;
    let _e16: Point = other_191;
    let _e21: MultiVector = self_223;
    let _e24: Point = other_191;
    return Scalar((((0.0 - (_e5.g0_.y * _e8.g0_.x)) - (_e13.g1_.z * _e16.g0_.y)) - (_e21.g1_.w * _e24.g0_.z)));
}

fn multi_vector_ideal_point_into(self_224: MultiVector) -> IdealPoint {
    var self_225: MultiVector;

    self_225 = self_224;
    let _e2: MultiVector = self_225;
    let _e5: MultiVector = self_225;
    return IdealPoint(vec2<f32>(_e2.g1_.z, _e5.g1_.w));
}

fn multi_vector_ideal_point_add(self_226: MultiVector, other_192: IdealPoint) -> MultiVector {
    var self_227: MultiVector;
    var other_193: IdealPoint;

    self_227 = self_226;
    other_193 = other_192;
    let _e4: MultiVector = self_227;
    let _e6: MultiVector = self_227;
    let _e8: IdealPoint = other_193;
    let _e11: IdealPoint = other_193;
    let _e14: IdealPoint = other_193;
    let _e17: IdealPoint = other_193;
    return MultiVector(_e4.g0_, (_e6.g1_ + (vec4<f32>(_e8.g0_.x, _e11.g0_.x, _e14.g0_.x, _e17.g0_.y) * vec4<f32>(0.0, 0.0, 1.0, 1.0))));
}

fn multi_vector_ideal_point_sub(self_228: MultiVector, other_194: IdealPoint) -> MultiVector {
    var self_229: MultiVector;
    var other_195: IdealPoint;

    self_229 = self_228;
    other_195 = other_194;
    let _e4: MultiVector = self_229;
    let _e6: MultiVector = self_229;
    let _e8: IdealPoint = other_195;
    let _e11: IdealPoint = other_195;
    let _e14: IdealPoint = other_195;
    let _e17: IdealPoint = other_195;
    return MultiVector(_e4.g0_, (_e6.g1_ - (vec4<f32>(_e8.g0_.x, _e11.g0_.x, _e14.g0_.x, _e17.g0_.y) * vec4<f32>(0.0, 0.0, 1.0, 1.0))));
}

fn multi_vector_ideal_point_geometric_product(self_230: MultiVector, other_196: IdealPoint) -> MultiVector {
    var self_231: MultiVector;
    var other_197: IdealPoint;

    self_231 = self_230;
    other_197 = other_196;
    let _e4: MultiVector = self_231;
    let _e8: IdealPoint = other_197;
    let _e11: IdealPoint = other_197;
    let _e14: IdealPoint = other_197;
    let _e17: IdealPoint = other_197;
    let _e30: MultiVector = self_231;
    let _e34: IdealPoint = other_197;
    let _e37: IdealPoint = other_197;
    let _e40: IdealPoint = other_197;
    let _e43: IdealPoint = other_197;
    let _e57: MultiVector = self_231;
    let _e60: IdealPoint = other_197;
    let _e63: IdealPoint = other_197;
    let _e66: IdealPoint = other_197;
    let _e69: IdealPoint = other_197;
    let _e83: MultiVector = self_231;
    let _e87: IdealPoint = other_197;
    let _e90: IdealPoint = other_197;
    let _e93: IdealPoint = other_197;
    let _e96: IdealPoint = other_197;
    let _e108: MultiVector = self_231;
    let _e112: IdealPoint = other_197;
    let _e115: IdealPoint = other_197;
    let _e118: IdealPoint = other_197;
    let _e121: IdealPoint = other_197;
    let _e133: MultiVector = self_231;
    let _e136: IdealPoint = other_197;
    let _e139: IdealPoint = other_197;
    let _e142: IdealPoint = other_197;
    let _e145: IdealPoint = other_197;
    return MultiVector(((((vec4<f32>(_e4.g1_.y) * vec4<f32>(_e8.g0_.y, _e11.g0_.y, _e14.g0_.y, _e17.g0_.x)) * vec4<f32>(0.0, 0.0, -(1.0), -(1.0))) + ((vec4<f32>(_e30.g1_.w) * vec4<f32>(_e34.g0_.y, _e37.g0_.x, _e40.g0_.y, _e43.g0_.y)) * vec4<f32>(-(1.0), -(1.0), 0.0, 0.0))) + ((_e57.g1_.zzxx * vec4<f32>(_e60.g0_.x, _e63.g0_.y, _e66.g0_.x, _e69.g0_.y)) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))), ((((vec4<f32>(_e83.g0_.y) * vec4<f32>(_e87.g0_.y, _e90.g0_.y, _e93.g0_.y, _e96.g0_.x)) * vec4<f32>(0.0, 0.0, -(1.0), 1.0)) + ((vec4<f32>(_e108.g0_.w) * vec4<f32>(_e112.g0_.y, _e115.g0_.x, _e118.g0_.y, _e121.g0_.y)) * vec4<f32>(1.0, 1.0, 0.0, 0.0))) + ((_e133.g0_.zzxx * vec4<f32>(_e136.g0_.x, _e139.g0_.y, _e142.g0_.x, _e145.g0_.y)) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0))));
}

fn multi_vector_ideal_point_scalar_product(self_232: MultiVector, other_198: IdealPoint) -> Scalar {
    var self_233: MultiVector;
    var other_199: IdealPoint;

    self_233 = self_232;
    other_199 = other_198;
    let _e5: MultiVector = self_233;
    let _e8: IdealPoint = other_199;
    let _e13: MultiVector = self_233;
    let _e16: IdealPoint = other_199;
    return Scalar(((0.0 - (_e5.g1_.z * _e8.g0_.x)) - (_e13.g1_.w * _e16.g0_.y)));
}

fn multi_vector_plane_into(self_234: MultiVector) -> Plane {
    var self_235: MultiVector;

    self_235 = self_234;
    let _e2: MultiVector = self_235;
    let _e5: MultiVector = self_235;
    let _e8: MultiVector = self_235;
    return Plane(vec3<f32>(_e2.g1_.x, _e5.g0_.w, _e8.g0_.z));
}

fn multi_vector_plane_add(self_236: MultiVector, other_200: Plane) -> MultiVector {
    var self_237: MultiVector;
    var other_201: Plane;

    self_237 = self_236;
    other_201 = other_200;
    let _e4: MultiVector = self_237;
    let _e6: Plane = other_201;
    let _e9: Plane = other_201;
    let _e12: Plane = other_201;
    let _e15: Plane = other_201;
    let _e26: MultiVector = self_237;
    let _e28: Plane = other_201;
    return MultiVector((_e4.g0_ + (vec4<f32>(_e6.g0_.x, _e9.g0_.x, _e12.g0_.z, _e15.g0_.y) * vec4<f32>(0.0, 0.0, 1.0, 1.0))), (_e26.g1_ + (vec4<f32>(_e28.g0_.x) * vec4<f32>(1.0, 0.0, 0.0, 0.0))));
}

fn multi_vector_plane_sub(self_238: MultiVector, other_202: Plane) -> MultiVector {
    var self_239: MultiVector;
    var other_203: Plane;

    self_239 = self_238;
    other_203 = other_202;
    let _e4: MultiVector = self_239;
    let _e6: Plane = other_203;
    let _e9: Plane = other_203;
    let _e12: Plane = other_203;
    let _e15: Plane = other_203;
    let _e26: MultiVector = self_239;
    let _e28: Plane = other_203;
    return MultiVector((_e4.g0_ - (vec4<f32>(_e6.g0_.x, _e9.g0_.x, _e12.g0_.z, _e15.g0_.y) * vec4<f32>(0.0, 0.0, 1.0, 1.0))), (_e26.g1_ - (vec4<f32>(_e28.g0_.x) * vec4<f32>(1.0, 0.0, 0.0, 0.0))));
}

fn multi_vector_plane_geometric_product(self_240: MultiVector, other_204: Plane) -> MultiVector {
    var self_241: MultiVector;
    var other_205: Plane;

    self_241 = self_240;
    other_205 = other_204;
    let _e4: MultiVector = self_241;
    let _e8: Plane = other_205;
    let _e11: Plane = other_205;
    let _e14: Plane = other_205;
    let _e17: Plane = other_205;
    let _e29: MultiVector = self_241;
    let _e33: Plane = other_205;
    let _e36: Plane = other_205;
    let _e39: Plane = other_205;
    let _e42: Plane = other_205;
    let _e55: MultiVector = self_241;
    let _e59: Plane = other_205;
    let _e71: MultiVector = self_241;
    let _e75: Plane = other_205;
    let _e87: MultiVector = self_241;
    let _e91: Plane = other_205;
    let _e104: MultiVector = self_241;
    let _e108: Plane = other_205;
    let _e120: MultiVector = self_241;
    let _e123: Plane = other_205;
    let _e126: Plane = other_205;
    let _e129: Plane = other_205;
    let _e132: Plane = other_205;
    let _e138: MultiVector = self_241;
    let _e142: Plane = other_205;
    let _e145: Plane = other_205;
    let _e148: Plane = other_205;
    let _e151: Plane = other_205;
    let _e163: MultiVector = self_241;
    let _e167: Plane = other_205;
    let _e170: Plane = other_205;
    let _e173: Plane = other_205;
    let _e176: Plane = other_205;
    let _e188: MultiVector = self_241;
    let _e192: Plane = other_205;
    let _e195: Plane = other_205;
    let _e198: Plane = other_205;
    let _e201: Plane = other_205;
    let _e213: MultiVector = self_241;
    let _e217: Plane = other_205;
    let _e220: Plane = other_205;
    let _e223: Plane = other_205;
    let _e226: Plane = other_205;
    let _e239: MultiVector = self_241;
    let _e241: Plane = other_205;
    return MultiVector(((((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y, _e11.g0_.y, _e14.g0_.y, _e17.g0_.z)) * vec4<f32>(0.0, 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e29.g0_.w) * vec4<f32>(_e33.g0_.y, _e36.g0_.z, _e39.g0_.y, _e42.g0_.y)) * vec4<f32>(1.0, -(1.0), 0.0, 0.0))) + ((vec4<f32>(_e55.g1_.x) * vec4<f32>(_e59.g0_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e71.g1_.y) * vec4<f32>(_e75.g0_.x)) * vec4<f32>(0.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e87.g1_.z) * vec4<f32>(_e91.g0_.x)) * vec4<f32>(0.0, 0.0, -(1.0), 0.0))) + ((vec4<f32>(_e104.g1_.w) * vec4<f32>(_e108.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + (_e120.g0_.zzxx * vec4<f32>(_e123.g0_.z, _e126.g0_.y, _e129.g0_.z, _e132.g0_.y))), ((((((vec4<f32>(_e138.g1_.x) * vec4<f32>(_e142.g0_.z, _e145.g0_.z, _e148.g0_.z, _e151.g0_.y)) * vec4<f32>(0.0, 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e163.g1_.y) * vec4<f32>(_e167.g0_.y, _e170.g0_.y, _e173.g0_.y, _e176.g0_.z)) * vec4<f32>(0.0, 0.0, 1.0, 1.0))) + ((vec4<f32>(_e188.g1_.z) * vec4<f32>(_e192.g0_.z, _e195.g0_.y, _e198.g0_.z, _e201.g0_.z)) * vec4<f32>(1.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e213.g1_.w) * vec4<f32>(_e217.g0_.y, _e220.g0_.z, _e223.g0_.y, _e226.g0_.y)) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))) + ((_e239.g0_ * vec4<f32>(_e241.g0_.x)) * vec4<f32>(1.0, 1.0, -(1.0), 1.0))));
}

fn multi_vector_plane_scalar_product(self_242: MultiVector, other_206: Plane) -> Scalar {
    var self_243: MultiVector;
    var other_207: Plane;

    self_243 = self_242;
    other_207 = other_206;
    let _e4: MultiVector = self_243;
    let _e7: Plane = other_207;
    let _e11: MultiVector = self_243;
    let _e14: Plane = other_207;
    let _e19: MultiVector = self_243;
    let _e22: Plane = other_207;
    return Scalar((((_e4.g0_.z * _e7.g0_.z) + (_e11.g0_.w * _e14.g0_.y)) + (_e19.g1_.x * _e22.g0_.x)));
}

fn multi_vector_translator_into(self_244: MultiVector) -> Translator {
    var self_245: MultiVector;

    self_245 = self_244;
    let _e2: MultiVector = self_245;
    let _e5: MultiVector = self_245;
    let _e8: MultiVector = self_245;
    return Translator(vec3<f32>(_e2.g0_.x, _e5.g1_.z, _e8.g1_.w));
}

fn multi_vector_translator_add(self_246: MultiVector, other_208: Translator) -> MultiVector {
    var self_247: MultiVector;
    var other_209: Translator;

    self_247 = self_246;
    other_209 = other_208;
    let _e4: MultiVector = self_247;
    let _e6: Translator = other_209;
    let _e17: MultiVector = self_247;
    let _e19: Translator = other_209;
    let _e22: Translator = other_209;
    let _e25: Translator = other_209;
    let _e28: Translator = other_209;
    return MultiVector((_e4.g0_ + (vec4<f32>(_e6.g0_.x) * vec4<f32>(1.0, 0.0, 0.0, 0.0))), (_e17.g1_ + (vec4<f32>(_e19.g0_.x, _e22.g0_.x, _e25.g0_.y, _e28.g0_.z) * vec4<f32>(0.0, 0.0, 1.0, 1.0))));
}

fn multi_vector_translator_sub(self_248: MultiVector, other_210: Translator) -> MultiVector {
    var self_249: MultiVector;
    var other_211: Translator;

    self_249 = self_248;
    other_211 = other_210;
    let _e4: MultiVector = self_249;
    let _e6: Translator = other_211;
    let _e17: MultiVector = self_249;
    let _e19: Translator = other_211;
    let _e22: Translator = other_211;
    let _e25: Translator = other_211;
    let _e28: Translator = other_211;
    return MultiVector((_e4.g0_ - (vec4<f32>(_e6.g0_.x) * vec4<f32>(1.0, 0.0, 0.0, 0.0))), (_e17.g1_ - (vec4<f32>(_e19.g0_.x, _e22.g0_.x, _e25.g0_.y, _e28.g0_.z) * vec4<f32>(0.0, 0.0, 1.0, 1.0))));
}

fn multi_vector_translator_geometric_product(self_250: MultiVector, other_212: Translator) -> MultiVector {
    var self_251: MultiVector;
    var other_213: Translator;

    self_251 = self_250;
    other_213 = other_212;
    let _e4: MultiVector = self_251;
    let _e8: Translator = other_213;
    let _e11: Translator = other_213;
    let _e14: Translator = other_213;
    let _e17: Translator = other_213;
    let _e29: MultiVector = self_251;
    let _e33: Translator = other_213;
    let _e36: Translator = other_213;
    let _e39: Translator = other_213;
    let _e42: Translator = other_213;
    let _e56: MultiVector = self_251;
    let _e60: Translator = other_213;
    let _e63: Translator = other_213;
    let _e66: Translator = other_213;
    let _e69: Translator = other_213;
    let _e82: MultiVector = self_251;
    let _e86: Translator = other_213;
    let _e89: Translator = other_213;
    let _e92: Translator = other_213;
    let _e95: Translator = other_213;
    let _e109: MultiVector = self_251;
    let _e111: Translator = other_213;
    let _e117: MultiVector = self_251;
    let _e121: Translator = other_213;
    let _e124: Translator = other_213;
    let _e127: Translator = other_213;
    let _e130: Translator = other_213;
    let _e142: MultiVector = self_251;
    let _e146: Translator = other_213;
    let _e149: Translator = other_213;
    let _e152: Translator = other_213;
    let _e155: Translator = other_213;
    let _e167: MultiVector = self_251;
    let _e171: Translator = other_213;
    let _e183: MultiVector = self_251;
    let _e187: Translator = other_213;
    let _e199: MultiVector = self_251;
    let _e203: Translator = other_213;
    let _e215: MultiVector = self_251;
    let _e219: Translator = other_213;
    let _e231: MultiVector = self_251;
    let _e234: Translator = other_213;
    let _e237: Translator = other_213;
    let _e240: Translator = other_213;
    let _e243: Translator = other_213;
    return MultiVector(((((((vec4<f32>(_e4.g1_.x) * vec4<f32>(_e8.g0_.y, _e11.g0_.y, _e14.g0_.y, _e17.g0_.z)) * vec4<f32>(0.0, 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e29.g1_.y) * vec4<f32>(_e33.g0_.z, _e36.g0_.z, _e39.g0_.z, _e42.g0_.y)) * vec4<f32>(0.0, 0.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e56.g1_.z) * vec4<f32>(_e60.g0_.y, _e63.g0_.z, _e66.g0_.y, _e69.g0_.y)) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))) + ((vec4<f32>(_e82.g1_.w) * vec4<f32>(_e86.g0_.z, _e89.g0_.y, _e92.g0_.z, _e95.g0_.z)) * vec4<f32>(-(1.0), -(1.0), 0.0, 0.0))) + (_e109.g0_ * vec4<f32>(_e111.g0_.x))), ((((((((vec4<f32>(_e117.g0_.y) * vec4<f32>(_e121.g0_.z, _e124.g0_.z, _e127.g0_.z, _e130.g0_.y)) * vec4<f32>(0.0, 0.0, -(1.0), 1.0)) + ((vec4<f32>(_e142.g0_.w) * vec4<f32>(_e146.g0_.z, _e149.g0_.y, _e152.g0_.z, _e155.g0_.z)) * vec4<f32>(1.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e167.g1_.x) * vec4<f32>(_e171.g0_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e183.g1_.y) * vec4<f32>(_e187.g0_.x)) * vec4<f32>(0.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e199.g1_.z) * vec4<f32>(_e203.g0_.x)) * vec4<f32>(0.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e215.g1_.w) * vec4<f32>(_e219.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((_e231.g0_.zzxx * vec4<f32>(_e234.g0_.y, _e237.g0_.z, _e240.g0_.y, _e243.g0_.z)) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0))));
}

fn multi_vector_translator_outer_product(self_252: MultiVector, other_214: Translator) -> MultiVector {
    var self_253: MultiVector;
    var other_215: Translator;

    self_253 = self_252;
    other_215 = other_214;
    let _e4: MultiVector = self_253;
    let _e6: Translator = other_215;
    let _e11: MultiVector = self_253;
    let _e15: Translator = other_215;
    let _e26: MultiVector = self_253;
    let _e30: Translator = other_215;
    let _e42: MultiVector = self_253;
    let _e46: Translator = other_215;
    let _e58: MultiVector = self_253;
    let _e62: Translator = other_215;
    let _e74: MultiVector = self_253;
    let _e77: MultiVector = self_253;
    let _e80: MultiVector = self_253;
    let _e83: MultiVector = self_253;
    let _e87: Translator = other_215;
    let _e90: Translator = other_215;
    let _e93: Translator = other_215;
    let _e96: Translator = other_215;
    return MultiVector((_e4.g0_ * vec4<f32>(_e6.g0_.x)), ((((((vec4<f32>(_e11.g0_.w) * vec4<f32>(_e15.g0_.y)) * vec4<f32>(0.0, 1.0, 0.0, 0.0)) + ((vec4<f32>(_e26.g1_.y) * vec4<f32>(_e30.g0_.x)) * vec4<f32>(0.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e42.g1_.z) * vec4<f32>(_e46.g0_.x)) * vec4<f32>(0.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e58.g1_.w) * vec4<f32>(_e62.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + (vec4<f32>(_e74.g1_.x, _e77.g0_.z, _e80.g0_.x, _e83.g0_.x) * vec4<f32>(_e87.g0_.x, _e90.g0_.z, _e93.g0_.y, _e96.g0_.z))));
}

fn multi_vector_translator_inner_product(self_254: MultiVector, other_216: Translator) -> MultiVector {
    var self_255: MultiVector;
    var other_217: Translator;

    self_255 = self_254;
    other_217 = other_216;
    let _e4: MultiVector = self_255;
    let _e8: Translator = other_217;
    let _e11: Translator = other_217;
    let _e14: Translator = other_217;
    let _e17: Translator = other_217;
    let _e29: MultiVector = self_255;
    let _e33: Translator = other_217;
    let _e36: Translator = other_217;
    let _e39: Translator = other_217;
    let _e42: Translator = other_217;
    let _e56: MultiVector = self_255;
    let _e60: Translator = other_217;
    let _e73: MultiVector = self_255;
    let _e77: Translator = other_217;
    let _e90: MultiVector = self_255;
    let _e92: Translator = other_217;
    let _e98: MultiVector = self_255;
    let _e102: Translator = other_217;
    let _e113: MultiVector = self_255;
    let _e117: Translator = other_217;
    let _e129: MultiVector = self_255;
    let _e133: Translator = other_217;
    let _e145: MultiVector = self_255;
    let _e149: Translator = other_217;
    let _e161: MultiVector = self_255;
    let _e164: MultiVector = self_255;
    let _e167: MultiVector = self_255;
    let _e170: MultiVector = self_255;
    let _e174: Translator = other_217;
    let _e177: Translator = other_217;
    let _e180: Translator = other_217;
    let _e183: Translator = other_217;
    return MultiVector(((((((vec4<f32>(_e4.g1_.x) * vec4<f32>(_e8.g0_.y, _e11.g0_.y, _e14.g0_.y, _e17.g0_.z)) * vec4<f32>(0.0, 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e29.g1_.y) * vec4<f32>(_e33.g0_.z, _e36.g0_.z, _e39.g0_.z, _e42.g0_.y)) * vec4<f32>(0.0, 0.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e56.g1_.z) * vec4<f32>(_e60.g0_.y)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((vec4<f32>(_e73.g1_.w) * vec4<f32>(_e77.g0_.z)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + (_e90.g0_ * vec4<f32>(_e92.g0_.x))), ((((((vec4<f32>(_e98.g0_.w) * vec4<f32>(_e102.g0_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) + ((vec4<f32>(_e113.g1_.x) * vec4<f32>(_e117.g0_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e129.g1_.z) * vec4<f32>(_e133.g0_.x)) * vec4<f32>(0.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e145.g1_.w) * vec4<f32>(_e149.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e161.g0_.z, _e164.g1_.y, _e167.g0_.x, _e170.g0_.x) * vec4<f32>(_e174.g0_.y, _e177.g0_.x, _e180.g0_.y, _e183.g0_.z)) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0))));
}

fn multi_vector_translator_right_contraction(self_256: MultiVector, other_218: Translator) -> MultiVector {
    var self_257: MultiVector;
    var other_219: Translator;

    self_257 = self_256;
    other_219 = other_218;
    let _e4: MultiVector = self_257;
    let _e8: Translator = other_219;
    let _e11: Translator = other_219;
    let _e14: Translator = other_219;
    let _e17: Translator = other_219;
    let _e30: MultiVector = self_257;
    let _e34: Translator = other_219;
    let _e47: MultiVector = self_257;
    let _e51: Translator = other_219;
    let _e64: MultiVector = self_257;
    let _e66: Translator = other_219;
    let _e72: MultiVector = self_257;
    let _e74: Translator = other_219;
    return MultiVector((((((vec4<f32>(_e4.g1_.y) * vec4<f32>(_e8.g0_.z, _e11.g0_.z, _e14.g0_.z, _e17.g0_.y)) * vec4<f32>(0.0, 0.0, -(1.0), -(1.0))) + ((vec4<f32>(_e30.g1_.z) * vec4<f32>(_e34.g0_.y)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((vec4<f32>(_e47.g1_.w) * vec4<f32>(_e51.g0_.z)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + (_e64.g0_ * vec4<f32>(_e66.g0_.x))), (_e72.g1_ * vec4<f32>(_e74.g0_.x)));
}

fn multi_vector_translator_scalar_product(self_258: MultiVector, other_220: Translator) -> Scalar {
    var self_259: MultiVector;
    var other_221: Translator;

    self_259 = self_258;
    other_221 = other_220;
    let _e4: MultiVector = self_259;
    let _e7: Translator = other_221;
    let _e11: MultiVector = self_259;
    let _e14: Translator = other_221;
    let _e19: MultiVector = self_259;
    let _e22: Translator = other_221;
    return Scalar((((_e4.g0_.x * _e7.g0_.x) - (_e11.g1_.z * _e14.g0_.y)) - (_e19.g1_.w * _e22.g0_.z)));
}

fn multi_vector_motor_into(self_260: MultiVector) -> Motor {
    var self_261: MultiVector;

    self_261 = self_260;
    let _e2: MultiVector = self_261;
    let _e5: MultiVector = self_261;
    let _e8: MultiVector = self_261;
    let _e11: MultiVector = self_261;
    return Motor(vec4<f32>(_e2.g0_.x, _e5.g0_.y, _e8.g1_.z, _e11.g1_.w));
}

fn multi_vector_motor_add(self_262: MultiVector, other_222: Motor) -> MultiVector {
    var self_263: MultiVector;
    var other_223: Motor;

    self_263 = self_262;
    other_223 = other_222;
    let _e4: MultiVector = self_263;
    let _e6: Motor = other_223;
    let _e16: MultiVector = self_263;
    let _e18: Motor = other_223;
    return MultiVector((_e4.g0_ + (_e6.g0_.xyxx * vec4<f32>(1.0, 1.0, 0.0, 0.0))), (_e16.g1_ + (_e18.g0_.xxzw * vec4<f32>(0.0, 0.0, 1.0, 1.0))));
}

fn multi_vector_motor_sub(self_264: MultiVector, other_224: Motor) -> MultiVector {
    var self_265: MultiVector;
    var other_225: Motor;

    self_265 = self_264;
    other_225 = other_224;
    let _e4: MultiVector = self_265;
    let _e6: Motor = other_225;
    let _e16: MultiVector = self_265;
    let _e18: Motor = other_225;
    return MultiVector((_e4.g0_ - (_e6.g0_.xyxx * vec4<f32>(1.0, 1.0, 0.0, 0.0))), (_e16.g1_ - (_e18.g0_.xxzw * vec4<f32>(0.0, 0.0, 1.0, 1.0))));
}

fn multi_vector_motor_geometric_product(self_266: MultiVector, other_226: Motor) -> MultiVector {
    var self_267: MultiVector;
    var other_227: Motor;

    self_267 = self_266;
    other_227 = other_226;
    let _e4: MultiVector = self_267;
    let _e8: Motor = other_227;
    let _e19: MultiVector = self_267;
    let _e23: Motor = other_227;
    let _e35: MultiVector = self_267;
    let _e39: Motor = other_227;
    let _e51: MultiVector = self_267;
    let _e55: Motor = other_227;
    let _e68: MultiVector = self_267;
    let _e72: Motor = other_227;
    let _e84: MultiVector = self_267;
    let _e88: Motor = other_227;
    let _e101: MultiVector = self_267;
    let _e104: Motor = other_227;
    let _e109: MultiVector = self_267;
    let _e113: Motor = other_227;
    let _e124: MultiVector = self_267;
    let _e128: Motor = other_227;
    let _e139: MultiVector = self_267;
    let _e143: Motor = other_227;
    let _e154: MultiVector = self_267;
    let _e158: Motor = other_227;
    let _e170: MultiVector = self_267;
    let _e174: Motor = other_227;
    let _e186: MultiVector = self_267;
    let _e190: Motor = other_227;
    let _e201: MultiVector = self_267;
    let _e204: Motor = other_227;
    return MultiVector(((((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0)) + ((vec4<f32>(_e19.g0_.w) * _e23.g0_.yyyx) * vec4<f32>(0.0, 0.0, -(1.0), 1.0))) + ((vec4<f32>(_e35.g1_.x) * _e39.g0_.zzzw) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e51.g1_.y) * _e55.g0_.wwwz) * vec4<f32>(0.0, 0.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e68.g1_.z) * _e72.g0_.zwzz) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))) + ((vec4<f32>(_e84.g1_.w) * _e88.g0_.wzww) * vec4<f32>(-(1.0), -(1.0), 0.0, 0.0))) + (_e101.g0_.xxzz * _e104.g0_.xyxy)), ((((((((vec4<f32>(_e109.g0_.y) * _e113.g0_.wwwz) * vec4<f32>(0.0, 0.0, -(1.0), 1.0)) + ((vec4<f32>(_e124.g0_.w) * _e128.g0_.wzww) * vec4<f32>(1.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e139.g1_.x) * _e143.g0_.xyxx) * vec4<f32>(1.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e154.g1_.y) * _e158.g0_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))) + ((vec4<f32>(_e170.g1_.z) * _e174.g0_.xxxy) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e186.g1_.w) * _e190.g0_.yyyx) * vec4<f32>(0.0, 0.0, 1.0, 1.0))) + ((_e201.g0_.zzxx * _e204.g0_.zwzw) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0))));
}

fn multi_vector_motor_outer_product(self_268: MultiVector, other_228: Motor) -> MultiVector {
    var self_269: MultiVector;
    var other_229: Motor;

    self_269 = self_268;
    other_229 = other_228;
    let _e4: MultiVector = self_269;
    let _e8: Motor = other_229;
    let _e19: MultiVector = self_269;
    let _e22: Motor = other_229;
    let _e27: MultiVector = self_269;
    let _e31: Motor = other_229;
    let _e42: MultiVector = self_269;
    let _e46: Motor = other_229;
    let _e57: MultiVector = self_269;
    let _e61: Motor = other_229;
    let _e73: MultiVector = self_269;
    let _e77: Motor = other_229;
    let _e89: MultiVector = self_269;
    let _e93: Motor = other_229;
    let _e105: MultiVector = self_269;
    let _e108: Motor = other_229;
    return MultiVector((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.x)) * vec4<f32>(0.0, 1.0, 0.0, 0.0)) + (_e19.g0_.xxzw * _e22.g0_.xyxx)), (((((((vec4<f32>(_e27.g0_.w) * vec4<f32>(_e31.g0_.z)) * vec4<f32>(0.0, 1.0, 0.0, 0.0)) + ((vec4<f32>(_e42.g1_.x) * _e46.g0_.xyxx) * vec4<f32>(1.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e57.g1_.y) * vec4<f32>(_e61.g0_.x)) * vec4<f32>(0.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e73.g1_.z) * vec4<f32>(_e77.g0_.x)) * vec4<f32>(0.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e89.g1_.w) * vec4<f32>(_e93.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((_e105.g0_.xzxx * _e108.g0_.xwzw) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn multi_vector_motor_inner_product(self_270: MultiVector, other_230: Motor) -> MultiVector {
    var self_271: MultiVector;
    var other_231: Motor;

    self_271 = self_270;
    other_231 = other_230;
    let _e4: MultiVector = self_271;
    let _e8: Motor = other_231;
    let _e19: MultiVector = self_271;
    let _e23: Motor = other_231;
    let _e35: MultiVector = self_271;
    let _e39: Motor = other_231;
    let _e51: MultiVector = self_271;
    let _e55: Motor = other_231;
    let _e68: MultiVector = self_271;
    let _e72: Motor = other_231;
    let _e85: MultiVector = self_271;
    let _e89: Motor = other_231;
    let _e102: MultiVector = self_271;
    let _e105: Motor = other_231;
    let _e110: MultiVector = self_271;
    let _e114: Motor = other_231;
    let _e125: MultiVector = self_271;
    let _e129: Motor = other_231;
    let _e141: MultiVector = self_271;
    let _e145: Motor = other_231;
    let _e157: MultiVector = self_271;
    let _e161: Motor = other_231;
    let _e173: MultiVector = self_271;
    let _e177: Motor = other_231;
    let _e189: MultiVector = self_271;
    let _e192: Motor = other_231;
    return MultiVector(((((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0)) + ((vec4<f32>(_e19.g0_.w) * _e23.g0_.yyyx) * vec4<f32>(0.0, 0.0, -(1.0), 1.0))) + ((vec4<f32>(_e35.g1_.x) * _e39.g0_.zzzw) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e51.g1_.y) * _e55.g0_.wwwz) * vec4<f32>(0.0, 0.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e68.g1_.z) * vec4<f32>(_e72.g0_.z)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((vec4<f32>(_e85.g1_.w) * vec4<f32>(_e89.g0_.w)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + (_e102.g0_.xxzz * _e105.g0_.xyxy)), (((((((vec4<f32>(_e110.g0_.w) * vec4<f32>(_e114.g0_.w)) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) + ((vec4<f32>(_e125.g1_.x) * vec4<f32>(_e129.g0_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e141.g1_.y) * _e145.g0_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))) + ((vec4<f32>(_e157.g1_.z) * vec4<f32>(_e161.g0_.x)) * vec4<f32>(0.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e173.g1_.w) * vec4<f32>(_e177.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((_e189.g0_.zxxx * _e192.g0_.zxzw) * vec4<f32>(-(1.0), 0.0, 1.0, 1.0))));
}

fn multi_vector_motor_right_contraction(self_272: MultiVector, other_232: Motor) -> MultiVector {
    var self_273: MultiVector;
    var other_233: Motor;

    self_273 = self_272;
    other_233 = other_232;
    let _e4: MultiVector = self_273;
    let _e8: Motor = other_233;
    let _e19: MultiVector = self_273;
    let _e23: Motor = other_233;
    let _e36: MultiVector = self_273;
    let _e40: Motor = other_233;
    let _e53: MultiVector = self_273;
    let _e57: Motor = other_233;
    let _e70: MultiVector = self_273;
    let _e73: Motor = other_233;
    let _e85: MultiVector = self_273;
    let _e89: Motor = other_233;
    let _e100: MultiVector = self_273;
    let _e103: Motor = other_233;
    return MultiVector(((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0)) + ((vec4<f32>(_e19.g1_.y) * _e23.g0_.wwwz) * vec4<f32>(0.0, 0.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e36.g1_.z) * vec4<f32>(_e40.g0_.z)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((vec4<f32>(_e53.g1_.w) * vec4<f32>(_e57.g0_.w)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((_e70.g0_.xxzw * vec4<f32>(_e73.g0_.x)) * vec4<f32>(1.0, 0.0, 1.0, 1.0))), (((vec4<f32>(_e85.g1_.y) * _e89.g0_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0)) + ((_e100.g1_.xxzw * vec4<f32>(_e103.g0_.x)) * vec4<f32>(1.0, 0.0, 1.0, 1.0))));
}

fn multi_vector_motor_scalar_product(self_274: MultiVector, other_234: Motor) -> Scalar {
    var self_275: MultiVector;
    var other_235: Motor;

    self_275 = self_274;
    other_235 = other_234;
    let _e4: MultiVector = self_275;
    let _e7: Motor = other_235;
    let _e11: MultiVector = self_275;
    let _e14: Motor = other_235;
    let _e19: MultiVector = self_275;
    let _e22: Motor = other_235;
    let _e27: MultiVector = self_275;
    let _e30: Motor = other_235;
    return Scalar(((((_e4.g0_.x * _e7.g0_.x) - (_e11.g0_.y * _e14.g0_.y)) - (_e19.g1_.z * _e22.g0_.z)) - (_e27.g1_.w * _e30.g0_.w)));
}

fn multi_vector_motor_dual_into(self_276: MultiVector) -> MotorDual {
    var self_277: MultiVector;

    self_277 = self_276;
    let _e2: MultiVector = self_277;
    let _e5: MultiVector = self_277;
    let _e8: MultiVector = self_277;
    let _e11: MultiVector = self_277;
    return MotorDual(vec4<f32>(_e2.g1_.y, _e5.g1_.x, _e8.g0_.w, _e11.g0_.z));
}

fn multi_vector_motor_dual_add(self_278: MultiVector, other_236: MotorDual) -> MultiVector {
    var self_279: MultiVector;
    var other_237: MotorDual;

    self_279 = self_278;
    other_237 = other_236;
    let _e4: MultiVector = self_279;
    let _e6: MotorDual = other_237;
    let _e16: MultiVector = self_279;
    let _e18: MotorDual = other_237;
    return MultiVector((_e4.g0_ + (_e6.g0_.xxwz * vec4<f32>(0.0, 0.0, 1.0, 1.0))), (_e16.g1_ + (_e18.g0_.yxxx * vec4<f32>(1.0, 1.0, 0.0, 0.0))));
}

fn multi_vector_motor_dual_sub(self_280: MultiVector, other_238: MotorDual) -> MultiVector {
    var self_281: MultiVector;
    var other_239: MotorDual;

    self_281 = self_280;
    other_239 = other_238;
    let _e4: MultiVector = self_281;
    let _e6: MotorDual = other_239;
    let _e16: MultiVector = self_281;
    let _e18: MotorDual = other_239;
    return MultiVector((_e4.g0_ - (_e6.g0_.xxwz * vec4<f32>(0.0, 0.0, 1.0, 1.0))), (_e16.g1_ - (_e18.g0_.yxxx * vec4<f32>(1.0, 1.0, 0.0, 0.0))));
}

fn multi_vector_motor_dual_geometric_product(self_282: MultiVector, other_240: MotorDual) -> MultiVector {
    var self_283: MultiVector;
    var other_241: MotorDual;

    self_283 = self_282;
    other_241 = other_240;
    let _e4: MultiVector = self_283;
    let _e8: MotorDual = other_241;
    let _e19: MultiVector = self_283;
    let _e23: MotorDual = other_241;
    let _e35: MultiVector = self_283;
    let _e39: MotorDual = other_241;
    let _e50: MultiVector = self_283;
    let _e54: MotorDual = other_241;
    let _e66: MultiVector = self_283;
    let _e70: MotorDual = other_241;
    let _e83: MultiVector = self_283;
    let _e87: MotorDual = other_241;
    let _e99: MultiVector = self_283;
    let _e102: MotorDual = other_241;
    let _e107: MultiVector = self_283;
    let _e111: MotorDual = other_241;
    let _e122: MultiVector = self_283;
    let _e126: MotorDual = other_241;
    let _e137: MultiVector = self_283;
    let _e141: MotorDual = other_241;
    let _e153: MultiVector = self_283;
    let _e157: MotorDual = other_241;
    let _e168: MultiVector = self_283;
    let _e172: MotorDual = other_241;
    let _e183: MultiVector = self_283;
    let _e187: MotorDual = other_241;
    let _e199: MultiVector = self_283;
    let _e202: MotorDual = other_241;
    return MultiVector(((((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.zzzw) * vec4<f32>(0.0, 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e19.g0_.w) * _e23.g0_.zwzz) * vec4<f32>(1.0, -(1.0), 0.0, 0.0))) + ((vec4<f32>(_e35.g1_.x) * _e39.g0_.yxyy) * vec4<f32>(1.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e50.g1_.y) * _e54.g0_.xyxx) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))) + ((vec4<f32>(_e66.g1_.z) * _e70.g0_.yyyx) * vec4<f32>(0.0, 0.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e83.g1_.w) * _e87.g0_.xxxy) * vec4<f32>(0.0, 0.0, -(1.0), 1.0))) + (_e99.g0_.zzxx * _e102.g0_.wzwz)), ((((((((vec4<f32>(_e107.g0_.y) * _e111.g0_.xyxx) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0)) + ((vec4<f32>(_e122.g0_.w) * _e126.g0_.xxxy) * vec4<f32>(0.0, 0.0, 1.0, 1.0))) + ((vec4<f32>(_e137.g1_.x) * _e141.g0_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e153.g1_.y) * _e157.g0_.zzzw) * vec4<f32>(0.0, 0.0, 1.0, 1.0))) + ((vec4<f32>(_e168.g1_.z) * _e172.g0_.wzww) * vec4<f32>(1.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e183.g1_.w) * _e187.g0_.zwzz) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))) + ((_e199.g0_.xxzz * _e202.g0_.yxyx) * vec4<f32>(1.0, 1.0, -(1.0), 1.0))));
}

fn multi_vector_motor_dual_regressive_product(self_284: MultiVector, other_242: MotorDual) -> MultiVector {
    var self_285: MultiVector;
    var other_243: MotorDual;

    self_285 = self_284;
    other_243 = other_242;
    let _e4: MultiVector = self_285;
    let _e8: MotorDual = other_243;
    let _e18: MultiVector = self_285;
    let _e22: MotorDual = other_243;
    let _e33: MultiVector = self_285;
    let _e37: MotorDual = other_243;
    let _e49: MultiVector = self_285;
    let _e53: MotorDual = other_243;
    let _e65: MultiVector = self_285;
    let _e68: MotorDual = other_243;
    let _e80: MultiVector = self_285;
    let _e84: MotorDual = other_243;
    let _e94: MultiVector = self_285;
    let _e97: MotorDual = other_243;
    return MultiVector(((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.yxyy) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) + ((vec4<f32>(_e18.g1_.y) * _e22.g0_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, 1.0))) + ((vec4<f32>(_e33.g1_.z) * vec4<f32>(_e37.g0_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e49.g1_.w) * vec4<f32>(_e53.g0_.w)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((_e65.g0_.xxzw * vec4<f32>(_e68.g0_.x)) * vec4<f32>(1.0, 0.0, 1.0, 1.0))), (((vec4<f32>(_e80.g1_.y) * _e84.g0_.yxyy) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) + ((_e94.g1_.xxzw * vec4<f32>(_e97.g0_.x)) * vec4<f32>(1.0, 0.0, 1.0, 1.0))));
}

fn multi_vector_motor_dual_inner_product(self_286: MultiVector, other_244: MotorDual) -> MultiVector {
    var self_287: MultiVector;
    var other_245: MotorDual;

    self_287 = self_286;
    other_245 = other_244;
    let _e4: MultiVector = self_287;
    let _e8: MotorDual = other_245;
    let _e19: MultiVector = self_287;
    let _e23: MotorDual = other_245;
    let _e35: MultiVector = self_287;
    let _e39: MotorDual = other_245;
    let _e50: MultiVector = self_287;
    let _e54: MotorDual = other_245;
    let _e66: MultiVector = self_287;
    let _e70: MotorDual = other_245;
    let _e83: MultiVector = self_287;
    let _e87: MotorDual = other_245;
    let _e99: MultiVector = self_287;
    let _e102: MotorDual = other_245;
    let _e113: MultiVector = self_287;
    let _e117: MotorDual = other_245;
    let _e129: MultiVector = self_287;
    let _e133: MotorDual = other_245;
    let _e144: MultiVector = self_287;
    let _e148: MotorDual = other_245;
    let _e160: MultiVector = self_287;
    let _e164: MotorDual = other_245;
    let _e177: MultiVector = self_287;
    let _e180: MotorDual = other_245;
    return MultiVector(((((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.zzzw) * vec4<f32>(0.0, 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e19.g0_.w) * vec4<f32>(_e23.g0_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e35.g1_.x) * _e39.g0_.yxyy) * vec4<f32>(1.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e50.g1_.y) * _e54.g0_.xyxx) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))) + ((vec4<f32>(_e66.g1_.z) * _e70.g0_.yyyx) * vec4<f32>(0.0, 0.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e83.g1_.w) * _e87.g0_.xxxy) * vec4<f32>(0.0, 0.0, -(1.0), 1.0))) + ((_e99.g0_.zxxx * _e102.g0_.wxwz) * vec4<f32>(1.0, 0.0, 1.0, 1.0))), ((((((vec4<f32>(_e113.g0_.y) * vec4<f32>(_e117.g0_.x)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0)) + ((vec4<f32>(_e129.g1_.y) * _e133.g0_.zzzw) * vec4<f32>(0.0, 0.0, 1.0, 1.0))) + ((vec4<f32>(_e144.g1_.z) * vec4<f32>(_e148.g0_.w)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e160.g1_.w) * vec4<f32>(_e164.g0_.z)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + (_e177.g0_.xxwz * _e180.g0_.yxxx)));
}

fn multi_vector_motor_dual_left_contraction(self_288: MultiVector, other_246: MotorDual) -> MultiVector {
    var self_289: MultiVector;
    var other_247: MotorDual;

    self_289 = self_288;
    other_247 = other_246;
    let _e4: MultiVector = self_289;
    let _e8: MotorDual = other_247;
    let _e19: MultiVector = self_289;
    let _e23: MotorDual = other_247;
    let _e34: MultiVector = self_289;
    let _e38: MotorDual = other_247;
    let _e51: MultiVector = self_289;
    let _e55: MotorDual = other_247;
    let _e68: MultiVector = self_289;
    let _e72: MotorDual = other_247;
    let _e85: MultiVector = self_289;
    let _e88: MotorDual = other_247;
    let _e99: MultiVector = self_289;
    let _e103: MotorDual = other_247;
    let _e115: MultiVector = self_289;
    let _e118: MotorDual = other_247;
    return MultiVector((((((((vec4<f32>(_e4.g0_.w) * vec4<f32>(_e8.g0_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) + ((vec4<f32>(_e19.g1_.x) * _e23.g0_.yxyy) * vec4<f32>(1.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e34.g1_.y) * vec4<f32>(_e38.g0_.x)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((vec4<f32>(_e51.g1_.z) * vec4<f32>(_e55.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e68.g1_.w) * vec4<f32>(_e72.g0_.x)) * vec4<f32>(0.0, 0.0, -(1.0), 0.0))) + ((_e85.g0_.zxxx * _e88.g0_.wxwz) * vec4<f32>(1.0, 0.0, 1.0, 1.0))), (((vec4<f32>(_e99.g0_.y) * vec4<f32>(_e103.g0_.x)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0)) + (_e115.g0_.xxwz * _e118.g0_.yxxx)));
}

fn multi_vector_motor_dual_scalar_product(self_290: MultiVector, other_248: MotorDual) -> Scalar {
    var self_291: MultiVector;
    var other_249: MotorDual;

    self_291 = self_290;
    other_249 = other_248;
    let _e4: MultiVector = self_291;
    let _e7: MotorDual = other_249;
    let _e11: MultiVector = self_291;
    let _e14: MotorDual = other_249;
    let _e19: MultiVector = self_291;
    let _e22: MotorDual = other_249;
    let _e27: MultiVector = self_291;
    let _e30: MotorDual = other_249;
    return Scalar(((((_e4.g0_.z * _e7.g0_.w) + (_e11.g0_.w * _e14.g0_.z)) + (_e19.g1_.x * _e22.g0_.y)) - (_e27.g1_.y * _e30.g0_.x)));
}

fn multi_vector_squared_magnitude(self_292: MultiVector) -> Scalar {
    var self_293: MultiVector;

    self_293 = self_292;
    let _e2: MultiVector = self_293;
    let _e3: MultiVector = self_293;
    let _e4: MultiVector = multi_vector_reversal(_e3);
    let _e5: Scalar = multi_vector_multi_vector_scalar_product(_e2, _e4);
    return _e5;
}

fn multi_vector_magnitude(self_294: MultiVector) -> Scalar {
    var self_295: MultiVector;

    self_295 = self_294;
    let _e2: MultiVector = self_295;
    let _e3: Scalar = multi_vector_squared_magnitude(_e2);
    return Scalar(sqrt(_e3.g0_));
}

fn multi_vector_scale(self_296: MultiVector, other_250: f32) -> MultiVector {
    var self_297: MultiVector;
    var other_251: f32;

    self_297 = self_296;
    other_251 = other_250;
    let _e4: MultiVector = self_297;
    let _e5: f32 = other_251;
    let _e7: MultiVector = multi_vector_scalar_geometric_product(_e4, Scalar(_e5));
    return _e7;
}

fn multi_vector_signum(self_298: MultiVector) -> MultiVector {
    var self_299: MultiVector;

    self_299 = self_298;
    let _e2: MultiVector = self_299;
    let _e3: MultiVector = self_299;
    let _e4: Scalar = multi_vector_magnitude(_e3);
    let _e9: MultiVector = multi_vector_scalar_geometric_product(_e2, Scalar((1.0 / _e4.g0_)));
    return _e9;
}

fn multi_vector_inverse(self_300: MultiVector) -> MultiVector {
    var self_301: MultiVector;

    self_301 = self_300;
    let _e2: MultiVector = self_301;
    let _e3: MultiVector = multi_vector_reversal(_e2);
    let _e4: MultiVector = self_301;
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

fn rotor_neg(self_302: Rotor) -> Rotor {
    var self_303: Rotor;

    self_303 = self_302;
    let _e2: Rotor = self_303;
    return Rotor((_e2.g0_ * vec2<f32>(-(1.0))));
}

fn rotor_automorphism(self_304: Rotor) -> Rotor {
    var self_305: Rotor;

    self_305 = self_304;
    let _e2: Rotor = self_305;
    return Rotor(_e2.g0_);
}

fn rotor_reversal(self_306: Rotor) -> Rotor {
    var self_307: Rotor;

    self_307 = self_306;
    let _e2: Rotor = self_307;
    return Rotor((_e2.g0_ * vec2<f32>(1.0, -(1.0))));
}

fn rotor_conjugation(self_308: Rotor) -> Rotor {
    var self_309: Rotor;

    self_309 = self_308;
    let _e2: Rotor = self_309;
    return Rotor((_e2.g0_ * vec2<f32>(1.0, -(1.0))));
}

fn rotor_scalar_into(self_310: Rotor) -> Scalar {
    var self_311: Rotor;

    self_311 = self_310;
    let _e2: Rotor = self_311;
    return Scalar(_e2.g0_.x);
}

fn rotor_scalar_add(self_312: Rotor, other_252: Scalar) -> Rotor {
    var self_313: Rotor;
    var other_253: Scalar;

    self_313 = self_312;
    other_253 = other_252;
    let _e4: Rotor = self_313;
    let _e6: Scalar = other_253;
    return Rotor((_e4.g0_ + (vec2<f32>(_e6.g0_) * vec2<f32>(1.0, 0.0))));
}

fn rotor_scalar_sub(self_314: Rotor, other_254: Scalar) -> Rotor {
    var self_315: Rotor;
    var other_255: Scalar;

    self_315 = self_314;
    other_255 = other_254;
    let _e4: Rotor = self_315;
    let _e6: Scalar = other_255;
    return Rotor((_e4.g0_ - (vec2<f32>(_e6.g0_) * vec2<f32>(1.0, 0.0))));
}

fn rotor_scalar_geometric_product(self_316: Rotor, other_256: Scalar) -> Rotor {
    var self_317: Rotor;
    var other_257: Scalar;

    self_317 = self_316;
    other_257 = other_256;
    let _e4: Rotor = self_317;
    let _e6: Scalar = other_257;
    return Rotor((_e4.g0_ * vec2<f32>(_e6.g0_)));
}

fn rotor_scalar_outer_product(self_318: Rotor, other_258: Scalar) -> Rotor {
    var self_319: Rotor;
    var other_259: Scalar;

    self_319 = self_318;
    other_259 = other_258;
    let _e4: Rotor = self_319;
    let _e6: Scalar = other_259;
    return Rotor((_e4.g0_ * vec2<f32>(_e6.g0_)));
}

fn rotor_scalar_inner_product(self_320: Rotor, other_260: Scalar) -> Rotor {
    var self_321: Rotor;
    var other_261: Scalar;

    self_321 = self_320;
    other_261 = other_260;
    let _e4: Rotor = self_321;
    let _e6: Scalar = other_261;
    return Rotor((_e4.g0_ * vec2<f32>(_e6.g0_)));
}

fn rotor_scalar_left_contraction(self_322: Rotor, other_262: Scalar) -> Scalar {
    var self_323: Rotor;
    var other_263: Scalar;

    self_323 = self_322;
    other_263 = other_262;
    let _e4: Rotor = self_323;
    let _e7: Scalar = other_263;
    return Scalar((_e4.g0_.x * _e7.g0_));
}

fn rotor_scalar_right_contraction(self_324: Rotor, other_264: Scalar) -> Rotor {
    var self_325: Rotor;
    var other_265: Scalar;

    self_325 = self_324;
    other_265 = other_264;
    let _e4: Rotor = self_325;
    let _e6: Scalar = other_265;
    return Rotor((_e4.g0_ * vec2<f32>(_e6.g0_)));
}

fn rotor_scalar_scalar_product(self_326: Rotor, other_266: Scalar) -> Scalar {
    var self_327: Rotor;
    var other_267: Scalar;

    self_327 = self_326;
    other_267 = other_266;
    let _e4: Rotor = self_327;
    let _e7: Scalar = other_267;
    return Scalar((_e4.g0_.x * _e7.g0_));
}

fn rotor_multi_vector_add(self_328: Rotor, other_268: MultiVector) -> MultiVector {
    var self_329: Rotor;
    var other_269: MultiVector;

    self_329 = self_328;
    other_269 = other_268;
    let _e4: Rotor = self_329;
    let _e7: Rotor = self_329;
    let _e10: Rotor = self_329;
    let _e13: Rotor = self_329;
    let _e23: MultiVector = other_269;
    let _e26: MultiVector = other_269;
    return MultiVector(((vec4<f32>(_e4.g0_.x, _e7.g0_.y, _e10.g0_.x, _e13.g0_.x) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) + _e23.g0_), _e26.g1_);
}

fn rotor_multi_vector_sub(self_330: Rotor, other_270: MultiVector) -> MultiVector {
    var self_331: Rotor;
    var other_271: MultiVector;

    self_331 = self_330;
    other_271 = other_270;
    let _e4: Rotor = self_331;
    let _e7: Rotor = self_331;
    let _e10: Rotor = self_331;
    let _e13: Rotor = self_331;
    let _e23: MultiVector = other_271;
    let _e28: MultiVector = other_271;
    return MultiVector(((vec4<f32>(_e4.g0_.x, _e7.g0_.y, _e10.g0_.x, _e13.g0_.x) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) - _e23.g0_), (vec4<f32>(0.0) - _e28.g1_));
}

fn rotor_multi_vector_geometric_product(self_332: Rotor, other_272: MultiVector) -> MultiVector {
    var self_333: Rotor;
    var other_273: MultiVector;

    self_333 = self_332;
    other_273 = other_272;
    let _e4: Rotor = self_333;
    let _e8: MultiVector = other_273;
    let _e11: Rotor = self_333;
    let _e15: MultiVector = other_273;
    let _e28: Rotor = self_333;
    let _e32: MultiVector = other_273;
    let _e35: Rotor = self_333;
    let _e39: MultiVector = other_273;
    return MultiVector(((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.y) * _e15.g0_.yxwz) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))), ((vec4<f32>(_e28.g0_.x) * _e32.g1_) + ((vec4<f32>(_e35.g0_.y) * _e39.g1_.yxwz) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))));
}

fn rotor_multi_vector_outer_product(self_334: Rotor, other_274: MultiVector) -> MultiVector {
    var self_335: Rotor;
    var other_275: MultiVector;

    self_335 = self_334;
    other_275 = other_274;
    let _e4: Rotor = self_335;
    let _e8: MultiVector = other_275;
    let _e11: Rotor = self_335;
    let _e14: Rotor = self_335;
    let _e17: Rotor = self_335;
    let _e20: Rotor = self_335;
    let _e24: MultiVector = other_275;
    let _e36: Rotor = self_335;
    let _e40: MultiVector = other_275;
    let _e43: Rotor = self_335;
    let _e46: Rotor = self_335;
    let _e49: Rotor = self_335;
    let _e52: Rotor = self_335;
    let _e56: MultiVector = other_275;
    return MultiVector(((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.x, _e14.g0_.y, _e17.g0_.x, _e20.g0_.x) * vec4<f32>(_e24.g0_.x)) * vec4<f32>(0.0, 1.0, 0.0, 0.0))), ((vec4<f32>(_e36.g0_.x) * _e40.g1_) + ((vec4<f32>(_e43.g0_.x, _e46.g0_.y, _e49.g0_.x, _e52.g0_.x) * vec4<f32>(_e56.g1_.x)) * vec4<f32>(0.0, 1.0, 0.0, 0.0))));
}

fn rotor_multi_vector_inner_product(self_336: Rotor, other_276: MultiVector) -> MultiVector {
    var self_337: Rotor;
    var other_277: MultiVector;

    self_337 = self_336;
    other_277 = other_276;
    let _e4: Rotor = self_337;
    let _e8: MultiVector = other_277;
    let _e11: Rotor = self_337;
    let _e15: MultiVector = other_277;
    let _e28: Rotor = self_337;
    let _e32: MultiVector = other_277;
    let _e35: Rotor = self_337;
    let _e38: Rotor = self_337;
    let _e41: Rotor = self_337;
    let _e44: Rotor = self_337;
    let _e48: MultiVector = other_277;
    return MultiVector(((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.y) * _e15.g0_.yxwz) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))), ((vec4<f32>(_e28.g0_.x) * _e32.g1_) + ((vec4<f32>(_e35.g0_.y, _e38.g0_.x, _e41.g0_.x, _e44.g0_.x) * _e48.g1_.yxxx) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))));
}

fn rotor_multi_vector_left_contraction(self_338: Rotor, other_278: MultiVector) -> MultiVector {
    var self_339: Rotor;
    var other_279: MultiVector;

    self_339 = self_338;
    other_279 = other_278;
    let _e4: Rotor = self_339;
    let _e8: MultiVector = other_279;
    let _e11: Rotor = self_339;
    let _e14: Rotor = self_339;
    let _e17: Rotor = self_339;
    let _e20: Rotor = self_339;
    let _e24: MultiVector = other_279;
    let _e36: Rotor = self_339;
    let _e40: MultiVector = other_279;
    let _e43: Rotor = self_339;
    let _e46: Rotor = self_339;
    let _e49: Rotor = self_339;
    let _e52: Rotor = self_339;
    let _e56: MultiVector = other_279;
    return MultiVector(((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.y, _e14.g0_.x, _e17.g0_.x, _e20.g0_.x) * _e24.g0_.yxxx) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))), ((vec4<f32>(_e36.g0_.x) * _e40.g1_) + ((vec4<f32>(_e43.g0_.y, _e46.g0_.x, _e49.g0_.x, _e52.g0_.x) * _e56.g1_.yxxx) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))));
}

fn rotor_multi_vector_scalar_product(self_340: Rotor, other_280: MultiVector) -> Scalar {
    var self_341: Rotor;
    var other_281: MultiVector;

    self_341 = self_340;
    other_281 = other_280;
    let _e4: Rotor = self_341;
    let _e7: MultiVector = other_281;
    let _e11: Rotor = self_341;
    let _e14: MultiVector = other_281;
    return Scalar(((_e4.g0_.x * _e7.g0_.x) - (_e11.g0_.y * _e14.g0_.y)));
}

fn rotor_rotor_add(self_342: Rotor, other_282: Rotor) -> Rotor {
    var self_343: Rotor;
    var other_283: Rotor;

    self_343 = self_342;
    other_283 = other_282;
    let _e4: Rotor = self_343;
    let _e6: Rotor = other_283;
    return Rotor((_e4.g0_ + _e6.g0_));
}

fn rotor_rotor_sub(self_344: Rotor, other_284: Rotor) -> Rotor {
    var self_345: Rotor;
    var other_285: Rotor;

    self_345 = self_344;
    other_285 = other_284;
    let _e4: Rotor = self_345;
    let _e6: Rotor = other_285;
    return Rotor((_e4.g0_ - _e6.g0_));
}

fn rotor_rotor_mul(self_346: Rotor, other_286: Rotor) -> Rotor {
    var self_347: Rotor;
    var other_287: Rotor;

    self_347 = self_346;
    other_287 = other_286;
    let _e4: Rotor = self_347;
    let _e6: Rotor = other_287;
    return Rotor((_e4.g0_ * _e6.g0_));
}

fn rotor_rotor_div(self_348: Rotor, other_288: Rotor) -> Rotor {
    var self_349: Rotor;
    var other_289: Rotor;

    self_349 = self_348;
    other_289 = other_288;
    let _e4: Rotor = self_349;
    let _e7: Rotor = self_349;
    let _e15: Rotor = other_289;
    let _e18: Rotor = other_289;
    return Rotor((((vec2<f32>(_e4.g0_.x, _e7.g0_.y) * vec2<f32>(1.0, 1.0)) / vec2<f32>(_e15.g0_.x, _e18.g0_.y)) * vec2<f32>(1.0, 1.0)));
}

fn rotor_rotor_geometric_product(self_350: Rotor, other_290: Rotor) -> Rotor {
    var self_351: Rotor;
    var other_291: Rotor;

    self_351 = self_350;
    other_291 = other_290;
    let _e4: Rotor = self_351;
    let _e8: Rotor = other_291;
    let _e11: Rotor = self_351;
    let _e15: Rotor = other_291;
    return Rotor(((vec2<f32>(_e4.g0_.x) * _e8.g0_) + ((vec2<f32>(_e11.g0_.y) * _e15.g0_.yx) * vec2<f32>(-(1.0), 1.0))));
}

fn rotor_rotor_outer_product(self_352: Rotor, other_292: Rotor) -> Rotor {
    var self_353: Rotor;
    var other_293: Rotor;

    self_353 = self_352;
    other_293 = other_292;
    let _e4: Rotor = self_353;
    let _e8: Rotor = other_293;
    let _e11: Rotor = self_353;
    let _e13: Rotor = other_293;
    return Rotor(((vec2<f32>(_e4.g0_.x) * _e8.g0_) + ((_e11.g0_ * vec2<f32>(_e13.g0_.x)) * vec2<f32>(0.0, 1.0))));
}

fn rotor_rotor_inner_product(self_354: Rotor, other_294: Rotor) -> Rotor {
    var self_355: Rotor;
    var other_295: Rotor;

    self_355 = self_354;
    other_295 = other_294;
    let _e4: Rotor = self_355;
    let _e8: Rotor = other_295;
    let _e11: Rotor = self_355;
    let _e15: Rotor = other_295;
    return Rotor(((vec2<f32>(_e4.g0_.x) * _e8.g0_) + ((vec2<f32>(_e11.g0_.y) * _e15.g0_.yx) * vec2<f32>(-(1.0), 1.0))));
}

fn rotor_rotor_left_contraction(self_356: Rotor, other_296: Rotor) -> Rotor {
    var self_357: Rotor;
    var other_297: Rotor;

    self_357 = self_356;
    other_297 = other_296;
    let _e4: Rotor = self_357;
    let _e8: Rotor = other_297;
    let _e11: Rotor = self_357;
    let _e14: Rotor = other_297;
    return Rotor(((vec2<f32>(_e4.g0_.x) * _e8.g0_) + ((_e11.g0_.yx * _e14.g0_.yx) * vec2<f32>(-(1.0), 0.0))));
}

fn rotor_rotor_right_contraction(self_358: Rotor, other_298: Rotor) -> Rotor {
    var self_359: Rotor;
    var other_299: Rotor;

    self_359 = self_358;
    other_299 = other_298;
    let _e4: Rotor = self_359;
    let _e8: Rotor = other_299;
    let _e17: Rotor = self_359;
    let _e21: Rotor = other_299;
    return Rotor((((vec2<f32>(_e4.g0_.y) * _e8.g0_.yx) * vec2<f32>(-(1.0), 1.0)) + ((vec2<f32>(_e17.g0_.x) * vec2<f32>(_e21.g0_.x)) * vec2<f32>(1.0, 0.0))));
}

fn rotor_rotor_scalar_product(self_360: Rotor, other_300: Rotor) -> Scalar {
    var self_361: Rotor;
    var other_301: Rotor;

    self_361 = self_360;
    other_301 = other_300;
    let _e4: Rotor = self_361;
    let _e7: Rotor = other_301;
    let _e11: Rotor = self_361;
    let _e14: Rotor = other_301;
    return Scalar(((_e4.g0_.x * _e7.g0_.x) - (_e11.g0_.y * _e14.g0_.y)));
}

fn rotor_point_add(self_362: Rotor, other_302: Point) -> Motor {
    var self_363: Rotor;
    var other_303: Point;

    self_363 = self_362;
    other_303 = other_302;
    let _e4: Rotor = self_363;
    let _e7: Rotor = self_363;
    let _e10: Rotor = self_363;
    let _e13: Rotor = self_363;
    let _e23: Point = other_303;
    let _e26: Point = other_303;
    let _e29: Point = other_303;
    let _e32: Point = other_303;
    return Motor(((vec4<f32>(_e4.g0_.x, _e7.g0_.y, _e10.g0_.x, _e13.g0_.x) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) + (vec4<f32>(_e23.g0_.x, _e26.g0_.x, _e29.g0_.y, _e32.g0_.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn rotor_point_sub(self_364: Rotor, other_304: Point) -> Motor {
    var self_365: Rotor;
    var other_305: Point;

    self_365 = self_364;
    other_305 = other_304;
    let _e4: Rotor = self_365;
    let _e7: Rotor = self_365;
    let _e10: Rotor = self_365;
    let _e13: Rotor = self_365;
    let _e23: Point = other_305;
    let _e26: Point = other_305;
    let _e29: Point = other_305;
    let _e32: Point = other_305;
    return Motor(((vec4<f32>(_e4.g0_.x, _e7.g0_.y, _e10.g0_.x, _e13.g0_.x) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) - (vec4<f32>(_e23.g0_.x, _e26.g0_.x, _e29.g0_.y, _e32.g0_.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn rotor_point_geometric_product(self_366: Rotor, other_306: Point) -> Motor {
    var self_367: Rotor;
    var other_307: Point;

    self_367 = self_366;
    other_307 = other_306;
    let _e4: Rotor = self_367;
    let _e8: Point = other_307;
    let _e11: Point = other_307;
    let _e14: Point = other_307;
    let _e17: Point = other_307;
    let _e30: Rotor = self_367;
    let _e34: Point = other_307;
    let _e37: Point = other_307;
    let _e40: Point = other_307;
    let _e43: Point = other_307;
    return Motor((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.x, _e11.g0_.x, _e14.g0_.z, _e17.g0_.y)) * vec4<f32>(-(1.0), 0.0, -(1.0), 1.0)) + ((vec4<f32>(_e30.g0_.x) * vec4<f32>(_e34.g0_.x, _e37.g0_.x, _e40.g0_.y, _e43.g0_.z)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn rotor_point_outer_product(self_368: Rotor, other_308: Point) -> Point {
    var self_369: Rotor;
    var other_309: Point;

    self_369 = self_368;
    other_309 = other_308;
    let _e4: Rotor = self_369;
    let _e8: Point = other_309;
    return Point((vec3<f32>(_e4.g0_.x) * _e8.g0_));
}

fn rotor_point_inner_product(self_370: Rotor, other_310: Point) -> Motor {
    var self_371: Rotor;
    var other_311: Point;

    self_371 = self_370;
    other_311 = other_310;
    let _e4: Rotor = self_371;
    let _e7: Rotor = self_371;
    let _e10: Rotor = self_371;
    let _e13: Rotor = self_371;
    let _e17: Point = other_311;
    let _e20: Point = other_311;
    let _e23: Point = other_311;
    let _e26: Point = other_311;
    return Motor(((vec4<f32>(_e4.g0_.y, _e7.g0_.x, _e10.g0_.x, _e13.g0_.x) * vec4<f32>(_e17.g0_.x, _e20.g0_.x, _e23.g0_.y, _e26.g0_.z)) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0)));
}

fn rotor_point_left_contraction(self_372: Rotor, other_312: Point) -> Motor {
    var self_373: Rotor;
    var other_313: Point;

    self_373 = self_372;
    other_313 = other_312;
    let _e4: Rotor = self_373;
    let _e7: Rotor = self_373;
    let _e10: Rotor = self_373;
    let _e13: Rotor = self_373;
    let _e17: Point = other_313;
    let _e20: Point = other_313;
    let _e23: Point = other_313;
    let _e26: Point = other_313;
    return Motor(((vec4<f32>(_e4.g0_.y, _e7.g0_.x, _e10.g0_.x, _e13.g0_.x) * vec4<f32>(_e17.g0_.x, _e20.g0_.x, _e23.g0_.y, _e26.g0_.z)) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0)));
}

fn rotor_point_right_contraction(self_374: Rotor, other_314: Point) -> Scalar {
    var self_375: Rotor;
    var other_315: Point;

    self_375 = self_374;
    other_315 = other_314;
    let _e5: Rotor = self_375;
    let _e8: Point = other_315;
    return Scalar((0.0 - (_e5.g0_.y * _e8.g0_.x)));
}

fn rotor_point_scalar_product(self_376: Rotor, other_316: Point) -> Scalar {
    var self_377: Rotor;
    var other_317: Point;

    self_377 = self_376;
    other_317 = other_316;
    let _e5: Rotor = self_377;
    let _e8: Point = other_317;
    return Scalar((0.0 - (_e5.g0_.y * _e8.g0_.x)));
}

fn rotor_ideal_point_add(self_378: Rotor, other_318: IdealPoint) -> Motor {
    var self_379: Rotor;
    var other_319: IdealPoint;

    self_379 = self_378;
    other_319 = other_318;
    let _e4: Rotor = self_379;
    let _e7: Rotor = self_379;
    let _e10: Rotor = self_379;
    let _e13: Rotor = self_379;
    let _e23: IdealPoint = other_319;
    let _e26: IdealPoint = other_319;
    let _e29: IdealPoint = other_319;
    let _e32: IdealPoint = other_319;
    return Motor(((vec4<f32>(_e4.g0_.x, _e7.g0_.y, _e10.g0_.x, _e13.g0_.x) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) + (vec4<f32>(_e23.g0_.x, _e26.g0_.x, _e29.g0_.x, _e32.g0_.y) * vec4<f32>(0.0, 0.0, 1.0, 1.0))));
}

fn rotor_ideal_point_sub(self_380: Rotor, other_320: IdealPoint) -> Motor {
    var self_381: Rotor;
    var other_321: IdealPoint;

    self_381 = self_380;
    other_321 = other_320;
    let _e4: Rotor = self_381;
    let _e7: Rotor = self_381;
    let _e10: Rotor = self_381;
    let _e13: Rotor = self_381;
    let _e23: IdealPoint = other_321;
    let _e26: IdealPoint = other_321;
    let _e29: IdealPoint = other_321;
    let _e32: IdealPoint = other_321;
    return Motor(((vec4<f32>(_e4.g0_.x, _e7.g0_.y, _e10.g0_.x, _e13.g0_.x) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) - (vec4<f32>(_e23.g0_.x, _e26.g0_.x, _e29.g0_.x, _e32.g0_.y) * vec4<f32>(0.0, 0.0, 1.0, 1.0))));
}

fn rotor_ideal_point_geometric_product(self_382: Rotor, other_322: IdealPoint) -> IdealPoint {
    var self_383: Rotor;
    var other_323: IdealPoint;

    self_383 = self_382;
    other_323 = other_322;
    let _e4: Rotor = self_383;
    let _e8: IdealPoint = other_323;
    let _e11: Rotor = self_383;
    let _e15: IdealPoint = other_323;
    return IdealPoint(((vec2<f32>(_e4.g0_.x) * _e8.g0_) + ((vec2<f32>(_e11.g0_.y) * _e15.g0_.yx) * vec2<f32>(-(1.0), 1.0))));
}

fn rotor_ideal_point_outer_product(self_384: Rotor, other_324: IdealPoint) -> IdealPoint {
    var self_385: Rotor;
    var other_325: IdealPoint;

    self_385 = self_384;
    other_325 = other_324;
    let _e4: Rotor = self_385;
    let _e8: IdealPoint = other_325;
    return IdealPoint((vec2<f32>(_e4.g0_.x) * _e8.g0_));
}

fn rotor_ideal_point_inner_product(self_386: Rotor, other_326: IdealPoint) -> IdealPoint {
    var self_387: Rotor;
    var other_327: IdealPoint;

    self_387 = self_386;
    other_327 = other_326;
    let _e4: Rotor = self_387;
    let _e8: IdealPoint = other_327;
    return IdealPoint((vec2<f32>(_e4.g0_.x) * _e8.g0_));
}

fn rotor_ideal_point_left_contraction(self_388: Rotor, other_328: IdealPoint) -> IdealPoint {
    var self_389: Rotor;
    var other_329: IdealPoint;

    self_389 = self_388;
    other_329 = other_328;
    let _e4: Rotor = self_389;
    let _e8: IdealPoint = other_329;
    return IdealPoint((vec2<f32>(_e4.g0_.x) * _e8.g0_));
}

fn rotor_plane_geometric_product(self_390: Rotor, other_330: Plane) -> MotorDual {
    var self_391: Rotor;
    var other_331: Plane;

    self_391 = self_390;
    other_331 = other_330;
    let _e4: Rotor = self_391;
    let _e8: Plane = other_331;
    let _e11: Plane = other_331;
    let _e14: Plane = other_331;
    let _e17: Plane = other_331;
    let _e29: Rotor = self_391;
    let _e33: Plane = other_331;
    let _e36: Plane = other_331;
    let _e39: Plane = other_331;
    let _e42: Plane = other_331;
    return MotorDual((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.x, _e11.g0_.x, _e14.g0_.z, _e17.g0_.y)) * vec4<f32>(1.0, 0.0, -(1.0), 1.0)) + ((vec4<f32>(_e29.g0_.x) * vec4<f32>(_e33.g0_.x, _e36.g0_.x, _e39.g0_.y, _e42.g0_.z)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn rotor_plane_regressive_product(self_392: Rotor, other_332: Plane) -> Scalar {
    var self_393: Rotor;
    var other_333: Plane;

    self_393 = self_392;
    other_333 = other_332;
    let _e4: Rotor = self_393;
    let _e7: Plane = other_333;
    return Scalar((_e4.g0_.y * _e7.g0_.x));
}

fn rotor_plane_outer_product(self_394: Rotor, other_334: Plane) -> MotorDual {
    var self_395: Rotor;
    var other_335: Plane;

    self_395 = self_394;
    other_335 = other_334;
    let _e4: Rotor = self_395;
    let _e7: Rotor = self_395;
    let _e10: Rotor = self_395;
    let _e13: Rotor = self_395;
    let _e17: Plane = other_335;
    let _e20: Plane = other_335;
    let _e23: Plane = other_335;
    let _e26: Plane = other_335;
    return MotorDual((vec4<f32>(_e4.g0_.y, _e7.g0_.x, _e10.g0_.x, _e13.g0_.x) * vec4<f32>(_e17.g0_.x, _e20.g0_.x, _e23.g0_.y, _e26.g0_.z)));
}

fn rotor_plane_inner_product(self_396: Rotor, other_336: Plane) -> Plane {
    var self_397: Rotor;
    var other_337: Plane;

    self_397 = self_396;
    other_337 = other_336;
    let _e4: Rotor = self_397;
    let _e8: Plane = other_337;
    let _e11: Rotor = self_397;
    let _e14: Rotor = self_397;
    let _e17: Rotor = self_397;
    let _e21: Plane = other_337;
    return Plane(((vec3<f32>(_e4.g0_.x) * _e8.g0_) + ((vec3<f32>(_e11.g0_.x, _e14.g0_.y, _e17.g0_.y) * _e21.g0_.xzy) * vec3<f32>(0.0, -(1.0), 1.0))));
}

fn rotor_plane_left_contraction(self_398: Rotor, other_338: Plane) -> Plane {
    var self_399: Rotor;
    var other_339: Plane;

    self_399 = self_398;
    other_339 = other_338;
    let _e4: Rotor = self_399;
    let _e8: Plane = other_339;
    return Plane((vec3<f32>(_e4.g0_.x) * _e8.g0_));
}

fn rotor_translator_add(self_400: Rotor, other_340: Translator) -> Motor {
    var self_401: Rotor;
    var other_341: Translator;

    self_401 = self_400;
    other_341 = other_340;
    let _e4: Rotor = self_401;
    let _e7: Rotor = self_401;
    let _e10: Rotor = self_401;
    let _e13: Rotor = self_401;
    let _e23: Translator = other_341;
    let _e26: Translator = other_341;
    let _e29: Translator = other_341;
    let _e32: Translator = other_341;
    return Motor(((vec4<f32>(_e4.g0_.x, _e7.g0_.y, _e10.g0_.x, _e13.g0_.x) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) + (vec4<f32>(_e23.g0_.x, _e26.g0_.x, _e29.g0_.y, _e32.g0_.z) * vec4<f32>(1.0, 0.0, 1.0, 1.0))));
}

fn rotor_translator_sub(self_402: Rotor, other_342: Translator) -> Motor {
    var self_403: Rotor;
    var other_343: Translator;

    self_403 = self_402;
    other_343 = other_342;
    let _e4: Rotor = self_403;
    let _e7: Rotor = self_403;
    let _e10: Rotor = self_403;
    let _e13: Rotor = self_403;
    let _e23: Translator = other_343;
    let _e26: Translator = other_343;
    let _e29: Translator = other_343;
    let _e32: Translator = other_343;
    return Motor(((vec4<f32>(_e4.g0_.x, _e7.g0_.y, _e10.g0_.x, _e13.g0_.x) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) - (vec4<f32>(_e23.g0_.x, _e26.g0_.x, _e29.g0_.y, _e32.g0_.z) * vec4<f32>(1.0, 0.0, 1.0, 1.0))));
}

fn rotor_translator_geometric_product(self_404: Rotor, other_344: Translator) -> Motor {
    var self_405: Rotor;
    var other_345: Translator;

    self_405 = self_404;
    other_345 = other_344;
    let _e4: Rotor = self_405;
    let _e8: Translator = other_345;
    let _e11: Translator = other_345;
    let _e14: Translator = other_345;
    let _e17: Translator = other_345;
    let _e29: Rotor = self_405;
    let _e33: Translator = other_345;
    let _e36: Translator = other_345;
    let _e39: Translator = other_345;
    let _e42: Translator = other_345;
    return Motor((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.x, _e11.g0_.x, _e14.g0_.z, _e17.g0_.y)) * vec4<f32>(0.0, 1.0, -(1.0), 1.0)) + ((vec4<f32>(_e29.g0_.x) * vec4<f32>(_e33.g0_.x, _e36.g0_.x, _e39.g0_.y, _e42.g0_.z)) * vec4<f32>(1.0, 0.0, 1.0, 1.0))));
}

fn rotor_translator_outer_product(self_406: Rotor, other_346: Translator) -> Motor {
    var self_407: Rotor;
    var other_347: Translator;

    self_407 = self_406;
    other_347 = other_346;
    let _e4: Rotor = self_407;
    let _e7: Rotor = self_407;
    let _e10: Rotor = self_407;
    let _e13: Rotor = self_407;
    let _e17: Translator = other_347;
    let _e20: Translator = other_347;
    let _e23: Translator = other_347;
    let _e26: Translator = other_347;
    return Motor((vec4<f32>(_e4.g0_.x, _e7.g0_.y, _e10.g0_.x, _e13.g0_.x) * vec4<f32>(_e17.g0_.x, _e20.g0_.x, _e23.g0_.y, _e26.g0_.z)));
}

fn rotor_translator_inner_product(self_408: Rotor, other_348: Translator) -> Motor {
    var self_409: Rotor;
    var other_349: Translator;

    self_409 = self_408;
    other_349 = other_348;
    let _e4: Rotor = self_409;
    let _e7: Rotor = self_409;
    let _e10: Rotor = self_409;
    let _e13: Rotor = self_409;
    let _e17: Translator = other_349;
    let _e20: Translator = other_349;
    let _e23: Translator = other_349;
    let _e26: Translator = other_349;
    return Motor((vec4<f32>(_e4.g0_.x, _e7.g0_.y, _e10.g0_.x, _e13.g0_.x) * vec4<f32>(_e17.g0_.x, _e20.g0_.x, _e23.g0_.y, _e26.g0_.z)));
}

fn rotor_translator_left_contraction(self_410: Rotor, other_350: Translator) -> Translator {
    var self_411: Rotor;
    var other_351: Translator;

    self_411 = self_410;
    other_351 = other_350;
    let _e4: Rotor = self_411;
    let _e8: Translator = other_351;
    return Translator((vec3<f32>(_e4.g0_.x) * _e8.g0_));
}

fn rotor_translator_right_contraction(self_412: Rotor, other_352: Translator) -> Rotor {
    var self_413: Rotor;
    var other_353: Translator;

    self_413 = self_412;
    other_353 = other_352;
    let _e4: Rotor = self_413;
    let _e6: Translator = other_353;
    return Rotor((_e4.g0_ * vec2<f32>(_e6.g0_.x)));
}

fn rotor_translator_scalar_product(self_414: Rotor, other_354: Translator) -> Scalar {
    var self_415: Rotor;
    var other_355: Translator;

    self_415 = self_414;
    other_355 = other_354;
    let _e4: Rotor = self_415;
    let _e7: Translator = other_355;
    return Scalar((_e4.g0_.x * _e7.g0_.x));
}

fn rotor_motor_add(self_416: Rotor, other_356: Motor) -> Motor {
    var self_417: Rotor;
    var other_357: Motor;

    self_417 = self_416;
    other_357 = other_356;
    let _e4: Rotor = self_417;
    let _e7: Rotor = self_417;
    let _e10: Rotor = self_417;
    let _e13: Rotor = self_417;
    let _e23: Motor = other_357;
    return Motor(((vec4<f32>(_e4.g0_.x, _e7.g0_.y, _e10.g0_.x, _e13.g0_.x) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) + _e23.g0_));
}

fn rotor_motor_sub(self_418: Rotor, other_358: Motor) -> Motor {
    var self_419: Rotor;
    var other_359: Motor;

    self_419 = self_418;
    other_359 = other_358;
    let _e4: Rotor = self_419;
    let _e7: Rotor = self_419;
    let _e10: Rotor = self_419;
    let _e13: Rotor = self_419;
    let _e23: Motor = other_359;
    return Motor(((vec4<f32>(_e4.g0_.x, _e7.g0_.y, _e10.g0_.x, _e13.g0_.x) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) - _e23.g0_));
}

fn rotor_motor_geometric_product(self_420: Rotor, other_360: Motor) -> Motor {
    var self_421: Rotor;
    var other_361: Motor;

    self_421 = self_420;
    other_361 = other_360;
    let _e4: Rotor = self_421;
    let _e8: Motor = other_361;
    let _e11: Rotor = self_421;
    let _e15: Motor = other_361;
    return Motor(((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.y) * _e15.g0_.yxwz) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))));
}

fn rotor_motor_outer_product(self_422: Rotor, other_362: Motor) -> Motor {
    var self_423: Rotor;
    var other_363: Motor;

    self_423 = self_422;
    other_363 = other_362;
    let _e4: Rotor = self_423;
    let _e8: Motor = other_363;
    let _e11: Rotor = self_423;
    let _e14: Rotor = self_423;
    let _e17: Rotor = self_423;
    let _e20: Rotor = self_423;
    let _e24: Motor = other_363;
    return Motor(((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.x, _e14.g0_.y, _e17.g0_.x, _e20.g0_.x) * vec4<f32>(_e24.g0_.x)) * vec4<f32>(0.0, 1.0, 0.0, 0.0))));
}

fn rotor_motor_inner_product(self_424: Rotor, other_364: Motor) -> Motor {
    var self_425: Rotor;
    var other_365: Motor;

    self_425 = self_424;
    other_365 = other_364;
    let _e4: Rotor = self_425;
    let _e8: Motor = other_365;
    let _e11: Rotor = self_425;
    let _e14: Rotor = self_425;
    let _e17: Rotor = self_425;
    let _e20: Rotor = self_425;
    let _e24: Motor = other_365;
    return Motor(((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.y, _e14.g0_.y, _e17.g0_.x, _e20.g0_.x) * _e24.g0_.yxxx) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))));
}

fn rotor_motor_left_contraction(self_426: Rotor, other_366: Motor) -> Motor {
    var self_427: Rotor;
    var other_367: Motor;

    self_427 = self_426;
    other_367 = other_366;
    let _e4: Rotor = self_427;
    let _e8: Motor = other_367;
    let _e11: Rotor = self_427;
    let _e14: Rotor = self_427;
    let _e17: Rotor = self_427;
    let _e20: Rotor = self_427;
    let _e24: Motor = other_367;
    return Motor(((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.y, _e14.g0_.x, _e17.g0_.x, _e20.g0_.x) * _e24.g0_.yxxx) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))));
}

fn rotor_motor_right_contraction(self_428: Rotor, other_368: Motor) -> Rotor {
    var self_429: Rotor;
    var other_369: Motor;

    self_429 = self_428;
    other_369 = other_368;
    let _e4: Rotor = self_429;
    let _e8: Motor = other_369;
    let _e11: Motor = other_369;
    let _e21: Rotor = self_429;
    let _e25: Motor = other_369;
    return Rotor((((vec2<f32>(_e4.g0_.y) * vec2<f32>(_e8.g0_.y, _e11.g0_.x)) * vec2<f32>(-(1.0), 1.0)) + ((vec2<f32>(_e21.g0_.x) * vec2<f32>(_e25.g0_.x)) * vec2<f32>(1.0, 0.0))));
}

fn rotor_motor_scalar_product(self_430: Rotor, other_370: Motor) -> Scalar {
    var self_431: Rotor;
    var other_371: Motor;

    self_431 = self_430;
    other_371 = other_370;
    let _e4: Rotor = self_431;
    let _e7: Motor = other_371;
    let _e11: Rotor = self_431;
    let _e14: Motor = other_371;
    return Scalar(((_e4.g0_.x * _e7.g0_.x) - (_e11.g0_.y * _e14.g0_.y)));
}

fn rotor_motor_dual_geometric_product(self_432: Rotor, other_372: MotorDual) -> MotorDual {
    var self_433: Rotor;
    var other_373: MotorDual;

    self_433 = self_432;
    other_373 = other_372;
    let _e4: Rotor = self_433;
    let _e8: MotorDual = other_373;
    let _e11: Rotor = self_433;
    let _e15: MotorDual = other_373;
    return MotorDual(((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.y) * _e15.g0_.yxwz) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0))));
}

fn rotor_motor_dual_regressive_product(self_434: Rotor, other_374: MotorDual) -> Rotor {
    var self_435: Rotor;
    var other_375: MotorDual;

    self_435 = self_434;
    other_375 = other_374;
    let _e4: Rotor = self_435;
    let _e8: MotorDual = other_375;
    let _e11: MotorDual = other_375;
    let _e16: Rotor = self_435;
    let _e20: MotorDual = other_375;
    return Rotor(((vec2<f32>(_e4.g0_.y) * vec2<f32>(_e8.g0_.y, _e11.g0_.x)) + ((vec2<f32>(_e16.g0_.x) * vec2<f32>(_e20.g0_.x)) * vec2<f32>(1.0, 0.0))));
}

fn rotor_motor_dual_outer_product(self_436: Rotor, other_376: MotorDual) -> MotorDual {
    var self_437: Rotor;
    var other_377: MotorDual;

    self_437 = self_436;
    other_377 = other_376;
    let _e4: Rotor = self_437;
    let _e8: MotorDual = other_377;
    let _e11: Rotor = self_437;
    let _e14: Rotor = self_437;
    let _e17: Rotor = self_437;
    let _e20: Rotor = self_437;
    let _e24: MotorDual = other_377;
    return MotorDual(((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.y, _e14.g0_.x, _e17.g0_.x, _e20.g0_.x) * _e24.g0_.yxxx) * vec4<f32>(1.0, 0.0, 0.0, 0.0))));
}

fn rotor_motor_dual_inner_product(self_438: Rotor, other_378: MotorDual) -> MotorDual {
    var self_439: Rotor;
    var other_379: MotorDual;

    self_439 = self_438;
    other_379 = other_378;
    let _e4: Rotor = self_439;
    let _e8: MotorDual = other_379;
    let _e11: Rotor = self_439;
    let _e14: Rotor = self_439;
    let _e17: Rotor = self_439;
    let _e20: Rotor = self_439;
    let _e24: MotorDual = other_379;
    return MotorDual(((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.x, _e14.g0_.y, _e17.g0_.y, _e20.g0_.y) * _e24.g0_.xxwz) * vec4<f32>(0.0, -(1.0), -(1.0), 1.0))));
}

fn rotor_motor_dual_left_contraction(self_440: Rotor, other_380: MotorDual) -> MotorDual {
    var self_441: Rotor;
    var other_381: MotorDual;

    self_441 = self_440;
    other_381 = other_380;
    let _e4: Rotor = self_441;
    let _e8: MotorDual = other_381;
    let _e11: Rotor = self_441;
    let _e14: Rotor = self_441;
    let _e17: Rotor = self_441;
    let _e20: Rotor = self_441;
    let _e24: MotorDual = other_381;
    return MotorDual(((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.x, _e14.g0_.y, _e17.g0_.x, _e20.g0_.x) * vec4<f32>(_e24.g0_.x)) * vec4<f32>(0.0, -(1.0), 0.0, 0.0))));
}

fn rotor_squared_magnitude(self_442: Rotor) -> Scalar {
    var self_443: Rotor;

    self_443 = self_442;
    let _e2: Rotor = self_443;
    let _e3: Rotor = self_443;
    let _e4: Rotor = rotor_reversal(_e3);
    let _e5: Scalar = rotor_rotor_scalar_product(_e2, _e4);
    return _e5;
}

fn rotor_magnitude(self_444: Rotor) -> Scalar {
    var self_445: Rotor;

    self_445 = self_444;
    let _e2: Rotor = self_445;
    let _e3: Scalar = rotor_squared_magnitude(_e2);
    return Scalar(sqrt(_e3.g0_));
}

fn rotor_scale(self_446: Rotor, other_382: f32) -> Rotor {
    var self_447: Rotor;
    var other_383: f32;

    self_447 = self_446;
    other_383 = other_382;
    let _e4: Rotor = self_447;
    let _e5: f32 = other_383;
    let _e7: Rotor = rotor_scalar_geometric_product(_e4, Scalar(_e5));
    return _e7;
}

fn rotor_signum(self_448: Rotor) -> Rotor {
    var self_449: Rotor;

    self_449 = self_448;
    let _e2: Rotor = self_449;
    let _e3: Rotor = self_449;
    let _e4: Scalar = rotor_magnitude(_e3);
    let _e9: Rotor = rotor_scalar_geometric_product(_e2, Scalar((1.0 / _e4.g0_)));
    return _e9;
}

fn rotor_inverse(self_450: Rotor) -> Rotor {
    var self_451: Rotor;

    self_451 = self_450;
    let _e2: Rotor = self_451;
    let _e3: Rotor = rotor_reversal(_e2);
    let _e4: Rotor = self_451;
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

fn point_neg(self_452: Point) -> Point {
    var self_453: Point;

    self_453 = self_452;
    let _e2: Point = self_453;
    return Point((_e2.g0_ * vec3<f32>(-(1.0))));
}

fn point_automorphism(self_454: Point) -> Point {
    var self_455: Point;

    self_455 = self_454;
    let _e2: Point = self_455;
    return Point(_e2.g0_);
}

fn point_reversal(self_456: Point) -> Point {
    var self_457: Point;

    self_457 = self_456;
    let _e2: Point = self_457;
    return Point((_e2.g0_ * vec3<f32>(-(1.0))));
}

fn point_conjugation(self_458: Point) -> Point {
    var self_459: Point;

    self_459 = self_458;
    let _e2: Point = self_459;
    return Point((_e2.g0_ * vec3<f32>(-(1.0))));
}

fn point_dual(self_460: Point) -> Plane {
    var self_461: Point;

    self_461 = self_460;
    let _e2: Point = self_461;
    return Plane(_e2.g0_);
}

fn point_scalar_add(self_462: Point, other_384: Scalar) -> Motor {
    var self_463: Point;
    var other_385: Scalar;

    self_463 = self_462;
    other_385 = other_384;
    let _e4: Point = self_463;
    let _e7: Point = self_463;
    let _e10: Point = self_463;
    let _e13: Point = self_463;
    let _e23: Scalar = other_385;
    return Motor(((vec4<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.y, _e13.g0_.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0)) + (vec4<f32>(_e23.g0_) * vec4<f32>(1.0, 0.0, 0.0, 0.0))));
}

fn point_scalar_sub(self_464: Point, other_386: Scalar) -> Motor {
    var self_465: Point;
    var other_387: Scalar;

    self_465 = self_464;
    other_387 = other_386;
    let _e4: Point = self_465;
    let _e7: Point = self_465;
    let _e10: Point = self_465;
    let _e13: Point = self_465;
    let _e23: Scalar = other_387;
    return Motor(((vec4<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.y, _e13.g0_.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0)) - (vec4<f32>(_e23.g0_) * vec4<f32>(1.0, 0.0, 0.0, 0.0))));
}

fn point_scalar_geometric_product(self_466: Point, other_388: Scalar) -> Point {
    var self_467: Point;
    var other_389: Scalar;

    self_467 = self_466;
    other_389 = other_388;
    let _e4: Point = self_467;
    let _e6: Scalar = other_389;
    return Point((_e4.g0_ * vec3<f32>(_e6.g0_)));
}

fn point_scalar_outer_product(self_468: Point, other_390: Scalar) -> Point {
    var self_469: Point;
    var other_391: Scalar;

    self_469 = self_468;
    other_391 = other_390;
    let _e4: Point = self_469;
    let _e6: Scalar = other_391;
    return Point((_e4.g0_ * vec3<f32>(_e6.g0_)));
}

fn point_scalar_inner_product(self_470: Point, other_392: Scalar) -> Point {
    var self_471: Point;
    var other_393: Scalar;

    self_471 = self_470;
    other_393 = other_392;
    let _e4: Point = self_471;
    let _e6: Scalar = other_393;
    return Point((_e4.g0_ * vec3<f32>(_e6.g0_)));
}

fn point_scalar_right_contraction(self_472: Point, other_394: Scalar) -> Point {
    var self_473: Point;
    var other_395: Scalar;

    self_473 = self_472;
    other_395 = other_394;
    let _e4: Point = self_473;
    let _e6: Scalar = other_395;
    return Point((_e4.g0_ * vec3<f32>(_e6.g0_)));
}

fn point_multi_vector_add(self_474: Point, other_396: MultiVector) -> MultiVector {
    var self_475: Point;
    var other_397: MultiVector;

    self_475 = self_474;
    other_397 = other_396;
    let _e4: Point = self_475;
    let _e14: MultiVector = other_397;
    let _e17: Point = self_475;
    let _e20: Point = self_475;
    let _e23: Point = self_475;
    let _e26: Point = self_475;
    let _e36: MultiVector = other_397;
    return MultiVector(((vec4<f32>(_e4.g0_.x) * vec4<f32>(0.0, 1.0, 0.0, 0.0)) + _e14.g0_), ((vec4<f32>(_e17.g0_.x, _e20.g0_.x, _e23.g0_.y, _e26.g0_.z) * vec4<f32>(0.0, 0.0, 1.0, 1.0)) + _e36.g1_));
}

fn point_multi_vector_sub(self_476: Point, other_398: MultiVector) -> MultiVector {
    var self_477: Point;
    var other_399: MultiVector;

    self_477 = self_476;
    other_399 = other_398;
    let _e4: Point = self_477;
    let _e14: MultiVector = other_399;
    let _e17: Point = self_477;
    let _e20: Point = self_477;
    let _e23: Point = self_477;
    let _e26: Point = self_477;
    let _e36: MultiVector = other_399;
    return MultiVector(((vec4<f32>(_e4.g0_.x) * vec4<f32>(0.0, 1.0, 0.0, 0.0)) - _e14.g0_), ((vec4<f32>(_e17.g0_.x, _e20.g0_.x, _e23.g0_.y, _e26.g0_.z) * vec4<f32>(0.0, 0.0, 1.0, 1.0)) - _e36.g1_));
}

fn point_multi_vector_geometric_product(self_478: Point, other_400: MultiVector) -> MultiVector {
    var self_479: Point;
    var other_401: MultiVector;

    self_479 = self_478;
    other_401 = other_400;
    let _e4: Point = self_479;
    let _e8: MultiVector = other_401;
    let _e20: Point = self_479;
    let _e24: MultiVector = other_401;
    let _e38: Point = self_479;
    let _e42: MultiVector = other_401;
    let _e56: Point = self_479;
    let _e60: MultiVector = other_401;
    let _e72: Point = self_479;
    let _e76: MultiVector = other_401;
    let _e88: Point = self_479;
    let _e92: MultiVector = other_401;
    return MultiVector(((((vec4<f32>(_e4.g0_.x) * _e8.g0_.yxwz) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0))) + ((vec4<f32>(_e20.g0_.y) * _e24.g1_.zwxy) * vec4<f32>(-(1.0), 1.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e38.g0_.z) * _e42.g1_.wzyx) * vec4<f32>(-(1.0), -(1.0), -(1.0), 1.0))), ((((vec4<f32>(_e56.g0_.x) * _e60.g1_.yxwz) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0)) + ((vec4<f32>(_e72.g0_.y) * _e76.g0_.zwxy) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e88.g0_.z) * _e92.g0_.wzyx) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0))));
}

fn point_multi_vector_scalar_product(self_480: Point, other_402: MultiVector) -> Scalar {
    var self_481: Point;
    var other_403: MultiVector;

    self_481 = self_480;
    other_403 = other_402;
    let _e5: Point = self_481;
    let _e8: MultiVector = other_403;
    let _e13: Point = self_481;
    let _e16: MultiVector = other_403;
    let _e21: Point = self_481;
    let _e24: MultiVector = other_403;
    return Scalar((((0.0 - (_e5.g0_.x * _e8.g0_.y)) - (_e13.g0_.y * _e16.g1_.z)) - (_e21.g0_.z * _e24.g1_.w)));
}

fn point_rotor_add(self_482: Point, other_404: Rotor) -> Motor {
    var self_483: Point;
    var other_405: Rotor;

    self_483 = self_482;
    other_405 = other_404;
    let _e4: Point = self_483;
    let _e7: Point = self_483;
    let _e10: Point = self_483;
    let _e13: Point = self_483;
    let _e23: Rotor = other_405;
    let _e26: Rotor = other_405;
    let _e29: Rotor = other_405;
    let _e32: Rotor = other_405;
    return Motor(((vec4<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.y, _e13.g0_.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0)) + (vec4<f32>(_e23.g0_.x, _e26.g0_.y, _e29.g0_.x, _e32.g0_.x) * vec4<f32>(1.0, 1.0, 0.0, 0.0))));
}

fn point_rotor_sub(self_484: Point, other_406: Rotor) -> Motor {
    var self_485: Point;
    var other_407: Rotor;

    self_485 = self_484;
    other_407 = other_406;
    let _e4: Point = self_485;
    let _e7: Point = self_485;
    let _e10: Point = self_485;
    let _e13: Point = self_485;
    let _e23: Rotor = other_407;
    let _e26: Rotor = other_407;
    let _e29: Rotor = other_407;
    let _e32: Rotor = other_407;
    return Motor(((vec4<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.y, _e13.g0_.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0)) - (vec4<f32>(_e23.g0_.x, _e26.g0_.y, _e29.g0_.x, _e32.g0_.x) * vec4<f32>(1.0, 1.0, 0.0, 0.0))));
}

fn point_rotor_geometric_product(self_486: Point, other_408: Rotor) -> Motor {
    var self_487: Point;
    var other_409: Rotor;

    self_487 = self_486;
    other_409 = other_408;
    let _e4: Point = self_487;
    let _e8: Rotor = other_409;
    let _e11: Rotor = other_409;
    let _e14: Rotor = other_409;
    let _e17: Rotor = other_409;
    let _e28: Point = self_487;
    let _e31: Point = self_487;
    let _e34: Point = self_487;
    let _e37: Point = self_487;
    let _e41: Rotor = other_409;
    let _e44: Rotor = other_409;
    let _e47: Rotor = other_409;
    let _e50: Rotor = other_409;
    return Motor((((vec4<f32>(_e4.g0_.z) * vec4<f32>(_e8.g0_.y, _e11.g0_.y, _e14.g0_.y, _e17.g0_.x)) * vec4<f32>(0.0, 0.0, 1.0, 1.0)) + ((vec4<f32>(_e28.g0_.x, _e31.g0_.x, _e34.g0_.y, _e37.g0_.y) * vec4<f32>(_e41.g0_.y, _e44.g0_.x, _e47.g0_.x, _e50.g0_.y)) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))));
}

fn point_rotor_outer_product(self_488: Point, other_410: Rotor) -> Point {
    var self_489: Point;
    var other_411: Rotor;

    self_489 = self_488;
    other_411 = other_410;
    let _e4: Point = self_489;
    let _e6: Rotor = other_411;
    return Point((_e4.g0_ * vec3<f32>(_e6.g0_.x)));
}

fn point_rotor_inner_product(self_490: Point, other_412: Rotor) -> Motor {
    var self_491: Point;
    var other_413: Rotor;

    self_491 = self_490;
    other_413 = other_412;
    let _e4: Point = self_491;
    let _e7: Point = self_491;
    let _e10: Point = self_491;
    let _e13: Point = self_491;
    let _e17: Rotor = other_413;
    let _e20: Rotor = other_413;
    let _e23: Rotor = other_413;
    let _e26: Rotor = other_413;
    return Motor(((vec4<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.y, _e13.g0_.z) * vec4<f32>(_e17.g0_.y, _e20.g0_.x, _e23.g0_.x, _e26.g0_.x)) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0)));
}

fn point_rotor_left_contraction(self_492: Point, other_414: Rotor) -> Scalar {
    var self_493: Point;
    var other_415: Rotor;

    self_493 = self_492;
    other_415 = other_414;
    let _e5: Point = self_493;
    let _e8: Rotor = other_415;
    return Scalar((0.0 - (_e5.g0_.x * _e8.g0_.y)));
}

fn point_rotor_right_contraction(self_494: Point, other_416: Rotor) -> Motor {
    var self_495: Point;
    var other_417: Rotor;

    self_495 = self_494;
    other_417 = other_416;
    let _e4: Point = self_495;
    let _e7: Point = self_495;
    let _e10: Point = self_495;
    let _e13: Point = self_495;
    let _e17: Rotor = other_417;
    let _e20: Rotor = other_417;
    let _e23: Rotor = other_417;
    let _e26: Rotor = other_417;
    return Motor(((vec4<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.y, _e13.g0_.z) * vec4<f32>(_e17.g0_.y, _e20.g0_.x, _e23.g0_.x, _e26.g0_.x)) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0)));
}

fn point_rotor_scalar_product(self_496: Point, other_418: Rotor) -> Scalar {
    var self_497: Point;
    var other_419: Rotor;

    self_497 = self_496;
    other_419 = other_418;
    let _e5: Point = self_497;
    let _e8: Rotor = other_419;
    return Scalar((0.0 - (_e5.g0_.x * _e8.g0_.y)));
}

fn point_point_add(self_498: Point, other_420: Point) -> Point {
    var self_499: Point;
    var other_421: Point;

    self_499 = self_498;
    other_421 = other_420;
    let _e4: Point = self_499;
    let _e6: Point = other_421;
    return Point((_e4.g0_ + _e6.g0_));
}

fn point_point_sub(self_500: Point, other_422: Point) -> Point {
    var self_501: Point;
    var other_423: Point;

    self_501 = self_500;
    other_423 = other_422;
    let _e4: Point = self_501;
    let _e6: Point = other_423;
    return Point((_e4.g0_ - _e6.g0_));
}

fn point_point_mul(self_502: Point, other_424: Point) -> Point {
    var self_503: Point;
    var other_425: Point;

    self_503 = self_502;
    other_425 = other_424;
    let _e4: Point = self_503;
    let _e6: Point = other_425;
    return Point((_e4.g0_ * _e6.g0_));
}

fn point_point_div(self_504: Point, other_426: Point) -> Point {
    var self_505: Point;
    var other_427: Point;

    self_505 = self_504;
    other_427 = other_426;
    let _e4: Point = self_505;
    let _e7: Point = self_505;
    let _e10: Point = self_505;
    let _e19: Point = other_427;
    let _e22: Point = other_427;
    let _e25: Point = other_427;
    return Point((((vec3<f32>(_e4.g0_.x, _e7.g0_.y, _e10.g0_.z) * vec3<f32>(1.0, 1.0, 1.0)) / vec3<f32>(_e19.g0_.x, _e22.g0_.y, _e25.g0_.z)) * vec3<f32>(1.0, 1.0, 1.0)));
}

fn point_point_geometric_product(self_506: Point, other_428: Point) -> Motor {
    var self_507: Point;
    var other_429: Point;

    self_507 = self_506;
    other_429 = other_428;
    let _e4: Point = self_507;
    let _e8: Point = other_429;
    let _e11: Point = other_429;
    let _e14: Point = other_429;
    let _e17: Point = other_429;
    let _e30: Point = self_507;
    let _e34: Point = other_429;
    let _e37: Point = other_429;
    let _e40: Point = other_429;
    let _e43: Point = other_429;
    let _e57: Point = self_507;
    let _e61: Point = other_429;
    let _e64: Point = other_429;
    let _e67: Point = other_429;
    let _e70: Point = other_429;
    return Motor(((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y, _e11.g0_.z, _e14.g0_.y, _e17.g0_.x)) * vec4<f32>(-(1.0), 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e30.g0_.z) * vec4<f32>(_e34.g0_.z, _e37.g0_.y, _e40.g0_.x, _e43.g0_.z)) * vec4<f32>(-(1.0), -(1.0), 1.0, 0.0))) + ((vec4<f32>(_e57.g0_.x) * vec4<f32>(_e61.g0_.x, _e64.g0_.x, _e67.g0_.z, _e70.g0_.y)) * vec4<f32>(-(1.0), 0.0, -(1.0), 1.0))));
}

fn point_point_regressive_product(self_508: Point, other_430: Point) -> Plane {
    var self_509: Point;
    var other_431: Point;

    self_509 = self_508;
    other_431 = other_430;
    let _e4: Point = self_509;
    let _e8: Point = other_431;
    let _e18: Point = self_509;
    let _e22: Point = other_431;
    let _e33: Point = self_509;
    let _e37: Point = other_431;
    return Plane(((((vec3<f32>(_e4.g0_.y) * _e8.g0_.zzx) * vec3<f32>(-(1.0), 0.0, 1.0)) + ((vec3<f32>(_e18.g0_.z) * _e22.g0_.yxy) * vec3<f32>(1.0, -(1.0), 0.0))) + ((vec3<f32>(_e33.g0_.x) * _e37.g0_.xzy) * vec3<f32>(0.0, 1.0, -(1.0)))));
}

fn point_point_inner_product(self_510: Point, other_432: Point) -> Scalar {
    var self_511: Point;
    var other_433: Point;

    self_511 = self_510;
    other_433 = other_432;
    let _e5: Point = self_511;
    let _e8: Point = other_433;
    let _e13: Point = self_511;
    let _e16: Point = other_433;
    let _e21: Point = self_511;
    let _e24: Point = other_433;
    return Scalar((((0.0 - (_e5.g0_.x * _e8.g0_.x)) - (_e13.g0_.y * _e16.g0_.y)) - (_e21.g0_.z * _e24.g0_.z)));
}

fn point_point_left_contraction(self_512: Point, other_434: Point) -> Scalar {
    var self_513: Point;
    var other_435: Point;

    self_513 = self_512;
    other_435 = other_434;
    let _e5: Point = self_513;
    let _e8: Point = other_435;
    let _e13: Point = self_513;
    let _e16: Point = other_435;
    let _e21: Point = self_513;
    let _e24: Point = other_435;
    return Scalar((((0.0 - (_e5.g0_.x * _e8.g0_.x)) - (_e13.g0_.y * _e16.g0_.y)) - (_e21.g0_.z * _e24.g0_.z)));
}

fn point_point_right_contraction(self_514: Point, other_436: Point) -> Scalar {
    var self_515: Point;
    var other_437: Point;

    self_515 = self_514;
    other_437 = other_436;
    let _e5: Point = self_515;
    let _e8: Point = other_437;
    let _e13: Point = self_515;
    let _e16: Point = other_437;
    let _e21: Point = self_515;
    let _e24: Point = other_437;
    return Scalar((((0.0 - (_e5.g0_.x * _e8.g0_.x)) - (_e13.g0_.y * _e16.g0_.y)) - (_e21.g0_.z * _e24.g0_.z)));
}

fn point_point_scalar_product(self_516: Point, other_438: Point) -> Scalar {
    var self_517: Point;
    var other_439: Point;

    self_517 = self_516;
    other_439 = other_438;
    let _e5: Point = self_517;
    let _e8: Point = other_439;
    let _e13: Point = self_517;
    let _e16: Point = other_439;
    let _e21: Point = self_517;
    let _e24: Point = other_439;
    return Scalar((((0.0 - (_e5.g0_.x * _e8.g0_.x)) - (_e13.g0_.y * _e16.g0_.y)) - (_e21.g0_.z * _e24.g0_.z)));
}

fn point_ideal_point_into(self_518: Point) -> IdealPoint {
    var self_519: Point;

    self_519 = self_518;
    let _e2: Point = self_519;
    let _e5: Point = self_519;
    return IdealPoint(vec2<f32>(_e2.g0_.y, _e5.g0_.z));
}

fn point_ideal_point_add(self_520: Point, other_440: IdealPoint) -> Point {
    var self_521: Point;
    var other_441: IdealPoint;

    self_521 = self_520;
    other_441 = other_440;
    let _e4: Point = self_521;
    let _e6: IdealPoint = other_441;
    let _e9: IdealPoint = other_441;
    let _e12: IdealPoint = other_441;
    return Point((_e4.g0_ + (vec3<f32>(_e6.g0_.x, _e9.g0_.x, _e12.g0_.y) * vec3<f32>(0.0, 1.0, 1.0))));
}

fn point_ideal_point_sub(self_522: Point, other_442: IdealPoint) -> Point {
    var self_523: Point;
    var other_443: IdealPoint;

    self_523 = self_522;
    other_443 = other_442;
    let _e4: Point = self_523;
    let _e6: IdealPoint = other_443;
    let _e9: IdealPoint = other_443;
    let _e12: IdealPoint = other_443;
    return Point((_e4.g0_ - (vec3<f32>(_e6.g0_.x, _e9.g0_.x, _e12.g0_.y) * vec3<f32>(0.0, 1.0, 1.0))));
}

fn point_ideal_point_geometric_product(self_524: Point, other_444: IdealPoint) -> Motor {
    var self_525: Point;
    var other_445: IdealPoint;

    self_525 = self_524;
    other_445 = other_444;
    let _e4: Point = self_525;
    let _e8: IdealPoint = other_445;
    let _e11: IdealPoint = other_445;
    let _e14: IdealPoint = other_445;
    let _e17: IdealPoint = other_445;
    let _e30: Point = self_525;
    let _e33: Point = self_525;
    let _e36: Point = self_525;
    let _e39: Point = self_525;
    let _e43: IdealPoint = other_445;
    let _e46: IdealPoint = other_445;
    let _e49: IdealPoint = other_445;
    let _e52: IdealPoint = other_445;
    return Motor((((vec4<f32>(_e4.g0_.z) * vec4<f32>(_e8.g0_.y, _e11.g0_.x, _e14.g0_.y, _e17.g0_.y)) * vec4<f32>(-(1.0), -(1.0), 0.0, 0.0)) + ((vec4<f32>(_e30.g0_.y, _e33.g0_.y, _e36.g0_.x, _e39.g0_.x) * vec4<f32>(_e43.g0_.x, _e46.g0_.y, _e49.g0_.y, _e52.g0_.x)) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))));
}

fn point_ideal_point_regressive_product(self_526: Point, other_446: IdealPoint) -> Plane {
    var self_527: Point;
    var other_447: IdealPoint;

    self_527 = self_526;
    other_447 = other_446;
    let _e4: Point = self_527;
    let _e8: IdealPoint = other_447;
    let _e18: Point = self_527;
    let _e21: IdealPoint = other_447;
    let _e24: IdealPoint = other_447;
    let _e27: IdealPoint = other_447;
    return Plane((((vec3<f32>(_e4.g0_.z) * vec3<f32>(_e8.g0_.x)) * vec3<f32>(1.0, 0.0, 0.0)) + ((_e18.g0_.yxx * vec3<f32>(_e21.g0_.y, _e24.g0_.y, _e27.g0_.x)) * vec3<f32>(-(1.0), 1.0, -(1.0)))));
}

fn point_ideal_point_inner_product(self_528: Point, other_448: IdealPoint) -> Scalar {
    var self_529: Point;
    var other_449: IdealPoint;

    self_529 = self_528;
    other_449 = other_448;
    let _e5: Point = self_529;
    let _e8: IdealPoint = other_449;
    let _e13: Point = self_529;
    let _e16: IdealPoint = other_449;
    return Scalar(((0.0 - (_e5.g0_.y * _e8.g0_.x)) - (_e13.g0_.z * _e16.g0_.y)));
}

fn point_ideal_point_left_contraction(self_530: Point, other_450: IdealPoint) -> Scalar {
    var self_531: Point;
    var other_451: IdealPoint;

    self_531 = self_530;
    other_451 = other_450;
    let _e5: Point = self_531;
    let _e8: IdealPoint = other_451;
    let _e13: Point = self_531;
    let _e16: IdealPoint = other_451;
    return Scalar(((0.0 - (_e5.g0_.y * _e8.g0_.x)) - (_e13.g0_.z * _e16.g0_.y)));
}

fn point_ideal_point_right_contraction(self_532: Point, other_452: IdealPoint) -> Scalar {
    var self_533: Point;
    var other_453: IdealPoint;

    self_533 = self_532;
    other_453 = other_452;
    let _e5: Point = self_533;
    let _e8: IdealPoint = other_453;
    let _e13: Point = self_533;
    let _e16: IdealPoint = other_453;
    return Scalar(((0.0 - (_e5.g0_.y * _e8.g0_.x)) - (_e13.g0_.z * _e16.g0_.y)));
}

fn point_ideal_point_scalar_product(self_534: Point, other_454: IdealPoint) -> Scalar {
    var self_535: Point;
    var other_455: IdealPoint;

    self_535 = self_534;
    other_455 = other_454;
    let _e5: Point = self_535;
    let _e8: IdealPoint = other_455;
    let _e13: Point = self_535;
    let _e16: IdealPoint = other_455;
    return Scalar(((0.0 - (_e5.g0_.y * _e8.g0_.x)) - (_e13.g0_.z * _e16.g0_.y)));
}

fn point_plane_geometric_product(self_536: Point, other_456: Plane) -> MotorDual {
    var self_537: Point;
    var other_457: Plane;

    self_537 = self_536;
    other_457 = other_456;
    let _e4: Point = self_537;
    let _e8: Plane = other_457;
    let _e11: Plane = other_457;
    let _e14: Plane = other_457;
    let _e17: Plane = other_457;
    let _e29: Point = self_537;
    let _e33: Plane = other_457;
    let _e36: Plane = other_457;
    let _e39: Plane = other_457;
    let _e42: Plane = other_457;
    let _e55: Point = self_537;
    let _e59: Plane = other_457;
    let _e62: Plane = other_457;
    let _e65: Plane = other_457;
    let _e68: Plane = other_457;
    return MotorDual(((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y, _e11.g0_.z, _e14.g0_.y, _e17.g0_.x)) * vec4<f32>(1.0, 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e29.g0_.z) * vec4<f32>(_e33.g0_.z, _e36.g0_.y, _e39.g0_.x, _e42.g0_.z)) * vec4<f32>(1.0, -(1.0), 1.0, 0.0))) + ((vec4<f32>(_e55.g0_.x) * vec4<f32>(_e59.g0_.x, _e62.g0_.x, _e65.g0_.z, _e68.g0_.y)) * vec4<f32>(1.0, 0.0, -(1.0), 1.0))));
}

fn point_plane_regressive_product(self_538: Point, other_458: Plane) -> Scalar {
    var self_539: Point;
    var other_459: Plane;

    self_539 = self_538;
    other_459 = other_458;
    let _e4: Point = self_539;
    let _e7: Plane = other_459;
    let _e11: Point = self_539;
    let _e14: Plane = other_459;
    let _e19: Point = self_539;
    let _e22: Plane = other_459;
    return Scalar((((_e4.g0_.x * _e7.g0_.x) + (_e11.g0_.y * _e14.g0_.y)) + (_e19.g0_.z * _e22.g0_.z)));
}

fn point_plane_inner_product(self_540: Point, other_460: Plane) -> Plane {
    var self_541: Point;
    var other_461: Plane;

    self_541 = self_540;
    other_461 = other_460;
    let _e4: Point = self_541;
    let _e8: Plane = other_461;
    let _e18: Point = self_541;
    let _e22: Plane = other_461;
    let _e33: Point = self_541;
    let _e37: Plane = other_461;
    return Plane(((((vec3<f32>(_e4.g0_.y) * _e8.g0_.zzx) * vec3<f32>(1.0, 0.0, -(1.0))) + ((vec3<f32>(_e18.g0_.z) * _e22.g0_.yxy) * vec3<f32>(-(1.0), 1.0, 0.0))) + ((vec3<f32>(_e33.g0_.x) * _e37.g0_.xzy) * vec3<f32>(0.0, -(1.0), 1.0))));
}

fn point_plane_right_contraction(self_542: Point, other_462: Plane) -> Plane {
    var self_543: Point;
    var other_463: Plane;

    self_543 = self_542;
    other_463 = other_462;
    let _e4: Point = self_543;
    let _e8: Plane = other_463;
    let _e18: Point = self_543;
    let _e22: Plane = other_463;
    let _e33: Point = self_543;
    let _e37: Plane = other_463;
    return Plane(((((vec3<f32>(_e4.g0_.y) * _e8.g0_.zzx) * vec3<f32>(1.0, 0.0, -(1.0))) + ((vec3<f32>(_e18.g0_.z) * _e22.g0_.yxy) * vec3<f32>(-(1.0), 1.0, 0.0))) + ((vec3<f32>(_e33.g0_.x) * _e37.g0_.xzy) * vec3<f32>(0.0, -(1.0), 1.0))));
}

fn point_translator_add(self_544: Point, other_464: Translator) -> Motor {
    var self_545: Point;
    var other_465: Translator;

    self_545 = self_544;
    other_465 = other_464;
    let _e4: Point = self_545;
    let _e7: Point = self_545;
    let _e10: Point = self_545;
    let _e13: Point = self_545;
    let _e23: Translator = other_465;
    let _e26: Translator = other_465;
    let _e29: Translator = other_465;
    let _e32: Translator = other_465;
    return Motor(((vec4<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.y, _e13.g0_.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0)) + (vec4<f32>(_e23.g0_.x, _e26.g0_.x, _e29.g0_.y, _e32.g0_.z) * vec4<f32>(1.0, 0.0, 1.0, 1.0))));
}

fn point_translator_sub(self_546: Point, other_466: Translator) -> Motor {
    var self_547: Point;
    var other_467: Translator;

    self_547 = self_546;
    other_467 = other_466;
    let _e4: Point = self_547;
    let _e7: Point = self_547;
    let _e10: Point = self_547;
    let _e13: Point = self_547;
    let _e23: Translator = other_467;
    let _e26: Translator = other_467;
    let _e29: Translator = other_467;
    let _e32: Translator = other_467;
    return Motor(((vec4<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.y, _e13.g0_.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0)) - (vec4<f32>(_e23.g0_.x, _e26.g0_.x, _e29.g0_.y, _e32.g0_.z) * vec4<f32>(1.0, 0.0, 1.0, 1.0))));
}

fn point_translator_geometric_product(self_548: Point, other_468: Translator) -> Motor {
    var self_549: Point;
    var other_469: Translator;

    self_549 = self_548;
    other_469 = other_468;
    let _e4: Point = self_549;
    let _e8: Translator = other_469;
    let _e11: Translator = other_469;
    let _e14: Translator = other_469;
    let _e17: Translator = other_469;
    let _e29: Point = self_549;
    let _e33: Translator = other_469;
    let _e36: Translator = other_469;
    let _e39: Translator = other_469;
    let _e42: Translator = other_469;
    let _e56: Point = self_549;
    let _e60: Translator = other_469;
    let _e63: Translator = other_469;
    let _e66: Translator = other_469;
    let _e69: Translator = other_469;
    return Motor(((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y, _e11.g0_.z, _e14.g0_.x, _e17.g0_.y)) * vec4<f32>(-(1.0), 1.0, 1.0, 0.0)) + ((vec4<f32>(_e29.g0_.z) * vec4<f32>(_e33.g0_.z, _e36.g0_.y, _e39.g0_.z, _e42.g0_.x)) * vec4<f32>(-(1.0), -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e56.g0_.x) * vec4<f32>(_e60.g0_.x, _e63.g0_.x, _e66.g0_.z, _e69.g0_.y)) * vec4<f32>(0.0, 1.0, -(1.0), 1.0))));
}

fn point_translator_regressive_product(self_550: Point, other_470: Translator) -> Plane {
    var self_551: Point;
    var other_471: Translator;

    self_551 = self_550;
    other_471 = other_470;
    let _e4: Point = self_551;
    let _e8: Translator = other_471;
    let _e18: Point = self_551;
    let _e21: Translator = other_471;
    return Plane((((vec3<f32>(_e4.g0_.z) * vec3<f32>(_e8.g0_.y)) * vec3<f32>(1.0, 0.0, 0.0)) + ((_e18.g0_.yxx * _e21.g0_.zzy) * vec3<f32>(-(1.0), 1.0, -(1.0)))));
}

fn point_translator_outer_product(self_552: Point, other_472: Translator) -> Point {
    var self_553: Point;
    var other_473: Translator;

    self_553 = self_552;
    other_473 = other_472;
    let _e4: Point = self_553;
    let _e6: Translator = other_473;
    return Point((_e4.g0_ * vec3<f32>(_e6.g0_.x)));
}

fn point_translator_inner_product(self_554: Point, other_474: Translator) -> Motor {
    var self_555: Point;
    var other_475: Translator;

    self_555 = self_554;
    other_475 = other_474;
    let _e4: Point = self_555;
    let _e8: Translator = other_475;
    let _e11: Translator = other_475;
    let _e14: Translator = other_475;
    let _e17: Translator = other_475;
    let _e29: Point = self_555;
    let _e32: Point = self_555;
    let _e35: Point = self_555;
    let _e38: Point = self_555;
    let _e42: Translator = other_475;
    let _e45: Translator = other_475;
    let _e48: Translator = other_475;
    let _e51: Translator = other_475;
    return Motor((((vec4<f32>(_e4.g0_.z) * vec4<f32>(_e8.g0_.z, _e11.g0_.z, _e14.g0_.z, _e17.g0_.x)) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0)) + ((vec4<f32>(_e29.g0_.y, _e32.g0_.x, _e35.g0_.y, _e38.g0_.x) * vec4<f32>(_e42.g0_.y, _e45.g0_.x, _e48.g0_.x, _e51.g0_.x)) * vec4<f32>(-(1.0), 1.0, 1.0, 0.0))));
}

fn point_translator_left_contraction(self_556: Point, other_476: Translator) -> Scalar {
    var self_557: Point;
    var other_477: Translator;

    self_557 = self_556;
    other_477 = other_476;
    let _e5: Point = self_557;
    let _e8: Translator = other_477;
    let _e13: Point = self_557;
    let _e16: Translator = other_477;
    return Scalar(((0.0 - (_e5.g0_.y * _e8.g0_.y)) - (_e13.g0_.z * _e16.g0_.z)));
}

fn point_translator_right_contraction(self_558: Point, other_478: Translator) -> Motor {
    var self_559: Point;
    var other_479: Translator;

    self_559 = self_558;
    other_479 = other_478;
    let _e4: Point = self_559;
    let _e8: Translator = other_479;
    let _e11: Translator = other_479;
    let _e14: Translator = other_479;
    let _e17: Translator = other_479;
    let _e29: Point = self_559;
    let _e32: Point = self_559;
    let _e35: Point = self_559;
    let _e38: Point = self_559;
    let _e42: Translator = other_479;
    let _e45: Translator = other_479;
    let _e48: Translator = other_479;
    let _e51: Translator = other_479;
    return Motor((((vec4<f32>(_e4.g0_.z) * vec4<f32>(_e8.g0_.z, _e11.g0_.z, _e14.g0_.z, _e17.g0_.x)) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0)) + ((vec4<f32>(_e29.g0_.y, _e32.g0_.x, _e35.g0_.y, _e38.g0_.x) * vec4<f32>(_e42.g0_.y, _e45.g0_.x, _e48.g0_.x, _e51.g0_.x)) * vec4<f32>(-(1.0), 1.0, 1.0, 0.0))));
}

fn point_translator_scalar_product(self_560: Point, other_480: Translator) -> Scalar {
    var self_561: Point;
    var other_481: Translator;

    self_561 = self_560;
    other_481 = other_480;
    let _e5: Point = self_561;
    let _e8: Translator = other_481;
    let _e13: Point = self_561;
    let _e16: Translator = other_481;
    return Scalar(((0.0 - (_e5.g0_.y * _e8.g0_.y)) - (_e13.g0_.z * _e16.g0_.z)));
}

fn point_motor_add(self_562: Point, other_482: Motor) -> Motor {
    var self_563: Point;
    var other_483: Motor;

    self_563 = self_562;
    other_483 = other_482;
    let _e4: Point = self_563;
    let _e7: Point = self_563;
    let _e10: Point = self_563;
    let _e13: Point = self_563;
    let _e23: Motor = other_483;
    return Motor(((vec4<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.y, _e13.g0_.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0)) + _e23.g0_));
}

fn point_motor_sub(self_564: Point, other_484: Motor) -> Motor {
    var self_565: Point;
    var other_485: Motor;

    self_565 = self_564;
    other_485 = other_484;
    let _e4: Point = self_565;
    let _e7: Point = self_565;
    let _e10: Point = self_565;
    let _e13: Point = self_565;
    let _e23: Motor = other_485;
    return Motor(((vec4<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.y, _e13.g0_.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0)) - _e23.g0_));
}

fn point_motor_geometric_product(self_566: Point, other_486: Motor) -> Motor {
    var self_567: Point;
    var other_487: Motor;

    self_567 = self_566;
    other_487 = other_486;
    let _e4: Point = self_567;
    let _e8: Motor = other_487;
    let _e20: Point = self_567;
    let _e24: Motor = other_487;
    let _e37: Point = self_567;
    let _e41: Motor = other_487;
    return Motor(((((vec4<f32>(_e4.g0_.x) * _e8.g0_.yxwz) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0)) + ((vec4<f32>(_e20.g0_.y) * _e24.g0_.zwxy) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e37.g0_.z) * _e41.g0_.wzyx) * vec4<f32>(-(1.0), -(1.0), 1.0, 1.0))));
}

fn point_motor_regressive_product(self_568: Point, other_488: Motor) -> Plane {
    var self_569: Point;
    var other_489: Motor;

    self_569 = self_568;
    other_489 = other_488;
    let _e4: Point = self_569;
    let _e8: Motor = other_489;
    let _e11: Motor = other_489;
    let _e14: Motor = other_489;
    let _e25: Point = self_569;
    let _e29: Motor = other_489;
    let _e32: Motor = other_489;
    let _e35: Motor = other_489;
    let _e47: Point = self_569;
    let _e51: Motor = other_489;
    let _e54: Motor = other_489;
    let _e57: Motor = other_489;
    return Plane(((((vec3<f32>(_e4.g0_.y) * vec3<f32>(_e8.g0_.w, _e11.g0_.w, _e14.g0_.y)) * vec3<f32>(-(1.0), 0.0, 1.0)) + ((vec3<f32>(_e25.g0_.z) * vec3<f32>(_e29.g0_.z, _e32.g0_.y, _e35.g0_.z)) * vec3<f32>(1.0, -(1.0), 0.0))) + ((vec3<f32>(_e47.g0_.x) * vec3<f32>(_e51.g0_.x, _e54.g0_.w, _e57.g0_.z)) * vec3<f32>(0.0, 1.0, -(1.0)))));
}

fn point_motor_outer_product(self_570: Point, other_490: Motor) -> Point {
    var self_571: Point;
    var other_491: Motor;

    self_571 = self_570;
    other_491 = other_490;
    let _e4: Point = self_571;
    let _e6: Motor = other_491;
    return Point((_e4.g0_ * vec3<f32>(_e6.g0_.x)));
}

fn point_motor_inner_product(self_572: Point, other_492: Motor) -> Motor {
    var self_573: Point;
    var other_493: Motor;

    self_573 = self_572;
    other_493 = other_492;
    let _e4: Point = self_573;
    let _e8: Motor = other_493;
    let _e19: Point = self_573;
    let _e23: Motor = other_493;
    let _e35: Point = self_573;
    let _e39: Motor = other_493;
    return Motor(((((vec4<f32>(_e4.g0_.y) * _e8.g0_.zzxz) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0)) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.wwwx) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))) + ((vec4<f32>(_e35.g0_.x) * _e39.g0_.yxxx) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))));
}

fn point_motor_left_contraction(self_574: Point, other_494: Motor) -> Scalar {
    var self_575: Point;
    var other_495: Motor;

    self_575 = self_574;
    other_495 = other_494;
    let _e5: Point = self_575;
    let _e8: Motor = other_495;
    let _e13: Point = self_575;
    let _e16: Motor = other_495;
    let _e21: Point = self_575;
    let _e24: Motor = other_495;
    return Scalar((((0.0 - (_e5.g0_.x * _e8.g0_.y)) - (_e13.g0_.y * _e16.g0_.z)) - (_e21.g0_.z * _e24.g0_.w)));
}

fn point_motor_right_contraction(self_576: Point, other_496: Motor) -> Motor {
    var self_577: Point;
    var other_497: Motor;

    self_577 = self_576;
    other_497 = other_496;
    let _e4: Point = self_577;
    let _e8: Motor = other_497;
    let _e19: Point = self_577;
    let _e23: Motor = other_497;
    let _e35: Point = self_577;
    let _e39: Motor = other_497;
    return Motor(((((vec4<f32>(_e4.g0_.y) * _e8.g0_.zzxz) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0)) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.wwwx) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))) + ((vec4<f32>(_e35.g0_.x) * _e39.g0_.yxxx) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))));
}

fn point_motor_scalar_product(self_578: Point, other_498: Motor) -> Scalar {
    var self_579: Point;
    var other_499: Motor;

    self_579 = self_578;
    other_499 = other_498;
    let _e5: Point = self_579;
    let _e8: Motor = other_499;
    let _e13: Point = self_579;
    let _e16: Motor = other_499;
    let _e21: Point = self_579;
    let _e24: Motor = other_499;
    return Scalar((((0.0 - (_e5.g0_.x * _e8.g0_.y)) - (_e13.g0_.y * _e16.g0_.z)) - (_e21.g0_.z * _e24.g0_.w)));
}

fn point_motor_dual_geometric_product(self_580: Point, other_500: MotorDual) -> MotorDual {
    var self_581: Point;
    var other_501: MotorDual;

    self_581 = self_580;
    other_501 = other_500;
    let _e4: Point = self_581;
    let _e8: MotorDual = other_501;
    let _e20: Point = self_581;
    let _e24: MotorDual = other_501;
    let _e37: Point = self_581;
    let _e41: MotorDual = other_501;
    return MotorDual(((((vec4<f32>(_e4.g0_.x) * _e8.g0_.yxwz) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0)) + ((vec4<f32>(_e20.g0_.y) * _e24.g0_.zwxy) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e37.g0_.z) * _e41.g0_.wzyx) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))));
}

fn point_motor_dual_regressive_product(self_582: Point, other_502: MotorDual) -> Motor {
    var self_583: Point;
    var other_503: MotorDual;

    self_583 = self_582;
    other_503 = other_502;
    let _e4: Point = self_583;
    let _e8: MotorDual = other_503;
    let _e18: Point = self_583;
    let _e22: MotorDual = other_503;
    let _e33: Point = self_583;
    let _e37: MotorDual = other_503;
    return Motor(((((vec4<f32>(_e4.g0_.y) * _e8.g0_.zzxz) * vec4<f32>(1.0, 0.0, 1.0, 0.0)) + ((vec4<f32>(_e18.g0_.z) * _e22.g0_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e33.g0_.x) * _e37.g0_.yxxx) * vec4<f32>(1.0, 1.0, 0.0, 0.0))));
}

fn point_motor_dual_inner_product(self_584: Point, other_504: MotorDual) -> Plane {
    var self_585: Point;
    var other_505: MotorDual;

    self_585 = self_584;
    other_505 = other_504;
    let _e4: Point = self_585;
    let _e8: MotorDual = other_505;
    let _e11: MotorDual = other_505;
    let _e14: MotorDual = other_505;
    let _e26: Point = self_585;
    let _e30: MotorDual = other_505;
    let _e33: MotorDual = other_505;
    let _e36: MotorDual = other_505;
    let _e49: Point = self_585;
    let _e53: MotorDual = other_505;
    let _e56: MotorDual = other_505;
    let _e59: MotorDual = other_505;
    return Plane(((((vec3<f32>(_e4.g0_.x) * vec3<f32>(_e8.g0_.x, _e11.g0_.w, _e14.g0_.z)) * vec3<f32>(-(1.0), -(1.0), 1.0)) + ((vec3<f32>(_e26.g0_.y) * vec3<f32>(_e30.g0_.w, _e33.g0_.x, _e36.g0_.y)) * vec3<f32>(1.0, -(1.0), -(1.0)))) + ((vec3<f32>(_e49.g0_.z) * vec3<f32>(_e53.g0_.z, _e56.g0_.y, _e59.g0_.x)) * vec3<f32>(-(1.0), 1.0, -(1.0)))));
}

fn point_motor_dual_left_contraction(self_586: Point, other_506: MotorDual) -> Plane {
    var self_587: Point;
    var other_507: MotorDual;

    self_587 = self_586;
    other_507 = other_506;
    let _e4: Point = self_587;
    let _e6: MotorDual = other_507;
    return Plane(((_e4.g0_ * vec3<f32>(_e6.g0_.x)) * vec3<f32>(-(1.0))));
}

fn point_motor_dual_right_contraction(self_588: Point, other_508: MotorDual) -> Plane {
    var self_589: Point;
    var other_509: MotorDual;

    self_589 = self_588;
    other_509 = other_508;
    let _e4: Point = self_589;
    let _e8: MotorDual = other_509;
    let _e11: MotorDual = other_509;
    let _e14: MotorDual = other_509;
    let _e25: Point = self_589;
    let _e29: MotorDual = other_509;
    let _e32: MotorDual = other_509;
    let _e35: MotorDual = other_509;
    let _e47: Point = self_589;
    let _e51: MotorDual = other_509;
    let _e54: MotorDual = other_509;
    let _e57: MotorDual = other_509;
    return Plane(((((vec3<f32>(_e4.g0_.y) * vec3<f32>(_e8.g0_.w, _e11.g0_.w, _e14.g0_.y)) * vec3<f32>(1.0, 0.0, -(1.0))) + ((vec3<f32>(_e25.g0_.z) * vec3<f32>(_e29.g0_.z, _e32.g0_.y, _e35.g0_.z)) * vec3<f32>(-(1.0), 1.0, 0.0))) + ((vec3<f32>(_e47.g0_.x) * vec3<f32>(_e51.g0_.x, _e54.g0_.w, _e57.g0_.z)) * vec3<f32>(0.0, -(1.0), 1.0))));
}

fn point_squared_magnitude(self_590: Point) -> Scalar {
    var self_591: Point;

    self_591 = self_590;
    let _e2: Point = self_591;
    let _e3: Point = self_591;
    let _e4: Point = point_reversal(_e3);
    let _e5: Scalar = point_point_scalar_product(_e2, _e4);
    return _e5;
}

fn point_magnitude(self_592: Point) -> Scalar {
    var self_593: Point;

    self_593 = self_592;
    let _e2: Point = self_593;
    let _e3: Scalar = point_squared_magnitude(_e2);
    return Scalar(sqrt(_e3.g0_));
}

fn point_scale(self_594: Point, other_510: f32) -> Point {
    var self_595: Point;
    var other_511: f32;

    self_595 = self_594;
    other_511 = other_510;
    let _e4: Point = self_595;
    let _e5: f32 = other_511;
    let _e7: Point = point_scalar_geometric_product(_e4, Scalar(_e5));
    return _e7;
}

fn point_signum(self_596: Point) -> Point {
    var self_597: Point;

    self_597 = self_596;
    let _e2: Point = self_597;
    let _e3: Point = self_597;
    let _e4: Scalar = point_magnitude(_e3);
    let _e9: Point = point_scalar_geometric_product(_e2, Scalar((1.0 / _e4.g0_)));
    return _e9;
}

fn point_inverse(self_598: Point) -> Point {
    var self_599: Point;

    self_599 = self_598;
    let _e2: Point = self_599;
    let _e3: Point = point_reversal(_e2);
    let _e4: Point = self_599;
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

fn ideal_point_neg(self_600: IdealPoint) -> IdealPoint {
    var self_601: IdealPoint;

    self_601 = self_600;
    let _e2: IdealPoint = self_601;
    return IdealPoint((_e2.g0_ * vec2<f32>(-(1.0))));
}

fn ideal_point_automorphism(self_602: IdealPoint) -> IdealPoint {
    var self_603: IdealPoint;

    self_603 = self_602;
    let _e2: IdealPoint = self_603;
    return IdealPoint(_e2.g0_);
}

fn ideal_point_reversal(self_604: IdealPoint) -> IdealPoint {
    var self_605: IdealPoint;

    self_605 = self_604;
    let _e2: IdealPoint = self_605;
    return IdealPoint((_e2.g0_ * vec2<f32>(-(1.0))));
}

fn ideal_point_conjugation(self_606: IdealPoint) -> IdealPoint {
    var self_607: IdealPoint;

    self_607 = self_606;
    let _e2: IdealPoint = self_607;
    return IdealPoint((_e2.g0_ * vec2<f32>(-(1.0))));
}

fn ideal_point_scalar_add(self_608: IdealPoint, other_512: Scalar) -> Translator {
    var self_609: IdealPoint;
    var other_513: Scalar;

    self_609 = self_608;
    other_513 = other_512;
    let _e4: IdealPoint = self_609;
    let _e7: IdealPoint = self_609;
    let _e10: IdealPoint = self_609;
    let _e19: Scalar = other_513;
    return Translator(((vec3<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.y) * vec3<f32>(0.0, 1.0, 1.0)) + (vec3<f32>(_e19.g0_) * vec3<f32>(1.0, 0.0, 0.0))));
}

fn ideal_point_scalar_sub(self_610: IdealPoint, other_514: Scalar) -> Translator {
    var self_611: IdealPoint;
    var other_515: Scalar;

    self_611 = self_610;
    other_515 = other_514;
    let _e4: IdealPoint = self_611;
    let _e7: IdealPoint = self_611;
    let _e10: IdealPoint = self_611;
    let _e19: Scalar = other_515;
    return Translator(((vec3<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.y) * vec3<f32>(0.0, 1.0, 1.0)) - (vec3<f32>(_e19.g0_) * vec3<f32>(1.0, 0.0, 0.0))));
}

fn ideal_point_scalar_geometric_product(self_612: IdealPoint, other_516: Scalar) -> IdealPoint {
    var self_613: IdealPoint;
    var other_517: Scalar;

    self_613 = self_612;
    other_517 = other_516;
    let _e4: IdealPoint = self_613;
    let _e6: Scalar = other_517;
    return IdealPoint((_e4.g0_ * vec2<f32>(_e6.g0_)));
}

fn ideal_point_scalar_outer_product(self_614: IdealPoint, other_518: Scalar) -> IdealPoint {
    var self_615: IdealPoint;
    var other_519: Scalar;

    self_615 = self_614;
    other_519 = other_518;
    let _e4: IdealPoint = self_615;
    let _e6: Scalar = other_519;
    return IdealPoint((_e4.g0_ * vec2<f32>(_e6.g0_)));
}

fn ideal_point_scalar_inner_product(self_616: IdealPoint, other_520: Scalar) -> IdealPoint {
    var self_617: IdealPoint;
    var other_521: Scalar;

    self_617 = self_616;
    other_521 = other_520;
    let _e4: IdealPoint = self_617;
    let _e6: Scalar = other_521;
    return IdealPoint((_e4.g0_ * vec2<f32>(_e6.g0_)));
}

fn ideal_point_scalar_right_contraction(self_618: IdealPoint, other_522: Scalar) -> IdealPoint {
    var self_619: IdealPoint;
    var other_523: Scalar;

    self_619 = self_618;
    other_523 = other_522;
    let _e4: IdealPoint = self_619;
    let _e6: Scalar = other_523;
    return IdealPoint((_e4.g0_ * vec2<f32>(_e6.g0_)));
}

fn ideal_point_multi_vector_add(self_620: IdealPoint, other_524: MultiVector) -> MultiVector {
    var self_621: IdealPoint;
    var other_525: MultiVector;

    self_621 = self_620;
    other_525 = other_524;
    let _e4: MultiVector = other_525;
    let _e6: IdealPoint = self_621;
    let _e9: IdealPoint = self_621;
    let _e12: IdealPoint = self_621;
    let _e15: IdealPoint = self_621;
    let _e25: MultiVector = other_525;
    return MultiVector(_e4.g0_, ((vec4<f32>(_e6.g0_.x, _e9.g0_.x, _e12.g0_.x, _e15.g0_.y) * vec4<f32>(0.0, 0.0, 1.0, 1.0)) + _e25.g1_));
}

fn ideal_point_multi_vector_sub(self_622: IdealPoint, other_526: MultiVector) -> MultiVector {
    var self_623: IdealPoint;
    var other_527: MultiVector;

    self_623 = self_622;
    other_527 = other_526;
    let _e6: MultiVector = other_527;
    let _e9: IdealPoint = self_623;
    let _e12: IdealPoint = self_623;
    let _e15: IdealPoint = self_623;
    let _e18: IdealPoint = self_623;
    let _e28: MultiVector = other_527;
    return MultiVector((vec4<f32>(0.0) - _e6.g0_), ((vec4<f32>(_e9.g0_.x, _e12.g0_.x, _e15.g0_.x, _e18.g0_.y) * vec4<f32>(0.0, 0.0, 1.0, 1.0)) - _e28.g1_));
}

fn ideal_point_multi_vector_geometric_product(self_624: IdealPoint, other_528: MultiVector) -> MultiVector {
    var self_625: IdealPoint;
    var other_529: MultiVector;

    self_625 = self_624;
    other_529 = other_528;
    let _e4: IdealPoint = self_625;
    let _e8: MultiVector = other_529;
    let _e21: IdealPoint = self_625;
    let _e25: MultiVector = other_529;
    let _e39: IdealPoint = self_625;
    let _e43: MultiVector = other_529;
    let _e54: IdealPoint = self_625;
    let _e58: MultiVector = other_529;
    return MultiVector((((vec4<f32>(_e4.g0_.x) * _e8.g1_.zwxy) * vec4<f32>(-(1.0), 1.0, -(1.0), -(1.0))) + ((vec4<f32>(_e21.g0_.y) * _e25.g1_.wzyx) * vec4<f32>(-(1.0), -(1.0), -(1.0), 1.0))), (((vec4<f32>(_e39.g0_.x) * _e43.g0_.zwxy) * vec4<f32>(1.0, 1.0, 1.0, -(1.0))) + ((vec4<f32>(_e54.g0_.y) * _e58.g0_.wzyx) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0))));
}

fn ideal_point_multi_vector_scalar_product(self_626: IdealPoint, other_530: MultiVector) -> Scalar {
    var self_627: IdealPoint;
    var other_531: MultiVector;

    self_627 = self_626;
    other_531 = other_530;
    let _e5: IdealPoint = self_627;
    let _e8: MultiVector = other_531;
    let _e13: IdealPoint = self_627;
    let _e16: MultiVector = other_531;
    return Scalar(((0.0 - (_e5.g0_.x * _e8.g1_.z)) - (_e13.g0_.y * _e16.g1_.w)));
}

fn ideal_point_rotor_add(self_628: IdealPoint, other_532: Rotor) -> Motor {
    var self_629: IdealPoint;
    var other_533: Rotor;

    self_629 = self_628;
    other_533 = other_532;
    let _e4: IdealPoint = self_629;
    let _e7: IdealPoint = self_629;
    let _e10: IdealPoint = self_629;
    let _e13: IdealPoint = self_629;
    let _e23: Rotor = other_533;
    let _e26: Rotor = other_533;
    let _e29: Rotor = other_533;
    let _e32: Rotor = other_533;
    return Motor(((vec4<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.x, _e13.g0_.y) * vec4<f32>(0.0, 0.0, 1.0, 1.0)) + (vec4<f32>(_e23.g0_.x, _e26.g0_.y, _e29.g0_.x, _e32.g0_.x) * vec4<f32>(1.0, 1.0, 0.0, 0.0))));
}

fn ideal_point_rotor_sub(self_630: IdealPoint, other_534: Rotor) -> Motor {
    var self_631: IdealPoint;
    var other_535: Rotor;

    self_631 = self_630;
    other_535 = other_534;
    let _e4: IdealPoint = self_631;
    let _e7: IdealPoint = self_631;
    let _e10: IdealPoint = self_631;
    let _e13: IdealPoint = self_631;
    let _e23: Rotor = other_535;
    let _e26: Rotor = other_535;
    let _e29: Rotor = other_535;
    let _e32: Rotor = other_535;
    return Motor(((vec4<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.x, _e13.g0_.y) * vec4<f32>(0.0, 0.0, 1.0, 1.0)) - (vec4<f32>(_e23.g0_.x, _e26.g0_.y, _e29.g0_.x, _e32.g0_.x) * vec4<f32>(1.0, 1.0, 0.0, 0.0))));
}

fn ideal_point_rotor_geometric_product(self_632: IdealPoint, other_536: Rotor) -> IdealPoint {
    var self_633: IdealPoint;
    var other_537: Rotor;

    self_633 = self_632;
    other_537 = other_536;
    let _e4: IdealPoint = self_633;
    let _e8: Rotor = other_537;
    let _e16: IdealPoint = self_633;
    let _e20: Rotor = other_537;
    return IdealPoint((((vec2<f32>(_e4.g0_.x) * _e8.g0_) * vec2<f32>(1.0, -(1.0))) + (vec2<f32>(_e16.g0_.y) * _e20.g0_.yx)));
}

fn ideal_point_rotor_outer_product(self_634: IdealPoint, other_538: Rotor) -> IdealPoint {
    var self_635: IdealPoint;
    var other_539: Rotor;

    self_635 = self_634;
    other_539 = other_538;
    let _e4: IdealPoint = self_635;
    let _e6: Rotor = other_539;
    return IdealPoint((_e4.g0_ * vec2<f32>(_e6.g0_.x)));
}

fn ideal_point_rotor_inner_product(self_636: IdealPoint, other_540: Rotor) -> IdealPoint {
    var self_637: IdealPoint;
    var other_541: Rotor;

    self_637 = self_636;
    other_541 = other_540;
    let _e4: IdealPoint = self_637;
    let _e6: Rotor = other_541;
    return IdealPoint((_e4.g0_ * vec2<f32>(_e6.g0_.x)));
}

fn ideal_point_rotor_right_contraction(self_638: IdealPoint, other_542: Rotor) -> IdealPoint {
    var self_639: IdealPoint;
    var other_543: Rotor;

    self_639 = self_638;
    other_543 = other_542;
    let _e4: IdealPoint = self_639;
    let _e6: Rotor = other_543;
    return IdealPoint((_e4.g0_ * vec2<f32>(_e6.g0_.x)));
}

fn ideal_point_point_add(self_640: IdealPoint, other_544: Point) -> Point {
    var self_641: IdealPoint;
    var other_545: Point;

    self_641 = self_640;
    other_545 = other_544;
    let _e4: IdealPoint = self_641;
    let _e7: IdealPoint = self_641;
    let _e10: IdealPoint = self_641;
    let _e19: Point = other_545;
    return Point(((vec3<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.y) * vec3<f32>(0.0, 1.0, 1.0)) + _e19.g0_));
}

fn ideal_point_point_sub(self_642: IdealPoint, other_546: Point) -> Point {
    var self_643: IdealPoint;
    var other_547: Point;

    self_643 = self_642;
    other_547 = other_546;
    let _e4: IdealPoint = self_643;
    let _e7: IdealPoint = self_643;
    let _e10: IdealPoint = self_643;
    let _e19: Point = other_547;
    return Point(((vec3<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.y) * vec3<f32>(0.0, 1.0, 1.0)) - _e19.g0_));
}

fn ideal_point_point_geometric_product(self_644: IdealPoint, other_548: Point) -> Motor {
    var self_645: IdealPoint;
    var other_549: Point;

    self_645 = self_644;
    other_549 = other_548;
    let _e4: IdealPoint = self_645;
    let _e8: Point = other_549;
    let _e11: Point = other_549;
    let _e14: Point = other_549;
    let _e17: Point = other_549;
    let _e30: IdealPoint = self_645;
    let _e34: Point = other_549;
    let _e37: Point = other_549;
    let _e40: Point = other_549;
    let _e43: Point = other_549;
    return Motor((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.z, _e11.g0_.y, _e14.g0_.x, _e17.g0_.z)) * vec4<f32>(-(1.0), -(1.0), 1.0, 0.0)) + ((vec4<f32>(_e30.g0_.x) * vec4<f32>(_e34.g0_.y, _e37.g0_.z, _e40.g0_.x, _e43.g0_.x)) * vec4<f32>(-(1.0), 1.0, 0.0, -(1.0)))));
}

fn ideal_point_point_regressive_product(self_646: IdealPoint, other_550: Point) -> Plane {
    var self_647: IdealPoint;
    var other_551: Point;

    self_647 = self_646;
    other_551 = other_550;
    let _e4: IdealPoint = self_647;
    let _e8: Point = other_551;
    let _e18: IdealPoint = self_647;
    let _e22: Point = other_551;
    return Plane((((vec3<f32>(_e4.g0_.y) * _e8.g0_.yxy) * vec3<f32>(1.0, -(1.0), 0.0)) + ((vec3<f32>(_e18.g0_.x) * _e22.g0_.zxx) * vec3<f32>(-(1.0), 0.0, 1.0))));
}

fn ideal_point_point_inner_product(self_648: IdealPoint, other_552: Point) -> Scalar {
    var self_649: IdealPoint;
    var other_553: Point;

    self_649 = self_648;
    other_553 = other_552;
    let _e5: IdealPoint = self_649;
    let _e8: Point = other_553;
    let _e13: IdealPoint = self_649;
    let _e16: Point = other_553;
    return Scalar(((0.0 - (_e5.g0_.x * _e8.g0_.y)) - (_e13.g0_.y * _e16.g0_.z)));
}

fn ideal_point_point_left_contraction(self_650: IdealPoint, other_554: Point) -> Scalar {
    var self_651: IdealPoint;
    var other_555: Point;

    self_651 = self_650;
    other_555 = other_554;
    let _e5: IdealPoint = self_651;
    let _e8: Point = other_555;
    let _e13: IdealPoint = self_651;
    let _e16: Point = other_555;
    return Scalar(((0.0 - (_e5.g0_.x * _e8.g0_.y)) - (_e13.g0_.y * _e16.g0_.z)));
}

fn ideal_point_point_right_contraction(self_652: IdealPoint, other_556: Point) -> Scalar {
    var self_653: IdealPoint;
    var other_557: Point;

    self_653 = self_652;
    other_557 = other_556;
    let _e5: IdealPoint = self_653;
    let _e8: Point = other_557;
    let _e13: IdealPoint = self_653;
    let _e16: Point = other_557;
    return Scalar(((0.0 - (_e5.g0_.x * _e8.g0_.y)) - (_e13.g0_.y * _e16.g0_.z)));
}

fn ideal_point_point_scalar_product(self_654: IdealPoint, other_558: Point) -> Scalar {
    var self_655: IdealPoint;
    var other_559: Point;

    self_655 = self_654;
    other_559 = other_558;
    let _e5: IdealPoint = self_655;
    let _e8: Point = other_559;
    let _e13: IdealPoint = self_655;
    let _e16: Point = other_559;
    return Scalar(((0.0 - (_e5.g0_.x * _e8.g0_.y)) - (_e13.g0_.y * _e16.g0_.z)));
}

fn ideal_point_ideal_point_add(self_656: IdealPoint, other_560: IdealPoint) -> IdealPoint {
    var self_657: IdealPoint;
    var other_561: IdealPoint;

    self_657 = self_656;
    other_561 = other_560;
    let _e4: IdealPoint = self_657;
    let _e6: IdealPoint = other_561;
    return IdealPoint((_e4.g0_ + _e6.g0_));
}

fn ideal_point_ideal_point_sub(self_658: IdealPoint, other_562: IdealPoint) -> IdealPoint {
    var self_659: IdealPoint;
    var other_563: IdealPoint;

    self_659 = self_658;
    other_563 = other_562;
    let _e4: IdealPoint = self_659;
    let _e6: IdealPoint = other_563;
    return IdealPoint((_e4.g0_ - _e6.g0_));
}

fn ideal_point_ideal_point_mul(self_660: IdealPoint, other_564: IdealPoint) -> IdealPoint {
    var self_661: IdealPoint;
    var other_565: IdealPoint;

    self_661 = self_660;
    other_565 = other_564;
    let _e4: IdealPoint = self_661;
    let _e6: IdealPoint = other_565;
    return IdealPoint((_e4.g0_ * _e6.g0_));
}

fn ideal_point_ideal_point_div(self_662: IdealPoint, other_566: IdealPoint) -> IdealPoint {
    var self_663: IdealPoint;
    var other_567: IdealPoint;

    self_663 = self_662;
    other_567 = other_566;
    let _e4: IdealPoint = self_663;
    let _e7: IdealPoint = self_663;
    let _e15: IdealPoint = other_567;
    let _e18: IdealPoint = other_567;
    return IdealPoint((((vec2<f32>(_e4.g0_.x, _e7.g0_.y) * vec2<f32>(1.0, 1.0)) / vec2<f32>(_e15.g0_.x, _e18.g0_.y)) * vec2<f32>(1.0, 1.0)));
}

fn ideal_point_ideal_point_geometric_product(self_664: IdealPoint, other_568: IdealPoint) -> Rotor {
    var self_665: IdealPoint;
    var other_569: IdealPoint;

    self_665 = self_664;
    other_569 = other_568;
    let _e4: IdealPoint = self_665;
    let _e8: IdealPoint = other_569;
    let _e16: IdealPoint = self_665;
    let _e20: IdealPoint = other_569;
    return Rotor((((vec2<f32>(_e4.g0_.x) * _e8.g0_) * vec2<f32>(-(1.0), 1.0)) - (vec2<f32>(_e16.g0_.y) * _e20.g0_.yx)));
}

fn ideal_point_ideal_point_inner_product(self_666: IdealPoint, other_570: IdealPoint) -> Scalar {
    var self_667: IdealPoint;
    var other_571: IdealPoint;

    self_667 = self_666;
    other_571 = other_570;
    let _e5: IdealPoint = self_667;
    let _e8: IdealPoint = other_571;
    let _e13: IdealPoint = self_667;
    let _e16: IdealPoint = other_571;
    return Scalar(((0.0 - (_e5.g0_.x * _e8.g0_.x)) - (_e13.g0_.y * _e16.g0_.y)));
}

fn ideal_point_ideal_point_left_contraction(self_668: IdealPoint, other_572: IdealPoint) -> Scalar {
    var self_669: IdealPoint;
    var other_573: IdealPoint;

    self_669 = self_668;
    other_573 = other_572;
    let _e5: IdealPoint = self_669;
    let _e8: IdealPoint = other_573;
    let _e13: IdealPoint = self_669;
    let _e16: IdealPoint = other_573;
    return Scalar(((0.0 - (_e5.g0_.x * _e8.g0_.x)) - (_e13.g0_.y * _e16.g0_.y)));
}

fn ideal_point_ideal_point_right_contraction(self_670: IdealPoint, other_574: IdealPoint) -> Scalar {
    var self_671: IdealPoint;
    var other_575: IdealPoint;

    self_671 = self_670;
    other_575 = other_574;
    let _e5: IdealPoint = self_671;
    let _e8: IdealPoint = other_575;
    let _e13: IdealPoint = self_671;
    let _e16: IdealPoint = other_575;
    return Scalar(((0.0 - (_e5.g0_.x * _e8.g0_.x)) - (_e13.g0_.y * _e16.g0_.y)));
}

fn ideal_point_ideal_point_scalar_product(self_672: IdealPoint, other_576: IdealPoint) -> Scalar {
    var self_673: IdealPoint;
    var other_577: IdealPoint;

    self_673 = self_672;
    other_577 = other_576;
    let _e5: IdealPoint = self_673;
    let _e8: IdealPoint = other_577;
    let _e13: IdealPoint = self_673;
    let _e16: IdealPoint = other_577;
    return Scalar(((0.0 - (_e5.g0_.x * _e8.g0_.x)) - (_e13.g0_.y * _e16.g0_.y)));
}

fn ideal_point_plane_geometric_product(self_674: IdealPoint, other_578: Plane) -> MotorDual {
    var self_675: IdealPoint;
    var other_579: Plane;

    self_675 = self_674;
    other_579 = other_578;
    let _e4: IdealPoint = self_675;
    let _e8: Plane = other_579;
    let _e11: Plane = other_579;
    let _e14: Plane = other_579;
    let _e17: Plane = other_579;
    let _e29: IdealPoint = self_675;
    let _e33: Plane = other_579;
    let _e36: Plane = other_579;
    let _e39: Plane = other_579;
    let _e42: Plane = other_579;
    return MotorDual((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.z, _e11.g0_.y, _e14.g0_.x, _e17.g0_.z)) * vec4<f32>(1.0, -(1.0), 1.0, 0.0)) + ((vec4<f32>(_e29.g0_.x) * vec4<f32>(_e33.g0_.y, _e36.g0_.z, _e39.g0_.x, _e42.g0_.x)) * vec4<f32>(1.0, 1.0, 0.0, -(1.0)))));
}

fn ideal_point_plane_regressive_product(self_676: IdealPoint, other_580: Plane) -> Scalar {
    var self_677: IdealPoint;
    var other_581: Plane;

    self_677 = self_676;
    other_581 = other_580;
    let _e4: IdealPoint = self_677;
    let _e7: Plane = other_581;
    let _e11: IdealPoint = self_677;
    let _e14: Plane = other_581;
    return Scalar(((_e4.g0_.x * _e7.g0_.y) + (_e11.g0_.y * _e14.g0_.z)));
}

fn ideal_point_plane_inner_product(self_678: IdealPoint, other_582: Plane) -> Plane {
    var self_679: IdealPoint;
    var other_583: Plane;

    self_679 = self_678;
    other_583 = other_582;
    let _e4: IdealPoint = self_679;
    let _e8: Plane = other_583;
    let _e18: IdealPoint = self_679;
    let _e22: Plane = other_583;
    return Plane((((vec3<f32>(_e4.g0_.y) * _e8.g0_.yxy) * vec3<f32>(-(1.0), 1.0, 0.0)) + ((vec3<f32>(_e18.g0_.x) * _e22.g0_.zxx) * vec3<f32>(1.0, 0.0, -(1.0)))));
}

fn ideal_point_plane_right_contraction(self_680: IdealPoint, other_584: Plane) -> Plane {
    var self_681: IdealPoint;
    var other_585: Plane;

    self_681 = self_680;
    other_585 = other_584;
    let _e4: IdealPoint = self_681;
    let _e8: Plane = other_585;
    let _e18: IdealPoint = self_681;
    let _e22: Plane = other_585;
    return Plane((((vec3<f32>(_e4.g0_.y) * _e8.g0_.yxy) * vec3<f32>(-(1.0), 1.0, 0.0)) + ((vec3<f32>(_e18.g0_.x) * _e22.g0_.zxx) * vec3<f32>(1.0, 0.0, -(1.0)))));
}

fn ideal_point_translator_add(self_682: IdealPoint, other_586: Translator) -> Translator {
    var self_683: IdealPoint;
    var other_587: Translator;

    self_683 = self_682;
    other_587 = other_586;
    let _e4: IdealPoint = self_683;
    let _e7: IdealPoint = self_683;
    let _e10: IdealPoint = self_683;
    let _e19: Translator = other_587;
    return Translator(((vec3<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.y) * vec3<f32>(0.0, 1.0, 1.0)) + _e19.g0_));
}

fn ideal_point_translator_sub(self_684: IdealPoint, other_588: Translator) -> Translator {
    var self_685: IdealPoint;
    var other_589: Translator;

    self_685 = self_684;
    other_589 = other_588;
    let _e4: IdealPoint = self_685;
    let _e7: IdealPoint = self_685;
    let _e10: IdealPoint = self_685;
    let _e19: Translator = other_589;
    return Translator(((vec3<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.y) * vec3<f32>(0.0, 1.0, 1.0)) - _e19.g0_));
}

fn ideal_point_translator_geometric_product(self_686: IdealPoint, other_590: Translator) -> Motor {
    var self_687: IdealPoint;
    var other_591: Translator;

    self_687 = self_686;
    other_591 = other_590;
    let _e4: IdealPoint = self_687;
    let _e8: Translator = other_591;
    let _e11: Translator = other_591;
    let _e14: Translator = other_591;
    let _e17: Translator = other_591;
    let _e30: IdealPoint = self_687;
    let _e34: Translator = other_591;
    let _e37: Translator = other_591;
    let _e40: Translator = other_591;
    let _e43: Translator = other_591;
    return Motor((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.z, _e11.g0_.y, _e14.g0_.z, _e17.g0_.x)) * vec4<f32>(-(1.0), -(1.0), 0.0, 1.0)) + ((vec4<f32>(_e30.g0_.x) * vec4<f32>(_e34.g0_.y, _e37.g0_.z, _e40.g0_.x, _e43.g0_.x)) * vec4<f32>(-(1.0), 1.0, 1.0, 0.0))));
}

fn ideal_point_translator_outer_product(self_688: IdealPoint, other_592: Translator) -> IdealPoint {
    var self_689: IdealPoint;
    var other_593: Translator;

    self_689 = self_688;
    other_593 = other_592;
    let _e4: IdealPoint = self_689;
    let _e6: Translator = other_593;
    return IdealPoint((_e4.g0_ * vec2<f32>(_e6.g0_.x)));
}

fn ideal_point_translator_inner_product(self_690: IdealPoint, other_594: Translator) -> Translator {
    var self_691: IdealPoint;
    var other_595: Translator;

    self_691 = self_690;
    other_595 = other_594;
    let _e4: IdealPoint = self_691;
    let _e8: Translator = other_595;
    let _e18: IdealPoint = self_691;
    let _e22: Translator = other_595;
    return Translator((((vec3<f32>(_e4.g0_.y) * _e8.g0_.zzx) * vec3<f32>(-(1.0), 0.0, 1.0)) + ((vec3<f32>(_e18.g0_.x) * _e22.g0_.yxx) * vec3<f32>(-(1.0), 1.0, 0.0))));
}

fn ideal_point_translator_left_contraction(self_692: IdealPoint, other_596: Translator) -> Scalar {
    var self_693: IdealPoint;
    var other_597: Translator;

    self_693 = self_692;
    other_597 = other_596;
    let _e5: IdealPoint = self_693;
    let _e8: Translator = other_597;
    let _e13: IdealPoint = self_693;
    let _e16: Translator = other_597;
    return Scalar(((0.0 - (_e5.g0_.x * _e8.g0_.y)) - (_e13.g0_.y * _e16.g0_.z)));
}

fn ideal_point_translator_right_contraction(self_694: IdealPoint, other_598: Translator) -> Translator {
    var self_695: IdealPoint;
    var other_599: Translator;

    self_695 = self_694;
    other_599 = other_598;
    let _e4: IdealPoint = self_695;
    let _e8: Translator = other_599;
    let _e18: IdealPoint = self_695;
    let _e22: Translator = other_599;
    return Translator((((vec3<f32>(_e4.g0_.y) * _e8.g0_.zzx) * vec3<f32>(-(1.0), 0.0, 1.0)) + ((vec3<f32>(_e18.g0_.x) * _e22.g0_.yxx) * vec3<f32>(-(1.0), 1.0, 0.0))));
}

fn ideal_point_translator_scalar_product(self_696: IdealPoint, other_600: Translator) -> Scalar {
    var self_697: IdealPoint;
    var other_601: Translator;

    self_697 = self_696;
    other_601 = other_600;
    let _e5: IdealPoint = self_697;
    let _e8: Translator = other_601;
    let _e13: IdealPoint = self_697;
    let _e16: Translator = other_601;
    return Scalar(((0.0 - (_e5.g0_.x * _e8.g0_.y)) - (_e13.g0_.y * _e16.g0_.z)));
}

fn ideal_point_motor_add(self_698: IdealPoint, other_602: Motor) -> Motor {
    var self_699: IdealPoint;
    var other_603: Motor;

    self_699 = self_698;
    other_603 = other_602;
    let _e4: IdealPoint = self_699;
    let _e7: IdealPoint = self_699;
    let _e10: IdealPoint = self_699;
    let _e13: IdealPoint = self_699;
    let _e23: Motor = other_603;
    return Motor(((vec4<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.x, _e13.g0_.y) * vec4<f32>(0.0, 0.0, 1.0, 1.0)) + _e23.g0_));
}

fn ideal_point_motor_sub(self_700: IdealPoint, other_604: Motor) -> Motor {
    var self_701: IdealPoint;
    var other_605: Motor;

    self_701 = self_700;
    other_605 = other_604;
    let _e4: IdealPoint = self_701;
    let _e7: IdealPoint = self_701;
    let _e10: IdealPoint = self_701;
    let _e13: IdealPoint = self_701;
    let _e23: Motor = other_605;
    return Motor(((vec4<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.x, _e13.g0_.y) * vec4<f32>(0.0, 0.0, 1.0, 1.0)) - _e23.g0_));
}

fn ideal_point_motor_geometric_product(self_702: IdealPoint, other_606: Motor) -> Motor {
    var self_703: IdealPoint;
    var other_607: Motor;

    self_703 = self_702;
    other_607 = other_606;
    let _e4: IdealPoint = self_703;
    let _e8: Motor = other_607;
    let _e20: IdealPoint = self_703;
    let _e24: Motor = other_607;
    return Motor((((vec4<f32>(_e4.g0_.x) * _e8.g0_.zwxy) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0))) + ((vec4<f32>(_e20.g0_.y) * _e24.g0_.wzyx) * vec4<f32>(-(1.0), -(1.0), 1.0, 1.0))));
}

fn ideal_point_motor_regressive_product(self_704: IdealPoint, other_608: Motor) -> Plane {
    var self_705: IdealPoint;
    var other_609: Motor;

    self_705 = self_704;
    other_609 = other_608;
    let _e4: IdealPoint = self_705;
    let _e8: Motor = other_609;
    let _e11: Motor = other_609;
    let _e14: Motor = other_609;
    let _e25: IdealPoint = self_705;
    let _e29: Motor = other_609;
    let _e32: Motor = other_609;
    let _e35: Motor = other_609;
    return Plane((((vec3<f32>(_e4.g0_.y) * vec3<f32>(_e8.g0_.z, _e11.g0_.y, _e14.g0_.z)) * vec3<f32>(1.0, -(1.0), 0.0)) + ((vec3<f32>(_e25.g0_.x) * vec3<f32>(_e29.g0_.w, _e32.g0_.x, _e35.g0_.y)) * vec3<f32>(-(1.0), 0.0, 1.0))));
}

fn ideal_point_motor_outer_product(self_706: IdealPoint, other_610: Motor) -> IdealPoint {
    var self_707: IdealPoint;
    var other_611: Motor;

    self_707 = self_706;
    other_611 = other_610;
    let _e4: IdealPoint = self_707;
    let _e6: Motor = other_611;
    return IdealPoint((_e4.g0_ * vec2<f32>(_e6.g0_.x)));
}

fn ideal_point_motor_inner_product(self_708: IdealPoint, other_612: Motor) -> Translator {
    var self_709: IdealPoint;
    var other_613: Motor;

    self_709 = self_708;
    other_613 = other_612;
    let _e4: IdealPoint = self_709;
    let _e8: Motor = other_613;
    let _e11: Motor = other_613;
    let _e14: Motor = other_613;
    let _e25: IdealPoint = self_709;
    let _e29: Motor = other_613;
    let _e32: Motor = other_613;
    let _e35: Motor = other_613;
    return Translator((((vec3<f32>(_e4.g0_.y) * vec3<f32>(_e8.g0_.w, _e11.g0_.w, _e14.g0_.x)) * vec3<f32>(-(1.0), 0.0, 1.0)) + ((vec3<f32>(_e25.g0_.x) * vec3<f32>(_e29.g0_.z, _e32.g0_.x, _e35.g0_.x)) * vec3<f32>(-(1.0), 1.0, 0.0))));
}

fn ideal_point_motor_left_contraction(self_710: IdealPoint, other_614: Motor) -> Scalar {
    var self_711: IdealPoint;
    var other_615: Motor;

    self_711 = self_710;
    other_615 = other_614;
    let _e5: IdealPoint = self_711;
    let _e8: Motor = other_615;
    let _e13: IdealPoint = self_711;
    let _e16: Motor = other_615;
    return Scalar(((0.0 - (_e5.g0_.x * _e8.g0_.z)) - (_e13.g0_.y * _e16.g0_.w)));
}

fn ideal_point_motor_right_contraction(self_712: IdealPoint, other_616: Motor) -> Translator {
    var self_713: IdealPoint;
    var other_617: Motor;

    self_713 = self_712;
    other_617 = other_616;
    let _e4: IdealPoint = self_713;
    let _e8: Motor = other_617;
    let _e11: Motor = other_617;
    let _e14: Motor = other_617;
    let _e25: IdealPoint = self_713;
    let _e29: Motor = other_617;
    let _e32: Motor = other_617;
    let _e35: Motor = other_617;
    return Translator((((vec3<f32>(_e4.g0_.y) * vec3<f32>(_e8.g0_.w, _e11.g0_.w, _e14.g0_.x)) * vec3<f32>(-(1.0), 0.0, 1.0)) + ((vec3<f32>(_e25.g0_.x) * vec3<f32>(_e29.g0_.z, _e32.g0_.x, _e35.g0_.x)) * vec3<f32>(-(1.0), 1.0, 0.0))));
}

fn ideal_point_motor_scalar_product(self_714: IdealPoint, other_618: Motor) -> Scalar {
    var self_715: IdealPoint;
    var other_619: Motor;

    self_715 = self_714;
    other_619 = other_618;
    let _e5: IdealPoint = self_715;
    let _e8: Motor = other_619;
    let _e13: IdealPoint = self_715;
    let _e16: Motor = other_619;
    return Scalar(((0.0 - (_e5.g0_.x * _e8.g0_.z)) - (_e13.g0_.y * _e16.g0_.w)));
}

fn ideal_point_motor_dual_geometric_product(self_716: IdealPoint, other_620: MotorDual) -> MotorDual {
    var self_717: IdealPoint;
    var other_621: MotorDual;

    self_717 = self_716;
    other_621 = other_620;
    let _e4: IdealPoint = self_717;
    let _e8: MotorDual = other_621;
    let _e20: IdealPoint = self_717;
    let _e24: MotorDual = other_621;
    return MotorDual((((vec4<f32>(_e4.g0_.x) * _e8.g0_.zwxy) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0))) + ((vec4<f32>(_e20.g0_.y) * _e24.g0_.wzyx) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))));
}

fn ideal_point_motor_dual_regressive_product(self_718: IdealPoint, other_622: MotorDual) -> Translator {
    var self_719: IdealPoint;
    var other_623: MotorDual;

    self_719 = self_718;
    other_623 = other_622;
    let _e4: IdealPoint = self_719;
    let _e8: MotorDual = other_623;
    let _e11: MotorDual = other_623;
    let _e14: MotorDual = other_623;
    let _e24: IdealPoint = self_719;
    let _e28: MotorDual = other_623;
    let _e31: MotorDual = other_623;
    let _e34: MotorDual = other_623;
    return Translator((((vec3<f32>(_e4.g0_.y) * vec3<f32>(_e8.g0_.w, _e11.g0_.w, _e14.g0_.x)) * vec3<f32>(1.0, 0.0, 1.0)) + ((vec3<f32>(_e24.g0_.x) * vec3<f32>(_e28.g0_.z, _e31.g0_.x, _e34.g0_.x)) * vec3<f32>(1.0, 1.0, 0.0))));
}

fn ideal_point_motor_dual_inner_product(self_720: IdealPoint, other_624: MotorDual) -> Plane {
    var self_721: IdealPoint;
    var other_625: MotorDual;

    self_721 = self_720;
    other_625 = other_624;
    let _e4: IdealPoint = self_721;
    let _e8: MotorDual = other_625;
    let _e11: MotorDual = other_625;
    let _e14: MotorDual = other_625;
    let _e26: IdealPoint = self_721;
    let _e30: MotorDual = other_625;
    let _e33: MotorDual = other_625;
    let _e36: MotorDual = other_625;
    return Plane((((vec3<f32>(_e4.g0_.x) * vec3<f32>(_e8.g0_.w, _e11.g0_.x, _e14.g0_.y)) * vec3<f32>(1.0, -(1.0), -(1.0))) + ((vec3<f32>(_e26.g0_.y) * vec3<f32>(_e30.g0_.z, _e33.g0_.y, _e36.g0_.x)) * vec3<f32>(-(1.0), 1.0, -(1.0)))));
}

fn ideal_point_motor_dual_right_contraction(self_722: IdealPoint, other_626: MotorDual) -> Plane {
    var self_723: IdealPoint;
    var other_627: MotorDual;

    self_723 = self_722;
    other_627 = other_626;
    let _e4: IdealPoint = self_723;
    let _e8: MotorDual = other_627;
    let _e11: MotorDual = other_627;
    let _e14: MotorDual = other_627;
    let _e25: IdealPoint = self_723;
    let _e29: MotorDual = other_627;
    let _e32: MotorDual = other_627;
    let _e35: MotorDual = other_627;
    return Plane((((vec3<f32>(_e4.g0_.y) * vec3<f32>(_e8.g0_.z, _e11.g0_.y, _e14.g0_.z)) * vec3<f32>(-(1.0), 1.0, 0.0)) + ((vec3<f32>(_e25.g0_.x) * vec3<f32>(_e29.g0_.w, _e32.g0_.x, _e35.g0_.y)) * vec3<f32>(1.0, 0.0, -(1.0)))));
}

fn ideal_point_squared_magnitude(self_724: IdealPoint) -> Scalar {
    var self_725: IdealPoint;

    self_725 = self_724;
    let _e2: IdealPoint = self_725;
    let _e3: IdealPoint = self_725;
    let _e4: IdealPoint = ideal_point_reversal(_e3);
    let _e5: Scalar = ideal_point_ideal_point_scalar_product(_e2, _e4);
    return _e5;
}

fn ideal_point_magnitude(self_726: IdealPoint) -> Scalar {
    var self_727: IdealPoint;

    self_727 = self_726;
    let _e2: IdealPoint = self_727;
    let _e3: Scalar = ideal_point_squared_magnitude(_e2);
    return Scalar(sqrt(_e3.g0_));
}

fn ideal_point_scale(self_728: IdealPoint, other_628: f32) -> IdealPoint {
    var self_729: IdealPoint;
    var other_629: f32;

    self_729 = self_728;
    other_629 = other_628;
    let _e4: IdealPoint = self_729;
    let _e5: f32 = other_629;
    let _e7: IdealPoint = ideal_point_scalar_geometric_product(_e4, Scalar(_e5));
    return _e7;
}

fn ideal_point_signum(self_730: IdealPoint) -> IdealPoint {
    var self_731: IdealPoint;

    self_731 = self_730;
    let _e2: IdealPoint = self_731;
    let _e3: IdealPoint = self_731;
    let _e4: Scalar = ideal_point_magnitude(_e3);
    let _e9: IdealPoint = ideal_point_scalar_geometric_product(_e2, Scalar((1.0 / _e4.g0_)));
    return _e9;
}

fn ideal_point_inverse(self_732: IdealPoint) -> IdealPoint {
    var self_733: IdealPoint;

    self_733 = self_732;
    let _e2: IdealPoint = self_733;
    let _e3: IdealPoint = ideal_point_reversal(_e2);
    let _e4: IdealPoint = self_733;
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

fn plane_neg(self_734: Plane) -> Plane {
    var self_735: Plane;

    self_735 = self_734;
    let _e2: Plane = self_735;
    return Plane((_e2.g0_ * vec3<f32>(-(1.0))));
}

fn plane_automorphism(self_736: Plane) -> Plane {
    var self_737: Plane;

    self_737 = self_736;
    let _e2: Plane = self_737;
    return Plane((_e2.g0_ * vec3<f32>(-(1.0))));
}

fn plane_reversal(self_738: Plane) -> Plane {
    var self_739: Plane;

    self_739 = self_738;
    let _e2: Plane = self_739;
    return Plane(_e2.g0_);
}

fn plane_conjugation(self_740: Plane) -> Plane {
    var self_741: Plane;

    self_741 = self_740;
    let _e2: Plane = self_741;
    return Plane((_e2.g0_ * vec3<f32>(-(1.0))));
}

fn plane_dual(self_742: Plane) -> Point {
    var self_743: Plane;

    self_743 = self_742;
    let _e2: Plane = self_743;
    return Point(_e2.g0_);
}

fn plane_scalar_geometric_product(self_744: Plane, other_630: Scalar) -> Plane {
    var self_745: Plane;
    var other_631: Scalar;

    self_745 = self_744;
    other_631 = other_630;
    let _e4: Plane = self_745;
    let _e6: Scalar = other_631;
    return Plane((_e4.g0_ * vec3<f32>(_e6.g0_)));
}

fn plane_scalar_outer_product(self_746: Plane, other_632: Scalar) -> Plane {
    var self_747: Plane;
    var other_633: Scalar;

    self_747 = self_746;
    other_633 = other_632;
    let _e4: Plane = self_747;
    let _e6: Scalar = other_633;
    return Plane((_e4.g0_ * vec3<f32>(_e6.g0_)));
}

fn plane_scalar_inner_product(self_748: Plane, other_634: Scalar) -> Plane {
    var self_749: Plane;
    var other_635: Scalar;

    self_749 = self_748;
    other_635 = other_634;
    let _e4: Plane = self_749;
    let _e6: Scalar = other_635;
    return Plane((_e4.g0_ * vec3<f32>(_e6.g0_)));
}

fn plane_scalar_right_contraction(self_750: Plane, other_636: Scalar) -> Plane {
    var self_751: Plane;
    var other_637: Scalar;

    self_751 = self_750;
    other_637 = other_636;
    let _e4: Plane = self_751;
    let _e6: Scalar = other_637;
    return Plane((_e4.g0_ * vec3<f32>(_e6.g0_)));
}

fn plane_multi_vector_add(self_752: Plane, other_638: MultiVector) -> MultiVector {
    var self_753: Plane;
    var other_639: MultiVector;

    self_753 = self_752;
    other_639 = other_638;
    let _e4: Plane = self_753;
    let _e7: Plane = self_753;
    let _e10: Plane = self_753;
    let _e13: Plane = self_753;
    let _e23: MultiVector = other_639;
    let _e26: Plane = self_753;
    let _e36: MultiVector = other_639;
    return MultiVector(((vec4<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.z, _e13.g0_.y) * vec4<f32>(0.0, 0.0, 1.0, 1.0)) + _e23.g0_), ((vec4<f32>(_e26.g0_.x) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) + _e36.g1_));
}

fn plane_multi_vector_sub(self_754: Plane, other_640: MultiVector) -> MultiVector {
    var self_755: Plane;
    var other_641: MultiVector;

    self_755 = self_754;
    other_641 = other_640;
    let _e4: Plane = self_755;
    let _e7: Plane = self_755;
    let _e10: Plane = self_755;
    let _e13: Plane = self_755;
    let _e23: MultiVector = other_641;
    let _e26: Plane = self_755;
    let _e36: MultiVector = other_641;
    return MultiVector(((vec4<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.z, _e13.g0_.y) * vec4<f32>(0.0, 0.0, 1.0, 1.0)) - _e23.g0_), ((vec4<f32>(_e26.g0_.x) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) - _e36.g1_));
}

fn plane_multi_vector_geometric_product(self_756: Plane, other_642: MultiVector) -> MultiVector {
    var self_757: Plane;
    var other_643: MultiVector;

    self_757 = self_756;
    other_643 = other_642;
    let _e4: Plane = self_757;
    let _e8: MultiVector = other_643;
    let _e18: Plane = self_757;
    let _e22: MultiVector = other_643;
    let _e35: Plane = self_757;
    let _e39: MultiVector = other_643;
    let _e44: Plane = self_757;
    let _e48: MultiVector = other_643;
    let _e58: Plane = self_757;
    let _e62: MultiVector = other_643;
    let _e67: Plane = self_757;
    let _e71: MultiVector = other_643;
    return MultiVector(((((vec4<f32>(_e4.g0_.x) * _e8.g1_) * vec4<f32>(1.0, 1.0, 1.0, -(1.0))) + ((vec4<f32>(_e18.g0_.y) * _e22.g0_.wzyx) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0))) + (vec4<f32>(_e35.g0_.z) * _e39.g0_.zwxy)), ((((vec4<f32>(_e44.g0_.x) * _e48.g0_) * vec4<f32>(1.0, 1.0, 1.0, -(1.0))) + (vec4<f32>(_e58.g0_.y) * _e62.g1_.wzyx)) + ((vec4<f32>(_e67.g0_.z) * _e71.g1_.zwxy) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))));
}

fn plane_multi_vector_scalar_product(self_758: Plane, other_644: MultiVector) -> Scalar {
    var self_759: Plane;
    var other_645: MultiVector;

    self_759 = self_758;
    other_645 = other_644;
    let _e4: Plane = self_759;
    let _e7: MultiVector = other_645;
    let _e11: Plane = self_759;
    let _e14: MultiVector = other_645;
    let _e19: Plane = self_759;
    let _e22: MultiVector = other_645;
    return Scalar((((_e4.g0_.x * _e7.g1_.x) + (_e11.g0_.y * _e14.g0_.w)) + (_e19.g0_.z * _e22.g0_.z)));
}

fn plane_rotor_geometric_product(self_760: Plane, other_646: Rotor) -> MotorDual {
    var self_761: Plane;
    var other_647: Rotor;

    self_761 = self_760;
    other_647 = other_646;
    let _e4: Plane = self_761;
    let _e8: Rotor = other_647;
    let _e11: Rotor = other_647;
    let _e14: Rotor = other_647;
    let _e17: Rotor = other_647;
    let _e28: Plane = self_761;
    let _e31: Plane = self_761;
    let _e34: Plane = self_761;
    let _e37: Plane = self_761;
    let _e41: Rotor = other_647;
    let _e44: Rotor = other_647;
    let _e47: Rotor = other_647;
    let _e50: Rotor = other_647;
    return MotorDual((((vec4<f32>(_e4.g0_.z) * vec4<f32>(_e8.g0_.y, _e11.g0_.y, _e14.g0_.y, _e17.g0_.x)) * vec4<f32>(0.0, 0.0, 1.0, 1.0)) + ((vec4<f32>(_e28.g0_.x, _e31.g0_.x, _e34.g0_.y, _e37.g0_.y) * vec4<f32>(_e41.g0_.y, _e44.g0_.x, _e47.g0_.x, _e50.g0_.y)) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))));
}

fn plane_rotor_regressive_product(self_762: Plane, other_648: Rotor) -> Scalar {
    var self_763: Plane;
    var other_649: Rotor;

    self_763 = self_762;
    other_649 = other_648;
    let _e4: Plane = self_763;
    let _e7: Rotor = other_649;
    return Scalar((_e4.g0_.x * _e7.g0_.y));
}

fn plane_rotor_outer_product(self_764: Plane, other_650: Rotor) -> MotorDual {
    var self_765: Plane;
    var other_651: Rotor;

    self_765 = self_764;
    other_651 = other_650;
    let _e4: Plane = self_765;
    let _e7: Plane = self_765;
    let _e10: Plane = self_765;
    let _e13: Plane = self_765;
    let _e17: Rotor = other_651;
    let _e20: Rotor = other_651;
    let _e23: Rotor = other_651;
    let _e26: Rotor = other_651;
    return MotorDual((vec4<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.y, _e13.g0_.z) * vec4<f32>(_e17.g0_.y, _e20.g0_.x, _e23.g0_.x, _e26.g0_.x)));
}

fn plane_rotor_inner_product(self_766: Plane, other_652: Rotor) -> Plane {
    var self_767: Plane;
    var other_653: Rotor;

    self_767 = self_766;
    other_653 = other_652;
    let _e4: Plane = self_767;
    let _e8: Rotor = other_653;
    let _e11: Rotor = other_653;
    let _e14: Rotor = other_653;
    let _e24: Plane = self_767;
    let _e27: Rotor = other_653;
    let _e30: Rotor = other_653;
    let _e33: Rotor = other_653;
    return Plane((((vec3<f32>(_e4.g0_.z) * vec3<f32>(_e8.g0_.y, _e11.g0_.y, _e14.g0_.x)) * vec3<f32>(0.0, 1.0, 1.0)) + ((_e24.g0_.xyy * vec3<f32>(_e27.g0_.x, _e30.g0_.x, _e33.g0_.y)) * vec3<f32>(1.0, 1.0, -(1.0)))));
}

fn plane_rotor_right_contraction(self_768: Plane, other_654: Rotor) -> Plane {
    var self_769: Plane;
    var other_655: Rotor;

    self_769 = self_768;
    other_655 = other_654;
    let _e4: Plane = self_769;
    let _e6: Rotor = other_655;
    return Plane((_e4.g0_ * vec3<f32>(_e6.g0_.x)));
}

fn plane_point_geometric_product(self_770: Plane, other_656: Point) -> MotorDual {
    var self_771: Plane;
    var other_657: Point;

    self_771 = self_770;
    other_657 = other_656;
    let _e4: Plane = self_771;
    let _e8: Point = other_657;
    let _e11: Point = other_657;
    let _e14: Point = other_657;
    let _e17: Point = other_657;
    let _e29: Plane = self_771;
    let _e33: Point = other_657;
    let _e36: Point = other_657;
    let _e39: Point = other_657;
    let _e42: Point = other_657;
    let _e55: Plane = self_771;
    let _e59: Point = other_657;
    let _e62: Point = other_657;
    let _e65: Point = other_657;
    let _e68: Point = other_657;
    return MotorDual(((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y, _e11.g0_.z, _e14.g0_.y, _e17.g0_.x)) * vec4<f32>(1.0, 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e29.g0_.z) * vec4<f32>(_e33.g0_.z, _e36.g0_.y, _e39.g0_.x, _e42.g0_.z)) * vec4<f32>(1.0, -(1.0), 1.0, 0.0))) + ((vec4<f32>(_e55.g0_.x) * vec4<f32>(_e59.g0_.x, _e62.g0_.x, _e65.g0_.z, _e68.g0_.y)) * vec4<f32>(1.0, 0.0, -(1.0), 1.0))));
}

fn plane_point_regressive_product(self_772: Plane, other_658: Point) -> Scalar {
    var self_773: Plane;
    var other_659: Point;

    self_773 = self_772;
    other_659 = other_658;
    let _e4: Plane = self_773;
    let _e7: Point = other_659;
    let _e11: Plane = self_773;
    let _e14: Point = other_659;
    let _e19: Plane = self_773;
    let _e22: Point = other_659;
    return Scalar((((_e4.g0_.x * _e7.g0_.x) + (_e11.g0_.y * _e14.g0_.y)) + (_e19.g0_.z * _e22.g0_.z)));
}

fn plane_point_inner_product(self_774: Plane, other_660: Point) -> Plane {
    var self_775: Plane;
    var other_661: Point;

    self_775 = self_774;
    other_661 = other_660;
    let _e4: Plane = self_775;
    let _e8: Point = other_661;
    let _e18: Plane = self_775;
    let _e22: Point = other_661;
    let _e33: Plane = self_775;
    let _e37: Point = other_661;
    return Plane(((((vec3<f32>(_e4.g0_.y) * _e8.g0_.zzx) * vec3<f32>(1.0, 0.0, -(1.0))) + ((vec3<f32>(_e18.g0_.z) * _e22.g0_.yxy) * vec3<f32>(-(1.0), 1.0, 0.0))) + ((vec3<f32>(_e33.g0_.x) * _e37.g0_.xzy) * vec3<f32>(0.0, -(1.0), 1.0))));
}

fn plane_point_left_contraction(self_776: Plane, other_662: Point) -> Plane {
    var self_777: Plane;
    var other_663: Point;

    self_777 = self_776;
    other_663 = other_662;
    let _e4: Plane = self_777;
    let _e8: Point = other_663;
    let _e18: Plane = self_777;
    let _e22: Point = other_663;
    let _e33: Plane = self_777;
    let _e37: Point = other_663;
    return Plane(((((vec3<f32>(_e4.g0_.y) * _e8.g0_.zzx) * vec3<f32>(1.0, 0.0, -(1.0))) + ((vec3<f32>(_e18.g0_.z) * _e22.g0_.yxy) * vec3<f32>(-(1.0), 1.0, 0.0))) + ((vec3<f32>(_e33.g0_.x) * _e37.g0_.xzy) * vec3<f32>(0.0, -(1.0), 1.0))));
}

fn plane_ideal_point_geometric_product(self_778: Plane, other_664: IdealPoint) -> MotorDual {
    var self_779: Plane;
    var other_665: IdealPoint;

    self_779 = self_778;
    other_665 = other_664;
    let _e4: Plane = self_779;
    let _e8: IdealPoint = other_665;
    let _e11: IdealPoint = other_665;
    let _e14: IdealPoint = other_665;
    let _e17: IdealPoint = other_665;
    let _e29: Plane = self_779;
    let _e32: Plane = self_779;
    let _e35: Plane = self_779;
    let _e38: Plane = self_779;
    let _e42: IdealPoint = other_665;
    let _e45: IdealPoint = other_665;
    let _e48: IdealPoint = other_665;
    let _e51: IdealPoint = other_665;
    return MotorDual((((vec4<f32>(_e4.g0_.z) * vec4<f32>(_e8.g0_.y, _e11.g0_.x, _e14.g0_.y, _e17.g0_.y)) * vec4<f32>(1.0, -(1.0), 0.0, 0.0)) + ((vec4<f32>(_e29.g0_.y, _e32.g0_.y, _e35.g0_.x, _e38.g0_.x) * vec4<f32>(_e42.g0_.x, _e45.g0_.y, _e48.g0_.y, _e51.g0_.x)) * vec4<f32>(1.0, 1.0, -(1.0), 1.0))));
}

fn plane_ideal_point_regressive_product(self_780: Plane, other_666: IdealPoint) -> Scalar {
    var self_781: Plane;
    var other_667: IdealPoint;

    self_781 = self_780;
    other_667 = other_666;
    let _e4: Plane = self_781;
    let _e7: IdealPoint = other_667;
    let _e11: Plane = self_781;
    let _e14: IdealPoint = other_667;
    return Scalar(((_e4.g0_.y * _e7.g0_.x) + (_e11.g0_.z * _e14.g0_.y)));
}

fn plane_ideal_point_inner_product(self_782: Plane, other_668: IdealPoint) -> Plane {
    var self_783: Plane;
    var other_669: IdealPoint;

    self_783 = self_782;
    other_669 = other_668;
    let _e4: Plane = self_783;
    let _e8: IdealPoint = other_669;
    let _e19: Plane = self_783;
    let _e22: IdealPoint = other_669;
    let _e25: IdealPoint = other_669;
    let _e28: IdealPoint = other_669;
    return Plane((((vec3<f32>(_e4.g0_.z) * vec3<f32>(_e8.g0_.x)) * vec3<f32>(-(1.0), 0.0, 0.0)) + ((_e19.g0_.yxx * vec3<f32>(_e22.g0_.y, _e25.g0_.y, _e28.g0_.x)) * vec3<f32>(1.0, -(1.0), 1.0))));
}

fn plane_ideal_point_left_contraction(self_784: Plane, other_670: IdealPoint) -> Plane {
    var self_785: Plane;
    var other_671: IdealPoint;

    self_785 = self_784;
    other_671 = other_670;
    let _e4: Plane = self_785;
    let _e8: IdealPoint = other_671;
    let _e19: Plane = self_785;
    let _e22: IdealPoint = other_671;
    let _e25: IdealPoint = other_671;
    let _e28: IdealPoint = other_671;
    return Plane((((vec3<f32>(_e4.g0_.z) * vec3<f32>(_e8.g0_.x)) * vec3<f32>(-(1.0), 0.0, 0.0)) + ((_e19.g0_.yxx * vec3<f32>(_e22.g0_.y, _e25.g0_.y, _e28.g0_.x)) * vec3<f32>(1.0, -(1.0), 1.0))));
}

fn plane_plane_add(self_786: Plane, other_672: Plane) -> Plane {
    var self_787: Plane;
    var other_673: Plane;

    self_787 = self_786;
    other_673 = other_672;
    let _e4: Plane = self_787;
    let _e6: Plane = other_673;
    return Plane((_e4.g0_ + _e6.g0_));
}

fn plane_plane_sub(self_788: Plane, other_674: Plane) -> Plane {
    var self_789: Plane;
    var other_675: Plane;

    self_789 = self_788;
    other_675 = other_674;
    let _e4: Plane = self_789;
    let _e6: Plane = other_675;
    return Plane((_e4.g0_ - _e6.g0_));
}

fn plane_plane_mul(self_790: Plane, other_676: Plane) -> Plane {
    var self_791: Plane;
    var other_677: Plane;

    self_791 = self_790;
    other_677 = other_676;
    let _e4: Plane = self_791;
    let _e6: Plane = other_677;
    return Plane((_e4.g0_ * _e6.g0_));
}

fn plane_plane_div(self_792: Plane, other_678: Plane) -> Plane {
    var self_793: Plane;
    var other_679: Plane;

    self_793 = self_792;
    other_679 = other_678;
    let _e4: Plane = self_793;
    let _e7: Plane = self_793;
    let _e10: Plane = self_793;
    let _e19: Plane = other_679;
    let _e22: Plane = other_679;
    let _e25: Plane = other_679;
    return Plane((((vec3<f32>(_e4.g0_.x, _e7.g0_.y, _e10.g0_.z) * vec3<f32>(1.0, 1.0, 1.0)) / vec3<f32>(_e19.g0_.x, _e22.g0_.y, _e25.g0_.z)) * vec3<f32>(1.0, 1.0, 1.0)));
}

fn plane_plane_geometric_product(self_794: Plane, other_680: Plane) -> Motor {
    var self_795: Plane;
    var other_681: Plane;

    self_795 = self_794;
    other_681 = other_680;
    let _e4: Plane = self_795;
    let _e8: Plane = other_681;
    let _e11: Plane = other_681;
    let _e14: Plane = other_681;
    let _e17: Plane = other_681;
    let _e29: Plane = self_795;
    let _e33: Plane = other_681;
    let _e36: Plane = other_681;
    let _e39: Plane = other_681;
    let _e42: Plane = other_681;
    let _e55: Plane = self_795;
    let _e59: Plane = other_681;
    let _e62: Plane = other_681;
    let _e65: Plane = other_681;
    let _e68: Plane = other_681;
    return Motor(((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y, _e11.g0_.z, _e14.g0_.y, _e17.g0_.x)) * vec4<f32>(1.0, -(1.0), 0.0, 1.0)) + ((vec4<f32>(_e29.g0_.z) * vec4<f32>(_e33.g0_.z, _e36.g0_.y, _e39.g0_.x, _e42.g0_.z)) * vec4<f32>(1.0, 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e55.g0_.x) * vec4<f32>(_e59.g0_.x, _e62.g0_.x, _e65.g0_.z, _e68.g0_.y)) * vec4<f32>(1.0, 0.0, 1.0, -(1.0)))));
}

fn plane_plane_outer_product(self_796: Plane, other_682: Plane) -> Point {
    var self_797: Plane;
    var other_683: Plane;

    self_797 = self_796;
    other_683 = other_682;
    let _e4: Plane = self_797;
    let _e8: Plane = other_683;
    let _e18: Plane = self_797;
    let _e22: Plane = other_683;
    let _e33: Plane = self_797;
    let _e37: Plane = other_683;
    return Point(((((vec3<f32>(_e4.g0_.y) * _e8.g0_.zzx) * vec3<f32>(-(1.0), 0.0, 1.0)) + ((vec3<f32>(_e18.g0_.z) * _e22.g0_.yxy) * vec3<f32>(1.0, -(1.0), 0.0))) + ((vec3<f32>(_e33.g0_.x) * _e37.g0_.xzy) * vec3<f32>(0.0, 1.0, -(1.0)))));
}

fn plane_plane_inner_product(self_798: Plane, other_684: Plane) -> Scalar {
    var self_799: Plane;
    var other_685: Plane;

    self_799 = self_798;
    other_685 = other_684;
    let _e4: Plane = self_799;
    let _e7: Plane = other_685;
    let _e11: Plane = self_799;
    let _e14: Plane = other_685;
    let _e19: Plane = self_799;
    let _e22: Plane = other_685;
    return Scalar((((_e4.g0_.x * _e7.g0_.x) + (_e11.g0_.y * _e14.g0_.y)) + (_e19.g0_.z * _e22.g0_.z)));
}

fn plane_plane_left_contraction(self_800: Plane, other_686: Plane) -> Scalar {
    var self_801: Plane;
    var other_687: Plane;

    self_801 = self_800;
    other_687 = other_686;
    let _e4: Plane = self_801;
    let _e7: Plane = other_687;
    let _e11: Plane = self_801;
    let _e14: Plane = other_687;
    let _e19: Plane = self_801;
    let _e22: Plane = other_687;
    return Scalar((((_e4.g0_.x * _e7.g0_.x) + (_e11.g0_.y * _e14.g0_.y)) + (_e19.g0_.z * _e22.g0_.z)));
}

fn plane_plane_right_contraction(self_802: Plane, other_688: Plane) -> Scalar {
    var self_803: Plane;
    var other_689: Plane;

    self_803 = self_802;
    other_689 = other_688;
    let _e4: Plane = self_803;
    let _e7: Plane = other_689;
    let _e11: Plane = self_803;
    let _e14: Plane = other_689;
    let _e19: Plane = self_803;
    let _e22: Plane = other_689;
    return Scalar((((_e4.g0_.x * _e7.g0_.x) + (_e11.g0_.y * _e14.g0_.y)) + (_e19.g0_.z * _e22.g0_.z)));
}

fn plane_plane_scalar_product(self_804: Plane, other_690: Plane) -> Scalar {
    var self_805: Plane;
    var other_691: Plane;

    self_805 = self_804;
    other_691 = other_690;
    let _e4: Plane = self_805;
    let _e7: Plane = other_691;
    let _e11: Plane = self_805;
    let _e14: Plane = other_691;
    let _e19: Plane = self_805;
    let _e22: Plane = other_691;
    return Scalar((((_e4.g0_.x * _e7.g0_.x) + (_e11.g0_.y * _e14.g0_.y)) + (_e19.g0_.z * _e22.g0_.z)));
}

fn plane_translator_geometric_product(self_806: Plane, other_692: Translator) -> MotorDual {
    var self_807: Plane;
    var other_693: Translator;

    self_807 = self_806;
    other_693 = other_692;
    let _e4: Plane = self_807;
    let _e8: Translator = other_693;
    let _e11: Translator = other_693;
    let _e14: Translator = other_693;
    let _e17: Translator = other_693;
    let _e28: Plane = self_807;
    let _e32: Translator = other_693;
    let _e35: Translator = other_693;
    let _e38: Translator = other_693;
    let _e41: Translator = other_693;
    let _e54: Plane = self_807;
    let _e58: Translator = other_693;
    let _e61: Translator = other_693;
    let _e64: Translator = other_693;
    let _e67: Translator = other_693;
    return MotorDual(((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y, _e11.g0_.z, _e14.g0_.x, _e17.g0_.y)) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) + ((vec4<f32>(_e28.g0_.z) * vec4<f32>(_e32.g0_.z, _e35.g0_.y, _e38.g0_.z, _e41.g0_.x)) * vec4<f32>(1.0, -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e54.g0_.x) * vec4<f32>(_e58.g0_.x, _e61.g0_.x, _e64.g0_.z, _e67.g0_.y)) * vec4<f32>(0.0, 1.0, -(1.0), 1.0))));
}

fn plane_translator_regressive_product(self_808: Plane, other_694: Translator) -> Scalar {
    var self_809: Plane;
    var other_695: Translator;

    self_809 = self_808;
    other_695 = other_694;
    let _e4: Plane = self_809;
    let _e7: Translator = other_695;
    let _e11: Plane = self_809;
    let _e14: Translator = other_695;
    return Scalar(((_e4.g0_.y * _e7.g0_.y) + (_e11.g0_.z * _e14.g0_.z)));
}

fn plane_translator_outer_product(self_810: Plane, other_696: Translator) -> MotorDual {
    var self_811: Plane;
    var other_697: Translator;

    self_811 = self_810;
    other_697 = other_696;
    let _e4: Plane = self_811;
    let _e8: Translator = other_697;
    let _e11: Translator = other_697;
    let _e14: Translator = other_697;
    let _e17: Translator = other_697;
    let _e28: Plane = self_811;
    let _e31: Plane = self_811;
    let _e34: Plane = self_811;
    let _e37: Plane = self_811;
    let _e41: Translator = other_697;
    let _e44: Translator = other_697;
    let _e47: Translator = other_697;
    let _e50: Translator = other_697;
    return MotorDual((((vec4<f32>(_e4.g0_.z) * vec4<f32>(_e8.g0_.z, _e11.g0_.z, _e14.g0_.z, _e17.g0_.x)) * vec4<f32>(1.0, 0.0, 0.0, 1.0)) + ((vec4<f32>(_e28.g0_.y, _e31.g0_.x, _e34.g0_.y, _e37.g0_.x) * vec4<f32>(_e41.g0_.y, _e44.g0_.x, _e47.g0_.x, _e50.g0_.x)) * vec4<f32>(1.0, 1.0, 1.0, 0.0))));
}

fn plane_translator_inner_product(self_812: Plane, other_698: Translator) -> Plane {
    var self_813: Plane;
    var other_699: Translator;

    self_813 = self_812;
    other_699 = other_698;
    let _e4: Plane = self_813;
    let _e8: Translator = other_699;
    let _e18: Plane = self_813;
    let _e22: Translator = other_699;
    let _e33: Plane = self_813;
    let _e36: Translator = other_699;
    return Plane(((((vec3<f32>(_e4.g0_.x) * _e8.g0_.xzy) * vec3<f32>(1.0, -(1.0), 1.0)) + ((vec3<f32>(_e18.g0_.z) * _e22.g0_.yyx) * vec3<f32>(-(1.0), 0.0, 1.0))) + ((_e33.g0_.yyx * _e36.g0_.zxx) * vec3<f32>(1.0, 1.0, 0.0))));
}

fn plane_translator_left_contraction(self_814: Plane, other_700: Translator) -> Plane {
    var self_815: Plane;
    var other_701: Translator;

    self_815 = self_814;
    other_701 = other_700;
    let _e4: Plane = self_815;
    let _e8: Translator = other_701;
    let _e19: Plane = self_815;
    let _e22: Translator = other_701;
    return Plane((((vec3<f32>(_e4.g0_.z) * vec3<f32>(_e8.g0_.y)) * vec3<f32>(-(1.0), 0.0, 0.0)) + ((_e19.g0_.yxx * _e22.g0_.zzy) * vec3<f32>(1.0, -(1.0), 1.0))));
}

fn plane_translator_right_contraction(self_816: Plane, other_702: Translator) -> Plane {
    var self_817: Plane;
    var other_703: Translator;

    self_817 = self_816;
    other_703 = other_702;
    let _e4: Plane = self_817;
    let _e6: Translator = other_703;
    return Plane((_e4.g0_ * vec3<f32>(_e6.g0_.x)));
}

fn plane_motor_geometric_product(self_818: Plane, other_704: Motor) -> MotorDual {
    var self_819: Plane;
    var other_705: Motor;

    self_819 = self_818;
    other_705 = other_704;
    let _e4: Plane = self_819;
    let _e8: Motor = other_705;
    let _e19: Plane = self_819;
    let _e23: Motor = other_705;
    let _e35: Plane = self_819;
    let _e39: Motor = other_705;
    return MotorDual(((((vec4<f32>(_e4.g0_.x) * _e8.g0_.yxwz) * vec4<f32>(1.0, 1.0, -(1.0), 1.0)) + ((vec4<f32>(_e19.g0_.y) * _e23.g0_.zwxy) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e35.g0_.z) * _e39.g0_.wzyx) * vec4<f32>(1.0, -(1.0), 1.0, 1.0))));
}

fn plane_motor_regressive_product(self_820: Plane, other_706: Motor) -> Scalar {
    var self_821: Plane;
    var other_707: Motor;

    self_821 = self_820;
    other_707 = other_706;
    let _e4: Plane = self_821;
    let _e7: Motor = other_707;
    let _e11: Plane = self_821;
    let _e14: Motor = other_707;
    let _e19: Plane = self_821;
    let _e22: Motor = other_707;
    return Scalar((((_e4.g0_.x * _e7.g0_.y) + (_e11.g0_.y * _e14.g0_.z)) + (_e19.g0_.z * _e22.g0_.w)));
}

fn plane_motor_outer_product(self_822: Plane, other_708: Motor) -> MotorDual {
    var self_823: Plane;
    var other_709: Motor;

    self_823 = self_822;
    other_709 = other_708;
    let _e4: Plane = self_823;
    let _e8: Motor = other_709;
    let _e18: Plane = self_823;
    let _e22: Motor = other_709;
    let _e33: Plane = self_823;
    let _e37: Motor = other_709;
    return MotorDual(((((vec4<f32>(_e4.g0_.y) * _e8.g0_.zzxz) * vec4<f32>(1.0, 0.0, 1.0, 0.0)) + ((vec4<f32>(_e18.g0_.z) * _e22.g0_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e33.g0_.x) * _e37.g0_.yxxx) * vec4<f32>(1.0, 1.0, 0.0, 0.0))));
}

fn plane_motor_inner_product(self_824: Plane, other_710: Motor) -> Plane {
    var self_825: Plane;
    var other_711: Motor;

    self_825 = self_824;
    other_711 = other_710;
    let _e4: Plane = self_825;
    let _e8: Motor = other_711;
    let _e11: Motor = other_711;
    let _e14: Motor = other_711;
    let _e25: Plane = self_825;
    let _e29: Motor = other_711;
    let _e32: Motor = other_711;
    let _e35: Motor = other_711;
    let _e47: Plane = self_825;
    let _e51: Motor = other_711;
    let _e54: Motor = other_711;
    let _e57: Motor = other_711;
    return Plane(((((vec3<f32>(_e4.g0_.x) * vec3<f32>(_e8.g0_.x, _e11.g0_.w, _e14.g0_.z)) * vec3<f32>(1.0, -(1.0), 1.0)) + ((vec3<f32>(_e25.g0_.y) * vec3<f32>(_e29.g0_.w, _e32.g0_.x, _e35.g0_.y)) * vec3<f32>(1.0, 1.0, -(1.0)))) + ((vec3<f32>(_e47.g0_.z) * vec3<f32>(_e51.g0_.z, _e54.g0_.y, _e57.g0_.x)) * vec3<f32>(-(1.0), 1.0, 1.0))));
}

fn plane_motor_left_contraction(self_826: Plane, other_712: Motor) -> Plane {
    var self_827: Plane;
    var other_713: Motor;

    self_827 = self_826;
    other_713 = other_712;
    let _e4: Plane = self_827;
    let _e8: Motor = other_713;
    let _e11: Motor = other_713;
    let _e14: Motor = other_713;
    let _e25: Plane = self_827;
    let _e29: Motor = other_713;
    let _e32: Motor = other_713;
    let _e35: Motor = other_713;
    let _e47: Plane = self_827;
    let _e51: Motor = other_713;
    let _e54: Motor = other_713;
    let _e57: Motor = other_713;
    return Plane(((((vec3<f32>(_e4.g0_.y) * vec3<f32>(_e8.g0_.w, _e11.g0_.w, _e14.g0_.y)) * vec3<f32>(1.0, 0.0, -(1.0))) + ((vec3<f32>(_e25.g0_.z) * vec3<f32>(_e29.g0_.z, _e32.g0_.y, _e35.g0_.z)) * vec3<f32>(-(1.0), 1.0, 0.0))) + ((vec3<f32>(_e47.g0_.x) * vec3<f32>(_e51.g0_.x, _e54.g0_.w, _e57.g0_.z)) * vec3<f32>(0.0, -(1.0), 1.0))));
}

fn plane_motor_right_contraction(self_828: Plane, other_714: Motor) -> Plane {
    var self_829: Plane;
    var other_715: Motor;

    self_829 = self_828;
    other_715 = other_714;
    let _e4: Plane = self_829;
    let _e6: Motor = other_715;
    return Plane((_e4.g0_ * vec3<f32>(_e6.g0_.x)));
}

fn plane_motor_dual_add(self_830: Plane, other_716: MotorDual) -> MotorDual {
    var self_831: Plane;
    var other_717: MotorDual;

    self_831 = self_830;
    other_717 = other_716;
    let _e4: Plane = self_831;
    let _e7: Plane = self_831;
    let _e10: Plane = self_831;
    let _e13: Plane = self_831;
    let _e23: MotorDual = other_717;
    return MotorDual(((vec4<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.y, _e13.g0_.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0)) + _e23.g0_));
}

fn plane_motor_dual_sub(self_832: Plane, other_718: MotorDual) -> MotorDual {
    var self_833: Plane;
    var other_719: MotorDual;

    self_833 = self_832;
    other_719 = other_718;
    let _e4: Plane = self_833;
    let _e7: Plane = self_833;
    let _e10: Plane = self_833;
    let _e13: Plane = self_833;
    let _e23: MotorDual = other_719;
    return MotorDual(((vec4<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.y, _e13.g0_.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0)) - _e23.g0_));
}

fn plane_motor_dual_geometric_product(self_834: Plane, other_720: MotorDual) -> Motor {
    var self_835: Plane;
    var other_721: MotorDual;

    self_835 = self_834;
    other_721 = other_720;
    let _e4: Plane = self_835;
    let _e8: MotorDual = other_721;
    let _e19: Plane = self_835;
    let _e23: MotorDual = other_721;
    let _e35: Plane = self_835;
    let _e39: MotorDual = other_721;
    return Motor(((((vec4<f32>(_e4.g0_.x) * _e8.g0_.yxwz) * vec4<f32>(1.0, 1.0, 1.0, -(1.0))) + ((vec4<f32>(_e19.g0_.y) * _e23.g0_.zwxy) * vec4<f32>(1.0, -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e35.g0_.z) * _e39.g0_.wzyx) * vec4<f32>(1.0, 1.0, -(1.0), 1.0))));
}

fn plane_motor_dual_regressive_product(self_836: Plane, other_722: MotorDual) -> Plane {
    var self_837: Plane;
    var other_723: MotorDual;

    self_837 = self_836;
    other_723 = other_722;
    let _e4: Plane = self_837;
    let _e6: MotorDual = other_723;
    return Plane((_e4.g0_ * vec3<f32>(_e6.g0_.x)));
}

fn plane_motor_dual_outer_product(self_838: Plane, other_724: MotorDual) -> Point {
    var self_839: Plane;
    var other_725: MotorDual;

    self_839 = self_838;
    other_725 = other_724;
    let _e4: Plane = self_839;
    let _e8: MotorDual = other_725;
    let _e11: MotorDual = other_725;
    let _e14: MotorDual = other_725;
    let _e25: Plane = self_839;
    let _e29: MotorDual = other_725;
    let _e32: MotorDual = other_725;
    let _e35: MotorDual = other_725;
    let _e47: Plane = self_839;
    let _e51: MotorDual = other_725;
    let _e54: MotorDual = other_725;
    let _e57: MotorDual = other_725;
    return Point(((((vec3<f32>(_e4.g0_.y) * vec3<f32>(_e8.g0_.w, _e11.g0_.w, _e14.g0_.y)) * vec3<f32>(-(1.0), 0.0, 1.0)) + ((vec3<f32>(_e25.g0_.z) * vec3<f32>(_e29.g0_.z, _e32.g0_.y, _e35.g0_.z)) * vec3<f32>(1.0, -(1.0), 0.0))) + ((vec3<f32>(_e47.g0_.x) * vec3<f32>(_e51.g0_.x, _e54.g0_.w, _e57.g0_.z)) * vec3<f32>(0.0, 1.0, -(1.0)))));
}

fn plane_motor_dual_inner_product(self_840: Plane, other_726: MotorDual) -> Motor {
    var self_841: Plane;
    var other_727: MotorDual;

    self_841 = self_840;
    other_727 = other_726;
    let _e4: Plane = self_841;
    let _e8: MotorDual = other_727;
    let _e18: Plane = self_841;
    let _e22: MotorDual = other_727;
    let _e33: Plane = self_841;
    let _e37: MotorDual = other_727;
    return Motor(((((vec4<f32>(_e4.g0_.y) * _e8.g0_.zzxz) * vec4<f32>(1.0, 0.0, 1.0, 0.0)) + ((vec4<f32>(_e18.g0_.z) * _e22.g0_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e33.g0_.x) * _e37.g0_.yxxx) * vec4<f32>(1.0, 1.0, 0.0, 0.0))));
}

fn plane_motor_dual_left_contraction(self_842: Plane, other_728: MotorDual) -> Motor {
    var self_843: Plane;
    var other_729: MotorDual;

    self_843 = self_842;
    other_729 = other_728;
    let _e4: Plane = self_843;
    let _e8: MotorDual = other_729;
    let _e18: Plane = self_843;
    let _e22: MotorDual = other_729;
    let _e33: Plane = self_843;
    let _e37: MotorDual = other_729;
    return Motor(((((vec4<f32>(_e4.g0_.y) * _e8.g0_.zzxz) * vec4<f32>(1.0, 0.0, 1.0, 0.0)) + ((vec4<f32>(_e18.g0_.z) * _e22.g0_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e33.g0_.x) * _e37.g0_.yxxx) * vec4<f32>(1.0, 1.0, 0.0, 0.0))));
}

fn plane_motor_dual_right_contraction(self_844: Plane, other_730: MotorDual) -> Scalar {
    var self_845: Plane;
    var other_731: MotorDual;

    self_845 = self_844;
    other_731 = other_730;
    let _e4: Plane = self_845;
    let _e7: MotorDual = other_731;
    let _e11: Plane = self_845;
    let _e14: MotorDual = other_731;
    let _e19: Plane = self_845;
    let _e22: MotorDual = other_731;
    return Scalar((((_e4.g0_.x * _e7.g0_.y) + (_e11.g0_.y * _e14.g0_.z)) + (_e19.g0_.z * _e22.g0_.w)));
}

fn plane_motor_dual_scalar_product(self_846: Plane, other_732: MotorDual) -> Scalar {
    var self_847: Plane;
    var other_733: MotorDual;

    self_847 = self_846;
    other_733 = other_732;
    let _e4: Plane = self_847;
    let _e7: MotorDual = other_733;
    let _e11: Plane = self_847;
    let _e14: MotorDual = other_733;
    let _e19: Plane = self_847;
    let _e22: MotorDual = other_733;
    return Scalar((((_e4.g0_.x * _e7.g0_.y) + (_e11.g0_.y * _e14.g0_.z)) + (_e19.g0_.z * _e22.g0_.w)));
}

fn plane_squared_magnitude(self_848: Plane) -> Scalar {
    var self_849: Plane;

    self_849 = self_848;
    let _e2: Plane = self_849;
    let _e3: Plane = self_849;
    let _e4: Plane = plane_reversal(_e3);
    let _e5: Scalar = plane_plane_scalar_product(_e2, _e4);
    return _e5;
}

fn plane_magnitude(self_850: Plane) -> Scalar {
    var self_851: Plane;

    self_851 = self_850;
    let _e2: Plane = self_851;
    let _e3: Scalar = plane_squared_magnitude(_e2);
    return Scalar(sqrt(_e3.g0_));
}

fn plane_scale(self_852: Plane, other_734: f32) -> Plane {
    var self_853: Plane;
    var other_735: f32;

    self_853 = self_852;
    other_735 = other_734;
    let _e4: Plane = self_853;
    let _e5: f32 = other_735;
    let _e7: Plane = plane_scalar_geometric_product(_e4, Scalar(_e5));
    return _e7;
}

fn plane_signum(self_854: Plane) -> Plane {
    var self_855: Plane;

    self_855 = self_854;
    let _e2: Plane = self_855;
    let _e3: Plane = self_855;
    let _e4: Scalar = plane_magnitude(_e3);
    let _e9: Plane = plane_scalar_geometric_product(_e2, Scalar((1.0 / _e4.g0_)));
    return _e9;
}

fn plane_inverse(self_856: Plane) -> Plane {
    var self_857: Plane;

    self_857 = self_856;
    let _e2: Plane = self_857;
    let _e3: Plane = plane_reversal(_e2);
    let _e4: Plane = self_857;
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

fn translator_neg(self_858: Translator) -> Translator {
    var self_859: Translator;

    self_859 = self_858;
    let _e2: Translator = self_859;
    return Translator((_e2.g0_ * vec3<f32>(-(1.0))));
}

fn translator_automorphism(self_860: Translator) -> Translator {
    var self_861: Translator;

    self_861 = self_860;
    let _e2: Translator = self_861;
    return Translator(_e2.g0_);
}

fn translator_reversal(self_862: Translator) -> Translator {
    var self_863: Translator;

    self_863 = self_862;
    let _e2: Translator = self_863;
    return Translator((_e2.g0_ * vec3<f32>(1.0, -(1.0), -(1.0))));
}

fn translator_conjugation(self_864: Translator) -> Translator {
    var self_865: Translator;

    self_865 = self_864;
    let _e2: Translator = self_865;
    return Translator((_e2.g0_ * vec3<f32>(1.0, -(1.0), -(1.0))));
}

fn translator_scalar_into(self_866: Translator) -> Scalar {
    var self_867: Translator;

    self_867 = self_866;
    let _e2: Translator = self_867;
    return Scalar(_e2.g0_.x);
}

fn translator_scalar_add(self_868: Translator, other_736: Scalar) -> Translator {
    var self_869: Translator;
    var other_737: Scalar;

    self_869 = self_868;
    other_737 = other_736;
    let _e4: Translator = self_869;
    let _e6: Scalar = other_737;
    return Translator((_e4.g0_ + (vec3<f32>(_e6.g0_) * vec3<f32>(1.0, 0.0, 0.0))));
}

fn translator_scalar_sub(self_870: Translator, other_738: Scalar) -> Translator {
    var self_871: Translator;
    var other_739: Scalar;

    self_871 = self_870;
    other_739 = other_738;
    let _e4: Translator = self_871;
    let _e6: Scalar = other_739;
    return Translator((_e4.g0_ - (vec3<f32>(_e6.g0_) * vec3<f32>(1.0, 0.0, 0.0))));
}

fn translator_scalar_geometric_product(self_872: Translator, other_740: Scalar) -> Translator {
    var self_873: Translator;
    var other_741: Scalar;

    self_873 = self_872;
    other_741 = other_740;
    let _e4: Translator = self_873;
    let _e6: Scalar = other_741;
    return Translator((_e4.g0_ * vec3<f32>(_e6.g0_)));
}

fn translator_scalar_outer_product(self_874: Translator, other_742: Scalar) -> Translator {
    var self_875: Translator;
    var other_743: Scalar;

    self_875 = self_874;
    other_743 = other_742;
    let _e4: Translator = self_875;
    let _e6: Scalar = other_743;
    return Translator((_e4.g0_ * vec3<f32>(_e6.g0_)));
}

fn translator_scalar_inner_product(self_876: Translator, other_744: Scalar) -> Translator {
    var self_877: Translator;
    var other_745: Scalar;

    self_877 = self_876;
    other_745 = other_744;
    let _e4: Translator = self_877;
    let _e6: Scalar = other_745;
    return Translator((_e4.g0_ * vec3<f32>(_e6.g0_)));
}

fn translator_scalar_left_contraction(self_878: Translator, other_746: Scalar) -> Scalar {
    var self_879: Translator;
    var other_747: Scalar;

    self_879 = self_878;
    other_747 = other_746;
    let _e4: Translator = self_879;
    let _e7: Scalar = other_747;
    return Scalar((_e4.g0_.x * _e7.g0_));
}

fn translator_scalar_right_contraction(self_880: Translator, other_748: Scalar) -> Translator {
    var self_881: Translator;
    var other_749: Scalar;

    self_881 = self_880;
    other_749 = other_748;
    let _e4: Translator = self_881;
    let _e6: Scalar = other_749;
    return Translator((_e4.g0_ * vec3<f32>(_e6.g0_)));
}

fn translator_scalar_scalar_product(self_882: Translator, other_750: Scalar) -> Scalar {
    var self_883: Translator;
    var other_751: Scalar;

    self_883 = self_882;
    other_751 = other_750;
    let _e4: Translator = self_883;
    let _e7: Scalar = other_751;
    return Scalar((_e4.g0_.x * _e7.g0_));
}

fn translator_multi_vector_add(self_884: Translator, other_752: MultiVector) -> MultiVector {
    var self_885: Translator;
    var other_753: MultiVector;

    self_885 = self_884;
    other_753 = other_752;
    let _e4: Translator = self_885;
    let _e14: MultiVector = other_753;
    let _e17: Translator = self_885;
    let _e20: Translator = self_885;
    let _e23: Translator = self_885;
    let _e26: Translator = self_885;
    let _e36: MultiVector = other_753;
    return MultiVector(((vec4<f32>(_e4.g0_.x) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) + _e14.g0_), ((vec4<f32>(_e17.g0_.x, _e20.g0_.x, _e23.g0_.y, _e26.g0_.z) * vec4<f32>(0.0, 0.0, 1.0, 1.0)) + _e36.g1_));
}

fn translator_multi_vector_sub(self_886: Translator, other_754: MultiVector) -> MultiVector {
    var self_887: Translator;
    var other_755: MultiVector;

    self_887 = self_886;
    other_755 = other_754;
    let _e4: Translator = self_887;
    let _e14: MultiVector = other_755;
    let _e17: Translator = self_887;
    let _e20: Translator = self_887;
    let _e23: Translator = self_887;
    let _e26: Translator = self_887;
    let _e36: MultiVector = other_755;
    return MultiVector(((vec4<f32>(_e4.g0_.x) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) - _e14.g0_), ((vec4<f32>(_e17.g0_.x, _e20.g0_.x, _e23.g0_.y, _e26.g0_.z) * vec4<f32>(0.0, 0.0, 1.0, 1.0)) - _e36.g1_));
}

fn translator_multi_vector_geometric_product(self_888: Translator, other_756: MultiVector) -> MultiVector {
    var self_889: Translator;
    var other_757: MultiVector;

    self_889 = self_888;
    other_757 = other_756;
    let _e4: Translator = self_889;
    let _e8: MultiVector = other_757;
    let _e11: Translator = self_889;
    let _e15: MultiVector = other_757;
    let _e29: Translator = self_889;
    let _e33: MultiVector = other_757;
    let _e47: Translator = self_889;
    let _e51: MultiVector = other_757;
    let _e54: Translator = self_889;
    let _e58: MultiVector = other_757;
    let _e70: Translator = self_889;
    let _e74: MultiVector = other_757;
    return MultiVector((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.y) * _e15.g1_.zwxy) * vec4<f32>(-(1.0), 1.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e29.g0_.z) * _e33.g1_.wzyx) * vec4<f32>(-(1.0), -(1.0), -(1.0), 1.0))), (((vec4<f32>(_e47.g0_.x) * _e51.g1_) + ((vec4<f32>(_e54.g0_.y) * _e58.g0_.zwxy) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e70.g0_.z) * _e74.g0_.wzyx) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0))));
}

fn translator_multi_vector_outer_product(self_890: Translator, other_758: MultiVector) -> MultiVector {
    var self_891: Translator;
    var other_759: MultiVector;

    self_891 = self_890;
    other_759 = other_758;
    let _e4: Translator = self_891;
    let _e8: MultiVector = other_759;
    let _e11: Translator = self_891;
    let _e15: MultiVector = other_759;
    let _e18: Translator = self_891;
    let _e22: MultiVector = other_759;
    let _e33: Translator = self_891;
    let _e36: Translator = self_891;
    let _e39: Translator = self_891;
    let _e42: Translator = self_891;
    let _e46: MultiVector = other_759;
    return MultiVector((vec4<f32>(_e4.g0_.x) * _e8.g0_), (((vec4<f32>(_e11.g0_.x) * _e15.g1_) + ((vec4<f32>(_e18.g0_.z) * _e22.g0_.zzzx) * vec4<f32>(0.0, 1.0, 0.0, 1.0))) + ((vec4<f32>(_e33.g0_.x, _e36.g0_.y, _e39.g0_.y, _e42.g0_.x) * _e46.g0_.xwxx) * vec4<f32>(0.0, 1.0, 1.0, 0.0))));
}

fn translator_multi_vector_inner_product(self_892: Translator, other_760: MultiVector) -> MultiVector {
    var self_893: Translator;
    var other_761: MultiVector;

    self_893 = self_892;
    other_761 = other_760;
    let _e4: Translator = self_893;
    let _e8: MultiVector = other_761;
    let _e11: Translator = self_893;
    let _e15: MultiVector = other_761;
    let _e28: Translator = self_893;
    let _e31: Translator = self_893;
    let _e34: Translator = self_893;
    let _e37: Translator = self_893;
    let _e41: MultiVector = other_761;
    let _e55: Translator = self_893;
    let _e59: MultiVector = other_761;
    let _e62: Translator = self_893;
    let _e66: MultiVector = other_761;
    let _e78: Translator = self_893;
    let _e81: Translator = self_893;
    let _e84: Translator = self_893;
    let _e87: Translator = self_893;
    let _e91: MultiVector = other_761;
    return MultiVector((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.z) * _e15.g1_.wwyx) * vec4<f32>(-(1.0), 0.0, -(1.0), 1.0))) + ((vec4<f32>(_e28.g0_.y, _e31.g0_.x, _e34.g0_.y, _e37.g0_.y) * _e41.g1_.zxxy) * vec4<f32>(-(1.0), 0.0, -(1.0), -(1.0)))), (((vec4<f32>(_e55.g0_.x) * _e59.g1_) + ((vec4<f32>(_e62.g0_.z) * _e66.g0_.wwwx) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))) + ((vec4<f32>(_e78.g0_.y, _e81.g0_.x, _e84.g0_.y, _e87.g0_.x) * _e91.g0_.zxxx) * vec4<f32>(1.0, 0.0, 1.0, 0.0))));
}

fn translator_multi_vector_left_contraction(self_894: Translator, other_762: MultiVector) -> MultiVector {
    var self_895: Translator;
    var other_763: MultiVector;

    self_895 = self_894;
    other_763 = other_762;
    let _e4: Translator = self_895;
    let _e8: MultiVector = other_763;
    let _e11: Translator = self_895;
    let _e15: MultiVector = other_763;
    let _e28: Translator = self_895;
    let _e31: Translator = self_895;
    let _e34: Translator = self_895;
    let _e37: Translator = self_895;
    let _e41: MultiVector = other_763;
    let _e54: Translator = self_895;
    let _e58: MultiVector = other_763;
    return MultiVector((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.z) * _e15.g1_.wwyw) * vec4<f32>(-(1.0), 0.0, -(1.0), 0.0))) + ((vec4<f32>(_e28.g0_.y, _e31.g0_.x, _e34.g0_.x, _e37.g0_.y) * _e41.g1_.zxxy) * vec4<f32>(-(1.0), 0.0, 0.0, -(1.0)))), (vec4<f32>(_e54.g0_.x) * _e58.g1_));
}

fn translator_multi_vector_scalar_product(self_896: Translator, other_764: MultiVector) -> Scalar {
    var self_897: Translator;
    var other_765: MultiVector;

    self_897 = self_896;
    other_765 = other_764;
    let _e4: Translator = self_897;
    let _e7: MultiVector = other_765;
    let _e11: Translator = self_897;
    let _e14: MultiVector = other_765;
    let _e19: Translator = self_897;
    let _e22: MultiVector = other_765;
    return Scalar((((_e4.g0_.x * _e7.g0_.x) - (_e11.g0_.y * _e14.g1_.z)) - (_e19.g0_.z * _e22.g1_.w)));
}

fn translator_rotor_add(self_898: Translator, other_766: Rotor) -> Motor {
    var self_899: Translator;
    var other_767: Rotor;

    self_899 = self_898;
    other_767 = other_766;
    let _e4: Translator = self_899;
    let _e7: Translator = self_899;
    let _e10: Translator = self_899;
    let _e13: Translator = self_899;
    let _e23: Rotor = other_767;
    let _e26: Rotor = other_767;
    let _e29: Rotor = other_767;
    let _e32: Rotor = other_767;
    return Motor(((vec4<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.y, _e13.g0_.z) * vec4<f32>(1.0, 0.0, 1.0, 1.0)) + (vec4<f32>(_e23.g0_.x, _e26.g0_.y, _e29.g0_.x, _e32.g0_.x) * vec4<f32>(1.0, 1.0, 0.0, 0.0))));
}

fn translator_rotor_sub(self_900: Translator, other_768: Rotor) -> Motor {
    var self_901: Translator;
    var other_769: Rotor;

    self_901 = self_900;
    other_769 = other_768;
    let _e4: Translator = self_901;
    let _e7: Translator = self_901;
    let _e10: Translator = self_901;
    let _e13: Translator = self_901;
    let _e23: Rotor = other_769;
    let _e26: Rotor = other_769;
    let _e29: Rotor = other_769;
    let _e32: Rotor = other_769;
    return Motor(((vec4<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.y, _e13.g0_.z) * vec4<f32>(1.0, 0.0, 1.0, 1.0)) - (vec4<f32>(_e23.g0_.x, _e26.g0_.y, _e29.g0_.x, _e32.g0_.x) * vec4<f32>(1.0, 1.0, 0.0, 0.0))));
}

fn translator_rotor_geometric_product(self_902: Translator, other_770: Rotor) -> Motor {
    var self_903: Translator;
    var other_771: Rotor;

    self_903 = self_902;
    other_771 = other_770;
    let _e4: Translator = self_903;
    let _e8: Rotor = other_771;
    let _e11: Rotor = other_771;
    let _e14: Rotor = other_771;
    let _e17: Rotor = other_771;
    let _e28: Translator = self_903;
    let _e31: Translator = self_903;
    let _e34: Translator = self_903;
    let _e37: Translator = self_903;
    let _e41: Rotor = other_771;
    let _e44: Rotor = other_771;
    let _e47: Rotor = other_771;
    let _e50: Rotor = other_771;
    return Motor((((vec4<f32>(_e4.g0_.z) * vec4<f32>(_e8.g0_.y, _e11.g0_.y, _e14.g0_.y, _e17.g0_.x)) * vec4<f32>(0.0, 0.0, 1.0, 1.0)) + ((vec4<f32>(_e28.g0_.x, _e31.g0_.x, _e34.g0_.y, _e37.g0_.y) * vec4<f32>(_e41.g0_.x, _e44.g0_.y, _e47.g0_.x, _e50.g0_.y)) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))));
}

fn translator_rotor_outer_product(self_904: Translator, other_772: Rotor) -> Motor {
    var self_905: Translator;
    var other_773: Rotor;

    self_905 = self_904;
    other_773 = other_772;
    let _e4: Translator = self_905;
    let _e7: Translator = self_905;
    let _e10: Translator = self_905;
    let _e13: Translator = self_905;
    let _e17: Rotor = other_773;
    let _e20: Rotor = other_773;
    let _e23: Rotor = other_773;
    let _e26: Rotor = other_773;
    return Motor((vec4<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.y, _e13.g0_.z) * vec4<f32>(_e17.g0_.x, _e20.g0_.y, _e23.g0_.x, _e26.g0_.x)));
}

fn translator_rotor_inner_product(self_906: Translator, other_774: Rotor) -> Motor {
    var self_907: Translator;
    var other_775: Rotor;

    self_907 = self_906;
    other_775 = other_774;
    let _e4: Translator = self_907;
    let _e7: Translator = self_907;
    let _e10: Translator = self_907;
    let _e13: Translator = self_907;
    let _e17: Rotor = other_775;
    let _e20: Rotor = other_775;
    let _e23: Rotor = other_775;
    let _e26: Rotor = other_775;
    return Motor((vec4<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.y, _e13.g0_.z) * vec4<f32>(_e17.g0_.x, _e20.g0_.y, _e23.g0_.x, _e26.g0_.x)));
}

fn translator_rotor_left_contraction(self_908: Translator, other_776: Rotor) -> Rotor {
    var self_909: Translator;
    var other_777: Rotor;

    self_909 = self_908;
    other_777 = other_776;
    let _e4: Translator = self_909;
    let _e8: Rotor = other_777;
    return Rotor((vec2<f32>(_e4.g0_.x) * _e8.g0_));
}

fn translator_rotor_right_contraction(self_910: Translator, other_778: Rotor) -> Translator {
    var self_911: Translator;
    var other_779: Rotor;

    self_911 = self_910;
    other_779 = other_778;
    let _e4: Translator = self_911;
    let _e6: Rotor = other_779;
    return Translator((_e4.g0_ * vec3<f32>(_e6.g0_.x)));
}

fn translator_rotor_scalar_product(self_912: Translator, other_780: Rotor) -> Scalar {
    var self_913: Translator;
    var other_781: Rotor;

    self_913 = self_912;
    other_781 = other_780;
    let _e4: Translator = self_913;
    let _e7: Rotor = other_781;
    return Scalar((_e4.g0_.x * _e7.g0_.x));
}

fn translator_point_add(self_914: Translator, other_782: Point) -> Motor {
    var self_915: Translator;
    var other_783: Point;

    self_915 = self_914;
    other_783 = other_782;
    let _e4: Translator = self_915;
    let _e7: Translator = self_915;
    let _e10: Translator = self_915;
    let _e13: Translator = self_915;
    let _e23: Point = other_783;
    let _e26: Point = other_783;
    let _e29: Point = other_783;
    let _e32: Point = other_783;
    return Motor(((vec4<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.y, _e13.g0_.z) * vec4<f32>(1.0, 0.0, 1.0, 1.0)) + (vec4<f32>(_e23.g0_.x, _e26.g0_.x, _e29.g0_.y, _e32.g0_.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn translator_point_sub(self_916: Translator, other_784: Point) -> Motor {
    var self_917: Translator;
    var other_785: Point;

    self_917 = self_916;
    other_785 = other_784;
    let _e4: Translator = self_917;
    let _e7: Translator = self_917;
    let _e10: Translator = self_917;
    let _e13: Translator = self_917;
    let _e23: Point = other_785;
    let _e26: Point = other_785;
    let _e29: Point = other_785;
    let _e32: Point = other_785;
    return Motor(((vec4<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.y, _e13.g0_.z) * vec4<f32>(1.0, 0.0, 1.0, 1.0)) - (vec4<f32>(_e23.g0_.x, _e26.g0_.x, _e29.g0_.y, _e32.g0_.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn translator_point_geometric_product(self_918: Translator, other_786: Point) -> Motor {
    var self_919: Translator;
    var other_787: Point;

    self_919 = self_918;
    other_787 = other_786;
    let _e4: Translator = self_919;
    let _e8: Point = other_787;
    let _e11: Point = other_787;
    let _e14: Point = other_787;
    let _e17: Point = other_787;
    let _e30: Translator = self_919;
    let _e34: Point = other_787;
    let _e37: Point = other_787;
    let _e40: Point = other_787;
    let _e43: Point = other_787;
    let _e57: Translator = self_919;
    let _e61: Point = other_787;
    let _e64: Point = other_787;
    let _e67: Point = other_787;
    let _e70: Point = other_787;
    return Motor(((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y, _e11.g0_.z, _e14.g0_.y, _e17.g0_.x)) * vec4<f32>(-(1.0), 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e30.g0_.z) * vec4<f32>(_e34.g0_.z, _e37.g0_.y, _e40.g0_.x, _e43.g0_.z)) * vec4<f32>(-(1.0), -(1.0), 1.0, 0.0))) + ((vec4<f32>(_e57.g0_.x) * vec4<f32>(_e61.g0_.x, _e64.g0_.x, _e67.g0_.y, _e70.g0_.z)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn translator_point_regressive_product(self_920: Translator, other_788: Point) -> Plane {
    var self_921: Translator;
    var other_789: Point;

    self_921 = self_920;
    other_789 = other_788;
    let _e4: Translator = self_921;
    let _e8: Point = other_789;
    let _e18: Translator = self_921;
    let _e21: Point = other_789;
    return Plane((((vec3<f32>(_e4.g0_.z) * _e8.g0_.yxy) * vec3<f32>(1.0, -(1.0), 0.0)) + ((_e18.g0_.yxy * _e21.g0_.zxx) * vec3<f32>(-(1.0), 0.0, 1.0))));
}

fn translator_point_outer_product(self_922: Translator, other_790: Point) -> Point {
    var self_923: Translator;
    var other_791: Point;

    self_923 = self_922;
    other_791 = other_790;
    let _e4: Translator = self_923;
    let _e8: Point = other_791;
    return Point((vec3<f32>(_e4.g0_.x) * _e8.g0_));
}

fn translator_point_inner_product(self_924: Translator, other_792: Point) -> Motor {
    var self_925: Translator;
    var other_793: Point;

    self_925 = self_924;
    other_793 = other_792;
    let _e4: Translator = self_925;
    let _e8: Point = other_793;
    let _e20: Translator = self_925;
    let _e23: Translator = self_925;
    let _e26: Translator = self_925;
    let _e29: Translator = self_925;
    let _e33: Point = other_793;
    let _e36: Point = other_793;
    let _e39: Point = other_793;
    let _e42: Point = other_793;
    return Motor((((vec4<f32>(_e4.g0_.z) * vec4<f32>(_e8.g0_.z)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0)) + ((vec4<f32>(_e20.g0_.y, _e23.g0_.x, _e26.g0_.x, _e29.g0_.x) * vec4<f32>(_e33.g0_.y, _e36.g0_.x, _e39.g0_.y, _e42.g0_.z)) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0))));
}

fn translator_point_left_contraction(self_926: Translator, other_794: Point) -> Motor {
    var self_927: Translator;
    var other_795: Point;

    self_927 = self_926;
    other_795 = other_794;
    let _e4: Translator = self_927;
    let _e8: Point = other_795;
    let _e20: Translator = self_927;
    let _e23: Translator = self_927;
    let _e26: Translator = self_927;
    let _e29: Translator = self_927;
    let _e33: Point = other_795;
    let _e36: Point = other_795;
    let _e39: Point = other_795;
    let _e42: Point = other_795;
    return Motor((((vec4<f32>(_e4.g0_.z) * vec4<f32>(_e8.g0_.z)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0)) + ((vec4<f32>(_e20.g0_.y, _e23.g0_.x, _e26.g0_.x, _e29.g0_.x) * vec4<f32>(_e33.g0_.y, _e36.g0_.x, _e39.g0_.y, _e42.g0_.z)) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0))));
}

fn translator_point_right_contraction(self_928: Translator, other_796: Point) -> Scalar {
    var self_929: Translator;
    var other_797: Point;

    self_929 = self_928;
    other_797 = other_796;
    let _e5: Translator = self_929;
    let _e8: Point = other_797;
    let _e13: Translator = self_929;
    let _e16: Point = other_797;
    return Scalar(((0.0 - (_e5.g0_.y * _e8.g0_.y)) - (_e13.g0_.z * _e16.g0_.z)));
}

fn translator_point_scalar_product(self_930: Translator, other_798: Point) -> Scalar {
    var self_931: Translator;
    var other_799: Point;

    self_931 = self_930;
    other_799 = other_798;
    let _e5: Translator = self_931;
    let _e8: Point = other_799;
    let _e13: Translator = self_931;
    let _e16: Point = other_799;
    return Scalar(((0.0 - (_e5.g0_.y * _e8.g0_.y)) - (_e13.g0_.z * _e16.g0_.z)));
}

fn translator_ideal_point_into(self_932: Translator) -> IdealPoint {
    var self_933: Translator;

    self_933 = self_932;
    let _e2: Translator = self_933;
    let _e5: Translator = self_933;
    return IdealPoint(vec2<f32>(_e2.g0_.y, _e5.g0_.z));
}

fn translator_ideal_point_add(self_934: Translator, other_800: IdealPoint) -> Translator {
    var self_935: Translator;
    var other_801: IdealPoint;

    self_935 = self_934;
    other_801 = other_800;
    let _e4: Translator = self_935;
    let _e6: IdealPoint = other_801;
    let _e9: IdealPoint = other_801;
    let _e12: IdealPoint = other_801;
    return Translator((_e4.g0_ + (vec3<f32>(_e6.g0_.x, _e9.g0_.x, _e12.g0_.y) * vec3<f32>(0.0, 1.0, 1.0))));
}

fn translator_ideal_point_sub(self_936: Translator, other_802: IdealPoint) -> Translator {
    var self_937: Translator;
    var other_803: IdealPoint;

    self_937 = self_936;
    other_803 = other_802;
    let _e4: Translator = self_937;
    let _e6: IdealPoint = other_803;
    let _e9: IdealPoint = other_803;
    let _e12: IdealPoint = other_803;
    return Translator((_e4.g0_ - (vec3<f32>(_e6.g0_.x, _e9.g0_.x, _e12.g0_.y) * vec3<f32>(0.0, 1.0, 1.0))));
}

fn translator_ideal_point_geometric_product(self_938: Translator, other_804: IdealPoint) -> Motor {
    var self_939: Translator;
    var other_805: IdealPoint;

    self_939 = self_938;
    other_805 = other_804;
    let _e4: Translator = self_939;
    let _e8: IdealPoint = other_805;
    let _e11: IdealPoint = other_805;
    let _e14: IdealPoint = other_805;
    let _e17: IdealPoint = other_805;
    let _e30: Translator = self_939;
    let _e33: Translator = self_939;
    let _e36: Translator = self_939;
    let _e39: Translator = self_939;
    let _e43: IdealPoint = other_805;
    let _e46: IdealPoint = other_805;
    let _e49: IdealPoint = other_805;
    let _e52: IdealPoint = other_805;
    return Motor((((vec4<f32>(_e4.g0_.z) * vec4<f32>(_e8.g0_.y, _e11.g0_.x, _e14.g0_.y, _e17.g0_.y)) * vec4<f32>(-(1.0), -(1.0), 0.0, 0.0)) + ((vec4<f32>(_e30.g0_.y, _e33.g0_.y, _e36.g0_.x, _e39.g0_.x) * vec4<f32>(_e43.g0_.x, _e46.g0_.y, _e49.g0_.x, _e52.g0_.y)) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0))));
}

fn translator_ideal_point_outer_product(self_940: Translator, other_806: IdealPoint) -> IdealPoint {
    var self_941: Translator;
    var other_807: IdealPoint;

    self_941 = self_940;
    other_807 = other_806;
    let _e4: Translator = self_941;
    let _e8: IdealPoint = other_807;
    return IdealPoint((vec2<f32>(_e4.g0_.x) * _e8.g0_));
}

fn translator_ideal_point_inner_product(self_942: Translator, other_808: IdealPoint) -> Translator {
    var self_943: Translator;
    var other_809: IdealPoint;

    self_943 = self_942;
    other_809 = other_808;
    let _e4: Translator = self_943;
    let _e8: IdealPoint = other_809;
    let _e19: Translator = self_943;
    let _e22: IdealPoint = other_809;
    let _e25: IdealPoint = other_809;
    let _e28: IdealPoint = other_809;
    return Translator((((vec3<f32>(_e4.g0_.z) * vec3<f32>(_e8.g0_.y)) * vec3<f32>(-(1.0), 0.0, 0.0)) + ((_e19.g0_.yxx * vec3<f32>(_e22.g0_.x, _e25.g0_.x, _e28.g0_.y)) * vec3<f32>(-(1.0), 1.0, 1.0))));
}

fn translator_ideal_point_left_contraction(self_944: Translator, other_810: IdealPoint) -> Translator {
    var self_945: Translator;
    var other_811: IdealPoint;

    self_945 = self_944;
    other_811 = other_810;
    let _e4: Translator = self_945;
    let _e8: IdealPoint = other_811;
    let _e19: Translator = self_945;
    let _e22: IdealPoint = other_811;
    let _e25: IdealPoint = other_811;
    let _e28: IdealPoint = other_811;
    return Translator((((vec3<f32>(_e4.g0_.z) * vec3<f32>(_e8.g0_.y)) * vec3<f32>(-(1.0), 0.0, 0.0)) + ((_e19.g0_.yxx * vec3<f32>(_e22.g0_.x, _e25.g0_.x, _e28.g0_.y)) * vec3<f32>(-(1.0), 1.0, 1.0))));
}

fn translator_ideal_point_right_contraction(self_946: Translator, other_812: IdealPoint) -> Scalar {
    var self_947: Translator;
    var other_813: IdealPoint;

    self_947 = self_946;
    other_813 = other_812;
    let _e5: Translator = self_947;
    let _e8: IdealPoint = other_813;
    let _e13: Translator = self_947;
    let _e16: IdealPoint = other_813;
    return Scalar(((0.0 - (_e5.g0_.y * _e8.g0_.x)) - (_e13.g0_.z * _e16.g0_.y)));
}

fn translator_ideal_point_scalar_product(self_948: Translator, other_814: IdealPoint) -> Scalar {
    var self_949: Translator;
    var other_815: IdealPoint;

    self_949 = self_948;
    other_815 = other_814;
    let _e5: Translator = self_949;
    let _e8: IdealPoint = other_815;
    let _e13: Translator = self_949;
    let _e16: IdealPoint = other_815;
    return Scalar(((0.0 - (_e5.g0_.y * _e8.g0_.x)) - (_e13.g0_.z * _e16.g0_.y)));
}

fn translator_plane_geometric_product(self_950: Translator, other_816: Plane) -> MotorDual {
    var self_951: Translator;
    var other_817: Plane;

    self_951 = self_950;
    other_817 = other_816;
    let _e4: Translator = self_951;
    let _e8: Plane = other_817;
    let _e11: Plane = other_817;
    let _e14: Plane = other_817;
    let _e17: Plane = other_817;
    let _e29: Translator = self_951;
    let _e33: Plane = other_817;
    let _e36: Plane = other_817;
    let _e39: Plane = other_817;
    let _e42: Plane = other_817;
    let _e55: Translator = self_951;
    let _e59: Plane = other_817;
    let _e62: Plane = other_817;
    let _e65: Plane = other_817;
    let _e68: Plane = other_817;
    return MotorDual(((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y, _e11.g0_.z, _e14.g0_.y, _e17.g0_.x)) * vec4<f32>(1.0, 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e29.g0_.z) * vec4<f32>(_e33.g0_.z, _e36.g0_.y, _e39.g0_.x, _e42.g0_.z)) * vec4<f32>(1.0, -(1.0), 1.0, 0.0))) + ((vec4<f32>(_e55.g0_.x) * vec4<f32>(_e59.g0_.x, _e62.g0_.x, _e65.g0_.y, _e68.g0_.z)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn translator_plane_regressive_product(self_952: Translator, other_818: Plane) -> Scalar {
    var self_953: Translator;
    var other_819: Plane;

    self_953 = self_952;
    other_819 = other_818;
    let _e4: Translator = self_953;
    let _e7: Plane = other_819;
    let _e11: Translator = self_953;
    let _e14: Plane = other_819;
    return Scalar(((_e4.g0_.y * _e7.g0_.y) + (_e11.g0_.z * _e14.g0_.z)));
}

fn translator_plane_outer_product(self_954: Translator, other_820: Plane) -> MotorDual {
    var self_955: Translator;
    var other_821: Plane;

    self_955 = self_954;
    other_821 = other_820;
    let _e4: Translator = self_955;
    let _e8: Plane = other_821;
    let _e19: Translator = self_955;
    let _e22: Translator = self_955;
    let _e25: Translator = self_955;
    let _e28: Translator = self_955;
    let _e32: Plane = other_821;
    let _e35: Plane = other_821;
    let _e38: Plane = other_821;
    let _e41: Plane = other_821;
    return MotorDual((((vec4<f32>(_e4.g0_.z) * vec4<f32>(_e8.g0_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) + (vec4<f32>(_e19.g0_.y, _e22.g0_.x, _e25.g0_.x, _e28.g0_.x) * vec4<f32>(_e32.g0_.y, _e35.g0_.x, _e38.g0_.y, _e41.g0_.z))));
}

fn translator_plane_inner_product(self_956: Translator, other_822: Plane) -> Plane {
    var self_957: Translator;
    var other_823: Plane;

    self_957 = self_956;
    other_823 = other_822;
    let _e4: Translator = self_957;
    let _e8: Plane = other_823;
    let _e11: Translator = self_957;
    let _e15: Plane = other_823;
    let _e26: Translator = self_957;
    let _e29: Plane = other_823;
    return Plane((((vec3<f32>(_e4.g0_.x) * _e8.g0_) + ((vec3<f32>(_e11.g0_.z) * _e15.g0_.yxy) * vec3<f32>(-(1.0), 1.0, 0.0))) + ((_e26.g0_.yxy * _e29.g0_.zxx) * vec3<f32>(1.0, 0.0, -(1.0)))));
}

fn translator_plane_left_contraction(self_958: Translator, other_824: Plane) -> Plane {
    var self_959: Translator;
    var other_825: Plane;

    self_959 = self_958;
    other_825 = other_824;
    let _e4: Translator = self_959;
    let _e8: Plane = other_825;
    return Plane((vec3<f32>(_e4.g0_.x) * _e8.g0_));
}

fn translator_plane_right_contraction(self_960: Translator, other_826: Plane) -> Plane {
    var self_961: Translator;
    var other_827: Plane;

    self_961 = self_960;
    other_827 = other_826;
    let _e4: Translator = self_961;
    let _e8: Plane = other_827;
    let _e18: Translator = self_961;
    let _e21: Plane = other_827;
    return Plane((((vec3<f32>(_e4.g0_.z) * _e8.g0_.yxy) * vec3<f32>(-(1.0), 1.0, 0.0)) + ((_e18.g0_.yxy * _e21.g0_.zxx) * vec3<f32>(1.0, 0.0, -(1.0)))));
}

fn translator_translator_add(self_962: Translator, other_828: Translator) -> Translator {
    var self_963: Translator;
    var other_829: Translator;

    self_963 = self_962;
    other_829 = other_828;
    let _e4: Translator = self_963;
    let _e6: Translator = other_829;
    return Translator((_e4.g0_ + _e6.g0_));
}

fn translator_translator_sub(self_964: Translator, other_830: Translator) -> Translator {
    var self_965: Translator;
    var other_831: Translator;

    self_965 = self_964;
    other_831 = other_830;
    let _e4: Translator = self_965;
    let _e6: Translator = other_831;
    return Translator((_e4.g0_ - _e6.g0_));
}

fn translator_translator_mul(self_966: Translator, other_832: Translator) -> Translator {
    var self_967: Translator;
    var other_833: Translator;

    self_967 = self_966;
    other_833 = other_832;
    let _e4: Translator = self_967;
    let _e6: Translator = other_833;
    return Translator((_e4.g0_ * _e6.g0_));
}

fn translator_translator_div(self_968: Translator, other_834: Translator) -> Translator {
    var self_969: Translator;
    var other_835: Translator;

    self_969 = self_968;
    other_835 = other_834;
    let _e4: Translator = self_969;
    let _e7: Translator = self_969;
    let _e10: Translator = self_969;
    let _e19: Translator = other_835;
    let _e22: Translator = other_835;
    let _e25: Translator = other_835;
    return Translator((((vec3<f32>(_e4.g0_.x, _e7.g0_.y, _e10.g0_.z) * vec3<f32>(1.0, 1.0, 1.0)) / vec3<f32>(_e19.g0_.x, _e22.g0_.y, _e25.g0_.z)) * vec3<f32>(1.0, 1.0, 1.0)));
}

fn translator_translator_geometric_product(self_970: Translator, other_836: Translator) -> Motor {
    var self_971: Translator;
    var other_837: Translator;

    self_971 = self_970;
    other_837 = other_836;
    let _e4: Translator = self_971;
    let _e8: Translator = other_837;
    let _e11: Translator = other_837;
    let _e14: Translator = other_837;
    let _e17: Translator = other_837;
    let _e29: Translator = self_971;
    let _e33: Translator = other_837;
    let _e36: Translator = other_837;
    let _e39: Translator = other_837;
    let _e42: Translator = other_837;
    let _e56: Translator = self_971;
    let _e60: Translator = other_837;
    let _e63: Translator = other_837;
    let _e66: Translator = other_837;
    let _e69: Translator = other_837;
    return Motor(((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y, _e11.g0_.z, _e14.g0_.x, _e17.g0_.y)) * vec4<f32>(-(1.0), 1.0, 1.0, 0.0)) + ((vec4<f32>(_e29.g0_.z) * vec4<f32>(_e33.g0_.z, _e36.g0_.y, _e39.g0_.z, _e42.g0_.x)) * vec4<f32>(-(1.0), -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e56.g0_.x) * vec4<f32>(_e60.g0_.x, _e63.g0_.x, _e66.g0_.y, _e69.g0_.z)) * vec4<f32>(1.0, 0.0, 1.0, 1.0))));
}

fn translator_translator_outer_product(self_972: Translator, other_838: Translator) -> Translator {
    var self_973: Translator;
    var other_839: Translator;

    self_973 = self_972;
    other_839 = other_838;
    let _e4: Translator = self_973;
    let _e8: Translator = other_839;
    let _e11: Translator = self_973;
    let _e13: Translator = other_839;
    return Translator(((vec3<f32>(_e4.g0_.x) * _e8.g0_) + ((_e11.g0_ * vec3<f32>(_e13.g0_.x)) * vec3<f32>(0.0, 1.0, 1.0))));
}

fn translator_translator_inner_product(self_974: Translator, other_840: Translator) -> Translator {
    var self_975: Translator;
    var other_841: Translator;

    self_975 = self_974;
    other_841 = other_840;
    let _e4: Translator = self_975;
    let _e8: Translator = other_841;
    let _e11: Translator = self_975;
    let _e15: Translator = other_841;
    let _e26: Translator = self_975;
    let _e29: Translator = other_841;
    return Translator((((vec3<f32>(_e4.g0_.x) * _e8.g0_) + ((vec3<f32>(_e11.g0_.z) * _e15.g0_.zzx) * vec3<f32>(-(1.0), 0.0, 1.0))) + ((_e26.g0_.yyx * _e29.g0_.yxx) * vec3<f32>(-(1.0), 1.0, 0.0))));
}

fn translator_translator_left_contraction(self_976: Translator, other_842: Translator) -> Translator {
    var self_977: Translator;
    var other_843: Translator;

    self_977 = self_976;
    other_843 = other_842;
    let _e4: Translator = self_977;
    let _e8: Translator = other_843;
    let _e11: Translator = self_977;
    let _e15: Translator = other_843;
    let _e27: Translator = self_977;
    let _e30: Translator = other_843;
    return Translator((((vec3<f32>(_e4.g0_.x) * _e8.g0_) + ((vec3<f32>(_e11.g0_.z) * vec3<f32>(_e15.g0_.z)) * vec3<f32>(-(1.0), 0.0, 0.0))) + ((_e27.g0_.yxx * _e30.g0_.yxx) * vec3<f32>(-(1.0), 0.0, 0.0))));
}

fn translator_translator_right_contraction(self_978: Translator, other_844: Translator) -> Translator {
    var self_979: Translator;
    var other_845: Translator;

    self_979 = self_978;
    other_845 = other_844;
    let _e4: Translator = self_979;
    let _e8: Translator = other_845;
    let _e18: Translator = self_979;
    let _e22: Translator = other_845;
    let _e33: Translator = self_979;
    let _e37: Translator = other_845;
    return Translator(((((vec3<f32>(_e4.g0_.y) * _e8.g0_.yxy) * vec3<f32>(-(1.0), 1.0, 0.0)) + ((vec3<f32>(_e18.g0_.z) * _e22.g0_.zzx) * vec3<f32>(-(1.0), 0.0, 1.0))) + ((vec3<f32>(_e33.g0_.x) * vec3<f32>(_e37.g0_.x)) * vec3<f32>(1.0, 0.0, 0.0))));
}

fn translator_translator_scalar_product(self_980: Translator, other_846: Translator) -> Scalar {
    var self_981: Translator;
    var other_847: Translator;

    self_981 = self_980;
    other_847 = other_846;
    let _e4: Translator = self_981;
    let _e7: Translator = other_847;
    let _e11: Translator = self_981;
    let _e14: Translator = other_847;
    let _e19: Translator = self_981;
    let _e22: Translator = other_847;
    return Scalar((((_e4.g0_.x * _e7.g0_.x) - (_e11.g0_.y * _e14.g0_.y)) - (_e19.g0_.z * _e22.g0_.z)));
}

fn translator_motor_add(self_982: Translator, other_848: Motor) -> Motor {
    var self_983: Translator;
    var other_849: Motor;

    self_983 = self_982;
    other_849 = other_848;
    let _e4: Translator = self_983;
    let _e7: Translator = self_983;
    let _e10: Translator = self_983;
    let _e13: Translator = self_983;
    let _e23: Motor = other_849;
    return Motor(((vec4<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.y, _e13.g0_.z) * vec4<f32>(1.0, 0.0, 1.0, 1.0)) + _e23.g0_));
}

fn translator_motor_sub(self_984: Translator, other_850: Motor) -> Motor {
    var self_985: Translator;
    var other_851: Motor;

    self_985 = self_984;
    other_851 = other_850;
    let _e4: Translator = self_985;
    let _e7: Translator = self_985;
    let _e10: Translator = self_985;
    let _e13: Translator = self_985;
    let _e23: Motor = other_851;
    return Motor(((vec4<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.y, _e13.g0_.z) * vec4<f32>(1.0, 0.0, 1.0, 1.0)) - _e23.g0_));
}

fn translator_motor_geometric_product(self_986: Translator, other_852: Motor) -> Motor {
    var self_987: Translator;
    var other_853: Motor;

    self_987 = self_986;
    other_853 = other_852;
    let _e4: Translator = self_987;
    let _e8: Motor = other_853;
    let _e11: Translator = self_987;
    let _e15: Motor = other_853;
    let _e28: Translator = self_987;
    let _e32: Motor = other_853;
    return Motor((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.y) * _e15.g0_.zwxy) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e28.g0_.z) * _e32.g0_.wzyx) * vec4<f32>(-(1.0), -(1.0), 1.0, 1.0))));
}

fn translator_motor_regressive_product(self_988: Translator, other_854: Motor) -> Plane {
    var self_989: Translator;
    var other_855: Motor;

    self_989 = self_988;
    other_855 = other_854;
    let _e4: Translator = self_989;
    let _e8: Motor = other_855;
    let _e11: Motor = other_855;
    let _e14: Motor = other_855;
    let _e25: Translator = self_989;
    let _e28: Motor = other_855;
    let _e31: Motor = other_855;
    let _e34: Motor = other_855;
    return Plane((((vec3<f32>(_e4.g0_.z) * vec3<f32>(_e8.g0_.z, _e11.g0_.y, _e14.g0_.z)) * vec3<f32>(1.0, -(1.0), 0.0)) + ((_e25.g0_.yxy * vec3<f32>(_e28.g0_.w, _e31.g0_.x, _e34.g0_.y)) * vec3<f32>(-(1.0), 0.0, 1.0))));
}

fn translator_motor_outer_product(self_990: Translator, other_856: Motor) -> Motor {
    var self_991: Translator;
    var other_857: Motor;

    self_991 = self_990;
    other_857 = other_856;
    let _e4: Translator = self_991;
    let _e8: Motor = other_857;
    let _e11: Translator = self_991;
    let _e14: Translator = self_991;
    let _e17: Translator = self_991;
    let _e20: Translator = self_991;
    let _e24: Motor = other_857;
    return Motor(((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.x, _e14.g0_.x, _e17.g0_.y, _e20.g0_.z) * vec4<f32>(_e24.g0_.x)) * vec4<f32>(0.0, 0.0, 1.0, 1.0))));
}

fn translator_motor_inner_product(self_992: Translator, other_858: Motor) -> Motor {
    var self_993: Translator;
    var other_859: Motor;

    self_993 = self_992;
    other_859 = other_858;
    let _e4: Translator = self_993;
    let _e8: Motor = other_859;
    let _e11: Translator = self_993;
    let _e15: Motor = other_859;
    let _e27: Translator = self_993;
    let _e30: Translator = self_993;
    let _e33: Translator = self_993;
    let _e36: Translator = self_993;
    let _e40: Motor = other_859;
    return Motor((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.z) * _e15.g0_.wwwx) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))) + ((vec4<f32>(_e27.g0_.y, _e30.g0_.x, _e33.g0_.y, _e36.g0_.x) * _e40.g0_.zxxx) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0))));
}

fn translator_motor_left_contraction(self_994: Translator, other_860: Motor) -> Motor {
    var self_995: Translator;
    var other_861: Motor;

    self_995 = self_994;
    other_861 = other_860;
    let _e4: Translator = self_995;
    let _e8: Motor = other_861;
    let _e11: Translator = self_995;
    let _e15: Motor = other_861;
    let _e28: Translator = self_995;
    let _e31: Translator = self_995;
    let _e34: Translator = self_995;
    let _e37: Translator = self_995;
    let _e41: Motor = other_861;
    return Motor((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.z) * vec4<f32>(_e15.g0_.w)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((vec4<f32>(_e28.g0_.y, _e31.g0_.x, _e34.g0_.x, _e37.g0_.x) * _e41.g0_.zxxx) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))));
}

fn translator_motor_right_contraction(self_996: Translator, other_862: Motor) -> Translator {
    var self_997: Translator;
    var other_863: Motor;

    self_997 = self_996;
    other_863 = other_862;
    let _e4: Translator = self_997;
    let _e8: Motor = other_863;
    let _e11: Motor = other_863;
    let _e14: Motor = other_863;
    let _e25: Translator = self_997;
    let _e29: Motor = other_863;
    let _e32: Motor = other_863;
    let _e35: Motor = other_863;
    let _e47: Translator = self_997;
    let _e51: Motor = other_863;
    return Translator(((((vec3<f32>(_e4.g0_.y) * vec3<f32>(_e8.g0_.z, _e11.g0_.x, _e14.g0_.z)) * vec3<f32>(-(1.0), 1.0, 0.0)) + ((vec3<f32>(_e25.g0_.z) * vec3<f32>(_e29.g0_.w, _e32.g0_.w, _e35.g0_.x)) * vec3<f32>(-(1.0), 0.0, 1.0))) + ((vec3<f32>(_e47.g0_.x) * vec3<f32>(_e51.g0_.x)) * vec3<f32>(1.0, 0.0, 0.0))));
}

fn translator_motor_scalar_product(self_998: Translator, other_864: Motor) -> Scalar {
    var self_999: Translator;
    var other_865: Motor;

    self_999 = self_998;
    other_865 = other_864;
    let _e4: Translator = self_999;
    let _e7: Motor = other_865;
    let _e11: Translator = self_999;
    let _e14: Motor = other_865;
    let _e19: Translator = self_999;
    let _e22: Motor = other_865;
    return Scalar((((_e4.g0_.x * _e7.g0_.x) - (_e11.g0_.y * _e14.g0_.z)) - (_e19.g0_.z * _e22.g0_.w)));
}

fn translator_motor_dual_geometric_product(self_1000: Translator, other_866: MotorDual) -> MotorDual {
    var self_1001: Translator;
    var other_867: MotorDual;

    self_1001 = self_1000;
    other_867 = other_866;
    let _e4: Translator = self_1001;
    let _e8: MotorDual = other_867;
    let _e11: Translator = self_1001;
    let _e15: MotorDual = other_867;
    let _e28: Translator = self_1001;
    let _e32: MotorDual = other_867;
    return MotorDual((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.y) * _e15.g0_.zwxy) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e28.g0_.z) * _e32.g0_.wzyx) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))));
}

fn translator_motor_dual_regressive_product(self_1002: Translator, other_868: MotorDual) -> Translator {
    var self_1003: Translator;
    var other_869: MotorDual;

    self_1003 = self_1002;
    other_869 = other_868;
    let _e4: Translator = self_1003;
    let _e8: MotorDual = other_869;
    let _e11: MotorDual = other_869;
    let _e14: MotorDual = other_869;
    let _e24: Translator = self_1003;
    let _e28: MotorDual = other_869;
    let _e31: MotorDual = other_869;
    let _e34: MotorDual = other_869;
    let _e45: Translator = self_1003;
    let _e49: MotorDual = other_869;
    return Translator(((((vec3<f32>(_e4.g0_.y) * vec3<f32>(_e8.g0_.z, _e11.g0_.x, _e14.g0_.z)) * vec3<f32>(1.0, 1.0, 0.0)) + ((vec3<f32>(_e24.g0_.z) * vec3<f32>(_e28.g0_.w, _e31.g0_.w, _e34.g0_.x)) * vec3<f32>(1.0, 0.0, 1.0))) + ((vec3<f32>(_e45.g0_.x) * vec3<f32>(_e49.g0_.x)) * vec3<f32>(1.0, 0.0, 0.0))));
}

fn translator_motor_dual_outer_product(self_1004: Translator, other_870: MotorDual) -> MotorDual {
    var self_1005: Translator;
    var other_871: MotorDual;

    self_1005 = self_1004;
    other_871 = other_870;
    let _e4: Translator = self_1005;
    let _e8: MotorDual = other_871;
    let _e11: Translator = self_1005;
    let _e15: MotorDual = other_871;
    let _e27: Translator = self_1005;
    let _e30: Translator = self_1005;
    let _e33: Translator = self_1005;
    let _e36: Translator = self_1005;
    let _e40: MotorDual = other_871;
    return MotorDual((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.z) * vec4<f32>(_e15.g0_.w)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e27.g0_.y, _e30.g0_.x, _e33.g0_.x, _e36.g0_.x) * _e40.g0_.zxxx) * vec4<f32>(1.0, 0.0, 0.0, 0.0))));
}

fn translator_motor_dual_inner_product(self_1006: Translator, other_872: MotorDual) -> MotorDual {
    var self_1007: Translator;
    var other_873: MotorDual;

    self_1007 = self_1006;
    other_873 = other_872;
    let _e4: Translator = self_1007;
    let _e8: MotorDual = other_873;
    let _e11: Translator = self_1007;
    let _e15: MotorDual = other_873;
    let _e28: Translator = self_1007;
    let _e31: Translator = self_1007;
    let _e34: Translator = self_1007;
    let _e37: Translator = self_1007;
    let _e41: MotorDual = other_873;
    return MotorDual((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.z) * _e15.g0_.zzyx) * vec4<f32>(0.0, -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e28.g0_.x, _e31.g0_.y, _e34.g0_.y, _e37.g0_.y) * _e41.g0_.xwxy) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))));
}

fn translator_motor_dual_left_contraction(self_1008: Translator, other_874: MotorDual) -> MotorDual {
    var self_1009: Translator;
    var other_875: MotorDual;

    self_1009 = self_1008;
    other_875 = other_874;
    let _e4: Translator = self_1009;
    let _e8: MotorDual = other_875;
    let _e11: Translator = self_1009;
    let _e14: Translator = self_1009;
    let _e17: Translator = self_1009;
    let _e20: Translator = self_1009;
    let _e24: MotorDual = other_875;
    return MotorDual(((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.x, _e14.g0_.x, _e17.g0_.y, _e20.g0_.z) * vec4<f32>(_e24.g0_.x)) * vec4<f32>(0.0, 0.0, -(1.0), -(1.0)))));
}

fn translator_motor_dual_right_contraction(self_1010: Translator, other_876: MotorDual) -> Plane {
    var self_1011: Translator;
    var other_877: MotorDual;

    self_1011 = self_1010;
    other_877 = other_876;
    let _e4: Translator = self_1011;
    let _e8: MotorDual = other_877;
    let _e11: MotorDual = other_877;
    let _e14: MotorDual = other_877;
    let _e25: Translator = self_1011;
    let _e28: MotorDual = other_877;
    let _e31: MotorDual = other_877;
    let _e34: MotorDual = other_877;
    return Plane((((vec3<f32>(_e4.g0_.z) * vec3<f32>(_e8.g0_.z, _e11.g0_.y, _e14.g0_.z)) * vec3<f32>(-(1.0), 1.0, 0.0)) + ((_e25.g0_.yxy * vec3<f32>(_e28.g0_.w, _e31.g0_.x, _e34.g0_.y)) * vec3<f32>(1.0, 0.0, -(1.0)))));
}

fn translator_squared_magnitude(self_1012: Translator) -> Scalar {
    var self_1013: Translator;

    self_1013 = self_1012;
    let _e2: Translator = self_1013;
    let _e3: Translator = self_1013;
    let _e4: Translator = translator_reversal(_e3);
    let _e5: Scalar = translator_translator_scalar_product(_e2, _e4);
    return _e5;
}

fn translator_magnitude(self_1014: Translator) -> Scalar {
    var self_1015: Translator;

    self_1015 = self_1014;
    let _e2: Translator = self_1015;
    let _e3: Scalar = translator_squared_magnitude(_e2);
    return Scalar(sqrt(_e3.g0_));
}

fn translator_scale(self_1016: Translator, other_878: f32) -> Translator {
    var self_1017: Translator;
    var other_879: f32;

    self_1017 = self_1016;
    other_879 = other_878;
    let _e4: Translator = self_1017;
    let _e5: f32 = other_879;
    let _e7: Translator = translator_scalar_geometric_product(_e4, Scalar(_e5));
    return _e7;
}

fn translator_signum(self_1018: Translator) -> Translator {
    var self_1019: Translator;

    self_1019 = self_1018;
    let _e2: Translator = self_1019;
    let _e3: Translator = self_1019;
    let _e4: Scalar = translator_magnitude(_e3);
    let _e9: Translator = translator_scalar_geometric_product(_e2, Scalar((1.0 / _e4.g0_)));
    return _e9;
}

fn translator_inverse(self_1020: Translator) -> Translator {
    var self_1021: Translator;

    self_1021 = self_1020;
    let _e2: Translator = self_1021;
    let _e3: Translator = translator_reversal(_e2);
    let _e4: Translator = self_1021;
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

fn motor_neg(self_1022: Motor) -> Motor {
    var self_1023: Motor;

    self_1023 = self_1022;
    let _e2: Motor = self_1023;
    return Motor((_e2.g0_ * vec4<f32>(-(1.0))));
}

fn motor_automorphism(self_1024: Motor) -> Motor {
    var self_1025: Motor;

    self_1025 = self_1024;
    let _e2: Motor = self_1025;
    return Motor(_e2.g0_);
}

fn motor_reversal(self_1026: Motor) -> Motor {
    var self_1027: Motor;

    self_1027 = self_1026;
    let _e2: Motor = self_1027;
    return Motor((_e2.g0_ * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))));
}

fn motor_conjugation(self_1028: Motor) -> Motor {
    var self_1029: Motor;

    self_1029 = self_1028;
    let _e2: Motor = self_1029;
    return Motor((_e2.g0_ * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))));
}

fn motor_dual(self_1030: Motor) -> MotorDual {
    var self_1031: Motor;

    self_1031 = self_1030;
    let _e2: Motor = self_1031;
    return MotorDual(_e2.g0_);
}

fn motor_scalar_into(self_1032: Motor) -> Scalar {
    var self_1033: Motor;

    self_1033 = self_1032;
    let _e2: Motor = self_1033;
    return Scalar(_e2.g0_.x);
}

fn motor_scalar_add(self_1034: Motor, other_880: Scalar) -> Motor {
    var self_1035: Motor;
    var other_881: Scalar;

    self_1035 = self_1034;
    other_881 = other_880;
    let _e4: Motor = self_1035;
    let _e6: Scalar = other_881;
    return Motor((_e4.g0_ + (vec4<f32>(_e6.g0_) * vec4<f32>(1.0, 0.0, 0.0, 0.0))));
}

fn motor_scalar_sub(self_1036: Motor, other_882: Scalar) -> Motor {
    var self_1037: Motor;
    var other_883: Scalar;

    self_1037 = self_1036;
    other_883 = other_882;
    let _e4: Motor = self_1037;
    let _e6: Scalar = other_883;
    return Motor((_e4.g0_ - (vec4<f32>(_e6.g0_) * vec4<f32>(1.0, 0.0, 0.0, 0.0))));
}

fn motor_scalar_geometric_product(self_1038: Motor, other_884: Scalar) -> Motor {
    var self_1039: Motor;
    var other_885: Scalar;

    self_1039 = self_1038;
    other_885 = other_884;
    let _e4: Motor = self_1039;
    let _e6: Scalar = other_885;
    return Motor((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn motor_scalar_outer_product(self_1040: Motor, other_886: Scalar) -> Motor {
    var self_1041: Motor;
    var other_887: Scalar;

    self_1041 = self_1040;
    other_887 = other_886;
    let _e4: Motor = self_1041;
    let _e6: Scalar = other_887;
    return Motor((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn motor_scalar_inner_product(self_1042: Motor, other_888: Scalar) -> Motor {
    var self_1043: Motor;
    var other_889: Scalar;

    self_1043 = self_1042;
    other_889 = other_888;
    let _e4: Motor = self_1043;
    let _e6: Scalar = other_889;
    return Motor((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn motor_scalar_left_contraction(self_1044: Motor, other_890: Scalar) -> Scalar {
    var self_1045: Motor;
    var other_891: Scalar;

    self_1045 = self_1044;
    other_891 = other_890;
    let _e4: Motor = self_1045;
    let _e7: Scalar = other_891;
    return Scalar((_e4.g0_.x * _e7.g0_));
}

fn motor_scalar_right_contraction(self_1046: Motor, other_892: Scalar) -> Motor {
    var self_1047: Motor;
    var other_893: Scalar;

    self_1047 = self_1046;
    other_893 = other_892;
    let _e4: Motor = self_1047;
    let _e6: Scalar = other_893;
    return Motor((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn motor_scalar_scalar_product(self_1048: Motor, other_894: Scalar) -> Scalar {
    var self_1049: Motor;
    var other_895: Scalar;

    self_1049 = self_1048;
    other_895 = other_894;
    let _e4: Motor = self_1049;
    let _e7: Scalar = other_895;
    return Scalar((_e4.g0_.x * _e7.g0_));
}

fn motor_multi_vector_add(self_1050: Motor, other_896: MultiVector) -> MultiVector {
    var self_1051: Motor;
    var other_897: MultiVector;

    self_1051 = self_1050;
    other_897 = other_896;
    let _e4: Motor = self_1051;
    let _e13: MultiVector = other_897;
    let _e16: Motor = self_1051;
    let _e25: MultiVector = other_897;
    return MultiVector(((_e4.g0_.xyxx * vec4<f32>(1.0, 1.0, 0.0, 0.0)) + _e13.g0_), ((_e16.g0_.xxzw * vec4<f32>(0.0, 0.0, 1.0, 1.0)) + _e25.g1_));
}

fn motor_multi_vector_sub(self_1052: Motor, other_898: MultiVector) -> MultiVector {
    var self_1053: Motor;
    var other_899: MultiVector;

    self_1053 = self_1052;
    other_899 = other_898;
    let _e4: Motor = self_1053;
    let _e13: MultiVector = other_899;
    let _e16: Motor = self_1053;
    let _e25: MultiVector = other_899;
    return MultiVector(((_e4.g0_.xyxx * vec4<f32>(1.0, 1.0, 0.0, 0.0)) - _e13.g0_), ((_e16.g0_.xxzw * vec4<f32>(0.0, 0.0, 1.0, 1.0)) - _e25.g1_));
}

fn motor_multi_vector_geometric_product(self_1054: Motor, other_900: MultiVector) -> MultiVector {
    var self_1055: Motor;
    var other_901: MultiVector;

    self_1055 = self_1054;
    other_901 = other_900;
    let _e4: Motor = self_1055;
    let _e8: MultiVector = other_901;
    let _e11: Motor = self_1055;
    let _e15: MultiVector = other_901;
    let _e28: Motor = self_1055;
    let _e32: MultiVector = other_901;
    let _e46: Motor = self_1055;
    let _e50: MultiVector = other_901;
    let _e64: Motor = self_1055;
    let _e68: MultiVector = other_901;
    let _e71: Motor = self_1055;
    let _e75: MultiVector = other_901;
    let _e88: Motor = self_1055;
    let _e92: MultiVector = other_901;
    let _e104: Motor = self_1055;
    let _e108: MultiVector = other_901;
    return MultiVector(((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.y) * _e15.g0_.yxwz) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e28.g0_.z) * _e32.g1_.zwxy) * vec4<f32>(-(1.0), 1.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e46.g0_.w) * _e50.g1_.wzyx) * vec4<f32>(-(1.0), -(1.0), -(1.0), 1.0))), ((((vec4<f32>(_e64.g0_.x) * _e68.g1_) + ((vec4<f32>(_e71.g0_.y) * _e75.g1_.yxwz) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))) + ((vec4<f32>(_e88.g0_.z) * _e92.g0_.zwxy) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e104.g0_.w) * _e108.g0_.wzyx) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0))));
}

fn motor_multi_vector_outer_product(self_1056: Motor, other_902: MultiVector) -> MultiVector {
    var self_1057: Motor;
    var other_903: MultiVector;

    self_1057 = self_1056;
    other_903 = other_902;
    let _e4: Motor = self_1057;
    let _e8: MultiVector = other_903;
    let _e11: Motor = self_1057;
    let _e14: MultiVector = other_903;
    let _e26: Motor = self_1057;
    let _e30: MultiVector = other_903;
    let _e33: Motor = self_1057;
    let _e37: MultiVector = other_903;
    let _e48: Motor = self_1057;
    let _e52: MultiVector = other_903;
    let _e63: Motor = self_1057;
    let _e66: MultiVector = other_903;
    return MultiVector(((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((_e11.g0_.xyxx * vec4<f32>(_e14.g0_.x)) * vec4<f32>(0.0, 1.0, 0.0, 0.0))), ((((vec4<f32>(_e26.g0_.x) * _e30.g1_) + ((vec4<f32>(_e33.g0_.z) * _e37.g0_.wwxw) * vec4<f32>(0.0, 1.0, 1.0, 0.0))) + ((vec4<f32>(_e48.g0_.w) * _e52.g0_.zzzx) * vec4<f32>(0.0, 1.0, 0.0, 1.0))) + ((_e63.g0_.xyxx * vec4<f32>(_e66.g1_.x)) * vec4<f32>(0.0, 1.0, 0.0, 0.0))));
}

fn motor_multi_vector_inner_product(self_1058: Motor, other_904: MultiVector) -> MultiVector {
    var self_1059: Motor;
    var other_905: MultiVector;

    self_1059 = self_1058;
    other_905 = other_904;
    let _e4: Motor = self_1059;
    let _e8: MultiVector = other_905;
    let _e11: Motor = self_1059;
    let _e15: MultiVector = other_905;
    let _e28: Motor = self_1059;
    let _e32: MultiVector = other_905;
    let _e45: Motor = self_1059;
    let _e48: MultiVector = other_905;
    let _e62: Motor = self_1059;
    let _e66: MultiVector = other_905;
    let _e69: Motor = self_1059;
    let _e73: MultiVector = other_905;
    let _e84: Motor = self_1059;
    let _e88: MultiVector = other_905;
    let _e100: Motor = self_1059;
    let _e103: MultiVector = other_905;
    return MultiVector(((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.y) * _e15.g0_.yxwz) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e28.g0_.w) * _e32.g1_.wwyx) * vec4<f32>(-(1.0), 0.0, -(1.0), 1.0))) + ((_e45.g0_.zxzz * _e48.g1_.zxxy) * vec4<f32>(-(1.0), 0.0, -(1.0), -(1.0)))), ((((vec4<f32>(_e62.g0_.x) * _e66.g1_) + ((vec4<f32>(_e69.g0_.z) * _e73.g0_.zzxz) * vec4<f32>(1.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e84.g0_.w) * _e88.g0_.wwwx) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))) + ((_e100.g0_.yxxx * _e103.g1_.yxxx) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))));
}

fn motor_multi_vector_left_contraction(self_1060: Motor, other_906: MultiVector) -> MultiVector {
    var self_1061: Motor;
    var other_907: MultiVector;

    self_1061 = self_1060;
    other_907 = other_906;
    let _e4: Motor = self_1061;
    let _e8: MultiVector = other_907;
    let _e11: Motor = self_1061;
    let _e15: MultiVector = other_907;
    let _e28: Motor = self_1061;
    let _e32: MultiVector = other_907;
    let _e45: Motor = self_1061;
    let _e48: MultiVector = other_907;
    let _e60: Motor = self_1061;
    let _e64: MultiVector = other_907;
    let _e67: Motor = self_1061;
    let _e70: MultiVector = other_907;
    return MultiVector(((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.z) * _e15.g1_.zzzy) * vec4<f32>(-(1.0), 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e28.g0_.w) * _e32.g1_.wwyw) * vec4<f32>(-(1.0), 0.0, -(1.0), 0.0))) + ((_e45.g0_.yxxx * _e48.g0_.yxxx) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))), ((vec4<f32>(_e60.g0_.x) * _e64.g1_) + ((_e67.g0_.yxxx * _e70.g1_.yxxx) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))));
}

fn motor_multi_vector_scalar_product(self_1062: Motor, other_908: MultiVector) -> Scalar {
    var self_1063: Motor;
    var other_909: MultiVector;

    self_1063 = self_1062;
    other_909 = other_908;
    let _e4: Motor = self_1063;
    let _e7: MultiVector = other_909;
    let _e11: Motor = self_1063;
    let _e14: MultiVector = other_909;
    let _e19: Motor = self_1063;
    let _e22: MultiVector = other_909;
    let _e27: Motor = self_1063;
    let _e30: MultiVector = other_909;
    return Scalar(((((_e4.g0_.x * _e7.g0_.x) - (_e11.g0_.y * _e14.g0_.y)) - (_e19.g0_.z * _e22.g1_.z)) - (_e27.g0_.w * _e30.g1_.w)));
}

fn motor_rotor_into(self_1064: Motor) -> Rotor {
    var self_1065: Motor;

    self_1065 = self_1064;
    let _e2: Motor = self_1065;
    let _e5: Motor = self_1065;
    return Rotor(vec2<f32>(_e2.g0_.x, _e5.g0_.y));
}

fn motor_rotor_add(self_1066: Motor, other_910: Rotor) -> Motor {
    var self_1067: Motor;
    var other_911: Rotor;

    self_1067 = self_1066;
    other_911 = other_910;
    let _e4: Motor = self_1067;
    let _e6: Rotor = other_911;
    let _e9: Rotor = other_911;
    let _e12: Rotor = other_911;
    let _e15: Rotor = other_911;
    return Motor((_e4.g0_ + (vec4<f32>(_e6.g0_.x, _e9.g0_.y, _e12.g0_.x, _e15.g0_.x) * vec4<f32>(1.0, 1.0, 0.0, 0.0))));
}

fn motor_rotor_sub(self_1068: Motor, other_912: Rotor) -> Motor {
    var self_1069: Motor;
    var other_913: Rotor;

    self_1069 = self_1068;
    other_913 = other_912;
    let _e4: Motor = self_1069;
    let _e6: Rotor = other_913;
    let _e9: Rotor = other_913;
    let _e12: Rotor = other_913;
    let _e15: Rotor = other_913;
    return Motor((_e4.g0_ - (vec4<f32>(_e6.g0_.x, _e9.g0_.y, _e12.g0_.x, _e15.g0_.x) * vec4<f32>(1.0, 1.0, 0.0, 0.0))));
}

fn motor_rotor_geometric_product(self_1070: Motor, other_914: Rotor) -> Motor {
    var self_1071: Motor;
    var other_915: Rotor;

    self_1071 = self_1070;
    other_915 = other_914;
    let _e4: Motor = self_1071;
    let _e8: Rotor = other_915;
    let _e11: Rotor = other_915;
    let _e14: Rotor = other_915;
    let _e17: Rotor = other_915;
    let _e29: Motor = self_1071;
    let _e33: Rotor = other_915;
    let _e36: Rotor = other_915;
    let _e39: Rotor = other_915;
    let _e42: Rotor = other_915;
    let _e54: Motor = self_1071;
    let _e57: Rotor = other_915;
    let _e60: Rotor = other_915;
    let _e63: Rotor = other_915;
    let _e66: Rotor = other_915;
    return Motor(((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y, _e11.g0_.x, _e14.g0_.y, _e17.g0_.y)) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0)) + ((vec4<f32>(_e29.g0_.w) * vec4<f32>(_e33.g0_.y, _e36.g0_.y, _e39.g0_.y, _e42.g0_.x)) * vec4<f32>(0.0, 0.0, 1.0, 1.0))) + ((_e54.g0_.xxzz * vec4<f32>(_e57.g0_.x, _e60.g0_.y, _e63.g0_.x, _e66.g0_.y)) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))));
}

fn motor_rotor_outer_product(self_1072: Motor, other_916: Rotor) -> Motor {
    var self_1073: Motor;
    var other_917: Rotor;

    self_1073 = self_1072;
    other_917 = other_916;
    let _e4: Motor = self_1073;
    let _e8: Rotor = other_917;
    let _e19: Motor = self_1073;
    let _e22: Rotor = other_917;
    let _e25: Rotor = other_917;
    let _e28: Rotor = other_917;
    let _e31: Rotor = other_917;
    return Motor((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.x)) * vec4<f32>(0.0, 1.0, 0.0, 0.0)) + (_e19.g0_.xxzw * vec4<f32>(_e22.g0_.x, _e25.g0_.y, _e28.g0_.x, _e31.g0_.x))));
}

fn motor_rotor_inner_product(self_1074: Motor, other_918: Rotor) -> Motor {
    var self_1075: Motor;
    var other_919: Rotor;

    self_1075 = self_1074;
    other_919 = other_918;
    let _e4: Motor = self_1075;
    let _e8: Rotor = other_919;
    let _e11: Rotor = other_919;
    let _e14: Rotor = other_919;
    let _e17: Rotor = other_919;
    let _e29: Motor = self_1075;
    let _e32: Rotor = other_919;
    let _e35: Rotor = other_919;
    let _e38: Rotor = other_919;
    let _e41: Rotor = other_919;
    return Motor((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y, _e11.g0_.x, _e14.g0_.y, _e17.g0_.y)) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0)) + (_e29.g0_.xxzw * vec4<f32>(_e32.g0_.x, _e35.g0_.y, _e38.g0_.x, _e41.g0_.x))));
}

fn motor_rotor_left_contraction(self_1076: Motor, other_920: Rotor) -> Rotor {
    var self_1077: Motor;
    var other_921: Rotor;

    self_1077 = self_1076;
    other_921 = other_920;
    let _e4: Motor = self_1077;
    let _e8: Rotor = other_921;
    let _e11: Motor = self_1077;
    let _e14: Motor = self_1077;
    let _e18: Rotor = other_921;
    return Rotor(((vec2<f32>(_e4.g0_.x) * _e8.g0_) + ((vec2<f32>(_e11.g0_.y, _e14.g0_.x) * _e18.g0_.yx) * vec2<f32>(-(1.0), 0.0))));
}

fn motor_rotor_right_contraction(self_1078: Motor, other_922: Rotor) -> Motor {
    var self_1079: Motor;
    var other_923: Rotor;

    self_1079 = self_1078;
    other_923 = other_922;
    let _e4: Motor = self_1079;
    let _e8: Rotor = other_923;
    let _e11: Rotor = other_923;
    let _e14: Rotor = other_923;
    let _e17: Rotor = other_923;
    let _e29: Motor = self_1079;
    let _e32: Rotor = other_923;
    return Motor((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y, _e11.g0_.x, _e14.g0_.y, _e17.g0_.y)) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0)) + ((_e29.g0_.xxzw * vec4<f32>(_e32.g0_.x)) * vec4<f32>(1.0, 0.0, 1.0, 1.0))));
}

fn motor_rotor_scalar_product(self_1080: Motor, other_924: Rotor) -> Scalar {
    var self_1081: Motor;
    var other_925: Rotor;

    self_1081 = self_1080;
    other_925 = other_924;
    let _e4: Motor = self_1081;
    let _e7: Rotor = other_925;
    let _e11: Motor = self_1081;
    let _e14: Rotor = other_925;
    return Scalar(((_e4.g0_.x * _e7.g0_.x) - (_e11.g0_.y * _e14.g0_.y)));
}

fn motor_point_into(self_1082: Motor) -> Point {
    var self_1083: Motor;

    self_1083 = self_1082;
    let _e2: Motor = self_1083;
    let _e5: Motor = self_1083;
    let _e8: Motor = self_1083;
    return Point(vec3<f32>(_e2.g0_.y, _e5.g0_.z, _e8.g0_.w));
}

fn motor_point_add(self_1084: Motor, other_926: Point) -> Motor {
    var self_1085: Motor;
    var other_927: Point;

    self_1085 = self_1084;
    other_927 = other_926;
    let _e4: Motor = self_1085;
    let _e6: Point = other_927;
    let _e9: Point = other_927;
    let _e12: Point = other_927;
    let _e15: Point = other_927;
    return Motor((_e4.g0_ + (vec4<f32>(_e6.g0_.x, _e9.g0_.x, _e12.g0_.y, _e15.g0_.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn motor_point_sub(self_1086: Motor, other_928: Point) -> Motor {
    var self_1087: Motor;
    var other_929: Point;

    self_1087 = self_1086;
    other_929 = other_928;
    let _e4: Motor = self_1087;
    let _e6: Point = other_929;
    let _e9: Point = other_929;
    let _e12: Point = other_929;
    let _e15: Point = other_929;
    return Motor((_e4.g0_ - (vec4<f32>(_e6.g0_.x, _e9.g0_.x, _e12.g0_.y, _e15.g0_.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn motor_point_geometric_product(self_1088: Motor, other_930: Point) -> Motor {
    var self_1089: Motor;
    var other_931: Point;

    self_1089 = self_1088;
    other_931 = other_930;
    let _e4: Motor = self_1089;
    let _e8: Point = other_931;
    let _e11: Point = other_931;
    let _e14: Point = other_931;
    let _e17: Point = other_931;
    let _e30: Motor = self_1089;
    let _e34: Point = other_931;
    let _e37: Point = other_931;
    let _e40: Point = other_931;
    let _e43: Point = other_931;
    let _e57: Motor = self_1089;
    let _e61: Point = other_931;
    let _e64: Point = other_931;
    let _e67: Point = other_931;
    let _e70: Point = other_931;
    let _e84: Motor = self_1089;
    let _e88: Point = other_931;
    let _e91: Point = other_931;
    let _e94: Point = other_931;
    let _e97: Point = other_931;
    return Motor((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.x, _e11.g0_.x, _e14.g0_.z, _e17.g0_.y)) * vec4<f32>(-(1.0), 0.0, -(1.0), 1.0)) + ((vec4<f32>(_e30.g0_.z) * vec4<f32>(_e34.g0_.y, _e37.g0_.z, _e40.g0_.y, _e43.g0_.x)) * vec4<f32>(-(1.0), 1.0, 0.0, -(1.0)))) + ((vec4<f32>(_e57.g0_.w) * vec4<f32>(_e61.g0_.z, _e64.g0_.y, _e67.g0_.x, _e70.g0_.z)) * vec4<f32>(-(1.0), -(1.0), 1.0, 0.0))) + ((vec4<f32>(_e84.g0_.x) * vec4<f32>(_e88.g0_.x, _e91.g0_.x, _e94.g0_.y, _e97.g0_.z)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn motor_point_regressive_product(self_1090: Motor, other_932: Point) -> Plane {
    var self_1091: Motor;
    var other_933: Point;

    self_1091 = self_1090;
    other_933 = other_932;
    let _e4: Motor = self_1091;
    let _e8: Point = other_933;
    let _e18: Motor = self_1091;
    let _e22: Point = other_933;
    let _e33: Motor = self_1091;
    let _e36: Motor = self_1091;
    let _e39: Motor = self_1091;
    let _e43: Point = other_933;
    return Plane(((((vec3<f32>(_e4.g0_.z) * _e8.g0_.zzx) * vec3<f32>(-(1.0), 0.0, 1.0)) + ((vec3<f32>(_e18.g0_.w) * _e22.g0_.yxy) * vec3<f32>(1.0, -(1.0), 0.0))) + ((vec3<f32>(_e33.g0_.x, _e36.g0_.y, _e39.g0_.y) * _e43.g0_.xzy) * vec3<f32>(0.0, 1.0, -(1.0)))));
}

fn motor_point_outer_product(self_1092: Motor, other_934: Point) -> Point {
    var self_1093: Motor;
    var other_935: Point;

    self_1093 = self_1092;
    other_935 = other_934;
    let _e4: Motor = self_1093;
    let _e8: Point = other_935;
    return Point((vec3<f32>(_e4.g0_.x) * _e8.g0_));
}

fn motor_point_inner_product(self_1094: Motor, other_936: Point) -> Motor {
    var self_1095: Motor;
    var other_937: Point;

    self_1095 = self_1094;
    other_937 = other_936;
    let _e4: Motor = self_1095;
    let _e8: Point = other_937;
    let _e20: Motor = self_1095;
    let _e24: Point = other_937;
    let _e37: Motor = self_1095;
    let _e40: Point = other_937;
    let _e43: Point = other_937;
    let _e46: Point = other_937;
    let _e49: Point = other_937;
    return Motor(((((vec4<f32>(_e4.g0_.z) * vec4<f32>(_e8.g0_.y)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0)) + ((vec4<f32>(_e20.g0_.w) * vec4<f32>(_e24.g0_.z)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((_e37.g0_.yxxx * vec4<f32>(_e40.g0_.x, _e43.g0_.x, _e46.g0_.y, _e49.g0_.z)) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0))));
}

fn motor_point_left_contraction(self_1096: Motor, other_938: Point) -> Motor {
    var self_1097: Motor;
    var other_939: Point;

    self_1097 = self_1096;
    other_939 = other_938;
    let _e4: Motor = self_1097;
    let _e8: Point = other_939;
    let _e20: Motor = self_1097;
    let _e24: Point = other_939;
    let _e37: Motor = self_1097;
    let _e40: Point = other_939;
    let _e43: Point = other_939;
    let _e46: Point = other_939;
    let _e49: Point = other_939;
    return Motor(((((vec4<f32>(_e4.g0_.z) * vec4<f32>(_e8.g0_.y)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0)) + ((vec4<f32>(_e20.g0_.w) * vec4<f32>(_e24.g0_.z)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((_e37.g0_.yxxx * vec4<f32>(_e40.g0_.x, _e43.g0_.x, _e46.g0_.y, _e49.g0_.z)) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0))));
}

fn motor_point_right_contraction(self_1098: Motor, other_940: Point) -> Scalar {
    var self_1099: Motor;
    var other_941: Point;

    self_1099 = self_1098;
    other_941 = other_940;
    let _e5: Motor = self_1099;
    let _e8: Point = other_941;
    let _e13: Motor = self_1099;
    let _e16: Point = other_941;
    let _e21: Motor = self_1099;
    let _e24: Point = other_941;
    return Scalar((((0.0 - (_e5.g0_.y * _e8.g0_.x)) - (_e13.g0_.z * _e16.g0_.y)) - (_e21.g0_.w * _e24.g0_.z)));
}

fn motor_point_scalar_product(self_1100: Motor, other_942: Point) -> Scalar {
    var self_1101: Motor;
    var other_943: Point;

    self_1101 = self_1100;
    other_943 = other_942;
    let _e5: Motor = self_1101;
    let _e8: Point = other_943;
    let _e13: Motor = self_1101;
    let _e16: Point = other_943;
    let _e21: Motor = self_1101;
    let _e24: Point = other_943;
    return Scalar((((0.0 - (_e5.g0_.y * _e8.g0_.x)) - (_e13.g0_.z * _e16.g0_.y)) - (_e21.g0_.w * _e24.g0_.z)));
}

fn motor_ideal_point_into(self_1102: Motor) -> IdealPoint {
    var self_1103: Motor;

    self_1103 = self_1102;
    let _e2: Motor = self_1103;
    let _e5: Motor = self_1103;
    return IdealPoint(vec2<f32>(_e2.g0_.z, _e5.g0_.w));
}

fn motor_ideal_point_add(self_1104: Motor, other_944: IdealPoint) -> Motor {
    var self_1105: Motor;
    var other_945: IdealPoint;

    self_1105 = self_1104;
    other_945 = other_944;
    let _e4: Motor = self_1105;
    let _e6: IdealPoint = other_945;
    let _e9: IdealPoint = other_945;
    let _e12: IdealPoint = other_945;
    let _e15: IdealPoint = other_945;
    return Motor((_e4.g0_ + (vec4<f32>(_e6.g0_.x, _e9.g0_.x, _e12.g0_.x, _e15.g0_.y) * vec4<f32>(0.0, 0.0, 1.0, 1.0))));
}

fn motor_ideal_point_sub(self_1106: Motor, other_946: IdealPoint) -> Motor {
    var self_1107: Motor;
    var other_947: IdealPoint;

    self_1107 = self_1106;
    other_947 = other_946;
    let _e4: Motor = self_1107;
    let _e6: IdealPoint = other_947;
    let _e9: IdealPoint = other_947;
    let _e12: IdealPoint = other_947;
    let _e15: IdealPoint = other_947;
    return Motor((_e4.g0_ - (vec4<f32>(_e6.g0_.x, _e9.g0_.x, _e12.g0_.x, _e15.g0_.y) * vec4<f32>(0.0, 0.0, 1.0, 1.0))));
}

fn motor_ideal_point_geometric_product(self_1108: Motor, other_948: IdealPoint) -> Motor {
    var self_1109: Motor;
    var other_949: IdealPoint;

    self_1109 = self_1108;
    other_949 = other_948;
    let _e4: Motor = self_1109;
    let _e8: IdealPoint = other_949;
    let _e11: IdealPoint = other_949;
    let _e14: IdealPoint = other_949;
    let _e17: IdealPoint = other_949;
    let _e29: Motor = self_1109;
    let _e33: IdealPoint = other_949;
    let _e36: IdealPoint = other_949;
    let _e39: IdealPoint = other_949;
    let _e42: IdealPoint = other_949;
    let _e56: Motor = self_1109;
    let _e59: IdealPoint = other_949;
    let _e62: IdealPoint = other_949;
    let _e65: IdealPoint = other_949;
    let _e68: IdealPoint = other_949;
    return Motor(((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y, _e11.g0_.y, _e14.g0_.y, _e17.g0_.x)) * vec4<f32>(0.0, 0.0, -(1.0), 1.0)) + ((vec4<f32>(_e29.g0_.w) * vec4<f32>(_e33.g0_.y, _e36.g0_.x, _e39.g0_.y, _e42.g0_.y)) * vec4<f32>(-(1.0), -(1.0), 0.0, 0.0))) + ((_e56.g0_.zzxx * vec4<f32>(_e59.g0_.x, _e62.g0_.y, _e65.g0_.x, _e68.g0_.y)) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0))));
}

fn motor_ideal_point_regressive_product(self_1110: Motor, other_950: IdealPoint) -> Plane {
    var self_1111: Motor;
    var other_951: IdealPoint;

    self_1111 = self_1110;
    other_951 = other_950;
    let _e4: Motor = self_1111;
    let _e8: IdealPoint = other_951;
    let _e18: Motor = self_1111;
    let _e21: Motor = self_1111;
    let _e24: Motor = self_1111;
    let _e28: IdealPoint = other_951;
    let _e31: IdealPoint = other_951;
    let _e34: IdealPoint = other_951;
    return Plane((((vec3<f32>(_e4.g0_.w) * vec3<f32>(_e8.g0_.x)) * vec3<f32>(1.0, 0.0, 0.0)) + ((vec3<f32>(_e18.g0_.z, _e21.g0_.y, _e24.g0_.y) * vec3<f32>(_e28.g0_.y, _e31.g0_.y, _e34.g0_.x)) * vec3<f32>(-(1.0), 1.0, -(1.0)))));
}

fn motor_ideal_point_outer_product(self_1112: Motor, other_952: IdealPoint) -> IdealPoint {
    var self_1113: Motor;
    var other_953: IdealPoint;

    self_1113 = self_1112;
    other_953 = other_952;
    let _e4: Motor = self_1113;
    let _e8: IdealPoint = other_953;
    return IdealPoint((vec2<f32>(_e4.g0_.x) * _e8.g0_));
}

fn motor_ideal_point_inner_product(self_1114: Motor, other_954: IdealPoint) -> Translator {
    var self_1115: Motor;
    var other_955: IdealPoint;

    self_1115 = self_1114;
    other_955 = other_954;
    let _e4: Motor = self_1115;
    let _e8: IdealPoint = other_955;
    let _e19: Motor = self_1115;
    let _e22: Motor = self_1115;
    let _e25: Motor = self_1115;
    let _e29: IdealPoint = other_955;
    let _e32: IdealPoint = other_955;
    let _e35: IdealPoint = other_955;
    return Translator((((vec3<f32>(_e4.g0_.w) * vec3<f32>(_e8.g0_.y)) * vec3<f32>(-(1.0), 0.0, 0.0)) + ((vec3<f32>(_e19.g0_.z, _e22.g0_.x, _e25.g0_.x) * vec3<f32>(_e29.g0_.x, _e32.g0_.x, _e35.g0_.y)) * vec3<f32>(-(1.0), 1.0, 1.0))));
}

fn motor_ideal_point_left_contraction(self_1116: Motor, other_956: IdealPoint) -> Translator {
    var self_1117: Motor;
    var other_957: IdealPoint;

    self_1117 = self_1116;
    other_957 = other_956;
    let _e4: Motor = self_1117;
    let _e8: IdealPoint = other_957;
    let _e19: Motor = self_1117;
    let _e22: Motor = self_1117;
    let _e25: Motor = self_1117;
    let _e29: IdealPoint = other_957;
    let _e32: IdealPoint = other_957;
    let _e35: IdealPoint = other_957;
    return Translator((((vec3<f32>(_e4.g0_.w) * vec3<f32>(_e8.g0_.y)) * vec3<f32>(-(1.0), 0.0, 0.0)) + ((vec3<f32>(_e19.g0_.z, _e22.g0_.x, _e25.g0_.x) * vec3<f32>(_e29.g0_.x, _e32.g0_.x, _e35.g0_.y)) * vec3<f32>(-(1.0), 1.0, 1.0))));
}

fn motor_ideal_point_right_contraction(self_1118: Motor, other_958: IdealPoint) -> Scalar {
    var self_1119: Motor;
    var other_959: IdealPoint;

    self_1119 = self_1118;
    other_959 = other_958;
    let _e5: Motor = self_1119;
    let _e8: IdealPoint = other_959;
    let _e13: Motor = self_1119;
    let _e16: IdealPoint = other_959;
    return Scalar(((0.0 - (_e5.g0_.z * _e8.g0_.x)) - (_e13.g0_.w * _e16.g0_.y)));
}

fn motor_ideal_point_scalar_product(self_1120: Motor, other_960: IdealPoint) -> Scalar {
    var self_1121: Motor;
    var other_961: IdealPoint;

    self_1121 = self_1120;
    other_961 = other_960;
    let _e5: Motor = self_1121;
    let _e8: IdealPoint = other_961;
    let _e13: Motor = self_1121;
    let _e16: IdealPoint = other_961;
    return Scalar(((0.0 - (_e5.g0_.z * _e8.g0_.x)) - (_e13.g0_.w * _e16.g0_.y)));
}

fn motor_plane_geometric_product(self_1122: Motor, other_962: Plane) -> MotorDual {
    var self_1123: Motor;
    var other_963: Plane;

    self_1123 = self_1122;
    other_963 = other_962;
    let _e4: Motor = self_1123;
    let _e8: Plane = other_963;
    let _e11: Plane = other_963;
    let _e14: Plane = other_963;
    let _e17: Plane = other_963;
    let _e29: Motor = self_1123;
    let _e33: Plane = other_963;
    let _e36: Plane = other_963;
    let _e39: Plane = other_963;
    let _e42: Plane = other_963;
    let _e55: Motor = self_1123;
    let _e59: Plane = other_963;
    let _e62: Plane = other_963;
    let _e65: Plane = other_963;
    let _e68: Plane = other_963;
    let _e81: Motor = self_1123;
    let _e85: Plane = other_963;
    let _e88: Plane = other_963;
    let _e91: Plane = other_963;
    let _e94: Plane = other_963;
    return MotorDual((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.x, _e11.g0_.x, _e14.g0_.z, _e17.g0_.y)) * vec4<f32>(1.0, 0.0, -(1.0), 1.0)) + ((vec4<f32>(_e29.g0_.z) * vec4<f32>(_e33.g0_.y, _e36.g0_.z, _e39.g0_.y, _e42.g0_.x)) * vec4<f32>(1.0, 1.0, 0.0, -(1.0)))) + ((vec4<f32>(_e55.g0_.w) * vec4<f32>(_e59.g0_.z, _e62.g0_.y, _e65.g0_.x, _e68.g0_.z)) * vec4<f32>(1.0, -(1.0), 1.0, 0.0))) + ((vec4<f32>(_e81.g0_.x) * vec4<f32>(_e85.g0_.x, _e88.g0_.x, _e91.g0_.y, _e94.g0_.z)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn motor_plane_regressive_product(self_1124: Motor, other_964: Plane) -> Scalar {
    var self_1125: Motor;
    var other_965: Plane;

    self_1125 = self_1124;
    other_965 = other_964;
    let _e4: Motor = self_1125;
    let _e7: Plane = other_965;
    let _e11: Motor = self_1125;
    let _e14: Plane = other_965;
    let _e19: Motor = self_1125;
    let _e22: Plane = other_965;
    return Scalar((((_e4.g0_.y * _e7.g0_.x) + (_e11.g0_.z * _e14.g0_.y)) + (_e19.g0_.w * _e22.g0_.z)));
}

fn motor_plane_outer_product(self_1126: Motor, other_966: Plane) -> MotorDual {
    var self_1127: Motor;
    var other_967: Plane;

    self_1127 = self_1126;
    other_967 = other_966;
    let _e4: Motor = self_1127;
    let _e8: Plane = other_967;
    let _e19: Motor = self_1127;
    let _e23: Plane = other_967;
    let _e35: Motor = self_1127;
    let _e38: Plane = other_967;
    let _e41: Plane = other_967;
    let _e44: Plane = other_967;
    let _e47: Plane = other_967;
    return MotorDual(((((vec4<f32>(_e4.g0_.z) * vec4<f32>(_e8.g0_.y)) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) + ((vec4<f32>(_e19.g0_.w) * vec4<f32>(_e23.g0_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + (_e35.g0_.yxxx * vec4<f32>(_e38.g0_.x, _e41.g0_.x, _e44.g0_.y, _e47.g0_.z))));
}

fn motor_plane_inner_product(self_1128: Motor, other_968: Plane) -> Plane {
    var self_1129: Motor;
    var other_969: Plane;

    self_1129 = self_1128;
    other_969 = other_968;
    let _e4: Motor = self_1129;
    let _e8: Plane = other_969;
    let _e11: Motor = self_1129;
    let _e15: Plane = other_969;
    let _e26: Motor = self_1129;
    let _e30: Plane = other_969;
    let _e41: Motor = self_1129;
    let _e44: Motor = self_1129;
    let _e47: Motor = self_1129;
    let _e51: Plane = other_969;
    return Plane(((((vec3<f32>(_e4.g0_.x) * _e8.g0_) + ((vec3<f32>(_e11.g0_.z) * _e15.g0_.zzx) * vec3<f32>(1.0, 0.0, -(1.0)))) + ((vec3<f32>(_e26.g0_.w) * _e30.g0_.yxy) * vec3<f32>(-(1.0), 1.0, 0.0))) + ((vec3<f32>(_e41.g0_.x, _e44.g0_.y, _e47.g0_.y) * _e51.g0_.xzy) * vec3<f32>(0.0, -(1.0), 1.0))));
}

fn motor_plane_left_contraction(self_1130: Motor, other_970: Plane) -> Plane {
    var self_1131: Motor;
    var other_971: Plane;

    self_1131 = self_1130;
    other_971 = other_970;
    let _e4: Motor = self_1131;
    let _e8: Plane = other_971;
    return Plane((vec3<f32>(_e4.g0_.x) * _e8.g0_));
}

fn motor_plane_right_contraction(self_1132: Motor, other_972: Plane) -> Plane {
    var self_1133: Motor;
    var other_973: Plane;

    self_1133 = self_1132;
    other_973 = other_972;
    let _e4: Motor = self_1133;
    let _e8: Plane = other_973;
    let _e18: Motor = self_1133;
    let _e22: Plane = other_973;
    let _e33: Motor = self_1133;
    let _e36: Motor = self_1133;
    let _e39: Motor = self_1133;
    let _e43: Plane = other_973;
    return Plane(((((vec3<f32>(_e4.g0_.z) * _e8.g0_.zzx) * vec3<f32>(1.0, 0.0, -(1.0))) + ((vec3<f32>(_e18.g0_.w) * _e22.g0_.yxy) * vec3<f32>(-(1.0), 1.0, 0.0))) + ((vec3<f32>(_e33.g0_.x, _e36.g0_.y, _e39.g0_.y) * _e43.g0_.xzy) * vec3<f32>(0.0, -(1.0), 1.0))));
}

fn motor_translator_into(self_1134: Motor) -> Translator {
    var self_1135: Motor;

    self_1135 = self_1134;
    let _e2: Motor = self_1135;
    let _e5: Motor = self_1135;
    let _e8: Motor = self_1135;
    return Translator(vec3<f32>(_e2.g0_.x, _e5.g0_.z, _e8.g0_.w));
}

fn motor_translator_add(self_1136: Motor, other_974: Translator) -> Motor {
    var self_1137: Motor;
    var other_975: Translator;

    self_1137 = self_1136;
    other_975 = other_974;
    let _e4: Motor = self_1137;
    let _e6: Translator = other_975;
    let _e9: Translator = other_975;
    let _e12: Translator = other_975;
    let _e15: Translator = other_975;
    return Motor((_e4.g0_ + (vec4<f32>(_e6.g0_.x, _e9.g0_.x, _e12.g0_.y, _e15.g0_.z) * vec4<f32>(1.0, 0.0, 1.0, 1.0))));
}

fn motor_translator_sub(self_1138: Motor, other_976: Translator) -> Motor {
    var self_1139: Motor;
    var other_977: Translator;

    self_1139 = self_1138;
    other_977 = other_976;
    let _e4: Motor = self_1139;
    let _e6: Translator = other_977;
    let _e9: Translator = other_977;
    let _e12: Translator = other_977;
    let _e15: Translator = other_977;
    return Motor((_e4.g0_ - (vec4<f32>(_e6.g0_.x, _e9.g0_.x, _e12.g0_.y, _e15.g0_.z) * vec4<f32>(1.0, 0.0, 1.0, 1.0))));
}

fn motor_translator_geometric_product(self_1140: Motor, other_978: Translator) -> Motor {
    var self_1141: Motor;
    var other_979: Translator;

    self_1141 = self_1140;
    other_979 = other_978;
    let _e4: Motor = self_1141;
    let _e8: Translator = other_979;
    let _e11: Translator = other_979;
    let _e14: Translator = other_979;
    let _e17: Translator = other_979;
    let _e29: Motor = self_1141;
    let _e33: Translator = other_979;
    let _e36: Translator = other_979;
    let _e39: Translator = other_979;
    let _e42: Translator = other_979;
    let _e55: Motor = self_1141;
    let _e59: Translator = other_979;
    let _e62: Translator = other_979;
    let _e65: Translator = other_979;
    let _e68: Translator = other_979;
    let _e82: Motor = self_1141;
    let _e86: Translator = other_979;
    let _e89: Translator = other_979;
    let _e92: Translator = other_979;
    let _e95: Translator = other_979;
    return Motor((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.x, _e11.g0_.x, _e14.g0_.z, _e17.g0_.y)) * vec4<f32>(0.0, 1.0, -(1.0), 1.0)) + ((vec4<f32>(_e29.g0_.z) * vec4<f32>(_e33.g0_.y, _e36.g0_.z, _e39.g0_.x, _e42.g0_.y)) * vec4<f32>(-(1.0), 1.0, 1.0, 0.0))) + ((vec4<f32>(_e55.g0_.w) * vec4<f32>(_e59.g0_.z, _e62.g0_.y, _e65.g0_.z, _e68.g0_.x)) * vec4<f32>(-(1.0), -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e82.g0_.x) * vec4<f32>(_e86.g0_.x, _e89.g0_.x, _e92.g0_.y, _e95.g0_.z)) * vec4<f32>(1.0, 0.0, 1.0, 1.0))));
}

fn motor_translator_regressive_product(self_1142: Motor, other_980: Translator) -> Plane {
    var self_1143: Motor;
    var other_981: Translator;

    self_1143 = self_1142;
    other_981 = other_980;
    let _e4: Motor = self_1143;
    let _e8: Translator = other_981;
    let _e18: Motor = self_1143;
    let _e21: Motor = self_1143;
    let _e24: Motor = self_1143;
    let _e28: Translator = other_981;
    return Plane((((vec3<f32>(_e4.g0_.w) * vec3<f32>(_e8.g0_.y)) * vec3<f32>(1.0, 0.0, 0.0)) + ((vec3<f32>(_e18.g0_.z, _e21.g0_.y, _e24.g0_.y) * _e28.g0_.zzy) * vec3<f32>(-(1.0), 1.0, -(1.0)))));
}

fn motor_translator_outer_product(self_1144: Motor, other_982: Translator) -> Motor {
    var self_1145: Motor;
    var other_983: Translator;

    self_1145 = self_1144;
    other_983 = other_982;
    let _e4: Motor = self_1145;
    let _e8: Translator = other_983;
    let _e19: Motor = self_1145;
    let _e23: Translator = other_983;
    let _e35: Motor = self_1145;
    let _e38: Translator = other_983;
    let _e41: Translator = other_983;
    let _e44: Translator = other_983;
    let _e47: Translator = other_983;
    return Motor(((((vec4<f32>(_e4.g0_.z) * vec4<f32>(_e8.g0_.x)) * vec4<f32>(0.0, 0.0, 1.0, 0.0)) + ((vec4<f32>(_e19.g0_.w) * vec4<f32>(_e23.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + (_e35.g0_.xyxx * vec4<f32>(_e38.g0_.x, _e41.g0_.x, _e44.g0_.y, _e47.g0_.z))));
}

fn motor_translator_inner_product(self_1146: Motor, other_984: Translator) -> Motor {
    var self_1147: Motor;
    var other_985: Translator;

    self_1147 = self_1146;
    other_985 = other_984;
    let _e4: Motor = self_1147;
    let _e8: Translator = other_985;
    let _e11: Translator = other_985;
    let _e14: Translator = other_985;
    let _e17: Translator = other_985;
    let _e29: Motor = self_1147;
    let _e33: Translator = other_985;
    let _e36: Translator = other_985;
    let _e39: Translator = other_985;
    let _e42: Translator = other_985;
    let _e55: Motor = self_1147;
    let _e58: Translator = other_985;
    let _e61: Translator = other_985;
    let _e64: Translator = other_985;
    let _e67: Translator = other_985;
    return Motor(((((vec4<f32>(_e4.g0_.z) * vec4<f32>(_e8.g0_.y, _e11.g0_.y, _e14.g0_.x, _e17.g0_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0)) + ((vec4<f32>(_e29.g0_.w) * vec4<f32>(_e33.g0_.z, _e36.g0_.z, _e39.g0_.z, _e42.g0_.x)) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))) + (_e55.g0_.xyxx * vec4<f32>(_e58.g0_.x, _e61.g0_.x, _e64.g0_.y, _e67.g0_.z))));
}

fn motor_translator_left_contraction(self_1148: Motor, other_986: Translator) -> Translator {
    var self_1149: Motor;
    var other_987: Translator;

    self_1149 = self_1148;
    other_987 = other_986;
    let _e4: Motor = self_1149;
    let _e8: Translator = other_987;
    let _e11: Motor = self_1149;
    let _e15: Translator = other_987;
    let _e27: Motor = self_1149;
    let _e30: Motor = self_1149;
    let _e33: Motor = self_1149;
    let _e37: Translator = other_987;
    return Translator((((vec3<f32>(_e4.g0_.x) * _e8.g0_) + ((vec3<f32>(_e11.g0_.w) * vec3<f32>(_e15.g0_.z)) * vec3<f32>(-(1.0), 0.0, 0.0))) + ((vec3<f32>(_e27.g0_.z, _e30.g0_.x, _e33.g0_.x) * _e37.g0_.yxx) * vec3<f32>(-(1.0), 0.0, 0.0))));
}

fn motor_translator_right_contraction(self_1150: Motor, other_988: Translator) -> Motor {
    var self_1151: Motor;
    var other_989: Translator;

    self_1151 = self_1150;
    other_989 = other_988;
    let _e4: Motor = self_1151;
    let _e8: Translator = other_989;
    let _e11: Translator = other_989;
    let _e14: Translator = other_989;
    let _e17: Translator = other_989;
    let _e29: Motor = self_1151;
    let _e33: Translator = other_989;
    let _e36: Translator = other_989;
    let _e39: Translator = other_989;
    let _e42: Translator = other_989;
    let _e55: Motor = self_1151;
    let _e58: Translator = other_989;
    return Motor(((((vec4<f32>(_e4.g0_.z) * vec4<f32>(_e8.g0_.y, _e11.g0_.y, _e14.g0_.x, _e17.g0_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0)) + ((vec4<f32>(_e29.g0_.w) * vec4<f32>(_e33.g0_.z, _e36.g0_.z, _e39.g0_.z, _e42.g0_.x)) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))) + ((_e55.g0_.xyxx * vec4<f32>(_e58.g0_.x)) * vec4<f32>(1.0, 1.0, 0.0, 0.0))));
}

fn motor_translator_scalar_product(self_1152: Motor, other_990: Translator) -> Scalar {
    var self_1153: Motor;
    var other_991: Translator;

    self_1153 = self_1152;
    other_991 = other_990;
    let _e4: Motor = self_1153;
    let _e7: Translator = other_991;
    let _e11: Motor = self_1153;
    let _e14: Translator = other_991;
    let _e19: Motor = self_1153;
    let _e22: Translator = other_991;
    return Scalar((((_e4.g0_.x * _e7.g0_.x) - (_e11.g0_.z * _e14.g0_.y)) - (_e19.g0_.w * _e22.g0_.z)));
}

fn motor_motor_add(self_1154: Motor, other_992: Motor) -> Motor {
    var self_1155: Motor;
    var other_993: Motor;

    self_1155 = self_1154;
    other_993 = other_992;
    let _e4: Motor = self_1155;
    let _e6: Motor = other_993;
    return Motor((_e4.g0_ + _e6.g0_));
}

fn motor_motor_sub(self_1156: Motor, other_994: Motor) -> Motor {
    var self_1157: Motor;
    var other_995: Motor;

    self_1157 = self_1156;
    other_995 = other_994;
    let _e4: Motor = self_1157;
    let _e6: Motor = other_995;
    return Motor((_e4.g0_ - _e6.g0_));
}

fn motor_motor_mul(self_1158: Motor, other_996: Motor) -> Motor {
    var self_1159: Motor;
    var other_997: Motor;

    self_1159 = self_1158;
    other_997 = other_996;
    let _e4: Motor = self_1159;
    let _e6: Motor = other_997;
    return Motor((_e4.g0_ * _e6.g0_));
}

fn motor_motor_div(self_1160: Motor, other_998: Motor) -> Motor {
    var self_1161: Motor;
    var other_999: Motor;

    self_1161 = self_1160;
    other_999 = other_998;
    let _e4: Motor = self_1161;
    let _e7: Motor = self_1161;
    let _e10: Motor = self_1161;
    let _e13: Motor = self_1161;
    let _e23: Motor = other_999;
    let _e26: Motor = other_999;
    let _e29: Motor = other_999;
    let _e32: Motor = other_999;
    return Motor((((vec4<f32>(_e4.g0_.x, _e7.g0_.y, _e10.g0_.z, _e13.g0_.w) * vec4<f32>(1.0, 1.0, 1.0, 1.0)) / vec4<f32>(_e23.g0_.x, _e26.g0_.y, _e29.g0_.z, _e32.g0_.w)) * vec4<f32>(1.0, 1.0, 1.0, 1.0)));
}

fn motor_motor_geometric_product(self_1162: Motor, other_1000: Motor) -> Motor {
    var self_1163: Motor;
    var other_1001: Motor;

    self_1163 = self_1162;
    other_1001 = other_1000;
    let _e4: Motor = self_1163;
    let _e8: Motor = other_1001;
    let _e11: Motor = self_1163;
    let _e15: Motor = other_1001;
    let _e28: Motor = self_1163;
    let _e32: Motor = other_1001;
    let _e45: Motor = self_1163;
    let _e49: Motor = other_1001;
    return Motor(((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.y) * _e15.g0_.yxwz) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))) + ((vec4<f32>(_e28.g0_.z) * _e32.g0_.zwxy) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e45.g0_.w) * _e49.g0_.wzyx) * vec4<f32>(-(1.0), -(1.0), 1.0, 1.0))));
}

fn motor_motor_regressive_product(self_1164: Motor, other_1002: Motor) -> Plane {
    var self_1165: Motor;
    var other_1003: Motor;

    self_1165 = self_1164;
    other_1003 = other_1002;
    let _e4: Motor = self_1165;
    let _e8: Motor = other_1003;
    let _e11: Motor = other_1003;
    let _e14: Motor = other_1003;
    let _e25: Motor = self_1165;
    let _e29: Motor = other_1003;
    let _e32: Motor = other_1003;
    let _e35: Motor = other_1003;
    let _e47: Motor = self_1165;
    let _e50: Motor = self_1165;
    let _e53: Motor = self_1165;
    let _e57: Motor = other_1003;
    let _e60: Motor = other_1003;
    let _e63: Motor = other_1003;
    return Plane(((((vec3<f32>(_e4.g0_.z) * vec3<f32>(_e8.g0_.w, _e11.g0_.w, _e14.g0_.y)) * vec3<f32>(-(1.0), 0.0, 1.0)) + ((vec3<f32>(_e25.g0_.w) * vec3<f32>(_e29.g0_.z, _e32.g0_.y, _e35.g0_.z)) * vec3<f32>(1.0, -(1.0), 0.0))) + ((vec3<f32>(_e47.g0_.x, _e50.g0_.y, _e53.g0_.y) * vec3<f32>(_e57.g0_.x, _e60.g0_.w, _e63.g0_.z)) * vec3<f32>(0.0, 1.0, -(1.0)))));
}

fn motor_motor_outer_product(self_1166: Motor, other_1004: Motor) -> Motor {
    var self_1167: Motor;
    var other_1005: Motor;

    self_1167 = self_1166;
    other_1005 = other_1004;
    let _e4: Motor = self_1167;
    let _e8: Motor = other_1005;
    let _e11: Motor = self_1167;
    let _e13: Motor = other_1005;
    return Motor(((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((_e11.g0_ * vec4<f32>(_e13.g0_.x)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn motor_motor_inner_product(self_1168: Motor, other_1006: Motor) -> Motor {
    var self_1169: Motor;
    var other_1007: Motor;

    self_1169 = self_1168;
    other_1007 = other_1006;
    let _e4: Motor = self_1169;
    let _e8: Motor = other_1007;
    let _e11: Motor = self_1169;
    let _e15: Motor = other_1007;
    let _e27: Motor = self_1169;
    let _e31: Motor = other_1007;
    let _e43: Motor = self_1169;
    let _e46: Motor = other_1007;
    return Motor(((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.z) * _e15.g0_.zzxz) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0))) + ((vec4<f32>(_e27.g0_.w) * _e31.g0_.wwwx) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))) + ((_e43.g0_.yyxx * _e46.g0_.yxxx) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))));
}

fn motor_motor_left_contraction(self_1170: Motor, other_1008: Motor) -> Motor {
    var self_1171: Motor;
    var other_1009: Motor;

    self_1171 = self_1170;
    other_1009 = other_1008;
    let _e4: Motor = self_1171;
    let _e8: Motor = other_1009;
    let _e11: Motor = self_1171;
    let _e15: Motor = other_1009;
    let _e28: Motor = self_1171;
    let _e32: Motor = other_1009;
    let _e45: Motor = self_1171;
    let _e48: Motor = other_1009;
    return Motor(((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.z) * vec4<f32>(_e15.g0_.z)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((vec4<f32>(_e28.g0_.w) * vec4<f32>(_e32.g0_.w)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((_e45.g0_.yxxx * _e48.g0_.yxxx) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))));
}

fn motor_motor_right_contraction(self_1172: Motor, other_1010: Motor) -> Motor {
    var self_1173: Motor;
    var other_1011: Motor;

    self_1173 = self_1172;
    other_1011 = other_1010;
    let _e4: Motor = self_1173;
    let _e8: Motor = other_1011;
    let _e19: Motor = self_1173;
    let _e23: Motor = other_1011;
    let _e35: Motor = self_1173;
    let _e39: Motor = other_1011;
    let _e51: Motor = self_1173;
    let _e55: Motor = other_1011;
    return Motor((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0)) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.zzxz) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0))) + ((vec4<f32>(_e35.g0_.w) * _e39.g0_.wwwx) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))) + ((vec4<f32>(_e51.g0_.x) * vec4<f32>(_e55.g0_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))));
}

fn motor_motor_scalar_product(self_1174: Motor, other_1012: Motor) -> Scalar {
    var self_1175: Motor;
    var other_1013: Motor;

    self_1175 = self_1174;
    other_1013 = other_1012;
    let _e4: Motor = self_1175;
    let _e7: Motor = other_1013;
    let _e11: Motor = self_1175;
    let _e14: Motor = other_1013;
    let _e19: Motor = self_1175;
    let _e22: Motor = other_1013;
    let _e27: Motor = self_1175;
    let _e30: Motor = other_1013;
    return Scalar(((((_e4.g0_.x * _e7.g0_.x) - (_e11.g0_.y * _e14.g0_.y)) - (_e19.g0_.z * _e22.g0_.z)) - (_e27.g0_.w * _e30.g0_.w)));
}

fn motor_motor_dual_add(self_1176: Motor, other_1014: MotorDual) -> MultiVector {
    var self_1177: Motor;
    var other_1015: MotorDual;

    self_1177 = self_1176;
    other_1015 = other_1014;
    let _e4: Motor = self_1177;
    let _e13: MotorDual = other_1015;
    let _e23: Motor = self_1177;
    let _e32: MotorDual = other_1015;
    return MultiVector(((_e4.g0_.xyxx * vec4<f32>(1.0, 1.0, 0.0, 0.0)) + (_e13.g0_.xxwz * vec4<f32>(0.0, 0.0, 1.0, 1.0))), ((_e23.g0_.xxzw * vec4<f32>(0.0, 0.0, 1.0, 1.0)) + (_e32.g0_.yxxx * vec4<f32>(1.0, 1.0, 0.0, 0.0))));
}

fn motor_motor_dual_sub(self_1178: Motor, other_1016: MotorDual) -> MultiVector {
    var self_1179: Motor;
    var other_1017: MotorDual;

    self_1179 = self_1178;
    other_1017 = other_1016;
    let _e4: Motor = self_1179;
    let _e13: MotorDual = other_1017;
    let _e23: Motor = self_1179;
    let _e32: MotorDual = other_1017;
    return MultiVector(((_e4.g0_.xyxx * vec4<f32>(1.0, 1.0, 0.0, 0.0)) - (_e13.g0_.xxwz * vec4<f32>(0.0, 0.0, 1.0, 1.0))), ((_e23.g0_.xxzw * vec4<f32>(0.0, 0.0, 1.0, 1.0)) - (_e32.g0_.yxxx * vec4<f32>(1.0, 1.0, 0.0, 0.0))));
}

fn motor_motor_dual_geometric_product(self_1180: Motor, other_1018: MotorDual) -> MotorDual {
    var self_1181: Motor;
    var other_1019: MotorDual;

    self_1181 = self_1180;
    other_1019 = other_1018;
    let _e4: Motor = self_1181;
    let _e8: MotorDual = other_1019;
    let _e11: Motor = self_1181;
    let _e15: MotorDual = other_1019;
    let _e28: Motor = self_1181;
    let _e32: MotorDual = other_1019;
    let _e45: Motor = self_1181;
    let _e49: MotorDual = other_1019;
    return MotorDual(((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.y) * _e15.g0_.yxwz) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e28.g0_.z) * _e32.g0_.zwxy) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e45.g0_.w) * _e49.g0_.wzyx) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))));
}

fn motor_motor_dual_regressive_product(self_1182: Motor, other_1020: MotorDual) -> Motor {
    var self_1183: Motor;
    var other_1021: MotorDual;

    self_1183 = self_1182;
    other_1021 = other_1020;
    let _e4: Motor = self_1183;
    let _e8: MotorDual = other_1021;
    let _e18: Motor = self_1183;
    let _e22: MotorDual = other_1021;
    let _e33: Motor = self_1183;
    let _e37: MotorDual = other_1021;
    let _e48: Motor = self_1183;
    let _e52: MotorDual = other_1021;
    return Motor((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.yxyy) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) + ((vec4<f32>(_e18.g0_.z) * _e22.g0_.zzxz) * vec4<f32>(1.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e33.g0_.w) * _e37.g0_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e48.g0_.x) * vec4<f32>(_e52.g0_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))));
}

fn motor_motor_dual_outer_product(self_1184: Motor, other_1022: MotorDual) -> MotorDual {
    var self_1185: Motor;
    var other_1023: MotorDual;

    self_1185 = self_1184;
    other_1023 = other_1022;
    let _e4: Motor = self_1185;
    let _e8: MotorDual = other_1023;
    let _e11: Motor = self_1185;
    let _e15: MotorDual = other_1023;
    let _e27: Motor = self_1185;
    let _e31: MotorDual = other_1023;
    let _e43: Motor = self_1185;
    let _e46: MotorDual = other_1023;
    return MotorDual(((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.z) * vec4<f32>(_e15.g0_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e27.g0_.w) * vec4<f32>(_e31.g0_.w)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((_e43.g0_.yxxx * _e46.g0_.yxxx) * vec4<f32>(1.0, 0.0, 0.0, 0.0))));
}

fn motor_motor_dual_inner_product(self_1186: Motor, other_1024: MotorDual) -> MotorDual {
    var self_1187: Motor;
    var other_1025: MotorDual;

    self_1187 = self_1186;
    other_1025 = other_1024;
    let _e4: Motor = self_1187;
    let _e8: MotorDual = other_1025;
    let _e11: Motor = self_1187;
    let _e15: MotorDual = other_1025;
    let _e28: Motor = self_1187;
    let _e32: MotorDual = other_1025;
    let _e45: Motor = self_1187;
    let _e48: MotorDual = other_1025;
    return MotorDual(((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.z) * _e15.g0_.wwxy) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e28.g0_.w) * _e32.g0_.zzyx) * vec4<f32>(0.0, -(1.0), 1.0, -(1.0)))) + ((_e45.g0_.xyyy * _e48.g0_.xxwz) * vec4<f32>(0.0, -(1.0), -(1.0), 1.0))));
}

fn motor_motor_dual_left_contraction(self_1188: Motor, other_1026: MotorDual) -> MotorDual {
    var self_1189: Motor;
    var other_1027: MotorDual;

    self_1189 = self_1188;
    other_1027 = other_1026;
    let _e4: Motor = self_1189;
    let _e8: MotorDual = other_1027;
    let _e11: Motor = self_1189;
    let _e13: MotorDual = other_1027;
    return MotorDual(((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((_e11.g0_ * vec4<f32>(_e13.g0_.x)) * vec4<f32>(0.0, -(1.0), -(1.0), -(1.0)))));
}

fn motor_motor_dual_right_contraction(self_1190: Motor, other_1028: MotorDual) -> Plane {
    var self_1191: Motor;
    var other_1029: MotorDual;

    self_1191 = self_1190;
    other_1029 = other_1028;
    let _e4: Motor = self_1191;
    let _e8: MotorDual = other_1029;
    let _e11: MotorDual = other_1029;
    let _e14: MotorDual = other_1029;
    let _e25: Motor = self_1191;
    let _e29: MotorDual = other_1029;
    let _e32: MotorDual = other_1029;
    let _e35: MotorDual = other_1029;
    let _e47: Motor = self_1191;
    let _e50: Motor = self_1191;
    let _e53: Motor = self_1191;
    let _e57: MotorDual = other_1029;
    let _e60: MotorDual = other_1029;
    let _e63: MotorDual = other_1029;
    return Plane(((((vec3<f32>(_e4.g0_.z) * vec3<f32>(_e8.g0_.w, _e11.g0_.w, _e14.g0_.y)) * vec3<f32>(1.0, 0.0, -(1.0))) + ((vec3<f32>(_e25.g0_.w) * vec3<f32>(_e29.g0_.z, _e32.g0_.y, _e35.g0_.z)) * vec3<f32>(-(1.0), 1.0, 0.0))) + ((vec3<f32>(_e47.g0_.x, _e50.g0_.y, _e53.g0_.y) * vec3<f32>(_e57.g0_.x, _e60.g0_.w, _e63.g0_.z)) * vec3<f32>(0.0, -(1.0), 1.0))));
}

fn motor_squared_magnitude(self_1192: Motor) -> Scalar {
    var self_1193: Motor;

    self_1193 = self_1192;
    let _e2: Motor = self_1193;
    let _e3: Motor = self_1193;
    let _e4: Motor = motor_reversal(_e3);
    let _e5: Scalar = motor_motor_scalar_product(_e2, _e4);
    return _e5;
}

fn motor_magnitude(self_1194: Motor) -> Scalar {
    var self_1195: Motor;

    self_1195 = self_1194;
    let _e2: Motor = self_1195;
    let _e3: Scalar = motor_squared_magnitude(_e2);
    return Scalar(sqrt(_e3.g0_));
}

fn motor_scale(self_1196: Motor, other_1030: f32) -> Motor {
    var self_1197: Motor;
    var other_1031: f32;

    self_1197 = self_1196;
    other_1031 = other_1030;
    let _e4: Motor = self_1197;
    let _e5: f32 = other_1031;
    let _e7: Motor = motor_scalar_geometric_product(_e4, Scalar(_e5));
    return _e7;
}

fn motor_signum(self_1198: Motor) -> Motor {
    var self_1199: Motor;

    self_1199 = self_1198;
    let _e2: Motor = self_1199;
    let _e3: Motor = self_1199;
    let _e4: Scalar = motor_magnitude(_e3);
    let _e9: Motor = motor_scalar_geometric_product(_e2, Scalar((1.0 / _e4.g0_)));
    return _e9;
}

fn motor_inverse(self_1200: Motor) -> Motor {
    var self_1201: Motor;

    self_1201 = self_1200;
    let _e2: Motor = self_1201;
    let _e3: Motor = motor_reversal(_e2);
    let _e4: Motor = self_1201;
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

fn motor_dual_neg(self_1202: MotorDual) -> MotorDual {
    var self_1203: MotorDual;

    self_1203 = self_1202;
    let _e2: MotorDual = self_1203;
    return MotorDual((_e2.g0_ * vec4<f32>(-(1.0))));
}

fn motor_dual_automorphism(self_1204: MotorDual) -> MotorDual {
    var self_1205: MotorDual;

    self_1205 = self_1204;
    let _e2: MotorDual = self_1205;
    return MotorDual((_e2.g0_ * vec4<f32>(-(1.0))));
}

fn motor_dual_reversal(self_1206: MotorDual) -> MotorDual {
    var self_1207: MotorDual;

    self_1207 = self_1206;
    let _e2: MotorDual = self_1207;
    return MotorDual((_e2.g0_ * vec4<f32>(-(1.0), 1.0, 1.0, 1.0)));
}

fn motor_dual_conjugation(self_1208: MotorDual) -> MotorDual {
    var self_1209: MotorDual;

    self_1209 = self_1208;
    let _e2: MotorDual = self_1209;
    return MotorDual((_e2.g0_ * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))));
}

fn motor_dual_dual(self_1210: MotorDual) -> Motor {
    var self_1211: MotorDual;

    self_1211 = self_1210;
    let _e2: MotorDual = self_1211;
    return Motor(_e2.g0_);
}

fn motor_dual_scalar_geometric_product(self_1212: MotorDual, other_1032: Scalar) -> MotorDual {
    var self_1213: MotorDual;
    var other_1033: Scalar;

    self_1213 = self_1212;
    other_1033 = other_1032;
    let _e4: MotorDual = self_1213;
    let _e6: Scalar = other_1033;
    return MotorDual((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn motor_dual_scalar_regressive_product(self_1214: MotorDual, other_1034: Scalar) -> Scalar {
    var self_1215: MotorDual;
    var other_1035: Scalar;

    self_1215 = self_1214;
    other_1035 = other_1034;
    let _e4: MotorDual = self_1215;
    let _e7: Scalar = other_1035;
    return Scalar((_e4.g0_.x * _e7.g0_));
}

fn motor_dual_scalar_outer_product(self_1216: MotorDual, other_1036: Scalar) -> MotorDual {
    var self_1217: MotorDual;
    var other_1037: Scalar;

    self_1217 = self_1216;
    other_1037 = other_1036;
    let _e4: MotorDual = self_1217;
    let _e6: Scalar = other_1037;
    return MotorDual((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn motor_dual_scalar_inner_product(self_1218: MotorDual, other_1038: Scalar) -> MotorDual {
    var self_1219: MotorDual;
    var other_1039: Scalar;

    self_1219 = self_1218;
    other_1039 = other_1038;
    let _e4: MotorDual = self_1219;
    let _e6: Scalar = other_1039;
    return MotorDual((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn motor_dual_scalar_right_contraction(self_1220: MotorDual, other_1040: Scalar) -> MotorDual {
    var self_1221: MotorDual;
    var other_1041: Scalar;

    self_1221 = self_1220;
    other_1041 = other_1040;
    let _e4: MotorDual = self_1221;
    let _e6: Scalar = other_1041;
    return MotorDual((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn motor_dual_multi_vector_add(self_1222: MotorDual, other_1042: MultiVector) -> MultiVector {
    var self_1223: MotorDual;
    var other_1043: MultiVector;

    self_1223 = self_1222;
    other_1043 = other_1042;
    let _e4: MotorDual = self_1223;
    let _e13: MultiVector = other_1043;
    let _e16: MotorDual = self_1223;
    let _e25: MultiVector = other_1043;
    return MultiVector(((_e4.g0_.xxwz * vec4<f32>(0.0, 0.0, 1.0, 1.0)) + _e13.g0_), ((_e16.g0_.yxxx * vec4<f32>(1.0, 1.0, 0.0, 0.0)) + _e25.g1_));
}

fn motor_dual_multi_vector_sub(self_1224: MotorDual, other_1044: MultiVector) -> MultiVector {
    var self_1225: MotorDual;
    var other_1045: MultiVector;

    self_1225 = self_1224;
    other_1045 = other_1044;
    let _e4: MotorDual = self_1225;
    let _e13: MultiVector = other_1045;
    let _e16: MotorDual = self_1225;
    let _e25: MultiVector = other_1045;
    return MultiVector(((_e4.g0_.xxwz * vec4<f32>(0.0, 0.0, 1.0, 1.0)) - _e13.g0_), ((_e16.g0_.yxxx * vec4<f32>(1.0, 1.0, 0.0, 0.0)) - _e25.g1_));
}

fn motor_dual_multi_vector_geometric_product(self_1226: MotorDual, other_1046: MultiVector) -> MultiVector {
    var self_1227: MotorDual;
    var other_1047: MultiVector;

    self_1227 = self_1226;
    other_1047 = other_1046;
    let _e4: MotorDual = self_1227;
    let _e8: MultiVector = other_1047;
    let _e21: MotorDual = self_1227;
    let _e25: MultiVector = other_1047;
    let _e36: MotorDual = self_1227;
    let _e40: MultiVector = other_1047;
    let _e53: MotorDual = self_1227;
    let _e57: MultiVector = other_1047;
    let _e62: MotorDual = self_1227;
    let _e66: MultiVector = other_1047;
    let _e77: MotorDual = self_1227;
    let _e81: MultiVector = other_1047;
    let _e92: MotorDual = self_1227;
    let _e96: MultiVector = other_1047;
    let _e101: MotorDual = self_1227;
    let _e105: MultiVector = other_1047;
    return MultiVector((((((vec4<f32>(_e4.g0_.x) * _e8.g1_.yxwz) * vec4<f32>(-(1.0), 1.0, -(1.0), -(1.0))) + ((vec4<f32>(_e21.g0_.y) * _e25.g1_) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e36.g0_.z) * _e40.g0_.wzyx) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0))) + (vec4<f32>(_e53.g0_.w) * _e57.g0_.zwxy)), (((((vec4<f32>(_e62.g0_.x) * _e66.g0_.yxwz) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0)) + ((vec4<f32>(_e77.g0_.y) * _e81.g0_) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))) + (vec4<f32>(_e92.g0_.z) * _e96.g1_.wzyx)) + ((vec4<f32>(_e101.g0_.w) * _e105.g1_.zwxy) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))));
}

fn motor_dual_multi_vector_regressive_product(self_1228: MotorDual, other_1048: MultiVector) -> MultiVector {
    var self_1229: MotorDual;
    var other_1049: MultiVector;

    self_1229 = self_1228;
    other_1049 = other_1048;
    let _e4: MotorDual = self_1229;
    let _e8: MultiVector = other_1049;
    let _e11: MotorDual = self_1229;
    let _e15: MultiVector = other_1049;
    let _e26: MotorDual = self_1229;
    let _e30: MultiVector = other_1049;
    let _e41: MotorDual = self_1229;
    let _e44: MultiVector = other_1049;
    let _e55: MotorDual = self_1229;
    let _e59: MultiVector = other_1049;
    let _e62: MotorDual = self_1229;
    let _e65: MultiVector = other_1049;
    return MultiVector(((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.z) * _e15.g1_.zzzy) * vec4<f32>(1.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e26.g0_.w) * _e30.g1_.wwyw) * vec4<f32>(1.0, 0.0, 1.0, 0.0))) + ((_e41.g0_.yxxx * _e44.g0_.yxxx) * vec4<f32>(1.0, 0.0, 0.0, 0.0))), ((vec4<f32>(_e55.g0_.x) * _e59.g1_) + ((_e62.g0_.yxxx * _e65.g1_.yxxx) * vec4<f32>(1.0, 0.0, 0.0, 0.0))));
}

fn motor_dual_multi_vector_inner_product(self_1230: MotorDual, other_1050: MultiVector) -> MultiVector {
    var self_1231: MotorDual;
    var other_1051: MultiVector;

    self_1231 = self_1230;
    other_1051 = other_1050;
    let _e4: MotorDual = self_1231;
    let _e8: MultiVector = other_1051;
    let _e21: MotorDual = self_1231;
    let _e25: MultiVector = other_1051;
    let _e36: MotorDual = self_1231;
    let _e40: MultiVector = other_1051;
    let _e51: MotorDual = self_1231;
    let _e54: MultiVector = other_1051;
    let _e66: MotorDual = self_1231;
    let _e70: MultiVector = other_1051;
    let _e81: MotorDual = self_1231;
    let _e85: MultiVector = other_1051;
    let _e96: MotorDual = self_1231;
    let _e100: MultiVector = other_1051;
    let _e112: MotorDual = self_1231;
    let _e115: MultiVector = other_1051;
    return MultiVector((((((vec4<f32>(_e4.g0_.x) * _e8.g1_.yxwz) * vec4<f32>(-(1.0), 1.0, -(1.0), -(1.0))) + ((vec4<f32>(_e21.g0_.y) * _e25.g1_) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e36.g0_.w) * _e40.g0_.zzxy) * vec4<f32>(1.0, 0.0, 1.0, 1.0))) + ((_e51.g0_.zxzz * _e54.g0_.wxyx) * vec4<f32>(1.0, 0.0, -(1.0), 1.0))), (((((vec4<f32>(_e66.g0_.x) * _e70.g0_.yxwz) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0)) + ((vec4<f32>(_e81.g0_.z) * _e85.g1_.wwyw) * vec4<f32>(1.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e96.g0_.w) * _e100.g1_.zzzy) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))) + ((_e112.g0_.yxxx * vec4<f32>(_e115.g0_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))));
}

fn motor_dual_multi_vector_right_contraction(self_1232: MotorDual, other_1052: MultiVector) -> MultiVector {
    var self_1233: MotorDual;
    var other_1053: MultiVector;

    self_1233 = self_1232;
    other_1053 = other_1052;
    let _e4: MotorDual = self_1233;
    let _e8: MultiVector = other_1053;
    let _e21: MotorDual = self_1233;
    let _e25: MultiVector = other_1053;
    let _e36: MotorDual = self_1233;
    let _e40: MultiVector = other_1053;
    let _e51: MotorDual = self_1233;
    let _e54: MultiVector = other_1053;
    let _e66: MotorDual = self_1233;
    let _e70: MultiVector = other_1053;
    let _e81: MotorDual = self_1233;
    let _e84: MultiVector = other_1053;
    return MultiVector((((((vec4<f32>(_e4.g0_.x) * _e8.g1_.yxwz) * vec4<f32>(-(1.0), 1.0, -(1.0), -(1.0))) + ((vec4<f32>(_e21.g0_.z) * _e25.g0_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e36.g0_.w) * _e40.g0_.zzxz) * vec4<f32>(1.0, 0.0, 1.0, 0.0))) + ((_e51.g0_.yxxx * vec4<f32>(_e54.g1_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))), (((vec4<f32>(_e66.g0_.x) * _e70.g0_.yxwz) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0)) + ((_e81.g0_.yxxx * vec4<f32>(_e84.g0_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))));
}

fn motor_dual_multi_vector_scalar_product(self_1234: MotorDual, other_1054: MultiVector) -> Scalar {
    var self_1235: MotorDual;
    var other_1055: MultiVector;

    self_1235 = self_1234;
    other_1055 = other_1054;
    let _e5: MotorDual = self_1235;
    let _e8: MultiVector = other_1055;
    let _e13: MotorDual = self_1235;
    let _e16: MultiVector = other_1055;
    let _e21: MotorDual = self_1235;
    let _e24: MultiVector = other_1055;
    let _e29: MotorDual = self_1235;
    let _e32: MultiVector = other_1055;
    return Scalar(((((0.0 - (_e5.g0_.x * _e8.g1_.y)) + (_e13.g0_.y * _e16.g1_.x)) + (_e21.g0_.z * _e24.g0_.w)) + (_e29.g0_.w * _e32.g0_.z)));
}

fn motor_dual_rotor_geometric_product(self_1236: MotorDual, other_1056: Rotor) -> MotorDual {
    var self_1237: MotorDual;
    var other_1057: Rotor;

    self_1237 = self_1236;
    other_1057 = other_1056;
    let _e4: MotorDual = self_1237;
    let _e8: Rotor = other_1057;
    let _e11: Rotor = other_1057;
    let _e14: Rotor = other_1057;
    let _e17: Rotor = other_1057;
    let _e28: MotorDual = self_1237;
    let _e32: Rotor = other_1057;
    let _e35: Rotor = other_1057;
    let _e38: Rotor = other_1057;
    let _e41: Rotor = other_1057;
    let _e53: MotorDual = self_1237;
    let _e56: Rotor = other_1057;
    let _e59: Rotor = other_1057;
    let _e62: Rotor = other_1057;
    let _e65: Rotor = other_1057;
    return MotorDual(((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y, _e11.g0_.x, _e14.g0_.y, _e17.g0_.y)) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) + ((vec4<f32>(_e28.g0_.w) * vec4<f32>(_e32.g0_.y, _e35.g0_.y, _e38.g0_.y, _e41.g0_.x)) * vec4<f32>(0.0, 0.0, 1.0, 1.0))) + ((_e53.g0_.xxzz * vec4<f32>(_e56.g0_.x, _e59.g0_.y, _e62.g0_.x, _e65.g0_.y)) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))));
}

fn motor_dual_rotor_regressive_product(self_1238: MotorDual, other_1058: Rotor) -> Rotor {
    var self_1239: MotorDual;
    var other_1059: Rotor;

    self_1239 = self_1238;
    other_1059 = other_1058;
    let _e4: MotorDual = self_1239;
    let _e8: Rotor = other_1059;
    let _e11: MotorDual = self_1239;
    let _e14: MotorDual = self_1239;
    let _e18: Rotor = other_1059;
    return Rotor(((vec2<f32>(_e4.g0_.x) * _e8.g0_) + ((vec2<f32>(_e11.g0_.y, _e14.g0_.x) * _e18.g0_.yx) * vec2<f32>(1.0, 0.0))));
}

fn motor_dual_rotor_outer_product(self_1240: MotorDual, other_1060: Rotor) -> MotorDual {
    var self_1241: MotorDual;
    var other_1061: Rotor;

    self_1241 = self_1240;
    other_1061 = other_1060;
    let _e4: MotorDual = self_1241;
    let _e8: Rotor = other_1061;
    let _e11: Rotor = other_1061;
    let _e14: Rotor = other_1061;
    let _e17: Rotor = other_1061;
    let _e28: MotorDual = self_1241;
    let _e31: Rotor = other_1061;
    return MotorDual((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y, _e11.g0_.x, _e14.g0_.y, _e17.g0_.y)) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) + ((_e28.g0_.xxzw * vec4<f32>(_e31.g0_.x)) * vec4<f32>(1.0, 0.0, 1.0, 1.0))));
}

fn motor_dual_rotor_inner_product(self_1242: MotorDual, other_1062: Rotor) -> MotorDual {
    var self_1243: MotorDual;
    var other_1063: Rotor;

    self_1243 = self_1242;
    other_1063 = other_1062;
    let _e4: MotorDual = self_1243;
    let _e8: Rotor = other_1063;
    let _e19: MotorDual = self_1243;
    let _e23: Rotor = other_1063;
    let _e26: Rotor = other_1063;
    let _e29: Rotor = other_1063;
    let _e32: Rotor = other_1063;
    let _e44: MotorDual = self_1243;
    let _e47: Rotor = other_1063;
    let _e50: Rotor = other_1063;
    let _e53: Rotor = other_1063;
    let _e56: Rotor = other_1063;
    return MotorDual(((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.x)) * vec4<f32>(0.0, 1.0, 0.0, 0.0)) + ((vec4<f32>(_e19.g0_.w) * vec4<f32>(_e23.g0_.y, _e26.g0_.y, _e29.g0_.y, _e32.g0_.x)) * vec4<f32>(0.0, 0.0, 1.0, 1.0))) + ((_e44.g0_.xxzz * vec4<f32>(_e47.g0_.x, _e50.g0_.y, _e53.g0_.x, _e56.g0_.y)) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))));
}

fn motor_dual_rotor_right_contraction(self_1244: MotorDual, other_1064: Rotor) -> MotorDual {
    var self_1245: MotorDual;
    var other_1065: Rotor;

    self_1245 = self_1244;
    other_1065 = other_1064;
    let _e4: MotorDual = self_1245;
    let _e8: Rotor = other_1065;
    let _e19: MotorDual = self_1245;
    let _e22: Rotor = other_1065;
    let _e25: Rotor = other_1065;
    let _e28: Rotor = other_1065;
    let _e31: Rotor = other_1065;
    return MotorDual((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.x)) * vec4<f32>(0.0, 1.0, 0.0, 0.0)) + ((_e19.g0_.xxzw * vec4<f32>(_e22.g0_.x, _e25.g0_.y, _e28.g0_.x, _e31.g0_.x)) * vec4<f32>(1.0, -(1.0), 1.0, 1.0))));
}

fn motor_dual_point_geometric_product(self_1246: MotorDual, other_1066: Point) -> MotorDual {
    var self_1247: MotorDual;
    var other_1067: Point;

    self_1247 = self_1246;
    other_1067 = other_1066;
    let _e4: MotorDual = self_1247;
    let _e8: Point = other_1067;
    let _e11: Point = other_1067;
    let _e14: Point = other_1067;
    let _e17: Point = other_1067;
    let _e29: MotorDual = self_1247;
    let _e33: Point = other_1067;
    let _e36: Point = other_1067;
    let _e39: Point = other_1067;
    let _e42: Point = other_1067;
    let _e55: MotorDual = self_1247;
    let _e59: Point = other_1067;
    let _e62: Point = other_1067;
    let _e65: Point = other_1067;
    let _e68: Point = other_1067;
    let _e81: MotorDual = self_1247;
    let _e85: Point = other_1067;
    let _e88: Point = other_1067;
    let _e91: Point = other_1067;
    let _e94: Point = other_1067;
    return MotorDual((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.x, _e11.g0_.x, _e14.g0_.z, _e17.g0_.y)) * vec4<f32>(1.0, 0.0, -(1.0), 1.0)) + ((vec4<f32>(_e29.g0_.z) * vec4<f32>(_e33.g0_.y, _e36.g0_.z, _e39.g0_.y, _e42.g0_.x)) * vec4<f32>(1.0, 1.0, 0.0, -(1.0)))) + ((vec4<f32>(_e55.g0_.w) * vec4<f32>(_e59.g0_.z, _e62.g0_.y, _e65.g0_.x, _e68.g0_.z)) * vec4<f32>(1.0, -(1.0), 1.0, 0.0))) + ((vec4<f32>(_e81.g0_.x) * vec4<f32>(_e85.g0_.x, _e88.g0_.x, _e91.g0_.y, _e94.g0_.z)) * vec4<f32>(0.0, -(1.0), -(1.0), -(1.0)))));
}

fn motor_dual_point_regressive_product(self_1248: MotorDual, other_1068: Point) -> Motor {
    var self_1249: MotorDual;
    var other_1069: Point;

    self_1249 = self_1248;
    other_1069 = other_1068;
    let _e4: MotorDual = self_1249;
    let _e8: Point = other_1069;
    let _e19: MotorDual = self_1249;
    let _e23: Point = other_1069;
    let _e35: MotorDual = self_1249;
    let _e38: Point = other_1069;
    let _e41: Point = other_1069;
    let _e44: Point = other_1069;
    let _e47: Point = other_1069;
    return Motor(((((vec4<f32>(_e4.g0_.z) * vec4<f32>(_e8.g0_.y)) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) + ((vec4<f32>(_e19.g0_.w) * vec4<f32>(_e23.g0_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + (_e35.g0_.yxxx * vec4<f32>(_e38.g0_.x, _e41.g0_.x, _e44.g0_.y, _e47.g0_.z))));
}

fn motor_dual_point_inner_product(self_1250: MotorDual, other_1070: Point) -> Plane {
    var self_1251: MotorDual;
    var other_1071: Point;

    self_1251 = self_1250;
    other_1071 = other_1070;
    let _e6: MotorDual = self_1251;
    let _e10: Point = other_1071;
    let _e14: MotorDual = self_1251;
    let _e18: Point = other_1071;
    let _e29: MotorDual = self_1251;
    let _e33: Point = other_1071;
    let _e44: MotorDual = self_1251;
    let _e47: MotorDual = self_1251;
    let _e50: MotorDual = self_1251;
    let _e54: Point = other_1071;
    return Plane(((((vec3<f32>(0.0) - (vec3<f32>(_e6.g0_.x) * _e10.g0_)) + ((vec3<f32>(_e14.g0_.z) * _e18.g0_.zzx) * vec3<f32>(1.0, 0.0, -(1.0)))) + ((vec3<f32>(_e29.g0_.w) * _e33.g0_.yxy) * vec3<f32>(-(1.0), 1.0, 0.0))) + ((vec3<f32>(_e44.g0_.x, _e47.g0_.y, _e50.g0_.y) * _e54.g0_.xzy) * vec3<f32>(0.0, -(1.0), 1.0))));
}

fn motor_dual_point_left_contraction(self_1252: MotorDual, other_1072: Point) -> Plane {
    var self_1253: MotorDual;
    var other_1073: Point;

    self_1253 = self_1252;
    other_1073 = other_1072;
    let _e4: MotorDual = self_1253;
    let _e8: Point = other_1073;
    let _e18: MotorDual = self_1253;
    let _e22: Point = other_1073;
    let _e33: MotorDual = self_1253;
    let _e36: MotorDual = self_1253;
    let _e39: MotorDual = self_1253;
    let _e43: Point = other_1073;
    return Plane(((((vec3<f32>(_e4.g0_.z) * _e8.g0_.zzx) * vec3<f32>(1.0, 0.0, -(1.0))) + ((vec3<f32>(_e18.g0_.w) * _e22.g0_.yxy) * vec3<f32>(-(1.0), 1.0, 0.0))) + ((vec3<f32>(_e33.g0_.x, _e36.g0_.y, _e39.g0_.y) * _e43.g0_.xzy) * vec3<f32>(0.0, -(1.0), 1.0))));
}

fn motor_dual_point_right_contraction(self_1254: MotorDual, other_1074: Point) -> Plane {
    var self_1255: MotorDual;
    var other_1075: Point;

    self_1255 = self_1254;
    other_1075 = other_1074;
    let _e6: MotorDual = self_1255;
    let _e10: Point = other_1075;
    return Plane((vec3<f32>(0.0) - (vec3<f32>(_e6.g0_.x) * _e10.g0_)));
}

fn motor_dual_ideal_point_geometric_product(self_1256: MotorDual, other_1076: IdealPoint) -> MotorDual {
    var self_1257: MotorDual;
    var other_1077: IdealPoint;

    self_1257 = self_1256;
    other_1077 = other_1076;
    let _e4: MotorDual = self_1257;
    let _e8: IdealPoint = other_1077;
    let _e11: IdealPoint = other_1077;
    let _e14: IdealPoint = other_1077;
    let _e17: IdealPoint = other_1077;
    let _e29: MotorDual = self_1257;
    let _e33: IdealPoint = other_1077;
    let _e36: IdealPoint = other_1077;
    let _e39: IdealPoint = other_1077;
    let _e42: IdealPoint = other_1077;
    let _e55: MotorDual = self_1257;
    let _e58: IdealPoint = other_1077;
    let _e61: IdealPoint = other_1077;
    let _e64: IdealPoint = other_1077;
    let _e67: IdealPoint = other_1077;
    return MotorDual(((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y, _e11.g0_.y, _e14.g0_.y, _e17.g0_.x)) * vec4<f32>(0.0, 0.0, -(1.0), 1.0)) + ((vec4<f32>(_e29.g0_.w) * vec4<f32>(_e33.g0_.y, _e36.g0_.x, _e39.g0_.y, _e42.g0_.y)) * vec4<f32>(1.0, -(1.0), 0.0, 0.0))) + ((_e55.g0_.zzxx * vec4<f32>(_e58.g0_.x, _e61.g0_.y, _e64.g0_.x, _e67.g0_.y)) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))));
}

fn motor_dual_ideal_point_regressive_product(self_1258: MotorDual, other_1078: IdealPoint) -> Translator {
    var self_1259: MotorDual;
    var other_1079: IdealPoint;

    self_1259 = self_1258;
    other_1079 = other_1078;
    let _e4: MotorDual = self_1259;
    let _e8: IdealPoint = other_1079;
    let _e18: MotorDual = self_1259;
    let _e21: MotorDual = self_1259;
    let _e24: MotorDual = self_1259;
    let _e28: IdealPoint = other_1079;
    let _e31: IdealPoint = other_1079;
    let _e34: IdealPoint = other_1079;
    return Translator((((vec3<f32>(_e4.g0_.w) * vec3<f32>(_e8.g0_.y)) * vec3<f32>(1.0, 0.0, 0.0)) + (vec3<f32>(_e18.g0_.z, _e21.g0_.x, _e24.g0_.x) * vec3<f32>(_e28.g0_.x, _e31.g0_.x, _e34.g0_.y))));
}

fn motor_dual_ideal_point_inner_product(self_1260: MotorDual, other_1080: IdealPoint) -> Plane {
    var self_1261: MotorDual;
    var other_1081: IdealPoint;

    self_1261 = self_1260;
    other_1081 = other_1080;
    let _e4: MotorDual = self_1261;
    let _e8: IdealPoint = other_1081;
    let _e11: IdealPoint = other_1081;
    let _e14: IdealPoint = other_1081;
    let _e25: MotorDual = self_1261;
    let _e29: IdealPoint = other_1081;
    let _e41: MotorDual = self_1261;
    let _e44: MotorDual = self_1261;
    let _e47: MotorDual = self_1261;
    let _e51: IdealPoint = other_1081;
    let _e54: IdealPoint = other_1081;
    let _e57: IdealPoint = other_1081;
    return Plane(((((vec3<f32>(_e4.g0_.y) * vec3<f32>(_e8.g0_.y, _e11.g0_.y, _e14.g0_.x)) * vec3<f32>(0.0, -(1.0), 1.0)) + ((vec3<f32>(_e25.g0_.w) * vec3<f32>(_e29.g0_.x)) * vec3<f32>(-(1.0), 0.0, 0.0))) + ((vec3<f32>(_e41.g0_.z, _e44.g0_.x, _e47.g0_.x) * vec3<f32>(_e51.g0_.y, _e54.g0_.x, _e57.g0_.y)) * vec3<f32>(1.0, -(1.0), -(1.0)))));
}

fn motor_dual_ideal_point_left_contraction(self_1262: MotorDual, other_1082: IdealPoint) -> Plane {
    var self_1263: MotorDual;
    var other_1083: IdealPoint;

    self_1263 = self_1262;
    other_1083 = other_1082;
    let _e4: MotorDual = self_1263;
    let _e8: IdealPoint = other_1083;
    let _e19: MotorDual = self_1263;
    let _e22: MotorDual = self_1263;
    let _e25: MotorDual = self_1263;
    let _e29: IdealPoint = other_1083;
    let _e32: IdealPoint = other_1083;
    let _e35: IdealPoint = other_1083;
    return Plane((((vec3<f32>(_e4.g0_.w) * vec3<f32>(_e8.g0_.x)) * vec3<f32>(-(1.0), 0.0, 0.0)) + ((vec3<f32>(_e19.g0_.z, _e22.g0_.y, _e25.g0_.y) * vec3<f32>(_e29.g0_.y, _e32.g0_.y, _e35.g0_.x)) * vec3<f32>(1.0, -(1.0), 1.0))));
}

fn motor_dual_plane_into(self_1264: MotorDual) -> Plane {
    var self_1265: MotorDual;

    self_1265 = self_1264;
    let _e2: MotorDual = self_1265;
    let _e5: MotorDual = self_1265;
    let _e8: MotorDual = self_1265;
    return Plane(vec3<f32>(_e2.g0_.y, _e5.g0_.z, _e8.g0_.w));
}

fn motor_dual_plane_add(self_1266: MotorDual, other_1084: Plane) -> MotorDual {
    var self_1267: MotorDual;
    var other_1085: Plane;

    self_1267 = self_1266;
    other_1085 = other_1084;
    let _e4: MotorDual = self_1267;
    let _e6: Plane = other_1085;
    let _e9: Plane = other_1085;
    let _e12: Plane = other_1085;
    let _e15: Plane = other_1085;
    return MotorDual((_e4.g0_ + (vec4<f32>(_e6.g0_.x, _e9.g0_.x, _e12.g0_.y, _e15.g0_.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn motor_dual_plane_sub(self_1268: MotorDual, other_1086: Plane) -> MotorDual {
    var self_1269: MotorDual;
    var other_1087: Plane;

    self_1269 = self_1268;
    other_1087 = other_1086;
    let _e4: MotorDual = self_1269;
    let _e6: Plane = other_1087;
    let _e9: Plane = other_1087;
    let _e12: Plane = other_1087;
    let _e15: Plane = other_1087;
    return MotorDual((_e4.g0_ - (vec4<f32>(_e6.g0_.x, _e9.g0_.x, _e12.g0_.y, _e15.g0_.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn motor_dual_plane_geometric_product(self_1270: MotorDual, other_1088: Plane) -> Motor {
    var self_1271: MotorDual;
    var other_1089: Plane;

    self_1271 = self_1270;
    other_1089 = other_1088;
    let _e4: MotorDual = self_1271;
    let _e8: Plane = other_1089;
    let _e11: Plane = other_1089;
    let _e14: Plane = other_1089;
    let _e17: Plane = other_1089;
    let _e29: MotorDual = self_1271;
    let _e33: Plane = other_1089;
    let _e36: Plane = other_1089;
    let _e39: Plane = other_1089;
    let _e42: Plane = other_1089;
    let _e55: MotorDual = self_1271;
    let _e59: Plane = other_1089;
    let _e62: Plane = other_1089;
    let _e65: Plane = other_1089;
    let _e68: Plane = other_1089;
    let _e81: MotorDual = self_1271;
    let _e85: Plane = other_1089;
    let _e88: Plane = other_1089;
    let _e91: Plane = other_1089;
    let _e94: Plane = other_1089;
    return Motor((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.x, _e11.g0_.x, _e14.g0_.z, _e17.g0_.y)) * vec4<f32>(1.0, 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e29.g0_.z) * vec4<f32>(_e33.g0_.y, _e36.g0_.z, _e39.g0_.y, _e42.g0_.x)) * vec4<f32>(1.0, -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e55.g0_.w) * vec4<f32>(_e59.g0_.z, _e62.g0_.y, _e65.g0_.x, _e68.g0_.z)) * vec4<f32>(1.0, 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e81.g0_.x) * vec4<f32>(_e85.g0_.x, _e88.g0_.x, _e91.g0_.y, _e94.g0_.z)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn motor_dual_plane_regressive_product(self_1272: MotorDual, other_1090: Plane) -> Plane {
    var self_1273: MotorDual;
    var other_1091: Plane;

    self_1273 = self_1272;
    other_1091 = other_1090;
    let _e4: MotorDual = self_1273;
    let _e8: Plane = other_1091;
    return Plane((vec3<f32>(_e4.g0_.x) * _e8.g0_));
}

fn motor_dual_plane_outer_product(self_1274: MotorDual, other_1092: Plane) -> Point {
    var self_1275: MotorDual;
    var other_1093: Plane;

    self_1275 = self_1274;
    other_1093 = other_1092;
    let _e4: MotorDual = self_1275;
    let _e8: Plane = other_1093;
    let _e18: MotorDual = self_1275;
    let _e22: Plane = other_1093;
    let _e33: MotorDual = self_1275;
    let _e36: MotorDual = self_1275;
    let _e39: MotorDual = self_1275;
    let _e43: Plane = other_1093;
    return Point(((((vec3<f32>(_e4.g0_.z) * _e8.g0_.zzx) * vec3<f32>(-(1.0), 0.0, 1.0)) + ((vec3<f32>(_e18.g0_.w) * _e22.g0_.yxy) * vec3<f32>(1.0, -(1.0), 0.0))) + ((vec3<f32>(_e33.g0_.x, _e36.g0_.y, _e39.g0_.y) * _e43.g0_.xzy) * vec3<f32>(0.0, 1.0, -(1.0)))));
}

fn motor_dual_plane_inner_product(self_1276: MotorDual, other_1094: Plane) -> Motor {
    var self_1277: MotorDual;
    var other_1095: Plane;

    self_1277 = self_1276;
    other_1095 = other_1094;
    let _e4: MotorDual = self_1277;
    let _e8: Plane = other_1095;
    let _e19: MotorDual = self_1277;
    let _e23: Plane = other_1095;
    let _e35: MotorDual = self_1277;
    let _e38: Plane = other_1095;
    let _e41: Plane = other_1095;
    let _e44: Plane = other_1095;
    let _e47: Plane = other_1095;
    return Motor(((((vec4<f32>(_e4.g0_.z) * vec4<f32>(_e8.g0_.y)) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) + ((vec4<f32>(_e19.g0_.w) * vec4<f32>(_e23.g0_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + (_e35.g0_.yxxx * vec4<f32>(_e38.g0_.x, _e41.g0_.x, _e44.g0_.y, _e47.g0_.z))));
}

fn motor_dual_plane_left_contraction(self_1278: MotorDual, other_1096: Plane) -> Scalar {
    var self_1279: MotorDual;
    var other_1097: Plane;

    self_1279 = self_1278;
    other_1097 = other_1096;
    let _e4: MotorDual = self_1279;
    let _e7: Plane = other_1097;
    let _e11: MotorDual = self_1279;
    let _e14: Plane = other_1097;
    let _e19: MotorDual = self_1279;
    let _e22: Plane = other_1097;
    return Scalar((((_e4.g0_.y * _e7.g0_.x) + (_e11.g0_.z * _e14.g0_.y)) + (_e19.g0_.w * _e22.g0_.z)));
}

fn motor_dual_plane_right_contraction(self_1280: MotorDual, other_1098: Plane) -> Motor {
    var self_1281: MotorDual;
    var other_1099: Plane;

    self_1281 = self_1280;
    other_1099 = other_1098;
    let _e4: MotorDual = self_1281;
    let _e8: Plane = other_1099;
    let _e19: MotorDual = self_1281;
    let _e23: Plane = other_1099;
    let _e35: MotorDual = self_1281;
    let _e38: Plane = other_1099;
    let _e41: Plane = other_1099;
    let _e44: Plane = other_1099;
    let _e47: Plane = other_1099;
    return Motor(((((vec4<f32>(_e4.g0_.z) * vec4<f32>(_e8.g0_.y)) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) + ((vec4<f32>(_e19.g0_.w) * vec4<f32>(_e23.g0_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + (_e35.g0_.yxxx * vec4<f32>(_e38.g0_.x, _e41.g0_.x, _e44.g0_.y, _e47.g0_.z))));
}

fn motor_dual_plane_scalar_product(self_1282: MotorDual, other_1100: Plane) -> Scalar {
    var self_1283: MotorDual;
    var other_1101: Plane;

    self_1283 = self_1282;
    other_1101 = other_1100;
    let _e4: MotorDual = self_1283;
    let _e7: Plane = other_1101;
    let _e11: MotorDual = self_1283;
    let _e14: Plane = other_1101;
    let _e19: MotorDual = self_1283;
    let _e22: Plane = other_1101;
    return Scalar((((_e4.g0_.y * _e7.g0_.x) + (_e11.g0_.z * _e14.g0_.y)) + (_e19.g0_.w * _e22.g0_.z)));
}

fn motor_dual_translator_geometric_product(self_1284: MotorDual, other_1102: Translator) -> MotorDual {
    var self_1285: MotorDual;
    var other_1103: Translator;

    self_1285 = self_1284;
    other_1103 = other_1102;
    let _e4: MotorDual = self_1285;
    let _e8: Translator = other_1103;
    let _e11: Translator = other_1103;
    let _e14: Translator = other_1103;
    let _e17: Translator = other_1103;
    let _e29: MotorDual = self_1285;
    let _e33: Translator = other_1103;
    let _e36: Translator = other_1103;
    let _e39: Translator = other_1103;
    let _e42: Translator = other_1103;
    let _e54: MotorDual = self_1285;
    let _e58: Translator = other_1103;
    let _e61: Translator = other_1103;
    let _e64: Translator = other_1103;
    let _e67: Translator = other_1103;
    let _e80: MotorDual = self_1285;
    let _e84: Translator = other_1103;
    let _e87: Translator = other_1103;
    let _e90: Translator = other_1103;
    let _e93: Translator = other_1103;
    return MotorDual((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.x, _e11.g0_.x, _e14.g0_.z, _e17.g0_.y)) * vec4<f32>(0.0, 1.0, -(1.0), 1.0)) + ((vec4<f32>(_e29.g0_.z) * vec4<f32>(_e33.g0_.y, _e36.g0_.z, _e39.g0_.x, _e42.g0_.y)) * vec4<f32>(1.0, 1.0, 1.0, 0.0))) + ((vec4<f32>(_e54.g0_.w) * vec4<f32>(_e58.g0_.z, _e61.g0_.y, _e64.g0_.z, _e67.g0_.x)) * vec4<f32>(1.0, -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e80.g0_.x) * vec4<f32>(_e84.g0_.x, _e87.g0_.x, _e90.g0_.y, _e93.g0_.z)) * vec4<f32>(1.0, 0.0, -(1.0), -(1.0)))));
}

fn motor_dual_translator_regressive_product(self_1286: MotorDual, other_1104: Translator) -> Translator {
    var self_1287: MotorDual;
    var other_1105: Translator;

    self_1287 = self_1286;
    other_1105 = other_1104;
    let _e4: MotorDual = self_1287;
    let _e8: Translator = other_1105;
    let _e11: MotorDual = self_1287;
    let _e15: Translator = other_1105;
    let _e26: MotorDual = self_1287;
    let _e29: MotorDual = self_1287;
    let _e32: MotorDual = self_1287;
    let _e36: Translator = other_1105;
    return Translator((((vec3<f32>(_e4.g0_.x) * _e8.g0_) + ((vec3<f32>(_e11.g0_.w) * vec3<f32>(_e15.g0_.z)) * vec3<f32>(1.0, 0.0, 0.0))) + ((vec3<f32>(_e26.g0_.z, _e29.g0_.x, _e32.g0_.x) * _e36.g0_.yxx) * vec3<f32>(1.0, 0.0, 0.0))));
}

fn motor_dual_translator_outer_product(self_1288: MotorDual, other_1106: Translator) -> MotorDual {
    var self_1289: MotorDual;
    var other_1107: Translator;

    self_1289 = self_1288;
    other_1107 = other_1106;
    let _e4: MotorDual = self_1289;
    let _e8: Translator = other_1107;
    let _e11: Translator = other_1107;
    let _e14: Translator = other_1107;
    let _e17: Translator = other_1107;
    let _e28: MotorDual = self_1289;
    let _e32: Translator = other_1107;
    let _e35: Translator = other_1107;
    let _e38: Translator = other_1107;
    let _e41: Translator = other_1107;
    let _e53: MotorDual = self_1289;
    let _e56: Translator = other_1107;
    return MotorDual(((((vec4<f32>(_e4.g0_.z) * vec4<f32>(_e8.g0_.y, _e11.g0_.y, _e14.g0_.x, _e17.g0_.y)) * vec4<f32>(1.0, 0.0, 1.0, 0.0)) + ((vec4<f32>(_e28.g0_.w) * vec4<f32>(_e32.g0_.z, _e35.g0_.z, _e38.g0_.z, _e41.g0_.x)) * vec4<f32>(1.0, 0.0, 0.0, 1.0))) + ((_e53.g0_.xyxx * vec4<f32>(_e56.g0_.x)) * vec4<f32>(1.0, 1.0, 0.0, 0.0))));
}

fn motor_dual_translator_inner_product(self_1290: MotorDual, other_1108: Translator) -> MotorDual {
    var self_1291: MotorDual;
    var other_1109: Translator;

    self_1291 = self_1290;
    other_1109 = other_1108;
    let _e4: MotorDual = self_1291;
    let _e8: Translator = other_1109;
    let _e11: Translator = other_1109;
    let _e14: Translator = other_1109;
    let _e17: Translator = other_1109;
    let _e29: MotorDual = self_1291;
    let _e33: Translator = other_1109;
    let _e36: Translator = other_1109;
    let _e39: Translator = other_1109;
    let _e42: Translator = other_1109;
    let _e54: MotorDual = self_1291;
    let _e58: Translator = other_1109;
    let _e61: Translator = other_1109;
    let _e64: Translator = other_1109;
    let _e67: Translator = other_1109;
    let _e80: MotorDual = self_1291;
    let _e84: Translator = other_1109;
    let _e87: Translator = other_1109;
    let _e90: Translator = other_1109;
    let _e93: Translator = other_1109;
    return MotorDual((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.x, _e11.g0_.x, _e14.g0_.z, _e17.g0_.y)) * vec4<f32>(0.0, 1.0, -(1.0), 1.0)) + ((vec4<f32>(_e29.g0_.z) * vec4<f32>(_e33.g0_.z, _e36.g0_.z, _e39.g0_.x, _e42.g0_.z)) * vec4<f32>(0.0, 1.0, 1.0, 0.0))) + ((vec4<f32>(_e54.g0_.w) * vec4<f32>(_e58.g0_.y, _e61.g0_.y, _e64.g0_.y, _e67.g0_.x)) * vec4<f32>(0.0, -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e80.g0_.x) * vec4<f32>(_e84.g0_.x, _e87.g0_.x, _e90.g0_.y, _e93.g0_.z)) * vec4<f32>(1.0, 0.0, -(1.0), -(1.0)))));
}

fn motor_dual_translator_left_contraction(self_1292: MotorDual, other_1110: Translator) -> Plane {
    var self_1293: MotorDual;
    var other_1111: Translator;

    self_1293 = self_1292;
    other_1111 = other_1110;
    let _e4: MotorDual = self_1293;
    let _e8: Translator = other_1111;
    let _e19: MotorDual = self_1293;
    let _e22: MotorDual = self_1293;
    let _e25: MotorDual = self_1293;
    let _e29: Translator = other_1111;
    return Plane((((vec3<f32>(_e4.g0_.w) * vec3<f32>(_e8.g0_.y)) * vec3<f32>(-(1.0), 0.0, 0.0)) + ((vec3<f32>(_e19.g0_.z, _e22.g0_.y, _e25.g0_.y) * _e29.g0_.zzy) * vec3<f32>(1.0, -(1.0), 1.0))));
}

fn motor_dual_translator_right_contraction(self_1294: MotorDual, other_1112: Translator) -> MotorDual {
    var self_1295: MotorDual;
    var other_1113: Translator;

    self_1295 = self_1294;
    other_1113 = other_1112;
    let _e4: MotorDual = self_1295;
    let _e8: Translator = other_1113;
    let _e19: MotorDual = self_1295;
    let _e23: Translator = other_1113;
    let _e35: MotorDual = self_1295;
    let _e38: Translator = other_1113;
    let _e41: Translator = other_1113;
    let _e44: Translator = other_1113;
    let _e47: Translator = other_1113;
    return MotorDual(((((vec4<f32>(_e4.g0_.z) * vec4<f32>(_e8.g0_.x)) * vec4<f32>(0.0, 0.0, 1.0, 0.0)) + ((vec4<f32>(_e19.g0_.w) * vec4<f32>(_e23.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((_e35.g0_.xyxx * vec4<f32>(_e38.g0_.x, _e41.g0_.x, _e44.g0_.y, _e47.g0_.z)) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))));
}

fn motor_dual_motor_add(self_1296: MotorDual, other_1114: Motor) -> MultiVector {
    var self_1297: MotorDual;
    var other_1115: Motor;

    self_1297 = self_1296;
    other_1115 = other_1114;
    let _e4: MotorDual = self_1297;
    let _e13: Motor = other_1115;
    let _e23: MotorDual = self_1297;
    let _e32: Motor = other_1115;
    return MultiVector(((_e4.g0_.xxwz * vec4<f32>(0.0, 0.0, 1.0, 1.0)) + (_e13.g0_.xyxx * vec4<f32>(1.0, 1.0, 0.0, 0.0))), ((_e23.g0_.yxxx * vec4<f32>(1.0, 1.0, 0.0, 0.0)) + (_e32.g0_.xxzw * vec4<f32>(0.0, 0.0, 1.0, 1.0))));
}

fn motor_dual_motor_sub(self_1298: MotorDual, other_1116: Motor) -> MultiVector {
    var self_1299: MotorDual;
    var other_1117: Motor;

    self_1299 = self_1298;
    other_1117 = other_1116;
    let _e4: MotorDual = self_1299;
    let _e13: Motor = other_1117;
    let _e23: MotorDual = self_1299;
    let _e32: Motor = other_1117;
    return MultiVector(((_e4.g0_.xxwz * vec4<f32>(0.0, 0.0, 1.0, 1.0)) - (_e13.g0_.xyxx * vec4<f32>(1.0, 1.0, 0.0, 0.0))), ((_e23.g0_.yxxx * vec4<f32>(1.0, 1.0, 0.0, 0.0)) - (_e32.g0_.xxzw * vec4<f32>(0.0, 0.0, 1.0, 1.0))));
}

fn motor_dual_motor_geometric_product(self_1300: MotorDual, other_1118: Motor) -> MotorDual {
    var self_1301: MotorDual;
    var other_1119: Motor;

    self_1301 = self_1300;
    other_1119 = other_1118;
    let _e4: MotorDual = self_1301;
    let _e8: Motor = other_1119;
    let _e20: MotorDual = self_1301;
    let _e24: Motor = other_1119;
    let _e36: MotorDual = self_1301;
    let _e40: Motor = other_1119;
    let _e52: MotorDual = self_1301;
    let _e56: Motor = other_1119;
    return MotorDual((((((vec4<f32>(_e4.g0_.x) * _e8.g0_) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))) + ((vec4<f32>(_e20.g0_.y) * _e24.g0_.yxwz) * vec4<f32>(1.0, 1.0, -(1.0), 1.0))) + ((vec4<f32>(_e36.g0_.z) * _e40.g0_.zwxy) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e52.g0_.w) * _e56.g0_.wzyx) * vec4<f32>(1.0, -(1.0), 1.0, 1.0))));
}

fn motor_dual_motor_regressive_product(self_1302: MotorDual, other_1120: Motor) -> Motor {
    var self_1303: MotorDual;
    var other_1121: Motor;

    self_1303 = self_1302;
    other_1121 = other_1120;
    let _e4: MotorDual = self_1303;
    let _e8: Motor = other_1121;
    let _e11: MotorDual = self_1303;
    let _e15: Motor = other_1121;
    let _e27: MotorDual = self_1303;
    let _e31: Motor = other_1121;
    let _e43: MotorDual = self_1303;
    let _e46: Motor = other_1121;
    return Motor(((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.z) * vec4<f32>(_e15.g0_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e27.g0_.w) * vec4<f32>(_e31.g0_.w)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((_e43.g0_.yxxx * _e46.g0_.yxxx) * vec4<f32>(1.0, 0.0, 0.0, 0.0))));
}

fn motor_dual_motor_outer_product(self_1304: MotorDual, other_1122: Motor) -> MotorDual {
    var self_1305: MotorDual;
    var other_1123: Motor;

    self_1305 = self_1304;
    other_1123 = other_1122;
    let _e4: MotorDual = self_1305;
    let _e8: Motor = other_1123;
    let _e18: MotorDual = self_1305;
    let _e22: Motor = other_1123;
    let _e33: MotorDual = self_1305;
    let _e37: Motor = other_1123;
    let _e48: MotorDual = self_1305;
    let _e52: Motor = other_1123;
    return MotorDual((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.yxyy) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) + ((vec4<f32>(_e18.g0_.z) * _e22.g0_.zzxz) * vec4<f32>(1.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e33.g0_.w) * _e37.g0_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e48.g0_.x) * vec4<f32>(_e52.g0_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))));
}

fn motor_dual_motor_inner_product(self_1306: MotorDual, other_1124: Motor) -> MotorDual {
    var self_1307: MotorDual;
    var other_1125: Motor;

    self_1307 = self_1306;
    other_1125 = other_1124;
    let _e4: MotorDual = self_1307;
    let _e8: Motor = other_1125;
    let _e20: MotorDual = self_1307;
    let _e24: Motor = other_1125;
    let _e36: MotorDual = self_1307;
    let _e40: Motor = other_1125;
    let _e52: MotorDual = self_1307;
    let _e55: Motor = other_1125;
    return MotorDual((((((vec4<f32>(_e4.g0_.x) * _e8.g0_) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))) + ((vec4<f32>(_e20.g0_.z) * _e24.g0_.wwxy) * vec4<f32>(0.0, 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e36.g0_.w) * _e40.g0_.zzyx) * vec4<f32>(0.0, -(1.0), 1.0, 1.0))) + ((_e52.g0_.xyyy * _e55.g0_.xxwz) * vec4<f32>(0.0, 1.0, -(1.0), 1.0))));
}

fn motor_dual_motor_left_contraction(self_1308: MotorDual, other_1126: Motor) -> Plane {
    var self_1309: MotorDual;
    var other_1127: Motor;

    self_1309 = self_1308;
    other_1127 = other_1126;
    let _e4: MotorDual = self_1309;
    let _e8: Motor = other_1127;
    let _e11: Motor = other_1127;
    let _e14: Motor = other_1127;
    let _e25: MotorDual = self_1309;
    let _e29: Motor = other_1127;
    let _e32: Motor = other_1127;
    let _e35: Motor = other_1127;
    let _e47: MotorDual = self_1309;
    let _e50: MotorDual = self_1309;
    let _e53: MotorDual = self_1309;
    let _e57: Motor = other_1127;
    let _e60: Motor = other_1127;
    let _e63: Motor = other_1127;
    return Plane(((((vec3<f32>(_e4.g0_.z) * vec3<f32>(_e8.g0_.w, _e11.g0_.w, _e14.g0_.y)) * vec3<f32>(1.0, 0.0, -(1.0))) + ((vec3<f32>(_e25.g0_.w) * vec3<f32>(_e29.g0_.z, _e32.g0_.y, _e35.g0_.z)) * vec3<f32>(-(1.0), 1.0, 0.0))) + ((vec3<f32>(_e47.g0_.x, _e50.g0_.y, _e53.g0_.y) * vec3<f32>(_e57.g0_.x, _e60.g0_.w, _e63.g0_.z)) * vec3<f32>(0.0, -(1.0), 1.0))));
}

fn motor_dual_motor_right_contraction(self_1310: MotorDual, other_1128: Motor) -> MotorDual {
    var self_1311: MotorDual;
    var other_1129: Motor;

    self_1311 = self_1310;
    other_1129 = other_1128;
    let _e4: MotorDual = self_1311;
    let _e8: Motor = other_1129;
    let _e20: MotorDual = self_1311;
    let _e22: Motor = other_1129;
    return MotorDual((((vec4<f32>(_e4.g0_.x) * _e8.g0_) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))) + ((_e20.g0_ * vec4<f32>(_e22.g0_.x)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn motor_dual_motor_dual_add(self_1312: MotorDual, other_1130: MotorDual) -> MotorDual {
    var self_1313: MotorDual;
    var other_1131: MotorDual;

    self_1313 = self_1312;
    other_1131 = other_1130;
    let _e4: MotorDual = self_1313;
    let _e6: MotorDual = other_1131;
    return MotorDual((_e4.g0_ + _e6.g0_));
}

fn motor_dual_motor_dual_sub(self_1314: MotorDual, other_1132: MotorDual) -> MotorDual {
    var self_1315: MotorDual;
    var other_1133: MotorDual;

    self_1315 = self_1314;
    other_1133 = other_1132;
    let _e4: MotorDual = self_1315;
    let _e6: MotorDual = other_1133;
    return MotorDual((_e4.g0_ - _e6.g0_));
}

fn motor_dual_motor_dual_mul(self_1316: MotorDual, other_1134: MotorDual) -> MotorDual {
    var self_1317: MotorDual;
    var other_1135: MotorDual;

    self_1317 = self_1316;
    other_1135 = other_1134;
    let _e4: MotorDual = self_1317;
    let _e6: MotorDual = other_1135;
    return MotorDual((_e4.g0_ * _e6.g0_));
}

fn motor_dual_motor_dual_div(self_1318: MotorDual, other_1136: MotorDual) -> MotorDual {
    var self_1319: MotorDual;
    var other_1137: MotorDual;

    self_1319 = self_1318;
    other_1137 = other_1136;
    let _e4: MotorDual = self_1319;
    let _e7: MotorDual = self_1319;
    let _e10: MotorDual = self_1319;
    let _e13: MotorDual = self_1319;
    let _e23: MotorDual = other_1137;
    let _e26: MotorDual = other_1137;
    let _e29: MotorDual = other_1137;
    let _e32: MotorDual = other_1137;
    return MotorDual((((vec4<f32>(_e4.g0_.x, _e7.g0_.y, _e10.g0_.z, _e13.g0_.w) * vec4<f32>(1.0, 1.0, 1.0, 1.0)) / vec4<f32>(_e23.g0_.x, _e26.g0_.y, _e29.g0_.z, _e32.g0_.w)) * vec4<f32>(1.0, 1.0, 1.0, 1.0)));
}

fn motor_dual_motor_dual_geometric_product(self_1320: MotorDual, other_1138: MotorDual) -> Motor {
    var self_1321: MotorDual;
    var other_1139: MotorDual;

    self_1321 = self_1320;
    other_1139 = other_1138;
    let _e4: MotorDual = self_1321;
    let _e8: MotorDual = other_1139;
    let _e18: MotorDual = self_1321;
    let _e22: MotorDual = other_1139;
    let _e34: MotorDual = self_1321;
    let _e38: MotorDual = other_1139;
    let _e50: MotorDual = self_1321;
    let _e54: MotorDual = other_1139;
    return Motor((((((vec4<f32>(_e4.g0_.x) * _e8.g0_) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0)) + ((vec4<f32>(_e18.g0_.y) * _e22.g0_.yxwz) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e34.g0_.z) * _e38.g0_.zwxy) * vec4<f32>(1.0, -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e50.g0_.w) * _e54.g0_.wzyx) * vec4<f32>(1.0, 1.0, -(1.0), 1.0))));
}

fn motor_dual_motor_dual_regressive_product(self_1322: MotorDual, other_1140: MotorDual) -> MotorDual {
    var self_1323: MotorDual;
    var other_1141: MotorDual;

    self_1323 = self_1322;
    other_1141 = other_1140;
    let _e4: MotorDual = self_1323;
    let _e8: MotorDual = other_1141;
    let _e11: MotorDual = self_1323;
    let _e13: MotorDual = other_1141;
    return MotorDual(((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((_e11.g0_ * vec4<f32>(_e13.g0_.x)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn motor_dual_motor_dual_outer_product(self_1324: MotorDual, other_1142: MotorDual) -> Point {
    var self_1325: MotorDual;
    var other_1143: MotorDual;

    self_1325 = self_1324;
    other_1143 = other_1142;
    let _e4: MotorDual = self_1325;
    let _e8: MotorDual = other_1143;
    let _e11: MotorDual = other_1143;
    let _e14: MotorDual = other_1143;
    let _e25: MotorDual = self_1325;
    let _e29: MotorDual = other_1143;
    let _e32: MotorDual = other_1143;
    let _e35: MotorDual = other_1143;
    let _e47: MotorDual = self_1325;
    let _e50: MotorDual = self_1325;
    let _e53: MotorDual = self_1325;
    let _e57: MotorDual = other_1143;
    let _e60: MotorDual = other_1143;
    let _e63: MotorDual = other_1143;
    return Point(((((vec3<f32>(_e4.g0_.z) * vec3<f32>(_e8.g0_.w, _e11.g0_.w, _e14.g0_.y)) * vec3<f32>(-(1.0), 0.0, 1.0)) + ((vec3<f32>(_e25.g0_.w) * vec3<f32>(_e29.g0_.z, _e32.g0_.y, _e35.g0_.z)) * vec3<f32>(1.0, -(1.0), 0.0))) + ((vec3<f32>(_e47.g0_.x, _e50.g0_.y, _e53.g0_.y) * vec3<f32>(_e57.g0_.x, _e60.g0_.w, _e63.g0_.z)) * vec3<f32>(0.0, 1.0, -(1.0)))));
}

fn motor_dual_motor_dual_inner_product(self_1326: MotorDual, other_1144: MotorDual) -> Motor {
    var self_1327: MotorDual;
    var other_1145: MotorDual;

    self_1327 = self_1326;
    other_1145 = other_1144;
    let _e4: MotorDual = self_1327;
    let _e8: MotorDual = other_1145;
    let _e18: MotorDual = self_1327;
    let _e22: MotorDual = other_1145;
    let _e33: MotorDual = self_1327;
    let _e37: MotorDual = other_1145;
    let _e48: MotorDual = self_1327;
    let _e51: MotorDual = other_1145;
    return Motor((((((vec4<f32>(_e4.g0_.x) * _e8.g0_) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0)) + ((vec4<f32>(_e18.g0_.z) * _e22.g0_.zzxz) * vec4<f32>(1.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e33.g0_.w) * _e37.g0_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, 1.0))) + ((_e48.g0_.yyxx * _e51.g0_.yxxx) * vec4<f32>(1.0, 1.0, 0.0, 0.0))));
}

fn motor_dual_motor_dual_left_contraction(self_1328: MotorDual, other_1146: MotorDual) -> Motor {
    var self_1329: MotorDual;
    var other_1147: MotorDual;

    self_1329 = self_1328;
    other_1147 = other_1146;
    let _e4: MotorDual = self_1329;
    let _e8: MotorDual = other_1147;
    let _e18: MotorDual = self_1329;
    let _e22: MotorDual = other_1147;
    let _e33: MotorDual = self_1329;
    let _e37: MotorDual = other_1147;
    let _e48: MotorDual = self_1329;
    let _e52: MotorDual = other_1147;
    return Motor((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.yxyy) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) + ((vec4<f32>(_e18.g0_.z) * _e22.g0_.zzxz) * vec4<f32>(1.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e33.g0_.w) * _e37.g0_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e48.g0_.x) * vec4<f32>(_e52.g0_.x)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))));
}

fn motor_dual_motor_dual_right_contraction(self_1330: MotorDual, other_1148: MotorDual) -> Motor {
    var self_1331: MotorDual;
    var other_1149: MotorDual;

    self_1331 = self_1330;
    other_1149 = other_1148;
    let _e4: MotorDual = self_1331;
    let _e8: MotorDual = other_1149;
    let _e18: MotorDual = self_1331;
    let _e22: MotorDual = other_1149;
    let _e34: MotorDual = self_1331;
    let _e38: MotorDual = other_1149;
    let _e50: MotorDual = self_1331;
    let _e53: MotorDual = other_1149;
    return Motor((((((vec4<f32>(_e4.g0_.x) * _e8.g0_) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0)) + ((vec4<f32>(_e18.g0_.z) * vec4<f32>(_e22.g0_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e34.g0_.w) * vec4<f32>(_e38.g0_.w)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((_e50.g0_.yxxx * _e53.g0_.yxxx) * vec4<f32>(1.0, 0.0, 0.0, 0.0))));
}

fn motor_dual_motor_dual_scalar_product(self_1332: MotorDual, other_1150: MotorDual) -> Scalar {
    var self_1333: MotorDual;
    var other_1151: MotorDual;

    self_1333 = self_1332;
    other_1151 = other_1150;
    let _e5: MotorDual = self_1333;
    let _e8: MotorDual = other_1151;
    let _e13: MotorDual = self_1333;
    let _e16: MotorDual = other_1151;
    let _e21: MotorDual = self_1333;
    let _e24: MotorDual = other_1151;
    let _e29: MotorDual = self_1333;
    let _e32: MotorDual = other_1151;
    return Scalar(((((0.0 - (_e5.g0_.x * _e8.g0_.x)) + (_e13.g0_.y * _e16.g0_.y)) + (_e21.g0_.z * _e24.g0_.z)) + (_e29.g0_.w * _e32.g0_.w)));
}

fn motor_dual_squared_magnitude(self_1334: MotorDual) -> Scalar {
    var self_1335: MotorDual;

    self_1335 = self_1334;
    let _e2: MotorDual = self_1335;
    let _e3: MotorDual = self_1335;
    let _e4: MotorDual = motor_dual_reversal(_e3);
    let _e5: Scalar = motor_dual_motor_dual_scalar_product(_e2, _e4);
    return _e5;
}

fn motor_dual_magnitude(self_1336: MotorDual) -> Scalar {
    var self_1337: MotorDual;

    self_1337 = self_1336;
    let _e2: MotorDual = self_1337;
    let _e3: Scalar = motor_dual_squared_magnitude(_e2);
    return Scalar(sqrt(_e3.g0_));
}

fn motor_dual_scale(self_1338: MotorDual, other_1152: f32) -> MotorDual {
    var self_1339: MotorDual;
    var other_1153: f32;

    self_1339 = self_1338;
    other_1153 = other_1152;
    let _e4: MotorDual = self_1339;
    let _e5: f32 = other_1153;
    let _e7: MotorDual = motor_dual_scalar_geometric_product(_e4, Scalar(_e5));
    return _e7;
}

fn motor_dual_signum(self_1340: MotorDual) -> MotorDual {
    var self_1341: MotorDual;

    self_1341 = self_1340;
    let _e2: MotorDual = self_1341;
    let _e3: MotorDual = self_1341;
    let _e4: Scalar = motor_dual_magnitude(_e3);
    let _e9: MotorDual = motor_dual_scalar_geometric_product(_e2, Scalar((1.0 / _e4.g0_)));
    return _e9;
}

fn motor_dual_inverse(self_1342: MotorDual) -> MotorDual {
    var self_1343: MotorDual;

    self_1343 = self_1342;
    let _e2: MotorDual = self_1343;
    let _e3: MotorDual = motor_dual_reversal(_e2);
    let _e4: MotorDual = self_1343;
    let _e5: Scalar = motor_dual_squared_magnitude(_e4);
    let _e10: MotorDual = motor_dual_scalar_geometric_product(_e3, Scalar((1.0 / _e5.g0_)));
    return _e10;
}

fn ideal_point_ideal_point_geometric_quotient(self_1344: IdealPoint, other_1154: IdealPoint) -> Rotor {
    var self_1345: IdealPoint;
    var other_1155: IdealPoint;

    self_1345 = self_1344;
    other_1155 = other_1154;
    let _e4: IdealPoint = self_1345;
    let _e5: IdealPoint = other_1155;
    let _e6: IdealPoint = ideal_point_inverse(_e5);
    let _e7: Rotor = ideal_point_ideal_point_geometric_product(_e4, _e6);
    return _e7;
}

fn ideal_point_ideal_point_transformation(self_1346: IdealPoint, other_1156: IdealPoint) -> IdealPoint {
    var self_1347: IdealPoint;
    var other_1157: IdealPoint;

    self_1347 = self_1346;
    other_1157 = other_1156;
    let _e4: IdealPoint = self_1347;
    let _e5: IdealPoint = other_1157;
    let _e6: Rotor = ideal_point_ideal_point_geometric_product(_e4, _e5);
    let _e7: IdealPoint = self_1347;
    let _e8: IdealPoint = ideal_point_reversal(_e7);
    let _e9: IdealPoint = rotor_ideal_point_geometric_product(_e6, _e8);
    return _e9;
}

fn ideal_point_motor_geometric_quotient(self_1348: IdealPoint, other_1158: Motor) -> Motor {
    var self_1349: IdealPoint;
    var other_1159: Motor;

    self_1349 = self_1348;
    other_1159 = other_1158;
    let _e4: IdealPoint = self_1349;
    let _e5: Motor = other_1159;
    let _e6: Motor = motor_inverse(_e5);
    let _e7: Motor = ideal_point_motor_geometric_product(_e4, _e6);
    return _e7;
}

fn ideal_point_motor_transformation(self_1350: IdealPoint, other_1160: Motor) -> Motor {
    var self_1351: IdealPoint;
    var other_1161: Motor;

    self_1351 = self_1350;
    other_1161 = other_1160;
    let _e4: IdealPoint = self_1351;
    let _e5: Motor = other_1161;
    let _e6: Motor = ideal_point_motor_geometric_product(_e4, _e5);
    let _e7: IdealPoint = self_1351;
    let _e8: IdealPoint = ideal_point_reversal(_e7);
    let _e9: Motor = motor_ideal_point_geometric_product(_e6, _e8);
    return _e9;
}

fn ideal_point_motor_dual_geometric_quotient(self_1352: IdealPoint, other_1162: MotorDual) -> MotorDual {
    var self_1353: IdealPoint;
    var other_1163: MotorDual;

    self_1353 = self_1352;
    other_1163 = other_1162;
    let _e4: IdealPoint = self_1353;
    let _e5: MotorDual = other_1163;
    let _e6: MotorDual = motor_dual_inverse(_e5);
    let _e7: MotorDual = ideal_point_motor_dual_geometric_product(_e4, _e6);
    return _e7;
}

fn ideal_point_motor_dual_transformation(self_1354: IdealPoint, other_1164: MotorDual) -> MotorDual {
    var self_1355: IdealPoint;
    var other_1165: MotorDual;

    self_1355 = self_1354;
    other_1165 = other_1164;
    let _e4: IdealPoint = self_1355;
    let _e5: MotorDual = other_1165;
    let _e6: MotorDual = ideal_point_motor_dual_geometric_product(_e4, _e5);
    let _e7: IdealPoint = self_1355;
    let _e8: IdealPoint = ideal_point_reversal(_e7);
    let _e9: MotorDual = motor_dual_ideal_point_geometric_product(_e6, _e8);
    return _e9;
}

fn ideal_point_multi_vector_geometric_quotient(self_1356: IdealPoint, other_1166: MultiVector) -> MultiVector {
    var self_1357: IdealPoint;
    var other_1167: MultiVector;

    self_1357 = self_1356;
    other_1167 = other_1166;
    let _e4: IdealPoint = self_1357;
    let _e5: MultiVector = other_1167;
    let _e6: MultiVector = multi_vector_inverse(_e5);
    let _e7: MultiVector = ideal_point_multi_vector_geometric_product(_e4, _e6);
    return _e7;
}

fn ideal_point_multi_vector_transformation(self_1358: IdealPoint, other_1168: MultiVector) -> MultiVector {
    var self_1359: IdealPoint;
    var other_1169: MultiVector;

    self_1359 = self_1358;
    other_1169 = other_1168;
    let _e4: IdealPoint = self_1359;
    let _e5: MultiVector = other_1169;
    let _e6: MultiVector = ideal_point_multi_vector_geometric_product(_e4, _e5);
    let _e7: IdealPoint = self_1359;
    let _e8: IdealPoint = ideal_point_reversal(_e7);
    let _e9: MultiVector = multi_vector_ideal_point_geometric_product(_e6, _e8);
    return _e9;
}

fn ideal_point_plane_geometric_quotient(self_1360: IdealPoint, other_1170: Plane) -> MotorDual {
    var self_1361: IdealPoint;
    var other_1171: Plane;

    self_1361 = self_1360;
    other_1171 = other_1170;
    let _e4: IdealPoint = self_1361;
    let _e5: Plane = other_1171;
    let _e6: Plane = plane_inverse(_e5);
    let _e7: MotorDual = ideal_point_plane_geometric_product(_e4, _e6);
    return _e7;
}

fn ideal_point_plane_transformation(self_1362: IdealPoint, other_1172: Plane) -> Plane {
    var self_1363: IdealPoint;
    var other_1173: Plane;

    self_1363 = self_1362;
    other_1173 = other_1172;
    let _e4: IdealPoint = self_1363;
    let _e5: Plane = other_1173;
    let _e6: MotorDual = ideal_point_plane_geometric_product(_e4, _e5);
    let _e7: IdealPoint = self_1363;
    let _e8: IdealPoint = ideal_point_reversal(_e7);
    let _e9: MotorDual = motor_dual_ideal_point_geometric_product(_e6, _e8);
    let _e10: Plane = motor_dual_plane_into(_e9);
    return _e10;
}

fn ideal_point_point_geometric_quotient(self_1364: IdealPoint, other_1174: Point) -> Motor {
    var self_1365: IdealPoint;
    var other_1175: Point;

    self_1365 = self_1364;
    other_1175 = other_1174;
    let _e4: IdealPoint = self_1365;
    let _e5: Point = other_1175;
    let _e6: Point = point_inverse(_e5);
    let _e7: Motor = ideal_point_point_geometric_product(_e4, _e6);
    return _e7;
}

fn ideal_point_point_transformation(self_1366: IdealPoint, other_1176: Point) -> Point {
    var self_1367: IdealPoint;
    var other_1177: Point;

    self_1367 = self_1366;
    other_1177 = other_1176;
    let _e4: IdealPoint = self_1367;
    let _e5: Point = other_1177;
    let _e6: Motor = ideal_point_point_geometric_product(_e4, _e5);
    let _e7: IdealPoint = self_1367;
    let _e8: IdealPoint = ideal_point_reversal(_e7);
    let _e9: Motor = motor_ideal_point_geometric_product(_e6, _e8);
    let _e10: Point = motor_point_into(_e9);
    return _e10;
}

fn ideal_point_rotor_geometric_quotient(self_1368: IdealPoint, other_1178: Rotor) -> IdealPoint {
    var self_1369: IdealPoint;
    var other_1179: Rotor;

    self_1369 = self_1368;
    other_1179 = other_1178;
    let _e4: IdealPoint = self_1369;
    let _e5: Rotor = other_1179;
    let _e6: Rotor = rotor_inverse(_e5);
    let _e7: IdealPoint = ideal_point_rotor_geometric_product(_e4, _e6);
    return _e7;
}

fn ideal_point_rotor_transformation(self_1370: IdealPoint, other_1180: Rotor) -> Rotor {
    var self_1371: IdealPoint;
    var other_1181: Rotor;

    self_1371 = self_1370;
    other_1181 = other_1180;
    let _e4: IdealPoint = self_1371;
    let _e5: Rotor = other_1181;
    let _e6: IdealPoint = ideal_point_rotor_geometric_product(_e4, _e5);
    let _e7: IdealPoint = self_1371;
    let _e8: IdealPoint = ideal_point_reversal(_e7);
    let _e9: Rotor = ideal_point_ideal_point_geometric_product(_e6, _e8);
    return _e9;
}

fn ideal_point_scalar_geometric_quotient(self_1372: IdealPoint, other_1182: Scalar) -> IdealPoint {
    var self_1373: IdealPoint;
    var other_1183: Scalar;

    self_1373 = self_1372;
    other_1183 = other_1182;
    let _e4: IdealPoint = self_1373;
    let _e5: Scalar = other_1183;
    let _e6: Scalar = scalar_inverse(_e5);
    let _e7: IdealPoint = ideal_point_scalar_geometric_product(_e4, _e6);
    return _e7;
}

fn ideal_point_scalar_transformation(self_1374: IdealPoint, other_1184: Scalar) -> Scalar {
    var self_1375: IdealPoint;
    var other_1185: Scalar;

    self_1375 = self_1374;
    other_1185 = other_1184;
    let _e4: IdealPoint = self_1375;
    let _e5: Scalar = other_1185;
    let _e6: IdealPoint = ideal_point_scalar_geometric_product(_e4, _e5);
    let _e7: IdealPoint = self_1375;
    let _e8: IdealPoint = ideal_point_reversal(_e7);
    let _e9: Rotor = ideal_point_ideal_point_geometric_product(_e6, _e8);
    let _e10: Scalar = rotor_scalar_into(_e9);
    return _e10;
}

fn ideal_point_translator_geometric_quotient(self_1376: IdealPoint, other_1186: Translator) -> Motor {
    var self_1377: IdealPoint;
    var other_1187: Translator;

    self_1377 = self_1376;
    other_1187 = other_1186;
    let _e4: IdealPoint = self_1377;
    let _e5: Translator = other_1187;
    let _e6: Translator = translator_inverse(_e5);
    let _e7: Motor = ideal_point_translator_geometric_product(_e4, _e6);
    return _e7;
}

fn ideal_point_translator_transformation(self_1378: IdealPoint, other_1188: Translator) -> Translator {
    var self_1379: IdealPoint;
    var other_1189: Translator;

    self_1379 = self_1378;
    other_1189 = other_1188;
    let _e4: IdealPoint = self_1379;
    let _e5: Translator = other_1189;
    let _e6: Motor = ideal_point_translator_geometric_product(_e4, _e5);
    let _e7: IdealPoint = self_1379;
    let _e8: IdealPoint = ideal_point_reversal(_e7);
    let _e9: Motor = motor_ideal_point_geometric_product(_e6, _e8);
    let _e10: Translator = motor_translator_into(_e9);
    return _e10;
}

fn motor_ideal_point_geometric_quotient(self_1380: Motor, other_1190: IdealPoint) -> Motor {
    var self_1381: Motor;
    var other_1191: IdealPoint;

    self_1381 = self_1380;
    other_1191 = other_1190;
    let _e4: Motor = self_1381;
    let _e5: IdealPoint = other_1191;
    let _e6: IdealPoint = ideal_point_inverse(_e5);
    let _e7: Motor = motor_ideal_point_geometric_product(_e4, _e6);
    return _e7;
}

fn motor_ideal_point_transformation(self_1382: Motor, other_1192: IdealPoint) -> IdealPoint {
    var self_1383: Motor;
    var other_1193: IdealPoint;

    self_1383 = self_1382;
    other_1193 = other_1192;
    let _e4: Motor = self_1383;
    let _e5: IdealPoint = other_1193;
    let _e6: Motor = motor_ideal_point_geometric_product(_e4, _e5);
    let _e7: Motor = self_1383;
    let _e8: Motor = motor_reversal(_e7);
    let _e9: Motor = motor_motor_geometric_product(_e6, _e8);
    let _e10: IdealPoint = motor_ideal_point_into(_e9);
    return _e10;
}

fn motor_powi(self_1384: Motor, exponent: i32) -> Motor {
    var self_1385: Motor;
    var exponent_1: i32;
    var local: Motor;
    var x: Motor;
    var y: Motor;
    var n: i32;

    self_1385 = self_1384;
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
        let _e11: Motor = self_1385;
        let _e12: Motor = motor_inverse(_e11);
        local = _e12;
    } else {
        let _e14: Motor = self_1385;
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

fn motor_motor_geometric_quotient(self_1386: Motor, other_1194: Motor) -> Motor {
    var self_1387: Motor;
    var other_1195: Motor;

    self_1387 = self_1386;
    other_1195 = other_1194;
    let _e4: Motor = self_1387;
    let _e5: Motor = other_1195;
    let _e6: Motor = motor_inverse(_e5);
    let _e7: Motor = motor_motor_geometric_product(_e4, _e6);
    return _e7;
}

fn motor_motor_transformation(self_1388: Motor, other_1196: Motor) -> Motor {
    var self_1389: Motor;
    var other_1197: Motor;

    self_1389 = self_1388;
    other_1197 = other_1196;
    let _e4: Motor = self_1389;
    let _e5: Motor = other_1197;
    let _e6: Motor = motor_motor_geometric_product(_e4, _e5);
    let _e7: Motor = self_1389;
    let _e8: Motor = motor_reversal(_e7);
    let _e9: Motor = motor_motor_geometric_product(_e6, _e8);
    return _e9;
}

fn motor_motor_dual_geometric_quotient(self_1390: Motor, other_1198: MotorDual) -> MotorDual {
    var self_1391: Motor;
    var other_1199: MotorDual;

    self_1391 = self_1390;
    other_1199 = other_1198;
    let _e4: Motor = self_1391;
    let _e5: MotorDual = other_1199;
    let _e6: MotorDual = motor_dual_inverse(_e5);
    let _e7: MotorDual = motor_motor_dual_geometric_product(_e4, _e6);
    return _e7;
}

fn motor_motor_dual_transformation(self_1392: Motor, other_1200: MotorDual) -> MotorDual {
    var self_1393: Motor;
    var other_1201: MotorDual;

    self_1393 = self_1392;
    other_1201 = other_1200;
    let _e4: Motor = self_1393;
    let _e5: MotorDual = other_1201;
    let _e6: MotorDual = motor_motor_dual_geometric_product(_e4, _e5);
    let _e7: Motor = self_1393;
    let _e8: Motor = motor_reversal(_e7);
    let _e9: MotorDual = motor_dual_motor_geometric_product(_e6, _e8);
    return _e9;
}

fn motor_multi_vector_geometric_quotient(self_1394: Motor, other_1202: MultiVector) -> MultiVector {
    var self_1395: Motor;
    var other_1203: MultiVector;

    self_1395 = self_1394;
    other_1203 = other_1202;
    let _e4: Motor = self_1395;
    let _e5: MultiVector = other_1203;
    let _e6: MultiVector = multi_vector_inverse(_e5);
    let _e7: MultiVector = motor_multi_vector_geometric_product(_e4, _e6);
    return _e7;
}

fn motor_multi_vector_transformation(self_1396: Motor, other_1204: MultiVector) -> MultiVector {
    var self_1397: Motor;
    var other_1205: MultiVector;

    self_1397 = self_1396;
    other_1205 = other_1204;
    let _e4: Motor = self_1397;
    let _e5: MultiVector = other_1205;
    let _e6: MultiVector = motor_multi_vector_geometric_product(_e4, _e5);
    let _e7: Motor = self_1397;
    let _e8: Motor = motor_reversal(_e7);
    let _e9: MultiVector = multi_vector_motor_geometric_product(_e6, _e8);
    return _e9;
}

fn motor_plane_geometric_quotient(self_1398: Motor, other_1206: Plane) -> MotorDual {
    var self_1399: Motor;
    var other_1207: Plane;

    self_1399 = self_1398;
    other_1207 = other_1206;
    let _e4: Motor = self_1399;
    let _e5: Plane = other_1207;
    let _e6: Plane = plane_inverse(_e5);
    let _e7: MotorDual = motor_plane_geometric_product(_e4, _e6);
    return _e7;
}

fn motor_plane_transformation(self_1400: Motor, other_1208: Plane) -> Plane {
    var self_1401: Motor;
    var other_1209: Plane;

    self_1401 = self_1400;
    other_1209 = other_1208;
    let _e4: Motor = self_1401;
    let _e5: Plane = other_1209;
    let _e6: MotorDual = motor_plane_geometric_product(_e4, _e5);
    let _e7: Motor = self_1401;
    let _e8: Motor = motor_reversal(_e7);
    let _e9: MotorDual = motor_dual_motor_geometric_product(_e6, _e8);
    let _e10: Plane = motor_dual_plane_into(_e9);
    return _e10;
}

fn motor_point_geometric_quotient(self_1402: Motor, other_1210: Point) -> Motor {
    var self_1403: Motor;
    var other_1211: Point;

    self_1403 = self_1402;
    other_1211 = other_1210;
    let _e4: Motor = self_1403;
    let _e5: Point = other_1211;
    let _e6: Point = point_inverse(_e5);
    let _e7: Motor = motor_point_geometric_product(_e4, _e6);
    return _e7;
}

fn motor_point_transformation(self_1404: Motor, other_1212: Point) -> Point {
    var self_1405: Motor;
    var other_1213: Point;

    self_1405 = self_1404;
    other_1213 = other_1212;
    let _e4: Motor = self_1405;
    let _e5: Point = other_1213;
    let _e6: Motor = motor_point_geometric_product(_e4, _e5);
    let _e7: Motor = self_1405;
    let _e8: Motor = motor_reversal(_e7);
    let _e9: Motor = motor_motor_geometric_product(_e6, _e8);
    let _e10: Point = motor_point_into(_e9);
    return _e10;
}

fn motor_rotor_geometric_quotient(self_1406: Motor, other_1214: Rotor) -> Motor {
    var self_1407: Motor;
    var other_1215: Rotor;

    self_1407 = self_1406;
    other_1215 = other_1214;
    let _e4: Motor = self_1407;
    let _e5: Rotor = other_1215;
    let _e6: Rotor = rotor_inverse(_e5);
    let _e7: Motor = motor_rotor_geometric_product(_e4, _e6);
    return _e7;
}

fn motor_rotor_transformation(self_1408: Motor, other_1216: Rotor) -> Rotor {
    var self_1409: Motor;
    var other_1217: Rotor;

    self_1409 = self_1408;
    other_1217 = other_1216;
    let _e4: Motor = self_1409;
    let _e5: Rotor = other_1217;
    let _e6: Motor = motor_rotor_geometric_product(_e4, _e5);
    let _e7: Motor = self_1409;
    let _e8: Motor = motor_reversal(_e7);
    let _e9: Motor = motor_motor_geometric_product(_e6, _e8);
    let _e10: Rotor = motor_rotor_into(_e9);
    return _e10;
}

fn motor_scalar_geometric_quotient(self_1410: Motor, other_1218: Scalar) -> Motor {
    var self_1411: Motor;
    var other_1219: Scalar;

    self_1411 = self_1410;
    other_1219 = other_1218;
    let _e4: Motor = self_1411;
    let _e5: Scalar = other_1219;
    let _e6: Scalar = scalar_inverse(_e5);
    let _e7: Motor = motor_scalar_geometric_product(_e4, _e6);
    return _e7;
}

fn motor_scalar_transformation(self_1412: Motor, other_1220: Scalar) -> Scalar {
    var self_1413: Motor;
    var other_1221: Scalar;

    self_1413 = self_1412;
    other_1221 = other_1220;
    let _e4: Motor = self_1413;
    let _e5: Scalar = other_1221;
    let _e6: Motor = motor_scalar_geometric_product(_e4, _e5);
    let _e7: Motor = self_1413;
    let _e8: Motor = motor_reversal(_e7);
    let _e9: Motor = motor_motor_geometric_product(_e6, _e8);
    let _e10: Scalar = motor_scalar_into(_e9);
    return _e10;
}

fn motor_translator_geometric_quotient(self_1414: Motor, other_1222: Translator) -> Motor {
    var self_1415: Motor;
    var other_1223: Translator;

    self_1415 = self_1414;
    other_1223 = other_1222;
    let _e4: Motor = self_1415;
    let _e5: Translator = other_1223;
    let _e6: Translator = translator_inverse(_e5);
    let _e7: Motor = motor_translator_geometric_product(_e4, _e6);
    return _e7;
}

fn motor_translator_transformation(self_1416: Motor, other_1224: Translator) -> Translator {
    var self_1417: Motor;
    var other_1225: Translator;

    self_1417 = self_1416;
    other_1225 = other_1224;
    let _e4: Motor = self_1417;
    let _e5: Translator = other_1225;
    let _e6: Motor = motor_translator_geometric_product(_e4, _e5);
    let _e7: Motor = self_1417;
    let _e8: Motor = motor_reversal(_e7);
    let _e9: Motor = motor_motor_geometric_product(_e6, _e8);
    let _e10: Translator = motor_translator_into(_e9);
    return _e10;
}

fn motor_dual_ideal_point_geometric_quotient(self_1418: MotorDual, other_1226: IdealPoint) -> MotorDual {
    var self_1419: MotorDual;
    var other_1227: IdealPoint;

    self_1419 = self_1418;
    other_1227 = other_1226;
    let _e4: MotorDual = self_1419;
    let _e5: IdealPoint = other_1227;
    let _e6: IdealPoint = ideal_point_inverse(_e5);
    let _e7: MotorDual = motor_dual_ideal_point_geometric_product(_e4, _e6);
    return _e7;
}

fn motor_dual_ideal_point_transformation(self_1420: MotorDual, other_1228: IdealPoint) -> IdealPoint {
    var self_1421: MotorDual;
    var other_1229: IdealPoint;

    self_1421 = self_1420;
    other_1229 = other_1228;
    let _e4: MotorDual = self_1421;
    let _e5: IdealPoint = other_1229;
    let _e6: MotorDual = motor_dual_ideal_point_geometric_product(_e4, _e5);
    let _e7: MotorDual = self_1421;
    let _e8: MotorDual = motor_dual_reversal(_e7);
    let _e9: Motor = motor_dual_motor_dual_geometric_product(_e6, _e8);
    let _e10: IdealPoint = motor_ideal_point_into(_e9);
    return _e10;
}

fn motor_dual_motor_geometric_quotient(self_1422: MotorDual, other_1230: Motor) -> MotorDual {
    var self_1423: MotorDual;
    var other_1231: Motor;

    self_1423 = self_1422;
    other_1231 = other_1230;
    let _e4: MotorDual = self_1423;
    let _e5: Motor = other_1231;
    let _e6: Motor = motor_inverse(_e5);
    let _e7: MotorDual = motor_dual_motor_geometric_product(_e4, _e6);
    return _e7;
}

fn motor_dual_motor_transformation(self_1424: MotorDual, other_1232: Motor) -> Motor {
    var self_1425: MotorDual;
    var other_1233: Motor;

    self_1425 = self_1424;
    other_1233 = other_1232;
    let _e4: MotorDual = self_1425;
    let _e5: Motor = other_1233;
    let _e6: MotorDual = motor_dual_motor_geometric_product(_e4, _e5);
    let _e7: MotorDual = self_1425;
    let _e8: MotorDual = motor_dual_reversal(_e7);
    let _e9: Motor = motor_dual_motor_dual_geometric_product(_e6, _e8);
    return _e9;
}

fn motor_dual_motor_dual_geometric_quotient(self_1426: MotorDual, other_1234: MotorDual) -> Motor {
    var self_1427: MotorDual;
    var other_1235: MotorDual;

    self_1427 = self_1426;
    other_1235 = other_1234;
    let _e4: MotorDual = self_1427;
    let _e5: MotorDual = other_1235;
    let _e6: MotorDual = motor_dual_inverse(_e5);
    let _e7: Motor = motor_dual_motor_dual_geometric_product(_e4, _e6);
    return _e7;
}

fn motor_dual_motor_dual_transformation(self_1428: MotorDual, other_1236: MotorDual) -> MotorDual {
    var self_1429: MotorDual;
    var other_1237: MotorDual;

    self_1429 = self_1428;
    other_1237 = other_1236;
    let _e4: MotorDual = self_1429;
    let _e5: MotorDual = other_1237;
    let _e6: Motor = motor_dual_motor_dual_geometric_product(_e4, _e5);
    let _e7: MotorDual = self_1429;
    let _e8: MotorDual = motor_dual_reversal(_e7);
    let _e9: MotorDual = motor_motor_dual_geometric_product(_e6, _e8);
    return _e9;
}

fn motor_dual_multi_vector_geometric_quotient(self_1430: MotorDual, other_1238: MultiVector) -> MultiVector {
    var self_1431: MotorDual;
    var other_1239: MultiVector;

    self_1431 = self_1430;
    other_1239 = other_1238;
    let _e4: MotorDual = self_1431;
    let _e5: MultiVector = other_1239;
    let _e6: MultiVector = multi_vector_inverse(_e5);
    let _e7: MultiVector = motor_dual_multi_vector_geometric_product(_e4, _e6);
    return _e7;
}

fn motor_dual_multi_vector_transformation(self_1432: MotorDual, other_1240: MultiVector) -> MultiVector {
    var self_1433: MotorDual;
    var other_1241: MultiVector;

    self_1433 = self_1432;
    other_1241 = other_1240;
    let _e4: MotorDual = self_1433;
    let _e5: MultiVector = other_1241;
    let _e6: MultiVector = motor_dual_multi_vector_geometric_product(_e4, _e5);
    let _e7: MotorDual = self_1433;
    let _e8: MotorDual = motor_dual_reversal(_e7);
    let _e9: MultiVector = multi_vector_motor_dual_geometric_product(_e6, _e8);
    return _e9;
}

fn motor_dual_plane_geometric_quotient(self_1434: MotorDual, other_1242: Plane) -> Motor {
    var self_1435: MotorDual;
    var other_1243: Plane;

    self_1435 = self_1434;
    other_1243 = other_1242;
    let _e4: MotorDual = self_1435;
    let _e5: Plane = other_1243;
    let _e6: Plane = plane_inverse(_e5);
    let _e7: Motor = motor_dual_plane_geometric_product(_e4, _e6);
    return _e7;
}

fn motor_dual_plane_transformation(self_1436: MotorDual, other_1244: Plane) -> Plane {
    var self_1437: MotorDual;
    var other_1245: Plane;

    self_1437 = self_1436;
    other_1245 = other_1244;
    let _e4: MotorDual = self_1437;
    let _e5: Plane = other_1245;
    let _e6: Motor = motor_dual_plane_geometric_product(_e4, _e5);
    let _e7: MotorDual = self_1437;
    let _e8: MotorDual = motor_dual_reversal(_e7);
    let _e9: MotorDual = motor_motor_dual_geometric_product(_e6, _e8);
    let _e10: Plane = motor_dual_plane_into(_e9);
    return _e10;
}

fn motor_dual_point_geometric_quotient(self_1438: MotorDual, other_1246: Point) -> MotorDual {
    var self_1439: MotorDual;
    var other_1247: Point;

    self_1439 = self_1438;
    other_1247 = other_1246;
    let _e4: MotorDual = self_1439;
    let _e5: Point = other_1247;
    let _e6: Point = point_inverse(_e5);
    let _e7: MotorDual = motor_dual_point_geometric_product(_e4, _e6);
    return _e7;
}

fn motor_dual_point_transformation(self_1440: MotorDual, other_1248: Point) -> Point {
    var self_1441: MotorDual;
    var other_1249: Point;

    self_1441 = self_1440;
    other_1249 = other_1248;
    let _e4: MotorDual = self_1441;
    let _e5: Point = other_1249;
    let _e6: MotorDual = motor_dual_point_geometric_product(_e4, _e5);
    let _e7: MotorDual = self_1441;
    let _e8: MotorDual = motor_dual_reversal(_e7);
    let _e9: Motor = motor_dual_motor_dual_geometric_product(_e6, _e8);
    let _e10: Point = motor_point_into(_e9);
    return _e10;
}

fn motor_dual_rotor_geometric_quotient(self_1442: MotorDual, other_1250: Rotor) -> MotorDual {
    var self_1443: MotorDual;
    var other_1251: Rotor;

    self_1443 = self_1442;
    other_1251 = other_1250;
    let _e4: MotorDual = self_1443;
    let _e5: Rotor = other_1251;
    let _e6: Rotor = rotor_inverse(_e5);
    let _e7: MotorDual = motor_dual_rotor_geometric_product(_e4, _e6);
    return _e7;
}

fn motor_dual_rotor_transformation(self_1444: MotorDual, other_1252: Rotor) -> Rotor {
    var self_1445: MotorDual;
    var other_1253: Rotor;

    self_1445 = self_1444;
    other_1253 = other_1252;
    let _e4: MotorDual = self_1445;
    let _e5: Rotor = other_1253;
    let _e6: MotorDual = motor_dual_rotor_geometric_product(_e4, _e5);
    let _e7: MotorDual = self_1445;
    let _e8: MotorDual = motor_dual_reversal(_e7);
    let _e9: Motor = motor_dual_motor_dual_geometric_product(_e6, _e8);
    let _e10: Rotor = motor_rotor_into(_e9);
    return _e10;
}

fn motor_dual_scalar_geometric_quotient(self_1446: MotorDual, other_1254: Scalar) -> MotorDual {
    var self_1447: MotorDual;
    var other_1255: Scalar;

    self_1447 = self_1446;
    other_1255 = other_1254;
    let _e4: MotorDual = self_1447;
    let _e5: Scalar = other_1255;
    let _e6: Scalar = scalar_inverse(_e5);
    let _e7: MotorDual = motor_dual_scalar_geometric_product(_e4, _e6);
    return _e7;
}

fn motor_dual_scalar_transformation(self_1448: MotorDual, other_1256: Scalar) -> Scalar {
    var self_1449: MotorDual;
    var other_1257: Scalar;

    self_1449 = self_1448;
    other_1257 = other_1256;
    let _e4: MotorDual = self_1449;
    let _e5: Scalar = other_1257;
    let _e6: MotorDual = motor_dual_scalar_geometric_product(_e4, _e5);
    let _e7: MotorDual = self_1449;
    let _e8: MotorDual = motor_dual_reversal(_e7);
    let _e9: Motor = motor_dual_motor_dual_geometric_product(_e6, _e8);
    let _e10: Scalar = motor_scalar_into(_e9);
    return _e10;
}

fn motor_dual_translator_geometric_quotient(self_1450: MotorDual, other_1258: Translator) -> MotorDual {
    var self_1451: MotorDual;
    var other_1259: Translator;

    self_1451 = self_1450;
    other_1259 = other_1258;
    let _e4: MotorDual = self_1451;
    let _e5: Translator = other_1259;
    let _e6: Translator = translator_inverse(_e5);
    let _e7: MotorDual = motor_dual_translator_geometric_product(_e4, _e6);
    return _e7;
}

fn motor_dual_translator_transformation(self_1452: MotorDual, other_1260: Translator) -> Translator {
    var self_1453: MotorDual;
    var other_1261: Translator;

    self_1453 = self_1452;
    other_1261 = other_1260;
    let _e4: MotorDual = self_1453;
    let _e5: Translator = other_1261;
    let _e6: MotorDual = motor_dual_translator_geometric_product(_e4, _e5);
    let _e7: MotorDual = self_1453;
    let _e8: MotorDual = motor_dual_reversal(_e7);
    let _e9: Motor = motor_dual_motor_dual_geometric_product(_e6, _e8);
    let _e10: Translator = motor_translator_into(_e9);
    return _e10;
}

fn multi_vector_ideal_point_geometric_quotient(self_1454: MultiVector, other_1262: IdealPoint) -> MultiVector {
    var self_1455: MultiVector;
    var other_1263: IdealPoint;

    self_1455 = self_1454;
    other_1263 = other_1262;
    let _e4: MultiVector = self_1455;
    let _e5: IdealPoint = other_1263;
    let _e6: IdealPoint = ideal_point_inverse(_e5);
    let _e7: MultiVector = multi_vector_ideal_point_geometric_product(_e4, _e6);
    return _e7;
}

fn multi_vector_ideal_point_transformation(self_1456: MultiVector, other_1264: IdealPoint) -> IdealPoint {
    var self_1457: MultiVector;
    var other_1265: IdealPoint;

    self_1457 = self_1456;
    other_1265 = other_1264;
    let _e4: MultiVector = self_1457;
    let _e5: IdealPoint = other_1265;
    let _e6: MultiVector = multi_vector_ideal_point_geometric_product(_e4, _e5);
    let _e7: MultiVector = self_1457;
    let _e8: MultiVector = multi_vector_reversal(_e7);
    let _e9: MultiVector = multi_vector_multi_vector_geometric_product(_e6, _e8);
    let _e10: IdealPoint = multi_vector_ideal_point_into(_e9);
    return _e10;
}

fn multi_vector_motor_geometric_quotient(self_1458: MultiVector, other_1266: Motor) -> MultiVector {
    var self_1459: MultiVector;
    var other_1267: Motor;

    self_1459 = self_1458;
    other_1267 = other_1266;
    let _e4: MultiVector = self_1459;
    let _e5: Motor = other_1267;
    let _e6: Motor = motor_inverse(_e5);
    let _e7: MultiVector = multi_vector_motor_geometric_product(_e4, _e6);
    return _e7;
}

fn multi_vector_motor_transformation(self_1460: MultiVector, other_1268: Motor) -> Motor {
    var self_1461: MultiVector;
    var other_1269: Motor;

    self_1461 = self_1460;
    other_1269 = other_1268;
    let _e4: MultiVector = self_1461;
    let _e5: Motor = other_1269;
    let _e6: MultiVector = multi_vector_motor_geometric_product(_e4, _e5);
    let _e7: MultiVector = self_1461;
    let _e8: MultiVector = multi_vector_reversal(_e7);
    let _e9: MultiVector = multi_vector_multi_vector_geometric_product(_e6, _e8);
    let _e10: Motor = multi_vector_motor_into(_e9);
    return _e10;
}

fn multi_vector_motor_dual_geometric_quotient(self_1462: MultiVector, other_1270: MotorDual) -> MultiVector {
    var self_1463: MultiVector;
    var other_1271: MotorDual;

    self_1463 = self_1462;
    other_1271 = other_1270;
    let _e4: MultiVector = self_1463;
    let _e5: MotorDual = other_1271;
    let _e6: MotorDual = motor_dual_inverse(_e5);
    let _e7: MultiVector = multi_vector_motor_dual_geometric_product(_e4, _e6);
    return _e7;
}

fn multi_vector_motor_dual_transformation(self_1464: MultiVector, other_1272: MotorDual) -> MotorDual {
    var self_1465: MultiVector;
    var other_1273: MotorDual;

    self_1465 = self_1464;
    other_1273 = other_1272;
    let _e4: MultiVector = self_1465;
    let _e5: MotorDual = other_1273;
    let _e6: MultiVector = multi_vector_motor_dual_geometric_product(_e4, _e5);
    let _e7: MultiVector = self_1465;
    let _e8: MultiVector = multi_vector_reversal(_e7);
    let _e9: MultiVector = multi_vector_multi_vector_geometric_product(_e6, _e8);
    let _e10: MotorDual = multi_vector_motor_dual_into(_e9);
    return _e10;
}

fn multi_vector_powi(self_1466: MultiVector, exponent_2: i32) -> MultiVector {
    var self_1467: MultiVector;
    var exponent_3: i32;
    var local_1: MultiVector;
    var x_1: MultiVector;
    var y_1: MultiVector;
    var n_1: i32;

    self_1467 = self_1466;
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
        let _e11: MultiVector = self_1467;
        let _e12: MultiVector = multi_vector_inverse(_e11);
        local_1 = _e12;
    } else {
        let _e14: MultiVector = self_1467;
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

fn multi_vector_multi_vector_geometric_quotient(self_1468: MultiVector, other_1274: MultiVector) -> MultiVector {
    var self_1469: MultiVector;
    var other_1275: MultiVector;

    self_1469 = self_1468;
    other_1275 = other_1274;
    let _e4: MultiVector = self_1469;
    let _e5: MultiVector = other_1275;
    let _e6: MultiVector = multi_vector_inverse(_e5);
    let _e7: MultiVector = multi_vector_multi_vector_geometric_product(_e4, _e6);
    return _e7;
}

fn multi_vector_multi_vector_transformation(self_1470: MultiVector, other_1276: MultiVector) -> MultiVector {
    var self_1471: MultiVector;
    var other_1277: MultiVector;

    self_1471 = self_1470;
    other_1277 = other_1276;
    let _e4: MultiVector = self_1471;
    let _e5: MultiVector = other_1277;
    let _e6: MultiVector = multi_vector_multi_vector_geometric_product(_e4, _e5);
    let _e7: MultiVector = self_1471;
    let _e8: MultiVector = multi_vector_reversal(_e7);
    let _e9: MultiVector = multi_vector_multi_vector_geometric_product(_e6, _e8);
    return _e9;
}

fn multi_vector_plane_geometric_quotient(self_1472: MultiVector, other_1278: Plane) -> MultiVector {
    var self_1473: MultiVector;
    var other_1279: Plane;

    self_1473 = self_1472;
    other_1279 = other_1278;
    let _e4: MultiVector = self_1473;
    let _e5: Plane = other_1279;
    let _e6: Plane = plane_inverse(_e5);
    let _e7: MultiVector = multi_vector_plane_geometric_product(_e4, _e6);
    return _e7;
}

fn multi_vector_plane_transformation(self_1474: MultiVector, other_1280: Plane) -> Plane {
    var self_1475: MultiVector;
    var other_1281: Plane;

    self_1475 = self_1474;
    other_1281 = other_1280;
    let _e4: MultiVector = self_1475;
    let _e5: Plane = other_1281;
    let _e6: MultiVector = multi_vector_plane_geometric_product(_e4, _e5);
    let _e7: MultiVector = self_1475;
    let _e8: MultiVector = multi_vector_reversal(_e7);
    let _e9: MultiVector = multi_vector_multi_vector_geometric_product(_e6, _e8);
    let _e10: Plane = multi_vector_plane_into(_e9);
    return _e10;
}

fn multi_vector_point_geometric_quotient(self_1476: MultiVector, other_1282: Point) -> MultiVector {
    var self_1477: MultiVector;
    var other_1283: Point;

    self_1477 = self_1476;
    other_1283 = other_1282;
    let _e4: MultiVector = self_1477;
    let _e5: Point = other_1283;
    let _e6: Point = point_inverse(_e5);
    let _e7: MultiVector = multi_vector_point_geometric_product(_e4, _e6);
    return _e7;
}

fn multi_vector_point_transformation(self_1478: MultiVector, other_1284: Point) -> Point {
    var self_1479: MultiVector;
    var other_1285: Point;

    self_1479 = self_1478;
    other_1285 = other_1284;
    let _e4: MultiVector = self_1479;
    let _e5: Point = other_1285;
    let _e6: MultiVector = multi_vector_point_geometric_product(_e4, _e5);
    let _e7: MultiVector = self_1479;
    let _e8: MultiVector = multi_vector_reversal(_e7);
    let _e9: MultiVector = multi_vector_multi_vector_geometric_product(_e6, _e8);
    let _e10: Point = multi_vector_point_into(_e9);
    return _e10;
}

fn multi_vector_rotor_geometric_quotient(self_1480: MultiVector, other_1286: Rotor) -> MultiVector {
    var self_1481: MultiVector;
    var other_1287: Rotor;

    self_1481 = self_1480;
    other_1287 = other_1286;
    let _e4: MultiVector = self_1481;
    let _e5: Rotor = other_1287;
    let _e6: Rotor = rotor_inverse(_e5);
    let _e7: MultiVector = multi_vector_rotor_geometric_product(_e4, _e6);
    return _e7;
}

fn multi_vector_rotor_transformation(self_1482: MultiVector, other_1288: Rotor) -> Rotor {
    var self_1483: MultiVector;
    var other_1289: Rotor;

    self_1483 = self_1482;
    other_1289 = other_1288;
    let _e4: MultiVector = self_1483;
    let _e5: Rotor = other_1289;
    let _e6: MultiVector = multi_vector_rotor_geometric_product(_e4, _e5);
    let _e7: MultiVector = self_1483;
    let _e8: MultiVector = multi_vector_reversal(_e7);
    let _e9: MultiVector = multi_vector_multi_vector_geometric_product(_e6, _e8);
    let _e10: Rotor = multi_vector_rotor_into(_e9);
    return _e10;
}

fn multi_vector_scalar_geometric_quotient(self_1484: MultiVector, other_1290: Scalar) -> MultiVector {
    var self_1485: MultiVector;
    var other_1291: Scalar;

    self_1485 = self_1484;
    other_1291 = other_1290;
    let _e4: MultiVector = self_1485;
    let _e5: Scalar = other_1291;
    let _e6: Scalar = scalar_inverse(_e5);
    let _e7: MultiVector = multi_vector_scalar_geometric_product(_e4, _e6);
    return _e7;
}

fn multi_vector_scalar_transformation(self_1486: MultiVector, other_1292: Scalar) -> Scalar {
    var self_1487: MultiVector;
    var other_1293: Scalar;

    self_1487 = self_1486;
    other_1293 = other_1292;
    let _e4: MultiVector = self_1487;
    let _e5: Scalar = other_1293;
    let _e6: MultiVector = multi_vector_scalar_geometric_product(_e4, _e5);
    let _e7: MultiVector = self_1487;
    let _e8: MultiVector = multi_vector_reversal(_e7);
    let _e9: MultiVector = multi_vector_multi_vector_geometric_product(_e6, _e8);
    let _e10: Scalar = multi_vector_scalar_into(_e9);
    return _e10;
}

fn multi_vector_translator_geometric_quotient(self_1488: MultiVector, other_1294: Translator) -> MultiVector {
    var self_1489: MultiVector;
    var other_1295: Translator;

    self_1489 = self_1488;
    other_1295 = other_1294;
    let _e4: MultiVector = self_1489;
    let _e5: Translator = other_1295;
    let _e6: Translator = translator_inverse(_e5);
    let _e7: MultiVector = multi_vector_translator_geometric_product(_e4, _e6);
    return _e7;
}

fn multi_vector_translator_transformation(self_1490: MultiVector, other_1296: Translator) -> Translator {
    var self_1491: MultiVector;
    var other_1297: Translator;

    self_1491 = self_1490;
    other_1297 = other_1296;
    let _e4: MultiVector = self_1491;
    let _e5: Translator = other_1297;
    let _e6: MultiVector = multi_vector_translator_geometric_product(_e4, _e5);
    let _e7: MultiVector = self_1491;
    let _e8: MultiVector = multi_vector_reversal(_e7);
    let _e9: MultiVector = multi_vector_multi_vector_geometric_product(_e6, _e8);
    let _e10: Translator = multi_vector_translator_into(_e9);
    return _e10;
}

fn plane_ideal_point_geometric_quotient(self_1492: Plane, other_1298: IdealPoint) -> MotorDual {
    var self_1493: Plane;
    var other_1299: IdealPoint;

    self_1493 = self_1492;
    other_1299 = other_1298;
    let _e4: Plane = self_1493;
    let _e5: IdealPoint = other_1299;
    let _e6: IdealPoint = ideal_point_inverse(_e5);
    let _e7: MotorDual = plane_ideal_point_geometric_product(_e4, _e6);
    return _e7;
}

fn plane_ideal_point_transformation(self_1494: Plane, other_1300: IdealPoint) -> IdealPoint {
    var self_1495: Plane;
    var other_1301: IdealPoint;

    self_1495 = self_1494;
    other_1301 = other_1300;
    let _e4: Plane = self_1495;
    let _e5: IdealPoint = other_1301;
    let _e6: MotorDual = plane_ideal_point_geometric_product(_e4, _e5);
    let _e7: Plane = self_1495;
    let _e8: Plane = plane_reversal(_e7);
    let _e9: Motor = motor_dual_plane_geometric_product(_e6, _e8);
    let _e10: IdealPoint = motor_ideal_point_into(_e9);
    return _e10;
}

fn plane_motor_geometric_quotient(self_1496: Plane, other_1302: Motor) -> MotorDual {
    var self_1497: Plane;
    var other_1303: Motor;

    self_1497 = self_1496;
    other_1303 = other_1302;
    let _e4: Plane = self_1497;
    let _e5: Motor = other_1303;
    let _e6: Motor = motor_inverse(_e5);
    let _e7: MotorDual = plane_motor_geometric_product(_e4, _e6);
    return _e7;
}

fn plane_motor_transformation(self_1498: Plane, other_1304: Motor) -> Motor {
    var self_1499: Plane;
    var other_1305: Motor;

    self_1499 = self_1498;
    other_1305 = other_1304;
    let _e4: Plane = self_1499;
    let _e5: Motor = other_1305;
    let _e6: MotorDual = plane_motor_geometric_product(_e4, _e5);
    let _e7: Plane = self_1499;
    let _e8: Plane = plane_reversal(_e7);
    let _e9: Motor = motor_dual_plane_geometric_product(_e6, _e8);
    return _e9;
}

fn plane_motor_dual_geometric_quotient(self_1500: Plane, other_1306: MotorDual) -> Motor {
    var self_1501: Plane;
    var other_1307: MotorDual;

    self_1501 = self_1500;
    other_1307 = other_1306;
    let _e4: Plane = self_1501;
    let _e5: MotorDual = other_1307;
    let _e6: MotorDual = motor_dual_inverse(_e5);
    let _e7: Motor = plane_motor_dual_geometric_product(_e4, _e6);
    return _e7;
}

fn plane_motor_dual_transformation(self_1502: Plane, other_1308: MotorDual) -> MotorDual {
    var self_1503: Plane;
    var other_1309: MotorDual;

    self_1503 = self_1502;
    other_1309 = other_1308;
    let _e4: Plane = self_1503;
    let _e5: MotorDual = other_1309;
    let _e6: Motor = plane_motor_dual_geometric_product(_e4, _e5);
    let _e7: Plane = self_1503;
    let _e8: Plane = plane_reversal(_e7);
    let _e9: MotorDual = motor_plane_geometric_product(_e6, _e8);
    return _e9;
}

fn plane_multi_vector_geometric_quotient(self_1504: Plane, other_1310: MultiVector) -> MultiVector {
    var self_1505: Plane;
    var other_1311: MultiVector;

    self_1505 = self_1504;
    other_1311 = other_1310;
    let _e4: Plane = self_1505;
    let _e5: MultiVector = other_1311;
    let _e6: MultiVector = multi_vector_inverse(_e5);
    let _e7: MultiVector = plane_multi_vector_geometric_product(_e4, _e6);
    return _e7;
}

fn plane_multi_vector_transformation(self_1506: Plane, other_1312: MultiVector) -> MultiVector {
    var self_1507: Plane;
    var other_1313: MultiVector;

    self_1507 = self_1506;
    other_1313 = other_1312;
    let _e4: Plane = self_1507;
    let _e5: MultiVector = other_1313;
    let _e6: MultiVector = plane_multi_vector_geometric_product(_e4, _e5);
    let _e7: Plane = self_1507;
    let _e8: Plane = plane_reversal(_e7);
    let _e9: MultiVector = multi_vector_plane_geometric_product(_e6, _e8);
    return _e9;
}

fn plane_plane_geometric_quotient(self_1508: Plane, other_1314: Plane) -> Motor {
    var self_1509: Plane;
    var other_1315: Plane;

    self_1509 = self_1508;
    other_1315 = other_1314;
    let _e4: Plane = self_1509;
    let _e5: Plane = other_1315;
    let _e6: Plane = plane_inverse(_e5);
    let _e7: Motor = plane_plane_geometric_product(_e4, _e6);
    return _e7;
}

fn plane_plane_transformation(self_1510: Plane, other_1316: Plane) -> Plane {
    var self_1511: Plane;
    var other_1317: Plane;

    self_1511 = self_1510;
    other_1317 = other_1316;
    let _e4: Plane = self_1511;
    let _e5: Plane = other_1317;
    let _e6: Motor = plane_plane_geometric_product(_e4, _e5);
    let _e7: Plane = self_1511;
    let _e8: Plane = plane_reversal(_e7);
    let _e9: MotorDual = motor_plane_geometric_product(_e6, _e8);
    let _e10: Plane = motor_dual_plane_into(_e9);
    return _e10;
}

fn plane_point_geometric_quotient(self_1512: Plane, other_1318: Point) -> MotorDual {
    var self_1513: Plane;
    var other_1319: Point;

    self_1513 = self_1512;
    other_1319 = other_1318;
    let _e4: Plane = self_1513;
    let _e5: Point = other_1319;
    let _e6: Point = point_inverse(_e5);
    let _e7: MotorDual = plane_point_geometric_product(_e4, _e6);
    return _e7;
}

fn plane_point_transformation(self_1514: Plane, other_1320: Point) -> Point {
    var self_1515: Plane;
    var other_1321: Point;

    self_1515 = self_1514;
    other_1321 = other_1320;
    let _e4: Plane = self_1515;
    let _e5: Point = other_1321;
    let _e6: MotorDual = plane_point_geometric_product(_e4, _e5);
    let _e7: Plane = self_1515;
    let _e8: Plane = plane_reversal(_e7);
    let _e9: Motor = motor_dual_plane_geometric_product(_e6, _e8);
    let _e10: Point = motor_point_into(_e9);
    return _e10;
}

fn plane_rotor_geometric_quotient(self_1516: Plane, other_1322: Rotor) -> MotorDual {
    var self_1517: Plane;
    var other_1323: Rotor;

    self_1517 = self_1516;
    other_1323 = other_1322;
    let _e4: Plane = self_1517;
    let _e5: Rotor = other_1323;
    let _e6: Rotor = rotor_inverse(_e5);
    let _e7: MotorDual = plane_rotor_geometric_product(_e4, _e6);
    return _e7;
}

fn plane_rotor_transformation(self_1518: Plane, other_1324: Rotor) -> Rotor {
    var self_1519: Plane;
    var other_1325: Rotor;

    self_1519 = self_1518;
    other_1325 = other_1324;
    let _e4: Plane = self_1519;
    let _e5: Rotor = other_1325;
    let _e6: MotorDual = plane_rotor_geometric_product(_e4, _e5);
    let _e7: Plane = self_1519;
    let _e8: Plane = plane_reversal(_e7);
    let _e9: Motor = motor_dual_plane_geometric_product(_e6, _e8);
    let _e10: Rotor = motor_rotor_into(_e9);
    return _e10;
}

fn plane_scalar_geometric_quotient(self_1520: Plane, other_1326: Scalar) -> Plane {
    var self_1521: Plane;
    var other_1327: Scalar;

    self_1521 = self_1520;
    other_1327 = other_1326;
    let _e4: Plane = self_1521;
    let _e5: Scalar = other_1327;
    let _e6: Scalar = scalar_inverse(_e5);
    let _e7: Plane = plane_scalar_geometric_product(_e4, _e6);
    return _e7;
}

fn plane_scalar_transformation(self_1522: Plane, other_1328: Scalar) -> Scalar {
    var self_1523: Plane;
    var other_1329: Scalar;

    self_1523 = self_1522;
    other_1329 = other_1328;
    let _e4: Plane = self_1523;
    let _e5: Scalar = other_1329;
    let _e6: Plane = plane_scalar_geometric_product(_e4, _e5);
    let _e7: Plane = self_1523;
    let _e8: Plane = plane_reversal(_e7);
    let _e9: Motor = plane_plane_geometric_product(_e6, _e8);
    let _e10: Scalar = motor_scalar_into(_e9);
    return _e10;
}

fn plane_translator_geometric_quotient(self_1524: Plane, other_1330: Translator) -> MotorDual {
    var self_1525: Plane;
    var other_1331: Translator;

    self_1525 = self_1524;
    other_1331 = other_1330;
    let _e4: Plane = self_1525;
    let _e5: Translator = other_1331;
    let _e6: Translator = translator_inverse(_e5);
    let _e7: MotorDual = plane_translator_geometric_product(_e4, _e6);
    return _e7;
}

fn plane_translator_transformation(self_1526: Plane, other_1332: Translator) -> Translator {
    var self_1527: Plane;
    var other_1333: Translator;

    self_1527 = self_1526;
    other_1333 = other_1332;
    let _e4: Plane = self_1527;
    let _e5: Translator = other_1333;
    let _e6: MotorDual = plane_translator_geometric_product(_e4, _e5);
    let _e7: Plane = self_1527;
    let _e8: Plane = plane_reversal(_e7);
    let _e9: Motor = motor_dual_plane_geometric_product(_e6, _e8);
    let _e10: Translator = motor_translator_into(_e9);
    return _e10;
}

fn point_ideal_point_geometric_quotient(self_1528: Point, other_1334: IdealPoint) -> Motor {
    var self_1529: Point;
    var other_1335: IdealPoint;

    self_1529 = self_1528;
    other_1335 = other_1334;
    let _e4: Point = self_1529;
    let _e5: IdealPoint = other_1335;
    let _e6: IdealPoint = ideal_point_inverse(_e5);
    let _e7: Motor = point_ideal_point_geometric_product(_e4, _e6);
    return _e7;
}

fn point_ideal_point_transformation(self_1530: Point, other_1336: IdealPoint) -> IdealPoint {
    var self_1531: Point;
    var other_1337: IdealPoint;

    self_1531 = self_1530;
    other_1337 = other_1336;
    let _e4: Point = self_1531;
    let _e5: IdealPoint = other_1337;
    let _e6: Motor = point_ideal_point_geometric_product(_e4, _e5);
    let _e7: Point = self_1531;
    let _e8: Point = point_reversal(_e7);
    let _e9: Motor = motor_point_geometric_product(_e6, _e8);
    let _e10: IdealPoint = motor_ideal_point_into(_e9);
    return _e10;
}

fn point_motor_geometric_quotient(self_1532: Point, other_1338: Motor) -> Motor {
    var self_1533: Point;
    var other_1339: Motor;

    self_1533 = self_1532;
    other_1339 = other_1338;
    let _e4: Point = self_1533;
    let _e5: Motor = other_1339;
    let _e6: Motor = motor_inverse(_e5);
    let _e7: Motor = point_motor_geometric_product(_e4, _e6);
    return _e7;
}

fn point_motor_transformation(self_1534: Point, other_1340: Motor) -> Motor {
    var self_1535: Point;
    var other_1341: Motor;

    self_1535 = self_1534;
    other_1341 = other_1340;
    let _e4: Point = self_1535;
    let _e5: Motor = other_1341;
    let _e6: Motor = point_motor_geometric_product(_e4, _e5);
    let _e7: Point = self_1535;
    let _e8: Point = point_reversal(_e7);
    let _e9: Motor = motor_point_geometric_product(_e6, _e8);
    return _e9;
}

fn point_motor_dual_geometric_quotient(self_1536: Point, other_1342: MotorDual) -> MotorDual {
    var self_1537: Point;
    var other_1343: MotorDual;

    self_1537 = self_1536;
    other_1343 = other_1342;
    let _e4: Point = self_1537;
    let _e5: MotorDual = other_1343;
    let _e6: MotorDual = motor_dual_inverse(_e5);
    let _e7: MotorDual = point_motor_dual_geometric_product(_e4, _e6);
    return _e7;
}

fn point_motor_dual_transformation(self_1538: Point, other_1344: MotorDual) -> MotorDual {
    var self_1539: Point;
    var other_1345: MotorDual;

    self_1539 = self_1538;
    other_1345 = other_1344;
    let _e4: Point = self_1539;
    let _e5: MotorDual = other_1345;
    let _e6: MotorDual = point_motor_dual_geometric_product(_e4, _e5);
    let _e7: Point = self_1539;
    let _e8: Point = point_reversal(_e7);
    let _e9: MotorDual = motor_dual_point_geometric_product(_e6, _e8);
    return _e9;
}

fn point_multi_vector_geometric_quotient(self_1540: Point, other_1346: MultiVector) -> MultiVector {
    var self_1541: Point;
    var other_1347: MultiVector;

    self_1541 = self_1540;
    other_1347 = other_1346;
    let _e4: Point = self_1541;
    let _e5: MultiVector = other_1347;
    let _e6: MultiVector = multi_vector_inverse(_e5);
    let _e7: MultiVector = point_multi_vector_geometric_product(_e4, _e6);
    return _e7;
}

fn point_multi_vector_transformation(self_1542: Point, other_1348: MultiVector) -> MultiVector {
    var self_1543: Point;
    var other_1349: MultiVector;

    self_1543 = self_1542;
    other_1349 = other_1348;
    let _e4: Point = self_1543;
    let _e5: MultiVector = other_1349;
    let _e6: MultiVector = point_multi_vector_geometric_product(_e4, _e5);
    let _e7: Point = self_1543;
    let _e8: Point = point_reversal(_e7);
    let _e9: MultiVector = multi_vector_point_geometric_product(_e6, _e8);
    return _e9;
}

fn point_plane_geometric_quotient(self_1544: Point, other_1350: Plane) -> MotorDual {
    var self_1545: Point;
    var other_1351: Plane;

    self_1545 = self_1544;
    other_1351 = other_1350;
    let _e4: Point = self_1545;
    let _e5: Plane = other_1351;
    let _e6: Plane = plane_inverse(_e5);
    let _e7: MotorDual = point_plane_geometric_product(_e4, _e6);
    return _e7;
}

fn point_plane_transformation(self_1546: Point, other_1352: Plane) -> Plane {
    var self_1547: Point;
    var other_1353: Plane;

    self_1547 = self_1546;
    other_1353 = other_1352;
    let _e4: Point = self_1547;
    let _e5: Plane = other_1353;
    let _e6: MotorDual = point_plane_geometric_product(_e4, _e5);
    let _e7: Point = self_1547;
    let _e8: Point = point_reversal(_e7);
    let _e9: MotorDual = motor_dual_point_geometric_product(_e6, _e8);
    let _e10: Plane = motor_dual_plane_into(_e9);
    return _e10;
}

fn point_point_geometric_quotient(self_1548: Point, other_1354: Point) -> Motor {
    var self_1549: Point;
    var other_1355: Point;

    self_1549 = self_1548;
    other_1355 = other_1354;
    let _e4: Point = self_1549;
    let _e5: Point = other_1355;
    let _e6: Point = point_inverse(_e5);
    let _e7: Motor = point_point_geometric_product(_e4, _e6);
    return _e7;
}

fn point_point_transformation(self_1550: Point, other_1356: Point) -> Point {
    var self_1551: Point;
    var other_1357: Point;

    self_1551 = self_1550;
    other_1357 = other_1356;
    let _e4: Point = self_1551;
    let _e5: Point = other_1357;
    let _e6: Motor = point_point_geometric_product(_e4, _e5);
    let _e7: Point = self_1551;
    let _e8: Point = point_reversal(_e7);
    let _e9: Motor = motor_point_geometric_product(_e6, _e8);
    let _e10: Point = motor_point_into(_e9);
    return _e10;
}

fn point_rotor_geometric_quotient(self_1552: Point, other_1358: Rotor) -> Motor {
    var self_1553: Point;
    var other_1359: Rotor;

    self_1553 = self_1552;
    other_1359 = other_1358;
    let _e4: Point = self_1553;
    let _e5: Rotor = other_1359;
    let _e6: Rotor = rotor_inverse(_e5);
    let _e7: Motor = point_rotor_geometric_product(_e4, _e6);
    return _e7;
}

fn point_rotor_transformation(self_1554: Point, other_1360: Rotor) -> Rotor {
    var self_1555: Point;
    var other_1361: Rotor;

    self_1555 = self_1554;
    other_1361 = other_1360;
    let _e4: Point = self_1555;
    let _e5: Rotor = other_1361;
    let _e6: Motor = point_rotor_geometric_product(_e4, _e5);
    let _e7: Point = self_1555;
    let _e8: Point = point_reversal(_e7);
    let _e9: Motor = motor_point_geometric_product(_e6, _e8);
    let _e10: Rotor = motor_rotor_into(_e9);
    return _e10;
}

fn point_scalar_geometric_quotient(self_1556: Point, other_1362: Scalar) -> Point {
    var self_1557: Point;
    var other_1363: Scalar;

    self_1557 = self_1556;
    other_1363 = other_1362;
    let _e4: Point = self_1557;
    let _e5: Scalar = other_1363;
    let _e6: Scalar = scalar_inverse(_e5);
    let _e7: Point = point_scalar_geometric_product(_e4, _e6);
    return _e7;
}

fn point_scalar_transformation(self_1558: Point, other_1364: Scalar) -> Scalar {
    var self_1559: Point;
    var other_1365: Scalar;

    self_1559 = self_1558;
    other_1365 = other_1364;
    let _e4: Point = self_1559;
    let _e5: Scalar = other_1365;
    let _e6: Point = point_scalar_geometric_product(_e4, _e5);
    let _e7: Point = self_1559;
    let _e8: Point = point_reversal(_e7);
    let _e9: Motor = point_point_geometric_product(_e6, _e8);
    let _e10: Scalar = motor_scalar_into(_e9);
    return _e10;
}

fn point_translator_geometric_quotient(self_1560: Point, other_1366: Translator) -> Motor {
    var self_1561: Point;
    var other_1367: Translator;

    self_1561 = self_1560;
    other_1367 = other_1366;
    let _e4: Point = self_1561;
    let _e5: Translator = other_1367;
    let _e6: Translator = translator_inverse(_e5);
    let _e7: Motor = point_translator_geometric_product(_e4, _e6);
    return _e7;
}

fn point_translator_transformation(self_1562: Point, other_1368: Translator) -> Translator {
    var self_1563: Point;
    var other_1369: Translator;

    self_1563 = self_1562;
    other_1369 = other_1368;
    let _e4: Point = self_1563;
    let _e5: Translator = other_1369;
    let _e6: Motor = point_translator_geometric_product(_e4, _e5);
    let _e7: Point = self_1563;
    let _e8: Point = point_reversal(_e7);
    let _e9: Motor = motor_point_geometric_product(_e6, _e8);
    let _e10: Translator = motor_translator_into(_e9);
    return _e10;
}

fn rotor_ideal_point_geometric_quotient(self_1564: Rotor, other_1370: IdealPoint) -> IdealPoint {
    var self_1565: Rotor;
    var other_1371: IdealPoint;

    self_1565 = self_1564;
    other_1371 = other_1370;
    let _e4: Rotor = self_1565;
    let _e5: IdealPoint = other_1371;
    let _e6: IdealPoint = ideal_point_inverse(_e5);
    let _e7: IdealPoint = rotor_ideal_point_geometric_product(_e4, _e6);
    return _e7;
}

fn rotor_ideal_point_transformation(self_1566: Rotor, other_1372: IdealPoint) -> IdealPoint {
    var self_1567: Rotor;
    var other_1373: IdealPoint;

    self_1567 = self_1566;
    other_1373 = other_1372;
    let _e4: Rotor = self_1567;
    let _e5: IdealPoint = other_1373;
    let _e6: IdealPoint = rotor_ideal_point_geometric_product(_e4, _e5);
    let _e7: Rotor = self_1567;
    let _e8: Rotor = rotor_reversal(_e7);
    let _e9: IdealPoint = ideal_point_rotor_geometric_product(_e6, _e8);
    return _e9;
}

fn rotor_motor_geometric_quotient(self_1568: Rotor, other_1374: Motor) -> Motor {
    var self_1569: Rotor;
    var other_1375: Motor;

    self_1569 = self_1568;
    other_1375 = other_1374;
    let _e4: Rotor = self_1569;
    let _e5: Motor = other_1375;
    let _e6: Motor = motor_inverse(_e5);
    let _e7: Motor = rotor_motor_geometric_product(_e4, _e6);
    return _e7;
}

fn rotor_motor_transformation(self_1570: Rotor, other_1376: Motor) -> Motor {
    var self_1571: Rotor;
    var other_1377: Motor;

    self_1571 = self_1570;
    other_1377 = other_1376;
    let _e4: Rotor = self_1571;
    let _e5: Motor = other_1377;
    let _e6: Motor = rotor_motor_geometric_product(_e4, _e5);
    let _e7: Rotor = self_1571;
    let _e8: Rotor = rotor_reversal(_e7);
    let _e9: Motor = motor_rotor_geometric_product(_e6, _e8);
    return _e9;
}

fn rotor_motor_dual_geometric_quotient(self_1572: Rotor, other_1378: MotorDual) -> MotorDual {
    var self_1573: Rotor;
    var other_1379: MotorDual;

    self_1573 = self_1572;
    other_1379 = other_1378;
    let _e4: Rotor = self_1573;
    let _e5: MotorDual = other_1379;
    let _e6: MotorDual = motor_dual_inverse(_e5);
    let _e7: MotorDual = rotor_motor_dual_geometric_product(_e4, _e6);
    return _e7;
}

fn rotor_motor_dual_transformation(self_1574: Rotor, other_1380: MotorDual) -> MotorDual {
    var self_1575: Rotor;
    var other_1381: MotorDual;

    self_1575 = self_1574;
    other_1381 = other_1380;
    let _e4: Rotor = self_1575;
    let _e5: MotorDual = other_1381;
    let _e6: MotorDual = rotor_motor_dual_geometric_product(_e4, _e5);
    let _e7: Rotor = self_1575;
    let _e8: Rotor = rotor_reversal(_e7);
    let _e9: MotorDual = motor_dual_rotor_geometric_product(_e6, _e8);
    return _e9;
}

fn rotor_multi_vector_geometric_quotient(self_1576: Rotor, other_1382: MultiVector) -> MultiVector {
    var self_1577: Rotor;
    var other_1383: MultiVector;

    self_1577 = self_1576;
    other_1383 = other_1382;
    let _e4: Rotor = self_1577;
    let _e5: MultiVector = other_1383;
    let _e6: MultiVector = multi_vector_inverse(_e5);
    let _e7: MultiVector = rotor_multi_vector_geometric_product(_e4, _e6);
    return _e7;
}

fn rotor_multi_vector_transformation(self_1578: Rotor, other_1384: MultiVector) -> MultiVector {
    var self_1579: Rotor;
    var other_1385: MultiVector;

    self_1579 = self_1578;
    other_1385 = other_1384;
    let _e4: Rotor = self_1579;
    let _e5: MultiVector = other_1385;
    let _e6: MultiVector = rotor_multi_vector_geometric_product(_e4, _e5);
    let _e7: Rotor = self_1579;
    let _e8: Rotor = rotor_reversal(_e7);
    let _e9: MultiVector = multi_vector_rotor_geometric_product(_e6, _e8);
    return _e9;
}

fn rotor_plane_geometric_quotient(self_1580: Rotor, other_1386: Plane) -> MotorDual {
    var self_1581: Rotor;
    var other_1387: Plane;

    self_1581 = self_1580;
    other_1387 = other_1386;
    let _e4: Rotor = self_1581;
    let _e5: Plane = other_1387;
    let _e6: Plane = plane_inverse(_e5);
    let _e7: MotorDual = rotor_plane_geometric_product(_e4, _e6);
    return _e7;
}

fn rotor_plane_transformation(self_1582: Rotor, other_1388: Plane) -> Plane {
    var self_1583: Rotor;
    var other_1389: Plane;

    self_1583 = self_1582;
    other_1389 = other_1388;
    let _e4: Rotor = self_1583;
    let _e5: Plane = other_1389;
    let _e6: MotorDual = rotor_plane_geometric_product(_e4, _e5);
    let _e7: Rotor = self_1583;
    let _e8: Rotor = rotor_reversal(_e7);
    let _e9: MotorDual = motor_dual_rotor_geometric_product(_e6, _e8);
    let _e10: Plane = motor_dual_plane_into(_e9);
    return _e10;
}

fn rotor_point_geometric_quotient(self_1584: Rotor, other_1390: Point) -> Motor {
    var self_1585: Rotor;
    var other_1391: Point;

    self_1585 = self_1584;
    other_1391 = other_1390;
    let _e4: Rotor = self_1585;
    let _e5: Point = other_1391;
    let _e6: Point = point_inverse(_e5);
    let _e7: Motor = rotor_point_geometric_product(_e4, _e6);
    return _e7;
}

fn rotor_point_transformation(self_1586: Rotor, other_1392: Point) -> Point {
    var self_1587: Rotor;
    var other_1393: Point;

    self_1587 = self_1586;
    other_1393 = other_1392;
    let _e4: Rotor = self_1587;
    let _e5: Point = other_1393;
    let _e6: Motor = rotor_point_geometric_product(_e4, _e5);
    let _e7: Rotor = self_1587;
    let _e8: Rotor = rotor_reversal(_e7);
    let _e9: Motor = motor_rotor_geometric_product(_e6, _e8);
    let _e10: Point = motor_point_into(_e9);
    return _e10;
}

fn rotor_powi(self_1588: Rotor, exponent_4: i32) -> Rotor {
    var self_1589: Rotor;
    var exponent_5: i32;
    var local_2: Rotor;
    var x_2: Rotor;
    var y_2: Rotor;
    var n_2: i32;

    self_1589 = self_1588;
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
        let _e11: Rotor = self_1589;
        let _e12: Rotor = rotor_inverse(_e11);
        local_2 = _e12;
    } else {
        let _e14: Rotor = self_1589;
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

fn rotor_rotor_geometric_quotient(self_1590: Rotor, other_1394: Rotor) -> Rotor {
    var self_1591: Rotor;
    var other_1395: Rotor;

    self_1591 = self_1590;
    other_1395 = other_1394;
    let _e4: Rotor = self_1591;
    let _e5: Rotor = other_1395;
    let _e6: Rotor = rotor_inverse(_e5);
    let _e7: Rotor = rotor_rotor_geometric_product(_e4, _e6);
    return _e7;
}

fn rotor_rotor_transformation(self_1592: Rotor, other_1396: Rotor) -> Rotor {
    var self_1593: Rotor;
    var other_1397: Rotor;

    self_1593 = self_1592;
    other_1397 = other_1396;
    let _e4: Rotor = self_1593;
    let _e5: Rotor = other_1397;
    let _e6: Rotor = rotor_rotor_geometric_product(_e4, _e5);
    let _e7: Rotor = self_1593;
    let _e8: Rotor = rotor_reversal(_e7);
    let _e9: Rotor = rotor_rotor_geometric_product(_e6, _e8);
    return _e9;
}

fn rotor_scalar_geometric_quotient(self_1594: Rotor, other_1398: Scalar) -> Rotor {
    var self_1595: Rotor;
    var other_1399: Scalar;

    self_1595 = self_1594;
    other_1399 = other_1398;
    let _e4: Rotor = self_1595;
    let _e5: Scalar = other_1399;
    let _e6: Scalar = scalar_inverse(_e5);
    let _e7: Rotor = rotor_scalar_geometric_product(_e4, _e6);
    return _e7;
}

fn rotor_scalar_transformation(self_1596: Rotor, other_1400: Scalar) -> Scalar {
    var self_1597: Rotor;
    var other_1401: Scalar;

    self_1597 = self_1596;
    other_1401 = other_1400;
    let _e4: Rotor = self_1597;
    let _e5: Scalar = other_1401;
    let _e6: Rotor = rotor_scalar_geometric_product(_e4, _e5);
    let _e7: Rotor = self_1597;
    let _e8: Rotor = rotor_reversal(_e7);
    let _e9: Rotor = rotor_rotor_geometric_product(_e6, _e8);
    let _e10: Scalar = rotor_scalar_into(_e9);
    return _e10;
}

fn rotor_translator_geometric_quotient(self_1598: Rotor, other_1402: Translator) -> Motor {
    var self_1599: Rotor;
    var other_1403: Translator;

    self_1599 = self_1598;
    other_1403 = other_1402;
    let _e4: Rotor = self_1599;
    let _e5: Translator = other_1403;
    let _e6: Translator = translator_inverse(_e5);
    let _e7: Motor = rotor_translator_geometric_product(_e4, _e6);
    return _e7;
}

fn rotor_translator_transformation(self_1600: Rotor, other_1404: Translator) -> Translator {
    var self_1601: Rotor;
    var other_1405: Translator;

    self_1601 = self_1600;
    other_1405 = other_1404;
    let _e4: Rotor = self_1601;
    let _e5: Translator = other_1405;
    let _e6: Motor = rotor_translator_geometric_product(_e4, _e5);
    let _e7: Rotor = self_1601;
    let _e8: Rotor = rotor_reversal(_e7);
    let _e9: Motor = motor_rotor_geometric_product(_e6, _e8);
    let _e10: Translator = motor_translator_into(_e9);
    return _e10;
}

fn scalar_ideal_point_geometric_quotient(self_1602: Scalar, other_1406: IdealPoint) -> IdealPoint {
    var self_1603: Scalar;
    var other_1407: IdealPoint;

    self_1603 = self_1602;
    other_1407 = other_1406;
    let _e4: Scalar = self_1603;
    let _e5: IdealPoint = other_1407;
    let _e6: IdealPoint = ideal_point_inverse(_e5);
    let _e7: IdealPoint = scalar_ideal_point_geometric_product(_e4, _e6);
    return _e7;
}

fn scalar_ideal_point_transformation(self_1604: Scalar, other_1408: IdealPoint) -> IdealPoint {
    var self_1605: Scalar;
    var other_1409: IdealPoint;

    self_1605 = self_1604;
    other_1409 = other_1408;
    let _e4: Scalar = self_1605;
    let _e5: IdealPoint = other_1409;
    let _e6: IdealPoint = scalar_ideal_point_geometric_product(_e4, _e5);
    let _e7: Scalar = self_1605;
    let _e8: Scalar = scalar_reversal(_e7);
    let _e9: IdealPoint = ideal_point_scalar_geometric_product(_e6, _e8);
    return _e9;
}

fn scalar_motor_geometric_quotient(self_1606: Scalar, other_1410: Motor) -> Motor {
    var self_1607: Scalar;
    var other_1411: Motor;

    self_1607 = self_1606;
    other_1411 = other_1410;
    let _e4: Scalar = self_1607;
    let _e5: Motor = other_1411;
    let _e6: Motor = motor_inverse(_e5);
    let _e7: Motor = scalar_motor_geometric_product(_e4, _e6);
    return _e7;
}

fn scalar_motor_transformation(self_1608: Scalar, other_1412: Motor) -> Motor {
    var self_1609: Scalar;
    var other_1413: Motor;

    self_1609 = self_1608;
    other_1413 = other_1412;
    let _e4: Scalar = self_1609;
    let _e5: Motor = other_1413;
    let _e6: Motor = scalar_motor_geometric_product(_e4, _e5);
    let _e7: Scalar = self_1609;
    let _e8: Scalar = scalar_reversal(_e7);
    let _e9: Motor = motor_scalar_geometric_product(_e6, _e8);
    return _e9;
}

fn scalar_motor_dual_geometric_quotient(self_1610: Scalar, other_1414: MotorDual) -> MotorDual {
    var self_1611: Scalar;
    var other_1415: MotorDual;

    self_1611 = self_1610;
    other_1415 = other_1414;
    let _e4: Scalar = self_1611;
    let _e5: MotorDual = other_1415;
    let _e6: MotorDual = motor_dual_inverse(_e5);
    let _e7: MotorDual = scalar_motor_dual_geometric_product(_e4, _e6);
    return _e7;
}

fn scalar_motor_dual_transformation(self_1612: Scalar, other_1416: MotorDual) -> MotorDual {
    var self_1613: Scalar;
    var other_1417: MotorDual;

    self_1613 = self_1612;
    other_1417 = other_1416;
    let _e4: Scalar = self_1613;
    let _e5: MotorDual = other_1417;
    let _e6: MotorDual = scalar_motor_dual_geometric_product(_e4, _e5);
    let _e7: Scalar = self_1613;
    let _e8: Scalar = scalar_reversal(_e7);
    let _e9: MotorDual = motor_dual_scalar_geometric_product(_e6, _e8);
    return _e9;
}

fn scalar_multi_vector_geometric_quotient(self_1614: Scalar, other_1418: MultiVector) -> MultiVector {
    var self_1615: Scalar;
    var other_1419: MultiVector;

    self_1615 = self_1614;
    other_1419 = other_1418;
    let _e4: Scalar = self_1615;
    let _e5: MultiVector = other_1419;
    let _e6: MultiVector = multi_vector_inverse(_e5);
    let _e7: MultiVector = scalar_multi_vector_geometric_product(_e4, _e6);
    return _e7;
}

fn scalar_multi_vector_transformation(self_1616: Scalar, other_1420: MultiVector) -> MultiVector {
    var self_1617: Scalar;
    var other_1421: MultiVector;

    self_1617 = self_1616;
    other_1421 = other_1420;
    let _e4: Scalar = self_1617;
    let _e5: MultiVector = other_1421;
    let _e6: MultiVector = scalar_multi_vector_geometric_product(_e4, _e5);
    let _e7: Scalar = self_1617;
    let _e8: Scalar = scalar_reversal(_e7);
    let _e9: MultiVector = multi_vector_scalar_geometric_product(_e6, _e8);
    return _e9;
}

fn scalar_plane_geometric_quotient(self_1618: Scalar, other_1422: Plane) -> Plane {
    var self_1619: Scalar;
    var other_1423: Plane;

    self_1619 = self_1618;
    other_1423 = other_1422;
    let _e4: Scalar = self_1619;
    let _e5: Plane = other_1423;
    let _e6: Plane = plane_inverse(_e5);
    let _e7: Plane = scalar_plane_geometric_product(_e4, _e6);
    return _e7;
}

fn scalar_plane_transformation(self_1620: Scalar, other_1424: Plane) -> Plane {
    var self_1621: Scalar;
    var other_1425: Plane;

    self_1621 = self_1620;
    other_1425 = other_1424;
    let _e4: Scalar = self_1621;
    let _e5: Plane = other_1425;
    let _e6: Plane = scalar_plane_geometric_product(_e4, _e5);
    let _e7: Scalar = self_1621;
    let _e8: Scalar = scalar_reversal(_e7);
    let _e9: Plane = plane_scalar_geometric_product(_e6, _e8);
    return _e9;
}

fn scalar_point_geometric_quotient(self_1622: Scalar, other_1426: Point) -> Point {
    var self_1623: Scalar;
    var other_1427: Point;

    self_1623 = self_1622;
    other_1427 = other_1426;
    let _e4: Scalar = self_1623;
    let _e5: Point = other_1427;
    let _e6: Point = point_inverse(_e5);
    let _e7: Point = scalar_point_geometric_product(_e4, _e6);
    return _e7;
}

fn scalar_point_transformation(self_1624: Scalar, other_1428: Point) -> Point {
    var self_1625: Scalar;
    var other_1429: Point;

    self_1625 = self_1624;
    other_1429 = other_1428;
    let _e4: Scalar = self_1625;
    let _e5: Point = other_1429;
    let _e6: Point = scalar_point_geometric_product(_e4, _e5);
    let _e7: Scalar = self_1625;
    let _e8: Scalar = scalar_reversal(_e7);
    let _e9: Point = point_scalar_geometric_product(_e6, _e8);
    return _e9;
}

fn scalar_rotor_geometric_quotient(self_1626: Scalar, other_1430: Rotor) -> Rotor {
    var self_1627: Scalar;
    var other_1431: Rotor;

    self_1627 = self_1626;
    other_1431 = other_1430;
    let _e4: Scalar = self_1627;
    let _e5: Rotor = other_1431;
    let _e6: Rotor = rotor_inverse(_e5);
    let _e7: Rotor = scalar_rotor_geometric_product(_e4, _e6);
    return _e7;
}

fn scalar_rotor_transformation(self_1628: Scalar, other_1432: Rotor) -> Rotor {
    var self_1629: Scalar;
    var other_1433: Rotor;

    self_1629 = self_1628;
    other_1433 = other_1432;
    let _e4: Scalar = self_1629;
    let _e5: Rotor = other_1433;
    let _e6: Rotor = scalar_rotor_geometric_product(_e4, _e5);
    let _e7: Scalar = self_1629;
    let _e8: Scalar = scalar_reversal(_e7);
    let _e9: Rotor = rotor_scalar_geometric_product(_e6, _e8);
    return _e9;
}

fn scalar_powi(self_1630: Scalar, exponent_6: i32) -> Scalar {
    var self_1631: Scalar;
    var exponent_7: i32;
    var local_3: Scalar;
    var x_3: Scalar;
    var y_3: Scalar;
    var n_3: i32;

    self_1631 = self_1630;
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
        let _e11: Scalar = self_1631;
        let _e12: Scalar = scalar_inverse(_e11);
        local_3 = _e12;
    } else {
        let _e14: Scalar = self_1631;
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

fn scalar_scalar_geometric_quotient(self_1632: Scalar, other_1434: Scalar) -> Scalar {
    var self_1633: Scalar;
    var other_1435: Scalar;

    self_1633 = self_1632;
    other_1435 = other_1434;
    let _e4: Scalar = self_1633;
    let _e5: Scalar = other_1435;
    let _e6: Scalar = scalar_inverse(_e5);
    let _e7: Scalar = scalar_scalar_geometric_product(_e4, _e6);
    return _e7;
}

fn scalar_scalar_transformation(self_1634: Scalar, other_1436: Scalar) -> Scalar {
    var self_1635: Scalar;
    var other_1437: Scalar;

    self_1635 = self_1634;
    other_1437 = other_1436;
    let _e4: Scalar = self_1635;
    let _e5: Scalar = other_1437;
    let _e6: Scalar = scalar_scalar_geometric_product(_e4, _e5);
    let _e7: Scalar = self_1635;
    let _e8: Scalar = scalar_reversal(_e7);
    let _e9: Scalar = scalar_scalar_geometric_product(_e6, _e8);
    return _e9;
}

fn scalar_translator_geometric_quotient(self_1636: Scalar, other_1438: Translator) -> Translator {
    var self_1637: Scalar;
    var other_1439: Translator;

    self_1637 = self_1636;
    other_1439 = other_1438;
    let _e4: Scalar = self_1637;
    let _e5: Translator = other_1439;
    let _e6: Translator = translator_inverse(_e5);
    let _e7: Translator = scalar_translator_geometric_product(_e4, _e6);
    return _e7;
}

fn scalar_translator_transformation(self_1638: Scalar, other_1440: Translator) -> Translator {
    var self_1639: Scalar;
    var other_1441: Translator;

    self_1639 = self_1638;
    other_1441 = other_1440;
    let _e4: Scalar = self_1639;
    let _e5: Translator = other_1441;
    let _e6: Translator = scalar_translator_geometric_product(_e4, _e5);
    let _e7: Scalar = self_1639;
    let _e8: Scalar = scalar_reversal(_e7);
    let _e9: Translator = translator_scalar_geometric_product(_e6, _e8);
    return _e9;
}

fn translator_ideal_point_geometric_quotient(self_1640: Translator, other_1442: IdealPoint) -> Motor {
    var self_1641: Translator;
    var other_1443: IdealPoint;

    self_1641 = self_1640;
    other_1443 = other_1442;
    let _e4: Translator = self_1641;
    let _e5: IdealPoint = other_1443;
    let _e6: IdealPoint = ideal_point_inverse(_e5);
    let _e7: Motor = translator_ideal_point_geometric_product(_e4, _e6);
    return _e7;
}

fn translator_ideal_point_transformation(self_1642: Translator, other_1444: IdealPoint) -> IdealPoint {
    var self_1643: Translator;
    var other_1445: IdealPoint;

    self_1643 = self_1642;
    other_1445 = other_1444;
    let _e4: Translator = self_1643;
    let _e5: IdealPoint = other_1445;
    let _e6: Motor = translator_ideal_point_geometric_product(_e4, _e5);
    let _e7: Translator = self_1643;
    let _e8: Translator = translator_reversal(_e7);
    let _e9: Motor = motor_translator_geometric_product(_e6, _e8);
    let _e10: IdealPoint = motor_ideal_point_into(_e9);
    return _e10;
}

fn translator_motor_geometric_quotient(self_1644: Translator, other_1446: Motor) -> Motor {
    var self_1645: Translator;
    var other_1447: Motor;

    self_1645 = self_1644;
    other_1447 = other_1446;
    let _e4: Translator = self_1645;
    let _e5: Motor = other_1447;
    let _e6: Motor = motor_inverse(_e5);
    let _e7: Motor = translator_motor_geometric_product(_e4, _e6);
    return _e7;
}

fn translator_motor_transformation(self_1646: Translator, other_1448: Motor) -> Motor {
    var self_1647: Translator;
    var other_1449: Motor;

    self_1647 = self_1646;
    other_1449 = other_1448;
    let _e4: Translator = self_1647;
    let _e5: Motor = other_1449;
    let _e6: Motor = translator_motor_geometric_product(_e4, _e5);
    let _e7: Translator = self_1647;
    let _e8: Translator = translator_reversal(_e7);
    let _e9: Motor = motor_translator_geometric_product(_e6, _e8);
    return _e9;
}

fn translator_motor_dual_geometric_quotient(self_1648: Translator, other_1450: MotorDual) -> MotorDual {
    var self_1649: Translator;
    var other_1451: MotorDual;

    self_1649 = self_1648;
    other_1451 = other_1450;
    let _e4: Translator = self_1649;
    let _e5: MotorDual = other_1451;
    let _e6: MotorDual = motor_dual_inverse(_e5);
    let _e7: MotorDual = translator_motor_dual_geometric_product(_e4, _e6);
    return _e7;
}

fn translator_motor_dual_transformation(self_1650: Translator, other_1452: MotorDual) -> MotorDual {
    var self_1651: Translator;
    var other_1453: MotorDual;

    self_1651 = self_1650;
    other_1453 = other_1452;
    let _e4: Translator = self_1651;
    let _e5: MotorDual = other_1453;
    let _e6: MotorDual = translator_motor_dual_geometric_product(_e4, _e5);
    let _e7: Translator = self_1651;
    let _e8: Translator = translator_reversal(_e7);
    let _e9: MotorDual = motor_dual_translator_geometric_product(_e6, _e8);
    return _e9;
}

fn translator_multi_vector_geometric_quotient(self_1652: Translator, other_1454: MultiVector) -> MultiVector {
    var self_1653: Translator;
    var other_1455: MultiVector;

    self_1653 = self_1652;
    other_1455 = other_1454;
    let _e4: Translator = self_1653;
    let _e5: MultiVector = other_1455;
    let _e6: MultiVector = multi_vector_inverse(_e5);
    let _e7: MultiVector = translator_multi_vector_geometric_product(_e4, _e6);
    return _e7;
}

fn translator_multi_vector_transformation(self_1654: Translator, other_1456: MultiVector) -> MultiVector {
    var self_1655: Translator;
    var other_1457: MultiVector;

    self_1655 = self_1654;
    other_1457 = other_1456;
    let _e4: Translator = self_1655;
    let _e5: MultiVector = other_1457;
    let _e6: MultiVector = translator_multi_vector_geometric_product(_e4, _e5);
    let _e7: Translator = self_1655;
    let _e8: Translator = translator_reversal(_e7);
    let _e9: MultiVector = multi_vector_translator_geometric_product(_e6, _e8);
    return _e9;
}

fn translator_plane_geometric_quotient(self_1656: Translator, other_1458: Plane) -> MotorDual {
    var self_1657: Translator;
    var other_1459: Plane;

    self_1657 = self_1656;
    other_1459 = other_1458;
    let _e4: Translator = self_1657;
    let _e5: Plane = other_1459;
    let _e6: Plane = plane_inverse(_e5);
    let _e7: MotorDual = translator_plane_geometric_product(_e4, _e6);
    return _e7;
}

fn translator_plane_transformation(self_1658: Translator, other_1460: Plane) -> Plane {
    var self_1659: Translator;
    var other_1461: Plane;

    self_1659 = self_1658;
    other_1461 = other_1460;
    let _e4: Translator = self_1659;
    let _e5: Plane = other_1461;
    let _e6: MotorDual = translator_plane_geometric_product(_e4, _e5);
    let _e7: Translator = self_1659;
    let _e8: Translator = translator_reversal(_e7);
    let _e9: MotorDual = motor_dual_translator_geometric_product(_e6, _e8);
    let _e10: Plane = motor_dual_plane_into(_e9);
    return _e10;
}

fn translator_point_geometric_quotient(self_1660: Translator, other_1462: Point) -> Motor {
    var self_1661: Translator;
    var other_1463: Point;

    self_1661 = self_1660;
    other_1463 = other_1462;
    let _e4: Translator = self_1661;
    let _e5: Point = other_1463;
    let _e6: Point = point_inverse(_e5);
    let _e7: Motor = translator_point_geometric_product(_e4, _e6);
    return _e7;
}

fn translator_point_transformation(self_1662: Translator, other_1464: Point) -> Point {
    var self_1663: Translator;
    var other_1465: Point;

    self_1663 = self_1662;
    other_1465 = other_1464;
    let _e4: Translator = self_1663;
    let _e5: Point = other_1465;
    let _e6: Motor = translator_point_geometric_product(_e4, _e5);
    let _e7: Translator = self_1663;
    let _e8: Translator = translator_reversal(_e7);
    let _e9: Motor = motor_translator_geometric_product(_e6, _e8);
    let _e10: Point = motor_point_into(_e9);
    return _e10;
}

fn translator_rotor_geometric_quotient(self_1664: Translator, other_1466: Rotor) -> Motor {
    var self_1665: Translator;
    var other_1467: Rotor;

    self_1665 = self_1664;
    other_1467 = other_1466;
    let _e4: Translator = self_1665;
    let _e5: Rotor = other_1467;
    let _e6: Rotor = rotor_inverse(_e5);
    let _e7: Motor = translator_rotor_geometric_product(_e4, _e6);
    return _e7;
}

fn translator_rotor_transformation(self_1666: Translator, other_1468: Rotor) -> Rotor {
    var self_1667: Translator;
    var other_1469: Rotor;

    self_1667 = self_1666;
    other_1469 = other_1468;
    let _e4: Translator = self_1667;
    let _e5: Rotor = other_1469;
    let _e6: Motor = translator_rotor_geometric_product(_e4, _e5);
    let _e7: Translator = self_1667;
    let _e8: Translator = translator_reversal(_e7);
    let _e9: Motor = motor_translator_geometric_product(_e6, _e8);
    let _e10: Rotor = motor_rotor_into(_e9);
    return _e10;
}

fn translator_scalar_geometric_quotient(self_1668: Translator, other_1470: Scalar) -> Translator {
    var self_1669: Translator;
    var other_1471: Scalar;

    self_1669 = self_1668;
    other_1471 = other_1470;
    let _e4: Translator = self_1669;
    let _e5: Scalar = other_1471;
    let _e6: Scalar = scalar_inverse(_e5);
    let _e7: Translator = translator_scalar_geometric_product(_e4, _e6);
    return _e7;
}

fn translator_scalar_transformation(self_1670: Translator, other_1472: Scalar) -> Scalar {
    var self_1671: Translator;
    var other_1473: Scalar;

    self_1671 = self_1670;
    other_1473 = other_1472;
    let _e4: Translator = self_1671;
    let _e5: Scalar = other_1473;
    let _e6: Translator = translator_scalar_geometric_product(_e4, _e5);
    let _e7: Translator = self_1671;
    let _e8: Translator = translator_reversal(_e7);
    let _e9: Motor = translator_translator_geometric_product(_e6, _e8);
    let _e10: Scalar = motor_scalar_into(_e9);
    return _e10;
}

fn translator_translator_geometric_quotient(self_1672: Translator, other_1474: Translator) -> Motor {
    var self_1673: Translator;
    var other_1475: Translator;

    self_1673 = self_1672;
    other_1475 = other_1474;
    let _e4: Translator = self_1673;
    let _e5: Translator = other_1475;
    let _e6: Translator = translator_inverse(_e5);
    let _e7: Motor = translator_translator_geometric_product(_e4, _e6);
    return _e7;
}

fn translator_translator_transformation(self_1674: Translator, other_1476: Translator) -> Translator {
    var self_1675: Translator;
    var other_1477: Translator;

    self_1675 = self_1674;
    other_1477 = other_1476;
    let _e4: Translator = self_1675;
    let _e5: Translator = other_1477;
    let _e6: Motor = translator_translator_geometric_product(_e4, _e5);
    let _e7: Translator = self_1675;
    let _e8: Translator = translator_reversal(_e7);
    let _e9: Motor = motor_translator_geometric_product(_e6, _e8);
    let _e10: Translator = motor_translator_into(_e9);
    return _e10;
}


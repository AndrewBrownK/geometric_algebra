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

struct IdealPoint {
    g0_: vec2<f32>,
}

struct Plane {
    g0_: vec3<f32>,
}

struct Translator {
    g0_: vec3<f32>,
}

struct Motor {
    g0_: vec4<f32>,
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
    let _e4: Scalar = self_137;
    let _e5: Scalar = scalar_reversal(_e4);
    let _e6: Scalar = self_137;
    let _e8: Scalar = self_137;
    let _e9: Scalar = scalar_reversal(_e8);
    let _e10: Scalar = scalar_scalar_scalar_product(_e6, _e9);
    return _e10;
}

fn scalar_magnitude(self_138: Scalar) -> Scalar {
    var self_139: Scalar;

    self_139 = self_138;
    let _e3: Scalar = self_139;
    let _e4: Scalar = scalar_squared_magnitude(_e3);
    let _e7: Scalar = self_139;
    let _e8: Scalar = scalar_squared_magnitude(_e7);
    return Scalar(sqrt(_e8.g0_));
}

fn scalar_scale(self_140: Scalar, other_128: f32) -> Scalar {
    var self_141: Scalar;
    var other_129: f32;

    self_141 = self_140;
    other_129 = other_128;
    let _e5: f32 = other_129;
    let _e7: Scalar = self_141;
    let _e8: f32 = other_129;
    let _e10: Scalar = scalar_scalar_geometric_product(_e7, Scalar(_e8));
    return _e10;
}

fn scalar_signum(self_142: Scalar) -> Scalar {
    var self_143: Scalar;

    self_143 = self_142;
    let _e5: Scalar = self_143;
    let _e6: Scalar = scalar_magnitude(_e5);
    let _e10: Scalar = self_143;
    let _e13: Scalar = self_143;
    let _e14: Scalar = scalar_magnitude(_e13);
    let _e18: Scalar = scalar_scalar_geometric_product(_e10, Scalar((1.0 / _e14.g0_)));
    return _e18;
}

fn scalar_inverse(self_144: Scalar) -> Scalar {
    var self_145: Scalar;

    self_145 = self_144;
    let _e3: Scalar = self_145;
    let _e4: Scalar = scalar_reversal(_e3);
    let _e7: Scalar = self_145;
    let _e8: Scalar = scalar_squared_magnitude(_e7);
    let _e13: Scalar = self_145;
    let _e14: Scalar = scalar_reversal(_e13);
    let _e17: Scalar = self_145;
    let _e18: Scalar = scalar_squared_magnitude(_e17);
    let _e22: Scalar = scalar_scalar_geometric_product(_e14, Scalar((1.0 / _e18.g0_)));
    return _e22;
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
    let _e61: MultiVector = self_185;
    let _e65: MultiVector = other_157;
    let _e78: MultiVector = self_185;
    let _e82: MultiVector = other_157;
    let _e95: MultiVector = self_185;
    let _e99: MultiVector = other_157;
    let _e104: MultiVector = self_185;
    let _e108: MultiVector = other_157;
    let _e119: MultiVector = self_185;
    let _e123: MultiVector = other_157;
    let _e135: MultiVector = self_185;
    let _e139: MultiVector = other_157;
    let _e151: MultiVector = self_185;
    let _e155: MultiVector = other_157;
    return MultiVector(((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.y) * _e15.g0_.yxwz) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + (vec4<f32>(_e28.g0_.z) * _e32.g0_.zwxy)) + ((vec4<f32>(_e37.g0_.w) * _e41.g0_.wzyx) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0))), ((((((((vec4<f32>(_e54.g0_.x) * _e58.g1_) + ((vec4<f32>(_e61.g0_.y) * _e65.g1_.yxwz) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))) + ((vec4<f32>(_e78.g0_.z) * _e82.g1_.zwxy) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))) + (vec4<f32>(_e95.g0_.w) * _e99.g1_.wzyx)) + ((vec4<f32>(_e104.g1_.x) * _e108.g0_) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e119.g1_.y) * _e123.g0_.yxwz) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0))) + ((vec4<f32>(_e135.g1_.z) * _e139.g0_.zwxy) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e151.g1_.w) * _e155.g0_.wzyx) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0))));
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
    let _e47: MultiVector = other_163;
    let _e58: MultiVector = self_191;
    let _e62: MultiVector = other_163;
    let _e65: MultiVector = self_191;
    let _e69: MultiVector = other_163;
    let _e81: MultiVector = self_191;
    let _e85: MultiVector = other_163;
    let _e96: MultiVector = self_191;
    let _e100: MultiVector = other_163;
    let _e112: MultiVector = self_191;
    let _e116: MultiVector = other_163;
    let _e128: MultiVector = self_191;
    let _e132: MultiVector = other_163;
    let _e143: MultiVector = self_191;
    let _e147: MultiVector = other_163;
    let _e159: MultiVector = self_191;
    let _e162: MultiVector = other_163;
    return MultiVector(((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.y) * _e15.g0_.yxwz) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e28.g0_.w) * _e32.g0_.wwyx) * vec4<f32>(1.0, 0.0, -(1.0), 1.0))) + ((_e44.g0_.zxzz * _e47.g0_.zxxy) * vec4<f32>(1.0, 0.0, 1.0, 1.0))), ((((((((vec4<f32>(_e58.g0_.x) * _e62.g1_) + ((vec4<f32>(_e65.g0_.z) * _e69.g1_.zzzy) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))) + ((vec4<f32>(_e81.g0_.w) * _e85.g1_.wwyw) * vec4<f32>(1.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e96.g1_.x) * vec4<f32>(_e100.g0_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e112.g1_.y) * _e116.g0_.yxwz) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0))) + ((vec4<f32>(_e128.g1_.z) * _e132.g0_.zzxz) * vec4<f32>(1.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e143.g1_.w) * _e147.g0_.wwwx) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))) + ((_e159.g0_.yxxx * _e162.g1_.yxxx) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))));
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
    let _e45: MultiVector = other_165;
    let _e57: MultiVector = self_193;
    let _e61: MultiVector = other_165;
    let _e64: MultiVector = self_193;
    let _e68: MultiVector = other_165;
    let _e80: MultiVector = self_193;
    let _e84: MultiVector = other_165;
    let _e95: MultiVector = self_193;
    let _e98: MultiVector = other_165;
    return MultiVector(((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.z) * _e15.g0_.zzzy) * vec4<f32>(1.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e26.g0_.w) * _e30.g0_.wwyw) * vec4<f32>(1.0, 0.0, -(1.0), 0.0))) + ((_e42.g0_.yxxx * _e45.g0_.yxxx) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))), ((((vec4<f32>(_e57.g0_.x) * _e61.g1_) + ((vec4<f32>(_e64.g0_.z) * _e68.g1_.zzzy) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))) + ((vec4<f32>(_e80.g0_.w) * _e84.g1_.wwyw) * vec4<f32>(1.0, 0.0, 1.0, 0.0))) + ((_e95.g0_.yxxx * _e98.g1_.yxxx) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))));
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
    let _e81: MultiVector = self_195;
    let _e85: MultiVector = other_167;
    let _e96: MultiVector = self_195;
    let _e100: MultiVector = other_167;
    let _e112: MultiVector = self_195;
    let _e116: MultiVector = other_167;
    return MultiVector((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.yxwz) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0))) + ((vec4<f32>(_e20.g0_.z) * _e24.g0_.zzxz) * vec4<f32>(1.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e35.g0_.w) * _e39.g0_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e50.g0_.x) * vec4<f32>(_e54.g0_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))), (((((vec4<f32>(_e66.g1_.y) * _e70.g0_.yxwz) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0)) + ((vec4<f32>(_e81.g1_.z) * _e85.g0_.zzxz) * vec4<f32>(1.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e96.g1_.w) * _e100.g0_.wwwx) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))) + ((vec4<f32>(_e112.g1_.x) * vec4<f32>(_e116.g0_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))));
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
    return Scalar(((((_e4.g0_.x * _e7.g0_.x) - (_e11.g0_.y * _e14.g0_.y)) + (_e19.g0_.z * _e22.g0_.z)) + (_e27.g0_.w * _e30.g0_.w)));
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
    let _e7: Point = other_189;
    let _e20: MultiVector = self_221;
    let _e24: Point = other_189;
    let _e27: Point = other_189;
    let _e30: Point = other_189;
    let _e33: Point = other_189;
    let _e45: MultiVector = self_221;
    let _e49: Point = other_189;
    let _e52: Point = other_189;
    let _e55: Point = other_189;
    let _e58: Point = other_189;
    let _e70: MultiVector = self_221;
    let _e74: Point = other_189;
    let _e86: MultiVector = self_221;
    let _e90: Point = other_189;
    let _e103: MultiVector = self_221;
    let _e107: Point = other_189;
    let _e120: MultiVector = self_221;
    let _e124: Point = other_189;
    let _e136: MultiVector = self_221;
    let _e139: Point = other_189;
    let _e142: Point = other_189;
    let _e145: Point = other_189;
    let _e148: Point = other_189;
    return MultiVector(((_e4.g0_.yxwz * vec4<f32>(_e7.g0_.x)) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0)), ((((((((vec4<f32>(_e20.g0_.y) * vec4<f32>(_e24.g0_.z, _e27.g0_.z, _e30.g0_.z, _e33.g0_.y)) * vec4<f32>(0.0, 0.0, -(1.0), 1.0)) + ((vec4<f32>(_e45.g0_.w) * vec4<f32>(_e49.g0_.z, _e52.g0_.y, _e55.g0_.z, _e58.g0_.z)) * vec4<f32>(1.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e70.g1_.x) * vec4<f32>(_e74.g0_.x)) * vec4<f32>(0.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e86.g1_.y) * vec4<f32>(_e90.g0_.x)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((vec4<f32>(_e103.g1_.z) * vec4<f32>(_e107.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e120.g1_.w) * vec4<f32>(_e124.g0_.x)) * vec4<f32>(0.0, 0.0, 1.0, 0.0))) + ((_e136.g0_.zzxx * vec4<f32>(_e139.g0_.y, _e142.g0_.z, _e145.g0_.y, _e148.g0_.z)) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0))));
}

fn multi_vector_point_scalar_product(self_222: MultiVector, other_190: Point) -> Scalar {
    var self_223: MultiVector;
    var other_191: Point;

    self_223 = self_222;
    other_191 = other_190;
    let _e5: MultiVector = self_223;
    let _e8: Point = other_191;
    return Scalar((0.0 - (_e5.g0_.y * _e8.g0_.x)));
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

fn multi_vector_plane_into(self_230: MultiVector) -> Plane {
    var self_231: MultiVector;

    self_231 = self_230;
    let _e2: MultiVector = self_231;
    let _e5: MultiVector = self_231;
    let _e8: MultiVector = self_231;
    return Plane(vec3<f32>(_e2.g1_.x, _e5.g0_.w, _e8.g0_.z));
}

fn multi_vector_plane_add(self_232: MultiVector, other_196: Plane) -> MultiVector {
    var self_233: MultiVector;
    var other_197: Plane;

    self_233 = self_232;
    other_197 = other_196;
    let _e4: MultiVector = self_233;
    let _e6: Plane = other_197;
    let _e9: Plane = other_197;
    let _e12: Plane = other_197;
    let _e15: Plane = other_197;
    let _e26: MultiVector = self_233;
    let _e28: Plane = other_197;
    return MultiVector((_e4.g0_ + (vec4<f32>(_e6.g0_.x, _e9.g0_.x, _e12.g0_.z, _e15.g0_.y) * vec4<f32>(0.0, 0.0, 1.0, 1.0))), (_e26.g1_ + (vec4<f32>(_e28.g0_.x) * vec4<f32>(1.0, 0.0, 0.0, 0.0))));
}

fn multi_vector_plane_sub(self_234: MultiVector, other_198: Plane) -> MultiVector {
    var self_235: MultiVector;
    var other_199: Plane;

    self_235 = self_234;
    other_199 = other_198;
    let _e4: MultiVector = self_235;
    let _e6: Plane = other_199;
    let _e9: Plane = other_199;
    let _e12: Plane = other_199;
    let _e15: Plane = other_199;
    let _e26: MultiVector = self_235;
    let _e28: Plane = other_199;
    return MultiVector((_e4.g0_ - (vec4<f32>(_e6.g0_.x, _e9.g0_.x, _e12.g0_.z, _e15.g0_.y) * vec4<f32>(0.0, 0.0, 1.0, 1.0))), (_e26.g1_ - (vec4<f32>(_e28.g0_.x) * vec4<f32>(1.0, 0.0, 0.0, 0.0))));
}

fn multi_vector_plane_geometric_product(self_236: MultiVector, other_200: Plane) -> MultiVector {
    var self_237: MultiVector;
    var other_201: Plane;

    self_237 = self_236;
    other_201 = other_200;
    let _e4: MultiVector = self_237;
    let _e8: Plane = other_201;
    let _e11: Plane = other_201;
    let _e14: Plane = other_201;
    let _e17: Plane = other_201;
    let _e29: MultiVector = self_237;
    let _e33: Plane = other_201;
    let _e36: Plane = other_201;
    let _e39: Plane = other_201;
    let _e42: Plane = other_201;
    let _e55: MultiVector = self_237;
    let _e58: Plane = other_201;
    let _e61: Plane = other_201;
    let _e64: Plane = other_201;
    let _e67: Plane = other_201;
    let _e73: MultiVector = self_237;
    let _e77: Plane = other_201;
    let _e80: Plane = other_201;
    let _e83: Plane = other_201;
    let _e86: Plane = other_201;
    let _e98: MultiVector = self_237;
    let _e102: Plane = other_201;
    let _e105: Plane = other_201;
    let _e108: Plane = other_201;
    let _e111: Plane = other_201;
    let _e123: MultiVector = self_237;
    let _e127: Plane = other_201;
    let _e130: Plane = other_201;
    let _e133: Plane = other_201;
    let _e136: Plane = other_201;
    let _e148: MultiVector = self_237;
    let _e152: Plane = other_201;
    let _e155: Plane = other_201;
    let _e158: Plane = other_201;
    let _e161: Plane = other_201;
    let _e174: MultiVector = self_237;
    let _e176: Plane = other_201;
    return MultiVector(((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y, _e11.g0_.y, _e14.g0_.y, _e17.g0_.z)) * vec4<f32>(0.0, 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e29.g0_.w) * vec4<f32>(_e33.g0_.y, _e36.g0_.z, _e39.g0_.y, _e42.g0_.y)) * vec4<f32>(1.0, -(1.0), 0.0, 0.0))) + (_e55.g0_.zzxx * vec4<f32>(_e58.g0_.z, _e61.g0_.y, _e64.g0_.z, _e67.g0_.y))), ((((((vec4<f32>(_e73.g1_.x) * vec4<f32>(_e77.g0_.z, _e80.g0_.z, _e83.g0_.z, _e86.g0_.y)) * vec4<f32>(0.0, 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e98.g1_.y) * vec4<f32>(_e102.g0_.y, _e105.g0_.y, _e108.g0_.y, _e111.g0_.z)) * vec4<f32>(0.0, 0.0, 1.0, 1.0))) + ((vec4<f32>(_e123.g1_.z) * vec4<f32>(_e127.g0_.z, _e130.g0_.y, _e133.g0_.z, _e136.g0_.z)) * vec4<f32>(1.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e148.g1_.w) * vec4<f32>(_e152.g0_.y, _e155.g0_.z, _e158.g0_.y, _e161.g0_.y)) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))) + ((_e174.g0_ * vec4<f32>(_e176.g0_.x)) * vec4<f32>(1.0, 1.0, -(1.0), 1.0))));
}

fn multi_vector_plane_scalar_product(self_238: MultiVector, other_202: Plane) -> Scalar {
    var self_239: MultiVector;
    var other_203: Plane;

    self_239 = self_238;
    other_203 = other_202;
    let _e4: MultiVector = self_239;
    let _e7: Plane = other_203;
    let _e11: MultiVector = self_239;
    let _e14: Plane = other_203;
    return Scalar(((_e4.g0_.z * _e7.g0_.z) + (_e11.g0_.w * _e14.g0_.y)));
}

fn multi_vector_translator_into(self_240: MultiVector) -> Translator {
    var self_241: MultiVector;

    self_241 = self_240;
    let _e2: MultiVector = self_241;
    let _e5: MultiVector = self_241;
    let _e8: MultiVector = self_241;
    return Translator(vec3<f32>(_e2.g0_.x, _e5.g1_.z, _e8.g1_.w));
}

fn multi_vector_translator_add(self_242: MultiVector, other_204: Translator) -> MultiVector {
    var self_243: MultiVector;
    var other_205: Translator;

    self_243 = self_242;
    other_205 = other_204;
    let _e4: MultiVector = self_243;
    let _e6: Translator = other_205;
    let _e17: MultiVector = self_243;
    let _e19: Translator = other_205;
    let _e22: Translator = other_205;
    let _e25: Translator = other_205;
    let _e28: Translator = other_205;
    return MultiVector((_e4.g0_ + (vec4<f32>(_e6.g0_.x) * vec4<f32>(1.0, 0.0, 0.0, 0.0))), (_e17.g1_ + (vec4<f32>(_e19.g0_.x, _e22.g0_.x, _e25.g0_.y, _e28.g0_.z) * vec4<f32>(0.0, 0.0, 1.0, 1.0))));
}

fn multi_vector_translator_sub(self_244: MultiVector, other_206: Translator) -> MultiVector {
    var self_245: MultiVector;
    var other_207: Translator;

    self_245 = self_244;
    other_207 = other_206;
    let _e4: MultiVector = self_245;
    let _e6: Translator = other_207;
    let _e17: MultiVector = self_245;
    let _e19: Translator = other_207;
    let _e22: Translator = other_207;
    let _e25: Translator = other_207;
    let _e28: Translator = other_207;
    return MultiVector((_e4.g0_ - (vec4<f32>(_e6.g0_.x) * vec4<f32>(1.0, 0.0, 0.0, 0.0))), (_e17.g1_ - (vec4<f32>(_e19.g0_.x, _e22.g0_.x, _e25.g0_.y, _e28.g0_.z) * vec4<f32>(0.0, 0.0, 1.0, 1.0))));
}

fn multi_vector_translator_geometric_product(self_246: MultiVector, other_208: Translator) -> MultiVector {
    var self_247: MultiVector;
    var other_209: Translator;

    self_247 = self_246;
    other_209 = other_208;
    let _e4: MultiVector = self_247;
    let _e6: Translator = other_209;
    let _e11: MultiVector = self_247;
    let _e15: Translator = other_209;
    let _e18: Translator = other_209;
    let _e21: Translator = other_209;
    let _e24: Translator = other_209;
    let _e36: MultiVector = self_247;
    let _e40: Translator = other_209;
    let _e43: Translator = other_209;
    let _e46: Translator = other_209;
    let _e49: Translator = other_209;
    let _e61: MultiVector = self_247;
    let _e65: Translator = other_209;
    let _e77: MultiVector = self_247;
    let _e81: Translator = other_209;
    let _e93: MultiVector = self_247;
    let _e97: Translator = other_209;
    let _e109: MultiVector = self_247;
    let _e113: Translator = other_209;
    let _e125: MultiVector = self_247;
    let _e128: Translator = other_209;
    let _e131: Translator = other_209;
    let _e134: Translator = other_209;
    let _e137: Translator = other_209;
    return MultiVector((_e4.g0_ * vec4<f32>(_e6.g0_.x)), ((((((((vec4<f32>(_e11.g0_.y) * vec4<f32>(_e15.g0_.z, _e18.g0_.z, _e21.g0_.z, _e24.g0_.y)) * vec4<f32>(0.0, 0.0, -(1.0), 1.0)) + ((vec4<f32>(_e36.g0_.w) * vec4<f32>(_e40.g0_.z, _e43.g0_.y, _e46.g0_.z, _e49.g0_.z)) * vec4<f32>(1.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e61.g1_.x) * vec4<f32>(_e65.g0_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e77.g1_.y) * vec4<f32>(_e81.g0_.x)) * vec4<f32>(0.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e93.g1_.z) * vec4<f32>(_e97.g0_.x)) * vec4<f32>(0.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e109.g1_.w) * vec4<f32>(_e113.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((_e125.g0_.zzxx * vec4<f32>(_e128.g0_.y, _e131.g0_.z, _e134.g0_.y, _e137.g0_.z)) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0))));
}

fn multi_vector_translator_outer_product(self_248: MultiVector, other_210: Translator) -> MultiVector {
    var self_249: MultiVector;
    var other_211: Translator;

    self_249 = self_248;
    other_211 = other_210;
    let _e4: MultiVector = self_249;
    let _e6: Translator = other_211;
    let _e11: MultiVector = self_249;
    let _e15: Translator = other_211;
    let _e26: MultiVector = self_249;
    let _e30: Translator = other_211;
    let _e42: MultiVector = self_249;
    let _e46: Translator = other_211;
    let _e58: MultiVector = self_249;
    let _e62: Translator = other_211;
    let _e74: MultiVector = self_249;
    let _e77: MultiVector = self_249;
    let _e80: MultiVector = self_249;
    let _e83: MultiVector = self_249;
    let _e87: Translator = other_211;
    let _e90: Translator = other_211;
    let _e93: Translator = other_211;
    let _e96: Translator = other_211;
    return MultiVector((_e4.g0_ * vec4<f32>(_e6.g0_.x)), ((((((vec4<f32>(_e11.g0_.w) * vec4<f32>(_e15.g0_.y)) * vec4<f32>(0.0, 1.0, 0.0, 0.0)) + ((vec4<f32>(_e26.g1_.y) * vec4<f32>(_e30.g0_.x)) * vec4<f32>(0.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e42.g1_.z) * vec4<f32>(_e46.g0_.x)) * vec4<f32>(0.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e58.g1_.w) * vec4<f32>(_e62.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + (vec4<f32>(_e74.g1_.x, _e77.g0_.z, _e80.g0_.x, _e83.g0_.x) * vec4<f32>(_e87.g0_.x, _e90.g0_.z, _e93.g0_.y, _e96.g0_.z))));
}

fn multi_vector_translator_inner_product(self_250: MultiVector, other_212: Translator) -> MultiVector {
    var self_251: MultiVector;
    var other_213: Translator;

    self_251 = self_250;
    other_213 = other_212;
    let _e4: MultiVector = self_251;
    let _e6: Translator = other_213;
    let _e11: MultiVector = self_251;
    let _e15: Translator = other_213;
    let _e26: MultiVector = self_251;
    let _e30: Translator = other_213;
    let _e42: MultiVector = self_251;
    let _e46: Translator = other_213;
    let _e58: MultiVector = self_251;
    let _e62: Translator = other_213;
    let _e74: MultiVector = self_251;
    let _e77: MultiVector = self_251;
    let _e80: MultiVector = self_251;
    let _e83: MultiVector = self_251;
    let _e87: Translator = other_213;
    let _e90: Translator = other_213;
    let _e93: Translator = other_213;
    let _e96: Translator = other_213;
    return MultiVector((_e4.g0_ * vec4<f32>(_e6.g0_.x)), ((((((vec4<f32>(_e11.g0_.w) * vec4<f32>(_e15.g0_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) + ((vec4<f32>(_e26.g1_.x) * vec4<f32>(_e30.g0_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e42.g1_.z) * vec4<f32>(_e46.g0_.x)) * vec4<f32>(0.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e58.g1_.w) * vec4<f32>(_e62.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e74.g0_.z, _e77.g1_.y, _e80.g0_.x, _e83.g0_.x) * vec4<f32>(_e87.g0_.y, _e90.g0_.x, _e93.g0_.y, _e96.g0_.z)) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0))));
}

fn multi_vector_translator_right_contraction(self_252: MultiVector, other_214: Translator) -> MultiVector {
    var self_253: MultiVector;
    var other_215: Translator;

    self_253 = self_252;
    other_215 = other_214;
    let _e4: MultiVector = self_253;
    let _e6: Translator = other_215;
    let _e11: MultiVector = self_253;
    let _e13: Translator = other_215;
    return MultiVector((_e4.g0_ * vec4<f32>(_e6.g0_.x)), (_e11.g1_ * vec4<f32>(_e13.g0_.x)));
}

fn multi_vector_translator_scalar_product(self_254: MultiVector, other_216: Translator) -> Scalar {
    var self_255: MultiVector;
    var other_217: Translator;

    self_255 = self_254;
    other_217 = other_216;
    let _e4: MultiVector = self_255;
    let _e7: Translator = other_217;
    return Scalar((_e4.g0_.x * _e7.g0_.x));
}

fn multi_vector_motor_into(self_256: MultiVector) -> Motor {
    var self_257: MultiVector;

    self_257 = self_256;
    let _e2: MultiVector = self_257;
    let _e5: MultiVector = self_257;
    let _e8: MultiVector = self_257;
    let _e11: MultiVector = self_257;
    return Motor(vec4<f32>(_e2.g0_.x, _e5.g0_.y, _e8.g1_.z, _e11.g1_.w));
}

fn multi_vector_motor_add(self_258: MultiVector, other_218: Motor) -> MultiVector {
    var self_259: MultiVector;
    var other_219: Motor;

    self_259 = self_258;
    other_219 = other_218;
    let _e4: MultiVector = self_259;
    let _e6: Motor = other_219;
    let _e16: MultiVector = self_259;
    let _e18: Motor = other_219;
    return MultiVector((_e4.g0_ + (_e6.g0_.xyxx * vec4<f32>(1.0, 1.0, 0.0, 0.0))), (_e16.g1_ + (_e18.g0_.xxzw * vec4<f32>(0.0, 0.0, 1.0, 1.0))));
}

fn multi_vector_motor_sub(self_260: MultiVector, other_220: Motor) -> MultiVector {
    var self_261: MultiVector;
    var other_221: Motor;

    self_261 = self_260;
    other_221 = other_220;
    let _e4: MultiVector = self_261;
    let _e6: Motor = other_221;
    let _e16: MultiVector = self_261;
    let _e18: Motor = other_221;
    return MultiVector((_e4.g0_ - (_e6.g0_.xyxx * vec4<f32>(1.0, 1.0, 0.0, 0.0))), (_e16.g1_ - (_e18.g0_.xxzw * vec4<f32>(0.0, 0.0, 1.0, 1.0))));
}

fn multi_vector_motor_geometric_product(self_262: MultiVector, other_222: Motor) -> MultiVector {
    var self_263: MultiVector;
    var other_223: Motor;

    self_263 = self_262;
    other_223 = other_222;
    let _e4: MultiVector = self_263;
    let _e8: Motor = other_223;
    let _e19: MultiVector = self_263;
    let _e23: Motor = other_223;
    let _e35: MultiVector = self_263;
    let _e38: Motor = other_223;
    let _e43: MultiVector = self_263;
    let _e47: Motor = other_223;
    let _e58: MultiVector = self_263;
    let _e62: Motor = other_223;
    let _e73: MultiVector = self_263;
    let _e77: Motor = other_223;
    let _e88: MultiVector = self_263;
    let _e92: Motor = other_223;
    let _e104: MultiVector = self_263;
    let _e108: Motor = other_223;
    let _e120: MultiVector = self_263;
    let _e124: Motor = other_223;
    let _e135: MultiVector = self_263;
    let _e138: Motor = other_223;
    return MultiVector(((((vec4<f32>(_e4.g0_.y) * _e8.g0_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0)) + ((vec4<f32>(_e19.g0_.w) * _e23.g0_.yyyx) * vec4<f32>(0.0, 0.0, -(1.0), 1.0))) + (_e35.g0_.xxzz * _e38.g0_.xyxy)), ((((((((vec4<f32>(_e43.g0_.y) * _e47.g0_.wwwz) * vec4<f32>(0.0, 0.0, -(1.0), 1.0)) + ((vec4<f32>(_e58.g0_.w) * _e62.g0_.wzww) * vec4<f32>(1.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e73.g1_.x) * _e77.g0_.xyxx) * vec4<f32>(1.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e88.g1_.y) * _e92.g0_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))) + ((vec4<f32>(_e104.g1_.z) * _e108.g0_.xxxy) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e120.g1_.w) * _e124.g0_.yyyx) * vec4<f32>(0.0, 0.0, 1.0, 1.0))) + ((_e135.g0_.zzxx * _e138.g0_.zwzw) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0))));
}

fn multi_vector_motor_outer_product(self_264: MultiVector, other_224: Motor) -> MultiVector {
    var self_265: MultiVector;
    var other_225: Motor;

    self_265 = self_264;
    other_225 = other_224;
    let _e4: MultiVector = self_265;
    let _e8: Motor = other_225;
    let _e19: MultiVector = self_265;
    let _e22: Motor = other_225;
    let _e27: MultiVector = self_265;
    let _e31: Motor = other_225;
    let _e42: MultiVector = self_265;
    let _e46: Motor = other_225;
    let _e57: MultiVector = self_265;
    let _e61: Motor = other_225;
    let _e73: MultiVector = self_265;
    let _e77: Motor = other_225;
    let _e89: MultiVector = self_265;
    let _e93: Motor = other_225;
    let _e105: MultiVector = self_265;
    let _e108: Motor = other_225;
    return MultiVector((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.x)) * vec4<f32>(0.0, 1.0, 0.0, 0.0)) + (_e19.g0_.xxzw * _e22.g0_.xyxx)), (((((((vec4<f32>(_e27.g0_.w) * vec4<f32>(_e31.g0_.z)) * vec4<f32>(0.0, 1.0, 0.0, 0.0)) + ((vec4<f32>(_e42.g1_.x) * _e46.g0_.xyxx) * vec4<f32>(1.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e57.g1_.y) * vec4<f32>(_e61.g0_.x)) * vec4<f32>(0.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e73.g1_.z) * vec4<f32>(_e77.g0_.x)) * vec4<f32>(0.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e89.g1_.w) * vec4<f32>(_e93.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((_e105.g0_.xzxx * _e108.g0_.xwzw) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn multi_vector_motor_inner_product(self_266: MultiVector, other_226: Motor) -> MultiVector {
    var self_267: MultiVector;
    var other_227: Motor;

    self_267 = self_266;
    other_227 = other_226;
    let _e4: MultiVector = self_267;
    let _e8: Motor = other_227;
    let _e19: MultiVector = self_267;
    let _e23: Motor = other_227;
    let _e35: MultiVector = self_267;
    let _e38: Motor = other_227;
    let _e43: MultiVector = self_267;
    let _e47: Motor = other_227;
    let _e58: MultiVector = self_267;
    let _e62: Motor = other_227;
    let _e74: MultiVector = self_267;
    let _e78: Motor = other_227;
    let _e90: MultiVector = self_267;
    let _e94: Motor = other_227;
    let _e106: MultiVector = self_267;
    let _e110: Motor = other_227;
    let _e122: MultiVector = self_267;
    let _e125: Motor = other_227;
    return MultiVector(((((vec4<f32>(_e4.g0_.y) * _e8.g0_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0)) + ((vec4<f32>(_e19.g0_.w) * _e23.g0_.yyyx) * vec4<f32>(0.0, 0.0, -(1.0), 1.0))) + (_e35.g0_.xxzz * _e38.g0_.xyxy)), (((((((vec4<f32>(_e43.g0_.w) * vec4<f32>(_e47.g0_.w)) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) + ((vec4<f32>(_e58.g1_.x) * vec4<f32>(_e62.g0_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e74.g1_.y) * _e78.g0_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))) + ((vec4<f32>(_e90.g1_.z) * vec4<f32>(_e94.g0_.x)) * vec4<f32>(0.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e106.g1_.w) * vec4<f32>(_e110.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((_e122.g0_.zxxx * _e125.g0_.zxzw) * vec4<f32>(-(1.0), 0.0, 1.0, 1.0))));
}

fn multi_vector_motor_right_contraction(self_268: MultiVector, other_228: Motor) -> MultiVector {
    var self_269: MultiVector;
    var other_229: Motor;

    self_269 = self_268;
    other_229 = other_228;
    let _e4: MultiVector = self_269;
    let _e8: Motor = other_229;
    let _e19: MultiVector = self_269;
    let _e22: Motor = other_229;
    let _e34: MultiVector = self_269;
    let _e38: Motor = other_229;
    let _e49: MultiVector = self_269;
    let _e52: Motor = other_229;
    return MultiVector((((vec4<f32>(_e4.g0_.y) * _e8.g0_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0)) + ((_e19.g0_.xxzw * vec4<f32>(_e22.g0_.x)) * vec4<f32>(1.0, 0.0, 1.0, 1.0))), (((vec4<f32>(_e34.g1_.y) * _e38.g0_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0)) + ((_e49.g1_.xxzw * vec4<f32>(_e52.g0_.x)) * vec4<f32>(1.0, 0.0, 1.0, 1.0))));
}

fn multi_vector_motor_scalar_product(self_270: MultiVector, other_230: Motor) -> Scalar {
    var self_271: MultiVector;
    var other_231: Motor;

    self_271 = self_270;
    other_231 = other_230;
    let _e4: MultiVector = self_271;
    let _e7: Motor = other_231;
    let _e11: MultiVector = self_271;
    let _e14: Motor = other_231;
    return Scalar(((_e4.g0_.x * _e7.g0_.x) - (_e11.g0_.y * _e14.g0_.y)));
}

fn multi_vector_motor_dual_into(self_272: MultiVector) -> MotorDual {
    var self_273: MultiVector;

    self_273 = self_272;
    let _e2: MultiVector = self_273;
    let _e5: MultiVector = self_273;
    let _e8: MultiVector = self_273;
    let _e11: MultiVector = self_273;
    return MotorDual(vec4<f32>(_e2.g1_.y, _e5.g1_.x, _e8.g0_.w, _e11.g0_.z));
}

fn multi_vector_motor_dual_add(self_274: MultiVector, other_232: MotorDual) -> MultiVector {
    var self_275: MultiVector;
    var other_233: MotorDual;

    self_275 = self_274;
    other_233 = other_232;
    let _e4: MultiVector = self_275;
    let _e6: MotorDual = other_233;
    let _e16: MultiVector = self_275;
    let _e18: MotorDual = other_233;
    return MultiVector((_e4.g0_ + (_e6.g0_.xxwz * vec4<f32>(0.0, 0.0, 1.0, 1.0))), (_e16.g1_ + (_e18.g0_.yxxx * vec4<f32>(1.0, 1.0, 0.0, 0.0))));
}

fn multi_vector_motor_dual_sub(self_276: MultiVector, other_234: MotorDual) -> MultiVector {
    var self_277: MultiVector;
    var other_235: MotorDual;

    self_277 = self_276;
    other_235 = other_234;
    let _e4: MultiVector = self_277;
    let _e6: MotorDual = other_235;
    let _e16: MultiVector = self_277;
    let _e18: MotorDual = other_235;
    return MultiVector((_e4.g0_ - (_e6.g0_.xxwz * vec4<f32>(0.0, 0.0, 1.0, 1.0))), (_e16.g1_ - (_e18.g0_.yxxx * vec4<f32>(1.0, 1.0, 0.0, 0.0))));
}

fn multi_vector_motor_dual_geometric_product(self_278: MultiVector, other_236: MotorDual) -> MultiVector {
    var self_279: MultiVector;
    var other_237: MotorDual;

    self_279 = self_278;
    other_237 = other_236;
    let _e4: MultiVector = self_279;
    let _e8: MotorDual = other_237;
    let _e19: MultiVector = self_279;
    let _e23: MotorDual = other_237;
    let _e35: MultiVector = self_279;
    let _e38: MotorDual = other_237;
    let _e43: MultiVector = self_279;
    let _e47: MotorDual = other_237;
    let _e58: MultiVector = self_279;
    let _e62: MotorDual = other_237;
    let _e73: MultiVector = self_279;
    let _e77: MotorDual = other_237;
    let _e89: MultiVector = self_279;
    let _e93: MotorDual = other_237;
    let _e104: MultiVector = self_279;
    let _e108: MotorDual = other_237;
    let _e119: MultiVector = self_279;
    let _e123: MotorDual = other_237;
    let _e135: MultiVector = self_279;
    let _e138: MotorDual = other_237;
    return MultiVector(((((vec4<f32>(_e4.g0_.y) * _e8.g0_.zzzw) * vec4<f32>(0.0, 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e19.g0_.w) * _e23.g0_.zwzz) * vec4<f32>(1.0, -(1.0), 0.0, 0.0))) + (_e35.g0_.zzxx * _e38.g0_.wzwz)), ((((((((vec4<f32>(_e43.g0_.y) * _e47.g0_.xyxx) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0)) + ((vec4<f32>(_e58.g0_.w) * _e62.g0_.xxxy) * vec4<f32>(0.0, 0.0, 1.0, 1.0))) + ((vec4<f32>(_e73.g1_.x) * _e77.g0_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e89.g1_.y) * _e93.g0_.zzzw) * vec4<f32>(0.0, 0.0, 1.0, 1.0))) + ((vec4<f32>(_e104.g1_.z) * _e108.g0_.wzww) * vec4<f32>(1.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e119.g1_.w) * _e123.g0_.zwzz) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))) + ((_e135.g0_.xxzz * _e138.g0_.yxyx) * vec4<f32>(1.0, 1.0, -(1.0), 1.0))));
}

fn multi_vector_motor_dual_regressive_product(self_280: MultiVector, other_238: MotorDual) -> MultiVector {
    var self_281: MultiVector;
    var other_239: MotorDual;

    self_281 = self_280;
    other_239 = other_238;
    let _e4: MultiVector = self_281;
    let _e8: MotorDual = other_239;
    let _e18: MultiVector = self_281;
    let _e22: MotorDual = other_239;
    let _e33: MultiVector = self_281;
    let _e37: MotorDual = other_239;
    let _e49: MultiVector = self_281;
    let _e53: MotorDual = other_239;
    let _e65: MultiVector = self_281;
    let _e68: MotorDual = other_239;
    let _e80: MultiVector = self_281;
    let _e84: MotorDual = other_239;
    let _e94: MultiVector = self_281;
    let _e97: MotorDual = other_239;
    return MultiVector(((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.yxyy) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) + ((vec4<f32>(_e18.g1_.y) * _e22.g0_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, 1.0))) + ((vec4<f32>(_e33.g1_.z) * vec4<f32>(_e37.g0_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e49.g1_.w) * vec4<f32>(_e53.g0_.w)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((_e65.g0_.xxzw * vec4<f32>(_e68.g0_.x)) * vec4<f32>(1.0, 0.0, 1.0, 1.0))), (((vec4<f32>(_e80.g1_.y) * _e84.g0_.yxyy) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) + ((_e94.g1_.xxzw * vec4<f32>(_e97.g0_.x)) * vec4<f32>(1.0, 0.0, 1.0, 1.0))));
}

fn multi_vector_motor_dual_scalar_product(self_282: MultiVector, other_240: MotorDual) -> Scalar {
    var self_283: MultiVector;
    var other_241: MotorDual;

    self_283 = self_282;
    other_241 = other_240;
    let _e4: MultiVector = self_283;
    let _e7: MotorDual = other_241;
    let _e11: MultiVector = self_283;
    let _e14: MotorDual = other_241;
    return Scalar(((_e4.g0_.z * _e7.g0_.w) + (_e11.g0_.w * _e14.g0_.z)));
}

fn multi_vector_squared_magnitude(self_284: MultiVector) -> Scalar {
    var self_285: MultiVector;

    self_285 = self_284;
    let _e4: MultiVector = self_285;
    let _e5: MultiVector = multi_vector_reversal(_e4);
    let _e6: MultiVector = self_285;
    let _e8: MultiVector = self_285;
    let _e9: MultiVector = multi_vector_reversal(_e8);
    let _e10: Scalar = multi_vector_multi_vector_scalar_product(_e6, _e9);
    return _e10;
}

fn multi_vector_magnitude(self_286: MultiVector) -> Scalar {
    var self_287: MultiVector;

    self_287 = self_286;
    let _e3: MultiVector = self_287;
    let _e4: Scalar = multi_vector_squared_magnitude(_e3);
    let _e7: MultiVector = self_287;
    let _e8: Scalar = multi_vector_squared_magnitude(_e7);
    return Scalar(sqrt(_e8.g0_));
}

fn multi_vector_scale(self_288: MultiVector, other_242: f32) -> MultiVector {
    var self_289: MultiVector;
    var other_243: f32;

    self_289 = self_288;
    other_243 = other_242;
    let _e5: f32 = other_243;
    let _e7: MultiVector = self_289;
    let _e8: f32 = other_243;
    let _e10: MultiVector = multi_vector_scalar_geometric_product(_e7, Scalar(_e8));
    return _e10;
}

fn multi_vector_signum(self_290: MultiVector) -> MultiVector {
    var self_291: MultiVector;

    self_291 = self_290;
    let _e5: MultiVector = self_291;
    let _e6: Scalar = multi_vector_magnitude(_e5);
    let _e10: MultiVector = self_291;
    let _e13: MultiVector = self_291;
    let _e14: Scalar = multi_vector_magnitude(_e13);
    let _e18: MultiVector = multi_vector_scalar_geometric_product(_e10, Scalar((1.0 / _e14.g0_)));
    return _e18;
}

fn multi_vector_inverse(self_292: MultiVector) -> MultiVector {
    var self_293: MultiVector;

    self_293 = self_292;
    let _e3: MultiVector = self_293;
    let _e4: MultiVector = multi_vector_reversal(_e3);
    let _e7: MultiVector = self_293;
    let _e8: Scalar = multi_vector_squared_magnitude(_e7);
    let _e13: MultiVector = self_293;
    let _e14: MultiVector = multi_vector_reversal(_e13);
    let _e17: MultiVector = self_293;
    let _e18: Scalar = multi_vector_squared_magnitude(_e17);
    let _e22: MultiVector = multi_vector_scalar_geometric_product(_e14, Scalar((1.0 / _e18.g0_)));
    return _e22;
}

fn rotor_zero() -> Rotor {
    return Rotor(vec2<f32>(0.0));
}

fn rotor_one() -> Rotor {
    return Rotor(vec2<f32>(1.0, 0.0));
}

fn rotor_neg(self_294: Rotor) -> Rotor {
    var self_295: Rotor;

    self_295 = self_294;
    let _e2: Rotor = self_295;
    return Rotor((_e2.g0_ * vec2<f32>(-(1.0))));
}

fn rotor_automorphism(self_296: Rotor) -> Rotor {
    var self_297: Rotor;

    self_297 = self_296;
    let _e2: Rotor = self_297;
    return Rotor(_e2.g0_);
}

fn rotor_reversal(self_298: Rotor) -> Rotor {
    var self_299: Rotor;

    self_299 = self_298;
    let _e2: Rotor = self_299;
    return Rotor((_e2.g0_ * vec2<f32>(1.0, -(1.0))));
}

fn rotor_conjugation(self_300: Rotor) -> Rotor {
    var self_301: Rotor;

    self_301 = self_300;
    let _e2: Rotor = self_301;
    return Rotor((_e2.g0_ * vec2<f32>(1.0, -(1.0))));
}

fn rotor_scalar_into(self_302: Rotor) -> Scalar {
    var self_303: Rotor;

    self_303 = self_302;
    let _e2: Rotor = self_303;
    return Scalar(_e2.g0_.x);
}

fn rotor_scalar_add(self_304: Rotor, other_244: Scalar) -> Rotor {
    var self_305: Rotor;
    var other_245: Scalar;

    self_305 = self_304;
    other_245 = other_244;
    let _e4: Rotor = self_305;
    let _e6: Scalar = other_245;
    return Rotor((_e4.g0_ + (vec2<f32>(_e6.g0_) * vec2<f32>(1.0, 0.0))));
}

fn rotor_scalar_sub(self_306: Rotor, other_246: Scalar) -> Rotor {
    var self_307: Rotor;
    var other_247: Scalar;

    self_307 = self_306;
    other_247 = other_246;
    let _e4: Rotor = self_307;
    let _e6: Scalar = other_247;
    return Rotor((_e4.g0_ - (vec2<f32>(_e6.g0_) * vec2<f32>(1.0, 0.0))));
}

fn rotor_scalar_geometric_product(self_308: Rotor, other_248: Scalar) -> Rotor {
    var self_309: Rotor;
    var other_249: Scalar;

    self_309 = self_308;
    other_249 = other_248;
    let _e4: Rotor = self_309;
    let _e6: Scalar = other_249;
    return Rotor((_e4.g0_ * vec2<f32>(_e6.g0_)));
}

fn rotor_scalar_outer_product(self_310: Rotor, other_250: Scalar) -> Rotor {
    var self_311: Rotor;
    var other_251: Scalar;

    self_311 = self_310;
    other_251 = other_250;
    let _e4: Rotor = self_311;
    let _e6: Scalar = other_251;
    return Rotor((_e4.g0_ * vec2<f32>(_e6.g0_)));
}

fn rotor_scalar_inner_product(self_312: Rotor, other_252: Scalar) -> Rotor {
    var self_313: Rotor;
    var other_253: Scalar;

    self_313 = self_312;
    other_253 = other_252;
    let _e4: Rotor = self_313;
    let _e6: Scalar = other_253;
    return Rotor((_e4.g0_ * vec2<f32>(_e6.g0_)));
}

fn rotor_scalar_left_contraction(self_314: Rotor, other_254: Scalar) -> Scalar {
    var self_315: Rotor;
    var other_255: Scalar;

    self_315 = self_314;
    other_255 = other_254;
    let _e4: Rotor = self_315;
    let _e7: Scalar = other_255;
    return Scalar((_e4.g0_.x * _e7.g0_));
}

fn rotor_scalar_right_contraction(self_316: Rotor, other_256: Scalar) -> Rotor {
    var self_317: Rotor;
    var other_257: Scalar;

    self_317 = self_316;
    other_257 = other_256;
    let _e4: Rotor = self_317;
    let _e6: Scalar = other_257;
    return Rotor((_e4.g0_ * vec2<f32>(_e6.g0_)));
}

fn rotor_scalar_scalar_product(self_318: Rotor, other_258: Scalar) -> Scalar {
    var self_319: Rotor;
    var other_259: Scalar;

    self_319 = self_318;
    other_259 = other_258;
    let _e4: Rotor = self_319;
    let _e7: Scalar = other_259;
    return Scalar((_e4.g0_.x * _e7.g0_));
}

fn rotor_multi_vector_add(self_320: Rotor, other_260: MultiVector) -> MultiVector {
    var self_321: Rotor;
    var other_261: MultiVector;

    self_321 = self_320;
    other_261 = other_260;
    let _e4: Rotor = self_321;
    let _e7: Rotor = self_321;
    let _e10: Rotor = self_321;
    let _e13: Rotor = self_321;
    let _e23: MultiVector = other_261;
    let _e26: MultiVector = other_261;
    return MultiVector(((vec4<f32>(_e4.g0_.x, _e7.g0_.y, _e10.g0_.x, _e13.g0_.x) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) + _e23.g0_), _e26.g1_);
}

fn rotor_multi_vector_sub(self_322: Rotor, other_262: MultiVector) -> MultiVector {
    var self_323: Rotor;
    var other_263: MultiVector;

    self_323 = self_322;
    other_263 = other_262;
    let _e4: Rotor = self_323;
    let _e7: Rotor = self_323;
    let _e10: Rotor = self_323;
    let _e13: Rotor = self_323;
    let _e23: MultiVector = other_263;
    let _e28: MultiVector = other_263;
    return MultiVector(((vec4<f32>(_e4.g0_.x, _e7.g0_.y, _e10.g0_.x, _e13.g0_.x) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) - _e23.g0_), (vec4<f32>(0.0) - _e28.g1_));
}

fn rotor_multi_vector_geometric_product(self_324: Rotor, other_264: MultiVector) -> MultiVector {
    var self_325: Rotor;
    var other_265: MultiVector;

    self_325 = self_324;
    other_265 = other_264;
    let _e4: Rotor = self_325;
    let _e8: MultiVector = other_265;
    let _e11: Rotor = self_325;
    let _e15: MultiVector = other_265;
    let _e28: Rotor = self_325;
    let _e32: MultiVector = other_265;
    let _e35: Rotor = self_325;
    let _e39: MultiVector = other_265;
    return MultiVector(((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.y) * _e15.g0_.yxwz) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))), ((vec4<f32>(_e28.g0_.x) * _e32.g1_) + ((vec4<f32>(_e35.g0_.y) * _e39.g1_.yxwz) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))));
}

fn rotor_multi_vector_outer_product(self_326: Rotor, other_266: MultiVector) -> MultiVector {
    var self_327: Rotor;
    var other_267: MultiVector;

    self_327 = self_326;
    other_267 = other_266;
    let _e4: Rotor = self_327;
    let _e8: MultiVector = other_267;
    let _e11: Rotor = self_327;
    let _e14: Rotor = self_327;
    let _e17: Rotor = self_327;
    let _e20: Rotor = self_327;
    let _e24: MultiVector = other_267;
    let _e36: Rotor = self_327;
    let _e40: MultiVector = other_267;
    let _e43: Rotor = self_327;
    let _e46: Rotor = self_327;
    let _e49: Rotor = self_327;
    let _e52: Rotor = self_327;
    let _e56: MultiVector = other_267;
    return MultiVector(((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.x, _e14.g0_.y, _e17.g0_.x, _e20.g0_.x) * vec4<f32>(_e24.g0_.x)) * vec4<f32>(0.0, 1.0, 0.0, 0.0))), ((vec4<f32>(_e36.g0_.x) * _e40.g1_) + ((vec4<f32>(_e43.g0_.x, _e46.g0_.y, _e49.g0_.x, _e52.g0_.x) * vec4<f32>(_e56.g1_.x)) * vec4<f32>(0.0, 1.0, 0.0, 0.0))));
}

fn rotor_multi_vector_inner_product(self_328: Rotor, other_268: MultiVector) -> MultiVector {
    var self_329: Rotor;
    var other_269: MultiVector;

    self_329 = self_328;
    other_269 = other_268;
    let _e4: Rotor = self_329;
    let _e8: MultiVector = other_269;
    let _e11: Rotor = self_329;
    let _e15: MultiVector = other_269;
    let _e28: Rotor = self_329;
    let _e32: MultiVector = other_269;
    let _e35: Rotor = self_329;
    let _e38: Rotor = self_329;
    let _e41: Rotor = self_329;
    let _e44: Rotor = self_329;
    let _e48: MultiVector = other_269;
    return MultiVector(((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.y) * _e15.g0_.yxwz) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))), ((vec4<f32>(_e28.g0_.x) * _e32.g1_) + ((vec4<f32>(_e35.g0_.y, _e38.g0_.x, _e41.g0_.x, _e44.g0_.x) * _e48.g1_.yxxx) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))));
}

fn rotor_multi_vector_left_contraction(self_330: Rotor, other_270: MultiVector) -> MultiVector {
    var self_331: Rotor;
    var other_271: MultiVector;

    self_331 = self_330;
    other_271 = other_270;
    let _e4: Rotor = self_331;
    let _e8: MultiVector = other_271;
    let _e11: Rotor = self_331;
    let _e14: Rotor = self_331;
    let _e17: Rotor = self_331;
    let _e20: Rotor = self_331;
    let _e24: MultiVector = other_271;
    let _e36: Rotor = self_331;
    let _e40: MultiVector = other_271;
    let _e43: Rotor = self_331;
    let _e46: Rotor = self_331;
    let _e49: Rotor = self_331;
    let _e52: Rotor = self_331;
    let _e56: MultiVector = other_271;
    return MultiVector(((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.y, _e14.g0_.x, _e17.g0_.x, _e20.g0_.x) * _e24.g0_.yxxx) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))), ((vec4<f32>(_e36.g0_.x) * _e40.g1_) + ((vec4<f32>(_e43.g0_.y, _e46.g0_.x, _e49.g0_.x, _e52.g0_.x) * _e56.g1_.yxxx) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))));
}

fn rotor_multi_vector_scalar_product(self_332: Rotor, other_272: MultiVector) -> Scalar {
    var self_333: Rotor;
    var other_273: MultiVector;

    self_333 = self_332;
    other_273 = other_272;
    let _e4: Rotor = self_333;
    let _e7: MultiVector = other_273;
    let _e11: Rotor = self_333;
    let _e14: MultiVector = other_273;
    return Scalar(((_e4.g0_.x * _e7.g0_.x) - (_e11.g0_.y * _e14.g0_.y)));
}

fn rotor_rotor_add(self_334: Rotor, other_274: Rotor) -> Rotor {
    var self_335: Rotor;
    var other_275: Rotor;

    self_335 = self_334;
    other_275 = other_274;
    let _e4: Rotor = self_335;
    let _e6: Rotor = other_275;
    return Rotor((_e4.g0_ + _e6.g0_));
}

fn rotor_rotor_sub(self_336: Rotor, other_276: Rotor) -> Rotor {
    var self_337: Rotor;
    var other_277: Rotor;

    self_337 = self_336;
    other_277 = other_276;
    let _e4: Rotor = self_337;
    let _e6: Rotor = other_277;
    return Rotor((_e4.g0_ - _e6.g0_));
}

fn rotor_rotor_mul(self_338: Rotor, other_278: Rotor) -> Rotor {
    var self_339: Rotor;
    var other_279: Rotor;

    self_339 = self_338;
    other_279 = other_278;
    let _e4: Rotor = self_339;
    let _e6: Rotor = other_279;
    return Rotor((_e4.g0_ * _e6.g0_));
}

fn rotor_rotor_div(self_340: Rotor, other_280: Rotor) -> Rotor {
    var self_341: Rotor;
    var other_281: Rotor;

    self_341 = self_340;
    other_281 = other_280;
    let _e4: Rotor = self_341;
    let _e7: Rotor = self_341;
    let _e15: Rotor = other_281;
    let _e18: Rotor = other_281;
    return Rotor((((vec2<f32>(_e4.g0_.x, _e7.g0_.y) * vec2<f32>(1.0, 1.0)) / vec2<f32>(_e15.g0_.x, _e18.g0_.y)) * vec2<f32>(1.0, 1.0)));
}

fn rotor_rotor_geometric_product(self_342: Rotor, other_282: Rotor) -> Rotor {
    var self_343: Rotor;
    var other_283: Rotor;

    self_343 = self_342;
    other_283 = other_282;
    let _e4: Rotor = self_343;
    let _e8: Rotor = other_283;
    let _e11: Rotor = self_343;
    let _e15: Rotor = other_283;
    return Rotor(((vec2<f32>(_e4.g0_.x) * _e8.g0_) + ((vec2<f32>(_e11.g0_.y) * _e15.g0_.yx) * vec2<f32>(-(1.0), 1.0))));
}

fn rotor_rotor_outer_product(self_344: Rotor, other_284: Rotor) -> Rotor {
    var self_345: Rotor;
    var other_285: Rotor;

    self_345 = self_344;
    other_285 = other_284;
    let _e4: Rotor = self_345;
    let _e8: Rotor = other_285;
    let _e11: Rotor = self_345;
    let _e13: Rotor = other_285;
    return Rotor(((vec2<f32>(_e4.g0_.x) * _e8.g0_) + ((_e11.g0_ * vec2<f32>(_e13.g0_.x)) * vec2<f32>(0.0, 1.0))));
}

fn rotor_rotor_inner_product(self_346: Rotor, other_286: Rotor) -> Rotor {
    var self_347: Rotor;
    var other_287: Rotor;

    self_347 = self_346;
    other_287 = other_286;
    let _e4: Rotor = self_347;
    let _e8: Rotor = other_287;
    let _e11: Rotor = self_347;
    let _e15: Rotor = other_287;
    return Rotor(((vec2<f32>(_e4.g0_.x) * _e8.g0_) + ((vec2<f32>(_e11.g0_.y) * _e15.g0_.yx) * vec2<f32>(-(1.0), 1.0))));
}

fn rotor_rotor_left_contraction(self_348: Rotor, other_288: Rotor) -> Rotor {
    var self_349: Rotor;
    var other_289: Rotor;

    self_349 = self_348;
    other_289 = other_288;
    let _e4: Rotor = self_349;
    let _e8: Rotor = other_289;
    let _e11: Rotor = self_349;
    let _e14: Rotor = other_289;
    return Rotor(((vec2<f32>(_e4.g0_.x) * _e8.g0_) + ((_e11.g0_.yx * _e14.g0_.yx) * vec2<f32>(-(1.0), 0.0))));
}

fn rotor_rotor_right_contraction(self_350: Rotor, other_290: Rotor) -> Rotor {
    var self_351: Rotor;
    var other_291: Rotor;

    self_351 = self_350;
    other_291 = other_290;
    let _e4: Rotor = self_351;
    let _e8: Rotor = other_291;
    let _e17: Rotor = self_351;
    let _e21: Rotor = other_291;
    return Rotor((((vec2<f32>(_e4.g0_.y) * _e8.g0_.yx) * vec2<f32>(-(1.0), 1.0)) + ((vec2<f32>(_e17.g0_.x) * vec2<f32>(_e21.g0_.x)) * vec2<f32>(1.0, 0.0))));
}

fn rotor_rotor_scalar_product(self_352: Rotor, other_292: Rotor) -> Scalar {
    var self_353: Rotor;
    var other_293: Rotor;

    self_353 = self_352;
    other_293 = other_292;
    let _e4: Rotor = self_353;
    let _e7: Rotor = other_293;
    let _e11: Rotor = self_353;
    let _e14: Rotor = other_293;
    return Scalar(((_e4.g0_.x * _e7.g0_.x) - (_e11.g0_.y * _e14.g0_.y)));
}

fn rotor_point_add(self_354: Rotor, other_294: Point) -> Motor {
    var self_355: Rotor;
    var other_295: Point;

    self_355 = self_354;
    other_295 = other_294;
    let _e4: Rotor = self_355;
    let _e7: Rotor = self_355;
    let _e10: Rotor = self_355;
    let _e13: Rotor = self_355;
    let _e23: Point = other_295;
    let _e26: Point = other_295;
    let _e29: Point = other_295;
    let _e32: Point = other_295;
    return Motor(((vec4<f32>(_e4.g0_.x, _e7.g0_.y, _e10.g0_.x, _e13.g0_.x) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) + (vec4<f32>(_e23.g0_.x, _e26.g0_.x, _e29.g0_.y, _e32.g0_.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn rotor_point_sub(self_356: Rotor, other_296: Point) -> Motor {
    var self_357: Rotor;
    var other_297: Point;

    self_357 = self_356;
    other_297 = other_296;
    let _e4: Rotor = self_357;
    let _e7: Rotor = self_357;
    let _e10: Rotor = self_357;
    let _e13: Rotor = self_357;
    let _e23: Point = other_297;
    let _e26: Point = other_297;
    let _e29: Point = other_297;
    let _e32: Point = other_297;
    return Motor(((vec4<f32>(_e4.g0_.x, _e7.g0_.y, _e10.g0_.x, _e13.g0_.x) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) - (vec4<f32>(_e23.g0_.x, _e26.g0_.x, _e29.g0_.y, _e32.g0_.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn rotor_point_geometric_product(self_358: Rotor, other_298: Point) -> Motor {
    var self_359: Rotor;
    var other_299: Point;

    self_359 = self_358;
    other_299 = other_298;
    let _e4: Rotor = self_359;
    let _e8: Point = other_299;
    let _e11: Point = other_299;
    let _e14: Point = other_299;
    let _e17: Point = other_299;
    let _e30: Rotor = self_359;
    let _e34: Point = other_299;
    let _e37: Point = other_299;
    let _e40: Point = other_299;
    let _e43: Point = other_299;
    return Motor((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.x, _e11.g0_.x, _e14.g0_.z, _e17.g0_.y)) * vec4<f32>(-(1.0), 0.0, -(1.0), 1.0)) + ((vec4<f32>(_e30.g0_.x) * vec4<f32>(_e34.g0_.x, _e37.g0_.x, _e40.g0_.y, _e43.g0_.z)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn rotor_point_outer_product(self_360: Rotor, other_300: Point) -> Point {
    var self_361: Rotor;
    var other_301: Point;

    self_361 = self_360;
    other_301 = other_300;
    let _e4: Rotor = self_361;
    let _e8: Point = other_301;
    return Point((vec3<f32>(_e4.g0_.x) * _e8.g0_));
}

fn rotor_point_inner_product(self_362: Rotor, other_302: Point) -> Motor {
    var self_363: Rotor;
    var other_303: Point;

    self_363 = self_362;
    other_303 = other_302;
    let _e4: Rotor = self_363;
    let _e7: Rotor = self_363;
    let _e10: Rotor = self_363;
    let _e13: Rotor = self_363;
    let _e17: Point = other_303;
    let _e20: Point = other_303;
    let _e23: Point = other_303;
    let _e26: Point = other_303;
    return Motor(((vec4<f32>(_e4.g0_.y, _e7.g0_.x, _e10.g0_.x, _e13.g0_.x) * vec4<f32>(_e17.g0_.x, _e20.g0_.x, _e23.g0_.y, _e26.g0_.z)) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0)));
}

fn rotor_point_left_contraction(self_364: Rotor, other_304: Point) -> Motor {
    var self_365: Rotor;
    var other_305: Point;

    self_365 = self_364;
    other_305 = other_304;
    let _e4: Rotor = self_365;
    let _e7: Rotor = self_365;
    let _e10: Rotor = self_365;
    let _e13: Rotor = self_365;
    let _e17: Point = other_305;
    let _e20: Point = other_305;
    let _e23: Point = other_305;
    let _e26: Point = other_305;
    return Motor(((vec4<f32>(_e4.g0_.y, _e7.g0_.x, _e10.g0_.x, _e13.g0_.x) * vec4<f32>(_e17.g0_.x, _e20.g0_.x, _e23.g0_.y, _e26.g0_.z)) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0)));
}

fn rotor_point_right_contraction(self_366: Rotor, other_306: Point) -> Scalar {
    var self_367: Rotor;
    var other_307: Point;

    self_367 = self_366;
    other_307 = other_306;
    let _e5: Rotor = self_367;
    let _e8: Point = other_307;
    return Scalar((0.0 - (_e5.g0_.y * _e8.g0_.x)));
}

fn rotor_point_scalar_product(self_368: Rotor, other_308: Point) -> Scalar {
    var self_369: Rotor;
    var other_309: Point;

    self_369 = self_368;
    other_309 = other_308;
    let _e5: Rotor = self_369;
    let _e8: Point = other_309;
    return Scalar((0.0 - (_e5.g0_.y * _e8.g0_.x)));
}

fn rotor_ideal_point_add(self_370: Rotor, other_310: IdealPoint) -> Motor {
    var self_371: Rotor;
    var other_311: IdealPoint;

    self_371 = self_370;
    other_311 = other_310;
    let _e4: Rotor = self_371;
    let _e7: Rotor = self_371;
    let _e10: Rotor = self_371;
    let _e13: Rotor = self_371;
    let _e23: IdealPoint = other_311;
    let _e26: IdealPoint = other_311;
    let _e29: IdealPoint = other_311;
    let _e32: IdealPoint = other_311;
    return Motor(((vec4<f32>(_e4.g0_.x, _e7.g0_.y, _e10.g0_.x, _e13.g0_.x) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) + (vec4<f32>(_e23.g0_.x, _e26.g0_.x, _e29.g0_.x, _e32.g0_.y) * vec4<f32>(0.0, 0.0, 1.0, 1.0))));
}

fn rotor_ideal_point_sub(self_372: Rotor, other_312: IdealPoint) -> Motor {
    var self_373: Rotor;
    var other_313: IdealPoint;

    self_373 = self_372;
    other_313 = other_312;
    let _e4: Rotor = self_373;
    let _e7: Rotor = self_373;
    let _e10: Rotor = self_373;
    let _e13: Rotor = self_373;
    let _e23: IdealPoint = other_313;
    let _e26: IdealPoint = other_313;
    let _e29: IdealPoint = other_313;
    let _e32: IdealPoint = other_313;
    return Motor(((vec4<f32>(_e4.g0_.x, _e7.g0_.y, _e10.g0_.x, _e13.g0_.x) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) - (vec4<f32>(_e23.g0_.x, _e26.g0_.x, _e29.g0_.x, _e32.g0_.y) * vec4<f32>(0.0, 0.0, 1.0, 1.0))));
}

fn rotor_ideal_point_geometric_product(self_374: Rotor, other_314: IdealPoint) -> IdealPoint {
    var self_375: Rotor;
    var other_315: IdealPoint;

    self_375 = self_374;
    other_315 = other_314;
    let _e4: Rotor = self_375;
    let _e8: IdealPoint = other_315;
    let _e11: Rotor = self_375;
    let _e15: IdealPoint = other_315;
    return IdealPoint(((vec2<f32>(_e4.g0_.x) * _e8.g0_) + ((vec2<f32>(_e11.g0_.y) * _e15.g0_.yx) * vec2<f32>(-(1.0), 1.0))));
}

fn rotor_ideal_point_outer_product(self_376: Rotor, other_316: IdealPoint) -> IdealPoint {
    var self_377: Rotor;
    var other_317: IdealPoint;

    self_377 = self_376;
    other_317 = other_316;
    let _e4: Rotor = self_377;
    let _e8: IdealPoint = other_317;
    return IdealPoint((vec2<f32>(_e4.g0_.x) * _e8.g0_));
}

fn rotor_ideal_point_inner_product(self_378: Rotor, other_318: IdealPoint) -> IdealPoint {
    var self_379: Rotor;
    var other_319: IdealPoint;

    self_379 = self_378;
    other_319 = other_318;
    let _e4: Rotor = self_379;
    let _e8: IdealPoint = other_319;
    return IdealPoint((vec2<f32>(_e4.g0_.x) * _e8.g0_));
}

fn rotor_ideal_point_left_contraction(self_380: Rotor, other_320: IdealPoint) -> IdealPoint {
    var self_381: Rotor;
    var other_321: IdealPoint;

    self_381 = self_380;
    other_321 = other_320;
    let _e4: Rotor = self_381;
    let _e8: IdealPoint = other_321;
    return IdealPoint((vec2<f32>(_e4.g0_.x) * _e8.g0_));
}

fn rotor_plane_geometric_product(self_382: Rotor, other_322: Plane) -> MotorDual {
    var self_383: Rotor;
    var other_323: Plane;

    self_383 = self_382;
    other_323 = other_322;
    let _e4: Rotor = self_383;
    let _e8: Plane = other_323;
    let _e11: Plane = other_323;
    let _e14: Plane = other_323;
    let _e17: Plane = other_323;
    let _e29: Rotor = self_383;
    let _e33: Plane = other_323;
    let _e36: Plane = other_323;
    let _e39: Plane = other_323;
    let _e42: Plane = other_323;
    return MotorDual((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.x, _e11.g0_.x, _e14.g0_.z, _e17.g0_.y)) * vec4<f32>(1.0, 0.0, -(1.0), 1.0)) + ((vec4<f32>(_e29.g0_.x) * vec4<f32>(_e33.g0_.x, _e36.g0_.x, _e39.g0_.y, _e42.g0_.z)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn rotor_plane_regressive_product(self_384: Rotor, other_324: Plane) -> Scalar {
    var self_385: Rotor;
    var other_325: Plane;

    self_385 = self_384;
    other_325 = other_324;
    let _e4: Rotor = self_385;
    let _e7: Plane = other_325;
    return Scalar((_e4.g0_.y * _e7.g0_.x));
}

fn rotor_plane_outer_product(self_386: Rotor, other_326: Plane) -> MotorDual {
    var self_387: Rotor;
    var other_327: Plane;

    self_387 = self_386;
    other_327 = other_326;
    let _e4: Rotor = self_387;
    let _e7: Rotor = self_387;
    let _e10: Rotor = self_387;
    let _e13: Rotor = self_387;
    let _e17: Plane = other_327;
    let _e20: Plane = other_327;
    let _e23: Plane = other_327;
    let _e26: Plane = other_327;
    return MotorDual((vec4<f32>(_e4.g0_.y, _e7.g0_.x, _e10.g0_.x, _e13.g0_.x) * vec4<f32>(_e17.g0_.x, _e20.g0_.x, _e23.g0_.y, _e26.g0_.z)));
}

fn rotor_plane_inner_product(self_388: Rotor, other_328: Plane) -> Plane {
    var self_389: Rotor;
    var other_329: Plane;

    self_389 = self_388;
    other_329 = other_328;
    let _e4: Rotor = self_389;
    let _e8: Plane = other_329;
    let _e11: Rotor = self_389;
    let _e14: Rotor = self_389;
    let _e17: Rotor = self_389;
    let _e21: Plane = other_329;
    return Plane(((vec3<f32>(_e4.g0_.x) * _e8.g0_) + ((vec3<f32>(_e11.g0_.x, _e14.g0_.y, _e17.g0_.y) * _e21.g0_.xzy) * vec3<f32>(0.0, -(1.0), 1.0))));
}

fn rotor_plane_left_contraction(self_390: Rotor, other_330: Plane) -> Plane {
    var self_391: Rotor;
    var other_331: Plane;

    self_391 = self_390;
    other_331 = other_330;
    let _e4: Rotor = self_391;
    let _e8: Plane = other_331;
    return Plane((vec3<f32>(_e4.g0_.x) * _e8.g0_));
}

fn rotor_translator_add(self_392: Rotor, other_332: Translator) -> Motor {
    var self_393: Rotor;
    var other_333: Translator;

    self_393 = self_392;
    other_333 = other_332;
    let _e4: Rotor = self_393;
    let _e7: Rotor = self_393;
    let _e10: Rotor = self_393;
    let _e13: Rotor = self_393;
    let _e23: Translator = other_333;
    let _e26: Translator = other_333;
    let _e29: Translator = other_333;
    let _e32: Translator = other_333;
    return Motor(((vec4<f32>(_e4.g0_.x, _e7.g0_.y, _e10.g0_.x, _e13.g0_.x) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) + (vec4<f32>(_e23.g0_.x, _e26.g0_.x, _e29.g0_.y, _e32.g0_.z) * vec4<f32>(1.0, 0.0, 1.0, 1.0))));
}

fn rotor_translator_sub(self_394: Rotor, other_334: Translator) -> Motor {
    var self_395: Rotor;
    var other_335: Translator;

    self_395 = self_394;
    other_335 = other_334;
    let _e4: Rotor = self_395;
    let _e7: Rotor = self_395;
    let _e10: Rotor = self_395;
    let _e13: Rotor = self_395;
    let _e23: Translator = other_335;
    let _e26: Translator = other_335;
    let _e29: Translator = other_335;
    let _e32: Translator = other_335;
    return Motor(((vec4<f32>(_e4.g0_.x, _e7.g0_.y, _e10.g0_.x, _e13.g0_.x) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) - (vec4<f32>(_e23.g0_.x, _e26.g0_.x, _e29.g0_.y, _e32.g0_.z) * vec4<f32>(1.0, 0.0, 1.0, 1.0))));
}

fn rotor_translator_geometric_product(self_396: Rotor, other_336: Translator) -> Motor {
    var self_397: Rotor;
    var other_337: Translator;

    self_397 = self_396;
    other_337 = other_336;
    let _e4: Rotor = self_397;
    let _e8: Translator = other_337;
    let _e11: Translator = other_337;
    let _e14: Translator = other_337;
    let _e17: Translator = other_337;
    let _e29: Rotor = self_397;
    let _e33: Translator = other_337;
    let _e36: Translator = other_337;
    let _e39: Translator = other_337;
    let _e42: Translator = other_337;
    return Motor((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.x, _e11.g0_.x, _e14.g0_.z, _e17.g0_.y)) * vec4<f32>(0.0, 1.0, -(1.0), 1.0)) + ((vec4<f32>(_e29.g0_.x) * vec4<f32>(_e33.g0_.x, _e36.g0_.x, _e39.g0_.y, _e42.g0_.z)) * vec4<f32>(1.0, 0.0, 1.0, 1.0))));
}

fn rotor_translator_outer_product(self_398: Rotor, other_338: Translator) -> Motor {
    var self_399: Rotor;
    var other_339: Translator;

    self_399 = self_398;
    other_339 = other_338;
    let _e4: Rotor = self_399;
    let _e7: Rotor = self_399;
    let _e10: Rotor = self_399;
    let _e13: Rotor = self_399;
    let _e17: Translator = other_339;
    let _e20: Translator = other_339;
    let _e23: Translator = other_339;
    let _e26: Translator = other_339;
    return Motor((vec4<f32>(_e4.g0_.x, _e7.g0_.y, _e10.g0_.x, _e13.g0_.x) * vec4<f32>(_e17.g0_.x, _e20.g0_.x, _e23.g0_.y, _e26.g0_.z)));
}

fn rotor_translator_inner_product(self_400: Rotor, other_340: Translator) -> Motor {
    var self_401: Rotor;
    var other_341: Translator;

    self_401 = self_400;
    other_341 = other_340;
    let _e4: Rotor = self_401;
    let _e7: Rotor = self_401;
    let _e10: Rotor = self_401;
    let _e13: Rotor = self_401;
    let _e17: Translator = other_341;
    let _e20: Translator = other_341;
    let _e23: Translator = other_341;
    let _e26: Translator = other_341;
    return Motor((vec4<f32>(_e4.g0_.x, _e7.g0_.y, _e10.g0_.x, _e13.g0_.x) * vec4<f32>(_e17.g0_.x, _e20.g0_.x, _e23.g0_.y, _e26.g0_.z)));
}

fn rotor_translator_left_contraction(self_402: Rotor, other_342: Translator) -> Translator {
    var self_403: Rotor;
    var other_343: Translator;

    self_403 = self_402;
    other_343 = other_342;
    let _e4: Rotor = self_403;
    let _e8: Translator = other_343;
    return Translator((vec3<f32>(_e4.g0_.x) * _e8.g0_));
}

fn rotor_translator_right_contraction(self_404: Rotor, other_344: Translator) -> Rotor {
    var self_405: Rotor;
    var other_345: Translator;

    self_405 = self_404;
    other_345 = other_344;
    let _e4: Rotor = self_405;
    let _e6: Translator = other_345;
    return Rotor((_e4.g0_ * vec2<f32>(_e6.g0_.x)));
}

fn rotor_translator_scalar_product(self_406: Rotor, other_346: Translator) -> Scalar {
    var self_407: Rotor;
    var other_347: Translator;

    self_407 = self_406;
    other_347 = other_346;
    let _e4: Rotor = self_407;
    let _e7: Translator = other_347;
    return Scalar((_e4.g0_.x * _e7.g0_.x));
}

fn rotor_motor_add(self_408: Rotor, other_348: Motor) -> Motor {
    var self_409: Rotor;
    var other_349: Motor;

    self_409 = self_408;
    other_349 = other_348;
    let _e4: Rotor = self_409;
    let _e7: Rotor = self_409;
    let _e10: Rotor = self_409;
    let _e13: Rotor = self_409;
    let _e23: Motor = other_349;
    return Motor(((vec4<f32>(_e4.g0_.x, _e7.g0_.y, _e10.g0_.x, _e13.g0_.x) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) + _e23.g0_));
}

fn rotor_motor_sub(self_410: Rotor, other_350: Motor) -> Motor {
    var self_411: Rotor;
    var other_351: Motor;

    self_411 = self_410;
    other_351 = other_350;
    let _e4: Rotor = self_411;
    let _e7: Rotor = self_411;
    let _e10: Rotor = self_411;
    let _e13: Rotor = self_411;
    let _e23: Motor = other_351;
    return Motor(((vec4<f32>(_e4.g0_.x, _e7.g0_.y, _e10.g0_.x, _e13.g0_.x) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) - _e23.g0_));
}

fn rotor_motor_geometric_product(self_412: Rotor, other_352: Motor) -> Motor {
    var self_413: Rotor;
    var other_353: Motor;

    self_413 = self_412;
    other_353 = other_352;
    let _e4: Rotor = self_413;
    let _e8: Motor = other_353;
    let _e11: Rotor = self_413;
    let _e15: Motor = other_353;
    return Motor(((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.y) * _e15.g0_.yxwz) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))));
}

fn rotor_motor_outer_product(self_414: Rotor, other_354: Motor) -> Motor {
    var self_415: Rotor;
    var other_355: Motor;

    self_415 = self_414;
    other_355 = other_354;
    let _e4: Rotor = self_415;
    let _e8: Motor = other_355;
    let _e11: Rotor = self_415;
    let _e14: Rotor = self_415;
    let _e17: Rotor = self_415;
    let _e20: Rotor = self_415;
    let _e24: Motor = other_355;
    return Motor(((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.x, _e14.g0_.y, _e17.g0_.x, _e20.g0_.x) * vec4<f32>(_e24.g0_.x)) * vec4<f32>(0.0, 1.0, 0.0, 0.0))));
}

fn rotor_motor_inner_product(self_416: Rotor, other_356: Motor) -> Motor {
    var self_417: Rotor;
    var other_357: Motor;

    self_417 = self_416;
    other_357 = other_356;
    let _e4: Rotor = self_417;
    let _e8: Motor = other_357;
    let _e11: Rotor = self_417;
    let _e14: Rotor = self_417;
    let _e17: Rotor = self_417;
    let _e20: Rotor = self_417;
    let _e24: Motor = other_357;
    return Motor(((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.y, _e14.g0_.y, _e17.g0_.x, _e20.g0_.x) * _e24.g0_.yxxx) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))));
}

fn rotor_motor_left_contraction(self_418: Rotor, other_358: Motor) -> Motor {
    var self_419: Rotor;
    var other_359: Motor;

    self_419 = self_418;
    other_359 = other_358;
    let _e4: Rotor = self_419;
    let _e8: Motor = other_359;
    let _e11: Rotor = self_419;
    let _e14: Rotor = self_419;
    let _e17: Rotor = self_419;
    let _e20: Rotor = self_419;
    let _e24: Motor = other_359;
    return Motor(((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.y, _e14.g0_.x, _e17.g0_.x, _e20.g0_.x) * _e24.g0_.yxxx) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))));
}

fn rotor_motor_right_contraction(self_420: Rotor, other_360: Motor) -> Rotor {
    var self_421: Rotor;
    var other_361: Motor;

    self_421 = self_420;
    other_361 = other_360;
    let _e4: Rotor = self_421;
    let _e8: Motor = other_361;
    let _e11: Motor = other_361;
    let _e21: Rotor = self_421;
    let _e25: Motor = other_361;
    return Rotor((((vec2<f32>(_e4.g0_.y) * vec2<f32>(_e8.g0_.y, _e11.g0_.x)) * vec2<f32>(-(1.0), 1.0)) + ((vec2<f32>(_e21.g0_.x) * vec2<f32>(_e25.g0_.x)) * vec2<f32>(1.0, 0.0))));
}

fn rotor_motor_scalar_product(self_422: Rotor, other_362: Motor) -> Scalar {
    var self_423: Rotor;
    var other_363: Motor;

    self_423 = self_422;
    other_363 = other_362;
    let _e4: Rotor = self_423;
    let _e7: Motor = other_363;
    let _e11: Rotor = self_423;
    let _e14: Motor = other_363;
    return Scalar(((_e4.g0_.x * _e7.g0_.x) - (_e11.g0_.y * _e14.g0_.y)));
}

fn rotor_motor_dual_geometric_product(self_424: Rotor, other_364: MotorDual) -> MotorDual {
    var self_425: Rotor;
    var other_365: MotorDual;

    self_425 = self_424;
    other_365 = other_364;
    let _e4: Rotor = self_425;
    let _e8: MotorDual = other_365;
    let _e11: Rotor = self_425;
    let _e15: MotorDual = other_365;
    return MotorDual(((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.y) * _e15.g0_.yxwz) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0))));
}

fn rotor_motor_dual_regressive_product(self_426: Rotor, other_366: MotorDual) -> Rotor {
    var self_427: Rotor;
    var other_367: MotorDual;

    self_427 = self_426;
    other_367 = other_366;
    let _e4: Rotor = self_427;
    let _e8: MotorDual = other_367;
    let _e11: MotorDual = other_367;
    let _e16: Rotor = self_427;
    let _e20: MotorDual = other_367;
    return Rotor(((vec2<f32>(_e4.g0_.y) * vec2<f32>(_e8.g0_.y, _e11.g0_.x)) + ((vec2<f32>(_e16.g0_.x) * vec2<f32>(_e20.g0_.x)) * vec2<f32>(1.0, 0.0))));
}

fn rotor_motor_dual_outer_product(self_428: Rotor, other_368: MotorDual) -> MotorDual {
    var self_429: Rotor;
    var other_369: MotorDual;

    self_429 = self_428;
    other_369 = other_368;
    let _e4: Rotor = self_429;
    let _e8: MotorDual = other_369;
    let _e11: Rotor = self_429;
    let _e14: Rotor = self_429;
    let _e17: Rotor = self_429;
    let _e20: Rotor = self_429;
    let _e24: MotorDual = other_369;
    return MotorDual(((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.y, _e14.g0_.x, _e17.g0_.x, _e20.g0_.x) * _e24.g0_.yxxx) * vec4<f32>(1.0, 0.0, 0.0, 0.0))));
}

fn rotor_motor_dual_inner_product(self_430: Rotor, other_370: MotorDual) -> MotorDual {
    var self_431: Rotor;
    var other_371: MotorDual;

    self_431 = self_430;
    other_371 = other_370;
    let _e4: Rotor = self_431;
    let _e8: MotorDual = other_371;
    let _e11: Rotor = self_431;
    let _e14: Rotor = self_431;
    let _e17: Rotor = self_431;
    let _e20: Rotor = self_431;
    let _e24: MotorDual = other_371;
    return MotorDual(((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.x, _e14.g0_.y, _e17.g0_.y, _e20.g0_.y) * _e24.g0_.xxwz) * vec4<f32>(0.0, -(1.0), -(1.0), 1.0))));
}

fn rotor_motor_dual_left_contraction(self_432: Rotor, other_372: MotorDual) -> MotorDual {
    var self_433: Rotor;
    var other_373: MotorDual;

    self_433 = self_432;
    other_373 = other_372;
    let _e4: Rotor = self_433;
    let _e8: MotorDual = other_373;
    let _e11: Rotor = self_433;
    let _e14: Rotor = self_433;
    let _e17: Rotor = self_433;
    let _e20: Rotor = self_433;
    let _e24: MotorDual = other_373;
    return MotorDual(((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.x, _e14.g0_.y, _e17.g0_.x, _e20.g0_.x) * vec4<f32>(_e24.g0_.x)) * vec4<f32>(0.0, -(1.0), 0.0, 0.0))));
}

fn rotor_squared_magnitude(self_434: Rotor) -> Scalar {
    var self_435: Rotor;

    self_435 = self_434;
    let _e4: Rotor = self_435;
    let _e5: Rotor = rotor_reversal(_e4);
    let _e6: Rotor = self_435;
    let _e8: Rotor = self_435;
    let _e9: Rotor = rotor_reversal(_e8);
    let _e10: Scalar = rotor_rotor_scalar_product(_e6, _e9);
    return _e10;
}

fn rotor_magnitude(self_436: Rotor) -> Scalar {
    var self_437: Rotor;

    self_437 = self_436;
    let _e3: Rotor = self_437;
    let _e4: Scalar = rotor_squared_magnitude(_e3);
    let _e7: Rotor = self_437;
    let _e8: Scalar = rotor_squared_magnitude(_e7);
    return Scalar(sqrt(_e8.g0_));
}

fn rotor_scale(self_438: Rotor, other_374: f32) -> Rotor {
    var self_439: Rotor;
    var other_375: f32;

    self_439 = self_438;
    other_375 = other_374;
    let _e5: f32 = other_375;
    let _e7: Rotor = self_439;
    let _e8: f32 = other_375;
    let _e10: Rotor = rotor_scalar_geometric_product(_e7, Scalar(_e8));
    return _e10;
}

fn rotor_signum(self_440: Rotor) -> Rotor {
    var self_441: Rotor;

    self_441 = self_440;
    let _e5: Rotor = self_441;
    let _e6: Scalar = rotor_magnitude(_e5);
    let _e10: Rotor = self_441;
    let _e13: Rotor = self_441;
    let _e14: Scalar = rotor_magnitude(_e13);
    let _e18: Rotor = rotor_scalar_geometric_product(_e10, Scalar((1.0 / _e14.g0_)));
    return _e18;
}

fn rotor_inverse(self_442: Rotor) -> Rotor {
    var self_443: Rotor;

    self_443 = self_442;
    let _e3: Rotor = self_443;
    let _e4: Rotor = rotor_reversal(_e3);
    let _e7: Rotor = self_443;
    let _e8: Scalar = rotor_squared_magnitude(_e7);
    let _e13: Rotor = self_443;
    let _e14: Rotor = rotor_reversal(_e13);
    let _e17: Rotor = self_443;
    let _e18: Scalar = rotor_squared_magnitude(_e17);
    let _e22: Rotor = rotor_scalar_geometric_product(_e14, Scalar((1.0 / _e18.g0_)));
    return _e22;
}

fn point_zero() -> Point {
    return Point(vec3<f32>(0.0));
}

fn point_one() -> Point {
    return Point(vec3<f32>(0.0));
}

fn point_neg(self_444: Point) -> Point {
    var self_445: Point;

    self_445 = self_444;
    let _e2: Point = self_445;
    return Point((_e2.g0_ * vec3<f32>(-(1.0))));
}

fn point_automorphism(self_446: Point) -> Point {
    var self_447: Point;

    self_447 = self_446;
    let _e2: Point = self_447;
    return Point(_e2.g0_);
}

fn point_reversal(self_448: Point) -> Point {
    var self_449: Point;

    self_449 = self_448;
    let _e2: Point = self_449;
    return Point((_e2.g0_ * vec3<f32>(-(1.0))));
}

fn point_conjugation(self_450: Point) -> Point {
    var self_451: Point;

    self_451 = self_450;
    let _e2: Point = self_451;
    return Point((_e2.g0_ * vec3<f32>(-(1.0))));
}

fn point_dual(self_452: Point) -> Plane {
    var self_453: Point;

    self_453 = self_452;
    let _e2: Point = self_453;
    return Plane(_e2.g0_);
}

fn point_scalar_add(self_454: Point, other_376: Scalar) -> Motor {
    var self_455: Point;
    var other_377: Scalar;

    self_455 = self_454;
    other_377 = other_376;
    let _e4: Point = self_455;
    let _e7: Point = self_455;
    let _e10: Point = self_455;
    let _e13: Point = self_455;
    let _e23: Scalar = other_377;
    return Motor(((vec4<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.y, _e13.g0_.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0)) + (vec4<f32>(_e23.g0_) * vec4<f32>(1.0, 0.0, 0.0, 0.0))));
}

fn point_scalar_sub(self_456: Point, other_378: Scalar) -> Motor {
    var self_457: Point;
    var other_379: Scalar;

    self_457 = self_456;
    other_379 = other_378;
    let _e4: Point = self_457;
    let _e7: Point = self_457;
    let _e10: Point = self_457;
    let _e13: Point = self_457;
    let _e23: Scalar = other_379;
    return Motor(((vec4<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.y, _e13.g0_.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0)) - (vec4<f32>(_e23.g0_) * vec4<f32>(1.0, 0.0, 0.0, 0.0))));
}

fn point_scalar_geometric_product(self_458: Point, other_380: Scalar) -> Point {
    var self_459: Point;
    var other_381: Scalar;

    self_459 = self_458;
    other_381 = other_380;
    let _e4: Point = self_459;
    let _e6: Scalar = other_381;
    return Point((_e4.g0_ * vec3<f32>(_e6.g0_)));
}

fn point_scalar_outer_product(self_460: Point, other_382: Scalar) -> Point {
    var self_461: Point;
    var other_383: Scalar;

    self_461 = self_460;
    other_383 = other_382;
    let _e4: Point = self_461;
    let _e6: Scalar = other_383;
    return Point((_e4.g0_ * vec3<f32>(_e6.g0_)));
}

fn point_scalar_inner_product(self_462: Point, other_384: Scalar) -> Point {
    var self_463: Point;
    var other_385: Scalar;

    self_463 = self_462;
    other_385 = other_384;
    let _e4: Point = self_463;
    let _e6: Scalar = other_385;
    return Point((_e4.g0_ * vec3<f32>(_e6.g0_)));
}

fn point_scalar_right_contraction(self_464: Point, other_386: Scalar) -> Point {
    var self_465: Point;
    var other_387: Scalar;

    self_465 = self_464;
    other_387 = other_386;
    let _e4: Point = self_465;
    let _e6: Scalar = other_387;
    return Point((_e4.g0_ * vec3<f32>(_e6.g0_)));
}

fn point_multi_vector_add(self_466: Point, other_388: MultiVector) -> MultiVector {
    var self_467: Point;
    var other_389: MultiVector;

    self_467 = self_466;
    other_389 = other_388;
    let _e4: Point = self_467;
    let _e14: MultiVector = other_389;
    let _e17: Point = self_467;
    let _e20: Point = self_467;
    let _e23: Point = self_467;
    let _e26: Point = self_467;
    let _e36: MultiVector = other_389;
    return MultiVector(((vec4<f32>(_e4.g0_.x) * vec4<f32>(0.0, 1.0, 0.0, 0.0)) + _e14.g0_), ((vec4<f32>(_e17.g0_.x, _e20.g0_.x, _e23.g0_.y, _e26.g0_.z) * vec4<f32>(0.0, 0.0, 1.0, 1.0)) + _e36.g1_));
}

fn point_multi_vector_sub(self_468: Point, other_390: MultiVector) -> MultiVector {
    var self_469: Point;
    var other_391: MultiVector;

    self_469 = self_468;
    other_391 = other_390;
    let _e4: Point = self_469;
    let _e14: MultiVector = other_391;
    let _e17: Point = self_469;
    let _e20: Point = self_469;
    let _e23: Point = self_469;
    let _e26: Point = self_469;
    let _e36: MultiVector = other_391;
    return MultiVector(((vec4<f32>(_e4.g0_.x) * vec4<f32>(0.0, 1.0, 0.0, 0.0)) - _e14.g0_), ((vec4<f32>(_e17.g0_.x, _e20.g0_.x, _e23.g0_.y, _e26.g0_.z) * vec4<f32>(0.0, 0.0, 1.0, 1.0)) - _e36.g1_));
}

fn point_multi_vector_geometric_product(self_470: Point, other_392: MultiVector) -> MultiVector {
    var self_471: Point;
    var other_393: MultiVector;

    self_471 = self_470;
    other_393 = other_392;
    let _e4: Point = self_471;
    let _e8: MultiVector = other_393;
    let _e20: Point = self_471;
    let _e24: MultiVector = other_393;
    let _e36: Point = self_471;
    let _e40: MultiVector = other_393;
    let _e52: Point = self_471;
    let _e56: MultiVector = other_393;
    return MultiVector(((vec4<f32>(_e4.g0_.x) * _e8.g0_.yxwz) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0))), ((((vec4<f32>(_e20.g0_.x) * _e24.g1_.yxwz) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0)) + ((vec4<f32>(_e36.g0_.y) * _e40.g0_.zwxy) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e52.g0_.z) * _e56.g0_.wzyx) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0))));
}

fn point_multi_vector_scalar_product(self_472: Point, other_394: MultiVector) -> Scalar {
    var self_473: Point;
    var other_395: MultiVector;

    self_473 = self_472;
    other_395 = other_394;
    let _e5: Point = self_473;
    let _e8: MultiVector = other_395;
    return Scalar((0.0 - (_e5.g0_.x * _e8.g0_.y)));
}

fn point_rotor_add(self_474: Point, other_396: Rotor) -> Motor {
    var self_475: Point;
    var other_397: Rotor;

    self_475 = self_474;
    other_397 = other_396;
    let _e4: Point = self_475;
    let _e7: Point = self_475;
    let _e10: Point = self_475;
    let _e13: Point = self_475;
    let _e23: Rotor = other_397;
    let _e26: Rotor = other_397;
    let _e29: Rotor = other_397;
    let _e32: Rotor = other_397;
    return Motor(((vec4<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.y, _e13.g0_.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0)) + (vec4<f32>(_e23.g0_.x, _e26.g0_.y, _e29.g0_.x, _e32.g0_.x) * vec4<f32>(1.0, 1.0, 0.0, 0.0))));
}

fn point_rotor_sub(self_476: Point, other_398: Rotor) -> Motor {
    var self_477: Point;
    var other_399: Rotor;

    self_477 = self_476;
    other_399 = other_398;
    let _e4: Point = self_477;
    let _e7: Point = self_477;
    let _e10: Point = self_477;
    let _e13: Point = self_477;
    let _e23: Rotor = other_399;
    let _e26: Rotor = other_399;
    let _e29: Rotor = other_399;
    let _e32: Rotor = other_399;
    return Motor(((vec4<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.y, _e13.g0_.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0)) - (vec4<f32>(_e23.g0_.x, _e26.g0_.y, _e29.g0_.x, _e32.g0_.x) * vec4<f32>(1.0, 1.0, 0.0, 0.0))));
}

fn point_rotor_geometric_product(self_478: Point, other_400: Rotor) -> Motor {
    var self_479: Point;
    var other_401: Rotor;

    self_479 = self_478;
    other_401 = other_400;
    let _e4: Point = self_479;
    let _e8: Rotor = other_401;
    let _e11: Rotor = other_401;
    let _e14: Rotor = other_401;
    let _e17: Rotor = other_401;
    let _e28: Point = self_479;
    let _e31: Point = self_479;
    let _e34: Point = self_479;
    let _e37: Point = self_479;
    let _e41: Rotor = other_401;
    let _e44: Rotor = other_401;
    let _e47: Rotor = other_401;
    let _e50: Rotor = other_401;
    return Motor((((vec4<f32>(_e4.g0_.z) * vec4<f32>(_e8.g0_.y, _e11.g0_.y, _e14.g0_.y, _e17.g0_.x)) * vec4<f32>(0.0, 0.0, 1.0, 1.0)) + ((vec4<f32>(_e28.g0_.x, _e31.g0_.x, _e34.g0_.y, _e37.g0_.y) * vec4<f32>(_e41.g0_.y, _e44.g0_.x, _e47.g0_.x, _e50.g0_.y)) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))));
}

fn point_rotor_outer_product(self_480: Point, other_402: Rotor) -> Point {
    var self_481: Point;
    var other_403: Rotor;

    self_481 = self_480;
    other_403 = other_402;
    let _e4: Point = self_481;
    let _e6: Rotor = other_403;
    return Point((_e4.g0_ * vec3<f32>(_e6.g0_.x)));
}

fn point_rotor_inner_product(self_482: Point, other_404: Rotor) -> Motor {
    var self_483: Point;
    var other_405: Rotor;

    self_483 = self_482;
    other_405 = other_404;
    let _e4: Point = self_483;
    let _e7: Point = self_483;
    let _e10: Point = self_483;
    let _e13: Point = self_483;
    let _e17: Rotor = other_405;
    let _e20: Rotor = other_405;
    let _e23: Rotor = other_405;
    let _e26: Rotor = other_405;
    return Motor(((vec4<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.y, _e13.g0_.z) * vec4<f32>(_e17.g0_.y, _e20.g0_.x, _e23.g0_.x, _e26.g0_.x)) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0)));
}

fn point_rotor_left_contraction(self_484: Point, other_406: Rotor) -> Scalar {
    var self_485: Point;
    var other_407: Rotor;

    self_485 = self_484;
    other_407 = other_406;
    let _e5: Point = self_485;
    let _e8: Rotor = other_407;
    return Scalar((0.0 - (_e5.g0_.x * _e8.g0_.y)));
}

fn point_rotor_right_contraction(self_486: Point, other_408: Rotor) -> Motor {
    var self_487: Point;
    var other_409: Rotor;

    self_487 = self_486;
    other_409 = other_408;
    let _e4: Point = self_487;
    let _e7: Point = self_487;
    let _e10: Point = self_487;
    let _e13: Point = self_487;
    let _e17: Rotor = other_409;
    let _e20: Rotor = other_409;
    let _e23: Rotor = other_409;
    let _e26: Rotor = other_409;
    return Motor(((vec4<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.y, _e13.g0_.z) * vec4<f32>(_e17.g0_.y, _e20.g0_.x, _e23.g0_.x, _e26.g0_.x)) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0)));
}

fn point_rotor_scalar_product(self_488: Point, other_410: Rotor) -> Scalar {
    var self_489: Point;
    var other_411: Rotor;

    self_489 = self_488;
    other_411 = other_410;
    let _e5: Point = self_489;
    let _e8: Rotor = other_411;
    return Scalar((0.0 - (_e5.g0_.x * _e8.g0_.y)));
}

fn point_point_add(self_490: Point, other_412: Point) -> Point {
    var self_491: Point;
    var other_413: Point;

    self_491 = self_490;
    other_413 = other_412;
    let _e4: Point = self_491;
    let _e6: Point = other_413;
    return Point((_e4.g0_ + _e6.g0_));
}

fn point_point_sub(self_492: Point, other_414: Point) -> Point {
    var self_493: Point;
    var other_415: Point;

    self_493 = self_492;
    other_415 = other_414;
    let _e4: Point = self_493;
    let _e6: Point = other_415;
    return Point((_e4.g0_ - _e6.g0_));
}

fn point_point_mul(self_494: Point, other_416: Point) -> Point {
    var self_495: Point;
    var other_417: Point;

    self_495 = self_494;
    other_417 = other_416;
    let _e4: Point = self_495;
    let _e6: Point = other_417;
    return Point((_e4.g0_ * _e6.g0_));
}

fn point_point_div(self_496: Point, other_418: Point) -> Point {
    var self_497: Point;
    var other_419: Point;

    self_497 = self_496;
    other_419 = other_418;
    let _e4: Point = self_497;
    let _e7: Point = self_497;
    let _e10: Point = self_497;
    let _e19: Point = other_419;
    let _e22: Point = other_419;
    let _e25: Point = other_419;
    return Point((((vec3<f32>(_e4.g0_.x, _e7.g0_.y, _e10.g0_.z) * vec3<f32>(1.0, 1.0, 1.0)) / vec3<f32>(_e19.g0_.x, _e22.g0_.y, _e25.g0_.z)) * vec3<f32>(1.0, 1.0, 1.0)));
}

fn point_point_geometric_product(self_498: Point, other_420: Point) -> Translator {
    var self_499: Point;
    var other_421: Point;

    self_499 = self_498;
    other_421 = other_420;
    let _e4: Point = self_499;
    let _e8: Point = other_421;
    let _e19: Point = self_499;
    let _e22: Point = other_421;
    return Translator((((vec3<f32>(_e4.g0_.x) * _e8.g0_.xzy) * vec3<f32>(-(1.0), -(1.0), 1.0)) + ((_e19.g0_.xzy * vec3<f32>(_e22.g0_.x)) * vec3<f32>(0.0, 1.0, -(1.0)))));
}

fn point_point_regressive_product(self_500: Point, other_422: Point) -> Plane {
    var self_501: Point;
    var other_423: Point;

    self_501 = self_500;
    other_423 = other_422;
    let _e4: Point = self_501;
    let _e8: Point = other_423;
    let _e18: Point = self_501;
    let _e22: Point = other_423;
    let _e33: Point = self_501;
    let _e37: Point = other_423;
    return Plane(((((vec3<f32>(_e4.g0_.y) * _e8.g0_.zzx) * vec3<f32>(-(1.0), 0.0, 1.0)) + ((vec3<f32>(_e18.g0_.z) * _e22.g0_.yxy) * vec3<f32>(1.0, -(1.0), 0.0))) + ((vec3<f32>(_e33.g0_.x) * _e37.g0_.xzy) * vec3<f32>(0.0, 1.0, -(1.0)))));
}

fn point_point_inner_product(self_502: Point, other_424: Point) -> Scalar {
    var self_503: Point;
    var other_425: Point;

    self_503 = self_502;
    other_425 = other_424;
    let _e5: Point = self_503;
    let _e8: Point = other_425;
    return Scalar((0.0 - (_e5.g0_.x * _e8.g0_.x)));
}

fn point_point_left_contraction(self_504: Point, other_426: Point) -> Scalar {
    var self_505: Point;
    var other_427: Point;

    self_505 = self_504;
    other_427 = other_426;
    let _e5: Point = self_505;
    let _e8: Point = other_427;
    return Scalar((0.0 - (_e5.g0_.x * _e8.g0_.x)));
}

fn point_point_right_contraction(self_506: Point, other_428: Point) -> Scalar {
    var self_507: Point;
    var other_429: Point;

    self_507 = self_506;
    other_429 = other_428;
    let _e5: Point = self_507;
    let _e8: Point = other_429;
    return Scalar((0.0 - (_e5.g0_.x * _e8.g0_.x)));
}

fn point_point_scalar_product(self_508: Point, other_430: Point) -> Scalar {
    var self_509: Point;
    var other_431: Point;

    self_509 = self_508;
    other_431 = other_430;
    let _e5: Point = self_509;
    let _e8: Point = other_431;
    return Scalar((0.0 - (_e5.g0_.x * _e8.g0_.x)));
}

fn point_ideal_point_into(self_510: Point) -> IdealPoint {
    var self_511: Point;

    self_511 = self_510;
    let _e2: Point = self_511;
    let _e5: Point = self_511;
    return IdealPoint(vec2<f32>(_e2.g0_.y, _e5.g0_.z));
}

fn point_ideal_point_add(self_512: Point, other_432: IdealPoint) -> Point {
    var self_513: Point;
    var other_433: IdealPoint;

    self_513 = self_512;
    other_433 = other_432;
    let _e4: Point = self_513;
    let _e6: IdealPoint = other_433;
    let _e9: IdealPoint = other_433;
    let _e12: IdealPoint = other_433;
    return Point((_e4.g0_ + (vec3<f32>(_e6.g0_.x, _e9.g0_.x, _e12.g0_.y) * vec3<f32>(0.0, 1.0, 1.0))));
}

fn point_ideal_point_sub(self_514: Point, other_434: IdealPoint) -> Point {
    var self_515: Point;
    var other_435: IdealPoint;

    self_515 = self_514;
    other_435 = other_434;
    let _e4: Point = self_515;
    let _e6: IdealPoint = other_435;
    let _e9: IdealPoint = other_435;
    let _e12: IdealPoint = other_435;
    return Point((_e4.g0_ - (vec3<f32>(_e6.g0_.x, _e9.g0_.x, _e12.g0_.y) * vec3<f32>(0.0, 1.0, 1.0))));
}

fn point_ideal_point_geometric_product(self_516: Point, other_436: IdealPoint) -> IdealPoint {
    var self_517: Point;
    var other_437: IdealPoint;

    self_517 = self_516;
    other_437 = other_436;
    let _e4: Point = self_517;
    let _e8: IdealPoint = other_437;
    return IdealPoint(((vec2<f32>(_e4.g0_.x) * _e8.g0_.yx) * vec2<f32>(-(1.0), 1.0)));
}

fn point_ideal_point_regressive_product(self_518: Point, other_438: IdealPoint) -> Plane {
    var self_519: Point;
    var other_439: IdealPoint;

    self_519 = self_518;
    other_439 = other_438;
    let _e4: Point = self_519;
    let _e8: IdealPoint = other_439;
    let _e18: Point = self_519;
    let _e21: IdealPoint = other_439;
    let _e24: IdealPoint = other_439;
    let _e27: IdealPoint = other_439;
    return Plane((((vec3<f32>(_e4.g0_.z) * vec3<f32>(_e8.g0_.x)) * vec3<f32>(1.0, 0.0, 0.0)) + ((_e18.g0_.yxx * vec3<f32>(_e21.g0_.y, _e24.g0_.y, _e27.g0_.x)) * vec3<f32>(-(1.0), 1.0, -(1.0)))));
}

fn point_plane_geometric_product(self_520: Point, other_440: Plane) -> MotorDual {
    var self_521: Point;
    var other_441: Plane;

    self_521 = self_520;
    other_441 = other_440;
    let _e4: Point = self_521;
    let _e8: Plane = other_441;
    let _e11: Plane = other_441;
    let _e14: Plane = other_441;
    let _e17: Plane = other_441;
    let _e28: Point = self_521;
    let _e32: Plane = other_441;
    let _e35: Plane = other_441;
    let _e38: Plane = other_441;
    let _e41: Plane = other_441;
    let _e54: Point = self_521;
    let _e58: Plane = other_441;
    let _e61: Plane = other_441;
    let _e64: Plane = other_441;
    let _e67: Plane = other_441;
    return MotorDual(((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y, _e11.g0_.z, _e14.g0_.y, _e17.g0_.y)) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) + ((vec4<f32>(_e28.g0_.z) * vec4<f32>(_e32.g0_.z, _e35.g0_.y, _e38.g0_.z, _e41.g0_.z)) * vec4<f32>(1.0, -(1.0), 0.0, 0.0))) + ((vec4<f32>(_e54.g0_.x) * vec4<f32>(_e58.g0_.x, _e61.g0_.x, _e64.g0_.z, _e67.g0_.y)) * vec4<f32>(1.0, 0.0, -(1.0), 1.0))));
}

fn point_plane_regressive_product(self_522: Point, other_442: Plane) -> Scalar {
    var self_523: Point;
    var other_443: Plane;

    self_523 = self_522;
    other_443 = other_442;
    let _e4: Point = self_523;
    let _e7: Plane = other_443;
    let _e11: Point = self_523;
    let _e14: Plane = other_443;
    let _e19: Point = self_523;
    let _e22: Plane = other_443;
    return Scalar((((_e4.g0_.x * _e7.g0_.x) + (_e11.g0_.y * _e14.g0_.y)) + (_e19.g0_.z * _e22.g0_.z)));
}

fn point_plane_inner_product(self_524: Point, other_444: Plane) -> Plane {
    var self_525: Point;
    var other_445: Plane;

    self_525 = self_524;
    other_445 = other_444;
    let _e4: Point = self_525;
    let _e8: Plane = other_445;
    let _e19: Point = self_525;
    let _e22: Plane = other_445;
    return Plane((((vec3<f32>(_e4.g0_.z) * vec3<f32>(_e8.g0_.y)) * vec3<f32>(-(1.0), 0.0, 0.0)) + ((_e19.g0_.yxx * _e22.g0_.zzy) * vec3<f32>(1.0, -(1.0), 1.0))));
}

fn point_plane_right_contraction(self_526: Point, other_446: Plane) -> Plane {
    var self_527: Point;
    var other_447: Plane;

    self_527 = self_526;
    other_447 = other_446;
    let _e4: Point = self_527;
    let _e8: Plane = other_447;
    let _e19: Point = self_527;
    let _e22: Plane = other_447;
    return Plane((((vec3<f32>(_e4.g0_.z) * vec3<f32>(_e8.g0_.y)) * vec3<f32>(-(1.0), 0.0, 0.0)) + ((_e19.g0_.yxx * _e22.g0_.zzy) * vec3<f32>(1.0, -(1.0), 1.0))));
}

fn point_translator_add(self_528: Point, other_448: Translator) -> Motor {
    var self_529: Point;
    var other_449: Translator;

    self_529 = self_528;
    other_449 = other_448;
    let _e4: Point = self_529;
    let _e7: Point = self_529;
    let _e10: Point = self_529;
    let _e13: Point = self_529;
    let _e23: Translator = other_449;
    let _e26: Translator = other_449;
    let _e29: Translator = other_449;
    let _e32: Translator = other_449;
    return Motor(((vec4<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.y, _e13.g0_.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0)) + (vec4<f32>(_e23.g0_.x, _e26.g0_.x, _e29.g0_.y, _e32.g0_.z) * vec4<f32>(1.0, 0.0, 1.0, 1.0))));
}

fn point_translator_sub(self_530: Point, other_450: Translator) -> Motor {
    var self_531: Point;
    var other_451: Translator;

    self_531 = self_530;
    other_451 = other_450;
    let _e4: Point = self_531;
    let _e7: Point = self_531;
    let _e10: Point = self_531;
    let _e13: Point = self_531;
    let _e23: Translator = other_451;
    let _e26: Translator = other_451;
    let _e29: Translator = other_451;
    let _e32: Translator = other_451;
    return Motor(((vec4<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.y, _e13.g0_.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0)) - (vec4<f32>(_e23.g0_.x, _e26.g0_.x, _e29.g0_.y, _e32.g0_.z) * vec4<f32>(1.0, 0.0, 1.0, 1.0))));
}

fn point_translator_geometric_product(self_532: Point, other_452: Translator) -> Point {
    var self_533: Point;
    var other_453: Translator;

    self_533 = self_532;
    other_453 = other_452;
    let _e4: Point = self_533;
    let _e8: Translator = other_453;
    let _e18: Point = self_533;
    let _e20: Translator = other_453;
    return Point((((vec3<f32>(_e4.g0_.x) * _e8.g0_.xzy) * vec3<f32>(1.0, -(1.0), 1.0)) + ((_e18.g0_ * vec3<f32>(_e20.g0_.x)) * vec3<f32>(0.0, 1.0, 1.0))));
}

fn point_translator_regressive_product(self_534: Point, other_454: Translator) -> Plane {
    var self_535: Point;
    var other_455: Translator;

    self_535 = self_534;
    other_455 = other_454;
    let _e4: Point = self_535;
    let _e8: Translator = other_455;
    let _e18: Point = self_535;
    let _e21: Translator = other_455;
    return Plane((((vec3<f32>(_e4.g0_.z) * vec3<f32>(_e8.g0_.y)) * vec3<f32>(1.0, 0.0, 0.0)) + ((_e18.g0_.yxx * _e21.g0_.zzy) * vec3<f32>(-(1.0), 1.0, -(1.0)))));
}

fn point_translator_outer_product(self_536: Point, other_456: Translator) -> Point {
    var self_537: Point;
    var other_457: Translator;

    self_537 = self_536;
    other_457 = other_456;
    let _e4: Point = self_537;
    let _e6: Translator = other_457;
    return Point((_e4.g0_ * vec3<f32>(_e6.g0_.x)));
}

fn point_translator_inner_product(self_538: Point, other_458: Translator) -> Point {
    var self_539: Point;
    var other_459: Translator;

    self_539 = self_538;
    other_459 = other_458;
    let _e4: Point = self_539;
    let _e6: Translator = other_459;
    return Point((_e4.g0_ * vec3<f32>(_e6.g0_.x)));
}

fn point_translator_right_contraction(self_540: Point, other_460: Translator) -> Point {
    var self_541: Point;
    var other_461: Translator;

    self_541 = self_540;
    other_461 = other_460;
    let _e4: Point = self_541;
    let _e6: Translator = other_461;
    return Point((_e4.g0_ * vec3<f32>(_e6.g0_.x)));
}

fn point_motor_add(self_542: Point, other_462: Motor) -> Motor {
    var self_543: Point;
    var other_463: Motor;

    self_543 = self_542;
    other_463 = other_462;
    let _e4: Point = self_543;
    let _e7: Point = self_543;
    let _e10: Point = self_543;
    let _e13: Point = self_543;
    let _e23: Motor = other_463;
    return Motor(((vec4<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.y, _e13.g0_.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0)) + _e23.g0_));
}

fn point_motor_sub(self_544: Point, other_464: Motor) -> Motor {
    var self_545: Point;
    var other_465: Motor;

    self_545 = self_544;
    other_465 = other_464;
    let _e4: Point = self_545;
    let _e7: Point = self_545;
    let _e10: Point = self_545;
    let _e13: Point = self_545;
    let _e23: Motor = other_465;
    return Motor(((vec4<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.y, _e13.g0_.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0)) - _e23.g0_));
}

fn point_motor_geometric_product(self_546: Point, other_466: Motor) -> Motor {
    var self_547: Point;
    var other_467: Motor;

    self_547 = self_546;
    other_467 = other_466;
    let _e4: Point = self_547;
    let _e8: Motor = other_467;
    let _e20: Point = self_547;
    let _e24: Motor = other_467;
    let _e35: Point = self_547;
    let _e38: Point = self_547;
    let _e41: Point = self_547;
    let _e44: Point = self_547;
    let _e48: Motor = other_467;
    return Motor(((((vec4<f32>(_e4.g0_.x) * _e8.g0_.yxwz) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0)) + ((vec4<f32>(_e20.g0_.z) * _e24.g0_.yyyx) * vec4<f32>(0.0, 0.0, 1.0, 1.0))) + ((vec4<f32>(_e35.g0_.x, _e38.g0_.x, _e41.g0_.y, _e44.g0_.y) * _e48.g0_.xxxy) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))));
}

fn point_motor_regressive_product(self_548: Point, other_468: Motor) -> Plane {
    var self_549: Point;
    var other_469: Motor;

    self_549 = self_548;
    other_469 = other_468;
    let _e4: Point = self_549;
    let _e8: Motor = other_469;
    let _e11: Motor = other_469;
    let _e14: Motor = other_469;
    let _e25: Point = self_549;
    let _e29: Motor = other_469;
    let _e32: Motor = other_469;
    let _e35: Motor = other_469;
    let _e47: Point = self_549;
    let _e51: Motor = other_469;
    let _e54: Motor = other_469;
    let _e57: Motor = other_469;
    return Plane(((((vec3<f32>(_e4.g0_.y) * vec3<f32>(_e8.g0_.w, _e11.g0_.w, _e14.g0_.y)) * vec3<f32>(-(1.0), 0.0, 1.0)) + ((vec3<f32>(_e25.g0_.z) * vec3<f32>(_e29.g0_.z, _e32.g0_.y, _e35.g0_.z)) * vec3<f32>(1.0, -(1.0), 0.0))) + ((vec3<f32>(_e47.g0_.x) * vec3<f32>(_e51.g0_.x, _e54.g0_.w, _e57.g0_.z)) * vec3<f32>(0.0, 1.0, -(1.0)))));
}

fn point_motor_outer_product(self_550: Point, other_470: Motor) -> Point {
    var self_551: Point;
    var other_471: Motor;

    self_551 = self_550;
    other_471 = other_470;
    let _e4: Point = self_551;
    let _e6: Motor = other_471;
    return Point((_e4.g0_ * vec3<f32>(_e6.g0_.x)));
}

fn point_motor_inner_product(self_552: Point, other_472: Motor) -> Motor {
    var self_553: Point;
    var other_473: Motor;

    self_553 = self_552;
    other_473 = other_472;
    let _e4: Point = self_553;
    let _e7: Point = self_553;
    let _e10: Point = self_553;
    let _e13: Point = self_553;
    let _e17: Motor = other_473;
    return Motor(((vec4<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.y, _e13.g0_.z) * _e17.g0_.yxxx) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0)));
}

fn point_motor_left_contraction(self_554: Point, other_474: Motor) -> Scalar {
    var self_555: Point;
    var other_475: Motor;

    self_555 = self_554;
    other_475 = other_474;
    let _e5: Point = self_555;
    let _e8: Motor = other_475;
    return Scalar((0.0 - (_e5.g0_.x * _e8.g0_.y)));
}

fn point_motor_right_contraction(self_556: Point, other_476: Motor) -> Motor {
    var self_557: Point;
    var other_477: Motor;

    self_557 = self_556;
    other_477 = other_476;
    let _e4: Point = self_557;
    let _e7: Point = self_557;
    let _e10: Point = self_557;
    let _e13: Point = self_557;
    let _e17: Motor = other_477;
    return Motor(((vec4<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.y, _e13.g0_.z) * _e17.g0_.yxxx) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0)));
}

fn point_motor_scalar_product(self_558: Point, other_478: Motor) -> Scalar {
    var self_559: Point;
    var other_479: Motor;

    self_559 = self_558;
    other_479 = other_478;
    let _e5: Point = self_559;
    let _e8: Motor = other_479;
    return Scalar((0.0 - (_e5.g0_.x * _e8.g0_.y)));
}

fn point_motor_dual_geometric_product(self_560: Point, other_480: MotorDual) -> MotorDual {
    var self_561: Point;
    var other_481: MotorDual;

    self_561 = self_560;
    other_481 = other_480;
    let _e4: Point = self_561;
    let _e8: MotorDual = other_481;
    let _e20: Point = self_561;
    let _e24: MotorDual = other_481;
    let _e36: Point = self_561;
    let _e39: Point = self_561;
    let _e42: Point = self_561;
    let _e45: Point = self_561;
    let _e49: MotorDual = other_481;
    return MotorDual(((((vec4<f32>(_e4.g0_.x) * _e8.g0_.yxwz) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0)) + ((vec4<f32>(_e20.g0_.z) * _e24.g0_.wzww) * vec4<f32>(1.0, -(1.0), 0.0, 0.0))) + ((vec4<f32>(_e36.g0_.y, _e39.g0_.y, _e42.g0_.x, _e45.g0_.x) * _e49.g0_.zwxx) * vec4<f32>(1.0, 1.0, 0.0, 0.0))));
}

fn point_motor_dual_regressive_product(self_562: Point, other_482: MotorDual) -> Motor {
    var self_563: Point;
    var other_483: MotorDual;

    self_563 = self_562;
    other_483 = other_482;
    let _e4: Point = self_563;
    let _e8: MotorDual = other_483;
    let _e18: Point = self_563;
    let _e22: MotorDual = other_483;
    let _e33: Point = self_563;
    let _e37: MotorDual = other_483;
    return Motor(((((vec4<f32>(_e4.g0_.y) * _e8.g0_.zzxz) * vec4<f32>(1.0, 0.0, 1.0, 0.0)) + ((vec4<f32>(_e18.g0_.z) * _e22.g0_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e33.g0_.x) * _e37.g0_.yxxx) * vec4<f32>(1.0, 1.0, 0.0, 0.0))));
}

fn point_motor_dual_inner_product(self_564: Point, other_484: MotorDual) -> Plane {
    var self_565: Point;
    var other_485: MotorDual;

    self_565 = self_564;
    other_485 = other_484;
    let _e4: Point = self_565;
    let _e8: MotorDual = other_485;
    let _e11: MotorDual = other_485;
    let _e14: MotorDual = other_485;
    let _e26: Point = self_565;
    let _e30: MotorDual = other_485;
    let _e42: Point = self_565;
    let _e45: MotorDual = other_485;
    let _e48: MotorDual = other_485;
    let _e51: MotorDual = other_485;
    return Plane(((((vec3<f32>(_e4.g0_.x) * vec3<f32>(_e8.g0_.x, _e11.g0_.w, _e14.g0_.z)) * vec3<f32>(-(1.0), -(1.0), 1.0)) + ((vec3<f32>(_e26.g0_.z) * vec3<f32>(_e30.g0_.z)) * vec3<f32>(-(1.0), 0.0, 0.0))) + ((_e42.g0_.yxx * vec3<f32>(_e45.g0_.w, _e48.g0_.x, _e51.g0_.x)) * vec3<f32>(1.0, 0.0, 0.0))));
}

fn point_motor_dual_right_contraction(self_566: Point, other_486: MotorDual) -> Plane {
    var self_567: Point;
    var other_487: MotorDual;

    self_567 = self_566;
    other_487 = other_486;
    let _e4: Point = self_567;
    let _e8: MotorDual = other_487;
    let _e19: Point = self_567;
    let _e22: MotorDual = other_487;
    let _e25: MotorDual = other_487;
    let _e28: MotorDual = other_487;
    return Plane((((vec3<f32>(_e4.g0_.z) * vec3<f32>(_e8.g0_.z)) * vec3<f32>(-(1.0), 0.0, 0.0)) + ((_e19.g0_.yxx * vec3<f32>(_e22.g0_.w, _e25.g0_.w, _e28.g0_.z)) * vec3<f32>(1.0, -(1.0), 1.0))));
}

fn point_squared_magnitude(self_568: Point) -> Scalar {
    var self_569: Point;

    self_569 = self_568;
    let _e4: Point = self_569;
    let _e5: Point = point_reversal(_e4);
    let _e6: Point = self_569;
    let _e8: Point = self_569;
    let _e9: Point = point_reversal(_e8);
    let _e10: Scalar = point_point_scalar_product(_e6, _e9);
    return _e10;
}

fn point_magnitude(self_570: Point) -> Scalar {
    var self_571: Point;

    self_571 = self_570;
    let _e3: Point = self_571;
    let _e4: Scalar = point_squared_magnitude(_e3);
    let _e7: Point = self_571;
    let _e8: Scalar = point_squared_magnitude(_e7);
    return Scalar(sqrt(_e8.g0_));
}

fn point_scale(self_572: Point, other_488: f32) -> Point {
    var self_573: Point;
    var other_489: f32;

    self_573 = self_572;
    other_489 = other_488;
    let _e5: f32 = other_489;
    let _e7: Point = self_573;
    let _e8: f32 = other_489;
    let _e10: Point = point_scalar_geometric_product(_e7, Scalar(_e8));
    return _e10;
}

fn point_signum(self_574: Point) -> Point {
    var self_575: Point;

    self_575 = self_574;
    let _e5: Point = self_575;
    let _e6: Scalar = point_magnitude(_e5);
    let _e10: Point = self_575;
    let _e13: Point = self_575;
    let _e14: Scalar = point_magnitude(_e13);
    let _e18: Point = point_scalar_geometric_product(_e10, Scalar((1.0 / _e14.g0_)));
    return _e18;
}

fn point_inverse(self_576: Point) -> Point {
    var self_577: Point;

    self_577 = self_576;
    let _e3: Point = self_577;
    let _e4: Point = point_reversal(_e3);
    let _e7: Point = self_577;
    let _e8: Scalar = point_squared_magnitude(_e7);
    let _e13: Point = self_577;
    let _e14: Point = point_reversal(_e13);
    let _e17: Point = self_577;
    let _e18: Scalar = point_squared_magnitude(_e17);
    let _e22: Point = point_scalar_geometric_product(_e14, Scalar((1.0 / _e18.g0_)));
    return _e22;
}

fn ideal_point_zero() -> IdealPoint {
    return IdealPoint(vec2<f32>(0.0));
}

fn ideal_point_one() -> IdealPoint {
    return IdealPoint(vec2<f32>(0.0));
}

fn ideal_point_neg(self_578: IdealPoint) -> IdealPoint {
    var self_579: IdealPoint;

    self_579 = self_578;
    let _e2: IdealPoint = self_579;
    return IdealPoint((_e2.g0_ * vec2<f32>(-(1.0))));
}

fn ideal_point_automorphism(self_580: IdealPoint) -> IdealPoint {
    var self_581: IdealPoint;

    self_581 = self_580;
    let _e2: IdealPoint = self_581;
    return IdealPoint(_e2.g0_);
}

fn ideal_point_reversal(self_582: IdealPoint) -> IdealPoint {
    var self_583: IdealPoint;

    self_583 = self_582;
    let _e2: IdealPoint = self_583;
    return IdealPoint((_e2.g0_ * vec2<f32>(-(1.0))));
}

fn ideal_point_conjugation(self_584: IdealPoint) -> IdealPoint {
    var self_585: IdealPoint;

    self_585 = self_584;
    let _e2: IdealPoint = self_585;
    return IdealPoint((_e2.g0_ * vec2<f32>(-(1.0))));
}

fn ideal_point_scalar_add(self_586: IdealPoint, other_490: Scalar) -> Translator {
    var self_587: IdealPoint;
    var other_491: Scalar;

    self_587 = self_586;
    other_491 = other_490;
    let _e4: IdealPoint = self_587;
    let _e7: IdealPoint = self_587;
    let _e10: IdealPoint = self_587;
    let _e19: Scalar = other_491;
    return Translator(((vec3<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.y) * vec3<f32>(0.0, 1.0, 1.0)) + (vec3<f32>(_e19.g0_) * vec3<f32>(1.0, 0.0, 0.0))));
}

fn ideal_point_scalar_sub(self_588: IdealPoint, other_492: Scalar) -> Translator {
    var self_589: IdealPoint;
    var other_493: Scalar;

    self_589 = self_588;
    other_493 = other_492;
    let _e4: IdealPoint = self_589;
    let _e7: IdealPoint = self_589;
    let _e10: IdealPoint = self_589;
    let _e19: Scalar = other_493;
    return Translator(((vec3<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.y) * vec3<f32>(0.0, 1.0, 1.0)) - (vec3<f32>(_e19.g0_) * vec3<f32>(1.0, 0.0, 0.0))));
}

fn ideal_point_scalar_geometric_product(self_590: IdealPoint, other_494: Scalar) -> IdealPoint {
    var self_591: IdealPoint;
    var other_495: Scalar;

    self_591 = self_590;
    other_495 = other_494;
    let _e4: IdealPoint = self_591;
    let _e6: Scalar = other_495;
    return IdealPoint((_e4.g0_ * vec2<f32>(_e6.g0_)));
}

fn ideal_point_scalar_outer_product(self_592: IdealPoint, other_496: Scalar) -> IdealPoint {
    var self_593: IdealPoint;
    var other_497: Scalar;

    self_593 = self_592;
    other_497 = other_496;
    let _e4: IdealPoint = self_593;
    let _e6: Scalar = other_497;
    return IdealPoint((_e4.g0_ * vec2<f32>(_e6.g0_)));
}

fn ideal_point_scalar_inner_product(self_594: IdealPoint, other_498: Scalar) -> IdealPoint {
    var self_595: IdealPoint;
    var other_499: Scalar;

    self_595 = self_594;
    other_499 = other_498;
    let _e4: IdealPoint = self_595;
    let _e6: Scalar = other_499;
    return IdealPoint((_e4.g0_ * vec2<f32>(_e6.g0_)));
}

fn ideal_point_scalar_right_contraction(self_596: IdealPoint, other_500: Scalar) -> IdealPoint {
    var self_597: IdealPoint;
    var other_501: Scalar;

    self_597 = self_596;
    other_501 = other_500;
    let _e4: IdealPoint = self_597;
    let _e6: Scalar = other_501;
    return IdealPoint((_e4.g0_ * vec2<f32>(_e6.g0_)));
}

fn ideal_point_multi_vector_add(self_598: IdealPoint, other_502: MultiVector) -> MultiVector {
    var self_599: IdealPoint;
    var other_503: MultiVector;

    self_599 = self_598;
    other_503 = other_502;
    let _e4: MultiVector = other_503;
    let _e6: IdealPoint = self_599;
    let _e9: IdealPoint = self_599;
    let _e12: IdealPoint = self_599;
    let _e15: IdealPoint = self_599;
    let _e25: MultiVector = other_503;
    return MultiVector(_e4.g0_, ((vec4<f32>(_e6.g0_.x, _e9.g0_.x, _e12.g0_.x, _e15.g0_.y) * vec4<f32>(0.0, 0.0, 1.0, 1.0)) + _e25.g1_));
}

fn ideal_point_multi_vector_sub(self_600: IdealPoint, other_504: MultiVector) -> MultiVector {
    var self_601: IdealPoint;
    var other_505: MultiVector;

    self_601 = self_600;
    other_505 = other_504;
    let _e6: MultiVector = other_505;
    let _e9: IdealPoint = self_601;
    let _e12: IdealPoint = self_601;
    let _e15: IdealPoint = self_601;
    let _e18: IdealPoint = self_601;
    let _e28: MultiVector = other_505;
    return MultiVector((vec4<f32>(0.0) - _e6.g0_), ((vec4<f32>(_e9.g0_.x, _e12.g0_.x, _e15.g0_.x, _e18.g0_.y) * vec4<f32>(0.0, 0.0, 1.0, 1.0)) - _e28.g1_));
}

fn ideal_point_rotor_add(self_602: IdealPoint, other_506: Rotor) -> Motor {
    var self_603: IdealPoint;
    var other_507: Rotor;

    self_603 = self_602;
    other_507 = other_506;
    let _e4: IdealPoint = self_603;
    let _e7: IdealPoint = self_603;
    let _e10: IdealPoint = self_603;
    let _e13: IdealPoint = self_603;
    let _e23: Rotor = other_507;
    let _e26: Rotor = other_507;
    let _e29: Rotor = other_507;
    let _e32: Rotor = other_507;
    return Motor(((vec4<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.x, _e13.g0_.y) * vec4<f32>(0.0, 0.0, 1.0, 1.0)) + (vec4<f32>(_e23.g0_.x, _e26.g0_.y, _e29.g0_.x, _e32.g0_.x) * vec4<f32>(1.0, 1.0, 0.0, 0.0))));
}

fn ideal_point_rotor_sub(self_604: IdealPoint, other_508: Rotor) -> Motor {
    var self_605: IdealPoint;
    var other_509: Rotor;

    self_605 = self_604;
    other_509 = other_508;
    let _e4: IdealPoint = self_605;
    let _e7: IdealPoint = self_605;
    let _e10: IdealPoint = self_605;
    let _e13: IdealPoint = self_605;
    let _e23: Rotor = other_509;
    let _e26: Rotor = other_509;
    let _e29: Rotor = other_509;
    let _e32: Rotor = other_509;
    return Motor(((vec4<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.x, _e13.g0_.y) * vec4<f32>(0.0, 0.0, 1.0, 1.0)) - (vec4<f32>(_e23.g0_.x, _e26.g0_.y, _e29.g0_.x, _e32.g0_.x) * vec4<f32>(1.0, 1.0, 0.0, 0.0))));
}

fn ideal_point_rotor_geometric_product(self_606: IdealPoint, other_510: Rotor) -> IdealPoint {
    var self_607: IdealPoint;
    var other_511: Rotor;

    self_607 = self_606;
    other_511 = other_510;
    let _e4: IdealPoint = self_607;
    let _e8: Rotor = other_511;
    let _e16: IdealPoint = self_607;
    let _e20: Rotor = other_511;
    return IdealPoint((((vec2<f32>(_e4.g0_.x) * _e8.g0_) * vec2<f32>(1.0, -(1.0))) + (vec2<f32>(_e16.g0_.y) * _e20.g0_.yx)));
}

fn ideal_point_rotor_outer_product(self_608: IdealPoint, other_512: Rotor) -> IdealPoint {
    var self_609: IdealPoint;
    var other_513: Rotor;

    self_609 = self_608;
    other_513 = other_512;
    let _e4: IdealPoint = self_609;
    let _e6: Rotor = other_513;
    return IdealPoint((_e4.g0_ * vec2<f32>(_e6.g0_.x)));
}

fn ideal_point_rotor_inner_product(self_610: IdealPoint, other_514: Rotor) -> IdealPoint {
    var self_611: IdealPoint;
    var other_515: Rotor;

    self_611 = self_610;
    other_515 = other_514;
    let _e4: IdealPoint = self_611;
    let _e6: Rotor = other_515;
    return IdealPoint((_e4.g0_ * vec2<f32>(_e6.g0_.x)));
}

fn ideal_point_rotor_right_contraction(self_612: IdealPoint, other_516: Rotor) -> IdealPoint {
    var self_613: IdealPoint;
    var other_517: Rotor;

    self_613 = self_612;
    other_517 = other_516;
    let _e4: IdealPoint = self_613;
    let _e6: Rotor = other_517;
    return IdealPoint((_e4.g0_ * vec2<f32>(_e6.g0_.x)));
}

fn ideal_point_point_add(self_614: IdealPoint, other_518: Point) -> Point {
    var self_615: IdealPoint;
    var other_519: Point;

    self_615 = self_614;
    other_519 = other_518;
    let _e4: IdealPoint = self_615;
    let _e7: IdealPoint = self_615;
    let _e10: IdealPoint = self_615;
    let _e19: Point = other_519;
    return Point(((vec3<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.y) * vec3<f32>(0.0, 1.0, 1.0)) + _e19.g0_));
}

fn ideal_point_point_sub(self_616: IdealPoint, other_520: Point) -> Point {
    var self_617: IdealPoint;
    var other_521: Point;

    self_617 = self_616;
    other_521 = other_520;
    let _e4: IdealPoint = self_617;
    let _e7: IdealPoint = self_617;
    let _e10: IdealPoint = self_617;
    let _e19: Point = other_521;
    return Point(((vec3<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.y) * vec3<f32>(0.0, 1.0, 1.0)) - _e19.g0_));
}

fn ideal_point_point_geometric_product(self_618: IdealPoint, other_522: Point) -> IdealPoint {
    var self_619: IdealPoint;
    var other_523: Point;

    self_619 = self_618;
    other_523 = other_522;
    let _e4: IdealPoint = self_619;
    let _e7: Point = other_523;
    return IdealPoint(((_e4.g0_.yx * vec2<f32>(_e7.g0_.x)) * vec2<f32>(1.0, -(1.0))));
}

fn ideal_point_point_regressive_product(self_620: IdealPoint, other_524: Point) -> Plane {
    var self_621: IdealPoint;
    var other_525: Point;

    self_621 = self_620;
    other_525 = other_524;
    let _e4: IdealPoint = self_621;
    let _e8: Point = other_525;
    let _e18: IdealPoint = self_621;
    let _e22: Point = other_525;
    return Plane((((vec3<f32>(_e4.g0_.y) * _e8.g0_.yxy) * vec3<f32>(1.0, -(1.0), 0.0)) + ((vec3<f32>(_e18.g0_.x) * _e22.g0_.zxx) * vec3<f32>(-(1.0), 0.0, 1.0))));
}

fn ideal_point_ideal_point_add(self_622: IdealPoint, other_526: IdealPoint) -> IdealPoint {
    var self_623: IdealPoint;
    var other_527: IdealPoint;

    self_623 = self_622;
    other_527 = other_526;
    let _e4: IdealPoint = self_623;
    let _e6: IdealPoint = other_527;
    return IdealPoint((_e4.g0_ + _e6.g0_));
}

fn ideal_point_ideal_point_sub(self_624: IdealPoint, other_528: IdealPoint) -> IdealPoint {
    var self_625: IdealPoint;
    var other_529: IdealPoint;

    self_625 = self_624;
    other_529 = other_528;
    let _e4: IdealPoint = self_625;
    let _e6: IdealPoint = other_529;
    return IdealPoint((_e4.g0_ - _e6.g0_));
}

fn ideal_point_ideal_point_mul(self_626: IdealPoint, other_530: IdealPoint) -> IdealPoint {
    var self_627: IdealPoint;
    var other_531: IdealPoint;

    self_627 = self_626;
    other_531 = other_530;
    let _e4: IdealPoint = self_627;
    let _e6: IdealPoint = other_531;
    return IdealPoint((_e4.g0_ * _e6.g0_));
}

fn ideal_point_ideal_point_div(self_628: IdealPoint, other_532: IdealPoint) -> IdealPoint {
    var self_629: IdealPoint;
    var other_533: IdealPoint;

    self_629 = self_628;
    other_533 = other_532;
    let _e4: IdealPoint = self_629;
    let _e7: IdealPoint = self_629;
    let _e15: IdealPoint = other_533;
    let _e18: IdealPoint = other_533;
    return IdealPoint((((vec2<f32>(_e4.g0_.x, _e7.g0_.y) * vec2<f32>(1.0, 1.0)) / vec2<f32>(_e15.g0_.x, _e18.g0_.y)) * vec2<f32>(1.0, 1.0)));
}

fn ideal_point_plane_regressive_product(self_630: IdealPoint, other_534: Plane) -> Scalar {
    var self_631: IdealPoint;
    var other_535: Plane;

    self_631 = self_630;
    other_535 = other_534;
    let _e4: IdealPoint = self_631;
    let _e7: Plane = other_535;
    let _e11: IdealPoint = self_631;
    let _e14: Plane = other_535;
    return Scalar(((_e4.g0_.x * _e7.g0_.y) + (_e11.g0_.y * _e14.g0_.z)));
}

fn ideal_point_translator_add(self_632: IdealPoint, other_536: Translator) -> Translator {
    var self_633: IdealPoint;
    var other_537: Translator;

    self_633 = self_632;
    other_537 = other_536;
    let _e4: IdealPoint = self_633;
    let _e7: IdealPoint = self_633;
    let _e10: IdealPoint = self_633;
    let _e19: Translator = other_537;
    return Translator(((vec3<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.y) * vec3<f32>(0.0, 1.0, 1.0)) + _e19.g0_));
}

fn ideal_point_translator_sub(self_634: IdealPoint, other_538: Translator) -> Translator {
    var self_635: IdealPoint;
    var other_539: Translator;

    self_635 = self_634;
    other_539 = other_538;
    let _e4: IdealPoint = self_635;
    let _e7: IdealPoint = self_635;
    let _e10: IdealPoint = self_635;
    let _e19: Translator = other_539;
    return Translator(((vec3<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.y) * vec3<f32>(0.0, 1.0, 1.0)) - _e19.g0_));
}

fn ideal_point_translator_geometric_product(self_636: IdealPoint, other_540: Translator) -> IdealPoint {
    var self_637: IdealPoint;
    var other_541: Translator;

    self_637 = self_636;
    other_541 = other_540;
    let _e4: IdealPoint = self_637;
    let _e6: Translator = other_541;
    return IdealPoint((_e4.g0_ * vec2<f32>(_e6.g0_.x)));
}

fn ideal_point_translator_outer_product(self_638: IdealPoint, other_542: Translator) -> IdealPoint {
    var self_639: IdealPoint;
    var other_543: Translator;

    self_639 = self_638;
    other_543 = other_542;
    let _e4: IdealPoint = self_639;
    let _e6: Translator = other_543;
    return IdealPoint((_e4.g0_ * vec2<f32>(_e6.g0_.x)));
}

fn ideal_point_translator_inner_product(self_640: IdealPoint, other_544: Translator) -> IdealPoint {
    var self_641: IdealPoint;
    var other_545: Translator;

    self_641 = self_640;
    other_545 = other_544;
    let _e4: IdealPoint = self_641;
    let _e6: Translator = other_545;
    return IdealPoint((_e4.g0_ * vec2<f32>(_e6.g0_.x)));
}

fn ideal_point_translator_right_contraction(self_642: IdealPoint, other_546: Translator) -> IdealPoint {
    var self_643: IdealPoint;
    var other_547: Translator;

    self_643 = self_642;
    other_547 = other_546;
    let _e4: IdealPoint = self_643;
    let _e6: Translator = other_547;
    return IdealPoint((_e4.g0_ * vec2<f32>(_e6.g0_.x)));
}

fn ideal_point_motor_add(self_644: IdealPoint, other_548: Motor) -> Motor {
    var self_645: IdealPoint;
    var other_549: Motor;

    self_645 = self_644;
    other_549 = other_548;
    let _e4: IdealPoint = self_645;
    let _e7: IdealPoint = self_645;
    let _e10: IdealPoint = self_645;
    let _e13: IdealPoint = self_645;
    let _e23: Motor = other_549;
    return Motor(((vec4<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.x, _e13.g0_.y) * vec4<f32>(0.0, 0.0, 1.0, 1.0)) + _e23.g0_));
}

fn ideal_point_motor_sub(self_646: IdealPoint, other_550: Motor) -> Motor {
    var self_647: IdealPoint;
    var other_551: Motor;

    self_647 = self_646;
    other_551 = other_550;
    let _e4: IdealPoint = self_647;
    let _e7: IdealPoint = self_647;
    let _e10: IdealPoint = self_647;
    let _e13: IdealPoint = self_647;
    let _e23: Motor = other_551;
    return Motor(((vec4<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.x, _e13.g0_.y) * vec4<f32>(0.0, 0.0, 1.0, 1.0)) - _e23.g0_));
}

fn ideal_point_motor_geometric_product(self_648: IdealPoint, other_552: Motor) -> IdealPoint {
    var self_649: IdealPoint;
    var other_553: Motor;

    self_649 = self_648;
    other_553 = other_552;
    let _e4: IdealPoint = self_649;
    let _e8: Motor = other_553;
    let _e11: Motor = other_553;
    let _e21: IdealPoint = self_649;
    let _e25: Motor = other_553;
    let _e28: Motor = other_553;
    return IdealPoint((((vec2<f32>(_e4.g0_.x) * vec2<f32>(_e8.g0_.x, _e11.g0_.y)) * vec2<f32>(1.0, -(1.0))) + (vec2<f32>(_e21.g0_.y) * vec2<f32>(_e25.g0_.y, _e28.g0_.x))));
}

fn ideal_point_motor_regressive_product(self_650: IdealPoint, other_554: Motor) -> Plane {
    var self_651: IdealPoint;
    var other_555: Motor;

    self_651 = self_650;
    other_555 = other_554;
    let _e4: IdealPoint = self_651;
    let _e8: Motor = other_555;
    let _e11: Motor = other_555;
    let _e14: Motor = other_555;
    let _e25: IdealPoint = self_651;
    let _e29: Motor = other_555;
    let _e32: Motor = other_555;
    let _e35: Motor = other_555;
    return Plane((((vec3<f32>(_e4.g0_.y) * vec3<f32>(_e8.g0_.z, _e11.g0_.y, _e14.g0_.z)) * vec3<f32>(1.0, -(1.0), 0.0)) + ((vec3<f32>(_e25.g0_.x) * vec3<f32>(_e29.g0_.w, _e32.g0_.x, _e35.g0_.y)) * vec3<f32>(-(1.0), 0.0, 1.0))));
}

fn ideal_point_motor_outer_product(self_652: IdealPoint, other_556: Motor) -> IdealPoint {
    var self_653: IdealPoint;
    var other_557: Motor;

    self_653 = self_652;
    other_557 = other_556;
    let _e4: IdealPoint = self_653;
    let _e6: Motor = other_557;
    return IdealPoint((_e4.g0_ * vec2<f32>(_e6.g0_.x)));
}

fn ideal_point_motor_inner_product(self_654: IdealPoint, other_558: Motor) -> IdealPoint {
    var self_655: IdealPoint;
    var other_559: Motor;

    self_655 = self_654;
    other_559 = other_558;
    let _e4: IdealPoint = self_655;
    let _e6: Motor = other_559;
    return IdealPoint((_e4.g0_ * vec2<f32>(_e6.g0_.x)));
}

fn ideal_point_motor_right_contraction(self_656: IdealPoint, other_560: Motor) -> IdealPoint {
    var self_657: IdealPoint;
    var other_561: Motor;

    self_657 = self_656;
    other_561 = other_560;
    let _e4: IdealPoint = self_657;
    let _e6: Motor = other_561;
    return IdealPoint((_e4.g0_ * vec2<f32>(_e6.g0_.x)));
}

fn ideal_point_motor_dual_regressive_product(self_658: IdealPoint, other_562: MotorDual) -> Translator {
    var self_659: IdealPoint;
    var other_563: MotorDual;

    self_659 = self_658;
    other_563 = other_562;
    let _e4: IdealPoint = self_659;
    let _e8: MotorDual = other_563;
    let _e11: MotorDual = other_563;
    let _e14: MotorDual = other_563;
    let _e24: IdealPoint = self_659;
    let _e28: MotorDual = other_563;
    let _e31: MotorDual = other_563;
    let _e34: MotorDual = other_563;
    return Translator((((vec3<f32>(_e4.g0_.y) * vec3<f32>(_e8.g0_.w, _e11.g0_.w, _e14.g0_.x)) * vec3<f32>(1.0, 0.0, 1.0)) + ((vec3<f32>(_e24.g0_.x) * vec3<f32>(_e28.g0_.z, _e31.g0_.x, _e34.g0_.x)) * vec3<f32>(1.0, 1.0, 0.0))));
}

fn ideal_point_scale(self_660: IdealPoint, other_564: f32) -> IdealPoint {
    var self_661: IdealPoint;
    var other_565: f32;

    self_661 = self_660;
    other_565 = other_564;
    let _e5: f32 = other_565;
    let _e7: IdealPoint = self_661;
    let _e8: f32 = other_565;
    let _e10: IdealPoint = ideal_point_scalar_geometric_product(_e7, Scalar(_e8));
    return _e10;
}

fn plane_zero() -> Plane {
    return Plane(vec3<f32>(0.0));
}

fn plane_one() -> Plane {
    return Plane(vec3<f32>(0.0));
}

fn plane_neg(self_662: Plane) -> Plane {
    var self_663: Plane;

    self_663 = self_662;
    let _e2: Plane = self_663;
    return Plane((_e2.g0_ * vec3<f32>(-(1.0))));
}

fn plane_automorphism(self_664: Plane) -> Plane {
    var self_665: Plane;

    self_665 = self_664;
    let _e2: Plane = self_665;
    return Plane((_e2.g0_ * vec3<f32>(-(1.0))));
}

fn plane_reversal(self_666: Plane) -> Plane {
    var self_667: Plane;

    self_667 = self_666;
    let _e2: Plane = self_667;
    return Plane(_e2.g0_);
}

fn plane_conjugation(self_668: Plane) -> Plane {
    var self_669: Plane;

    self_669 = self_668;
    let _e2: Plane = self_669;
    return Plane((_e2.g0_ * vec3<f32>(-(1.0))));
}

fn plane_dual(self_670: Plane) -> Point {
    var self_671: Plane;

    self_671 = self_670;
    let _e2: Plane = self_671;
    return Point(_e2.g0_);
}

fn plane_scalar_geometric_product(self_672: Plane, other_566: Scalar) -> Plane {
    var self_673: Plane;
    var other_567: Scalar;

    self_673 = self_672;
    other_567 = other_566;
    let _e4: Plane = self_673;
    let _e6: Scalar = other_567;
    return Plane((_e4.g0_ * vec3<f32>(_e6.g0_)));
}

fn plane_scalar_outer_product(self_674: Plane, other_568: Scalar) -> Plane {
    var self_675: Plane;
    var other_569: Scalar;

    self_675 = self_674;
    other_569 = other_568;
    let _e4: Plane = self_675;
    let _e6: Scalar = other_569;
    return Plane((_e4.g0_ * vec3<f32>(_e6.g0_)));
}

fn plane_scalar_inner_product(self_676: Plane, other_570: Scalar) -> Plane {
    var self_677: Plane;
    var other_571: Scalar;

    self_677 = self_676;
    other_571 = other_570;
    let _e4: Plane = self_677;
    let _e6: Scalar = other_571;
    return Plane((_e4.g0_ * vec3<f32>(_e6.g0_)));
}

fn plane_scalar_right_contraction(self_678: Plane, other_572: Scalar) -> Plane {
    var self_679: Plane;
    var other_573: Scalar;

    self_679 = self_678;
    other_573 = other_572;
    let _e4: Plane = self_679;
    let _e6: Scalar = other_573;
    return Plane((_e4.g0_ * vec3<f32>(_e6.g0_)));
}

fn plane_multi_vector_add(self_680: Plane, other_574: MultiVector) -> MultiVector {
    var self_681: Plane;
    var other_575: MultiVector;

    self_681 = self_680;
    other_575 = other_574;
    let _e4: Plane = self_681;
    let _e7: Plane = self_681;
    let _e10: Plane = self_681;
    let _e13: Plane = self_681;
    let _e23: MultiVector = other_575;
    let _e26: Plane = self_681;
    let _e36: MultiVector = other_575;
    return MultiVector(((vec4<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.z, _e13.g0_.y) * vec4<f32>(0.0, 0.0, 1.0, 1.0)) + _e23.g0_), ((vec4<f32>(_e26.g0_.x) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) + _e36.g1_));
}

fn plane_multi_vector_sub(self_682: Plane, other_576: MultiVector) -> MultiVector {
    var self_683: Plane;
    var other_577: MultiVector;

    self_683 = self_682;
    other_577 = other_576;
    let _e4: Plane = self_683;
    let _e7: Plane = self_683;
    let _e10: Plane = self_683;
    let _e13: Plane = self_683;
    let _e23: MultiVector = other_577;
    let _e26: Plane = self_683;
    let _e36: MultiVector = other_577;
    return MultiVector(((vec4<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.z, _e13.g0_.y) * vec4<f32>(0.0, 0.0, 1.0, 1.0)) - _e23.g0_), ((vec4<f32>(_e26.g0_.x) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) - _e36.g1_));
}

fn plane_multi_vector_geometric_product(self_684: Plane, other_578: MultiVector) -> MultiVector {
    var self_685: Plane;
    var other_579: MultiVector;

    self_685 = self_684;
    other_579 = other_578;
    let _e4: Plane = self_685;
    let _e8: MultiVector = other_579;
    let _e20: Plane = self_685;
    let _e24: MultiVector = other_579;
    let _e29: Plane = self_685;
    let _e33: MultiVector = other_579;
    let _e43: Plane = self_685;
    let _e47: MultiVector = other_579;
    let _e52: Plane = self_685;
    let _e56: MultiVector = other_579;
    return MultiVector((((vec4<f32>(_e4.g0_.y) * _e8.g0_.wzyx) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0)) + (vec4<f32>(_e20.g0_.z) * _e24.g0_.zwxy)), ((((vec4<f32>(_e29.g0_.x) * _e33.g0_) * vec4<f32>(1.0, 1.0, 1.0, -(1.0))) + (vec4<f32>(_e43.g0_.y) * _e47.g1_.wzyx)) + ((vec4<f32>(_e52.g0_.z) * _e56.g1_.zwxy) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))));
}

fn plane_multi_vector_scalar_product(self_686: Plane, other_580: MultiVector) -> Scalar {
    var self_687: Plane;
    var other_581: MultiVector;

    self_687 = self_686;
    other_581 = other_580;
    let _e4: Plane = self_687;
    let _e7: MultiVector = other_581;
    let _e11: Plane = self_687;
    let _e14: MultiVector = other_581;
    return Scalar(((_e4.g0_.y * _e7.g0_.w) + (_e11.g0_.z * _e14.g0_.z)));
}

fn plane_rotor_geometric_product(self_688: Plane, other_582: Rotor) -> MotorDual {
    var self_689: Plane;
    var other_583: Rotor;

    self_689 = self_688;
    other_583 = other_582;
    let _e4: Plane = self_689;
    let _e8: Rotor = other_583;
    let _e11: Rotor = other_583;
    let _e14: Rotor = other_583;
    let _e17: Rotor = other_583;
    let _e28: Plane = self_689;
    let _e31: Plane = self_689;
    let _e34: Plane = self_689;
    let _e37: Plane = self_689;
    let _e41: Rotor = other_583;
    let _e44: Rotor = other_583;
    let _e47: Rotor = other_583;
    let _e50: Rotor = other_583;
    return MotorDual((((vec4<f32>(_e4.g0_.z) * vec4<f32>(_e8.g0_.y, _e11.g0_.y, _e14.g0_.y, _e17.g0_.x)) * vec4<f32>(0.0, 0.0, 1.0, 1.0)) + ((vec4<f32>(_e28.g0_.x, _e31.g0_.x, _e34.g0_.y, _e37.g0_.y) * vec4<f32>(_e41.g0_.y, _e44.g0_.x, _e47.g0_.x, _e50.g0_.y)) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))));
}

fn plane_rotor_regressive_product(self_690: Plane, other_584: Rotor) -> Scalar {
    var self_691: Plane;
    var other_585: Rotor;

    self_691 = self_690;
    other_585 = other_584;
    let _e4: Plane = self_691;
    let _e7: Rotor = other_585;
    return Scalar((_e4.g0_.x * _e7.g0_.y));
}

fn plane_rotor_outer_product(self_692: Plane, other_586: Rotor) -> MotorDual {
    var self_693: Plane;
    var other_587: Rotor;

    self_693 = self_692;
    other_587 = other_586;
    let _e4: Plane = self_693;
    let _e7: Plane = self_693;
    let _e10: Plane = self_693;
    let _e13: Plane = self_693;
    let _e17: Rotor = other_587;
    let _e20: Rotor = other_587;
    let _e23: Rotor = other_587;
    let _e26: Rotor = other_587;
    return MotorDual((vec4<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.y, _e13.g0_.z) * vec4<f32>(_e17.g0_.y, _e20.g0_.x, _e23.g0_.x, _e26.g0_.x)));
}

fn plane_rotor_inner_product(self_694: Plane, other_588: Rotor) -> Plane {
    var self_695: Plane;
    var other_589: Rotor;

    self_695 = self_694;
    other_589 = other_588;
    let _e4: Plane = self_695;
    let _e8: Rotor = other_589;
    let _e11: Rotor = other_589;
    let _e14: Rotor = other_589;
    let _e24: Plane = self_695;
    let _e27: Rotor = other_589;
    let _e30: Rotor = other_589;
    let _e33: Rotor = other_589;
    return Plane((((vec3<f32>(_e4.g0_.z) * vec3<f32>(_e8.g0_.y, _e11.g0_.y, _e14.g0_.x)) * vec3<f32>(0.0, 1.0, 1.0)) + ((_e24.g0_.xyy * vec3<f32>(_e27.g0_.x, _e30.g0_.x, _e33.g0_.y)) * vec3<f32>(1.0, 1.0, -(1.0)))));
}

fn plane_rotor_right_contraction(self_696: Plane, other_590: Rotor) -> Plane {
    var self_697: Plane;
    var other_591: Rotor;

    self_697 = self_696;
    other_591 = other_590;
    let _e4: Plane = self_697;
    let _e6: Rotor = other_591;
    return Plane((_e4.g0_ * vec3<f32>(_e6.g0_.x)));
}

fn plane_point_geometric_product(self_698: Plane, other_592: Point) -> MotorDual {
    var self_699: Plane;
    var other_593: Point;

    self_699 = self_698;
    other_593 = other_592;
    let _e4: Plane = self_699;
    let _e8: Point = other_593;
    let _e11: Point = other_593;
    let _e14: Point = other_593;
    let _e17: Point = other_593;
    let _e29: Plane = self_699;
    let _e33: Point = other_593;
    let _e36: Point = other_593;
    let _e39: Point = other_593;
    let _e42: Point = other_593;
    let _e55: Plane = self_699;
    let _e59: Point = other_593;
    return MotorDual(((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y, _e11.g0_.z, _e14.g0_.y, _e17.g0_.x)) * vec4<f32>(1.0, 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e29.g0_.z) * vec4<f32>(_e33.g0_.z, _e36.g0_.y, _e39.g0_.x, _e42.g0_.z)) * vec4<f32>(1.0, -(1.0), 1.0, 0.0))) + ((vec4<f32>(_e55.g0_.x) * vec4<f32>(_e59.g0_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))));
}

fn plane_point_regressive_product(self_700: Plane, other_594: Point) -> Scalar {
    var self_701: Plane;
    var other_595: Point;

    self_701 = self_700;
    other_595 = other_594;
    let _e4: Plane = self_701;
    let _e7: Point = other_595;
    let _e11: Plane = self_701;
    let _e14: Point = other_595;
    let _e19: Plane = self_701;
    let _e22: Point = other_595;
    return Scalar((((_e4.g0_.x * _e7.g0_.x) + (_e11.g0_.y * _e14.g0_.y)) + (_e19.g0_.z * _e22.g0_.z)));
}

fn plane_point_inner_product(self_702: Plane, other_596: Point) -> Plane {
    var self_703: Plane;
    var other_597: Point;

    self_703 = self_702;
    other_597 = other_596;
    let _e4: Plane = self_703;
    let _e8: Point = other_597;
    let _e18: Plane = self_703;
    let _e21: Point = other_597;
    return Plane((((vec3<f32>(_e4.g0_.z) * _e8.g0_.yxy) * vec3<f32>(-(1.0), 1.0, 0.0)) + ((_e18.g0_.yxy * _e21.g0_.zxx) * vec3<f32>(1.0, 0.0, -(1.0)))));
}

fn plane_point_left_contraction(self_704: Plane, other_598: Point) -> Plane {
    var self_705: Plane;
    var other_599: Point;

    self_705 = self_704;
    other_599 = other_598;
    let _e4: Plane = self_705;
    let _e8: Point = other_599;
    let _e18: Plane = self_705;
    let _e21: Point = other_599;
    return Plane((((vec3<f32>(_e4.g0_.z) * _e8.g0_.yxy) * vec3<f32>(-(1.0), 1.0, 0.0)) + ((_e18.g0_.yxy * _e21.g0_.zxx) * vec3<f32>(1.0, 0.0, -(1.0)))));
}

fn plane_ideal_point_regressive_product(self_706: Plane, other_600: IdealPoint) -> Scalar {
    var self_707: Plane;
    var other_601: IdealPoint;

    self_707 = self_706;
    other_601 = other_600;
    let _e4: Plane = self_707;
    let _e7: IdealPoint = other_601;
    let _e11: Plane = self_707;
    let _e14: IdealPoint = other_601;
    return Scalar(((_e4.g0_.y * _e7.g0_.x) + (_e11.g0_.z * _e14.g0_.y)));
}

fn plane_plane_add(self_708: Plane, other_602: Plane) -> Plane {
    var self_709: Plane;
    var other_603: Plane;

    self_709 = self_708;
    other_603 = other_602;
    let _e4: Plane = self_709;
    let _e6: Plane = other_603;
    return Plane((_e4.g0_ + _e6.g0_));
}

fn plane_plane_sub(self_710: Plane, other_604: Plane) -> Plane {
    var self_711: Plane;
    var other_605: Plane;

    self_711 = self_710;
    other_605 = other_604;
    let _e4: Plane = self_711;
    let _e6: Plane = other_605;
    return Plane((_e4.g0_ - _e6.g0_));
}

fn plane_plane_mul(self_712: Plane, other_606: Plane) -> Plane {
    var self_713: Plane;
    var other_607: Plane;

    self_713 = self_712;
    other_607 = other_606;
    let _e4: Plane = self_713;
    let _e6: Plane = other_607;
    return Plane((_e4.g0_ * _e6.g0_));
}

fn plane_plane_div(self_714: Plane, other_608: Plane) -> Plane {
    var self_715: Plane;
    var other_609: Plane;

    self_715 = self_714;
    other_609 = other_608;
    let _e4: Plane = self_715;
    let _e7: Plane = self_715;
    let _e10: Plane = self_715;
    let _e19: Plane = other_609;
    let _e22: Plane = other_609;
    let _e25: Plane = other_609;
    return Plane((((vec3<f32>(_e4.g0_.x, _e7.g0_.y, _e10.g0_.z) * vec3<f32>(1.0, 1.0, 1.0)) / vec3<f32>(_e19.g0_.x, _e22.g0_.y, _e25.g0_.z)) * vec3<f32>(1.0, 1.0, 1.0)));
}

fn plane_plane_geometric_product(self_716: Plane, other_610: Plane) -> Motor {
    var self_717: Plane;
    var other_611: Plane;

    self_717 = self_716;
    other_611 = other_610;
    let _e4: Plane = self_717;
    let _e8: Plane = other_611;
    let _e11: Plane = other_611;
    let _e14: Plane = other_611;
    let _e17: Plane = other_611;
    let _e29: Plane = self_717;
    let _e33: Plane = other_611;
    let _e36: Plane = other_611;
    let _e39: Plane = other_611;
    let _e42: Plane = other_611;
    let _e55: Plane = self_717;
    let _e59: Plane = other_611;
    let _e62: Plane = other_611;
    let _e65: Plane = other_611;
    let _e68: Plane = other_611;
    return Motor(((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y, _e11.g0_.z, _e14.g0_.y, _e17.g0_.x)) * vec4<f32>(1.0, -(1.0), 0.0, 1.0)) + ((vec4<f32>(_e29.g0_.z) * vec4<f32>(_e33.g0_.z, _e36.g0_.y, _e39.g0_.x, _e42.g0_.z)) * vec4<f32>(1.0, 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e55.g0_.x) * vec4<f32>(_e59.g0_.x, _e62.g0_.x, _e65.g0_.z, _e68.g0_.y)) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))));
}

fn plane_plane_outer_product(self_718: Plane, other_612: Plane) -> Point {
    var self_719: Plane;
    var other_613: Plane;

    self_719 = self_718;
    other_613 = other_612;
    let _e4: Plane = self_719;
    let _e8: Plane = other_613;
    let _e18: Plane = self_719;
    let _e22: Plane = other_613;
    let _e33: Plane = self_719;
    let _e37: Plane = other_613;
    return Point(((((vec3<f32>(_e4.g0_.y) * _e8.g0_.zzx) * vec3<f32>(-(1.0), 0.0, 1.0)) + ((vec3<f32>(_e18.g0_.z) * _e22.g0_.yxy) * vec3<f32>(1.0, -(1.0), 0.0))) + ((vec3<f32>(_e33.g0_.x) * _e37.g0_.xzy) * vec3<f32>(0.0, 1.0, -(1.0)))));
}

fn plane_plane_inner_product(self_720: Plane, other_614: Plane) -> Scalar {
    var self_721: Plane;
    var other_615: Plane;

    self_721 = self_720;
    other_615 = other_614;
    let _e4: Plane = self_721;
    let _e7: Plane = other_615;
    let _e11: Plane = self_721;
    let _e14: Plane = other_615;
    return Scalar(((_e4.g0_.y * _e7.g0_.y) + (_e11.g0_.z * _e14.g0_.z)));
}

fn plane_plane_left_contraction(self_722: Plane, other_616: Plane) -> Scalar {
    var self_723: Plane;
    var other_617: Plane;

    self_723 = self_722;
    other_617 = other_616;
    let _e4: Plane = self_723;
    let _e7: Plane = other_617;
    let _e11: Plane = self_723;
    let _e14: Plane = other_617;
    return Scalar(((_e4.g0_.y * _e7.g0_.y) + (_e11.g0_.z * _e14.g0_.z)));
}

fn plane_plane_right_contraction(self_724: Plane, other_618: Plane) -> Scalar {
    var self_725: Plane;
    var other_619: Plane;

    self_725 = self_724;
    other_619 = other_618;
    let _e4: Plane = self_725;
    let _e7: Plane = other_619;
    let _e11: Plane = self_725;
    let _e14: Plane = other_619;
    return Scalar(((_e4.g0_.y * _e7.g0_.y) + (_e11.g0_.z * _e14.g0_.z)));
}

fn plane_plane_scalar_product(self_726: Plane, other_620: Plane) -> Scalar {
    var self_727: Plane;
    var other_621: Plane;

    self_727 = self_726;
    other_621 = other_620;
    let _e4: Plane = self_727;
    let _e7: Plane = other_621;
    let _e11: Plane = self_727;
    let _e14: Plane = other_621;
    return Scalar(((_e4.g0_.y * _e7.g0_.y) + (_e11.g0_.z * _e14.g0_.z)));
}

fn plane_translator_geometric_product(self_728: Plane, other_622: Translator) -> MotorDual {
    var self_729: Plane;
    var other_623: Translator;

    self_729 = self_728;
    other_623 = other_622;
    let _e4: Plane = self_729;
    let _e8: Translator = other_623;
    let _e11: Translator = other_623;
    let _e14: Translator = other_623;
    let _e17: Translator = other_623;
    let _e28: Plane = self_729;
    let _e32: Translator = other_623;
    let _e35: Translator = other_623;
    let _e38: Translator = other_623;
    let _e41: Translator = other_623;
    let _e54: Plane = self_729;
    let _e58: Translator = other_623;
    return MotorDual(((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y, _e11.g0_.z, _e14.g0_.x, _e17.g0_.y)) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) + ((vec4<f32>(_e28.g0_.z) * vec4<f32>(_e32.g0_.z, _e35.g0_.y, _e38.g0_.z, _e41.g0_.x)) * vec4<f32>(1.0, -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e54.g0_.x) * vec4<f32>(_e58.g0_.x)) * vec4<f32>(0.0, 1.0, 0.0, 0.0))));
}

fn plane_translator_regressive_product(self_730: Plane, other_624: Translator) -> Scalar {
    var self_731: Plane;
    var other_625: Translator;

    self_731 = self_730;
    other_625 = other_624;
    let _e4: Plane = self_731;
    let _e7: Translator = other_625;
    let _e11: Plane = self_731;
    let _e14: Translator = other_625;
    return Scalar(((_e4.g0_.y * _e7.g0_.y) + (_e11.g0_.z * _e14.g0_.z)));
}

fn plane_translator_outer_product(self_732: Plane, other_626: Translator) -> MotorDual {
    var self_733: Plane;
    var other_627: Translator;

    self_733 = self_732;
    other_627 = other_626;
    let _e4: Plane = self_733;
    let _e8: Translator = other_627;
    let _e11: Translator = other_627;
    let _e14: Translator = other_627;
    let _e17: Translator = other_627;
    let _e28: Plane = self_733;
    let _e31: Plane = self_733;
    let _e34: Plane = self_733;
    let _e37: Plane = self_733;
    let _e41: Translator = other_627;
    let _e44: Translator = other_627;
    let _e47: Translator = other_627;
    let _e50: Translator = other_627;
    return MotorDual((((vec4<f32>(_e4.g0_.z) * vec4<f32>(_e8.g0_.z, _e11.g0_.z, _e14.g0_.z, _e17.g0_.x)) * vec4<f32>(1.0, 0.0, 0.0, 1.0)) + ((vec4<f32>(_e28.g0_.y, _e31.g0_.x, _e34.g0_.y, _e37.g0_.x) * vec4<f32>(_e41.g0_.y, _e44.g0_.x, _e47.g0_.x, _e50.g0_.x)) * vec4<f32>(1.0, 1.0, 1.0, 0.0))));
}

fn plane_translator_inner_product(self_734: Plane, other_628: Translator) -> Plane {
    var self_735: Plane;
    var other_629: Translator;

    self_735 = self_734;
    other_629 = other_628;
    let _e4: Plane = self_735;
    let _e8: Translator = other_629;
    let _e17: Plane = self_735;
    let _e21: Translator = other_629;
    let _e32: Plane = self_735;
    let _e36: Translator = other_629;
    return Plane(((((vec3<f32>(_e4.g0_.y) * _e8.g0_.zxz) * vec3<f32>(1.0, 1.0, 0.0)) + ((vec3<f32>(_e17.g0_.z) * _e21.g0_.yyx) * vec3<f32>(-(1.0), 0.0, 1.0))) + ((vec3<f32>(_e32.g0_.x) * vec3<f32>(_e36.g0_.x)) * vec3<f32>(1.0, 0.0, 0.0))));
}

fn plane_translator_right_contraction(self_736: Plane, other_630: Translator) -> Plane {
    var self_737: Plane;
    var other_631: Translator;

    self_737 = self_736;
    other_631 = other_630;
    let _e4: Plane = self_737;
    let _e6: Translator = other_631;
    return Plane((_e4.g0_ * vec3<f32>(_e6.g0_.x)));
}

fn plane_motor_geometric_product(self_738: Plane, other_632: Motor) -> MotorDual {
    var self_739: Plane;
    var other_633: Motor;

    self_739 = self_738;
    other_633 = other_632;
    let _e4: Plane = self_739;
    let _e8: Motor = other_633;
    let _e19: Plane = self_739;
    let _e23: Motor = other_633;
    let _e35: Plane = self_739;
    let _e39: Motor = other_633;
    return MotorDual(((((vec4<f32>(_e4.g0_.y) * _e8.g0_.zwxy) * vec4<f32>(1.0, 1.0, 1.0, -(1.0))) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.wzyx) * vec4<f32>(1.0, -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e35.g0_.x) * _e39.g0_.yxxx) * vec4<f32>(1.0, 1.0, 0.0, 0.0))));
}

fn plane_motor_regressive_product(self_740: Plane, other_634: Motor) -> Scalar {
    var self_741: Plane;
    var other_635: Motor;

    self_741 = self_740;
    other_635 = other_634;
    let _e4: Plane = self_741;
    let _e7: Motor = other_635;
    let _e11: Plane = self_741;
    let _e14: Motor = other_635;
    let _e19: Plane = self_741;
    let _e22: Motor = other_635;
    return Scalar((((_e4.g0_.x * _e7.g0_.y) + (_e11.g0_.y * _e14.g0_.z)) + (_e19.g0_.z * _e22.g0_.w)));
}

fn plane_motor_outer_product(self_742: Plane, other_636: Motor) -> MotorDual {
    var self_743: Plane;
    var other_637: Motor;

    self_743 = self_742;
    other_637 = other_636;
    let _e4: Plane = self_743;
    let _e8: Motor = other_637;
    let _e18: Plane = self_743;
    let _e22: Motor = other_637;
    let _e33: Plane = self_743;
    let _e37: Motor = other_637;
    return MotorDual(((((vec4<f32>(_e4.g0_.y) * _e8.g0_.zzxz) * vec4<f32>(1.0, 0.0, 1.0, 0.0)) + ((vec4<f32>(_e18.g0_.z) * _e22.g0_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e33.g0_.x) * _e37.g0_.yxxx) * vec4<f32>(1.0, 1.0, 0.0, 0.0))));
}

fn plane_motor_inner_product(self_744: Plane, other_638: Motor) -> Plane {
    var self_745: Plane;
    var other_639: Motor;

    self_745 = self_744;
    other_639 = other_638;
    let _e4: Plane = self_745;
    let _e8: Motor = other_639;
    let _e11: Motor = other_639;
    let _e14: Motor = other_639;
    let _e25: Plane = self_745;
    let _e29: Motor = other_639;
    let _e32: Motor = other_639;
    let _e35: Motor = other_639;
    let _e47: Plane = self_745;
    let _e51: Motor = other_639;
    return Plane(((((vec3<f32>(_e4.g0_.y) * vec3<f32>(_e8.g0_.w, _e11.g0_.x, _e14.g0_.y)) * vec3<f32>(1.0, 1.0, -(1.0))) + ((vec3<f32>(_e25.g0_.z) * vec3<f32>(_e29.g0_.z, _e32.g0_.y, _e35.g0_.x)) * vec3<f32>(-(1.0), 1.0, 1.0))) + ((vec3<f32>(_e47.g0_.x) * vec3<f32>(_e51.g0_.x)) * vec3<f32>(1.0, 0.0, 0.0))));
}

fn plane_motor_left_contraction(self_746: Plane, other_640: Motor) -> Plane {
    var self_747: Plane;
    var other_641: Motor;

    self_747 = self_746;
    other_641 = other_640;
    let _e4: Plane = self_747;
    let _e8: Motor = other_641;
    let _e11: Motor = other_641;
    let _e14: Motor = other_641;
    let _e25: Plane = self_747;
    let _e28: Motor = other_641;
    let _e31: Motor = other_641;
    let _e34: Motor = other_641;
    return Plane((((vec3<f32>(_e4.g0_.z) * vec3<f32>(_e8.g0_.z, _e11.g0_.y, _e14.g0_.z)) * vec3<f32>(-(1.0), 1.0, 0.0)) + ((_e25.g0_.yxy * vec3<f32>(_e28.g0_.w, _e31.g0_.x, _e34.g0_.y)) * vec3<f32>(1.0, 0.0, -(1.0)))));
}

fn plane_motor_right_contraction(self_748: Plane, other_642: Motor) -> Plane {
    var self_749: Plane;
    var other_643: Motor;

    self_749 = self_748;
    other_643 = other_642;
    let _e4: Plane = self_749;
    let _e6: Motor = other_643;
    return Plane((_e4.g0_ * vec3<f32>(_e6.g0_.x)));
}

fn plane_motor_dual_add(self_750: Plane, other_644: MotorDual) -> MotorDual {
    var self_751: Plane;
    var other_645: MotorDual;

    self_751 = self_750;
    other_645 = other_644;
    let _e4: Plane = self_751;
    let _e7: Plane = self_751;
    let _e10: Plane = self_751;
    let _e13: Plane = self_751;
    let _e23: MotorDual = other_645;
    return MotorDual(((vec4<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.y, _e13.g0_.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0)) + _e23.g0_));
}

fn plane_motor_dual_sub(self_752: Plane, other_646: MotorDual) -> MotorDual {
    var self_753: Plane;
    var other_647: MotorDual;

    self_753 = self_752;
    other_647 = other_646;
    let _e4: Plane = self_753;
    let _e7: Plane = self_753;
    let _e10: Plane = self_753;
    let _e13: Plane = self_753;
    let _e23: MotorDual = other_647;
    return MotorDual(((vec4<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.y, _e13.g0_.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0)) - _e23.g0_));
}

fn plane_motor_dual_geometric_product(self_754: Plane, other_648: MotorDual) -> Motor {
    var self_755: Plane;
    var other_649: MotorDual;

    self_755 = self_754;
    other_649 = other_648;
    let _e4: Plane = self_755;
    let _e8: MotorDual = other_649;
    let _e19: Plane = self_755;
    let _e23: MotorDual = other_649;
    let _e35: Plane = self_755;
    let _e39: MotorDual = other_649;
    return Motor(((((vec4<f32>(_e4.g0_.y) * _e8.g0_.zwxy) * vec4<f32>(1.0, -(1.0), 1.0, 1.0)) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.wzyx) * vec4<f32>(1.0, 1.0, -(1.0), 1.0))) + ((vec4<f32>(_e35.g0_.x) * _e39.g0_.xxwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))));
}

fn plane_motor_dual_regressive_product(self_756: Plane, other_650: MotorDual) -> Plane {
    var self_757: Plane;
    var other_651: MotorDual;

    self_757 = self_756;
    other_651 = other_650;
    let _e4: Plane = self_757;
    let _e6: MotorDual = other_651;
    return Plane((_e4.g0_ * vec3<f32>(_e6.g0_.x)));
}

fn plane_motor_dual_outer_product(self_758: Plane, other_652: MotorDual) -> Point {
    var self_759: Plane;
    var other_653: MotorDual;

    self_759 = self_758;
    other_653 = other_652;
    let _e4: Plane = self_759;
    let _e8: MotorDual = other_653;
    let _e11: MotorDual = other_653;
    let _e14: MotorDual = other_653;
    let _e25: Plane = self_759;
    let _e29: MotorDual = other_653;
    let _e32: MotorDual = other_653;
    let _e35: MotorDual = other_653;
    let _e47: Plane = self_759;
    let _e51: MotorDual = other_653;
    let _e54: MotorDual = other_653;
    let _e57: MotorDual = other_653;
    return Point(((((vec3<f32>(_e4.g0_.y) * vec3<f32>(_e8.g0_.w, _e11.g0_.w, _e14.g0_.y)) * vec3<f32>(-(1.0), 0.0, 1.0)) + ((vec3<f32>(_e25.g0_.z) * vec3<f32>(_e29.g0_.z, _e32.g0_.y, _e35.g0_.z)) * vec3<f32>(1.0, -(1.0), 0.0))) + ((vec3<f32>(_e47.g0_.x) * vec3<f32>(_e51.g0_.x, _e54.g0_.w, _e57.g0_.z)) * vec3<f32>(0.0, 1.0, -(1.0)))));
}

fn plane_motor_dual_inner_product(self_760: Plane, other_654: MotorDual) -> Translator {
    var self_761: Plane;
    var other_655: MotorDual;

    self_761 = self_760;
    other_655 = other_654;
    let _e4: Plane = self_761;
    let _e8: MotorDual = other_655;
    let _e11: MotorDual = other_655;
    let _e14: MotorDual = other_655;
    let _e24: Plane = self_761;
    let _e27: MotorDual = other_655;
    let _e30: MotorDual = other_655;
    let _e33: MotorDual = other_655;
    return Translator((((vec3<f32>(_e4.g0_.z) * vec3<f32>(_e8.g0_.w, _e11.g0_.w, _e14.g0_.x)) * vec3<f32>(1.0, 0.0, 1.0)) + ((_e24.g0_.yyx * vec3<f32>(_e27.g0_.z, _e30.g0_.x, _e33.g0_.x)) * vec3<f32>(1.0, 1.0, 0.0))));
}

fn plane_motor_dual_left_contraction(self_762: Plane, other_656: MotorDual) -> Translator {
    var self_763: Plane;
    var other_657: MotorDual;

    self_763 = self_762;
    other_657 = other_656;
    let _e4: Plane = self_763;
    let _e8: MotorDual = other_657;
    let _e11: MotorDual = other_657;
    let _e14: MotorDual = other_657;
    let _e24: Plane = self_763;
    let _e27: MotorDual = other_657;
    let _e30: MotorDual = other_657;
    let _e33: MotorDual = other_657;
    return Translator((((vec3<f32>(_e4.g0_.z) * vec3<f32>(_e8.g0_.w, _e11.g0_.w, _e14.g0_.x)) * vec3<f32>(1.0, 0.0, 1.0)) + ((_e24.g0_.yyx * vec3<f32>(_e27.g0_.z, _e30.g0_.x, _e33.g0_.x)) * vec3<f32>(1.0, 1.0, 0.0))));
}

fn plane_motor_dual_right_contraction(self_764: Plane, other_658: MotorDual) -> Scalar {
    var self_765: Plane;
    var other_659: MotorDual;

    self_765 = self_764;
    other_659 = other_658;
    let _e4: Plane = self_765;
    let _e7: MotorDual = other_659;
    let _e11: Plane = self_765;
    let _e14: MotorDual = other_659;
    return Scalar(((_e4.g0_.y * _e7.g0_.z) + (_e11.g0_.z * _e14.g0_.w)));
}

fn plane_motor_dual_scalar_product(self_766: Plane, other_660: MotorDual) -> Scalar {
    var self_767: Plane;
    var other_661: MotorDual;

    self_767 = self_766;
    other_661 = other_660;
    let _e4: Plane = self_767;
    let _e7: MotorDual = other_661;
    let _e11: Plane = self_767;
    let _e14: MotorDual = other_661;
    return Scalar(((_e4.g0_.y * _e7.g0_.z) + (_e11.g0_.z * _e14.g0_.w)));
}

fn plane_squared_magnitude(self_768: Plane) -> Scalar {
    var self_769: Plane;

    self_769 = self_768;
    let _e4: Plane = self_769;
    let _e5: Plane = plane_reversal(_e4);
    let _e6: Plane = self_769;
    let _e8: Plane = self_769;
    let _e9: Plane = plane_reversal(_e8);
    let _e10: Scalar = plane_plane_scalar_product(_e6, _e9);
    return _e10;
}

fn plane_magnitude(self_770: Plane) -> Scalar {
    var self_771: Plane;

    self_771 = self_770;
    let _e3: Plane = self_771;
    let _e4: Scalar = plane_squared_magnitude(_e3);
    let _e7: Plane = self_771;
    let _e8: Scalar = plane_squared_magnitude(_e7);
    return Scalar(sqrt(_e8.g0_));
}

fn plane_scale(self_772: Plane, other_662: f32) -> Plane {
    var self_773: Plane;
    var other_663: f32;

    self_773 = self_772;
    other_663 = other_662;
    let _e5: f32 = other_663;
    let _e7: Plane = self_773;
    let _e8: f32 = other_663;
    let _e10: Plane = plane_scalar_geometric_product(_e7, Scalar(_e8));
    return _e10;
}

fn plane_signum(self_774: Plane) -> Plane {
    var self_775: Plane;

    self_775 = self_774;
    let _e5: Plane = self_775;
    let _e6: Scalar = plane_magnitude(_e5);
    let _e10: Plane = self_775;
    let _e13: Plane = self_775;
    let _e14: Scalar = plane_magnitude(_e13);
    let _e18: Plane = plane_scalar_geometric_product(_e10, Scalar((1.0 / _e14.g0_)));
    return _e18;
}

fn plane_inverse(self_776: Plane) -> Plane {
    var self_777: Plane;

    self_777 = self_776;
    let _e3: Plane = self_777;
    let _e4: Plane = plane_reversal(_e3);
    let _e7: Plane = self_777;
    let _e8: Scalar = plane_squared_magnitude(_e7);
    let _e13: Plane = self_777;
    let _e14: Plane = plane_reversal(_e13);
    let _e17: Plane = self_777;
    let _e18: Scalar = plane_squared_magnitude(_e17);
    let _e22: Plane = plane_scalar_geometric_product(_e14, Scalar((1.0 / _e18.g0_)));
    return _e22;
}

fn translator_zero() -> Translator {
    return Translator(vec3<f32>(0.0));
}

fn translator_one() -> Translator {
    return Translator(vec3<f32>(1.0, 0.0, 0.0));
}

fn translator_neg(self_778: Translator) -> Translator {
    var self_779: Translator;

    self_779 = self_778;
    let _e2: Translator = self_779;
    return Translator((_e2.g0_ * vec3<f32>(-(1.0))));
}

fn translator_automorphism(self_780: Translator) -> Translator {
    var self_781: Translator;

    self_781 = self_780;
    let _e2: Translator = self_781;
    return Translator(_e2.g0_);
}

fn translator_reversal(self_782: Translator) -> Translator {
    var self_783: Translator;

    self_783 = self_782;
    let _e2: Translator = self_783;
    return Translator((_e2.g0_ * vec3<f32>(1.0, -(1.0), -(1.0))));
}

fn translator_conjugation(self_784: Translator) -> Translator {
    var self_785: Translator;

    self_785 = self_784;
    let _e2: Translator = self_785;
    return Translator((_e2.g0_ * vec3<f32>(1.0, -(1.0), -(1.0))));
}

fn translator_scalar_into(self_786: Translator) -> Scalar {
    var self_787: Translator;

    self_787 = self_786;
    let _e2: Translator = self_787;
    return Scalar(_e2.g0_.x);
}

fn translator_scalar_add(self_788: Translator, other_664: Scalar) -> Translator {
    var self_789: Translator;
    var other_665: Scalar;

    self_789 = self_788;
    other_665 = other_664;
    let _e4: Translator = self_789;
    let _e6: Scalar = other_665;
    return Translator((_e4.g0_ + (vec3<f32>(_e6.g0_) * vec3<f32>(1.0, 0.0, 0.0))));
}

fn translator_scalar_sub(self_790: Translator, other_666: Scalar) -> Translator {
    var self_791: Translator;
    var other_667: Scalar;

    self_791 = self_790;
    other_667 = other_666;
    let _e4: Translator = self_791;
    let _e6: Scalar = other_667;
    return Translator((_e4.g0_ - (vec3<f32>(_e6.g0_) * vec3<f32>(1.0, 0.0, 0.0))));
}

fn translator_scalar_geometric_product(self_792: Translator, other_668: Scalar) -> Translator {
    var self_793: Translator;
    var other_669: Scalar;

    self_793 = self_792;
    other_669 = other_668;
    let _e4: Translator = self_793;
    let _e6: Scalar = other_669;
    return Translator((_e4.g0_ * vec3<f32>(_e6.g0_)));
}

fn translator_scalar_outer_product(self_794: Translator, other_670: Scalar) -> Translator {
    var self_795: Translator;
    var other_671: Scalar;

    self_795 = self_794;
    other_671 = other_670;
    let _e4: Translator = self_795;
    let _e6: Scalar = other_671;
    return Translator((_e4.g0_ * vec3<f32>(_e6.g0_)));
}

fn translator_scalar_inner_product(self_796: Translator, other_672: Scalar) -> Translator {
    var self_797: Translator;
    var other_673: Scalar;

    self_797 = self_796;
    other_673 = other_672;
    let _e4: Translator = self_797;
    let _e6: Scalar = other_673;
    return Translator((_e4.g0_ * vec3<f32>(_e6.g0_)));
}

fn translator_scalar_left_contraction(self_798: Translator, other_674: Scalar) -> Scalar {
    var self_799: Translator;
    var other_675: Scalar;

    self_799 = self_798;
    other_675 = other_674;
    let _e4: Translator = self_799;
    let _e7: Scalar = other_675;
    return Scalar((_e4.g0_.x * _e7.g0_));
}

fn translator_scalar_right_contraction(self_800: Translator, other_676: Scalar) -> Translator {
    var self_801: Translator;
    var other_677: Scalar;

    self_801 = self_800;
    other_677 = other_676;
    let _e4: Translator = self_801;
    let _e6: Scalar = other_677;
    return Translator((_e4.g0_ * vec3<f32>(_e6.g0_)));
}

fn translator_scalar_scalar_product(self_802: Translator, other_678: Scalar) -> Scalar {
    var self_803: Translator;
    var other_679: Scalar;

    self_803 = self_802;
    other_679 = other_678;
    let _e4: Translator = self_803;
    let _e7: Scalar = other_679;
    return Scalar((_e4.g0_.x * _e7.g0_));
}

fn translator_multi_vector_add(self_804: Translator, other_680: MultiVector) -> MultiVector {
    var self_805: Translator;
    var other_681: MultiVector;

    self_805 = self_804;
    other_681 = other_680;
    let _e4: Translator = self_805;
    let _e14: MultiVector = other_681;
    let _e17: Translator = self_805;
    let _e20: Translator = self_805;
    let _e23: Translator = self_805;
    let _e26: Translator = self_805;
    let _e36: MultiVector = other_681;
    return MultiVector(((vec4<f32>(_e4.g0_.x) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) + _e14.g0_), ((vec4<f32>(_e17.g0_.x, _e20.g0_.x, _e23.g0_.y, _e26.g0_.z) * vec4<f32>(0.0, 0.0, 1.0, 1.0)) + _e36.g1_));
}

fn translator_multi_vector_sub(self_806: Translator, other_682: MultiVector) -> MultiVector {
    var self_807: Translator;
    var other_683: MultiVector;

    self_807 = self_806;
    other_683 = other_682;
    let _e4: Translator = self_807;
    let _e14: MultiVector = other_683;
    let _e17: Translator = self_807;
    let _e20: Translator = self_807;
    let _e23: Translator = self_807;
    let _e26: Translator = self_807;
    let _e36: MultiVector = other_683;
    return MultiVector(((vec4<f32>(_e4.g0_.x) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) - _e14.g0_), ((vec4<f32>(_e17.g0_.x, _e20.g0_.x, _e23.g0_.y, _e26.g0_.z) * vec4<f32>(0.0, 0.0, 1.0, 1.0)) - _e36.g1_));
}

fn translator_multi_vector_geometric_product(self_808: Translator, other_684: MultiVector) -> MultiVector {
    var self_809: Translator;
    var other_685: MultiVector;

    self_809 = self_808;
    other_685 = other_684;
    let _e4: Translator = self_809;
    let _e8: MultiVector = other_685;
    let _e11: Translator = self_809;
    let _e15: MultiVector = other_685;
    let _e18: Translator = self_809;
    let _e22: MultiVector = other_685;
    let _e34: Translator = self_809;
    let _e38: MultiVector = other_685;
    return MultiVector((vec4<f32>(_e4.g0_.x) * _e8.g0_), (((vec4<f32>(_e11.g0_.x) * _e15.g1_) + ((vec4<f32>(_e18.g0_.y) * _e22.g0_.zwxy) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e34.g0_.z) * _e38.g0_.wzyx) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0))));
}

fn translator_multi_vector_outer_product(self_810: Translator, other_686: MultiVector) -> MultiVector {
    var self_811: Translator;
    var other_687: MultiVector;

    self_811 = self_810;
    other_687 = other_686;
    let _e4: Translator = self_811;
    let _e8: MultiVector = other_687;
    let _e11: Translator = self_811;
    let _e15: MultiVector = other_687;
    let _e18: Translator = self_811;
    let _e22: MultiVector = other_687;
    let _e33: Translator = self_811;
    let _e36: Translator = self_811;
    let _e39: Translator = self_811;
    let _e42: Translator = self_811;
    let _e46: MultiVector = other_687;
    return MultiVector((vec4<f32>(_e4.g0_.x) * _e8.g0_), (((vec4<f32>(_e11.g0_.x) * _e15.g1_) + ((vec4<f32>(_e18.g0_.z) * _e22.g0_.zzzx) * vec4<f32>(0.0, 1.0, 0.0, 1.0))) + ((vec4<f32>(_e33.g0_.x, _e36.g0_.y, _e39.g0_.y, _e42.g0_.x) * _e46.g0_.xwxx) * vec4<f32>(0.0, 1.0, 1.0, 0.0))));
}

fn translator_multi_vector_inner_product(self_812: Translator, other_688: MultiVector) -> MultiVector {
    var self_813: Translator;
    var other_689: MultiVector;

    self_813 = self_812;
    other_689 = other_688;
    let _e4: Translator = self_813;
    let _e8: MultiVector = other_689;
    let _e11: Translator = self_813;
    let _e15: MultiVector = other_689;
    let _e18: Translator = self_813;
    let _e22: MultiVector = other_689;
    let _e34: Translator = self_813;
    let _e37: Translator = self_813;
    let _e40: Translator = self_813;
    let _e43: Translator = self_813;
    let _e47: MultiVector = other_689;
    return MultiVector((vec4<f32>(_e4.g0_.x) * _e8.g0_), (((vec4<f32>(_e11.g0_.x) * _e15.g1_) + ((vec4<f32>(_e18.g0_.z) * _e22.g0_.wwwx) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))) + ((vec4<f32>(_e34.g0_.y, _e37.g0_.x, _e40.g0_.y, _e43.g0_.x) * _e47.g0_.zxxx) * vec4<f32>(1.0, 0.0, 1.0, 0.0))));
}

fn translator_multi_vector_left_contraction(self_814: Translator, other_690: MultiVector) -> MultiVector {
    var self_815: Translator;
    var other_691: MultiVector;

    self_815 = self_814;
    other_691 = other_690;
    let _e4: Translator = self_815;
    let _e8: MultiVector = other_691;
    let _e11: Translator = self_815;
    let _e15: MultiVector = other_691;
    return MultiVector((vec4<f32>(_e4.g0_.x) * _e8.g0_), (vec4<f32>(_e11.g0_.x) * _e15.g1_));
}

fn translator_multi_vector_scalar_product(self_816: Translator, other_692: MultiVector) -> Scalar {
    var self_817: Translator;
    var other_693: MultiVector;

    self_817 = self_816;
    other_693 = other_692;
    let _e4: Translator = self_817;
    let _e7: MultiVector = other_693;
    return Scalar((_e4.g0_.x * _e7.g0_.x));
}

fn translator_rotor_add(self_818: Translator, other_694: Rotor) -> Motor {
    var self_819: Translator;
    var other_695: Rotor;

    self_819 = self_818;
    other_695 = other_694;
    let _e4: Translator = self_819;
    let _e7: Translator = self_819;
    let _e10: Translator = self_819;
    let _e13: Translator = self_819;
    let _e23: Rotor = other_695;
    let _e26: Rotor = other_695;
    let _e29: Rotor = other_695;
    let _e32: Rotor = other_695;
    return Motor(((vec4<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.y, _e13.g0_.z) * vec4<f32>(1.0, 0.0, 1.0, 1.0)) + (vec4<f32>(_e23.g0_.x, _e26.g0_.y, _e29.g0_.x, _e32.g0_.x) * vec4<f32>(1.0, 1.0, 0.0, 0.0))));
}

fn translator_rotor_sub(self_820: Translator, other_696: Rotor) -> Motor {
    var self_821: Translator;
    var other_697: Rotor;

    self_821 = self_820;
    other_697 = other_696;
    let _e4: Translator = self_821;
    let _e7: Translator = self_821;
    let _e10: Translator = self_821;
    let _e13: Translator = self_821;
    let _e23: Rotor = other_697;
    let _e26: Rotor = other_697;
    let _e29: Rotor = other_697;
    let _e32: Rotor = other_697;
    return Motor(((vec4<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.y, _e13.g0_.z) * vec4<f32>(1.0, 0.0, 1.0, 1.0)) - (vec4<f32>(_e23.g0_.x, _e26.g0_.y, _e29.g0_.x, _e32.g0_.x) * vec4<f32>(1.0, 1.0, 0.0, 0.0))));
}

fn translator_rotor_geometric_product(self_822: Translator, other_698: Rotor) -> Motor {
    var self_823: Translator;
    var other_699: Rotor;

    self_823 = self_822;
    other_699 = other_698;
    let _e4: Translator = self_823;
    let _e8: Rotor = other_699;
    let _e11: Rotor = other_699;
    let _e14: Rotor = other_699;
    let _e17: Rotor = other_699;
    let _e28: Translator = self_823;
    let _e31: Translator = self_823;
    let _e34: Translator = self_823;
    let _e37: Translator = self_823;
    let _e41: Rotor = other_699;
    let _e44: Rotor = other_699;
    let _e47: Rotor = other_699;
    let _e50: Rotor = other_699;
    return Motor((((vec4<f32>(_e4.g0_.z) * vec4<f32>(_e8.g0_.y, _e11.g0_.y, _e14.g0_.y, _e17.g0_.x)) * vec4<f32>(0.0, 0.0, 1.0, 1.0)) + ((vec4<f32>(_e28.g0_.x, _e31.g0_.x, _e34.g0_.y, _e37.g0_.y) * vec4<f32>(_e41.g0_.x, _e44.g0_.y, _e47.g0_.x, _e50.g0_.y)) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))));
}

fn translator_rotor_outer_product(self_824: Translator, other_700: Rotor) -> Motor {
    var self_825: Translator;
    var other_701: Rotor;

    self_825 = self_824;
    other_701 = other_700;
    let _e4: Translator = self_825;
    let _e7: Translator = self_825;
    let _e10: Translator = self_825;
    let _e13: Translator = self_825;
    let _e17: Rotor = other_701;
    let _e20: Rotor = other_701;
    let _e23: Rotor = other_701;
    let _e26: Rotor = other_701;
    return Motor((vec4<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.y, _e13.g0_.z) * vec4<f32>(_e17.g0_.x, _e20.g0_.y, _e23.g0_.x, _e26.g0_.x)));
}

fn translator_rotor_inner_product(self_826: Translator, other_702: Rotor) -> Motor {
    var self_827: Translator;
    var other_703: Rotor;

    self_827 = self_826;
    other_703 = other_702;
    let _e4: Translator = self_827;
    let _e7: Translator = self_827;
    let _e10: Translator = self_827;
    let _e13: Translator = self_827;
    let _e17: Rotor = other_703;
    let _e20: Rotor = other_703;
    let _e23: Rotor = other_703;
    let _e26: Rotor = other_703;
    return Motor((vec4<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.y, _e13.g0_.z) * vec4<f32>(_e17.g0_.x, _e20.g0_.y, _e23.g0_.x, _e26.g0_.x)));
}

fn translator_rotor_left_contraction(self_828: Translator, other_704: Rotor) -> Rotor {
    var self_829: Translator;
    var other_705: Rotor;

    self_829 = self_828;
    other_705 = other_704;
    let _e4: Translator = self_829;
    let _e8: Rotor = other_705;
    return Rotor((vec2<f32>(_e4.g0_.x) * _e8.g0_));
}

fn translator_rotor_right_contraction(self_830: Translator, other_706: Rotor) -> Translator {
    var self_831: Translator;
    var other_707: Rotor;

    self_831 = self_830;
    other_707 = other_706;
    let _e4: Translator = self_831;
    let _e6: Rotor = other_707;
    return Translator((_e4.g0_ * vec3<f32>(_e6.g0_.x)));
}

fn translator_rotor_scalar_product(self_832: Translator, other_708: Rotor) -> Scalar {
    var self_833: Translator;
    var other_709: Rotor;

    self_833 = self_832;
    other_709 = other_708;
    let _e4: Translator = self_833;
    let _e7: Rotor = other_709;
    return Scalar((_e4.g0_.x * _e7.g0_.x));
}

fn translator_point_add(self_834: Translator, other_710: Point) -> Motor {
    var self_835: Translator;
    var other_711: Point;

    self_835 = self_834;
    other_711 = other_710;
    let _e4: Translator = self_835;
    let _e7: Translator = self_835;
    let _e10: Translator = self_835;
    let _e13: Translator = self_835;
    let _e23: Point = other_711;
    let _e26: Point = other_711;
    let _e29: Point = other_711;
    let _e32: Point = other_711;
    return Motor(((vec4<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.y, _e13.g0_.z) * vec4<f32>(1.0, 0.0, 1.0, 1.0)) + (vec4<f32>(_e23.g0_.x, _e26.g0_.x, _e29.g0_.y, _e32.g0_.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn translator_point_sub(self_836: Translator, other_712: Point) -> Motor {
    var self_837: Translator;
    var other_713: Point;

    self_837 = self_836;
    other_713 = other_712;
    let _e4: Translator = self_837;
    let _e7: Translator = self_837;
    let _e10: Translator = self_837;
    let _e13: Translator = self_837;
    let _e23: Point = other_713;
    let _e26: Point = other_713;
    let _e29: Point = other_713;
    let _e32: Point = other_713;
    return Motor(((vec4<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.y, _e13.g0_.z) * vec4<f32>(1.0, 0.0, 1.0, 1.0)) - (vec4<f32>(_e23.g0_.x, _e26.g0_.x, _e29.g0_.y, _e32.g0_.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn translator_point_geometric_product(self_838: Translator, other_714: Point) -> Point {
    var self_839: Translator;
    var other_715: Point;

    self_839 = self_838;
    other_715 = other_714;
    let _e4: Translator = self_839;
    let _e8: Point = other_715;
    let _e11: Translator = self_839;
    let _e14: Point = other_715;
    return Point(((vec3<f32>(_e4.g0_.x) * _e8.g0_) + ((_e11.g0_.xzy * vec3<f32>(_e14.g0_.x)) * vec3<f32>(0.0, 1.0, -(1.0)))));
}

fn translator_point_regressive_product(self_840: Translator, other_716: Point) -> Plane {
    var self_841: Translator;
    var other_717: Point;

    self_841 = self_840;
    other_717 = other_716;
    let _e4: Translator = self_841;
    let _e8: Point = other_717;
    let _e18: Translator = self_841;
    let _e21: Point = other_717;
    return Plane((((vec3<f32>(_e4.g0_.z) * _e8.g0_.yxy) * vec3<f32>(1.0, -(1.0), 0.0)) + ((_e18.g0_.yxy * _e21.g0_.zxx) * vec3<f32>(-(1.0), 0.0, 1.0))));
}

fn translator_point_outer_product(self_842: Translator, other_718: Point) -> Point {
    var self_843: Translator;
    var other_719: Point;

    self_843 = self_842;
    other_719 = other_718;
    let _e4: Translator = self_843;
    let _e8: Point = other_719;
    return Point((vec3<f32>(_e4.g0_.x) * _e8.g0_));
}

fn translator_point_inner_product(self_844: Translator, other_720: Point) -> Point {
    var self_845: Translator;
    var other_721: Point;

    self_845 = self_844;
    other_721 = other_720;
    let _e4: Translator = self_845;
    let _e8: Point = other_721;
    return Point((vec3<f32>(_e4.g0_.x) * _e8.g0_));
}

fn translator_point_left_contraction(self_846: Translator, other_722: Point) -> Point {
    var self_847: Translator;
    var other_723: Point;

    self_847 = self_846;
    other_723 = other_722;
    let _e4: Translator = self_847;
    let _e8: Point = other_723;
    return Point((vec3<f32>(_e4.g0_.x) * _e8.g0_));
}

fn translator_ideal_point_into(self_848: Translator) -> IdealPoint {
    var self_849: Translator;

    self_849 = self_848;
    let _e2: Translator = self_849;
    let _e5: Translator = self_849;
    return IdealPoint(vec2<f32>(_e2.g0_.y, _e5.g0_.z));
}

fn translator_ideal_point_add(self_850: Translator, other_724: IdealPoint) -> Translator {
    var self_851: Translator;
    var other_725: IdealPoint;

    self_851 = self_850;
    other_725 = other_724;
    let _e4: Translator = self_851;
    let _e6: IdealPoint = other_725;
    let _e9: IdealPoint = other_725;
    let _e12: IdealPoint = other_725;
    return Translator((_e4.g0_ + (vec3<f32>(_e6.g0_.x, _e9.g0_.x, _e12.g0_.y) * vec3<f32>(0.0, 1.0, 1.0))));
}

fn translator_ideal_point_sub(self_852: Translator, other_726: IdealPoint) -> Translator {
    var self_853: Translator;
    var other_727: IdealPoint;

    self_853 = self_852;
    other_727 = other_726;
    let _e4: Translator = self_853;
    let _e6: IdealPoint = other_727;
    let _e9: IdealPoint = other_727;
    let _e12: IdealPoint = other_727;
    return Translator((_e4.g0_ - (vec3<f32>(_e6.g0_.x, _e9.g0_.x, _e12.g0_.y) * vec3<f32>(0.0, 1.0, 1.0))));
}

fn translator_ideal_point_geometric_product(self_854: Translator, other_728: IdealPoint) -> IdealPoint {
    var self_855: Translator;
    var other_729: IdealPoint;

    self_855 = self_854;
    other_729 = other_728;
    let _e4: Translator = self_855;
    let _e8: IdealPoint = other_729;
    return IdealPoint((vec2<f32>(_e4.g0_.x) * _e8.g0_));
}

fn translator_ideal_point_outer_product(self_856: Translator, other_730: IdealPoint) -> IdealPoint {
    var self_857: Translator;
    var other_731: IdealPoint;

    self_857 = self_856;
    other_731 = other_730;
    let _e4: Translator = self_857;
    let _e8: IdealPoint = other_731;
    return IdealPoint((vec2<f32>(_e4.g0_.x) * _e8.g0_));
}

fn translator_ideal_point_inner_product(self_858: Translator, other_732: IdealPoint) -> IdealPoint {
    var self_859: Translator;
    var other_733: IdealPoint;

    self_859 = self_858;
    other_733 = other_732;
    let _e4: Translator = self_859;
    let _e8: IdealPoint = other_733;
    return IdealPoint((vec2<f32>(_e4.g0_.x) * _e8.g0_));
}

fn translator_ideal_point_left_contraction(self_860: Translator, other_734: IdealPoint) -> IdealPoint {
    var self_861: Translator;
    var other_735: IdealPoint;

    self_861 = self_860;
    other_735 = other_734;
    let _e4: Translator = self_861;
    let _e8: IdealPoint = other_735;
    return IdealPoint((vec2<f32>(_e4.g0_.x) * _e8.g0_));
}

fn translator_plane_geometric_product(self_862: Translator, other_736: Plane) -> MotorDual {
    var self_863: Translator;
    var other_737: Plane;

    self_863 = self_862;
    other_737 = other_736;
    let _e4: Translator = self_863;
    let _e8: Plane = other_737;
    let _e11: Plane = other_737;
    let _e14: Plane = other_737;
    let _e17: Plane = other_737;
    let _e28: Translator = self_863;
    let _e32: Plane = other_737;
    let _e35: Plane = other_737;
    let _e38: Plane = other_737;
    let _e41: Plane = other_737;
    let _e54: Translator = self_863;
    let _e58: Plane = other_737;
    let _e61: Plane = other_737;
    let _e64: Plane = other_737;
    let _e67: Plane = other_737;
    return MotorDual(((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y, _e11.g0_.z, _e14.g0_.y, _e17.g0_.y)) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) + ((vec4<f32>(_e28.g0_.z) * vec4<f32>(_e32.g0_.z, _e35.g0_.y, _e38.g0_.z, _e41.g0_.z)) * vec4<f32>(1.0, -(1.0), 0.0, 0.0))) + ((vec4<f32>(_e54.g0_.x) * vec4<f32>(_e58.g0_.x, _e61.g0_.x, _e64.g0_.y, _e67.g0_.z)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn translator_plane_regressive_product(self_864: Translator, other_738: Plane) -> Scalar {
    var self_865: Translator;
    var other_739: Plane;

    self_865 = self_864;
    other_739 = other_738;
    let _e4: Translator = self_865;
    let _e7: Plane = other_739;
    let _e11: Translator = self_865;
    let _e14: Plane = other_739;
    return Scalar(((_e4.g0_.y * _e7.g0_.y) + (_e11.g0_.z * _e14.g0_.z)));
}

fn translator_plane_outer_product(self_866: Translator, other_740: Plane) -> MotorDual {
    var self_867: Translator;
    var other_741: Plane;

    self_867 = self_866;
    other_741 = other_740;
    let _e4: Translator = self_867;
    let _e8: Plane = other_741;
    let _e19: Translator = self_867;
    let _e22: Translator = self_867;
    let _e25: Translator = self_867;
    let _e28: Translator = self_867;
    let _e32: Plane = other_741;
    let _e35: Plane = other_741;
    let _e38: Plane = other_741;
    let _e41: Plane = other_741;
    return MotorDual((((vec4<f32>(_e4.g0_.z) * vec4<f32>(_e8.g0_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) + (vec4<f32>(_e19.g0_.y, _e22.g0_.x, _e25.g0_.x, _e28.g0_.x) * vec4<f32>(_e32.g0_.y, _e35.g0_.x, _e38.g0_.y, _e41.g0_.z))));
}

fn translator_plane_inner_product(self_868: Translator, other_742: Plane) -> Plane {
    var self_869: Translator;
    var other_743: Plane;

    self_869 = self_868;
    other_743 = other_742;
    let _e4: Translator = self_869;
    let _e8: Plane = other_743;
    let _e11: Translator = self_869;
    let _e15: Plane = other_743;
    let _e27: Translator = self_869;
    let _e30: Plane = other_743;
    return Plane((((vec3<f32>(_e4.g0_.x) * _e8.g0_) + ((vec3<f32>(_e11.g0_.z) * vec3<f32>(_e15.g0_.y)) * vec3<f32>(-(1.0), 0.0, 0.0))) + ((_e27.g0_.yxx * _e30.g0_.zxx) * vec3<f32>(1.0, 0.0, 0.0))));
}

fn translator_plane_left_contraction(self_870: Translator, other_744: Plane) -> Plane {
    var self_871: Translator;
    var other_745: Plane;

    self_871 = self_870;
    other_745 = other_744;
    let _e4: Translator = self_871;
    let _e8: Plane = other_745;
    return Plane((vec3<f32>(_e4.g0_.x) * _e8.g0_));
}

fn translator_translator_add(self_872: Translator, other_746: Translator) -> Translator {
    var self_873: Translator;
    var other_747: Translator;

    self_873 = self_872;
    other_747 = other_746;
    let _e4: Translator = self_873;
    let _e6: Translator = other_747;
    return Translator((_e4.g0_ + _e6.g0_));
}

fn translator_translator_sub(self_874: Translator, other_748: Translator) -> Translator {
    var self_875: Translator;
    var other_749: Translator;

    self_875 = self_874;
    other_749 = other_748;
    let _e4: Translator = self_875;
    let _e6: Translator = other_749;
    return Translator((_e4.g0_ - _e6.g0_));
}

fn translator_translator_mul(self_876: Translator, other_750: Translator) -> Translator {
    var self_877: Translator;
    var other_751: Translator;

    self_877 = self_876;
    other_751 = other_750;
    let _e4: Translator = self_877;
    let _e6: Translator = other_751;
    return Translator((_e4.g0_ * _e6.g0_));
}

fn translator_translator_div(self_878: Translator, other_752: Translator) -> Translator {
    var self_879: Translator;
    var other_753: Translator;

    self_879 = self_878;
    other_753 = other_752;
    let _e4: Translator = self_879;
    let _e7: Translator = self_879;
    let _e10: Translator = self_879;
    let _e19: Translator = other_753;
    let _e22: Translator = other_753;
    let _e25: Translator = other_753;
    return Translator((((vec3<f32>(_e4.g0_.x, _e7.g0_.y, _e10.g0_.z) * vec3<f32>(1.0, 1.0, 1.0)) / vec3<f32>(_e19.g0_.x, _e22.g0_.y, _e25.g0_.z)) * vec3<f32>(1.0, 1.0, 1.0)));
}

fn translator_translator_geometric_product(self_880: Translator, other_754: Translator) -> Translator {
    var self_881: Translator;
    var other_755: Translator;

    self_881 = self_880;
    other_755 = other_754;
    let _e4: Translator = self_881;
    let _e8: Translator = other_755;
    let _e11: Translator = self_881;
    let _e13: Translator = other_755;
    return Translator(((vec3<f32>(_e4.g0_.x) * _e8.g0_) + ((_e11.g0_ * vec3<f32>(_e13.g0_.x)) * vec3<f32>(0.0, 1.0, 1.0))));
}

fn translator_translator_outer_product(self_882: Translator, other_756: Translator) -> Translator {
    var self_883: Translator;
    var other_757: Translator;

    self_883 = self_882;
    other_757 = other_756;
    let _e4: Translator = self_883;
    let _e8: Translator = other_757;
    let _e11: Translator = self_883;
    let _e13: Translator = other_757;
    return Translator(((vec3<f32>(_e4.g0_.x) * _e8.g0_) + ((_e11.g0_ * vec3<f32>(_e13.g0_.x)) * vec3<f32>(0.0, 1.0, 1.0))));
}

fn translator_translator_inner_product(self_884: Translator, other_758: Translator) -> Translator {
    var self_885: Translator;
    var other_759: Translator;

    self_885 = self_884;
    other_759 = other_758;
    let _e4: Translator = self_885;
    let _e8: Translator = other_759;
    let _e11: Translator = self_885;
    let _e13: Translator = other_759;
    return Translator(((vec3<f32>(_e4.g0_.x) * _e8.g0_) + ((_e11.g0_ * vec3<f32>(_e13.g0_.x)) * vec3<f32>(0.0, 1.0, 1.0))));
}

fn translator_translator_left_contraction(self_886: Translator, other_760: Translator) -> Translator {
    var self_887: Translator;
    var other_761: Translator;

    self_887 = self_886;
    other_761 = other_760;
    let _e4: Translator = self_887;
    let _e8: Translator = other_761;
    return Translator((vec3<f32>(_e4.g0_.x) * _e8.g0_));
}

fn translator_translator_right_contraction(self_888: Translator, other_762: Translator) -> Translator {
    var self_889: Translator;
    var other_763: Translator;

    self_889 = self_888;
    other_763 = other_762;
    let _e4: Translator = self_889;
    let _e6: Translator = other_763;
    return Translator((_e4.g0_ * vec3<f32>(_e6.g0_.x)));
}

fn translator_translator_scalar_product(self_890: Translator, other_764: Translator) -> Scalar {
    var self_891: Translator;
    var other_765: Translator;

    self_891 = self_890;
    other_765 = other_764;
    let _e4: Translator = self_891;
    let _e7: Translator = other_765;
    return Scalar((_e4.g0_.x * _e7.g0_.x));
}

fn translator_motor_add(self_892: Translator, other_766: Motor) -> Motor {
    var self_893: Translator;
    var other_767: Motor;

    self_893 = self_892;
    other_767 = other_766;
    let _e4: Translator = self_893;
    let _e7: Translator = self_893;
    let _e10: Translator = self_893;
    let _e13: Translator = self_893;
    let _e23: Motor = other_767;
    return Motor(((vec4<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.y, _e13.g0_.z) * vec4<f32>(1.0, 0.0, 1.0, 1.0)) + _e23.g0_));
}

fn translator_motor_sub(self_894: Translator, other_768: Motor) -> Motor {
    var self_895: Translator;
    var other_769: Motor;

    self_895 = self_894;
    other_769 = other_768;
    let _e4: Translator = self_895;
    let _e7: Translator = self_895;
    let _e10: Translator = self_895;
    let _e13: Translator = self_895;
    let _e23: Motor = other_769;
    return Motor(((vec4<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.y, _e13.g0_.z) * vec4<f32>(1.0, 0.0, 1.0, 1.0)) - _e23.g0_));
}

fn translator_motor_geometric_product(self_896: Translator, other_770: Motor) -> Motor {
    var self_897: Translator;
    var other_771: Motor;

    self_897 = self_896;
    other_771 = other_770;
    let _e4: Translator = self_897;
    let _e8: Motor = other_771;
    let _e11: Translator = self_897;
    let _e15: Motor = other_771;
    let _e26: Translator = self_897;
    let _e29: Translator = self_897;
    let _e32: Translator = self_897;
    let _e35: Translator = self_897;
    let _e39: Motor = other_771;
    return Motor((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.z) * _e15.g0_.yyyx) * vec4<f32>(0.0, 0.0, 1.0, 1.0))) + ((vec4<f32>(_e26.g0_.x, _e29.g0_.x, _e32.g0_.y, _e35.g0_.y) * _e39.g0_.xxxy) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))));
}

fn translator_motor_regressive_product(self_898: Translator, other_772: Motor) -> Plane {
    var self_899: Translator;
    var other_773: Motor;

    self_899 = self_898;
    other_773 = other_772;
    let _e4: Translator = self_899;
    let _e8: Motor = other_773;
    let _e11: Motor = other_773;
    let _e14: Motor = other_773;
    let _e25: Translator = self_899;
    let _e28: Motor = other_773;
    let _e31: Motor = other_773;
    let _e34: Motor = other_773;
    return Plane((((vec3<f32>(_e4.g0_.z) * vec3<f32>(_e8.g0_.z, _e11.g0_.y, _e14.g0_.z)) * vec3<f32>(1.0, -(1.0), 0.0)) + ((_e25.g0_.yxy * vec3<f32>(_e28.g0_.w, _e31.g0_.x, _e34.g0_.y)) * vec3<f32>(-(1.0), 0.0, 1.0))));
}

fn translator_motor_outer_product(self_900: Translator, other_774: Motor) -> Motor {
    var self_901: Translator;
    var other_775: Motor;

    self_901 = self_900;
    other_775 = other_774;
    let _e4: Translator = self_901;
    let _e8: Motor = other_775;
    let _e11: Translator = self_901;
    let _e14: Translator = self_901;
    let _e17: Translator = self_901;
    let _e20: Translator = self_901;
    let _e24: Motor = other_775;
    return Motor(((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.x, _e14.g0_.x, _e17.g0_.y, _e20.g0_.z) * vec4<f32>(_e24.g0_.x)) * vec4<f32>(0.0, 0.0, 1.0, 1.0))));
}

fn translator_motor_inner_product(self_902: Translator, other_776: Motor) -> Motor {
    var self_903: Translator;
    var other_777: Motor;

    self_903 = self_902;
    other_777 = other_776;
    let _e4: Translator = self_903;
    let _e8: Motor = other_777;
    let _e11: Translator = self_903;
    let _e14: Translator = self_903;
    let _e17: Translator = self_903;
    let _e20: Translator = self_903;
    let _e24: Motor = other_777;
    return Motor(((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.x, _e14.g0_.x, _e17.g0_.y, _e20.g0_.z) * vec4<f32>(_e24.g0_.x)) * vec4<f32>(0.0, 0.0, 1.0, 1.0))));
}

fn translator_motor_left_contraction(self_904: Translator, other_778: Motor) -> Motor {
    var self_905: Translator;
    var other_779: Motor;

    self_905 = self_904;
    other_779 = other_778;
    let _e4: Translator = self_905;
    let _e8: Motor = other_779;
    return Motor((vec4<f32>(_e4.g0_.x) * _e8.g0_));
}

fn translator_motor_right_contraction(self_906: Translator, other_780: Motor) -> Translator {
    var self_907: Translator;
    var other_781: Motor;

    self_907 = self_906;
    other_781 = other_780;
    let _e4: Translator = self_907;
    let _e6: Motor = other_781;
    return Translator((_e4.g0_ * vec3<f32>(_e6.g0_.x)));
}

fn translator_motor_scalar_product(self_908: Translator, other_782: Motor) -> Scalar {
    var self_909: Translator;
    var other_783: Motor;

    self_909 = self_908;
    other_783 = other_782;
    let _e4: Translator = self_909;
    let _e7: Motor = other_783;
    return Scalar((_e4.g0_.x * _e7.g0_.x));
}

fn translator_motor_dual_geometric_product(self_910: Translator, other_784: MotorDual) -> MotorDual {
    var self_911: Translator;
    var other_785: MotorDual;

    self_911 = self_910;
    other_785 = other_784;
    let _e4: Translator = self_911;
    let _e8: MotorDual = other_785;
    let _e11: Translator = self_911;
    let _e15: MotorDual = other_785;
    let _e27: Translator = self_911;
    let _e30: Translator = self_911;
    let _e33: Translator = self_911;
    let _e36: Translator = self_911;
    let _e40: MotorDual = other_785;
    return MotorDual((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.z) * _e15.g0_.wzww) * vec4<f32>(1.0, -(1.0), 0.0, 0.0))) + ((vec4<f32>(_e27.g0_.y, _e30.g0_.y, _e33.g0_.x, _e36.g0_.x) * _e40.g0_.zwxx) * vec4<f32>(1.0, 1.0, 0.0, 0.0))));
}

fn translator_motor_dual_regressive_product(self_912: Translator, other_786: MotorDual) -> Translator {
    var self_913: Translator;
    var other_787: MotorDual;

    self_913 = self_912;
    other_787 = other_786;
    let _e4: Translator = self_913;
    let _e8: MotorDual = other_787;
    let _e11: MotorDual = other_787;
    let _e14: MotorDual = other_787;
    let _e24: Translator = self_913;
    let _e28: MotorDual = other_787;
    let _e31: MotorDual = other_787;
    let _e34: MotorDual = other_787;
    let _e45: Translator = self_913;
    let _e49: MotorDual = other_787;
    return Translator(((((vec3<f32>(_e4.g0_.y) * vec3<f32>(_e8.g0_.z, _e11.g0_.x, _e14.g0_.z)) * vec3<f32>(1.0, 1.0, 0.0)) + ((vec3<f32>(_e24.g0_.z) * vec3<f32>(_e28.g0_.w, _e31.g0_.w, _e34.g0_.x)) * vec3<f32>(1.0, 0.0, 1.0))) + ((vec3<f32>(_e45.g0_.x) * vec3<f32>(_e49.g0_.x)) * vec3<f32>(1.0, 0.0, 0.0))));
}

fn translator_motor_dual_outer_product(self_914: Translator, other_788: MotorDual) -> MotorDual {
    var self_915: Translator;
    var other_789: MotorDual;

    self_915 = self_914;
    other_789 = other_788;
    let _e4: Translator = self_915;
    let _e8: MotorDual = other_789;
    let _e11: Translator = self_915;
    let _e15: MotorDual = other_789;
    let _e27: Translator = self_915;
    let _e30: Translator = self_915;
    let _e33: Translator = self_915;
    let _e36: Translator = self_915;
    let _e40: MotorDual = other_789;
    return MotorDual((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.z) * vec4<f32>(_e15.g0_.w)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e27.g0_.y, _e30.g0_.x, _e33.g0_.x, _e36.g0_.x) * _e40.g0_.zxxx) * vec4<f32>(1.0, 0.0, 0.0, 0.0))));
}

fn translator_motor_dual_inner_product(self_916: Translator, other_790: MotorDual) -> MotorDual {
    var self_917: Translator;
    var other_791: MotorDual;

    self_917 = self_916;
    other_791 = other_790;
    let _e4: Translator = self_917;
    let _e8: MotorDual = other_791;
    let _e11: Translator = self_917;
    let _e15: MotorDual = other_791;
    let _e28: Translator = self_917;
    let _e31: Translator = self_917;
    let _e34: Translator = self_917;
    let _e37: Translator = self_917;
    let _e41: MotorDual = other_791;
    return MotorDual((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.z) * vec4<f32>(_e15.g0_.z)) * vec4<f32>(0.0, -(1.0), 0.0, 0.0))) + ((vec4<f32>(_e28.g0_.x, _e31.g0_.y, _e34.g0_.x, _e37.g0_.x) * _e41.g0_.xwxx) * vec4<f32>(0.0, 1.0, 0.0, 0.0))));
}

fn translator_motor_dual_left_contraction(self_918: Translator, other_792: MotorDual) -> MotorDual {
    var self_919: Translator;
    var other_793: MotorDual;

    self_919 = self_918;
    other_793 = other_792;
    let _e4: Translator = self_919;
    let _e8: MotorDual = other_793;
    return MotorDual((vec4<f32>(_e4.g0_.x) * _e8.g0_));
}

fn translator_squared_magnitude(self_920: Translator) -> Scalar {
    var self_921: Translator;

    self_921 = self_920;
    let _e4: Translator = self_921;
    let _e5: Translator = translator_reversal(_e4);
    let _e6: Translator = self_921;
    let _e8: Translator = self_921;
    let _e9: Translator = translator_reversal(_e8);
    let _e10: Scalar = translator_translator_scalar_product(_e6, _e9);
    return _e10;
}

fn translator_magnitude(self_922: Translator) -> Scalar {
    var self_923: Translator;

    self_923 = self_922;
    let _e3: Translator = self_923;
    let _e4: Scalar = translator_squared_magnitude(_e3);
    let _e7: Translator = self_923;
    let _e8: Scalar = translator_squared_magnitude(_e7);
    return Scalar(sqrt(_e8.g0_));
}

fn translator_scale(self_924: Translator, other_794: f32) -> Translator {
    var self_925: Translator;
    var other_795: f32;

    self_925 = self_924;
    other_795 = other_794;
    let _e5: f32 = other_795;
    let _e7: Translator = self_925;
    let _e8: f32 = other_795;
    let _e10: Translator = translator_scalar_geometric_product(_e7, Scalar(_e8));
    return _e10;
}

fn translator_signum(self_926: Translator) -> Translator {
    var self_927: Translator;

    self_927 = self_926;
    let _e5: Translator = self_927;
    let _e6: Scalar = translator_magnitude(_e5);
    let _e10: Translator = self_927;
    let _e13: Translator = self_927;
    let _e14: Scalar = translator_magnitude(_e13);
    let _e18: Translator = translator_scalar_geometric_product(_e10, Scalar((1.0 / _e14.g0_)));
    return _e18;
}

fn translator_inverse(self_928: Translator) -> Translator {
    var self_929: Translator;

    self_929 = self_928;
    let _e3: Translator = self_929;
    let _e4: Translator = translator_reversal(_e3);
    let _e7: Translator = self_929;
    let _e8: Scalar = translator_squared_magnitude(_e7);
    let _e13: Translator = self_929;
    let _e14: Translator = translator_reversal(_e13);
    let _e17: Translator = self_929;
    let _e18: Scalar = translator_squared_magnitude(_e17);
    let _e22: Translator = translator_scalar_geometric_product(_e14, Scalar((1.0 / _e18.g0_)));
    return _e22;
}

fn motor_zero() -> Motor {
    return Motor(vec4<f32>(0.0));
}

fn motor_one() -> Motor {
    return Motor(vec4<f32>(1.0, 0.0, 0.0, 0.0));
}

fn motor_neg(self_930: Motor) -> Motor {
    var self_931: Motor;

    self_931 = self_930;
    let _e2: Motor = self_931;
    return Motor((_e2.g0_ * vec4<f32>(-(1.0))));
}

fn motor_automorphism(self_932: Motor) -> Motor {
    var self_933: Motor;

    self_933 = self_932;
    let _e2: Motor = self_933;
    return Motor(_e2.g0_);
}

fn motor_reversal(self_934: Motor) -> Motor {
    var self_935: Motor;

    self_935 = self_934;
    let _e2: Motor = self_935;
    return Motor((_e2.g0_ * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))));
}

fn motor_conjugation(self_936: Motor) -> Motor {
    var self_937: Motor;

    self_937 = self_936;
    let _e2: Motor = self_937;
    return Motor((_e2.g0_ * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))));
}

fn motor_dual(self_938: Motor) -> MotorDual {
    var self_939: Motor;

    self_939 = self_938;
    let _e2: Motor = self_939;
    return MotorDual(_e2.g0_);
}

fn motor_scalar_into(self_940: Motor) -> Scalar {
    var self_941: Motor;

    self_941 = self_940;
    let _e2: Motor = self_941;
    return Scalar(_e2.g0_.x);
}

fn motor_scalar_add(self_942: Motor, other_796: Scalar) -> Motor {
    var self_943: Motor;
    var other_797: Scalar;

    self_943 = self_942;
    other_797 = other_796;
    let _e4: Motor = self_943;
    let _e6: Scalar = other_797;
    return Motor((_e4.g0_ + (vec4<f32>(_e6.g0_) * vec4<f32>(1.0, 0.0, 0.0, 0.0))));
}

fn motor_scalar_sub(self_944: Motor, other_798: Scalar) -> Motor {
    var self_945: Motor;
    var other_799: Scalar;

    self_945 = self_944;
    other_799 = other_798;
    let _e4: Motor = self_945;
    let _e6: Scalar = other_799;
    return Motor((_e4.g0_ - (vec4<f32>(_e6.g0_) * vec4<f32>(1.0, 0.0, 0.0, 0.0))));
}

fn motor_scalar_geometric_product(self_946: Motor, other_800: Scalar) -> Motor {
    var self_947: Motor;
    var other_801: Scalar;

    self_947 = self_946;
    other_801 = other_800;
    let _e4: Motor = self_947;
    let _e6: Scalar = other_801;
    return Motor((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn motor_scalar_outer_product(self_948: Motor, other_802: Scalar) -> Motor {
    var self_949: Motor;
    var other_803: Scalar;

    self_949 = self_948;
    other_803 = other_802;
    let _e4: Motor = self_949;
    let _e6: Scalar = other_803;
    return Motor((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn motor_scalar_inner_product(self_950: Motor, other_804: Scalar) -> Motor {
    var self_951: Motor;
    var other_805: Scalar;

    self_951 = self_950;
    other_805 = other_804;
    let _e4: Motor = self_951;
    let _e6: Scalar = other_805;
    return Motor((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn motor_scalar_left_contraction(self_952: Motor, other_806: Scalar) -> Scalar {
    var self_953: Motor;
    var other_807: Scalar;

    self_953 = self_952;
    other_807 = other_806;
    let _e4: Motor = self_953;
    let _e7: Scalar = other_807;
    return Scalar((_e4.g0_.x * _e7.g0_));
}

fn motor_scalar_right_contraction(self_954: Motor, other_808: Scalar) -> Motor {
    var self_955: Motor;
    var other_809: Scalar;

    self_955 = self_954;
    other_809 = other_808;
    let _e4: Motor = self_955;
    let _e6: Scalar = other_809;
    return Motor((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn motor_scalar_scalar_product(self_956: Motor, other_810: Scalar) -> Scalar {
    var self_957: Motor;
    var other_811: Scalar;

    self_957 = self_956;
    other_811 = other_810;
    let _e4: Motor = self_957;
    let _e7: Scalar = other_811;
    return Scalar((_e4.g0_.x * _e7.g0_));
}

fn motor_multi_vector_add(self_958: Motor, other_812: MultiVector) -> MultiVector {
    var self_959: Motor;
    var other_813: MultiVector;

    self_959 = self_958;
    other_813 = other_812;
    let _e4: Motor = self_959;
    let _e13: MultiVector = other_813;
    let _e16: Motor = self_959;
    let _e25: MultiVector = other_813;
    return MultiVector(((_e4.g0_.xyxx * vec4<f32>(1.0, 1.0, 0.0, 0.0)) + _e13.g0_), ((_e16.g0_.xxzw * vec4<f32>(0.0, 0.0, 1.0, 1.0)) + _e25.g1_));
}

fn motor_multi_vector_sub(self_960: Motor, other_814: MultiVector) -> MultiVector {
    var self_961: Motor;
    var other_815: MultiVector;

    self_961 = self_960;
    other_815 = other_814;
    let _e4: Motor = self_961;
    let _e13: MultiVector = other_815;
    let _e16: Motor = self_961;
    let _e25: MultiVector = other_815;
    return MultiVector(((_e4.g0_.xyxx * vec4<f32>(1.0, 1.0, 0.0, 0.0)) - _e13.g0_), ((_e16.g0_.xxzw * vec4<f32>(0.0, 0.0, 1.0, 1.0)) - _e25.g1_));
}

fn motor_multi_vector_geometric_product(self_962: Motor, other_816: MultiVector) -> MultiVector {
    var self_963: Motor;
    var other_817: MultiVector;

    self_963 = self_962;
    other_817 = other_816;
    let _e4: Motor = self_963;
    let _e8: MultiVector = other_817;
    let _e11: Motor = self_963;
    let _e15: MultiVector = other_817;
    let _e28: Motor = self_963;
    let _e32: MultiVector = other_817;
    let _e35: Motor = self_963;
    let _e39: MultiVector = other_817;
    let _e52: Motor = self_963;
    let _e56: MultiVector = other_817;
    let _e68: Motor = self_963;
    let _e72: MultiVector = other_817;
    return MultiVector(((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.y) * _e15.g0_.yxwz) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))), ((((vec4<f32>(_e28.g0_.x) * _e32.g1_) + ((vec4<f32>(_e35.g0_.y) * _e39.g1_.yxwz) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))) + ((vec4<f32>(_e52.g0_.z) * _e56.g0_.zwxy) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e68.g0_.w) * _e72.g0_.wzyx) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0))));
}

fn motor_multi_vector_outer_product(self_964: Motor, other_818: MultiVector) -> MultiVector {
    var self_965: Motor;
    var other_819: MultiVector;

    self_965 = self_964;
    other_819 = other_818;
    let _e4: Motor = self_965;
    let _e8: MultiVector = other_819;
    let _e11: Motor = self_965;
    let _e14: MultiVector = other_819;
    let _e26: Motor = self_965;
    let _e30: MultiVector = other_819;
    let _e33: Motor = self_965;
    let _e37: MultiVector = other_819;
    let _e48: Motor = self_965;
    let _e52: MultiVector = other_819;
    let _e63: Motor = self_965;
    let _e66: MultiVector = other_819;
    return MultiVector(((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((_e11.g0_.xyxx * vec4<f32>(_e14.g0_.x)) * vec4<f32>(0.0, 1.0, 0.0, 0.0))), ((((vec4<f32>(_e26.g0_.x) * _e30.g1_) + ((vec4<f32>(_e33.g0_.z) * _e37.g0_.wwxw) * vec4<f32>(0.0, 1.0, 1.0, 0.0))) + ((vec4<f32>(_e48.g0_.w) * _e52.g0_.zzzx) * vec4<f32>(0.0, 1.0, 0.0, 1.0))) + ((_e63.g0_.xyxx * vec4<f32>(_e66.g1_.x)) * vec4<f32>(0.0, 1.0, 0.0, 0.0))));
}

fn motor_multi_vector_inner_product(self_966: Motor, other_820: MultiVector) -> MultiVector {
    var self_967: Motor;
    var other_821: MultiVector;

    self_967 = self_966;
    other_821 = other_820;
    let _e4: Motor = self_967;
    let _e8: MultiVector = other_821;
    let _e11: Motor = self_967;
    let _e15: MultiVector = other_821;
    let _e28: Motor = self_967;
    let _e32: MultiVector = other_821;
    let _e35: Motor = self_967;
    let _e39: MultiVector = other_821;
    let _e50: Motor = self_967;
    let _e54: MultiVector = other_821;
    let _e66: Motor = self_967;
    let _e69: MultiVector = other_821;
    return MultiVector(((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.y) * _e15.g0_.yxwz) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))), ((((vec4<f32>(_e28.g0_.x) * _e32.g1_) + ((vec4<f32>(_e35.g0_.z) * _e39.g0_.zzxz) * vec4<f32>(1.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e50.g0_.w) * _e54.g0_.wwwx) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))) + ((_e66.g0_.yxxx * _e69.g1_.yxxx) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))));
}

fn motor_multi_vector_left_contraction(self_968: Motor, other_822: MultiVector) -> MultiVector {
    var self_969: Motor;
    var other_823: MultiVector;

    self_969 = self_968;
    other_823 = other_822;
    let _e4: Motor = self_969;
    let _e8: MultiVector = other_823;
    let _e11: Motor = self_969;
    let _e14: MultiVector = other_823;
    let _e26: Motor = self_969;
    let _e30: MultiVector = other_823;
    let _e33: Motor = self_969;
    let _e36: MultiVector = other_823;
    return MultiVector(((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((_e11.g0_.yxxx * _e14.g0_.yxxx) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))), ((vec4<f32>(_e26.g0_.x) * _e30.g1_) + ((_e33.g0_.yxxx * _e36.g1_.yxxx) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))));
}

fn motor_multi_vector_scalar_product(self_970: Motor, other_824: MultiVector) -> Scalar {
    var self_971: Motor;
    var other_825: MultiVector;

    self_971 = self_970;
    other_825 = other_824;
    let _e4: Motor = self_971;
    let _e7: MultiVector = other_825;
    let _e11: Motor = self_971;
    let _e14: MultiVector = other_825;
    return Scalar(((_e4.g0_.x * _e7.g0_.x) - (_e11.g0_.y * _e14.g0_.y)));
}

fn motor_rotor_into(self_972: Motor) -> Rotor {
    var self_973: Motor;

    self_973 = self_972;
    let _e2: Motor = self_973;
    let _e5: Motor = self_973;
    return Rotor(vec2<f32>(_e2.g0_.x, _e5.g0_.y));
}

fn motor_rotor_add(self_974: Motor, other_826: Rotor) -> Motor {
    var self_975: Motor;
    var other_827: Rotor;

    self_975 = self_974;
    other_827 = other_826;
    let _e4: Motor = self_975;
    let _e6: Rotor = other_827;
    let _e9: Rotor = other_827;
    let _e12: Rotor = other_827;
    let _e15: Rotor = other_827;
    return Motor((_e4.g0_ + (vec4<f32>(_e6.g0_.x, _e9.g0_.y, _e12.g0_.x, _e15.g0_.x) * vec4<f32>(1.0, 1.0, 0.0, 0.0))));
}

fn motor_rotor_sub(self_976: Motor, other_828: Rotor) -> Motor {
    var self_977: Motor;
    var other_829: Rotor;

    self_977 = self_976;
    other_829 = other_828;
    let _e4: Motor = self_977;
    let _e6: Rotor = other_829;
    let _e9: Rotor = other_829;
    let _e12: Rotor = other_829;
    let _e15: Rotor = other_829;
    return Motor((_e4.g0_ - (vec4<f32>(_e6.g0_.x, _e9.g0_.y, _e12.g0_.x, _e15.g0_.x) * vec4<f32>(1.0, 1.0, 0.0, 0.0))));
}

fn motor_rotor_geometric_product(self_978: Motor, other_830: Rotor) -> Motor {
    var self_979: Motor;
    var other_831: Rotor;

    self_979 = self_978;
    other_831 = other_830;
    let _e4: Motor = self_979;
    let _e8: Rotor = other_831;
    let _e11: Rotor = other_831;
    let _e14: Rotor = other_831;
    let _e17: Rotor = other_831;
    let _e29: Motor = self_979;
    let _e33: Rotor = other_831;
    let _e36: Rotor = other_831;
    let _e39: Rotor = other_831;
    let _e42: Rotor = other_831;
    let _e54: Motor = self_979;
    let _e57: Rotor = other_831;
    let _e60: Rotor = other_831;
    let _e63: Rotor = other_831;
    let _e66: Rotor = other_831;
    return Motor(((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y, _e11.g0_.x, _e14.g0_.y, _e17.g0_.y)) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0)) + ((vec4<f32>(_e29.g0_.w) * vec4<f32>(_e33.g0_.y, _e36.g0_.y, _e39.g0_.y, _e42.g0_.x)) * vec4<f32>(0.0, 0.0, 1.0, 1.0))) + ((_e54.g0_.xxzz * vec4<f32>(_e57.g0_.x, _e60.g0_.y, _e63.g0_.x, _e66.g0_.y)) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))));
}

fn motor_rotor_outer_product(self_980: Motor, other_832: Rotor) -> Motor {
    var self_981: Motor;
    var other_833: Rotor;

    self_981 = self_980;
    other_833 = other_832;
    let _e4: Motor = self_981;
    let _e8: Rotor = other_833;
    let _e19: Motor = self_981;
    let _e22: Rotor = other_833;
    let _e25: Rotor = other_833;
    let _e28: Rotor = other_833;
    let _e31: Rotor = other_833;
    return Motor((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.x)) * vec4<f32>(0.0, 1.0, 0.0, 0.0)) + (_e19.g0_.xxzw * vec4<f32>(_e22.g0_.x, _e25.g0_.y, _e28.g0_.x, _e31.g0_.x))));
}

fn motor_rotor_inner_product(self_982: Motor, other_834: Rotor) -> Motor {
    var self_983: Motor;
    var other_835: Rotor;

    self_983 = self_982;
    other_835 = other_834;
    let _e4: Motor = self_983;
    let _e8: Rotor = other_835;
    let _e11: Rotor = other_835;
    let _e14: Rotor = other_835;
    let _e17: Rotor = other_835;
    let _e29: Motor = self_983;
    let _e32: Rotor = other_835;
    let _e35: Rotor = other_835;
    let _e38: Rotor = other_835;
    let _e41: Rotor = other_835;
    return Motor((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y, _e11.g0_.x, _e14.g0_.y, _e17.g0_.y)) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0)) + (_e29.g0_.xxzw * vec4<f32>(_e32.g0_.x, _e35.g0_.y, _e38.g0_.x, _e41.g0_.x))));
}

fn motor_rotor_left_contraction(self_984: Motor, other_836: Rotor) -> Rotor {
    var self_985: Motor;
    var other_837: Rotor;

    self_985 = self_984;
    other_837 = other_836;
    let _e4: Motor = self_985;
    let _e8: Rotor = other_837;
    let _e11: Motor = self_985;
    let _e14: Motor = self_985;
    let _e18: Rotor = other_837;
    return Rotor(((vec2<f32>(_e4.g0_.x) * _e8.g0_) + ((vec2<f32>(_e11.g0_.y, _e14.g0_.x) * _e18.g0_.yx) * vec2<f32>(-(1.0), 0.0))));
}

fn motor_rotor_right_contraction(self_986: Motor, other_838: Rotor) -> Motor {
    var self_987: Motor;
    var other_839: Rotor;

    self_987 = self_986;
    other_839 = other_838;
    let _e4: Motor = self_987;
    let _e8: Rotor = other_839;
    let _e11: Rotor = other_839;
    let _e14: Rotor = other_839;
    let _e17: Rotor = other_839;
    let _e29: Motor = self_987;
    let _e32: Rotor = other_839;
    return Motor((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y, _e11.g0_.x, _e14.g0_.y, _e17.g0_.y)) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0)) + ((_e29.g0_.xxzw * vec4<f32>(_e32.g0_.x)) * vec4<f32>(1.0, 0.0, 1.0, 1.0))));
}

fn motor_rotor_scalar_product(self_988: Motor, other_840: Rotor) -> Scalar {
    var self_989: Motor;
    var other_841: Rotor;

    self_989 = self_988;
    other_841 = other_840;
    let _e4: Motor = self_989;
    let _e7: Rotor = other_841;
    let _e11: Motor = self_989;
    let _e14: Rotor = other_841;
    return Scalar(((_e4.g0_.x * _e7.g0_.x) - (_e11.g0_.y * _e14.g0_.y)));
}

fn motor_point_into(self_990: Motor) -> Point {
    var self_991: Motor;

    self_991 = self_990;
    let _e2: Motor = self_991;
    let _e5: Motor = self_991;
    let _e8: Motor = self_991;
    return Point(vec3<f32>(_e2.g0_.y, _e5.g0_.z, _e8.g0_.w));
}

fn motor_point_add(self_992: Motor, other_842: Point) -> Motor {
    var self_993: Motor;
    var other_843: Point;

    self_993 = self_992;
    other_843 = other_842;
    let _e4: Motor = self_993;
    let _e6: Point = other_843;
    let _e9: Point = other_843;
    let _e12: Point = other_843;
    let _e15: Point = other_843;
    return Motor((_e4.g0_ + (vec4<f32>(_e6.g0_.x, _e9.g0_.x, _e12.g0_.y, _e15.g0_.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn motor_point_sub(self_994: Motor, other_844: Point) -> Motor {
    var self_995: Motor;
    var other_845: Point;

    self_995 = self_994;
    other_845 = other_844;
    let _e4: Motor = self_995;
    let _e6: Point = other_845;
    let _e9: Point = other_845;
    let _e12: Point = other_845;
    let _e15: Point = other_845;
    return Motor((_e4.g0_ - (vec4<f32>(_e6.g0_.x, _e9.g0_.x, _e12.g0_.y, _e15.g0_.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn motor_point_geometric_product(self_996: Motor, other_846: Point) -> Motor {
    var self_997: Motor;
    var other_847: Point;

    self_997 = self_996;
    other_847 = other_846;
    let _e4: Motor = self_997;
    let _e8: Point = other_847;
    let _e11: Point = other_847;
    let _e14: Point = other_847;
    let _e17: Point = other_847;
    let _e30: Motor = self_997;
    let _e34: Point = other_847;
    let _e47: Motor = self_997;
    let _e51: Point = other_847;
    let _e63: Motor = self_997;
    let _e67: Point = other_847;
    let _e70: Point = other_847;
    let _e73: Point = other_847;
    let _e76: Point = other_847;
    return Motor((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.x, _e11.g0_.x, _e14.g0_.z, _e17.g0_.y)) * vec4<f32>(-(1.0), 0.0, -(1.0), 1.0)) + ((vec4<f32>(_e30.g0_.z) * vec4<f32>(_e34.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e47.g0_.w) * vec4<f32>(_e51.g0_.x)) * vec4<f32>(0.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e63.g0_.x) * vec4<f32>(_e67.g0_.x, _e70.g0_.x, _e73.g0_.y, _e76.g0_.z)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn motor_point_regressive_product(self_998: Motor, other_848: Point) -> Plane {
    var self_999: Motor;
    var other_849: Point;

    self_999 = self_998;
    other_849 = other_848;
    let _e4: Motor = self_999;
    let _e8: Point = other_849;
    let _e18: Motor = self_999;
    let _e22: Point = other_849;
    let _e33: Motor = self_999;
    let _e36: Motor = self_999;
    let _e39: Motor = self_999;
    let _e43: Point = other_849;
    return Plane(((((vec3<f32>(_e4.g0_.z) * _e8.g0_.zzx) * vec3<f32>(-(1.0), 0.0, 1.0)) + ((vec3<f32>(_e18.g0_.w) * _e22.g0_.yxy) * vec3<f32>(1.0, -(1.0), 0.0))) + ((vec3<f32>(_e33.g0_.x, _e36.g0_.y, _e39.g0_.y) * _e43.g0_.xzy) * vec3<f32>(0.0, 1.0, -(1.0)))));
}

fn motor_point_outer_product(self_1000: Motor, other_850: Point) -> Point {
    var self_1001: Motor;
    var other_851: Point;

    self_1001 = self_1000;
    other_851 = other_850;
    let _e4: Motor = self_1001;
    let _e8: Point = other_851;
    return Point((vec3<f32>(_e4.g0_.x) * _e8.g0_));
}

fn motor_point_inner_product(self_1002: Motor, other_852: Point) -> Motor {
    var self_1003: Motor;
    var other_853: Point;

    self_1003 = self_1002;
    other_853 = other_852;
    let _e4: Motor = self_1003;
    let _e7: Point = other_853;
    let _e10: Point = other_853;
    let _e13: Point = other_853;
    let _e16: Point = other_853;
    return Motor(((_e4.g0_.yxxx * vec4<f32>(_e7.g0_.x, _e10.g0_.x, _e13.g0_.y, _e16.g0_.z)) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0)));
}

fn motor_point_left_contraction(self_1004: Motor, other_854: Point) -> Motor {
    var self_1005: Motor;
    var other_855: Point;

    self_1005 = self_1004;
    other_855 = other_854;
    let _e4: Motor = self_1005;
    let _e7: Point = other_855;
    let _e10: Point = other_855;
    let _e13: Point = other_855;
    let _e16: Point = other_855;
    return Motor(((_e4.g0_.yxxx * vec4<f32>(_e7.g0_.x, _e10.g0_.x, _e13.g0_.y, _e16.g0_.z)) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0)));
}

fn motor_point_right_contraction(self_1006: Motor, other_856: Point) -> Scalar {
    var self_1007: Motor;
    var other_857: Point;

    self_1007 = self_1006;
    other_857 = other_856;
    let _e5: Motor = self_1007;
    let _e8: Point = other_857;
    return Scalar((0.0 - (_e5.g0_.y * _e8.g0_.x)));
}

fn motor_point_scalar_product(self_1008: Motor, other_858: Point) -> Scalar {
    var self_1009: Motor;
    var other_859: Point;

    self_1009 = self_1008;
    other_859 = other_858;
    let _e5: Motor = self_1009;
    let _e8: Point = other_859;
    return Scalar((0.0 - (_e5.g0_.y * _e8.g0_.x)));
}

fn motor_ideal_point_into(self_1010: Motor) -> IdealPoint {
    var self_1011: Motor;

    self_1011 = self_1010;
    let _e2: Motor = self_1011;
    let _e5: Motor = self_1011;
    return IdealPoint(vec2<f32>(_e2.g0_.z, _e5.g0_.w));
}

fn motor_ideal_point_add(self_1012: Motor, other_860: IdealPoint) -> Motor {
    var self_1013: Motor;
    var other_861: IdealPoint;

    self_1013 = self_1012;
    other_861 = other_860;
    let _e4: Motor = self_1013;
    let _e6: IdealPoint = other_861;
    let _e9: IdealPoint = other_861;
    let _e12: IdealPoint = other_861;
    let _e15: IdealPoint = other_861;
    return Motor((_e4.g0_ + (vec4<f32>(_e6.g0_.x, _e9.g0_.x, _e12.g0_.x, _e15.g0_.y) * vec4<f32>(0.0, 0.0, 1.0, 1.0))));
}

fn motor_ideal_point_sub(self_1014: Motor, other_862: IdealPoint) -> Motor {
    var self_1015: Motor;
    var other_863: IdealPoint;

    self_1015 = self_1014;
    other_863 = other_862;
    let _e4: Motor = self_1015;
    let _e6: IdealPoint = other_863;
    let _e9: IdealPoint = other_863;
    let _e12: IdealPoint = other_863;
    let _e15: IdealPoint = other_863;
    return Motor((_e4.g0_ - (vec4<f32>(_e6.g0_.x, _e9.g0_.x, _e12.g0_.x, _e15.g0_.y) * vec4<f32>(0.0, 0.0, 1.0, 1.0))));
}

fn motor_ideal_point_geometric_product(self_1016: Motor, other_864: IdealPoint) -> IdealPoint {
    var self_1017: Motor;
    var other_865: IdealPoint;

    self_1017 = self_1016;
    other_865 = other_864;
    let _e4: Motor = self_1017;
    let _e8: IdealPoint = other_865;
    let _e11: Motor = self_1017;
    let _e15: IdealPoint = other_865;
    return IdealPoint(((vec2<f32>(_e4.g0_.x) * _e8.g0_) + ((vec2<f32>(_e11.g0_.y) * _e15.g0_.yx) * vec2<f32>(-(1.0), 1.0))));
}

fn motor_ideal_point_regressive_product(self_1018: Motor, other_866: IdealPoint) -> Plane {
    var self_1019: Motor;
    var other_867: IdealPoint;

    self_1019 = self_1018;
    other_867 = other_866;
    let _e4: Motor = self_1019;
    let _e8: IdealPoint = other_867;
    let _e18: Motor = self_1019;
    let _e21: Motor = self_1019;
    let _e24: Motor = self_1019;
    let _e28: IdealPoint = other_867;
    let _e31: IdealPoint = other_867;
    let _e34: IdealPoint = other_867;
    return Plane((((vec3<f32>(_e4.g0_.w) * vec3<f32>(_e8.g0_.x)) * vec3<f32>(1.0, 0.0, 0.0)) + ((vec3<f32>(_e18.g0_.z, _e21.g0_.y, _e24.g0_.y) * vec3<f32>(_e28.g0_.y, _e31.g0_.y, _e34.g0_.x)) * vec3<f32>(-(1.0), 1.0, -(1.0)))));
}

fn motor_ideal_point_outer_product(self_1020: Motor, other_868: IdealPoint) -> IdealPoint {
    var self_1021: Motor;
    var other_869: IdealPoint;

    self_1021 = self_1020;
    other_869 = other_868;
    let _e4: Motor = self_1021;
    let _e8: IdealPoint = other_869;
    return IdealPoint((vec2<f32>(_e4.g0_.x) * _e8.g0_));
}

fn motor_ideal_point_inner_product(self_1022: Motor, other_870: IdealPoint) -> IdealPoint {
    var self_1023: Motor;
    var other_871: IdealPoint;

    self_1023 = self_1022;
    other_871 = other_870;
    let _e4: Motor = self_1023;
    let _e8: IdealPoint = other_871;
    return IdealPoint((vec2<f32>(_e4.g0_.x) * _e8.g0_));
}

fn motor_ideal_point_left_contraction(self_1024: Motor, other_872: IdealPoint) -> IdealPoint {
    var self_1025: Motor;
    var other_873: IdealPoint;

    self_1025 = self_1024;
    other_873 = other_872;
    let _e4: Motor = self_1025;
    let _e8: IdealPoint = other_873;
    return IdealPoint((vec2<f32>(_e4.g0_.x) * _e8.g0_));
}

fn motor_plane_geometric_product(self_1026: Motor, other_874: Plane) -> MotorDual {
    var self_1027: Motor;
    var other_875: Plane;

    self_1027 = self_1026;
    other_875 = other_874;
    let _e4: Motor = self_1027;
    let _e8: Plane = other_875;
    let _e11: Plane = other_875;
    let _e14: Plane = other_875;
    let _e17: Plane = other_875;
    let _e29: Motor = self_1027;
    let _e33: Plane = other_875;
    let _e36: Plane = other_875;
    let _e39: Plane = other_875;
    let _e42: Plane = other_875;
    let _e54: Motor = self_1027;
    let _e58: Plane = other_875;
    let _e61: Plane = other_875;
    let _e64: Plane = other_875;
    let _e67: Plane = other_875;
    let _e80: Motor = self_1027;
    let _e84: Plane = other_875;
    let _e87: Plane = other_875;
    let _e90: Plane = other_875;
    let _e93: Plane = other_875;
    return MotorDual((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.x, _e11.g0_.x, _e14.g0_.z, _e17.g0_.y)) * vec4<f32>(1.0, 0.0, -(1.0), 1.0)) + ((vec4<f32>(_e29.g0_.z) * vec4<f32>(_e33.g0_.y, _e36.g0_.z, _e39.g0_.y, _e42.g0_.y)) * vec4<f32>(1.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e54.g0_.w) * vec4<f32>(_e58.g0_.z, _e61.g0_.y, _e64.g0_.z, _e67.g0_.z)) * vec4<f32>(1.0, -(1.0), 0.0, 0.0))) + ((vec4<f32>(_e80.g0_.x) * vec4<f32>(_e84.g0_.x, _e87.g0_.x, _e90.g0_.y, _e93.g0_.z)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn motor_plane_regressive_product(self_1028: Motor, other_876: Plane) -> Scalar {
    var self_1029: Motor;
    var other_877: Plane;

    self_1029 = self_1028;
    other_877 = other_876;
    let _e4: Motor = self_1029;
    let _e7: Plane = other_877;
    let _e11: Motor = self_1029;
    let _e14: Plane = other_877;
    let _e19: Motor = self_1029;
    let _e22: Plane = other_877;
    return Scalar((((_e4.g0_.y * _e7.g0_.x) + (_e11.g0_.z * _e14.g0_.y)) + (_e19.g0_.w * _e22.g0_.z)));
}

fn motor_plane_outer_product(self_1030: Motor, other_878: Plane) -> MotorDual {
    var self_1031: Motor;
    var other_879: Plane;

    self_1031 = self_1030;
    other_879 = other_878;
    let _e4: Motor = self_1031;
    let _e8: Plane = other_879;
    let _e19: Motor = self_1031;
    let _e23: Plane = other_879;
    let _e35: Motor = self_1031;
    let _e38: Plane = other_879;
    let _e41: Plane = other_879;
    let _e44: Plane = other_879;
    let _e47: Plane = other_879;
    return MotorDual(((((vec4<f32>(_e4.g0_.z) * vec4<f32>(_e8.g0_.y)) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) + ((vec4<f32>(_e19.g0_.w) * vec4<f32>(_e23.g0_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + (_e35.g0_.yxxx * vec4<f32>(_e38.g0_.x, _e41.g0_.x, _e44.g0_.y, _e47.g0_.z))));
}

fn motor_plane_inner_product(self_1032: Motor, other_880: Plane) -> Plane {
    var self_1033: Motor;
    var other_881: Plane;

    self_1033 = self_1032;
    other_881 = other_880;
    let _e4: Motor = self_1033;
    let _e8: Plane = other_881;
    let _e11: Motor = self_1033;
    let _e15: Plane = other_881;
    let _e27: Motor = self_1033;
    let _e30: Motor = self_1033;
    let _e33: Motor = self_1033;
    let _e37: Plane = other_881;
    return Plane((((vec3<f32>(_e4.g0_.x) * _e8.g0_) + ((vec3<f32>(_e11.g0_.w) * vec3<f32>(_e15.g0_.y)) * vec3<f32>(-(1.0), 0.0, 0.0))) + ((vec3<f32>(_e27.g0_.z, _e30.g0_.y, _e33.g0_.y) * _e37.g0_.zzy) * vec3<f32>(1.0, -(1.0), 1.0))));
}

fn motor_plane_left_contraction(self_1034: Motor, other_882: Plane) -> Plane {
    var self_1035: Motor;
    var other_883: Plane;

    self_1035 = self_1034;
    other_883 = other_882;
    let _e4: Motor = self_1035;
    let _e8: Plane = other_883;
    return Plane((vec3<f32>(_e4.g0_.x) * _e8.g0_));
}

fn motor_plane_right_contraction(self_1036: Motor, other_884: Plane) -> Plane {
    var self_1037: Motor;
    var other_885: Plane;

    self_1037 = self_1036;
    other_885 = other_884;
    let _e4: Motor = self_1037;
    let _e8: Plane = other_885;
    let _e19: Motor = self_1037;
    let _e22: Motor = self_1037;
    let _e25: Motor = self_1037;
    let _e29: Plane = other_885;
    return Plane((((vec3<f32>(_e4.g0_.w) * vec3<f32>(_e8.g0_.y)) * vec3<f32>(-(1.0), 0.0, 0.0)) + ((vec3<f32>(_e19.g0_.z, _e22.g0_.y, _e25.g0_.y) * _e29.g0_.zzy) * vec3<f32>(1.0, -(1.0), 1.0))));
}

fn motor_translator_into(self_1038: Motor) -> Translator {
    var self_1039: Motor;

    self_1039 = self_1038;
    let _e2: Motor = self_1039;
    let _e5: Motor = self_1039;
    let _e8: Motor = self_1039;
    return Translator(vec3<f32>(_e2.g0_.x, _e5.g0_.z, _e8.g0_.w));
}

fn motor_translator_add(self_1040: Motor, other_886: Translator) -> Motor {
    var self_1041: Motor;
    var other_887: Translator;

    self_1041 = self_1040;
    other_887 = other_886;
    let _e4: Motor = self_1041;
    let _e6: Translator = other_887;
    let _e9: Translator = other_887;
    let _e12: Translator = other_887;
    let _e15: Translator = other_887;
    return Motor((_e4.g0_ + (vec4<f32>(_e6.g0_.x, _e9.g0_.x, _e12.g0_.y, _e15.g0_.z) * vec4<f32>(1.0, 0.0, 1.0, 1.0))));
}

fn motor_translator_sub(self_1042: Motor, other_888: Translator) -> Motor {
    var self_1043: Motor;
    var other_889: Translator;

    self_1043 = self_1042;
    other_889 = other_888;
    let _e4: Motor = self_1043;
    let _e6: Translator = other_889;
    let _e9: Translator = other_889;
    let _e12: Translator = other_889;
    let _e15: Translator = other_889;
    return Motor((_e4.g0_ - (vec4<f32>(_e6.g0_.x, _e9.g0_.x, _e12.g0_.y, _e15.g0_.z) * vec4<f32>(1.0, 0.0, 1.0, 1.0))));
}

fn motor_translator_geometric_product(self_1044: Motor, other_890: Translator) -> Motor {
    var self_1045: Motor;
    var other_891: Translator;

    self_1045 = self_1044;
    other_891 = other_890;
    let _e4: Motor = self_1045;
    let _e8: Translator = other_891;
    let _e11: Translator = other_891;
    let _e14: Translator = other_891;
    let _e17: Translator = other_891;
    let _e29: Motor = self_1045;
    let _e33: Translator = other_891;
    let _e45: Motor = self_1045;
    let _e49: Translator = other_891;
    let _e61: Motor = self_1045;
    let _e65: Translator = other_891;
    let _e68: Translator = other_891;
    let _e71: Translator = other_891;
    let _e74: Translator = other_891;
    return Motor((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.x, _e11.g0_.x, _e14.g0_.z, _e17.g0_.y)) * vec4<f32>(0.0, 1.0, -(1.0), 1.0)) + ((vec4<f32>(_e29.g0_.z) * vec4<f32>(_e33.g0_.x)) * vec4<f32>(0.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e45.g0_.w) * vec4<f32>(_e49.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e61.g0_.x) * vec4<f32>(_e65.g0_.x, _e68.g0_.x, _e71.g0_.y, _e74.g0_.z)) * vec4<f32>(1.0, 0.0, 1.0, 1.0))));
}

fn motor_translator_regressive_product(self_1046: Motor, other_892: Translator) -> Plane {
    var self_1047: Motor;
    var other_893: Translator;

    self_1047 = self_1046;
    other_893 = other_892;
    let _e4: Motor = self_1047;
    let _e8: Translator = other_893;
    let _e18: Motor = self_1047;
    let _e21: Motor = self_1047;
    let _e24: Motor = self_1047;
    let _e28: Translator = other_893;
    return Plane((((vec3<f32>(_e4.g0_.w) * vec3<f32>(_e8.g0_.y)) * vec3<f32>(1.0, 0.0, 0.0)) + ((vec3<f32>(_e18.g0_.z, _e21.g0_.y, _e24.g0_.y) * _e28.g0_.zzy) * vec3<f32>(-(1.0), 1.0, -(1.0)))));
}

fn motor_translator_outer_product(self_1048: Motor, other_894: Translator) -> Motor {
    var self_1049: Motor;
    var other_895: Translator;

    self_1049 = self_1048;
    other_895 = other_894;
    let _e4: Motor = self_1049;
    let _e8: Translator = other_895;
    let _e19: Motor = self_1049;
    let _e23: Translator = other_895;
    let _e35: Motor = self_1049;
    let _e38: Translator = other_895;
    let _e41: Translator = other_895;
    let _e44: Translator = other_895;
    let _e47: Translator = other_895;
    return Motor(((((vec4<f32>(_e4.g0_.z) * vec4<f32>(_e8.g0_.x)) * vec4<f32>(0.0, 0.0, 1.0, 0.0)) + ((vec4<f32>(_e19.g0_.w) * vec4<f32>(_e23.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + (_e35.g0_.xyxx * vec4<f32>(_e38.g0_.x, _e41.g0_.x, _e44.g0_.y, _e47.g0_.z))));
}

fn motor_translator_inner_product(self_1050: Motor, other_896: Translator) -> Motor {
    var self_1051: Motor;
    var other_897: Translator;

    self_1051 = self_1050;
    other_897 = other_896;
    let _e4: Motor = self_1051;
    let _e8: Translator = other_897;
    let _e19: Motor = self_1051;
    let _e23: Translator = other_897;
    let _e35: Motor = self_1051;
    let _e38: Translator = other_897;
    let _e41: Translator = other_897;
    let _e44: Translator = other_897;
    let _e47: Translator = other_897;
    return Motor(((((vec4<f32>(_e4.g0_.z) * vec4<f32>(_e8.g0_.x)) * vec4<f32>(0.0, 0.0, 1.0, 0.0)) + ((vec4<f32>(_e19.g0_.w) * vec4<f32>(_e23.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + (_e35.g0_.xyxx * vec4<f32>(_e38.g0_.x, _e41.g0_.x, _e44.g0_.y, _e47.g0_.z))));
}

fn motor_translator_left_contraction(self_1052: Motor, other_898: Translator) -> Translator {
    var self_1053: Motor;
    var other_899: Translator;

    self_1053 = self_1052;
    other_899 = other_898;
    let _e4: Motor = self_1053;
    let _e8: Translator = other_899;
    return Translator((vec3<f32>(_e4.g0_.x) * _e8.g0_));
}

fn motor_translator_right_contraction(self_1054: Motor, other_900: Translator) -> Motor {
    var self_1055: Motor;
    var other_901: Translator;

    self_1055 = self_1054;
    other_901 = other_900;
    let _e4: Motor = self_1055;
    let _e6: Translator = other_901;
    return Motor((_e4.g0_ * vec4<f32>(_e6.g0_.x)));
}

fn motor_translator_scalar_product(self_1056: Motor, other_902: Translator) -> Scalar {
    var self_1057: Motor;
    var other_903: Translator;

    self_1057 = self_1056;
    other_903 = other_902;
    let _e4: Motor = self_1057;
    let _e7: Translator = other_903;
    return Scalar((_e4.g0_.x * _e7.g0_.x));
}

fn motor_motor_add(self_1058: Motor, other_904: Motor) -> Motor {
    var self_1059: Motor;
    var other_905: Motor;

    self_1059 = self_1058;
    other_905 = other_904;
    let _e4: Motor = self_1059;
    let _e6: Motor = other_905;
    return Motor((_e4.g0_ + _e6.g0_));
}

fn motor_motor_sub(self_1060: Motor, other_906: Motor) -> Motor {
    var self_1061: Motor;
    var other_907: Motor;

    self_1061 = self_1060;
    other_907 = other_906;
    let _e4: Motor = self_1061;
    let _e6: Motor = other_907;
    return Motor((_e4.g0_ - _e6.g0_));
}

fn motor_motor_mul(self_1062: Motor, other_908: Motor) -> Motor {
    var self_1063: Motor;
    var other_909: Motor;

    self_1063 = self_1062;
    other_909 = other_908;
    let _e4: Motor = self_1063;
    let _e6: Motor = other_909;
    return Motor((_e4.g0_ * _e6.g0_));
}

fn motor_motor_div(self_1064: Motor, other_910: Motor) -> Motor {
    var self_1065: Motor;
    var other_911: Motor;

    self_1065 = self_1064;
    other_911 = other_910;
    let _e4: Motor = self_1065;
    let _e7: Motor = self_1065;
    let _e10: Motor = self_1065;
    let _e13: Motor = self_1065;
    let _e23: Motor = other_911;
    let _e26: Motor = other_911;
    let _e29: Motor = other_911;
    let _e32: Motor = other_911;
    return Motor((((vec4<f32>(_e4.g0_.x, _e7.g0_.y, _e10.g0_.z, _e13.g0_.w) * vec4<f32>(1.0, 1.0, 1.0, 1.0)) / vec4<f32>(_e23.g0_.x, _e26.g0_.y, _e29.g0_.z, _e32.g0_.w)) * vec4<f32>(1.0, 1.0, 1.0, 1.0)));
}

fn motor_motor_geometric_product(self_1066: Motor, other_912: Motor) -> Motor {
    var self_1067: Motor;
    var other_913: Motor;

    self_1067 = self_1066;
    other_913 = other_912;
    let _e4: Motor = self_1067;
    let _e8: Motor = other_913;
    let _e11: Motor = self_1067;
    let _e15: Motor = other_913;
    let _e28: Motor = self_1067;
    let _e32: Motor = other_913;
    let _e43: Motor = self_1067;
    let _e46: Motor = other_913;
    return Motor(((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.y) * _e15.g0_.yxwz) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))) + ((vec4<f32>(_e28.g0_.w) * _e32.g0_.yyyx) * vec4<f32>(0.0, 0.0, 1.0, 1.0))) + ((_e43.g0_.xxzz * _e46.g0_.xxxy) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))));
}

fn motor_motor_regressive_product(self_1068: Motor, other_914: Motor) -> Plane {
    var self_1069: Motor;
    var other_915: Motor;

    self_1069 = self_1068;
    other_915 = other_914;
    let _e4: Motor = self_1069;
    let _e8: Motor = other_915;
    let _e11: Motor = other_915;
    let _e14: Motor = other_915;
    let _e25: Motor = self_1069;
    let _e29: Motor = other_915;
    let _e32: Motor = other_915;
    let _e35: Motor = other_915;
    let _e47: Motor = self_1069;
    let _e50: Motor = self_1069;
    let _e53: Motor = self_1069;
    let _e57: Motor = other_915;
    let _e60: Motor = other_915;
    let _e63: Motor = other_915;
    return Plane(((((vec3<f32>(_e4.g0_.z) * vec3<f32>(_e8.g0_.w, _e11.g0_.w, _e14.g0_.y)) * vec3<f32>(-(1.0), 0.0, 1.0)) + ((vec3<f32>(_e25.g0_.w) * vec3<f32>(_e29.g0_.z, _e32.g0_.y, _e35.g0_.z)) * vec3<f32>(1.0, -(1.0), 0.0))) + ((vec3<f32>(_e47.g0_.x, _e50.g0_.y, _e53.g0_.y) * vec3<f32>(_e57.g0_.x, _e60.g0_.w, _e63.g0_.z)) * vec3<f32>(0.0, 1.0, -(1.0)))));
}

fn motor_motor_outer_product(self_1070: Motor, other_916: Motor) -> Motor {
    var self_1071: Motor;
    var other_917: Motor;

    self_1071 = self_1070;
    other_917 = other_916;
    let _e4: Motor = self_1071;
    let _e8: Motor = other_917;
    let _e11: Motor = self_1071;
    let _e13: Motor = other_917;
    return Motor(((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((_e11.g0_ * vec4<f32>(_e13.g0_.x)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn motor_motor_inner_product(self_1072: Motor, other_918: Motor) -> Motor {
    var self_1073: Motor;
    var other_919: Motor;

    self_1073 = self_1072;
    other_919 = other_918;
    let _e4: Motor = self_1073;
    let _e8: Motor = other_919;
    let _e11: Motor = self_1073;
    let _e14: Motor = other_919;
    return Motor(((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((_e11.g0_.yyzw * _e14.g0_.yxxx) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0))));
}

fn motor_motor_left_contraction(self_1074: Motor, other_920: Motor) -> Motor {
    var self_1075: Motor;
    var other_921: Motor;

    self_1075 = self_1074;
    other_921 = other_920;
    let _e4: Motor = self_1075;
    let _e8: Motor = other_921;
    let _e11: Motor = self_1075;
    let _e14: Motor = other_921;
    return Motor(((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((_e11.g0_.yxxx * _e14.g0_.yxxx) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))));
}

fn motor_motor_right_contraction(self_1076: Motor, other_922: Motor) -> Motor {
    var self_1077: Motor;
    var other_923: Motor;

    self_1077 = self_1076;
    other_923 = other_922;
    let _e4: Motor = self_1077;
    let _e8: Motor = other_923;
    let _e19: Motor = self_1077;
    let _e22: Motor = other_923;
    return Motor((((vec4<f32>(_e4.g0_.y) * _e8.g0_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0)) + ((_e19.g0_.xxzw * vec4<f32>(_e22.g0_.x)) * vec4<f32>(1.0, 0.0, 1.0, 1.0))));
}

fn motor_motor_scalar_product(self_1078: Motor, other_924: Motor) -> Scalar {
    var self_1079: Motor;
    var other_925: Motor;

    self_1079 = self_1078;
    other_925 = other_924;
    let _e4: Motor = self_1079;
    let _e7: Motor = other_925;
    let _e11: Motor = self_1079;
    let _e14: Motor = other_925;
    return Scalar(((_e4.g0_.x * _e7.g0_.x) - (_e11.g0_.y * _e14.g0_.y)));
}

fn motor_motor_dual_add(self_1080: Motor, other_926: MotorDual) -> MultiVector {
    var self_1081: Motor;
    var other_927: MotorDual;

    self_1081 = self_1080;
    other_927 = other_926;
    let _e4: Motor = self_1081;
    let _e13: MotorDual = other_927;
    let _e23: Motor = self_1081;
    let _e32: MotorDual = other_927;
    return MultiVector(((_e4.g0_.xyxx * vec4<f32>(1.0, 1.0, 0.0, 0.0)) + (_e13.g0_.xxwz * vec4<f32>(0.0, 0.0, 1.0, 1.0))), ((_e23.g0_.xxzw * vec4<f32>(0.0, 0.0, 1.0, 1.0)) + (_e32.g0_.yxxx * vec4<f32>(1.0, 1.0, 0.0, 0.0))));
}

fn motor_motor_dual_sub(self_1082: Motor, other_928: MotorDual) -> MultiVector {
    var self_1083: Motor;
    var other_929: MotorDual;

    self_1083 = self_1082;
    other_929 = other_928;
    let _e4: Motor = self_1083;
    let _e13: MotorDual = other_929;
    let _e23: Motor = self_1083;
    let _e32: MotorDual = other_929;
    return MultiVector(((_e4.g0_.xyxx * vec4<f32>(1.0, 1.0, 0.0, 0.0)) - (_e13.g0_.xxwz * vec4<f32>(0.0, 0.0, 1.0, 1.0))), ((_e23.g0_.xxzw * vec4<f32>(0.0, 0.0, 1.0, 1.0)) - (_e32.g0_.yxxx * vec4<f32>(1.0, 1.0, 0.0, 0.0))));
}

fn motor_motor_dual_geometric_product(self_1084: Motor, other_930: MotorDual) -> MotorDual {
    var self_1085: Motor;
    var other_931: MotorDual;

    self_1085 = self_1084;
    other_931 = other_930;
    let _e4: Motor = self_1085;
    let _e8: MotorDual = other_931;
    let _e11: Motor = self_1085;
    let _e15: MotorDual = other_931;
    let _e28: Motor = self_1085;
    let _e32: MotorDual = other_931;
    let _e44: Motor = self_1085;
    let _e47: MotorDual = other_931;
    return MotorDual(((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.y) * _e15.g0_.yxwz) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e28.g0_.w) * _e32.g0_.wzww) * vec4<f32>(1.0, -(1.0), 0.0, 0.0))) + ((_e44.g0_.zzxx * _e47.g0_.zwxx) * vec4<f32>(1.0, 1.0, 0.0, 0.0))));
}

fn motor_motor_dual_regressive_product(self_1086: Motor, other_932: MotorDual) -> Motor {
    var self_1087: Motor;
    var other_933: MotorDual;

    self_1087 = self_1086;
    other_933 = other_932;
    let _e4: Motor = self_1087;
    let _e8: MotorDual = other_933;
    let _e18: Motor = self_1087;
    let _e22: MotorDual = other_933;
    let _e33: Motor = self_1087;
    let _e37: MotorDual = other_933;
    let _e48: Motor = self_1087;
    let _e52: MotorDual = other_933;
    return Motor((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.yxyy) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) + ((vec4<f32>(_e18.g0_.z) * _e22.g0_.zzxz) * vec4<f32>(1.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e33.g0_.w) * _e37.g0_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e48.g0_.x) * vec4<f32>(_e52.g0_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))));
}

fn motor_motor_dual_outer_product(self_1088: Motor, other_934: MotorDual) -> MotorDual {
    var self_1089: Motor;
    var other_935: MotorDual;

    self_1089 = self_1088;
    other_935 = other_934;
    let _e4: Motor = self_1089;
    let _e8: MotorDual = other_935;
    let _e11: Motor = self_1089;
    let _e15: MotorDual = other_935;
    let _e27: Motor = self_1089;
    let _e31: MotorDual = other_935;
    let _e43: Motor = self_1089;
    let _e46: MotorDual = other_935;
    return MotorDual(((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.z) * vec4<f32>(_e15.g0_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e27.g0_.w) * vec4<f32>(_e31.g0_.w)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((_e43.g0_.yxxx * _e46.g0_.yxxx) * vec4<f32>(1.0, 0.0, 0.0, 0.0))));
}

fn motor_motor_dual_inner_product(self_1090: Motor, other_936: MotorDual) -> MotorDual {
    var self_1091: Motor;
    var other_937: MotorDual;

    self_1091 = self_1090;
    other_937 = other_936;
    let _e4: Motor = self_1091;
    let _e8: MotorDual = other_937;
    let _e11: Motor = self_1091;
    let _e15: MotorDual = other_937;
    let _e27: Motor = self_1091;
    let _e31: MotorDual = other_937;
    let _e44: Motor = self_1091;
    let _e47: MotorDual = other_937;
    return MotorDual(((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.z) * vec4<f32>(_e15.g0_.w)) * vec4<f32>(0.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e27.g0_.w) * vec4<f32>(_e31.g0_.z)) * vec4<f32>(0.0, -(1.0), 0.0, 0.0))) + ((_e44.g0_.xyyy * _e47.g0_.xxwz) * vec4<f32>(0.0, -(1.0), -(1.0), 1.0))));
}

fn motor_motor_dual_left_contraction(self_1092: Motor, other_938: MotorDual) -> MotorDual {
    var self_1093: Motor;
    var other_939: MotorDual;

    self_1093 = self_1092;
    other_939 = other_938;
    let _e4: Motor = self_1093;
    let _e8: MotorDual = other_939;
    let _e11: Motor = self_1093;
    let _e14: MotorDual = other_939;
    return MotorDual(((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((_e11.g0_.xyxx * vec4<f32>(_e14.g0_.x)) * vec4<f32>(0.0, -(1.0), 0.0, 0.0))));
}

fn motor_motor_dual_right_contraction(self_1094: Motor, other_940: MotorDual) -> Plane {
    var self_1095: Motor;
    var other_941: MotorDual;

    self_1095 = self_1094;
    other_941 = other_940;
    let _e4: Motor = self_1095;
    let _e8: MotorDual = other_941;
    let _e19: Motor = self_1095;
    let _e22: Motor = self_1095;
    let _e25: Motor = self_1095;
    let _e29: MotorDual = other_941;
    let _e32: MotorDual = other_941;
    let _e35: MotorDual = other_941;
    return Plane((((vec3<f32>(_e4.g0_.w) * vec3<f32>(_e8.g0_.z)) * vec3<f32>(-(1.0), 0.0, 0.0)) + ((vec3<f32>(_e19.g0_.z, _e22.g0_.y, _e25.g0_.y) * vec3<f32>(_e29.g0_.w, _e32.g0_.w, _e35.g0_.z)) * vec3<f32>(1.0, -(1.0), 1.0))));
}

fn motor_squared_magnitude(self_1096: Motor) -> Scalar {
    var self_1097: Motor;

    self_1097 = self_1096;
    let _e4: Motor = self_1097;
    let _e5: Motor = motor_reversal(_e4);
    let _e6: Motor = self_1097;
    let _e8: Motor = self_1097;
    let _e9: Motor = motor_reversal(_e8);
    let _e10: Scalar = motor_motor_scalar_product(_e6, _e9);
    return _e10;
}

fn motor_magnitude(self_1098: Motor) -> Scalar {
    var self_1099: Motor;

    self_1099 = self_1098;
    let _e3: Motor = self_1099;
    let _e4: Scalar = motor_squared_magnitude(_e3);
    let _e7: Motor = self_1099;
    let _e8: Scalar = motor_squared_magnitude(_e7);
    return Scalar(sqrt(_e8.g0_));
}

fn motor_scale(self_1100: Motor, other_942: f32) -> Motor {
    var self_1101: Motor;
    var other_943: f32;

    self_1101 = self_1100;
    other_943 = other_942;
    let _e5: f32 = other_943;
    let _e7: Motor = self_1101;
    let _e8: f32 = other_943;
    let _e10: Motor = motor_scalar_geometric_product(_e7, Scalar(_e8));
    return _e10;
}

fn motor_signum(self_1102: Motor) -> Motor {
    var self_1103: Motor;

    self_1103 = self_1102;
    let _e5: Motor = self_1103;
    let _e6: Scalar = motor_magnitude(_e5);
    let _e10: Motor = self_1103;
    let _e13: Motor = self_1103;
    let _e14: Scalar = motor_magnitude(_e13);
    let _e18: Motor = motor_scalar_geometric_product(_e10, Scalar((1.0 / _e14.g0_)));
    return _e18;
}

fn motor_inverse(self_1104: Motor) -> Motor {
    var self_1105: Motor;

    self_1105 = self_1104;
    let _e3: Motor = self_1105;
    let _e4: Motor = motor_reversal(_e3);
    let _e7: Motor = self_1105;
    let _e8: Scalar = motor_squared_magnitude(_e7);
    let _e13: Motor = self_1105;
    let _e14: Motor = motor_reversal(_e13);
    let _e17: Motor = self_1105;
    let _e18: Scalar = motor_squared_magnitude(_e17);
    let _e22: Motor = motor_scalar_geometric_product(_e14, Scalar((1.0 / _e18.g0_)));
    return _e22;
}

fn motor_dual_zero() -> MotorDual {
    return MotorDual(vec4<f32>(0.0));
}

fn motor_dual_one() -> MotorDual {
    return MotorDual(vec4<f32>(0.0));
}

fn motor_dual_neg(self_1106: MotorDual) -> MotorDual {
    var self_1107: MotorDual;

    self_1107 = self_1106;
    let _e2: MotorDual = self_1107;
    return MotorDual((_e2.g0_ * vec4<f32>(-(1.0))));
}

fn motor_dual_automorphism(self_1108: MotorDual) -> MotorDual {
    var self_1109: MotorDual;

    self_1109 = self_1108;
    let _e2: MotorDual = self_1109;
    return MotorDual((_e2.g0_ * vec4<f32>(-(1.0))));
}

fn motor_dual_reversal(self_1110: MotorDual) -> MotorDual {
    var self_1111: MotorDual;

    self_1111 = self_1110;
    let _e2: MotorDual = self_1111;
    return MotorDual((_e2.g0_ * vec4<f32>(-(1.0), 1.0, 1.0, 1.0)));
}

fn motor_dual_conjugation(self_1112: MotorDual) -> MotorDual {
    var self_1113: MotorDual;

    self_1113 = self_1112;
    let _e2: MotorDual = self_1113;
    return MotorDual((_e2.g0_ * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))));
}

fn motor_dual_dual(self_1114: MotorDual) -> Motor {
    var self_1115: MotorDual;

    self_1115 = self_1114;
    let _e2: MotorDual = self_1115;
    return Motor(_e2.g0_);
}

fn motor_dual_scalar_geometric_product(self_1116: MotorDual, other_944: Scalar) -> MotorDual {
    var self_1117: MotorDual;
    var other_945: Scalar;

    self_1117 = self_1116;
    other_945 = other_944;
    let _e4: MotorDual = self_1117;
    let _e6: Scalar = other_945;
    return MotorDual((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn motor_dual_scalar_regressive_product(self_1118: MotorDual, other_946: Scalar) -> Scalar {
    var self_1119: MotorDual;
    var other_947: Scalar;

    self_1119 = self_1118;
    other_947 = other_946;
    let _e4: MotorDual = self_1119;
    let _e7: Scalar = other_947;
    return Scalar((_e4.g0_.x * _e7.g0_));
}

fn motor_dual_scalar_outer_product(self_1120: MotorDual, other_948: Scalar) -> MotorDual {
    var self_1121: MotorDual;
    var other_949: Scalar;

    self_1121 = self_1120;
    other_949 = other_948;
    let _e4: MotorDual = self_1121;
    let _e6: Scalar = other_949;
    return MotorDual((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn motor_dual_scalar_inner_product(self_1122: MotorDual, other_950: Scalar) -> MotorDual {
    var self_1123: MotorDual;
    var other_951: Scalar;

    self_1123 = self_1122;
    other_951 = other_950;
    let _e4: MotorDual = self_1123;
    let _e6: Scalar = other_951;
    return MotorDual((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn motor_dual_scalar_right_contraction(self_1124: MotorDual, other_952: Scalar) -> MotorDual {
    var self_1125: MotorDual;
    var other_953: Scalar;

    self_1125 = self_1124;
    other_953 = other_952;
    let _e4: MotorDual = self_1125;
    let _e6: Scalar = other_953;
    return MotorDual((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn motor_dual_multi_vector_add(self_1126: MotorDual, other_954: MultiVector) -> MultiVector {
    var self_1127: MotorDual;
    var other_955: MultiVector;

    self_1127 = self_1126;
    other_955 = other_954;
    let _e4: MotorDual = self_1127;
    let _e13: MultiVector = other_955;
    let _e16: MotorDual = self_1127;
    let _e25: MultiVector = other_955;
    return MultiVector(((_e4.g0_.xxwz * vec4<f32>(0.0, 0.0, 1.0, 1.0)) + _e13.g0_), ((_e16.g0_.yxxx * vec4<f32>(1.0, 1.0, 0.0, 0.0)) + _e25.g1_));
}

fn motor_dual_multi_vector_sub(self_1128: MotorDual, other_956: MultiVector) -> MultiVector {
    var self_1129: MotorDual;
    var other_957: MultiVector;

    self_1129 = self_1128;
    other_957 = other_956;
    let _e4: MotorDual = self_1129;
    let _e13: MultiVector = other_957;
    let _e16: MotorDual = self_1129;
    let _e25: MultiVector = other_957;
    return MultiVector(((_e4.g0_.xxwz * vec4<f32>(0.0, 0.0, 1.0, 1.0)) - _e13.g0_), ((_e16.g0_.yxxx * vec4<f32>(1.0, 1.0, 0.0, 0.0)) - _e25.g1_));
}

fn motor_dual_multi_vector_geometric_product(self_1130: MotorDual, other_958: MultiVector) -> MultiVector {
    var self_1131: MotorDual;
    var other_959: MultiVector;

    self_1131 = self_1130;
    other_959 = other_958;
    let _e4: MotorDual = self_1131;
    let _e8: MultiVector = other_959;
    let _e20: MotorDual = self_1131;
    let _e24: MultiVector = other_959;
    let _e29: MotorDual = self_1131;
    let _e33: MultiVector = other_959;
    let _e44: MotorDual = self_1131;
    let _e48: MultiVector = other_959;
    let _e59: MotorDual = self_1131;
    let _e63: MultiVector = other_959;
    let _e68: MotorDual = self_1131;
    let _e72: MultiVector = other_959;
    return MultiVector((((vec4<f32>(_e4.g0_.z) * _e8.g0_.wzyx) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0)) + (vec4<f32>(_e20.g0_.w) * _e24.g0_.zwxy)), (((((vec4<f32>(_e29.g0_.x) * _e33.g0_.yxwz) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0)) + ((vec4<f32>(_e44.g0_.y) * _e48.g0_) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))) + (vec4<f32>(_e59.g0_.z) * _e63.g1_.wzyx)) + ((vec4<f32>(_e68.g0_.w) * _e72.g1_.zwxy) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))));
}

fn motor_dual_multi_vector_regressive_product(self_1132: MotorDual, other_960: MultiVector) -> MultiVector {
    var self_1133: MotorDual;
    var other_961: MultiVector;

    self_1133 = self_1132;
    other_961 = other_960;
    let _e4: MotorDual = self_1133;
    let _e8: MultiVector = other_961;
    let _e11: MotorDual = self_1133;
    let _e15: MultiVector = other_961;
    let _e26: MotorDual = self_1133;
    let _e30: MultiVector = other_961;
    let _e41: MotorDual = self_1133;
    let _e44: MultiVector = other_961;
    let _e55: MotorDual = self_1133;
    let _e59: MultiVector = other_961;
    let _e62: MotorDual = self_1133;
    let _e65: MultiVector = other_961;
    return MultiVector(((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.z) * _e15.g1_.zzzy) * vec4<f32>(1.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e26.g0_.w) * _e30.g1_.wwyw) * vec4<f32>(1.0, 0.0, 1.0, 0.0))) + ((_e41.g0_.yxxx * _e44.g0_.yxxx) * vec4<f32>(1.0, 0.0, 0.0, 0.0))), ((vec4<f32>(_e55.g0_.x) * _e59.g1_) + ((_e62.g0_.yxxx * _e65.g1_.yxxx) * vec4<f32>(1.0, 0.0, 0.0, 0.0))));
}

fn motor_dual_multi_vector_scalar_product(self_1134: MotorDual, other_962: MultiVector) -> Scalar {
    var self_1135: MotorDual;
    var other_963: MultiVector;

    self_1135 = self_1134;
    other_963 = other_962;
    let _e4: MotorDual = self_1135;
    let _e7: MultiVector = other_963;
    let _e11: MotorDual = self_1135;
    let _e14: MultiVector = other_963;
    return Scalar(((_e4.g0_.z * _e7.g0_.w) + (_e11.g0_.w * _e14.g0_.z)));
}

fn motor_dual_rotor_geometric_product(self_1136: MotorDual, other_964: Rotor) -> MotorDual {
    var self_1137: MotorDual;
    var other_965: Rotor;

    self_1137 = self_1136;
    other_965 = other_964;
    let _e4: MotorDual = self_1137;
    let _e8: Rotor = other_965;
    let _e11: Rotor = other_965;
    let _e14: Rotor = other_965;
    let _e17: Rotor = other_965;
    let _e28: MotorDual = self_1137;
    let _e32: Rotor = other_965;
    let _e35: Rotor = other_965;
    let _e38: Rotor = other_965;
    let _e41: Rotor = other_965;
    let _e53: MotorDual = self_1137;
    let _e56: Rotor = other_965;
    let _e59: Rotor = other_965;
    let _e62: Rotor = other_965;
    let _e65: Rotor = other_965;
    return MotorDual(((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y, _e11.g0_.x, _e14.g0_.y, _e17.g0_.y)) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) + ((vec4<f32>(_e28.g0_.w) * vec4<f32>(_e32.g0_.y, _e35.g0_.y, _e38.g0_.y, _e41.g0_.x)) * vec4<f32>(0.0, 0.0, 1.0, 1.0))) + ((_e53.g0_.xxzz * vec4<f32>(_e56.g0_.x, _e59.g0_.y, _e62.g0_.x, _e65.g0_.y)) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))));
}

fn motor_dual_rotor_regressive_product(self_1138: MotorDual, other_966: Rotor) -> Rotor {
    var self_1139: MotorDual;
    var other_967: Rotor;

    self_1139 = self_1138;
    other_967 = other_966;
    let _e4: MotorDual = self_1139;
    let _e8: Rotor = other_967;
    let _e11: MotorDual = self_1139;
    let _e14: MotorDual = self_1139;
    let _e18: Rotor = other_967;
    return Rotor(((vec2<f32>(_e4.g0_.x) * _e8.g0_) + ((vec2<f32>(_e11.g0_.y, _e14.g0_.x) * _e18.g0_.yx) * vec2<f32>(1.0, 0.0))));
}

fn motor_dual_rotor_outer_product(self_1140: MotorDual, other_968: Rotor) -> MotorDual {
    var self_1141: MotorDual;
    var other_969: Rotor;

    self_1141 = self_1140;
    other_969 = other_968;
    let _e4: MotorDual = self_1141;
    let _e8: Rotor = other_969;
    let _e11: Rotor = other_969;
    let _e14: Rotor = other_969;
    let _e17: Rotor = other_969;
    let _e28: MotorDual = self_1141;
    let _e31: Rotor = other_969;
    return MotorDual((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y, _e11.g0_.x, _e14.g0_.y, _e17.g0_.y)) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) + ((_e28.g0_.xxzw * vec4<f32>(_e31.g0_.x)) * vec4<f32>(1.0, 0.0, 1.0, 1.0))));
}

fn motor_dual_rotor_inner_product(self_1142: MotorDual, other_970: Rotor) -> MotorDual {
    var self_1143: MotorDual;
    var other_971: Rotor;

    self_1143 = self_1142;
    other_971 = other_970;
    let _e4: MotorDual = self_1143;
    let _e8: Rotor = other_971;
    let _e19: MotorDual = self_1143;
    let _e23: Rotor = other_971;
    let _e26: Rotor = other_971;
    let _e29: Rotor = other_971;
    let _e32: Rotor = other_971;
    let _e44: MotorDual = self_1143;
    let _e47: Rotor = other_971;
    let _e50: Rotor = other_971;
    let _e53: Rotor = other_971;
    let _e56: Rotor = other_971;
    return MotorDual(((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.x)) * vec4<f32>(0.0, 1.0, 0.0, 0.0)) + ((vec4<f32>(_e19.g0_.w) * vec4<f32>(_e23.g0_.y, _e26.g0_.y, _e29.g0_.y, _e32.g0_.x)) * vec4<f32>(0.0, 0.0, 1.0, 1.0))) + ((_e44.g0_.xxzz * vec4<f32>(_e47.g0_.x, _e50.g0_.y, _e53.g0_.x, _e56.g0_.y)) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))));
}

fn motor_dual_rotor_right_contraction(self_1144: MotorDual, other_972: Rotor) -> MotorDual {
    var self_1145: MotorDual;
    var other_973: Rotor;

    self_1145 = self_1144;
    other_973 = other_972;
    let _e4: MotorDual = self_1145;
    let _e8: Rotor = other_973;
    let _e19: MotorDual = self_1145;
    let _e22: Rotor = other_973;
    let _e25: Rotor = other_973;
    let _e28: Rotor = other_973;
    let _e31: Rotor = other_973;
    return MotorDual((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.x)) * vec4<f32>(0.0, 1.0, 0.0, 0.0)) + ((_e19.g0_.xxzw * vec4<f32>(_e22.g0_.x, _e25.g0_.y, _e28.g0_.x, _e31.g0_.x)) * vec4<f32>(1.0, -(1.0), 1.0, 1.0))));
}

fn motor_dual_point_geometric_product(self_1146: MotorDual, other_974: Point) -> MotorDual {
    var self_1147: MotorDual;
    var other_975: Point;

    self_1147 = self_1146;
    other_975 = other_974;
    let _e4: MotorDual = self_1147;
    let _e8: Point = other_975;
    let _e11: Point = other_975;
    let _e14: Point = other_975;
    let _e17: Point = other_975;
    let _e29: MotorDual = self_1147;
    let _e33: Point = other_975;
    let _e36: Point = other_975;
    let _e39: Point = other_975;
    let _e42: Point = other_975;
    let _e55: MotorDual = self_1147;
    let _e58: Point = other_975;
    return MotorDual(((((vec4<f32>(_e4.g0_.z) * vec4<f32>(_e8.g0_.y, _e11.g0_.z, _e14.g0_.y, _e17.g0_.x)) * vec4<f32>(1.0, 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e29.g0_.w) * vec4<f32>(_e33.g0_.z, _e36.g0_.y, _e39.g0_.x, _e42.g0_.z)) * vec4<f32>(1.0, -(1.0), 1.0, 0.0))) + ((_e55.g0_.yxxx * vec4<f32>(_e58.g0_.x)) * vec4<f32>(1.0, -(1.0), 0.0, 0.0))));
}

fn motor_dual_point_regressive_product(self_1148: MotorDual, other_976: Point) -> Motor {
    var self_1149: MotorDual;
    var other_977: Point;

    self_1149 = self_1148;
    other_977 = other_976;
    let _e4: MotorDual = self_1149;
    let _e8: Point = other_977;
    let _e19: MotorDual = self_1149;
    let _e23: Point = other_977;
    let _e35: MotorDual = self_1149;
    let _e38: Point = other_977;
    let _e41: Point = other_977;
    let _e44: Point = other_977;
    let _e47: Point = other_977;
    return Motor(((((vec4<f32>(_e4.g0_.z) * vec4<f32>(_e8.g0_.y)) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) + ((vec4<f32>(_e19.g0_.w) * vec4<f32>(_e23.g0_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + (_e35.g0_.yxxx * vec4<f32>(_e38.g0_.x, _e41.g0_.x, _e44.g0_.y, _e47.g0_.z))));
}

fn motor_dual_point_inner_product(self_1150: MotorDual, other_978: Point) -> Plane {
    var self_1151: MotorDual;
    var other_979: Point;

    self_1151 = self_1150;
    other_979 = other_978;
    let _e4: MotorDual = self_1151;
    let _e8: Point = other_979;
    let _e18: MotorDual = self_1151;
    let _e22: Point = other_979;
    let _e33: MotorDual = self_1151;
    let _e37: Point = other_979;
    return Plane(((((vec3<f32>(_e4.g0_.z) * _e8.g0_.zzx) * vec3<f32>(1.0, 0.0, -(1.0))) + ((vec3<f32>(_e18.g0_.w) * _e22.g0_.yxy) * vec3<f32>(-(1.0), 1.0, 0.0))) + ((vec3<f32>(_e33.g0_.x) * vec3<f32>(_e37.g0_.x)) * vec3<f32>(-(1.0), 0.0, 0.0))));
}

fn motor_dual_point_left_contraction(self_1152: MotorDual, other_980: Point) -> Plane {
    var self_1153: MotorDual;
    var other_981: Point;

    self_1153 = self_1152;
    other_981 = other_980;
    let _e4: MotorDual = self_1153;
    let _e8: Point = other_981;
    let _e18: MotorDual = self_1153;
    let _e21: MotorDual = self_1153;
    let _e24: MotorDual = self_1153;
    let _e28: Point = other_981;
    return Plane((((vec3<f32>(_e4.g0_.w) * _e8.g0_.yxy) * vec3<f32>(-(1.0), 1.0, 0.0)) + ((vec3<f32>(_e18.g0_.z, _e21.g0_.x, _e24.g0_.z) * _e28.g0_.zxx) * vec3<f32>(1.0, 0.0, -(1.0)))));
}

fn motor_dual_ideal_point_regressive_product(self_1154: MotorDual, other_982: IdealPoint) -> Translator {
    var self_1155: MotorDual;
    var other_983: IdealPoint;

    self_1155 = self_1154;
    other_983 = other_982;
    let _e4: MotorDual = self_1155;
    let _e8: IdealPoint = other_983;
    let _e18: MotorDual = self_1155;
    let _e21: MotorDual = self_1155;
    let _e24: MotorDual = self_1155;
    let _e28: IdealPoint = other_983;
    let _e31: IdealPoint = other_983;
    let _e34: IdealPoint = other_983;
    return Translator((((vec3<f32>(_e4.g0_.w) * vec3<f32>(_e8.g0_.y)) * vec3<f32>(1.0, 0.0, 0.0)) + (vec3<f32>(_e18.g0_.z, _e21.g0_.x, _e24.g0_.x) * vec3<f32>(_e28.g0_.x, _e31.g0_.x, _e34.g0_.y))));
}

fn motor_dual_plane_into(self_1156: MotorDual) -> Plane {
    var self_1157: MotorDual;

    self_1157 = self_1156;
    let _e2: MotorDual = self_1157;
    let _e5: MotorDual = self_1157;
    let _e8: MotorDual = self_1157;
    return Plane(vec3<f32>(_e2.g0_.y, _e5.g0_.z, _e8.g0_.w));
}

fn motor_dual_plane_add(self_1158: MotorDual, other_984: Plane) -> MotorDual {
    var self_1159: MotorDual;
    var other_985: Plane;

    self_1159 = self_1158;
    other_985 = other_984;
    let _e4: MotorDual = self_1159;
    let _e6: Plane = other_985;
    let _e9: Plane = other_985;
    let _e12: Plane = other_985;
    let _e15: Plane = other_985;
    return MotorDual((_e4.g0_ + (vec4<f32>(_e6.g0_.x, _e9.g0_.x, _e12.g0_.y, _e15.g0_.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn motor_dual_plane_sub(self_1160: MotorDual, other_986: Plane) -> MotorDual {
    var self_1161: MotorDual;
    var other_987: Plane;

    self_1161 = self_1160;
    other_987 = other_986;
    let _e4: MotorDual = self_1161;
    let _e6: Plane = other_987;
    let _e9: Plane = other_987;
    let _e12: Plane = other_987;
    let _e15: Plane = other_987;
    return MotorDual((_e4.g0_ - (vec4<f32>(_e6.g0_.x, _e9.g0_.x, _e12.g0_.y, _e15.g0_.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn motor_dual_plane_geometric_product(self_1162: MotorDual, other_988: Plane) -> Motor {
    var self_1163: MotorDual;
    var other_989: Plane;

    self_1163 = self_1162;
    other_989 = other_988;
    let _e4: MotorDual = self_1163;
    let _e8: Plane = other_989;
    let _e11: Plane = other_989;
    let _e14: Plane = other_989;
    let _e17: Plane = other_989;
    let _e29: MotorDual = self_1163;
    let _e33: Plane = other_989;
    let _e36: Plane = other_989;
    let _e39: Plane = other_989;
    let _e42: Plane = other_989;
    let _e55: MotorDual = self_1163;
    let _e59: Plane = other_989;
    let _e62: Plane = other_989;
    let _e65: Plane = other_989;
    let _e68: Plane = other_989;
    let _e81: MotorDual = self_1163;
    let _e85: Plane = other_989;
    let _e88: Plane = other_989;
    let _e91: Plane = other_989;
    let _e94: Plane = other_989;
    return Motor((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.z, _e11.g0_.z, _e14.g0_.z, _e17.g0_.y)) * vec4<f32>(0.0, 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e29.g0_.z) * vec4<f32>(_e33.g0_.y, _e36.g0_.z, _e39.g0_.y, _e42.g0_.x)) * vec4<f32>(1.0, -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e55.g0_.w) * vec4<f32>(_e59.g0_.z, _e62.g0_.y, _e65.g0_.x, _e68.g0_.z)) * vec4<f32>(1.0, 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e81.g0_.x) * vec4<f32>(_e85.g0_.x, _e88.g0_.x, _e91.g0_.y, _e94.g0_.z)) * vec4<f32>(0.0, 0.0, 1.0, 1.0))));
}

fn motor_dual_plane_regressive_product(self_1164: MotorDual, other_990: Plane) -> Plane {
    var self_1165: MotorDual;
    var other_991: Plane;

    self_1165 = self_1164;
    other_991 = other_990;
    let _e4: MotorDual = self_1165;
    let _e8: Plane = other_991;
    return Plane((vec3<f32>(_e4.g0_.x) * _e8.g0_));
}

fn motor_dual_plane_outer_product(self_1166: MotorDual, other_992: Plane) -> Point {
    var self_1167: MotorDual;
    var other_993: Plane;

    self_1167 = self_1166;
    other_993 = other_992;
    let _e4: MotorDual = self_1167;
    let _e8: Plane = other_993;
    let _e18: MotorDual = self_1167;
    let _e22: Plane = other_993;
    let _e33: MotorDual = self_1167;
    let _e36: MotorDual = self_1167;
    let _e39: MotorDual = self_1167;
    let _e43: Plane = other_993;
    return Point(((((vec3<f32>(_e4.g0_.z) * _e8.g0_.zzx) * vec3<f32>(-(1.0), 0.0, 1.0)) + ((vec3<f32>(_e18.g0_.w) * _e22.g0_.yxy) * vec3<f32>(1.0, -(1.0), 0.0))) + ((vec3<f32>(_e33.g0_.x, _e36.g0_.y, _e39.g0_.y) * _e43.g0_.xzy) * vec3<f32>(0.0, 1.0, -(1.0)))));
}

fn motor_dual_plane_inner_product(self_1168: MotorDual, other_994: Plane) -> Translator {
    var self_1169: MotorDual;
    var other_995: Plane;

    self_1169 = self_1168;
    other_995 = other_994;
    let _e4: MotorDual = self_1169;
    let _e8: Plane = other_995;
    let _e18: MotorDual = self_1169;
    let _e21: MotorDual = self_1169;
    let _e24: MotorDual = self_1169;
    let _e28: Plane = other_995;
    return Translator((((vec3<f32>(_e4.g0_.w) * vec3<f32>(_e8.g0_.z)) * vec3<f32>(1.0, 0.0, 0.0)) + (vec3<f32>(_e18.g0_.z, _e21.g0_.x, _e24.g0_.x) * _e28.g0_.yyz)));
}

fn motor_dual_plane_left_contraction(self_1170: MotorDual, other_996: Plane) -> Scalar {
    var self_1171: MotorDual;
    var other_997: Plane;

    self_1171 = self_1170;
    other_997 = other_996;
    let _e4: MotorDual = self_1171;
    let _e7: Plane = other_997;
    let _e11: MotorDual = self_1171;
    let _e14: Plane = other_997;
    return Scalar(((_e4.g0_.z * _e7.g0_.y) + (_e11.g0_.w * _e14.g0_.z)));
}

fn motor_dual_plane_right_contraction(self_1172: MotorDual, other_998: Plane) -> Translator {
    var self_1173: MotorDual;
    var other_999: Plane;

    self_1173 = self_1172;
    other_999 = other_998;
    let _e4: MotorDual = self_1173;
    let _e8: Plane = other_999;
    let _e18: MotorDual = self_1173;
    let _e21: MotorDual = self_1173;
    let _e24: MotorDual = self_1173;
    let _e28: Plane = other_999;
    return Translator((((vec3<f32>(_e4.g0_.w) * vec3<f32>(_e8.g0_.z)) * vec3<f32>(1.0, 0.0, 0.0)) + (vec3<f32>(_e18.g0_.z, _e21.g0_.x, _e24.g0_.x) * _e28.g0_.yyz)));
}

fn motor_dual_plane_scalar_product(self_1174: MotorDual, other_1000: Plane) -> Scalar {
    var self_1175: MotorDual;
    var other_1001: Plane;

    self_1175 = self_1174;
    other_1001 = other_1000;
    let _e4: MotorDual = self_1175;
    let _e7: Plane = other_1001;
    let _e11: MotorDual = self_1175;
    let _e14: Plane = other_1001;
    return Scalar(((_e4.g0_.z * _e7.g0_.y) + (_e11.g0_.w * _e14.g0_.z)));
}

fn motor_dual_translator_geometric_product(self_1176: MotorDual, other_1002: Translator) -> MotorDual {
    var self_1177: MotorDual;
    var other_1003: Translator;

    self_1177 = self_1176;
    other_1003 = other_1002;
    let _e4: MotorDual = self_1177;
    let _e8: Translator = other_1003;
    let _e11: Translator = other_1003;
    let _e14: Translator = other_1003;
    let _e17: Translator = other_1003;
    let _e28: MotorDual = self_1177;
    let _e32: Translator = other_1003;
    let _e35: Translator = other_1003;
    let _e38: Translator = other_1003;
    let _e41: Translator = other_1003;
    let _e54: MotorDual = self_1177;
    let _e57: Translator = other_1003;
    return MotorDual(((((vec4<f32>(_e4.g0_.z) * vec4<f32>(_e8.g0_.y, _e11.g0_.z, _e14.g0_.x, _e17.g0_.y)) * vec4<f32>(1.0, 1.0, 1.0, 0.0)) + ((vec4<f32>(_e28.g0_.w) * vec4<f32>(_e32.g0_.z, _e35.g0_.y, _e38.g0_.z, _e41.g0_.x)) * vec4<f32>(1.0, -(1.0), 0.0, 1.0))) + ((_e54.g0_.xyxx * vec4<f32>(_e57.g0_.x)) * vec4<f32>(1.0, 1.0, 0.0, 0.0))));
}

fn motor_dual_translator_regressive_product(self_1178: MotorDual, other_1004: Translator) -> Translator {
    var self_1179: MotorDual;
    var other_1005: Translator;

    self_1179 = self_1178;
    other_1005 = other_1004;
    let _e4: MotorDual = self_1179;
    let _e8: Translator = other_1005;
    let _e11: MotorDual = self_1179;
    let _e15: Translator = other_1005;
    let _e26: MotorDual = self_1179;
    let _e29: MotorDual = self_1179;
    let _e32: MotorDual = self_1179;
    let _e36: Translator = other_1005;
    return Translator((((vec3<f32>(_e4.g0_.x) * _e8.g0_) + ((vec3<f32>(_e11.g0_.w) * vec3<f32>(_e15.g0_.z)) * vec3<f32>(1.0, 0.0, 0.0))) + ((vec3<f32>(_e26.g0_.z, _e29.g0_.x, _e32.g0_.x) * _e36.g0_.yxx) * vec3<f32>(1.0, 0.0, 0.0))));
}

fn motor_dual_translator_outer_product(self_1180: MotorDual, other_1006: Translator) -> MotorDual {
    var self_1181: MotorDual;
    var other_1007: Translator;

    self_1181 = self_1180;
    other_1007 = other_1006;
    let _e4: MotorDual = self_1181;
    let _e8: Translator = other_1007;
    let _e11: Translator = other_1007;
    let _e14: Translator = other_1007;
    let _e17: Translator = other_1007;
    let _e28: MotorDual = self_1181;
    let _e32: Translator = other_1007;
    let _e35: Translator = other_1007;
    let _e38: Translator = other_1007;
    let _e41: Translator = other_1007;
    let _e53: MotorDual = self_1181;
    let _e56: Translator = other_1007;
    return MotorDual(((((vec4<f32>(_e4.g0_.z) * vec4<f32>(_e8.g0_.y, _e11.g0_.y, _e14.g0_.x, _e17.g0_.y)) * vec4<f32>(1.0, 0.0, 1.0, 0.0)) + ((vec4<f32>(_e28.g0_.w) * vec4<f32>(_e32.g0_.z, _e35.g0_.z, _e38.g0_.z, _e41.g0_.x)) * vec4<f32>(1.0, 0.0, 0.0, 1.0))) + ((_e53.g0_.xyxx * vec4<f32>(_e56.g0_.x)) * vec4<f32>(1.0, 1.0, 0.0, 0.0))));
}

fn motor_dual_translator_inner_product(self_1182: MotorDual, other_1008: Translator) -> MotorDual {
    var self_1183: MotorDual;
    var other_1009: Translator;

    self_1183 = self_1182;
    other_1009 = other_1008;
    let _e4: MotorDual = self_1183;
    let _e8: Translator = other_1009;
    let _e11: Translator = other_1009;
    let _e14: Translator = other_1009;
    let _e17: Translator = other_1009;
    let _e28: MotorDual = self_1183;
    let _e32: Translator = other_1009;
    let _e35: Translator = other_1009;
    let _e38: Translator = other_1009;
    let _e41: Translator = other_1009;
    let _e54: MotorDual = self_1183;
    let _e57: Translator = other_1009;
    return MotorDual(((((vec4<f32>(_e4.g0_.z) * vec4<f32>(_e8.g0_.z, _e11.g0_.z, _e14.g0_.x, _e17.g0_.z)) * vec4<f32>(0.0, 1.0, 1.0, 0.0)) + ((vec4<f32>(_e28.g0_.w) * vec4<f32>(_e32.g0_.y, _e35.g0_.y, _e38.g0_.y, _e41.g0_.x)) * vec4<f32>(0.0, -(1.0), 0.0, 1.0))) + ((_e54.g0_.xyxx * vec4<f32>(_e57.g0_.x)) * vec4<f32>(1.0, 1.0, 0.0, 0.0))));
}

fn motor_dual_translator_right_contraction(self_1184: MotorDual, other_1010: Translator) -> MotorDual {
    var self_1185: MotorDual;
    var other_1011: Translator;

    self_1185 = self_1184;
    other_1011 = other_1010;
    let _e4: MotorDual = self_1185;
    let _e6: Translator = other_1011;
    return MotorDual((_e4.g0_ * vec4<f32>(_e6.g0_.x)));
}

fn motor_dual_motor_add(self_1186: MotorDual, other_1012: Motor) -> MultiVector {
    var self_1187: MotorDual;
    var other_1013: Motor;

    self_1187 = self_1186;
    other_1013 = other_1012;
    let _e4: MotorDual = self_1187;
    let _e13: Motor = other_1013;
    let _e23: MotorDual = self_1187;
    let _e32: Motor = other_1013;
    return MultiVector(((_e4.g0_.xxwz * vec4<f32>(0.0, 0.0, 1.0, 1.0)) + (_e13.g0_.xyxx * vec4<f32>(1.0, 1.0, 0.0, 0.0))), ((_e23.g0_.yxxx * vec4<f32>(1.0, 1.0, 0.0, 0.0)) + (_e32.g0_.xxzw * vec4<f32>(0.0, 0.0, 1.0, 1.0))));
}

fn motor_dual_motor_sub(self_1188: MotorDual, other_1014: Motor) -> MultiVector {
    var self_1189: MotorDual;
    var other_1015: Motor;

    self_1189 = self_1188;
    other_1015 = other_1014;
    let _e4: MotorDual = self_1189;
    let _e13: Motor = other_1015;
    let _e23: MotorDual = self_1189;
    let _e32: Motor = other_1015;
    return MultiVector(((_e4.g0_.xxwz * vec4<f32>(0.0, 0.0, 1.0, 1.0)) - (_e13.g0_.xyxx * vec4<f32>(1.0, 1.0, 0.0, 0.0))), ((_e23.g0_.yxxx * vec4<f32>(1.0, 1.0, 0.0, 0.0)) - (_e32.g0_.xxzw * vec4<f32>(0.0, 0.0, 1.0, 1.0))));
}

fn motor_dual_motor_geometric_product(self_1190: MotorDual, other_1016: Motor) -> MotorDual {
    var self_1191: MotorDual;
    var other_1017: Motor;

    self_1191 = self_1190;
    other_1017 = other_1016;
    let _e4: MotorDual = self_1191;
    let _e8: Motor = other_1017;
    let _e18: MotorDual = self_1191;
    let _e22: Motor = other_1017;
    let _e34: MotorDual = self_1191;
    let _e38: Motor = other_1017;
    let _e50: MotorDual = self_1191;
    let _e54: Motor = other_1017;
    return MotorDual((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.yxyy) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) + ((vec4<f32>(_e18.g0_.z) * _e22.g0_.zwxy) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e34.g0_.w) * _e38.g0_.wzyx) * vec4<f32>(1.0, -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e50.g0_.x) * _e54.g0_.xyxx) * vec4<f32>(1.0, -(1.0), 0.0, 0.0))));
}

fn motor_dual_motor_regressive_product(self_1192: MotorDual, other_1018: Motor) -> Motor {
    var self_1193: MotorDual;
    var other_1019: Motor;

    self_1193 = self_1192;
    other_1019 = other_1018;
    let _e4: MotorDual = self_1193;
    let _e8: Motor = other_1019;
    let _e11: MotorDual = self_1193;
    let _e15: Motor = other_1019;
    let _e27: MotorDual = self_1193;
    let _e31: Motor = other_1019;
    let _e43: MotorDual = self_1193;
    let _e46: Motor = other_1019;
    return Motor(((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.z) * vec4<f32>(_e15.g0_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e27.g0_.w) * vec4<f32>(_e31.g0_.w)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((_e43.g0_.yxxx * _e46.g0_.yxxx) * vec4<f32>(1.0, 0.0, 0.0, 0.0))));
}

fn motor_dual_motor_outer_product(self_1194: MotorDual, other_1020: Motor) -> MotorDual {
    var self_1195: MotorDual;
    var other_1021: Motor;

    self_1195 = self_1194;
    other_1021 = other_1020;
    let _e4: MotorDual = self_1195;
    let _e8: Motor = other_1021;
    let _e18: MotorDual = self_1195;
    let _e22: Motor = other_1021;
    let _e33: MotorDual = self_1195;
    let _e37: Motor = other_1021;
    let _e48: MotorDual = self_1195;
    let _e52: Motor = other_1021;
    return MotorDual((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.yxyy) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) + ((vec4<f32>(_e18.g0_.z) * _e22.g0_.zzxz) * vec4<f32>(1.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e33.g0_.w) * _e37.g0_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e48.g0_.x) * vec4<f32>(_e52.g0_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))));
}

fn motor_dual_motor_inner_product(self_1196: MotorDual, other_1022: Motor) -> MotorDual {
    var self_1197: MotorDual;
    var other_1023: Motor;

    self_1197 = self_1196;
    other_1023 = other_1022;
    let _e4: MotorDual = self_1197;
    let _e8: Motor = other_1023;
    let _e19: MotorDual = self_1197;
    let _e23: Motor = other_1023;
    let _e35: MotorDual = self_1197;
    let _e39: Motor = other_1023;
    let _e51: MotorDual = self_1197;
    let _e55: Motor = other_1023;
    return MotorDual((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.x)) * vec4<f32>(0.0, 1.0, 0.0, 0.0)) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.wwxy) * vec4<f32>(0.0, 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e35.g0_.w) * _e39.g0_.zzyx) * vec4<f32>(0.0, -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e51.g0_.x) * _e55.g0_.xyxx) * vec4<f32>(1.0, -(1.0), 0.0, 0.0))));
}

fn motor_dual_motor_left_contraction(self_1198: MotorDual, other_1024: Motor) -> Plane {
    var self_1199: MotorDual;
    var other_1025: Motor;

    self_1199 = self_1198;
    other_1025 = other_1024;
    let _e4: MotorDual = self_1199;
    let _e8: Motor = other_1025;
    let _e11: Motor = other_1025;
    let _e14: Motor = other_1025;
    let _e25: MotorDual = self_1199;
    let _e28: MotorDual = self_1199;
    let _e31: MotorDual = self_1199;
    let _e35: Motor = other_1025;
    let _e38: Motor = other_1025;
    let _e41: Motor = other_1025;
    return Plane((((vec3<f32>(_e4.g0_.w) * vec3<f32>(_e8.g0_.z, _e11.g0_.y, _e14.g0_.z)) * vec3<f32>(-(1.0), 1.0, 0.0)) + ((vec3<f32>(_e25.g0_.z, _e28.g0_.x, _e31.g0_.z) * vec3<f32>(_e35.g0_.w, _e38.g0_.x, _e41.g0_.y)) * vec3<f32>(1.0, 0.0, -(1.0)))));
}

fn motor_dual_motor_right_contraction(self_1200: MotorDual, other_1026: Motor) -> MotorDual {
    var self_1201: MotorDual;
    var other_1027: Motor;

    self_1201 = self_1200;
    other_1027 = other_1026;
    let _e4: MotorDual = self_1201;
    let _e8: Motor = other_1027;
    let _e19: MotorDual = self_1201;
    let _e22: Motor = other_1027;
    return MotorDual((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.x)) * vec4<f32>(0.0, 1.0, 0.0, 0.0)) + ((_e19.g0_.xxzw * _e22.g0_.xyxx) * vec4<f32>(1.0, -(1.0), 1.0, 1.0))));
}

fn motor_dual_motor_dual_add(self_1202: MotorDual, other_1028: MotorDual) -> MotorDual {
    var self_1203: MotorDual;
    var other_1029: MotorDual;

    self_1203 = self_1202;
    other_1029 = other_1028;
    let _e4: MotorDual = self_1203;
    let _e6: MotorDual = other_1029;
    return MotorDual((_e4.g0_ + _e6.g0_));
}

fn motor_dual_motor_dual_sub(self_1204: MotorDual, other_1030: MotorDual) -> MotorDual {
    var self_1205: MotorDual;
    var other_1031: MotorDual;

    self_1205 = self_1204;
    other_1031 = other_1030;
    let _e4: MotorDual = self_1205;
    let _e6: MotorDual = other_1031;
    return MotorDual((_e4.g0_ - _e6.g0_));
}

fn motor_dual_motor_dual_mul(self_1206: MotorDual, other_1032: MotorDual) -> MotorDual {
    var self_1207: MotorDual;
    var other_1033: MotorDual;

    self_1207 = self_1206;
    other_1033 = other_1032;
    let _e4: MotorDual = self_1207;
    let _e6: MotorDual = other_1033;
    return MotorDual((_e4.g0_ * _e6.g0_));
}

fn motor_dual_motor_dual_div(self_1208: MotorDual, other_1034: MotorDual) -> MotorDual {
    var self_1209: MotorDual;
    var other_1035: MotorDual;

    self_1209 = self_1208;
    other_1035 = other_1034;
    let _e4: MotorDual = self_1209;
    let _e7: MotorDual = self_1209;
    let _e10: MotorDual = self_1209;
    let _e13: MotorDual = self_1209;
    let _e23: MotorDual = other_1035;
    let _e26: MotorDual = other_1035;
    let _e29: MotorDual = other_1035;
    let _e32: MotorDual = other_1035;
    return MotorDual((((vec4<f32>(_e4.g0_.x, _e7.g0_.y, _e10.g0_.z, _e13.g0_.w) * vec4<f32>(1.0, 1.0, 1.0, 1.0)) / vec4<f32>(_e23.g0_.x, _e26.g0_.y, _e29.g0_.z, _e32.g0_.w)) * vec4<f32>(1.0, 1.0, 1.0, 1.0)));
}

fn motor_dual_motor_dual_geometric_product(self_1210: MotorDual, other_1036: MotorDual) -> Motor {
    var self_1211: MotorDual;
    var other_1037: MotorDual;

    self_1211 = self_1210;
    other_1037 = other_1036;
    let _e4: MotorDual = self_1211;
    let _e8: MotorDual = other_1037;
    let _e19: MotorDual = self_1211;
    let _e23: MotorDual = other_1037;
    let _e35: MotorDual = self_1211;
    let _e39: MotorDual = other_1037;
    let _e51: MotorDual = self_1211;
    let _e55: MotorDual = other_1037;
    return Motor((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.zwxy) * vec4<f32>(1.0, -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e35.g0_.w) * _e39.g0_.wzyx) * vec4<f32>(1.0, 1.0, -(1.0), 1.0))) + ((vec4<f32>(_e51.g0_.x) * _e55.g0_.xxzw) * vec4<f32>(0.0, 0.0, 1.0, 1.0))));
}

fn motor_dual_motor_dual_regressive_product(self_1212: MotorDual, other_1038: MotorDual) -> MotorDual {
    var self_1213: MotorDual;
    var other_1039: MotorDual;

    self_1213 = self_1212;
    other_1039 = other_1038;
    let _e4: MotorDual = self_1213;
    let _e8: MotorDual = other_1039;
    let _e11: MotorDual = self_1213;
    let _e13: MotorDual = other_1039;
    return MotorDual(((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((_e11.g0_ * vec4<f32>(_e13.g0_.x)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn motor_dual_motor_dual_outer_product(self_1214: MotorDual, other_1040: MotorDual) -> Point {
    var self_1215: MotorDual;
    var other_1041: MotorDual;

    self_1215 = self_1214;
    other_1041 = other_1040;
    let _e4: MotorDual = self_1215;
    let _e8: MotorDual = other_1041;
    let _e11: MotorDual = other_1041;
    let _e14: MotorDual = other_1041;
    let _e25: MotorDual = self_1215;
    let _e29: MotorDual = other_1041;
    let _e32: MotorDual = other_1041;
    let _e35: MotorDual = other_1041;
    let _e47: MotorDual = self_1215;
    let _e50: MotorDual = self_1215;
    let _e53: MotorDual = self_1215;
    let _e57: MotorDual = other_1041;
    let _e60: MotorDual = other_1041;
    let _e63: MotorDual = other_1041;
    return Point(((((vec3<f32>(_e4.g0_.z) * vec3<f32>(_e8.g0_.w, _e11.g0_.w, _e14.g0_.y)) * vec3<f32>(-(1.0), 0.0, 1.0)) + ((vec3<f32>(_e25.g0_.w) * vec3<f32>(_e29.g0_.z, _e32.g0_.y, _e35.g0_.z)) * vec3<f32>(1.0, -(1.0), 0.0))) + ((vec3<f32>(_e47.g0_.x, _e50.g0_.y, _e53.g0_.y) * vec3<f32>(_e57.g0_.x, _e60.g0_.w, _e63.g0_.z)) * vec3<f32>(0.0, 1.0, -(1.0)))));
}

fn motor_dual_motor_dual_inner_product(self_1216: MotorDual, other_1042: MotorDual) -> Translator {
    var self_1217: MotorDual;
    var other_1043: MotorDual;

    self_1217 = self_1216;
    other_1043 = other_1042;
    let _e4: MotorDual = self_1217;
    let _e8: MotorDual = other_1043;
    let _e11: MotorDual = other_1043;
    let _e14: MotorDual = other_1043;
    let _e24: MotorDual = self_1217;
    let _e28: MotorDual = other_1043;
    let _e31: MotorDual = other_1043;
    let _e34: MotorDual = other_1043;
    let _e45: MotorDual = self_1217;
    let _e49: MotorDual = other_1043;
    let _e52: MotorDual = other_1043;
    let _e55: MotorDual = other_1043;
    return Translator(((((vec3<f32>(_e4.g0_.z) * vec3<f32>(_e8.g0_.z, _e11.g0_.x, _e14.g0_.z)) * vec3<f32>(1.0, 1.0, 0.0)) + ((vec3<f32>(_e24.g0_.w) * vec3<f32>(_e28.g0_.w, _e31.g0_.w, _e34.g0_.x)) * vec3<f32>(1.0, 0.0, 1.0))) + ((vec3<f32>(_e45.g0_.x) * vec3<f32>(_e49.g0_.x, _e52.g0_.z, _e55.g0_.w)) * vec3<f32>(0.0, 1.0, 1.0))));
}

fn motor_dual_motor_dual_left_contraction(self_1218: MotorDual, other_1044: MotorDual) -> Translator {
    var self_1219: MotorDual;
    var other_1045: MotorDual;

    self_1219 = self_1218;
    other_1045 = other_1044;
    let _e4: MotorDual = self_1219;
    let _e8: MotorDual = other_1045;
    let _e11: MotorDual = other_1045;
    let _e14: MotorDual = other_1045;
    let _e24: MotorDual = self_1219;
    let _e27: MotorDual = self_1219;
    let _e30: MotorDual = self_1219;
    let _e34: MotorDual = other_1045;
    let _e37: MotorDual = other_1045;
    let _e40: MotorDual = other_1045;
    return Translator((((vec3<f32>(_e4.g0_.w) * vec3<f32>(_e8.g0_.w, _e11.g0_.w, _e14.g0_.x)) * vec3<f32>(1.0, 0.0, 1.0)) + ((vec3<f32>(_e24.g0_.z, _e27.g0_.z, _e30.g0_.x) * vec3<f32>(_e34.g0_.z, _e37.g0_.x, _e40.g0_.x)) * vec3<f32>(1.0, 1.0, 0.0))));
}

fn motor_dual_motor_dual_right_contraction(self_1220: MotorDual, other_1046: MotorDual) -> Translator {
    var self_1221: MotorDual;
    var other_1047: MotorDual;

    self_1221 = self_1220;
    other_1047 = other_1046;
    let _e4: MotorDual = self_1221;
    let _e8: MotorDual = other_1047;
    let _e18: MotorDual = self_1221;
    let _e21: MotorDual = self_1221;
    let _e24: MotorDual = self_1221;
    let _e28: MotorDual = other_1047;
    let _e31: MotorDual = other_1047;
    let _e34: MotorDual = other_1047;
    return Translator((((vec3<f32>(_e4.g0_.w) * vec3<f32>(_e8.g0_.w)) * vec3<f32>(1.0, 0.0, 0.0)) + (vec3<f32>(_e18.g0_.z, _e21.g0_.x, _e24.g0_.x) * vec3<f32>(_e28.g0_.z, _e31.g0_.z, _e34.g0_.w))));
}

fn motor_dual_motor_dual_scalar_product(self_1222: MotorDual, other_1048: MotorDual) -> Scalar {
    var self_1223: MotorDual;
    var other_1049: MotorDual;

    self_1223 = self_1222;
    other_1049 = other_1048;
    let _e4: MotorDual = self_1223;
    let _e7: MotorDual = other_1049;
    let _e11: MotorDual = self_1223;
    let _e14: MotorDual = other_1049;
    return Scalar(((_e4.g0_.z * _e7.g0_.z) + (_e11.g0_.w * _e14.g0_.w)));
}

fn motor_dual_squared_magnitude(self_1224: MotorDual) -> Scalar {
    var self_1225: MotorDual;

    self_1225 = self_1224;
    let _e4: MotorDual = self_1225;
    let _e5: MotorDual = motor_dual_reversal(_e4);
    let _e6: MotorDual = self_1225;
    let _e8: MotorDual = self_1225;
    let _e9: MotorDual = motor_dual_reversal(_e8);
    let _e10: Scalar = motor_dual_motor_dual_scalar_product(_e6, _e9);
    return _e10;
}

fn motor_dual_magnitude(self_1226: MotorDual) -> Scalar {
    var self_1227: MotorDual;

    self_1227 = self_1226;
    let _e3: MotorDual = self_1227;
    let _e4: Scalar = motor_dual_squared_magnitude(_e3);
    let _e7: MotorDual = self_1227;
    let _e8: Scalar = motor_dual_squared_magnitude(_e7);
    return Scalar(sqrt(_e8.g0_));
}

fn motor_dual_scale(self_1228: MotorDual, other_1050: f32) -> MotorDual {
    var self_1229: MotorDual;
    var other_1051: f32;

    self_1229 = self_1228;
    other_1051 = other_1050;
    let _e5: f32 = other_1051;
    let _e7: MotorDual = self_1229;
    let _e8: f32 = other_1051;
    let _e10: MotorDual = motor_dual_scalar_geometric_product(_e7, Scalar(_e8));
    return _e10;
}

fn motor_dual_signum(self_1230: MotorDual) -> MotorDual {
    var self_1231: MotorDual;

    self_1231 = self_1230;
    let _e5: MotorDual = self_1231;
    let _e6: Scalar = motor_dual_magnitude(_e5);
    let _e10: MotorDual = self_1231;
    let _e13: MotorDual = self_1231;
    let _e14: Scalar = motor_dual_magnitude(_e13);
    let _e18: MotorDual = motor_dual_scalar_geometric_product(_e10, Scalar((1.0 / _e14.g0_)));
    return _e18;
}

fn motor_dual_inverse(self_1232: MotorDual) -> MotorDual {
    var self_1233: MotorDual;

    self_1233 = self_1232;
    let _e3: MotorDual = self_1233;
    let _e4: MotorDual = motor_dual_reversal(_e3);
    let _e7: MotorDual = self_1233;
    let _e8: Scalar = motor_dual_squared_magnitude(_e7);
    let _e13: MotorDual = self_1233;
    let _e14: MotorDual = motor_dual_reversal(_e13);
    let _e17: MotorDual = self_1233;
    let _e18: Scalar = motor_dual_squared_magnitude(_e17);
    let _e22: MotorDual = motor_dual_scalar_geometric_product(_e14, Scalar((1.0 / _e18.g0_)));
    return _e22;
}

fn ideal_point_motor_geometric_quotient(self_1234: IdealPoint, other_1052: Motor) -> IdealPoint {
    var self_1235: IdealPoint;
    var other_1053: Motor;

    self_1235 = self_1234;
    other_1053 = other_1052;
    let _e6: Motor = other_1053;
    let _e7: Motor = motor_inverse(_e6);
    let _e8: IdealPoint = self_1235;
    let _e10: Motor = other_1053;
    let _e11: Motor = motor_inverse(_e10);
    let _e12: IdealPoint = ideal_point_motor_geometric_product(_e8, _e11);
    return _e12;
}

fn ideal_point_point_geometric_quotient(self_1236: IdealPoint, other_1054: Point) -> IdealPoint {
    var self_1237: IdealPoint;
    var other_1055: Point;

    self_1237 = self_1236;
    other_1055 = other_1054;
    let _e6: Point = other_1055;
    let _e7: Point = point_inverse(_e6);
    let _e8: IdealPoint = self_1237;
    let _e10: Point = other_1055;
    let _e11: Point = point_inverse(_e10);
    let _e12: IdealPoint = ideal_point_point_geometric_product(_e8, _e11);
    return _e12;
}

fn ideal_point_rotor_geometric_quotient(self_1238: IdealPoint, other_1056: Rotor) -> IdealPoint {
    var self_1239: IdealPoint;
    var other_1057: Rotor;

    self_1239 = self_1238;
    other_1057 = other_1056;
    let _e6: Rotor = other_1057;
    let _e7: Rotor = rotor_inverse(_e6);
    let _e8: IdealPoint = self_1239;
    let _e10: Rotor = other_1057;
    let _e11: Rotor = rotor_inverse(_e10);
    let _e12: IdealPoint = ideal_point_rotor_geometric_product(_e8, _e11);
    return _e12;
}

fn ideal_point_scalar_geometric_quotient(self_1240: IdealPoint, other_1058: Scalar) -> IdealPoint {
    var self_1241: IdealPoint;
    var other_1059: Scalar;

    self_1241 = self_1240;
    other_1059 = other_1058;
    let _e6: Scalar = other_1059;
    let _e7: Scalar = scalar_inverse(_e6);
    let _e8: IdealPoint = self_1241;
    let _e10: Scalar = other_1059;
    let _e11: Scalar = scalar_inverse(_e10);
    let _e12: IdealPoint = ideal_point_scalar_geometric_product(_e8, _e11);
    return _e12;
}

fn ideal_point_translator_geometric_quotient(self_1242: IdealPoint, other_1060: Translator) -> IdealPoint {
    var self_1243: IdealPoint;
    var other_1061: Translator;

    self_1243 = self_1242;
    other_1061 = other_1060;
    let _e6: Translator = other_1061;
    let _e7: Translator = translator_inverse(_e6);
    let _e8: IdealPoint = self_1243;
    let _e10: Translator = other_1061;
    let _e11: Translator = translator_inverse(_e10);
    let _e12: IdealPoint = ideal_point_translator_geometric_product(_e8, _e11);
    return _e12;
}

fn motor_ideal_point_transformation(self_1244: Motor, other_1062: IdealPoint) -> IdealPoint {
    var self_1245: Motor;
    var other_1063: IdealPoint;

    self_1245 = self_1244;
    other_1063 = other_1062;
    let _e6: Motor = self_1245;
    let _e7: IdealPoint = other_1063;
    let _e8: IdealPoint = motor_ideal_point_geometric_product(_e6, _e7);
    let _e10: Motor = self_1245;
    let _e11: Motor = motor_reversal(_e10);
    let _e14: Motor = self_1245;
    let _e15: IdealPoint = other_1063;
    let _e16: IdealPoint = motor_ideal_point_geometric_product(_e14, _e15);
    let _e18: Motor = self_1245;
    let _e19: Motor = motor_reversal(_e18);
    let _e20: IdealPoint = ideal_point_motor_geometric_product(_e16, _e19);
    return _e20;
}

fn motor_powi(self_1246: Motor, exponent: i32) -> Motor {
    var self_1247: Motor;
    var exponent_1: i32;
    var local: Motor;
    var x: Motor;
    var y: Motor;
    var n: i32;

    self_1247 = self_1246;
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
        let _e12: Motor = self_1247;
        let _e13: Motor = motor_inverse(_e12);
        local = _e13;
    } else {
        let _e14: Motor = self_1247;
        local = _e14;
    }
    let _e16: Motor = local;
    x = _e16;
    let _e18: Motor = motor_one();
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
                    let _e35: Motor = x;
                    let _e36: Motor = y;
                    let _e37: Motor = motor_motor_geometric_product(_e35, _e36);
                    y = _e37;
                }
            }
            let _e40: Motor = x;
            let _e41: Motor = x;
            let _e42: Motor = motor_motor_geometric_product(_e40, _e41);
            x = _e42;
            let _e43: i32 = n;
            n = (_e43 >> u32(1));
        }
    }
    let _e49: Motor = x;
    let _e50: Motor = y;
    let _e51: Motor = motor_motor_geometric_product(_e49, _e50);
    return _e51;
}

fn motor_motor_geometric_quotient(self_1248: Motor, other_1064: Motor) -> Motor {
    var self_1249: Motor;
    var other_1065: Motor;

    self_1249 = self_1248;
    other_1065 = other_1064;
    let _e6: Motor = other_1065;
    let _e7: Motor = motor_inverse(_e6);
    let _e8: Motor = self_1249;
    let _e10: Motor = other_1065;
    let _e11: Motor = motor_inverse(_e10);
    let _e12: Motor = motor_motor_geometric_product(_e8, _e11);
    return _e12;
}

fn motor_motor_transformation(self_1250: Motor, other_1066: Motor) -> Motor {
    var self_1251: Motor;
    var other_1067: Motor;

    self_1251 = self_1250;
    other_1067 = other_1066;
    let _e6: Motor = self_1251;
    let _e7: Motor = other_1067;
    let _e8: Motor = motor_motor_geometric_product(_e6, _e7);
    let _e10: Motor = self_1251;
    let _e11: Motor = motor_reversal(_e10);
    let _e14: Motor = self_1251;
    let _e15: Motor = other_1067;
    let _e16: Motor = motor_motor_geometric_product(_e14, _e15);
    let _e18: Motor = self_1251;
    let _e19: Motor = motor_reversal(_e18);
    let _e20: Motor = motor_motor_geometric_product(_e16, _e19);
    return _e20;
}

fn motor_motor_dual_geometric_quotient(self_1252: Motor, other_1068: MotorDual) -> MotorDual {
    var self_1253: Motor;
    var other_1069: MotorDual;

    self_1253 = self_1252;
    other_1069 = other_1068;
    let _e6: MotorDual = other_1069;
    let _e7: MotorDual = motor_dual_inverse(_e6);
    let _e8: Motor = self_1253;
    let _e10: MotorDual = other_1069;
    let _e11: MotorDual = motor_dual_inverse(_e10);
    let _e12: MotorDual = motor_motor_dual_geometric_product(_e8, _e11);
    return _e12;
}

fn motor_motor_dual_transformation(self_1254: Motor, other_1070: MotorDual) -> MotorDual {
    var self_1255: Motor;
    var other_1071: MotorDual;

    self_1255 = self_1254;
    other_1071 = other_1070;
    let _e6: Motor = self_1255;
    let _e7: MotorDual = other_1071;
    let _e8: MotorDual = motor_motor_dual_geometric_product(_e6, _e7);
    let _e10: Motor = self_1255;
    let _e11: Motor = motor_reversal(_e10);
    let _e14: Motor = self_1255;
    let _e15: MotorDual = other_1071;
    let _e16: MotorDual = motor_motor_dual_geometric_product(_e14, _e15);
    let _e18: Motor = self_1255;
    let _e19: Motor = motor_reversal(_e18);
    let _e20: MotorDual = motor_dual_motor_geometric_product(_e16, _e19);
    return _e20;
}

fn motor_multi_vector_geometric_quotient(self_1256: Motor, other_1072: MultiVector) -> MultiVector {
    var self_1257: Motor;
    var other_1073: MultiVector;

    self_1257 = self_1256;
    other_1073 = other_1072;
    let _e6: MultiVector = other_1073;
    let _e7: MultiVector = multi_vector_inverse(_e6);
    let _e8: Motor = self_1257;
    let _e10: MultiVector = other_1073;
    let _e11: MultiVector = multi_vector_inverse(_e10);
    let _e12: MultiVector = motor_multi_vector_geometric_product(_e8, _e11);
    return _e12;
}

fn motor_multi_vector_transformation(self_1258: Motor, other_1074: MultiVector) -> MultiVector {
    var self_1259: Motor;
    var other_1075: MultiVector;

    self_1259 = self_1258;
    other_1075 = other_1074;
    let _e6: Motor = self_1259;
    let _e7: MultiVector = other_1075;
    let _e8: MultiVector = motor_multi_vector_geometric_product(_e6, _e7);
    let _e10: Motor = self_1259;
    let _e11: Motor = motor_reversal(_e10);
    let _e14: Motor = self_1259;
    let _e15: MultiVector = other_1075;
    let _e16: MultiVector = motor_multi_vector_geometric_product(_e14, _e15);
    let _e18: Motor = self_1259;
    let _e19: Motor = motor_reversal(_e18);
    let _e20: MultiVector = multi_vector_motor_geometric_product(_e16, _e19);
    return _e20;
}

fn motor_plane_geometric_quotient(self_1260: Motor, other_1076: Plane) -> MotorDual {
    var self_1261: Motor;
    var other_1077: Plane;

    self_1261 = self_1260;
    other_1077 = other_1076;
    let _e6: Plane = other_1077;
    let _e7: Plane = plane_inverse(_e6);
    let _e8: Motor = self_1261;
    let _e10: Plane = other_1077;
    let _e11: Plane = plane_inverse(_e10);
    let _e12: MotorDual = motor_plane_geometric_product(_e8, _e11);
    return _e12;
}

fn motor_plane_transformation(self_1262: Motor, other_1078: Plane) -> Plane {
    var self_1263: Motor;
    var other_1079: Plane;

    self_1263 = self_1262;
    other_1079 = other_1078;
    let _e6: Motor = self_1263;
    let _e7: Plane = other_1079;
    let _e8: MotorDual = motor_plane_geometric_product(_e6, _e7);
    let _e10: Motor = self_1263;
    let _e11: Motor = motor_reversal(_e10);
    let _e14: Motor = self_1263;
    let _e15: Plane = other_1079;
    let _e16: MotorDual = motor_plane_geometric_product(_e14, _e15);
    let _e18: Motor = self_1263;
    let _e19: Motor = motor_reversal(_e18);
    let _e20: MotorDual = motor_dual_motor_geometric_product(_e16, _e19);
    let _e23: Motor = self_1263;
    let _e24: Plane = other_1079;
    let _e25: MotorDual = motor_plane_geometric_product(_e23, _e24);
    let _e27: Motor = self_1263;
    let _e28: Motor = motor_reversal(_e27);
    let _e31: Motor = self_1263;
    let _e32: Plane = other_1079;
    let _e33: MotorDual = motor_plane_geometric_product(_e31, _e32);
    let _e35: Motor = self_1263;
    let _e36: Motor = motor_reversal(_e35);
    let _e37: MotorDual = motor_dual_motor_geometric_product(_e33, _e36);
    let _e38: Plane = motor_dual_plane_into(_e37);
    return _e38;
}

fn motor_point_geometric_quotient(self_1264: Motor, other_1080: Point) -> Motor {
    var self_1265: Motor;
    var other_1081: Point;

    self_1265 = self_1264;
    other_1081 = other_1080;
    let _e6: Point = other_1081;
    let _e7: Point = point_inverse(_e6);
    let _e8: Motor = self_1265;
    let _e10: Point = other_1081;
    let _e11: Point = point_inverse(_e10);
    let _e12: Motor = motor_point_geometric_product(_e8, _e11);
    return _e12;
}

fn motor_point_transformation(self_1266: Motor, other_1082: Point) -> Point {
    var self_1267: Motor;
    var other_1083: Point;

    self_1267 = self_1266;
    other_1083 = other_1082;
    let _e6: Motor = self_1267;
    let _e7: Point = other_1083;
    let _e8: Motor = motor_point_geometric_product(_e6, _e7);
    let _e10: Motor = self_1267;
    let _e11: Motor = motor_reversal(_e10);
    let _e14: Motor = self_1267;
    let _e15: Point = other_1083;
    let _e16: Motor = motor_point_geometric_product(_e14, _e15);
    let _e18: Motor = self_1267;
    let _e19: Motor = motor_reversal(_e18);
    let _e20: Motor = motor_motor_geometric_product(_e16, _e19);
    let _e23: Motor = self_1267;
    let _e24: Point = other_1083;
    let _e25: Motor = motor_point_geometric_product(_e23, _e24);
    let _e27: Motor = self_1267;
    let _e28: Motor = motor_reversal(_e27);
    let _e31: Motor = self_1267;
    let _e32: Point = other_1083;
    let _e33: Motor = motor_point_geometric_product(_e31, _e32);
    let _e35: Motor = self_1267;
    let _e36: Motor = motor_reversal(_e35);
    let _e37: Motor = motor_motor_geometric_product(_e33, _e36);
    let _e38: Point = motor_point_into(_e37);
    return _e38;
}

fn motor_rotor_geometric_quotient(self_1268: Motor, other_1084: Rotor) -> Motor {
    var self_1269: Motor;
    var other_1085: Rotor;

    self_1269 = self_1268;
    other_1085 = other_1084;
    let _e6: Rotor = other_1085;
    let _e7: Rotor = rotor_inverse(_e6);
    let _e8: Motor = self_1269;
    let _e10: Rotor = other_1085;
    let _e11: Rotor = rotor_inverse(_e10);
    let _e12: Motor = motor_rotor_geometric_product(_e8, _e11);
    return _e12;
}

fn motor_rotor_transformation(self_1270: Motor, other_1086: Rotor) -> Rotor {
    var self_1271: Motor;
    var other_1087: Rotor;

    self_1271 = self_1270;
    other_1087 = other_1086;
    let _e6: Motor = self_1271;
    let _e7: Rotor = other_1087;
    let _e8: Motor = motor_rotor_geometric_product(_e6, _e7);
    let _e10: Motor = self_1271;
    let _e11: Motor = motor_reversal(_e10);
    let _e14: Motor = self_1271;
    let _e15: Rotor = other_1087;
    let _e16: Motor = motor_rotor_geometric_product(_e14, _e15);
    let _e18: Motor = self_1271;
    let _e19: Motor = motor_reversal(_e18);
    let _e20: Motor = motor_motor_geometric_product(_e16, _e19);
    let _e23: Motor = self_1271;
    let _e24: Rotor = other_1087;
    let _e25: Motor = motor_rotor_geometric_product(_e23, _e24);
    let _e27: Motor = self_1271;
    let _e28: Motor = motor_reversal(_e27);
    let _e31: Motor = self_1271;
    let _e32: Rotor = other_1087;
    let _e33: Motor = motor_rotor_geometric_product(_e31, _e32);
    let _e35: Motor = self_1271;
    let _e36: Motor = motor_reversal(_e35);
    let _e37: Motor = motor_motor_geometric_product(_e33, _e36);
    let _e38: Rotor = motor_rotor_into(_e37);
    return _e38;
}

fn motor_scalar_geometric_quotient(self_1272: Motor, other_1088: Scalar) -> Motor {
    var self_1273: Motor;
    var other_1089: Scalar;

    self_1273 = self_1272;
    other_1089 = other_1088;
    let _e6: Scalar = other_1089;
    let _e7: Scalar = scalar_inverse(_e6);
    let _e8: Motor = self_1273;
    let _e10: Scalar = other_1089;
    let _e11: Scalar = scalar_inverse(_e10);
    let _e12: Motor = motor_scalar_geometric_product(_e8, _e11);
    return _e12;
}

fn motor_scalar_transformation(self_1274: Motor, other_1090: Scalar) -> Scalar {
    var self_1275: Motor;
    var other_1091: Scalar;

    self_1275 = self_1274;
    other_1091 = other_1090;
    let _e6: Motor = self_1275;
    let _e7: Scalar = other_1091;
    let _e8: Motor = motor_scalar_geometric_product(_e6, _e7);
    let _e10: Motor = self_1275;
    let _e11: Motor = motor_reversal(_e10);
    let _e14: Motor = self_1275;
    let _e15: Scalar = other_1091;
    let _e16: Motor = motor_scalar_geometric_product(_e14, _e15);
    let _e18: Motor = self_1275;
    let _e19: Motor = motor_reversal(_e18);
    let _e20: Motor = motor_motor_geometric_product(_e16, _e19);
    let _e23: Motor = self_1275;
    let _e24: Scalar = other_1091;
    let _e25: Motor = motor_scalar_geometric_product(_e23, _e24);
    let _e27: Motor = self_1275;
    let _e28: Motor = motor_reversal(_e27);
    let _e31: Motor = self_1275;
    let _e32: Scalar = other_1091;
    let _e33: Motor = motor_scalar_geometric_product(_e31, _e32);
    let _e35: Motor = self_1275;
    let _e36: Motor = motor_reversal(_e35);
    let _e37: Motor = motor_motor_geometric_product(_e33, _e36);
    let _e38: Scalar = motor_scalar_into(_e37);
    return _e38;
}

fn motor_translator_geometric_quotient(self_1276: Motor, other_1092: Translator) -> Motor {
    var self_1277: Motor;
    var other_1093: Translator;

    self_1277 = self_1276;
    other_1093 = other_1092;
    let _e6: Translator = other_1093;
    let _e7: Translator = translator_inverse(_e6);
    let _e8: Motor = self_1277;
    let _e10: Translator = other_1093;
    let _e11: Translator = translator_inverse(_e10);
    let _e12: Motor = motor_translator_geometric_product(_e8, _e11);
    return _e12;
}

fn motor_translator_transformation(self_1278: Motor, other_1094: Translator) -> Translator {
    var self_1279: Motor;
    var other_1095: Translator;

    self_1279 = self_1278;
    other_1095 = other_1094;
    let _e6: Motor = self_1279;
    let _e7: Translator = other_1095;
    let _e8: Motor = motor_translator_geometric_product(_e6, _e7);
    let _e10: Motor = self_1279;
    let _e11: Motor = motor_reversal(_e10);
    let _e14: Motor = self_1279;
    let _e15: Translator = other_1095;
    let _e16: Motor = motor_translator_geometric_product(_e14, _e15);
    let _e18: Motor = self_1279;
    let _e19: Motor = motor_reversal(_e18);
    let _e20: Motor = motor_motor_geometric_product(_e16, _e19);
    let _e23: Motor = self_1279;
    let _e24: Translator = other_1095;
    let _e25: Motor = motor_translator_geometric_product(_e23, _e24);
    let _e27: Motor = self_1279;
    let _e28: Motor = motor_reversal(_e27);
    let _e31: Motor = self_1279;
    let _e32: Translator = other_1095;
    let _e33: Motor = motor_translator_geometric_product(_e31, _e32);
    let _e35: Motor = self_1279;
    let _e36: Motor = motor_reversal(_e35);
    let _e37: Motor = motor_motor_geometric_product(_e33, _e36);
    let _e38: Translator = motor_translator_into(_e37);
    return _e38;
}

fn motor_dual_motor_geometric_quotient(self_1280: MotorDual, other_1096: Motor) -> MotorDual {
    var self_1281: MotorDual;
    var other_1097: Motor;

    self_1281 = self_1280;
    other_1097 = other_1096;
    let _e6: Motor = other_1097;
    let _e7: Motor = motor_inverse(_e6);
    let _e8: MotorDual = self_1281;
    let _e10: Motor = other_1097;
    let _e11: Motor = motor_inverse(_e10);
    let _e12: MotorDual = motor_dual_motor_geometric_product(_e8, _e11);
    return _e12;
}

fn motor_dual_motor_transformation(self_1282: MotorDual, other_1098: Motor) -> Motor {
    var self_1283: MotorDual;
    var other_1099: Motor;

    self_1283 = self_1282;
    other_1099 = other_1098;
    let _e6: MotorDual = self_1283;
    let _e7: Motor = other_1099;
    let _e8: MotorDual = motor_dual_motor_geometric_product(_e6, _e7);
    let _e10: MotorDual = self_1283;
    let _e11: MotorDual = motor_dual_reversal(_e10);
    let _e14: MotorDual = self_1283;
    let _e15: Motor = other_1099;
    let _e16: MotorDual = motor_dual_motor_geometric_product(_e14, _e15);
    let _e18: MotorDual = self_1283;
    let _e19: MotorDual = motor_dual_reversal(_e18);
    let _e20: Motor = motor_dual_motor_dual_geometric_product(_e16, _e19);
    return _e20;
}

fn motor_dual_motor_dual_geometric_quotient(self_1284: MotorDual, other_1100: MotorDual) -> Motor {
    var self_1285: MotorDual;
    var other_1101: MotorDual;

    self_1285 = self_1284;
    other_1101 = other_1100;
    let _e6: MotorDual = other_1101;
    let _e7: MotorDual = motor_dual_inverse(_e6);
    let _e8: MotorDual = self_1285;
    let _e10: MotorDual = other_1101;
    let _e11: MotorDual = motor_dual_inverse(_e10);
    let _e12: Motor = motor_dual_motor_dual_geometric_product(_e8, _e11);
    return _e12;
}

fn motor_dual_motor_dual_transformation(self_1286: MotorDual, other_1102: MotorDual) -> MotorDual {
    var self_1287: MotorDual;
    var other_1103: MotorDual;

    self_1287 = self_1286;
    other_1103 = other_1102;
    let _e6: MotorDual = self_1287;
    let _e7: MotorDual = other_1103;
    let _e8: Motor = motor_dual_motor_dual_geometric_product(_e6, _e7);
    let _e10: MotorDual = self_1287;
    let _e11: MotorDual = motor_dual_reversal(_e10);
    let _e14: MotorDual = self_1287;
    let _e15: MotorDual = other_1103;
    let _e16: Motor = motor_dual_motor_dual_geometric_product(_e14, _e15);
    let _e18: MotorDual = self_1287;
    let _e19: MotorDual = motor_dual_reversal(_e18);
    let _e20: MotorDual = motor_motor_dual_geometric_product(_e16, _e19);
    return _e20;
}

fn motor_dual_multi_vector_geometric_quotient(self_1288: MotorDual, other_1104: MultiVector) -> MultiVector {
    var self_1289: MotorDual;
    var other_1105: MultiVector;

    self_1289 = self_1288;
    other_1105 = other_1104;
    let _e6: MultiVector = other_1105;
    let _e7: MultiVector = multi_vector_inverse(_e6);
    let _e8: MotorDual = self_1289;
    let _e10: MultiVector = other_1105;
    let _e11: MultiVector = multi_vector_inverse(_e10);
    let _e12: MultiVector = motor_dual_multi_vector_geometric_product(_e8, _e11);
    return _e12;
}

fn motor_dual_multi_vector_transformation(self_1290: MotorDual, other_1106: MultiVector) -> MultiVector {
    var self_1291: MotorDual;
    var other_1107: MultiVector;

    self_1291 = self_1290;
    other_1107 = other_1106;
    let _e6: MotorDual = self_1291;
    let _e7: MultiVector = other_1107;
    let _e8: MultiVector = motor_dual_multi_vector_geometric_product(_e6, _e7);
    let _e10: MotorDual = self_1291;
    let _e11: MotorDual = motor_dual_reversal(_e10);
    let _e14: MotorDual = self_1291;
    let _e15: MultiVector = other_1107;
    let _e16: MultiVector = motor_dual_multi_vector_geometric_product(_e14, _e15);
    let _e18: MotorDual = self_1291;
    let _e19: MotorDual = motor_dual_reversal(_e18);
    let _e20: MultiVector = multi_vector_motor_dual_geometric_product(_e16, _e19);
    return _e20;
}

fn motor_dual_plane_geometric_quotient(self_1292: MotorDual, other_1108: Plane) -> Motor {
    var self_1293: MotorDual;
    var other_1109: Plane;

    self_1293 = self_1292;
    other_1109 = other_1108;
    let _e6: Plane = other_1109;
    let _e7: Plane = plane_inverse(_e6);
    let _e8: MotorDual = self_1293;
    let _e10: Plane = other_1109;
    let _e11: Plane = plane_inverse(_e10);
    let _e12: Motor = motor_dual_plane_geometric_product(_e8, _e11);
    return _e12;
}

fn motor_dual_plane_transformation(self_1294: MotorDual, other_1110: Plane) -> Plane {
    var self_1295: MotorDual;
    var other_1111: Plane;

    self_1295 = self_1294;
    other_1111 = other_1110;
    let _e6: MotorDual = self_1295;
    let _e7: Plane = other_1111;
    let _e8: Motor = motor_dual_plane_geometric_product(_e6, _e7);
    let _e10: MotorDual = self_1295;
    let _e11: MotorDual = motor_dual_reversal(_e10);
    let _e14: MotorDual = self_1295;
    let _e15: Plane = other_1111;
    let _e16: Motor = motor_dual_plane_geometric_product(_e14, _e15);
    let _e18: MotorDual = self_1295;
    let _e19: MotorDual = motor_dual_reversal(_e18);
    let _e20: MotorDual = motor_motor_dual_geometric_product(_e16, _e19);
    let _e23: MotorDual = self_1295;
    let _e24: Plane = other_1111;
    let _e25: Motor = motor_dual_plane_geometric_product(_e23, _e24);
    let _e27: MotorDual = self_1295;
    let _e28: MotorDual = motor_dual_reversal(_e27);
    let _e31: MotorDual = self_1295;
    let _e32: Plane = other_1111;
    let _e33: Motor = motor_dual_plane_geometric_product(_e31, _e32);
    let _e35: MotorDual = self_1295;
    let _e36: MotorDual = motor_dual_reversal(_e35);
    let _e37: MotorDual = motor_motor_dual_geometric_product(_e33, _e36);
    let _e38: Plane = motor_dual_plane_into(_e37);
    return _e38;
}

fn motor_dual_point_geometric_quotient(self_1296: MotorDual, other_1112: Point) -> MotorDual {
    var self_1297: MotorDual;
    var other_1113: Point;

    self_1297 = self_1296;
    other_1113 = other_1112;
    let _e6: Point = other_1113;
    let _e7: Point = point_inverse(_e6);
    let _e8: MotorDual = self_1297;
    let _e10: Point = other_1113;
    let _e11: Point = point_inverse(_e10);
    let _e12: MotorDual = motor_dual_point_geometric_product(_e8, _e11);
    return _e12;
}

fn motor_dual_point_transformation(self_1298: MotorDual, other_1114: Point) -> Point {
    var self_1299: MotorDual;
    var other_1115: Point;

    self_1299 = self_1298;
    other_1115 = other_1114;
    let _e6: MotorDual = self_1299;
    let _e7: Point = other_1115;
    let _e8: MotorDual = motor_dual_point_geometric_product(_e6, _e7);
    let _e10: MotorDual = self_1299;
    let _e11: MotorDual = motor_dual_reversal(_e10);
    let _e14: MotorDual = self_1299;
    let _e15: Point = other_1115;
    let _e16: MotorDual = motor_dual_point_geometric_product(_e14, _e15);
    let _e18: MotorDual = self_1299;
    let _e19: MotorDual = motor_dual_reversal(_e18);
    let _e20: Motor = motor_dual_motor_dual_geometric_product(_e16, _e19);
    let _e23: MotorDual = self_1299;
    let _e24: Point = other_1115;
    let _e25: MotorDual = motor_dual_point_geometric_product(_e23, _e24);
    let _e27: MotorDual = self_1299;
    let _e28: MotorDual = motor_dual_reversal(_e27);
    let _e31: MotorDual = self_1299;
    let _e32: Point = other_1115;
    let _e33: MotorDual = motor_dual_point_geometric_product(_e31, _e32);
    let _e35: MotorDual = self_1299;
    let _e36: MotorDual = motor_dual_reversal(_e35);
    let _e37: Motor = motor_dual_motor_dual_geometric_product(_e33, _e36);
    let _e38: Point = motor_point_into(_e37);
    return _e38;
}

fn motor_dual_rotor_geometric_quotient(self_1300: MotorDual, other_1116: Rotor) -> MotorDual {
    var self_1301: MotorDual;
    var other_1117: Rotor;

    self_1301 = self_1300;
    other_1117 = other_1116;
    let _e6: Rotor = other_1117;
    let _e7: Rotor = rotor_inverse(_e6);
    let _e8: MotorDual = self_1301;
    let _e10: Rotor = other_1117;
    let _e11: Rotor = rotor_inverse(_e10);
    let _e12: MotorDual = motor_dual_rotor_geometric_product(_e8, _e11);
    return _e12;
}

fn motor_dual_rotor_transformation(self_1302: MotorDual, other_1118: Rotor) -> Rotor {
    var self_1303: MotorDual;
    var other_1119: Rotor;

    self_1303 = self_1302;
    other_1119 = other_1118;
    let _e6: MotorDual = self_1303;
    let _e7: Rotor = other_1119;
    let _e8: MotorDual = motor_dual_rotor_geometric_product(_e6, _e7);
    let _e10: MotorDual = self_1303;
    let _e11: MotorDual = motor_dual_reversal(_e10);
    let _e14: MotorDual = self_1303;
    let _e15: Rotor = other_1119;
    let _e16: MotorDual = motor_dual_rotor_geometric_product(_e14, _e15);
    let _e18: MotorDual = self_1303;
    let _e19: MotorDual = motor_dual_reversal(_e18);
    let _e20: Motor = motor_dual_motor_dual_geometric_product(_e16, _e19);
    let _e23: MotorDual = self_1303;
    let _e24: Rotor = other_1119;
    let _e25: MotorDual = motor_dual_rotor_geometric_product(_e23, _e24);
    let _e27: MotorDual = self_1303;
    let _e28: MotorDual = motor_dual_reversal(_e27);
    let _e31: MotorDual = self_1303;
    let _e32: Rotor = other_1119;
    let _e33: MotorDual = motor_dual_rotor_geometric_product(_e31, _e32);
    let _e35: MotorDual = self_1303;
    let _e36: MotorDual = motor_dual_reversal(_e35);
    let _e37: Motor = motor_dual_motor_dual_geometric_product(_e33, _e36);
    let _e38: Rotor = motor_rotor_into(_e37);
    return _e38;
}

fn motor_dual_scalar_geometric_quotient(self_1304: MotorDual, other_1120: Scalar) -> MotorDual {
    var self_1305: MotorDual;
    var other_1121: Scalar;

    self_1305 = self_1304;
    other_1121 = other_1120;
    let _e6: Scalar = other_1121;
    let _e7: Scalar = scalar_inverse(_e6);
    let _e8: MotorDual = self_1305;
    let _e10: Scalar = other_1121;
    let _e11: Scalar = scalar_inverse(_e10);
    let _e12: MotorDual = motor_dual_scalar_geometric_product(_e8, _e11);
    return _e12;
}

fn motor_dual_scalar_transformation(self_1306: MotorDual, other_1122: Scalar) -> Scalar {
    var self_1307: MotorDual;
    var other_1123: Scalar;

    self_1307 = self_1306;
    other_1123 = other_1122;
    let _e6: MotorDual = self_1307;
    let _e7: Scalar = other_1123;
    let _e8: MotorDual = motor_dual_scalar_geometric_product(_e6, _e7);
    let _e10: MotorDual = self_1307;
    let _e11: MotorDual = motor_dual_reversal(_e10);
    let _e14: MotorDual = self_1307;
    let _e15: Scalar = other_1123;
    let _e16: MotorDual = motor_dual_scalar_geometric_product(_e14, _e15);
    let _e18: MotorDual = self_1307;
    let _e19: MotorDual = motor_dual_reversal(_e18);
    let _e20: Motor = motor_dual_motor_dual_geometric_product(_e16, _e19);
    let _e23: MotorDual = self_1307;
    let _e24: Scalar = other_1123;
    let _e25: MotorDual = motor_dual_scalar_geometric_product(_e23, _e24);
    let _e27: MotorDual = self_1307;
    let _e28: MotorDual = motor_dual_reversal(_e27);
    let _e31: MotorDual = self_1307;
    let _e32: Scalar = other_1123;
    let _e33: MotorDual = motor_dual_scalar_geometric_product(_e31, _e32);
    let _e35: MotorDual = self_1307;
    let _e36: MotorDual = motor_dual_reversal(_e35);
    let _e37: Motor = motor_dual_motor_dual_geometric_product(_e33, _e36);
    let _e38: Scalar = motor_scalar_into(_e37);
    return _e38;
}

fn motor_dual_translator_geometric_quotient(self_1308: MotorDual, other_1124: Translator) -> MotorDual {
    var self_1309: MotorDual;
    var other_1125: Translator;

    self_1309 = self_1308;
    other_1125 = other_1124;
    let _e6: Translator = other_1125;
    let _e7: Translator = translator_inverse(_e6);
    let _e8: MotorDual = self_1309;
    let _e10: Translator = other_1125;
    let _e11: Translator = translator_inverse(_e10);
    let _e12: MotorDual = motor_dual_translator_geometric_product(_e8, _e11);
    return _e12;
}

fn motor_dual_translator_transformation(self_1310: MotorDual, other_1126: Translator) -> Translator {
    var self_1311: MotorDual;
    var other_1127: Translator;

    self_1311 = self_1310;
    other_1127 = other_1126;
    let _e6: MotorDual = self_1311;
    let _e7: Translator = other_1127;
    let _e8: MotorDual = motor_dual_translator_geometric_product(_e6, _e7);
    let _e10: MotorDual = self_1311;
    let _e11: MotorDual = motor_dual_reversal(_e10);
    let _e14: MotorDual = self_1311;
    let _e15: Translator = other_1127;
    let _e16: MotorDual = motor_dual_translator_geometric_product(_e14, _e15);
    let _e18: MotorDual = self_1311;
    let _e19: MotorDual = motor_dual_reversal(_e18);
    let _e20: Motor = motor_dual_motor_dual_geometric_product(_e16, _e19);
    let _e23: MotorDual = self_1311;
    let _e24: Translator = other_1127;
    let _e25: MotorDual = motor_dual_translator_geometric_product(_e23, _e24);
    let _e27: MotorDual = self_1311;
    let _e28: MotorDual = motor_dual_reversal(_e27);
    let _e31: MotorDual = self_1311;
    let _e32: Translator = other_1127;
    let _e33: MotorDual = motor_dual_translator_geometric_product(_e31, _e32);
    let _e35: MotorDual = self_1311;
    let _e36: MotorDual = motor_dual_reversal(_e35);
    let _e37: Motor = motor_dual_motor_dual_geometric_product(_e33, _e36);
    let _e38: Translator = motor_translator_into(_e37);
    return _e38;
}

fn multi_vector_motor_geometric_quotient(self_1312: MultiVector, other_1128: Motor) -> MultiVector {
    var self_1313: MultiVector;
    var other_1129: Motor;

    self_1313 = self_1312;
    other_1129 = other_1128;
    let _e6: Motor = other_1129;
    let _e7: Motor = motor_inverse(_e6);
    let _e8: MultiVector = self_1313;
    let _e10: Motor = other_1129;
    let _e11: Motor = motor_inverse(_e10);
    let _e12: MultiVector = multi_vector_motor_geometric_product(_e8, _e11);
    return _e12;
}

fn multi_vector_motor_transformation(self_1314: MultiVector, other_1130: Motor) -> Motor {
    var self_1315: MultiVector;
    var other_1131: Motor;

    self_1315 = self_1314;
    other_1131 = other_1130;
    let _e6: MultiVector = self_1315;
    let _e7: Motor = other_1131;
    let _e8: MultiVector = multi_vector_motor_geometric_product(_e6, _e7);
    let _e10: MultiVector = self_1315;
    let _e11: MultiVector = multi_vector_reversal(_e10);
    let _e14: MultiVector = self_1315;
    let _e15: Motor = other_1131;
    let _e16: MultiVector = multi_vector_motor_geometric_product(_e14, _e15);
    let _e18: MultiVector = self_1315;
    let _e19: MultiVector = multi_vector_reversal(_e18);
    let _e20: MultiVector = multi_vector_multi_vector_geometric_product(_e16, _e19);
    let _e23: MultiVector = self_1315;
    let _e24: Motor = other_1131;
    let _e25: MultiVector = multi_vector_motor_geometric_product(_e23, _e24);
    let _e27: MultiVector = self_1315;
    let _e28: MultiVector = multi_vector_reversal(_e27);
    let _e31: MultiVector = self_1315;
    let _e32: Motor = other_1131;
    let _e33: MultiVector = multi_vector_motor_geometric_product(_e31, _e32);
    let _e35: MultiVector = self_1315;
    let _e36: MultiVector = multi_vector_reversal(_e35);
    let _e37: MultiVector = multi_vector_multi_vector_geometric_product(_e33, _e36);
    let _e38: Motor = multi_vector_motor_into(_e37);
    return _e38;
}

fn multi_vector_motor_dual_geometric_quotient(self_1316: MultiVector, other_1132: MotorDual) -> MultiVector {
    var self_1317: MultiVector;
    var other_1133: MotorDual;

    self_1317 = self_1316;
    other_1133 = other_1132;
    let _e6: MotorDual = other_1133;
    let _e7: MotorDual = motor_dual_inverse(_e6);
    let _e8: MultiVector = self_1317;
    let _e10: MotorDual = other_1133;
    let _e11: MotorDual = motor_dual_inverse(_e10);
    let _e12: MultiVector = multi_vector_motor_dual_geometric_product(_e8, _e11);
    return _e12;
}

fn multi_vector_motor_dual_transformation(self_1318: MultiVector, other_1134: MotorDual) -> MotorDual {
    var self_1319: MultiVector;
    var other_1135: MotorDual;

    self_1319 = self_1318;
    other_1135 = other_1134;
    let _e6: MultiVector = self_1319;
    let _e7: MotorDual = other_1135;
    let _e8: MultiVector = multi_vector_motor_dual_geometric_product(_e6, _e7);
    let _e10: MultiVector = self_1319;
    let _e11: MultiVector = multi_vector_reversal(_e10);
    let _e14: MultiVector = self_1319;
    let _e15: MotorDual = other_1135;
    let _e16: MultiVector = multi_vector_motor_dual_geometric_product(_e14, _e15);
    let _e18: MultiVector = self_1319;
    let _e19: MultiVector = multi_vector_reversal(_e18);
    let _e20: MultiVector = multi_vector_multi_vector_geometric_product(_e16, _e19);
    let _e23: MultiVector = self_1319;
    let _e24: MotorDual = other_1135;
    let _e25: MultiVector = multi_vector_motor_dual_geometric_product(_e23, _e24);
    let _e27: MultiVector = self_1319;
    let _e28: MultiVector = multi_vector_reversal(_e27);
    let _e31: MultiVector = self_1319;
    let _e32: MotorDual = other_1135;
    let _e33: MultiVector = multi_vector_motor_dual_geometric_product(_e31, _e32);
    let _e35: MultiVector = self_1319;
    let _e36: MultiVector = multi_vector_reversal(_e35);
    let _e37: MultiVector = multi_vector_multi_vector_geometric_product(_e33, _e36);
    let _e38: MotorDual = multi_vector_motor_dual_into(_e37);
    return _e38;
}

fn multi_vector_powi(self_1320: MultiVector, exponent_2: i32) -> MultiVector {
    var self_1321: MultiVector;
    var exponent_3: i32;
    var local_1: MultiVector;
    var x_1: MultiVector;
    var y_1: MultiVector;
    var n_1: i32;

    self_1321 = self_1320;
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
        let _e12: MultiVector = self_1321;
        let _e13: MultiVector = multi_vector_inverse(_e12);
        local_1 = _e13;
    } else {
        let _e14: MultiVector = self_1321;
        local_1 = _e14;
    }
    let _e16: MultiVector = local_1;
    x_1 = _e16;
    let _e18: MultiVector = multi_vector_one();
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
                    let _e35: MultiVector = x_1;
                    let _e36: MultiVector = y_1;
                    let _e37: MultiVector = multi_vector_multi_vector_geometric_product(_e35, _e36);
                    y_1 = _e37;
                }
            }
            let _e40: MultiVector = x_1;
            let _e41: MultiVector = x_1;
            let _e42: MultiVector = multi_vector_multi_vector_geometric_product(_e40, _e41);
            x_1 = _e42;
            let _e43: i32 = n_1;
            n_1 = (_e43 >> u32(1));
        }
    }
    let _e49: MultiVector = x_1;
    let _e50: MultiVector = y_1;
    let _e51: MultiVector = multi_vector_multi_vector_geometric_product(_e49, _e50);
    return _e51;
}

fn multi_vector_multi_vector_geometric_quotient(self_1322: MultiVector, other_1136: MultiVector) -> MultiVector {
    var self_1323: MultiVector;
    var other_1137: MultiVector;

    self_1323 = self_1322;
    other_1137 = other_1136;
    let _e6: MultiVector = other_1137;
    let _e7: MultiVector = multi_vector_inverse(_e6);
    let _e8: MultiVector = self_1323;
    let _e10: MultiVector = other_1137;
    let _e11: MultiVector = multi_vector_inverse(_e10);
    let _e12: MultiVector = multi_vector_multi_vector_geometric_product(_e8, _e11);
    return _e12;
}

fn multi_vector_multi_vector_transformation(self_1324: MultiVector, other_1138: MultiVector) -> MultiVector {
    var self_1325: MultiVector;
    var other_1139: MultiVector;

    self_1325 = self_1324;
    other_1139 = other_1138;
    let _e6: MultiVector = self_1325;
    let _e7: MultiVector = other_1139;
    let _e8: MultiVector = multi_vector_multi_vector_geometric_product(_e6, _e7);
    let _e10: MultiVector = self_1325;
    let _e11: MultiVector = multi_vector_reversal(_e10);
    let _e14: MultiVector = self_1325;
    let _e15: MultiVector = other_1139;
    let _e16: MultiVector = multi_vector_multi_vector_geometric_product(_e14, _e15);
    let _e18: MultiVector = self_1325;
    let _e19: MultiVector = multi_vector_reversal(_e18);
    let _e20: MultiVector = multi_vector_multi_vector_geometric_product(_e16, _e19);
    return _e20;
}

fn multi_vector_plane_geometric_quotient(self_1326: MultiVector, other_1140: Plane) -> MultiVector {
    var self_1327: MultiVector;
    var other_1141: Plane;

    self_1327 = self_1326;
    other_1141 = other_1140;
    let _e6: Plane = other_1141;
    let _e7: Plane = plane_inverse(_e6);
    let _e8: MultiVector = self_1327;
    let _e10: Plane = other_1141;
    let _e11: Plane = plane_inverse(_e10);
    let _e12: MultiVector = multi_vector_plane_geometric_product(_e8, _e11);
    return _e12;
}

fn multi_vector_plane_transformation(self_1328: MultiVector, other_1142: Plane) -> Plane {
    var self_1329: MultiVector;
    var other_1143: Plane;

    self_1329 = self_1328;
    other_1143 = other_1142;
    let _e6: MultiVector = self_1329;
    let _e7: Plane = other_1143;
    let _e8: MultiVector = multi_vector_plane_geometric_product(_e6, _e7);
    let _e10: MultiVector = self_1329;
    let _e11: MultiVector = multi_vector_reversal(_e10);
    let _e14: MultiVector = self_1329;
    let _e15: Plane = other_1143;
    let _e16: MultiVector = multi_vector_plane_geometric_product(_e14, _e15);
    let _e18: MultiVector = self_1329;
    let _e19: MultiVector = multi_vector_reversal(_e18);
    let _e20: MultiVector = multi_vector_multi_vector_geometric_product(_e16, _e19);
    let _e23: MultiVector = self_1329;
    let _e24: Plane = other_1143;
    let _e25: MultiVector = multi_vector_plane_geometric_product(_e23, _e24);
    let _e27: MultiVector = self_1329;
    let _e28: MultiVector = multi_vector_reversal(_e27);
    let _e31: MultiVector = self_1329;
    let _e32: Plane = other_1143;
    let _e33: MultiVector = multi_vector_plane_geometric_product(_e31, _e32);
    let _e35: MultiVector = self_1329;
    let _e36: MultiVector = multi_vector_reversal(_e35);
    let _e37: MultiVector = multi_vector_multi_vector_geometric_product(_e33, _e36);
    let _e38: Plane = multi_vector_plane_into(_e37);
    return _e38;
}

fn multi_vector_point_geometric_quotient(self_1330: MultiVector, other_1144: Point) -> MultiVector {
    var self_1331: MultiVector;
    var other_1145: Point;

    self_1331 = self_1330;
    other_1145 = other_1144;
    let _e6: Point = other_1145;
    let _e7: Point = point_inverse(_e6);
    let _e8: MultiVector = self_1331;
    let _e10: Point = other_1145;
    let _e11: Point = point_inverse(_e10);
    let _e12: MultiVector = multi_vector_point_geometric_product(_e8, _e11);
    return _e12;
}

fn multi_vector_point_transformation(self_1332: MultiVector, other_1146: Point) -> Point {
    var self_1333: MultiVector;
    var other_1147: Point;

    self_1333 = self_1332;
    other_1147 = other_1146;
    let _e6: MultiVector = self_1333;
    let _e7: Point = other_1147;
    let _e8: MultiVector = multi_vector_point_geometric_product(_e6, _e7);
    let _e10: MultiVector = self_1333;
    let _e11: MultiVector = multi_vector_reversal(_e10);
    let _e14: MultiVector = self_1333;
    let _e15: Point = other_1147;
    let _e16: MultiVector = multi_vector_point_geometric_product(_e14, _e15);
    let _e18: MultiVector = self_1333;
    let _e19: MultiVector = multi_vector_reversal(_e18);
    let _e20: MultiVector = multi_vector_multi_vector_geometric_product(_e16, _e19);
    let _e23: MultiVector = self_1333;
    let _e24: Point = other_1147;
    let _e25: MultiVector = multi_vector_point_geometric_product(_e23, _e24);
    let _e27: MultiVector = self_1333;
    let _e28: MultiVector = multi_vector_reversal(_e27);
    let _e31: MultiVector = self_1333;
    let _e32: Point = other_1147;
    let _e33: MultiVector = multi_vector_point_geometric_product(_e31, _e32);
    let _e35: MultiVector = self_1333;
    let _e36: MultiVector = multi_vector_reversal(_e35);
    let _e37: MultiVector = multi_vector_multi_vector_geometric_product(_e33, _e36);
    let _e38: Point = multi_vector_point_into(_e37);
    return _e38;
}

fn multi_vector_rotor_geometric_quotient(self_1334: MultiVector, other_1148: Rotor) -> MultiVector {
    var self_1335: MultiVector;
    var other_1149: Rotor;

    self_1335 = self_1334;
    other_1149 = other_1148;
    let _e6: Rotor = other_1149;
    let _e7: Rotor = rotor_inverse(_e6);
    let _e8: MultiVector = self_1335;
    let _e10: Rotor = other_1149;
    let _e11: Rotor = rotor_inverse(_e10);
    let _e12: MultiVector = multi_vector_rotor_geometric_product(_e8, _e11);
    return _e12;
}

fn multi_vector_rotor_transformation(self_1336: MultiVector, other_1150: Rotor) -> Rotor {
    var self_1337: MultiVector;
    var other_1151: Rotor;

    self_1337 = self_1336;
    other_1151 = other_1150;
    let _e6: MultiVector = self_1337;
    let _e7: Rotor = other_1151;
    let _e8: MultiVector = multi_vector_rotor_geometric_product(_e6, _e7);
    let _e10: MultiVector = self_1337;
    let _e11: MultiVector = multi_vector_reversal(_e10);
    let _e14: MultiVector = self_1337;
    let _e15: Rotor = other_1151;
    let _e16: MultiVector = multi_vector_rotor_geometric_product(_e14, _e15);
    let _e18: MultiVector = self_1337;
    let _e19: MultiVector = multi_vector_reversal(_e18);
    let _e20: MultiVector = multi_vector_multi_vector_geometric_product(_e16, _e19);
    let _e23: MultiVector = self_1337;
    let _e24: Rotor = other_1151;
    let _e25: MultiVector = multi_vector_rotor_geometric_product(_e23, _e24);
    let _e27: MultiVector = self_1337;
    let _e28: MultiVector = multi_vector_reversal(_e27);
    let _e31: MultiVector = self_1337;
    let _e32: Rotor = other_1151;
    let _e33: MultiVector = multi_vector_rotor_geometric_product(_e31, _e32);
    let _e35: MultiVector = self_1337;
    let _e36: MultiVector = multi_vector_reversal(_e35);
    let _e37: MultiVector = multi_vector_multi_vector_geometric_product(_e33, _e36);
    let _e38: Rotor = multi_vector_rotor_into(_e37);
    return _e38;
}

fn multi_vector_scalar_geometric_quotient(self_1338: MultiVector, other_1152: Scalar) -> MultiVector {
    var self_1339: MultiVector;
    var other_1153: Scalar;

    self_1339 = self_1338;
    other_1153 = other_1152;
    let _e6: Scalar = other_1153;
    let _e7: Scalar = scalar_inverse(_e6);
    let _e8: MultiVector = self_1339;
    let _e10: Scalar = other_1153;
    let _e11: Scalar = scalar_inverse(_e10);
    let _e12: MultiVector = multi_vector_scalar_geometric_product(_e8, _e11);
    return _e12;
}

fn multi_vector_scalar_transformation(self_1340: MultiVector, other_1154: Scalar) -> Scalar {
    var self_1341: MultiVector;
    var other_1155: Scalar;

    self_1341 = self_1340;
    other_1155 = other_1154;
    let _e6: MultiVector = self_1341;
    let _e7: Scalar = other_1155;
    let _e8: MultiVector = multi_vector_scalar_geometric_product(_e6, _e7);
    let _e10: MultiVector = self_1341;
    let _e11: MultiVector = multi_vector_reversal(_e10);
    let _e14: MultiVector = self_1341;
    let _e15: Scalar = other_1155;
    let _e16: MultiVector = multi_vector_scalar_geometric_product(_e14, _e15);
    let _e18: MultiVector = self_1341;
    let _e19: MultiVector = multi_vector_reversal(_e18);
    let _e20: MultiVector = multi_vector_multi_vector_geometric_product(_e16, _e19);
    let _e23: MultiVector = self_1341;
    let _e24: Scalar = other_1155;
    let _e25: MultiVector = multi_vector_scalar_geometric_product(_e23, _e24);
    let _e27: MultiVector = self_1341;
    let _e28: MultiVector = multi_vector_reversal(_e27);
    let _e31: MultiVector = self_1341;
    let _e32: Scalar = other_1155;
    let _e33: MultiVector = multi_vector_scalar_geometric_product(_e31, _e32);
    let _e35: MultiVector = self_1341;
    let _e36: MultiVector = multi_vector_reversal(_e35);
    let _e37: MultiVector = multi_vector_multi_vector_geometric_product(_e33, _e36);
    let _e38: Scalar = multi_vector_scalar_into(_e37);
    return _e38;
}

fn multi_vector_translator_geometric_quotient(self_1342: MultiVector, other_1156: Translator) -> MultiVector {
    var self_1343: MultiVector;
    var other_1157: Translator;

    self_1343 = self_1342;
    other_1157 = other_1156;
    let _e6: Translator = other_1157;
    let _e7: Translator = translator_inverse(_e6);
    let _e8: MultiVector = self_1343;
    let _e10: Translator = other_1157;
    let _e11: Translator = translator_inverse(_e10);
    let _e12: MultiVector = multi_vector_translator_geometric_product(_e8, _e11);
    return _e12;
}

fn multi_vector_translator_transformation(self_1344: MultiVector, other_1158: Translator) -> Translator {
    var self_1345: MultiVector;
    var other_1159: Translator;

    self_1345 = self_1344;
    other_1159 = other_1158;
    let _e6: MultiVector = self_1345;
    let _e7: Translator = other_1159;
    let _e8: MultiVector = multi_vector_translator_geometric_product(_e6, _e7);
    let _e10: MultiVector = self_1345;
    let _e11: MultiVector = multi_vector_reversal(_e10);
    let _e14: MultiVector = self_1345;
    let _e15: Translator = other_1159;
    let _e16: MultiVector = multi_vector_translator_geometric_product(_e14, _e15);
    let _e18: MultiVector = self_1345;
    let _e19: MultiVector = multi_vector_reversal(_e18);
    let _e20: MultiVector = multi_vector_multi_vector_geometric_product(_e16, _e19);
    let _e23: MultiVector = self_1345;
    let _e24: Translator = other_1159;
    let _e25: MultiVector = multi_vector_translator_geometric_product(_e23, _e24);
    let _e27: MultiVector = self_1345;
    let _e28: MultiVector = multi_vector_reversal(_e27);
    let _e31: MultiVector = self_1345;
    let _e32: Translator = other_1159;
    let _e33: MultiVector = multi_vector_translator_geometric_product(_e31, _e32);
    let _e35: MultiVector = self_1345;
    let _e36: MultiVector = multi_vector_reversal(_e35);
    let _e37: MultiVector = multi_vector_multi_vector_geometric_product(_e33, _e36);
    let _e38: Translator = multi_vector_translator_into(_e37);
    return _e38;
}

fn plane_motor_geometric_quotient(self_1346: Plane, other_1160: Motor) -> MotorDual {
    var self_1347: Plane;
    var other_1161: Motor;

    self_1347 = self_1346;
    other_1161 = other_1160;
    let _e6: Motor = other_1161;
    let _e7: Motor = motor_inverse(_e6);
    let _e8: Plane = self_1347;
    let _e10: Motor = other_1161;
    let _e11: Motor = motor_inverse(_e10);
    let _e12: MotorDual = plane_motor_geometric_product(_e8, _e11);
    return _e12;
}

fn plane_motor_transformation(self_1348: Plane, other_1162: Motor) -> Motor {
    var self_1349: Plane;
    var other_1163: Motor;

    self_1349 = self_1348;
    other_1163 = other_1162;
    let _e6: Plane = self_1349;
    let _e7: Motor = other_1163;
    let _e8: MotorDual = plane_motor_geometric_product(_e6, _e7);
    let _e10: Plane = self_1349;
    let _e11: Plane = plane_reversal(_e10);
    let _e14: Plane = self_1349;
    let _e15: Motor = other_1163;
    let _e16: MotorDual = plane_motor_geometric_product(_e14, _e15);
    let _e18: Plane = self_1349;
    let _e19: Plane = plane_reversal(_e18);
    let _e20: Motor = motor_dual_plane_geometric_product(_e16, _e19);
    return _e20;
}

fn plane_motor_dual_geometric_quotient(self_1350: Plane, other_1164: MotorDual) -> Motor {
    var self_1351: Plane;
    var other_1165: MotorDual;

    self_1351 = self_1350;
    other_1165 = other_1164;
    let _e6: MotorDual = other_1165;
    let _e7: MotorDual = motor_dual_inverse(_e6);
    let _e8: Plane = self_1351;
    let _e10: MotorDual = other_1165;
    let _e11: MotorDual = motor_dual_inverse(_e10);
    let _e12: Motor = plane_motor_dual_geometric_product(_e8, _e11);
    return _e12;
}

fn plane_motor_dual_transformation(self_1352: Plane, other_1166: MotorDual) -> MotorDual {
    var self_1353: Plane;
    var other_1167: MotorDual;

    self_1353 = self_1352;
    other_1167 = other_1166;
    let _e6: Plane = self_1353;
    let _e7: MotorDual = other_1167;
    let _e8: Motor = plane_motor_dual_geometric_product(_e6, _e7);
    let _e10: Plane = self_1353;
    let _e11: Plane = plane_reversal(_e10);
    let _e14: Plane = self_1353;
    let _e15: MotorDual = other_1167;
    let _e16: Motor = plane_motor_dual_geometric_product(_e14, _e15);
    let _e18: Plane = self_1353;
    let _e19: Plane = plane_reversal(_e18);
    let _e20: MotorDual = motor_plane_geometric_product(_e16, _e19);
    return _e20;
}

fn plane_multi_vector_geometric_quotient(self_1354: Plane, other_1168: MultiVector) -> MultiVector {
    var self_1355: Plane;
    var other_1169: MultiVector;

    self_1355 = self_1354;
    other_1169 = other_1168;
    let _e6: MultiVector = other_1169;
    let _e7: MultiVector = multi_vector_inverse(_e6);
    let _e8: Plane = self_1355;
    let _e10: MultiVector = other_1169;
    let _e11: MultiVector = multi_vector_inverse(_e10);
    let _e12: MultiVector = plane_multi_vector_geometric_product(_e8, _e11);
    return _e12;
}

fn plane_multi_vector_transformation(self_1356: Plane, other_1170: MultiVector) -> MultiVector {
    var self_1357: Plane;
    var other_1171: MultiVector;

    self_1357 = self_1356;
    other_1171 = other_1170;
    let _e6: Plane = self_1357;
    let _e7: MultiVector = other_1171;
    let _e8: MultiVector = plane_multi_vector_geometric_product(_e6, _e7);
    let _e10: Plane = self_1357;
    let _e11: Plane = plane_reversal(_e10);
    let _e14: Plane = self_1357;
    let _e15: MultiVector = other_1171;
    let _e16: MultiVector = plane_multi_vector_geometric_product(_e14, _e15);
    let _e18: Plane = self_1357;
    let _e19: Plane = plane_reversal(_e18);
    let _e20: MultiVector = multi_vector_plane_geometric_product(_e16, _e19);
    return _e20;
}

fn plane_plane_geometric_quotient(self_1358: Plane, other_1172: Plane) -> Motor {
    var self_1359: Plane;
    var other_1173: Plane;

    self_1359 = self_1358;
    other_1173 = other_1172;
    let _e6: Plane = other_1173;
    let _e7: Plane = plane_inverse(_e6);
    let _e8: Plane = self_1359;
    let _e10: Plane = other_1173;
    let _e11: Plane = plane_inverse(_e10);
    let _e12: Motor = plane_plane_geometric_product(_e8, _e11);
    return _e12;
}

fn plane_plane_transformation(self_1360: Plane, other_1174: Plane) -> Plane {
    var self_1361: Plane;
    var other_1175: Plane;

    self_1361 = self_1360;
    other_1175 = other_1174;
    let _e6: Plane = self_1361;
    let _e7: Plane = other_1175;
    let _e8: Motor = plane_plane_geometric_product(_e6, _e7);
    let _e10: Plane = self_1361;
    let _e11: Plane = plane_reversal(_e10);
    let _e14: Plane = self_1361;
    let _e15: Plane = other_1175;
    let _e16: Motor = plane_plane_geometric_product(_e14, _e15);
    let _e18: Plane = self_1361;
    let _e19: Plane = plane_reversal(_e18);
    let _e20: MotorDual = motor_plane_geometric_product(_e16, _e19);
    let _e23: Plane = self_1361;
    let _e24: Plane = other_1175;
    let _e25: Motor = plane_plane_geometric_product(_e23, _e24);
    let _e27: Plane = self_1361;
    let _e28: Plane = plane_reversal(_e27);
    let _e31: Plane = self_1361;
    let _e32: Plane = other_1175;
    let _e33: Motor = plane_plane_geometric_product(_e31, _e32);
    let _e35: Plane = self_1361;
    let _e36: Plane = plane_reversal(_e35);
    let _e37: MotorDual = motor_plane_geometric_product(_e33, _e36);
    let _e38: Plane = motor_dual_plane_into(_e37);
    return _e38;
}

fn plane_point_geometric_quotient(self_1362: Plane, other_1176: Point) -> MotorDual {
    var self_1363: Plane;
    var other_1177: Point;

    self_1363 = self_1362;
    other_1177 = other_1176;
    let _e6: Point = other_1177;
    let _e7: Point = point_inverse(_e6);
    let _e8: Plane = self_1363;
    let _e10: Point = other_1177;
    let _e11: Point = point_inverse(_e10);
    let _e12: MotorDual = plane_point_geometric_product(_e8, _e11);
    return _e12;
}

fn plane_point_transformation(self_1364: Plane, other_1178: Point) -> Point {
    var self_1365: Plane;
    var other_1179: Point;

    self_1365 = self_1364;
    other_1179 = other_1178;
    let _e6: Plane = self_1365;
    let _e7: Point = other_1179;
    let _e8: MotorDual = plane_point_geometric_product(_e6, _e7);
    let _e10: Plane = self_1365;
    let _e11: Plane = plane_reversal(_e10);
    let _e14: Plane = self_1365;
    let _e15: Point = other_1179;
    let _e16: MotorDual = plane_point_geometric_product(_e14, _e15);
    let _e18: Plane = self_1365;
    let _e19: Plane = plane_reversal(_e18);
    let _e20: Motor = motor_dual_plane_geometric_product(_e16, _e19);
    let _e23: Plane = self_1365;
    let _e24: Point = other_1179;
    let _e25: MotorDual = plane_point_geometric_product(_e23, _e24);
    let _e27: Plane = self_1365;
    let _e28: Plane = plane_reversal(_e27);
    let _e31: Plane = self_1365;
    let _e32: Point = other_1179;
    let _e33: MotorDual = plane_point_geometric_product(_e31, _e32);
    let _e35: Plane = self_1365;
    let _e36: Plane = plane_reversal(_e35);
    let _e37: Motor = motor_dual_plane_geometric_product(_e33, _e36);
    let _e38: Point = motor_point_into(_e37);
    return _e38;
}

fn plane_rotor_geometric_quotient(self_1366: Plane, other_1180: Rotor) -> MotorDual {
    var self_1367: Plane;
    var other_1181: Rotor;

    self_1367 = self_1366;
    other_1181 = other_1180;
    let _e6: Rotor = other_1181;
    let _e7: Rotor = rotor_inverse(_e6);
    let _e8: Plane = self_1367;
    let _e10: Rotor = other_1181;
    let _e11: Rotor = rotor_inverse(_e10);
    let _e12: MotorDual = plane_rotor_geometric_product(_e8, _e11);
    return _e12;
}

fn plane_rotor_transformation(self_1368: Plane, other_1182: Rotor) -> Rotor {
    var self_1369: Plane;
    var other_1183: Rotor;

    self_1369 = self_1368;
    other_1183 = other_1182;
    let _e6: Plane = self_1369;
    let _e7: Rotor = other_1183;
    let _e8: MotorDual = plane_rotor_geometric_product(_e6, _e7);
    let _e10: Plane = self_1369;
    let _e11: Plane = plane_reversal(_e10);
    let _e14: Plane = self_1369;
    let _e15: Rotor = other_1183;
    let _e16: MotorDual = plane_rotor_geometric_product(_e14, _e15);
    let _e18: Plane = self_1369;
    let _e19: Plane = plane_reversal(_e18);
    let _e20: Motor = motor_dual_plane_geometric_product(_e16, _e19);
    let _e23: Plane = self_1369;
    let _e24: Rotor = other_1183;
    let _e25: MotorDual = plane_rotor_geometric_product(_e23, _e24);
    let _e27: Plane = self_1369;
    let _e28: Plane = plane_reversal(_e27);
    let _e31: Plane = self_1369;
    let _e32: Rotor = other_1183;
    let _e33: MotorDual = plane_rotor_geometric_product(_e31, _e32);
    let _e35: Plane = self_1369;
    let _e36: Plane = plane_reversal(_e35);
    let _e37: Motor = motor_dual_plane_geometric_product(_e33, _e36);
    let _e38: Rotor = motor_rotor_into(_e37);
    return _e38;
}

fn plane_scalar_geometric_quotient(self_1370: Plane, other_1184: Scalar) -> Plane {
    var self_1371: Plane;
    var other_1185: Scalar;

    self_1371 = self_1370;
    other_1185 = other_1184;
    let _e6: Scalar = other_1185;
    let _e7: Scalar = scalar_inverse(_e6);
    let _e8: Plane = self_1371;
    let _e10: Scalar = other_1185;
    let _e11: Scalar = scalar_inverse(_e10);
    let _e12: Plane = plane_scalar_geometric_product(_e8, _e11);
    return _e12;
}

fn plane_scalar_transformation(self_1372: Plane, other_1186: Scalar) -> Scalar {
    var self_1373: Plane;
    var other_1187: Scalar;

    self_1373 = self_1372;
    other_1187 = other_1186;
    let _e6: Plane = self_1373;
    let _e7: Scalar = other_1187;
    let _e8: Plane = plane_scalar_geometric_product(_e6, _e7);
    let _e10: Plane = self_1373;
    let _e11: Plane = plane_reversal(_e10);
    let _e14: Plane = self_1373;
    let _e15: Scalar = other_1187;
    let _e16: Plane = plane_scalar_geometric_product(_e14, _e15);
    let _e18: Plane = self_1373;
    let _e19: Plane = plane_reversal(_e18);
    let _e20: Motor = plane_plane_geometric_product(_e16, _e19);
    let _e23: Plane = self_1373;
    let _e24: Scalar = other_1187;
    let _e25: Plane = plane_scalar_geometric_product(_e23, _e24);
    let _e27: Plane = self_1373;
    let _e28: Plane = plane_reversal(_e27);
    let _e31: Plane = self_1373;
    let _e32: Scalar = other_1187;
    let _e33: Plane = plane_scalar_geometric_product(_e31, _e32);
    let _e35: Plane = self_1373;
    let _e36: Plane = plane_reversal(_e35);
    let _e37: Motor = plane_plane_geometric_product(_e33, _e36);
    let _e38: Scalar = motor_scalar_into(_e37);
    return _e38;
}

fn plane_translator_geometric_quotient(self_1374: Plane, other_1188: Translator) -> MotorDual {
    var self_1375: Plane;
    var other_1189: Translator;

    self_1375 = self_1374;
    other_1189 = other_1188;
    let _e6: Translator = other_1189;
    let _e7: Translator = translator_inverse(_e6);
    let _e8: Plane = self_1375;
    let _e10: Translator = other_1189;
    let _e11: Translator = translator_inverse(_e10);
    let _e12: MotorDual = plane_translator_geometric_product(_e8, _e11);
    return _e12;
}

fn plane_translator_transformation(self_1376: Plane, other_1190: Translator) -> Translator {
    var self_1377: Plane;
    var other_1191: Translator;

    self_1377 = self_1376;
    other_1191 = other_1190;
    let _e6: Plane = self_1377;
    let _e7: Translator = other_1191;
    let _e8: MotorDual = plane_translator_geometric_product(_e6, _e7);
    let _e10: Plane = self_1377;
    let _e11: Plane = plane_reversal(_e10);
    let _e14: Plane = self_1377;
    let _e15: Translator = other_1191;
    let _e16: MotorDual = plane_translator_geometric_product(_e14, _e15);
    let _e18: Plane = self_1377;
    let _e19: Plane = plane_reversal(_e18);
    let _e20: Motor = motor_dual_plane_geometric_product(_e16, _e19);
    let _e23: Plane = self_1377;
    let _e24: Translator = other_1191;
    let _e25: MotorDual = plane_translator_geometric_product(_e23, _e24);
    let _e27: Plane = self_1377;
    let _e28: Plane = plane_reversal(_e27);
    let _e31: Plane = self_1377;
    let _e32: Translator = other_1191;
    let _e33: MotorDual = plane_translator_geometric_product(_e31, _e32);
    let _e35: Plane = self_1377;
    let _e36: Plane = plane_reversal(_e35);
    let _e37: Motor = motor_dual_plane_geometric_product(_e33, _e36);
    let _e38: Translator = motor_translator_into(_e37);
    return _e38;
}

fn point_ideal_point_transformation(self_1378: Point, other_1192: IdealPoint) -> IdealPoint {
    var self_1379: Point;
    var other_1193: IdealPoint;

    self_1379 = self_1378;
    other_1193 = other_1192;
    let _e6: Point = self_1379;
    let _e7: IdealPoint = other_1193;
    let _e8: IdealPoint = point_ideal_point_geometric_product(_e6, _e7);
    let _e10: Point = self_1379;
    let _e11: Point = point_reversal(_e10);
    let _e14: Point = self_1379;
    let _e15: IdealPoint = other_1193;
    let _e16: IdealPoint = point_ideal_point_geometric_product(_e14, _e15);
    let _e18: Point = self_1379;
    let _e19: Point = point_reversal(_e18);
    let _e20: IdealPoint = ideal_point_point_geometric_product(_e16, _e19);
    return _e20;
}

fn point_motor_geometric_quotient(self_1380: Point, other_1194: Motor) -> Motor {
    var self_1381: Point;
    var other_1195: Motor;

    self_1381 = self_1380;
    other_1195 = other_1194;
    let _e6: Motor = other_1195;
    let _e7: Motor = motor_inverse(_e6);
    let _e8: Point = self_1381;
    let _e10: Motor = other_1195;
    let _e11: Motor = motor_inverse(_e10);
    let _e12: Motor = point_motor_geometric_product(_e8, _e11);
    return _e12;
}

fn point_motor_transformation(self_1382: Point, other_1196: Motor) -> Motor {
    var self_1383: Point;
    var other_1197: Motor;

    self_1383 = self_1382;
    other_1197 = other_1196;
    let _e6: Point = self_1383;
    let _e7: Motor = other_1197;
    let _e8: Motor = point_motor_geometric_product(_e6, _e7);
    let _e10: Point = self_1383;
    let _e11: Point = point_reversal(_e10);
    let _e14: Point = self_1383;
    let _e15: Motor = other_1197;
    let _e16: Motor = point_motor_geometric_product(_e14, _e15);
    let _e18: Point = self_1383;
    let _e19: Point = point_reversal(_e18);
    let _e20: Motor = motor_point_geometric_product(_e16, _e19);
    return _e20;
}

fn point_motor_dual_geometric_quotient(self_1384: Point, other_1198: MotorDual) -> MotorDual {
    var self_1385: Point;
    var other_1199: MotorDual;

    self_1385 = self_1384;
    other_1199 = other_1198;
    let _e6: MotorDual = other_1199;
    let _e7: MotorDual = motor_dual_inverse(_e6);
    let _e8: Point = self_1385;
    let _e10: MotorDual = other_1199;
    let _e11: MotorDual = motor_dual_inverse(_e10);
    let _e12: MotorDual = point_motor_dual_geometric_product(_e8, _e11);
    return _e12;
}

fn point_motor_dual_transformation(self_1386: Point, other_1200: MotorDual) -> MotorDual {
    var self_1387: Point;
    var other_1201: MotorDual;

    self_1387 = self_1386;
    other_1201 = other_1200;
    let _e6: Point = self_1387;
    let _e7: MotorDual = other_1201;
    let _e8: MotorDual = point_motor_dual_geometric_product(_e6, _e7);
    let _e10: Point = self_1387;
    let _e11: Point = point_reversal(_e10);
    let _e14: Point = self_1387;
    let _e15: MotorDual = other_1201;
    let _e16: MotorDual = point_motor_dual_geometric_product(_e14, _e15);
    let _e18: Point = self_1387;
    let _e19: Point = point_reversal(_e18);
    let _e20: MotorDual = motor_dual_point_geometric_product(_e16, _e19);
    return _e20;
}

fn point_multi_vector_geometric_quotient(self_1388: Point, other_1202: MultiVector) -> MultiVector {
    var self_1389: Point;
    var other_1203: MultiVector;

    self_1389 = self_1388;
    other_1203 = other_1202;
    let _e6: MultiVector = other_1203;
    let _e7: MultiVector = multi_vector_inverse(_e6);
    let _e8: Point = self_1389;
    let _e10: MultiVector = other_1203;
    let _e11: MultiVector = multi_vector_inverse(_e10);
    let _e12: MultiVector = point_multi_vector_geometric_product(_e8, _e11);
    return _e12;
}

fn point_multi_vector_transformation(self_1390: Point, other_1204: MultiVector) -> MultiVector {
    var self_1391: Point;
    var other_1205: MultiVector;

    self_1391 = self_1390;
    other_1205 = other_1204;
    let _e6: Point = self_1391;
    let _e7: MultiVector = other_1205;
    let _e8: MultiVector = point_multi_vector_geometric_product(_e6, _e7);
    let _e10: Point = self_1391;
    let _e11: Point = point_reversal(_e10);
    let _e14: Point = self_1391;
    let _e15: MultiVector = other_1205;
    let _e16: MultiVector = point_multi_vector_geometric_product(_e14, _e15);
    let _e18: Point = self_1391;
    let _e19: Point = point_reversal(_e18);
    let _e20: MultiVector = multi_vector_point_geometric_product(_e16, _e19);
    return _e20;
}

fn point_plane_geometric_quotient(self_1392: Point, other_1206: Plane) -> MotorDual {
    var self_1393: Point;
    var other_1207: Plane;

    self_1393 = self_1392;
    other_1207 = other_1206;
    let _e6: Plane = other_1207;
    let _e7: Plane = plane_inverse(_e6);
    let _e8: Point = self_1393;
    let _e10: Plane = other_1207;
    let _e11: Plane = plane_inverse(_e10);
    let _e12: MotorDual = point_plane_geometric_product(_e8, _e11);
    return _e12;
}

fn point_plane_transformation(self_1394: Point, other_1208: Plane) -> Plane {
    var self_1395: Point;
    var other_1209: Plane;

    self_1395 = self_1394;
    other_1209 = other_1208;
    let _e6: Point = self_1395;
    let _e7: Plane = other_1209;
    let _e8: MotorDual = point_plane_geometric_product(_e6, _e7);
    let _e10: Point = self_1395;
    let _e11: Point = point_reversal(_e10);
    let _e14: Point = self_1395;
    let _e15: Plane = other_1209;
    let _e16: MotorDual = point_plane_geometric_product(_e14, _e15);
    let _e18: Point = self_1395;
    let _e19: Point = point_reversal(_e18);
    let _e20: MotorDual = motor_dual_point_geometric_product(_e16, _e19);
    let _e23: Point = self_1395;
    let _e24: Plane = other_1209;
    let _e25: MotorDual = point_plane_geometric_product(_e23, _e24);
    let _e27: Point = self_1395;
    let _e28: Point = point_reversal(_e27);
    let _e31: Point = self_1395;
    let _e32: Plane = other_1209;
    let _e33: MotorDual = point_plane_geometric_product(_e31, _e32);
    let _e35: Point = self_1395;
    let _e36: Point = point_reversal(_e35);
    let _e37: MotorDual = motor_dual_point_geometric_product(_e33, _e36);
    let _e38: Plane = motor_dual_plane_into(_e37);
    return _e38;
}

fn point_point_geometric_quotient(self_1396: Point, other_1210: Point) -> Translator {
    var self_1397: Point;
    var other_1211: Point;

    self_1397 = self_1396;
    other_1211 = other_1210;
    let _e6: Point = other_1211;
    let _e7: Point = point_inverse(_e6);
    let _e8: Point = self_1397;
    let _e10: Point = other_1211;
    let _e11: Point = point_inverse(_e10);
    let _e12: Translator = point_point_geometric_product(_e8, _e11);
    return _e12;
}

fn point_point_transformation(self_1398: Point, other_1212: Point) -> Point {
    var self_1399: Point;
    var other_1213: Point;

    self_1399 = self_1398;
    other_1213 = other_1212;
    let _e6: Point = self_1399;
    let _e7: Point = other_1213;
    let _e8: Translator = point_point_geometric_product(_e6, _e7);
    let _e10: Point = self_1399;
    let _e11: Point = point_reversal(_e10);
    let _e14: Point = self_1399;
    let _e15: Point = other_1213;
    let _e16: Translator = point_point_geometric_product(_e14, _e15);
    let _e18: Point = self_1399;
    let _e19: Point = point_reversal(_e18);
    let _e20: Point = translator_point_geometric_product(_e16, _e19);
    return _e20;
}

fn point_rotor_geometric_quotient(self_1400: Point, other_1214: Rotor) -> Motor {
    var self_1401: Point;
    var other_1215: Rotor;

    self_1401 = self_1400;
    other_1215 = other_1214;
    let _e6: Rotor = other_1215;
    let _e7: Rotor = rotor_inverse(_e6);
    let _e8: Point = self_1401;
    let _e10: Rotor = other_1215;
    let _e11: Rotor = rotor_inverse(_e10);
    let _e12: Motor = point_rotor_geometric_product(_e8, _e11);
    return _e12;
}

fn point_rotor_transformation(self_1402: Point, other_1216: Rotor) -> Rotor {
    var self_1403: Point;
    var other_1217: Rotor;

    self_1403 = self_1402;
    other_1217 = other_1216;
    let _e6: Point = self_1403;
    let _e7: Rotor = other_1217;
    let _e8: Motor = point_rotor_geometric_product(_e6, _e7);
    let _e10: Point = self_1403;
    let _e11: Point = point_reversal(_e10);
    let _e14: Point = self_1403;
    let _e15: Rotor = other_1217;
    let _e16: Motor = point_rotor_geometric_product(_e14, _e15);
    let _e18: Point = self_1403;
    let _e19: Point = point_reversal(_e18);
    let _e20: Motor = motor_point_geometric_product(_e16, _e19);
    let _e23: Point = self_1403;
    let _e24: Rotor = other_1217;
    let _e25: Motor = point_rotor_geometric_product(_e23, _e24);
    let _e27: Point = self_1403;
    let _e28: Point = point_reversal(_e27);
    let _e31: Point = self_1403;
    let _e32: Rotor = other_1217;
    let _e33: Motor = point_rotor_geometric_product(_e31, _e32);
    let _e35: Point = self_1403;
    let _e36: Point = point_reversal(_e35);
    let _e37: Motor = motor_point_geometric_product(_e33, _e36);
    let _e38: Rotor = motor_rotor_into(_e37);
    return _e38;
}

fn point_scalar_geometric_quotient(self_1404: Point, other_1218: Scalar) -> Point {
    var self_1405: Point;
    var other_1219: Scalar;

    self_1405 = self_1404;
    other_1219 = other_1218;
    let _e6: Scalar = other_1219;
    let _e7: Scalar = scalar_inverse(_e6);
    let _e8: Point = self_1405;
    let _e10: Scalar = other_1219;
    let _e11: Scalar = scalar_inverse(_e10);
    let _e12: Point = point_scalar_geometric_product(_e8, _e11);
    return _e12;
}

fn point_scalar_transformation(self_1406: Point, other_1220: Scalar) -> Scalar {
    var self_1407: Point;
    var other_1221: Scalar;

    self_1407 = self_1406;
    other_1221 = other_1220;
    let _e6: Point = self_1407;
    let _e7: Scalar = other_1221;
    let _e8: Point = point_scalar_geometric_product(_e6, _e7);
    let _e10: Point = self_1407;
    let _e11: Point = point_reversal(_e10);
    let _e14: Point = self_1407;
    let _e15: Scalar = other_1221;
    let _e16: Point = point_scalar_geometric_product(_e14, _e15);
    let _e18: Point = self_1407;
    let _e19: Point = point_reversal(_e18);
    let _e20: Translator = point_point_geometric_product(_e16, _e19);
    let _e23: Point = self_1407;
    let _e24: Scalar = other_1221;
    let _e25: Point = point_scalar_geometric_product(_e23, _e24);
    let _e27: Point = self_1407;
    let _e28: Point = point_reversal(_e27);
    let _e31: Point = self_1407;
    let _e32: Scalar = other_1221;
    let _e33: Point = point_scalar_geometric_product(_e31, _e32);
    let _e35: Point = self_1407;
    let _e36: Point = point_reversal(_e35);
    let _e37: Translator = point_point_geometric_product(_e33, _e36);
    let _e38: Scalar = translator_scalar_into(_e37);
    return _e38;
}

fn point_translator_geometric_quotient(self_1408: Point, other_1222: Translator) -> Point {
    var self_1409: Point;
    var other_1223: Translator;

    self_1409 = self_1408;
    other_1223 = other_1222;
    let _e6: Translator = other_1223;
    let _e7: Translator = translator_inverse(_e6);
    let _e8: Point = self_1409;
    let _e10: Translator = other_1223;
    let _e11: Translator = translator_inverse(_e10);
    let _e12: Point = point_translator_geometric_product(_e8, _e11);
    return _e12;
}

fn point_translator_transformation(self_1410: Point, other_1224: Translator) -> Translator {
    var self_1411: Point;
    var other_1225: Translator;

    self_1411 = self_1410;
    other_1225 = other_1224;
    let _e6: Point = self_1411;
    let _e7: Translator = other_1225;
    let _e8: Point = point_translator_geometric_product(_e6, _e7);
    let _e10: Point = self_1411;
    let _e11: Point = point_reversal(_e10);
    let _e14: Point = self_1411;
    let _e15: Translator = other_1225;
    let _e16: Point = point_translator_geometric_product(_e14, _e15);
    let _e18: Point = self_1411;
    let _e19: Point = point_reversal(_e18);
    let _e20: Translator = point_point_geometric_product(_e16, _e19);
    return _e20;
}

fn rotor_ideal_point_transformation(self_1412: Rotor, other_1226: IdealPoint) -> IdealPoint {
    var self_1413: Rotor;
    var other_1227: IdealPoint;

    self_1413 = self_1412;
    other_1227 = other_1226;
    let _e6: Rotor = self_1413;
    let _e7: IdealPoint = other_1227;
    let _e8: IdealPoint = rotor_ideal_point_geometric_product(_e6, _e7);
    let _e10: Rotor = self_1413;
    let _e11: Rotor = rotor_reversal(_e10);
    let _e14: Rotor = self_1413;
    let _e15: IdealPoint = other_1227;
    let _e16: IdealPoint = rotor_ideal_point_geometric_product(_e14, _e15);
    let _e18: Rotor = self_1413;
    let _e19: Rotor = rotor_reversal(_e18);
    let _e20: IdealPoint = ideal_point_rotor_geometric_product(_e16, _e19);
    return _e20;
}

fn rotor_motor_geometric_quotient(self_1414: Rotor, other_1228: Motor) -> Motor {
    var self_1415: Rotor;
    var other_1229: Motor;

    self_1415 = self_1414;
    other_1229 = other_1228;
    let _e6: Motor = other_1229;
    let _e7: Motor = motor_inverse(_e6);
    let _e8: Rotor = self_1415;
    let _e10: Motor = other_1229;
    let _e11: Motor = motor_inverse(_e10);
    let _e12: Motor = rotor_motor_geometric_product(_e8, _e11);
    return _e12;
}

fn rotor_motor_transformation(self_1416: Rotor, other_1230: Motor) -> Motor {
    var self_1417: Rotor;
    var other_1231: Motor;

    self_1417 = self_1416;
    other_1231 = other_1230;
    let _e6: Rotor = self_1417;
    let _e7: Motor = other_1231;
    let _e8: Motor = rotor_motor_geometric_product(_e6, _e7);
    let _e10: Rotor = self_1417;
    let _e11: Rotor = rotor_reversal(_e10);
    let _e14: Rotor = self_1417;
    let _e15: Motor = other_1231;
    let _e16: Motor = rotor_motor_geometric_product(_e14, _e15);
    let _e18: Rotor = self_1417;
    let _e19: Rotor = rotor_reversal(_e18);
    let _e20: Motor = motor_rotor_geometric_product(_e16, _e19);
    return _e20;
}

fn rotor_motor_dual_geometric_quotient(self_1418: Rotor, other_1232: MotorDual) -> MotorDual {
    var self_1419: Rotor;
    var other_1233: MotorDual;

    self_1419 = self_1418;
    other_1233 = other_1232;
    let _e6: MotorDual = other_1233;
    let _e7: MotorDual = motor_dual_inverse(_e6);
    let _e8: Rotor = self_1419;
    let _e10: MotorDual = other_1233;
    let _e11: MotorDual = motor_dual_inverse(_e10);
    let _e12: MotorDual = rotor_motor_dual_geometric_product(_e8, _e11);
    return _e12;
}

fn rotor_motor_dual_transformation(self_1420: Rotor, other_1234: MotorDual) -> MotorDual {
    var self_1421: Rotor;
    var other_1235: MotorDual;

    self_1421 = self_1420;
    other_1235 = other_1234;
    let _e6: Rotor = self_1421;
    let _e7: MotorDual = other_1235;
    let _e8: MotorDual = rotor_motor_dual_geometric_product(_e6, _e7);
    let _e10: Rotor = self_1421;
    let _e11: Rotor = rotor_reversal(_e10);
    let _e14: Rotor = self_1421;
    let _e15: MotorDual = other_1235;
    let _e16: MotorDual = rotor_motor_dual_geometric_product(_e14, _e15);
    let _e18: Rotor = self_1421;
    let _e19: Rotor = rotor_reversal(_e18);
    let _e20: MotorDual = motor_dual_rotor_geometric_product(_e16, _e19);
    return _e20;
}

fn rotor_multi_vector_geometric_quotient(self_1422: Rotor, other_1236: MultiVector) -> MultiVector {
    var self_1423: Rotor;
    var other_1237: MultiVector;

    self_1423 = self_1422;
    other_1237 = other_1236;
    let _e6: MultiVector = other_1237;
    let _e7: MultiVector = multi_vector_inverse(_e6);
    let _e8: Rotor = self_1423;
    let _e10: MultiVector = other_1237;
    let _e11: MultiVector = multi_vector_inverse(_e10);
    let _e12: MultiVector = rotor_multi_vector_geometric_product(_e8, _e11);
    return _e12;
}

fn rotor_multi_vector_transformation(self_1424: Rotor, other_1238: MultiVector) -> MultiVector {
    var self_1425: Rotor;
    var other_1239: MultiVector;

    self_1425 = self_1424;
    other_1239 = other_1238;
    let _e6: Rotor = self_1425;
    let _e7: MultiVector = other_1239;
    let _e8: MultiVector = rotor_multi_vector_geometric_product(_e6, _e7);
    let _e10: Rotor = self_1425;
    let _e11: Rotor = rotor_reversal(_e10);
    let _e14: Rotor = self_1425;
    let _e15: MultiVector = other_1239;
    let _e16: MultiVector = rotor_multi_vector_geometric_product(_e14, _e15);
    let _e18: Rotor = self_1425;
    let _e19: Rotor = rotor_reversal(_e18);
    let _e20: MultiVector = multi_vector_rotor_geometric_product(_e16, _e19);
    return _e20;
}

fn rotor_plane_geometric_quotient(self_1426: Rotor, other_1240: Plane) -> MotorDual {
    var self_1427: Rotor;
    var other_1241: Plane;

    self_1427 = self_1426;
    other_1241 = other_1240;
    let _e6: Plane = other_1241;
    let _e7: Plane = plane_inverse(_e6);
    let _e8: Rotor = self_1427;
    let _e10: Plane = other_1241;
    let _e11: Plane = plane_inverse(_e10);
    let _e12: MotorDual = rotor_plane_geometric_product(_e8, _e11);
    return _e12;
}

fn rotor_plane_transformation(self_1428: Rotor, other_1242: Plane) -> Plane {
    var self_1429: Rotor;
    var other_1243: Plane;

    self_1429 = self_1428;
    other_1243 = other_1242;
    let _e6: Rotor = self_1429;
    let _e7: Plane = other_1243;
    let _e8: MotorDual = rotor_plane_geometric_product(_e6, _e7);
    let _e10: Rotor = self_1429;
    let _e11: Rotor = rotor_reversal(_e10);
    let _e14: Rotor = self_1429;
    let _e15: Plane = other_1243;
    let _e16: MotorDual = rotor_plane_geometric_product(_e14, _e15);
    let _e18: Rotor = self_1429;
    let _e19: Rotor = rotor_reversal(_e18);
    let _e20: MotorDual = motor_dual_rotor_geometric_product(_e16, _e19);
    let _e23: Rotor = self_1429;
    let _e24: Plane = other_1243;
    let _e25: MotorDual = rotor_plane_geometric_product(_e23, _e24);
    let _e27: Rotor = self_1429;
    let _e28: Rotor = rotor_reversal(_e27);
    let _e31: Rotor = self_1429;
    let _e32: Plane = other_1243;
    let _e33: MotorDual = rotor_plane_geometric_product(_e31, _e32);
    let _e35: Rotor = self_1429;
    let _e36: Rotor = rotor_reversal(_e35);
    let _e37: MotorDual = motor_dual_rotor_geometric_product(_e33, _e36);
    let _e38: Plane = motor_dual_plane_into(_e37);
    return _e38;
}

fn rotor_point_geometric_quotient(self_1430: Rotor, other_1244: Point) -> Motor {
    var self_1431: Rotor;
    var other_1245: Point;

    self_1431 = self_1430;
    other_1245 = other_1244;
    let _e6: Point = other_1245;
    let _e7: Point = point_inverse(_e6);
    let _e8: Rotor = self_1431;
    let _e10: Point = other_1245;
    let _e11: Point = point_inverse(_e10);
    let _e12: Motor = rotor_point_geometric_product(_e8, _e11);
    return _e12;
}

fn rotor_point_transformation(self_1432: Rotor, other_1246: Point) -> Point {
    var self_1433: Rotor;
    var other_1247: Point;

    self_1433 = self_1432;
    other_1247 = other_1246;
    let _e6: Rotor = self_1433;
    let _e7: Point = other_1247;
    let _e8: Motor = rotor_point_geometric_product(_e6, _e7);
    let _e10: Rotor = self_1433;
    let _e11: Rotor = rotor_reversal(_e10);
    let _e14: Rotor = self_1433;
    let _e15: Point = other_1247;
    let _e16: Motor = rotor_point_geometric_product(_e14, _e15);
    let _e18: Rotor = self_1433;
    let _e19: Rotor = rotor_reversal(_e18);
    let _e20: Motor = motor_rotor_geometric_product(_e16, _e19);
    let _e23: Rotor = self_1433;
    let _e24: Point = other_1247;
    let _e25: Motor = rotor_point_geometric_product(_e23, _e24);
    let _e27: Rotor = self_1433;
    let _e28: Rotor = rotor_reversal(_e27);
    let _e31: Rotor = self_1433;
    let _e32: Point = other_1247;
    let _e33: Motor = rotor_point_geometric_product(_e31, _e32);
    let _e35: Rotor = self_1433;
    let _e36: Rotor = rotor_reversal(_e35);
    let _e37: Motor = motor_rotor_geometric_product(_e33, _e36);
    let _e38: Point = motor_point_into(_e37);
    return _e38;
}

fn rotor_powi(self_1434: Rotor, exponent_4: i32) -> Rotor {
    var self_1435: Rotor;
    var exponent_5: i32;
    var local_2: Rotor;
    var x_2: Rotor;
    var y_2: Rotor;
    var n_2: i32;

    self_1435 = self_1434;
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
        let _e12: Rotor = self_1435;
        let _e13: Rotor = rotor_inverse(_e12);
        local_2 = _e13;
    } else {
        let _e14: Rotor = self_1435;
        local_2 = _e14;
    }
    let _e16: Rotor = local_2;
    x_2 = _e16;
    let _e18: Rotor = rotor_one();
    y_2 = _e18;
    let _e21: i32 = exponent_5;
    n_2 = abs(_e21);
    loop {
        let _e25: i32 = n_2;
        if !((1 < _e25)) {
            break;
        }
        {
            let _e28: i32 = n_2;
            if ((_e28 & 1) == 1) {
                {
                    let _e35: Rotor = x_2;
                    let _e36: Rotor = y_2;
                    let _e37: Rotor = rotor_rotor_geometric_product(_e35, _e36);
                    y_2 = _e37;
                }
            }
            let _e40: Rotor = x_2;
            let _e41: Rotor = x_2;
            let _e42: Rotor = rotor_rotor_geometric_product(_e40, _e41);
            x_2 = _e42;
            let _e43: i32 = n_2;
            n_2 = (_e43 >> u32(1));
        }
    }
    let _e49: Rotor = x_2;
    let _e50: Rotor = y_2;
    let _e51: Rotor = rotor_rotor_geometric_product(_e49, _e50);
    return _e51;
}

fn rotor_rotor_geometric_quotient(self_1436: Rotor, other_1248: Rotor) -> Rotor {
    var self_1437: Rotor;
    var other_1249: Rotor;

    self_1437 = self_1436;
    other_1249 = other_1248;
    let _e6: Rotor = other_1249;
    let _e7: Rotor = rotor_inverse(_e6);
    let _e8: Rotor = self_1437;
    let _e10: Rotor = other_1249;
    let _e11: Rotor = rotor_inverse(_e10);
    let _e12: Rotor = rotor_rotor_geometric_product(_e8, _e11);
    return _e12;
}

fn rotor_rotor_transformation(self_1438: Rotor, other_1250: Rotor) -> Rotor {
    var self_1439: Rotor;
    var other_1251: Rotor;

    self_1439 = self_1438;
    other_1251 = other_1250;
    let _e6: Rotor = self_1439;
    let _e7: Rotor = other_1251;
    let _e8: Rotor = rotor_rotor_geometric_product(_e6, _e7);
    let _e10: Rotor = self_1439;
    let _e11: Rotor = rotor_reversal(_e10);
    let _e14: Rotor = self_1439;
    let _e15: Rotor = other_1251;
    let _e16: Rotor = rotor_rotor_geometric_product(_e14, _e15);
    let _e18: Rotor = self_1439;
    let _e19: Rotor = rotor_reversal(_e18);
    let _e20: Rotor = rotor_rotor_geometric_product(_e16, _e19);
    return _e20;
}

fn rotor_scalar_geometric_quotient(self_1440: Rotor, other_1252: Scalar) -> Rotor {
    var self_1441: Rotor;
    var other_1253: Scalar;

    self_1441 = self_1440;
    other_1253 = other_1252;
    let _e6: Scalar = other_1253;
    let _e7: Scalar = scalar_inverse(_e6);
    let _e8: Rotor = self_1441;
    let _e10: Scalar = other_1253;
    let _e11: Scalar = scalar_inverse(_e10);
    let _e12: Rotor = rotor_scalar_geometric_product(_e8, _e11);
    return _e12;
}

fn rotor_scalar_transformation(self_1442: Rotor, other_1254: Scalar) -> Scalar {
    var self_1443: Rotor;
    var other_1255: Scalar;

    self_1443 = self_1442;
    other_1255 = other_1254;
    let _e6: Rotor = self_1443;
    let _e7: Scalar = other_1255;
    let _e8: Rotor = rotor_scalar_geometric_product(_e6, _e7);
    let _e10: Rotor = self_1443;
    let _e11: Rotor = rotor_reversal(_e10);
    let _e14: Rotor = self_1443;
    let _e15: Scalar = other_1255;
    let _e16: Rotor = rotor_scalar_geometric_product(_e14, _e15);
    let _e18: Rotor = self_1443;
    let _e19: Rotor = rotor_reversal(_e18);
    let _e20: Rotor = rotor_rotor_geometric_product(_e16, _e19);
    let _e23: Rotor = self_1443;
    let _e24: Scalar = other_1255;
    let _e25: Rotor = rotor_scalar_geometric_product(_e23, _e24);
    let _e27: Rotor = self_1443;
    let _e28: Rotor = rotor_reversal(_e27);
    let _e31: Rotor = self_1443;
    let _e32: Scalar = other_1255;
    let _e33: Rotor = rotor_scalar_geometric_product(_e31, _e32);
    let _e35: Rotor = self_1443;
    let _e36: Rotor = rotor_reversal(_e35);
    let _e37: Rotor = rotor_rotor_geometric_product(_e33, _e36);
    let _e38: Scalar = rotor_scalar_into(_e37);
    return _e38;
}

fn rotor_translator_geometric_quotient(self_1444: Rotor, other_1256: Translator) -> Motor {
    var self_1445: Rotor;
    var other_1257: Translator;

    self_1445 = self_1444;
    other_1257 = other_1256;
    let _e6: Translator = other_1257;
    let _e7: Translator = translator_inverse(_e6);
    let _e8: Rotor = self_1445;
    let _e10: Translator = other_1257;
    let _e11: Translator = translator_inverse(_e10);
    let _e12: Motor = rotor_translator_geometric_product(_e8, _e11);
    return _e12;
}

fn rotor_translator_transformation(self_1446: Rotor, other_1258: Translator) -> Translator {
    var self_1447: Rotor;
    var other_1259: Translator;

    self_1447 = self_1446;
    other_1259 = other_1258;
    let _e6: Rotor = self_1447;
    let _e7: Translator = other_1259;
    let _e8: Motor = rotor_translator_geometric_product(_e6, _e7);
    let _e10: Rotor = self_1447;
    let _e11: Rotor = rotor_reversal(_e10);
    let _e14: Rotor = self_1447;
    let _e15: Translator = other_1259;
    let _e16: Motor = rotor_translator_geometric_product(_e14, _e15);
    let _e18: Rotor = self_1447;
    let _e19: Rotor = rotor_reversal(_e18);
    let _e20: Motor = motor_rotor_geometric_product(_e16, _e19);
    let _e23: Rotor = self_1447;
    let _e24: Translator = other_1259;
    let _e25: Motor = rotor_translator_geometric_product(_e23, _e24);
    let _e27: Rotor = self_1447;
    let _e28: Rotor = rotor_reversal(_e27);
    let _e31: Rotor = self_1447;
    let _e32: Translator = other_1259;
    let _e33: Motor = rotor_translator_geometric_product(_e31, _e32);
    let _e35: Rotor = self_1447;
    let _e36: Rotor = rotor_reversal(_e35);
    let _e37: Motor = motor_rotor_geometric_product(_e33, _e36);
    let _e38: Translator = motor_translator_into(_e37);
    return _e38;
}

fn scalar_ideal_point_transformation(self_1448: Scalar, other_1260: IdealPoint) -> IdealPoint {
    var self_1449: Scalar;
    var other_1261: IdealPoint;

    self_1449 = self_1448;
    other_1261 = other_1260;
    let _e6: Scalar = self_1449;
    let _e7: IdealPoint = other_1261;
    let _e8: IdealPoint = scalar_ideal_point_geometric_product(_e6, _e7);
    let _e10: Scalar = self_1449;
    let _e11: Scalar = scalar_reversal(_e10);
    let _e14: Scalar = self_1449;
    let _e15: IdealPoint = other_1261;
    let _e16: IdealPoint = scalar_ideal_point_geometric_product(_e14, _e15);
    let _e18: Scalar = self_1449;
    let _e19: Scalar = scalar_reversal(_e18);
    let _e20: IdealPoint = ideal_point_scalar_geometric_product(_e16, _e19);
    return _e20;
}

fn scalar_motor_geometric_quotient(self_1450: Scalar, other_1262: Motor) -> Motor {
    var self_1451: Scalar;
    var other_1263: Motor;

    self_1451 = self_1450;
    other_1263 = other_1262;
    let _e6: Motor = other_1263;
    let _e7: Motor = motor_inverse(_e6);
    let _e8: Scalar = self_1451;
    let _e10: Motor = other_1263;
    let _e11: Motor = motor_inverse(_e10);
    let _e12: Motor = scalar_motor_geometric_product(_e8, _e11);
    return _e12;
}

fn scalar_motor_transformation(self_1452: Scalar, other_1264: Motor) -> Motor {
    var self_1453: Scalar;
    var other_1265: Motor;

    self_1453 = self_1452;
    other_1265 = other_1264;
    let _e6: Scalar = self_1453;
    let _e7: Motor = other_1265;
    let _e8: Motor = scalar_motor_geometric_product(_e6, _e7);
    let _e10: Scalar = self_1453;
    let _e11: Scalar = scalar_reversal(_e10);
    let _e14: Scalar = self_1453;
    let _e15: Motor = other_1265;
    let _e16: Motor = scalar_motor_geometric_product(_e14, _e15);
    let _e18: Scalar = self_1453;
    let _e19: Scalar = scalar_reversal(_e18);
    let _e20: Motor = motor_scalar_geometric_product(_e16, _e19);
    return _e20;
}

fn scalar_motor_dual_geometric_quotient(self_1454: Scalar, other_1266: MotorDual) -> MotorDual {
    var self_1455: Scalar;
    var other_1267: MotorDual;

    self_1455 = self_1454;
    other_1267 = other_1266;
    let _e6: MotorDual = other_1267;
    let _e7: MotorDual = motor_dual_inverse(_e6);
    let _e8: Scalar = self_1455;
    let _e10: MotorDual = other_1267;
    let _e11: MotorDual = motor_dual_inverse(_e10);
    let _e12: MotorDual = scalar_motor_dual_geometric_product(_e8, _e11);
    return _e12;
}

fn scalar_motor_dual_transformation(self_1456: Scalar, other_1268: MotorDual) -> MotorDual {
    var self_1457: Scalar;
    var other_1269: MotorDual;

    self_1457 = self_1456;
    other_1269 = other_1268;
    let _e6: Scalar = self_1457;
    let _e7: MotorDual = other_1269;
    let _e8: MotorDual = scalar_motor_dual_geometric_product(_e6, _e7);
    let _e10: Scalar = self_1457;
    let _e11: Scalar = scalar_reversal(_e10);
    let _e14: Scalar = self_1457;
    let _e15: MotorDual = other_1269;
    let _e16: MotorDual = scalar_motor_dual_geometric_product(_e14, _e15);
    let _e18: Scalar = self_1457;
    let _e19: Scalar = scalar_reversal(_e18);
    let _e20: MotorDual = motor_dual_scalar_geometric_product(_e16, _e19);
    return _e20;
}

fn scalar_multi_vector_geometric_quotient(self_1458: Scalar, other_1270: MultiVector) -> MultiVector {
    var self_1459: Scalar;
    var other_1271: MultiVector;

    self_1459 = self_1458;
    other_1271 = other_1270;
    let _e6: MultiVector = other_1271;
    let _e7: MultiVector = multi_vector_inverse(_e6);
    let _e8: Scalar = self_1459;
    let _e10: MultiVector = other_1271;
    let _e11: MultiVector = multi_vector_inverse(_e10);
    let _e12: MultiVector = scalar_multi_vector_geometric_product(_e8, _e11);
    return _e12;
}

fn scalar_multi_vector_transformation(self_1460: Scalar, other_1272: MultiVector) -> MultiVector {
    var self_1461: Scalar;
    var other_1273: MultiVector;

    self_1461 = self_1460;
    other_1273 = other_1272;
    let _e6: Scalar = self_1461;
    let _e7: MultiVector = other_1273;
    let _e8: MultiVector = scalar_multi_vector_geometric_product(_e6, _e7);
    let _e10: Scalar = self_1461;
    let _e11: Scalar = scalar_reversal(_e10);
    let _e14: Scalar = self_1461;
    let _e15: MultiVector = other_1273;
    let _e16: MultiVector = scalar_multi_vector_geometric_product(_e14, _e15);
    let _e18: Scalar = self_1461;
    let _e19: Scalar = scalar_reversal(_e18);
    let _e20: MultiVector = multi_vector_scalar_geometric_product(_e16, _e19);
    return _e20;
}

fn scalar_plane_geometric_quotient(self_1462: Scalar, other_1274: Plane) -> Plane {
    var self_1463: Scalar;
    var other_1275: Plane;

    self_1463 = self_1462;
    other_1275 = other_1274;
    let _e6: Plane = other_1275;
    let _e7: Plane = plane_inverse(_e6);
    let _e8: Scalar = self_1463;
    let _e10: Plane = other_1275;
    let _e11: Plane = plane_inverse(_e10);
    let _e12: Plane = scalar_plane_geometric_product(_e8, _e11);
    return _e12;
}

fn scalar_plane_transformation(self_1464: Scalar, other_1276: Plane) -> Plane {
    var self_1465: Scalar;
    var other_1277: Plane;

    self_1465 = self_1464;
    other_1277 = other_1276;
    let _e6: Scalar = self_1465;
    let _e7: Plane = other_1277;
    let _e8: Plane = scalar_plane_geometric_product(_e6, _e7);
    let _e10: Scalar = self_1465;
    let _e11: Scalar = scalar_reversal(_e10);
    let _e14: Scalar = self_1465;
    let _e15: Plane = other_1277;
    let _e16: Plane = scalar_plane_geometric_product(_e14, _e15);
    let _e18: Scalar = self_1465;
    let _e19: Scalar = scalar_reversal(_e18);
    let _e20: Plane = plane_scalar_geometric_product(_e16, _e19);
    return _e20;
}

fn scalar_point_geometric_quotient(self_1466: Scalar, other_1278: Point) -> Point {
    var self_1467: Scalar;
    var other_1279: Point;

    self_1467 = self_1466;
    other_1279 = other_1278;
    let _e6: Point = other_1279;
    let _e7: Point = point_inverse(_e6);
    let _e8: Scalar = self_1467;
    let _e10: Point = other_1279;
    let _e11: Point = point_inverse(_e10);
    let _e12: Point = scalar_point_geometric_product(_e8, _e11);
    return _e12;
}

fn scalar_point_transformation(self_1468: Scalar, other_1280: Point) -> Point {
    var self_1469: Scalar;
    var other_1281: Point;

    self_1469 = self_1468;
    other_1281 = other_1280;
    let _e6: Scalar = self_1469;
    let _e7: Point = other_1281;
    let _e8: Point = scalar_point_geometric_product(_e6, _e7);
    let _e10: Scalar = self_1469;
    let _e11: Scalar = scalar_reversal(_e10);
    let _e14: Scalar = self_1469;
    let _e15: Point = other_1281;
    let _e16: Point = scalar_point_geometric_product(_e14, _e15);
    let _e18: Scalar = self_1469;
    let _e19: Scalar = scalar_reversal(_e18);
    let _e20: Point = point_scalar_geometric_product(_e16, _e19);
    return _e20;
}

fn scalar_rotor_geometric_quotient(self_1470: Scalar, other_1282: Rotor) -> Rotor {
    var self_1471: Scalar;
    var other_1283: Rotor;

    self_1471 = self_1470;
    other_1283 = other_1282;
    let _e6: Rotor = other_1283;
    let _e7: Rotor = rotor_inverse(_e6);
    let _e8: Scalar = self_1471;
    let _e10: Rotor = other_1283;
    let _e11: Rotor = rotor_inverse(_e10);
    let _e12: Rotor = scalar_rotor_geometric_product(_e8, _e11);
    return _e12;
}

fn scalar_rotor_transformation(self_1472: Scalar, other_1284: Rotor) -> Rotor {
    var self_1473: Scalar;
    var other_1285: Rotor;

    self_1473 = self_1472;
    other_1285 = other_1284;
    let _e6: Scalar = self_1473;
    let _e7: Rotor = other_1285;
    let _e8: Rotor = scalar_rotor_geometric_product(_e6, _e7);
    let _e10: Scalar = self_1473;
    let _e11: Scalar = scalar_reversal(_e10);
    let _e14: Scalar = self_1473;
    let _e15: Rotor = other_1285;
    let _e16: Rotor = scalar_rotor_geometric_product(_e14, _e15);
    let _e18: Scalar = self_1473;
    let _e19: Scalar = scalar_reversal(_e18);
    let _e20: Rotor = rotor_scalar_geometric_product(_e16, _e19);
    return _e20;
}

fn scalar_powi(self_1474: Scalar, exponent_6: i32) -> Scalar {
    var self_1475: Scalar;
    var exponent_7: i32;
    var local_3: Scalar;
    var x_3: Scalar;
    var y_3: Scalar;
    var n_3: i32;

    self_1475 = self_1474;
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
        let _e12: Scalar = self_1475;
        let _e13: Scalar = scalar_inverse(_e12);
        local_3 = _e13;
    } else {
        let _e14: Scalar = self_1475;
        local_3 = _e14;
    }
    let _e16: Scalar = local_3;
    x_3 = _e16;
    let _e18: Scalar = scalar_one();
    y_3 = _e18;
    let _e21: i32 = exponent_7;
    n_3 = abs(_e21);
    loop {
        let _e25: i32 = n_3;
        if !((1 < _e25)) {
            break;
        }
        {
            let _e28: i32 = n_3;
            if ((_e28 & 1) == 1) {
                {
                    let _e35: Scalar = x_3;
                    let _e36: Scalar = y_3;
                    let _e37: Scalar = scalar_scalar_geometric_product(_e35, _e36);
                    y_3 = _e37;
                }
            }
            let _e40: Scalar = x_3;
            let _e41: Scalar = x_3;
            let _e42: Scalar = scalar_scalar_geometric_product(_e40, _e41);
            x_3 = _e42;
            let _e43: i32 = n_3;
            n_3 = (_e43 >> u32(1));
        }
    }
    let _e49: Scalar = x_3;
    let _e50: Scalar = y_3;
    let _e51: Scalar = scalar_scalar_geometric_product(_e49, _e50);
    return _e51;
}

fn scalar_scalar_geometric_quotient(self_1476: Scalar, other_1286: Scalar) -> Scalar {
    var self_1477: Scalar;
    var other_1287: Scalar;

    self_1477 = self_1476;
    other_1287 = other_1286;
    let _e6: Scalar = other_1287;
    let _e7: Scalar = scalar_inverse(_e6);
    let _e8: Scalar = self_1477;
    let _e10: Scalar = other_1287;
    let _e11: Scalar = scalar_inverse(_e10);
    let _e12: Scalar = scalar_scalar_geometric_product(_e8, _e11);
    return _e12;
}

fn scalar_scalar_transformation(self_1478: Scalar, other_1288: Scalar) -> Scalar {
    var self_1479: Scalar;
    var other_1289: Scalar;

    self_1479 = self_1478;
    other_1289 = other_1288;
    let _e6: Scalar = self_1479;
    let _e7: Scalar = other_1289;
    let _e8: Scalar = scalar_scalar_geometric_product(_e6, _e7);
    let _e10: Scalar = self_1479;
    let _e11: Scalar = scalar_reversal(_e10);
    let _e14: Scalar = self_1479;
    let _e15: Scalar = other_1289;
    let _e16: Scalar = scalar_scalar_geometric_product(_e14, _e15);
    let _e18: Scalar = self_1479;
    let _e19: Scalar = scalar_reversal(_e18);
    let _e20: Scalar = scalar_scalar_geometric_product(_e16, _e19);
    return _e20;
}

fn scalar_translator_geometric_quotient(self_1480: Scalar, other_1290: Translator) -> Translator {
    var self_1481: Scalar;
    var other_1291: Translator;

    self_1481 = self_1480;
    other_1291 = other_1290;
    let _e6: Translator = other_1291;
    let _e7: Translator = translator_inverse(_e6);
    let _e8: Scalar = self_1481;
    let _e10: Translator = other_1291;
    let _e11: Translator = translator_inverse(_e10);
    let _e12: Translator = scalar_translator_geometric_product(_e8, _e11);
    return _e12;
}

fn scalar_translator_transformation(self_1482: Scalar, other_1292: Translator) -> Translator {
    var self_1483: Scalar;
    var other_1293: Translator;

    self_1483 = self_1482;
    other_1293 = other_1292;
    let _e6: Scalar = self_1483;
    let _e7: Translator = other_1293;
    let _e8: Translator = scalar_translator_geometric_product(_e6, _e7);
    let _e10: Scalar = self_1483;
    let _e11: Scalar = scalar_reversal(_e10);
    let _e14: Scalar = self_1483;
    let _e15: Translator = other_1293;
    let _e16: Translator = scalar_translator_geometric_product(_e14, _e15);
    let _e18: Scalar = self_1483;
    let _e19: Scalar = scalar_reversal(_e18);
    let _e20: Translator = translator_scalar_geometric_product(_e16, _e19);
    return _e20;
}

fn translator_ideal_point_transformation(self_1484: Translator, other_1294: IdealPoint) -> IdealPoint {
    var self_1485: Translator;
    var other_1295: IdealPoint;

    self_1485 = self_1484;
    other_1295 = other_1294;
    let _e6: Translator = self_1485;
    let _e7: IdealPoint = other_1295;
    let _e8: IdealPoint = translator_ideal_point_geometric_product(_e6, _e7);
    let _e10: Translator = self_1485;
    let _e11: Translator = translator_reversal(_e10);
    let _e14: Translator = self_1485;
    let _e15: IdealPoint = other_1295;
    let _e16: IdealPoint = translator_ideal_point_geometric_product(_e14, _e15);
    let _e18: Translator = self_1485;
    let _e19: Translator = translator_reversal(_e18);
    let _e20: IdealPoint = ideal_point_translator_geometric_product(_e16, _e19);
    return _e20;
}

fn translator_motor_geometric_quotient(self_1486: Translator, other_1296: Motor) -> Motor {
    var self_1487: Translator;
    var other_1297: Motor;

    self_1487 = self_1486;
    other_1297 = other_1296;
    let _e6: Motor = other_1297;
    let _e7: Motor = motor_inverse(_e6);
    let _e8: Translator = self_1487;
    let _e10: Motor = other_1297;
    let _e11: Motor = motor_inverse(_e10);
    let _e12: Motor = translator_motor_geometric_product(_e8, _e11);
    return _e12;
}

fn translator_motor_transformation(self_1488: Translator, other_1298: Motor) -> Motor {
    var self_1489: Translator;
    var other_1299: Motor;

    self_1489 = self_1488;
    other_1299 = other_1298;
    let _e6: Translator = self_1489;
    let _e7: Motor = other_1299;
    let _e8: Motor = translator_motor_geometric_product(_e6, _e7);
    let _e10: Translator = self_1489;
    let _e11: Translator = translator_reversal(_e10);
    let _e14: Translator = self_1489;
    let _e15: Motor = other_1299;
    let _e16: Motor = translator_motor_geometric_product(_e14, _e15);
    let _e18: Translator = self_1489;
    let _e19: Translator = translator_reversal(_e18);
    let _e20: Motor = motor_translator_geometric_product(_e16, _e19);
    return _e20;
}

fn translator_motor_dual_geometric_quotient(self_1490: Translator, other_1300: MotorDual) -> MotorDual {
    var self_1491: Translator;
    var other_1301: MotorDual;

    self_1491 = self_1490;
    other_1301 = other_1300;
    let _e6: MotorDual = other_1301;
    let _e7: MotorDual = motor_dual_inverse(_e6);
    let _e8: Translator = self_1491;
    let _e10: MotorDual = other_1301;
    let _e11: MotorDual = motor_dual_inverse(_e10);
    let _e12: MotorDual = translator_motor_dual_geometric_product(_e8, _e11);
    return _e12;
}

fn translator_motor_dual_transformation(self_1492: Translator, other_1302: MotorDual) -> MotorDual {
    var self_1493: Translator;
    var other_1303: MotorDual;

    self_1493 = self_1492;
    other_1303 = other_1302;
    let _e6: Translator = self_1493;
    let _e7: MotorDual = other_1303;
    let _e8: MotorDual = translator_motor_dual_geometric_product(_e6, _e7);
    let _e10: Translator = self_1493;
    let _e11: Translator = translator_reversal(_e10);
    let _e14: Translator = self_1493;
    let _e15: MotorDual = other_1303;
    let _e16: MotorDual = translator_motor_dual_geometric_product(_e14, _e15);
    let _e18: Translator = self_1493;
    let _e19: Translator = translator_reversal(_e18);
    let _e20: MotorDual = motor_dual_translator_geometric_product(_e16, _e19);
    return _e20;
}

fn translator_multi_vector_geometric_quotient(self_1494: Translator, other_1304: MultiVector) -> MultiVector {
    var self_1495: Translator;
    var other_1305: MultiVector;

    self_1495 = self_1494;
    other_1305 = other_1304;
    let _e6: MultiVector = other_1305;
    let _e7: MultiVector = multi_vector_inverse(_e6);
    let _e8: Translator = self_1495;
    let _e10: MultiVector = other_1305;
    let _e11: MultiVector = multi_vector_inverse(_e10);
    let _e12: MultiVector = translator_multi_vector_geometric_product(_e8, _e11);
    return _e12;
}

fn translator_multi_vector_transformation(self_1496: Translator, other_1306: MultiVector) -> MultiVector {
    var self_1497: Translator;
    var other_1307: MultiVector;

    self_1497 = self_1496;
    other_1307 = other_1306;
    let _e6: Translator = self_1497;
    let _e7: MultiVector = other_1307;
    let _e8: MultiVector = translator_multi_vector_geometric_product(_e6, _e7);
    let _e10: Translator = self_1497;
    let _e11: Translator = translator_reversal(_e10);
    let _e14: Translator = self_1497;
    let _e15: MultiVector = other_1307;
    let _e16: MultiVector = translator_multi_vector_geometric_product(_e14, _e15);
    let _e18: Translator = self_1497;
    let _e19: Translator = translator_reversal(_e18);
    let _e20: MultiVector = multi_vector_translator_geometric_product(_e16, _e19);
    return _e20;
}

fn translator_plane_geometric_quotient(self_1498: Translator, other_1308: Plane) -> MotorDual {
    var self_1499: Translator;
    var other_1309: Plane;

    self_1499 = self_1498;
    other_1309 = other_1308;
    let _e6: Plane = other_1309;
    let _e7: Plane = plane_inverse(_e6);
    let _e8: Translator = self_1499;
    let _e10: Plane = other_1309;
    let _e11: Plane = plane_inverse(_e10);
    let _e12: MotorDual = translator_plane_geometric_product(_e8, _e11);
    return _e12;
}

fn translator_plane_transformation(self_1500: Translator, other_1310: Plane) -> Plane {
    var self_1501: Translator;
    var other_1311: Plane;

    self_1501 = self_1500;
    other_1311 = other_1310;
    let _e6: Translator = self_1501;
    let _e7: Plane = other_1311;
    let _e8: MotorDual = translator_plane_geometric_product(_e6, _e7);
    let _e10: Translator = self_1501;
    let _e11: Translator = translator_reversal(_e10);
    let _e14: Translator = self_1501;
    let _e15: Plane = other_1311;
    let _e16: MotorDual = translator_plane_geometric_product(_e14, _e15);
    let _e18: Translator = self_1501;
    let _e19: Translator = translator_reversal(_e18);
    let _e20: MotorDual = motor_dual_translator_geometric_product(_e16, _e19);
    let _e23: Translator = self_1501;
    let _e24: Plane = other_1311;
    let _e25: MotorDual = translator_plane_geometric_product(_e23, _e24);
    let _e27: Translator = self_1501;
    let _e28: Translator = translator_reversal(_e27);
    let _e31: Translator = self_1501;
    let _e32: Plane = other_1311;
    let _e33: MotorDual = translator_plane_geometric_product(_e31, _e32);
    let _e35: Translator = self_1501;
    let _e36: Translator = translator_reversal(_e35);
    let _e37: MotorDual = motor_dual_translator_geometric_product(_e33, _e36);
    let _e38: Plane = motor_dual_plane_into(_e37);
    return _e38;
}

fn translator_point_geometric_quotient(self_1502: Translator, other_1312: Point) -> Point {
    var self_1503: Translator;
    var other_1313: Point;

    self_1503 = self_1502;
    other_1313 = other_1312;
    let _e6: Point = other_1313;
    let _e7: Point = point_inverse(_e6);
    let _e8: Translator = self_1503;
    let _e10: Point = other_1313;
    let _e11: Point = point_inverse(_e10);
    let _e12: Point = translator_point_geometric_product(_e8, _e11);
    return _e12;
}

fn translator_point_transformation(self_1504: Translator, other_1314: Point) -> Point {
    var self_1505: Translator;
    var other_1315: Point;

    self_1505 = self_1504;
    other_1315 = other_1314;
    let _e6: Translator = self_1505;
    let _e7: Point = other_1315;
    let _e8: Point = translator_point_geometric_product(_e6, _e7);
    let _e10: Translator = self_1505;
    let _e11: Translator = translator_reversal(_e10);
    let _e14: Translator = self_1505;
    let _e15: Point = other_1315;
    let _e16: Point = translator_point_geometric_product(_e14, _e15);
    let _e18: Translator = self_1505;
    let _e19: Translator = translator_reversal(_e18);
    let _e20: Point = point_translator_geometric_product(_e16, _e19);
    return _e20;
}

fn translator_rotor_geometric_quotient(self_1506: Translator, other_1316: Rotor) -> Motor {
    var self_1507: Translator;
    var other_1317: Rotor;

    self_1507 = self_1506;
    other_1317 = other_1316;
    let _e6: Rotor = other_1317;
    let _e7: Rotor = rotor_inverse(_e6);
    let _e8: Translator = self_1507;
    let _e10: Rotor = other_1317;
    let _e11: Rotor = rotor_inverse(_e10);
    let _e12: Motor = translator_rotor_geometric_product(_e8, _e11);
    return _e12;
}

fn translator_rotor_transformation(self_1508: Translator, other_1318: Rotor) -> Rotor {
    var self_1509: Translator;
    var other_1319: Rotor;

    self_1509 = self_1508;
    other_1319 = other_1318;
    let _e6: Translator = self_1509;
    let _e7: Rotor = other_1319;
    let _e8: Motor = translator_rotor_geometric_product(_e6, _e7);
    let _e10: Translator = self_1509;
    let _e11: Translator = translator_reversal(_e10);
    let _e14: Translator = self_1509;
    let _e15: Rotor = other_1319;
    let _e16: Motor = translator_rotor_geometric_product(_e14, _e15);
    let _e18: Translator = self_1509;
    let _e19: Translator = translator_reversal(_e18);
    let _e20: Motor = motor_translator_geometric_product(_e16, _e19);
    let _e23: Translator = self_1509;
    let _e24: Rotor = other_1319;
    let _e25: Motor = translator_rotor_geometric_product(_e23, _e24);
    let _e27: Translator = self_1509;
    let _e28: Translator = translator_reversal(_e27);
    let _e31: Translator = self_1509;
    let _e32: Rotor = other_1319;
    let _e33: Motor = translator_rotor_geometric_product(_e31, _e32);
    let _e35: Translator = self_1509;
    let _e36: Translator = translator_reversal(_e35);
    let _e37: Motor = motor_translator_geometric_product(_e33, _e36);
    let _e38: Rotor = motor_rotor_into(_e37);
    return _e38;
}

fn translator_scalar_geometric_quotient(self_1510: Translator, other_1320: Scalar) -> Translator {
    var self_1511: Translator;
    var other_1321: Scalar;

    self_1511 = self_1510;
    other_1321 = other_1320;
    let _e6: Scalar = other_1321;
    let _e7: Scalar = scalar_inverse(_e6);
    let _e8: Translator = self_1511;
    let _e10: Scalar = other_1321;
    let _e11: Scalar = scalar_inverse(_e10);
    let _e12: Translator = translator_scalar_geometric_product(_e8, _e11);
    return _e12;
}

fn translator_scalar_transformation(self_1512: Translator, other_1322: Scalar) -> Scalar {
    var self_1513: Translator;
    var other_1323: Scalar;

    self_1513 = self_1512;
    other_1323 = other_1322;
    let _e6: Translator = self_1513;
    let _e7: Scalar = other_1323;
    let _e8: Translator = translator_scalar_geometric_product(_e6, _e7);
    let _e10: Translator = self_1513;
    let _e11: Translator = translator_reversal(_e10);
    let _e14: Translator = self_1513;
    let _e15: Scalar = other_1323;
    let _e16: Translator = translator_scalar_geometric_product(_e14, _e15);
    let _e18: Translator = self_1513;
    let _e19: Translator = translator_reversal(_e18);
    let _e20: Translator = translator_translator_geometric_product(_e16, _e19);
    let _e23: Translator = self_1513;
    let _e24: Scalar = other_1323;
    let _e25: Translator = translator_scalar_geometric_product(_e23, _e24);
    let _e27: Translator = self_1513;
    let _e28: Translator = translator_reversal(_e27);
    let _e31: Translator = self_1513;
    let _e32: Scalar = other_1323;
    let _e33: Translator = translator_scalar_geometric_product(_e31, _e32);
    let _e35: Translator = self_1513;
    let _e36: Translator = translator_reversal(_e35);
    let _e37: Translator = translator_translator_geometric_product(_e33, _e36);
    let _e38: Scalar = translator_scalar_into(_e37);
    return _e38;
}

fn translator_powi(self_1514: Translator, exponent_8: i32) -> Translator {
    var self_1515: Translator;
    var exponent_9: i32;
    var local_4: Translator;
    var x_4: Translator;
    var y_4: Translator;
    var n_4: i32;

    self_1515 = self_1514;
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
        let _e12: Translator = self_1515;
        let _e13: Translator = translator_inverse(_e12);
        local_4 = _e13;
    } else {
        let _e14: Translator = self_1515;
        local_4 = _e14;
    }
    let _e16: Translator = local_4;
    x_4 = _e16;
    let _e18: Translator = translator_one();
    y_4 = _e18;
    let _e21: i32 = exponent_9;
    n_4 = abs(_e21);
    loop {
        let _e25: i32 = n_4;
        if !((1 < _e25)) {
            break;
        }
        {
            let _e28: i32 = n_4;
            if ((_e28 & 1) == 1) {
                {
                    let _e35: Translator = x_4;
                    let _e36: Translator = y_4;
                    let _e37: Translator = translator_translator_geometric_product(_e35, _e36);
                    y_4 = _e37;
                }
            }
            let _e40: Translator = x_4;
            let _e41: Translator = x_4;
            let _e42: Translator = translator_translator_geometric_product(_e40, _e41);
            x_4 = _e42;
            let _e43: i32 = n_4;
            n_4 = (_e43 >> u32(1));
        }
    }
    let _e49: Translator = x_4;
    let _e50: Translator = y_4;
    let _e51: Translator = translator_translator_geometric_product(_e49, _e50);
    return _e51;
}

fn translator_translator_geometric_quotient(self_1516: Translator, other_1324: Translator) -> Translator {
    var self_1517: Translator;
    var other_1325: Translator;

    self_1517 = self_1516;
    other_1325 = other_1324;
    let _e6: Translator = other_1325;
    let _e7: Translator = translator_inverse(_e6);
    let _e8: Translator = self_1517;
    let _e10: Translator = other_1325;
    let _e11: Translator = translator_inverse(_e10);
    let _e12: Translator = translator_translator_geometric_product(_e8, _e11);
    return _e12;
}

fn translator_translator_transformation(self_1518: Translator, other_1326: Translator) -> Translator {
    var self_1519: Translator;
    var other_1327: Translator;

    self_1519 = self_1518;
    other_1327 = other_1326;
    let _e6: Translator = self_1519;
    let _e7: Translator = other_1327;
    let _e8: Translator = translator_translator_geometric_product(_e6, _e7);
    let _e10: Translator = self_1519;
    let _e11: Translator = translator_reversal(_e10);
    let _e14: Translator = self_1519;
    let _e15: Translator = other_1327;
    let _e16: Translator = translator_translator_geometric_product(_e14, _e15);
    let _e18: Translator = self_1519;
    let _e19: Translator = translator_reversal(_e18);
    let _e20: Translator = translator_translator_geometric_product(_e16, _e19);
    return _e20;
}

fn main_1() {
    return;
}

@compute @workgroup_size(1, 1, 1) 
fn main() {
    main_1();
    return;
}

struct Scalar {
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

struct Plane {
    g0_: vec4<f32>,
}

struct Line {
    g0_: vec3<f32>,
    g1_: vec3<f32>,
}

struct Translator {
    g0_: vec4<f32>,
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
    let _e18: MultiVector = other_21;
    let _e20: MultiVector = other_21;
    return MultiVector(((vec4<f32>(_e4.g0_) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) + _e13.g0_), _e16.g1_, _e18.g2_, _e20.g3_);
}

fn scalar_multi_vector_sub(self_30: Scalar, other_22: MultiVector) -> MultiVector {
    var self_31: Scalar;
    var other_23: MultiVector;

    self_31 = self_30;
    other_23 = other_22;
    let _e4: Scalar = self_31;
    let _e13: MultiVector = other_23;
    let _e18: MultiVector = other_23;
    let _e23: MultiVector = other_23;
    let _e28: MultiVector = other_23;
    return MultiVector(((vec4<f32>(_e4.g0_) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) - _e13.g0_), (vec4<f32>(0.0) - _e18.g1_), (vec4<f32>(0.0) - _e23.g2_), (vec4<f32>(0.0) - _e28.g3_));
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
    let _e16: Scalar = self_33;
    let _e19: MultiVector = other_25;
    let _e22: Scalar = self_33;
    let _e25: MultiVector = other_25;
    return MultiVector((vec4<f32>(_e4.g0_) * _e7.g0_), (vec4<f32>(_e10.g0_) * _e13.g1_), (vec4<f32>(_e16.g0_) * _e19.g2_), (vec4<f32>(_e22.g0_) * _e25.g3_));
}

fn scalar_multi_vector_regressive_product(self_34: Scalar, other_26: MultiVector) -> Scalar {
    var self_35: Scalar;
    var other_27: MultiVector;

    self_35 = self_34;
    other_27 = other_26;
    let _e4: Scalar = self_35;
    let _e6: MultiVector = other_27;
    return Scalar((_e4.g0_ * _e6.g3_.x));
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
    let _e16: Scalar = self_37;
    let _e19: MultiVector = other_29;
    let _e22: Scalar = self_37;
    let _e25: MultiVector = other_29;
    return MultiVector((vec4<f32>(_e4.g0_) * _e7.g0_), (vec4<f32>(_e10.g0_) * _e13.g1_), (vec4<f32>(_e16.g0_) * _e19.g2_), (vec4<f32>(_e22.g0_) * _e25.g3_));
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
    let _e16: Scalar = self_39;
    let _e19: MultiVector = other_31;
    let _e22: Scalar = self_39;
    let _e25: MultiVector = other_31;
    return MultiVector((vec4<f32>(_e4.g0_) * _e7.g0_), (vec4<f32>(_e10.g0_) * _e13.g1_), (vec4<f32>(_e16.g0_) * _e19.g2_), (vec4<f32>(_e22.g0_) * _e25.g3_));
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
    let _e16: Scalar = self_41;
    let _e19: MultiVector = other_33;
    let _e22: Scalar = self_41;
    let _e25: MultiVector = other_33;
    return MultiVector((vec4<f32>(_e4.g0_) * _e7.g0_), (vec4<f32>(_e10.g0_) * _e13.g1_), (vec4<f32>(_e16.g0_) * _e19.g2_), (vec4<f32>(_e22.g0_) * _e25.g3_));
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
    let _e13: Rotor = other_39;
    return Rotor(((vec4<f32>(_e4.g0_) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) + _e13.g0_));
}

fn scalar_rotor_sub(self_48: Scalar, other_40: Rotor) -> Rotor {
    var self_49: Scalar;
    var other_41: Rotor;

    self_49 = self_48;
    other_41 = other_40;
    let _e4: Scalar = self_49;
    let _e13: Rotor = other_41;
    return Rotor(((vec4<f32>(_e4.g0_) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) - _e13.g0_));
}

fn scalar_rotor_geometric_product(self_50: Scalar, other_42: Rotor) -> Rotor {
    var self_51: Scalar;
    var other_43: Rotor;

    self_51 = self_50;
    other_43 = other_42;
    let _e4: Scalar = self_51;
    let _e7: Rotor = other_43;
    return Rotor((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_rotor_outer_product(self_52: Scalar, other_44: Rotor) -> Rotor {
    var self_53: Scalar;
    var other_45: Rotor;

    self_53 = self_52;
    other_45 = other_44;
    let _e4: Scalar = self_53;
    let _e7: Rotor = other_45;
    return Rotor((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_rotor_inner_product(self_54: Scalar, other_46: Rotor) -> Rotor {
    var self_55: Scalar;
    var other_47: Rotor;

    self_55 = self_54;
    other_47 = other_46;
    let _e4: Scalar = self_55;
    let _e7: Rotor = other_47;
    return Rotor((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_rotor_left_contraction(self_56: Scalar, other_48: Rotor) -> Rotor {
    var self_57: Scalar;
    var other_49: Rotor;

    self_57 = self_56;
    other_49 = other_48;
    let _e4: Scalar = self_57;
    let _e7: Rotor = other_49;
    return Rotor((vec4<f32>(_e4.g0_) * _e7.g0_));
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

fn scalar_point_geometric_product(self_62: Scalar, other_54: Point) -> Point {
    var self_63: Scalar;
    var other_55: Point;

    self_63 = self_62;
    other_55 = other_54;
    let _e4: Scalar = self_63;
    let _e7: Point = other_55;
    return Point((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_point_outer_product(self_64: Scalar, other_56: Point) -> Point {
    var self_65: Scalar;
    var other_57: Point;

    self_65 = self_64;
    other_57 = other_56;
    let _e4: Scalar = self_65;
    let _e7: Point = other_57;
    return Point((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_point_inner_product(self_66: Scalar, other_58: Point) -> Point {
    var self_67: Scalar;
    var other_59: Point;

    self_67 = self_66;
    other_59 = other_58;
    let _e4: Scalar = self_67;
    let _e7: Point = other_59;
    return Point((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_point_left_contraction(self_68: Scalar, other_60: Point) -> Point {
    var self_69: Scalar;
    var other_61: Point;

    self_69 = self_68;
    other_61 = other_60;
    let _e4: Scalar = self_69;
    let _e7: Point = other_61;
    return Point((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_ideal_point_add(self_70: Scalar, other_62: IdealPoint) -> Translator {
    var self_71: Scalar;
    var other_63: IdealPoint;

    self_71 = self_70;
    other_63 = other_62;
    let _e4: Scalar = self_71;
    let _e13: IdealPoint = other_63;
    let _e16: IdealPoint = other_63;
    let _e19: IdealPoint = other_63;
    let _e22: IdealPoint = other_63;
    return Translator(((vec4<f32>(_e4.g0_) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) + (vec4<f32>(_e13.g0_.x, _e16.g0_.x, _e19.g0_.y, _e22.g0_.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn scalar_ideal_point_sub(self_72: Scalar, other_64: IdealPoint) -> Translator {
    var self_73: Scalar;
    var other_65: IdealPoint;

    self_73 = self_72;
    other_65 = other_64;
    let _e4: Scalar = self_73;
    let _e13: IdealPoint = other_65;
    let _e16: IdealPoint = other_65;
    let _e19: IdealPoint = other_65;
    let _e22: IdealPoint = other_65;
    return Translator(((vec4<f32>(_e4.g0_) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) - (vec4<f32>(_e13.g0_.x, _e16.g0_.x, _e19.g0_.y, _e22.g0_.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn scalar_ideal_point_geometric_product(self_74: Scalar, other_66: IdealPoint) -> IdealPoint {
    var self_75: Scalar;
    var other_67: IdealPoint;

    self_75 = self_74;
    other_67 = other_66;
    let _e4: Scalar = self_75;
    let _e7: IdealPoint = other_67;
    return IdealPoint((vec3<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_ideal_point_outer_product(self_76: Scalar, other_68: IdealPoint) -> IdealPoint {
    var self_77: Scalar;
    var other_69: IdealPoint;

    self_77 = self_76;
    other_69 = other_68;
    let _e4: Scalar = self_77;
    let _e7: IdealPoint = other_69;
    return IdealPoint((vec3<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_ideal_point_inner_product(self_78: Scalar, other_70: IdealPoint) -> IdealPoint {
    var self_79: Scalar;
    var other_71: IdealPoint;

    self_79 = self_78;
    other_71 = other_70;
    let _e4: Scalar = self_79;
    let _e7: IdealPoint = other_71;
    return IdealPoint((vec3<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_ideal_point_left_contraction(self_80: Scalar, other_72: IdealPoint) -> IdealPoint {
    var self_81: Scalar;
    var other_73: IdealPoint;

    self_81 = self_80;
    other_73 = other_72;
    let _e4: Scalar = self_81;
    let _e7: IdealPoint = other_73;
    return IdealPoint((vec3<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_plane_geometric_product(self_82: Scalar, other_74: Plane) -> Plane {
    var self_83: Scalar;
    var other_75: Plane;

    self_83 = self_82;
    other_75 = other_74;
    let _e4: Scalar = self_83;
    let _e7: Plane = other_75;
    return Plane((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_plane_outer_product(self_84: Scalar, other_76: Plane) -> Plane {
    var self_85: Scalar;
    var other_77: Plane;

    self_85 = self_84;
    other_77 = other_76;
    let _e4: Scalar = self_85;
    let _e7: Plane = other_77;
    return Plane((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_plane_inner_product(self_86: Scalar, other_78: Plane) -> Plane {
    var self_87: Scalar;
    var other_79: Plane;

    self_87 = self_86;
    other_79 = other_78;
    let _e4: Scalar = self_87;
    let _e7: Plane = other_79;
    return Plane((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_plane_left_contraction(self_88: Scalar, other_80: Plane) -> Plane {
    var self_89: Scalar;
    var other_81: Plane;

    self_89 = self_88;
    other_81 = other_80;
    let _e4: Scalar = self_89;
    let _e7: Plane = other_81;
    return Plane((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_line_geometric_product(self_90: Scalar, other_82: Line) -> Line {
    var self_91: Scalar;
    var other_83: Line;

    self_91 = self_90;
    other_83 = other_82;
    let _e4: Scalar = self_91;
    let _e7: Line = other_83;
    let _e10: Scalar = self_91;
    let _e13: Line = other_83;
    return Line((vec3<f32>(_e4.g0_) * _e7.g0_), (vec3<f32>(_e10.g0_) * _e13.g1_));
}

fn scalar_line_outer_product(self_92: Scalar, other_84: Line) -> Line {
    var self_93: Scalar;
    var other_85: Line;

    self_93 = self_92;
    other_85 = other_84;
    let _e4: Scalar = self_93;
    let _e7: Line = other_85;
    let _e10: Scalar = self_93;
    let _e13: Line = other_85;
    return Line((vec3<f32>(_e4.g0_) * _e7.g0_), (vec3<f32>(_e10.g0_) * _e13.g1_));
}

fn scalar_line_inner_product(self_94: Scalar, other_86: Line) -> Line {
    var self_95: Scalar;
    var other_87: Line;

    self_95 = self_94;
    other_87 = other_86;
    let _e4: Scalar = self_95;
    let _e7: Line = other_87;
    let _e10: Scalar = self_95;
    let _e13: Line = other_87;
    return Line((vec3<f32>(_e4.g0_) * _e7.g0_), (vec3<f32>(_e10.g0_) * _e13.g1_));
}

fn scalar_line_left_contraction(self_96: Scalar, other_88: Line) -> Line {
    var self_97: Scalar;
    var other_89: Line;

    self_97 = self_96;
    other_89 = other_88;
    let _e4: Scalar = self_97;
    let _e7: Line = other_89;
    let _e10: Scalar = self_97;
    let _e13: Line = other_89;
    return Line((vec3<f32>(_e4.g0_) * _e7.g0_), (vec3<f32>(_e10.g0_) * _e13.g1_));
}

fn scalar_translator_add(self_98: Scalar, other_90: Translator) -> Translator {
    var self_99: Scalar;
    var other_91: Translator;

    self_99 = self_98;
    other_91 = other_90;
    let _e4: Scalar = self_99;
    let _e13: Translator = other_91;
    return Translator(((vec4<f32>(_e4.g0_) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) + _e13.g0_));
}

fn scalar_translator_sub(self_100: Scalar, other_92: Translator) -> Translator {
    var self_101: Scalar;
    var other_93: Translator;

    self_101 = self_100;
    other_93 = other_92;
    let _e4: Scalar = self_101;
    let _e13: Translator = other_93;
    return Translator(((vec4<f32>(_e4.g0_) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) - _e13.g0_));
}

fn scalar_translator_geometric_product(self_102: Scalar, other_94: Translator) -> Translator {
    var self_103: Scalar;
    var other_95: Translator;

    self_103 = self_102;
    other_95 = other_94;
    let _e4: Scalar = self_103;
    let _e7: Translator = other_95;
    return Translator((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_translator_outer_product(self_104: Scalar, other_96: Translator) -> Translator {
    var self_105: Scalar;
    var other_97: Translator;

    self_105 = self_104;
    other_97 = other_96;
    let _e4: Scalar = self_105;
    let _e7: Translator = other_97;
    return Translator((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_translator_inner_product(self_106: Scalar, other_98: Translator) -> Translator {
    var self_107: Scalar;
    var other_99: Translator;

    self_107 = self_106;
    other_99 = other_98;
    let _e4: Scalar = self_107;
    let _e7: Translator = other_99;
    return Translator((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_translator_left_contraction(self_108: Scalar, other_100: Translator) -> Translator {
    var self_109: Scalar;
    var other_101: Translator;

    self_109 = self_108;
    other_101 = other_100;
    let _e4: Scalar = self_109;
    let _e7: Translator = other_101;
    return Translator((vec4<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_translator_right_contraction(self_110: Scalar, other_102: Translator) -> Scalar {
    var self_111: Scalar;
    var other_103: Translator;

    self_111 = self_110;
    other_103 = other_102;
    let _e4: Scalar = self_111;
    let _e6: Translator = other_103;
    return Scalar((_e4.g0_ * _e6.g0_.x));
}

fn scalar_translator_scalar_product(self_112: Scalar, other_104: Translator) -> Scalar {
    var self_113: Scalar;
    var other_105: Translator;

    self_113 = self_112;
    other_105 = other_104;
    let _e4: Scalar = self_113;
    let _e6: Translator = other_105;
    return Scalar((_e4.g0_ * _e6.g0_.x));
}

fn scalar_motor_add(self_114: Scalar, other_106: Motor) -> Motor {
    var self_115: Scalar;
    var other_107: Motor;

    self_115 = self_114;
    other_107 = other_106;
    let _e4: Scalar = self_115;
    let _e13: Motor = other_107;
    let _e16: Motor = other_107;
    return Motor(((vec4<f32>(_e4.g0_) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) + _e13.g0_), _e16.g1_);
}

fn scalar_motor_sub(self_116: Scalar, other_108: Motor) -> Motor {
    var self_117: Scalar;
    var other_109: Motor;

    self_117 = self_116;
    other_109 = other_108;
    let _e4: Scalar = self_117;
    let _e13: Motor = other_109;
    let _e18: Motor = other_109;
    return Motor(((vec4<f32>(_e4.g0_) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) - _e13.g0_), (vec4<f32>(0.0) - _e18.g1_));
}

fn scalar_motor_geometric_product(self_118: Scalar, other_110: Motor) -> Motor {
    var self_119: Scalar;
    var other_111: Motor;

    self_119 = self_118;
    other_111 = other_110;
    let _e4: Scalar = self_119;
    let _e7: Motor = other_111;
    let _e10: Scalar = self_119;
    let _e13: Motor = other_111;
    return Motor((vec4<f32>(_e4.g0_) * _e7.g0_), (vec4<f32>(_e10.g0_) * _e13.g1_));
}

fn scalar_motor_regressive_product(self_120: Scalar, other_112: Motor) -> Scalar {
    var self_121: Scalar;
    var other_113: Motor;

    self_121 = self_120;
    other_113 = other_112;
    let _e4: Scalar = self_121;
    let _e6: Motor = other_113;
    return Scalar((_e4.g0_ * _e6.g1_.x));
}

fn scalar_motor_outer_product(self_122: Scalar, other_114: Motor) -> Motor {
    var self_123: Scalar;
    var other_115: Motor;

    self_123 = self_122;
    other_115 = other_114;
    let _e4: Scalar = self_123;
    let _e7: Motor = other_115;
    let _e10: Scalar = self_123;
    let _e13: Motor = other_115;
    return Motor((vec4<f32>(_e4.g0_) * _e7.g0_), (vec4<f32>(_e10.g0_) * _e13.g1_));
}

fn scalar_motor_inner_product(self_124: Scalar, other_116: Motor) -> Motor {
    var self_125: Scalar;
    var other_117: Motor;

    self_125 = self_124;
    other_117 = other_116;
    let _e4: Scalar = self_125;
    let _e7: Motor = other_117;
    let _e10: Scalar = self_125;
    let _e13: Motor = other_117;
    return Motor((vec4<f32>(_e4.g0_) * _e7.g0_), (vec4<f32>(_e10.g0_) * _e13.g1_));
}

fn scalar_motor_left_contraction(self_126: Scalar, other_118: Motor) -> Motor {
    var self_127: Scalar;
    var other_119: Motor;

    self_127 = self_126;
    other_119 = other_118;
    let _e4: Scalar = self_127;
    let _e7: Motor = other_119;
    let _e10: Scalar = self_127;
    let _e13: Motor = other_119;
    return Motor((vec4<f32>(_e4.g0_) * _e7.g0_), (vec4<f32>(_e10.g0_) * _e13.g1_));
}

fn scalar_motor_right_contraction(self_128: Scalar, other_120: Motor) -> Scalar {
    var self_129: Scalar;
    var other_121: Motor;

    self_129 = self_128;
    other_121 = other_120;
    let _e4: Scalar = self_129;
    let _e6: Motor = other_121;
    return Scalar((_e4.g0_ * _e6.g0_.x));
}

fn scalar_motor_scalar_product(self_130: Scalar, other_122: Motor) -> Scalar {
    var self_131: Scalar;
    var other_123: Motor;

    self_131 = self_130;
    other_123 = other_122;
    let _e4: Scalar = self_131;
    let _e6: Motor = other_123;
    return Scalar((_e4.g0_ * _e6.g0_.x));
}

fn scalar_point_and_plane_geometric_product(self_132: Scalar, other_124: PointAndPlane) -> PointAndPlane {
    var self_133: Scalar;
    var other_125: PointAndPlane;

    self_133 = self_132;
    other_125 = other_124;
    let _e4: Scalar = self_133;
    let _e7: PointAndPlane = other_125;
    let _e10: Scalar = self_133;
    let _e13: PointAndPlane = other_125;
    return PointAndPlane((vec4<f32>(_e4.g0_) * _e7.g0_), (vec4<f32>(_e10.g0_) * _e13.g1_));
}

fn scalar_point_and_plane_outer_product(self_134: Scalar, other_126: PointAndPlane) -> PointAndPlane {
    var self_135: Scalar;
    var other_127: PointAndPlane;

    self_135 = self_134;
    other_127 = other_126;
    let _e4: Scalar = self_135;
    let _e7: PointAndPlane = other_127;
    let _e10: Scalar = self_135;
    let _e13: PointAndPlane = other_127;
    return PointAndPlane((vec4<f32>(_e4.g0_) * _e7.g0_), (vec4<f32>(_e10.g0_) * _e13.g1_));
}

fn scalar_point_and_plane_inner_product(self_136: Scalar, other_128: PointAndPlane) -> PointAndPlane {
    var self_137: Scalar;
    var other_129: PointAndPlane;

    self_137 = self_136;
    other_129 = other_128;
    let _e4: Scalar = self_137;
    let _e7: PointAndPlane = other_129;
    let _e10: Scalar = self_137;
    let _e13: PointAndPlane = other_129;
    return PointAndPlane((vec4<f32>(_e4.g0_) * _e7.g0_), (vec4<f32>(_e10.g0_) * _e13.g1_));
}

fn scalar_point_and_plane_left_contraction(self_138: Scalar, other_130: PointAndPlane) -> PointAndPlane {
    var self_139: Scalar;
    var other_131: PointAndPlane;

    self_139 = self_138;
    other_131 = other_130;
    let _e4: Scalar = self_139;
    let _e7: PointAndPlane = other_131;
    let _e10: Scalar = self_139;
    let _e13: PointAndPlane = other_131;
    return PointAndPlane((vec4<f32>(_e4.g0_) * _e7.g0_), (vec4<f32>(_e10.g0_) * _e13.g1_));
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

fn multi_vector_zero() -> MultiVector {
    return MultiVector(vec4<f32>(0.0), vec4<f32>(0.0), vec4<f32>(0.0), vec4<f32>(0.0));
}

fn multi_vector_one() -> MultiVector {
    return MultiVector(vec4<f32>(1.0, 0.0, 0.0, 0.0), vec4<f32>(0.0), vec4<f32>(0.0), vec4<f32>(0.0));
}

fn multi_vector_neg(self_150: MultiVector) -> MultiVector {
    var self_151: MultiVector;

    self_151 = self_150;
    let _e2: MultiVector = self_151;
    let _e8: MultiVector = self_151;
    let _e14: MultiVector = self_151;
    let _e20: MultiVector = self_151;
    return MultiVector((_e2.g0_ * vec4<f32>(-(1.0))), (_e8.g1_ * vec4<f32>(-(1.0))), (_e14.g2_ * vec4<f32>(-(1.0))), (_e20.g3_ * vec4<f32>(-(1.0))));
}

fn multi_vector_automorphism(self_152: MultiVector) -> MultiVector {
    var self_153: MultiVector;

    self_153 = self_152;
    let _e2: MultiVector = self_153;
    let _e4: MultiVector = self_153;
    let _e10: MultiVector = self_153;
    let _e16: MultiVector = self_153;
    return MultiVector(_e2.g0_, (_e4.g1_ * vec4<f32>(-(1.0))), (_e10.g2_ * vec4<f32>(-(1.0))), _e16.g3_);
}

fn multi_vector_reversal(self_154: MultiVector) -> MultiVector {
    var self_155: MultiVector;

    self_155 = self_154;
    let _e2: MultiVector = self_155;
    let _e13: MultiVector = self_155;
    let _e24: MultiVector = self_155;
    let _e33: MultiVector = self_155;
    return MultiVector((_e2.g0_ * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))), (_e13.g1_ * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))), (_e24.g2_ * vec4<f32>(-(1.0), 1.0, 1.0, 1.0)), (_e33.g3_ * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))));
}

fn multi_vector_conjugation(self_156: MultiVector) -> MultiVector {
    var self_157: MultiVector;

    self_157 = self_156;
    let _e2: MultiVector = self_157;
    let _e13: MultiVector = self_157;
    let _e22: MultiVector = self_157;
    let _e33: MultiVector = self_157;
    return MultiVector((_e2.g0_ * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))), (_e13.g1_ * vec4<f32>(-(1.0), 1.0, 1.0, 1.0)), (_e22.g2_ * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))), (_e33.g3_ * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))));
}

fn multi_vector_dual(self_158: MultiVector) -> MultiVector {
    var self_159: MultiVector;

    self_159 = self_158;
    let _e2: MultiVector = self_159;
    let _e4: MultiVector = self_159;
    let _e13: MultiVector = self_159;
    let _e24: MultiVector = self_159;
    return MultiVector(_e2.g3_, (_e4.g2_ * vec4<f32>(-(1.0), 1.0, 1.0, 1.0)), (_e13.g1_ * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))), _e24.g0_);
}

fn multi_vector_scalar_into(self_160: MultiVector) -> Scalar {
    var self_161: MultiVector;

    self_161 = self_160;
    let _e2: MultiVector = self_161;
    return Scalar(_e2.g0_.x);
}

fn multi_vector_scalar_add(self_162: MultiVector, other_134: Scalar) -> MultiVector {
    var self_163: MultiVector;
    var other_135: Scalar;

    self_163 = self_162;
    other_135 = other_134;
    let _e4: MultiVector = self_163;
    let _e6: Scalar = other_135;
    let _e16: MultiVector = self_163;
    let _e18: MultiVector = self_163;
    let _e20: MultiVector = self_163;
    return MultiVector((_e4.g0_ + (vec4<f32>(_e6.g0_) * vec4<f32>(1.0, 0.0, 0.0, 0.0))), _e16.g1_, _e18.g2_, _e20.g3_);
}

fn multi_vector_scalar_sub(self_164: MultiVector, other_136: Scalar) -> MultiVector {
    var self_165: MultiVector;
    var other_137: Scalar;

    self_165 = self_164;
    other_137 = other_136;
    let _e4: MultiVector = self_165;
    let _e6: Scalar = other_137;
    let _e16: MultiVector = self_165;
    let _e18: MultiVector = self_165;
    let _e20: MultiVector = self_165;
    return MultiVector((_e4.g0_ - (vec4<f32>(_e6.g0_) * vec4<f32>(1.0, 0.0, 0.0, 0.0))), _e16.g1_, _e18.g2_, _e20.g3_);
}

fn multi_vector_scalar_geometric_product(self_166: MultiVector, other_138: Scalar) -> MultiVector {
    var self_167: MultiVector;
    var other_139: Scalar;

    self_167 = self_166;
    other_139 = other_138;
    let _e4: MultiVector = self_167;
    let _e6: Scalar = other_139;
    let _e10: MultiVector = self_167;
    let _e12: Scalar = other_139;
    let _e16: MultiVector = self_167;
    let _e18: Scalar = other_139;
    let _e22: MultiVector = self_167;
    let _e24: Scalar = other_139;
    return MultiVector((_e4.g0_ * vec4<f32>(_e6.g0_)), (_e10.g1_ * vec4<f32>(_e12.g0_)), (_e16.g2_ * vec4<f32>(_e18.g0_)), (_e22.g3_ * vec4<f32>(_e24.g0_)));
}

fn multi_vector_scalar_regressive_product(self_168: MultiVector, other_140: Scalar) -> Scalar {
    var self_169: MultiVector;
    var other_141: Scalar;

    self_169 = self_168;
    other_141 = other_140;
    let _e4: MultiVector = self_169;
    let _e7: Scalar = other_141;
    return Scalar((_e4.g3_.x * _e7.g0_));
}

fn multi_vector_scalar_outer_product(self_170: MultiVector, other_142: Scalar) -> MultiVector {
    var self_171: MultiVector;
    var other_143: Scalar;

    self_171 = self_170;
    other_143 = other_142;
    let _e4: MultiVector = self_171;
    let _e6: Scalar = other_143;
    let _e10: MultiVector = self_171;
    let _e12: Scalar = other_143;
    let _e16: MultiVector = self_171;
    let _e18: Scalar = other_143;
    let _e22: MultiVector = self_171;
    let _e24: Scalar = other_143;
    return MultiVector((_e4.g0_ * vec4<f32>(_e6.g0_)), (_e10.g1_ * vec4<f32>(_e12.g0_)), (_e16.g2_ * vec4<f32>(_e18.g0_)), (_e22.g3_ * vec4<f32>(_e24.g0_)));
}

fn multi_vector_scalar_inner_product(self_172: MultiVector, other_144: Scalar) -> MultiVector {
    var self_173: MultiVector;
    var other_145: Scalar;

    self_173 = self_172;
    other_145 = other_144;
    let _e4: MultiVector = self_173;
    let _e6: Scalar = other_145;
    let _e10: MultiVector = self_173;
    let _e12: Scalar = other_145;
    let _e16: MultiVector = self_173;
    let _e18: Scalar = other_145;
    let _e22: MultiVector = self_173;
    let _e24: Scalar = other_145;
    return MultiVector((_e4.g0_ * vec4<f32>(_e6.g0_)), (_e10.g1_ * vec4<f32>(_e12.g0_)), (_e16.g2_ * vec4<f32>(_e18.g0_)), (_e22.g3_ * vec4<f32>(_e24.g0_)));
}

fn multi_vector_scalar_left_contraction(self_174: MultiVector, other_146: Scalar) -> Scalar {
    var self_175: MultiVector;
    var other_147: Scalar;

    self_175 = self_174;
    other_147 = other_146;
    let _e4: MultiVector = self_175;
    let _e7: Scalar = other_147;
    return Scalar((_e4.g0_.x * _e7.g0_));
}

fn multi_vector_scalar_right_contraction(self_176: MultiVector, other_148: Scalar) -> MultiVector {
    var self_177: MultiVector;
    var other_149: Scalar;

    self_177 = self_176;
    other_149 = other_148;
    let _e4: MultiVector = self_177;
    let _e6: Scalar = other_149;
    let _e10: MultiVector = self_177;
    let _e12: Scalar = other_149;
    let _e16: MultiVector = self_177;
    let _e18: Scalar = other_149;
    let _e22: MultiVector = self_177;
    let _e24: Scalar = other_149;
    return MultiVector((_e4.g0_ * vec4<f32>(_e6.g0_)), (_e10.g1_ * vec4<f32>(_e12.g0_)), (_e16.g2_ * vec4<f32>(_e18.g0_)), (_e22.g3_ * vec4<f32>(_e24.g0_)));
}

fn multi_vector_scalar_scalar_product(self_178: MultiVector, other_150: Scalar) -> Scalar {
    var self_179: MultiVector;
    var other_151: Scalar;

    self_179 = self_178;
    other_151 = other_150;
    let _e4: MultiVector = self_179;
    let _e7: Scalar = other_151;
    return Scalar((_e4.g0_.x * _e7.g0_));
}

fn multi_vector_multi_vector_add(self_180: MultiVector, other_152: MultiVector) -> MultiVector {
    var self_181: MultiVector;
    var other_153: MultiVector;

    self_181 = self_180;
    other_153 = other_152;
    let _e4: MultiVector = self_181;
    let _e6: MultiVector = other_153;
    let _e9: MultiVector = self_181;
    let _e11: MultiVector = other_153;
    let _e14: MultiVector = self_181;
    let _e16: MultiVector = other_153;
    let _e19: MultiVector = self_181;
    let _e21: MultiVector = other_153;
    return MultiVector((_e4.g0_ + _e6.g0_), (_e9.g1_ + _e11.g1_), (_e14.g2_ + _e16.g2_), (_e19.g3_ + _e21.g3_));
}

fn multi_vector_multi_vector_sub(self_182: MultiVector, other_154: MultiVector) -> MultiVector {
    var self_183: MultiVector;
    var other_155: MultiVector;

    self_183 = self_182;
    other_155 = other_154;
    let _e4: MultiVector = self_183;
    let _e6: MultiVector = other_155;
    let _e9: MultiVector = self_183;
    let _e11: MultiVector = other_155;
    let _e14: MultiVector = self_183;
    let _e16: MultiVector = other_155;
    let _e19: MultiVector = self_183;
    let _e21: MultiVector = other_155;
    return MultiVector((_e4.g0_ - _e6.g0_), (_e9.g1_ - _e11.g1_), (_e14.g2_ - _e16.g2_), (_e19.g3_ - _e21.g3_));
}

fn multi_vector_multi_vector_mul(self_184: MultiVector, other_156: MultiVector) -> MultiVector {
    var self_185: MultiVector;
    var other_157: MultiVector;

    self_185 = self_184;
    other_157 = other_156;
    let _e4: MultiVector = self_185;
    let _e6: MultiVector = other_157;
    let _e9: MultiVector = self_185;
    let _e11: MultiVector = other_157;
    let _e14: MultiVector = self_185;
    let _e16: MultiVector = other_157;
    let _e19: MultiVector = self_185;
    let _e21: MultiVector = other_157;
    return MultiVector((_e4.g0_ * _e6.g0_), (_e9.g1_ * _e11.g1_), (_e14.g2_ * _e16.g2_), (_e19.g3_ * _e21.g3_));
}

fn multi_vector_multi_vector_div(self_186: MultiVector, other_158: MultiVector) -> MultiVector {
    var self_187: MultiVector;
    var other_159: MultiVector;

    self_187 = self_186;
    other_159 = other_158;
    let _e4: MultiVector = self_187;
    let _e7: MultiVector = self_187;
    let _e10: MultiVector = self_187;
    let _e13: MultiVector = self_187;
    let _e23: MultiVector = other_159;
    let _e26: MultiVector = other_159;
    let _e29: MultiVector = other_159;
    let _e32: MultiVector = other_159;
    let _e43: MultiVector = self_187;
    let _e46: MultiVector = self_187;
    let _e49: MultiVector = self_187;
    let _e52: MultiVector = self_187;
    let _e62: MultiVector = other_159;
    let _e65: MultiVector = other_159;
    let _e68: MultiVector = other_159;
    let _e71: MultiVector = other_159;
    let _e82: MultiVector = self_187;
    let _e85: MultiVector = self_187;
    let _e88: MultiVector = self_187;
    let _e91: MultiVector = self_187;
    let _e101: MultiVector = other_159;
    let _e104: MultiVector = other_159;
    let _e107: MultiVector = other_159;
    let _e110: MultiVector = other_159;
    let _e121: MultiVector = self_187;
    let _e124: MultiVector = self_187;
    let _e127: MultiVector = self_187;
    let _e130: MultiVector = self_187;
    let _e140: MultiVector = other_159;
    let _e143: MultiVector = other_159;
    let _e146: MultiVector = other_159;
    let _e149: MultiVector = other_159;
    return MultiVector((((vec4<f32>(_e4.g0_.x, _e7.g0_.y, _e10.g0_.z, _e13.g0_.w) * vec4<f32>(1.0, 1.0, 1.0, 1.0)) / vec4<f32>(_e23.g0_.x, _e26.g0_.y, _e29.g0_.z, _e32.g0_.w)) * vec4<f32>(1.0, 1.0, 1.0, 1.0)), (((vec4<f32>(_e43.g1_.x, _e46.g1_.y, _e49.g1_.z, _e52.g1_.w) * vec4<f32>(1.0, 1.0, 1.0, 1.0)) / vec4<f32>(_e62.g1_.x, _e65.g1_.y, _e68.g1_.z, _e71.g1_.w)) * vec4<f32>(1.0, 1.0, 1.0, 1.0)), (((vec4<f32>(_e82.g2_.x, _e85.g2_.y, _e88.g2_.z, _e91.g2_.w) * vec4<f32>(1.0, 1.0, 1.0, 1.0)) / vec4<f32>(_e101.g2_.x, _e104.g2_.y, _e107.g2_.z, _e110.g2_.w)) * vec4<f32>(1.0, 1.0, 1.0, 1.0)), (((vec4<f32>(_e121.g3_.x, _e124.g3_.y, _e127.g3_.z, _e130.g3_.w) * vec4<f32>(1.0, 1.0, 1.0, 1.0)) / vec4<f32>(_e140.g3_.x, _e143.g3_.y, _e146.g3_.z, _e149.g3_.w)) * vec4<f32>(1.0, 1.0, 1.0, 1.0)));
}

fn multi_vector_multi_vector_geometric_product(self_188: MultiVector, other_160: MultiVector) -> MultiVector {
    var self_189: MultiVector;
    var other_161: MultiVector;

    self_189 = self_188;
    other_161 = other_160;
    let _e4: MultiVector = self_189;
    let _e8: MultiVector = other_161;
    let _e11: MultiVector = self_189;
    let _e15: MultiVector = other_161;
    let _e28: MultiVector = self_189;
    let _e32: MultiVector = other_161;
    let _e45: MultiVector = self_189;
    let _e49: MultiVector = other_161;
    let _e62: MultiVector = self_189;
    let _e66: MultiVector = other_161;
    let _e79: MultiVector = self_189;
    let _e83: MultiVector = other_161;
    let _e97: MultiVector = self_189;
    let _e101: MultiVector = other_161;
    let _e115: MultiVector = self_189;
    let _e119: MultiVector = other_161;
    let _e133: MultiVector = self_189;
    let _e137: MultiVector = other_161;
    let _e148: MultiVector = self_189;
    let _e152: MultiVector = other_161;
    let _e164: MultiVector = self_189;
    let _e168: MultiVector = other_161;
    let _e180: MultiVector = self_189;
    let _e184: MultiVector = other_161;
    let _e196: MultiVector = self_189;
    let _e200: MultiVector = other_161;
    let _e213: MultiVector = self_189;
    let _e217: MultiVector = other_161;
    let _e231: MultiVector = self_189;
    let _e235: MultiVector = other_161;
    let _e249: MultiVector = self_189;
    let _e253: MultiVector = other_161;
    let _e267: MultiVector = self_189;
    let _e271: MultiVector = other_161;
    let _e274: MultiVector = self_189;
    let _e278: MultiVector = other_161;
    let _e291: MultiVector = self_189;
    let _e295: MultiVector = other_161;
    let _e308: MultiVector = self_189;
    let _e312: MultiVector = other_161;
    let _e325: MultiVector = self_189;
    let _e329: MultiVector = other_161;
    let _e342: MultiVector = self_189;
    let _e346: MultiVector = other_161;
    let _e358: MultiVector = self_189;
    let _e362: MultiVector = other_161;
    let _e374: MultiVector = self_189;
    let _e378: MultiVector = other_161;
    let _e390: MultiVector = self_189;
    let _e394: MultiVector = other_161;
    let _e398: MultiVector = self_189;
    let _e402: MultiVector = other_161;
    let _e415: MultiVector = self_189;
    let _e419: MultiVector = other_161;
    let _e432: MultiVector = self_189;
    let _e436: MultiVector = other_161;
    let _e449: MultiVector = self_189;
    let _e453: MultiVector = other_161;
    let _e457: MultiVector = self_189;
    let _e461: MultiVector = other_161;
    let _e474: MultiVector = self_189;
    let _e478: MultiVector = other_161;
    let _e491: MultiVector = self_189;
    let _e495: MultiVector = other_161;
    let _e508: MultiVector = self_189;
    let _e512: MultiVector = other_161;
    let _e515: MultiVector = self_189;
    let _e519: MultiVector = other_161;
    let _e532: MultiVector = self_189;
    let _e536: MultiVector = other_161;
    let _e549: MultiVector = self_189;
    let _e553: MultiVector = other_161;
    let _e566: MultiVector = self_189;
    let _e570: MultiVector = other_161;
    let _e574: MultiVector = self_189;
    let _e578: MultiVector = other_161;
    let _e591: MultiVector = self_189;
    let _e595: MultiVector = other_161;
    let _e608: MultiVector = self_189;
    let _e612: MultiVector = other_161;
    let _e625: MultiVector = self_189;
    let _e629: MultiVector = other_161;
    let _e642: MultiVector = self_189;
    let _e646: MultiVector = other_161;
    let _e658: MultiVector = self_189;
    let _e662: MultiVector = other_161;
    let _e674: MultiVector = self_189;
    let _e678: MultiVector = other_161;
    let _e690: MultiVector = self_189;
    let _e694: MultiVector = other_161;
    let _e698: MultiVector = self_189;
    let _e702: MultiVector = other_161;
    let _e715: MultiVector = self_189;
    let _e719: MultiVector = other_161;
    let _e732: MultiVector = self_189;
    let _e736: MultiVector = other_161;
    let _e749: MultiVector = self_189;
    let _e753: MultiVector = other_161;
    let _e756: MultiVector = self_189;
    let _e760: MultiVector = other_161;
    let _e773: MultiVector = self_189;
    let _e777: MultiVector = other_161;
    let _e790: MultiVector = self_189;
    let _e794: MultiVector = other_161;
    let _e807: MultiVector = self_189;
    let _e811: MultiVector = other_161;
    let _e815: MultiVector = self_189;
    let _e819: MultiVector = other_161;
    let _e832: MultiVector = self_189;
    let _e836: MultiVector = other_161;
    let _e849: MultiVector = self_189;
    let _e853: MultiVector = other_161;
    let _e866: MultiVector = self_189;
    let _e870: MultiVector = other_161;
    let _e874: MultiVector = self_189;
    let _e878: MultiVector = other_161;
    let _e891: MultiVector = self_189;
    let _e895: MultiVector = other_161;
    let _e908: MultiVector = self_189;
    let _e912: MultiVector = other_161;
    let _e925: MultiVector = self_189;
    let _e929: MultiVector = other_161;
    let _e942: MultiVector = self_189;
    let _e946: MultiVector = other_161;
    let _e958: MultiVector = self_189;
    let _e962: MultiVector = other_161;
    let _e974: MultiVector = self_189;
    let _e978: MultiVector = other_161;
    return MultiVector(((((((((((((((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.y) * _e15.g0_.yxwz) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e28.g0_.z) * _e32.g0_.zwxy) * vec4<f32>(-(1.0), -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e45.g0_.w) * _e49.g0_.wzyx) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))) + ((vec4<f32>(_e62.g1_.x) * _e66.g1_) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0)))) + ((vec4<f32>(_e79.g1_.y) * _e83.g1_.yxwz) * vec4<f32>(-(1.0), -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e97.g1_.z) * _e101.g1_.zwxy) * vec4<f32>(-(1.0), -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e115.g1_.w) * _e119.g1_.wzyx) * vec4<f32>(-(1.0), 1.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e133.g2_.x) * _e137.g2_) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0))) + ((vec4<f32>(_e148.g2_.y) * _e152.g2_.yxwz) * vec4<f32>(1.0, 1.0, -(1.0), 1.0))) + ((vec4<f32>(_e164.g2_.z) * _e168.g2_.zwxy) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e180.g2_.w) * _e184.g2_.wzyx) * vec4<f32>(1.0, -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e196.g3_.x) * _e200.g3_) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0)))) + ((vec4<f32>(_e213.g3_.y) * _e217.g3_.yxwz) * vec4<f32>(-(1.0), -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e231.g3_.z) * _e235.g3_.zwxy) * vec4<f32>(-(1.0), -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e249.g3_.w) * _e253.g3_.wzyx) * vec4<f32>(-(1.0), 1.0, -(1.0), -(1.0)))), ((((((((((((((((vec4<f32>(_e267.g0_.x) * _e271.g1_) + ((vec4<f32>(_e274.g0_.y) * _e278.g1_.yxwz) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e291.g0_.z) * _e295.g1_.zwxy) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e308.g0_.w) * _e312.g1_.wzyx) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e325.g1_.x) * _e329.g0_) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0)))) + ((vec4<f32>(_e342.g1_.y) * _e346.g0_.yxwz) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e358.g1_.z) * _e362.g0_.zwxy) * vec4<f32>(1.0, -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e374.g1_.w) * _e378.g0_.wzyx) * vec4<f32>(1.0, 1.0, -(1.0), 1.0))) + (vec4<f32>(_e390.g2_.x) * _e394.g3_)) + ((vec4<f32>(_e398.g2_.y) * _e402.g3_.yxwz) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))) + ((vec4<f32>(_e415.g2_.z) * _e419.g3_.zwxy) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e432.g2_.w) * _e436.g3_.wzyx) * vec4<f32>(-(1.0), -(1.0), 1.0, 1.0))) - (vec4<f32>(_e449.g3_.x) * _e453.g2_)) + ((vec4<f32>(_e457.g3_.y) * _e461.g2_.yxwz) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e474.g3_.z) * _e478.g2_.zwxy) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e491.g3_.w) * _e495.g2_.wzyx) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))), ((((((((((((((((vec4<f32>(_e508.g0_.x) * _e512.g2_) + ((vec4<f32>(_e515.g0_.y) * _e519.g2_.yxwz) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e532.g0_.z) * _e536.g2_.zwxy) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e549.g0_.w) * _e553.g2_.wzyx) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))) + (vec4<f32>(_e566.g1_.x) * _e570.g3_)) + ((vec4<f32>(_e574.g1_.y) * _e578.g3_.yxwz) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))) + ((vec4<f32>(_e591.g1_.z) * _e595.g3_.zwxy) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e608.g1_.w) * _e612.g3_.wzyx) * vec4<f32>(-(1.0), -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e625.g2_.x) * _e629.g0_) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0)))) + ((vec4<f32>(_e642.g2_.y) * _e646.g0_.yxwz) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e658.g2_.z) * _e662.g0_.zwxy) * vec4<f32>(1.0, -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e674.g2_.w) * _e678.g0_.wzyx) * vec4<f32>(1.0, 1.0, -(1.0), 1.0))) - (vec4<f32>(_e690.g3_.x) * _e694.g1_)) + ((vec4<f32>(_e698.g3_.y) * _e702.g1_.yxwz) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e715.g3_.z) * _e719.g1_.zwxy) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e732.g3_.w) * _e736.g1_.wzyx) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))), ((((((((((((((((vec4<f32>(_e749.g0_.x) * _e753.g3_) + ((vec4<f32>(_e756.g0_.y) * _e760.g3_.yxwz) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e773.g0_.z) * _e777.g3_.zwxy) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e790.g0_.w) * _e794.g3_.wzyx) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))) + (vec4<f32>(_e807.g1_.x) * _e811.g2_)) + ((vec4<f32>(_e815.g1_.y) * _e819.g2_.yxwz) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))) + ((vec4<f32>(_e832.g1_.z) * _e836.g2_.zwxy) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e849.g1_.w) * _e853.g2_.wzyx) * vec4<f32>(-(1.0), -(1.0), 1.0, 1.0))) - (vec4<f32>(_e866.g2_.x) * _e870.g1_)) + ((vec4<f32>(_e874.g2_.y) * _e878.g1_.yxwz) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e891.g2_.z) * _e895.g1_.zwxy) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e908.g2_.w) * _e912.g1_.wzyx) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e925.g3_.x) * _e929.g0_) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0)))) + ((vec4<f32>(_e942.g3_.y) * _e946.g0_.yxwz) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e958.g3_.z) * _e962.g0_.zwxy) * vec4<f32>(1.0, -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e974.g3_.w) * _e978.g0_.wzyx) * vec4<f32>(1.0, 1.0, -(1.0), 1.0))));
}

fn multi_vector_multi_vector_regressive_product(self_190: MultiVector, other_162: MultiVector) -> MultiVector {
    var self_191: MultiVector;
    var other_163: MultiVector;

    self_191 = self_190;
    other_163 = other_162;
    let _e4: MultiVector = self_191;
    let _e8: MultiVector = other_163;
    let _e18: MultiVector = self_191;
    let _e22: MultiVector = other_163;
    let _e33: MultiVector = self_191;
    let _e37: MultiVector = other_163;
    let _e48: MultiVector = self_191;
    let _e52: MultiVector = other_163;
    let _e64: MultiVector = self_191;
    let _e68: MultiVector = other_163;
    let _e81: MultiVector = self_191;
    let _e85: MultiVector = other_163;
    let _e98: MultiVector = self_191;
    let _e102: MultiVector = other_163;
    let _e115: MultiVector = self_191;
    let _e119: MultiVector = other_163;
    let _e130: MultiVector = self_191;
    let _e134: MultiVector = other_163;
    let _e146: MultiVector = self_191;
    let _e150: MultiVector = other_163;
    let _e162: MultiVector = self_191;
    let _e166: MultiVector = other_163;
    let _e178: MultiVector = self_191;
    let _e182: MultiVector = other_163;
    let _e186: MultiVector = self_191;
    let _e190: MultiVector = other_163;
    let _e202: MultiVector = self_191;
    let _e206: MultiVector = other_163;
    let _e218: MultiVector = self_191;
    let _e222: MultiVector = other_163;
    let _e234: MultiVector = self_191;
    let _e238: MultiVector = other_163;
    let _e250: MultiVector = self_191;
    let _e254: MultiVector = other_163;
    let _e265: MultiVector = self_191;
    let _e269: MultiVector = other_163;
    let _e281: MultiVector = self_191;
    let _e285: MultiVector = other_163;
    let _e297: MultiVector = self_191;
    let _e301: MultiVector = other_163;
    let _e305: MultiVector = self_191;
    let _e309: MultiVector = other_163;
    let _e322: MultiVector = self_191;
    let _e326: MultiVector = other_163;
    let _e339: MultiVector = self_191;
    let _e343: MultiVector = other_163;
    let _e356: MultiVector = self_191;
    let _e360: MultiVector = other_163;
    let _e372: MultiVector = self_191;
    let _e376: MultiVector = other_163;
    let _e387: MultiVector = self_191;
    let _e391: MultiVector = other_163;
    let _e403: MultiVector = self_191;
    let _e407: MultiVector = other_163;
    let _e419: MultiVector = self_191;
    let _e423: MultiVector = other_163;
    let _e435: MultiVector = self_191;
    let _e439: MultiVector = other_163;
    let _e451: MultiVector = self_191;
    let _e455: MultiVector = other_163;
    let _e459: MultiVector = self_191;
    let _e463: MultiVector = other_163;
    let _e475: MultiVector = self_191;
    let _e479: MultiVector = other_163;
    let _e491: MultiVector = self_191;
    let _e495: MultiVector = other_163;
    let _e499: MultiVector = self_191;
    let _e503: MultiVector = other_163;
    let _e515: MultiVector = self_191;
    let _e519: MultiVector = other_163;
    let _e531: MultiVector = self_191;
    let _e535: MultiVector = other_163;
    let _e547: MultiVector = self_191;
    let _e550: MultiVector = self_191;
    let _e553: MultiVector = self_191;
    let _e556: MultiVector = self_191;
    let _e560: MultiVector = other_163;
    let _e563: MultiVector = other_163;
    let _e566: MultiVector = other_163;
    let _e569: MultiVector = other_163;
    let _e582: MultiVector = self_191;
    let _e586: MultiVector = other_163;
    let _e597: MultiVector = self_191;
    let _e601: MultiVector = other_163;
    let _e613: MultiVector = self_191;
    let _e617: MultiVector = other_163;
    let _e621: MultiVector = self_191;
    let _e625: MultiVector = other_163;
    let _e637: MultiVector = self_191;
    let _e641: MultiVector = other_163;
    let _e653: MultiVector = self_191;
    let _e656: MultiVector = self_191;
    let _e659: MultiVector = self_191;
    let _e662: MultiVector = self_191;
    let _e666: MultiVector = other_163;
    let _e669: MultiVector = other_163;
    let _e672: MultiVector = other_163;
    let _e675: MultiVector = other_163;
    return MultiVector((((((((((((((((((vec4<f32>(_e4.g0_.y) * _e8.g3_.yxyy) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) + ((vec4<f32>(_e18.g0_.z) * _e22.g3_.zzxz) * vec4<f32>(1.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e33.g0_.w) * _e37.g3_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e48.g1_.x) * vec4<f32>(_e52.g2_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e64.g1_.y) * _e68.g2_.yxyy) * vec4<f32>(-(1.0), -(1.0), 0.0, 0.0))) + ((vec4<f32>(_e81.g1_.z) * _e85.g2_.zzxz) * vec4<f32>(-(1.0), 0.0, -(1.0), 0.0))) + ((vec4<f32>(_e98.g1_.w) * _e102.g2_.wwwx) * vec4<f32>(-(1.0), 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e115.g2_.x) * _e119.g1_) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0))) + ((vec4<f32>(_e130.g2_.y) * vec4<f32>(_e134.g1_.y)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e146.g2_.z) * vec4<f32>(_e150.g1_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e162.g2_.w) * vec4<f32>(_e166.g1_.w)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + (vec4<f32>(_e178.g3_.x) * _e182.g0_)) + ((vec4<f32>(_e186.g3_.y) * vec4<f32>(_e190.g0_.y)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e202.g3_.z) * vec4<f32>(_e206.g0_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e218.g3_.w) * vec4<f32>(_e222.g0_.w)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e234.g0_.x) * vec4<f32>(_e238.g3_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))), (((((((((vec4<f32>(_e250.g1_.y) * _e254.g3_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0)) + ((vec4<f32>(_e265.g1_.z) * _e269.g3_.zzxz) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0))) + ((vec4<f32>(_e281.g1_.w) * _e285.g3_.wwwx) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))) + (vec4<f32>(_e297.g3_.x) * _e301.g1_)) + ((vec4<f32>(_e305.g3_.y) * vec4<f32>(_e309.g1_.y)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((vec4<f32>(_e322.g3_.z) * vec4<f32>(_e326.g1_.z)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((vec4<f32>(_e339.g3_.w) * vec4<f32>(_e343.g1_.w)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((vec4<f32>(_e356.g1_.x) * vec4<f32>(_e360.g3_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))), ((((((((((((((vec4<f32>(_e372.g0_.z) * _e376.g1_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e387.g0_.w) * _e391.g1_.zzyz) * vec4<f32>(0.0, -(1.0), 1.0, 0.0))) + ((vec4<f32>(_e403.g1_.y) * _e407.g0_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e419.g1_.z) * _e423.g0_.wwwy) * vec4<f32>(0.0, -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e435.g1_.w) * _e439.g0_.zzyz) * vec4<f32>(0.0, 1.0, -(1.0), 0.0))) + (vec4<f32>(_e451.g2_.x) * _e455.g3_)) + ((vec4<f32>(_e459.g2_.z) * vec4<f32>(_e463.g3_.x)) * vec4<f32>(0.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e475.g2_.w) * vec4<f32>(_e479.g3_.x)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + (vec4<f32>(_e491.g3_.x) * _e495.g2_)) + ((vec4<f32>(_e499.g3_.y) * vec4<f32>(_e503.g2_.x)) * vec4<f32>(0.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e515.g3_.z) * vec4<f32>(_e519.g2_.x)) * vec4<f32>(0.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e531.g3_.w) * vec4<f32>(_e535.g2_.x)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e547.g0_.x, _e550.g2_.y, _e553.g0_.y, _e556.g0_.y) * vec4<f32>(_e560.g1_.x, _e563.g3_.x, _e566.g1_.w, _e569.g1_.z)) * vec4<f32>(0.0, 1.0, -(1.0), 1.0))), (((((((vec4<f32>(_e582.g1_.z) * _e586.g1_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e597.g1_.w) * _e601.g1_.zzyz) * vec4<f32>(0.0, -(1.0), 1.0, 0.0))) + (vec4<f32>(_e613.g3_.x) * _e617.g3_)) + ((vec4<f32>(_e621.g3_.z) * vec4<f32>(_e625.g3_.x)) * vec4<f32>(0.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e637.g3_.w) * vec4<f32>(_e641.g3_.x)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e653.g1_.x, _e656.g3_.y, _e659.g1_.y, _e662.g1_.y) * vec4<f32>(_e666.g1_.x, _e669.g3_.x, _e672.g1_.w, _e675.g1_.z)) * vec4<f32>(0.0, 1.0, -(1.0), 1.0))));
}

fn multi_vector_multi_vector_outer_product(self_192: MultiVector, other_164: MultiVector) -> MultiVector {
    var self_193: MultiVector;
    var other_165: MultiVector;

    self_193 = self_192;
    other_165 = other_164;
    let _e4: MultiVector = self_193;
    let _e8: MultiVector = other_165;
    let _e11: MultiVector = self_193;
    let _e15: MultiVector = other_165;
    let _e27: MultiVector = self_193;
    let _e31: MultiVector = other_165;
    let _e43: MultiVector = self_193;
    let _e47: MultiVector = other_165;
    let _e59: MultiVector = self_193;
    let _e61: MultiVector = other_165;
    let _e73: MultiVector = self_193;
    let _e77: MultiVector = other_165;
    let _e80: MultiVector = self_193;
    let _e84: MultiVector = other_165;
    let _e97: MultiVector = self_193;
    let _e101: MultiVector = other_165;
    let _e113: MultiVector = self_193;
    let _e117: MultiVector = other_165;
    let _e129: MultiVector = self_193;
    let _e133: MultiVector = other_165;
    let _e145: MultiVector = self_193;
    let _e149: MultiVector = other_165;
    let _e161: MultiVector = self_193;
    let _e165: MultiVector = other_165;
    let _e177: MultiVector = self_193;
    let _e181: MultiVector = other_165;
    let _e193: MultiVector = self_193;
    let _e197: MultiVector = other_165;
    let _e209: MultiVector = self_193;
    let _e213: MultiVector = other_165;
    let _e225: MultiVector = self_193;
    let _e229: MultiVector = other_165;
    let _e241: MultiVector = self_193;
    let _e243: MultiVector = other_165;
    let _e258: MultiVector = self_193;
    let _e262: MultiVector = other_165;
    let _e265: MultiVector = self_193;
    let _e269: MultiVector = other_165;
    let _e281: MultiVector = self_193;
    let _e285: MultiVector = other_165;
    let _e297: MultiVector = self_193;
    let _e301: MultiVector = other_165;
    let _e313: MultiVector = self_193;
    let _e317: MultiVector = other_165;
    let _e328: MultiVector = self_193;
    let _e332: MultiVector = other_165;
    let _e343: MultiVector = self_193;
    let _e347: MultiVector = other_165;
    let _e358: MultiVector = self_193;
    let _e361: MultiVector = other_165;
    let _e372: MultiVector = self_193;
    let _e376: MultiVector = other_165;
    let _e379: MultiVector = self_193;
    let _e383: MultiVector = other_165;
    let _e395: MultiVector = self_193;
    let _e399: MultiVector = other_165;
    let _e411: MultiVector = self_193;
    let _e415: MultiVector = other_165;
    let _e419: MultiVector = self_193;
    let _e423: MultiVector = other_165;
    let _e436: MultiVector = self_193;
    let _e440: MultiVector = other_165;
    let _e453: MultiVector = self_193;
    let _e457: MultiVector = other_165;
    let _e470: MultiVector = self_193;
    let _e474: MultiVector = other_165;
    let _e487: MultiVector = self_193;
    let _e491: MultiVector = other_165;
    let _e503: MultiVector = self_193;
    let _e507: MultiVector = other_165;
    let _e519: MultiVector = self_193;
    let _e523: MultiVector = other_165;
    let _e535: MultiVector = self_193;
    let _e539: MultiVector = other_165;
    let _e551: MultiVector = self_193;
    let _e555: MultiVector = other_165;
    let _e566: MultiVector = self_193;
    let _e570: MultiVector = other_165;
    let _e581: MultiVector = self_193;
    let _e585: MultiVector = other_165;
    let _e596: MultiVector = self_193;
    let _e599: MultiVector = other_165;
    return MultiVector((((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g2_.y) * _e15.g2_.wwwz) * vec4<f32>(0.0, 0.0, -(1.0), 1.0))) + ((vec4<f32>(_e27.g2_.z) * _e31.g2_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0)))) + ((vec4<f32>(_e43.g2_.w) * _e47.g2_.zzyz) * vec4<f32>(0.0, -(1.0), 1.0, 0.0))) + ((_e59.g0_ * vec4<f32>(_e61.g0_.x)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))), ((((((((((((vec4<f32>(_e73.g0_.x) * _e77.g1_) + ((vec4<f32>(_e80.g1_.x) * _e84.g0_) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0)))) + ((vec4<f32>(_e97.g1_.y) * vec4<f32>(_e101.g0_.x)) * vec4<f32>(0.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e113.g1_.z) * vec4<f32>(_e117.g0_.x)) * vec4<f32>(0.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e129.g1_.w) * vec4<f32>(_e133.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e145.g2_.y) * _e149.g3_.wwwz) * vec4<f32>(0.0, 0.0, -(1.0), 1.0))) + ((vec4<f32>(_e161.g2_.z) * _e165.g3_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0)))) + ((vec4<f32>(_e177.g2_.w) * _e181.g3_.zzyz) * vec4<f32>(0.0, -(1.0), 1.0, 0.0))) + ((vec4<f32>(_e193.g3_.y) * _e197.g2_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e209.g3_.z) * _e213.g2_.wwwy) * vec4<f32>(0.0, -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e225.g3_.w) * _e229.g2_.zzyz) * vec4<f32>(0.0, 1.0, -(1.0), 0.0))) + ((_e241.g0_ * vec4<f32>(_e243.g1_.x)) * vec4<f32>(0.0, -(1.0), -(1.0), -(1.0)))), ((((((((vec4<f32>(_e258.g0_.x) * _e262.g2_) + ((vec4<f32>(_e265.g0_.z) * vec4<f32>(_e269.g2_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e281.g0_.w) * vec4<f32>(_e285.g2_.w)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e297.g2_.x) * vec4<f32>(_e301.g0_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e313.g2_.y) * _e317.g0_.yxyy) * vec4<f32>(1.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e328.g2_.z) * _e332.g0_.zzxz) * vec4<f32>(1.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e343.g2_.w) * _e347.g0_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, 1.0))) + ((_e358.g0_.yxxx * _e361.g2_.yxxx) * vec4<f32>(1.0, 0.0, 0.0, 0.0))), ((((((((((((((((vec4<f32>(_e372.g0_.x) * _e376.g3_) + ((vec4<f32>(_e379.g0_.z) * vec4<f32>(_e383.g3_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e395.g0_.w) * vec4<f32>(_e399.g3_.w)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + (vec4<f32>(_e411.g1_.x) * _e415.g2_)) + ((vec4<f32>(_e419.g1_.y) * vec4<f32>(_e423.g2_.y)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((vec4<f32>(_e436.g1_.z) * vec4<f32>(_e440.g2_.z)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((vec4<f32>(_e453.g1_.w) * vec4<f32>(_e457.g2_.w)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((vec4<f32>(_e470.g2_.x) * vec4<f32>(_e474.g1_.x)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((vec4<f32>(_e487.g2_.y) * _e491.g1_.yxyy) * vec4<f32>(1.0, -(1.0), 0.0, 0.0))) + ((vec4<f32>(_e503.g2_.z) * _e507.g1_.zzxz) * vec4<f32>(1.0, 0.0, -(1.0), 0.0))) + ((vec4<f32>(_e519.g2_.w) * _e523.g1_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e535.g3_.x) * vec4<f32>(_e539.g0_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e551.g3_.y) * _e555.g0_.yxyy) * vec4<f32>(1.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e566.g3_.z) * _e570.g0_.zzxz) * vec4<f32>(1.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e581.g3_.w) * _e585.g0_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, 1.0))) + ((_e596.g0_.yxxx * _e599.g3_.yxxx) * vec4<f32>(1.0, 0.0, 0.0, 0.0))));
}

fn multi_vector_multi_vector_inner_product(self_194: MultiVector, other_166: MultiVector) -> MultiVector {
    var self_195: MultiVector;
    var other_167: MultiVector;

    self_195 = self_194;
    other_167 = other_166;
    let _e4: MultiVector = self_195;
    let _e8: MultiVector = other_167;
    let _e11: MultiVector = self_195;
    let _e15: MultiVector = other_167;
    let _e27: MultiVector = self_195;
    let _e31: MultiVector = other_167;
    let _e43: MultiVector = self_195;
    let _e47: MultiVector = other_167;
    let _e60: MultiVector = self_195;
    let _e64: MultiVector = other_167;
    let _e77: MultiVector = self_195;
    let _e81: MultiVector = other_167;
    let _e94: MultiVector = self_195;
    let _e98: MultiVector = other_167;
    let _e111: MultiVector = self_195;
    let _e115: MultiVector = other_167;
    let _e126: MultiVector = self_195;
    let _e130: MultiVector = other_167;
    let _e141: MultiVector = self_195;
    let _e145: MultiVector = other_167;
    let _e156: MultiVector = self_195;
    let _e160: MultiVector = other_167;
    let _e171: MultiVector = self_195;
    let _e175: MultiVector = other_167;
    let _e188: MultiVector = self_195;
    let _e192: MultiVector = other_167;
    let _e205: MultiVector = self_195;
    let _e209: MultiVector = other_167;
    let _e222: MultiVector = self_195;
    let _e226: MultiVector = other_167;
    let _e239: MultiVector = self_195;
    let _e242: MultiVector = other_167;
    let _e254: MultiVector = self_195;
    let _e258: MultiVector = other_167;
    let _e261: MultiVector = self_195;
    let _e265: MultiVector = other_167;
    let _e277: MultiVector = self_195;
    let _e281: MultiVector = other_167;
    let _e293: MultiVector = self_195;
    let _e297: MultiVector = other_167;
    let _e309: MultiVector = self_195;
    let _e313: MultiVector = other_167;
    let _e324: MultiVector = self_195;
    let _e328: MultiVector = other_167;
    let _e339: MultiVector = self_195;
    let _e343: MultiVector = other_167;
    let _e354: MultiVector = self_195;
    let _e358: MultiVector = other_167;
    let _e370: MultiVector = self_195;
    let _e374: MultiVector = other_167;
    let _e386: MultiVector = self_195;
    let _e390: MultiVector = other_167;
    let _e402: MultiVector = self_195;
    let _e406: MultiVector = other_167;
    let _e418: MultiVector = self_195;
    let _e422: MultiVector = other_167;
    let _e426: MultiVector = self_195;
    let _e430: MultiVector = other_167;
    let _e442: MultiVector = self_195;
    let _e446: MultiVector = other_167;
    let _e458: MultiVector = self_195;
    let _e462: MultiVector = other_167;
    let _e474: MultiVector = self_195;
    let _e477: MultiVector = other_167;
    let _e488: MultiVector = self_195;
    let _e492: MultiVector = other_167;
    let _e495: MultiVector = self_195;
    let _e499: MultiVector = other_167;
    let _e512: MultiVector = self_195;
    let _e516: MultiVector = other_167;
    let _e529: MultiVector = self_195;
    let _e533: MultiVector = other_167;
    let _e537: MultiVector = self_195;
    let _e541: MultiVector = other_167;
    let _e553: MultiVector = self_195;
    let _e557: MultiVector = other_167;
    let _e569: MultiVector = self_195;
    let _e573: MultiVector = other_167;
    let _e585: MultiVector = self_195;
    let _e589: MultiVector = other_167;
    let _e602: MultiVector = self_195;
    let _e606: MultiVector = other_167;
    let _e618: MultiVector = self_195;
    let _e622: MultiVector = other_167;
    let _e634: MultiVector = self_195;
    let _e638: MultiVector = other_167;
    let _e650: MultiVector = self_195;
    let _e654: MultiVector = other_167;
    let _e658: MultiVector = self_195;
    let _e662: MultiVector = other_167;
    let _e675: MultiVector = self_195;
    let _e679: MultiVector = other_167;
    let _e692: MultiVector = self_195;
    let _e696: MultiVector = other_167;
    let _e709: MultiVector = self_195;
    let _e712: MultiVector = other_167;
    let _e725: MultiVector = self_195;
    let _e729: MultiVector = other_167;
    let _e732: MultiVector = self_195;
    let _e736: MultiVector = other_167;
    let _e748: MultiVector = self_195;
    let _e752: MultiVector = other_167;
    let _e764: MultiVector = self_195;
    let _e768: MultiVector = other_167;
    let _e780: MultiVector = self_195;
    let _e784: MultiVector = other_167;
    let _e796: MultiVector = self_195;
    let _e800: MultiVector = other_167;
    let _e812: MultiVector = self_195;
    let _e816: MultiVector = other_167;
    let _e828: MultiVector = self_195;
    let _e832: MultiVector = other_167;
    let _e845: MultiVector = self_195;
    let _e849: MultiVector = other_167;
    let _e861: MultiVector = self_195;
    let _e865: MultiVector = other_167;
    let _e877: MultiVector = self_195;
    let _e881: MultiVector = other_167;
    let _e893: MultiVector = self_195;
    let _e895: MultiVector = other_167;
    return MultiVector(((((((((((((((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.z) * _e15.g0_.zzxz) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0))) + ((vec4<f32>(_e27.g0_.w) * _e31.g0_.wwwx) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))) + ((vec4<f32>(_e43.g1_.x) * _e47.g1_) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0)))) + ((vec4<f32>(_e60.g1_.y) * _e64.g1_.yxyy) * vec4<f32>(-(1.0), -(1.0), 0.0, 0.0))) + ((vec4<f32>(_e77.g1_.z) * _e81.g1_.zzxz) * vec4<f32>(-(1.0), 0.0, -(1.0), 0.0))) + ((vec4<f32>(_e94.g1_.w) * _e98.g1_.wwwx) * vec4<f32>(-(1.0), 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e111.g2_.x) * _e115.g2_) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0))) + ((vec4<f32>(_e126.g2_.y) * _e130.g2_.yxyy) * vec4<f32>(1.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e141.g2_.z) * _e145.g2_.zzxz) * vec4<f32>(1.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e156.g2_.w) * _e160.g2_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e171.g3_.x) * _e175.g3_) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0)))) + ((vec4<f32>(_e188.g3_.y) * _e192.g3_.yxyy) * vec4<f32>(-(1.0), -(1.0), 0.0, 0.0))) + ((vec4<f32>(_e205.g3_.z) * _e209.g3_.zzxz) * vec4<f32>(-(1.0), 0.0, -(1.0), 0.0))) + ((vec4<f32>(_e222.g3_.w) * _e226.g3_.wwwx) * vec4<f32>(-(1.0), 0.0, 0.0, -(1.0)))) + ((_e239.g0_.yyxx * _e242.g0_.yxxx) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))), ((((((((((((((((vec4<f32>(_e254.g0_.x) * _e258.g1_) + ((vec4<f32>(_e261.g0_.z) * vec4<f32>(_e265.g1_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e277.g0_.w) * vec4<f32>(_e281.g1_.w)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e293.g1_.x) * vec4<f32>(_e297.g0_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e309.g1_.y) * _e313.g0_.yxyy) * vec4<f32>(1.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e324.g1_.z) * _e328.g0_.zzxz) * vec4<f32>(1.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e339.g1_.w) * _e343.g0_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e354.g2_.x) * vec4<f32>(_e358.g3_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e370.g2_.y) * _e374.g3_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))) + ((vec4<f32>(_e386.g2_.z) * _e390.g3_.zzxz) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0))) + ((vec4<f32>(_e402.g2_.w) * _e406.g3_.wwwx) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))) - (vec4<f32>(_e418.g3_.x) * _e422.g2_)) + ((vec4<f32>(_e426.g3_.y) * vec4<f32>(_e430.g2_.y)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e442.g3_.z) * vec4<f32>(_e446.g2_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e458.g3_.w) * vec4<f32>(_e462.g2_.w)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((_e474.g0_.yxxx * _e477.g1_.yxxx) * vec4<f32>(1.0, 0.0, 0.0, 0.0))), ((((((((((((((((vec4<f32>(_e488.g0_.x) * _e492.g2_) + ((vec4<f32>(_e495.g0_.z) * _e499.g2_.wwxy) * vec4<f32>(0.0, -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e512.g0_.w) * _e516.g2_.zzyx) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))) + (vec4<f32>(_e529.g1_.x) * _e533.g3_)) + ((vec4<f32>(_e537.g1_.y) * _e541.g3_.xxwz) * vec4<f32>(0.0, 1.0, -(1.0), 1.0))) + ((vec4<f32>(_e553.g1_.z) * _e557.g3_.wwxy) * vec4<f32>(0.0, 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e569.g1_.w) * _e573.g3_.zzyx) * vec4<f32>(0.0, -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e585.g2_.x) * _e589.g0_) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0)))) + ((vec4<f32>(_e602.g2_.y) * _e606.g0_.xxwz) * vec4<f32>(0.0, 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e618.g2_.z) * _e622.g0_.wwxy) * vec4<f32>(0.0, -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e634.g2_.w) * _e638.g0_.zzyx) * vec4<f32>(0.0, 1.0, -(1.0), 1.0))) - (vec4<f32>(_e650.g3_.x) * _e654.g1_)) + ((vec4<f32>(_e658.g3_.y) * _e662.g1_.xxwz) * vec4<f32>(0.0, -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e675.g3_.z) * _e679.g1_.wwxy) * vec4<f32>(0.0, -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e692.g3_.w) * _e696.g1_.zzyx) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))) + ((_e709.g0_.xyyy * _e712.g2_.xxwz) * vec4<f32>(0.0, -(1.0), 1.0, -(1.0)))), ((((((((((((vec4<f32>(_e725.g0_.x) * _e729.g3_) + ((vec4<f32>(_e732.g1_.y) * _e736.g2_.wwwz) * vec4<f32>(0.0, 0.0, -(1.0), 1.0))) + ((vec4<f32>(_e748.g1_.z) * _e752.g2_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0)))) + ((vec4<f32>(_e764.g1_.w) * _e768.g2_.zzyz) * vec4<f32>(0.0, -(1.0), 1.0, 0.0))) + ((vec4<f32>(_e780.g2_.y) * _e784.g1_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e796.g2_.z) * _e800.g1_.wwwy) * vec4<f32>(0.0, -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e812.g2_.w) * _e816.g1_.zzyz) * vec4<f32>(0.0, 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e828.g3_.x) * _e832.g0_) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0)))) + ((vec4<f32>(_e845.g3_.y) * vec4<f32>(_e849.g0_.x)) * vec4<f32>(0.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e861.g3_.z) * vec4<f32>(_e865.g0_.x)) * vec4<f32>(0.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e877.g3_.w) * vec4<f32>(_e881.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((_e893.g0_ * vec4<f32>(_e895.g3_.x)) * vec4<f32>(0.0, -(1.0), -(1.0), -(1.0)))));
}

fn multi_vector_multi_vector_left_contraction(self_196: MultiVector, other_168: MultiVector) -> MultiVector {
    var self_197: MultiVector;
    var other_169: MultiVector;

    self_197 = self_196;
    other_169 = other_168;
    let _e4: MultiVector = self_197;
    let _e8: MultiVector = other_169;
    let _e11: MultiVector = self_197;
    let _e15: MultiVector = other_169;
    let _e28: MultiVector = self_197;
    let _e32: MultiVector = other_169;
    let _e45: MultiVector = self_197;
    let _e49: MultiVector = other_169;
    let _e62: MultiVector = self_197;
    let _e66: MultiVector = other_169;
    let _e79: MultiVector = self_197;
    let _e83: MultiVector = other_169;
    let _e96: MultiVector = self_197;
    let _e100: MultiVector = other_169;
    let _e113: MultiVector = self_197;
    let _e117: MultiVector = other_169;
    let _e130: MultiVector = self_197;
    let _e134: MultiVector = other_169;
    let _e145: MultiVector = self_197;
    let _e149: MultiVector = other_169;
    let _e160: MultiVector = self_197;
    let _e164: MultiVector = other_169;
    let _e175: MultiVector = self_197;
    let _e179: MultiVector = other_169;
    let _e191: MultiVector = self_197;
    let _e195: MultiVector = other_169;
    let _e208: MultiVector = self_197;
    let _e212: MultiVector = other_169;
    let _e225: MultiVector = self_197;
    let _e229: MultiVector = other_169;
    let _e242: MultiVector = self_197;
    let _e245: MultiVector = other_169;
    let _e257: MultiVector = self_197;
    let _e261: MultiVector = other_169;
    let _e264: MultiVector = self_197;
    let _e268: MultiVector = other_169;
    let _e280: MultiVector = self_197;
    let _e284: MultiVector = other_169;
    let _e296: MultiVector = self_197;
    let _e300: MultiVector = other_169;
    let _e312: MultiVector = self_197;
    let _e316: MultiVector = other_169;
    let _e328: MultiVector = self_197;
    let _e332: MultiVector = other_169;
    let _e344: MultiVector = self_197;
    let _e348: MultiVector = other_169;
    let _e360: MultiVector = self_197;
    let _e363: MultiVector = other_169;
    let _e374: MultiVector = self_197;
    let _e378: MultiVector = other_169;
    let _e381: MultiVector = self_197;
    let _e385: MultiVector = other_169;
    let _e389: MultiVector = self_197;
    let _e393: MultiVector = other_169;
    let _e405: MultiVector = self_197;
    let _e409: MultiVector = other_169;
    let _e421: MultiVector = self_197;
    let _e425: MultiVector = other_169;
    let _e437: MultiVector = self_197;
    let _e441: MultiVector = other_169;
    let _e453: MultiVector = self_197;
    let _e457: MultiVector = other_169;
    let _e469: MultiVector = self_197;
    let _e473: MultiVector = other_169;
    let _e485: MultiVector = self_197;
    let _e489: MultiVector = other_169;
    let _e501: MultiVector = self_197;
    let _e505: MultiVector = other_169;
    let _e517: MultiVector = self_197;
    let _e521: MultiVector = other_169;
    let _e533: MultiVector = self_197;
    let _e535: MultiVector = other_169;
    let _e550: MultiVector = self_197;
    let _e554: MultiVector = other_169;
    let _e557: MultiVector = self_197;
    let _e561: MultiVector = other_169;
    let _e573: MultiVector = self_197;
    let _e577: MultiVector = other_169;
    let _e589: MultiVector = self_197;
    let _e593: MultiVector = other_169;
    let _e605: MultiVector = self_197;
    let _e607: MultiVector = other_169;
    return MultiVector(((((((((((((((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.z) * vec4<f32>(_e15.g0_.z)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((vec4<f32>(_e28.g0_.w) * vec4<f32>(_e32.g0_.w)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((vec4<f32>(_e45.g1_.x) * _e49.g1_) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0)))) + ((vec4<f32>(_e62.g1_.y) * vec4<f32>(_e66.g1_.y)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((vec4<f32>(_e79.g1_.z) * vec4<f32>(_e83.g1_.z)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((vec4<f32>(_e96.g1_.w) * vec4<f32>(_e100.g1_.w)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((vec4<f32>(_e113.g2_.x) * vec4<f32>(_e117.g2_.x)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((vec4<f32>(_e130.g2_.y) * _e134.g2_.yxyy) * vec4<f32>(1.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e145.g2_.z) * _e149.g2_.zzxz) * vec4<f32>(1.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e160.g2_.w) * _e164.g2_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e175.g3_.x) * vec4<f32>(_e179.g3_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e191.g3_.y) * _e195.g3_.yxyy) * vec4<f32>(-(1.0), -(1.0), 0.0, 0.0))) + ((vec4<f32>(_e208.g3_.z) * _e212.g3_.zzxz) * vec4<f32>(-(1.0), 0.0, -(1.0), 0.0))) + ((vec4<f32>(_e225.g3_.w) * _e229.g3_.wwwx) * vec4<f32>(-(1.0), 0.0, 0.0, -(1.0)))) + ((_e242.g0_.yxxx * _e245.g0_.yxxx) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))), ((((((((vec4<f32>(_e257.g0_.x) * _e261.g1_) + ((vec4<f32>(_e264.g0_.z) * vec4<f32>(_e268.g1_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e280.g0_.w) * vec4<f32>(_e284.g1_.w)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e296.g2_.x) * vec4<f32>(_e300.g3_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e312.g2_.y) * _e316.g3_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))) + ((vec4<f32>(_e328.g2_.z) * _e332.g3_.zzxz) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0))) + ((vec4<f32>(_e344.g2_.w) * _e348.g3_.wwwx) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))) + ((_e360.g0_.yxxx * _e363.g1_.yxxx) * vec4<f32>(1.0, 0.0, 0.0, 0.0))), ((((((((((((vec4<f32>(_e374.g0_.x) * _e378.g2_) + (vec4<f32>(_e381.g1_.x) * _e385.g3_)) + ((vec4<f32>(_e389.g1_.y) * vec4<f32>(_e393.g3_.x)) * vec4<f32>(0.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e405.g1_.z) * vec4<f32>(_e409.g3_.x)) * vec4<f32>(0.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e421.g1_.w) * vec4<f32>(_e425.g3_.x)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e437.g2_.y) * _e441.g0_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e453.g2_.z) * _e457.g0_.wwwy) * vec4<f32>(0.0, -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e469.g2_.w) * _e473.g0_.zzyz) * vec4<f32>(0.0, 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e485.g3_.y) * _e489.g1_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e501.g3_.z) * _e505.g1_.wwwy) * vec4<f32>(0.0, -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e517.g3_.w) * _e521.g1_.zzyz) * vec4<f32>(0.0, 1.0, -(1.0), 0.0))) + ((_e533.g0_ * vec4<f32>(_e535.g2_.x)) * vec4<f32>(0.0, -(1.0), -(1.0), -(1.0)))), (((((vec4<f32>(_e550.g0_.x) * _e554.g3_) + ((vec4<f32>(_e557.g2_.y) * _e561.g1_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e573.g2_.z) * _e577.g1_.wwwy) * vec4<f32>(0.0, -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e589.g2_.w) * _e593.g1_.zzyz) * vec4<f32>(0.0, 1.0, -(1.0), 0.0))) + ((_e605.g0_ * vec4<f32>(_e607.g3_.x)) * vec4<f32>(0.0, -(1.0), -(1.0), -(1.0)))));
}

fn multi_vector_multi_vector_right_contraction(self_198: MultiVector, other_170: MultiVector) -> MultiVector {
    var self_199: MultiVector;
    var other_171: MultiVector;

    self_199 = self_198;
    other_171 = other_170;
    let _e4: MultiVector = self_199;
    let _e8: MultiVector = other_171;
    let _e19: MultiVector = self_199;
    let _e23: MultiVector = other_171;
    let _e35: MultiVector = self_199;
    let _e39: MultiVector = other_171;
    let _e51: MultiVector = self_199;
    let _e55: MultiVector = other_171;
    let _e67: MultiVector = self_199;
    let _e71: MultiVector = other_171;
    let _e84: MultiVector = self_199;
    let _e88: MultiVector = other_171;
    let _e101: MultiVector = self_199;
    let _e105: MultiVector = other_171;
    let _e118: MultiVector = self_199;
    let _e122: MultiVector = other_171;
    let _e133: MultiVector = self_199;
    let _e137: MultiVector = other_171;
    let _e149: MultiVector = self_199;
    let _e153: MultiVector = other_171;
    let _e165: MultiVector = self_199;
    let _e169: MultiVector = other_171;
    let _e181: MultiVector = self_199;
    let _e185: MultiVector = other_171;
    let _e198: MultiVector = self_199;
    let _e202: MultiVector = other_171;
    let _e215: MultiVector = self_199;
    let _e219: MultiVector = other_171;
    let _e232: MultiVector = self_199;
    let _e236: MultiVector = other_171;
    let _e249: MultiVector = self_199;
    let _e253: MultiVector = other_171;
    let _e265: MultiVector = self_199;
    let _e269: MultiVector = other_171;
    let _e279: MultiVector = self_199;
    let _e283: MultiVector = other_171;
    let _e294: MultiVector = self_199;
    let _e298: MultiVector = other_171;
    let _e309: MultiVector = self_199;
    let _e313: MultiVector = other_171;
    let _e317: MultiVector = self_199;
    let _e321: MultiVector = other_171;
    let _e333: MultiVector = self_199;
    let _e337: MultiVector = other_171;
    let _e349: MultiVector = self_199;
    let _e353: MultiVector = other_171;
    let _e365: MultiVector = self_199;
    let _e369: MultiVector = other_171;
    let _e381: MultiVector = self_199;
    let _e385: MultiVector = other_171;
    let _e396: MultiVector = self_199;
    let _e400: MultiVector = other_171;
    let _e412: MultiVector = self_199;
    let _e416: MultiVector = other_171;
    let _e428: MultiVector = self_199;
    let _e432: MultiVector = other_171;
    let _e444: MultiVector = self_199;
    let _e448: MultiVector = other_171;
    let _e460: MultiVector = self_199;
    let _e464: MultiVector = other_171;
    let _e477: MultiVector = self_199;
    let _e481: MultiVector = other_171;
    let _e493: MultiVector = self_199;
    let _e497: MultiVector = other_171;
    let _e509: MultiVector = self_199;
    let _e513: MultiVector = other_171;
    let _e517: MultiVector = self_199;
    let _e521: MultiVector = other_171;
    let _e534: MultiVector = self_199;
    let _e538: MultiVector = other_171;
    let _e551: MultiVector = self_199;
    let _e555: MultiVector = other_171;
    let _e568: MultiVector = self_199;
    let _e571: MultiVector = self_199;
    let _e574: MultiVector = self_199;
    let _e577: MultiVector = self_199;
    let _e581: MultiVector = other_171;
    let _e584: MultiVector = other_171;
    let _e587: MultiVector = other_171;
    let _e590: MultiVector = other_171;
    let _e603: MultiVector = self_199;
    let _e607: MultiVector = other_171;
    let _e618: MultiVector = self_199;
    let _e622: MultiVector = other_171;
    let _e634: MultiVector = self_199;
    let _e638: MultiVector = other_171;
    let _e651: MultiVector = self_199;
    let _e655: MultiVector = other_171;
    let _e667: MultiVector = self_199;
    let _e671: MultiVector = other_171;
    let _e683: MultiVector = self_199;
    let _e686: MultiVector = self_199;
    let _e689: MultiVector = self_199;
    let _e692: MultiVector = self_199;
    let _e696: MultiVector = other_171;
    let _e699: MultiVector = other_171;
    let _e702: MultiVector = other_171;
    let _e705: MultiVector = other_171;
    return MultiVector((((((((((((((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0)) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.zzxz) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0))) + ((vec4<f32>(_e35.g0_.w) * _e39.g0_.wwwx) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))) + ((vec4<f32>(_e51.g1_.x) * vec4<f32>(_e55.g1_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e67.g1_.y) * _e71.g1_.yxyy) * vec4<f32>(-(1.0), -(1.0), 0.0, 0.0))) + ((vec4<f32>(_e84.g1_.z) * _e88.g1_.zzxz) * vec4<f32>(-(1.0), 0.0, -(1.0), 0.0))) + ((vec4<f32>(_e101.g1_.w) * _e105.g1_.wwwx) * vec4<f32>(-(1.0), 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e118.g2_.x) * _e122.g2_) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0))) + ((vec4<f32>(_e133.g2_.y) * vec4<f32>(_e137.g2_.y)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e149.g2_.z) * vec4<f32>(_e153.g2_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e165.g2_.w) * vec4<f32>(_e169.g2_.w)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e181.g3_.x) * _e185.g3_) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0)))) + ((vec4<f32>(_e198.g3_.y) * vec4<f32>(_e202.g3_.y)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((vec4<f32>(_e215.g3_.z) * vec4<f32>(_e219.g3_.z)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((vec4<f32>(_e232.g3_.w) * vec4<f32>(_e236.g3_.w)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((vec4<f32>(_e249.g0_.x) * vec4<f32>(_e253.g0_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))), (((((((((vec4<f32>(_e265.g1_.y) * _e269.g0_.yxyy) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) + ((vec4<f32>(_e279.g1_.z) * _e283.g0_.zzxz) * vec4<f32>(1.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e294.g1_.w) * _e298.g0_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, 1.0))) - (vec4<f32>(_e309.g3_.x) * _e313.g2_)) + ((vec4<f32>(_e317.g3_.y) * vec4<f32>(_e321.g2_.y)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e333.g3_.z) * vec4<f32>(_e337.g2_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e349.g3_.w) * vec4<f32>(_e353.g2_.w)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e365.g1_.x) * vec4<f32>(_e369.g0_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))), ((((((((((((((vec4<f32>(_e381.g0_.z) * _e385.g2_.wwwy) * vec4<f32>(0.0, -(1.0), 0.0, 1.0)) + ((vec4<f32>(_e396.g0_.w) * _e400.g2_.zzyz) * vec4<f32>(0.0, 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e412.g1_.y) * _e416.g3_.wwwz) * vec4<f32>(0.0, 0.0, -(1.0), 1.0))) + ((vec4<f32>(_e428.g1_.z) * _e432.g3_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0)))) + ((vec4<f32>(_e444.g1_.w) * _e448.g3_.zzyz) * vec4<f32>(0.0, -(1.0), 1.0, 0.0))) + ((vec4<f32>(_e460.g2_.x) * _e464.g0_) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0)))) + ((vec4<f32>(_e477.g2_.z) * vec4<f32>(_e481.g0_.x)) * vec4<f32>(0.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e493.g2_.w) * vec4<f32>(_e497.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) - (vec4<f32>(_e509.g3_.x) * _e513.g1_)) + ((vec4<f32>(_e517.g3_.y) * vec4<f32>(_e521.g1_.x)) * vec4<f32>(0.0, -(1.0), 0.0, 0.0))) + ((vec4<f32>(_e534.g3_.z) * vec4<f32>(_e538.g1_.x)) * vec4<f32>(0.0, 0.0, -(1.0), 0.0))) + ((vec4<f32>(_e551.g3_.w) * vec4<f32>(_e555.g1_.x)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e568.g0_.x, _e571.g2_.y, _e574.g0_.y, _e577.g0_.y) * vec4<f32>(_e581.g2_.x, _e584.g0_.x, _e587.g2_.w, _e590.g2_.z)) * vec4<f32>(0.0, 1.0, 1.0, -(1.0)))), (((((((vec4<f32>(_e603.g1_.z) * _e607.g2_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e618.g1_.w) * _e622.g2_.zzyz) * vec4<f32>(0.0, -(1.0), 1.0, 0.0))) + ((vec4<f32>(_e634.g3_.x) * _e638.g0_) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0)))) + ((vec4<f32>(_e651.g3_.z) * vec4<f32>(_e655.g0_.x)) * vec4<f32>(0.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e667.g3_.w) * vec4<f32>(_e671.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e683.g1_.x, _e686.g3_.y, _e689.g1_.y, _e692.g1_.y) * vec4<f32>(_e696.g2_.x, _e699.g0_.x, _e702.g2_.w, _e705.g2_.z)) * vec4<f32>(0.0, 1.0, -(1.0), 1.0))));
}

fn multi_vector_multi_vector_scalar_product(self_200: MultiVector, other_172: MultiVector) -> Scalar {
    var self_201: MultiVector;
    var other_173: MultiVector;

    self_201 = self_200;
    other_173 = other_172;
    let _e4: MultiVector = self_201;
    let _e7: MultiVector = other_173;
    let _e11: MultiVector = self_201;
    let _e14: MultiVector = other_173;
    let _e19: MultiVector = self_201;
    let _e22: MultiVector = other_173;
    let _e27: MultiVector = self_201;
    let _e30: MultiVector = other_173;
    let _e35: MultiVector = self_201;
    let _e38: MultiVector = other_173;
    let _e43: MultiVector = self_201;
    let _e46: MultiVector = other_173;
    let _e51: MultiVector = self_201;
    let _e54: MultiVector = other_173;
    let _e59: MultiVector = self_201;
    let _e62: MultiVector = other_173;
    let _e67: MultiVector = self_201;
    let _e70: MultiVector = other_173;
    let _e75: MultiVector = self_201;
    let _e78: MultiVector = other_173;
    let _e83: MultiVector = self_201;
    let _e86: MultiVector = other_173;
    let _e91: MultiVector = self_201;
    let _e94: MultiVector = other_173;
    let _e99: MultiVector = self_201;
    let _e102: MultiVector = other_173;
    let _e107: MultiVector = self_201;
    let _e110: MultiVector = other_173;
    let _e115: MultiVector = self_201;
    let _e118: MultiVector = other_173;
    let _e123: MultiVector = self_201;
    let _e126: MultiVector = other_173;
    return Scalar(((((((((((((((((_e4.g0_.x * _e7.g0_.x) - (_e11.g0_.y * _e14.g0_.y)) - (_e19.g0_.z * _e22.g0_.z)) - (_e27.g0_.w * _e30.g0_.w)) + (_e35.g1_.x * _e38.g1_.x)) - (_e43.g1_.y * _e46.g1_.y)) - (_e51.g1_.z * _e54.g1_.z)) - (_e59.g1_.w * _e62.g1_.w)) - (_e67.g2_.x * _e70.g2_.x)) + (_e75.g2_.y * _e78.g2_.y)) + (_e83.g2_.z * _e86.g2_.z)) + (_e91.g2_.w * _e94.g2_.w)) + (_e99.g3_.x * _e102.g3_.x)) - (_e107.g3_.y * _e110.g3_.y)) - (_e115.g3_.z * _e118.g3_.z)) - (_e123.g3_.w * _e126.g3_.w)));
}

fn multi_vector_rotor_into(self_202: MultiVector) -> Rotor {
    var self_203: MultiVector;

    self_203 = self_202;
    let _e2: MultiVector = self_203;
    return Rotor(_e2.g0_);
}

fn multi_vector_rotor_add(self_204: MultiVector, other_174: Rotor) -> MultiVector {
    var self_205: MultiVector;
    var other_175: Rotor;

    self_205 = self_204;
    other_175 = other_174;
    let _e4: MultiVector = self_205;
    let _e6: Rotor = other_175;
    let _e9: MultiVector = self_205;
    let _e11: MultiVector = self_205;
    let _e13: MultiVector = self_205;
    return MultiVector((_e4.g0_ + _e6.g0_), _e9.g1_, _e11.g2_, _e13.g3_);
}

fn multi_vector_rotor_sub(self_206: MultiVector, other_176: Rotor) -> MultiVector {
    var self_207: MultiVector;
    var other_177: Rotor;

    self_207 = self_206;
    other_177 = other_176;
    let _e4: MultiVector = self_207;
    let _e6: Rotor = other_177;
    let _e9: MultiVector = self_207;
    let _e11: MultiVector = self_207;
    let _e13: MultiVector = self_207;
    return MultiVector((_e4.g0_ - _e6.g0_), _e9.g1_, _e11.g2_, _e13.g3_);
}

fn multi_vector_rotor_geometric_product(self_208: MultiVector, other_178: Rotor) -> MultiVector {
    var self_209: MultiVector;
    var other_179: Rotor;

    self_209 = self_208;
    other_179 = other_178;
    let _e4: MultiVector = self_209;
    let _e8: Rotor = other_179;
    let _e11: MultiVector = self_209;
    let _e15: Rotor = other_179;
    let _e28: MultiVector = self_209;
    let _e32: Rotor = other_179;
    let _e45: MultiVector = self_209;
    let _e49: Rotor = other_179;
    let _e62: MultiVector = self_209;
    let _e66: Rotor = other_179;
    let _e78: MultiVector = self_209;
    let _e82: Rotor = other_179;
    let _e94: MultiVector = self_209;
    let _e98: Rotor = other_179;
    let _e110: MultiVector = self_209;
    let _e114: Rotor = other_179;
    let _e126: MultiVector = self_209;
    let _e130: Rotor = other_179;
    let _e142: MultiVector = self_209;
    let _e146: Rotor = other_179;
    let _e158: MultiVector = self_209;
    let _e162: Rotor = other_179;
    let _e174: MultiVector = self_209;
    let _e178: Rotor = other_179;
    let _e190: MultiVector = self_209;
    let _e194: Rotor = other_179;
    let _e206: MultiVector = self_209;
    let _e210: Rotor = other_179;
    let _e222: MultiVector = self_209;
    let _e226: Rotor = other_179;
    let _e238: MultiVector = self_209;
    let _e242: Rotor = other_179;
    return MultiVector(((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.y) * _e15.g0_.yxwz) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e28.g0_.z) * _e32.g0_.zwxy) * vec4<f32>(-(1.0), -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e45.g0_.w) * _e49.g0_.wzyx) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))), (((((vec4<f32>(_e62.g1_.x) * _e66.g0_) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))) + ((vec4<f32>(_e78.g1_.y) * _e82.g0_.yxwz) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e94.g1_.z) * _e98.g0_.zwxy) * vec4<f32>(1.0, -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e110.g1_.w) * _e114.g0_.wzyx) * vec4<f32>(1.0, 1.0, -(1.0), 1.0))), (((((vec4<f32>(_e126.g2_.x) * _e130.g0_) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))) + ((vec4<f32>(_e142.g2_.y) * _e146.g0_.yxwz) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e158.g2_.z) * _e162.g0_.zwxy) * vec4<f32>(1.0, -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e174.g2_.w) * _e178.g0_.wzyx) * vec4<f32>(1.0, 1.0, -(1.0), 1.0))), (((((vec4<f32>(_e190.g3_.x) * _e194.g0_) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))) + ((vec4<f32>(_e206.g3_.y) * _e210.g0_.yxwz) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e222.g3_.z) * _e226.g0_.zwxy) * vec4<f32>(1.0, -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e238.g3_.w) * _e242.g0_.wzyx) * vec4<f32>(1.0, 1.0, -(1.0), 1.0))));
}

fn multi_vector_rotor_outer_product(self_210: MultiVector, other_180: Rotor) -> MultiVector {
    var self_211: MultiVector;
    var other_181: Rotor;

    self_211 = self_210;
    other_181 = other_180;
    let _e4: MultiVector = self_211;
    let _e8: Rotor = other_181;
    let _e11: MultiVector = self_211;
    let _e13: Rotor = other_181;
    let _e25: MultiVector = self_211;
    let _e29: Rotor = other_181;
    let _e41: MultiVector = self_211;
    let _e43: Rotor = other_181;
    let _e55: MultiVector = self_211;
    let _e59: Rotor = other_181;
    let _e69: MultiVector = self_211;
    let _e73: Rotor = other_181;
    let _e84: MultiVector = self_211;
    let _e88: Rotor = other_181;
    let _e99: MultiVector = self_211;
    let _e103: Rotor = other_181;
    let _e115: MultiVector = self_211;
    let _e119: Rotor = other_181;
    let _e129: MultiVector = self_211;
    let _e133: Rotor = other_181;
    let _e144: MultiVector = self_211;
    let _e148: Rotor = other_181;
    let _e159: MultiVector = self_211;
    let _e163: Rotor = other_181;
    return MultiVector(((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((_e11.g0_ * vec4<f32>(_e13.g0_.x)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))), (((vec4<f32>(_e25.g1_.x) * _e29.g0_) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))) + ((_e41.g1_ * vec4<f32>(_e43.g0_.x)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))), (((((vec4<f32>(_e55.g2_.y) * _e59.g0_.yxyy) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) + ((vec4<f32>(_e69.g2_.z) * _e73.g0_.zzxz) * vec4<f32>(1.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e84.g2_.w) * _e88.g0_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e99.g2_.x) * vec4<f32>(_e103.g0_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))), (((((vec4<f32>(_e115.g3_.y) * _e119.g0_.yxyy) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) + ((vec4<f32>(_e129.g3_.z) * _e133.g0_.zzxz) * vec4<f32>(1.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e144.g3_.w) * _e148.g0_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e159.g3_.x) * vec4<f32>(_e163.g0_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))));
}

fn multi_vector_rotor_inner_product(self_212: MultiVector, other_182: Rotor) -> MultiVector {
    var self_213: MultiVector;
    var other_183: Rotor;

    self_213 = self_212;
    other_183 = other_182;
    let _e4: MultiVector = self_213;
    let _e8: Rotor = other_183;
    let _e11: MultiVector = self_213;
    let _e15: Rotor = other_183;
    let _e27: MultiVector = self_213;
    let _e31: Rotor = other_183;
    let _e43: MultiVector = self_213;
    let _e46: Rotor = other_183;
    let _e58: MultiVector = self_213;
    let _e62: Rotor = other_183;
    let _e72: MultiVector = self_213;
    let _e76: Rotor = other_183;
    let _e87: MultiVector = self_213;
    let _e91: Rotor = other_183;
    let _e102: MultiVector = self_213;
    let _e106: Rotor = other_183;
    let _e118: MultiVector = self_213;
    let _e122: Rotor = other_183;
    let _e134: MultiVector = self_213;
    let _e138: Rotor = other_183;
    let _e150: MultiVector = self_213;
    let _e154: Rotor = other_183;
    let _e166: MultiVector = self_213;
    let _e169: Rotor = other_183;
    let _e181: MultiVector = self_213;
    let _e185: Rotor = other_183;
    let _e197: MultiVector = self_213;
    let _e199: Rotor = other_183;
    return MultiVector(((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.z) * _e15.g0_.zzxz) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0))) + ((vec4<f32>(_e27.g0_.w) * _e31.g0_.wwwx) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))) + ((_e43.g0_.yyxx * _e46.g0_.yxxx) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))), (((((vec4<f32>(_e58.g1_.y) * _e62.g0_.yxyy) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) + ((vec4<f32>(_e72.g1_.z) * _e76.g0_.zzxz) * vec4<f32>(1.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e87.g1_.w) * _e91.g0_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e102.g1_.x) * vec4<f32>(_e106.g0_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))), (((((vec4<f32>(_e118.g2_.x) * _e122.g0_) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))) + ((vec4<f32>(_e134.g2_.z) * _e138.g0_.wwxy) * vec4<f32>(0.0, -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e150.g2_.w) * _e154.g0_.zzyx) * vec4<f32>(0.0, 1.0, -(1.0), 1.0))) + ((_e166.g2_.xyyy * _e169.g0_.xxwz) * vec4<f32>(0.0, 1.0, 1.0, -(1.0)))), (((vec4<f32>(_e181.g3_.x) * _e185.g0_) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))) + ((_e197.g3_ * vec4<f32>(_e199.g0_.x)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn multi_vector_rotor_right_contraction(self_214: MultiVector, other_184: Rotor) -> MultiVector {
    var self_215: MultiVector;
    var other_185: Rotor;

    self_215 = self_214;
    other_185 = other_184;
    let _e4: MultiVector = self_215;
    let _e8: Rotor = other_185;
    let _e19: MultiVector = self_215;
    let _e23: Rotor = other_185;
    let _e35: MultiVector = self_215;
    let _e39: Rotor = other_185;
    let _e51: MultiVector = self_215;
    let _e55: Rotor = other_185;
    let _e67: MultiVector = self_215;
    let _e71: Rotor = other_185;
    let _e81: MultiVector = self_215;
    let _e85: Rotor = other_185;
    let _e96: MultiVector = self_215;
    let _e100: Rotor = other_185;
    let _e111: MultiVector = self_215;
    let _e115: Rotor = other_185;
    let _e127: MultiVector = self_215;
    let _e131: Rotor = other_185;
    let _e143: MultiVector = self_215;
    let _e145: Rotor = other_185;
    let _e157: MultiVector = self_215;
    let _e161: Rotor = other_185;
    let _e173: MultiVector = self_215;
    let _e175: Rotor = other_185;
    return MultiVector((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0)) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.zzxz) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0))) + ((vec4<f32>(_e35.g0_.w) * _e39.g0_.wwwx) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))) + ((vec4<f32>(_e51.g0_.x) * vec4<f32>(_e55.g0_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))), (((((vec4<f32>(_e67.g1_.y) * _e71.g0_.yxyy) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) + ((vec4<f32>(_e81.g1_.z) * _e85.g0_.zzxz) * vec4<f32>(1.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e96.g1_.w) * _e100.g0_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e111.g1_.x) * vec4<f32>(_e115.g0_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))), (((vec4<f32>(_e127.g2_.x) * _e131.g0_) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))) + ((_e143.g2_ * vec4<f32>(_e145.g0_.x)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))), (((vec4<f32>(_e157.g3_.x) * _e161.g0_) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))) + ((_e173.g3_ * vec4<f32>(_e175.g0_.x)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn multi_vector_rotor_scalar_product(self_216: MultiVector, other_186: Rotor) -> Scalar {
    var self_217: MultiVector;
    var other_187: Rotor;

    self_217 = self_216;
    other_187 = other_186;
    let _e4: MultiVector = self_217;
    let _e7: Rotor = other_187;
    let _e11: MultiVector = self_217;
    let _e14: Rotor = other_187;
    let _e19: MultiVector = self_217;
    let _e22: Rotor = other_187;
    let _e27: MultiVector = self_217;
    let _e30: Rotor = other_187;
    return Scalar(((((_e4.g0_.x * _e7.g0_.x) - (_e11.g0_.y * _e14.g0_.y)) - (_e19.g0_.z * _e22.g0_.z)) - (_e27.g0_.w * _e30.g0_.w)));
}

fn multi_vector_point_into(self_218: MultiVector) -> Point {
    var self_219: MultiVector;

    self_219 = self_218;
    let _e2: MultiVector = self_219;
    let _e5: MultiVector = self_219;
    let _e8: MultiVector = self_219;
    let _e11: MultiVector = self_219;
    return Point(vec4<f32>(_e2.g2_.x, _e5.g1_.y, _e8.g1_.z, _e11.g1_.w));
}

fn multi_vector_point_add(self_220: MultiVector, other_188: Point) -> MultiVector {
    var self_221: MultiVector;
    var other_189: Point;

    self_221 = self_220;
    other_189 = other_188;
    let _e4: MultiVector = self_221;
    let _e6: MultiVector = self_221;
    let _e8: Point = other_189;
    let _e17: MultiVector = self_221;
    let _e19: Point = other_189;
    let _e30: MultiVector = self_221;
    return MultiVector(_e4.g0_, (_e6.g1_ + (_e8.g0_ * vec4<f32>(0.0, 1.0, 1.0, 1.0))), (_e17.g2_ + (vec4<f32>(_e19.g0_.x) * vec4<f32>(1.0, 0.0, 0.0, 0.0))), _e30.g3_);
}

fn multi_vector_point_sub(self_222: MultiVector, other_190: Point) -> MultiVector {
    var self_223: MultiVector;
    var other_191: Point;

    self_223 = self_222;
    other_191 = other_190;
    let _e4: MultiVector = self_223;
    let _e6: MultiVector = self_223;
    let _e8: Point = other_191;
    let _e17: MultiVector = self_223;
    let _e19: Point = other_191;
    let _e30: MultiVector = self_223;
    return MultiVector(_e4.g0_, (_e6.g1_ - (_e8.g0_ * vec4<f32>(0.0, 1.0, 1.0, 1.0))), (_e17.g2_ - (vec4<f32>(_e19.g0_.x) * vec4<f32>(1.0, 0.0, 0.0, 0.0))), _e30.g3_);
}

fn multi_vector_point_geometric_product(self_224: MultiVector, other_192: Point) -> MultiVector {
    var self_225: MultiVector;
    var other_193: Point;

    self_225 = self_224;
    other_193 = other_192;
    let _e4: MultiVector = self_225;
    let _e8: Point = other_193;
    let _e20: MultiVector = self_225;
    let _e24: Point = other_193;
    let _e37: MultiVector = self_225;
    let _e41: Point = other_193;
    let _e54: MultiVector = self_225;
    let _e58: Point = other_193;
    let _e70: MultiVector = self_225;
    let _e74: Point = other_193;
    let _e86: MultiVector = self_225;
    let _e90: Point = other_193;
    let _e102: MultiVector = self_225;
    let _e105: MultiVector = self_225;
    let _e108: MultiVector = self_225;
    let _e111: MultiVector = self_225;
    let _e115: Point = other_193;
    let _e123: MultiVector = self_225;
    let _e127: Point = other_193;
    let _e138: MultiVector = self_225;
    let _e142: Point = other_193;
    let _e154: MultiVector = self_225;
    let _e158: Point = other_193;
    let _e170: MultiVector = self_225;
    let _e174: Point = other_193;
    let _e187: MultiVector = self_225;
    let _e191: Point = other_193;
    let _e204: MultiVector = self_225;
    let _e208: Point = other_193;
    let _e221: MultiVector = self_225;
    let _e224: MultiVector = self_225;
    let _e227: MultiVector = self_225;
    let _e230: MultiVector = self_225;
    let _e234: Point = other_193;
    let _e245: MultiVector = self_225;
    let _e249: Point = other_193;
    let _e262: MultiVector = self_225;
    let _e266: Point = other_193;
    let _e278: MultiVector = self_225;
    let _e282: Point = other_193;
    let _e294: MultiVector = self_225;
    let _e298: Point = other_193;
    let _e310: MultiVector = self_225;
    let _e312: Point = other_193;
    let _e327: MultiVector = self_225;
    let _e331: Point = other_193;
    let _e344: MultiVector = self_225;
    let _e348: Point = other_193;
    let _e360: MultiVector = self_225;
    let _e364: Point = other_193;
    let _e376: MultiVector = self_225;
    let _e380: Point = other_193;
    let _e392: MultiVector = self_225;
    let _e394: Point = other_193;
    return MultiVector(((((((((vec4<f32>(_e4.g1_.y) * _e8.g0_.yywz) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e20.g1_.z) * _e24.g0_.zwzy) * vec4<f32>(-(1.0), -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e37.g1_.w) * _e41.g0_.wzyw) * vec4<f32>(-(1.0), 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e54.g2_.y) * vec4<f32>(_e58.g0_.x)) * vec4<f32>(0.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e70.g2_.z) * vec4<f32>(_e74.g0_.x)) * vec4<f32>(0.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e86.g2_.w) * vec4<f32>(_e90.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e102.g2_.x, _e105.g1_.x, _e108.g1_.x, _e111.g1_.x) * _e115.g0_) * vec4<f32>(-(1.0)))), ((((((((vec4<f32>(_e123.g0_.y) * _e127.g0_.yywz) * vec4<f32>(1.0, 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e138.g0_.z) * _e142.g0_.zwzy) * vec4<f32>(1.0, -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e154.g0_.w) * _e158.g0_.wzyw) * vec4<f32>(1.0, 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e170.g3_.y) * vec4<f32>(_e174.g0_.x)) * vec4<f32>(0.0, -(1.0), 0.0, 0.0))) + ((vec4<f32>(_e187.g3_.z) * vec4<f32>(_e191.g0_.x)) * vec4<f32>(0.0, 0.0, -(1.0), 0.0))) + ((vec4<f32>(_e204.g3_.w) * vec4<f32>(_e208.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e221.g3_.x, _e224.g0_.x, _e227.g0_.x, _e230.g0_.x) * _e234.g0_) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0))), ((((((vec4<f32>(_e245.g3_.x) * _e249.g0_.yyzw) * vec4<f32>(0.0, -(1.0), -(1.0), -(1.0))) + ((vec4<f32>(_e262.g3_.y) * _e266.g0_.yywz) * vec4<f32>(1.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e278.g3_.z) * _e282.g0_.zwzy) * vec4<f32>(1.0, -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e294.g3_.w) * _e298.g0_.wzyw) * vec4<f32>(1.0, 1.0, -(1.0), 0.0))) + ((_e310.g0_ * vec4<f32>(_e312.g0_.x)) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0)))), ((((((vec4<f32>(_e327.g2_.x) * _e331.g0_.yyzw) * vec4<f32>(0.0, -(1.0), -(1.0), -(1.0))) + ((vec4<f32>(_e344.g2_.y) * _e348.g0_.yywz) * vec4<f32>(1.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e360.g2_.z) * _e364.g0_.zwzy) * vec4<f32>(1.0, -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e376.g2_.w) * _e380.g0_.wzyw) * vec4<f32>(1.0, 1.0, -(1.0), 0.0))) + (_e392.g1_ * vec4<f32>(_e394.g0_.x))));
}

fn multi_vector_point_scalar_product(self_226: MultiVector, other_194: Point) -> Scalar {
    var self_227: MultiVector;
    var other_195: Point;

    self_227 = self_226;
    other_195 = other_194;
    let _e5: MultiVector = self_227;
    let _e8: Point = other_195;
    let _e13: MultiVector = self_227;
    let _e16: Point = other_195;
    let _e21: MultiVector = self_227;
    let _e24: Point = other_195;
    let _e29: MultiVector = self_227;
    let _e32: Point = other_195;
    return Scalar(((((0.0 - (_e5.g1_.y * _e8.g0_.y)) - (_e13.g1_.z * _e16.g0_.z)) - (_e21.g1_.w * _e24.g0_.w)) - (_e29.g2_.x * _e32.g0_.x)));
}

fn multi_vector_ideal_point_into(self_228: MultiVector) -> IdealPoint {
    var self_229: MultiVector;

    self_229 = self_228;
    let _e2: MultiVector = self_229;
    let _e5: MultiVector = self_229;
    let _e8: MultiVector = self_229;
    return IdealPoint(vec3<f32>(_e2.g3_.y, _e5.g3_.z, _e8.g3_.w));
}

fn multi_vector_ideal_point_add(self_230: MultiVector, other_196: IdealPoint) -> MultiVector {
    var self_231: MultiVector;
    var other_197: IdealPoint;

    self_231 = self_230;
    other_197 = other_196;
    let _e4: MultiVector = self_231;
    let _e6: MultiVector = self_231;
    let _e8: MultiVector = self_231;
    let _e10: MultiVector = self_231;
    let _e12: IdealPoint = other_197;
    let _e15: IdealPoint = other_197;
    let _e18: IdealPoint = other_197;
    let _e21: IdealPoint = other_197;
    return MultiVector(_e4.g0_, _e6.g1_, _e8.g2_, (_e10.g3_ + (vec4<f32>(_e12.g0_.x, _e15.g0_.x, _e18.g0_.y, _e21.g0_.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn multi_vector_ideal_point_sub(self_232: MultiVector, other_198: IdealPoint) -> MultiVector {
    var self_233: MultiVector;
    var other_199: IdealPoint;

    self_233 = self_232;
    other_199 = other_198;
    let _e4: MultiVector = self_233;
    let _e6: MultiVector = self_233;
    let _e8: MultiVector = self_233;
    let _e10: MultiVector = self_233;
    let _e12: IdealPoint = other_199;
    let _e15: IdealPoint = other_199;
    let _e18: IdealPoint = other_199;
    let _e21: IdealPoint = other_199;
    return MultiVector(_e4.g0_, _e6.g1_, _e8.g2_, (_e10.g3_ - (vec4<f32>(_e12.g0_.x, _e15.g0_.x, _e18.g0_.y, _e21.g0_.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn multi_vector_ideal_point_geometric_product(self_234: MultiVector, other_200: IdealPoint) -> MultiVector {
    var self_235: MultiVector;
    var other_201: IdealPoint;

    self_235 = self_234;
    other_201 = other_200;
    let _e4: MultiVector = self_235;
    let _e8: IdealPoint = other_201;
    let _e11: IdealPoint = other_201;
    let _e14: IdealPoint = other_201;
    let _e17: IdealPoint = other_201;
    let _e30: MultiVector = self_235;
    let _e34: IdealPoint = other_201;
    let _e37: IdealPoint = other_201;
    let _e40: IdealPoint = other_201;
    let _e43: IdealPoint = other_201;
    let _e57: MultiVector = self_235;
    let _e61: IdealPoint = other_201;
    let _e64: IdealPoint = other_201;
    let _e67: IdealPoint = other_201;
    let _e70: IdealPoint = other_201;
    let _e84: MultiVector = self_235;
    let _e88: IdealPoint = other_201;
    let _e91: IdealPoint = other_201;
    let _e94: IdealPoint = other_201;
    let _e97: IdealPoint = other_201;
    let _e112: MultiVector = self_235;
    let _e116: IdealPoint = other_201;
    let _e119: IdealPoint = other_201;
    let _e122: IdealPoint = other_201;
    let _e125: IdealPoint = other_201;
    let _e138: MultiVector = self_235;
    let _e142: IdealPoint = other_201;
    let _e145: IdealPoint = other_201;
    let _e148: IdealPoint = other_201;
    let _e151: IdealPoint = other_201;
    let _e165: MultiVector = self_235;
    let _e169: IdealPoint = other_201;
    let _e172: IdealPoint = other_201;
    let _e175: IdealPoint = other_201;
    let _e178: IdealPoint = other_201;
    let _e192: MultiVector = self_235;
    let _e196: IdealPoint = other_201;
    let _e199: IdealPoint = other_201;
    let _e202: IdealPoint = other_201;
    let _e205: IdealPoint = other_201;
    let _e217: MultiVector = self_235;
    let _e221: IdealPoint = other_201;
    let _e224: IdealPoint = other_201;
    let _e227: IdealPoint = other_201;
    let _e230: IdealPoint = other_201;
    let _e243: MultiVector = self_235;
    let _e247: IdealPoint = other_201;
    let _e250: IdealPoint = other_201;
    let _e253: IdealPoint = other_201;
    let _e256: IdealPoint = other_201;
    let _e270: MultiVector = self_235;
    let _e274: IdealPoint = other_201;
    let _e277: IdealPoint = other_201;
    let _e280: IdealPoint = other_201;
    let _e283: IdealPoint = other_201;
    let _e297: MultiVector = self_235;
    let _e301: IdealPoint = other_201;
    let _e304: IdealPoint = other_201;
    let _e307: IdealPoint = other_201;
    let _e310: IdealPoint = other_201;
    let _e322: MultiVector = self_235;
    let _e326: IdealPoint = other_201;
    let _e329: IdealPoint = other_201;
    let _e332: IdealPoint = other_201;
    let _e335: IdealPoint = other_201;
    let _e347: MultiVector = self_235;
    let _e351: IdealPoint = other_201;
    let _e354: IdealPoint = other_201;
    let _e357: IdealPoint = other_201;
    let _e360: IdealPoint = other_201;
    let _e373: MultiVector = self_235;
    let _e377: IdealPoint = other_201;
    let _e380: IdealPoint = other_201;
    let _e383: IdealPoint = other_201;
    let _e386: IdealPoint = other_201;
    let _e399: MultiVector = self_235;
    let _e403: IdealPoint = other_201;
    let _e406: IdealPoint = other_201;
    let _e409: IdealPoint = other_201;
    let _e412: IdealPoint = other_201;
    return MultiVector((((((vec4<f32>(_e4.g3_.y) * vec4<f32>(_e8.g0_.x, _e11.g0_.x, _e14.g0_.z, _e17.g0_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e30.g3_.z) * vec4<f32>(_e34.g0_.y, _e37.g0_.z, _e40.g0_.y, _e43.g0_.x)) * vec4<f32>(-(1.0), -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e57.g3_.w) * vec4<f32>(_e61.g0_.z, _e64.g0_.y, _e67.g0_.x, _e70.g0_.z)) * vec4<f32>(-(1.0), 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e84.g3_.x) * vec4<f32>(_e88.g0_.x, _e91.g0_.x, _e94.g0_.y, _e97.g0_.z)) * vec4<f32>(0.0, -(1.0), -(1.0), -(1.0)))), (((((vec4<f32>(_e112.g2_.y) * vec4<f32>(_e116.g0_.x, _e119.g0_.x, _e122.g0_.z, _e125.g0_.y)) * vec4<f32>(-(1.0), 0.0, -(1.0), 1.0)) + ((vec4<f32>(_e138.g2_.z) * vec4<f32>(_e142.g0_.y, _e145.g0_.z, _e148.g0_.y, _e151.g0_.x)) * vec4<f32>(-(1.0), 1.0, 0.0, -(1.0)))) + ((vec4<f32>(_e165.g2_.w) * vec4<f32>(_e169.g0_.z, _e172.g0_.y, _e175.g0_.x, _e178.g0_.z)) * vec4<f32>(-(1.0), -(1.0), 1.0, 0.0))) + ((vec4<f32>(_e192.g2_.x) * vec4<f32>(_e196.g0_.x, _e199.g0_.x, _e202.g0_.y, _e205.g0_.z)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))), (((((vec4<f32>(_e217.g1_.y) * vec4<f32>(_e221.g0_.x, _e224.g0_.x, _e227.g0_.z, _e230.g0_.y)) * vec4<f32>(-(1.0), 0.0, -(1.0), 1.0)) + ((vec4<f32>(_e243.g1_.z) * vec4<f32>(_e247.g0_.y, _e250.g0_.z, _e253.g0_.y, _e256.g0_.x)) * vec4<f32>(-(1.0), 1.0, 0.0, -(1.0)))) + ((vec4<f32>(_e270.g1_.w) * vec4<f32>(_e274.g0_.z, _e277.g0_.y, _e280.g0_.x, _e283.g0_.z)) * vec4<f32>(-(1.0), -(1.0), 1.0, 0.0))) + ((vec4<f32>(_e297.g1_.x) * vec4<f32>(_e301.g0_.x, _e304.g0_.x, _e307.g0_.y, _e310.g0_.z)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))), (((((vec4<f32>(_e322.g0_.y) * vec4<f32>(_e326.g0_.x, _e329.g0_.x, _e332.g0_.z, _e335.g0_.y)) * vec4<f32>(1.0, 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e347.g0_.z) * vec4<f32>(_e351.g0_.y, _e354.g0_.z, _e357.g0_.y, _e360.g0_.x)) * vec4<f32>(1.0, -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e373.g0_.w) * vec4<f32>(_e377.g0_.z, _e380.g0_.y, _e383.g0_.x, _e386.g0_.z)) * vec4<f32>(1.0, 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e399.g0_.x) * vec4<f32>(_e403.g0_.x, _e406.g0_.x, _e409.g0_.y, _e412.g0_.z)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn multi_vector_ideal_point_scalar_product(self_236: MultiVector, other_202: IdealPoint) -> Scalar {
    var self_237: MultiVector;
    var other_203: IdealPoint;

    self_237 = self_236;
    other_203 = other_202;
    let _e5: MultiVector = self_237;
    let _e8: IdealPoint = other_203;
    let _e13: MultiVector = self_237;
    let _e16: IdealPoint = other_203;
    let _e21: MultiVector = self_237;
    let _e24: IdealPoint = other_203;
    return Scalar((((0.0 - (_e5.g3_.y * _e8.g0_.x)) - (_e13.g3_.z * _e16.g0_.y)) - (_e21.g3_.w * _e24.g0_.z)));
}

fn multi_vector_plane_into(self_238: MultiVector) -> Plane {
    var self_239: MultiVector;

    self_239 = self_238;
    let _e2: MultiVector = self_239;
    let _e5: MultiVector = self_239;
    let _e8: MultiVector = self_239;
    let _e11: MultiVector = self_239;
    return Plane(vec4<f32>(_e2.g1_.x, _e5.g2_.y, _e8.g2_.z, _e11.g2_.w));
}

fn multi_vector_plane_add(self_240: MultiVector, other_204: Plane) -> MultiVector {
    var self_241: MultiVector;
    var other_205: Plane;

    self_241 = self_240;
    other_205 = other_204;
    let _e4: MultiVector = self_241;
    let _e6: MultiVector = self_241;
    let _e8: Plane = other_205;
    let _e19: MultiVector = self_241;
    let _e21: Plane = other_205;
    let _e30: MultiVector = self_241;
    return MultiVector(_e4.g0_, (_e6.g1_ + (vec4<f32>(_e8.g0_.x) * vec4<f32>(1.0, 0.0, 0.0, 0.0))), (_e19.g2_ + (_e21.g0_ * vec4<f32>(0.0, 1.0, 1.0, 1.0))), _e30.g3_);
}

fn multi_vector_plane_sub(self_242: MultiVector, other_206: Plane) -> MultiVector {
    var self_243: MultiVector;
    var other_207: Plane;

    self_243 = self_242;
    other_207 = other_206;
    let _e4: MultiVector = self_243;
    let _e6: MultiVector = self_243;
    let _e8: Plane = other_207;
    let _e19: MultiVector = self_243;
    let _e21: Plane = other_207;
    let _e30: MultiVector = self_243;
    return MultiVector(_e4.g0_, (_e6.g1_ - (vec4<f32>(_e8.g0_.x) * vec4<f32>(1.0, 0.0, 0.0, 0.0))), (_e19.g2_ - (_e21.g0_ * vec4<f32>(0.0, 1.0, 1.0, 1.0))), _e30.g3_);
}

fn multi_vector_plane_geometric_product(self_244: MultiVector, other_208: Plane) -> MultiVector {
    var self_245: MultiVector;
    var other_209: Plane;

    self_245 = self_244;
    other_209 = other_208;
    let _e4: MultiVector = self_245;
    let _e8: Plane = other_209;
    let _e18: MultiVector = self_245;
    let _e22: Plane = other_209;
    let _e34: MultiVector = self_245;
    let _e38: Plane = other_209;
    let _e50: MultiVector = self_245;
    let _e54: Plane = other_209;
    let _e66: MultiVector = self_245;
    let _e68: Plane = other_209;
    let _e83: MultiVector = self_245;
    let _e87: Plane = other_209;
    let _e100: MultiVector = self_245;
    let _e104: Plane = other_209;
    let _e116: MultiVector = self_245;
    let _e120: Plane = other_209;
    let _e132: MultiVector = self_245;
    let _e136: Plane = other_209;
    let _e148: MultiVector = self_245;
    let _e150: Plane = other_209;
    let _e165: MultiVector = self_245;
    let _e169: Plane = other_209;
    let _e180: MultiVector = self_245;
    let _e184: Plane = other_209;
    let _e196: MultiVector = self_245;
    let _e200: Plane = other_209;
    let _e212: MultiVector = self_245;
    let _e216: Plane = other_209;
    let _e229: MultiVector = self_245;
    let _e233: Plane = other_209;
    let _e246: MultiVector = self_245;
    let _e250: Plane = other_209;
    let _e263: MultiVector = self_245;
    let _e266: MultiVector = self_245;
    let _e269: MultiVector = self_245;
    let _e272: MultiVector = self_245;
    let _e276: Plane = other_209;
    let _e287: MultiVector = self_245;
    let _e291: Plane = other_209;
    let _e303: MultiVector = self_245;
    let _e307: Plane = other_209;
    let _e320: MultiVector = self_245;
    let _e324: Plane = other_209;
    let _e337: MultiVector = self_245;
    let _e341: Plane = other_209;
    let _e354: MultiVector = self_245;
    let _e358: Plane = other_209;
    let _e371: MultiVector = self_245;
    let _e375: Plane = other_209;
    let _e388: MultiVector = self_245;
    let _e391: MultiVector = self_245;
    let _e394: MultiVector = self_245;
    let _e397: MultiVector = self_245;
    let _e401: Plane = other_209;
    return MultiVector(((((((vec4<f32>(_e4.g2_.x) * _e8.g0_.yyzw) * vec4<f32>(0.0, 1.0, 1.0, 1.0)) + ((vec4<f32>(_e18.g2_.y) * _e22.g0_.yywz) * vec4<f32>(1.0, 0.0, -(1.0), 1.0))) + ((vec4<f32>(_e34.g2_.z) * _e38.g0_.zwzy) * vec4<f32>(1.0, 1.0, 0.0, -(1.0)))) + ((vec4<f32>(_e50.g2_.w) * _e54.g0_.wzyw) * vec4<f32>(1.0, -(1.0), 1.0, 0.0))) + ((_e66.g1_ * vec4<f32>(_e68.g0_.x)) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0)))), ((((((vec4<f32>(_e83.g3_.x) * _e87.g0_.yyzw) * vec4<f32>(0.0, -(1.0), -(1.0), -(1.0))) + ((vec4<f32>(_e100.g3_.y) * _e104.g0_.yywz) * vec4<f32>(1.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e116.g3_.z) * _e120.g0_.zwzy) * vec4<f32>(1.0, -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e132.g3_.w) * _e136.g0_.wzyw) * vec4<f32>(1.0, 1.0, -(1.0), 0.0))) + ((_e148.g0_ * vec4<f32>(_e150.g0_.x)) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0)))), ((((((((vec4<f32>(_e165.g0_.y) * _e169.g0_.yywz) * vec4<f32>(1.0, 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e180.g0_.z) * _e184.g0_.zwzy) * vec4<f32>(1.0, -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e196.g0_.w) * _e200.g0_.wzyw) * vec4<f32>(1.0, 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e212.g3_.y) * vec4<f32>(_e216.g0_.x)) * vec4<f32>(0.0, -(1.0), 0.0, 0.0))) + ((vec4<f32>(_e229.g3_.z) * vec4<f32>(_e233.g0_.x)) * vec4<f32>(0.0, 0.0, -(1.0), 0.0))) + ((vec4<f32>(_e246.g3_.w) * vec4<f32>(_e250.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e263.g3_.x, _e266.g0_.x, _e269.g0_.x, _e272.g0_.x) * _e276.g0_) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0))), ((((((((vec4<f32>(_e287.g1_.y) * _e291.g0_.yywz) * vec4<f32>(-(1.0), 0.0, -(1.0), 1.0)) + ((vec4<f32>(_e303.g1_.z) * _e307.g0_.zwzy) * vec4<f32>(-(1.0), 1.0, 0.0, -(1.0)))) + ((vec4<f32>(_e320.g1_.w) * _e324.g0_.wzyw) * vec4<f32>(-(1.0), -(1.0), 1.0, 0.0))) + ((vec4<f32>(_e337.g2_.y) * vec4<f32>(_e341.g0_.x)) * vec4<f32>(0.0, -(1.0), 0.0, 0.0))) + ((vec4<f32>(_e354.g2_.z) * vec4<f32>(_e358.g0_.x)) * vec4<f32>(0.0, 0.0, -(1.0), 0.0))) + ((vec4<f32>(_e371.g2_.w) * vec4<f32>(_e375.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e388.g2_.x, _e391.g1_.x, _e394.g1_.x, _e397.g1_.x) * _e401.g0_) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0))));
}

fn multi_vector_plane_scalar_product(self_246: MultiVector, other_210: Plane) -> Scalar {
    var self_247: MultiVector;
    var other_211: Plane;

    self_247 = self_246;
    other_211 = other_210;
    let _e4: MultiVector = self_247;
    let _e7: Plane = other_211;
    let _e11: MultiVector = self_247;
    let _e14: Plane = other_211;
    let _e19: MultiVector = self_247;
    let _e22: Plane = other_211;
    let _e27: MultiVector = self_247;
    let _e30: Plane = other_211;
    return Scalar(((((_e4.g1_.x * _e7.g0_.x) + (_e11.g2_.y * _e14.g0_.y)) + (_e19.g2_.z * _e22.g0_.z)) + (_e27.g2_.w * _e30.g0_.w)));
}

fn multi_vector_line_into(self_248: MultiVector) -> Line {
    var self_249: MultiVector;

    self_249 = self_248;
    let _e2: MultiVector = self_249;
    let _e5: MultiVector = self_249;
    let _e8: MultiVector = self_249;
    let _e12: MultiVector = self_249;
    let _e15: MultiVector = self_249;
    let _e18: MultiVector = self_249;
    return Line(vec3<f32>(_e2.g3_.y, _e5.g3_.z, _e8.g3_.w), vec3<f32>(_e12.g0_.y, _e15.g0_.z, _e18.g0_.w));
}

fn multi_vector_line_add(self_250: MultiVector, other_212: Line) -> MultiVector {
    var self_251: MultiVector;
    var other_213: Line;

    self_251 = self_250;
    other_213 = other_212;
    let _e4: MultiVector = self_251;
    let _e6: Line = other_213;
    let _e9: Line = other_213;
    let _e12: Line = other_213;
    let _e15: Line = other_213;
    let _e26: MultiVector = self_251;
    let _e28: MultiVector = self_251;
    let _e30: MultiVector = self_251;
    let _e32: Line = other_213;
    let _e35: Line = other_213;
    let _e38: Line = other_213;
    let _e41: Line = other_213;
    return MultiVector((_e4.g0_ + (vec4<f32>(_e6.g0_.x, _e9.g1_.x, _e12.g1_.y, _e15.g1_.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0))), _e26.g1_, _e28.g2_, (_e30.g3_ + (vec4<f32>(_e32.g0_.x, _e35.g0_.x, _e38.g0_.y, _e41.g0_.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn multi_vector_line_sub(self_252: MultiVector, other_214: Line) -> MultiVector {
    var self_253: MultiVector;
    var other_215: Line;

    self_253 = self_252;
    other_215 = other_214;
    let _e4: MultiVector = self_253;
    let _e6: Line = other_215;
    let _e9: Line = other_215;
    let _e12: Line = other_215;
    let _e15: Line = other_215;
    let _e26: MultiVector = self_253;
    let _e28: MultiVector = self_253;
    let _e30: MultiVector = self_253;
    let _e32: Line = other_215;
    let _e35: Line = other_215;
    let _e38: Line = other_215;
    let _e41: Line = other_215;
    return MultiVector((_e4.g0_ - (vec4<f32>(_e6.g0_.x, _e9.g1_.x, _e12.g1_.y, _e15.g1_.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0))), _e26.g1_, _e28.g2_, (_e30.g3_ - (vec4<f32>(_e32.g0_.x, _e35.g0_.x, _e38.g0_.y, _e41.g0_.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn multi_vector_line_geometric_product(self_254: MultiVector, other_216: Line) -> MultiVector {
    var self_255: MultiVector;
    var other_217: Line;

    self_255 = self_254;
    other_217 = other_216;
    let _e4: MultiVector = self_255;
    let _e8: Line = other_217;
    let _e11: Line = other_217;
    let _e14: Line = other_217;
    let _e17: Line = other_217;
    let _e30: MultiVector = self_255;
    let _e34: Line = other_217;
    let _e37: Line = other_217;
    let _e40: Line = other_217;
    let _e43: Line = other_217;
    let _e57: MultiVector = self_255;
    let _e61: Line = other_217;
    let _e64: Line = other_217;
    let _e67: Line = other_217;
    let _e70: Line = other_217;
    let _e84: MultiVector = self_255;
    let _e88: Line = other_217;
    let _e91: Line = other_217;
    let _e94: Line = other_217;
    let _e97: Line = other_217;
    let _e112: MultiVector = self_255;
    let _e116: Line = other_217;
    let _e119: Line = other_217;
    let _e122: Line = other_217;
    let _e125: Line = other_217;
    let _e139: MultiVector = self_255;
    let _e143: Line = other_217;
    let _e146: Line = other_217;
    let _e149: Line = other_217;
    let _e152: Line = other_217;
    let _e166: MultiVector = self_255;
    let _e170: Line = other_217;
    let _e173: Line = other_217;
    let _e176: Line = other_217;
    let _e179: Line = other_217;
    let _e193: MultiVector = self_255;
    let _e197: Line = other_217;
    let _e200: Line = other_217;
    let _e203: Line = other_217;
    let _e206: Line = other_217;
    let _e218: MultiVector = self_255;
    let _e222: Line = other_217;
    let _e225: Line = other_217;
    let _e228: Line = other_217;
    let _e231: Line = other_217;
    let _e243: MultiVector = self_255;
    let _e247: Line = other_217;
    let _e250: Line = other_217;
    let _e253: Line = other_217;
    let _e256: Line = other_217;
    let _e269: MultiVector = self_255;
    let _e273: Line = other_217;
    let _e276: Line = other_217;
    let _e279: Line = other_217;
    let _e282: Line = other_217;
    let _e295: MultiVector = self_255;
    let _e299: Line = other_217;
    let _e302: Line = other_217;
    let _e305: Line = other_217;
    let _e308: Line = other_217;
    let _e320: MultiVector = self_255;
    let _e324: Line = other_217;
    let _e327: Line = other_217;
    let _e330: Line = other_217;
    let _e333: Line = other_217;
    let _e347: MultiVector = self_255;
    let _e351: Line = other_217;
    let _e354: Line = other_217;
    let _e357: Line = other_217;
    let _e360: Line = other_217;
    let _e374: MultiVector = self_255;
    let _e378: Line = other_217;
    let _e381: Line = other_217;
    let _e384: Line = other_217;
    let _e387: Line = other_217;
    let _e401: MultiVector = self_255;
    let _e405: Line = other_217;
    let _e408: Line = other_217;
    let _e411: Line = other_217;
    let _e414: Line = other_217;
    let _e429: MultiVector = self_255;
    let _e433: Line = other_217;
    let _e436: Line = other_217;
    let _e439: Line = other_217;
    let _e442: Line = other_217;
    let _e455: MultiVector = self_255;
    let _e459: Line = other_217;
    let _e462: Line = other_217;
    let _e465: Line = other_217;
    let _e468: Line = other_217;
    let _e482: MultiVector = self_255;
    let _e486: Line = other_217;
    let _e489: Line = other_217;
    let _e492: Line = other_217;
    let _e495: Line = other_217;
    let _e509: MultiVector = self_255;
    let _e513: Line = other_217;
    let _e516: Line = other_217;
    let _e519: Line = other_217;
    let _e522: Line = other_217;
    let _e537: MultiVector = self_255;
    let _e541: Line = other_217;
    let _e544: Line = other_217;
    let _e547: Line = other_217;
    let _e550: Line = other_217;
    let _e563: MultiVector = self_255;
    let _e567: Line = other_217;
    let _e570: Line = other_217;
    let _e573: Line = other_217;
    let _e576: Line = other_217;
    let _e589: MultiVector = self_255;
    let _e593: Line = other_217;
    let _e596: Line = other_217;
    let _e599: Line = other_217;
    let _e602: Line = other_217;
    let _e615: MultiVector = self_255;
    let _e619: Line = other_217;
    let _e622: Line = other_217;
    let _e625: Line = other_217;
    let _e628: Line = other_217;
    let _e640: MultiVector = self_255;
    let _e644: Line = other_217;
    let _e647: Line = other_217;
    let _e650: Line = other_217;
    let _e653: Line = other_217;
    let _e665: MultiVector = self_255;
    let _e669: Line = other_217;
    let _e672: Line = other_217;
    let _e675: Line = other_217;
    let _e678: Line = other_217;
    let _e691: MultiVector = self_255;
    let _e695: Line = other_217;
    let _e698: Line = other_217;
    let _e701: Line = other_217;
    let _e704: Line = other_217;
    let _e717: MultiVector = self_255;
    let _e721: Line = other_217;
    let _e724: Line = other_217;
    let _e727: Line = other_217;
    let _e730: Line = other_217;
    let _e745: MultiVector = self_255;
    let _e749: Line = other_217;
    let _e752: Line = other_217;
    let _e755: Line = other_217;
    let _e758: Line = other_217;
    let _e771: MultiVector = self_255;
    let _e775: Line = other_217;
    let _e778: Line = other_217;
    let _e781: Line = other_217;
    let _e784: Line = other_217;
    let _e797: MultiVector = self_255;
    let _e801: Line = other_217;
    let _e804: Line = other_217;
    let _e807: Line = other_217;
    let _e810: Line = other_217;
    let _e823: MultiVector = self_255;
    let _e827: Line = other_217;
    let _e830: Line = other_217;
    let _e833: Line = other_217;
    let _e836: Line = other_217;
    return MultiVector((((((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g1_.x, _e11.g1_.x, _e14.g1_.z, _e17.g1_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e30.g0_.z) * vec4<f32>(_e34.g1_.y, _e37.g1_.z, _e40.g1_.y, _e43.g1_.x)) * vec4<f32>(-(1.0), -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e57.g0_.w) * vec4<f32>(_e61.g1_.z, _e64.g1_.y, _e67.g1_.x, _e70.g1_.z)) * vec4<f32>(-(1.0), 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e84.g3_.x) * vec4<f32>(_e88.g0_.x, _e91.g0_.x, _e94.g0_.y, _e97.g0_.z)) * vec4<f32>(0.0, -(1.0), -(1.0), -(1.0)))) + ((vec4<f32>(_e112.g3_.y) * vec4<f32>(_e116.g0_.x, _e119.g0_.x, _e122.g0_.z, _e125.g0_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e139.g3_.z) * vec4<f32>(_e143.g0_.y, _e146.g0_.z, _e149.g0_.y, _e152.g0_.x)) * vec4<f32>(-(1.0), -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e166.g3_.w) * vec4<f32>(_e170.g0_.z, _e173.g0_.y, _e176.g0_.x, _e179.g0_.z)) * vec4<f32>(-(1.0), 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e193.g0_.x) * vec4<f32>(_e197.g1_.x, _e200.g1_.x, _e203.g1_.y, _e206.g1_.z)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))), (((((((((vec4<f32>(_e218.g1_.y) * vec4<f32>(_e222.g1_.x, _e225.g1_.x, _e228.g1_.z, _e231.g1_.y)) * vec4<f32>(1.0, 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e243.g1_.z) * vec4<f32>(_e247.g1_.y, _e250.g1_.z, _e253.g1_.y, _e256.g1_.x)) * vec4<f32>(1.0, -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e269.g1_.w) * vec4<f32>(_e273.g1_.z, _e276.g1_.y, _e279.g1_.x, _e282.g1_.z)) * vec4<f32>(1.0, 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e295.g2_.x) * vec4<f32>(_e299.g0_.x, _e302.g0_.x, _e305.g0_.y, _e308.g0_.z)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))) + ((vec4<f32>(_e320.g2_.y) * vec4<f32>(_e324.g0_.x, _e327.g0_.x, _e330.g0_.z, _e333.g0_.y)) * vec4<f32>(-(1.0), 0.0, -(1.0), 1.0))) + ((vec4<f32>(_e347.g2_.z) * vec4<f32>(_e351.g0_.y, _e354.g0_.z, _e357.g0_.y, _e360.g0_.x)) * vec4<f32>(-(1.0), 1.0, 0.0, -(1.0)))) + ((vec4<f32>(_e374.g2_.w) * vec4<f32>(_e378.g0_.z, _e381.g0_.y, _e384.g0_.x, _e387.g0_.z)) * vec4<f32>(-(1.0), -(1.0), 1.0, 0.0))) + ((vec4<f32>(_e401.g1_.x) * vec4<f32>(_e405.g1_.x, _e408.g1_.x, _e411.g1_.y, _e414.g1_.z)) * vec4<f32>(0.0, -(1.0), -(1.0), -(1.0)))), (((((((((vec4<f32>(_e429.g1_.y) * vec4<f32>(_e433.g0_.x, _e436.g0_.x, _e439.g0_.z, _e442.g0_.y)) * vec4<f32>(-(1.0), 0.0, -(1.0), 1.0)) + ((vec4<f32>(_e455.g1_.z) * vec4<f32>(_e459.g0_.y, _e462.g0_.z, _e465.g0_.y, _e468.g0_.x)) * vec4<f32>(-(1.0), 1.0, 0.0, -(1.0)))) + ((vec4<f32>(_e482.g1_.w) * vec4<f32>(_e486.g0_.z, _e489.g0_.y, _e492.g0_.x, _e495.g0_.z)) * vec4<f32>(-(1.0), -(1.0), 1.0, 0.0))) + ((vec4<f32>(_e509.g2_.x) * vec4<f32>(_e513.g1_.x, _e516.g1_.x, _e519.g1_.y, _e522.g1_.z)) * vec4<f32>(0.0, -(1.0), -(1.0), -(1.0)))) + ((vec4<f32>(_e537.g2_.y) * vec4<f32>(_e541.g1_.x, _e544.g1_.x, _e547.g1_.z, _e550.g1_.y)) * vec4<f32>(1.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e563.g2_.z) * vec4<f32>(_e567.g1_.y, _e570.g1_.z, _e573.g1_.y, _e576.g1_.x)) * vec4<f32>(1.0, -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e589.g2_.w) * vec4<f32>(_e593.g1_.z, _e596.g1_.y, _e599.g1_.x, _e602.g1_.z)) * vec4<f32>(1.0, 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e615.g1_.x) * vec4<f32>(_e619.g0_.x, _e622.g0_.x, _e625.g0_.y, _e628.g0_.z)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))), (((((((((vec4<f32>(_e640.g0_.y) * vec4<f32>(_e644.g0_.x, _e647.g0_.x, _e650.g0_.z, _e653.g0_.y)) * vec4<f32>(1.0, 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e665.g0_.z) * vec4<f32>(_e669.g0_.y, _e672.g0_.z, _e675.g0_.y, _e678.g0_.x)) * vec4<f32>(1.0, -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e691.g0_.w) * vec4<f32>(_e695.g0_.z, _e698.g0_.y, _e701.g0_.x, _e704.g0_.z)) * vec4<f32>(1.0, 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e717.g3_.x) * vec4<f32>(_e721.g1_.x, _e724.g1_.x, _e727.g1_.y, _e730.g1_.z)) * vec4<f32>(0.0, -(1.0), -(1.0), -(1.0)))) + ((vec4<f32>(_e745.g3_.y) * vec4<f32>(_e749.g1_.x, _e752.g1_.x, _e755.g1_.z, _e758.g1_.y)) * vec4<f32>(1.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e771.g3_.z) * vec4<f32>(_e775.g1_.y, _e778.g1_.z, _e781.g1_.y, _e784.g1_.x)) * vec4<f32>(1.0, -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e797.g3_.w) * vec4<f32>(_e801.g1_.z, _e804.g1_.y, _e807.g1_.x, _e810.g1_.z)) * vec4<f32>(1.0, 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e823.g0_.x) * vec4<f32>(_e827.g0_.x, _e830.g0_.x, _e833.g0_.y, _e836.g0_.z)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn multi_vector_line_scalar_product(self_256: MultiVector, other_218: Line) -> Scalar {
    var self_257: MultiVector;
    var other_219: Line;

    self_257 = self_256;
    other_219 = other_218;
    let _e5: MultiVector = self_257;
    let _e8: Line = other_219;
    let _e13: MultiVector = self_257;
    let _e16: Line = other_219;
    let _e21: MultiVector = self_257;
    let _e24: Line = other_219;
    let _e29: MultiVector = self_257;
    let _e32: Line = other_219;
    let _e37: MultiVector = self_257;
    let _e40: Line = other_219;
    let _e45: MultiVector = self_257;
    let _e48: Line = other_219;
    return Scalar(((((((0.0 - (_e5.g0_.y * _e8.g1_.x)) - (_e13.g0_.z * _e16.g1_.y)) - (_e21.g0_.w * _e24.g1_.z)) - (_e29.g3_.y * _e32.g0_.x)) - (_e37.g3_.z * _e40.g0_.y)) - (_e45.g3_.w * _e48.g0_.z)));
}

fn multi_vector_translator_into(self_258: MultiVector) -> Translator {
    var self_259: MultiVector;

    self_259 = self_258;
    let _e2: MultiVector = self_259;
    let _e5: MultiVector = self_259;
    let _e8: MultiVector = self_259;
    let _e11: MultiVector = self_259;
    return Translator(vec4<f32>(_e2.g0_.x, _e5.g3_.y, _e8.g3_.z, _e11.g3_.w));
}

fn multi_vector_translator_add(self_260: MultiVector, other_220: Translator) -> MultiVector {
    var self_261: MultiVector;
    var other_221: Translator;

    self_261 = self_260;
    other_221 = other_220;
    let _e4: MultiVector = self_261;
    let _e6: Translator = other_221;
    let _e17: MultiVector = self_261;
    let _e19: MultiVector = self_261;
    let _e21: MultiVector = self_261;
    let _e23: Translator = other_221;
    return MultiVector((_e4.g0_ + (vec4<f32>(_e6.g0_.x) * vec4<f32>(1.0, 0.0, 0.0, 0.0))), _e17.g1_, _e19.g2_, (_e21.g3_ + (_e23.g0_ * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn multi_vector_translator_sub(self_262: MultiVector, other_222: Translator) -> MultiVector {
    var self_263: MultiVector;
    var other_223: Translator;

    self_263 = self_262;
    other_223 = other_222;
    let _e4: MultiVector = self_263;
    let _e6: Translator = other_223;
    let _e17: MultiVector = self_263;
    let _e19: MultiVector = self_263;
    let _e21: MultiVector = self_263;
    let _e23: Translator = other_223;
    return MultiVector((_e4.g0_ - (vec4<f32>(_e6.g0_.x) * vec4<f32>(1.0, 0.0, 0.0, 0.0))), _e17.g1_, _e19.g2_, (_e21.g3_ - (_e23.g0_ * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn multi_vector_translator_geometric_product(self_264: MultiVector, other_224: Translator) -> MultiVector {
    var self_265: MultiVector;
    var other_225: Translator;

    self_265 = self_264;
    other_225 = other_224;
    let _e4: MultiVector = self_265;
    let _e8: Translator = other_225;
    let _e21: MultiVector = self_265;
    let _e25: Translator = other_225;
    let _e38: MultiVector = self_265;
    let _e42: Translator = other_225;
    let _e55: MultiVector = self_265;
    let _e59: Translator = other_225;
    let _e72: MultiVector = self_265;
    let _e74: Translator = other_225;
    let _e80: MultiVector = self_265;
    let _e84: Translator = other_225;
    let _e94: MultiVector = self_265;
    let _e98: Translator = other_225;
    let _e111: MultiVector = self_265;
    let _e115: Translator = other_225;
    let _e128: MultiVector = self_265;
    let _e132: Translator = other_225;
    let _e145: MultiVector = self_265;
    let _e147: Translator = other_225;
    let _e153: MultiVector = self_265;
    let _e157: Translator = other_225;
    let _e169: MultiVector = self_265;
    let _e173: Translator = other_225;
    let _e186: MultiVector = self_265;
    let _e190: Translator = other_225;
    let _e203: MultiVector = self_265;
    let _e207: Translator = other_225;
    let _e219: MultiVector = self_265;
    let _e223: Translator = other_225;
    let _e235: MultiVector = self_265;
    let _e239: Translator = other_225;
    let _e251: MultiVector = self_265;
    let _e254: MultiVector = self_265;
    let _e257: MultiVector = self_265;
    let _e260: MultiVector = self_265;
    let _e264: Translator = other_225;
    let _e268: MultiVector = self_265;
    let _e272: Translator = other_225;
    let _e283: MultiVector = self_265;
    let _e287: Translator = other_225;
    let _e299: MultiVector = self_265;
    let _e303: Translator = other_225;
    let _e315: MultiVector = self_265;
    let _e319: Translator = other_225;
    let _e331: MultiVector = self_265;
    let _e335: Translator = other_225;
    let _e347: MultiVector = self_265;
    let _e351: Translator = other_225;
    let _e363: MultiVector = self_265;
    let _e366: MultiVector = self_265;
    let _e369: MultiVector = self_265;
    let _e372: MultiVector = self_265;
    let _e376: Translator = other_225;
    return MultiVector(((((((vec4<f32>(_e4.g3_.x) * _e8.g0_.yyzw) * vec4<f32>(0.0, -(1.0), -(1.0), -(1.0))) + ((vec4<f32>(_e21.g3_.y) * _e25.g0_.yywz) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e38.g3_.z) * _e42.g0_.zwzy) * vec4<f32>(-(1.0), -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e55.g3_.w) * _e59.g0_.wzyw) * vec4<f32>(-(1.0), 1.0, -(1.0), 0.0))) + (_e72.g0_ * vec4<f32>(_e74.g0_.x))), ((((((vec4<f32>(_e80.g2_.x) * _e84.g0_.yyzw) * vec4<f32>(0.0, 1.0, 1.0, 1.0)) + ((vec4<f32>(_e94.g2_.y) * _e98.g0_.yywz) * vec4<f32>(-(1.0), 0.0, -(1.0), 1.0))) + ((vec4<f32>(_e111.g2_.z) * _e115.g0_.zwzy) * vec4<f32>(-(1.0), 1.0, 0.0, -(1.0)))) + ((vec4<f32>(_e128.g2_.w) * _e132.g0_.wzyw) * vec4<f32>(-(1.0), -(1.0), 1.0, 0.0))) + (_e145.g1_ * vec4<f32>(_e147.g0_.x))), ((((((((vec4<f32>(_e153.g1_.y) * _e157.g0_.yywz) * vec4<f32>(-(1.0), 0.0, -(1.0), 1.0)) + ((vec4<f32>(_e169.g1_.z) * _e173.g0_.zwzy) * vec4<f32>(-(1.0), 1.0, 0.0, -(1.0)))) + ((vec4<f32>(_e186.g1_.w) * _e190.g0_.wzyw) * vec4<f32>(-(1.0), -(1.0), 1.0, 0.0))) + ((vec4<f32>(_e203.g2_.y) * vec4<f32>(_e207.g0_.x)) * vec4<f32>(0.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e219.g2_.z) * vec4<f32>(_e223.g0_.x)) * vec4<f32>(0.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e235.g2_.w) * vec4<f32>(_e239.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + (vec4<f32>(_e251.g2_.x, _e254.g1_.x, _e257.g1_.x, _e260.g1_.x) * _e264.g0_)), ((((((((vec4<f32>(_e268.g0_.y) * _e272.g0_.yywz) * vec4<f32>(1.0, 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e283.g0_.z) * _e287.g0_.zwzy) * vec4<f32>(1.0, -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e299.g0_.w) * _e303.g0_.wzyw) * vec4<f32>(1.0, 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e315.g3_.y) * vec4<f32>(_e319.g0_.x)) * vec4<f32>(0.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e331.g3_.z) * vec4<f32>(_e335.g0_.x)) * vec4<f32>(0.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e347.g3_.w) * vec4<f32>(_e351.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + (vec4<f32>(_e363.g3_.x, _e366.g0_.x, _e369.g0_.x, _e372.g0_.x) * _e376.g0_)));
}

fn multi_vector_translator_outer_product(self_266: MultiVector, other_226: Translator) -> MultiVector {
    var self_267: MultiVector;
    var other_227: Translator;

    self_267 = self_266;
    other_227 = other_226;
    let _e4: MultiVector = self_267;
    let _e6: Translator = other_227;
    let _e11: MultiVector = self_267;
    let _e15: Translator = other_227;
    let _e26: MultiVector = self_267;
    let _e30: Translator = other_227;
    let _e42: MultiVector = self_267;
    let _e46: Translator = other_227;
    let _e58: MultiVector = self_267;
    let _e60: Translator = other_227;
    let _e66: MultiVector = self_267;
    let _e68: Translator = other_227;
    let _e73: MultiVector = self_267;
    let _e77: Translator = other_227;
    let _e88: MultiVector = self_267;
    let _e92: Translator = other_227;
    let _e104: MultiVector = self_267;
    let _e108: Translator = other_227;
    let _e120: MultiVector = self_267;
    let _e124: Translator = other_227;
    let _e136: MultiVector = self_267;
    let _e140: Translator = other_227;
    let _e152: MultiVector = self_267;
    let _e156: Translator = other_227;
    let _e168: MultiVector = self_267;
    let _e171: Translator = other_227;
    return MultiVector((_e4.g0_ * vec4<f32>(_e6.g0_.x)), (((((vec4<f32>(_e11.g2_.y) * _e15.g0_.wwwz) * vec4<f32>(0.0, 0.0, -(1.0), 1.0)) + ((vec4<f32>(_e26.g2_.z) * _e30.g0_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0)))) + ((vec4<f32>(_e42.g2_.w) * _e46.g0_.zzyz) * vec4<f32>(0.0, -(1.0), 1.0, 0.0))) + (_e58.g1_ * vec4<f32>(_e60.g0_.x))), (_e66.g2_ * vec4<f32>(_e68.g0_.x)), ((((((((vec4<f32>(_e73.g0_.z) * vec4<f32>(_e77.g0_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) + ((vec4<f32>(_e88.g0_.w) * vec4<f32>(_e92.g0_.w)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e104.g3_.x) * vec4<f32>(_e108.g0_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e120.g3_.y) * vec4<f32>(_e124.g0_.x)) * vec4<f32>(0.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e136.g3_.z) * vec4<f32>(_e140.g0_.x)) * vec4<f32>(0.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e152.g3_.w) * vec4<f32>(_e156.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + (_e168.g0_.yxxx * _e171.g0_.yyzw)));
}

fn multi_vector_translator_inner_product(self_268: MultiVector, other_228: Translator) -> MultiVector {
    var self_269: MultiVector;
    var other_229: Translator;

    self_269 = self_268;
    other_229 = other_228;
    let _e4: MultiVector = self_269;
    let _e8: Translator = other_229;
    let _e21: MultiVector = self_269;
    let _e25: Translator = other_229;
    let _e38: MultiVector = self_269;
    let _e42: Translator = other_229;
    let _e55: MultiVector = self_269;
    let _e59: Translator = other_229;
    let _e72: MultiVector = self_269;
    let _e74: Translator = other_229;
    let _e80: MultiVector = self_269;
    let _e84: Translator = other_229;
    let _e96: MultiVector = self_269;
    let _e100: Translator = other_229;
    let _e113: MultiVector = self_269;
    let _e117: Translator = other_229;
    let _e130: MultiVector = self_269;
    let _e132: Translator = other_229;
    let _e138: MultiVector = self_269;
    let _e142: Translator = other_229;
    let _e153: MultiVector = self_269;
    let _e157: Translator = other_229;
    let _e169: MultiVector = self_269;
    let _e173: Translator = other_229;
    let _e185: MultiVector = self_269;
    let _e189: Translator = other_229;
    let _e201: MultiVector = self_269;
    let _e205: Translator = other_229;
    let _e217: MultiVector = self_269;
    let _e221: Translator = other_229;
    let _e233: MultiVector = self_269;
    let _e236: MultiVector = self_269;
    let _e239: MultiVector = self_269;
    let _e242: MultiVector = self_269;
    let _e246: Translator = other_229;
    let _e250: MultiVector = self_269;
    let _e254: Translator = other_229;
    let _e265: MultiVector = self_269;
    let _e269: Translator = other_229;
    let _e281: MultiVector = self_269;
    let _e285: Translator = other_229;
    let _e297: MultiVector = self_269;
    let _e300: MultiVector = self_269;
    let _e303: MultiVector = self_269;
    let _e306: MultiVector = self_269;
    let _e310: Translator = other_229;
    return MultiVector(((((((vec4<f32>(_e4.g3_.x) * _e8.g0_.yyzw) * vec4<f32>(0.0, -(1.0), -(1.0), -(1.0))) + ((vec4<f32>(_e21.g3_.y) * vec4<f32>(_e25.g0_.y)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((vec4<f32>(_e38.g3_.z) * vec4<f32>(_e42.g0_.z)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((vec4<f32>(_e55.g3_.w) * vec4<f32>(_e59.g0_.w)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + (_e72.g0_ * vec4<f32>(_e74.g0_.x))), (((((vec4<f32>(_e80.g2_.y) * vec4<f32>(_e84.g0_.y)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0)) + ((vec4<f32>(_e96.g2_.z) * vec4<f32>(_e100.g0_.z)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((vec4<f32>(_e113.g2_.w) * vec4<f32>(_e117.g0_.w)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + (_e130.g1_ * vec4<f32>(_e132.g0_.x))), ((((((((vec4<f32>(_e138.g1_.y) * _e142.g0_.wwwz) * vec4<f32>(0.0, 0.0, -(1.0), 1.0)) + ((vec4<f32>(_e153.g1_.z) * _e157.g0_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0)))) + ((vec4<f32>(_e169.g1_.w) * _e173.g0_.zzyz) * vec4<f32>(0.0, -(1.0), 1.0, 0.0))) + ((vec4<f32>(_e185.g2_.y) * vec4<f32>(_e189.g0_.x)) * vec4<f32>(0.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e201.g2_.z) * vec4<f32>(_e205.g0_.x)) * vec4<f32>(0.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e217.g2_.w) * vec4<f32>(_e221.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + (vec4<f32>(_e233.g2_.x, _e236.g1_.x, _e239.g1_.x, _e242.g1_.x) * _e246.g0_)), (((((vec4<f32>(_e250.g3_.y) * vec4<f32>(_e254.g0_.x)) * vec4<f32>(0.0, 1.0, 0.0, 0.0)) + ((vec4<f32>(_e265.g3_.z) * vec4<f32>(_e269.g0_.x)) * vec4<f32>(0.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e281.g3_.w) * vec4<f32>(_e285.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + (vec4<f32>(_e297.g3_.x, _e300.g0_.x, _e303.g0_.x, _e306.g0_.x) * _e310.g0_)));
}

fn multi_vector_translator_right_contraction(self_270: MultiVector, other_230: Translator) -> MultiVector {
    var self_271: MultiVector;
    var other_231: Translator;

    self_271 = self_270;
    other_231 = other_230;
    let _e4: MultiVector = self_271;
    let _e8: Translator = other_231;
    let _e21: MultiVector = self_271;
    let _e25: Translator = other_231;
    let _e38: MultiVector = self_271;
    let _e42: Translator = other_231;
    let _e55: MultiVector = self_271;
    let _e59: Translator = other_231;
    let _e72: MultiVector = self_271;
    let _e74: Translator = other_231;
    let _e80: MultiVector = self_271;
    let _e82: Translator = other_231;
    let _e87: MultiVector = self_271;
    let _e91: Translator = other_231;
    let _e102: MultiVector = self_271;
    let _e106: Translator = other_231;
    let _e118: MultiVector = self_271;
    let _e122: Translator = other_231;
    let _e134: MultiVector = self_271;
    let _e138: Translator = other_231;
    let _e150: MultiVector = self_271;
    let _e153: MultiVector = self_271;
    let _e156: MultiVector = self_271;
    let _e159: MultiVector = self_271;
    let _e163: Translator = other_231;
    let _e175: MultiVector = self_271;
    let _e177: Translator = other_231;
    return MultiVector(((((((vec4<f32>(_e4.g3_.x) * _e8.g0_.yyzw) * vec4<f32>(0.0, -(1.0), -(1.0), -(1.0))) + ((vec4<f32>(_e21.g3_.y) * vec4<f32>(_e25.g0_.y)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((vec4<f32>(_e38.g3_.z) * vec4<f32>(_e42.g0_.z)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((vec4<f32>(_e55.g3_.w) * vec4<f32>(_e59.g0_.w)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + (_e72.g0_ * vec4<f32>(_e74.g0_.x))), (_e80.g1_ * vec4<f32>(_e82.g0_.x)), ((((((vec4<f32>(_e87.g1_.z) * _e91.g0_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e102.g1_.w) * _e106.g0_.zzyz) * vec4<f32>(0.0, -(1.0), 1.0, 0.0))) + ((vec4<f32>(_e118.g2_.z) * vec4<f32>(_e122.g0_.x)) * vec4<f32>(0.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e134.g2_.w) * vec4<f32>(_e138.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e150.g2_.x, _e153.g2_.y, _e156.g1_.y, _e159.g1_.y) * _e163.g0_.xxwz) * vec4<f32>(1.0, 1.0, -(1.0), 1.0))), (_e175.g3_ * vec4<f32>(_e177.g0_.x)));
}

fn multi_vector_translator_scalar_product(self_272: MultiVector, other_232: Translator) -> Scalar {
    var self_273: MultiVector;
    var other_233: Translator;

    self_273 = self_272;
    other_233 = other_232;
    let _e4: MultiVector = self_273;
    let _e7: Translator = other_233;
    let _e11: MultiVector = self_273;
    let _e14: Translator = other_233;
    let _e19: MultiVector = self_273;
    let _e22: Translator = other_233;
    let _e27: MultiVector = self_273;
    let _e30: Translator = other_233;
    return Scalar(((((_e4.g0_.x * _e7.g0_.x) - (_e11.g3_.y * _e14.g0_.y)) - (_e19.g3_.z * _e22.g0_.z)) - (_e27.g3_.w * _e30.g0_.w)));
}

fn multi_vector_motor_into(self_274: MultiVector) -> Motor {
    var self_275: MultiVector;

    self_275 = self_274;
    let _e2: MultiVector = self_275;
    let _e4: MultiVector = self_275;
    return Motor(_e2.g0_, _e4.g3_);
}

fn multi_vector_motor_add(self_276: MultiVector, other_234: Motor) -> MultiVector {
    var self_277: MultiVector;
    var other_235: Motor;

    self_277 = self_276;
    other_235 = other_234;
    let _e4: MultiVector = self_277;
    let _e6: Motor = other_235;
    let _e9: MultiVector = self_277;
    let _e11: MultiVector = self_277;
    let _e13: MultiVector = self_277;
    let _e15: Motor = other_235;
    return MultiVector((_e4.g0_ + _e6.g0_), _e9.g1_, _e11.g2_, (_e13.g3_ + _e15.g1_));
}

fn multi_vector_motor_sub(self_278: MultiVector, other_236: Motor) -> MultiVector {
    var self_279: MultiVector;
    var other_237: Motor;

    self_279 = self_278;
    other_237 = other_236;
    let _e4: MultiVector = self_279;
    let _e6: Motor = other_237;
    let _e9: MultiVector = self_279;
    let _e11: MultiVector = self_279;
    let _e13: MultiVector = self_279;
    let _e15: Motor = other_237;
    return MultiVector((_e4.g0_ - _e6.g0_), _e9.g1_, _e11.g2_, (_e13.g3_ - _e15.g1_));
}

fn multi_vector_motor_geometric_product(self_280: MultiVector, other_238: Motor) -> MultiVector {
    var self_281: MultiVector;
    var other_239: Motor;

    self_281 = self_280;
    other_239 = other_238;
    let _e4: MultiVector = self_281;
    let _e8: Motor = other_239;
    let _e11: MultiVector = self_281;
    let _e15: Motor = other_239;
    let _e28: MultiVector = self_281;
    let _e32: Motor = other_239;
    let _e45: MultiVector = self_281;
    let _e49: Motor = other_239;
    let _e62: MultiVector = self_281;
    let _e66: Motor = other_239;
    let _e79: MultiVector = self_281;
    let _e83: Motor = other_239;
    let _e97: MultiVector = self_281;
    let _e101: Motor = other_239;
    let _e115: MultiVector = self_281;
    let _e119: Motor = other_239;
    let _e133: MultiVector = self_281;
    let _e137: Motor = other_239;
    let _e149: MultiVector = self_281;
    let _e153: Motor = other_239;
    let _e165: MultiVector = self_281;
    let _e169: Motor = other_239;
    let _e181: MultiVector = self_281;
    let _e185: Motor = other_239;
    let _e197: MultiVector = self_281;
    let _e201: Motor = other_239;
    let _e205: MultiVector = self_281;
    let _e209: Motor = other_239;
    let _e222: MultiVector = self_281;
    let _e226: Motor = other_239;
    let _e239: MultiVector = self_281;
    let _e243: Motor = other_239;
    let _e256: MultiVector = self_281;
    let _e260: Motor = other_239;
    let _e263: MultiVector = self_281;
    let _e267: Motor = other_239;
    let _e280: MultiVector = self_281;
    let _e284: Motor = other_239;
    let _e297: MultiVector = self_281;
    let _e301: Motor = other_239;
    let _e314: MultiVector = self_281;
    let _e318: Motor = other_239;
    let _e331: MultiVector = self_281;
    let _e335: Motor = other_239;
    let _e347: MultiVector = self_281;
    let _e351: Motor = other_239;
    let _e363: MultiVector = self_281;
    let _e367: Motor = other_239;
    let _e379: MultiVector = self_281;
    let _e383: Motor = other_239;
    let _e386: MultiVector = self_281;
    let _e390: Motor = other_239;
    let _e403: MultiVector = self_281;
    let _e407: Motor = other_239;
    let _e420: MultiVector = self_281;
    let _e424: Motor = other_239;
    let _e437: MultiVector = self_281;
    let _e441: Motor = other_239;
    let _e454: MultiVector = self_281;
    let _e458: Motor = other_239;
    let _e470: MultiVector = self_281;
    let _e474: Motor = other_239;
    let _e486: MultiVector = self_281;
    let _e490: Motor = other_239;
    return MultiVector(((((((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.y) * _e15.g0_.yxwz) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e28.g0_.z) * _e32.g0_.zwxy) * vec4<f32>(-(1.0), -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e45.g0_.w) * _e49.g0_.wzyx) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))) + ((vec4<f32>(_e62.g3_.x) * _e66.g1_) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0)))) + ((vec4<f32>(_e79.g3_.y) * _e83.g1_.yxwz) * vec4<f32>(-(1.0), -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e97.g3_.z) * _e101.g1_.zwxy) * vec4<f32>(-(1.0), -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e115.g3_.w) * _e119.g1_.wzyx) * vec4<f32>(-(1.0), 1.0, -(1.0), -(1.0)))), (((((((((vec4<f32>(_e133.g1_.x) * _e137.g0_) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))) + ((vec4<f32>(_e149.g1_.y) * _e153.g0_.yxwz) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e165.g1_.z) * _e169.g0_.zwxy) * vec4<f32>(1.0, -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e181.g1_.w) * _e185.g0_.wzyx) * vec4<f32>(1.0, 1.0, -(1.0), 1.0))) + (vec4<f32>(_e197.g2_.x) * _e201.g1_)) + ((vec4<f32>(_e205.g2_.y) * _e209.g1_.yxwz) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))) + ((vec4<f32>(_e222.g2_.z) * _e226.g1_.zwxy) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e239.g2_.w) * _e243.g1_.wzyx) * vec4<f32>(-(1.0), -(1.0), 1.0, 1.0))), ((((((((vec4<f32>(_e256.g1_.x) * _e260.g1_) + ((vec4<f32>(_e263.g1_.y) * _e267.g1_.yxwz) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))) + ((vec4<f32>(_e280.g1_.z) * _e284.g1_.zwxy) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e297.g1_.w) * _e301.g1_.wzyx) * vec4<f32>(-(1.0), -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e314.g2_.x) * _e318.g0_) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0)))) + ((vec4<f32>(_e331.g2_.y) * _e335.g0_.yxwz) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e347.g2_.z) * _e351.g0_.zwxy) * vec4<f32>(1.0, -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e363.g2_.w) * _e367.g0_.wzyx) * vec4<f32>(1.0, 1.0, -(1.0), 1.0))), ((((((((vec4<f32>(_e379.g0_.x) * _e383.g1_) + ((vec4<f32>(_e386.g0_.y) * _e390.g1_.yxwz) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e403.g0_.z) * _e407.g1_.zwxy) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e420.g0_.w) * _e424.g1_.wzyx) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e437.g3_.x) * _e441.g0_) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0)))) + ((vec4<f32>(_e454.g3_.y) * _e458.g0_.yxwz) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e470.g3_.z) * _e474.g0_.zwxy) * vec4<f32>(1.0, -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e486.g3_.w) * _e490.g0_.wzyx) * vec4<f32>(1.0, 1.0, -(1.0), 1.0))));
}

fn multi_vector_motor_regressive_product(self_282: MultiVector, other_240: Motor) -> MultiVector {
    var self_283: MultiVector;
    var other_241: Motor;

    self_283 = self_282;
    other_241 = other_240;
    let _e4: MultiVector = self_283;
    let _e8: Motor = other_241;
    let _e18: MultiVector = self_283;
    let _e22: Motor = other_241;
    let _e33: MultiVector = self_283;
    let _e37: Motor = other_241;
    let _e48: MultiVector = self_283;
    let _e52: Motor = other_241;
    let _e56: MultiVector = self_283;
    let _e60: Motor = other_241;
    let _e72: MultiVector = self_283;
    let _e76: Motor = other_241;
    let _e88: MultiVector = self_283;
    let _e92: Motor = other_241;
    let _e104: MultiVector = self_283;
    let _e108: Motor = other_241;
    let _e120: MultiVector = self_283;
    let _e124: Motor = other_241;
    let _e135: MultiVector = self_283;
    let _e139: Motor = other_241;
    let _e151: MultiVector = self_283;
    let _e155: Motor = other_241;
    let _e167: MultiVector = self_283;
    let _e171: Motor = other_241;
    let _e183: MultiVector = self_283;
    let _e187: Motor = other_241;
    let _e198: MultiVector = self_283;
    let _e202: Motor = other_241;
    let _e214: MultiVector = self_283;
    let _e218: Motor = other_241;
    let _e222: MultiVector = self_283;
    let _e226: Motor = other_241;
    let _e238: MultiVector = self_283;
    let _e242: Motor = other_241;
    let _e254: MultiVector = self_283;
    let _e257: MultiVector = self_283;
    let _e260: MultiVector = self_283;
    let _e263: MultiVector = self_283;
    let _e267: Motor = other_241;
    let _e270: Motor = other_241;
    let _e273: Motor = other_241;
    let _e276: Motor = other_241;
    let _e289: MultiVector = self_283;
    let _e293: Motor = other_241;
    let _e296: MultiVector = self_283;
    let _e298: Motor = other_241;
    return MultiVector((((((((((vec4<f32>(_e4.g0_.y) * _e8.g1_.yxyy) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) + ((vec4<f32>(_e18.g0_.z) * _e22.g1_.zzxz) * vec4<f32>(1.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e33.g0_.w) * _e37.g1_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, 1.0))) + (vec4<f32>(_e48.g3_.x) * _e52.g0_)) + ((vec4<f32>(_e56.g3_.y) * vec4<f32>(_e60.g0_.y)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e72.g3_.z) * vec4<f32>(_e76.g0_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e88.g3_.w) * vec4<f32>(_e92.g0_.w)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e104.g0_.x) * vec4<f32>(_e108.g1_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))), (((((vec4<f32>(_e120.g1_.y) * _e124.g1_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0)) + ((vec4<f32>(_e135.g1_.z) * _e139.g1_.zzxz) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0))) + ((vec4<f32>(_e151.g1_.w) * _e155.g1_.wwwx) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))) + ((vec4<f32>(_e167.g1_.x) * vec4<f32>(_e171.g1_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))), (((((((vec4<f32>(_e183.g1_.z) * _e187.g0_.wwwy) * vec4<f32>(0.0, -(1.0), 0.0, 1.0)) + ((vec4<f32>(_e198.g1_.w) * _e202.g0_.zzyz) * vec4<f32>(0.0, 1.0, -(1.0), 0.0))) + (vec4<f32>(_e214.g2_.x) * _e218.g1_)) + ((vec4<f32>(_e222.g2_.z) * vec4<f32>(_e226.g1_.x)) * vec4<f32>(0.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e238.g2_.w) * vec4<f32>(_e242.g1_.x)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e254.g1_.x, _e257.g2_.y, _e260.g1_.y, _e263.g1_.y) * vec4<f32>(_e267.g0_.x, _e270.g1_.x, _e273.g0_.w, _e276.g0_.z)) * vec4<f32>(0.0, 1.0, 1.0, -(1.0)))), ((vec4<f32>(_e289.g3_.x) * _e293.g1_) + ((_e296.g3_ * vec4<f32>(_e298.g1_.x)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn multi_vector_motor_outer_product(self_284: MultiVector, other_242: Motor) -> MultiVector {
    var self_285: MultiVector;
    var other_243: Motor;

    self_285 = self_284;
    other_243 = other_242;
    let _e4: MultiVector = self_285;
    let _e8: Motor = other_243;
    let _e11: MultiVector = self_285;
    let _e13: Motor = other_243;
    let _e25: MultiVector = self_285;
    let _e29: Motor = other_243;
    let _e41: MultiVector = self_285;
    let _e45: Motor = other_243;
    let _e57: MultiVector = self_285;
    let _e61: Motor = other_243;
    let _e73: MultiVector = self_285;
    let _e77: Motor = other_243;
    let _e89: MultiVector = self_285;
    let _e91: Motor = other_243;
    let _e103: MultiVector = self_285;
    let _e107: Motor = other_243;
    let _e117: MultiVector = self_285;
    let _e121: Motor = other_243;
    let _e132: MultiVector = self_285;
    let _e136: Motor = other_243;
    let _e147: MultiVector = self_285;
    let _e151: Motor = other_243;
    let _e163: MultiVector = self_285;
    let _e167: Motor = other_243;
    let _e170: MultiVector = self_285;
    let _e174: Motor = other_243;
    let _e186: MultiVector = self_285;
    let _e190: Motor = other_243;
    let _e202: MultiVector = self_285;
    let _e206: Motor = other_243;
    let _e218: MultiVector = self_285;
    let _e222: Motor = other_243;
    let _e233: MultiVector = self_285;
    let _e237: Motor = other_243;
    let _e248: MultiVector = self_285;
    let _e252: Motor = other_243;
    let _e263: MultiVector = self_285;
    let _e266: Motor = other_243;
    return MultiVector(((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((_e11.g0_ * vec4<f32>(_e13.g0_.x)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))), ((((((vec4<f32>(_e25.g1_.x) * _e29.g0_) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))) + ((vec4<f32>(_e41.g2_.y) * _e45.g1_.wwwz) * vec4<f32>(0.0, 0.0, -(1.0), 1.0))) + ((vec4<f32>(_e57.g2_.z) * _e61.g1_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0)))) + ((vec4<f32>(_e73.g2_.w) * _e77.g1_.zzyz) * vec4<f32>(0.0, -(1.0), 1.0, 0.0))) + ((_e89.g1_ * vec4<f32>(_e91.g0_.x)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))), (((((vec4<f32>(_e103.g2_.y) * _e107.g0_.yxyy) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) + ((vec4<f32>(_e117.g2_.z) * _e121.g0_.zzxz) * vec4<f32>(1.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e132.g2_.w) * _e136.g0_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e147.g2_.x) * vec4<f32>(_e151.g0_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))), ((((((((vec4<f32>(_e163.g0_.x) * _e167.g1_) + ((vec4<f32>(_e170.g0_.z) * vec4<f32>(_e174.g1_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e186.g0_.w) * vec4<f32>(_e190.g1_.w)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e202.g3_.x) * vec4<f32>(_e206.g0_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e218.g3_.y) * _e222.g0_.yxyy) * vec4<f32>(1.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e233.g3_.z) * _e237.g0_.zzxz) * vec4<f32>(1.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e248.g3_.w) * _e252.g0_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, 1.0))) + ((_e263.g0_.yxxx * _e266.g1_.yxxx) * vec4<f32>(1.0, 0.0, 0.0, 0.0))));
}

fn multi_vector_motor_inner_product(self_286: MultiVector, other_244: Motor) -> MultiVector {
    var self_287: MultiVector;
    var other_245: Motor;

    self_287 = self_286;
    other_245 = other_244;
    let _e4: MultiVector = self_287;
    let _e8: Motor = other_245;
    let _e11: MultiVector = self_287;
    let _e15: Motor = other_245;
    let _e27: MultiVector = self_287;
    let _e31: Motor = other_245;
    let _e43: MultiVector = self_287;
    let _e47: Motor = other_245;
    let _e60: MultiVector = self_287;
    let _e64: Motor = other_245;
    let _e77: MultiVector = self_287;
    let _e81: Motor = other_245;
    let _e94: MultiVector = self_287;
    let _e98: Motor = other_245;
    let _e111: MultiVector = self_287;
    let _e114: Motor = other_245;
    let _e126: MultiVector = self_287;
    let _e130: Motor = other_245;
    let _e140: MultiVector = self_287;
    let _e144: Motor = other_245;
    let _e155: MultiVector = self_287;
    let _e159: Motor = other_245;
    let _e170: MultiVector = self_287;
    let _e174: Motor = other_245;
    let _e186: MultiVector = self_287;
    let _e190: Motor = other_245;
    let _e202: MultiVector = self_287;
    let _e206: Motor = other_245;
    let _e218: MultiVector = self_287;
    let _e222: Motor = other_245;
    let _e234: MultiVector = self_287;
    let _e238: Motor = other_245;
    let _e250: MultiVector = self_287;
    let _e254: Motor = other_245;
    let _e257: MultiVector = self_287;
    let _e261: Motor = other_245;
    let _e273: MultiVector = self_287;
    let _e277: Motor = other_245;
    let _e289: MultiVector = self_287;
    let _e293: Motor = other_245;
    let _e306: MultiVector = self_287;
    let _e310: Motor = other_245;
    let _e322: MultiVector = self_287;
    let _e326: Motor = other_245;
    let _e338: MultiVector = self_287;
    let _e342: Motor = other_245;
    let _e354: MultiVector = self_287;
    let _e357: Motor = other_245;
    let _e369: MultiVector = self_287;
    let _e373: Motor = other_245;
    let _e376: MultiVector = self_287;
    let _e380: Motor = other_245;
    let _e393: MultiVector = self_287;
    let _e397: Motor = other_245;
    let _e409: MultiVector = self_287;
    let _e413: Motor = other_245;
    let _e425: MultiVector = self_287;
    let _e429: Motor = other_245;
    let _e441: MultiVector = self_287;
    let _e443: Motor = other_245;
    return MultiVector(((((((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.z) * _e15.g0_.zzxz) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0))) + ((vec4<f32>(_e27.g0_.w) * _e31.g0_.wwwx) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))) + ((vec4<f32>(_e43.g3_.x) * _e47.g1_) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0)))) + ((vec4<f32>(_e60.g3_.y) * _e64.g1_.yxyy) * vec4<f32>(-(1.0), -(1.0), 0.0, 0.0))) + ((vec4<f32>(_e77.g3_.z) * _e81.g1_.zzxz) * vec4<f32>(-(1.0), 0.0, -(1.0), 0.0))) + ((vec4<f32>(_e94.g3_.w) * _e98.g1_.wwwx) * vec4<f32>(-(1.0), 0.0, 0.0, -(1.0)))) + ((_e111.g0_.yyxx * _e114.g0_.yxxx) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))), (((((((((vec4<f32>(_e126.g1_.y) * _e130.g0_.yxyy) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) + ((vec4<f32>(_e140.g1_.z) * _e144.g0_.zzxz) * vec4<f32>(1.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e155.g1_.w) * _e159.g0_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e170.g2_.x) * vec4<f32>(_e174.g1_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e186.g2_.y) * _e190.g1_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))) + ((vec4<f32>(_e202.g2_.z) * _e206.g1_.zzxz) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0))) + ((vec4<f32>(_e218.g2_.w) * _e222.g1_.wwwx) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))) + ((vec4<f32>(_e234.g1_.x) * vec4<f32>(_e238.g0_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))), ((((((((vec4<f32>(_e250.g1_.x) * _e254.g1_) + ((vec4<f32>(_e257.g1_.z) * _e261.g1_.wwxy) * vec4<f32>(0.0, 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e273.g1_.w) * _e277.g1_.zzyx) * vec4<f32>(0.0, -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e289.g2_.x) * _e293.g0_) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0)))) + ((vec4<f32>(_e306.g2_.y) * _e310.g0_.xxwz) * vec4<f32>(0.0, 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e322.g2_.z) * _e326.g0_.wwxy) * vec4<f32>(0.0, -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e338.g2_.w) * _e342.g0_.zzyx) * vec4<f32>(0.0, 1.0, -(1.0), 1.0))) + ((_e354.g1_.xyyy * _e357.g1_.xxwz) * vec4<f32>(0.0, 1.0, -(1.0), 1.0))), ((((((vec4<f32>(_e369.g0_.x) * _e373.g1_) + ((vec4<f32>(_e376.g3_.x) * _e380.g0_) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0)))) + ((vec4<f32>(_e393.g3_.y) * vec4<f32>(_e397.g0_.x)) * vec4<f32>(0.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e409.g3_.z) * vec4<f32>(_e413.g0_.x)) * vec4<f32>(0.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e425.g3_.w) * vec4<f32>(_e429.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((_e441.g0_ * vec4<f32>(_e443.g1_.x)) * vec4<f32>(0.0, -(1.0), -(1.0), -(1.0)))));
}

fn multi_vector_motor_left_contraction(self_288: MultiVector, other_246: Motor) -> MultiVector {
    var self_289: MultiVector;
    var other_247: Motor;

    self_289 = self_288;
    other_247 = other_246;
    let _e4: MultiVector = self_289;
    let _e8: Motor = other_247;
    let _e11: MultiVector = self_289;
    let _e15: Motor = other_247;
    let _e28: MultiVector = self_289;
    let _e32: Motor = other_247;
    let _e45: MultiVector = self_289;
    let _e49: Motor = other_247;
    let _e61: MultiVector = self_289;
    let _e65: Motor = other_247;
    let _e78: MultiVector = self_289;
    let _e82: Motor = other_247;
    let _e95: MultiVector = self_289;
    let _e99: Motor = other_247;
    let _e112: MultiVector = self_289;
    let _e115: Motor = other_247;
    let _e127: MultiVector = self_289;
    let _e131: Motor = other_247;
    let _e142: MultiVector = self_289;
    let _e146: Motor = other_247;
    let _e158: MultiVector = self_289;
    let _e162: Motor = other_247;
    let _e174: MultiVector = self_289;
    let _e178: Motor = other_247;
    let _e190: MultiVector = self_289;
    let _e194: Motor = other_247;
    let _e197: MultiVector = self_289;
    let _e201: Motor = other_247;
    let _e213: MultiVector = self_289;
    let _e217: Motor = other_247;
    let _e229: MultiVector = self_289;
    let _e233: Motor = other_247;
    let _e245: MultiVector = self_289;
    let _e247: Motor = other_247;
    let _e259: MultiVector = self_289;
    let _e263: Motor = other_247;
    let _e266: MultiVector = self_289;
    let _e268: Motor = other_247;
    return MultiVector(((((((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.z) * vec4<f32>(_e15.g0_.z)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((vec4<f32>(_e28.g0_.w) * vec4<f32>(_e32.g0_.w)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((vec4<f32>(_e45.g3_.x) * vec4<f32>(_e49.g1_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e61.g3_.y) * _e65.g1_.yxyy) * vec4<f32>(-(1.0), -(1.0), 0.0, 0.0))) + ((vec4<f32>(_e78.g3_.z) * _e82.g1_.zzxz) * vec4<f32>(-(1.0), 0.0, -(1.0), 0.0))) + ((vec4<f32>(_e95.g3_.w) * _e99.g1_.wwwx) * vec4<f32>(-(1.0), 0.0, 0.0, -(1.0)))) + ((_e112.g0_.yxxx * _e115.g0_.yxxx) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))), (((((vec4<f32>(_e127.g2_.y) * _e131.g1_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0)) + ((vec4<f32>(_e142.g2_.z) * _e146.g1_.zzxz) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0))) + ((vec4<f32>(_e158.g2_.w) * _e162.g1_.wwwx) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))) + ((vec4<f32>(_e174.g2_.x) * vec4<f32>(_e178.g1_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))), (((((vec4<f32>(_e190.g1_.x) * _e194.g1_) + ((vec4<f32>(_e197.g2_.y) * _e201.g0_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e213.g2_.z) * _e217.g0_.wwwy) * vec4<f32>(0.0, -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e229.g2_.w) * _e233.g0_.zzyz) * vec4<f32>(0.0, 1.0, -(1.0), 0.0))) + ((_e245.g1_ * vec4<f32>(_e247.g1_.x)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))), ((vec4<f32>(_e259.g0_.x) * _e263.g1_) + ((_e266.g0_ * vec4<f32>(_e268.g1_.x)) * vec4<f32>(0.0, -(1.0), -(1.0), -(1.0)))));
}

fn multi_vector_motor_right_contraction(self_290: MultiVector, other_248: Motor) -> MultiVector {
    var self_291: MultiVector;
    var other_249: Motor;

    self_291 = self_290;
    other_249 = other_248;
    let _e4: MultiVector = self_291;
    let _e8: Motor = other_249;
    let _e19: MultiVector = self_291;
    let _e23: Motor = other_249;
    let _e35: MultiVector = self_291;
    let _e39: Motor = other_249;
    let _e51: MultiVector = self_291;
    let _e55: Motor = other_249;
    let _e68: MultiVector = self_291;
    let _e72: Motor = other_249;
    let _e85: MultiVector = self_291;
    let _e89: Motor = other_249;
    let _e102: MultiVector = self_291;
    let _e106: Motor = other_249;
    let _e119: MultiVector = self_291;
    let _e123: Motor = other_249;
    let _e135: MultiVector = self_291;
    let _e139: Motor = other_249;
    let _e149: MultiVector = self_291;
    let _e153: Motor = other_249;
    let _e164: MultiVector = self_291;
    let _e168: Motor = other_249;
    let _e179: MultiVector = self_291;
    let _e183: Motor = other_249;
    let _e195: MultiVector = self_291;
    let _e199: Motor = other_249;
    let _e210: MultiVector = self_291;
    let _e214: Motor = other_249;
    let _e226: MultiVector = self_291;
    let _e230: Motor = other_249;
    let _e243: MultiVector = self_291;
    let _e247: Motor = other_249;
    let _e259: MultiVector = self_291;
    let _e263: Motor = other_249;
    let _e275: MultiVector = self_291;
    let _e278: MultiVector = self_291;
    let _e281: MultiVector = self_291;
    let _e284: MultiVector = self_291;
    let _e288: Motor = other_249;
    let _e291: Motor = other_249;
    let _e294: Motor = other_249;
    let _e297: Motor = other_249;
    let _e310: MultiVector = self_291;
    let _e314: Motor = other_249;
    let _e326: MultiVector = self_291;
    let _e328: Motor = other_249;
    return MultiVector((((((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0)) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.zzxz) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0))) + ((vec4<f32>(_e35.g0_.w) * _e39.g0_.wwwx) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))) + ((vec4<f32>(_e51.g3_.x) * _e55.g1_) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0)))) + ((vec4<f32>(_e68.g3_.y) * vec4<f32>(_e72.g1_.y)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((vec4<f32>(_e85.g3_.z) * vec4<f32>(_e89.g1_.z)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((vec4<f32>(_e102.g3_.w) * vec4<f32>(_e106.g1_.w)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((vec4<f32>(_e119.g0_.x) * vec4<f32>(_e123.g0_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))), (((((vec4<f32>(_e135.g1_.y) * _e139.g0_.yxyy) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) + ((vec4<f32>(_e149.g1_.z) * _e153.g0_.zzxz) * vec4<f32>(1.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e164.g1_.w) * _e168.g0_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e179.g1_.x) * vec4<f32>(_e183.g0_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))), (((((((vec4<f32>(_e195.g1_.z) * _e199.g1_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e210.g1_.w) * _e214.g1_.zzyz) * vec4<f32>(0.0, -(1.0), 1.0, 0.0))) + ((vec4<f32>(_e226.g2_.x) * _e230.g0_) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0)))) + ((vec4<f32>(_e243.g2_.z) * vec4<f32>(_e247.g0_.x)) * vec4<f32>(0.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e259.g2_.w) * vec4<f32>(_e263.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e275.g1_.x, _e278.g2_.y, _e281.g1_.y, _e284.g1_.y) * vec4<f32>(_e288.g1_.x, _e291.g0_.x, _e294.g1_.w, _e297.g1_.z)) * vec4<f32>(0.0, 1.0, -(1.0), 1.0))), (((vec4<f32>(_e310.g3_.x) * _e314.g0_) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))) + ((_e326.g3_ * vec4<f32>(_e328.g0_.x)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn multi_vector_motor_scalar_product(self_292: MultiVector, other_250: Motor) -> Scalar {
    var self_293: MultiVector;
    var other_251: Motor;

    self_293 = self_292;
    other_251 = other_250;
    let _e4: MultiVector = self_293;
    let _e7: Motor = other_251;
    let _e11: MultiVector = self_293;
    let _e14: Motor = other_251;
    let _e19: MultiVector = self_293;
    let _e22: Motor = other_251;
    let _e27: MultiVector = self_293;
    let _e30: Motor = other_251;
    let _e35: MultiVector = self_293;
    let _e38: Motor = other_251;
    let _e43: MultiVector = self_293;
    let _e46: Motor = other_251;
    let _e51: MultiVector = self_293;
    let _e54: Motor = other_251;
    let _e59: MultiVector = self_293;
    let _e62: Motor = other_251;
    return Scalar(((((((((_e4.g0_.x * _e7.g0_.x) - (_e11.g0_.y * _e14.g0_.y)) - (_e19.g0_.z * _e22.g0_.z)) - (_e27.g0_.w * _e30.g0_.w)) + (_e35.g3_.x * _e38.g1_.x)) - (_e43.g3_.y * _e46.g1_.y)) - (_e51.g3_.z * _e54.g1_.z)) - (_e59.g3_.w * _e62.g1_.w)));
}

fn multi_vector_point_and_plane_into(self_294: MultiVector) -> PointAndPlane {
    var self_295: MultiVector;

    self_295 = self_294;
    let _e2: MultiVector = self_295;
    let _e5: MultiVector = self_295;
    let _e8: MultiVector = self_295;
    let _e11: MultiVector = self_295;
    let _e15: MultiVector = self_295;
    let _e18: MultiVector = self_295;
    let _e21: MultiVector = self_295;
    let _e24: MultiVector = self_295;
    return PointAndPlane(vec4<f32>(_e2.g2_.x, _e5.g1_.y, _e8.g1_.z, _e11.g1_.w), vec4<f32>(_e15.g1_.x, _e18.g2_.y, _e21.g2_.z, _e24.g2_.w));
}

fn multi_vector_point_and_plane_add(self_296: MultiVector, other_252: PointAndPlane) -> MultiVector {
    var self_297: MultiVector;
    var other_253: PointAndPlane;

    self_297 = self_296;
    other_253 = other_252;
    let _e4: MultiVector = self_297;
    let _e6: MultiVector = self_297;
    let _e8: PointAndPlane = other_253;
    let _e11: PointAndPlane = other_253;
    let _e14: PointAndPlane = other_253;
    let _e17: PointAndPlane = other_253;
    let _e22: MultiVector = self_297;
    let _e24: PointAndPlane = other_253;
    let _e27: PointAndPlane = other_253;
    let _e30: PointAndPlane = other_253;
    let _e33: PointAndPlane = other_253;
    let _e38: MultiVector = self_297;
    return MultiVector(_e4.g0_, (_e6.g1_ + vec4<f32>(_e8.g1_.x, _e11.g0_.y, _e14.g0_.z, _e17.g0_.w)), (_e22.g2_ + vec4<f32>(_e24.g0_.x, _e27.g1_.y, _e30.g1_.z, _e33.g1_.w)), _e38.g3_);
}

fn multi_vector_point_and_plane_sub(self_298: MultiVector, other_254: PointAndPlane) -> MultiVector {
    var self_299: MultiVector;
    var other_255: PointAndPlane;

    self_299 = self_298;
    other_255 = other_254;
    let _e4: MultiVector = self_299;
    let _e6: MultiVector = self_299;
    let _e8: PointAndPlane = other_255;
    let _e11: PointAndPlane = other_255;
    let _e14: PointAndPlane = other_255;
    let _e17: PointAndPlane = other_255;
    let _e22: MultiVector = self_299;
    let _e24: PointAndPlane = other_255;
    let _e27: PointAndPlane = other_255;
    let _e30: PointAndPlane = other_255;
    let _e33: PointAndPlane = other_255;
    let _e38: MultiVector = self_299;
    return MultiVector(_e4.g0_, (_e6.g1_ - vec4<f32>(_e8.g1_.x, _e11.g0_.y, _e14.g0_.z, _e17.g0_.w)), (_e22.g2_ - vec4<f32>(_e24.g0_.x, _e27.g1_.y, _e30.g1_.z, _e33.g1_.w)), _e38.g3_);
}

fn multi_vector_point_and_plane_geometric_product(self_300: MultiVector, other_256: PointAndPlane) -> MultiVector {
    var self_301: MultiVector;
    var other_257: PointAndPlane;

    self_301 = self_300;
    other_257 = other_256;
    let _e4: MultiVector = self_301;
    let _e8: PointAndPlane = other_257;
    let _e11: PointAndPlane = other_257;
    let _e14: PointAndPlane = other_257;
    let _e17: PointAndPlane = other_257;
    let _e31: MultiVector = self_301;
    let _e35: PointAndPlane = other_257;
    let _e38: PointAndPlane = other_257;
    let _e41: PointAndPlane = other_257;
    let _e44: PointAndPlane = other_257;
    let _e59: MultiVector = self_301;
    let _e63: PointAndPlane = other_257;
    let _e66: PointAndPlane = other_257;
    let _e69: PointAndPlane = other_257;
    let _e72: PointAndPlane = other_257;
    let _e87: MultiVector = self_301;
    let _e91: PointAndPlane = other_257;
    let _e94: PointAndPlane = other_257;
    let _e97: PointAndPlane = other_257;
    let _e100: PointAndPlane = other_257;
    let _e115: MultiVector = self_301;
    let _e119: PointAndPlane = other_257;
    let _e122: PointAndPlane = other_257;
    let _e125: PointAndPlane = other_257;
    let _e128: PointAndPlane = other_257;
    let _e141: MultiVector = self_301;
    let _e145: PointAndPlane = other_257;
    let _e148: PointAndPlane = other_257;
    let _e151: PointAndPlane = other_257;
    let _e154: PointAndPlane = other_257;
    let _e167: MultiVector = self_301;
    let _e171: PointAndPlane = other_257;
    let _e174: PointAndPlane = other_257;
    let _e177: PointAndPlane = other_257;
    let _e180: PointAndPlane = other_257;
    let _e193: MultiVector = self_301;
    let _e197: PointAndPlane = other_257;
    let _e200: PointAndPlane = other_257;
    let _e203: PointAndPlane = other_257;
    let _e206: PointAndPlane = other_257;
    let _e219: MultiVector = self_301;
    let _e223: PointAndPlane = other_257;
    let _e226: PointAndPlane = other_257;
    let _e229: PointAndPlane = other_257;
    let _e232: PointAndPlane = other_257;
    let _e237: MultiVector = self_301;
    let _e241: PointAndPlane = other_257;
    let _e244: PointAndPlane = other_257;
    let _e247: PointAndPlane = other_257;
    let _e250: PointAndPlane = other_257;
    let _e264: MultiVector = self_301;
    let _e268: PointAndPlane = other_257;
    let _e271: PointAndPlane = other_257;
    let _e274: PointAndPlane = other_257;
    let _e277: PointAndPlane = other_257;
    let _e291: MultiVector = self_301;
    let _e295: PointAndPlane = other_257;
    let _e298: PointAndPlane = other_257;
    let _e301: PointAndPlane = other_257;
    let _e304: PointAndPlane = other_257;
    let _e318: MultiVector = self_301;
    let _e322: PointAndPlane = other_257;
    let _e325: PointAndPlane = other_257;
    let _e328: PointAndPlane = other_257;
    let _e331: PointAndPlane = other_257;
    let _e337: MultiVector = self_301;
    let _e341: PointAndPlane = other_257;
    let _e344: PointAndPlane = other_257;
    let _e347: PointAndPlane = other_257;
    let _e350: PointAndPlane = other_257;
    let _e364: MultiVector = self_301;
    let _e368: PointAndPlane = other_257;
    let _e371: PointAndPlane = other_257;
    let _e374: PointAndPlane = other_257;
    let _e377: PointAndPlane = other_257;
    let _e391: MultiVector = self_301;
    let _e395: PointAndPlane = other_257;
    let _e398: PointAndPlane = other_257;
    let _e401: PointAndPlane = other_257;
    let _e404: PointAndPlane = other_257;
    let _e418: MultiVector = self_301;
    let _e422: PointAndPlane = other_257;
    let _e425: PointAndPlane = other_257;
    let _e428: PointAndPlane = other_257;
    let _e431: PointAndPlane = other_257;
    let _e436: MultiVector = self_301;
    let _e440: PointAndPlane = other_257;
    let _e443: PointAndPlane = other_257;
    let _e446: PointAndPlane = other_257;
    let _e449: PointAndPlane = other_257;
    let _e463: MultiVector = self_301;
    let _e467: PointAndPlane = other_257;
    let _e470: PointAndPlane = other_257;
    let _e473: PointAndPlane = other_257;
    let _e476: PointAndPlane = other_257;
    let _e490: MultiVector = self_301;
    let _e494: PointAndPlane = other_257;
    let _e497: PointAndPlane = other_257;
    let _e500: PointAndPlane = other_257;
    let _e503: PointAndPlane = other_257;
    let _e517: MultiVector = self_301;
    let _e521: PointAndPlane = other_257;
    let _e524: PointAndPlane = other_257;
    let _e527: PointAndPlane = other_257;
    let _e530: PointAndPlane = other_257;
    let _e536: MultiVector = self_301;
    let _e540: PointAndPlane = other_257;
    let _e543: PointAndPlane = other_257;
    let _e546: PointAndPlane = other_257;
    let _e549: PointAndPlane = other_257;
    let _e563: MultiVector = self_301;
    let _e567: PointAndPlane = other_257;
    let _e570: PointAndPlane = other_257;
    let _e573: PointAndPlane = other_257;
    let _e576: PointAndPlane = other_257;
    let _e590: MultiVector = self_301;
    let _e594: PointAndPlane = other_257;
    let _e597: PointAndPlane = other_257;
    let _e600: PointAndPlane = other_257;
    let _e603: PointAndPlane = other_257;
    let _e617: MultiVector = self_301;
    let _e621: PointAndPlane = other_257;
    let _e624: PointAndPlane = other_257;
    let _e627: PointAndPlane = other_257;
    let _e630: PointAndPlane = other_257;
    let _e635: MultiVector = self_301;
    let _e639: PointAndPlane = other_257;
    let _e642: PointAndPlane = other_257;
    let _e645: PointAndPlane = other_257;
    let _e648: PointAndPlane = other_257;
    let _e662: MultiVector = self_301;
    let _e666: PointAndPlane = other_257;
    let _e669: PointAndPlane = other_257;
    let _e672: PointAndPlane = other_257;
    let _e675: PointAndPlane = other_257;
    let _e689: MultiVector = self_301;
    let _e693: PointAndPlane = other_257;
    let _e696: PointAndPlane = other_257;
    let _e699: PointAndPlane = other_257;
    let _e702: PointAndPlane = other_257;
    let _e716: MultiVector = self_301;
    let _e720: PointAndPlane = other_257;
    let _e723: PointAndPlane = other_257;
    let _e726: PointAndPlane = other_257;
    let _e729: PointAndPlane = other_257;
    let _e735: MultiVector = self_301;
    let _e739: PointAndPlane = other_257;
    let _e742: PointAndPlane = other_257;
    let _e745: PointAndPlane = other_257;
    let _e748: PointAndPlane = other_257;
    let _e762: MultiVector = self_301;
    let _e766: PointAndPlane = other_257;
    let _e769: PointAndPlane = other_257;
    let _e772: PointAndPlane = other_257;
    let _e775: PointAndPlane = other_257;
    let _e789: MultiVector = self_301;
    let _e793: PointAndPlane = other_257;
    let _e796: PointAndPlane = other_257;
    let _e799: PointAndPlane = other_257;
    let _e802: PointAndPlane = other_257;
    return MultiVector((((((((((vec4<f32>(_e4.g1_.x) * vec4<f32>(_e8.g1_.x, _e11.g0_.y, _e14.g0_.z, _e17.g0_.w)) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))) + ((vec4<f32>(_e31.g1_.y) * vec4<f32>(_e35.g0_.y, _e38.g1_.x, _e41.g0_.w, _e44.g0_.z)) * vec4<f32>(-(1.0), -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e59.g1_.z) * vec4<f32>(_e63.g0_.z, _e66.g0_.w, _e69.g1_.x, _e72.g0_.y)) * vec4<f32>(-(1.0), -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e87.g1_.w) * vec4<f32>(_e91.g0_.w, _e94.g0_.z, _e97.g0_.y, _e100.g1_.x)) * vec4<f32>(-(1.0), 1.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e115.g2_.x) * vec4<f32>(_e119.g0_.x, _e122.g1_.y, _e125.g1_.z, _e128.g1_.w)) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0))) + ((vec4<f32>(_e141.g2_.y) * vec4<f32>(_e145.g1_.y, _e148.g0_.x, _e151.g1_.w, _e154.g1_.z)) * vec4<f32>(1.0, 1.0, -(1.0), 1.0))) + ((vec4<f32>(_e167.g2_.z) * vec4<f32>(_e171.g1_.z, _e174.g1_.w, _e177.g0_.x, _e180.g1_.y)) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e193.g2_.w) * vec4<f32>(_e197.g1_.w, _e200.g1_.z, _e203.g1_.y, _e206.g0_.x)) * vec4<f32>(1.0, -(1.0), 1.0, 1.0))), ((((((((vec4<f32>(_e219.g0_.x) * vec4<f32>(_e223.g1_.x, _e226.g0_.y, _e229.g0_.z, _e232.g0_.w)) + ((vec4<f32>(_e237.g0_.y) * vec4<f32>(_e241.g0_.y, _e244.g1_.x, _e247.g0_.w, _e250.g0_.z)) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e264.g0_.z) * vec4<f32>(_e268.g0_.z, _e271.g0_.w, _e274.g1_.x, _e277.g0_.y)) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e291.g0_.w) * vec4<f32>(_e295.g0_.w, _e298.g0_.z, _e301.g0_.y, _e304.g1_.x)) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))) - (vec4<f32>(_e318.g3_.x) * vec4<f32>(_e322.g0_.x, _e325.g1_.y, _e328.g1_.z, _e331.g1_.w))) + ((vec4<f32>(_e337.g3_.y) * vec4<f32>(_e341.g1_.y, _e344.g0_.x, _e347.g1_.w, _e350.g1_.z)) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e364.g3_.z) * vec4<f32>(_e368.g1_.z, _e371.g1_.w, _e374.g0_.x, _e377.g1_.y)) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e391.g3_.w) * vec4<f32>(_e395.g1_.w, _e398.g1_.z, _e401.g1_.y, _e404.g0_.x)) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))), ((((((((vec4<f32>(_e418.g0_.x) * vec4<f32>(_e422.g0_.x, _e425.g1_.y, _e428.g1_.z, _e431.g1_.w)) + ((vec4<f32>(_e436.g0_.y) * vec4<f32>(_e440.g1_.y, _e443.g0_.x, _e446.g1_.w, _e449.g1_.z)) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e463.g0_.z) * vec4<f32>(_e467.g1_.z, _e470.g1_.w, _e473.g0_.x, _e476.g1_.y)) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e490.g0_.w) * vec4<f32>(_e494.g1_.w, _e497.g1_.z, _e500.g1_.y, _e503.g0_.x)) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))) - (vec4<f32>(_e517.g3_.x) * vec4<f32>(_e521.g1_.x, _e524.g0_.y, _e527.g0_.z, _e530.g0_.w))) + ((vec4<f32>(_e536.g3_.y) * vec4<f32>(_e540.g0_.y, _e543.g1_.x, _e546.g0_.w, _e549.g0_.z)) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e563.g3_.z) * vec4<f32>(_e567.g0_.z, _e570.g0_.w, _e573.g1_.x, _e576.g0_.y)) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e590.g3_.w) * vec4<f32>(_e594.g0_.w, _e597.g0_.z, _e600.g0_.y, _e603.g1_.x)) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))), ((((((((vec4<f32>(_e617.g1_.x) * vec4<f32>(_e621.g0_.x, _e624.g1_.y, _e627.g1_.z, _e630.g1_.w)) + ((vec4<f32>(_e635.g1_.y) * vec4<f32>(_e639.g1_.y, _e642.g0_.x, _e645.g1_.w, _e648.g1_.z)) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))) + ((vec4<f32>(_e662.g1_.z) * vec4<f32>(_e666.g1_.z, _e669.g1_.w, _e672.g0_.x, _e675.g1_.y)) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e689.g1_.w) * vec4<f32>(_e693.g1_.w, _e696.g1_.z, _e699.g1_.y, _e702.g0_.x)) * vec4<f32>(-(1.0), -(1.0), 1.0, 1.0))) - (vec4<f32>(_e716.g2_.x) * vec4<f32>(_e720.g1_.x, _e723.g0_.y, _e726.g0_.z, _e729.g0_.w))) + ((vec4<f32>(_e735.g2_.y) * vec4<f32>(_e739.g0_.y, _e742.g1_.x, _e745.g0_.w, _e748.g0_.z)) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e762.g2_.z) * vec4<f32>(_e766.g0_.z, _e769.g0_.w, _e772.g1_.x, _e775.g0_.y)) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e789.g2_.w) * vec4<f32>(_e793.g0_.w, _e796.g0_.z, _e799.g0_.y, _e802.g1_.x)) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))));
}

fn multi_vector_point_and_plane_scalar_product(self_302: MultiVector, other_258: PointAndPlane) -> Scalar {
    var self_303: MultiVector;
    var other_259: PointAndPlane;

    self_303 = self_302;
    other_259 = other_258;
    let _e4: MultiVector = self_303;
    let _e7: PointAndPlane = other_259;
    let _e11: MultiVector = self_303;
    let _e14: PointAndPlane = other_259;
    let _e19: MultiVector = self_303;
    let _e22: PointAndPlane = other_259;
    let _e27: MultiVector = self_303;
    let _e30: PointAndPlane = other_259;
    let _e35: MultiVector = self_303;
    let _e38: PointAndPlane = other_259;
    let _e43: MultiVector = self_303;
    let _e46: PointAndPlane = other_259;
    let _e51: MultiVector = self_303;
    let _e54: PointAndPlane = other_259;
    let _e59: MultiVector = self_303;
    let _e62: PointAndPlane = other_259;
    return Scalar(((((((((_e4.g1_.x * _e7.g1_.x) - (_e11.g1_.y * _e14.g0_.y)) - (_e19.g1_.z * _e22.g0_.z)) - (_e27.g1_.w * _e30.g0_.w)) - (_e35.g2_.x * _e38.g0_.x)) + (_e43.g2_.y * _e46.g1_.y)) + (_e51.g2_.z * _e54.g1_.z)) + (_e59.g2_.w * _e62.g1_.w)));
}

fn multi_vector_squared_magnitude(self_304: MultiVector) -> Scalar {
    var self_305: MultiVector;

    self_305 = self_304;
    let _e4: MultiVector = self_305;
    let _e5: MultiVector = multi_vector_reversal(_e4);
    let _e6: MultiVector = self_305;
    let _e8: MultiVector = self_305;
    let _e9: MultiVector = multi_vector_reversal(_e8);
    let _e10: Scalar = multi_vector_multi_vector_scalar_product(_e6, _e9);
    return _e10;
}

fn multi_vector_magnitude(self_306: MultiVector) -> Scalar {
    var self_307: MultiVector;

    self_307 = self_306;
    let _e3: MultiVector = self_307;
    let _e4: Scalar = multi_vector_squared_magnitude(_e3);
    let _e7: MultiVector = self_307;
    let _e8: Scalar = multi_vector_squared_magnitude(_e7);
    return Scalar(sqrt(_e8.g0_));
}

fn multi_vector_scale(self_308: MultiVector, other_260: f32) -> MultiVector {
    var self_309: MultiVector;
    var other_261: f32;

    self_309 = self_308;
    other_261 = other_260;
    let _e5: f32 = other_261;
    let _e7: MultiVector = self_309;
    let _e8: f32 = other_261;
    let _e10: MultiVector = multi_vector_scalar_geometric_product(_e7, Scalar(_e8));
    return _e10;
}

fn multi_vector_signum(self_310: MultiVector) -> MultiVector {
    var self_311: MultiVector;

    self_311 = self_310;
    let _e5: MultiVector = self_311;
    let _e6: Scalar = multi_vector_magnitude(_e5);
    let _e10: MultiVector = self_311;
    let _e13: MultiVector = self_311;
    let _e14: Scalar = multi_vector_magnitude(_e13);
    let _e18: MultiVector = multi_vector_scalar_geometric_product(_e10, Scalar((1.0 / _e14.g0_)));
    return _e18;
}

fn multi_vector_inverse(self_312: MultiVector) -> MultiVector {
    var self_313: MultiVector;

    self_313 = self_312;
    let _e3: MultiVector = self_313;
    let _e4: MultiVector = multi_vector_reversal(_e3);
    let _e7: MultiVector = self_313;
    let _e8: Scalar = multi_vector_squared_magnitude(_e7);
    let _e13: MultiVector = self_313;
    let _e14: MultiVector = multi_vector_reversal(_e13);
    let _e17: MultiVector = self_313;
    let _e18: Scalar = multi_vector_squared_magnitude(_e17);
    let _e22: MultiVector = multi_vector_scalar_geometric_product(_e14, Scalar((1.0 / _e18.g0_)));
    return _e22;
}

fn rotor_zero() -> Rotor {
    return Rotor(vec4<f32>(0.0));
}

fn rotor_one() -> Rotor {
    return Rotor(vec4<f32>(1.0, 0.0, 0.0, 0.0));
}

fn rotor_neg(self_314: Rotor) -> Rotor {
    var self_315: Rotor;

    self_315 = self_314;
    let _e2: Rotor = self_315;
    return Rotor((_e2.g0_ * vec4<f32>(-(1.0))));
}

fn rotor_automorphism(self_316: Rotor) -> Rotor {
    var self_317: Rotor;

    self_317 = self_316;
    let _e2: Rotor = self_317;
    return Rotor(_e2.g0_);
}

fn rotor_reversal(self_318: Rotor) -> Rotor {
    var self_319: Rotor;

    self_319 = self_318;
    let _e2: Rotor = self_319;
    return Rotor((_e2.g0_ * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))));
}

fn rotor_conjugation(self_320: Rotor) -> Rotor {
    var self_321: Rotor;

    self_321 = self_320;
    let _e2: Rotor = self_321;
    return Rotor((_e2.g0_ * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))));
}

fn rotor_scalar_into(self_322: Rotor) -> Scalar {
    var self_323: Rotor;

    self_323 = self_322;
    let _e2: Rotor = self_323;
    return Scalar(_e2.g0_.x);
}

fn rotor_scalar_add(self_324: Rotor, other_262: Scalar) -> Rotor {
    var self_325: Rotor;
    var other_263: Scalar;

    self_325 = self_324;
    other_263 = other_262;
    let _e4: Rotor = self_325;
    let _e6: Scalar = other_263;
    return Rotor((_e4.g0_ + (vec4<f32>(_e6.g0_) * vec4<f32>(1.0, 0.0, 0.0, 0.0))));
}

fn rotor_scalar_sub(self_326: Rotor, other_264: Scalar) -> Rotor {
    var self_327: Rotor;
    var other_265: Scalar;

    self_327 = self_326;
    other_265 = other_264;
    let _e4: Rotor = self_327;
    let _e6: Scalar = other_265;
    return Rotor((_e4.g0_ - (vec4<f32>(_e6.g0_) * vec4<f32>(1.0, 0.0, 0.0, 0.0))));
}

fn rotor_scalar_geometric_product(self_328: Rotor, other_266: Scalar) -> Rotor {
    var self_329: Rotor;
    var other_267: Scalar;

    self_329 = self_328;
    other_267 = other_266;
    let _e4: Rotor = self_329;
    let _e6: Scalar = other_267;
    return Rotor((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn rotor_scalar_outer_product(self_330: Rotor, other_268: Scalar) -> Rotor {
    var self_331: Rotor;
    var other_269: Scalar;

    self_331 = self_330;
    other_269 = other_268;
    let _e4: Rotor = self_331;
    let _e6: Scalar = other_269;
    return Rotor((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn rotor_scalar_inner_product(self_332: Rotor, other_270: Scalar) -> Rotor {
    var self_333: Rotor;
    var other_271: Scalar;

    self_333 = self_332;
    other_271 = other_270;
    let _e4: Rotor = self_333;
    let _e6: Scalar = other_271;
    return Rotor((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn rotor_scalar_left_contraction(self_334: Rotor, other_272: Scalar) -> Scalar {
    var self_335: Rotor;
    var other_273: Scalar;

    self_335 = self_334;
    other_273 = other_272;
    let _e4: Rotor = self_335;
    let _e7: Scalar = other_273;
    return Scalar((_e4.g0_.x * _e7.g0_));
}

fn rotor_scalar_right_contraction(self_336: Rotor, other_274: Scalar) -> Rotor {
    var self_337: Rotor;
    var other_275: Scalar;

    self_337 = self_336;
    other_275 = other_274;
    let _e4: Rotor = self_337;
    let _e6: Scalar = other_275;
    return Rotor((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn rotor_scalar_scalar_product(self_338: Rotor, other_276: Scalar) -> Scalar {
    var self_339: Rotor;
    var other_277: Scalar;

    self_339 = self_338;
    other_277 = other_276;
    let _e4: Rotor = self_339;
    let _e7: Scalar = other_277;
    return Scalar((_e4.g0_.x * _e7.g0_));
}

fn rotor_multi_vector_add(self_340: Rotor, other_278: MultiVector) -> MultiVector {
    var self_341: Rotor;
    var other_279: MultiVector;

    self_341 = self_340;
    other_279 = other_278;
    let _e4: Rotor = self_341;
    let _e6: MultiVector = other_279;
    let _e9: MultiVector = other_279;
    let _e11: MultiVector = other_279;
    let _e13: MultiVector = other_279;
    return MultiVector((_e4.g0_ + _e6.g0_), _e9.g1_, _e11.g2_, _e13.g3_);
}

fn rotor_multi_vector_sub(self_342: Rotor, other_280: MultiVector) -> MultiVector {
    var self_343: Rotor;
    var other_281: MultiVector;

    self_343 = self_342;
    other_281 = other_280;
    let _e4: Rotor = self_343;
    let _e6: MultiVector = other_281;
    let _e11: MultiVector = other_281;
    let _e16: MultiVector = other_281;
    let _e21: MultiVector = other_281;
    return MultiVector((_e4.g0_ - _e6.g0_), (vec4<f32>(0.0) - _e11.g1_), (vec4<f32>(0.0) - _e16.g2_), (vec4<f32>(0.0) - _e21.g3_));
}

fn rotor_multi_vector_geometric_product(self_344: Rotor, other_282: MultiVector) -> MultiVector {
    var self_345: Rotor;
    var other_283: MultiVector;

    self_345 = self_344;
    other_283 = other_282;
    let _e4: Rotor = self_345;
    let _e8: MultiVector = other_283;
    let _e11: Rotor = self_345;
    let _e15: MultiVector = other_283;
    let _e28: Rotor = self_345;
    let _e32: MultiVector = other_283;
    let _e45: Rotor = self_345;
    let _e49: MultiVector = other_283;
    let _e62: Rotor = self_345;
    let _e66: MultiVector = other_283;
    let _e69: Rotor = self_345;
    let _e73: MultiVector = other_283;
    let _e86: Rotor = self_345;
    let _e90: MultiVector = other_283;
    let _e103: Rotor = self_345;
    let _e107: MultiVector = other_283;
    let _e120: Rotor = self_345;
    let _e124: MultiVector = other_283;
    let _e127: Rotor = self_345;
    let _e131: MultiVector = other_283;
    let _e144: Rotor = self_345;
    let _e148: MultiVector = other_283;
    let _e161: Rotor = self_345;
    let _e165: MultiVector = other_283;
    let _e178: Rotor = self_345;
    let _e182: MultiVector = other_283;
    let _e185: Rotor = self_345;
    let _e189: MultiVector = other_283;
    let _e202: Rotor = self_345;
    let _e206: MultiVector = other_283;
    let _e219: Rotor = self_345;
    let _e223: MultiVector = other_283;
    return MultiVector(((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.y) * _e15.g0_.yxwz) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e28.g0_.z) * _e32.g0_.zwxy) * vec4<f32>(-(1.0), -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e45.g0_.w) * _e49.g0_.wzyx) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))), ((((vec4<f32>(_e62.g0_.x) * _e66.g1_) + ((vec4<f32>(_e69.g0_.y) * _e73.g1_.yxwz) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e86.g0_.z) * _e90.g1_.zwxy) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e103.g0_.w) * _e107.g1_.wzyx) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))), ((((vec4<f32>(_e120.g0_.x) * _e124.g2_) + ((vec4<f32>(_e127.g0_.y) * _e131.g2_.yxwz) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e144.g0_.z) * _e148.g2_.zwxy) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e161.g0_.w) * _e165.g2_.wzyx) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))), ((((vec4<f32>(_e178.g0_.x) * _e182.g3_) + ((vec4<f32>(_e185.g0_.y) * _e189.g3_.yxwz) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e202.g0_.z) * _e206.g3_.zwxy) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e219.g0_.w) * _e223.g3_.wzyx) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))));
}

fn rotor_multi_vector_outer_product(self_346: Rotor, other_284: MultiVector) -> MultiVector {
    var self_347: Rotor;
    var other_285: MultiVector;

    self_347 = self_346;
    other_285 = other_284;
    let _e4: Rotor = self_347;
    let _e8: MultiVector = other_285;
    let _e11: Rotor = self_347;
    let _e13: MultiVector = other_285;
    let _e25: Rotor = self_347;
    let _e29: MultiVector = other_285;
    let _e32: Rotor = self_347;
    let _e34: MultiVector = other_285;
    let _e49: Rotor = self_347;
    let _e53: MultiVector = other_285;
    let _e56: Rotor = self_347;
    let _e60: MultiVector = other_285;
    let _e72: Rotor = self_347;
    let _e76: MultiVector = other_285;
    let _e88: Rotor = self_347;
    let _e91: MultiVector = other_285;
    let _e102: Rotor = self_347;
    let _e106: MultiVector = other_285;
    let _e109: Rotor = self_347;
    let _e113: MultiVector = other_285;
    let _e125: Rotor = self_347;
    let _e129: MultiVector = other_285;
    let _e141: Rotor = self_347;
    let _e144: MultiVector = other_285;
    return MultiVector(((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((_e11.g0_ * vec4<f32>(_e13.g0_.x)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))), ((vec4<f32>(_e25.g0_.x) * _e29.g1_) + ((_e32.g0_ * vec4<f32>(_e34.g1_.x)) * vec4<f32>(0.0, -(1.0), -(1.0), -(1.0)))), ((((vec4<f32>(_e49.g0_.x) * _e53.g2_) + ((vec4<f32>(_e56.g0_.z) * vec4<f32>(_e60.g2_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e72.g0_.w) * vec4<f32>(_e76.g2_.w)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((_e88.g0_.yxxx * _e91.g2_.yxxx) * vec4<f32>(1.0, 0.0, 0.0, 0.0))), ((((vec4<f32>(_e102.g0_.x) * _e106.g3_) + ((vec4<f32>(_e109.g0_.z) * vec4<f32>(_e113.g3_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e125.g0_.w) * vec4<f32>(_e129.g3_.w)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((_e141.g0_.yxxx * _e144.g3_.yxxx) * vec4<f32>(1.0, 0.0, 0.0, 0.0))));
}

fn rotor_multi_vector_inner_product(self_348: Rotor, other_286: MultiVector) -> MultiVector {
    var self_349: Rotor;
    var other_287: MultiVector;

    self_349 = self_348;
    other_287 = other_286;
    let _e4: Rotor = self_349;
    let _e8: MultiVector = other_287;
    let _e11: Rotor = self_349;
    let _e15: MultiVector = other_287;
    let _e27: Rotor = self_349;
    let _e31: MultiVector = other_287;
    let _e43: Rotor = self_349;
    let _e46: MultiVector = other_287;
    let _e58: Rotor = self_349;
    let _e62: MultiVector = other_287;
    let _e65: Rotor = self_349;
    let _e69: MultiVector = other_287;
    let _e81: Rotor = self_349;
    let _e85: MultiVector = other_287;
    let _e97: Rotor = self_349;
    let _e100: MultiVector = other_287;
    let _e111: Rotor = self_349;
    let _e115: MultiVector = other_287;
    let _e118: Rotor = self_349;
    let _e122: MultiVector = other_287;
    let _e135: Rotor = self_349;
    let _e139: MultiVector = other_287;
    let _e152: Rotor = self_349;
    let _e155: MultiVector = other_287;
    let _e168: Rotor = self_349;
    let _e172: MultiVector = other_287;
    let _e175: Rotor = self_349;
    let _e177: MultiVector = other_287;
    return MultiVector(((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.z) * _e15.g0_.zzxz) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0))) + ((vec4<f32>(_e27.g0_.w) * _e31.g0_.wwwx) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))) + ((_e43.g0_.yyxx * _e46.g0_.yxxx) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))), ((((vec4<f32>(_e58.g0_.x) * _e62.g1_) + ((vec4<f32>(_e65.g0_.z) * vec4<f32>(_e69.g1_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e81.g0_.w) * vec4<f32>(_e85.g1_.w)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((_e97.g0_.yxxx * _e100.g1_.yxxx) * vec4<f32>(1.0, 0.0, 0.0, 0.0))), ((((vec4<f32>(_e111.g0_.x) * _e115.g2_) + ((vec4<f32>(_e118.g0_.z) * _e122.g2_.wwxy) * vec4<f32>(0.0, -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e135.g0_.w) * _e139.g2_.zzyx) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))) + ((_e152.g0_.xyyy * _e155.g2_.xxwz) * vec4<f32>(0.0, -(1.0), 1.0, -(1.0)))), ((vec4<f32>(_e168.g0_.x) * _e172.g3_) + ((_e175.g0_ * vec4<f32>(_e177.g3_.x)) * vec4<f32>(0.0, -(1.0), -(1.0), -(1.0)))));
}

fn rotor_multi_vector_left_contraction(self_350: Rotor, other_288: MultiVector) -> MultiVector {
    var self_351: Rotor;
    var other_289: MultiVector;

    self_351 = self_350;
    other_289 = other_288;
    let _e4: Rotor = self_351;
    let _e8: MultiVector = other_289;
    let _e11: Rotor = self_351;
    let _e15: MultiVector = other_289;
    let _e28: Rotor = self_351;
    let _e32: MultiVector = other_289;
    let _e45: Rotor = self_351;
    let _e48: MultiVector = other_289;
    let _e60: Rotor = self_351;
    let _e64: MultiVector = other_289;
    let _e67: Rotor = self_351;
    let _e71: MultiVector = other_289;
    let _e83: Rotor = self_351;
    let _e87: MultiVector = other_289;
    let _e99: Rotor = self_351;
    let _e102: MultiVector = other_289;
    let _e113: Rotor = self_351;
    let _e117: MultiVector = other_289;
    let _e120: Rotor = self_351;
    let _e122: MultiVector = other_289;
    let _e137: Rotor = self_351;
    let _e141: MultiVector = other_289;
    let _e144: Rotor = self_351;
    let _e146: MultiVector = other_289;
    return MultiVector(((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.z) * vec4<f32>(_e15.g0_.z)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((vec4<f32>(_e28.g0_.w) * vec4<f32>(_e32.g0_.w)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((_e45.g0_.yxxx * _e48.g0_.yxxx) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))), ((((vec4<f32>(_e60.g0_.x) * _e64.g1_) + ((vec4<f32>(_e67.g0_.z) * vec4<f32>(_e71.g1_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e83.g0_.w) * vec4<f32>(_e87.g1_.w)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((_e99.g0_.yxxx * _e102.g1_.yxxx) * vec4<f32>(1.0, 0.0, 0.0, 0.0))), ((vec4<f32>(_e113.g0_.x) * _e117.g2_) + ((_e120.g0_ * vec4<f32>(_e122.g2_.x)) * vec4<f32>(0.0, -(1.0), -(1.0), -(1.0)))), ((vec4<f32>(_e137.g0_.x) * _e141.g3_) + ((_e144.g0_ * vec4<f32>(_e146.g3_.x)) * vec4<f32>(0.0, -(1.0), -(1.0), -(1.0)))));
}

fn rotor_multi_vector_scalar_product(self_352: Rotor, other_290: MultiVector) -> Scalar {
    var self_353: Rotor;
    var other_291: MultiVector;

    self_353 = self_352;
    other_291 = other_290;
    let _e4: Rotor = self_353;
    let _e7: MultiVector = other_291;
    let _e11: Rotor = self_353;
    let _e14: MultiVector = other_291;
    let _e19: Rotor = self_353;
    let _e22: MultiVector = other_291;
    let _e27: Rotor = self_353;
    let _e30: MultiVector = other_291;
    return Scalar(((((_e4.g0_.x * _e7.g0_.x) - (_e11.g0_.y * _e14.g0_.y)) - (_e19.g0_.z * _e22.g0_.z)) - (_e27.g0_.w * _e30.g0_.w)));
}

fn rotor_rotor_add(self_354: Rotor, other_292: Rotor) -> Rotor {
    var self_355: Rotor;
    var other_293: Rotor;

    self_355 = self_354;
    other_293 = other_292;
    let _e4: Rotor = self_355;
    let _e6: Rotor = other_293;
    return Rotor((_e4.g0_ + _e6.g0_));
}

fn rotor_rotor_sub(self_356: Rotor, other_294: Rotor) -> Rotor {
    var self_357: Rotor;
    var other_295: Rotor;

    self_357 = self_356;
    other_295 = other_294;
    let _e4: Rotor = self_357;
    let _e6: Rotor = other_295;
    return Rotor((_e4.g0_ - _e6.g0_));
}

fn rotor_rotor_mul(self_358: Rotor, other_296: Rotor) -> Rotor {
    var self_359: Rotor;
    var other_297: Rotor;

    self_359 = self_358;
    other_297 = other_296;
    let _e4: Rotor = self_359;
    let _e6: Rotor = other_297;
    return Rotor((_e4.g0_ * _e6.g0_));
}

fn rotor_rotor_div(self_360: Rotor, other_298: Rotor) -> Rotor {
    var self_361: Rotor;
    var other_299: Rotor;

    self_361 = self_360;
    other_299 = other_298;
    let _e4: Rotor = self_361;
    let _e7: Rotor = self_361;
    let _e10: Rotor = self_361;
    let _e13: Rotor = self_361;
    let _e23: Rotor = other_299;
    let _e26: Rotor = other_299;
    let _e29: Rotor = other_299;
    let _e32: Rotor = other_299;
    return Rotor((((vec4<f32>(_e4.g0_.x, _e7.g0_.y, _e10.g0_.z, _e13.g0_.w) * vec4<f32>(1.0, 1.0, 1.0, 1.0)) / vec4<f32>(_e23.g0_.x, _e26.g0_.y, _e29.g0_.z, _e32.g0_.w)) * vec4<f32>(1.0, 1.0, 1.0, 1.0)));
}

fn rotor_rotor_geometric_product(self_362: Rotor, other_300: Rotor) -> Rotor {
    var self_363: Rotor;
    var other_301: Rotor;

    self_363 = self_362;
    other_301 = other_300;
    let _e4: Rotor = self_363;
    let _e8: Rotor = other_301;
    let _e11: Rotor = self_363;
    let _e15: Rotor = other_301;
    let _e28: Rotor = self_363;
    let _e32: Rotor = other_301;
    let _e45: Rotor = self_363;
    let _e49: Rotor = other_301;
    return Rotor(((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.y) * _e15.g0_.yxwz) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e28.g0_.z) * _e32.g0_.zwxy) * vec4<f32>(-(1.0), -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e45.g0_.w) * _e49.g0_.wzyx) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))));
}

fn rotor_rotor_outer_product(self_364: Rotor, other_302: Rotor) -> Rotor {
    var self_365: Rotor;
    var other_303: Rotor;

    self_365 = self_364;
    other_303 = other_302;
    let _e4: Rotor = self_365;
    let _e8: Rotor = other_303;
    let _e11: Rotor = self_365;
    let _e13: Rotor = other_303;
    return Rotor(((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((_e11.g0_ * vec4<f32>(_e13.g0_.x)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn rotor_rotor_inner_product(self_366: Rotor, other_304: Rotor) -> Rotor {
    var self_367: Rotor;
    var other_305: Rotor;

    self_367 = self_366;
    other_305 = other_304;
    let _e4: Rotor = self_367;
    let _e8: Rotor = other_305;
    let _e11: Rotor = self_367;
    let _e15: Rotor = other_305;
    let _e27: Rotor = self_367;
    let _e31: Rotor = other_305;
    let _e43: Rotor = self_367;
    let _e46: Rotor = other_305;
    return Rotor(((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.z) * _e15.g0_.zzxz) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0))) + ((vec4<f32>(_e27.g0_.w) * _e31.g0_.wwwx) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))) + ((_e43.g0_.yyxx * _e46.g0_.yxxx) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))));
}

fn rotor_rotor_left_contraction(self_368: Rotor, other_306: Rotor) -> Rotor {
    var self_369: Rotor;
    var other_307: Rotor;

    self_369 = self_368;
    other_307 = other_306;
    let _e4: Rotor = self_369;
    let _e8: Rotor = other_307;
    let _e11: Rotor = self_369;
    let _e15: Rotor = other_307;
    let _e28: Rotor = self_369;
    let _e32: Rotor = other_307;
    let _e45: Rotor = self_369;
    let _e48: Rotor = other_307;
    return Rotor(((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.z) * vec4<f32>(_e15.g0_.z)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((vec4<f32>(_e28.g0_.w) * vec4<f32>(_e32.g0_.w)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((_e45.g0_.yxxx * _e48.g0_.yxxx) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))));
}

fn rotor_rotor_right_contraction(self_370: Rotor, other_308: Rotor) -> Rotor {
    var self_371: Rotor;
    var other_309: Rotor;

    self_371 = self_370;
    other_309 = other_308;
    let _e4: Rotor = self_371;
    let _e8: Rotor = other_309;
    let _e19: Rotor = self_371;
    let _e23: Rotor = other_309;
    let _e35: Rotor = self_371;
    let _e39: Rotor = other_309;
    let _e51: Rotor = self_371;
    let _e55: Rotor = other_309;
    return Rotor((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0)) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.zzxz) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0))) + ((vec4<f32>(_e35.g0_.w) * _e39.g0_.wwwx) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))) + ((vec4<f32>(_e51.g0_.x) * vec4<f32>(_e55.g0_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))));
}

fn rotor_rotor_scalar_product(self_372: Rotor, other_310: Rotor) -> Scalar {
    var self_373: Rotor;
    var other_311: Rotor;

    self_373 = self_372;
    other_311 = other_310;
    let _e4: Rotor = self_373;
    let _e7: Rotor = other_311;
    let _e11: Rotor = self_373;
    let _e14: Rotor = other_311;
    let _e19: Rotor = self_373;
    let _e22: Rotor = other_311;
    let _e27: Rotor = self_373;
    let _e30: Rotor = other_311;
    return Scalar(((((_e4.g0_.x * _e7.g0_.x) - (_e11.g0_.y * _e14.g0_.y)) - (_e19.g0_.z * _e22.g0_.z)) - (_e27.g0_.w * _e30.g0_.w)));
}

fn rotor_point_geometric_product(self_374: Rotor, other_312: Point) -> PointAndPlane {
    var self_375: Rotor;
    var other_313: Point;

    self_375 = self_374;
    other_313 = other_312;
    let _e4: Rotor = self_375;
    let _e8: Point = other_313;
    let _e11: Rotor = self_375;
    let _e15: Point = other_313;
    let _e27: Rotor = self_375;
    let _e31: Point = other_313;
    let _e43: Rotor = self_375;
    let _e46: Point = other_313;
    let _e58: Rotor = self_375;
    let _e62: Point = other_313;
    let _e73: Rotor = self_375;
    let _e77: Point = other_313;
    let _e89: Rotor = self_375;
    let _e92: Point = other_313;
    return PointAndPlane(((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.z) * _e15.g0_.wwwy) * vec4<f32>(0.0, -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e27.g0_.w) * _e31.g0_.zzyz) * vec4<f32>(0.0, 1.0, -(1.0), 0.0))) + ((_e43.g0_.xxyy * _e46.g0_.xxwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))), ((((vec4<f32>(_e58.g0_.z) * _e62.g0_.zzxz) * vec4<f32>(1.0, 0.0, -(1.0), 0.0)) + ((vec4<f32>(_e73.g0_.w) * _e77.g0_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))) + ((_e89.g0_.yyxx * _e92.g0_.yxxx) * vec4<f32>(1.0, -(1.0), 0.0, 0.0))));
}

fn rotor_point_outer_product(self_376: Rotor, other_314: Point) -> Point {
    var self_377: Rotor;
    var other_315: Point;

    self_377 = self_376;
    other_315 = other_314;
    let _e4: Rotor = self_377;
    let _e8: Point = other_315;
    return Point((vec4<f32>(_e4.g0_.x) * _e8.g0_));
}

fn rotor_point_inner_product(self_378: Rotor, other_316: Point) -> PointAndPlane {
    var self_379: Rotor;
    var other_317: Point;

    self_379 = self_378;
    other_317 = other_316;
    let _e4: Rotor = self_379;
    let _e8: Point = other_317;
    let _e11: Rotor = self_379;
    let _e15: Point = other_317;
    let _e26: Rotor = self_379;
    let _e30: Point = other_317;
    let _e42: Rotor = self_379;
    let _e45: Point = other_317;
    return PointAndPlane((vec4<f32>(_e4.g0_.x) * _e8.g0_), ((((vec4<f32>(_e11.g0_.z) * _e15.g0_.zzxz) * vec4<f32>(1.0, 0.0, -(1.0), 0.0)) + ((vec4<f32>(_e26.g0_.w) * _e30.g0_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))) + ((_e42.g0_.yyxx * _e45.g0_.yxxx) * vec4<f32>(1.0, -(1.0), 0.0, 0.0))));
}

fn rotor_point_left_contraction(self_380: Rotor, other_318: Point) -> PointAndPlane {
    var self_381: Rotor;
    var other_319: Point;

    self_381 = self_380;
    other_319 = other_318;
    let _e4: Rotor = self_381;
    let _e8: Point = other_319;
    let _e11: Rotor = self_381;
    let _e15: Point = other_319;
    let _e26: Rotor = self_381;
    let _e30: Point = other_319;
    let _e42: Rotor = self_381;
    let _e45: Point = other_319;
    return PointAndPlane((vec4<f32>(_e4.g0_.x) * _e8.g0_), ((((vec4<f32>(_e11.g0_.z) * _e15.g0_.zzxz) * vec4<f32>(1.0, 0.0, -(1.0), 0.0)) + ((vec4<f32>(_e26.g0_.w) * _e30.g0_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))) + ((_e42.g0_.yyxx * _e45.g0_.yxxx) * vec4<f32>(1.0, -(1.0), 0.0, 0.0))));
}

fn rotor_ideal_point_regressive_product(self_382: Rotor, other_320: IdealPoint) -> Scalar {
    var self_383: Rotor;
    var other_321: IdealPoint;

    self_383 = self_382;
    other_321 = other_320;
    let _e4: Rotor = self_383;
    let _e7: IdealPoint = other_321;
    let _e11: Rotor = self_383;
    let _e14: IdealPoint = other_321;
    let _e19: Rotor = self_383;
    let _e22: IdealPoint = other_321;
    return Scalar((((_e4.g0_.y * _e7.g0_.x) + (_e11.g0_.z * _e14.g0_.y)) + (_e19.g0_.w * _e22.g0_.z)));
}

fn rotor_ideal_point_inner_product(self_384: Rotor, other_322: IdealPoint) -> IdealPoint {
    var self_385: Rotor;
    var other_323: IdealPoint;

    self_385 = self_384;
    other_323 = other_322;
    let _e4: Rotor = self_385;
    let _e8: IdealPoint = other_323;
    return IdealPoint((vec3<f32>(_e4.g0_.x) * _e8.g0_));
}

fn rotor_ideal_point_left_contraction(self_386: Rotor, other_324: IdealPoint) -> IdealPoint {
    var self_387: Rotor;
    var other_325: IdealPoint;

    self_387 = self_386;
    other_325 = other_324;
    let _e4: Rotor = self_387;
    let _e8: IdealPoint = other_325;
    return IdealPoint((vec3<f32>(_e4.g0_.x) * _e8.g0_));
}

fn rotor_plane_geometric_product(self_388: Rotor, other_326: Plane) -> PointAndPlane {
    var self_389: Rotor;
    var other_327: Plane;

    self_389 = self_388;
    other_327 = other_326;
    let _e4: Rotor = self_389;
    let _e8: Plane = other_327;
    let _e19: Rotor = self_389;
    let _e23: Plane = other_327;
    let _e35: Rotor = self_389;
    let _e38: Plane = other_327;
    let _e50: Rotor = self_389;
    let _e54: Plane = other_327;
    let _e57: Rotor = self_389;
    let _e61: Plane = other_327;
    let _e73: Rotor = self_389;
    let _e77: Plane = other_327;
    let _e89: Rotor = self_389;
    let _e92: Plane = other_327;
    return PointAndPlane(((((vec4<f32>(_e4.g0_.z) * _e8.g0_.zzxz) * vec4<f32>(1.0, 0.0, -(1.0), 0.0)) + ((vec4<f32>(_e19.g0_.w) * _e23.g0_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))) + ((_e35.g0_.yyxx * _e38.g0_.yxxx) * vec4<f32>(1.0, -(1.0), 0.0, 0.0))), ((((vec4<f32>(_e50.g0_.x) * _e54.g0_) + ((vec4<f32>(_e57.g0_.z) * _e61.g0_.wwwy) * vec4<f32>(0.0, -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e73.g0_.w) * _e77.g0_.zzyz) * vec4<f32>(0.0, 1.0, -(1.0), 0.0))) + ((_e89.g0_.xxyy * _e92.g0_.xxwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))));
}

fn rotor_plane_outer_product(self_390: Rotor, other_328: Plane) -> PointAndPlane {
    var self_391: Rotor;
    var other_329: Plane;

    self_391 = self_390;
    other_329 = other_328;
    let _e4: Rotor = self_391;
    let _e8: Plane = other_329;
    let _e19: Rotor = self_391;
    let _e23: Plane = other_329;
    let _e35: Rotor = self_391;
    let _e38: Plane = other_329;
    let _e50: Rotor = self_391;
    let _e54: Plane = other_329;
    return PointAndPlane(((((vec4<f32>(_e4.g0_.z) * _e8.g0_.zzxz) * vec4<f32>(1.0, 0.0, -(1.0), 0.0)) + ((vec4<f32>(_e19.g0_.w) * _e23.g0_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))) + ((_e35.g0_.yyxx * _e38.g0_.yxxx) * vec4<f32>(1.0, -(1.0), 0.0, 0.0))), (vec4<f32>(_e50.g0_.x) * _e54.g0_));
}

fn rotor_plane_inner_product(self_392: Rotor, other_330: Plane) -> Plane {
    var self_393: Rotor;
    var other_331: Plane;

    self_393 = self_392;
    other_331 = other_330;
    let _e4: Rotor = self_393;
    let _e8: Plane = other_331;
    let _e11: Rotor = self_393;
    let _e15: Plane = other_331;
    let _e27: Rotor = self_393;
    let _e31: Plane = other_331;
    let _e43: Rotor = self_393;
    let _e46: Plane = other_331;
    return Plane(((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.z) * _e15.g0_.wwwy) * vec4<f32>(0.0, -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e27.g0_.w) * _e31.g0_.zzyz) * vec4<f32>(0.0, 1.0, -(1.0), 0.0))) + ((_e43.g0_.xxyy * _e46.g0_.xxwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))));
}

fn rotor_plane_left_contraction(self_394: Rotor, other_332: Plane) -> Plane {
    var self_395: Rotor;
    var other_333: Plane;

    self_395 = self_394;
    other_333 = other_332;
    let _e4: Rotor = self_395;
    let _e8: Plane = other_333;
    return Plane((vec4<f32>(_e4.g0_.x) * _e8.g0_));
}

fn rotor_line_geometric_product(self_396: Rotor, other_334: Line) -> Motor {
    var self_397: Rotor;
    var other_335: Line;

    self_397 = self_396;
    other_335 = other_334;
    let _e4: Rotor = self_397;
    let _e8: Line = other_335;
    let _e11: Line = other_335;
    let _e14: Line = other_335;
    let _e17: Line = other_335;
    let _e30: Rotor = self_397;
    let _e34: Line = other_335;
    let _e37: Line = other_335;
    let _e40: Line = other_335;
    let _e43: Line = other_335;
    let _e57: Rotor = self_397;
    let _e61: Line = other_335;
    let _e64: Line = other_335;
    let _e67: Line = other_335;
    let _e70: Line = other_335;
    let _e84: Rotor = self_397;
    let _e88: Line = other_335;
    let _e91: Line = other_335;
    let _e94: Line = other_335;
    let _e97: Line = other_335;
    let _e109: Rotor = self_397;
    let _e113: Line = other_335;
    let _e116: Line = other_335;
    let _e119: Line = other_335;
    let _e122: Line = other_335;
    let _e134: Rotor = self_397;
    let _e138: Line = other_335;
    let _e141: Line = other_335;
    let _e144: Line = other_335;
    let _e147: Line = other_335;
    let _e160: Rotor = self_397;
    let _e164: Line = other_335;
    let _e167: Line = other_335;
    let _e170: Line = other_335;
    let _e173: Line = other_335;
    let _e186: Rotor = self_397;
    let _e190: Line = other_335;
    let _e193: Line = other_335;
    let _e196: Line = other_335;
    let _e199: Line = other_335;
    return Motor((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g1_.x, _e11.g1_.x, _e14.g1_.z, _e17.g1_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e30.g0_.z) * vec4<f32>(_e34.g1_.y, _e37.g1_.z, _e40.g1_.y, _e43.g1_.x)) * vec4<f32>(-(1.0), -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e57.g0_.w) * vec4<f32>(_e61.g1_.z, _e64.g1_.y, _e67.g1_.x, _e70.g1_.z)) * vec4<f32>(-(1.0), 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e84.g0_.x) * vec4<f32>(_e88.g1_.x, _e91.g1_.x, _e94.g1_.y, _e97.g1_.z)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))), (((((vec4<f32>(_e109.g0_.y) * vec4<f32>(_e113.g0_.x, _e116.g0_.x, _e119.g0_.z, _e122.g0_.y)) * vec4<f32>(1.0, 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e134.g0_.z) * vec4<f32>(_e138.g0_.y, _e141.g0_.z, _e144.g0_.y, _e147.g0_.x)) * vec4<f32>(1.0, -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e160.g0_.w) * vec4<f32>(_e164.g0_.z, _e167.g0_.y, _e170.g0_.x, _e173.g0_.z)) * vec4<f32>(1.0, 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e186.g0_.x) * vec4<f32>(_e190.g0_.x, _e193.g0_.x, _e196.g0_.y, _e199.g0_.z)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn rotor_line_regressive_product(self_398: Rotor, other_336: Line) -> Scalar {
    var self_399: Rotor;
    var other_337: Line;

    self_399 = self_398;
    other_337 = other_336;
    let _e4: Rotor = self_399;
    let _e7: Line = other_337;
    let _e11: Rotor = self_399;
    let _e14: Line = other_337;
    let _e19: Rotor = self_399;
    let _e22: Line = other_337;
    return Scalar((((_e4.g0_.y * _e7.g0_.x) + (_e11.g0_.z * _e14.g0_.y)) + (_e19.g0_.w * _e22.g0_.z)));
}

fn rotor_line_right_contraction(self_400: Rotor, other_338: Line) -> Scalar {
    var self_401: Rotor;
    var other_339: Line;

    self_401 = self_400;
    other_339 = other_338;
    let _e5: Rotor = self_401;
    let _e8: Line = other_339;
    let _e13: Rotor = self_401;
    let _e16: Line = other_339;
    let _e21: Rotor = self_401;
    let _e24: Line = other_339;
    return Scalar((((0.0 - (_e5.g0_.y * _e8.g1_.x)) - (_e13.g0_.z * _e16.g1_.y)) - (_e21.g0_.w * _e24.g1_.z)));
}

fn rotor_line_scalar_product(self_402: Rotor, other_340: Line) -> Scalar {
    var self_403: Rotor;
    var other_341: Line;

    self_403 = self_402;
    other_341 = other_340;
    let _e5: Rotor = self_403;
    let _e8: Line = other_341;
    let _e13: Rotor = self_403;
    let _e16: Line = other_341;
    let _e21: Rotor = self_403;
    let _e24: Line = other_341;
    return Scalar((((0.0 - (_e5.g0_.y * _e8.g1_.x)) - (_e13.g0_.z * _e16.g1_.y)) - (_e21.g0_.w * _e24.g1_.z)));
}

fn rotor_translator_geometric_product(self_404: Rotor, other_342: Translator) -> Motor {
    var self_405: Rotor;
    var other_343: Translator;

    self_405 = self_404;
    other_343 = other_342;
    let _e4: Rotor = self_405;
    let _e6: Translator = other_343;
    let _e11: Rotor = self_405;
    let _e15: Translator = other_343;
    let _e26: Rotor = self_405;
    let _e30: Translator = other_343;
    let _e42: Rotor = self_405;
    let _e46: Translator = other_343;
    let _e58: Rotor = self_405;
    let _e62: Translator = other_343;
    return Motor((_e4.g0_ * vec4<f32>(_e6.g0_.x)), (((((vec4<f32>(_e11.g0_.y) * _e15.g0_.yywz) * vec4<f32>(1.0, 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e26.g0_.z) * _e30.g0_.zwzy) * vec4<f32>(1.0, -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e42.g0_.w) * _e46.g0_.wzyw) * vec4<f32>(1.0, 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e58.g0_.x) * _e62.g0_) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn rotor_translator_regressive_product(self_406: Rotor, other_344: Translator) -> Scalar {
    var self_407: Rotor;
    var other_345: Translator;

    self_407 = self_406;
    other_345 = other_344;
    let _e4: Rotor = self_407;
    let _e7: Translator = other_345;
    let _e11: Rotor = self_407;
    let _e14: Translator = other_345;
    let _e19: Rotor = self_407;
    let _e22: Translator = other_345;
    return Scalar((((_e4.g0_.y * _e7.g0_.y) + (_e11.g0_.z * _e14.g0_.z)) + (_e19.g0_.w * _e22.g0_.w)));
}

fn rotor_translator_outer_product(self_408: Rotor, other_346: Translator) -> Motor {
    var self_409: Rotor;
    var other_347: Translator;

    self_409 = self_408;
    other_347 = other_346;
    let _e4: Rotor = self_409;
    let _e6: Translator = other_347;
    let _e11: Rotor = self_409;
    let _e15: Translator = other_347;
    let _e26: Rotor = self_409;
    let _e30: Translator = other_347;
    let _e42: Rotor = self_409;
    let _e45: Translator = other_347;
    return Motor((_e4.g0_ * vec4<f32>(_e6.g0_.x)), ((((vec4<f32>(_e11.g0_.z) * vec4<f32>(_e15.g0_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) + ((vec4<f32>(_e26.g0_.w) * vec4<f32>(_e30.g0_.w)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + (_e42.g0_.yxxx * _e45.g0_.yyzw)));
}

fn rotor_translator_left_contraction(self_410: Rotor, other_348: Translator) -> Translator {
    var self_411: Rotor;
    var other_349: Translator;

    self_411 = self_410;
    other_349 = other_348;
    let _e4: Rotor = self_411;
    let _e8: Translator = other_349;
    return Translator((vec4<f32>(_e4.g0_.x) * _e8.g0_));
}

fn rotor_translator_right_contraction(self_412: Rotor, other_350: Translator) -> Rotor {
    var self_413: Rotor;
    var other_351: Translator;

    self_413 = self_412;
    other_351 = other_350;
    let _e4: Rotor = self_413;
    let _e6: Translator = other_351;
    return Rotor((_e4.g0_ * vec4<f32>(_e6.g0_.x)));
}

fn rotor_translator_scalar_product(self_414: Rotor, other_352: Translator) -> Scalar {
    var self_415: Rotor;
    var other_353: Translator;

    self_415 = self_414;
    other_353 = other_352;
    let _e4: Rotor = self_415;
    let _e7: Translator = other_353;
    return Scalar((_e4.g0_.x * _e7.g0_.x));
}

fn rotor_motor_add(self_416: Rotor, other_354: Motor) -> Motor {
    var self_417: Rotor;
    var other_355: Motor;

    self_417 = self_416;
    other_355 = other_354;
    let _e4: Rotor = self_417;
    let _e6: Motor = other_355;
    let _e9: Motor = other_355;
    return Motor((_e4.g0_ + _e6.g0_), _e9.g1_);
}

fn rotor_motor_sub(self_418: Rotor, other_356: Motor) -> Motor {
    var self_419: Rotor;
    var other_357: Motor;

    self_419 = self_418;
    other_357 = other_356;
    let _e4: Rotor = self_419;
    let _e6: Motor = other_357;
    let _e11: Motor = other_357;
    return Motor((_e4.g0_ - _e6.g0_), (vec4<f32>(0.0) - _e11.g1_));
}

fn rotor_motor_geometric_product(self_420: Rotor, other_358: Motor) -> Motor {
    var self_421: Rotor;
    var other_359: Motor;

    self_421 = self_420;
    other_359 = other_358;
    let _e4: Rotor = self_421;
    let _e8: Motor = other_359;
    let _e11: Rotor = self_421;
    let _e15: Motor = other_359;
    let _e28: Rotor = self_421;
    let _e32: Motor = other_359;
    let _e45: Rotor = self_421;
    let _e49: Motor = other_359;
    let _e62: Rotor = self_421;
    let _e66: Motor = other_359;
    let _e69: Rotor = self_421;
    let _e73: Motor = other_359;
    let _e86: Rotor = self_421;
    let _e90: Motor = other_359;
    let _e103: Rotor = self_421;
    let _e107: Motor = other_359;
    return Motor(((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.y) * _e15.g0_.yxwz) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e28.g0_.z) * _e32.g0_.zwxy) * vec4<f32>(-(1.0), -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e45.g0_.w) * _e49.g0_.wzyx) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))), ((((vec4<f32>(_e62.g0_.x) * _e66.g1_) + ((vec4<f32>(_e69.g0_.y) * _e73.g1_.yxwz) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e86.g0_.z) * _e90.g1_.zwxy) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e103.g0_.w) * _e107.g1_.wzyx) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))));
}

fn rotor_motor_regressive_product(self_422: Rotor, other_360: Motor) -> Rotor {
    var self_423: Rotor;
    var other_361: Motor;

    self_423 = self_422;
    other_361 = other_360;
    let _e4: Rotor = self_423;
    let _e8: Motor = other_361;
    let _e18: Rotor = self_423;
    let _e22: Motor = other_361;
    let _e33: Rotor = self_423;
    let _e37: Motor = other_361;
    let _e48: Rotor = self_423;
    let _e52: Motor = other_361;
    return Rotor((((((vec4<f32>(_e4.g0_.y) * _e8.g1_.yxyy) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) + ((vec4<f32>(_e18.g0_.z) * _e22.g1_.zzxz) * vec4<f32>(1.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e33.g0_.w) * _e37.g1_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e48.g0_.x) * vec4<f32>(_e52.g1_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))));
}

fn rotor_motor_outer_product(self_424: Rotor, other_362: Motor) -> Motor {
    var self_425: Rotor;
    var other_363: Motor;

    self_425 = self_424;
    other_363 = other_362;
    let _e4: Rotor = self_425;
    let _e8: Motor = other_363;
    let _e11: Rotor = self_425;
    let _e13: Motor = other_363;
    let _e25: Rotor = self_425;
    let _e29: Motor = other_363;
    let _e32: Rotor = self_425;
    let _e36: Motor = other_363;
    let _e48: Rotor = self_425;
    let _e52: Motor = other_363;
    let _e64: Rotor = self_425;
    let _e67: Motor = other_363;
    return Motor(((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((_e11.g0_ * vec4<f32>(_e13.g0_.x)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))), ((((vec4<f32>(_e25.g0_.x) * _e29.g1_) + ((vec4<f32>(_e32.g0_.z) * vec4<f32>(_e36.g1_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e48.g0_.w) * vec4<f32>(_e52.g1_.w)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((_e64.g0_.yxxx * _e67.g1_.yxxx) * vec4<f32>(1.0, 0.0, 0.0, 0.0))));
}

fn rotor_motor_inner_product(self_426: Rotor, other_364: Motor) -> Motor {
    var self_427: Rotor;
    var other_365: Motor;

    self_427 = self_426;
    other_365 = other_364;
    let _e4: Rotor = self_427;
    let _e8: Motor = other_365;
    let _e11: Rotor = self_427;
    let _e15: Motor = other_365;
    let _e27: Rotor = self_427;
    let _e31: Motor = other_365;
    let _e43: Rotor = self_427;
    let _e46: Motor = other_365;
    let _e58: Rotor = self_427;
    let _e62: Motor = other_365;
    let _e65: Rotor = self_427;
    let _e67: Motor = other_365;
    return Motor(((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.z) * _e15.g0_.zzxz) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0))) + ((vec4<f32>(_e27.g0_.w) * _e31.g0_.wwwx) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))) + ((_e43.g0_.yyxx * _e46.g0_.yxxx) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))), ((vec4<f32>(_e58.g0_.x) * _e62.g1_) + ((_e65.g0_ * vec4<f32>(_e67.g1_.x)) * vec4<f32>(0.0, -(1.0), -(1.0), -(1.0)))));
}

fn rotor_motor_left_contraction(self_428: Rotor, other_366: Motor) -> Motor {
    var self_429: Rotor;
    var other_367: Motor;

    self_429 = self_428;
    other_367 = other_366;
    let _e4: Rotor = self_429;
    let _e8: Motor = other_367;
    let _e11: Rotor = self_429;
    let _e15: Motor = other_367;
    let _e28: Rotor = self_429;
    let _e32: Motor = other_367;
    let _e45: Rotor = self_429;
    let _e48: Motor = other_367;
    let _e60: Rotor = self_429;
    let _e64: Motor = other_367;
    let _e67: Rotor = self_429;
    let _e69: Motor = other_367;
    return Motor(((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.z) * vec4<f32>(_e15.g0_.z)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((vec4<f32>(_e28.g0_.w) * vec4<f32>(_e32.g0_.w)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((_e45.g0_.yxxx * _e48.g0_.yxxx) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))), ((vec4<f32>(_e60.g0_.x) * _e64.g1_) + ((_e67.g0_ * vec4<f32>(_e69.g1_.x)) * vec4<f32>(0.0, -(1.0), -(1.0), -(1.0)))));
}

fn rotor_motor_right_contraction(self_430: Rotor, other_368: Motor) -> Rotor {
    var self_431: Rotor;
    var other_369: Motor;

    self_431 = self_430;
    other_369 = other_368;
    let _e4: Rotor = self_431;
    let _e8: Motor = other_369;
    let _e19: Rotor = self_431;
    let _e23: Motor = other_369;
    let _e35: Rotor = self_431;
    let _e39: Motor = other_369;
    let _e51: Rotor = self_431;
    let _e55: Motor = other_369;
    return Rotor((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0)) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.zzxz) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0))) + ((vec4<f32>(_e35.g0_.w) * _e39.g0_.wwwx) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))) + ((vec4<f32>(_e51.g0_.x) * vec4<f32>(_e55.g0_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))));
}

fn rotor_motor_scalar_product(self_432: Rotor, other_370: Motor) -> Scalar {
    var self_433: Rotor;
    var other_371: Motor;

    self_433 = self_432;
    other_371 = other_370;
    let _e4: Rotor = self_433;
    let _e7: Motor = other_371;
    let _e11: Rotor = self_433;
    let _e14: Motor = other_371;
    let _e19: Rotor = self_433;
    let _e22: Motor = other_371;
    let _e27: Rotor = self_433;
    let _e30: Motor = other_371;
    return Scalar(((((_e4.g0_.x * _e7.g0_.x) - (_e11.g0_.y * _e14.g0_.y)) - (_e19.g0_.z * _e22.g0_.z)) - (_e27.g0_.w * _e30.g0_.w)));
}

fn rotor_point_and_plane_geometric_product(self_434: Rotor, other_372: PointAndPlane) -> PointAndPlane {
    var self_435: Rotor;
    var other_373: PointAndPlane;

    self_435 = self_434;
    other_373 = other_372;
    let _e4: Rotor = self_435;
    let _e8: PointAndPlane = other_373;
    let _e11: Rotor = self_435;
    let _e15: PointAndPlane = other_373;
    let _e18: PointAndPlane = other_373;
    let _e21: PointAndPlane = other_373;
    let _e24: PointAndPlane = other_373;
    let _e38: Rotor = self_435;
    let _e42: PointAndPlane = other_373;
    let _e45: PointAndPlane = other_373;
    let _e48: PointAndPlane = other_373;
    let _e51: PointAndPlane = other_373;
    let _e65: Rotor = self_435;
    let _e69: PointAndPlane = other_373;
    let _e72: PointAndPlane = other_373;
    let _e75: PointAndPlane = other_373;
    let _e78: PointAndPlane = other_373;
    let _e92: Rotor = self_435;
    let _e96: PointAndPlane = other_373;
    let _e99: Rotor = self_435;
    let _e103: PointAndPlane = other_373;
    let _e106: PointAndPlane = other_373;
    let _e109: PointAndPlane = other_373;
    let _e112: PointAndPlane = other_373;
    let _e126: Rotor = self_435;
    let _e130: PointAndPlane = other_373;
    let _e133: PointAndPlane = other_373;
    let _e136: PointAndPlane = other_373;
    let _e139: PointAndPlane = other_373;
    let _e153: Rotor = self_435;
    let _e157: PointAndPlane = other_373;
    let _e160: PointAndPlane = other_373;
    let _e163: PointAndPlane = other_373;
    let _e166: PointAndPlane = other_373;
    return PointAndPlane(((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.y) * vec4<f32>(_e15.g1_.y, _e18.g1_.x, _e21.g0_.w, _e24.g0_.z)) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e38.g0_.z) * vec4<f32>(_e42.g1_.z, _e45.g0_.w, _e48.g1_.x, _e51.g0_.y)) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e65.g0_.w) * vec4<f32>(_e69.g1_.w, _e72.g0_.z, _e75.g0_.y, _e78.g1_.x)) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))), ((((vec4<f32>(_e92.g0_.x) * _e96.g1_) + ((vec4<f32>(_e99.g0_.y) * vec4<f32>(_e103.g0_.y, _e106.g0_.x, _e109.g1_.w, _e112.g1_.z)) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e126.g0_.z) * vec4<f32>(_e130.g0_.z, _e133.g1_.w, _e136.g0_.x, _e139.g1_.y)) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e153.g0_.w) * vec4<f32>(_e157.g0_.w, _e160.g1_.z, _e163.g1_.y, _e166.g0_.x)) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))));
}

fn rotor_point_and_plane_outer_product(self_436: Rotor, other_374: PointAndPlane) -> PointAndPlane {
    var self_437: Rotor;
    var other_375: PointAndPlane;

    self_437 = self_436;
    other_375 = other_374;
    let _e4: Rotor = self_437;
    let _e8: PointAndPlane = other_375;
    let _e11: Rotor = self_437;
    let _e15: PointAndPlane = other_375;
    let _e27: Rotor = self_437;
    let _e31: PointAndPlane = other_375;
    let _e43: Rotor = self_437;
    let _e46: PointAndPlane = other_375;
    let _e58: Rotor = self_437;
    let _e62: PointAndPlane = other_375;
    return PointAndPlane(((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.z) * _e15.g1_.zzxz) * vec4<f32>(1.0, 0.0, -(1.0), 0.0))) + ((vec4<f32>(_e27.g0_.w) * _e31.g1_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))) + ((_e43.g0_.yyxx * _e46.g1_.yxxx) * vec4<f32>(1.0, -(1.0), 0.0, 0.0))), (vec4<f32>(_e58.g0_.x) * _e62.g1_));
}

fn rotor_point_and_plane_inner_product(self_438: Rotor, other_376: PointAndPlane) -> PointAndPlane {
    var self_439: Rotor;
    var other_377: PointAndPlane;

    self_439 = self_438;
    other_377 = other_376;
    let _e4: Rotor = self_439;
    let _e8: PointAndPlane = other_377;
    let _e11: Rotor = self_439;
    let _e15: PointAndPlane = other_377;
    let _e18: Rotor = self_439;
    let _e22: PointAndPlane = other_377;
    let _e25: PointAndPlane = other_377;
    let _e28: PointAndPlane = other_377;
    let _e31: PointAndPlane = other_377;
    let _e45: Rotor = self_439;
    let _e49: PointAndPlane = other_377;
    let _e52: PointAndPlane = other_377;
    let _e55: PointAndPlane = other_377;
    let _e58: PointAndPlane = other_377;
    let _e72: Rotor = self_439;
    let _e76: PointAndPlane = other_377;
    let _e79: PointAndPlane = other_377;
    let _e82: PointAndPlane = other_377;
    let _e85: PointAndPlane = other_377;
    return PointAndPlane((vec4<f32>(_e4.g0_.x) * _e8.g0_), ((((vec4<f32>(_e11.g0_.x) * _e15.g1_) + ((vec4<f32>(_e18.g0_.y) * vec4<f32>(_e22.g0_.y, _e25.g0_.x, _e28.g1_.w, _e31.g1_.z)) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e45.g0_.z) * vec4<f32>(_e49.g0_.z, _e52.g1_.w, _e55.g0_.x, _e58.g1_.y)) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e72.g0_.w) * vec4<f32>(_e76.g0_.w, _e79.g1_.z, _e82.g1_.y, _e85.g0_.x)) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))));
}

fn rotor_point_and_plane_left_contraction(self_440: Rotor, other_378: PointAndPlane) -> PointAndPlane {
    var self_441: Rotor;
    var other_379: PointAndPlane;

    self_441 = self_440;
    other_379 = other_378;
    let _e4: Rotor = self_441;
    let _e8: PointAndPlane = other_379;
    let _e11: Rotor = self_441;
    let _e15: PointAndPlane = other_379;
    let _e18: Rotor = self_441;
    let _e22: PointAndPlane = other_379;
    let _e34: Rotor = self_441;
    let _e38: PointAndPlane = other_379;
    let _e50: Rotor = self_441;
    let _e53: PointAndPlane = other_379;
    return PointAndPlane((vec4<f32>(_e4.g0_.x) * _e8.g0_), ((((vec4<f32>(_e11.g0_.x) * _e15.g1_) + ((vec4<f32>(_e18.g0_.z) * _e22.g0_.zzxz) * vec4<f32>(1.0, 0.0, -(1.0), 0.0))) + ((vec4<f32>(_e34.g0_.w) * _e38.g0_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))) + ((_e50.g0_.yyxx * _e53.g0_.yxxx) * vec4<f32>(1.0, -(1.0), 0.0, 0.0))));
}

fn rotor_squared_magnitude(self_442: Rotor) -> Scalar {
    var self_443: Rotor;

    self_443 = self_442;
    let _e4: Rotor = self_443;
    let _e5: Rotor = rotor_reversal(_e4);
    let _e6: Rotor = self_443;
    let _e8: Rotor = self_443;
    let _e9: Rotor = rotor_reversal(_e8);
    let _e10: Scalar = rotor_rotor_scalar_product(_e6, _e9);
    return _e10;
}

fn rotor_magnitude(self_444: Rotor) -> Scalar {
    var self_445: Rotor;

    self_445 = self_444;
    let _e3: Rotor = self_445;
    let _e4: Scalar = rotor_squared_magnitude(_e3);
    let _e7: Rotor = self_445;
    let _e8: Scalar = rotor_squared_magnitude(_e7);
    return Scalar(sqrt(_e8.g0_));
}

fn rotor_scale(self_446: Rotor, other_380: f32) -> Rotor {
    var self_447: Rotor;
    var other_381: f32;

    self_447 = self_446;
    other_381 = other_380;
    let _e5: f32 = other_381;
    let _e7: Rotor = self_447;
    let _e8: f32 = other_381;
    let _e10: Rotor = rotor_scalar_geometric_product(_e7, Scalar(_e8));
    return _e10;
}

fn rotor_signum(self_448: Rotor) -> Rotor {
    var self_449: Rotor;

    self_449 = self_448;
    let _e5: Rotor = self_449;
    let _e6: Scalar = rotor_magnitude(_e5);
    let _e10: Rotor = self_449;
    let _e13: Rotor = self_449;
    let _e14: Scalar = rotor_magnitude(_e13);
    let _e18: Rotor = rotor_scalar_geometric_product(_e10, Scalar((1.0 / _e14.g0_)));
    return _e18;
}

fn rotor_inverse(self_450: Rotor) -> Rotor {
    var self_451: Rotor;

    self_451 = self_450;
    let _e3: Rotor = self_451;
    let _e4: Rotor = rotor_reversal(_e3);
    let _e7: Rotor = self_451;
    let _e8: Scalar = rotor_squared_magnitude(_e7);
    let _e13: Rotor = self_451;
    let _e14: Rotor = rotor_reversal(_e13);
    let _e17: Rotor = self_451;
    let _e18: Scalar = rotor_squared_magnitude(_e17);
    let _e22: Rotor = rotor_scalar_geometric_product(_e14, Scalar((1.0 / _e18.g0_)));
    return _e22;
}

fn point_zero() -> Point {
    return Point(vec4<f32>(0.0));
}

fn point_one() -> Point {
    return Point(vec4<f32>(0.0));
}

fn point_neg(self_452: Point) -> Point {
    var self_453: Point;

    self_453 = self_452;
    let _e2: Point = self_453;
    return Point((_e2.g0_ * vec4<f32>(-(1.0))));
}

fn point_automorphism(self_454: Point) -> Point {
    var self_455: Point;

    self_455 = self_454;
    let _e2: Point = self_455;
    return Point((_e2.g0_ * vec4<f32>(-(1.0))));
}

fn point_reversal(self_456: Point) -> Point {
    var self_457: Point;

    self_457 = self_456;
    let _e2: Point = self_457;
    return Point((_e2.g0_ * vec4<f32>(-(1.0))));
}

fn point_conjugation(self_458: Point) -> Point {
    var self_459: Point;

    self_459 = self_458;
    let _e2: Point = self_459;
    return Point(_e2.g0_);
}

fn point_dual(self_460: Point) -> Plane {
    var self_461: Point;

    self_461 = self_460;
    let _e2: Point = self_461;
    return Plane((_e2.g0_ * vec4<f32>(-(1.0))));
}

fn point_scalar_geometric_product(self_462: Point, other_382: Scalar) -> Point {
    var self_463: Point;
    var other_383: Scalar;

    self_463 = self_462;
    other_383 = other_382;
    let _e4: Point = self_463;
    let _e6: Scalar = other_383;
    return Point((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn point_scalar_outer_product(self_464: Point, other_384: Scalar) -> Point {
    var self_465: Point;
    var other_385: Scalar;

    self_465 = self_464;
    other_385 = other_384;
    let _e4: Point = self_465;
    let _e6: Scalar = other_385;
    return Point((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn point_scalar_inner_product(self_466: Point, other_386: Scalar) -> Point {
    var self_467: Point;
    var other_387: Scalar;

    self_467 = self_466;
    other_387 = other_386;
    let _e4: Point = self_467;
    let _e6: Scalar = other_387;
    return Point((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn point_scalar_right_contraction(self_468: Point, other_388: Scalar) -> Point {
    var self_469: Point;
    var other_389: Scalar;

    self_469 = self_468;
    other_389 = other_388;
    let _e4: Point = self_469;
    let _e6: Scalar = other_389;
    return Point((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn point_multi_vector_add(self_470: Point, other_390: MultiVector) -> MultiVector {
    var self_471: Point;
    var other_391: MultiVector;

    self_471 = self_470;
    other_391 = other_390;
    let _e4: MultiVector = other_391;
    let _e6: Point = self_471;
    let _e14: MultiVector = other_391;
    let _e17: Point = self_471;
    let _e27: MultiVector = other_391;
    let _e30: MultiVector = other_391;
    return MultiVector(_e4.g0_, ((_e6.g0_ * vec4<f32>(0.0, 1.0, 1.0, 1.0)) + _e14.g1_), ((vec4<f32>(_e17.g0_.x) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) + _e27.g2_), _e30.g3_);
}

fn point_multi_vector_sub(self_472: Point, other_392: MultiVector) -> MultiVector {
    var self_473: Point;
    var other_393: MultiVector;

    self_473 = self_472;
    other_393 = other_392;
    let _e6: MultiVector = other_393;
    let _e9: Point = self_473;
    let _e17: MultiVector = other_393;
    let _e20: Point = self_473;
    let _e30: MultiVector = other_393;
    let _e35: MultiVector = other_393;
    return MultiVector((vec4<f32>(0.0) - _e6.g0_), ((_e9.g0_ * vec4<f32>(0.0, 1.0, 1.0, 1.0)) - _e17.g1_), ((vec4<f32>(_e20.g0_.x) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) - _e30.g2_), (vec4<f32>(0.0) - _e35.g3_));
}

fn point_multi_vector_geometric_product(self_474: Point, other_394: MultiVector) -> MultiVector {
    var self_475: Point;
    var other_395: MultiVector;

    self_475 = self_474;
    other_395 = other_394;
    let _e4: Point = self_475;
    let _e8: MultiVector = other_395;
    let _e18: Point = self_475;
    let _e22: MultiVector = other_395;
    let _e36: Point = self_475;
    let _e40: MultiVector = other_395;
    let _e54: Point = self_475;
    let _e58: MultiVector = other_395;
    let _e72: Point = self_475;
    let _e76: MultiVector = other_395;
    let _e79: Point = self_475;
    let _e83: MultiVector = other_395;
    let _e95: Point = self_475;
    let _e99: MultiVector = other_395;
    let _e111: Point = self_475;
    let _e115: MultiVector = other_395;
    let _e127: Point = self_475;
    let _e131: MultiVector = other_395;
    let _e143: Point = self_475;
    let _e147: MultiVector = other_395;
    let _e160: Point = self_475;
    let _e164: MultiVector = other_395;
    let _e177: Point = self_475;
    let _e181: MultiVector = other_395;
    let _e196: Point = self_475;
    let _e200: MultiVector = other_395;
    let _e204: Point = self_475;
    let _e208: MultiVector = other_395;
    let _e221: Point = self_475;
    let _e225: MultiVector = other_395;
    let _e238: Point = self_475;
    let _e242: MultiVector = other_395;
    return MultiVector((((((vec4<f32>(_e4.g0_.x) * _e8.g2_) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0)) + ((vec4<f32>(_e18.g0_.y) * _e22.g1_.yxwz) * vec4<f32>(-(1.0), -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e36.g0_.z) * _e40.g1_.zwxy) * vec4<f32>(-(1.0), -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e54.g0_.w) * _e58.g1_.wzyx) * vec4<f32>(-(1.0), 1.0, -(1.0), -(1.0)))), ((((vec4<f32>(_e72.g0_.x) * _e76.g3_) + ((vec4<f32>(_e79.g0_.y) * _e83.g0_.yxwz) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e95.g0_.z) * _e99.g0_.zwxy) * vec4<f32>(1.0, -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e111.g0_.w) * _e115.g0_.wzyx) * vec4<f32>(1.0, 1.0, -(1.0), 1.0))), (((((vec4<f32>(_e127.g0_.x) * _e131.g0_) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))) + ((vec4<f32>(_e143.g0_.y) * _e147.g3_.yxwz) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))) + ((vec4<f32>(_e160.g0_.z) * _e164.g3_.zwxy) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e177.g0_.w) * _e181.g3_.wzyx) * vec4<f32>(-(1.0), -(1.0), 1.0, 1.0))), ((((vec4<f32>(0.0) - (vec4<f32>(_e196.g0_.x) * _e200.g1_)) + ((vec4<f32>(_e204.g0_.y) * _e208.g2_.yxwz) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))) + ((vec4<f32>(_e221.g0_.z) * _e225.g2_.zwxy) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e238.g0_.w) * _e242.g2_.wzyx) * vec4<f32>(-(1.0), -(1.0), 1.0, 1.0))));
}

fn point_multi_vector_scalar_product(self_476: Point, other_396: MultiVector) -> Scalar {
    var self_477: Point;
    var other_397: MultiVector;

    self_477 = self_476;
    other_397 = other_396;
    let _e5: Point = self_477;
    let _e8: MultiVector = other_397;
    let _e13: Point = self_477;
    let _e16: MultiVector = other_397;
    let _e21: Point = self_477;
    let _e24: MultiVector = other_397;
    let _e29: Point = self_477;
    let _e32: MultiVector = other_397;
    return Scalar(((((0.0 - (_e5.g0_.x * _e8.g2_.x)) - (_e13.g0_.y * _e16.g1_.y)) - (_e21.g0_.z * _e24.g1_.z)) - (_e29.g0_.w * _e32.g1_.w)));
}

fn point_rotor_geometric_product(self_478: Point, other_398: Rotor) -> PointAndPlane {
    var self_479: Point;
    var other_399: Rotor;

    self_479 = self_478;
    other_399 = other_398;
    let _e4: Point = self_479;
    let _e8: Rotor = other_399;
    let _e19: Point = self_479;
    let _e23: Rotor = other_399;
    let _e35: Point = self_479;
    let _e38: Rotor = other_399;
    let _e50: Point = self_479;
    let _e54: Rotor = other_399;
    let _e65: Point = self_479;
    let _e69: Rotor = other_399;
    let _e81: Point = self_479;
    let _e84: Rotor = other_399;
    return PointAndPlane(((((vec4<f32>(_e4.g0_.z) * _e8.g0_.wwxy) * vec4<f32>(0.0, -(1.0), 1.0, 1.0)) + ((vec4<f32>(_e19.g0_.w) * _e23.g0_.zzyx) * vec4<f32>(0.0, 1.0, -(1.0), 1.0))) + ((_e35.g0_.xyyy * _e38.g0_.xxwz) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))), ((((vec4<f32>(_e50.g0_.z) * vec4<f32>(_e54.g0_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) + ((vec4<f32>(_e65.g0_.w) * vec4<f32>(_e69.g0_.w)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((_e81.g0_.yxxx * _e84.g0_.yyzw) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0)))));
}

fn point_rotor_outer_product(self_480: Point, other_400: Rotor) -> Point {
    var self_481: Point;
    var other_401: Rotor;

    self_481 = self_480;
    other_401 = other_400;
    let _e4: Point = self_481;
    let _e6: Rotor = other_401;
    return Point((_e4.g0_ * vec4<f32>(_e6.g0_.x)));
}

fn point_rotor_inner_product(self_482: Point, other_402: Rotor) -> PointAndPlane {
    var self_483: Point;
    var other_403: Rotor;

    self_483 = self_482;
    other_403 = other_402;
    let _e4: Point = self_483;
    let _e6: Rotor = other_403;
    let _e11: Point = self_483;
    let _e15: Rotor = other_403;
    let _e26: Point = self_483;
    let _e30: Rotor = other_403;
    let _e42: Point = self_483;
    let _e45: Rotor = other_403;
    return PointAndPlane((_e4.g0_ * vec4<f32>(_e6.g0_.x)), ((((vec4<f32>(_e11.g0_.z) * vec4<f32>(_e15.g0_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) + ((vec4<f32>(_e26.g0_.w) * vec4<f32>(_e30.g0_.w)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((_e42.g0_.yxxx * _e45.g0_.yyzw) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0)))));
}

fn point_rotor_right_contraction(self_484: Point, other_404: Rotor) -> PointAndPlane {
    var self_485: Point;
    var other_405: Rotor;

    self_485 = self_484;
    other_405 = other_404;
    let _e4: Point = self_485;
    let _e6: Rotor = other_405;
    let _e11: Point = self_485;
    let _e15: Rotor = other_405;
    let _e26: Point = self_485;
    let _e30: Rotor = other_405;
    let _e42: Point = self_485;
    let _e45: Rotor = other_405;
    return PointAndPlane((_e4.g0_ * vec4<f32>(_e6.g0_.x)), ((((vec4<f32>(_e11.g0_.z) * vec4<f32>(_e15.g0_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) + ((vec4<f32>(_e26.g0_.w) * vec4<f32>(_e30.g0_.w)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((_e42.g0_.yxxx * _e45.g0_.yyzw) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0)))));
}

fn point_point_add(self_486: Point, other_406: Point) -> Point {
    var self_487: Point;
    var other_407: Point;

    self_487 = self_486;
    other_407 = other_406;
    let _e4: Point = self_487;
    let _e6: Point = other_407;
    return Point((_e4.g0_ + _e6.g0_));
}

fn point_point_sub(self_488: Point, other_408: Point) -> Point {
    var self_489: Point;
    var other_409: Point;

    self_489 = self_488;
    other_409 = other_408;
    let _e4: Point = self_489;
    let _e6: Point = other_409;
    return Point((_e4.g0_ - _e6.g0_));
}

fn point_point_mul(self_490: Point, other_410: Point) -> Point {
    var self_491: Point;
    var other_411: Point;

    self_491 = self_490;
    other_411 = other_410;
    let _e4: Point = self_491;
    let _e6: Point = other_411;
    return Point((_e4.g0_ * _e6.g0_));
}

fn point_point_div(self_492: Point, other_412: Point) -> Point {
    var self_493: Point;
    var other_413: Point;

    self_493 = self_492;
    other_413 = other_412;
    let _e4: Point = self_493;
    let _e7: Point = self_493;
    let _e10: Point = self_493;
    let _e13: Point = self_493;
    let _e23: Point = other_413;
    let _e26: Point = other_413;
    let _e29: Point = other_413;
    let _e32: Point = other_413;
    return Point((((vec4<f32>(_e4.g0_.x, _e7.g0_.y, _e10.g0_.z, _e13.g0_.w) * vec4<f32>(1.0, 1.0, 1.0, 1.0)) / vec4<f32>(_e23.g0_.x, _e26.g0_.y, _e29.g0_.z, _e32.g0_.w)) * vec4<f32>(1.0, 1.0, 1.0, 1.0)));
}

fn point_point_regressive_product(self_494: Point, other_414: Point) -> Line {
    var self_495: Point;
    var other_415: Point;

    self_495 = self_494;
    other_415 = other_414;
    let _e4: Point = self_495;
    let _e8: Point = other_415;
    let _e11: Point = other_415;
    let _e14: Point = other_415;
    let _e25: Point = self_495;
    let _e29: Point = other_415;
    let _e32: Point = other_415;
    let _e35: Point = other_415;
    let _e47: Point = self_495;
    let _e50: Point = self_495;
    let _e53: Point = self_495;
    let _e57: Point = other_415;
    let _e60: Point = other_415;
    let _e63: Point = other_415;
    let _e75: Point = self_495;
    let _e79: Point = other_415;
    let _e82: Point = other_415;
    let _e85: Point = other_415;
    let _e90: Point = self_495;
    let _e93: Point = self_495;
    let _e96: Point = self_495;
    let _e100: Point = other_415;
    return Line(((((vec3<f32>(_e4.g0_.z) * vec3<f32>(_e8.g0_.w, _e11.g0_.w, _e14.g0_.y)) * vec3<f32>(1.0, 0.0, -(1.0))) + ((vec3<f32>(_e25.g0_.w) * vec3<f32>(_e29.g0_.z, _e32.g0_.y, _e35.g0_.z)) * vec3<f32>(-(1.0), 1.0, 0.0))) + ((vec3<f32>(_e47.g0_.x, _e50.g0_.y, _e53.g0_.y) * vec3<f32>(_e57.g0_.x, _e60.g0_.w, _e63.g0_.z)) * vec3<f32>(0.0, -(1.0), 1.0))), ((vec3<f32>(_e75.g0_.x) * vec3<f32>(_e79.g0_.y, _e82.g0_.z, _e85.g0_.w)) + ((vec3<f32>(_e90.g0_.y, _e93.g0_.z, _e96.g0_.w) * vec3<f32>(_e100.g0_.x)) * vec3<f32>(-(1.0)))));
}

fn point_point_inner_product(self_496: Point, other_416: Point) -> Scalar {
    var self_497: Point;
    var other_417: Point;

    self_497 = self_496;
    other_417 = other_416;
    let _e5: Point = self_497;
    let _e8: Point = other_417;
    let _e13: Point = self_497;
    let _e16: Point = other_417;
    let _e21: Point = self_497;
    let _e24: Point = other_417;
    let _e29: Point = self_497;
    let _e32: Point = other_417;
    return Scalar(((((0.0 - (_e5.g0_.x * _e8.g0_.x)) - (_e13.g0_.y * _e16.g0_.y)) - (_e21.g0_.z * _e24.g0_.z)) - (_e29.g0_.w * _e32.g0_.w)));
}

fn point_point_left_contraction(self_498: Point, other_418: Point) -> Scalar {
    var self_499: Point;
    var other_419: Point;

    self_499 = self_498;
    other_419 = other_418;
    let _e5: Point = self_499;
    let _e8: Point = other_419;
    let _e13: Point = self_499;
    let _e16: Point = other_419;
    let _e21: Point = self_499;
    let _e24: Point = other_419;
    let _e29: Point = self_499;
    let _e32: Point = other_419;
    return Scalar(((((0.0 - (_e5.g0_.x * _e8.g0_.x)) - (_e13.g0_.y * _e16.g0_.y)) - (_e21.g0_.z * _e24.g0_.z)) - (_e29.g0_.w * _e32.g0_.w)));
}

fn point_point_right_contraction(self_500: Point, other_420: Point) -> Scalar {
    var self_501: Point;
    var other_421: Point;

    self_501 = self_500;
    other_421 = other_420;
    let _e5: Point = self_501;
    let _e8: Point = other_421;
    let _e13: Point = self_501;
    let _e16: Point = other_421;
    let _e21: Point = self_501;
    let _e24: Point = other_421;
    let _e29: Point = self_501;
    let _e32: Point = other_421;
    return Scalar(((((0.0 - (_e5.g0_.x * _e8.g0_.x)) - (_e13.g0_.y * _e16.g0_.y)) - (_e21.g0_.z * _e24.g0_.z)) - (_e29.g0_.w * _e32.g0_.w)));
}

fn point_point_scalar_product(self_502: Point, other_422: Point) -> Scalar {
    var self_503: Point;
    var other_423: Point;

    self_503 = self_502;
    other_423 = other_422;
    let _e5: Point = self_503;
    let _e8: Point = other_423;
    let _e13: Point = self_503;
    let _e16: Point = other_423;
    let _e21: Point = self_503;
    let _e24: Point = other_423;
    let _e29: Point = self_503;
    let _e32: Point = other_423;
    return Scalar(((((0.0 - (_e5.g0_.x * _e8.g0_.x)) - (_e13.g0_.y * _e16.g0_.y)) - (_e21.g0_.z * _e24.g0_.z)) - (_e29.g0_.w * _e32.g0_.w)));
}

fn point_ideal_point_regressive_product(self_504: Point, other_424: IdealPoint) -> Plane {
    var self_505: Point;
    var other_425: IdealPoint;

    self_505 = self_504;
    other_425 = other_424;
    let _e4: Point = self_505;
    let _e8: IdealPoint = other_425;
    let _e20: Point = self_505;
    let _e24: IdealPoint = other_425;
    let _e37: Point = self_505;
    let _e40: IdealPoint = other_425;
    let _e43: IdealPoint = other_425;
    let _e46: IdealPoint = other_425;
    let _e49: IdealPoint = other_425;
    return Plane(((((vec4<f32>(_e4.g0_.z) * vec4<f32>(_e8.g0_.y)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0)) + ((vec4<f32>(_e20.g0_.w) * vec4<f32>(_e24.g0_.z)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((_e37.g0_.yxxx * vec4<f32>(_e40.g0_.x, _e43.g0_.x, _e46.g0_.y, _e49.g0_.z)) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0))));
}

fn point_plane_add(self_506: Point, other_426: Plane) -> PointAndPlane {
    var self_507: Point;
    var other_427: Plane;

    self_507 = self_506;
    other_427 = other_426;
    let _e4: Point = self_507;
    let _e6: Plane = other_427;
    return PointAndPlane(_e4.g0_, _e6.g0_);
}

fn point_plane_sub(self_508: Point, other_428: Plane) -> PointAndPlane {
    var self_509: Point;
    var other_429: Plane;

    self_509 = self_508;
    other_429 = other_428;
    let _e4: Point = self_509;
    let _e8: Plane = other_429;
    return PointAndPlane(_e4.g0_, (vec4<f32>(0.0) - _e8.g0_));
}

fn point_plane_regressive_product(self_510: Point, other_430: Plane) -> Scalar {
    var self_511: Point;
    var other_431: Plane;

    self_511 = self_510;
    other_431 = other_430;
    let _e5: Point = self_511;
    let _e8: Plane = other_431;
    let _e13: Point = self_511;
    let _e16: Plane = other_431;
    let _e21: Point = self_511;
    let _e24: Plane = other_431;
    let _e29: Point = self_511;
    let _e32: Plane = other_431;
    return Scalar(((((0.0 - (_e5.g0_.x * _e8.g0_.x)) - (_e13.g0_.y * _e16.g0_.y)) - (_e21.g0_.z * _e24.g0_.z)) - (_e29.g0_.w * _e32.g0_.w)));
}

fn point_plane_inner_product(self_512: Point, other_432: Plane) -> Line {
    var self_513: Point;
    var other_433: Plane;

    self_513 = self_512;
    other_433 = other_432;
    let _e4: Point = self_513;
    let _e8: Plane = other_433;
    let _e11: Plane = other_433;
    let _e14: Plane = other_433;
    let _e25: Point = self_513;
    let _e29: Plane = other_433;
    let _e32: Plane = other_433;
    let _e35: Plane = other_433;
    let _e47: Point = self_513;
    let _e50: Point = self_513;
    let _e53: Point = self_513;
    let _e57: Plane = other_433;
    let _e60: Plane = other_433;
    let _e63: Plane = other_433;
    let _e75: Point = self_513;
    let _e79: Plane = other_433;
    let _e82: Plane = other_433;
    let _e85: Plane = other_433;
    let _e90: Point = self_513;
    let _e93: Point = self_513;
    let _e96: Point = self_513;
    let _e100: Plane = other_433;
    return Line(((((vec3<f32>(_e4.g0_.z) * vec3<f32>(_e8.g0_.w, _e11.g0_.w, _e14.g0_.y)) * vec3<f32>(1.0, 0.0, -(1.0))) + ((vec3<f32>(_e25.g0_.w) * vec3<f32>(_e29.g0_.z, _e32.g0_.y, _e35.g0_.z)) * vec3<f32>(-(1.0), 1.0, 0.0))) + ((vec3<f32>(_e47.g0_.x, _e50.g0_.y, _e53.g0_.y) * vec3<f32>(_e57.g0_.x, _e60.g0_.w, _e63.g0_.z)) * vec3<f32>(0.0, -(1.0), 1.0))), ((vec3<f32>(_e75.g0_.x) * vec3<f32>(_e79.g0_.y, _e82.g0_.z, _e85.g0_.w)) + ((vec3<f32>(_e90.g0_.y, _e93.g0_.z, _e96.g0_.w) * vec3<f32>(_e100.g0_.x)) * vec3<f32>(-(1.0)))));
}

fn point_plane_right_contraction(self_514: Point, other_434: Plane) -> Line {
    var self_515: Point;
    var other_435: Plane;

    self_515 = self_514;
    other_435 = other_434;
    let _e4: Point = self_515;
    let _e8: Plane = other_435;
    let _e11: Plane = other_435;
    let _e14: Plane = other_435;
    let _e25: Point = self_515;
    let _e29: Plane = other_435;
    let _e32: Plane = other_435;
    let _e35: Plane = other_435;
    let _e47: Point = self_515;
    let _e50: Point = self_515;
    let _e53: Point = self_515;
    let _e57: Plane = other_435;
    let _e60: Plane = other_435;
    let _e63: Plane = other_435;
    let _e75: Point = self_515;
    let _e79: Plane = other_435;
    let _e82: Plane = other_435;
    let _e85: Plane = other_435;
    let _e90: Point = self_515;
    let _e93: Point = self_515;
    let _e96: Point = self_515;
    let _e100: Plane = other_435;
    return Line(((((vec3<f32>(_e4.g0_.z) * vec3<f32>(_e8.g0_.w, _e11.g0_.w, _e14.g0_.y)) * vec3<f32>(1.0, 0.0, -(1.0))) + ((vec3<f32>(_e25.g0_.w) * vec3<f32>(_e29.g0_.z, _e32.g0_.y, _e35.g0_.z)) * vec3<f32>(-(1.0), 1.0, 0.0))) + ((vec3<f32>(_e47.g0_.x, _e50.g0_.y, _e53.g0_.y) * vec3<f32>(_e57.g0_.x, _e60.g0_.w, _e63.g0_.z)) * vec3<f32>(0.0, -(1.0), 1.0))), ((vec3<f32>(_e75.g0_.x) * vec3<f32>(_e79.g0_.y, _e82.g0_.z, _e85.g0_.w)) + ((vec3<f32>(_e90.g0_.y, _e93.g0_.z, _e96.g0_.w) * vec3<f32>(_e100.g0_.x)) * vec3<f32>(-(1.0)))));
}

fn point_line_geometric_product(self_516: Point, other_436: Line) -> PointAndPlane {
    var self_517: Point;
    var other_437: Line;

    self_517 = self_516;
    other_437 = other_436;
    let _e4: Point = self_517;
    let _e8: Line = other_437;
    let _e11: Line = other_437;
    let _e14: Line = other_437;
    let _e17: Line = other_437;
    let _e30: Point = self_517;
    let _e34: Line = other_437;
    let _e37: Line = other_437;
    let _e40: Line = other_437;
    let _e43: Line = other_437;
    let _e57: Point = self_517;
    let _e61: Line = other_437;
    let _e64: Line = other_437;
    let _e67: Line = other_437;
    let _e70: Line = other_437;
    let _e84: Point = self_517;
    let _e88: Line = other_437;
    let _e91: Line = other_437;
    let _e94: Line = other_437;
    let _e97: Line = other_437;
    let _e109: Point = self_517;
    let _e113: Line = other_437;
    let _e116: Line = other_437;
    let _e119: Line = other_437;
    let _e122: Line = other_437;
    let _e134: Point = self_517;
    let _e138: Line = other_437;
    let _e141: Line = other_437;
    let _e144: Line = other_437;
    let _e147: Line = other_437;
    let _e160: Point = self_517;
    let _e164: Line = other_437;
    let _e167: Line = other_437;
    let _e170: Line = other_437;
    let _e173: Line = other_437;
    let _e186: Point = self_517;
    let _e190: Line = other_437;
    let _e193: Line = other_437;
    let _e196: Line = other_437;
    let _e199: Line = other_437;
    return PointAndPlane((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.x, _e11.g0_.x, _e14.g1_.z, _e17.g1_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e30.g0_.z) * vec4<f32>(_e34.g0_.y, _e37.g1_.z, _e40.g0_.y, _e43.g1_.x)) * vec4<f32>(-(1.0), -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e57.g0_.w) * vec4<f32>(_e61.g0_.z, _e64.g1_.y, _e67.g1_.x, _e70.g0_.z)) * vec4<f32>(-(1.0), 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e84.g0_.x) * vec4<f32>(_e88.g0_.x, _e91.g0_.x, _e94.g0_.y, _e97.g0_.z)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))), (((((vec4<f32>(_e109.g0_.y) * vec4<f32>(_e113.g1_.x, _e116.g1_.x, _e119.g0_.z, _e122.g0_.y)) * vec4<f32>(1.0, 0.0, -(1.0), 1.0)) + ((vec4<f32>(_e134.g0_.z) * vec4<f32>(_e138.g1_.y, _e141.g0_.z, _e144.g1_.y, _e147.g0_.x)) * vec4<f32>(1.0, 1.0, 0.0, -(1.0)))) + ((vec4<f32>(_e160.g0_.w) * vec4<f32>(_e164.g1_.z, _e167.g0_.y, _e170.g0_.x, _e173.g1_.z)) * vec4<f32>(1.0, -(1.0), 1.0, 0.0))) + ((vec4<f32>(_e186.g0_.x) * vec4<f32>(_e190.g1_.x, _e193.g1_.x, _e196.g1_.y, _e199.g1_.z)) * vec4<f32>(0.0, -(1.0), -(1.0), -(1.0)))));
}

fn point_line_regressive_product(self_518: Point, other_438: Line) -> Plane {
    var self_519: Point;
    var other_439: Line;

    self_519 = self_518;
    other_439 = other_438;
    let _e4: Point = self_519;
    let _e8: Line = other_439;
    let _e11: Line = other_439;
    let _e14: Line = other_439;
    let _e17: Line = other_439;
    let _e30: Point = self_519;
    let _e34: Line = other_439;
    let _e37: Line = other_439;
    let _e40: Line = other_439;
    let _e43: Line = other_439;
    let _e57: Point = self_519;
    let _e61: Line = other_439;
    let _e64: Line = other_439;
    let _e67: Line = other_439;
    let _e70: Line = other_439;
    let _e84: Point = self_519;
    let _e88: Line = other_439;
    let _e91: Line = other_439;
    let _e94: Line = other_439;
    let _e97: Line = other_439;
    return Plane((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.x, _e11.g0_.x, _e14.g1_.z, _e17.g1_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e30.g0_.z) * vec4<f32>(_e34.g0_.y, _e37.g1_.z, _e40.g0_.y, _e43.g1_.x)) * vec4<f32>(-(1.0), -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e57.g0_.w) * vec4<f32>(_e61.g0_.z, _e64.g1_.y, _e67.g1_.x, _e70.g0_.z)) * vec4<f32>(-(1.0), 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e84.g0_.x) * vec4<f32>(_e88.g0_.x, _e91.g0_.x, _e94.g0_.y, _e97.g0_.z)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn point_line_inner_product(self_520: Point, other_440: Line) -> Plane {
    var self_521: Point;
    var other_441: Line;

    self_521 = self_520;
    other_441 = other_440;
    let _e4: Point = self_521;
    let _e8: Line = other_441;
    let _e11: Line = other_441;
    let _e14: Line = other_441;
    let _e17: Line = other_441;
    let _e29: Point = self_521;
    let _e33: Line = other_441;
    let _e36: Line = other_441;
    let _e39: Line = other_441;
    let _e42: Line = other_441;
    let _e55: Point = self_521;
    let _e59: Line = other_441;
    let _e62: Line = other_441;
    let _e65: Line = other_441;
    let _e68: Line = other_441;
    let _e81: Point = self_521;
    let _e85: Line = other_441;
    let _e88: Line = other_441;
    let _e91: Line = other_441;
    let _e94: Line = other_441;
    return Plane((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g1_.x, _e11.g1_.x, _e14.g0_.z, _e17.g0_.y)) * vec4<f32>(1.0, 0.0, -(1.0), 1.0)) + ((vec4<f32>(_e29.g0_.z) * vec4<f32>(_e33.g1_.y, _e36.g0_.z, _e39.g1_.y, _e42.g0_.x)) * vec4<f32>(1.0, 1.0, 0.0, -(1.0)))) + ((vec4<f32>(_e55.g0_.w) * vec4<f32>(_e59.g1_.z, _e62.g0_.y, _e65.g0_.x, _e68.g1_.z)) * vec4<f32>(1.0, -(1.0), 1.0, 0.0))) + ((vec4<f32>(_e81.g0_.x) * vec4<f32>(_e85.g1_.x, _e88.g1_.x, _e91.g1_.y, _e94.g1_.z)) * vec4<f32>(0.0, -(1.0), -(1.0), -(1.0)))));
}

fn point_line_right_contraction(self_522: Point, other_442: Line) -> Plane {
    var self_523: Point;
    var other_443: Line;

    self_523 = self_522;
    other_443 = other_442;
    let _e4: Point = self_523;
    let _e8: Line = other_443;
    let _e11: Line = other_443;
    let _e14: Line = other_443;
    let _e17: Line = other_443;
    let _e29: Point = self_523;
    let _e33: Line = other_443;
    let _e36: Line = other_443;
    let _e39: Line = other_443;
    let _e42: Line = other_443;
    let _e55: Point = self_523;
    let _e59: Line = other_443;
    let _e62: Line = other_443;
    let _e65: Line = other_443;
    let _e68: Line = other_443;
    let _e81: Point = self_523;
    let _e85: Line = other_443;
    let _e88: Line = other_443;
    let _e91: Line = other_443;
    let _e94: Line = other_443;
    return Plane((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g1_.x, _e11.g1_.x, _e14.g0_.z, _e17.g0_.y)) * vec4<f32>(1.0, 0.0, -(1.0), 1.0)) + ((vec4<f32>(_e29.g0_.z) * vec4<f32>(_e33.g1_.y, _e36.g0_.z, _e39.g1_.y, _e42.g0_.x)) * vec4<f32>(1.0, 1.0, 0.0, -(1.0)))) + ((vec4<f32>(_e55.g0_.w) * vec4<f32>(_e59.g1_.z, _e62.g0_.y, _e65.g0_.x, _e68.g1_.z)) * vec4<f32>(1.0, -(1.0), 1.0, 0.0))) + ((vec4<f32>(_e81.g0_.x) * vec4<f32>(_e85.g1_.x, _e88.g1_.x, _e91.g1_.y, _e94.g1_.z)) * vec4<f32>(0.0, -(1.0), -(1.0), -(1.0)))));
}

fn point_translator_regressive_product(self_524: Point, other_444: Translator) -> Plane {
    var self_525: Point;
    var other_445: Translator;

    self_525 = self_524;
    other_445 = other_444;
    let _e4: Point = self_525;
    let _e8: Translator = other_445;
    let _e20: Point = self_525;
    let _e24: Translator = other_445;
    let _e37: Point = self_525;
    let _e40: Translator = other_445;
    return Plane(((((vec4<f32>(_e4.g0_.z) * vec4<f32>(_e8.g0_.z)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0)) + ((vec4<f32>(_e20.g0_.w) * vec4<f32>(_e24.g0_.w)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((_e37.g0_.yxxx * _e40.g0_.yyzw) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0))));
}

fn point_translator_outer_product(self_526: Point, other_446: Translator) -> Point {
    var self_527: Point;
    var other_447: Translator;

    self_527 = self_526;
    other_447 = other_446;
    let _e4: Point = self_527;
    let _e6: Translator = other_447;
    return Point((_e4.g0_ * vec4<f32>(_e6.g0_.x)));
}

fn point_motor_geometric_product(self_528: Point, other_448: Motor) -> PointAndPlane {
    var self_529: Point;
    var other_449: Motor;

    self_529 = self_528;
    other_449 = other_448;
    let _e4: Point = self_529;
    let _e8: Motor = other_449;
    let _e11: Motor = other_449;
    let _e14: Motor = other_449;
    let _e17: Motor = other_449;
    let _e22: Point = self_529;
    let _e26: Motor = other_449;
    let _e29: Motor = other_449;
    let _e32: Motor = other_449;
    let _e35: Motor = other_449;
    let _e49: Point = self_529;
    let _e53: Motor = other_449;
    let _e56: Motor = other_449;
    let _e59: Motor = other_449;
    let _e62: Motor = other_449;
    let _e76: Point = self_529;
    let _e80: Motor = other_449;
    let _e83: Motor = other_449;
    let _e86: Motor = other_449;
    let _e89: Motor = other_449;
    let _e103: Point = self_529;
    let _e107: Motor = other_449;
    let _e110: Motor = other_449;
    let _e113: Motor = other_449;
    let _e116: Motor = other_449;
    let _e130: Point = self_529;
    let _e134: Motor = other_449;
    let _e137: Motor = other_449;
    let _e140: Motor = other_449;
    let _e143: Motor = other_449;
    let _e156: Point = self_529;
    let _e160: Motor = other_449;
    let _e163: Motor = other_449;
    let _e166: Motor = other_449;
    let _e169: Motor = other_449;
    let _e182: Point = self_529;
    let _e186: Motor = other_449;
    let _e189: Motor = other_449;
    let _e192: Motor = other_449;
    let _e195: Motor = other_449;
    return PointAndPlane(((((vec4<f32>(_e4.g0_.x) * vec4<f32>(_e8.g0_.x, _e11.g1_.y, _e14.g1_.z, _e17.g1_.w)) + ((vec4<f32>(_e22.g0_.y) * vec4<f32>(_e26.g1_.y, _e29.g0_.x, _e32.g0_.w, _e35.g0_.z)) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e49.g0_.z) * vec4<f32>(_e53.g1_.z, _e56.g0_.w, _e59.g0_.x, _e62.g0_.y)) * vec4<f32>(-(1.0), -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e76.g0_.w) * vec4<f32>(_e80.g1_.w, _e83.g0_.z, _e86.g0_.y, _e89.g0_.x)) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))), (((((vec4<f32>(_e103.g0_.x) * vec4<f32>(_e107.g1_.x, _e110.g0_.y, _e113.g0_.z, _e116.g0_.w)) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))) + ((vec4<f32>(_e130.g0_.y) * vec4<f32>(_e134.g0_.y, _e137.g1_.x, _e140.g1_.w, _e143.g1_.z)) * vec4<f32>(1.0, 1.0, -(1.0), 1.0))) + ((vec4<f32>(_e156.g0_.z) * vec4<f32>(_e160.g0_.z, _e163.g1_.w, _e166.g1_.x, _e169.g1_.y)) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e182.g0_.w) * vec4<f32>(_e186.g0_.w, _e189.g1_.z, _e192.g1_.y, _e195.g1_.x)) * vec4<f32>(1.0, -(1.0), 1.0, 1.0))));
}

fn point_motor_regressive_product(self_530: Point, other_450: Motor) -> PointAndPlane {
    var self_531: Point;
    var other_451: Motor;

    self_531 = self_530;
    other_451 = other_450;
    let _e4: Point = self_531;
    let _e6: Motor = other_451;
    let _e11: Point = self_531;
    let _e15: Motor = other_451;
    let _e18: Motor = other_451;
    let _e21: Motor = other_451;
    let _e24: Motor = other_451;
    let _e37: Point = self_531;
    let _e41: Motor = other_451;
    let _e44: Motor = other_451;
    let _e47: Motor = other_451;
    let _e50: Motor = other_451;
    let _e64: Point = self_531;
    let _e68: Motor = other_451;
    let _e71: Motor = other_451;
    let _e74: Motor = other_451;
    let _e77: Motor = other_451;
    let _e91: Point = self_531;
    let _e95: Motor = other_451;
    return PointAndPlane((_e4.g0_ * vec4<f32>(_e6.g1_.x)), (((((vec4<f32>(_e11.g0_.y) * vec4<f32>(_e15.g1_.y, _e18.g1_.y, _e21.g0_.w, _e24.g0_.z)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e37.g0_.z) * vec4<f32>(_e41.g1_.z, _e44.g0_.w, _e47.g1_.z, _e50.g0_.y)) * vec4<f32>(-(1.0), -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e64.g0_.w) * vec4<f32>(_e68.g1_.w, _e71.g0_.z, _e74.g0_.y, _e77.g1_.w)) * vec4<f32>(-(1.0), 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e91.g0_.x) * _e95.g1_) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn point_motor_outer_product(self_532: Point, other_452: Motor) -> Point {
    var self_533: Point;
    var other_453: Motor;

    self_533 = self_532;
    other_453 = other_452;
    let _e4: Point = self_533;
    let _e6: Motor = other_453;
    return Point((_e4.g0_ * vec4<f32>(_e6.g0_.x)));
}

fn point_motor_inner_product(self_534: Point, other_454: Motor) -> PointAndPlane {
    var self_535: Point;
    var other_455: Motor;

    self_535 = self_534;
    other_455 = other_454;
    let _e4: Point = self_535;
    let _e6: Motor = other_455;
    let _e11: Point = self_535;
    let _e15: Motor = other_455;
    let _e18: Motor = other_455;
    let _e21: Motor = other_455;
    let _e24: Motor = other_455;
    let _e38: Point = self_535;
    let _e42: Motor = other_455;
    let _e45: Motor = other_455;
    let _e48: Motor = other_455;
    let _e51: Motor = other_455;
    let _e64: Point = self_535;
    let _e68: Motor = other_455;
    let _e71: Motor = other_455;
    let _e74: Motor = other_455;
    let _e77: Motor = other_455;
    let _e90: Point = self_535;
    let _e94: Motor = other_455;
    let _e97: Motor = other_455;
    let _e100: Motor = other_455;
    let _e103: Motor = other_455;
    return PointAndPlane((_e4.g0_ * vec4<f32>(_e6.g0_.x)), (((((vec4<f32>(_e11.g0_.x) * vec4<f32>(_e15.g1_.x, _e18.g0_.y, _e21.g0_.z, _e24.g0_.w)) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))) + ((vec4<f32>(_e38.g0_.y) * vec4<f32>(_e42.g0_.y, _e45.g1_.x, _e48.g1_.w, _e51.g1_.z)) * vec4<f32>(1.0, 1.0, -(1.0), 1.0))) + ((vec4<f32>(_e64.g0_.z) * vec4<f32>(_e68.g0_.z, _e71.g1_.w, _e74.g1_.x, _e77.g1_.y)) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e90.g0_.w) * vec4<f32>(_e94.g0_.w, _e97.g1_.z, _e100.g1_.y, _e103.g1_.x)) * vec4<f32>(1.0, -(1.0), 1.0, 1.0))));
}

fn point_motor_left_contraction(self_536: Point, other_456: Motor) -> Plane {
    var self_537: Point;
    var other_457: Motor;

    self_537 = self_536;
    other_457 = other_456;
    let _e4: Point = self_537;
    let _e6: Motor = other_457;
    return Plane((_e4.g0_ * vec4<f32>(_e6.g1_.x)));
}

fn point_motor_right_contraction(self_538: Point, other_458: Motor) -> PointAndPlane {
    var self_539: Point;
    var other_459: Motor;

    self_539 = self_538;
    other_459 = other_458;
    let _e4: Point = self_539;
    let _e6: Motor = other_459;
    let _e11: Point = self_539;
    let _e15: Motor = other_459;
    let _e18: Motor = other_459;
    let _e21: Motor = other_459;
    let _e24: Motor = other_459;
    let _e36: Point = self_539;
    let _e40: Motor = other_459;
    let _e43: Motor = other_459;
    let _e46: Motor = other_459;
    let _e49: Motor = other_459;
    let _e62: Point = self_539;
    let _e66: Motor = other_459;
    let _e69: Motor = other_459;
    let _e72: Motor = other_459;
    let _e75: Motor = other_459;
    let _e88: Point = self_539;
    let _e92: Motor = other_459;
    return PointAndPlane((_e4.g0_ * vec4<f32>(_e6.g0_.x)), (((((vec4<f32>(_e11.g0_.y) * vec4<f32>(_e15.g0_.y, _e18.g0_.y, _e21.g1_.w, _e24.g1_.z)) * vec4<f32>(1.0, 0.0, -(1.0), 1.0)) + ((vec4<f32>(_e36.g0_.z) * vec4<f32>(_e40.g0_.z, _e43.g1_.w, _e46.g0_.z, _e49.g1_.y)) * vec4<f32>(1.0, 1.0, 0.0, -(1.0)))) + ((vec4<f32>(_e62.g0_.w) * vec4<f32>(_e66.g0_.w, _e69.g1_.z, _e72.g1_.y, _e75.g0_.w)) * vec4<f32>(1.0, -(1.0), 1.0, 0.0))) + ((vec4<f32>(_e88.g0_.x) * _e92.g0_) * vec4<f32>(0.0, -(1.0), -(1.0), -(1.0)))));
}

fn point_point_and_plane_add(self_540: Point, other_460: PointAndPlane) -> PointAndPlane {
    var self_541: Point;
    var other_461: PointAndPlane;

    self_541 = self_540;
    other_461 = other_460;
    let _e4: Point = self_541;
    let _e6: PointAndPlane = other_461;
    let _e9: PointAndPlane = other_461;
    return PointAndPlane((_e4.g0_ + _e6.g0_), _e9.g1_);
}

fn point_point_and_plane_sub(self_542: Point, other_462: PointAndPlane) -> PointAndPlane {
    var self_543: Point;
    var other_463: PointAndPlane;

    self_543 = self_542;
    other_463 = other_462;
    let _e4: Point = self_543;
    let _e6: PointAndPlane = other_463;
    let _e11: PointAndPlane = other_463;
    return PointAndPlane((_e4.g0_ - _e6.g0_), (vec4<f32>(0.0) - _e11.g1_));
}

fn point_point_and_plane_geometric_product(self_544: Point, other_464: PointAndPlane) -> Motor {
    var self_545: Point;
    var other_465: PointAndPlane;

    self_545 = self_544;
    other_465 = other_464;
    let _e4: Point = self_545;
    let _e8: PointAndPlane = other_465;
    let _e11: PointAndPlane = other_465;
    let _e14: PointAndPlane = other_465;
    let _e17: PointAndPlane = other_465;
    let _e29: Point = self_545;
    let _e33: PointAndPlane = other_465;
    let _e36: PointAndPlane = other_465;
    let _e39: PointAndPlane = other_465;
    let _e42: PointAndPlane = other_465;
    let _e57: Point = self_545;
    let _e61: PointAndPlane = other_465;
    let _e64: PointAndPlane = other_465;
    let _e67: PointAndPlane = other_465;
    let _e70: PointAndPlane = other_465;
    let _e85: Point = self_545;
    let _e89: PointAndPlane = other_465;
    let _e92: PointAndPlane = other_465;
    let _e95: PointAndPlane = other_465;
    let _e98: PointAndPlane = other_465;
    let _e115: Point = self_545;
    let _e119: PointAndPlane = other_465;
    let _e122: PointAndPlane = other_465;
    let _e125: PointAndPlane = other_465;
    let _e128: PointAndPlane = other_465;
    let _e134: Point = self_545;
    let _e138: PointAndPlane = other_465;
    let _e141: PointAndPlane = other_465;
    let _e144: PointAndPlane = other_465;
    let _e147: PointAndPlane = other_465;
    let _e161: Point = self_545;
    let _e165: PointAndPlane = other_465;
    let _e168: PointAndPlane = other_465;
    let _e171: PointAndPlane = other_465;
    let _e174: PointAndPlane = other_465;
    let _e188: Point = self_545;
    let _e192: PointAndPlane = other_465;
    let _e195: PointAndPlane = other_465;
    let _e198: PointAndPlane = other_465;
    let _e201: PointAndPlane = other_465;
    return Motor((((((vec4<f32>(_e4.g0_.x) * vec4<f32>(_e8.g0_.x, _e11.g1_.y, _e14.g1_.z, _e17.g1_.w)) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0)) + ((vec4<f32>(_e29.g0_.y) * vec4<f32>(_e33.g0_.y, _e36.g1_.x, _e39.g0_.w, _e42.g0_.z)) * vec4<f32>(-(1.0), -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e57.g0_.z) * vec4<f32>(_e61.g0_.z, _e64.g0_.w, _e67.g1_.x, _e70.g0_.y)) * vec4<f32>(-(1.0), -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e85.g0_.w) * vec4<f32>(_e89.g0_.w, _e92.g0_.z, _e95.g0_.y, _e98.g1_.x)) * vec4<f32>(-(1.0), 1.0, -(1.0), -(1.0)))), ((((vec4<f32>(0.0) - (vec4<f32>(_e115.g0_.x) * vec4<f32>(_e119.g1_.x, _e122.g0_.y, _e125.g0_.z, _e128.g0_.w))) + ((vec4<f32>(_e134.g0_.y) * vec4<f32>(_e138.g1_.y, _e141.g0_.x, _e144.g1_.w, _e147.g1_.z)) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))) + ((vec4<f32>(_e161.g0_.z) * vec4<f32>(_e165.g1_.z, _e168.g1_.w, _e171.g0_.x, _e174.g1_.y)) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e188.g0_.w) * vec4<f32>(_e192.g1_.w, _e195.g1_.z, _e198.g1_.y, _e201.g0_.x)) * vec4<f32>(-(1.0), -(1.0), 1.0, 1.0))));
}

fn point_point_and_plane_left_contraction(self_546: Point, other_466: PointAndPlane) -> Scalar {
    var self_547: Point;
    var other_467: PointAndPlane;

    self_547 = self_546;
    other_467 = other_466;
    let _e5: Point = self_547;
    let _e8: PointAndPlane = other_467;
    let _e13: Point = self_547;
    let _e16: PointAndPlane = other_467;
    let _e21: Point = self_547;
    let _e24: PointAndPlane = other_467;
    let _e29: Point = self_547;
    let _e32: PointAndPlane = other_467;
    return Scalar(((((0.0 - (_e5.g0_.x * _e8.g0_.x)) - (_e13.g0_.y * _e16.g0_.y)) - (_e21.g0_.z * _e24.g0_.z)) - (_e29.g0_.w * _e32.g0_.w)));
}

fn point_point_and_plane_scalar_product(self_548: Point, other_468: PointAndPlane) -> Scalar {
    var self_549: Point;
    var other_469: PointAndPlane;

    self_549 = self_548;
    other_469 = other_468;
    let _e5: Point = self_549;
    let _e8: PointAndPlane = other_469;
    let _e13: Point = self_549;
    let _e16: PointAndPlane = other_469;
    let _e21: Point = self_549;
    let _e24: PointAndPlane = other_469;
    let _e29: Point = self_549;
    let _e32: PointAndPlane = other_469;
    return Scalar(((((0.0 - (_e5.g0_.x * _e8.g0_.x)) - (_e13.g0_.y * _e16.g0_.y)) - (_e21.g0_.z * _e24.g0_.z)) - (_e29.g0_.w * _e32.g0_.w)));
}

fn point_squared_magnitude(self_550: Point) -> Scalar {
    var self_551: Point;

    self_551 = self_550;
    let _e4: Point = self_551;
    let _e5: Point = point_reversal(_e4);
    let _e6: Point = self_551;
    let _e8: Point = self_551;
    let _e9: Point = point_reversal(_e8);
    let _e10: Scalar = point_point_scalar_product(_e6, _e9);
    return _e10;
}

fn point_magnitude(self_552: Point) -> Scalar {
    var self_553: Point;

    self_553 = self_552;
    let _e3: Point = self_553;
    let _e4: Scalar = point_squared_magnitude(_e3);
    let _e7: Point = self_553;
    let _e8: Scalar = point_squared_magnitude(_e7);
    return Scalar(sqrt(_e8.g0_));
}

fn point_scale(self_554: Point, other_470: f32) -> Point {
    var self_555: Point;
    var other_471: f32;

    self_555 = self_554;
    other_471 = other_470;
    let _e5: f32 = other_471;
    let _e7: Point = self_555;
    let _e8: f32 = other_471;
    let _e10: Point = point_scalar_geometric_product(_e7, Scalar(_e8));
    return _e10;
}

fn point_signum(self_556: Point) -> Point {
    var self_557: Point;

    self_557 = self_556;
    let _e5: Point = self_557;
    let _e6: Scalar = point_magnitude(_e5);
    let _e10: Point = self_557;
    let _e13: Point = self_557;
    let _e14: Scalar = point_magnitude(_e13);
    let _e18: Point = point_scalar_geometric_product(_e10, Scalar((1.0 / _e14.g0_)));
    return _e18;
}

fn point_inverse(self_558: Point) -> Point {
    var self_559: Point;

    self_559 = self_558;
    let _e3: Point = self_559;
    let _e4: Point = point_reversal(_e3);
    let _e7: Point = self_559;
    let _e8: Scalar = point_squared_magnitude(_e7);
    let _e13: Point = self_559;
    let _e14: Point = point_reversal(_e13);
    let _e17: Point = self_559;
    let _e18: Scalar = point_squared_magnitude(_e17);
    let _e22: Point = point_scalar_geometric_product(_e14, Scalar((1.0 / _e18.g0_)));
    return _e22;
}

fn ideal_point_zero() -> IdealPoint {
    return IdealPoint(vec3<f32>(0.0));
}

fn ideal_point_one() -> IdealPoint {
    return IdealPoint(vec3<f32>(0.0));
}

fn ideal_point_neg(self_560: IdealPoint) -> IdealPoint {
    var self_561: IdealPoint;

    self_561 = self_560;
    let _e2: IdealPoint = self_561;
    return IdealPoint((_e2.g0_ * vec3<f32>(-(1.0))));
}

fn ideal_point_automorphism(self_562: IdealPoint) -> IdealPoint {
    var self_563: IdealPoint;

    self_563 = self_562;
    let _e2: IdealPoint = self_563;
    return IdealPoint(_e2.g0_);
}

fn ideal_point_reversal(self_564: IdealPoint) -> IdealPoint {
    var self_565: IdealPoint;

    self_565 = self_564;
    let _e2: IdealPoint = self_565;
    return IdealPoint((_e2.g0_ * vec3<f32>(-(1.0))));
}

fn ideal_point_conjugation(self_566: IdealPoint) -> IdealPoint {
    var self_567: IdealPoint;

    self_567 = self_566;
    let _e2: IdealPoint = self_567;
    return IdealPoint((_e2.g0_ * vec3<f32>(-(1.0))));
}

fn ideal_point_scalar_add(self_568: IdealPoint, other_472: Scalar) -> Translator {
    var self_569: IdealPoint;
    var other_473: Scalar;

    self_569 = self_568;
    other_473 = other_472;
    let _e4: IdealPoint = self_569;
    let _e7: IdealPoint = self_569;
    let _e10: IdealPoint = self_569;
    let _e13: IdealPoint = self_569;
    let _e23: Scalar = other_473;
    return Translator(((vec4<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.y, _e13.g0_.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0)) + (vec4<f32>(_e23.g0_) * vec4<f32>(1.0, 0.0, 0.0, 0.0))));
}

fn ideal_point_scalar_sub(self_570: IdealPoint, other_474: Scalar) -> Translator {
    var self_571: IdealPoint;
    var other_475: Scalar;

    self_571 = self_570;
    other_475 = other_474;
    let _e4: IdealPoint = self_571;
    let _e7: IdealPoint = self_571;
    let _e10: IdealPoint = self_571;
    let _e13: IdealPoint = self_571;
    let _e23: Scalar = other_475;
    return Translator(((vec4<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.y, _e13.g0_.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0)) - (vec4<f32>(_e23.g0_) * vec4<f32>(1.0, 0.0, 0.0, 0.0))));
}

fn ideal_point_scalar_geometric_product(self_572: IdealPoint, other_476: Scalar) -> IdealPoint {
    var self_573: IdealPoint;
    var other_477: Scalar;

    self_573 = self_572;
    other_477 = other_476;
    let _e4: IdealPoint = self_573;
    let _e6: Scalar = other_477;
    return IdealPoint((_e4.g0_ * vec3<f32>(_e6.g0_)));
}

fn ideal_point_scalar_outer_product(self_574: IdealPoint, other_478: Scalar) -> IdealPoint {
    var self_575: IdealPoint;
    var other_479: Scalar;

    self_575 = self_574;
    other_479 = other_478;
    let _e4: IdealPoint = self_575;
    let _e6: Scalar = other_479;
    return IdealPoint((_e4.g0_ * vec3<f32>(_e6.g0_)));
}

fn ideal_point_scalar_inner_product(self_576: IdealPoint, other_480: Scalar) -> IdealPoint {
    var self_577: IdealPoint;
    var other_481: Scalar;

    self_577 = self_576;
    other_481 = other_480;
    let _e4: IdealPoint = self_577;
    let _e6: Scalar = other_481;
    return IdealPoint((_e4.g0_ * vec3<f32>(_e6.g0_)));
}

fn ideal_point_scalar_right_contraction(self_578: IdealPoint, other_482: Scalar) -> IdealPoint {
    var self_579: IdealPoint;
    var other_483: Scalar;

    self_579 = self_578;
    other_483 = other_482;
    let _e4: IdealPoint = self_579;
    let _e6: Scalar = other_483;
    return IdealPoint((_e4.g0_ * vec3<f32>(_e6.g0_)));
}

fn ideal_point_multi_vector_add(self_580: IdealPoint, other_484: MultiVector) -> MultiVector {
    var self_581: IdealPoint;
    var other_485: MultiVector;

    self_581 = self_580;
    other_485 = other_484;
    let _e4: MultiVector = other_485;
    let _e6: MultiVector = other_485;
    let _e8: MultiVector = other_485;
    let _e10: IdealPoint = self_581;
    let _e13: IdealPoint = self_581;
    let _e16: IdealPoint = self_581;
    let _e19: IdealPoint = self_581;
    let _e29: MultiVector = other_485;
    return MultiVector(_e4.g0_, _e6.g1_, _e8.g2_, ((vec4<f32>(_e10.g0_.x, _e13.g0_.x, _e16.g0_.y, _e19.g0_.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0)) + _e29.g3_));
}

fn ideal_point_multi_vector_sub(self_582: IdealPoint, other_486: MultiVector) -> MultiVector {
    var self_583: IdealPoint;
    var other_487: MultiVector;

    self_583 = self_582;
    other_487 = other_486;
    let _e6: MultiVector = other_487;
    let _e11: MultiVector = other_487;
    let _e16: MultiVector = other_487;
    let _e19: IdealPoint = self_583;
    let _e22: IdealPoint = self_583;
    let _e25: IdealPoint = self_583;
    let _e28: IdealPoint = self_583;
    let _e38: MultiVector = other_487;
    return MultiVector((vec4<f32>(0.0) - _e6.g0_), (vec4<f32>(0.0) - _e11.g1_), (vec4<f32>(0.0) - _e16.g2_), ((vec4<f32>(_e19.g0_.x, _e22.g0_.x, _e25.g0_.y, _e28.g0_.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0)) - _e38.g3_));
}

fn ideal_point_multi_vector_geometric_product(self_584: IdealPoint, other_488: MultiVector) -> MultiVector {
    var self_585: IdealPoint;
    var other_489: MultiVector;

    self_585 = self_584;
    other_489 = other_488;
    let _e4: IdealPoint = self_585;
    let _e8: MultiVector = other_489;
    let _e21: IdealPoint = self_585;
    let _e25: MultiVector = other_489;
    let _e39: IdealPoint = self_585;
    let _e43: MultiVector = other_489;
    let _e57: IdealPoint = self_585;
    let _e61: MultiVector = other_489;
    let _e73: IdealPoint = self_585;
    let _e77: MultiVector = other_489;
    let _e90: IdealPoint = self_585;
    let _e94: MultiVector = other_489;
    let _e107: IdealPoint = self_585;
    let _e111: MultiVector = other_489;
    let _e123: IdealPoint = self_585;
    let _e127: MultiVector = other_489;
    let _e140: IdealPoint = self_585;
    let _e144: MultiVector = other_489;
    let _e157: IdealPoint = self_585;
    let _e161: MultiVector = other_489;
    let _e172: IdealPoint = self_585;
    let _e176: MultiVector = other_489;
    let _e188: IdealPoint = self_585;
    let _e192: MultiVector = other_489;
    return MultiVector(((((vec4<f32>(_e4.g0_.x) * _e8.g3_.yxwz) * vec4<f32>(-(1.0), -(1.0), 1.0, -(1.0))) + ((vec4<f32>(_e21.g0_.y) * _e25.g3_.zwxy) * vec4<f32>(-(1.0), -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e39.g0_.z) * _e43.g3_.wzyx) * vec4<f32>(-(1.0), 1.0, -(1.0), -(1.0)))), ((((vec4<f32>(_e57.g0_.x) * _e61.g2_.yxwz) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0))) + ((vec4<f32>(_e73.g0_.y) * _e77.g2_.zwxy) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e90.g0_.z) * _e94.g2_.wzyx) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))), ((((vec4<f32>(_e107.g0_.x) * _e111.g1_.yxwz) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0))) + ((vec4<f32>(_e123.g0_.y) * _e127.g1_.zwxy) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e140.g0_.z) * _e144.g1_.wzyx) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))), ((((vec4<f32>(_e157.g0_.x) * _e161.g0_.yxwz) * vec4<f32>(1.0, 1.0, 1.0, -(1.0))) + ((vec4<f32>(_e172.g0_.y) * _e176.g0_.zwxy) * vec4<f32>(1.0, -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e188.g0_.z) * _e192.g0_.wzyx) * vec4<f32>(1.0, 1.0, -(1.0), 1.0))));
}

fn ideal_point_multi_vector_scalar_product(self_586: IdealPoint, other_490: MultiVector) -> Scalar {
    var self_587: IdealPoint;
    var other_491: MultiVector;

    self_587 = self_586;
    other_491 = other_490;
    let _e5: IdealPoint = self_587;
    let _e8: MultiVector = other_491;
    let _e13: IdealPoint = self_587;
    let _e16: MultiVector = other_491;
    let _e21: IdealPoint = self_587;
    let _e24: MultiVector = other_491;
    return Scalar((((0.0 - (_e5.g0_.x * _e8.g3_.y)) - (_e13.g0_.y * _e16.g3_.z)) - (_e21.g0_.z * _e24.g3_.w)));
}

fn ideal_point_rotor_regressive_product(self_588: IdealPoint, other_492: Rotor) -> Scalar {
    var self_589: IdealPoint;
    var other_493: Rotor;

    self_589 = self_588;
    other_493 = other_492;
    let _e4: IdealPoint = self_589;
    let _e7: Rotor = other_493;
    let _e11: IdealPoint = self_589;
    let _e14: Rotor = other_493;
    let _e19: IdealPoint = self_589;
    let _e22: Rotor = other_493;
    return Scalar((((_e4.g0_.x * _e7.g0_.y) + (_e11.g0_.y * _e14.g0_.z)) + (_e19.g0_.z * _e22.g0_.w)));
}

fn ideal_point_rotor_inner_product(self_590: IdealPoint, other_494: Rotor) -> IdealPoint {
    var self_591: IdealPoint;
    var other_495: Rotor;

    self_591 = self_590;
    other_495 = other_494;
    let _e4: IdealPoint = self_591;
    let _e6: Rotor = other_495;
    return IdealPoint((_e4.g0_ * vec3<f32>(_e6.g0_.x)));
}

fn ideal_point_rotor_right_contraction(self_592: IdealPoint, other_496: Rotor) -> IdealPoint {
    var self_593: IdealPoint;
    var other_497: Rotor;

    self_593 = self_592;
    other_497 = other_496;
    let _e4: IdealPoint = self_593;
    let _e6: Rotor = other_497;
    return IdealPoint((_e4.g0_ * vec3<f32>(_e6.g0_.x)));
}

fn ideal_point_point_regressive_product(self_594: IdealPoint, other_498: Point) -> Plane {
    var self_595: IdealPoint;
    var other_499: Point;

    self_595 = self_594;
    other_499 = other_498;
    let _e4: IdealPoint = self_595;
    let _e8: Point = other_499;
    let _e19: IdealPoint = self_595;
    let _e23: Point = other_499;
    let _e35: IdealPoint = self_595;
    let _e39: Point = other_499;
    return Plane(((((vec4<f32>(_e4.g0_.y) * _e8.g0_.zzxz) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0)) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.wwwx) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))) + ((vec4<f32>(_e35.g0_.x) * _e39.g0_.yxxx) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))));
}

fn ideal_point_ideal_point_add(self_596: IdealPoint, other_500: IdealPoint) -> IdealPoint {
    var self_597: IdealPoint;
    var other_501: IdealPoint;

    self_597 = self_596;
    other_501 = other_500;
    let _e4: IdealPoint = self_597;
    let _e6: IdealPoint = other_501;
    return IdealPoint((_e4.g0_ + _e6.g0_));
}

fn ideal_point_ideal_point_sub(self_598: IdealPoint, other_502: IdealPoint) -> IdealPoint {
    var self_599: IdealPoint;
    var other_503: IdealPoint;

    self_599 = self_598;
    other_503 = other_502;
    let _e4: IdealPoint = self_599;
    let _e6: IdealPoint = other_503;
    return IdealPoint((_e4.g0_ - _e6.g0_));
}

fn ideal_point_ideal_point_mul(self_600: IdealPoint, other_504: IdealPoint) -> IdealPoint {
    var self_601: IdealPoint;
    var other_505: IdealPoint;

    self_601 = self_600;
    other_505 = other_504;
    let _e4: IdealPoint = self_601;
    let _e6: IdealPoint = other_505;
    return IdealPoint((_e4.g0_ * _e6.g0_));
}

fn ideal_point_ideal_point_div(self_602: IdealPoint, other_506: IdealPoint) -> IdealPoint {
    var self_603: IdealPoint;
    var other_507: IdealPoint;

    self_603 = self_602;
    other_507 = other_506;
    let _e4: IdealPoint = self_603;
    let _e7: IdealPoint = self_603;
    let _e10: IdealPoint = self_603;
    let _e19: IdealPoint = other_507;
    let _e22: IdealPoint = other_507;
    let _e25: IdealPoint = other_507;
    return IdealPoint((((vec3<f32>(_e4.g0_.x, _e7.g0_.y, _e10.g0_.z) * vec3<f32>(1.0, 1.0, 1.0)) / vec3<f32>(_e19.g0_.x, _e22.g0_.y, _e25.g0_.z)) * vec3<f32>(1.0, 1.0, 1.0)));
}

fn ideal_point_ideal_point_geometric_product(self_604: IdealPoint, other_508: IdealPoint) -> Rotor {
    var self_605: IdealPoint;
    var other_509: IdealPoint;

    self_605 = self_604;
    other_509 = other_508;
    let _e4: IdealPoint = self_605;
    let _e8: IdealPoint = other_509;
    let _e11: IdealPoint = other_509;
    let _e14: IdealPoint = other_509;
    let _e17: IdealPoint = other_509;
    let _e30: IdealPoint = self_605;
    let _e34: IdealPoint = other_509;
    let _e37: IdealPoint = other_509;
    let _e40: IdealPoint = other_509;
    let _e43: IdealPoint = other_509;
    let _e57: IdealPoint = self_605;
    let _e61: IdealPoint = other_509;
    let _e64: IdealPoint = other_509;
    let _e67: IdealPoint = other_509;
    let _e70: IdealPoint = other_509;
    return Rotor(((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y, _e11.g0_.z, _e14.g0_.y, _e17.g0_.x)) * vec4<f32>(-(1.0), -(1.0), 0.0, 1.0)) + ((vec4<f32>(_e30.g0_.z) * vec4<f32>(_e34.g0_.z, _e37.g0_.y, _e40.g0_.x, _e43.g0_.z)) * vec4<f32>(-(1.0), 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e57.g0_.x) * vec4<f32>(_e61.g0_.x, _e64.g0_.x, _e67.g0_.z, _e70.g0_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0)))));
}

fn ideal_point_ideal_point_inner_product(self_606: IdealPoint, other_510: IdealPoint) -> Scalar {
    var self_607: IdealPoint;
    var other_511: IdealPoint;

    self_607 = self_606;
    other_511 = other_510;
    let _e5: IdealPoint = self_607;
    let _e8: IdealPoint = other_511;
    let _e13: IdealPoint = self_607;
    let _e16: IdealPoint = other_511;
    let _e21: IdealPoint = self_607;
    let _e24: IdealPoint = other_511;
    return Scalar((((0.0 - (_e5.g0_.x * _e8.g0_.x)) - (_e13.g0_.y * _e16.g0_.y)) - (_e21.g0_.z * _e24.g0_.z)));
}

fn ideal_point_ideal_point_left_contraction(self_608: IdealPoint, other_512: IdealPoint) -> Scalar {
    var self_609: IdealPoint;
    var other_513: IdealPoint;

    self_609 = self_608;
    other_513 = other_512;
    let _e5: IdealPoint = self_609;
    let _e8: IdealPoint = other_513;
    let _e13: IdealPoint = self_609;
    let _e16: IdealPoint = other_513;
    let _e21: IdealPoint = self_609;
    let _e24: IdealPoint = other_513;
    return Scalar((((0.0 - (_e5.g0_.x * _e8.g0_.x)) - (_e13.g0_.y * _e16.g0_.y)) - (_e21.g0_.z * _e24.g0_.z)));
}

fn ideal_point_ideal_point_right_contraction(self_610: IdealPoint, other_514: IdealPoint) -> Scalar {
    var self_611: IdealPoint;
    var other_515: IdealPoint;

    self_611 = self_610;
    other_515 = other_514;
    let _e5: IdealPoint = self_611;
    let _e8: IdealPoint = other_515;
    let _e13: IdealPoint = self_611;
    let _e16: IdealPoint = other_515;
    let _e21: IdealPoint = self_611;
    let _e24: IdealPoint = other_515;
    return Scalar((((0.0 - (_e5.g0_.x * _e8.g0_.x)) - (_e13.g0_.y * _e16.g0_.y)) - (_e21.g0_.z * _e24.g0_.z)));
}

fn ideal_point_ideal_point_scalar_product(self_612: IdealPoint, other_516: IdealPoint) -> Scalar {
    var self_613: IdealPoint;
    var other_517: IdealPoint;

    self_613 = self_612;
    other_517 = other_516;
    let _e5: IdealPoint = self_613;
    let _e8: IdealPoint = other_517;
    let _e13: IdealPoint = self_613;
    let _e16: IdealPoint = other_517;
    let _e21: IdealPoint = self_613;
    let _e24: IdealPoint = other_517;
    return Scalar((((0.0 - (_e5.g0_.x * _e8.g0_.x)) - (_e13.g0_.y * _e16.g0_.y)) - (_e21.g0_.z * _e24.g0_.z)));
}

fn ideal_point_plane_inner_product(self_614: IdealPoint, other_518: Plane) -> Plane {
    var self_615: IdealPoint;
    var other_519: Plane;

    self_615 = self_614;
    other_519 = other_518;
    let _e4: IdealPoint = self_615;
    let _e8: Plane = other_519;
    let _e19: IdealPoint = self_615;
    let _e23: Plane = other_519;
    let _e35: IdealPoint = self_615;
    let _e39: Plane = other_519;
    return Plane(((((vec4<f32>(_e4.g0_.y) * _e8.g0_.zzxz) * vec4<f32>(1.0, 0.0, -(1.0), 0.0)) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e35.g0_.x) * _e39.g0_.yxxx) * vec4<f32>(1.0, -(1.0), 0.0, 0.0))));
}

fn ideal_point_plane_right_contraction(self_616: IdealPoint, other_520: Plane) -> Plane {
    var self_617: IdealPoint;
    var other_521: Plane;

    self_617 = self_616;
    other_521 = other_520;
    let _e4: IdealPoint = self_617;
    let _e8: Plane = other_521;
    let _e19: IdealPoint = self_617;
    let _e23: Plane = other_521;
    let _e35: IdealPoint = self_617;
    let _e39: Plane = other_521;
    return Plane(((((vec4<f32>(_e4.g0_.y) * _e8.g0_.zzxz) * vec4<f32>(1.0, 0.0, -(1.0), 0.0)) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e35.g0_.x) * _e39.g0_.yxxx) * vec4<f32>(1.0, -(1.0), 0.0, 0.0))));
}

fn ideal_point_line_add(self_618: IdealPoint, other_522: Line) -> Line {
    var self_619: IdealPoint;
    var other_523: Line;

    self_619 = self_618;
    other_523 = other_522;
    let _e4: IdealPoint = self_619;
    let _e6: Line = other_523;
    let _e9: Line = other_523;
    return Line((_e4.g0_ + _e6.g0_), _e9.g1_);
}

fn ideal_point_line_sub(self_620: IdealPoint, other_524: Line) -> Line {
    var self_621: IdealPoint;
    var other_525: Line;

    self_621 = self_620;
    other_525 = other_524;
    let _e4: IdealPoint = self_621;
    let _e6: Line = other_525;
    let _e11: Line = other_525;
    return Line((_e4.g0_ - _e6.g0_), (vec3<f32>(0.0) - _e11.g1_));
}

fn ideal_point_line_geometric_product(self_622: IdealPoint, other_526: Line) -> Motor {
    var self_623: IdealPoint;
    var other_527: Line;

    self_623 = self_622;
    other_527 = other_526;
    let _e4: IdealPoint = self_623;
    let _e8: Line = other_527;
    let _e11: Line = other_527;
    let _e14: Line = other_527;
    let _e17: Line = other_527;
    let _e30: IdealPoint = self_623;
    let _e34: Line = other_527;
    let _e37: Line = other_527;
    let _e40: Line = other_527;
    let _e43: Line = other_527;
    let _e57: IdealPoint = self_623;
    let _e61: Line = other_527;
    let _e64: Line = other_527;
    let _e67: Line = other_527;
    let _e70: Line = other_527;
    let _e84: IdealPoint = self_623;
    let _e88: Line = other_527;
    let _e91: Line = other_527;
    let _e94: Line = other_527;
    let _e97: Line = other_527;
    let _e109: IdealPoint = self_623;
    let _e113: Line = other_527;
    let _e116: Line = other_527;
    let _e119: Line = other_527;
    let _e122: Line = other_527;
    let _e135: IdealPoint = self_623;
    let _e139: Line = other_527;
    let _e142: Line = other_527;
    let _e145: Line = other_527;
    let _e148: Line = other_527;
    return Motor(((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y, _e11.g0_.z, _e14.g0_.y, _e17.g0_.x)) * vec4<f32>(-(1.0), -(1.0), 0.0, 1.0)) + ((vec4<f32>(_e30.g0_.z) * vec4<f32>(_e34.g0_.z, _e37.g0_.y, _e40.g0_.x, _e43.g0_.z)) * vec4<f32>(-(1.0), 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e57.g0_.x) * vec4<f32>(_e61.g0_.x, _e64.g0_.x, _e67.g0_.z, _e70.g0_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0)))), ((((vec4<f32>(_e84.g0_.y) * vec4<f32>(_e88.g1_.y, _e91.g1_.z, _e94.g1_.y, _e97.g1_.x)) * vec4<f32>(1.0, -(1.0), 0.0, 1.0)) + ((vec4<f32>(_e109.g0_.z) * vec4<f32>(_e113.g1_.z, _e116.g1_.y, _e119.g1_.x, _e122.g1_.z)) * vec4<f32>(1.0, 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e135.g0_.x) * vec4<f32>(_e139.g1_.x, _e142.g1_.x, _e145.g1_.z, _e148.g1_.y)) * vec4<f32>(1.0, 0.0, 1.0, -(1.0)))));
}

fn ideal_point_line_regressive_product(self_624: IdealPoint, other_528: Line) -> Scalar {
    var self_625: IdealPoint;
    var other_529: Line;

    self_625 = self_624;
    other_529 = other_528;
    let _e4: IdealPoint = self_625;
    let _e7: Line = other_529;
    let _e11: IdealPoint = self_625;
    let _e14: Line = other_529;
    let _e19: IdealPoint = self_625;
    let _e22: Line = other_529;
    return Scalar((((_e4.g0_.x * _e7.g1_.x) + (_e11.g0_.y * _e14.g1_.y)) + (_e19.g0_.z * _e22.g1_.z)));
}

fn ideal_point_line_inner_product(self_626: IdealPoint, other_530: Line) -> Scalar {
    var self_627: IdealPoint;
    var other_531: Line;

    self_627 = self_626;
    other_531 = other_530;
    let _e5: IdealPoint = self_627;
    let _e8: Line = other_531;
    let _e13: IdealPoint = self_627;
    let _e16: Line = other_531;
    let _e21: IdealPoint = self_627;
    let _e24: Line = other_531;
    return Scalar((((0.0 - (_e5.g0_.x * _e8.g0_.x)) - (_e13.g0_.y * _e16.g0_.y)) - (_e21.g0_.z * _e24.g0_.z)));
}

fn ideal_point_line_left_contraction(self_628: IdealPoint, other_532: Line) -> Scalar {
    var self_629: IdealPoint;
    var other_533: Line;

    self_629 = self_628;
    other_533 = other_532;
    let _e5: IdealPoint = self_629;
    let _e8: Line = other_533;
    let _e13: IdealPoint = self_629;
    let _e16: Line = other_533;
    let _e21: IdealPoint = self_629;
    let _e24: Line = other_533;
    return Scalar((((0.0 - (_e5.g0_.x * _e8.g0_.x)) - (_e13.g0_.y * _e16.g0_.y)) - (_e21.g0_.z * _e24.g0_.z)));
}

fn ideal_point_line_right_contraction(self_630: IdealPoint, other_534: Line) -> Scalar {
    var self_631: IdealPoint;
    var other_535: Line;

    self_631 = self_630;
    other_535 = other_534;
    let _e5: IdealPoint = self_631;
    let _e8: Line = other_535;
    let _e13: IdealPoint = self_631;
    let _e16: Line = other_535;
    let _e21: IdealPoint = self_631;
    let _e24: Line = other_535;
    return Scalar((((0.0 - (_e5.g0_.x * _e8.g0_.x)) - (_e13.g0_.y * _e16.g0_.y)) - (_e21.g0_.z * _e24.g0_.z)));
}

fn ideal_point_line_scalar_product(self_632: IdealPoint, other_536: Line) -> Scalar {
    var self_633: IdealPoint;
    var other_537: Line;

    self_633 = self_632;
    other_537 = other_536;
    let _e5: IdealPoint = self_633;
    let _e8: Line = other_537;
    let _e13: IdealPoint = self_633;
    let _e16: Line = other_537;
    let _e21: IdealPoint = self_633;
    let _e24: Line = other_537;
    return Scalar((((0.0 - (_e5.g0_.x * _e8.g0_.x)) - (_e13.g0_.y * _e16.g0_.y)) - (_e21.g0_.z * _e24.g0_.z)));
}

fn ideal_point_translator_add(self_634: IdealPoint, other_538: Translator) -> Translator {
    var self_635: IdealPoint;
    var other_539: Translator;

    self_635 = self_634;
    other_539 = other_538;
    let _e4: IdealPoint = self_635;
    let _e7: IdealPoint = self_635;
    let _e10: IdealPoint = self_635;
    let _e13: IdealPoint = self_635;
    let _e23: Translator = other_539;
    return Translator(((vec4<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.y, _e13.g0_.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0)) + _e23.g0_));
}

fn ideal_point_translator_sub(self_636: IdealPoint, other_540: Translator) -> Translator {
    var self_637: IdealPoint;
    var other_541: Translator;

    self_637 = self_636;
    other_541 = other_540;
    let _e4: IdealPoint = self_637;
    let _e7: IdealPoint = self_637;
    let _e10: IdealPoint = self_637;
    let _e13: IdealPoint = self_637;
    let _e23: Translator = other_541;
    return Translator(((vec4<f32>(_e4.g0_.x, _e7.g0_.x, _e10.g0_.y, _e13.g0_.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0)) - _e23.g0_));
}

fn ideal_point_translator_outer_product(self_638: IdealPoint, other_542: Translator) -> IdealPoint {
    var self_639: IdealPoint;
    var other_543: Translator;

    self_639 = self_638;
    other_543 = other_542;
    let _e4: IdealPoint = self_639;
    let _e6: Translator = other_543;
    return IdealPoint((_e4.g0_ * vec3<f32>(_e6.g0_.x)));
}

fn ideal_point_translator_inner_product(self_640: IdealPoint, other_544: Translator) -> Translator {
    var self_641: IdealPoint;
    var other_545: Translator;

    self_641 = self_640;
    other_545 = other_544;
    let _e4: IdealPoint = self_641;
    let _e8: Translator = other_545;
    let _e19: IdealPoint = self_641;
    let _e23: Translator = other_545;
    let _e35: IdealPoint = self_641;
    let _e39: Translator = other_545;
    return Translator(((((vec4<f32>(_e4.g0_.y) * _e8.g0_.zzxz) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0)) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.wwwx) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))) + ((vec4<f32>(_e35.g0_.x) * _e39.g0_.yxxx) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))));
}

fn ideal_point_translator_left_contraction(self_642: IdealPoint, other_546: Translator) -> Scalar {
    var self_643: IdealPoint;
    var other_547: Translator;

    self_643 = self_642;
    other_547 = other_546;
    let _e5: IdealPoint = self_643;
    let _e8: Translator = other_547;
    let _e13: IdealPoint = self_643;
    let _e16: Translator = other_547;
    let _e21: IdealPoint = self_643;
    let _e24: Translator = other_547;
    return Scalar((((0.0 - (_e5.g0_.x * _e8.g0_.y)) - (_e13.g0_.y * _e16.g0_.z)) - (_e21.g0_.z * _e24.g0_.w)));
}

fn ideal_point_translator_right_contraction(self_644: IdealPoint, other_548: Translator) -> Translator {
    var self_645: IdealPoint;
    var other_549: Translator;

    self_645 = self_644;
    other_549 = other_548;
    let _e4: IdealPoint = self_645;
    let _e8: Translator = other_549;
    let _e19: IdealPoint = self_645;
    let _e23: Translator = other_549;
    let _e35: IdealPoint = self_645;
    let _e39: Translator = other_549;
    return Translator(((((vec4<f32>(_e4.g0_.y) * _e8.g0_.zzxz) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0)) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.wwwx) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))) + ((vec4<f32>(_e35.g0_.x) * _e39.g0_.yxxx) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))));
}

fn ideal_point_translator_scalar_product(self_646: IdealPoint, other_550: Translator) -> Scalar {
    var self_647: IdealPoint;
    var other_551: Translator;

    self_647 = self_646;
    other_551 = other_550;
    let _e5: IdealPoint = self_647;
    let _e8: Translator = other_551;
    let _e13: IdealPoint = self_647;
    let _e16: Translator = other_551;
    let _e21: IdealPoint = self_647;
    let _e24: Translator = other_551;
    return Scalar((((0.0 - (_e5.g0_.x * _e8.g0_.y)) - (_e13.g0_.y * _e16.g0_.z)) - (_e21.g0_.z * _e24.g0_.w)));
}

fn ideal_point_motor_add(self_648: IdealPoint, other_552: Motor) -> Motor {
    var self_649: IdealPoint;
    var other_553: Motor;

    self_649 = self_648;
    other_553 = other_552;
    let _e4: Motor = other_553;
    let _e6: IdealPoint = self_649;
    let _e9: IdealPoint = self_649;
    let _e12: IdealPoint = self_649;
    let _e15: IdealPoint = self_649;
    let _e25: Motor = other_553;
    return Motor(_e4.g0_, ((vec4<f32>(_e6.g0_.x, _e9.g0_.x, _e12.g0_.y, _e15.g0_.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0)) + _e25.g1_));
}

fn ideal_point_motor_sub(self_650: IdealPoint, other_554: Motor) -> Motor {
    var self_651: IdealPoint;
    var other_555: Motor;

    self_651 = self_650;
    other_555 = other_554;
    let _e6: Motor = other_555;
    let _e9: IdealPoint = self_651;
    let _e12: IdealPoint = self_651;
    let _e15: IdealPoint = self_651;
    let _e18: IdealPoint = self_651;
    let _e28: Motor = other_555;
    return Motor((vec4<f32>(0.0) - _e6.g0_), ((vec4<f32>(_e9.g0_.x, _e12.g0_.x, _e15.g0_.y, _e18.g0_.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0)) - _e28.g1_));
}

fn ideal_point_motor_geometric_product(self_652: IdealPoint, other_556: Motor) -> Motor {
    var self_653: IdealPoint;
    var other_557: Motor;

    self_653 = self_652;
    other_557 = other_556;
    let _e4: IdealPoint = self_653;
    let _e8: Motor = other_557;
    let _e21: IdealPoint = self_653;
    let _e25: Motor = other_557;
    let _e39: IdealPoint = self_653;
    let _e43: Motor = other_557;
    let _e57: IdealPoint = self_653;
    let _e61: Motor = other_557;
    let _e72: IdealPoint = self_653;
    let _e76: Motor = other_557;
    let _e88: IdealPoint = self_653;
    let _e92: Motor = other_557;
    return Motor(((((vec4<f32>(_e4.g0_.x) * _e8.g1_.yxwz) * vec4<f32>(-(1.0), -(1.0), 1.0, -(1.0))) + ((vec4<f32>(_e21.g0_.y) * _e25.g1_.zwxy) * vec4<f32>(-(1.0), -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e39.g0_.z) * _e43.g1_.wzyx) * vec4<f32>(-(1.0), 1.0, -(1.0), -(1.0)))), ((((vec4<f32>(_e57.g0_.x) * _e61.g0_.yxwz) * vec4<f32>(1.0, 1.0, 1.0, -(1.0))) + ((vec4<f32>(_e72.g0_.y) * _e76.g0_.zwxy) * vec4<f32>(1.0, -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e88.g0_.z) * _e92.g0_.wzyx) * vec4<f32>(1.0, 1.0, -(1.0), 1.0))));
}

fn ideal_point_motor_regressive_product(self_654: IdealPoint, other_558: Motor) -> Translator {
    var self_655: IdealPoint;
    var other_559: Motor;

    self_655 = self_654;
    other_559 = other_558;
    let _e4: IdealPoint = self_655;
    let _e8: Motor = other_559;
    let _e11: Motor = other_559;
    let _e14: Motor = other_559;
    let _e17: Motor = other_559;
    let _e28: IdealPoint = self_655;
    let _e32: Motor = other_559;
    let _e35: Motor = other_559;
    let _e38: Motor = other_559;
    let _e41: Motor = other_559;
    let _e53: IdealPoint = self_655;
    let _e57: Motor = other_559;
    let _e60: Motor = other_559;
    let _e63: Motor = other_559;
    let _e66: Motor = other_559;
    return Translator(((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.z, _e11.g0_.z, _e14.g1_.x, _e17.g0_.z)) * vec4<f32>(1.0, 0.0, 1.0, 0.0)) + ((vec4<f32>(_e28.g0_.z) * vec4<f32>(_e32.g0_.w, _e35.g0_.w, _e38.g0_.w, _e41.g1_.x)) * vec4<f32>(1.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e53.g0_.x) * vec4<f32>(_e57.g0_.y, _e60.g1_.x, _e63.g0_.x, _e66.g0_.x)) * vec4<f32>(1.0, 1.0, 0.0, 0.0))));
}

fn ideal_point_motor_left_contraction(self_656: IdealPoint, other_560: Motor) -> Rotor {
    var self_657: IdealPoint;
    var other_561: Motor;

    self_657 = self_656;
    other_561 = other_560;
    let _e4: IdealPoint = self_657;
    let _e8: Motor = other_561;
    let _e20: IdealPoint = self_657;
    let _e24: Motor = other_561;
    let _e37: IdealPoint = self_657;
    let _e41: Motor = other_561;
    return Rotor(((((vec4<f32>(_e4.g0_.y) * _e8.g1_.zzxz) * vec4<f32>(-(1.0), 0.0, -(1.0), 0.0)) + ((vec4<f32>(_e20.g0_.z) * _e24.g1_.wwwx) * vec4<f32>(-(1.0), 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e37.g0_.x) * _e41.g1_.yxxx) * vec4<f32>(-(1.0), -(1.0), 0.0, 0.0))));
}

fn ideal_point_motor_right_contraction(self_658: IdealPoint, other_562: Motor) -> Translator {
    var self_659: IdealPoint;
    var other_563: Motor;

    self_659 = self_658;
    other_563 = other_562;
    let _e4: IdealPoint = self_659;
    let _e8: Motor = other_563;
    let _e11: Motor = other_563;
    let _e14: Motor = other_563;
    let _e17: Motor = other_563;
    let _e29: IdealPoint = self_659;
    let _e33: Motor = other_563;
    let _e36: Motor = other_563;
    let _e39: Motor = other_563;
    let _e42: Motor = other_563;
    let _e55: IdealPoint = self_659;
    let _e59: Motor = other_563;
    let _e62: Motor = other_563;
    let _e65: Motor = other_563;
    let _e68: Motor = other_563;
    return Translator(((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g1_.z, _e11.g1_.z, _e14.g0_.x, _e17.g1_.z)) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0)) + ((vec4<f32>(_e29.g0_.z) * vec4<f32>(_e33.g1_.w, _e36.g1_.w, _e39.g1_.w, _e42.g0_.x)) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))) + ((vec4<f32>(_e55.g0_.x) * vec4<f32>(_e59.g1_.y, _e62.g0_.x, _e65.g1_.x, _e68.g1_.x)) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))));
}

fn ideal_point_motor_scalar_product(self_660: IdealPoint, other_564: Motor) -> Scalar {
    var self_661: IdealPoint;
    var other_565: Motor;

    self_661 = self_660;
    other_565 = other_564;
    let _e5: IdealPoint = self_661;
    let _e8: Motor = other_565;
    let _e13: IdealPoint = self_661;
    let _e16: Motor = other_565;
    let _e21: IdealPoint = self_661;
    let _e24: Motor = other_565;
    return Scalar((((0.0 - (_e5.g0_.x * _e8.g1_.y)) - (_e13.g0_.y * _e16.g1_.z)) - (_e21.g0_.z * _e24.g1_.w)));
}

fn ideal_point_point_and_plane_geometric_product(self_662: IdealPoint, other_566: PointAndPlane) -> PointAndPlane {
    var self_663: IdealPoint;
    var other_567: PointAndPlane;

    self_663 = self_662;
    other_567 = other_566;
    let _e4: IdealPoint = self_663;
    let _e8: PointAndPlane = other_567;
    let _e11: PointAndPlane = other_567;
    let _e14: PointAndPlane = other_567;
    let _e17: PointAndPlane = other_567;
    let _e30: IdealPoint = self_663;
    let _e34: PointAndPlane = other_567;
    let _e37: PointAndPlane = other_567;
    let _e40: PointAndPlane = other_567;
    let _e43: PointAndPlane = other_567;
    let _e57: IdealPoint = self_663;
    let _e61: PointAndPlane = other_567;
    let _e64: PointAndPlane = other_567;
    let _e67: PointAndPlane = other_567;
    let _e70: PointAndPlane = other_567;
    let _e84: IdealPoint = self_663;
    let _e88: PointAndPlane = other_567;
    let _e91: PointAndPlane = other_567;
    let _e94: PointAndPlane = other_567;
    let _e97: PointAndPlane = other_567;
    let _e110: IdealPoint = self_663;
    let _e114: PointAndPlane = other_567;
    let _e117: PointAndPlane = other_567;
    let _e120: PointAndPlane = other_567;
    let _e123: PointAndPlane = other_567;
    let _e137: IdealPoint = self_663;
    let _e141: PointAndPlane = other_567;
    let _e144: PointAndPlane = other_567;
    let _e147: PointAndPlane = other_567;
    let _e150: PointAndPlane = other_567;
    return PointAndPlane(((((vec4<f32>(_e4.g0_.x) * vec4<f32>(_e8.g0_.y, _e11.g0_.x, _e14.g1_.w, _e17.g1_.z)) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0))) + ((vec4<f32>(_e30.g0_.y) * vec4<f32>(_e34.g0_.z, _e37.g1_.w, _e40.g0_.x, _e43.g1_.y)) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e57.g0_.z) * vec4<f32>(_e61.g0_.w, _e64.g1_.z, _e67.g1_.y, _e70.g0_.x)) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))), ((((vec4<f32>(_e84.g0_.x) * vec4<f32>(_e88.g1_.y, _e91.g1_.x, _e94.g0_.w, _e97.g0_.z)) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0))) + ((vec4<f32>(_e110.g0_.y) * vec4<f32>(_e114.g1_.z, _e117.g0_.w, _e120.g1_.x, _e123.g0_.y)) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e137.g0_.z) * vec4<f32>(_e141.g1_.w, _e144.g0_.z, _e147.g0_.y, _e150.g1_.x)) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))));
}

fn ideal_point_point_and_plane_regressive_product(self_664: IdealPoint, other_568: PointAndPlane) -> Plane {
    var self_665: IdealPoint;
    var other_569: PointAndPlane;

    self_665 = self_664;
    other_569 = other_568;
    let _e4: IdealPoint = self_665;
    let _e8: PointAndPlane = other_569;
    let _e19: IdealPoint = self_665;
    let _e23: PointAndPlane = other_569;
    let _e35: IdealPoint = self_665;
    let _e39: PointAndPlane = other_569;
    return Plane(((((vec4<f32>(_e4.g0_.y) * _e8.g0_.zzxz) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0)) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.wwwx) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))) + ((vec4<f32>(_e35.g0_.x) * _e39.g0_.yxxx) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))));
}

fn ideal_point_point_and_plane_inner_product(self_666: IdealPoint, other_570: PointAndPlane) -> Plane {
    var self_667: IdealPoint;
    var other_571: PointAndPlane;

    self_667 = self_666;
    other_571 = other_570;
    let _e4: IdealPoint = self_667;
    let _e8: PointAndPlane = other_571;
    let _e11: PointAndPlane = other_571;
    let _e14: PointAndPlane = other_571;
    let _e17: PointAndPlane = other_571;
    let _e30: IdealPoint = self_667;
    let _e34: PointAndPlane = other_571;
    let _e37: PointAndPlane = other_571;
    let _e40: PointAndPlane = other_571;
    let _e43: PointAndPlane = other_571;
    let _e57: IdealPoint = self_667;
    let _e61: PointAndPlane = other_571;
    let _e64: PointAndPlane = other_571;
    let _e67: PointAndPlane = other_571;
    let _e70: PointAndPlane = other_571;
    return Plane(((((vec4<f32>(_e4.g0_.x) * vec4<f32>(_e8.g1_.y, _e11.g1_.x, _e14.g0_.w, _e17.g0_.z)) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0))) + ((vec4<f32>(_e30.g0_.y) * vec4<f32>(_e34.g1_.z, _e37.g0_.w, _e40.g1_.x, _e43.g0_.y)) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e57.g0_.z) * vec4<f32>(_e61.g1_.w, _e64.g0_.z, _e67.g0_.y, _e70.g1_.x)) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))));
}

fn ideal_point_point_and_plane_right_contraction(self_668: IdealPoint, other_572: PointAndPlane) -> Plane {
    var self_669: IdealPoint;
    var other_573: PointAndPlane;

    self_669 = self_668;
    other_573 = other_572;
    let _e4: IdealPoint = self_669;
    let _e8: PointAndPlane = other_573;
    let _e19: IdealPoint = self_669;
    let _e23: PointAndPlane = other_573;
    let _e35: IdealPoint = self_669;
    let _e39: PointAndPlane = other_573;
    return Plane(((((vec4<f32>(_e4.g0_.y) * _e8.g1_.zzxz) * vec4<f32>(1.0, 0.0, -(1.0), 0.0)) + ((vec4<f32>(_e19.g0_.z) * _e23.g1_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e35.g0_.x) * _e39.g1_.yxxx) * vec4<f32>(1.0, -(1.0), 0.0, 0.0))));
}

fn ideal_point_squared_magnitude(self_670: IdealPoint) -> Scalar {
    var self_671: IdealPoint;

    self_671 = self_670;
    let _e4: IdealPoint = self_671;
    let _e5: IdealPoint = ideal_point_reversal(_e4);
    let _e6: IdealPoint = self_671;
    let _e8: IdealPoint = self_671;
    let _e9: IdealPoint = ideal_point_reversal(_e8);
    let _e10: Scalar = ideal_point_ideal_point_scalar_product(_e6, _e9);
    return _e10;
}

fn ideal_point_magnitude(self_672: IdealPoint) -> Scalar {
    var self_673: IdealPoint;

    self_673 = self_672;
    let _e3: IdealPoint = self_673;
    let _e4: Scalar = ideal_point_squared_magnitude(_e3);
    let _e7: IdealPoint = self_673;
    let _e8: Scalar = ideal_point_squared_magnitude(_e7);
    return Scalar(sqrt(_e8.g0_));
}

fn ideal_point_scale(self_674: IdealPoint, other_574: f32) -> IdealPoint {
    var self_675: IdealPoint;
    var other_575: f32;

    self_675 = self_674;
    other_575 = other_574;
    let _e5: f32 = other_575;
    let _e7: IdealPoint = self_675;
    let _e8: f32 = other_575;
    let _e10: IdealPoint = ideal_point_scalar_geometric_product(_e7, Scalar(_e8));
    return _e10;
}

fn ideal_point_signum(self_676: IdealPoint) -> IdealPoint {
    var self_677: IdealPoint;

    self_677 = self_676;
    let _e5: IdealPoint = self_677;
    let _e6: Scalar = ideal_point_magnitude(_e5);
    let _e10: IdealPoint = self_677;
    let _e13: IdealPoint = self_677;
    let _e14: Scalar = ideal_point_magnitude(_e13);
    let _e18: IdealPoint = ideal_point_scalar_geometric_product(_e10, Scalar((1.0 / _e14.g0_)));
    return _e18;
}

fn ideal_point_inverse(self_678: IdealPoint) -> IdealPoint {
    var self_679: IdealPoint;

    self_679 = self_678;
    let _e3: IdealPoint = self_679;
    let _e4: IdealPoint = ideal_point_reversal(_e3);
    let _e7: IdealPoint = self_679;
    let _e8: Scalar = ideal_point_squared_magnitude(_e7);
    let _e13: IdealPoint = self_679;
    let _e14: IdealPoint = ideal_point_reversal(_e13);
    let _e17: IdealPoint = self_679;
    let _e18: Scalar = ideal_point_squared_magnitude(_e17);
    let _e22: IdealPoint = ideal_point_scalar_geometric_product(_e14, Scalar((1.0 / _e18.g0_)));
    return _e22;
}

fn plane_zero() -> Plane {
    return Plane(vec4<f32>(0.0));
}

fn plane_one() -> Plane {
    return Plane(vec4<f32>(0.0));
}

fn plane_neg(self_680: Plane) -> Plane {
    var self_681: Plane;

    self_681 = self_680;
    let _e2: Plane = self_681;
    return Plane((_e2.g0_ * vec4<f32>(-(1.0))));
}

fn plane_automorphism(self_682: Plane) -> Plane {
    var self_683: Plane;

    self_683 = self_682;
    let _e2: Plane = self_683;
    return Plane((_e2.g0_ * vec4<f32>(-(1.0))));
}

fn plane_reversal(self_684: Plane) -> Plane {
    var self_685: Plane;

    self_685 = self_684;
    let _e2: Plane = self_685;
    return Plane(_e2.g0_);
}

fn plane_conjugation(self_686: Plane) -> Plane {
    var self_687: Plane;

    self_687 = self_686;
    let _e2: Plane = self_687;
    return Plane((_e2.g0_ * vec4<f32>(-(1.0))));
}

fn plane_dual(self_688: Plane) -> Point {
    var self_689: Plane;

    self_689 = self_688;
    let _e2: Plane = self_689;
    return Point(_e2.g0_);
}

fn plane_scalar_geometric_product(self_690: Plane, other_576: Scalar) -> Plane {
    var self_691: Plane;
    var other_577: Scalar;

    self_691 = self_690;
    other_577 = other_576;
    let _e4: Plane = self_691;
    let _e6: Scalar = other_577;
    return Plane((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn plane_scalar_outer_product(self_692: Plane, other_578: Scalar) -> Plane {
    var self_693: Plane;
    var other_579: Scalar;

    self_693 = self_692;
    other_579 = other_578;
    let _e4: Plane = self_693;
    let _e6: Scalar = other_579;
    return Plane((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn plane_scalar_inner_product(self_694: Plane, other_580: Scalar) -> Plane {
    var self_695: Plane;
    var other_581: Scalar;

    self_695 = self_694;
    other_581 = other_580;
    let _e4: Plane = self_695;
    let _e6: Scalar = other_581;
    return Plane((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn plane_scalar_right_contraction(self_696: Plane, other_582: Scalar) -> Plane {
    var self_697: Plane;
    var other_583: Scalar;

    self_697 = self_696;
    other_583 = other_582;
    let _e4: Plane = self_697;
    let _e6: Scalar = other_583;
    return Plane((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn plane_multi_vector_add(self_698: Plane, other_584: MultiVector) -> MultiVector {
    var self_699: Plane;
    var other_585: MultiVector;

    self_699 = self_698;
    other_585 = other_584;
    let _e4: MultiVector = other_585;
    let _e6: Plane = self_699;
    let _e16: MultiVector = other_585;
    let _e19: Plane = self_699;
    let _e27: MultiVector = other_585;
    let _e30: MultiVector = other_585;
    return MultiVector(_e4.g0_, ((vec4<f32>(_e6.g0_.x) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) + _e16.g1_), ((_e19.g0_ * vec4<f32>(0.0, 1.0, 1.0, 1.0)) + _e27.g2_), _e30.g3_);
}

fn plane_multi_vector_sub(self_700: Plane, other_586: MultiVector) -> MultiVector {
    var self_701: Plane;
    var other_587: MultiVector;

    self_701 = self_700;
    other_587 = other_586;
    let _e6: MultiVector = other_587;
    let _e9: Plane = self_701;
    let _e19: MultiVector = other_587;
    let _e22: Plane = self_701;
    let _e30: MultiVector = other_587;
    let _e35: MultiVector = other_587;
    return MultiVector((vec4<f32>(0.0) - _e6.g0_), ((vec4<f32>(_e9.g0_.x) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) - _e19.g1_), ((_e22.g0_ * vec4<f32>(0.0, 1.0, 1.0, 1.0)) - _e30.g2_), (vec4<f32>(0.0) - _e35.g3_));
}

fn plane_multi_vector_geometric_product(self_702: Plane, other_588: MultiVector) -> MultiVector {
    var self_703: Plane;
    var other_589: MultiVector;

    self_703 = self_702;
    other_589 = other_588;
    let _e4: Plane = self_703;
    let _e8: MultiVector = other_589;
    let _e20: Plane = self_703;
    let _e24: MultiVector = other_589;
    let _e36: Plane = self_703;
    let _e40: MultiVector = other_589;
    let _e52: Plane = self_703;
    let _e56: MultiVector = other_589;
    let _e68: Plane = self_703;
    let _e72: MultiVector = other_589;
    let _e84: Plane = self_703;
    let _e88: MultiVector = other_589;
    let _e101: Plane = self_703;
    let _e105: MultiVector = other_589;
    let _e118: Plane = self_703;
    let _e122: MultiVector = other_589;
    let _e135: Plane = self_703;
    let _e139: MultiVector = other_589;
    let _e142: Plane = self_703;
    let _e146: MultiVector = other_589;
    let _e158: Plane = self_703;
    let _e162: MultiVector = other_589;
    let _e174: Plane = self_703;
    let _e178: MultiVector = other_589;
    let _e190: Plane = self_703;
    let _e194: MultiVector = other_589;
    let _e197: Plane = self_703;
    let _e201: MultiVector = other_589;
    let _e214: Plane = self_703;
    let _e218: MultiVector = other_589;
    let _e231: Plane = self_703;
    let _e235: MultiVector = other_589;
    return MultiVector((((((vec4<f32>(_e4.g0_.x) * _e8.g1_) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))) + ((vec4<f32>(_e20.g0_.y) * _e24.g2_.yxwz) * vec4<f32>(1.0, 1.0, -(1.0), 1.0))) + ((vec4<f32>(_e36.g0_.z) * _e40.g2_.zwxy) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e52.g0_.w) * _e56.g2_.wzyx) * vec4<f32>(1.0, -(1.0), 1.0, 1.0))), (((((vec4<f32>(_e68.g0_.x) * _e72.g0_) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))) + ((vec4<f32>(_e84.g0_.y) * _e88.g3_.yxwz) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))) + ((vec4<f32>(_e101.g0_.z) * _e105.g3_.zwxy) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e118.g0_.w) * _e122.g3_.wzyx) * vec4<f32>(-(1.0), -(1.0), 1.0, 1.0))), ((((vec4<f32>(_e135.g0_.x) * _e139.g3_) + ((vec4<f32>(_e142.g0_.y) * _e146.g0_.yxwz) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e158.g0_.z) * _e162.g0_.zwxy) * vec4<f32>(1.0, -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e174.g0_.w) * _e178.g0_.wzyx) * vec4<f32>(1.0, 1.0, -(1.0), 1.0))), ((((vec4<f32>(_e190.g0_.x) * _e194.g2_) + ((vec4<f32>(_e197.g0_.y) * _e201.g1_.yxwz) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e214.g0_.z) * _e218.g1_.zwxy) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e231.g0_.w) * _e235.g1_.wzyx) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))));
}

fn plane_multi_vector_scalar_product(self_704: Plane, other_590: MultiVector) -> Scalar {
    var self_705: Plane;
    var other_591: MultiVector;

    self_705 = self_704;
    other_591 = other_590;
    let _e4: Plane = self_705;
    let _e7: MultiVector = other_591;
    let _e11: Plane = self_705;
    let _e14: MultiVector = other_591;
    let _e19: Plane = self_705;
    let _e22: MultiVector = other_591;
    let _e27: Plane = self_705;
    let _e30: MultiVector = other_591;
    return Scalar(((((_e4.g0_.x * _e7.g1_.x) + (_e11.g0_.y * _e14.g2_.y)) + (_e19.g0_.z * _e22.g2_.z)) + (_e27.g0_.w * _e30.g2_.w)));
}

fn plane_rotor_geometric_product(self_706: Plane, other_592: Rotor) -> PointAndPlane {
    var self_707: Plane;
    var other_593: Rotor;

    self_707 = self_706;
    other_593 = other_592;
    let _e4: Plane = self_707;
    let _e8: Rotor = other_593;
    let _e19: Plane = self_707;
    let _e23: Rotor = other_593;
    let _e35: Plane = self_707;
    let _e38: Rotor = other_593;
    let _e52: Plane = self_707;
    let _e56: Rotor = other_593;
    let _e67: Plane = self_707;
    let _e71: Rotor = other_593;
    let _e83: Plane = self_707;
    let _e86: Rotor = other_593;
    return PointAndPlane(((((vec4<f32>(_e4.g0_.z) * vec4<f32>(_e8.g0_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) + ((vec4<f32>(_e19.g0_.w) * vec4<f32>(_e23.g0_.w)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((_e35.g0_.yxxx * _e38.g0_.yyzw) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0)))), ((((vec4<f32>(_e52.g0_.z) * _e56.g0_.wwxy) * vec4<f32>(0.0, -(1.0), 1.0, 1.0)) + ((vec4<f32>(_e67.g0_.w) * _e71.g0_.zzyx) * vec4<f32>(0.0, 1.0, -(1.0), 1.0))) + ((_e83.g0_.xyyy * _e86.g0_.xxwz) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))));
}

fn plane_rotor_outer_product(self_708: Plane, other_594: Rotor) -> PointAndPlane {
    var self_709: Plane;
    var other_595: Rotor;

    self_709 = self_708;
    other_595 = other_594;
    let _e4: Plane = self_709;
    let _e8: Rotor = other_595;
    let _e19: Plane = self_709;
    let _e23: Rotor = other_595;
    let _e35: Plane = self_709;
    let _e38: Rotor = other_595;
    let _e52: Plane = self_709;
    let _e54: Rotor = other_595;
    return PointAndPlane(((((vec4<f32>(_e4.g0_.z) * vec4<f32>(_e8.g0_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) + ((vec4<f32>(_e19.g0_.w) * vec4<f32>(_e23.g0_.w)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((_e35.g0_.yxxx * _e38.g0_.yyzw) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0)))), (_e52.g0_ * vec4<f32>(_e54.g0_.x)));
}

fn plane_rotor_inner_product(self_710: Plane, other_596: Rotor) -> Plane {
    var self_711: Plane;
    var other_597: Rotor;

    self_711 = self_710;
    other_597 = other_596;
    let _e4: Plane = self_711;
    let _e8: Rotor = other_597;
    let _e19: Plane = self_711;
    let _e23: Rotor = other_597;
    let _e35: Plane = self_711;
    let _e38: Rotor = other_597;
    return Plane(((((vec4<f32>(_e4.g0_.z) * _e8.g0_.wwxy) * vec4<f32>(0.0, -(1.0), 1.0, 1.0)) + ((vec4<f32>(_e19.g0_.w) * _e23.g0_.zzyx) * vec4<f32>(0.0, 1.0, -(1.0), 1.0))) + ((_e35.g0_.xyyy * _e38.g0_.xxwz) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))));
}

fn plane_rotor_right_contraction(self_712: Plane, other_598: Rotor) -> Plane {
    var self_713: Plane;
    var other_599: Rotor;

    self_713 = self_712;
    other_599 = other_598;
    let _e4: Plane = self_713;
    let _e6: Rotor = other_599;
    return Plane((_e4.g0_ * vec4<f32>(_e6.g0_.x)));
}

fn plane_point_add(self_714: Plane, other_600: Point) -> PointAndPlane {
    var self_715: Plane;
    var other_601: Point;

    self_715 = self_714;
    other_601 = other_600;
    let _e4: Point = other_601;
    let _e6: Plane = self_715;
    return PointAndPlane(_e4.g0_, _e6.g0_);
}

fn plane_point_sub(self_716: Plane, other_602: Point) -> PointAndPlane {
    var self_717: Plane;
    var other_603: Point;

    self_717 = self_716;
    other_603 = other_602;
    let _e6: Point = other_603;
    let _e9: Plane = self_717;
    return PointAndPlane((vec4<f32>(0.0) - _e6.g0_), _e9.g0_);
}

fn plane_point_regressive_product(self_718: Plane, other_604: Point) -> Scalar {
    var self_719: Plane;
    var other_605: Point;

    self_719 = self_718;
    other_605 = other_604;
    let _e4: Plane = self_719;
    let _e7: Point = other_605;
    let _e11: Plane = self_719;
    let _e14: Point = other_605;
    let _e19: Plane = self_719;
    let _e22: Point = other_605;
    let _e27: Plane = self_719;
    let _e30: Point = other_605;
    return Scalar(((((_e4.g0_.x * _e7.g0_.x) + (_e11.g0_.y * _e14.g0_.y)) + (_e19.g0_.z * _e22.g0_.z)) + (_e27.g0_.w * _e30.g0_.w)));
}

fn plane_point_inner_product(self_720: Plane, other_606: Point) -> Line {
    var self_721: Plane;
    var other_607: Point;

    self_721 = self_720;
    other_607 = other_606;
    let _e4: Plane = self_721;
    let _e8: Point = other_607;
    let _e11: Point = other_607;
    let _e14: Point = other_607;
    let _e25: Plane = self_721;
    let _e29: Point = other_607;
    let _e32: Point = other_607;
    let _e35: Point = other_607;
    let _e47: Plane = self_721;
    let _e50: Plane = self_721;
    let _e53: Plane = self_721;
    let _e57: Point = other_607;
    let _e60: Point = other_607;
    let _e63: Point = other_607;
    let _e77: Plane = self_721;
    let _e81: Point = other_607;
    let _e84: Point = other_607;
    let _e87: Point = other_607;
    let _e93: Plane = self_721;
    let _e96: Plane = self_721;
    let _e99: Plane = self_721;
    let _e103: Point = other_607;
    return Line(((((vec3<f32>(_e4.g0_.z) * vec3<f32>(_e8.g0_.w, _e11.g0_.w, _e14.g0_.y)) * vec3<f32>(-(1.0), 0.0, 1.0)) + ((vec3<f32>(_e25.g0_.w) * vec3<f32>(_e29.g0_.z, _e32.g0_.y, _e35.g0_.z)) * vec3<f32>(1.0, -(1.0), 0.0))) + ((vec3<f32>(_e47.g0_.x, _e50.g0_.y, _e53.g0_.y) * vec3<f32>(_e57.g0_.x, _e60.g0_.w, _e63.g0_.z)) * vec3<f32>(0.0, 1.0, -(1.0)))), ((vec3<f32>(0.0) - (vec3<f32>(_e77.g0_.x) * vec3<f32>(_e81.g0_.y, _e84.g0_.z, _e87.g0_.w))) + (vec3<f32>(_e93.g0_.y, _e96.g0_.z, _e99.g0_.w) * vec3<f32>(_e103.g0_.x))));
}

fn plane_point_left_contraction(self_722: Plane, other_608: Point) -> Line {
    var self_723: Plane;
    var other_609: Point;

    self_723 = self_722;
    other_609 = other_608;
    let _e4: Plane = self_723;
    let _e8: Point = other_609;
    let _e11: Point = other_609;
    let _e14: Point = other_609;
    let _e25: Plane = self_723;
    let _e29: Point = other_609;
    let _e32: Point = other_609;
    let _e35: Point = other_609;
    let _e47: Plane = self_723;
    let _e50: Plane = self_723;
    let _e53: Plane = self_723;
    let _e57: Point = other_609;
    let _e60: Point = other_609;
    let _e63: Point = other_609;
    let _e77: Plane = self_723;
    let _e81: Point = other_609;
    let _e84: Point = other_609;
    let _e87: Point = other_609;
    let _e93: Plane = self_723;
    let _e96: Plane = self_723;
    let _e99: Plane = self_723;
    let _e103: Point = other_609;
    return Line(((((vec3<f32>(_e4.g0_.z) * vec3<f32>(_e8.g0_.w, _e11.g0_.w, _e14.g0_.y)) * vec3<f32>(-(1.0), 0.0, 1.0)) + ((vec3<f32>(_e25.g0_.w) * vec3<f32>(_e29.g0_.z, _e32.g0_.y, _e35.g0_.z)) * vec3<f32>(1.0, -(1.0), 0.0))) + ((vec3<f32>(_e47.g0_.x, _e50.g0_.y, _e53.g0_.y) * vec3<f32>(_e57.g0_.x, _e60.g0_.w, _e63.g0_.z)) * vec3<f32>(0.0, 1.0, -(1.0)))), ((vec3<f32>(0.0) - (vec3<f32>(_e77.g0_.x) * vec3<f32>(_e81.g0_.y, _e84.g0_.z, _e87.g0_.w))) + (vec3<f32>(_e93.g0_.y, _e96.g0_.z, _e99.g0_.w) * vec3<f32>(_e103.g0_.x))));
}

fn plane_ideal_point_inner_product(self_724: Plane, other_610: IdealPoint) -> Plane {
    var self_725: Plane;
    var other_611: IdealPoint;

    self_725 = self_724;
    other_611 = other_610;
    let _e4: Plane = self_725;
    let _e8: IdealPoint = other_611;
    let _e20: Plane = self_725;
    let _e24: IdealPoint = other_611;
    let _e37: Plane = self_725;
    let _e40: IdealPoint = other_611;
    let _e43: IdealPoint = other_611;
    let _e46: IdealPoint = other_611;
    let _e49: IdealPoint = other_611;
    return Plane(((((vec4<f32>(_e4.g0_.z) * vec4<f32>(_e8.g0_.y)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0)) + ((vec4<f32>(_e20.g0_.w) * vec4<f32>(_e24.g0_.z)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((_e37.g0_.yxxx * vec4<f32>(_e40.g0_.x, _e43.g0_.x, _e46.g0_.y, _e49.g0_.z)) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0))));
}

fn plane_ideal_point_left_contraction(self_726: Plane, other_612: IdealPoint) -> Plane {
    var self_727: Plane;
    var other_613: IdealPoint;

    self_727 = self_726;
    other_613 = other_612;
    let _e4: Plane = self_727;
    let _e8: IdealPoint = other_613;
    let _e20: Plane = self_727;
    let _e24: IdealPoint = other_613;
    let _e37: Plane = self_727;
    let _e40: IdealPoint = other_613;
    let _e43: IdealPoint = other_613;
    let _e46: IdealPoint = other_613;
    let _e49: IdealPoint = other_613;
    return Plane(((((vec4<f32>(_e4.g0_.z) * vec4<f32>(_e8.g0_.y)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0)) + ((vec4<f32>(_e20.g0_.w) * vec4<f32>(_e24.g0_.z)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((_e37.g0_.yxxx * vec4<f32>(_e40.g0_.x, _e43.g0_.x, _e46.g0_.y, _e49.g0_.z)) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0))));
}

fn plane_plane_add(self_728: Plane, other_614: Plane) -> Plane {
    var self_729: Plane;
    var other_615: Plane;

    self_729 = self_728;
    other_615 = other_614;
    let _e4: Plane = self_729;
    let _e6: Plane = other_615;
    return Plane((_e4.g0_ + _e6.g0_));
}

fn plane_plane_sub(self_730: Plane, other_616: Plane) -> Plane {
    var self_731: Plane;
    var other_617: Plane;

    self_731 = self_730;
    other_617 = other_616;
    let _e4: Plane = self_731;
    let _e6: Plane = other_617;
    return Plane((_e4.g0_ - _e6.g0_));
}

fn plane_plane_mul(self_732: Plane, other_618: Plane) -> Plane {
    var self_733: Plane;
    var other_619: Plane;

    self_733 = self_732;
    other_619 = other_618;
    let _e4: Plane = self_733;
    let _e6: Plane = other_619;
    return Plane((_e4.g0_ * _e6.g0_));
}

fn plane_plane_div(self_734: Plane, other_620: Plane) -> Plane {
    var self_735: Plane;
    var other_621: Plane;

    self_735 = self_734;
    other_621 = other_620;
    let _e4: Plane = self_735;
    let _e7: Plane = self_735;
    let _e10: Plane = self_735;
    let _e13: Plane = self_735;
    let _e23: Plane = other_621;
    let _e26: Plane = other_621;
    let _e29: Plane = other_621;
    let _e32: Plane = other_621;
    return Plane((((vec4<f32>(_e4.g0_.x, _e7.g0_.y, _e10.g0_.z, _e13.g0_.w) * vec4<f32>(1.0, 1.0, 1.0, 1.0)) / vec4<f32>(_e23.g0_.x, _e26.g0_.y, _e29.g0_.z, _e32.g0_.w)) * vec4<f32>(1.0, 1.0, 1.0, 1.0)));
}

fn plane_plane_outer_product(self_736: Plane, other_622: Plane) -> Line {
    var self_737: Plane;
    var other_623: Plane;

    self_737 = self_736;
    other_623 = other_622;
    let _e4: Plane = self_737;
    let _e8: Plane = other_623;
    let _e11: Plane = other_623;
    let _e14: Plane = other_623;
    let _e19: Plane = self_737;
    let _e22: Plane = self_737;
    let _e25: Plane = self_737;
    let _e29: Plane = other_623;
    let _e39: Plane = self_737;
    let _e43: Plane = other_623;
    let _e46: Plane = other_623;
    let _e49: Plane = other_623;
    let _e60: Plane = self_737;
    let _e64: Plane = other_623;
    let _e67: Plane = other_623;
    let _e70: Plane = other_623;
    let _e82: Plane = self_737;
    let _e85: Plane = self_737;
    let _e88: Plane = self_737;
    let _e92: Plane = other_623;
    let _e95: Plane = other_623;
    let _e98: Plane = other_623;
    return Line(((vec3<f32>(_e4.g0_.x) * vec3<f32>(_e8.g0_.y, _e11.g0_.z, _e14.g0_.w)) + ((vec3<f32>(_e19.g0_.y, _e22.g0_.z, _e25.g0_.w) * vec3<f32>(_e29.g0_.x)) * vec3<f32>(-(1.0)))), ((((vec3<f32>(_e39.g0_.z) * vec3<f32>(_e43.g0_.w, _e46.g0_.w, _e49.g0_.y)) * vec3<f32>(1.0, 0.0, -(1.0))) + ((vec3<f32>(_e60.g0_.w) * vec3<f32>(_e64.g0_.z, _e67.g0_.y, _e70.g0_.z)) * vec3<f32>(-(1.0), 1.0, 0.0))) + ((vec3<f32>(_e82.g0_.x, _e85.g0_.y, _e88.g0_.y) * vec3<f32>(_e92.g0_.x, _e95.g0_.w, _e98.g0_.z)) * vec3<f32>(0.0, -(1.0), 1.0))));
}

fn plane_plane_inner_product(self_738: Plane, other_624: Plane) -> Scalar {
    var self_739: Plane;
    var other_625: Plane;

    self_739 = self_738;
    other_625 = other_624;
    let _e4: Plane = self_739;
    let _e7: Plane = other_625;
    let _e11: Plane = self_739;
    let _e14: Plane = other_625;
    let _e19: Plane = self_739;
    let _e22: Plane = other_625;
    let _e27: Plane = self_739;
    let _e30: Plane = other_625;
    return Scalar(((((_e4.g0_.x * _e7.g0_.x) + (_e11.g0_.y * _e14.g0_.y)) + (_e19.g0_.z * _e22.g0_.z)) + (_e27.g0_.w * _e30.g0_.w)));
}

fn plane_plane_left_contraction(self_740: Plane, other_626: Plane) -> Scalar {
    var self_741: Plane;
    var other_627: Plane;

    self_741 = self_740;
    other_627 = other_626;
    let _e4: Plane = self_741;
    let _e7: Plane = other_627;
    let _e11: Plane = self_741;
    let _e14: Plane = other_627;
    let _e19: Plane = self_741;
    let _e22: Plane = other_627;
    let _e27: Plane = self_741;
    let _e30: Plane = other_627;
    return Scalar(((((_e4.g0_.x * _e7.g0_.x) + (_e11.g0_.y * _e14.g0_.y)) + (_e19.g0_.z * _e22.g0_.z)) + (_e27.g0_.w * _e30.g0_.w)));
}

fn plane_plane_right_contraction(self_742: Plane, other_628: Plane) -> Scalar {
    var self_743: Plane;
    var other_629: Plane;

    self_743 = self_742;
    other_629 = other_628;
    let _e4: Plane = self_743;
    let _e7: Plane = other_629;
    let _e11: Plane = self_743;
    let _e14: Plane = other_629;
    let _e19: Plane = self_743;
    let _e22: Plane = other_629;
    let _e27: Plane = self_743;
    let _e30: Plane = other_629;
    return Scalar(((((_e4.g0_.x * _e7.g0_.x) + (_e11.g0_.y * _e14.g0_.y)) + (_e19.g0_.z * _e22.g0_.z)) + (_e27.g0_.w * _e30.g0_.w)));
}

fn plane_plane_scalar_product(self_744: Plane, other_630: Plane) -> Scalar {
    var self_745: Plane;
    var other_631: Plane;

    self_745 = self_744;
    other_631 = other_630;
    let _e4: Plane = self_745;
    let _e7: Plane = other_631;
    let _e11: Plane = self_745;
    let _e14: Plane = other_631;
    let _e19: Plane = self_745;
    let _e22: Plane = other_631;
    let _e27: Plane = self_745;
    let _e30: Plane = other_631;
    return Scalar(((((_e4.g0_.x * _e7.g0_.x) + (_e11.g0_.y * _e14.g0_.y)) + (_e19.g0_.z * _e22.g0_.z)) + (_e27.g0_.w * _e30.g0_.w)));
}

fn plane_line_geometric_product(self_746: Plane, other_632: Line) -> PointAndPlane {
    var self_747: Plane;
    var other_633: Line;

    self_747 = self_746;
    other_633 = other_632;
    let _e4: Plane = self_747;
    let _e8: Line = other_633;
    let _e11: Line = other_633;
    let _e14: Line = other_633;
    let _e17: Line = other_633;
    let _e29: Plane = self_747;
    let _e33: Line = other_633;
    let _e36: Line = other_633;
    let _e39: Line = other_633;
    let _e42: Line = other_633;
    let _e55: Plane = self_747;
    let _e59: Line = other_633;
    let _e62: Line = other_633;
    let _e65: Line = other_633;
    let _e68: Line = other_633;
    let _e81: Plane = self_747;
    let _e85: Line = other_633;
    let _e88: Line = other_633;
    let _e91: Line = other_633;
    let _e94: Line = other_633;
    let _e109: Plane = self_747;
    let _e113: Line = other_633;
    let _e116: Line = other_633;
    let _e119: Line = other_633;
    let _e122: Line = other_633;
    let _e135: Plane = self_747;
    let _e139: Line = other_633;
    let _e142: Line = other_633;
    let _e145: Line = other_633;
    let _e148: Line = other_633;
    let _e162: Plane = self_747;
    let _e166: Line = other_633;
    let _e169: Line = other_633;
    let _e172: Line = other_633;
    let _e175: Line = other_633;
    let _e189: Plane = self_747;
    let _e193: Line = other_633;
    let _e196: Line = other_633;
    let _e199: Line = other_633;
    let _e202: Line = other_633;
    return PointAndPlane((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g1_.x, _e11.g1_.x, _e14.g0_.z, _e17.g0_.y)) * vec4<f32>(1.0, 0.0, -(1.0), 1.0)) + ((vec4<f32>(_e29.g0_.z) * vec4<f32>(_e33.g1_.y, _e36.g0_.z, _e39.g1_.y, _e42.g0_.x)) * vec4<f32>(1.0, 1.0, 0.0, -(1.0)))) + ((vec4<f32>(_e55.g0_.w) * vec4<f32>(_e59.g1_.z, _e62.g0_.y, _e65.g0_.x, _e68.g1_.z)) * vec4<f32>(1.0, -(1.0), 1.0, 0.0))) + ((vec4<f32>(_e81.g0_.x) * vec4<f32>(_e85.g1_.x, _e88.g1_.x, _e91.g1_.y, _e94.g1_.z)) * vec4<f32>(0.0, -(1.0), -(1.0), -(1.0)))), (((((vec4<f32>(_e109.g0_.y) * vec4<f32>(_e113.g0_.x, _e116.g0_.x, _e119.g1_.z, _e122.g1_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e135.g0_.z) * vec4<f32>(_e139.g0_.y, _e142.g1_.z, _e145.g0_.y, _e148.g1_.x)) * vec4<f32>(-(1.0), -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e162.g0_.w) * vec4<f32>(_e166.g0_.z, _e169.g1_.y, _e172.g1_.x, _e175.g0_.z)) * vec4<f32>(-(1.0), 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e189.g0_.x) * vec4<f32>(_e193.g0_.x, _e196.g0_.x, _e199.g0_.y, _e202.g0_.z)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn plane_line_outer_product(self_748: Plane, other_634: Line) -> Point {
    var self_749: Plane;
    var other_635: Line;

    self_749 = self_748;
    other_635 = other_634;
    let _e4: Plane = self_749;
    let _e8: Line = other_635;
    let _e11: Line = other_635;
    let _e14: Line = other_635;
    let _e17: Line = other_635;
    let _e29: Plane = self_749;
    let _e33: Line = other_635;
    let _e36: Line = other_635;
    let _e39: Line = other_635;
    let _e42: Line = other_635;
    let _e55: Plane = self_749;
    let _e59: Line = other_635;
    let _e62: Line = other_635;
    let _e65: Line = other_635;
    let _e68: Line = other_635;
    let _e81: Plane = self_749;
    let _e85: Line = other_635;
    let _e88: Line = other_635;
    let _e91: Line = other_635;
    let _e94: Line = other_635;
    return Point((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g1_.x, _e11.g1_.x, _e14.g0_.z, _e17.g0_.y)) * vec4<f32>(1.0, 0.0, -(1.0), 1.0)) + ((vec4<f32>(_e29.g0_.z) * vec4<f32>(_e33.g1_.y, _e36.g0_.z, _e39.g1_.y, _e42.g0_.x)) * vec4<f32>(1.0, 1.0, 0.0, -(1.0)))) + ((vec4<f32>(_e55.g0_.w) * vec4<f32>(_e59.g1_.z, _e62.g0_.y, _e65.g0_.x, _e68.g1_.z)) * vec4<f32>(1.0, -(1.0), 1.0, 0.0))) + ((vec4<f32>(_e81.g0_.x) * vec4<f32>(_e85.g1_.x, _e88.g1_.x, _e91.g1_.y, _e94.g1_.z)) * vec4<f32>(0.0, -(1.0), -(1.0), -(1.0)))));
}

fn plane_line_inner_product(self_750: Plane, other_636: Line) -> Plane {
    var self_751: Plane;
    var other_637: Line;

    self_751 = self_750;
    other_637 = other_636;
    let _e4: Plane = self_751;
    let _e8: Line = other_637;
    let _e11: Line = other_637;
    let _e14: Line = other_637;
    let _e17: Line = other_637;
    let _e30: Plane = self_751;
    let _e34: Line = other_637;
    let _e37: Line = other_637;
    let _e40: Line = other_637;
    let _e43: Line = other_637;
    let _e57: Plane = self_751;
    let _e61: Line = other_637;
    let _e64: Line = other_637;
    let _e67: Line = other_637;
    let _e70: Line = other_637;
    let _e84: Plane = self_751;
    let _e88: Line = other_637;
    let _e91: Line = other_637;
    let _e94: Line = other_637;
    let _e97: Line = other_637;
    return Plane((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.x, _e11.g0_.x, _e14.g1_.z, _e17.g1_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e30.g0_.z) * vec4<f32>(_e34.g0_.y, _e37.g1_.z, _e40.g0_.y, _e43.g1_.x)) * vec4<f32>(-(1.0), -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e57.g0_.w) * vec4<f32>(_e61.g0_.z, _e64.g1_.y, _e67.g1_.x, _e70.g0_.z)) * vec4<f32>(-(1.0), 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e84.g0_.x) * vec4<f32>(_e88.g0_.x, _e91.g0_.x, _e94.g0_.y, _e97.g0_.z)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn plane_line_left_contraction(self_752: Plane, other_638: Line) -> Plane {
    var self_753: Plane;
    var other_639: Line;

    self_753 = self_752;
    other_639 = other_638;
    let _e4: Plane = self_753;
    let _e8: Line = other_639;
    let _e11: Line = other_639;
    let _e14: Line = other_639;
    let _e17: Line = other_639;
    let _e30: Plane = self_753;
    let _e34: Line = other_639;
    let _e37: Line = other_639;
    let _e40: Line = other_639;
    let _e43: Line = other_639;
    let _e57: Plane = self_753;
    let _e61: Line = other_639;
    let _e64: Line = other_639;
    let _e67: Line = other_639;
    let _e70: Line = other_639;
    let _e84: Plane = self_753;
    let _e88: Line = other_639;
    let _e91: Line = other_639;
    let _e94: Line = other_639;
    let _e97: Line = other_639;
    return Plane((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.x, _e11.g0_.x, _e14.g1_.z, _e17.g1_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e30.g0_.z) * vec4<f32>(_e34.g0_.y, _e37.g1_.z, _e40.g0_.y, _e43.g1_.x)) * vec4<f32>(-(1.0), -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e57.g0_.w) * vec4<f32>(_e61.g0_.z, _e64.g1_.y, _e67.g1_.x, _e70.g0_.z)) * vec4<f32>(-(1.0), 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e84.g0_.x) * vec4<f32>(_e88.g0_.x, _e91.g0_.x, _e94.g0_.y, _e97.g0_.z)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn plane_translator_inner_product(self_754: Plane, other_640: Translator) -> Plane {
    var self_755: Plane;
    var other_641: Translator;

    self_755 = self_754;
    other_641 = other_640;
    let _e4: Plane = self_755;
    let _e8: Translator = other_641;
    let _e11: Plane = self_755;
    let _e15: Translator = other_641;
    let _e27: Plane = self_755;
    let _e31: Translator = other_641;
    let _e43: Plane = self_755;
    let _e46: Translator = other_641;
    return Plane(((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.z) * _e15.g0_.zzxz) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0))) + ((vec4<f32>(_e27.g0_.w) * _e31.g0_.wwwx) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))) + ((_e43.g0_.yyxx * _e46.g0_.yxxx) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))));
}

fn plane_translator_left_contraction(self_756: Plane, other_642: Translator) -> Plane {
    var self_757: Plane;
    var other_643: Translator;

    self_757 = self_756;
    other_643 = other_642;
    let _e4: Plane = self_757;
    let _e8: Translator = other_643;
    let _e20: Plane = self_757;
    let _e24: Translator = other_643;
    let _e37: Plane = self_757;
    let _e40: Translator = other_643;
    return Plane(((((vec4<f32>(_e4.g0_.z) * vec4<f32>(_e8.g0_.z)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0)) + ((vec4<f32>(_e20.g0_.w) * vec4<f32>(_e24.g0_.w)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((_e37.g0_.yxxx * _e40.g0_.yyzw) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0))));
}

fn plane_translator_right_contraction(self_758: Plane, other_644: Translator) -> Plane {
    var self_759: Plane;
    var other_645: Translator;

    self_759 = self_758;
    other_645 = other_644;
    let _e4: Plane = self_759;
    let _e6: Translator = other_645;
    return Plane((_e4.g0_ * vec4<f32>(_e6.g0_.x)));
}

fn plane_motor_geometric_product(self_760: Plane, other_646: Motor) -> PointAndPlane {
    var self_761: Plane;
    var other_647: Motor;

    self_761 = self_760;
    other_647 = other_646;
    let _e4: Plane = self_761;
    let _e8: Motor = other_647;
    let _e11: Motor = other_647;
    let _e14: Motor = other_647;
    let _e17: Motor = other_647;
    let _e31: Plane = self_761;
    let _e35: Motor = other_647;
    let _e38: Motor = other_647;
    let _e41: Motor = other_647;
    let _e44: Motor = other_647;
    let _e57: Plane = self_761;
    let _e61: Motor = other_647;
    let _e64: Motor = other_647;
    let _e67: Motor = other_647;
    let _e70: Motor = other_647;
    let _e83: Plane = self_761;
    let _e87: Motor = other_647;
    let _e90: Motor = other_647;
    let _e93: Motor = other_647;
    let _e96: Motor = other_647;
    let _e109: Plane = self_761;
    let _e113: Motor = other_647;
    let _e116: Motor = other_647;
    let _e119: Motor = other_647;
    let _e122: Motor = other_647;
    let _e127: Plane = self_761;
    let _e131: Motor = other_647;
    let _e134: Motor = other_647;
    let _e137: Motor = other_647;
    let _e140: Motor = other_647;
    let _e154: Plane = self_761;
    let _e158: Motor = other_647;
    let _e161: Motor = other_647;
    let _e164: Motor = other_647;
    let _e167: Motor = other_647;
    let _e181: Plane = self_761;
    let _e185: Motor = other_647;
    let _e188: Motor = other_647;
    let _e191: Motor = other_647;
    let _e194: Motor = other_647;
    return PointAndPlane((((((vec4<f32>(_e4.g0_.x) * vec4<f32>(_e8.g1_.x, _e11.g0_.y, _e14.g0_.z, _e17.g0_.w)) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))) + ((vec4<f32>(_e31.g0_.y) * vec4<f32>(_e35.g0_.y, _e38.g1_.x, _e41.g1_.w, _e44.g1_.z)) * vec4<f32>(1.0, 1.0, -(1.0), 1.0))) + ((vec4<f32>(_e57.g0_.z) * vec4<f32>(_e61.g0_.z, _e64.g1_.w, _e67.g1_.x, _e70.g1_.y)) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e83.g0_.w) * vec4<f32>(_e87.g0_.w, _e90.g1_.z, _e93.g1_.y, _e96.g1_.x)) * vec4<f32>(1.0, -(1.0), 1.0, 1.0))), ((((vec4<f32>(_e109.g0_.x) * vec4<f32>(_e113.g0_.x, _e116.g1_.y, _e119.g1_.z, _e122.g1_.w)) + ((vec4<f32>(_e127.g0_.y) * vec4<f32>(_e131.g1_.y, _e134.g0_.x, _e137.g0_.w, _e140.g0_.z)) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e154.g0_.z) * vec4<f32>(_e158.g1_.z, _e161.g0_.w, _e164.g0_.x, _e167.g0_.y)) * vec4<f32>(-(1.0), -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e181.g0_.w) * vec4<f32>(_e185.g1_.w, _e188.g0_.z, _e191.g0_.y, _e194.g0_.x)) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))));
}

fn plane_motor_regressive_product(self_762: Plane, other_648: Motor) -> Plane {
    var self_763: Plane;
    var other_649: Motor;

    self_763 = self_762;
    other_649 = other_648;
    let _e4: Plane = self_763;
    let _e6: Motor = other_649;
    return Plane((_e4.g0_ * vec4<f32>(_e6.g1_.x)));
}

fn plane_motor_outer_product(self_764: Plane, other_650: Motor) -> PointAndPlane {
    var self_765: Plane;
    var other_651: Motor;

    self_765 = self_764;
    other_651 = other_650;
    let _e4: Plane = self_765;
    let _e8: Motor = other_651;
    let _e11: Motor = other_651;
    let _e14: Motor = other_651;
    let _e17: Motor = other_651;
    let _e29: Plane = self_765;
    let _e33: Motor = other_651;
    let _e36: Motor = other_651;
    let _e39: Motor = other_651;
    let _e42: Motor = other_651;
    let _e55: Plane = self_765;
    let _e59: Motor = other_651;
    let _e62: Motor = other_651;
    let _e65: Motor = other_651;
    let _e68: Motor = other_651;
    let _e81: Plane = self_765;
    let _e85: Motor = other_651;
    let _e98: Plane = self_765;
    let _e100: Motor = other_651;
    return PointAndPlane((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y, _e11.g0_.y, _e14.g1_.w, _e17.g1_.z)) * vec4<f32>(1.0, 0.0, -(1.0), 1.0)) + ((vec4<f32>(_e29.g0_.z) * vec4<f32>(_e33.g0_.z, _e36.g1_.w, _e39.g0_.z, _e42.g1_.y)) * vec4<f32>(1.0, 1.0, 0.0, -(1.0)))) + ((vec4<f32>(_e55.g0_.w) * vec4<f32>(_e59.g0_.w, _e62.g1_.z, _e65.g1_.y, _e68.g0_.w)) * vec4<f32>(1.0, -(1.0), 1.0, 0.0))) + ((vec4<f32>(_e81.g0_.x) * _e85.g0_) * vec4<f32>(0.0, -(1.0), -(1.0), -(1.0)))), (_e98.g0_ * vec4<f32>(_e100.g0_.x)));
}

fn plane_motor_inner_product(self_766: Plane, other_652: Motor) -> PointAndPlane {
    var self_767: Plane;
    var other_653: Motor;

    self_767 = self_766;
    other_653 = other_652;
    let _e4: Plane = self_767;
    let _e6: Motor = other_653;
    let _e11: Plane = self_767;
    let _e15: Motor = other_653;
    let _e18: Motor = other_653;
    let _e21: Motor = other_653;
    let _e24: Motor = other_653;
    let _e29: Plane = self_767;
    let _e33: Motor = other_653;
    let _e36: Motor = other_653;
    let _e39: Motor = other_653;
    let _e42: Motor = other_653;
    let _e56: Plane = self_767;
    let _e60: Motor = other_653;
    let _e63: Motor = other_653;
    let _e66: Motor = other_653;
    let _e69: Motor = other_653;
    let _e83: Plane = self_767;
    let _e87: Motor = other_653;
    let _e90: Motor = other_653;
    let _e93: Motor = other_653;
    let _e96: Motor = other_653;
    return PointAndPlane((_e4.g0_ * vec4<f32>(_e6.g1_.x)), ((((vec4<f32>(_e11.g0_.x) * vec4<f32>(_e15.g0_.x, _e18.g1_.y, _e21.g1_.z, _e24.g1_.w)) + ((vec4<f32>(_e29.g0_.y) * vec4<f32>(_e33.g1_.y, _e36.g0_.x, _e39.g0_.w, _e42.g0_.z)) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e56.g0_.z) * vec4<f32>(_e60.g1_.z, _e63.g0_.w, _e66.g0_.x, _e69.g0_.y)) * vec4<f32>(-(1.0), -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e83.g0_.w) * vec4<f32>(_e87.g1_.w, _e90.g0_.z, _e93.g0_.y, _e96.g0_.x)) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))));
}

fn plane_motor_left_contraction(self_768: Plane, other_654: Motor) -> PointAndPlane {
    var self_769: Plane;
    var other_655: Motor;

    self_769 = self_768;
    other_655 = other_654;
    let _e4: Plane = self_769;
    let _e6: Motor = other_655;
    let _e11: Plane = self_769;
    let _e15: Motor = other_655;
    let _e18: Motor = other_655;
    let _e21: Motor = other_655;
    let _e24: Motor = other_655;
    let _e37: Plane = self_769;
    let _e41: Motor = other_655;
    let _e44: Motor = other_655;
    let _e47: Motor = other_655;
    let _e50: Motor = other_655;
    let _e64: Plane = self_769;
    let _e68: Motor = other_655;
    let _e71: Motor = other_655;
    let _e74: Motor = other_655;
    let _e77: Motor = other_655;
    let _e91: Plane = self_769;
    let _e95: Motor = other_655;
    return PointAndPlane((_e4.g0_ * vec4<f32>(_e6.g1_.x)), (((((vec4<f32>(_e11.g0_.y) * vec4<f32>(_e15.g1_.y, _e18.g1_.y, _e21.g0_.w, _e24.g0_.z)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e37.g0_.z) * vec4<f32>(_e41.g1_.z, _e44.g0_.w, _e47.g1_.z, _e50.g0_.y)) * vec4<f32>(-(1.0), -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e64.g0_.w) * vec4<f32>(_e68.g1_.w, _e71.g0_.z, _e74.g0_.y, _e77.g1_.w)) * vec4<f32>(-(1.0), 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e91.g0_.x) * _e95.g1_) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn plane_motor_right_contraction(self_770: Plane, other_656: Motor) -> Plane {
    var self_771: Plane;
    var other_657: Motor;

    self_771 = self_770;
    other_657 = other_656;
    let _e4: Plane = self_771;
    let _e6: Motor = other_657;
    return Plane((_e4.g0_ * vec4<f32>(_e6.g0_.x)));
}

fn plane_point_and_plane_add(self_772: Plane, other_658: PointAndPlane) -> PointAndPlane {
    var self_773: Plane;
    var other_659: PointAndPlane;

    self_773 = self_772;
    other_659 = other_658;
    let _e4: PointAndPlane = other_659;
    let _e6: Plane = self_773;
    let _e8: PointAndPlane = other_659;
    return PointAndPlane(_e4.g0_, (_e6.g0_ + _e8.g1_));
}

fn plane_point_and_plane_sub(self_774: Plane, other_660: PointAndPlane) -> PointAndPlane {
    var self_775: Plane;
    var other_661: PointAndPlane;

    self_775 = self_774;
    other_661 = other_660;
    let _e6: PointAndPlane = other_661;
    let _e9: Plane = self_775;
    let _e11: PointAndPlane = other_661;
    return PointAndPlane((vec4<f32>(0.0) - _e6.g0_), (_e9.g0_ - _e11.g1_));
}

fn plane_point_and_plane_geometric_product(self_776: Plane, other_662: PointAndPlane) -> Motor {
    var self_777: Plane;
    var other_663: PointAndPlane;

    self_777 = self_776;
    other_663 = other_662;
    let _e4: Plane = self_777;
    let _e8: PointAndPlane = other_663;
    let _e11: PointAndPlane = other_663;
    let _e14: PointAndPlane = other_663;
    let _e17: PointAndPlane = other_663;
    let _e31: Plane = self_777;
    let _e35: PointAndPlane = other_663;
    let _e38: PointAndPlane = other_663;
    let _e41: PointAndPlane = other_663;
    let _e44: PointAndPlane = other_663;
    let _e57: Plane = self_777;
    let _e61: PointAndPlane = other_663;
    let _e64: PointAndPlane = other_663;
    let _e67: PointAndPlane = other_663;
    let _e70: PointAndPlane = other_663;
    let _e83: Plane = self_777;
    let _e87: PointAndPlane = other_663;
    let _e90: PointAndPlane = other_663;
    let _e93: PointAndPlane = other_663;
    let _e96: PointAndPlane = other_663;
    let _e109: Plane = self_777;
    let _e113: PointAndPlane = other_663;
    let _e116: PointAndPlane = other_663;
    let _e119: PointAndPlane = other_663;
    let _e122: PointAndPlane = other_663;
    let _e127: Plane = self_777;
    let _e131: PointAndPlane = other_663;
    let _e134: PointAndPlane = other_663;
    let _e137: PointAndPlane = other_663;
    let _e140: PointAndPlane = other_663;
    let _e154: Plane = self_777;
    let _e158: PointAndPlane = other_663;
    let _e161: PointAndPlane = other_663;
    let _e164: PointAndPlane = other_663;
    let _e167: PointAndPlane = other_663;
    let _e181: Plane = self_777;
    let _e185: PointAndPlane = other_663;
    let _e188: PointAndPlane = other_663;
    let _e191: PointAndPlane = other_663;
    let _e194: PointAndPlane = other_663;
    return Motor((((((vec4<f32>(_e4.g0_.x) * vec4<f32>(_e8.g1_.x, _e11.g0_.y, _e14.g0_.z, _e17.g0_.w)) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))) + ((vec4<f32>(_e31.g0_.y) * vec4<f32>(_e35.g1_.y, _e38.g0_.x, _e41.g1_.w, _e44.g1_.z)) * vec4<f32>(1.0, 1.0, -(1.0), 1.0))) + ((vec4<f32>(_e57.g0_.z) * vec4<f32>(_e61.g1_.z, _e64.g1_.w, _e67.g0_.x, _e70.g1_.y)) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e83.g0_.w) * vec4<f32>(_e87.g1_.w, _e90.g1_.z, _e93.g1_.y, _e96.g0_.x)) * vec4<f32>(1.0, -(1.0), 1.0, 1.0))), ((((vec4<f32>(_e109.g0_.x) * vec4<f32>(_e113.g0_.x, _e116.g1_.y, _e119.g1_.z, _e122.g1_.w)) + ((vec4<f32>(_e127.g0_.y) * vec4<f32>(_e131.g0_.y, _e134.g1_.x, _e137.g0_.w, _e140.g0_.z)) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e154.g0_.z) * vec4<f32>(_e158.g0_.z, _e161.g0_.w, _e164.g1_.x, _e167.g0_.y)) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e181.g0_.w) * vec4<f32>(_e185.g0_.w, _e188.g0_.z, _e191.g0_.y, _e194.g1_.x)) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))));
}

fn plane_point_and_plane_regressive_product(self_778: Plane, other_664: PointAndPlane) -> Scalar {
    var self_779: Plane;
    var other_665: PointAndPlane;

    self_779 = self_778;
    other_665 = other_664;
    let _e4: Plane = self_779;
    let _e7: PointAndPlane = other_665;
    let _e11: Plane = self_779;
    let _e14: PointAndPlane = other_665;
    let _e19: Plane = self_779;
    let _e22: PointAndPlane = other_665;
    let _e27: Plane = self_779;
    let _e30: PointAndPlane = other_665;
    return Scalar(((((_e4.g0_.x * _e7.g0_.x) + (_e11.g0_.y * _e14.g0_.y)) + (_e19.g0_.z * _e22.g0_.z)) + (_e27.g0_.w * _e30.g0_.w)));
}

fn plane_point_and_plane_right_contraction(self_780: Plane, other_666: PointAndPlane) -> Scalar {
    var self_781: Plane;
    var other_667: PointAndPlane;

    self_781 = self_780;
    other_667 = other_666;
    let _e4: Plane = self_781;
    let _e7: PointAndPlane = other_667;
    let _e11: Plane = self_781;
    let _e14: PointAndPlane = other_667;
    let _e19: Plane = self_781;
    let _e22: PointAndPlane = other_667;
    let _e27: Plane = self_781;
    let _e30: PointAndPlane = other_667;
    return Scalar(((((_e4.g0_.x * _e7.g1_.x) + (_e11.g0_.y * _e14.g1_.y)) + (_e19.g0_.z * _e22.g1_.z)) + (_e27.g0_.w * _e30.g1_.w)));
}

fn plane_point_and_plane_scalar_product(self_782: Plane, other_668: PointAndPlane) -> Scalar {
    var self_783: Plane;
    var other_669: PointAndPlane;

    self_783 = self_782;
    other_669 = other_668;
    let _e4: Plane = self_783;
    let _e7: PointAndPlane = other_669;
    let _e11: Plane = self_783;
    let _e14: PointAndPlane = other_669;
    let _e19: Plane = self_783;
    let _e22: PointAndPlane = other_669;
    let _e27: Plane = self_783;
    let _e30: PointAndPlane = other_669;
    return Scalar(((((_e4.g0_.x * _e7.g1_.x) + (_e11.g0_.y * _e14.g1_.y)) + (_e19.g0_.z * _e22.g1_.z)) + (_e27.g0_.w * _e30.g1_.w)));
}

fn plane_squared_magnitude(self_784: Plane) -> Scalar {
    var self_785: Plane;

    self_785 = self_784;
    let _e4: Plane = self_785;
    let _e5: Plane = plane_reversal(_e4);
    let _e6: Plane = self_785;
    let _e8: Plane = self_785;
    let _e9: Plane = plane_reversal(_e8);
    let _e10: Scalar = plane_plane_scalar_product(_e6, _e9);
    return _e10;
}

fn plane_magnitude(self_786: Plane) -> Scalar {
    var self_787: Plane;

    self_787 = self_786;
    let _e3: Plane = self_787;
    let _e4: Scalar = plane_squared_magnitude(_e3);
    let _e7: Plane = self_787;
    let _e8: Scalar = plane_squared_magnitude(_e7);
    return Scalar(sqrt(_e8.g0_));
}

fn plane_scale(self_788: Plane, other_670: f32) -> Plane {
    var self_789: Plane;
    var other_671: f32;

    self_789 = self_788;
    other_671 = other_670;
    let _e5: f32 = other_671;
    let _e7: Plane = self_789;
    let _e8: f32 = other_671;
    let _e10: Plane = plane_scalar_geometric_product(_e7, Scalar(_e8));
    return _e10;
}

fn plane_signum(self_790: Plane) -> Plane {
    var self_791: Plane;

    self_791 = self_790;
    let _e5: Plane = self_791;
    let _e6: Scalar = plane_magnitude(_e5);
    let _e10: Plane = self_791;
    let _e13: Plane = self_791;
    let _e14: Scalar = plane_magnitude(_e13);
    let _e18: Plane = plane_scalar_geometric_product(_e10, Scalar((1.0 / _e14.g0_)));
    return _e18;
}

fn plane_inverse(self_792: Plane) -> Plane {
    var self_793: Plane;

    self_793 = self_792;
    let _e3: Plane = self_793;
    let _e4: Plane = plane_reversal(_e3);
    let _e7: Plane = self_793;
    let _e8: Scalar = plane_squared_magnitude(_e7);
    let _e13: Plane = self_793;
    let _e14: Plane = plane_reversal(_e13);
    let _e17: Plane = self_793;
    let _e18: Scalar = plane_squared_magnitude(_e17);
    let _e22: Plane = plane_scalar_geometric_product(_e14, Scalar((1.0 / _e18.g0_)));
    return _e22;
}

fn line_zero() -> Line {
    return Line(vec3<f32>(0.0), vec3<f32>(0.0));
}

fn line_one() -> Line {
    return Line(vec3<f32>(0.0), vec3<f32>(0.0));
}

fn line_neg(self_794: Line) -> Line {
    var self_795: Line;

    self_795 = self_794;
    let _e2: Line = self_795;
    let _e8: Line = self_795;
    return Line((_e2.g0_ * vec3<f32>(-(1.0))), (_e8.g1_ * vec3<f32>(-(1.0))));
}

fn line_automorphism(self_796: Line) -> Line {
    var self_797: Line;

    self_797 = self_796;
    let _e2: Line = self_797;
    let _e4: Line = self_797;
    return Line(_e2.g0_, _e4.g1_);
}

fn line_reversal(self_798: Line) -> Line {
    var self_799: Line;

    self_799 = self_798;
    let _e2: Line = self_799;
    let _e8: Line = self_799;
    return Line((_e2.g0_ * vec3<f32>(-(1.0))), (_e8.g1_ * vec3<f32>(-(1.0))));
}

fn line_conjugation(self_800: Line) -> Line {
    var self_801: Line;

    self_801 = self_800;
    let _e2: Line = self_801;
    let _e8: Line = self_801;
    return Line((_e2.g0_ * vec3<f32>(-(1.0))), (_e8.g1_ * vec3<f32>(-(1.0))));
}

fn line_dual(self_802: Line) -> Line {
    var self_803: Line;

    self_803 = self_802;
    let _e2: Line = self_803;
    let _e4: Line = self_803;
    return Line(_e2.g1_, _e4.g0_);
}

fn line_scalar_geometric_product(self_804: Line, other_672: Scalar) -> Line {
    var self_805: Line;
    var other_673: Scalar;

    self_805 = self_804;
    other_673 = other_672;
    let _e4: Line = self_805;
    let _e6: Scalar = other_673;
    let _e10: Line = self_805;
    let _e12: Scalar = other_673;
    return Line((_e4.g0_ * vec3<f32>(_e6.g0_)), (_e10.g1_ * vec3<f32>(_e12.g0_)));
}

fn line_scalar_outer_product(self_806: Line, other_674: Scalar) -> Line {
    var self_807: Line;
    var other_675: Scalar;

    self_807 = self_806;
    other_675 = other_674;
    let _e4: Line = self_807;
    let _e6: Scalar = other_675;
    let _e10: Line = self_807;
    let _e12: Scalar = other_675;
    return Line((_e4.g0_ * vec3<f32>(_e6.g0_)), (_e10.g1_ * vec3<f32>(_e12.g0_)));
}

fn line_scalar_inner_product(self_808: Line, other_676: Scalar) -> Line {
    var self_809: Line;
    var other_677: Scalar;

    self_809 = self_808;
    other_677 = other_676;
    let _e4: Line = self_809;
    let _e6: Scalar = other_677;
    let _e10: Line = self_809;
    let _e12: Scalar = other_677;
    return Line((_e4.g0_ * vec3<f32>(_e6.g0_)), (_e10.g1_ * vec3<f32>(_e12.g0_)));
}

fn line_scalar_right_contraction(self_810: Line, other_678: Scalar) -> Line {
    var self_811: Line;
    var other_679: Scalar;

    self_811 = self_810;
    other_679 = other_678;
    let _e4: Line = self_811;
    let _e6: Scalar = other_679;
    let _e10: Line = self_811;
    let _e12: Scalar = other_679;
    return Line((_e4.g0_ * vec3<f32>(_e6.g0_)), (_e10.g1_ * vec3<f32>(_e12.g0_)));
}

fn line_multi_vector_add(self_812: Line, other_680: MultiVector) -> MultiVector {
    var self_813: Line;
    var other_681: MultiVector;

    self_813 = self_812;
    other_681 = other_680;
    let _e4: Line = self_813;
    let _e7: Line = self_813;
    let _e10: Line = self_813;
    let _e13: Line = self_813;
    let _e23: MultiVector = other_681;
    let _e26: MultiVector = other_681;
    let _e28: MultiVector = other_681;
    let _e30: Line = self_813;
    let _e33: Line = self_813;
    let _e36: Line = self_813;
    let _e39: Line = self_813;
    let _e49: MultiVector = other_681;
    return MultiVector(((vec4<f32>(_e4.g0_.x, _e7.g1_.x, _e10.g1_.y, _e13.g1_.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0)) + _e23.g0_), _e26.g1_, _e28.g2_, ((vec4<f32>(_e30.g0_.x, _e33.g0_.x, _e36.g0_.y, _e39.g0_.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0)) + _e49.g3_));
}

fn line_multi_vector_sub(self_814: Line, other_682: MultiVector) -> MultiVector {
    var self_815: Line;
    var other_683: MultiVector;

    self_815 = self_814;
    other_683 = other_682;
    let _e4: Line = self_815;
    let _e7: Line = self_815;
    let _e10: Line = self_815;
    let _e13: Line = self_815;
    let _e23: MultiVector = other_683;
    let _e28: MultiVector = other_683;
    let _e33: MultiVector = other_683;
    let _e36: Line = self_815;
    let _e39: Line = self_815;
    let _e42: Line = self_815;
    let _e45: Line = self_815;
    let _e55: MultiVector = other_683;
    return MultiVector(((vec4<f32>(_e4.g0_.x, _e7.g1_.x, _e10.g1_.y, _e13.g1_.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0)) - _e23.g0_), (vec4<f32>(0.0) - _e28.g1_), (vec4<f32>(0.0) - _e33.g2_), ((vec4<f32>(_e36.g0_.x, _e39.g0_.x, _e42.g0_.y, _e45.g0_.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0)) - _e55.g3_));
}

fn line_multi_vector_geometric_product(self_816: Line, other_684: MultiVector) -> MultiVector {
    var self_817: Line;
    var other_685: MultiVector;

    self_817 = self_816;
    other_685 = other_684;
    let _e4: Line = self_817;
    let _e8: MultiVector = other_685;
    let _e21: Line = self_817;
    let _e25: MultiVector = other_685;
    let _e39: Line = self_817;
    let _e43: MultiVector = other_685;
    let _e57: Line = self_817;
    let _e61: MultiVector = other_685;
    let _e74: Line = self_817;
    let _e78: MultiVector = other_685;
    let _e91: Line = self_817;
    let _e95: MultiVector = other_685;
    let _e108: Line = self_817;
    let _e112: MultiVector = other_685;
    let _e124: Line = self_817;
    let _e128: MultiVector = other_685;
    let _e141: Line = self_817;
    let _e145: MultiVector = other_685;
    let _e158: Line = self_817;
    let _e162: MultiVector = other_685;
    let _e175: Line = self_817;
    let _e179: MultiVector = other_685;
    let _e192: Line = self_817;
    let _e196: MultiVector = other_685;
    let _e209: Line = self_817;
    let _e213: MultiVector = other_685;
    let _e225: Line = self_817;
    let _e229: MultiVector = other_685;
    let _e242: Line = self_817;
    let _e246: MultiVector = other_685;
    let _e259: Line = self_817;
    let _e263: MultiVector = other_685;
    let _e276: Line = self_817;
    let _e280: MultiVector = other_685;
    let _e293: Line = self_817;
    let _e297: MultiVector = other_685;
    let _e310: Line = self_817;
    let _e314: MultiVector = other_685;
    let _e325: Line = self_817;
    let _e329: MultiVector = other_685;
    let _e341: Line = self_817;
    let _e345: MultiVector = other_685;
    let _e357: Line = self_817;
    let _e361: MultiVector = other_685;
    let _e374: Line = self_817;
    let _e378: MultiVector = other_685;
    let _e391: Line = self_817;
    let _e395: MultiVector = other_685;
    return MultiVector((((((((vec4<f32>(_e4.g0_.x) * _e8.g3_.yxwz) * vec4<f32>(-(1.0), -(1.0), 1.0, -(1.0))) + ((vec4<f32>(_e21.g0_.y) * _e25.g3_.zwxy) * vec4<f32>(-(1.0), -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e39.g0_.z) * _e43.g3_.wzyx) * vec4<f32>(-(1.0), 1.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e57.g1_.x) * _e61.g0_.yxwz) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e74.g1_.y) * _e78.g0_.zwxy) * vec4<f32>(-(1.0), -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e91.g1_.z) * _e95.g0_.wzyx) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))), (((((((vec4<f32>(_e108.g0_.x) * _e112.g2_.yxwz) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0))) + ((vec4<f32>(_e124.g0_.y) * _e128.g2_.zwxy) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e141.g0_.z) * _e145.g2_.wzyx) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e158.g1_.x) * _e162.g1_.yxwz) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e175.g1_.y) * _e179.g1_.zwxy) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e192.g1_.z) * _e196.g1_.wzyx) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))), (((((((vec4<f32>(_e209.g0_.x) * _e213.g1_.yxwz) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0))) + ((vec4<f32>(_e225.g0_.y) * _e229.g1_.zwxy) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e242.g0_.z) * _e246.g1_.wzyx) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e259.g1_.x) * _e263.g2_.yxwz) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e276.g1_.y) * _e280.g2_.zwxy) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e293.g1_.z) * _e297.g2_.wzyx) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))), (((((((vec4<f32>(_e310.g0_.x) * _e314.g0_.yxwz) * vec4<f32>(1.0, 1.0, 1.0, -(1.0))) + ((vec4<f32>(_e325.g0_.y) * _e329.g0_.zwxy) * vec4<f32>(1.0, -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e341.g0_.z) * _e345.g0_.wzyx) * vec4<f32>(1.0, 1.0, -(1.0), 1.0))) + ((vec4<f32>(_e357.g1_.x) * _e361.g3_.yxwz) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e374.g1_.y) * _e378.g3_.zwxy) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e391.g1_.z) * _e395.g3_.wzyx) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))));
}

fn line_multi_vector_scalar_product(self_818: Line, other_686: MultiVector) -> Scalar {
    var self_819: Line;
    var other_687: MultiVector;

    self_819 = self_818;
    other_687 = other_686;
    let _e5: Line = self_819;
    let _e8: MultiVector = other_687;
    let _e13: Line = self_819;
    let _e16: MultiVector = other_687;
    let _e21: Line = self_819;
    let _e24: MultiVector = other_687;
    let _e29: Line = self_819;
    let _e32: MultiVector = other_687;
    let _e37: Line = self_819;
    let _e40: MultiVector = other_687;
    let _e45: Line = self_819;
    let _e48: MultiVector = other_687;
    return Scalar(((((((0.0 - (_e5.g0_.x * _e8.g3_.y)) - (_e13.g0_.y * _e16.g3_.z)) - (_e21.g0_.z * _e24.g3_.w)) - (_e29.g1_.x * _e32.g0_.y)) - (_e37.g1_.y * _e40.g0_.z)) - (_e45.g1_.z * _e48.g0_.w)));
}

fn line_rotor_geometric_product(self_820: Line, other_688: Rotor) -> Motor {
    var self_821: Line;
    var other_689: Rotor;

    self_821 = self_820;
    other_689 = other_688;
    let _e4: Line = self_821;
    let _e8: Rotor = other_689;
    let _e20: Line = self_821;
    let _e24: Rotor = other_689;
    let _e37: Line = self_821;
    let _e41: Rotor = other_689;
    let _e54: Line = self_821;
    let _e58: Rotor = other_689;
    let _e69: Line = self_821;
    let _e73: Rotor = other_689;
    let _e85: Line = self_821;
    let _e89: Rotor = other_689;
    return Motor(((((vec4<f32>(_e4.g1_.x) * _e8.g0_.yxwz) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0))) + ((vec4<f32>(_e20.g1_.y) * _e24.g0_.zwxy) * vec4<f32>(-(1.0), -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e37.g1_.z) * _e41.g0_.wzyx) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))), ((((vec4<f32>(_e54.g0_.x) * _e58.g0_.yxwz) * vec4<f32>(1.0, 1.0, 1.0, -(1.0))) + ((vec4<f32>(_e69.g0_.y) * _e73.g0_.zwxy) * vec4<f32>(1.0, -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e85.g0_.z) * _e89.g0_.wzyx) * vec4<f32>(1.0, 1.0, -(1.0), 1.0))));
}

fn line_rotor_regressive_product(self_822: Line, other_690: Rotor) -> Scalar {
    var self_823: Line;
    var other_691: Rotor;

    self_823 = self_822;
    other_691 = other_690;
    let _e4: Line = self_823;
    let _e7: Rotor = other_691;
    let _e11: Line = self_823;
    let _e14: Rotor = other_691;
    let _e19: Line = self_823;
    let _e22: Rotor = other_691;
    return Scalar((((_e4.g0_.x * _e7.g0_.y) + (_e11.g0_.y * _e14.g0_.z)) + (_e19.g0_.z * _e22.g0_.w)));
}

fn line_rotor_left_contraction(self_824: Line, other_692: Rotor) -> Scalar {
    var self_825: Line;
    var other_693: Rotor;

    self_825 = self_824;
    other_693 = other_692;
    let _e5: Line = self_825;
    let _e8: Rotor = other_693;
    let _e13: Line = self_825;
    let _e16: Rotor = other_693;
    let _e21: Line = self_825;
    let _e24: Rotor = other_693;
    return Scalar((((0.0 - (_e5.g1_.x * _e8.g0_.y)) - (_e13.g1_.y * _e16.g0_.z)) - (_e21.g1_.z * _e24.g0_.w)));
}

fn line_rotor_scalar_product(self_826: Line, other_694: Rotor) -> Scalar {
    var self_827: Line;
    var other_695: Rotor;

    self_827 = self_826;
    other_695 = other_694;
    let _e5: Line = self_827;
    let _e8: Rotor = other_695;
    let _e13: Line = self_827;
    let _e16: Rotor = other_695;
    let _e21: Line = self_827;
    let _e24: Rotor = other_695;
    return Scalar((((0.0 - (_e5.g1_.x * _e8.g0_.y)) - (_e13.g1_.y * _e16.g0_.z)) - (_e21.g1_.z * _e24.g0_.w)));
}

fn line_point_geometric_product(self_828: Line, other_696: Point) -> PointAndPlane {
    var self_829: Line;
    var other_697: Point;

    self_829 = self_828;
    other_697 = other_696;
    let _e4: Line = self_829;
    let _e8: Point = other_697;
    let _e19: Line = self_829;
    let _e23: Point = other_697;
    let _e35: Line = self_829;
    let _e39: Point = other_697;
    let _e51: Line = self_829;
    let _e55: Point = other_697;
    let _e67: Line = self_829;
    let _e70: Line = self_829;
    let _e73: Line = self_829;
    let _e76: Line = self_829;
    let _e80: Point = other_697;
    let _e93: Line = self_829;
    let _e97: Point = other_697;
    let _e108: Line = self_829;
    let _e112: Point = other_697;
    let _e124: Line = self_829;
    let _e128: Point = other_697;
    let _e140: Line = self_829;
    let _e144: Point = other_697;
    let _e156: Line = self_829;
    let _e159: Line = self_829;
    let _e162: Line = self_829;
    let _e165: Line = self_829;
    let _e169: Point = other_697;
    return PointAndPlane(((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.zzxz) * vec4<f32>(1.0, 0.0, -(1.0), 0.0)) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e35.g1_.y) * _e39.g0_.wwwy) * vec4<f32>(0.0, -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e51.g1_.z) * _e55.g0_.zzyz) * vec4<f32>(0.0, 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e67.g0_.x, _e70.g0_.x, _e73.g1_.x, _e76.g1_.x) * _e80.g0_.yxwz) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))), ((((((vec4<f32>(_e93.g0_.y) * _e97.g0_.wwwy) * vec4<f32>(0.0, -(1.0), 0.0, 1.0)) + ((vec4<f32>(_e108.g0_.z) * _e112.g0_.zzyz) * vec4<f32>(0.0, 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e124.g1_.y) * _e128.g0_.zzxz) * vec4<f32>(1.0, 0.0, -(1.0), 0.0))) + ((vec4<f32>(_e140.g1_.z) * _e144.g0_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e156.g1_.x, _e159.g1_.x, _e162.g0_.x, _e165.g0_.x) * _e169.g0_.yxwz) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))));
}

fn line_point_regressive_product(self_830: Line, other_698: Point) -> Plane {
    var self_831: Line;
    var other_699: Point;

    self_831 = self_830;
    other_699 = other_698;
    let _e4: Line = self_831;
    let _e8: Point = other_699;
    let _e19: Line = self_831;
    let _e23: Point = other_699;
    let _e35: Line = self_831;
    let _e39: Point = other_699;
    let _e51: Line = self_831;
    let _e55: Point = other_699;
    let _e67: Line = self_831;
    let _e70: Line = self_831;
    let _e73: Line = self_831;
    let _e76: Line = self_831;
    let _e80: Point = other_699;
    return Plane(((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.zzxz) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0)) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.wwwx) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))) + ((vec4<f32>(_e35.g1_.y) * _e39.g0_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0)))) + ((vec4<f32>(_e51.g1_.z) * _e55.g0_.zzyz) * vec4<f32>(0.0, -(1.0), 1.0, 0.0))) + ((vec4<f32>(_e67.g0_.x, _e70.g0_.x, _e73.g1_.x, _e76.g1_.x) * _e80.g0_.yxwz) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))));
}

fn line_point_inner_product(self_832: Line, other_700: Point) -> Plane {
    var self_833: Line;
    var other_701: Point;

    self_833 = self_832;
    other_701 = other_700;
    let _e4: Line = self_833;
    let _e8: Point = other_701;
    let _e19: Line = self_833;
    let _e23: Point = other_701;
    let _e35: Line = self_833;
    let _e39: Point = other_701;
    let _e51: Line = self_833;
    let _e55: Point = other_701;
    let _e67: Line = self_833;
    let _e70: Line = self_833;
    let _e73: Line = self_833;
    let _e76: Line = self_833;
    let _e80: Point = other_701;
    return Plane(((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.wwwy) * vec4<f32>(0.0, -(1.0), 0.0, 1.0)) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.zzyz) * vec4<f32>(0.0, 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e35.g1_.y) * _e39.g0_.zzxz) * vec4<f32>(1.0, 0.0, -(1.0), 0.0))) + ((vec4<f32>(_e51.g1_.z) * _e55.g0_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e67.g1_.x, _e70.g1_.x, _e73.g0_.x, _e76.g0_.x) * _e80.g0_.yxwz) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))));
}

fn line_point_left_contraction(self_834: Line, other_702: Point) -> Plane {
    var self_835: Line;
    var other_703: Point;

    self_835 = self_834;
    other_703 = other_702;
    let _e4: Line = self_835;
    let _e8: Point = other_703;
    let _e19: Line = self_835;
    let _e23: Point = other_703;
    let _e35: Line = self_835;
    let _e39: Point = other_703;
    let _e51: Line = self_835;
    let _e55: Point = other_703;
    let _e67: Line = self_835;
    let _e70: Line = self_835;
    let _e73: Line = self_835;
    let _e76: Line = self_835;
    let _e80: Point = other_703;
    return Plane(((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.wwwy) * vec4<f32>(0.0, -(1.0), 0.0, 1.0)) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.zzyz) * vec4<f32>(0.0, 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e35.g1_.y) * _e39.g0_.zzxz) * vec4<f32>(1.0, 0.0, -(1.0), 0.0))) + ((vec4<f32>(_e51.g1_.z) * _e55.g0_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e67.g1_.x, _e70.g1_.x, _e73.g0_.x, _e76.g0_.x) * _e80.g0_.yxwz) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))));
}

fn line_ideal_point_into(self_836: Line) -> IdealPoint {
    var self_837: Line;

    self_837 = self_836;
    let _e2: Line = self_837;
    return IdealPoint(_e2.g0_);
}

fn line_ideal_point_add(self_838: Line, other_704: IdealPoint) -> Line {
    var self_839: Line;
    var other_705: IdealPoint;

    self_839 = self_838;
    other_705 = other_704;
    let _e4: Line = self_839;
    let _e6: IdealPoint = other_705;
    let _e9: Line = self_839;
    return Line((_e4.g0_ + _e6.g0_), _e9.g1_);
}

fn line_ideal_point_sub(self_840: Line, other_706: IdealPoint) -> Line {
    var self_841: Line;
    var other_707: IdealPoint;

    self_841 = self_840;
    other_707 = other_706;
    let _e4: Line = self_841;
    let _e6: IdealPoint = other_707;
    let _e9: Line = self_841;
    return Line((_e4.g0_ - _e6.g0_), _e9.g1_);
}

fn line_ideal_point_geometric_product(self_842: Line, other_708: IdealPoint) -> Motor {
    var self_843: Line;
    var other_709: IdealPoint;

    self_843 = self_842;
    other_709 = other_708;
    let _e4: Line = self_843;
    let _e8: IdealPoint = other_709;
    let _e11: IdealPoint = other_709;
    let _e14: IdealPoint = other_709;
    let _e17: IdealPoint = other_709;
    let _e30: Line = self_843;
    let _e34: IdealPoint = other_709;
    let _e37: IdealPoint = other_709;
    let _e40: IdealPoint = other_709;
    let _e43: IdealPoint = other_709;
    let _e57: Line = self_843;
    let _e61: IdealPoint = other_709;
    let _e64: IdealPoint = other_709;
    let _e67: IdealPoint = other_709;
    let _e70: IdealPoint = other_709;
    let _e84: Line = self_843;
    let _e88: IdealPoint = other_709;
    let _e91: IdealPoint = other_709;
    let _e94: IdealPoint = other_709;
    let _e97: IdealPoint = other_709;
    let _e109: Line = self_843;
    let _e113: IdealPoint = other_709;
    let _e116: IdealPoint = other_709;
    let _e119: IdealPoint = other_709;
    let _e122: IdealPoint = other_709;
    let _e135: Line = self_843;
    let _e139: IdealPoint = other_709;
    let _e142: IdealPoint = other_709;
    let _e145: IdealPoint = other_709;
    let _e148: IdealPoint = other_709;
    return Motor(((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y, _e11.g0_.z, _e14.g0_.y, _e17.g0_.x)) * vec4<f32>(-(1.0), -(1.0), 0.0, 1.0)) + ((vec4<f32>(_e30.g0_.z) * vec4<f32>(_e34.g0_.z, _e37.g0_.y, _e40.g0_.x, _e43.g0_.z)) * vec4<f32>(-(1.0), 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e57.g0_.x) * vec4<f32>(_e61.g0_.x, _e64.g0_.x, _e67.g0_.z, _e70.g0_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0)))), ((((vec4<f32>(_e84.g1_.y) * vec4<f32>(_e88.g0_.y, _e91.g0_.z, _e94.g0_.y, _e97.g0_.x)) * vec4<f32>(1.0, -(1.0), 0.0, 1.0)) + ((vec4<f32>(_e109.g1_.z) * vec4<f32>(_e113.g0_.z, _e116.g0_.y, _e119.g0_.x, _e122.g0_.z)) * vec4<f32>(1.0, 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e135.g1_.x) * vec4<f32>(_e139.g0_.x, _e142.g0_.x, _e145.g0_.z, _e148.g0_.y)) * vec4<f32>(1.0, 0.0, 1.0, -(1.0)))));
}

fn line_ideal_point_regressive_product(self_844: Line, other_710: IdealPoint) -> Scalar {
    var self_845: Line;
    var other_711: IdealPoint;

    self_845 = self_844;
    other_711 = other_710;
    let _e4: Line = self_845;
    let _e7: IdealPoint = other_711;
    let _e11: Line = self_845;
    let _e14: IdealPoint = other_711;
    let _e19: Line = self_845;
    let _e22: IdealPoint = other_711;
    return Scalar((((_e4.g1_.x * _e7.g0_.x) + (_e11.g1_.y * _e14.g0_.y)) + (_e19.g1_.z * _e22.g0_.z)));
}

fn line_ideal_point_inner_product(self_846: Line, other_712: IdealPoint) -> Scalar {
    var self_847: Line;
    var other_713: IdealPoint;

    self_847 = self_846;
    other_713 = other_712;
    let _e5: Line = self_847;
    let _e8: IdealPoint = other_713;
    let _e13: Line = self_847;
    let _e16: IdealPoint = other_713;
    let _e21: Line = self_847;
    let _e24: IdealPoint = other_713;
    return Scalar((((0.0 - (_e5.g0_.x * _e8.g0_.x)) - (_e13.g0_.y * _e16.g0_.y)) - (_e21.g0_.z * _e24.g0_.z)));
}

fn line_ideal_point_left_contraction(self_848: Line, other_714: IdealPoint) -> Scalar {
    var self_849: Line;
    var other_715: IdealPoint;

    self_849 = self_848;
    other_715 = other_714;
    let _e5: Line = self_849;
    let _e8: IdealPoint = other_715;
    let _e13: Line = self_849;
    let _e16: IdealPoint = other_715;
    let _e21: Line = self_849;
    let _e24: IdealPoint = other_715;
    return Scalar((((0.0 - (_e5.g0_.x * _e8.g0_.x)) - (_e13.g0_.y * _e16.g0_.y)) - (_e21.g0_.z * _e24.g0_.z)));
}

fn line_ideal_point_right_contraction(self_850: Line, other_716: IdealPoint) -> Scalar {
    var self_851: Line;
    var other_717: IdealPoint;

    self_851 = self_850;
    other_717 = other_716;
    let _e5: Line = self_851;
    let _e8: IdealPoint = other_717;
    let _e13: Line = self_851;
    let _e16: IdealPoint = other_717;
    let _e21: Line = self_851;
    let _e24: IdealPoint = other_717;
    return Scalar((((0.0 - (_e5.g0_.x * _e8.g0_.x)) - (_e13.g0_.y * _e16.g0_.y)) - (_e21.g0_.z * _e24.g0_.z)));
}

fn line_ideal_point_scalar_product(self_852: Line, other_718: IdealPoint) -> Scalar {
    var self_853: Line;
    var other_719: IdealPoint;

    self_853 = self_852;
    other_719 = other_718;
    let _e5: Line = self_853;
    let _e8: IdealPoint = other_719;
    let _e13: Line = self_853;
    let _e16: IdealPoint = other_719;
    let _e21: Line = self_853;
    let _e24: IdealPoint = other_719;
    return Scalar((((0.0 - (_e5.g0_.x * _e8.g0_.x)) - (_e13.g0_.y * _e16.g0_.y)) - (_e21.g0_.z * _e24.g0_.z)));
}

fn line_plane_geometric_product(self_854: Line, other_720: Plane) -> PointAndPlane {
    var self_855: Line;
    var other_721: Plane;

    self_855 = self_854;
    other_721 = other_720;
    let _e4: Line = self_855;
    let _e8: Plane = other_721;
    let _e19: Line = self_855;
    let _e23: Plane = other_721;
    let _e35: Line = self_855;
    let _e39: Plane = other_721;
    let _e51: Line = self_855;
    let _e55: Plane = other_721;
    let _e67: Line = self_855;
    let _e70: Line = self_855;
    let _e73: Line = self_855;
    let _e76: Line = self_855;
    let _e80: Plane = other_721;
    let _e93: Line = self_855;
    let _e97: Plane = other_721;
    let _e108: Line = self_855;
    let _e112: Plane = other_721;
    let _e124: Line = self_855;
    let _e128: Plane = other_721;
    let _e140: Line = self_855;
    let _e144: Plane = other_721;
    let _e156: Line = self_855;
    let _e159: Line = self_855;
    let _e162: Line = self_855;
    let _e165: Line = self_855;
    let _e169: Plane = other_721;
    return PointAndPlane(((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.wwwy) * vec4<f32>(0.0, -(1.0), 0.0, 1.0)) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.zzyz) * vec4<f32>(0.0, 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e35.g1_.y) * _e39.g0_.zzxz) * vec4<f32>(1.0, 0.0, -(1.0), 0.0))) + ((vec4<f32>(_e51.g1_.z) * _e55.g0_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e67.g1_.x, _e70.g1_.x, _e73.g0_.x, _e76.g0_.x) * _e80.g0_.yxwz) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))), ((((((vec4<f32>(_e93.g0_.y) * _e97.g0_.zzxz) * vec4<f32>(1.0, 0.0, -(1.0), 0.0)) + ((vec4<f32>(_e108.g0_.z) * _e112.g0_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e124.g1_.y) * _e128.g0_.wwwy) * vec4<f32>(0.0, -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e140.g1_.z) * _e144.g0_.zzyz) * vec4<f32>(0.0, 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e156.g0_.x, _e159.g0_.x, _e162.g1_.x, _e165.g1_.x) * _e169.g0_.yxwz) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))));
}

fn line_plane_outer_product(self_856: Line, other_722: Plane) -> Point {
    var self_857: Line;
    var other_723: Plane;

    self_857 = self_856;
    other_723 = other_722;
    let _e4: Line = self_857;
    let _e8: Plane = other_723;
    let _e19: Line = self_857;
    let _e23: Plane = other_723;
    let _e35: Line = self_857;
    let _e39: Plane = other_723;
    let _e51: Line = self_857;
    let _e55: Plane = other_723;
    let _e67: Line = self_857;
    let _e70: Line = self_857;
    let _e73: Line = self_857;
    let _e76: Line = self_857;
    let _e80: Plane = other_723;
    return Point(((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.wwwy) * vec4<f32>(0.0, -(1.0), 0.0, 1.0)) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.zzyz) * vec4<f32>(0.0, 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e35.g1_.y) * _e39.g0_.zzxz) * vec4<f32>(1.0, 0.0, -(1.0), 0.0))) + ((vec4<f32>(_e51.g1_.z) * _e55.g0_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e67.g1_.x, _e70.g1_.x, _e73.g0_.x, _e76.g0_.x) * _e80.g0_.yxwz) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))));
}

fn line_plane_inner_product(self_858: Line, other_724: Plane) -> Plane {
    var self_859: Line;
    var other_725: Plane;

    self_859 = self_858;
    other_725 = other_724;
    let _e4: Line = self_859;
    let _e8: Plane = other_725;
    let _e19: Line = self_859;
    let _e23: Plane = other_725;
    let _e35: Line = self_859;
    let _e39: Plane = other_725;
    let _e51: Line = self_859;
    let _e55: Plane = other_725;
    let _e67: Line = self_859;
    let _e70: Line = self_859;
    let _e73: Line = self_859;
    let _e76: Line = self_859;
    let _e80: Plane = other_725;
    return Plane(((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.zzxz) * vec4<f32>(1.0, 0.0, -(1.0), 0.0)) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e35.g1_.y) * _e39.g0_.wwwy) * vec4<f32>(0.0, -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e51.g1_.z) * _e55.g0_.zzyz) * vec4<f32>(0.0, 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e67.g0_.x, _e70.g0_.x, _e73.g1_.x, _e76.g1_.x) * _e80.g0_.yxwz) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))));
}

fn line_plane_right_contraction(self_860: Line, other_726: Plane) -> Plane {
    var self_861: Line;
    var other_727: Plane;

    self_861 = self_860;
    other_727 = other_726;
    let _e4: Line = self_861;
    let _e8: Plane = other_727;
    let _e19: Line = self_861;
    let _e23: Plane = other_727;
    let _e35: Line = self_861;
    let _e39: Plane = other_727;
    let _e51: Line = self_861;
    let _e55: Plane = other_727;
    let _e67: Line = self_861;
    let _e70: Line = self_861;
    let _e73: Line = self_861;
    let _e76: Line = self_861;
    let _e80: Plane = other_727;
    return Plane(((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.zzxz) * vec4<f32>(1.0, 0.0, -(1.0), 0.0)) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e35.g1_.y) * _e39.g0_.wwwy) * vec4<f32>(0.0, -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e51.g1_.z) * _e55.g0_.zzyz) * vec4<f32>(0.0, 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e67.g0_.x, _e70.g0_.x, _e73.g1_.x, _e76.g1_.x) * _e80.g0_.yxwz) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))));
}

fn line_line_add(self_862: Line, other_728: Line) -> Line {
    var self_863: Line;
    var other_729: Line;

    self_863 = self_862;
    other_729 = other_728;
    let _e4: Line = self_863;
    let _e6: Line = other_729;
    let _e9: Line = self_863;
    let _e11: Line = other_729;
    return Line((_e4.g0_ + _e6.g0_), (_e9.g1_ + _e11.g1_));
}

fn line_line_sub(self_864: Line, other_730: Line) -> Line {
    var self_865: Line;
    var other_731: Line;

    self_865 = self_864;
    other_731 = other_730;
    let _e4: Line = self_865;
    let _e6: Line = other_731;
    let _e9: Line = self_865;
    let _e11: Line = other_731;
    return Line((_e4.g0_ - _e6.g0_), (_e9.g1_ - _e11.g1_));
}

fn line_line_mul(self_866: Line, other_732: Line) -> Line {
    var self_867: Line;
    var other_733: Line;

    self_867 = self_866;
    other_733 = other_732;
    let _e4: Line = self_867;
    let _e6: Line = other_733;
    let _e9: Line = self_867;
    let _e11: Line = other_733;
    return Line((_e4.g0_ * _e6.g0_), (_e9.g1_ * _e11.g1_));
}

fn line_line_div(self_868: Line, other_734: Line) -> Line {
    var self_869: Line;
    var other_735: Line;

    self_869 = self_868;
    other_735 = other_734;
    let _e4: Line = self_869;
    let _e7: Line = self_869;
    let _e10: Line = self_869;
    let _e19: Line = other_735;
    let _e22: Line = other_735;
    let _e25: Line = other_735;
    let _e35: Line = self_869;
    let _e38: Line = self_869;
    let _e41: Line = self_869;
    let _e50: Line = other_735;
    let _e53: Line = other_735;
    let _e56: Line = other_735;
    return Line((((vec3<f32>(_e4.g0_.x, _e7.g0_.y, _e10.g0_.z) * vec3<f32>(1.0, 1.0, 1.0)) / vec3<f32>(_e19.g0_.x, _e22.g0_.y, _e25.g0_.z)) * vec3<f32>(1.0, 1.0, 1.0)), (((vec3<f32>(_e35.g1_.x, _e38.g1_.y, _e41.g1_.z) * vec3<f32>(1.0, 1.0, 1.0)) / vec3<f32>(_e50.g1_.x, _e53.g1_.y, _e56.g1_.z)) * vec3<f32>(1.0, 1.0, 1.0)));
}

fn line_line_geometric_product(self_870: Line, other_736: Line) -> Motor {
    var self_871: Line;
    var other_737: Line;

    self_871 = self_870;
    other_737 = other_736;
    let _e4: Line = self_871;
    let _e8: Line = other_737;
    let _e11: Line = other_737;
    let _e14: Line = other_737;
    let _e17: Line = other_737;
    let _e30: Line = self_871;
    let _e34: Line = other_737;
    let _e37: Line = other_737;
    let _e40: Line = other_737;
    let _e43: Line = other_737;
    let _e57: Line = self_871;
    let _e61: Line = other_737;
    let _e64: Line = other_737;
    let _e67: Line = other_737;
    let _e70: Line = other_737;
    let _e84: Line = self_871;
    let _e88: Line = other_737;
    let _e91: Line = other_737;
    let _e94: Line = other_737;
    let _e97: Line = other_737;
    let _e111: Line = self_871;
    let _e115: Line = other_737;
    let _e118: Line = other_737;
    let _e121: Line = other_737;
    let _e124: Line = other_737;
    let _e138: Line = self_871;
    let _e142: Line = other_737;
    let _e145: Line = other_737;
    let _e148: Line = other_737;
    let _e151: Line = other_737;
    let _e165: Line = self_871;
    let _e169: Line = other_737;
    let _e172: Line = other_737;
    let _e175: Line = other_737;
    let _e178: Line = other_737;
    let _e190: Line = self_871;
    let _e194: Line = other_737;
    let _e197: Line = other_737;
    let _e200: Line = other_737;
    let _e203: Line = other_737;
    let _e216: Line = self_871;
    let _e220: Line = other_737;
    let _e223: Line = other_737;
    let _e226: Line = other_737;
    let _e229: Line = other_737;
    let _e242: Line = self_871;
    let _e246: Line = other_737;
    let _e249: Line = other_737;
    let _e252: Line = other_737;
    let _e255: Line = other_737;
    let _e268: Line = self_871;
    let _e272: Line = other_737;
    let _e275: Line = other_737;
    let _e278: Line = other_737;
    let _e281: Line = other_737;
    let _e294: Line = self_871;
    let _e298: Line = other_737;
    let _e301: Line = other_737;
    let _e304: Line = other_737;
    let _e307: Line = other_737;
    return Motor((((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y, _e11.g0_.z, _e14.g0_.y, _e17.g0_.x)) * vec4<f32>(-(1.0), -(1.0), 0.0, 1.0)) + ((vec4<f32>(_e30.g0_.z) * vec4<f32>(_e34.g0_.z, _e37.g0_.y, _e40.g0_.x, _e43.g0_.z)) * vec4<f32>(-(1.0), 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e57.g1_.x) * vec4<f32>(_e61.g1_.x, _e64.g1_.x, _e67.g1_.z, _e70.g1_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e84.g1_.y) * vec4<f32>(_e88.g1_.y, _e91.g1_.z, _e94.g1_.y, _e97.g1_.x)) * vec4<f32>(-(1.0), -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e111.g1_.z) * vec4<f32>(_e115.g1_.z, _e118.g1_.y, _e121.g1_.x, _e124.g1_.z)) * vec4<f32>(-(1.0), 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e138.g0_.x) * vec4<f32>(_e142.g0_.x, _e145.g0_.x, _e148.g0_.z, _e151.g0_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0)))), (((((((vec4<f32>(_e165.g0_.y) * vec4<f32>(_e169.g1_.y, _e172.g1_.z, _e175.g1_.y, _e178.g1_.x)) * vec4<f32>(1.0, -(1.0), 0.0, 1.0)) + ((vec4<f32>(_e190.g0_.z) * vec4<f32>(_e194.g1_.z, _e197.g1_.y, _e200.g1_.x, _e203.g1_.z)) * vec4<f32>(1.0, 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e216.g1_.x) * vec4<f32>(_e220.g0_.x, _e223.g0_.x, _e226.g0_.z, _e229.g0_.y)) * vec4<f32>(1.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e242.g1_.y) * vec4<f32>(_e246.g0_.y, _e249.g0_.z, _e252.g0_.y, _e255.g0_.x)) * vec4<f32>(1.0, -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e268.g1_.z) * vec4<f32>(_e272.g0_.z, _e275.g0_.y, _e278.g0_.x, _e281.g0_.z)) * vec4<f32>(1.0, 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e294.g0_.x) * vec4<f32>(_e298.g1_.x, _e301.g1_.x, _e304.g1_.z, _e307.g1_.y)) * vec4<f32>(1.0, 0.0, 1.0, -(1.0)))));
}

fn line_line_regressive_product(self_872: Line, other_738: Line) -> Scalar {
    var self_873: Line;
    var other_739: Line;

    self_873 = self_872;
    other_739 = other_738;
    let _e4: Line = self_873;
    let _e7: Line = other_739;
    let _e11: Line = self_873;
    let _e14: Line = other_739;
    let _e19: Line = self_873;
    let _e22: Line = other_739;
    let _e27: Line = self_873;
    let _e30: Line = other_739;
    let _e35: Line = self_873;
    let _e38: Line = other_739;
    let _e43: Line = self_873;
    let _e46: Line = other_739;
    return Scalar(((((((_e4.g0_.x * _e7.g1_.x) + (_e11.g0_.y * _e14.g1_.y)) + (_e19.g0_.z * _e22.g1_.z)) + (_e27.g1_.x * _e30.g0_.x)) + (_e35.g1_.y * _e38.g0_.y)) + (_e43.g1_.z * _e46.g0_.z)));
}

fn line_line_inner_product(self_874: Line, other_740: Line) -> Scalar {
    var self_875: Line;
    var other_741: Line;

    self_875 = self_874;
    other_741 = other_740;
    let _e5: Line = self_875;
    let _e8: Line = other_741;
    let _e13: Line = self_875;
    let _e16: Line = other_741;
    let _e21: Line = self_875;
    let _e24: Line = other_741;
    let _e29: Line = self_875;
    let _e32: Line = other_741;
    let _e37: Line = self_875;
    let _e40: Line = other_741;
    let _e45: Line = self_875;
    let _e48: Line = other_741;
    return Scalar(((((((0.0 - (_e5.g0_.x * _e8.g0_.x)) - (_e13.g0_.y * _e16.g0_.y)) - (_e21.g0_.z * _e24.g0_.z)) - (_e29.g1_.x * _e32.g1_.x)) - (_e37.g1_.y * _e40.g1_.y)) - (_e45.g1_.z * _e48.g1_.z)));
}

fn line_line_left_contraction(self_876: Line, other_742: Line) -> Scalar {
    var self_877: Line;
    var other_743: Line;

    self_877 = self_876;
    other_743 = other_742;
    let _e5: Line = self_877;
    let _e8: Line = other_743;
    let _e13: Line = self_877;
    let _e16: Line = other_743;
    let _e21: Line = self_877;
    let _e24: Line = other_743;
    let _e29: Line = self_877;
    let _e32: Line = other_743;
    let _e37: Line = self_877;
    let _e40: Line = other_743;
    let _e45: Line = self_877;
    let _e48: Line = other_743;
    return Scalar(((((((0.0 - (_e5.g0_.x * _e8.g0_.x)) - (_e13.g0_.y * _e16.g0_.y)) - (_e21.g0_.z * _e24.g0_.z)) - (_e29.g1_.x * _e32.g1_.x)) - (_e37.g1_.y * _e40.g1_.y)) - (_e45.g1_.z * _e48.g1_.z)));
}

fn line_line_right_contraction(self_878: Line, other_744: Line) -> Scalar {
    var self_879: Line;
    var other_745: Line;

    self_879 = self_878;
    other_745 = other_744;
    let _e5: Line = self_879;
    let _e8: Line = other_745;
    let _e13: Line = self_879;
    let _e16: Line = other_745;
    let _e21: Line = self_879;
    let _e24: Line = other_745;
    let _e29: Line = self_879;
    let _e32: Line = other_745;
    let _e37: Line = self_879;
    let _e40: Line = other_745;
    let _e45: Line = self_879;
    let _e48: Line = other_745;
    return Scalar(((((((0.0 - (_e5.g0_.x * _e8.g0_.x)) - (_e13.g0_.y * _e16.g0_.y)) - (_e21.g0_.z * _e24.g0_.z)) - (_e29.g1_.x * _e32.g1_.x)) - (_e37.g1_.y * _e40.g1_.y)) - (_e45.g1_.z * _e48.g1_.z)));
}

fn line_line_scalar_product(self_880: Line, other_746: Line) -> Scalar {
    var self_881: Line;
    var other_747: Line;

    self_881 = self_880;
    other_747 = other_746;
    let _e5: Line = self_881;
    let _e8: Line = other_747;
    let _e13: Line = self_881;
    let _e16: Line = other_747;
    let _e21: Line = self_881;
    let _e24: Line = other_747;
    let _e29: Line = self_881;
    let _e32: Line = other_747;
    let _e37: Line = self_881;
    let _e40: Line = other_747;
    let _e45: Line = self_881;
    let _e48: Line = other_747;
    return Scalar(((((((0.0 - (_e5.g0_.x * _e8.g0_.x)) - (_e13.g0_.y * _e16.g0_.y)) - (_e21.g0_.z * _e24.g0_.z)) - (_e29.g1_.x * _e32.g1_.x)) - (_e37.g1_.y * _e40.g1_.y)) - (_e45.g1_.z * _e48.g1_.z)));
}

fn line_translator_geometric_product(self_882: Line, other_748: Translator) -> Motor {
    var self_883: Line;
    var other_749: Translator;

    self_883 = self_882;
    other_749 = other_748;
    let _e4: Line = self_883;
    let _e8: Translator = other_749;
    let _e20: Line = self_883;
    let _e24: Translator = other_749;
    let _e37: Line = self_883;
    let _e41: Translator = other_749;
    let _e53: Line = self_883;
    let _e57: Translator = other_749;
    let _e69: Line = self_883;
    let _e72: Line = self_883;
    let _e75: Line = self_883;
    let _e78: Line = self_883;
    let _e82: Translator = other_749;
    let _e95: Line = self_883;
    let _e99: Translator = other_749;
    let _e110: Line = self_883;
    let _e114: Translator = other_749;
    let _e126: Line = self_883;
    let _e130: Translator = other_749;
    let _e142: Line = self_883;
    let _e145: Line = self_883;
    let _e148: Line = self_883;
    let _e151: Line = self_883;
    let _e155: Translator = other_749;
    return Motor(((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.zwzy) * vec4<f32>(-(1.0), -(1.0), 0.0, 1.0)) + ((vec4<f32>(_e20.g0_.z) * _e24.g0_.wzyw) * vec4<f32>(-(1.0), 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e37.g1_.y) * vec4<f32>(_e41.g0_.x)) * vec4<f32>(0.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e53.g1_.z) * vec4<f32>(_e57.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e69.g0_.x, _e72.g1_.x, _e75.g0_.x, _e78.g0_.x) * _e82.g0_.yxwz) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))), (((((vec4<f32>(_e95.g1_.x) * _e99.g0_.yywz) * vec4<f32>(1.0, 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e110.g1_.y) * _e114.g0_.zwzy) * vec4<f32>(1.0, -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e126.g1_.z) * _e130.g0_.wzyw) * vec4<f32>(1.0, 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e142.g0_.x, _e145.g0_.x, _e148.g0_.y, _e151.g0_.z) * vec4<f32>(_e155.g0_.x)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn line_translator_regressive_product(self_884: Line, other_750: Translator) -> Scalar {
    var self_885: Line;
    var other_751: Translator;

    self_885 = self_884;
    other_751 = other_750;
    let _e4: Line = self_885;
    let _e7: Translator = other_751;
    let _e11: Line = self_885;
    let _e14: Translator = other_751;
    let _e19: Line = self_885;
    let _e22: Translator = other_751;
    return Scalar((((_e4.g1_.x * _e7.g0_.y) + (_e11.g1_.y * _e14.g0_.z)) + (_e19.g1_.z * _e22.g0_.w)));
}

fn line_translator_left_contraction(self_886: Line, other_752: Translator) -> Scalar {
    var self_887: Line;
    var other_753: Translator;

    self_887 = self_886;
    other_753 = other_752;
    let _e5: Line = self_887;
    let _e8: Translator = other_753;
    let _e13: Line = self_887;
    let _e16: Translator = other_753;
    let _e21: Line = self_887;
    let _e24: Translator = other_753;
    return Scalar((((0.0 - (_e5.g0_.x * _e8.g0_.y)) - (_e13.g0_.y * _e16.g0_.z)) - (_e21.g0_.z * _e24.g0_.w)));
}

fn line_translator_scalar_product(self_888: Line, other_754: Translator) -> Scalar {
    var self_889: Line;
    var other_755: Translator;

    self_889 = self_888;
    other_755 = other_754;
    let _e5: Line = self_889;
    let _e8: Translator = other_755;
    let _e13: Line = self_889;
    let _e16: Translator = other_755;
    let _e21: Line = self_889;
    let _e24: Translator = other_755;
    return Scalar((((0.0 - (_e5.g0_.x * _e8.g0_.y)) - (_e13.g0_.y * _e16.g0_.z)) - (_e21.g0_.z * _e24.g0_.w)));
}

fn line_motor_add(self_890: Line, other_756: Motor) -> Motor {
    var self_891: Line;
    var other_757: Motor;

    self_891 = self_890;
    other_757 = other_756;
    let _e4: Line = self_891;
    let _e7: Line = self_891;
    let _e10: Line = self_891;
    let _e13: Line = self_891;
    let _e23: Motor = other_757;
    let _e26: Line = self_891;
    let _e29: Line = self_891;
    let _e32: Line = self_891;
    let _e35: Line = self_891;
    let _e45: Motor = other_757;
    return Motor(((vec4<f32>(_e4.g0_.x, _e7.g1_.x, _e10.g1_.y, _e13.g1_.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0)) + _e23.g0_), ((vec4<f32>(_e26.g0_.x, _e29.g0_.x, _e32.g0_.y, _e35.g0_.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0)) + _e45.g1_));
}

fn line_motor_sub(self_892: Line, other_758: Motor) -> Motor {
    var self_893: Line;
    var other_759: Motor;

    self_893 = self_892;
    other_759 = other_758;
    let _e4: Line = self_893;
    let _e7: Line = self_893;
    let _e10: Line = self_893;
    let _e13: Line = self_893;
    let _e23: Motor = other_759;
    let _e26: Line = self_893;
    let _e29: Line = self_893;
    let _e32: Line = self_893;
    let _e35: Line = self_893;
    let _e45: Motor = other_759;
    return Motor(((vec4<f32>(_e4.g0_.x, _e7.g1_.x, _e10.g1_.y, _e13.g1_.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0)) - _e23.g0_), ((vec4<f32>(_e26.g0_.x, _e29.g0_.x, _e32.g0_.y, _e35.g0_.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0)) - _e45.g1_));
}

fn line_motor_geometric_product(self_894: Line, other_760: Motor) -> Motor {
    var self_895: Line;
    var other_761: Motor;

    self_895 = self_894;
    other_761 = other_760;
    let _e4: Line = self_895;
    let _e8: Motor = other_761;
    let _e21: Line = self_895;
    let _e25: Motor = other_761;
    let _e39: Line = self_895;
    let _e43: Motor = other_761;
    let _e57: Line = self_895;
    let _e61: Motor = other_761;
    let _e74: Line = self_895;
    let _e78: Motor = other_761;
    let _e91: Line = self_895;
    let _e95: Motor = other_761;
    let _e108: Line = self_895;
    let _e112: Motor = other_761;
    let _e123: Line = self_895;
    let _e127: Motor = other_761;
    let _e139: Line = self_895;
    let _e143: Motor = other_761;
    let _e155: Line = self_895;
    let _e159: Motor = other_761;
    let _e172: Line = self_895;
    let _e176: Motor = other_761;
    let _e189: Line = self_895;
    let _e193: Motor = other_761;
    return Motor((((((((vec4<f32>(_e4.g0_.x) * _e8.g1_.yxwz) * vec4<f32>(-(1.0), -(1.0), 1.0, -(1.0))) + ((vec4<f32>(_e21.g0_.y) * _e25.g1_.zwxy) * vec4<f32>(-(1.0), -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e39.g0_.z) * _e43.g1_.wzyx) * vec4<f32>(-(1.0), 1.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e57.g1_.x) * _e61.g0_.yxwz) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e74.g1_.y) * _e78.g0_.zwxy) * vec4<f32>(-(1.0), -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e91.g1_.z) * _e95.g0_.wzyx) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))), (((((((vec4<f32>(_e108.g0_.x) * _e112.g0_.yxwz) * vec4<f32>(1.0, 1.0, 1.0, -(1.0))) + ((vec4<f32>(_e123.g0_.y) * _e127.g0_.zwxy) * vec4<f32>(1.0, -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e139.g0_.z) * _e143.g0_.wzyx) * vec4<f32>(1.0, 1.0, -(1.0), 1.0))) + ((vec4<f32>(_e155.g1_.x) * _e159.g1_.yxwz) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e172.g1_.y) * _e176.g1_.zwxy) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e189.g1_.z) * _e193.g1_.wzyx) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))));
}

fn line_motor_scalar_product(self_896: Line, other_762: Motor) -> Scalar {
    var self_897: Line;
    var other_763: Motor;

    self_897 = self_896;
    other_763 = other_762;
    let _e5: Line = self_897;
    let _e8: Motor = other_763;
    let _e13: Line = self_897;
    let _e16: Motor = other_763;
    let _e21: Line = self_897;
    let _e24: Motor = other_763;
    let _e29: Line = self_897;
    let _e32: Motor = other_763;
    let _e37: Line = self_897;
    let _e40: Motor = other_763;
    let _e45: Line = self_897;
    let _e48: Motor = other_763;
    return Scalar(((((((0.0 - (_e5.g0_.x * _e8.g1_.y)) - (_e13.g0_.y * _e16.g1_.z)) - (_e21.g0_.z * _e24.g1_.w)) - (_e29.g1_.x * _e32.g0_.y)) - (_e37.g1_.y * _e40.g0_.z)) - (_e45.g1_.z * _e48.g0_.w)));
}

fn line_point_and_plane_geometric_product(self_898: Line, other_764: PointAndPlane) -> PointAndPlane {
    var self_899: Line;
    var other_765: PointAndPlane;

    self_899 = self_898;
    other_765 = other_764;
    let _e4: Line = self_899;
    let _e8: PointAndPlane = other_765;
    let _e11: PointAndPlane = other_765;
    let _e14: PointAndPlane = other_765;
    let _e17: PointAndPlane = other_765;
    let _e30: Line = self_899;
    let _e34: PointAndPlane = other_765;
    let _e37: PointAndPlane = other_765;
    let _e40: PointAndPlane = other_765;
    let _e43: PointAndPlane = other_765;
    let _e57: Line = self_899;
    let _e61: PointAndPlane = other_765;
    let _e64: PointAndPlane = other_765;
    let _e67: PointAndPlane = other_765;
    let _e70: PointAndPlane = other_765;
    let _e84: Line = self_899;
    let _e88: PointAndPlane = other_765;
    let _e91: PointAndPlane = other_765;
    let _e94: PointAndPlane = other_765;
    let _e97: PointAndPlane = other_765;
    let _e111: Line = self_899;
    let _e115: PointAndPlane = other_765;
    let _e118: PointAndPlane = other_765;
    let _e121: PointAndPlane = other_765;
    let _e124: PointAndPlane = other_765;
    let _e138: Line = self_899;
    let _e142: PointAndPlane = other_765;
    let _e145: PointAndPlane = other_765;
    let _e148: PointAndPlane = other_765;
    let _e151: PointAndPlane = other_765;
    let _e165: Line = self_899;
    let _e169: PointAndPlane = other_765;
    let _e172: PointAndPlane = other_765;
    let _e175: PointAndPlane = other_765;
    let _e178: PointAndPlane = other_765;
    let _e191: Line = self_899;
    let _e195: PointAndPlane = other_765;
    let _e198: PointAndPlane = other_765;
    let _e201: PointAndPlane = other_765;
    let _e204: PointAndPlane = other_765;
    let _e218: Line = self_899;
    let _e222: PointAndPlane = other_765;
    let _e225: PointAndPlane = other_765;
    let _e228: PointAndPlane = other_765;
    let _e231: PointAndPlane = other_765;
    let _e245: Line = self_899;
    let _e249: PointAndPlane = other_765;
    let _e252: PointAndPlane = other_765;
    let _e255: PointAndPlane = other_765;
    let _e258: PointAndPlane = other_765;
    let _e272: Line = self_899;
    let _e276: PointAndPlane = other_765;
    let _e279: PointAndPlane = other_765;
    let _e282: PointAndPlane = other_765;
    let _e285: PointAndPlane = other_765;
    let _e299: Line = self_899;
    let _e303: PointAndPlane = other_765;
    let _e306: PointAndPlane = other_765;
    let _e309: PointAndPlane = other_765;
    let _e312: PointAndPlane = other_765;
    return PointAndPlane((((((((vec4<f32>(_e4.g0_.x) * vec4<f32>(_e8.g0_.y, _e11.g0_.x, _e14.g1_.w, _e17.g1_.z)) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0))) + ((vec4<f32>(_e30.g0_.y) * vec4<f32>(_e34.g0_.z, _e37.g1_.w, _e40.g0_.x, _e43.g1_.y)) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e57.g0_.z) * vec4<f32>(_e61.g0_.w, _e64.g1_.z, _e67.g1_.y, _e70.g0_.x)) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e84.g1_.x) * vec4<f32>(_e88.g1_.y, _e91.g1_.x, _e94.g0_.w, _e97.g0_.z)) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e111.g1_.y) * vec4<f32>(_e115.g1_.z, _e118.g0_.w, _e121.g1_.x, _e124.g0_.y)) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e138.g1_.z) * vec4<f32>(_e142.g1_.w, _e145.g0_.z, _e148.g0_.y, _e151.g1_.x)) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))), (((((((vec4<f32>(_e165.g0_.x) * vec4<f32>(_e169.g1_.y, _e172.g1_.x, _e175.g0_.w, _e178.g0_.z)) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0))) + ((vec4<f32>(_e191.g0_.y) * vec4<f32>(_e195.g1_.z, _e198.g0_.w, _e201.g1_.x, _e204.g0_.y)) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e218.g0_.z) * vec4<f32>(_e222.g1_.w, _e225.g0_.z, _e228.g0_.y, _e231.g1_.x)) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e245.g1_.x) * vec4<f32>(_e249.g0_.y, _e252.g0_.x, _e255.g1_.w, _e258.g1_.z)) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e272.g1_.y) * vec4<f32>(_e276.g0_.z, _e279.g1_.w, _e282.g0_.x, _e285.g1_.y)) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e299.g1_.z) * vec4<f32>(_e303.g0_.w, _e306.g1_.z, _e309.g1_.y, _e312.g0_.x)) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))));
}

fn line_point_and_plane_regressive_product(self_900: Line, other_766: PointAndPlane) -> Plane {
    var self_901: Line;
    var other_767: PointAndPlane;

    self_901 = self_900;
    other_767 = other_766;
    let _e4: Line = self_901;
    let _e8: PointAndPlane = other_767;
    let _e19: Line = self_901;
    let _e23: PointAndPlane = other_767;
    let _e35: Line = self_901;
    let _e39: PointAndPlane = other_767;
    let _e51: Line = self_901;
    let _e55: PointAndPlane = other_767;
    let _e67: Line = self_901;
    let _e70: Line = self_901;
    let _e73: Line = self_901;
    let _e76: Line = self_901;
    let _e80: PointAndPlane = other_767;
    return Plane(((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.zzxz) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0)) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.wwwx) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))) + ((vec4<f32>(_e35.g1_.y) * _e39.g0_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0)))) + ((vec4<f32>(_e51.g1_.z) * _e55.g0_.zzyz) * vec4<f32>(0.0, -(1.0), 1.0, 0.0))) + ((vec4<f32>(_e67.g0_.x, _e70.g0_.x, _e73.g1_.x, _e76.g1_.x) * _e80.g0_.yxwz) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))));
}

fn line_point_and_plane_outer_product(self_902: Line, other_768: PointAndPlane) -> Point {
    var self_903: Line;
    var other_769: PointAndPlane;

    self_903 = self_902;
    other_769 = other_768;
    let _e4: Line = self_903;
    let _e8: PointAndPlane = other_769;
    let _e19: Line = self_903;
    let _e23: PointAndPlane = other_769;
    let _e35: Line = self_903;
    let _e39: PointAndPlane = other_769;
    let _e51: Line = self_903;
    let _e55: PointAndPlane = other_769;
    let _e67: Line = self_903;
    let _e70: Line = self_903;
    let _e73: Line = self_903;
    let _e76: Line = self_903;
    let _e80: PointAndPlane = other_769;
    return Point(((((((vec4<f32>(_e4.g0_.y) * _e8.g1_.wwwy) * vec4<f32>(0.0, -(1.0), 0.0, 1.0)) + ((vec4<f32>(_e19.g0_.z) * _e23.g1_.zzyz) * vec4<f32>(0.0, 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e35.g1_.y) * _e39.g1_.zzxz) * vec4<f32>(1.0, 0.0, -(1.0), 0.0))) + ((vec4<f32>(_e51.g1_.z) * _e55.g1_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e67.g1_.x, _e70.g1_.x, _e73.g0_.x, _e76.g0_.x) * _e80.g1_.yxwz) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))));
}

fn line_point_and_plane_inner_product(self_904: Line, other_770: PointAndPlane) -> Plane {
    var self_905: Line;
    var other_771: PointAndPlane;

    self_905 = self_904;
    other_771 = other_770;
    let _e4: Line = self_905;
    let _e8: PointAndPlane = other_771;
    let _e11: PointAndPlane = other_771;
    let _e14: PointAndPlane = other_771;
    let _e17: PointAndPlane = other_771;
    let _e30: Line = self_905;
    let _e34: PointAndPlane = other_771;
    let _e37: PointAndPlane = other_771;
    let _e40: PointAndPlane = other_771;
    let _e43: PointAndPlane = other_771;
    let _e57: Line = self_905;
    let _e61: PointAndPlane = other_771;
    let _e64: PointAndPlane = other_771;
    let _e67: PointAndPlane = other_771;
    let _e70: PointAndPlane = other_771;
    let _e84: Line = self_905;
    let _e88: PointAndPlane = other_771;
    let _e91: PointAndPlane = other_771;
    let _e94: PointAndPlane = other_771;
    let _e97: PointAndPlane = other_771;
    let _e111: Line = self_905;
    let _e115: PointAndPlane = other_771;
    let _e118: PointAndPlane = other_771;
    let _e121: PointAndPlane = other_771;
    let _e124: PointAndPlane = other_771;
    let _e138: Line = self_905;
    let _e142: PointAndPlane = other_771;
    let _e145: PointAndPlane = other_771;
    let _e148: PointAndPlane = other_771;
    let _e151: PointAndPlane = other_771;
    return Plane((((((((vec4<f32>(_e4.g0_.x) * vec4<f32>(_e8.g1_.y, _e11.g1_.x, _e14.g0_.w, _e17.g0_.z)) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0))) + ((vec4<f32>(_e30.g0_.y) * vec4<f32>(_e34.g1_.z, _e37.g0_.w, _e40.g1_.x, _e43.g0_.y)) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e57.g0_.z) * vec4<f32>(_e61.g1_.w, _e64.g0_.z, _e67.g0_.y, _e70.g1_.x)) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e84.g1_.x) * vec4<f32>(_e88.g0_.y, _e91.g0_.x, _e94.g1_.w, _e97.g1_.z)) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e111.g1_.y) * vec4<f32>(_e115.g0_.z, _e118.g1_.w, _e121.g0_.x, _e124.g1_.y)) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e138.g1_.z) * vec4<f32>(_e142.g0_.w, _e145.g1_.z, _e148.g1_.y, _e151.g0_.x)) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))));
}

fn line_point_and_plane_left_contraction(self_906: Line, other_772: PointAndPlane) -> Plane {
    var self_907: Line;
    var other_773: PointAndPlane;

    self_907 = self_906;
    other_773 = other_772;
    let _e4: Line = self_907;
    let _e8: PointAndPlane = other_773;
    let _e19: Line = self_907;
    let _e23: PointAndPlane = other_773;
    let _e35: Line = self_907;
    let _e39: PointAndPlane = other_773;
    let _e51: Line = self_907;
    let _e55: PointAndPlane = other_773;
    let _e67: Line = self_907;
    let _e70: Line = self_907;
    let _e73: Line = self_907;
    let _e76: Line = self_907;
    let _e80: PointAndPlane = other_773;
    return Plane(((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.wwwy) * vec4<f32>(0.0, -(1.0), 0.0, 1.0)) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.zzyz) * vec4<f32>(0.0, 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e35.g1_.y) * _e39.g0_.zzxz) * vec4<f32>(1.0, 0.0, -(1.0), 0.0))) + ((vec4<f32>(_e51.g1_.z) * _e55.g0_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e67.g1_.x, _e70.g1_.x, _e73.g0_.x, _e76.g0_.x) * _e80.g0_.yxwz) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))));
}

fn line_point_and_plane_right_contraction(self_908: Line, other_774: PointAndPlane) -> Plane {
    var self_909: Line;
    var other_775: PointAndPlane;

    self_909 = self_908;
    other_775 = other_774;
    let _e4: Line = self_909;
    let _e8: PointAndPlane = other_775;
    let _e19: Line = self_909;
    let _e23: PointAndPlane = other_775;
    let _e35: Line = self_909;
    let _e39: PointAndPlane = other_775;
    let _e51: Line = self_909;
    let _e55: PointAndPlane = other_775;
    let _e67: Line = self_909;
    let _e70: Line = self_909;
    let _e73: Line = self_909;
    let _e76: Line = self_909;
    let _e80: PointAndPlane = other_775;
    return Plane(((((((vec4<f32>(_e4.g0_.y) * _e8.g1_.zzxz) * vec4<f32>(1.0, 0.0, -(1.0), 0.0)) + ((vec4<f32>(_e19.g0_.z) * _e23.g1_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e35.g1_.y) * _e39.g1_.wwwy) * vec4<f32>(0.0, -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e51.g1_.z) * _e55.g1_.zzyz) * vec4<f32>(0.0, 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e67.g0_.x, _e70.g0_.x, _e73.g1_.x, _e76.g1_.x) * _e80.g1_.yxwz) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))));
}

fn line_squared_magnitude(self_910: Line) -> Scalar {
    var self_911: Line;

    self_911 = self_910;
    let _e4: Line = self_911;
    let _e5: Line = line_reversal(_e4);
    let _e6: Line = self_911;
    let _e8: Line = self_911;
    let _e9: Line = line_reversal(_e8);
    let _e10: Scalar = line_line_scalar_product(_e6, _e9);
    return _e10;
}

fn line_magnitude(self_912: Line) -> Scalar {
    var self_913: Line;

    self_913 = self_912;
    let _e3: Line = self_913;
    let _e4: Scalar = line_squared_magnitude(_e3);
    let _e7: Line = self_913;
    let _e8: Scalar = line_squared_magnitude(_e7);
    return Scalar(sqrt(_e8.g0_));
}

fn line_scale(self_914: Line, other_776: f32) -> Line {
    var self_915: Line;
    var other_777: f32;

    self_915 = self_914;
    other_777 = other_776;
    let _e5: f32 = other_777;
    let _e7: Line = self_915;
    let _e8: f32 = other_777;
    let _e10: Line = line_scalar_geometric_product(_e7, Scalar(_e8));
    return _e10;
}

fn line_signum(self_916: Line) -> Line {
    var self_917: Line;

    self_917 = self_916;
    let _e5: Line = self_917;
    let _e6: Scalar = line_magnitude(_e5);
    let _e10: Line = self_917;
    let _e13: Line = self_917;
    let _e14: Scalar = line_magnitude(_e13);
    let _e18: Line = line_scalar_geometric_product(_e10, Scalar((1.0 / _e14.g0_)));
    return _e18;
}

fn line_inverse(self_918: Line) -> Line {
    var self_919: Line;

    self_919 = self_918;
    let _e3: Line = self_919;
    let _e4: Line = line_reversal(_e3);
    let _e7: Line = self_919;
    let _e8: Scalar = line_squared_magnitude(_e7);
    let _e13: Line = self_919;
    let _e14: Line = line_reversal(_e13);
    let _e17: Line = self_919;
    let _e18: Scalar = line_squared_magnitude(_e17);
    let _e22: Line = line_scalar_geometric_product(_e14, Scalar((1.0 / _e18.g0_)));
    return _e22;
}

fn translator_zero() -> Translator {
    return Translator(vec4<f32>(0.0));
}

fn translator_one() -> Translator {
    return Translator(vec4<f32>(1.0, 0.0, 0.0, 0.0));
}

fn translator_neg(self_920: Translator) -> Translator {
    var self_921: Translator;

    self_921 = self_920;
    let _e2: Translator = self_921;
    return Translator((_e2.g0_ * vec4<f32>(-(1.0))));
}

fn translator_automorphism(self_922: Translator) -> Translator {
    var self_923: Translator;

    self_923 = self_922;
    let _e2: Translator = self_923;
    return Translator(_e2.g0_);
}

fn translator_reversal(self_924: Translator) -> Translator {
    var self_925: Translator;

    self_925 = self_924;
    let _e2: Translator = self_925;
    return Translator((_e2.g0_ * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))));
}

fn translator_conjugation(self_926: Translator) -> Translator {
    var self_927: Translator;

    self_927 = self_926;
    let _e2: Translator = self_927;
    return Translator((_e2.g0_ * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))));
}

fn translator_scalar_into(self_928: Translator) -> Scalar {
    var self_929: Translator;

    self_929 = self_928;
    let _e2: Translator = self_929;
    return Scalar(_e2.g0_.x);
}

fn translator_scalar_add(self_930: Translator, other_778: Scalar) -> Translator {
    var self_931: Translator;
    var other_779: Scalar;

    self_931 = self_930;
    other_779 = other_778;
    let _e4: Translator = self_931;
    let _e6: Scalar = other_779;
    return Translator((_e4.g0_ + (vec4<f32>(_e6.g0_) * vec4<f32>(1.0, 0.0, 0.0, 0.0))));
}

fn translator_scalar_sub(self_932: Translator, other_780: Scalar) -> Translator {
    var self_933: Translator;
    var other_781: Scalar;

    self_933 = self_932;
    other_781 = other_780;
    let _e4: Translator = self_933;
    let _e6: Scalar = other_781;
    return Translator((_e4.g0_ - (vec4<f32>(_e6.g0_) * vec4<f32>(1.0, 0.0, 0.0, 0.0))));
}

fn translator_scalar_geometric_product(self_934: Translator, other_782: Scalar) -> Translator {
    var self_935: Translator;
    var other_783: Scalar;

    self_935 = self_934;
    other_783 = other_782;
    let _e4: Translator = self_935;
    let _e6: Scalar = other_783;
    return Translator((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn translator_scalar_outer_product(self_936: Translator, other_784: Scalar) -> Translator {
    var self_937: Translator;
    var other_785: Scalar;

    self_937 = self_936;
    other_785 = other_784;
    let _e4: Translator = self_937;
    let _e6: Scalar = other_785;
    return Translator((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn translator_scalar_inner_product(self_938: Translator, other_786: Scalar) -> Translator {
    var self_939: Translator;
    var other_787: Scalar;

    self_939 = self_938;
    other_787 = other_786;
    let _e4: Translator = self_939;
    let _e6: Scalar = other_787;
    return Translator((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn translator_scalar_left_contraction(self_940: Translator, other_788: Scalar) -> Scalar {
    var self_941: Translator;
    var other_789: Scalar;

    self_941 = self_940;
    other_789 = other_788;
    let _e4: Translator = self_941;
    let _e7: Scalar = other_789;
    return Scalar((_e4.g0_.x * _e7.g0_));
}

fn translator_scalar_right_contraction(self_942: Translator, other_790: Scalar) -> Translator {
    var self_943: Translator;
    var other_791: Scalar;

    self_943 = self_942;
    other_791 = other_790;
    let _e4: Translator = self_943;
    let _e6: Scalar = other_791;
    return Translator((_e4.g0_ * vec4<f32>(_e6.g0_)));
}

fn translator_scalar_scalar_product(self_944: Translator, other_792: Scalar) -> Scalar {
    var self_945: Translator;
    var other_793: Scalar;

    self_945 = self_944;
    other_793 = other_792;
    let _e4: Translator = self_945;
    let _e7: Scalar = other_793;
    return Scalar((_e4.g0_.x * _e7.g0_));
}

fn translator_multi_vector_add(self_946: Translator, other_794: MultiVector) -> MultiVector {
    var self_947: Translator;
    var other_795: MultiVector;

    self_947 = self_946;
    other_795 = other_794;
    let _e4: Translator = self_947;
    let _e14: MultiVector = other_795;
    let _e17: MultiVector = other_795;
    let _e19: MultiVector = other_795;
    let _e21: Translator = self_947;
    let _e29: MultiVector = other_795;
    return MultiVector(((vec4<f32>(_e4.g0_.x) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) + _e14.g0_), _e17.g1_, _e19.g2_, ((_e21.g0_ * vec4<f32>(0.0, 1.0, 1.0, 1.0)) + _e29.g3_));
}

fn translator_multi_vector_sub(self_948: Translator, other_796: MultiVector) -> MultiVector {
    var self_949: Translator;
    var other_797: MultiVector;

    self_949 = self_948;
    other_797 = other_796;
    let _e4: Translator = self_949;
    let _e14: MultiVector = other_797;
    let _e19: MultiVector = other_797;
    let _e24: MultiVector = other_797;
    let _e27: Translator = self_949;
    let _e35: MultiVector = other_797;
    return MultiVector(((vec4<f32>(_e4.g0_.x) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) - _e14.g0_), (vec4<f32>(0.0) - _e19.g1_), (vec4<f32>(0.0) - _e24.g2_), ((_e27.g0_ * vec4<f32>(0.0, 1.0, 1.0, 1.0)) - _e35.g3_));
}

fn translator_multi_vector_geometric_product(self_950: Translator, other_798: MultiVector) -> MultiVector {
    var self_951: Translator;
    var other_799: MultiVector;

    self_951 = self_950;
    other_799 = other_798;
    let _e4: Translator = self_951;
    let _e8: MultiVector = other_799;
    let _e11: Translator = self_951;
    let _e15: MultiVector = other_799;
    let _e29: Translator = self_951;
    let _e33: MultiVector = other_799;
    let _e47: Translator = self_951;
    let _e51: MultiVector = other_799;
    let _e65: Translator = self_951;
    let _e69: MultiVector = other_799;
    let _e72: Translator = self_951;
    let _e76: MultiVector = other_799;
    let _e89: Translator = self_951;
    let _e93: MultiVector = other_799;
    let _e106: Translator = self_951;
    let _e110: MultiVector = other_799;
    let _e123: Translator = self_951;
    let _e127: MultiVector = other_799;
    let _e130: Translator = self_951;
    let _e134: MultiVector = other_799;
    let _e147: Translator = self_951;
    let _e151: MultiVector = other_799;
    let _e164: Translator = self_951;
    let _e168: MultiVector = other_799;
    let _e181: Translator = self_951;
    let _e185: MultiVector = other_799;
    let _e188: Translator = self_951;
    let _e192: MultiVector = other_799;
    let _e204: Translator = self_951;
    let _e208: MultiVector = other_799;
    let _e220: Translator = self_951;
    let _e224: MultiVector = other_799;
    return MultiVector(((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.y) * _e15.g3_.yxwz) * vec4<f32>(-(1.0), -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e29.g0_.z) * _e33.g3_.zwxy) * vec4<f32>(-(1.0), -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e47.g0_.w) * _e51.g3_.wzyx) * vec4<f32>(-(1.0), 1.0, -(1.0), -(1.0)))), ((((vec4<f32>(_e65.g0_.x) * _e69.g1_) + ((vec4<f32>(_e72.g0_.y) * _e76.g2_.yxwz) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e89.g0_.z) * _e93.g2_.zwxy) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e106.g0_.w) * _e110.g2_.wzyx) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))), ((((vec4<f32>(_e123.g0_.x) * _e127.g2_) + ((vec4<f32>(_e130.g0_.y) * _e134.g1_.yxwz) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e147.g0_.z) * _e151.g1_.zwxy) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e164.g0_.w) * _e168.g1_.wzyx) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))), ((((vec4<f32>(_e181.g0_.x) * _e185.g3_) + ((vec4<f32>(_e188.g0_.y) * _e192.g0_.yxwz) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e204.g0_.z) * _e208.g0_.zwxy) * vec4<f32>(1.0, -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e220.g0_.w) * _e224.g0_.wzyx) * vec4<f32>(1.0, 1.0, -(1.0), 1.0))));
}

fn translator_multi_vector_outer_product(self_952: Translator, other_800: MultiVector) -> MultiVector {
    var self_953: Translator;
    var other_801: MultiVector;

    self_953 = self_952;
    other_801 = other_800;
    let _e4: Translator = self_953;
    let _e8: MultiVector = other_801;
    let _e11: Translator = self_953;
    let _e15: MultiVector = other_801;
    let _e18: Translator = self_953;
    let _e22: MultiVector = other_801;
    let _e34: Translator = self_953;
    let _e38: MultiVector = other_801;
    let _e50: Translator = self_953;
    let _e53: MultiVector = other_801;
    let _e65: Translator = self_953;
    let _e69: MultiVector = other_801;
    let _e72: Translator = self_953;
    let _e76: MultiVector = other_801;
    let _e79: Translator = self_953;
    let _e83: MultiVector = other_801;
    let _e94: Translator = self_953;
    let _e98: MultiVector = other_801;
    let _e109: Translator = self_953;
    let _e112: MultiVector = other_801;
    return MultiVector((vec4<f32>(_e4.g0_.x) * _e8.g0_), ((((vec4<f32>(_e11.g0_.x) * _e15.g1_) + ((vec4<f32>(_e18.g0_.z) * _e22.g2_.wwwy) * vec4<f32>(0.0, -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e34.g0_.w) * _e38.g2_.zzyz) * vec4<f32>(0.0, 1.0, -(1.0), 0.0))) + ((_e50.g0_.xxyy * _e53.g2_.xxwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))), (vec4<f32>(_e65.g0_.x) * _e69.g2_), ((((vec4<f32>(_e72.g0_.x) * _e76.g3_) + ((vec4<f32>(_e79.g0_.z) * _e83.g0_.zzxz) * vec4<f32>(1.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e94.g0_.w) * _e98.g0_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, 1.0))) + ((_e109.g0_.yyxx * _e112.g0_.yxxx) * vec4<f32>(1.0, 1.0, 0.0, 0.0))));
}

fn translator_multi_vector_inner_product(self_954: Translator, other_802: MultiVector) -> MultiVector {
    var self_955: Translator;
    var other_803: MultiVector;

    self_955 = self_954;
    other_803 = other_802;
    let _e4: Translator = self_955;
    let _e8: MultiVector = other_803;
    let _e11: Translator = self_955;
    let _e15: MultiVector = other_803;
    let _e28: Translator = self_955;
    let _e32: MultiVector = other_803;
    let _e45: Translator = self_955;
    let _e48: MultiVector = other_803;
    let _e61: Translator = self_955;
    let _e65: MultiVector = other_803;
    let _e68: Translator = self_955;
    let _e72: MultiVector = other_803;
    let _e84: Translator = self_955;
    let _e88: MultiVector = other_803;
    let _e100: Translator = self_955;
    let _e103: MultiVector = other_803;
    let _e114: Translator = self_955;
    let _e118: MultiVector = other_803;
    let _e121: Translator = self_955;
    let _e125: MultiVector = other_803;
    let _e138: Translator = self_955;
    let _e142: MultiVector = other_803;
    let _e155: Translator = self_955;
    let _e158: MultiVector = other_803;
    let _e171: Translator = self_955;
    let _e175: MultiVector = other_803;
    let _e178: Translator = self_955;
    let _e180: MultiVector = other_803;
    return MultiVector(((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.z) * _e15.g3_.zzxz) * vec4<f32>(-(1.0), 0.0, -(1.0), 0.0))) + ((vec4<f32>(_e28.g0_.w) * _e32.g3_.wwwx) * vec4<f32>(-(1.0), 0.0, 0.0, -(1.0)))) + ((_e45.g0_.yyxx * _e48.g3_.yxxx) * vec4<f32>(-(1.0), -(1.0), 0.0, 0.0))), ((((vec4<f32>(_e61.g0_.x) * _e65.g1_) + ((vec4<f32>(_e68.g0_.z) * vec4<f32>(_e72.g2_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e84.g0_.w) * vec4<f32>(_e88.g2_.w)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((_e100.g0_.yxxx * _e103.g2_.yxxx) * vec4<f32>(1.0, 0.0, 0.0, 0.0))), ((((vec4<f32>(_e114.g0_.x) * _e118.g2_) + ((vec4<f32>(_e121.g0_.z) * _e125.g1_.wwxy) * vec4<f32>(0.0, -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e138.g0_.w) * _e142.g1_.zzyx) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))) + ((_e155.g0_.xyyy * _e158.g1_.xxwz) * vec4<f32>(0.0, -(1.0), 1.0, -(1.0)))), ((vec4<f32>(_e171.g0_.x) * _e175.g3_) + ((_e178.g0_ * vec4<f32>(_e180.g0_.x)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn translator_multi_vector_left_contraction(self_956: Translator, other_804: MultiVector) -> MultiVector {
    var self_957: Translator;
    var other_805: MultiVector;

    self_957 = self_956;
    other_805 = other_804;
    let _e4: Translator = self_957;
    let _e8: MultiVector = other_805;
    let _e11: Translator = self_957;
    let _e15: MultiVector = other_805;
    let _e28: Translator = self_957;
    let _e32: MultiVector = other_805;
    let _e45: Translator = self_957;
    let _e48: MultiVector = other_805;
    let _e61: Translator = self_957;
    let _e65: MultiVector = other_805;
    let _e68: Translator = self_957;
    let _e72: MultiVector = other_805;
    let _e75: Translator = self_957;
    let _e79: MultiVector = other_805;
    let _e91: Translator = self_957;
    let _e95: MultiVector = other_805;
    let _e107: Translator = self_957;
    let _e110: MultiVector = other_805;
    let _e122: Translator = self_957;
    let _e126: MultiVector = other_805;
    return MultiVector(((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.z) * _e15.g3_.zzxz) * vec4<f32>(-(1.0), 0.0, -(1.0), 0.0))) + ((vec4<f32>(_e28.g0_.w) * _e32.g3_.wwwx) * vec4<f32>(-(1.0), 0.0, 0.0, -(1.0)))) + ((_e45.g0_.yyxx * _e48.g3_.yxxx) * vec4<f32>(-(1.0), -(1.0), 0.0, 0.0))), (vec4<f32>(_e61.g0_.x) * _e65.g1_), ((((vec4<f32>(_e68.g0_.x) * _e72.g2_) + ((vec4<f32>(_e75.g0_.z) * _e79.g1_.wwwy) * vec4<f32>(0.0, -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e91.g0_.w) * _e95.g1_.zzyz) * vec4<f32>(0.0, 1.0, -(1.0), 0.0))) + ((_e107.g0_.xxyy * _e110.g1_.xxwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))), (vec4<f32>(_e122.g0_.x) * _e126.g3_));
}

fn translator_multi_vector_scalar_product(self_958: Translator, other_806: MultiVector) -> Scalar {
    var self_959: Translator;
    var other_807: MultiVector;

    self_959 = self_958;
    other_807 = other_806;
    let _e4: Translator = self_959;
    let _e7: MultiVector = other_807;
    let _e11: Translator = self_959;
    let _e14: MultiVector = other_807;
    let _e19: Translator = self_959;
    let _e22: MultiVector = other_807;
    let _e27: Translator = self_959;
    let _e30: MultiVector = other_807;
    return Scalar(((((_e4.g0_.x * _e7.g0_.x) - (_e11.g0_.y * _e14.g3_.y)) - (_e19.g0_.z * _e22.g3_.z)) - (_e27.g0_.w * _e30.g3_.w)));
}

fn translator_rotor_geometric_product(self_960: Translator, other_808: Rotor) -> Motor {
    var self_961: Translator;
    var other_809: Rotor;

    self_961 = self_960;
    other_809 = other_808;
    let _e4: Translator = self_961;
    let _e8: Rotor = other_809;
    let _e11: Translator = self_961;
    let _e15: Rotor = other_809;
    let _e26: Translator = self_961;
    let _e30: Rotor = other_809;
    let _e42: Translator = self_961;
    let _e46: Rotor = other_809;
    return Motor((vec4<f32>(_e4.g0_.x) * _e8.g0_), ((((vec4<f32>(_e11.g0_.y) * _e15.g0_.yxwz) * vec4<f32>(1.0, 1.0, 1.0, -(1.0))) + ((vec4<f32>(_e26.g0_.z) * _e30.g0_.zwxy) * vec4<f32>(1.0, -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e42.g0_.w) * _e46.g0_.wzyx) * vec4<f32>(1.0, 1.0, -(1.0), 1.0))));
}

fn translator_rotor_regressive_product(self_962: Translator, other_810: Rotor) -> Scalar {
    var self_963: Translator;
    var other_811: Rotor;

    self_963 = self_962;
    other_811 = other_810;
    let _e4: Translator = self_963;
    let _e7: Rotor = other_811;
    let _e11: Translator = self_963;
    let _e14: Rotor = other_811;
    let _e19: Translator = self_963;
    let _e22: Rotor = other_811;
    return Scalar((((_e4.g0_.y * _e7.g0_.y) + (_e11.g0_.z * _e14.g0_.z)) + (_e19.g0_.w * _e22.g0_.w)));
}

fn translator_rotor_outer_product(self_964: Translator, other_812: Rotor) -> Motor {
    var self_965: Translator;
    var other_813: Rotor;

    self_965 = self_964;
    other_813 = other_812;
    let _e4: Translator = self_965;
    let _e8: Rotor = other_813;
    let _e11: Translator = self_965;
    let _e15: Rotor = other_813;
    let _e25: Translator = self_965;
    let _e29: Rotor = other_813;
    let _e40: Translator = self_965;
    let _e43: Rotor = other_813;
    return Motor((vec4<f32>(_e4.g0_.x) * _e8.g0_), ((((vec4<f32>(_e11.g0_.z) * _e15.g0_.zzxz) * vec4<f32>(1.0, 0.0, 1.0, 0.0)) + ((vec4<f32>(_e25.g0_.w) * _e29.g0_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, 1.0))) + ((_e40.g0_.yyxx * _e43.g0_.yxxx) * vec4<f32>(1.0, 1.0, 0.0, 0.0))));
}

fn translator_rotor_left_contraction(self_966: Translator, other_814: Rotor) -> Rotor {
    var self_967: Translator;
    var other_815: Rotor;

    self_967 = self_966;
    other_815 = other_814;
    let _e4: Translator = self_967;
    let _e8: Rotor = other_815;
    return Rotor((vec4<f32>(_e4.g0_.x) * _e8.g0_));
}

fn translator_rotor_right_contraction(self_968: Translator, other_816: Rotor) -> Translator {
    var self_969: Translator;
    var other_817: Rotor;

    self_969 = self_968;
    other_817 = other_816;
    let _e4: Translator = self_969;
    let _e6: Rotor = other_817;
    return Translator((_e4.g0_ * vec4<f32>(_e6.g0_.x)));
}

fn translator_rotor_scalar_product(self_970: Translator, other_818: Rotor) -> Scalar {
    var self_971: Translator;
    var other_819: Rotor;

    self_971 = self_970;
    other_819 = other_818;
    let _e4: Translator = self_971;
    let _e7: Rotor = other_819;
    return Scalar((_e4.g0_.x * _e7.g0_.x));
}

fn translator_point_regressive_product(self_972: Translator, other_820: Point) -> Plane {
    var self_973: Translator;
    var other_821: Point;

    self_973 = self_972;
    other_821 = other_820;
    let _e4: Translator = self_973;
    let _e8: Point = other_821;
    let _e19: Translator = self_973;
    let _e23: Point = other_821;
    let _e35: Translator = self_973;
    let _e38: Point = other_821;
    return Plane(((((vec4<f32>(_e4.g0_.z) * _e8.g0_.zzxz) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0)) + ((vec4<f32>(_e19.g0_.w) * _e23.g0_.wwwx) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))) + ((_e35.g0_.yyxx * _e38.g0_.yxxx) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))));
}

fn translator_point_outer_product(self_974: Translator, other_822: Point) -> Point {
    var self_975: Translator;
    var other_823: Point;

    self_975 = self_974;
    other_823 = other_822;
    let _e4: Translator = self_975;
    let _e8: Point = other_823;
    return Point((vec4<f32>(_e4.g0_.x) * _e8.g0_));
}

fn translator_ideal_point_into(self_976: Translator) -> IdealPoint {
    var self_977: Translator;

    self_977 = self_976;
    let _e2: Translator = self_977;
    let _e5: Translator = self_977;
    let _e8: Translator = self_977;
    return IdealPoint(vec3<f32>(_e2.g0_.y, _e5.g0_.z, _e8.g0_.w));
}

fn translator_ideal_point_add(self_978: Translator, other_824: IdealPoint) -> Translator {
    var self_979: Translator;
    var other_825: IdealPoint;

    self_979 = self_978;
    other_825 = other_824;
    let _e4: Translator = self_979;
    let _e6: IdealPoint = other_825;
    let _e9: IdealPoint = other_825;
    let _e12: IdealPoint = other_825;
    let _e15: IdealPoint = other_825;
    return Translator((_e4.g0_ + (vec4<f32>(_e6.g0_.x, _e9.g0_.x, _e12.g0_.y, _e15.g0_.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn translator_ideal_point_sub(self_980: Translator, other_826: IdealPoint) -> Translator {
    var self_981: Translator;
    var other_827: IdealPoint;

    self_981 = self_980;
    other_827 = other_826;
    let _e4: Translator = self_981;
    let _e6: IdealPoint = other_827;
    let _e9: IdealPoint = other_827;
    let _e12: IdealPoint = other_827;
    let _e15: IdealPoint = other_827;
    return Translator((_e4.g0_ - (vec4<f32>(_e6.g0_.x, _e9.g0_.x, _e12.g0_.y, _e15.g0_.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn translator_ideal_point_outer_product(self_982: Translator, other_828: IdealPoint) -> IdealPoint {
    var self_983: Translator;
    var other_829: IdealPoint;

    self_983 = self_982;
    other_829 = other_828;
    let _e4: Translator = self_983;
    let _e8: IdealPoint = other_829;
    return IdealPoint((vec3<f32>(_e4.g0_.x) * _e8.g0_));
}

fn translator_ideal_point_inner_product(self_984: Translator, other_830: IdealPoint) -> Translator {
    var self_985: Translator;
    var other_831: IdealPoint;

    self_985 = self_984;
    other_831 = other_830;
    let _e4: Translator = self_985;
    let _e8: IdealPoint = other_831;
    let _e20: Translator = self_985;
    let _e24: IdealPoint = other_831;
    let _e37: Translator = self_985;
    let _e40: IdealPoint = other_831;
    let _e43: IdealPoint = other_831;
    let _e46: IdealPoint = other_831;
    let _e49: IdealPoint = other_831;
    return Translator(((((vec4<f32>(_e4.g0_.z) * vec4<f32>(_e8.g0_.y)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0)) + ((vec4<f32>(_e20.g0_.w) * vec4<f32>(_e24.g0_.z)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((_e37.g0_.yxxx * vec4<f32>(_e40.g0_.x, _e43.g0_.x, _e46.g0_.y, _e49.g0_.z)) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0))));
}

fn translator_ideal_point_left_contraction(self_986: Translator, other_832: IdealPoint) -> Translator {
    var self_987: Translator;
    var other_833: IdealPoint;

    self_987 = self_986;
    other_833 = other_832;
    let _e4: Translator = self_987;
    let _e8: IdealPoint = other_833;
    let _e20: Translator = self_987;
    let _e24: IdealPoint = other_833;
    let _e37: Translator = self_987;
    let _e40: IdealPoint = other_833;
    let _e43: IdealPoint = other_833;
    let _e46: IdealPoint = other_833;
    let _e49: IdealPoint = other_833;
    return Translator(((((vec4<f32>(_e4.g0_.z) * vec4<f32>(_e8.g0_.y)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0)) + ((vec4<f32>(_e20.g0_.w) * vec4<f32>(_e24.g0_.z)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((_e37.g0_.yxxx * vec4<f32>(_e40.g0_.x, _e43.g0_.x, _e46.g0_.y, _e49.g0_.z)) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0))));
}

fn translator_ideal_point_right_contraction(self_988: Translator, other_834: IdealPoint) -> Scalar {
    var self_989: Translator;
    var other_835: IdealPoint;

    self_989 = self_988;
    other_835 = other_834;
    let _e5: Translator = self_989;
    let _e8: IdealPoint = other_835;
    let _e13: Translator = self_989;
    let _e16: IdealPoint = other_835;
    let _e21: Translator = self_989;
    let _e24: IdealPoint = other_835;
    return Scalar((((0.0 - (_e5.g0_.y * _e8.g0_.x)) - (_e13.g0_.z * _e16.g0_.y)) - (_e21.g0_.w * _e24.g0_.z)));
}

fn translator_ideal_point_scalar_product(self_990: Translator, other_836: IdealPoint) -> Scalar {
    var self_991: Translator;
    var other_837: IdealPoint;

    self_991 = self_990;
    other_837 = other_836;
    let _e5: Translator = self_991;
    let _e8: IdealPoint = other_837;
    let _e13: Translator = self_991;
    let _e16: IdealPoint = other_837;
    let _e21: Translator = self_991;
    let _e24: IdealPoint = other_837;
    return Scalar((((0.0 - (_e5.g0_.y * _e8.g0_.x)) - (_e13.g0_.z * _e16.g0_.y)) - (_e21.g0_.w * _e24.g0_.z)));
}

fn translator_plane_inner_product(self_992: Translator, other_838: Plane) -> Plane {
    var self_993: Translator;
    var other_839: Plane;

    self_993 = self_992;
    other_839 = other_838;
    let _e4: Translator = self_993;
    let _e8: Plane = other_839;
    let _e11: Translator = self_993;
    let _e15: Plane = other_839;
    let _e27: Translator = self_993;
    let _e31: Plane = other_839;
    let _e43: Translator = self_993;
    let _e46: Plane = other_839;
    return Plane(((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.z) * _e15.g0_.zzxz) * vec4<f32>(1.0, 0.0, -(1.0), 0.0))) + ((vec4<f32>(_e27.g0_.w) * _e31.g0_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))) + ((_e43.g0_.yyxx * _e46.g0_.yxxx) * vec4<f32>(1.0, -(1.0), 0.0, 0.0))));
}

fn translator_plane_left_contraction(self_994: Translator, other_840: Plane) -> Plane {
    var self_995: Translator;
    var other_841: Plane;

    self_995 = self_994;
    other_841 = other_840;
    let _e4: Translator = self_995;
    let _e8: Plane = other_841;
    return Plane((vec4<f32>(_e4.g0_.x) * _e8.g0_));
}

fn translator_plane_right_contraction(self_996: Translator, other_842: Plane) -> Plane {
    var self_997: Translator;
    var other_843: Plane;

    self_997 = self_996;
    other_843 = other_842;
    let _e4: Translator = self_997;
    let _e8: Plane = other_843;
    let _e19: Translator = self_997;
    let _e23: Plane = other_843;
    let _e35: Translator = self_997;
    let _e38: Plane = other_843;
    return Plane(((((vec4<f32>(_e4.g0_.z) * _e8.g0_.zzxz) * vec4<f32>(1.0, 0.0, -(1.0), 0.0)) + ((vec4<f32>(_e19.g0_.w) * _e23.g0_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))) + ((_e35.g0_.yyxx * _e38.g0_.yxxx) * vec4<f32>(1.0, -(1.0), 0.0, 0.0))));
}

fn translator_line_geometric_product(self_998: Translator, other_844: Line) -> Motor {
    var self_999: Translator;
    var other_845: Line;

    self_999 = self_998;
    other_845 = other_844;
    let _e4: Translator = self_999;
    let _e8: Line = other_845;
    let _e11: Line = other_845;
    let _e14: Line = other_845;
    let _e17: Line = other_845;
    let _e30: Translator = self_999;
    let _e34: Line = other_845;
    let _e37: Line = other_845;
    let _e40: Line = other_845;
    let _e43: Line = other_845;
    let _e57: Translator = self_999;
    let _e61: Line = other_845;
    let _e64: Line = other_845;
    let _e67: Line = other_845;
    let _e70: Line = other_845;
    let _e84: Translator = self_999;
    let _e88: Line = other_845;
    let _e91: Line = other_845;
    let _e94: Line = other_845;
    let _e97: Line = other_845;
    let _e109: Translator = self_999;
    let _e113: Line = other_845;
    let _e116: Line = other_845;
    let _e119: Line = other_845;
    let _e122: Line = other_845;
    let _e134: Translator = self_999;
    let _e138: Line = other_845;
    let _e141: Line = other_845;
    let _e144: Line = other_845;
    let _e147: Line = other_845;
    let _e160: Translator = self_999;
    let _e164: Line = other_845;
    let _e167: Line = other_845;
    let _e170: Line = other_845;
    let _e173: Line = other_845;
    let _e186: Translator = self_999;
    let _e190: Line = other_845;
    let _e193: Line = other_845;
    let _e196: Line = other_845;
    let _e199: Line = other_845;
    return Motor((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.x, _e11.g0_.x, _e14.g0_.z, _e17.g0_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e30.g0_.z) * vec4<f32>(_e34.g0_.y, _e37.g0_.z, _e40.g0_.y, _e43.g0_.x)) * vec4<f32>(-(1.0), -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e57.g0_.w) * vec4<f32>(_e61.g0_.z, _e64.g0_.y, _e67.g0_.x, _e70.g0_.z)) * vec4<f32>(-(1.0), 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e84.g0_.x) * vec4<f32>(_e88.g1_.x, _e91.g1_.x, _e94.g1_.y, _e97.g1_.z)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))), (((((vec4<f32>(_e109.g0_.y) * vec4<f32>(_e113.g1_.x, _e116.g1_.x, _e119.g1_.z, _e122.g1_.y)) * vec4<f32>(1.0, 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e134.g0_.z) * vec4<f32>(_e138.g1_.y, _e141.g1_.z, _e144.g1_.y, _e147.g1_.x)) * vec4<f32>(1.0, -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e160.g0_.w) * vec4<f32>(_e164.g1_.z, _e167.g1_.y, _e170.g1_.x, _e173.g1_.z)) * vec4<f32>(1.0, 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e186.g0_.x) * vec4<f32>(_e190.g0_.x, _e193.g0_.x, _e196.g0_.y, _e199.g0_.z)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn translator_line_regressive_product(self_1000: Translator, other_846: Line) -> Scalar {
    var self_1001: Translator;
    var other_847: Line;

    self_1001 = self_1000;
    other_847 = other_846;
    let _e4: Translator = self_1001;
    let _e7: Line = other_847;
    let _e11: Translator = self_1001;
    let _e14: Line = other_847;
    let _e19: Translator = self_1001;
    let _e22: Line = other_847;
    return Scalar((((_e4.g0_.y * _e7.g1_.x) + (_e11.g0_.z * _e14.g1_.y)) + (_e19.g0_.w * _e22.g1_.z)));
}

fn translator_line_right_contraction(self_1002: Translator, other_848: Line) -> Scalar {
    var self_1003: Translator;
    var other_849: Line;

    self_1003 = self_1002;
    other_849 = other_848;
    let _e5: Translator = self_1003;
    let _e8: Line = other_849;
    let _e13: Translator = self_1003;
    let _e16: Line = other_849;
    let _e21: Translator = self_1003;
    let _e24: Line = other_849;
    return Scalar((((0.0 - (_e5.g0_.y * _e8.g0_.x)) - (_e13.g0_.z * _e16.g0_.y)) - (_e21.g0_.w * _e24.g0_.z)));
}

fn translator_line_scalar_product(self_1004: Translator, other_850: Line) -> Scalar {
    var self_1005: Translator;
    var other_851: Line;

    self_1005 = self_1004;
    other_851 = other_850;
    let _e5: Translator = self_1005;
    let _e8: Line = other_851;
    let _e13: Translator = self_1005;
    let _e16: Line = other_851;
    let _e21: Translator = self_1005;
    let _e24: Line = other_851;
    return Scalar((((0.0 - (_e5.g0_.y * _e8.g0_.x)) - (_e13.g0_.z * _e16.g0_.y)) - (_e21.g0_.w * _e24.g0_.z)));
}

fn translator_translator_add(self_1006: Translator, other_852: Translator) -> Translator {
    var self_1007: Translator;
    var other_853: Translator;

    self_1007 = self_1006;
    other_853 = other_852;
    let _e4: Translator = self_1007;
    let _e6: Translator = other_853;
    return Translator((_e4.g0_ + _e6.g0_));
}

fn translator_translator_sub(self_1008: Translator, other_854: Translator) -> Translator {
    var self_1009: Translator;
    var other_855: Translator;

    self_1009 = self_1008;
    other_855 = other_854;
    let _e4: Translator = self_1009;
    let _e6: Translator = other_855;
    return Translator((_e4.g0_ - _e6.g0_));
}

fn translator_translator_mul(self_1010: Translator, other_856: Translator) -> Translator {
    var self_1011: Translator;
    var other_857: Translator;

    self_1011 = self_1010;
    other_857 = other_856;
    let _e4: Translator = self_1011;
    let _e6: Translator = other_857;
    return Translator((_e4.g0_ * _e6.g0_));
}

fn translator_translator_div(self_1012: Translator, other_858: Translator) -> Translator {
    var self_1013: Translator;
    var other_859: Translator;

    self_1013 = self_1012;
    other_859 = other_858;
    let _e4: Translator = self_1013;
    let _e7: Translator = self_1013;
    let _e10: Translator = self_1013;
    let _e13: Translator = self_1013;
    let _e23: Translator = other_859;
    let _e26: Translator = other_859;
    let _e29: Translator = other_859;
    let _e32: Translator = other_859;
    return Translator((((vec4<f32>(_e4.g0_.x, _e7.g0_.y, _e10.g0_.z, _e13.g0_.w) * vec4<f32>(1.0, 1.0, 1.0, 1.0)) / vec4<f32>(_e23.g0_.x, _e26.g0_.y, _e29.g0_.z, _e32.g0_.w)) * vec4<f32>(1.0, 1.0, 1.0, 1.0)));
}

fn translator_translator_outer_product(self_1014: Translator, other_860: Translator) -> Translator {
    var self_1015: Translator;
    var other_861: Translator;

    self_1015 = self_1014;
    other_861 = other_860;
    let _e4: Translator = self_1015;
    let _e8: Translator = other_861;
    let _e11: Translator = self_1015;
    let _e13: Translator = other_861;
    return Translator(((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((_e11.g0_ * vec4<f32>(_e13.g0_.x)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn translator_translator_inner_product(self_1016: Translator, other_862: Translator) -> Translator {
    var self_1017: Translator;
    var other_863: Translator;

    self_1017 = self_1016;
    other_863 = other_862;
    let _e4: Translator = self_1017;
    let _e8: Translator = other_863;
    let _e11: Translator = self_1017;
    let _e15: Translator = other_863;
    let _e27: Translator = self_1017;
    let _e31: Translator = other_863;
    let _e43: Translator = self_1017;
    let _e46: Translator = other_863;
    return Translator(((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.z) * _e15.g0_.zzxz) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0))) + ((vec4<f32>(_e27.g0_.w) * _e31.g0_.wwwx) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))) + ((_e43.g0_.yyxx * _e46.g0_.yxxx) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))));
}

fn translator_translator_left_contraction(self_1018: Translator, other_864: Translator) -> Translator {
    var self_1019: Translator;
    var other_865: Translator;

    self_1019 = self_1018;
    other_865 = other_864;
    let _e4: Translator = self_1019;
    let _e8: Translator = other_865;
    let _e11: Translator = self_1019;
    let _e15: Translator = other_865;
    let _e28: Translator = self_1019;
    let _e32: Translator = other_865;
    let _e45: Translator = self_1019;
    let _e48: Translator = other_865;
    return Translator(((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.z) * vec4<f32>(_e15.g0_.z)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((vec4<f32>(_e28.g0_.w) * vec4<f32>(_e32.g0_.w)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((_e45.g0_.yxxx * _e48.g0_.yxxx) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))));
}

fn translator_translator_right_contraction(self_1020: Translator, other_866: Translator) -> Translator {
    var self_1021: Translator;
    var other_867: Translator;

    self_1021 = self_1020;
    other_867 = other_866;
    let _e4: Translator = self_1021;
    let _e8: Translator = other_867;
    let _e19: Translator = self_1021;
    let _e23: Translator = other_867;
    let _e35: Translator = self_1021;
    let _e39: Translator = other_867;
    let _e51: Translator = self_1021;
    let _e55: Translator = other_867;
    return Translator((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0)) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.zzxz) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0))) + ((vec4<f32>(_e35.g0_.w) * _e39.g0_.wwwx) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))) + ((vec4<f32>(_e51.g0_.x) * vec4<f32>(_e55.g0_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))));
}

fn translator_translator_scalar_product(self_1022: Translator, other_868: Translator) -> Scalar {
    var self_1023: Translator;
    var other_869: Translator;

    self_1023 = self_1022;
    other_869 = other_868;
    let _e4: Translator = self_1023;
    let _e7: Translator = other_869;
    let _e11: Translator = self_1023;
    let _e14: Translator = other_869;
    let _e19: Translator = self_1023;
    let _e22: Translator = other_869;
    let _e27: Translator = self_1023;
    let _e30: Translator = other_869;
    return Scalar(((((_e4.g0_.x * _e7.g0_.x) - (_e11.g0_.y * _e14.g0_.y)) - (_e19.g0_.z * _e22.g0_.z)) - (_e27.g0_.w * _e30.g0_.w)));
}

fn translator_motor_add(self_1024: Translator, other_870: Motor) -> Motor {
    var self_1025: Translator;
    var other_871: Motor;

    self_1025 = self_1024;
    other_871 = other_870;
    let _e4: Translator = self_1025;
    let _e14: Motor = other_871;
    let _e17: Translator = self_1025;
    let _e25: Motor = other_871;
    return Motor(((vec4<f32>(_e4.g0_.x) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) + _e14.g0_), ((_e17.g0_ * vec4<f32>(0.0, 1.0, 1.0, 1.0)) + _e25.g1_));
}

fn translator_motor_sub(self_1026: Translator, other_872: Motor) -> Motor {
    var self_1027: Translator;
    var other_873: Motor;

    self_1027 = self_1026;
    other_873 = other_872;
    let _e4: Translator = self_1027;
    let _e14: Motor = other_873;
    let _e17: Translator = self_1027;
    let _e25: Motor = other_873;
    return Motor(((vec4<f32>(_e4.g0_.x) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) - _e14.g0_), ((_e17.g0_ * vec4<f32>(0.0, 1.0, 1.0, 1.0)) - _e25.g1_));
}

fn translator_motor_geometric_product(self_1028: Translator, other_874: Motor) -> Motor {
    var self_1029: Translator;
    var other_875: Motor;

    self_1029 = self_1028;
    other_875 = other_874;
    let _e4: Translator = self_1029;
    let _e8: Motor = other_875;
    let _e11: Translator = self_1029;
    let _e15: Motor = other_875;
    let _e29: Translator = self_1029;
    let _e33: Motor = other_875;
    let _e47: Translator = self_1029;
    let _e51: Motor = other_875;
    let _e65: Translator = self_1029;
    let _e69: Motor = other_875;
    let _e72: Translator = self_1029;
    let _e76: Motor = other_875;
    let _e88: Translator = self_1029;
    let _e92: Motor = other_875;
    let _e104: Translator = self_1029;
    let _e108: Motor = other_875;
    return Motor(((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.y) * _e15.g1_.yxwz) * vec4<f32>(-(1.0), -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e29.g0_.z) * _e33.g1_.zwxy) * vec4<f32>(-(1.0), -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e47.g0_.w) * _e51.g1_.wzyx) * vec4<f32>(-(1.0), 1.0, -(1.0), -(1.0)))), ((((vec4<f32>(_e65.g0_.x) * _e69.g1_) + ((vec4<f32>(_e72.g0_.y) * _e76.g0_.yxwz) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e88.g0_.z) * _e92.g0_.zwxy) * vec4<f32>(1.0, -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e104.g0_.w) * _e108.g0_.wzyx) * vec4<f32>(1.0, 1.0, -(1.0), 1.0))));
}

fn translator_motor_regressive_product(self_1030: Translator, other_876: Motor) -> Translator {
    var self_1031: Translator;
    var other_877: Motor;

    self_1031 = self_1030;
    other_877 = other_876;
    let _e4: Translator = self_1031;
    let _e8: Motor = other_877;
    let _e11: Motor = other_877;
    let _e14: Motor = other_877;
    let _e17: Motor = other_877;
    let _e28: Translator = self_1031;
    let _e32: Motor = other_877;
    let _e35: Motor = other_877;
    let _e38: Motor = other_877;
    let _e41: Motor = other_877;
    let _e53: Translator = self_1031;
    let _e57: Motor = other_877;
    let _e60: Motor = other_877;
    let _e63: Motor = other_877;
    let _e66: Motor = other_877;
    let _e78: Translator = self_1031;
    let _e82: Motor = other_877;
    return Translator((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.y, _e11.g1_.x, _e14.g0_.y, _e17.g0_.y)) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) + ((vec4<f32>(_e28.g0_.z) * vec4<f32>(_e32.g0_.z, _e35.g0_.z, _e38.g1_.x, _e41.g0_.z)) * vec4<f32>(1.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e53.g0_.w) * vec4<f32>(_e57.g0_.w, _e60.g0_.w, _e63.g0_.w, _e66.g1_.x)) * vec4<f32>(1.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e78.g0_.x) * vec4<f32>(_e82.g1_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))));
}

fn translator_motor_outer_product(self_1032: Translator, other_878: Motor) -> Motor {
    var self_1033: Translator;
    var other_879: Motor;

    self_1033 = self_1032;
    other_879 = other_878;
    let _e4: Translator = self_1033;
    let _e8: Motor = other_879;
    let _e11: Translator = self_1033;
    let _e15: Motor = other_879;
    let _e18: Translator = self_1033;
    let _e22: Motor = other_879;
    let _e33: Translator = self_1033;
    let _e37: Motor = other_879;
    let _e48: Translator = self_1033;
    let _e51: Motor = other_879;
    return Motor((vec4<f32>(_e4.g0_.x) * _e8.g0_), ((((vec4<f32>(_e11.g0_.x) * _e15.g1_) + ((vec4<f32>(_e18.g0_.z) * _e22.g0_.zzxz) * vec4<f32>(1.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e33.g0_.w) * _e37.g0_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, 1.0))) + ((_e48.g0_.yyxx * _e51.g0_.yxxx) * vec4<f32>(1.0, 1.0, 0.0, 0.0))));
}

fn translator_motor_inner_product(self_1034: Translator, other_880: Motor) -> Motor {
    var self_1035: Translator;
    var other_881: Motor;

    self_1035 = self_1034;
    other_881 = other_880;
    let _e4: Translator = self_1035;
    let _e8: Motor = other_881;
    let _e11: Translator = self_1035;
    let _e15: Motor = other_881;
    let _e28: Translator = self_1035;
    let _e32: Motor = other_881;
    let _e45: Translator = self_1035;
    let _e48: Motor = other_881;
    let _e61: Translator = self_1035;
    let _e65: Motor = other_881;
    let _e68: Translator = self_1035;
    let _e70: Motor = other_881;
    return Motor(((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.z) * _e15.g1_.zzxz) * vec4<f32>(-(1.0), 0.0, -(1.0), 0.0))) + ((vec4<f32>(_e28.g0_.w) * _e32.g1_.wwwx) * vec4<f32>(-(1.0), 0.0, 0.0, -(1.0)))) + ((_e45.g0_.yyxx * _e48.g1_.yxxx) * vec4<f32>(-(1.0), -(1.0), 0.0, 0.0))), ((vec4<f32>(_e61.g0_.x) * _e65.g1_) + ((_e68.g0_ * vec4<f32>(_e70.g0_.x)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn translator_motor_left_contraction(self_1036: Translator, other_882: Motor) -> Motor {
    var self_1037: Translator;
    var other_883: Motor;

    self_1037 = self_1036;
    other_883 = other_882;
    let _e4: Translator = self_1037;
    let _e8: Motor = other_883;
    let _e11: Translator = self_1037;
    let _e15: Motor = other_883;
    let _e28: Translator = self_1037;
    let _e32: Motor = other_883;
    let _e45: Translator = self_1037;
    let _e48: Motor = other_883;
    let _e61: Translator = self_1037;
    let _e65: Motor = other_883;
    return Motor(((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.z) * _e15.g1_.zzxz) * vec4<f32>(-(1.0), 0.0, -(1.0), 0.0))) + ((vec4<f32>(_e28.g0_.w) * _e32.g1_.wwwx) * vec4<f32>(-(1.0), 0.0, 0.0, -(1.0)))) + ((_e45.g0_.yyxx * _e48.g1_.yxxx) * vec4<f32>(-(1.0), -(1.0), 0.0, 0.0))), (vec4<f32>(_e61.g0_.x) * _e65.g1_));
}

fn translator_motor_right_contraction(self_1038: Translator, other_884: Motor) -> Translator {
    var self_1039: Translator;
    var other_885: Motor;

    self_1039 = self_1038;
    other_885 = other_884;
    let _e4: Translator = self_1039;
    let _e8: Motor = other_885;
    let _e11: Motor = other_885;
    let _e14: Motor = other_885;
    let _e17: Motor = other_885;
    let _e29: Translator = self_1039;
    let _e33: Motor = other_885;
    let _e36: Motor = other_885;
    let _e39: Motor = other_885;
    let _e42: Motor = other_885;
    let _e55: Translator = self_1039;
    let _e59: Motor = other_885;
    let _e62: Motor = other_885;
    let _e65: Motor = other_885;
    let _e68: Motor = other_885;
    let _e81: Translator = self_1039;
    let _e85: Motor = other_885;
    return Translator((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g1_.y, _e11.g0_.x, _e14.g1_.y, _e17.g1_.y)) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0)) + ((vec4<f32>(_e29.g0_.z) * vec4<f32>(_e33.g1_.z, _e36.g1_.z, _e39.g0_.x, _e42.g1_.z)) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0))) + ((vec4<f32>(_e55.g0_.w) * vec4<f32>(_e59.g1_.w, _e62.g1_.w, _e65.g1_.w, _e68.g0_.x)) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))) + ((vec4<f32>(_e81.g0_.x) * vec4<f32>(_e85.g0_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))));
}

fn translator_motor_scalar_product(self_1040: Translator, other_886: Motor) -> Scalar {
    var self_1041: Translator;
    var other_887: Motor;

    self_1041 = self_1040;
    other_887 = other_886;
    let _e4: Translator = self_1041;
    let _e7: Motor = other_887;
    let _e11: Translator = self_1041;
    let _e14: Motor = other_887;
    let _e19: Translator = self_1041;
    let _e22: Motor = other_887;
    let _e27: Translator = self_1041;
    let _e30: Motor = other_887;
    return Scalar(((((_e4.g0_.x * _e7.g0_.x) - (_e11.g0_.y * _e14.g1_.y)) - (_e19.g0_.z * _e22.g1_.z)) - (_e27.g0_.w * _e30.g1_.w)));
}

fn translator_point_and_plane_geometric_product(self_1042: Translator, other_888: PointAndPlane) -> PointAndPlane {
    var self_1043: Translator;
    var other_889: PointAndPlane;

    self_1043 = self_1042;
    other_889 = other_888;
    let _e4: Translator = self_1043;
    let _e8: PointAndPlane = other_889;
    let _e11: Translator = self_1043;
    let _e15: PointAndPlane = other_889;
    let _e18: PointAndPlane = other_889;
    let _e21: PointAndPlane = other_889;
    let _e24: PointAndPlane = other_889;
    let _e38: Translator = self_1043;
    let _e42: PointAndPlane = other_889;
    let _e45: PointAndPlane = other_889;
    let _e48: PointAndPlane = other_889;
    let _e51: PointAndPlane = other_889;
    let _e65: Translator = self_1043;
    let _e69: PointAndPlane = other_889;
    let _e72: PointAndPlane = other_889;
    let _e75: PointAndPlane = other_889;
    let _e78: PointAndPlane = other_889;
    let _e92: Translator = self_1043;
    let _e96: PointAndPlane = other_889;
    let _e99: Translator = self_1043;
    let _e103: PointAndPlane = other_889;
    let _e106: PointAndPlane = other_889;
    let _e109: PointAndPlane = other_889;
    let _e112: PointAndPlane = other_889;
    let _e126: Translator = self_1043;
    let _e130: PointAndPlane = other_889;
    let _e133: PointAndPlane = other_889;
    let _e136: PointAndPlane = other_889;
    let _e139: PointAndPlane = other_889;
    let _e153: Translator = self_1043;
    let _e157: PointAndPlane = other_889;
    let _e160: PointAndPlane = other_889;
    let _e163: PointAndPlane = other_889;
    let _e166: PointAndPlane = other_889;
    return PointAndPlane(((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.y) * vec4<f32>(_e15.g0_.y, _e18.g0_.x, _e21.g1_.w, _e24.g1_.z)) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e38.g0_.z) * vec4<f32>(_e42.g0_.z, _e45.g1_.w, _e48.g0_.x, _e51.g1_.y)) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e65.g0_.w) * vec4<f32>(_e69.g0_.w, _e72.g1_.z, _e75.g1_.y, _e78.g0_.x)) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))), ((((vec4<f32>(_e92.g0_.x) * _e96.g1_) + ((vec4<f32>(_e99.g0_.y) * vec4<f32>(_e103.g1_.y, _e106.g1_.x, _e109.g0_.w, _e112.g0_.z)) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e126.g0_.z) * vec4<f32>(_e130.g1_.z, _e133.g0_.w, _e136.g1_.x, _e139.g0_.y)) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e153.g0_.w) * vec4<f32>(_e157.g1_.w, _e160.g0_.z, _e163.g0_.y, _e166.g1_.x)) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))));
}

fn translator_point_and_plane_regressive_product(self_1044: Translator, other_890: PointAndPlane) -> Plane {
    var self_1045: Translator;
    var other_891: PointAndPlane;

    self_1045 = self_1044;
    other_891 = other_890;
    let _e4: Translator = self_1045;
    let _e8: PointAndPlane = other_891;
    let _e19: Translator = self_1045;
    let _e23: PointAndPlane = other_891;
    let _e35: Translator = self_1045;
    let _e38: PointAndPlane = other_891;
    return Plane(((((vec4<f32>(_e4.g0_.z) * _e8.g0_.zzxz) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0)) + ((vec4<f32>(_e19.g0_.w) * _e23.g0_.wwwx) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))) + ((_e35.g0_.yyxx * _e38.g0_.yxxx) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))));
}

fn translator_point_and_plane_outer_product(self_1046: Translator, other_892: PointAndPlane) -> PointAndPlane {
    var self_1047: Translator;
    var other_893: PointAndPlane;

    self_1047 = self_1046;
    other_893 = other_892;
    let _e4: Translator = self_1047;
    let _e8: PointAndPlane = other_893;
    let _e11: Translator = self_1047;
    let _e15: PointAndPlane = other_893;
    let _e27: Translator = self_1047;
    let _e31: PointAndPlane = other_893;
    let _e43: Translator = self_1047;
    let _e46: PointAndPlane = other_893;
    let _e58: Translator = self_1047;
    let _e62: PointAndPlane = other_893;
    return PointAndPlane(((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.z) * _e15.g1_.wwwy) * vec4<f32>(0.0, -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e27.g0_.w) * _e31.g1_.zzyz) * vec4<f32>(0.0, 1.0, -(1.0), 0.0))) + ((_e43.g0_.xxyy * _e46.g1_.xxwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))), (vec4<f32>(_e58.g0_.x) * _e62.g1_));
}

fn translator_point_and_plane_inner_product(self_1048: Translator, other_894: PointAndPlane) -> PointAndPlane {
    var self_1049: Translator;
    var other_895: PointAndPlane;

    self_1049 = self_1048;
    other_895 = other_894;
    let _e4: Translator = self_1049;
    let _e8: PointAndPlane = other_895;
    let _e11: Translator = self_1049;
    let _e15: PointAndPlane = other_895;
    let _e18: Translator = self_1049;
    let _e22: PointAndPlane = other_895;
    let _e25: PointAndPlane = other_895;
    let _e28: PointAndPlane = other_895;
    let _e31: PointAndPlane = other_895;
    let _e45: Translator = self_1049;
    let _e49: PointAndPlane = other_895;
    let _e52: PointAndPlane = other_895;
    let _e55: PointAndPlane = other_895;
    let _e58: PointAndPlane = other_895;
    let _e72: Translator = self_1049;
    let _e76: PointAndPlane = other_895;
    let _e79: PointAndPlane = other_895;
    let _e82: PointAndPlane = other_895;
    let _e85: PointAndPlane = other_895;
    return PointAndPlane((vec4<f32>(_e4.g0_.x) * _e8.g0_), ((((vec4<f32>(_e11.g0_.x) * _e15.g1_) + ((vec4<f32>(_e18.g0_.y) * vec4<f32>(_e22.g1_.y, _e25.g1_.x, _e28.g0_.w, _e31.g0_.z)) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e45.g0_.z) * vec4<f32>(_e49.g1_.z, _e52.g0_.w, _e55.g1_.x, _e58.g0_.y)) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e72.g0_.w) * vec4<f32>(_e76.g1_.w, _e79.g0_.z, _e82.g0_.y, _e85.g1_.x)) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))));
}

fn translator_point_and_plane_left_contraction(self_1050: Translator, other_896: PointAndPlane) -> PointAndPlane {
    var self_1051: Translator;
    var other_897: PointAndPlane;

    self_1051 = self_1050;
    other_897 = other_896;
    let _e4: Translator = self_1051;
    let _e8: PointAndPlane = other_897;
    let _e11: Translator = self_1051;
    let _e15: PointAndPlane = other_897;
    let _e18: Translator = self_1051;
    let _e22: PointAndPlane = other_897;
    let _e34: Translator = self_1051;
    let _e38: PointAndPlane = other_897;
    let _e50: Translator = self_1051;
    let _e53: PointAndPlane = other_897;
    return PointAndPlane((vec4<f32>(_e4.g0_.x) * _e8.g0_), ((((vec4<f32>(_e11.g0_.x) * _e15.g1_) + ((vec4<f32>(_e18.g0_.z) * _e22.g0_.wwwy) * vec4<f32>(0.0, -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e34.g0_.w) * _e38.g0_.zzyz) * vec4<f32>(0.0, 1.0, -(1.0), 0.0))) + ((_e50.g0_.xxyy * _e53.g0_.xxwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))));
}

fn translator_point_and_plane_right_contraction(self_1052: Translator, other_898: PointAndPlane) -> Plane {
    var self_1053: Translator;
    var other_899: PointAndPlane;

    self_1053 = self_1052;
    other_899 = other_898;
    let _e4: Translator = self_1053;
    let _e8: PointAndPlane = other_899;
    let _e19: Translator = self_1053;
    let _e23: PointAndPlane = other_899;
    let _e35: Translator = self_1053;
    let _e38: PointAndPlane = other_899;
    return Plane(((((vec4<f32>(_e4.g0_.z) * _e8.g1_.zzxz) * vec4<f32>(1.0, 0.0, -(1.0), 0.0)) + ((vec4<f32>(_e19.g0_.w) * _e23.g1_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))) + ((_e35.g0_.yyxx * _e38.g1_.yxxx) * vec4<f32>(1.0, -(1.0), 0.0, 0.0))));
}

fn translator_squared_magnitude(self_1054: Translator) -> Scalar {
    var self_1055: Translator;

    self_1055 = self_1054;
    let _e4: Translator = self_1055;
    let _e5: Translator = translator_reversal(_e4);
    let _e6: Translator = self_1055;
    let _e8: Translator = self_1055;
    let _e9: Translator = translator_reversal(_e8);
    let _e10: Scalar = translator_translator_scalar_product(_e6, _e9);
    return _e10;
}

fn translator_magnitude(self_1056: Translator) -> Scalar {
    var self_1057: Translator;

    self_1057 = self_1056;
    let _e3: Translator = self_1057;
    let _e4: Scalar = translator_squared_magnitude(_e3);
    let _e7: Translator = self_1057;
    let _e8: Scalar = translator_squared_magnitude(_e7);
    return Scalar(sqrt(_e8.g0_));
}

fn translator_scale(self_1058: Translator, other_900: f32) -> Translator {
    var self_1059: Translator;
    var other_901: f32;

    self_1059 = self_1058;
    other_901 = other_900;
    let _e5: f32 = other_901;
    let _e7: Translator = self_1059;
    let _e8: f32 = other_901;
    let _e10: Translator = translator_scalar_geometric_product(_e7, Scalar(_e8));
    return _e10;
}

fn translator_signum(self_1060: Translator) -> Translator {
    var self_1061: Translator;

    self_1061 = self_1060;
    let _e5: Translator = self_1061;
    let _e6: Scalar = translator_magnitude(_e5);
    let _e10: Translator = self_1061;
    let _e13: Translator = self_1061;
    let _e14: Scalar = translator_magnitude(_e13);
    let _e18: Translator = translator_scalar_geometric_product(_e10, Scalar((1.0 / _e14.g0_)));
    return _e18;
}

fn translator_inverse(self_1062: Translator) -> Translator {
    var self_1063: Translator;

    self_1063 = self_1062;
    let _e3: Translator = self_1063;
    let _e4: Translator = translator_reversal(_e3);
    let _e7: Translator = self_1063;
    let _e8: Scalar = translator_squared_magnitude(_e7);
    let _e13: Translator = self_1063;
    let _e14: Translator = translator_reversal(_e13);
    let _e17: Translator = self_1063;
    let _e18: Scalar = translator_squared_magnitude(_e17);
    let _e22: Translator = translator_scalar_geometric_product(_e14, Scalar((1.0 / _e18.g0_)));
    return _e22;
}

fn motor_zero() -> Motor {
    return Motor(vec4<f32>(0.0), vec4<f32>(0.0));
}

fn motor_one() -> Motor {
    return Motor(vec4<f32>(1.0, 0.0, 0.0, 0.0), vec4<f32>(0.0));
}

fn motor_neg(self_1064: Motor) -> Motor {
    var self_1065: Motor;

    self_1065 = self_1064;
    let _e2: Motor = self_1065;
    let _e8: Motor = self_1065;
    return Motor((_e2.g0_ * vec4<f32>(-(1.0))), (_e8.g1_ * vec4<f32>(-(1.0))));
}

fn motor_automorphism(self_1066: Motor) -> Motor {
    var self_1067: Motor;

    self_1067 = self_1066;
    let _e2: Motor = self_1067;
    let _e4: Motor = self_1067;
    return Motor(_e2.g0_, _e4.g1_);
}

fn motor_reversal(self_1068: Motor) -> Motor {
    var self_1069: Motor;

    self_1069 = self_1068;
    let _e2: Motor = self_1069;
    let _e13: Motor = self_1069;
    return Motor((_e2.g0_ * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))), (_e13.g1_ * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))));
}

fn motor_conjugation(self_1070: Motor) -> Motor {
    var self_1071: Motor;

    self_1071 = self_1070;
    let _e2: Motor = self_1071;
    let _e13: Motor = self_1071;
    return Motor((_e2.g0_ * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))), (_e13.g1_ * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))));
}

fn motor_dual(self_1072: Motor) -> Motor {
    var self_1073: Motor;

    self_1073 = self_1072;
    let _e2: Motor = self_1073;
    let _e4: Motor = self_1073;
    return Motor(_e2.g1_, _e4.g0_);
}

fn motor_scalar_into(self_1074: Motor) -> Scalar {
    var self_1075: Motor;

    self_1075 = self_1074;
    let _e2: Motor = self_1075;
    return Scalar(_e2.g0_.x);
}

fn motor_scalar_add(self_1076: Motor, other_902: Scalar) -> Motor {
    var self_1077: Motor;
    var other_903: Scalar;

    self_1077 = self_1076;
    other_903 = other_902;
    let _e4: Motor = self_1077;
    let _e6: Scalar = other_903;
    let _e16: Motor = self_1077;
    return Motor((_e4.g0_ + (vec4<f32>(_e6.g0_) * vec4<f32>(1.0, 0.0, 0.0, 0.0))), _e16.g1_);
}

fn motor_scalar_sub(self_1078: Motor, other_904: Scalar) -> Motor {
    var self_1079: Motor;
    var other_905: Scalar;

    self_1079 = self_1078;
    other_905 = other_904;
    let _e4: Motor = self_1079;
    let _e6: Scalar = other_905;
    let _e16: Motor = self_1079;
    return Motor((_e4.g0_ - (vec4<f32>(_e6.g0_) * vec4<f32>(1.0, 0.0, 0.0, 0.0))), _e16.g1_);
}

fn motor_scalar_geometric_product(self_1080: Motor, other_906: Scalar) -> Motor {
    var self_1081: Motor;
    var other_907: Scalar;

    self_1081 = self_1080;
    other_907 = other_906;
    let _e4: Motor = self_1081;
    let _e6: Scalar = other_907;
    let _e10: Motor = self_1081;
    let _e12: Scalar = other_907;
    return Motor((_e4.g0_ * vec4<f32>(_e6.g0_)), (_e10.g1_ * vec4<f32>(_e12.g0_)));
}

fn motor_scalar_regressive_product(self_1082: Motor, other_908: Scalar) -> Scalar {
    var self_1083: Motor;
    var other_909: Scalar;

    self_1083 = self_1082;
    other_909 = other_908;
    let _e4: Motor = self_1083;
    let _e7: Scalar = other_909;
    return Scalar((_e4.g1_.x * _e7.g0_));
}

fn motor_scalar_outer_product(self_1084: Motor, other_910: Scalar) -> Motor {
    var self_1085: Motor;
    var other_911: Scalar;

    self_1085 = self_1084;
    other_911 = other_910;
    let _e4: Motor = self_1085;
    let _e6: Scalar = other_911;
    let _e10: Motor = self_1085;
    let _e12: Scalar = other_911;
    return Motor((_e4.g0_ * vec4<f32>(_e6.g0_)), (_e10.g1_ * vec4<f32>(_e12.g0_)));
}

fn motor_scalar_inner_product(self_1086: Motor, other_912: Scalar) -> Motor {
    var self_1087: Motor;
    var other_913: Scalar;

    self_1087 = self_1086;
    other_913 = other_912;
    let _e4: Motor = self_1087;
    let _e6: Scalar = other_913;
    let _e10: Motor = self_1087;
    let _e12: Scalar = other_913;
    return Motor((_e4.g0_ * vec4<f32>(_e6.g0_)), (_e10.g1_ * vec4<f32>(_e12.g0_)));
}

fn motor_scalar_left_contraction(self_1088: Motor, other_914: Scalar) -> Scalar {
    var self_1089: Motor;
    var other_915: Scalar;

    self_1089 = self_1088;
    other_915 = other_914;
    let _e4: Motor = self_1089;
    let _e7: Scalar = other_915;
    return Scalar((_e4.g0_.x * _e7.g0_));
}

fn motor_scalar_right_contraction(self_1090: Motor, other_916: Scalar) -> Motor {
    var self_1091: Motor;
    var other_917: Scalar;

    self_1091 = self_1090;
    other_917 = other_916;
    let _e4: Motor = self_1091;
    let _e6: Scalar = other_917;
    let _e10: Motor = self_1091;
    let _e12: Scalar = other_917;
    return Motor((_e4.g0_ * vec4<f32>(_e6.g0_)), (_e10.g1_ * vec4<f32>(_e12.g0_)));
}

fn motor_scalar_scalar_product(self_1092: Motor, other_918: Scalar) -> Scalar {
    var self_1093: Motor;
    var other_919: Scalar;

    self_1093 = self_1092;
    other_919 = other_918;
    let _e4: Motor = self_1093;
    let _e7: Scalar = other_919;
    return Scalar((_e4.g0_.x * _e7.g0_));
}

fn motor_multi_vector_add(self_1094: Motor, other_920: MultiVector) -> MultiVector {
    var self_1095: Motor;
    var other_921: MultiVector;

    self_1095 = self_1094;
    other_921 = other_920;
    let _e4: Motor = self_1095;
    let _e6: MultiVector = other_921;
    let _e9: MultiVector = other_921;
    let _e11: MultiVector = other_921;
    let _e13: Motor = self_1095;
    let _e15: MultiVector = other_921;
    return MultiVector((_e4.g0_ + _e6.g0_), _e9.g1_, _e11.g2_, (_e13.g1_ + _e15.g3_));
}

fn motor_multi_vector_sub(self_1096: Motor, other_922: MultiVector) -> MultiVector {
    var self_1097: Motor;
    var other_923: MultiVector;

    self_1097 = self_1096;
    other_923 = other_922;
    let _e4: Motor = self_1097;
    let _e6: MultiVector = other_923;
    let _e11: MultiVector = other_923;
    let _e16: MultiVector = other_923;
    let _e19: Motor = self_1097;
    let _e21: MultiVector = other_923;
    return MultiVector((_e4.g0_ - _e6.g0_), (vec4<f32>(0.0) - _e11.g1_), (vec4<f32>(0.0) - _e16.g2_), (_e19.g1_ - _e21.g3_));
}

fn motor_multi_vector_geometric_product(self_1098: Motor, other_924: MultiVector) -> MultiVector {
    var self_1099: Motor;
    var other_925: MultiVector;

    self_1099 = self_1098;
    other_925 = other_924;
    let _e4: Motor = self_1099;
    let _e8: MultiVector = other_925;
    let _e11: Motor = self_1099;
    let _e15: MultiVector = other_925;
    let _e28: Motor = self_1099;
    let _e32: MultiVector = other_925;
    let _e45: Motor = self_1099;
    let _e49: MultiVector = other_925;
    let _e62: Motor = self_1099;
    let _e66: MultiVector = other_925;
    let _e79: Motor = self_1099;
    let _e83: MultiVector = other_925;
    let _e97: Motor = self_1099;
    let _e101: MultiVector = other_925;
    let _e115: Motor = self_1099;
    let _e119: MultiVector = other_925;
    let _e133: Motor = self_1099;
    let _e137: MultiVector = other_925;
    let _e140: Motor = self_1099;
    let _e144: MultiVector = other_925;
    let _e157: Motor = self_1099;
    let _e161: MultiVector = other_925;
    let _e174: Motor = self_1099;
    let _e178: MultiVector = other_925;
    let _e191: Motor = self_1099;
    let _e195: MultiVector = other_925;
    let _e199: Motor = self_1099;
    let _e203: MultiVector = other_925;
    let _e216: Motor = self_1099;
    let _e220: MultiVector = other_925;
    let _e233: Motor = self_1099;
    let _e237: MultiVector = other_925;
    let _e250: Motor = self_1099;
    let _e254: MultiVector = other_925;
    let _e257: Motor = self_1099;
    let _e261: MultiVector = other_925;
    let _e274: Motor = self_1099;
    let _e278: MultiVector = other_925;
    let _e291: Motor = self_1099;
    let _e295: MultiVector = other_925;
    let _e308: Motor = self_1099;
    let _e312: MultiVector = other_925;
    let _e316: Motor = self_1099;
    let _e320: MultiVector = other_925;
    let _e333: Motor = self_1099;
    let _e337: MultiVector = other_925;
    let _e350: Motor = self_1099;
    let _e354: MultiVector = other_925;
    let _e367: Motor = self_1099;
    let _e371: MultiVector = other_925;
    let _e374: Motor = self_1099;
    let _e378: MultiVector = other_925;
    let _e391: Motor = self_1099;
    let _e395: MultiVector = other_925;
    let _e408: Motor = self_1099;
    let _e412: MultiVector = other_925;
    let _e425: Motor = self_1099;
    let _e429: MultiVector = other_925;
    let _e442: Motor = self_1099;
    let _e446: MultiVector = other_925;
    let _e458: Motor = self_1099;
    let _e462: MultiVector = other_925;
    let _e474: Motor = self_1099;
    let _e478: MultiVector = other_925;
    return MultiVector(((((((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.y) * _e15.g0_.yxwz) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e28.g0_.z) * _e32.g0_.zwxy) * vec4<f32>(-(1.0), -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e45.g0_.w) * _e49.g0_.wzyx) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))) + ((vec4<f32>(_e62.g1_.x) * _e66.g3_) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0)))) + ((vec4<f32>(_e79.g1_.y) * _e83.g3_.yxwz) * vec4<f32>(-(1.0), -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e97.g1_.z) * _e101.g3_.zwxy) * vec4<f32>(-(1.0), -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e115.g1_.w) * _e119.g3_.wzyx) * vec4<f32>(-(1.0), 1.0, -(1.0), -(1.0)))), ((((((((vec4<f32>(_e133.g0_.x) * _e137.g1_) + ((vec4<f32>(_e140.g0_.y) * _e144.g1_.yxwz) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e157.g0_.z) * _e161.g1_.zwxy) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e174.g0_.w) * _e178.g1_.wzyx) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))) - (vec4<f32>(_e191.g1_.x) * _e195.g2_)) + ((vec4<f32>(_e199.g1_.y) * _e203.g2_.yxwz) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e216.g1_.z) * _e220.g2_.zwxy) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e233.g1_.w) * _e237.g2_.wzyx) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))), ((((((((vec4<f32>(_e250.g0_.x) * _e254.g2_) + ((vec4<f32>(_e257.g0_.y) * _e261.g2_.yxwz) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e274.g0_.z) * _e278.g2_.zwxy) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e291.g0_.w) * _e295.g2_.wzyx) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))) - (vec4<f32>(_e308.g1_.x) * _e312.g1_)) + ((vec4<f32>(_e316.g1_.y) * _e320.g1_.yxwz) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e333.g1_.z) * _e337.g1_.zwxy) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e350.g1_.w) * _e354.g1_.wzyx) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))), ((((((((vec4<f32>(_e367.g0_.x) * _e371.g3_) + ((vec4<f32>(_e374.g0_.y) * _e378.g3_.yxwz) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e391.g0_.z) * _e395.g3_.zwxy) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e408.g0_.w) * _e412.g3_.wzyx) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e425.g1_.x) * _e429.g0_) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0)))) + ((vec4<f32>(_e442.g1_.y) * _e446.g0_.yxwz) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e458.g1_.z) * _e462.g0_.zwxy) * vec4<f32>(1.0, -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e474.g1_.w) * _e478.g0_.wzyx) * vec4<f32>(1.0, 1.0, -(1.0), 1.0))));
}

fn motor_multi_vector_regressive_product(self_1100: Motor, other_926: MultiVector) -> MultiVector {
    var self_1101: Motor;
    var other_927: MultiVector;

    self_1101 = self_1100;
    other_927 = other_926;
    let _e4: Motor = self_1101;
    let _e8: MultiVector = other_927;
    let _e18: Motor = self_1101;
    let _e22: MultiVector = other_927;
    let _e33: Motor = self_1101;
    let _e37: MultiVector = other_927;
    let _e48: Motor = self_1101;
    let _e52: MultiVector = other_927;
    let _e56: Motor = self_1101;
    let _e60: MultiVector = other_927;
    let _e72: Motor = self_1101;
    let _e76: MultiVector = other_927;
    let _e88: Motor = self_1101;
    let _e92: MultiVector = other_927;
    let _e104: Motor = self_1101;
    let _e108: MultiVector = other_927;
    let _e120: Motor = self_1101;
    let _e124: MultiVector = other_927;
    let _e127: Motor = self_1101;
    let _e131: MultiVector = other_927;
    let _e144: Motor = self_1101;
    let _e148: MultiVector = other_927;
    let _e161: Motor = self_1101;
    let _e164: MultiVector = other_927;
    let _e176: Motor = self_1101;
    let _e180: MultiVector = other_927;
    let _e191: Motor = self_1101;
    let _e195: MultiVector = other_927;
    let _e207: Motor = self_1101;
    let _e211: MultiVector = other_927;
    let _e215: Motor = self_1101;
    let _e219: MultiVector = other_927;
    let _e231: Motor = self_1101;
    let _e235: MultiVector = other_927;
    let _e247: Motor = self_1101;
    let _e250: Motor = self_1101;
    let _e253: Motor = self_1101;
    let _e256: Motor = self_1101;
    let _e260: MultiVector = other_927;
    let _e263: MultiVector = other_927;
    let _e266: MultiVector = other_927;
    let _e269: MultiVector = other_927;
    let _e282: Motor = self_1101;
    let _e286: MultiVector = other_927;
    let _e289: Motor = self_1101;
    let _e291: MultiVector = other_927;
    return MultiVector((((((((((vec4<f32>(_e4.g0_.y) * _e8.g3_.yxyy) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) + ((vec4<f32>(_e18.g0_.z) * _e22.g3_.zzxz) * vec4<f32>(1.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e33.g0_.w) * _e37.g3_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, 1.0))) + (vec4<f32>(_e48.g1_.x) * _e52.g0_)) + ((vec4<f32>(_e56.g1_.y) * vec4<f32>(_e60.g0_.y)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e72.g1_.z) * vec4<f32>(_e76.g0_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e88.g1_.w) * vec4<f32>(_e92.g0_.w)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e104.g0_.x) * vec4<f32>(_e108.g3_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))), ((((vec4<f32>(_e120.g1_.x) * _e124.g1_) + ((vec4<f32>(_e127.g1_.z) * vec4<f32>(_e131.g1_.z)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((vec4<f32>(_e144.g1_.w) * vec4<f32>(_e148.g1_.w)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((_e161.g1_.yxxx * _e164.g1_.yxxx) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))), (((((((vec4<f32>(_e176.g0_.z) * _e180.g1_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e191.g0_.w) * _e195.g1_.zzyz) * vec4<f32>(0.0, -(1.0), 1.0, 0.0))) + (vec4<f32>(_e207.g1_.x) * _e211.g2_)) + ((vec4<f32>(_e215.g1_.z) * vec4<f32>(_e219.g2_.x)) * vec4<f32>(0.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e231.g1_.w) * vec4<f32>(_e235.g2_.x)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e247.g0_.x, _e250.g1_.y, _e253.g0_.y, _e256.g0_.y) * vec4<f32>(_e260.g1_.x, _e263.g2_.x, _e266.g1_.w, _e269.g1_.z)) * vec4<f32>(0.0, 1.0, -(1.0), 1.0))), ((vec4<f32>(_e282.g1_.x) * _e286.g3_) + ((_e289.g1_ * vec4<f32>(_e291.g3_.x)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn motor_multi_vector_outer_product(self_1102: Motor, other_928: MultiVector) -> MultiVector {
    var self_1103: Motor;
    var other_929: MultiVector;

    self_1103 = self_1102;
    other_929 = other_928;
    let _e4: Motor = self_1103;
    let _e8: MultiVector = other_929;
    let _e11: Motor = self_1103;
    let _e13: MultiVector = other_929;
    let _e25: Motor = self_1103;
    let _e29: MultiVector = other_929;
    let _e32: Motor = self_1103;
    let _e36: MultiVector = other_929;
    let _e48: Motor = self_1103;
    let _e52: MultiVector = other_929;
    let _e64: Motor = self_1103;
    let _e68: MultiVector = other_929;
    let _e80: Motor = self_1103;
    let _e82: MultiVector = other_929;
    let _e97: Motor = self_1103;
    let _e101: MultiVector = other_929;
    let _e104: Motor = self_1103;
    let _e108: MultiVector = other_929;
    let _e120: Motor = self_1103;
    let _e124: MultiVector = other_929;
    let _e136: Motor = self_1103;
    let _e139: MultiVector = other_929;
    let _e150: Motor = self_1103;
    let _e154: MultiVector = other_929;
    let _e157: Motor = self_1103;
    let _e161: MultiVector = other_929;
    let _e173: Motor = self_1103;
    let _e177: MultiVector = other_929;
    let _e189: Motor = self_1103;
    let _e193: MultiVector = other_929;
    let _e205: Motor = self_1103;
    let _e209: MultiVector = other_929;
    let _e220: Motor = self_1103;
    let _e224: MultiVector = other_929;
    let _e235: Motor = self_1103;
    let _e239: MultiVector = other_929;
    let _e250: Motor = self_1103;
    let _e253: MultiVector = other_929;
    return MultiVector(((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((_e11.g0_ * vec4<f32>(_e13.g0_.x)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))), (((((vec4<f32>(_e25.g0_.x) * _e29.g1_) + ((vec4<f32>(_e32.g1_.y) * _e36.g2_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e48.g1_.z) * _e52.g2_.wwwy) * vec4<f32>(0.0, -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e64.g1_.w) * _e68.g2_.zzyz) * vec4<f32>(0.0, 1.0, -(1.0), 0.0))) + ((_e80.g0_ * vec4<f32>(_e82.g1_.x)) * vec4<f32>(0.0, -(1.0), -(1.0), -(1.0)))), ((((vec4<f32>(_e97.g0_.x) * _e101.g2_) + ((vec4<f32>(_e104.g0_.z) * vec4<f32>(_e108.g2_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e120.g0_.w) * vec4<f32>(_e124.g2_.w)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((_e136.g0_.yxxx * _e139.g2_.yxxx) * vec4<f32>(1.0, 0.0, 0.0, 0.0))), ((((((((vec4<f32>(_e150.g0_.x) * _e154.g3_) + ((vec4<f32>(_e157.g0_.z) * vec4<f32>(_e161.g3_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e173.g0_.w) * vec4<f32>(_e177.g3_.w)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e189.g1_.x) * vec4<f32>(_e193.g0_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e205.g1_.y) * _e209.g0_.yxyy) * vec4<f32>(1.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e220.g1_.z) * _e224.g0_.zzxz) * vec4<f32>(1.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e235.g1_.w) * _e239.g0_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, 1.0))) + ((_e250.g0_.yxxx * _e253.g3_.yxxx) * vec4<f32>(1.0, 0.0, 0.0, 0.0))));
}

fn motor_multi_vector_inner_product(self_1104: Motor, other_930: MultiVector) -> MultiVector {
    var self_1105: Motor;
    var other_931: MultiVector;

    self_1105 = self_1104;
    other_931 = other_930;
    let _e4: Motor = self_1105;
    let _e8: MultiVector = other_931;
    let _e11: Motor = self_1105;
    let _e15: MultiVector = other_931;
    let _e27: Motor = self_1105;
    let _e31: MultiVector = other_931;
    let _e43: Motor = self_1105;
    let _e47: MultiVector = other_931;
    let _e60: Motor = self_1105;
    let _e64: MultiVector = other_931;
    let _e77: Motor = self_1105;
    let _e81: MultiVector = other_931;
    let _e94: Motor = self_1105;
    let _e98: MultiVector = other_931;
    let _e111: Motor = self_1105;
    let _e114: MultiVector = other_931;
    let _e126: Motor = self_1105;
    let _e130: MultiVector = other_931;
    let _e133: Motor = self_1105;
    let _e137: MultiVector = other_931;
    let _e149: Motor = self_1105;
    let _e153: MultiVector = other_931;
    let _e165: Motor = self_1105;
    let _e169: MultiVector = other_931;
    let _e173: Motor = self_1105;
    let _e177: MultiVector = other_931;
    let _e189: Motor = self_1105;
    let _e193: MultiVector = other_931;
    let _e205: Motor = self_1105;
    let _e209: MultiVector = other_931;
    let _e221: Motor = self_1105;
    let _e224: MultiVector = other_931;
    let _e235: Motor = self_1105;
    let _e239: MultiVector = other_931;
    let _e242: Motor = self_1105;
    let _e246: MultiVector = other_931;
    let _e259: Motor = self_1105;
    let _e263: MultiVector = other_931;
    let _e276: Motor = self_1105;
    let _e280: MultiVector = other_931;
    let _e284: Motor = self_1105;
    let _e288: MultiVector = other_931;
    let _e301: Motor = self_1105;
    let _e305: MultiVector = other_931;
    let _e318: Motor = self_1105;
    let _e322: MultiVector = other_931;
    let _e335: Motor = self_1105;
    let _e338: MultiVector = other_931;
    let _e351: Motor = self_1105;
    let _e355: MultiVector = other_931;
    let _e358: Motor = self_1105;
    let _e362: MultiVector = other_931;
    let _e375: Motor = self_1105;
    let _e379: MultiVector = other_931;
    let _e391: Motor = self_1105;
    let _e395: MultiVector = other_931;
    let _e407: Motor = self_1105;
    let _e411: MultiVector = other_931;
    let _e423: Motor = self_1105;
    let _e425: MultiVector = other_931;
    return MultiVector(((((((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.z) * _e15.g0_.zzxz) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0))) + ((vec4<f32>(_e27.g0_.w) * _e31.g0_.wwwx) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))) + ((vec4<f32>(_e43.g1_.x) * _e47.g3_) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0)))) + ((vec4<f32>(_e60.g1_.y) * _e64.g3_.yxyy) * vec4<f32>(-(1.0), -(1.0), 0.0, 0.0))) + ((vec4<f32>(_e77.g1_.z) * _e81.g3_.zzxz) * vec4<f32>(-(1.0), 0.0, -(1.0), 0.0))) + ((vec4<f32>(_e94.g1_.w) * _e98.g3_.wwwx) * vec4<f32>(-(1.0), 0.0, 0.0, -(1.0)))) + ((_e111.g0_.yyxx * _e114.g0_.yxxx) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))), ((((((((vec4<f32>(_e126.g0_.x) * _e130.g1_) + ((vec4<f32>(_e133.g0_.z) * vec4<f32>(_e137.g1_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e149.g0_.w) * vec4<f32>(_e153.g1_.w)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) - (vec4<f32>(_e165.g1_.x) * _e169.g2_)) + ((vec4<f32>(_e173.g1_.y) * vec4<f32>(_e177.g2_.y)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e189.g1_.z) * vec4<f32>(_e193.g2_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e205.g1_.w) * vec4<f32>(_e209.g2_.w)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((_e221.g0_.yxxx * _e224.g1_.yxxx) * vec4<f32>(1.0, 0.0, 0.0, 0.0))), ((((((((vec4<f32>(_e235.g0_.x) * _e239.g2_) + ((vec4<f32>(_e242.g0_.z) * _e246.g2_.wwxy) * vec4<f32>(0.0, -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e259.g0_.w) * _e263.g2_.zzyx) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))) - (vec4<f32>(_e276.g1_.x) * _e280.g1_)) + ((vec4<f32>(_e284.g1_.y) * _e288.g1_.xxwz) * vec4<f32>(0.0, -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e301.g1_.z) * _e305.g1_.wwxy) * vec4<f32>(0.0, -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e318.g1_.w) * _e322.g1_.zzyx) * vec4<f32>(0.0, 1.0, -(1.0), -(1.0)))) + ((_e335.g0_.xyyy * _e338.g2_.xxwz) * vec4<f32>(0.0, -(1.0), 1.0, -(1.0)))), ((((((vec4<f32>(_e351.g0_.x) * _e355.g3_) + ((vec4<f32>(_e358.g1_.x) * _e362.g0_) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0)))) + ((vec4<f32>(_e375.g1_.y) * vec4<f32>(_e379.g0_.x)) * vec4<f32>(0.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e391.g1_.z) * vec4<f32>(_e395.g0_.x)) * vec4<f32>(0.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e407.g1_.w) * vec4<f32>(_e411.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((_e423.g0_ * vec4<f32>(_e425.g3_.x)) * vec4<f32>(0.0, -(1.0), -(1.0), -(1.0)))));
}

fn motor_multi_vector_left_contraction(self_1106: Motor, other_932: MultiVector) -> MultiVector {
    var self_1107: Motor;
    var other_933: MultiVector;

    self_1107 = self_1106;
    other_933 = other_932;
    let _e4: Motor = self_1107;
    let _e8: MultiVector = other_933;
    let _e11: Motor = self_1107;
    let _e15: MultiVector = other_933;
    let _e28: Motor = self_1107;
    let _e32: MultiVector = other_933;
    let _e45: Motor = self_1107;
    let _e49: MultiVector = other_933;
    let _e61: Motor = self_1107;
    let _e65: MultiVector = other_933;
    let _e78: Motor = self_1107;
    let _e82: MultiVector = other_933;
    let _e95: Motor = self_1107;
    let _e99: MultiVector = other_933;
    let _e112: Motor = self_1107;
    let _e115: MultiVector = other_933;
    let _e127: Motor = self_1107;
    let _e131: MultiVector = other_933;
    let _e134: Motor = self_1107;
    let _e138: MultiVector = other_933;
    let _e150: Motor = self_1107;
    let _e154: MultiVector = other_933;
    let _e166: Motor = self_1107;
    let _e169: MultiVector = other_933;
    let _e180: Motor = self_1107;
    let _e184: MultiVector = other_933;
    let _e187: Motor = self_1107;
    let _e191: MultiVector = other_933;
    let _e203: Motor = self_1107;
    let _e207: MultiVector = other_933;
    let _e219: Motor = self_1107;
    let _e223: MultiVector = other_933;
    let _e235: Motor = self_1107;
    let _e237: MultiVector = other_933;
    let _e252: Motor = self_1107;
    let _e256: MultiVector = other_933;
    let _e259: Motor = self_1107;
    let _e261: MultiVector = other_933;
    return MultiVector(((((((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.z) * vec4<f32>(_e15.g0_.z)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((vec4<f32>(_e28.g0_.w) * vec4<f32>(_e32.g0_.w)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((vec4<f32>(_e45.g1_.x) * vec4<f32>(_e49.g3_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e61.g1_.y) * _e65.g3_.yxyy) * vec4<f32>(-(1.0), -(1.0), 0.0, 0.0))) + ((vec4<f32>(_e78.g1_.z) * _e82.g3_.zzxz) * vec4<f32>(-(1.0), 0.0, -(1.0), 0.0))) + ((vec4<f32>(_e95.g1_.w) * _e99.g3_.wwwx) * vec4<f32>(-(1.0), 0.0, 0.0, -(1.0)))) + ((_e112.g0_.yxxx * _e115.g0_.yxxx) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))), ((((vec4<f32>(_e127.g0_.x) * _e131.g1_) + ((vec4<f32>(_e134.g0_.z) * vec4<f32>(_e138.g1_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e150.g0_.w) * vec4<f32>(_e154.g1_.w)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((_e166.g0_.yxxx * _e169.g1_.yxxx) * vec4<f32>(1.0, 0.0, 0.0, 0.0))), (((((vec4<f32>(_e180.g0_.x) * _e184.g2_) + ((vec4<f32>(_e187.g1_.y) * _e191.g1_.wwwz) * vec4<f32>(0.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e203.g1_.z) * _e207.g1_.wwwy) * vec4<f32>(0.0, -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e219.g1_.w) * _e223.g1_.zzyz) * vec4<f32>(0.0, 1.0, -(1.0), 0.0))) + ((_e235.g0_ * vec4<f32>(_e237.g2_.x)) * vec4<f32>(0.0, -(1.0), -(1.0), -(1.0)))), ((vec4<f32>(_e252.g0_.x) * _e256.g3_) + ((_e259.g0_ * vec4<f32>(_e261.g3_.x)) * vec4<f32>(0.0, -(1.0), -(1.0), -(1.0)))));
}

fn motor_multi_vector_right_contraction(self_1108: Motor, other_934: MultiVector) -> MultiVector {
    var self_1109: Motor;
    var other_935: MultiVector;

    self_1109 = self_1108;
    other_935 = other_934;
    let _e4: Motor = self_1109;
    let _e8: MultiVector = other_935;
    let _e19: Motor = self_1109;
    let _e23: MultiVector = other_935;
    let _e35: Motor = self_1109;
    let _e39: MultiVector = other_935;
    let _e51: Motor = self_1109;
    let _e55: MultiVector = other_935;
    let _e68: Motor = self_1109;
    let _e72: MultiVector = other_935;
    let _e85: Motor = self_1109;
    let _e89: MultiVector = other_935;
    let _e102: Motor = self_1109;
    let _e106: MultiVector = other_935;
    let _e119: Motor = self_1109;
    let _e123: MultiVector = other_935;
    let _e137: Motor = self_1109;
    let _e141: MultiVector = other_935;
    let _e145: Motor = self_1109;
    let _e149: MultiVector = other_935;
    let _e161: Motor = self_1109;
    let _e165: MultiVector = other_935;
    let _e177: Motor = self_1109;
    let _e180: MultiVector = other_935;
    let _e191: Motor = self_1109;
    let _e195: MultiVector = other_935;
    let _e206: Motor = self_1109;
    let _e210: MultiVector = other_935;
    let _e222: Motor = self_1109;
    let _e226: MultiVector = other_935;
    let _e230: Motor = self_1109;
    let _e234: MultiVector = other_935;
    let _e247: Motor = self_1109;
    let _e251: MultiVector = other_935;
    let _e264: Motor = self_1109;
    let _e267: Motor = self_1109;
    let _e270: Motor = self_1109;
    let _e273: Motor = self_1109;
    let _e277: MultiVector = other_935;
    let _e280: MultiVector = other_935;
    let _e283: MultiVector = other_935;
    let _e286: MultiVector = other_935;
    let _e300: Motor = self_1109;
    let _e304: MultiVector = other_935;
    let _e316: Motor = self_1109;
    let _e318: MultiVector = other_935;
    return MultiVector((((((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0)) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.zzxz) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0))) + ((vec4<f32>(_e35.g0_.w) * _e39.g0_.wwwx) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))) + ((vec4<f32>(_e51.g1_.x) * _e55.g3_) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0)))) + ((vec4<f32>(_e68.g1_.y) * vec4<f32>(_e72.g3_.y)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((vec4<f32>(_e85.g1_.z) * vec4<f32>(_e89.g3_.z)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((vec4<f32>(_e102.g1_.w) * vec4<f32>(_e106.g3_.w)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((vec4<f32>(_e119.g0_.x) * vec4<f32>(_e123.g0_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))), ((((vec4<f32>(0.0) - (vec4<f32>(_e137.g1_.x) * _e141.g2_)) + ((vec4<f32>(_e145.g1_.z) * vec4<f32>(_e149.g2_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e161.g1_.w) * vec4<f32>(_e165.g2_.w)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((_e177.g1_.yxxx * _e180.g2_.yxxx) * vec4<f32>(1.0, 0.0, 0.0, 0.0))), (((((((vec4<f32>(_e191.g0_.z) * _e195.g2_.wwwy) * vec4<f32>(0.0, -(1.0), 0.0, 1.0)) + ((vec4<f32>(_e206.g0_.w) * _e210.g2_.zzyz) * vec4<f32>(0.0, 1.0, -(1.0), 0.0))) - (vec4<f32>(_e222.g1_.x) * _e226.g1_)) + ((vec4<f32>(_e230.g1_.z) * vec4<f32>(_e234.g1_.x)) * vec4<f32>(0.0, 0.0, -(1.0), 0.0))) + ((vec4<f32>(_e247.g1_.w) * vec4<f32>(_e251.g1_.x)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e264.g0_.x, _e267.g1_.y, _e270.g0_.y, _e273.g0_.y) * vec4<f32>(_e277.g2_.x, _e280.g1_.x, _e283.g2_.w, _e286.g2_.z)) * vec4<f32>(0.0, -(1.0), 1.0, -(1.0)))), (((vec4<f32>(_e300.g1_.x) * _e304.g0_) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))) + ((_e316.g1_ * vec4<f32>(_e318.g0_.x)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn motor_multi_vector_scalar_product(self_1110: Motor, other_936: MultiVector) -> Scalar {
    var self_1111: Motor;
    var other_937: MultiVector;

    self_1111 = self_1110;
    other_937 = other_936;
    let _e4: Motor = self_1111;
    let _e7: MultiVector = other_937;
    let _e11: Motor = self_1111;
    let _e14: MultiVector = other_937;
    let _e19: Motor = self_1111;
    let _e22: MultiVector = other_937;
    let _e27: Motor = self_1111;
    let _e30: MultiVector = other_937;
    let _e35: Motor = self_1111;
    let _e38: MultiVector = other_937;
    let _e43: Motor = self_1111;
    let _e46: MultiVector = other_937;
    let _e51: Motor = self_1111;
    let _e54: MultiVector = other_937;
    let _e59: Motor = self_1111;
    let _e62: MultiVector = other_937;
    return Scalar(((((((((_e4.g0_.x * _e7.g0_.x) - (_e11.g0_.y * _e14.g0_.y)) - (_e19.g0_.z * _e22.g0_.z)) - (_e27.g0_.w * _e30.g0_.w)) + (_e35.g1_.x * _e38.g3_.x)) - (_e43.g1_.y * _e46.g3_.y)) - (_e51.g1_.z * _e54.g3_.z)) - (_e59.g1_.w * _e62.g3_.w)));
}

fn motor_rotor_into(self_1112: Motor) -> Rotor {
    var self_1113: Motor;

    self_1113 = self_1112;
    let _e2: Motor = self_1113;
    return Rotor(_e2.g0_);
}

fn motor_rotor_add(self_1114: Motor, other_938: Rotor) -> Motor {
    var self_1115: Motor;
    var other_939: Rotor;

    self_1115 = self_1114;
    other_939 = other_938;
    let _e4: Motor = self_1115;
    let _e6: Rotor = other_939;
    let _e9: Motor = self_1115;
    return Motor((_e4.g0_ + _e6.g0_), _e9.g1_);
}

fn motor_rotor_sub(self_1116: Motor, other_940: Rotor) -> Motor {
    var self_1117: Motor;
    var other_941: Rotor;

    self_1117 = self_1116;
    other_941 = other_940;
    let _e4: Motor = self_1117;
    let _e6: Rotor = other_941;
    let _e9: Motor = self_1117;
    return Motor((_e4.g0_ - _e6.g0_), _e9.g1_);
}

fn motor_rotor_geometric_product(self_1118: Motor, other_942: Rotor) -> Motor {
    var self_1119: Motor;
    var other_943: Rotor;

    self_1119 = self_1118;
    other_943 = other_942;
    let _e4: Motor = self_1119;
    let _e8: Rotor = other_943;
    let _e11: Motor = self_1119;
    let _e15: Rotor = other_943;
    let _e28: Motor = self_1119;
    let _e32: Rotor = other_943;
    let _e45: Motor = self_1119;
    let _e49: Rotor = other_943;
    let _e62: Motor = self_1119;
    let _e66: Rotor = other_943;
    let _e78: Motor = self_1119;
    let _e82: Rotor = other_943;
    let _e94: Motor = self_1119;
    let _e98: Rotor = other_943;
    let _e110: Motor = self_1119;
    let _e114: Rotor = other_943;
    return Motor(((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.y) * _e15.g0_.yxwz) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e28.g0_.z) * _e32.g0_.zwxy) * vec4<f32>(-(1.0), -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e45.g0_.w) * _e49.g0_.wzyx) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))), (((((vec4<f32>(_e62.g1_.x) * _e66.g0_) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))) + ((vec4<f32>(_e78.g1_.y) * _e82.g0_.yxwz) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e94.g1_.z) * _e98.g0_.zwxy) * vec4<f32>(1.0, -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e110.g1_.w) * _e114.g0_.wzyx) * vec4<f32>(1.0, 1.0, -(1.0), 1.0))));
}

fn motor_rotor_regressive_product(self_1120: Motor, other_944: Rotor) -> Rotor {
    var self_1121: Motor;
    var other_945: Rotor;

    self_1121 = self_1120;
    other_945 = other_944;
    let _e4: Motor = self_1121;
    let _e8: Rotor = other_945;
    let _e11: Motor = self_1121;
    let _e15: Rotor = other_945;
    let _e27: Motor = self_1121;
    let _e31: Rotor = other_945;
    let _e43: Motor = self_1121;
    let _e46: Rotor = other_945;
    return Rotor(((((vec4<f32>(_e4.g1_.x) * _e8.g0_) + ((vec4<f32>(_e11.g1_.z) * vec4<f32>(_e15.g0_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e27.g1_.w) * vec4<f32>(_e31.g0_.w)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((_e43.g1_.yxxx * _e46.g0_.yxxx) * vec4<f32>(1.0, 0.0, 0.0, 0.0))));
}

fn motor_rotor_outer_product(self_1122: Motor, other_946: Rotor) -> Motor {
    var self_1123: Motor;
    var other_947: Rotor;

    self_1123 = self_1122;
    other_947 = other_946;
    let _e4: Motor = self_1123;
    let _e8: Rotor = other_947;
    let _e11: Motor = self_1123;
    let _e13: Rotor = other_947;
    let _e25: Motor = self_1123;
    let _e29: Rotor = other_947;
    let _e39: Motor = self_1123;
    let _e43: Rotor = other_947;
    let _e54: Motor = self_1123;
    let _e58: Rotor = other_947;
    let _e69: Motor = self_1123;
    let _e73: Rotor = other_947;
    return Motor(((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((_e11.g0_ * vec4<f32>(_e13.g0_.x)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))), (((((vec4<f32>(_e25.g1_.y) * _e29.g0_.yxyy) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) + ((vec4<f32>(_e39.g1_.z) * _e43.g0_.zzxz) * vec4<f32>(1.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e54.g1_.w) * _e58.g0_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e69.g1_.x) * vec4<f32>(_e73.g0_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))));
}

fn motor_rotor_inner_product(self_1124: Motor, other_948: Rotor) -> Motor {
    var self_1125: Motor;
    var other_949: Rotor;

    self_1125 = self_1124;
    other_949 = other_948;
    let _e4: Motor = self_1125;
    let _e8: Rotor = other_949;
    let _e11: Motor = self_1125;
    let _e15: Rotor = other_949;
    let _e27: Motor = self_1125;
    let _e31: Rotor = other_949;
    let _e43: Motor = self_1125;
    let _e46: Rotor = other_949;
    let _e58: Motor = self_1125;
    let _e62: Rotor = other_949;
    let _e74: Motor = self_1125;
    let _e76: Rotor = other_949;
    return Motor(((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.z) * _e15.g0_.zzxz) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0))) + ((vec4<f32>(_e27.g0_.w) * _e31.g0_.wwwx) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))) + ((_e43.g0_.yyxx * _e46.g0_.yxxx) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))), (((vec4<f32>(_e58.g1_.x) * _e62.g0_) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))) + ((_e74.g1_ * vec4<f32>(_e76.g0_.x)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn motor_rotor_left_contraction(self_1126: Motor, other_950: Rotor) -> Rotor {
    var self_1127: Motor;
    var other_951: Rotor;

    self_1127 = self_1126;
    other_951 = other_950;
    let _e4: Motor = self_1127;
    let _e8: Rotor = other_951;
    let _e11: Motor = self_1127;
    let _e15: Rotor = other_951;
    let _e28: Motor = self_1127;
    let _e32: Rotor = other_951;
    let _e45: Motor = self_1127;
    let _e48: Rotor = other_951;
    return Rotor(((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.z) * vec4<f32>(_e15.g0_.z)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((vec4<f32>(_e28.g0_.w) * vec4<f32>(_e32.g0_.w)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((_e45.g0_.yxxx * _e48.g0_.yxxx) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))));
}

fn motor_rotor_right_contraction(self_1128: Motor, other_952: Rotor) -> Motor {
    var self_1129: Motor;
    var other_953: Rotor;

    self_1129 = self_1128;
    other_953 = other_952;
    let _e4: Motor = self_1129;
    let _e8: Rotor = other_953;
    let _e19: Motor = self_1129;
    let _e23: Rotor = other_953;
    let _e35: Motor = self_1129;
    let _e39: Rotor = other_953;
    let _e51: Motor = self_1129;
    let _e55: Rotor = other_953;
    let _e67: Motor = self_1129;
    let _e71: Rotor = other_953;
    let _e83: Motor = self_1129;
    let _e85: Rotor = other_953;
    return Motor((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0)) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.zzxz) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0))) + ((vec4<f32>(_e35.g0_.w) * _e39.g0_.wwwx) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))) + ((vec4<f32>(_e51.g0_.x) * vec4<f32>(_e55.g0_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))), (((vec4<f32>(_e67.g1_.x) * _e71.g0_) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))) + ((_e83.g1_ * vec4<f32>(_e85.g0_.x)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn motor_rotor_scalar_product(self_1130: Motor, other_954: Rotor) -> Scalar {
    var self_1131: Motor;
    var other_955: Rotor;

    self_1131 = self_1130;
    other_955 = other_954;
    let _e4: Motor = self_1131;
    let _e7: Rotor = other_955;
    let _e11: Motor = self_1131;
    let _e14: Rotor = other_955;
    let _e19: Motor = self_1131;
    let _e22: Rotor = other_955;
    let _e27: Motor = self_1131;
    let _e30: Rotor = other_955;
    return Scalar(((((_e4.g0_.x * _e7.g0_.x) - (_e11.g0_.y * _e14.g0_.y)) - (_e19.g0_.z * _e22.g0_.z)) - (_e27.g0_.w * _e30.g0_.w)));
}

fn motor_point_geometric_product(self_1132: Motor, other_956: Point) -> PointAndPlane {
    var self_1133: Motor;
    var other_957: Point;

    self_1133 = self_1132;
    other_957 = other_956;
    let _e4: Motor = self_1133;
    let _e8: Point = other_957;
    let _e11: Motor = self_1133;
    let _e15: Point = other_957;
    let _e27: Motor = self_1133;
    let _e31: Point = other_957;
    let _e43: Motor = self_1133;
    let _e47: Point = other_957;
    let _e59: Motor = self_1133;
    let _e63: Point = other_957;
    let _e75: Motor = self_1133;
    let _e78: Motor = self_1133;
    let _e81: Motor = self_1133;
    let _e84: Motor = self_1133;
    let _e88: Point = other_957;
    let _e101: Motor = self_1133;
    let _e105: Point = other_957;
    let _e116: Motor = self_1133;
    let _e120: Point = other_957;
    let _e132: Motor = self_1133;
    let _e136: Point = other_957;
    let _e140: Motor = self_1133;
    let _e144: Point = other_957;
    let _e156: Motor = self_1133;
    let _e160: Point = other_957;
    let _e172: Motor = self_1133;
    let _e175: Motor = self_1133;
    let _e178: Motor = self_1133;
    let _e181: Motor = self_1133;
    let _e185: Point = other_957;
    return PointAndPlane(((((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.z) * _e15.g0_.wwwy) * vec4<f32>(0.0, -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e27.g0_.w) * _e31.g0_.zzyz) * vec4<f32>(0.0, 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e43.g1_.z) * _e47.g0_.zzxz) * vec4<f32>(1.0, 0.0, -(1.0), 0.0))) + ((vec4<f32>(_e59.g1_.w) * _e63.g0_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e75.g1_.y, _e78.g1_.y, _e81.g0_.y, _e84.g0_.y) * _e88.g0_.yxwz) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))), (((((((vec4<f32>(_e101.g0_.z) * _e105.g0_.zzxz) * vec4<f32>(1.0, 0.0, -(1.0), 0.0)) + ((vec4<f32>(_e116.g0_.w) * _e120.g0_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))) - (vec4<f32>(_e132.g1_.x) * _e136.g0_)) + ((vec4<f32>(_e140.g1_.z) * _e144.g0_.wwwy) * vec4<f32>(0.0, -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e156.g1_.w) * _e160.g0_.zzyz) * vec4<f32>(0.0, 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e172.g0_.y, _e175.g0_.y, _e178.g1_.y, _e181.g1_.y) * _e185.g0_.yxwz) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))));
}

fn motor_point_regressive_product(self_1134: Motor, other_958: Point) -> PointAndPlane {
    var self_1135: Motor;
    var other_959: Point;

    self_1135 = self_1134;
    other_959 = other_958;
    let _e4: Motor = self_1135;
    let _e8: Point = other_959;
    let _e11: Motor = self_1135;
    let _e15: Point = other_959;
    let _e26: Motor = self_1135;
    let _e30: Point = other_959;
    let _e42: Motor = self_1135;
    let _e46: Point = other_959;
    let _e58: Motor = self_1135;
    let _e62: Point = other_959;
    let _e74: Motor = self_1135;
    let _e77: Motor = self_1135;
    let _e80: Motor = self_1135;
    let _e83: Motor = self_1135;
    let _e87: Point = other_959;
    return PointAndPlane((vec4<f32>(_e4.g1_.x) * _e8.g0_), ((((((vec4<f32>(_e11.g0_.z) * _e15.g0_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e26.g0_.w) * _e30.g0_.zzyz) * vec4<f32>(0.0, -(1.0), 1.0, 0.0))) + ((vec4<f32>(_e42.g1_.z) * _e46.g0_.zzxz) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0))) + ((vec4<f32>(_e58.g1_.w) * _e62.g0_.wwwx) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))) + ((vec4<f32>(_e74.g1_.y, _e77.g1_.y, _e80.g0_.y, _e83.g0_.y) * _e87.g0_.yxwz) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))));
}

fn motor_point_outer_product(self_1136: Motor, other_960: Point) -> Point {
    var self_1137: Motor;
    var other_961: Point;

    self_1137 = self_1136;
    other_961 = other_960;
    let _e4: Motor = self_1137;
    let _e8: Point = other_961;
    return Point((vec4<f32>(_e4.g0_.x) * _e8.g0_));
}

fn motor_point_inner_product(self_1138: Motor, other_962: Point) -> PointAndPlane {
    var self_1139: Motor;
    var other_963: Point;

    self_1139 = self_1138;
    other_963 = other_962;
    let _e4: Motor = self_1139;
    let _e8: Point = other_963;
    let _e11: Motor = self_1139;
    let _e15: Point = other_963;
    let _e26: Motor = self_1139;
    let _e30: Point = other_963;
    let _e42: Motor = self_1139;
    let _e46: Point = other_963;
    let _e50: Motor = self_1139;
    let _e54: Point = other_963;
    let _e66: Motor = self_1139;
    let _e70: Point = other_963;
    let _e82: Motor = self_1139;
    let _e85: Motor = self_1139;
    let _e88: Motor = self_1139;
    let _e91: Motor = self_1139;
    let _e95: Point = other_963;
    return PointAndPlane((vec4<f32>(_e4.g0_.x) * _e8.g0_), (((((((vec4<f32>(_e11.g0_.z) * _e15.g0_.zzxz) * vec4<f32>(1.0, 0.0, -(1.0), 0.0)) + ((vec4<f32>(_e26.g0_.w) * _e30.g0_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))) - (vec4<f32>(_e42.g1_.x) * _e46.g0_)) + ((vec4<f32>(_e50.g1_.z) * _e54.g0_.wwwy) * vec4<f32>(0.0, -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e66.g1_.w) * _e70.g0_.zzyz) * vec4<f32>(0.0, 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e82.g0_.y, _e85.g0_.y, _e88.g1_.y, _e91.g1_.y) * _e95.g0_.yxwz) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))));
}

fn motor_point_left_contraction(self_1140: Motor, other_964: Point) -> PointAndPlane {
    var self_1141: Motor;
    var other_965: Point;

    self_1141 = self_1140;
    other_965 = other_964;
    let _e4: Motor = self_1141;
    let _e8: Point = other_965;
    let _e11: Motor = self_1141;
    let _e15: Point = other_965;
    let _e26: Motor = self_1141;
    let _e30: Point = other_965;
    let _e42: Motor = self_1141;
    let _e46: Point = other_965;
    let _e58: Motor = self_1141;
    let _e62: Point = other_965;
    let _e74: Motor = self_1141;
    let _e77: Motor = self_1141;
    let _e80: Motor = self_1141;
    let _e83: Motor = self_1141;
    let _e87: Point = other_965;
    return PointAndPlane((vec4<f32>(_e4.g0_.x) * _e8.g0_), ((((((vec4<f32>(_e11.g0_.z) * _e15.g0_.zzxz) * vec4<f32>(1.0, 0.0, -(1.0), 0.0)) + ((vec4<f32>(_e26.g0_.w) * _e30.g0_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e42.g1_.z) * _e46.g0_.wwwy) * vec4<f32>(0.0, -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e58.g1_.w) * _e62.g0_.zzyz) * vec4<f32>(0.0, 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e74.g0_.y, _e77.g0_.y, _e80.g1_.y, _e83.g1_.y) * _e87.g0_.yxwz) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))));
}

fn motor_point_right_contraction(self_1142: Motor, other_966: Point) -> Plane {
    var self_1143: Motor;
    var other_967: Point;

    self_1143 = self_1142;
    other_967 = other_966;
    let _e6: Motor = self_1143;
    let _e10: Point = other_967;
    return Plane((vec4<f32>(0.0) - (vec4<f32>(_e6.g1_.x) * _e10.g0_)));
}

fn motor_ideal_point_into(self_1144: Motor) -> IdealPoint {
    var self_1145: Motor;

    self_1145 = self_1144;
    let _e2: Motor = self_1145;
    let _e5: Motor = self_1145;
    let _e8: Motor = self_1145;
    return IdealPoint(vec3<f32>(_e2.g1_.y, _e5.g1_.z, _e8.g1_.w));
}

fn motor_ideal_point_add(self_1146: Motor, other_968: IdealPoint) -> Motor {
    var self_1147: Motor;
    var other_969: IdealPoint;

    self_1147 = self_1146;
    other_969 = other_968;
    let _e4: Motor = self_1147;
    let _e6: Motor = self_1147;
    let _e8: IdealPoint = other_969;
    let _e11: IdealPoint = other_969;
    let _e14: IdealPoint = other_969;
    let _e17: IdealPoint = other_969;
    return Motor(_e4.g0_, (_e6.g1_ + (vec4<f32>(_e8.g0_.x, _e11.g0_.x, _e14.g0_.y, _e17.g0_.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn motor_ideal_point_sub(self_1148: Motor, other_970: IdealPoint) -> Motor {
    var self_1149: Motor;
    var other_971: IdealPoint;

    self_1149 = self_1148;
    other_971 = other_970;
    let _e4: Motor = self_1149;
    let _e6: Motor = self_1149;
    let _e8: IdealPoint = other_971;
    let _e11: IdealPoint = other_971;
    let _e14: IdealPoint = other_971;
    let _e17: IdealPoint = other_971;
    return Motor(_e4.g0_, (_e6.g1_ - (vec4<f32>(_e8.g0_.x, _e11.g0_.x, _e14.g0_.y, _e17.g0_.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn motor_ideal_point_geometric_product(self_1150: Motor, other_972: IdealPoint) -> Motor {
    var self_1151: Motor;
    var other_973: IdealPoint;

    self_1151 = self_1150;
    other_973 = other_972;
    let _e4: Motor = self_1151;
    let _e8: IdealPoint = other_973;
    let _e11: IdealPoint = other_973;
    let _e14: IdealPoint = other_973;
    let _e17: IdealPoint = other_973;
    let _e30: Motor = self_1151;
    let _e34: IdealPoint = other_973;
    let _e37: IdealPoint = other_973;
    let _e40: IdealPoint = other_973;
    let _e43: IdealPoint = other_973;
    let _e57: Motor = self_1151;
    let _e61: IdealPoint = other_973;
    let _e64: IdealPoint = other_973;
    let _e67: IdealPoint = other_973;
    let _e70: IdealPoint = other_973;
    let _e84: Motor = self_1151;
    let _e88: IdealPoint = other_973;
    let _e91: IdealPoint = other_973;
    let _e94: IdealPoint = other_973;
    let _e97: IdealPoint = other_973;
    let _e112: Motor = self_1151;
    let _e116: IdealPoint = other_973;
    let _e119: IdealPoint = other_973;
    let _e122: IdealPoint = other_973;
    let _e125: IdealPoint = other_973;
    let _e137: Motor = self_1151;
    let _e141: IdealPoint = other_973;
    let _e144: IdealPoint = other_973;
    let _e147: IdealPoint = other_973;
    let _e150: IdealPoint = other_973;
    let _e163: Motor = self_1151;
    let _e167: IdealPoint = other_973;
    let _e170: IdealPoint = other_973;
    let _e173: IdealPoint = other_973;
    let _e176: IdealPoint = other_973;
    let _e189: Motor = self_1151;
    let _e193: IdealPoint = other_973;
    let _e196: IdealPoint = other_973;
    let _e199: IdealPoint = other_973;
    let _e202: IdealPoint = other_973;
    return Motor((((((vec4<f32>(_e4.g1_.y) * vec4<f32>(_e8.g0_.x, _e11.g0_.x, _e14.g0_.z, _e17.g0_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e30.g1_.z) * vec4<f32>(_e34.g0_.y, _e37.g0_.z, _e40.g0_.y, _e43.g0_.x)) * vec4<f32>(-(1.0), -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e57.g1_.w) * vec4<f32>(_e61.g0_.z, _e64.g0_.y, _e67.g0_.x, _e70.g0_.z)) * vec4<f32>(-(1.0), 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e84.g1_.x) * vec4<f32>(_e88.g0_.x, _e91.g0_.x, _e94.g0_.y, _e97.g0_.z)) * vec4<f32>(0.0, -(1.0), -(1.0), -(1.0)))), (((((vec4<f32>(_e112.g0_.y) * vec4<f32>(_e116.g0_.x, _e119.g0_.x, _e122.g0_.z, _e125.g0_.y)) * vec4<f32>(1.0, 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e137.g0_.z) * vec4<f32>(_e141.g0_.y, _e144.g0_.z, _e147.g0_.y, _e150.g0_.x)) * vec4<f32>(1.0, -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e163.g0_.w) * vec4<f32>(_e167.g0_.z, _e170.g0_.y, _e173.g0_.x, _e176.g0_.z)) * vec4<f32>(1.0, 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e189.g0_.x) * vec4<f32>(_e193.g0_.x, _e196.g0_.x, _e199.g0_.y, _e202.g0_.z)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn motor_ideal_point_regressive_product(self_1152: Motor, other_974: IdealPoint) -> Translator {
    var self_1153: Motor;
    var other_975: IdealPoint;

    self_1153 = self_1152;
    other_975 = other_974;
    let _e4: Motor = self_1153;
    let _e8: IdealPoint = other_975;
    let _e19: Motor = self_1153;
    let _e23: IdealPoint = other_975;
    let _e35: Motor = self_1153;
    let _e38: Motor = self_1153;
    let _e41: Motor = self_1153;
    let _e44: Motor = self_1153;
    let _e48: IdealPoint = other_975;
    let _e51: IdealPoint = other_975;
    let _e54: IdealPoint = other_975;
    let _e57: IdealPoint = other_975;
    return Translator(((((vec4<f32>(_e4.g0_.z) * vec4<f32>(_e8.g0_.y)) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) + ((vec4<f32>(_e19.g0_.w) * vec4<f32>(_e23.g0_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + (vec4<f32>(_e35.g0_.y, _e38.g1_.x, _e41.g1_.x, _e44.g1_.x) * vec4<f32>(_e48.g0_.x, _e51.g0_.x, _e54.g0_.y, _e57.g0_.z))));
}

fn motor_ideal_point_left_contraction(self_1154: Motor, other_976: IdealPoint) -> Translator {
    var self_1155: Motor;
    var other_977: IdealPoint;

    self_1155 = self_1154;
    other_977 = other_976;
    let _e4: Motor = self_1155;
    let _e8: IdealPoint = other_977;
    let _e20: Motor = self_1155;
    let _e24: IdealPoint = other_977;
    let _e37: Motor = self_1155;
    let _e40: Motor = self_1155;
    let _e43: Motor = self_1155;
    let _e46: Motor = self_1155;
    let _e50: IdealPoint = other_977;
    let _e53: IdealPoint = other_977;
    let _e56: IdealPoint = other_977;
    let _e59: IdealPoint = other_977;
    return Translator(((((vec4<f32>(_e4.g1_.z) * vec4<f32>(_e8.g0_.y)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0)) + ((vec4<f32>(_e20.g1_.w) * vec4<f32>(_e24.g0_.z)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((vec4<f32>(_e37.g1_.y, _e40.g0_.x, _e43.g0_.x, _e46.g0_.x) * vec4<f32>(_e50.g0_.x, _e53.g0_.x, _e56.g0_.y, _e59.g0_.z)) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0))));
}

fn motor_ideal_point_right_contraction(self_1156: Motor, other_978: IdealPoint) -> Rotor {
    var self_1157: Motor;
    var other_979: IdealPoint;

    self_1157 = self_1156;
    other_979 = other_978;
    let _e4: Motor = self_1157;
    let _e8: IdealPoint = other_979;
    let _e20: Motor = self_1157;
    let _e24: IdealPoint = other_979;
    let _e37: Motor = self_1157;
    let _e40: IdealPoint = other_979;
    let _e43: IdealPoint = other_979;
    let _e46: IdealPoint = other_979;
    let _e49: IdealPoint = other_979;
    return Rotor(((((vec4<f32>(_e4.g1_.z) * vec4<f32>(_e8.g0_.y)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0)) + ((vec4<f32>(_e20.g1_.w) * vec4<f32>(_e24.g0_.z)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((_e37.g1_.yxxx * vec4<f32>(_e40.g0_.x, _e43.g0_.x, _e46.g0_.y, _e49.g0_.z)) * vec4<f32>(-(1.0)))));
}

fn motor_ideal_point_scalar_product(self_1158: Motor, other_980: IdealPoint) -> Scalar {
    var self_1159: Motor;
    var other_981: IdealPoint;

    self_1159 = self_1158;
    other_981 = other_980;
    let _e5: Motor = self_1159;
    let _e8: IdealPoint = other_981;
    let _e13: Motor = self_1159;
    let _e16: IdealPoint = other_981;
    let _e21: Motor = self_1159;
    let _e24: IdealPoint = other_981;
    return Scalar((((0.0 - (_e5.g1_.y * _e8.g0_.x)) - (_e13.g1_.z * _e16.g0_.y)) - (_e21.g1_.w * _e24.g0_.z)));
}

fn motor_plane_geometric_product(self_1160: Motor, other_982: Plane) -> PointAndPlane {
    var self_1161: Motor;
    var other_983: Plane;

    self_1161 = self_1160;
    other_983 = other_982;
    let _e4: Motor = self_1161;
    let _e8: Plane = other_983;
    let _e19: Motor = self_1161;
    let _e23: Plane = other_983;
    let _e35: Motor = self_1161;
    let _e39: Plane = other_983;
    let _e43: Motor = self_1161;
    let _e47: Plane = other_983;
    let _e59: Motor = self_1161;
    let _e63: Plane = other_983;
    let _e75: Motor = self_1161;
    let _e78: Motor = self_1161;
    let _e81: Motor = self_1161;
    let _e84: Motor = self_1161;
    let _e88: Plane = other_983;
    let _e101: Motor = self_1161;
    let _e105: Plane = other_983;
    let _e108: Motor = self_1161;
    let _e112: Plane = other_983;
    let _e124: Motor = self_1161;
    let _e128: Plane = other_983;
    let _e140: Motor = self_1161;
    let _e144: Plane = other_983;
    let _e156: Motor = self_1161;
    let _e160: Plane = other_983;
    let _e172: Motor = self_1161;
    let _e175: Motor = self_1161;
    let _e178: Motor = self_1161;
    let _e181: Motor = self_1161;
    let _e185: Plane = other_983;
    return PointAndPlane((((((((vec4<f32>(_e4.g0_.z) * _e8.g0_.zzxz) * vec4<f32>(1.0, 0.0, -(1.0), 0.0)) + ((vec4<f32>(_e19.g0_.w) * _e23.g0_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))) - (vec4<f32>(_e35.g1_.x) * _e39.g0_)) + ((vec4<f32>(_e43.g1_.z) * _e47.g0_.wwwy) * vec4<f32>(0.0, -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e59.g1_.w) * _e63.g0_.zzyz) * vec4<f32>(0.0, 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e75.g0_.y, _e78.g0_.y, _e81.g1_.y, _e84.g1_.y) * _e88.g0_.yxwz) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))), ((((((vec4<f32>(_e101.g0_.x) * _e105.g0_) + ((vec4<f32>(_e108.g0_.z) * _e112.g0_.wwwy) * vec4<f32>(0.0, -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e124.g0_.w) * _e128.g0_.zzyz) * vec4<f32>(0.0, 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e140.g1_.z) * _e144.g0_.zzxz) * vec4<f32>(1.0, 0.0, -(1.0), 0.0))) + ((vec4<f32>(_e156.g1_.w) * _e160.g0_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e172.g1_.y, _e175.g1_.y, _e178.g0_.y, _e181.g0_.y) * _e185.g0_.yxwz) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))));
}

fn motor_plane_regressive_product(self_1162: Motor, other_984: Plane) -> Plane {
    var self_1163: Motor;
    var other_985: Plane;

    self_1163 = self_1162;
    other_985 = other_984;
    let _e4: Motor = self_1163;
    let _e8: Plane = other_985;
    return Plane((vec4<f32>(_e4.g1_.x) * _e8.g0_));
}

fn motor_plane_outer_product(self_1164: Motor, other_986: Plane) -> PointAndPlane {
    var self_1165: Motor;
    var other_987: Plane;

    self_1165 = self_1164;
    other_987 = other_986;
    let _e4: Motor = self_1165;
    let _e8: Plane = other_987;
    let _e19: Motor = self_1165;
    let _e23: Plane = other_987;
    let _e35: Motor = self_1165;
    let _e39: Plane = other_987;
    let _e51: Motor = self_1165;
    let _e55: Plane = other_987;
    let _e67: Motor = self_1165;
    let _e70: Motor = self_1165;
    let _e73: Motor = self_1165;
    let _e76: Motor = self_1165;
    let _e80: Plane = other_987;
    let _e93: Motor = self_1165;
    let _e97: Plane = other_987;
    return PointAndPlane(((((((vec4<f32>(_e4.g0_.z) * _e8.g0_.zzxz) * vec4<f32>(1.0, 0.0, -(1.0), 0.0)) + ((vec4<f32>(_e19.g0_.w) * _e23.g0_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e35.g1_.z) * _e39.g0_.wwwy) * vec4<f32>(0.0, -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e51.g1_.w) * _e55.g0_.zzyz) * vec4<f32>(0.0, 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e67.g0_.y, _e70.g0_.y, _e73.g1_.y, _e76.g1_.y) * _e80.g0_.yxwz) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))), (vec4<f32>(_e93.g0_.x) * _e97.g0_));
}

fn motor_plane_inner_product(self_1166: Motor, other_988: Plane) -> PointAndPlane {
    var self_1167: Motor;
    var other_989: Plane;

    self_1167 = self_1166;
    other_989 = other_988;
    let _e6: Motor = self_1167;
    let _e10: Plane = other_989;
    let _e14: Motor = self_1167;
    let _e18: Plane = other_989;
    let _e21: Motor = self_1167;
    let _e25: Plane = other_989;
    let _e37: Motor = self_1167;
    let _e41: Plane = other_989;
    let _e53: Motor = self_1167;
    let _e57: Plane = other_989;
    let _e69: Motor = self_1167;
    let _e73: Plane = other_989;
    let _e85: Motor = self_1167;
    let _e88: Motor = self_1167;
    let _e91: Motor = self_1167;
    let _e94: Motor = self_1167;
    let _e98: Plane = other_989;
    return PointAndPlane((vec4<f32>(0.0) - (vec4<f32>(_e6.g1_.x) * _e10.g0_)), ((((((vec4<f32>(_e14.g0_.x) * _e18.g0_) + ((vec4<f32>(_e21.g0_.z) * _e25.g0_.wwwy) * vec4<f32>(0.0, -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e37.g0_.w) * _e41.g0_.zzyz) * vec4<f32>(0.0, 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e53.g1_.z) * _e57.g0_.zzxz) * vec4<f32>(1.0, 0.0, -(1.0), 0.0))) + ((vec4<f32>(_e69.g1_.w) * _e73.g0_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e85.g1_.y, _e88.g1_.y, _e91.g0_.y, _e94.g0_.y) * _e98.g0_.yxwz) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))));
}

fn motor_plane_left_contraction(self_1168: Motor, other_990: Plane) -> Plane {
    var self_1169: Motor;
    var other_991: Plane;

    self_1169 = self_1168;
    other_991 = other_990;
    let _e4: Motor = self_1169;
    let _e8: Plane = other_991;
    return Plane((vec4<f32>(_e4.g0_.x) * _e8.g0_));
}

fn motor_plane_right_contraction(self_1170: Motor, other_992: Plane) -> PointAndPlane {
    var self_1171: Motor;
    var other_993: Plane;

    self_1171 = self_1170;
    other_993 = other_992;
    let _e6: Motor = self_1171;
    let _e10: Plane = other_993;
    let _e14: Motor = self_1171;
    let _e18: Plane = other_993;
    let _e29: Motor = self_1171;
    let _e33: Plane = other_993;
    let _e45: Motor = self_1171;
    let _e49: Plane = other_993;
    let _e61: Motor = self_1171;
    let _e65: Plane = other_993;
    let _e77: Motor = self_1171;
    let _e80: Motor = self_1171;
    let _e83: Motor = self_1171;
    let _e86: Motor = self_1171;
    let _e90: Plane = other_993;
    return PointAndPlane((vec4<f32>(0.0) - (vec4<f32>(_e6.g1_.x) * _e10.g0_)), ((((((vec4<f32>(_e14.g0_.z) * _e18.g0_.wwwy) * vec4<f32>(0.0, -(1.0), 0.0, 1.0)) + ((vec4<f32>(_e29.g0_.w) * _e33.g0_.zzyz) * vec4<f32>(0.0, 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e45.g1_.z) * _e49.g0_.zzxz) * vec4<f32>(1.0, 0.0, -(1.0), 0.0))) + ((vec4<f32>(_e61.g1_.w) * _e65.g0_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e77.g1_.y, _e80.g1_.y, _e83.g0_.y, _e86.g0_.y) * _e90.g0_.yxwz) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))));
}

fn motor_line_into(self_1172: Motor) -> Line {
    var self_1173: Motor;

    self_1173 = self_1172;
    let _e2: Motor = self_1173;
    let _e5: Motor = self_1173;
    let _e8: Motor = self_1173;
    let _e12: Motor = self_1173;
    let _e15: Motor = self_1173;
    let _e18: Motor = self_1173;
    return Line(vec3<f32>(_e2.g1_.y, _e5.g1_.z, _e8.g1_.w), vec3<f32>(_e12.g0_.y, _e15.g0_.z, _e18.g0_.w));
}

fn motor_line_add(self_1174: Motor, other_994: Line) -> Motor {
    var self_1175: Motor;
    var other_995: Line;

    self_1175 = self_1174;
    other_995 = other_994;
    let _e4: Motor = self_1175;
    let _e6: Line = other_995;
    let _e9: Line = other_995;
    let _e12: Line = other_995;
    let _e15: Line = other_995;
    let _e26: Motor = self_1175;
    let _e28: Line = other_995;
    let _e31: Line = other_995;
    let _e34: Line = other_995;
    let _e37: Line = other_995;
    return Motor((_e4.g0_ + (vec4<f32>(_e6.g0_.x, _e9.g1_.x, _e12.g1_.y, _e15.g1_.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0))), (_e26.g1_ + (vec4<f32>(_e28.g0_.x, _e31.g0_.x, _e34.g0_.y, _e37.g0_.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn motor_line_sub(self_1176: Motor, other_996: Line) -> Motor {
    var self_1177: Motor;
    var other_997: Line;

    self_1177 = self_1176;
    other_997 = other_996;
    let _e4: Motor = self_1177;
    let _e6: Line = other_997;
    let _e9: Line = other_997;
    let _e12: Line = other_997;
    let _e15: Line = other_997;
    let _e26: Motor = self_1177;
    let _e28: Line = other_997;
    let _e31: Line = other_997;
    let _e34: Line = other_997;
    let _e37: Line = other_997;
    return Motor((_e4.g0_ - (vec4<f32>(_e6.g0_.x, _e9.g1_.x, _e12.g1_.y, _e15.g1_.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0))), (_e26.g1_ - (vec4<f32>(_e28.g0_.x, _e31.g0_.x, _e34.g0_.y, _e37.g0_.z) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn motor_line_geometric_product(self_1178: Motor, other_998: Line) -> Motor {
    var self_1179: Motor;
    var other_999: Line;

    self_1179 = self_1178;
    other_999 = other_998;
    let _e4: Motor = self_1179;
    let _e8: Line = other_999;
    let _e11: Line = other_999;
    let _e14: Line = other_999;
    let _e17: Line = other_999;
    let _e30: Motor = self_1179;
    let _e34: Line = other_999;
    let _e37: Line = other_999;
    let _e40: Line = other_999;
    let _e43: Line = other_999;
    let _e57: Motor = self_1179;
    let _e61: Line = other_999;
    let _e64: Line = other_999;
    let _e67: Line = other_999;
    let _e70: Line = other_999;
    let _e84: Motor = self_1179;
    let _e88: Line = other_999;
    let _e91: Line = other_999;
    let _e94: Line = other_999;
    let _e97: Line = other_999;
    let _e112: Motor = self_1179;
    let _e116: Line = other_999;
    let _e119: Line = other_999;
    let _e122: Line = other_999;
    let _e125: Line = other_999;
    let _e139: Motor = self_1179;
    let _e143: Line = other_999;
    let _e146: Line = other_999;
    let _e149: Line = other_999;
    let _e152: Line = other_999;
    let _e166: Motor = self_1179;
    let _e170: Line = other_999;
    let _e173: Line = other_999;
    let _e176: Line = other_999;
    let _e179: Line = other_999;
    let _e193: Motor = self_1179;
    let _e197: Line = other_999;
    let _e200: Line = other_999;
    let _e203: Line = other_999;
    let _e206: Line = other_999;
    let _e218: Motor = self_1179;
    let _e222: Line = other_999;
    let _e225: Line = other_999;
    let _e228: Line = other_999;
    let _e231: Line = other_999;
    let _e243: Motor = self_1179;
    let _e247: Line = other_999;
    let _e250: Line = other_999;
    let _e253: Line = other_999;
    let _e256: Line = other_999;
    let _e269: Motor = self_1179;
    let _e273: Line = other_999;
    let _e276: Line = other_999;
    let _e279: Line = other_999;
    let _e282: Line = other_999;
    let _e295: Motor = self_1179;
    let _e299: Line = other_999;
    let _e302: Line = other_999;
    let _e305: Line = other_999;
    let _e308: Line = other_999;
    let _e323: Motor = self_1179;
    let _e327: Line = other_999;
    let _e330: Line = other_999;
    let _e333: Line = other_999;
    let _e336: Line = other_999;
    let _e349: Motor = self_1179;
    let _e353: Line = other_999;
    let _e356: Line = other_999;
    let _e359: Line = other_999;
    let _e362: Line = other_999;
    let _e375: Motor = self_1179;
    let _e379: Line = other_999;
    let _e382: Line = other_999;
    let _e385: Line = other_999;
    let _e388: Line = other_999;
    let _e401: Motor = self_1179;
    let _e405: Line = other_999;
    let _e408: Line = other_999;
    let _e411: Line = other_999;
    let _e414: Line = other_999;
    return Motor((((((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g1_.x, _e11.g1_.x, _e14.g1_.z, _e17.g1_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e30.g0_.z) * vec4<f32>(_e34.g1_.y, _e37.g1_.z, _e40.g1_.y, _e43.g1_.x)) * vec4<f32>(-(1.0), -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e57.g0_.w) * vec4<f32>(_e61.g1_.z, _e64.g1_.y, _e67.g1_.x, _e70.g1_.z)) * vec4<f32>(-(1.0), 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e84.g1_.x) * vec4<f32>(_e88.g0_.x, _e91.g0_.x, _e94.g0_.y, _e97.g0_.z)) * vec4<f32>(0.0, -(1.0), -(1.0), -(1.0)))) + ((vec4<f32>(_e112.g1_.y) * vec4<f32>(_e116.g0_.x, _e119.g0_.x, _e122.g0_.z, _e125.g0_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e139.g1_.z) * vec4<f32>(_e143.g0_.y, _e146.g0_.z, _e149.g0_.y, _e152.g0_.x)) * vec4<f32>(-(1.0), -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e166.g1_.w) * vec4<f32>(_e170.g0_.z, _e173.g0_.y, _e176.g0_.x, _e179.g0_.z)) * vec4<f32>(-(1.0), 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e193.g0_.x) * vec4<f32>(_e197.g1_.x, _e200.g1_.x, _e203.g1_.y, _e206.g1_.z)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))), (((((((((vec4<f32>(_e218.g0_.y) * vec4<f32>(_e222.g0_.x, _e225.g0_.x, _e228.g0_.z, _e231.g0_.y)) * vec4<f32>(1.0, 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e243.g0_.z) * vec4<f32>(_e247.g0_.y, _e250.g0_.z, _e253.g0_.y, _e256.g0_.x)) * vec4<f32>(1.0, -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e269.g0_.w) * vec4<f32>(_e273.g0_.z, _e276.g0_.y, _e279.g0_.x, _e282.g0_.z)) * vec4<f32>(1.0, 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e295.g1_.x) * vec4<f32>(_e299.g1_.x, _e302.g1_.x, _e305.g1_.y, _e308.g1_.z)) * vec4<f32>(0.0, -(1.0), -(1.0), -(1.0)))) + ((vec4<f32>(_e323.g1_.y) * vec4<f32>(_e327.g1_.x, _e330.g1_.x, _e333.g1_.z, _e336.g1_.y)) * vec4<f32>(1.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e349.g1_.z) * vec4<f32>(_e353.g1_.y, _e356.g1_.z, _e359.g1_.y, _e362.g1_.x)) * vec4<f32>(1.0, -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e375.g1_.w) * vec4<f32>(_e379.g1_.z, _e382.g1_.y, _e385.g1_.x, _e388.g1_.z)) * vec4<f32>(1.0, 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e401.g0_.x) * vec4<f32>(_e405.g0_.x, _e408.g0_.x, _e411.g0_.y, _e414.g0_.z)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn motor_line_scalar_product(self_1180: Motor, other_1000: Line) -> Scalar {
    var self_1181: Motor;
    var other_1001: Line;

    self_1181 = self_1180;
    other_1001 = other_1000;
    let _e5: Motor = self_1181;
    let _e8: Line = other_1001;
    let _e13: Motor = self_1181;
    let _e16: Line = other_1001;
    let _e21: Motor = self_1181;
    let _e24: Line = other_1001;
    let _e29: Motor = self_1181;
    let _e32: Line = other_1001;
    let _e37: Motor = self_1181;
    let _e40: Line = other_1001;
    let _e45: Motor = self_1181;
    let _e48: Line = other_1001;
    return Scalar(((((((0.0 - (_e5.g0_.y * _e8.g1_.x)) - (_e13.g0_.z * _e16.g1_.y)) - (_e21.g0_.w * _e24.g1_.z)) - (_e29.g1_.y * _e32.g0_.x)) - (_e37.g1_.z * _e40.g0_.y)) - (_e45.g1_.w * _e48.g0_.z)));
}

fn motor_translator_into(self_1182: Motor) -> Translator {
    var self_1183: Motor;

    self_1183 = self_1182;
    let _e2: Motor = self_1183;
    let _e5: Motor = self_1183;
    let _e8: Motor = self_1183;
    let _e11: Motor = self_1183;
    return Translator(vec4<f32>(_e2.g0_.x, _e5.g1_.y, _e8.g1_.z, _e11.g1_.w));
}

fn motor_translator_add(self_1184: Motor, other_1002: Translator) -> Motor {
    var self_1185: Motor;
    var other_1003: Translator;

    self_1185 = self_1184;
    other_1003 = other_1002;
    let _e4: Motor = self_1185;
    let _e6: Translator = other_1003;
    let _e17: Motor = self_1185;
    let _e19: Translator = other_1003;
    return Motor((_e4.g0_ + (vec4<f32>(_e6.g0_.x) * vec4<f32>(1.0, 0.0, 0.0, 0.0))), (_e17.g1_ + (_e19.g0_ * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn motor_translator_sub(self_1186: Motor, other_1004: Translator) -> Motor {
    var self_1187: Motor;
    var other_1005: Translator;

    self_1187 = self_1186;
    other_1005 = other_1004;
    let _e4: Motor = self_1187;
    let _e6: Translator = other_1005;
    let _e17: Motor = self_1187;
    let _e19: Translator = other_1005;
    return Motor((_e4.g0_ - (vec4<f32>(_e6.g0_.x) * vec4<f32>(1.0, 0.0, 0.0, 0.0))), (_e17.g1_ - (_e19.g0_ * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn motor_translator_geometric_product(self_1188: Motor, other_1006: Translator) -> Motor {
    var self_1189: Motor;
    var other_1007: Translator;

    self_1189 = self_1188;
    other_1007 = other_1006;
    let _e4: Motor = self_1189;
    let _e8: Translator = other_1007;
    let _e21: Motor = self_1189;
    let _e25: Translator = other_1007;
    let _e38: Motor = self_1189;
    let _e42: Translator = other_1007;
    let _e55: Motor = self_1189;
    let _e59: Translator = other_1007;
    let _e72: Motor = self_1189;
    let _e74: Translator = other_1007;
    let _e80: Motor = self_1189;
    let _e84: Translator = other_1007;
    let _e95: Motor = self_1189;
    let _e99: Translator = other_1007;
    let _e111: Motor = self_1189;
    let _e115: Translator = other_1007;
    let _e127: Motor = self_1189;
    let _e131: Translator = other_1007;
    let _e143: Motor = self_1189;
    let _e147: Translator = other_1007;
    let _e159: Motor = self_1189;
    let _e163: Translator = other_1007;
    let _e175: Motor = self_1189;
    let _e178: Motor = self_1189;
    let _e181: Motor = self_1189;
    let _e184: Motor = self_1189;
    let _e188: Translator = other_1007;
    return Motor(((((((vec4<f32>(_e4.g1_.x) * _e8.g0_.yyzw) * vec4<f32>(0.0, -(1.0), -(1.0), -(1.0))) + ((vec4<f32>(_e21.g1_.y) * _e25.g0_.yywz) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e38.g1_.z) * _e42.g0_.zwzy) * vec4<f32>(-(1.0), -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e55.g1_.w) * _e59.g0_.wzyw) * vec4<f32>(-(1.0), 1.0, -(1.0), 0.0))) + (_e72.g0_ * vec4<f32>(_e74.g0_.x))), ((((((((vec4<f32>(_e80.g0_.y) * _e84.g0_.yywz) * vec4<f32>(1.0, 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e95.g0_.z) * _e99.g0_.zwzy) * vec4<f32>(1.0, -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e111.g0_.w) * _e115.g0_.wzyw) * vec4<f32>(1.0, 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e127.g1_.y) * vec4<f32>(_e131.g0_.x)) * vec4<f32>(0.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e143.g1_.z) * vec4<f32>(_e147.g0_.x)) * vec4<f32>(0.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e159.g1_.w) * vec4<f32>(_e163.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + (vec4<f32>(_e175.g1_.x, _e178.g0_.x, _e181.g0_.x, _e184.g0_.x) * _e188.g0_)));
}

fn motor_translator_regressive_product(self_1190: Motor, other_1008: Translator) -> Translator {
    var self_1191: Motor;
    var other_1009: Translator;

    self_1191 = self_1190;
    other_1009 = other_1008;
    let _e4: Motor = self_1191;
    let _e8: Translator = other_1009;
    let _e19: Motor = self_1191;
    let _e23: Translator = other_1009;
    let _e35: Motor = self_1191;
    let _e39: Translator = other_1009;
    let _e43: Motor = self_1191;
    let _e46: Translator = other_1009;
    return Translator((((((vec4<f32>(_e4.g0_.z) * vec4<f32>(_e8.g0_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) + ((vec4<f32>(_e19.g0_.w) * vec4<f32>(_e23.g0_.w)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + (vec4<f32>(_e35.g1_.x) * _e39.g0_)) + ((_e43.g0_.yxxx * _e46.g0_.yxxx) * vec4<f32>(1.0, 0.0, 0.0, 0.0))));
}

fn motor_translator_outer_product(self_1192: Motor, other_1010: Translator) -> Motor {
    var self_1193: Motor;
    var other_1011: Translator;

    self_1193 = self_1192;
    other_1011 = other_1010;
    let _e4: Motor = self_1193;
    let _e6: Translator = other_1011;
    let _e11: Motor = self_1193;
    let _e15: Translator = other_1011;
    let _e26: Motor = self_1193;
    let _e30: Translator = other_1011;
    let _e42: Motor = self_1193;
    let _e46: Translator = other_1011;
    let _e58: Motor = self_1193;
    let _e62: Translator = other_1011;
    let _e74: Motor = self_1193;
    let _e78: Translator = other_1011;
    let _e90: Motor = self_1193;
    let _e94: Translator = other_1011;
    let _e106: Motor = self_1193;
    let _e109: Translator = other_1011;
    return Motor((_e4.g0_ * vec4<f32>(_e6.g0_.x)), ((((((((vec4<f32>(_e11.g0_.z) * vec4<f32>(_e15.g0_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) + ((vec4<f32>(_e26.g0_.w) * vec4<f32>(_e30.g0_.w)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e42.g1_.x) * vec4<f32>(_e46.g0_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e58.g1_.y) * vec4<f32>(_e62.g0_.x)) * vec4<f32>(0.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e74.g1_.z) * vec4<f32>(_e78.g0_.x)) * vec4<f32>(0.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e90.g1_.w) * vec4<f32>(_e94.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + (_e106.g0_.yxxx * _e109.g0_.yyzw)));
}

fn motor_translator_inner_product(self_1194: Motor, other_1012: Translator) -> Motor {
    var self_1195: Motor;
    var other_1013: Translator;

    self_1195 = self_1194;
    other_1013 = other_1012;
    let _e4: Motor = self_1195;
    let _e8: Translator = other_1013;
    let _e21: Motor = self_1195;
    let _e25: Translator = other_1013;
    let _e38: Motor = self_1195;
    let _e42: Translator = other_1013;
    let _e55: Motor = self_1195;
    let _e59: Translator = other_1013;
    let _e72: Motor = self_1195;
    let _e74: Translator = other_1013;
    let _e80: Motor = self_1195;
    let _e84: Translator = other_1013;
    let _e95: Motor = self_1195;
    let _e99: Translator = other_1013;
    let _e111: Motor = self_1195;
    let _e115: Translator = other_1013;
    let _e127: Motor = self_1195;
    let _e130: Motor = self_1195;
    let _e133: Motor = self_1195;
    let _e136: Motor = self_1195;
    let _e140: Translator = other_1013;
    return Motor(((((((vec4<f32>(_e4.g1_.x) * _e8.g0_.yyzw) * vec4<f32>(0.0, -(1.0), -(1.0), -(1.0))) + ((vec4<f32>(_e21.g1_.y) * vec4<f32>(_e25.g0_.y)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((vec4<f32>(_e38.g1_.z) * vec4<f32>(_e42.g0_.z)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((vec4<f32>(_e55.g1_.w) * vec4<f32>(_e59.g0_.w)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + (_e72.g0_ * vec4<f32>(_e74.g0_.x))), (((((vec4<f32>(_e80.g1_.y) * vec4<f32>(_e84.g0_.x)) * vec4<f32>(0.0, 1.0, 0.0, 0.0)) + ((vec4<f32>(_e95.g1_.z) * vec4<f32>(_e99.g0_.x)) * vec4<f32>(0.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e111.g1_.w) * vec4<f32>(_e115.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + (vec4<f32>(_e127.g1_.x, _e130.g0_.x, _e133.g0_.x, _e136.g0_.x) * _e140.g0_)));
}

fn motor_translator_left_contraction(self_1196: Motor, other_1014: Translator) -> Translator {
    var self_1197: Motor;
    var other_1015: Translator;

    self_1197 = self_1196;
    other_1015 = other_1014;
    let _e4: Motor = self_1197;
    let _e8: Translator = other_1015;
    let _e11: Motor = self_1197;
    let _e15: Translator = other_1015;
    let _e28: Motor = self_1197;
    let _e32: Translator = other_1015;
    let _e45: Motor = self_1197;
    let _e48: Translator = other_1015;
    return Translator(((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g1_.z) * vec4<f32>(_e15.g0_.z)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((vec4<f32>(_e28.g1_.w) * vec4<f32>(_e32.g0_.w)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((_e45.g1_.yxxx * _e48.g0_.yxxx) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))));
}

fn motor_translator_right_contraction(self_1198: Motor, other_1016: Translator) -> Motor {
    var self_1199: Motor;
    var other_1017: Translator;

    self_1199 = self_1198;
    other_1017 = other_1016;
    let _e4: Motor = self_1199;
    let _e8: Translator = other_1017;
    let _e21: Motor = self_1199;
    let _e25: Translator = other_1017;
    let _e38: Motor = self_1199;
    let _e42: Translator = other_1017;
    let _e55: Motor = self_1199;
    let _e59: Translator = other_1017;
    let _e72: Motor = self_1199;
    let _e74: Translator = other_1017;
    let _e80: Motor = self_1199;
    let _e82: Translator = other_1017;
    return Motor(((((((vec4<f32>(_e4.g1_.x) * _e8.g0_.yyzw) * vec4<f32>(0.0, -(1.0), -(1.0), -(1.0))) + ((vec4<f32>(_e21.g1_.y) * vec4<f32>(_e25.g0_.y)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((vec4<f32>(_e38.g1_.z) * vec4<f32>(_e42.g0_.z)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((vec4<f32>(_e55.g1_.w) * vec4<f32>(_e59.g0_.w)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + (_e72.g0_ * vec4<f32>(_e74.g0_.x))), (_e80.g1_ * vec4<f32>(_e82.g0_.x)));
}

fn motor_translator_scalar_product(self_1200: Motor, other_1018: Translator) -> Scalar {
    var self_1201: Motor;
    var other_1019: Translator;

    self_1201 = self_1200;
    other_1019 = other_1018;
    let _e4: Motor = self_1201;
    let _e7: Translator = other_1019;
    let _e11: Motor = self_1201;
    let _e14: Translator = other_1019;
    let _e19: Motor = self_1201;
    let _e22: Translator = other_1019;
    let _e27: Motor = self_1201;
    let _e30: Translator = other_1019;
    return Scalar(((((_e4.g0_.x * _e7.g0_.x) - (_e11.g1_.y * _e14.g0_.y)) - (_e19.g1_.z * _e22.g0_.z)) - (_e27.g1_.w * _e30.g0_.w)));
}

fn motor_motor_add(self_1202: Motor, other_1020: Motor) -> Motor {
    var self_1203: Motor;
    var other_1021: Motor;

    self_1203 = self_1202;
    other_1021 = other_1020;
    let _e4: Motor = self_1203;
    let _e6: Motor = other_1021;
    let _e9: Motor = self_1203;
    let _e11: Motor = other_1021;
    return Motor((_e4.g0_ + _e6.g0_), (_e9.g1_ + _e11.g1_));
}

fn motor_motor_sub(self_1204: Motor, other_1022: Motor) -> Motor {
    var self_1205: Motor;
    var other_1023: Motor;

    self_1205 = self_1204;
    other_1023 = other_1022;
    let _e4: Motor = self_1205;
    let _e6: Motor = other_1023;
    let _e9: Motor = self_1205;
    let _e11: Motor = other_1023;
    return Motor((_e4.g0_ - _e6.g0_), (_e9.g1_ - _e11.g1_));
}

fn motor_motor_mul(self_1206: Motor, other_1024: Motor) -> Motor {
    var self_1207: Motor;
    var other_1025: Motor;

    self_1207 = self_1206;
    other_1025 = other_1024;
    let _e4: Motor = self_1207;
    let _e6: Motor = other_1025;
    let _e9: Motor = self_1207;
    let _e11: Motor = other_1025;
    return Motor((_e4.g0_ * _e6.g0_), (_e9.g1_ * _e11.g1_));
}

fn motor_motor_div(self_1208: Motor, other_1026: Motor) -> Motor {
    var self_1209: Motor;
    var other_1027: Motor;

    self_1209 = self_1208;
    other_1027 = other_1026;
    let _e4: Motor = self_1209;
    let _e7: Motor = self_1209;
    let _e10: Motor = self_1209;
    let _e13: Motor = self_1209;
    let _e23: Motor = other_1027;
    let _e26: Motor = other_1027;
    let _e29: Motor = other_1027;
    let _e32: Motor = other_1027;
    let _e43: Motor = self_1209;
    let _e46: Motor = self_1209;
    let _e49: Motor = self_1209;
    let _e52: Motor = self_1209;
    let _e62: Motor = other_1027;
    let _e65: Motor = other_1027;
    let _e68: Motor = other_1027;
    let _e71: Motor = other_1027;
    return Motor((((vec4<f32>(_e4.g0_.x, _e7.g0_.y, _e10.g0_.z, _e13.g0_.w) * vec4<f32>(1.0, 1.0, 1.0, 1.0)) / vec4<f32>(_e23.g0_.x, _e26.g0_.y, _e29.g0_.z, _e32.g0_.w)) * vec4<f32>(1.0, 1.0, 1.0, 1.0)), (((vec4<f32>(_e43.g1_.x, _e46.g1_.y, _e49.g1_.z, _e52.g1_.w) * vec4<f32>(1.0, 1.0, 1.0, 1.0)) / vec4<f32>(_e62.g1_.x, _e65.g1_.y, _e68.g1_.z, _e71.g1_.w)) * vec4<f32>(1.0, 1.0, 1.0, 1.0)));
}

fn motor_motor_geometric_product(self_1210: Motor, other_1028: Motor) -> Motor {
    var self_1211: Motor;
    var other_1029: Motor;

    self_1211 = self_1210;
    other_1029 = other_1028;
    let _e4: Motor = self_1211;
    let _e8: Motor = other_1029;
    let _e11: Motor = self_1211;
    let _e15: Motor = other_1029;
    let _e28: Motor = self_1211;
    let _e32: Motor = other_1029;
    let _e45: Motor = self_1211;
    let _e49: Motor = other_1029;
    let _e62: Motor = self_1211;
    let _e66: Motor = other_1029;
    let _e79: Motor = self_1211;
    let _e83: Motor = other_1029;
    let _e97: Motor = self_1211;
    let _e101: Motor = other_1029;
    let _e115: Motor = self_1211;
    let _e119: Motor = other_1029;
    let _e133: Motor = self_1211;
    let _e137: Motor = other_1029;
    let _e140: Motor = self_1211;
    let _e144: Motor = other_1029;
    let _e157: Motor = self_1211;
    let _e161: Motor = other_1029;
    let _e174: Motor = self_1211;
    let _e178: Motor = other_1029;
    let _e191: Motor = self_1211;
    let _e195: Motor = other_1029;
    let _e208: Motor = self_1211;
    let _e212: Motor = other_1029;
    let _e224: Motor = self_1211;
    let _e228: Motor = other_1029;
    let _e240: Motor = self_1211;
    let _e244: Motor = other_1029;
    return Motor(((((((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.y) * _e15.g0_.yxwz) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e28.g0_.z) * _e32.g0_.zwxy) * vec4<f32>(-(1.0), -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e45.g0_.w) * _e49.g0_.wzyx) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))) + ((vec4<f32>(_e62.g1_.x) * _e66.g1_) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0)))) + ((vec4<f32>(_e79.g1_.y) * _e83.g1_.yxwz) * vec4<f32>(-(1.0), -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e97.g1_.z) * _e101.g1_.zwxy) * vec4<f32>(-(1.0), -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e115.g1_.w) * _e119.g1_.wzyx) * vec4<f32>(-(1.0), 1.0, -(1.0), -(1.0)))), ((((((((vec4<f32>(_e133.g0_.x) * _e137.g1_) + ((vec4<f32>(_e140.g0_.y) * _e144.g1_.yxwz) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e157.g0_.z) * _e161.g1_.zwxy) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e174.g0_.w) * _e178.g1_.wzyx) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e191.g1_.x) * _e195.g0_) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0)))) + ((vec4<f32>(_e208.g1_.y) * _e212.g0_.yxwz) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e224.g1_.z) * _e228.g0_.zwxy) * vec4<f32>(1.0, -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e240.g1_.w) * _e244.g0_.wzyx) * vec4<f32>(1.0, 1.0, -(1.0), 1.0))));
}

fn motor_motor_regressive_product(self_1212: Motor, other_1030: Motor) -> Motor {
    var self_1213: Motor;
    var other_1031: Motor;

    self_1213 = self_1212;
    other_1031 = other_1030;
    let _e4: Motor = self_1213;
    let _e8: Motor = other_1031;
    let _e18: Motor = self_1213;
    let _e22: Motor = other_1031;
    let _e33: Motor = self_1213;
    let _e37: Motor = other_1031;
    let _e48: Motor = self_1213;
    let _e52: Motor = other_1031;
    let _e56: Motor = self_1213;
    let _e60: Motor = other_1031;
    let _e72: Motor = self_1213;
    let _e76: Motor = other_1031;
    let _e88: Motor = self_1213;
    let _e92: Motor = other_1031;
    let _e104: Motor = self_1213;
    let _e108: Motor = other_1031;
    let _e120: Motor = self_1213;
    let _e124: Motor = other_1031;
    let _e127: Motor = self_1213;
    let _e129: Motor = other_1031;
    return Motor((((((((((vec4<f32>(_e4.g0_.y) * _e8.g1_.yxyy) * vec4<f32>(1.0, 1.0, 0.0, 0.0)) + ((vec4<f32>(_e18.g0_.z) * _e22.g1_.zzxz) * vec4<f32>(1.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e33.g0_.w) * _e37.g1_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, 1.0))) + (vec4<f32>(_e48.g1_.x) * _e52.g0_)) + ((vec4<f32>(_e56.g1_.y) * vec4<f32>(_e60.g0_.y)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e72.g1_.z) * vec4<f32>(_e76.g0_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e88.g1_.w) * vec4<f32>(_e92.g0_.w)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e104.g0_.x) * vec4<f32>(_e108.g1_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))), ((vec4<f32>(_e120.g1_.x) * _e124.g1_) + ((_e127.g1_ * vec4<f32>(_e129.g1_.x)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn motor_motor_outer_product(self_1214: Motor, other_1032: Motor) -> Motor {
    var self_1215: Motor;
    var other_1033: Motor;

    self_1215 = self_1214;
    other_1033 = other_1032;
    let _e4: Motor = self_1215;
    let _e8: Motor = other_1033;
    let _e11: Motor = self_1215;
    let _e13: Motor = other_1033;
    let _e25: Motor = self_1215;
    let _e29: Motor = other_1033;
    let _e32: Motor = self_1215;
    let _e36: Motor = other_1033;
    let _e48: Motor = self_1215;
    let _e52: Motor = other_1033;
    let _e64: Motor = self_1215;
    let _e68: Motor = other_1033;
    let _e80: Motor = self_1215;
    let _e84: Motor = other_1033;
    let _e95: Motor = self_1215;
    let _e99: Motor = other_1033;
    let _e110: Motor = self_1215;
    let _e114: Motor = other_1033;
    let _e125: Motor = self_1215;
    let _e128: Motor = other_1033;
    return Motor(((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((_e11.g0_ * vec4<f32>(_e13.g0_.x)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))), ((((((((vec4<f32>(_e25.g0_.x) * _e29.g1_) + ((vec4<f32>(_e32.g0_.z) * vec4<f32>(_e36.g1_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e48.g0_.w) * vec4<f32>(_e52.g1_.w)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e64.g1_.x) * vec4<f32>(_e68.g0_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e80.g1_.y) * _e84.g0_.yxyy) * vec4<f32>(1.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e95.g1_.z) * _e99.g0_.zzxz) * vec4<f32>(1.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e110.g1_.w) * _e114.g0_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, 1.0))) + ((_e125.g0_.yxxx * _e128.g1_.yxxx) * vec4<f32>(1.0, 0.0, 0.0, 0.0))));
}

fn motor_motor_inner_product(self_1216: Motor, other_1034: Motor) -> Motor {
    var self_1217: Motor;
    var other_1035: Motor;

    self_1217 = self_1216;
    other_1035 = other_1034;
    let _e4: Motor = self_1217;
    let _e8: Motor = other_1035;
    let _e11: Motor = self_1217;
    let _e15: Motor = other_1035;
    let _e27: Motor = self_1217;
    let _e31: Motor = other_1035;
    let _e43: Motor = self_1217;
    let _e47: Motor = other_1035;
    let _e60: Motor = self_1217;
    let _e64: Motor = other_1035;
    let _e77: Motor = self_1217;
    let _e81: Motor = other_1035;
    let _e94: Motor = self_1217;
    let _e98: Motor = other_1035;
    let _e111: Motor = self_1217;
    let _e114: Motor = other_1035;
    let _e126: Motor = self_1217;
    let _e130: Motor = other_1035;
    let _e133: Motor = self_1217;
    let _e137: Motor = other_1035;
    let _e150: Motor = self_1217;
    let _e154: Motor = other_1035;
    let _e166: Motor = self_1217;
    let _e170: Motor = other_1035;
    let _e182: Motor = self_1217;
    let _e186: Motor = other_1035;
    let _e198: Motor = self_1217;
    let _e200: Motor = other_1035;
    return Motor(((((((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.z) * _e15.g0_.zzxz) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0))) + ((vec4<f32>(_e27.g0_.w) * _e31.g0_.wwwx) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))) + ((vec4<f32>(_e43.g1_.x) * _e47.g1_) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0)))) + ((vec4<f32>(_e60.g1_.y) * _e64.g1_.yxyy) * vec4<f32>(-(1.0), -(1.0), 0.0, 0.0))) + ((vec4<f32>(_e77.g1_.z) * _e81.g1_.zzxz) * vec4<f32>(-(1.0), 0.0, -(1.0), 0.0))) + ((vec4<f32>(_e94.g1_.w) * _e98.g1_.wwwx) * vec4<f32>(-(1.0), 0.0, 0.0, -(1.0)))) + ((_e111.g0_.yyxx * _e114.g0_.yxxx) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0))), ((((((vec4<f32>(_e126.g0_.x) * _e130.g1_) + ((vec4<f32>(_e133.g1_.x) * _e137.g0_) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0)))) + ((vec4<f32>(_e150.g1_.y) * vec4<f32>(_e154.g0_.x)) * vec4<f32>(0.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e166.g1_.z) * vec4<f32>(_e170.g0_.x)) * vec4<f32>(0.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e182.g1_.w) * vec4<f32>(_e186.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((_e198.g0_ * vec4<f32>(_e200.g1_.x)) * vec4<f32>(0.0, -(1.0), -(1.0), -(1.0)))));
}

fn motor_motor_left_contraction(self_1218: Motor, other_1036: Motor) -> Motor {
    var self_1219: Motor;
    var other_1037: Motor;

    self_1219 = self_1218;
    other_1037 = other_1036;
    let _e4: Motor = self_1219;
    let _e8: Motor = other_1037;
    let _e11: Motor = self_1219;
    let _e15: Motor = other_1037;
    let _e28: Motor = self_1219;
    let _e32: Motor = other_1037;
    let _e45: Motor = self_1219;
    let _e49: Motor = other_1037;
    let _e61: Motor = self_1219;
    let _e65: Motor = other_1037;
    let _e78: Motor = self_1219;
    let _e82: Motor = other_1037;
    let _e95: Motor = self_1219;
    let _e99: Motor = other_1037;
    let _e112: Motor = self_1219;
    let _e115: Motor = other_1037;
    let _e127: Motor = self_1219;
    let _e131: Motor = other_1037;
    let _e134: Motor = self_1219;
    let _e136: Motor = other_1037;
    return Motor(((((((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.z) * vec4<f32>(_e15.g0_.z)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((vec4<f32>(_e28.g0_.w) * vec4<f32>(_e32.g0_.w)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((vec4<f32>(_e45.g1_.x) * vec4<f32>(_e49.g1_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e61.g1_.y) * _e65.g1_.yxyy) * vec4<f32>(-(1.0), -(1.0), 0.0, 0.0))) + ((vec4<f32>(_e78.g1_.z) * _e82.g1_.zzxz) * vec4<f32>(-(1.0), 0.0, -(1.0), 0.0))) + ((vec4<f32>(_e95.g1_.w) * _e99.g1_.wwwx) * vec4<f32>(-(1.0), 0.0, 0.0, -(1.0)))) + ((_e112.g0_.yxxx * _e115.g0_.yxxx) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))), ((vec4<f32>(_e127.g0_.x) * _e131.g1_) + ((_e134.g0_ * vec4<f32>(_e136.g1_.x)) * vec4<f32>(0.0, -(1.0), -(1.0), -(1.0)))));
}

fn motor_motor_right_contraction(self_1220: Motor, other_1038: Motor) -> Motor {
    var self_1221: Motor;
    var other_1039: Motor;

    self_1221 = self_1220;
    other_1039 = other_1038;
    let _e4: Motor = self_1221;
    let _e8: Motor = other_1039;
    let _e19: Motor = self_1221;
    let _e23: Motor = other_1039;
    let _e35: Motor = self_1221;
    let _e39: Motor = other_1039;
    let _e51: Motor = self_1221;
    let _e55: Motor = other_1039;
    let _e68: Motor = self_1221;
    let _e72: Motor = other_1039;
    let _e85: Motor = self_1221;
    let _e89: Motor = other_1039;
    let _e102: Motor = self_1221;
    let _e106: Motor = other_1039;
    let _e119: Motor = self_1221;
    let _e123: Motor = other_1039;
    let _e135: Motor = self_1221;
    let _e139: Motor = other_1039;
    let _e151: Motor = self_1221;
    let _e153: Motor = other_1039;
    return Motor((((((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.yxyy) * vec4<f32>(-(1.0), 1.0, 0.0, 0.0)) + ((vec4<f32>(_e19.g0_.z) * _e23.g0_.zzxz) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0))) + ((vec4<f32>(_e35.g0_.w) * _e39.g0_.wwwx) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))) + ((vec4<f32>(_e51.g1_.x) * _e55.g1_) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0)))) + ((vec4<f32>(_e68.g1_.y) * vec4<f32>(_e72.g1_.y)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((vec4<f32>(_e85.g1_.z) * vec4<f32>(_e89.g1_.z)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((vec4<f32>(_e102.g1_.w) * vec4<f32>(_e106.g1_.w)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((vec4<f32>(_e119.g0_.x) * vec4<f32>(_e123.g0_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))), (((vec4<f32>(_e135.g1_.x) * _e139.g0_) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))) + ((_e151.g1_ * vec4<f32>(_e153.g0_.x)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn motor_motor_scalar_product(self_1222: Motor, other_1040: Motor) -> Scalar {
    var self_1223: Motor;
    var other_1041: Motor;

    self_1223 = self_1222;
    other_1041 = other_1040;
    let _e4: Motor = self_1223;
    let _e7: Motor = other_1041;
    let _e11: Motor = self_1223;
    let _e14: Motor = other_1041;
    let _e19: Motor = self_1223;
    let _e22: Motor = other_1041;
    let _e27: Motor = self_1223;
    let _e30: Motor = other_1041;
    let _e35: Motor = self_1223;
    let _e38: Motor = other_1041;
    let _e43: Motor = self_1223;
    let _e46: Motor = other_1041;
    let _e51: Motor = self_1223;
    let _e54: Motor = other_1041;
    let _e59: Motor = self_1223;
    let _e62: Motor = other_1041;
    return Scalar(((((((((_e4.g0_.x * _e7.g0_.x) - (_e11.g0_.y * _e14.g0_.y)) - (_e19.g0_.z * _e22.g0_.z)) - (_e27.g0_.w * _e30.g0_.w)) + (_e35.g1_.x * _e38.g1_.x)) - (_e43.g1_.y * _e46.g1_.y)) - (_e51.g1_.z * _e54.g1_.z)) - (_e59.g1_.w * _e62.g1_.w)));
}

fn motor_point_and_plane_add(self_1224: Motor, other_1042: PointAndPlane) -> MultiVector {
    var self_1225: Motor;
    var other_1043: PointAndPlane;

    self_1225 = self_1224;
    other_1043 = other_1042;
    let _e4: Motor = self_1225;
    let _e6: PointAndPlane = other_1043;
    let _e9: PointAndPlane = other_1043;
    let _e12: PointAndPlane = other_1043;
    let _e15: PointAndPlane = other_1043;
    let _e19: PointAndPlane = other_1043;
    let _e22: PointAndPlane = other_1043;
    let _e25: PointAndPlane = other_1043;
    let _e28: PointAndPlane = other_1043;
    let _e32: Motor = self_1225;
    return MultiVector(_e4.g0_, vec4<f32>(_e6.g1_.x, _e9.g0_.y, _e12.g0_.z, _e15.g0_.w), vec4<f32>(_e19.g0_.x, _e22.g1_.y, _e25.g1_.z, _e28.g1_.w), _e32.g1_);
}

fn motor_point_and_plane_sub(self_1226: Motor, other_1044: PointAndPlane) -> MultiVector {
    var self_1227: Motor;
    var other_1045: PointAndPlane;

    self_1227 = self_1226;
    other_1045 = other_1044;
    let _e4: Motor = self_1227;
    let _e8: PointAndPlane = other_1045;
    let _e11: PointAndPlane = other_1045;
    let _e14: PointAndPlane = other_1045;
    let _e17: PointAndPlane = other_1045;
    let _e24: PointAndPlane = other_1045;
    let _e27: PointAndPlane = other_1045;
    let _e30: PointAndPlane = other_1045;
    let _e33: PointAndPlane = other_1045;
    let _e38: Motor = self_1227;
    return MultiVector(_e4.g0_, (vec4<f32>(0.0) - vec4<f32>(_e8.g1_.x, _e11.g0_.y, _e14.g0_.z, _e17.g0_.w)), (vec4<f32>(0.0) - vec4<f32>(_e24.g0_.x, _e27.g1_.y, _e30.g1_.z, _e33.g1_.w)), _e38.g1_);
}

fn motor_point_and_plane_geometric_product(self_1228: Motor, other_1046: PointAndPlane) -> PointAndPlane {
    var self_1229: Motor;
    var other_1047: PointAndPlane;

    self_1229 = self_1228;
    other_1047 = other_1046;
    let _e4: Motor = self_1229;
    let _e8: PointAndPlane = other_1047;
    let _e11: Motor = self_1229;
    let _e15: PointAndPlane = other_1047;
    let _e18: PointAndPlane = other_1047;
    let _e21: PointAndPlane = other_1047;
    let _e24: PointAndPlane = other_1047;
    let _e38: Motor = self_1229;
    let _e42: PointAndPlane = other_1047;
    let _e45: PointAndPlane = other_1047;
    let _e48: PointAndPlane = other_1047;
    let _e51: PointAndPlane = other_1047;
    let _e65: Motor = self_1229;
    let _e69: PointAndPlane = other_1047;
    let _e72: PointAndPlane = other_1047;
    let _e75: PointAndPlane = other_1047;
    let _e78: PointAndPlane = other_1047;
    let _e92: Motor = self_1229;
    let _e96: PointAndPlane = other_1047;
    let _e100: Motor = self_1229;
    let _e104: PointAndPlane = other_1047;
    let _e107: PointAndPlane = other_1047;
    let _e110: PointAndPlane = other_1047;
    let _e113: PointAndPlane = other_1047;
    let _e127: Motor = self_1229;
    let _e131: PointAndPlane = other_1047;
    let _e134: PointAndPlane = other_1047;
    let _e137: PointAndPlane = other_1047;
    let _e140: PointAndPlane = other_1047;
    let _e154: Motor = self_1229;
    let _e158: PointAndPlane = other_1047;
    let _e161: PointAndPlane = other_1047;
    let _e164: PointAndPlane = other_1047;
    let _e167: PointAndPlane = other_1047;
    let _e181: Motor = self_1229;
    let _e185: PointAndPlane = other_1047;
    let _e188: Motor = self_1229;
    let _e192: PointAndPlane = other_1047;
    let _e195: PointAndPlane = other_1047;
    let _e198: PointAndPlane = other_1047;
    let _e201: PointAndPlane = other_1047;
    let _e215: Motor = self_1229;
    let _e219: PointAndPlane = other_1047;
    let _e222: PointAndPlane = other_1047;
    let _e225: PointAndPlane = other_1047;
    let _e228: PointAndPlane = other_1047;
    let _e242: Motor = self_1229;
    let _e246: PointAndPlane = other_1047;
    let _e249: PointAndPlane = other_1047;
    let _e252: PointAndPlane = other_1047;
    let _e255: PointAndPlane = other_1047;
    let _e269: Motor = self_1229;
    let _e273: PointAndPlane = other_1047;
    let _e277: Motor = self_1229;
    let _e281: PointAndPlane = other_1047;
    let _e284: PointAndPlane = other_1047;
    let _e287: PointAndPlane = other_1047;
    let _e290: PointAndPlane = other_1047;
    let _e304: Motor = self_1229;
    let _e308: PointAndPlane = other_1047;
    let _e311: PointAndPlane = other_1047;
    let _e314: PointAndPlane = other_1047;
    let _e317: PointAndPlane = other_1047;
    let _e331: Motor = self_1229;
    let _e335: PointAndPlane = other_1047;
    let _e338: PointAndPlane = other_1047;
    let _e341: PointAndPlane = other_1047;
    let _e344: PointAndPlane = other_1047;
    return PointAndPlane(((((((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.y) * vec4<f32>(_e15.g1_.y, _e18.g1_.x, _e21.g0_.w, _e24.g0_.z)) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e38.g0_.z) * vec4<f32>(_e42.g1_.z, _e45.g0_.w, _e48.g1_.x, _e51.g0_.y)) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e65.g0_.w) * vec4<f32>(_e69.g1_.w, _e72.g0_.z, _e75.g0_.y, _e78.g1_.x)) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))) - (vec4<f32>(_e92.g1_.x) * _e96.g1_)) + ((vec4<f32>(_e100.g1_.y) * vec4<f32>(_e104.g0_.y, _e107.g0_.x, _e110.g1_.w, _e113.g1_.z)) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e127.g1_.z) * vec4<f32>(_e131.g0_.z, _e134.g1_.w, _e137.g0_.x, _e140.g1_.y)) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e154.g1_.w) * vec4<f32>(_e158.g0_.w, _e161.g1_.z, _e164.g1_.y, _e167.g0_.x)) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))), ((((((((vec4<f32>(_e181.g0_.x) * _e185.g1_) + ((vec4<f32>(_e188.g0_.y) * vec4<f32>(_e192.g0_.y, _e195.g0_.x, _e198.g1_.w, _e201.g1_.z)) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e215.g0_.z) * vec4<f32>(_e219.g0_.z, _e222.g1_.w, _e225.g0_.x, _e228.g1_.y)) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e242.g0_.w) * vec4<f32>(_e246.g0_.w, _e249.g1_.z, _e252.g1_.y, _e255.g0_.x)) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))) - (vec4<f32>(_e269.g1_.x) * _e273.g0_)) + ((vec4<f32>(_e277.g1_.y) * vec4<f32>(_e281.g1_.y, _e284.g1_.x, _e287.g0_.w, _e290.g0_.z)) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e304.g1_.z) * vec4<f32>(_e308.g1_.z, _e311.g0_.w, _e314.g1_.x, _e317.g0_.y)) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e331.g1_.w) * vec4<f32>(_e335.g1_.w, _e338.g0_.z, _e341.g0_.y, _e344.g1_.x)) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))));
}

fn motor_point_and_plane_regressive_product(self_1230: Motor, other_1048: PointAndPlane) -> PointAndPlane {
    var self_1231: Motor;
    var other_1049: PointAndPlane;

    self_1231 = self_1230;
    other_1049 = other_1048;
    let _e4: Motor = self_1231;
    let _e8: PointAndPlane = other_1049;
    let _e11: Motor = self_1231;
    let _e15: PointAndPlane = other_1049;
    let _e26: Motor = self_1231;
    let _e30: PointAndPlane = other_1049;
    let _e42: Motor = self_1231;
    let _e46: PointAndPlane = other_1049;
    let _e50: Motor = self_1231;
    let _e54: PointAndPlane = other_1049;
    let _e66: Motor = self_1231;
    let _e70: PointAndPlane = other_1049;
    let _e82: Motor = self_1231;
    let _e85: Motor = self_1231;
    let _e88: Motor = self_1231;
    let _e91: Motor = self_1231;
    let _e95: PointAndPlane = other_1049;
    return PointAndPlane((vec4<f32>(_e4.g1_.x) * _e8.g0_), (((((((vec4<f32>(_e11.g0_.z) * _e15.g0_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e26.g0_.w) * _e30.g0_.zzyz) * vec4<f32>(0.0, -(1.0), 1.0, 0.0))) + (vec4<f32>(_e42.g1_.x) * _e46.g1_)) + ((vec4<f32>(_e50.g1_.z) * _e54.g0_.zzxz) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0))) + ((vec4<f32>(_e66.g1_.w) * _e70.g0_.wwwx) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))) + ((vec4<f32>(_e82.g1_.y, _e85.g1_.y, _e88.g0_.y, _e91.g0_.y) * _e95.g0_.yxwz) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))));
}

fn motor_point_and_plane_outer_product(self_1232: Motor, other_1050: PointAndPlane) -> PointAndPlane {
    var self_1233: Motor;
    var other_1051: PointAndPlane;

    self_1233 = self_1232;
    other_1051 = other_1050;
    let _e4: Motor = self_1233;
    let _e8: PointAndPlane = other_1051;
    let _e11: Motor = self_1233;
    let _e15: PointAndPlane = other_1051;
    let _e27: Motor = self_1233;
    let _e31: PointAndPlane = other_1051;
    let _e43: Motor = self_1233;
    let _e47: PointAndPlane = other_1051;
    let _e59: Motor = self_1233;
    let _e63: PointAndPlane = other_1051;
    let _e75: Motor = self_1233;
    let _e78: Motor = self_1233;
    let _e81: Motor = self_1233;
    let _e84: Motor = self_1233;
    let _e88: PointAndPlane = other_1051;
    let _e101: Motor = self_1233;
    let _e105: PointAndPlane = other_1051;
    return PointAndPlane(((((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.z) * _e15.g1_.zzxz) * vec4<f32>(1.0, 0.0, -(1.0), 0.0))) + ((vec4<f32>(_e27.g0_.w) * _e31.g1_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e43.g1_.z) * _e47.g1_.wwwy) * vec4<f32>(0.0, -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e59.g1_.w) * _e63.g1_.zzyz) * vec4<f32>(0.0, 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e75.g0_.y, _e78.g0_.y, _e81.g1_.y, _e84.g1_.y) * _e88.g1_.yxwz) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))), (vec4<f32>(_e101.g0_.x) * _e105.g1_));
}

fn motor_point_and_plane_inner_product(self_1234: Motor, other_1052: PointAndPlane) -> PointAndPlane {
    var self_1235: Motor;
    var other_1053: PointAndPlane;

    self_1235 = self_1234;
    other_1053 = other_1052;
    let _e4: Motor = self_1235;
    let _e8: PointAndPlane = other_1053;
    let _e11: Motor = self_1235;
    let _e15: PointAndPlane = other_1053;
    let _e19: Motor = self_1235;
    let _e23: PointAndPlane = other_1053;
    let _e26: Motor = self_1235;
    let _e30: PointAndPlane = other_1053;
    let _e33: PointAndPlane = other_1053;
    let _e36: PointAndPlane = other_1053;
    let _e39: PointAndPlane = other_1053;
    let _e53: Motor = self_1235;
    let _e57: PointAndPlane = other_1053;
    let _e60: PointAndPlane = other_1053;
    let _e63: PointAndPlane = other_1053;
    let _e66: PointAndPlane = other_1053;
    let _e80: Motor = self_1235;
    let _e84: PointAndPlane = other_1053;
    let _e87: PointAndPlane = other_1053;
    let _e90: PointAndPlane = other_1053;
    let _e93: PointAndPlane = other_1053;
    let _e107: Motor = self_1235;
    let _e111: PointAndPlane = other_1053;
    let _e115: Motor = self_1235;
    let _e119: PointAndPlane = other_1053;
    let _e122: PointAndPlane = other_1053;
    let _e125: PointAndPlane = other_1053;
    let _e128: PointAndPlane = other_1053;
    let _e142: Motor = self_1235;
    let _e146: PointAndPlane = other_1053;
    let _e149: PointAndPlane = other_1053;
    let _e152: PointAndPlane = other_1053;
    let _e155: PointAndPlane = other_1053;
    let _e169: Motor = self_1235;
    let _e173: PointAndPlane = other_1053;
    let _e176: PointAndPlane = other_1053;
    let _e179: PointAndPlane = other_1053;
    let _e182: PointAndPlane = other_1053;
    return PointAndPlane(((vec4<f32>(_e4.g0_.x) * _e8.g0_) - (vec4<f32>(_e11.g1_.x) * _e15.g1_)), ((((((((vec4<f32>(_e19.g0_.x) * _e23.g1_) + ((vec4<f32>(_e26.g0_.y) * vec4<f32>(_e30.g0_.y, _e33.g0_.x, _e36.g1_.w, _e39.g1_.z)) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e53.g0_.z) * vec4<f32>(_e57.g0_.z, _e60.g1_.w, _e63.g0_.x, _e66.g1_.y)) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e80.g0_.w) * vec4<f32>(_e84.g0_.w, _e87.g1_.z, _e90.g1_.y, _e93.g0_.x)) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))) - (vec4<f32>(_e107.g1_.x) * _e111.g0_)) + ((vec4<f32>(_e115.g1_.y) * vec4<f32>(_e119.g1_.y, _e122.g1_.x, _e125.g0_.w, _e128.g0_.z)) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e142.g1_.z) * vec4<f32>(_e146.g1_.z, _e149.g0_.w, _e152.g1_.x, _e155.g0_.y)) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e169.g1_.w) * vec4<f32>(_e173.g1_.w, _e176.g0_.z, _e179.g0_.y, _e182.g1_.x)) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))));
}

fn motor_point_and_plane_left_contraction(self_1236: Motor, other_1054: PointAndPlane) -> PointAndPlane {
    var self_1237: Motor;
    var other_1055: PointAndPlane;

    self_1237 = self_1236;
    other_1055 = other_1054;
    let _e4: Motor = self_1237;
    let _e8: PointAndPlane = other_1055;
    let _e11: Motor = self_1237;
    let _e15: PointAndPlane = other_1055;
    let _e18: Motor = self_1237;
    let _e22: PointAndPlane = other_1055;
    let _e34: Motor = self_1237;
    let _e38: PointAndPlane = other_1055;
    let _e50: Motor = self_1237;
    let _e54: PointAndPlane = other_1055;
    let _e66: Motor = self_1237;
    let _e70: PointAndPlane = other_1055;
    let _e82: Motor = self_1237;
    let _e85: Motor = self_1237;
    let _e88: Motor = self_1237;
    let _e91: Motor = self_1237;
    let _e95: PointAndPlane = other_1055;
    return PointAndPlane((vec4<f32>(_e4.g0_.x) * _e8.g0_), ((((((vec4<f32>(_e11.g0_.x) * _e15.g1_) + ((vec4<f32>(_e18.g0_.z) * _e22.g0_.zzxz) * vec4<f32>(1.0, 0.0, -(1.0), 0.0))) + ((vec4<f32>(_e34.g0_.w) * _e38.g0_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e50.g1_.z) * _e54.g0_.wwwy) * vec4<f32>(0.0, -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e66.g1_.w) * _e70.g0_.zzyz) * vec4<f32>(0.0, 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e82.g0_.y, _e85.g0_.y, _e88.g1_.y, _e91.g1_.y) * _e95.g0_.yxwz) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))));
}

fn motor_point_and_plane_right_contraction(self_1238: Motor, other_1056: PointAndPlane) -> PointAndPlane {
    var self_1239: Motor;
    var other_1057: PointAndPlane;

    self_1239 = self_1238;
    other_1057 = other_1056;
    let _e6: Motor = self_1239;
    let _e10: PointAndPlane = other_1057;
    let _e14: Motor = self_1239;
    let _e18: PointAndPlane = other_1057;
    let _e29: Motor = self_1239;
    let _e33: PointAndPlane = other_1057;
    let _e45: Motor = self_1239;
    let _e49: PointAndPlane = other_1057;
    let _e53: Motor = self_1239;
    let _e57: PointAndPlane = other_1057;
    let _e69: Motor = self_1239;
    let _e73: PointAndPlane = other_1057;
    let _e85: Motor = self_1239;
    let _e88: Motor = self_1239;
    let _e91: Motor = self_1239;
    let _e94: Motor = self_1239;
    let _e98: PointAndPlane = other_1057;
    return PointAndPlane((vec4<f32>(0.0) - (vec4<f32>(_e6.g1_.x) * _e10.g1_)), (((((((vec4<f32>(_e14.g0_.z) * _e18.g1_.wwwy) * vec4<f32>(0.0, -(1.0), 0.0, 1.0)) + ((vec4<f32>(_e29.g0_.w) * _e33.g1_.zzyz) * vec4<f32>(0.0, 1.0, -(1.0), 0.0))) - (vec4<f32>(_e45.g1_.x) * _e49.g0_)) + ((vec4<f32>(_e53.g1_.z) * _e57.g1_.zzxz) * vec4<f32>(1.0, 0.0, -(1.0), 0.0))) + ((vec4<f32>(_e69.g1_.w) * _e73.g1_.wwwx) * vec4<f32>(1.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e85.g1_.y, _e88.g1_.y, _e91.g0_.y, _e94.g0_.y) * _e98.g1_.yxwz) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))));
}

fn motor_squared_magnitude(self_1240: Motor) -> Scalar {
    var self_1241: Motor;

    self_1241 = self_1240;
    let _e4: Motor = self_1241;
    let _e5: Motor = motor_reversal(_e4);
    let _e6: Motor = self_1241;
    let _e8: Motor = self_1241;
    let _e9: Motor = motor_reversal(_e8);
    let _e10: Scalar = motor_motor_scalar_product(_e6, _e9);
    return _e10;
}

fn motor_magnitude(self_1242: Motor) -> Scalar {
    var self_1243: Motor;

    self_1243 = self_1242;
    let _e3: Motor = self_1243;
    let _e4: Scalar = motor_squared_magnitude(_e3);
    let _e7: Motor = self_1243;
    let _e8: Scalar = motor_squared_magnitude(_e7);
    return Scalar(sqrt(_e8.g0_));
}

fn motor_scale(self_1244: Motor, other_1058: f32) -> Motor {
    var self_1245: Motor;
    var other_1059: f32;

    self_1245 = self_1244;
    other_1059 = other_1058;
    let _e5: f32 = other_1059;
    let _e7: Motor = self_1245;
    let _e8: f32 = other_1059;
    let _e10: Motor = motor_scalar_geometric_product(_e7, Scalar(_e8));
    return _e10;
}

fn motor_signum(self_1246: Motor) -> Motor {
    var self_1247: Motor;

    self_1247 = self_1246;
    let _e5: Motor = self_1247;
    let _e6: Scalar = motor_magnitude(_e5);
    let _e10: Motor = self_1247;
    let _e13: Motor = self_1247;
    let _e14: Scalar = motor_magnitude(_e13);
    let _e18: Motor = motor_scalar_geometric_product(_e10, Scalar((1.0 / _e14.g0_)));
    return _e18;
}

fn motor_inverse(self_1248: Motor) -> Motor {
    var self_1249: Motor;

    self_1249 = self_1248;
    let _e3: Motor = self_1249;
    let _e4: Motor = motor_reversal(_e3);
    let _e7: Motor = self_1249;
    let _e8: Scalar = motor_squared_magnitude(_e7);
    let _e13: Motor = self_1249;
    let _e14: Motor = motor_reversal(_e13);
    let _e17: Motor = self_1249;
    let _e18: Scalar = motor_squared_magnitude(_e17);
    let _e22: Motor = motor_scalar_geometric_product(_e14, Scalar((1.0 / _e18.g0_)));
    return _e22;
}

fn point_and_plane_zero() -> PointAndPlane {
    return PointAndPlane(vec4<f32>(0.0), vec4<f32>(0.0));
}

fn point_and_plane_one() -> PointAndPlane {
    return PointAndPlane(vec4<f32>(0.0), vec4<f32>(0.0));
}

fn point_and_plane_neg(self_1250: PointAndPlane) -> PointAndPlane {
    var self_1251: PointAndPlane;

    self_1251 = self_1250;
    let _e2: PointAndPlane = self_1251;
    let _e8: PointAndPlane = self_1251;
    return PointAndPlane((_e2.g0_ * vec4<f32>(-(1.0))), (_e8.g1_ * vec4<f32>(-(1.0))));
}

fn point_and_plane_automorphism(self_1252: PointAndPlane) -> PointAndPlane {
    var self_1253: PointAndPlane;

    self_1253 = self_1252;
    let _e2: PointAndPlane = self_1253;
    let _e8: PointAndPlane = self_1253;
    return PointAndPlane((_e2.g0_ * vec4<f32>(-(1.0))), (_e8.g1_ * vec4<f32>(-(1.0))));
}

fn point_and_plane_reversal(self_1254: PointAndPlane) -> PointAndPlane {
    var self_1255: PointAndPlane;

    self_1255 = self_1254;
    let _e2: PointAndPlane = self_1255;
    let _e8: PointAndPlane = self_1255;
    return PointAndPlane((_e2.g0_ * vec4<f32>(-(1.0))), _e8.g1_);
}

fn point_and_plane_conjugation(self_1256: PointAndPlane) -> PointAndPlane {
    var self_1257: PointAndPlane;

    self_1257 = self_1256;
    let _e2: PointAndPlane = self_1257;
    let _e4: PointAndPlane = self_1257;
    return PointAndPlane(_e2.g0_, (_e4.g1_ * vec4<f32>(-(1.0))));
}

fn point_and_plane_dual(self_1258: PointAndPlane) -> PointAndPlane {
    var self_1259: PointAndPlane;

    self_1259 = self_1258;
    let _e2: PointAndPlane = self_1259;
    let _e4: PointAndPlane = self_1259;
    return PointAndPlane(_e2.g1_, (_e4.g0_ * vec4<f32>(-(1.0))));
}

fn point_and_plane_scalar_geometric_product(self_1260: PointAndPlane, other_1060: Scalar) -> PointAndPlane {
    var self_1261: PointAndPlane;
    var other_1061: Scalar;

    self_1261 = self_1260;
    other_1061 = other_1060;
    let _e4: PointAndPlane = self_1261;
    let _e6: Scalar = other_1061;
    let _e10: PointAndPlane = self_1261;
    let _e12: Scalar = other_1061;
    return PointAndPlane((_e4.g0_ * vec4<f32>(_e6.g0_)), (_e10.g1_ * vec4<f32>(_e12.g0_)));
}

fn point_and_plane_scalar_outer_product(self_1262: PointAndPlane, other_1062: Scalar) -> PointAndPlane {
    var self_1263: PointAndPlane;
    var other_1063: Scalar;

    self_1263 = self_1262;
    other_1063 = other_1062;
    let _e4: PointAndPlane = self_1263;
    let _e6: Scalar = other_1063;
    let _e10: PointAndPlane = self_1263;
    let _e12: Scalar = other_1063;
    return PointAndPlane((_e4.g0_ * vec4<f32>(_e6.g0_)), (_e10.g1_ * vec4<f32>(_e12.g0_)));
}

fn point_and_plane_scalar_inner_product(self_1264: PointAndPlane, other_1064: Scalar) -> PointAndPlane {
    var self_1265: PointAndPlane;
    var other_1065: Scalar;

    self_1265 = self_1264;
    other_1065 = other_1064;
    let _e4: PointAndPlane = self_1265;
    let _e6: Scalar = other_1065;
    let _e10: PointAndPlane = self_1265;
    let _e12: Scalar = other_1065;
    return PointAndPlane((_e4.g0_ * vec4<f32>(_e6.g0_)), (_e10.g1_ * vec4<f32>(_e12.g0_)));
}

fn point_and_plane_scalar_right_contraction(self_1266: PointAndPlane, other_1066: Scalar) -> PointAndPlane {
    var self_1267: PointAndPlane;
    var other_1067: Scalar;

    self_1267 = self_1266;
    other_1067 = other_1066;
    let _e4: PointAndPlane = self_1267;
    let _e6: Scalar = other_1067;
    let _e10: PointAndPlane = self_1267;
    let _e12: Scalar = other_1067;
    return PointAndPlane((_e4.g0_ * vec4<f32>(_e6.g0_)), (_e10.g1_ * vec4<f32>(_e12.g0_)));
}

fn point_and_plane_multi_vector_add(self_1268: PointAndPlane, other_1068: MultiVector) -> MultiVector {
    var self_1269: PointAndPlane;
    var other_1069: MultiVector;

    self_1269 = self_1268;
    other_1069 = other_1068;
    let _e4: MultiVector = other_1069;
    let _e6: PointAndPlane = self_1269;
    let _e9: PointAndPlane = self_1269;
    let _e12: PointAndPlane = self_1269;
    let _e15: PointAndPlane = self_1269;
    let _e19: MultiVector = other_1069;
    let _e22: PointAndPlane = self_1269;
    let _e25: PointAndPlane = self_1269;
    let _e28: PointAndPlane = self_1269;
    let _e31: PointAndPlane = self_1269;
    let _e35: MultiVector = other_1069;
    let _e38: MultiVector = other_1069;
    return MultiVector(_e4.g0_, (vec4<f32>(_e6.g1_.x, _e9.g0_.y, _e12.g0_.z, _e15.g0_.w) + _e19.g1_), (vec4<f32>(_e22.g0_.x, _e25.g1_.y, _e28.g1_.z, _e31.g1_.w) + _e35.g2_), _e38.g3_);
}

fn point_and_plane_multi_vector_sub(self_1270: PointAndPlane, other_1070: MultiVector) -> MultiVector {
    var self_1271: PointAndPlane;
    var other_1071: MultiVector;

    self_1271 = self_1270;
    other_1071 = other_1070;
    let _e6: MultiVector = other_1071;
    let _e9: PointAndPlane = self_1271;
    let _e12: PointAndPlane = self_1271;
    let _e15: PointAndPlane = self_1271;
    let _e18: PointAndPlane = self_1271;
    let _e22: MultiVector = other_1071;
    let _e25: PointAndPlane = self_1271;
    let _e28: PointAndPlane = self_1271;
    let _e31: PointAndPlane = self_1271;
    let _e34: PointAndPlane = self_1271;
    let _e38: MultiVector = other_1071;
    let _e43: MultiVector = other_1071;
    return MultiVector((vec4<f32>(0.0) - _e6.g0_), (vec4<f32>(_e9.g1_.x, _e12.g0_.y, _e15.g0_.z, _e18.g0_.w) - _e22.g1_), (vec4<f32>(_e25.g0_.x, _e28.g1_.y, _e31.g1_.z, _e34.g1_.w) - _e38.g2_), (vec4<f32>(0.0) - _e43.g3_));
}

fn point_and_plane_multi_vector_geometric_product(self_1272: PointAndPlane, other_1072: MultiVector) -> MultiVector {
    var self_1273: PointAndPlane;
    var other_1073: MultiVector;

    self_1273 = self_1272;
    other_1073 = other_1072;
    let _e4: PointAndPlane = self_1273;
    let _e8: MultiVector = other_1073;
    let _e18: PointAndPlane = self_1273;
    let _e22: MultiVector = other_1073;
    let _e36: PointAndPlane = self_1273;
    let _e40: MultiVector = other_1073;
    let _e54: PointAndPlane = self_1273;
    let _e58: MultiVector = other_1073;
    let _e72: PointAndPlane = self_1273;
    let _e76: MultiVector = other_1073;
    let _e89: PointAndPlane = self_1273;
    let _e93: MultiVector = other_1073;
    let _e105: PointAndPlane = self_1273;
    let _e109: MultiVector = other_1073;
    let _e121: PointAndPlane = self_1273;
    let _e125: MultiVector = other_1073;
    let _e137: PointAndPlane = self_1273;
    let _e141: MultiVector = other_1073;
    let _e144: PointAndPlane = self_1273;
    let _e148: MultiVector = other_1073;
    let _e160: PointAndPlane = self_1273;
    let _e164: MultiVector = other_1073;
    let _e176: PointAndPlane = self_1273;
    let _e180: MultiVector = other_1073;
    let _e192: PointAndPlane = self_1273;
    let _e196: MultiVector = other_1073;
    let _e209: PointAndPlane = self_1273;
    let _e213: MultiVector = other_1073;
    let _e226: PointAndPlane = self_1273;
    let _e230: MultiVector = other_1073;
    let _e243: PointAndPlane = self_1273;
    let _e247: MultiVector = other_1073;
    let _e260: PointAndPlane = self_1273;
    let _e264: MultiVector = other_1073;
    let _e276: PointAndPlane = self_1273;
    let _e280: MultiVector = other_1073;
    let _e293: PointAndPlane = self_1273;
    let _e297: MultiVector = other_1073;
    let _e310: PointAndPlane = self_1273;
    let _e314: MultiVector = other_1073;
    let _e327: PointAndPlane = self_1273;
    let _e331: MultiVector = other_1073;
    let _e335: PointAndPlane = self_1273;
    let _e339: MultiVector = other_1073;
    let _e351: PointAndPlane = self_1273;
    let _e355: MultiVector = other_1073;
    let _e367: PointAndPlane = self_1273;
    let _e371: MultiVector = other_1073;
    let _e385: PointAndPlane = self_1273;
    let _e389: MultiVector = other_1073;
    let _e393: PointAndPlane = self_1273;
    let _e397: MultiVector = other_1073;
    let _e410: PointAndPlane = self_1273;
    let _e414: MultiVector = other_1073;
    let _e427: PointAndPlane = self_1273;
    let _e431: MultiVector = other_1073;
    let _e444: PointAndPlane = self_1273;
    let _e448: MultiVector = other_1073;
    let _e452: PointAndPlane = self_1273;
    let _e456: MultiVector = other_1073;
    let _e469: PointAndPlane = self_1273;
    let _e473: MultiVector = other_1073;
    let _e486: PointAndPlane = self_1273;
    let _e490: MultiVector = other_1073;
    return MultiVector((((((((((vec4<f32>(_e4.g0_.x) * _e8.g2_) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0)) + ((vec4<f32>(_e18.g0_.y) * _e22.g1_.yxwz) * vec4<f32>(-(1.0), -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e36.g0_.z) * _e40.g1_.zwxy) * vec4<f32>(-(1.0), -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e54.g0_.w) * _e58.g1_.wzyx) * vec4<f32>(-(1.0), 1.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e72.g1_.x) * _e76.g1_) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0)))) + ((vec4<f32>(_e89.g1_.y) * _e93.g2_.yxwz) * vec4<f32>(1.0, 1.0, -(1.0), 1.0))) + ((vec4<f32>(_e105.g1_.z) * _e109.g2_.zwxy) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e121.g1_.w) * _e125.g2_.wzyx) * vec4<f32>(1.0, -(1.0), 1.0, 1.0))), ((((((((vec4<f32>(_e137.g0_.x) * _e141.g3_) + ((vec4<f32>(_e144.g0_.y) * _e148.g0_.yxwz) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e160.g0_.z) * _e164.g0_.zwxy) * vec4<f32>(1.0, -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e176.g0_.w) * _e180.g0_.wzyx) * vec4<f32>(1.0, 1.0, -(1.0), 1.0))) + ((vec4<f32>(_e192.g1_.x) * _e196.g0_) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0)))) + ((vec4<f32>(_e209.g1_.y) * _e213.g3_.yxwz) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))) + ((vec4<f32>(_e226.g1_.z) * _e230.g3_.zwxy) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e243.g1_.w) * _e247.g3_.wzyx) * vec4<f32>(-(1.0), -(1.0), 1.0, 1.0))), (((((((((vec4<f32>(_e260.g0_.x) * _e264.g0_) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))) + ((vec4<f32>(_e276.g0_.y) * _e280.g3_.yxwz) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))) + ((vec4<f32>(_e293.g0_.z) * _e297.g3_.zwxy) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e310.g0_.w) * _e314.g3_.wzyx) * vec4<f32>(-(1.0), -(1.0), 1.0, 1.0))) + (vec4<f32>(_e327.g1_.x) * _e331.g3_)) + ((vec4<f32>(_e335.g1_.y) * _e339.g0_.yxwz) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e351.g1_.z) * _e355.g0_.zwxy) * vec4<f32>(1.0, -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e367.g1_.w) * _e371.g0_.wzyx) * vec4<f32>(1.0, 1.0, -(1.0), 1.0))), ((((((((vec4<f32>(0.0) - (vec4<f32>(_e385.g0_.x) * _e389.g1_)) + ((vec4<f32>(_e393.g0_.y) * _e397.g2_.yxwz) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))) + ((vec4<f32>(_e410.g0_.z) * _e414.g2_.zwxy) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e427.g0_.w) * _e431.g2_.wzyx) * vec4<f32>(-(1.0), -(1.0), 1.0, 1.0))) + (vec4<f32>(_e444.g1_.x) * _e448.g2_)) + ((vec4<f32>(_e452.g1_.y) * _e456.g1_.yxwz) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e469.g1_.z) * _e473.g1_.zwxy) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e486.g1_.w) * _e490.g1_.wzyx) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))));
}

fn point_and_plane_multi_vector_scalar_product(self_1274: PointAndPlane, other_1074: MultiVector) -> Scalar {
    var self_1275: PointAndPlane;
    var other_1075: MultiVector;

    self_1275 = self_1274;
    other_1075 = other_1074;
    let _e5: PointAndPlane = self_1275;
    let _e8: MultiVector = other_1075;
    let _e13: PointAndPlane = self_1275;
    let _e16: MultiVector = other_1075;
    let _e21: PointAndPlane = self_1275;
    let _e24: MultiVector = other_1075;
    let _e29: PointAndPlane = self_1275;
    let _e32: MultiVector = other_1075;
    let _e37: PointAndPlane = self_1275;
    let _e40: MultiVector = other_1075;
    let _e45: PointAndPlane = self_1275;
    let _e48: MultiVector = other_1075;
    let _e53: PointAndPlane = self_1275;
    let _e56: MultiVector = other_1075;
    let _e61: PointAndPlane = self_1275;
    let _e64: MultiVector = other_1075;
    return Scalar(((((((((0.0 - (_e5.g0_.x * _e8.g2_.x)) - (_e13.g0_.y * _e16.g1_.y)) - (_e21.g0_.z * _e24.g1_.z)) - (_e29.g0_.w * _e32.g1_.w)) + (_e37.g1_.x * _e40.g1_.x)) + (_e45.g1_.y * _e48.g2_.y)) + (_e53.g1_.z * _e56.g2_.z)) + (_e61.g1_.w * _e64.g2_.w)));
}

fn point_and_plane_rotor_geometric_product(self_1276: PointAndPlane, other_1076: Rotor) -> PointAndPlane {
    var self_1277: PointAndPlane;
    var other_1077: Rotor;

    self_1277 = self_1276;
    other_1077 = other_1076;
    let _e4: PointAndPlane = self_1277;
    let _e8: Rotor = other_1077;
    let _e19: PointAndPlane = self_1277;
    let _e23: Rotor = other_1077;
    let _e35: PointAndPlane = self_1277;
    let _e39: Rotor = other_1077;
    let _e53: PointAndPlane = self_1277;
    let _e57: Rotor = other_1077;
    let _e69: PointAndPlane = self_1277;
    let _e73: Rotor = other_1077;
    let _e85: PointAndPlane = self_1277;
    let _e89: Rotor = other_1077;
    let _e101: PointAndPlane = self_1277;
    let _e104: Rotor = other_1077;
    let _e116: PointAndPlane = self_1277;
    let _e120: Rotor = other_1077;
    let _e131: PointAndPlane = self_1277;
    let _e135: Rotor = other_1077;
    let _e147: PointAndPlane = self_1277;
    let _e151: Rotor = other_1077;
    let _e163: PointAndPlane = self_1277;
    let _e167: Rotor = other_1077;
    let _e179: PointAndPlane = self_1277;
    let _e183: Rotor = other_1077;
    let _e195: PointAndPlane = self_1277;
    let _e199: Rotor = other_1077;
    let _e211: PointAndPlane = self_1277;
    let _e214: Rotor = other_1077;
    return PointAndPlane(((((((((vec4<f32>(_e4.g0_.z) * _e8.g0_.wwxy) * vec4<f32>(0.0, -(1.0), 1.0, 1.0)) + ((vec4<f32>(_e19.g0_.w) * _e23.g0_.zzyx) * vec4<f32>(0.0, 1.0, -(1.0), 1.0))) + ((vec4<f32>(_e35.g1_.x) * _e39.g0_.yyzw) * vec4<f32>(0.0, -(1.0), -(1.0), -(1.0)))) + ((vec4<f32>(_e53.g1_.y) * vec4<f32>(_e57.g0_.y)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e69.g1_.z) * vec4<f32>(_e73.g0_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e85.g1_.w) * vec4<f32>(_e89.g0_.w)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((_e101.g0_.xyyy * _e104.g0_.xxwz) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))), ((((((((vec4<f32>(_e116.g0_.z) * vec4<f32>(_e120.g0_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) + ((vec4<f32>(_e131.g0_.w) * vec4<f32>(_e135.g0_.w)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e147.g1_.x) * vec4<f32>(_e151.g0_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e163.g1_.y) * _e167.g0_.xxwz) * vec4<f32>(0.0, 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e179.g1_.z) * _e183.g0_.wwxy) * vec4<f32>(0.0, -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e195.g1_.w) * _e199.g0_.zzyx) * vec4<f32>(0.0, 1.0, -(1.0), 1.0))) + ((_e211.g0_.yxxx * _e214.g0_.yyzw) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0)))));
}

fn point_and_plane_rotor_outer_product(self_1278: PointAndPlane, other_1078: Rotor) -> PointAndPlane {
    var self_1279: PointAndPlane;
    var other_1079: Rotor;

    self_1279 = self_1278;
    other_1079 = other_1078;
    let _e4: PointAndPlane = self_1279;
    let _e8: Rotor = other_1079;
    let _e21: PointAndPlane = self_1279;
    let _e25: Rotor = other_1079;
    let _e37: PointAndPlane = self_1279;
    let _e41: Rotor = other_1079;
    let _e53: PointAndPlane = self_1279;
    let _e57: Rotor = other_1079;
    let _e69: PointAndPlane = self_1279;
    let _e71: Rotor = other_1079;
    let _e77: PointAndPlane = self_1279;
    let _e79: Rotor = other_1079;
    return PointAndPlane(((((((vec4<f32>(_e4.g1_.x) * _e8.g0_.yyzw) * vec4<f32>(0.0, -(1.0), -(1.0), -(1.0))) + ((vec4<f32>(_e21.g1_.y) * vec4<f32>(_e25.g0_.y)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e37.g1_.z) * vec4<f32>(_e41.g0_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e53.g1_.w) * vec4<f32>(_e57.g0_.w)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + (_e69.g0_ * vec4<f32>(_e71.g0_.x))), (_e77.g1_ * vec4<f32>(_e79.g0_.x)));
}

fn point_and_plane_rotor_inner_product(self_1280: PointAndPlane, other_1080: Rotor) -> PointAndPlane {
    var self_1281: PointAndPlane;
    var other_1081: Rotor;

    self_1281 = self_1280;
    other_1081 = other_1080;
    let _e4: PointAndPlane = self_1281;
    let _e6: Rotor = other_1081;
    let _e11: PointAndPlane = self_1281;
    let _e15: Rotor = other_1081;
    let _e26: PointAndPlane = self_1281;
    let _e30: Rotor = other_1081;
    let _e42: PointAndPlane = self_1281;
    let _e46: Rotor = other_1081;
    let _e58: PointAndPlane = self_1281;
    let _e62: Rotor = other_1081;
    let _e74: PointAndPlane = self_1281;
    let _e78: Rotor = other_1081;
    let _e90: PointAndPlane = self_1281;
    let _e94: Rotor = other_1081;
    let _e106: PointAndPlane = self_1281;
    let _e109: Rotor = other_1081;
    return PointAndPlane((_e4.g0_ * vec4<f32>(_e6.g0_.x)), ((((((((vec4<f32>(_e11.g0_.z) * vec4<f32>(_e15.g0_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) + ((vec4<f32>(_e26.g0_.w) * vec4<f32>(_e30.g0_.w)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e42.g1_.x) * vec4<f32>(_e46.g0_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e58.g1_.y) * _e62.g0_.xxwz) * vec4<f32>(0.0, 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e74.g1_.z) * _e78.g0_.wwxy) * vec4<f32>(0.0, -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e90.g1_.w) * _e94.g0_.zzyx) * vec4<f32>(0.0, 1.0, -(1.0), 1.0))) + ((_e106.g0_.yxxx * _e109.g0_.yyzw) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0)))));
}

fn point_and_plane_rotor_right_contraction(self_1282: PointAndPlane, other_1082: Rotor) -> PointAndPlane {
    var self_1283: PointAndPlane;
    var other_1083: Rotor;

    self_1283 = self_1282;
    other_1083 = other_1082;
    let _e4: PointAndPlane = self_1283;
    let _e6: Rotor = other_1083;
    let _e11: PointAndPlane = self_1283;
    let _e15: Rotor = other_1083;
    let _e26: PointAndPlane = self_1283;
    let _e30: Rotor = other_1083;
    let _e42: PointAndPlane = self_1283;
    let _e46: Rotor = other_1083;
    let _e58: PointAndPlane = self_1283;
    let _e62: Rotor = other_1083;
    let _e74: PointAndPlane = self_1283;
    let _e78: Rotor = other_1083;
    let _e90: PointAndPlane = self_1283;
    let _e94: Rotor = other_1083;
    let _e106: PointAndPlane = self_1283;
    let _e109: Rotor = other_1083;
    return PointAndPlane((_e4.g0_ * vec4<f32>(_e6.g0_.x)), ((((((((vec4<f32>(_e11.g0_.z) * vec4<f32>(_e15.g0_.z)) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) + ((vec4<f32>(_e26.g0_.w) * vec4<f32>(_e30.g0_.w)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e42.g1_.x) * vec4<f32>(_e46.g0_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0))) + ((vec4<f32>(_e58.g1_.y) * vec4<f32>(_e62.g0_.x)) * vec4<f32>(0.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e74.g1_.z) * vec4<f32>(_e78.g0_.x)) * vec4<f32>(0.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e90.g1_.w) * vec4<f32>(_e94.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((_e106.g0_.yxxx * _e109.g0_.yyzw) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0)))));
}

fn point_and_plane_point_into(self_1284: PointAndPlane) -> Point {
    var self_1285: PointAndPlane;

    self_1285 = self_1284;
    let _e2: PointAndPlane = self_1285;
    return Point(_e2.g0_);
}

fn point_and_plane_point_add(self_1286: PointAndPlane, other_1084: Point) -> PointAndPlane {
    var self_1287: PointAndPlane;
    var other_1085: Point;

    self_1287 = self_1286;
    other_1085 = other_1084;
    let _e4: PointAndPlane = self_1287;
    let _e6: Point = other_1085;
    let _e9: PointAndPlane = self_1287;
    return PointAndPlane((_e4.g0_ + _e6.g0_), _e9.g1_);
}

fn point_and_plane_point_sub(self_1288: PointAndPlane, other_1086: Point) -> PointAndPlane {
    var self_1289: PointAndPlane;
    var other_1087: Point;

    self_1289 = self_1288;
    other_1087 = other_1086;
    let _e4: PointAndPlane = self_1289;
    let _e6: Point = other_1087;
    let _e9: PointAndPlane = self_1289;
    return PointAndPlane((_e4.g0_ - _e6.g0_), _e9.g1_);
}

fn point_and_plane_point_geometric_product(self_1290: PointAndPlane, other_1088: Point) -> Motor {
    var self_1291: PointAndPlane;
    var other_1089: Point;

    self_1291 = self_1290;
    other_1089 = other_1088;
    let _e4: PointAndPlane = self_1291;
    let _e8: Point = other_1089;
    let _e20: PointAndPlane = self_1291;
    let _e24: Point = other_1089;
    let _e37: PointAndPlane = self_1291;
    let _e41: Point = other_1089;
    let _e54: PointAndPlane = self_1291;
    let _e58: Point = other_1089;
    let _e70: PointAndPlane = self_1291;
    let _e74: Point = other_1089;
    let _e86: PointAndPlane = self_1291;
    let _e90: Point = other_1089;
    let _e102: PointAndPlane = self_1291;
    let _e105: PointAndPlane = self_1291;
    let _e108: PointAndPlane = self_1291;
    let _e111: PointAndPlane = self_1291;
    let _e115: Point = other_1089;
    let _e123: PointAndPlane = self_1291;
    let _e127: Point = other_1089;
    let _e138: PointAndPlane = self_1291;
    let _e142: Point = other_1089;
    let _e154: PointAndPlane = self_1291;
    let _e158: Point = other_1089;
    let _e170: PointAndPlane = self_1291;
    let _e174: Point = other_1089;
    let _e186: PointAndPlane = self_1291;
    let _e190: Point = other_1089;
    let _e202: PointAndPlane = self_1291;
    let _e206: Point = other_1089;
    let _e218: PointAndPlane = self_1291;
    let _e221: PointAndPlane = self_1291;
    let _e224: PointAndPlane = self_1291;
    let _e227: PointAndPlane = self_1291;
    let _e231: Point = other_1089;
    return Motor(((((((((vec4<f32>(_e4.g0_.y) * _e8.g0_.yywz) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e20.g0_.z) * _e24.g0_.zwzy) * vec4<f32>(-(1.0), -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e37.g0_.w) * _e41.g0_.wzyw) * vec4<f32>(-(1.0), 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e54.g1_.y) * vec4<f32>(_e58.g0_.x)) * vec4<f32>(0.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e70.g1_.z) * vec4<f32>(_e74.g0_.x)) * vec4<f32>(0.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e86.g1_.w) * vec4<f32>(_e90.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e102.g0_.x, _e105.g1_.x, _e108.g1_.x, _e111.g1_.x) * _e115.g0_) * vec4<f32>(-(1.0)))), ((((((((vec4<f32>(_e123.g0_.y) * vec4<f32>(_e127.g0_.x)) * vec4<f32>(0.0, 1.0, 0.0, 0.0)) + ((vec4<f32>(_e138.g0_.z) * vec4<f32>(_e142.g0_.x)) * vec4<f32>(0.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e154.g0_.w) * vec4<f32>(_e158.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e170.g1_.y) * _e174.g0_.yywz) * vec4<f32>(1.0, 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e186.g1_.z) * _e190.g0_.zwzy) * vec4<f32>(1.0, -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e202.g1_.w) * _e206.g0_.wzyw) * vec4<f32>(1.0, 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e218.g1_.x, _e221.g0_.x, _e224.g0_.x, _e227.g0_.x) * _e231.g0_) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0)))));
}

fn point_and_plane_point_right_contraction(self_1292: PointAndPlane, other_1090: Point) -> Scalar {
    var self_1293: PointAndPlane;
    var other_1091: Point;

    self_1293 = self_1292;
    other_1091 = other_1090;
    let _e5: PointAndPlane = self_1293;
    let _e8: Point = other_1091;
    let _e13: PointAndPlane = self_1293;
    let _e16: Point = other_1091;
    let _e21: PointAndPlane = self_1293;
    let _e24: Point = other_1091;
    let _e29: PointAndPlane = self_1293;
    let _e32: Point = other_1091;
    return Scalar(((((0.0 - (_e5.g0_.x * _e8.g0_.x)) - (_e13.g0_.y * _e16.g0_.y)) - (_e21.g0_.z * _e24.g0_.z)) - (_e29.g0_.w * _e32.g0_.w)));
}

fn point_and_plane_point_scalar_product(self_1294: PointAndPlane, other_1092: Point) -> Scalar {
    var self_1295: PointAndPlane;
    var other_1093: Point;

    self_1295 = self_1294;
    other_1093 = other_1092;
    let _e5: PointAndPlane = self_1295;
    let _e8: Point = other_1093;
    let _e13: PointAndPlane = self_1295;
    let _e16: Point = other_1093;
    let _e21: PointAndPlane = self_1295;
    let _e24: Point = other_1093;
    let _e29: PointAndPlane = self_1295;
    let _e32: Point = other_1093;
    return Scalar(((((0.0 - (_e5.g0_.x * _e8.g0_.x)) - (_e13.g0_.y * _e16.g0_.y)) - (_e21.g0_.z * _e24.g0_.z)) - (_e29.g0_.w * _e32.g0_.w)));
}

fn point_and_plane_ideal_point_geometric_product(self_1296: PointAndPlane, other_1094: IdealPoint) -> PointAndPlane {
    var self_1297: PointAndPlane;
    var other_1095: IdealPoint;

    self_1297 = self_1296;
    other_1095 = other_1094;
    let _e4: PointAndPlane = self_1297;
    let _e8: IdealPoint = other_1095;
    let _e20: PointAndPlane = self_1297;
    let _e24: IdealPoint = other_1095;
    let _e37: PointAndPlane = self_1297;
    let _e41: IdealPoint = other_1095;
    let _e44: IdealPoint = other_1095;
    let _e47: IdealPoint = other_1095;
    let _e50: IdealPoint = other_1095;
    let _e63: PointAndPlane = self_1297;
    let _e67: IdealPoint = other_1095;
    let _e70: IdealPoint = other_1095;
    let _e73: IdealPoint = other_1095;
    let _e76: IdealPoint = other_1095;
    let _e89: PointAndPlane = self_1297;
    let _e93: IdealPoint = other_1095;
    let _e96: IdealPoint = other_1095;
    let _e99: IdealPoint = other_1095;
    let _e102: IdealPoint = other_1095;
    let _e115: PointAndPlane = self_1297;
    let _e118: IdealPoint = other_1095;
    let _e121: IdealPoint = other_1095;
    let _e124: IdealPoint = other_1095;
    let _e127: IdealPoint = other_1095;
    let _e140: PointAndPlane = self_1297;
    let _e144: IdealPoint = other_1095;
    let _e147: IdealPoint = other_1095;
    let _e150: IdealPoint = other_1095;
    let _e153: IdealPoint = other_1095;
    let _e165: PointAndPlane = self_1297;
    let _e169: IdealPoint = other_1095;
    let _e172: IdealPoint = other_1095;
    let _e175: IdealPoint = other_1095;
    let _e178: IdealPoint = other_1095;
    let _e191: PointAndPlane = self_1297;
    let _e195: IdealPoint = other_1095;
    let _e198: IdealPoint = other_1095;
    let _e201: IdealPoint = other_1095;
    let _e204: IdealPoint = other_1095;
    let _e216: PointAndPlane = self_1297;
    let _e220: IdealPoint = other_1095;
    let _e233: PointAndPlane = self_1297;
    let _e237: IdealPoint = other_1095;
    let _e250: PointAndPlane = self_1297;
    let _e253: PointAndPlane = self_1297;
    let _e256: PointAndPlane = self_1297;
    let _e259: PointAndPlane = self_1297;
    let _e263: IdealPoint = other_1095;
    let _e266: IdealPoint = other_1095;
    let _e269: IdealPoint = other_1095;
    let _e272: IdealPoint = other_1095;
    return PointAndPlane((((((((vec4<f32>(_e4.g0_.z) * vec4<f32>(_e8.g0_.y)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0)) + ((vec4<f32>(_e20.g0_.w) * vec4<f32>(_e24.g0_.z)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((vec4<f32>(_e37.g1_.y) * vec4<f32>(_e41.g0_.z, _e44.g0_.z, _e47.g0_.z, _e50.g0_.y)) * vec4<f32>(0.0, 0.0, -(1.0), 1.0))) + ((vec4<f32>(_e63.g1_.z) * vec4<f32>(_e67.g0_.z, _e70.g0_.z, _e73.g0_.z, _e76.g0_.x)) * vec4<f32>(0.0, 1.0, 0.0, -(1.0)))) + ((vec4<f32>(_e89.g1_.w) * vec4<f32>(_e93.g0_.y, _e96.g0_.y, _e99.g0_.x, _e102.g0_.y)) * vec4<f32>(0.0, -(1.0), 1.0, 0.0))) + ((_e115.g0_.yxxx * vec4<f32>(_e118.g0_.x, _e121.g0_.x, _e124.g0_.y, _e127.g0_.z)) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0))), (((((((vec4<f32>(_e140.g0_.z) * vec4<f32>(_e144.g0_.z, _e147.g0_.z, _e150.g0_.z, _e153.g0_.x)) * vec4<f32>(0.0, 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e165.g0_.w) * vec4<f32>(_e169.g0_.y, _e172.g0_.y, _e175.g0_.x, _e178.g0_.y)) * vec4<f32>(0.0, -(1.0), 1.0, 0.0))) + ((vec4<f32>(_e191.g1_.x) * vec4<f32>(_e195.g0_.x, _e198.g0_.x, _e201.g0_.y, _e204.g0_.z)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))) + ((vec4<f32>(_e216.g1_.z) * vec4<f32>(_e220.g0_.y)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((vec4<f32>(_e233.g1_.w) * vec4<f32>(_e237.g0_.z)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((vec4<f32>(_e250.g1_.y, _e253.g0_.x, _e256.g0_.y, _e259.g0_.y) * vec4<f32>(_e263.g0_.x, _e266.g0_.x, _e269.g0_.z, _e272.g0_.y)) * vec4<f32>(-(1.0), 0.0, -(1.0), 1.0))));
}

fn point_and_plane_ideal_point_regressive_product(self_1298: PointAndPlane, other_1096: IdealPoint) -> Plane {
    var self_1299: PointAndPlane;
    var other_1097: IdealPoint;

    self_1299 = self_1298;
    other_1097 = other_1096;
    let _e4: PointAndPlane = self_1299;
    let _e8: IdealPoint = other_1097;
    let _e20: PointAndPlane = self_1299;
    let _e24: IdealPoint = other_1097;
    let _e37: PointAndPlane = self_1299;
    let _e40: IdealPoint = other_1097;
    let _e43: IdealPoint = other_1097;
    let _e46: IdealPoint = other_1097;
    let _e49: IdealPoint = other_1097;
    return Plane(((((vec4<f32>(_e4.g0_.z) * vec4<f32>(_e8.g0_.y)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0)) + ((vec4<f32>(_e20.g0_.w) * vec4<f32>(_e24.g0_.z)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((_e37.g0_.yxxx * vec4<f32>(_e40.g0_.x, _e43.g0_.x, _e46.g0_.y, _e49.g0_.z)) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0))));
}

fn point_and_plane_ideal_point_inner_product(self_1300: PointAndPlane, other_1098: IdealPoint) -> Plane {
    var self_1301: PointAndPlane;
    var other_1099: IdealPoint;

    self_1301 = self_1300;
    other_1099 = other_1098;
    let _e4: PointAndPlane = self_1301;
    let _e8: IdealPoint = other_1099;
    let _e11: IdealPoint = other_1099;
    let _e14: IdealPoint = other_1099;
    let _e17: IdealPoint = other_1099;
    let _e29: PointAndPlane = self_1301;
    let _e33: IdealPoint = other_1099;
    let _e36: IdealPoint = other_1099;
    let _e39: IdealPoint = other_1099;
    let _e42: IdealPoint = other_1099;
    let _e55: PointAndPlane = self_1301;
    let _e59: IdealPoint = other_1099;
    let _e62: IdealPoint = other_1099;
    let _e65: IdealPoint = other_1099;
    let _e68: IdealPoint = other_1099;
    let _e80: PointAndPlane = self_1301;
    let _e84: IdealPoint = other_1099;
    let _e97: PointAndPlane = self_1301;
    let _e101: IdealPoint = other_1099;
    let _e114: PointAndPlane = self_1301;
    let _e117: PointAndPlane = self_1301;
    let _e120: PointAndPlane = self_1301;
    let _e123: PointAndPlane = self_1301;
    let _e127: IdealPoint = other_1099;
    let _e130: IdealPoint = other_1099;
    let _e133: IdealPoint = other_1099;
    let _e136: IdealPoint = other_1099;
    return Plane((((((((vec4<f32>(_e4.g0_.z) * vec4<f32>(_e8.g0_.z, _e11.g0_.z, _e14.g0_.z, _e17.g0_.x)) * vec4<f32>(0.0, 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e29.g0_.w) * vec4<f32>(_e33.g0_.y, _e36.g0_.y, _e39.g0_.x, _e42.g0_.y)) * vec4<f32>(0.0, -(1.0), 1.0, 0.0))) + ((vec4<f32>(_e55.g1_.x) * vec4<f32>(_e59.g0_.x, _e62.g0_.x, _e65.g0_.y, _e68.g0_.z)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))) + ((vec4<f32>(_e80.g1_.z) * vec4<f32>(_e84.g0_.y)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((vec4<f32>(_e97.g1_.w) * vec4<f32>(_e101.g0_.z)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((vec4<f32>(_e114.g1_.y, _e117.g0_.x, _e120.g0_.y, _e123.g0_.y) * vec4<f32>(_e127.g0_.x, _e130.g0_.x, _e133.g0_.z, _e136.g0_.y)) * vec4<f32>(-(1.0), 0.0, -(1.0), 1.0))));
}

fn point_and_plane_ideal_point_left_contraction(self_1302: PointAndPlane, other_1100: IdealPoint) -> Plane {
    var self_1303: PointAndPlane;
    var other_1101: IdealPoint;

    self_1303 = self_1302;
    other_1101 = other_1100;
    let _e4: PointAndPlane = self_1303;
    let _e8: IdealPoint = other_1101;
    let _e20: PointAndPlane = self_1303;
    let _e24: IdealPoint = other_1101;
    let _e37: PointAndPlane = self_1303;
    let _e40: IdealPoint = other_1101;
    let _e43: IdealPoint = other_1101;
    let _e46: IdealPoint = other_1101;
    let _e49: IdealPoint = other_1101;
    return Plane(((((vec4<f32>(_e4.g1_.z) * vec4<f32>(_e8.g0_.y)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0)) + ((vec4<f32>(_e20.g1_.w) * vec4<f32>(_e24.g0_.z)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((_e37.g1_.yxxx * vec4<f32>(_e40.g0_.x, _e43.g0_.x, _e46.g0_.y, _e49.g0_.z)) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0))));
}

fn point_and_plane_plane_into(self_1304: PointAndPlane) -> Plane {
    var self_1305: PointAndPlane;

    self_1305 = self_1304;
    let _e2: PointAndPlane = self_1305;
    return Plane(_e2.g1_);
}

fn point_and_plane_plane_add(self_1306: PointAndPlane, other_1102: Plane) -> PointAndPlane {
    var self_1307: PointAndPlane;
    var other_1103: Plane;

    self_1307 = self_1306;
    other_1103 = other_1102;
    let _e4: PointAndPlane = self_1307;
    let _e6: PointAndPlane = self_1307;
    let _e8: Plane = other_1103;
    return PointAndPlane(_e4.g0_, (_e6.g1_ + _e8.g0_));
}

fn point_and_plane_plane_sub(self_1308: PointAndPlane, other_1104: Plane) -> PointAndPlane {
    var self_1309: PointAndPlane;
    var other_1105: Plane;

    self_1309 = self_1308;
    other_1105 = other_1104;
    let _e4: PointAndPlane = self_1309;
    let _e6: PointAndPlane = self_1309;
    let _e8: Plane = other_1105;
    return PointAndPlane(_e4.g0_, (_e6.g1_ - _e8.g0_));
}

fn point_and_plane_plane_geometric_product(self_1310: PointAndPlane, other_1106: Plane) -> Motor {
    var self_1311: PointAndPlane;
    var other_1107: Plane;

    self_1311 = self_1310;
    other_1107 = other_1106;
    let _e4: PointAndPlane = self_1311;
    let _e8: Plane = other_1107;
    let _e20: PointAndPlane = self_1311;
    let _e24: Plane = other_1107;
    let _e37: PointAndPlane = self_1311;
    let _e41: Plane = other_1107;
    let _e54: PointAndPlane = self_1311;
    let _e58: Plane = other_1107;
    let _e70: PointAndPlane = self_1311;
    let _e74: Plane = other_1107;
    let _e86: PointAndPlane = self_1311;
    let _e90: Plane = other_1107;
    let _e102: PointAndPlane = self_1311;
    let _e105: PointAndPlane = self_1311;
    let _e108: PointAndPlane = self_1311;
    let _e111: PointAndPlane = self_1311;
    let _e115: Plane = other_1107;
    let _e119: PointAndPlane = self_1311;
    let _e123: Plane = other_1107;
    let _e135: PointAndPlane = self_1311;
    let _e139: Plane = other_1107;
    let _e152: PointAndPlane = self_1311;
    let _e156: Plane = other_1107;
    let _e169: PointAndPlane = self_1311;
    let _e173: Plane = other_1107;
    let _e186: PointAndPlane = self_1311;
    let _e190: Plane = other_1107;
    let _e203: PointAndPlane = self_1311;
    let _e207: Plane = other_1107;
    let _e220: PointAndPlane = self_1311;
    let _e223: PointAndPlane = self_1311;
    let _e226: PointAndPlane = self_1311;
    let _e229: PointAndPlane = self_1311;
    let _e233: Plane = other_1107;
    return Motor(((((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.x)) * vec4<f32>(0.0, -(1.0), 0.0, 0.0)) + ((vec4<f32>(_e20.g0_.z) * vec4<f32>(_e24.g0_.x)) * vec4<f32>(0.0, 0.0, -(1.0), 0.0))) + ((vec4<f32>(_e37.g0_.w) * vec4<f32>(_e41.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e54.g1_.y) * _e58.g0_.yywz) * vec4<f32>(1.0, 0.0, -(1.0), 1.0))) + ((vec4<f32>(_e70.g1_.z) * _e74.g0_.zwzy) * vec4<f32>(1.0, 1.0, 0.0, -(1.0)))) + ((vec4<f32>(_e86.g1_.w) * _e90.g0_.wzyw) * vec4<f32>(1.0, -(1.0), 1.0, 0.0))) + (vec4<f32>(_e102.g1_.x, _e105.g0_.x, _e108.g0_.x, _e111.g0_.x) * _e115.g0_)), ((((((((vec4<f32>(_e119.g0_.y) * _e123.g0_.yywz) * vec4<f32>(-(1.0), 0.0, -(1.0), 1.0)) + ((vec4<f32>(_e135.g0_.z) * _e139.g0_.zwzy) * vec4<f32>(-(1.0), 1.0, 0.0, -(1.0)))) + ((vec4<f32>(_e152.g0_.w) * _e156.g0_.wzyw) * vec4<f32>(-(1.0), -(1.0), 1.0, 0.0))) + ((vec4<f32>(_e169.g1_.y) * vec4<f32>(_e173.g0_.x)) * vec4<f32>(0.0, -(1.0), 0.0, 0.0))) + ((vec4<f32>(_e186.g1_.z) * vec4<f32>(_e190.g0_.x)) * vec4<f32>(0.0, 0.0, -(1.0), 0.0))) + ((vec4<f32>(_e203.g1_.w) * vec4<f32>(_e207.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, -(1.0)))) + ((vec4<f32>(_e220.g0_.x, _e223.g1_.x, _e226.g1_.x, _e229.g1_.x) * _e233.g0_) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0))));
}

fn point_and_plane_plane_regressive_product(self_1312: PointAndPlane, other_1108: Plane) -> Scalar {
    var self_1313: PointAndPlane;
    var other_1109: Plane;

    self_1313 = self_1312;
    other_1109 = other_1108;
    let _e5: PointAndPlane = self_1313;
    let _e8: Plane = other_1109;
    let _e13: PointAndPlane = self_1313;
    let _e16: Plane = other_1109;
    let _e21: PointAndPlane = self_1313;
    let _e24: Plane = other_1109;
    let _e29: PointAndPlane = self_1313;
    let _e32: Plane = other_1109;
    return Scalar(((((0.0 - (_e5.g0_.x * _e8.g0_.x)) - (_e13.g0_.y * _e16.g0_.y)) - (_e21.g0_.z * _e24.g0_.z)) - (_e29.g0_.w * _e32.g0_.w)));
}

fn point_and_plane_plane_left_contraction(self_1314: PointAndPlane, other_1110: Plane) -> Scalar {
    var self_1315: PointAndPlane;
    var other_1111: Plane;

    self_1315 = self_1314;
    other_1111 = other_1110;
    let _e4: PointAndPlane = self_1315;
    let _e7: Plane = other_1111;
    let _e11: PointAndPlane = self_1315;
    let _e14: Plane = other_1111;
    let _e19: PointAndPlane = self_1315;
    let _e22: Plane = other_1111;
    let _e27: PointAndPlane = self_1315;
    let _e30: Plane = other_1111;
    return Scalar(((((_e4.g1_.x * _e7.g0_.x) + (_e11.g1_.y * _e14.g0_.y)) + (_e19.g1_.z * _e22.g0_.z)) + (_e27.g1_.w * _e30.g0_.w)));
}

fn point_and_plane_plane_scalar_product(self_1316: PointAndPlane, other_1112: Plane) -> Scalar {
    var self_1317: PointAndPlane;
    var other_1113: Plane;

    self_1317 = self_1316;
    other_1113 = other_1112;
    let _e4: PointAndPlane = self_1317;
    let _e7: Plane = other_1113;
    let _e11: PointAndPlane = self_1317;
    let _e14: Plane = other_1113;
    let _e19: PointAndPlane = self_1317;
    let _e22: Plane = other_1113;
    let _e27: PointAndPlane = self_1317;
    let _e30: Plane = other_1113;
    return Scalar(((((_e4.g1_.x * _e7.g0_.x) + (_e11.g1_.y * _e14.g0_.y)) + (_e19.g1_.z * _e22.g0_.z)) + (_e27.g1_.w * _e30.g0_.w)));
}

fn point_and_plane_line_geometric_product(self_1318: PointAndPlane, other_1114: Line) -> PointAndPlane {
    var self_1319: PointAndPlane;
    var other_1115: Line;

    self_1319 = self_1318;
    other_1115 = other_1114;
    let _e4: PointAndPlane = self_1319;
    let _e8: Line = other_1115;
    let _e11: Line = other_1115;
    let _e14: Line = other_1115;
    let _e17: Line = other_1115;
    let _e30: PointAndPlane = self_1319;
    let _e34: Line = other_1115;
    let _e37: Line = other_1115;
    let _e40: Line = other_1115;
    let _e43: Line = other_1115;
    let _e57: PointAndPlane = self_1319;
    let _e61: Line = other_1115;
    let _e64: Line = other_1115;
    let _e67: Line = other_1115;
    let _e70: Line = other_1115;
    let _e84: PointAndPlane = self_1319;
    let _e88: Line = other_1115;
    let _e91: Line = other_1115;
    let _e94: Line = other_1115;
    let _e97: Line = other_1115;
    let _e112: PointAndPlane = self_1319;
    let _e116: Line = other_1115;
    let _e119: Line = other_1115;
    let _e122: Line = other_1115;
    let _e125: Line = other_1115;
    let _e138: PointAndPlane = self_1319;
    let _e142: Line = other_1115;
    let _e145: Line = other_1115;
    let _e148: Line = other_1115;
    let _e151: Line = other_1115;
    let _e164: PointAndPlane = self_1319;
    let _e168: Line = other_1115;
    let _e171: Line = other_1115;
    let _e174: Line = other_1115;
    let _e177: Line = other_1115;
    let _e190: PointAndPlane = self_1319;
    let _e194: Line = other_1115;
    let _e197: Line = other_1115;
    let _e200: Line = other_1115;
    let _e203: Line = other_1115;
    let _e215: PointAndPlane = self_1319;
    let _e219: Line = other_1115;
    let _e222: Line = other_1115;
    let _e225: Line = other_1115;
    let _e228: Line = other_1115;
    let _e240: PointAndPlane = self_1319;
    let _e244: Line = other_1115;
    let _e247: Line = other_1115;
    let _e250: Line = other_1115;
    let _e253: Line = other_1115;
    let _e266: PointAndPlane = self_1319;
    let _e270: Line = other_1115;
    let _e273: Line = other_1115;
    let _e276: Line = other_1115;
    let _e279: Line = other_1115;
    let _e292: PointAndPlane = self_1319;
    let _e296: Line = other_1115;
    let _e299: Line = other_1115;
    let _e302: Line = other_1115;
    let _e305: Line = other_1115;
    let _e317: PointAndPlane = self_1319;
    let _e321: Line = other_1115;
    let _e324: Line = other_1115;
    let _e327: Line = other_1115;
    let _e330: Line = other_1115;
    let _e344: PointAndPlane = self_1319;
    let _e348: Line = other_1115;
    let _e351: Line = other_1115;
    let _e354: Line = other_1115;
    let _e357: Line = other_1115;
    let _e371: PointAndPlane = self_1319;
    let _e375: Line = other_1115;
    let _e378: Line = other_1115;
    let _e381: Line = other_1115;
    let _e384: Line = other_1115;
    let _e398: PointAndPlane = self_1319;
    let _e402: Line = other_1115;
    let _e405: Line = other_1115;
    let _e408: Line = other_1115;
    let _e411: Line = other_1115;
    return PointAndPlane((((((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.x, _e11.g0_.x, _e14.g1_.z, _e17.g1_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e30.g0_.z) * vec4<f32>(_e34.g0_.y, _e37.g1_.z, _e40.g0_.y, _e43.g1_.x)) * vec4<f32>(-(1.0), -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e57.g0_.w) * vec4<f32>(_e61.g0_.z, _e64.g1_.y, _e67.g1_.x, _e70.g0_.z)) * vec4<f32>(-(1.0), 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e84.g1_.x) * vec4<f32>(_e88.g1_.x, _e91.g1_.x, _e94.g1_.y, _e97.g1_.z)) * vec4<f32>(0.0, -(1.0), -(1.0), -(1.0)))) + ((vec4<f32>(_e112.g1_.y) * vec4<f32>(_e116.g1_.x, _e119.g1_.x, _e122.g0_.z, _e125.g0_.y)) * vec4<f32>(1.0, 0.0, -(1.0), 1.0))) + ((vec4<f32>(_e138.g1_.z) * vec4<f32>(_e142.g1_.y, _e145.g0_.z, _e148.g1_.y, _e151.g0_.x)) * vec4<f32>(1.0, 1.0, 0.0, -(1.0)))) + ((vec4<f32>(_e164.g1_.w) * vec4<f32>(_e168.g1_.z, _e171.g0_.y, _e174.g0_.x, _e177.g1_.z)) * vec4<f32>(1.0, -(1.0), 1.0, 0.0))) + ((vec4<f32>(_e190.g0_.x) * vec4<f32>(_e194.g0_.x, _e197.g0_.x, _e200.g0_.y, _e203.g0_.z)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))), (((((((((vec4<f32>(_e215.g0_.y) * vec4<f32>(_e219.g1_.x, _e222.g1_.x, _e225.g0_.z, _e228.g0_.y)) * vec4<f32>(1.0, 0.0, -(1.0), 1.0)) + ((vec4<f32>(_e240.g0_.z) * vec4<f32>(_e244.g1_.y, _e247.g0_.z, _e250.g1_.y, _e253.g0_.x)) * vec4<f32>(1.0, 1.0, 0.0, -(1.0)))) + ((vec4<f32>(_e266.g0_.w) * vec4<f32>(_e270.g1_.z, _e273.g0_.y, _e276.g0_.x, _e279.g1_.z)) * vec4<f32>(1.0, -(1.0), 1.0, 0.0))) + ((vec4<f32>(_e292.g1_.x) * vec4<f32>(_e296.g0_.x, _e299.g0_.x, _e302.g0_.y, _e305.g0_.z)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))) + ((vec4<f32>(_e317.g1_.y) * vec4<f32>(_e321.g0_.x, _e324.g0_.x, _e327.g1_.z, _e330.g1_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e344.g1_.z) * vec4<f32>(_e348.g0_.y, _e351.g1_.z, _e354.g0_.y, _e357.g1_.x)) * vec4<f32>(-(1.0), -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e371.g1_.w) * vec4<f32>(_e375.g0_.z, _e378.g1_.y, _e381.g1_.x, _e384.g0_.z)) * vec4<f32>(-(1.0), 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e398.g0_.x) * vec4<f32>(_e402.g1_.x, _e405.g1_.x, _e408.g1_.y, _e411.g1_.z)) * vec4<f32>(0.0, -(1.0), -(1.0), -(1.0)))));
}

fn point_and_plane_line_regressive_product(self_1320: PointAndPlane, other_1116: Line) -> Plane {
    var self_1321: PointAndPlane;
    var other_1117: Line;

    self_1321 = self_1320;
    other_1117 = other_1116;
    let _e4: PointAndPlane = self_1321;
    let _e8: Line = other_1117;
    let _e11: Line = other_1117;
    let _e14: Line = other_1117;
    let _e17: Line = other_1117;
    let _e30: PointAndPlane = self_1321;
    let _e34: Line = other_1117;
    let _e37: Line = other_1117;
    let _e40: Line = other_1117;
    let _e43: Line = other_1117;
    let _e57: PointAndPlane = self_1321;
    let _e61: Line = other_1117;
    let _e64: Line = other_1117;
    let _e67: Line = other_1117;
    let _e70: Line = other_1117;
    let _e84: PointAndPlane = self_1321;
    let _e88: Line = other_1117;
    let _e91: Line = other_1117;
    let _e94: Line = other_1117;
    let _e97: Line = other_1117;
    return Plane((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g0_.x, _e11.g0_.x, _e14.g1_.z, _e17.g1_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e30.g0_.z) * vec4<f32>(_e34.g0_.y, _e37.g1_.z, _e40.g0_.y, _e43.g1_.x)) * vec4<f32>(-(1.0), -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e57.g0_.w) * vec4<f32>(_e61.g0_.z, _e64.g1_.y, _e67.g1_.x, _e70.g0_.z)) * vec4<f32>(-(1.0), 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e84.g0_.x) * vec4<f32>(_e88.g0_.x, _e91.g0_.x, _e94.g0_.y, _e97.g0_.z)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn point_and_plane_line_outer_product(self_1322: PointAndPlane, other_1118: Line) -> Point {
    var self_1323: PointAndPlane;
    var other_1119: Line;

    self_1323 = self_1322;
    other_1119 = other_1118;
    let _e4: PointAndPlane = self_1323;
    let _e8: Line = other_1119;
    let _e11: Line = other_1119;
    let _e14: Line = other_1119;
    let _e17: Line = other_1119;
    let _e29: PointAndPlane = self_1323;
    let _e33: Line = other_1119;
    let _e36: Line = other_1119;
    let _e39: Line = other_1119;
    let _e42: Line = other_1119;
    let _e55: PointAndPlane = self_1323;
    let _e59: Line = other_1119;
    let _e62: Line = other_1119;
    let _e65: Line = other_1119;
    let _e68: Line = other_1119;
    let _e81: PointAndPlane = self_1323;
    let _e85: Line = other_1119;
    let _e88: Line = other_1119;
    let _e91: Line = other_1119;
    let _e94: Line = other_1119;
    return Point((((((vec4<f32>(_e4.g1_.y) * vec4<f32>(_e8.g1_.x, _e11.g1_.x, _e14.g0_.z, _e17.g0_.y)) * vec4<f32>(1.0, 0.0, -(1.0), 1.0)) + ((vec4<f32>(_e29.g1_.z) * vec4<f32>(_e33.g1_.y, _e36.g0_.z, _e39.g1_.y, _e42.g0_.x)) * vec4<f32>(1.0, 1.0, 0.0, -(1.0)))) + ((vec4<f32>(_e55.g1_.w) * vec4<f32>(_e59.g1_.z, _e62.g0_.y, _e65.g0_.x, _e68.g1_.z)) * vec4<f32>(1.0, -(1.0), 1.0, 0.0))) + ((vec4<f32>(_e81.g1_.x) * vec4<f32>(_e85.g1_.x, _e88.g1_.x, _e91.g1_.y, _e94.g1_.z)) * vec4<f32>(0.0, -(1.0), -(1.0), -(1.0)))));
}

fn point_and_plane_line_inner_product(self_1324: PointAndPlane, other_1120: Line) -> Plane {
    var self_1325: PointAndPlane;
    var other_1121: Line;

    self_1325 = self_1324;
    other_1121 = other_1120;
    let _e4: PointAndPlane = self_1325;
    let _e8: Line = other_1121;
    let _e11: Line = other_1121;
    let _e14: Line = other_1121;
    let _e17: Line = other_1121;
    let _e29: PointAndPlane = self_1325;
    let _e33: Line = other_1121;
    let _e36: Line = other_1121;
    let _e39: Line = other_1121;
    let _e42: Line = other_1121;
    let _e55: PointAndPlane = self_1325;
    let _e59: Line = other_1121;
    let _e62: Line = other_1121;
    let _e65: Line = other_1121;
    let _e68: Line = other_1121;
    let _e81: PointAndPlane = self_1325;
    let _e85: Line = other_1121;
    let _e88: Line = other_1121;
    let _e91: Line = other_1121;
    let _e94: Line = other_1121;
    let _e106: PointAndPlane = self_1325;
    let _e110: Line = other_1121;
    let _e113: Line = other_1121;
    let _e116: Line = other_1121;
    let _e119: Line = other_1121;
    let _e133: PointAndPlane = self_1325;
    let _e137: Line = other_1121;
    let _e140: Line = other_1121;
    let _e143: Line = other_1121;
    let _e146: Line = other_1121;
    let _e160: PointAndPlane = self_1325;
    let _e164: Line = other_1121;
    let _e167: Line = other_1121;
    let _e170: Line = other_1121;
    let _e173: Line = other_1121;
    let _e187: PointAndPlane = self_1325;
    let _e191: Line = other_1121;
    let _e194: Line = other_1121;
    let _e197: Line = other_1121;
    let _e200: Line = other_1121;
    return Plane((((((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g1_.x, _e11.g1_.x, _e14.g0_.z, _e17.g0_.y)) * vec4<f32>(1.0, 0.0, -(1.0), 1.0)) + ((vec4<f32>(_e29.g0_.z) * vec4<f32>(_e33.g1_.y, _e36.g0_.z, _e39.g1_.y, _e42.g0_.x)) * vec4<f32>(1.0, 1.0, 0.0, -(1.0)))) + ((vec4<f32>(_e55.g0_.w) * vec4<f32>(_e59.g1_.z, _e62.g0_.y, _e65.g0_.x, _e68.g1_.z)) * vec4<f32>(1.0, -(1.0), 1.0, 0.0))) + ((vec4<f32>(_e81.g1_.x) * vec4<f32>(_e85.g0_.x, _e88.g0_.x, _e91.g0_.y, _e94.g0_.z)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))) + ((vec4<f32>(_e106.g1_.y) * vec4<f32>(_e110.g0_.x, _e113.g0_.x, _e116.g1_.z, _e119.g1_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e133.g1_.z) * vec4<f32>(_e137.g0_.y, _e140.g1_.z, _e143.g0_.y, _e146.g1_.x)) * vec4<f32>(-(1.0), -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e160.g1_.w) * vec4<f32>(_e164.g0_.z, _e167.g1_.y, _e170.g1_.x, _e173.g0_.z)) * vec4<f32>(-(1.0), 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e187.g0_.x) * vec4<f32>(_e191.g1_.x, _e194.g1_.x, _e197.g1_.y, _e200.g1_.z)) * vec4<f32>(0.0, -(1.0), -(1.0), -(1.0)))));
}

fn point_and_plane_line_left_contraction(self_1326: PointAndPlane, other_1122: Line) -> Plane {
    var self_1327: PointAndPlane;
    var other_1123: Line;

    self_1327 = self_1326;
    other_1123 = other_1122;
    let _e4: PointAndPlane = self_1327;
    let _e8: Line = other_1123;
    let _e11: Line = other_1123;
    let _e14: Line = other_1123;
    let _e17: Line = other_1123;
    let _e30: PointAndPlane = self_1327;
    let _e34: Line = other_1123;
    let _e37: Line = other_1123;
    let _e40: Line = other_1123;
    let _e43: Line = other_1123;
    let _e57: PointAndPlane = self_1327;
    let _e61: Line = other_1123;
    let _e64: Line = other_1123;
    let _e67: Line = other_1123;
    let _e70: Line = other_1123;
    let _e84: PointAndPlane = self_1327;
    let _e88: Line = other_1123;
    let _e91: Line = other_1123;
    let _e94: Line = other_1123;
    let _e97: Line = other_1123;
    return Plane((((((vec4<f32>(_e4.g1_.y) * vec4<f32>(_e8.g0_.x, _e11.g0_.x, _e14.g1_.z, _e17.g1_.y)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e30.g1_.z) * vec4<f32>(_e34.g0_.y, _e37.g1_.z, _e40.g0_.y, _e43.g1_.x)) * vec4<f32>(-(1.0), -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e57.g1_.w) * vec4<f32>(_e61.g0_.z, _e64.g1_.y, _e67.g1_.x, _e70.g0_.z)) * vec4<f32>(-(1.0), 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e84.g1_.x) * vec4<f32>(_e88.g0_.x, _e91.g0_.x, _e94.g0_.y, _e97.g0_.z)) * vec4<f32>(0.0, 1.0, 1.0, 1.0))));
}

fn point_and_plane_line_right_contraction(self_1328: PointAndPlane, other_1124: Line) -> Plane {
    var self_1329: PointAndPlane;
    var other_1125: Line;

    self_1329 = self_1328;
    other_1125 = other_1124;
    let _e4: PointAndPlane = self_1329;
    let _e8: Line = other_1125;
    let _e11: Line = other_1125;
    let _e14: Line = other_1125;
    let _e17: Line = other_1125;
    let _e29: PointAndPlane = self_1329;
    let _e33: Line = other_1125;
    let _e36: Line = other_1125;
    let _e39: Line = other_1125;
    let _e42: Line = other_1125;
    let _e55: PointAndPlane = self_1329;
    let _e59: Line = other_1125;
    let _e62: Line = other_1125;
    let _e65: Line = other_1125;
    let _e68: Line = other_1125;
    let _e81: PointAndPlane = self_1329;
    let _e85: Line = other_1125;
    let _e88: Line = other_1125;
    let _e91: Line = other_1125;
    let _e94: Line = other_1125;
    return Plane((((((vec4<f32>(_e4.g0_.y) * vec4<f32>(_e8.g1_.x, _e11.g1_.x, _e14.g0_.z, _e17.g0_.y)) * vec4<f32>(1.0, 0.0, -(1.0), 1.0)) + ((vec4<f32>(_e29.g0_.z) * vec4<f32>(_e33.g1_.y, _e36.g0_.z, _e39.g1_.y, _e42.g0_.x)) * vec4<f32>(1.0, 1.0, 0.0, -(1.0)))) + ((vec4<f32>(_e55.g0_.w) * vec4<f32>(_e59.g1_.z, _e62.g0_.y, _e65.g0_.x, _e68.g1_.z)) * vec4<f32>(1.0, -(1.0), 1.0, 0.0))) + ((vec4<f32>(_e81.g0_.x) * vec4<f32>(_e85.g1_.x, _e88.g1_.x, _e91.g1_.y, _e94.g1_.z)) * vec4<f32>(0.0, -(1.0), -(1.0), -(1.0)))));
}

fn point_and_plane_translator_geometric_product(self_1330: PointAndPlane, other_1126: Translator) -> PointAndPlane {
    var self_1331: PointAndPlane;
    var other_1127: Translator;

    self_1331 = self_1330;
    other_1127 = other_1126;
    let _e4: PointAndPlane = self_1331;
    let _e8: Translator = other_1127;
    let _e11: PointAndPlane = self_1331;
    let _e15: Translator = other_1127;
    let _e27: PointAndPlane = self_1331;
    let _e31: Translator = other_1127;
    let _e43: PointAndPlane = self_1331;
    let _e47: Translator = other_1127;
    let _e59: PointAndPlane = self_1331;
    let _e63: Translator = other_1127;
    let _e75: PointAndPlane = self_1331;
    let _e78: PointAndPlane = self_1331;
    let _e81: PointAndPlane = self_1331;
    let _e84: PointAndPlane = self_1331;
    let _e88: Translator = other_1127;
    let _e101: PointAndPlane = self_1331;
    let _e105: Translator = other_1127;
    let _e116: PointAndPlane = self_1331;
    let _e120: Translator = other_1127;
    let _e132: PointAndPlane = self_1331;
    let _e136: Translator = other_1127;
    let _e140: PointAndPlane = self_1331;
    let _e144: Translator = other_1127;
    let _e156: PointAndPlane = self_1331;
    let _e160: Translator = other_1127;
    let _e172: PointAndPlane = self_1331;
    let _e175: PointAndPlane = self_1331;
    let _e178: PointAndPlane = self_1331;
    let _e181: PointAndPlane = self_1331;
    let _e185: Translator = other_1127;
    return PointAndPlane(((((((vec4<f32>(_e4.g0_.x) * _e8.g0_) + ((vec4<f32>(_e11.g0_.z) * _e15.g0_.zzxz) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0))) + ((vec4<f32>(_e27.g0_.w) * _e31.g0_.wwwx) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))) + ((vec4<f32>(_e43.g1_.z) * _e47.g0_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0)))) + ((vec4<f32>(_e59.g1_.w) * _e63.g0_.zzyz) * vec4<f32>(0.0, -(1.0), 1.0, 0.0))) + ((vec4<f32>(_e75.g0_.y, _e78.g0_.y, _e81.g1_.y, _e84.g1_.y) * _e88.g0_.yxwz) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))), (((((((vec4<f32>(_e101.g0_.z) * _e105.g0_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e116.g0_.w) * _e120.g0_.zzyz) * vec4<f32>(0.0, -(1.0), 1.0, 0.0))) + (vec4<f32>(_e132.g1_.x) * _e136.g0_)) + ((vec4<f32>(_e140.g1_.z) * _e144.g0_.zzxz) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0))) + ((vec4<f32>(_e156.g1_.w) * _e160.g0_.wwwx) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))) + ((vec4<f32>(_e172.g1_.y, _e175.g1_.y, _e178.g0_.y, _e181.g0_.y) * _e185.g0_.yxwz) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))));
}

fn point_and_plane_translator_regressive_product(self_1332: PointAndPlane, other_1128: Translator) -> Plane {
    var self_1333: PointAndPlane;
    var other_1129: Translator;

    self_1333 = self_1332;
    other_1129 = other_1128;
    let _e4: PointAndPlane = self_1333;
    let _e8: Translator = other_1129;
    let _e20: PointAndPlane = self_1333;
    let _e24: Translator = other_1129;
    let _e37: PointAndPlane = self_1333;
    let _e40: Translator = other_1129;
    return Plane(((((vec4<f32>(_e4.g0_.z) * vec4<f32>(_e8.g0_.z)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0)) + ((vec4<f32>(_e20.g0_.w) * vec4<f32>(_e24.g0_.w)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((_e37.g0_.yxxx * _e40.g0_.yyzw) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0))));
}

fn point_and_plane_translator_outer_product(self_1334: PointAndPlane, other_1130: Translator) -> PointAndPlane {
    var self_1335: PointAndPlane;
    var other_1131: Translator;

    self_1335 = self_1334;
    other_1131 = other_1130;
    let _e4: PointAndPlane = self_1335;
    let _e8: Translator = other_1131;
    let _e19: PointAndPlane = self_1335;
    let _e23: Translator = other_1131;
    let _e35: PointAndPlane = self_1335;
    let _e39: Translator = other_1131;
    let _e51: PointAndPlane = self_1335;
    let _e53: Translator = other_1131;
    let _e59: PointAndPlane = self_1335;
    let _e61: Translator = other_1131;
    return PointAndPlane((((((vec4<f32>(_e4.g1_.y) * _e8.g0_.wwwz) * vec4<f32>(0.0, 0.0, -(1.0), 1.0)) + ((vec4<f32>(_e19.g1_.z) * _e23.g0_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0)))) + ((vec4<f32>(_e35.g1_.w) * _e39.g0_.zzyz) * vec4<f32>(0.0, -(1.0), 1.0, 0.0))) + (_e51.g0_ * vec4<f32>(_e53.g0_.x))), (_e59.g1_ * vec4<f32>(_e61.g0_.x)));
}

fn point_and_plane_translator_inner_product(self_1336: PointAndPlane, other_1132: Translator) -> PointAndPlane {
    var self_1337: PointAndPlane;
    var other_1133: Translator;

    self_1337 = self_1336;
    other_1133 = other_1132;
    let _e4: PointAndPlane = self_1337;
    let _e6: Translator = other_1133;
    let _e11: PointAndPlane = self_1337;
    let _e15: Translator = other_1133;
    let _e26: PointAndPlane = self_1337;
    let _e30: Translator = other_1133;
    let _e42: PointAndPlane = self_1337;
    let _e46: Translator = other_1133;
    let _e50: PointAndPlane = self_1337;
    let _e54: Translator = other_1133;
    let _e66: PointAndPlane = self_1337;
    let _e70: Translator = other_1133;
    let _e82: PointAndPlane = self_1337;
    let _e85: PointAndPlane = self_1337;
    let _e88: PointAndPlane = self_1337;
    let _e91: PointAndPlane = self_1337;
    let _e95: Translator = other_1133;
    return PointAndPlane((_e4.g0_ * vec4<f32>(_e6.g0_.x)), (((((((vec4<f32>(_e11.g0_.z) * _e15.g0_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e26.g0_.w) * _e30.g0_.zzyz) * vec4<f32>(0.0, -(1.0), 1.0, 0.0))) + (vec4<f32>(_e42.g1_.x) * _e46.g0_)) + ((vec4<f32>(_e50.g1_.z) * _e54.g0_.zzxz) * vec4<f32>(-(1.0), 0.0, 1.0, 0.0))) + ((vec4<f32>(_e66.g1_.w) * _e70.g0_.wwwx) * vec4<f32>(-(1.0), 0.0, 0.0, 1.0))) + ((vec4<f32>(_e82.g1_.y, _e85.g1_.y, _e88.g0_.y, _e91.g0_.y) * _e95.g0_.yxwz) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))));
}

fn point_and_plane_translator_left_contraction(self_1338: PointAndPlane, other_1134: Translator) -> Plane {
    var self_1339: PointAndPlane;
    var other_1135: Translator;

    self_1339 = self_1338;
    other_1135 = other_1134;
    let _e4: PointAndPlane = self_1339;
    let _e8: Translator = other_1135;
    let _e20: PointAndPlane = self_1339;
    let _e24: Translator = other_1135;
    let _e37: PointAndPlane = self_1339;
    let _e40: Translator = other_1135;
    return Plane(((((vec4<f32>(_e4.g1_.z) * vec4<f32>(_e8.g0_.z)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0)) + ((vec4<f32>(_e20.g1_.w) * vec4<f32>(_e24.g0_.w)) * vec4<f32>(-(1.0), 0.0, 0.0, 0.0))) + ((_e37.g1_.yxxx * _e40.g0_.yyzw) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0))));
}

fn point_and_plane_translator_right_contraction(self_1340: PointAndPlane, other_1136: Translator) -> PointAndPlane {
    var self_1341: PointAndPlane;
    var other_1137: Translator;

    self_1341 = self_1340;
    other_1137 = other_1136;
    let _e4: PointAndPlane = self_1341;
    let _e6: Translator = other_1137;
    let _e11: PointAndPlane = self_1341;
    let _e15: Translator = other_1137;
    let _e26: PointAndPlane = self_1341;
    let _e30: Translator = other_1137;
    let _e42: PointAndPlane = self_1341;
    let _e46: Translator = other_1137;
    let _e58: PointAndPlane = self_1341;
    let _e62: Translator = other_1137;
    let _e74: PointAndPlane = self_1341;
    let _e77: PointAndPlane = self_1341;
    let _e80: PointAndPlane = self_1341;
    let _e83: PointAndPlane = self_1341;
    let _e87: Translator = other_1137;
    return PointAndPlane((_e4.g0_ * vec4<f32>(_e6.g0_.x)), ((((((vec4<f32>(_e11.g0_.z) * _e15.g0_.wwwy) * vec4<f32>(0.0, 1.0, 0.0, -(1.0))) + ((vec4<f32>(_e26.g0_.w) * _e30.g0_.zzyz) * vec4<f32>(0.0, -(1.0), 1.0, 0.0))) + ((vec4<f32>(_e42.g1_.z) * vec4<f32>(_e46.g0_.x)) * vec4<f32>(0.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e58.g1_.w) * vec4<f32>(_e62.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e74.g1_.x, _e77.g1_.y, _e80.g0_.y, _e83.g0_.y) * _e87.g0_.xxwz) * vec4<f32>(1.0, 1.0, -(1.0), 1.0))));
}

fn point_and_plane_motor_add(self_1342: PointAndPlane, other_1138: Motor) -> MultiVector {
    var self_1343: PointAndPlane;
    var other_1139: Motor;

    self_1343 = self_1342;
    other_1139 = other_1138;
    let _e4: Motor = other_1139;
    let _e6: PointAndPlane = self_1343;
    let _e9: PointAndPlane = self_1343;
    let _e12: PointAndPlane = self_1343;
    let _e15: PointAndPlane = self_1343;
    let _e19: PointAndPlane = self_1343;
    let _e22: PointAndPlane = self_1343;
    let _e25: PointAndPlane = self_1343;
    let _e28: PointAndPlane = self_1343;
    let _e32: Motor = other_1139;
    return MultiVector(_e4.g0_, vec4<f32>(_e6.g1_.x, _e9.g0_.y, _e12.g0_.z, _e15.g0_.w), vec4<f32>(_e19.g0_.x, _e22.g1_.y, _e25.g1_.z, _e28.g1_.w), _e32.g1_);
}

fn point_and_plane_motor_sub(self_1344: PointAndPlane, other_1140: Motor) -> MultiVector {
    var self_1345: PointAndPlane;
    var other_1141: Motor;

    self_1345 = self_1344;
    other_1141 = other_1140;
    let _e6: Motor = other_1141;
    let _e9: PointAndPlane = self_1345;
    let _e12: PointAndPlane = self_1345;
    let _e15: PointAndPlane = self_1345;
    let _e18: PointAndPlane = self_1345;
    let _e22: PointAndPlane = self_1345;
    let _e25: PointAndPlane = self_1345;
    let _e28: PointAndPlane = self_1345;
    let _e31: PointAndPlane = self_1345;
    let _e37: Motor = other_1141;
    return MultiVector((vec4<f32>(0.0) - _e6.g0_), vec4<f32>(_e9.g1_.x, _e12.g0_.y, _e15.g0_.z, _e18.g0_.w), vec4<f32>(_e22.g0_.x, _e25.g1_.y, _e28.g1_.z, _e31.g1_.w), (vec4<f32>(0.0) - _e37.g1_));
}

fn point_and_plane_motor_geometric_product(self_1346: PointAndPlane, other_1142: Motor) -> PointAndPlane {
    var self_1347: PointAndPlane;
    var other_1143: Motor;

    self_1347 = self_1346;
    other_1143 = other_1142;
    let _e4: PointAndPlane = self_1347;
    let _e8: Motor = other_1143;
    let _e11: Motor = other_1143;
    let _e14: Motor = other_1143;
    let _e17: Motor = other_1143;
    let _e22: PointAndPlane = self_1347;
    let _e26: Motor = other_1143;
    let _e29: Motor = other_1143;
    let _e32: Motor = other_1143;
    let _e35: Motor = other_1143;
    let _e49: PointAndPlane = self_1347;
    let _e53: Motor = other_1143;
    let _e56: Motor = other_1143;
    let _e59: Motor = other_1143;
    let _e62: Motor = other_1143;
    let _e76: PointAndPlane = self_1347;
    let _e80: Motor = other_1143;
    let _e83: Motor = other_1143;
    let _e86: Motor = other_1143;
    let _e89: Motor = other_1143;
    let _e103: PointAndPlane = self_1347;
    let _e107: Motor = other_1143;
    let _e110: Motor = other_1143;
    let _e113: Motor = other_1143;
    let _e116: Motor = other_1143;
    let _e131: PointAndPlane = self_1347;
    let _e135: Motor = other_1143;
    let _e138: Motor = other_1143;
    let _e141: Motor = other_1143;
    let _e144: Motor = other_1143;
    let _e157: PointAndPlane = self_1347;
    let _e161: Motor = other_1143;
    let _e164: Motor = other_1143;
    let _e167: Motor = other_1143;
    let _e170: Motor = other_1143;
    let _e183: PointAndPlane = self_1347;
    let _e187: Motor = other_1143;
    let _e190: Motor = other_1143;
    let _e193: Motor = other_1143;
    let _e196: Motor = other_1143;
    let _e209: PointAndPlane = self_1347;
    let _e213: Motor = other_1143;
    let _e216: Motor = other_1143;
    let _e219: Motor = other_1143;
    let _e222: Motor = other_1143;
    let _e236: PointAndPlane = self_1347;
    let _e240: Motor = other_1143;
    let _e243: Motor = other_1143;
    let _e246: Motor = other_1143;
    let _e249: Motor = other_1143;
    let _e262: PointAndPlane = self_1347;
    let _e266: Motor = other_1143;
    let _e269: Motor = other_1143;
    let _e272: Motor = other_1143;
    let _e275: Motor = other_1143;
    let _e288: PointAndPlane = self_1347;
    let _e292: Motor = other_1143;
    let _e295: Motor = other_1143;
    let _e298: Motor = other_1143;
    let _e301: Motor = other_1143;
    let _e314: PointAndPlane = self_1347;
    let _e318: Motor = other_1143;
    let _e321: Motor = other_1143;
    let _e324: Motor = other_1143;
    let _e327: Motor = other_1143;
    let _e333: PointAndPlane = self_1347;
    let _e337: Motor = other_1143;
    let _e340: Motor = other_1143;
    let _e343: Motor = other_1143;
    let _e346: Motor = other_1143;
    let _e360: PointAndPlane = self_1347;
    let _e364: Motor = other_1143;
    let _e367: Motor = other_1143;
    let _e370: Motor = other_1143;
    let _e373: Motor = other_1143;
    let _e387: PointAndPlane = self_1347;
    let _e391: Motor = other_1143;
    let _e394: Motor = other_1143;
    let _e397: Motor = other_1143;
    let _e400: Motor = other_1143;
    return PointAndPlane(((((((((vec4<f32>(_e4.g0_.x) * vec4<f32>(_e8.g0_.x, _e11.g1_.y, _e14.g1_.z, _e17.g1_.w)) + ((vec4<f32>(_e22.g0_.y) * vec4<f32>(_e26.g1_.y, _e29.g0_.x, _e32.g0_.w, _e35.g0_.z)) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e49.g0_.z) * vec4<f32>(_e53.g1_.z, _e56.g0_.w, _e59.g0_.x, _e62.g0_.y)) * vec4<f32>(-(1.0), -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e76.g0_.w) * vec4<f32>(_e80.g1_.w, _e83.g0_.z, _e86.g0_.y, _e89.g0_.x)) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))) + ((vec4<f32>(_e103.g1_.x) * vec4<f32>(_e107.g1_.x, _e110.g0_.y, _e113.g0_.z, _e116.g0_.w)) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0)))) + ((vec4<f32>(_e131.g1_.y) * vec4<f32>(_e135.g0_.y, _e138.g1_.x, _e141.g1_.w, _e144.g1_.z)) * vec4<f32>(1.0, 1.0, -(1.0), 1.0))) + ((vec4<f32>(_e157.g1_.z) * vec4<f32>(_e161.g0_.z, _e164.g1_.w, _e167.g1_.x, _e170.g1_.y)) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e183.g1_.w) * vec4<f32>(_e187.g0_.w, _e190.g1_.z, _e193.g1_.y, _e196.g1_.x)) * vec4<f32>(1.0, -(1.0), 1.0, 1.0))), (((((((((vec4<f32>(_e209.g0_.x) * vec4<f32>(_e213.g1_.x, _e216.g0_.y, _e219.g0_.z, _e222.g0_.w)) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))) + ((vec4<f32>(_e236.g0_.y) * vec4<f32>(_e240.g0_.y, _e243.g1_.x, _e246.g1_.w, _e249.g1_.z)) * vec4<f32>(1.0, 1.0, -(1.0), 1.0))) + ((vec4<f32>(_e262.g0_.z) * vec4<f32>(_e266.g0_.z, _e269.g1_.w, _e272.g1_.x, _e275.g1_.y)) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e288.g0_.w) * vec4<f32>(_e292.g0_.w, _e295.g1_.z, _e298.g1_.y, _e301.g1_.x)) * vec4<f32>(1.0, -(1.0), 1.0, 1.0))) + (vec4<f32>(_e314.g1_.x) * vec4<f32>(_e318.g0_.x, _e321.g1_.y, _e324.g1_.z, _e327.g1_.w))) + ((vec4<f32>(_e333.g1_.y) * vec4<f32>(_e337.g1_.y, _e340.g0_.x, _e343.g0_.w, _e346.g0_.z)) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e360.g1_.z) * vec4<f32>(_e364.g1_.z, _e367.g0_.w, _e370.g0_.x, _e373.g0_.y)) * vec4<f32>(-(1.0), -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e387.g1_.w) * vec4<f32>(_e391.g1_.w, _e394.g0_.z, _e397.g0_.y, _e400.g0_.x)) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))));
}

fn point_and_plane_motor_regressive_product(self_1348: PointAndPlane, other_1144: Motor) -> PointAndPlane {
    var self_1349: PointAndPlane;
    var other_1145: Motor;

    self_1349 = self_1348;
    other_1145 = other_1144;
    let _e4: PointAndPlane = self_1349;
    let _e6: Motor = other_1145;
    let _e11: PointAndPlane = self_1349;
    let _e15: Motor = other_1145;
    let _e18: Motor = other_1145;
    let _e21: Motor = other_1145;
    let _e24: Motor = other_1145;
    let _e37: PointAndPlane = self_1349;
    let _e41: Motor = other_1145;
    let _e44: Motor = other_1145;
    let _e47: Motor = other_1145;
    let _e50: Motor = other_1145;
    let _e64: PointAndPlane = self_1349;
    let _e68: Motor = other_1145;
    let _e71: Motor = other_1145;
    let _e74: Motor = other_1145;
    let _e77: Motor = other_1145;
    let _e91: PointAndPlane = self_1349;
    let _e95: Motor = other_1145;
    let _e107: PointAndPlane = self_1349;
    let _e111: Motor = other_1145;
    let _e123: PointAndPlane = self_1349;
    let _e127: Motor = other_1145;
    let _e139: PointAndPlane = self_1349;
    let _e142: PointAndPlane = self_1349;
    let _e145: PointAndPlane = self_1349;
    let _e148: PointAndPlane = self_1349;
    let _e152: Motor = other_1145;
    return PointAndPlane((_e4.g0_ * vec4<f32>(_e6.g1_.x)), ((((((((vec4<f32>(_e11.g0_.y) * vec4<f32>(_e15.g1_.y, _e18.g1_.y, _e21.g0_.w, _e24.g0_.z)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0))) + ((vec4<f32>(_e37.g0_.z) * vec4<f32>(_e41.g1_.z, _e44.g0_.w, _e47.g1_.z, _e50.g0_.y)) * vec4<f32>(-(1.0), -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e64.g0_.w) * vec4<f32>(_e68.g1_.w, _e71.g0_.z, _e74.g0_.y, _e77.g1_.w)) * vec4<f32>(-(1.0), 1.0, -(1.0), 0.0))) + ((vec4<f32>(_e91.g1_.y) * vec4<f32>(_e95.g1_.x)) * vec4<f32>(0.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e107.g1_.z) * vec4<f32>(_e111.g1_.x)) * vec4<f32>(0.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e123.g1_.w) * vec4<f32>(_e127.g1_.x)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + (vec4<f32>(_e139.g1_.x, _e142.g0_.x, _e145.g0_.x, _e148.g0_.x) * _e152.g1_)));
}

fn point_and_plane_motor_outer_product(self_1350: PointAndPlane, other_1146: Motor) -> PointAndPlane {
    var self_1351: PointAndPlane;
    var other_1147: Motor;

    self_1351 = self_1350;
    other_1147 = other_1146;
    let _e4: PointAndPlane = self_1351;
    let _e8: Motor = other_1147;
    let _e21: PointAndPlane = self_1351;
    let _e25: Motor = other_1147;
    let _e28: Motor = other_1147;
    let _e31: Motor = other_1147;
    let _e34: Motor = other_1147;
    let _e47: PointAndPlane = self_1351;
    let _e51: Motor = other_1147;
    let _e54: Motor = other_1147;
    let _e57: Motor = other_1147;
    let _e60: Motor = other_1147;
    let _e73: PointAndPlane = self_1351;
    let _e77: Motor = other_1147;
    let _e80: Motor = other_1147;
    let _e83: Motor = other_1147;
    let _e86: Motor = other_1147;
    let _e99: PointAndPlane = self_1351;
    let _e101: Motor = other_1147;
    let _e107: PointAndPlane = self_1351;
    let _e109: Motor = other_1147;
    return PointAndPlane(((((((vec4<f32>(_e4.g1_.x) * _e8.g0_.yyzw) * vec4<f32>(0.0, -(1.0), -(1.0), -(1.0))) + ((vec4<f32>(_e21.g1_.y) * vec4<f32>(_e25.g0_.y, _e28.g0_.y, _e31.g1_.w, _e34.g1_.z)) * vec4<f32>(1.0, 0.0, -(1.0), 1.0))) + ((vec4<f32>(_e47.g1_.z) * vec4<f32>(_e51.g0_.z, _e54.g1_.w, _e57.g0_.z, _e60.g1_.y)) * vec4<f32>(1.0, 1.0, 0.0, -(1.0)))) + ((vec4<f32>(_e73.g1_.w) * vec4<f32>(_e77.g0_.w, _e80.g1_.z, _e83.g1_.y, _e86.g0_.w)) * vec4<f32>(1.0, -(1.0), 1.0, 0.0))) + (_e99.g0_ * vec4<f32>(_e101.g0_.x))), (_e107.g1_ * vec4<f32>(_e109.g0_.x)));
}

fn point_and_plane_motor_inner_product(self_1352: PointAndPlane, other_1148: Motor) -> PointAndPlane {
    var self_1353: PointAndPlane;
    var other_1149: Motor;

    self_1353 = self_1352;
    other_1149 = other_1148;
    let _e4: PointAndPlane = self_1353;
    let _e8: Motor = other_1149;
    let _e19: PointAndPlane = self_1353;
    let _e23: Motor = other_1149;
    let _e35: PointAndPlane = self_1353;
    let _e39: Motor = other_1149;
    let _e51: PointAndPlane = self_1353;
    let _e55: Motor = other_1149;
    let _e67: PointAndPlane = self_1353;
    let _e69: Motor = other_1149;
    let _e75: PointAndPlane = self_1353;
    let _e79: Motor = other_1149;
    let _e82: Motor = other_1149;
    let _e85: Motor = other_1149;
    let _e88: Motor = other_1149;
    let _e102: PointAndPlane = self_1353;
    let _e106: Motor = other_1149;
    let _e109: Motor = other_1149;
    let _e112: Motor = other_1149;
    let _e115: Motor = other_1149;
    let _e128: PointAndPlane = self_1353;
    let _e132: Motor = other_1149;
    let _e135: Motor = other_1149;
    let _e138: Motor = other_1149;
    let _e141: Motor = other_1149;
    let _e154: PointAndPlane = self_1353;
    let _e158: Motor = other_1149;
    let _e161: Motor = other_1149;
    let _e164: Motor = other_1149;
    let _e167: Motor = other_1149;
    let _e180: PointAndPlane = self_1353;
    let _e184: Motor = other_1149;
    let _e187: Motor = other_1149;
    let _e190: Motor = other_1149;
    let _e193: Motor = other_1149;
    let _e199: PointAndPlane = self_1353;
    let _e203: Motor = other_1149;
    let _e206: Motor = other_1149;
    let _e209: Motor = other_1149;
    let _e212: Motor = other_1149;
    let _e226: PointAndPlane = self_1353;
    let _e230: Motor = other_1149;
    let _e233: Motor = other_1149;
    let _e236: Motor = other_1149;
    let _e239: Motor = other_1149;
    let _e253: PointAndPlane = self_1353;
    let _e257: Motor = other_1149;
    let _e260: Motor = other_1149;
    let _e263: Motor = other_1149;
    let _e266: Motor = other_1149;
    return PointAndPlane(((((((vec4<f32>(_e4.g1_.x) * vec4<f32>(_e8.g1_.x)) * vec4<f32>(1.0, 0.0, 0.0, 0.0)) + ((vec4<f32>(_e19.g1_.y) * vec4<f32>(_e23.g1_.x)) * vec4<f32>(0.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e35.g1_.z) * vec4<f32>(_e39.g1_.x)) * vec4<f32>(0.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e51.g1_.w) * vec4<f32>(_e55.g1_.x)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + (_e67.g0_ * vec4<f32>(_e69.g0_.x))), (((((((((vec4<f32>(_e75.g0_.x) * vec4<f32>(_e79.g1_.x, _e82.g0_.y, _e85.g0_.z, _e88.g0_.w)) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0))) + ((vec4<f32>(_e102.g0_.y) * vec4<f32>(_e106.g0_.y, _e109.g1_.x, _e112.g1_.w, _e115.g1_.z)) * vec4<f32>(1.0, 1.0, -(1.0), 1.0))) + ((vec4<f32>(_e128.g0_.z) * vec4<f32>(_e132.g0_.z, _e135.g1_.w, _e138.g1_.x, _e141.g1_.y)) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e154.g0_.w) * vec4<f32>(_e158.g0_.w, _e161.g1_.z, _e164.g1_.y, _e167.g1_.x)) * vec4<f32>(1.0, -(1.0), 1.0, 1.0))) + (vec4<f32>(_e180.g1_.x) * vec4<f32>(_e184.g0_.x, _e187.g1_.y, _e190.g1_.z, _e193.g1_.w))) + ((vec4<f32>(_e199.g1_.y) * vec4<f32>(_e203.g1_.y, _e206.g0_.x, _e209.g0_.w, _e212.g0_.z)) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e226.g1_.z) * vec4<f32>(_e230.g1_.z, _e233.g0_.w, _e236.g0_.x, _e239.g0_.y)) * vec4<f32>(-(1.0), -(1.0), 1.0, 1.0))) + ((vec4<f32>(_e253.g1_.w) * vec4<f32>(_e257.g1_.w, _e260.g0_.z, _e263.g0_.y, _e266.g0_.x)) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))));
}

fn point_and_plane_motor_left_contraction(self_1354: PointAndPlane, other_1150: Motor) -> PointAndPlane {
    var self_1355: PointAndPlane;
    var other_1151: Motor;

    self_1355 = self_1354;
    other_1151 = other_1150;
    let _e4: PointAndPlane = self_1355;
    let _e6: Motor = other_1151;
    let _e11: PointAndPlane = self_1355;
    let _e15: Motor = other_1151;
    let _e25: PointAndPlane = self_1355;
    let _e29: Motor = other_1151;
    let _e32: Motor = other_1151;
    let _e35: Motor = other_1151;
    let _e38: Motor = other_1151;
    let _e52: PointAndPlane = self_1355;
    let _e56: Motor = other_1151;
    let _e59: Motor = other_1151;
    let _e62: Motor = other_1151;
    let _e65: Motor = other_1151;
    let _e79: PointAndPlane = self_1355;
    let _e83: Motor = other_1151;
    let _e86: Motor = other_1151;
    let _e89: Motor = other_1151;
    let _e92: Motor = other_1151;
    let _e106: PointAndPlane = self_1355;
    let _e108: Motor = other_1151;
    return PointAndPlane((_e4.g1_ * vec4<f32>(_e6.g1_.x)), ((((((vec4<f32>(_e11.g1_.x) * _e15.g1_.yyzw) * vec4<f32>(0.0, 1.0, 1.0, 1.0)) + ((vec4<f32>(_e25.g1_.y) * vec4<f32>(_e29.g1_.y, _e32.g1_.y, _e35.g0_.w, _e38.g0_.z)) * vec4<f32>(-(1.0), 0.0, 1.0, -(1.0)))) + ((vec4<f32>(_e52.g1_.z) * vec4<f32>(_e56.g1_.z, _e59.g0_.w, _e62.g1_.z, _e65.g0_.y)) * vec4<f32>(-(1.0), -(1.0), 0.0, 1.0))) + ((vec4<f32>(_e79.g1_.w) * vec4<f32>(_e83.g1_.w, _e86.g0_.z, _e89.g0_.y, _e92.g1_.w)) * vec4<f32>(-(1.0), 1.0, -(1.0), 0.0))) + (_e106.g0_ * vec4<f32>(_e108.g1_.x))));
}

fn point_and_plane_motor_right_contraction(self_1356: PointAndPlane, other_1152: Motor) -> PointAndPlane {
    var self_1357: PointAndPlane;
    var other_1153: Motor;

    self_1357 = self_1356;
    other_1153 = other_1152;
    let _e4: PointAndPlane = self_1357;
    let _e6: Motor = other_1153;
    let _e11: PointAndPlane = self_1357;
    let _e15: Motor = other_1153;
    let _e18: Motor = other_1153;
    let _e21: Motor = other_1153;
    let _e24: Motor = other_1153;
    let _e36: PointAndPlane = self_1357;
    let _e40: Motor = other_1153;
    let _e43: Motor = other_1153;
    let _e46: Motor = other_1153;
    let _e49: Motor = other_1153;
    let _e62: PointAndPlane = self_1357;
    let _e66: Motor = other_1153;
    let _e69: Motor = other_1153;
    let _e72: Motor = other_1153;
    let _e75: Motor = other_1153;
    let _e88: PointAndPlane = self_1357;
    let _e92: Motor = other_1153;
    let _e104: PointAndPlane = self_1357;
    let _e108: Motor = other_1153;
    let _e120: PointAndPlane = self_1357;
    let _e124: Motor = other_1153;
    let _e136: PointAndPlane = self_1357;
    let _e139: PointAndPlane = self_1357;
    let _e142: PointAndPlane = self_1357;
    let _e145: PointAndPlane = self_1357;
    let _e149: Motor = other_1153;
    return PointAndPlane((_e4.g0_ * vec4<f32>(_e6.g0_.x)), ((((((((vec4<f32>(_e11.g0_.y) * vec4<f32>(_e15.g0_.y, _e18.g0_.y, _e21.g1_.w, _e24.g1_.z)) * vec4<f32>(1.0, 0.0, -(1.0), 1.0)) + ((vec4<f32>(_e36.g0_.z) * vec4<f32>(_e40.g0_.z, _e43.g1_.w, _e46.g0_.z, _e49.g1_.y)) * vec4<f32>(1.0, 1.0, 0.0, -(1.0)))) + ((vec4<f32>(_e62.g0_.w) * vec4<f32>(_e66.g0_.w, _e69.g1_.z, _e72.g1_.y, _e75.g0_.w)) * vec4<f32>(1.0, -(1.0), 1.0, 0.0))) + ((vec4<f32>(_e88.g1_.y) * vec4<f32>(_e92.g0_.x)) * vec4<f32>(0.0, 1.0, 0.0, 0.0))) + ((vec4<f32>(_e104.g1_.z) * vec4<f32>(_e108.g0_.x)) * vec4<f32>(0.0, 0.0, 1.0, 0.0))) + ((vec4<f32>(_e120.g1_.w) * vec4<f32>(_e124.g0_.x)) * vec4<f32>(0.0, 0.0, 0.0, 1.0))) + ((vec4<f32>(_e136.g1_.x, _e139.g0_.x, _e142.g0_.x, _e145.g0_.x) * _e149.g0_) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0)))));
}

fn point_and_plane_point_and_plane_add(self_1358: PointAndPlane, other_1154: PointAndPlane) -> PointAndPlane {
    var self_1359: PointAndPlane;
    var other_1155: PointAndPlane;

    self_1359 = self_1358;
    other_1155 = other_1154;
    let _e4: PointAndPlane = self_1359;
    let _e6: PointAndPlane = other_1155;
    let _e9: PointAndPlane = self_1359;
    let _e11: PointAndPlane = other_1155;
    return PointAndPlane((_e4.g0_ + _e6.g0_), (_e9.g1_ + _e11.g1_));
}

fn point_and_plane_point_and_plane_sub(self_1360: PointAndPlane, other_1156: PointAndPlane) -> PointAndPlane {
    var self_1361: PointAndPlane;
    var other_1157: PointAndPlane;

    self_1361 = self_1360;
    other_1157 = other_1156;
    let _e4: PointAndPlane = self_1361;
    let _e6: PointAndPlane = other_1157;
    let _e9: PointAndPlane = self_1361;
    let _e11: PointAndPlane = other_1157;
    return PointAndPlane((_e4.g0_ - _e6.g0_), (_e9.g1_ - _e11.g1_));
}

fn point_and_plane_point_and_plane_mul(self_1362: PointAndPlane, other_1158: PointAndPlane) -> PointAndPlane {
    var self_1363: PointAndPlane;
    var other_1159: PointAndPlane;

    self_1363 = self_1362;
    other_1159 = other_1158;
    let _e4: PointAndPlane = self_1363;
    let _e6: PointAndPlane = other_1159;
    let _e9: PointAndPlane = self_1363;
    let _e11: PointAndPlane = other_1159;
    return PointAndPlane((_e4.g0_ * _e6.g0_), (_e9.g1_ * _e11.g1_));
}

fn point_and_plane_point_and_plane_div(self_1364: PointAndPlane, other_1160: PointAndPlane) -> PointAndPlane {
    var self_1365: PointAndPlane;
    var other_1161: PointAndPlane;

    self_1365 = self_1364;
    other_1161 = other_1160;
    let _e4: PointAndPlane = self_1365;
    let _e7: PointAndPlane = self_1365;
    let _e10: PointAndPlane = self_1365;
    let _e13: PointAndPlane = self_1365;
    let _e23: PointAndPlane = other_1161;
    let _e26: PointAndPlane = other_1161;
    let _e29: PointAndPlane = other_1161;
    let _e32: PointAndPlane = other_1161;
    let _e43: PointAndPlane = self_1365;
    let _e46: PointAndPlane = self_1365;
    let _e49: PointAndPlane = self_1365;
    let _e52: PointAndPlane = self_1365;
    let _e62: PointAndPlane = other_1161;
    let _e65: PointAndPlane = other_1161;
    let _e68: PointAndPlane = other_1161;
    let _e71: PointAndPlane = other_1161;
    return PointAndPlane((((vec4<f32>(_e4.g0_.x, _e7.g0_.y, _e10.g0_.z, _e13.g0_.w) * vec4<f32>(1.0, 1.0, 1.0, 1.0)) / vec4<f32>(_e23.g0_.x, _e26.g0_.y, _e29.g0_.z, _e32.g0_.w)) * vec4<f32>(1.0, 1.0, 1.0, 1.0)), (((vec4<f32>(_e43.g1_.x, _e46.g1_.y, _e49.g1_.z, _e52.g1_.w) * vec4<f32>(1.0, 1.0, 1.0, 1.0)) / vec4<f32>(_e62.g1_.x, _e65.g1_.y, _e68.g1_.z, _e71.g1_.w)) * vec4<f32>(1.0, 1.0, 1.0, 1.0)));
}

fn point_and_plane_point_and_plane_geometric_product(self_1366: PointAndPlane, other_1162: PointAndPlane) -> Motor {
    var self_1367: PointAndPlane;
    var other_1163: PointAndPlane;

    self_1367 = self_1366;
    other_1163 = other_1162;
    let _e4: PointAndPlane = self_1367;
    let _e8: PointAndPlane = other_1163;
    let _e11: PointAndPlane = other_1163;
    let _e14: PointAndPlane = other_1163;
    let _e17: PointAndPlane = other_1163;
    let _e29: PointAndPlane = self_1367;
    let _e33: PointAndPlane = other_1163;
    let _e36: PointAndPlane = other_1163;
    let _e39: PointAndPlane = other_1163;
    let _e42: PointAndPlane = other_1163;
    let _e57: PointAndPlane = self_1367;
    let _e61: PointAndPlane = other_1163;
    let _e64: PointAndPlane = other_1163;
    let _e67: PointAndPlane = other_1163;
    let _e70: PointAndPlane = other_1163;
    let _e85: PointAndPlane = self_1367;
    let _e89: PointAndPlane = other_1163;
    let _e92: PointAndPlane = other_1163;
    let _e95: PointAndPlane = other_1163;
    let _e98: PointAndPlane = other_1163;
    let _e113: PointAndPlane = self_1367;
    let _e117: PointAndPlane = other_1163;
    let _e120: PointAndPlane = other_1163;
    let _e123: PointAndPlane = other_1163;
    let _e126: PointAndPlane = other_1163;
    let _e141: PointAndPlane = self_1367;
    let _e145: PointAndPlane = other_1163;
    let _e148: PointAndPlane = other_1163;
    let _e151: PointAndPlane = other_1163;
    let _e154: PointAndPlane = other_1163;
    let _e167: PointAndPlane = self_1367;
    let _e171: PointAndPlane = other_1163;
    let _e174: PointAndPlane = other_1163;
    let _e177: PointAndPlane = other_1163;
    let _e180: PointAndPlane = other_1163;
    let _e193: PointAndPlane = self_1367;
    let _e197: PointAndPlane = other_1163;
    let _e200: PointAndPlane = other_1163;
    let _e203: PointAndPlane = other_1163;
    let _e206: PointAndPlane = other_1163;
    let _e221: PointAndPlane = self_1367;
    let _e225: PointAndPlane = other_1163;
    let _e228: PointAndPlane = other_1163;
    let _e231: PointAndPlane = other_1163;
    let _e234: PointAndPlane = other_1163;
    let _e240: PointAndPlane = self_1367;
    let _e244: PointAndPlane = other_1163;
    let _e247: PointAndPlane = other_1163;
    let _e250: PointAndPlane = other_1163;
    let _e253: PointAndPlane = other_1163;
    let _e267: PointAndPlane = self_1367;
    let _e271: PointAndPlane = other_1163;
    let _e274: PointAndPlane = other_1163;
    let _e277: PointAndPlane = other_1163;
    let _e280: PointAndPlane = other_1163;
    let _e294: PointAndPlane = self_1367;
    let _e298: PointAndPlane = other_1163;
    let _e301: PointAndPlane = other_1163;
    let _e304: PointAndPlane = other_1163;
    let _e307: PointAndPlane = other_1163;
    let _e321: PointAndPlane = self_1367;
    let _e325: PointAndPlane = other_1163;
    let _e328: PointAndPlane = other_1163;
    let _e331: PointAndPlane = other_1163;
    let _e334: PointAndPlane = other_1163;
    let _e340: PointAndPlane = self_1367;
    let _e344: PointAndPlane = other_1163;
    let _e347: PointAndPlane = other_1163;
    let _e350: PointAndPlane = other_1163;
    let _e353: PointAndPlane = other_1163;
    let _e367: PointAndPlane = self_1367;
    let _e371: PointAndPlane = other_1163;
    let _e374: PointAndPlane = other_1163;
    let _e377: PointAndPlane = other_1163;
    let _e380: PointAndPlane = other_1163;
    let _e394: PointAndPlane = self_1367;
    let _e398: PointAndPlane = other_1163;
    let _e401: PointAndPlane = other_1163;
    let _e404: PointAndPlane = other_1163;
    let _e407: PointAndPlane = other_1163;
    return Motor((((((((((vec4<f32>(_e4.g0_.x) * vec4<f32>(_e8.g0_.x, _e11.g1_.y, _e14.g1_.z, _e17.g1_.w)) * vec4<f32>(-(1.0), 1.0, 1.0, 1.0)) + ((vec4<f32>(_e29.g0_.y) * vec4<f32>(_e33.g0_.y, _e36.g1_.x, _e39.g0_.w, _e42.g0_.z)) * vec4<f32>(-(1.0), -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e57.g0_.z) * vec4<f32>(_e61.g0_.z, _e64.g0_.w, _e67.g1_.x, _e70.g0_.y)) * vec4<f32>(-(1.0), -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e85.g0_.w) * vec4<f32>(_e89.g0_.w, _e92.g0_.z, _e95.g0_.y, _e98.g1_.x)) * vec4<f32>(-(1.0), 1.0, -(1.0), -(1.0)))) + ((vec4<f32>(_e113.g1_.x) * vec4<f32>(_e117.g1_.x, _e120.g0_.y, _e123.g0_.z, _e126.g0_.w)) * vec4<f32>(1.0, -(1.0), -(1.0), -(1.0)))) + ((vec4<f32>(_e141.g1_.y) * vec4<f32>(_e145.g1_.y, _e148.g0_.x, _e151.g1_.w, _e154.g1_.z)) * vec4<f32>(1.0, 1.0, -(1.0), 1.0))) + ((vec4<f32>(_e167.g1_.z) * vec4<f32>(_e171.g1_.z, _e174.g1_.w, _e177.g0_.x, _e180.g1_.y)) * vec4<f32>(1.0, 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e193.g1_.w) * vec4<f32>(_e197.g1_.w, _e200.g1_.z, _e203.g1_.y, _e206.g0_.x)) * vec4<f32>(1.0, -(1.0), 1.0, 1.0))), ((((((((vec4<f32>(0.0) - (vec4<f32>(_e221.g0_.x) * vec4<f32>(_e225.g1_.x, _e228.g0_.y, _e231.g0_.z, _e234.g0_.w))) + ((vec4<f32>(_e240.g0_.y) * vec4<f32>(_e244.g1_.y, _e247.g0_.x, _e250.g1_.w, _e253.g1_.z)) * vec4<f32>(-(1.0), 1.0, -(1.0), 1.0))) + ((vec4<f32>(_e267.g0_.z) * vec4<f32>(_e271.g1_.z, _e274.g1_.w, _e277.g0_.x, _e280.g1_.y)) * vec4<f32>(-(1.0), 1.0, 1.0, -(1.0)))) + ((vec4<f32>(_e294.g0_.w) * vec4<f32>(_e298.g1_.w, _e301.g1_.z, _e304.g1_.y, _e307.g0_.x)) * vec4<f32>(-(1.0), -(1.0), 1.0, 1.0))) + (vec4<f32>(_e321.g1_.x) * vec4<f32>(_e325.g0_.x, _e328.g1_.y, _e331.g1_.z, _e334.g1_.w))) + ((vec4<f32>(_e340.g1_.y) * vec4<f32>(_e344.g0_.y, _e347.g1_.x, _e350.g0_.w, _e353.g0_.z)) * vec4<f32>(1.0, -(1.0), 1.0, -(1.0)))) + ((vec4<f32>(_e367.g1_.z) * vec4<f32>(_e371.g0_.z, _e374.g0_.w, _e377.g1_.x, _e380.g0_.y)) * vec4<f32>(1.0, -(1.0), -(1.0), 1.0))) + ((vec4<f32>(_e394.g1_.w) * vec4<f32>(_e398.g0_.w, _e401.g0_.z, _e404.g0_.y, _e407.g1_.x)) * vec4<f32>(1.0, 1.0, -(1.0), -(1.0)))));
}

fn point_and_plane_point_and_plane_scalar_product(self_1368: PointAndPlane, other_1164: PointAndPlane) -> Scalar {
    var self_1369: PointAndPlane;
    var other_1165: PointAndPlane;

    self_1369 = self_1368;
    other_1165 = other_1164;
    let _e5: PointAndPlane = self_1369;
    let _e8: PointAndPlane = other_1165;
    let _e13: PointAndPlane = self_1369;
    let _e16: PointAndPlane = other_1165;
    let _e21: PointAndPlane = self_1369;
    let _e24: PointAndPlane = other_1165;
    let _e29: PointAndPlane = self_1369;
    let _e32: PointAndPlane = other_1165;
    let _e37: PointAndPlane = self_1369;
    let _e40: PointAndPlane = other_1165;
    let _e45: PointAndPlane = self_1369;
    let _e48: PointAndPlane = other_1165;
    let _e53: PointAndPlane = self_1369;
    let _e56: PointAndPlane = other_1165;
    let _e61: PointAndPlane = self_1369;
    let _e64: PointAndPlane = other_1165;
    return Scalar(((((((((0.0 - (_e5.g0_.x * _e8.g0_.x)) - (_e13.g0_.y * _e16.g0_.y)) - (_e21.g0_.z * _e24.g0_.z)) - (_e29.g0_.w * _e32.g0_.w)) + (_e37.g1_.x * _e40.g1_.x)) + (_e45.g1_.y * _e48.g1_.y)) + (_e53.g1_.z * _e56.g1_.z)) + (_e61.g1_.w * _e64.g1_.w)));
}

fn point_and_plane_squared_magnitude(self_1370: PointAndPlane) -> Scalar {
    var self_1371: PointAndPlane;

    self_1371 = self_1370;
    let _e4: PointAndPlane = self_1371;
    let _e5: PointAndPlane = point_and_plane_reversal(_e4);
    let _e6: PointAndPlane = self_1371;
    let _e8: PointAndPlane = self_1371;
    let _e9: PointAndPlane = point_and_plane_reversal(_e8);
    let _e10: Scalar = point_and_plane_point_and_plane_scalar_product(_e6, _e9);
    return _e10;
}

fn point_and_plane_magnitude(self_1372: PointAndPlane) -> Scalar {
    var self_1373: PointAndPlane;

    self_1373 = self_1372;
    let _e3: PointAndPlane = self_1373;
    let _e4: Scalar = point_and_plane_squared_magnitude(_e3);
    let _e7: PointAndPlane = self_1373;
    let _e8: Scalar = point_and_plane_squared_magnitude(_e7);
    return Scalar(sqrt(_e8.g0_));
}

fn point_and_plane_scale(self_1374: PointAndPlane, other_1166: f32) -> PointAndPlane {
    var self_1375: PointAndPlane;
    var other_1167: f32;

    self_1375 = self_1374;
    other_1167 = other_1166;
    let _e5: f32 = other_1167;
    let _e7: PointAndPlane = self_1375;
    let _e8: f32 = other_1167;
    let _e10: PointAndPlane = point_and_plane_scalar_geometric_product(_e7, Scalar(_e8));
    return _e10;
}

fn point_and_plane_signum(self_1376: PointAndPlane) -> PointAndPlane {
    var self_1377: PointAndPlane;

    self_1377 = self_1376;
    let _e5: PointAndPlane = self_1377;
    let _e6: Scalar = point_and_plane_magnitude(_e5);
    let _e10: PointAndPlane = self_1377;
    let _e13: PointAndPlane = self_1377;
    let _e14: Scalar = point_and_plane_magnitude(_e13);
    let _e18: PointAndPlane = point_and_plane_scalar_geometric_product(_e10, Scalar((1.0 / _e14.g0_)));
    return _e18;
}

fn point_and_plane_inverse(self_1378: PointAndPlane) -> PointAndPlane {
    var self_1379: PointAndPlane;

    self_1379 = self_1378;
    let _e3: PointAndPlane = self_1379;
    let _e4: PointAndPlane = point_and_plane_reversal(_e3);
    let _e7: PointAndPlane = self_1379;
    let _e8: Scalar = point_and_plane_squared_magnitude(_e7);
    let _e13: PointAndPlane = self_1379;
    let _e14: PointAndPlane = point_and_plane_reversal(_e13);
    let _e17: PointAndPlane = self_1379;
    let _e18: Scalar = point_and_plane_squared_magnitude(_e17);
    let _e22: PointAndPlane = point_and_plane_scalar_geometric_product(_e14, Scalar((1.0 / _e18.g0_)));
    return _e22;
}

fn ideal_point_ideal_point_geometric_quotient(self_1380: IdealPoint, other_1168: IdealPoint) -> Rotor {
    var self_1381: IdealPoint;
    var other_1169: IdealPoint;

    self_1381 = self_1380;
    other_1169 = other_1168;
    let _e6: IdealPoint = other_1169;
    let _e7: IdealPoint = ideal_point_inverse(_e6);
    let _e8: IdealPoint = self_1381;
    let _e10: IdealPoint = other_1169;
    let _e11: IdealPoint = ideal_point_inverse(_e10);
    let _e12: Rotor = ideal_point_ideal_point_geometric_product(_e8, _e11);
    return _e12;
}

fn ideal_point_line_geometric_quotient(self_1382: IdealPoint, other_1170: Line) -> Motor {
    var self_1383: IdealPoint;
    var other_1171: Line;

    self_1383 = self_1382;
    other_1171 = other_1170;
    let _e6: Line = other_1171;
    let _e7: Line = line_inverse(_e6);
    let _e8: IdealPoint = self_1383;
    let _e10: Line = other_1171;
    let _e11: Line = line_inverse(_e10);
    let _e12: Motor = ideal_point_line_geometric_product(_e8, _e11);
    return _e12;
}

fn ideal_point_line_transformation(self_1384: IdealPoint, other_1172: Line) -> Line {
    var self_1385: IdealPoint;
    var other_1173: Line;

    self_1385 = self_1384;
    other_1173 = other_1172;
    let _e6: IdealPoint = self_1385;
    let _e7: Line = other_1173;
    let _e8: Motor = ideal_point_line_geometric_product(_e6, _e7);
    let _e10: IdealPoint = self_1385;
    let _e11: IdealPoint = ideal_point_reversal(_e10);
    let _e14: IdealPoint = self_1385;
    let _e15: Line = other_1173;
    let _e16: Motor = ideal_point_line_geometric_product(_e14, _e15);
    let _e18: IdealPoint = self_1385;
    let _e19: IdealPoint = ideal_point_reversal(_e18);
    let _e20: Motor = motor_ideal_point_geometric_product(_e16, _e19);
    let _e23: IdealPoint = self_1385;
    let _e24: Line = other_1173;
    let _e25: Motor = ideal_point_line_geometric_product(_e23, _e24);
    let _e27: IdealPoint = self_1385;
    let _e28: IdealPoint = ideal_point_reversal(_e27);
    let _e31: IdealPoint = self_1385;
    let _e32: Line = other_1173;
    let _e33: Motor = ideal_point_line_geometric_product(_e31, _e32);
    let _e35: IdealPoint = self_1385;
    let _e36: IdealPoint = ideal_point_reversal(_e35);
    let _e37: Motor = motor_ideal_point_geometric_product(_e33, _e36);
    let _e38: Line = motor_line_into(_e37);
    return _e38;
}

fn ideal_point_motor_geometric_quotient(self_1386: IdealPoint, other_1174: Motor) -> Motor {
    var self_1387: IdealPoint;
    var other_1175: Motor;

    self_1387 = self_1386;
    other_1175 = other_1174;
    let _e6: Motor = other_1175;
    let _e7: Motor = motor_inverse(_e6);
    let _e8: IdealPoint = self_1387;
    let _e10: Motor = other_1175;
    let _e11: Motor = motor_inverse(_e10);
    let _e12: Motor = ideal_point_motor_geometric_product(_e8, _e11);
    return _e12;
}

fn ideal_point_motor_transformation(self_1388: IdealPoint, other_1176: Motor) -> Motor {
    var self_1389: IdealPoint;
    var other_1177: Motor;

    self_1389 = self_1388;
    other_1177 = other_1176;
    let _e6: IdealPoint = self_1389;
    let _e7: Motor = other_1177;
    let _e8: Motor = ideal_point_motor_geometric_product(_e6, _e7);
    let _e10: IdealPoint = self_1389;
    let _e11: IdealPoint = ideal_point_reversal(_e10);
    let _e14: IdealPoint = self_1389;
    let _e15: Motor = other_1177;
    let _e16: Motor = ideal_point_motor_geometric_product(_e14, _e15);
    let _e18: IdealPoint = self_1389;
    let _e19: IdealPoint = ideal_point_reversal(_e18);
    let _e20: Motor = motor_ideal_point_geometric_product(_e16, _e19);
    return _e20;
}

fn ideal_point_multi_vector_geometric_quotient(self_1390: IdealPoint, other_1178: MultiVector) -> MultiVector {
    var self_1391: IdealPoint;
    var other_1179: MultiVector;

    self_1391 = self_1390;
    other_1179 = other_1178;
    let _e6: MultiVector = other_1179;
    let _e7: MultiVector = multi_vector_inverse(_e6);
    let _e8: IdealPoint = self_1391;
    let _e10: MultiVector = other_1179;
    let _e11: MultiVector = multi_vector_inverse(_e10);
    let _e12: MultiVector = ideal_point_multi_vector_geometric_product(_e8, _e11);
    return _e12;
}

fn ideal_point_multi_vector_transformation(self_1392: IdealPoint, other_1180: MultiVector) -> MultiVector {
    var self_1393: IdealPoint;
    var other_1181: MultiVector;

    self_1393 = self_1392;
    other_1181 = other_1180;
    let _e6: IdealPoint = self_1393;
    let _e7: MultiVector = other_1181;
    let _e8: MultiVector = ideal_point_multi_vector_geometric_product(_e6, _e7);
    let _e10: IdealPoint = self_1393;
    let _e11: IdealPoint = ideal_point_reversal(_e10);
    let _e14: IdealPoint = self_1393;
    let _e15: MultiVector = other_1181;
    let _e16: MultiVector = ideal_point_multi_vector_geometric_product(_e14, _e15);
    let _e18: IdealPoint = self_1393;
    let _e19: IdealPoint = ideal_point_reversal(_e18);
    let _e20: MultiVector = multi_vector_ideal_point_geometric_product(_e16, _e19);
    return _e20;
}

fn ideal_point_point_and_plane_geometric_quotient(self_1394: IdealPoint, other_1182: PointAndPlane) -> PointAndPlane {
    var self_1395: IdealPoint;
    var other_1183: PointAndPlane;

    self_1395 = self_1394;
    other_1183 = other_1182;
    let _e6: PointAndPlane = other_1183;
    let _e7: PointAndPlane = point_and_plane_inverse(_e6);
    let _e8: IdealPoint = self_1395;
    let _e10: PointAndPlane = other_1183;
    let _e11: PointAndPlane = point_and_plane_inverse(_e10);
    let _e12: PointAndPlane = ideal_point_point_and_plane_geometric_product(_e8, _e11);
    return _e12;
}

fn ideal_point_point_and_plane_transformation(self_1396: IdealPoint, other_1184: PointAndPlane) -> PointAndPlane {
    var self_1397: IdealPoint;
    var other_1185: PointAndPlane;

    self_1397 = self_1396;
    other_1185 = other_1184;
    let _e6: IdealPoint = self_1397;
    let _e7: PointAndPlane = other_1185;
    let _e8: PointAndPlane = ideal_point_point_and_plane_geometric_product(_e6, _e7);
    let _e10: IdealPoint = self_1397;
    let _e11: IdealPoint = ideal_point_reversal(_e10);
    let _e14: IdealPoint = self_1397;
    let _e15: PointAndPlane = other_1185;
    let _e16: PointAndPlane = ideal_point_point_and_plane_geometric_product(_e14, _e15);
    let _e18: IdealPoint = self_1397;
    let _e19: IdealPoint = ideal_point_reversal(_e18);
    let _e20: PointAndPlane = point_and_plane_ideal_point_geometric_product(_e16, _e19);
    return _e20;
}

fn ideal_point_scalar_geometric_quotient(self_1398: IdealPoint, other_1186: Scalar) -> IdealPoint {
    var self_1399: IdealPoint;
    var other_1187: Scalar;

    self_1399 = self_1398;
    other_1187 = other_1186;
    let _e6: Scalar = other_1187;
    let _e7: Scalar = scalar_inverse(_e6);
    let _e8: IdealPoint = self_1399;
    let _e10: Scalar = other_1187;
    let _e11: Scalar = scalar_inverse(_e10);
    let _e12: IdealPoint = ideal_point_scalar_geometric_product(_e8, _e11);
    return _e12;
}

fn ideal_point_scalar_transformation(self_1400: IdealPoint, other_1188: Scalar) -> Scalar {
    var self_1401: IdealPoint;
    var other_1189: Scalar;

    self_1401 = self_1400;
    other_1189 = other_1188;
    let _e6: IdealPoint = self_1401;
    let _e7: Scalar = other_1189;
    let _e8: IdealPoint = ideal_point_scalar_geometric_product(_e6, _e7);
    let _e10: IdealPoint = self_1401;
    let _e11: IdealPoint = ideal_point_reversal(_e10);
    let _e14: IdealPoint = self_1401;
    let _e15: Scalar = other_1189;
    let _e16: IdealPoint = ideal_point_scalar_geometric_product(_e14, _e15);
    let _e18: IdealPoint = self_1401;
    let _e19: IdealPoint = ideal_point_reversal(_e18);
    let _e20: Rotor = ideal_point_ideal_point_geometric_product(_e16, _e19);
    let _e23: IdealPoint = self_1401;
    let _e24: Scalar = other_1189;
    let _e25: IdealPoint = ideal_point_scalar_geometric_product(_e23, _e24);
    let _e27: IdealPoint = self_1401;
    let _e28: IdealPoint = ideal_point_reversal(_e27);
    let _e31: IdealPoint = self_1401;
    let _e32: Scalar = other_1189;
    let _e33: IdealPoint = ideal_point_scalar_geometric_product(_e31, _e32);
    let _e35: IdealPoint = self_1401;
    let _e36: IdealPoint = ideal_point_reversal(_e35);
    let _e37: Rotor = ideal_point_ideal_point_geometric_product(_e33, _e36);
    let _e38: Scalar = rotor_scalar_into(_e37);
    return _e38;
}

fn line_ideal_point_geometric_quotient(self_1402: Line, other_1190: IdealPoint) -> Motor {
    var self_1403: Line;
    var other_1191: IdealPoint;

    self_1403 = self_1402;
    other_1191 = other_1190;
    let _e6: IdealPoint = other_1191;
    let _e7: IdealPoint = ideal_point_inverse(_e6);
    let _e8: Line = self_1403;
    let _e10: IdealPoint = other_1191;
    let _e11: IdealPoint = ideal_point_inverse(_e10);
    let _e12: Motor = line_ideal_point_geometric_product(_e8, _e11);
    return _e12;
}

fn line_ideal_point_transformation(self_1404: Line, other_1192: IdealPoint) -> IdealPoint {
    var self_1405: Line;
    var other_1193: IdealPoint;

    self_1405 = self_1404;
    other_1193 = other_1192;
    let _e6: Line = self_1405;
    let _e7: IdealPoint = other_1193;
    let _e8: Motor = line_ideal_point_geometric_product(_e6, _e7);
    let _e10: Line = self_1405;
    let _e11: Line = line_reversal(_e10);
    let _e14: Line = self_1405;
    let _e15: IdealPoint = other_1193;
    let _e16: Motor = line_ideal_point_geometric_product(_e14, _e15);
    let _e18: Line = self_1405;
    let _e19: Line = line_reversal(_e18);
    let _e20: Motor = motor_line_geometric_product(_e16, _e19);
    let _e23: Line = self_1405;
    let _e24: IdealPoint = other_1193;
    let _e25: Motor = line_ideal_point_geometric_product(_e23, _e24);
    let _e27: Line = self_1405;
    let _e28: Line = line_reversal(_e27);
    let _e31: Line = self_1405;
    let _e32: IdealPoint = other_1193;
    let _e33: Motor = line_ideal_point_geometric_product(_e31, _e32);
    let _e35: Line = self_1405;
    let _e36: Line = line_reversal(_e35);
    let _e37: Motor = motor_line_geometric_product(_e33, _e36);
    let _e38: IdealPoint = motor_ideal_point_into(_e37);
    return _e38;
}

fn line_line_geometric_quotient(self_1406: Line, other_1194: Line) -> Motor {
    var self_1407: Line;
    var other_1195: Line;

    self_1407 = self_1406;
    other_1195 = other_1194;
    let _e6: Line = other_1195;
    let _e7: Line = line_inverse(_e6);
    let _e8: Line = self_1407;
    let _e10: Line = other_1195;
    let _e11: Line = line_inverse(_e10);
    let _e12: Motor = line_line_geometric_product(_e8, _e11);
    return _e12;
}

fn line_line_transformation(self_1408: Line, other_1196: Line) -> Line {
    var self_1409: Line;
    var other_1197: Line;

    self_1409 = self_1408;
    other_1197 = other_1196;
    let _e6: Line = self_1409;
    let _e7: Line = other_1197;
    let _e8: Motor = line_line_geometric_product(_e6, _e7);
    let _e10: Line = self_1409;
    let _e11: Line = line_reversal(_e10);
    let _e14: Line = self_1409;
    let _e15: Line = other_1197;
    let _e16: Motor = line_line_geometric_product(_e14, _e15);
    let _e18: Line = self_1409;
    let _e19: Line = line_reversal(_e18);
    let _e20: Motor = motor_line_geometric_product(_e16, _e19);
    let _e23: Line = self_1409;
    let _e24: Line = other_1197;
    let _e25: Motor = line_line_geometric_product(_e23, _e24);
    let _e27: Line = self_1409;
    let _e28: Line = line_reversal(_e27);
    let _e31: Line = self_1409;
    let _e32: Line = other_1197;
    let _e33: Motor = line_line_geometric_product(_e31, _e32);
    let _e35: Line = self_1409;
    let _e36: Line = line_reversal(_e35);
    let _e37: Motor = motor_line_geometric_product(_e33, _e36);
    let _e38: Line = motor_line_into(_e37);
    return _e38;
}

fn line_motor_geometric_quotient(self_1410: Line, other_1198: Motor) -> Motor {
    var self_1411: Line;
    var other_1199: Motor;

    self_1411 = self_1410;
    other_1199 = other_1198;
    let _e6: Motor = other_1199;
    let _e7: Motor = motor_inverse(_e6);
    let _e8: Line = self_1411;
    let _e10: Motor = other_1199;
    let _e11: Motor = motor_inverse(_e10);
    let _e12: Motor = line_motor_geometric_product(_e8, _e11);
    return _e12;
}

fn line_motor_transformation(self_1412: Line, other_1200: Motor) -> Motor {
    var self_1413: Line;
    var other_1201: Motor;

    self_1413 = self_1412;
    other_1201 = other_1200;
    let _e6: Line = self_1413;
    let _e7: Motor = other_1201;
    let _e8: Motor = line_motor_geometric_product(_e6, _e7);
    let _e10: Line = self_1413;
    let _e11: Line = line_reversal(_e10);
    let _e14: Line = self_1413;
    let _e15: Motor = other_1201;
    let _e16: Motor = line_motor_geometric_product(_e14, _e15);
    let _e18: Line = self_1413;
    let _e19: Line = line_reversal(_e18);
    let _e20: Motor = motor_line_geometric_product(_e16, _e19);
    return _e20;
}

fn line_multi_vector_geometric_quotient(self_1414: Line, other_1202: MultiVector) -> MultiVector {
    var self_1415: Line;
    var other_1203: MultiVector;

    self_1415 = self_1414;
    other_1203 = other_1202;
    let _e6: MultiVector = other_1203;
    let _e7: MultiVector = multi_vector_inverse(_e6);
    let _e8: Line = self_1415;
    let _e10: MultiVector = other_1203;
    let _e11: MultiVector = multi_vector_inverse(_e10);
    let _e12: MultiVector = line_multi_vector_geometric_product(_e8, _e11);
    return _e12;
}

fn line_multi_vector_transformation(self_1416: Line, other_1204: MultiVector) -> MultiVector {
    var self_1417: Line;
    var other_1205: MultiVector;

    self_1417 = self_1416;
    other_1205 = other_1204;
    let _e6: Line = self_1417;
    let _e7: MultiVector = other_1205;
    let _e8: MultiVector = line_multi_vector_geometric_product(_e6, _e7);
    let _e10: Line = self_1417;
    let _e11: Line = line_reversal(_e10);
    let _e14: Line = self_1417;
    let _e15: MultiVector = other_1205;
    let _e16: MultiVector = line_multi_vector_geometric_product(_e14, _e15);
    let _e18: Line = self_1417;
    let _e19: Line = line_reversal(_e18);
    let _e20: MultiVector = multi_vector_line_geometric_product(_e16, _e19);
    return _e20;
}

fn line_plane_geometric_quotient(self_1418: Line, other_1206: Plane) -> PointAndPlane {
    var self_1419: Line;
    var other_1207: Plane;

    self_1419 = self_1418;
    other_1207 = other_1206;
    let _e6: Plane = other_1207;
    let _e7: Plane = plane_inverse(_e6);
    let _e8: Line = self_1419;
    let _e10: Plane = other_1207;
    let _e11: Plane = plane_inverse(_e10);
    let _e12: PointAndPlane = line_plane_geometric_product(_e8, _e11);
    return _e12;
}

fn line_plane_transformation(self_1420: Line, other_1208: Plane) -> Plane {
    var self_1421: Line;
    var other_1209: Plane;

    self_1421 = self_1420;
    other_1209 = other_1208;
    let _e6: Line = self_1421;
    let _e7: Plane = other_1209;
    let _e8: PointAndPlane = line_plane_geometric_product(_e6, _e7);
    let _e10: Line = self_1421;
    let _e11: Line = line_reversal(_e10);
    let _e14: Line = self_1421;
    let _e15: Plane = other_1209;
    let _e16: PointAndPlane = line_plane_geometric_product(_e14, _e15);
    let _e18: Line = self_1421;
    let _e19: Line = line_reversal(_e18);
    let _e20: PointAndPlane = point_and_plane_line_geometric_product(_e16, _e19);
    let _e23: Line = self_1421;
    let _e24: Plane = other_1209;
    let _e25: PointAndPlane = line_plane_geometric_product(_e23, _e24);
    let _e27: Line = self_1421;
    let _e28: Line = line_reversal(_e27);
    let _e31: Line = self_1421;
    let _e32: Plane = other_1209;
    let _e33: PointAndPlane = line_plane_geometric_product(_e31, _e32);
    let _e35: Line = self_1421;
    let _e36: Line = line_reversal(_e35);
    let _e37: PointAndPlane = point_and_plane_line_geometric_product(_e33, _e36);
    let _e38: Plane = point_and_plane_plane_into(_e37);
    return _e38;
}

fn line_point_geometric_quotient(self_1422: Line, other_1210: Point) -> PointAndPlane {
    var self_1423: Line;
    var other_1211: Point;

    self_1423 = self_1422;
    other_1211 = other_1210;
    let _e6: Point = other_1211;
    let _e7: Point = point_inverse(_e6);
    let _e8: Line = self_1423;
    let _e10: Point = other_1211;
    let _e11: Point = point_inverse(_e10);
    let _e12: PointAndPlane = line_point_geometric_product(_e8, _e11);
    return _e12;
}

fn line_point_transformation(self_1424: Line, other_1212: Point) -> Point {
    var self_1425: Line;
    var other_1213: Point;

    self_1425 = self_1424;
    other_1213 = other_1212;
    let _e6: Line = self_1425;
    let _e7: Point = other_1213;
    let _e8: PointAndPlane = line_point_geometric_product(_e6, _e7);
    let _e10: Line = self_1425;
    let _e11: Line = line_reversal(_e10);
    let _e14: Line = self_1425;
    let _e15: Point = other_1213;
    let _e16: PointAndPlane = line_point_geometric_product(_e14, _e15);
    let _e18: Line = self_1425;
    let _e19: Line = line_reversal(_e18);
    let _e20: PointAndPlane = point_and_plane_line_geometric_product(_e16, _e19);
    let _e23: Line = self_1425;
    let _e24: Point = other_1213;
    let _e25: PointAndPlane = line_point_geometric_product(_e23, _e24);
    let _e27: Line = self_1425;
    let _e28: Line = line_reversal(_e27);
    let _e31: Line = self_1425;
    let _e32: Point = other_1213;
    let _e33: PointAndPlane = line_point_geometric_product(_e31, _e32);
    let _e35: Line = self_1425;
    let _e36: Line = line_reversal(_e35);
    let _e37: PointAndPlane = point_and_plane_line_geometric_product(_e33, _e36);
    let _e38: Point = point_and_plane_point_into(_e37);
    return _e38;
}

fn line_point_and_plane_geometric_quotient(self_1426: Line, other_1214: PointAndPlane) -> PointAndPlane {
    var self_1427: Line;
    var other_1215: PointAndPlane;

    self_1427 = self_1426;
    other_1215 = other_1214;
    let _e6: PointAndPlane = other_1215;
    let _e7: PointAndPlane = point_and_plane_inverse(_e6);
    let _e8: Line = self_1427;
    let _e10: PointAndPlane = other_1215;
    let _e11: PointAndPlane = point_and_plane_inverse(_e10);
    let _e12: PointAndPlane = line_point_and_plane_geometric_product(_e8, _e11);
    return _e12;
}

fn line_point_and_plane_transformation(self_1428: Line, other_1216: PointAndPlane) -> PointAndPlane {
    var self_1429: Line;
    var other_1217: PointAndPlane;

    self_1429 = self_1428;
    other_1217 = other_1216;
    let _e6: Line = self_1429;
    let _e7: PointAndPlane = other_1217;
    let _e8: PointAndPlane = line_point_and_plane_geometric_product(_e6, _e7);
    let _e10: Line = self_1429;
    let _e11: Line = line_reversal(_e10);
    let _e14: Line = self_1429;
    let _e15: PointAndPlane = other_1217;
    let _e16: PointAndPlane = line_point_and_plane_geometric_product(_e14, _e15);
    let _e18: Line = self_1429;
    let _e19: Line = line_reversal(_e18);
    let _e20: PointAndPlane = point_and_plane_line_geometric_product(_e16, _e19);
    return _e20;
}

fn line_rotor_geometric_quotient(self_1430: Line, other_1218: Rotor) -> Motor {
    var self_1431: Line;
    var other_1219: Rotor;

    self_1431 = self_1430;
    other_1219 = other_1218;
    let _e6: Rotor = other_1219;
    let _e7: Rotor = rotor_inverse(_e6);
    let _e8: Line = self_1431;
    let _e10: Rotor = other_1219;
    let _e11: Rotor = rotor_inverse(_e10);
    let _e12: Motor = line_rotor_geometric_product(_e8, _e11);
    return _e12;
}

fn line_rotor_transformation(self_1432: Line, other_1220: Rotor) -> Rotor {
    var self_1433: Line;
    var other_1221: Rotor;

    self_1433 = self_1432;
    other_1221 = other_1220;
    let _e6: Line = self_1433;
    let _e7: Rotor = other_1221;
    let _e8: Motor = line_rotor_geometric_product(_e6, _e7);
    let _e10: Line = self_1433;
    let _e11: Line = line_reversal(_e10);
    let _e14: Line = self_1433;
    let _e15: Rotor = other_1221;
    let _e16: Motor = line_rotor_geometric_product(_e14, _e15);
    let _e18: Line = self_1433;
    let _e19: Line = line_reversal(_e18);
    let _e20: Motor = motor_line_geometric_product(_e16, _e19);
    let _e23: Line = self_1433;
    let _e24: Rotor = other_1221;
    let _e25: Motor = line_rotor_geometric_product(_e23, _e24);
    let _e27: Line = self_1433;
    let _e28: Line = line_reversal(_e27);
    let _e31: Line = self_1433;
    let _e32: Rotor = other_1221;
    let _e33: Motor = line_rotor_geometric_product(_e31, _e32);
    let _e35: Line = self_1433;
    let _e36: Line = line_reversal(_e35);
    let _e37: Motor = motor_line_geometric_product(_e33, _e36);
    let _e38: Rotor = motor_rotor_into(_e37);
    return _e38;
}

fn line_scalar_geometric_quotient(self_1434: Line, other_1222: Scalar) -> Line {
    var self_1435: Line;
    var other_1223: Scalar;

    self_1435 = self_1434;
    other_1223 = other_1222;
    let _e6: Scalar = other_1223;
    let _e7: Scalar = scalar_inverse(_e6);
    let _e8: Line = self_1435;
    let _e10: Scalar = other_1223;
    let _e11: Scalar = scalar_inverse(_e10);
    let _e12: Line = line_scalar_geometric_product(_e8, _e11);
    return _e12;
}

fn line_scalar_transformation(self_1436: Line, other_1224: Scalar) -> Scalar {
    var self_1437: Line;
    var other_1225: Scalar;

    self_1437 = self_1436;
    other_1225 = other_1224;
    let _e6: Line = self_1437;
    let _e7: Scalar = other_1225;
    let _e8: Line = line_scalar_geometric_product(_e6, _e7);
    let _e10: Line = self_1437;
    let _e11: Line = line_reversal(_e10);
    let _e14: Line = self_1437;
    let _e15: Scalar = other_1225;
    let _e16: Line = line_scalar_geometric_product(_e14, _e15);
    let _e18: Line = self_1437;
    let _e19: Line = line_reversal(_e18);
    let _e20: Motor = line_line_geometric_product(_e16, _e19);
    let _e23: Line = self_1437;
    let _e24: Scalar = other_1225;
    let _e25: Line = line_scalar_geometric_product(_e23, _e24);
    let _e27: Line = self_1437;
    let _e28: Line = line_reversal(_e27);
    let _e31: Line = self_1437;
    let _e32: Scalar = other_1225;
    let _e33: Line = line_scalar_geometric_product(_e31, _e32);
    let _e35: Line = self_1437;
    let _e36: Line = line_reversal(_e35);
    let _e37: Motor = line_line_geometric_product(_e33, _e36);
    let _e38: Scalar = motor_scalar_into(_e37);
    return _e38;
}

fn line_translator_geometric_quotient(self_1438: Line, other_1226: Translator) -> Motor {
    var self_1439: Line;
    var other_1227: Translator;

    self_1439 = self_1438;
    other_1227 = other_1226;
    let _e6: Translator = other_1227;
    let _e7: Translator = translator_inverse(_e6);
    let _e8: Line = self_1439;
    let _e10: Translator = other_1227;
    let _e11: Translator = translator_inverse(_e10);
    let _e12: Motor = line_translator_geometric_product(_e8, _e11);
    return _e12;
}

fn line_translator_transformation(self_1440: Line, other_1228: Translator) -> Translator {
    var self_1441: Line;
    var other_1229: Translator;

    self_1441 = self_1440;
    other_1229 = other_1228;
    let _e6: Line = self_1441;
    let _e7: Translator = other_1229;
    let _e8: Motor = line_translator_geometric_product(_e6, _e7);
    let _e10: Line = self_1441;
    let _e11: Line = line_reversal(_e10);
    let _e14: Line = self_1441;
    let _e15: Translator = other_1229;
    let _e16: Motor = line_translator_geometric_product(_e14, _e15);
    let _e18: Line = self_1441;
    let _e19: Line = line_reversal(_e18);
    let _e20: Motor = motor_line_geometric_product(_e16, _e19);
    let _e23: Line = self_1441;
    let _e24: Translator = other_1229;
    let _e25: Motor = line_translator_geometric_product(_e23, _e24);
    let _e27: Line = self_1441;
    let _e28: Line = line_reversal(_e27);
    let _e31: Line = self_1441;
    let _e32: Translator = other_1229;
    let _e33: Motor = line_translator_geometric_product(_e31, _e32);
    let _e35: Line = self_1441;
    let _e36: Line = line_reversal(_e35);
    let _e37: Motor = motor_line_geometric_product(_e33, _e36);
    let _e38: Translator = motor_translator_into(_e37);
    return _e38;
}

fn motor_ideal_point_geometric_quotient(self_1442: Motor, other_1230: IdealPoint) -> Motor {
    var self_1443: Motor;
    var other_1231: IdealPoint;

    self_1443 = self_1442;
    other_1231 = other_1230;
    let _e6: IdealPoint = other_1231;
    let _e7: IdealPoint = ideal_point_inverse(_e6);
    let _e8: Motor = self_1443;
    let _e10: IdealPoint = other_1231;
    let _e11: IdealPoint = ideal_point_inverse(_e10);
    let _e12: Motor = motor_ideal_point_geometric_product(_e8, _e11);
    return _e12;
}

fn motor_ideal_point_transformation(self_1444: Motor, other_1232: IdealPoint) -> IdealPoint {
    var self_1445: Motor;
    var other_1233: IdealPoint;

    self_1445 = self_1444;
    other_1233 = other_1232;
    let _e6: Motor = self_1445;
    let _e7: IdealPoint = other_1233;
    let _e8: Motor = motor_ideal_point_geometric_product(_e6, _e7);
    let _e10: Motor = self_1445;
    let _e11: Motor = motor_reversal(_e10);
    let _e14: Motor = self_1445;
    let _e15: IdealPoint = other_1233;
    let _e16: Motor = motor_ideal_point_geometric_product(_e14, _e15);
    let _e18: Motor = self_1445;
    let _e19: Motor = motor_reversal(_e18);
    let _e20: Motor = motor_motor_geometric_product(_e16, _e19);
    let _e23: Motor = self_1445;
    let _e24: IdealPoint = other_1233;
    let _e25: Motor = motor_ideal_point_geometric_product(_e23, _e24);
    let _e27: Motor = self_1445;
    let _e28: Motor = motor_reversal(_e27);
    let _e31: Motor = self_1445;
    let _e32: IdealPoint = other_1233;
    let _e33: Motor = motor_ideal_point_geometric_product(_e31, _e32);
    let _e35: Motor = self_1445;
    let _e36: Motor = motor_reversal(_e35);
    let _e37: Motor = motor_motor_geometric_product(_e33, _e36);
    let _e38: IdealPoint = motor_ideal_point_into(_e37);
    return _e38;
}

fn motor_line_geometric_quotient(self_1446: Motor, other_1234: Line) -> Motor {
    var self_1447: Motor;
    var other_1235: Line;

    self_1447 = self_1446;
    other_1235 = other_1234;
    let _e6: Line = other_1235;
    let _e7: Line = line_inverse(_e6);
    let _e8: Motor = self_1447;
    let _e10: Line = other_1235;
    let _e11: Line = line_inverse(_e10);
    let _e12: Motor = motor_line_geometric_product(_e8, _e11);
    return _e12;
}

fn motor_line_transformation(self_1448: Motor, other_1236: Line) -> Line {
    var self_1449: Motor;
    var other_1237: Line;

    self_1449 = self_1448;
    other_1237 = other_1236;
    let _e6: Motor = self_1449;
    let _e7: Line = other_1237;
    let _e8: Motor = motor_line_geometric_product(_e6, _e7);
    let _e10: Motor = self_1449;
    let _e11: Motor = motor_reversal(_e10);
    let _e14: Motor = self_1449;
    let _e15: Line = other_1237;
    let _e16: Motor = motor_line_geometric_product(_e14, _e15);
    let _e18: Motor = self_1449;
    let _e19: Motor = motor_reversal(_e18);
    let _e20: Motor = motor_motor_geometric_product(_e16, _e19);
    let _e23: Motor = self_1449;
    let _e24: Line = other_1237;
    let _e25: Motor = motor_line_geometric_product(_e23, _e24);
    let _e27: Motor = self_1449;
    let _e28: Motor = motor_reversal(_e27);
    let _e31: Motor = self_1449;
    let _e32: Line = other_1237;
    let _e33: Motor = motor_line_geometric_product(_e31, _e32);
    let _e35: Motor = self_1449;
    let _e36: Motor = motor_reversal(_e35);
    let _e37: Motor = motor_motor_geometric_product(_e33, _e36);
    let _e38: Line = motor_line_into(_e37);
    return _e38;
}

fn motor_powi(self_1450: Motor, exponent: i32) -> Motor {
    var self_1451: Motor;
    var exponent_1: i32;
    var local: Motor;
    var x: Motor;
    var y: Motor;
    var n: i32;

    self_1451 = self_1450;
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
        let _e12: Motor = self_1451;
        let _e13: Motor = motor_inverse(_e12);
        local = _e13;
    } else {
        let _e14: Motor = self_1451;
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

fn motor_motor_geometric_quotient(self_1452: Motor, other_1238: Motor) -> Motor {
    var self_1453: Motor;
    var other_1239: Motor;

    self_1453 = self_1452;
    other_1239 = other_1238;
    let _e6: Motor = other_1239;
    let _e7: Motor = motor_inverse(_e6);
    let _e8: Motor = self_1453;
    let _e10: Motor = other_1239;
    let _e11: Motor = motor_inverse(_e10);
    let _e12: Motor = motor_motor_geometric_product(_e8, _e11);
    return _e12;
}

fn motor_motor_transformation(self_1454: Motor, other_1240: Motor) -> Motor {
    var self_1455: Motor;
    var other_1241: Motor;

    self_1455 = self_1454;
    other_1241 = other_1240;
    let _e6: Motor = self_1455;
    let _e7: Motor = other_1241;
    let _e8: Motor = motor_motor_geometric_product(_e6, _e7);
    let _e10: Motor = self_1455;
    let _e11: Motor = motor_reversal(_e10);
    let _e14: Motor = self_1455;
    let _e15: Motor = other_1241;
    let _e16: Motor = motor_motor_geometric_product(_e14, _e15);
    let _e18: Motor = self_1455;
    let _e19: Motor = motor_reversal(_e18);
    let _e20: Motor = motor_motor_geometric_product(_e16, _e19);
    return _e20;
}

fn motor_multi_vector_geometric_quotient(self_1456: Motor, other_1242: MultiVector) -> MultiVector {
    var self_1457: Motor;
    var other_1243: MultiVector;

    self_1457 = self_1456;
    other_1243 = other_1242;
    let _e6: MultiVector = other_1243;
    let _e7: MultiVector = multi_vector_inverse(_e6);
    let _e8: Motor = self_1457;
    let _e10: MultiVector = other_1243;
    let _e11: MultiVector = multi_vector_inverse(_e10);
    let _e12: MultiVector = motor_multi_vector_geometric_product(_e8, _e11);
    return _e12;
}

fn motor_multi_vector_transformation(self_1458: Motor, other_1244: MultiVector) -> MultiVector {
    var self_1459: Motor;
    var other_1245: MultiVector;

    self_1459 = self_1458;
    other_1245 = other_1244;
    let _e6: Motor = self_1459;
    let _e7: MultiVector = other_1245;
    let _e8: MultiVector = motor_multi_vector_geometric_product(_e6, _e7);
    let _e10: Motor = self_1459;
    let _e11: Motor = motor_reversal(_e10);
    let _e14: Motor = self_1459;
    let _e15: MultiVector = other_1245;
    let _e16: MultiVector = motor_multi_vector_geometric_product(_e14, _e15);
    let _e18: Motor = self_1459;
    let _e19: Motor = motor_reversal(_e18);
    let _e20: MultiVector = multi_vector_motor_geometric_product(_e16, _e19);
    return _e20;
}

fn motor_plane_geometric_quotient(self_1460: Motor, other_1246: Plane) -> PointAndPlane {
    var self_1461: Motor;
    var other_1247: Plane;

    self_1461 = self_1460;
    other_1247 = other_1246;
    let _e6: Plane = other_1247;
    let _e7: Plane = plane_inverse(_e6);
    let _e8: Motor = self_1461;
    let _e10: Plane = other_1247;
    let _e11: Plane = plane_inverse(_e10);
    let _e12: PointAndPlane = motor_plane_geometric_product(_e8, _e11);
    return _e12;
}

fn motor_plane_transformation(self_1462: Motor, other_1248: Plane) -> Plane {
    var self_1463: Motor;
    var other_1249: Plane;

    self_1463 = self_1462;
    other_1249 = other_1248;
    let _e6: Motor = self_1463;
    let _e7: Plane = other_1249;
    let _e8: PointAndPlane = motor_plane_geometric_product(_e6, _e7);
    let _e10: Motor = self_1463;
    let _e11: Motor = motor_reversal(_e10);
    let _e14: Motor = self_1463;
    let _e15: Plane = other_1249;
    let _e16: PointAndPlane = motor_plane_geometric_product(_e14, _e15);
    let _e18: Motor = self_1463;
    let _e19: Motor = motor_reversal(_e18);
    let _e20: PointAndPlane = point_and_plane_motor_geometric_product(_e16, _e19);
    let _e23: Motor = self_1463;
    let _e24: Plane = other_1249;
    let _e25: PointAndPlane = motor_plane_geometric_product(_e23, _e24);
    let _e27: Motor = self_1463;
    let _e28: Motor = motor_reversal(_e27);
    let _e31: Motor = self_1463;
    let _e32: Plane = other_1249;
    let _e33: PointAndPlane = motor_plane_geometric_product(_e31, _e32);
    let _e35: Motor = self_1463;
    let _e36: Motor = motor_reversal(_e35);
    let _e37: PointAndPlane = point_and_plane_motor_geometric_product(_e33, _e36);
    let _e38: Plane = point_and_plane_plane_into(_e37);
    return _e38;
}

fn motor_point_geometric_quotient(self_1464: Motor, other_1250: Point) -> PointAndPlane {
    var self_1465: Motor;
    var other_1251: Point;

    self_1465 = self_1464;
    other_1251 = other_1250;
    let _e6: Point = other_1251;
    let _e7: Point = point_inverse(_e6);
    let _e8: Motor = self_1465;
    let _e10: Point = other_1251;
    let _e11: Point = point_inverse(_e10);
    let _e12: PointAndPlane = motor_point_geometric_product(_e8, _e11);
    return _e12;
}

fn motor_point_transformation(self_1466: Motor, other_1252: Point) -> Point {
    var self_1467: Motor;
    var other_1253: Point;

    self_1467 = self_1466;
    other_1253 = other_1252;
    let _e6: Motor = self_1467;
    let _e7: Point = other_1253;
    let _e8: PointAndPlane = motor_point_geometric_product(_e6, _e7);
    let _e10: Motor = self_1467;
    let _e11: Motor = motor_reversal(_e10);
    let _e14: Motor = self_1467;
    let _e15: Point = other_1253;
    let _e16: PointAndPlane = motor_point_geometric_product(_e14, _e15);
    let _e18: Motor = self_1467;
    let _e19: Motor = motor_reversal(_e18);
    let _e20: PointAndPlane = point_and_plane_motor_geometric_product(_e16, _e19);
    let _e23: Motor = self_1467;
    let _e24: Point = other_1253;
    let _e25: PointAndPlane = motor_point_geometric_product(_e23, _e24);
    let _e27: Motor = self_1467;
    let _e28: Motor = motor_reversal(_e27);
    let _e31: Motor = self_1467;
    let _e32: Point = other_1253;
    let _e33: PointAndPlane = motor_point_geometric_product(_e31, _e32);
    let _e35: Motor = self_1467;
    let _e36: Motor = motor_reversal(_e35);
    let _e37: PointAndPlane = point_and_plane_motor_geometric_product(_e33, _e36);
    let _e38: Point = point_and_plane_point_into(_e37);
    return _e38;
}

fn motor_point_and_plane_geometric_quotient(self_1468: Motor, other_1254: PointAndPlane) -> PointAndPlane {
    var self_1469: Motor;
    var other_1255: PointAndPlane;

    self_1469 = self_1468;
    other_1255 = other_1254;
    let _e6: PointAndPlane = other_1255;
    let _e7: PointAndPlane = point_and_plane_inverse(_e6);
    let _e8: Motor = self_1469;
    let _e10: PointAndPlane = other_1255;
    let _e11: PointAndPlane = point_and_plane_inverse(_e10);
    let _e12: PointAndPlane = motor_point_and_plane_geometric_product(_e8, _e11);
    return _e12;
}

fn motor_point_and_plane_transformation(self_1470: Motor, other_1256: PointAndPlane) -> PointAndPlane {
    var self_1471: Motor;
    var other_1257: PointAndPlane;

    self_1471 = self_1470;
    other_1257 = other_1256;
    let _e6: Motor = self_1471;
    let _e7: PointAndPlane = other_1257;
    let _e8: PointAndPlane = motor_point_and_plane_geometric_product(_e6, _e7);
    let _e10: Motor = self_1471;
    let _e11: Motor = motor_reversal(_e10);
    let _e14: Motor = self_1471;
    let _e15: PointAndPlane = other_1257;
    let _e16: PointAndPlane = motor_point_and_plane_geometric_product(_e14, _e15);
    let _e18: Motor = self_1471;
    let _e19: Motor = motor_reversal(_e18);
    let _e20: PointAndPlane = point_and_plane_motor_geometric_product(_e16, _e19);
    return _e20;
}

fn motor_rotor_geometric_quotient(self_1472: Motor, other_1258: Rotor) -> Motor {
    var self_1473: Motor;
    var other_1259: Rotor;

    self_1473 = self_1472;
    other_1259 = other_1258;
    let _e6: Rotor = other_1259;
    let _e7: Rotor = rotor_inverse(_e6);
    let _e8: Motor = self_1473;
    let _e10: Rotor = other_1259;
    let _e11: Rotor = rotor_inverse(_e10);
    let _e12: Motor = motor_rotor_geometric_product(_e8, _e11);
    return _e12;
}

fn motor_rotor_transformation(self_1474: Motor, other_1260: Rotor) -> Rotor {
    var self_1475: Motor;
    var other_1261: Rotor;

    self_1475 = self_1474;
    other_1261 = other_1260;
    let _e6: Motor = self_1475;
    let _e7: Rotor = other_1261;
    let _e8: Motor = motor_rotor_geometric_product(_e6, _e7);
    let _e10: Motor = self_1475;
    let _e11: Motor = motor_reversal(_e10);
    let _e14: Motor = self_1475;
    let _e15: Rotor = other_1261;
    let _e16: Motor = motor_rotor_geometric_product(_e14, _e15);
    let _e18: Motor = self_1475;
    let _e19: Motor = motor_reversal(_e18);
    let _e20: Motor = motor_motor_geometric_product(_e16, _e19);
    let _e23: Motor = self_1475;
    let _e24: Rotor = other_1261;
    let _e25: Motor = motor_rotor_geometric_product(_e23, _e24);
    let _e27: Motor = self_1475;
    let _e28: Motor = motor_reversal(_e27);
    let _e31: Motor = self_1475;
    let _e32: Rotor = other_1261;
    let _e33: Motor = motor_rotor_geometric_product(_e31, _e32);
    let _e35: Motor = self_1475;
    let _e36: Motor = motor_reversal(_e35);
    let _e37: Motor = motor_motor_geometric_product(_e33, _e36);
    let _e38: Rotor = motor_rotor_into(_e37);
    return _e38;
}

fn motor_scalar_geometric_quotient(self_1476: Motor, other_1262: Scalar) -> Motor {
    var self_1477: Motor;
    var other_1263: Scalar;

    self_1477 = self_1476;
    other_1263 = other_1262;
    let _e6: Scalar = other_1263;
    let _e7: Scalar = scalar_inverse(_e6);
    let _e8: Motor = self_1477;
    let _e10: Scalar = other_1263;
    let _e11: Scalar = scalar_inverse(_e10);
    let _e12: Motor = motor_scalar_geometric_product(_e8, _e11);
    return _e12;
}

fn motor_scalar_transformation(self_1478: Motor, other_1264: Scalar) -> Scalar {
    var self_1479: Motor;
    var other_1265: Scalar;

    self_1479 = self_1478;
    other_1265 = other_1264;
    let _e6: Motor = self_1479;
    let _e7: Scalar = other_1265;
    let _e8: Motor = motor_scalar_geometric_product(_e6, _e7);
    let _e10: Motor = self_1479;
    let _e11: Motor = motor_reversal(_e10);
    let _e14: Motor = self_1479;
    let _e15: Scalar = other_1265;
    let _e16: Motor = motor_scalar_geometric_product(_e14, _e15);
    let _e18: Motor = self_1479;
    let _e19: Motor = motor_reversal(_e18);
    let _e20: Motor = motor_motor_geometric_product(_e16, _e19);
    let _e23: Motor = self_1479;
    let _e24: Scalar = other_1265;
    let _e25: Motor = motor_scalar_geometric_product(_e23, _e24);
    let _e27: Motor = self_1479;
    let _e28: Motor = motor_reversal(_e27);
    let _e31: Motor = self_1479;
    let _e32: Scalar = other_1265;
    let _e33: Motor = motor_scalar_geometric_product(_e31, _e32);
    let _e35: Motor = self_1479;
    let _e36: Motor = motor_reversal(_e35);
    let _e37: Motor = motor_motor_geometric_product(_e33, _e36);
    let _e38: Scalar = motor_scalar_into(_e37);
    return _e38;
}

fn motor_translator_geometric_quotient(self_1480: Motor, other_1266: Translator) -> Motor {
    var self_1481: Motor;
    var other_1267: Translator;

    self_1481 = self_1480;
    other_1267 = other_1266;
    let _e6: Translator = other_1267;
    let _e7: Translator = translator_inverse(_e6);
    let _e8: Motor = self_1481;
    let _e10: Translator = other_1267;
    let _e11: Translator = translator_inverse(_e10);
    let _e12: Motor = motor_translator_geometric_product(_e8, _e11);
    return _e12;
}

fn motor_translator_transformation(self_1482: Motor, other_1268: Translator) -> Translator {
    var self_1483: Motor;
    var other_1269: Translator;

    self_1483 = self_1482;
    other_1269 = other_1268;
    let _e6: Motor = self_1483;
    let _e7: Translator = other_1269;
    let _e8: Motor = motor_translator_geometric_product(_e6, _e7);
    let _e10: Motor = self_1483;
    let _e11: Motor = motor_reversal(_e10);
    let _e14: Motor = self_1483;
    let _e15: Translator = other_1269;
    let _e16: Motor = motor_translator_geometric_product(_e14, _e15);
    let _e18: Motor = self_1483;
    let _e19: Motor = motor_reversal(_e18);
    let _e20: Motor = motor_motor_geometric_product(_e16, _e19);
    let _e23: Motor = self_1483;
    let _e24: Translator = other_1269;
    let _e25: Motor = motor_translator_geometric_product(_e23, _e24);
    let _e27: Motor = self_1483;
    let _e28: Motor = motor_reversal(_e27);
    let _e31: Motor = self_1483;
    let _e32: Translator = other_1269;
    let _e33: Motor = motor_translator_geometric_product(_e31, _e32);
    let _e35: Motor = self_1483;
    let _e36: Motor = motor_reversal(_e35);
    let _e37: Motor = motor_motor_geometric_product(_e33, _e36);
    let _e38: Translator = motor_translator_into(_e37);
    return _e38;
}

fn multi_vector_ideal_point_geometric_quotient(self_1484: MultiVector, other_1270: IdealPoint) -> MultiVector {
    var self_1485: MultiVector;
    var other_1271: IdealPoint;

    self_1485 = self_1484;
    other_1271 = other_1270;
    let _e6: IdealPoint = other_1271;
    let _e7: IdealPoint = ideal_point_inverse(_e6);
    let _e8: MultiVector = self_1485;
    let _e10: IdealPoint = other_1271;
    let _e11: IdealPoint = ideal_point_inverse(_e10);
    let _e12: MultiVector = multi_vector_ideal_point_geometric_product(_e8, _e11);
    return _e12;
}

fn multi_vector_ideal_point_transformation(self_1486: MultiVector, other_1272: IdealPoint) -> IdealPoint {
    var self_1487: MultiVector;
    var other_1273: IdealPoint;

    self_1487 = self_1486;
    other_1273 = other_1272;
    let _e6: MultiVector = self_1487;
    let _e7: IdealPoint = other_1273;
    let _e8: MultiVector = multi_vector_ideal_point_geometric_product(_e6, _e7);
    let _e10: MultiVector = self_1487;
    let _e11: MultiVector = multi_vector_reversal(_e10);
    let _e14: MultiVector = self_1487;
    let _e15: IdealPoint = other_1273;
    let _e16: MultiVector = multi_vector_ideal_point_geometric_product(_e14, _e15);
    let _e18: MultiVector = self_1487;
    let _e19: MultiVector = multi_vector_reversal(_e18);
    let _e20: MultiVector = multi_vector_multi_vector_geometric_product(_e16, _e19);
    let _e23: MultiVector = self_1487;
    let _e24: IdealPoint = other_1273;
    let _e25: MultiVector = multi_vector_ideal_point_geometric_product(_e23, _e24);
    let _e27: MultiVector = self_1487;
    let _e28: MultiVector = multi_vector_reversal(_e27);
    let _e31: MultiVector = self_1487;
    let _e32: IdealPoint = other_1273;
    let _e33: MultiVector = multi_vector_ideal_point_geometric_product(_e31, _e32);
    let _e35: MultiVector = self_1487;
    let _e36: MultiVector = multi_vector_reversal(_e35);
    let _e37: MultiVector = multi_vector_multi_vector_geometric_product(_e33, _e36);
    let _e38: IdealPoint = multi_vector_ideal_point_into(_e37);
    return _e38;
}

fn multi_vector_line_geometric_quotient(self_1488: MultiVector, other_1274: Line) -> MultiVector {
    var self_1489: MultiVector;
    var other_1275: Line;

    self_1489 = self_1488;
    other_1275 = other_1274;
    let _e6: Line = other_1275;
    let _e7: Line = line_inverse(_e6);
    let _e8: MultiVector = self_1489;
    let _e10: Line = other_1275;
    let _e11: Line = line_inverse(_e10);
    let _e12: MultiVector = multi_vector_line_geometric_product(_e8, _e11);
    return _e12;
}

fn multi_vector_line_transformation(self_1490: MultiVector, other_1276: Line) -> Line {
    var self_1491: MultiVector;
    var other_1277: Line;

    self_1491 = self_1490;
    other_1277 = other_1276;
    let _e6: MultiVector = self_1491;
    let _e7: Line = other_1277;
    let _e8: MultiVector = multi_vector_line_geometric_product(_e6, _e7);
    let _e10: MultiVector = self_1491;
    let _e11: MultiVector = multi_vector_reversal(_e10);
    let _e14: MultiVector = self_1491;
    let _e15: Line = other_1277;
    let _e16: MultiVector = multi_vector_line_geometric_product(_e14, _e15);
    let _e18: MultiVector = self_1491;
    let _e19: MultiVector = multi_vector_reversal(_e18);
    let _e20: MultiVector = multi_vector_multi_vector_geometric_product(_e16, _e19);
    let _e23: MultiVector = self_1491;
    let _e24: Line = other_1277;
    let _e25: MultiVector = multi_vector_line_geometric_product(_e23, _e24);
    let _e27: MultiVector = self_1491;
    let _e28: MultiVector = multi_vector_reversal(_e27);
    let _e31: MultiVector = self_1491;
    let _e32: Line = other_1277;
    let _e33: MultiVector = multi_vector_line_geometric_product(_e31, _e32);
    let _e35: MultiVector = self_1491;
    let _e36: MultiVector = multi_vector_reversal(_e35);
    let _e37: MultiVector = multi_vector_multi_vector_geometric_product(_e33, _e36);
    let _e38: Line = multi_vector_line_into(_e37);
    return _e38;
}

fn multi_vector_motor_geometric_quotient(self_1492: MultiVector, other_1278: Motor) -> MultiVector {
    var self_1493: MultiVector;
    var other_1279: Motor;

    self_1493 = self_1492;
    other_1279 = other_1278;
    let _e6: Motor = other_1279;
    let _e7: Motor = motor_inverse(_e6);
    let _e8: MultiVector = self_1493;
    let _e10: Motor = other_1279;
    let _e11: Motor = motor_inverse(_e10);
    let _e12: MultiVector = multi_vector_motor_geometric_product(_e8, _e11);
    return _e12;
}

fn multi_vector_motor_transformation(self_1494: MultiVector, other_1280: Motor) -> Motor {
    var self_1495: MultiVector;
    var other_1281: Motor;

    self_1495 = self_1494;
    other_1281 = other_1280;
    let _e6: MultiVector = self_1495;
    let _e7: Motor = other_1281;
    let _e8: MultiVector = multi_vector_motor_geometric_product(_e6, _e7);
    let _e10: MultiVector = self_1495;
    let _e11: MultiVector = multi_vector_reversal(_e10);
    let _e14: MultiVector = self_1495;
    let _e15: Motor = other_1281;
    let _e16: MultiVector = multi_vector_motor_geometric_product(_e14, _e15);
    let _e18: MultiVector = self_1495;
    let _e19: MultiVector = multi_vector_reversal(_e18);
    let _e20: MultiVector = multi_vector_multi_vector_geometric_product(_e16, _e19);
    let _e23: MultiVector = self_1495;
    let _e24: Motor = other_1281;
    let _e25: MultiVector = multi_vector_motor_geometric_product(_e23, _e24);
    let _e27: MultiVector = self_1495;
    let _e28: MultiVector = multi_vector_reversal(_e27);
    let _e31: MultiVector = self_1495;
    let _e32: Motor = other_1281;
    let _e33: MultiVector = multi_vector_motor_geometric_product(_e31, _e32);
    let _e35: MultiVector = self_1495;
    let _e36: MultiVector = multi_vector_reversal(_e35);
    let _e37: MultiVector = multi_vector_multi_vector_geometric_product(_e33, _e36);
    let _e38: Motor = multi_vector_motor_into(_e37);
    return _e38;
}

fn multi_vector_powi(self_1496: MultiVector, exponent_2: i32) -> MultiVector {
    var self_1497: MultiVector;
    var exponent_3: i32;
    var local_1: MultiVector;
    var x_1: MultiVector;
    var y_1: MultiVector;
    var n_1: i32;

    self_1497 = self_1496;
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
        let _e12: MultiVector = self_1497;
        let _e13: MultiVector = multi_vector_inverse(_e12);
        local_1 = _e13;
    } else {
        let _e14: MultiVector = self_1497;
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

fn multi_vector_multi_vector_geometric_quotient(self_1498: MultiVector, other_1282: MultiVector) -> MultiVector {
    var self_1499: MultiVector;
    var other_1283: MultiVector;

    self_1499 = self_1498;
    other_1283 = other_1282;
    let _e6: MultiVector = other_1283;
    let _e7: MultiVector = multi_vector_inverse(_e6);
    let _e8: MultiVector = self_1499;
    let _e10: MultiVector = other_1283;
    let _e11: MultiVector = multi_vector_inverse(_e10);
    let _e12: MultiVector = multi_vector_multi_vector_geometric_product(_e8, _e11);
    return _e12;
}

fn multi_vector_multi_vector_transformation(self_1500: MultiVector, other_1284: MultiVector) -> MultiVector {
    var self_1501: MultiVector;
    var other_1285: MultiVector;

    self_1501 = self_1500;
    other_1285 = other_1284;
    let _e6: MultiVector = self_1501;
    let _e7: MultiVector = other_1285;
    let _e8: MultiVector = multi_vector_multi_vector_geometric_product(_e6, _e7);
    let _e10: MultiVector = self_1501;
    let _e11: MultiVector = multi_vector_reversal(_e10);
    let _e14: MultiVector = self_1501;
    let _e15: MultiVector = other_1285;
    let _e16: MultiVector = multi_vector_multi_vector_geometric_product(_e14, _e15);
    let _e18: MultiVector = self_1501;
    let _e19: MultiVector = multi_vector_reversal(_e18);
    let _e20: MultiVector = multi_vector_multi_vector_geometric_product(_e16, _e19);
    return _e20;
}

fn multi_vector_plane_geometric_quotient(self_1502: MultiVector, other_1286: Plane) -> MultiVector {
    var self_1503: MultiVector;
    var other_1287: Plane;

    self_1503 = self_1502;
    other_1287 = other_1286;
    let _e6: Plane = other_1287;
    let _e7: Plane = plane_inverse(_e6);
    let _e8: MultiVector = self_1503;
    let _e10: Plane = other_1287;
    let _e11: Plane = plane_inverse(_e10);
    let _e12: MultiVector = multi_vector_plane_geometric_product(_e8, _e11);
    return _e12;
}

fn multi_vector_plane_transformation(self_1504: MultiVector, other_1288: Plane) -> Plane {
    var self_1505: MultiVector;
    var other_1289: Plane;

    self_1505 = self_1504;
    other_1289 = other_1288;
    let _e6: MultiVector = self_1505;
    let _e7: Plane = other_1289;
    let _e8: MultiVector = multi_vector_plane_geometric_product(_e6, _e7);
    let _e10: MultiVector = self_1505;
    let _e11: MultiVector = multi_vector_reversal(_e10);
    let _e14: MultiVector = self_1505;
    let _e15: Plane = other_1289;
    let _e16: MultiVector = multi_vector_plane_geometric_product(_e14, _e15);
    let _e18: MultiVector = self_1505;
    let _e19: MultiVector = multi_vector_reversal(_e18);
    let _e20: MultiVector = multi_vector_multi_vector_geometric_product(_e16, _e19);
    let _e23: MultiVector = self_1505;
    let _e24: Plane = other_1289;
    let _e25: MultiVector = multi_vector_plane_geometric_product(_e23, _e24);
    let _e27: MultiVector = self_1505;
    let _e28: MultiVector = multi_vector_reversal(_e27);
    let _e31: MultiVector = self_1505;
    let _e32: Plane = other_1289;
    let _e33: MultiVector = multi_vector_plane_geometric_product(_e31, _e32);
    let _e35: MultiVector = self_1505;
    let _e36: MultiVector = multi_vector_reversal(_e35);
    let _e37: MultiVector = multi_vector_multi_vector_geometric_product(_e33, _e36);
    let _e38: Plane = multi_vector_plane_into(_e37);
    return _e38;
}

fn multi_vector_point_geometric_quotient(self_1506: MultiVector, other_1290: Point) -> MultiVector {
    var self_1507: MultiVector;
    var other_1291: Point;

    self_1507 = self_1506;
    other_1291 = other_1290;
    let _e6: Point = other_1291;
    let _e7: Point = point_inverse(_e6);
    let _e8: MultiVector = self_1507;
    let _e10: Point = other_1291;
    let _e11: Point = point_inverse(_e10);
    let _e12: MultiVector = multi_vector_point_geometric_product(_e8, _e11);
    return _e12;
}

fn multi_vector_point_transformation(self_1508: MultiVector, other_1292: Point) -> Point {
    var self_1509: MultiVector;
    var other_1293: Point;

    self_1509 = self_1508;
    other_1293 = other_1292;
    let _e6: MultiVector = self_1509;
    let _e7: Point = other_1293;
    let _e8: MultiVector = multi_vector_point_geometric_product(_e6, _e7);
    let _e10: MultiVector = self_1509;
    let _e11: MultiVector = multi_vector_reversal(_e10);
    let _e14: MultiVector = self_1509;
    let _e15: Point = other_1293;
    let _e16: MultiVector = multi_vector_point_geometric_product(_e14, _e15);
    let _e18: MultiVector = self_1509;
    let _e19: MultiVector = multi_vector_reversal(_e18);
    let _e20: MultiVector = multi_vector_multi_vector_geometric_product(_e16, _e19);
    let _e23: MultiVector = self_1509;
    let _e24: Point = other_1293;
    let _e25: MultiVector = multi_vector_point_geometric_product(_e23, _e24);
    let _e27: MultiVector = self_1509;
    let _e28: MultiVector = multi_vector_reversal(_e27);
    let _e31: MultiVector = self_1509;
    let _e32: Point = other_1293;
    let _e33: MultiVector = multi_vector_point_geometric_product(_e31, _e32);
    let _e35: MultiVector = self_1509;
    let _e36: MultiVector = multi_vector_reversal(_e35);
    let _e37: MultiVector = multi_vector_multi_vector_geometric_product(_e33, _e36);
    let _e38: Point = multi_vector_point_into(_e37);
    return _e38;
}

fn multi_vector_point_and_plane_geometric_quotient(self_1510: MultiVector, other_1294: PointAndPlane) -> MultiVector {
    var self_1511: MultiVector;
    var other_1295: PointAndPlane;

    self_1511 = self_1510;
    other_1295 = other_1294;
    let _e6: PointAndPlane = other_1295;
    let _e7: PointAndPlane = point_and_plane_inverse(_e6);
    let _e8: MultiVector = self_1511;
    let _e10: PointAndPlane = other_1295;
    let _e11: PointAndPlane = point_and_plane_inverse(_e10);
    let _e12: MultiVector = multi_vector_point_and_plane_geometric_product(_e8, _e11);
    return _e12;
}

fn multi_vector_point_and_plane_transformation(self_1512: MultiVector, other_1296: PointAndPlane) -> PointAndPlane {
    var self_1513: MultiVector;
    var other_1297: PointAndPlane;

    self_1513 = self_1512;
    other_1297 = other_1296;
    let _e6: MultiVector = self_1513;
    let _e7: PointAndPlane = other_1297;
    let _e8: MultiVector = multi_vector_point_and_plane_geometric_product(_e6, _e7);
    let _e10: MultiVector = self_1513;
    let _e11: MultiVector = multi_vector_reversal(_e10);
    let _e14: MultiVector = self_1513;
    let _e15: PointAndPlane = other_1297;
    let _e16: MultiVector = multi_vector_point_and_plane_geometric_product(_e14, _e15);
    let _e18: MultiVector = self_1513;
    let _e19: MultiVector = multi_vector_reversal(_e18);
    let _e20: MultiVector = multi_vector_multi_vector_geometric_product(_e16, _e19);
    let _e23: MultiVector = self_1513;
    let _e24: PointAndPlane = other_1297;
    let _e25: MultiVector = multi_vector_point_and_plane_geometric_product(_e23, _e24);
    let _e27: MultiVector = self_1513;
    let _e28: MultiVector = multi_vector_reversal(_e27);
    let _e31: MultiVector = self_1513;
    let _e32: PointAndPlane = other_1297;
    let _e33: MultiVector = multi_vector_point_and_plane_geometric_product(_e31, _e32);
    let _e35: MultiVector = self_1513;
    let _e36: MultiVector = multi_vector_reversal(_e35);
    let _e37: MultiVector = multi_vector_multi_vector_geometric_product(_e33, _e36);
    let _e38: PointAndPlane = multi_vector_point_and_plane_into(_e37);
    return _e38;
}

fn multi_vector_rotor_geometric_quotient(self_1514: MultiVector, other_1298: Rotor) -> MultiVector {
    var self_1515: MultiVector;
    var other_1299: Rotor;

    self_1515 = self_1514;
    other_1299 = other_1298;
    let _e6: Rotor = other_1299;
    let _e7: Rotor = rotor_inverse(_e6);
    let _e8: MultiVector = self_1515;
    let _e10: Rotor = other_1299;
    let _e11: Rotor = rotor_inverse(_e10);
    let _e12: MultiVector = multi_vector_rotor_geometric_product(_e8, _e11);
    return _e12;
}

fn multi_vector_rotor_transformation(self_1516: MultiVector, other_1300: Rotor) -> Rotor {
    var self_1517: MultiVector;
    var other_1301: Rotor;

    self_1517 = self_1516;
    other_1301 = other_1300;
    let _e6: MultiVector = self_1517;
    let _e7: Rotor = other_1301;
    let _e8: MultiVector = multi_vector_rotor_geometric_product(_e6, _e7);
    let _e10: MultiVector = self_1517;
    let _e11: MultiVector = multi_vector_reversal(_e10);
    let _e14: MultiVector = self_1517;
    let _e15: Rotor = other_1301;
    let _e16: MultiVector = multi_vector_rotor_geometric_product(_e14, _e15);
    let _e18: MultiVector = self_1517;
    let _e19: MultiVector = multi_vector_reversal(_e18);
    let _e20: MultiVector = multi_vector_multi_vector_geometric_product(_e16, _e19);
    let _e23: MultiVector = self_1517;
    let _e24: Rotor = other_1301;
    let _e25: MultiVector = multi_vector_rotor_geometric_product(_e23, _e24);
    let _e27: MultiVector = self_1517;
    let _e28: MultiVector = multi_vector_reversal(_e27);
    let _e31: MultiVector = self_1517;
    let _e32: Rotor = other_1301;
    let _e33: MultiVector = multi_vector_rotor_geometric_product(_e31, _e32);
    let _e35: MultiVector = self_1517;
    let _e36: MultiVector = multi_vector_reversal(_e35);
    let _e37: MultiVector = multi_vector_multi_vector_geometric_product(_e33, _e36);
    let _e38: Rotor = multi_vector_rotor_into(_e37);
    return _e38;
}

fn multi_vector_scalar_geometric_quotient(self_1518: MultiVector, other_1302: Scalar) -> MultiVector {
    var self_1519: MultiVector;
    var other_1303: Scalar;

    self_1519 = self_1518;
    other_1303 = other_1302;
    let _e6: Scalar = other_1303;
    let _e7: Scalar = scalar_inverse(_e6);
    let _e8: MultiVector = self_1519;
    let _e10: Scalar = other_1303;
    let _e11: Scalar = scalar_inverse(_e10);
    let _e12: MultiVector = multi_vector_scalar_geometric_product(_e8, _e11);
    return _e12;
}

fn multi_vector_scalar_transformation(self_1520: MultiVector, other_1304: Scalar) -> Scalar {
    var self_1521: MultiVector;
    var other_1305: Scalar;

    self_1521 = self_1520;
    other_1305 = other_1304;
    let _e6: MultiVector = self_1521;
    let _e7: Scalar = other_1305;
    let _e8: MultiVector = multi_vector_scalar_geometric_product(_e6, _e7);
    let _e10: MultiVector = self_1521;
    let _e11: MultiVector = multi_vector_reversal(_e10);
    let _e14: MultiVector = self_1521;
    let _e15: Scalar = other_1305;
    let _e16: MultiVector = multi_vector_scalar_geometric_product(_e14, _e15);
    let _e18: MultiVector = self_1521;
    let _e19: MultiVector = multi_vector_reversal(_e18);
    let _e20: MultiVector = multi_vector_multi_vector_geometric_product(_e16, _e19);
    let _e23: MultiVector = self_1521;
    let _e24: Scalar = other_1305;
    let _e25: MultiVector = multi_vector_scalar_geometric_product(_e23, _e24);
    let _e27: MultiVector = self_1521;
    let _e28: MultiVector = multi_vector_reversal(_e27);
    let _e31: MultiVector = self_1521;
    let _e32: Scalar = other_1305;
    let _e33: MultiVector = multi_vector_scalar_geometric_product(_e31, _e32);
    let _e35: MultiVector = self_1521;
    let _e36: MultiVector = multi_vector_reversal(_e35);
    let _e37: MultiVector = multi_vector_multi_vector_geometric_product(_e33, _e36);
    let _e38: Scalar = multi_vector_scalar_into(_e37);
    return _e38;
}

fn multi_vector_translator_geometric_quotient(self_1522: MultiVector, other_1306: Translator) -> MultiVector {
    var self_1523: MultiVector;
    var other_1307: Translator;

    self_1523 = self_1522;
    other_1307 = other_1306;
    let _e6: Translator = other_1307;
    let _e7: Translator = translator_inverse(_e6);
    let _e8: MultiVector = self_1523;
    let _e10: Translator = other_1307;
    let _e11: Translator = translator_inverse(_e10);
    let _e12: MultiVector = multi_vector_translator_geometric_product(_e8, _e11);
    return _e12;
}

fn multi_vector_translator_transformation(self_1524: MultiVector, other_1308: Translator) -> Translator {
    var self_1525: MultiVector;
    var other_1309: Translator;

    self_1525 = self_1524;
    other_1309 = other_1308;
    let _e6: MultiVector = self_1525;
    let _e7: Translator = other_1309;
    let _e8: MultiVector = multi_vector_translator_geometric_product(_e6, _e7);
    let _e10: MultiVector = self_1525;
    let _e11: MultiVector = multi_vector_reversal(_e10);
    let _e14: MultiVector = self_1525;
    let _e15: Translator = other_1309;
    let _e16: MultiVector = multi_vector_translator_geometric_product(_e14, _e15);
    let _e18: MultiVector = self_1525;
    let _e19: MultiVector = multi_vector_reversal(_e18);
    let _e20: MultiVector = multi_vector_multi_vector_geometric_product(_e16, _e19);
    let _e23: MultiVector = self_1525;
    let _e24: Translator = other_1309;
    let _e25: MultiVector = multi_vector_translator_geometric_product(_e23, _e24);
    let _e27: MultiVector = self_1525;
    let _e28: MultiVector = multi_vector_reversal(_e27);
    let _e31: MultiVector = self_1525;
    let _e32: Translator = other_1309;
    let _e33: MultiVector = multi_vector_translator_geometric_product(_e31, _e32);
    let _e35: MultiVector = self_1525;
    let _e36: MultiVector = multi_vector_reversal(_e35);
    let _e37: MultiVector = multi_vector_multi_vector_geometric_product(_e33, _e36);
    let _e38: Translator = multi_vector_translator_into(_e37);
    return _e38;
}

fn plane_line_geometric_quotient(self_1526: Plane, other_1310: Line) -> PointAndPlane {
    var self_1527: Plane;
    var other_1311: Line;

    self_1527 = self_1526;
    other_1311 = other_1310;
    let _e6: Line = other_1311;
    let _e7: Line = line_inverse(_e6);
    let _e8: Plane = self_1527;
    let _e10: Line = other_1311;
    let _e11: Line = line_inverse(_e10);
    let _e12: PointAndPlane = plane_line_geometric_product(_e8, _e11);
    return _e12;
}

fn plane_line_transformation(self_1528: Plane, other_1312: Line) -> Line {
    var self_1529: Plane;
    var other_1313: Line;

    self_1529 = self_1528;
    other_1313 = other_1312;
    let _e6: Plane = self_1529;
    let _e7: Line = other_1313;
    let _e8: PointAndPlane = plane_line_geometric_product(_e6, _e7);
    let _e10: Plane = self_1529;
    let _e11: Plane = plane_reversal(_e10);
    let _e14: Plane = self_1529;
    let _e15: Line = other_1313;
    let _e16: PointAndPlane = plane_line_geometric_product(_e14, _e15);
    let _e18: Plane = self_1529;
    let _e19: Plane = plane_reversal(_e18);
    let _e20: Motor = point_and_plane_plane_geometric_product(_e16, _e19);
    let _e23: Plane = self_1529;
    let _e24: Line = other_1313;
    let _e25: PointAndPlane = plane_line_geometric_product(_e23, _e24);
    let _e27: Plane = self_1529;
    let _e28: Plane = plane_reversal(_e27);
    let _e31: Plane = self_1529;
    let _e32: Line = other_1313;
    let _e33: PointAndPlane = plane_line_geometric_product(_e31, _e32);
    let _e35: Plane = self_1529;
    let _e36: Plane = plane_reversal(_e35);
    let _e37: Motor = point_and_plane_plane_geometric_product(_e33, _e36);
    let _e38: Line = motor_line_into(_e37);
    return _e38;
}

fn plane_motor_geometric_quotient(self_1530: Plane, other_1314: Motor) -> PointAndPlane {
    var self_1531: Plane;
    var other_1315: Motor;

    self_1531 = self_1530;
    other_1315 = other_1314;
    let _e6: Motor = other_1315;
    let _e7: Motor = motor_inverse(_e6);
    let _e8: Plane = self_1531;
    let _e10: Motor = other_1315;
    let _e11: Motor = motor_inverse(_e10);
    let _e12: PointAndPlane = plane_motor_geometric_product(_e8, _e11);
    return _e12;
}

fn plane_motor_transformation(self_1532: Plane, other_1316: Motor) -> Motor {
    var self_1533: Plane;
    var other_1317: Motor;

    self_1533 = self_1532;
    other_1317 = other_1316;
    let _e6: Plane = self_1533;
    let _e7: Motor = other_1317;
    let _e8: PointAndPlane = plane_motor_geometric_product(_e6, _e7);
    let _e10: Plane = self_1533;
    let _e11: Plane = plane_reversal(_e10);
    let _e14: Plane = self_1533;
    let _e15: Motor = other_1317;
    let _e16: PointAndPlane = plane_motor_geometric_product(_e14, _e15);
    let _e18: Plane = self_1533;
    let _e19: Plane = plane_reversal(_e18);
    let _e20: Motor = point_and_plane_plane_geometric_product(_e16, _e19);
    return _e20;
}

fn plane_multi_vector_geometric_quotient(self_1534: Plane, other_1318: MultiVector) -> MultiVector {
    var self_1535: Plane;
    var other_1319: MultiVector;

    self_1535 = self_1534;
    other_1319 = other_1318;
    let _e6: MultiVector = other_1319;
    let _e7: MultiVector = multi_vector_inverse(_e6);
    let _e8: Plane = self_1535;
    let _e10: MultiVector = other_1319;
    let _e11: MultiVector = multi_vector_inverse(_e10);
    let _e12: MultiVector = plane_multi_vector_geometric_product(_e8, _e11);
    return _e12;
}

fn plane_multi_vector_transformation(self_1536: Plane, other_1320: MultiVector) -> MultiVector {
    var self_1537: Plane;
    var other_1321: MultiVector;

    self_1537 = self_1536;
    other_1321 = other_1320;
    let _e6: Plane = self_1537;
    let _e7: MultiVector = other_1321;
    let _e8: MultiVector = plane_multi_vector_geometric_product(_e6, _e7);
    let _e10: Plane = self_1537;
    let _e11: Plane = plane_reversal(_e10);
    let _e14: Plane = self_1537;
    let _e15: MultiVector = other_1321;
    let _e16: MultiVector = plane_multi_vector_geometric_product(_e14, _e15);
    let _e18: Plane = self_1537;
    let _e19: Plane = plane_reversal(_e18);
    let _e20: MultiVector = multi_vector_plane_geometric_product(_e16, _e19);
    return _e20;
}

fn plane_point_and_plane_geometric_quotient(self_1538: Plane, other_1322: PointAndPlane) -> Motor {
    var self_1539: Plane;
    var other_1323: PointAndPlane;

    self_1539 = self_1538;
    other_1323 = other_1322;
    let _e6: PointAndPlane = other_1323;
    let _e7: PointAndPlane = point_and_plane_inverse(_e6);
    let _e8: Plane = self_1539;
    let _e10: PointAndPlane = other_1323;
    let _e11: PointAndPlane = point_and_plane_inverse(_e10);
    let _e12: Motor = plane_point_and_plane_geometric_product(_e8, _e11);
    return _e12;
}

fn plane_point_and_plane_transformation(self_1540: Plane, other_1324: PointAndPlane) -> PointAndPlane {
    var self_1541: Plane;
    var other_1325: PointAndPlane;

    self_1541 = self_1540;
    other_1325 = other_1324;
    let _e6: Plane = self_1541;
    let _e7: PointAndPlane = other_1325;
    let _e8: Motor = plane_point_and_plane_geometric_product(_e6, _e7);
    let _e10: Plane = self_1541;
    let _e11: Plane = plane_reversal(_e10);
    let _e14: Plane = self_1541;
    let _e15: PointAndPlane = other_1325;
    let _e16: Motor = plane_point_and_plane_geometric_product(_e14, _e15);
    let _e18: Plane = self_1541;
    let _e19: Plane = plane_reversal(_e18);
    let _e20: PointAndPlane = motor_plane_geometric_product(_e16, _e19);
    return _e20;
}

fn plane_rotor_geometric_quotient(self_1542: Plane, other_1326: Rotor) -> PointAndPlane {
    var self_1543: Plane;
    var other_1327: Rotor;

    self_1543 = self_1542;
    other_1327 = other_1326;
    let _e6: Rotor = other_1327;
    let _e7: Rotor = rotor_inverse(_e6);
    let _e8: Plane = self_1543;
    let _e10: Rotor = other_1327;
    let _e11: Rotor = rotor_inverse(_e10);
    let _e12: PointAndPlane = plane_rotor_geometric_product(_e8, _e11);
    return _e12;
}

fn plane_rotor_transformation(self_1544: Plane, other_1328: Rotor) -> Rotor {
    var self_1545: Plane;
    var other_1329: Rotor;

    self_1545 = self_1544;
    other_1329 = other_1328;
    let _e6: Plane = self_1545;
    let _e7: Rotor = other_1329;
    let _e8: PointAndPlane = plane_rotor_geometric_product(_e6, _e7);
    let _e10: Plane = self_1545;
    let _e11: Plane = plane_reversal(_e10);
    let _e14: Plane = self_1545;
    let _e15: Rotor = other_1329;
    let _e16: PointAndPlane = plane_rotor_geometric_product(_e14, _e15);
    let _e18: Plane = self_1545;
    let _e19: Plane = plane_reversal(_e18);
    let _e20: Motor = point_and_plane_plane_geometric_product(_e16, _e19);
    let _e23: Plane = self_1545;
    let _e24: Rotor = other_1329;
    let _e25: PointAndPlane = plane_rotor_geometric_product(_e23, _e24);
    let _e27: Plane = self_1545;
    let _e28: Plane = plane_reversal(_e27);
    let _e31: Plane = self_1545;
    let _e32: Rotor = other_1329;
    let _e33: PointAndPlane = plane_rotor_geometric_product(_e31, _e32);
    let _e35: Plane = self_1545;
    let _e36: Plane = plane_reversal(_e35);
    let _e37: Motor = point_and_plane_plane_geometric_product(_e33, _e36);
    let _e38: Rotor = motor_rotor_into(_e37);
    return _e38;
}

fn plane_scalar_geometric_quotient(self_1546: Plane, other_1330: Scalar) -> Plane {
    var self_1547: Plane;
    var other_1331: Scalar;

    self_1547 = self_1546;
    other_1331 = other_1330;
    let _e6: Scalar = other_1331;
    let _e7: Scalar = scalar_inverse(_e6);
    let _e8: Plane = self_1547;
    let _e10: Scalar = other_1331;
    let _e11: Scalar = scalar_inverse(_e10);
    let _e12: Plane = plane_scalar_geometric_product(_e8, _e11);
    return _e12;
}

fn point_line_geometric_quotient(self_1548: Point, other_1332: Line) -> PointAndPlane {
    var self_1549: Point;
    var other_1333: Line;

    self_1549 = self_1548;
    other_1333 = other_1332;
    let _e6: Line = other_1333;
    let _e7: Line = line_inverse(_e6);
    let _e8: Point = self_1549;
    let _e10: Line = other_1333;
    let _e11: Line = line_inverse(_e10);
    let _e12: PointAndPlane = point_line_geometric_product(_e8, _e11);
    return _e12;
}

fn point_line_transformation(self_1550: Point, other_1334: Line) -> Line {
    var self_1551: Point;
    var other_1335: Line;

    self_1551 = self_1550;
    other_1335 = other_1334;
    let _e6: Point = self_1551;
    let _e7: Line = other_1335;
    let _e8: PointAndPlane = point_line_geometric_product(_e6, _e7);
    let _e10: Point = self_1551;
    let _e11: Point = point_reversal(_e10);
    let _e14: Point = self_1551;
    let _e15: Line = other_1335;
    let _e16: PointAndPlane = point_line_geometric_product(_e14, _e15);
    let _e18: Point = self_1551;
    let _e19: Point = point_reversal(_e18);
    let _e20: Motor = point_and_plane_point_geometric_product(_e16, _e19);
    let _e23: Point = self_1551;
    let _e24: Line = other_1335;
    let _e25: PointAndPlane = point_line_geometric_product(_e23, _e24);
    let _e27: Point = self_1551;
    let _e28: Point = point_reversal(_e27);
    let _e31: Point = self_1551;
    let _e32: Line = other_1335;
    let _e33: PointAndPlane = point_line_geometric_product(_e31, _e32);
    let _e35: Point = self_1551;
    let _e36: Point = point_reversal(_e35);
    let _e37: Motor = point_and_plane_point_geometric_product(_e33, _e36);
    let _e38: Line = motor_line_into(_e37);
    return _e38;
}

fn point_motor_geometric_quotient(self_1552: Point, other_1336: Motor) -> PointAndPlane {
    var self_1553: Point;
    var other_1337: Motor;

    self_1553 = self_1552;
    other_1337 = other_1336;
    let _e6: Motor = other_1337;
    let _e7: Motor = motor_inverse(_e6);
    let _e8: Point = self_1553;
    let _e10: Motor = other_1337;
    let _e11: Motor = motor_inverse(_e10);
    let _e12: PointAndPlane = point_motor_geometric_product(_e8, _e11);
    return _e12;
}

fn point_motor_transformation(self_1554: Point, other_1338: Motor) -> Motor {
    var self_1555: Point;
    var other_1339: Motor;

    self_1555 = self_1554;
    other_1339 = other_1338;
    let _e6: Point = self_1555;
    let _e7: Motor = other_1339;
    let _e8: PointAndPlane = point_motor_geometric_product(_e6, _e7);
    let _e10: Point = self_1555;
    let _e11: Point = point_reversal(_e10);
    let _e14: Point = self_1555;
    let _e15: Motor = other_1339;
    let _e16: PointAndPlane = point_motor_geometric_product(_e14, _e15);
    let _e18: Point = self_1555;
    let _e19: Point = point_reversal(_e18);
    let _e20: Motor = point_and_plane_point_geometric_product(_e16, _e19);
    return _e20;
}

fn point_multi_vector_geometric_quotient(self_1556: Point, other_1340: MultiVector) -> MultiVector {
    var self_1557: Point;
    var other_1341: MultiVector;

    self_1557 = self_1556;
    other_1341 = other_1340;
    let _e6: MultiVector = other_1341;
    let _e7: MultiVector = multi_vector_inverse(_e6);
    let _e8: Point = self_1557;
    let _e10: MultiVector = other_1341;
    let _e11: MultiVector = multi_vector_inverse(_e10);
    let _e12: MultiVector = point_multi_vector_geometric_product(_e8, _e11);
    return _e12;
}

fn point_multi_vector_transformation(self_1558: Point, other_1342: MultiVector) -> MultiVector {
    var self_1559: Point;
    var other_1343: MultiVector;

    self_1559 = self_1558;
    other_1343 = other_1342;
    let _e6: Point = self_1559;
    let _e7: MultiVector = other_1343;
    let _e8: MultiVector = point_multi_vector_geometric_product(_e6, _e7);
    let _e10: Point = self_1559;
    let _e11: Point = point_reversal(_e10);
    let _e14: Point = self_1559;
    let _e15: MultiVector = other_1343;
    let _e16: MultiVector = point_multi_vector_geometric_product(_e14, _e15);
    let _e18: Point = self_1559;
    let _e19: Point = point_reversal(_e18);
    let _e20: MultiVector = multi_vector_point_geometric_product(_e16, _e19);
    return _e20;
}

fn point_point_and_plane_geometric_quotient(self_1560: Point, other_1344: PointAndPlane) -> Motor {
    var self_1561: Point;
    var other_1345: PointAndPlane;

    self_1561 = self_1560;
    other_1345 = other_1344;
    let _e6: PointAndPlane = other_1345;
    let _e7: PointAndPlane = point_and_plane_inverse(_e6);
    let _e8: Point = self_1561;
    let _e10: PointAndPlane = other_1345;
    let _e11: PointAndPlane = point_and_plane_inverse(_e10);
    let _e12: Motor = point_point_and_plane_geometric_product(_e8, _e11);
    return _e12;
}

fn point_point_and_plane_transformation(self_1562: Point, other_1346: PointAndPlane) -> PointAndPlane {
    var self_1563: Point;
    var other_1347: PointAndPlane;

    self_1563 = self_1562;
    other_1347 = other_1346;
    let _e6: Point = self_1563;
    let _e7: PointAndPlane = other_1347;
    let _e8: Motor = point_point_and_plane_geometric_product(_e6, _e7);
    let _e10: Point = self_1563;
    let _e11: Point = point_reversal(_e10);
    let _e14: Point = self_1563;
    let _e15: PointAndPlane = other_1347;
    let _e16: Motor = point_point_and_plane_geometric_product(_e14, _e15);
    let _e18: Point = self_1563;
    let _e19: Point = point_reversal(_e18);
    let _e20: PointAndPlane = motor_point_geometric_product(_e16, _e19);
    return _e20;
}

fn point_rotor_geometric_quotient(self_1564: Point, other_1348: Rotor) -> PointAndPlane {
    var self_1565: Point;
    var other_1349: Rotor;

    self_1565 = self_1564;
    other_1349 = other_1348;
    let _e6: Rotor = other_1349;
    let _e7: Rotor = rotor_inverse(_e6);
    let _e8: Point = self_1565;
    let _e10: Rotor = other_1349;
    let _e11: Rotor = rotor_inverse(_e10);
    let _e12: PointAndPlane = point_rotor_geometric_product(_e8, _e11);
    return _e12;
}

fn point_rotor_transformation(self_1566: Point, other_1350: Rotor) -> Rotor {
    var self_1567: Point;
    var other_1351: Rotor;

    self_1567 = self_1566;
    other_1351 = other_1350;
    let _e6: Point = self_1567;
    let _e7: Rotor = other_1351;
    let _e8: PointAndPlane = point_rotor_geometric_product(_e6, _e7);
    let _e10: Point = self_1567;
    let _e11: Point = point_reversal(_e10);
    let _e14: Point = self_1567;
    let _e15: Rotor = other_1351;
    let _e16: PointAndPlane = point_rotor_geometric_product(_e14, _e15);
    let _e18: Point = self_1567;
    let _e19: Point = point_reversal(_e18);
    let _e20: Motor = point_and_plane_point_geometric_product(_e16, _e19);
    let _e23: Point = self_1567;
    let _e24: Rotor = other_1351;
    let _e25: PointAndPlane = point_rotor_geometric_product(_e23, _e24);
    let _e27: Point = self_1567;
    let _e28: Point = point_reversal(_e27);
    let _e31: Point = self_1567;
    let _e32: Rotor = other_1351;
    let _e33: PointAndPlane = point_rotor_geometric_product(_e31, _e32);
    let _e35: Point = self_1567;
    let _e36: Point = point_reversal(_e35);
    let _e37: Motor = point_and_plane_point_geometric_product(_e33, _e36);
    let _e38: Rotor = motor_rotor_into(_e37);
    return _e38;
}

fn point_scalar_geometric_quotient(self_1568: Point, other_1352: Scalar) -> Point {
    var self_1569: Point;
    var other_1353: Scalar;

    self_1569 = self_1568;
    other_1353 = other_1352;
    let _e6: Scalar = other_1353;
    let _e7: Scalar = scalar_inverse(_e6);
    let _e8: Point = self_1569;
    let _e10: Scalar = other_1353;
    let _e11: Scalar = scalar_inverse(_e10);
    let _e12: Point = point_scalar_geometric_product(_e8, _e11);
    return _e12;
}

fn point_and_plane_ideal_point_geometric_quotient(self_1570: PointAndPlane, other_1354: IdealPoint) -> PointAndPlane {
    var self_1571: PointAndPlane;
    var other_1355: IdealPoint;

    self_1571 = self_1570;
    other_1355 = other_1354;
    let _e6: IdealPoint = other_1355;
    let _e7: IdealPoint = ideal_point_inverse(_e6);
    let _e8: PointAndPlane = self_1571;
    let _e10: IdealPoint = other_1355;
    let _e11: IdealPoint = ideal_point_inverse(_e10);
    let _e12: PointAndPlane = point_and_plane_ideal_point_geometric_product(_e8, _e11);
    return _e12;
}

fn point_and_plane_ideal_point_transformation(self_1572: PointAndPlane, other_1356: IdealPoint) -> IdealPoint {
    var self_1573: PointAndPlane;
    var other_1357: IdealPoint;

    self_1573 = self_1572;
    other_1357 = other_1356;
    let _e6: PointAndPlane = self_1573;
    let _e7: IdealPoint = other_1357;
    let _e8: PointAndPlane = point_and_plane_ideal_point_geometric_product(_e6, _e7);
    let _e10: PointAndPlane = self_1573;
    let _e11: PointAndPlane = point_and_plane_reversal(_e10);
    let _e14: PointAndPlane = self_1573;
    let _e15: IdealPoint = other_1357;
    let _e16: PointAndPlane = point_and_plane_ideal_point_geometric_product(_e14, _e15);
    let _e18: PointAndPlane = self_1573;
    let _e19: PointAndPlane = point_and_plane_reversal(_e18);
    let _e20: Motor = point_and_plane_point_and_plane_geometric_product(_e16, _e19);
    let _e23: PointAndPlane = self_1573;
    let _e24: IdealPoint = other_1357;
    let _e25: PointAndPlane = point_and_plane_ideal_point_geometric_product(_e23, _e24);
    let _e27: PointAndPlane = self_1573;
    let _e28: PointAndPlane = point_and_plane_reversal(_e27);
    let _e31: PointAndPlane = self_1573;
    let _e32: IdealPoint = other_1357;
    let _e33: PointAndPlane = point_and_plane_ideal_point_geometric_product(_e31, _e32);
    let _e35: PointAndPlane = self_1573;
    let _e36: PointAndPlane = point_and_plane_reversal(_e35);
    let _e37: Motor = point_and_plane_point_and_plane_geometric_product(_e33, _e36);
    let _e38: IdealPoint = motor_ideal_point_into(_e37);
    return _e38;
}

fn point_and_plane_line_geometric_quotient(self_1574: PointAndPlane, other_1358: Line) -> PointAndPlane {
    var self_1575: PointAndPlane;
    var other_1359: Line;

    self_1575 = self_1574;
    other_1359 = other_1358;
    let _e6: Line = other_1359;
    let _e7: Line = line_inverse(_e6);
    let _e8: PointAndPlane = self_1575;
    let _e10: Line = other_1359;
    let _e11: Line = line_inverse(_e10);
    let _e12: PointAndPlane = point_and_plane_line_geometric_product(_e8, _e11);
    return _e12;
}

fn point_and_plane_line_transformation(self_1576: PointAndPlane, other_1360: Line) -> Line {
    var self_1577: PointAndPlane;
    var other_1361: Line;

    self_1577 = self_1576;
    other_1361 = other_1360;
    let _e6: PointAndPlane = self_1577;
    let _e7: Line = other_1361;
    let _e8: PointAndPlane = point_and_plane_line_geometric_product(_e6, _e7);
    let _e10: PointAndPlane = self_1577;
    let _e11: PointAndPlane = point_and_plane_reversal(_e10);
    let _e14: PointAndPlane = self_1577;
    let _e15: Line = other_1361;
    let _e16: PointAndPlane = point_and_plane_line_geometric_product(_e14, _e15);
    let _e18: PointAndPlane = self_1577;
    let _e19: PointAndPlane = point_and_plane_reversal(_e18);
    let _e20: Motor = point_and_plane_point_and_plane_geometric_product(_e16, _e19);
    let _e23: PointAndPlane = self_1577;
    let _e24: Line = other_1361;
    let _e25: PointAndPlane = point_and_plane_line_geometric_product(_e23, _e24);
    let _e27: PointAndPlane = self_1577;
    let _e28: PointAndPlane = point_and_plane_reversal(_e27);
    let _e31: PointAndPlane = self_1577;
    let _e32: Line = other_1361;
    let _e33: PointAndPlane = point_and_plane_line_geometric_product(_e31, _e32);
    let _e35: PointAndPlane = self_1577;
    let _e36: PointAndPlane = point_and_plane_reversal(_e35);
    let _e37: Motor = point_and_plane_point_and_plane_geometric_product(_e33, _e36);
    let _e38: Line = motor_line_into(_e37);
    return _e38;
}

fn point_and_plane_motor_geometric_quotient(self_1578: PointAndPlane, other_1362: Motor) -> PointAndPlane {
    var self_1579: PointAndPlane;
    var other_1363: Motor;

    self_1579 = self_1578;
    other_1363 = other_1362;
    let _e6: Motor = other_1363;
    let _e7: Motor = motor_inverse(_e6);
    let _e8: PointAndPlane = self_1579;
    let _e10: Motor = other_1363;
    let _e11: Motor = motor_inverse(_e10);
    let _e12: PointAndPlane = point_and_plane_motor_geometric_product(_e8, _e11);
    return _e12;
}

fn point_and_plane_motor_transformation(self_1580: PointAndPlane, other_1364: Motor) -> Motor {
    var self_1581: PointAndPlane;
    var other_1365: Motor;

    self_1581 = self_1580;
    other_1365 = other_1364;
    let _e6: PointAndPlane = self_1581;
    let _e7: Motor = other_1365;
    let _e8: PointAndPlane = point_and_plane_motor_geometric_product(_e6, _e7);
    let _e10: PointAndPlane = self_1581;
    let _e11: PointAndPlane = point_and_plane_reversal(_e10);
    let _e14: PointAndPlane = self_1581;
    let _e15: Motor = other_1365;
    let _e16: PointAndPlane = point_and_plane_motor_geometric_product(_e14, _e15);
    let _e18: PointAndPlane = self_1581;
    let _e19: PointAndPlane = point_and_plane_reversal(_e18);
    let _e20: Motor = point_and_plane_point_and_plane_geometric_product(_e16, _e19);
    return _e20;
}

fn point_and_plane_multi_vector_geometric_quotient(self_1582: PointAndPlane, other_1366: MultiVector) -> MultiVector {
    var self_1583: PointAndPlane;
    var other_1367: MultiVector;

    self_1583 = self_1582;
    other_1367 = other_1366;
    let _e6: MultiVector = other_1367;
    let _e7: MultiVector = multi_vector_inverse(_e6);
    let _e8: PointAndPlane = self_1583;
    let _e10: MultiVector = other_1367;
    let _e11: MultiVector = multi_vector_inverse(_e10);
    let _e12: MultiVector = point_and_plane_multi_vector_geometric_product(_e8, _e11);
    return _e12;
}

fn point_and_plane_multi_vector_transformation(self_1584: PointAndPlane, other_1368: MultiVector) -> MultiVector {
    var self_1585: PointAndPlane;
    var other_1369: MultiVector;

    self_1585 = self_1584;
    other_1369 = other_1368;
    let _e6: PointAndPlane = self_1585;
    let _e7: MultiVector = other_1369;
    let _e8: MultiVector = point_and_plane_multi_vector_geometric_product(_e6, _e7);
    let _e10: PointAndPlane = self_1585;
    let _e11: PointAndPlane = point_and_plane_reversal(_e10);
    let _e14: PointAndPlane = self_1585;
    let _e15: MultiVector = other_1369;
    let _e16: MultiVector = point_and_plane_multi_vector_geometric_product(_e14, _e15);
    let _e18: PointAndPlane = self_1585;
    let _e19: PointAndPlane = point_and_plane_reversal(_e18);
    let _e20: MultiVector = multi_vector_point_and_plane_geometric_product(_e16, _e19);
    return _e20;
}

fn point_and_plane_plane_geometric_quotient(self_1586: PointAndPlane, other_1370: Plane) -> Motor {
    var self_1587: PointAndPlane;
    var other_1371: Plane;

    self_1587 = self_1586;
    other_1371 = other_1370;
    let _e6: Plane = other_1371;
    let _e7: Plane = plane_inverse(_e6);
    let _e8: PointAndPlane = self_1587;
    let _e10: Plane = other_1371;
    let _e11: Plane = plane_inverse(_e10);
    let _e12: Motor = point_and_plane_plane_geometric_product(_e8, _e11);
    return _e12;
}

fn point_and_plane_plane_transformation(self_1588: PointAndPlane, other_1372: Plane) -> Plane {
    var self_1589: PointAndPlane;
    var other_1373: Plane;

    self_1589 = self_1588;
    other_1373 = other_1372;
    let _e6: PointAndPlane = self_1589;
    let _e7: Plane = other_1373;
    let _e8: Motor = point_and_plane_plane_geometric_product(_e6, _e7);
    let _e10: PointAndPlane = self_1589;
    let _e11: PointAndPlane = point_and_plane_reversal(_e10);
    let _e14: PointAndPlane = self_1589;
    let _e15: Plane = other_1373;
    let _e16: Motor = point_and_plane_plane_geometric_product(_e14, _e15);
    let _e18: PointAndPlane = self_1589;
    let _e19: PointAndPlane = point_and_plane_reversal(_e18);
    let _e20: PointAndPlane = motor_point_and_plane_geometric_product(_e16, _e19);
    let _e23: PointAndPlane = self_1589;
    let _e24: Plane = other_1373;
    let _e25: Motor = point_and_plane_plane_geometric_product(_e23, _e24);
    let _e27: PointAndPlane = self_1589;
    let _e28: PointAndPlane = point_and_plane_reversal(_e27);
    let _e31: PointAndPlane = self_1589;
    let _e32: Plane = other_1373;
    let _e33: Motor = point_and_plane_plane_geometric_product(_e31, _e32);
    let _e35: PointAndPlane = self_1589;
    let _e36: PointAndPlane = point_and_plane_reversal(_e35);
    let _e37: PointAndPlane = motor_point_and_plane_geometric_product(_e33, _e36);
    let _e38: Plane = point_and_plane_plane_into(_e37);
    return _e38;
}

fn point_and_plane_point_geometric_quotient(self_1590: PointAndPlane, other_1374: Point) -> Motor {
    var self_1591: PointAndPlane;
    var other_1375: Point;

    self_1591 = self_1590;
    other_1375 = other_1374;
    let _e6: Point = other_1375;
    let _e7: Point = point_inverse(_e6);
    let _e8: PointAndPlane = self_1591;
    let _e10: Point = other_1375;
    let _e11: Point = point_inverse(_e10);
    let _e12: Motor = point_and_plane_point_geometric_product(_e8, _e11);
    return _e12;
}

fn point_and_plane_point_transformation(self_1592: PointAndPlane, other_1376: Point) -> Point {
    var self_1593: PointAndPlane;
    var other_1377: Point;

    self_1593 = self_1592;
    other_1377 = other_1376;
    let _e6: PointAndPlane = self_1593;
    let _e7: Point = other_1377;
    let _e8: Motor = point_and_plane_point_geometric_product(_e6, _e7);
    let _e10: PointAndPlane = self_1593;
    let _e11: PointAndPlane = point_and_plane_reversal(_e10);
    let _e14: PointAndPlane = self_1593;
    let _e15: Point = other_1377;
    let _e16: Motor = point_and_plane_point_geometric_product(_e14, _e15);
    let _e18: PointAndPlane = self_1593;
    let _e19: PointAndPlane = point_and_plane_reversal(_e18);
    let _e20: PointAndPlane = motor_point_and_plane_geometric_product(_e16, _e19);
    let _e23: PointAndPlane = self_1593;
    let _e24: Point = other_1377;
    let _e25: Motor = point_and_plane_point_geometric_product(_e23, _e24);
    let _e27: PointAndPlane = self_1593;
    let _e28: PointAndPlane = point_and_plane_reversal(_e27);
    let _e31: PointAndPlane = self_1593;
    let _e32: Point = other_1377;
    let _e33: Motor = point_and_plane_point_geometric_product(_e31, _e32);
    let _e35: PointAndPlane = self_1593;
    let _e36: PointAndPlane = point_and_plane_reversal(_e35);
    let _e37: PointAndPlane = motor_point_and_plane_geometric_product(_e33, _e36);
    let _e38: Point = point_and_plane_point_into(_e37);
    return _e38;
}

fn point_and_plane_point_and_plane_geometric_quotient(self_1594: PointAndPlane, other_1378: PointAndPlane) -> Motor {
    var self_1595: PointAndPlane;
    var other_1379: PointAndPlane;

    self_1595 = self_1594;
    other_1379 = other_1378;
    let _e6: PointAndPlane = other_1379;
    let _e7: PointAndPlane = point_and_plane_inverse(_e6);
    let _e8: PointAndPlane = self_1595;
    let _e10: PointAndPlane = other_1379;
    let _e11: PointAndPlane = point_and_plane_inverse(_e10);
    let _e12: Motor = point_and_plane_point_and_plane_geometric_product(_e8, _e11);
    return _e12;
}

fn point_and_plane_point_and_plane_transformation(self_1596: PointAndPlane, other_1380: PointAndPlane) -> PointAndPlane {
    var self_1597: PointAndPlane;
    var other_1381: PointAndPlane;

    self_1597 = self_1596;
    other_1381 = other_1380;
    let _e6: PointAndPlane = self_1597;
    let _e7: PointAndPlane = other_1381;
    let _e8: Motor = point_and_plane_point_and_plane_geometric_product(_e6, _e7);
    let _e10: PointAndPlane = self_1597;
    let _e11: PointAndPlane = point_and_plane_reversal(_e10);
    let _e14: PointAndPlane = self_1597;
    let _e15: PointAndPlane = other_1381;
    let _e16: Motor = point_and_plane_point_and_plane_geometric_product(_e14, _e15);
    let _e18: PointAndPlane = self_1597;
    let _e19: PointAndPlane = point_and_plane_reversal(_e18);
    let _e20: PointAndPlane = motor_point_and_plane_geometric_product(_e16, _e19);
    return _e20;
}

fn point_and_plane_rotor_geometric_quotient(self_1598: PointAndPlane, other_1382: Rotor) -> PointAndPlane {
    var self_1599: PointAndPlane;
    var other_1383: Rotor;

    self_1599 = self_1598;
    other_1383 = other_1382;
    let _e6: Rotor = other_1383;
    let _e7: Rotor = rotor_inverse(_e6);
    let _e8: PointAndPlane = self_1599;
    let _e10: Rotor = other_1383;
    let _e11: Rotor = rotor_inverse(_e10);
    let _e12: PointAndPlane = point_and_plane_rotor_geometric_product(_e8, _e11);
    return _e12;
}

fn point_and_plane_rotor_transformation(self_1600: PointAndPlane, other_1384: Rotor) -> Rotor {
    var self_1601: PointAndPlane;
    var other_1385: Rotor;

    self_1601 = self_1600;
    other_1385 = other_1384;
    let _e6: PointAndPlane = self_1601;
    let _e7: Rotor = other_1385;
    let _e8: PointAndPlane = point_and_plane_rotor_geometric_product(_e6, _e7);
    let _e10: PointAndPlane = self_1601;
    let _e11: PointAndPlane = point_and_plane_reversal(_e10);
    let _e14: PointAndPlane = self_1601;
    let _e15: Rotor = other_1385;
    let _e16: PointAndPlane = point_and_plane_rotor_geometric_product(_e14, _e15);
    let _e18: PointAndPlane = self_1601;
    let _e19: PointAndPlane = point_and_plane_reversal(_e18);
    let _e20: Motor = point_and_plane_point_and_plane_geometric_product(_e16, _e19);
    let _e23: PointAndPlane = self_1601;
    let _e24: Rotor = other_1385;
    let _e25: PointAndPlane = point_and_plane_rotor_geometric_product(_e23, _e24);
    let _e27: PointAndPlane = self_1601;
    let _e28: PointAndPlane = point_and_plane_reversal(_e27);
    let _e31: PointAndPlane = self_1601;
    let _e32: Rotor = other_1385;
    let _e33: PointAndPlane = point_and_plane_rotor_geometric_product(_e31, _e32);
    let _e35: PointAndPlane = self_1601;
    let _e36: PointAndPlane = point_and_plane_reversal(_e35);
    let _e37: Motor = point_and_plane_point_and_plane_geometric_product(_e33, _e36);
    let _e38: Rotor = motor_rotor_into(_e37);
    return _e38;
}

fn point_and_plane_scalar_geometric_quotient(self_1602: PointAndPlane, other_1386: Scalar) -> PointAndPlane {
    var self_1603: PointAndPlane;
    var other_1387: Scalar;

    self_1603 = self_1602;
    other_1387 = other_1386;
    let _e6: Scalar = other_1387;
    let _e7: Scalar = scalar_inverse(_e6);
    let _e8: PointAndPlane = self_1603;
    let _e10: Scalar = other_1387;
    let _e11: Scalar = scalar_inverse(_e10);
    let _e12: PointAndPlane = point_and_plane_scalar_geometric_product(_e8, _e11);
    return _e12;
}

fn point_and_plane_scalar_transformation(self_1604: PointAndPlane, other_1388: Scalar) -> Scalar {
    var self_1605: PointAndPlane;
    var other_1389: Scalar;

    self_1605 = self_1604;
    other_1389 = other_1388;
    let _e6: PointAndPlane = self_1605;
    let _e7: Scalar = other_1389;
    let _e8: PointAndPlane = point_and_plane_scalar_geometric_product(_e6, _e7);
    let _e10: PointAndPlane = self_1605;
    let _e11: PointAndPlane = point_and_plane_reversal(_e10);
    let _e14: PointAndPlane = self_1605;
    let _e15: Scalar = other_1389;
    let _e16: PointAndPlane = point_and_plane_scalar_geometric_product(_e14, _e15);
    let _e18: PointAndPlane = self_1605;
    let _e19: PointAndPlane = point_and_plane_reversal(_e18);
    let _e20: Motor = point_and_plane_point_and_plane_geometric_product(_e16, _e19);
    let _e23: PointAndPlane = self_1605;
    let _e24: Scalar = other_1389;
    let _e25: PointAndPlane = point_and_plane_scalar_geometric_product(_e23, _e24);
    let _e27: PointAndPlane = self_1605;
    let _e28: PointAndPlane = point_and_plane_reversal(_e27);
    let _e31: PointAndPlane = self_1605;
    let _e32: Scalar = other_1389;
    let _e33: PointAndPlane = point_and_plane_scalar_geometric_product(_e31, _e32);
    let _e35: PointAndPlane = self_1605;
    let _e36: PointAndPlane = point_and_plane_reversal(_e35);
    let _e37: Motor = point_and_plane_point_and_plane_geometric_product(_e33, _e36);
    let _e38: Scalar = motor_scalar_into(_e37);
    return _e38;
}

fn point_and_plane_translator_geometric_quotient(self_1606: PointAndPlane, other_1390: Translator) -> PointAndPlane {
    var self_1607: PointAndPlane;
    var other_1391: Translator;

    self_1607 = self_1606;
    other_1391 = other_1390;
    let _e6: Translator = other_1391;
    let _e7: Translator = translator_inverse(_e6);
    let _e8: PointAndPlane = self_1607;
    let _e10: Translator = other_1391;
    let _e11: Translator = translator_inverse(_e10);
    let _e12: PointAndPlane = point_and_plane_translator_geometric_product(_e8, _e11);
    return _e12;
}

fn point_and_plane_translator_transformation(self_1608: PointAndPlane, other_1392: Translator) -> Translator {
    var self_1609: PointAndPlane;
    var other_1393: Translator;

    self_1609 = self_1608;
    other_1393 = other_1392;
    let _e6: PointAndPlane = self_1609;
    let _e7: Translator = other_1393;
    let _e8: PointAndPlane = point_and_plane_translator_geometric_product(_e6, _e7);
    let _e10: PointAndPlane = self_1609;
    let _e11: PointAndPlane = point_and_plane_reversal(_e10);
    let _e14: PointAndPlane = self_1609;
    let _e15: Translator = other_1393;
    let _e16: PointAndPlane = point_and_plane_translator_geometric_product(_e14, _e15);
    let _e18: PointAndPlane = self_1609;
    let _e19: PointAndPlane = point_and_plane_reversal(_e18);
    let _e20: Motor = point_and_plane_point_and_plane_geometric_product(_e16, _e19);
    let _e23: PointAndPlane = self_1609;
    let _e24: Translator = other_1393;
    let _e25: PointAndPlane = point_and_plane_translator_geometric_product(_e23, _e24);
    let _e27: PointAndPlane = self_1609;
    let _e28: PointAndPlane = point_and_plane_reversal(_e27);
    let _e31: PointAndPlane = self_1609;
    let _e32: Translator = other_1393;
    let _e33: PointAndPlane = point_and_plane_translator_geometric_product(_e31, _e32);
    let _e35: PointAndPlane = self_1609;
    let _e36: PointAndPlane = point_and_plane_reversal(_e35);
    let _e37: Motor = point_and_plane_point_and_plane_geometric_product(_e33, _e36);
    let _e38: Translator = motor_translator_into(_e37);
    return _e38;
}

fn rotor_line_geometric_quotient(self_1610: Rotor, other_1394: Line) -> Motor {
    var self_1611: Rotor;
    var other_1395: Line;

    self_1611 = self_1610;
    other_1395 = other_1394;
    let _e6: Line = other_1395;
    let _e7: Line = line_inverse(_e6);
    let _e8: Rotor = self_1611;
    let _e10: Line = other_1395;
    let _e11: Line = line_inverse(_e10);
    let _e12: Motor = rotor_line_geometric_product(_e8, _e11);
    return _e12;
}

fn rotor_line_transformation(self_1612: Rotor, other_1396: Line) -> Line {
    var self_1613: Rotor;
    var other_1397: Line;

    self_1613 = self_1612;
    other_1397 = other_1396;
    let _e6: Rotor = self_1613;
    let _e7: Line = other_1397;
    let _e8: Motor = rotor_line_geometric_product(_e6, _e7);
    let _e10: Rotor = self_1613;
    let _e11: Rotor = rotor_reversal(_e10);
    let _e14: Rotor = self_1613;
    let _e15: Line = other_1397;
    let _e16: Motor = rotor_line_geometric_product(_e14, _e15);
    let _e18: Rotor = self_1613;
    let _e19: Rotor = rotor_reversal(_e18);
    let _e20: Motor = motor_rotor_geometric_product(_e16, _e19);
    let _e23: Rotor = self_1613;
    let _e24: Line = other_1397;
    let _e25: Motor = rotor_line_geometric_product(_e23, _e24);
    let _e27: Rotor = self_1613;
    let _e28: Rotor = rotor_reversal(_e27);
    let _e31: Rotor = self_1613;
    let _e32: Line = other_1397;
    let _e33: Motor = rotor_line_geometric_product(_e31, _e32);
    let _e35: Rotor = self_1613;
    let _e36: Rotor = rotor_reversal(_e35);
    let _e37: Motor = motor_rotor_geometric_product(_e33, _e36);
    let _e38: Line = motor_line_into(_e37);
    return _e38;
}

fn rotor_motor_geometric_quotient(self_1614: Rotor, other_1398: Motor) -> Motor {
    var self_1615: Rotor;
    var other_1399: Motor;

    self_1615 = self_1614;
    other_1399 = other_1398;
    let _e6: Motor = other_1399;
    let _e7: Motor = motor_inverse(_e6);
    let _e8: Rotor = self_1615;
    let _e10: Motor = other_1399;
    let _e11: Motor = motor_inverse(_e10);
    let _e12: Motor = rotor_motor_geometric_product(_e8, _e11);
    return _e12;
}

fn rotor_motor_transformation(self_1616: Rotor, other_1400: Motor) -> Motor {
    var self_1617: Rotor;
    var other_1401: Motor;

    self_1617 = self_1616;
    other_1401 = other_1400;
    let _e6: Rotor = self_1617;
    let _e7: Motor = other_1401;
    let _e8: Motor = rotor_motor_geometric_product(_e6, _e7);
    let _e10: Rotor = self_1617;
    let _e11: Rotor = rotor_reversal(_e10);
    let _e14: Rotor = self_1617;
    let _e15: Motor = other_1401;
    let _e16: Motor = rotor_motor_geometric_product(_e14, _e15);
    let _e18: Rotor = self_1617;
    let _e19: Rotor = rotor_reversal(_e18);
    let _e20: Motor = motor_rotor_geometric_product(_e16, _e19);
    return _e20;
}

fn rotor_multi_vector_geometric_quotient(self_1618: Rotor, other_1402: MultiVector) -> MultiVector {
    var self_1619: Rotor;
    var other_1403: MultiVector;

    self_1619 = self_1618;
    other_1403 = other_1402;
    let _e6: MultiVector = other_1403;
    let _e7: MultiVector = multi_vector_inverse(_e6);
    let _e8: Rotor = self_1619;
    let _e10: MultiVector = other_1403;
    let _e11: MultiVector = multi_vector_inverse(_e10);
    let _e12: MultiVector = rotor_multi_vector_geometric_product(_e8, _e11);
    return _e12;
}

fn rotor_multi_vector_transformation(self_1620: Rotor, other_1404: MultiVector) -> MultiVector {
    var self_1621: Rotor;
    var other_1405: MultiVector;

    self_1621 = self_1620;
    other_1405 = other_1404;
    let _e6: Rotor = self_1621;
    let _e7: MultiVector = other_1405;
    let _e8: MultiVector = rotor_multi_vector_geometric_product(_e6, _e7);
    let _e10: Rotor = self_1621;
    let _e11: Rotor = rotor_reversal(_e10);
    let _e14: Rotor = self_1621;
    let _e15: MultiVector = other_1405;
    let _e16: MultiVector = rotor_multi_vector_geometric_product(_e14, _e15);
    let _e18: Rotor = self_1621;
    let _e19: Rotor = rotor_reversal(_e18);
    let _e20: MultiVector = multi_vector_rotor_geometric_product(_e16, _e19);
    return _e20;
}

fn rotor_plane_geometric_quotient(self_1622: Rotor, other_1406: Plane) -> PointAndPlane {
    var self_1623: Rotor;
    var other_1407: Plane;

    self_1623 = self_1622;
    other_1407 = other_1406;
    let _e6: Plane = other_1407;
    let _e7: Plane = plane_inverse(_e6);
    let _e8: Rotor = self_1623;
    let _e10: Plane = other_1407;
    let _e11: Plane = plane_inverse(_e10);
    let _e12: PointAndPlane = rotor_plane_geometric_product(_e8, _e11);
    return _e12;
}

fn rotor_plane_transformation(self_1624: Rotor, other_1408: Plane) -> Plane {
    var self_1625: Rotor;
    var other_1409: Plane;

    self_1625 = self_1624;
    other_1409 = other_1408;
    let _e6: Rotor = self_1625;
    let _e7: Plane = other_1409;
    let _e8: PointAndPlane = rotor_plane_geometric_product(_e6, _e7);
    let _e10: Rotor = self_1625;
    let _e11: Rotor = rotor_reversal(_e10);
    let _e14: Rotor = self_1625;
    let _e15: Plane = other_1409;
    let _e16: PointAndPlane = rotor_plane_geometric_product(_e14, _e15);
    let _e18: Rotor = self_1625;
    let _e19: Rotor = rotor_reversal(_e18);
    let _e20: PointAndPlane = point_and_plane_rotor_geometric_product(_e16, _e19);
    let _e23: Rotor = self_1625;
    let _e24: Plane = other_1409;
    let _e25: PointAndPlane = rotor_plane_geometric_product(_e23, _e24);
    let _e27: Rotor = self_1625;
    let _e28: Rotor = rotor_reversal(_e27);
    let _e31: Rotor = self_1625;
    let _e32: Plane = other_1409;
    let _e33: PointAndPlane = rotor_plane_geometric_product(_e31, _e32);
    let _e35: Rotor = self_1625;
    let _e36: Rotor = rotor_reversal(_e35);
    let _e37: PointAndPlane = point_and_plane_rotor_geometric_product(_e33, _e36);
    let _e38: Plane = point_and_plane_plane_into(_e37);
    return _e38;
}

fn rotor_point_geometric_quotient(self_1626: Rotor, other_1410: Point) -> PointAndPlane {
    var self_1627: Rotor;
    var other_1411: Point;

    self_1627 = self_1626;
    other_1411 = other_1410;
    let _e6: Point = other_1411;
    let _e7: Point = point_inverse(_e6);
    let _e8: Rotor = self_1627;
    let _e10: Point = other_1411;
    let _e11: Point = point_inverse(_e10);
    let _e12: PointAndPlane = rotor_point_geometric_product(_e8, _e11);
    return _e12;
}

fn rotor_point_transformation(self_1628: Rotor, other_1412: Point) -> Point {
    var self_1629: Rotor;
    var other_1413: Point;

    self_1629 = self_1628;
    other_1413 = other_1412;
    let _e6: Rotor = self_1629;
    let _e7: Point = other_1413;
    let _e8: PointAndPlane = rotor_point_geometric_product(_e6, _e7);
    let _e10: Rotor = self_1629;
    let _e11: Rotor = rotor_reversal(_e10);
    let _e14: Rotor = self_1629;
    let _e15: Point = other_1413;
    let _e16: PointAndPlane = rotor_point_geometric_product(_e14, _e15);
    let _e18: Rotor = self_1629;
    let _e19: Rotor = rotor_reversal(_e18);
    let _e20: PointAndPlane = point_and_plane_rotor_geometric_product(_e16, _e19);
    let _e23: Rotor = self_1629;
    let _e24: Point = other_1413;
    let _e25: PointAndPlane = rotor_point_geometric_product(_e23, _e24);
    let _e27: Rotor = self_1629;
    let _e28: Rotor = rotor_reversal(_e27);
    let _e31: Rotor = self_1629;
    let _e32: Point = other_1413;
    let _e33: PointAndPlane = rotor_point_geometric_product(_e31, _e32);
    let _e35: Rotor = self_1629;
    let _e36: Rotor = rotor_reversal(_e35);
    let _e37: PointAndPlane = point_and_plane_rotor_geometric_product(_e33, _e36);
    let _e38: Point = point_and_plane_point_into(_e37);
    return _e38;
}

fn rotor_point_and_plane_geometric_quotient(self_1630: Rotor, other_1414: PointAndPlane) -> PointAndPlane {
    var self_1631: Rotor;
    var other_1415: PointAndPlane;

    self_1631 = self_1630;
    other_1415 = other_1414;
    let _e6: PointAndPlane = other_1415;
    let _e7: PointAndPlane = point_and_plane_inverse(_e6);
    let _e8: Rotor = self_1631;
    let _e10: PointAndPlane = other_1415;
    let _e11: PointAndPlane = point_and_plane_inverse(_e10);
    let _e12: PointAndPlane = rotor_point_and_plane_geometric_product(_e8, _e11);
    return _e12;
}

fn rotor_point_and_plane_transformation(self_1632: Rotor, other_1416: PointAndPlane) -> PointAndPlane {
    var self_1633: Rotor;
    var other_1417: PointAndPlane;

    self_1633 = self_1632;
    other_1417 = other_1416;
    let _e6: Rotor = self_1633;
    let _e7: PointAndPlane = other_1417;
    let _e8: PointAndPlane = rotor_point_and_plane_geometric_product(_e6, _e7);
    let _e10: Rotor = self_1633;
    let _e11: Rotor = rotor_reversal(_e10);
    let _e14: Rotor = self_1633;
    let _e15: PointAndPlane = other_1417;
    let _e16: PointAndPlane = rotor_point_and_plane_geometric_product(_e14, _e15);
    let _e18: Rotor = self_1633;
    let _e19: Rotor = rotor_reversal(_e18);
    let _e20: PointAndPlane = point_and_plane_rotor_geometric_product(_e16, _e19);
    return _e20;
}

fn rotor_powi(self_1634: Rotor, exponent_4: i32) -> Rotor {
    var self_1635: Rotor;
    var exponent_5: i32;
    var local_2: Rotor;
    var x_2: Rotor;
    var y_2: Rotor;
    var n_2: i32;

    self_1635 = self_1634;
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
        let _e12: Rotor = self_1635;
        let _e13: Rotor = rotor_inverse(_e12);
        local_2 = _e13;
    } else {
        let _e14: Rotor = self_1635;
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

fn rotor_rotor_geometric_quotient(self_1636: Rotor, other_1418: Rotor) -> Rotor {
    var self_1637: Rotor;
    var other_1419: Rotor;

    self_1637 = self_1636;
    other_1419 = other_1418;
    let _e6: Rotor = other_1419;
    let _e7: Rotor = rotor_inverse(_e6);
    let _e8: Rotor = self_1637;
    let _e10: Rotor = other_1419;
    let _e11: Rotor = rotor_inverse(_e10);
    let _e12: Rotor = rotor_rotor_geometric_product(_e8, _e11);
    return _e12;
}

fn rotor_rotor_transformation(self_1638: Rotor, other_1420: Rotor) -> Rotor {
    var self_1639: Rotor;
    var other_1421: Rotor;

    self_1639 = self_1638;
    other_1421 = other_1420;
    let _e6: Rotor = self_1639;
    let _e7: Rotor = other_1421;
    let _e8: Rotor = rotor_rotor_geometric_product(_e6, _e7);
    let _e10: Rotor = self_1639;
    let _e11: Rotor = rotor_reversal(_e10);
    let _e14: Rotor = self_1639;
    let _e15: Rotor = other_1421;
    let _e16: Rotor = rotor_rotor_geometric_product(_e14, _e15);
    let _e18: Rotor = self_1639;
    let _e19: Rotor = rotor_reversal(_e18);
    let _e20: Rotor = rotor_rotor_geometric_product(_e16, _e19);
    return _e20;
}

fn rotor_scalar_geometric_quotient(self_1640: Rotor, other_1422: Scalar) -> Rotor {
    var self_1641: Rotor;
    var other_1423: Scalar;

    self_1641 = self_1640;
    other_1423 = other_1422;
    let _e6: Scalar = other_1423;
    let _e7: Scalar = scalar_inverse(_e6);
    let _e8: Rotor = self_1641;
    let _e10: Scalar = other_1423;
    let _e11: Scalar = scalar_inverse(_e10);
    let _e12: Rotor = rotor_scalar_geometric_product(_e8, _e11);
    return _e12;
}

fn rotor_scalar_transformation(self_1642: Rotor, other_1424: Scalar) -> Scalar {
    var self_1643: Rotor;
    var other_1425: Scalar;

    self_1643 = self_1642;
    other_1425 = other_1424;
    let _e6: Rotor = self_1643;
    let _e7: Scalar = other_1425;
    let _e8: Rotor = rotor_scalar_geometric_product(_e6, _e7);
    let _e10: Rotor = self_1643;
    let _e11: Rotor = rotor_reversal(_e10);
    let _e14: Rotor = self_1643;
    let _e15: Scalar = other_1425;
    let _e16: Rotor = rotor_scalar_geometric_product(_e14, _e15);
    let _e18: Rotor = self_1643;
    let _e19: Rotor = rotor_reversal(_e18);
    let _e20: Rotor = rotor_rotor_geometric_product(_e16, _e19);
    let _e23: Rotor = self_1643;
    let _e24: Scalar = other_1425;
    let _e25: Rotor = rotor_scalar_geometric_product(_e23, _e24);
    let _e27: Rotor = self_1643;
    let _e28: Rotor = rotor_reversal(_e27);
    let _e31: Rotor = self_1643;
    let _e32: Scalar = other_1425;
    let _e33: Rotor = rotor_scalar_geometric_product(_e31, _e32);
    let _e35: Rotor = self_1643;
    let _e36: Rotor = rotor_reversal(_e35);
    let _e37: Rotor = rotor_rotor_geometric_product(_e33, _e36);
    let _e38: Scalar = rotor_scalar_into(_e37);
    return _e38;
}

fn rotor_translator_geometric_quotient(self_1644: Rotor, other_1426: Translator) -> Motor {
    var self_1645: Rotor;
    var other_1427: Translator;

    self_1645 = self_1644;
    other_1427 = other_1426;
    let _e6: Translator = other_1427;
    let _e7: Translator = translator_inverse(_e6);
    let _e8: Rotor = self_1645;
    let _e10: Translator = other_1427;
    let _e11: Translator = translator_inverse(_e10);
    let _e12: Motor = rotor_translator_geometric_product(_e8, _e11);
    return _e12;
}

fn rotor_translator_transformation(self_1646: Rotor, other_1428: Translator) -> Translator {
    var self_1647: Rotor;
    var other_1429: Translator;

    self_1647 = self_1646;
    other_1429 = other_1428;
    let _e6: Rotor = self_1647;
    let _e7: Translator = other_1429;
    let _e8: Motor = rotor_translator_geometric_product(_e6, _e7);
    let _e10: Rotor = self_1647;
    let _e11: Rotor = rotor_reversal(_e10);
    let _e14: Rotor = self_1647;
    let _e15: Translator = other_1429;
    let _e16: Motor = rotor_translator_geometric_product(_e14, _e15);
    let _e18: Rotor = self_1647;
    let _e19: Rotor = rotor_reversal(_e18);
    let _e20: Motor = motor_rotor_geometric_product(_e16, _e19);
    let _e23: Rotor = self_1647;
    let _e24: Translator = other_1429;
    let _e25: Motor = rotor_translator_geometric_product(_e23, _e24);
    let _e27: Rotor = self_1647;
    let _e28: Rotor = rotor_reversal(_e27);
    let _e31: Rotor = self_1647;
    let _e32: Translator = other_1429;
    let _e33: Motor = rotor_translator_geometric_product(_e31, _e32);
    let _e35: Rotor = self_1647;
    let _e36: Rotor = rotor_reversal(_e35);
    let _e37: Motor = motor_rotor_geometric_product(_e33, _e36);
    let _e38: Translator = motor_translator_into(_e37);
    return _e38;
}

fn scalar_ideal_point_geometric_quotient(self_1648: Scalar, other_1430: IdealPoint) -> IdealPoint {
    var self_1649: Scalar;
    var other_1431: IdealPoint;

    self_1649 = self_1648;
    other_1431 = other_1430;
    let _e6: IdealPoint = other_1431;
    let _e7: IdealPoint = ideal_point_inverse(_e6);
    let _e8: Scalar = self_1649;
    let _e10: IdealPoint = other_1431;
    let _e11: IdealPoint = ideal_point_inverse(_e10);
    let _e12: IdealPoint = scalar_ideal_point_geometric_product(_e8, _e11);
    return _e12;
}

fn scalar_ideal_point_transformation(self_1650: Scalar, other_1432: IdealPoint) -> IdealPoint {
    var self_1651: Scalar;
    var other_1433: IdealPoint;

    self_1651 = self_1650;
    other_1433 = other_1432;
    let _e6: Scalar = self_1651;
    let _e7: IdealPoint = other_1433;
    let _e8: IdealPoint = scalar_ideal_point_geometric_product(_e6, _e7);
    let _e10: Scalar = self_1651;
    let _e11: Scalar = scalar_reversal(_e10);
    let _e14: Scalar = self_1651;
    let _e15: IdealPoint = other_1433;
    let _e16: IdealPoint = scalar_ideal_point_geometric_product(_e14, _e15);
    let _e18: Scalar = self_1651;
    let _e19: Scalar = scalar_reversal(_e18);
    let _e20: IdealPoint = ideal_point_scalar_geometric_product(_e16, _e19);
    return _e20;
}

fn scalar_line_geometric_quotient(self_1652: Scalar, other_1434: Line) -> Line {
    var self_1653: Scalar;
    var other_1435: Line;

    self_1653 = self_1652;
    other_1435 = other_1434;
    let _e6: Line = other_1435;
    let _e7: Line = line_inverse(_e6);
    let _e8: Scalar = self_1653;
    let _e10: Line = other_1435;
    let _e11: Line = line_inverse(_e10);
    let _e12: Line = scalar_line_geometric_product(_e8, _e11);
    return _e12;
}

fn scalar_line_transformation(self_1654: Scalar, other_1436: Line) -> Line {
    var self_1655: Scalar;
    var other_1437: Line;

    self_1655 = self_1654;
    other_1437 = other_1436;
    let _e6: Scalar = self_1655;
    let _e7: Line = other_1437;
    let _e8: Line = scalar_line_geometric_product(_e6, _e7);
    let _e10: Scalar = self_1655;
    let _e11: Scalar = scalar_reversal(_e10);
    let _e14: Scalar = self_1655;
    let _e15: Line = other_1437;
    let _e16: Line = scalar_line_geometric_product(_e14, _e15);
    let _e18: Scalar = self_1655;
    let _e19: Scalar = scalar_reversal(_e18);
    let _e20: Line = line_scalar_geometric_product(_e16, _e19);
    return _e20;
}

fn scalar_motor_geometric_quotient(self_1656: Scalar, other_1438: Motor) -> Motor {
    var self_1657: Scalar;
    var other_1439: Motor;

    self_1657 = self_1656;
    other_1439 = other_1438;
    let _e6: Motor = other_1439;
    let _e7: Motor = motor_inverse(_e6);
    let _e8: Scalar = self_1657;
    let _e10: Motor = other_1439;
    let _e11: Motor = motor_inverse(_e10);
    let _e12: Motor = scalar_motor_geometric_product(_e8, _e11);
    return _e12;
}

fn scalar_motor_transformation(self_1658: Scalar, other_1440: Motor) -> Motor {
    var self_1659: Scalar;
    var other_1441: Motor;

    self_1659 = self_1658;
    other_1441 = other_1440;
    let _e6: Scalar = self_1659;
    let _e7: Motor = other_1441;
    let _e8: Motor = scalar_motor_geometric_product(_e6, _e7);
    let _e10: Scalar = self_1659;
    let _e11: Scalar = scalar_reversal(_e10);
    let _e14: Scalar = self_1659;
    let _e15: Motor = other_1441;
    let _e16: Motor = scalar_motor_geometric_product(_e14, _e15);
    let _e18: Scalar = self_1659;
    let _e19: Scalar = scalar_reversal(_e18);
    let _e20: Motor = motor_scalar_geometric_product(_e16, _e19);
    return _e20;
}

fn scalar_multi_vector_geometric_quotient(self_1660: Scalar, other_1442: MultiVector) -> MultiVector {
    var self_1661: Scalar;
    var other_1443: MultiVector;

    self_1661 = self_1660;
    other_1443 = other_1442;
    let _e6: MultiVector = other_1443;
    let _e7: MultiVector = multi_vector_inverse(_e6);
    let _e8: Scalar = self_1661;
    let _e10: MultiVector = other_1443;
    let _e11: MultiVector = multi_vector_inverse(_e10);
    let _e12: MultiVector = scalar_multi_vector_geometric_product(_e8, _e11);
    return _e12;
}

fn scalar_multi_vector_transformation(self_1662: Scalar, other_1444: MultiVector) -> MultiVector {
    var self_1663: Scalar;
    var other_1445: MultiVector;

    self_1663 = self_1662;
    other_1445 = other_1444;
    let _e6: Scalar = self_1663;
    let _e7: MultiVector = other_1445;
    let _e8: MultiVector = scalar_multi_vector_geometric_product(_e6, _e7);
    let _e10: Scalar = self_1663;
    let _e11: Scalar = scalar_reversal(_e10);
    let _e14: Scalar = self_1663;
    let _e15: MultiVector = other_1445;
    let _e16: MultiVector = scalar_multi_vector_geometric_product(_e14, _e15);
    let _e18: Scalar = self_1663;
    let _e19: Scalar = scalar_reversal(_e18);
    let _e20: MultiVector = multi_vector_scalar_geometric_product(_e16, _e19);
    return _e20;
}

fn scalar_plane_geometric_quotient(self_1664: Scalar, other_1446: Plane) -> Plane {
    var self_1665: Scalar;
    var other_1447: Plane;

    self_1665 = self_1664;
    other_1447 = other_1446;
    let _e6: Plane = other_1447;
    let _e7: Plane = plane_inverse(_e6);
    let _e8: Scalar = self_1665;
    let _e10: Plane = other_1447;
    let _e11: Plane = plane_inverse(_e10);
    let _e12: Plane = scalar_plane_geometric_product(_e8, _e11);
    return _e12;
}

fn scalar_plane_transformation(self_1666: Scalar, other_1448: Plane) -> Plane {
    var self_1667: Scalar;
    var other_1449: Plane;

    self_1667 = self_1666;
    other_1449 = other_1448;
    let _e6: Scalar = self_1667;
    let _e7: Plane = other_1449;
    let _e8: Plane = scalar_plane_geometric_product(_e6, _e7);
    let _e10: Scalar = self_1667;
    let _e11: Scalar = scalar_reversal(_e10);
    let _e14: Scalar = self_1667;
    let _e15: Plane = other_1449;
    let _e16: Plane = scalar_plane_geometric_product(_e14, _e15);
    let _e18: Scalar = self_1667;
    let _e19: Scalar = scalar_reversal(_e18);
    let _e20: Plane = plane_scalar_geometric_product(_e16, _e19);
    return _e20;
}

fn scalar_point_geometric_quotient(self_1668: Scalar, other_1450: Point) -> Point {
    var self_1669: Scalar;
    var other_1451: Point;

    self_1669 = self_1668;
    other_1451 = other_1450;
    let _e6: Point = other_1451;
    let _e7: Point = point_inverse(_e6);
    let _e8: Scalar = self_1669;
    let _e10: Point = other_1451;
    let _e11: Point = point_inverse(_e10);
    let _e12: Point = scalar_point_geometric_product(_e8, _e11);
    return _e12;
}

fn scalar_point_transformation(self_1670: Scalar, other_1452: Point) -> Point {
    var self_1671: Scalar;
    var other_1453: Point;

    self_1671 = self_1670;
    other_1453 = other_1452;
    let _e6: Scalar = self_1671;
    let _e7: Point = other_1453;
    let _e8: Point = scalar_point_geometric_product(_e6, _e7);
    let _e10: Scalar = self_1671;
    let _e11: Scalar = scalar_reversal(_e10);
    let _e14: Scalar = self_1671;
    let _e15: Point = other_1453;
    let _e16: Point = scalar_point_geometric_product(_e14, _e15);
    let _e18: Scalar = self_1671;
    let _e19: Scalar = scalar_reversal(_e18);
    let _e20: Point = point_scalar_geometric_product(_e16, _e19);
    return _e20;
}

fn scalar_point_and_plane_geometric_quotient(self_1672: Scalar, other_1454: PointAndPlane) -> PointAndPlane {
    var self_1673: Scalar;
    var other_1455: PointAndPlane;

    self_1673 = self_1672;
    other_1455 = other_1454;
    let _e6: PointAndPlane = other_1455;
    let _e7: PointAndPlane = point_and_plane_inverse(_e6);
    let _e8: Scalar = self_1673;
    let _e10: PointAndPlane = other_1455;
    let _e11: PointAndPlane = point_and_plane_inverse(_e10);
    let _e12: PointAndPlane = scalar_point_and_plane_geometric_product(_e8, _e11);
    return _e12;
}

fn scalar_point_and_plane_transformation(self_1674: Scalar, other_1456: PointAndPlane) -> PointAndPlane {
    var self_1675: Scalar;
    var other_1457: PointAndPlane;

    self_1675 = self_1674;
    other_1457 = other_1456;
    let _e6: Scalar = self_1675;
    let _e7: PointAndPlane = other_1457;
    let _e8: PointAndPlane = scalar_point_and_plane_geometric_product(_e6, _e7);
    let _e10: Scalar = self_1675;
    let _e11: Scalar = scalar_reversal(_e10);
    let _e14: Scalar = self_1675;
    let _e15: PointAndPlane = other_1457;
    let _e16: PointAndPlane = scalar_point_and_plane_geometric_product(_e14, _e15);
    let _e18: Scalar = self_1675;
    let _e19: Scalar = scalar_reversal(_e18);
    let _e20: PointAndPlane = point_and_plane_scalar_geometric_product(_e16, _e19);
    return _e20;
}

fn scalar_rotor_geometric_quotient(self_1676: Scalar, other_1458: Rotor) -> Rotor {
    var self_1677: Scalar;
    var other_1459: Rotor;

    self_1677 = self_1676;
    other_1459 = other_1458;
    let _e6: Rotor = other_1459;
    let _e7: Rotor = rotor_inverse(_e6);
    let _e8: Scalar = self_1677;
    let _e10: Rotor = other_1459;
    let _e11: Rotor = rotor_inverse(_e10);
    let _e12: Rotor = scalar_rotor_geometric_product(_e8, _e11);
    return _e12;
}

fn scalar_rotor_transformation(self_1678: Scalar, other_1460: Rotor) -> Rotor {
    var self_1679: Scalar;
    var other_1461: Rotor;

    self_1679 = self_1678;
    other_1461 = other_1460;
    let _e6: Scalar = self_1679;
    let _e7: Rotor = other_1461;
    let _e8: Rotor = scalar_rotor_geometric_product(_e6, _e7);
    let _e10: Scalar = self_1679;
    let _e11: Scalar = scalar_reversal(_e10);
    let _e14: Scalar = self_1679;
    let _e15: Rotor = other_1461;
    let _e16: Rotor = scalar_rotor_geometric_product(_e14, _e15);
    let _e18: Scalar = self_1679;
    let _e19: Scalar = scalar_reversal(_e18);
    let _e20: Rotor = rotor_scalar_geometric_product(_e16, _e19);
    return _e20;
}

fn scalar_powi(self_1680: Scalar, exponent_6: i32) -> Scalar {
    var self_1681: Scalar;
    var exponent_7: i32;
    var local_3: Scalar;
    var x_3: Scalar;
    var y_3: Scalar;
    var n_3: i32;

    self_1681 = self_1680;
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
        let _e12: Scalar = self_1681;
        let _e13: Scalar = scalar_inverse(_e12);
        local_3 = _e13;
    } else {
        let _e14: Scalar = self_1681;
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

fn scalar_scalar_geometric_quotient(self_1682: Scalar, other_1462: Scalar) -> Scalar {
    var self_1683: Scalar;
    var other_1463: Scalar;

    self_1683 = self_1682;
    other_1463 = other_1462;
    let _e6: Scalar = other_1463;
    let _e7: Scalar = scalar_inverse(_e6);
    let _e8: Scalar = self_1683;
    let _e10: Scalar = other_1463;
    let _e11: Scalar = scalar_inverse(_e10);
    let _e12: Scalar = scalar_scalar_geometric_product(_e8, _e11);
    return _e12;
}

fn scalar_scalar_transformation(self_1684: Scalar, other_1464: Scalar) -> Scalar {
    var self_1685: Scalar;
    var other_1465: Scalar;

    self_1685 = self_1684;
    other_1465 = other_1464;
    let _e6: Scalar = self_1685;
    let _e7: Scalar = other_1465;
    let _e8: Scalar = scalar_scalar_geometric_product(_e6, _e7);
    let _e10: Scalar = self_1685;
    let _e11: Scalar = scalar_reversal(_e10);
    let _e14: Scalar = self_1685;
    let _e15: Scalar = other_1465;
    let _e16: Scalar = scalar_scalar_geometric_product(_e14, _e15);
    let _e18: Scalar = self_1685;
    let _e19: Scalar = scalar_reversal(_e18);
    let _e20: Scalar = scalar_scalar_geometric_product(_e16, _e19);
    return _e20;
}

fn scalar_translator_geometric_quotient(self_1686: Scalar, other_1466: Translator) -> Translator {
    var self_1687: Scalar;
    var other_1467: Translator;

    self_1687 = self_1686;
    other_1467 = other_1466;
    let _e6: Translator = other_1467;
    let _e7: Translator = translator_inverse(_e6);
    let _e8: Scalar = self_1687;
    let _e10: Translator = other_1467;
    let _e11: Translator = translator_inverse(_e10);
    let _e12: Translator = scalar_translator_geometric_product(_e8, _e11);
    return _e12;
}

fn scalar_translator_transformation(self_1688: Scalar, other_1468: Translator) -> Translator {
    var self_1689: Scalar;
    var other_1469: Translator;

    self_1689 = self_1688;
    other_1469 = other_1468;
    let _e6: Scalar = self_1689;
    let _e7: Translator = other_1469;
    let _e8: Translator = scalar_translator_geometric_product(_e6, _e7);
    let _e10: Scalar = self_1689;
    let _e11: Scalar = scalar_reversal(_e10);
    let _e14: Scalar = self_1689;
    let _e15: Translator = other_1469;
    let _e16: Translator = scalar_translator_geometric_product(_e14, _e15);
    let _e18: Scalar = self_1689;
    let _e19: Scalar = scalar_reversal(_e18);
    let _e20: Translator = translator_scalar_geometric_product(_e16, _e19);
    return _e20;
}

fn translator_line_geometric_quotient(self_1690: Translator, other_1470: Line) -> Motor {
    var self_1691: Translator;
    var other_1471: Line;

    self_1691 = self_1690;
    other_1471 = other_1470;
    let _e6: Line = other_1471;
    let _e7: Line = line_inverse(_e6);
    let _e8: Translator = self_1691;
    let _e10: Line = other_1471;
    let _e11: Line = line_inverse(_e10);
    let _e12: Motor = translator_line_geometric_product(_e8, _e11);
    return _e12;
}

fn translator_line_transformation(self_1692: Translator, other_1472: Line) -> Line {
    var self_1693: Translator;
    var other_1473: Line;

    self_1693 = self_1692;
    other_1473 = other_1472;
    let _e6: Translator = self_1693;
    let _e7: Line = other_1473;
    let _e8: Motor = translator_line_geometric_product(_e6, _e7);
    let _e10: Translator = self_1693;
    let _e11: Translator = translator_reversal(_e10);
    let _e14: Translator = self_1693;
    let _e15: Line = other_1473;
    let _e16: Motor = translator_line_geometric_product(_e14, _e15);
    let _e18: Translator = self_1693;
    let _e19: Translator = translator_reversal(_e18);
    let _e20: Motor = motor_translator_geometric_product(_e16, _e19);
    let _e23: Translator = self_1693;
    let _e24: Line = other_1473;
    let _e25: Motor = translator_line_geometric_product(_e23, _e24);
    let _e27: Translator = self_1693;
    let _e28: Translator = translator_reversal(_e27);
    let _e31: Translator = self_1693;
    let _e32: Line = other_1473;
    let _e33: Motor = translator_line_geometric_product(_e31, _e32);
    let _e35: Translator = self_1693;
    let _e36: Translator = translator_reversal(_e35);
    let _e37: Motor = motor_translator_geometric_product(_e33, _e36);
    let _e38: Line = motor_line_into(_e37);
    return _e38;
}

fn translator_motor_geometric_quotient(self_1694: Translator, other_1474: Motor) -> Motor {
    var self_1695: Translator;
    var other_1475: Motor;

    self_1695 = self_1694;
    other_1475 = other_1474;
    let _e6: Motor = other_1475;
    let _e7: Motor = motor_inverse(_e6);
    let _e8: Translator = self_1695;
    let _e10: Motor = other_1475;
    let _e11: Motor = motor_inverse(_e10);
    let _e12: Motor = translator_motor_geometric_product(_e8, _e11);
    return _e12;
}

fn translator_motor_transformation(self_1696: Translator, other_1476: Motor) -> Motor {
    var self_1697: Translator;
    var other_1477: Motor;

    self_1697 = self_1696;
    other_1477 = other_1476;
    let _e6: Translator = self_1697;
    let _e7: Motor = other_1477;
    let _e8: Motor = translator_motor_geometric_product(_e6, _e7);
    let _e10: Translator = self_1697;
    let _e11: Translator = translator_reversal(_e10);
    let _e14: Translator = self_1697;
    let _e15: Motor = other_1477;
    let _e16: Motor = translator_motor_geometric_product(_e14, _e15);
    let _e18: Translator = self_1697;
    let _e19: Translator = translator_reversal(_e18);
    let _e20: Motor = motor_translator_geometric_product(_e16, _e19);
    return _e20;
}

fn translator_multi_vector_geometric_quotient(self_1698: Translator, other_1478: MultiVector) -> MultiVector {
    var self_1699: Translator;
    var other_1479: MultiVector;

    self_1699 = self_1698;
    other_1479 = other_1478;
    let _e6: MultiVector = other_1479;
    let _e7: MultiVector = multi_vector_inverse(_e6);
    let _e8: Translator = self_1699;
    let _e10: MultiVector = other_1479;
    let _e11: MultiVector = multi_vector_inverse(_e10);
    let _e12: MultiVector = translator_multi_vector_geometric_product(_e8, _e11);
    return _e12;
}

fn translator_multi_vector_transformation(self_1700: Translator, other_1480: MultiVector) -> MultiVector {
    var self_1701: Translator;
    var other_1481: MultiVector;

    self_1701 = self_1700;
    other_1481 = other_1480;
    let _e6: Translator = self_1701;
    let _e7: MultiVector = other_1481;
    let _e8: MultiVector = translator_multi_vector_geometric_product(_e6, _e7);
    let _e10: Translator = self_1701;
    let _e11: Translator = translator_reversal(_e10);
    let _e14: Translator = self_1701;
    let _e15: MultiVector = other_1481;
    let _e16: MultiVector = translator_multi_vector_geometric_product(_e14, _e15);
    let _e18: Translator = self_1701;
    let _e19: Translator = translator_reversal(_e18);
    let _e20: MultiVector = multi_vector_translator_geometric_product(_e16, _e19);
    return _e20;
}

fn translator_point_and_plane_geometric_quotient(self_1702: Translator, other_1482: PointAndPlane) -> PointAndPlane {
    var self_1703: Translator;
    var other_1483: PointAndPlane;

    self_1703 = self_1702;
    other_1483 = other_1482;
    let _e6: PointAndPlane = other_1483;
    let _e7: PointAndPlane = point_and_plane_inverse(_e6);
    let _e8: Translator = self_1703;
    let _e10: PointAndPlane = other_1483;
    let _e11: PointAndPlane = point_and_plane_inverse(_e10);
    let _e12: PointAndPlane = translator_point_and_plane_geometric_product(_e8, _e11);
    return _e12;
}

fn translator_point_and_plane_transformation(self_1704: Translator, other_1484: PointAndPlane) -> PointAndPlane {
    var self_1705: Translator;
    var other_1485: PointAndPlane;

    self_1705 = self_1704;
    other_1485 = other_1484;
    let _e6: Translator = self_1705;
    let _e7: PointAndPlane = other_1485;
    let _e8: PointAndPlane = translator_point_and_plane_geometric_product(_e6, _e7);
    let _e10: Translator = self_1705;
    let _e11: Translator = translator_reversal(_e10);
    let _e14: Translator = self_1705;
    let _e15: PointAndPlane = other_1485;
    let _e16: PointAndPlane = translator_point_and_plane_geometric_product(_e14, _e15);
    let _e18: Translator = self_1705;
    let _e19: Translator = translator_reversal(_e18);
    let _e20: PointAndPlane = point_and_plane_translator_geometric_product(_e16, _e19);
    return _e20;
}

fn translator_rotor_geometric_quotient(self_1706: Translator, other_1486: Rotor) -> Motor {
    var self_1707: Translator;
    var other_1487: Rotor;

    self_1707 = self_1706;
    other_1487 = other_1486;
    let _e6: Rotor = other_1487;
    let _e7: Rotor = rotor_inverse(_e6);
    let _e8: Translator = self_1707;
    let _e10: Rotor = other_1487;
    let _e11: Rotor = rotor_inverse(_e10);
    let _e12: Motor = translator_rotor_geometric_product(_e8, _e11);
    return _e12;
}

fn translator_rotor_transformation(self_1708: Translator, other_1488: Rotor) -> Rotor {
    var self_1709: Translator;
    var other_1489: Rotor;

    self_1709 = self_1708;
    other_1489 = other_1488;
    let _e6: Translator = self_1709;
    let _e7: Rotor = other_1489;
    let _e8: Motor = translator_rotor_geometric_product(_e6, _e7);
    let _e10: Translator = self_1709;
    let _e11: Translator = translator_reversal(_e10);
    let _e14: Translator = self_1709;
    let _e15: Rotor = other_1489;
    let _e16: Motor = translator_rotor_geometric_product(_e14, _e15);
    let _e18: Translator = self_1709;
    let _e19: Translator = translator_reversal(_e18);
    let _e20: Motor = motor_translator_geometric_product(_e16, _e19);
    let _e23: Translator = self_1709;
    let _e24: Rotor = other_1489;
    let _e25: Motor = translator_rotor_geometric_product(_e23, _e24);
    let _e27: Translator = self_1709;
    let _e28: Translator = translator_reversal(_e27);
    let _e31: Translator = self_1709;
    let _e32: Rotor = other_1489;
    let _e33: Motor = translator_rotor_geometric_product(_e31, _e32);
    let _e35: Translator = self_1709;
    let _e36: Translator = translator_reversal(_e35);
    let _e37: Motor = motor_translator_geometric_product(_e33, _e36);
    let _e38: Rotor = motor_rotor_into(_e37);
    return _e38;
}

fn translator_scalar_geometric_quotient(self_1710: Translator, other_1490: Scalar) -> Translator {
    var self_1711: Translator;
    var other_1491: Scalar;

    self_1711 = self_1710;
    other_1491 = other_1490;
    let _e6: Scalar = other_1491;
    let _e7: Scalar = scalar_inverse(_e6);
    let _e8: Translator = self_1711;
    let _e10: Scalar = other_1491;
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

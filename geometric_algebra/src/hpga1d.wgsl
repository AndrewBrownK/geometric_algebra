struct Scalar {
    g0_: f32,
}

struct SplitComplexNumber {
    g0_: vec2<f32>,
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

fn scalar_split_complex_number_add(self_28: Scalar, other_20: SplitComplexNumber) -> SplitComplexNumber {
    var self_29: Scalar;
    var other_21: SplitComplexNumber;

    self_29 = self_28;
    other_21 = other_20;
    let _e4: Scalar = self_29;
    let _e11: SplitComplexNumber = other_21;
    return SplitComplexNumber(((vec2<f32>(_e4.g0_) * vec2<f32>(1.0, 0.0)) + _e11.g0_));
}

fn scalar_split_complex_number_sub(self_30: Scalar, other_22: SplitComplexNumber) -> SplitComplexNumber {
    var self_31: Scalar;
    var other_23: SplitComplexNumber;

    self_31 = self_30;
    other_23 = other_22;
    let _e4: Scalar = self_31;
    let _e11: SplitComplexNumber = other_23;
    return SplitComplexNumber(((vec2<f32>(_e4.g0_) * vec2<f32>(1.0, 0.0)) - _e11.g0_));
}

fn scalar_split_complex_number_geometric_product(self_32: Scalar, other_24: SplitComplexNumber) -> SplitComplexNumber {
    var self_33: Scalar;
    var other_25: SplitComplexNumber;

    self_33 = self_32;
    other_25 = other_24;
    let _e4: Scalar = self_33;
    let _e7: SplitComplexNumber = other_25;
    return SplitComplexNumber((vec2<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_split_complex_number_regressive_product(self_34: Scalar, other_26: SplitComplexNumber) -> Scalar {
    var self_35: Scalar;
    var other_27: SplitComplexNumber;

    self_35 = self_34;
    other_27 = other_26;
    let _e4: Scalar = self_35;
    let _e6: SplitComplexNumber = other_27;
    return Scalar((_e4.g0_ * _e6.g0_.y));
}

fn scalar_split_complex_number_outer_product(self_36: Scalar, other_28: SplitComplexNumber) -> SplitComplexNumber {
    var self_37: Scalar;
    var other_29: SplitComplexNumber;

    self_37 = self_36;
    other_29 = other_28;
    let _e4: Scalar = self_37;
    let _e7: SplitComplexNumber = other_29;
    return SplitComplexNumber((vec2<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_split_complex_number_inner_product(self_38: Scalar, other_30: SplitComplexNumber) -> SplitComplexNumber {
    var self_39: Scalar;
    var other_31: SplitComplexNumber;

    self_39 = self_38;
    other_31 = other_30;
    let _e4: Scalar = self_39;
    let _e7: SplitComplexNumber = other_31;
    return SplitComplexNumber((vec2<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_split_complex_number_left_contraction(self_40: Scalar, other_32: SplitComplexNumber) -> SplitComplexNumber {
    var self_41: Scalar;
    var other_33: SplitComplexNumber;

    self_41 = self_40;
    other_33 = other_32;
    let _e4: Scalar = self_41;
    let _e7: SplitComplexNumber = other_33;
    return SplitComplexNumber((vec2<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_split_complex_number_right_contraction(self_42: Scalar, other_34: SplitComplexNumber) -> Scalar {
    var self_43: Scalar;
    var other_35: SplitComplexNumber;

    self_43 = self_42;
    other_35 = other_34;
    let _e4: Scalar = self_43;
    let _e6: SplitComplexNumber = other_35;
    return Scalar((_e4.g0_ * _e6.g0_.x));
}

fn scalar_split_complex_number_scalar_product(self_44: Scalar, other_36: SplitComplexNumber) -> Scalar {
    var self_45: Scalar;
    var other_37: SplitComplexNumber;

    self_45 = self_44;
    other_37 = other_36;
    let _e4: Scalar = self_45;
    let _e6: SplitComplexNumber = other_37;
    return Scalar((_e4.g0_ * _e6.g0_.x));
}

fn scalar_squared_magnitude(self_46: Scalar) -> Scalar {
    var self_47: Scalar;

    self_47 = self_46;
    let _e4: Scalar = self_47;
    let _e5: Scalar = scalar_reversal(_e4);
    let _e6: Scalar = self_47;
    let _e8: Scalar = self_47;
    let _e9: Scalar = scalar_reversal(_e8);
    let _e10: Scalar = scalar_scalar_scalar_product(_e6, _e9);
    return _e10;
}

fn scalar_magnitude(self_48: Scalar) -> Scalar {
    var self_49: Scalar;

    self_49 = self_48;
    let _e3: Scalar = self_49;
    let _e4: Scalar = scalar_squared_magnitude(_e3);
    let _e7: Scalar = self_49;
    let _e8: Scalar = scalar_squared_magnitude(_e7);
    return Scalar(sqrt(_e8.g0_));
}

fn scalar_scale(self_50: Scalar, other_38: f32) -> Scalar {
    var self_51: Scalar;
    var other_39: f32;

    self_51 = self_50;
    other_39 = other_38;
    let _e5: f32 = other_39;
    let _e7: Scalar = self_51;
    let _e8: f32 = other_39;
    let _e10: Scalar = scalar_scalar_geometric_product(_e7, Scalar(_e8));
    return _e10;
}

fn scalar_signum(self_52: Scalar) -> Scalar {
    var self_53: Scalar;

    self_53 = self_52;
    let _e5: Scalar = self_53;
    let _e6: Scalar = scalar_magnitude(_e5);
    let _e10: Scalar = self_53;
    let _e13: Scalar = self_53;
    let _e14: Scalar = scalar_magnitude(_e13);
    let _e18: Scalar = scalar_scalar_geometric_product(_e10, Scalar((1.0 / _e14.g0_)));
    return _e18;
}

fn scalar_inverse(self_54: Scalar) -> Scalar {
    var self_55: Scalar;

    self_55 = self_54;
    let _e3: Scalar = self_55;
    let _e4: Scalar = scalar_reversal(_e3);
    let _e7: Scalar = self_55;
    let _e8: Scalar = scalar_squared_magnitude(_e7);
    let _e13: Scalar = self_55;
    let _e14: Scalar = scalar_reversal(_e13);
    let _e17: Scalar = self_55;
    let _e18: Scalar = scalar_squared_magnitude(_e17);
    let _e22: Scalar = scalar_scalar_geometric_product(_e14, Scalar((1.0 / _e18.g0_)));
    return _e22;
}

fn split_complex_number_zero() -> SplitComplexNumber {
    return SplitComplexNumber(vec2<f32>(0.0));
}

fn split_complex_number_one() -> SplitComplexNumber {
    return SplitComplexNumber(vec2<f32>(1.0, 0.0));
}

fn split_complex_number_neg(self_56: SplitComplexNumber) -> SplitComplexNumber {
    var self_57: SplitComplexNumber;

    self_57 = self_56;
    let _e2: SplitComplexNumber = self_57;
    return SplitComplexNumber((_e2.g0_ * vec2<f32>(-(1.0))));
}

fn split_complex_number_automorphism(self_58: SplitComplexNumber) -> SplitComplexNumber {
    var self_59: SplitComplexNumber;

    self_59 = self_58;
    let _e2: SplitComplexNumber = self_59;
    return SplitComplexNumber(_e2.g0_);
}

fn split_complex_number_reversal(self_60: SplitComplexNumber) -> SplitComplexNumber {
    var self_61: SplitComplexNumber;

    self_61 = self_60;
    let _e2: SplitComplexNumber = self_61;
    return SplitComplexNumber((_e2.g0_ * vec2<f32>(1.0, -(1.0))));
}

fn split_complex_number_conjugation(self_62: SplitComplexNumber) -> SplitComplexNumber {
    var self_63: SplitComplexNumber;

    self_63 = self_62;
    let _e2: SplitComplexNumber = self_63;
    return SplitComplexNumber((_e2.g0_ * vec2<f32>(1.0, -(1.0))));
}

fn split_complex_number_dual(self_64: SplitComplexNumber) -> SplitComplexNumber {
    var self_65: SplitComplexNumber;

    self_65 = self_64;
    let _e2: SplitComplexNumber = self_65;
    return SplitComplexNumber(_e2.g0_.yx);
}

fn split_complex_number_scalar_into(self_66: SplitComplexNumber) -> Scalar {
    var self_67: SplitComplexNumber;

    self_67 = self_66;
    let _e2: SplitComplexNumber = self_67;
    return Scalar(_e2.g0_.x);
}

fn split_complex_number_scalar_add(self_68: SplitComplexNumber, other_40: Scalar) -> SplitComplexNumber {
    var self_69: SplitComplexNumber;
    var other_41: Scalar;

    self_69 = self_68;
    other_41 = other_40;
    let _e4: SplitComplexNumber = self_69;
    let _e6: Scalar = other_41;
    return SplitComplexNumber((_e4.g0_ + (vec2<f32>(_e6.g0_) * vec2<f32>(1.0, 0.0))));
}

fn split_complex_number_scalar_sub(self_70: SplitComplexNumber, other_42: Scalar) -> SplitComplexNumber {
    var self_71: SplitComplexNumber;
    var other_43: Scalar;

    self_71 = self_70;
    other_43 = other_42;
    let _e4: SplitComplexNumber = self_71;
    let _e6: Scalar = other_43;
    return SplitComplexNumber((_e4.g0_ - (vec2<f32>(_e6.g0_) * vec2<f32>(1.0, 0.0))));
}

fn split_complex_number_scalar_geometric_product(self_72: SplitComplexNumber, other_44: Scalar) -> SplitComplexNumber {
    var self_73: SplitComplexNumber;
    var other_45: Scalar;

    self_73 = self_72;
    other_45 = other_44;
    let _e4: SplitComplexNumber = self_73;
    let _e6: Scalar = other_45;
    return SplitComplexNumber((_e4.g0_ * vec2<f32>(_e6.g0_)));
}

fn split_complex_number_scalar_regressive_product(self_74: SplitComplexNumber, other_46: Scalar) -> Scalar {
    var self_75: SplitComplexNumber;
    var other_47: Scalar;

    self_75 = self_74;
    other_47 = other_46;
    let _e4: SplitComplexNumber = self_75;
    let _e7: Scalar = other_47;
    return Scalar((_e4.g0_.y * _e7.g0_));
}

fn split_complex_number_scalar_outer_product(self_76: SplitComplexNumber, other_48: Scalar) -> SplitComplexNumber {
    var self_77: SplitComplexNumber;
    var other_49: Scalar;

    self_77 = self_76;
    other_49 = other_48;
    let _e4: SplitComplexNumber = self_77;
    let _e6: Scalar = other_49;
    return SplitComplexNumber((_e4.g0_ * vec2<f32>(_e6.g0_)));
}

fn split_complex_number_scalar_inner_product(self_78: SplitComplexNumber, other_50: Scalar) -> SplitComplexNumber {
    var self_79: SplitComplexNumber;
    var other_51: Scalar;

    self_79 = self_78;
    other_51 = other_50;
    let _e4: SplitComplexNumber = self_79;
    let _e6: Scalar = other_51;
    return SplitComplexNumber((_e4.g0_ * vec2<f32>(_e6.g0_)));
}

fn split_complex_number_scalar_left_contraction(self_80: SplitComplexNumber, other_52: Scalar) -> Scalar {
    var self_81: SplitComplexNumber;
    var other_53: Scalar;

    self_81 = self_80;
    other_53 = other_52;
    let _e4: SplitComplexNumber = self_81;
    let _e7: Scalar = other_53;
    return Scalar((_e4.g0_.x * _e7.g0_));
}

fn split_complex_number_scalar_right_contraction(self_82: SplitComplexNumber, other_54: Scalar) -> SplitComplexNumber {
    var self_83: SplitComplexNumber;
    var other_55: Scalar;

    self_83 = self_82;
    other_55 = other_54;
    let _e4: SplitComplexNumber = self_83;
    let _e6: Scalar = other_55;
    return SplitComplexNumber((_e4.g0_ * vec2<f32>(_e6.g0_)));
}

fn split_complex_number_scalar_scalar_product(self_84: SplitComplexNumber, other_56: Scalar) -> Scalar {
    var self_85: SplitComplexNumber;
    var other_57: Scalar;

    self_85 = self_84;
    other_57 = other_56;
    let _e4: SplitComplexNumber = self_85;
    let _e7: Scalar = other_57;
    return Scalar((_e4.g0_.x * _e7.g0_));
}

fn split_complex_number_split_complex_number_add(self_86: SplitComplexNumber, other_58: SplitComplexNumber) -> SplitComplexNumber {
    var self_87: SplitComplexNumber;
    var other_59: SplitComplexNumber;

    self_87 = self_86;
    other_59 = other_58;
    let _e4: SplitComplexNumber = self_87;
    let _e6: SplitComplexNumber = other_59;
    return SplitComplexNumber((_e4.g0_ + _e6.g0_));
}

fn split_complex_number_split_complex_number_sub(self_88: SplitComplexNumber, other_60: SplitComplexNumber) -> SplitComplexNumber {
    var self_89: SplitComplexNumber;
    var other_61: SplitComplexNumber;

    self_89 = self_88;
    other_61 = other_60;
    let _e4: SplitComplexNumber = self_89;
    let _e6: SplitComplexNumber = other_61;
    return SplitComplexNumber((_e4.g0_ - _e6.g0_));
}

fn split_complex_number_split_complex_number_mul(self_90: SplitComplexNumber, other_62: SplitComplexNumber) -> SplitComplexNumber {
    var self_91: SplitComplexNumber;
    var other_63: SplitComplexNumber;

    self_91 = self_90;
    other_63 = other_62;
    let _e4: SplitComplexNumber = self_91;
    let _e6: SplitComplexNumber = other_63;
    return SplitComplexNumber((_e4.g0_ * _e6.g0_));
}

fn split_complex_number_split_complex_number_div(self_92: SplitComplexNumber, other_64: SplitComplexNumber) -> SplitComplexNumber {
    var self_93: SplitComplexNumber;
    var other_65: SplitComplexNumber;

    self_93 = self_92;
    other_65 = other_64;
    let _e4: SplitComplexNumber = self_93;
    let _e7: SplitComplexNumber = self_93;
    let _e15: SplitComplexNumber = other_65;
    let _e18: SplitComplexNumber = other_65;
    return SplitComplexNumber((((vec2<f32>(_e4.g0_.x, _e7.g0_.y) * vec2<f32>(1.0, 1.0)) / vec2<f32>(_e15.g0_.x, _e18.g0_.y)) * vec2<f32>(1.0, 1.0)));
}

fn split_complex_number_split_complex_number_geometric_product(self_94: SplitComplexNumber, other_66: SplitComplexNumber) -> SplitComplexNumber {
    var self_95: SplitComplexNumber;
    var other_67: SplitComplexNumber;

    self_95 = self_94;
    other_67 = other_66;
    let _e4: SplitComplexNumber = self_95;
    let _e8: SplitComplexNumber = other_67;
    let _e11: SplitComplexNumber = self_95;
    let _e15: SplitComplexNumber = other_67;
    return SplitComplexNumber(((vec2<f32>(_e4.g0_.x) * _e8.g0_) + (vec2<f32>(_e11.g0_.y) * _e15.g0_.yx)));
}

fn split_complex_number_split_complex_number_regressive_product(self_96: SplitComplexNumber, other_68: SplitComplexNumber) -> SplitComplexNumber {
    var self_97: SplitComplexNumber;
    var other_69: SplitComplexNumber;

    self_97 = self_96;
    other_69 = other_68;
    let _e4: SplitComplexNumber = self_97;
    let _e8: SplitComplexNumber = other_69;
    let _e11: SplitComplexNumber = self_97;
    let _e15: SplitComplexNumber = other_69;
    return SplitComplexNumber(((vec2<f32>(_e4.g0_.y) * _e8.g0_) + ((vec2<f32>(_e11.g0_.x) * _e15.g0_.yx) * vec2<f32>(1.0, 0.0))));
}

fn split_complex_number_split_complex_number_outer_product(self_98: SplitComplexNumber, other_70: SplitComplexNumber) -> SplitComplexNumber {
    var self_99: SplitComplexNumber;
    var other_71: SplitComplexNumber;

    self_99 = self_98;
    other_71 = other_70;
    let _e4: SplitComplexNumber = self_99;
    let _e8: SplitComplexNumber = other_71;
    let _e11: SplitComplexNumber = self_99;
    let _e13: SplitComplexNumber = other_71;
    return SplitComplexNumber(((vec2<f32>(_e4.g0_.x) * _e8.g0_) + ((_e11.g0_ * vec2<f32>(_e13.g0_.x)) * vec2<f32>(0.0, 1.0))));
}

fn split_complex_number_split_complex_number_inner_product(self_100: SplitComplexNumber, other_72: SplitComplexNumber) -> SplitComplexNumber {
    var self_101: SplitComplexNumber;
    var other_73: SplitComplexNumber;

    self_101 = self_100;
    other_73 = other_72;
    let _e4: SplitComplexNumber = self_101;
    let _e8: SplitComplexNumber = other_73;
    let _e11: SplitComplexNumber = self_101;
    let _e15: SplitComplexNumber = other_73;
    return SplitComplexNumber(((vec2<f32>(_e4.g0_.x) * _e8.g0_) + (vec2<f32>(_e11.g0_.y) * _e15.g0_.yx)));
}

fn split_complex_number_split_complex_number_left_contraction(self_102: SplitComplexNumber, other_74: SplitComplexNumber) -> SplitComplexNumber {
    var self_103: SplitComplexNumber;
    var other_75: SplitComplexNumber;

    self_103 = self_102;
    other_75 = other_74;
    let _e4: SplitComplexNumber = self_103;
    let _e8: SplitComplexNumber = other_75;
    let _e11: SplitComplexNumber = self_103;
    let _e14: SplitComplexNumber = other_75;
    return SplitComplexNumber(((vec2<f32>(_e4.g0_.x) * _e8.g0_) + ((_e11.g0_.yx * _e14.g0_.yx) * vec2<f32>(1.0, 0.0))));
}

fn split_complex_number_split_complex_number_right_contraction(self_104: SplitComplexNumber, other_76: SplitComplexNumber) -> SplitComplexNumber {
    var self_105: SplitComplexNumber;
    var other_77: SplitComplexNumber;

    self_105 = self_104;
    other_77 = other_76;
    let _e4: SplitComplexNumber = self_105;
    let _e8: SplitComplexNumber = other_77;
    let _e12: SplitComplexNumber = self_105;
    let _e16: SplitComplexNumber = other_77;
    return SplitComplexNumber(((vec2<f32>(_e4.g0_.y) * _e8.g0_.yx) + ((vec2<f32>(_e12.g0_.x) * vec2<f32>(_e16.g0_.x)) * vec2<f32>(1.0, 0.0))));
}

fn split_complex_number_split_complex_number_scalar_product(self_106: SplitComplexNumber, other_78: SplitComplexNumber) -> Scalar {
    var self_107: SplitComplexNumber;
    var other_79: SplitComplexNumber;

    self_107 = self_106;
    other_79 = other_78;
    let _e4: SplitComplexNumber = self_107;
    let _e7: SplitComplexNumber = other_79;
    let _e11: SplitComplexNumber = self_107;
    let _e14: SplitComplexNumber = other_79;
    return Scalar(((_e4.g0_.x * _e7.g0_.x) + (_e11.g0_.y * _e14.g0_.y)));
}

fn split_complex_number_squared_magnitude(self_108: SplitComplexNumber) -> Scalar {
    var self_109: SplitComplexNumber;

    self_109 = self_108;
    let _e4: SplitComplexNumber = self_109;
    let _e5: SplitComplexNumber = split_complex_number_reversal(_e4);
    let _e6: SplitComplexNumber = self_109;
    let _e8: SplitComplexNumber = self_109;
    let _e9: SplitComplexNumber = split_complex_number_reversal(_e8);
    let _e10: Scalar = split_complex_number_split_complex_number_scalar_product(_e6, _e9);
    return _e10;
}

fn split_complex_number_magnitude(self_110: SplitComplexNumber) -> Scalar {
    var self_111: SplitComplexNumber;

    self_111 = self_110;
    let _e3: SplitComplexNumber = self_111;
    let _e4: Scalar = split_complex_number_squared_magnitude(_e3);
    let _e7: SplitComplexNumber = self_111;
    let _e8: Scalar = split_complex_number_squared_magnitude(_e7);
    return Scalar(sqrt(_e8.g0_));
}

fn split_complex_number_scale(self_112: SplitComplexNumber, other_80: f32) -> SplitComplexNumber {
    var self_113: SplitComplexNumber;
    var other_81: f32;

    self_113 = self_112;
    other_81 = other_80;
    let _e5: f32 = other_81;
    let _e7: SplitComplexNumber = self_113;
    let _e8: f32 = other_81;
    let _e10: SplitComplexNumber = split_complex_number_scalar_geometric_product(_e7, Scalar(_e8));
    return _e10;
}

fn split_complex_number_signum(self_114: SplitComplexNumber) -> SplitComplexNumber {
    var self_115: SplitComplexNumber;

    self_115 = self_114;
    let _e5: SplitComplexNumber = self_115;
    let _e6: Scalar = split_complex_number_magnitude(_e5);
    let _e10: SplitComplexNumber = self_115;
    let _e13: SplitComplexNumber = self_115;
    let _e14: Scalar = split_complex_number_magnitude(_e13);
    let _e18: SplitComplexNumber = split_complex_number_scalar_geometric_product(_e10, Scalar((1.0 / _e14.g0_)));
    return _e18;
}

fn split_complex_number_inverse(self_116: SplitComplexNumber) -> SplitComplexNumber {
    var self_117: SplitComplexNumber;

    self_117 = self_116;
    let _e3: SplitComplexNumber = self_117;
    let _e4: SplitComplexNumber = split_complex_number_reversal(_e3);
    let _e7: SplitComplexNumber = self_117;
    let _e8: Scalar = split_complex_number_squared_magnitude(_e7);
    let _e13: SplitComplexNumber = self_117;
    let _e14: SplitComplexNumber = split_complex_number_reversal(_e13);
    let _e17: SplitComplexNumber = self_117;
    let _e18: Scalar = split_complex_number_squared_magnitude(_e17);
    let _e22: SplitComplexNumber = split_complex_number_scalar_geometric_product(_e14, Scalar((1.0 / _e18.g0_)));
    return _e22;
}

fn scalar_powi(self_118: Scalar, exponent: i32) -> Scalar {
    var self_119: Scalar;
    var exponent_1: i32;
    var local: Scalar;
    var x: Scalar;
    var y: Scalar;
    var n: i32;

    self_119 = self_118;
    exponent_1 = exponent;
    let _e4: i32 = exponent_1;
    if (_e4 == 0) {
        {
            let _e7: Scalar = scalar_one();
            return _e7;
        }
    }
    let _e8: i32 = exponent_1;
    if (_e8 < 0) {
        let _e12: Scalar = self_119;
        let _e13: Scalar = scalar_inverse(_e12);
        local = _e13;
    } else {
        let _e14: Scalar = self_119;
        local = _e14;
    }
    let _e16: Scalar = local;
    x = _e16;
    let _e18: Scalar = scalar_one();
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
                    let _e35: Scalar = x;
                    let _e36: Scalar = y;
                    let _e37: Scalar = scalar_scalar_geometric_product(_e35, _e36);
                    y = _e37;
                }
            }
            let _e40: Scalar = x;
            let _e41: Scalar = x;
            let _e42: Scalar = scalar_scalar_geometric_product(_e40, _e41);
            x = _e42;
            let _e43: i32 = n;
            n = (_e43 >> u32(1));
        }
    }
    let _e49: Scalar = x;
    let _e50: Scalar = y;
    let _e51: Scalar = scalar_scalar_geometric_product(_e49, _e50);
    return _e51;
}

fn scalar_scalar_geometric_quotient(self_120: Scalar, other_82: Scalar) -> Scalar {
    var self_121: Scalar;
    var other_83: Scalar;

    self_121 = self_120;
    other_83 = other_82;
    let _e6: Scalar = other_83;
    let _e7: Scalar = scalar_inverse(_e6);
    let _e8: Scalar = self_121;
    let _e10: Scalar = other_83;
    let _e11: Scalar = scalar_inverse(_e10);
    let _e12: Scalar = scalar_scalar_geometric_product(_e8, _e11);
    return _e12;
}

fn scalar_scalar_transformation(self_122: Scalar, other_84: Scalar) -> Scalar {
    var self_123: Scalar;
    var other_85: Scalar;

    self_123 = self_122;
    other_85 = other_84;
    let _e6: Scalar = self_123;
    let _e7: Scalar = other_85;
    let _e8: Scalar = scalar_scalar_geometric_product(_e6, _e7);
    let _e10: Scalar = self_123;
    let _e11: Scalar = scalar_reversal(_e10);
    let _e14: Scalar = self_123;
    let _e15: Scalar = other_85;
    let _e16: Scalar = scalar_scalar_geometric_product(_e14, _e15);
    let _e18: Scalar = self_123;
    let _e19: Scalar = scalar_reversal(_e18);
    let _e20: Scalar = scalar_scalar_geometric_product(_e16, _e19);
    return _e20;
}

fn scalar_split_complex_number_geometric_quotient(self_124: Scalar, other_86: SplitComplexNumber) -> SplitComplexNumber {
    var self_125: Scalar;
    var other_87: SplitComplexNumber;

    self_125 = self_124;
    other_87 = other_86;
    let _e6: SplitComplexNumber = other_87;
    let _e7: SplitComplexNumber = split_complex_number_inverse(_e6);
    let _e8: Scalar = self_125;
    let _e10: SplitComplexNumber = other_87;
    let _e11: SplitComplexNumber = split_complex_number_inverse(_e10);
    let _e12: SplitComplexNumber = scalar_split_complex_number_geometric_product(_e8, _e11);
    return _e12;
}

fn scalar_split_complex_number_transformation(self_126: Scalar, other_88: SplitComplexNumber) -> SplitComplexNumber {
    var self_127: Scalar;
    var other_89: SplitComplexNumber;

    self_127 = self_126;
    other_89 = other_88;
    let _e6: Scalar = self_127;
    let _e7: SplitComplexNumber = other_89;
    let _e8: SplitComplexNumber = scalar_split_complex_number_geometric_product(_e6, _e7);
    let _e10: Scalar = self_127;
    let _e11: Scalar = scalar_reversal(_e10);
    let _e14: Scalar = self_127;
    let _e15: SplitComplexNumber = other_89;
    let _e16: SplitComplexNumber = scalar_split_complex_number_geometric_product(_e14, _e15);
    let _e18: Scalar = self_127;
    let _e19: Scalar = scalar_reversal(_e18);
    let _e20: SplitComplexNumber = split_complex_number_scalar_geometric_product(_e16, _e19);
    return _e20;
}

fn split_complex_number_scalar_geometric_quotient(self_128: SplitComplexNumber, other_90: Scalar) -> SplitComplexNumber {
    var self_129: SplitComplexNumber;
    var other_91: Scalar;

    self_129 = self_128;
    other_91 = other_90;
    let _e6: Scalar = other_91;
    let _e7: Scalar = scalar_inverse(_e6);
    let _e8: SplitComplexNumber = self_129;
    let _e10: Scalar = other_91;
    let _e11: Scalar = scalar_inverse(_e10);
    let _e12: SplitComplexNumber = split_complex_number_scalar_geometric_product(_e8, _e11);
    return _e12;
}

fn split_complex_number_scalar_transformation(self_130: SplitComplexNumber, other_92: Scalar) -> Scalar {
    var self_131: SplitComplexNumber;
    var other_93: Scalar;

    self_131 = self_130;
    other_93 = other_92;
    let _e6: SplitComplexNumber = self_131;
    let _e7: Scalar = other_93;
    let _e8: SplitComplexNumber = split_complex_number_scalar_geometric_product(_e6, _e7);
    let _e10: SplitComplexNumber = self_131;
    let _e11: SplitComplexNumber = split_complex_number_reversal(_e10);
    let _e14: SplitComplexNumber = self_131;
    let _e15: Scalar = other_93;
    let _e16: SplitComplexNumber = split_complex_number_scalar_geometric_product(_e14, _e15);
    let _e18: SplitComplexNumber = self_131;
    let _e19: SplitComplexNumber = split_complex_number_reversal(_e18);
    let _e20: SplitComplexNumber = split_complex_number_split_complex_number_geometric_product(_e16, _e19);
    let _e23: SplitComplexNumber = self_131;
    let _e24: Scalar = other_93;
    let _e25: SplitComplexNumber = split_complex_number_scalar_geometric_product(_e23, _e24);
    let _e27: SplitComplexNumber = self_131;
    let _e28: SplitComplexNumber = split_complex_number_reversal(_e27);
    let _e31: SplitComplexNumber = self_131;
    let _e32: Scalar = other_93;
    let _e33: SplitComplexNumber = split_complex_number_scalar_geometric_product(_e31, _e32);
    let _e35: SplitComplexNumber = self_131;
    let _e36: SplitComplexNumber = split_complex_number_reversal(_e35);
    let _e37: SplitComplexNumber = split_complex_number_split_complex_number_geometric_product(_e33, _e36);
    let _e38: Scalar = split_complex_number_scalar_into(_e37);
    return _e38;
}

fn split_complex_number_powi(self_132: SplitComplexNumber, exponent_2: i32) -> SplitComplexNumber {
    var self_133: SplitComplexNumber;
    var exponent_3: i32;
    var local_1: SplitComplexNumber;
    var x_1: SplitComplexNumber;
    var y_1: SplitComplexNumber;
    var n_1: i32;

    self_133 = self_132;
    exponent_3 = exponent_2;
    let _e4: i32 = exponent_3;
    if (_e4 == 0) {
        {
            let _e7: SplitComplexNumber = split_complex_number_one();
            return _e7;
        }
    }
    let _e8: i32 = exponent_3;
    if (_e8 < 0) {
        let _e12: SplitComplexNumber = self_133;
        let _e13: SplitComplexNumber = split_complex_number_inverse(_e12);
        local_1 = _e13;
    } else {
        let _e14: SplitComplexNumber = self_133;
        local_1 = _e14;
    }
    let _e16: SplitComplexNumber = local_1;
    x_1 = _e16;
    let _e18: SplitComplexNumber = split_complex_number_one();
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
                    let _e35: SplitComplexNumber = x_1;
                    let _e36: SplitComplexNumber = y_1;
                    let _e37: SplitComplexNumber = split_complex_number_split_complex_number_geometric_product(_e35, _e36);
                    y_1 = _e37;
                }
            }
            let _e40: SplitComplexNumber = x_1;
            let _e41: SplitComplexNumber = x_1;
            let _e42: SplitComplexNumber = split_complex_number_split_complex_number_geometric_product(_e40, _e41);
            x_1 = _e42;
            let _e43: i32 = n_1;
            n_1 = (_e43 >> u32(1));
        }
    }
    let _e49: SplitComplexNumber = x_1;
    let _e50: SplitComplexNumber = y_1;
    let _e51: SplitComplexNumber = split_complex_number_split_complex_number_geometric_product(_e49, _e50);
    return _e51;
}

fn split_complex_number_split_complex_number_geometric_quotient(self_134: SplitComplexNumber, other_94: SplitComplexNumber) -> SplitComplexNumber {
    var self_135: SplitComplexNumber;
    var other_95: SplitComplexNumber;

    self_135 = self_134;
    other_95 = other_94;
    let _e6: SplitComplexNumber = other_95;
    let _e7: SplitComplexNumber = split_complex_number_inverse(_e6);
    let _e8: SplitComplexNumber = self_135;
    let _e10: SplitComplexNumber = other_95;
    let _e11: SplitComplexNumber = split_complex_number_inverse(_e10);
    let _e12: SplitComplexNumber = split_complex_number_split_complex_number_geometric_product(_e8, _e11);
    return _e12;
}

fn split_complex_number_split_complex_number_transformation(self_136: SplitComplexNumber, other_96: SplitComplexNumber) -> SplitComplexNumber {
    var self_137: SplitComplexNumber;
    var other_97: SplitComplexNumber;

    self_137 = self_136;
    other_97 = other_96;
    let _e6: SplitComplexNumber = self_137;
    let _e7: SplitComplexNumber = other_97;
    let _e8: SplitComplexNumber = split_complex_number_split_complex_number_geometric_product(_e6, _e7);
    let _e10: SplitComplexNumber = self_137;
    let _e11: SplitComplexNumber = split_complex_number_reversal(_e10);
    let _e14: SplitComplexNumber = self_137;
    let _e15: SplitComplexNumber = other_97;
    let _e16: SplitComplexNumber = split_complex_number_split_complex_number_geometric_product(_e14, _e15);
    let _e18: SplitComplexNumber = self_137;
    let _e19: SplitComplexNumber = split_complex_number_reversal(_e18);
    let _e20: SplitComplexNumber = split_complex_number_split_complex_number_geometric_product(_e16, _e19);
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

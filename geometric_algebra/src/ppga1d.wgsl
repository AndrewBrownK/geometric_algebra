struct Scalar {
    g0_: f32,
}

struct DualNumber {
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

fn scalar_dual_number_add(self_28: Scalar, other_20: DualNumber) -> DualNumber {
    var self_29: Scalar;
    var other_21: DualNumber;

    self_29 = self_28;
    other_21 = other_20;
    let _e4: Scalar = self_29;
    let _e11: DualNumber = other_21;
    return DualNumber(((vec2<f32>(_e4.g0_) * vec2<f32>(1.0, 0.0)) + _e11.g0_));
}

fn scalar_dual_number_sub(self_30: Scalar, other_22: DualNumber) -> DualNumber {
    var self_31: Scalar;
    var other_23: DualNumber;

    self_31 = self_30;
    other_23 = other_22;
    let _e4: Scalar = self_31;
    let _e11: DualNumber = other_23;
    return DualNumber(((vec2<f32>(_e4.g0_) * vec2<f32>(1.0, 0.0)) - _e11.g0_));
}

fn scalar_dual_number_geometric_product(self_32: Scalar, other_24: DualNumber) -> DualNumber {
    var self_33: Scalar;
    var other_25: DualNumber;

    self_33 = self_32;
    other_25 = other_24;
    let _e4: Scalar = self_33;
    let _e7: DualNumber = other_25;
    return DualNumber((vec2<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_dual_number_regressive_product(self_34: Scalar, other_26: DualNumber) -> Scalar {
    var self_35: Scalar;
    var other_27: DualNumber;

    self_35 = self_34;
    other_27 = other_26;
    let _e4: Scalar = self_35;
    let _e6: DualNumber = other_27;
    return Scalar((_e4.g0_ * _e6.g0_.y));
}

fn scalar_dual_number_outer_product(self_36: Scalar, other_28: DualNumber) -> DualNumber {
    var self_37: Scalar;
    var other_29: DualNumber;

    self_37 = self_36;
    other_29 = other_28;
    let _e4: Scalar = self_37;
    let _e7: DualNumber = other_29;
    return DualNumber((vec2<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_dual_number_inner_product(self_38: Scalar, other_30: DualNumber) -> DualNumber {
    var self_39: Scalar;
    var other_31: DualNumber;

    self_39 = self_38;
    other_31 = other_30;
    let _e4: Scalar = self_39;
    let _e7: DualNumber = other_31;
    return DualNumber((vec2<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_dual_number_left_contraction(self_40: Scalar, other_32: DualNumber) -> DualNumber {
    var self_41: Scalar;
    var other_33: DualNumber;

    self_41 = self_40;
    other_33 = other_32;
    let _e4: Scalar = self_41;
    let _e7: DualNumber = other_33;
    return DualNumber((vec2<f32>(_e4.g0_) * _e7.g0_));
}

fn scalar_dual_number_right_contraction(self_42: Scalar, other_34: DualNumber) -> Scalar {
    var self_43: Scalar;
    var other_35: DualNumber;

    self_43 = self_42;
    other_35 = other_34;
    let _e4: Scalar = self_43;
    let _e6: DualNumber = other_35;
    return Scalar((_e4.g0_ * _e6.g0_.x));
}

fn scalar_dual_number_scalar_product(self_44: Scalar, other_36: DualNumber) -> Scalar {
    var self_45: Scalar;
    var other_37: DualNumber;

    self_45 = self_44;
    other_37 = other_36;
    let _e4: Scalar = self_45;
    let _e6: DualNumber = other_37;
    return Scalar((_e4.g0_ * _e6.g0_.x));
}

fn scalar_squared_magnitude(self_46: Scalar) -> Scalar {
    var self_47: Scalar;

    self_47 = self_46;
    let _e2: Scalar = self_47;
    let _e3: Scalar = self_47;
    let _e4: Scalar = scalar_reversal(_e3);
    let _e5: Scalar = scalar_scalar_scalar_product(_e2, _e4);
    return _e5;
}

fn scalar_magnitude(self_48: Scalar) -> Scalar {
    var self_49: Scalar;

    self_49 = self_48;
    let _e2: Scalar = self_49;
    let _e3: Scalar = scalar_squared_magnitude(_e2);
    return Scalar(sqrt(_e3.g0_));
}

fn scalar_scale(self_50: Scalar, other_38: f32) -> Scalar {
    var self_51: Scalar;
    var other_39: f32;

    self_51 = self_50;
    other_39 = other_38;
    let _e4: Scalar = self_51;
    let _e5: f32 = other_39;
    let _e7: Scalar = scalar_scalar_geometric_product(_e4, Scalar(_e5));
    return _e7;
}

fn scalar_signum(self_52: Scalar) -> Scalar {
    var self_53: Scalar;

    self_53 = self_52;
    let _e2: Scalar = self_53;
    let _e3: Scalar = self_53;
    let _e4: Scalar = scalar_magnitude(_e3);
    let _e9: Scalar = scalar_scalar_geometric_product(_e2, Scalar((1.0 / _e4.g0_)));
    return _e9;
}

fn scalar_inverse(self_54: Scalar) -> Scalar {
    var self_55: Scalar;

    self_55 = self_54;
    let _e2: Scalar = self_55;
    let _e3: Scalar = scalar_reversal(_e2);
    let _e4: Scalar = self_55;
    let _e5: Scalar = scalar_squared_magnitude(_e4);
    let _e10: Scalar = scalar_scalar_geometric_product(_e3, Scalar((1.0 / _e5.g0_)));
    return _e10;
}

fn dual_number_zero() -> DualNumber {
    return DualNumber(vec2<f32>(0.0));
}

fn dual_number_one() -> DualNumber {
    return DualNumber(vec2<f32>(1.0, 0.0));
}

fn dual_number_neg(self_56: DualNumber) -> DualNumber {
    var self_57: DualNumber;

    self_57 = self_56;
    let _e2: DualNumber = self_57;
    return DualNumber((_e2.g0_ * vec2<f32>(-(1.0))));
}

fn dual_number_automorphism(self_58: DualNumber) -> DualNumber {
    var self_59: DualNumber;

    self_59 = self_58;
    let _e2: DualNumber = self_59;
    return DualNumber(_e2.g0_);
}

fn dual_number_reversal(self_60: DualNumber) -> DualNumber {
    var self_61: DualNumber;

    self_61 = self_60;
    let _e2: DualNumber = self_61;
    return DualNumber((_e2.g0_ * vec2<f32>(1.0, -(1.0))));
}

fn dual_number_conjugation(self_62: DualNumber) -> DualNumber {
    var self_63: DualNumber;

    self_63 = self_62;
    let _e2: DualNumber = self_63;
    return DualNumber((_e2.g0_ * vec2<f32>(1.0, -(1.0))));
}

fn dual_number_dual(self_64: DualNumber) -> DualNumber {
    var self_65: DualNumber;

    self_65 = self_64;
    let _e2: DualNumber = self_65;
    return DualNumber(_e2.g0_.yx);
}

fn dual_number_scalar_into(self_66: DualNumber) -> Scalar {
    var self_67: DualNumber;

    self_67 = self_66;
    let _e2: DualNumber = self_67;
    return Scalar(_e2.g0_.x);
}

fn dual_number_scalar_add(self_68: DualNumber, other_40: Scalar) -> DualNumber {
    var self_69: DualNumber;
    var other_41: Scalar;

    self_69 = self_68;
    other_41 = other_40;
    let _e4: DualNumber = self_69;
    let _e6: Scalar = other_41;
    return DualNumber((_e4.g0_ + (vec2<f32>(_e6.g0_) * vec2<f32>(1.0, 0.0))));
}

fn dual_number_scalar_sub(self_70: DualNumber, other_42: Scalar) -> DualNumber {
    var self_71: DualNumber;
    var other_43: Scalar;

    self_71 = self_70;
    other_43 = other_42;
    let _e4: DualNumber = self_71;
    let _e6: Scalar = other_43;
    return DualNumber((_e4.g0_ - (vec2<f32>(_e6.g0_) * vec2<f32>(1.0, 0.0))));
}

fn dual_number_scalar_geometric_product(self_72: DualNumber, other_44: Scalar) -> DualNumber {
    var self_73: DualNumber;
    var other_45: Scalar;

    self_73 = self_72;
    other_45 = other_44;
    let _e4: DualNumber = self_73;
    let _e6: Scalar = other_45;
    return DualNumber((_e4.g0_ * vec2<f32>(_e6.g0_)));
}

fn dual_number_scalar_regressive_product(self_74: DualNumber, other_46: Scalar) -> Scalar {
    var self_75: DualNumber;
    var other_47: Scalar;

    self_75 = self_74;
    other_47 = other_46;
    let _e4: DualNumber = self_75;
    let _e7: Scalar = other_47;
    return Scalar((_e4.g0_.y * _e7.g0_));
}

fn dual_number_scalar_outer_product(self_76: DualNumber, other_48: Scalar) -> DualNumber {
    var self_77: DualNumber;
    var other_49: Scalar;

    self_77 = self_76;
    other_49 = other_48;
    let _e4: DualNumber = self_77;
    let _e6: Scalar = other_49;
    return DualNumber((_e4.g0_ * vec2<f32>(_e6.g0_)));
}

fn dual_number_scalar_inner_product(self_78: DualNumber, other_50: Scalar) -> DualNumber {
    var self_79: DualNumber;
    var other_51: Scalar;

    self_79 = self_78;
    other_51 = other_50;
    let _e4: DualNumber = self_79;
    let _e6: Scalar = other_51;
    return DualNumber((_e4.g0_ * vec2<f32>(_e6.g0_)));
}

fn dual_number_scalar_left_contraction(self_80: DualNumber, other_52: Scalar) -> Scalar {
    var self_81: DualNumber;
    var other_53: Scalar;

    self_81 = self_80;
    other_53 = other_52;
    let _e4: DualNumber = self_81;
    let _e7: Scalar = other_53;
    return Scalar((_e4.g0_.x * _e7.g0_));
}

fn dual_number_scalar_right_contraction(self_82: DualNumber, other_54: Scalar) -> DualNumber {
    var self_83: DualNumber;
    var other_55: Scalar;

    self_83 = self_82;
    other_55 = other_54;
    let _e4: DualNumber = self_83;
    let _e6: Scalar = other_55;
    return DualNumber((_e4.g0_ * vec2<f32>(_e6.g0_)));
}

fn dual_number_scalar_scalar_product(self_84: DualNumber, other_56: Scalar) -> Scalar {
    var self_85: DualNumber;
    var other_57: Scalar;

    self_85 = self_84;
    other_57 = other_56;
    let _e4: DualNumber = self_85;
    let _e7: Scalar = other_57;
    return Scalar((_e4.g0_.x * _e7.g0_));
}

fn dual_number_dual_number_add(self_86: DualNumber, other_58: DualNumber) -> DualNumber {
    var self_87: DualNumber;
    var other_59: DualNumber;

    self_87 = self_86;
    other_59 = other_58;
    let _e4: DualNumber = self_87;
    let _e6: DualNumber = other_59;
    return DualNumber((_e4.g0_ + _e6.g0_));
}

fn dual_number_dual_number_sub(self_88: DualNumber, other_60: DualNumber) -> DualNumber {
    var self_89: DualNumber;
    var other_61: DualNumber;

    self_89 = self_88;
    other_61 = other_60;
    let _e4: DualNumber = self_89;
    let _e6: DualNumber = other_61;
    return DualNumber((_e4.g0_ - _e6.g0_));
}

fn dual_number_dual_number_mul(self_90: DualNumber, other_62: DualNumber) -> DualNumber {
    var self_91: DualNumber;
    var other_63: DualNumber;

    self_91 = self_90;
    other_63 = other_62;
    let _e4: DualNumber = self_91;
    let _e6: DualNumber = other_63;
    return DualNumber((_e4.g0_ * _e6.g0_));
}

fn dual_number_dual_number_div(self_92: DualNumber, other_64: DualNumber) -> DualNumber {
    var self_93: DualNumber;
    var other_65: DualNumber;

    self_93 = self_92;
    other_65 = other_64;
    let _e4: DualNumber = self_93;
    let _e7: DualNumber = self_93;
    let _e15: DualNumber = other_65;
    let _e18: DualNumber = other_65;
    return DualNumber((((vec2<f32>(_e4.g0_.x, _e7.g0_.y) * vec2<f32>(1.0, 1.0)) / vec2<f32>(_e15.g0_.x, _e18.g0_.y)) * vec2<f32>(1.0, 1.0)));
}

fn dual_number_dual_number_geometric_product(self_94: DualNumber, other_66: DualNumber) -> DualNumber {
    var self_95: DualNumber;
    var other_67: DualNumber;

    self_95 = self_94;
    other_67 = other_66;
    let _e4: DualNumber = self_95;
    let _e8: DualNumber = other_67;
    let _e11: DualNumber = self_95;
    let _e13: DualNumber = other_67;
    return DualNumber(((vec2<f32>(_e4.g0_.x) * _e8.g0_) + ((_e11.g0_ * vec2<f32>(_e13.g0_.x)) * vec2<f32>(0.0, 1.0))));
}

fn dual_number_dual_number_regressive_product(self_96: DualNumber, other_68: DualNumber) -> DualNumber {
    var self_97: DualNumber;
    var other_69: DualNumber;

    self_97 = self_96;
    other_69 = other_68;
    let _e4: DualNumber = self_97;
    let _e8: DualNumber = other_69;
    let _e11: DualNumber = self_97;
    let _e15: DualNumber = other_69;
    return DualNumber(((vec2<f32>(_e4.g0_.y) * _e8.g0_) + ((vec2<f32>(_e11.g0_.x) * _e15.g0_.yx) * vec2<f32>(1.0, 0.0))));
}

fn dual_number_dual_number_outer_product(self_98: DualNumber, other_70: DualNumber) -> DualNumber {
    var self_99: DualNumber;
    var other_71: DualNumber;

    self_99 = self_98;
    other_71 = other_70;
    let _e4: DualNumber = self_99;
    let _e8: DualNumber = other_71;
    let _e11: DualNumber = self_99;
    let _e13: DualNumber = other_71;
    return DualNumber(((vec2<f32>(_e4.g0_.x) * _e8.g0_) + ((_e11.g0_ * vec2<f32>(_e13.g0_.x)) * vec2<f32>(0.0, 1.0))));
}

fn dual_number_dual_number_inner_product(self_100: DualNumber, other_72: DualNumber) -> DualNumber {
    var self_101: DualNumber;
    var other_73: DualNumber;

    self_101 = self_100;
    other_73 = other_72;
    let _e4: DualNumber = self_101;
    let _e8: DualNumber = other_73;
    let _e11: DualNumber = self_101;
    let _e13: DualNumber = other_73;
    return DualNumber(((vec2<f32>(_e4.g0_.x) * _e8.g0_) + ((_e11.g0_ * vec2<f32>(_e13.g0_.x)) * vec2<f32>(0.0, 1.0))));
}

fn dual_number_dual_number_left_contraction(self_102: DualNumber, other_74: DualNumber) -> DualNumber {
    var self_103: DualNumber;
    var other_75: DualNumber;

    self_103 = self_102;
    other_75 = other_74;
    let _e4: DualNumber = self_103;
    let _e8: DualNumber = other_75;
    return DualNumber((vec2<f32>(_e4.g0_.x) * _e8.g0_));
}

fn dual_number_dual_number_right_contraction(self_104: DualNumber, other_76: DualNumber) -> DualNumber {
    var self_105: DualNumber;
    var other_77: DualNumber;

    self_105 = self_104;
    other_77 = other_76;
    let _e4: DualNumber = self_105;
    let _e6: DualNumber = other_77;
    return DualNumber((_e4.g0_ * vec2<f32>(_e6.g0_.x)));
}

fn dual_number_dual_number_scalar_product(self_106: DualNumber, other_78: DualNumber) -> Scalar {
    var self_107: DualNumber;
    var other_79: DualNumber;

    self_107 = self_106;
    other_79 = other_78;
    let _e4: DualNumber = self_107;
    let _e7: DualNumber = other_79;
    return Scalar((_e4.g0_.x * _e7.g0_.x));
}

fn dual_number_squared_magnitude(self_108: DualNumber) -> Scalar {
    var self_109: DualNumber;

    self_109 = self_108;
    let _e2: DualNumber = self_109;
    let _e3: DualNumber = self_109;
    let _e4: DualNumber = dual_number_reversal(_e3);
    let _e5: Scalar = dual_number_dual_number_scalar_product(_e2, _e4);
    return _e5;
}

fn dual_number_magnitude(self_110: DualNumber) -> Scalar {
    var self_111: DualNumber;

    self_111 = self_110;
    let _e2: DualNumber = self_111;
    let _e3: Scalar = dual_number_squared_magnitude(_e2);
    return Scalar(sqrt(_e3.g0_));
}

fn dual_number_scale(self_112: DualNumber, other_80: f32) -> DualNumber {
    var self_113: DualNumber;
    var other_81: f32;

    self_113 = self_112;
    other_81 = other_80;
    let _e4: DualNumber = self_113;
    let _e5: f32 = other_81;
    let _e7: DualNumber = dual_number_scalar_geometric_product(_e4, Scalar(_e5));
    return _e7;
}

fn dual_number_signum(self_114: DualNumber) -> DualNumber {
    var self_115: DualNumber;

    self_115 = self_114;
    let _e2: DualNumber = self_115;
    let _e3: DualNumber = self_115;
    let _e4: Scalar = dual_number_magnitude(_e3);
    let _e9: DualNumber = dual_number_scalar_geometric_product(_e2, Scalar((1.0 / _e4.g0_)));
    return _e9;
}

fn dual_number_inverse(self_116: DualNumber) -> DualNumber {
    var self_117: DualNumber;

    self_117 = self_116;
    let _e2: DualNumber = self_117;
    let _e3: DualNumber = dual_number_reversal(_e2);
    let _e4: DualNumber = self_117;
    let _e5: Scalar = dual_number_squared_magnitude(_e4);
    let _e10: DualNumber = dual_number_scalar_geometric_product(_e3, Scalar((1.0 / _e5.g0_)));
    return _e10;
}

fn dual_number_powi(self_118: DualNumber, exponent: i32) -> DualNumber {
    var self_119: DualNumber;
    var exponent_1: i32;
    var local: DualNumber;
    var x: DualNumber;
    var y: DualNumber;
    var n: i32;

    self_119 = self_118;
    exponent_1 = exponent;
    let _e4: i32 = exponent_1;
    if (_e4 == 0) {
        {
            let _e7: DualNumber = dual_number_one();
            return _e7;
        }
    }
    let _e8: i32 = exponent_1;
    if (_e8 < 0) {
        let _e11: DualNumber = self_119;
        let _e12: DualNumber = dual_number_inverse(_e11);
        local = _e12;
    } else {
        let _e14: DualNumber = self_119;
        local = _e14;
    }
    let _e15: DualNumber = local;
    x = _e15;
    let _e17: DualNumber = dual_number_one();
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
                    let _e31: DualNumber = x;
                    let _e32: DualNumber = y;
                    let _e33: DualNumber = dual_number_dual_number_geometric_product(_e31, _e32);
                    y = _e33;
                }
            }
            let _e34: DualNumber = x;
            let _e35: DualNumber = x;
            let _e36: DualNumber = dual_number_dual_number_geometric_product(_e34, _e35);
            x = _e36;
            let _e37: i32 = n;
            n = (_e37 >> u32(1));
        }
    }
    let _e41: DualNumber = x;
    let _e42: DualNumber = y;
    let _e43: DualNumber = dual_number_dual_number_geometric_product(_e41, _e42);
    return _e43;
}

fn dual_number_dual_number_geometric_quotient(self_120: DualNumber, other_82: DualNumber) -> DualNumber {
    var self_121: DualNumber;
    var other_83: DualNumber;

    self_121 = self_120;
    other_83 = other_82;
    let _e4: DualNumber = self_121;
    let _e5: DualNumber = other_83;
    let _e6: DualNumber = dual_number_inverse(_e5);
    let _e7: DualNumber = dual_number_dual_number_geometric_product(_e4, _e6);
    return _e7;
}

fn dual_number_dual_number_transformation(self_122: DualNumber, other_84: DualNumber) -> DualNumber {
    var self_123: DualNumber;
    var other_85: DualNumber;

    self_123 = self_122;
    other_85 = other_84;
    let _e4: DualNumber = self_123;
    let _e5: DualNumber = other_85;
    let _e6: DualNumber = dual_number_dual_number_geometric_product(_e4, _e5);
    let _e7: DualNumber = self_123;
    let _e8: DualNumber = dual_number_reversal(_e7);
    let _e9: DualNumber = dual_number_dual_number_geometric_product(_e6, _e8);
    return _e9;
}

fn dual_number_scalar_geometric_quotient(self_124: DualNumber, other_86: Scalar) -> DualNumber {
    var self_125: DualNumber;
    var other_87: Scalar;

    self_125 = self_124;
    other_87 = other_86;
    let _e4: DualNumber = self_125;
    let _e5: Scalar = other_87;
    let _e6: Scalar = scalar_inverse(_e5);
    let _e7: DualNumber = dual_number_scalar_geometric_product(_e4, _e6);
    return _e7;
}

fn dual_number_scalar_transformation(self_126: DualNumber, other_88: Scalar) -> Scalar {
    var self_127: DualNumber;
    var other_89: Scalar;

    self_127 = self_126;
    other_89 = other_88;
    let _e4: DualNumber = self_127;
    let _e5: Scalar = other_89;
    let _e6: DualNumber = dual_number_scalar_geometric_product(_e4, _e5);
    let _e7: DualNumber = self_127;
    let _e8: DualNumber = dual_number_reversal(_e7);
    let _e9: DualNumber = dual_number_dual_number_geometric_product(_e6, _e8);
    let _e10: Scalar = dual_number_scalar_into(_e9);
    return _e10;
}

fn scalar_dual_number_geometric_quotient(self_128: Scalar, other_90: DualNumber) -> DualNumber {
    var self_129: Scalar;
    var other_91: DualNumber;

    self_129 = self_128;
    other_91 = other_90;
    let _e4: Scalar = self_129;
    let _e5: DualNumber = other_91;
    let _e6: DualNumber = dual_number_inverse(_e5);
    let _e7: DualNumber = scalar_dual_number_geometric_product(_e4, _e6);
    return _e7;
}

fn scalar_dual_number_transformation(self_130: Scalar, other_92: DualNumber) -> DualNumber {
    var self_131: Scalar;
    var other_93: DualNumber;

    self_131 = self_130;
    other_93 = other_92;
    let _e4: Scalar = self_131;
    let _e5: DualNumber = other_93;
    let _e6: DualNumber = scalar_dual_number_geometric_product(_e4, _e5);
    let _e7: Scalar = self_131;
    let _e8: Scalar = scalar_reversal(_e7);
    let _e9: DualNumber = dual_number_scalar_geometric_product(_e6, _e8);
    return _e9;
}

fn scalar_powi(self_132: Scalar, exponent_2: i32) -> Scalar {
    var self_133: Scalar;
    var exponent_3: i32;
    var local_1: Scalar;
    var x_1: Scalar;
    var y_1: Scalar;
    var n_1: i32;

    self_133 = self_132;
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
        let _e11: Scalar = self_133;
        let _e12: Scalar = scalar_inverse(_e11);
        local_1 = _e12;
    } else {
        let _e14: Scalar = self_133;
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

fn scalar_scalar_geometric_quotient(self_134: Scalar, other_94: Scalar) -> Scalar {
    var self_135: Scalar;
    var other_95: Scalar;

    self_135 = self_134;
    other_95 = other_94;
    let _e4: Scalar = self_135;
    let _e5: Scalar = other_95;
    let _e6: Scalar = scalar_inverse(_e5);
    let _e7: Scalar = scalar_scalar_geometric_product(_e4, _e6);
    return _e7;
}

fn scalar_scalar_transformation(self_136: Scalar, other_96: Scalar) -> Scalar {
    var self_137: Scalar;
    var other_97: Scalar;

    self_137 = self_136;
    other_97 = other_96;
    let _e4: Scalar = self_137;
    let _e5: Scalar = other_97;
    let _e6: Scalar = scalar_scalar_geometric_product(_e4, _e5);
    let _e7: Scalar = self_137;
    let _e8: Scalar = scalar_reversal(_e7);
    let _e9: Scalar = scalar_scalar_geometric_product(_e6, _e8);
    return _e9;
}


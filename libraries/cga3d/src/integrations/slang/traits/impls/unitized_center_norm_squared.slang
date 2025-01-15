implementing cga3d;
using traits;
// Fancy postfix trick
public float operator /(AntiCircleRotor lhs, UnitizedCenterNormSquaredPrefixOrPostfix rhs)  {
    return lhs.unitized_center_norm_squared();
    }
// Fancy prefix trick
public float operator /(UnitizedCenterNormSquaredPrefixOrPostfix lhs, AntiCircleRotor rhs) {
    return rhs.unitized_center_norm_squared();
}
public extension AntiCircleRotor {
    public float unitized_center_norm_squared() {
        return this.center_norm_squared().scalar / (this.round_weight_norm_squared().e12345);
    }
}
// Fancy postfix trick
public float operator /(AntiDipoleInversion lhs, UnitizedCenterNormSquaredPrefixOrPostfix rhs)  {
    return lhs.unitized_center_norm_squared();
    }
// Fancy prefix trick
public float operator /(UnitizedCenterNormSquaredPrefixOrPostfix lhs, AntiDipoleInversion rhs) {
    return rhs.unitized_center_norm_squared();
}
public extension AntiDipoleInversion {
    public float unitized_center_norm_squared() {
        return this.center_norm_squared().scalar / (this.round_weight_norm_squared().e12345);
    }
}
// Fancy postfix trick
public float operator /(Circle lhs, UnitizedCenterNormSquaredPrefixOrPostfix rhs)  {
    return lhs.unitized_center_norm_squared();
    }
// Fancy prefix trick
public float operator /(UnitizedCenterNormSquaredPrefixOrPostfix lhs, Circle rhs) {
    return rhs.unitized_center_norm_squared();
}
public extension Circle {
    public float unitized_center_norm_squared() {
        return this.center_norm_squared().scalar / (this.round_weight_norm_squared().e12345);
    }
}
// Fancy postfix trick
public float operator /(CircleRotor lhs, UnitizedCenterNormSquaredPrefixOrPostfix rhs)  {
    return lhs.unitized_center_norm_squared();
    }
// Fancy prefix trick
public float operator /(UnitizedCenterNormSquaredPrefixOrPostfix lhs, CircleRotor rhs) {
    return rhs.unitized_center_norm_squared();
}
public extension CircleRotor {
    public float unitized_center_norm_squared() {
        return this.center_norm_squared().scalar / (this.round_weight_norm_squared().e12345);
    }
}
// Fancy postfix trick
public float operator /(Dipole lhs, UnitizedCenterNormSquaredPrefixOrPostfix rhs)  {
    return lhs.unitized_center_norm_squared();
    }
// Fancy prefix trick
public float operator /(UnitizedCenterNormSquaredPrefixOrPostfix lhs, Dipole rhs) {
    return rhs.unitized_center_norm_squared();
}
public extension Dipole {
    public float unitized_center_norm_squared() {
        return this.center_norm_squared().scalar / (this.round_weight_norm_squared().e12345);
    }
}
// Fancy postfix trick
public float operator /(DipoleInversion lhs, UnitizedCenterNormSquaredPrefixOrPostfix rhs)  {
    return lhs.unitized_center_norm_squared();
    }
// Fancy prefix trick
public float operator /(UnitizedCenterNormSquaredPrefixOrPostfix lhs, DipoleInversion rhs) {
    return rhs.unitized_center_norm_squared();
}
public extension DipoleInversion {
    public float unitized_center_norm_squared() {
        return this.center_norm_squared().scalar / (this.round_weight_norm_squared().e12345);
    }
}
// Fancy postfix trick
public float operator /(MultiVector lhs, UnitizedCenterNormSquaredPrefixOrPostfix rhs)  {
    return lhs.unitized_center_norm_squared();
    }
// Fancy prefix trick
public float operator /(UnitizedCenterNormSquaredPrefixOrPostfix lhs, MultiVector rhs) {
    return rhs.unitized_center_norm_squared();
}
public extension MultiVector {
    public float unitized_center_norm_squared() {
        return this.center_norm_squared().scalar / (this.round_weight_norm_squared().e12345);
    }
}
// Fancy postfix trick
public float operator /(VersorEven lhs, UnitizedCenterNormSquaredPrefixOrPostfix rhs)  {
    return lhs.unitized_center_norm_squared();
    }
// Fancy prefix trick
public float operator /(UnitizedCenterNormSquaredPrefixOrPostfix lhs, VersorEven rhs) {
    return rhs.unitized_center_norm_squared();
}
public extension VersorEven {
    public float unitized_center_norm_squared() {
        return this.center_norm_squared().scalar / (this.round_weight_norm_squared().e12345);
    }
}
// Fancy postfix trick
public float operator /(VersorOdd lhs, UnitizedCenterNormSquaredPrefixOrPostfix rhs)  {
    return lhs.unitized_center_norm_squared();
    }
// Fancy prefix trick
public float operator /(UnitizedCenterNormSquaredPrefixOrPostfix lhs, VersorOdd rhs) {
    return rhs.unitized_center_norm_squared();
}
public extension VersorOdd {
    public float unitized_center_norm_squared() {
        return this.center_norm_squared().scalar / (this.round_weight_norm_squared().e12345);
    }
}

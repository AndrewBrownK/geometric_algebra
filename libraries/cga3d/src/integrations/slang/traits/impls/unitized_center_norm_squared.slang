implementing cga3d;
using traits;
public extension AntiCircleRotor {
    // Fancy postfix trick
    public float operator /(UnitizedCenterNormSquaredPrefixOrPostfix rhs)  {
        return this.unitized_center_norm_squared();
    }
}
public extension UnitizedCenterNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public float operator /(AntiCircleRotor rhs) {
        return rhs.unitized_center_norm_squared();
    }
}
public extension AntiCircleRotor {
    public float unitized_center_norm_squared() {
        return this.center_norm_squared().scalar / (this.round_weight_norm_squared().e12345);
    }
}
public extension AntiDipoleInversion {
    // Fancy postfix trick
    public float operator /(UnitizedCenterNormSquaredPrefixOrPostfix rhs)  {
        return this.unitized_center_norm_squared();
    }
}
public extension UnitizedCenterNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public float operator /(AntiDipoleInversion rhs) {
        return rhs.unitized_center_norm_squared();
    }
}
public extension AntiDipoleInversion {
    public float unitized_center_norm_squared() {
        return this.center_norm_squared().scalar / (this.round_weight_norm_squared().e12345);
    }
}
public extension Circle {
    // Fancy postfix trick
    public float operator /(UnitizedCenterNormSquaredPrefixOrPostfix rhs)  {
        return this.unitized_center_norm_squared();
    }
}
public extension UnitizedCenterNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public float operator /(Circle rhs) {
        return rhs.unitized_center_norm_squared();
    }
}
public extension Circle {
    public float unitized_center_norm_squared() {
        return this.center_norm_squared().scalar / (this.round_weight_norm_squared().e12345);
    }
}
public extension CircleRotor {
    // Fancy postfix trick
    public float operator /(UnitizedCenterNormSquaredPrefixOrPostfix rhs)  {
        return this.unitized_center_norm_squared();
    }
}
public extension UnitizedCenterNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public float operator /(CircleRotor rhs) {
        return rhs.unitized_center_norm_squared();
    }
}
public extension CircleRotor {
    public float unitized_center_norm_squared() {
        return this.center_norm_squared().scalar / (this.round_weight_norm_squared().e12345);
    }
}
public extension Dipole {
    // Fancy postfix trick
    public float operator /(UnitizedCenterNormSquaredPrefixOrPostfix rhs)  {
        return this.unitized_center_norm_squared();
    }
}
public extension UnitizedCenterNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public float operator /(Dipole rhs) {
        return rhs.unitized_center_norm_squared();
    }
}
public extension Dipole {
    public float unitized_center_norm_squared() {
        return this.center_norm_squared().scalar / (this.round_weight_norm_squared().e12345);
    }
}
public extension DipoleInversion {
    // Fancy postfix trick
    public float operator /(UnitizedCenterNormSquaredPrefixOrPostfix rhs)  {
        return this.unitized_center_norm_squared();
    }
}
public extension UnitizedCenterNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public float operator /(DipoleInversion rhs) {
        return rhs.unitized_center_norm_squared();
    }
}
public extension DipoleInversion {
    public float unitized_center_norm_squared() {
        return this.center_norm_squared().scalar / (this.round_weight_norm_squared().e12345);
    }
}
public extension MultiVector {
    // Fancy postfix trick
    public float operator /(UnitizedCenterNormSquaredPrefixOrPostfix rhs)  {
        return this.unitized_center_norm_squared();
    }
}
public extension UnitizedCenterNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public float operator /(MultiVector rhs) {
        return rhs.unitized_center_norm_squared();
    }
}
public extension MultiVector {
    public float unitized_center_norm_squared() {
        return this.center_norm_squared().scalar / (this.round_weight_norm_squared().e12345);
    }
}
public extension VersorEven {
    // Fancy postfix trick
    public float operator /(UnitizedCenterNormSquaredPrefixOrPostfix rhs)  {
        return this.unitized_center_norm_squared();
    }
}
public extension UnitizedCenterNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public float operator /(VersorEven rhs) {
        return rhs.unitized_center_norm_squared();
    }
}
public extension VersorEven {
    public float unitized_center_norm_squared() {
        return this.center_norm_squared().scalar / (this.round_weight_norm_squared().e12345);
    }
}
public extension VersorOdd {
    // Fancy postfix trick
    public float operator /(UnitizedCenterNormSquaredPrefixOrPostfix rhs)  {
        return this.unitized_center_norm_squared();
    }
}
public extension UnitizedCenterNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public float operator /(VersorOdd rhs) {
        return rhs.unitized_center_norm_squared();
    }
}
public extension VersorOdd {
    public float unitized_center_norm_squared() {
        return this.center_norm_squared().scalar / (this.round_weight_norm_squared().e12345);
    }
}

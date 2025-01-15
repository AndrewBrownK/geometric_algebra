implementing cga3d;
using traits;
// Fancy postfix trick
public float operator /(AntiCircleRotor lhs, UnitizedRadiusNormSquaredPrefixOrPostfix rhs)  {
    return lhs.unitized_radius_norm_squared();
    }
// Fancy prefix trick
public float operator /(UnitizedRadiusNormSquaredPrefixOrPostfix lhs, AntiCircleRotor rhs) {
    return rhs.unitized_radius_norm_squared();
}
public extension AntiCircleRotor {
    public float unitized_radius_norm_squared() {
        return this.radius_norm_squared().scalar / (this.round_weight_norm_squared().e12345);
    }
}
// Fancy postfix trick
public float operator /(AntiDipoleInversion lhs, UnitizedRadiusNormSquaredPrefixOrPostfix rhs)  {
    return lhs.unitized_radius_norm_squared();
    }
// Fancy prefix trick
public float operator /(UnitizedRadiusNormSquaredPrefixOrPostfix lhs, AntiDipoleInversion rhs) {
    return rhs.unitized_radius_norm_squared();
}
public extension AntiDipoleInversion {
    public float unitized_radius_norm_squared() {
        return this.radius_norm_squared().scalar / (this.round_weight_norm_squared().e12345);
    }
}
// Fancy postfix trick
public float operator /(Circle lhs, UnitizedRadiusNormSquaredPrefixOrPostfix rhs)  {
    return lhs.unitized_radius_norm_squared();
    }
// Fancy prefix trick
public float operator /(UnitizedRadiusNormSquaredPrefixOrPostfix lhs, Circle rhs) {
    return rhs.unitized_radius_norm_squared();
}
public extension Circle {
    public float unitized_radius_norm_squared() {
        return this.radius_norm_squared().scalar / (this.round_weight_norm_squared().e12345);
    }
}
// Fancy postfix trick
public float operator /(CircleRotor lhs, UnitizedRadiusNormSquaredPrefixOrPostfix rhs)  {
    return lhs.unitized_radius_norm_squared();
    }
// Fancy prefix trick
public float operator /(UnitizedRadiusNormSquaredPrefixOrPostfix lhs, CircleRotor rhs) {
    return rhs.unitized_radius_norm_squared();
}
public extension CircleRotor {
    public float unitized_radius_norm_squared() {
        return this.radius_norm_squared().scalar / (this.round_weight_norm_squared().e12345);
    }
}
// Fancy postfix trick
public float operator /(Dipole lhs, UnitizedRadiusNormSquaredPrefixOrPostfix rhs)  {
    return lhs.unitized_radius_norm_squared();
    }
// Fancy prefix trick
public float operator /(UnitizedRadiusNormSquaredPrefixOrPostfix lhs, Dipole rhs) {
    return rhs.unitized_radius_norm_squared();
}
public extension Dipole {
    public float unitized_radius_norm_squared() {
        return this.radius_norm_squared().scalar / (this.round_weight_norm_squared().e12345);
    }
}
// Fancy postfix trick
public float operator /(DipoleInversion lhs, UnitizedRadiusNormSquaredPrefixOrPostfix rhs)  {
    return lhs.unitized_radius_norm_squared();
    }
// Fancy prefix trick
public float operator /(UnitizedRadiusNormSquaredPrefixOrPostfix lhs, DipoleInversion rhs) {
    return rhs.unitized_radius_norm_squared();
}
public extension DipoleInversion {
    public float unitized_radius_norm_squared() {
        return this.radius_norm_squared().scalar / (this.round_weight_norm_squared().e12345);
    }
}
// Fancy postfix trick
public float operator /(MultiVector lhs, UnitizedRadiusNormSquaredPrefixOrPostfix rhs)  {
    return lhs.unitized_radius_norm_squared();
    }
// Fancy prefix trick
public float operator /(UnitizedRadiusNormSquaredPrefixOrPostfix lhs, MultiVector rhs) {
    return rhs.unitized_radius_norm_squared();
}
public extension MultiVector {
    public float unitized_radius_norm_squared() {
        return this.radius_norm_squared().scalar / (this.round_weight_norm_squared().e12345);
    }
}
// Fancy postfix trick
public float operator /(RoundPoint lhs, UnitizedRadiusNormSquaredPrefixOrPostfix rhs)  {
    return lhs.unitized_radius_norm_squared();
    }
// Fancy prefix trick
public float operator /(UnitizedRadiusNormSquaredPrefixOrPostfix lhs, RoundPoint rhs) {
    return rhs.unitized_radius_norm_squared();
}
public extension RoundPoint {
    public float unitized_radius_norm_squared() {
        return this.radius_norm_squared().scalar / (this.round_weight_norm_squared().e12345);
    }
}
// Fancy postfix trick
public float operator /(Sphere lhs, UnitizedRadiusNormSquaredPrefixOrPostfix rhs)  {
    return lhs.unitized_radius_norm_squared();
    }
// Fancy prefix trick
public float operator /(UnitizedRadiusNormSquaredPrefixOrPostfix lhs, Sphere rhs) {
    return rhs.unitized_radius_norm_squared();
}
public extension Sphere {
    public float unitized_radius_norm_squared() {
        return this.radius_norm_squared().scalar / (this.round_weight_norm_squared().e12345);
    }
}
// Fancy postfix trick
public float operator /(VersorEven lhs, UnitizedRadiusNormSquaredPrefixOrPostfix rhs)  {
    return lhs.unitized_radius_norm_squared();
    }
// Fancy prefix trick
public float operator /(UnitizedRadiusNormSquaredPrefixOrPostfix lhs, VersorEven rhs) {
    return rhs.unitized_radius_norm_squared();
}
public extension VersorEven {
    public float unitized_radius_norm_squared() {
        return this.radius_norm_squared().scalar / (this.round_weight_norm_squared().e12345);
    }
}
// Fancy postfix trick
public float operator /(VersorOdd lhs, UnitizedRadiusNormSquaredPrefixOrPostfix rhs)  {
    return lhs.unitized_radius_norm_squared();
    }
// Fancy prefix trick
public float operator /(UnitizedRadiusNormSquaredPrefixOrPostfix lhs, VersorOdd rhs) {
    return rhs.unitized_radius_norm_squared();
}
public extension VersorOdd {
    public float unitized_radius_norm_squared() {
        return this.radius_norm_squared().scalar / (this.round_weight_norm_squared().e12345);
    }
}

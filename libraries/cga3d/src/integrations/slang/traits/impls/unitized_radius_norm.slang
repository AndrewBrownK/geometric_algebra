implementing cga3d;
using traits;
// Fancy postfix trick
public float operator /(AntiCircleRotor lhs, UnitizedRadiusNormPrefixOrPostfix rhs)  {
    return lhs.unitized_radius_norm();
    }
// Fancy prefix trick
public float operator /(UnitizedRadiusNormPrefixOrPostfix lhs, AntiCircleRotor rhs) {
    return rhs.unitized_radius_norm();
}
public extension AntiCircleRotor {
    public float unitized_radius_norm() {
        return pow(this.unitized_radius_norm_squared(), 0.5);
    }
}
// Fancy postfix trick
public float operator /(AntiDipoleInversion lhs, UnitizedRadiusNormPrefixOrPostfix rhs)  {
    return lhs.unitized_radius_norm();
    }
// Fancy prefix trick
public float operator /(UnitizedRadiusNormPrefixOrPostfix lhs, AntiDipoleInversion rhs) {
    return rhs.unitized_radius_norm();
}
public extension AntiDipoleInversion {
    public float unitized_radius_norm() {
        return pow(this.unitized_radius_norm_squared(), 0.5);
    }
}
// Fancy postfix trick
public float operator /(Circle lhs, UnitizedRadiusNormPrefixOrPostfix rhs)  {
    return lhs.unitized_radius_norm();
    }
// Fancy prefix trick
public float operator /(UnitizedRadiusNormPrefixOrPostfix lhs, Circle rhs) {
    return rhs.unitized_radius_norm();
}
public extension Circle {
    public float unitized_radius_norm() {
        return pow(this.unitized_radius_norm_squared(), 0.5);
    }
}
// Fancy postfix trick
public float operator /(CircleRotor lhs, UnitizedRadiusNormPrefixOrPostfix rhs)  {
    return lhs.unitized_radius_norm();
    }
// Fancy prefix trick
public float operator /(UnitizedRadiusNormPrefixOrPostfix lhs, CircleRotor rhs) {
    return rhs.unitized_radius_norm();
}
public extension CircleRotor {
    public float unitized_radius_norm() {
        return pow(this.unitized_radius_norm_squared(), 0.5);
    }
}
// Fancy postfix trick
public float operator /(Dipole lhs, UnitizedRadiusNormPrefixOrPostfix rhs)  {
    return lhs.unitized_radius_norm();
    }
// Fancy prefix trick
public float operator /(UnitizedRadiusNormPrefixOrPostfix lhs, Dipole rhs) {
    return rhs.unitized_radius_norm();
}
public extension Dipole {
    public float unitized_radius_norm() {
        return pow(this.unitized_radius_norm_squared(), 0.5);
    }
}
// Fancy postfix trick
public float operator /(DipoleInversion lhs, UnitizedRadiusNormPrefixOrPostfix rhs)  {
    return lhs.unitized_radius_norm();
    }
// Fancy prefix trick
public float operator /(UnitizedRadiusNormPrefixOrPostfix lhs, DipoleInversion rhs) {
    return rhs.unitized_radius_norm();
}
public extension DipoleInversion {
    public float unitized_radius_norm() {
        return pow(this.unitized_radius_norm_squared(), 0.5);
    }
}
// Fancy postfix trick
public float operator /(MultiVector lhs, UnitizedRadiusNormPrefixOrPostfix rhs)  {
    return lhs.unitized_radius_norm();
    }
// Fancy prefix trick
public float operator /(UnitizedRadiusNormPrefixOrPostfix lhs, MultiVector rhs) {
    return rhs.unitized_radius_norm();
}
public extension MultiVector {
    public float unitized_radius_norm() {
        return pow(this.unitized_radius_norm_squared(), 0.5);
    }
}
// Fancy postfix trick
public float operator /(RoundPoint lhs, UnitizedRadiusNormPrefixOrPostfix rhs)  {
    return lhs.unitized_radius_norm();
    }
// Fancy prefix trick
public float operator /(UnitizedRadiusNormPrefixOrPostfix lhs, RoundPoint rhs) {
    return rhs.unitized_radius_norm();
}
public extension RoundPoint {
    public float unitized_radius_norm() {
        return pow(this.unitized_radius_norm_squared(), 0.5);
    }
}
// Fancy postfix trick
public float operator /(Sphere lhs, UnitizedRadiusNormPrefixOrPostfix rhs)  {
    return lhs.unitized_radius_norm();
    }
// Fancy prefix trick
public float operator /(UnitizedRadiusNormPrefixOrPostfix lhs, Sphere rhs) {
    return rhs.unitized_radius_norm();
}
public extension Sphere {
    public float unitized_radius_norm() {
        return pow(this.unitized_radius_norm_squared(), 0.5);
    }
}
// Fancy postfix trick
public float operator /(VersorEven lhs, UnitizedRadiusNormPrefixOrPostfix rhs)  {
    return lhs.unitized_radius_norm();
    }
// Fancy prefix trick
public float operator /(UnitizedRadiusNormPrefixOrPostfix lhs, VersorEven rhs) {
    return rhs.unitized_radius_norm();
}
public extension VersorEven {
    public float unitized_radius_norm() {
        return pow(this.unitized_radius_norm_squared(), 0.5);
    }
}
// Fancy postfix trick
public float operator /(VersorOdd lhs, UnitizedRadiusNormPrefixOrPostfix rhs)  {
    return lhs.unitized_radius_norm();
    }
// Fancy prefix trick
public float operator /(UnitizedRadiusNormPrefixOrPostfix lhs, VersorOdd rhs) {
    return rhs.unitized_radius_norm();
}
public extension VersorOdd {
    public float unitized_radius_norm() {
        return pow(this.unitized_radius_norm_squared(), 0.5);
    }
}

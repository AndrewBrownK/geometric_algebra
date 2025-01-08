implementing cga3d;
using traits;
public extension AntiCircleRotor {
    // Fancy postfix trick
    public float operator /(UnitizedRadiusNormPrefixOrPostfix rhs)  {
        return this.unitized_radius_norm();
    }
}
public extension UnitizedRadiusNormPrefixOrPostfix {
    // Fancy prefix trick
    public float operator /(AntiCircleRotor rhs) {
        return rhs.unitized_radius_norm();
    }
}
public extension AntiCircleRotor {
    public float unitized_radius_norm() {
        return pow(this.unitized_radius_norm_squared(), 0.5);
    }
}
public extension AntiDipoleInversion {
    // Fancy postfix trick
    public float operator /(UnitizedRadiusNormPrefixOrPostfix rhs)  {
        return this.unitized_radius_norm();
    }
}
public extension UnitizedRadiusNormPrefixOrPostfix {
    // Fancy prefix trick
    public float operator /(AntiDipoleInversion rhs) {
        return rhs.unitized_radius_norm();
    }
}
public extension AntiDipoleInversion {
    public float unitized_radius_norm() {
        return pow(this.unitized_radius_norm_squared(), 0.5);
    }
}
public extension Circle {
    // Fancy postfix trick
    public float operator /(UnitizedRadiusNormPrefixOrPostfix rhs)  {
        return this.unitized_radius_norm();
    }
}
public extension UnitizedRadiusNormPrefixOrPostfix {
    // Fancy prefix trick
    public float operator /(Circle rhs) {
        return rhs.unitized_radius_norm();
    }
}
public extension Circle {
    public float unitized_radius_norm() {
        return pow(this.unitized_radius_norm_squared(), 0.5);
    }
}
public extension CircleRotor {
    // Fancy postfix trick
    public float operator /(UnitizedRadiusNormPrefixOrPostfix rhs)  {
        return this.unitized_radius_norm();
    }
}
public extension UnitizedRadiusNormPrefixOrPostfix {
    // Fancy prefix trick
    public float operator /(CircleRotor rhs) {
        return rhs.unitized_radius_norm();
    }
}
public extension CircleRotor {
    public float unitized_radius_norm() {
        return pow(this.unitized_radius_norm_squared(), 0.5);
    }
}
public extension Dipole {
    // Fancy postfix trick
    public float operator /(UnitizedRadiusNormPrefixOrPostfix rhs)  {
        return this.unitized_radius_norm();
    }
}
public extension UnitizedRadiusNormPrefixOrPostfix {
    // Fancy prefix trick
    public float operator /(Dipole rhs) {
        return rhs.unitized_radius_norm();
    }
}
public extension Dipole {
    public float unitized_radius_norm() {
        return pow(this.unitized_radius_norm_squared(), 0.5);
    }
}
public extension DipoleInversion {
    // Fancy postfix trick
    public float operator /(UnitizedRadiusNormPrefixOrPostfix rhs)  {
        return this.unitized_radius_norm();
    }
}
public extension UnitizedRadiusNormPrefixOrPostfix {
    // Fancy prefix trick
    public float operator /(DipoleInversion rhs) {
        return rhs.unitized_radius_norm();
    }
}
public extension DipoleInversion {
    public float unitized_radius_norm() {
        return pow(this.unitized_radius_norm_squared(), 0.5);
    }
}
public extension MultiVector {
    // Fancy postfix trick
    public float operator /(UnitizedRadiusNormPrefixOrPostfix rhs)  {
        return this.unitized_radius_norm();
    }
}
public extension UnitizedRadiusNormPrefixOrPostfix {
    // Fancy prefix trick
    public float operator /(MultiVector rhs) {
        return rhs.unitized_radius_norm();
    }
}
public extension MultiVector {
    public float unitized_radius_norm() {
        return pow(this.unitized_radius_norm_squared(), 0.5);
    }
}
public extension RoundPoint {
    // Fancy postfix trick
    public float operator /(UnitizedRadiusNormPrefixOrPostfix rhs)  {
        return this.unitized_radius_norm();
    }
}
public extension UnitizedRadiusNormPrefixOrPostfix {
    // Fancy prefix trick
    public float operator /(RoundPoint rhs) {
        return rhs.unitized_radius_norm();
    }
}
public extension RoundPoint {
    public float unitized_radius_norm() {
        return pow(this.unitized_radius_norm_squared(), 0.5);
    }
}
public extension Sphere {
    // Fancy postfix trick
    public float operator /(UnitizedRadiusNormPrefixOrPostfix rhs)  {
        return this.unitized_radius_norm();
    }
}
public extension UnitizedRadiusNormPrefixOrPostfix {
    // Fancy prefix trick
    public float operator /(Sphere rhs) {
        return rhs.unitized_radius_norm();
    }
}
public extension Sphere {
    public float unitized_radius_norm() {
        return pow(this.unitized_radius_norm_squared(), 0.5);
    }
}
public extension VersorEven {
    // Fancy postfix trick
    public float operator /(UnitizedRadiusNormPrefixOrPostfix rhs)  {
        return this.unitized_radius_norm();
    }
}
public extension UnitizedRadiusNormPrefixOrPostfix {
    // Fancy prefix trick
    public float operator /(VersorEven rhs) {
        return rhs.unitized_radius_norm();
    }
}
public extension VersorEven {
    public float unitized_radius_norm() {
        return pow(this.unitized_radius_norm_squared(), 0.5);
    }
}
public extension VersorOdd {
    // Fancy postfix trick
    public float operator /(UnitizedRadiusNormPrefixOrPostfix rhs)  {
        return this.unitized_radius_norm();
    }
}
public extension UnitizedRadiusNormPrefixOrPostfix {
    // Fancy prefix trick
    public float operator /(VersorOdd rhs) {
        return rhs.unitized_radius_norm();
    }
}
public extension VersorOdd {
    public float unitized_radius_norm() {
        return pow(this.unitized_radius_norm_squared(), 0.5);
    }
}

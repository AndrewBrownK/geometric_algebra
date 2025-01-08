implementing cga3d;
using traits;
public extension AntiCircleRotor {
    // Fancy postfix trick
    public float operator /(UnitizedFlatNormPrefixOrPostfix rhs)  {
        return this.unitized_flat_norm();
    }
}
public extension UnitizedFlatNormPrefixOrPostfix {
    // Fancy prefix trick
    public float operator /(AntiCircleRotor rhs) {
        return rhs.unitized_flat_norm();
    }
}
public extension AntiCircleRotor {
    public float unitized_flat_norm() {
        return pow(this.unitized_flat_norm_squared(), 0.5);
    }
}
public extension AntiDipoleInversion {
    // Fancy postfix trick
    public float operator /(UnitizedFlatNormPrefixOrPostfix rhs)  {
        return this.unitized_flat_norm();
    }
}
public extension UnitizedFlatNormPrefixOrPostfix {
    // Fancy prefix trick
    public float operator /(AntiDipoleInversion rhs) {
        return rhs.unitized_flat_norm();
    }
}
public extension AntiDipoleInversion {
    public float unitized_flat_norm() {
        return pow(this.unitized_flat_norm_squared(), 0.5);
    }
}
public extension Circle {
    // Fancy postfix trick
    public float operator /(UnitizedFlatNormPrefixOrPostfix rhs)  {
        return this.unitized_flat_norm();
    }
}
public extension UnitizedFlatNormPrefixOrPostfix {
    // Fancy prefix trick
    public float operator /(Circle rhs) {
        return rhs.unitized_flat_norm();
    }
}
public extension Circle {
    public float unitized_flat_norm() {
        return pow(this.unitized_flat_norm_squared(), 0.5);
    }
}
public extension CircleRotor {
    // Fancy postfix trick
    public float operator /(UnitizedFlatNormPrefixOrPostfix rhs)  {
        return this.unitized_flat_norm();
    }
}
public extension UnitizedFlatNormPrefixOrPostfix {
    // Fancy prefix trick
    public float operator /(CircleRotor rhs) {
        return rhs.unitized_flat_norm();
    }
}
public extension CircleRotor {
    public float unitized_flat_norm() {
        return pow(this.unitized_flat_norm_squared(), 0.5);
    }
}
public extension Dipole {
    // Fancy postfix trick
    public float operator /(UnitizedFlatNormPrefixOrPostfix rhs)  {
        return this.unitized_flat_norm();
    }
}
public extension UnitizedFlatNormPrefixOrPostfix {
    // Fancy prefix trick
    public float operator /(Dipole rhs) {
        return rhs.unitized_flat_norm();
    }
}
public extension Dipole {
    public float unitized_flat_norm() {
        return pow(this.unitized_flat_norm_squared(), 0.5);
    }
}
public extension DipoleInversion {
    // Fancy postfix trick
    public float operator /(UnitizedFlatNormPrefixOrPostfix rhs)  {
        return this.unitized_flat_norm();
    }
}
public extension UnitizedFlatNormPrefixOrPostfix {
    // Fancy prefix trick
    public float operator /(DipoleInversion rhs) {
        return rhs.unitized_flat_norm();
    }
}
public extension DipoleInversion {
    public float unitized_flat_norm() {
        return pow(this.unitized_flat_norm_squared(), 0.5);
    }
}
public extension DualNum {
    // Fancy postfix trick
    public float operator /(UnitizedFlatNormPrefixOrPostfix rhs)  {
        return this.unitized_flat_norm();
    }
}
public extension UnitizedFlatNormPrefixOrPostfix {
    // Fancy prefix trick
    public float operator /(DualNum rhs) {
        return rhs.unitized_flat_norm();
    }
}
public extension DualNum {
    public float unitized_flat_norm() {
        return pow(this.unitized_flat_norm_squared(), 0.5);
    }
}
public extension FlatPoint {
    // Fancy postfix trick
    public float operator /(UnitizedFlatNormPrefixOrPostfix rhs)  {
        return this.unitized_flat_norm();
    }
}
public extension UnitizedFlatNormPrefixOrPostfix {
    // Fancy prefix trick
    public float operator /(FlatPoint rhs) {
        return rhs.unitized_flat_norm();
    }
}
public extension FlatPoint {
    public float unitized_flat_norm() {
        return pow(this.unitized_flat_norm_squared(), 0.5);
    }
}
public extension Flector {
    // Fancy postfix trick
    public float operator /(UnitizedFlatNormPrefixOrPostfix rhs)  {
        return this.unitized_flat_norm();
    }
}
public extension UnitizedFlatNormPrefixOrPostfix {
    // Fancy prefix trick
    public float operator /(Flector rhs) {
        return rhs.unitized_flat_norm();
    }
}
public extension Flector {
    public float unitized_flat_norm() {
        return pow(this.unitized_flat_norm_squared(), 0.5);
    }
}
public extension Line {
    // Fancy postfix trick
    public float operator /(UnitizedFlatNormPrefixOrPostfix rhs)  {
        return this.unitized_flat_norm();
    }
}
public extension UnitizedFlatNormPrefixOrPostfix {
    // Fancy prefix trick
    public float operator /(Line rhs) {
        return rhs.unitized_flat_norm();
    }
}
public extension Line {
    public float unitized_flat_norm() {
        return pow(this.unitized_flat_norm_squared(), 0.5);
    }
}
public extension Motor {
    // Fancy postfix trick
    public float operator /(UnitizedFlatNormPrefixOrPostfix rhs)  {
        return this.unitized_flat_norm();
    }
}
public extension UnitizedFlatNormPrefixOrPostfix {
    // Fancy prefix trick
    public float operator /(Motor rhs) {
        return rhs.unitized_flat_norm();
    }
}
public extension Motor {
    public float unitized_flat_norm() {
        return pow(this.unitized_flat_norm_squared(), 0.5);
    }
}
public extension MultiVector {
    // Fancy postfix trick
    public float operator /(UnitizedFlatNormPrefixOrPostfix rhs)  {
        return this.unitized_flat_norm();
    }
}
public extension UnitizedFlatNormPrefixOrPostfix {
    // Fancy prefix trick
    public float operator /(MultiVector rhs) {
        return rhs.unitized_flat_norm();
    }
}
public extension MultiVector {
    public float unitized_flat_norm() {
        return pow(this.unitized_flat_norm_squared(), 0.5);
    }
}
public extension Plane {
    // Fancy postfix trick
    public float operator /(UnitizedFlatNormPrefixOrPostfix rhs)  {
        return this.unitized_flat_norm();
    }
}
public extension UnitizedFlatNormPrefixOrPostfix {
    // Fancy prefix trick
    public float operator /(Plane rhs) {
        return rhs.unitized_flat_norm();
    }
}
public extension Plane {
    public float unitized_flat_norm() {
        return pow(this.unitized_flat_norm_squared(), 0.5);
    }
}
public extension Sphere {
    // Fancy postfix trick
    public float operator /(UnitizedFlatNormPrefixOrPostfix rhs)  {
        return this.unitized_flat_norm();
    }
}
public extension UnitizedFlatNormPrefixOrPostfix {
    // Fancy prefix trick
    public float operator /(Sphere rhs) {
        return rhs.unitized_flat_norm();
    }
}
public extension Sphere {
    public float unitized_flat_norm() {
        return pow(this.unitized_flat_norm_squared(), 0.5);
    }
}
public extension VersorEven {
    // Fancy postfix trick
    public float operator /(UnitizedFlatNormPrefixOrPostfix rhs)  {
        return this.unitized_flat_norm();
    }
}
public extension UnitizedFlatNormPrefixOrPostfix {
    // Fancy prefix trick
    public float operator /(VersorEven rhs) {
        return rhs.unitized_flat_norm();
    }
}
public extension VersorEven {
    public float unitized_flat_norm() {
        return pow(this.unitized_flat_norm_squared(), 0.5);
    }
}
public extension VersorOdd {
    // Fancy postfix trick
    public float operator /(UnitizedFlatNormPrefixOrPostfix rhs)  {
        return this.unitized_flat_norm();
    }
}
public extension UnitizedFlatNormPrefixOrPostfix {
    // Fancy prefix trick
    public float operator /(VersorOdd rhs) {
        return rhs.unitized_flat_norm();
    }
}
public extension VersorOdd {
    public float unitized_flat_norm() {
        return pow(this.unitized_flat_norm_squared(), 0.5);
    }
}

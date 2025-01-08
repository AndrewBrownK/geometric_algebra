implementing cga3d;
using traits;
public extension AntiCircleRotor {
    // Fancy postfix trick
    public float operator /(UnitizedFlatNormSquaredPrefixOrPostfix rhs)  {
        return this.unitized_flat_norm_squared();
    }
}
public extension UnitizedFlatNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public float operator /(AntiCircleRotor rhs) {
        return rhs.unitized_flat_norm_squared();
    }
}
public extension AntiCircleRotor {
    public float unitized_flat_norm_squared() {
        return this.flat_bulk_norm_squared().scalar / (this.flat_weight_norm_squared().e12345);
    }
}
public extension AntiDipoleInversion {
    // Fancy postfix trick
    public float operator /(UnitizedFlatNormSquaredPrefixOrPostfix rhs)  {
        return this.unitized_flat_norm_squared();
    }
}
public extension UnitizedFlatNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public float operator /(AntiDipoleInversion rhs) {
        return rhs.unitized_flat_norm_squared();
    }
}
public extension AntiDipoleInversion {
    public float unitized_flat_norm_squared() {
        return this.flat_bulk_norm_squared().scalar / (this.flat_weight_norm_squared().e12345);
    }
}
public extension Circle {
    // Fancy postfix trick
    public float operator /(UnitizedFlatNormSquaredPrefixOrPostfix rhs)  {
        return this.unitized_flat_norm_squared();
    }
}
public extension UnitizedFlatNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public float operator /(Circle rhs) {
        return rhs.unitized_flat_norm_squared();
    }
}
public extension Circle {
    public float unitized_flat_norm_squared() {
        return this.flat_bulk_norm_squared().scalar / (this.flat_weight_norm_squared().e12345);
    }
}
public extension CircleRotor {
    // Fancy postfix trick
    public float operator /(UnitizedFlatNormSquaredPrefixOrPostfix rhs)  {
        return this.unitized_flat_norm_squared();
    }
}
public extension UnitizedFlatNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public float operator /(CircleRotor rhs) {
        return rhs.unitized_flat_norm_squared();
    }
}
public extension CircleRotor {
    public float unitized_flat_norm_squared() {
        return this.flat_bulk_norm_squared().scalar / (this.flat_weight_norm_squared().e12345);
    }
}
public extension Dipole {
    // Fancy postfix trick
    public float operator /(UnitizedFlatNormSquaredPrefixOrPostfix rhs)  {
        return this.unitized_flat_norm_squared();
    }
}
public extension UnitizedFlatNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public float operator /(Dipole rhs) {
        return rhs.unitized_flat_norm_squared();
    }
}
public extension Dipole {
    public float unitized_flat_norm_squared() {
        return this.flat_bulk_norm_squared().scalar / (this.flat_weight_norm_squared().e12345);
    }
}
public extension DipoleInversion {
    // Fancy postfix trick
    public float operator /(UnitizedFlatNormSquaredPrefixOrPostfix rhs)  {
        return this.unitized_flat_norm_squared();
    }
}
public extension UnitizedFlatNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public float operator /(DipoleInversion rhs) {
        return rhs.unitized_flat_norm_squared();
    }
}
public extension DipoleInversion {
    public float unitized_flat_norm_squared() {
        return this.flat_bulk_norm_squared().scalar / (this.flat_weight_norm_squared().e12345);
    }
}
public extension DualNum {
    // Fancy postfix trick
    public float operator /(UnitizedFlatNormSquaredPrefixOrPostfix rhs)  {
        return this.unitized_flat_norm_squared();
    }
}
public extension UnitizedFlatNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public float operator /(DualNum rhs) {
        return rhs.unitized_flat_norm_squared();
    }
}
public extension DualNum {
    public float unitized_flat_norm_squared() {
        return this.flat_bulk_norm_squared().scalar / (this.flat_weight_norm_squared().e12345);
    }
}
public extension FlatPoint {
    // Fancy postfix trick
    public float operator /(UnitizedFlatNormSquaredPrefixOrPostfix rhs)  {
        return this.unitized_flat_norm_squared();
    }
}
public extension UnitizedFlatNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public float operator /(FlatPoint rhs) {
        return rhs.unitized_flat_norm_squared();
    }
}
public extension FlatPoint {
    public float unitized_flat_norm_squared() {
        return this.flat_bulk_norm_squared().scalar / (this.flat_weight_norm_squared().e12345);
    }
}
public extension Flector {
    // Fancy postfix trick
    public float operator /(UnitizedFlatNormSquaredPrefixOrPostfix rhs)  {
        return this.unitized_flat_norm_squared();
    }
}
public extension UnitizedFlatNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public float operator /(Flector rhs) {
        return rhs.unitized_flat_norm_squared();
    }
}
public extension Flector {
    public float unitized_flat_norm_squared() {
        return this.flat_bulk_norm_squared().scalar / (this.flat_weight_norm_squared().e12345);
    }
}
public extension Line {
    // Fancy postfix trick
    public float operator /(UnitizedFlatNormSquaredPrefixOrPostfix rhs)  {
        return this.unitized_flat_norm_squared();
    }
}
public extension UnitizedFlatNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public float operator /(Line rhs) {
        return rhs.unitized_flat_norm_squared();
    }
}
public extension Line {
    public float unitized_flat_norm_squared() {
        return this.flat_bulk_norm_squared().scalar / (this.flat_weight_norm_squared().e12345);
    }
}
public extension Motor {
    // Fancy postfix trick
    public float operator /(UnitizedFlatNormSquaredPrefixOrPostfix rhs)  {
        return this.unitized_flat_norm_squared();
    }
}
public extension UnitizedFlatNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public float operator /(Motor rhs) {
        return rhs.unitized_flat_norm_squared();
    }
}
public extension Motor {
    public float unitized_flat_norm_squared() {
        return this.flat_bulk_norm_squared().scalar / (this.flat_weight_norm_squared().e12345);
    }
}
public extension MultiVector {
    // Fancy postfix trick
    public float operator /(UnitizedFlatNormSquaredPrefixOrPostfix rhs)  {
        return this.unitized_flat_norm_squared();
    }
}
public extension UnitizedFlatNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public float operator /(MultiVector rhs) {
        return rhs.unitized_flat_norm_squared();
    }
}
public extension MultiVector {
    public float unitized_flat_norm_squared() {
        return this.flat_bulk_norm_squared().scalar / (this.flat_weight_norm_squared().e12345);
    }
}
public extension Plane {
    // Fancy postfix trick
    public float operator /(UnitizedFlatNormSquaredPrefixOrPostfix rhs)  {
        return this.unitized_flat_norm_squared();
    }
}
public extension UnitizedFlatNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public float operator /(Plane rhs) {
        return rhs.unitized_flat_norm_squared();
    }
}
public extension Plane {
    public float unitized_flat_norm_squared() {
        return this.flat_bulk_norm_squared().scalar / (this.flat_weight_norm_squared().e12345);
    }
}
public extension Sphere {
    // Fancy postfix trick
    public float operator /(UnitizedFlatNormSquaredPrefixOrPostfix rhs)  {
        return this.unitized_flat_norm_squared();
    }
}
public extension UnitizedFlatNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public float operator /(Sphere rhs) {
        return rhs.unitized_flat_norm_squared();
    }
}
public extension Sphere {
    public float unitized_flat_norm_squared() {
        return this.flat_bulk_norm_squared().scalar / (this.flat_weight_norm_squared().e12345);
    }
}
public extension VersorEven {
    // Fancy postfix trick
    public float operator /(UnitizedFlatNormSquaredPrefixOrPostfix rhs)  {
        return this.unitized_flat_norm_squared();
    }
}
public extension UnitizedFlatNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public float operator /(VersorEven rhs) {
        return rhs.unitized_flat_norm_squared();
    }
}
public extension VersorEven {
    public float unitized_flat_norm_squared() {
        return this.flat_bulk_norm_squared().scalar / (this.flat_weight_norm_squared().e12345);
    }
}
public extension VersorOdd {
    // Fancy postfix trick
    public float operator /(UnitizedFlatNormSquaredPrefixOrPostfix rhs)  {
        return this.unitized_flat_norm_squared();
    }
}
public extension UnitizedFlatNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public float operator /(VersorOdd rhs) {
        return rhs.unitized_flat_norm_squared();
    }
}
public extension VersorOdd {
    public float unitized_flat_norm_squared() {
        return this.flat_bulk_norm_squared().scalar / (this.flat_weight_norm_squared().e12345);
    }
}

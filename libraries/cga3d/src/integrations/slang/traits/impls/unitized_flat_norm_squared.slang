implementing cga3d;
using traits;
// Fancy postfix trick
public float operator /(AntiCircleRotor lhs, UnitizedFlatNormSquaredPrefixOrPostfix rhs)  {
    return lhs.unitized_flat_norm_squared();
    }
// Fancy prefix trick
public float operator /(UnitizedFlatNormSquaredPrefixOrPostfix lhs, AntiCircleRotor rhs) {
    return rhs.unitized_flat_norm_squared();
}
public extension AntiCircleRotor {
    public float unitized_flat_norm_squared() {
        return this.flat_bulk_norm_squared().scalar / (this.flat_weight_norm_squared().e12345);
    }
}
// Fancy postfix trick
public float operator /(AntiDipoleInversion lhs, UnitizedFlatNormSquaredPrefixOrPostfix rhs)  {
    return lhs.unitized_flat_norm_squared();
    }
// Fancy prefix trick
public float operator /(UnitizedFlatNormSquaredPrefixOrPostfix lhs, AntiDipoleInversion rhs) {
    return rhs.unitized_flat_norm_squared();
}
public extension AntiDipoleInversion {
    public float unitized_flat_norm_squared() {
        return this.flat_bulk_norm_squared().scalar / (this.flat_weight_norm_squared().e12345);
    }
}
// Fancy postfix trick
public float operator /(Circle lhs, UnitizedFlatNormSquaredPrefixOrPostfix rhs)  {
    return lhs.unitized_flat_norm_squared();
    }
// Fancy prefix trick
public float operator /(UnitizedFlatNormSquaredPrefixOrPostfix lhs, Circle rhs) {
    return rhs.unitized_flat_norm_squared();
}
public extension Circle {
    public float unitized_flat_norm_squared() {
        return this.flat_bulk_norm_squared().scalar / (this.flat_weight_norm_squared().e12345);
    }
}
// Fancy postfix trick
public float operator /(CircleRotor lhs, UnitizedFlatNormSquaredPrefixOrPostfix rhs)  {
    return lhs.unitized_flat_norm_squared();
    }
// Fancy prefix trick
public float operator /(UnitizedFlatNormSquaredPrefixOrPostfix lhs, CircleRotor rhs) {
    return rhs.unitized_flat_norm_squared();
}
public extension CircleRotor {
    public float unitized_flat_norm_squared() {
        return this.flat_bulk_norm_squared().scalar / (this.flat_weight_norm_squared().e12345);
    }
}
// Fancy postfix trick
public float operator /(Dipole lhs, UnitizedFlatNormSquaredPrefixOrPostfix rhs)  {
    return lhs.unitized_flat_norm_squared();
    }
// Fancy prefix trick
public float operator /(UnitizedFlatNormSquaredPrefixOrPostfix lhs, Dipole rhs) {
    return rhs.unitized_flat_norm_squared();
}
public extension Dipole {
    public float unitized_flat_norm_squared() {
        return this.flat_bulk_norm_squared().scalar / (this.flat_weight_norm_squared().e12345);
    }
}
// Fancy postfix trick
public float operator /(DipoleInversion lhs, UnitizedFlatNormSquaredPrefixOrPostfix rhs)  {
    return lhs.unitized_flat_norm_squared();
    }
// Fancy prefix trick
public float operator /(UnitizedFlatNormSquaredPrefixOrPostfix lhs, DipoleInversion rhs) {
    return rhs.unitized_flat_norm_squared();
}
public extension DipoleInversion {
    public float unitized_flat_norm_squared() {
        return this.flat_bulk_norm_squared().scalar / (this.flat_weight_norm_squared().e12345);
    }
}
// Fancy postfix trick
public float operator /(DualNum lhs, UnitizedFlatNormSquaredPrefixOrPostfix rhs)  {
    return lhs.unitized_flat_norm_squared();
    }
// Fancy prefix trick
public float operator /(UnitizedFlatNormSquaredPrefixOrPostfix lhs, DualNum rhs) {
    return rhs.unitized_flat_norm_squared();
}
public extension DualNum {
    public float unitized_flat_norm_squared() {
        return this.flat_bulk_norm_squared().scalar / (this.flat_weight_norm_squared().e12345);
    }
}
// Fancy postfix trick
public float operator /(FlatPoint lhs, UnitizedFlatNormSquaredPrefixOrPostfix rhs)  {
    return lhs.unitized_flat_norm_squared();
    }
// Fancy prefix trick
public float operator /(UnitizedFlatNormSquaredPrefixOrPostfix lhs, FlatPoint rhs) {
    return rhs.unitized_flat_norm_squared();
}
public extension FlatPoint {
    public float unitized_flat_norm_squared() {
        return this.flat_bulk_norm_squared().scalar / (this.flat_weight_norm_squared().e12345);
    }
}
// Fancy postfix trick
public float operator /(Flector lhs, UnitizedFlatNormSquaredPrefixOrPostfix rhs)  {
    return lhs.unitized_flat_norm_squared();
    }
// Fancy prefix trick
public float operator /(UnitizedFlatNormSquaredPrefixOrPostfix lhs, Flector rhs) {
    return rhs.unitized_flat_norm_squared();
}
public extension Flector {
    public float unitized_flat_norm_squared() {
        return this.flat_bulk_norm_squared().scalar / (this.flat_weight_norm_squared().e12345);
    }
}
// Fancy postfix trick
public float operator /(Line lhs, UnitizedFlatNormSquaredPrefixOrPostfix rhs)  {
    return lhs.unitized_flat_norm_squared();
    }
// Fancy prefix trick
public float operator /(UnitizedFlatNormSquaredPrefixOrPostfix lhs, Line rhs) {
    return rhs.unitized_flat_norm_squared();
}
public extension Line {
    public float unitized_flat_norm_squared() {
        return this.flat_bulk_norm_squared().scalar / (this.flat_weight_norm_squared().e12345);
    }
}
// Fancy postfix trick
public float operator /(Motor lhs, UnitizedFlatNormSquaredPrefixOrPostfix rhs)  {
    return lhs.unitized_flat_norm_squared();
    }
// Fancy prefix trick
public float operator /(UnitizedFlatNormSquaredPrefixOrPostfix lhs, Motor rhs) {
    return rhs.unitized_flat_norm_squared();
}
public extension Motor {
    public float unitized_flat_norm_squared() {
        return this.flat_bulk_norm_squared().scalar / (this.flat_weight_norm_squared().e12345);
    }
}
// Fancy postfix trick
public float operator /(MultiVector lhs, UnitizedFlatNormSquaredPrefixOrPostfix rhs)  {
    return lhs.unitized_flat_norm_squared();
    }
// Fancy prefix trick
public float operator /(UnitizedFlatNormSquaredPrefixOrPostfix lhs, MultiVector rhs) {
    return rhs.unitized_flat_norm_squared();
}
public extension MultiVector {
    public float unitized_flat_norm_squared() {
        return this.flat_bulk_norm_squared().scalar / (this.flat_weight_norm_squared().e12345);
    }
}
// Fancy postfix trick
public float operator /(Plane lhs, UnitizedFlatNormSquaredPrefixOrPostfix rhs)  {
    return lhs.unitized_flat_norm_squared();
    }
// Fancy prefix trick
public float operator /(UnitizedFlatNormSquaredPrefixOrPostfix lhs, Plane rhs) {
    return rhs.unitized_flat_norm_squared();
}
public extension Plane {
    public float unitized_flat_norm_squared() {
        return this.flat_bulk_norm_squared().scalar / (this.flat_weight_norm_squared().e12345);
    }
}
// Fancy postfix trick
public float operator /(Sphere lhs, UnitizedFlatNormSquaredPrefixOrPostfix rhs)  {
    return lhs.unitized_flat_norm_squared();
    }
// Fancy prefix trick
public float operator /(UnitizedFlatNormSquaredPrefixOrPostfix lhs, Sphere rhs) {
    return rhs.unitized_flat_norm_squared();
}
public extension Sphere {
    public float unitized_flat_norm_squared() {
        return this.flat_bulk_norm_squared().scalar / (this.flat_weight_norm_squared().e12345);
    }
}
// Fancy postfix trick
public float operator /(VersorEven lhs, UnitizedFlatNormSquaredPrefixOrPostfix rhs)  {
    return lhs.unitized_flat_norm_squared();
    }
// Fancy prefix trick
public float operator /(UnitizedFlatNormSquaredPrefixOrPostfix lhs, VersorEven rhs) {
    return rhs.unitized_flat_norm_squared();
}
public extension VersorEven {
    public float unitized_flat_norm_squared() {
        return this.flat_bulk_norm_squared().scalar / (this.flat_weight_norm_squared().e12345);
    }
}
// Fancy postfix trick
public float operator /(VersorOdd lhs, UnitizedFlatNormSquaredPrefixOrPostfix rhs)  {
    return lhs.unitized_flat_norm_squared();
    }
// Fancy prefix trick
public float operator /(UnitizedFlatNormSquaredPrefixOrPostfix lhs, VersorOdd rhs) {
    return rhs.unitized_flat_norm_squared();
}
public extension VersorOdd {
    public float unitized_flat_norm_squared() {
        return this.flat_bulk_norm_squared().scalar / (this.flat_weight_norm_squared().e12345);
    }
}

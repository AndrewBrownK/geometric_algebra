implementing cga3d;
using traits;
// Fancy postfix trick
public AntiScalar operator /(AntiCircleRotor lhs, FlatWeightNormPrefixOrPostfix rhs)  {
    return lhs.flat_weight_norm();
    }
// Fancy prefix trick
public AntiScalar operator /(FlatWeightNormPrefixOrPostfix lhs, AntiCircleRotor rhs) {
    return rhs.flat_weight_norm();
}
public extension AntiCircleRotor {
    public AntiScalar flat_weight_norm() {
        return this.flat_weight_norm_squared().anti_square_root();
    }
}
// Fancy postfix trick
public AntiScalar operator /(AntiDipoleInversion lhs, FlatWeightNormPrefixOrPostfix rhs)  {
    return lhs.flat_weight_norm();
    }
// Fancy prefix trick
public AntiScalar operator /(FlatWeightNormPrefixOrPostfix lhs, AntiDipoleInversion rhs) {
    return rhs.flat_weight_norm();
}
public extension AntiDipoleInversion {
    public AntiScalar flat_weight_norm() {
        return this.flat_weight_norm_squared().anti_square_root();
    }
}
// Fancy postfix trick
public AntiScalar operator /(AntiScalar lhs, FlatWeightNormPrefixOrPostfix rhs)  {
    return lhs.flat_weight_norm();
    }
// Fancy prefix trick
public AntiScalar operator /(FlatWeightNormPrefixOrPostfix lhs, AntiScalar rhs) {
    return rhs.flat_weight_norm();
}
public extension AntiScalar {
    public AntiScalar flat_weight_norm() {
        return this.flat_weight_norm_squared().anti_square_root();
    }
}
// Fancy postfix trick
public AntiScalar operator /(Circle lhs, FlatWeightNormPrefixOrPostfix rhs)  {
    return lhs.flat_weight_norm();
    }
// Fancy prefix trick
public AntiScalar operator /(FlatWeightNormPrefixOrPostfix lhs, Circle rhs) {
    return rhs.flat_weight_norm();
}
public extension Circle {
    public AntiScalar flat_weight_norm() {
        return this.flat_weight_norm_squared().anti_square_root();
    }
}
// Fancy postfix trick
public AntiScalar operator /(CircleRotor lhs, FlatWeightNormPrefixOrPostfix rhs)  {
    return lhs.flat_weight_norm();
    }
// Fancy prefix trick
public AntiScalar operator /(FlatWeightNormPrefixOrPostfix lhs, CircleRotor rhs) {
    return rhs.flat_weight_norm();
}
public extension CircleRotor {
    public AntiScalar flat_weight_norm() {
        return this.flat_weight_norm_squared().anti_square_root();
    }
}
// Fancy postfix trick
public AntiScalar operator /(Dipole lhs, FlatWeightNormPrefixOrPostfix rhs)  {
    return lhs.flat_weight_norm();
    }
// Fancy prefix trick
public AntiScalar operator /(FlatWeightNormPrefixOrPostfix lhs, Dipole rhs) {
    return rhs.flat_weight_norm();
}
public extension Dipole {
    public AntiScalar flat_weight_norm() {
        return this.flat_weight_norm_squared().anti_square_root();
    }
}
// Fancy postfix trick
public AntiScalar operator /(DipoleInversion lhs, FlatWeightNormPrefixOrPostfix rhs)  {
    return lhs.flat_weight_norm();
    }
// Fancy prefix trick
public AntiScalar operator /(FlatWeightNormPrefixOrPostfix lhs, DipoleInversion rhs) {
    return rhs.flat_weight_norm();
}
public extension DipoleInversion {
    public AntiScalar flat_weight_norm() {
        return this.flat_weight_norm_squared().anti_square_root();
    }
}
// Fancy postfix trick
public AntiScalar operator /(DualNum lhs, FlatWeightNormPrefixOrPostfix rhs)  {
    return lhs.flat_weight_norm();
    }
// Fancy prefix trick
public AntiScalar operator /(FlatWeightNormPrefixOrPostfix lhs, DualNum rhs) {
    return rhs.flat_weight_norm();
}
public extension DualNum {
    public AntiScalar flat_weight_norm() {
        return this.flat_weight_norm_squared().anti_square_root();
    }
}
// Fancy postfix trick
public AntiScalar operator /(FlatPoint lhs, FlatWeightNormPrefixOrPostfix rhs)  {
    return lhs.flat_weight_norm();
    }
// Fancy prefix trick
public AntiScalar operator /(FlatWeightNormPrefixOrPostfix lhs, FlatPoint rhs) {
    return rhs.flat_weight_norm();
}
public extension FlatPoint {
    public AntiScalar flat_weight_norm() {
        return this.flat_weight_norm_squared().anti_square_root();
    }
}
// Fancy postfix trick
public AntiScalar operator /(Flector lhs, FlatWeightNormPrefixOrPostfix rhs)  {
    return lhs.flat_weight_norm();
    }
// Fancy prefix trick
public AntiScalar operator /(FlatWeightNormPrefixOrPostfix lhs, Flector rhs) {
    return rhs.flat_weight_norm();
}
public extension Flector {
    public AntiScalar flat_weight_norm() {
        return this.flat_weight_norm_squared().anti_square_root();
    }
}
// Fancy postfix trick
public AntiScalar operator /(Line lhs, FlatWeightNormPrefixOrPostfix rhs)  {
    return lhs.flat_weight_norm();
    }
// Fancy prefix trick
public AntiScalar operator /(FlatWeightNormPrefixOrPostfix lhs, Line rhs) {
    return rhs.flat_weight_norm();
}
public extension Line {
    public AntiScalar flat_weight_norm() {
        return this.flat_weight_norm_squared().anti_square_root();
    }
}
// Fancy postfix trick
public AntiScalar operator /(Motor lhs, FlatWeightNormPrefixOrPostfix rhs)  {
    return lhs.flat_weight_norm();
    }
// Fancy prefix trick
public AntiScalar operator /(FlatWeightNormPrefixOrPostfix lhs, Motor rhs) {
    return rhs.flat_weight_norm();
}
public extension Motor {
    public AntiScalar flat_weight_norm() {
        return this.flat_weight_norm_squared().anti_square_root();
    }
}
// Fancy postfix trick
public AntiScalar operator /(MultiVector lhs, FlatWeightNormPrefixOrPostfix rhs)  {
    return lhs.flat_weight_norm();
    }
// Fancy prefix trick
public AntiScalar operator /(FlatWeightNormPrefixOrPostfix lhs, MultiVector rhs) {
    return rhs.flat_weight_norm();
}
public extension MultiVector {
    public AntiScalar flat_weight_norm() {
        return this.flat_weight_norm_squared().anti_square_root();
    }
}
// Fancy postfix trick
public AntiScalar operator /(Plane lhs, FlatWeightNormPrefixOrPostfix rhs)  {
    return lhs.flat_weight_norm();
    }
// Fancy prefix trick
public AntiScalar operator /(FlatWeightNormPrefixOrPostfix lhs, Plane rhs) {
    return rhs.flat_weight_norm();
}
public extension Plane {
    public AntiScalar flat_weight_norm() {
        return this.flat_weight_norm_squared().anti_square_root();
    }
}
// Fancy postfix trick
public AntiScalar operator /(Sphere lhs, FlatWeightNormPrefixOrPostfix rhs)  {
    return lhs.flat_weight_norm();
    }
// Fancy prefix trick
public AntiScalar operator /(FlatWeightNormPrefixOrPostfix lhs, Sphere rhs) {
    return rhs.flat_weight_norm();
}
public extension Sphere {
    public AntiScalar flat_weight_norm() {
        return this.flat_weight_norm_squared().anti_square_root();
    }
}
// Fancy postfix trick
public AntiScalar operator /(VersorEven lhs, FlatWeightNormPrefixOrPostfix rhs)  {
    return lhs.flat_weight_norm();
    }
// Fancy prefix trick
public AntiScalar operator /(FlatWeightNormPrefixOrPostfix lhs, VersorEven rhs) {
    return rhs.flat_weight_norm();
}
public extension VersorEven {
    public AntiScalar flat_weight_norm() {
        return this.flat_weight_norm_squared().anti_square_root();
    }
}
// Fancy postfix trick
public AntiScalar operator /(VersorOdd lhs, FlatWeightNormPrefixOrPostfix rhs)  {
    return lhs.flat_weight_norm();
    }
// Fancy prefix trick
public AntiScalar operator /(FlatWeightNormPrefixOrPostfix lhs, VersorOdd rhs) {
    return rhs.flat_weight_norm();
}
public extension VersorOdd {
    public AntiScalar flat_weight_norm() {
        return this.flat_weight_norm_squared().anti_square_root();
    }
}

implementing cga3d;
using traits;
public extension AntiCircleRotor {
    // Fancy postfix trick
    public AntiScalar operator /(FlatWeightNormPrefixOrPostfix rhs)  {
        return this.flat_weight_norm();
    }
}
public extension FlatWeightNormPrefixOrPostfix {
    // Fancy prefix trick
    public AntiScalar operator /(AntiCircleRotor rhs) {
        return rhs.flat_weight_norm();
    }
}
public extension AntiCircleRotor {
    public AntiScalar flat_weight_norm() {
        return this.flat_weight_norm_squared().anti_square_root();
    }
}
public extension AntiDipoleInversion {
    // Fancy postfix trick
    public AntiScalar operator /(FlatWeightNormPrefixOrPostfix rhs)  {
        return this.flat_weight_norm();
    }
}
public extension FlatWeightNormPrefixOrPostfix {
    // Fancy prefix trick
    public AntiScalar operator /(AntiDipoleInversion rhs) {
        return rhs.flat_weight_norm();
    }
}
public extension AntiDipoleInversion {
    public AntiScalar flat_weight_norm() {
        return this.flat_weight_norm_squared().anti_square_root();
    }
}
public extension AntiScalar {
    // Fancy postfix trick
    public AntiScalar operator /(FlatWeightNormPrefixOrPostfix rhs)  {
        return this.flat_weight_norm();
    }
}
public extension FlatWeightNormPrefixOrPostfix {
    // Fancy prefix trick
    public AntiScalar operator /(AntiScalar rhs) {
        return rhs.flat_weight_norm();
    }
}
public extension AntiScalar {
    public AntiScalar flat_weight_norm() {
        return this.flat_weight_norm_squared().anti_square_root();
    }
}
public extension Circle {
    // Fancy postfix trick
    public AntiScalar operator /(FlatWeightNormPrefixOrPostfix rhs)  {
        return this.flat_weight_norm();
    }
}
public extension FlatWeightNormPrefixOrPostfix {
    // Fancy prefix trick
    public AntiScalar operator /(Circle rhs) {
        return rhs.flat_weight_norm();
    }
}
public extension Circle {
    public AntiScalar flat_weight_norm() {
        return this.flat_weight_norm_squared().anti_square_root();
    }
}
public extension CircleRotor {
    // Fancy postfix trick
    public AntiScalar operator /(FlatWeightNormPrefixOrPostfix rhs)  {
        return this.flat_weight_norm();
    }
}
public extension FlatWeightNormPrefixOrPostfix {
    // Fancy prefix trick
    public AntiScalar operator /(CircleRotor rhs) {
        return rhs.flat_weight_norm();
    }
}
public extension CircleRotor {
    public AntiScalar flat_weight_norm() {
        return this.flat_weight_norm_squared().anti_square_root();
    }
}
public extension Dipole {
    // Fancy postfix trick
    public AntiScalar operator /(FlatWeightNormPrefixOrPostfix rhs)  {
        return this.flat_weight_norm();
    }
}
public extension FlatWeightNormPrefixOrPostfix {
    // Fancy prefix trick
    public AntiScalar operator /(Dipole rhs) {
        return rhs.flat_weight_norm();
    }
}
public extension Dipole {
    public AntiScalar flat_weight_norm() {
        return this.flat_weight_norm_squared().anti_square_root();
    }
}
public extension DipoleInversion {
    // Fancy postfix trick
    public AntiScalar operator /(FlatWeightNormPrefixOrPostfix rhs)  {
        return this.flat_weight_norm();
    }
}
public extension FlatWeightNormPrefixOrPostfix {
    // Fancy prefix trick
    public AntiScalar operator /(DipoleInversion rhs) {
        return rhs.flat_weight_norm();
    }
}
public extension DipoleInversion {
    public AntiScalar flat_weight_norm() {
        return this.flat_weight_norm_squared().anti_square_root();
    }
}
public extension DualNum {
    // Fancy postfix trick
    public AntiScalar operator /(FlatWeightNormPrefixOrPostfix rhs)  {
        return this.flat_weight_norm();
    }
}
public extension FlatWeightNormPrefixOrPostfix {
    // Fancy prefix trick
    public AntiScalar operator /(DualNum rhs) {
        return rhs.flat_weight_norm();
    }
}
public extension DualNum {
    public AntiScalar flat_weight_norm() {
        return this.flat_weight_norm_squared().anti_square_root();
    }
}
public extension FlatPoint {
    // Fancy postfix trick
    public AntiScalar operator /(FlatWeightNormPrefixOrPostfix rhs)  {
        return this.flat_weight_norm();
    }
}
public extension FlatWeightNormPrefixOrPostfix {
    // Fancy prefix trick
    public AntiScalar operator /(FlatPoint rhs) {
        return rhs.flat_weight_norm();
    }
}
public extension FlatPoint {
    public AntiScalar flat_weight_norm() {
        return this.flat_weight_norm_squared().anti_square_root();
    }
}
public extension Flector {
    // Fancy postfix trick
    public AntiScalar operator /(FlatWeightNormPrefixOrPostfix rhs)  {
        return this.flat_weight_norm();
    }
}
public extension FlatWeightNormPrefixOrPostfix {
    // Fancy prefix trick
    public AntiScalar operator /(Flector rhs) {
        return rhs.flat_weight_norm();
    }
}
public extension Flector {
    public AntiScalar flat_weight_norm() {
        return this.flat_weight_norm_squared().anti_square_root();
    }
}
public extension Line {
    // Fancy postfix trick
    public AntiScalar operator /(FlatWeightNormPrefixOrPostfix rhs)  {
        return this.flat_weight_norm();
    }
}
public extension FlatWeightNormPrefixOrPostfix {
    // Fancy prefix trick
    public AntiScalar operator /(Line rhs) {
        return rhs.flat_weight_norm();
    }
}
public extension Line {
    public AntiScalar flat_weight_norm() {
        return this.flat_weight_norm_squared().anti_square_root();
    }
}
public extension Motor {
    // Fancy postfix trick
    public AntiScalar operator /(FlatWeightNormPrefixOrPostfix rhs)  {
        return this.flat_weight_norm();
    }
}
public extension FlatWeightNormPrefixOrPostfix {
    // Fancy prefix trick
    public AntiScalar operator /(Motor rhs) {
        return rhs.flat_weight_norm();
    }
}
public extension Motor {
    public AntiScalar flat_weight_norm() {
        return this.flat_weight_norm_squared().anti_square_root();
    }
}
public extension MultiVector {
    // Fancy postfix trick
    public AntiScalar operator /(FlatWeightNormPrefixOrPostfix rhs)  {
        return this.flat_weight_norm();
    }
}
public extension FlatWeightNormPrefixOrPostfix {
    // Fancy prefix trick
    public AntiScalar operator /(MultiVector rhs) {
        return rhs.flat_weight_norm();
    }
}
public extension MultiVector {
    public AntiScalar flat_weight_norm() {
        return this.flat_weight_norm_squared().anti_square_root();
    }
}
public extension Plane {
    // Fancy postfix trick
    public AntiScalar operator /(FlatWeightNormPrefixOrPostfix rhs)  {
        return this.flat_weight_norm();
    }
}
public extension FlatWeightNormPrefixOrPostfix {
    // Fancy prefix trick
    public AntiScalar operator /(Plane rhs) {
        return rhs.flat_weight_norm();
    }
}
public extension Plane {
    public AntiScalar flat_weight_norm() {
        return this.flat_weight_norm_squared().anti_square_root();
    }
}
public extension Sphere {
    // Fancy postfix trick
    public AntiScalar operator /(FlatWeightNormPrefixOrPostfix rhs)  {
        return this.flat_weight_norm();
    }
}
public extension FlatWeightNormPrefixOrPostfix {
    // Fancy prefix trick
    public AntiScalar operator /(Sphere rhs) {
        return rhs.flat_weight_norm();
    }
}
public extension Sphere {
    public AntiScalar flat_weight_norm() {
        return this.flat_weight_norm_squared().anti_square_root();
    }
}
public extension VersorEven {
    // Fancy postfix trick
    public AntiScalar operator /(FlatWeightNormPrefixOrPostfix rhs)  {
        return this.flat_weight_norm();
    }
}
public extension FlatWeightNormPrefixOrPostfix {
    // Fancy prefix trick
    public AntiScalar operator /(VersorEven rhs) {
        return rhs.flat_weight_norm();
    }
}
public extension VersorEven {
    public AntiScalar flat_weight_norm() {
        return this.flat_weight_norm_squared().anti_square_root();
    }
}
public extension VersorOdd {
    // Fancy postfix trick
    public AntiScalar operator /(FlatWeightNormPrefixOrPostfix rhs)  {
        return this.flat_weight_norm();
    }
}
public extension FlatWeightNormPrefixOrPostfix {
    // Fancy prefix trick
    public AntiScalar operator /(VersorOdd rhs) {
        return rhs.flat_weight_norm();
    }
}
public extension VersorOdd {
    public AntiScalar flat_weight_norm() {
        return this.flat_weight_norm_squared().anti_square_root();
    }
}

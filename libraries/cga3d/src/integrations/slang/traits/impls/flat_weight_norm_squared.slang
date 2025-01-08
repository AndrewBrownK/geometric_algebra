implementing cga3d;
using traits;
public extension AntiCircleRotor {
    // Fancy postfix trick
    public AntiScalar operator /(FlatWeightNormSquaredPrefixOrPostfix rhs)  {
        return this.flat_weight_norm_squared();
    }
}
public extension FlatWeightNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public AntiScalar operator /(AntiCircleRotor rhs) {
        return rhs.flat_weight_norm_squared();
    }
}
public extension AntiCircleRotor {
    public AntiScalar flat_weight_norm_squared() {
        FlatPoint flat_weight = this.flat_weight();
        return flat_weight.anti_dot_product(flat_weight);
    }
}
public extension AntiDipoleInversion {
    // Fancy postfix trick
    public AntiScalar operator /(FlatWeightNormSquaredPrefixOrPostfix rhs)  {
        return this.flat_weight_norm_squared();
    }
}
public extension FlatWeightNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public AntiScalar operator /(AntiDipoleInversion rhs) {
        return rhs.flat_weight_norm_squared();
    }
}
public extension AntiDipoleInversion {
    public AntiScalar flat_weight_norm_squared() {
        Line flat_weight = this.flat_weight();
        return flat_weight.anti_dot_product(flat_weight);
    }
}
public extension AntiScalar {
    // Fancy postfix trick
    public AntiScalar operator /(FlatWeightNormSquaredPrefixOrPostfix rhs)  {
        return this.flat_weight_norm_squared();
    }
}
public extension FlatWeightNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public AntiScalar operator /(AntiScalar rhs) {
        return rhs.flat_weight_norm_squared();
    }
}
public extension AntiScalar {
    public AntiScalar flat_weight_norm_squared() {
        AntiScalar flat_weight = this.flat_weight();
        return flat_weight.anti_dot_product(flat_weight);
    }
}
public extension Circle {
    // Fancy postfix trick
    public AntiScalar operator /(FlatWeightNormSquaredPrefixOrPostfix rhs)  {
        return this.flat_weight_norm_squared();
    }
}
public extension FlatWeightNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public AntiScalar operator /(Circle rhs) {
        return rhs.flat_weight_norm_squared();
    }
}
public extension Circle {
    public AntiScalar flat_weight_norm_squared() {
        Line flat_weight = this.flat_weight();
        return flat_weight.anti_dot_product(flat_weight);
    }
}
public extension CircleRotor {
    // Fancy postfix trick
    public AntiScalar operator /(FlatWeightNormSquaredPrefixOrPostfix rhs)  {
        return this.flat_weight_norm_squared();
    }
}
public extension FlatWeightNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public AntiScalar operator /(CircleRotor rhs) {
        return rhs.flat_weight_norm_squared();
    }
}
public extension CircleRotor {
    public AntiScalar flat_weight_norm_squared() {
        Motor flat_weight = this.flat_weight();
        return flat_weight.anti_dot_product(flat_weight);
    }
}
public extension Dipole {
    // Fancy postfix trick
    public AntiScalar operator /(FlatWeightNormSquaredPrefixOrPostfix rhs)  {
        return this.flat_weight_norm_squared();
    }
}
public extension FlatWeightNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public AntiScalar operator /(Dipole rhs) {
        return rhs.flat_weight_norm_squared();
    }
}
public extension Dipole {
    public AntiScalar flat_weight_norm_squared() {
        FlatPoint flat_weight = this.flat_weight();
        return flat_weight.anti_dot_product(flat_weight);
    }
}
public extension DipoleInversion {
    // Fancy postfix trick
    public AntiScalar operator /(FlatWeightNormSquaredPrefixOrPostfix rhs)  {
        return this.flat_weight_norm_squared();
    }
}
public extension FlatWeightNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public AntiScalar operator /(DipoleInversion rhs) {
        return rhs.flat_weight_norm_squared();
    }
}
public extension DipoleInversion {
    public AntiScalar flat_weight_norm_squared() {
        Flector flat_weight = this.flat_weight();
        return flat_weight.anti_dot_product(flat_weight);
    }
}
public extension DualNum {
    // Fancy postfix trick
    public AntiScalar operator /(FlatWeightNormSquaredPrefixOrPostfix rhs)  {
        return this.flat_weight_norm_squared();
    }
}
public extension FlatWeightNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public AntiScalar operator /(DualNum rhs) {
        return rhs.flat_weight_norm_squared();
    }
}
public extension DualNum {
    public AntiScalar flat_weight_norm_squared() {
        AntiScalar flat_weight = this.flat_weight();
        return flat_weight.anti_dot_product(flat_weight);
    }
}
public extension FlatPoint {
    // Fancy postfix trick
    public AntiScalar operator /(FlatWeightNormSquaredPrefixOrPostfix rhs)  {
        return this.flat_weight_norm_squared();
    }
}
public extension FlatWeightNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public AntiScalar operator /(FlatPoint rhs) {
        return rhs.flat_weight_norm_squared();
    }
}
public extension FlatPoint {
    public AntiScalar flat_weight_norm_squared() {
        FlatPoint flat_weight = this.flat_weight();
        return flat_weight.anti_dot_product(flat_weight);
    }
}
public extension Flector {
    // Fancy postfix trick
    public AntiScalar operator /(FlatWeightNormSquaredPrefixOrPostfix rhs)  {
        return this.flat_weight_norm_squared();
    }
}
public extension FlatWeightNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public AntiScalar operator /(Flector rhs) {
        return rhs.flat_weight_norm_squared();
    }
}
public extension Flector {
    public AntiScalar flat_weight_norm_squared() {
        Flector flat_weight = this.flat_weight();
        return flat_weight.anti_dot_product(flat_weight);
    }
}
public extension Line {
    // Fancy postfix trick
    public AntiScalar operator /(FlatWeightNormSquaredPrefixOrPostfix rhs)  {
        return this.flat_weight_norm_squared();
    }
}
public extension FlatWeightNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public AntiScalar operator /(Line rhs) {
        return rhs.flat_weight_norm_squared();
    }
}
public extension Line {
    public AntiScalar flat_weight_norm_squared() {
        Line flat_weight = this.flat_weight();
        return flat_weight.anti_dot_product(flat_weight);
    }
}
public extension Motor {
    // Fancy postfix trick
    public AntiScalar operator /(FlatWeightNormSquaredPrefixOrPostfix rhs)  {
        return this.flat_weight_norm_squared();
    }
}
public extension FlatWeightNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public AntiScalar operator /(Motor rhs) {
        return rhs.flat_weight_norm_squared();
    }
}
public extension Motor {
    public AntiScalar flat_weight_norm_squared() {
        Motor flat_weight = this.flat_weight();
        return flat_weight.anti_dot_product(flat_weight);
    }
}
public extension MultiVector {
    // Fancy postfix trick
    public AntiScalar operator /(FlatWeightNormSquaredPrefixOrPostfix rhs)  {
        return this.flat_weight_norm_squared();
    }
}
public extension FlatWeightNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public AntiScalar operator /(MultiVector rhs) {
        return rhs.flat_weight_norm_squared();
    }
}
public extension MultiVector {
    public AntiScalar flat_weight_norm_squared() {
        MultiVector flat_weight = this.flat_weight();
        return flat_weight.anti_dot_product(flat_weight);
    }
}
public extension Plane {
    // Fancy postfix trick
    public AntiScalar operator /(FlatWeightNormSquaredPrefixOrPostfix rhs)  {
        return this.flat_weight_norm_squared();
    }
}
public extension FlatWeightNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public AntiScalar operator /(Plane rhs) {
        return rhs.flat_weight_norm_squared();
    }
}
public extension Plane {
    public AntiScalar flat_weight_norm_squared() {
        Plane flat_weight = this.flat_weight();
        return flat_weight.anti_dot_product(flat_weight);
    }
}
public extension Sphere {
    // Fancy postfix trick
    public AntiScalar operator /(FlatWeightNormSquaredPrefixOrPostfix rhs)  {
        return this.flat_weight_norm_squared();
    }
}
public extension FlatWeightNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public AntiScalar operator /(Sphere rhs) {
        return rhs.flat_weight_norm_squared();
    }
}
public extension Sphere {
    public AntiScalar flat_weight_norm_squared() {
        Plane flat_weight = this.flat_weight();
        return flat_weight.anti_dot_product(flat_weight);
    }
}
public extension VersorEven {
    // Fancy postfix trick
    public AntiScalar operator /(FlatWeightNormSquaredPrefixOrPostfix rhs)  {
        return this.flat_weight_norm_squared();
    }
}
public extension FlatWeightNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public AntiScalar operator /(VersorEven rhs) {
        return rhs.flat_weight_norm_squared();
    }
}
public extension VersorEven {
    public AntiScalar flat_weight_norm_squared() {
        Motor flat_weight = this.flat_weight();
        return flat_weight.anti_dot_product(flat_weight);
    }
}
public extension VersorOdd {
    // Fancy postfix trick
    public AntiScalar operator /(FlatWeightNormSquaredPrefixOrPostfix rhs)  {
        return this.flat_weight_norm_squared();
    }
}
public extension FlatWeightNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public AntiScalar operator /(VersorOdd rhs) {
        return rhs.flat_weight_norm_squared();
    }
}
public extension VersorOdd {
    public AntiScalar flat_weight_norm_squared() {
        Flector flat_weight = this.flat_weight();
        return flat_weight.anti_dot_product(flat_weight);
    }
}

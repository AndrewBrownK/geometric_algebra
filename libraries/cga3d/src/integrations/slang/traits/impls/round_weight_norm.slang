implementing cga3d;
using traits;
public extension AntiCircleRotor {
    // Fancy postfix trick
    public AntiScalar operator /(RoundWeightNormPrefixOrPostfix rhs)  {
        return this.round_weight_norm();
    }
}
public extension RoundWeightNormPrefixOrPostfix {
    // Fancy prefix trick
    public AntiScalar operator /(AntiCircleRotor rhs) {
        return rhs.round_weight_norm();
    }
}
public extension AntiCircleRotor {
    public AntiScalar round_weight_norm() {
        return this.round_weight_norm_squared().anti_square_root();
    }
}
public extension AntiDipoleInversion {
    // Fancy postfix trick
    public AntiScalar operator /(RoundWeightNormPrefixOrPostfix rhs)  {
        return this.round_weight_norm();
    }
}
public extension RoundWeightNormPrefixOrPostfix {
    // Fancy prefix trick
    public AntiScalar operator /(AntiDipoleInversion rhs) {
        return rhs.round_weight_norm();
    }
}
public extension AntiDipoleInversion {
    public AntiScalar round_weight_norm() {
        return this.round_weight_norm_squared().anti_square_root();
    }
}
public extension Circle {
    // Fancy postfix trick
    public AntiScalar operator /(RoundWeightNormPrefixOrPostfix rhs)  {
        return this.round_weight_norm();
    }
}
public extension RoundWeightNormPrefixOrPostfix {
    // Fancy prefix trick
    public AntiScalar operator /(Circle rhs) {
        return rhs.round_weight_norm();
    }
}
public extension Circle {
    public AntiScalar round_weight_norm() {
        return this.round_weight_norm_squared().anti_square_root();
    }
}
public extension CircleRotor {
    // Fancy postfix trick
    public AntiScalar operator /(RoundWeightNormPrefixOrPostfix rhs)  {
        return this.round_weight_norm();
    }
}
public extension RoundWeightNormPrefixOrPostfix {
    // Fancy prefix trick
    public AntiScalar operator /(CircleRotor rhs) {
        return rhs.round_weight_norm();
    }
}
public extension CircleRotor {
    public AntiScalar round_weight_norm() {
        return this.round_weight_norm_squared().anti_square_root();
    }
}
public extension Dipole {
    // Fancy postfix trick
    public AntiScalar operator /(RoundWeightNormPrefixOrPostfix rhs)  {
        return this.round_weight_norm();
    }
}
public extension RoundWeightNormPrefixOrPostfix {
    // Fancy prefix trick
    public AntiScalar operator /(Dipole rhs) {
        return rhs.round_weight_norm();
    }
}
public extension Dipole {
    public AntiScalar round_weight_norm() {
        return this.round_weight_norm_squared().anti_square_root();
    }
}
public extension DipoleInversion {
    // Fancy postfix trick
    public AntiScalar operator /(RoundWeightNormPrefixOrPostfix rhs)  {
        return this.round_weight_norm();
    }
}
public extension RoundWeightNormPrefixOrPostfix {
    // Fancy prefix trick
    public AntiScalar operator /(DipoleInversion rhs) {
        return rhs.round_weight_norm();
    }
}
public extension DipoleInversion {
    public AntiScalar round_weight_norm() {
        return this.round_weight_norm_squared().anti_square_root();
    }
}
public extension MultiVector {
    // Fancy postfix trick
    public AntiScalar operator /(RoundWeightNormPrefixOrPostfix rhs)  {
        return this.round_weight_norm();
    }
}
public extension RoundWeightNormPrefixOrPostfix {
    // Fancy prefix trick
    public AntiScalar operator /(MultiVector rhs) {
        return rhs.round_weight_norm();
    }
}
public extension MultiVector {
    public AntiScalar round_weight_norm() {
        return this.round_weight_norm_squared().anti_square_root();
    }
}
public extension RoundPoint {
    // Fancy postfix trick
    public AntiScalar operator /(RoundWeightNormPrefixOrPostfix rhs)  {
        return this.round_weight_norm();
    }
}
public extension RoundWeightNormPrefixOrPostfix {
    // Fancy prefix trick
    public AntiScalar operator /(RoundPoint rhs) {
        return rhs.round_weight_norm();
    }
}
public extension RoundPoint {
    public AntiScalar round_weight_norm() {
        return this.round_weight_norm_squared().anti_square_root();
    }
}
public extension Sphere {
    // Fancy postfix trick
    public AntiScalar operator /(RoundWeightNormPrefixOrPostfix rhs)  {
        return this.round_weight_norm();
    }
}
public extension RoundWeightNormPrefixOrPostfix {
    // Fancy prefix trick
    public AntiScalar operator /(Sphere rhs) {
        return rhs.round_weight_norm();
    }
}
public extension Sphere {
    public AntiScalar round_weight_norm() {
        return this.round_weight_norm_squared().anti_square_root();
    }
}
public extension VersorEven {
    // Fancy postfix trick
    public AntiScalar operator /(RoundWeightNormPrefixOrPostfix rhs)  {
        return this.round_weight_norm();
    }
}
public extension RoundWeightNormPrefixOrPostfix {
    // Fancy prefix trick
    public AntiScalar operator /(VersorEven rhs) {
        return rhs.round_weight_norm();
    }
}
public extension VersorEven {
    public AntiScalar round_weight_norm() {
        return this.round_weight_norm_squared().anti_square_root();
    }
}
public extension VersorOdd {
    // Fancy postfix trick
    public AntiScalar operator /(RoundWeightNormPrefixOrPostfix rhs)  {
        return this.round_weight_norm();
    }
}
public extension RoundWeightNormPrefixOrPostfix {
    // Fancy prefix trick
    public AntiScalar operator /(VersorOdd rhs) {
        return rhs.round_weight_norm();
    }
}
public extension VersorOdd {
    public AntiScalar round_weight_norm() {
        return this.round_weight_norm_squared().anti_square_root();
    }
}

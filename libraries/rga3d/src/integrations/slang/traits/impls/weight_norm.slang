implementing rga3d;
using traits;
public extension AntiScalar {
    // Fancy postfix trick
    public AntiScalar operator /(WeightNormPrefixOrPostfix rhs)  {
        return this.weight_norm();
    }
}
public extension WeightNormPrefixOrPostfix {
    // Fancy prefix trick
    public AntiScalar operator /(AntiScalar rhs) {
        return rhs.weight_norm();
    }
}
public extension AntiScalar {
    public AntiScalar weight_norm() {
        return this.flat_weight_norm_squared().anti_square_root();
    }
}
public extension DualNum {
    // Fancy postfix trick
    public AntiScalar operator /(WeightNormPrefixOrPostfix rhs)  {
        return this.weight_norm();
    }
}
public extension WeightNormPrefixOrPostfix {
    // Fancy prefix trick
    public AntiScalar operator /(DualNum rhs) {
        return rhs.weight_norm();
    }
}
public extension DualNum {
    public AntiScalar weight_norm() {
        return this.flat_weight_norm_squared().anti_square_root();
    }
}
public extension Flector {
    // Fancy postfix trick
    public AntiScalar operator /(WeightNormPrefixOrPostfix rhs)  {
        return this.weight_norm();
    }
}
public extension WeightNormPrefixOrPostfix {
    // Fancy prefix trick
    public AntiScalar operator /(Flector rhs) {
        return rhs.weight_norm();
    }
}
public extension Flector {
    public AntiScalar weight_norm() {
        return this.flat_weight_norm_squared().anti_square_root();
    }
}
public extension Line {
    // Fancy postfix trick
    public AntiScalar operator /(WeightNormPrefixOrPostfix rhs)  {
        return this.weight_norm();
    }
}
public extension WeightNormPrefixOrPostfix {
    // Fancy prefix trick
    public AntiScalar operator /(Line rhs) {
        return rhs.weight_norm();
    }
}
public extension Line {
    public AntiScalar weight_norm() {
        return this.flat_weight_norm_squared().anti_square_root();
    }
}
public extension Motor {
    // Fancy postfix trick
    public AntiScalar operator /(WeightNormPrefixOrPostfix rhs)  {
        return this.weight_norm();
    }
}
public extension WeightNormPrefixOrPostfix {
    // Fancy prefix trick
    public AntiScalar operator /(Motor rhs) {
        return rhs.weight_norm();
    }
}
public extension Motor {
    public AntiScalar weight_norm() {
        return this.flat_weight_norm_squared().anti_square_root();
    }
}
public extension MultiVector {
    // Fancy postfix trick
    public AntiScalar operator /(WeightNormPrefixOrPostfix rhs)  {
        return this.weight_norm();
    }
}
public extension WeightNormPrefixOrPostfix {
    // Fancy prefix trick
    public AntiScalar operator /(MultiVector rhs) {
        return rhs.weight_norm();
    }
}
public extension MultiVector {
    public AntiScalar weight_norm() {
        return this.flat_weight_norm_squared().anti_square_root();
    }
}
public extension Origin {
    // Fancy postfix trick
    public AntiScalar operator /(WeightNormPrefixOrPostfix rhs)  {
        return this.weight_norm();
    }
}
public extension WeightNormPrefixOrPostfix {
    // Fancy prefix trick
    public AntiScalar operator /(Origin rhs) {
        return rhs.weight_norm();
    }
}
public extension Origin {
    public AntiScalar weight_norm() {
        return this.flat_weight_norm_squared().anti_square_root();
    }
}
public extension Plane {
    // Fancy postfix trick
    public AntiScalar operator /(WeightNormPrefixOrPostfix rhs)  {
        return this.weight_norm();
    }
}
public extension WeightNormPrefixOrPostfix {
    // Fancy prefix trick
    public AntiScalar operator /(Plane rhs) {
        return rhs.weight_norm();
    }
}
public extension Plane {
    public AntiScalar weight_norm() {
        return this.flat_weight_norm_squared().anti_square_root();
    }
}
public extension Point {
    // Fancy postfix trick
    public AntiScalar operator /(WeightNormPrefixOrPostfix rhs)  {
        return this.weight_norm();
    }
}
public extension WeightNormPrefixOrPostfix {
    // Fancy prefix trick
    public AntiScalar operator /(Point rhs) {
        return rhs.weight_norm();
    }
}
public extension Point {
    public AntiScalar weight_norm() {
        return this.flat_weight_norm_squared().anti_square_root();
    }
}

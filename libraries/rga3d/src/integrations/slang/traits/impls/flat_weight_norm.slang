implementing rga3d;
using traits;
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
public extension Point {
    // Fancy postfix trick
    public AntiScalar operator /(FlatWeightNormPrefixOrPostfix rhs)  {
        return this.flat_weight_norm();
    }
}
public extension FlatWeightNormPrefixOrPostfix {
    // Fancy prefix trick
    public AntiScalar operator /(Point rhs) {
        return rhs.flat_weight_norm();
    }
}
public extension Point {
    public AntiScalar flat_weight_norm() {
        return this.flat_weight_norm_squared().anti_square_root();
    }
}

implementing rga3d;
using traits;
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
public AntiScalar operator /(Point lhs, FlatWeightNormPrefixOrPostfix rhs)  {
    return lhs.flat_weight_norm();
    }
// Fancy prefix trick
public AntiScalar operator /(FlatWeightNormPrefixOrPostfix lhs, Point rhs) {
    return rhs.flat_weight_norm();
}
public extension Point {
    public AntiScalar flat_weight_norm() {
        return this.flat_weight_norm_squared().anti_square_root();
    }
}

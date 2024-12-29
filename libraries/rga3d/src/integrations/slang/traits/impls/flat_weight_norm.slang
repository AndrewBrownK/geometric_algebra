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
public extension Flector: FlatWeightNorm {
    public typedef AntiScalar FlatWeightNormOutput;
    public func flat_weight_norm() -> AntiScalar {
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
public extension Line: FlatWeightNorm {
    public typedef AntiScalar FlatWeightNormOutput;
    public func flat_weight_norm() -> AntiScalar {
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
public extension Motor: FlatWeightNorm {
    public typedef AntiScalar FlatWeightNormOutput;
    public func flat_weight_norm() -> AntiScalar {
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
public extension MultiVector: FlatWeightNorm {
    public typedef AntiScalar FlatWeightNormOutput;
    public func flat_weight_norm() -> AntiScalar {
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
public extension Point: FlatWeightNorm {
    public typedef AntiScalar FlatWeightNormOutput;
    public func flat_weight_norm() -> AntiScalar {
        return this.flat_weight_norm_squared().anti_square_root();
    }
}

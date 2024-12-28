using traits;
using traits;
public extension Flector {
    // Fancy postfix trick
    func operator/(_rhs: FlatWeightNormPrefixOrPostfix) -> AntiScalar {
        return this.flat_weight_norm();
    }
}
public extension FlatWeightNormPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: Flector) -> Flector.Output {
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
    func operator/(_rhs: FlatWeightNormPrefixOrPostfix) -> AntiScalar {
        return this.flat_weight_norm();
    }
}
public extension FlatWeightNormPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: Line) -> Line.Output {
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
    func operator/(_rhs: FlatWeightNormPrefixOrPostfix) -> AntiScalar {
        return this.flat_weight_norm();
    }
}
public extension FlatWeightNormPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: Motor) -> Motor.Output {
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
    func operator/(_rhs: FlatWeightNormPrefixOrPostfix) -> AntiScalar {
        return this.flat_weight_norm();
    }
}
public extension FlatWeightNormPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: MultiVector) -> MultiVector.Output {
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
    func operator/(_rhs: FlatWeightNormPrefixOrPostfix) -> AntiScalar {
        return this.flat_weight_norm();
    }
}
public extension FlatWeightNormPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: Point) -> Point.Output {
        return rhs.flat_weight_norm();
    }
}
public extension Point: FlatWeightNorm {
    public typedef AntiScalar FlatWeightNormOutput;
    public func flat_weight_norm() -> AntiScalar {
        return this.flat_weight_norm_squared().anti_square_root();
    }
}

using traits;
using traits;
public extension AntiScalar {
    // Fancy postfix trick
    func operator/(_rhs: WeightNormPrefixOrPostfix) -> AntiScalar {
        return this.weight_norm();
    }
    // Fancy postfix self-assign
    public func operator/=(const WeightNormPrefixOrPostfix& _rhs) -> AntiScalar& {
        this = this.weight_norm();
        return *this;
    }
}
public extension WeightNormPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: AntiScalar) -> AntiScalar.Output {
        return rhs.weight_norm();
    }
}
public extension AntiScalar: WeightNorm {
    public typedef AntiScalar WeightNormOutput;
    public func weight_norm() -> AntiScalar {
        return this.flat_weight_norm_squared().anti_square_root();
    }
}
public extension DualNum {
    // Fancy postfix trick
    func operator/(_rhs: WeightNormPrefixOrPostfix) -> AntiScalar {
        return this.weight_norm();
    }
}
public extension WeightNormPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: DualNum) -> DualNum.Output {
        return rhs.weight_norm();
    }
}
public extension DualNum: WeightNorm {
    public typedef AntiScalar WeightNormOutput;
    public func weight_norm() -> AntiScalar {
        return this.flat_weight_norm_squared().anti_square_root();
    }
}
public extension Flector {
    // Fancy postfix trick
    func operator/(_rhs: WeightNormPrefixOrPostfix) -> AntiScalar {
        return this.weight_norm();
    }
}
public extension WeightNormPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: Flector) -> Flector.Output {
        return rhs.weight_norm();
    }
}
public extension Flector: WeightNorm {
    public typedef AntiScalar WeightNormOutput;
    public func weight_norm() -> AntiScalar {
        return this.flat_weight_norm_squared().anti_square_root();
    }
}
public extension Line {
    // Fancy postfix trick
    func operator/(_rhs: WeightNormPrefixOrPostfix) -> AntiScalar {
        return this.weight_norm();
    }
}
public extension WeightNormPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: Line) -> Line.Output {
        return rhs.weight_norm();
    }
}
public extension Line: WeightNorm {
    public typedef AntiScalar WeightNormOutput;
    public func weight_norm() -> AntiScalar {
        return this.flat_weight_norm_squared().anti_square_root();
    }
}
public extension Motor {
    // Fancy postfix trick
    func operator/(_rhs: WeightNormPrefixOrPostfix) -> AntiScalar {
        return this.weight_norm();
    }
}
public extension WeightNormPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: Motor) -> Motor.Output {
        return rhs.weight_norm();
    }
}
public extension Motor: WeightNorm {
    public typedef AntiScalar WeightNormOutput;
    public func weight_norm() -> AntiScalar {
        return this.flat_weight_norm_squared().anti_square_root();
    }
}
public extension MultiVector {
    // Fancy postfix trick
    func operator/(_rhs: WeightNormPrefixOrPostfix) -> AntiScalar {
        return this.weight_norm();
    }
}
public extension WeightNormPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: MultiVector) -> MultiVector.Output {
        return rhs.weight_norm();
    }
}
public extension MultiVector: WeightNorm {
    public typedef AntiScalar WeightNormOutput;
    public func weight_norm() -> AntiScalar {
        return this.flat_weight_norm_squared().anti_square_root();
    }
}
public extension Origin {
    // Fancy postfix trick
    func operator/(_rhs: WeightNormPrefixOrPostfix) -> AntiScalar {
        return this.weight_norm();
    }
}
public extension WeightNormPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: Origin) -> Origin.Output {
        return rhs.weight_norm();
    }
}
public extension Origin: WeightNorm {
    public typedef AntiScalar WeightNormOutput;
    public func weight_norm() -> AntiScalar {
        return this.flat_weight_norm_squared().anti_square_root();
    }
}
public extension Plane {
    // Fancy postfix trick
    func operator/(_rhs: WeightNormPrefixOrPostfix) -> AntiScalar {
        return this.weight_norm();
    }
}
public extension WeightNormPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: Plane) -> Plane.Output {
        return rhs.weight_norm();
    }
}
public extension Plane: WeightNorm {
    public typedef AntiScalar WeightNormOutput;
    public func weight_norm() -> AntiScalar {
        return this.flat_weight_norm_squared().anti_square_root();
    }
}
public extension Point {
    // Fancy postfix trick
    func operator/(_rhs: WeightNormPrefixOrPostfix) -> AntiScalar {
        return this.weight_norm();
    }
}
public extension WeightNormPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: Point) -> Point.Output {
        return rhs.weight_norm();
    }
}
public extension Point: WeightNorm {
    public typedef AntiScalar WeightNormOutput;
    public func weight_norm() -> AntiScalar {
        return this.flat_weight_norm_squared().anti_square_root();
    }
}

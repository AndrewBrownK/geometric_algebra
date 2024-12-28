using traits;
using traits;
public extension AntiScalar {
    // Fancy postfix trick
    func operator/(_rhs: FlatWeightNormSquaredPrefixOrPostfix) -> AntiScalar {
        return this.flat_weight_norm_squared();
    }
    // Fancy postfix self-assign
    public func operator/=(const FlatWeightNormSquaredPrefixOrPostfix& _rhs) -> AntiScalar& {
        this = this.flat_weight_norm_squared();
        return *this;
    }
}
public extension FlatWeightNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: AntiScalar) -> AntiScalar.Output {
        return rhs.flat_weight_norm_squared();
    }
}
public extension AntiScalar: FlatWeightNormSquared {
    public typedef AntiScalar FlatWeightNormSquaredOutput;
    public func flat_weight_norm_squared() -> AntiScalar {
        AntiScalar flat_weight = this.flat_weight();
        return flat_weight.anti_dot_product(flat_weight);
    }
}
public extension DualNum {
    // Fancy postfix trick
    func operator/(_rhs: FlatWeightNormSquaredPrefixOrPostfix) -> AntiScalar {
        return this.flat_weight_norm_squared();
    }
}
public extension FlatWeightNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: DualNum) -> DualNum.Output {
        return rhs.flat_weight_norm_squared();
    }
}
public extension DualNum: FlatWeightNormSquared {
    public typedef AntiScalar FlatWeightNormSquaredOutput;
    public func flat_weight_norm_squared() -> AntiScalar {
        AntiScalar flat_weight = this.flat_weight();
        return flat_weight.anti_dot_product(flat_weight);
    }
}
public extension Flector {
    // Fancy postfix trick
    func operator/(_rhs: FlatWeightNormSquaredPrefixOrPostfix) -> AntiScalar {
        return this.flat_weight_norm_squared();
    }
}
public extension FlatWeightNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: Flector) -> Flector.Output {
        return rhs.flat_weight_norm_squared();
    }
}
public extension Flector: FlatWeightNormSquared {
    public typedef AntiScalar FlatWeightNormSquaredOutput;
    public func flat_weight_norm_squared() -> AntiScalar {
        Flector flat_weight = this.flat_weight();
        return flat_weight.anti_dot_product(flat_weight);
    }
}
public extension Line {
    // Fancy postfix trick
    func operator/(_rhs: FlatWeightNormSquaredPrefixOrPostfix) -> AntiScalar {
        return this.flat_weight_norm_squared();
    }
}
public extension FlatWeightNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: Line) -> Line.Output {
        return rhs.flat_weight_norm_squared();
    }
}
public extension Line: FlatWeightNormSquared {
    public typedef AntiScalar FlatWeightNormSquaredOutput;
    public func flat_weight_norm_squared() -> AntiScalar {
        Line flat_weight = this.flat_weight();
        return flat_weight.anti_dot_product(flat_weight);
    }
}
public extension Motor {
    // Fancy postfix trick
    func operator/(_rhs: FlatWeightNormSquaredPrefixOrPostfix) -> AntiScalar {
        return this.flat_weight_norm_squared();
    }
}
public extension FlatWeightNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: Motor) -> Motor.Output {
        return rhs.flat_weight_norm_squared();
    }
}
public extension Motor: FlatWeightNormSquared {
    public typedef AntiScalar FlatWeightNormSquaredOutput;
    public func flat_weight_norm_squared() -> AntiScalar {
        Motor flat_weight = this.flat_weight();
        return flat_weight.anti_dot_product(flat_weight);
    }
}
public extension MultiVector {
    // Fancy postfix trick
    func operator/(_rhs: FlatWeightNormSquaredPrefixOrPostfix) -> AntiScalar {
        return this.flat_weight_norm_squared();
    }
}
public extension FlatWeightNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: MultiVector) -> MultiVector.Output {
        return rhs.flat_weight_norm_squared();
    }
}
public extension MultiVector: FlatWeightNormSquared {
    public typedef AntiScalar FlatWeightNormSquaredOutput;
    public func flat_weight_norm_squared() -> AntiScalar {
        MultiVector flat_weight = this.flat_weight();
        return flat_weight.anti_dot_product(flat_weight);
    }
}
public extension Origin {
    // Fancy postfix trick
    func operator/(_rhs: FlatWeightNormSquaredPrefixOrPostfix) -> AntiScalar {
        return this.flat_weight_norm_squared();
    }
}
public extension FlatWeightNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: Origin) -> Origin.Output {
        return rhs.flat_weight_norm_squared();
    }
}
public extension Origin: FlatWeightNormSquared {
    public typedef AntiScalar FlatWeightNormSquaredOutput;
    public func flat_weight_norm_squared() -> AntiScalar {
        Origin flat_weight = this.flat_weight();
        return flat_weight.anti_dot_product(flat_weight);
    }
}
public extension Plane {
    // Fancy postfix trick
    func operator/(_rhs: FlatWeightNormSquaredPrefixOrPostfix) -> AntiScalar {
        return this.flat_weight_norm_squared();
    }
}
public extension FlatWeightNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: Plane) -> Plane.Output {
        return rhs.flat_weight_norm_squared();
    }
}
public extension Plane: FlatWeightNormSquared {
    public typedef AntiScalar FlatWeightNormSquaredOutput;
    public func flat_weight_norm_squared() -> AntiScalar {
        Plane flat_weight = this.flat_weight();
        return flat_weight.anti_dot_product(flat_weight);
    }
}
public extension Point {
    // Fancy postfix trick
    func operator/(_rhs: FlatWeightNormSquaredPrefixOrPostfix) -> AntiScalar {
        return this.flat_weight_norm_squared();
    }
}
public extension FlatWeightNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: Point) -> Point.Output {
        return rhs.flat_weight_norm_squared();
    }
}
public extension Point: FlatWeightNormSquared {
    public typedef AntiScalar FlatWeightNormSquaredOutput;
    public func flat_weight_norm_squared() -> AntiScalar {
        Origin flat_weight = this.flat_weight();
        return flat_weight.anti_dot_product(flat_weight);
    }
}

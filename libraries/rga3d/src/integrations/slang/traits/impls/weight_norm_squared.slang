using traits;
using traits;
public extension AntiScalar {
    // Fancy postfix trick
    func operator/(_rhs: WeightNormSquaredPrefixOrPostfix) -> AntiScalar {
        return this.weight_norm_squared();
    }
    // Fancy postfix self-assign
    public func operator/=(const WeightNormSquaredPrefixOrPostfix& _rhs) -> AntiScalar& {
        this = this.weight_norm_squared();
        return *this;
    }
}
public extension WeightNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: AntiScalar) -> AntiScalar.Output {
        return rhs.weight_norm_squared();
    }
}
public extension AntiScalar: WeightNormSquared {
    public typedef AntiScalar WeightNormSquaredOutput;
    public func weight_norm_squared() -> AntiScalar {
        AntiScalar flat_weight = this.flat_weight();
        return flat_weight.anti_dot_product(flat_weight);
    }
}
public extension DualNum {
    // Fancy postfix trick
    func operator/(_rhs: WeightNormSquaredPrefixOrPostfix) -> AntiScalar {
        return this.weight_norm_squared();
    }
}
public extension WeightNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: DualNum) -> DualNum.Output {
        return rhs.weight_norm_squared();
    }
}
public extension DualNum: WeightNormSquared {
    public typedef AntiScalar WeightNormSquaredOutput;
    public func weight_norm_squared() -> AntiScalar {
        AntiScalar flat_weight = this.flat_weight();
        return flat_weight.anti_dot_product(flat_weight);
    }
}
public extension Flector {
    // Fancy postfix trick
    func operator/(_rhs: WeightNormSquaredPrefixOrPostfix) -> AntiScalar {
        return this.weight_norm_squared();
    }
}
public extension WeightNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: Flector) -> Flector.Output {
        return rhs.weight_norm_squared();
    }
}
public extension Flector: WeightNormSquared {
    public typedef AntiScalar WeightNormSquaredOutput;
    public func weight_norm_squared() -> AntiScalar {
        Flector flat_weight = this.flat_weight();
        return flat_weight.anti_dot_product(flat_weight);
    }
}
public extension Line {
    // Fancy postfix trick
    func operator/(_rhs: WeightNormSquaredPrefixOrPostfix) -> AntiScalar {
        return this.weight_norm_squared();
    }
}
public extension WeightNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: Line) -> Line.Output {
        return rhs.weight_norm_squared();
    }
}
public extension Line: WeightNormSquared {
    public typedef AntiScalar WeightNormSquaredOutput;
    public func weight_norm_squared() -> AntiScalar {
        Line flat_weight = this.flat_weight();
        return flat_weight.anti_dot_product(flat_weight);
    }
}
public extension Motor {
    // Fancy postfix trick
    func operator/(_rhs: WeightNormSquaredPrefixOrPostfix) -> AntiScalar {
        return this.weight_norm_squared();
    }
}
public extension WeightNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: Motor) -> Motor.Output {
        return rhs.weight_norm_squared();
    }
}
public extension Motor: WeightNormSquared {
    public typedef AntiScalar WeightNormSquaredOutput;
    public func weight_norm_squared() -> AntiScalar {
        Motor flat_weight = this.flat_weight();
        return flat_weight.anti_dot_product(flat_weight);
    }
}
public extension MultiVector {
    // Fancy postfix trick
    func operator/(_rhs: WeightNormSquaredPrefixOrPostfix) -> AntiScalar {
        return this.weight_norm_squared();
    }
}
public extension WeightNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: MultiVector) -> MultiVector.Output {
        return rhs.weight_norm_squared();
    }
}
public extension MultiVector: WeightNormSquared {
    public typedef AntiScalar WeightNormSquaredOutput;
    public func weight_norm_squared() -> AntiScalar {
        MultiVector flat_weight = this.flat_weight();
        return flat_weight.anti_dot_product(flat_weight);
    }
}
public extension Origin {
    // Fancy postfix trick
    func operator/(_rhs: WeightNormSquaredPrefixOrPostfix) -> AntiScalar {
        return this.weight_norm_squared();
    }
}
public extension WeightNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: Origin) -> Origin.Output {
        return rhs.weight_norm_squared();
    }
}
public extension Origin: WeightNormSquared {
    public typedef AntiScalar WeightNormSquaredOutput;
    public func weight_norm_squared() -> AntiScalar {
        Origin flat_weight = this.flat_weight();
        return flat_weight.anti_dot_product(flat_weight);
    }
}
public extension Plane {
    // Fancy postfix trick
    func operator/(_rhs: WeightNormSquaredPrefixOrPostfix) -> AntiScalar {
        return this.weight_norm_squared();
    }
}
public extension WeightNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: Plane) -> Plane.Output {
        return rhs.weight_norm_squared();
    }
}
public extension Plane: WeightNormSquared {
    public typedef AntiScalar WeightNormSquaredOutput;
    public func weight_norm_squared() -> AntiScalar {
        Plane flat_weight = this.flat_weight();
        return flat_weight.anti_dot_product(flat_weight);
    }
}
public extension Point {
    // Fancy postfix trick
    func operator/(_rhs: WeightNormSquaredPrefixOrPostfix) -> AntiScalar {
        return this.weight_norm_squared();
    }
}
public extension WeightNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: Point) -> Point.Output {
        return rhs.weight_norm_squared();
    }
}
public extension Point: WeightNormSquared {
    public typedef AntiScalar WeightNormSquaredOutput;
    public func weight_norm_squared() -> AntiScalar {
        Origin flat_weight = this.flat_weight();
        return flat_weight.anti_dot_product(flat_weight);
    }
}

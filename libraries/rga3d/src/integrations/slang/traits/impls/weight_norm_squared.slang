implementing rga3d;
using traits;
public extension AntiScalar {
    // Fancy postfix trick
    public AntiScalar operator /(WeightNormSquaredPrefixOrPostfix rhs)  {
        return this.weight_norm_squared();
    }
    // Fancy postfix self-assign
    public func operator /=(const WeightNormSquaredPrefixOrPostfix& rhs) -> AntiScalar& {
        this = this.weight_norm_squared();
        return *this;
    }
}
public extension WeightNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public AntiScalar operator /(AntiScalar rhs) {
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
    public AntiScalar operator /(WeightNormSquaredPrefixOrPostfix rhs)  {
        return this.weight_norm_squared();
    }
}
public extension WeightNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public AntiScalar operator /(DualNum rhs) {
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
    public AntiScalar operator /(WeightNormSquaredPrefixOrPostfix rhs)  {
        return this.weight_norm_squared();
    }
}
public extension WeightNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public AntiScalar operator /(Flector rhs) {
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
    public AntiScalar operator /(WeightNormSquaredPrefixOrPostfix rhs)  {
        return this.weight_norm_squared();
    }
}
public extension WeightNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public AntiScalar operator /(Line rhs) {
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
    public AntiScalar operator /(WeightNormSquaredPrefixOrPostfix rhs)  {
        return this.weight_norm_squared();
    }
}
public extension WeightNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public AntiScalar operator /(Motor rhs) {
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
    public AntiScalar operator /(WeightNormSquaredPrefixOrPostfix rhs)  {
        return this.weight_norm_squared();
    }
}
public extension WeightNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public AntiScalar operator /(MultiVector rhs) {
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
    public AntiScalar operator /(WeightNormSquaredPrefixOrPostfix rhs)  {
        return this.weight_norm_squared();
    }
}
public extension WeightNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public AntiScalar operator /(Origin rhs) {
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
    public AntiScalar operator /(WeightNormSquaredPrefixOrPostfix rhs)  {
        return this.weight_norm_squared();
    }
}
public extension WeightNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public AntiScalar operator /(Plane rhs) {
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
    public AntiScalar operator /(WeightNormSquaredPrefixOrPostfix rhs)  {
        return this.weight_norm_squared();
    }
}
public extension WeightNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public AntiScalar operator /(Point rhs) {
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

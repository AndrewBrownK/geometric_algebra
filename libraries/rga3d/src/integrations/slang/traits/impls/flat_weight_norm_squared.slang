implementing rga3d;
using traits;
public extension AntiScalar {
    // Fancy postfix trick
    public AntiScalar operator /(FlatWeightNormSquaredPrefixOrPostfix rhs)  {
        return this.flat_weight_norm_squared();
    }
    // Fancy postfix self-assign
    public func operator /=(const FlatWeightNormSquaredPrefixOrPostfix& rhs) -> AntiScalar& {
        this = this.flat_weight_norm_squared();
        return *this;
    }
}
public extension FlatWeightNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public AntiScalar operator /(AntiScalar rhs) {
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
    public AntiScalar operator /(FlatWeightNormSquaredPrefixOrPostfix rhs)  {
        return this.flat_weight_norm_squared();
    }
}
public extension FlatWeightNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public AntiScalar operator /(DualNum rhs) {
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
    public AntiScalar operator /(FlatWeightNormSquaredPrefixOrPostfix rhs)  {
        return this.flat_weight_norm_squared();
    }
}
public extension FlatWeightNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public AntiScalar operator /(Flector rhs) {
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
    public AntiScalar operator /(FlatWeightNormSquaredPrefixOrPostfix rhs)  {
        return this.flat_weight_norm_squared();
    }
}
public extension FlatWeightNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public AntiScalar operator /(Line rhs) {
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
    public AntiScalar operator /(FlatWeightNormSquaredPrefixOrPostfix rhs)  {
        return this.flat_weight_norm_squared();
    }
}
public extension FlatWeightNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public AntiScalar operator /(Motor rhs) {
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
    public AntiScalar operator /(FlatWeightNormSquaredPrefixOrPostfix rhs)  {
        return this.flat_weight_norm_squared();
    }
}
public extension FlatWeightNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public AntiScalar operator /(MultiVector rhs) {
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
    public AntiScalar operator /(FlatWeightNormSquaredPrefixOrPostfix rhs)  {
        return this.flat_weight_norm_squared();
    }
}
public extension FlatWeightNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public AntiScalar operator /(Origin rhs) {
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
    public AntiScalar operator /(FlatWeightNormSquaredPrefixOrPostfix rhs)  {
        return this.flat_weight_norm_squared();
    }
}
public extension FlatWeightNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public AntiScalar operator /(Plane rhs) {
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
    public AntiScalar operator /(FlatWeightNormSquaredPrefixOrPostfix rhs)  {
        return this.flat_weight_norm_squared();
    }
}
public extension FlatWeightNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public AntiScalar operator /(Point rhs) {
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

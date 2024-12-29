implementing rga3d;
using traits;
public extension AntiScalar {
    // Fancy postfix trick
    public AntiScalar operator /(WeightNormSquaredPrefixOrPostfix rhs)  {
        return this.weight_norm_squared();
    }
}
public extension WeightNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public AntiScalar operator /(AntiScalar rhs) {
        return rhs.weight_norm_squared();
    }
}
public extension AntiScalar {
    public AntiScalar weight_norm_squared() {
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
public extension DualNum {
    public AntiScalar weight_norm_squared() {
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
public extension Flector {
    public AntiScalar weight_norm_squared() {
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
public extension Line {
    public AntiScalar weight_norm_squared() {
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
public extension Motor {
    public AntiScalar weight_norm_squared() {
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
public extension MultiVector {
    public AntiScalar weight_norm_squared() {
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
public extension Origin {
    public AntiScalar weight_norm_squared() {
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
public extension Plane {
    public AntiScalar weight_norm_squared() {
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
public extension Point {
    public AntiScalar weight_norm_squared() {
        Origin flat_weight = this.flat_weight();
        return flat_weight.anti_dot_product(flat_weight);
    }
}

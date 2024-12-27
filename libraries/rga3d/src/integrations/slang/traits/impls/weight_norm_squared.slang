using traits::AntiDotProduct;
using traits::FlatWeight;
extension AntiScalar {
    // Fancy infix trick
    func operator/(_rhs: WeightNormSquaredPrefixOrPostfix) -> AntiScalar {
        return this.weight_norm_squared();
    }
    func operator/=(const WeightNormSquaredPrefixOrPostfix& _rhs) -> AntiScalar& {
        this = this.weight_norm_squared();
        return *this;
    }
}
extension AntiScalar: WeightNormSquared {
    associatedtype Output = AntiScalar;
    func weight_norm_squared() -> Output {
AntiScalar flat_weight = this.flat_weight();
        return flat_weight.anti_dot_product(flat_weight);
    }
}
extension DualNum {
    // Fancy infix trick
    func operator/(_rhs: WeightNormSquaredPrefixOrPostfix) -> AntiScalar {
        return this.weight_norm_squared();
    }
}
extension DualNum: WeightNormSquared {
    associatedtype Output = AntiScalar;
    func weight_norm_squared() -> Output {
AntiScalar flat_weight = this.flat_weight();
        return flat_weight.anti_dot_product(flat_weight);
    }
}
extension Flector {
    // Fancy infix trick
    func operator/(_rhs: WeightNormSquaredPrefixOrPostfix) -> AntiScalar {
        return this.weight_norm_squared();
    }
}
extension Flector: WeightNormSquared {
    associatedtype Output = AntiScalar;
    func weight_norm_squared() -> Output {
Flector flat_weight = this.flat_weight();
        return flat_weight.anti_dot_product(flat_weight);
    }
}
extension Line {
    // Fancy infix trick
    func operator/(_rhs: WeightNormSquaredPrefixOrPostfix) -> AntiScalar {
        return this.weight_norm_squared();
    }
}
extension Line: WeightNormSquared {
    associatedtype Output = AntiScalar;
    func weight_norm_squared() -> Output {
Line flat_weight = this.flat_weight();
        return flat_weight.anti_dot_product(flat_weight);
    }
}
extension Motor {
    // Fancy infix trick
    func operator/(_rhs: WeightNormSquaredPrefixOrPostfix) -> AntiScalar {
        return this.weight_norm_squared();
    }
}
extension Motor: WeightNormSquared {
    associatedtype Output = AntiScalar;
    func weight_norm_squared() -> Output {
Motor flat_weight = this.flat_weight();
        return flat_weight.anti_dot_product(flat_weight);
    }
}
extension MultiVector {
    // Fancy infix trick
    func operator/(_rhs: WeightNormSquaredPrefixOrPostfix) -> AntiScalar {
        return this.weight_norm_squared();
    }
}
extension MultiVector: WeightNormSquared {
    associatedtype Output = AntiScalar;
    func weight_norm_squared() -> Output {
MultiVector flat_weight = this.flat_weight();
        return flat_weight.anti_dot_product(flat_weight);
    }
}
extension Origin {
    // Fancy infix trick
    func operator/(_rhs: WeightNormSquaredPrefixOrPostfix) -> AntiScalar {
        return this.weight_norm_squared();
    }
}
extension Origin: WeightNormSquared {
    associatedtype Output = AntiScalar;
    func weight_norm_squared() -> Output {
Origin flat_weight = this.flat_weight();
        return flat_weight.anti_dot_product(flat_weight);
    }
}
extension Plane {
    // Fancy infix trick
    func operator/(_rhs: WeightNormSquaredPrefixOrPostfix) -> AntiScalar {
        return this.weight_norm_squared();
    }
}
extension Plane: WeightNormSquared {
    associatedtype Output = AntiScalar;
    func weight_norm_squared() -> Output {
Plane flat_weight = this.flat_weight();
        return flat_weight.anti_dot_product(flat_weight);
    }
}
extension Point {
    // Fancy infix trick
    func operator/(_rhs: WeightNormSquaredPrefixOrPostfix) -> AntiScalar {
        return this.weight_norm_squared();
    }
}
extension Point: WeightNormSquared {
    associatedtype Output = AntiScalar;
    func weight_norm_squared() -> Output {
Origin flat_weight = this.flat_weight();
        return flat_weight.anti_dot_product(flat_weight);
    }
}

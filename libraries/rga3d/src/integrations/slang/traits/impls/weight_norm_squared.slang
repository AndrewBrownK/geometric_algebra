using traits::AntiDotProduct;
using traits::FlatWeight;
extension AntiScalar: WeightNormSquared {
    associatedtype Output = AntiScalar;
    func weight_norm_squared() -> Output {
AntiScalar flat_weight = this.flat_weight();
        return flat_weight.anti_dot_product(flat_weight);
    }
    func operator/(_rhs: WeightNormSquaredInfix) -> AntiScalar {
        return this.weight_norm_squared();
    }
    func operator/=(const WeightNormSquaredInfix& _rhs) -> AntiScalar& {
        this = this.weight_norm_squared();
        return *this;
    }
}
extension DualNum: WeightNormSquared {
    associatedtype Output = AntiScalar;
    func weight_norm_squared() -> Output {
AntiScalar flat_weight = this.flat_weight();
        return flat_weight.anti_dot_product(flat_weight);
    }
    func operator/(_rhs: WeightNormSquaredInfix) -> AntiScalar {
        return this.weight_norm_squared();
    }
}
extension Flector: WeightNormSquared {
    associatedtype Output = AntiScalar;
    func weight_norm_squared() -> Output {
Flector flat_weight = this.flat_weight();
        return flat_weight.anti_dot_product(flat_weight);
    }
    func operator/(_rhs: WeightNormSquaredInfix) -> AntiScalar {
        return this.weight_norm_squared();
    }
}
extension Line: WeightNormSquared {
    associatedtype Output = AntiScalar;
    func weight_norm_squared() -> Output {
Line flat_weight = this.flat_weight();
        return flat_weight.anti_dot_product(flat_weight);
    }
    func operator/(_rhs: WeightNormSquaredInfix) -> AntiScalar {
        return this.weight_norm_squared();
    }
}
extension Motor: WeightNormSquared {
    associatedtype Output = AntiScalar;
    func weight_norm_squared() -> Output {
Motor flat_weight = this.flat_weight();
        return flat_weight.anti_dot_product(flat_weight);
    }
    func operator/(_rhs: WeightNormSquaredInfix) -> AntiScalar {
        return this.weight_norm_squared();
    }
}
extension MultiVector: WeightNormSquared {
    associatedtype Output = AntiScalar;
    func weight_norm_squared() -> Output {
MultiVector flat_weight = this.flat_weight();
        return flat_weight.anti_dot_product(flat_weight);
    }
    func operator/(_rhs: WeightNormSquaredInfix) -> AntiScalar {
        return this.weight_norm_squared();
    }
}
extension Origin: WeightNormSquared {
    associatedtype Output = AntiScalar;
    func weight_norm_squared() -> Output {
Origin flat_weight = this.flat_weight();
        return flat_weight.anti_dot_product(flat_weight);
    }
    func operator/(_rhs: WeightNormSquaredInfix) -> AntiScalar {
        return this.weight_norm_squared();
    }
}
extension Plane: WeightNormSquared {
    associatedtype Output = AntiScalar;
    func weight_norm_squared() -> Output {
Plane flat_weight = this.flat_weight();
        return flat_weight.anti_dot_product(flat_weight);
    }
    func operator/(_rhs: WeightNormSquaredInfix) -> AntiScalar {
        return this.weight_norm_squared();
    }
}
extension Point: WeightNormSquared {
    associatedtype Output = AntiScalar;
    func weight_norm_squared() -> Output {
Origin flat_weight = this.flat_weight();
        return flat_weight.anti_dot_product(flat_weight);
    }
    func operator/(_rhs: WeightNormSquaredInfix) -> AntiScalar {
        return this.weight_norm_squared();
    }
}

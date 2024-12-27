using traits::AntiDotProduct;
using traits::FlatWeight;
extension AntiScalar {
    // Fancy infix trick
    func operator/(_rhs: FlatWeightNormSquaredPrefixOrPostfix) -> AntiScalar {
        return this.flat_weight_norm_squared();
    }
    func operator/=(const FlatWeightNormSquaredPrefixOrPostfix& _rhs) -> AntiScalar& {
        this = this.flat_weight_norm_squared();
        return *this;
    }
}
extension AntiScalar: FlatWeightNormSquared {
    associatedtype Output = AntiScalar;
    func flat_weight_norm_squared() -> Output {
AntiScalar flat_weight = this.flat_weight();
        return flat_weight.anti_dot_product(flat_weight);
    }
}
extension DualNum {
    // Fancy infix trick
    func operator/(_rhs: FlatWeightNormSquaredPrefixOrPostfix) -> AntiScalar {
        return this.flat_weight_norm_squared();
    }
}
extension DualNum: FlatWeightNormSquared {
    associatedtype Output = AntiScalar;
    func flat_weight_norm_squared() -> Output {
AntiScalar flat_weight = this.flat_weight();
        return flat_weight.anti_dot_product(flat_weight);
    }
}
extension Flector {
    // Fancy infix trick
    func operator/(_rhs: FlatWeightNormSquaredPrefixOrPostfix) -> AntiScalar {
        return this.flat_weight_norm_squared();
    }
}
extension Flector: FlatWeightNormSquared {
    associatedtype Output = AntiScalar;
    func flat_weight_norm_squared() -> Output {
Flector flat_weight = this.flat_weight();
        return flat_weight.anti_dot_product(flat_weight);
    }
}
extension Line {
    // Fancy infix trick
    func operator/(_rhs: FlatWeightNormSquaredPrefixOrPostfix) -> AntiScalar {
        return this.flat_weight_norm_squared();
    }
}
extension Line: FlatWeightNormSquared {
    associatedtype Output = AntiScalar;
    func flat_weight_norm_squared() -> Output {
Line flat_weight = this.flat_weight();
        return flat_weight.anti_dot_product(flat_weight);
    }
}
extension Motor {
    // Fancy infix trick
    func operator/(_rhs: FlatWeightNormSquaredPrefixOrPostfix) -> AntiScalar {
        return this.flat_weight_norm_squared();
    }
}
extension Motor: FlatWeightNormSquared {
    associatedtype Output = AntiScalar;
    func flat_weight_norm_squared() -> Output {
Motor flat_weight = this.flat_weight();
        return flat_weight.anti_dot_product(flat_weight);
    }
}
extension MultiVector {
    // Fancy infix trick
    func operator/(_rhs: FlatWeightNormSquaredPrefixOrPostfix) -> AntiScalar {
        return this.flat_weight_norm_squared();
    }
}
extension MultiVector: FlatWeightNormSquared {
    associatedtype Output = AntiScalar;
    func flat_weight_norm_squared() -> Output {
MultiVector flat_weight = this.flat_weight();
        return flat_weight.anti_dot_product(flat_weight);
    }
}
extension Origin {
    // Fancy infix trick
    func operator/(_rhs: FlatWeightNormSquaredPrefixOrPostfix) -> AntiScalar {
        return this.flat_weight_norm_squared();
    }
}
extension Origin: FlatWeightNormSquared {
    associatedtype Output = AntiScalar;
    func flat_weight_norm_squared() -> Output {
Origin flat_weight = this.flat_weight();
        return flat_weight.anti_dot_product(flat_weight);
    }
}
extension Plane {
    // Fancy infix trick
    func operator/(_rhs: FlatWeightNormSquaredPrefixOrPostfix) -> AntiScalar {
        return this.flat_weight_norm_squared();
    }
}
extension Plane: FlatWeightNormSquared {
    associatedtype Output = AntiScalar;
    func flat_weight_norm_squared() -> Output {
Plane flat_weight = this.flat_weight();
        return flat_weight.anti_dot_product(flat_weight);
    }
}
extension Point {
    // Fancy infix trick
    func operator/(_rhs: FlatWeightNormSquaredPrefixOrPostfix) -> AntiScalar {
        return this.flat_weight_norm_squared();
    }
}
extension Point: FlatWeightNormSquared {
    associatedtype Output = AntiScalar;
    func flat_weight_norm_squared() -> Output {
Origin flat_weight = this.flat_weight();
        return flat_weight.anti_dot_product(flat_weight);
    }
}

using traits::AntiDotProduct;
using traits::FlatWeight;
extension AntiScalar {
    func div(_rhs: weight_norm_squared) -> 
AntiScalar {
        this.weight_norm_squared()
    }
}
extension AntiScalar {
    func div=(const weight_norm_squared& _rhs) -> AntiScalar& {
        this = this.weight_norm_squared()
        return *this;
    }
}
extension AntiScalar: WeightNormSquared {
    func weight_norm_squared() -> AntiScalar {
AntiScalar flat_weight = this.flat_weight();
        return flat_weight.anti_dot_product(flat_weight);
    }
}
extension DualNum {
    func div(_rhs: weight_norm_squared) -> 
AntiScalar {
        this.weight_norm_squared()
    }
}
extension DualNum: WeightNormSquared {
    func weight_norm_squared() -> AntiScalar {
AntiScalar flat_weight = this.flat_weight();
        return flat_weight.anti_dot_product(flat_weight);
    }
}
extension Flector {
    func div(_rhs: weight_norm_squared) -> 
AntiScalar {
        this.weight_norm_squared()
    }
}
extension Flector: WeightNormSquared {
    func weight_norm_squared() -> AntiScalar {
Flector flat_weight = this.flat_weight();
        return flat_weight.anti_dot_product(flat_weight);
    }
}
extension Line {
    func div(_rhs: weight_norm_squared) -> 
AntiScalar {
        this.weight_norm_squared()
    }
}
extension Line: WeightNormSquared {
    func weight_norm_squared() -> AntiScalar {
Line flat_weight = this.flat_weight();
        return flat_weight.anti_dot_product(flat_weight);
    }
}
extension Motor {
    func div(_rhs: weight_norm_squared) -> 
AntiScalar {
        this.weight_norm_squared()
    }
}
extension Motor: WeightNormSquared {
    func weight_norm_squared() -> AntiScalar {
Motor flat_weight = this.flat_weight();
        return flat_weight.anti_dot_product(flat_weight);
    }
}
extension MultiVector {
    func div(_rhs: weight_norm_squared) -> 
AntiScalar {
        this.weight_norm_squared()
    }
}
extension MultiVector: WeightNormSquared {
    func weight_norm_squared() -> AntiScalar {
MultiVector flat_weight = this.flat_weight();
        return flat_weight.anti_dot_product(flat_weight);
    }
}
extension Origin {
    func div(_rhs: weight_norm_squared) -> 
AntiScalar {
        this.weight_norm_squared()
    }
}
extension Origin: WeightNormSquared {
    func weight_norm_squared() -> AntiScalar {
Origin flat_weight = this.flat_weight();
        return flat_weight.anti_dot_product(flat_weight);
    }
}
extension Plane {
    func div(_rhs: weight_norm_squared) -> 
AntiScalar {
        this.weight_norm_squared()
    }
}
extension Plane: WeightNormSquared {
    func weight_norm_squared() -> AntiScalar {
Plane flat_weight = this.flat_weight();
        return flat_weight.anti_dot_product(flat_weight);
    }
}
extension Point {
    func div(_rhs: weight_norm_squared) -> 
AntiScalar {
        this.weight_norm_squared()
    }
}
extension Point: WeightNormSquared {
    func weight_norm_squared() -> AntiScalar {
Origin flat_weight = this.flat_weight();
        return flat_weight.anti_dot_product(flat_weight);
    }
}

using traits::AntiDotProduct;
using traits::FlatWeight;
extension AntiScalar {
    func div(_rhs: flat_weight_norm_squared) -> 
AntiScalar {
        this.flat_weight_norm_squared()
    }
}
extension AntiScalar {
    func div=(const flat_weight_norm_squared& _rhs) -> AntiScalar& {
        this = this.flat_weight_norm_squared()
        return *this;
    }
}
extension AntiScalar: FlatWeightNormSquared {
    func flat_weight_norm_squared(self) -> AntiScalar {
AntiScalar flat_weight = self.flat_weight();
        return flat_weight.anti_dot_product(flat_weight);
    }
}
extension DualNum {
    func div(_rhs: flat_weight_norm_squared) -> 
AntiScalar {
        this.flat_weight_norm_squared()
    }
}
extension DualNum: FlatWeightNormSquared {
    func flat_weight_norm_squared(self) -> AntiScalar {
AntiScalar flat_weight = self.flat_weight();
        return flat_weight.anti_dot_product(flat_weight);
    }
}
extension Flector {
    func div(_rhs: flat_weight_norm_squared) -> 
AntiScalar {
        this.flat_weight_norm_squared()
    }
}
extension Flector: FlatWeightNormSquared {
    func flat_weight_norm_squared(self) -> AntiScalar {
Flector flat_weight = self.flat_weight();
        return flat_weight.anti_dot_product(flat_weight);
    }
}
extension Line {
    func div(_rhs: flat_weight_norm_squared) -> 
AntiScalar {
        this.flat_weight_norm_squared()
    }
}
extension Line: FlatWeightNormSquared {
    func flat_weight_norm_squared(self) -> AntiScalar {
Line flat_weight = self.flat_weight();
        return flat_weight.anti_dot_product(flat_weight);
    }
}
extension Motor {
    func div(_rhs: flat_weight_norm_squared) -> 
AntiScalar {
        this.flat_weight_norm_squared()
    }
}
extension Motor: FlatWeightNormSquared {
    func flat_weight_norm_squared(self) -> AntiScalar {
Motor flat_weight = self.flat_weight();
        return flat_weight.anti_dot_product(flat_weight);
    }
}
extension MultiVector {
    func div(_rhs: flat_weight_norm_squared) -> 
AntiScalar {
        this.flat_weight_norm_squared()
    }
}
extension MultiVector: FlatWeightNormSquared {
    func flat_weight_norm_squared(self) -> AntiScalar {
MultiVector flat_weight = self.flat_weight();
        return flat_weight.anti_dot_product(flat_weight);
    }
}
extension Origin {
    func div(_rhs: flat_weight_norm_squared) -> 
AntiScalar {
        this.flat_weight_norm_squared()
    }
}
extension Origin: FlatWeightNormSquared {
    func flat_weight_norm_squared(self) -> AntiScalar {
Origin flat_weight = self.flat_weight();
        return flat_weight.anti_dot_product(flat_weight);
    }
}
extension Plane {
    func div(_rhs: flat_weight_norm_squared) -> 
AntiScalar {
        this.flat_weight_norm_squared()
    }
}
extension Plane: FlatWeightNormSquared {
    func flat_weight_norm_squared(self) -> AntiScalar {
Plane flat_weight = self.flat_weight();
        return flat_weight.anti_dot_product(flat_weight);
    }
}
extension Point {
    func div(_rhs: flat_weight_norm_squared) -> 
AntiScalar {
        this.flat_weight_norm_squared()
    }
}
extension Point: FlatWeightNormSquared {
    func flat_weight_norm_squared(self) -> AntiScalar {
Origin flat_weight = self.flat_weight();
        return flat_weight.anti_dot_product(flat_weight);
    }
}

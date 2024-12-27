using traits::AntiSquareRoot;
using traits::FlatWeightNormSquared;
extension AntiScalar {
    func div(_rhs: weight_norm) -> 
AntiScalar {
        this.weight_norm()
    }
}
extension AntiScalar {
    func div=(const weight_norm& _rhs) -> AntiScalar& {
        this = this.weight_norm()
        return *this;
    }
}
extension AntiScalar: WeightNorm {
    func weight_norm() -> AntiScalar {
        return this.flat_weight_norm_squared().anti_square_root();
    }
}
extension DualNum {
    func div(_rhs: weight_norm) -> 
AntiScalar {
        this.weight_norm()
    }
}
extension DualNum: WeightNorm {
    func weight_norm() -> AntiScalar {
        return this.flat_weight_norm_squared().anti_square_root();
    }
}
extension Flector {
    func div(_rhs: weight_norm) -> 
AntiScalar {
        this.weight_norm()
    }
}
extension Flector: WeightNorm {
    func weight_norm() -> AntiScalar {
        return this.flat_weight_norm_squared().anti_square_root();
    }
}
extension Line {
    func div(_rhs: weight_norm) -> 
AntiScalar {
        this.weight_norm()
    }
}
extension Line: WeightNorm {
    func weight_norm() -> AntiScalar {
        return this.flat_weight_norm_squared().anti_square_root();
    }
}
extension Motor {
    func div(_rhs: weight_norm) -> 
AntiScalar {
        this.weight_norm()
    }
}
extension Motor: WeightNorm {
    func weight_norm() -> AntiScalar {
        return this.flat_weight_norm_squared().anti_square_root();
    }
}
extension MultiVector {
    func div(_rhs: weight_norm) -> 
AntiScalar {
        this.weight_norm()
    }
}
extension MultiVector: WeightNorm {
    func weight_norm() -> AntiScalar {
        return this.flat_weight_norm_squared().anti_square_root();
    }
}
extension Origin {
    func div(_rhs: weight_norm) -> 
AntiScalar {
        this.weight_norm()
    }
}
extension Origin: WeightNorm {
    func weight_norm() -> AntiScalar {
        return this.flat_weight_norm_squared().anti_square_root();
    }
}
extension Plane {
    func div(_rhs: weight_norm) -> 
AntiScalar {
        this.weight_norm()
    }
}
extension Plane: WeightNorm {
    func weight_norm() -> AntiScalar {
        return this.flat_weight_norm_squared().anti_square_root();
    }
}
extension Point {
    func div(_rhs: weight_norm) -> 
AntiScalar {
        this.weight_norm()
    }
}
extension Point: WeightNorm {
    func weight_norm() -> AntiScalar {
        return this.flat_weight_norm_squared().anti_square_root();
    }
}

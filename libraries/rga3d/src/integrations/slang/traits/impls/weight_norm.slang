using traits::AntiSquareRoot;
using traits::FlatWeightNormSquared;
extension AntiScalar {
    // Fancy infix trick
    func operator/(_rhs: WeightNormPrefixOrPostfix) -> AntiScalar {
        return this.weight_norm();
    }
    func operator/=(const WeightNormPrefixOrPostfix& _rhs) -> AntiScalar& {
        this = this.weight_norm();
        return *this;
    }
}
extension AntiScalar: WeightNorm {
    associatedtype Output = AntiScalar;
    func weight_norm() -> Output {
        return this.flat_weight_norm_squared().anti_square_root();
    }
}
extension DualNum {
    // Fancy infix trick
    func operator/(_rhs: WeightNormPrefixOrPostfix) -> AntiScalar {
        return this.weight_norm();
    }
}
extension DualNum: WeightNorm {
    associatedtype Output = AntiScalar;
    func weight_norm() -> Output {
        return this.flat_weight_norm_squared().anti_square_root();
    }
}
extension Flector {
    // Fancy infix trick
    func operator/(_rhs: WeightNormPrefixOrPostfix) -> AntiScalar {
        return this.weight_norm();
    }
}
extension Flector: WeightNorm {
    associatedtype Output = AntiScalar;
    func weight_norm() -> Output {
        return this.flat_weight_norm_squared().anti_square_root();
    }
}
extension Line {
    // Fancy infix trick
    func operator/(_rhs: WeightNormPrefixOrPostfix) -> AntiScalar {
        return this.weight_norm();
    }
}
extension Line: WeightNorm {
    associatedtype Output = AntiScalar;
    func weight_norm() -> Output {
        return this.flat_weight_norm_squared().anti_square_root();
    }
}
extension Motor {
    // Fancy infix trick
    func operator/(_rhs: WeightNormPrefixOrPostfix) -> AntiScalar {
        return this.weight_norm();
    }
}
extension Motor: WeightNorm {
    associatedtype Output = AntiScalar;
    func weight_norm() -> Output {
        return this.flat_weight_norm_squared().anti_square_root();
    }
}
extension MultiVector {
    // Fancy infix trick
    func operator/(_rhs: WeightNormPrefixOrPostfix) -> AntiScalar {
        return this.weight_norm();
    }
}
extension MultiVector: WeightNorm {
    associatedtype Output = AntiScalar;
    func weight_norm() -> Output {
        return this.flat_weight_norm_squared().anti_square_root();
    }
}
extension Origin {
    // Fancy infix trick
    func operator/(_rhs: WeightNormPrefixOrPostfix) -> AntiScalar {
        return this.weight_norm();
    }
}
extension Origin: WeightNorm {
    associatedtype Output = AntiScalar;
    func weight_norm() -> Output {
        return this.flat_weight_norm_squared().anti_square_root();
    }
}
extension Plane {
    // Fancy infix trick
    func operator/(_rhs: WeightNormPrefixOrPostfix) -> AntiScalar {
        return this.weight_norm();
    }
}
extension Plane: WeightNorm {
    associatedtype Output = AntiScalar;
    func weight_norm() -> Output {
        return this.flat_weight_norm_squared().anti_square_root();
    }
}
extension Point {
    // Fancy infix trick
    func operator/(_rhs: WeightNormPrefixOrPostfix) -> AntiScalar {
        return this.weight_norm();
    }
}
extension Point: WeightNorm {
    associatedtype Output = AntiScalar;
    func weight_norm() -> Output {
        return this.flat_weight_norm_squared().anti_square_root();
    }
}

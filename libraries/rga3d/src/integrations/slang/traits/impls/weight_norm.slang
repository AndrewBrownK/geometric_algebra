using traits::AntiSquareRoot;
using traits::FlatWeightNormSquared;
extension AntiScalar: WeightNorm {
    associatedtype Output = AntiScalar;
    func weight_norm() -> Output {
        return this.flat_weight_norm_squared().anti_square_root();
    }
    func operator/(_rhs: WeightNormInfix) -> AntiScalar {
        return this.weight_norm();
    }
    func operator/=(const WeightNormInfix& _rhs) -> AntiScalar& {
        this = this.weight_norm();
        return *this;
    }
}
extension DualNum: WeightNorm {
    associatedtype Output = AntiScalar;
    func weight_norm() -> Output {
        return this.flat_weight_norm_squared().anti_square_root();
    }
    func operator/(_rhs: WeightNormInfix) -> AntiScalar {
        return this.weight_norm();
    }
}
extension Flector: WeightNorm {
    associatedtype Output = AntiScalar;
    func weight_norm() -> Output {
        return this.flat_weight_norm_squared().anti_square_root();
    }
    func operator/(_rhs: WeightNormInfix) -> AntiScalar {
        return this.weight_norm();
    }
}
extension Line: WeightNorm {
    associatedtype Output = AntiScalar;
    func weight_norm() -> Output {
        return this.flat_weight_norm_squared().anti_square_root();
    }
    func operator/(_rhs: WeightNormInfix) -> AntiScalar {
        return this.weight_norm();
    }
}
extension Motor: WeightNorm {
    associatedtype Output = AntiScalar;
    func weight_norm() -> Output {
        return this.flat_weight_norm_squared().anti_square_root();
    }
    func operator/(_rhs: WeightNormInfix) -> AntiScalar {
        return this.weight_norm();
    }
}
extension MultiVector: WeightNorm {
    associatedtype Output = AntiScalar;
    func weight_norm() -> Output {
        return this.flat_weight_norm_squared().anti_square_root();
    }
    func operator/(_rhs: WeightNormInfix) -> AntiScalar {
        return this.weight_norm();
    }
}
extension Origin: WeightNorm {
    associatedtype Output = AntiScalar;
    func weight_norm() -> Output {
        return this.flat_weight_norm_squared().anti_square_root();
    }
    func operator/(_rhs: WeightNormInfix) -> AntiScalar {
        return this.weight_norm();
    }
}
extension Plane: WeightNorm {
    associatedtype Output = AntiScalar;
    func weight_norm() -> Output {
        return this.flat_weight_norm_squared().anti_square_root();
    }
    func operator/(_rhs: WeightNormInfix) -> AntiScalar {
        return this.weight_norm();
    }
}
extension Point: WeightNorm {
    associatedtype Output = AntiScalar;
    func weight_norm() -> Output {
        return this.flat_weight_norm_squared().anti_square_root();
    }
    func operator/(_rhs: WeightNormInfix) -> AntiScalar {
        return this.weight_norm();
    }
}

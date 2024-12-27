using traits::AntiSquareRoot;
using traits::FlatWeightNormSquared;
extension Flector: FlatWeightNorm {
    associatedtype Output = AntiScalar;
    func flat_weight_norm() -> Output {
        return this.flat_weight_norm_squared().anti_square_root();
    }
    func operator/(_rhs: FlatWeightNormInfix) -> AntiScalar {
        return this.flat_weight_norm();
    }
}
extension Line: FlatWeightNorm {
    associatedtype Output = AntiScalar;
    func flat_weight_norm() -> Output {
        return this.flat_weight_norm_squared().anti_square_root();
    }
    func operator/(_rhs: FlatWeightNormInfix) -> AntiScalar {
        return this.flat_weight_norm();
    }
}
extension Motor: FlatWeightNorm {
    associatedtype Output = AntiScalar;
    func flat_weight_norm() -> Output {
        return this.flat_weight_norm_squared().anti_square_root();
    }
    func operator/(_rhs: FlatWeightNormInfix) -> AntiScalar {
        return this.flat_weight_norm();
    }
}
extension MultiVector: FlatWeightNorm {
    associatedtype Output = AntiScalar;
    func flat_weight_norm() -> Output {
        return this.flat_weight_norm_squared().anti_square_root();
    }
    func operator/(_rhs: FlatWeightNormInfix) -> AntiScalar {
        return this.flat_weight_norm();
    }
}
extension Point: FlatWeightNorm {
    associatedtype Output = AntiScalar;
    func flat_weight_norm() -> Output {
        return this.flat_weight_norm_squared().anti_square_root();
    }
    func operator/(_rhs: FlatWeightNormInfix) -> AntiScalar {
        return this.flat_weight_norm();
    }
}

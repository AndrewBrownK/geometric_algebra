using traits::AntiSquareRoot;
using traits::RoundWeightNormSquared;
extension AntiCircleRotor {
    // Fancy infix trick
    func operator/(_rhs: RoundWeightNormPrefixOrPostfix) -> AntiScalar {
        return this.round_weight_norm();
    }
}
extension AntiCircleRotor: RoundWeightNorm {
    associatedtype Output = AntiScalar;
    func round_weight_norm() -> Output {
        return this.round_weight_norm_squared().anti_square_root();
    }
}
extension AntiDipoleInversion {
    // Fancy infix trick
    func operator/(_rhs: RoundWeightNormPrefixOrPostfix) -> AntiScalar {
        return this.round_weight_norm();
    }
}
extension AntiDipoleInversion: RoundWeightNorm {
    associatedtype Output = AntiScalar;
    func round_weight_norm() -> Output {
        return this.round_weight_norm_squared().anti_square_root();
    }
}
extension Circle {
    // Fancy infix trick
    func operator/(_rhs: RoundWeightNormPrefixOrPostfix) -> AntiScalar {
        return this.round_weight_norm();
    }
}
extension Circle: RoundWeightNorm {
    associatedtype Output = AntiScalar;
    func round_weight_norm() -> Output {
        return this.round_weight_norm_squared().anti_square_root();
    }
}
extension CircleRotor {
    // Fancy infix trick
    func operator/(_rhs: RoundWeightNormPrefixOrPostfix) -> AntiScalar {
        return this.round_weight_norm();
    }
}
extension CircleRotor: RoundWeightNorm {
    associatedtype Output = AntiScalar;
    func round_weight_norm() -> Output {
        return this.round_weight_norm_squared().anti_square_root();
    }
}
extension Dipole {
    // Fancy infix trick
    func operator/(_rhs: RoundWeightNormPrefixOrPostfix) -> AntiScalar {
        return this.round_weight_norm();
    }
}
extension Dipole: RoundWeightNorm {
    associatedtype Output = AntiScalar;
    func round_weight_norm() -> Output {
        return this.round_weight_norm_squared().anti_square_root();
    }
}
extension DipoleInversion {
    // Fancy infix trick
    func operator/(_rhs: RoundWeightNormPrefixOrPostfix) -> AntiScalar {
        return this.round_weight_norm();
    }
}
extension DipoleInversion: RoundWeightNorm {
    associatedtype Output = AntiScalar;
    func round_weight_norm() -> Output {
        return this.round_weight_norm_squared().anti_square_root();
    }
}
extension MultiVector {
    // Fancy infix trick
    func operator/(_rhs: RoundWeightNormPrefixOrPostfix) -> AntiScalar {
        return this.round_weight_norm();
    }
}
extension MultiVector: RoundWeightNorm {
    associatedtype Output = AntiScalar;
    func round_weight_norm() -> Output {
        return this.round_weight_norm_squared().anti_square_root();
    }
}
extension RoundPoint {
    // Fancy infix trick
    func operator/(_rhs: RoundWeightNormPrefixOrPostfix) -> AntiScalar {
        return this.round_weight_norm();
    }
}
extension RoundPoint: RoundWeightNorm {
    associatedtype Output = AntiScalar;
    func round_weight_norm() -> Output {
        return this.round_weight_norm_squared().anti_square_root();
    }
}
extension Sphere {
    // Fancy infix trick
    func operator/(_rhs: RoundWeightNormPrefixOrPostfix) -> AntiScalar {
        return this.round_weight_norm();
    }
}
extension Sphere: RoundWeightNorm {
    associatedtype Output = AntiScalar;
    func round_weight_norm() -> Output {
        return this.round_weight_norm_squared().anti_square_root();
    }
}
extension VersorEven {
    // Fancy infix trick
    func operator/(_rhs: RoundWeightNormPrefixOrPostfix) -> AntiScalar {
        return this.round_weight_norm();
    }
}
extension VersorEven: RoundWeightNorm {
    associatedtype Output = AntiScalar;
    func round_weight_norm() -> Output {
        return this.round_weight_norm_squared().anti_square_root();
    }
}
extension VersorOdd {
    // Fancy infix trick
    func operator/(_rhs: RoundWeightNormPrefixOrPostfix) -> AntiScalar {
        return this.round_weight_norm();
    }
}
extension VersorOdd: RoundWeightNorm {
    associatedtype Output = AntiScalar;
    func round_weight_norm() -> Output {
        return this.round_weight_norm_squared().anti_square_root();
    }
}

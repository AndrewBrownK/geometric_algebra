using traits::AntiSquareRoot;
using traits::FlatWeightNormSquared;
extension AntiCircleRotor {
    // Fancy infix trick
    func operator/(_rhs: FlatWeightNormPrefixOrPostfix) -> AntiScalar {
        return this.flat_weight_norm();
    }
}
extension AntiCircleRotor: FlatWeightNorm {
    associatedtype Output = AntiScalar;
    func flat_weight_norm() -> Output {
        return this.flat_weight_norm_squared().anti_square_root();
    }
}
extension AntiDipoleInversion {
    // Fancy infix trick
    func operator/(_rhs: FlatWeightNormPrefixOrPostfix) -> AntiScalar {
        return this.flat_weight_norm();
    }
}
extension AntiDipoleInversion: FlatWeightNorm {
    associatedtype Output = AntiScalar;
    func flat_weight_norm() -> Output {
        return this.flat_weight_norm_squared().anti_square_root();
    }
}
extension AntiScalar {
    // Fancy infix trick
    func operator/(_rhs: FlatWeightNormPrefixOrPostfix) -> AntiScalar {
        return this.flat_weight_norm();
    }
    func operator/=(const FlatWeightNormPrefixOrPostfix& _rhs) -> AntiScalar& {
        this = this.flat_weight_norm();
        return *this;
    }
}
extension AntiScalar: FlatWeightNorm {
    associatedtype Output = AntiScalar;
    func flat_weight_norm() -> Output {
        return this.flat_weight_norm_squared().anti_square_root();
    }
}
extension Circle {
    // Fancy infix trick
    func operator/(_rhs: FlatWeightNormPrefixOrPostfix) -> AntiScalar {
        return this.flat_weight_norm();
    }
}
extension Circle: FlatWeightNorm {
    associatedtype Output = AntiScalar;
    func flat_weight_norm() -> Output {
        return this.flat_weight_norm_squared().anti_square_root();
    }
}
extension CircleRotor {
    // Fancy infix trick
    func operator/(_rhs: FlatWeightNormPrefixOrPostfix) -> AntiScalar {
        return this.flat_weight_norm();
    }
}
extension CircleRotor: FlatWeightNorm {
    associatedtype Output = AntiScalar;
    func flat_weight_norm() -> Output {
        return this.flat_weight_norm_squared().anti_square_root();
    }
}
extension Dipole {
    // Fancy infix trick
    func operator/(_rhs: FlatWeightNormPrefixOrPostfix) -> AntiScalar {
        return this.flat_weight_norm();
    }
}
extension Dipole: FlatWeightNorm {
    associatedtype Output = AntiScalar;
    func flat_weight_norm() -> Output {
        return this.flat_weight_norm_squared().anti_square_root();
    }
}
extension DipoleInversion {
    // Fancy infix trick
    func operator/(_rhs: FlatWeightNormPrefixOrPostfix) -> AntiScalar {
        return this.flat_weight_norm();
    }
}
extension DipoleInversion: FlatWeightNorm {
    associatedtype Output = AntiScalar;
    func flat_weight_norm() -> Output {
        return this.flat_weight_norm_squared().anti_square_root();
    }
}
extension DualNum {
    // Fancy infix trick
    func operator/(_rhs: FlatWeightNormPrefixOrPostfix) -> AntiScalar {
        return this.flat_weight_norm();
    }
}
extension DualNum: FlatWeightNorm {
    associatedtype Output = AntiScalar;
    func flat_weight_norm() -> Output {
        return this.flat_weight_norm_squared().anti_square_root();
    }
}
extension FlatPoint {
    // Fancy infix trick
    func operator/(_rhs: FlatWeightNormPrefixOrPostfix) -> AntiScalar {
        return this.flat_weight_norm();
    }
}
extension FlatPoint: FlatWeightNorm {
    associatedtype Output = AntiScalar;
    func flat_weight_norm() -> Output {
        return this.flat_weight_norm_squared().anti_square_root();
    }
}
extension Flector {
    // Fancy infix trick
    func operator/(_rhs: FlatWeightNormPrefixOrPostfix) -> AntiScalar {
        return this.flat_weight_norm();
    }
}
extension Flector: FlatWeightNorm {
    associatedtype Output = AntiScalar;
    func flat_weight_norm() -> Output {
        return this.flat_weight_norm_squared().anti_square_root();
    }
}
extension Line {
    // Fancy infix trick
    func operator/(_rhs: FlatWeightNormPrefixOrPostfix) -> AntiScalar {
        return this.flat_weight_norm();
    }
}
extension Line: FlatWeightNorm {
    associatedtype Output = AntiScalar;
    func flat_weight_norm() -> Output {
        return this.flat_weight_norm_squared().anti_square_root();
    }
}
extension Motor {
    // Fancy infix trick
    func operator/(_rhs: FlatWeightNormPrefixOrPostfix) -> AntiScalar {
        return this.flat_weight_norm();
    }
}
extension Motor: FlatWeightNorm {
    associatedtype Output = AntiScalar;
    func flat_weight_norm() -> Output {
        return this.flat_weight_norm_squared().anti_square_root();
    }
}
extension MultiVector {
    // Fancy infix trick
    func operator/(_rhs: FlatWeightNormPrefixOrPostfix) -> AntiScalar {
        return this.flat_weight_norm();
    }
}
extension MultiVector: FlatWeightNorm {
    associatedtype Output = AntiScalar;
    func flat_weight_norm() -> Output {
        return this.flat_weight_norm_squared().anti_square_root();
    }
}
extension Plane {
    // Fancy infix trick
    func operator/(_rhs: FlatWeightNormPrefixOrPostfix) -> AntiScalar {
        return this.flat_weight_norm();
    }
}
extension Plane: FlatWeightNorm {
    associatedtype Output = AntiScalar;
    func flat_weight_norm() -> Output {
        return this.flat_weight_norm_squared().anti_square_root();
    }
}
extension Sphere {
    // Fancy infix trick
    func operator/(_rhs: FlatWeightNormPrefixOrPostfix) -> AntiScalar {
        return this.flat_weight_norm();
    }
}
extension Sphere: FlatWeightNorm {
    associatedtype Output = AntiScalar;
    func flat_weight_norm() -> Output {
        return this.flat_weight_norm_squared().anti_square_root();
    }
}
extension VersorEven {
    // Fancy infix trick
    func operator/(_rhs: FlatWeightNormPrefixOrPostfix) -> AntiScalar {
        return this.flat_weight_norm();
    }
}
extension VersorEven: FlatWeightNorm {
    associatedtype Output = AntiScalar;
    func flat_weight_norm() -> Output {
        return this.flat_weight_norm_squared().anti_square_root();
    }
}
extension VersorOdd {
    // Fancy infix trick
    func operator/(_rhs: FlatWeightNormPrefixOrPostfix) -> AntiScalar {
        return this.flat_weight_norm();
    }
}
extension VersorOdd: FlatWeightNorm {
    associatedtype Output = AntiScalar;
    func flat_weight_norm() -> Output {
        return this.flat_weight_norm_squared().anti_square_root();
    }
}

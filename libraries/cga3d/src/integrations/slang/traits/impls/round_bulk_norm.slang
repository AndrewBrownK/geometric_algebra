using traits::RoundBulkNormSquared;
using traits::SquareRoot;
extension AntiCircleRotor {
    // Fancy infix trick
    func operator/(_rhs: RoundBulkNormPrefixOrPostfix) -> Scalar {
        return this.round_bulk_norm();
    }
}
extension AntiCircleRotor: RoundBulkNorm {
    associatedtype Output = Scalar;
    func round_bulk_norm() -> Output {
        return this.round_bulk_norm_squared().square_root();
    }
}
extension AntiDipoleInversion {
    // Fancy infix trick
    func operator/(_rhs: RoundBulkNormPrefixOrPostfix) -> Scalar {
        return this.round_bulk_norm();
    }
}
extension AntiDipoleInversion: RoundBulkNorm {
    associatedtype Output = Scalar;
    func round_bulk_norm() -> Output {
        return this.round_bulk_norm_squared().square_root();
    }
}
extension AntiDualNum {
    // Fancy infix trick
    func operator/(_rhs: RoundBulkNormPrefixOrPostfix) -> Scalar {
        return this.round_bulk_norm();
    }
}
extension AntiDualNum: RoundBulkNorm {
    associatedtype Output = Scalar;
    func round_bulk_norm() -> Output {
        return this.round_bulk_norm_squared().square_root();
    }
}
extension AntiFlatPoint {
    // Fancy infix trick
    func operator/(_rhs: RoundBulkNormPrefixOrPostfix) -> Scalar {
        return this.round_bulk_norm();
    }
}
extension AntiFlatPoint: RoundBulkNorm {
    associatedtype Output = Scalar;
    func round_bulk_norm() -> Output {
        return this.round_bulk_norm_squared().square_root();
    }
}
extension AntiFlector {
    // Fancy infix trick
    func operator/(_rhs: RoundBulkNormPrefixOrPostfix) -> Scalar {
        return this.round_bulk_norm();
    }
}
extension AntiFlector: RoundBulkNorm {
    associatedtype Output = Scalar;
    func round_bulk_norm() -> Output {
        return this.round_bulk_norm_squared().square_root();
    }
}
extension AntiLine {
    // Fancy infix trick
    func operator/(_rhs: RoundBulkNormPrefixOrPostfix) -> Scalar {
        return this.round_bulk_norm();
    }
}
extension AntiLine: RoundBulkNorm {
    associatedtype Output = Scalar;
    func round_bulk_norm() -> Output {
        return this.round_bulk_norm_squared().square_root();
    }
}
extension AntiMotor {
    // Fancy infix trick
    func operator/(_rhs: RoundBulkNormPrefixOrPostfix) -> Scalar {
        return this.round_bulk_norm();
    }
}
extension AntiMotor: RoundBulkNorm {
    associatedtype Output = Scalar;
    func round_bulk_norm() -> Output {
        return this.round_bulk_norm_squared().square_root();
    }
}
extension AntiPlane {
    // Fancy infix trick
    func operator/(_rhs: RoundBulkNormPrefixOrPostfix) -> Scalar {
        return this.round_bulk_norm();
    }
}
extension AntiPlane: RoundBulkNorm {
    associatedtype Output = Scalar;
    func round_bulk_norm() -> Output {
        return this.round_bulk_norm_squared().square_root();
    }
}
extension Circle {
    // Fancy infix trick
    func operator/(_rhs: RoundBulkNormPrefixOrPostfix) -> Scalar {
        return this.round_bulk_norm();
    }
}
extension Circle: RoundBulkNorm {
    associatedtype Output = Scalar;
    func round_bulk_norm() -> Output {
        return this.round_bulk_norm_squared().square_root();
    }
}
extension CircleRotor {
    // Fancy infix trick
    func operator/(_rhs: RoundBulkNormPrefixOrPostfix) -> Scalar {
        return this.round_bulk_norm();
    }
}
extension CircleRotor: RoundBulkNorm {
    associatedtype Output = Scalar;
    func round_bulk_norm() -> Output {
        return this.round_bulk_norm_squared().square_root();
    }
}
extension Dipole {
    // Fancy infix trick
    func operator/(_rhs: RoundBulkNormPrefixOrPostfix) -> Scalar {
        return this.round_bulk_norm();
    }
}
extension Dipole: RoundBulkNorm {
    associatedtype Output = Scalar;
    func round_bulk_norm() -> Output {
        return this.round_bulk_norm_squared().square_root();
    }
}
extension DipoleInversion {
    // Fancy infix trick
    func operator/(_rhs: RoundBulkNormPrefixOrPostfix) -> Scalar {
        return this.round_bulk_norm();
    }
}
extension DipoleInversion: RoundBulkNorm {
    associatedtype Output = Scalar;
    func round_bulk_norm() -> Output {
        return this.round_bulk_norm_squared().square_root();
    }
}
extension MultiVector {
    // Fancy infix trick
    func operator/(_rhs: RoundBulkNormPrefixOrPostfix) -> Scalar {
        return this.round_bulk_norm();
    }
}
extension MultiVector: RoundBulkNorm {
    associatedtype Output = Scalar;
    func round_bulk_norm() -> Output {
        return this.round_bulk_norm_squared().square_root();
    }
}
extension RoundPoint {
    // Fancy infix trick
    func operator/(_rhs: RoundBulkNormPrefixOrPostfix) -> Scalar {
        return this.round_bulk_norm();
    }
}
extension RoundPoint: RoundBulkNorm {
    associatedtype Output = Scalar;
    func round_bulk_norm() -> Output {
        return this.round_bulk_norm_squared().square_root();
    }
}
extension Scalar {
    // Fancy infix trick
    func operator/(_rhs: RoundBulkNormPrefixOrPostfix) -> Scalar {
        return this.round_bulk_norm();
    }
    func operator/=(const RoundBulkNormPrefixOrPostfix& _rhs) -> Scalar& {
        this = this.round_bulk_norm();
        return *this;
    }
}
extension Scalar: RoundBulkNorm {
    associatedtype Output = Scalar;
    func round_bulk_norm() -> Output {
        return this.round_bulk_norm_squared().square_root();
    }
}
extension VersorEven {
    // Fancy infix trick
    func operator/(_rhs: RoundBulkNormPrefixOrPostfix) -> Scalar {
        return this.round_bulk_norm();
    }
}
extension VersorEven: RoundBulkNorm {
    associatedtype Output = Scalar;
    func round_bulk_norm() -> Output {
        return this.round_bulk_norm_squared().square_root();
    }
}
extension VersorOdd {
    // Fancy infix trick
    func operator/(_rhs: RoundBulkNormPrefixOrPostfix) -> Scalar {
        return this.round_bulk_norm();
    }
}
extension VersorOdd: RoundBulkNorm {
    associatedtype Output = Scalar;
    func round_bulk_norm() -> Output {
        return this.round_bulk_norm_squared().square_root();
    }
}

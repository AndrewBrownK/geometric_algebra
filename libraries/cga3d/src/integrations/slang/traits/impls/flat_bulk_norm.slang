using traits::FlatBulkNormSquared;
using traits::SquareRoot;
extension AntiCircleRotor {
    // Fancy infix trick
    func operator/(_rhs: FlatBulkNormPrefixOrPostfix) -> Scalar {
        return this.flat_bulk_norm();
    }
}
extension AntiCircleRotor: FlatBulkNorm {
    associatedtype Output = Scalar;
    func flat_bulk_norm() -> Output {
        return this.flat_bulk_norm_squared().square_root();
    }
}
extension AntiDipoleInversion {
    // Fancy infix trick
    func operator/(_rhs: FlatBulkNormPrefixOrPostfix) -> Scalar {
        return this.flat_bulk_norm();
    }
}
extension AntiDipoleInversion: FlatBulkNorm {
    associatedtype Output = Scalar;
    func flat_bulk_norm() -> Output {
        return this.flat_bulk_norm_squared().square_root();
    }
}
extension AntiDualNum {
    // Fancy infix trick
    func operator/(_rhs: FlatBulkNormPrefixOrPostfix) -> Scalar {
        return this.flat_bulk_norm();
    }
}
extension AntiDualNum: FlatBulkNorm {
    associatedtype Output = Scalar;
    func flat_bulk_norm() -> Output {
        return this.flat_bulk_norm_squared().square_root();
    }
}
extension AntiFlatPoint {
    // Fancy infix trick
    func operator/(_rhs: FlatBulkNormPrefixOrPostfix) -> Scalar {
        return this.flat_bulk_norm();
    }
}
extension AntiFlatPoint: FlatBulkNorm {
    associatedtype Output = Scalar;
    func flat_bulk_norm() -> Output {
        return this.flat_bulk_norm_squared().square_root();
    }
}
extension AntiFlector {
    // Fancy infix trick
    func operator/(_rhs: FlatBulkNormPrefixOrPostfix) -> Scalar {
        return this.flat_bulk_norm();
    }
}
extension AntiFlector: FlatBulkNorm {
    associatedtype Output = Scalar;
    func flat_bulk_norm() -> Output {
        return this.flat_bulk_norm_squared().square_root();
    }
}
extension AntiLine {
    // Fancy infix trick
    func operator/(_rhs: FlatBulkNormPrefixOrPostfix) -> Scalar {
        return this.flat_bulk_norm();
    }
}
extension AntiLine: FlatBulkNorm {
    associatedtype Output = Scalar;
    func flat_bulk_norm() -> Output {
        return this.flat_bulk_norm_squared().square_root();
    }
}
extension AntiMotor {
    // Fancy infix trick
    func operator/(_rhs: FlatBulkNormPrefixOrPostfix) -> Scalar {
        return this.flat_bulk_norm();
    }
}
extension AntiMotor: FlatBulkNorm {
    associatedtype Output = Scalar;
    func flat_bulk_norm() -> Output {
        return this.flat_bulk_norm_squared().square_root();
    }
}
extension AntiPlane {
    // Fancy infix trick
    func operator/(_rhs: FlatBulkNormPrefixOrPostfix) -> Scalar {
        return this.flat_bulk_norm();
    }
}
extension AntiPlane: FlatBulkNorm {
    associatedtype Output = Scalar;
    func flat_bulk_norm() -> Output {
        return this.flat_bulk_norm_squared().square_root();
    }
}
extension Circle {
    // Fancy infix trick
    func operator/(_rhs: FlatBulkNormPrefixOrPostfix) -> Scalar {
        return this.flat_bulk_norm();
    }
}
extension Circle: FlatBulkNorm {
    associatedtype Output = Scalar;
    func flat_bulk_norm() -> Output {
        return this.flat_bulk_norm_squared().square_root();
    }
}
extension CircleRotor {
    // Fancy infix trick
    func operator/(_rhs: FlatBulkNormPrefixOrPostfix) -> Scalar {
        return this.flat_bulk_norm();
    }
}
extension CircleRotor: FlatBulkNorm {
    associatedtype Output = Scalar;
    func flat_bulk_norm() -> Output {
        return this.flat_bulk_norm_squared().square_root();
    }
}
extension Dipole {
    // Fancy infix trick
    func operator/(_rhs: FlatBulkNormPrefixOrPostfix) -> Scalar {
        return this.flat_bulk_norm();
    }
}
extension Dipole: FlatBulkNorm {
    associatedtype Output = Scalar;
    func flat_bulk_norm() -> Output {
        return this.flat_bulk_norm_squared().square_root();
    }
}
extension DipoleInversion {
    // Fancy infix trick
    func operator/(_rhs: FlatBulkNormPrefixOrPostfix) -> Scalar {
        return this.flat_bulk_norm();
    }
}
extension DipoleInversion: FlatBulkNorm {
    associatedtype Output = Scalar;
    func flat_bulk_norm() -> Output {
        return this.flat_bulk_norm_squared().square_root();
    }
}
extension DualNum {
    // Fancy infix trick
    func operator/(_rhs: FlatBulkNormPrefixOrPostfix) -> Scalar {
        return this.flat_bulk_norm();
    }
}
extension DualNum: FlatBulkNorm {
    associatedtype Output = Scalar;
    func flat_bulk_norm() -> Output {
        return this.flat_bulk_norm_squared().square_root();
    }
}
extension FlatPoint {
    // Fancy infix trick
    func operator/(_rhs: FlatBulkNormPrefixOrPostfix) -> Scalar {
        return this.flat_bulk_norm();
    }
}
extension FlatPoint: FlatBulkNorm {
    associatedtype Output = Scalar;
    func flat_bulk_norm() -> Output {
        return this.flat_bulk_norm_squared().square_root();
    }
}
extension Flector {
    // Fancy infix trick
    func operator/(_rhs: FlatBulkNormPrefixOrPostfix) -> Scalar {
        return this.flat_bulk_norm();
    }
}
extension Flector: FlatBulkNorm {
    associatedtype Output = Scalar;
    func flat_bulk_norm() -> Output {
        return this.flat_bulk_norm_squared().square_root();
    }
}
extension Line {
    // Fancy infix trick
    func operator/(_rhs: FlatBulkNormPrefixOrPostfix) -> Scalar {
        return this.flat_bulk_norm();
    }
}
extension Line: FlatBulkNorm {
    associatedtype Output = Scalar;
    func flat_bulk_norm() -> Output {
        return this.flat_bulk_norm_squared().square_root();
    }
}
extension Motor {
    // Fancy infix trick
    func operator/(_rhs: FlatBulkNormPrefixOrPostfix) -> Scalar {
        return this.flat_bulk_norm();
    }
}
extension Motor: FlatBulkNorm {
    associatedtype Output = Scalar;
    func flat_bulk_norm() -> Output {
        return this.flat_bulk_norm_squared().square_root();
    }
}
extension MultiVector {
    // Fancy infix trick
    func operator/(_rhs: FlatBulkNormPrefixOrPostfix) -> Scalar {
        return this.flat_bulk_norm();
    }
}
extension MultiVector: FlatBulkNorm {
    associatedtype Output = Scalar;
    func flat_bulk_norm() -> Output {
        return this.flat_bulk_norm_squared().square_root();
    }
}
extension Plane {
    // Fancy infix trick
    func operator/(_rhs: FlatBulkNormPrefixOrPostfix) -> Scalar {
        return this.flat_bulk_norm();
    }
}
extension Plane: FlatBulkNorm {
    associatedtype Output = Scalar;
    func flat_bulk_norm() -> Output {
        return this.flat_bulk_norm_squared().square_root();
    }
}
extension RoundPoint {
    // Fancy infix trick
    func operator/(_rhs: FlatBulkNormPrefixOrPostfix) -> Scalar {
        return this.flat_bulk_norm();
    }
}
extension RoundPoint: FlatBulkNorm {
    associatedtype Output = Scalar;
    func flat_bulk_norm() -> Output {
        return this.flat_bulk_norm_squared().square_root();
    }
}
extension Sphere {
    // Fancy infix trick
    func operator/(_rhs: FlatBulkNormPrefixOrPostfix) -> Scalar {
        return this.flat_bulk_norm();
    }
}
extension Sphere: FlatBulkNorm {
    associatedtype Output = Scalar;
    func flat_bulk_norm() -> Output {
        return this.flat_bulk_norm_squared().square_root();
    }
}
extension VersorEven {
    // Fancy infix trick
    func operator/(_rhs: FlatBulkNormPrefixOrPostfix) -> Scalar {
        return this.flat_bulk_norm();
    }
}
extension VersorEven: FlatBulkNorm {
    associatedtype Output = Scalar;
    func flat_bulk_norm() -> Output {
        return this.flat_bulk_norm_squared().square_root();
    }
}
extension VersorOdd {
    // Fancy infix trick
    func operator/(_rhs: FlatBulkNormPrefixOrPostfix) -> Scalar {
        return this.flat_bulk_norm();
    }
}
extension VersorOdd: FlatBulkNorm {
    associatedtype Output = Scalar;
    func flat_bulk_norm() -> Output {
        return this.flat_bulk_norm_squared().square_root();
    }
}

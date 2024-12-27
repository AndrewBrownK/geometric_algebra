using traits::RadiusNormSquared;
using traits::SquareRoot;
extension AntiCircleRotor {
    // Fancy infix trick
    func operator/(_rhs: RadiusNormPrefixOrPostfix) -> Scalar {
        return this.radius_norm();
    }
}
extension AntiCircleRotor: RadiusNorm {
    associatedtype Output = Scalar;
    func radius_norm() -> Output {
        return this.radius_norm_squared().square_root();
    }
}
extension AntiDipoleInversion {
    // Fancy infix trick
    func operator/(_rhs: RadiusNormPrefixOrPostfix) -> Scalar {
        return this.radius_norm();
    }
}
extension AntiDipoleInversion: RadiusNorm {
    associatedtype Output = Scalar;
    func radius_norm() -> Output {
        return this.radius_norm_squared().square_root();
    }
}
extension AntiDualNum {
    // Fancy infix trick
    func operator/(_rhs: RadiusNormPrefixOrPostfix) -> Scalar {
        return this.radius_norm();
    }
}
extension AntiDualNum: RadiusNorm {
    associatedtype Output = Scalar;
    func radius_norm() -> Output {
        return this.radius_norm_squared().square_root();
    }
}
extension AntiFlatPoint {
    // Fancy infix trick
    func operator/(_rhs: RadiusNormPrefixOrPostfix) -> Scalar {
        return this.radius_norm();
    }
}
extension AntiFlatPoint: RadiusNorm {
    associatedtype Output = Scalar;
    func radius_norm() -> Output {
        return this.radius_norm_squared().square_root();
    }
}
extension AntiFlector {
    // Fancy infix trick
    func operator/(_rhs: RadiusNormPrefixOrPostfix) -> Scalar {
        return this.radius_norm();
    }
}
extension AntiFlector: RadiusNorm {
    associatedtype Output = Scalar;
    func radius_norm() -> Output {
        return this.radius_norm_squared().square_root();
    }
}
extension AntiLine {
    // Fancy infix trick
    func operator/(_rhs: RadiusNormPrefixOrPostfix) -> Scalar {
        return this.radius_norm();
    }
}
extension AntiLine: RadiusNorm {
    associatedtype Output = Scalar;
    func radius_norm() -> Output {
        return this.radius_norm_squared().square_root();
    }
}
extension AntiMotor {
    // Fancy infix trick
    func operator/(_rhs: RadiusNormPrefixOrPostfix) -> Scalar {
        return this.radius_norm();
    }
}
extension AntiMotor: RadiusNorm {
    associatedtype Output = Scalar;
    func radius_norm() -> Output {
        return this.radius_norm_squared().square_root();
    }
}
extension AntiPlane {
    // Fancy infix trick
    func operator/(_rhs: RadiusNormPrefixOrPostfix) -> Scalar {
        return this.radius_norm();
    }
}
extension AntiPlane: RadiusNorm {
    associatedtype Output = Scalar;
    func radius_norm() -> Output {
        return this.radius_norm_squared().square_root();
    }
}
extension AntiScalar {
    // Fancy infix trick
    func operator/(_rhs: RadiusNormPrefixOrPostfix) -> Scalar {
        return this.radius_norm();
    }
}
extension AntiScalar: RadiusNorm {
    associatedtype Output = Scalar;
    func radius_norm() -> Output {
        return this.radius_norm_squared().square_root();
    }
}
extension Circle {
    // Fancy infix trick
    func operator/(_rhs: RadiusNormPrefixOrPostfix) -> Scalar {
        return this.radius_norm();
    }
}
extension Circle: RadiusNorm {
    associatedtype Output = Scalar;
    func radius_norm() -> Output {
        return this.radius_norm_squared().square_root();
    }
}
extension CircleRotor {
    // Fancy infix trick
    func operator/(_rhs: RadiusNormPrefixOrPostfix) -> Scalar {
        return this.radius_norm();
    }
}
extension CircleRotor: RadiusNorm {
    associatedtype Output = Scalar;
    func radius_norm() -> Output {
        return this.radius_norm_squared().square_root();
    }
}
extension Dipole {
    // Fancy infix trick
    func operator/(_rhs: RadiusNormPrefixOrPostfix) -> Scalar {
        return this.radius_norm();
    }
}
extension Dipole: RadiusNorm {
    associatedtype Output = Scalar;
    func radius_norm() -> Output {
        return this.radius_norm_squared().square_root();
    }
}
extension DipoleInversion {
    // Fancy infix trick
    func operator/(_rhs: RadiusNormPrefixOrPostfix) -> Scalar {
        return this.radius_norm();
    }
}
extension DipoleInversion: RadiusNorm {
    associatedtype Output = Scalar;
    func radius_norm() -> Output {
        return this.radius_norm_squared().square_root();
    }
}
extension DualNum {
    // Fancy infix trick
    func operator/(_rhs: RadiusNormPrefixOrPostfix) -> Scalar {
        return this.radius_norm();
    }
}
extension DualNum: RadiusNorm {
    associatedtype Output = Scalar;
    func radius_norm() -> Output {
        return this.radius_norm_squared().square_root();
    }
}
extension FlatPoint {
    // Fancy infix trick
    func operator/(_rhs: RadiusNormPrefixOrPostfix) -> Scalar {
        return this.radius_norm();
    }
}
extension FlatPoint: RadiusNorm {
    associatedtype Output = Scalar;
    func radius_norm() -> Output {
        return this.radius_norm_squared().square_root();
    }
}
extension Flector {
    // Fancy infix trick
    func operator/(_rhs: RadiusNormPrefixOrPostfix) -> Scalar {
        return this.radius_norm();
    }
}
extension Flector: RadiusNorm {
    associatedtype Output = Scalar;
    func radius_norm() -> Output {
        return this.radius_norm_squared().square_root();
    }
}
extension Line {
    // Fancy infix trick
    func operator/(_rhs: RadiusNormPrefixOrPostfix) -> Scalar {
        return this.radius_norm();
    }
}
extension Line: RadiusNorm {
    associatedtype Output = Scalar;
    func radius_norm() -> Output {
        return this.radius_norm_squared().square_root();
    }
}
extension Motor {
    // Fancy infix trick
    func operator/(_rhs: RadiusNormPrefixOrPostfix) -> Scalar {
        return this.radius_norm();
    }
}
extension Motor: RadiusNorm {
    associatedtype Output = Scalar;
    func radius_norm() -> Output {
        return this.radius_norm_squared().square_root();
    }
}
extension MultiVector {
    // Fancy infix trick
    func operator/(_rhs: RadiusNormPrefixOrPostfix) -> Scalar {
        return this.radius_norm();
    }
}
extension MultiVector: RadiusNorm {
    associatedtype Output = Scalar;
    func radius_norm() -> Output {
        return this.radius_norm_squared().square_root();
    }
}
extension Plane {
    // Fancy infix trick
    func operator/(_rhs: RadiusNormPrefixOrPostfix) -> Scalar {
        return this.radius_norm();
    }
}
extension Plane: RadiusNorm {
    associatedtype Output = Scalar;
    func radius_norm() -> Output {
        return this.radius_norm_squared().square_root();
    }
}
extension RoundPoint {
    // Fancy infix trick
    func operator/(_rhs: RadiusNormPrefixOrPostfix) -> Scalar {
        return this.radius_norm();
    }
}
extension RoundPoint: RadiusNorm {
    associatedtype Output = Scalar;
    func radius_norm() -> Output {
        return this.radius_norm_squared().square_root();
    }
}
extension Scalar {
    // Fancy infix trick
    func operator/(_rhs: RadiusNormPrefixOrPostfix) -> Scalar {
        return this.radius_norm();
    }
    func operator/=(const RadiusNormPrefixOrPostfix& _rhs) -> Scalar& {
        this = this.radius_norm();
        return *this;
    }
}
extension Scalar: RadiusNorm {
    associatedtype Output = Scalar;
    func radius_norm() -> Output {
        return this.radius_norm_squared().square_root();
    }
}
extension Sphere {
    // Fancy infix trick
    func operator/(_rhs: RadiusNormPrefixOrPostfix) -> Scalar {
        return this.radius_norm();
    }
}
extension Sphere: RadiusNorm {
    associatedtype Output = Scalar;
    func radius_norm() -> Output {
        return this.radius_norm_squared().square_root();
    }
}
extension VersorEven {
    // Fancy infix trick
    func operator/(_rhs: RadiusNormPrefixOrPostfix) -> Scalar {
        return this.radius_norm();
    }
}
extension VersorEven: RadiusNorm {
    associatedtype Output = Scalar;
    func radius_norm() -> Output {
        return this.radius_norm_squared().square_root();
    }
}
extension VersorOdd {
    // Fancy infix trick
    func operator/(_rhs: RadiusNormPrefixOrPostfix) -> Scalar {
        return this.radius_norm();
    }
}
extension VersorOdd: RadiusNorm {
    associatedtype Output = Scalar;
    func radius_norm() -> Output {
        return this.radius_norm_squared().square_root();
    }
}

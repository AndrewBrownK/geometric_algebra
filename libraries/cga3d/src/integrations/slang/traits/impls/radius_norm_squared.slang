using traits::AntiDotProduct;
using traits::RightAntiDual;
extension AntiCircleRotor {
    // Fancy infix trick
    func operator/(_rhs: RadiusNormSquaredPrefixOrPostfix) -> Scalar {
        return this.radius_norm_squared();
    }
}
extension AntiCircleRotor: RadiusNormSquared {
    associatedtype Output = Scalar;
    func radius_norm_squared() -> Output {
        return this.anti_dot_product(this).right_anti_dual();
    }
}
extension AntiDipoleInversion {
    // Fancy infix trick
    func operator/(_rhs: RadiusNormSquaredPrefixOrPostfix) -> Scalar {
        return this.radius_norm_squared();
    }
}
extension AntiDipoleInversion: RadiusNormSquared {
    associatedtype Output = Scalar;
    func radius_norm_squared() -> Output {
        return this.anti_dot_product(this).right_anti_dual();
    }
}
extension AntiDualNum {
    // Fancy infix trick
    func operator/(_rhs: RadiusNormSquaredPrefixOrPostfix) -> Scalar {
        return this.radius_norm_squared();
    }
}
extension AntiDualNum: RadiusNormSquared {
    associatedtype Output = Scalar;
    func radius_norm_squared() -> Output {
        return this.anti_dot_product(this).right_anti_dual();
    }
}
extension AntiFlatPoint {
    // Fancy infix trick
    func operator/(_rhs: RadiusNormSquaredPrefixOrPostfix) -> Scalar {
        return this.radius_norm_squared();
    }
}
extension AntiFlatPoint: RadiusNormSquared {
    associatedtype Output = Scalar;
    func radius_norm_squared() -> Output {
        return this.anti_dot_product(this).right_anti_dual();
    }
}
extension AntiFlector {
    // Fancy infix trick
    func operator/(_rhs: RadiusNormSquaredPrefixOrPostfix) -> Scalar {
        return this.radius_norm_squared();
    }
}
extension AntiFlector: RadiusNormSquared {
    associatedtype Output = Scalar;
    func radius_norm_squared() -> Output {
        return this.anti_dot_product(this).right_anti_dual();
    }
}
extension AntiLine {
    // Fancy infix trick
    func operator/(_rhs: RadiusNormSquaredPrefixOrPostfix) -> Scalar {
        return this.radius_norm_squared();
    }
}
extension AntiLine: RadiusNormSquared {
    associatedtype Output = Scalar;
    func radius_norm_squared() -> Output {
        return this.anti_dot_product(this).right_anti_dual();
    }
}
extension AntiMotor {
    // Fancy infix trick
    func operator/(_rhs: RadiusNormSquaredPrefixOrPostfix) -> Scalar {
        return this.radius_norm_squared();
    }
}
extension AntiMotor: RadiusNormSquared {
    associatedtype Output = Scalar;
    func radius_norm_squared() -> Output {
        return this.anti_dot_product(this).right_anti_dual();
    }
}
extension AntiPlane {
    // Fancy infix trick
    func operator/(_rhs: RadiusNormSquaredPrefixOrPostfix) -> Scalar {
        return this.radius_norm_squared();
    }
}
extension AntiPlane: RadiusNormSquared {
    associatedtype Output = Scalar;
    func radius_norm_squared() -> Output {
        return this.anti_dot_product(this).right_anti_dual();
    }
}
extension AntiScalar {
    // Fancy infix trick
    func operator/(_rhs: RadiusNormSquaredPrefixOrPostfix) -> Scalar {
        return this.radius_norm_squared();
    }
}
extension AntiScalar: RadiusNormSquared {
    associatedtype Output = Scalar;
    func radius_norm_squared() -> Output {
        return this.anti_dot_product(this).right_anti_dual();
    }
}
extension Circle {
    // Fancy infix trick
    func operator/(_rhs: RadiusNormSquaredPrefixOrPostfix) -> Scalar {
        return this.radius_norm_squared();
    }
}
extension Circle: RadiusNormSquared {
    associatedtype Output = Scalar;
    func radius_norm_squared() -> Output {
        return this.anti_dot_product(this).right_anti_dual();
    }
}
extension CircleRotor {
    // Fancy infix trick
    func operator/(_rhs: RadiusNormSquaredPrefixOrPostfix) -> Scalar {
        return this.radius_norm_squared();
    }
}
extension CircleRotor: RadiusNormSquared {
    associatedtype Output = Scalar;
    func radius_norm_squared() -> Output {
        return this.anti_dot_product(this).right_anti_dual();
    }
}
extension Dipole {
    // Fancy infix trick
    func operator/(_rhs: RadiusNormSquaredPrefixOrPostfix) -> Scalar {
        return this.radius_norm_squared();
    }
}
extension Dipole: RadiusNormSquared {
    associatedtype Output = Scalar;
    func radius_norm_squared() -> Output {
        return this.anti_dot_product(this).right_anti_dual();
    }
}
extension DipoleInversion {
    // Fancy infix trick
    func operator/(_rhs: RadiusNormSquaredPrefixOrPostfix) -> Scalar {
        return this.radius_norm_squared();
    }
}
extension DipoleInversion: RadiusNormSquared {
    associatedtype Output = Scalar;
    func radius_norm_squared() -> Output {
        return this.anti_dot_product(this).right_anti_dual();
    }
}
extension DualNum {
    // Fancy infix trick
    func operator/(_rhs: RadiusNormSquaredPrefixOrPostfix) -> Scalar {
        return this.radius_norm_squared();
    }
}
extension DualNum: RadiusNormSquared {
    associatedtype Output = Scalar;
    func radius_norm_squared() -> Output {
        return this.anti_dot_product(this).right_anti_dual();
    }
}
extension FlatPoint {
    // Fancy infix trick
    func operator/(_rhs: RadiusNormSquaredPrefixOrPostfix) -> Scalar {
        return this.radius_norm_squared();
    }
}
extension FlatPoint: RadiusNormSquared {
    associatedtype Output = Scalar;
    func radius_norm_squared() -> Output {
        return this.anti_dot_product(this).right_anti_dual();
    }
}
extension Flector {
    // Fancy infix trick
    func operator/(_rhs: RadiusNormSquaredPrefixOrPostfix) -> Scalar {
        return this.radius_norm_squared();
    }
}
extension Flector: RadiusNormSquared {
    associatedtype Output = Scalar;
    func radius_norm_squared() -> Output {
        return this.anti_dot_product(this).right_anti_dual();
    }
}
extension Line {
    // Fancy infix trick
    func operator/(_rhs: RadiusNormSquaredPrefixOrPostfix) -> Scalar {
        return this.radius_norm_squared();
    }
}
extension Line: RadiusNormSquared {
    associatedtype Output = Scalar;
    func radius_norm_squared() -> Output {
        return this.anti_dot_product(this).right_anti_dual();
    }
}
extension Motor {
    // Fancy infix trick
    func operator/(_rhs: RadiusNormSquaredPrefixOrPostfix) -> Scalar {
        return this.radius_norm_squared();
    }
}
extension Motor: RadiusNormSquared {
    associatedtype Output = Scalar;
    func radius_norm_squared() -> Output {
        return this.anti_dot_product(this).right_anti_dual();
    }
}
extension MultiVector {
    // Fancy infix trick
    func operator/(_rhs: RadiusNormSquaredPrefixOrPostfix) -> Scalar {
        return this.radius_norm_squared();
    }
}
extension MultiVector: RadiusNormSquared {
    associatedtype Output = Scalar;
    func radius_norm_squared() -> Output {
        return this.anti_dot_product(this).right_anti_dual();
    }
}
extension Plane {
    // Fancy infix trick
    func operator/(_rhs: RadiusNormSquaredPrefixOrPostfix) -> Scalar {
        return this.radius_norm_squared();
    }
}
extension Plane: RadiusNormSquared {
    associatedtype Output = Scalar;
    func radius_norm_squared() -> Output {
        return this.anti_dot_product(this).right_anti_dual();
    }
}
extension RoundPoint {
    // Fancy infix trick
    func operator/(_rhs: RadiusNormSquaredPrefixOrPostfix) -> Scalar {
        return this.radius_norm_squared();
    }
}
extension RoundPoint: RadiusNormSquared {
    associatedtype Output = Scalar;
    func radius_norm_squared() -> Output {
        return this.anti_dot_product(this).right_anti_dual();
    }
}
extension Scalar {
    // Fancy infix trick
    func operator/(_rhs: RadiusNormSquaredPrefixOrPostfix) -> Scalar {
        return this.radius_norm_squared();
    }
    func operator/=(const RadiusNormSquaredPrefixOrPostfix& _rhs) -> Scalar& {
        this = this.radius_norm_squared();
        return *this;
    }
}
extension Scalar: RadiusNormSquared {
    associatedtype Output = Scalar;
    func radius_norm_squared() -> Output {
        return this.anti_dot_product(this).right_anti_dual();
    }
}
extension Sphere {
    // Fancy infix trick
    func operator/(_rhs: RadiusNormSquaredPrefixOrPostfix) -> Scalar {
        return this.radius_norm_squared();
    }
}
extension Sphere: RadiusNormSquared {
    associatedtype Output = Scalar;
    func radius_norm_squared() -> Output {
        return this.anti_dot_product(this).right_anti_dual();
    }
}
extension VersorEven {
    // Fancy infix trick
    func operator/(_rhs: RadiusNormSquaredPrefixOrPostfix) -> Scalar {
        return this.radius_norm_squared();
    }
}
extension VersorEven: RadiusNormSquared {
    associatedtype Output = Scalar;
    func radius_norm_squared() -> Output {
        return this.anti_dot_product(this).right_anti_dual();
    }
}
extension VersorOdd {
    // Fancy infix trick
    func operator/(_rhs: RadiusNormSquaredPrefixOrPostfix) -> Scalar {
        return this.radius_norm_squared();
    }
}
extension VersorOdd: RadiusNormSquared {
    associatedtype Output = Scalar;
    func radius_norm_squared() -> Output {
        return this.anti_dot_product(this).right_anti_dual();
    }
}

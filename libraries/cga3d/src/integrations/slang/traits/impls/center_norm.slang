using traits::CenterNormSquared;
using traits::SquareRoot;
extension AntiCircleRotor {
    // Fancy infix trick
    func operator/(_rhs: CenterNormPrefixOrPostfix) -> Scalar {
        return this.center_norm();
    }
}
extension AntiCircleRotor: CenterNorm {
    associatedtype Output = Scalar;
    func center_norm() -> Output {
        return this.center_norm_squared().square_root();
    }
}
extension AntiDipoleInversion {
    // Fancy infix trick
    func operator/(_rhs: CenterNormPrefixOrPostfix) -> Scalar {
        return this.center_norm();
    }
}
extension AntiDipoleInversion: CenterNorm {
    associatedtype Output = Scalar;
    func center_norm() -> Output {
        return this.center_norm_squared().square_root();
    }
}
extension Circle {
    // Fancy infix trick
    func operator/(_rhs: CenterNormPrefixOrPostfix) -> Scalar {
        return this.center_norm();
    }
}
extension Circle: CenterNorm {
    associatedtype Output = Scalar;
    func center_norm() -> Output {
        return this.center_norm_squared().square_root();
    }
}
extension CircleRotor {
    // Fancy infix trick
    func operator/(_rhs: CenterNormPrefixOrPostfix) -> Scalar {
        return this.center_norm();
    }
}
extension CircleRotor: CenterNorm {
    associatedtype Output = Scalar;
    func center_norm() -> Output {
        return this.center_norm_squared().square_root();
    }
}
extension Dipole {
    // Fancy infix trick
    func operator/(_rhs: CenterNormPrefixOrPostfix) -> Scalar {
        return this.center_norm();
    }
}
extension Dipole: CenterNorm {
    associatedtype Output = Scalar;
    func center_norm() -> Output {
        return this.center_norm_squared().square_root();
    }
}
extension DipoleInversion {
    // Fancy infix trick
    func operator/(_rhs: CenterNormPrefixOrPostfix) -> Scalar {
        return this.center_norm();
    }
}
extension DipoleInversion: CenterNorm {
    associatedtype Output = Scalar;
    func center_norm() -> Output {
        return this.center_norm_squared().square_root();
    }
}
extension MultiVector {
    // Fancy infix trick
    func operator/(_rhs: CenterNormPrefixOrPostfix) -> Scalar {
        return this.center_norm();
    }
}
extension MultiVector: CenterNorm {
    associatedtype Output = Scalar;
    func center_norm() -> Output {
        return this.center_norm_squared().square_root();
    }
}
extension VersorEven {
    // Fancy infix trick
    func operator/(_rhs: CenterNormPrefixOrPostfix) -> Scalar {
        return this.center_norm();
    }
}
extension VersorEven: CenterNorm {
    associatedtype Output = Scalar;
    func center_norm() -> Output {
        return this.center_norm_squared().square_root();
    }
}
extension VersorOdd {
    // Fancy infix trick
    func operator/(_rhs: CenterNormPrefixOrPostfix) -> Scalar {
        return this.center_norm();
    }
}
extension VersorOdd: CenterNorm {
    associatedtype Output = Scalar;
    func center_norm() -> Output {
        return this.center_norm_squared().square_root();
    }
}

using traits::UnitizedCenterNormSquared;
extension AntiCircleRotor {
    // Fancy infix trick
    func operator/(_rhs: UnitizedCenterNormPrefixOrPostfix) -> float {
        return this.unitized_center_norm();
    }
}
extension AntiCircleRotor: UnitizedCenterNorm {
    associatedtype Output = float;
    func unitized_center_norm() -> Output {
        return powf(this.unitized_center_norm_squared(), 0.5);
    }
}
extension AntiDipoleInversion {
    // Fancy infix trick
    func operator/(_rhs: UnitizedCenterNormPrefixOrPostfix) -> float {
        return this.unitized_center_norm();
    }
}
extension AntiDipoleInversion: UnitizedCenterNorm {
    associatedtype Output = float;
    func unitized_center_norm() -> Output {
        return powf(this.unitized_center_norm_squared(), 0.5);
    }
}
extension Circle {
    // Fancy infix trick
    func operator/(_rhs: UnitizedCenterNormPrefixOrPostfix) -> float {
        return this.unitized_center_norm();
    }
}
extension Circle: UnitizedCenterNorm {
    associatedtype Output = float;
    func unitized_center_norm() -> Output {
        return powf(this.unitized_center_norm_squared(), 0.5);
    }
}
extension CircleRotor {
    // Fancy infix trick
    func operator/(_rhs: UnitizedCenterNormPrefixOrPostfix) -> float {
        return this.unitized_center_norm();
    }
}
extension CircleRotor: UnitizedCenterNorm {
    associatedtype Output = float;
    func unitized_center_norm() -> Output {
        return powf(this.unitized_center_norm_squared(), 0.5);
    }
}
extension Dipole {
    // Fancy infix trick
    func operator/(_rhs: UnitizedCenterNormPrefixOrPostfix) -> float {
        return this.unitized_center_norm();
    }
}
extension Dipole: UnitizedCenterNorm {
    associatedtype Output = float;
    func unitized_center_norm() -> Output {
        return powf(this.unitized_center_norm_squared(), 0.5);
    }
}
extension DipoleInversion {
    // Fancy infix trick
    func operator/(_rhs: UnitizedCenterNormPrefixOrPostfix) -> float {
        return this.unitized_center_norm();
    }
}
extension DipoleInversion: UnitizedCenterNorm {
    associatedtype Output = float;
    func unitized_center_norm() -> Output {
        return powf(this.unitized_center_norm_squared(), 0.5);
    }
}
extension MultiVector {
    // Fancy infix trick
    func operator/(_rhs: UnitizedCenterNormPrefixOrPostfix) -> float {
        return this.unitized_center_norm();
    }
}
extension MultiVector: UnitizedCenterNorm {
    associatedtype Output = float;
    func unitized_center_norm() -> Output {
        return powf(this.unitized_center_norm_squared(), 0.5);
    }
}
extension VersorEven {
    // Fancy infix trick
    func operator/(_rhs: UnitizedCenterNormPrefixOrPostfix) -> float {
        return this.unitized_center_norm();
    }
}
extension VersorEven: UnitizedCenterNorm {
    associatedtype Output = float;
    func unitized_center_norm() -> Output {
        return powf(this.unitized_center_norm_squared(), 0.5);
    }
}
extension VersorOdd {
    // Fancy infix trick
    func operator/(_rhs: UnitizedCenterNormPrefixOrPostfix) -> float {
        return this.unitized_center_norm();
    }
}
extension VersorOdd: UnitizedCenterNorm {
    associatedtype Output = float;
    func unitized_center_norm() -> Output {
        return powf(this.unitized_center_norm_squared(), 0.5);
    }
}

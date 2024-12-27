using traits::UnitizedRadiusNormSquared;
extension AntiCircleRotor {
    // Fancy infix trick
    func operator/(_rhs: UnitizedRadiusNormPrefixOrPostfix) -> float {
        return this.unitized_radius_norm();
    }
}
extension AntiCircleRotor: UnitizedRadiusNorm {
    associatedtype Output = float;
    func unitized_radius_norm() -> Output {
        return powf(this.unitized_radius_norm_squared(), 0.5);
    }
}
extension AntiDipoleInversion {
    // Fancy infix trick
    func operator/(_rhs: UnitizedRadiusNormPrefixOrPostfix) -> float {
        return this.unitized_radius_norm();
    }
}
extension AntiDipoleInversion: UnitizedRadiusNorm {
    associatedtype Output = float;
    func unitized_radius_norm() -> Output {
        return powf(this.unitized_radius_norm_squared(), 0.5);
    }
}
extension Circle {
    // Fancy infix trick
    func operator/(_rhs: UnitizedRadiusNormPrefixOrPostfix) -> float {
        return this.unitized_radius_norm();
    }
}
extension Circle: UnitizedRadiusNorm {
    associatedtype Output = float;
    func unitized_radius_norm() -> Output {
        return powf(this.unitized_radius_norm_squared(), 0.5);
    }
}
extension CircleRotor {
    // Fancy infix trick
    func operator/(_rhs: UnitizedRadiusNormPrefixOrPostfix) -> float {
        return this.unitized_radius_norm();
    }
}
extension CircleRotor: UnitizedRadiusNorm {
    associatedtype Output = float;
    func unitized_radius_norm() -> Output {
        return powf(this.unitized_radius_norm_squared(), 0.5);
    }
}
extension Dipole {
    // Fancy infix trick
    func operator/(_rhs: UnitizedRadiusNormPrefixOrPostfix) -> float {
        return this.unitized_radius_norm();
    }
}
extension Dipole: UnitizedRadiusNorm {
    associatedtype Output = float;
    func unitized_radius_norm() -> Output {
        return powf(this.unitized_radius_norm_squared(), 0.5);
    }
}
extension DipoleInversion {
    // Fancy infix trick
    func operator/(_rhs: UnitizedRadiusNormPrefixOrPostfix) -> float {
        return this.unitized_radius_norm();
    }
}
extension DipoleInversion: UnitizedRadiusNorm {
    associatedtype Output = float;
    func unitized_radius_norm() -> Output {
        return powf(this.unitized_radius_norm_squared(), 0.5);
    }
}
extension MultiVector {
    // Fancy infix trick
    func operator/(_rhs: UnitizedRadiusNormPrefixOrPostfix) -> float {
        return this.unitized_radius_norm();
    }
}
extension MultiVector: UnitizedRadiusNorm {
    associatedtype Output = float;
    func unitized_radius_norm() -> Output {
        return powf(this.unitized_radius_norm_squared(), 0.5);
    }
}
extension RoundPoint {
    // Fancy infix trick
    func operator/(_rhs: UnitizedRadiusNormPrefixOrPostfix) -> float {
        return this.unitized_radius_norm();
    }
}
extension RoundPoint: UnitizedRadiusNorm {
    associatedtype Output = float;
    func unitized_radius_norm() -> Output {
        return powf(this.unitized_radius_norm_squared(), 0.5);
    }
}
extension Sphere {
    // Fancy infix trick
    func operator/(_rhs: UnitizedRadiusNormPrefixOrPostfix) -> float {
        return this.unitized_radius_norm();
    }
}
extension Sphere: UnitizedRadiusNorm {
    associatedtype Output = float;
    func unitized_radius_norm() -> Output {
        return powf(this.unitized_radius_norm_squared(), 0.5);
    }
}
extension VersorEven {
    // Fancy infix trick
    func operator/(_rhs: UnitizedRadiusNormPrefixOrPostfix) -> float {
        return this.unitized_radius_norm();
    }
}
extension VersorEven: UnitizedRadiusNorm {
    associatedtype Output = float;
    func unitized_radius_norm() -> Output {
        return powf(this.unitized_radius_norm_squared(), 0.5);
    }
}
extension VersorOdd {
    // Fancy infix trick
    func operator/(_rhs: UnitizedRadiusNormPrefixOrPostfix) -> float {
        return this.unitized_radius_norm();
    }
}
extension VersorOdd: UnitizedRadiusNorm {
    associatedtype Output = float;
    func unitized_radius_norm() -> Output {
        return powf(this.unitized_radius_norm_squared(), 0.5);
    }
}

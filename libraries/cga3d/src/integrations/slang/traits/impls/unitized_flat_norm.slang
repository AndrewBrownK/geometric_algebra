using traits::UnitizedFlatNormSquared;
extension AntiCircleRotor {
    // Fancy infix trick
    func operator/(_rhs: UnitizedFlatNormPrefixOrPostfix) -> float {
        return this.unitized_flat_norm();
    }
}
extension AntiCircleRotor: UnitizedFlatNorm {
    associatedtype Output = float;
    func unitized_flat_norm() -> Output {
        return powf(this.unitized_flat_norm_squared(), 0.5);
    }
}
extension AntiDipoleInversion {
    // Fancy infix trick
    func operator/(_rhs: UnitizedFlatNormPrefixOrPostfix) -> float {
        return this.unitized_flat_norm();
    }
}
extension AntiDipoleInversion: UnitizedFlatNorm {
    associatedtype Output = float;
    func unitized_flat_norm() -> Output {
        return powf(this.unitized_flat_norm_squared(), 0.5);
    }
}
extension Circle {
    // Fancy infix trick
    func operator/(_rhs: UnitizedFlatNormPrefixOrPostfix) -> float {
        return this.unitized_flat_norm();
    }
}
extension Circle: UnitizedFlatNorm {
    associatedtype Output = float;
    func unitized_flat_norm() -> Output {
        return powf(this.unitized_flat_norm_squared(), 0.5);
    }
}
extension CircleRotor {
    // Fancy infix trick
    func operator/(_rhs: UnitizedFlatNormPrefixOrPostfix) -> float {
        return this.unitized_flat_norm();
    }
}
extension CircleRotor: UnitizedFlatNorm {
    associatedtype Output = float;
    func unitized_flat_norm() -> Output {
        return powf(this.unitized_flat_norm_squared(), 0.5);
    }
}
extension Dipole {
    // Fancy infix trick
    func operator/(_rhs: UnitizedFlatNormPrefixOrPostfix) -> float {
        return this.unitized_flat_norm();
    }
}
extension Dipole: UnitizedFlatNorm {
    associatedtype Output = float;
    func unitized_flat_norm() -> Output {
        return powf(this.unitized_flat_norm_squared(), 0.5);
    }
}
extension DipoleInversion {
    // Fancy infix trick
    func operator/(_rhs: UnitizedFlatNormPrefixOrPostfix) -> float {
        return this.unitized_flat_norm();
    }
}
extension DipoleInversion: UnitizedFlatNorm {
    associatedtype Output = float;
    func unitized_flat_norm() -> Output {
        return powf(this.unitized_flat_norm_squared(), 0.5);
    }
}
extension DualNum {
    // Fancy infix trick
    func operator/(_rhs: UnitizedFlatNormPrefixOrPostfix) -> float {
        return this.unitized_flat_norm();
    }
}
extension DualNum: UnitizedFlatNorm {
    associatedtype Output = float;
    func unitized_flat_norm() -> Output {
        return powf(this.unitized_flat_norm_squared(), 0.5);
    }
}
extension FlatPoint {
    // Fancy infix trick
    func operator/(_rhs: UnitizedFlatNormPrefixOrPostfix) -> float {
        return this.unitized_flat_norm();
    }
}
extension FlatPoint: UnitizedFlatNorm {
    associatedtype Output = float;
    func unitized_flat_norm() -> Output {
        return powf(this.unitized_flat_norm_squared(), 0.5);
    }
}
extension Flector {
    // Fancy infix trick
    func operator/(_rhs: UnitizedFlatNormPrefixOrPostfix) -> float {
        return this.unitized_flat_norm();
    }
}
extension Flector: UnitizedFlatNorm {
    associatedtype Output = float;
    func unitized_flat_norm() -> Output {
        return powf(this.unitized_flat_norm_squared(), 0.5);
    }
}
extension Line {
    // Fancy infix trick
    func operator/(_rhs: UnitizedFlatNormPrefixOrPostfix) -> float {
        return this.unitized_flat_norm();
    }
}
extension Line: UnitizedFlatNorm {
    associatedtype Output = float;
    func unitized_flat_norm() -> Output {
        return powf(this.unitized_flat_norm_squared(), 0.5);
    }
}
extension Motor {
    // Fancy infix trick
    func operator/(_rhs: UnitizedFlatNormPrefixOrPostfix) -> float {
        return this.unitized_flat_norm();
    }
}
extension Motor: UnitizedFlatNorm {
    associatedtype Output = float;
    func unitized_flat_norm() -> Output {
        return powf(this.unitized_flat_norm_squared(), 0.5);
    }
}
extension MultiVector {
    // Fancy infix trick
    func operator/(_rhs: UnitizedFlatNormPrefixOrPostfix) -> float {
        return this.unitized_flat_norm();
    }
}
extension MultiVector: UnitizedFlatNorm {
    associatedtype Output = float;
    func unitized_flat_norm() -> Output {
        return powf(this.unitized_flat_norm_squared(), 0.5);
    }
}
extension Plane {
    // Fancy infix trick
    func operator/(_rhs: UnitizedFlatNormPrefixOrPostfix) -> float {
        return this.unitized_flat_norm();
    }
}
extension Plane: UnitizedFlatNorm {
    associatedtype Output = float;
    func unitized_flat_norm() -> Output {
        return powf(this.unitized_flat_norm_squared(), 0.5);
    }
}
extension Sphere {
    // Fancy infix trick
    func operator/(_rhs: UnitizedFlatNormPrefixOrPostfix) -> float {
        return this.unitized_flat_norm();
    }
}
extension Sphere: UnitizedFlatNorm {
    associatedtype Output = float;
    func unitized_flat_norm() -> Output {
        return powf(this.unitized_flat_norm_squared(), 0.5);
    }
}
extension VersorEven {
    // Fancy infix trick
    func operator/(_rhs: UnitizedFlatNormPrefixOrPostfix) -> float {
        return this.unitized_flat_norm();
    }
}
extension VersorEven: UnitizedFlatNorm {
    associatedtype Output = float;
    func unitized_flat_norm() -> Output {
        return powf(this.unitized_flat_norm_squared(), 0.5);
    }
}
extension VersorOdd {
    // Fancy infix trick
    func operator/(_rhs: UnitizedFlatNormPrefixOrPostfix) -> float {
        return this.unitized_flat_norm();
    }
}
extension VersorOdd: UnitizedFlatNorm {
    associatedtype Output = float;
    func unitized_flat_norm() -> Output {
        return powf(this.unitized_flat_norm_squared(), 0.5);
    }
}

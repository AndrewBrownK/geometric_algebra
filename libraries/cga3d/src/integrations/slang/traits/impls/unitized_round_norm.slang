using traits::UnitizedRoundNormSquared;
extension AntiCircleRotor {
    // Fancy infix trick
    func operator/(_rhs: UnitizedRoundNormPrefixOrPostfix) -> float {
        return this.unitized_round_norm();
    }
}
extension AntiCircleRotor: UnitizedRoundNorm {
    associatedtype Output = float;
    func unitized_round_norm() -> Output {
        return powf(this.unitized_round_norm_squared(), 0.5);
    }
}
extension AntiDipoleInversion {
    // Fancy infix trick
    func operator/(_rhs: UnitizedRoundNormPrefixOrPostfix) -> float {
        return this.unitized_round_norm();
    }
}
extension AntiDipoleInversion: UnitizedRoundNorm {
    associatedtype Output = float;
    func unitized_round_norm() -> Output {
        return powf(this.unitized_round_norm_squared(), 0.5);
    }
}
extension Circle {
    // Fancy infix trick
    func operator/(_rhs: UnitizedRoundNormPrefixOrPostfix) -> float {
        return this.unitized_round_norm();
    }
}
extension Circle: UnitizedRoundNorm {
    associatedtype Output = float;
    func unitized_round_norm() -> Output {
        return powf(this.unitized_round_norm_squared(), 0.5);
    }
}
extension CircleRotor {
    // Fancy infix trick
    func operator/(_rhs: UnitizedRoundNormPrefixOrPostfix) -> float {
        return this.unitized_round_norm();
    }
}
extension CircleRotor: UnitizedRoundNorm {
    associatedtype Output = float;
    func unitized_round_norm() -> Output {
        return powf(this.unitized_round_norm_squared(), 0.5);
    }
}
extension Dipole {
    // Fancy infix trick
    func operator/(_rhs: UnitizedRoundNormPrefixOrPostfix) -> float {
        return this.unitized_round_norm();
    }
}
extension Dipole: UnitizedRoundNorm {
    associatedtype Output = float;
    func unitized_round_norm() -> Output {
        return powf(this.unitized_round_norm_squared(), 0.5);
    }
}
extension DipoleInversion {
    // Fancy infix trick
    func operator/(_rhs: UnitizedRoundNormPrefixOrPostfix) -> float {
        return this.unitized_round_norm();
    }
}
extension DipoleInversion: UnitizedRoundNorm {
    associatedtype Output = float;
    func unitized_round_norm() -> Output {
        return powf(this.unitized_round_norm_squared(), 0.5);
    }
}
extension MultiVector {
    // Fancy infix trick
    func operator/(_rhs: UnitizedRoundNormPrefixOrPostfix) -> float {
        return this.unitized_round_norm();
    }
}
extension MultiVector: UnitizedRoundNorm {
    associatedtype Output = float;
    func unitized_round_norm() -> Output {
        return powf(this.unitized_round_norm_squared(), 0.5);
    }
}
extension RoundPoint {
    // Fancy infix trick
    func operator/(_rhs: UnitizedRoundNormPrefixOrPostfix) -> float {
        return this.unitized_round_norm();
    }
}
extension RoundPoint: UnitizedRoundNorm {
    associatedtype Output = float;
    func unitized_round_norm() -> Output {
        return powf(this.unitized_round_norm_squared(), 0.5);
    }
}
extension VersorEven {
    // Fancy infix trick
    func operator/(_rhs: UnitizedRoundNormPrefixOrPostfix) -> float {
        return this.unitized_round_norm();
    }
}
extension VersorEven: UnitizedRoundNorm {
    associatedtype Output = float;
    func unitized_round_norm() -> Output {
        return powf(this.unitized_round_norm_squared(), 0.5);
    }
}
extension VersorOdd {
    // Fancy infix trick
    func operator/(_rhs: UnitizedRoundNormPrefixOrPostfix) -> float {
        return this.unitized_round_norm();
    }
}
extension VersorOdd: UnitizedRoundNorm {
    associatedtype Output = float;
    func unitized_round_norm() -> Output {
        return powf(this.unitized_round_norm_squared(), 0.5);
    }
}

using traits::RoundBulkNormSquared;
using traits::RoundWeightNormSquared;
extension AntiCircleRotor {
    // Fancy infix trick
    func operator/(_rhs: UnitizedRoundNormSquaredPrefixOrPostfix) -> float {
        return this.unitized_round_norm_squared();
    }
}
extension AntiCircleRotor: UnitizedRoundNormSquared {
    associatedtype Output = float;
    func unitized_round_norm_squared() -> Output {
        return this.round_bulk_norm_squared().scalar / (this.round_weight_norm_squared().e12345);
    }
}
extension AntiDipoleInversion {
    // Fancy infix trick
    func operator/(_rhs: UnitizedRoundNormSquaredPrefixOrPostfix) -> float {
        return this.unitized_round_norm_squared();
    }
}
extension AntiDipoleInversion: UnitizedRoundNormSquared {
    associatedtype Output = float;
    func unitized_round_norm_squared() -> Output {
        return this.round_bulk_norm_squared().scalar / (this.round_weight_norm_squared().e12345);
    }
}
extension Circle {
    // Fancy infix trick
    func operator/(_rhs: UnitizedRoundNormSquaredPrefixOrPostfix) -> float {
        return this.unitized_round_norm_squared();
    }
}
extension Circle: UnitizedRoundNormSquared {
    associatedtype Output = float;
    func unitized_round_norm_squared() -> Output {
        return this.round_bulk_norm_squared().scalar / (this.round_weight_norm_squared().e12345);
    }
}
extension CircleRotor {
    // Fancy infix trick
    func operator/(_rhs: UnitizedRoundNormSquaredPrefixOrPostfix) -> float {
        return this.unitized_round_norm_squared();
    }
}
extension CircleRotor: UnitizedRoundNormSquared {
    associatedtype Output = float;
    func unitized_round_norm_squared() -> Output {
        return this.round_bulk_norm_squared().scalar / (this.round_weight_norm_squared().e12345);
    }
}
extension Dipole {
    // Fancy infix trick
    func operator/(_rhs: UnitizedRoundNormSquaredPrefixOrPostfix) -> float {
        return this.unitized_round_norm_squared();
    }
}
extension Dipole: UnitizedRoundNormSquared {
    associatedtype Output = float;
    func unitized_round_norm_squared() -> Output {
        return this.round_bulk_norm_squared().scalar / (this.round_weight_norm_squared().e12345);
    }
}
extension DipoleInversion {
    // Fancy infix trick
    func operator/(_rhs: UnitizedRoundNormSquaredPrefixOrPostfix) -> float {
        return this.unitized_round_norm_squared();
    }
}
extension DipoleInversion: UnitizedRoundNormSquared {
    associatedtype Output = float;
    func unitized_round_norm_squared() -> Output {
        return this.round_bulk_norm_squared().scalar / (this.round_weight_norm_squared().e12345);
    }
}
extension MultiVector {
    // Fancy infix trick
    func operator/(_rhs: UnitizedRoundNormSquaredPrefixOrPostfix) -> float {
        return this.unitized_round_norm_squared();
    }
}
extension MultiVector: UnitizedRoundNormSquared {
    associatedtype Output = float;
    func unitized_round_norm_squared() -> Output {
        return this.round_bulk_norm_squared().scalar / (this.round_weight_norm_squared().e12345);
    }
}
extension RoundPoint {
    // Fancy infix trick
    func operator/(_rhs: UnitizedRoundNormSquaredPrefixOrPostfix) -> float {
        return this.unitized_round_norm_squared();
    }
}
extension RoundPoint: UnitizedRoundNormSquared {
    associatedtype Output = float;
    func unitized_round_norm_squared() -> Output {
        return this.round_bulk_norm_squared().scalar / (this.round_weight_norm_squared().e12345);
    }
}
extension VersorEven {
    // Fancy infix trick
    func operator/(_rhs: UnitizedRoundNormSquaredPrefixOrPostfix) -> float {
        return this.unitized_round_norm_squared();
    }
}
extension VersorEven: UnitizedRoundNormSquared {
    associatedtype Output = float;
    func unitized_round_norm_squared() -> Output {
        return this.round_bulk_norm_squared().scalar / (this.round_weight_norm_squared().e12345);
    }
}
extension VersorOdd {
    // Fancy infix trick
    func operator/(_rhs: UnitizedRoundNormSquaredPrefixOrPostfix) -> float {
        return this.unitized_round_norm_squared();
    }
}
extension VersorOdd: UnitizedRoundNormSquared {
    associatedtype Output = float;
    func unitized_round_norm_squared() -> Output {
        return this.round_bulk_norm_squared().scalar / (this.round_weight_norm_squared().e12345);
    }
}

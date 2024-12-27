using traits::CenterNormSquared;
using traits::RoundWeightNormSquared;
extension AntiCircleRotor {
    // Fancy infix trick
    func operator/(_rhs: UnitizedCenterNormSquaredPrefixOrPostfix) -> float {
        return this.unitized_center_norm_squared();
    }
}
extension AntiCircleRotor: UnitizedCenterNormSquared {
    associatedtype Output = float;
    func unitized_center_norm_squared() -> Output {
        return this.center_norm_squared().scalar / (this.round_weight_norm_squared().e12345);
    }
}
extension AntiDipoleInversion {
    // Fancy infix trick
    func operator/(_rhs: UnitizedCenterNormSquaredPrefixOrPostfix) -> float {
        return this.unitized_center_norm_squared();
    }
}
extension AntiDipoleInversion: UnitizedCenterNormSquared {
    associatedtype Output = float;
    func unitized_center_norm_squared() -> Output {
        return this.center_norm_squared().scalar / (this.round_weight_norm_squared().e12345);
    }
}
extension Circle {
    // Fancy infix trick
    func operator/(_rhs: UnitizedCenterNormSquaredPrefixOrPostfix) -> float {
        return this.unitized_center_norm_squared();
    }
}
extension Circle: UnitizedCenterNormSquared {
    associatedtype Output = float;
    func unitized_center_norm_squared() -> Output {
        return this.center_norm_squared().scalar / (this.round_weight_norm_squared().e12345);
    }
}
extension CircleRotor {
    // Fancy infix trick
    func operator/(_rhs: UnitizedCenterNormSquaredPrefixOrPostfix) -> float {
        return this.unitized_center_norm_squared();
    }
}
extension CircleRotor: UnitizedCenterNormSquared {
    associatedtype Output = float;
    func unitized_center_norm_squared() -> Output {
        return this.center_norm_squared().scalar / (this.round_weight_norm_squared().e12345);
    }
}
extension Dipole {
    // Fancy infix trick
    func operator/(_rhs: UnitizedCenterNormSquaredPrefixOrPostfix) -> float {
        return this.unitized_center_norm_squared();
    }
}
extension Dipole: UnitizedCenterNormSquared {
    associatedtype Output = float;
    func unitized_center_norm_squared() -> Output {
        return this.center_norm_squared().scalar / (this.round_weight_norm_squared().e12345);
    }
}
extension DipoleInversion {
    // Fancy infix trick
    func operator/(_rhs: UnitizedCenterNormSquaredPrefixOrPostfix) -> float {
        return this.unitized_center_norm_squared();
    }
}
extension DipoleInversion: UnitizedCenterNormSquared {
    associatedtype Output = float;
    func unitized_center_norm_squared() -> Output {
        return this.center_norm_squared().scalar / (this.round_weight_norm_squared().e12345);
    }
}
extension MultiVector {
    // Fancy infix trick
    func operator/(_rhs: UnitizedCenterNormSquaredPrefixOrPostfix) -> float {
        return this.unitized_center_norm_squared();
    }
}
extension MultiVector: UnitizedCenterNormSquared {
    associatedtype Output = float;
    func unitized_center_norm_squared() -> Output {
        return this.center_norm_squared().scalar / (this.round_weight_norm_squared().e12345);
    }
}
extension VersorEven {
    // Fancy infix trick
    func operator/(_rhs: UnitizedCenterNormSquaredPrefixOrPostfix) -> float {
        return this.unitized_center_norm_squared();
    }
}
extension VersorEven: UnitizedCenterNormSquared {
    associatedtype Output = float;
    func unitized_center_norm_squared() -> Output {
        return this.center_norm_squared().scalar / (this.round_weight_norm_squared().e12345);
    }
}
extension VersorOdd {
    // Fancy infix trick
    func operator/(_rhs: UnitizedCenterNormSquaredPrefixOrPostfix) -> float {
        return this.unitized_center_norm_squared();
    }
}
extension VersorOdd: UnitizedCenterNormSquared {
    associatedtype Output = float;
    func unitized_center_norm_squared() -> Output {
        return this.center_norm_squared().scalar / (this.round_weight_norm_squared().e12345);
    }
}

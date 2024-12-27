using traits::RadiusNormSquared;
using traits::RoundWeightNormSquared;
extension AntiCircleRotor {
    // Fancy infix trick
    func operator/(_rhs: UnitizedRadiusNormSquaredPrefixOrPostfix) -> float {
        return this.unitized_radius_norm_squared();
    }
}
extension AntiCircleRotor: UnitizedRadiusNormSquared {
    associatedtype Output = float;
    func unitized_radius_norm_squared() -> Output {
        return this.radius_norm_squared().scalar / (this.round_weight_norm_squared().e12345);
    }
}
extension AntiDipoleInversion {
    // Fancy infix trick
    func operator/(_rhs: UnitizedRadiusNormSquaredPrefixOrPostfix) -> float {
        return this.unitized_radius_norm_squared();
    }
}
extension AntiDipoleInversion: UnitizedRadiusNormSquared {
    associatedtype Output = float;
    func unitized_radius_norm_squared() -> Output {
        return this.radius_norm_squared().scalar / (this.round_weight_norm_squared().e12345);
    }
}
extension Circle {
    // Fancy infix trick
    func operator/(_rhs: UnitizedRadiusNormSquaredPrefixOrPostfix) -> float {
        return this.unitized_radius_norm_squared();
    }
}
extension Circle: UnitizedRadiusNormSquared {
    associatedtype Output = float;
    func unitized_radius_norm_squared() -> Output {
        return this.radius_norm_squared().scalar / (this.round_weight_norm_squared().e12345);
    }
}
extension CircleRotor {
    // Fancy infix trick
    func operator/(_rhs: UnitizedRadiusNormSquaredPrefixOrPostfix) -> float {
        return this.unitized_radius_norm_squared();
    }
}
extension CircleRotor: UnitizedRadiusNormSquared {
    associatedtype Output = float;
    func unitized_radius_norm_squared() -> Output {
        return this.radius_norm_squared().scalar / (this.round_weight_norm_squared().e12345);
    }
}
extension Dipole {
    // Fancy infix trick
    func operator/(_rhs: UnitizedRadiusNormSquaredPrefixOrPostfix) -> float {
        return this.unitized_radius_norm_squared();
    }
}
extension Dipole: UnitizedRadiusNormSquared {
    associatedtype Output = float;
    func unitized_radius_norm_squared() -> Output {
        return this.radius_norm_squared().scalar / (this.round_weight_norm_squared().e12345);
    }
}
extension DipoleInversion {
    // Fancy infix trick
    func operator/(_rhs: UnitizedRadiusNormSquaredPrefixOrPostfix) -> float {
        return this.unitized_radius_norm_squared();
    }
}
extension DipoleInversion: UnitizedRadiusNormSquared {
    associatedtype Output = float;
    func unitized_radius_norm_squared() -> Output {
        return this.radius_norm_squared().scalar / (this.round_weight_norm_squared().e12345);
    }
}
extension MultiVector {
    // Fancy infix trick
    func operator/(_rhs: UnitizedRadiusNormSquaredPrefixOrPostfix) -> float {
        return this.unitized_radius_norm_squared();
    }
}
extension MultiVector: UnitizedRadiusNormSquared {
    associatedtype Output = float;
    func unitized_radius_norm_squared() -> Output {
        return this.radius_norm_squared().scalar / (this.round_weight_norm_squared().e12345);
    }
}
extension RoundPoint {
    // Fancy infix trick
    func operator/(_rhs: UnitizedRadiusNormSquaredPrefixOrPostfix) -> float {
        return this.unitized_radius_norm_squared();
    }
}
extension RoundPoint: UnitizedRadiusNormSquared {
    associatedtype Output = float;
    func unitized_radius_norm_squared() -> Output {
        return this.radius_norm_squared().scalar / (this.round_weight_norm_squared().e12345);
    }
}
extension Sphere {
    // Fancy infix trick
    func operator/(_rhs: UnitizedRadiusNormSquaredPrefixOrPostfix) -> float {
        return this.unitized_radius_norm_squared();
    }
}
extension Sphere: UnitizedRadiusNormSquared {
    associatedtype Output = float;
    func unitized_radius_norm_squared() -> Output {
        return this.radius_norm_squared().scalar / (this.round_weight_norm_squared().e12345);
    }
}
extension VersorEven {
    // Fancy infix trick
    func operator/(_rhs: UnitizedRadiusNormSquaredPrefixOrPostfix) -> float {
        return this.unitized_radius_norm_squared();
    }
}
extension VersorEven: UnitizedRadiusNormSquared {
    associatedtype Output = float;
    func unitized_radius_norm_squared() -> Output {
        return this.radius_norm_squared().scalar / (this.round_weight_norm_squared().e12345);
    }
}
extension VersorOdd {
    // Fancy infix trick
    func operator/(_rhs: UnitizedRadiusNormSquaredPrefixOrPostfix) -> float {
        return this.unitized_radius_norm_squared();
    }
}
extension VersorOdd: UnitizedRadiusNormSquared {
    associatedtype Output = float;
    func unitized_radius_norm_squared() -> Output {
        return this.radius_norm_squared().scalar / (this.round_weight_norm_squared().e12345);
    }
}

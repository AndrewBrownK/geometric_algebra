using traits::FlatBulkNormSquared;
using traits::FlatWeightNormSquared;
extension AntiCircleRotor {
    // Fancy infix trick
    func operator/(_rhs: UnitizedFlatNormSquaredPrefixOrPostfix) -> float {
        return this.unitized_flat_norm_squared();
    }
}
extension AntiCircleRotor: UnitizedFlatNormSquared {
    associatedtype Output = float;
    func unitized_flat_norm_squared() -> Output {
        return this.flat_bulk_norm_squared().scalar / (this.flat_weight_norm_squared().e12345);
    }
}
extension AntiDipoleInversion {
    // Fancy infix trick
    func operator/(_rhs: UnitizedFlatNormSquaredPrefixOrPostfix) -> float {
        return this.unitized_flat_norm_squared();
    }
}
extension AntiDipoleInversion: UnitizedFlatNormSquared {
    associatedtype Output = float;
    func unitized_flat_norm_squared() -> Output {
        return this.flat_bulk_norm_squared().scalar / (this.flat_weight_norm_squared().e12345);
    }
}
extension Circle {
    // Fancy infix trick
    func operator/(_rhs: UnitizedFlatNormSquaredPrefixOrPostfix) -> float {
        return this.unitized_flat_norm_squared();
    }
}
extension Circle: UnitizedFlatNormSquared {
    associatedtype Output = float;
    func unitized_flat_norm_squared() -> Output {
        return this.flat_bulk_norm_squared().scalar / (this.flat_weight_norm_squared().e12345);
    }
}
extension CircleRotor {
    // Fancy infix trick
    func operator/(_rhs: UnitizedFlatNormSquaredPrefixOrPostfix) -> float {
        return this.unitized_flat_norm_squared();
    }
}
extension CircleRotor: UnitizedFlatNormSquared {
    associatedtype Output = float;
    func unitized_flat_norm_squared() -> Output {
        return this.flat_bulk_norm_squared().scalar / (this.flat_weight_norm_squared().e12345);
    }
}
extension Dipole {
    // Fancy infix trick
    func operator/(_rhs: UnitizedFlatNormSquaredPrefixOrPostfix) -> float {
        return this.unitized_flat_norm_squared();
    }
}
extension Dipole: UnitizedFlatNormSquared {
    associatedtype Output = float;
    func unitized_flat_norm_squared() -> Output {
        return this.flat_bulk_norm_squared().scalar / (this.flat_weight_norm_squared().e12345);
    }
}
extension DipoleInversion {
    // Fancy infix trick
    func operator/(_rhs: UnitizedFlatNormSquaredPrefixOrPostfix) -> float {
        return this.unitized_flat_norm_squared();
    }
}
extension DipoleInversion: UnitizedFlatNormSquared {
    associatedtype Output = float;
    func unitized_flat_norm_squared() -> Output {
        return this.flat_bulk_norm_squared().scalar / (this.flat_weight_norm_squared().e12345);
    }
}
extension DualNum {
    // Fancy infix trick
    func operator/(_rhs: UnitizedFlatNormSquaredPrefixOrPostfix) -> float {
        return this.unitized_flat_norm_squared();
    }
}
extension DualNum: UnitizedFlatNormSquared {
    associatedtype Output = float;
    func unitized_flat_norm_squared() -> Output {
        return this.flat_bulk_norm_squared().scalar / (this.flat_weight_norm_squared().e12345);
    }
}
extension FlatPoint {
    // Fancy infix trick
    func operator/(_rhs: UnitizedFlatNormSquaredPrefixOrPostfix) -> float {
        return this.unitized_flat_norm_squared();
    }
}
extension FlatPoint: UnitizedFlatNormSquared {
    associatedtype Output = float;
    func unitized_flat_norm_squared() -> Output {
        return this.flat_bulk_norm_squared().scalar / (this.flat_weight_norm_squared().e12345);
    }
}
extension Flector {
    // Fancy infix trick
    func operator/(_rhs: UnitizedFlatNormSquaredPrefixOrPostfix) -> float {
        return this.unitized_flat_norm_squared();
    }
}
extension Flector: UnitizedFlatNormSquared {
    associatedtype Output = float;
    func unitized_flat_norm_squared() -> Output {
        return this.flat_bulk_norm_squared().scalar / (this.flat_weight_norm_squared().e12345);
    }
}
extension Line {
    // Fancy infix trick
    func operator/(_rhs: UnitizedFlatNormSquaredPrefixOrPostfix) -> float {
        return this.unitized_flat_norm_squared();
    }
}
extension Line: UnitizedFlatNormSquared {
    associatedtype Output = float;
    func unitized_flat_norm_squared() -> Output {
        return this.flat_bulk_norm_squared().scalar / (this.flat_weight_norm_squared().e12345);
    }
}
extension Motor {
    // Fancy infix trick
    func operator/(_rhs: UnitizedFlatNormSquaredPrefixOrPostfix) -> float {
        return this.unitized_flat_norm_squared();
    }
}
extension Motor: UnitizedFlatNormSquared {
    associatedtype Output = float;
    func unitized_flat_norm_squared() -> Output {
        return this.flat_bulk_norm_squared().scalar / (this.flat_weight_norm_squared().e12345);
    }
}
extension MultiVector {
    // Fancy infix trick
    func operator/(_rhs: UnitizedFlatNormSquaredPrefixOrPostfix) -> float {
        return this.unitized_flat_norm_squared();
    }
}
extension MultiVector: UnitizedFlatNormSquared {
    associatedtype Output = float;
    func unitized_flat_norm_squared() -> Output {
        return this.flat_bulk_norm_squared().scalar / (this.flat_weight_norm_squared().e12345);
    }
}
extension Plane {
    // Fancy infix trick
    func operator/(_rhs: UnitizedFlatNormSquaredPrefixOrPostfix) -> float {
        return this.unitized_flat_norm_squared();
    }
}
extension Plane: UnitizedFlatNormSquared {
    associatedtype Output = float;
    func unitized_flat_norm_squared() -> Output {
        return this.flat_bulk_norm_squared().scalar / (this.flat_weight_norm_squared().e12345);
    }
}
extension Sphere {
    // Fancy infix trick
    func operator/(_rhs: UnitizedFlatNormSquaredPrefixOrPostfix) -> float {
        return this.unitized_flat_norm_squared();
    }
}
extension Sphere: UnitizedFlatNormSquared {
    associatedtype Output = float;
    func unitized_flat_norm_squared() -> Output {
        return this.flat_bulk_norm_squared().scalar / (this.flat_weight_norm_squared().e12345);
    }
}
extension VersorEven {
    // Fancy infix trick
    func operator/(_rhs: UnitizedFlatNormSquaredPrefixOrPostfix) -> float {
        return this.unitized_flat_norm_squared();
    }
}
extension VersorEven: UnitizedFlatNormSquared {
    associatedtype Output = float;
    func unitized_flat_norm_squared() -> Output {
        return this.flat_bulk_norm_squared().scalar / (this.flat_weight_norm_squared().e12345);
    }
}
extension VersorOdd {
    // Fancy infix trick
    func operator/(_rhs: UnitizedFlatNormSquaredPrefixOrPostfix) -> float {
        return this.unitized_flat_norm_squared();
    }
}
extension VersorOdd: UnitizedFlatNormSquared {
    associatedtype Output = float;
    func unitized_flat_norm_squared() -> Output {
        return this.flat_bulk_norm_squared().scalar / (this.flat_weight_norm_squared().e12345);
    }
}

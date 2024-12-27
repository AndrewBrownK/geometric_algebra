using traits::FlatBulkNormSquared;
using traits::FlatWeightNormSquared;
extension Flector {
    // Fancy infix trick
    func operator/(_rhs: UnitizedFlatNormSquaredPrefixOrPostfix) -> float {
        return this.unitized_flat_norm_squared();
    }
}
extension Flector: UnitizedFlatNormSquared {
    associatedtype Output = float;
    func unitized_flat_norm_squared() -> Output {
        return this.flat_bulk_norm_squared().scalar / (this.flat_weight_norm_squared().e1234);
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
        return this.flat_bulk_norm_squared().scalar / (this.flat_weight_norm_squared().e1234);
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
        return this.flat_bulk_norm_squared().scalar / (this.flat_weight_norm_squared().e1234);
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
        return this.flat_bulk_norm_squared().scalar / (this.flat_weight_norm_squared().e1234);
    }
}
extension Point {
    // Fancy infix trick
    func operator/(_rhs: UnitizedFlatNormSquaredPrefixOrPostfix) -> float {
        return this.unitized_flat_norm_squared();
    }
}
extension Point: UnitizedFlatNormSquared {
    associatedtype Output = float;
    func unitized_flat_norm_squared() -> Output {
        return this.flat_bulk_norm_squared().scalar / (this.flat_weight_norm_squared().e1234);
    }
}

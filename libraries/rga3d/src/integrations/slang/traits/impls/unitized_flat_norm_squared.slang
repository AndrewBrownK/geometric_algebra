using traits::FlatBulkNormSquared;
using traits::FlatWeightNormSquared;
extension Flector: UnitizedFlatNormSquared {
    associatedtype Output = float;
    func unitized_flat_norm_squared() -> Output {
        return this.flat_bulk_norm_squared().scalar / (this.flat_weight_norm_squared().e1234);
    }
    func operator/(_rhs: UnitizedFlatNormSquaredInfix) -> float {
        return this.unitized_flat_norm_squared();
    }
}
extension Line: UnitizedFlatNormSquared {
    associatedtype Output = float;
    func unitized_flat_norm_squared() -> Output {
        return this.flat_bulk_norm_squared().scalar / (this.flat_weight_norm_squared().e1234);
    }
    func operator/(_rhs: UnitizedFlatNormSquaredInfix) -> float {
        return this.unitized_flat_norm_squared();
    }
}
extension Motor: UnitizedFlatNormSquared {
    associatedtype Output = float;
    func unitized_flat_norm_squared() -> Output {
        return this.flat_bulk_norm_squared().scalar / (this.flat_weight_norm_squared().e1234);
    }
    func operator/(_rhs: UnitizedFlatNormSquaredInfix) -> float {
        return this.unitized_flat_norm_squared();
    }
}
extension MultiVector: UnitizedFlatNormSquared {
    associatedtype Output = float;
    func unitized_flat_norm_squared() -> Output {
        return this.flat_bulk_norm_squared().scalar / (this.flat_weight_norm_squared().e1234);
    }
    func operator/(_rhs: UnitizedFlatNormSquaredInfix) -> float {
        return this.unitized_flat_norm_squared();
    }
}
extension Point: UnitizedFlatNormSquared {
    associatedtype Output = float;
    func unitized_flat_norm_squared() -> Output {
        return this.flat_bulk_norm_squared().scalar / (this.flat_weight_norm_squared().e1234);
    }
    func operator/(_rhs: UnitizedFlatNormSquaredInfix) -> float {
        return this.unitized_flat_norm_squared();
    }
}

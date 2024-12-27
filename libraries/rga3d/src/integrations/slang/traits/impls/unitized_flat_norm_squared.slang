using traits::FlatBulkNormSquared;
using traits::FlatWeightNormSquared;
extension Flector {
    func div(_rhs: unitized_flat_norm_squared) -> 
float {
        this.unitized_flat_norm_squared()
    }
}
extension Flector: UnitizedFlatNormSquared {
    func unitized_flat_norm_squared() -> float {
        return this.flat_bulk_norm_squared().scalar / (this.flat_weight_norm_squared().e1234);
    }
}
extension Line {
    func div(_rhs: unitized_flat_norm_squared) -> 
float {
        this.unitized_flat_norm_squared()
    }
}
extension Line: UnitizedFlatNormSquared {
    func unitized_flat_norm_squared() -> float {
        return this.flat_bulk_norm_squared().scalar / (this.flat_weight_norm_squared().e1234);
    }
}
extension Motor {
    func div(_rhs: unitized_flat_norm_squared) -> 
float {
        this.unitized_flat_norm_squared()
    }
}
extension Motor: UnitizedFlatNormSquared {
    func unitized_flat_norm_squared() -> float {
        return this.flat_bulk_norm_squared().scalar / (this.flat_weight_norm_squared().e1234);
    }
}
extension MultiVector {
    func div(_rhs: unitized_flat_norm_squared) -> 
float {
        this.unitized_flat_norm_squared()
    }
}
extension MultiVector: UnitizedFlatNormSquared {
    func unitized_flat_norm_squared() -> float {
        return this.flat_bulk_norm_squared().scalar / (this.flat_weight_norm_squared().e1234);
    }
}
extension Point {
    func div(_rhs: unitized_flat_norm_squared) -> 
float {
        this.unitized_flat_norm_squared()
    }
}
extension Point: UnitizedFlatNormSquared {
    func unitized_flat_norm_squared() -> float {
        return this.flat_bulk_norm_squared().scalar / (this.flat_weight_norm_squared().e1234);
    }
}

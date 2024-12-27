using traits::UnitizedFlatNormSquared;
extension Flector: UnitizedNorm {
    associatedtype Output = float;
    func unitized_norm() -> Output {
        return powf(this.unitized_flat_norm_squared(), 0.5);
    }
    func operator/(_rhs: UnitizedNormInfix) -> float {
        return this.unitized_norm();
    }
}
extension Line: UnitizedNorm {
    associatedtype Output = float;
    func unitized_norm() -> Output {
        return powf(this.unitized_flat_norm_squared(), 0.5);
    }
    func operator/(_rhs: UnitizedNormInfix) -> float {
        return this.unitized_norm();
    }
}
extension Motor: UnitizedNorm {
    associatedtype Output = float;
    func unitized_norm() -> Output {
        return powf(this.unitized_flat_norm_squared(), 0.5);
    }
    func operator/(_rhs: UnitizedNormInfix) -> float {
        return this.unitized_norm();
    }
}
extension MultiVector: UnitizedNorm {
    associatedtype Output = float;
    func unitized_norm() -> Output {
        return powf(this.unitized_flat_norm_squared(), 0.5);
    }
    func operator/(_rhs: UnitizedNormInfix) -> float {
        return this.unitized_norm();
    }
}
extension Point: UnitizedNorm {
    associatedtype Output = float;
    func unitized_norm() -> Output {
        return powf(this.unitized_flat_norm_squared(), 0.5);
    }
    func operator/(_rhs: UnitizedNormInfix) -> float {
        return this.unitized_norm();
    }
}

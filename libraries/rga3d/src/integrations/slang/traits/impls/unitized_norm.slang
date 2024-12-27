using traits::UnitizedFlatNormSquared;
extension Flector {
    // Fancy infix trick
    func operator/(_rhs: UnitizedNormPrefixOrPostfix) -> float {
        return this.unitized_norm();
    }
}
extension Flector: UnitizedNorm {
    associatedtype Output = float;
    func unitized_norm() -> Output {
        return powf(this.unitized_flat_norm_squared(), 0.5);
    }
}
extension Line {
    // Fancy infix trick
    func operator/(_rhs: UnitizedNormPrefixOrPostfix) -> float {
        return this.unitized_norm();
    }
}
extension Line: UnitizedNorm {
    associatedtype Output = float;
    func unitized_norm() -> Output {
        return powf(this.unitized_flat_norm_squared(), 0.5);
    }
}
extension Motor {
    // Fancy infix trick
    func operator/(_rhs: UnitizedNormPrefixOrPostfix) -> float {
        return this.unitized_norm();
    }
}
extension Motor: UnitizedNorm {
    associatedtype Output = float;
    func unitized_norm() -> Output {
        return powf(this.unitized_flat_norm_squared(), 0.5);
    }
}
extension MultiVector {
    // Fancy infix trick
    func operator/(_rhs: UnitizedNormPrefixOrPostfix) -> float {
        return this.unitized_norm();
    }
}
extension MultiVector: UnitizedNorm {
    associatedtype Output = float;
    func unitized_norm() -> Output {
        return powf(this.unitized_flat_norm_squared(), 0.5);
    }
}
extension Point {
    // Fancy infix trick
    func operator/(_rhs: UnitizedNormPrefixOrPostfix) -> float {
        return this.unitized_norm();
    }
}
extension Point: UnitizedNorm {
    associatedtype Output = float;
    func unitized_norm() -> Output {
        return powf(this.unitized_flat_norm_squared(), 0.5);
    }
}

using data::*;

/// UnitizedNorm
/// Unitized FlatNorm.
public interface UnitizedNorm {
    func unitized_norm() -> float;
}
public struct unitized_norm {}
extension unitized_norm for A: UnitizedNorm {
    func operator/(rhs: A) -> A.Output {
        return rhs.unitized_norm();
    }
}
__include ./impls/unitized_norm;

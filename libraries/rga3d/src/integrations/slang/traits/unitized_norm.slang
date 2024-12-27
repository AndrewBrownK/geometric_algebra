using data::*;

/// UnitizedNorm
/// Unitized FlatNorm.
public interface UnitizedNorm {
    func unitized_norm() -> float;
}
public const static unitized_norm: UnitizedNormInfix = UnitizedNormInfix {}
public struct UnitizedNormInfix {}
extension UnitizedNormInfix for A: UnitizedNorm {
    func operator/(rhs: A) -> A.Output {
        return rhs.unitized_norm();
    }
}
__include ./impls/unitized_norm;

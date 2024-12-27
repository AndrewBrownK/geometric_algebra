using data::*;

/// UnitizedNorm
/// Unitized FlatNorm.
public interface UnitizedNorm {
    func unitized_norm() -> float;
}
public static const unitized_norm: UnitizedNormPrefixOrPostfix = UnitizedNormPrefixOrPostfix {};
public struct UnitizedNormPrefixOrPostfix {}
extension UnitizedNormPrefixOrPostfix for A: UnitizedNorm {
    func operator/(rhs: A) -> A.Output {
        return rhs.unitized_norm();
    }
}
__include ./impls/unitized_norm;

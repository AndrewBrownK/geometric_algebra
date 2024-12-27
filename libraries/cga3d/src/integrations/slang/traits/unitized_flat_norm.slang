using data::*;

/// UnitizedFlatNorm
/// Unitized FlatNorm.
public interface UnitizedFlatNorm {
    func unitized_flat_norm() -> float;
}
public static const unitized_flat_norm: UnitizedFlatNormPrefixOrPostfix = UnitizedFlatNormPrefixOrPostfix {};
public struct UnitizedFlatNormPrefixOrPostfix {}
extension UnitizedFlatNormPrefixOrPostfix for A: UnitizedFlatNorm {
    func operator/(rhs: A) -> A.Output {
        return rhs.unitized_flat_norm();
    }
}
__include ./impls/unitized_flat_norm;

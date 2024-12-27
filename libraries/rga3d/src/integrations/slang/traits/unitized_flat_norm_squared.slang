using data::*;

/// UnitizedFlatNormSquared
/// Intermediate result to UnitizedFlatNorm.
public interface UnitizedFlatNormSquared {
    func unitized_flat_norm_squared() -> float;
}
public static const unitized_flat_norm_squared: UnitizedFlatNormSquaredPrefixOrPostfix = UnitizedFlatNormSquaredPrefixOrPostfix {};
public struct UnitizedFlatNormSquaredPrefixOrPostfix {}
extension UnitizedFlatNormSquaredPrefixOrPostfix for A: UnitizedFlatNormSquared {
    func operator/(rhs: A) -> A.Output {
        return rhs.unitized_flat_norm_squared();
    }
}
__include ./impls/unitized_flat_norm_squared;

using data::*;

/// UnitizedFlatNormSquared
/// Intermediate result to UnitizedFlatNorm.
public interface UnitizedFlatNormSquared {
    func unitized_flat_norm_squared() -> float;
}
public const static unitized_flat_norm_squared: UnitizedFlatNormSquaredInfix = UnitizedFlatNormSquaredInfix {}
public struct UnitizedFlatNormSquaredInfix {}
extension UnitizedFlatNormSquaredInfix for A: UnitizedFlatNormSquared {
    func operator/(rhs: A) -> A.Output {
        return rhs.unitized_flat_norm_squared();
    }
}
__include ./impls/unitized_flat_norm_squared;

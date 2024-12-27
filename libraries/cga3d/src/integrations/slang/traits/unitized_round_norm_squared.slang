using data::*;

/// UnitizedRoundNormSquared
/// Intermediate result to UnitizedRoundNorm.
public interface UnitizedRoundNormSquared {
    func unitized_round_norm_squared() -> float;
}
public static const unitized_round_norm_squared: UnitizedRoundNormSquaredPrefixOrPostfix = UnitizedRoundNormSquaredPrefixOrPostfix {};
public struct UnitizedRoundNormSquaredPrefixOrPostfix {}
extension UnitizedRoundNormSquaredPrefixOrPostfix for A: UnitizedRoundNormSquared {
    func operator/(rhs: A) -> A.Output {
        return rhs.unitized_round_norm_squared();
    }
}
__include ./impls/unitized_round_norm_squared;

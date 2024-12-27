using data::*;

/// RoundWeightNormSquared
/// Intermediate result for RoundWeight.
public interface RoundWeightNormSquared {
    func round_weight_norm_squared() -> AntiScalar;
}
public static const round_weight_norm_squared: RoundWeightNormSquaredPrefixOrPostfix = RoundWeightNormSquaredPrefixOrPostfix {};
public struct RoundWeightNormSquaredPrefixOrPostfix {}
extension RoundWeightNormSquaredPrefixOrPostfix for A: RoundWeightNormSquared {
    func operator/(rhs: A) -> A.Output {
        return rhs.round_weight_norm_squared();
    }
}
__include ./impls/round_weight_norm_squared;

using data::*;

/// FlatWeightNormSquared
/// Intermediate result to FlatWeightNorm.
public interface FlatWeightNormSquared {
    func flat_weight_norm_squared() -> AntiScalar;
}
public static const flat_weight_norm_squared: FlatWeightNormSquaredPrefixOrPostfix = FlatWeightNormSquaredPrefixOrPostfix {};
public struct FlatWeightNormSquaredPrefixOrPostfix {}
extension FlatWeightNormSquaredPrefixOrPostfix for A: FlatWeightNormSquared {
    func operator/(rhs: A) -> A.Output {
        return rhs.flat_weight_norm_squared();
    }
}
__include ./impls/flat_weight_norm_squared;

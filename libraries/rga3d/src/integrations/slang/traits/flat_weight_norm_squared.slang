implementing rga3d;using data;

/// FlatWeightNormSquared
/// Intermediate result to FlatWeightNorm.
public interface FlatWeightNormSquared {
    func flat_weight_norm_squared() -> AntiScalar;
}
public static const FlatWeightNormSquaredPrefixOrPostfix flat_weight_norm_squared = FlatWeightNormSquaredPrefixOrPostfix();
public struct FlatWeightNormSquaredPrefixOrPostfix {}
__include "./impls/flat_weight_norm_squared";

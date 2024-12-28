implementing rga3d;using data;

/// WeightNormSquared
/// Intermediate result to FlatWeightNorm.
public interface WeightNormSquared {
    func weight_norm_squared() -> AntiScalar;
}
public static const WeightNormSquaredPrefixOrPostfix weight_norm_squared = WeightNormSquaredPrefixOrPostfix();
public struct WeightNormSquaredPrefixOrPostfix {}
__include "./impls/weight_norm_squared";

using data::*;

/// WeightNormSquared
/// Intermediate result to FlatWeightNorm.
public interface WeightNormSquared {
    func weight_norm_squared() -> AntiScalar;
}
public static const weight_norm_squared: WeightNormSquaredPrefixOrPostfix = WeightNormSquaredPrefixOrPostfix {};
public struct WeightNormSquaredPrefixOrPostfix {}
extension WeightNormSquaredPrefixOrPostfix for A: WeightNormSquared {
    func operator/(rhs: A) -> A.Output {
        return rhs.weight_norm_squared();
    }
}
__include ./impls/weight_norm_squared;

using data::*;

/// WeightNormSquared
/// Intermediate result to FlatWeightNorm.
public interface WeightNormSquared {
    func weight_norm_squared() -> AntiScalar;
}
public const static weight_norm_squared: WeightNormSquaredInfix = WeightNormSquaredInfix {}
public struct WeightNormSquaredInfix {}
extension WeightNormSquaredInfix for A: WeightNormSquared {
    func operator/(rhs: A) -> A.Output {
        return rhs.weight_norm_squared();
    }
}
__include ./impls/weight_norm_squared;

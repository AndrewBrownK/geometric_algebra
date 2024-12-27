using data::*;

/// WeightNormSquared
/// Intermediate result to FlatWeightNorm.
public interface WeightNormSquared {
    func weight_norm_squared() -> AntiScalar;
}
public struct weight_norm_squared {}
extension weight_norm_squared for A: WeightNormSquared {
    func operator/(rhs: A) -> A.Output {
        return rhs.weight_norm_squared();
    }
}
__include ./impls/weight_norm_squared;

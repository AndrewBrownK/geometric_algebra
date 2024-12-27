using data::*;

/// FlatWeightNormSquared
/// Intermediate result to FlatWeightNorm.
public interface FlatWeightNormSquared {
    func flat_weight_norm_squared() -> AntiScalar;
}
public struct flat_weight_norm_squared {}
extension flat_weight_norm_squared for A: FlatWeightNormSquared {
    func operator/(rhs: A) -> A.Output {
        return rhs.flat_weight_norm_squared();
    }
}
__include ./impls/flat_weight_norm_squared;

using data::*;

/// FlatWeightNormSquared
/// Intermediate result to FlatWeightNorm.
public interface FlatWeightNormSquared {
    fn flat_weight_norm_squared() -> AntiScalar;
}
public struct flat_weight_norm_squared;
extension flat_weight_norm_squared {    associatedtype Output = AntiScalar;
    func operator/(rhs: A) -> Self::Output {
        rhs.flat_weight_norm_squared()
    }
}
__include ./impls/flat_weight_norm_squared;

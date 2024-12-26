using data::*;

/// WeightNormSquared
/// Intermediate result to FlatWeightNorm.
public interface WeightNormSquared {
    fn weight_norm_squared() -> AntiScalar;
}
public struct weight_norm_squared;
extension weight_norm_squared {    associatedtype Output = AntiScalar;
    func operator/(rhs: A) -> Self::Output {
        rhs.weight_norm_squared()
    }
}
__include ./impls/weight_norm_squared;

using data::*;

/// FlatWeightNorm
/// Weight Norm for flat aspect.
public interface FlatWeightNorm {
    func flat_weight_norm() -> AntiScalar;
}
public struct flat_weight_norm {}
extension flat_weight_norm for A: FlatWeightNorm {
    func operator/(rhs: A) -> A.Output {
        return rhs.flat_weight_norm();
    }
}
__include ./impls/flat_weight_norm;

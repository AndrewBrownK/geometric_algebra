using data::*;

/// FlatWeightNorm
/// Weight Norm for flat aspect.
public interface FlatWeightNorm {
    func flat_weight_norm() -> AntiScalar;
}
public static const flat_weight_norm: FlatWeightNormPrefixOrPostfix = FlatWeightNormPrefixOrPostfix {};
public struct FlatWeightNormPrefixOrPostfix {}
extension FlatWeightNormPrefixOrPostfix for A: FlatWeightNorm {
    func operator/(rhs: A) -> A.Output {
        return rhs.flat_weight_norm();
    }
}
__include ./impls/flat_weight_norm;

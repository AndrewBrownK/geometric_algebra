using data::*;

/// FlatWeightNorm
/// Weight Norm for flat aspect.
public interface FlatWeightNorm {
    func flat_weight_norm() -> AntiScalar;
}
public const static flat_weight_norm: FlatWeightNormInfix = FlatWeightNormInfix {}
public struct FlatWeightNormInfix {}
extension FlatWeightNormInfix for A: FlatWeightNorm {
    func operator/(rhs: A) -> A.Output {
        return rhs.flat_weight_norm();
    }
}
__include ./impls/flat_weight_norm;

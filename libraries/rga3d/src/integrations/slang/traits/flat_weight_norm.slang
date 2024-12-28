implementing rga3d;using data;

/// FlatWeightNorm
/// Weight Norm for flat aspect.
public interface FlatWeightNorm {
    func flat_weight_norm() -> AntiScalar;
}
public static const FlatWeightNormPrefixOrPostfix flat_weight_norm = FlatWeightNormPrefixOrPostfix();
public struct FlatWeightNormPrefixOrPostfix {}
__include "./impls/flat_weight_norm";

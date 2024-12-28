implementing rga3d;using data;

/// WeightNorm
/// Weight Norm for flat aspect.
public interface WeightNorm {
    func weight_norm() -> AntiScalar;
}
public static const WeightNormPrefixOrPostfix weight_norm = WeightNormPrefixOrPostfix();
public struct WeightNormPrefixOrPostfix {}
__include "./impls/weight_norm";

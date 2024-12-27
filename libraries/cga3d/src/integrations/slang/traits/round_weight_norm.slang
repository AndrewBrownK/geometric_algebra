using data::*;

/// RoundWeightNorm
/// Weight Norm for round aspect.
public interface RoundWeightNorm {
    func round_weight_norm() -> AntiScalar;
}
public static const round_weight_norm: RoundWeightNormPrefixOrPostfix = RoundWeightNormPrefixOrPostfix {};
public struct RoundWeightNormPrefixOrPostfix {}
extension RoundWeightNormPrefixOrPostfix for A: RoundWeightNorm {
    func operator/(rhs: A) -> A.Output {
        return rhs.round_weight_norm();
    }
}
__include ./impls/round_weight_norm;

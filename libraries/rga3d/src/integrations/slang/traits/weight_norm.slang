using data::*;

/// WeightNorm
/// Weight Norm for flat aspect.
public interface WeightNorm {
    func weight_norm() -> AntiScalar;
}
public static const weight_norm: WeightNormPrefixOrPostfix = WeightNormPrefixOrPostfix {};
public struct WeightNormPrefixOrPostfix {}
extension WeightNormPrefixOrPostfix for A: WeightNorm {
    func operator/(rhs: A) -> A.Output {
        return rhs.weight_norm();
    }
}
__include ./impls/weight_norm;

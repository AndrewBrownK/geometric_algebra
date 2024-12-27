using data::*;

/// WeightNorm
/// Weight Norm for flat aspect.
public interface WeightNorm {
    func weight_norm() -> AntiScalar;
}
public const static weight_norm: WeightNormInfix = WeightNormInfix {}
public struct WeightNormInfix {}
extension WeightNormInfix for A: WeightNorm {
    func operator/(rhs: A) -> A.Output {
        return rhs.weight_norm();
    }
}
__include ./impls/weight_norm;

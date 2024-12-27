using data::*;

/// WeightNorm
/// Weight Norm for flat aspect.
public interface WeightNorm {
    func weight_norm() -> AntiScalar;
}
public struct weight_norm {}
extension weight_norm for A: WeightNorm {
    func operator/(rhs: A) -> A.Output {
        return rhs.weight_norm();
    }
}
__include ./impls/weight_norm;

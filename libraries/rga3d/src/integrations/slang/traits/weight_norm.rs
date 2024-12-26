using data::*;

/// WeightNorm
/// Weight Norm for flat aspect.
public interface WeightNorm {
    fn weight_norm() -> AntiScalar;
}
public struct weight_norm;
extension weight_norm {    associatedtype Output = AntiScalar;
    func operator/(rhs: A) -> Self::Output {
        rhs.weight_norm()
    }
}
__include ./impls/weight_norm;

using data::*;

/// FlatWeightNorm
/// Weight Norm for flat aspect.
public interface FlatWeightNorm {
    fn flat_weight_norm() -> AntiScalar;
}
public struct flat_weight_norm;
extension flat_weight_norm {    associatedtype Output = AntiScalar;
    func operator/(rhs: A) -> Self::Output {
        rhs.flat_weight_norm()
    }
}
__include ./impls/flat_weight_norm;

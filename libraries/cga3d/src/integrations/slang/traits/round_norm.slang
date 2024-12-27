using data::*;

/// RoundNorm
/// Norm for Round aspect.
public interface RoundNorm {
    func round_norm() -> MultiVector;
}
public static const round_norm: RoundNormPrefixOrPostfix = RoundNormPrefixOrPostfix {};
public struct RoundNormPrefixOrPostfix {}
extension RoundNormPrefixOrPostfix for A: RoundNorm {
    func operator/(rhs: A) -> A.Output {
        return rhs.round_norm();
    }
}
__include ./impls/round_norm;

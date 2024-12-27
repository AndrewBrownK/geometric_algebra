using data::*;

/// RoundNormSquared
/// Intermediate result for RoundNorm.
public interface RoundNormSquared {
    func round_norm_squared() -> MultiVector;
}
public static const round_norm_squared: RoundNormSquaredPrefixOrPostfix = RoundNormSquaredPrefixOrPostfix {};
public struct RoundNormSquaredPrefixOrPostfix {}
extension RoundNormSquaredPrefixOrPostfix for A: RoundNormSquared {
    func operator/(rhs: A) -> A.Output {
        return rhs.round_norm_squared();
    }
}
__include ./impls/round_norm_squared;

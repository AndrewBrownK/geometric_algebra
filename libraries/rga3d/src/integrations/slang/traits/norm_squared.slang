using data::*;

/// NormSquared
/// Intermediate result to FlatNorm.
public interface NormSquared {
    func norm_squared() -> DualNum;
}
public const static norm_squared: NormSquaredInfix = NormSquaredInfix {}
public struct NormSquaredInfix {}
extension NormSquaredInfix for A: NormSquared {
    func operator/(rhs: A) -> A.Output {
        return rhs.norm_squared();
    }
}
__include ./impls/norm_squared;

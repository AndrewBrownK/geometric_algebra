using data::*;

/// NormSquared
/// Intermediate result to FlatNorm.
public interface NormSquared {
    func norm_squared() -> DualNum;
}
public static const norm_squared: NormSquaredPrefixOrPostfix = NormSquaredPrefixOrPostfix {};
public struct NormSquaredPrefixOrPostfix {}
extension NormSquaredPrefixOrPostfix for A: NormSquared {
    func operator/(rhs: A) -> A.Output {
        return rhs.norm_squared();
    }
}
__include ./impls/norm_squared;

using data::*;

/// FlatNormSquared
/// Intermediate result to FlatNorm.
public interface FlatNormSquared {
    func flat_norm_squared() -> MultiVector;
}
public static const flat_norm_squared: FlatNormSquaredPrefixOrPostfix = FlatNormSquaredPrefixOrPostfix {};
public struct FlatNormSquaredPrefixOrPostfix {}
extension FlatNormSquaredPrefixOrPostfix for A: FlatNormSquared {
    func operator/(rhs: A) -> A.Output {
        return rhs.flat_norm_squared();
    }
}
__include ./impls/flat_norm_squared;

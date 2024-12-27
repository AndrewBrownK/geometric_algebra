using data::*;

/// CenterNormSquared
/// Intermediate result to CenterNorm.
public interface CenterNormSquared {
    func center_norm_squared() -> Scalar;
}
public static const center_norm_squared: CenterNormSquaredPrefixOrPostfix = CenterNormSquaredPrefixOrPostfix {};
public struct CenterNormSquaredPrefixOrPostfix {}
extension CenterNormSquaredPrefixOrPostfix for A: CenterNormSquared {
    func operator/(rhs: A) -> A.Output {
        return rhs.center_norm_squared();
    }
}
__include ./impls/center_norm_squared;

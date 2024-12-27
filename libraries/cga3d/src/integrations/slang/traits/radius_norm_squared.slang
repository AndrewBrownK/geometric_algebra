using data::*;

/// RadiusNormSquared
/// Intermediate result to RadiusNorm.
public interface RadiusNormSquared {
    func radius_norm_squared() -> Scalar;
}
public static const radius_norm_squared: RadiusNormSquaredPrefixOrPostfix = RadiusNormSquaredPrefixOrPostfix {};
public struct RadiusNormSquaredPrefixOrPostfix {}
extension RadiusNormSquaredPrefixOrPostfix for A: RadiusNormSquared {
    func operator/(rhs: A) -> A.Output {
        return rhs.radius_norm_squared();
    }
}
__include ./impls/radius_norm_squared;

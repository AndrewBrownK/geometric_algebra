using data::*;

/// RadiusNorm
/// Distance radius of a round object (not yet unitized, still requires division by round weight).
public interface RadiusNorm {
    func radius_norm() -> Scalar;
}
public static const radius_norm: RadiusNormPrefixOrPostfix = RadiusNormPrefixOrPostfix {};
public struct RadiusNormPrefixOrPostfix {}
extension RadiusNormPrefixOrPostfix for A: RadiusNorm {
    func operator/(rhs: A) -> A.Output {
        return rhs.radius_norm();
    }
}
__include ./impls/radius_norm;

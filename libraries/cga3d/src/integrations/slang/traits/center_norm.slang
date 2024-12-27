using data::*;

/// CenterNorm
/// Distance between origin and center (not yet unitized, still requires division by round weight).
public interface CenterNorm {
    func center_norm() -> Scalar;
}
public static const center_norm: CenterNormPrefixOrPostfix = CenterNormPrefixOrPostfix {};
public struct CenterNormPrefixOrPostfix {}
extension CenterNormPrefixOrPostfix for A: CenterNorm {
    func operator/(rhs: A) -> A.Output {
        return rhs.center_norm();
    }
}
__include ./impls/center_norm;

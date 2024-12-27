using data::*;

/// Norm
/// Norm for flat aspect.
public interface Norm {
    func norm() -> DualNum;
}
public static const norm: NormPrefixOrPostfix = NormPrefixOrPostfix {};
public struct NormPrefixOrPostfix {}
extension NormPrefixOrPostfix for A: Norm {
    func operator/(rhs: A) -> A.Output {
        return rhs.norm();
    }
}
__include ./impls/norm;

using data::*;

/// Norm
/// Norm for flat aspect.
public interface Norm {
    func norm() -> DualNum;
}
public const static norm: NormInfix = NormInfix {}
public struct NormInfix {}
extension NormInfix for A: Norm {
    func operator/(rhs: A) -> A.Output {
        return rhs.norm();
    }
}
__include ./impls/norm;

using data::*;

/// FlatNorm
/// Norm for flat aspect.
public interface FlatNorm {
    func flat_norm() -> MultiVector;
}
public static const flat_norm: FlatNormPrefixOrPostfix = FlatNormPrefixOrPostfix {};
public struct FlatNormPrefixOrPostfix {}
extension FlatNormPrefixOrPostfix for A: FlatNorm {
    func operator/(rhs: A) -> A.Output {
        return rhs.flat_norm();
    }
}
__include ./impls/flat_norm;

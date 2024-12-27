using data::*;

/// FlatBulkNorm
/// BulkNorm for flat aspect.
public interface FlatBulkNorm {
    func flat_bulk_norm() -> Scalar;
}
public static const flat_bulk_norm: FlatBulkNormPrefixOrPostfix = FlatBulkNormPrefixOrPostfix {};
public struct FlatBulkNormPrefixOrPostfix {}
extension FlatBulkNormPrefixOrPostfix for A: FlatBulkNorm {
    func operator/(rhs: A) -> A.Output {
        return rhs.flat_bulk_norm();
    }
}
__include ./impls/flat_bulk_norm;

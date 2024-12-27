using data::*;

/// BulkNorm
/// BulkNorm for flat aspect.
public interface BulkNorm {
    func bulk_norm() -> Scalar;
}
public static const bulk_norm: BulkNormPrefixOrPostfix = BulkNormPrefixOrPostfix {};
public struct BulkNormPrefixOrPostfix {}
extension BulkNormPrefixOrPostfix for A: BulkNorm {
    func operator/(rhs: A) -> A.Output {
        return rhs.bulk_norm();
    }
}
__include ./impls/bulk_norm;

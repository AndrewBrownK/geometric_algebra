using data::*;

/// BulkNorm
/// BulkNorm for flat aspect.
public interface BulkNorm {
    func bulk_norm() -> Scalar;
}
public const static bulk_norm: BulkNormInfix = BulkNormInfix {}
public struct BulkNormInfix {}
extension BulkNormInfix for A: BulkNorm {
    func operator/(rhs: A) -> A.Output {
        return rhs.bulk_norm();
    }
}
__include ./impls/bulk_norm;

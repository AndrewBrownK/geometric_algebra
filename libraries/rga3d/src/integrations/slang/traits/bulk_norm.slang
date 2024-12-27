using data::*;

/// BulkNorm
/// BulkNorm for flat aspect.
public interface BulkNorm {
    func bulk_norm() -> Scalar;
}
public struct bulk_norm {}
extension bulk_norm for A: BulkNorm {
    func operator/(rhs: A) -> A.Output {
        return rhs.bulk_norm();
    }
}
__include ./impls/bulk_norm;

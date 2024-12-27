using data::*;

/// FlatBulkNorm
/// BulkNorm for flat aspect.
public interface FlatBulkNorm {
    func flat_bulk_norm() -> Scalar;
}
public struct flat_bulk_norm {}
extension flat_bulk_norm for A: FlatBulkNorm {
    func operator/(rhs: A) -> A.Output {
        return rhs.flat_bulk_norm();
    }
}
__include ./impls/flat_bulk_norm;

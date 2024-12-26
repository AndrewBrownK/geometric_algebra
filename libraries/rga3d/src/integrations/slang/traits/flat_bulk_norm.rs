using data::*;

/// FlatBulkNorm
/// BulkNorm for flat aspect.
public interface FlatBulkNorm {
    fn flat_bulk_norm() -> Scalar;
}
public struct flat_bulk_norm;
extension flat_bulk_norm {    associatedtype Output = Scalar;
    func operator/(rhs: A) -> Self::Output {
        rhs.flat_bulk_norm()
    }
}
__include ./impls/flat_bulk_norm;

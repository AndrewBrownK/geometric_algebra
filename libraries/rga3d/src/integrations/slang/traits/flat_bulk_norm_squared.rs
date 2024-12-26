using data::*;

/// FlatBulkNormSquared
/// Intermediate result for FlatBulkNorm.
public interface FlatBulkNormSquared {
    fn flat_bulk_norm_squared() -> Scalar;
}
public struct flat_bulk_norm_squared;
extension flat_bulk_norm_squared {    associatedtype Output = Scalar;
    func operator/(rhs: A) -> Self::Output {
        rhs.flat_bulk_norm_squared()
    }
}
__include ./impls/flat_bulk_norm_squared;

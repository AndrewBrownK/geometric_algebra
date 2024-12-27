using data::*;

/// FlatBulkNormSquared
/// Intermediate result for FlatBulkNorm.
public interface FlatBulkNormSquared {
    func flat_bulk_norm_squared() -> Scalar;
}
public struct flat_bulk_norm_squared {}
extension flat_bulk_norm_squared for A: FlatBulkNormSquared {
    func operator/(rhs: A) -> A.Output {
        return rhs.flat_bulk_norm_squared();
    }
}
__include ./impls/flat_bulk_norm_squared;

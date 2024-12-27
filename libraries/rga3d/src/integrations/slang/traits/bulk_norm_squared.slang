using data::*;

/// BulkNormSquared
/// Intermediate result for FlatBulkNorm.
public interface BulkNormSquared {
    func bulk_norm_squared() -> Scalar;
}
public struct bulk_norm_squared {}
extension bulk_norm_squared for A: BulkNormSquared {
    func operator/(rhs: A) -> A.Output {
        return rhs.bulk_norm_squared();
    }
}
__include ./impls/bulk_norm_squared;

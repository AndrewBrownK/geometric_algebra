using data::*;

/// BulkNormSquared
/// Intermediate result for FlatBulkNorm.
public interface BulkNormSquared {
    func bulk_norm_squared() -> Scalar;
}
public const static bulk_norm_squared: BulkNormSquaredInfix = BulkNormSquaredInfix {}
public struct BulkNormSquaredInfix {}
extension BulkNormSquaredInfix for A: BulkNormSquared {
    func operator/(rhs: A) -> A.Output {
        return rhs.bulk_norm_squared();
    }
}
__include ./impls/bulk_norm_squared;

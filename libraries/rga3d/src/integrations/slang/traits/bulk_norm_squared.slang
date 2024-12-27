using data::*;

/// BulkNormSquared
/// Intermediate result for FlatBulkNorm.
public interface BulkNormSquared {
    func bulk_norm_squared() -> Scalar;
}
public static const bulk_norm_squared: BulkNormSquaredPrefixOrPostfix = BulkNormSquaredPrefixOrPostfix {};
public struct BulkNormSquaredPrefixOrPostfix {}
extension BulkNormSquaredPrefixOrPostfix for A: BulkNormSquared {
    func operator/(rhs: A) -> A.Output {
        return rhs.bulk_norm_squared();
    }
}
__include ./impls/bulk_norm_squared;

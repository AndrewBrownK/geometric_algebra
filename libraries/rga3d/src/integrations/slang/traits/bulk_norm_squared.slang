implementing rga3d;using data;

/// BulkNormSquared
/// Intermediate result for FlatBulkNorm.
public interface BulkNormSquared {
    func bulk_norm_squared() -> Scalar;
}
public static const BulkNormSquaredPrefixOrPostfix bulk_norm_squared = BulkNormSquaredPrefixOrPostfix();
public struct BulkNormSquaredPrefixOrPostfix {}
__include "./impls/bulk_norm_squared";

implementing rga3d;using data;

/// FlatBulkNormSquared
/// Intermediate result for FlatBulkNorm.
public interface FlatBulkNormSquared {
    func flat_bulk_norm_squared() -> Scalar;
}
public static const FlatBulkNormSquaredPrefixOrPostfix flat_bulk_norm_squared = FlatBulkNormSquaredPrefixOrPostfix();
public struct FlatBulkNormSquaredPrefixOrPostfix {}
__include "./impls/flat_bulk_norm_squared";

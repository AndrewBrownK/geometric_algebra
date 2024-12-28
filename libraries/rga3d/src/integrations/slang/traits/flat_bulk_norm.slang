implementing rga3d;using data;

/// FlatBulkNorm
/// BulkNorm for flat aspect.
public interface FlatBulkNorm {
    func flat_bulk_norm() -> Scalar;
}
public static const FlatBulkNormPrefixOrPostfix flat_bulk_norm = FlatBulkNormPrefixOrPostfix();
public struct FlatBulkNormPrefixOrPostfix {}
__include "./impls/flat_bulk_norm";

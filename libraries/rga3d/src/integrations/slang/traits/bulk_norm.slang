implementing rga3d;using data;

/// BulkNorm
/// BulkNorm for flat aspect.
public interface BulkNorm {
    func bulk_norm() -> Scalar;
}
public static const BulkNormPrefixOrPostfix bulk_norm = BulkNormPrefixOrPostfix();
public struct BulkNormPrefixOrPostfix {}
__include "./impls/bulk_norm";

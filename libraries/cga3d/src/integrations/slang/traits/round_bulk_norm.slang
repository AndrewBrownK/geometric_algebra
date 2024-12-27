using data::*;

/// RoundBulkNorm
/// Bulk Norm for round aspect.
public interface RoundBulkNorm {
    func round_bulk_norm() -> Scalar;
}
public static const round_bulk_norm: RoundBulkNormPrefixOrPostfix = RoundBulkNormPrefixOrPostfix {};
public struct RoundBulkNormPrefixOrPostfix {}
extension RoundBulkNormPrefixOrPostfix for A: RoundBulkNorm {
    func operator/(rhs: A) -> A.Output {
        return rhs.round_bulk_norm();
    }
}
__include ./impls/round_bulk_norm;

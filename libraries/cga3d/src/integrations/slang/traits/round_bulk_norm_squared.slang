using data::*;

/// RoundBulkNormSquared
/// Intermediate result to RoundBulkNorm.
public interface RoundBulkNormSquared {
    func round_bulk_norm_squared() -> Scalar;
}
public static const round_bulk_norm_squared: RoundBulkNormSquaredPrefixOrPostfix = RoundBulkNormSquaredPrefixOrPostfix {};
public struct RoundBulkNormSquaredPrefixOrPostfix {}
extension RoundBulkNormSquaredPrefixOrPostfix for A: RoundBulkNormSquared {
    func operator/(rhs: A) -> A.Output {
        return rhs.round_bulk_norm_squared();
    }
}
__include ./impls/round_bulk_norm_squared;

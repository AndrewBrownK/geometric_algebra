using data::*;

/// FlatBulkNormSquared
/// Intermediate result for FlatBulkNorm.
public interface FlatBulkNormSquared {
    func flat_bulk_norm_squared() -> Scalar;
}
public const static flat_bulk_norm_squared: FlatBulkNormSquaredInfix = FlatBulkNormSquaredInfix {}
public struct FlatBulkNormSquaredInfix {}
extension FlatBulkNormSquaredInfix for A: FlatBulkNormSquared {
    func operator/(rhs: A) -> A.Output {
        return rhs.flat_bulk_norm_squared();
    }
}
__include ./impls/flat_bulk_norm_squared;

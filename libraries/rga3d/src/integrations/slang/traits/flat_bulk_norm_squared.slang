using data::*;

/// FlatBulkNormSquared
/// Intermediate result for FlatBulkNorm.
public interface FlatBulkNormSquared {
    func flat_bulk_norm_squared() -> Scalar;
}
public static const flat_bulk_norm_squared: FlatBulkNormSquaredPrefixOrPostfix = FlatBulkNormSquaredPrefixOrPostfix {};
public struct FlatBulkNormSquaredPrefixOrPostfix {}
extension FlatBulkNormSquaredPrefixOrPostfix for A: FlatBulkNormSquared {
    func operator/(rhs: A) -> A.Output {
        return rhs.flat_bulk_norm_squared();
    }
}
__include ./impls/flat_bulk_norm_squared;

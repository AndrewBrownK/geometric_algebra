using data::*;

/// BulkNormSquared
/// Intermediate result for FlatBulkNorm.
public interface BulkNormSquared {
    fn bulk_norm_squared() -> Scalar;
}
public struct bulk_norm_squared;
extension bulk_norm_squared {    associatedtype Output = Scalar;
    func operator/(rhs: A) -> Self::Output {
        rhs.bulk_norm_squared()
    }
}
__include ./impls/bulk_norm_squared;

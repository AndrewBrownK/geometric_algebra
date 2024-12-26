using data::*;

/// BulkNorm
/// BulkNorm for flat aspect.
public interface BulkNorm {
    fn bulk_norm() -> Scalar;
}
public struct bulk_norm;
extension bulk_norm {    associatedtype Output = Scalar;
    func operator/(rhs: A) -> Self::Output {
        rhs.bulk_norm()
    }
}
__include ./impls/bulk_norm;

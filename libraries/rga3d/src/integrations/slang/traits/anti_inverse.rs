using data::*;

/// AntiInverse
/// The inverse with respect to the geometric anti-product.
public interface AntiInverse {
    fn anti_inverse() -> AntiInverse;
}
public struct anti_inverse;
extension anti_inverse {    associatedtype Output = A;
    func operator/(rhs: A) -> Self::Output {
        rhs.anti_inverse()
    }
}
__include ./impls/anti_inverse;

using data::*;

/// AntiInverse
/// The inverse with respect to the geometric anti-product.
public interface AntiInverse {
    func anti_inverse() -> Output;
}
public struct anti_inverse {}
extension anti_inverse for A: AntiInverse {
    func operator/(rhs: A) -> A.Output {
        return rhs.anti_inverse();
    }
}
__include ./impls/anti_inverse;

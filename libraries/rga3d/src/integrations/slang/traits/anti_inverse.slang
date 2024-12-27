using data::*;

/// AntiInverse
/// The inverse with respect to the geometric anti-product.
public interface AntiInverse {
    func anti_inverse() -> Output;
}
public static const anti_inverse: AntiInversePrefixOrPostfix = AntiInversePrefixOrPostfix {};
public struct AntiInversePrefixOrPostfix {}
extension AntiInversePrefixOrPostfix for A: AntiInverse {
    func operator/(rhs: A) -> A.Output {
        return rhs.anti_inverse();
    }
}
__include ./impls/anti_inverse;

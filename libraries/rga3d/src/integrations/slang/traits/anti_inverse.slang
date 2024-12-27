using data::*;

/// AntiInverse
/// The inverse with respect to the geometric anti-product.
public interface AntiInverse {
    func anti_inverse() -> Output;
}
public const static anti_inverse: AntiInverseInfix = AntiInverseInfix {}
public struct AntiInverseInfix {}
extension AntiInverseInfix for A: AntiInverse {
    func operator/(rhs: A) -> A.Output {
        return rhs.anti_inverse();
    }
}
__include ./impls/anti_inverse;

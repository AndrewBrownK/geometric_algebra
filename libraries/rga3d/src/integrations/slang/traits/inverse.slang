using data::*;

/// Inverse
/// The inverse with respect to geometric product. Inverse(x) = x^-1.
public interface Inverse {
    func inverse() -> Output;
}
public static const inverse: InversePrefixOrPostfix = InversePrefixOrPostfix {};
public struct InversePrefixOrPostfix {}
extension InversePrefixOrPostfix for A: Inverse {
    func operator/(rhs: A) -> A.Output {
        return rhs.inverse();
    }
}
__include ./impls/inverse;

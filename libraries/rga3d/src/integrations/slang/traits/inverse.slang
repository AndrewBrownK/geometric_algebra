using data::*;

/// Inverse
/// The inverse with respect to geometric product. Inverse(x) = x^-1.
public interface Inverse {
    func inverse() -> Output;
}
public struct inverse {}
extension inverse for A: Inverse {
    func operator/(rhs: A) -> A.Output {
        return rhs.inverse();
    }
}
__include ./impls/inverse;

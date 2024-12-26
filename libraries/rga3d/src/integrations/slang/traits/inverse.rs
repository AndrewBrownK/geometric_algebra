using data::*;

/// Inverse
/// The inverse with respect to geometric product. Inverse(x) = x^-1.
public interface Inverse {
    fn inverse() -> Inverse;
}
public struct inverse;
extension inverse {    associatedtype Output = A;
    func operator/(rhs: A) -> Self::Output {
        rhs.inverse()
    }
}
__include ./impls/inverse;

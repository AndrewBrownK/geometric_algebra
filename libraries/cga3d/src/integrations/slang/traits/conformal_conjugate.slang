using data::*;

/// ConformalConjugate
/// The conformal conjugate negates the flat elements of an object, and is useful in calculating the center norm of the object.
public interface ConformalConjugate {
    func conformal_conjugate() -> Output;
}
public static const conformal_conjugate: ConformalConjugatePrefixOrPostfix = ConformalConjugatePrefixOrPostfix {};
public struct ConformalConjugatePrefixOrPostfix {}
extension ConformalConjugatePrefixOrPostfix for A: ConformalConjugate {
    func operator/(rhs: A) -> A.Output {
        return rhs.conformal_conjugate();
    }
}
__include ./impls/conformal_conjugate;

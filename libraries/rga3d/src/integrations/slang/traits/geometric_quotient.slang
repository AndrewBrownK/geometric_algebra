using data::*;

/// GeometricQuotient
/// Product of A with Inverse of B.
public interface GeometricQuotient<T> {
    associatedtype Output;
    func geometric_quotient(other: T) -> Output;
}
public const static geometric_quotient: GeometricQuotientInfix = GeometricQuotientInfix {}
public struct GeometricQuotientInfix {}
public struct GeometricQuotientInfixPartial<A> { a: A }
extension GeometricQuotientInfixPartial<A> for A: GeometricQuotient<B> {
    func operator/(rhs: B) -> A.Output {
        return this.a.geometric_quotient(rhs);
    }
}
__include ./impls/geometric_quotient;

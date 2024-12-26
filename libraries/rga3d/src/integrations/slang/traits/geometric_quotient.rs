using data::*;

/// GeometricQuotient
/// Product of A with Inverse of B.
public interface GeometricQuotient<T> {
    associatedtype Output;
    fn geometric_quotient(other: T) -> Self::Output;
}
public struct geometric_quotient;
public struct geometric_quotient_partial<A> { a: A }
extension geometric_quotient_partial<A> {    associatedtype Output = <A as GeometricQuotient<B>>::Output;
    func operator/(rhs: B) -> Self::Output {
        this.a.geometric_quotient(rhs)
    }
}
__include ./impls/geometric_quotient;

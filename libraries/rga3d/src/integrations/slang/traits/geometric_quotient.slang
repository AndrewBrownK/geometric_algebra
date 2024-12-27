using data::*;

/// GeometricQuotient
/// Product of A with Inverse of B.
public interface GeometricQuotient<T> {
    associatedtype Output;
    func geometric_quotient(other: T) -> Output;
}
public struct geometric_quotient {}
public struct geometric_quotient_partial<A> { a: A }
extension geometric_quotient_partial<A> for A: GeometricQuotient<B> {
    func operator/(rhs: B) -> A.Output {
        return this.a.geometric_quotient(rhs);
    }
}
__include ./impls/geometric_quotient;

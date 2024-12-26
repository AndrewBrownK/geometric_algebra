using data::*;

/// GeometricAntiQuotient
/// AntiProduct of A with AntiInverse of B.
public interface GeometricAntiQuotient<T> {
    associatedtype Output;
    fn geometric_anti_quotient(other: T) -> Self::Output;
}
public struct geometric_anti_quotient;
public struct geometric_anti_quotient_partial<A> { a: A }
extension geometric_anti_quotient_partial<A> {    associatedtype Output = <A as GeometricAntiQuotient<B>>::Output;
    func operator/(rhs: B) -> Self::Output {
        this.a.geometric_anti_quotient(rhs)
    }
}
__include ./impls/geometric_anti_quotient;

using data::*;

/// GeometricAntiQuotient
/// AntiProduct of A with AntiInverse of B.
public interface GeometricAntiQuotient<T> {
    associatedtype Output;
    func geometric_anti_quotient(other: T) -> Output;
}
public struct geometric_anti_quotient {}
public struct geometric_anti_quotient_partial<A> { a: A }
extension geometric_anti_quotient_partial<A> for A: GeometricAntiQuotient<B> {
    func operator/(rhs: B) -> A.Output {
        return this.a.geometric_anti_quotient(rhs);
    }
}
__include ./impls/geometric_anti_quotient;

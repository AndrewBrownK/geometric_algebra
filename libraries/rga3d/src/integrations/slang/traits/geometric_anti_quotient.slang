using data::*;

/// GeometricAntiQuotient
/// AntiProduct of A with AntiInverse of B.
public interface GeometricAntiQuotient<T> {
    associatedtype Output;
    func geometric_anti_quotient(other: T) -> Output;
}
public const static geometric_anti_quotient: GeometricAntiQuotientInfix = GeometricAntiQuotientInfix {}
public struct GeometricAntiQuotientInfix {}
public struct GeometricAntiQuotientInfixPartial<A> { a: A }
extension GeometricAntiQuotientInfixPartial<A> for A: GeometricAntiQuotient<B> {
    func operator/(rhs: B) -> A.Output {
        return this.a.geometric_anti_quotient(rhs);
    }
}
__include ./impls/geometric_anti_quotient;

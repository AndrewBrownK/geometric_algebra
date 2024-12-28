implementing rga3d;using data;

/// GeometricAntiQuotient
/// AntiProduct of A with AntiInverse of B.
public interface GeometricAntiQuotient<T> {
    associatedtype GeometricAntiQuotientOutput;
    func geometric_anti_quotient(other: T) -> Output;
}
public static const GeometricAntiQuotientInfix geometric_anti_quotient = GeometricAntiQuotientInfix();
public struct GeometricAntiQuotientInfix {}
public struct GeometricAntiQuotientInfixPartial<A> { a: A; }
__include "./impls/geometric_anti_quotient";

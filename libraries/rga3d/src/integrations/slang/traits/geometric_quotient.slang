implementing rga3d;using data;

/// GeometricQuotient
/// Product of A with Inverse of B.
public interface GeometricQuotient<T> {
    associatedtype GeometricQuotientOutput;
    func geometric_quotient(other: T) -> Output;
}
public static const GeometricQuotientInfix geometric_quotient = GeometricQuotientInfix();
public struct GeometricQuotientInfix {}
public struct GeometricQuotientInfixPartial<A> { a: A; }
__include "./impls/geometric_quotient";

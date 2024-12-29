implementing rga3d;using data;

/// GeometricQuotient
/// Product of A with Inverse of B.
public interface GeometricQuotient<T> {
    associatedtype GeometricQuotientOutput;
    func geometric_quotient(other: T) -> GeometricQuotientOutput;
}
public static const GeometricQuotientInfix geometric_quotient = GeometricQuotientInfix();
public struct GeometricQuotientInfix {}
public struct GeometricQuotientInfixPartial<A> { internal A a; }
__include "./impls/geometric_quotient";

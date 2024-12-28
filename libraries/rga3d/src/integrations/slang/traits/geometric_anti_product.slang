implementing rga3d;using data;

/// GeometricAntiProduct
/// The GeometricAntiProduct or sometimes called AntiProduct is the dual to the GeometricProduct. It depends on a specified AntiScalar. Anti-Multiplying uniform grade geometry may result in mixed grade anti-products, bottoming out at the Scalar. See also AntiSandwich.
public interface GeometricAntiProduct<T> {
    associatedtype GeometricAntiProductOutput;
    func geometric_anti_product(other: T) -> Output;
}
public static const GeometricAntiProductInfix geometric_anti_product = GeometricAntiProductInfix();
public struct GeometricAntiProductInfix {}
public struct GeometricAntiProductInfixPartial<A> { a: A; }
__include "./impls/geometric_anti_product";

using data::*;

/// GeometricAntiProduct
/// The GeometricAntiProduct or sometimes called AntiProduct is the dual to the GeometricProduct. It depends on a specified AntiScalar. Anti-Multiplying uniform grade geometry may result in mixed grade anti-products, bottoming out at the Scalar. See also AntiSandwich.
public interface GeometricAntiProduct<T> {
    associatedtype Output;
    func geometric_anti_product(other: T) -> Output;
}
public const static geometric_anti_product: GeometricAntiProductInfix = GeometricAntiProductInfix {}
public struct GeometricAntiProductInfix {}
public struct GeometricAntiProductInfixPartial<A> { a: A }
extension GeometricAntiProductInfixPartial<A> for A: GeometricAntiProduct<B> {
    func operator/(rhs: B) -> A.Output {
        return this.a.geometric_anti_product(rhs);
    }
}
__include ./impls/geometric_anti_product;

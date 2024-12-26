using data::*;

/// GeometricAntiProduct
/// The GeometricAntiProduct or sometimes called AntiProduct is the dual to the GeometricProduct. It depends on a specified AntiScalar. Anti-Multiplying uniform grade geometry may result in mixed grade anti-products, bottoming out at the Scalar. See also AntiSandwich.
public interface GeometricAntiProduct<T> {
    associatedtype Output;
    fn geometric_anti_product(other: T) -> Self::Output;
}
public struct geometric_anti_product;
public struct geometric_anti_product_partial<A> { a: A }
extension geometric_anti_product_partial<A> {    associatedtype Output = <A as GeometricAntiProduct<B>>::Output;
    func operator/(rhs: B) -> Self::Output {
        this.a.geometric_anti_product(rhs)
    }
}
__include ./impls/geometric_anti_product;

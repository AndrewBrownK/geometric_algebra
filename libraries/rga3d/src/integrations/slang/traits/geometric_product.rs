using data::*;

/// GeometricProduct
/// The geometric product is what lets us treat geometry like algebra. It is literally multiplication of geometry. It is derived by the wedge product and the algebra's metric. Generator elements (grade 1 BasisElements) will square to a particular value (as specified by the algebra and metric), typically 1, -1, or 0. This is in contrast to the wedge product which always wedge-squares elements to zero. Multiplying uniform grade geometry may result in mixed grade products, topping out at the AntiScalar. See also Sandwich.
public interface GeometricProduct<T> {
    associatedtype Output;
    fn geometric_product(other: T) -> Self::Output;
}
public struct geometric_product;
public struct geometric_product_partial<A> { a: A }
extension geometric_product_partial<A> {    associatedtype Output = <A as GeometricProduct<B>>::Output;
    func operator/(rhs: B) -> Self::Output {
        this.a.geometric_product(rhs)
    }
}
__include ./impls/geometric_product;

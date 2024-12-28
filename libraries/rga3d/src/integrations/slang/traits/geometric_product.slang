implementing rga3d;using data;

/// GeometricProduct
/// The geometric product is what lets us treat geometry like algebra. It is literally multiplication of geometry. It is derived by the wedge product and the algebra's metric. Generator elements (grade 1 BasisElements) will square to a particular value (as specified by the algebra and metric), typically 1, -1, or 0. This is in contrast to the wedge product which always wedge-squares elements to zero. Multiplying uniform grade geometry may result in mixed grade products, topping out at the AntiScalar. See also Sandwich.
public interface GeometricProduct<T> {
    associatedtype GeometricProductOutput;
    func geometric_product(other: T) -> Output;
}
public static const GeometricProductInfix geometric_product = GeometricProductInfix();
public struct GeometricProductInfix {}
public struct GeometricProductInfixPartial<A> { a: A; }
__include "./impls/geometric_product";

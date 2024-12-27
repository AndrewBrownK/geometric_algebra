using data::*;

/// DotProduct
/// This dot product is almost exactly what you would expect from regular vector algebra. It always returns a scalar result. It is determined by the metric of the algebra, so intermediate terms might come out to zero or negative, depending on the generator squares. The "dot product" is overloaded to several different meanings depending on which community you discuss with, so if someone or something refers to the "dot product" then use care and double check the definition that is intended.
public interface DotProduct<T> {
    func dot_product(other: T) -> Scalar;
}
public struct dot_product {}
public struct dot_product_partial<A> { a: A }
extension dot_product_partial<A> for A: DotProduct<B> {
    func operator/(rhs: B) -> A.Output {
        return this.a.dot_product(rhs);
    }
}
__include ./impls/dot_product;

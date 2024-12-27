using data::*;

/// AntiDotProduct
/// This is the dual to the dot product, and always returns an AntiScalar.
public interface AntiDotProduct<T> {
    func anti_dot_product(other: T) -> AntiScalar;
}
public struct anti_dot_product {}
public struct anti_dot_product_partial<A> { a: A }
extension anti_dot_product_partial<A> for A: AntiDotProduct<B> {
    func operator/(rhs: B) -> A.Output {
        return this.a.anti_dot_product(rhs);
    }
}
__include ./impls/anti_dot_product;

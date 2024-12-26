using data::*;

/// AntiDotProduct
/// This is the dual to the dot product, and always returns an AntiScalar.
public interface AntiDotProduct<T> {
    fn anti_dot_product(other: T) -> AntiScalar;
}
public struct anti_dot_product;
public struct anti_dot_product_partial<A> { a: A }
extension anti_dot_product_partial<A> {    associatedtype Output = AntiScalar;
    func operator/(rhs: B) -> Self::Output {
        this.a.anti_dot_product(rhs)
    }
}
__include ./impls/anti_dot_product;

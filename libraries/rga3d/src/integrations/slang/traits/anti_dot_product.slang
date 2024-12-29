implementing rga3d;using data;

/// AntiDotProduct
/// This is the dual to the dot product, and always returns an AntiScalar.
public interface AntiDotProduct<T> {
    func anti_dot_product(other: T) -> AntiScalar;
}
public static const AntiDotProductInfix anti_dot_product = AntiDotProductInfix();
public struct AntiDotProductInfix {}
public struct AntiDotProductInfixPartial<A> { internal A a; }
__include "./impls/anti_dot_product";

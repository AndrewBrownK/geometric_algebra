implementing rga3d;using data;

/// AntiInverse
/// The inverse with respect to the geometric anti-product.
public interface AntiInverse {
    func anti_inverse() -> This;
}
public static const AntiInversePrefixOrPostfix anti_inverse = AntiInversePrefixOrPostfix();
public struct AntiInversePrefixOrPostfix {}
__include "./impls/anti_inverse";

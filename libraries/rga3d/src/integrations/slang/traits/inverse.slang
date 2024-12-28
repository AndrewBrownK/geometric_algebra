implementing rga3d;using data;

/// Inverse
/// The inverse with respect to geometric product. Inverse(x) = x^-1.
public interface Inverse {
    func inverse() -> This;
}
public static const InversePrefixOrPostfix inverse = InversePrefixOrPostfix();
public struct InversePrefixOrPostfix {}
__include "./impls/inverse";

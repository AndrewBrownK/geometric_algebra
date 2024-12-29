implementing rga3d;
using data;

/// Inverse
/// The inverse with respect to geometric product. Inverse(x) = x^-1.
public static const InversePrefixOrPostfix inverse = InversePrefixOrPostfix();
public struct InversePrefixOrPostfix {}
__include "./impls/inverse";

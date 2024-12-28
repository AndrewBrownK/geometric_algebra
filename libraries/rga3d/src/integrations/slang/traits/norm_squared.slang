implementing rga3d;using data;

/// NormSquared
/// Intermediate result to FlatNorm.
public interface NormSquared {
    func norm_squared() -> DualNum;
}
public static const NormSquaredPrefixOrPostfix norm_squared = NormSquaredPrefixOrPostfix();
public struct NormSquaredPrefixOrPostfix {}
__include "./impls/norm_squared";

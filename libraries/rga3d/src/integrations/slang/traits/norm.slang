implementing rga3d;using data;

/// Norm
/// Norm for flat aspect.
public interface Norm {
    func norm() -> DualNum;
}
public static const NormPrefixOrPostfix norm = NormPrefixOrPostfix();
public struct NormPrefixOrPostfix {}
__include "./impls/norm";

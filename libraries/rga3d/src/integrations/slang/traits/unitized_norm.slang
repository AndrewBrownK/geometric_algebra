implementing rga3d;using data;

/// UnitizedNorm
/// Unitized FlatNorm.
public interface UnitizedNorm {
    func unitized_norm() -> float;
}
public static const UnitizedNormPrefixOrPostfix unitized_norm = UnitizedNormPrefixOrPostfix();
public struct UnitizedNormPrefixOrPostfix {}
__include "./impls/unitized_norm";

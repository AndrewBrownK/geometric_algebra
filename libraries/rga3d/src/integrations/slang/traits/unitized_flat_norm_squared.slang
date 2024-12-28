implementing rga3d;using data;

/// UnitizedFlatNormSquared
/// Intermediate result to UnitizedFlatNorm.
public interface UnitizedFlatNormSquared {
    func unitized_flat_norm_squared() -> float;
}
public static const UnitizedFlatNormSquaredPrefixOrPostfix unitized_flat_norm_squared = UnitizedFlatNormSquaredPrefixOrPostfix();
public struct UnitizedFlatNormSquaredPrefixOrPostfix {}
__include "./impls/unitized_flat_norm_squared";

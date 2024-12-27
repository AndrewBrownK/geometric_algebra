using data::*;

/// UnitizedRadiusNormSquared
/// Intermediate result to UnitizedRadiusNorm.
public interface UnitizedRadiusNormSquared {
    func unitized_radius_norm_squared() -> float;
}
public static const unitized_radius_norm_squared: UnitizedRadiusNormSquaredPrefixOrPostfix = UnitizedRadiusNormSquaredPrefixOrPostfix {};
public struct UnitizedRadiusNormSquaredPrefixOrPostfix {}
extension UnitizedRadiusNormSquaredPrefixOrPostfix for A: UnitizedRadiusNormSquared {
    func operator/(rhs: A) -> A.Output {
        return rhs.unitized_radius_norm_squared();
    }
}
__include ./impls/unitized_radius_norm_squared;

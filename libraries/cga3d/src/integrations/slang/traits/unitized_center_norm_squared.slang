using data::*;

/// UnitizedCenterNormSquared
/// Intermediate result to UnitizedCenterNorm.
public interface UnitizedCenterNormSquared {
    func unitized_center_norm_squared() -> float;
}
public static const unitized_center_norm_squared: UnitizedCenterNormSquaredPrefixOrPostfix = UnitizedCenterNormSquaredPrefixOrPostfix {};
public struct UnitizedCenterNormSquaredPrefixOrPostfix {}
extension UnitizedCenterNormSquaredPrefixOrPostfix for A: UnitizedCenterNormSquared {
    func operator/(rhs: A) -> A.Output {
        return rhs.unitized_center_norm_squared();
    }
}
__include ./impls/unitized_center_norm_squared;

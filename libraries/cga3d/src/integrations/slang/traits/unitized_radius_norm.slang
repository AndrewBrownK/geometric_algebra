using data::*;

/// UnitizedRadiusNorm
/// Unitized radius of an object.
public interface UnitizedRadiusNorm {
    func unitized_radius_norm() -> float;
}
public static const unitized_radius_norm: UnitizedRadiusNormPrefixOrPostfix = UnitizedRadiusNormPrefixOrPostfix {};
public struct UnitizedRadiusNormPrefixOrPostfix {}
extension UnitizedRadiusNormPrefixOrPostfix for A: UnitizedRadiusNorm {
    func operator/(rhs: A) -> A.Output {
        return rhs.unitized_radius_norm();
    }
}
__include ./impls/unitized_radius_norm;

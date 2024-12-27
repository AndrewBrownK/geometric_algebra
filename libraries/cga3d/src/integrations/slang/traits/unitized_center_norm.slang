using data::*;

/// UnitizedCenterNorm
/// Unitized distance from origin to center of round object.
public interface UnitizedCenterNorm {
    func unitized_center_norm() -> float;
}
public static const unitized_center_norm: UnitizedCenterNormPrefixOrPostfix = UnitizedCenterNormPrefixOrPostfix {};
public struct UnitizedCenterNormPrefixOrPostfix {}
extension UnitizedCenterNormPrefixOrPostfix for A: UnitizedCenterNorm {
    func operator/(rhs: A) -> A.Output {
        return rhs.unitized_center_norm();
    }
}
__include ./impls/unitized_center_norm;

using data::*;

/// UnitizedRoundNorm
/// Unitized Norm for round aspect.
public interface UnitizedRoundNorm {
    func unitized_round_norm() -> float;
}
public static const unitized_round_norm: UnitizedRoundNormPrefixOrPostfix = UnitizedRoundNormPrefixOrPostfix {};
public struct UnitizedRoundNormPrefixOrPostfix {}
extension UnitizedRoundNormPrefixOrPostfix for A: UnitizedRoundNorm {
    func operator/(rhs: A) -> A.Output {
        return rhs.unitized_round_norm();
    }
}
__include ./impls/unitized_round_norm;

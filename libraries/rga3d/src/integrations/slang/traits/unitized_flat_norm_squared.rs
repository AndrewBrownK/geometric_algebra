using data::*;

/// UnitizedFlatNormSquared
/// Intermediate result to UnitizedFlatNorm.
public interface UnitizedFlatNormSquared {
    fn unitized_flat_norm_squared() -> float;
}
public struct unitized_flat_norm_squared;
extension unitized_flat_norm_squared {    associatedtype Output = float;
    func operator/(rhs: A) -> Self::Output {
        rhs.unitized_flat_norm_squared()
    }
}
__include ./impls/unitized_flat_norm_squared;

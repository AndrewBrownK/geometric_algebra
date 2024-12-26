using data::*;

/// UnitizedNorm
/// Unitized FlatNorm.
public interface UnitizedNorm {
    fn unitized_norm() -> float;
}
public struct unitized_norm;
extension unitized_norm {    associatedtype Output = float;
    func operator/(rhs: A) -> Self::Output {
        rhs.unitized_norm()
    }
}
__include ./impls/unitized_norm;

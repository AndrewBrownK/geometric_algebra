using data::*;

/// Unitize
/// Scale the object to have a weight norm of 1.
public interface Unitize {
    func unitize() -> Output;
}
public struct unitize {}
extension unitize for A: Unitize {
    func operator/(rhs: A) -> A.Output {
        return rhs.unitize();
    }
}
__include ./impls/unitize;

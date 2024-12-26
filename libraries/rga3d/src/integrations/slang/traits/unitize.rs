using data::*;

/// Unitize
/// Scale the object to have a weight norm of 1.
public interface Unitize {
    fn unitize() -> Unitize;
}
public struct unitize;
extension unitize {    associatedtype Output = A;
    func operator/(rhs: A) -> Self::Output {
        rhs.unitize()
    }
}
__include ./impls/unitize;

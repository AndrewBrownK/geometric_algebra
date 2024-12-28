implementing rga3d;using data;

/// Unitize
/// Scale the object to have a weight norm of 1.
public interface Unitize {
    func unitize() -> This;
}
public static const UnitizePrefixOrPostfix unitize = UnitizePrefixOrPostfix();
public struct UnitizePrefixOrPostfix {}
__include "./impls/unitize";

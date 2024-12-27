using data::*;

/// Unit
/// All elements set to one.
public interface Unit {
    func unit() -> Output;
}
public const static unit: UnitInfix = UnitInfix {}
public struct UnitInfix {}
__include ./impls/unit;

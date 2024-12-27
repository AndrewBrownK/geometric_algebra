using data::*;

/// Unit
/// All elements set to one.
public interface Unit {
    func unit() -> Output;
}
public struct unit {}
__include ./impls/unit;

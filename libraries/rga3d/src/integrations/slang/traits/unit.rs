using data::*;

/// Unit
/// All elements set to one.
public interface Unit {
    fn unit() -> Unit;
}
public struct unit;
__include ./impls/unit;

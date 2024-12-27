using data::*;

/// Zero
/// All elements set to zero.
public interface Zero {
    func zero() -> Output;
}
public struct zero {}
__include ./impls/zero;

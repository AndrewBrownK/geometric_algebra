using data::*;

/// Zero
/// All elements set to zero.
public interface Zero {
    fn zero() -> Zero;
}
public struct zero;
__include ./impls/zero;

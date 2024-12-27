using data::*;

/// Zero
/// All elements set to zero.
public interface Zero {
    func zero() -> Output;
}
public const static zero: ZeroInfix = ZeroInfix {}
public struct ZeroInfix {}
__include ./impls/zero;

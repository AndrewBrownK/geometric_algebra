using data::*;

/// One
/// The scalar element set to one, and all other elements set to zero.
public interface One {
    fn one() -> One;
}
public struct one;
__include ./impls/one;

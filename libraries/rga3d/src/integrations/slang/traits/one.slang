using data::*;

/// One
/// The scalar element set to one, and all other elements set to zero.
public interface One {
    func one() -> Output;
}
__include ./impls/one;

using data::*;

/// AntiOne
/// The anti-scalar element set to one, and all other elements set to zero.
public interface AntiOne {
    fn anti_one() -> AntiOne;
}
public struct anti_one;
__include ./impls/anti_one;

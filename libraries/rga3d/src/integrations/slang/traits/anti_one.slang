using data::*;

/// AntiOne
/// The anti-scalar element set to one, and all other elements set to zero.
public interface AntiOne {
    func anti_one() -> Output;
}
public struct anti_one {}
__include ./impls/anti_one;

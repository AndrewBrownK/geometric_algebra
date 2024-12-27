using data::*;

/// AntiOne
/// The anti-scalar element set to one, and all other elements set to zero.
public interface AntiOne {
    func anti_one() -> Output;
}
public const static anti_one: AntiOneInfix = AntiOneInfix {}
public struct AntiOneInfix {}
__include ./impls/anti_one;

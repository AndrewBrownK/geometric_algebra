using data::*;

/// AntiRejectViaHorizonFrom
/// Counterpart to AntiProjectViaHorizonOnto.
public interface AntiRejectViaHorizonFrom<T> {
    associatedtype Output;
    func anti_reject_via_horizon_from(other: T) -> Output;
}
public static const anti_reject_via_horizon_from: AntiRejectViaHorizonFromInfix = AntiRejectViaHorizonFromInfix {};
public struct AntiRejectViaHorizonFromInfix {}
public struct AntiRejectViaHorizonFromInfixPartial<A> { a: A }
extension AntiRejectViaHorizonFromInfixPartial<A> for A: AntiRejectViaHorizonFrom<B> {
    func operator/(rhs: B) -> A.Output {
        return this.a.anti_reject_via_horizon_from(rhs);
    }
}
__include ./impls/anti_reject_via_horizon_from;

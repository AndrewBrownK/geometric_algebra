using data::*;

/// AntiRejectViaHorizonFrom
/// Counterpart to AntiProjectViaHorizonOnto.
public interface AntiRejectViaHorizonFrom<T> {
    associatedtype Output;
    func anti_reject_via_horizon_from(other: T) -> Output;
}
public struct anti_reject_via_horizon_from {}
public struct anti_reject_via_horizon_from_partial<A> { a: A }
extension anti_reject_via_horizon_from_partial<A> for A: AntiRejectViaHorizonFrom<B> {
    func operator/(rhs: B) -> A.Output {
        return this.a.anti_reject_via_horizon_from(rhs);
    }
}
__include ./impls/anti_reject_via_horizon_from;

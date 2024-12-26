using data::*;

/// AntiRejectViaHorizonFrom
/// Counterpart to AntiProjectViaHorizonOnto.
public interface AntiRejectViaHorizonFrom<T> {
    associatedtype Output;
    fn anti_reject_via_horizon_from(other: T) -> Self::Output;
}
public struct anti_reject_via_horizon_from;
public struct anti_reject_via_horizon_from_partial<A> { a: A }
extension anti_reject_via_horizon_from_partial<A> {    associatedtype Output = <A as AntiRejectViaHorizonFrom<B>>::Output;
    func operator/(rhs: B) -> Self::Output {
        this.a.anti_reject_via_horizon_from(rhs)
    }
}
__include ./impls/anti_reject_via_horizon_from;

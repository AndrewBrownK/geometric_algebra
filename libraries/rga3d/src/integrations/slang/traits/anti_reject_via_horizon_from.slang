implementing rga3d;using data;

/// AntiRejectViaHorizonFrom
/// Counterpart to AntiProjectViaHorizonOnto.
public interface AntiRejectViaHorizonFrom<T> {
    associatedtype AntiRejectViaHorizonFromOutput;
    func anti_reject_via_horizon_from(other: T) -> Output;
}
public static const AntiRejectViaHorizonFromInfix anti_reject_via_horizon_from = AntiRejectViaHorizonFromInfix();
public struct AntiRejectViaHorizonFromInfix {}
public struct AntiRejectViaHorizonFromInfixPartial<A> { a: A; }
__include "./impls/anti_reject_via_horizon_from";

implementing rga3d;using data;

/// AntiProjectViaHorizonOnto
/// Outward (to horizon) AntiProjection.
public interface AntiProjectViaHorizonOnto<T> {
    associatedtype AntiProjectViaHorizonOntoOutput;
    func anti_project_via_horizon_onto(other: T) -> Output;
}
public static const AntiProjectViaHorizonOntoInfix anti_project_via_horizon_onto = AntiProjectViaHorizonOntoInfix();
public struct AntiProjectViaHorizonOntoInfix {}
public struct AntiProjectViaHorizonOntoInfixPartial<A> { a: A; }
__include "./impls/anti_project_via_horizon_onto";

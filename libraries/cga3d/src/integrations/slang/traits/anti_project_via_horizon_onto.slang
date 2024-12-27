using data::*;

/// AntiProjectViaHorizonOnto
/// Outward (to horizon) AntiProjection.
public interface AntiProjectViaHorizonOnto<T> {
    associatedtype Output;
    func anti_project_via_horizon_onto(other: T) -> Output;
}
public static const anti_project_via_horizon_onto: AntiProjectViaHorizonOntoInfix = AntiProjectViaHorizonOntoInfix {};
public struct AntiProjectViaHorizonOntoInfix {}
public struct AntiProjectViaHorizonOntoInfixPartial<A> { a: A }
extension AntiProjectViaHorizonOntoInfixPartial<A> for A: AntiProjectViaHorizonOnto<B> {
    func operator/(rhs: B) -> A.Output {
        return this.a.anti_project_via_horizon_onto(rhs);
    }
}
__include ./impls/anti_project_via_horizon_onto;

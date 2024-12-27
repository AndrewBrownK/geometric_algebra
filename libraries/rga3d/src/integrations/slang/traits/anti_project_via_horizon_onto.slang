using data::*;

/// AntiProjectViaHorizonOnto
/// Outward (to horizon) AntiProjection.
public interface AntiProjectViaHorizonOnto<T> {
    associatedtype Output;
    func anti_project_via_horizon_onto(other: T) -> Output;
}
public struct anti_project_via_horizon_onto {}
public struct anti_project_via_horizon_onto_partial<A> { a: A }
extension anti_project_via_horizon_onto_partial<A> for A: AntiProjectViaHorizonOnto<B> {
    func operator/(rhs: B) -> A.Output {
        return this.a.anti_project_via_horizon_onto(rhs);
    }
}
__include ./impls/anti_project_via_horizon_onto;

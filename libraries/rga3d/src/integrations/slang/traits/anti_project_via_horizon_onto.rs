using data::*;

/// AntiProjectViaHorizonOnto
/// Outward (to horizon) AntiProjection.
public interface AntiProjectViaHorizonOnto<T> {
    associatedtype Output;
    fn anti_project_via_horizon_onto(other: T) -> Self::Output;
}
public struct anti_project_via_horizon_onto;
public struct anti_project_via_horizon_onto_partial<A> { a: A }
extension anti_project_via_horizon_onto_partial<A> {    associatedtype Output = <A as AntiProjectViaHorizonOnto<B>>::Output;
    func operator/(rhs: B) -> Self::Output {
        this.a.anti_project_via_horizon_onto(rhs)
    }
}
__include ./impls/anti_project_via_horizon_onto;

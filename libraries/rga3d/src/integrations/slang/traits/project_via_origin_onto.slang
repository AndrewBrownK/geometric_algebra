using data::*;

/// ProjectViaOriginOnto
/// Central (to origin) Projection.
public interface ProjectViaOriginOnto<T> {
    associatedtype Output;
    func project_via_origin_onto(other: T) -> Output;
}
public struct project_via_origin_onto {}
public struct project_via_origin_onto_partial<A> { a: A }
extension project_via_origin_onto_partial<A> for A: ProjectViaOriginOnto<B> {
    func operator/(rhs: B) -> A.Output {
        return this.a.project_via_origin_onto(rhs);
    }
}
__include ./impls/project_via_origin_onto;

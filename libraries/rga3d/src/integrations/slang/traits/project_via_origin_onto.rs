using data::*;

/// ProjectViaOriginOnto
/// Central (to origin) Projection.
public interface ProjectViaOriginOnto<T> {
    associatedtype Output;
    fn project_via_origin_onto(other: T) -> Self::Output;
}
public struct project_via_origin_onto;
public struct project_via_origin_onto_partial<A> { a: A }
extension project_via_origin_onto_partial<A> {    associatedtype Output = <A as ProjectViaOriginOnto<B>>::Output;
    func operator/(rhs: B) -> Self::Output {
        this.a.project_via_origin_onto(rhs)
    }
}
__include ./impls/project_via_origin_onto;

using data::*;

/// ProjectViaOriginOnto
/// Central (to origin) Projection.
public interface ProjectViaOriginOnto<T> {
    associatedtype Output;
    func project_via_origin_onto(other: T) -> Output;
}
public static const project_via_origin_onto: ProjectViaOriginOntoInfix = ProjectViaOriginOntoInfix {};
public struct ProjectViaOriginOntoInfix {}
public struct ProjectViaOriginOntoInfixPartial<A> { a: A }
extension ProjectViaOriginOntoInfixPartial<A> for A: ProjectViaOriginOnto<B> {
    func operator/(rhs: B) -> A.Output {
        return this.a.project_via_origin_onto(rhs);
    }
}
__include ./impls/project_via_origin_onto;

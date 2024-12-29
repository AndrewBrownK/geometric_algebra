implementing rga3d;using data;

/// ProjectViaOriginOnto
/// Central (to origin) Projection.
public interface ProjectViaOriginOnto<T> {
    associatedtype ProjectViaOriginOntoOutput;
    func project_via_origin_onto(other: T) -> ProjectViaOriginOntoOutput;
}
public static const ProjectViaOriginOntoInfix project_via_origin_onto = ProjectViaOriginOntoInfix();
public struct ProjectViaOriginOntoInfix {}
public struct ProjectViaOriginOntoInfixPartial<A> { internal A a; }
__include "./impls/project_via_origin_onto";

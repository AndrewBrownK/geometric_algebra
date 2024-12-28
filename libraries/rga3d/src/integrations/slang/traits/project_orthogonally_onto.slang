implementing rga3d;using data;

/// ProjectOrthogonallyOnto
/// Typically involves bringing a lower dimensional object to a higher dimensional object.
public interface ProjectOrthogonallyOnto<T> {
    associatedtype ProjectOrthogonallyOntoOutput;
    func project_orthogonally_onto(other: T) -> Output;
}
public static const ProjectOrthogonallyOntoInfix project_orthogonally_onto = ProjectOrthogonallyOntoInfix();
public struct ProjectOrthogonallyOntoInfix {}
public struct ProjectOrthogonallyOntoInfixPartial<A> { a: A; }
__include "./impls/project_orthogonally_onto";

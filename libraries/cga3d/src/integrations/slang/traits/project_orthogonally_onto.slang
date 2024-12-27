using data::*;

/// ProjectOrthogonallyOnto
/// Typically involves bringing a lower dimensional object to a higher dimensional object.
public interface ProjectOrthogonallyOnto<T> {
    associatedtype Output;
    func project_orthogonally_onto(other: T) -> Output;
}
public static const project_orthogonally_onto: ProjectOrthogonallyOntoInfix = ProjectOrthogonallyOntoInfix {};
public struct ProjectOrthogonallyOntoInfix {}
public struct ProjectOrthogonallyOntoInfixPartial<A> { a: A }
extension ProjectOrthogonallyOntoInfixPartial<A> for A: ProjectOrthogonallyOnto<B> {
    func operator/(rhs: B) -> A.Output {
        return this.a.project_orthogonally_onto(rhs);
    }
}
__include ./impls/project_orthogonally_onto;

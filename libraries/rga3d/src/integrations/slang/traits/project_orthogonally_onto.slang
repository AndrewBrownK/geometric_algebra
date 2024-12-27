using data::*;

/// ProjectOrthogonallyOnto
/// Typically involves bringing a lower dimensional object to a higher dimensional object.
public interface ProjectOrthogonallyOnto<T> {
    associatedtype Output;
    func project_orthogonally_onto(other: T) -> Output;
}
public struct project_orthogonally_onto {}
public struct project_orthogonally_onto_partial<A> { a: A }
extension project_orthogonally_onto_partial<A> for A: ProjectOrthogonallyOnto<B> {
    func operator/(rhs: B) -> A.Output {
        return this.a.project_orthogonally_onto(rhs);
    }
}
__include ./impls/project_orthogonally_onto;

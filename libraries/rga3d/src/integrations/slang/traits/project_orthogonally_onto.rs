using data::*;

/// ProjectOrthogonallyOnto
/// Typically involves bringing a lower dimensional object to a higher dimensional object.
public interface ProjectOrthogonallyOnto<T> {
    associatedtype Output;
    fn project_orthogonally_onto(other: T) -> Self::Output;
}
public struct project_orthogonally_onto;
public struct project_orthogonally_onto_partial<A> { a: A }
extension project_orthogonally_onto_partial<A> {    associatedtype Output = <A as ProjectOrthogonallyOnto<B>>::Output;
    func operator/(rhs: B) -> Self::Output {
        this.a.project_orthogonally_onto(rhs)
    }
}
__include ./impls/project_orthogonally_onto;

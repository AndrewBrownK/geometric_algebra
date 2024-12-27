using data::*;

/// AntiProjectOrthogonallyOnto
/// Typically involves bringing a higher dimensional object to a lower dimensional object.
public interface AntiProjectOrthogonallyOnto<T> {
    associatedtype Output;
    func anti_project_orthogonally_onto(other: T) -> Output;
}
public struct anti_project_orthogonally_onto {}
public struct anti_project_orthogonally_onto_partial<A> { a: A }
extension anti_project_orthogonally_onto_partial<A> for A: AntiProjectOrthogonallyOnto<B> {
    func operator/(rhs: B) -> A.Output {
        return this.a.anti_project_orthogonally_onto(rhs);
    }
}
__include ./impls/anti_project_orthogonally_onto;

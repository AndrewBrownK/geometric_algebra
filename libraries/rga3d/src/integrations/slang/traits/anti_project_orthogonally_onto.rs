using data::*;

/// AntiProjectOrthogonallyOnto
/// Typically involves bringing a higher dimensional object to a lower dimensional object.
public interface AntiProjectOrthogonallyOnto<T> {
    associatedtype Output;
    fn anti_project_orthogonally_onto(other: T) -> Self::Output;
}
public struct anti_project_orthogonally_onto;
public struct anti_project_orthogonally_onto_partial<A> { a: A }
extension anti_project_orthogonally_onto_partial<A> {    associatedtype Output = <A as AntiProjectOrthogonallyOnto<B>>::Output;
    func operator/(rhs: B) -> Self::Output {
        this.a.anti_project_orthogonally_onto(rhs)
    }
}
__include ./impls/anti_project_orthogonally_onto;

using data::*;

/// RejectOrthogonallyFrom
/// Counterpart to ProjectOrthogonallyOnto.
public interface RejectOrthogonallyFrom<T> {
    associatedtype Output;
    func reject_orthogonally_from(other: T) -> Output;
}
public struct reject_orthogonally_from {}
public struct reject_orthogonally_from_partial<A> { a: A }
extension reject_orthogonally_from_partial<A> for A: RejectOrthogonallyFrom<B> {
    func operator/(rhs: B) -> A.Output {
        return this.a.reject_orthogonally_from(rhs);
    }
}
__include ./impls/reject_orthogonally_from;

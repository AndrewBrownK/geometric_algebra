using data::*;

/// RejectOrthogonallyFrom
/// Counterpart to ProjectOrthogonallyOnto.
public interface RejectOrthogonallyFrom<T> {
    associatedtype Output;
    fn reject_orthogonally_from(other: T) -> Self::Output;
}
public struct reject_orthogonally_from;
public struct reject_orthogonally_from_partial<A> { a: A }
extension reject_orthogonally_from_partial<A> {    associatedtype Output = <A as RejectOrthogonallyFrom<B>>::Output;
    func operator/(rhs: B) -> Self::Output {
        this.a.reject_orthogonally_from(rhs)
    }
}
__include ./impls/reject_orthogonally_from;

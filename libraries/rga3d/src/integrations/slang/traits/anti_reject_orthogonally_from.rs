using data::*;

/// AntiRejectOrthogonallyFrom
/// Counterpart to AntiProjectOrthogonallyOnto.
public interface AntiRejectOrthogonallyFrom<T> {
    associatedtype Output;
    fn anti_reject_orthogonally_from(other: T) -> Self::Output;
}
public struct anti_reject_orthogonally_from;
public struct anti_reject_orthogonally_from_partial<A> { a: A }
extension anti_reject_orthogonally_from_partial<A> {    associatedtype Output = <A as AntiRejectOrthogonallyFrom<B>>::Output;
    func operator/(rhs: B) -> Self::Output {
        this.a.anti_reject_orthogonally_from(rhs)
    }
}
__include ./impls/anti_reject_orthogonally_from;

using data::*;

/// AntiRejectOrthogonallyFrom
/// Counterpart to AntiProjectOrthogonallyOnto.
public interface AntiRejectOrthogonallyFrom<T> {
    associatedtype Output;
    func anti_reject_orthogonally_from(other: T) -> Output;
}
public struct anti_reject_orthogonally_from {}
public struct anti_reject_orthogonally_from_partial<A> { a: A }
extension anti_reject_orthogonally_from_partial<A> for A: AntiRejectOrthogonallyFrom<B> {
    func operator/(rhs: B) -> A.Output {
        return this.a.anti_reject_orthogonally_from(rhs);
    }
}
__include ./impls/anti_reject_orthogonally_from;

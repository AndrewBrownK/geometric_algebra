using data::*;

/// AntiRejectOrthogonallyFrom
/// Counterpart to AntiProjectOrthogonallyOnto.
public interface AntiRejectOrthogonallyFrom<T> {
    associatedtype Output;
    func anti_reject_orthogonally_from(other: T) -> Output;
}
public static const anti_reject_orthogonally_from: AntiRejectOrthogonallyFromInfix = AntiRejectOrthogonallyFromInfix {};
public struct AntiRejectOrthogonallyFromInfix {}
public struct AntiRejectOrthogonallyFromInfixPartial<A> { a: A }
extension AntiRejectOrthogonallyFromInfixPartial<A> for A: AntiRejectOrthogonallyFrom<B> {
    func operator/(rhs: B) -> A.Output {
        return this.a.anti_reject_orthogonally_from(rhs);
    }
}
__include ./impls/anti_reject_orthogonally_from;

using data::*;

/// RejectOrthogonallyFrom
/// Counterpart to ProjectOrthogonallyOnto.
public interface RejectOrthogonallyFrom<T> {
    associatedtype Output;
    func reject_orthogonally_from(other: T) -> Output;
}
public static const reject_orthogonally_from: RejectOrthogonallyFromInfix = RejectOrthogonallyFromInfix {};
public struct RejectOrthogonallyFromInfix {}
public struct RejectOrthogonallyFromInfixPartial<A> { a: A }
extension RejectOrthogonallyFromInfixPartial<A> for A: RejectOrthogonallyFrom<B> {
    func operator/(rhs: B) -> A.Output {
        return this.a.reject_orthogonally_from(rhs);
    }
}
__include ./impls/reject_orthogonally_from;

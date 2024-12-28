implementing rga3d;using data;

/// RejectOrthogonallyFrom
/// Counterpart to ProjectOrthogonallyOnto.
public interface RejectOrthogonallyFrom<T> {
    associatedtype RejectOrthogonallyFromOutput;
    func reject_orthogonally_from(other: T) -> Output;
}
public static const RejectOrthogonallyFromInfix reject_orthogonally_from = RejectOrthogonallyFromInfix();
public struct RejectOrthogonallyFromInfix {}
public struct RejectOrthogonallyFromInfixPartial<A> { a: A; }
__include "./impls/reject_orthogonally_from";

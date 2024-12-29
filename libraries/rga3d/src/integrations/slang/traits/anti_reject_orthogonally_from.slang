implementing rga3d;using data;

/// AntiRejectOrthogonallyFrom
/// Counterpart to AntiProjectOrthogonallyOnto.
public interface AntiRejectOrthogonallyFrom<T> {
    associatedtype AntiRejectOrthogonallyFromOutput;
    func anti_reject_orthogonally_from(other: T) -> AntiRejectOrthogonallyFromOutput;
}
public static const AntiRejectOrthogonallyFromInfix anti_reject_orthogonally_from = AntiRejectOrthogonallyFromInfix();
public struct AntiRejectOrthogonallyFromInfix {}
public struct AntiRejectOrthogonallyFromInfixPartial<A> { internal A a; }
__include "./impls/anti_reject_orthogonally_from";

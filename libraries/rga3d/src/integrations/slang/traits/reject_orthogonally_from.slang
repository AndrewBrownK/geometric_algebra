implementing rga3d;
using data;

/// RejectOrthogonallyFrom
/// Counterpart to ProjectOrthogonallyOnto.
public static const RejectOrthogonallyFromInfix reject_orthogonally_from = RejectOrthogonallyFromInfix();
public struct RejectOrthogonallyFromInfix {}
public struct RejectOrthogonallyFromInfixPartial<A> { internal A a; }
__include "./impls/reject_orthogonally_from";

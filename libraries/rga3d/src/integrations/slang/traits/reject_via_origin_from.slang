implementing rga3d;using data;

/// RejectViaOriginFrom
/// Counterpart to ProjectViaOriginOnto.
public interface RejectViaOriginFrom<T> {
    associatedtype RejectViaOriginFromOutput;
    func reject_via_origin_from(other: T) -> RejectViaOriginFromOutput;
}
public static const RejectViaOriginFromInfix reject_via_origin_from = RejectViaOriginFromInfix();
public struct RejectViaOriginFromInfix {}
public struct RejectViaOriginFromInfixPartial<A> { internal A a; }
__include "./impls/reject_via_origin_from";

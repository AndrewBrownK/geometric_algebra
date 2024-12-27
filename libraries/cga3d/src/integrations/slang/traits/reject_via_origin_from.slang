using data::*;

/// RejectViaOriginFrom
/// Counterpart to ProjectViaOriginOnto.
public interface RejectViaOriginFrom<T> {
    associatedtype Output;
    func reject_via_origin_from(other: T) -> Output;
}
public static const reject_via_origin_from: RejectViaOriginFromInfix = RejectViaOriginFromInfix {};
public struct RejectViaOriginFromInfix {}
public struct RejectViaOriginFromInfixPartial<A> { a: A }
extension RejectViaOriginFromInfixPartial<A> for A: RejectViaOriginFrom<B> {
    func operator/(rhs: B) -> A.Output {
        return this.a.reject_via_origin_from(rhs);
    }
}
__include ./impls/reject_via_origin_from;

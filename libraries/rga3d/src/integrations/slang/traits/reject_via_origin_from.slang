using data::*;

/// RejectViaOriginFrom
/// Counterpart to ProjectViaOriginOnto.
public interface RejectViaOriginFrom<T> {
    associatedtype Output;
    func reject_via_origin_from(other: T) -> Output;
}
public struct reject_via_origin_from {}
public struct reject_via_origin_from_partial<A> { a: A }
extension reject_via_origin_from_partial<A> for A: RejectViaOriginFrom<B> {
    func operator/(rhs: B) -> A.Output {
        return this.a.reject_via_origin_from(rhs);
    }
}
__include ./impls/reject_via_origin_from;

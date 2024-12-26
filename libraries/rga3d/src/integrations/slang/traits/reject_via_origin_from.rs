using data::*;

/// RejectViaOriginFrom
/// Counterpart to ProjectViaOriginOnto.
public interface RejectViaOriginFrom<T> {
    associatedtype Output;
    fn reject_via_origin_from(other: T) -> Self::Output;
}
public struct reject_via_origin_from;
public struct reject_via_origin_from_partial<A> { a: A }
extension reject_via_origin_from_partial<A> {    associatedtype Output = <A as RejectViaOriginFrom<B>>::Output;
    func operator/(rhs: B) -> Self::Output {
        this.a.reject_via_origin_from(rhs)
    }
}
__include ./impls/reject_via_origin_from;

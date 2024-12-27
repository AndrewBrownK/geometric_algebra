using data::*;

/// AntiWedge
/// The AntiWedge product is the dual operation to the Wedge product, that depends on a specified AntiScalar. It combines BasisElements by which parts are missing, instead of which parts are present. For example, with an AntiScalar of e1234, anti_wedge(e423, e321) = e23. This behaves something like an intersection of the subscripts in the BasisElements.
public interface AntiWedge<T> {
    associatedtype Output;
    func anti_wedge(other: T) -> Output;
}
public struct anti_wedge {}
public struct anti_wedge_partial<A> { a: A }
extension anti_wedge_partial<A> for A: AntiWedge<B> {
    func operator/(rhs: B) -> A.Output {
        return this.a.anti_wedge(rhs);
    }
}
__include ./impls/anti_wedge;

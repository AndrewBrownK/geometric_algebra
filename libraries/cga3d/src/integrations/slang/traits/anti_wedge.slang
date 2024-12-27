using data::*;

/// AntiWedge
/// The AntiWedge product is the dual operation to the Wedge product, that depends on a specified AntiScalar. It combines BasisElements by which parts are missing, instead of which parts are present. For example, with an AntiScalar of e1234, anti_wedge(e423, e321) = e23. This behaves something like an intersection of the subscripts in the BasisElements.
public interface AntiWedge<T> {
    associatedtype Output;
    func anti_wedge(other: T) -> Output;
}
public static const anti_wedge: AntiWedgeInfix = AntiWedgeInfix {};
public struct AntiWedgeInfix {}
public struct AntiWedgeInfixPartial<A> { a: A }
extension AntiWedgeInfixPartial<A> for A: AntiWedge<B> {
    func operator/(rhs: B) -> A.Output {
        return this.a.anti_wedge(rhs);
    }
}
__include ./impls/anti_wedge;

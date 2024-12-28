implementing rga3d;using data;

/// AntiWedge
/// The AntiWedge product is the dual operation to the Wedge product, that depends on a specified AntiScalar. It combines BasisElements by which parts are missing, instead of which parts are present. For example, with an AntiScalar of e1234, anti_wedge(e423, e321) = e23. This behaves something like an intersection of the subscripts in the BasisElements.
public interface AntiWedge<T> {
    associatedtype AntiWedgeOutput;
    func anti_wedge(other: T) -> Output;
}
public static const AntiWedgeInfix anti_wedge = AntiWedgeInfix();
public struct AntiWedgeInfix {}
public struct AntiWedgeInfixPartial<A> { a: A; }
__include "./impls/anti_wedge";

using data::*;

/// Complement
/// The Complement of a BasisElement is the missing BasisElement that when wedged together will create the AntiScalar. In geometric algebras with an odd number of dimensions, the LeftComplement and RightComplement are the same, so we just call it the Complement. For example, with an AntiScalar of e123, the complement of e1 can be found equivalently by solving e123 = wedge(e1, x) or e123 = wedge(x, e1). See also DoubleComplement.
public interface Complement {
    associatedtype Output;
    func complement() -> Output;
}
public static const complement: ComplementPrefixOrPostfix = ComplementPrefixOrPostfix {};
public struct ComplementPrefixOrPostfix {}
extension ComplementPrefixOrPostfix for A: Complement {
    func operator/(rhs: A) -> A.Output {
        return rhs.complement();
    }
}
__include ./impls/complement;

using data::*;

/// LeftComplement
/// The LeftComplement of a BasisElement is the missing BasisElement that when wedged together will create the AntiScalar. For example, with an AntiScalar of e1234, the left_complement(e1) = -e234, because wedge(e234, e1) = e1234. In this example, the left_complement(e234) = e1, because wedge(e1, e234) = e1234. See also RightComplement and DoubleComplement. The RightComplement can be used to undo a LeftComplement.
public interface LeftComplement {
    associatedtype Output;
    func left_complement() -> Output;
}
public static const left_complement: LeftComplementPrefixOrPostfix = LeftComplementPrefixOrPostfix {};
public struct LeftComplementPrefixOrPostfix {}
extension LeftComplementPrefixOrPostfix for A: LeftComplement {
    func operator/(rhs: A) -> A.Output {
        return rhs.left_complement();
    }
}
__include ./impls/left_complement;

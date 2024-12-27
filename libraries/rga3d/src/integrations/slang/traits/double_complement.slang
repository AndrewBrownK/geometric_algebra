using data::*;

/// DoubleComplement
/// Repeatedly taking a Complement will eventually return the original object. In geometric algebras with an even number of dimensions, double_complement(x) = right_complement(right_complement(x)) = left_complement(left_complement(x)). In geometric algebras with an odd number of dimensions, double_complement(x) = complement(complement(x)). In all cases, x = double_complement(double_complement(x)).
public interface DoubleComplement {
    func double_complement() -> Output;
}
public struct double_complement {}
extension double_complement for A: DoubleComplement {
    func operator/(rhs: A) -> A.Output {
        return rhs.double_complement();
    }
}
__include ./impls/double_complement;

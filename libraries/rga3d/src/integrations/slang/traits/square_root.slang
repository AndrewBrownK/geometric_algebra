using data::*;

/// SquareRoot
/// Square root of geometry. Multiple different types of geometry might square to one type of geometry, so this is only defined for types that are closed with themselves under the geometric product.
public interface SquareRoot {
    func square_root() -> Scalar;
}
public const static square_root: SquareRootInfix = SquareRootInfix {}
public struct SquareRootInfix {}
extension SquareRootInfix for A: SquareRoot {
    func operator/(rhs: A) -> A.Output {
        return rhs.square_root();
    }
}
__include ./impls/square_root;

using data::*;

/// SquareRoot
/// Square root of geometry. Multiple different types of geometry might square to one type of geometry, so this is only defined for types that are closed with themselves under the geometric product.
public interface SquareRoot {
    fn square_root() -> Scalar;
}
public struct square_root;
extension square_root {    associatedtype Output = Scalar;
    func operator/(rhs: A) -> Self::Output {
        rhs.square_root()
    }
}
__include ./impls/square_root;

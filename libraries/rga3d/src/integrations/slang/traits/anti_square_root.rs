using data::*;

/// AntiSquareRoot
/// Square root of geometry with respect to the AntiProduct. Multiple different types of geometry might anti-square to one type of geometry, so this is only defined for types that are closed with themselves under the geometric anti-product.
public interface AntiSquareRoot {
    fn anti_square_root() -> AntiScalar;
}
public struct anti_square_root;
extension anti_square_root {    associatedtype Output = AntiScalar;
    func operator/(rhs: A) -> Self::Output {
        rhs.anti_square_root()
    }
}
__include ./impls/anti_square_root;

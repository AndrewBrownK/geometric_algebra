using data::*;

/// AntiSquareRoot
/// Square root of geometry with respect to the AntiProduct. Multiple different types of geometry might anti-square to one type of geometry, so this is only defined for types that are closed with themselves under the geometric anti-product.
public interface AntiSquareRoot {
    func anti_square_root() -> AntiScalar;
}
public struct anti_square_root {}
extension anti_square_root for A: AntiSquareRoot {
    func operator/(rhs: A) -> A.Output {
        return rhs.anti_square_root();
    }
}
__include ./impls/anti_square_root;

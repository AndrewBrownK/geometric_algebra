using data::*;

/// NormSquared
/// Intermediate result to FlatNorm.
public interface NormSquared {
    func norm_squared() -> DualNum;
}
public struct norm_squared {}
extension norm_squared for A: NormSquared {
    func operator/(rhs: A) -> A.Output {
        return rhs.norm_squared();
    }
}
__include ./impls/norm_squared;

using data::*;

/// NormSquared
/// Intermediate result to FlatNorm.
public interface NormSquared {
    fn norm_squared() -> DualNum;
}
public struct norm_squared;
extension norm_squared {    associatedtype Output = DualNum;
    func operator/(rhs: A) -> Self::Output {
        rhs.norm_squared()
    }
}
__include ./impls/norm_squared;

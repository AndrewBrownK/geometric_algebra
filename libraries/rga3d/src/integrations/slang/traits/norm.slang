using data::*;

/// Norm
/// Norm for flat aspect.
public interface Norm {
    func norm() -> DualNum;
}
public struct norm {}
extension norm for A: Norm {
    func operator/(rhs: A) -> A.Output {
        return rhs.norm();
    }
}
__include ./impls/norm;

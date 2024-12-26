using data::*;

/// Norm
/// Norm for flat aspect.
public interface Norm {
    fn norm() -> DualNum;
}
public struct norm;
extension norm {    associatedtype Output = DualNum;
    func operator/(rhs: A) -> Self::Output {
        rhs.norm()
    }
}
__include ./impls/norm;

using data::*;

/// Support
/// The support is the point enclosed by the object closest to the origin.
public interface Support {
    fn support() -> Line;
}
public struct support;
extension support {    associatedtype Output = Line;
    func operator/(rhs: A) -> Self::Output {
        rhs.support()
    }
}
__include ./impls/support;

using data::*;

/// Support
/// The support is the point enclosed by the object closest to the origin.
public interface Support {
    func support() -> Line;
}
public struct support {}
extension support for A: Support {
    func operator/(rhs: A) -> A.Output {
        return rhs.support();
    }
}
__include ./impls/support;

using data::*;

/// Support
/// The support is the point enclosed by the object closest to the origin.
public interface Support {
    associatedtype Output;
    func support() -> Output;
}
public static const support: SupportPrefixOrPostfix = SupportPrefixOrPostfix {};
public struct SupportPrefixOrPostfix {}
extension SupportPrefixOrPostfix for A: Support {
    func operator/(rhs: A) -> A.Output {
        return rhs.support();
    }
}
__include ./impls/support;

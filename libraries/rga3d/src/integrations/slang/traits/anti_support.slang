using data::*;

/// AntiSupport
/// The anti-support is the anti-vector furthest from the origin that encloses the object.
public interface AntiSupport {
    associatedtype Output;
    func anti_support() -> Output;
}
public const static anti_support: AntiSupportInfix = AntiSupportInfix {}
public struct AntiSupportInfix {}
extension AntiSupportInfix for A: AntiSupport {
    func operator/(rhs: A) -> A.Output {
        return rhs.anti_support();
    }
}
__include ./impls/anti_support;

using data::*;

/// AntiSupport
/// The anti-support is the anti-vector furthest from the origin that encloses the object.
public interface AntiSupport {
    associatedtype Output;
    fn anti_support() -> Self::Output;
}
public struct anti_support;
extension anti_support {    associatedtype Output = <A as AntiSupport>::Output;
    func operator/(rhs: A) -> Self::Output {
        rhs.anti_support()
    }
}
__include ./impls/anti_support;

implementing rga3d;using data;

/// AntiSupport
/// The anti-support is the anti-vector furthest from the origin that encloses the object.
public interface AntiSupport {
    associatedtype AntiSupportOutput;
    func anti_support() -> AntiSupportOutput;
}
public static const AntiSupportPrefixOrPostfix anti_support = AntiSupportPrefixOrPostfix();
public struct AntiSupportPrefixOrPostfix {}
__include "./impls/anti_support";

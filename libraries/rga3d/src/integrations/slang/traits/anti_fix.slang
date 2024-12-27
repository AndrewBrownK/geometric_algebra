using data::*;

/// AntiFix
/// Automatically fix the anti geometric constraint by adjusting the bulk to comply with the weight, and then weight normalize the result.
public interface AntiFix {
    func anti_fix() -> Output;
}
public static const anti_fix: AntiFixPrefixOrPostfix = AntiFixPrefixOrPostfix {};
public struct AntiFixPrefixOrPostfix {}
extension AntiFixPrefixOrPostfix for A: AntiFix {
    func operator/(rhs: A) -> A.Output {
        return rhs.anti_fix();
    }
}
__include ./impls/anti_fix;

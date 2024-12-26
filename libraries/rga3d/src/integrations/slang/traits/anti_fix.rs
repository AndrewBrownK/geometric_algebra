using data::*;

/// AntiFix
/// Automatically fix the anti geometric constraint by adjusting the bulk to comply with the weight, and then weight normalize the result.
public interface AntiFix {
    fn anti_fix() -> AntiFix;
}
public struct anti_fix;
extension anti_fix {    associatedtype Output = A;
    func operator/(rhs: A) -> Self::Output {
        rhs.anti_fix()
    }
}
__include ./impls/anti_fix;

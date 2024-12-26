using data::*;

/// Fix
/// Automatically fix the geometric constraint by adjusting the weight to comply with the bulk, and then bulk normalize the result.
public interface Fix {
    fn fix() -> Fix;
}
public struct fix;
extension fix {    associatedtype Output = A;
    func operator/(rhs: A) -> Self::Output {
        rhs.fix()
    }
}
__include ./impls/fix;

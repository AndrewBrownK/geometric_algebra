using data::*;

/// Fix
/// Automatically fix the geometric constraint by adjusting the weight to comply with the bulk, and then bulk normalize the result.
public interface Fix {
    func fix() -> Output;
}
public struct fix {}
extension fix for A: Fix {
    func operator/(rhs: A) -> A.Output {
        return rhs.fix();
    }
}
__include ./impls/fix;

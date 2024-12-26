using data::*;

/// Bulk
/// This characterizes the flat aspect's relationship with the origin.
public interface Bulk {
    associatedtype Output;
    fn bulk() -> Self::Output;
}
public struct bulk;
extension bulk {    associatedtype Output = <A as Bulk>::Output;
    func operator/(rhs: A) -> Self::Output {
        rhs.bulk()
    }
}
__include ./impls/bulk;

using data::*;

/// FlatBulk
/// This characterizes the flat aspect's relationship with the origin.
public interface FlatBulk {
    associatedtype Output;
    fn flat_bulk() -> Self::Output;
}
public struct flat_bulk;
extension flat_bulk {    associatedtype Output = <A as FlatBulk>::Output;
    func operator/(rhs: A) -> Self::Output {
        rhs.flat_bulk()
    }
}
__include ./impls/flat_bulk;

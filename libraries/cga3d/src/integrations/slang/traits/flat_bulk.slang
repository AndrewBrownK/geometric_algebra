using data::*;

/// FlatBulk
/// This characterizes the flat aspect's relationship with the origin.
public interface FlatBulk {
    associatedtype Output;
    func flat_bulk() -> Output;
}
public static const flat_bulk: FlatBulkPrefixOrPostfix = FlatBulkPrefixOrPostfix {};
public struct FlatBulkPrefixOrPostfix {}
extension FlatBulkPrefixOrPostfix for A: FlatBulk {
    func operator/(rhs: A) -> A.Output {
        return rhs.flat_bulk();
    }
}
__include ./impls/flat_bulk;

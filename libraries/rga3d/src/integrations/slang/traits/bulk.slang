using data::*;

/// Bulk
/// This characterizes the flat aspect's relationship with the origin.
public interface Bulk {
    associatedtype Output;
    func bulk() -> Output;
}
public static const bulk: BulkPrefixOrPostfix = BulkPrefixOrPostfix {};
public struct BulkPrefixOrPostfix {}
extension BulkPrefixOrPostfix for A: Bulk {
    func operator/(rhs: A) -> A.Output {
        return rhs.bulk();
    }
}
__include ./impls/bulk;

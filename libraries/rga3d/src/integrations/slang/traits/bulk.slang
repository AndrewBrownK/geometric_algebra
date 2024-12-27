using data::*;

/// Bulk
/// This characterizes the flat aspect's relationship with the origin.
public interface Bulk {
    associatedtype Output;
    func bulk() -> Output;
}
public const static bulk: BulkInfix = BulkInfix {}
public struct BulkInfix {}
extension BulkInfix for A: Bulk {
    func operator/(rhs: A) -> A.Output {
        return rhs.bulk();
    }
}
__include ./impls/bulk;

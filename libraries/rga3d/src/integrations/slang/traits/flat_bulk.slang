using data::*;

/// FlatBulk
/// This characterizes the flat aspect's relationship with the origin.
public interface FlatBulk {
    associatedtype Output;
    func flat_bulk() -> Output;
}
public const static flat_bulk: FlatBulkInfix = FlatBulkInfix {}
public struct FlatBulkInfix {}
extension FlatBulkInfix for A: FlatBulk {
    func operator/(rhs: A) -> A.Output {
        return rhs.flat_bulk();
    }
}
__include ./impls/flat_bulk;

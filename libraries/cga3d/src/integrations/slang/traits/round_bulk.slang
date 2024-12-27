using data::*;

/// RoundBulk
/// This is the aspect of a round object that characterizes the carrier's relationship with the origin.
public interface RoundBulk {
    associatedtype Output;
    func round_bulk() -> Output;
}
public static const round_bulk: RoundBulkPrefixOrPostfix = RoundBulkPrefixOrPostfix {};
public struct RoundBulkPrefixOrPostfix {}
extension RoundBulkPrefixOrPostfix for A: RoundBulk {
    func operator/(rhs: A) -> A.Output {
        return rhs.round_bulk();
    }
}
__include ./impls/round_bulk;

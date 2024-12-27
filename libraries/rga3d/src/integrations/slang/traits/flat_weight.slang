using data::*;

/// FlatWeight
/// This characterizes the flat aspect's relationship with the horizon.
public interface FlatWeight {
    associatedtype Output;
    func flat_weight() -> Output;
}
public const static flat_weight: FlatWeightInfix = FlatWeightInfix {}
public struct FlatWeightInfix {}
extension FlatWeightInfix for A: FlatWeight {
    func operator/(rhs: A) -> A.Output {
        return rhs.flat_weight();
    }
}
__include ./impls/flat_weight;

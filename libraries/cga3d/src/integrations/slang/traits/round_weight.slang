using data::*;

/// RoundWeight
/// This is the aspect of a round object that characterizes the carrier's relationship with the horizon.
public interface RoundWeight {
    associatedtype Output;
    func round_weight() -> Output;
}
public static const round_weight: RoundWeightPrefixOrPostfix = RoundWeightPrefixOrPostfix {};
public struct RoundWeightPrefixOrPostfix {}
extension RoundWeightPrefixOrPostfix for A: RoundWeight {
    func operator/(rhs: A) -> A.Output {
        return rhs.round_weight();
    }
}
__include ./impls/round_weight;

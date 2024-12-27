using data::*;

/// Weight
/// This characterizes the flat aspect's relationship with the horizon.
public interface Weight {
    associatedtype Output;
    func weight() -> Output;
}
public const static weight: WeightInfix = WeightInfix {}
public struct WeightInfix {}
extension WeightInfix for A: Weight {
    func operator/(rhs: A) -> A.Output {
        return rhs.weight();
    }
}
__include ./impls/weight;

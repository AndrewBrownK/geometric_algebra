using data::*;

/// Weight
/// This characterizes the flat aspect's relationship with the horizon.
public interface Weight {
    associatedtype Output;
    func weight() -> Output;
}
public struct weight {}
extension weight for A: Weight {
    func operator/(rhs: A) -> A.Output {
        return rhs.weight();
    }
}
__include ./impls/weight;

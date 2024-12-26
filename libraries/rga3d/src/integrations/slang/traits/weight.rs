using data::*;

/// Weight
/// This characterizes the flat aspect's relationship with the horizon.
public interface Weight {
    associatedtype Output;
    fn weight() -> Self::Output;
}
public struct weight;
extension weight {    associatedtype Output = <A as Weight>::Output;
    func operator/(rhs: A) -> Self::Output {
        rhs.weight()
    }
}
__include ./impls/weight;

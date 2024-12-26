using data::*;

/// FlatWeight
/// This characterizes the flat aspect's relationship with the horizon.
public interface FlatWeight {
    associatedtype Output;
    fn flat_weight() -> Self::Output;
}
public struct flat_weight;
extension flat_weight {    associatedtype Output = <A as FlatWeight>::Output;
    func operator/(rhs: A) -> Self::Output {
        rhs.flat_weight()
    }
}
__include ./impls/flat_weight;

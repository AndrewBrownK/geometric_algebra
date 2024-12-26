using data::*;

/// RightDual
/// This dual is the "Metric Right Dual". To take this dual of an object, the object is multiplied by the metric, and then we take the right complement. This will turn a Scalar into an AntiScalar, a Vector into an AntiVector, so on, and vice versa. The use of the metric may give distinct results from a simple right complement, typically by changing the coefficient on some terms (1, -1, or 0)
public interface RightDual {
    associatedtype Output;
    fn right_dual() -> Self::Output;
}
public struct right_dual;
extension right_dual {    associatedtype Output = <A as RightDual>::Output;
    func operator/(rhs: A) -> Self::Output {
        rhs.right_dual()
    }
}
__include ./impls/right_dual;

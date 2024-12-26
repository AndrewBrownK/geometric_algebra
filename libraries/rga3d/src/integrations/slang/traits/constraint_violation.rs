using data::*;

/// ConstraintViolation
/// Not every combinations of floats is valid geometry. Some types of objects are required to fulfill a constraint in order to be valid geometry. We call this the geometric constraint. If a type of object may possibly violate this constraint, then it will implement this trait. The constraint is violated if a non-zero value is returned. See also ConstraintValid and Fix.
public interface ConstraintViolation {
    associatedtype Output;
    fn constraint_violation() -> Self::Output;
}
public struct constraint_violation;
extension constraint_violation {    associatedtype Output = <A as ConstraintViolation>::Output;
    func operator/(rhs: A) -> Self::Output {
        rhs.constraint_violation()
    }
}
__include ./impls/constraint_violation;

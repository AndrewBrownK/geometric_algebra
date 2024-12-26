using data::*;

/// AntiConstraintViolation
/// Not every combinations of floats is valid geometry. Some types of objects are required to fulfill a constraint in order to be valid geometry. We call this the (anti) geometric constraint. If a type of object may possibly violate this constraint, then it will implement this trait. The constraint is violated if a non-zero value is returned. See also AntiConstraintValid and AntiFix.
public interface AntiConstraintViolation {
    associatedtype Output;
    fn anti_constraint_violation() -> Self::Output;
}
public struct anti_constraint_violation;
extension anti_constraint_violation {    associatedtype Output = <A as AntiConstraintViolation>::Output;
    func operator/(rhs: A) -> Self::Output {
        rhs.anti_constraint_violation()
    }
}
__include ./impls/anti_constraint_violation;

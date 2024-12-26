using data::*;

/// ConstraintValid
/// Implementors of this trait cannot violate the geometric constraint. They always represent valid geometry. This trait does not exist to perform any calculation, it just exists to serve as contrasting information side-by-side with ConstraintViolation. See also ConstraintViolation and Fix.
public interface ConstraintValid {
    fn constraint_valid() -> ConstraintValid;
}
public struct constraint_valid;
extension constraint_valid {    associatedtype Output = A;
    func operator/(rhs: A) -> Self::Output {
        rhs.constraint_valid()
    }
}
__include ./impls/constraint_valid;

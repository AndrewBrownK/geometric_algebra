using data::*;

/// ConstraintValid
/// Implementors of this trait cannot violate the geometric constraint. They always represent valid geometry. This trait does not exist to perform any calculation, it just exists to serve as contrasting information side-by-side with ConstraintViolation. See also ConstraintViolation and Fix.
public interface ConstraintValid {
    func constraint_valid() -> Output;
}
public static const constraint_valid: ConstraintValidPrefixOrPostfix = ConstraintValidPrefixOrPostfix {};
public struct ConstraintValidPrefixOrPostfix {}
extension ConstraintValidPrefixOrPostfix for A: ConstraintValid {
    func operator/(rhs: A) -> A.Output {
        return rhs.constraint_valid();
    }
}
__include ./impls/constraint_valid;

using data::*;

/// AntiConstraintValid
/// Implementors of this trait cannot violate the anti geometric constraint. They always represent valid geometry. This trait does not exist to perform any calculation, it just exists to serve as contrasting information side-by-side with AntiConstraintViolation. See also AntiConstraintViolation and AntiFix.
public interface AntiConstraintValid {
    fn anti_constraint_valid() -> AntiConstraintValid;
}
public struct anti_constraint_valid;
extension anti_constraint_valid {    associatedtype Output = A;
    func operator/(rhs: A) -> Self::Output {
        rhs.anti_constraint_valid()
    }
}
__include ./impls/anti_constraint_valid;

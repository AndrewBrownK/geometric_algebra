using data::*;

/// AntiConstraintValid
/// Implementors of this trait cannot violate the anti geometric constraint. They always represent valid geometry. This trait does not exist to perform any calculation, it just exists to serve as contrasting information side-by-side with AntiConstraintViolation. See also AntiConstraintViolation and AntiFix.
public interface AntiConstraintValid {
    func anti_constraint_valid() -> Output;
}
public const static anti_constraint_valid: AntiConstraintValidInfix = AntiConstraintValidInfix {}
public struct AntiConstraintValidInfix {}
extension AntiConstraintValidInfix for A: AntiConstraintValid {
    func operator/(rhs: A) -> A.Output {
        return rhs.anti_constraint_valid();
    }
}
__include ./impls/anti_constraint_valid;

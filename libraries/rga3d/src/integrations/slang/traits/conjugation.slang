using data::*;

/// Conjugation
/// This composes the reverse and grade involution (automorphism).
public interface Conjugation {
    func conjugation() -> Output;
}
public const static conjugation: ConjugationInfix = ConjugationInfix {}
public struct ConjugationInfix {}
extension ConjugationInfix for A: Conjugation {
    func operator/(rhs: A) -> A.Output {
        return rhs.conjugation();
    }
}
__include ./impls/conjugation;

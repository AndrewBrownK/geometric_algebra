using data::*;

/// Conjugation
/// This composes the reverse and grade involution (automorphism).
public interface Conjugation {
    func conjugation() -> Output;
}
public static const conjugation: ConjugationPrefixOrPostfix = ConjugationPrefixOrPostfix {};
public struct ConjugationPrefixOrPostfix {}
extension ConjugationPrefixOrPostfix for A: Conjugation {
    func operator/(rhs: A) -> A.Output {
        return rhs.conjugation();
    }
}
__include ./impls/conjugation;

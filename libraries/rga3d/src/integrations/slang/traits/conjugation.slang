implementing rga3d;using data;

/// Conjugation
/// This composes the reverse and grade involution (automorphism).
public interface Conjugation {
    func conjugation() -> This;
}
public static const ConjugationPrefixOrPostfix conjugation = ConjugationPrefixOrPostfix();
public struct ConjugationPrefixOrPostfix {}
__include "./impls/conjugation";

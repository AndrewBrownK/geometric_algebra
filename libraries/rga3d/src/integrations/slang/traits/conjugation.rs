using data::*;

/// Conjugation
/// This composes the reverse and grade involution (automorphism).
public interface Conjugation {
    fn conjugation() -> Conjugation;
}
public struct conjugation;
extension conjugation {    associatedtype Output = A;
    func operator/(rhs: A) -> Self::Output {
        rhs.conjugation()
    }
}
__include ./impls/conjugation;

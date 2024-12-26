using data::*;

/// AntiAutoMorphism
/// Negate every BasisElement with an odd AntiGrade.
public interface AntiAutoMorphism {
    fn anti_auto_morphism() -> AntiAutoMorphism;
}
public struct anti_auto_morphism;
extension anti_auto_morphism {    associatedtype Output = A;
    func operator/(rhs: A) -> Self::Output {
        rhs.anti_auto_morphism()
    }
}
__include ./impls/anti_auto_morphism;

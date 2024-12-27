using data::*;

/// AntiAutoMorphism
/// Negate every BasisElement with an odd AntiGrade.
public interface AntiAutoMorphism {
    func anti_auto_morphism() -> Output;
}
public struct anti_auto_morphism {}
extension anti_auto_morphism for A: AntiAutoMorphism {
    func operator/(rhs: A) -> A.Output {
        return rhs.anti_auto_morphism();
    }
}
__include ./impls/anti_auto_morphism;

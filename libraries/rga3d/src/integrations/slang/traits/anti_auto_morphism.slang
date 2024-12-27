using data::*;

/// AntiAutoMorphism
/// Negate every BasisElement with an odd AntiGrade.
public interface AntiAutoMorphism {
    func anti_auto_morphism() -> Output;
}
public const static anti_auto_morphism: AntiAutoMorphismInfix = AntiAutoMorphismInfix {}
public struct AntiAutoMorphismInfix {}
extension AntiAutoMorphismInfix for A: AntiAutoMorphism {
    func operator/(rhs: A) -> A.Output {
        return rhs.anti_auto_morphism();
    }
}
__include ./impls/anti_auto_morphism;

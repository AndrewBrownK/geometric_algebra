implementing rga3d;using data;

/// AntiAutoMorphism
/// Negate every BasisElement with an odd AntiGrade.
public interface AntiAutoMorphism {
    func anti_auto_morphism() -> This;
}
public static const AntiAutoMorphismPrefixOrPostfix anti_auto_morphism = AntiAutoMorphismPrefixOrPostfix();
public struct AntiAutoMorphismPrefixOrPostfix {}
__include "./impls/anti_auto_morphism";

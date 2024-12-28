implementing rga3d;using data;

/// AutoMorphism
/// Negate every BasisElement with an odd Grade. Also known as grade involution.
public interface AutoMorphism {
    func auto_morphism() -> This;
}
public static const AutoMorphismPrefixOrPostfix auto_morphism = AutoMorphismPrefixOrPostfix();
public struct AutoMorphismPrefixOrPostfix {}
__include "./impls/auto_morphism";

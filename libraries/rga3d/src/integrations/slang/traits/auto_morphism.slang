using data::*;

/// AutoMorphism
/// Negate every BasisElement with an odd Grade. Also known as grade involution.
public interface AutoMorphism {
    func auto_morphism() -> Output;
}
public const static auto_morphism: AutoMorphismInfix = AutoMorphismInfix {}
public struct AutoMorphismInfix {}
extension AutoMorphismInfix for A: AutoMorphism {
    func operator/(rhs: A) -> A.Output {
        return rhs.auto_morphism();
    }
}
__include ./impls/auto_morphism;

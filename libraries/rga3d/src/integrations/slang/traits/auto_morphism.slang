using data::*;

/// AutoMorphism
/// Negate every BasisElement with an odd Grade. Also known as grade involution.
public interface AutoMorphism {
    func auto_morphism() -> Output;
}
public struct auto_morphism {}
extension auto_morphism for A: AutoMorphism {
    func operator/(rhs: A) -> A.Output {
        return rhs.auto_morphism();
    }
}
__include ./impls/auto_morphism;

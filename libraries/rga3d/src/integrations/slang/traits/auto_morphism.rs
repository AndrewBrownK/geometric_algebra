using data::*;

/// AutoMorphism
/// Negate every BasisElement with an odd Grade. Also known as grade involution.
public interface AutoMorphism {
    fn auto_morphism() -> AutoMorphism;
}
public struct auto_morphism;
extension auto_morphism {    associatedtype Output = A;
    func operator/(rhs: A) -> Self::Output {
        rhs.auto_morphism()
    }
}
__include ./impls/auto_morphism;

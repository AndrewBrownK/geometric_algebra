using data::*;

/// AntiSandwich
/// The anti-sandwich is the dual to the sandwich, using the AntiProduct instead of the Product, and the AntiReverse instead of the Reverse. This is also used to represent geometric transformations, for example reflecting across a plane or rotating around a line. The Sandwich and AntiSandwich are not identical for the purposes of transforming geometry, you simply choose which one to use depending on your geometric interpretation and the algebra. For example, in G(3,0,1) you may interpret grade 1 vectors as points or planes, since they are dual to one another. The sandwich product gives euclidean transformations in the grade 1 = planes interpretation, and the AntiSandwich gives euclidean transformations in the grade 1 = points interpretation.
public interface AntiSandwich<T> {
    associatedtype Output;
    func anti_sandwich(other: T) -> Output;
}
public struct anti_sandwich {}
public struct anti_sandwich_partial<A> { a: A }
extension anti_sandwich_partial<A> for A: AntiSandwich<B> {
    func operator/(rhs: B) -> A.Output {
        return this.a.anti_sandwich(rhs);
    }
}
__include ./impls/anti_sandwich;

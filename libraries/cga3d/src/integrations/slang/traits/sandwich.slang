using data::*;

/// Sandwich
/// The so-called "sandwich product" squeezes some factor A between another factor B and the reversal of B. This is frequently used to represent geometric transformations, for example reflecting across a plane or rotating around a line.
public interface Sandwich<T> {
    associatedtype Output;
    func sandwich(other: T) -> Output;
}
public static const sandwich: SandwichInfix = SandwichInfix {};
public struct SandwichInfix {}
public struct SandwichInfixPartial<A> { a: A }
extension SandwichInfixPartial<A> for A: Sandwich<B> {
    func operator/(rhs: B) -> A.Output {
        return this.a.sandwich(rhs);
    }
}
__include ./impls/sandwich;

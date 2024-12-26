using data::*;

/// Sandwich
/// The so-called "sandwich product" squeezes some factor A between another factor B and the reversal of B. This is frequently used to represent geometric transformations, for example reflecting across a plane or rotating around a line.
public interface Sandwich<T> {
    associatedtype Output;
    fn sandwich(other: T) -> Self::Output;
}
public struct sandwich;
public struct sandwich_partial<A> { a: A }
extension sandwich_partial<A> {    associatedtype Output = <A as Sandwich<B>>::Output;
    func operator/(rhs: B) -> Self::Output {
        this.a.sandwich(rhs)
    }
}
__include ./impls/sandwich;

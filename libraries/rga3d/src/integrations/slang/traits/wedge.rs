using data::*;

/// Wedge
/// The Wedge product (also known as "Exterior Product" or Grassmann's "Progressive Combinatorial Product") combines BasisElements into higher grade BasisElements. For example, wedge(e1, e2) = e12, and wedge(e1, e23) = e123. The Wedge product is anti-commutative, so wedge(a, b) = -wedge(b, a). A non-scalar element wedged with itself is zero. This behaves something like a union of the subscripts in the BasisElements.
public interface Wedge<T> {
    associatedtype Output;
    fn wedge(other: T) -> Self::Output;
}
public struct wedge;
public struct wedge_partial<A> { a: A }
extension wedge_partial<A> {    associatedtype Output = <A as Wedge<B>>::Output;
    func operator/(rhs: B) -> Self::Output {
        this.a.wedge(rhs)
    }
}
__include ./impls/wedge;

using data::*;

/// WeightExpansion
/// This is an interior product (contrast with inner product and exterior product). The interior products are derived by Wedging (or AntiWedging) one object with the Dual (or AntiDual) of another object.
public interface WeightExpansion<T> {
    associatedtype Output;
    func weight_expansion(other: T) -> Output;
}
public static const weight_expansion: WeightExpansionInfix = WeightExpansionInfix {};
public struct WeightExpansionInfix {}
public struct WeightExpansionInfixPartial<A> { a: A }
extension WeightExpansionInfixPartial<A> for A: WeightExpansion<B> {
    func operator/(rhs: B) -> A.Output {
        return this.a.weight_expansion(rhs);
    }
}
__include ./impls/weight_expansion;

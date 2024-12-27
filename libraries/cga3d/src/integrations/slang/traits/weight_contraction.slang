using data::*;

/// WeightContraction
/// This is an interior product (contrast with inner product and exterior product). The interior products are derived by Wedging (or AntiWedging) one object with the Dual (or AntiDual) of another object.
public interface WeightContraction<T> {
    associatedtype Output;
    func weight_contraction(other: T) -> Output;
}
public static const weight_contraction: WeightContractionInfix = WeightContractionInfix {};
public struct WeightContractionInfix {}
public struct WeightContractionInfixPartial<A> { a: A }
extension WeightContractionInfixPartial<A> for A: WeightContraction<B> {
    func operator/(rhs: B) -> A.Output {
        return this.a.weight_contraction(rhs);
    }
}
__include ./impls/weight_contraction;

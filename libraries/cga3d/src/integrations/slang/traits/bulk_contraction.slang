using data::*;

/// BulkContraction
/// This is an interior product (contrast with inner product and exterior product). The interior products are derived by Wedging (or AntiWedging) one object with the Dual (or AntiDual) of another object.
public interface BulkContraction<T> {
    associatedtype Output;
    func bulk_contraction(other: T) -> Output;
}
public static const bulk_contraction: BulkContractionInfix = BulkContractionInfix {};
public struct BulkContractionInfix {}
public struct BulkContractionInfixPartial<A> { a: A }
extension BulkContractionInfixPartial<A> for A: BulkContraction<B> {
    func operator/(rhs: B) -> A.Output {
        return this.a.bulk_contraction(rhs);
    }
}
__include ./impls/bulk_contraction;

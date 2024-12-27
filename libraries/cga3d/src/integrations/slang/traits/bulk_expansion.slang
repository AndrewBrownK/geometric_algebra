using data::*;

/// BulkExpansion
/// This is an interior product (contrast with inner product and exterior product). The interior products are derived by Wedging (or AntiWedging) one object with the Dual (or AntiDual) of another object.
public interface BulkExpansion<T> {
    associatedtype Output;
    func bulk_expansion(other: T) -> Output;
}
public static const bulk_expansion: BulkExpansionInfix = BulkExpansionInfix {};
public struct BulkExpansionInfix {}
public struct BulkExpansionInfixPartial<A> { a: A }
extension BulkExpansionInfixPartial<A> for A: BulkExpansion<B> {
    func operator/(rhs: B) -> A.Output {
        return this.a.bulk_expansion(rhs);
    }
}
__include ./impls/bulk_expansion;

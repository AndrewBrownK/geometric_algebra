using data::*;

/// Into
/// Reliably convert one type into another.
public interface Into<Output> {
    func into() -> Output;
}
public interface From<Other> {
    associatedtype Self;
    static func from(other: Other) -> Self;
}
__include ./impls/into;

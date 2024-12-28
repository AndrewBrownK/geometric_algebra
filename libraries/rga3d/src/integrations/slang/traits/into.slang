implementing rga3d;using data;

/// Into
/// Reliably convert one type into another.
public interface Into<Output> {
    func into() -> Output;
}
public interface From<Other> {
    static func from(Other other) -> This;
}
__include "./impls/into";

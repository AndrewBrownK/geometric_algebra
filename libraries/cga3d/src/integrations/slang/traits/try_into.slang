using data::*;

/// TryInto
/// Fallibly convert one type into another.
public interface TryInto<Output> {
    func try_into() -> Optional<Output>;
}
public interface TryFrom<Other> {
    associatedtype Self;
    static func try_from(other: Other) -> Optional<Self>;
}
__include ./impls/try_into;

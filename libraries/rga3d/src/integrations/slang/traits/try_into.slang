implementing rga3d;using data;

/// TryInto
/// Fallibly convert one type into another.
public interface TryInto<Output> {
    func try_into() -> Optional<Output>;
}
public interface TryFrom<Other> {
    static func try_from(Other other) -> Optional<This>;
}
__include "./impls/try_into";

implementing rga3d;using data;

/// One
/// The scalar element set to one, and all other elements set to zero.
public interface One {
    func one() -> This;
}
__include "./impls/one";

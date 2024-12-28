implementing rga3d;using data;

/// AntiOne
/// The anti-scalar element set to one, and all other elements set to zero.
public interface AntiOne {
    func anti_one() -> This;
}
__include "./impls/anti_one";

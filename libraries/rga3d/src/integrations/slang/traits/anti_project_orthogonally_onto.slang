implementing rga3d;using data;

/// AntiProjectOrthogonallyOnto
/// Typically involves bringing a higher dimensional object to a lower dimensional object.
public interface AntiProjectOrthogonallyOnto<T> {
    associatedtype AntiProjectOrthogonallyOntoOutput;
    func anti_project_orthogonally_onto(other: T) -> AntiProjectOrthogonallyOntoOutput;
}
public static const AntiProjectOrthogonallyOntoInfix anti_project_orthogonally_onto = AntiProjectOrthogonallyOntoInfix();
public struct AntiProjectOrthogonallyOntoInfix {}
public struct AntiProjectOrthogonallyOntoInfixPartial<A> { internal A a; }
__include "./impls/anti_project_orthogonally_onto";

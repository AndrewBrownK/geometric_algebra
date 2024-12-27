using data::*;

/// AntiProjectOrthogonallyOnto
/// Typically involves bringing a higher dimensional object to a lower dimensional object.
public interface AntiProjectOrthogonallyOnto<T> {
    associatedtype Output;
    func anti_project_orthogonally_onto(other: T) -> Output;
}
public const static anti_project_orthogonally_onto: AntiProjectOrthogonallyOntoInfix = AntiProjectOrthogonallyOntoInfix {}
public struct AntiProjectOrthogonallyOntoInfix {}
public struct AntiProjectOrthogonallyOntoInfixPartial<A> { a: A }
extension AntiProjectOrthogonallyOntoInfixPartial<A> for A: AntiProjectOrthogonallyOnto<B> {
    func operator/(rhs: B) -> A.Output {
        return this.a.anti_project_orthogonally_onto(rhs);
    }
}
__include ./impls/anti_project_orthogonally_onto;

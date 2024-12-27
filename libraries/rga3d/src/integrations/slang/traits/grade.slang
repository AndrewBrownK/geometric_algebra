using data::*;

/// Grade
/// A multivector may have uniform grade, or mixed grade, depending on the grades of its elements. This trait only characterizes uniform grade multivectors.
public interface Grade {
    func grade() -> uint;
}
public const static grade: GradeInfix = GradeInfix {}
public struct GradeInfix {}
__include ./impls/grade;

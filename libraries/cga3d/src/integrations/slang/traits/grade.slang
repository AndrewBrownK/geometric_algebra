using data::*;

/// Grade
/// A multivector may have uniform grade, or mixed grade, depending on the grades of its elements. This trait only characterizes uniform grade multivectors.
public interface Grade {
    func grade() -> uint;
}
__include ./impls/grade;

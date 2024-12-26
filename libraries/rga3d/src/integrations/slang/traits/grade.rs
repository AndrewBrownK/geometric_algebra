using data::*;

/// Grade
/// A multivector may have uniform grade, or mixed grade, depending on the grades of its elements. This trait only characterizes uniform grade multivectors.
public interface Grade {
    fn grade() -> uint;
}
public struct grade;
__include ./impls/grade;

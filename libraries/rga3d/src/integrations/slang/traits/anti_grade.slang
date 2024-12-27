using data::*;

/// AntiGrade
/// The AntiGrade can be described as the missing Grade with respect to an AntiScalar. This trait only characterizes uniform anti-grade multivectors.
public interface AntiGrade {
    func anti_grade() -> uint;
}
public struct anti_grade {}
__include ./impls/anti_grade;

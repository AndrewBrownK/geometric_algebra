implementing rga3d;using data;

/// AntiGrade
/// The AntiGrade can be described as the missing Grade with respect to an AntiScalar. This trait only characterizes uniform anti-grade multivectors.
public interface AntiGrade {
    func anti_grade() -> uint;
}
__include "./impls/anti_grade";

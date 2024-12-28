implementing rga3d;using data;

/// Weight
/// This characterizes the flat aspect's relationship with the horizon.
public interface Weight {
    associatedtype WeightOutput;
    func weight() -> Output;
}
public static const WeightPrefixOrPostfix weight = WeightPrefixOrPostfix();
public struct WeightPrefixOrPostfix {}
__include "./impls/weight";

implementing rga3d;using data;

/// FlatBulk
/// This characterizes the flat aspect's relationship with the origin.
public interface FlatBulk {
    associatedtype FlatBulkOutput;
    func flat_bulk() -> Output;
}
public static const FlatBulkPrefixOrPostfix flat_bulk = FlatBulkPrefixOrPostfix();
public struct FlatBulkPrefixOrPostfix {}
__include "./impls/flat_bulk";

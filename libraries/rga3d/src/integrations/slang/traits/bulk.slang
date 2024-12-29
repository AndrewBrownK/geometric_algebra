implementing rga3d;using data;

/// Bulk
/// This characterizes the flat aspect's relationship with the origin.
public interface Bulk {
    associatedtype BulkOutput;
    func bulk() -> BulkOutput;
}
public static const BulkPrefixOrPostfix bulk = BulkPrefixOrPostfix();
public struct BulkPrefixOrPostfix {}
__include "./impls/bulk";

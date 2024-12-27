implementing cga3d;
using data::*;

/// FlatPoint
public struct FlatPoint {
    // e15, e25, e35, e45
    internal group0: float4
}
extension FlatPoint {
    public property e15: float {
        get { return group0[0]; }
        set { group0[0] = newValue; }
    }
    public property e25: float {
        get { return group0[1]; }
        set { group0[1] = newValue; }
    }
    public property e35: float {
        get { return group0[2]; }
        set { group0[2] = newValue; }
    }
    public property e45: float {
        get { return group0[3]; }
        set { group0[3] = newValue; }
    }
    public static func from_elements(
        e15: float, e25: float, e35: float, e45: float
    ) -> FlatPoint {
        return FlatPoint { float4(e15, e25, e35, e45) };
    }
    internal static func from_groups(
        g0: float4
    ) -> FlatPoint {
        return FlatPoint {
            group0: g0
        };
    }
}
extension FlatPoint: IComparable {
    bool lessThan(IComparable another) {
        FlatPoint other = (FlatPoint)another;
        return this.group0.lessThan(other.group0);
    }
    bool equals(IComparable another) {
        FlatPoint other = (FlatPoint)another;
        return this.group0.equals(other.group0);
    }
}
__include "impls/flat_point";

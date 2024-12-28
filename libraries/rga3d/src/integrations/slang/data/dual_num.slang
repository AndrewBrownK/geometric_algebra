implementing rga3d;
using data;

/// DualNum
public struct DualNum {
    // scalar, e1234, 0, 0
    internal float4 group0;
}
public extension DualNum {
    public property scalar: float {
        get { return group0[0]; }
        set { group0[0] = newValue; }
    }
    public property e1234: float {
        get { return group0[1]; }
        set { group0[1] = newValue; }
    }
    public static func from_elements(
        scalar: float, e1234: float
    ) -> DualNum {
        return DualNum(float4(scalar, e1234, 0.0, 0.0));
    }
    internal static func from_groups(
        g0: float2
    ) -> DualNum {
        return DualNum(float4(g0, 0.0, 0.0));
    }
}
public extension DualNum: IComparable {
    public bool equals(DualNum other) {
        return equalsHelper(this.group0, other.group0);
    }
    public bool lessThan(DualNum other) {
        return lessThanHelper(this.group0, other.group0);
    }
    public bool lessThanOrEquals(DualNum other) {
        return lessThanOrEqualsHelper(this.group0, other.group0);
    }
}
__include "impls/dual_num";

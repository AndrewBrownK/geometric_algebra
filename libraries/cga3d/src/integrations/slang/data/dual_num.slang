implementing cga3d;
using data::*;

/// DualNum
public struct DualNum {
    // e5, e12345, 0, 0
    internal group0: float4
}
extension DualNum {
    public property e5: float {
        get { return group0[0]; }
        set { group0[0] = newValue; }
    }
    public property e12345: float {
        get { return group0[1]; }
        set { group0[1] = newValue; }
    }
    public static func from_elements(
        e5: float, e12345: float
    ) -> DualNum {
        return DualNum { float4(e5, e12345, 0.0, 0.0) };
    }
    internal static func from_groups(
        g0: float2
    ) -> DualNum {
        return DualNum {
            group0: g0
        };
    }
}
extension DualNum: IComparable {
    bool lessThan(IComparable another) {
        DualNum other = (DualNum)another;
        return this.group0.lessThan(other.group0);
    }
    bool equals(IComparable another) {
        DualNum other = (DualNum)another;
        return this.group0.equals(other.group0);
    }
}
__include "impls/dual_num";

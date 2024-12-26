implementing rga3d;
using data::*;

/// DualNum
public struct DualNum {
    // scalar, e1234, 0, 0
    internal group0: float4
}
extension DualNum {
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
        return DualNum { float4(scalar, e1234, 0.0, 0.0) };
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

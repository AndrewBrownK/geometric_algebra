implementing cga3d;
using data::*;

/// AntiDualNum
public struct AntiDualNum {
    // e3215, scalar, 0, 0
    internal group0: float4
}
extension AntiDualNum {
    public property e3215: float {
        get { return group0[0]; }
        set { group0[0] = newValue; }
    }
    public property scalar: float {
        get { return group0[1]; }
        set { group0[1] = newValue; }
    }
    public static func from_elements(
        e3215: float, scalar: float
    ) -> AntiDualNum {
        return AntiDualNum { float4(e3215, scalar, 0.0, 0.0) };
    }
    internal static func from_groups(
        g0: float2
    ) -> AntiDualNum {
        return AntiDualNum {
            group0: g0
        };
    }
}
extension AntiDualNum: IComparable {
    bool lessThan(IComparable another) {
        AntiDualNum other = (AntiDualNum)another;
        return this.group0.lessThan(other.group0);
    }
    bool equals(IComparable another) {
        AntiDualNum other = (AntiDualNum)another;
        return this.group0.equals(other.group0);
    }
}
__include "impls/anti_dual_num";

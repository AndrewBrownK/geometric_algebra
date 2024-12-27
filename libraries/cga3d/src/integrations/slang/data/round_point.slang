implementing cga3d;
using data::*;

/// RoundPoint
public struct RoundPoint {
    // e1, e2, e3, e4
    internal group0: float4,
    // e5, 0, 0, 0
    internal group1: float4
}
extension RoundPoint {
    public property e1: float {
        get { return group0[0]; }
        set { group0[0] = newValue; }
    }
    public property e2: float {
        get { return group0[1]; }
        set { group0[1] = newValue; }
    }
    public property e3: float {
        get { return group0[2]; }
        set { group0[2] = newValue; }
    }
    public property e4: float {
        get { return group0[3]; }
        set { group0[3] = newValue; }
    }
    public property e5: float {
        get { return group1[0]; }
        set { group1[0] = newValue; }
    }
    public static func from_elements(
        e1: float, e2: float, e3: float, e4: float, e5: float
    ) -> RoundPoint {
        return RoundPoint { float4(e1, e2, e3, e4), float4(e5, 0.0, 0.0, 0.0) };
    }
    internal static func from_groups(
        g0: float4, g1: float
    ) -> RoundPoint {
        return RoundPoint {
            group0: g0, group1: g1
        };
    }
}
extension RoundPoint: IComparable {
    bool lessThan(IComparable another) {
        RoundPoint other = (RoundPoint)another;
        if (this.group0 != other.group0)
            return this.group0.lessThan(other.group0);
        return this.group1.lessThan(other.group1);
    }
    bool equals(IComparable another) {
        RoundPoint other = (RoundPoint)another;
        return this.group0.equals(other.group0) && this.group1.equals(other.group1);
    }
}
__include "impls/round_point";

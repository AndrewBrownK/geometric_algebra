implementing cga3d;
using data::*;

/// AntiLine
public struct AntiLine {
    // e23, e31, e12, 0
    internal group0: float4,
    // e15, e25, e35, 0
    internal group1: float4
}
extension AntiLine {
    public property e23: float {
        get { return group0[0]; }
        set { group0[0] = newValue; }
    }
    public property e31: float {
        get { return group0[1]; }
        set { group0[1] = newValue; }
    }
    public property e12: float {
        get { return group0[2]; }
        set { group0[2] = newValue; }
    }
    public property e15: float {
        get { return group1[0]; }
        set { group1[0] = newValue; }
    }
    public property e25: float {
        get { return group1[1]; }
        set { group1[1] = newValue; }
    }
    public property e35: float {
        get { return group1[2]; }
        set { group1[2] = newValue; }
    }
    public static func from_elements(
        e23: float, e31: float, e12: float, e15: float, e25: float, e35: float
    ) -> AntiLine {
        return AntiLine { float4(e23, e31, e12, 0.0), float4(e15, e25, e35, 0.0) };
    }
    internal static func from_groups(
        g0: float3, g1: float3
    ) -> AntiLine {
        return AntiLine {
            group0: g0, group1: g1
        };
    }
}
extension AntiLine: IComparable {
    bool lessThan(IComparable another) {
        AntiLine other = (AntiLine)another;
        if (this.group0 != other.group0)
            return this.group0.lessThan(other.group0);
        return this.group1.lessThan(other.group1);
    }
    bool equals(IComparable another) {
        AntiLine other = (AntiLine)another;
        return this.group0.equals(other.group0) && this.group1.equals(other.group1);
    }
}
__include "impls/anti_line";

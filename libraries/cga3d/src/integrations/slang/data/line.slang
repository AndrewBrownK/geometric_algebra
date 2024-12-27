implementing cga3d;
using data::*;

/// Line
public struct Line {
    // e415, e425, e435, 0
    internal group0: float4,
    // e235, e315, e125, 0
    internal group1: float4
}
extension Line {
    public property e415: float {
        get { return group0[0]; }
        set { group0[0] = newValue; }
    }
    public property e425: float {
        get { return group0[1]; }
        set { group0[1] = newValue; }
    }
    public property e435: float {
        get { return group0[2]; }
        set { group0[2] = newValue; }
    }
    public property e235: float {
        get { return group1[0]; }
        set { group1[0] = newValue; }
    }
    public property e315: float {
        get { return group1[1]; }
        set { group1[1] = newValue; }
    }
    public property e125: float {
        get { return group1[2]; }
        set { group1[2] = newValue; }
    }
    public static func from_elements(
        e415: float, e425: float, e435: float, e235: float, e315: float, e125: float
    ) -> Line {
        return Line { float4(e415, e425, e435, 0.0), float4(e235, e315, e125, 0.0) };
    }
    internal static func from_groups(
        g0: float3, g1: float3
    ) -> Line {
        return Line {
            group0: g0, group1: g1
        };
    }
}
extension Line: IComparable {
    bool lessThan(IComparable another) {
        Line other = (Line)another;
        if (this.group0 != other.group0)
            return this.group0.lessThan(other.group0);
        return this.group1.lessThan(other.group1);
    }
    bool equals(IComparable another) {
        Line other = (Line)another;
        return this.group0.equals(other.group0) && this.group1.equals(other.group1);
    }
}
__include "impls/line";

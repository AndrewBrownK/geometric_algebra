implementing rga3d;
using data::*;

/// Line
public struct Line {
    // e41, e42, e43, 0
    internal group0: float4,
    // e23, e31, e12, 0
    internal group1: float4
}
extension Line {
    public property e41: float {
        get { return group0[0]; }
        set { group0[0] = newValue; }
    }
    public property e42: float {
        get { return group0[1]; }
        set { group0[1] = newValue; }
    }
    public property e43: float {
        get { return group0[2]; }
        set { group0[2] = newValue; }
    }
    public property e23: float {
        get { return group1[0]; }
        set { group1[0] = newValue; }
    }
    public property e31: float {
        get { return group1[1]; }
        set { group1[1] = newValue; }
    }
    public property e12: float {
        get { return group1[2]; }
        set { group1[2] = newValue; }
    }
    public static func from_elements(
        e41: float, e42: float, e43: float, e23: float, e31: float, e12: float
    ) -> Line {
        return Line { float4(e41, e42, e43, 0.0), float4(e23, e31, e12, 0.0) };
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

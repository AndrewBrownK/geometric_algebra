implementing cga3d;
using data::*;

/// Flector
public struct Flector {
    // e15, e25, e35, e45
    internal group0: float4,
    // e4235, e4315, e4125, e3215
    internal group1: float4
}
extension Flector {
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
    public property e4235: float {
        get { return group1[0]; }
        set { group1[0] = newValue; }
    }
    public property e4315: float {
        get { return group1[1]; }
        set { group1[1] = newValue; }
    }
    public property e4125: float {
        get { return group1[2]; }
        set { group1[2] = newValue; }
    }
    public property e3215: float {
        get { return group1[3]; }
        set { group1[3] = newValue; }
    }
    public static func from_elements(
        e15: float, e25: float, e35: float, e45: float, e4235: float, e4315: float, e4125: float, e3215: float
    ) -> Flector {
        return Flector { float4(e15, e25, e35, e45), float4(e4235, e4315, e4125, e3215) };
    }
    internal static func from_groups(
        g0: float4, g1: float4
    ) -> Flector {
        return Flector {
            group0: g0, group1: g1
        };
    }
}
extension Flector: IComparable {
    bool lessThan(IComparable another) {
        Flector other = (Flector)another;
        if (this.group0 != other.group0)
            return this.group0.lessThan(other.group0);
        return this.group1.lessThan(other.group1);
    }
    bool equals(IComparable another) {
        Flector other = (Flector)another;
        return this.group0.equals(other.group0) && this.group1.equals(other.group1);
    }
}
__include "impls/flector";

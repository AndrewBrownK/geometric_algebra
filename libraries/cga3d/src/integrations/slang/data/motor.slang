implementing cga3d;
using data::*;

/// Motor
public struct Motor {
    // e415, e425, e435, e12345
    internal group0: float4,
    // e235, e315, e125, e5
    internal group1: float4
}
extension Motor {
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
    public property e12345: float {
        get { return group0[3]; }
        set { group0[3] = newValue; }
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
    public property e5: float {
        get { return group1[3]; }
        set { group1[3] = newValue; }
    }
    public static func from_elements(
        e415: float, e425: float, e435: float, e12345: float, e235: float, e315: float, e125: float, e5: float
    ) -> Motor {
        return Motor { float4(e415, e425, e435, e12345), float4(e235, e315, e125, e5) };
    }
    internal static func from_groups(
        g0: float4, g1: float4
    ) -> Motor {
        return Motor {
            group0: g0, group1: g1
        };
    }
}
extension Motor: IComparable {
    bool lessThan(IComparable another) {
        Motor other = (Motor)another;
        if (this.group0 != other.group0)
            return this.group0.lessThan(other.group0);
        return this.group1.lessThan(other.group1);
    }
    bool equals(IComparable another) {
        Motor other = (Motor)another;
        return this.group0.equals(other.group0) && this.group1.equals(other.group1);
    }
}
__include "impls/motor";

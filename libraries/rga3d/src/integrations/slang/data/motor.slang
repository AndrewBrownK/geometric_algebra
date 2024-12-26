implementing rga3d;
using data::*;

/// Motor
public struct Motor {
    // e41, e42, e43, e1234
    internal group0: float4,
    // e23, e31, e12, scalar
    internal group1: float4
}
extension Motor {
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
    public property e1234: float {
        get { return group0[3]; }
        set { group0[3] = newValue; }
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
    public property scalar: float {
        get { return group1[3]; }
        set { group1[3] = newValue; }
    }
    public static func from_elements(
        e41: float, e42: float, e43: float, e1234: float, e23: float, e31: float, e12: float, scalar: float
    ) -> Motor {
        return Motor { float4(e41, e42, e43, e1234), float4(e23, e31, e12, scalar) };
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

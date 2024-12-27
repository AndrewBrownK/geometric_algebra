implementing cga3d;
using data::*;

/// VersorOdd
public struct VersorOdd {
    // e41, e42, e43, scalar
    internal group0: float4,
    // e23, e31, e12, e45
    internal group1: float4,
    // e15, e25, e35, e1234
    internal group2: float4,
    // e4235, e4315, e4125, e3215
    internal group3: float4
}
extension VersorOdd {
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
    public property scalar: float {
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
    public property e45: float {
        get { return group1[3]; }
        set { group1[3] = newValue; }
    }
    public property e15: float {
        get { return group2[0]; }
        set { group2[0] = newValue; }
    }
    public property e25: float {
        get { return group2[1]; }
        set { group2[1] = newValue; }
    }
    public property e35: float {
        get { return group2[2]; }
        set { group2[2] = newValue; }
    }
    public property e1234: float {
        get { return group2[3]; }
        set { group2[3] = newValue; }
    }
    public property e4235: float {
        get { return group3[0]; }
        set { group3[0] = newValue; }
    }
    public property e4315: float {
        get { return group3[1]; }
        set { group3[1] = newValue; }
    }
    public property e4125: float {
        get { return group3[2]; }
        set { group3[2] = newValue; }
    }
    public property e3215: float {
        get { return group3[3]; }
        set { group3[3] = newValue; }
    }
    public static func from_elements(
        e41: float, e42: float, e43: float, scalar: float, e23: float, e31: float, e12: float, e45: float, e15: float, e25: float, e35: float, e1234: float, e4235: float, e4315: float, e4125: float, e3215: float
    ) -> VersorOdd {
        return VersorOdd { float4(e41, e42, e43, scalar), float4(e23, e31, e12, e45), float4(e15, e25, e35, e1234), float4(e4235, e4315, e4125, e3215) };
    }
    internal static func from_groups(
        g0: float4, g1: float4, g2: float4, g3: float4
    ) -> VersorOdd {
        return VersorOdd {
            group0: g0, group1: g1, group2: g2, group3: g3
        };
    }
}
extension VersorOdd: IComparable {
    bool lessThan(IComparable another) {
        VersorOdd other = (VersorOdd)another;
        if (this.group0 != other.group0)
            return this.group0.lessThan(other.group0);
        if (this.group1 != other.group1)
            return this.group1.lessThan(other.group1);
        if (this.group2 != other.group2)
            return this.group2.lessThan(other.group2);
        return this.group3.lessThan(other.group3);
    }
    bool equals(IComparable another) {
        VersorOdd other = (VersorOdd)another;
        return this.group0.equals(other.group0) && this.group1.equals(other.group1) && this.group2.equals(other.group2) && this.group3.equals(other.group3);
    }
}
__include "impls/versor_odd";

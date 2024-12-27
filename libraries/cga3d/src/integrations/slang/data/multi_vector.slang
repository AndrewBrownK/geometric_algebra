implementing cga3d;
using data::*;

/// MultiVector
public struct MultiVector {
    // scalar, e12345, 0, 0
    internal group0: float4,
    // e1, e2, e3, e4
    internal group1: float4,
    // e5, 0, 0, 0
    internal group2: float4,
    // e15, e25, e35, e45
    internal group3: float4,
    // e41, e42, e43, 0
    internal group4: float4,
    // e23, e31, e12, 0
    internal group5: float4,
    // e415, e425, e435, e321
    internal group6: float4,
    // e423, e431, e412, 0
    internal group7: float4,
    // e235, e315, e125, 0
    internal group8: float4,
    // e4235, e4315, e4125, e3215
    internal group9: float4,
    // e1234, 0, 0, 0
    internal group10: float4
}
extension MultiVector {
    public property scalar: float {
        get { return group0[0]; }
        set { group0[0] = newValue; }
    }
    public property e12345: float {
        get { return group0[1]; }
        set { group0[1] = newValue; }
    }
    public property e1: float {
        get { return group1[0]; }
        set { group1[0] = newValue; }
    }
    public property e2: float {
        get { return group1[1]; }
        set { group1[1] = newValue; }
    }
    public property e3: float {
        get { return group1[2]; }
        set { group1[2] = newValue; }
    }
    public property e4: float {
        get { return group1[3]; }
        set { group1[3] = newValue; }
    }
    public property e5: float {
        get { return group2[0]; }
        set { group2[0] = newValue; }
    }
    public property e15: float {
        get { return group3[0]; }
        set { group3[0] = newValue; }
    }
    public property e25: float {
        get { return group3[1]; }
        set { group3[1] = newValue; }
    }
    public property e35: float {
        get { return group3[2]; }
        set { group3[2] = newValue; }
    }
    public property e45: float {
        get { return group3[3]; }
        set { group3[3] = newValue; }
    }
    public property e41: float {
        get { return group4[0]; }
        set { group4[0] = newValue; }
    }
    public property e42: float {
        get { return group4[1]; }
        set { group4[1] = newValue; }
    }
    public property e43: float {
        get { return group4[2]; }
        set { group4[2] = newValue; }
    }
    public property e23: float {
        get { return group5[0]; }
        set { group5[0] = newValue; }
    }
    public property e31: float {
        get { return group5[1]; }
        set { group5[1] = newValue; }
    }
    public property e12: float {
        get { return group5[2]; }
        set { group5[2] = newValue; }
    }
    public property e415: float {
        get { return group6[0]; }
        set { group6[0] = newValue; }
    }
    public property e425: float {
        get { return group6[1]; }
        set { group6[1] = newValue; }
    }
    public property e435: float {
        get { return group6[2]; }
        set { group6[2] = newValue; }
    }
    public property e321: float {
        get { return group6[3]; }
        set { group6[3] = newValue; }
    }
    public property e423: float {
        get { return group7[0]; }
        set { group7[0] = newValue; }
    }
    public property e431: float {
        get { return group7[1]; }
        set { group7[1] = newValue; }
    }
    public property e412: float {
        get { return group7[2]; }
        set { group7[2] = newValue; }
    }
    public property e235: float {
        get { return group8[0]; }
        set { group8[0] = newValue; }
    }
    public property e315: float {
        get { return group8[1]; }
        set { group8[1] = newValue; }
    }
    public property e125: float {
        get { return group8[2]; }
        set { group8[2] = newValue; }
    }
    public property e4235: float {
        get { return group9[0]; }
        set { group9[0] = newValue; }
    }
    public property e4315: float {
        get { return group9[1]; }
        set { group9[1] = newValue; }
    }
    public property e4125: float {
        get { return group9[2]; }
        set { group9[2] = newValue; }
    }
    public property e3215: float {
        get { return group9[3]; }
        set { group9[3] = newValue; }
    }
    public property e1234: float {
        get { return group10[0]; }
        set { group10[0] = newValue; }
    }
    public static func from_elements(
        scalar: float, e12345: float, e1: float, e2: float, e3: float, e4: float, e5: float, e15: float, e25: float, e35: float, e45: float, e41: float, e42: float, e43: float, e23: float, e31: float, e12: float, e415: float, e425: float, e435: float, e321: float, e423: float, e431: float, e412: float, e235: float, e315: float, e125: float, e4235: float, e4315: float, e4125: float, e3215: float, e1234: float
    ) -> MultiVector {
        return MultiVector { float4(scalar, e12345, 0.0, 0.0), float4(e1, e2, e3, e4), float4(e5, 0.0, 0.0, 0.0), float4(e15, e25, e35, e45), float4(e41, e42, e43, 0.0), float4(e23, e31, e12, 0.0), float4(e415, e425, e435, e321), float4(e423, e431, e412, 0.0), float4(e235, e315, e125, 0.0), float4(e4235, e4315, e4125, e3215), float4(e1234, 0.0, 0.0, 0.0) };
    }
    internal static func from_groups(
        g0: float2, g1: float4, g2: float, g3: float4, g4: float3, g5: float3, g6: float4, g7: float3, g8: float3, g9: float4, g10: float
    ) -> MultiVector {
        return MultiVector {
            group0: g0, group1: g1, group2: g2, group3: g3, group4: g4, group5: g5, group6: g6, group7: g7, group8: g8, group9: g9, group10: g10
        };
    }
}
extension MultiVector: IComparable {
    bool lessThan(IComparable another) {
        MultiVector other = (MultiVector)another;
        if (this.group0 != other.group0)
            return this.group0.lessThan(other.group0);
        if (this.group1 != other.group1)
            return this.group1.lessThan(other.group1);
        if (this.group2 != other.group2)
            return this.group2.lessThan(other.group2);
        if (this.group3 != other.group3)
            return this.group3.lessThan(other.group3);
        if (this.group4 != other.group4)
            return this.group4.lessThan(other.group4);
        if (this.group5 != other.group5)
            return this.group5.lessThan(other.group5);
        if (this.group6 != other.group6)
            return this.group6.lessThan(other.group6);
        if (this.group7 != other.group7)
            return this.group7.lessThan(other.group7);
        if (this.group8 != other.group8)
            return this.group8.lessThan(other.group8);
        if (this.group9 != other.group9)
            return this.group9.lessThan(other.group9);
        return this.group10.lessThan(other.group10);
    }
    bool equals(IComparable another) {
        MultiVector other = (MultiVector)another;
        return this.group0.equals(other.group0) && this.group1.equals(other.group1) && this.group2.equals(other.group2) && this.group3.equals(other.group3) && this.group4.equals(other.group4) && this.group5.equals(other.group5) && this.group6.equals(other.group6) && this.group7.equals(other.group7) && this.group8.equals(other.group8) && this.group9.equals(other.group9) && this.group10.equals(other.group10);
    }
}
__include "impls/multi_vector";

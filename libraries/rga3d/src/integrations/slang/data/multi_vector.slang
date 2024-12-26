implementing rga3d;
using data::*;

/// MultiVector
public struct MultiVector {
    // scalar, e1234, 0, 0
    internal group0: float4,
    // e1, e2, e3, e4
    internal group1: float4,
    // e41, e42, e43, 0
    internal group2: float4,
    // e23, e31, e12, 0
    internal group3: float4,
    // e423, e431, e412, e321
    internal group4: float4
}
extension MultiVector {
    public property scalar: float {
        get { return group0[0]; }
        set { group0[0] = newValue; }
    }
    public property e1234: float {
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
    public property e41: float {
        get { return group2[0]; }
        set { group2[0] = newValue; }
    }
    public property e42: float {
        get { return group2[1]; }
        set { group2[1] = newValue; }
    }
    public property e43: float {
        get { return group2[2]; }
        set { group2[2] = newValue; }
    }
    public property e23: float {
        get { return group3[0]; }
        set { group3[0] = newValue; }
    }
    public property e31: float {
        get { return group3[1]; }
        set { group3[1] = newValue; }
    }
    public property e12: float {
        get { return group3[2]; }
        set { group3[2] = newValue; }
    }
    public property e423: float {
        get { return group4[0]; }
        set { group4[0] = newValue; }
    }
    public property e431: float {
        get { return group4[1]; }
        set { group4[1] = newValue; }
    }
    public property e412: float {
        get { return group4[2]; }
        set { group4[2] = newValue; }
    }
    public property e321: float {
        get { return group4[3]; }
        set { group4[3] = newValue; }
    }
    public static func from_elements(
        scalar: float, e1234: float, e1: float, e2: float, e3: float, e4: float, e41: float, e42: float, e43: float, e23: float, e31: float, e12: float, e423: float, e431: float, e412: float, e321: float
    ) -> MultiVector {
        return MultiVector { float4(scalar, e1234, 0.0, 0.0), float4(e1, e2, e3, e4), float4(e41, e42, e43, 0.0), float4(e23, e31, e12, 0.0), float4(e423, e431, e412, e321) };
    }
    internal static func from_groups(
        g0: float2, g1: float4, g2: float3, g3: float3, g4: float4
    ) -> MultiVector {
        return MultiVector {
            group0: g0, group1: g1, group2: g2, group3: g3, group4: g4
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
        return this.group4.lessThan(other.group4);
    }
    bool equals(IComparable another) {
        MultiVector other = (MultiVector)another;
        return this.group0.equals(other.group0) && this.group1.equals(other.group1) && this.group2.equals(other.group2) && this.group3.equals(other.group3) && this.group4.equals(other.group4);
    }
}
__include "impls/multi_vector";

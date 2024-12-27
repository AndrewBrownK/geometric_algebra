implementing cga3d;
using data::*;

/// CircleRotor
public struct CircleRotor {
    // e423, e431, e412, 0
    internal group0: float4,
    // e415, e425, e435, e321
    internal group1: float4,
    // e235, e315, e125, e12345
    internal group2: float4
}
extension CircleRotor {
    public property e423: float {
        get { return group0[0]; }
        set { group0[0] = newValue; }
    }
    public property e431: float {
        get { return group0[1]; }
        set { group0[1] = newValue; }
    }
    public property e412: float {
        get { return group0[2]; }
        set { group0[2] = newValue; }
    }
    public property e415: float {
        get { return group1[0]; }
        set { group1[0] = newValue; }
    }
    public property e425: float {
        get { return group1[1]; }
        set { group1[1] = newValue; }
    }
    public property e435: float {
        get { return group1[2]; }
        set { group1[2] = newValue; }
    }
    public property e321: float {
        get { return group1[3]; }
        set { group1[3] = newValue; }
    }
    public property e235: float {
        get { return group2[0]; }
        set { group2[0] = newValue; }
    }
    public property e315: float {
        get { return group2[1]; }
        set { group2[1] = newValue; }
    }
    public property e125: float {
        get { return group2[2]; }
        set { group2[2] = newValue; }
    }
    public property e12345: float {
        get { return group2[3]; }
        set { group2[3] = newValue; }
    }
    public static func from_elements(
        e423: float, e431: float, e412: float, e415: float, e425: float, e435: float, e321: float, e235: float, e315: float, e125: float, e12345: float
    ) -> CircleRotor {
        return CircleRotor { float4(e423, e431, e412, 0.0), float4(e415, e425, e435, e321), float4(e235, e315, e125, e12345) };
    }
    internal static func from_groups(
        g0: float3, g1: float4, g2: float4
    ) -> CircleRotor {
        return CircleRotor {
            group0: g0, group1: g1, group2: g2
        };
    }
}
extension CircleRotor: IComparable {
    bool lessThan(IComparable another) {
        CircleRotor other = (CircleRotor)another;
        if (this.group0 != other.group0)
            return this.group0.lessThan(other.group0);
        if (this.group1 != other.group1)
            return this.group1.lessThan(other.group1);
        return this.group2.lessThan(other.group2);
    }
    bool equals(IComparable another) {
        CircleRotor other = (CircleRotor)another;
        return this.group0.equals(other.group0) && this.group1.equals(other.group1) && this.group2.equals(other.group2);
    }
}
__include "impls/circle_rotor";

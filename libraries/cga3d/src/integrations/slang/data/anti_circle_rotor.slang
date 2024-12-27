implementing cga3d;
using data::*;

/// AntiCircleRotor
public struct AntiCircleRotor {
    // e41, e42, e43, 0
    internal group0: float4,
    // e23, e31, e12, e45
    internal group1: float4,
    // e15, e25, e35, scalar
    internal group2: float4
}
extension AntiCircleRotor {
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
    public property scalar: float {
        get { return group2[3]; }
        set { group2[3] = newValue; }
    }
    public static func from_elements(
        e41: float, e42: float, e43: float, e23: float, e31: float, e12: float, e45: float, e15: float, e25: float, e35: float, scalar: float
    ) -> AntiCircleRotor {
        return AntiCircleRotor { float4(e41, e42, e43, 0.0), float4(e23, e31, e12, e45), float4(e15, e25, e35, scalar) };
    }
    internal static func from_groups(
        g0: float3, g1: float4, g2: float4
    ) -> AntiCircleRotor {
        return AntiCircleRotor {
            group0: g0, group1: g1, group2: g2
        };
    }
}
extension AntiCircleRotor: IComparable {
    bool lessThan(IComparable another) {
        AntiCircleRotor other = (AntiCircleRotor)another;
        if (this.group0 != other.group0)
            return this.group0.lessThan(other.group0);
        if (this.group1 != other.group1)
            return this.group1.lessThan(other.group1);
        return this.group2.lessThan(other.group2);
    }
    bool equals(IComparable another) {
        AntiCircleRotor other = (AntiCircleRotor)another;
        return this.group0.equals(other.group0) && this.group1.equals(other.group1) && this.group2.equals(other.group2);
    }
}
__include "impls/anti_circle_rotor";

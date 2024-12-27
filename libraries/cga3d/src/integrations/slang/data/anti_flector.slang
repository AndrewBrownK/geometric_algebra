implementing cga3d;
using data::*;

/// AntiFlector
public struct AntiFlector {
    // e235, e315, e125, e321
    internal group0: float4,
    // e1, e2, e3, e5
    internal group1: float4
}
extension AntiFlector {
    public property e235: float {
        get { return group0[0]; }
        set { group0[0] = newValue; }
    }
    public property e315: float {
        get { return group0[1]; }
        set { group0[1] = newValue; }
    }
    public property e125: float {
        get { return group0[2]; }
        set { group0[2] = newValue; }
    }
    public property e321: float {
        get { return group0[3]; }
        set { group0[3] = newValue; }
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
    public property e5: float {
        get { return group1[3]; }
        set { group1[3] = newValue; }
    }
    public static func from_elements(
        e235: float, e315: float, e125: float, e321: float, e1: float, e2: float, e3: float, e5: float
    ) -> AntiFlector {
        return AntiFlector { float4(e235, e315, e125, e321), float4(e1, e2, e3, e5) };
    }
    internal static func from_groups(
        g0: float4, g1: float4
    ) -> AntiFlector {
        return AntiFlector {
            group0: g0, group1: g1
        };
    }
}
extension AntiFlector: IComparable {
    bool lessThan(IComparable another) {
        AntiFlector other = (AntiFlector)another;
        if (this.group0 != other.group0)
            return this.group0.lessThan(other.group0);
        return this.group1.lessThan(other.group1);
    }
    bool equals(IComparable another) {
        AntiFlector other = (AntiFlector)another;
        return this.group0.equals(other.group0) && this.group1.equals(other.group1);
    }
}
__include "impls/anti_flector";

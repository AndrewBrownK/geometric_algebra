implementing cga3d;
using data::*;

/// Sphere
public struct Sphere {
    // e4235, e4315, e4125, e3215
    internal group0: float4,
    // e1234, 0, 0, 0
    internal group1: float4
}
extension Sphere {
    public property e4235: float {
        get { return group0[0]; }
        set { group0[0] = newValue; }
    }
    public property e4315: float {
        get { return group0[1]; }
        set { group0[1] = newValue; }
    }
    public property e4125: float {
        get { return group0[2]; }
        set { group0[2] = newValue; }
    }
    public property e3215: float {
        get { return group0[3]; }
        set { group0[3] = newValue; }
    }
    public property e1234: float {
        get { return group1[0]; }
        set { group1[0] = newValue; }
    }
    public static func from_elements(
        e4235: float, e4315: float, e4125: float, e3215: float, e1234: float
    ) -> Sphere {
        return Sphere { float4(e4235, e4315, e4125, e3215), float4(e1234, 0.0, 0.0, 0.0) };
    }
    internal static func from_groups(
        g0: float4, g1: float
    ) -> Sphere {
        return Sphere {
            group0: g0, group1: g1
        };
    }
}
extension Sphere: IComparable {
    bool lessThan(IComparable another) {
        Sphere other = (Sphere)another;
        if (this.group0 != other.group0)
            return this.group0.lessThan(other.group0);
        return this.group1.lessThan(other.group1);
    }
    bool equals(IComparable another) {
        Sphere other = (Sphere)another;
        return this.group0.equals(other.group0) && this.group1.equals(other.group1);
    }
}
__include "impls/sphere";

implementing cga3d;
using data::*;

/// Plane
public struct Plane {
    // e4235, e4315, e4125, e3215
    internal group0: float4
}
extension Plane {
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
    public static func from_elements(
        e4235: float, e4315: float, e4125: float, e3215: float
    ) -> Plane {
        return Plane { float4(e4235, e4315, e4125, e3215) };
    }
    internal static func from_groups(
        g0: float4
    ) -> Plane {
        return Plane {
            group0: g0
        };
    }
}
extension Plane: IComparable {
    bool lessThan(IComparable another) {
        Plane other = (Plane)another;
        return this.group0.lessThan(other.group0);
    }
    bool equals(IComparable another) {
        Plane other = (Plane)another;
        return this.group0.equals(other.group0);
    }
}
__include "impls/plane";

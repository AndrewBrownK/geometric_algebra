implementing rga3d;
using data::*;

/// Plane
public struct Plane {
    // e423, e431, e412, e321
    internal group0: float4
}
extension Plane {
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
    public property e321: float {
        get { return group0[3]; }
        set { group0[3] = newValue; }
    }
    public static func from_elements(
        e423: float, e431: float, e412: float, e321: float
    ) -> Plane {
        return Plane { float4(e423, e431, e412, e321) };
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

implementing cga3d;
using data::*;

/// AntiPlane
public struct AntiPlane {
    // e1, e2, e3, e5
    internal group0: float4
}
extension AntiPlane {
    public property e1: float {
        get { return group0[0]; }
        set { group0[0] = newValue; }
    }
    public property e2: float {
        get { return group0[1]; }
        set { group0[1] = newValue; }
    }
    public property e3: float {
        get { return group0[2]; }
        set { group0[2] = newValue; }
    }
    public property e5: float {
        get { return group0[3]; }
        set { group0[3] = newValue; }
    }
    public static func from_elements(
        e1: float, e2: float, e3: float, e5: float
    ) -> AntiPlane {
        return AntiPlane { float4(e1, e2, e3, e5) };
    }
    internal static func from_groups(
        g0: float4
    ) -> AntiPlane {
        return AntiPlane {
            group0: g0
        };
    }
}
extension AntiPlane: IComparable {
    bool lessThan(IComparable another) {
        AntiPlane other = (AntiPlane)another;
        return this.group0.lessThan(other.group0);
    }
    bool equals(IComparable another) {
        AntiPlane other = (AntiPlane)another;
        return this.group0.equals(other.group0);
    }
}
__include "impls/anti_plane";

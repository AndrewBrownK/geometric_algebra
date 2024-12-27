implementing cga3d;
using data::*;

/// AntiFlatPoint
public struct AntiFlatPoint {
    // e235, e315, e125, e321
    internal group0: float4
}
extension AntiFlatPoint {
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
    public static func from_elements(
        e235: float, e315: float, e125: float, e321: float
    ) -> AntiFlatPoint {
        return AntiFlatPoint { float4(e235, e315, e125, e321) };
    }
    internal static func from_groups(
        g0: float4
    ) -> AntiFlatPoint {
        return AntiFlatPoint {
            group0: g0
        };
    }
}
extension AntiFlatPoint: IComparable {
    bool lessThan(IComparable another) {
        AntiFlatPoint other = (AntiFlatPoint)another;
        return this.group0.lessThan(other.group0);
    }
    bool equals(IComparable another) {
        AntiFlatPoint other = (AntiFlatPoint)another;
        return this.group0.equals(other.group0);
    }
}
__include "impls/anti_flat_point";

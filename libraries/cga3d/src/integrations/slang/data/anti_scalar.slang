implementing cga3d;
using data::*;

/// AntiScalar
public struct AntiScalar {
    // e12345, 0, 0, 0
    internal group0: float4
}
extension AntiScalar {
    public property e12345: float {
        get { return group0[0]; }
        set { group0[0] = newValue; }
    }
    public static func from_elements(
        e12345: float
    ) -> AntiScalar {
        return AntiScalar { float4(e12345, 0.0, 0.0, 0.0) };
    }
    internal static func from_groups(
        g0: float
    ) -> AntiScalar {
        return AntiScalar {
            group0: g0
        };
    }
}
extension AntiScalar: IComparable {
    bool lessThan(IComparable another) {
        AntiScalar other = (AntiScalar)another;
        return this.group0.lessThan(other.group0);
    }
    bool equals(IComparable another) {
        AntiScalar other = (AntiScalar)another;
        return this.group0.equals(other.group0);
    }
}
__include "impls/anti_scalar";

implementing rga3d;
using data::*;

/// Horizon
public struct Horizon {
    // e321, 0, 0, 0
    internal group0: float4
}
extension Horizon {
    public property e321: float {
        get { return group0[0]; }
        set { group0[0] = newValue; }
    }
    public static func from_elements(
        e321: float
    ) -> Horizon {
        return Horizon { float4(e321, 0.0, 0.0, 0.0) };
    }
    internal static func from_groups(
        g0: float
    ) -> Horizon {
        return Horizon {
            group0: g0
        };
    }
}
extension Horizon: IComparable {
    bool lessThan(IComparable another) {
        Horizon other = (Horizon)another;
        return this.group0.lessThan(other.group0);
    }
    bool equals(IComparable another) {
        Horizon other = (Horizon)another;
        return this.group0.equals(other.group0);
    }
}
__include "impls/horizon";

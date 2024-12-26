implementing rga3d;
using data::*;

/// Origin.
/// The Origin is the RoundPoint where x, y, z, and radius are all zero.
/// It is the base element e4.
/// Not to be confused with FlatOrigin, which is a Dipole connecting Origin and Infinity.
public struct Origin {
    // e4, 0, 0, 0
    internal group0: float4
}
extension Origin {
    public property e4: float {
        get { return group0[0]; }
        set { group0[0] = newValue; }
    }
    public static func from_elements(
        e4: float
    ) -> Origin {
        return Origin { float4(e4, 0.0, 0.0, 0.0) };
    }
    internal static func from_groups(
        g0: float
    ) -> Origin {
        return Origin {
            group0: g0
        };
    }
}
extension Origin: IComparable {
    bool lessThan(IComparable another) {
        Origin other = (Origin)another;
        return this.group0.lessThan(other.group0);
    }
    bool equals(IComparable another) {
        Origin other = (Origin)another;
        return this.group0.equals(other.group0);
    }
}
__include "impls/origin";

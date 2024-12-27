implementing cga3d;
using data::*;

/// Scalar
public struct Scalar {
    // scalar, 0, 0, 0
    internal group0: float4
}
extension Scalar {
    public property scalar: float {
        get { return group0[0]; }
        set { group0[0] = newValue; }
    }
    public static func from_elements(
        scalar: float
    ) -> Scalar {
        return Scalar { float4(scalar, 0.0, 0.0, 0.0) };
    }
    internal static func from_groups(
        g0: float
    ) -> Scalar {
        return Scalar {
            group0: g0
        };
    }
}
extension Scalar: IComparable {
    bool lessThan(IComparable another) {
        Scalar other = (Scalar)another;
        return this.group0.lessThan(other.group0);
    }
    bool equals(IComparable another) {
        Scalar other = (Scalar)another;
        return this.group0.equals(other.group0);
    }
}
__include "impls/scalar";

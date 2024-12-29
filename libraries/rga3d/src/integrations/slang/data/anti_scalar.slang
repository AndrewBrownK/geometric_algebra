implementing rga3d;
using data;

/// AntiScalar
public struct AntiScalar {
    // e1234, 0, 0, 0
    internal float4 group0;
}
public extension AntiScalar {
    public property e1234: float {
        get { return group0[0]; }
        set { group0[0] = newValue; }
    }
    public static AntiScalar from_elements(
        float e1234
    ) {
        return AntiScalar(float4(e1234, 0.0, 0.0, 0.0));
    }
    internal static AntiScalar from_groups(
        float g0
    ) {
        return AntiScalar(float4(g0, 0.0, 0.0, 0.0));
    }
}
public extension AntiScalar: IComparable {
    public bool equals(AntiScalar other) {
        return equalsHelper(this.group0, other.group0);
    }
    public bool lessThan(AntiScalar other) {
        return lessThanHelper(this.group0, other.group0);
    }
    public bool lessThanOrEquals(AntiScalar other) {
        return lessThanOrEqualsHelper(this.group0, other.group0);
    }
}
__include "impls/anti_scalar";

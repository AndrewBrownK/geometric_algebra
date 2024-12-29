implementing rga3d;
using data;

/// Horizon
public struct Horizon {
    // e321, 0, 0, 0
    internal float4 group0;
}
public extension Horizon {
    public property e321: float {
        get { return group0[0]; }
        set { group0[0] = newValue; }
    }
    public static Horizon from_elements(
        float e321
    ) {
        return Horizon(float4(e321, 0.0, 0.0, 0.0));
    }
    internal static Horizon from_groups(
        float g0
    ) {
        return Horizon(float4(g0, 0.0, 0.0, 0.0));
    }
}
public extension Horizon: IComparable {
    public bool equals(Horizon other) {
        return equalsHelper(this.group0, other.group0);
    }
    public bool lessThan(Horizon other) {
        return lessThanHelper(this.group0, other.group0);
    }
    public bool lessThanOrEquals(Horizon other) {
        return lessThanOrEqualsHelper(this.group0, other.group0);
    }
}
__include "impls/horizon";

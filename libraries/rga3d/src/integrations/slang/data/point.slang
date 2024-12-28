implementing rga3d;
using data;

/// Point
public struct Point {
    // e1, e2, e3, e4
    internal float4 group0;
}
public extension Point {
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
    public property e4: float {
        get { return group0[3]; }
        set { group0[3] = newValue; }
    }
    public static func from_elements(
        e1: float, e2: float, e3: float, e4: float
    ) -> Point {
        return Point(float4(e1, e2, e3, e4));
    }
    internal static func from_groups(
        g0: float4
    ) -> Point {
        return Point(g0);
    }
}
public extension Point: IComparable {
    public bool equals(Point other) {
        return equalsHelper(this.group0, other.group0);
    }
    public bool lessThan(Point other) {
        return lessThanHelper(this.group0, other.group0);
    }
    public bool lessThanOrEquals(Point other) {
        return lessThanOrEqualsHelper(this.group0, other.group0);
    }
}
__include "impls/point";

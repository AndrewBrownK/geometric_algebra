implementing rga3d;
using data::*;

/// Flector
public struct Flector {
    // e1, e2, e3, e4
    internal group0: float4,
    // e423, e431, e412, e321
    internal group1: float4
}
extension Flector {
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
    public property e423: float {
        get { return group1[0]; }
        set { group1[0] = newValue; }
    }
    public property e431: float {
        get { return group1[1]; }
        set { group1[1] = newValue; }
    }
    public property e412: float {
        get { return group1[2]; }
        set { group1[2] = newValue; }
    }
    public property e321: float {
        get { return group1[3]; }
        set { group1[3] = newValue; }
    }
    public static func from_elements(
        e1: float, e2: float, e3: float, e4: float, e423: float, e431: float, e412: float, e321: float
    ) -> Flector {
        return Flector { float4(e1, e2, e3, e4), float4(e423, e431, e412, e321) };
    }
    internal static func from_groups(
        g0: float4, g1: float4
    ) -> Flector {
        return Flector {
            group0: g0, group1: g1
        };
    }
}
extension Flector: IComparable {
    bool lessThan(IComparable another) {
        Flector other = (Flector)another;
        if (this.group0 != other.group0)
            return this.group0.lessThan(other.group0);
        return this.group1.lessThan(other.group1);
    }
    bool equals(IComparable another) {
        Flector other = (Flector)another;
        return this.group0.equals(other.group0) && this.group1.equals(other.group1);
    }
}
__include "impls/flector";

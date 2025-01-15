implementing rga3d;
using traits;
// Fancy postfix trick
public AntiScalar operator /(AntiScalar lhs, FlatWeightPrefixOrPostfix rhs)  {
    return lhs.flat_weight();
    }
// Fancy prefix trick
public AntiScalar operator /(FlatWeightPrefixOrPostfix lhs, AntiScalar rhs) {
    return rhs.flat_weight();
}
public extension AntiScalar {
    public AntiScalar flat_weight() {
        return this;
    }
}
// Fancy postfix trick
public AntiScalar operator /(DualNum lhs, FlatWeightPrefixOrPostfix rhs)  {
    return lhs.flat_weight();
    }
// Fancy prefix trick
public AntiScalar operator /(FlatWeightPrefixOrPostfix lhs, DualNum rhs) {
    return rhs.flat_weight();
}
public extension DualNum {
    public AntiScalar flat_weight() {
        return AntiScalar(
            /* e1234 */
            this.e1234
        );
    }
}
// Fancy postfix trick
public Flector operator /(Flector lhs, FlatWeightPrefixOrPostfix rhs)  {
    return lhs.flat_weight();
    }
// Fancy prefix trick
public Flector operator /(FlatWeightPrefixOrPostfix lhs, Flector rhs) {
    return rhs.flat_weight();
}
public extension Flector {
    public Flector flat_weight() {
        return Flector(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.e4), 
            /* e423, e431, e412, e321 */
            float4(this.group1.xyz, 0.0)
        );
    }
}
// Fancy postfix trick
public Line operator /(Line lhs, FlatWeightPrefixOrPostfix rhs)  {
    return lhs.flat_weight();
    }
// Fancy prefix trick
public Line operator /(FlatWeightPrefixOrPostfix lhs, Line rhs) {
    return rhs.flat_weight();
}
public extension Line {
    public Line flat_weight() {
        return Line(
            /* e41, e42, e43 */
            this.group0.xyz, 
            /* e23, e31, e12 */
            float3(0.0)
        );
    }
}
// Fancy postfix trick
public Motor operator /(Motor lhs, FlatWeightPrefixOrPostfix rhs)  {
    return lhs.flat_weight();
    }
// Fancy prefix trick
public Motor operator /(FlatWeightPrefixOrPostfix lhs, Motor rhs) {
    return rhs.flat_weight();
}
public extension Motor {
    public Motor flat_weight() {
        return Motor(
            /* e41, e42, e43, e1234 */
            this.group0, 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
    }
}
// Fancy postfix trick
public MultiVector operator /(MultiVector lhs, FlatWeightPrefixOrPostfix rhs)  {
    return lhs.flat_weight();
    }
// Fancy prefix trick
public MultiVector operator /(FlatWeightPrefixOrPostfix lhs, MultiVector rhs) {
    return rhs.flat_weight();
}
public extension MultiVector {
    public MultiVector flat_weight() {
        return MultiVector(
            /* scalar, e1234 */
            float2(0.0, this.e1234), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.e4), 
            /* e41, e42, e43 */
            this.group2.xyz, 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(this.group4.xyz, 0.0)
        );
    }
}
// Fancy postfix trick
public Origin operator /(Origin lhs, FlatWeightPrefixOrPostfix rhs)  {
    return lhs.flat_weight();
    }
// Fancy prefix trick
public Origin operator /(FlatWeightPrefixOrPostfix lhs, Origin rhs) {
    return rhs.flat_weight();
}
public extension Origin {
    public Origin flat_weight() {
        return this;
    }
}
// Fancy postfix trick
public Plane operator /(Plane lhs, FlatWeightPrefixOrPostfix rhs)  {
    return lhs.flat_weight();
    }
// Fancy prefix trick
public Plane operator /(FlatWeightPrefixOrPostfix lhs, Plane rhs) {
    return rhs.flat_weight();
}
public extension Plane {
    public Plane flat_weight() {
        return Plane(
            /* e423, e431, e412, e321 */
            float4(this.group0.xyz, 0.0)
        );
    }
}
// Fancy postfix trick
public Origin operator /(Point lhs, FlatWeightPrefixOrPostfix rhs)  {
    return lhs.flat_weight();
    }
// Fancy prefix trick
public Origin operator /(FlatWeightPrefixOrPostfix lhs, Point rhs) {
    return rhs.flat_weight();
}
public extension Point {
    public Origin flat_weight() {
        return Origin(
            /* e4 */
            this.e4
        );
    }
}

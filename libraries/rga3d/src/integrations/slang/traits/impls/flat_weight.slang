implementing rga3d;
using traits;
public extension AntiScalar {
    // Fancy postfix trick
    public AntiScalar operator /(FlatWeightPrefixOrPostfix rhs)  {
        return this.flat_weight();
    }
}
public extension FlatWeightPrefixOrPostfix {
    // Fancy prefix trick
    public AntiScalar operator /(AntiScalar rhs) {
        return rhs.flat_weight();
    }
}
public extension AntiScalar {
    public AntiScalar flat_weight() {
        return this;
    }
}
public extension DualNum {
    // Fancy postfix trick
    public AntiScalar operator /(FlatWeightPrefixOrPostfix rhs)  {
        return this.flat_weight();
    }
}
public extension FlatWeightPrefixOrPostfix {
    // Fancy prefix trick
    public AntiScalar operator /(DualNum rhs) {
        return rhs.flat_weight();
    }
}
public extension DualNum {
    public AntiScalar flat_weight() {
        return AntiScalar.from_groups(
            /* e1234 */
            this.e1234
        );
    }
}
public extension Flector {
    // Fancy postfix trick
    public Flector operator /(FlatWeightPrefixOrPostfix rhs)  {
        return this.flat_weight();
    }
}
public extension FlatWeightPrefixOrPostfix {
    // Fancy prefix trick
    public Flector operator /(Flector rhs) {
        return rhs.flat_weight();
    }
}
public extension Flector {
    public Flector flat_weight() {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.e4), 
            /* e423, e431, e412, e321 */
            float4(this.group1.xyz, 0.0)
        );
    }
}
public extension Line {
    // Fancy postfix trick
    public Line operator /(FlatWeightPrefixOrPostfix rhs)  {
        return this.flat_weight();
    }
}
public extension FlatWeightPrefixOrPostfix {
    // Fancy prefix trick
    public Line operator /(Line rhs) {
        return rhs.flat_weight();
    }
}
public extension Line {
    public Line flat_weight() {
        return Line.from_groups(
            /* e41, e42, e43 */
            this.group0.xyz, 
            /* e23, e31, e12 */
            float3(0.0)
        );
    }
}
public extension Motor {
    // Fancy postfix trick
    public Motor operator /(FlatWeightPrefixOrPostfix rhs)  {
        return this.flat_weight();
    }
}
public extension FlatWeightPrefixOrPostfix {
    // Fancy prefix trick
    public Motor operator /(Motor rhs) {
        return rhs.flat_weight();
    }
}
public extension Motor {
    public Motor flat_weight() {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            this.group0, 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
    }
}
public extension MultiVector {
    // Fancy postfix trick
    public MultiVector operator /(FlatWeightPrefixOrPostfix rhs)  {
        return this.flat_weight();
    }
}
public extension FlatWeightPrefixOrPostfix {
    // Fancy prefix trick
    public MultiVector operator /(MultiVector rhs) {
        return rhs.flat_weight();
    }
}
public extension MultiVector {
    public MultiVector flat_weight() {
        return MultiVector.from_groups(
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
public extension Origin {
    // Fancy postfix trick
    public Origin operator /(FlatWeightPrefixOrPostfix rhs)  {
        return this.flat_weight();
    }
}
public extension FlatWeightPrefixOrPostfix {
    // Fancy prefix trick
    public Origin operator /(Origin rhs) {
        return rhs.flat_weight();
    }
}
public extension Origin {
    public Origin flat_weight() {
        return this;
    }
}
public extension Plane {
    // Fancy postfix trick
    public Plane operator /(FlatWeightPrefixOrPostfix rhs)  {
        return this.flat_weight();
    }
}
public extension FlatWeightPrefixOrPostfix {
    // Fancy prefix trick
    public Plane operator /(Plane rhs) {
        return rhs.flat_weight();
    }
}
public extension Plane {
    public Plane flat_weight() {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(this.group0.xyz, 0.0)
        );
    }
}
public extension Point {
    // Fancy postfix trick
    public Origin operator /(FlatWeightPrefixOrPostfix rhs)  {
        return this.flat_weight();
    }
}
public extension FlatWeightPrefixOrPostfix {
    // Fancy prefix trick
    public Origin operator /(Point rhs) {
        return rhs.flat_weight();
    }
}
public extension Point {
    public Origin flat_weight() {
        return Origin.from_groups(
            /* e4 */
            this.e4
        );
    }
}

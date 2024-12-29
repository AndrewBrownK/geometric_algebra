implementing rga3d;
using traits;
public extension DualNum {
    // Fancy postfix trick
    public Scalar operator /(FlatBulkPrefixOrPostfix rhs)  {
        return this.flat_bulk();
    }
}
public extension FlatBulkPrefixOrPostfix {
    // Fancy prefix trick
    public Scalar operator /(DualNum rhs) {
        return rhs.flat_bulk();
    }
}
public extension DualNum {
    public Scalar flat_bulk() {
        return Scalar.from_groups(
            /* scalar */
            this.scalar
        );
    }
}
public extension Flector {
    // Fancy postfix trick
    public Flector operator /(FlatBulkPrefixOrPostfix rhs)  {
        return this.flat_bulk();
    }
}
public extension FlatBulkPrefixOrPostfix {
    // Fancy prefix trick
    public Flector operator /(Flector rhs) {
        return rhs.flat_bulk();
    }
}
public extension Flector {
    public Flector flat_bulk() {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.group0.xyz, 0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), this.e321)
        );
    }
}
public extension Horizon {
    // Fancy postfix trick
    public Horizon operator /(FlatBulkPrefixOrPostfix rhs)  {
        return this.flat_bulk();
    }
}
public extension FlatBulkPrefixOrPostfix {
    // Fancy prefix trick
    public Horizon operator /(Horizon rhs) {
        return rhs.flat_bulk();
    }
}
public extension Horizon {
    public Horizon flat_bulk() {
        return this;
    }
}
public extension Line {
    // Fancy postfix trick
    public Line operator /(FlatBulkPrefixOrPostfix rhs)  {
        return this.flat_bulk();
    }
}
public extension FlatBulkPrefixOrPostfix {
    // Fancy prefix trick
    public Line operator /(Line rhs) {
        return rhs.flat_bulk();
    }
}
public extension Line {
    public Line flat_bulk() {
        return Line.from_groups(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            this.group1.xyz
        );
    }
}
public extension Motor {
    // Fancy postfix trick
    public Motor operator /(FlatBulkPrefixOrPostfix rhs)  {
        return this.flat_bulk();
    }
}
public extension FlatBulkPrefixOrPostfix {
    // Fancy prefix trick
    public Motor operator /(Motor rhs) {
        return rhs.flat_bulk();
    }
}
public extension Motor {
    public Motor flat_bulk() {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(0.0), 
            /* e23, e31, e12, scalar */
            this.group1
        );
    }
}
public extension MultiVector {
    // Fancy postfix trick
    public MultiVector operator /(FlatBulkPrefixOrPostfix rhs)  {
        return this.flat_bulk();
    }
}
public extension FlatBulkPrefixOrPostfix {
    // Fancy prefix trick
    public MultiVector operator /(MultiVector rhs) {
        return rhs.flat_bulk();
    }
}
public extension MultiVector {
    public MultiVector flat_bulk() {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(this.scalar, 0.0), 
            /* e1, e2, e3, e4 */
            float4(this.group1.xyz, 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            this.group3.xyz, 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), this.e321)
        );
    }
}
public extension Plane {
    // Fancy postfix trick
    public Horizon operator /(FlatBulkPrefixOrPostfix rhs)  {
        return this.flat_bulk();
    }
}
public extension FlatBulkPrefixOrPostfix {
    // Fancy prefix trick
    public Horizon operator /(Plane rhs) {
        return rhs.flat_bulk();
    }
}
public extension Plane {
    public Horizon flat_bulk() {
        return Horizon.from_groups(
            /* e321 */
            this.e321
        );
    }
}
public extension Point {
    // Fancy postfix trick
    public Point operator /(FlatBulkPrefixOrPostfix rhs)  {
        return this.flat_bulk();
    }
}
public extension FlatBulkPrefixOrPostfix {
    // Fancy prefix trick
    public Point operator /(Point rhs) {
        return rhs.flat_bulk();
    }
}
public extension Point {
    public Point flat_bulk() {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.group0.xyz, 0.0)
        );
    }
}
public extension Scalar {
    // Fancy postfix trick
    public Scalar operator /(FlatBulkPrefixOrPostfix rhs)  {
        return this.flat_bulk();
    }
}
public extension FlatBulkPrefixOrPostfix {
    // Fancy prefix trick
    public Scalar operator /(Scalar rhs) {
        return rhs.flat_bulk();
    }
}
public extension Scalar {
    public Scalar flat_bulk() {
        return this;
    }
}

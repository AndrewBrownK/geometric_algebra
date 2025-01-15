implementing rga3d;
using traits;
// Fancy postfix trick
public Scalar operator /(DualNum lhs, FlatBulkPrefixOrPostfix rhs)  {
    return lhs.flat_bulk();
    }
// Fancy prefix trick
public Scalar operator /(FlatBulkPrefixOrPostfix lhs, DualNum rhs) {
    return rhs.flat_bulk();
}
public extension DualNum {
    public Scalar flat_bulk() {
        return Scalar(
            /* scalar */
            this.scalar
        );
    }
}
// Fancy postfix trick
public Flector operator /(Flector lhs, FlatBulkPrefixOrPostfix rhs)  {
    return lhs.flat_bulk();
    }
// Fancy prefix trick
public Flector operator /(FlatBulkPrefixOrPostfix lhs, Flector rhs) {
    return rhs.flat_bulk();
}
public extension Flector {
    public Flector flat_bulk() {
        return Flector(
            /* e1, e2, e3, e4 */
            float4(this.group0.xyz, 0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), this.e321)
        );
    }
}
// Fancy postfix trick
public Horizon operator /(Horizon lhs, FlatBulkPrefixOrPostfix rhs)  {
    return lhs.flat_bulk();
    }
// Fancy prefix trick
public Horizon operator /(FlatBulkPrefixOrPostfix lhs, Horizon rhs) {
    return rhs.flat_bulk();
}
public extension Horizon {
    public Horizon flat_bulk() {
        return this;
    }
}
// Fancy postfix trick
public Line operator /(Line lhs, FlatBulkPrefixOrPostfix rhs)  {
    return lhs.flat_bulk();
    }
// Fancy prefix trick
public Line operator /(FlatBulkPrefixOrPostfix lhs, Line rhs) {
    return rhs.flat_bulk();
}
public extension Line {
    public Line flat_bulk() {
        return Line(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            this.group1.xyz
        );
    }
}
// Fancy postfix trick
public Motor operator /(Motor lhs, FlatBulkPrefixOrPostfix rhs)  {
    return lhs.flat_bulk();
    }
// Fancy prefix trick
public Motor operator /(FlatBulkPrefixOrPostfix lhs, Motor rhs) {
    return rhs.flat_bulk();
}
public extension Motor {
    public Motor flat_bulk() {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(0.0), 
            /* e23, e31, e12, scalar */
            this.group1
        );
    }
}
// Fancy postfix trick
public MultiVector operator /(MultiVector lhs, FlatBulkPrefixOrPostfix rhs)  {
    return lhs.flat_bulk();
    }
// Fancy prefix trick
public MultiVector operator /(FlatBulkPrefixOrPostfix lhs, MultiVector rhs) {
    return rhs.flat_bulk();
}
public extension MultiVector {
    public MultiVector flat_bulk() {
        return MultiVector(
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
// Fancy postfix trick
public Horizon operator /(Plane lhs, FlatBulkPrefixOrPostfix rhs)  {
    return lhs.flat_bulk();
    }
// Fancy prefix trick
public Horizon operator /(FlatBulkPrefixOrPostfix lhs, Plane rhs) {
    return rhs.flat_bulk();
}
public extension Plane {
    public Horizon flat_bulk() {
        return Horizon(
            /* e321 */
            this.e321
        );
    }
}
// Fancy postfix trick
public Point operator /(Point lhs, FlatBulkPrefixOrPostfix rhs)  {
    return lhs.flat_bulk();
    }
// Fancy prefix trick
public Point operator /(FlatBulkPrefixOrPostfix lhs, Point rhs) {
    return rhs.flat_bulk();
}
public extension Point {
    public Point flat_bulk() {
        return Point(
            /* e1, e2, e3, e4 */
            float4(this.group0.xyz, 0.0)
        );
    }
}
// Fancy postfix trick
public Scalar operator /(Scalar lhs, FlatBulkPrefixOrPostfix rhs)  {
    return lhs.flat_bulk();
    }
// Fancy prefix trick
public Scalar operator /(FlatBulkPrefixOrPostfix lhs, Scalar rhs) {
    return rhs.flat_bulk();
}
public extension Scalar {
    public Scalar flat_bulk() {
        return this;
    }
}

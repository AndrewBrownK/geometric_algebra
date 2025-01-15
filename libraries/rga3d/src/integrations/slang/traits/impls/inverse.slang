implementing rga3d;
using traits;
// Fancy postfix trick
public DualNum operator /(DualNum lhs, InversePrefixOrPostfix rhs)  {
    return lhs.inverse();
    }
// Fancy prefix trick
public DualNum operator /(InversePrefixOrPostfix lhs, DualNum rhs) {
    return rhs.inverse();
}
public extension DualNum {
    public DualNum inverse() {
        return DualNum(
            /* scalar, e1234 */
            float2(pow(this.scalar, -2)) * this.group0.xy
        );
    }
}
// Fancy postfix trick
public Flector operator /(Flector lhs, InversePrefixOrPostfix rhs)  {
    return lhs.inverse();
    }
// Fancy prefix trick
public Flector operator /(InversePrefixOrPostfix lhs, Flector rhs) {
    return rhs.inverse();
}
public extension Flector {
    public Flector inverse() {
        Flector reverse = Flector(
            /* e1, e2, e3, e4 */
            this.group0, 
            /* e423, e431, e412, e321 */
            this.group1 * float4(-1.0)
        );
        Scalar other = Scalar(
            /* scalar */
            pow(this.e1, 2) + pow(this.e2, 2) + pow(this.e3, 2) + pow(this.e321, 2)
        );
        return Flector(
            /* e1, e2, e3, e4 */
            float4(other.scalar) * reverse.group0, 
            /* e423, e431, e412, e321 */
            float4(other.scalar) * reverse.group1
        );
    }
}
// Fancy postfix trick
public Horizon operator /(Horizon lhs, InversePrefixOrPostfix rhs)  {
    return lhs.inverse();
    }
// Fancy prefix trick
public Horizon operator /(InversePrefixOrPostfix lhs, Horizon rhs) {
    return rhs.inverse();
}
public extension Horizon {
    public Horizon inverse() {
        return Horizon(
            /* e321 */
            1.0/this.e321 * -1.0
        );
    }
}
// Fancy postfix trick
public Line operator /(Line lhs, InversePrefixOrPostfix rhs)  {
    return lhs.inverse();
    }
// Fancy prefix trick
public Line operator /(InversePrefixOrPostfix lhs, Line rhs) {
    return rhs.inverse();
}
public extension Line {
    public Line inverse() {
        Line reverse = Line(
            /* e41, e42, e43 */
            this.group0.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            this.group1.xyz * float3(-1.0)
        );
        Scalar other = Scalar(
            /* scalar */
            pow(this.e23, 2) + pow(this.e31, 2) + pow(this.e12, 2)
        );
        return Line(
            /* e41, e42, e43 */
            float3(other.scalar) * reverse.group0.xyz, 
            /* e23, e31, e12 */
            float3(other.scalar) * reverse.group1.xyz
        );
    }
}
// Fancy postfix trick
public Motor operator /(Motor lhs, InversePrefixOrPostfix rhs)  {
    return lhs.inverse();
    }
// Fancy prefix trick
public Motor operator /(InversePrefixOrPostfix lhs, Motor rhs) {
    return rhs.inverse();
}
public extension Motor {
    public Motor inverse() {
        Motor reverse = Motor(
            /* e41, e42, e43, e1234 */
            this.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            this.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        Scalar other = Scalar(
            /* scalar */
            pow(this.e23, 2) + pow(this.e31, 2) + pow(this.e12, 2) + pow(this.scalar, 2)
        );
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(other.scalar) * reverse.group0, 
            /* e23, e31, e12, scalar */
            float4(other.scalar) * reverse.group1
        );
    }
}
// Fancy postfix trick
public MultiVector operator /(MultiVector lhs, InversePrefixOrPostfix rhs)  {
    return lhs.inverse();
    }
// Fancy prefix trick
public MultiVector operator /(InversePrefixOrPostfix lhs, MultiVector rhs) {
    return rhs.inverse();
}
public extension MultiVector {
    public MultiVector inverse() {
        MultiVector reverse = MultiVector(
            /* scalar, e1234 */
            this.group0.xy, 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e41, e42, e43 */
            this.group2.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            this.group3.xyz * float3(-1.0), 
            /* e423, e431, e412, e321 */
            this.group4 * float4(-1.0)
        );
        Scalar other = Scalar(
            /* scalar */
            pow(this.scalar, 2) + pow(this.e1, 2) + pow(this.e2, 2) + pow(this.e3, 2) + pow(this.e23, 2) + pow(this.e31, 2) + pow(this.e12, 2) + pow(this.e321, 2)
        );
        return MultiVector(
            /* scalar, e1234 */
            float2(other.scalar) * reverse.group0.xy, 
            /* e1, e2, e3, e4 */
            float4(other.scalar) * reverse.group1, 
            /* e41, e42, e43 */
            float3(other.scalar) * reverse.group2.xyz, 
            /* e23, e31, e12 */
            float3(other.scalar) * reverse.group3.xyz, 
            /* e423, e431, e412, e321 */
            float4(other.scalar) * reverse.group4
        );
    }
}
// Fancy postfix trick
public Plane operator /(Plane lhs, InversePrefixOrPostfix rhs)  {
    return lhs.inverse();
    }
// Fancy prefix trick
public Plane operator /(InversePrefixOrPostfix lhs, Plane rhs) {
    return rhs.inverse();
}
public extension Plane {
    public Plane inverse() {
        return Plane(
            /* e423, e431, e412, e321 */
            float4(pow(this.e321, -2)) * float4(this.e423 * -1.0, this.e431 * -1.0, this.e412 * -1.0, this.e321 * -1.0)
        );
    }
}
// Fancy postfix trick
public Point operator /(Point lhs, InversePrefixOrPostfix rhs)  {
    return lhs.inverse();
    }
// Fancy prefix trick
public Point operator /(InversePrefixOrPostfix lhs, Point rhs) {
    return rhs.inverse();
}
public extension Point {
    public Point inverse() {
        return Point(
            /* e1, e2, e3, e4 */
            float4(pow(this.e1, 2) + pow(this.e2, 2) + pow(this.e3, 2)) * this.group0
        );
    }
}
// Fancy postfix trick
public Scalar operator /(Scalar lhs, InversePrefixOrPostfix rhs)  {
    return lhs.inverse();
    }
// Fancy prefix trick
public Scalar operator /(InversePrefixOrPostfix lhs, Scalar rhs) {
    return rhs.inverse();
}
public extension Scalar {
    public Scalar inverse() {
        return Scalar(
            /* scalar */
            1.0/this.scalar
        );
    }
}

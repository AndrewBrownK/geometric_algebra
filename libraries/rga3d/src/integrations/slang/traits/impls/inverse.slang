implementing rga3d;
using traits;
public extension DualNum {
    // Fancy postfix trick
    public DualNum operator /(InversePrefixOrPostfix rhs)  {
        return this.inverse();
    }
}
public extension InversePrefixOrPostfix {
    // Fancy prefix trick
    public DualNum operator /(DualNum rhs) {
        return rhs.inverse();
    }
}
public extension DualNum {
    public DualNum inverse() {
        return DualNum.from_groups(
            /* scalar, e1234 */
            float2(pow(this.scalar, -2)) * this.group0.xy
        );
    }
}
public extension Flector {
    // Fancy postfix trick
    public Flector operator /(InversePrefixOrPostfix rhs)  {
        return this.inverse();
    }
}
public extension InversePrefixOrPostfix {
    // Fancy prefix trick
    public Flector operator /(Flector rhs) {
        return rhs.inverse();
    }
}
public extension Flector {
    public Flector inverse() {
        Flector reverse = Flector.from_groups(
            /* e1, e2, e3, e4 */
            this.group0, 
            /* e423, e431, e412, e321 */
            this.group1 * float4(-1.0)
        );
        Scalar other = Scalar.from_groups(
            /* scalar */
            pow(this.e1, 2) + pow(this.e2, 2) + pow(this.e3, 2) + pow(this.e321, 2)
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.scalar) * reverse.group0, 
            /* e423, e431, e412, e321 */
            float4(other.scalar) * reverse.group1
        );
    }
}
public extension Horizon {
    // Fancy postfix trick
    public Horizon operator /(InversePrefixOrPostfix rhs)  {
        return this.inverse();
    }
}
public extension InversePrefixOrPostfix {
    // Fancy prefix trick
    public Horizon operator /(Horizon rhs) {
        return rhs.inverse();
    }
}
public extension Horizon {
    public Horizon inverse() {
        return Horizon.from_groups(
            /* e321 */
            1.0/this.e321 * -1.0
        );
    }
}
public extension Line {
    // Fancy postfix trick
    public Line operator /(InversePrefixOrPostfix rhs)  {
        return this.inverse();
    }
}
public extension InversePrefixOrPostfix {
    // Fancy prefix trick
    public Line operator /(Line rhs) {
        return rhs.inverse();
    }
}
public extension Line {
    public Line inverse() {
        Line reverse = Line.from_groups(
            /* e41, e42, e43 */
            this.group0.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            this.group1.xyz * float3(-1.0)
        );
        Scalar other = Scalar.from_groups(
            /* scalar */
            pow(this.e23, 2) + pow(this.e31, 2) + pow(this.e12, 2)
        );
        return Line.from_groups(
            /* e41, e42, e43 */
            float3(other.scalar) * reverse.group0.xyz, 
            /* e23, e31, e12 */
            float3(other.scalar) * reverse.group1.xyz
        );
    }
}
public extension Motor {
    // Fancy postfix trick
    public Motor operator /(InversePrefixOrPostfix rhs)  {
        return this.inverse();
    }
}
public extension InversePrefixOrPostfix {
    // Fancy prefix trick
    public Motor operator /(Motor rhs) {
        return rhs.inverse();
    }
}
public extension Motor {
    public Motor inverse() {
        Motor reverse = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            this.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            this.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        Scalar other = Scalar.from_groups(
            /* scalar */
            pow(this.e23, 2) + pow(this.e31, 2) + pow(this.e12, 2) + pow(this.scalar, 2)
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(other.scalar) * reverse.group0, 
            /* e23, e31, e12, scalar */
            float4(other.scalar) * reverse.group1
        );
    }
}
public extension MultiVector {
    // Fancy postfix trick
    public MultiVector operator /(InversePrefixOrPostfix rhs)  {
        return this.inverse();
    }
}
public extension InversePrefixOrPostfix {
    // Fancy prefix trick
    public MultiVector operator /(MultiVector rhs) {
        return rhs.inverse();
    }
}
public extension MultiVector {
    public MultiVector inverse() {
        MultiVector reverse = MultiVector.from_groups(
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
        Scalar other = Scalar.from_groups(
            /* scalar */
            pow(this.scalar, 2) + pow(this.e1, 2) + pow(this.e2, 2) + pow(this.e3, 2) + pow(this.e23, 2) + pow(this.e31, 2) + pow(this.e12, 2) + pow(this.e321, 2)
        );
        return MultiVector.from_groups(
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
public extension Plane {
    // Fancy postfix trick
    public Plane operator /(InversePrefixOrPostfix rhs)  {
        return this.inverse();
    }
}
public extension InversePrefixOrPostfix {
    // Fancy prefix trick
    public Plane operator /(Plane rhs) {
        return rhs.inverse();
    }
}
public extension Plane {
    public Plane inverse() {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(pow(this.e321, -2)) * float4(this.e423 * -1.0, this.e431 * -1.0, this.e412 * -1.0, this.e321 * -1.0)
        );
    }
}
public extension Point {
    // Fancy postfix trick
    public Point operator /(InversePrefixOrPostfix rhs)  {
        return this.inverse();
    }
}
public extension InversePrefixOrPostfix {
    // Fancy prefix trick
    public Point operator /(Point rhs) {
        return rhs.inverse();
    }
}
public extension Point {
    public Point inverse() {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(pow(this.e1, 2) + pow(this.e2, 2) + pow(this.e3, 2)) * this.group0
        );
    }
}
public extension Scalar {
    // Fancy postfix trick
    public Scalar operator /(InversePrefixOrPostfix rhs)  {
        return this.inverse();
    }
}
public extension InversePrefixOrPostfix {
    // Fancy prefix trick
    public Scalar operator /(Scalar rhs) {
        return rhs.inverse();
    }
}
public extension Scalar {
    public Scalar inverse() {
        return Scalar.from_groups(
            /* scalar */
            1.0/this.scalar
        );
    }
}

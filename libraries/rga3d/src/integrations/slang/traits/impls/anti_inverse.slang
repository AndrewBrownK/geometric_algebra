implementing rga3d;
using traits;
public extension AntiScalar {
    // Fancy postfix trick
    public AntiScalar operator /(AntiInversePrefixOrPostfix rhs)  {
        return this.anti_inverse();
    }
}
public extension AntiInversePrefixOrPostfix {
    // Fancy prefix trick
    public AntiScalar operator /(AntiScalar rhs) {
        return rhs.anti_inverse();
    }
}
public extension AntiScalar {
    public AntiScalar anti_inverse() {
        return AntiScalar(
            /* e1234 */
            1.0/this.e1234
        );
    }
}
public extension DualNum {
    // Fancy postfix trick
    public DualNum operator /(AntiInversePrefixOrPostfix rhs)  {
        return this.anti_inverse();
    }
}
public extension AntiInversePrefixOrPostfix {
    // Fancy prefix trick
    public DualNum operator /(DualNum rhs) {
        return rhs.anti_inverse();
    }
}
public extension DualNum {
    public DualNum anti_inverse() {
        return DualNum(
            /* scalar, e1234 */
            float2(pow(this.e1234, -2)) * this.group0.xy
        );
    }
}
public extension Flector {
    // Fancy postfix trick
    public Flector operator /(AntiInversePrefixOrPostfix rhs)  {
        return this.anti_inverse();
    }
}
public extension AntiInversePrefixOrPostfix {
    // Fancy prefix trick
    public Flector operator /(Flector rhs) {
        return rhs.anti_inverse();
    }
}
public extension Flector {
    public Flector anti_inverse() {
        Flector anti_reverse = Flector(
            /* e1, e2, e3, e4 */
            this.group0 * float4(-1.0), 
            /* e423, e431, e412, e321 */
            this.group1
        );
        AntiScalar other = AntiScalar(
            /* e1234 */
            pow(this.e4, 2) + pow(this.e423, 2) + pow(this.e431, 2) + pow(this.e412, 2)
        );
        return Flector(
            /* e1, e2, e3, e4 */
            float4(other.e1234) * anti_reverse.group0, 
            /* e423, e431, e412, e321 */
            float4(other.e1234) * anti_reverse.group1
        );
    }
}
public extension Line {
    // Fancy postfix trick
    public Line operator /(AntiInversePrefixOrPostfix rhs)  {
        return this.anti_inverse();
    }
}
public extension AntiInversePrefixOrPostfix {
    // Fancy prefix trick
    public Line operator /(Line rhs) {
        return rhs.anti_inverse();
    }
}
public extension Line {
    public Line anti_inverse() {
        Line anti_reverse = Line(
            /* e41, e42, e43 */
            this.group0.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            this.group1.xyz * float3(-1.0)
        );
        AntiScalar other = AntiScalar(
            /* e1234 */
            pow(this.e41, 2) + pow(this.e42, 2) + pow(this.e43, 2)
        );
        return Line(
            /* e41, e42, e43 */
            float3(other.e1234) * anti_reverse.group0.xyz, 
            /* e23, e31, e12 */
            float3(other.e1234) * anti_reverse.group1.xyz
        );
    }
}
public extension Motor {
    // Fancy postfix trick
    public Motor operator /(AntiInversePrefixOrPostfix rhs)  {
        return this.anti_inverse();
    }
}
public extension AntiInversePrefixOrPostfix {
    // Fancy prefix trick
    public Motor operator /(Motor rhs) {
        return rhs.anti_inverse();
    }
}
public extension Motor {
    public Motor anti_inverse() {
        Motor anti_reverse = Motor(
            /* e41, e42, e43, e1234 */
            this.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            this.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        AntiScalar other = AntiScalar(
            /* e1234 */
            pow(this.e41, 2) + pow(this.e42, 2) + pow(this.e43, 2) + pow(this.e1234, 2)
        );
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(other.e1234) * anti_reverse.group0, 
            /* e23, e31, e12, scalar */
            float4(other.e1234) * anti_reverse.group1
        );
    }
}
public extension MultiVector {
    // Fancy postfix trick
    public MultiVector operator /(AntiInversePrefixOrPostfix rhs)  {
        return this.anti_inverse();
    }
}
public extension AntiInversePrefixOrPostfix {
    // Fancy prefix trick
    public MultiVector operator /(MultiVector rhs) {
        return rhs.anti_inverse();
    }
}
public extension MultiVector {
    public MultiVector anti_inverse() {
        MultiVector anti_reverse = MultiVector(
            /* scalar, e1234 */
            this.group0.xy, 
            /* e1, e2, e3, e4 */
            this.group1 * float4(-1.0), 
            /* e41, e42, e43 */
            this.group2.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            this.group3.xyz * float3(-1.0), 
            /* e423, e431, e412, e321 */
            this.group4
        );
        AntiScalar other = AntiScalar(
            /* e1234 */
            pow(this.e1234, 2) + pow(this.e4, 2) + pow(this.e41, 2) + pow(this.e42, 2) + pow(this.e43, 2) + pow(this.e423, 2) + pow(this.e431, 2) + pow(this.e412, 2)
        );
        return MultiVector(
            /* scalar, e1234 */
            float2(other.e1234) * anti_reverse.group0.xy, 
            /* e1, e2, e3, e4 */
            float4(other.e1234) * anti_reverse.group1, 
            /* e41, e42, e43 */
            float3(other.e1234) * anti_reverse.group2.xyz, 
            /* e23, e31, e12 */
            float3(other.e1234) * anti_reverse.group3.xyz, 
            /* e423, e431, e412, e321 */
            float4(other.e1234) * anti_reverse.group4
        );
    }
}
public extension Origin {
    // Fancy postfix trick
    public Origin operator /(AntiInversePrefixOrPostfix rhs)  {
        return this.anti_inverse();
    }
}
public extension AntiInversePrefixOrPostfix {
    // Fancy prefix trick
    public Origin operator /(Origin rhs) {
        return rhs.anti_inverse();
    }
}
public extension Origin {
    public Origin anti_inverse() {
        return Origin(
            /* e4 */
            1.0/this.e4 * -1.0
        );
    }
}
public extension Plane {
    // Fancy postfix trick
    public Plane operator /(AntiInversePrefixOrPostfix rhs)  {
        return this.anti_inverse();
    }
}
public extension AntiInversePrefixOrPostfix {
    // Fancy prefix trick
    public Plane operator /(Plane rhs) {
        return rhs.anti_inverse();
    }
}
public extension Plane {
    public Plane anti_inverse() {
        return Plane(
            /* e423, e431, e412, e321 */
            float4(pow(this.e423, 2) + pow(this.e431, 2) + pow(this.e412, 2)) * this.group0
        );
    }
}
public extension Point {
    // Fancy postfix trick
    public Point operator /(AntiInversePrefixOrPostfix rhs)  {
        return this.anti_inverse();
    }
}
public extension AntiInversePrefixOrPostfix {
    // Fancy prefix trick
    public Point operator /(Point rhs) {
        return rhs.anti_inverse();
    }
}
public extension Point {
    public Point anti_inverse() {
        return Point(
            /* e1, e2, e3, e4 */
            float4(pow(this.e4, -2)) * float4(this.e1 * -1.0, this.e2 * -1.0, this.e3 * -1.0, this.e4 * -1.0)
        );
    }
}

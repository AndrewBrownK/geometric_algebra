implementing rga3d;
using traits;
public extension DualNum {
    // Fancy postfix trick
    public Horizon operator /(AntiSupportPrefixOrPostfix rhs)  {
        return this.anti_support();
    }
}
public extension AntiSupportPrefixOrPostfix {
    // Fancy prefix trick
    public Horizon operator /(DualNum rhs) {
        return rhs.anti_support();
    }
}
public extension DualNum {
    public Horizon anti_support() {
        return Horizon(
            /* e321 */
            this.scalar
        );
    }
}
public extension Flector {
    // Fancy postfix trick
    public Motor operator /(AntiSupportPrefixOrPostfix rhs)  {
        return this.anti_support();
    }
}
public extension AntiSupportPrefixOrPostfix {
    // Fancy prefix trick
    public Motor operator /(Flector rhs) {
        return rhs.anti_support();
    }
}
public extension Flector {
    public Motor anti_support() {
        Flector right_dual = Flector(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(this.group0.xyz, 0.0)
        );
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(0.0), 
            /* e23, e31, e12, scalar */
            float4(right_dual.group1.xyz, right_dual.e4) * float4(-1.0)
        );
    }
}
public extension Horizon {
    // Fancy postfix trick
    public Scalar operator /(AntiSupportPrefixOrPostfix rhs)  {
        return this.anti_support();
    }
}
public extension AntiSupportPrefixOrPostfix {
    // Fancy prefix trick
    public Scalar operator /(Horizon rhs) {
        return rhs.anti_support();
    }
}
public extension Horizon {
    public Scalar anti_support() {
        return Scalar(
            /* scalar */
            this.e321
        );
    }
}
public extension Line {
    // Fancy postfix trick
    public Point operator /(AntiSupportPrefixOrPostfix rhs)  {
        return this.anti_support();
    }
}
public extension AntiSupportPrefixOrPostfix {
    // Fancy prefix trick
    public Point operator /(Line rhs) {
        return rhs.anti_support();
    }
}
public extension Line {
    public Point anti_support() {
        return Point(
            /* e1, e2, e3, e4 */
            float4((this.group1.xyz * float3(-1.0)), 0.0)
        );
    }
}
public extension Motor {
    // Fancy postfix trick
    public Flector operator /(AntiSupportPrefixOrPostfix rhs)  {
        return this.anti_support();
    }
}
public extension AntiSupportPrefixOrPostfix {
    // Fancy prefix trick
    public Flector operator /(Motor rhs) {
        return rhs.anti_support();
    }
}
public extension Motor {
    public Flector anti_support() {
        Horizon right_complement = Horizon(
            /* e321 */
            1.0
        );
        Motor right_dual = Motor(
            /* e41, e42, e43, e1234 */
            this.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
        return Flector(
            /* e1, e2, e3, e4 */
            float4((float3(right_complement.e321) * right_dual.group0.xyz), 0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), right_complement.e321 * right_dual.e1234)
        );
    }
}
public extension MultiVector {
    // Fancy postfix trick
    public MultiVector operator /(AntiSupportPrefixOrPostfix rhs)  {
        return this.anti_support();
    }
}
public extension AntiSupportPrefixOrPostfix {
    // Fancy prefix trick
    public MultiVector operator /(MultiVector rhs) {
        return rhs.anti_support();
    }
}
public extension MultiVector {
    public MultiVector anti_support() {
        Horizon right_complement = Horizon(
            /* e321 */
            1.0
        );
        MultiVector right_dual = MultiVector(
            /* scalar, e1234 */
            float2(0.0, this.scalar), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.e321 * -1.0), 
            /* e41, e42, e43 */
            this.group3.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(this.group1.xyz, 0.0)
        );
        return MultiVector(
            /* scalar, e1234 */
            float2(right_complement.e321 * right_dual.e4, 1.0) * float2(-1.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((float3(right_complement.e321) * right_dual.group2.xyz), 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(right_complement.e321) * right_dual.group4.xyz * float3(-1.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), right_complement.e321 * right_dual.e1234)
        );
    }
}
public extension Plane {
    // Fancy postfix trick
    public Scalar operator /(AntiSupportPrefixOrPostfix rhs)  {
        return this.anti_support();
    }
}
public extension AntiSupportPrefixOrPostfix {
    // Fancy prefix trick
    public Scalar operator /(Plane rhs) {
        return rhs.anti_support();
    }
}
public extension Plane {
    public Scalar anti_support() {
        return Scalar(
            /* scalar */
            this.e321
        );
    }
}
public extension Point {
    // Fancy postfix trick
    public Line operator /(AntiSupportPrefixOrPostfix rhs)  {
        return this.anti_support();
    }
}
public extension AntiSupportPrefixOrPostfix {
    // Fancy prefix trick
    public Line operator /(Point rhs) {
        return rhs.anti_support();
    }
}
public extension Point {
    public Line anti_support() {
        return Line(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            this.group0.xyz * float3(-1.0)
        );
    }
}
public extension Scalar {
    // Fancy postfix trick
    public Horizon operator /(AntiSupportPrefixOrPostfix rhs)  {
        return this.anti_support();
    }
}
public extension AntiSupportPrefixOrPostfix {
    // Fancy prefix trick
    public Horizon operator /(Scalar rhs) {
        return rhs.anti_support();
    }
}
public extension Scalar {
    public Horizon anti_support() {
        return Horizon(
            /* e321 */
            this.scalar
        );
    }
}

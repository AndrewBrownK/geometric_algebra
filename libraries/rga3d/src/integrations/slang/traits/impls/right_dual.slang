implementing rga3d;
using traits;
public extension DualNum {
    // Fancy postfix trick
    public AntiScalar operator /(RightDualPrefixOrPostfix rhs)  {
        return this.right_dual();
    }
}
public extension RightDualPrefixOrPostfix {
    // Fancy prefix trick
    public AntiScalar operator /(DualNum rhs) {
        return rhs.right_dual();
    }
}
public extension DualNum {
    public AntiScalar right_dual() {
        return AntiScalar(
            /* e1234 */
            this.scalar
        );
    }
}
public extension Flector {
    // Fancy postfix trick
    public Flector operator /(RightDualPrefixOrPostfix rhs)  {
        return this.right_dual();
    }
}
public extension RightDualPrefixOrPostfix {
    // Fancy prefix trick
    public Flector operator /(Flector rhs) {
        return rhs.right_dual();
    }
}
public extension Flector {
    public Flector right_dual() {
        return Flector(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(this.group0.xyz, 0.0)
        );
    }
}
public extension Horizon {
    // Fancy postfix trick
    public Origin operator /(RightDualPrefixOrPostfix rhs)  {
        return this.right_dual();
    }
}
public extension RightDualPrefixOrPostfix {
    // Fancy prefix trick
    public Origin operator /(Horizon rhs) {
        return rhs.right_dual();
    }
}
public extension Horizon {
    public Origin right_dual() {
        return Origin(
            /* e4 */
            this.e321 * -1.0
        );
    }
}
public extension Line {
    // Fancy postfix trick
    public Line operator /(RightDualPrefixOrPostfix rhs)  {
        return this.right_dual();
    }
}
public extension RightDualPrefixOrPostfix {
    // Fancy prefix trick
    public Line operator /(Line rhs) {
        return rhs.right_dual();
    }
}
public extension Line {
    public Line right_dual() {
        return Line(
            /* e41, e42, e43 */
            this.group1.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0)
        );
    }
}
public extension Motor {
    // Fancy postfix trick
    public Motor operator /(RightDualPrefixOrPostfix rhs)  {
        return this.right_dual();
    }
}
public extension RightDualPrefixOrPostfix {
    // Fancy prefix trick
    public Motor operator /(Motor rhs) {
        return rhs.right_dual();
    }
}
public extension Motor {
    public Motor right_dual() {
        return Motor(
            /* e41, e42, e43, e1234 */
            this.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
    }
}
public extension MultiVector {
    // Fancy postfix trick
    public MultiVector operator /(RightDualPrefixOrPostfix rhs)  {
        return this.right_dual();
    }
}
public extension RightDualPrefixOrPostfix {
    // Fancy prefix trick
    public MultiVector operator /(MultiVector rhs) {
        return rhs.right_dual();
    }
}
public extension MultiVector {
    public MultiVector right_dual() {
        return MultiVector(
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
    }
}
public extension Plane {
    // Fancy postfix trick
    public Origin operator /(RightDualPrefixOrPostfix rhs)  {
        return this.right_dual();
    }
}
public extension RightDualPrefixOrPostfix {
    // Fancy prefix trick
    public Origin operator /(Plane rhs) {
        return rhs.right_dual();
    }
}
public extension Plane {
    public Origin right_dual() {
        return Origin(
            /* e4 */
            this.e321 * -1.0
        );
    }
}
public extension Point {
    // Fancy postfix trick
    public Plane operator /(RightDualPrefixOrPostfix rhs)  {
        return this.right_dual();
    }
}
public extension RightDualPrefixOrPostfix {
    // Fancy prefix trick
    public Plane operator /(Point rhs) {
        return rhs.right_dual();
    }
}
public extension Point {
    public Plane right_dual() {
        return Plane(
            /* e423, e431, e412, e321 */
            float4(this.group0.xyz, 0.0)
        );
    }
}
public extension Scalar {
    // Fancy postfix trick
    public AntiScalar operator /(RightDualPrefixOrPostfix rhs)  {
        return this.right_dual();
    }
}
public extension RightDualPrefixOrPostfix {
    // Fancy prefix trick
    public AntiScalar operator /(Scalar rhs) {
        return rhs.right_dual();
    }
}
public extension Scalar {
    public AntiScalar right_dual() {
        return AntiScalar(
            /* e1234 */
            this.scalar
        );
    }
}

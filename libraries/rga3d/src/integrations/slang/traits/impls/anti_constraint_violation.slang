implementing rga3d;
using traits;
// Fancy postfix trick
public Scalar operator /(DualNum lhs, AntiConstraintViolationPrefixOrPostfix rhs)  {
    return lhs.anti_constraint_violation();
    }
// Fancy prefix trick
public Scalar operator /(AntiConstraintViolationPrefixOrPostfix lhs, DualNum rhs) {
    return rhs.anti_constraint_violation();
}
public extension DualNum {
    public Scalar anti_constraint_violation() {
        return Scalar(
            /* scalar */
            this.scalar * this.e1234 * 2.0
        );
    }
}
// Fancy postfix trick
public Scalar operator /(Flector lhs, AntiConstraintViolationPrefixOrPostfix rhs)  {
    return lhs.anti_constraint_violation();
    }
// Fancy prefix trick
public Scalar operator /(AntiConstraintViolationPrefixOrPostfix lhs, Flector rhs) {
    return rhs.anti_constraint_violation();
}
public extension Flector {
    public Scalar anti_constraint_violation() {
        Flector anti_reverse = Flector(
            /* e1, e2, e3, e4 */
            this.group0 * float4(-1.0), 
            /* e423, e431, e412, e321 */
            this.group1
        );
        return Scalar(
            /* scalar */
            (anti_reverse.e423 * this.e1) + (anti_reverse.e431 * this.e2) + (anti_reverse.e412 * this.e3) + (anti_reverse.e321 * this.e4) - (anti_reverse.e1 * this.e423) - (anti_reverse.e2 * this.e431) - (anti_reverse.e3 * this.e412) - (anti_reverse.e4 * this.e321)
        );
    }
}
// Fancy postfix trick
public Scalar operator /(Line lhs, AntiConstraintViolationPrefixOrPostfix rhs)  {
    return lhs.anti_constraint_violation();
    }
// Fancy prefix trick
public Scalar operator /(AntiConstraintViolationPrefixOrPostfix lhs, Line rhs) {
    return rhs.anti_constraint_violation();
}
public extension Line {
    public Scalar anti_constraint_violation() {
        Line anti_reverse = Line(
            /* e41, e42, e43 */
            this.group0.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            this.group1.xyz * float3(-1.0)
        );
        return Scalar(
            /* scalar */
            -(anti_reverse.e41 * this.e23) - (anti_reverse.e42 * this.e31) - (anti_reverse.e43 * this.e12) - (anti_reverse.e23 * this.e41) - (anti_reverse.e31 * this.e42) - (anti_reverse.e12 * this.e43)
        );
    }
}
// Fancy postfix trick
public Scalar operator /(Motor lhs, AntiConstraintViolationPrefixOrPostfix rhs)  {
    return lhs.anti_constraint_violation();
    }
// Fancy prefix trick
public Scalar operator /(AntiConstraintViolationPrefixOrPostfix lhs, Motor rhs) {
    return rhs.anti_constraint_violation();
}
public extension Motor {
    public Scalar anti_constraint_violation() {
        Motor anti_reverse = Motor(
            /* e41, e42, e43, e1234 */
            this.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            this.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Scalar(
            /* scalar */
            (anti_reverse.e1234 * this.scalar) + (anti_reverse.scalar * this.e1234) - (anti_reverse.e41 * this.e23) - (anti_reverse.e42 * this.e31) - (anti_reverse.e43 * this.e12) - (anti_reverse.e23 * this.e41) - (anti_reverse.e31 * this.e42) - (anti_reverse.e12 * this.e43)
        );
    }
}
// Fancy postfix trick
public MultiVector operator /(MultiVector lhs, AntiConstraintViolationPrefixOrPostfix rhs)  {
    return lhs.anti_constraint_violation();
    }
// Fancy prefix trick
public MultiVector operator /(AntiConstraintViolationPrefixOrPostfix lhs, MultiVector rhs) {
    return rhs.anti_constraint_violation();
}
public extension MultiVector {
    public MultiVector anti_constraint_violation() {
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
        MultiVector geometric_anti_product = MultiVector(
            /* scalar, e1234 */
            float2((anti_reverse.e1234 * this.scalar) + (anti_reverse.e321 * this.e4) - (anti_reverse.e2 * this.e431) - (anti_reverse.e3 * this.e412) - (anti_reverse.e4 * this.e321) - (anti_reverse.e23 * this.e41) - (anti_reverse.e31 * this.e42) - (anti_reverse.e12 * this.e43), 0.0) + (float2(anti_reverse.e423) * float2(this.e1, this.e423)) + (float2(anti_reverse.e431) * float2(this.e2, this.e431)) + (float2(anti_reverse.e412) * float2(this.e3, this.e412)) + (float2(this.e1234) * anti_reverse.group0.xy) - (float2(anti_reverse.e41) * float2(this.e23, this.e41)) - (float2(anti_reverse.e42) * float2(this.e31, this.e42)) - (float2(anti_reverse.e43) * float2(this.e12, this.e43)) - (float2(this.e423, this.e4) * anti_reverse.group1.xw), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            (float4(anti_reverse.e1234) * this.group4) + (float4(this.e4, this.e412, this.e423, anti_reverse.e321) * float4(anti_reverse.group2.xyz.xxy, this.e1234)) + (float4(this.e431, this.e4, this.e4, this.e423) * float4(anti_reverse.group2.xyz.zyz, anti_reverse.e23)) + (float4(this.group0.xy.yy, this.e1234, this.scalar) * float4(anti_reverse.group4.xyz, anti_reverse.e4)) + (float4(anti_reverse.group1.ww, anti_reverse.e431, this.e431) * float4(this.group2.xyz.xyx, anti_reverse.e31)) + (float4(anti_reverse.group4.zx, anti_reverse.e4, this.e412) * float4(this.group2.xyz.yzz, anti_reverse.e12)) + float4(float3(0.0), -(anti_reverse.e1 * this.e41) - (anti_reverse.e2 * this.e42) - (anti_reverse.e3 * this.e43) - (anti_reverse.e42 * this.e2) - (anti_reverse.e43 * this.e3) - (anti_reverse.e423 * this.e23) - (anti_reverse.e431 * this.e31) - (anti_reverse.e412 * this.e12)) - float4((anti_reverse.group2.xyz.yzx * this.group4.zxy), anti_reverse.scalar * this.e4) - float4((this.group2.xyz.zxy * anti_reverse.group4.yzx), anti_reverse.e41 * this.e1)
        );
        return MultiVector(
            /* scalar, e1234 */
            float2(geometric_anti_product.scalar, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            geometric_anti_product.group4
        );
    }
}

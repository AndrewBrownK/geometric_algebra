implementing rga3d;
using traits;
public extension DualNum {
    // Fancy postfix trick
    public AntiScalar operator /(ConstraintViolationPrefixOrPostfix rhs)  {
        return this.constraint_violation();
    }
}
public extension ConstraintViolationPrefixOrPostfix {
    // Fancy prefix trick
    public AntiScalar operator /(DualNum rhs) {
        return rhs.constraint_violation();
    }
}
public extension DualNum: ConstraintViolation {
    public typedef AntiScalar ConstraintViolationOutput;
    public func constraint_violation() -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            this.scalar * this.e1234 * 2.0
        );
    }
}
public extension Flector {
    // Fancy postfix trick
    public AntiScalar operator /(ConstraintViolationPrefixOrPostfix rhs)  {
        return this.constraint_violation();
    }
}
public extension ConstraintViolationPrefixOrPostfix {
    // Fancy prefix trick
    public AntiScalar operator /(Flector rhs) {
        return rhs.constraint_violation();
    }
}
public extension Flector: ConstraintViolation {
    public typedef AntiScalar ConstraintViolationOutput;
    public func constraint_violation() -> AntiScalar {
        Flector reverse = Flector.from_groups(
            /* e1, e2, e3, e4 */
            this.group0, 
            /* e423, e431, e412, e321 */
            this.group1 * float4(-1.0)
        );
        return AntiScalar.from_groups(
            /* e1234 */
            (reverse.e423 * this.e1) + (reverse.e431 * this.e2) + (reverse.e412 * this.e3) + (reverse.e321 * this.e4) - (reverse.e1 * this.e423) - (reverse.e2 * this.e431) - (reverse.e3 * this.e412) - (reverse.e4 * this.e321)
        );
    }
}
public extension Line {
    // Fancy postfix trick
    public AntiScalar operator /(ConstraintViolationPrefixOrPostfix rhs)  {
        return this.constraint_violation();
    }
}
public extension ConstraintViolationPrefixOrPostfix {
    // Fancy prefix trick
    public AntiScalar operator /(Line rhs) {
        return rhs.constraint_violation();
    }
}
public extension Line: ConstraintViolation {
    public typedef AntiScalar ConstraintViolationOutput;
    public func constraint_violation() -> AntiScalar {
        Line reverse = Line.from_groups(
            /* e41, e42, e43 */
            this.group0.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            this.group1.xyz * float3(-1.0)
        );
        return AntiScalar.from_groups(
            /* e1234 */
            -(reverse.e41 * this.e23) - (reverse.e42 * this.e31) - (reverse.e43 * this.e12) - (reverse.e23 * this.e41) - (reverse.e31 * this.e42) - (reverse.e12 * this.e43)
        );
    }
}
public extension Motor {
    // Fancy postfix trick
    public AntiScalar operator /(ConstraintViolationPrefixOrPostfix rhs)  {
        return this.constraint_violation();
    }
}
public extension ConstraintViolationPrefixOrPostfix {
    // Fancy prefix trick
    public AntiScalar operator /(Motor rhs) {
        return rhs.constraint_violation();
    }
}
public extension Motor: ConstraintViolation {
    public typedef AntiScalar ConstraintViolationOutput;
    public func constraint_violation() -> AntiScalar {
        Motor reverse = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            this.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            this.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiScalar.from_groups(
            /* e1234 */
            (reverse.e1234 * this.scalar) + (reverse.scalar * this.e1234) - (reverse.e41 * this.e23) - (reverse.e42 * this.e31) - (reverse.e43 * this.e12) - (reverse.e23 * this.e41) - (reverse.e31 * this.e42) - (reverse.e12 * this.e43)
        );
    }
}
public extension MultiVector {
    // Fancy postfix trick
    public MultiVector operator /(ConstraintViolationPrefixOrPostfix rhs)  {
        return this.constraint_violation();
    }
    // Fancy postfix self-assign
    public func operator /=(const ConstraintViolationPrefixOrPostfix& rhs) -> MultiVector& {
        this = this.constraint_violation();
        return *this;
    }
}
public extension ConstraintViolationPrefixOrPostfix {
    // Fancy prefix trick
    public MultiVector operator /(MultiVector rhs) {
        return rhs.constraint_violation();
    }
}
public extension MultiVector: ConstraintViolation {
    public typedef MultiVector ConstraintViolationOutput;
    public func constraint_violation() -> MultiVector {
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
        MultiVector geometric_product = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, (reverse.e1234 * this.scalar) + (reverse.e321 * this.e4) - (reverse.e2 * this.e431) - (reverse.e3 * this.e412) - (reverse.e4 * this.e321) - (reverse.e23 * this.e41) - (reverse.e31 * this.e42) - (reverse.e12 * this.e43)) + (float2(reverse.scalar) * this.group0.xy) + (float2(this.e1) * float2(reverse.e1, reverse.e423)) + (float2(this.e2) * float2(reverse.e2, reverse.e431)) + (float2(this.e3) * float2(reverse.e3, reverse.e412)) - (float2(this.e23) * float2(reverse.e23, reverse.e41)) - (float2(this.e31) * float2(reverse.e31, reverse.e42)) - (float2(this.e12) * float2(reverse.e12, reverse.e43)) - (float2(reverse.e321, reverse.e1) * this.group4.wx), 
            /* e1, e2, e3, e4 */
            (float4(reverse.scalar) * this.group1) + (float4(reverse.e2, reverse.e321, reverse.e321, reverse.e3) * float4(this.group3.xyz.zyz, this.e43)) + (float4(reverse.e321, reverse.e3, reverse.e1, reverse.e2) * float4(this.group3.xyz.xxy, this.e42)) + (float4(this.group0.xy.xx, this.scalar, reverse.e1234) * float4(reverse.group1.xyz, this.e321)) + (float4(this.group1.zx, this.e321, reverse.e1) * float4(reverse.group3.xyz.yzz, this.e41)) + (float4(this.group4.ww, this.e2, reverse.e4) * float4(reverse.group3.xyz.xyx, this.scalar)) + float4(float3(0.0), -(reverse.e42 * this.e2) - (reverse.e43 * this.e3) - (reverse.e23 * this.e423) - (reverse.e31 * this.e431) - (reverse.e12 * this.e412) - (reverse.e423 * this.e23) - (reverse.e431 * this.e31) - (reverse.e412 * this.e12)) - float4((reverse.group3.xyz.zxy * this.group1.yzx), reverse.e321 * this.e1234) - float4((this.group3.xyz.yzx * reverse.group1.zxy), reverse.e41 * this.e1), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, geometric_product.e1234), 
            /* e1, e2, e3, e4 */
            geometric_product.group1, 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}

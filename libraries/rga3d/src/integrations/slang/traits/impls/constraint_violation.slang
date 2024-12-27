extension DualNum {
    // Fancy infix trick
    func operator/(_rhs: ConstraintViolationPrefixOrPostfix) -> AntiScalar {
        return this.constraint_violation();
    }
}
extension DualNum: ConstraintViolation {
    associatedtype Output = AntiScalar;
    func constraint_violation() -> Output {
        return AntiScalar.from_groups(
            /* e1234, 0, 0, 0 */
            float4(this.scalar * this.e1234 * 2.0, 0.0, 0.0, 0.0)
        );
    }
}
extension Flector {
    // Fancy infix trick
    func operator/(_rhs: ConstraintViolationPrefixOrPostfix) -> AntiScalar {
        return this.constraint_violation();
    }
}
extension Flector: ConstraintViolation {
    associatedtype Output = AntiScalar;
    func constraint_violation() -> Output {
        Flector reverse = Flector.from_groups(
            /* e1, e2, e3, e4 */
            this.group0, 
            /* e423, e431, e412, e321 */
            this.group1 * float4(-1.0)
        );
        return AntiScalar.from_groups(
            /* e1234, 0, 0, 0 */
            float4((reverse.e423 * this.e1) + (reverse.e431 * this.e2) + (reverse.e412 * this.e3) + (reverse.e321 * this.e4) - (reverse.e1 * this.e423) - (reverse.e2 * this.e431) - (reverse.e3 * this.e412) - (reverse.e4 * this.e321), 0.0, 0.0, 0.0)
        );
    }
}
extension Line {
    // Fancy infix trick
    func operator/(_rhs: ConstraintViolationPrefixOrPostfix) -> AntiScalar {
        return this.constraint_violation();
    }
}
extension Line: ConstraintViolation {
    associatedtype Output = AntiScalar;
    func constraint_violation() -> Output {
        Line reverse = Line.from_groups(
            /* e41, e42, e43, 0 */
            float4(this.group0.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group1.xyz * float3(-1.0), 0.0)
        );
        return AntiScalar.from_groups(
            /* e1234, 0, 0, 0 */
            float4(-(reverse.e41 * this.e23) - (reverse.e42 * this.e31) - (reverse.e43 * this.e12) - (reverse.e23 * this.e41) - (reverse.e31 * this.e42) - (reverse.e12 * this.e43), 0.0, 0.0, 0.0)
        );
    }
}
extension Motor {
    // Fancy infix trick
    func operator/(_rhs: ConstraintViolationPrefixOrPostfix) -> AntiScalar {
        return this.constraint_violation();
    }
}
extension Motor: ConstraintViolation {
    associatedtype Output = AntiScalar;
    func constraint_violation() -> Output {
        Motor reverse = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            this.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            this.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiScalar.from_groups(
            /* e1234, 0, 0, 0 */
            float4((reverse.e1234 * this.scalar) + (reverse.scalar * this.e1234) - (reverse.e41 * this.e23) - (reverse.e42 * this.e31) - (reverse.e43 * this.e12) - (reverse.e23 * this.e41) - (reverse.e31 * this.e42) - (reverse.e12 * this.e43), 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector {
    // Fancy infix trick
    func operator/(_rhs: ConstraintViolationPrefixOrPostfix) -> MultiVector {
        return this.constraint_violation();
    }
    func operator/=(const ConstraintViolationPrefixOrPostfix& _rhs) -> MultiVector& {
        this = this.constraint_violation();
        return *this;
    }
}
extension MultiVector: ConstraintViolation {
    associatedtype Output = MultiVector;
    func constraint_violation() -> Output {
        MultiVector reverse = MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e41, e42, e43, 0 */
            float4(this.group2.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group3.xyz * float3(-1.0), 0.0), 
            /* e423, e431, e412, e321 */
            this.group4 * float4(-1.0)
        );
        MultiVector geometric_product = MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(0.0, (reverse.e1234 * this.scalar) + (reverse.e321 * this.e4) - (reverse.e2 * this.e431) - (reverse.e3 * this.e412) - (reverse.e4 * this.e321) - (reverse.e23 * this.e41) - (reverse.e31 * this.e42) - (reverse.e12 * this.e43)) + (float2(reverse.scalar) * this.group0.xy) + (float2(this.e1) * float2(reverse.e1, reverse.e423)) + (float2(this.e2) * float2(reverse.e2, reverse.e431)) + (float2(this.e3) * float2(reverse.e3, reverse.e412)) - (float2(this.e23) * float2(reverse.e23, reverse.e41)) - (float2(this.e31) * float2(reverse.e31, reverse.e42)) - (float2(this.e12) * float2(reverse.e12, reverse.e43)) - (float2(reverse.e321, reverse.e1) * this.group4.wx), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            (float4(reverse.scalar) * this.group1) + (float4(reverse.e2, reverse.e321, reverse.e321, reverse.e3) * float4(this.group3.xyz.zyz, this.e43)) + (float4(reverse.e321, reverse.e3, reverse.e1, reverse.e2) * float4(this.group3.xyz.xxy, this.e42)) + (float4(this.group0.xy.xx, this.scalar, reverse.e1234) * float4(reverse.group1.xyz, this.e321)) + (float4(this.group1.zx, this.e321, reverse.e1) * float4(reverse.group3.xyz.yzz, this.e41)) + (float4(this.group4.ww, this.e2, reverse.e4) * float4(reverse.group3.xyz.xyx, this.scalar)) + float4(float3(0.0), -(reverse.e42 * this.e2) - (reverse.e43 * this.e3) - (reverse.e23 * this.e423) - (reverse.e31 * this.e431) - (reverse.e12 * this.e412) - (reverse.e423 * this.e23) - (reverse.e431 * this.e31) - (reverse.e412 * this.e12)) - float4((reverse.group3.xyz.zxy * this.group1.yzx), reverse.e321 * this.e1234) - float4((this.group3.xyz.yzx * reverse.group1.zxy), reverse.e41 * this.e1), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(0.0, geometric_product.e1234), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            geometric_product.group1, 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}

extension DualNum {
    // Fancy infix trick
    func operator/(_rhs: AntiConstraintViolationPrefixOrPostfix) -> Scalar {
        return this.anti_constraint_violation();
    }
}
extension DualNum: AntiConstraintViolation {
    associatedtype Output = Scalar;
    func anti_constraint_violation() -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(this.scalar * this.e1234 * 2.0, 0.0, 0.0, 0.0)
        );
    }
}
extension Flector {
    // Fancy infix trick
    func operator/(_rhs: AntiConstraintViolationPrefixOrPostfix) -> Scalar {
        return this.anti_constraint_violation();
    }
}
extension Flector: AntiConstraintViolation {
    associatedtype Output = Scalar;
    func anti_constraint_violation() -> Output {
        Flector anti_reverse = Flector.from_groups(
            /* e1, e2, e3, e4 */
            this.group0 * float4(-1.0), 
            /* e423, e431, e412, e321 */
            this.group1
        );
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((anti_reverse.e423 * this.e1) + (anti_reverse.e431 * this.e2) + (anti_reverse.e412 * this.e3) + (anti_reverse.e321 * this.e4) - (anti_reverse.e1 * this.e423) - (anti_reverse.e2 * this.e431) - (anti_reverse.e3 * this.e412) - (anti_reverse.e4 * this.e321), 0.0, 0.0, 0.0)
        );
    }
}
extension Line {
    // Fancy infix trick
    func operator/(_rhs: AntiConstraintViolationPrefixOrPostfix) -> Scalar {
        return this.anti_constraint_violation();
    }
}
extension Line: AntiConstraintViolation {
    associatedtype Output = Scalar;
    func anti_constraint_violation() -> Output {
        Line anti_reverse = Line.from_groups(
            /* e41, e42, e43, 0 */
            float4(this.group0.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group1.xyz * float3(-1.0), 0.0)
        );
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(-(anti_reverse.e41 * this.e23) - (anti_reverse.e42 * this.e31) - (anti_reverse.e43 * this.e12) - (anti_reverse.e23 * this.e41) - (anti_reverse.e31 * this.e42) - (anti_reverse.e12 * this.e43), 0.0, 0.0, 0.0)
        );
    }
}
extension Motor {
    // Fancy infix trick
    func operator/(_rhs: AntiConstraintViolationPrefixOrPostfix) -> Scalar {
        return this.anti_constraint_violation();
    }
}
extension Motor: AntiConstraintViolation {
    associatedtype Output = Scalar;
    func anti_constraint_violation() -> Output {
        Motor anti_reverse = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            this.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            this.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((anti_reverse.e1234 * this.scalar) + (anti_reverse.scalar * this.e1234) - (anti_reverse.e41 * this.e23) - (anti_reverse.e42 * this.e31) - (anti_reverse.e43 * this.e12) - (anti_reverse.e23 * this.e41) - (anti_reverse.e31 * this.e42) - (anti_reverse.e12 * this.e43), 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector {
    // Fancy infix trick
    func operator/(_rhs: AntiConstraintViolationPrefixOrPostfix) -> MultiVector {
        return this.anti_constraint_violation();
    }
    func operator/=(const AntiConstraintViolationPrefixOrPostfix& _rhs) -> MultiVector& {
        this = this.anti_constraint_violation();
        return *this;
    }
}
extension MultiVector: AntiConstraintViolation {
    associatedtype Output = MultiVector;
    func anti_constraint_violation() -> Output {
        MultiVector anti_reverse = MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group1 * float4(-1.0), 
            /* e41, e42, e43, 0 */
            float4(this.group2.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group3.xyz * float3(-1.0), 0.0), 
            /* e423, e431, e412, e321 */
            this.group4
        );
        MultiVector geometric_anti_product = MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2((anti_reverse.e1234 * this.scalar) + (anti_reverse.e321 * this.e4) - (anti_reverse.e2 * this.e431) - (anti_reverse.e3 * this.e412) - (anti_reverse.e4 * this.e321) - (anti_reverse.e23 * this.e41) - (anti_reverse.e31 * this.e42) - (anti_reverse.e12 * this.e43), 0.0) + (float2(anti_reverse.e423) * float2(this.e1, this.e423)) + (float2(anti_reverse.e431) * float2(this.e2, this.e431)) + (float2(anti_reverse.e412) * float2(this.e3, this.e412)) + (float2(this.e1234) * anti_reverse.group0.xy) - (float2(anti_reverse.e41) * float2(this.e23, this.e41)) - (float2(anti_reverse.e42) * float2(this.e31, this.e42)) - (float2(anti_reverse.e43) * float2(this.e12, this.e43)) - (float2(this.e423, this.e4) * anti_reverse.group1.xw), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e423, e431, e412, e321 */
            (float4(anti_reverse.e1234) * this.group4) + (float4(this.e4, this.e412, this.e423, anti_reverse.e321) * float4(anti_reverse.group2.xyz.xxy, this.e1234)) + (float4(this.e431, this.e4, this.e4, this.e423) * float4(anti_reverse.group2.xyz.zyz, anti_reverse.e23)) + (float4(this.group0.xy.yy, this.e1234, this.scalar) * float4(anti_reverse.group4.xyz, anti_reverse.e4)) + (float4(anti_reverse.group1.ww, anti_reverse.e431, this.e431) * float4(this.group2.xyz.xyx, anti_reverse.e31)) + (float4(anti_reverse.group4.zx, anti_reverse.e4, this.e412) * float4(this.group2.xyz.yzz, anti_reverse.e12)) + float4(float3(0.0), -(anti_reverse.e1 * this.e41) - (anti_reverse.e2 * this.e42) - (anti_reverse.e3 * this.e43) - (anti_reverse.e42 * this.e2) - (anti_reverse.e43 * this.e3) - (anti_reverse.e423 * this.e23) - (anti_reverse.e431 * this.e31) - (anti_reverse.e412 * this.e12)) - float4((anti_reverse.group2.xyz.yzx * this.group4.zxy), anti_reverse.scalar * this.e4) - float4((this.group2.xyz.zxy * anti_reverse.group4.yzx), anti_reverse.e41 * this.e1)
        );
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(geometric_anti_product.scalar, 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e423, e431, e412, e321 */
            geometric_anti_product.group4
        );
    }
}

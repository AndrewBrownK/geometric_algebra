public extension DualNum {
    // Fancy postfix trick
    func operator/(_rhs: AntiConstraintViolationPrefixOrPostfix) -> Scalar {
        return this.anti_constraint_violation();
    }
}
public extension AntiConstraintViolationPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: DualNum) -> DualNum.Output {
        return rhs.anti_constraint_violation();
    }
}
public extension DualNum: AntiConstraintViolation {
    public typedef Scalar AntiConstraintViolationOutput;
    public func anti_constraint_violation() -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            this.scalar * this.e1234 * 2.0
        );
    }
}
public extension Flector {
    // Fancy postfix trick
    func operator/(_rhs: AntiConstraintViolationPrefixOrPostfix) -> Scalar {
        return this.anti_constraint_violation();
    }
}
public extension AntiConstraintViolationPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: Flector) -> Flector.Output {
        return rhs.anti_constraint_violation();
    }
}
public extension Flector: AntiConstraintViolation {
    public typedef Scalar AntiConstraintViolationOutput;
    public func anti_constraint_violation() -> Scalar {
        Flector anti_reverse = Flector.from_groups(
            /* e1, e2, e3, e4 */
            this.group0 * float4(-1.0), 
            /* e423, e431, e412, e321 */
            this.group1
        );
        return Scalar.from_groups(
            /* scalar */
            (anti_reverse.e423 * this.e1) + (anti_reverse.e431 * this.e2) + (anti_reverse.e412 * this.e3) + (anti_reverse.e321 * this.e4) - (anti_reverse.e1 * this.e423) - (anti_reverse.e2 * this.e431) - (anti_reverse.e3 * this.e412) - (anti_reverse.e4 * this.e321)
        );
    }
}
public extension Line {
    // Fancy postfix trick
    func operator/(_rhs: AntiConstraintViolationPrefixOrPostfix) -> Scalar {
        return this.anti_constraint_violation();
    }
}
public extension AntiConstraintViolationPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: Line) -> Line.Output {
        return rhs.anti_constraint_violation();
    }
}
public extension Line: AntiConstraintViolation {
    public typedef Scalar AntiConstraintViolationOutput;
    public func anti_constraint_violation() -> Scalar {
        Line anti_reverse = Line.from_groups(
            /* e41, e42, e43 */
            this.group0.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            this.group1.xyz * float3(-1.0)
        );
        return Scalar.from_groups(
            /* scalar */
            -(anti_reverse.e41 * this.e23) - (anti_reverse.e42 * this.e31) - (anti_reverse.e43 * this.e12) - (anti_reverse.e23 * this.e41) - (anti_reverse.e31 * this.e42) - (anti_reverse.e12 * this.e43)
        );
    }
}
public extension Motor {
    // Fancy postfix trick
    func operator/(_rhs: AntiConstraintViolationPrefixOrPostfix) -> Scalar {
        return this.anti_constraint_violation();
    }
}
public extension AntiConstraintViolationPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: Motor) -> Motor.Output {
        return rhs.anti_constraint_violation();
    }
}
public extension Motor: AntiConstraintViolation {
    public typedef Scalar AntiConstraintViolationOutput;
    public func anti_constraint_violation() -> Scalar {
        Motor anti_reverse = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            this.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            this.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Scalar.from_groups(
            /* scalar */
            (anti_reverse.e1234 * this.scalar) + (anti_reverse.scalar * this.e1234) - (anti_reverse.e41 * this.e23) - (anti_reverse.e42 * this.e31) - (anti_reverse.e43 * this.e12) - (anti_reverse.e23 * this.e41) - (anti_reverse.e31 * this.e42) - (anti_reverse.e12 * this.e43)
        );
    }
}
public extension MultiVector {
    // Fancy postfix trick
    func operator/(_rhs: AntiConstraintViolationPrefixOrPostfix) -> MultiVector {
        return this.anti_constraint_violation();
    }
    // Fancy postfix self-assign
    public func operator/=(const AntiConstraintViolationPrefixOrPostfix& _rhs) -> MultiVector& {
        this = this.anti_constraint_violation();
        return *this;
    }
}
public extension AntiConstraintViolationPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: MultiVector) -> MultiVector.Output {
        return rhs.anti_constraint_violation();
    }
}
public extension MultiVector: AntiConstraintViolation {
    public typedef MultiVector AntiConstraintViolationOutput;
    public func anti_constraint_violation() -> MultiVector {
        MultiVector anti_reverse = MultiVector.from_groups(
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
        MultiVector geometric_anti_product = MultiVector.from_groups(
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
        return MultiVector.from_groups(
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

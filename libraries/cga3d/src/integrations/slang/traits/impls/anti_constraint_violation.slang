extension AntiCircleRotor {
    // Fancy infix trick
    func operator/(_rhs: AntiConstraintViolationPrefixOrPostfix) -> VersorEven {
        return this.anti_constraint_violation();
    }
}
extension AntiCircleRotor: AntiConstraintViolation {
    associatedtype Output = VersorEven;
    func anti_constraint_violation() -> Output {
        AntiCircleRotor anti_reverse = AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(this.group0.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, e45 */
            this.group1 * float4(-1.0), 
            /* e15, e25, e35, scalar */
            this.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        VersorEven geometric_anti_product = VersorEven.from_groups(
            /* e423, e431, e412, e12345 */
            float4(float3(0.0), (anti_reverse.e41 * this.e15) + (anti_reverse.e42 * this.e25) + (anti_reverse.e43 * this.e35) + (anti_reverse.e23 * this.e23) + (anti_reverse.e31 * this.e31) + (anti_reverse.e12 * this.e12) + (anti_reverse.e15 * this.e41) + (anti_reverse.e25 * this.e42) + (anti_reverse.e35 * this.e43) - (anti_reverse.e45 * this.e45) - (anti_reverse.scalar * this.scalar)), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e235, e315, e125, e5 */
            float4(float3(0.0), (anti_reverse.e23 * this.e15) + (anti_reverse.e31 * this.e25) + (anti_reverse.e12 * this.e35) + (anti_reverse.e15 * this.e23) + (anti_reverse.e25 * this.e31) + (anti_reverse.e35 * this.e12)), 
            /* e1, e2, e3, e4 */
            float4(-(anti_reverse.e43 * this.e25) - (anti_reverse.e25 * this.e43), -(anti_reverse.e41 * this.e35) - (anti_reverse.e35 * this.e41), -(anti_reverse.e42 * this.e15) - (anti_reverse.e15 * this.e42), (anti_reverse.e31 * this.e42) + (anti_reverse.e12 * this.e43)) + (this.group1.wwwz * float4(anti_reverse.group1.xyz, anti_reverse.e43)) + float4((anti_reverse.group0.xyz.yzx * this.group2.zxy), anti_reverse.e41 * this.e23) + float4((this.group0.xyz.yzx * anti_reverse.group2.zxy), anti_reverse.e42 * this.e31) + float4((anti_reverse.group1.www * this.group1.xyz), anti_reverse.e23 * this.e41)
        );
        return VersorEven.from_groups(
            /* e423, e431, e412, e12345 */
            geometric_anti_product.group0 + float4(float3(0.0), 2.0*(this.e41 * this.e15) + 2.0 * (this.e42 * this.e25) + 2.0 * (this.e43 * this.e35) + powi(this.e23, 2) + powi(this.e31, 2) + powi(this.e12, 2) + powi(this.scalar, 2) - powi(this.e45, 2)), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e235, e315, e125, e5 */
            geometric_anti_product.group2, 
            /* e1, e2, e3, e4 */
            geometric_anti_product.group3
        );
    }
}
extension AntiDipoleInversion {
    // Fancy infix trick
    func operator/(_rhs: AntiConstraintViolationPrefixOrPostfix) -> VersorEven {
        return this.anti_constraint_violation();
    }
}
extension AntiDipoleInversion: AntiConstraintViolation {
    associatedtype Output = VersorEven;
    func anti_constraint_violation() -> Output {
        AntiDipoleInversion anti_reverse = AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(this.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            this.group1 * float4(-1.0), 
            /* e235, e315, e125, e4 */
            this.group2 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            this.group3
        );
        VersorEven geometric_anti_product = VersorEven.from_groups(
            /* e423, e431, e412, e12345 */
            float4(float3(0.0), (anti_reverse.e321 * this.e321) + (anti_reverse.e4 * this.e5) + (anti_reverse.e5 * this.e4) - (anti_reverse.e423 * this.e235) - (anti_reverse.e431 * this.e315) - (anti_reverse.e412 * this.e125) - (anti_reverse.e415 * this.e415) - (anti_reverse.e425 * this.e425) - (anti_reverse.e435 * this.e435) - (anti_reverse.e235 * this.e423) - (anti_reverse.e315 * this.e431) - (anti_reverse.e125 * this.e412) - (anti_reverse.e1 * this.e1) - (anti_reverse.e2 * this.e2) - (anti_reverse.e3 * this.e3)), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e235, e315, e125, e5 */
            float4(float3(0.0), (anti_reverse.e1 * this.e235) + (anti_reverse.e2 * this.e315) + (anti_reverse.e3 * this.e125) + (anti_reverse.e5 * this.e321) - (anti_reverse.e415 * this.e235) - (anti_reverse.e425 * this.e315) - (anti_reverse.e435 * this.e125) - (anti_reverse.e321 * this.e5) - (anti_reverse.e235 * this.e415) - (anti_reverse.e235 * this.e1) - (anti_reverse.e315 * this.e425) - (anti_reverse.e315 * this.e2) - (anti_reverse.e125 * this.e435) - (anti_reverse.e125 * this.e3)), 
            /* e1, e2, e3, e4 */
            float4((anti_reverse.e435 * this.e2) + (anti_reverse.e321 * this.e415) + (anti_reverse.e4 * this.e235) + (anti_reverse.e3 * this.e425), (anti_reverse.e425 * this.e321) + (anti_reverse.e321 * this.e425) + (anti_reverse.e4 * this.e315) + (anti_reverse.e1 * this.e435), (anti_reverse.e435 * this.e321) + (anti_reverse.e321 * this.e435) + (anti_reverse.e4 * this.e125) + (anti_reverse.e2 * this.e415), -(anti_reverse.e435 * this.e412) - (anti_reverse.e4 * this.e321) - (anti_reverse.e2 * this.e431) - (anti_reverse.e3 * this.e412)) + (float4(this.e321, this.e3, this.e1, this.e4) * anti_reverse.group1.xxyw) + (float4(this.e315, this.e5, this.e5, this.e2) * float4(anti_reverse.group0.xyz.zyz, anti_reverse.e431)) + (float4(this.e5, this.e125, this.e235, this.e1) * float4(anti_reverse.group0.xyz.xxy, anti_reverse.e423)) + float4((this.group0.xyz.zxy * anti_reverse.group2.yzx), anti_reverse.e412 * this.e3) - (float4(anti_reverse.group2.zx, anti_reverse.e5, this.e435) * float4(this.group0.xyz.yzz, anti_reverse.e412)) - (float4(anti_reverse.group3.ww, anti_reverse.e315, this.e425) * float4(this.group0.xyz.xyx, anti_reverse.e431)) - float4((anti_reverse.group0.xyz.yzx * this.group2.zxy), anti_reverse.e423 * this.e415) - float4((anti_reverse.group1.yzx * this.group3.zxy), anti_reverse.e415 * this.e423) - float4((anti_reverse.group3.yzx * this.group1.zxy), anti_reverse.e425 * this.e431) - float4((this.group2.www * anti_reverse.group2.xyz), anti_reverse.e1 * this.e423)
        );
        return VersorEven.from_groups(
            /* e423, e431, e412, e12345 */
            geometric_anti_product.group0 + float4(float3(0.0), powi(this.e321, 2) + powi(this.e1, 2) + powi(this.e2, 2) + powi(this.e3, 2) - powi(this.e415, 2) - powi(this.e425, 2) - powi(this.e435, 2) - 2.0 * (this.e423 * this.e235) - 2.0 * (this.e431 * this.e315) - 2.0 * (this.e412 * this.e125) - 2.0 * (this.e4 * this.e5)), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e235, e315, e125, e5 */
            geometric_anti_product.group2, 
            /* e1, e2, e3, e4 */
            geometric_anti_product.group3
        );
    }
}
extension AntiDualNum {
    // Fancy infix trick
    func operator/(_rhs: AntiConstraintViolationPrefixOrPostfix) -> DualNum {
        return this.anti_constraint_violation();
    }
}
extension AntiDualNum: AntiConstraintViolation {
    associatedtype Output = DualNum;
    func anti_constraint_violation() -> Output {
        return DualNum.from_groups(
            /* e5, e12345, 0, 0 */
            float4(float2(this.e3215 * this.scalar * -2.0, 0.0), 0.0, 0.0)
        );
    }
}
extension AntiFlector {
    // Fancy infix trick
    func operator/(_rhs: AntiConstraintViolationPrefixOrPostfix) -> DualNum {
        return this.anti_constraint_violation();
    }
}
extension AntiFlector: AntiConstraintViolation {
    associatedtype Output = DualNum;
    func anti_constraint_violation() -> Output {
        AntiFlector anti_reverse = AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            this.group0 * float4(-1.0), 
            /* e1, e2, e3, e5 */
            this.group1
        );
        return DualNum.from_groups(
            /* e5, e12345, 0, 0 */
            float4(float2((anti_reverse.e1 * this.e235) + (anti_reverse.e2 * this.e315) + (anti_reverse.e3 * this.e125) + (anti_reverse.e5 * this.e321) - (anti_reverse.e235 * this.e1) - (anti_reverse.e315 * this.e2) - (anti_reverse.e125 * this.e3) - (anti_reverse.e321 * this.e5), 0.0), 0.0, 0.0)
        );
    }
}
extension AntiLine {
    // Fancy infix trick
    func operator/(_rhs: AntiConstraintViolationPrefixOrPostfix) -> DualNum {
        return this.anti_constraint_violation();
    }
}
extension AntiLine: AntiConstraintViolation {
    associatedtype Output = DualNum;
    func anti_constraint_violation() -> Output {
        AntiLine anti_reverse = AntiLine.from_groups(
            /* e23, e31, e12, 0 */
            float4(this.group0.xyz * float3(-1.0), 0.0), 
            /* e15, e25, e35, 0 */
            float4(this.group1.xyz * float3(-1.0), 0.0)
        );
        return DualNum.from_groups(
            /* e5, e12345, 0, 0 */
            float4(float2((anti_reverse.e23 * this.e15) + (anti_reverse.e31 * this.e25) + (anti_reverse.e12 * this.e35) + (anti_reverse.e15 * this.e23) + (anti_reverse.e25 * this.e31) + (anti_reverse.e35 * this.e12), 0.0), 0.0, 0.0)
        );
    }
}
extension AntiMotor {
    // Fancy infix trick
    func operator/(_rhs: AntiConstraintViolationPrefixOrPostfix) -> DualNum {
        return this.anti_constraint_violation();
    }
}
extension AntiMotor: AntiConstraintViolation {
    associatedtype Output = DualNum;
    func anti_constraint_violation() -> Output {
        AntiMotor anti_reverse = AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            this.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e15, e25, e35, e3215 */
            this.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return DualNum.from_groups(
            /* e5, e12345, 0, 0 */
            float4(float2((anti_reverse.e23 * this.e15) + (anti_reverse.e31 * this.e25) + (anti_reverse.e12 * this.e35) + (anti_reverse.e15 * this.e23) + (anti_reverse.e25 * this.e31) + (anti_reverse.e35 * this.e12) - (anti_reverse.scalar * this.e3215) - (anti_reverse.e3215 * this.scalar), 0.0), 0.0, 0.0)
        );
    }
}
extension Circle {
    // Fancy infix trick
    func operator/(_rhs: AntiConstraintViolationPrefixOrPostfix) -> VersorEven {
        return this.anti_constraint_violation();
    }
}
extension Circle: AntiConstraintViolation {
    associatedtype Output = VersorEven;
    func anti_constraint_violation() -> Output {
        Circle anti_reverse = Circle.from_groups(
            /* e423, e431, e412, 0 */
            float4(this.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            this.group1 * float4(-1.0), 
            /* e235, e315, e125, 0 */
            float4(this.group2.xyz * float3(-1.0), 0.0)
        );
        VersorEven geometric_anti_product = VersorEven.from_groups(
            /* e423, e431, e412, e12345 */
            float4(float3(0.0), (anti_reverse.e321 * this.e321) - (anti_reverse.e423 * this.e235) - (anti_reverse.e431 * this.e315) - (anti_reverse.e412 * this.e125) - (anti_reverse.e415 * this.e415) - (anti_reverse.e425 * this.e425) - (anti_reverse.e435 * this.e435) - (anti_reverse.e235 * this.e423) - (anti_reverse.e315 * this.e431) - (anti_reverse.e125 * this.e412)), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e235, e315, e125, e5 */
            float4(float3(0.0), -(anti_reverse.e415 * this.e235) - (anti_reverse.e425 * this.e315) - (anti_reverse.e435 * this.e125) - (anti_reverse.e235 * this.e415) - (anti_reverse.e315 * this.e425) - (anti_reverse.e125 * this.e435)), 
            /* e1, e2, e3, e4 */
            float4((anti_reverse.e412 * this.e315) + (anti_reverse.e415 * this.e321) + (anti_reverse.e321 * this.e415) + (anti_reverse.e315 * this.e412), (anti_reverse.e423 * this.e125) + (anti_reverse.e425 * this.e321) + (anti_reverse.e321 * this.e425) + (anti_reverse.e125 * this.e423), (anti_reverse.e431 * this.e235) + (anti_reverse.e435 * this.e321) + (anti_reverse.e321 * this.e435) + (anti_reverse.e235 * this.e431), -(anti_reverse.e412 * this.e435) - (anti_reverse.e415 * this.e423) - (anti_reverse.e425 * this.e431) - (anti_reverse.e435 * this.e412)) - float4((anti_reverse.group0.xyz.yzx * this.group2.xyz.zxy), anti_reverse.e423 * this.e415) - float4((anti_reverse.group2.xyz.zxy * this.group0.xyz.yzx), anti_reverse.e431 * this.e425)
        );
        return VersorEven.from_groups(
            /* e423, e431, e412, e12345 */
            geometric_anti_product.group0 + float4(float3(0.0), powi(this.e321, 2) - powi(this.e415, 2) - powi(this.e425, 2) - powi(this.e435, 2) - 2.0 * (this.e423 * this.e235) - 2.0 * (this.e431 * this.e315) - 2.0 * (this.e412 * this.e125)), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e235, e315, e125, e5 */
            geometric_anti_product.group2, 
            /* e1, e2, e3, e4 */
            geometric_anti_product.group3
        );
    }
}
extension CircleRotor {
    // Fancy infix trick
    func operator/(_rhs: AntiConstraintViolationPrefixOrPostfix) -> VersorEven {
        return this.anti_constraint_violation();
    }
}
extension CircleRotor: AntiConstraintViolation {
    associatedtype Output = VersorEven;
    func anti_constraint_violation() -> Output {
        CircleRotor anti_reverse = CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(this.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            this.group1 * float4(-1.0), 
            /* e235, e315, e125, e12345 */
            this.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        VersorEven geometric_anti_product = VersorEven.from_groups(
            /* e423, e431, e412, e12345 */
            float4(float3(0.0), (anti_reverse.e321 * this.e321) + (anti_reverse.e12345 * this.e12345) - (anti_reverse.e423 * this.e235) - (anti_reverse.e431 * this.e315) - (anti_reverse.e412 * this.e125) - (anti_reverse.e415 * this.e415) - (anti_reverse.e425 * this.e425) - (anti_reverse.e435 * this.e435) - (anti_reverse.e235 * this.e423) - (anti_reverse.e315 * this.e431) - (anti_reverse.e125 * this.e412)), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e235, e315, e125, e5 */
            float4(float3(0.0), -(anti_reverse.e415 * this.e235) - (anti_reverse.e425 * this.e315) - (anti_reverse.e435 * this.e125) - (anti_reverse.e235 * this.e415) - (anti_reverse.e315 * this.e425) - (anti_reverse.e125 * this.e435)), 
            /* e1, e2, e3, e4 */
            float4((anti_reverse.e412 * this.e315) + (anti_reverse.e415 * this.e321) + (anti_reverse.e321 * this.e415) + (anti_reverse.e315 * this.e412), (anti_reverse.e423 * this.e125) + (anti_reverse.e425 * this.e321) + (anti_reverse.e321 * this.e425) + (anti_reverse.e125 * this.e423), (anti_reverse.e431 * this.e235) + (anti_reverse.e435 * this.e321) + (anti_reverse.e321 * this.e435) + (anti_reverse.e235 * this.e431), -(anti_reverse.e412 * this.e435) - (anti_reverse.e415 * this.e423) - (anti_reverse.e425 * this.e431) - (anti_reverse.e435 * this.e412)) - float4((anti_reverse.group0.xyz.yzx * this.group2.zxy), anti_reverse.e423 * this.e415) - float4((this.group0.xyz.yzx * anti_reverse.group2.zxy), anti_reverse.e431 * this.e425)
        );
        return VersorEven.from_groups(
            /* e423, e431, e412, e12345 */
            geometric_anti_product.group0 + float4(float3(0.0), powi(this.e321, 2) - powi(this.e415, 2) - powi(this.e425, 2) - powi(this.e435, 2) - powi(this.e12345, 2) - 2.0 * (this.e423 * this.e235) - 2.0 * (this.e431 * this.e315) - 2.0 * (this.e412 * this.e125)), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e235, e315, e125, e5 */
            geometric_anti_product.group2, 
            /* e1, e2, e3, e4 */
            geometric_anti_product.group3
        );
    }
}
extension Dipole {
    // Fancy infix trick
    func operator/(_rhs: AntiConstraintViolationPrefixOrPostfix) -> VersorEven {
        return this.anti_constraint_violation();
    }
}
extension Dipole: AntiConstraintViolation {
    associatedtype Output = VersorEven;
    func anti_constraint_violation() -> Output {
        Dipole anti_reverse = Dipole.from_groups(
            /* e41, e42, e43, 0 */
            float4(this.group0.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, e45 */
            this.group1 * float4(-1.0), 
            /* e15, e25, e35, 0 */
            float4(this.group2.xyz * float3(-1.0), 0.0)
        );
        VersorEven geometric_anti_product = VersorEven.from_groups(
            /* e423, e431, e412, e12345 */
            float4(float3(0.0), (anti_reverse.e41 * this.e15) + (anti_reverse.e42 * this.e25) + (anti_reverse.e43 * this.e35) + (anti_reverse.e23 * this.e23) + (anti_reverse.e31 * this.e31) + (anti_reverse.e12 * this.e12) + (anti_reverse.e15 * this.e41) + (anti_reverse.e25 * this.e42) + (anti_reverse.e35 * this.e43) - (anti_reverse.e45 * this.e45)), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e235, e315, e125, e5 */
            float4(float3(0.0), (anti_reverse.e23 * this.e15) + (anti_reverse.e31 * this.e25) + (anti_reverse.e12 * this.e35) + (anti_reverse.e15 * this.e23) + (anti_reverse.e25 * this.e31) + (anti_reverse.e35 * this.e12)), 
            /* e1, e2, e3, e4 */
            float4(-(anti_reverse.e43 * this.e25) - (anti_reverse.e25 * this.e43), -(anti_reverse.e41 * this.e35) - (anti_reverse.e35 * this.e41), -(anti_reverse.e42 * this.e15) - (anti_reverse.e15 * this.e42), (anti_reverse.e31 * this.e42) + (anti_reverse.e12 * this.e43)) + (this.group1.wwwz * float4(anti_reverse.group1.xyz, anti_reverse.e43)) + float4((anti_reverse.group0.xyz.yzx * this.group2.xyz.zxy), anti_reverse.e41 * this.e23) + float4((anti_reverse.group2.xyz.zxy * this.group0.xyz.yzx), anti_reverse.e42 * this.e31) + float4((anti_reverse.group1.www * this.group1.xyz), anti_reverse.e23 * this.e41)
        );
        return VersorEven.from_groups(
            /* e423, e431, e412, e12345 */
            geometric_anti_product.group0 + float4(float3(0.0), 2.0*(this.e41 * this.e15) + 2.0 * (this.e42 * this.e25) + 2.0 * (this.e43 * this.e35) + powi(this.e23, 2) + powi(this.e31, 2) + powi(this.e12, 2) - powi(this.e45, 2)), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e235, e315, e125, e5 */
            geometric_anti_product.group2, 
            /* e1, e2, e3, e4 */
            geometric_anti_product.group3
        );
    }
}
extension DipoleInversion {
    // Fancy infix trick
    func operator/(_rhs: AntiConstraintViolationPrefixOrPostfix) -> VersorEven {
        return this.anti_constraint_violation();
    }
}
extension DipoleInversion: AntiConstraintViolation {
    associatedtype Output = VersorEven;
    func anti_constraint_violation() -> Output {
        DipoleInversion anti_reverse = DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(this.group0.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, e45 */
            this.group1 * float4(-1.0), 
            /* e15, e25, e35, e1234 */
            this.group2 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group3
        );
        VersorEven geometric_anti_product = VersorEven.from_groups(
            /* e423, e431, e412, e12345 */
            float4(float3(0.0), (anti_reverse.e41 * this.e15) + (anti_reverse.e42 * this.e25) + (anti_reverse.e43 * this.e35) + (anti_reverse.e23 * this.e23) + (anti_reverse.e31 * this.e31) + (anti_reverse.e12 * this.e12) + (anti_reverse.e15 * this.e41) + (anti_reverse.e25 * this.e42) + (anti_reverse.e35 * this.e43) + (anti_reverse.e4235 * this.e4235) + (anti_reverse.e4315 * this.e4315) + (anti_reverse.e4125 * this.e4125) - (anti_reverse.e45 * this.e45) - (anti_reverse.e1234 * this.e3215) - (anti_reverse.e3215 * this.e1234)), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e235, e315, e125, e5 */
            float4(float3(0.0), (anti_reverse.e23 * this.e15) + (anti_reverse.e31 * this.e25) + (anti_reverse.e12 * this.e35) + (anti_reverse.e15 * this.e23) + (anti_reverse.e25 * this.e31) + (anti_reverse.e35 * this.e12) + (anti_reverse.e4235 * this.e15) + (anti_reverse.e4315 * this.e25) + (anti_reverse.e4125 * this.e35) + (anti_reverse.e3215 * this.e45) - (anti_reverse.e45 * this.e3215) - (anti_reverse.e15 * this.e4235) - (anti_reverse.e25 * this.e4315) - (anti_reverse.e35 * this.e4125)), 
            /* e1, e2, e3, e4 */
            float4(-(anti_reverse.e1234 * this.e15) - (anti_reverse.e4315 * this.e12), -(anti_reverse.e1234 * this.e25) - (anti_reverse.e4125 * this.e23), -(anti_reverse.e1234 * this.e35) - (anti_reverse.e4235 * this.e31), (anti_reverse.e12 * this.e43) + (anti_reverse.e45 * this.e1234)) + (float4(this.e45, this.e4125, this.e4235, this.e4315) * float4(anti_reverse.group1.xxy, anti_reverse.e42)) + (float4(this.e4315, this.e45, this.e45, this.e12) * float4(anti_reverse.group1.zyz, anti_reverse.e43)) + (float4(anti_reverse.group2.zx, anti_reverse.e3215, this.e31) * float4(this.group0.xyz.yzz, anti_reverse.e42)) + (float4(anti_reverse.group3.ww, anti_reverse.e25, this.e4235) * float4(this.group0.xyz.xyx, anti_reverse.e41)) + float4((anti_reverse.group0.xyz.yzx * this.group2.zxy), anti_reverse.e41 * this.e23) + float4((anti_reverse.group1.www * this.group1.xyz), anti_reverse.e43 * this.e4125) + float4((anti_reverse.group3.zxy * this.group1.yzx), anti_reverse.e31 * this.e42) + float4((this.group2.www * anti_reverse.group2.xyz), anti_reverse.e23 * this.e41) - (float4(this.e25, this.e3215, this.e3215, anti_reverse.e4315) * float4(anti_reverse.group0.xyz.zyz, this.e42)) - (float4(this.e3215, this.e35, this.e15, anti_reverse.e4235) * float4(anti_reverse.group0.xyz.xxy, this.e41)) - float4((this.group0.xyz.zxy * anti_reverse.group2.yzx), anti_reverse.e4125 * this.e43) - float4((anti_reverse.group1.yzx * this.group3.zxy), anti_reverse.e1234 * this.e45)
        );
        return VersorEven.from_groups(
            /* e423, e431, e412, e12345 */
            geometric_anti_product.group0 + float4(float3(0.0), 2.0*(this.e41 * this.e15) + 2.0 * (this.e42 * this.e25) + 2.0 * (this.e43 * this.e35) + 2.0 * (this.e1234 * this.e3215) + powi(this.e23, 2) + powi(this.e31, 2) + powi(this.e12, 2) - powi(this.e45, 2) - powi(this.e4235, 2) - powi(this.e4315, 2) - powi(this.e4125, 2)), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e235, e315, e125, e5 */
            geometric_anti_product.group2, 
            /* e1, e2, e3, e4 */
            geometric_anti_product.group3
        );
    }
}
extension DualNum {
    // Fancy infix trick
    func operator/(_rhs: AntiConstraintViolationPrefixOrPostfix) -> DualNum {
        return this.anti_constraint_violation();
    }
    func operator/=(const AntiConstraintViolationPrefixOrPostfix& _rhs) -> DualNum& {
        this = this.anti_constraint_violation();
        return *this;
    }
}
extension DualNum: AntiConstraintViolation {
    associatedtype Output = DualNum;
    func anti_constraint_violation() -> Output {
        return DualNum.from_groups(
            /* e5, e12345, 0, 0 */
            float4(float2(this.e5 * this.e12345 * 2.0, 0.0), 0.0, 0.0)
        );
    }
}
extension Flector {
    // Fancy infix trick
    func operator/(_rhs: AntiConstraintViolationPrefixOrPostfix) -> DualNum {
        return this.anti_constraint_violation();
    }
}
extension Flector: AntiConstraintViolation {
    associatedtype Output = DualNum;
    func anti_constraint_violation() -> Output {
        Flector anti_reverse = Flector.from_groups(
            /* e15, e25, e35, e45 */
            this.group0 * float4(-1.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group1
        );
        return DualNum.from_groups(
            /* e5, e12345, 0, 0 */
            float4(float2((anti_reverse.e4235 * this.e15) + (anti_reverse.e4315 * this.e25) + (anti_reverse.e4125 * this.e35) + (anti_reverse.e3215 * this.e45) - (anti_reverse.e15 * this.e4235) - (anti_reverse.e25 * this.e4315) - (anti_reverse.e35 * this.e4125) - (anti_reverse.e45 * this.e3215), 0.0), 0.0, 0.0)
        );
    }
}
extension Line {
    // Fancy infix trick
    func operator/(_rhs: AntiConstraintViolationPrefixOrPostfix) -> DualNum {
        return this.anti_constraint_violation();
    }
}
extension Line: AntiConstraintViolation {
    associatedtype Output = DualNum;
    func anti_constraint_violation() -> Output {
        Line anti_reverse = Line.from_groups(
            /* e415, e425, e435, 0 */
            float4(this.group0.xyz * float3(-1.0), 0.0), 
            /* e235, e315, e125, 0 */
            float4(this.group1.xyz * float3(-1.0), 0.0)
        );
        return DualNum.from_groups(
            /* e5, e12345, 0, 0 */
            float4(float2(-(anti_reverse.e415 * this.e235) - (anti_reverse.e425 * this.e315) - (anti_reverse.e435 * this.e125) - (anti_reverse.e235 * this.e415) - (anti_reverse.e315 * this.e425) - (anti_reverse.e125 * this.e435), 0.0), 0.0, 0.0)
        );
    }
}
extension Motor {
    // Fancy infix trick
    func operator/(_rhs: AntiConstraintViolationPrefixOrPostfix) -> DualNum {
        return this.anti_constraint_violation();
    }
}
extension Motor: AntiConstraintViolation {
    associatedtype Output = DualNum;
    func anti_constraint_violation() -> Output {
        Motor anti_reverse = Motor.from_groups(
            /* e415, e425, e435, e12345 */
            this.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            this.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return DualNum.from_groups(
            /* e5, e12345, 0, 0 */
            float4(float2((anti_reverse.e12345 * this.e5) + (anti_reverse.e5 * this.e12345) - (anti_reverse.e415 * this.e235) - (anti_reverse.e425 * this.e315) - (anti_reverse.e435 * this.e125) - (anti_reverse.e235 * this.e415) - (anti_reverse.e315 * this.e425) - (anti_reverse.e125 * this.e435), 0.0), 0.0, 0.0)
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
            /* scalar, e12345, 0, 0 */
            float4(this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e5, 0, 0, 0 */
            float4(this.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            this.group3 * float4(-1.0), 
            /* e41, e42, e43, 0 */
            float4(this.group4.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group5.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            this.group6 * float4(-1.0), 
            /* e423, e431, e412, 0 */
            float4(this.group7.xyz * float3(-1.0), 0.0), 
            /* e235, e315, e125, 0 */
            float4(this.group8.xyz * float3(-1.0), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group9, 
            /* e1234, 0, 0, 0 */
            float4(this.e1234, 0.0, 0.0, 0.0)
        );
        MultiVector geometric_anti_product = MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2((anti_reverse.e12345 * this.scalar) + (anti_reverse.e5 * this.e1234) + (anti_reverse.e4315 * this.e2) + (anti_reverse.e4125 * this.e3) + (anti_reverse.e3215 * this.e4) + (anti_reverse.e1234 * this.e5) - (anti_reverse.e15 * this.e423) - (anti_reverse.e25 * this.e431) - (anti_reverse.e35 * this.e412) - (anti_reverse.e45 * this.e321) - (anti_reverse.e415 * this.e23) - (anti_reverse.e425 * this.e31) - (anti_reverse.e435 * this.e12) - (anti_reverse.e321 * this.e45) - (anti_reverse.e431 * this.e25) - (anti_reverse.e412 * this.e35), (anti_reverse.e15 * this.e41) + (anti_reverse.e25 * this.e42) + (anti_reverse.e35 * this.e43) + (anti_reverse.e41 * this.e15) + (anti_reverse.e42 * this.e25) + (anti_reverse.e43 * this.e35) + (anti_reverse.e23 * this.e23) + (anti_reverse.e31 * this.e31) + (anti_reverse.e12 * this.e12) + (anti_reverse.e321 * this.e321) - (anti_reverse.scalar * this.scalar) - (anti_reverse.e1 * this.e1) - (anti_reverse.e2 * this.e2) - (anti_reverse.e3 * this.e3) - (anti_reverse.e3215 * this.e1234) - (anti_reverse.e1234 * this.e3215)) + (float2(this.e12345) * anti_reverse.group0.xy) + (float2(anti_reverse.e2, anti_reverse.e4235) * this.group9.yx) + (float2(anti_reverse.e3, anti_reverse.e4315) * this.group9.zy) + (float2(anti_reverse.e4, anti_reverse.e4125) * this.group9.wz) + (float2(anti_reverse.e4235, anti_reverse.e5) * this.group1.xw) + (float2(this.e4235, this.e5) * anti_reverse.group1.xw) - (float2(anti_reverse.e235) * float2(this.e41, this.e423)) - (float2(anti_reverse.e315) * float2(this.e42, this.e431)) - (float2(anti_reverse.e125) * float2(this.e43, this.e412)) - (float2(this.e415) * float2(anti_reverse.e23, anti_reverse.e415)) - (float2(this.e425) * float2(anti_reverse.e31, anti_reverse.e425)) - (float2(this.e435) * float2(anti_reverse.e12, anti_reverse.e435)) - (float2(this.e235) * float2(anti_reverse.e41, anti_reverse.e423)) - (float2(this.e315) * float2(anti_reverse.e42, anti_reverse.e431)) - (float2(this.e125) * float2(anti_reverse.e43, anti_reverse.e412)) - (float2(anti_reverse.e423, anti_reverse.e45) * this.group3.xw), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((anti_reverse.e15 * this.e1234) + (anti_reverse.e415 * this.e321) + (anti_reverse.e435 * this.e2) + (anti_reverse.e321 * this.e415), (anti_reverse.e25 * this.e1234) + (anti_reverse.e415 * this.e3) + (anti_reverse.e425 * this.e321) + (anti_reverse.e321 * this.e425), (anti_reverse.e35 * this.e1234) + (anti_reverse.e425 * this.e1) + (anti_reverse.e435 * this.e321) + (anti_reverse.e321 * this.e435), -(anti_reverse.e3 * this.e412) - (anti_reverse.e4 * this.e321) - (anti_reverse.e435 * this.e412) - (anti_reverse.e1234 * this.e45)) + (float4(this.e5, this.e125, this.e235, this.e4125) * float4(anti_reverse.group7.xyz.xxy, anti_reverse.e43)) + (float4(this.e45, this.e4125, this.e4235, this.e4315) * float4(anti_reverse.group5.xyz.xxy, anti_reverse.e42)) + (float4(this.e315, this.e5, this.e5, this.e41) * float4(anti_reverse.group7.xyz.zyz, anti_reverse.e23)) + (float4(this.e4315, this.e45, this.e45, this.e12) * float4(anti_reverse.group5.xyz.zyz, anti_reverse.e43)) + (float4(anti_reverse.group0.xy.xx, anti_reverse.scalar, anti_reverse.e12345) * float4(this.group9.xyz, this.e4)) + (float4(anti_reverse.group0.xy.yy, anti_reverse.e12345, this.e12345) * float4(this.group1.xyz, anti_reverse.e4)) + (float4(this.group0.xy.xx, this.scalar, anti_reverse.e41) * float4(anti_reverse.group9.xyz, this.e23)) + (float4(this.group0.xy.yy, this.e12345, anti_reverse.e41) * float4(anti_reverse.group1.xyz, this.e4235)) + (float4(anti_reverse.group3.zx, anti_reverse.e3215, this.e1) * float4(this.group4.xyz.yzz, anti_reverse.e423)) + (float4(anti_reverse.group3.ww, anti_reverse.e4315, this.e2) * float4(this.group5.xyz.xyx, anti_reverse.e431)) + (float4(anti_reverse.group9.zx, anti_reverse.e45, this.e3) * float4(this.group5.xyz.yzz, anti_reverse.e412)) + (float4(anti_reverse.group9.ww, anti_reverse.e25, this.e43) * float4(this.group4.xyz.xyx, anti_reverse.e12)) + float4((this.group8.xyz * anti_reverse.group1.www), anti_reverse.e45 * this.e1234) + float4((anti_reverse.group4.xyz.yzx * this.group3.zxy), anti_reverse.e42 * this.e31) + float4((anti_reverse.group8.xyz.yzx * this.group7.xyz.zxy), anti_reverse.e31 * this.e42) + float4((anti_reverse.group1.zxy * this.group6.yzx), anti_reverse.e321 * this.e4) - (float4(this.e4, this.e412, this.e423, this.e435) * float4(anti_reverse.group8.xyz.xxy, anti_reverse.e412)) - (float4(this.e25, this.e3215, this.e3215, anti_reverse.e1234) * float4(anti_reverse.group4.xyz.zyz, this.scalar)) - (float4(this.e431, this.e4, this.e4, anti_reverse.e4235) * float4(anti_reverse.group8.xyz.zyz, this.e41)) - (float4(this.e3215, this.e35, this.e15, this.e1234) * float4(anti_reverse.group4.xyz.xxy, anti_reverse.scalar)) - (anti_reverse.group9.yzxz * float4(this.group5.xyz.zxy, this.e43)) - float4((float3(anti_reverse.e5) * this.group7.xyz), anti_reverse.e1 * this.e423) - float4((float3(anti_reverse.e1234) * this.group3.xyz), anti_reverse.e425 * this.e431) - float4((anti_reverse.group5.xyz.yzx * this.group9.zxy), anti_reverse.e423 * this.e415) - float4((anti_reverse.group7.xyz.yzx * this.group8.xyz.zxy), anti_reverse.e431 * this.e425) - float4((this.group4.xyz.zxy * anti_reverse.group3.yzx), anti_reverse.e4315 * this.e42) - float4((anti_reverse.group1.yzx * this.group6.zxy), anti_reverse.e415 * this.e423) - float4((anti_reverse.group6.yzx * this.group1.zxy), anti_reverse.e2 * this.e431), 
            /* e5, 0, 0, 0 */
            float4((anti_reverse.e12345 * this.e5) + (anti_reverse.e1 * this.e235) + (anti_reverse.e2 * this.e315) + (anti_reverse.e3 * this.e125) + (anti_reverse.e5 * this.e12345) + (anti_reverse.e5 * this.e321) + (anti_reverse.e15 * this.e23) + (anti_reverse.e25 * this.e31) + (anti_reverse.e35 * this.e12) + (anti_reverse.e23 * this.e15) + (anti_reverse.e31 * this.e25) + (anti_reverse.e12 * this.e35) + (anti_reverse.e4235 * this.e15) + (anti_reverse.e4315 * this.e25) + (anti_reverse.e4125 * this.e35) + (anti_reverse.e3215 * this.e45) - (anti_reverse.scalar * this.e3215) - (anti_reverse.e15 * this.e4235) - (anti_reverse.e25 * this.e4315) - (anti_reverse.e35 * this.e4125) - (anti_reverse.e45 * this.e3215) - (anti_reverse.e415 * this.e235) - (anti_reverse.e425 * this.e315) - (anti_reverse.e435 * this.e125) - (anti_reverse.e321 * this.e5) - (anti_reverse.e235 * this.e1) - (anti_reverse.e235 * this.e415) - (anti_reverse.e315 * this.e2) - (anti_reverse.e315 * this.e425) - (anti_reverse.e125 * this.e3) - (anti_reverse.e125 * this.e435) - (anti_reverse.e3215 * this.scalar), 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(0.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e235, e315, e125, 0 */
            float4(0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4((anti_reverse.e45 * this.e415) + (anti_reverse.e415 * this.e45) + (anti_reverse.e435 * this.e4315) + (anti_reverse.e4125 * this.e425), (anti_reverse.e45 * this.e425) + (anti_reverse.e415 * this.e4125) + (anti_reverse.e425 * this.e45) + (anti_reverse.e4235 * this.e435), (anti_reverse.e45 * this.e435) + (anti_reverse.e425 * this.e4235) + (anti_reverse.e435 * this.e45) + (anti_reverse.e4315 * this.e415), -(anti_reverse.e5 * this.e45) - (anti_reverse.e425 * this.e25) - (anti_reverse.e435 * this.e35) - (anti_reverse.e321 * this.e3215)) + (float4(this.e42, this.e1234, this.e1234, this.e4125) * float4(anti_reverse.group8.xyz.zyz, anti_reverse.e125)) + (float4(this.e1234, this.e43, this.e41, this.e4315) * float4(anti_reverse.group8.xyz.xxy, anti_reverse.e315)) + (float4(anti_reverse.group0.xy.yy, anti_reverse.e12345, anti_reverse.scalar) * float4(this.group9.xyz, this.e5)) + (float4(this.group0.xy.yy, this.e12345, anti_reverse.e12345) * float4(anti_reverse.group9.xyz, this.e3215)) + (float4(anti_reverse.group3.zx, anti_reverse.e3215, this.e5) * float4(this.group7.xyz.yzz, anti_reverse.e45)) + (float4(anti_reverse.group9.ww, anti_reverse.e25, this.e35) * float4(this.group7.xyz.xyx, anti_reverse.e3)) + float4((float3(anti_reverse.e5) * this.group4.xyz), anti_reverse.e1 * this.e15) + float4((anti_reverse.group4.xyz.yzx * this.group8.xyz.zxy), anti_reverse.e5 * this.scalar) + float4((anti_reverse.group5.xyz.yzx * this.group1.zxy), anti_reverse.e3215 * this.e12345) + float4((anti_reverse.group7.xyz.yzx * this.group3.zxy), anti_reverse.e235 * this.e4235) + float4((this.group5.xyz.zxy * anti_reverse.group1.yzx), anti_reverse.e2 * this.e25) + float4((this.group1.www * anti_reverse.group3.xyz), anti_reverse.e3215 * this.e321) - (float4(this.e2, this.e321, this.e321, this.e12) * float4(anti_reverse.group5.xyz.zyz, anti_reverse.e125)) - (float4(this.e5, this.e125, this.e235, this.e125) * float4(anti_reverse.group4.xyz.xxy, anti_reverse.e12)) - (float4(this.e25, this.e3215, this.e3215, anti_reverse.e4315) * float4(anti_reverse.group7.xyz.zyz, this.e315)) - (float4(this.e321, this.e3, this.e1, this.e31) * float4(anti_reverse.group5.xyz.xxy, anti_reverse.e315)) - (float4(this.e315, this.e5, this.e5, this.e23) * float4(anti_reverse.group4.xyz.zyz, anti_reverse.e235)) - (float4(this.e3215, this.e35, this.e15, anti_reverse.e4235) * float4(anti_reverse.group7.xyz.xxy, this.e235)) - (float4(anti_reverse.group0.xy.xx, anti_reverse.scalar, anti_reverse.e23) * float4(this.group1.xyz, this.e235)) - (float4(this.group0.xy.xx, this.scalar, anti_reverse.e31) * float4(anti_reverse.group1.xyz, this.e315)) - (float4(anti_reverse.group1.zx, anti_reverse.e321, this.e415) * float4(this.group5.xyz.yzz, anti_reverse.e15)) - (float4(anti_reverse.group6.ww, anti_reverse.e2, this.e1) * float4(this.group5.xyz.xyx, anti_reverse.e15)) - float4((float3(anti_reverse.e1234) * this.group8.xyz), anti_reverse.e25 * this.e425) - float4((anti_reverse.group8.xyz.yzx * this.group4.xyz.zxy), anti_reverse.e4125 * this.e125) - float4((this.group7.xyz.zxy * anti_reverse.group3.yzx), anti_reverse.e25 * this.e2) - float4((anti_reverse.group1.www * this.group3.xyz), anti_reverse.e35 * this.e3) - float4((anti_reverse.group6.yzx * this.group9.zxy), anti_reverse.e35 * this.e435) - float4((anti_reverse.group9.yzx * this.group6.zxy), anti_reverse.e415 * this.e15), 
            /* e1234, 0, 0, 0 */
            float4((anti_reverse.scalar * this.e4) + (anti_reverse.e12345 * this.e1234) + (anti_reverse.e4 * this.scalar) + (anti_reverse.e4 * this.e45) + (anti_reverse.e41 * this.e1) + (anti_reverse.e42 * this.e2) + (anti_reverse.e43 * this.e3) + (anti_reverse.e321 * this.e1234) + (anti_reverse.e4235 * this.e423) + (anti_reverse.e4315 * this.e431) + (anti_reverse.e4125 * this.e412) + (anti_reverse.e1234 * this.e12345) - (anti_reverse.e1 * this.e41) - (anti_reverse.e2 * this.e42) - (anti_reverse.e3 * this.e43) - (anti_reverse.e45 * this.e4) - (anti_reverse.e41 * this.e415) - (anti_reverse.e42 * this.e425) - (anti_reverse.e43 * this.e435) - (anti_reverse.e23 * this.e423) - (anti_reverse.e31 * this.e431) - (anti_reverse.e12 * this.e412) - (anti_reverse.e415 * this.e41) - (anti_reverse.e425 * this.e42) - (anti_reverse.e435 * this.e43) - (anti_reverse.e423 * this.e23) - (anti_reverse.e423 * this.e4235) - (anti_reverse.e431 * this.e31) - (anti_reverse.e431 * this.e4315) - (anti_reverse.e412 * this.e12) - (anti_reverse.e412 * this.e4125) - (anti_reverse.e1234 * this.e321), 0.0, 0.0, 0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(geometric_anti_product.scalar, 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            geometric_anti_product.group1, 
            /* e5, 0, 0, 0 */
            float4(geometric_anti_product.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(0.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e235, e315, e125, 0 */
            float4(0.0), 
            /* e4235, e4315, e4125, e3215 */
            geometric_anti_product.group9, 
            /* e1234, 0, 0, 0 */
            float4(geometric_anti_product.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension VersorEven {
    // Fancy infix trick
    func operator/(_rhs: AntiConstraintViolationPrefixOrPostfix) -> VersorEven {
        return this.anti_constraint_violation();
    }
    func operator/=(const AntiConstraintViolationPrefixOrPostfix& _rhs) -> VersorEven& {
        this = this.anti_constraint_violation();
        return *this;
    }
}
extension VersorEven: AntiConstraintViolation {
    associatedtype Output = VersorEven;
    func anti_constraint_violation() -> Output {
        VersorEven anti_reverse = VersorEven.from_groups(
            /* e423, e431, e412, e12345 */
            this.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e415, e425, e435, e321 */
            this.group1 * float4(-1.0), 
            /* e235, e315, e125, e5 */
            this.group2 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e4 */
            this.group3
        );
        VersorEven geometric_anti_product = VersorEven.from_groups(
            /* e423, e431, e412, e12345 */
            float4(float3(0.0), (anti_reverse.e12345 * this.e12345) + (anti_reverse.e321 * this.e321) + (anti_reverse.e5 * this.e4) + (anti_reverse.e4 * this.e5) - (anti_reverse.e423 * this.e235) - (anti_reverse.e431 * this.e315) - (anti_reverse.e412 * this.e125) - (anti_reverse.e415 * this.e415) - (anti_reverse.e425 * this.e425) - (anti_reverse.e435 * this.e435) - (anti_reverse.e235 * this.e423) - (anti_reverse.e315 * this.e431) - (anti_reverse.e125 * this.e412) - (anti_reverse.e1 * this.e1) - (anti_reverse.e2 * this.e2) - (anti_reverse.e3 * this.e3)), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e235, e315, e125, e5 */
            float4(float3(0.0), (anti_reverse.e12345 * this.e5) + (anti_reverse.e5 * this.e12345) + (anti_reverse.e5 * this.e321) + (anti_reverse.e1 * this.e235) + (anti_reverse.e2 * this.e315) + (anti_reverse.e3 * this.e125) - (anti_reverse.e415 * this.e235) - (anti_reverse.e425 * this.e315) - (anti_reverse.e435 * this.e125) - (anti_reverse.e321 * this.e5) - (anti_reverse.e235 * this.e415) - (anti_reverse.e235 * this.e1) - (anti_reverse.e315 * this.e425) - (anti_reverse.e315 * this.e2) - (anti_reverse.e125 * this.e435) - (anti_reverse.e125 * this.e3)), 
            /* e1, e2, e3, e4 */
            float4((anti_reverse.e315 * this.e412) + (anti_reverse.e1 * this.e12345) + (anti_reverse.e3 * this.e425) + (anti_reverse.e4 * this.e235), (anti_reverse.e125 * this.e423) + (anti_reverse.e1 * this.e435) + (anti_reverse.e2 * this.e12345) + (anti_reverse.e4 * this.e315), (anti_reverse.e235 * this.e431) + (anti_reverse.e2 * this.e415) + (anti_reverse.e3 * this.e12345) + (anti_reverse.e4 * this.e125), -(anti_reverse.e1 * this.e423) - (anti_reverse.e2 * this.e431) - (anti_reverse.e3 * this.e412) - (anti_reverse.e4 * this.e321)) + (float4(this.e321, this.e3, this.e1, this.e4) * float4(anti_reverse.group1.xxy, anti_reverse.e12345)) + (float4(this.e2, this.e321, this.e321, this.e4) * anti_reverse.group1.zyzw) + (anti_reverse.group0.xxyx * float4(this.group2.wzx, this.e1)) + (anti_reverse.group0.zyzy * float4(this.group2.yww, this.e2)) + (anti_reverse.group0.wwwz * this.group3.xyzz) + float4((anti_reverse.group1.www * this.group1.xyz), anti_reverse.e4 * this.e12345) - (float4(this.e431, this.e4, this.e4, this.e423) * float4(anti_reverse.group2.zyz, anti_reverse.e415)) - (float4(this.e4, this.e412, this.e423, this.e435) * float4(anti_reverse.group2.xxy, anti_reverse.e412)) - (anti_reverse.group0.yzxx * float4(this.group2.zxy, this.e415)) - (this.group0.xyzy * float4(anti_reverse.group2.www, anti_reverse.e425)) - float4((anti_reverse.group1.yzx * this.group3.zxy), anti_reverse.e431 * this.e425) - float4((anti_reverse.group3.yzx * this.group1.zxy), anti_reverse.e435 * this.e412)
        );
        return VersorEven.from_groups(
            /* e423, e431, e412, e12345 */
            geometric_anti_product.group0 + float4(float3(0.0), powi(this.e321, 2) + powi(this.e1, 2) + powi(this.e2, 2) + powi(this.e3, 2) - powi(this.e12345, 2) - powi(this.e415, 2) - powi(this.e425, 2) - powi(this.e435, 2) - 2.0 * (this.e423 * this.e235) - 2.0 * (this.e431 * this.e315) - 2.0 * (this.e412 * this.e125) - 2.0 * (this.e5 * this.e4)), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e235, e315, e125, e5 */
            geometric_anti_product.group2, 
            /* e1, e2, e3, e4 */
            geometric_anti_product.group3
        );
    }
}
extension VersorOdd {
    // Fancy infix trick
    func operator/(_rhs: AntiConstraintViolationPrefixOrPostfix) -> VersorEven {
        return this.anti_constraint_violation();
    }
}
extension VersorOdd: AntiConstraintViolation {
    associatedtype Output = VersorEven;
    func anti_constraint_violation() -> Output {
        VersorOdd anti_reverse = VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            this.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, e45 */
            this.group1 * float4(-1.0), 
            /* e15, e25, e35, e1234 */
            this.group2 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group3
        );
        VersorEven geometric_anti_product = VersorEven.from_groups(
            /* e423, e431, e412, e12345 */
            float4(float3(0.0), (anti_reverse.e41 * this.e15) + (anti_reverse.e42 * this.e25) + (anti_reverse.e43 * this.e35) + (anti_reverse.e23 * this.e23) + (anti_reverse.e31 * this.e31) + (anti_reverse.e12 * this.e12) + (anti_reverse.e15 * this.e41) + (anti_reverse.e25 * this.e42) + (anti_reverse.e35 * this.e43) + (anti_reverse.e4235 * this.e4235) + (anti_reverse.e4315 * this.e4315) + (anti_reverse.e4125 * this.e4125) - (anti_reverse.scalar * this.scalar) - (anti_reverse.e45 * this.e45) - (anti_reverse.e1234 * this.e3215) - (anti_reverse.e3215 * this.e1234)), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e235, e315, e125, e5 */
            float4(float3(0.0), (anti_reverse.e23 * this.e15) + (anti_reverse.e31 * this.e25) + (anti_reverse.e12 * this.e35) + (anti_reverse.e15 * this.e23) + (anti_reverse.e25 * this.e31) + (anti_reverse.e35 * this.e12) + (anti_reverse.e4235 * this.e15) + (anti_reverse.e4315 * this.e25) + (anti_reverse.e4125 * this.e35) + (anti_reverse.e3215 * this.e45) - (anti_reverse.scalar * this.e3215) - (anti_reverse.e45 * this.e3215) - (anti_reverse.e15 * this.e4235) - (anti_reverse.e25 * this.e4315) - (anti_reverse.e35 * this.e4125) - (anti_reverse.e3215 * this.scalar)), 
            /* e1, e2, e3, e4 */
            (float4(this.e42, this.e1234, this.e1234, this.e41) * float4(anti_reverse.group2.zyz, anti_reverse.e23)) + (float4(this.scalar, this.e12, this.e23, this.e42) * float4(anti_reverse.group3.xxy, anti_reverse.e31)) + (float4(this.e31, this.scalar, this.scalar, this.e43) * float4(anti_reverse.group3.zyz, anti_reverse.e12)) + (float4(this.e45, this.e4125, this.e4235, this.e31) * float4(anti_reverse.group1.xxy, anti_reverse.e42)) + (float4(this.e1234, this.e43, this.e41, this.e4125) * float4(anti_reverse.group2.xxy, anti_reverse.e43)) + (float4(this.e4315, this.e45, this.e45, this.e4315) * float4(anti_reverse.group1.zyz, anti_reverse.e42)) + (anti_reverse.group0.yzxx * float4(this.group2.zxy, this.e23)) + (anti_reverse.group0.wwwx * this.group3.xyzx) + (this.group1.xyzz * float4(anti_reverse.group1.www, anti_reverse.e43)) + float4((anti_reverse.group3.www * this.group0.xyz), anti_reverse.e45 * this.e1234) - (float4(this.e25, this.e3215, this.e3215, this.scalar) * float4(anti_reverse.group0.zyz, anti_reverse.e1234)) - (float4(this.e3215, this.e35, this.e15, this.e1234) * anti_reverse.group0.xxyw) - (anti_reverse.group3.yzxz * float4(this.group1.zxy, this.e43)) - (this.group0.zxyx * float4(anti_reverse.group2.yzx, anti_reverse.e4235)) - float4((anti_reverse.group1.yzx * this.group3.zxy), anti_reverse.e1234 * this.e45) - float4((anti_reverse.group2.www * this.group2.xyz), anti_reverse.e4315 * this.e42)
        );
        return VersorEven.from_groups(
            /* e423, e431, e412, e12345 */
            geometric_anti_product.group0 + float4(float3(0.0), 2.0*(this.e41 * this.e15) + 2.0 * (this.e42 * this.e25) + 2.0 * (this.e43 * this.e35) + 2.0 * (this.e1234 * this.e3215) + powi(this.scalar, 2) + powi(this.e23, 2) + powi(this.e31, 2) + powi(this.e12, 2) - powi(this.e45, 2) - powi(this.e4235, 2) - powi(this.e4315, 2) - powi(this.e4125, 2)), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e235, e315, e125, e5 */
            geometric_anti_product.group2, 
            /* e1, e2, e3, e4 */
            geometric_anti_product.group3
        );
    }
}

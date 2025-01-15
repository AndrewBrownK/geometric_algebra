implementing cga3d;
using traits;
// Fancy postfix trick
public VersorEven operator /(AntiCircleRotor lhs, AntiConstraintViolationPrefixOrPostfix rhs)  {
    return lhs.anti_constraint_violation();
    }
// Fancy prefix trick
public VersorEven operator /(AntiConstraintViolationPrefixOrPostfix lhs, AntiCircleRotor rhs) {
    return rhs.anti_constraint_violation();
}
public extension AntiCircleRotor {
    public VersorEven anti_constraint_violation() {
        AntiCircleRotor anti_reverse = AntiCircleRotor(
            /* e41, e42, e43 */
            this.group0.xyz * float3(-1.0), 
            /* e23, e31, e12, e45 */
            this.group1 * float4(-1.0), 
            /* e15, e25, e35, scalar */
            this.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        VersorEven geometric_anti_product = VersorEven(
            /* e423, e431, e412, e12345 */
            float4(float3(0.0), (anti_reverse.e41 * this.e15) + (anti_reverse.e42 * this.e25) + (anti_reverse.e43 * this.e35) + (anti_reverse.e23 * this.e23) + (anti_reverse.e31 * this.e31) + (anti_reverse.e12 * this.e12) + (anti_reverse.e15 * this.e41) + (anti_reverse.e25 * this.e42) + (anti_reverse.e35 * this.e43) - (anti_reverse.e45 * this.e45) - (anti_reverse.scalar * this.scalar)), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e235, e315, e125, e5 */
            float4(float3(0.0), (anti_reverse.e23 * this.e15) + (anti_reverse.e31 * this.e25) + (anti_reverse.e12 * this.e35) + (anti_reverse.e15 * this.e23) + (anti_reverse.e25 * this.e31) + (anti_reverse.e35 * this.e12)), 
            /* e1, e2, e3, e4 */
            float4(-(anti_reverse.e43 * this.e25) - (anti_reverse.e25 * this.e43), -(anti_reverse.e41 * this.e35) - (anti_reverse.e35 * this.e41), -(anti_reverse.e42 * this.e15) - (anti_reverse.e15 * this.e42), (anti_reverse.e31 * this.e42) + (anti_reverse.e12 * this.e43)) + (this.group1.wwwz * float4(anti_reverse.group1.xyz, anti_reverse.e43)) + float4((anti_reverse.group0.xyz.yzx * this.group2.zxy), anti_reverse.e41 * this.e23) + float4((this.group0.xyz.yzx * anti_reverse.group2.zxy), anti_reverse.e42 * this.e31) + float4((anti_reverse.group1.www * this.group1.xyz), anti_reverse.e23 * this.e41)
        );
        return VersorEven(
            /* e423, e431, e412, e12345 */
            geometric_anti_product.group0 + float4(float3(0.0), 2.0*(this.e41 * this.e15) + 2.0 * (this.e42 * this.e25) + 2.0 * (this.e43 * this.e35) + pow(this.e23, 2) + pow(this.e31, 2) + pow(this.e12, 2) + pow(this.scalar, 2) - pow(this.e45, 2)), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e235, e315, e125, e5 */
            geometric_anti_product.group2, 
            /* e1, e2, e3, e4 */
            geometric_anti_product.group3
        );
    }
}
// Fancy postfix trick
public VersorEven operator /(AntiDipoleInversion lhs, AntiConstraintViolationPrefixOrPostfix rhs)  {
    return lhs.anti_constraint_violation();
    }
// Fancy prefix trick
public VersorEven operator /(AntiConstraintViolationPrefixOrPostfix lhs, AntiDipoleInversion rhs) {
    return rhs.anti_constraint_violation();
}
public extension AntiDipoleInversion {
    public VersorEven anti_constraint_violation() {
        AntiDipoleInversion anti_reverse = AntiDipoleInversion(
            /* e423, e431, e412 */
            this.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            this.group1 * float4(-1.0), 
            /* e235, e315, e125, e4 */
            this.group2 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            this.group3
        );
        VersorEven geometric_anti_product = VersorEven(
            /* e423, e431, e412, e12345 */
            float4(float3(0.0), (anti_reverse.e321 * this.e321) + (anti_reverse.e4 * this.e5) + (anti_reverse.e5 * this.e4) - (anti_reverse.e423 * this.e235) - (anti_reverse.e431 * this.e315) - (anti_reverse.e412 * this.e125) - (anti_reverse.e415 * this.e415) - (anti_reverse.e425 * this.e425) - (anti_reverse.e435 * this.e435) - (anti_reverse.e235 * this.e423) - (anti_reverse.e315 * this.e431) - (anti_reverse.e125 * this.e412) - (anti_reverse.e1 * this.e1) - (anti_reverse.e2 * this.e2) - (anti_reverse.e3 * this.e3)), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e235, e315, e125, e5 */
            float4(float3(0.0), (anti_reverse.e1 * this.e235) + (anti_reverse.e2 * this.e315) + (anti_reverse.e3 * this.e125) + (anti_reverse.e5 * this.e321) - (anti_reverse.e415 * this.e235) - (anti_reverse.e425 * this.e315) - (anti_reverse.e435 * this.e125) - (anti_reverse.e321 * this.e5) - (anti_reverse.e235 * this.e415) - (anti_reverse.e235 * this.e1) - (anti_reverse.e315 * this.e425) - (anti_reverse.e315 * this.e2) - (anti_reverse.e125 * this.e435) - (anti_reverse.e125 * this.e3)), 
            /* e1, e2, e3, e4 */
            float4((anti_reverse.e435 * this.e2) + (anti_reverse.e321 * this.e415) + (anti_reverse.e4 * this.e235) + (anti_reverse.e3 * this.e425), (anti_reverse.e425 * this.e321) + (anti_reverse.e321 * this.e425) + (anti_reverse.e4 * this.e315) + (anti_reverse.e1 * this.e435), (anti_reverse.e435 * this.e321) + (anti_reverse.e321 * this.e435) + (anti_reverse.e4 * this.e125) + (anti_reverse.e2 * this.e415), -(anti_reverse.e435 * this.e412) - (anti_reverse.e4 * this.e321) - (anti_reverse.e2 * this.e431) - (anti_reverse.e3 * this.e412)) + (float4(this.e321, this.e3, this.e1, this.e4) * anti_reverse.group1.xxyw) + (float4(this.e315, this.e5, this.e5, this.e2) * float4(anti_reverse.group0.xyz.zyz, anti_reverse.e431)) + (float4(this.e5, this.e125, this.e235, this.e1) * float4(anti_reverse.group0.xyz.xxy, anti_reverse.e423)) + float4((this.group0.xyz.zxy * anti_reverse.group2.yzx), anti_reverse.e412 * this.e3) - (float4(anti_reverse.group2.zx, anti_reverse.e5, this.e435) * float4(this.group0.xyz.yzz, anti_reverse.e412)) - (float4(anti_reverse.group3.ww, anti_reverse.e315, this.e425) * float4(this.group0.xyz.xyx, anti_reverse.e431)) - float4((anti_reverse.group0.xyz.yzx * this.group2.zxy), anti_reverse.e423 * this.e415) - float4((anti_reverse.group1.yzx * this.group3.zxy), anti_reverse.e415 * this.e423) - float4((anti_reverse.group3.yzx * this.group1.zxy), anti_reverse.e425 * this.e431) - float4((this.group2.www * anti_reverse.group2.xyz), anti_reverse.e1 * this.e423)
        );
        return VersorEven(
            /* e423, e431, e412, e12345 */
            geometric_anti_product.group0 + float4(float3(0.0), pow(this.e321, 2) + pow(this.e1, 2) + pow(this.e2, 2) + pow(this.e3, 2) - pow(this.e415, 2) - pow(this.e425, 2) - pow(this.e435, 2) - 2.0 * (this.e423 * this.e235) - 2.0 * (this.e431 * this.e315) - 2.0 * (this.e412 * this.e125) - 2.0 * (this.e4 * this.e5)), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e235, e315, e125, e5 */
            geometric_anti_product.group2, 
            /* e1, e2, e3, e4 */
            geometric_anti_product.group3
        );
    }
}
// Fancy postfix trick
public DualNum operator /(AntiDualNum lhs, AntiConstraintViolationPrefixOrPostfix rhs)  {
    return lhs.anti_constraint_violation();
    }
// Fancy prefix trick
public DualNum operator /(AntiConstraintViolationPrefixOrPostfix lhs, AntiDualNum rhs) {
    return rhs.anti_constraint_violation();
}
public extension AntiDualNum {
    public DualNum anti_constraint_violation() {
        return DualNum(
            /* e5, e12345 */
            float2(this.e3215 * this.scalar * -2.0, 0.0)
        );
    }
}
// Fancy postfix trick
public DualNum operator /(AntiFlector lhs, AntiConstraintViolationPrefixOrPostfix rhs)  {
    return lhs.anti_constraint_violation();
    }
// Fancy prefix trick
public DualNum operator /(AntiConstraintViolationPrefixOrPostfix lhs, AntiFlector rhs) {
    return rhs.anti_constraint_violation();
}
public extension AntiFlector {
    public DualNum anti_constraint_violation() {
        AntiFlector anti_reverse = AntiFlector(
            /* e235, e315, e125, e321 */
            this.group0 * float4(-1.0), 
            /* e1, e2, e3, e5 */
            this.group1
        );
        return DualNum(
            /* e5, e12345 */
            float2((anti_reverse.e1 * this.e235) + (anti_reverse.e2 * this.e315) + (anti_reverse.e3 * this.e125) + (anti_reverse.e5 * this.e321) - (anti_reverse.e235 * this.e1) - (anti_reverse.e315 * this.e2) - (anti_reverse.e125 * this.e3) - (anti_reverse.e321 * this.e5), 0.0)
        );
    }
}
// Fancy postfix trick
public DualNum operator /(AntiLine lhs, AntiConstraintViolationPrefixOrPostfix rhs)  {
    return lhs.anti_constraint_violation();
    }
// Fancy prefix trick
public DualNum operator /(AntiConstraintViolationPrefixOrPostfix lhs, AntiLine rhs) {
    return rhs.anti_constraint_violation();
}
public extension AntiLine {
    public DualNum anti_constraint_violation() {
        AntiLine anti_reverse = AntiLine(
            /* e23, e31, e12 */
            this.group0.xyz * float3(-1.0), 
            /* e15, e25, e35 */
            this.group1.xyz * float3(-1.0)
        );
        return DualNum(
            /* e5, e12345 */
            float2((anti_reverse.e23 * this.e15) + (anti_reverse.e31 * this.e25) + (anti_reverse.e12 * this.e35) + (anti_reverse.e15 * this.e23) + (anti_reverse.e25 * this.e31) + (anti_reverse.e35 * this.e12), 0.0)
        );
    }
}
// Fancy postfix trick
public DualNum operator /(AntiMotor lhs, AntiConstraintViolationPrefixOrPostfix rhs)  {
    return lhs.anti_constraint_violation();
    }
// Fancy prefix trick
public DualNum operator /(AntiConstraintViolationPrefixOrPostfix lhs, AntiMotor rhs) {
    return rhs.anti_constraint_violation();
}
public extension AntiMotor {
    public DualNum anti_constraint_violation() {
        AntiMotor anti_reverse = AntiMotor(
            /* e23, e31, e12, scalar */
            this.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e15, e25, e35, e3215 */
            this.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return DualNum(
            /* e5, e12345 */
            float2((anti_reverse.e23 * this.e15) + (anti_reverse.e31 * this.e25) + (anti_reverse.e12 * this.e35) + (anti_reverse.e15 * this.e23) + (anti_reverse.e25 * this.e31) + (anti_reverse.e35 * this.e12) - (anti_reverse.scalar * this.e3215) - (anti_reverse.e3215 * this.scalar), 0.0)
        );
    }
}
// Fancy postfix trick
public VersorEven operator /(Circle lhs, AntiConstraintViolationPrefixOrPostfix rhs)  {
    return lhs.anti_constraint_violation();
    }
// Fancy prefix trick
public VersorEven operator /(AntiConstraintViolationPrefixOrPostfix lhs, Circle rhs) {
    return rhs.anti_constraint_violation();
}
public extension Circle {
    public VersorEven anti_constraint_violation() {
        Circle anti_reverse = Circle(
            /* e423, e431, e412 */
            this.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            this.group1 * float4(-1.0), 
            /* e235, e315, e125 */
            this.group2.xyz * float3(-1.0)
        );
        VersorEven geometric_anti_product = VersorEven(
            /* e423, e431, e412, e12345 */
            float4(float3(0.0), (anti_reverse.e321 * this.e321) - (anti_reverse.e423 * this.e235) - (anti_reverse.e431 * this.e315) - (anti_reverse.e412 * this.e125) - (anti_reverse.e415 * this.e415) - (anti_reverse.e425 * this.e425) - (anti_reverse.e435 * this.e435) - (anti_reverse.e235 * this.e423) - (anti_reverse.e315 * this.e431) - (anti_reverse.e125 * this.e412)), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e235, e315, e125, e5 */
            float4(float3(0.0), -(anti_reverse.e415 * this.e235) - (anti_reverse.e425 * this.e315) - (anti_reverse.e435 * this.e125) - (anti_reverse.e235 * this.e415) - (anti_reverse.e315 * this.e425) - (anti_reverse.e125 * this.e435)), 
            /* e1, e2, e3, e4 */
            float4((anti_reverse.e412 * this.e315) + (anti_reverse.e415 * this.e321) + (anti_reverse.e321 * this.e415) + (anti_reverse.e315 * this.e412), (anti_reverse.e423 * this.e125) + (anti_reverse.e425 * this.e321) + (anti_reverse.e321 * this.e425) + (anti_reverse.e125 * this.e423), (anti_reverse.e431 * this.e235) + (anti_reverse.e435 * this.e321) + (anti_reverse.e321 * this.e435) + (anti_reverse.e235 * this.e431), -(anti_reverse.e412 * this.e435) - (anti_reverse.e415 * this.e423) - (anti_reverse.e425 * this.e431) - (anti_reverse.e435 * this.e412)) - float4((anti_reverse.group0.xyz.yzx * this.group2.xyz.zxy), anti_reverse.e423 * this.e415) - float4((anti_reverse.group2.xyz.zxy * this.group0.xyz.yzx), anti_reverse.e431 * this.e425)
        );
        return VersorEven(
            /* e423, e431, e412, e12345 */
            geometric_anti_product.group0 + float4(float3(0.0), pow(this.e321, 2) - pow(this.e415, 2) - pow(this.e425, 2) - pow(this.e435, 2) - 2.0 * (this.e423 * this.e235) - 2.0 * (this.e431 * this.e315) - 2.0 * (this.e412 * this.e125)), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e235, e315, e125, e5 */
            geometric_anti_product.group2, 
            /* e1, e2, e3, e4 */
            geometric_anti_product.group3
        );
    }
}
// Fancy postfix trick
public VersorEven operator /(CircleRotor lhs, AntiConstraintViolationPrefixOrPostfix rhs)  {
    return lhs.anti_constraint_violation();
    }
// Fancy prefix trick
public VersorEven operator /(AntiConstraintViolationPrefixOrPostfix lhs, CircleRotor rhs) {
    return rhs.anti_constraint_violation();
}
public extension CircleRotor {
    public VersorEven anti_constraint_violation() {
        CircleRotor anti_reverse = CircleRotor(
            /* e423, e431, e412 */
            this.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            this.group1 * float4(-1.0), 
            /* e235, e315, e125, e12345 */
            this.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        VersorEven geometric_anti_product = VersorEven(
            /* e423, e431, e412, e12345 */
            float4(float3(0.0), (anti_reverse.e321 * this.e321) + (anti_reverse.e12345 * this.e12345) - (anti_reverse.e423 * this.e235) - (anti_reverse.e431 * this.e315) - (anti_reverse.e412 * this.e125) - (anti_reverse.e415 * this.e415) - (anti_reverse.e425 * this.e425) - (anti_reverse.e435 * this.e435) - (anti_reverse.e235 * this.e423) - (anti_reverse.e315 * this.e431) - (anti_reverse.e125 * this.e412)), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e235, e315, e125, e5 */
            float4(float3(0.0), -(anti_reverse.e415 * this.e235) - (anti_reverse.e425 * this.e315) - (anti_reverse.e435 * this.e125) - (anti_reverse.e235 * this.e415) - (anti_reverse.e315 * this.e425) - (anti_reverse.e125 * this.e435)), 
            /* e1, e2, e3, e4 */
            float4((anti_reverse.e412 * this.e315) + (anti_reverse.e415 * this.e321) + (anti_reverse.e321 * this.e415) + (anti_reverse.e315 * this.e412), (anti_reverse.e423 * this.e125) + (anti_reverse.e425 * this.e321) + (anti_reverse.e321 * this.e425) + (anti_reverse.e125 * this.e423), (anti_reverse.e431 * this.e235) + (anti_reverse.e435 * this.e321) + (anti_reverse.e321 * this.e435) + (anti_reverse.e235 * this.e431), -(anti_reverse.e412 * this.e435) - (anti_reverse.e415 * this.e423) - (anti_reverse.e425 * this.e431) - (anti_reverse.e435 * this.e412)) - float4((anti_reverse.group0.xyz.yzx * this.group2.zxy), anti_reverse.e423 * this.e415) - float4((this.group0.xyz.yzx * anti_reverse.group2.zxy), anti_reverse.e431 * this.e425)
        );
        return VersorEven(
            /* e423, e431, e412, e12345 */
            geometric_anti_product.group0 + float4(float3(0.0), pow(this.e321, 2) - pow(this.e415, 2) - pow(this.e425, 2) - pow(this.e435, 2) - pow(this.e12345, 2) - 2.0 * (this.e423 * this.e235) - 2.0 * (this.e431 * this.e315) - 2.0 * (this.e412 * this.e125)), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e235, e315, e125, e5 */
            geometric_anti_product.group2, 
            /* e1, e2, e3, e4 */
            geometric_anti_product.group3
        );
    }
}
// Fancy postfix trick
public VersorEven operator /(Dipole lhs, AntiConstraintViolationPrefixOrPostfix rhs)  {
    return lhs.anti_constraint_violation();
    }
// Fancy prefix trick
public VersorEven operator /(AntiConstraintViolationPrefixOrPostfix lhs, Dipole rhs) {
    return rhs.anti_constraint_violation();
}
public extension Dipole {
    public VersorEven anti_constraint_violation() {
        Dipole anti_reverse = Dipole(
            /* e41, e42, e43 */
            this.group0.xyz * float3(-1.0), 
            /* e23, e31, e12, e45 */
            this.group1 * float4(-1.0), 
            /* e15, e25, e35 */
            this.group2.xyz * float3(-1.0)
        );
        VersorEven geometric_anti_product = VersorEven(
            /* e423, e431, e412, e12345 */
            float4(float3(0.0), (anti_reverse.e41 * this.e15) + (anti_reverse.e42 * this.e25) + (anti_reverse.e43 * this.e35) + (anti_reverse.e23 * this.e23) + (anti_reverse.e31 * this.e31) + (anti_reverse.e12 * this.e12) + (anti_reverse.e15 * this.e41) + (anti_reverse.e25 * this.e42) + (anti_reverse.e35 * this.e43) - (anti_reverse.e45 * this.e45)), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e235, e315, e125, e5 */
            float4(float3(0.0), (anti_reverse.e23 * this.e15) + (anti_reverse.e31 * this.e25) + (anti_reverse.e12 * this.e35) + (anti_reverse.e15 * this.e23) + (anti_reverse.e25 * this.e31) + (anti_reverse.e35 * this.e12)), 
            /* e1, e2, e3, e4 */
            float4(-(anti_reverse.e43 * this.e25) - (anti_reverse.e25 * this.e43), -(anti_reverse.e41 * this.e35) - (anti_reverse.e35 * this.e41), -(anti_reverse.e42 * this.e15) - (anti_reverse.e15 * this.e42), (anti_reverse.e31 * this.e42) + (anti_reverse.e12 * this.e43)) + (this.group1.wwwz * float4(anti_reverse.group1.xyz, anti_reverse.e43)) + float4((anti_reverse.group0.xyz.yzx * this.group2.xyz.zxy), anti_reverse.e41 * this.e23) + float4((anti_reverse.group2.xyz.zxy * this.group0.xyz.yzx), anti_reverse.e42 * this.e31) + float4((anti_reverse.group1.www * this.group1.xyz), anti_reverse.e23 * this.e41)
        );
        return VersorEven(
            /* e423, e431, e412, e12345 */
            geometric_anti_product.group0 + float4(float3(0.0), 2.0*(this.e41 * this.e15) + 2.0 * (this.e42 * this.e25) + 2.0 * (this.e43 * this.e35) + pow(this.e23, 2) + pow(this.e31, 2) + pow(this.e12, 2) - pow(this.e45, 2)), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e235, e315, e125, e5 */
            geometric_anti_product.group2, 
            /* e1, e2, e3, e4 */
            geometric_anti_product.group3
        );
    }
}
// Fancy postfix trick
public VersorEven operator /(DipoleInversion lhs, AntiConstraintViolationPrefixOrPostfix rhs)  {
    return lhs.anti_constraint_violation();
    }
// Fancy prefix trick
public VersorEven operator /(AntiConstraintViolationPrefixOrPostfix lhs, DipoleInversion rhs) {
    return rhs.anti_constraint_violation();
}
public extension DipoleInversion {
    public VersorEven anti_constraint_violation() {
        DipoleInversion anti_reverse = DipoleInversion(
            /* e41, e42, e43 */
            this.group0.xyz * float3(-1.0), 
            /* e23, e31, e12, e45 */
            this.group1 * float4(-1.0), 
            /* e15, e25, e35, e1234 */
            this.group2 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group3
        );
        VersorEven geometric_anti_product = VersorEven(
            /* e423, e431, e412, e12345 */
            float4(float3(0.0), (anti_reverse.e41 * this.e15) + (anti_reverse.e42 * this.e25) + (anti_reverse.e43 * this.e35) + (anti_reverse.e23 * this.e23) + (anti_reverse.e31 * this.e31) + (anti_reverse.e12 * this.e12) + (anti_reverse.e15 * this.e41) + (anti_reverse.e25 * this.e42) + (anti_reverse.e35 * this.e43) + (anti_reverse.e4235 * this.e4235) + (anti_reverse.e4315 * this.e4315) + (anti_reverse.e4125 * this.e4125) - (anti_reverse.e45 * this.e45) - (anti_reverse.e1234 * this.e3215) - (anti_reverse.e3215 * this.e1234)), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e235, e315, e125, e5 */
            float4(float3(0.0), (anti_reverse.e23 * this.e15) + (anti_reverse.e31 * this.e25) + (anti_reverse.e12 * this.e35) + (anti_reverse.e15 * this.e23) + (anti_reverse.e25 * this.e31) + (anti_reverse.e35 * this.e12) + (anti_reverse.e4235 * this.e15) + (anti_reverse.e4315 * this.e25) + (anti_reverse.e4125 * this.e35) + (anti_reverse.e3215 * this.e45) - (anti_reverse.e45 * this.e3215) - (anti_reverse.e15 * this.e4235) - (anti_reverse.e25 * this.e4315) - (anti_reverse.e35 * this.e4125)), 
            /* e1, e2, e3, e4 */
            float4(-(anti_reverse.e1234 * this.e15) - (anti_reverse.e4315 * this.e12), -(anti_reverse.e1234 * this.e25) - (anti_reverse.e4125 * this.e23), -(anti_reverse.e1234 * this.e35) - (anti_reverse.e4235 * this.e31), (anti_reverse.e12 * this.e43) + (anti_reverse.e45 * this.e1234)) + (float4(this.e45, this.e4125, this.e4235, this.e4315) * float4(anti_reverse.group1.xxy, anti_reverse.e42)) + (float4(this.e4315, this.e45, this.e45, this.e12) * float4(anti_reverse.group1.zyz, anti_reverse.e43)) + (float4(anti_reverse.group2.zx, anti_reverse.e3215, this.e31) * float4(this.group0.xyz.yzz, anti_reverse.e42)) + (float4(anti_reverse.group3.ww, anti_reverse.e25, this.e4235) * float4(this.group0.xyz.xyx, anti_reverse.e41)) + float4((anti_reverse.group0.xyz.yzx * this.group2.zxy), anti_reverse.e41 * this.e23) + float4((anti_reverse.group1.www * this.group1.xyz), anti_reverse.e43 * this.e4125) + float4((anti_reverse.group3.zxy * this.group1.yzx), anti_reverse.e31 * this.e42) + float4((this.group2.www * anti_reverse.group2.xyz), anti_reverse.e23 * this.e41) - (float4(this.e25, this.e3215, this.e3215, anti_reverse.e4315) * float4(anti_reverse.group0.xyz.zyz, this.e42)) - (float4(this.e3215, this.e35, this.e15, anti_reverse.e4235) * float4(anti_reverse.group0.xyz.xxy, this.e41)) - float4((this.group0.xyz.zxy * anti_reverse.group2.yzx), anti_reverse.e4125 * this.e43) - float4((anti_reverse.group1.yzx * this.group3.zxy), anti_reverse.e1234 * this.e45)
        );
        return VersorEven(
            /* e423, e431, e412, e12345 */
            geometric_anti_product.group0 + float4(float3(0.0), 2.0*(this.e41 * this.e15) + 2.0 * (this.e42 * this.e25) + 2.0 * (this.e43 * this.e35) + 2.0 * (this.e1234 * this.e3215) + pow(this.e23, 2) + pow(this.e31, 2) + pow(this.e12, 2) - pow(this.e45, 2) - pow(this.e4235, 2) - pow(this.e4315, 2) - pow(this.e4125, 2)), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e235, e315, e125, e5 */
            geometric_anti_product.group2, 
            /* e1, e2, e3, e4 */
            geometric_anti_product.group3
        );
    }
}
// Fancy postfix trick
public DualNum operator /(DualNum lhs, AntiConstraintViolationPrefixOrPostfix rhs)  {
    return lhs.anti_constraint_violation();
    }
// Fancy prefix trick
public DualNum operator /(AntiConstraintViolationPrefixOrPostfix lhs, DualNum rhs) {
    return rhs.anti_constraint_violation();
}
public extension DualNum {
    public DualNum anti_constraint_violation() {
        return DualNum(
            /* e5, e12345 */
            float2(this.e5 * this.e12345 * 2.0, 0.0)
        );
    }
}
// Fancy postfix trick
public DualNum operator /(Flector lhs, AntiConstraintViolationPrefixOrPostfix rhs)  {
    return lhs.anti_constraint_violation();
    }
// Fancy prefix trick
public DualNum operator /(AntiConstraintViolationPrefixOrPostfix lhs, Flector rhs) {
    return rhs.anti_constraint_violation();
}
public extension Flector {
    public DualNum anti_constraint_violation() {
        Flector anti_reverse = Flector(
            /* e15, e25, e35, e45 */
            this.group0 * float4(-1.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group1
        );
        return DualNum(
            /* e5, e12345 */
            float2((anti_reverse.e4235 * this.e15) + (anti_reverse.e4315 * this.e25) + (anti_reverse.e4125 * this.e35) + (anti_reverse.e3215 * this.e45) - (anti_reverse.e15 * this.e4235) - (anti_reverse.e25 * this.e4315) - (anti_reverse.e35 * this.e4125) - (anti_reverse.e45 * this.e3215), 0.0)
        );
    }
}
// Fancy postfix trick
public DualNum operator /(Line lhs, AntiConstraintViolationPrefixOrPostfix rhs)  {
    return lhs.anti_constraint_violation();
    }
// Fancy prefix trick
public DualNum operator /(AntiConstraintViolationPrefixOrPostfix lhs, Line rhs) {
    return rhs.anti_constraint_violation();
}
public extension Line {
    public DualNum anti_constraint_violation() {
        Line anti_reverse = Line(
            /* e415, e425, e435 */
            this.group0.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            this.group1.xyz * float3(-1.0)
        );
        return DualNum(
            /* e5, e12345 */
            float2(-(anti_reverse.e415 * this.e235) - (anti_reverse.e425 * this.e315) - (anti_reverse.e435 * this.e125) - (anti_reverse.e235 * this.e415) - (anti_reverse.e315 * this.e425) - (anti_reverse.e125 * this.e435), 0.0)
        );
    }
}
// Fancy postfix trick
public DualNum operator /(Motor lhs, AntiConstraintViolationPrefixOrPostfix rhs)  {
    return lhs.anti_constraint_violation();
    }
// Fancy prefix trick
public DualNum operator /(AntiConstraintViolationPrefixOrPostfix lhs, Motor rhs) {
    return rhs.anti_constraint_violation();
}
public extension Motor {
    public DualNum anti_constraint_violation() {
        Motor anti_reverse = Motor(
            /* e415, e425, e435, e12345 */
            this.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            this.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return DualNum(
            /* e5, e12345 */
            float2((anti_reverse.e12345 * this.e5) + (anti_reverse.e5 * this.e12345) - (anti_reverse.e415 * this.e235) - (anti_reverse.e425 * this.e315) - (anti_reverse.e435 * this.e125) - (anti_reverse.e235 * this.e415) - (anti_reverse.e315 * this.e425) - (anti_reverse.e125 * this.e435), 0.0)
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
            /* scalar, e12345 */
            this.group0.xy, 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e5 */
            this.e5, 
            /* e15, e25, e35, e45 */
            this.group3 * float4(-1.0), 
            /* e41, e42, e43 */
            this.group4.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            this.group5.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            this.group6 * float4(-1.0), 
            /* e423, e431, e412 */
            this.group7.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            this.group8.xyz * float3(-1.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group9, 
            /* e1234 */
            this.e1234
        );
        MultiVector geometric_anti_product = MultiVector(
            /* scalar, e12345 */
            float2((anti_reverse.e12345 * this.scalar) + (anti_reverse.e5 * this.e1234) + (anti_reverse.e4315 * this.e2) + (anti_reverse.e4125 * this.e3) + (anti_reverse.e3215 * this.e4) + (anti_reverse.e1234 * this.e5) - (anti_reverse.e15 * this.e423) - (anti_reverse.e25 * this.e431) - (anti_reverse.e35 * this.e412) - (anti_reverse.e45 * this.e321) - (anti_reverse.e415 * this.e23) - (anti_reverse.e425 * this.e31) - (anti_reverse.e435 * this.e12) - (anti_reverse.e321 * this.e45) - (anti_reverse.e431 * this.e25) - (anti_reverse.e412 * this.e35), (anti_reverse.e15 * this.e41) + (anti_reverse.e25 * this.e42) + (anti_reverse.e35 * this.e43) + (anti_reverse.e41 * this.e15) + (anti_reverse.e42 * this.e25) + (anti_reverse.e43 * this.e35) + (anti_reverse.e23 * this.e23) + (anti_reverse.e31 * this.e31) + (anti_reverse.e12 * this.e12) + (anti_reverse.e321 * this.e321) - (anti_reverse.scalar * this.scalar) - (anti_reverse.e1 * this.e1) - (anti_reverse.e2 * this.e2) - (anti_reverse.e3 * this.e3) - (anti_reverse.e3215 * this.e1234) - (anti_reverse.e1234 * this.e3215)) + (float2(this.e12345) * anti_reverse.group0.xy) + (float2(anti_reverse.e2, anti_reverse.e4235) * this.group9.yx) + (float2(anti_reverse.e3, anti_reverse.e4315) * this.group9.zy) + (float2(anti_reverse.e4, anti_reverse.e4125) * this.group9.wz) + (float2(anti_reverse.e4235, anti_reverse.e5) * this.group1.xw) + (float2(this.e4235, this.e5) * anti_reverse.group1.xw) - (float2(anti_reverse.e235) * float2(this.e41, this.e423)) - (float2(anti_reverse.e315) * float2(this.e42, this.e431)) - (float2(anti_reverse.e125) * float2(this.e43, this.e412)) - (float2(this.e415) * float2(anti_reverse.e23, anti_reverse.e415)) - (float2(this.e425) * float2(anti_reverse.e31, anti_reverse.e425)) - (float2(this.e435) * float2(anti_reverse.e12, anti_reverse.e435)) - (float2(this.e235) * float2(anti_reverse.e41, anti_reverse.e423)) - (float2(this.e315) * float2(anti_reverse.e42, anti_reverse.e431)) - (float2(this.e125) * float2(anti_reverse.e43, anti_reverse.e412)) - (float2(anti_reverse.e423, anti_reverse.e45) * this.group3.xw), 
            /* e1, e2, e3, e4 */
            float4((anti_reverse.e15 * this.e1234) + (anti_reverse.e415 * this.e321) + (anti_reverse.e435 * this.e2) + (anti_reverse.e321 * this.e415), (anti_reverse.e25 * this.e1234) + (anti_reverse.e415 * this.e3) + (anti_reverse.e425 * this.e321) + (anti_reverse.e321 * this.e425), (anti_reverse.e35 * this.e1234) + (anti_reverse.e425 * this.e1) + (anti_reverse.e435 * this.e321) + (anti_reverse.e321 * this.e435), -(anti_reverse.e3 * this.e412) - (anti_reverse.e4 * this.e321) - (anti_reverse.e435 * this.e412) - (anti_reverse.e1234 * this.e45)) + (float4(this.e5, this.e125, this.e235, this.e4125) * float4(anti_reverse.group7.xyz.xxy, anti_reverse.e43)) + (float4(this.e45, this.e4125, this.e4235, this.e4315) * float4(anti_reverse.group5.xyz.xxy, anti_reverse.e42)) + (float4(this.e315, this.e5, this.e5, this.e41) * float4(anti_reverse.group7.xyz.zyz, anti_reverse.e23)) + (float4(this.e4315, this.e45, this.e45, this.e12) * float4(anti_reverse.group5.xyz.zyz, anti_reverse.e43)) + (float4(anti_reverse.group0.xy.xx, anti_reverse.scalar, anti_reverse.e12345) * float4(this.group9.xyz, this.e4)) + (float4(anti_reverse.group0.xy.yy, anti_reverse.e12345, this.e12345) * float4(this.group1.xyz, anti_reverse.e4)) + (float4(this.group0.xy.xx, this.scalar, anti_reverse.e41) * float4(anti_reverse.group9.xyz, this.e23)) + (float4(this.group0.xy.yy, this.e12345, anti_reverse.e41) * float4(anti_reverse.group1.xyz, this.e4235)) + (float4(anti_reverse.group3.zx, anti_reverse.e3215, this.e1) * float4(this.group4.xyz.yzz, anti_reverse.e423)) + (float4(anti_reverse.group3.ww, anti_reverse.e4315, this.e2) * float4(this.group5.xyz.xyx, anti_reverse.e431)) + (float4(anti_reverse.group9.zx, anti_reverse.e45, this.e3) * float4(this.group5.xyz.yzz, anti_reverse.e412)) + (float4(anti_reverse.group9.ww, anti_reverse.e25, this.e43) * float4(this.group4.xyz.xyx, anti_reverse.e12)) + float4((this.group8.xyz * anti_reverse.group1.www), anti_reverse.e45 * this.e1234) + float4((anti_reverse.group4.xyz.yzx * this.group3.zxy), anti_reverse.e42 * this.e31) + float4((anti_reverse.group8.xyz.yzx * this.group7.xyz.zxy), anti_reverse.e31 * this.e42) + float4((anti_reverse.group1.zxy * this.group6.yzx), anti_reverse.e321 * this.e4) - (float4(this.e4, this.e412, this.e423, this.e435) * float4(anti_reverse.group8.xyz.xxy, anti_reverse.e412)) - (float4(this.e25, this.e3215, this.e3215, anti_reverse.e1234) * float4(anti_reverse.group4.xyz.zyz, this.scalar)) - (float4(this.e431, this.e4, this.e4, anti_reverse.e4235) * float4(anti_reverse.group8.xyz.zyz, this.e41)) - (float4(this.e3215, this.e35, this.e15, this.e1234) * float4(anti_reverse.group4.xyz.xxy, anti_reverse.scalar)) - (anti_reverse.group9.yzxz * float4(this.group5.xyz.zxy, this.e43)) - float4((float3(anti_reverse.e5) * this.group7.xyz), anti_reverse.e1 * this.e423) - float4((float3(anti_reverse.e1234) * this.group3.xyz), anti_reverse.e425 * this.e431) - float4((anti_reverse.group5.xyz.yzx * this.group9.zxy), anti_reverse.e423 * this.e415) - float4((anti_reverse.group7.xyz.yzx * this.group8.xyz.zxy), anti_reverse.e431 * this.e425) - float4((this.group4.xyz.zxy * anti_reverse.group3.yzx), anti_reverse.e4315 * this.e42) - float4((anti_reverse.group1.yzx * this.group6.zxy), anti_reverse.e415 * this.e423) - float4((anti_reverse.group6.yzx * this.group1.zxy), anti_reverse.e2 * this.e431), 
            /* e5 */
            (anti_reverse.e12345 * this.e5) + (anti_reverse.e1 * this.e235) + (anti_reverse.e2 * this.e315) + (anti_reverse.e3 * this.e125) + (anti_reverse.e5 * this.e12345) + (anti_reverse.e5 * this.e321) + (anti_reverse.e15 * this.e23) + (anti_reverse.e25 * this.e31) + (anti_reverse.e35 * this.e12) + (anti_reverse.e23 * this.e15) + (anti_reverse.e31 * this.e25) + (anti_reverse.e12 * this.e35) + (anti_reverse.e4235 * this.e15) + (anti_reverse.e4315 * this.e25) + (anti_reverse.e4125 * this.e35) + (anti_reverse.e3215 * this.e45) - (anti_reverse.scalar * this.e3215) - (anti_reverse.e15 * this.e4235) - (anti_reverse.e25 * this.e4315) - (anti_reverse.e35 * this.e4125) - (anti_reverse.e45 * this.e3215) - (anti_reverse.e415 * this.e235) - (anti_reverse.e425 * this.e315) - (anti_reverse.e435 * this.e125) - (anti_reverse.e321 * this.e5) - (anti_reverse.e235 * this.e1) - (anti_reverse.e235 * this.e415) - (anti_reverse.e315 * this.e2) - (anti_reverse.e315 * this.e425) - (anti_reverse.e125 * this.e3) - (anti_reverse.e125 * this.e435) - (anti_reverse.e3215 * this.scalar), 
            /* e15, e25, e35, e45 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4((anti_reverse.e45 * this.e415) + (anti_reverse.e415 * this.e45) + (anti_reverse.e435 * this.e4315) + (anti_reverse.e4125 * this.e425), (anti_reverse.e45 * this.e425) + (anti_reverse.e415 * this.e4125) + (anti_reverse.e425 * this.e45) + (anti_reverse.e4235 * this.e435), (anti_reverse.e45 * this.e435) + (anti_reverse.e425 * this.e4235) + (anti_reverse.e435 * this.e45) + (anti_reverse.e4315 * this.e415), -(anti_reverse.e5 * this.e45) - (anti_reverse.e425 * this.e25) - (anti_reverse.e435 * this.e35) - (anti_reverse.e321 * this.e3215)) + (float4(this.e42, this.e1234, this.e1234, this.e4125) * float4(anti_reverse.group8.xyz.zyz, anti_reverse.e125)) + (float4(this.e1234, this.e43, this.e41, this.e4315) * float4(anti_reverse.group8.xyz.xxy, anti_reverse.e315)) + (float4(anti_reverse.group0.xy.yy, anti_reverse.e12345, anti_reverse.scalar) * float4(this.group9.xyz, this.e5)) + (float4(this.group0.xy.yy, this.e12345, anti_reverse.e12345) * float4(anti_reverse.group9.xyz, this.e3215)) + (float4(anti_reverse.group3.zx, anti_reverse.e3215, this.e5) * float4(this.group7.xyz.yzz, anti_reverse.e45)) + (float4(anti_reverse.group9.ww, anti_reverse.e25, this.e35) * float4(this.group7.xyz.xyx, anti_reverse.e3)) + float4((float3(anti_reverse.e5) * this.group4.xyz), anti_reverse.e1 * this.e15) + float4((anti_reverse.group4.xyz.yzx * this.group8.xyz.zxy), anti_reverse.e5 * this.scalar) + float4((anti_reverse.group5.xyz.yzx * this.group1.zxy), anti_reverse.e3215 * this.e12345) + float4((anti_reverse.group7.xyz.yzx * this.group3.zxy), anti_reverse.e235 * this.e4235) + float4((this.group5.xyz.zxy * anti_reverse.group1.yzx), anti_reverse.e2 * this.e25) + float4((this.group1.www * anti_reverse.group3.xyz), anti_reverse.e3215 * this.e321) - (float4(this.e2, this.e321, this.e321, this.e12) * float4(anti_reverse.group5.xyz.zyz, anti_reverse.e125)) - (float4(this.e5, this.e125, this.e235, this.e125) * float4(anti_reverse.group4.xyz.xxy, anti_reverse.e12)) - (float4(this.e25, this.e3215, this.e3215, anti_reverse.e4315) * float4(anti_reverse.group7.xyz.zyz, this.e315)) - (float4(this.e321, this.e3, this.e1, this.e31) * float4(anti_reverse.group5.xyz.xxy, anti_reverse.e315)) - (float4(this.e315, this.e5, this.e5, this.e23) * float4(anti_reverse.group4.xyz.zyz, anti_reverse.e235)) - (float4(this.e3215, this.e35, this.e15, anti_reverse.e4235) * float4(anti_reverse.group7.xyz.xxy, this.e235)) - (float4(anti_reverse.group0.xy.xx, anti_reverse.scalar, anti_reverse.e23) * float4(this.group1.xyz, this.e235)) - (float4(this.group0.xy.xx, this.scalar, anti_reverse.e31) * float4(anti_reverse.group1.xyz, this.e315)) - (float4(anti_reverse.group1.zx, anti_reverse.e321, this.e415) * float4(this.group5.xyz.yzz, anti_reverse.e15)) - (float4(anti_reverse.group6.ww, anti_reverse.e2, this.e1) * float4(this.group5.xyz.xyx, anti_reverse.e15)) - float4((float3(anti_reverse.e1234) * this.group8.xyz), anti_reverse.e25 * this.e425) - float4((anti_reverse.group8.xyz.yzx * this.group4.xyz.zxy), anti_reverse.e4125 * this.e125) - float4((this.group7.xyz.zxy * anti_reverse.group3.yzx), anti_reverse.e25 * this.e2) - float4((anti_reverse.group1.www * this.group3.xyz), anti_reverse.e35 * this.e3) - float4((anti_reverse.group6.yzx * this.group9.zxy), anti_reverse.e35 * this.e435) - float4((anti_reverse.group9.yzx * this.group6.zxy), anti_reverse.e415 * this.e15), 
            /* e1234 */
            (anti_reverse.scalar * this.e4) + (anti_reverse.e12345 * this.e1234) + (anti_reverse.e4 * this.scalar) + (anti_reverse.e4 * this.e45) + (anti_reverse.e41 * this.e1) + (anti_reverse.e42 * this.e2) + (anti_reverse.e43 * this.e3) + (anti_reverse.e321 * this.e1234) + (anti_reverse.e4235 * this.e423) + (anti_reverse.e4315 * this.e431) + (anti_reverse.e4125 * this.e412) + (anti_reverse.e1234 * this.e12345) - (anti_reverse.e1 * this.e41) - (anti_reverse.e2 * this.e42) - (anti_reverse.e3 * this.e43) - (anti_reverse.e45 * this.e4) - (anti_reverse.e41 * this.e415) - (anti_reverse.e42 * this.e425) - (anti_reverse.e43 * this.e435) - (anti_reverse.e23 * this.e423) - (anti_reverse.e31 * this.e431) - (anti_reverse.e12 * this.e412) - (anti_reverse.e415 * this.e41) - (anti_reverse.e425 * this.e42) - (anti_reverse.e435 * this.e43) - (anti_reverse.e423 * this.e23) - (anti_reverse.e423 * this.e4235) - (anti_reverse.e431 * this.e31) - (anti_reverse.e431 * this.e4315) - (anti_reverse.e412 * this.e12) - (anti_reverse.e412 * this.e4125) - (anti_reverse.e1234 * this.e321)
        );
        return MultiVector(
            /* scalar, e12345 */
            float2(geometric_anti_product.scalar, 0.0), 
            /* e1, e2, e3, e4 */
            geometric_anti_product.group1, 
            /* e5 */
            geometric_anti_product.e5, 
            /* e15, e25, e35, e45 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(0.0), 
            /* e4235, e4315, e4125, e3215 */
            geometric_anti_product.group9, 
            /* e1234 */
            geometric_anti_product.e1234
        );
    }
}
// Fancy postfix trick
public VersorEven operator /(VersorEven lhs, AntiConstraintViolationPrefixOrPostfix rhs)  {
    return lhs.anti_constraint_violation();
    }
// Fancy prefix trick
public VersorEven operator /(AntiConstraintViolationPrefixOrPostfix lhs, VersorEven rhs) {
    return rhs.anti_constraint_violation();
}
public extension VersorEven {
    public VersorEven anti_constraint_violation() {
        VersorEven anti_reverse = VersorEven(
            /* e423, e431, e412, e12345 */
            this.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e415, e425, e435, e321 */
            this.group1 * float4(-1.0), 
            /* e235, e315, e125, e5 */
            this.group2 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e4 */
            this.group3
        );
        VersorEven geometric_anti_product = VersorEven(
            /* e423, e431, e412, e12345 */
            float4(float3(0.0), (anti_reverse.e12345 * this.e12345) + (anti_reverse.e321 * this.e321) + (anti_reverse.e5 * this.e4) + (anti_reverse.e4 * this.e5) - (anti_reverse.e423 * this.e235) - (anti_reverse.e431 * this.e315) - (anti_reverse.e412 * this.e125) - (anti_reverse.e415 * this.e415) - (anti_reverse.e425 * this.e425) - (anti_reverse.e435 * this.e435) - (anti_reverse.e235 * this.e423) - (anti_reverse.e315 * this.e431) - (anti_reverse.e125 * this.e412) - (anti_reverse.e1 * this.e1) - (anti_reverse.e2 * this.e2) - (anti_reverse.e3 * this.e3)), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e235, e315, e125, e5 */
            float4(float3(0.0), (anti_reverse.e12345 * this.e5) + (anti_reverse.e5 * this.e12345) + (anti_reverse.e5 * this.e321) + (anti_reverse.e1 * this.e235) + (anti_reverse.e2 * this.e315) + (anti_reverse.e3 * this.e125) - (anti_reverse.e415 * this.e235) - (anti_reverse.e425 * this.e315) - (anti_reverse.e435 * this.e125) - (anti_reverse.e321 * this.e5) - (anti_reverse.e235 * this.e415) - (anti_reverse.e235 * this.e1) - (anti_reverse.e315 * this.e425) - (anti_reverse.e315 * this.e2) - (anti_reverse.e125 * this.e435) - (anti_reverse.e125 * this.e3)), 
            /* e1, e2, e3, e4 */
            float4((anti_reverse.e315 * this.e412) + (anti_reverse.e1 * this.e12345) + (anti_reverse.e3 * this.e425) + (anti_reverse.e4 * this.e235), (anti_reverse.e125 * this.e423) + (anti_reverse.e1 * this.e435) + (anti_reverse.e2 * this.e12345) + (anti_reverse.e4 * this.e315), (anti_reverse.e235 * this.e431) + (anti_reverse.e2 * this.e415) + (anti_reverse.e3 * this.e12345) + (anti_reverse.e4 * this.e125), -(anti_reverse.e1 * this.e423) - (anti_reverse.e2 * this.e431) - (anti_reverse.e3 * this.e412) - (anti_reverse.e4 * this.e321)) + (float4(this.e321, this.e3, this.e1, this.e4) * float4(anti_reverse.group1.xxy, anti_reverse.e12345)) + (float4(this.e2, this.e321, this.e321, this.e4) * anti_reverse.group1.zyzw) + (anti_reverse.group0.xxyx * float4(this.group2.wzx, this.e1)) + (anti_reverse.group0.zyzy * float4(this.group2.yww, this.e2)) + (anti_reverse.group0.wwwz * this.group3.xyzz) + float4((anti_reverse.group1.www * this.group1.xyz), anti_reverse.e4 * this.e12345) - (float4(this.e431, this.e4, this.e4, this.e423) * float4(anti_reverse.group2.zyz, anti_reverse.e415)) - (float4(this.e4, this.e412, this.e423, this.e435) * float4(anti_reverse.group2.xxy, anti_reverse.e412)) - (anti_reverse.group0.yzxx * float4(this.group2.zxy, this.e415)) - (this.group0.xyzy * float4(anti_reverse.group2.www, anti_reverse.e425)) - float4((anti_reverse.group1.yzx * this.group3.zxy), anti_reverse.e431 * this.e425) - float4((anti_reverse.group3.yzx * this.group1.zxy), anti_reverse.e435 * this.e412)
        );
        return VersorEven(
            /* e423, e431, e412, e12345 */
            geometric_anti_product.group0 + float4(float3(0.0), pow(this.e321, 2) + pow(this.e1, 2) + pow(this.e2, 2) + pow(this.e3, 2) - pow(this.e12345, 2) - pow(this.e415, 2) - pow(this.e425, 2) - pow(this.e435, 2) - 2.0 * (this.e423 * this.e235) - 2.0 * (this.e431 * this.e315) - 2.0 * (this.e412 * this.e125) - 2.0 * (this.e5 * this.e4)), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e235, e315, e125, e5 */
            geometric_anti_product.group2, 
            /* e1, e2, e3, e4 */
            geometric_anti_product.group3
        );
    }
}
// Fancy postfix trick
public VersorEven operator /(VersorOdd lhs, AntiConstraintViolationPrefixOrPostfix rhs)  {
    return lhs.anti_constraint_violation();
    }
// Fancy prefix trick
public VersorEven operator /(AntiConstraintViolationPrefixOrPostfix lhs, VersorOdd rhs) {
    return rhs.anti_constraint_violation();
}
public extension VersorOdd {
    public VersorEven anti_constraint_violation() {
        VersorOdd anti_reverse = VersorOdd(
            /* e41, e42, e43, scalar */
            this.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, e45 */
            this.group1 * float4(-1.0), 
            /* e15, e25, e35, e1234 */
            this.group2 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group3
        );
        VersorEven geometric_anti_product = VersorEven(
            /* e423, e431, e412, e12345 */
            float4(float3(0.0), (anti_reverse.e41 * this.e15) + (anti_reverse.e42 * this.e25) + (anti_reverse.e43 * this.e35) + (anti_reverse.e23 * this.e23) + (anti_reverse.e31 * this.e31) + (anti_reverse.e12 * this.e12) + (anti_reverse.e15 * this.e41) + (anti_reverse.e25 * this.e42) + (anti_reverse.e35 * this.e43) + (anti_reverse.e4235 * this.e4235) + (anti_reverse.e4315 * this.e4315) + (anti_reverse.e4125 * this.e4125) - (anti_reverse.scalar * this.scalar) - (anti_reverse.e45 * this.e45) - (anti_reverse.e1234 * this.e3215) - (anti_reverse.e3215 * this.e1234)), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e235, e315, e125, e5 */
            float4(float3(0.0), (anti_reverse.e23 * this.e15) + (anti_reverse.e31 * this.e25) + (anti_reverse.e12 * this.e35) + (anti_reverse.e15 * this.e23) + (anti_reverse.e25 * this.e31) + (anti_reverse.e35 * this.e12) + (anti_reverse.e4235 * this.e15) + (anti_reverse.e4315 * this.e25) + (anti_reverse.e4125 * this.e35) + (anti_reverse.e3215 * this.e45) - (anti_reverse.scalar * this.e3215) - (anti_reverse.e45 * this.e3215) - (anti_reverse.e15 * this.e4235) - (anti_reverse.e25 * this.e4315) - (anti_reverse.e35 * this.e4125) - (anti_reverse.e3215 * this.scalar)), 
            /* e1, e2, e3, e4 */
            (float4(this.e42, this.e1234, this.e1234, this.e41) * float4(anti_reverse.group2.zyz, anti_reverse.e23)) + (float4(this.scalar, this.e12, this.e23, this.e42) * float4(anti_reverse.group3.xxy, anti_reverse.e31)) + (float4(this.e31, this.scalar, this.scalar, this.e43) * float4(anti_reverse.group3.zyz, anti_reverse.e12)) + (float4(this.e45, this.e4125, this.e4235, this.e31) * float4(anti_reverse.group1.xxy, anti_reverse.e42)) + (float4(this.e1234, this.e43, this.e41, this.e4125) * float4(anti_reverse.group2.xxy, anti_reverse.e43)) + (float4(this.e4315, this.e45, this.e45, this.e4315) * float4(anti_reverse.group1.zyz, anti_reverse.e42)) + (anti_reverse.group0.yzxx * float4(this.group2.zxy, this.e23)) + (anti_reverse.group0.wwwx * this.group3.xyzx) + (this.group1.xyzz * float4(anti_reverse.group1.www, anti_reverse.e43)) + float4((anti_reverse.group3.www * this.group0.xyz), anti_reverse.e45 * this.e1234) - (float4(this.e25, this.e3215, this.e3215, this.scalar) * float4(anti_reverse.group0.zyz, anti_reverse.e1234)) - (float4(this.e3215, this.e35, this.e15, this.e1234) * anti_reverse.group0.xxyw) - (anti_reverse.group3.yzxz * float4(this.group1.zxy, this.e43)) - (this.group0.zxyx * float4(anti_reverse.group2.yzx, anti_reverse.e4235)) - float4((anti_reverse.group1.yzx * this.group3.zxy), anti_reverse.e1234 * this.e45) - float4((anti_reverse.group2.www * this.group2.xyz), anti_reverse.e4315 * this.e42)
        );
        return VersorEven(
            /* e423, e431, e412, e12345 */
            geometric_anti_product.group0 + float4(float3(0.0), 2.0*(this.e41 * this.e15) + 2.0 * (this.e42 * this.e25) + 2.0 * (this.e43 * this.e35) + 2.0 * (this.e1234 * this.e3215) + pow(this.scalar, 2) + pow(this.e23, 2) + pow(this.e31, 2) + pow(this.e12, 2) - pow(this.e45, 2) - pow(this.e4235, 2) - pow(this.e4315, 2) - pow(this.e4125, 2)), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e235, e315, e125, e5 */
            geometric_anti_product.group2, 
            /* e1, e2, e3, e4 */
            geometric_anti_product.group3
        );
    }
}

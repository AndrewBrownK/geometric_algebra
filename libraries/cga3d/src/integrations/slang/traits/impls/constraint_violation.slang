implementing cga3d;
using traits;
public extension AntiCircleRotor {
    // Fancy postfix trick
    public VersorOdd operator /(ConstraintViolationPrefixOrPostfix rhs)  {
        return this.constraint_violation();
    }
}
public extension ConstraintViolationPrefixOrPostfix {
    // Fancy prefix trick
    public VersorOdd operator /(AntiCircleRotor rhs) {
        return rhs.constraint_violation();
    }
}
public extension AntiCircleRotor {
    public VersorOdd constraint_violation() {
        AntiCircleRotor reverse = AntiCircleRotor(
            /* e41, e42, e43 */
            this.group0.xyz * float3(-1.0), 
            /* e23, e31, e12, e45 */
            this.group1 * float4(-1.0), 
            /* e15, e25, e35, scalar */
            this.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        VersorOdd geometric_product = VersorOdd(
            /* e41, e42, e43, scalar */
            float4(float3(0.0), (reverse.e45 * this.e45) + (reverse.scalar * this.scalar) - (reverse.e41 * this.e15) - (reverse.e42 * this.e25) - (reverse.e43 * this.e35) - (reverse.e23 * this.e23) - (reverse.e31 * this.e31) - (reverse.e12 * this.e12) - (reverse.e15 * this.e41) - (reverse.e25 * this.e42) - (reverse.e35 * this.e43)), 
            /* e23, e31, e12, e45 */
            float4(0.0), 
            /* e15, e25, e35, e1234 */
            float4(float3(0.0), -(reverse.e41 * this.e23) - (reverse.e42 * this.e31) - (reverse.e43 * this.e12) - (reverse.e23 * this.e41) - (reverse.e31 * this.e42) - (reverse.e12 * this.e43)), 
            /* e4235, e4315, e4125, e3215 */
            float4((reverse.e42 * this.e35) + (reverse.e23 * this.e45) + (reverse.e45 * this.e23) + (reverse.e35 * this.e42), (reverse.e43 * this.e15) + (reverse.e31 * this.e45) + (reverse.e45 * this.e31) + (reverse.e15 * this.e43), (reverse.e41 * this.e25) + (reverse.e12 * this.e45) + (reverse.e45 * this.e12) + (reverse.e25 * this.e41), -(reverse.e12 * this.e35) - (reverse.e15 * this.e23) - (reverse.e25 * this.e31) - (reverse.e35 * this.e12)) - (this.group2.yzxx * float4(reverse.group0.xyz.zxy, reverse.e23)) - float4((this.group0.xyz.zxy * reverse.group2.yzx), reverse.e31 * this.e25)
        );
        return VersorOdd(
            /* e41, e42, e43, scalar */
            geometric_product.group0 + float4(float3(0.0), pow(this.e45, 2) - pow(this.e23, 2) - pow(this.e31, 2) - pow(this.e12, 2) - pow(this.scalar, 2) - 2.0 * (this.e41 * this.e15) - 2.0 * (this.e42 * this.e25) - 2.0 * (this.e43 * this.e35)), 
            /* e23, e31, e12, e45 */
            float4(0.0), 
            /* e15, e25, e35, e1234 */
            geometric_product.group2, 
            /* e4235, e4315, e4125, e3215 */
            geometric_product.group3
        );
    }
}
public extension AntiDipoleInversion {
    // Fancy postfix trick
    public VersorOdd operator /(ConstraintViolationPrefixOrPostfix rhs)  {
        return this.constraint_violation();
    }
}
public extension ConstraintViolationPrefixOrPostfix {
    // Fancy prefix trick
    public VersorOdd operator /(AntiDipoleInversion rhs) {
        return rhs.constraint_violation();
    }
}
public extension AntiDipoleInversion {
    public VersorOdd constraint_violation() {
        AntiDipoleInversion reverse = AntiDipoleInversion(
            /* e423, e431, e412 */
            this.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            this.group1 * float4(-1.0), 
            /* e235, e315, e125, e4 */
            this.group2 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            this.group3
        );
        VersorOdd geometric_product = VersorOdd(
            /* e41, e42, e43, scalar */
            float4(float3(0.0), (reverse.e423 * this.e235) + (reverse.e431 * this.e315) + (reverse.e412 * this.e125) + (reverse.e415 * this.e415) + (reverse.e425 * this.e425) + (reverse.e435 * this.e435) + (reverse.e235 * this.e423) + (reverse.e315 * this.e431) + (reverse.e125 * this.e412) + (reverse.e1 * this.e1) + (reverse.e2 * this.e2) + (reverse.e3 * this.e3) - (reverse.e321 * this.e321) - (reverse.e4 * this.e5) - (reverse.e5 * this.e4)), 
            /* e23, e31, e12, e45 */
            float4(0.0), 
            /* e15, e25, e35, e1234 */
            float4(float3(0.0), (reverse.e423 * this.e415) + (reverse.e423 * this.e1) + (reverse.e431 * this.e425) + (reverse.e431 * this.e2) + (reverse.e412 * this.e435) + (reverse.e412 * this.e3) + (reverse.e415 * this.e423) + (reverse.e425 * this.e431) + (reverse.e435 * this.e412) + (reverse.e321 * this.e4) - (reverse.e4 * this.e321) - (reverse.e1 * this.e423) - (reverse.e2 * this.e431) - (reverse.e3 * this.e412)), 
            /* e4235, e4315, e4125, e3215 */
            float4(-(reverse.e4 * this.e235) - (reverse.e3 * this.e425), -(reverse.e4 * this.e315) - (reverse.e1 * this.e435), -(reverse.e4 * this.e125) - (reverse.e2 * this.e415), (reverse.e3 * this.e125) + (reverse.e5 * this.e321)) + (float4(reverse.e315, reverse.e5, reverse.e5, this.e125) * float4(this.group0.xyz.zyz, reverse.e435)) + (float4(reverse.e5, reverse.e125, reverse.e235, this.e315) * float4(this.group0.xyz.xxy, reverse.e425)) + (reverse.group3.yzxy * float4(this.group1.zxy, this.e315)) + (this.group1.xyzz * float4(reverse.group1.www, reverse.e125)) + (this.group2.yzxx * float4(reverse.group0.xyz.zxy, reverse.e415)) + (this.group2.wwwx * float4(reverse.group2.xyz, reverse.e1)) + (float4(this.group1.ww, this.e2, this.e415) * float4(reverse.group1.xyx, reverse.e235)) + (float4(this.group3.zx, this.e321, this.e425) * float4(reverse.group1.yzz, reverse.e315)) - (this.group3.yzxz * float4(reverse.group1.zxy, reverse.e125)) - (float4(this.group2.zx, this.e5, this.e1) * float4(reverse.group0.xyz.yzz, reverse.e235)) - (float4(this.group3.ww, this.e315, this.e5) * float4(reverse.group0.xyz.xyx, reverse.e321)) - float4((this.group0.xyz.yzx * reverse.group2.zxy), reverse.e315 * this.e2)
        );
        return VersorOdd(
            /* e41, e42, e43, scalar */
            geometric_product.group0 + float4(float3(0.0), 2.0*(this.e423 * this.e235) + 2.0 * (this.e431 * this.e315) + 2.0 * (this.e412 * this.e125) + 2.0 * (this.e4 * this.e5) + pow(this.e415, 2) + pow(this.e425, 2) + pow(this.e435, 2) - pow(this.e321, 2) - pow(this.e1, 2) - pow(this.e2, 2) - pow(this.e3, 2)), 
            /* e23, e31, e12, e45 */
            float4(0.0), 
            /* e15, e25, e35, e1234 */
            geometric_product.group2, 
            /* e4235, e4315, e4125, e3215 */
            geometric_product.group3
        );
    }
}
public extension AntiDualNum {
    // Fancy postfix trick
    public AntiDualNum operator /(ConstraintViolationPrefixOrPostfix rhs)  {
        return this.constraint_violation();
    }
}
public extension ConstraintViolationPrefixOrPostfix {
    // Fancy prefix trick
    public AntiDualNum operator /(AntiDualNum rhs) {
        return rhs.constraint_violation();
    }
}
public extension AntiDualNum {
    public AntiDualNum constraint_violation() {
        return AntiDualNum(
            /* e3215, scalar */
            float2(this.e3215 * this.scalar * 2.0, 0.0)
        );
    }
}
public extension AntiFlector {
    // Fancy postfix trick
    public AntiDualNum operator /(ConstraintViolationPrefixOrPostfix rhs)  {
        return this.constraint_violation();
    }
}
public extension ConstraintViolationPrefixOrPostfix {
    // Fancy prefix trick
    public AntiDualNum operator /(AntiFlector rhs) {
        return rhs.constraint_violation();
    }
}
public extension AntiFlector {
    public AntiDualNum constraint_violation() {
        AntiFlector reverse = AntiFlector(
            /* e235, e315, e125, e321 */
            this.group0 * float4(-1.0), 
            /* e1, e2, e3, e5 */
            this.group1
        );
        return AntiDualNum(
            /* e3215, scalar */
            float2((reverse.e1 * this.e235) + (reverse.e2 * this.e315) + (reverse.e3 * this.e125) + (reverse.e5 * this.e321) - (reverse.e235 * this.e1) - (reverse.e315 * this.e2) - (reverse.e125 * this.e3) - (reverse.e321 * this.e5), 0.0)
        );
    }
}
public extension AntiLine {
    // Fancy postfix trick
    public AntiDualNum operator /(ConstraintViolationPrefixOrPostfix rhs)  {
        return this.constraint_violation();
    }
}
public extension ConstraintViolationPrefixOrPostfix {
    // Fancy prefix trick
    public AntiDualNum operator /(AntiLine rhs) {
        return rhs.constraint_violation();
    }
}
public extension AntiLine {
    public AntiDualNum constraint_violation() {
        AntiLine reverse = AntiLine(
            /* e23, e31, e12 */
            this.group0.xyz * float3(-1.0), 
            /* e15, e25, e35 */
            this.group1.xyz * float3(-1.0)
        );
        return AntiDualNum(
            /* e3215, scalar */
            float2(-(reverse.e23 * this.e15) - (reverse.e31 * this.e25) - (reverse.e12 * this.e35) - (reverse.e15 * this.e23) - (reverse.e25 * this.e31) - (reverse.e35 * this.e12), 0.0)
        );
    }
}
public extension AntiMotor {
    // Fancy postfix trick
    public AntiDualNum operator /(ConstraintViolationPrefixOrPostfix rhs)  {
        return this.constraint_violation();
    }
}
public extension ConstraintViolationPrefixOrPostfix {
    // Fancy prefix trick
    public AntiDualNum operator /(AntiMotor rhs) {
        return rhs.constraint_violation();
    }
}
public extension AntiMotor {
    public AntiDualNum constraint_violation() {
        AntiMotor reverse = AntiMotor(
            /* e23, e31, e12, scalar */
            this.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e15, e25, e35, e3215 */
            this.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiDualNum(
            /* e3215, scalar */
            float2((reverse.scalar * this.e3215) + (reverse.e3215 * this.scalar) - (reverse.e23 * this.e15) - (reverse.e31 * this.e25) - (reverse.e12 * this.e35) - (reverse.e15 * this.e23) - (reverse.e25 * this.e31) - (reverse.e35 * this.e12), 0.0)
        );
    }
}
public extension Circle {
    // Fancy postfix trick
    public VersorOdd operator /(ConstraintViolationPrefixOrPostfix rhs)  {
        return this.constraint_violation();
    }
}
public extension ConstraintViolationPrefixOrPostfix {
    // Fancy prefix trick
    public VersorOdd operator /(Circle rhs) {
        return rhs.constraint_violation();
    }
}
public extension Circle {
    public VersorOdd constraint_violation() {
        Circle reverse = Circle(
            /* e423, e431, e412 */
            this.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            this.group1 * float4(-1.0), 
            /* e235, e315, e125 */
            this.group2.xyz * float3(-1.0)
        );
        VersorOdd geometric_product = VersorOdd(
            /* e41, e42, e43, scalar */
            float4(float3(0.0), (reverse.e423 * this.e235) + (reverse.e431 * this.e315) + (reverse.e412 * this.e125) + (reverse.e415 * this.e415) + (reverse.e425 * this.e425) + (reverse.e435 * this.e435) + (reverse.e235 * this.e423) + (reverse.e315 * this.e431) + (reverse.e125 * this.e412) - (reverse.e321 * this.e321)), 
            /* e23, e31, e12, e45 */
            float4(0.0), 
            /* e15, e25, e35, e1234 */
            float4(float3(0.0), (reverse.e423 * this.e415) + (reverse.e431 * this.e425) + (reverse.e412 * this.e435) + (reverse.e415 * this.e423) + (reverse.e425 * this.e431) + (reverse.e435 * this.e412)), 
            /* e4235, e4315, e4125, e3215 */
            float4(-(reverse.e431 * this.e125) - (reverse.e125 * this.e431), -(reverse.e412 * this.e235) - (reverse.e235 * this.e412), -(reverse.e423 * this.e315) - (reverse.e315 * this.e423), (reverse.e425 * this.e315) + (reverse.e435 * this.e125)) + (this.group1.wwwz * float4(reverse.group1.xyz, reverse.e125)) + float4((reverse.group0.xyz.zxy * this.group2.xyz.yzx), reverse.e235 * this.e415) + float4((reverse.group2.xyz.yzx * this.group0.xyz.zxy), reverse.e315 * this.e425) + float4((reverse.group1.www * this.group1.xyz), reverse.e415 * this.e235)
        );
        return VersorOdd(
            /* e41, e42, e43, scalar */
            geometric_product.group0 + float4(float3(0.0), 2.0*(this.e423 * this.e235) + 2.0 * (this.e431 * this.e315) + 2.0 * (this.e412 * this.e125) + pow(this.e415, 2) + pow(this.e425, 2) + pow(this.e435, 2) - pow(this.e321, 2)), 
            /* e23, e31, e12, e45 */
            float4(0.0), 
            /* e15, e25, e35, e1234 */
            geometric_product.group2, 
            /* e4235, e4315, e4125, e3215 */
            geometric_product.group3
        );
    }
}
public extension CircleRotor {
    // Fancy postfix trick
    public VersorOdd operator /(ConstraintViolationPrefixOrPostfix rhs)  {
        return this.constraint_violation();
    }
}
public extension ConstraintViolationPrefixOrPostfix {
    // Fancy prefix trick
    public VersorOdd operator /(CircleRotor rhs) {
        return rhs.constraint_violation();
    }
}
public extension CircleRotor {
    public VersorOdd constraint_violation() {
        CircleRotor reverse = CircleRotor(
            /* e423, e431, e412 */
            this.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            this.group1 * float4(-1.0), 
            /* e235, e315, e125, e12345 */
            this.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        VersorOdd geometric_product = VersorOdd(
            /* e41, e42, e43, scalar */
            float4(float3(0.0), (reverse.e423 * this.e235) + (reverse.e431 * this.e315) + (reverse.e412 * this.e125) + (reverse.e415 * this.e415) + (reverse.e425 * this.e425) + (reverse.e435 * this.e435) + (reverse.e235 * this.e423) + (reverse.e315 * this.e431) + (reverse.e125 * this.e412) - (reverse.e321 * this.e321) - (reverse.e12345 * this.e12345)), 
            /* e23, e31, e12, e45 */
            float4(0.0), 
            /* e15, e25, e35, e1234 */
            float4(float3(0.0), (reverse.e423 * this.e415) + (reverse.e431 * this.e425) + (reverse.e412 * this.e435) + (reverse.e415 * this.e423) + (reverse.e425 * this.e431) + (reverse.e435 * this.e412)), 
            /* e4235, e4315, e4125, e3215 */
            float4(-(reverse.e431 * this.e125) - (reverse.e125 * this.e431), -(reverse.e412 * this.e235) - (reverse.e235 * this.e412), -(reverse.e423 * this.e315) - (reverse.e315 * this.e423), (reverse.e315 * this.e425) + (reverse.e125 * this.e435)) + (reverse.group1.xyzz * float4(this.group1.www, this.e125)) + (this.group1.xyzx * float4(reverse.group1.www, reverse.e235)) + (this.group2.yzxx * float4(reverse.group0.xyz.zxy, reverse.e415)) + float4((this.group0.xyz.zxy * reverse.group2.yzx), reverse.e425 * this.e315)
        );
        return VersorOdd(
            /* e41, e42, e43, scalar */
            geometric_product.group0 + float4(float3(0.0), 2.0*(this.e423 * this.e235) + 2.0 * (this.e431 * this.e315) + 2.0 * (this.e412 * this.e125) + pow(this.e415, 2) + pow(this.e425, 2) + pow(this.e435, 2) + pow(this.e12345, 2) - pow(this.e321, 2)), 
            /* e23, e31, e12, e45 */
            float4(0.0), 
            /* e15, e25, e35, e1234 */
            geometric_product.group2, 
            /* e4235, e4315, e4125, e3215 */
            geometric_product.group3
        );
    }
}
public extension Dipole {
    // Fancy postfix trick
    public VersorOdd operator /(ConstraintViolationPrefixOrPostfix rhs)  {
        return this.constraint_violation();
    }
}
public extension ConstraintViolationPrefixOrPostfix {
    // Fancy prefix trick
    public VersorOdd operator /(Dipole rhs) {
        return rhs.constraint_violation();
    }
}
public extension Dipole {
    public VersorOdd constraint_violation() {
        Dipole reverse = Dipole(
            /* e41, e42, e43 */
            this.group0.xyz * float3(-1.0), 
            /* e23, e31, e12, e45 */
            this.group1 * float4(-1.0), 
            /* e15, e25, e35 */
            this.group2.xyz * float3(-1.0)
        );
        VersorOdd geometric_product = VersorOdd(
            /* e41, e42, e43, scalar */
            float4(float3(0.0), (reverse.e45 * this.e45) - (reverse.e41 * this.e15) - (reverse.e42 * this.e25) - (reverse.e43 * this.e35) - (reverse.e23 * this.e23) - (reverse.e31 * this.e31) - (reverse.e12 * this.e12) - (reverse.e15 * this.e41) - (reverse.e25 * this.e42) - (reverse.e35 * this.e43)), 
            /* e23, e31, e12, e45 */
            float4(0.0), 
            /* e15, e25, e35, e1234 */
            float4(float3(0.0), -(reverse.e41 * this.e23) - (reverse.e42 * this.e31) - (reverse.e43 * this.e12) - (reverse.e23 * this.e41) - (reverse.e31 * this.e42) - (reverse.e12 * this.e43)), 
            /* e4235, e4315, e4125, e3215 */
            float4((reverse.e42 * this.e35) + (reverse.e23 * this.e45) + (reverse.e45 * this.e23) + (reverse.e35 * this.e42), (reverse.e43 * this.e15) + (reverse.e31 * this.e45) + (reverse.e45 * this.e31) + (reverse.e15 * this.e43), (reverse.e41 * this.e25) + (reverse.e12 * this.e45) + (reverse.e45 * this.e12) + (reverse.e25 * this.e41), -(reverse.e23 * this.e15) - (reverse.e31 * this.e25) - (reverse.e12 * this.e35) - (reverse.e35 * this.e12)) - float4((reverse.group0.xyz.zxy * this.group2.xyz.yzx), reverse.e15 * this.e23) - float4((reverse.group2.xyz.yzx * this.group0.xyz.zxy), reverse.e25 * this.e31)
        );
        return VersorOdd(
            /* e41, e42, e43, scalar */
            geometric_product.group0 + float4(float3(0.0), pow(this.e45, 2) - pow(this.e23, 2) - pow(this.e31, 2) - pow(this.e12, 2) - 2.0 * (this.e41 * this.e15) - 2.0 * (this.e42 * this.e25) - 2.0 * (this.e43 * this.e35)), 
            /* e23, e31, e12, e45 */
            float4(0.0), 
            /* e15, e25, e35, e1234 */
            geometric_product.group2, 
            /* e4235, e4315, e4125, e3215 */
            geometric_product.group3
        );
    }
}
public extension DipoleInversion {
    // Fancy postfix trick
    public VersorOdd operator /(ConstraintViolationPrefixOrPostfix rhs)  {
        return this.constraint_violation();
    }
}
public extension ConstraintViolationPrefixOrPostfix {
    // Fancy prefix trick
    public VersorOdd operator /(DipoleInversion rhs) {
        return rhs.constraint_violation();
    }
}
public extension DipoleInversion {
    public VersorOdd constraint_violation() {
        DipoleInversion reverse = DipoleInversion(
            /* e41, e42, e43 */
            this.group0.xyz * float3(-1.0), 
            /* e23, e31, e12, e45 */
            this.group1 * float4(-1.0), 
            /* e15, e25, e35, e1234 */
            this.group2 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group3
        );
        VersorOdd geometric_product = VersorOdd(
            /* e41, e42, e43, scalar */
            float4(float3(0.0), (reverse.e45 * this.e45) + (reverse.e1234 * this.e3215) + (reverse.e3215 * this.e1234) - (reverse.e41 * this.e15) - (reverse.e42 * this.e25) - (reverse.e43 * this.e35) - (reverse.e23 * this.e23) - (reverse.e31 * this.e31) - (reverse.e12 * this.e12) - (reverse.e15 * this.e41) - (reverse.e25 * this.e42) - (reverse.e35 * this.e43) - (reverse.e4235 * this.e4235) - (reverse.e4315 * this.e4315) - (reverse.e4125 * this.e4125)), 
            /* e23, e31, e12, e45 */
            float4(0.0), 
            /* e15, e25, e35, e1234 */
            float4(float3(0.0), (reverse.e41 * this.e4235) + (reverse.e42 * this.e4315) + (reverse.e43 * this.e4125) + (reverse.e45 * this.e1234) - (reverse.e41 * this.e23) - (reverse.e42 * this.e31) - (reverse.e43 * this.e12) - (reverse.e23 * this.e41) - (reverse.e31 * this.e42) - (reverse.e12 * this.e43) - (reverse.e1234 * this.e45) - (reverse.e4235 * this.e41) - (reverse.e4315 * this.e42) - (reverse.e4125 * this.e43)), 
            /* e4235, e4315, e4125, e3215 */
            float4((reverse.e31 * this.e4125) + (reverse.e45 * this.e23) + (reverse.e1234 * this.e15) + (reverse.e4315 * this.e12), (reverse.e12 * this.e4235) + (reverse.e45 * this.e31) + (reverse.e1234 * this.e25) + (reverse.e4125 * this.e23), (reverse.e12 * this.e45) + (reverse.e45 * this.e12) + (reverse.e1234 * this.e35) + (reverse.e4235 * this.e31), -(reverse.e25 * this.e31) - (reverse.e25 * this.e4315) - (reverse.e35 * this.e12) - (reverse.e35 * this.e4125)) + (float4(this.group1.ww, this.e4315, this.e45) * float4(reverse.group1.xyx, reverse.e3215)) + (float4(this.group2.zx, this.e3215, this.e25) * float4(reverse.group0.xyz.yzz, reverse.e4315)) + (float4(this.group3.ww, this.e25, this.e15) * float4(reverse.group0.xyz.xyx, reverse.e4235)) + float4((this.group0.xyz.yzx * reverse.group2.zxy), reverse.e4125 * this.e35) - (float4(reverse.e25, reverse.e3215, reverse.e3215, this.e35) * float4(this.group0.xyz.zyz, reverse.e12)) - (float4(reverse.e3215, reverse.e35, reverse.e15, this.e25) * float4(this.group0.xyz.xxy, reverse.e31)) - (reverse.group1.zxyw * this.group3.yzxw) - (reverse.group2.xyzx * float4(this.group2.www, this.e23)) - (this.group2.yzxx * float4(reverse.group0.xyz.zxy, reverse.e23)) - float4((reverse.group3.zxy * this.group1.yzx), reverse.e15 * this.e4235)
        );
        return VersorOdd(
            /* e41, e42, e43, scalar */
            geometric_product.group0 + float4(float3(0.0), pow(this.e45, 2) + pow(this.e4235, 2) + pow(this.e4315, 2) + pow(this.e4125, 2) - pow(this.e23, 2) - pow(this.e31, 2) - pow(this.e12, 2) - 2.0 * (this.e41 * this.e15) - 2.0 * (this.e42 * this.e25) - 2.0 * (this.e43 * this.e35) - 2.0 * (this.e1234 * this.e3215)), 
            /* e23, e31, e12, e45 */
            float4(0.0), 
            /* e15, e25, e35, e1234 */
            geometric_product.group2, 
            /* e4235, e4315, e4125, e3215 */
            geometric_product.group3
        );
    }
}
public extension DualNum {
    // Fancy postfix trick
    public AntiDualNum operator /(ConstraintViolationPrefixOrPostfix rhs)  {
        return this.constraint_violation();
    }
}
public extension ConstraintViolationPrefixOrPostfix {
    // Fancy prefix trick
    public AntiDualNum operator /(DualNum rhs) {
        return rhs.constraint_violation();
    }
}
public extension DualNum {
    public AntiDualNum constraint_violation() {
        return AntiDualNum(
            /* e3215, scalar */
            float2(this.e5 * this.e12345 * -2.0, 0.0)
        );
    }
}
public extension Flector {
    // Fancy postfix trick
    public AntiDualNum operator /(ConstraintViolationPrefixOrPostfix rhs)  {
        return this.constraint_violation();
    }
}
public extension ConstraintViolationPrefixOrPostfix {
    // Fancy prefix trick
    public AntiDualNum operator /(Flector rhs) {
        return rhs.constraint_violation();
    }
}
public extension Flector {
    public AntiDualNum constraint_violation() {
        Flector reverse = Flector(
            /* e15, e25, e35, e45 */
            this.group0 * float4(-1.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group1
        );
        return AntiDualNum(
            /* e3215, scalar */
            float2((reverse.e4235 * this.e15) + (reverse.e4315 * this.e25) + (reverse.e4125 * this.e35) + (reverse.e3215 * this.e45) - (reverse.e15 * this.e4235) - (reverse.e25 * this.e4315) - (reverse.e35 * this.e4125) - (reverse.e45 * this.e3215), 0.0)
        );
    }
}
public extension Line {
    // Fancy postfix trick
    public AntiDualNum operator /(ConstraintViolationPrefixOrPostfix rhs)  {
        return this.constraint_violation();
    }
}
public extension ConstraintViolationPrefixOrPostfix {
    // Fancy prefix trick
    public AntiDualNum operator /(Line rhs) {
        return rhs.constraint_violation();
    }
}
public extension Line {
    public AntiDualNum constraint_violation() {
        Line reverse = Line(
            /* e415, e425, e435 */
            this.group0.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            this.group1.xyz * float3(-1.0)
        );
        return AntiDualNum(
            /* e3215, scalar */
            float2((reverse.e415 * this.e235) + (reverse.e425 * this.e315) + (reverse.e435 * this.e125) + (reverse.e235 * this.e415) + (reverse.e315 * this.e425) + (reverse.e125 * this.e435), 0.0)
        );
    }
}
public extension Motor {
    // Fancy postfix trick
    public AntiDualNum operator /(ConstraintViolationPrefixOrPostfix rhs)  {
        return this.constraint_violation();
    }
}
public extension ConstraintViolationPrefixOrPostfix {
    // Fancy prefix trick
    public AntiDualNum operator /(Motor rhs) {
        return rhs.constraint_violation();
    }
}
public extension Motor {
    public AntiDualNum constraint_violation() {
        Motor reverse = Motor(
            /* e415, e425, e435, e12345 */
            this.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            this.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiDualNum(
            /* e3215, scalar */
            float2((reverse.e415 * this.e235) + (reverse.e425 * this.e315) + (reverse.e435 * this.e125) + (reverse.e235 * this.e415) + (reverse.e315 * this.e425) + (reverse.e125 * this.e435) - (reverse.e12345 * this.e5) - (reverse.e5 * this.e12345), 0.0)
        );
    }
}
public extension MultiVector {
    // Fancy postfix trick
    public MultiVector operator /(ConstraintViolationPrefixOrPostfix rhs)  {
        return this.constraint_violation();
    }
}
public extension ConstraintViolationPrefixOrPostfix {
    // Fancy prefix trick
    public MultiVector operator /(MultiVector rhs) {
        return rhs.constraint_violation();
    }
}
public extension MultiVector {
    public MultiVector constraint_violation() {
        MultiVector reverse = MultiVector(
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
        MultiVector geometric_product = MultiVector(
            /* scalar, e12345 */
            float2((reverse.e45 * this.e45) + (reverse.e415 * this.e415) + (reverse.e425 * this.e425) + (reverse.e435 * this.e435) + (reverse.e423 * this.e235) + (reverse.e431 * this.e315) + (reverse.e412 * this.e125) + (reverse.e235 * this.e423) + (reverse.e315 * this.e431) + (reverse.e125 * this.e412) - (reverse.e12345 * this.e12345) - (reverse.e4 * this.e5) - (reverse.e5 * this.e4) - (reverse.e4235 * this.e4235) - (reverse.e4315 * this.e4315) - (reverse.e4125 * this.e4125), (reverse.e12345 * this.scalar) + (reverse.e5 * this.e1234) + (reverse.e4315 * this.e2) + (reverse.e4125 * this.e3) + (reverse.e3215 * this.e4) + (reverse.e1234 * this.e5) - (reverse.e15 * this.e423) - (reverse.e25 * this.e431) - (reverse.e35 * this.e412) - (reverse.e45 * this.e321) - (reverse.e425 * this.e31) - (reverse.e435 * this.e12) - (reverse.e321 * this.e45) - (reverse.e423 * this.e15) - (reverse.e431 * this.e25) - (reverse.e412 * this.e35)) + (float2(reverse.scalar) * this.group0.xy) + (float2(reverse.e1) * float2(this.e1, this.e4235)) + (float2(reverse.e2) * float2(this.e2, this.e4315)) + (float2(reverse.e3) * float2(this.e3, this.e4125)) + (float2(this.e3215) * float2(reverse.e1234, reverse.e4)) + (float2(this.e1234, this.e1) * reverse.group9.wx) - (float2(reverse.e41) * float2(this.e15, this.e235)) - (float2(reverse.e42) * float2(this.e25, this.e315)) - (float2(reverse.e43) * float2(this.e35, this.e125)) - (float2(reverse.e23) * float2(this.e23, this.e415)) - (float2(reverse.e31) * float2(this.e31, this.e425)) - (float2(reverse.e12) * float2(this.e12, this.e435)) - (float2(this.e41) * float2(reverse.e15, reverse.e235)) - (float2(this.e42) * float2(reverse.e25, reverse.e315)) - (float2(this.e43) * float2(reverse.e35, reverse.e125)) - (float2(this.e321, this.e23) * reverse.group6.wx), 
            /* e1, e2, e3, e4 */
            float4((reverse.e15 * this.e4) + (reverse.e25 * this.e412) + (reverse.e435 * this.e4315) + (reverse.e4125 * this.e425), (reverse.e25 * this.e4) + (reverse.e415 * this.e4125) + (reverse.e4235 * this.e435) + (reverse.e3215 * this.e431), (reverse.e35 * this.e4) + (reverse.e425 * this.e4235) + (reverse.e4315 * this.e415) + (reverse.e3215 * this.e412), -(reverse.e4 * this.e45) - (reverse.e321 * this.e1234) - (reverse.e4315 * this.e431) - (reverse.e4125 * this.e412)) + (float4(reverse.scalar) * this.group1) + (float4(reverse.e2, reverse.e321, reverse.e321, this.e4) * float4(this.group5.xyz.zyz, reverse.e45)) + (float4(reverse.e321, reverse.e3, reverse.e1, reverse.e3) * float4(this.group5.xyz.xxy, this.e43)) + (float4(reverse.e3215, reverse.e35, reverse.e15, reverse.e1234) * float4(this.group7.xyz.xxy, this.e321)) + (float4(float2(this.e1234), this.e42, this.e4125) * float4(reverse.group8.xyz.xyx, reverse.e412)) + (float4(this.group0.xy.xx, this.scalar, reverse.e12345) * float4(reverse.group1.xyz, this.e1234)) + (float4(this.group4.xyz.zx, this.e1234, reverse.e1) * float4(reverse.group8.xyz.yzz, this.e41)) + (float4(this.group1.zx, this.e321, this.e4235) * float4(reverse.group5.xyz.yzz, reverse.e423)) + (float4(this.group6.ww, this.e2, reverse.e1234) * float4(reverse.group5.xyz.xyx, this.e12345)) + float4((float3(reverse.e5) * this.group4.xyz), reverse.e2 * this.e42) + float4((reverse.group4.xyz.zxy * this.group8.xyz.yzx), reverse.e4 * this.scalar) + float4((reverse.group7.xyz.zxy * this.group3.yzx), reverse.e431 * this.e4315) - (this.group1.yzxz * float4(reverse.group5.xyz.zxy, reverse.e43)) - (float4(float2(this.e5), this.e315, this.e2) * float4(reverse.group4.xyz.xyx, reverse.e42)) - (float4(reverse.group0.xy.yy, reverse.e12345, reverse.e41) * float4(this.group9.xyz, this.e1)) - (float4(this.group0.xy.yy, this.e12345, reverse.e41) * float4(reverse.group9.xyz, this.e415)) - (float4(this.group8.xyz.zx, this.e5, this.e425) * float4(reverse.group4.xyz.yzz, reverse.e42)) - (float4(this.group3.zx, this.e3215, this.e423) * float4(reverse.group7.xyz.yzz, reverse.e23)) - (float4(this.group3.ww, this.e4315, reverse.e425) * float4(reverse.group6.xyx, this.e42)) - (float4(this.group9.zx, this.e45, reverse.e435) * float4(reverse.group6.yzz, this.e43)) - (float4(this.group9.ww, this.e25, this.e435) * float4(reverse.group7.xyz.xyx, reverse.e43)) - float4((float3(reverse.e1234) * this.group8.xyz), reverse.e431 * this.e31) - float4((reverse.group8.xyz.zxy * this.group4.xyz.yzx), reverse.e31 * this.e431) - float4((this.group5.xyz.yzx * reverse.group1.zxy), reverse.e12 * this.e412) - float4((this.group7.xyz.yzx * reverse.group3.zxy), reverse.e423 * this.e23) - float4((reverse.group1.www * this.group3.xyz), reverse.e412 * this.e12) - float4((reverse.group3.www * this.group6.xyz), reverse.e415 * this.e41) - float4((reverse.group9.yzx * this.group6.zxy), reverse.e4235 * this.e423), 
            /* e5 */
            (reverse.scalar * this.e5) + (reverse.e12345 * this.e3215) + (reverse.e5 * this.scalar) + (reverse.e5 * this.e45) + (reverse.e15 * this.e1) + (reverse.e25 * this.e2) + (reverse.e35 * this.e3) + (reverse.e321 * this.e3215) + (reverse.e4235 * this.e235) + (reverse.e4315 * this.e315) + (reverse.e4125 * this.e125) + (reverse.e3215 * this.e12345) - (reverse.e1 * this.e15) - (reverse.e2 * this.e25) - (reverse.e3 * this.e35) - (reverse.e15 * this.e415) - (reverse.e25 * this.e425) - (reverse.e35 * this.e435) - (reverse.e45 * this.e5) - (reverse.e23 * this.e235) - (reverse.e31 * this.e315) - (reverse.e12 * this.e125) - (reverse.e415 * this.e15) - (reverse.e425 * this.e25) - (reverse.e435 * this.e35) - (reverse.e235 * this.e23) - (reverse.e235 * this.e4235) - (reverse.e315 * this.e31) - (reverse.e315 * this.e4315) - (reverse.e125 * this.e12) - (reverse.e125 * this.e4125) - (reverse.e3215 * this.e321), 
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
            float4((reverse.e415 * this.e321) + (reverse.e425 * this.e3) + (reverse.e321 * this.e415) + (reverse.e1234 * this.e15), (reverse.e425 * this.e321) + (reverse.e435 * this.e1) + (reverse.e321 * this.e425) + (reverse.e1234 * this.e25), (reverse.e415 * this.e2) + (reverse.e435 * this.e321) + (reverse.e321 * this.e435) + (reverse.e1234 * this.e35), -(reverse.e25 * this.e4315) - (reverse.e35 * this.e4125) - (reverse.e45 * this.e3215) - (reverse.e321 * this.e5)) + (float4(reverse.scalar) * this.group9) + (float4(reverse.e45, reverse.e4125, reverse.e4235, this.e25) * float4(this.group5.xyz.xxy, reverse.e4315)) + (float4(reverse.e4315, reverse.e45, reverse.e45, this.e35) * float4(this.group5.xyz.zyz, reverse.e4125)) + (float4(reverse.group0.xy.yy, reverse.e12345, this.scalar) * float4(this.group1.xyz, reverse.e3215)) + (float4(this.group0.xy.xx, this.scalar, reverse.e235) * float4(reverse.group9.xyz, this.e415)) + (float4(this.group0.xy.yy, this.e12345, reverse.e315) * float4(reverse.group1.xyz, this.e425)) + (float4(this.group7.xyz.zx, this.e4, reverse.e435) * float4(reverse.group8.xyz.yzz, this.e125)) + (float4(this.group1.ww, this.e431, reverse.e3) * float4(reverse.group8.xyz.xyx, this.e125)) + (float4(this.group3.zx, this.e3215, reverse.e1) * float4(reverse.group4.xyz.yzz, this.e235)) + (float4(this.group3.ww, this.e4315, reverse.e415) * float4(reverse.group5.xyz.xyx, this.e235)) + (float4(this.group9.zx, this.e45, reverse.e2) * float4(reverse.group5.xyz.yzz, this.e315)) + (float4(this.group9.ww, this.e25, this.e435) * float4(reverse.group4.xyz.xyx, reverse.e125)) + float4((float3(reverse.e5) * this.group7.xyz), reverse.e3215 * this.e45) + float4((reverse.group7.xyz.zxy * this.group8.xyz.yzx), reverse.e425 * this.e315) + float4((this.group4.xyz.yzx * reverse.group3.zxy), reverse.e4235 * this.e15) + float4((reverse.group1.yzx * this.group6.zxy), reverse.e5 * this.e321) - (float4(reverse.e25, reverse.e3215, reverse.e3215, this.e2) * float4(this.group4.xyz.zyz, reverse.e315)) - (float4(reverse.e3215, reverse.e35, reverse.e15, this.e1) * float4(this.group4.xyz.xxy, reverse.e235)) - (float4(float2(this.e5), this.e315, this.e15) * float4(reverse.group7.xyz.xyx, reverse.e23)) - (float4(this.group8.xyz.zx, this.e5, this.e25) * float4(reverse.group7.xyz.yzz, reverse.e31)) - float4((float3(this.e1234) * reverse.group3.xyz), reverse.e35 * this.e12) - float4((this.group8.xyz * reverse.group1.www), reverse.e15 * this.e23) - float4((reverse.group4.xyz.zxy * this.group3.yzx), reverse.e12345 * this.e5) - float4((reverse.group5.xyz.zxy * this.group9.yzx), reverse.e5 * this.e12345) - float4((reverse.group8.xyz.zxy * this.group7.xyz.yzx), reverse.e12 * this.e35) - float4((this.group5.xyz.yzx * reverse.group9.zxy), reverse.e125 * this.e3) - float4((reverse.group1.zxy * this.group6.yzx), reverse.e25 * this.e31) - float4((reverse.group6.zxy * this.group1.yzx), reverse.e15 * this.e4235), 
            /* e1234 */
            (reverse.scalar * this.e1234) + (reverse.e45 * this.e1234) + (reverse.e41 * this.e4235) + (reverse.e42 * this.e4315) + (reverse.e43 * this.e4125) + (reverse.e415 * this.e423) + (reverse.e425 * this.e431) + (reverse.e435 * this.e412) + (reverse.e321 * this.e4) + (reverse.e423 * this.e1) + (reverse.e423 * this.e415) + (reverse.e431 * this.e2) + (reverse.e431 * this.e425) + (reverse.e412 * this.e3) + (reverse.e412 * this.e435) + (reverse.e1234 * this.scalar) - (reverse.e12345 * this.e4) - (reverse.e1 * this.e423) - (reverse.e2 * this.e431) - (reverse.e3 * this.e412) - (reverse.e4 * this.e12345) - (reverse.e4 * this.e321) - (reverse.e41 * this.e23) - (reverse.e42 * this.e31) - (reverse.e43 * this.e12) - (reverse.e23 * this.e41) - (reverse.e31 * this.e42) - (reverse.e12 * this.e43) - (reverse.e4235 * this.e41) - (reverse.e4315 * this.e42) - (reverse.e4125 * this.e43) - (reverse.e1234 * this.e45)
        );
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, geometric_product.e12345), 
            /* e1, e2, e3, e4 */
            geometric_product.group1, 
            /* e5 */
            geometric_product.e5, 
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
            geometric_product.group9, 
            /* e1234 */
            geometric_product.e1234
        );
    }
}
public extension VersorEven {
    // Fancy postfix trick
    public VersorOdd operator /(ConstraintViolationPrefixOrPostfix rhs)  {
        return this.constraint_violation();
    }
}
public extension ConstraintViolationPrefixOrPostfix {
    // Fancy prefix trick
    public VersorOdd operator /(VersorEven rhs) {
        return rhs.constraint_violation();
    }
}
public extension VersorEven {
    public VersorOdd constraint_violation() {
        VersorEven reverse = VersorEven(
            /* e423, e431, e412, e12345 */
            this.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e415, e425, e435, e321 */
            this.group1 * float4(-1.0), 
            /* e235, e315, e125, e5 */
            this.group2 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e4 */
            this.group3
        );
        VersorOdd geometric_product = VersorOdd(
            /* e41, e42, e43, scalar */
            float4(float3(0.0), (reverse.e423 * this.e235) + (reverse.e431 * this.e315) + (reverse.e412 * this.e125) + (reverse.e415 * this.e415) + (reverse.e425 * this.e425) + (reverse.e435 * this.e435) + (reverse.e235 * this.e423) + (reverse.e315 * this.e431) + (reverse.e125 * this.e412) + (reverse.e1 * this.e1) + (reverse.e2 * this.e2) + (reverse.e3 * this.e3) - (reverse.e12345 * this.e12345) - (reverse.e321 * this.e321) - (reverse.e5 * this.e4) - (reverse.e4 * this.e5)), 
            /* e23, e31, e12, e45 */
            float4(0.0), 
            /* e15, e25, e35, e1234 */
            float4(float3(0.0), (reverse.e423 * this.e415) + (reverse.e423 * this.e1) + (reverse.e431 * this.e425) + (reverse.e431 * this.e2) + (reverse.e412 * this.e435) + (reverse.e412 * this.e3) + (reverse.e415 * this.e423) + (reverse.e425 * this.e431) + (reverse.e435 * this.e412) + (reverse.e321 * this.e4) - (reverse.e12345 * this.e4) - (reverse.e1 * this.e423) - (reverse.e2 * this.e431) - (reverse.e3 * this.e412) - (reverse.e4 * this.e12345) - (reverse.e4 * this.e321)), 
            /* e4235, e4315, e4125, e3215 */
            (reverse.group1.xyxz * float4(this.group1.ww, this.e2, this.e125)) + (reverse.group2.xyxz * float4(this.group3.ww, this.e431, this.e435)) + (reverse.group2.yzzw * float4(this.group0.zx, this.e4, this.e321)) + (reverse.group3.xyxy * float4(this.group0.ww, this.e425, this.e315)) + (reverse.group3.yzzz * float4(this.group1.zx, this.e12345, this.e125)) + (this.group1.xyzy * float4(reverse.group1.www, reverse.e315)) + (this.group2.yzxx * float4(reverse.group0.zxy, reverse.e415)) + (float4(this.group3.zx, this.e321, this.e415) * float4(reverse.group1.yzz, reverse.e235)) + float4((reverse.group0.www * this.group3.xyz), reverse.e425 * this.e315) + float4((reverse.group2.www * this.group0.xyz), reverse.e1 * this.e235) - (reverse.group0.xyxw * this.group2.wwyw) - (reverse.group2.zxyy * float4(this.group0.yzx, this.e2)) - (this.group2.zxww * float4(reverse.group0.yzz, reverse.e321)) - (this.group3.yzxx * float4(reverse.group1.zxy, reverse.e235)) - float4((reverse.group3.zxy * this.group1.yzx), reverse.e125 * this.e3) - float4((reverse.group3.www * this.group2.xyz), reverse.e5 * this.e12345)
        );
        return VersorOdd(
            /* e41, e42, e43, scalar */
            geometric_product.group0 + float4(float3(0.0), 2.0*(this.e423 * this.e235) + 2.0 * (this.e431 * this.e315) + 2.0 * (this.e412 * this.e125) + 2.0 * (this.e5 * this.e4) + pow(this.e12345, 2) + pow(this.e415, 2) + pow(this.e425, 2) + pow(this.e435, 2) - pow(this.e321, 2) - pow(this.e1, 2) - pow(this.e2, 2) - pow(this.e3, 2)), 
            /* e23, e31, e12, e45 */
            float4(0.0), 
            /* e15, e25, e35, e1234 */
            geometric_product.group2, 
            /* e4235, e4315, e4125, e3215 */
            geometric_product.group3
        );
    }
}
public extension VersorOdd {
    // Fancy postfix trick
    public VersorOdd operator /(ConstraintViolationPrefixOrPostfix rhs)  {
        return this.constraint_violation();
    }
}
public extension ConstraintViolationPrefixOrPostfix {
    // Fancy prefix trick
    public VersorOdd operator /(VersorOdd rhs) {
        return rhs.constraint_violation();
    }
}
public extension VersorOdd {
    public VersorOdd constraint_violation() {
        VersorOdd reverse = VersorOdd(
            /* e41, e42, e43, scalar */
            this.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, e45 */
            this.group1 * float4(-1.0), 
            /* e15, e25, e35, e1234 */
            this.group2 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group3
        );
        VersorOdd geometric_product = VersorOdd(
            /* e41, e42, e43, scalar */
            float4(float3(0.0), (reverse.scalar * this.scalar) + (reverse.e45 * this.e45) + (reverse.e1234 * this.e3215) + (reverse.e3215 * this.e1234) - (reverse.e41 * this.e15) - (reverse.e42 * this.e25) - (reverse.e43 * this.e35) - (reverse.e23 * this.e23) - (reverse.e31 * this.e31) - (reverse.e12 * this.e12) - (reverse.e15 * this.e41) - (reverse.e25 * this.e42) - (reverse.e35 * this.e43) - (reverse.e4235 * this.e4235) - (reverse.e4315 * this.e4315) - (reverse.e4125 * this.e4125)), 
            /* e23, e31, e12, e45 */
            float4(0.0), 
            /* e15, e25, e35, e1234 */
            float4(float3(0.0), (reverse.e41 * this.e4235) + (reverse.e42 * this.e4315) + (reverse.e43 * this.e4125) + (reverse.scalar * this.e1234) + (reverse.e45 * this.e1234) + (reverse.e1234 * this.scalar) - (reverse.e41 * this.e23) - (reverse.e42 * this.e31) - (reverse.e43 * this.e12) - (reverse.e23 * this.e41) - (reverse.e31 * this.e42) - (reverse.e12 * this.e43) - (reverse.e1234 * this.e45) - (reverse.e4235 * this.e41) - (reverse.e4315 * this.e42) - (reverse.e4125 * this.e43)), 
            /* e4235, e4315, e4125, e3215 */
            float4((reverse.e35 * this.e42) + (reverse.e1234 * this.e15) + (reverse.e4235 * this.scalar) + (reverse.e4315 * this.e12), (reverse.e15 * this.e43) + (reverse.e1234 * this.e25) + (reverse.e4315 * this.scalar) + (reverse.e4125 * this.e23), (reverse.e25 * this.e41) + (reverse.e1234 * this.e35) + (reverse.e4235 * this.e31) + (reverse.e4125 * this.scalar), -(reverse.e25 * this.e31) - (reverse.e25 * this.e4315) - (reverse.e35 * this.e12) - (reverse.e35 * this.e4125)) + (this.group1 * float4(reverse.group1.www, reverse.e3215)) + (reverse.group0.xyxw * float4(this.group3.ww, this.e25, this.e3215)) + (float4(this.group1.ww, this.e4315, this.e35) * float4(reverse.group1.xyx, reverse.e4125)) + (float4(this.group2.zx, this.e3215, this.e15) * float4(reverse.group0.yzz, reverse.e4235)) + (float4(this.group3.zx, this.e45, this.scalar) * float4(reverse.group1.yzz, reverse.e3215)) + float4((reverse.group0.www * this.group3.xyz), reverse.e4315 * this.e25) - (reverse.group1.zxyy * float4(this.group3.yzx, this.e25)) - (this.group1.yzxx * float4(reverse.group3.zxy, reverse.e15)) - (this.group2.yzxx * float4(reverse.group0.zxy, reverse.e23)) - (float4(this.group0.zx, this.e1234, this.e3215) * float4(reverse.group2.yzz, reverse.e45)) - (float4(this.group2.ww, this.e42, this.e35) * float4(reverse.group2.xyx, reverse.e12)) - float4((reverse.group3.www * this.group0.xyz), reverse.e15 * this.e4235)
        );
        return VersorOdd(
            /* e41, e42, e43, scalar */
            geometric_product.group0 + float4(float3(0.0), pow(this.e45, 2) + pow(this.e4235, 2) + pow(this.e4315, 2) + pow(this.e4125, 2) - pow(this.scalar, 2) - pow(this.e23, 2) - pow(this.e31, 2) - pow(this.e12, 2) - 2.0 * (this.e41 * this.e15) - 2.0 * (this.e42 * this.e25) - 2.0 * (this.e43 * this.e35) - 2.0 * (this.e1234 * this.e3215)), 
            /* e23, e31, e12, e45 */
            float4(0.0), 
            /* e15, e25, e35, e1234 */
            geometric_product.group2, 
            /* e4235, e4315, e4125, e3215 */
            geometric_product.group3
        );
    }
}

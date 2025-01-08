implementing cga3d;
using traits;
public extension AntiCircleRotor {
    // Fancy infix trick (first half)
    public ProjectOrthogonallyOntoInfixPartial<AntiCircleRotor> operator /(ProjectOrthogonallyOntoInfix rhs) {
        return ProjectOrthogonallyOntoInfixPartial<AntiCircleRotor>(this);
    }
}
public extension ProjectOrthogonallyOntoInfixPartial<AntiCircleRotor> {
    // Fancy infix trick (second half)
    public AntiCircleRotor operator /(AntiCircleRotor rhs) {
        return this.a.project_orthogonally_onto(rhs);
    }
}
public extension AntiCircleRotor {
    public AntiCircleRotor project_orthogonally_onto(AntiCircleRotor other) {
        CircleRotor right_anti_dual = CircleRotor(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        CircleRotor wedge = CircleRotor(
            /* e423, e431, e412 */
            float3(this.scalar) * right_anti_dual.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(this.scalar) * right_anti_dual.group1, 
            /* e235, e315, e125, e12345 */
            float4((this.group2.www * right_anti_dual.group2.xyz), (this.scalar * right_anti_dual.e12345) - (this.e41 * right_anti_dual.e235) - (this.e42 * right_anti_dual.e315) - (this.e43 * right_anti_dual.e125) - (this.e23 * right_anti_dual.e415) - (this.e31 * right_anti_dual.e425) - (this.e12 * right_anti_dual.e435) - (this.e45 * right_anti_dual.e321) - (this.e15 * right_anti_dual.e423) - (this.e25 * right_anti_dual.e431) - (this.e35 * right_anti_dual.e412))
        );
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(wedge.e12345) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(wedge.e12345) * other.group1, 
            /* e15, e25, e35, scalar */
            float4((float3(wedge.e12345) * other.group2.xyz), (other.scalar * wedge.e12345) - (other.e41 * wedge.e235) - (other.e42 * wedge.e315) - (other.e43 * wedge.e125) - (other.e23 * wedge.e415) - (other.e31 * wedge.e425) - (other.e12 * wedge.e435) - (other.e45 * wedge.e321) - (other.e15 * wedge.e423) - (other.e25 * wedge.e431) - (other.e35 * wedge.e412))
        );
    }
}
public extension AntiCircleRotor {
    public AntiCircleRotor project_orthogonally_onto(AntiDipoleInversion other) {
        DipoleInversion right_anti_dual = DipoleInversion(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e1234 */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group3 * float4(1.0, 1.0, 1.0, -1.0)
        );
        DipoleInversion wedge = DipoleInversion(
            /* e41, e42, e43 */
            float3(this.scalar) * right_anti_dual.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(this.scalar) * right_anti_dual.group1, 
            /* e15, e25, e35, e1234 */
            float4((this.group2.www * right_anti_dual.group2.xyz), (this.scalar * right_anti_dual.e1234) - (this.e41 * right_anti_dual.e23) - (this.e42 * right_anti_dual.e31) - (this.e43 * right_anti_dual.e12) - (this.e23 * right_anti_dual.e41) - (this.e31 * right_anti_dual.e42) - (this.e12 * right_anti_dual.e43)), 
            /* e4235, e4315, e4125, e3215 */
            float4((this.e23 * right_anti_dual.e45) + (this.e45 * right_anti_dual.e23) + (this.e35 * right_anti_dual.e42) + (this.scalar * right_anti_dual.e4235), (this.e31 * right_anti_dual.e45) + (this.e45 * right_anti_dual.e31) + (this.e15 * right_anti_dual.e43) + (this.scalar * right_anti_dual.e4315), (this.e12 * right_anti_dual.e45) + (this.e45 * right_anti_dual.e12) + (this.e25 * right_anti_dual.e41) + (this.scalar * right_anti_dual.e4125), -(this.e12 * right_anti_dual.e35) - (this.e15 * right_anti_dual.e23) - (this.e25 * right_anti_dual.e31) - (this.e35 * right_anti_dual.e12)) + float4((this.group0.xyz.yzx * right_anti_dual.group2.zxy), this.scalar * right_anti_dual.e3215) - (right_anti_dual.group2.yzxx * float4(this.group0.xyz.zxy, this.e23)) - float4((right_anti_dual.group0.xyz.zxy * this.group2.yzx), this.e31 * right_anti_dual.e25)
        );
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (float3(wedge.e1234) * other.group1.xyz) + (other.group0.xyz.zxy * wedge.group3.yzx) - (other.group0.xyz.yzx * wedge.group3.zxy), 
            /* e23, e31, e12, e45 */
            float4((other.e423 * wedge.e3215) + (other.e235 * wedge.e1234), (other.e431 * wedge.e3215) + (other.e315 * wedge.e1234), (other.e412 * wedge.e3215) + (other.e125 * wedge.e1234), -(other.e425 * wedge.e4315) - (other.e435 * wedge.e4125)) - (other.group1.wwwx * wedge.group3.xyzx), 
            /* e15, e25, e35, scalar */
            (float4(wedge.e3215) * float4(other.group1.xyz, other.e4)) + (wedge.group3.zxyx * float4(other.group2.yzx, other.e1)) + float4(float3(0.0), (other.e2 * wedge.e4315) + (other.e3 * wedge.e4125) + (other.e5 * wedge.e1234) - (other.e431 * wedge.e25) - (other.e412 * wedge.e35) - (other.e415 * wedge.e23) - (other.e425 * wedge.e31) - (other.e435 * wedge.e12) - (other.e321 * wedge.e45) - (other.e235 * wedge.e41) - (other.e315 * wedge.e42) - (other.e125 * wedge.e43)) - float4((other.group2.zxy * wedge.group3.yzx), other.e423 * wedge.e15)
        );
    }
}
public extension AntiCircleRotor {
    public AntiMotor project_orthogonally_onto(AntiDualNum other) {
        DualNum right_anti_dual = DualNum(
            /* e5, e12345 */
            other.group0.xy
        );
        Motor wedge = Motor(
            /* e415, e425, e435, e12345 */
            float4(right_anti_dual.group0.xy.xx, right_anti_dual.e5, right_anti_dual.e12345) * float4(this.group0.xyz, this.scalar), 
            /* e235, e315, e125, e5 */
            float4(right_anti_dual.e5) * float4(this.group1.xyz, this.scalar)
        );
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(float3(0.0), other.scalar * wedge.e12345), 
            /* e15, e25, e35, e3215 */
            float4(other.e3215) * wedge.group0
        );
    }
}
public extension AntiCircleRotor {
    public AntiMotor project_orthogonally_onto(AntiFlatPoint other) {
        FlatPoint right_anti_dual = FlatPoint(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        Flector wedge = Flector(
            /* e15, e25, e35, e45 */
            float4(this.scalar) * right_anti_dual.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4((this.e42 * right_anti_dual.e35) + (this.e23 * right_anti_dual.e45), (this.e43 * right_anti_dual.e15) + (this.e31 * right_anti_dual.e45), (this.e41 * right_anti_dual.e25) + (this.e12 * right_anti_dual.e45), -(this.e31 * right_anti_dual.e25) - (this.e12 * right_anti_dual.e35)) - (right_anti_dual.group0.yzxx * float4(this.group0.xyz.zxy, this.e23))
        );
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(other.e321) * float4(wedge.group1.xyz, wedge.e45) * float4(-1.0), 
            /* e15, e25, e35, e3215 */
            float4(((other.group0.yzx * wedge.group1.zxy) - (other.group0.zxy * wedge.group1.yzx)), 0.0)
        );
    }
}
public extension AntiCircleRotor {
    public AntiMotor project_orthogonally_onto(AntiFlector other) {
        Flector right_anti_dual = Flector(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        Flector wedge = Flector(
            /* e15, e25, e35, e45 */
            float4(this.scalar) * right_anti_dual.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4((this.e23 * right_anti_dual.e45) + (this.scalar * right_anti_dual.e4235), (this.e31 * right_anti_dual.e45) + (this.scalar * right_anti_dual.e4315), (this.e12 * right_anti_dual.e45) + (this.scalar * right_anti_dual.e4125), -(this.e31 * right_anti_dual.e25) - (this.e12 * right_anti_dual.e35)) + float4((this.group0.xyz.yzx * right_anti_dual.group0.zxy), this.scalar * right_anti_dual.e3215) - (right_anti_dual.group0.yzxx * float4(this.group0.xyz.zxy, this.e23))
        );
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((other.group0.www * wedge.group1.xyz * float3(-1.0)), (other.e1 * wedge.e4235) + (other.e2 * wedge.e4315) + (other.e3 * wedge.e4125) - (other.e321 * wedge.e45)), 
            /* e15, e25, e35, e3215 */
            float4(((other.group0.yzx * wedge.group1.zxy) - (other.group0.zxy * wedge.group1.yzx)), 0.0)
        );
    }
}
public extension AntiCircleRotor {
    public AntiMotor project_orthogonally_onto(AntiLine other) {
        Line right_anti_dual = Line(
            /* e415, e425, e435 */
            other.group0.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            other.group1.xyz * float3(-1.0)
        );
        Motor wedge = Motor(
            /* e415, e425, e435, e12345 */
            float4((float3(this.scalar) * right_anti_dual.group0.xyz), -(this.e41 * right_anti_dual.e235) - (this.e42 * right_anti_dual.e315) - (this.e43 * right_anti_dual.e125) - (this.e23 * right_anti_dual.e415) - (this.e31 * right_anti_dual.e425) - (this.e12 * right_anti_dual.e435)), 
            /* e235, e315, e125, e5 */
            float4(float3(1.0), 0.0) * float4(right_anti_dual.group1.xyz, 0.0) * float4(this.group2.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((float3(wedge.e12345) * other.group0.xyz), -(other.e23 * wedge.e415) - (other.e31 * wedge.e425) - (other.e12 * wedge.e435)), 
            /* e15, e25, e35, e3215 */
            float4(float3(1.0), 0.0) * float4(other.group1.xyz, 0.0) * float4(wedge.group0.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
public extension AntiCircleRotor {
    public AntiMotor project_orthogonally_onto(AntiMotor other) {
        Motor right_anti_dual = Motor(
            /* e415, e425, e435, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        Motor wedge = Motor(
            /* e415, e425, e435, e12345 */
            float4(this.scalar * right_anti_dual.e415, this.scalar * right_anti_dual.e425, this.scalar * right_anti_dual.e435, -(this.e41 * right_anti_dual.e235) - (this.e42 * right_anti_dual.e315) - (this.e43 * right_anti_dual.e125) - (this.e23 * right_anti_dual.e415) - (this.e31 * right_anti_dual.e425) - (this.e12 * right_anti_dual.e435)) + float4((this.group0.xyz * right_anti_dual.group1.www), this.scalar * right_anti_dual.e12345), 
            /* e235, e315, e125, e5 */
            float4(((float3(this.scalar) * right_anti_dual.group1.xyz) + (float3(right_anti_dual.e5) * this.group1.xyz)), this.scalar * right_anti_dual.e5)
        );
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((float3(wedge.e12345) * other.group0.xyz), (other.scalar * wedge.e12345) - (other.e23 * wedge.e415) - (other.e31 * wedge.e425) - (other.e12 * wedge.e435)), 
            /* e15, e25, e35, e3215 */
            float4(((float3(other.e3215) * wedge.group0.xyz) + (float3(wedge.e12345) * other.group1.xyz)), other.e3215 * wedge.e12345)
        );
    }
}
public extension AntiCircleRotor {
    public Scalar project_orthogonally_onto(AntiPlane other) {
        Plane wedge = Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar) * float4(other.group0.xyz, other.e5 * -1.0)
        );
        return Scalar(
            /* scalar */
            (other.e1 * wedge.e4235) + (other.e2 * wedge.e4315) + (other.e3 * wedge.e4125)
        );
    }
}
public extension AntiCircleRotor {
    public AntiCircleRotor project_orthogonally_onto(AntiScalar other) {
        Scalar right_anti_dual = Scalar(
            /* scalar */
            other.e12345 * -1.0
        );
        AntiCircleRotor wedge = AntiCircleRotor(
            /* e41, e42, e43 */
            float3(right_anti_dual.scalar) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(right_anti_dual.scalar) * this.group1, 
            /* e15, e25, e35, scalar */
            float4(right_anti_dual.scalar) * this.group2
        );
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(other.e12345) * wedge.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(other.e12345) * wedge.group1, 
            /* e15, e25, e35, scalar */
            float4(other.e12345) * wedge.group2
        );
    }
}
public extension AntiCircleRotor {
    public AntiCircleRotor project_orthogonally_onto(Circle other) {
        Dipole right_anti_dual = Dipole(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35 */
            other.group2.xyz
        );
        DipoleInversion wedge = DipoleInversion(
            /* e41, e42, e43 */
            float3(this.scalar) * right_anti_dual.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(this.scalar) * right_anti_dual.group1, 
            /* e15, e25, e35, e1234 */
            float4((float3(this.scalar) * right_anti_dual.group2.xyz), -(this.e41 * right_anti_dual.e23) - (this.e42 * right_anti_dual.e31) - (this.e43 * right_anti_dual.e12) - (this.e23 * right_anti_dual.e41) - (this.e31 * right_anti_dual.e42) - (this.e12 * right_anti_dual.e43)), 
            /* e4235, e4315, e4125, e3215 */
            float4((this.e42 * right_anti_dual.e35) + (this.e23 * right_anti_dual.e45) + (this.e45 * right_anti_dual.e23) + (this.e35 * right_anti_dual.e42), (this.e43 * right_anti_dual.e15) + (this.e31 * right_anti_dual.e45) + (this.e45 * right_anti_dual.e31) + (this.e15 * right_anti_dual.e43), (this.e41 * right_anti_dual.e25) + (this.e12 * right_anti_dual.e45) + (this.e45 * right_anti_dual.e12) + (this.e25 * right_anti_dual.e41), -(this.e12 * right_anti_dual.e35) - (this.e15 * right_anti_dual.e23) - (this.e25 * right_anti_dual.e31) - (this.e35 * right_anti_dual.e12)) - float4((this.group0.xyz.zxy * right_anti_dual.group2.xyz.yzx), this.e23 * right_anti_dual.e15) - float4((right_anti_dual.group0.xyz.zxy * this.group2.yzx), this.e31 * right_anti_dual.e25)
        );
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (float3(wedge.e1234) * other.group1.xyz) + (other.group0.xyz.zxy * wedge.group3.yzx) - (other.group0.xyz.yzx * wedge.group3.zxy), 
            /* e23, e31, e12, e45 */
            float4((other.e423 * wedge.e3215) + (other.e235 * wedge.e1234), (other.e431 * wedge.e3215) + (other.e315 * wedge.e1234), (other.e412 * wedge.e3215) + (other.e125 * wedge.e1234), -(other.e425 * wedge.e4315) - (other.e435 * wedge.e4125)) - (other.group1.wwwx * wedge.group3.xyzx), 
            /* e15, e25, e35, scalar */
            float4((other.e415 * wedge.e3215) + (other.e315 * wedge.e4125), (other.e425 * wedge.e3215) + (other.e125 * wedge.e4235), (other.e435 * wedge.e3215) + (other.e235 * wedge.e4315), -(other.e431 * wedge.e25) - (other.e412 * wedge.e35) - (other.e415 * wedge.e23) - (other.e425 * wedge.e31) - (other.e435 * wedge.e12) - (other.e321 * wedge.e45) - (other.e235 * wedge.e41) - (other.e315 * wedge.e42) - (other.e125 * wedge.e43)) - float4((other.group2.xyz.zxy * wedge.group3.yzx), other.e423 * wedge.e15)
        );
    }
}
public extension AntiCircleRotor {
    public VersorOdd project_orthogonally_onto(CircleRotor other) {
        AntiCircleRotor right_anti_dual = AntiCircleRotor(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        VersorOdd wedge = VersorOdd(
            /* e41, e42, e43, scalar */
            float4(((float3(right_anti_dual.scalar) * this.group0.xyz) + (float3(this.scalar) * right_anti_dual.group0.xyz)), right_anti_dual.scalar * this.scalar), 
            /* e23, e31, e12, e45 */
            (float4(right_anti_dual.scalar) * this.group1) + (float4(this.scalar) * right_anti_dual.group1), 
            /* e15, e25, e35, e1234 */
            float4((right_anti_dual.e15 * this.scalar) + (right_anti_dual.scalar * this.e15), (right_anti_dual.e25 * this.scalar) + (right_anti_dual.scalar * this.e25), (right_anti_dual.e35 * this.scalar) + (right_anti_dual.scalar * this.e35), -(right_anti_dual.e41 * this.e23) - (right_anti_dual.e42 * this.e31) - (right_anti_dual.e43 * this.e12) - (right_anti_dual.e23 * this.e41) - (right_anti_dual.e31 * this.e42) - (right_anti_dual.e12 * this.e43)), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual.e42 * this.e35) + (right_anti_dual.e23 * this.e45) + (right_anti_dual.e45 * this.e23) + (right_anti_dual.e35 * this.e42), (right_anti_dual.e43 * this.e15) + (right_anti_dual.e31 * this.e45) + (right_anti_dual.e45 * this.e31) + (right_anti_dual.e15 * this.e43), (right_anti_dual.e41 * this.e25) + (right_anti_dual.e12 * this.e45) + (right_anti_dual.e45 * this.e12) + (right_anti_dual.e25 * this.e41), -(right_anti_dual.e12 * this.e35) - (right_anti_dual.e15 * this.e23) - (right_anti_dual.e25 * this.e31) - (right_anti_dual.e35 * this.e12)) - (this.group2.yzxx * float4(right_anti_dual.group0.xyz.zxy, right_anti_dual.e23)) - float4((this.group0.xyz.zxy * right_anti_dual.group2.yzx), right_anti_dual.e31 * this.e25)
        );
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4((other.e415 * wedge.e1234) + (other.e12345 * wedge.e41), (other.e425 * wedge.e1234) + (other.e12345 * wedge.e42), (other.e435 * wedge.e1234) + (other.e12345 * wedge.e43), -(other.e431 * wedge.e25) - (other.e412 * wedge.e35) - (other.e415 * wedge.e23) - (other.e425 * wedge.e31) - (other.e435 * wedge.e12) - (other.e321 * wedge.e45) - (other.e235 * wedge.e41) - (other.e315 * wedge.e42) - (other.e125 * wedge.e43)) + float4((other.group0.xyz.zxy * wedge.group3.yzx), other.e12345 * wedge.scalar) - float4((other.group0.xyz.yzx * wedge.group3.zxy), other.e423 * wedge.e15), 
            /* e23, e31, e12, e45 */
            float4((other.e235 * wedge.e1234) + (other.e12345 * wedge.e23), (other.e315 * wedge.e1234) + (other.e12345 * wedge.e31), (other.e125 * wedge.e1234) + (other.e12345 * wedge.e12), -(other.e425 * wedge.e4315) - (other.e435 * wedge.e4125)) + float4((other.group0.xyz * wedge.group3.www), other.e12345 * wedge.e45) - (other.group1.wwwx * wedge.group3.xyzx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(other.e12345) * wedge.group2.xyz) + (float3(wedge.e3215) * other.group1.xyz) + (other.group2.yzx * wedge.group3.zxy) - (other.group2.zxy * wedge.group3.yzx)), other.e12345 * wedge.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345) * wedge.group3
        );
    }
}
public extension AntiCircleRotor {
    public AntiCircleRotor project_orthogonally_onto(Dipole other) {
        Circle right_anti_dual = Circle(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125 */
            other.group2.xyz * float3(-1.0)
        );
        CircleRotor wedge = CircleRotor(
            /* e423, e431, e412 */
            float3(this.scalar) * right_anti_dual.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(this.scalar) * right_anti_dual.group1, 
            /* e235, e315, e125, e12345 */
            float4((float3(this.scalar) * right_anti_dual.group2.xyz), -(this.e41 * right_anti_dual.e235) - (this.e42 * right_anti_dual.e315) - (this.e43 * right_anti_dual.e125) - (this.e23 * right_anti_dual.e415) - (this.e31 * right_anti_dual.e425) - (this.e12 * right_anti_dual.e435) - (this.e45 * right_anti_dual.e321) - (this.e15 * right_anti_dual.e423) - (this.e25 * right_anti_dual.e431) - (this.e35 * right_anti_dual.e412))
        );
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(wedge.e12345) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(wedge.e12345) * other.group1, 
            /* e15, e25, e35, scalar */
            float4((float3(wedge.e12345) * other.group2.xyz), -(wedge.e423 * other.e15) - (wedge.e431 * other.e25) - (wedge.e412 * other.e35) - (wedge.e415 * other.e23) - (wedge.e425 * other.e31) - (wedge.e435 * other.e12) - (wedge.e321 * other.e45) - (wedge.e235 * other.e41) - (wedge.e315 * other.e42) - (wedge.e125 * other.e43))
        );
    }
}
public extension AntiCircleRotor {
    public VersorOdd project_orthogonally_onto(DipoleInversion other) {
        AntiDipoleInversion right_anti_dual = AntiDipoleInversion(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e4 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group3 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        VersorEven wedge = VersorEven(
            /* e423, e431, e412, e12345 */
            float4((this.e42 * right_anti_dual.e3) + (this.e23 * right_anti_dual.e4) + (this.scalar * right_anti_dual.e423), (this.e43 * right_anti_dual.e1) + (this.e31 * right_anti_dual.e4) + (this.scalar * right_anti_dual.e431), (this.e41 * right_anti_dual.e2) + (this.e12 * right_anti_dual.e4) + (this.scalar * right_anti_dual.e412), -(this.e42 * right_anti_dual.e315) - (this.e43 * right_anti_dual.e125) - (this.e23 * right_anti_dual.e415) - (this.e31 * right_anti_dual.e425) - (this.e12 * right_anti_dual.e435) - (this.e45 * right_anti_dual.e321) - (this.e15 * right_anti_dual.e423) - (this.e25 * right_anti_dual.e431) - (this.e35 * right_anti_dual.e412)) - float4((this.group0.xyz.zxy * right_anti_dual.group3.yzx), this.e41 * right_anti_dual.e235), 
            /* e415, e425, e435, e321 */
            float4((this.e15 * right_anti_dual.e4) + (this.scalar * right_anti_dual.e415), (this.e25 * right_anti_dual.e4) + (this.scalar * right_anti_dual.e425), (this.e35 * right_anti_dual.e4) + (this.scalar * right_anti_dual.e435), -(this.e31 * right_anti_dual.e2) - (this.e12 * right_anti_dual.e3)) + float4((this.group0.xyz * right_anti_dual.group3.www), this.scalar * right_anti_dual.e321) - (this.group1.wwwx * right_anti_dual.group3.xyzx), 
            /* e235, e315, e125, e5 */
            float4(((float3(this.scalar) * right_anti_dual.group2.xyz) + (float3(right_anti_dual.e5) * this.group1.xyz) + (this.group2.zxy * right_anti_dual.group3.yzx) - (this.group2.yzx * right_anti_dual.group3.zxy)), this.scalar * right_anti_dual.e5), 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * float4(right_anti_dual.group3.xyz, right_anti_dual.e4)
        );
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (other.group3.yzxy * float4(wedge.group0.zxy, wedge.e2)) + float4(float3(0.0), (other.e4125 * wedge.e3) + (other.e3215 * wedge.e4) - (other.e42 * wedge.e315) - (other.e43 * wedge.e125) - (other.e23 * wedge.e415) - (other.e31 * wedge.e425) - (other.e12 * wedge.e435) - (other.e45 * wedge.e321) - (other.e15 * wedge.e423) - (other.e25 * wedge.e431) - (other.e35 * wedge.e412)) + float4((other.group0.xyz * wedge.group0.www), other.e1234 * wedge.e5) + float4((other.group2.www * wedge.group1.xyz), other.e4235 * wedge.e1) - float4((other.group3.zxy * wedge.group0.yzx), other.e41 * wedge.e235), 
            /* e23, e31, e12, e45 */
            float4((other.e1234 * wedge.e235) + (other.e3215 * wedge.e423), (other.e1234 * wedge.e315) + (other.e3215 * wedge.e431), (other.e1234 * wedge.e125) + (other.e3215 * wedge.e412), -(other.e4315 * wedge.e425) - (other.e4125 * wedge.e435)) + (float4(wedge.e12345) * other.group1) - (other.group3.xyzx * wedge.group1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(other.e3215) * wedge.group1.xyz) + (float3(wedge.e12345) * other.group2.xyz) + (other.group3.zxy * wedge.group2.yzx) - (other.group3.yzx * wedge.group2.zxy)), other.e1234 * wedge.e12345), 
            /* e4235, e4315, e4125, e3215 */
            float4(wedge.e12345) * other.group3
        );
    }
}
public extension AntiCircleRotor {
    public VersorOdd project_orthogonally_onto(DualNum other) {
        AntiDualNum right_anti_dual = AntiDualNum(
            /* e3215, scalar */
            other.group0.xy * float2(-1.0)
        );
        VersorOdd wedge = VersorOdd(
            /* e41, e42, e43, scalar */
            float4(right_anti_dual.scalar) * float4(this.group0.xyz, this.scalar), 
            /* e23, e31, e12, e45 */
            float4(right_anti_dual.scalar) * this.group1, 
            /* e15, e25, e35, e1234 */
            float4(right_anti_dual.group0.xy.yy, right_anti_dual.scalar, 0.0) * float4(float3(1.0), 0.0) * float4(this.group2.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), this.scalar * right_anti_dual.e3215)
        );
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(other.group0.xy.yy, other.e12345, (other.e5 * wedge.e1234) + (other.e12345 * wedge.scalar)) * float4(wedge.group0.xyz, 1.0), 
            /* e23, e31, e12, e45 */
            float4(other.e12345) * wedge.group1, 
            /* e15, e25, e35, e1234 */
            float4(other.group0.xy.yy, other.e12345, 0.0) * float4(float3(1.0), 0.0) * float4(wedge.group2.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345) * wedge.group3
        );
    }
}
public extension AntiCircleRotor {
    public AntiCircleRotor project_orthogonally_onto(FlatPoint other) {
        AntiFlatPoint right_anti_dual = AntiFlatPoint(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        CircleRotor wedge = CircleRotor(
            /* e423, e431, e412 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), this.scalar * right_anti_dual.e321), 
            /* e235, e315, e125, e12345 */
            float4((this.group2.www * right_anti_dual.group0.xyz), -(this.e41 * right_anti_dual.e235) - (this.e42 * right_anti_dual.e315) - (this.e43 * right_anti_dual.e125) - (this.e45 * right_anti_dual.e321))
        );
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), wedge.e12345 * other.e45), 
            /* e15, e25, e35, scalar */
            float4((wedge.group2.www * other.group0.xyz), -(wedge.e423 * other.e15) - (wedge.e431 * other.e25) - (wedge.e412 * other.e35) - (wedge.e321 * other.e45))
        );
    }
}
public extension AntiCircleRotor {
    public VersorOdd project_orthogonally_onto(Flector other) {
        AntiFlector right_anti_dual = AntiFlector(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        VersorEven wedge = VersorEven(
            /* e423, e431, e412, e12345 */
            float4(this.e42 * right_anti_dual.e3, this.e43 * right_anti_dual.e1, this.e41 * right_anti_dual.e2, -(this.e42 * right_anti_dual.e315) - (this.e43 * right_anti_dual.e125) - (this.e45 * right_anti_dual.e321)) - float4((this.group0.xyz.zxy * right_anti_dual.group1.yzx), this.e41 * right_anti_dual.e235), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), -(this.e31 * right_anti_dual.e2) - (this.e12 * right_anti_dual.e3)) + float4((this.group0.xyz * right_anti_dual.group1.www), this.scalar * right_anti_dual.e321) - (this.group1.wwwx * right_anti_dual.group1.xyzx), 
            /* e235, e315, e125, e5 */
            float4(((float3(this.scalar) * right_anti_dual.group0.xyz) + (float3(right_anti_dual.e5) * this.group1.xyz) + (this.group2.zxy * right_anti_dual.group1.yzx) - (this.group2.yzx * right_anti_dual.group1.zxy)), this.scalar * right_anti_dual.e5), 
            /* e1, e2, e3, e4 */
            float4(float3(1.0), 0.0) * float4(this.group2.www, 0.0) * float4(right_anti_dual.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (other.group1.yzxx * float4(wedge.group0.zxy, wedge.e1)) + float4(float3(0.0), (other.e4315 * wedge.e2) + (other.e4125 * wedge.e3) + (other.e3215 * wedge.e4) - (other.e25 * wedge.e431) - (other.e35 * wedge.e412) - (other.e45 * wedge.e321)) - (wedge.group0.yzxx * float4(other.group1.zxy, other.e15)), 
            /* e23, e31, e12, e45 */
            (wedge.group0 * float4(other.group1.www, other.e45)) + float4(float3(0.0), -(other.e4315 * wedge.e425) - (other.e4125 * wedge.e435)) - (other.group1.xyzx * wedge.group1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4((float3(other.e3215) * wedge.group1.xyz), 0.0) + float4((float3(wedge.e12345) * other.group0.xyz), 0.0) + float4((other.group1.zxy * wedge.group2.yzx), 0.0) - float4((other.group1.yzx * wedge.group2.zxy), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(wedge.e12345) * other.group1
        );
    }
}
public extension AntiCircleRotor {
    public AntiCircleRotor project_orthogonally_onto(Line other) {
        AntiLine right_anti_dual = AntiLine(
            /* e23, e31, e12 */
            other.group0.xyz, 
            /* e15, e25, e35 */
            other.group1.xyz
        );
        DipoleInversion wedge = DipoleInversion(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12, e45 */
            float4(float3(1.0), 0.0) * float4(right_anti_dual.group0.xyz, 0.0) * float4(this.group2.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e15, e25, e35, e1234 */
            float4((float3(this.scalar) * right_anti_dual.group1.xyz), -(this.e41 * right_anti_dual.e23) - (this.e42 * right_anti_dual.e31) - (this.e43 * right_anti_dual.e12)), 
            /* e4235, e4315, e4125, e3215 */
            float4((this.e42 * right_anti_dual.e35) + (this.e45 * right_anti_dual.e23), (this.e43 * right_anti_dual.e15) + (this.e45 * right_anti_dual.e31), (this.e41 * right_anti_dual.e25) + (this.e45 * right_anti_dual.e12), -(this.e23 * right_anti_dual.e15) - (this.e31 * right_anti_dual.e25) - (this.e12 * right_anti_dual.e35) - (this.e25 * right_anti_dual.e31) - (this.e35 * right_anti_dual.e12)) - float4((this.group0.xyz.zxy * right_anti_dual.group1.xyz.yzx), this.e15 * right_anti_dual.e23)
        );
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(wedge.e1234) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4((float3(wedge.e1234) * other.group1.xyz), -(wedge.e4235 * other.e415) - (wedge.e4315 * other.e425) - (wedge.e4125 * other.e435)), 
            /* e15, e25, e35, scalar */
            float4((wedge.e4125 * other.e315) + (wedge.e3215 * other.e415), (wedge.e4235 * other.e125) + (wedge.e3215 * other.e425), (wedge.e4315 * other.e235) + (wedge.e3215 * other.e435), -(wedge.e42 * other.e315) - (wedge.e43 * other.e125) - (wedge.e23 * other.e415) - (wedge.e31 * other.e425) - (wedge.e12 * other.e435)) - float4((other.group1.xyz.zxy * wedge.group3.yzx), wedge.e41 * other.e235)
        );
    }
}
public extension AntiCircleRotor {
    public VersorOdd project_orthogonally_onto(Motor other) {
        AntiMotor right_anti_dual = AntiMotor(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        VersorOdd wedge = VersorOdd(
            /* e41, e42, e43, scalar */
            float4(right_anti_dual.scalar) * float4(this.group0.xyz, this.scalar), 
            /* e23, e31, e12, e45 */
            float4(((float3(this.scalar) * right_anti_dual.group0.xyz) + (float3(right_anti_dual.scalar) * this.group1.xyz)), this.e45 * right_anti_dual.scalar), 
            /* e15, e25, e35, e1234 */
            float4((this.e15 * right_anti_dual.scalar) + (this.scalar * right_anti_dual.e15), (this.e25 * right_anti_dual.scalar) + (this.scalar * right_anti_dual.e25), (this.e35 * right_anti_dual.scalar) + (this.scalar * right_anti_dual.e35), -(this.e41 * right_anti_dual.e23) - (this.e42 * right_anti_dual.e31) - (this.e43 * right_anti_dual.e12)), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e45 * right_anti_dual.e23, this.e45 * right_anti_dual.e31, this.e45 * right_anti_dual.e12, -(this.e31 * right_anti_dual.e25) - (this.e12 * right_anti_dual.e35) - (this.e15 * right_anti_dual.e23) - (this.e25 * right_anti_dual.e31) - (this.e35 * right_anti_dual.e12)) + (right_anti_dual.group1.zxyw * float4(this.group0.xyz.yzx, this.scalar)) - (right_anti_dual.group1.yzxx * float4(this.group0.xyz.zxy, this.e23))
        );
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (other.group0 * float4(wedge.group2.www, wedge.scalar)) + float4(float3(0.0), -(other.e415 * wedge.e23) - (other.e425 * wedge.e31) - (other.e435 * wedge.e12) - (other.e235 * wedge.e41) - (other.e315 * wedge.e42) - (other.e125 * wedge.e43)) + float4((other.group0.www * wedge.group0.xyz), other.e5 * wedge.e1234), 
            /* e23, e31, e12, e45 */
            float4(other.e235 * wedge.e1234, other.e315 * wedge.e1234, other.e125 * wedge.e1234, -(other.e415 * wedge.e4235) - (other.e425 * wedge.e4315) - (other.e435 * wedge.e4125)) + (float4(other.e12345) * wedge.group1), 
            /* e15, e25, e35, e1234 */
            float4(((float3(other.e12345) * wedge.group2.xyz) + (float3(wedge.e3215) * other.group0.xyz) + (other.group1.yzx * wedge.group3.zxy) - (other.group1.zxy * wedge.group3.yzx)), other.e12345 * wedge.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345) * wedge.group3
        );
    }
}
public extension AntiCircleRotor {
    public MultiVector project_orthogonally_onto(MultiVector other) {
        MultiVector right_anti_dual = MultiVector(
            /* scalar, e12345 */
            other.group0.xy.yx * float2(-1.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4(other.group9.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5 */
            other.e3215, 
            /* e15, e25, e35, e45 */
            float4(other.group8.xyz, other.e321 * -1.0), 
            /* e41, e42, e43 */
            other.group7.xyz, 
            /* e23, e31, e12 */
            other.group6.xyz, 
            /* e415, e425, e435, e321 */
            float4(other.group5.xyz, other.e45) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e423, e431, e412 */
            other.group4.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            other.group3.xyz * float3(-1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.group1.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234 */
            other.e4 * -1.0
        );
        MultiVector wedge = MultiVector(
            /* scalar, e12345 */
            float2(this.scalar * right_anti_dual.scalar, (this.scalar * right_anti_dual.e12345) - (this.e41 * right_anti_dual.e235) - (this.e42 * right_anti_dual.e315) - (this.e43 * right_anti_dual.e125) - (this.e23 * right_anti_dual.e415) - (this.e31 * right_anti_dual.e425) - (this.e12 * right_anti_dual.e435) - (this.e45 * right_anti_dual.e321) - (this.e15 * right_anti_dual.e423) - (this.e25 * right_anti_dual.e431) - (this.e35 * right_anti_dual.e412)), 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * right_anti_dual.group1, 
            /* e5 */
            this.scalar * right_anti_dual.e5, 
            /* e15, e25, e35, e45 */
            (float4(this.scalar) * right_anti_dual.group3) + (float4(right_anti_dual.scalar) * float4(this.group2.xyz, this.e45)), 
            /* e41, e42, e43 */
            (float3(this.scalar) * right_anti_dual.group4.xyz) + (float3(right_anti_dual.scalar) * this.group0.xyz), 
            /* e23, e31, e12 */
            (float3(this.scalar) * right_anti_dual.group5.xyz) + (float3(right_anti_dual.scalar) * this.group1.xyz), 
            /* e415, e425, e435, e321 */
            float4((this.e15 * right_anti_dual.e4) + (this.scalar * right_anti_dual.e415), (this.e25 * right_anti_dual.e4) + (this.scalar * right_anti_dual.e425), (this.e35 * right_anti_dual.e4) + (this.scalar * right_anti_dual.e435), -(this.e31 * right_anti_dual.e2) - (this.e12 * right_anti_dual.e3)) + float4((float3(right_anti_dual.e5) * this.group0.xyz), this.scalar * right_anti_dual.e321) - (this.group1.wwwx * right_anti_dual.group1.xyzx), 
            /* e423, e431, e412 */
            (float3(this.scalar) * right_anti_dual.group7.xyz) + (float3(right_anti_dual.e4) * this.group1.xyz) + (this.group0.xyz.yzx * right_anti_dual.group1.zxy) - (this.group0.xyz.zxy * right_anti_dual.group1.yzx), 
            /* e235, e315, e125 */
            (float3(this.scalar) * right_anti_dual.group8.xyz) + (float3(right_anti_dual.e5) * this.group1.xyz) + (this.group2.zxy * right_anti_dual.group1.yzx) - (this.group2.yzx * right_anti_dual.group1.zxy), 
            /* e4235, e4315, e4125, e3215 */
            float4((this.e23 * right_anti_dual.e45) + (this.e45 * right_anti_dual.e23) + (this.e35 * right_anti_dual.e42) + (this.scalar * right_anti_dual.e4235), (this.e31 * right_anti_dual.e45) + (this.e45 * right_anti_dual.e31) + (this.e15 * right_anti_dual.e43) + (this.scalar * right_anti_dual.e4315), (this.e12 * right_anti_dual.e45) + (this.e45 * right_anti_dual.e12) + (this.e25 * right_anti_dual.e41) + (this.scalar * right_anti_dual.e4125), -(this.e23 * right_anti_dual.e15) - (this.e31 * right_anti_dual.e25) - (this.e12 * right_anti_dual.e35) - (this.e35 * right_anti_dual.e12)) + float4((this.group0.xyz.yzx * right_anti_dual.group3.zxy), this.scalar * right_anti_dual.e3215) - (this.group2.yzxy * float4(right_anti_dual.group4.xyz.zxy, right_anti_dual.e31)) - float4((this.group0.xyz.zxy * right_anti_dual.group3.yzx), this.e15 * right_anti_dual.e23), 
            /* e1234 */
            (this.scalar * right_anti_dual.e1234) - (this.e41 * right_anti_dual.e23) - (this.e42 * right_anti_dual.e31) - (this.e43 * right_anti_dual.e12) - (this.e23 * right_anti_dual.e41) - (this.e31 * right_anti_dual.e42) - (this.e12 * right_anti_dual.e43)
        );
        return MultiVector(
            /* scalar, e12345 */
            float2((other.scalar * wedge.e12345) + (other.e12345 * wedge.scalar) + (other.e1 * wedge.e4235) + (other.e2 * wedge.e4315) + (other.e3 * wedge.e4125) + (other.e4 * wedge.e3215) + (other.e5 * wedge.e1234) + (other.e4235 * wedge.e1) + (other.e4315 * wedge.e2) + (other.e4125 * wedge.e3) + (other.e3215 * wedge.e4) + (other.e1234 * wedge.e5) - (other.e15 * wedge.e423) - (other.e25 * wedge.e431) - (other.e35 * wedge.e412) - (other.e45 * wedge.e321) - (other.e41 * wedge.e235) - (other.e42 * wedge.e315) - (other.e43 * wedge.e125) - (other.e23 * wedge.e415) - (other.e31 * wedge.e425) - (other.e12 * wedge.e435) - (other.e415 * wedge.e23) - (other.e425 * wedge.e31) - (other.e435 * wedge.e12) - (other.e321 * wedge.e45) - (other.e423 * wedge.e15) - (other.e431 * wedge.e25) - (other.e412 * wedge.e35) - (other.e235 * wedge.e41) - (other.e315 * wedge.e42) - (other.e125 * wedge.e43), other.e12345 * wedge.e12345), 
            /* e1, e2, e3, e4 */
            float4((other.e415 * wedge.e321) + (other.e321 * wedge.e415) + (other.e4315 * wedge.e12) + (other.e1234 * wedge.e15), (other.e425 * wedge.e321) + (other.e321 * wedge.e425) + (other.e4125 * wedge.e23) + (other.e1234 * wedge.e25), (other.e435 * wedge.e321) + (other.e321 * wedge.e435) + (other.e4235 * wedge.e31) + (other.e1234 * wedge.e35), -(other.e45 * wedge.e1234) - (other.e415 * wedge.e423) - (other.e425 * wedge.e431) - (other.e435 * wedge.e412)) + (float4(other.e12345) * wedge.group1) + (float4(wedge.e12345) * other.group1) + float4((other.group4.xyz * wedge.group9.www), other.e4235 * wedge.e41) + float4((other.group5.xyz.yzx * wedge.group9.zxy), other.e4315 * wedge.e42) + float4((other.group7.xyz.zxy * wedge.group8.xyz.yzx), other.e4125 * wedge.e43) + float4((other.group8.xyz.yzx * wedge.group7.xyz.zxy), other.e1234 * wedge.e45) - (wedge.group9.yzxx * float4(other.group5.xyz.zxy, other.e41)) - float4((float3(wedge.e1234) * other.group3.xyz), other.e412 * wedge.e435) - float4((wedge.group4.xyz * other.group9.www), other.e423 * wedge.e415) - float4((other.group7.xyz.yzx * wedge.group8.xyz.zxy), other.e42 * wedge.e4315) - float4((other.group8.xyz.zxy * wedge.group7.xyz.yzx), other.e43 * wedge.e4125) - float4((wedge.group5.xyz.yzx * other.group9.zxy), other.e431 * wedge.e425), 
            /* e5 */
            (other.e12345 * wedge.e5) + (other.e5 * wedge.e12345) + (other.e15 * wedge.e4235) + (other.e25 * wedge.e4315) + (other.e35 * wedge.e4125) + (other.e45 * wedge.e3215) - (other.e415 * wedge.e235) - (other.e425 * wedge.e315) - (other.e435 * wedge.e125) - (other.e235 * wedge.e415) - (other.e315 * wedge.e425) - (other.e125 * wedge.e435) - (other.e4235 * wedge.e15) - (other.e4315 * wedge.e25) - (other.e4125 * wedge.e35) - (other.e3215 * wedge.e45), 
            /* e15, e25, e35, e45 */
            float4((other.e415 * wedge.e3215) + (other.e315 * wedge.e4125) + (other.e4125 * wedge.e315) + (other.e3215 * wedge.e415), (other.e425 * wedge.e3215) + (other.e125 * wedge.e4235) + (other.e4235 * wedge.e125) + (other.e3215 * wedge.e425), (other.e435 * wedge.e3215) + (other.e235 * wedge.e4315) + (other.e4315 * wedge.e235) + (other.e3215 * wedge.e435), -(other.e435 * wedge.e4125) - (other.e4235 * wedge.e415) - (other.e4315 * wedge.e425) - (other.e4125 * wedge.e435)) + (float4(other.e12345) * wedge.group3) + (float4(wedge.e12345) * other.group3) - (wedge.group9.yzxx * float4(other.group8.xyz.zxy, other.e415)) - float4((wedge.group8.xyz.zxy * other.group9.yzx), other.e425 * wedge.e4315), 
            /* e41, e42, e43 */
            (float3(other.e12345) * wedge.group4.xyz) + (float3(other.e1234) * wedge.group6.xyz) + (float3(wedge.e12345) * other.group4.xyz) + (float3(wedge.e1234) * other.group6.xyz) + (other.group7.xyz.zxy * wedge.group9.yzx) + (wedge.group7.xyz.zxy * other.group9.yzx) - (other.group7.xyz.yzx * wedge.group9.zxy) - (wedge.group7.xyz.yzx * other.group9.zxy), 
            /* e23, e31, e12 */
            (float3(other.e12345) * wedge.group5.xyz) + (float3(other.e3215) * wedge.group7.xyz) + (float3(other.e1234) * wedge.group8.xyz) + (float3(wedge.e12345) * other.group5.xyz) + (float3(wedge.e3215) * other.group7.xyz) + (float3(wedge.e1234) * other.group8.xyz) - (float3(other.e321) * wedge.group9.xyz) - (float3(wedge.e321) * other.group9.xyz), 
            /* e415, e425, e435, e321 */
            (float4(other.e12345) * wedge.group6) + (float4(wedge.e12345) * other.group6) + float4((other.group9.zxy * wedge.group9.yzx), other.e1234 * wedge.e3215) - (other.group9.yzxw * float4(wedge.group9.zxy, wedge.e1234)), 
            /* e423, e431, e412 */
            (float3(other.e12345) * wedge.group7.xyz) + (float3(other.e1234) * wedge.group9.xyz) + (float3(wedge.e12345) * other.group7.xyz) - (float3(wedge.e1234) * other.group9.xyz), 
            /* e235, e315, e125 */
            (float3(other.e12345) * wedge.group8.xyz) + (float3(wedge.e12345) * other.group8.xyz) + (float3(wedge.e3215) * other.group9.xyz) - (float3(other.e3215) * wedge.group9.xyz), 
            /* e4235, e4315, e4125, e3215 */
            (float4(other.e12345) * wedge.group9) + (float4(wedge.e12345) * other.group9), 
            /* e1234 */
            (other.e12345 * wedge.e1234) + (other.e1234 * wedge.e12345)
        );
    }
}
public extension AntiCircleRotor {
    public AntiCircleRotor project_orthogonally_onto(Plane other) {
        AntiPlane right_anti_dual = AntiPlane(
            /* e1, e2, e3, e5 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        AntiDipoleInversion wedge = AntiDipoleInversion(
            /* e423, e431, e412 */
            (this.group0.xyz.yzx * right_anti_dual.group0.zxy) - (this.group0.xyz.zxy * right_anti_dual.group0.yzx), 
            /* e415, e425, e435, e321 */
            float4(this.e41 * right_anti_dual.e5, this.e42 * right_anti_dual.e5, this.e43 * right_anti_dual.e5, -(this.e31 * right_anti_dual.e2) - (this.e12 * right_anti_dual.e3)) - (this.group1.wwwx * right_anti_dual.group0.xyzx), 
            /* e235, e315, e125, e4 */
            float4((float3(right_anti_dual.e5) * this.group1.xyz), 0.0) + float4((this.group2.zxy * right_anti_dual.group0.yzx), 0.0) - float4((this.group2.yzx * right_anti_dual.group0.zxy), 0.0), 
            /* e1, e2, e3, e5 */
            float4(this.scalar) * right_anti_dual.group0
        );
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (wedge.group0.xyz.zxy * other.group0.yzx) - (wedge.group0.xyz.yzx * other.group0.zxy), 
            /* e23, e31, e12, e45 */
            float4(wedge.e423 * other.e3215, wedge.e431 * other.e3215, wedge.e412 * other.e3215, -(wedge.e425 * other.e4315) - (wedge.e435 * other.e4125)) - (wedge.group1.wwwx * other.group0.xyzx), 
            /* e15, e25, e35, scalar */
            float4(wedge.e125 * other.e4315 * -1.0, wedge.e235 * other.e4125 * -1.0, wedge.e315 * other.e4235 * -1.0, (wedge.e2 * other.e4315) + (wedge.e3 * other.e4125)) + (float4(other.e3215) * float4(wedge.group1.xyz, wedge.e4)) + (other.group0.zxyx * float4(wedge.group2.yzx, wedge.e1))
        );
    }
}
public extension AntiCircleRotor {
    public Scalar project_orthogonally_onto(RoundPoint other) {
        Sphere right_anti_dual = Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234 */
            other.e4 * -1.0
        );
        Sphere wedge = Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar) * right_anti_dual.group0, 
            /* e1234 */
            this.scalar * right_anti_dual.e1234
        );
        return Scalar(
            /* scalar */
            (other.e1 * wedge.e4235) + (other.e2 * wedge.e4315) + (other.e3 * wedge.e4125) + (other.e4 * wedge.e3215) + (other.e5 * wedge.e1234)
        );
    }
}
public extension AntiCircleRotor {
    public Scalar project_orthogonally_onto(Scalar other) {
        return Scalar(
            /* scalar */
            pow(other.scalar, 2) * this.scalar
        );
    }
}
public extension AntiCircleRotor {
    public AntiCircleRotor project_orthogonally_onto(Sphere other) {
        RoundPoint right_anti_dual = RoundPoint(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5 */
            other.e3215
        );
        AntiDipoleInversion wedge = AntiDipoleInversion(
            /* e423, e431, e412 */
            (float3(right_anti_dual.e4) * this.group1.xyz) + (this.group0.xyz.yzx * right_anti_dual.group0.zxy) - (this.group0.xyz.zxy * right_anti_dual.group0.yzx), 
            /* e415, e425, e435, e321 */
            float4((this.e41 * right_anti_dual.e5) + (this.e15 * right_anti_dual.e4), (this.e42 * right_anti_dual.e5) + (this.e25 * right_anti_dual.e4), (this.e43 * right_anti_dual.e5) + (this.e35 * right_anti_dual.e4), -(this.e31 * right_anti_dual.e2) - (this.e12 * right_anti_dual.e3)) - (this.group1.wwwx * right_anti_dual.group0.xyzx), 
            /* e235, e315, e125, e4 */
            float4(((float3(right_anti_dual.e5) * this.group1.xyz) + (this.group2.zxy * right_anti_dual.group0.yzx) - (this.group2.yzx * right_anti_dual.group0.zxy)), this.scalar * right_anti_dual.e4), 
            /* e1, e2, e3, e5 */
            float4(this.scalar) * float4(right_anti_dual.group0.xyz, right_anti_dual.e5)
        );
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (float3(other.e1234) * wedge.group1.xyz) + (wedge.group0.xyz.zxy * other.group0.yzx) - (wedge.group0.xyz.yzx * other.group0.zxy), 
            /* e23, e31, e12, e45 */
            float4((wedge.e423 * other.e3215) + (wedge.e235 * other.e1234), (wedge.e431 * other.e3215) + (wedge.e315 * other.e1234), (wedge.e412 * other.e3215) + (wedge.e125 * other.e1234), -(wedge.e425 * other.e4315) - (wedge.e435 * other.e4125)) - (wedge.group1.wwwx * other.group0.xyzx), 
            /* e15, e25, e35, scalar */
            float4(wedge.e125 * other.e4315 * -1.0, wedge.e235 * other.e4125 * -1.0, wedge.e315 * other.e4235 * -1.0, (wedge.e2 * other.e4315) + (wedge.e3 * other.e4125) + (wedge.e5 * other.e1234)) + (float4(other.e3215) * float4(wedge.group1.xyz, wedge.e4)) + (other.group0.zxyx * float4(wedge.group2.yzx, wedge.e1))
        );
    }
}
public extension AntiCircleRotor {
    public VersorOdd project_orthogonally_onto(VersorEven other) {
        VersorOdd right_anti_dual = VersorOdd(
            /* e41, e42, e43, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e1234 */
            float4(other.group2.xyz, other.e4) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.group3.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0)
        );
        VersorOdd wedge = VersorOdd(
            /* e41, e42, e43, scalar */
            float4(((float3(this.scalar) * right_anti_dual.group0.xyz) + (float3(right_anti_dual.scalar) * this.group0.xyz)), this.scalar * right_anti_dual.scalar), 
            /* e23, e31, e12, e45 */
            (float4(this.scalar) * right_anti_dual.group1) + (float4(right_anti_dual.scalar) * this.group1), 
            /* e15, e25, e35, e1234 */
            float4(this.scalar * right_anti_dual.e15, this.scalar * right_anti_dual.e25, this.scalar * right_anti_dual.e35, -(this.e41 * right_anti_dual.e23) - (this.e42 * right_anti_dual.e31) - (this.e43 * right_anti_dual.e12) - (this.e23 * right_anti_dual.e41) - (this.e31 * right_anti_dual.e42) - (this.e12 * right_anti_dual.e43)) + (this.group2 * float4(right_anti_dual.group0.www, right_anti_dual.e1234)), 
            /* e4235, e4315, e4125, e3215 */
            float4((this.e23 * right_anti_dual.e45) + (this.e45 * right_anti_dual.e23) + (this.e35 * right_anti_dual.e42) + (this.scalar * right_anti_dual.e4235), (this.e31 * right_anti_dual.e45) + (this.e45 * right_anti_dual.e31) + (this.e15 * right_anti_dual.e43) + (this.scalar * right_anti_dual.e4315), (this.e12 * right_anti_dual.e45) + (this.e45 * right_anti_dual.e12) + (this.e25 * right_anti_dual.e41) + (this.scalar * right_anti_dual.e4125), -(this.e12 * right_anti_dual.e35) - (this.e15 * right_anti_dual.e23) - (this.e25 * right_anti_dual.e31) - (this.e35 * right_anti_dual.e12)) + float4((this.group0.xyz.yzx * right_anti_dual.group2.zxy), this.scalar * right_anti_dual.e3215) - (right_anti_dual.group2.yzxx * float4(this.group0.xyz.zxy, this.e23)) - float4((this.group2.yzx * right_anti_dual.group0.zxy), this.e31 * right_anti_dual.e25)
        );
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (other.group0.zxyw * float4(wedge.group3.yzx, wedge.scalar)) + float4(float3(0.0), (other.e2 * wedge.e4315) + (other.e3 * wedge.e4125) + (other.e4 * wedge.e3215) - (other.e431 * wedge.e25) - (other.e412 * wedge.e35) - (other.e415 * wedge.e23) - (other.e425 * wedge.e31) - (other.e435 * wedge.e12) - (other.e321 * wedge.e45) - (other.e235 * wedge.e41) - (other.e315 * wedge.e42) - (other.e125 * wedge.e43)) + float4((other.group0.www * wedge.group0.xyz), other.e5 * wedge.e1234) + float4((wedge.group2.www * other.group1.xyz), other.e1 * wedge.e4235) - (other.group0.yzxx * float4(wedge.group3.zxy, wedge.e15)), 
            /* e23, e31, e12, e45 */
            float4((other.e12345 * wedge.e23) + (other.e235 * wedge.e1234), (other.e12345 * wedge.e31) + (other.e315 * wedge.e1234), (other.e12345 * wedge.e12) + (other.e125 * wedge.e1234), -(other.e425 * wedge.e4315) - (other.e435 * wedge.e4125)) + (other.group0 * float4(wedge.group3.www, wedge.e45)) - (other.group1.wwwx * wedge.group3.xyzx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(other.e12345) * wedge.group2.xyz) + (float3(wedge.e3215) * other.group1.xyz) + (other.group2.yzx * wedge.group3.zxy) - (other.group2.zxy * wedge.group3.yzx)), other.e12345 * wedge.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345) * wedge.group3
        );
    }
}
public extension AntiCircleRotor {
    public VersorOdd project_orthogonally_onto(VersorOdd other) {
        VersorEven right_anti_dual = VersorEven(
            /* e423, e431, e412, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            float4(other.group2.xyz, other.e3215) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4(other.group3.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0)
        );
        VersorEven wedge = VersorEven(
            /* e423, e431, e412, e12345 */
            float4((this.e23 * right_anti_dual.e4) + (this.scalar * right_anti_dual.e423), (this.e31 * right_anti_dual.e4) + (this.scalar * right_anti_dual.e431), (this.e12 * right_anti_dual.e4) + (this.scalar * right_anti_dual.e412), -(this.e42 * right_anti_dual.e315) - (this.e43 * right_anti_dual.e125) - (this.e23 * right_anti_dual.e415) - (this.e31 * right_anti_dual.e425) - (this.e12 * right_anti_dual.e435) - (this.e45 * right_anti_dual.e321) - (this.e15 * right_anti_dual.e423) - (this.e25 * right_anti_dual.e431) - (this.e35 * right_anti_dual.e412)) + float4((this.group0.xyz.yzx * right_anti_dual.group3.zxy), this.scalar * right_anti_dual.e12345) - float4((this.group0.xyz.zxy * right_anti_dual.group3.yzx), this.e41 * right_anti_dual.e235), 
            /* e415, e425, e435, e321 */
            float4((this.e15 * right_anti_dual.e4) + (this.scalar * right_anti_dual.e415), (this.e25 * right_anti_dual.e4) + (this.scalar * right_anti_dual.e425), (this.e35 * right_anti_dual.e4) + (this.scalar * right_anti_dual.e435), -(this.e31 * right_anti_dual.e2) - (this.e12 * right_anti_dual.e3)) + float4((this.group0.xyz * right_anti_dual.group2.www), this.scalar * right_anti_dual.e321) - (this.group1.wwwx * right_anti_dual.group3.xyzx), 
            /* e235, e315, e125, e5 */
            float4(((float3(this.scalar) * right_anti_dual.group2.xyz) + (float3(right_anti_dual.e5) * this.group1.xyz) + (this.group2.zxy * right_anti_dual.group3.yzx) - (this.group2.yzx * right_anti_dual.group3.zxy)), this.scalar * right_anti_dual.e5), 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * right_anti_dual.group3
        );
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (wedge.group0.zxyw * float4(other.group3.yzx, other.scalar)) + float4(float3(0.0), (wedge.e2 * other.e4315) + (wedge.e3 * other.e4125) + (wedge.e4 * other.e3215) - (wedge.e431 * other.e25) - (wedge.e412 * other.e35) - (wedge.e415 * other.e23) - (wedge.e425 * other.e31) - (wedge.e435 * other.e12) - (wedge.e321 * other.e45) - (wedge.e235 * other.e41) - (wedge.e315 * other.e42) - (wedge.e125 * other.e43)) + float4((wedge.group0.www * other.group0.xyz), wedge.e5 * other.e1234) + float4((other.group2.www * wedge.group1.xyz), wedge.e1 * other.e4235) - (wedge.group0.yzxx * float4(other.group3.zxy, other.e15)), 
            /* e23, e31, e12, e45 */
            float4((wedge.e12345 * other.e23) + (wedge.e235 * other.e1234), (wedge.e12345 * other.e31) + (wedge.e315 * other.e1234), (wedge.e12345 * other.e12) + (wedge.e125 * other.e1234), -(wedge.e425 * other.e4315) - (wedge.e435 * other.e4125)) + (wedge.group0 * float4(other.group3.www, other.e45)) - (wedge.group1.wwwx * other.group3.xyzx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(wedge.e12345) * other.group2.xyz) + (float3(other.e3215) * wedge.group1.xyz) + (wedge.group2.yzx * other.group3.zxy) - (wedge.group2.zxy * other.group3.yzx)), wedge.e12345 * other.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(wedge.e12345) * other.group3
        );
    }
}
public extension AntiDipoleInversion {
    // Fancy infix trick (first half)
    public ProjectOrthogonallyOntoInfixPartial<AntiDipoleInversion> operator /(ProjectOrthogonallyOntoInfix rhs) {
        return ProjectOrthogonallyOntoInfixPartial<AntiDipoleInversion>(this);
    }
}
public extension ProjectOrthogonallyOntoInfixPartial<AntiDipoleInversion> {
    // Fancy infix trick (second half)
    public RoundPoint operator /(AntiCircleRotor rhs) {
        return this.a.project_orthogonally_onto(rhs);
    }
}
public extension AntiDipoleInversion {
    public RoundPoint project_orthogonally_onto(AntiCircleRotor other) {
        CircleRotor right_anti_dual = CircleRotor(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        Sphere wedge = Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((this.e4 * right_anti_dual.e235) + (this.e3 * right_anti_dual.e425), (this.e4 * right_anti_dual.e315) + (this.e1 * right_anti_dual.e435), (this.e4 * right_anti_dual.e125) + (this.e2 * right_anti_dual.e415), -(this.e3 * right_anti_dual.e125) - (this.e5 * right_anti_dual.e321)) - (this.group3.yzxy * float4(right_anti_dual.group1.zxy, right_anti_dual.e315)) - float4((right_anti_dual.group0.xyz * this.group3.www), this.e1 * right_anti_dual.e235), 
            /* e1234 */
            (this.e4 * right_anti_dual.e321) + (this.e1 * right_anti_dual.e423) + (this.e2 * right_anti_dual.e431) + (this.e3 * right_anti_dual.e412)
        );
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4((other.e41 * wedge.e3215) + (other.e31 * wedge.e4125), (other.e42 * wedge.e3215) + (other.e12 * wedge.e4235), (other.e43 * wedge.e3215) + (other.e23 * wedge.e4315), -(other.e43 * wedge.e4125) - (other.e45 * wedge.e1234)) - (wedge.group0.yzxx * float4(other.group1.zxy, other.e41)) - float4((float3(wedge.e1234) * other.group2.xyz), other.e42 * wedge.e4315), 
            /* e5 */
            (other.e45 * wedge.e3215) + (other.e15 * wedge.e4235) + (other.e25 * wedge.e4315) + (other.e35 * wedge.e4125)
        );
    }
}
public extension AntiDipoleInversion {
    public AntiDipoleInversion project_orthogonally_onto(AntiDipoleInversion other) {
        DipoleInversion right_anti_dual = DipoleInversion(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e1234 */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group3 * float4(1.0, 1.0, 1.0, -1.0)
        );
        CircleRotor wedge = CircleRotor(
            /* e423, e431, e412 */
            (float3(this.e4) * right_anti_dual.group1.xyz) + (right_anti_dual.group0.xyz.yzx * this.group3.zxy) - (right_anti_dual.group0.xyz.zxy * this.group3.yzx), 
            /* e415, e425, e435, e321 */
            float4((this.e4 * right_anti_dual.e15) + (this.e5 * right_anti_dual.e41), (this.e4 * right_anti_dual.e25) + (this.e5 * right_anti_dual.e42), (this.e4 * right_anti_dual.e35) + (this.e5 * right_anti_dual.e43), -(this.e2 * right_anti_dual.e31) - (this.e3 * right_anti_dual.e12)) - (this.group3.xyzx * right_anti_dual.group1.wwwx), 
            /* e235, e315, e125, e12345 */
            (this.group3.wwwx * float4(right_anti_dual.group1.xyz, right_anti_dual.e4235)) + float4(float3(0.0), (this.e2 * right_anti_dual.e4315) + (this.e3 * right_anti_dual.e4125) + (this.e5 * right_anti_dual.e1234) - (this.e431 * right_anti_dual.e25) - (this.e412 * right_anti_dual.e35) - (this.e415 * right_anti_dual.e23) - (this.e425 * right_anti_dual.e31) - (this.e435 * right_anti_dual.e12) - (this.e321 * right_anti_dual.e45) - (this.e235 * right_anti_dual.e41) - (this.e315 * right_anti_dual.e42) - (this.e125 * right_anti_dual.e43)) + float4((this.group3.yzx * right_anti_dual.group2.zxy), this.e4 * right_anti_dual.e3215) - (right_anti_dual.group2.yzxx * float4(this.group3.zxy, this.e423))
        );
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(wedge.e12345) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(wedge.e12345) * other.group1, 
            /* e235, e315, e125, e4 */
            float4((float3(wedge.e12345) * other.group2.xyz), (other.e4 * wedge.e12345) - (other.e423 * wedge.e415) - (other.e431 * wedge.e425) - (other.e412 * wedge.e435) - (other.e415 * wedge.e423) - (other.e425 * wedge.e431) - (other.e435 * wedge.e412)), 
            /* e1, e2, e3, e5 */
            float4((other.e415 * wedge.e321) + (other.e321 * wedge.e415) + (other.e315 * wedge.e412) + (other.e1 * wedge.e12345), (other.e425 * wedge.e321) + (other.e321 * wedge.e425) + (other.e125 * wedge.e423) + (other.e2 * wedge.e12345), (other.e435 * wedge.e321) + (other.e321 * wedge.e435) + (other.e235 * wedge.e431) + (other.e3 * wedge.e12345), -(other.e435 * wedge.e125) - (other.e235 * wedge.e415) - (other.e315 * wedge.e425) - (other.e125 * wedge.e435)) + (wedge.group2.yzxw * float4(other.group0.xyz.zxy, other.e5)) - (wedge.group2.zxyx * float4(other.group0.xyz.yzx, other.e415)) - float4((wedge.group0.xyz.yzx * other.group2.zxy), other.e425 * wedge.e315)
        );
    }
}
public extension AntiDipoleInversion {
    public AntiFlector project_orthogonally_onto(AntiDualNum other) {
        DualNum right_anti_dual = DualNum(
            /* e5, e12345 */
            other.group0.xy
        );
        Flector wedge = Flector(
            /* e15, e25, e35, e45 */
            float4(right_anti_dual.e5) * float4(this.group3.xyz, this.e4), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual.e5) * float4(this.group0.xyz, this.e321)
        );
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(other.group0.xy.xx, other.e3215, 0.0) * float4(float3(1.0), 0.0) * float4(wedge.group1.xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), other.e3215 * wedge.e45 * -1.0)
        );
    }
}
public extension AntiDipoleInversion {
    public AntiFlector project_orthogonally_onto(AntiFlatPoint other) {
        FlatPoint right_anti_dual = FlatPoint(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        Motor wedge = Motor(
            /* e415, e425, e435, e12345 */
            float4(this.e4 * right_anti_dual.e15, this.e4 * right_anti_dual.e25, this.e4 * right_anti_dual.e35, -(this.e431 * right_anti_dual.e25) - (this.e412 * right_anti_dual.e35) - (this.e321 * right_anti_dual.e45)) - (right_anti_dual.group0.wwwx * float4(this.group3.xyz, this.e423)), 
            /* e235, e315, e125, e5 */
            float4(((this.group3.yzx * right_anti_dual.group0.zxy) - (this.group3.zxy * right_anti_dual.group0.yzx)), 0.0)
        );
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(wedge.e12345) * other.group0, 
            /* e1, e2, e3, e5 */
            float4((other.group0.www * wedge.group0.xyz), -(other.e235 * wedge.e415) - (other.e315 * wedge.e425) - (other.e125 * wedge.e435))
        );
    }
}
public extension AntiDipoleInversion {
    public AntiFlector project_orthogonally_onto(AntiFlector other) {
        Flector right_anti_dual = Flector(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        Motor wedge = Motor(
            /* e415, e425, e435, e12345 */
            (float4(this.e4) * float4(right_anti_dual.group0.xyz, right_anti_dual.e3215)) + float4(float3(0.0), (this.e1 * right_anti_dual.e4235) + (this.e2 * right_anti_dual.e4315) + (this.e3 * right_anti_dual.e4125) - (this.e431 * right_anti_dual.e25) - (this.e412 * right_anti_dual.e35) - (this.e321 * right_anti_dual.e45)) - (right_anti_dual.group0.wwwx * float4(this.group3.xyz, this.e423)), 
            /* e235, e315, e125, e5 */
            float4(((this.group3.yzx * right_anti_dual.group0.zxy) - (this.group3.zxy * right_anti_dual.group0.yzx)), 0.0)
        );
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(wedge.e12345) * other.group0, 
            /* e1, e2, e3, e5 */
            float4(other.e1 * wedge.e12345, other.e2 * wedge.e12345, other.e3 * wedge.e12345, -(other.e235 * wedge.e415) - (other.e315 * wedge.e425) - (other.e125 * wedge.e435)) + (wedge.group0 * float4(other.group0.www, other.e5))
        );
    }
}
public extension AntiDipoleInversion {
    public AntiPlane project_orthogonally_onto(AntiLine other) {
        Line right_anti_dual = Line(
            /* e415, e425, e435 */
            other.group0.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            other.group1.xyz * float3(-1.0)
        );
        Plane wedge = Plane(
            /* e4235, e4315, e4125, e3215 */
            float4((this.e4 * right_anti_dual.e235) + (this.e3 * right_anti_dual.e425), (this.e4 * right_anti_dual.e315) + (this.e1 * right_anti_dual.e435), (this.e4 * right_anti_dual.e125) + (this.e2 * right_anti_dual.e415), -(this.e2 * right_anti_dual.e315) - (this.e3 * right_anti_dual.e125)) - (this.group3.yzxx * float4(right_anti_dual.group0.xyz.zxy, right_anti_dual.e235))
        );
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(other.e12 * wedge.e4315 * -1.0, other.e23 * wedge.e4125 * -1.0, other.e31 * wedge.e4235 * -1.0, (other.e25 * wedge.e4315) + (other.e35 * wedge.e4125)) + (wedge.group0.zxyx * float4(other.group0.xyz.yzx, other.e15))
        );
    }
}
public extension AntiDipoleInversion {
    public AntiFlector project_orthogonally_onto(AntiMotor other) {
        Motor right_anti_dual = Motor(
            /* e415, e425, e435, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        Flector wedge = Flector(
            /* e15, e25, e35, e45 */
            float4(right_anti_dual.e5) * float4(this.group3.xyz, this.e4), 
            /* e4235, e4315, e4125, e3215 */
            float4((this.e4 * right_anti_dual.e235) + (this.e3 * right_anti_dual.e425), (this.e4 * right_anti_dual.e315) + (this.e1 * right_anti_dual.e435), (this.e4 * right_anti_dual.e125) + (this.e2 * right_anti_dual.e415), -(this.e2 * right_anti_dual.e315) - (this.e3 * right_anti_dual.e125)) + (float4(right_anti_dual.e5) * float4(this.group0.xyz, this.e321)) - (this.group3.yzxx * float4(right_anti_dual.group0.zxy, right_anti_dual.e235))
        );
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(float3(1.0), 0.0) * float4(other.group1.www, 0.0) * float4(wedge.group1.xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0), 
            /* e1, e2, e3, e5 */
            (wedge.group1.zxyx * float4(other.group0.yzx, other.e15)) + float4(float3(0.0), (other.e25 * wedge.e4315) + (other.e35 * wedge.e4125)) - float4((other.group0.zxy * wedge.group1.yzx), other.e3215 * wedge.e45)
        );
    }
}
public extension AntiDipoleInversion {
    public AntiPlane project_orthogonally_onto(AntiPlane other) {
        Plane right_anti_dual = Plane(
            /* e4235, e4315, e4125, e3215 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4((this.e4 * right_anti_dual.e3215) + (this.e1 * right_anti_dual.e4235) + (this.e2 * right_anti_dual.e4315) + (this.e3 * right_anti_dual.e4125)) * other.group0
        );
    }
}
public extension AntiDipoleInversion {
    public AntiDipoleInversion project_orthogonally_onto(AntiScalar other) {
        Scalar right_anti_dual = Scalar(
            /* scalar */
            other.e12345 * -1.0
        );
        AntiDipoleInversion wedge = AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(right_anti_dual.scalar) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(right_anti_dual.scalar) * this.group1, 
            /* e235, e315, e125, e4 */
            float4(right_anti_dual.scalar) * this.group2, 
            /* e1, e2, e3, e5 */
            float4(right_anti_dual.scalar) * this.group3
        );
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(other.e12345) * wedge.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(other.e12345) * wedge.group1, 
            /* e235, e315, e125, e4 */
            float4(other.e12345) * wedge.group2, 
            /* e1, e2, e3, e5 */
            float4(other.e12345) * wedge.group3
        );
    }
}
public extension AntiDipoleInversion {
    public AntiDipoleInversion project_orthogonally_onto(Circle other) {
        Dipole right_anti_dual = Dipole(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35 */
            other.group2.xyz
        );
        CircleRotor wedge = CircleRotor(
            /* e423, e431, e412 */
            (float3(this.e4) * right_anti_dual.group1.xyz) + (right_anti_dual.group0.xyz.yzx * this.group3.zxy) - (right_anti_dual.group0.xyz.zxy * this.group3.yzx), 
            /* e415, e425, e435, e321 */
            float4((this.e4 * right_anti_dual.e15) + (this.e5 * right_anti_dual.e41), (this.e4 * right_anti_dual.e25) + (this.e5 * right_anti_dual.e42), (this.e4 * right_anti_dual.e35) + (this.e5 * right_anti_dual.e43), -(this.e2 * right_anti_dual.e31) - (this.e3 * right_anti_dual.e12)) - (this.group3.xyzx * right_anti_dual.group1.wwwx), 
            /* e235, e315, e125, e12345 */
            float4((this.e2 * right_anti_dual.e35) + (this.e5 * right_anti_dual.e23), (this.e3 * right_anti_dual.e15) + (this.e5 * right_anti_dual.e31), (this.e1 * right_anti_dual.e25) + (this.e5 * right_anti_dual.e12), -(this.e431 * right_anti_dual.e25) - (this.e412 * right_anti_dual.e35) - (this.e415 * right_anti_dual.e23) - (this.e425 * right_anti_dual.e31) - (this.e435 * right_anti_dual.e12) - (this.e321 * right_anti_dual.e45) - (this.e235 * right_anti_dual.e41) - (this.e315 * right_anti_dual.e42) - (this.e125 * right_anti_dual.e43)) - float4((right_anti_dual.group2.xyz.yzx * this.group3.zxy), this.e423 * right_anti_dual.e15)
        );
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(wedge.e12345) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(wedge.e12345) * other.group1, 
            /* e235, e315, e125, e4 */
            float4((float3(wedge.e12345) * other.group2.xyz), -(other.e423 * wedge.e415) - (other.e431 * wedge.e425) - (other.e412 * wedge.e435) - (other.e415 * wedge.e423) - (other.e425 * wedge.e431) - (other.e435 * wedge.e412)), 
            /* e1, e2, e3, e5 */
            float4((other.e412 * wedge.e315) + (other.e415 * wedge.e321) + (other.e321 * wedge.e415) + (other.e315 * wedge.e412), (other.e423 * wedge.e125) + (other.e425 * wedge.e321) + (other.e321 * wedge.e425) + (other.e125 * wedge.e423), (other.e431 * wedge.e235) + (other.e435 * wedge.e321) + (other.e321 * wedge.e435) + (other.e235 * wedge.e431), -(other.e415 * wedge.e235) - (other.e425 * wedge.e315) - (other.e435 * wedge.e125) - (other.e125 * wedge.e435)) - float4((other.group0.xyz.yzx * wedge.group2.zxy), other.e235 * wedge.e415) - float4((other.group2.xyz.zxy * wedge.group0.xyz.yzx), other.e315 * wedge.e425)
        );
    }
}
public extension AntiDipoleInversion {
    public VersorEven project_orthogonally_onto(CircleRotor other) {
        AntiCircleRotor right_anti_dual = AntiCircleRotor(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        VersorEven wedge = VersorEven(
            /* e423, e431, e412, e12345 */
            float4((right_anti_dual.e42 * this.e3) + (right_anti_dual.e23 * this.e4) + (right_anti_dual.scalar * this.e423), (right_anti_dual.e43 * this.e1) + (right_anti_dual.e31 * this.e4) + (right_anti_dual.scalar * this.e431), (right_anti_dual.e41 * this.e2) + (right_anti_dual.e12 * this.e4) + (right_anti_dual.scalar * this.e412), -(right_anti_dual.e42 * this.e315) - (right_anti_dual.e43 * this.e125) - (right_anti_dual.e23 * this.e415) - (right_anti_dual.e31 * this.e425) - (right_anti_dual.e12 * this.e435) - (right_anti_dual.e45 * this.e321) - (right_anti_dual.e15 * this.e423) - (right_anti_dual.e25 * this.e431) - (right_anti_dual.e35 * this.e412)) - float4((right_anti_dual.group0.xyz.zxy * this.group3.yzx), right_anti_dual.e41 * this.e235), 
            /* e415, e425, e435, e321 */
            float4((right_anti_dual.e15 * this.e4) + (right_anti_dual.scalar * this.e415), (right_anti_dual.e25 * this.e4) + (right_anti_dual.scalar * this.e425), (right_anti_dual.e35 * this.e4) + (right_anti_dual.scalar * this.e435), -(right_anti_dual.e31 * this.e2) - (right_anti_dual.e12 * this.e3)) + float4((right_anti_dual.group0.xyz * this.group3.www), right_anti_dual.scalar * this.e321) - (right_anti_dual.group1.wwwx * this.group3.xyzx), 
            /* e235, e315, e125, e5 */
            float4(((float3(right_anti_dual.scalar) * this.group2.xyz) + (float3(this.e5) * right_anti_dual.group1.xyz) + (right_anti_dual.group2.zxy * this.group3.yzx) - (right_anti_dual.group2.yzx * this.group3.zxy)), right_anti_dual.scalar * this.e5), 
            /* e1, e2, e3, e4 */
            float4(right_anti_dual.scalar) * float4(this.group3.xyz, this.e4)
        );
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(((float3(other.e12345) * wedge.group0.xyz) + (float3(wedge.e12345) * other.group0.xyz)), other.e12345 * wedge.e12345), 
            /* e415, e425, e435, e321 */
            (float4(other.e12345) * wedge.group1) + (float4(wedge.e12345) * other.group1), 
            /* e235, e315, e125, e5 */
            float4(other.e12345 * wedge.e235, other.e12345 * wedge.e315, other.e12345 * wedge.e125, -(other.e415 * wedge.e235) - (other.e425 * wedge.e315) - (other.e435 * wedge.e125) - (other.e235 * wedge.e415) - (other.e315 * wedge.e425) - (other.e125 * wedge.e435)) + (other.group2 * float4(wedge.group0.www, wedge.e5)), 
            /* e1, e2, e3, e4 */
            float4((other.e415 * wedge.e321) + (other.e321 * wedge.e415) + (other.e315 * wedge.e412) + (other.e12345 * wedge.e1), (other.e425 * wedge.e321) + (other.e321 * wedge.e425) + (other.e125 * wedge.e423) + (other.e12345 * wedge.e2), (other.e435 * wedge.e321) + (other.e321 * wedge.e435) + (other.e235 * wedge.e431) + (other.e12345 * wedge.e3), -(other.e412 * wedge.e435) - (other.e415 * wedge.e423) - (other.e425 * wedge.e431) - (other.e435 * wedge.e412)) + float4((other.group0.xyz.zxy * wedge.group2.yzx), other.e12345 * wedge.e4) - float4((other.group0.xyz.yzx * wedge.group2.zxy), other.e423 * wedge.e415) - float4((other.group2.zxy * wedge.group0.yzx), other.e431 * wedge.e425)
        );
    }
}
public extension AntiDipoleInversion {
    public RoundPoint project_orthogonally_onto(Dipole other) {
        Circle right_anti_dual = Circle(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125 */
            other.group2.xyz * float3(-1.0)
        );
        Sphere wedge = Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((this.e4 * right_anti_dual.e235) + (this.e3 * right_anti_dual.e425), (this.e4 * right_anti_dual.e315) + (this.e1 * right_anti_dual.e435), (this.e4 * right_anti_dual.e125) + (this.e2 * right_anti_dual.e415), -(this.e3 * right_anti_dual.e125) - (this.e5 * right_anti_dual.e321)) - (this.group3.wwwx * float4(right_anti_dual.group0.xyz, right_anti_dual.e235)) - float4((this.group3.yzx * right_anti_dual.group1.zxy), this.e2 * right_anti_dual.e315), 
            /* e1234 */
            (this.e4 * right_anti_dual.e321) + (this.e1 * right_anti_dual.e423) + (this.e2 * right_anti_dual.e431) + (this.e3 * right_anti_dual.e412)
        );
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4((other.e41 * wedge.e3215) + (other.e31 * wedge.e4125), (other.e42 * wedge.e3215) + (other.e12 * wedge.e4235), (other.e43 * wedge.e3215) + (other.e23 * wedge.e4315), -(other.e43 * wedge.e4125) - (other.e45 * wedge.e1234)) - (wedge.group0.yzxy * float4(other.group1.zxy, other.e42)) - float4((float3(wedge.e1234) * other.group2.xyz), other.e41 * wedge.e4235), 
            /* e5 */
            (other.e45 * wedge.e3215) + (other.e15 * wedge.e4235) + (other.e25 * wedge.e4315) + (other.e35 * wedge.e4125)
        );
    }
}
public extension AntiDipoleInversion {
    public AntiDipoleInversion project_orthogonally_onto(DipoleInversion other) {
        AntiDipoleInversion right_anti_dual = AntiDipoleInversion(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e4 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group3 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        DipoleInversion wedge = DipoleInversion(
            /* e41, e42, e43 */
            (float3(this.e4) * right_anti_dual.group3.xyz) - (float3(right_anti_dual.e4) * this.group3.xyz), 
            /* e23, e31, e12, e45 */
            (right_anti_dual.group3.zxyw * float4(this.group3.yzx, this.e4)) - (this.group3.zxyw * float4(right_anti_dual.group3.yzx, right_anti_dual.e4)), 
            /* e15, e25, e35, e1234 */
            (this.group3.xyzx * float4(right_anti_dual.group3.www, right_anti_dual.e423)) + float4(float3(0.0), (right_anti_dual.e431 * this.e2) + (right_anti_dual.e412 * this.e3) + (right_anti_dual.e321 * this.e4) - (right_anti_dual.e4 * this.e321) - (right_anti_dual.e2 * this.e431) - (right_anti_dual.e3 * this.e412)) - float4((this.group3.www * right_anti_dual.group3.xyz), right_anti_dual.e1 * this.e423), 
            /* e4235, e4315, e4125, e3215 */
            (right_anti_dual.group3.yzxw * this.group1.zxyw) + (this.group2.wwwz * float4(right_anti_dual.group2.xyz, right_anti_dual.e3)) + float4((this.group0.xyz * right_anti_dual.group3.www), right_anti_dual.e1 * this.e235) + float4((right_anti_dual.group1.yzx * this.group3.zxy), right_anti_dual.e2 * this.e315) - (float4(this.e5) * float4(right_anti_dual.group0.xyz, right_anti_dual.e321)) - (right_anti_dual.group2.wwwy * float4(this.group2.xyz, this.e2)) - (this.group3.yzxx * float4(right_anti_dual.group1.zxy, right_anti_dual.e235)) - float4((right_anti_dual.group3.zxy * this.group1.yzx), right_anti_dual.e125 * this.e3)
        );
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (float3(other.e1234) * wedge.group3.xyz) - (float3(wedge.e1234) * other.group3.xyz), 
            /* e415, e425, e435, e321 */
            (wedge.group3.yzxw * float4(other.group3.zxy, other.e1234)) - (other.group3.yzxw * float4(wedge.group3.zxy, wedge.e1234)), 
            /* e235, e315, e125, e4 */
            float4(float3(0.0), (other.e1234 * wedge.e45) + (other.e4315 * wedge.e42) + (other.e4125 * wedge.e43) - (other.e42 * wedge.e4315) - (other.e43 * wedge.e4125) - (other.e45 * wedge.e1234)) + float4((wedge.group3.www * other.group3.xyz), other.e4235 * wedge.e41) - (wedge.group3.xyzx * float4(other.group3.www, other.e41)), 
            /* e1, e2, e3, e5 */
            (float4(wedge.e3215) * float4(other.group0.xyz, other.e45)) + (other.group2.wwwy * float4(wedge.group2.xyz, wedge.e4315)) + (wedge.group3.zxyx * float4(other.group1.yzx, other.e15)) + float4((other.group3.yzx * wedge.group1.zxy), other.e35 * wedge.e4125) - (other.group3.zxyw * wedge.group1.yzxw) - (wedge.group2.wwwz * float4(other.group2.xyz, other.e4125)) - float4((wedge.group0.xyz * other.group3.www), other.e4235 * wedge.e15) - float4((other.group1.zxy * wedge.group3.yzx), other.e4315 * wedge.e25)
        );
    }
}
public extension AntiDipoleInversion {
    public VersorEven project_orthogonally_onto(DualNum other) {
        AntiDualNum right_anti_dual = AntiDualNum(
            /* e3215, scalar */
            other.group0.xy * float2(-1.0)
        );
        VersorEven wedge = VersorEven(
            /* e423, e431, e412, e12345 */
            float4(right_anti_dual.group0.xy.yy, right_anti_dual.scalar, right_anti_dual.e3215) * float4(this.group0.xyz, this.e4), 
            /* e415, e425, e435, e321 */
            float4(right_anti_dual.scalar) * this.group1, 
            /* e235, e315, e125, e5 */
            float4(right_anti_dual.scalar) * float4(this.group2.xyz, this.e5), 
            /* e1, e2, e3, e4 */
            float4(right_anti_dual.scalar) * float4(this.group3.xyz, this.e4)
        );
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(other.e12345) * wedge.group0, 
            /* e415, e425, e435, e321 */
            float4(other.e12345) * wedge.group1, 
            /* e235, e315, e125, e5 */
            float4(other.group0.xy.yy, other.e12345, (other.e5 * wedge.e12345) + (other.e12345 * wedge.e5)) * float4(wedge.group2.xyz, 1.0), 
            /* e1, e2, e3, e4 */
            float4(other.e12345) * wedge.group3
        );
    }
}
public extension AntiDipoleInversion {
    public RoundPoint project_orthogonally_onto(FlatPoint other) {
        AntiFlatPoint right_anti_dual = AntiFlatPoint(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        Sphere wedge = Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((this.group2.www * right_anti_dual.group0.xyz), -(this.e1 * right_anti_dual.e235) - (this.e2 * right_anti_dual.e315) - (this.e3 * right_anti_dual.e125) - (this.e5 * right_anti_dual.e321)), 
            /* e1234 */
            this.e4 * right_anti_dual.e321
        );
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(wedge.e1234) * other.group0 * float4(-1.0), 
            /* e5 */
            (other.e15 * wedge.e4235) + (other.e25 * wedge.e4315) + (other.e35 * wedge.e4125) + (other.e45 * wedge.e3215)
        );
    }
}
public extension AntiDipoleInversion {
    public AntiDipoleInversion project_orthogonally_onto(Flector other) {
        AntiFlector right_anti_dual = AntiFlector(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        DipoleInversion wedge = DipoleInversion(
            /* e41, e42, e43 */
            float3(this.e4) * right_anti_dual.group1.xyz, 
            /* e23, e31, e12, e45 */
            float4(((this.group3.yzx * right_anti_dual.group1.zxy) - (this.group3.zxy * right_anti_dual.group1.yzx)), this.e4 * right_anti_dual.e5), 
            /* e15, e25, e35, e1234 */
            float4(float3(0.0), -(this.e431 * right_anti_dual.e2) - (this.e412 * right_anti_dual.e3)) + float4((right_anti_dual.group1.www * this.group3.xyz), this.e4 * right_anti_dual.e321) - (right_anti_dual.group1.xyzx * float4(this.group3.www, this.e423)), 
            /* e4235, e4315, e4125, e3215 */
            (float4(right_anti_dual.e5) * float4(this.group0.xyz, this.e321)) + (this.group2.wwwy * float4(right_anti_dual.group0.xyz, right_anti_dual.e2)) + (right_anti_dual.group1.yzxx * float4(this.group1.zxy, this.e235)) + float4(float3(0.0), (this.e125 * right_anti_dual.e3) - (this.e2 * right_anti_dual.e315) - (this.e3 * right_anti_dual.e125) - (this.e5 * right_anti_dual.e321)) - float4((this.group1.yzx * right_anti_dual.group1.zxy), this.e1 * right_anti_dual.e235)
        );
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(wedge.e1234) * other.group1.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            float4(((wedge.group3.yzx * other.group1.zxy) - (wedge.group3.zxy * other.group1.yzx)), wedge.e1234 * other.e3215 * -1.0), 
            /* e235, e315, e125, e4 */
            (other.group1.xyzx * float4(wedge.group3.www, wedge.e41)) + float4(float3(0.0), (wedge.e42 * other.e4315) + (wedge.e43 * other.e4125)) - float4((other.group1.www * wedge.group3.xyz), wedge.e1234 * other.e45), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), (wedge.e4315 * other.e25) + (wedge.e4125 * other.e35) + (wedge.e3215 * other.e45) - (wedge.e35 * other.e4125)) + float4((wedge.group1.zxy * other.group1.yzx), wedge.e4235 * other.e15) - (float4(other.e3215) * float4(wedge.group0.xyz, wedge.e45)) - (wedge.group2.wwwy * float4(other.group0.xyz, other.e4315)) - (other.group1.zxyx * float4(wedge.group1.yzx, wedge.e15))
        );
    }
}
public extension AntiDipoleInversion {
    public AntiDipoleInversion project_orthogonally_onto(Line other) {
        AntiLine right_anti_dual = AntiLine(
            /* e23, e31, e12 */
            other.group0.xyz, 
            /* e15, e25, e35 */
            other.group1.xyz
        );
        CircleRotor wedge = CircleRotor(
            /* e423, e431, e412 */
            float3(this.e4) * right_anti_dual.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4((float3(this.e4) * right_anti_dual.group1.xyz), -(this.e1 * right_anti_dual.e23) - (this.e2 * right_anti_dual.e31) - (this.e3 * right_anti_dual.e12)), 
            /* e235, e315, e125, e12345 */
            float4((this.e2 * right_anti_dual.e35) + (this.e5 * right_anti_dual.e23), (this.e3 * right_anti_dual.e15) + (this.e5 * right_anti_dual.e31), (this.e1 * right_anti_dual.e25) + (this.e5 * right_anti_dual.e12), -(this.e431 * right_anti_dual.e25) - (this.e412 * right_anti_dual.e35) - (this.e415 * right_anti_dual.e23) - (this.e425 * right_anti_dual.e31) - (this.e435 * right_anti_dual.e12)) - float4((right_anti_dual.group1.xyz.yzx * this.group3.zxy), this.e423 * right_anti_dual.e15)
        );
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(1.0), 0.0) * float4(other.group0.xyz, 0.0) * float4(wedge.group2.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e235, e315, e125, e4 */
            float4((float3(wedge.e12345) * other.group1.xyz), -(wedge.e423 * other.e415) - (wedge.e431 * other.e425) - (wedge.e412 * other.e435)), 
            /* e1, e2, e3, e5 */
            float4((wedge.e412 * other.e315) + (wedge.e321 * other.e415), (wedge.e423 * other.e125) + (wedge.e321 * other.e425), (wedge.e431 * other.e235) + (wedge.e321 * other.e435), -(wedge.e415 * other.e235) - (wedge.e425 * other.e315) - (wedge.e435 * other.e125) - (wedge.e315 * other.e425) - (wedge.e125 * other.e435)) - float4((wedge.group0.xyz.yzx * other.group1.xyz.zxy), wedge.e235 * other.e415)
        );
    }
}
public extension AntiDipoleInversion {
    public VersorEven project_orthogonally_onto(Motor other) {
        AntiMotor right_anti_dual = AntiMotor(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        VersorEven wedge = VersorEven(
            /* e423, e431, e412, e12345 */
            float4(this.e4 * right_anti_dual.e23, this.e4 * right_anti_dual.e31, this.e4 * right_anti_dual.e12, -(this.e423 * right_anti_dual.e15) - (this.e431 * right_anti_dual.e25) - (this.e412 * right_anti_dual.e35) - (this.e415 * right_anti_dual.e23) - (this.e425 * right_anti_dual.e31) - (this.e435 * right_anti_dual.e12)) + float4((this.group0.xyz * right_anti_dual.group0.www), this.e4 * right_anti_dual.e3215), 
            /* e415, e425, e435, e321 */
            float4(this.e4 * right_anti_dual.e15, this.e4 * right_anti_dual.e25, this.e4 * right_anti_dual.e35, -(this.e1 * right_anti_dual.e23) - (this.e2 * right_anti_dual.e31) - (this.e3 * right_anti_dual.e12)) + (float4(right_anti_dual.scalar) * this.group1), 
            /* e235, e315, e125, e5 */
            float4(((float3(this.e5) * right_anti_dual.group0.xyz) + (float3(right_anti_dual.scalar) * this.group2.xyz) + (this.group3.yzx * right_anti_dual.group1.zxy) - (this.group3.zxy * right_anti_dual.group1.yzx)), this.e5 * right_anti_dual.scalar), 
            /* e1, e2, e3, e4 */
            float4(right_anti_dual.scalar) * float4(this.group3.xyz, this.e4)
        );
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(other.e12345) * wedge.group0, 
            /* e415, e425, e435, e321 */
            float4(((float3(other.e12345) * wedge.group1.xyz) + (float3(wedge.e12345) * other.group0.xyz)), other.e12345 * wedge.e321), 
            /* e235, e315, e125, e5 */
            (float4(other.e12345) * wedge.group2) + (float4(wedge.e12345) * other.group1) + float4(float3(0.0), -(other.e415 * wedge.e235) - (other.e425 * wedge.e315) - (other.e435 * wedge.e125) - (other.e235 * wedge.e415) - (other.e315 * wedge.e425) - (other.e125 * wedge.e435)), 
            /* e1, e2, e3, e4 */
            float4((other.e12345 * wedge.e1) + (other.e315 * wedge.e412), (other.e12345 * wedge.e2) + (other.e125 * wedge.e423), (other.e12345 * wedge.e3) + (other.e235 * wedge.e431), -(other.e425 * wedge.e431) - (other.e435 * wedge.e412)) + (other.group0 * float4(wedge.group1.www, wedge.e4)) - (wedge.group0.yzxx * float4(other.group1.zxy, other.e415))
        );
    }
}
public extension AntiDipoleInversion {
    public MultiVector project_orthogonally_onto(MultiVector other) {
        MultiVector right_anti_dual = MultiVector(
            /* scalar, e12345 */
            other.group0.xy.yx * float2(-1.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4(other.group9.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5 */
            other.e3215, 
            /* e15, e25, e35, e45 */
            float4(other.group8.xyz, other.e321 * -1.0), 
            /* e41, e42, e43 */
            other.group7.xyz, 
            /* e23, e31, e12 */
            other.group6.xyz, 
            /* e415, e425, e435, e321 */
            float4(other.group5.xyz, other.e45) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e423, e431, e412 */
            other.group4.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            other.group3.xyz * float3(-1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.group1.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234 */
            other.e4 * -1.0
        );
        MultiVector wedge = MultiVector(
            /* scalar, e12345 */
            float2(0.0, (this.e4 * right_anti_dual.e3215) + (this.e1 * right_anti_dual.e4235) + (this.e2 * right_anti_dual.e4315) + (this.e3 * right_anti_dual.e4125) + (this.e5 * right_anti_dual.e1234) - (this.e423 * right_anti_dual.e15) - (this.e431 * right_anti_dual.e25) - (this.e412 * right_anti_dual.e35) - (this.e415 * right_anti_dual.e23) - (this.e425 * right_anti_dual.e31) - (this.e435 * right_anti_dual.e12) - (this.e321 * right_anti_dual.e45) - (this.e235 * right_anti_dual.e41) - (this.e315 * right_anti_dual.e42) - (this.e125 * right_anti_dual.e43)), 
            /* e1, e2, e3, e4 */
            float4(right_anti_dual.scalar) * float4(this.group3.xyz, this.e4), 
            /* e5 */
            this.e5 * right_anti_dual.scalar, 
            /* e15, e25, e35, e45 */
            (float4(right_anti_dual.e5) * float4(this.group3.xyz, this.e4)) - (float4(this.e5) * right_anti_dual.group1), 
            /* e41, e42, e43 */
            (float3(this.e4) * right_anti_dual.group1.xyz) - (float3(right_anti_dual.e4) * this.group3.xyz), 
            /* e23, e31, e12 */
            (this.group3.yzx * right_anti_dual.group1.zxy) - (this.group3.zxy * right_anti_dual.group1.yzx), 
            /* e415, e425, e435, e321 */
            float4((this.e4 * right_anti_dual.e15) + (this.e5 * right_anti_dual.e41), (this.e4 * right_anti_dual.e25) + (this.e5 * right_anti_dual.e42), (this.e4 * right_anti_dual.e35) + (this.e5 * right_anti_dual.e43), -(this.e2 * right_anti_dual.e31) - (this.e3 * right_anti_dual.e12)) + (float4(right_anti_dual.scalar) * this.group1) - float4((right_anti_dual.group3.www * this.group3.xyz), this.e1 * right_anti_dual.e23), 
            /* e423, e431, e412 */
            (float3(this.e4) * right_anti_dual.group5.xyz) + (float3(right_anti_dual.scalar) * this.group0.xyz) + (right_anti_dual.group4.xyz.yzx * this.group3.zxy) - (right_anti_dual.group4.xyz.zxy * this.group3.yzx), 
            /* e235, e315, e125 */
            (float3(this.e5) * right_anti_dual.group5.xyz) + (float3(right_anti_dual.scalar) * this.group2.xyz) + (this.group3.yzx * right_anti_dual.group3.zxy) - (this.group3.zxy * right_anti_dual.group3.yzx), 
            /* e4235, e4315, e4125, e3215 */
            (float4(right_anti_dual.e5) * float4(this.group0.xyz, this.e321)) + (right_anti_dual.group1.yzxy * float4(this.group1.zxy, this.e315)) + float4((right_anti_dual.group8.xyz * this.group2.www), this.e235 * right_anti_dual.e1) + float4((this.group3.zxy * right_anti_dual.group6.yzx), this.e125 * right_anti_dual.e3) - (this.group3.yzxw * right_anti_dual.group6.zxyw) - (this.group3.wwwx * float4(right_anti_dual.group7.xyz, right_anti_dual.e235)) - float4((this.group1.yzx * right_anti_dual.group1.zxy), this.e2 * right_anti_dual.e315) - float4((right_anti_dual.group1.www * this.group2.xyz), this.e3 * right_anti_dual.e125), 
            /* e1234 */
            (this.e4 * right_anti_dual.e321) + (this.e1 * right_anti_dual.e423) + (this.e2 * right_anti_dual.e431) + (this.e3 * right_anti_dual.e412) - (this.e423 * right_anti_dual.e1) - (this.e431 * right_anti_dual.e2) - (this.e412 * right_anti_dual.e3) - (this.e321 * right_anti_dual.e4)
        );
        return MultiVector(
            /* scalar, e12345 */
            float2((other.scalar * wedge.e12345) + (other.e12345 * wedge.scalar) + (other.e1 * wedge.e4235) + (other.e2 * wedge.e4315) + (other.e3 * wedge.e4125) + (other.e4 * wedge.e3215) + (other.e5 * wedge.e1234) + (other.e4235 * wedge.e1) + (other.e4315 * wedge.e2) + (other.e4125 * wedge.e3) + (other.e3215 * wedge.e4) + (other.e1234 * wedge.e5) - (other.e15 * wedge.e423) - (other.e25 * wedge.e431) - (other.e35 * wedge.e412) - (other.e45 * wedge.e321) - (other.e41 * wedge.e235) - (other.e42 * wedge.e315) - (other.e43 * wedge.e125) - (other.e23 * wedge.e415) - (other.e31 * wedge.e425) - (other.e12 * wedge.e435) - (other.e415 * wedge.e23) - (other.e425 * wedge.e31) - (other.e435 * wedge.e12) - (other.e321 * wedge.e45) - (other.e423 * wedge.e15) - (other.e431 * wedge.e25) - (other.e412 * wedge.e35) - (other.e235 * wedge.e41) - (other.e315 * wedge.e42) - (other.e125 * wedge.e43), other.e12345 * wedge.e12345), 
            /* e1, e2, e3, e4 */
            float4((other.e415 * wedge.e321) + (other.e321 * wedge.e415) + (other.e4315 * wedge.e12) + (other.e1234 * wedge.e15), (other.e425 * wedge.e321) + (other.e321 * wedge.e425) + (other.e4125 * wedge.e23) + (other.e1234 * wedge.e25), (other.e435 * wedge.e321) + (other.e321 * wedge.e435) + (other.e4235 * wedge.e31) + (other.e1234 * wedge.e35), -(other.e45 * wedge.e1234) - (other.e415 * wedge.e423) - (other.e425 * wedge.e431) - (other.e435 * wedge.e412)) + (float4(other.e12345) * wedge.group1) + (float4(wedge.e12345) * other.group1) + float4((other.group4.xyz * wedge.group9.www), other.e4235 * wedge.e41) + float4((other.group5.xyz.yzx * wedge.group9.zxy), other.e4315 * wedge.e42) + float4((other.group7.xyz.zxy * wedge.group8.xyz.yzx), other.e4125 * wedge.e43) + float4((other.group8.xyz.yzx * wedge.group7.xyz.zxy), other.e1234 * wedge.e45) - (wedge.group9.yzxx * float4(other.group5.xyz.zxy, other.e41)) - float4((float3(wedge.e1234) * other.group3.xyz), other.e412 * wedge.e435) - float4((wedge.group4.xyz * other.group9.www), other.e423 * wedge.e415) - float4((other.group7.xyz.yzx * wedge.group8.xyz.zxy), other.e42 * wedge.e4315) - float4((other.group8.xyz.zxy * wedge.group7.xyz.yzx), other.e43 * wedge.e4125) - float4((wedge.group5.xyz.yzx * other.group9.zxy), other.e431 * wedge.e425), 
            /* e5 */
            (other.e12345 * wedge.e5) + (other.e5 * wedge.e12345) + (other.e15 * wedge.e4235) + (other.e25 * wedge.e4315) + (other.e35 * wedge.e4125) + (other.e45 * wedge.e3215) - (other.e415 * wedge.e235) - (other.e425 * wedge.e315) - (other.e435 * wedge.e125) - (other.e235 * wedge.e415) - (other.e315 * wedge.e425) - (other.e125 * wedge.e435) - (other.e4235 * wedge.e15) - (other.e4315 * wedge.e25) - (other.e4125 * wedge.e35) - (other.e3215 * wedge.e45), 
            /* e15, e25, e35, e45 */
            float4((other.e415 * wedge.e3215) + (other.e315 * wedge.e4125) + (other.e4125 * wedge.e315) + (other.e3215 * wedge.e415), (other.e425 * wedge.e3215) + (other.e125 * wedge.e4235) + (other.e4235 * wedge.e125) + (other.e3215 * wedge.e425), (other.e435 * wedge.e3215) + (other.e235 * wedge.e4315) + (other.e4315 * wedge.e235) + (other.e3215 * wedge.e435), -(other.e435 * wedge.e4125) - (other.e4235 * wedge.e415) - (other.e4315 * wedge.e425) - (other.e4125 * wedge.e435)) + (float4(other.e12345) * wedge.group3) + (float4(wedge.e12345) * other.group3) - (wedge.group9.yzxx * float4(other.group8.xyz.zxy, other.e415)) - float4((wedge.group8.xyz.zxy * other.group9.yzx), other.e425 * wedge.e4315), 
            /* e41, e42, e43 */
            (float3(other.e12345) * wedge.group4.xyz) + (float3(other.e1234) * wedge.group6.xyz) + (float3(wedge.e12345) * other.group4.xyz) + (float3(wedge.e1234) * other.group6.xyz) + (other.group7.xyz.zxy * wedge.group9.yzx) + (wedge.group7.xyz.zxy * other.group9.yzx) - (other.group7.xyz.yzx * wedge.group9.zxy) - (wedge.group7.xyz.yzx * other.group9.zxy), 
            /* e23, e31, e12 */
            (float3(other.e12345) * wedge.group5.xyz) + (float3(other.e3215) * wedge.group7.xyz) + (float3(other.e1234) * wedge.group8.xyz) + (float3(wedge.e12345) * other.group5.xyz) + (float3(wedge.e3215) * other.group7.xyz) + (float3(wedge.e1234) * other.group8.xyz) - (float3(other.e321) * wedge.group9.xyz) - (float3(wedge.e321) * other.group9.xyz), 
            /* e415, e425, e435, e321 */
            (float4(other.e12345) * wedge.group6) + (float4(wedge.e12345) * other.group6) + float4((other.group9.zxy * wedge.group9.yzx), other.e1234 * wedge.e3215) - (other.group9.yzxw * float4(wedge.group9.zxy, wedge.e1234)), 
            /* e423, e431, e412 */
            (float3(other.e12345) * wedge.group7.xyz) + (float3(other.e1234) * wedge.group9.xyz) + (float3(wedge.e12345) * other.group7.xyz) - (float3(wedge.e1234) * other.group9.xyz), 
            /* e235, e315, e125 */
            (float3(other.e12345) * wedge.group8.xyz) + (float3(wedge.e12345) * other.group8.xyz) + (float3(wedge.e3215) * other.group9.xyz) - (float3(other.e3215) * wedge.group9.xyz), 
            /* e4235, e4315, e4125, e3215 */
            (float4(other.e12345) * wedge.group9) + (float4(wedge.e12345) * other.group9), 
            /* e1234 */
            (other.e12345 * wedge.e1234) + (other.e1234 * wedge.e12345)
        );
    }
}
public extension AntiDipoleInversion {
    public AntiDipoleInversion project_orthogonally_onto(Plane other) {
        AntiPlane right_anti_dual = AntiPlane(
            /* e1, e2, e3, e5 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        DipoleInversion wedge = DipoleInversion(
            /* e41, e42, e43 */
            float3(this.e4) * right_anti_dual.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(((this.group3.yzx * right_anti_dual.group0.zxy) - (this.group3.zxy * right_anti_dual.group0.yzx)), this.e4 * right_anti_dual.e5), 
            /* e15, e25, e35, e1234 */
            float4(this.e1 * right_anti_dual.e5, this.e2 * right_anti_dual.e5, this.e3 * right_anti_dual.e5, -(this.e431 * right_anti_dual.e2) - (this.e412 * right_anti_dual.e3)) - (right_anti_dual.group0.xyzx * float4(this.group3.www, this.e423)), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e425 * right_anti_dual.e3 * -1.0, this.e435 * right_anti_dual.e1 * -1.0, this.e415 * right_anti_dual.e2 * -1.0, (this.e315 * right_anti_dual.e2) + (this.e125 * right_anti_dual.e3)) + (float4(right_anti_dual.e5) * float4(this.group0.xyz, this.e321)) + (right_anti_dual.group0.yzxx * float4(this.group1.zxy, this.e235))
        );
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(wedge.e1234) * other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            float4(((wedge.group3.yzx * other.group0.zxy) - (wedge.group3.zxy * other.group0.yzx)), wedge.e1234 * other.e3215 * -1.0), 
            /* e235, e315, e125, e4 */
            float4(wedge.e4235 * other.e3215 * -1.0, wedge.e4315 * other.e3215 * -1.0, wedge.e4125 * other.e3215 * -1.0, (wedge.e42 * other.e4315) + (wedge.e43 * other.e4125)) + (other.group0.xyzx * float4(wedge.group3.www, wedge.e41)), 
            /* e1, e2, e3, e5 */
            float4(wedge.e12 * other.e4315, wedge.e23 * other.e4125, wedge.e31 * other.e4235, -(wedge.e25 * other.e4315) - (wedge.e35 * other.e4125)) - (float4(other.e3215) * float4(wedge.group0.xyz, wedge.e45)) - (other.group0.zxyx * float4(wedge.group1.yzx, wedge.e15))
        );
    }
}
public extension AntiDipoleInversion {
    public RoundPoint project_orthogonally_onto(RoundPoint other) {
        Sphere right_anti_dual = Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234 */
            other.e4 * -1.0
        );
        AntiScalar wedge = AntiScalar(
            /* e12345 */
            (this.e4 * right_anti_dual.e3215) + (this.e1 * right_anti_dual.e4235) + (this.e2 * right_anti_dual.e4315) + (this.e3 * right_anti_dual.e4125) + (this.e5 * right_anti_dual.e1234)
        );
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(wedge.e12345) * other.group0, 
            /* e5 */
            wedge.e12345 * other.e5
        );
    }
}
public extension AntiDipoleInversion {
    public AntiDipoleInversion project_orthogonally_onto(Sphere other) {
        RoundPoint right_anti_dual = RoundPoint(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5 */
            other.e3215
        );
        DipoleInversion wedge = DipoleInversion(
            /* e41, e42, e43 */
            (float3(this.e4) * right_anti_dual.group0.xyz) - (float3(right_anti_dual.e4) * this.group3.xyz), 
            /* e23, e31, e12, e45 */
            float4((this.group3.yzx * right_anti_dual.group0.zxy), this.e4 * right_anti_dual.e5) - (this.group3.zxyw * right_anti_dual.group0.yzxw), 
            /* e15, e25, e35, e1234 */
            float4(this.e1 * right_anti_dual.e5, this.e2 * right_anti_dual.e5, this.e3 * right_anti_dual.e5, -(this.e431 * right_anti_dual.e2) - (this.e412 * right_anti_dual.e3) - (this.e321 * right_anti_dual.e4)) - (right_anti_dual.group0.xyzx * float4(this.group3.www, this.e423)), 
            /* e4235, e4315, e4125, e3215 */
            float4(-(this.e425 * right_anti_dual.e3) - (this.e235 * right_anti_dual.e4), -(this.e435 * right_anti_dual.e1) - (this.e315 * right_anti_dual.e4), -(this.e415 * right_anti_dual.e2) - (this.e125 * right_anti_dual.e4), (this.e315 * right_anti_dual.e2) + (this.e125 * right_anti_dual.e3)) + (float4(right_anti_dual.e5) * float4(this.group0.xyz, this.e321)) + (right_anti_dual.group0.yzxx * float4(this.group1.zxy, this.e235))
        );
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (float3(other.e1234) * wedge.group3.xyz) - (float3(wedge.e1234) * other.group0.xyz), 
            /* e415, e425, e435, e321 */
            (wedge.group3.yzxw * float4(other.group0.zxy, other.e1234)) - (other.group0.yzxw * float4(wedge.group3.zxy, wedge.e1234)), 
            /* e235, e315, e125, e4 */
            float4(wedge.e4235 * other.e3215 * -1.0, wedge.e4315 * other.e3215 * -1.0, wedge.e4125 * other.e3215 * -1.0, (wedge.e42 * other.e4315) + (wedge.e43 * other.e4125) + (wedge.e45 * other.e1234)) + (other.group0.xyzx * float4(wedge.group3.www, wedge.e41)), 
            /* e1, e2, e3, e5 */
            float4((wedge.e12 * other.e4315) + (wedge.e15 * other.e1234), (wedge.e23 * other.e4125) + (wedge.e25 * other.e1234), (wedge.e31 * other.e4235) + (wedge.e35 * other.e1234), -(wedge.e25 * other.e4315) - (wedge.e35 * other.e4125)) - (float4(other.e3215) * float4(wedge.group0.xyz, wedge.e45)) - (other.group0.zxyx * float4(wedge.group1.yzx, wedge.e15))
        );
    }
}
public extension AntiDipoleInversion {
    public VersorEven project_orthogonally_onto(VersorEven other) {
        VersorOdd right_anti_dual = VersorOdd(
            /* e41, e42, e43, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e1234 */
            float4(other.group2.xyz, other.e4) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.group3.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0)
        );
        VersorEven wedge = VersorEven(
            /* e423, e431, e412, e12345 */
            (this.group3.zxyy * float4(right_anti_dual.group0.yzx, right_anti_dual.e4315)) + float4(float3(0.0), (this.e3 * right_anti_dual.e4125) + (this.e5 * right_anti_dual.e1234) - (this.e431 * right_anti_dual.e25) - (this.e412 * right_anti_dual.e35) - (this.e415 * right_anti_dual.e23) - (this.e425 * right_anti_dual.e31) - (this.e435 * right_anti_dual.e12) - (this.e321 * right_anti_dual.e45) - (this.e235 * right_anti_dual.e41) - (this.e315 * right_anti_dual.e42) - (this.e125 * right_anti_dual.e43)) + float4((this.group0.xyz * right_anti_dual.group0.www), this.e4 * right_anti_dual.e3215) + float4((this.group2.www * right_anti_dual.group1.xyz), this.e1 * right_anti_dual.e4235) - float4((this.group3.yzx * right_anti_dual.group0.zxy), this.e423 * right_anti_dual.e15), 
            /* e415, e425, e435, e321 */
            float4((this.e4 * right_anti_dual.e15) + (this.e5 * right_anti_dual.e41), (this.e4 * right_anti_dual.e25) + (this.e5 * right_anti_dual.e42), (this.e4 * right_anti_dual.e35) + (this.e5 * right_anti_dual.e43), -(this.e2 * right_anti_dual.e31) - (this.e3 * right_anti_dual.e12)) + (float4(right_anti_dual.scalar) * this.group1) - (this.group3.xyzx * right_anti_dual.group1.wwwx), 
            /* e235, e315, e125, e5 */
            float4(((float3(this.e5) * right_anti_dual.group1.xyz) + (float3(right_anti_dual.scalar) * this.group2.xyz) + (this.group3.yzx * right_anti_dual.group2.zxy) - (this.group3.zxy * right_anti_dual.group2.yzx)), this.e5 * right_anti_dual.scalar), 
            /* e1, e2, e3, e4 */
            float4(right_anti_dual.scalar) * float4(this.group3.xyz, this.e4)
        );
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(((float3(other.e12345) * wedge.group0.xyz) + (float3(wedge.e12345) * other.group0.xyz)), other.e12345 * wedge.e12345), 
            /* e415, e425, e435, e321 */
            (float4(other.e12345) * wedge.group1) + (float4(wedge.e12345) * other.group1), 
            /* e235, e315, e125, e5 */
            (float4(other.e12345) * wedge.group2) + (float4(wedge.e12345) * other.group2) + float4(float3(0.0), -(other.e415 * wedge.e235) - (other.e425 * wedge.e315) - (other.e435 * wedge.e125) - (other.e235 * wedge.e415) - (other.e315 * wedge.e425) - (other.e125 * wedge.e435)), 
            /* e1, e2, e3, e4 */
            float4((other.e415 * wedge.e321) + (other.e321 * wedge.e415) + (other.e315 * wedge.e412) + (other.e1 * wedge.e12345), (other.e425 * wedge.e321) + (other.e321 * wedge.e425) + (other.e125 * wedge.e423) + (other.e2 * wedge.e12345), (other.e435 * wedge.e321) + (other.e321 * wedge.e435) + (other.e235 * wedge.e431) + (other.e3 * wedge.e12345), -(other.e412 * wedge.e435) - (other.e415 * wedge.e423) - (other.e425 * wedge.e431) - (other.e435 * wedge.e412)) + (other.group0.zxyw * float4(wedge.group2.yzx, wedge.e4)) + float4((other.group0.www * wedge.group3.xyz), other.e4 * wedge.e12345) - (other.group0.yzxx * float4(wedge.group2.zxy, wedge.e415)) - float4((other.group2.zxy * wedge.group0.yzx), other.e431 * wedge.e425)
        );
    }
}
public extension AntiDipoleInversion {
    public AntiDipoleInversion project_orthogonally_onto(VersorOdd other) {
        VersorEven right_anti_dual = VersorEven(
            /* e423, e431, e412, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            float4(other.group2.xyz, other.e3215) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4(other.group3.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0)
        );
        DipoleInversion wedge = DipoleInversion(
            /* e41, e42, e43 */
            (float3(this.e4) * right_anti_dual.group3.xyz) - (float3(right_anti_dual.e4) * this.group3.xyz), 
            /* e23, e31, e12, e45 */
            float4((this.group3.yzx * right_anti_dual.group3.zxy), this.e4 * right_anti_dual.e5) - (this.group3.zxyw * right_anti_dual.group3.yzxw), 
            /* e15, e25, e35, e1234 */
            float4(float3(0.0), (this.e1 * right_anti_dual.e423) + (this.e2 * right_anti_dual.e431) + (this.e3 * right_anti_dual.e412) - (this.e431 * right_anti_dual.e2) - (this.e412 * right_anti_dual.e3) - (this.e321 * right_anti_dual.e4)) + float4((right_anti_dual.group2.www * this.group3.xyz), this.e4 * right_anti_dual.e321) - (right_anti_dual.group3.xyzx * float4(this.group3.www, this.e423)), 
            /* e4235, e4315, e4125, e3215 */
            (float4(right_anti_dual.e5) * float4(this.group0.xyz, this.e321)) + (this.group2.wwwy * float4(right_anti_dual.group2.xyz, right_anti_dual.e2)) + (right_anti_dual.group3.yzxx * float4(this.group1.zxy, this.e235)) + float4((this.group3.zxy * right_anti_dual.group1.yzx), this.e125 * right_anti_dual.e3) - (float4(this.e5) * float4(right_anti_dual.group0.xyz, right_anti_dual.e321)) - (this.group3.yzxz * float4(right_anti_dual.group1.zxy, right_anti_dual.e125)) - float4((this.group1.yzx * right_anti_dual.group3.zxy), this.e1 * right_anti_dual.e235) - float4((right_anti_dual.group3.www * this.group2.xyz), this.e2 * right_anti_dual.e315)
        );
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (float3(other.e1234) * wedge.group3.xyz) - (float3(wedge.e1234) * other.group3.xyz), 
            /* e415, e425, e435, e321 */
            (wedge.group3.yzxw * float4(other.group3.zxy, other.e1234)) - (other.group3.yzxw * float4(wedge.group3.zxy, wedge.e1234)), 
            /* e235, e315, e125, e4 */
            (other.group3.xyzx * float4(wedge.group3.www, wedge.e41)) + float4(float3(0.0), (wedge.e42 * other.e4315) + (wedge.e43 * other.e4125) + (wedge.e45 * other.e1234) - (wedge.e4235 * other.e41) - (wedge.e4315 * other.e42) - (wedge.e4125 * other.e43)) - float4((other.group3.www * wedge.group3.xyz), wedge.e1234 * other.e45), 
            /* e1, e2, e3, e5 */
            (float4(wedge.e3215) * float4(other.group0.xyz, other.e45)) + (wedge.group3.zxyz * float4(other.group1.yzx, other.e35)) + (other.group2.wwwy * float4(wedge.group2.xyz, wedge.e4315)) + float4((wedge.group1.zxy * other.group3.yzx), wedge.e4235 * other.e15) - (float4(other.e3215) * float4(wedge.group0.xyz, wedge.e45)) - (wedge.group2.wwwy * float4(other.group2.xyz, other.e4315)) - (other.group3.zxyx * float4(wedge.group1.yzx, wedge.e15)) - float4((wedge.group3.yzx * other.group1.zxy), wedge.e35 * other.e4125)
        );
    }
}
public extension AntiDualNum {
    // Fancy infix trick (first half)
    public ProjectOrthogonallyOntoInfixPartial<AntiDualNum> operator /(ProjectOrthogonallyOntoInfix rhs) {
        return ProjectOrthogonallyOntoInfixPartial<AntiDualNum>(this);
    }
}
public extension ProjectOrthogonallyOntoInfixPartial<AntiDualNum> {
    // Fancy infix trick (second half)
    public AntiCircleRotor operator /(AntiCircleRotor rhs) {
        return this.a.project_orthogonally_onto(rhs);
    }
}
public extension AntiDualNum {
    public AntiCircleRotor project_orthogonally_onto(AntiCircleRotor other) {
        CircleRotor right_anti_dual = CircleRotor(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        CircleRotor wedge = CircleRotor(
            /* e423, e431, e412 */
            float3(this.scalar) * right_anti_dual.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(this.scalar) * right_anti_dual.group1, 
            /* e235, e315, e125, e12345 */
            float4(this.scalar) * right_anti_dual.group2
        );
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(wedge.e12345) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(wedge.e12345) * other.group1, 
            /* e15, e25, e35, scalar */
            float4((float3(wedge.e12345) * other.group2.xyz), (other.scalar * wedge.e12345) - (other.e41 * wedge.e235) - (other.e42 * wedge.e315) - (other.e43 * wedge.e125) - (other.e23 * wedge.e415) - (other.e31 * wedge.e425) - (other.e12 * wedge.e435) - (other.e45 * wedge.e321) - (other.e15 * wedge.e423) - (other.e25 * wedge.e431) - (other.e35 * wedge.e412))
        );
    }
}
public extension AntiDualNum {
    public AntiCircleRotor project_orthogonally_onto(AntiDipoleInversion other) {
        DipoleInversion right_anti_dual = DipoleInversion(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e1234 */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group3 * float4(1.0, 1.0, 1.0, -1.0)
        );
        DipoleInversion wedge = DipoleInversion(
            /* e41, e42, e43 */
            float3(this.scalar) * right_anti_dual.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(this.scalar) * right_anti_dual.group1, 
            /* e15, e25, e35, e1234 */
            float4(this.scalar) * right_anti_dual.group2, 
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar) * right_anti_dual.group3
        );
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (float3(wedge.e1234) * other.group1.xyz) + (other.group0.xyz.zxy * wedge.group3.yzx) - (other.group0.xyz.yzx * wedge.group3.zxy), 
            /* e23, e31, e12, e45 */
            float4((other.e423 * wedge.e3215) + (other.e235 * wedge.e1234), (other.e431 * wedge.e3215) + (other.e315 * wedge.e1234), (other.e412 * wedge.e3215) + (other.e125 * wedge.e1234), -(other.e425 * wedge.e4315) - (other.e435 * wedge.e4125)) - (other.group1.wwwx * wedge.group3.xyzx), 
            /* e15, e25, e35, scalar */
            (float4(wedge.e3215) * float4(other.group1.xyz, other.e4)) + (wedge.group3.zxyx * float4(other.group2.yzx, other.e1)) + float4(float3(0.0), (other.e2 * wedge.e4315) + (other.e3 * wedge.e4125) + (other.e5 * wedge.e1234) - (other.e431 * wedge.e25) - (other.e412 * wedge.e35) - (other.e415 * wedge.e23) - (other.e425 * wedge.e31) - (other.e435 * wedge.e12) - (other.e321 * wedge.e45) - (other.e235 * wedge.e41) - (other.e315 * wedge.e42) - (other.e125 * wedge.e43)) - float4((other.group2.zxy * wedge.group3.yzx), other.e423 * wedge.e15)
        );
    }
}
public extension AntiDualNum {
    public AntiDualNum project_orthogonally_onto(AntiDualNum other) {
        return AntiDualNum(
            /* e3215, scalar */
            float2(other.scalar * this.scalar) * other.group0.xy
        );
    }
}
public extension AntiDualNum {
    public Scalar project_orthogonally_onto(AntiFlatPoint other) {
        return Scalar(
            /* scalar */
            pow(other.e321, 2) * this.scalar
        );
    }
}
public extension AntiDualNum {
    public AntiMotor project_orthogonally_onto(AntiFlector other) {
        Flector right_anti_dual = Flector(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        Flector wedge = Flector(
            /* e15, e25, e35, e45 */
            float4(this.scalar) * right_anti_dual.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar) * right_anti_dual.group1
        );
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((other.group0.www * wedge.group1.xyz * float3(-1.0)), (other.e1 * wedge.e4235) + (other.e2 * wedge.e4315) + (other.e3 * wedge.e4125) - (other.e321 * wedge.e45)), 
            /* e15, e25, e35, e3215 */
            float4(((other.group0.yzx * wedge.group1.zxy) - (other.group0.zxy * wedge.group1.yzx)), 0.0)
        );
    }
}
public extension AntiDualNum {
    public Scalar project_orthogonally_onto(AntiLine other) {
        Line right_anti_dual = Line(
            /* e415, e425, e435 */
            other.group0.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            other.group1.xyz * float3(-1.0)
        );
        Line wedge = Line(
            /* e415, e425, e435 */
            float3(this.scalar) * right_anti_dual.group0.xyz, 
            /* e235, e315, e125 */
            float3(this.scalar) * right_anti_dual.group1.xyz
        );
        return Scalar(
            /* scalar */
            -(other.e23 * wedge.e415) - (other.e31 * wedge.e425) - (other.e12 * wedge.e435)
        );
    }
}
public extension AntiDualNum {
    public AntiMotor project_orthogonally_onto(AntiMotor other) {
        Motor right_anti_dual = Motor(
            /* e415, e425, e435, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        Motor wedge = Motor(
            /* e415, e425, e435, e12345 */
            float4(this.scalar) * right_anti_dual.group0, 
            /* e235, e315, e125, e5 */
            float4(this.scalar) * right_anti_dual.group1
        );
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((float3(wedge.e12345) * other.group0.xyz), (other.scalar * wedge.e12345) - (other.e23 * wedge.e415) - (other.e31 * wedge.e425) - (other.e12 * wedge.e435)), 
            /* e15, e25, e35, e3215 */
            float4(((float3(other.e3215) * wedge.group0.xyz) + (float3(wedge.e12345) * other.group1.xyz)), other.e3215 * wedge.e12345)
        );
    }
}
public extension AntiDualNum {
    public Scalar project_orthogonally_onto(AntiPlane other) {
        Plane wedge = Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar) * float4(other.group0.xyz, other.e5 * -1.0)
        );
        return Scalar(
            /* scalar */
            (other.e1 * wedge.e4235) + (other.e2 * wedge.e4315) + (other.e3 * wedge.e4125)
        );
    }
}
public extension AntiDualNum {
    public AntiDualNum project_orthogonally_onto(AntiScalar other) {
        Scalar right_anti_dual = Scalar(
            /* scalar */
            other.e12345 * -1.0
        );
        return AntiDualNum(
            /* e3215, scalar */
            float2(other.e12345) * float2(this.e3215 * right_anti_dual.scalar, this.scalar * right_anti_dual.scalar)
        );
    }
}
public extension AntiDualNum {
    public Scalar project_orthogonally_onto(Circle other) {
        Dipole right_anti_dual = Dipole(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35 */
            other.group2.xyz
        );
        Dipole wedge = Dipole(
            /* e41, e42, e43 */
            float3(this.scalar) * right_anti_dual.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(this.scalar) * right_anti_dual.group1, 
            /* e15, e25, e35 */
            float3(this.scalar) * right_anti_dual.group2.xyz
        );
        return Scalar(
            /* scalar */
            -(other.e423 * wedge.e15) - (other.e431 * wedge.e25) - (other.e412 * wedge.e35) - (other.e415 * wedge.e23) - (other.e425 * wedge.e31) - (other.e435 * wedge.e12) - (other.e321 * wedge.e45) - (other.e235 * wedge.e41) - (other.e315 * wedge.e42) - (other.e125 * wedge.e43)
        );
    }
}
public extension AntiDualNum {
    public VersorOdd project_orthogonally_onto(CircleRotor other) {
        AntiCircleRotor right_anti_dual = AntiCircleRotor(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        VersorOdd wedge = VersorOdd(
            /* e41, e42, e43, scalar */
            float4(this.scalar) * float4(right_anti_dual.group0.xyz, right_anti_dual.scalar), 
            /* e23, e31, e12, e45 */
            float4(this.scalar) * right_anti_dual.group1, 
            /* e15, e25, e35, e1234 */
            float4(this.group0.xy.yy, this.scalar, 0.0) * float4(float3(1.0), 0.0) * float4(right_anti_dual.group2.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), right_anti_dual.scalar * this.e3215)
        );
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4((other.e415 * wedge.e1234) + (other.e12345 * wedge.e41), (other.e425 * wedge.e1234) + (other.e12345 * wedge.e42), (other.e435 * wedge.e1234) + (other.e12345 * wedge.e43), -(other.e431 * wedge.e25) - (other.e412 * wedge.e35) - (other.e415 * wedge.e23) - (other.e425 * wedge.e31) - (other.e435 * wedge.e12) - (other.e321 * wedge.e45) - (other.e235 * wedge.e41) - (other.e315 * wedge.e42) - (other.e125 * wedge.e43)) + float4((other.group0.xyz.zxy * wedge.group3.yzx), other.e12345 * wedge.scalar) - float4((other.group0.xyz.yzx * wedge.group3.zxy), other.e423 * wedge.e15), 
            /* e23, e31, e12, e45 */
            float4((other.e235 * wedge.e1234) + (other.e12345 * wedge.e23), (other.e315 * wedge.e1234) + (other.e12345 * wedge.e31), (other.e125 * wedge.e1234) + (other.e12345 * wedge.e12), -(other.e425 * wedge.e4315) - (other.e435 * wedge.e4125)) + float4((other.group0.xyz * wedge.group3.www), other.e12345 * wedge.e45) - (other.group1.wwwx * wedge.group3.xyzx), 
            /* e15, e25, e35, e1234 */
            float4((float3(other.e12345) * wedge.group2.xyz), 0.0) + float4((float3(wedge.e3215) * other.group1.xyz), 0.0) + float4((other.group2.yzx * wedge.group3.zxy), 0.0) - float4((other.group2.zxy * wedge.group3.yzx), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345) * wedge.group3
        );
    }
}
public extension AntiDualNum {
    public Scalar project_orthogonally_onto(Dipole other) {
        Circle right_anti_dual = Circle(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125 */
            other.group2.xyz * float3(-1.0)
        );
        Circle wedge = Circle(
            /* e423, e431, e412 */
            float3(this.scalar) * right_anti_dual.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(this.scalar) * right_anti_dual.group1, 
            /* e235, e315, e125 */
            float3(this.scalar) * right_anti_dual.group2.xyz
        );
        return Scalar(
            /* scalar */
            -(wedge.e423 * other.e15) - (wedge.e431 * other.e25) - (wedge.e412 * other.e35) - (wedge.e415 * other.e23) - (wedge.e425 * other.e31) - (wedge.e435 * other.e12) - (wedge.e321 * other.e45) - (wedge.e235 * other.e41) - (wedge.e315 * other.e42) - (wedge.e125 * other.e43)
        );
    }
}
public extension AntiDualNum {
    public VersorOdd project_orthogonally_onto(DipoleInversion other) {
        AntiDipoleInversion right_anti_dual = AntiDipoleInversion(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e4 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group3 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        VersorEven wedge = VersorEven(
            /* e423, e431, e412, e12345 */
            float4(this.group0.xy.yy, this.scalar, this.e3215) * float4(right_anti_dual.group0.xyz, right_anti_dual.e4), 
            /* e415, e425, e435, e321 */
            float4(this.scalar) * right_anti_dual.group1, 
            /* e235, e315, e125, e5 */
            float4(this.scalar) * float4(right_anti_dual.group2.xyz, right_anti_dual.e5), 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * float4(right_anti_dual.group3.xyz, right_anti_dual.e4)
        );
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (other.group3.yzxy * float4(wedge.group0.zxy, wedge.e2)) + float4(float3(0.0), (other.e4125 * wedge.e3) + (other.e3215 * wedge.e4) - (other.e42 * wedge.e315) - (other.e43 * wedge.e125) - (other.e23 * wedge.e415) - (other.e31 * wedge.e425) - (other.e12 * wedge.e435) - (other.e45 * wedge.e321) - (other.e15 * wedge.e423) - (other.e25 * wedge.e431) - (other.e35 * wedge.e412)) + float4((other.group0.xyz * wedge.group0.www), other.e1234 * wedge.e5) + float4((other.group2.www * wedge.group1.xyz), other.e4235 * wedge.e1) - float4((other.group3.zxy * wedge.group0.yzx), other.e41 * wedge.e235), 
            /* e23, e31, e12, e45 */
            float4((other.e1234 * wedge.e235) + (other.e3215 * wedge.e423), (other.e1234 * wedge.e315) + (other.e3215 * wedge.e431), (other.e1234 * wedge.e125) + (other.e3215 * wedge.e412), -(other.e4315 * wedge.e425) - (other.e4125 * wedge.e435)) + (float4(wedge.e12345) * other.group1) - (other.group3.xyzx * wedge.group1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(other.e3215) * wedge.group1.xyz) + (float3(wedge.e12345) * other.group2.xyz) + (other.group3.zxy * wedge.group2.yzx) - (other.group3.yzx * wedge.group2.zxy)), other.e1234 * wedge.e12345), 
            /* e4235, e4315, e4125, e3215 */
            float4(wedge.e12345) * other.group3
        );
    }
}
public extension AntiDualNum {
    public AntiDualNum project_orthogonally_onto(DualNum other) {
        AntiDualNum right_anti_dual = AntiDualNum(
            /* e3215, scalar */
            other.group0.xy * float2(-1.0)
        );
        return AntiDualNum(
            /* e3215, scalar */
            float2(other.e12345) * float2((right_anti_dual.e3215 * this.scalar) + (right_anti_dual.scalar * this.e3215), right_anti_dual.scalar * this.scalar)
        );
    }
}
public extension AntiDualNum {
    public Scalar project_orthogonally_onto(FlatPoint other) {
        return Scalar(
            /* scalar */
            pow(other.e45, 2) * this.scalar * -1.0
        );
    }
}
public extension AntiDualNum {
    public AntiMotor project_orthogonally_onto(Flector other) {
        AntiFlector right_anti_dual = AntiFlector(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        AntiFlector wedge = AntiFlector(
            /* e235, e315, e125, e321 */
            float4(this.scalar) * right_anti_dual.group0, 
            /* e1, e2, e3, e5 */
            float4(this.scalar) * right_anti_dual.group1
        );
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((wedge.group0.www * other.group1.xyz * float3(-1.0)), (wedge.e1 * other.e4235) + (wedge.e2 * other.e4315) + (wedge.e3 * other.e4125) - (wedge.e321 * other.e45)), 
            /* e15, e25, e35, e3215 */
            float4(((wedge.group0.yzx * other.group1.zxy) - (wedge.group0.zxy * other.group1.yzx)), 0.0)
        );
    }
}
public extension AntiDualNum {
    public Scalar project_orthogonally_onto(Line other) {
        AntiLine right_anti_dual = AntiLine(
            /* e23, e31, e12 */
            other.group0.xyz, 
            /* e15, e25, e35 */
            other.group1.xyz
        );
        AntiLine wedge = AntiLine(
            /* e23, e31, e12 */
            float3(this.scalar) * right_anti_dual.group0.xyz, 
            /* e15, e25, e35 */
            float3(this.scalar) * right_anti_dual.group1.xyz
        );
        return Scalar(
            /* scalar */
            -(wedge.e23 * other.e415) - (wedge.e31 * other.e425) - (wedge.e12 * other.e435)
        );
    }
}
public extension AntiDualNum {
    public AntiMotor project_orthogonally_onto(Motor other) {
        AntiMotor right_anti_dual = AntiMotor(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        AntiMotor wedge = AntiMotor(
            /* e23, e31, e12, scalar */
            float4(this.scalar) * right_anti_dual.group0, 
            /* e15, e25, e35, e3215 */
            float4(this.group0.xy.yy, this.scalar, (this.e3215 * right_anti_dual.scalar) + (this.scalar * right_anti_dual.e3215)) * float4(right_anti_dual.group1.xyz, 1.0)
        );
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((float3(other.e12345) * wedge.group0.xyz), (wedge.scalar * other.e12345) - (wedge.e23 * other.e415) - (wedge.e31 * other.e425) - (wedge.e12 * other.e435)), 
            /* e15, e25, e35, e3215 */
            float4(((float3(wedge.e3215) * other.group0.xyz) + (float3(other.e12345) * wedge.group1.xyz)), wedge.e3215 * other.e12345)
        );
    }
}
public extension AntiDualNum {
    public MultiVector project_orthogonally_onto(MultiVector other) {
        MultiVector right_anti_dual = MultiVector(
            /* scalar, e12345 */
            other.group0.xy.yx * float2(-1.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4(other.group9.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5 */
            other.e3215, 
            /* e15, e25, e35, e45 */
            float4(other.group8.xyz, other.e321 * -1.0), 
            /* e41, e42, e43 */
            other.group7.xyz, 
            /* e23, e31, e12 */
            other.group6.xyz, 
            /* e415, e425, e435, e321 */
            float4(other.group5.xyz, other.e45) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e423, e431, e412 */
            other.group4.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            other.group3.xyz * float3(-1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.group1.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234 */
            other.e4 * -1.0
        );
        MultiVector wedge = MultiVector(
            /* scalar, e12345 */
            float2(this.scalar * right_anti_dual.scalar, (this.e3215 * right_anti_dual.e4) + (this.scalar * right_anti_dual.e12345)), 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * right_anti_dual.group1, 
            /* e5 */
            this.scalar * right_anti_dual.e5, 
            /* e15, e25, e35, e45 */
            float4(this.scalar) * right_anti_dual.group3, 
            /* e41, e42, e43 */
            float3(this.scalar) * right_anti_dual.group4.xyz, 
            /* e23, e31, e12 */
            float3(this.scalar) * right_anti_dual.group5.xyz, 
            /* e415, e425, e435, e321 */
            float4(this.scalar) * right_anti_dual.group6, 
            /* e423, e431, e412 */
            float3(this.scalar) * right_anti_dual.group7.xyz, 
            /* e235, e315, e125 */
            float3(this.scalar) * right_anti_dual.group8.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(this.group0.xy.yy, this.scalar, (this.e3215 * right_anti_dual.scalar) + (this.scalar * right_anti_dual.e3215)) * float4(right_anti_dual.group9.xyz, 1.0), 
            /* e1234 */
            this.scalar * right_anti_dual.e1234
        );
        return MultiVector(
            /* scalar, e12345 */
            float2((other.scalar * wedge.e12345) + (other.e12345 * wedge.scalar) + (other.e1 * wedge.e4235) + (other.e2 * wedge.e4315) + (other.e3 * wedge.e4125) + (other.e4 * wedge.e3215) + (other.e5 * wedge.e1234) + (other.e4235 * wedge.e1) + (other.e4315 * wedge.e2) + (other.e4125 * wedge.e3) + (other.e3215 * wedge.e4) + (other.e1234 * wedge.e5) - (other.e15 * wedge.e423) - (other.e25 * wedge.e431) - (other.e35 * wedge.e412) - (other.e45 * wedge.e321) - (other.e41 * wedge.e235) - (other.e42 * wedge.e315) - (other.e43 * wedge.e125) - (other.e23 * wedge.e415) - (other.e31 * wedge.e425) - (other.e12 * wedge.e435) - (other.e415 * wedge.e23) - (other.e425 * wedge.e31) - (other.e435 * wedge.e12) - (other.e321 * wedge.e45) - (other.e423 * wedge.e15) - (other.e431 * wedge.e25) - (other.e412 * wedge.e35) - (other.e235 * wedge.e41) - (other.e315 * wedge.e42) - (other.e125 * wedge.e43), other.e12345 * wedge.e12345), 
            /* e1, e2, e3, e4 */
            float4((other.e415 * wedge.e321) + (other.e321 * wedge.e415) + (other.e4315 * wedge.e12) + (other.e1234 * wedge.e15), (other.e425 * wedge.e321) + (other.e321 * wedge.e425) + (other.e4125 * wedge.e23) + (other.e1234 * wedge.e25), (other.e435 * wedge.e321) + (other.e321 * wedge.e435) + (other.e4235 * wedge.e31) + (other.e1234 * wedge.e35), -(other.e45 * wedge.e1234) - (other.e415 * wedge.e423) - (other.e425 * wedge.e431) - (other.e435 * wedge.e412)) + (float4(other.e12345) * wedge.group1) + (float4(wedge.e12345) * other.group1) + float4((other.group4.xyz * wedge.group9.www), other.e4235 * wedge.e41) + float4((other.group5.xyz.yzx * wedge.group9.zxy), other.e4315 * wedge.e42) + float4((other.group7.xyz.zxy * wedge.group8.xyz.yzx), other.e4125 * wedge.e43) + float4((other.group8.xyz.yzx * wedge.group7.xyz.zxy), other.e1234 * wedge.e45) - (wedge.group9.yzxx * float4(other.group5.xyz.zxy, other.e41)) - float4((float3(wedge.e1234) * other.group3.xyz), other.e412 * wedge.e435) - float4((wedge.group4.xyz * other.group9.www), other.e423 * wedge.e415) - float4((other.group7.xyz.yzx * wedge.group8.xyz.zxy), other.e42 * wedge.e4315) - float4((other.group8.xyz.zxy * wedge.group7.xyz.yzx), other.e43 * wedge.e4125) - float4((wedge.group5.xyz.yzx * other.group9.zxy), other.e431 * wedge.e425), 
            /* e5 */
            (other.e12345 * wedge.e5) + (other.e5 * wedge.e12345) + (other.e15 * wedge.e4235) + (other.e25 * wedge.e4315) + (other.e35 * wedge.e4125) + (other.e45 * wedge.e3215) - (other.e415 * wedge.e235) - (other.e425 * wedge.e315) - (other.e435 * wedge.e125) - (other.e235 * wedge.e415) - (other.e315 * wedge.e425) - (other.e125 * wedge.e435) - (other.e4235 * wedge.e15) - (other.e4315 * wedge.e25) - (other.e4125 * wedge.e35) - (other.e3215 * wedge.e45), 
            /* e15, e25, e35, e45 */
            float4((other.e415 * wedge.e3215) + (other.e315 * wedge.e4125) + (other.e4125 * wedge.e315) + (other.e3215 * wedge.e415), (other.e425 * wedge.e3215) + (other.e125 * wedge.e4235) + (other.e4235 * wedge.e125) + (other.e3215 * wedge.e425), (other.e435 * wedge.e3215) + (other.e235 * wedge.e4315) + (other.e4315 * wedge.e235) + (other.e3215 * wedge.e435), -(other.e435 * wedge.e4125) - (other.e4235 * wedge.e415) - (other.e4315 * wedge.e425) - (other.e4125 * wedge.e435)) + (float4(other.e12345) * wedge.group3) + (float4(wedge.e12345) * other.group3) - (wedge.group9.yzxx * float4(other.group8.xyz.zxy, other.e415)) - float4((wedge.group8.xyz.zxy * other.group9.yzx), other.e425 * wedge.e4315), 
            /* e41, e42, e43 */
            (float3(other.e12345) * wedge.group4.xyz) + (float3(other.e1234) * wedge.group6.xyz) + (float3(wedge.e12345) * other.group4.xyz) + (float3(wedge.e1234) * other.group6.xyz) + (other.group7.xyz.zxy * wedge.group9.yzx) + (wedge.group7.xyz.zxy * other.group9.yzx) - (other.group7.xyz.yzx * wedge.group9.zxy) - (wedge.group7.xyz.yzx * other.group9.zxy), 
            /* e23, e31, e12 */
            (float3(other.e12345) * wedge.group5.xyz) + (float3(other.e3215) * wedge.group7.xyz) + (float3(other.e1234) * wedge.group8.xyz) + (float3(wedge.e12345) * other.group5.xyz) + (float3(wedge.e3215) * other.group7.xyz) + (float3(wedge.e1234) * other.group8.xyz) - (float3(other.e321) * wedge.group9.xyz) - (float3(wedge.e321) * other.group9.xyz), 
            /* e415, e425, e435, e321 */
            (float4(other.e12345) * wedge.group6) + (float4(wedge.e12345) * other.group6) + float4((other.group9.zxy * wedge.group9.yzx), other.e1234 * wedge.e3215) - (other.group9.yzxw * float4(wedge.group9.zxy, wedge.e1234)), 
            /* e423, e431, e412 */
            (float3(other.e12345) * wedge.group7.xyz) + (float3(other.e1234) * wedge.group9.xyz) + (float3(wedge.e12345) * other.group7.xyz) - (float3(wedge.e1234) * other.group9.xyz), 
            /* e235, e315, e125 */
            (float3(other.e12345) * wedge.group8.xyz) + (float3(wedge.e12345) * other.group8.xyz) + (float3(wedge.e3215) * other.group9.xyz) - (float3(other.e3215) * wedge.group9.xyz), 
            /* e4235, e4315, e4125, e3215 */
            (float4(other.e12345) * wedge.group9) + (float4(wedge.e12345) * other.group9), 
            /* e1234 */
            (other.e12345 * wedge.e1234) + (other.e1234 * wedge.e12345)
        );
    }
}
public extension AntiDualNum {
    public Scalar project_orthogonally_onto(Plane other) {
        AntiPlane wedge = AntiPlane(
            /* e1, e2, e3, e5 */
            float4(this.scalar) * float4(other.e4235 * -1.0, other.e4315 * -1.0, other.e4125 * -1.0, other.e3215)
        );
        return Scalar(
            /* scalar */
            (wedge.e1 * other.e4235) + (wedge.e2 * other.e4315) + (wedge.e3 * other.e4125)
        );
    }
}
public extension AntiDualNum {
    public Scalar project_orthogonally_onto(RoundPoint other) {
        Sphere right_anti_dual = Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234 */
            other.e4 * -1.0
        );
        Sphere wedge = Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar) * right_anti_dual.group0, 
            /* e1234 */
            this.scalar * right_anti_dual.e1234
        );
        return Scalar(
            /* scalar */
            (other.e1 * wedge.e4235) + (other.e2 * wedge.e4315) + (other.e3 * wedge.e4125) + (other.e4 * wedge.e3215) + (other.e5 * wedge.e1234)
        );
    }
}
public extension AntiDualNum {
    public Scalar project_orthogonally_onto(Scalar other) {
        return Scalar(
            /* scalar */
            pow(other.scalar, 2) * this.scalar
        );
    }
}
public extension AntiDualNum {
    public VersorOdd project_orthogonally_onto(Sphere other) {
        RoundPoint right_anti_dual = RoundPoint(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5 */
            other.e3215
        );
        VersorEven wedge = VersorEven(
            /* e423, e431, e412, e12345 */
            float4(float3(0.0), this.e3215 * right_anti_dual.e4), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e235, e315, e125, e5 */
            float4(float3(0.0), this.scalar * right_anti_dual.e5), 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * right_anti_dual.group0
        );
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(other.e4125 * wedge.e431 * -1.0, other.e4235 * wedge.e412 * -1.0, other.e4315 * wedge.e423 * -1.0, (other.e4125 * wedge.e3) + (other.e3215 * wedge.e4) + (other.e1234 * wedge.e5)) + (other.group0.yzxx * float4(wedge.group0.zxy, wedge.e1)) + float4((float3(other.e1234) * wedge.group1.xyz), other.e4315 * wedge.e2), 
            /* e23, e31, e12, e45 */
            float4((float3(other.e3215) * wedge.group0.xyz), 0.0) + float4((float3(other.e1234) * wedge.group2.xyz), 0.0) - float4((float3(wedge.e321) * other.group0.xyz), 0.0), 
            /* e15, e25, e35, e1234 */
            float4(((float3(other.e3215) * wedge.group1.xyz) + (other.group0.zxy * wedge.group2.yzx) - (other.group0.yzx * wedge.group2.zxy)), other.e1234 * wedge.e12345), 
            /* e4235, e4315, e4125, e3215 */
            float4(wedge.e12345) * other.group0
        );
    }
}
public extension AntiDualNum {
    public VersorOdd project_orthogonally_onto(VersorEven other) {
        VersorOdd right_anti_dual = VersorOdd(
            /* e41, e42, e43, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e1234 */
            float4(other.group2.xyz, other.e4) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.group3.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0)
        );
        VersorOdd wedge = VersorOdd(
            /* e41, e42, e43, scalar */
            float4(this.scalar) * right_anti_dual.group0, 
            /* e23, e31, e12, e45 */
            float4(this.scalar) * right_anti_dual.group1, 
            /* e15, e25, e35, e1234 */
            float4(this.scalar) * right_anti_dual.group2, 
            /* e4235, e4315, e4125, e3215 */
            float4(this.group0.xy.yy, this.scalar, (this.e3215 * right_anti_dual.scalar) + (this.scalar * right_anti_dual.e3215)) * float4(right_anti_dual.group3.xyz, 1.0)
        );
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (other.group0.zxyw * float4(wedge.group3.yzx, wedge.scalar)) + float4(float3(0.0), (other.e2 * wedge.e4315) + (other.e3 * wedge.e4125) + (other.e4 * wedge.e3215) - (other.e431 * wedge.e25) - (other.e412 * wedge.e35) - (other.e415 * wedge.e23) - (other.e425 * wedge.e31) - (other.e435 * wedge.e12) - (other.e321 * wedge.e45) - (other.e235 * wedge.e41) - (other.e315 * wedge.e42) - (other.e125 * wedge.e43)) + float4((other.group0.www * wedge.group0.xyz), other.e5 * wedge.e1234) + float4((wedge.group2.www * other.group1.xyz), other.e1 * wedge.e4235) - (other.group0.yzxx * float4(wedge.group3.zxy, wedge.e15)), 
            /* e23, e31, e12, e45 */
            float4((other.e12345 * wedge.e23) + (other.e235 * wedge.e1234), (other.e12345 * wedge.e31) + (other.e315 * wedge.e1234), (other.e12345 * wedge.e12) + (other.e125 * wedge.e1234), -(other.e425 * wedge.e4315) - (other.e435 * wedge.e4125)) + (other.group0 * float4(wedge.group3.www, wedge.e45)) - (other.group1.wwwx * wedge.group3.xyzx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(other.e12345) * wedge.group2.xyz) + (float3(wedge.e3215) * other.group1.xyz) + (other.group2.yzx * wedge.group3.zxy) - (other.group2.zxy * wedge.group3.yzx)), other.e12345 * wedge.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345) * wedge.group3
        );
    }
}
public extension AntiDualNum {
    public VersorOdd project_orthogonally_onto(VersorOdd other) {
        VersorEven right_anti_dual = VersorEven(
            /* e423, e431, e412, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            float4(other.group2.xyz, other.e3215) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4(other.group3.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0)
        );
        VersorEven wedge = VersorEven(
            /* e423, e431, e412, e12345 */
            float4(this.group0.xy.yy, this.scalar, (this.e3215 * right_anti_dual.e4) + (this.scalar * right_anti_dual.e12345)) * float4(right_anti_dual.group0.xyz, 1.0), 
            /* e415, e425, e435, e321 */
            float4(this.scalar) * right_anti_dual.group1, 
            /* e235, e315, e125, e5 */
            float4(this.scalar) * right_anti_dual.group2, 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * right_anti_dual.group3
        );
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (wedge.group0.zxyw * float4(other.group3.yzx, other.scalar)) + float4(float3(0.0), (wedge.e2 * other.e4315) + (wedge.e3 * other.e4125) + (wedge.e4 * other.e3215) - (wedge.e431 * other.e25) - (wedge.e412 * other.e35) - (wedge.e415 * other.e23) - (wedge.e425 * other.e31) - (wedge.e435 * other.e12) - (wedge.e321 * other.e45) - (wedge.e235 * other.e41) - (wedge.e315 * other.e42) - (wedge.e125 * other.e43)) + float4((wedge.group0.www * other.group0.xyz), wedge.e5 * other.e1234) + float4((other.group2.www * wedge.group1.xyz), wedge.e1 * other.e4235) - (wedge.group0.yzxx * float4(other.group3.zxy, other.e15)), 
            /* e23, e31, e12, e45 */
            float4((wedge.e12345 * other.e23) + (wedge.e235 * other.e1234), (wedge.e12345 * other.e31) + (wedge.e315 * other.e1234), (wedge.e12345 * other.e12) + (wedge.e125 * other.e1234), -(wedge.e425 * other.e4315) - (wedge.e435 * other.e4125)) + (wedge.group0 * float4(other.group3.www, other.e45)) - (wedge.group1.wwwx * other.group3.xyzx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(wedge.e12345) * other.group2.xyz) + (float3(other.e3215) * wedge.group1.xyz) + (wedge.group2.yzx * other.group3.zxy) - (wedge.group2.zxy * other.group3.yzx)), wedge.e12345 * other.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(wedge.e12345) * other.group3
        );
    }
}
public extension AntiFlatPoint {
    // Fancy infix trick (first half)
    public ProjectOrthogonallyOntoInfixPartial<AntiFlatPoint> operator /(ProjectOrthogonallyOntoInfix rhs) {
        return ProjectOrthogonallyOntoInfixPartial<AntiFlatPoint>(this);
    }
}
public extension ProjectOrthogonallyOntoInfixPartial<AntiFlatPoint> {
    // Fancy infix trick (second half)
    public AntiDipoleInversion operator /(AntiDipoleInversion rhs) {
        return this.a.project_orthogonally_onto(rhs);
    }
}
public extension AntiFlatPoint {
    public AntiDipoleInversion project_orthogonally_onto(AntiDipoleInversion other) {
        DipoleInversion right_anti_dual = DipoleInversion(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e1234 */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group3 * float4(1.0, 1.0, 1.0, -1.0)
        );
        AntiScalar wedge = AntiScalar(
            /* e12345 */
            -(this.e235 * right_anti_dual.e41) - (this.e315 * right_anti_dual.e42) - (this.e125 * right_anti_dual.e43) - (this.e321 * right_anti_dual.e45)
        );
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(wedge.e12345) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(wedge.e12345) * other.group1, 
            /* e235, e315, e125, e4 */
            float4(wedge.e12345) * other.group2, 
            /* e1, e2, e3, e5 */
            float4(wedge.e12345) * other.group3
        );
    }
}
public extension AntiFlatPoint {
    public AntiFlatPoint project_orthogonally_onto(AntiFlatPoint other) {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(other.e321 * this.e321) * other.group0
        );
    }
}
public extension AntiFlatPoint {
    public AntiFlector project_orthogonally_onto(AntiFlector other) {
        AntiScalar wedge = AntiScalar(
            /* e12345 */
            (other.group0 * float4(1.0, 1.0, 1.0, -1.0))[3] * this.e321 * -1.0
        );
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(wedge.e12345) * other.group0, 
            /* e1, e2, e3, e5 */
            float4(wedge.e12345) * other.group1
        );
    }
}
public extension AntiFlatPoint {
    public AntiFlatPoint project_orthogonally_onto(AntiScalar other) {
        Scalar right_anti_dual = Scalar(
            /* scalar */
            other.e12345 * -1.0
        );
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(other.e12345) * float4(this.e235 * right_anti_dual.scalar, this.e315 * right_anti_dual.scalar, this.e125 * right_anti_dual.scalar, this.e321 * right_anti_dual.scalar)
        );
    }
}
public extension AntiFlatPoint {
    public Circle project_orthogonally_onto(Circle other) {
        Dipole right_anti_dual = Dipole(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35 */
            other.group2.xyz
        );
        AntiScalar wedge = AntiScalar(
            /* e12345 */
            -(this.e235 * right_anti_dual.e41) - (this.e315 * right_anti_dual.e42) - (this.e125 * right_anti_dual.e43) - (this.e321 * right_anti_dual.e45)
        );
        return Circle(
            /* e423, e431, e412 */
            float3(wedge.e12345) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(wedge.e12345) * other.group1, 
            /* e235, e315, e125 */
            float3(wedge.e12345) * other.group2.xyz
        );
    }
}
public extension AntiFlatPoint {
    public VersorEven project_orthogonally_onto(CircleRotor other) {
        AntiCircleRotor right_anti_dual = AntiCircleRotor(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        CircleRotor wedge = CircleRotor(
            /* e423, e431, e412 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), right_anti_dual.scalar * this.e321), 
            /* e235, e315, e125, e12345 */
            float4((right_anti_dual.group2.www * this.group0.xyz), -(right_anti_dual.e41 * this.e235) - (right_anti_dual.e42 * this.e315) - (right_anti_dual.e43 * this.e125) - (right_anti_dual.e45 * this.e321))
        );
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(((float3(other.e12345) * wedge.group0.xyz) + (float3(wedge.e12345) * other.group0.xyz)), other.e12345 * wedge.e12345), 
            /* e415, e425, e435, e321 */
            (float4(other.e12345) * wedge.group1) + (float4(wedge.e12345) * other.group1), 
            /* e235, e315, e125, e5 */
            float4((other.e235 * wedge.e12345) + (other.e12345 * wedge.e235), (other.e315 * wedge.e12345) + (other.e12345 * wedge.e315), (other.e125 * wedge.e12345) + (other.e12345 * wedge.e125), -(other.e415 * wedge.e235) - (other.e425 * wedge.e315) - (other.e435 * wedge.e125) - (other.e235 * wedge.e415) - (other.e315 * wedge.e425) - (other.e125 * wedge.e435)), 
            /* e1, e2, e3, e4 */
            float4((other.e412 * wedge.e315) + (other.e415 * wedge.e321) + (other.e321 * wedge.e415) + (other.e315 * wedge.e412), (other.e423 * wedge.e125) + (other.e425 * wedge.e321) + (other.e321 * wedge.e425) + (other.e125 * wedge.e423), (other.e431 * wedge.e235) + (other.e435 * wedge.e321) + (other.e321 * wedge.e435) + (other.e235 * wedge.e431), -(other.e412 * wedge.e435) - (other.e415 * wedge.e423) - (other.e425 * wedge.e431) - (other.e435 * wedge.e412)) - float4((other.group0.xyz.yzx * wedge.group2.zxy), other.e423 * wedge.e415) - float4((wedge.group0.xyz.yzx * other.group2.zxy), other.e431 * wedge.e425)
        );
    }
}
public extension AntiFlatPoint {
    public AntiDipoleInversion project_orthogonally_onto(DipoleInversion other) {
        AntiDipoleInversion right_anti_dual = AntiDipoleInversion(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e4 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group3 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        Sphere wedge = Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual.group2.www * this.group0.xyz * float3(-1.0)), (right_anti_dual.e1 * this.e235) + (right_anti_dual.e2 * this.e315) + (right_anti_dual.e3 * this.e125) + (right_anti_dual.e5 * this.e321)), 
            /* e1234 */
            right_anti_dual.e4 * this.e321 * -1.0
        );
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (float3(other.e1234) * wedge.group0.xyz) - (float3(wedge.e1234) * other.group3.xyz), 
            /* e415, e425, e435, e321 */
            (wedge.group0.yzxw * float4(other.group3.zxy, other.e1234)) - (other.group3.yzxw * float4(wedge.group0.zxy, wedge.e1234)), 
            /* e235, e315, e125, e4 */
            float4(other.e4235 * wedge.e3215, other.e4315 * wedge.e3215, other.e4125 * wedge.e3215, -(other.e42 * wedge.e4315) - (other.e43 * wedge.e4125) - (other.e45 * wedge.e1234)) - (wedge.group0.xyzx * float4(other.group3.www, other.e41)), 
            /* e1, e2, e3, e5 */
            float4(-(other.e12 * wedge.e4315) - (other.e15 * wedge.e1234), -(other.e23 * wedge.e4125) - (other.e25 * wedge.e1234), -(other.e31 * wedge.e4235) - (other.e35 * wedge.e1234), (other.e25 * wedge.e4315) + (other.e35 * wedge.e4125)) + (float4(wedge.e3215) * float4(other.group0.xyz, other.e45)) + (wedge.group0.zxyx * float4(other.group1.yzx, other.e15))
        );
    }
}
public extension AntiFlatPoint {
    public AntiFlatPoint project_orthogonally_onto(DualNum other) {
        AntiDualNum right_anti_dual = AntiDualNum(
            /* e3215, scalar */
            other.group0.xy * float2(-1.0)
        );
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(other.e12345) * float4(right_anti_dual.scalar * this.e235, right_anti_dual.scalar * this.e315, right_anti_dual.scalar * this.e125, right_anti_dual.scalar * this.e321)
        );
    }
}
public extension AntiFlatPoint {
    public AntiFlector project_orthogonally_onto(Flector other) {
        AntiFlector right_anti_dual = AntiFlector(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        AntiDualNum wedge = AntiDualNum(
            /* e3215, scalar */
            float2((this.e235 * right_anti_dual.e1) + (this.e315 * right_anti_dual.e2) + (this.e125 * right_anti_dual.e3) + (this.e321 * right_anti_dual.e5), 0.0)
        );
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(wedge.group0.xy.xx, wedge.e3215, 0.0) * float4(float3(1.0), 0.0) * float4(other.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), wedge.e3215 * other.e45)
        );
    }
}
public extension AntiFlatPoint {
    public AntiFlector project_orthogonally_onto(Motor other) {
        AntiFlatPoint wedge = AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(other.e12345 * -1.0) * this.group0
        );
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(other.e12345) * wedge.group0, 
            /* e1, e2, e3, e5 */
            float4((wedge.group0.www * other.group0.xyz), -(wedge.e235 * other.e415) - (wedge.e315 * other.e425) - (wedge.e125 * other.e435))
        );
    }
}
public extension AntiFlatPoint {
    public MultiVector project_orthogonally_onto(MultiVector other) {
        MultiVector right_anti_dual = MultiVector(
            /* scalar, e12345 */
            other.group0.xy.yx * float2(-1.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4(other.group9.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5 */
            other.e3215, 
            /* e15, e25, e35, e45 */
            float4(other.group8.xyz, other.e321 * -1.0), 
            /* e41, e42, e43 */
            other.group7.xyz, 
            /* e23, e31, e12 */
            other.group6.xyz, 
            /* e415, e425, e435, e321 */
            float4(other.group5.xyz, other.e45) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e423, e431, e412 */
            other.group4.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            other.group3.xyz * float3(-1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.group1.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234 */
            other.e4 * -1.0
        );
        MultiVector wedge = MultiVector(
            /* scalar, e12345 */
            float2(0.0, -(this.e235 * right_anti_dual.e41) - (this.e315 * right_anti_dual.e42) - (this.e125 * right_anti_dual.e43) - (this.e321 * right_anti_dual.e45)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), this.e321 * right_anti_dual.scalar), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(right_anti_dual.scalar) * this.group0.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4((float3(right_anti_dual.e4) * this.group0.xyz * float3(-1.0)), (this.e235 * right_anti_dual.e1) + (this.e315 * right_anti_dual.e2) + (this.e125 * right_anti_dual.e3) + (this.e321 * right_anti_dual.e5)), 
            /* e1234 */
            this.e321 * right_anti_dual.e4 * -1.0
        );
        return MultiVector(
            /* scalar, e12345 */
            float2((other.scalar * wedge.e12345) + (other.e12345 * wedge.scalar) + (other.e1 * wedge.e4235) + (other.e2 * wedge.e4315) + (other.e3 * wedge.e4125) + (other.e4 * wedge.e3215) + (other.e5 * wedge.e1234) + (other.e4235 * wedge.e1) + (other.e4315 * wedge.e2) + (other.e4125 * wedge.e3) + (other.e3215 * wedge.e4) + (other.e1234 * wedge.e5) - (other.e15 * wedge.e423) - (other.e25 * wedge.e431) - (other.e35 * wedge.e412) - (other.e45 * wedge.e321) - (other.e41 * wedge.e235) - (other.e42 * wedge.e315) - (other.e43 * wedge.e125) - (other.e23 * wedge.e415) - (other.e31 * wedge.e425) - (other.e12 * wedge.e435) - (other.e415 * wedge.e23) - (other.e425 * wedge.e31) - (other.e435 * wedge.e12) - (other.e321 * wedge.e45) - (other.e423 * wedge.e15) - (other.e431 * wedge.e25) - (other.e412 * wedge.e35) - (other.e235 * wedge.e41) - (other.e315 * wedge.e42) - (other.e125 * wedge.e43), other.e12345 * wedge.e12345), 
            /* e1, e2, e3, e4 */
            float4((other.e415 * wedge.e321) + (other.e321 * wedge.e415) + (other.e4315 * wedge.e12) + (other.e1234 * wedge.e15), (other.e425 * wedge.e321) + (other.e321 * wedge.e425) + (other.e4125 * wedge.e23) + (other.e1234 * wedge.e25), (other.e435 * wedge.e321) + (other.e321 * wedge.e435) + (other.e4235 * wedge.e31) + (other.e1234 * wedge.e35), -(other.e45 * wedge.e1234) - (other.e415 * wedge.e423) - (other.e425 * wedge.e431) - (other.e435 * wedge.e412)) + (float4(other.e12345) * wedge.group1) + (float4(wedge.e12345) * other.group1) + float4((other.group4.xyz * wedge.group9.www), other.e4235 * wedge.e41) + float4((other.group5.xyz.yzx * wedge.group9.zxy), other.e4315 * wedge.e42) + float4((other.group7.xyz.zxy * wedge.group8.xyz.yzx), other.e4125 * wedge.e43) + float4((other.group8.xyz.yzx * wedge.group7.xyz.zxy), other.e1234 * wedge.e45) - (wedge.group9.yzxx * float4(other.group5.xyz.zxy, other.e41)) - float4((float3(wedge.e1234) * other.group3.xyz), other.e412 * wedge.e435) - float4((wedge.group4.xyz * other.group9.www), other.e423 * wedge.e415) - float4((other.group7.xyz.yzx * wedge.group8.xyz.zxy), other.e42 * wedge.e4315) - float4((other.group8.xyz.zxy * wedge.group7.xyz.yzx), other.e43 * wedge.e4125) - float4((wedge.group5.xyz.yzx * other.group9.zxy), other.e431 * wedge.e425), 
            /* e5 */
            (other.e12345 * wedge.e5) + (other.e5 * wedge.e12345) + (other.e15 * wedge.e4235) + (other.e25 * wedge.e4315) + (other.e35 * wedge.e4125) + (other.e45 * wedge.e3215) - (other.e415 * wedge.e235) - (other.e425 * wedge.e315) - (other.e435 * wedge.e125) - (other.e235 * wedge.e415) - (other.e315 * wedge.e425) - (other.e125 * wedge.e435) - (other.e4235 * wedge.e15) - (other.e4315 * wedge.e25) - (other.e4125 * wedge.e35) - (other.e3215 * wedge.e45), 
            /* e15, e25, e35, e45 */
            float4((other.e415 * wedge.e3215) + (other.e315 * wedge.e4125) + (other.e4125 * wedge.e315) + (other.e3215 * wedge.e415), (other.e425 * wedge.e3215) + (other.e125 * wedge.e4235) + (other.e4235 * wedge.e125) + (other.e3215 * wedge.e425), (other.e435 * wedge.e3215) + (other.e235 * wedge.e4315) + (other.e4315 * wedge.e235) + (other.e3215 * wedge.e435), -(other.e435 * wedge.e4125) - (other.e4235 * wedge.e415) - (other.e4315 * wedge.e425) - (other.e4125 * wedge.e435)) + (float4(other.e12345) * wedge.group3) + (float4(wedge.e12345) * other.group3) - (wedge.group9.yzxx * float4(other.group8.xyz.zxy, other.e415)) - float4((wedge.group8.xyz.zxy * other.group9.yzx), other.e425 * wedge.e4315), 
            /* e41, e42, e43 */
            (float3(other.e12345) * wedge.group4.xyz) + (float3(other.e1234) * wedge.group6.xyz) + (float3(wedge.e12345) * other.group4.xyz) + (float3(wedge.e1234) * other.group6.xyz) + (other.group7.xyz.zxy * wedge.group9.yzx) + (wedge.group7.xyz.zxy * other.group9.yzx) - (other.group7.xyz.yzx * wedge.group9.zxy) - (wedge.group7.xyz.yzx * other.group9.zxy), 
            /* e23, e31, e12 */
            (float3(other.e12345) * wedge.group5.xyz) + (float3(other.e3215) * wedge.group7.xyz) + (float3(other.e1234) * wedge.group8.xyz) + (float3(wedge.e12345) * other.group5.xyz) + (float3(wedge.e3215) * other.group7.xyz) + (float3(wedge.e1234) * other.group8.xyz) - (float3(other.e321) * wedge.group9.xyz) - (float3(wedge.e321) * other.group9.xyz), 
            /* e415, e425, e435, e321 */
            (float4(other.e12345) * wedge.group6) + (float4(wedge.e12345) * other.group6) + float4((other.group9.zxy * wedge.group9.yzx), other.e1234 * wedge.e3215) - (other.group9.yzxw * float4(wedge.group9.zxy, wedge.e1234)), 
            /* e423, e431, e412 */
            (float3(other.e12345) * wedge.group7.xyz) + (float3(other.e1234) * wedge.group9.xyz) + (float3(wedge.e12345) * other.group7.xyz) - (float3(wedge.e1234) * other.group9.xyz), 
            /* e235, e315, e125 */
            (float3(other.e12345) * wedge.group8.xyz) + (float3(wedge.e12345) * other.group8.xyz) + (float3(wedge.e3215) * other.group9.xyz) - (float3(other.e3215) * wedge.group9.xyz), 
            /* e4235, e4315, e4125, e3215 */
            (float4(other.e12345) * wedge.group9) + (float4(wedge.e12345) * other.group9), 
            /* e1234 */
            (other.e12345 * wedge.e1234) + (other.e1234 * wedge.e12345)
        );
    }
}
public extension AntiFlatPoint {
    public AntiFlatPoint project_orthogonally_onto(Plane other) {
        AntiPlane right_anti_dual = AntiPlane(
            /* e1, e2, e3, e5 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        AntiDualNum wedge = AntiDualNum(
            /* e3215, scalar */
            float2((this.e235 * right_anti_dual.e1) + (this.e315 * right_anti_dual.e2) + (this.e125 * right_anti_dual.e3) + (this.e321 * right_anti_dual.e5), 0.0)
        );
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(wedge.group0.xy.xx, wedge.e3215, 0.0) * float4(float3(1.0), 0.0) * float4(other.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
public extension AntiFlatPoint {
    public Circle project_orthogonally_onto(Sphere other) {
        RoundPoint right_anti_dual = RoundPoint(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5 */
            other.e3215
        );
        Sphere wedge = Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((float3(right_anti_dual.e4) * this.group0.xyz * float3(-1.0)), (this.e235 * right_anti_dual.e1) + (this.e315 * right_anti_dual.e2) + (this.e125 * right_anti_dual.e3) + (this.e321 * right_anti_dual.e5)), 
            /* e1234 */
            this.e321 * right_anti_dual.e4 * -1.0
        );
        return Circle(
            /* e423, e431, e412 */
            (float3(other.e1234) * wedge.group0.xyz) - (float3(wedge.e1234) * other.group0.xyz), 
            /* e415, e425, e435, e321 */
            float4((other.group0.zxy * wedge.group0.yzx), other.e1234 * wedge.e3215) - (other.group0.yzxw * float4(wedge.group0.zxy, wedge.e1234)), 
            /* e235, e315, e125 */
            (float3(wedge.e3215) * other.group0.xyz) - (float3(other.e3215) * wedge.group0.xyz)
        );
    }
}
public extension AntiFlatPoint {
    public VersorEven project_orthogonally_onto(VersorEven other) {
        VersorOdd right_anti_dual = VersorOdd(
            /* e41, e42, e43, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e1234 */
            float4(other.group2.xyz, other.e4) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.group3.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0)
        );
        CircleRotor wedge = CircleRotor(
            /* e423, e431, e412 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), this.e321 * right_anti_dual.scalar), 
            /* e235, e315, e125, e12345 */
            float4((float3(right_anti_dual.scalar) * this.group0.xyz), -(this.e235 * right_anti_dual.e41) - (this.e315 * right_anti_dual.e42) - (this.e125 * right_anti_dual.e43) - (this.e321 * right_anti_dual.e45))
        );
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(((float3(wedge.e12345) * other.group0.xyz) + (float3(other.e12345) * wedge.group0.xyz)), wedge.e12345 * other.e12345), 
            /* e415, e425, e435, e321 */
            (float4(wedge.e12345) * other.group1) + (float4(other.e12345) * wedge.group1), 
            /* e235, e315, e125, e5 */
            float4(wedge.e12345 * other.e235, wedge.e12345 * other.e315, wedge.e12345 * other.e125, -(wedge.e415 * other.e235) - (wedge.e425 * other.e315) - (wedge.e435 * other.e125) - (wedge.e235 * other.e415) - (wedge.e315 * other.e425) - (wedge.e125 * other.e435)) + (wedge.group2 * float4(other.group0.www, other.e5)), 
            /* e1, e2, e3, e4 */
            float4((wedge.e415 * other.e321) + (wedge.e321 * other.e415) + (wedge.e315 * other.e412) + (wedge.e12345 * other.e1), (wedge.e425 * other.e321) + (wedge.e321 * other.e425) + (wedge.e125 * other.e423) + (wedge.e12345 * other.e2), (wedge.e435 * other.e321) + (wedge.e321 * other.e435) + (wedge.e235 * other.e431) + (wedge.e12345 * other.e3), -(wedge.e412 * other.e435) - (wedge.e415 * other.e423) - (wedge.e425 * other.e431) - (wedge.e435 * other.e412)) + float4((wedge.group0.xyz.zxy * other.group2.yzx), wedge.e12345 * other.e4) - float4((wedge.group0.xyz.yzx * other.group2.zxy), wedge.e423 * other.e415) - float4((wedge.group2.zxy * other.group0.yzx), wedge.e431 * other.e425)
        );
    }
}
public extension AntiFlatPoint {
    public AntiDipoleInversion project_orthogonally_onto(VersorOdd other) {
        VersorEven right_anti_dual = VersorEven(
            /* e423, e431, e412, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            float4(other.group2.xyz, other.e3215) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4(other.group3.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0)
        );
        Sphere wedge = Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((float3(right_anti_dual.e4) * this.group0.xyz * float3(-1.0)), (this.e235 * right_anti_dual.e1) + (this.e315 * right_anti_dual.e2) + (this.e125 * right_anti_dual.e3) + (this.e321 * right_anti_dual.e5)), 
            /* e1234 */
            this.e321 * right_anti_dual.e4 * -1.0
        );
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (float3(other.e1234) * wedge.group0.xyz) - (float3(wedge.e1234) * other.group3.xyz), 
            /* e415, e425, e435, e321 */
            (wedge.group0.yzxw * float4(other.group3.zxy, other.e1234)) - float4((wedge.group0.zxy * other.group3.yzx), wedge.e1234 * other.e3215), 
            /* e235, e315, e125, e4 */
            float4(wedge.e3215 * other.e4235, wedge.e3215 * other.e4315, wedge.e3215 * other.e4125, -(wedge.e4315 * other.e42) - (wedge.e4125 * other.e43) - (wedge.e1234 * other.e45)) - (wedge.group0.xyzx * float4(other.group3.www, other.e41)), 
            /* e1, e2, e3, e5 */
            float4(-(wedge.e4315 * other.e12) - (wedge.e1234 * other.e15), -(wedge.e4125 * other.e23) - (wedge.e1234 * other.e25), -(wedge.e4235 * other.e31) - (wedge.e1234 * other.e35), (wedge.e4125 * other.e35) + (wedge.e3215 * other.e45)) + (wedge.group0.zxyx * float4(other.group1.yzx, other.e15)) + (wedge.group0.wwwy * float4(other.group0.xyz, other.e25))
        );
    }
}
public extension AntiFlector {
    // Fancy infix trick (first half)
    public ProjectOrthogonallyOntoInfixPartial<AntiFlector> operator /(ProjectOrthogonallyOntoInfix rhs) {
        return ProjectOrthogonallyOntoInfixPartial<AntiFlector>(this);
    }
}
public extension ProjectOrthogonallyOntoInfixPartial<AntiFlector> {
    // Fancy infix trick (second half)
    public RoundPoint operator /(AntiCircleRotor rhs) {
        return this.a.project_orthogonally_onto(rhs);
    }
}
public extension AntiFlector {
    public RoundPoint project_orthogonally_onto(AntiCircleRotor other) {
        CircleRotor right_anti_dual = CircleRotor(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        Sphere wedge = Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(this.e3 * right_anti_dual.e425, this.e1 * right_anti_dual.e435, this.e2 * right_anti_dual.e415, -(this.e3 * right_anti_dual.e125) - (this.e5 * right_anti_dual.e321)) - (this.group1.yzxy * float4(right_anti_dual.group1.zxy, right_anti_dual.e315)) - float4((right_anti_dual.group0.xyz * this.group1.www), this.e1 * right_anti_dual.e235), 
            /* e1234 */
            (this.e1 * right_anti_dual.e423) + (this.e2 * right_anti_dual.e431) + (this.e3 * right_anti_dual.e412)
        );
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4((other.e41 * wedge.e3215) + (other.e31 * wedge.e4125), (other.e42 * wedge.e3215) + (other.e12 * wedge.e4235), (other.e43 * wedge.e3215) + (other.e23 * wedge.e4315), -(other.e43 * wedge.e4125) - (other.e45 * wedge.e1234)) - (wedge.group0.yzxx * float4(other.group1.zxy, other.e41)) - float4((float3(wedge.e1234) * other.group2.xyz), other.e42 * wedge.e4315), 
            /* e5 */
            (other.e45 * wedge.e3215) + (other.e15 * wedge.e4235) + (other.e25 * wedge.e4315) + (other.e35 * wedge.e4125)
        );
    }
}
public extension AntiFlector {
    public AntiDipoleInversion project_orthogonally_onto(AntiDipoleInversion other) {
        DipoleInversion right_anti_dual = DipoleInversion(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e1234 */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group3 * float4(1.0, 1.0, 1.0, -1.0)
        );
        CircleRotor wedge = CircleRotor(
            /* e423, e431, e412 */
            (right_anti_dual.group0.xyz.yzx * this.group1.zxy) - (right_anti_dual.group0.xyz.zxy * this.group1.yzx), 
            /* e415, e425, e435, e321 */
            float4(this.e5 * right_anti_dual.e41, this.e5 * right_anti_dual.e42, this.e5 * right_anti_dual.e43, -(this.e2 * right_anti_dual.e31) - (this.e3 * right_anti_dual.e12)) - (this.group1.xyzx * right_anti_dual.group1.wwwx), 
            /* e235, e315, e125, e12345 */
            (this.group1.yzxx * float4(right_anti_dual.group2.zxy, right_anti_dual.e4235)) + (this.group1.wwwy * float4(right_anti_dual.group1.xyz, right_anti_dual.e4315)) + float4(float3(0.0), (this.e3 * right_anti_dual.e4125) + (this.e5 * right_anti_dual.e1234) - (this.e315 * right_anti_dual.e42) - (this.e125 * right_anti_dual.e43) - (this.e321 * right_anti_dual.e45)) - float4((this.group1.zxy * right_anti_dual.group2.yzx), this.e235 * right_anti_dual.e41)
        );
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(wedge.e12345) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(wedge.e12345) * other.group1, 
            /* e235, e315, e125, e4 */
            float4((float3(wedge.e12345) * other.group2.xyz), (other.e4 * wedge.e12345) - (other.e423 * wedge.e415) - (other.e431 * wedge.e425) - (other.e412 * wedge.e435) - (other.e415 * wedge.e423) - (other.e425 * wedge.e431) - (other.e435 * wedge.e412)), 
            /* e1, e2, e3, e5 */
            float4((other.e415 * wedge.e321) + (other.e321 * wedge.e415) + (other.e315 * wedge.e412) + (other.e1 * wedge.e12345), (other.e425 * wedge.e321) + (other.e321 * wedge.e425) + (other.e125 * wedge.e423) + (other.e2 * wedge.e12345), (other.e435 * wedge.e321) + (other.e321 * wedge.e435) + (other.e235 * wedge.e431) + (other.e3 * wedge.e12345), -(other.e435 * wedge.e125) - (other.e235 * wedge.e415) - (other.e315 * wedge.e425) - (other.e125 * wedge.e435)) + (wedge.group2.yzxw * float4(other.group0.xyz.zxy, other.e5)) - (wedge.group2.zxyx * float4(other.group0.xyz.yzx, other.e415)) - float4((wedge.group0.xyz.yzx * other.group2.zxy), other.e425 * wedge.e315)
        );
    }
}
public extension AntiFlector {
    public AntiFlatPoint project_orthogonally_onto(AntiDualNum other) {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(other.group0.xy.xx, other.e3215, 0.0) * float4(float3(0.0), 0.0) * float4(float3(1.0), 0.0) * float4(-1.0, -1.0, -1.0, 0.0)
        );
    }
}
public extension AntiFlector {
    public AntiFlector project_orthogonally_onto(AntiFlatPoint other) {
        FlatPoint right_anti_dual = FlatPoint(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        Motor wedge = Motor(
            /* e415, e425, e435, e12345 */
            float4(right_anti_dual.e45) * float4(this.group1.xyz, this.e321) * float4(-1.0), 
            /* e235, e315, e125, e5 */
            float4(((this.group1.yzx * right_anti_dual.group0.zxy) - (this.group1.zxy * right_anti_dual.group0.yzx)), 0.0)
        );
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(wedge.e12345) * other.group0, 
            /* e1, e2, e3, e5 */
            float4((other.group0.www * wedge.group0.xyz), -(other.e235 * wedge.e415) - (other.e315 * wedge.e425) - (other.e125 * wedge.e435))
        );
    }
}
public extension AntiFlector {
    public AntiFlector project_orthogonally_onto(AntiFlector other) {
        Flector right_anti_dual = Flector(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        Motor wedge = Motor(
            /* e415, e425, e435, e12345 */
            float4((float3(right_anti_dual.e45) * this.group1.xyz * float3(-1.0)), (this.e1 * right_anti_dual.e4235) + (this.e2 * right_anti_dual.e4315) + (this.e3 * right_anti_dual.e4125) - (this.e321 * right_anti_dual.e45)), 
            /* e235, e315, e125, e5 */
            float4(((this.group1.yzx * right_anti_dual.group0.zxy) - (this.group1.zxy * right_anti_dual.group0.yzx)), 0.0)
        );
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(wedge.e12345) * other.group0, 
            /* e1, e2, e3, e5 */
            float4(other.e1 * wedge.e12345, other.e2 * wedge.e12345, other.e3 * wedge.e12345, -(other.e235 * wedge.e415) - (other.e315 * wedge.e425) - (other.e125 * wedge.e435)) + (wedge.group0 * float4(other.group0.www, other.e5))
        );
    }
}
public extension AntiFlector {
    public AntiPlane project_orthogonally_onto(AntiLine other) {
        Line right_anti_dual = Line(
            /* e415, e425, e435 */
            other.group0.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            other.group1.xyz * float3(-1.0)
        );
        Plane wedge = Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(this.e3 * right_anti_dual.e425, this.e1 * right_anti_dual.e435, this.e2 * right_anti_dual.e415, -(this.e2 * right_anti_dual.e315) - (this.e3 * right_anti_dual.e125)) - (this.group1.yzxx * float4(right_anti_dual.group0.xyz.zxy, right_anti_dual.e235))
        );
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(other.e12 * wedge.e4315 * -1.0, other.e23 * wedge.e4125 * -1.0, other.e31 * wedge.e4235 * -1.0, (other.e25 * wedge.e4315) + (other.e35 * wedge.e4125)) + (wedge.group0.zxyx * float4(other.group0.xyz.yzx, other.e15))
        );
    }
}
public extension AntiFlector {
    public AntiFlector project_orthogonally_onto(AntiMotor other) {
        Motor right_anti_dual = Motor(
            /* e415, e425, e435, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        Flector wedge = Flector(
            /* e15, e25, e35, e45 */
            float4(float3(1.0), 0.0) * float4(right_anti_dual.group1.www, 0.0) * float4(this.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), -(this.e2 * right_anti_dual.e315) - (this.e3 * right_anti_dual.e125)) + float4((this.group1.zxy * right_anti_dual.group0.yzx), this.e321 * right_anti_dual.e5) - (this.group1.yzxx * float4(right_anti_dual.group0.zxy, right_anti_dual.e235))
        );
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(float3(1.0), 0.0) * float4(other.group1.www, 0.0) * float4(wedge.group1.xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0), 
            /* e1, e2, e3, e5 */
            (wedge.group1.zxyx * float4(other.group0.yzx, other.e15)) + float4(float3(0.0), (other.e25 * wedge.e4315) + (other.e35 * wedge.e4125)) - float4((other.group0.zxy * wedge.group1.yzx), other.e3215 * wedge.e45)
        );
    }
}
public extension AntiFlector {
    public AntiPlane project_orthogonally_onto(AntiPlane other) {
        Plane right_anti_dual = Plane(
            /* e4235, e4315, e4125, e3215 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4((this.e1 * right_anti_dual.e4235) + (this.e2 * right_anti_dual.e4315) + (this.e3 * right_anti_dual.e4125)) * other.group0
        );
    }
}
public extension AntiFlector {
    public AntiFlector project_orthogonally_onto(AntiScalar other) {
        Scalar right_anti_dual = Scalar(
            /* scalar */
            other.e12345 * -1.0
        );
        AntiFlector wedge = AntiFlector(
            /* e235, e315, e125, e321 */
            float4(right_anti_dual.scalar) * this.group0, 
            /* e1, e2, e3, e5 */
            float4(right_anti_dual.scalar) * this.group1
        );
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(other.e12345) * wedge.group0, 
            /* e1, e2, e3, e5 */
            float4(other.e12345) * wedge.group1
        );
    }
}
public extension AntiFlector {
    public AntiDipoleInversion project_orthogonally_onto(Circle other) {
        Dipole right_anti_dual = Dipole(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35 */
            other.group2.xyz
        );
        CircleRotor wedge = CircleRotor(
            /* e423, e431, e412 */
            (right_anti_dual.group0.xyz.yzx * this.group1.zxy) - (right_anti_dual.group0.xyz.zxy * this.group1.yzx), 
            /* e415, e425, e435, e321 */
            float4(this.e5 * right_anti_dual.e41, this.e5 * right_anti_dual.e42, this.e5 * right_anti_dual.e43, -(this.e2 * right_anti_dual.e31) - (this.e3 * right_anti_dual.e12)) - (this.group1.xyzx * right_anti_dual.group1.wwwx), 
            /* e235, e315, e125, e12345 */
            float4((this.e2 * right_anti_dual.e35) + (this.e5 * right_anti_dual.e23), (this.e3 * right_anti_dual.e15) + (this.e5 * right_anti_dual.e31), (this.e1 * right_anti_dual.e25) + (this.e5 * right_anti_dual.e12), -(this.e315 * right_anti_dual.e42) - (this.e125 * right_anti_dual.e43) - (this.e321 * right_anti_dual.e45)) - float4((right_anti_dual.group2.xyz.yzx * this.group1.zxy), this.e235 * right_anti_dual.e41)
        );
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(wedge.e12345) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(wedge.e12345) * other.group1, 
            /* e235, e315, e125, e4 */
            float4((float3(wedge.e12345) * other.group2.xyz), -(other.e423 * wedge.e415) - (other.e431 * wedge.e425) - (other.e412 * wedge.e435) - (other.e415 * wedge.e423) - (other.e425 * wedge.e431) - (other.e435 * wedge.e412)), 
            /* e1, e2, e3, e5 */
            float4((other.e412 * wedge.e315) + (other.e415 * wedge.e321) + (other.e321 * wedge.e415) + (other.e315 * wedge.e412), (other.e423 * wedge.e125) + (other.e425 * wedge.e321) + (other.e321 * wedge.e425) + (other.e125 * wedge.e423), (other.e431 * wedge.e235) + (other.e435 * wedge.e321) + (other.e321 * wedge.e435) + (other.e235 * wedge.e431), -(other.e415 * wedge.e235) - (other.e425 * wedge.e315) - (other.e435 * wedge.e125) - (other.e125 * wedge.e435)) - float4((other.group0.xyz.yzx * wedge.group2.zxy), other.e235 * wedge.e415) - float4((other.group2.xyz.zxy * wedge.group0.xyz.yzx), other.e315 * wedge.e425)
        );
    }
}
public extension AntiFlector {
    public VersorEven project_orthogonally_onto(CircleRotor other) {
        AntiCircleRotor right_anti_dual = AntiCircleRotor(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        VersorEven wedge = VersorEven(
            /* e423, e431, e412, e12345 */
            float4(right_anti_dual.e42 * this.e3, right_anti_dual.e43 * this.e1, right_anti_dual.e41 * this.e2, -(right_anti_dual.e42 * this.e315) - (right_anti_dual.e43 * this.e125) - (right_anti_dual.e45 * this.e321)) - float4((right_anti_dual.group0.xyz.zxy * this.group1.yzx), right_anti_dual.e41 * this.e235), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), -(right_anti_dual.e31 * this.e2) - (right_anti_dual.e12 * this.e3)) + float4((right_anti_dual.group0.xyz * this.group1.www), right_anti_dual.scalar * this.e321) - (right_anti_dual.group1.wwwx * this.group1.xyzx), 
            /* e235, e315, e125, e5 */
            float4(((float3(right_anti_dual.scalar) * this.group0.xyz) + (float3(this.e5) * right_anti_dual.group1.xyz) + (right_anti_dual.group2.zxy * this.group1.yzx) - (right_anti_dual.group2.yzx * this.group1.zxy)), right_anti_dual.scalar * this.e5), 
            /* e1, e2, e3, e4 */
            float4(float3(1.0), 0.0) * float4(right_anti_dual.group2.www, 0.0) * float4(this.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(((float3(other.e12345) * wedge.group0.xyz) + (float3(wedge.e12345) * other.group0.xyz)), other.e12345 * wedge.e12345), 
            /* e415, e425, e435, e321 */
            (float4(other.e12345) * wedge.group1) + (float4(wedge.e12345) * other.group1), 
            /* e235, e315, e125, e5 */
            float4(other.e12345 * wedge.e235, other.e12345 * wedge.e315, other.e12345 * wedge.e125, -(other.e415 * wedge.e235) - (other.e425 * wedge.e315) - (other.e435 * wedge.e125) - (other.e235 * wedge.e415) - (other.e315 * wedge.e425) - (other.e125 * wedge.e435)) + (other.group2 * float4(wedge.group0.www, wedge.e5)), 
            /* e1, e2, e3, e4 */
            float4((other.e415 * wedge.e321) + (other.e321 * wedge.e415) + (other.e315 * wedge.e412) + (other.e12345 * wedge.e1), (other.e425 * wedge.e321) + (other.e321 * wedge.e425) + (other.e125 * wedge.e423) + (other.e12345 * wedge.e2), (other.e435 * wedge.e321) + (other.e321 * wedge.e435) + (other.e235 * wedge.e431) + (other.e12345 * wedge.e3), -(other.e412 * wedge.e435) - (other.e415 * wedge.e423) - (other.e425 * wedge.e431) - (other.e435 * wedge.e412)) + float4((other.group0.xyz.zxy * wedge.group2.yzx), other.e12345 * wedge.e4) - float4((other.group0.xyz.yzx * wedge.group2.zxy), other.e423 * wedge.e415) - float4((other.group2.zxy * wedge.group0.yzx), other.e431 * wedge.e425)
        );
    }
}
public extension AntiFlector {
    public RoundPoint project_orthogonally_onto(Dipole other) {
        Circle right_anti_dual = Circle(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125 */
            other.group2.xyz * float3(-1.0)
        );
        Sphere wedge = Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(this.e3 * right_anti_dual.e425, this.e1 * right_anti_dual.e435, this.e2 * right_anti_dual.e415, -(this.e3 * right_anti_dual.e125) - (this.e5 * right_anti_dual.e321)) - (this.group1.wwwx * float4(right_anti_dual.group0.xyz, right_anti_dual.e235)) - float4((this.group1.yzx * right_anti_dual.group1.zxy), this.e2 * right_anti_dual.e315), 
            /* e1234 */
            (this.e1 * right_anti_dual.e423) + (this.e2 * right_anti_dual.e431) + (this.e3 * right_anti_dual.e412)
        );
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4((other.e41 * wedge.e3215) + (other.e31 * wedge.e4125), (other.e42 * wedge.e3215) + (other.e12 * wedge.e4235), (other.e43 * wedge.e3215) + (other.e23 * wedge.e4315), -(other.e43 * wedge.e4125) - (other.e45 * wedge.e1234)) - (wedge.group0.yzxy * float4(other.group1.zxy, other.e42)) - float4((float3(wedge.e1234) * other.group2.xyz), other.e41 * wedge.e4235), 
            /* e5 */
            (other.e45 * wedge.e3215) + (other.e15 * wedge.e4235) + (other.e25 * wedge.e4315) + (other.e35 * wedge.e4125)
        );
    }
}
public extension AntiFlector {
    public AntiDipoleInversion project_orthogonally_onto(DipoleInversion other) {
        AntiDipoleInversion right_anti_dual = AntiDipoleInversion(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e4 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group3 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        DipoleInversion wedge = DipoleInversion(
            /* e41, e42, e43 */
            float3(right_anti_dual.e4) * this.group1.xyz * float3(-1.0), 
            /* e23, e31, e12, e45 */
            float4(((right_anti_dual.group3.zxy * this.group1.yzx) - (right_anti_dual.group3.yzx * this.group1.zxy)), right_anti_dual.e4 * this.e5 * -1.0), 
            /* e15, e25, e35, e1234 */
            (this.group1.xyzx * float4(right_anti_dual.group3.www, right_anti_dual.e423)) + float4(float3(0.0), (right_anti_dual.e431 * this.e2) + (right_anti_dual.e412 * this.e3)) - float4((this.group1.www * right_anti_dual.group3.xyz), right_anti_dual.e4 * this.e321), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), (right_anti_dual.e2 * this.e315) + (right_anti_dual.e3 * this.e125) + (right_anti_dual.e5 * this.e321) - (right_anti_dual.e125 * this.e3)) + float4((right_anti_dual.group1.yzx * this.group1.zxy), right_anti_dual.e1 * this.e235) - (float4(this.e5) * float4(right_anti_dual.group0.xyz, right_anti_dual.e321)) - (right_anti_dual.group2.wwwy * float4(this.group0.xyz, this.e2)) - (this.group1.yzxx * float4(right_anti_dual.group1.zxy, right_anti_dual.e235))
        );
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (float3(other.e1234) * wedge.group3.xyz) - (float3(wedge.e1234) * other.group3.xyz), 
            /* e415, e425, e435, e321 */
            (wedge.group3.yzxw * float4(other.group3.zxy, other.e1234)) - (other.group3.yzxw * float4(wedge.group3.zxy, wedge.e1234)), 
            /* e235, e315, e125, e4 */
            float4(float3(0.0), (other.e1234 * wedge.e45) + (other.e4315 * wedge.e42) + (other.e4125 * wedge.e43) - (other.e42 * wedge.e4315) - (other.e43 * wedge.e4125) - (other.e45 * wedge.e1234)) + float4((wedge.group3.www * other.group3.xyz), other.e4235 * wedge.e41) - (wedge.group3.xyzx * float4(other.group3.www, other.e41)), 
            /* e1, e2, e3, e5 */
            (float4(wedge.e3215) * float4(other.group0.xyz, other.e45)) + (other.group2.wwwy * float4(wedge.group2.xyz, wedge.e4315)) + (wedge.group3.zxyx * float4(other.group1.yzx, other.e15)) + float4((other.group3.yzx * wedge.group1.zxy), other.e35 * wedge.e4125) - (other.group3.zxyw * wedge.group1.yzxw) - (wedge.group2.wwwz * float4(other.group2.xyz, other.e4125)) - float4((wedge.group0.xyz * other.group3.www), other.e4235 * wedge.e15) - float4((other.group1.zxy * wedge.group3.yzx), other.e4315 * wedge.e25)
        );
    }
}
public extension AntiFlector {
    public AntiFlector project_orthogonally_onto(DualNum other) {
        AntiDualNum right_anti_dual = AntiDualNum(
            /* e3215, scalar */
            other.group0.xy * float2(-1.0)
        );
        AntiFlector wedge = AntiFlector(
            /* e235, e315, e125, e321 */
            float4(right_anti_dual.scalar) * this.group0, 
            /* e1, e2, e3, e5 */
            float4(right_anti_dual.scalar) * this.group1
        );
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(other.e12345) * wedge.group0, 
            /* e1, e2, e3, e5 */
            float4(other.e12345) * wedge.group1
        );
    }
}
public extension AntiFlector {
    public DualNum project_orthogonally_onto(FlatPoint other) {
        AntiFlatPoint right_anti_dual = AntiFlatPoint(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return DualNum(
            /* e5, e12345 */
            float2(-(right_anti_dual.e235 * this.e1 * other.e45) - (right_anti_dual.e315 * this.e2 * other.e45) - (right_anti_dual.e125 * this.e3 * other.e45) - (right_anti_dual.e321 * this.e5 * other.e45), 1.0) * float2(1.0, 0.0)
        );
    }
}
public extension AntiFlector {
    public AntiFlector project_orthogonally_onto(Flector other) {
        AntiFlector right_anti_dual = AntiFlector(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        AntiMotor wedge = AntiMotor(
            /* e23, e31, e12, scalar */
            float4(((right_anti_dual.group1.zxy * this.group1.yzx) - (right_anti_dual.group1.yzx * this.group1.zxy)), 0.0), 
            /* e15, e25, e35, e3215 */
            (right_anti_dual.group1.wwwx * float4(this.group1.xyz, this.e235)) + float4(float3(0.0), (right_anti_dual.e2 * this.e315) + (right_anti_dual.e3 * this.e125) + (right_anti_dual.e5 * this.e321) - (right_anti_dual.e315 * this.e2) - (right_anti_dual.e125 * this.e3) - (right_anti_dual.e321 * this.e5)) - (this.group1.wwwx * float4(right_anti_dual.group1.xyz, right_anti_dual.e235))
        );
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(float3(1.0), 0.0) * float4(wedge.group1.www, 0.0) * float4(other.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), -(wedge.e25 * other.e4315) - (wedge.e35 * other.e4125)) + float4((wedge.group0.zxy * other.group1.yzx), wedge.e3215 * other.e45) - (other.group1.zxyx * float4(wedge.group0.yzx, wedge.e15))
        );
    }
}
public extension AntiFlector {
    public AntiPlane project_orthogonally_onto(Line other) {
        AntiLine right_anti_dual = AntiLine(
            /* e23, e31, e12 */
            other.group0.xyz, 
            /* e15, e25, e35 */
            other.group1.xyz
        );
        AntiFlatPoint wedge = AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4((this.e2 * right_anti_dual.e35) + (this.e5 * right_anti_dual.e23), (this.e3 * right_anti_dual.e15) + (this.e5 * right_anti_dual.e31), (this.e1 * right_anti_dual.e25) + (this.e5 * right_anti_dual.e12), -(this.e2 * right_anti_dual.e31) - (this.e3 * right_anti_dual.e12)) - (this.group1.zxyx * float4(right_anti_dual.group1.xyz.yzx, right_anti_dual.e23))
        );
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4((float3(wedge.e321) * other.group0.xyz), -(wedge.e235 * other.e415) - (wedge.e315 * other.e425) - (wedge.e125 * other.e435))
        );
    }
}
public extension AntiFlector {
    public AntiFlector project_orthogonally_onto(Motor other) {
        AntiMotor right_anti_dual = AntiMotor(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        AntiFlector wedge = AntiFlector(
            /* e235, e315, e125, e321 */
            float4((this.e2 * right_anti_dual.e35) + (this.e5 * right_anti_dual.e23), (this.e3 * right_anti_dual.e15) + (this.e5 * right_anti_dual.e31), (this.e1 * right_anti_dual.e25) + (this.e5 * right_anti_dual.e12), -(this.e2 * right_anti_dual.e31) - (this.e3 * right_anti_dual.e12)) + (float4(right_anti_dual.scalar) * this.group0) - (this.group1.zxyx * float4(right_anti_dual.group1.yzx, right_anti_dual.e23)), 
            /* e1, e2, e3, e5 */
            float4(right_anti_dual.scalar) * this.group1
        );
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(other.e12345) * wedge.group0, 
            /* e1, e2, e3, e5 */
            float4(wedge.e1 * other.e12345, wedge.e2 * other.e12345, wedge.e3 * other.e12345, -(wedge.e235 * other.e415) - (wedge.e315 * other.e425) - (wedge.e125 * other.e435)) + (other.group0 * float4(wedge.group0.www, wedge.e5))
        );
    }
}
public extension AntiFlector {
    public MultiVector project_orthogonally_onto(MultiVector other) {
        MultiVector right_anti_dual = MultiVector(
            /* scalar, e12345 */
            other.group0.xy.yx * float2(-1.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4(other.group9.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5 */
            other.e3215, 
            /* e15, e25, e35, e45 */
            float4(other.group8.xyz, other.e321 * -1.0), 
            /* e41, e42, e43 */
            other.group7.xyz, 
            /* e23, e31, e12 */
            other.group6.xyz, 
            /* e415, e425, e435, e321 */
            float4(other.group5.xyz, other.e45) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e423, e431, e412 */
            other.group4.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            other.group3.xyz * float3(-1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.group1.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234 */
            other.e4 * -1.0
        );
        MultiVector wedge = MultiVector(
            /* scalar, e12345 */
            float2(0.0, (this.e1 * right_anti_dual.e4235) + (this.e2 * right_anti_dual.e4315) + (this.e3 * right_anti_dual.e4125) + (this.e5 * right_anti_dual.e1234) - (this.e235 * right_anti_dual.e41) - (this.e315 * right_anti_dual.e42) - (this.e125 * right_anti_dual.e43) - (this.e321 * right_anti_dual.e45)), 
            /* e1, e2, e3, e4 */
            float4(right_anti_dual.group0.xy.xx, right_anti_dual.scalar, 0.0) * float4(float3(1.0), 0.0) * float4(this.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e5 */
            this.e5 * right_anti_dual.scalar, 
            /* e15, e25, e35, e45 */
            float4(((float3(right_anti_dual.e5) * this.group1.xyz) - (float3(this.e5) * right_anti_dual.group1.xyz)), this.e5 * right_anti_dual.e4 * -1.0), 
            /* e41, e42, e43 */
            float3(right_anti_dual.e4) * this.group1.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            (this.group1.yzx * right_anti_dual.group1.zxy) - (this.group1.zxy * right_anti_dual.group1.yzx), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), -(this.e2 * right_anti_dual.e31) - (this.e3 * right_anti_dual.e12)) + float4((right_anti_dual.group4.xyz * this.group1.www), this.e321 * right_anti_dual.scalar) - float4((right_anti_dual.group3.www * this.group1.xyz), this.e1 * right_anti_dual.e23), 
            /* e423, e431, e412 */
            (right_anti_dual.group4.xyz.yzx * this.group1.zxy) - (right_anti_dual.group4.xyz.zxy * this.group1.yzx), 
            /* e235, e315, e125 */
            (float3(this.e5) * right_anti_dual.group5.xyz) + (float3(right_anti_dual.scalar) * this.group0.xyz) + (this.group1.yzx * right_anti_dual.group3.zxy) - (this.group1.zxy * right_anti_dual.group3.yzx), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), (this.e315 * right_anti_dual.e2) + (this.e125 * right_anti_dual.e3) + (this.e321 * right_anti_dual.e5) - (this.e5 * right_anti_dual.e321)) + float4((this.group1.zxy * right_anti_dual.group6.yzx), this.e235 * right_anti_dual.e1) - (this.group1.wwwx * float4(right_anti_dual.group7.xyz, right_anti_dual.e235)) - float4((this.group1.yzx * right_anti_dual.group6.zxy), this.e3 * right_anti_dual.e125) - float4((right_anti_dual.group1.www * this.group0.xyz), this.e2 * right_anti_dual.e315), 
            /* e1234 */
            (this.e1 * right_anti_dual.e423) + (this.e2 * right_anti_dual.e431) + (this.e3 * right_anti_dual.e412) - (this.e321 * right_anti_dual.e4)
        );
        return MultiVector(
            /* scalar, e12345 */
            float2((other.scalar * wedge.e12345) + (other.e12345 * wedge.scalar) + (other.e1 * wedge.e4235) + (other.e2 * wedge.e4315) + (other.e3 * wedge.e4125) + (other.e4 * wedge.e3215) + (other.e5 * wedge.e1234) + (other.e4235 * wedge.e1) + (other.e4315 * wedge.e2) + (other.e4125 * wedge.e3) + (other.e3215 * wedge.e4) + (other.e1234 * wedge.e5) - (other.e15 * wedge.e423) - (other.e25 * wedge.e431) - (other.e35 * wedge.e412) - (other.e45 * wedge.e321) - (other.e41 * wedge.e235) - (other.e42 * wedge.e315) - (other.e43 * wedge.e125) - (other.e23 * wedge.e415) - (other.e31 * wedge.e425) - (other.e12 * wedge.e435) - (other.e415 * wedge.e23) - (other.e425 * wedge.e31) - (other.e435 * wedge.e12) - (other.e321 * wedge.e45) - (other.e423 * wedge.e15) - (other.e431 * wedge.e25) - (other.e412 * wedge.e35) - (other.e235 * wedge.e41) - (other.e315 * wedge.e42) - (other.e125 * wedge.e43), other.e12345 * wedge.e12345), 
            /* e1, e2, e3, e4 */
            float4((other.e415 * wedge.e321) + (other.e321 * wedge.e415) + (other.e4315 * wedge.e12) + (other.e1234 * wedge.e15), (other.e425 * wedge.e321) + (other.e321 * wedge.e425) + (other.e4125 * wedge.e23) + (other.e1234 * wedge.e25), (other.e435 * wedge.e321) + (other.e321 * wedge.e435) + (other.e4235 * wedge.e31) + (other.e1234 * wedge.e35), -(other.e45 * wedge.e1234) - (other.e415 * wedge.e423) - (other.e425 * wedge.e431) - (other.e435 * wedge.e412)) + (float4(other.e12345) * wedge.group1) + (float4(wedge.e12345) * other.group1) + float4((other.group4.xyz * wedge.group9.www), other.e4235 * wedge.e41) + float4((other.group5.xyz.yzx * wedge.group9.zxy), other.e4315 * wedge.e42) + float4((other.group7.xyz.zxy * wedge.group8.xyz.yzx), other.e4125 * wedge.e43) + float4((other.group8.xyz.yzx * wedge.group7.xyz.zxy), other.e1234 * wedge.e45) - (wedge.group9.yzxx * float4(other.group5.xyz.zxy, other.e41)) - float4((float3(wedge.e1234) * other.group3.xyz), other.e412 * wedge.e435) - float4((wedge.group4.xyz * other.group9.www), other.e423 * wedge.e415) - float4((other.group7.xyz.yzx * wedge.group8.xyz.zxy), other.e42 * wedge.e4315) - float4((other.group8.xyz.zxy * wedge.group7.xyz.yzx), other.e43 * wedge.e4125) - float4((wedge.group5.xyz.yzx * other.group9.zxy), other.e431 * wedge.e425), 
            /* e5 */
            (other.e12345 * wedge.e5) + (other.e5 * wedge.e12345) + (other.e15 * wedge.e4235) + (other.e25 * wedge.e4315) + (other.e35 * wedge.e4125) + (other.e45 * wedge.e3215) - (other.e415 * wedge.e235) - (other.e425 * wedge.e315) - (other.e435 * wedge.e125) - (other.e235 * wedge.e415) - (other.e315 * wedge.e425) - (other.e125 * wedge.e435) - (other.e4235 * wedge.e15) - (other.e4315 * wedge.e25) - (other.e4125 * wedge.e35) - (other.e3215 * wedge.e45), 
            /* e15, e25, e35, e45 */
            float4((other.e415 * wedge.e3215) + (other.e315 * wedge.e4125) + (other.e4125 * wedge.e315) + (other.e3215 * wedge.e415), (other.e425 * wedge.e3215) + (other.e125 * wedge.e4235) + (other.e4235 * wedge.e125) + (other.e3215 * wedge.e425), (other.e435 * wedge.e3215) + (other.e235 * wedge.e4315) + (other.e4315 * wedge.e235) + (other.e3215 * wedge.e435), -(other.e435 * wedge.e4125) - (other.e4235 * wedge.e415) - (other.e4315 * wedge.e425) - (other.e4125 * wedge.e435)) + (float4(other.e12345) * wedge.group3) + (float4(wedge.e12345) * other.group3) - (wedge.group9.yzxx * float4(other.group8.xyz.zxy, other.e415)) - float4((wedge.group8.xyz.zxy * other.group9.yzx), other.e425 * wedge.e4315), 
            /* e41, e42, e43 */
            (float3(other.e12345) * wedge.group4.xyz) + (float3(other.e1234) * wedge.group6.xyz) + (float3(wedge.e12345) * other.group4.xyz) + (float3(wedge.e1234) * other.group6.xyz) + (other.group7.xyz.zxy * wedge.group9.yzx) + (wedge.group7.xyz.zxy * other.group9.yzx) - (other.group7.xyz.yzx * wedge.group9.zxy) - (wedge.group7.xyz.yzx * other.group9.zxy), 
            /* e23, e31, e12 */
            (float3(other.e12345) * wedge.group5.xyz) + (float3(other.e3215) * wedge.group7.xyz) + (float3(other.e1234) * wedge.group8.xyz) + (float3(wedge.e12345) * other.group5.xyz) + (float3(wedge.e3215) * other.group7.xyz) + (float3(wedge.e1234) * other.group8.xyz) - (float3(other.e321) * wedge.group9.xyz) - (float3(wedge.e321) * other.group9.xyz), 
            /* e415, e425, e435, e321 */
            (float4(other.e12345) * wedge.group6) + (float4(wedge.e12345) * other.group6) + float4((other.group9.zxy * wedge.group9.yzx), other.e1234 * wedge.e3215) - (other.group9.yzxw * float4(wedge.group9.zxy, wedge.e1234)), 
            /* e423, e431, e412 */
            (float3(other.e12345) * wedge.group7.xyz) + (float3(other.e1234) * wedge.group9.xyz) + (float3(wedge.e12345) * other.group7.xyz) - (float3(wedge.e1234) * other.group9.xyz), 
            /* e235, e315, e125 */
            (float3(other.e12345) * wedge.group8.xyz) + (float3(wedge.e12345) * other.group8.xyz) + (float3(wedge.e3215) * other.group9.xyz) - (float3(other.e3215) * wedge.group9.xyz), 
            /* e4235, e4315, e4125, e3215 */
            (float4(other.e12345) * wedge.group9) + (float4(wedge.e12345) * other.group9), 
            /* e1234 */
            (other.e12345 * wedge.e1234) + (other.e1234 * wedge.e12345)
        );
    }
}
public extension AntiFlector {
    public AntiFlector project_orthogonally_onto(Plane other) {
        AntiPlane right_anti_dual = AntiPlane(
            /* e1, e2, e3, e5 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        AntiMotor wedge = AntiMotor(
            /* e23, e31, e12, scalar */
            float4(((this.group1.yzx * right_anti_dual.group0.zxy) - (this.group1.zxy * right_anti_dual.group0.yzx)), 0.0), 
            /* e15, e25, e35, e3215 */
            float4(this.e5 * right_anti_dual.e1 * -1.0, this.e5 * right_anti_dual.e2 * -1.0, this.e5 * right_anti_dual.e3 * -1.0, (this.e315 * right_anti_dual.e2) + (this.e125 * right_anti_dual.e3) + (this.e321 * right_anti_dual.e5)) + (right_anti_dual.group0.wwwx * float4(this.group1.xyz, this.e235))
        );
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(float3(1.0), 0.0) * float4(wedge.group1.www, 0.0) * float4(other.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e1, e2, e3, e5 */
            float4(wedge.e12 * other.e4315, wedge.e23 * other.e4125, wedge.e31 * other.e4235, -(wedge.e25 * other.e4315) - (wedge.e35 * other.e4125)) - (other.group0.zxyx * float4(wedge.group0.yzx, wedge.e15))
        );
    }
}
public extension AntiFlector {
    public RoundPoint project_orthogonally_onto(RoundPoint other) {
        Sphere right_anti_dual = Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234 */
            other.e4 * -1.0
        );
        AntiScalar wedge = AntiScalar(
            /* e12345 */
            (this.e1 * right_anti_dual.e4235) + (this.e2 * right_anti_dual.e4315) + (this.e3 * right_anti_dual.e4125) + (this.e5 * right_anti_dual.e1234)
        );
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(wedge.e12345) * other.group0, 
            /* e5 */
            wedge.e12345 * other.e5
        );
    }
}
public extension AntiFlector {
    public AntiDipoleInversion project_orthogonally_onto(Sphere other) {
        RoundPoint right_anti_dual = RoundPoint(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5 */
            other.e3215
        );
        DipoleInversion wedge = DipoleInversion(
            /* e41, e42, e43 */
            float3(right_anti_dual.e4) * this.group1.xyz * float3(-1.0), 
            /* e23, e31, e12, e45 */
            float4(((this.group1.yzx * right_anti_dual.group0.zxy) - (this.group1.zxy * right_anti_dual.group0.yzx)), this.e5 * right_anti_dual.e4 * -1.0), 
            /* e15, e25, e35, e1234 */
            float4(((float3(right_anti_dual.e5) * this.group1.xyz) - (float3(this.e5) * right_anti_dual.group0.xyz)), this.e321 * right_anti_dual.e4 * -1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4((float3(right_anti_dual.e4) * this.group0.xyz * float3(-1.0)), (this.e235 * right_anti_dual.e1) + (this.e315 * right_anti_dual.e2) + (this.e125 * right_anti_dual.e3) + (this.e321 * right_anti_dual.e5))
        );
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (float3(other.e1234) * wedge.group3.xyz) - (float3(wedge.e1234) * other.group0.xyz), 
            /* e415, e425, e435, e321 */
            (wedge.group3.yzxw * float4(other.group0.zxy, other.e1234)) - (other.group0.yzxw * float4(wedge.group3.zxy, wedge.e1234)), 
            /* e235, e315, e125, e4 */
            float4(wedge.e4235 * other.e3215 * -1.0, wedge.e4315 * other.e3215 * -1.0, wedge.e4125 * other.e3215 * -1.0, (wedge.e42 * other.e4315) + (wedge.e43 * other.e4125) + (wedge.e45 * other.e1234)) + (other.group0.xyzx * float4(wedge.group3.www, wedge.e41)), 
            /* e1, e2, e3, e5 */
            float4((wedge.e12 * other.e4315) + (wedge.e15 * other.e1234), (wedge.e23 * other.e4125) + (wedge.e25 * other.e1234), (wedge.e31 * other.e4235) + (wedge.e35 * other.e1234), -(wedge.e25 * other.e4315) - (wedge.e35 * other.e4125)) - (float4(other.e3215) * float4(wedge.group0.xyz, wedge.e45)) - (other.group0.zxyx * float4(wedge.group1.yzx, wedge.e15))
        );
    }
}
public extension AntiFlector {
    public VersorEven project_orthogonally_onto(VersorEven other) {
        VersorOdd right_anti_dual = VersorOdd(
            /* e41, e42, e43, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e1234 */
            float4(other.group2.xyz, other.e4) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.group3.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0)
        );
        VersorEven wedge = VersorEven(
            /* e423, e431, e412, e12345 */
            (this.group1.zxyx * float4(right_anti_dual.group0.yzx, right_anti_dual.e4235)) + float4(float3(0.0), (this.e2 * right_anti_dual.e4315) + (this.e3 * right_anti_dual.e4125) + (this.e5 * right_anti_dual.e1234) - (this.e315 * right_anti_dual.e42) - (this.e125 * right_anti_dual.e43) - (this.e321 * right_anti_dual.e45)) - (right_anti_dual.group0.zxyx * float4(this.group1.yzx, this.e235)), 
            /* e415, e425, e435, e321 */
            (right_anti_dual.group0 * float4(this.group1.www, this.e321)) + float4(float3(0.0), -(this.e2 * right_anti_dual.e31) - (this.e3 * right_anti_dual.e12)) - (this.group1.xyzx * right_anti_dual.group1.wwwx), 
            /* e235, e315, e125, e5 */
            float4(((float3(this.e5) * right_anti_dual.group1.xyz) + (float3(right_anti_dual.scalar) * this.group0.xyz) + (this.group1.yzx * right_anti_dual.group2.zxy) - (this.group1.zxy * right_anti_dual.group2.yzx)), this.e5 * right_anti_dual.scalar), 
            /* e1, e2, e3, e4 */
            float4(float3(1.0), 0.0) * float4(right_anti_dual.group0.www, 0.0) * float4(this.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(((float3(other.e12345) * wedge.group0.xyz) + (float3(wedge.e12345) * other.group0.xyz)), other.e12345 * wedge.e12345), 
            /* e415, e425, e435, e321 */
            (float4(other.e12345) * wedge.group1) + (float4(wedge.e12345) * other.group1), 
            /* e235, e315, e125, e5 */
            (float4(other.e12345) * wedge.group2) + (float4(wedge.e12345) * other.group2) + float4(float3(0.0), -(other.e415 * wedge.e235) - (other.e425 * wedge.e315) - (other.e435 * wedge.e125) - (other.e235 * wedge.e415) - (other.e315 * wedge.e425) - (other.e125 * wedge.e435)), 
            /* e1, e2, e3, e4 */
            float4((other.e415 * wedge.e321) + (other.e321 * wedge.e415) + (other.e315 * wedge.e412) + (other.e1 * wedge.e12345), (other.e425 * wedge.e321) + (other.e321 * wedge.e425) + (other.e125 * wedge.e423) + (other.e2 * wedge.e12345), (other.e435 * wedge.e321) + (other.e321 * wedge.e435) + (other.e235 * wedge.e431) + (other.e3 * wedge.e12345), -(other.e412 * wedge.e435) - (other.e415 * wedge.e423) - (other.e425 * wedge.e431) - (other.e435 * wedge.e412)) + (other.group0.zxyw * float4(wedge.group2.yzx, wedge.e4)) + float4((other.group0.www * wedge.group3.xyz), other.e4 * wedge.e12345) - (other.group0.yzxx * float4(wedge.group2.zxy, wedge.e415)) - float4((other.group2.zxy * wedge.group0.yzx), other.e431 * wedge.e425)
        );
    }
}
public extension AntiFlector {
    public AntiDipoleInversion project_orthogonally_onto(VersorOdd other) {
        VersorEven right_anti_dual = VersorEven(
            /* e423, e431, e412, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            float4(other.group2.xyz, other.e3215) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4(other.group3.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0)
        );
        DipoleInversion wedge = DipoleInversion(
            /* e41, e42, e43 */
            float3(right_anti_dual.e4) * this.group1.xyz * float3(-1.0), 
            /* e23, e31, e12, e45 */
            float4(((this.group1.yzx * right_anti_dual.group3.zxy) - (this.group1.zxy * right_anti_dual.group3.yzx)), this.e5 * right_anti_dual.e4 * -1.0), 
            /* e15, e25, e35, e1234 */
            (this.group1.xyzx * float4(right_anti_dual.group2.www, right_anti_dual.e423)) + float4(float3(0.0), (this.e2 * right_anti_dual.e431) + (this.e3 * right_anti_dual.e412)) - (right_anti_dual.group3 * float4(this.group1.www, this.e321)), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), (this.e315 * right_anti_dual.e2) + (this.e125 * right_anti_dual.e3) + (this.e321 * right_anti_dual.e5) - (this.e5 * right_anti_dual.e321)) + float4((this.group1.zxy * right_anti_dual.group1.yzx), this.e235 * right_anti_dual.e1) - (this.group1.yzxy * float4(right_anti_dual.group1.zxy, right_anti_dual.e315)) - (this.group1.wwwz * float4(right_anti_dual.group0.xyz, right_anti_dual.e125)) - float4((right_anti_dual.group3.www * this.group0.xyz), this.e1 * right_anti_dual.e235)
        );
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (float3(other.e1234) * wedge.group3.xyz) - (float3(wedge.e1234) * other.group3.xyz), 
            /* e415, e425, e435, e321 */
            (wedge.group3.yzxw * float4(other.group3.zxy, other.e1234)) - (other.group3.yzxw * float4(wedge.group3.zxy, wedge.e1234)), 
            /* e235, e315, e125, e4 */
            (other.group3.xyzx * float4(wedge.group3.www, wedge.e41)) + float4(float3(0.0), (wedge.e42 * other.e4315) + (wedge.e43 * other.e4125) + (wedge.e45 * other.e1234) - (wedge.e4235 * other.e41) - (wedge.e4315 * other.e42) - (wedge.e4125 * other.e43)) - float4((other.group3.www * wedge.group3.xyz), wedge.e1234 * other.e45), 
            /* e1, e2, e3, e5 */
            (float4(wedge.e3215) * float4(other.group0.xyz, other.e45)) + (wedge.group3.zxyz * float4(other.group1.yzx, other.e35)) + (other.group2.wwwy * float4(wedge.group2.xyz, wedge.e4315)) + float4((wedge.group1.zxy * other.group3.yzx), wedge.e4235 * other.e15) - (float4(other.e3215) * float4(wedge.group0.xyz, wedge.e45)) - (wedge.group2.wwwy * float4(other.group2.xyz, other.e4315)) - (other.group3.zxyx * float4(wedge.group1.yzx, wedge.e15)) - float4((wedge.group3.yzx * other.group1.zxy), wedge.e35 * other.e4125)
        );
    }
}
public extension AntiLine {
    // Fancy infix trick (first half)
    public ProjectOrthogonallyOntoInfixPartial<AntiLine> operator /(ProjectOrthogonallyOntoInfix rhs) {
        return ProjectOrthogonallyOntoInfixPartial<AntiLine>(this);
    }
}
public extension ProjectOrthogonallyOntoInfixPartial<AntiLine> {
    // Fancy infix trick (second half)
    public AntiCircleRotor operator /(AntiCircleRotor rhs) {
        return this.a.project_orthogonally_onto(rhs);
    }
}
public extension AntiLine {
    public AntiCircleRotor project_orthogonally_onto(AntiCircleRotor other) {
        CircleRotor right_anti_dual = CircleRotor(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        AntiScalar wedge = AntiScalar(
            /* e12345 */
            -(this.e23 * right_anti_dual.e415) - (this.e31 * right_anti_dual.e425) - (this.e12 * right_anti_dual.e435) - (this.e15 * right_anti_dual.e423) - (this.e25 * right_anti_dual.e431) - (this.e35 * right_anti_dual.e412)
        );
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(wedge.e12345) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(wedge.e12345) * other.group1, 
            /* e15, e25, e35, scalar */
            float4(wedge.e12345) * other.group2
        );
    }
}
public extension AntiLine {
    public AntiCircleRotor project_orthogonally_onto(AntiDipoleInversion other) {
        DipoleInversion right_anti_dual = DipoleInversion(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e1234 */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group3 * float4(1.0, 1.0, 1.0, -1.0)
        );
        Sphere wedge = Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((this.e23 * right_anti_dual.e45) + (this.e35 * right_anti_dual.e42), (this.e31 * right_anti_dual.e45) + (this.e15 * right_anti_dual.e43), (this.e12 * right_anti_dual.e45) + (this.e25 * right_anti_dual.e41), -(this.e31 * right_anti_dual.e25) - (this.e12 * right_anti_dual.e35) - (this.e15 * right_anti_dual.e23) - (this.e25 * right_anti_dual.e31) - (this.e35 * right_anti_dual.e12)) - float4((this.group1.xyz.yzx * right_anti_dual.group0.xyz.zxy), this.e23 * right_anti_dual.e15), 
            /* e1234 */
            -(this.e23 * right_anti_dual.e41) - (this.e31 * right_anti_dual.e42) - (this.e12 * right_anti_dual.e43)
        );
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (float3(wedge.e1234) * other.group1.xyz) + (other.group0.xyz.zxy * wedge.group0.yzx) - (other.group0.xyz.yzx * wedge.group0.zxy), 
            /* e23, e31, e12, e45 */
            float4((other.e423 * wedge.e3215) + (other.e235 * wedge.e1234), (other.e431 * wedge.e3215) + (other.e315 * wedge.e1234), (other.e412 * wedge.e3215) + (other.e125 * wedge.e1234), -(other.e425 * wedge.e4315) - (other.e435 * wedge.e4125)) - (other.group1.wwwx * wedge.group0.xyzx), 
            /* e15, e25, e35, scalar */
            float4(other.e125 * wedge.e4315 * -1.0, other.e235 * wedge.e4125 * -1.0, other.e315 * wedge.e4235 * -1.0, (other.e2 * wedge.e4315) + (other.e3 * wedge.e4125) + (other.e5 * wedge.e1234)) + (float4(wedge.e3215) * float4(other.group1.xyz, other.e4)) + (wedge.group0.zxyx * float4(other.group2.yzx, other.e1))
        );
    }
}
public extension AntiLine {
    public AntiLine project_orthogonally_onto(AntiFlatPoint other) {
        FlatPoint right_anti_dual = FlatPoint(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        Plane wedge = Plane(
            /* e4235, e4315, e4125, e3215 */
            float4((float3(right_anti_dual.e45) * this.group0.xyz), -(this.e23 * right_anti_dual.e15) - (this.e31 * right_anti_dual.e25) - (this.e12 * right_anti_dual.e35))
        );
        return AntiLine(
            /* e23, e31, e12 */
            float3(other.e321) * wedge.group0.xyz * float3(-1.0), 
            /* e15, e25, e35 */
            (other.group0.yzx * wedge.group0.zxy) - (other.group0.zxy * wedge.group0.yzx)
        );
    }
}
public extension AntiLine {
    public AntiMotor project_orthogonally_onto(AntiFlector other) {
        Flector right_anti_dual = Flector(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        Plane wedge = Plane(
            /* e4235, e4315, e4125, e3215 */
            float4((float3(right_anti_dual.e45) * this.group0.xyz), -(this.e23 * right_anti_dual.e15) - (this.e31 * right_anti_dual.e25) - (this.e12 * right_anti_dual.e35))
        );
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((other.group0.www * wedge.group0.xyz * float3(-1.0)), (other.e1 * wedge.e4235) + (other.e2 * wedge.e4315) + (other.e3 * wedge.e4125)), 
            /* e15, e25, e35, e3215 */
            float4(((other.group0.yzx * wedge.group0.zxy) - (other.group0.zxy * wedge.group0.yzx)), 0.0)
        );
    }
}
public extension AntiLine {
    public AntiLine project_orthogonally_onto(AntiLine other) {
        Line right_anti_dual = Line(
            /* e415, e425, e435 */
            other.group0.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            other.group1.xyz * float3(-1.0)
        );
        AntiScalar wedge = AntiScalar(
            /* e12345 */
            -(this.e23 * right_anti_dual.e415) - (this.e31 * right_anti_dual.e425) - (this.e12 * right_anti_dual.e435)
        );
        return AntiLine(
            /* e23, e31, e12 */
            float3(wedge.e12345) * other.group0.xyz, 
            /* e15, e25, e35 */
            float3(wedge.e12345) * other.group1.xyz
        );
    }
}
public extension AntiLine {
    public AntiMotor project_orthogonally_onto(AntiMotor other) {
        Motor right_anti_dual = Motor(
            /* e415, e425, e435, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        Motor wedge = Motor(
            /* e415, e425, e435, e12345 */
            float4(float3(0.0), -(this.e23 * right_anti_dual.e415) - (this.e31 * right_anti_dual.e425) - (this.e12 * right_anti_dual.e435)), 
            /* e235, e315, e125, e5 */
            float4(float3(1.0), 0.0) * float4(this.group0.xyz, 0.0) * float4(right_anti_dual.group1.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((float3(wedge.e12345) * other.group0.xyz), (other.scalar * wedge.e12345) - (other.e23 * wedge.e415) - (other.e31 * wedge.e425) - (other.e12 * wedge.e435)), 
            /* e15, e25, e35, e3215 */
            float4(((float3(other.e3215) * wedge.group0.xyz) + (float3(wedge.e12345) * other.group1.xyz)), other.e3215 * wedge.e12345)
        );
    }
}
public extension AntiLine {
    public AntiLine project_orthogonally_onto(AntiScalar other) {
        Scalar right_anti_dual = Scalar(
            /* scalar */
            other.e12345 * -1.0
        );
        AntiLine wedge = AntiLine(
            /* e23, e31, e12 */
            float3(right_anti_dual.scalar) * this.group0.xyz, 
            /* e15, e25, e35 */
            float3(right_anti_dual.scalar) * this.group1.xyz
        );
        return AntiLine(
            /* e23, e31, e12 */
            float3(other.e12345) * wedge.group0.xyz, 
            /* e15, e25, e35 */
            float3(other.e12345) * wedge.group1.xyz
        );
    }
}
public extension AntiLine {
    public Dipole project_orthogonally_onto(Circle other) {
        Dipole right_anti_dual = Dipole(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35 */
            other.group2.xyz
        );
        Sphere wedge = Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((this.e23 * right_anti_dual.e45) + (this.e35 * right_anti_dual.e42), (this.e31 * right_anti_dual.e45) + (this.e15 * right_anti_dual.e43), (this.e12 * right_anti_dual.e45) + (this.e25 * right_anti_dual.e41), -(this.e31 * right_anti_dual.e25) - (this.e12 * right_anti_dual.e35) - (this.e15 * right_anti_dual.e23) - (this.e25 * right_anti_dual.e31) - (this.e35 * right_anti_dual.e12)) - float4((this.group1.xyz.yzx * right_anti_dual.group0.xyz.zxy), this.e23 * right_anti_dual.e15), 
            /* e1234 */
            -(this.e23 * right_anti_dual.e41) - (this.e31 * right_anti_dual.e42) - (this.e12 * right_anti_dual.e43)
        );
        return Dipole(
            /* e41, e42, e43 */
            (float3(wedge.e1234) * other.group1.xyz) + (other.group0.xyz.zxy * wedge.group0.yzx) - (other.group0.xyz.yzx * wedge.group0.zxy), 
            /* e23, e31, e12, e45 */
            float4((other.e423 * wedge.e3215) + (other.e235 * wedge.e1234), (other.e431 * wedge.e3215) + (other.e315 * wedge.e1234), (other.e412 * wedge.e3215) + (other.e125 * wedge.e1234), -(other.e425 * wedge.e4315) - (other.e435 * wedge.e4125)) - (other.group1.wwwx * wedge.group0.xyzx), 
            /* e15, e25, e35 */
            (float3(wedge.e3215) * other.group1.xyz) + (other.group2.xyz.yzx * wedge.group0.zxy) - (other.group2.xyz.zxy * wedge.group0.yzx)
        );
    }
}
public extension AntiLine {
    public VersorOdd project_orthogonally_onto(CircleRotor other) {
        AntiCircleRotor right_anti_dual = AntiCircleRotor(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        DipoleInversion wedge = DipoleInversion(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12, e45 */
            float4(float3(1.0), 0.0) * float4(this.group0.xyz, 0.0) * float4(right_anti_dual.group2.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e15, e25, e35, e1234 */
            float4((float3(right_anti_dual.scalar) * this.group1.xyz), -(right_anti_dual.e41 * this.e23) - (right_anti_dual.e42 * this.e31) - (right_anti_dual.e43 * this.e12)), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual.e42 * this.e35) + (right_anti_dual.e45 * this.e23), (right_anti_dual.e43 * this.e15) + (right_anti_dual.e45 * this.e31), (right_anti_dual.e41 * this.e25) + (right_anti_dual.e45 * this.e12), -(right_anti_dual.e23 * this.e15) - (right_anti_dual.e31 * this.e25) - (right_anti_dual.e12 * this.e35) - (right_anti_dual.e25 * this.e31) - (right_anti_dual.e35 * this.e12)) - float4((right_anti_dual.group0.xyz.zxy * this.group1.xyz.yzx), right_anti_dual.e15 * this.e23)
        );
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4((other.e412 * wedge.e4315) + (other.e415 * wedge.e1234) + (other.e12345 * wedge.e41), (other.e423 * wedge.e4125) + (other.e425 * wedge.e1234) + (other.e12345 * wedge.e42), (other.e431 * wedge.e4235) + (other.e435 * wedge.e1234) + (other.e12345 * wedge.e43), -(other.e431 * wedge.e25) - (other.e412 * wedge.e35) - (other.e415 * wedge.e23) - (other.e425 * wedge.e31) - (other.e435 * wedge.e12) - (other.e321 * wedge.e45) - (other.e235 * wedge.e41) - (other.e315 * wedge.e42) - (other.e125 * wedge.e43)) - float4((other.group0.xyz.yzx * wedge.group3.zxy), other.e423 * wedge.e15), 
            /* e23, e31, e12, e45 */
            float4((other.e235 * wedge.e1234) + (other.e12345 * wedge.e23), (other.e315 * wedge.e1234) + (other.e12345 * wedge.e31), (other.e125 * wedge.e1234) + (other.e12345 * wedge.e12), -(other.e425 * wedge.e4315) - (other.e435 * wedge.e4125)) + float4((other.group0.xyz * wedge.group3.www), other.e12345 * wedge.e45) - (other.group1.wwwx * wedge.group3.xyzx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(other.e12345) * wedge.group2.xyz) + (float3(wedge.e3215) * other.group1.xyz) + (other.group2.yzx * wedge.group3.zxy) - (other.group2.zxy * wedge.group3.yzx)), other.e12345 * wedge.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345) * wedge.group3
        );
    }
}
public extension AntiLine {
    public Dipole project_orthogonally_onto(Dipole other) {
        Circle right_anti_dual = Circle(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125 */
            other.group2.xyz * float3(-1.0)
        );
        AntiScalar wedge = AntiScalar(
            /* e12345 */
            -(this.e23 * right_anti_dual.e415) - (this.e31 * right_anti_dual.e425) - (this.e12 * right_anti_dual.e435) - (this.e15 * right_anti_dual.e423) - (this.e25 * right_anti_dual.e431) - (this.e35 * right_anti_dual.e412)
        );
        return Dipole(
            /* e41, e42, e43 */
            float3(wedge.e12345) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(wedge.e12345) * other.group1, 
            /* e15, e25, e35 */
            float3(wedge.e12345) * other.group2.xyz
        );
    }
}
public extension AntiLine {
    public VersorOdd project_orthogonally_onto(DipoleInversion other) {
        AntiDipoleInversion right_anti_dual = AntiDipoleInversion(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e4 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group3 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        CircleRotor wedge = CircleRotor(
            /* e423, e431, e412 */
            float3(right_anti_dual.e4) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4((float3(right_anti_dual.e4) * this.group1.xyz), -(right_anti_dual.e1 * this.e23) - (right_anti_dual.e2 * this.e31) - (right_anti_dual.e3 * this.e12)), 
            /* e235, e315, e125, e12345 */
            float4((right_anti_dual.e2 * this.e35) + (right_anti_dual.e5 * this.e23), (right_anti_dual.e3 * this.e15) + (right_anti_dual.e5 * this.e31), (right_anti_dual.e1 * this.e25) + (right_anti_dual.e5 * this.e12), -(right_anti_dual.e431 * this.e25) - (right_anti_dual.e412 * this.e35) - (right_anti_dual.e415 * this.e23) - (right_anti_dual.e425 * this.e31) - (right_anti_dual.e435 * this.e12)) - float4((this.group1.xyz.yzx * right_anti_dual.group3.zxy), right_anti_dual.e423 * this.e15)
        );
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4((wedge.e412 * other.e4315) + (wedge.e415 * other.e1234) + (wedge.e12345 * other.e41), (wedge.e423 * other.e4125) + (wedge.e425 * other.e1234) + (wedge.e12345 * other.e42), (wedge.e431 * other.e4235) + (wedge.e435 * other.e1234) + (wedge.e12345 * other.e43), -(wedge.e431 * other.e25) - (wedge.e412 * other.e35) - (wedge.e415 * other.e23) - (wedge.e425 * other.e31) - (wedge.e435 * other.e12) - (wedge.e321 * other.e45) - (wedge.e235 * other.e41) - (wedge.e315 * other.e42) - (wedge.e125 * other.e43)) - float4((wedge.group0.xyz.yzx * other.group3.zxy), wedge.e423 * other.e15), 
            /* e23, e31, e12, e45 */
            float4((wedge.e235 * other.e1234) + (wedge.e12345 * other.e23), (wedge.e315 * other.e1234) + (wedge.e12345 * other.e31), (wedge.e125 * other.e1234) + (wedge.e12345 * other.e12), -(wedge.e425 * other.e4315) - (wedge.e435 * other.e4125)) + float4((wedge.group0.xyz * other.group3.www), wedge.e12345 * other.e45) - (wedge.group1.wwwx * other.group3.xyzx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(wedge.e12345) * other.group2.xyz) + (float3(other.e3215) * wedge.group1.xyz) + (wedge.group2.yzx * other.group3.zxy) - (wedge.group2.zxy * other.group3.yzx)), wedge.e12345 * other.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(wedge.e12345) * other.group3
        );
    }
}
public extension AntiLine {
    public AntiLine project_orthogonally_onto(DualNum other) {
        AntiDualNum right_anti_dual = AntiDualNum(
            /* e3215, scalar */
            other.group0.xy * float2(-1.0)
        );
        AntiLine wedge = AntiLine(
            /* e23, e31, e12 */
            float3(right_anti_dual.scalar) * this.group0.xyz, 
            /* e15, e25, e35 */
            float3(right_anti_dual.scalar) * this.group1.xyz
        );
        return AntiLine(
            /* e23, e31, e12 */
            float3(other.e12345) * wedge.group0.xyz, 
            /* e15, e25, e35 */
            float3(other.e12345) * wedge.group1.xyz
        );
    }
}
public extension AntiLine {
    public AntiMotor project_orthogonally_onto(Flector other) {
        AntiFlector right_anti_dual = AntiFlector(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        AntiFlatPoint wedge = AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4((right_anti_dual.e2 * this.e35) + (right_anti_dual.e5 * this.e23), (right_anti_dual.e3 * this.e15) + (right_anti_dual.e5 * this.e31), (right_anti_dual.e1 * this.e25) + (right_anti_dual.e5 * this.e12), -(right_anti_dual.e2 * this.e31) - (right_anti_dual.e3 * this.e12)) - (right_anti_dual.group1.zxyx * float4(this.group1.xyz.yzx, this.e23))
        );
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(wedge.e321) * float4(other.group1.xyz, other.e45) * float4(-1.0), 
            /* e15, e25, e35, e3215 */
            float4(((wedge.group0.yzx * other.group1.zxy) - (wedge.group0.zxy * other.group1.yzx)), 0.0)
        );
    }
}
public extension AntiLine {
    public FlatPoint project_orthogonally_onto(Line other) {
        AntiLine right_anti_dual = AntiLine(
            /* e23, e31, e12 */
            other.group0.xyz, 
            /* e15, e25, e35 */
            other.group1.xyz
        );
        AntiDualNum wedge = AntiDualNum(
            /* e3215, scalar */
            float2(-(right_anti_dual.e23 * this.e15) - (right_anti_dual.e31 * this.e25) - (right_anti_dual.e12 * this.e35) - (right_anti_dual.e15 * this.e23) - (right_anti_dual.e25 * this.e31) - (right_anti_dual.e35 * this.e12), 0.0)
        );
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(wedge.group0.xy.xx, wedge.e3215, 0.0) * float4(float3(1.0), 0.0) * float4(other.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
public extension AntiLine {
    public AntiMotor project_orthogonally_onto(Motor other) {
        AntiMotor right_anti_dual = AntiMotor(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        AntiMotor wedge = AntiMotor(
            /* e23, e31, e12, scalar */
            float4(float3(1.0), 0.0) * float4(this.group0.xyz, 0.0) * float4(right_anti_dual.group0.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e15, e25, e35, e3215 */
            float4((float3(right_anti_dual.scalar) * this.group1.xyz), -(this.e23 * right_anti_dual.e15) - (this.e31 * right_anti_dual.e25) - (this.e12 * right_anti_dual.e35) - (this.e15 * right_anti_dual.e23) - (this.e25 * right_anti_dual.e31) - (this.e35 * right_anti_dual.e12))
        );
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((float3(other.e12345) * wedge.group0.xyz), (wedge.scalar * other.e12345) - (wedge.e23 * other.e415) - (wedge.e31 * other.e425) - (wedge.e12 * other.e435)), 
            /* e15, e25, e35, e3215 */
            float4(((float3(wedge.e3215) * other.group0.xyz) + (float3(other.e12345) * wedge.group1.xyz)), wedge.e3215 * other.e12345)
        );
    }
}
public extension AntiLine {
    public MultiVector project_orthogonally_onto(MultiVector other) {
        MultiVector right_anti_dual = MultiVector(
            /* scalar, e12345 */
            other.group0.xy.yx * float2(-1.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4(other.group9.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5 */
            other.e3215, 
            /* e15, e25, e35, e45 */
            float4(other.group8.xyz, other.e321 * -1.0), 
            /* e41, e42, e43 */
            other.group7.xyz, 
            /* e23, e31, e12 */
            other.group6.xyz, 
            /* e415, e425, e435, e321 */
            float4(other.group5.xyz, other.e45) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e423, e431, e412 */
            other.group4.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            other.group3.xyz * float3(-1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.group1.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234 */
            other.e4 * -1.0
        );
        MultiVector wedge = MultiVector(
            /* scalar, e12345 */
            float2(0.0, -(this.e23 * right_anti_dual.e415) - (this.e31 * right_anti_dual.e425) - (this.e12 * right_anti_dual.e435) - (this.e15 * right_anti_dual.e423) - (this.e25 * right_anti_dual.e431) - (this.e35 * right_anti_dual.e412)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            float4(right_anti_dual.group0.xy.xx, right_anti_dual.scalar, 0.0) * float4(float3(1.0), 0.0) * float4(this.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(right_anti_dual.scalar) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4((float3(right_anti_dual.e4) * this.group1.xyz), -(this.e23 * right_anti_dual.e1) - (this.e31 * right_anti_dual.e2) - (this.e12 * right_anti_dual.e3)), 
            /* e423, e431, e412 */
            float3(right_anti_dual.e4) * this.group0.xyz, 
            /* e235, e315, e125 */
            (float3(right_anti_dual.e5) * this.group0.xyz) + (this.group1.xyz.zxy * right_anti_dual.group1.yzx) - (this.group1.xyz.yzx * right_anti_dual.group1.zxy), 
            /* e4235, e4315, e4125, e3215 */
            float4((this.e23 * right_anti_dual.e45) + (this.e35 * right_anti_dual.e42), (this.e31 * right_anti_dual.e45) + (this.e15 * right_anti_dual.e43), (this.e12 * right_anti_dual.e45) + (this.e25 * right_anti_dual.e41), -(this.e31 * right_anti_dual.e25) - (this.e12 * right_anti_dual.e35) - (this.e15 * right_anti_dual.e23) - (this.e25 * right_anti_dual.e31) - (this.e35 * right_anti_dual.e12)) - float4((this.group1.xyz.yzx * right_anti_dual.group4.xyz.zxy), this.e23 * right_anti_dual.e15), 
            /* e1234 */
            -(this.e23 * right_anti_dual.e41) - (this.e31 * right_anti_dual.e42) - (this.e12 * right_anti_dual.e43)
        );
        return MultiVector(
            /* scalar, e12345 */
            float2((other.scalar * wedge.e12345) + (other.e12345 * wedge.scalar) + (other.e1 * wedge.e4235) + (other.e2 * wedge.e4315) + (other.e3 * wedge.e4125) + (other.e4 * wedge.e3215) + (other.e5 * wedge.e1234) + (other.e4235 * wedge.e1) + (other.e4315 * wedge.e2) + (other.e4125 * wedge.e3) + (other.e3215 * wedge.e4) + (other.e1234 * wedge.e5) - (other.e15 * wedge.e423) - (other.e25 * wedge.e431) - (other.e35 * wedge.e412) - (other.e45 * wedge.e321) - (other.e41 * wedge.e235) - (other.e42 * wedge.e315) - (other.e43 * wedge.e125) - (other.e23 * wedge.e415) - (other.e31 * wedge.e425) - (other.e12 * wedge.e435) - (other.e415 * wedge.e23) - (other.e425 * wedge.e31) - (other.e435 * wedge.e12) - (other.e321 * wedge.e45) - (other.e423 * wedge.e15) - (other.e431 * wedge.e25) - (other.e412 * wedge.e35) - (other.e235 * wedge.e41) - (other.e315 * wedge.e42) - (other.e125 * wedge.e43), other.e12345 * wedge.e12345), 
            /* e1, e2, e3, e4 */
            float4((other.e415 * wedge.e321) + (other.e321 * wedge.e415) + (other.e4315 * wedge.e12) + (other.e1234 * wedge.e15), (other.e425 * wedge.e321) + (other.e321 * wedge.e425) + (other.e4125 * wedge.e23) + (other.e1234 * wedge.e25), (other.e435 * wedge.e321) + (other.e321 * wedge.e435) + (other.e4235 * wedge.e31) + (other.e1234 * wedge.e35), -(other.e45 * wedge.e1234) - (other.e415 * wedge.e423) - (other.e425 * wedge.e431) - (other.e435 * wedge.e412)) + (float4(other.e12345) * wedge.group1) + (float4(wedge.e12345) * other.group1) + float4((other.group4.xyz * wedge.group9.www), other.e4235 * wedge.e41) + float4((other.group5.xyz.yzx * wedge.group9.zxy), other.e4315 * wedge.e42) + float4((other.group7.xyz.zxy * wedge.group8.xyz.yzx), other.e4125 * wedge.e43) + float4((other.group8.xyz.yzx * wedge.group7.xyz.zxy), other.e1234 * wedge.e45) - (wedge.group9.yzxx * float4(other.group5.xyz.zxy, other.e41)) - float4((float3(wedge.e1234) * other.group3.xyz), other.e412 * wedge.e435) - float4((wedge.group4.xyz * other.group9.www), other.e423 * wedge.e415) - float4((other.group7.xyz.yzx * wedge.group8.xyz.zxy), other.e42 * wedge.e4315) - float4((other.group8.xyz.zxy * wedge.group7.xyz.yzx), other.e43 * wedge.e4125) - float4((wedge.group5.xyz.yzx * other.group9.zxy), other.e431 * wedge.e425), 
            /* e5 */
            (other.e12345 * wedge.e5) + (other.e5 * wedge.e12345) + (other.e15 * wedge.e4235) + (other.e25 * wedge.e4315) + (other.e35 * wedge.e4125) + (other.e45 * wedge.e3215) - (other.e415 * wedge.e235) - (other.e425 * wedge.e315) - (other.e435 * wedge.e125) - (other.e235 * wedge.e415) - (other.e315 * wedge.e425) - (other.e125 * wedge.e435) - (other.e4235 * wedge.e15) - (other.e4315 * wedge.e25) - (other.e4125 * wedge.e35) - (other.e3215 * wedge.e45), 
            /* e15, e25, e35, e45 */
            float4((other.e415 * wedge.e3215) + (other.e315 * wedge.e4125) + (other.e4125 * wedge.e315) + (other.e3215 * wedge.e415), (other.e425 * wedge.e3215) + (other.e125 * wedge.e4235) + (other.e4235 * wedge.e125) + (other.e3215 * wedge.e425), (other.e435 * wedge.e3215) + (other.e235 * wedge.e4315) + (other.e4315 * wedge.e235) + (other.e3215 * wedge.e435), -(other.e435 * wedge.e4125) - (other.e4235 * wedge.e415) - (other.e4315 * wedge.e425) - (other.e4125 * wedge.e435)) + (float4(other.e12345) * wedge.group3) + (float4(wedge.e12345) * other.group3) - (wedge.group9.yzxx * float4(other.group8.xyz.zxy, other.e415)) - float4((wedge.group8.xyz.zxy * other.group9.yzx), other.e425 * wedge.e4315), 
            /* e41, e42, e43 */
            (float3(other.e12345) * wedge.group4.xyz) + (float3(other.e1234) * wedge.group6.xyz) + (float3(wedge.e12345) * other.group4.xyz) + (float3(wedge.e1234) * other.group6.xyz) + (other.group7.xyz.zxy * wedge.group9.yzx) + (wedge.group7.xyz.zxy * other.group9.yzx) - (other.group7.xyz.yzx * wedge.group9.zxy) - (wedge.group7.xyz.yzx * other.group9.zxy), 
            /* e23, e31, e12 */
            (float3(other.e12345) * wedge.group5.xyz) + (float3(other.e3215) * wedge.group7.xyz) + (float3(other.e1234) * wedge.group8.xyz) + (float3(wedge.e12345) * other.group5.xyz) + (float3(wedge.e3215) * other.group7.xyz) + (float3(wedge.e1234) * other.group8.xyz) - (float3(other.e321) * wedge.group9.xyz) - (float3(wedge.e321) * other.group9.xyz), 
            /* e415, e425, e435, e321 */
            (float4(other.e12345) * wedge.group6) + (float4(wedge.e12345) * other.group6) + float4((other.group9.zxy * wedge.group9.yzx), other.e1234 * wedge.e3215) - (other.group9.yzxw * float4(wedge.group9.zxy, wedge.e1234)), 
            /* e423, e431, e412 */
            (float3(other.e12345) * wedge.group7.xyz) + (float3(other.e1234) * wedge.group9.xyz) + (float3(wedge.e12345) * other.group7.xyz) - (float3(wedge.e1234) * other.group9.xyz), 
            /* e235, e315, e125 */
            (float3(other.e12345) * wedge.group8.xyz) + (float3(wedge.e12345) * other.group8.xyz) + (float3(wedge.e3215) * other.group9.xyz) - (float3(other.e3215) * wedge.group9.xyz), 
            /* e4235, e4315, e4125, e3215 */
            (float4(other.e12345) * wedge.group9) + (float4(wedge.e12345) * other.group9), 
            /* e1234 */
            (other.e12345 * wedge.e1234) + (other.e1234 * wedge.e12345)
        );
    }
}
public extension AntiLine {
    public AntiLine project_orthogonally_onto(Plane other) {
        AntiPlane right_anti_dual = AntiPlane(
            /* e1, e2, e3, e5 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        AntiFlatPoint wedge = AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4((this.e23 * right_anti_dual.e5) + (this.e35 * right_anti_dual.e2), (this.e31 * right_anti_dual.e5) + (this.e15 * right_anti_dual.e3), (this.e12 * right_anti_dual.e5) + (this.e25 * right_anti_dual.e1), -(this.e31 * right_anti_dual.e2) - (this.e12 * right_anti_dual.e3)) - (right_anti_dual.group0.zxyx * float4(this.group1.xyz.yzx, this.e23))
        );
        return AntiLine(
            /* e23, e31, e12 */
            float3(wedge.e321) * other.group0.xyz * float3(-1.0), 
            /* e15, e25, e35 */
            (wedge.group0.yzx * other.group0.zxy) - (wedge.group0.zxy * other.group0.yzx)
        );
    }
}
public extension AntiLine {
    public Dipole project_orthogonally_onto(Sphere other) {
        RoundPoint right_anti_dual = RoundPoint(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5 */
            other.e3215
        );
        Circle wedge = Circle(
            /* e423, e431, e412 */
            float3(right_anti_dual.e4) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4((float3(right_anti_dual.e4) * this.group1.xyz), -(this.e23 * right_anti_dual.e1) - (this.e31 * right_anti_dual.e2) - (this.e12 * right_anti_dual.e3)), 
            /* e235, e315, e125 */
            (float3(right_anti_dual.e5) * this.group0.xyz) + (this.group1.xyz.zxy * right_anti_dual.group0.yzx) - (this.group1.xyz.yzx * right_anti_dual.group0.zxy)
        );
        return Dipole(
            /* e41, e42, e43 */
            (float3(other.e1234) * wedge.group1.xyz) + (wedge.group0.xyz.zxy * other.group0.yzx) - (wedge.group0.xyz.yzx * other.group0.zxy), 
            /* e23, e31, e12, e45 */
            float4((wedge.e423 * other.e3215) + (wedge.e235 * other.e1234), (wedge.e431 * other.e3215) + (wedge.e315 * other.e1234), (wedge.e412 * other.e3215) + (wedge.e125 * other.e1234), -(wedge.e425 * other.e4315) - (wedge.e435 * other.e4125)) - (wedge.group1.wwwx * other.group0.xyzx), 
            /* e15, e25, e35 */
            (float3(other.e3215) * wedge.group1.xyz) + (wedge.group2.xyz.yzx * other.group0.zxy) - (wedge.group2.xyz.zxy * other.group0.yzx)
        );
    }
}
public extension AntiLine {
    public VersorOdd project_orthogonally_onto(VersorEven other) {
        VersorOdd right_anti_dual = VersorOdd(
            /* e41, e42, e43, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e1234 */
            float4(other.group2.xyz, other.e4) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.group3.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0)
        );
        DipoleInversion wedge = DipoleInversion(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12, e45 */
            float4(float3(1.0), 0.0) * float4(this.group0.xyz, 0.0) * float4(right_anti_dual.group0.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e15, e25, e35, e1234 */
            float4((float3(right_anti_dual.scalar) * this.group1.xyz), -(this.e23 * right_anti_dual.e41) - (this.e31 * right_anti_dual.e42) - (this.e12 * right_anti_dual.e43)), 
            /* e4235, e4315, e4125, e3215 */
            float4((this.e23 * right_anti_dual.e45) + (this.e35 * right_anti_dual.e42), (this.e31 * right_anti_dual.e45) + (this.e15 * right_anti_dual.e43), (this.e12 * right_anti_dual.e45) + (this.e25 * right_anti_dual.e41), -(this.e31 * right_anti_dual.e25) - (this.e12 * right_anti_dual.e35) - (this.e15 * right_anti_dual.e23) - (this.e25 * right_anti_dual.e31) - (this.e35 * right_anti_dual.e12)) - float4((this.group1.xyz.yzx * right_anti_dual.group0.zxy), this.e23 * right_anti_dual.e15)
        );
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (wedge.group3.yzxy * float4(other.group0.zxy, other.e2)) + float4(float3(0.0), (wedge.e4125 * other.e3) + (wedge.e3215 * other.e4) - (wedge.e42 * other.e315) - (wedge.e43 * other.e125) - (wedge.e23 * other.e415) - (wedge.e31 * other.e425) - (wedge.e12 * other.e435) - (wedge.e45 * other.e321) - (wedge.e15 * other.e423) - (wedge.e25 * other.e431) - (wedge.e35 * other.e412)) + float4((wedge.group0.xyz * other.group0.www), wedge.e1234 * other.e5) + float4((wedge.group2.www * other.group1.xyz), wedge.e4235 * other.e1) - float4((wedge.group3.zxy * other.group0.yzx), wedge.e41 * other.e235), 
            /* e23, e31, e12, e45 */
            float4((wedge.e1234 * other.e235) + (wedge.e3215 * other.e423), (wedge.e1234 * other.e315) + (wedge.e3215 * other.e431), (wedge.e1234 * other.e125) + (wedge.e3215 * other.e412), -(wedge.e4315 * other.e425) - (wedge.e4125 * other.e435)) + (float4(other.e12345) * wedge.group1) - (wedge.group3.xyzx * other.group1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(wedge.e3215) * other.group1.xyz) + (float3(other.e12345) * wedge.group2.xyz) + (wedge.group3.zxy * other.group2.yzx) - (wedge.group3.yzx * other.group2.zxy)), wedge.e1234 * other.e12345), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345) * wedge.group3
        );
    }
}
public extension AntiLine {
    public VersorOdd project_orthogonally_onto(VersorOdd other) {
        VersorEven right_anti_dual = VersorEven(
            /* e423, e431, e412, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            float4(other.group2.xyz, other.e3215) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4(other.group3.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0)
        );
        CircleRotor wedge = CircleRotor(
            /* e423, e431, e412 */
            float3(right_anti_dual.e4) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4((float3(right_anti_dual.e4) * this.group1.xyz), -(this.e23 * right_anti_dual.e1) - (this.e31 * right_anti_dual.e2) - (this.e12 * right_anti_dual.e3)), 
            /* e235, e315, e125, e12345 */
            float4((this.e23 * right_anti_dual.e5) + (this.e35 * right_anti_dual.e2), (this.e31 * right_anti_dual.e5) + (this.e15 * right_anti_dual.e3), (this.e12 * right_anti_dual.e5) + (this.e25 * right_anti_dual.e1), -(this.e31 * right_anti_dual.e425) - (this.e12 * right_anti_dual.e435) - (this.e15 * right_anti_dual.e423) - (this.e25 * right_anti_dual.e431) - (this.e35 * right_anti_dual.e412)) - float4((this.group1.xyz.yzx * right_anti_dual.group3.zxy), this.e23 * right_anti_dual.e415)
        );
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4((wedge.e415 * other.e1234) + (wedge.e12345 * other.e41), (wedge.e425 * other.e1234) + (wedge.e12345 * other.e42), (wedge.e435 * other.e1234) + (wedge.e12345 * other.e43), -(wedge.e431 * other.e25) - (wedge.e412 * other.e35) - (wedge.e415 * other.e23) - (wedge.e425 * other.e31) - (wedge.e435 * other.e12) - (wedge.e321 * other.e45) - (wedge.e235 * other.e41) - (wedge.e315 * other.e42) - (wedge.e125 * other.e43)) + float4((wedge.group0.xyz.zxy * other.group3.yzx), wedge.e12345 * other.scalar) - float4((wedge.group0.xyz.yzx * other.group3.zxy), wedge.e423 * other.e15), 
            /* e23, e31, e12, e45 */
            float4((wedge.e235 * other.e1234) + (wedge.e12345 * other.e23), (wedge.e315 * other.e1234) + (wedge.e12345 * other.e31), (wedge.e125 * other.e1234) + (wedge.e12345 * other.e12), -(wedge.e425 * other.e4315) - (wedge.e435 * other.e4125)) + float4((wedge.group0.xyz * other.group3.www), wedge.e12345 * other.e45) - (wedge.group1.wwwx * other.group3.xyzx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(wedge.e12345) * other.group2.xyz) + (float3(other.e3215) * wedge.group1.xyz) + (wedge.group2.yzx * other.group3.zxy) - (wedge.group2.zxy * other.group3.yzx)), wedge.e12345 * other.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(wedge.e12345) * other.group3
        );
    }
}
public extension AntiMotor {
    // Fancy infix trick (first half)
    public ProjectOrthogonallyOntoInfixPartial<AntiMotor> operator /(ProjectOrthogonallyOntoInfix rhs) {
        return ProjectOrthogonallyOntoInfixPartial<AntiMotor>(this);
    }
}
public extension ProjectOrthogonallyOntoInfixPartial<AntiMotor> {
    // Fancy infix trick (second half)
    public AntiCircleRotor operator /(AntiCircleRotor rhs) {
        return this.a.project_orthogonally_onto(rhs);
    }
}
public extension AntiMotor {
    public AntiCircleRotor project_orthogonally_onto(AntiCircleRotor other) {
        CircleRotor right_anti_dual = CircleRotor(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        CircleRotor wedge = CircleRotor(
            /* e423, e431, e412 */
            float3(this.scalar) * right_anti_dual.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(this.scalar) * right_anti_dual.group1, 
            /* e235, e315, e125, e12345 */
            float4((this.group0.www * right_anti_dual.group2.xyz), (this.scalar * right_anti_dual.e12345) - (this.e23 * right_anti_dual.e415) - (this.e31 * right_anti_dual.e425) - (this.e12 * right_anti_dual.e435) - (this.e15 * right_anti_dual.e423) - (this.e25 * right_anti_dual.e431) - (this.e35 * right_anti_dual.e412))
        );
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(wedge.e12345) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(wedge.e12345) * other.group1, 
            /* e15, e25, e35, scalar */
            float4((float3(wedge.e12345) * other.group2.xyz), (other.scalar * wedge.e12345) - (other.e41 * wedge.e235) - (other.e42 * wedge.e315) - (other.e43 * wedge.e125) - (other.e23 * wedge.e415) - (other.e31 * wedge.e425) - (other.e12 * wedge.e435) - (other.e45 * wedge.e321) - (other.e15 * wedge.e423) - (other.e25 * wedge.e431) - (other.e35 * wedge.e412))
        );
    }
}
public extension AntiMotor {
    public AntiCircleRotor project_orthogonally_onto(AntiDipoleInversion other) {
        DipoleInversion right_anti_dual = DipoleInversion(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e1234 */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group3 * float4(1.0, 1.0, 1.0, -1.0)
        );
        DipoleInversion wedge = DipoleInversion(
            /* e41, e42, e43 */
            float3(this.scalar) * right_anti_dual.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(this.scalar) * right_anti_dual.group1, 
            /* e15, e25, e35, e1234 */
            float4((this.group0.www * right_anti_dual.group2.xyz), (this.scalar * right_anti_dual.e1234) - (this.e23 * right_anti_dual.e41) - (this.e31 * right_anti_dual.e42) - (this.e12 * right_anti_dual.e43)), 
            /* e4235, e4315, e4125, e3215 */
            float4((this.e23 * right_anti_dual.e45) + (this.scalar * right_anti_dual.e4235), (this.e31 * right_anti_dual.e45) + (this.scalar * right_anti_dual.e4315), (this.e12 * right_anti_dual.e45) + (this.scalar * right_anti_dual.e4125), -(this.e31 * right_anti_dual.e25) - (this.e12 * right_anti_dual.e35) - (this.e15 * right_anti_dual.e23) - (this.e25 * right_anti_dual.e31) - (this.e35 * right_anti_dual.e12)) + float4((right_anti_dual.group0.xyz.yzx * this.group1.zxy), this.scalar * right_anti_dual.e3215) - float4((right_anti_dual.group0.xyz.zxy * this.group1.yzx), this.e23 * right_anti_dual.e15)
        );
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (float3(wedge.e1234) * other.group1.xyz) + (other.group0.xyz.zxy * wedge.group3.yzx) - (other.group0.xyz.yzx * wedge.group3.zxy), 
            /* e23, e31, e12, e45 */
            float4((other.e423 * wedge.e3215) + (other.e235 * wedge.e1234), (other.e431 * wedge.e3215) + (other.e315 * wedge.e1234), (other.e412 * wedge.e3215) + (other.e125 * wedge.e1234), -(other.e425 * wedge.e4315) - (other.e435 * wedge.e4125)) - (other.group1.wwwx * wedge.group3.xyzx), 
            /* e15, e25, e35, scalar */
            (float4(wedge.e3215) * float4(other.group1.xyz, other.e4)) + (wedge.group3.zxyx * float4(other.group2.yzx, other.e1)) + float4(float3(0.0), (other.e2 * wedge.e4315) + (other.e3 * wedge.e4125) + (other.e5 * wedge.e1234) - (other.e431 * wedge.e25) - (other.e412 * wedge.e35) - (other.e415 * wedge.e23) - (other.e425 * wedge.e31) - (other.e435 * wedge.e12) - (other.e321 * wedge.e45) - (other.e235 * wedge.e41) - (other.e315 * wedge.e42) - (other.e125 * wedge.e43)) - float4((other.group2.zxy * wedge.group3.yzx), other.e423 * wedge.e15)
        );
    }
}
public extension AntiMotor {
    public AntiMotor project_orthogonally_onto(AntiDualNum other) {
        DualNum right_anti_dual = DualNum(
            /* e5, e12345 */
            other.group0.xy
        );
        Motor wedge = Motor(
            /* e415, e425, e435, e12345 */
            float4(float3(0.0), this.scalar * right_anti_dual.e12345), 
            /* e235, e315, e125, e5 */
            float4(right_anti_dual.e5) * this.group0
        );
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(float3(0.0), other.scalar * wedge.e12345), 
            /* e15, e25, e35, e3215 */
            float4(other.e3215) * wedge.group0
        );
    }
}
public extension AntiMotor {
    public AntiMotor project_orthogonally_onto(AntiFlatPoint other) {
        FlatPoint right_anti_dual = FlatPoint(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        Flector wedge = Flector(
            /* e15, e25, e35, e45 */
            float4(this.scalar) * right_anti_dual.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4((float3(right_anti_dual.e45) * this.group0.xyz), -(this.e23 * right_anti_dual.e15) - (this.e31 * right_anti_dual.e25) - (this.e12 * right_anti_dual.e35))
        );
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(other.e321) * float4(wedge.group1.xyz, wedge.e45) * float4(-1.0), 
            /* e15, e25, e35, e3215 */
            float4(((other.group0.yzx * wedge.group1.zxy) - (other.group0.zxy * wedge.group1.yzx)), 0.0)
        );
    }
}
public extension AntiMotor {
    public AntiMotor project_orthogonally_onto(AntiFlector other) {
        Flector right_anti_dual = Flector(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        Flector wedge = Flector(
            /* e15, e25, e35, e45 */
            float4(this.scalar) * right_anti_dual.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar * right_anti_dual.e4235, this.scalar * right_anti_dual.e4315, this.scalar * right_anti_dual.e4125, -(this.e23 * right_anti_dual.e15) - (this.e31 * right_anti_dual.e25) - (this.e12 * right_anti_dual.e35)) + (this.group0 * float4(right_anti_dual.group0.www, right_anti_dual.e3215))
        );
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((other.group0.www * wedge.group1.xyz * float3(-1.0)), (other.e1 * wedge.e4235) + (other.e2 * wedge.e4315) + (other.e3 * wedge.e4125) - (other.e321 * wedge.e45)), 
            /* e15, e25, e35, e3215 */
            float4(((other.group0.yzx * wedge.group1.zxy) - (other.group0.zxy * wedge.group1.yzx)), 0.0)
        );
    }
}
public extension AntiMotor {
    public AntiMotor project_orthogonally_onto(AntiLine other) {
        Line right_anti_dual = Line(
            /* e415, e425, e435 */
            other.group0.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            other.group1.xyz * float3(-1.0)
        );
        Motor wedge = Motor(
            /* e415, e425, e435, e12345 */
            float4((float3(this.scalar) * right_anti_dual.group0.xyz), -(this.e23 * right_anti_dual.e415) - (this.e31 * right_anti_dual.e425) - (this.e12 * right_anti_dual.e435)), 
            /* e235, e315, e125, e5 */
            float4(float3(1.0), 0.0) * float4(right_anti_dual.group1.xyz, 0.0) * float4(this.group0.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((float3(wedge.e12345) * other.group0.xyz), -(other.e23 * wedge.e415) - (other.e31 * wedge.e425) - (other.e12 * wedge.e435)), 
            /* e15, e25, e35, e3215 */
            float4(float3(1.0), 0.0) * float4(other.group1.xyz, 0.0) * float4(wedge.group0.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
public extension AntiMotor {
    public AntiMotor project_orthogonally_onto(AntiMotor other) {
        Motor right_anti_dual = Motor(
            /* e415, e425, e435, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        Motor wedge = Motor(
            /* e415, e425, e435, e12345 */
            float4((this.group0.www * right_anti_dual.group0.xyz), (this.scalar * right_anti_dual.e12345) - (this.e23 * right_anti_dual.e415) - (this.e31 * right_anti_dual.e425) - (this.e12 * right_anti_dual.e435)), 
            /* e235, e315, e125, e5 */
            float4(((float3(this.scalar) * right_anti_dual.group1.xyz) + (float3(right_anti_dual.e5) * this.group0.xyz)), this.scalar * right_anti_dual.e5)
        );
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((float3(wedge.e12345) * other.group0.xyz), (other.scalar * wedge.e12345) - (other.e23 * wedge.e415) - (other.e31 * wedge.e425) - (other.e12 * wedge.e435)), 
            /* e15, e25, e35, e3215 */
            float4(((float3(other.e3215) * wedge.group0.xyz) + (float3(wedge.e12345) * other.group1.xyz)), other.e3215 * wedge.e12345)
        );
    }
}
public extension AntiMotor {
    public Scalar project_orthogonally_onto(AntiPlane other) {
        Plane wedge = Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar) * float4(other.group0.xyz, other.e5 * -1.0)
        );
        return Scalar(
            /* scalar */
            (other.e1 * wedge.e4235) + (other.e2 * wedge.e4315) + (other.e3 * wedge.e4125)
        );
    }
}
public extension AntiMotor {
    public AntiMotor project_orthogonally_onto(AntiScalar other) {
        Scalar right_anti_dual = Scalar(
            /* scalar */
            other.e12345 * -1.0
        );
        AntiMotor wedge = AntiMotor(
            /* e23, e31, e12, scalar */
            float4(right_anti_dual.scalar) * this.group0, 
            /* e15, e25, e35, e3215 */
            float4(right_anti_dual.scalar) * this.group1
        );
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(other.e12345) * wedge.group0, 
            /* e15, e25, e35, e3215 */
            float4(other.e12345) * wedge.group1
        );
    }
}
public extension AntiMotor {
    public AntiCircleRotor project_orthogonally_onto(Circle other) {
        Dipole right_anti_dual = Dipole(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35 */
            other.group2.xyz
        );
        DipoleInversion wedge = DipoleInversion(
            /* e41, e42, e43 */
            float3(this.scalar) * right_anti_dual.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(this.scalar) * right_anti_dual.group1, 
            /* e15, e25, e35, e1234 */
            float4((float3(this.scalar) * right_anti_dual.group2.xyz), -(this.e23 * right_anti_dual.e41) - (this.e31 * right_anti_dual.e42) - (this.e12 * right_anti_dual.e43)), 
            /* e4235, e4315, e4125, e3215 */
            float4((this.e23 * right_anti_dual.e45) + (this.e35 * right_anti_dual.e42), (this.e31 * right_anti_dual.e45) + (this.e15 * right_anti_dual.e43), (this.e12 * right_anti_dual.e45) + (this.e25 * right_anti_dual.e41), -(this.e31 * right_anti_dual.e25) - (this.e12 * right_anti_dual.e35) - (this.e15 * right_anti_dual.e23) - (this.e25 * right_anti_dual.e31) - (this.e35 * right_anti_dual.e12)) - float4((right_anti_dual.group0.xyz.zxy * this.group1.yzx), this.e23 * right_anti_dual.e15)
        );
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (float3(wedge.e1234) * other.group1.xyz) + (other.group0.xyz.zxy * wedge.group3.yzx) - (other.group0.xyz.yzx * wedge.group3.zxy), 
            /* e23, e31, e12, e45 */
            float4((other.e423 * wedge.e3215) + (other.e235 * wedge.e1234), (other.e431 * wedge.e3215) + (other.e315 * wedge.e1234), (other.e412 * wedge.e3215) + (other.e125 * wedge.e1234), -(other.e425 * wedge.e4315) - (other.e435 * wedge.e4125)) - (other.group1.wwwx * wedge.group3.xyzx), 
            /* e15, e25, e35, scalar */
            float4((other.e415 * wedge.e3215) + (other.e315 * wedge.e4125), (other.e425 * wedge.e3215) + (other.e125 * wedge.e4235), (other.e435 * wedge.e3215) + (other.e235 * wedge.e4315), -(other.e431 * wedge.e25) - (other.e412 * wedge.e35) - (other.e415 * wedge.e23) - (other.e425 * wedge.e31) - (other.e435 * wedge.e12) - (other.e321 * wedge.e45) - (other.e235 * wedge.e41) - (other.e315 * wedge.e42) - (other.e125 * wedge.e43)) - float4((other.group2.xyz.zxy * wedge.group3.yzx), other.e423 * wedge.e15)
        );
    }
}
public extension AntiMotor {
    public VersorOdd project_orthogonally_onto(CircleRotor other) {
        AntiCircleRotor right_anti_dual = AntiCircleRotor(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        VersorOdd wedge = VersorOdd(
            /* e41, e42, e43, scalar */
            float4(this.scalar) * float4(right_anti_dual.group0.xyz, right_anti_dual.scalar), 
            /* e23, e31, e12, e45 */
            float4(((float3(right_anti_dual.scalar) * this.group0.xyz) + (float3(this.scalar) * right_anti_dual.group1.xyz)), right_anti_dual.e45 * this.scalar), 
            /* e15, e25, e35, e1234 */
            float4((right_anti_dual.e15 * this.scalar) + (right_anti_dual.scalar * this.e15), (right_anti_dual.e25 * this.scalar) + (right_anti_dual.scalar * this.e25), (right_anti_dual.e35 * this.scalar) + (right_anti_dual.scalar * this.e35), -(right_anti_dual.e41 * this.e23) - (right_anti_dual.e42 * this.e31) - (right_anti_dual.e43 * this.e12)), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual.e45 * this.e23, right_anti_dual.e45 * this.e31, right_anti_dual.e45 * this.e12, -(right_anti_dual.e31 * this.e25) - (right_anti_dual.e12 * this.e35) - (right_anti_dual.e15 * this.e23) - (right_anti_dual.e25 * this.e31) - (right_anti_dual.e35 * this.e12)) + (this.group1.zxyw * float4(right_anti_dual.group0.xyz.yzx, right_anti_dual.scalar)) - (this.group1.yzxx * float4(right_anti_dual.group0.xyz.zxy, right_anti_dual.e23))
        );
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4((other.e415 * wedge.e1234) + (other.e12345 * wedge.e41), (other.e425 * wedge.e1234) + (other.e12345 * wedge.e42), (other.e435 * wedge.e1234) + (other.e12345 * wedge.e43), -(other.e431 * wedge.e25) - (other.e412 * wedge.e35) - (other.e415 * wedge.e23) - (other.e425 * wedge.e31) - (other.e435 * wedge.e12) - (other.e321 * wedge.e45) - (other.e235 * wedge.e41) - (other.e315 * wedge.e42) - (other.e125 * wedge.e43)) + float4((other.group0.xyz.zxy * wedge.group3.yzx), other.e12345 * wedge.scalar) - float4((other.group0.xyz.yzx * wedge.group3.zxy), other.e423 * wedge.e15), 
            /* e23, e31, e12, e45 */
            float4((other.e235 * wedge.e1234) + (other.e12345 * wedge.e23), (other.e315 * wedge.e1234) + (other.e12345 * wedge.e31), (other.e125 * wedge.e1234) + (other.e12345 * wedge.e12), -(other.e425 * wedge.e4315) - (other.e435 * wedge.e4125)) + float4((other.group0.xyz * wedge.group3.www), other.e12345 * wedge.e45) - (other.group1.wwwx * wedge.group3.xyzx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(other.e12345) * wedge.group2.xyz) + (float3(wedge.e3215) * other.group1.xyz) + (other.group2.yzx * wedge.group3.zxy) - (other.group2.zxy * wedge.group3.yzx)), other.e12345 * wedge.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345) * wedge.group3
        );
    }
}
public extension AntiMotor {
    public AntiCircleRotor project_orthogonally_onto(Dipole other) {
        Circle right_anti_dual = Circle(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125 */
            other.group2.xyz * float3(-1.0)
        );
        CircleRotor wedge = CircleRotor(
            /* e423, e431, e412 */
            float3(this.scalar) * right_anti_dual.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(this.scalar) * right_anti_dual.group1, 
            /* e235, e315, e125, e12345 */
            float4((float3(this.scalar) * right_anti_dual.group2.xyz), -(this.e23 * right_anti_dual.e415) - (this.e31 * right_anti_dual.e425) - (this.e12 * right_anti_dual.e435) - (this.e15 * right_anti_dual.e423) - (this.e25 * right_anti_dual.e431) - (this.e35 * right_anti_dual.e412))
        );
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(wedge.e12345) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(wedge.e12345) * other.group1, 
            /* e15, e25, e35, scalar */
            float4((float3(wedge.e12345) * other.group2.xyz), -(wedge.e423 * other.e15) - (wedge.e431 * other.e25) - (wedge.e412 * other.e35) - (wedge.e415 * other.e23) - (wedge.e425 * other.e31) - (wedge.e435 * other.e12) - (wedge.e321 * other.e45) - (wedge.e235 * other.e41) - (wedge.e315 * other.e42) - (wedge.e125 * other.e43))
        );
    }
}
public extension AntiMotor {
    public VersorOdd project_orthogonally_onto(DipoleInversion other) {
        AntiDipoleInversion right_anti_dual = AntiDipoleInversion(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e4 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group3 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        VersorEven wedge = VersorEven(
            /* e423, e431, e412, e12345 */
            float4(right_anti_dual.e4 * this.e23, right_anti_dual.e4 * this.e31, right_anti_dual.e4 * this.e12, -(right_anti_dual.e423 * this.e15) - (right_anti_dual.e431 * this.e25) - (right_anti_dual.e412 * this.e35) - (right_anti_dual.e415 * this.e23) - (right_anti_dual.e425 * this.e31) - (right_anti_dual.e435 * this.e12)) + float4((right_anti_dual.group0.xyz * this.group0.www), right_anti_dual.e4 * this.e3215), 
            /* e415, e425, e435, e321 */
            float4(right_anti_dual.e4 * this.e15, right_anti_dual.e4 * this.e25, right_anti_dual.e4 * this.e35, -(right_anti_dual.e1 * this.e23) - (right_anti_dual.e2 * this.e31) - (right_anti_dual.e3 * this.e12)) + (float4(this.scalar) * right_anti_dual.group1), 
            /* e235, e315, e125, e5 */
            float4(((float3(right_anti_dual.e5) * this.group0.xyz) + (float3(this.scalar) * right_anti_dual.group2.xyz) + (right_anti_dual.group3.yzx * this.group1.zxy) - (right_anti_dual.group3.zxy * this.group1.yzx)), right_anti_dual.e5 * this.scalar), 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * float4(right_anti_dual.group3.xyz, right_anti_dual.e4)
        );
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (other.group3.yzxy * float4(wedge.group0.zxy, wedge.e2)) + float4(float3(0.0), (other.e4125 * wedge.e3) + (other.e3215 * wedge.e4) - (other.e42 * wedge.e315) - (other.e43 * wedge.e125) - (other.e23 * wedge.e415) - (other.e31 * wedge.e425) - (other.e12 * wedge.e435) - (other.e45 * wedge.e321) - (other.e15 * wedge.e423) - (other.e25 * wedge.e431) - (other.e35 * wedge.e412)) + float4((other.group0.xyz * wedge.group0.www), other.e1234 * wedge.e5) + float4((other.group2.www * wedge.group1.xyz), other.e4235 * wedge.e1) - float4((other.group3.zxy * wedge.group0.yzx), other.e41 * wedge.e235), 
            /* e23, e31, e12, e45 */
            float4((other.e1234 * wedge.e235) + (other.e3215 * wedge.e423), (other.e1234 * wedge.e315) + (other.e3215 * wedge.e431), (other.e1234 * wedge.e125) + (other.e3215 * wedge.e412), -(other.e4315 * wedge.e425) - (other.e4125 * wedge.e435)) + (float4(wedge.e12345) * other.group1) - (other.group3.xyzx * wedge.group1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(other.e3215) * wedge.group1.xyz) + (float3(wedge.e12345) * other.group2.xyz) + (other.group3.zxy * wedge.group2.yzx) - (other.group3.yzx * wedge.group2.zxy)), other.e1234 * wedge.e12345), 
            /* e4235, e4315, e4125, e3215 */
            float4(wedge.e12345) * other.group3
        );
    }
}
public extension AntiMotor {
    public AntiMotor project_orthogonally_onto(DualNum other) {
        AntiDualNum right_anti_dual = AntiDualNum(
            /* e3215, scalar */
            other.group0.xy * float2(-1.0)
        );
        AntiMotor wedge = AntiMotor(
            /* e23, e31, e12, scalar */
            float4(right_anti_dual.scalar) * this.group0, 
            /* e15, e25, e35, e3215 */
            float4(right_anti_dual.group0.xy.yy, right_anti_dual.scalar, (right_anti_dual.e3215 * this.scalar) + (right_anti_dual.scalar * this.e3215)) * float4(this.group1.xyz, 1.0)
        );
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(other.e12345) * wedge.group0, 
            /* e15, e25, e35, e3215 */
            float4(other.e12345) * wedge.group1
        );
    }
}
public extension AntiMotor {
    public Scalar project_orthogonally_onto(FlatPoint other) {
        return Scalar(
            /* scalar */
            pow(other.e45, 2) * this.scalar * -1.0
        );
    }
}
public extension AntiMotor {
    public AntiMotor project_orthogonally_onto(Flector other) {
        AntiFlector right_anti_dual = AntiFlector(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        AntiFlector wedge = AntiFlector(
            /* e235, e315, e125, e321 */
            float4((right_anti_dual.e2 * this.e35) + (right_anti_dual.e5 * this.e23), (right_anti_dual.e3 * this.e15) + (right_anti_dual.e5 * this.e31), (right_anti_dual.e1 * this.e25) + (right_anti_dual.e5 * this.e12), -(right_anti_dual.e2 * this.e31) - (right_anti_dual.e3 * this.e12)) + (float4(this.scalar) * right_anti_dual.group0) - (right_anti_dual.group1.zxyx * float4(this.group1.yzx, this.e23)), 
            /* e1, e2, e3, e5 */
            float4(this.scalar) * right_anti_dual.group1
        );
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((wedge.group0.www * other.group1.xyz * float3(-1.0)), (wedge.e1 * other.e4235) + (wedge.e2 * other.e4315) + (wedge.e3 * other.e4125) - (wedge.e321 * other.e45)), 
            /* e15, e25, e35, e3215 */
            float4(((wedge.group0.yzx * other.group1.zxy) - (wedge.group0.zxy * other.group1.yzx)), 0.0)
        );
    }
}
public extension AntiMotor {
    public AntiMotor project_orthogonally_onto(Line other) {
        AntiLine right_anti_dual = AntiLine(
            /* e23, e31, e12 */
            other.group0.xyz, 
            /* e15, e25, e35 */
            other.group1.xyz
        );
        AntiMotor wedge = AntiMotor(
            /* e23, e31, e12, scalar */
            float4(float3(1.0), 0.0) * float4(right_anti_dual.group0.xyz, 0.0) * float4(this.group0.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e15, e25, e35, e3215 */
            float4((float3(this.scalar) * right_anti_dual.group1.xyz), -(right_anti_dual.e23 * this.e15) - (right_anti_dual.e31 * this.e25) - (right_anti_dual.e12 * this.e35) - (right_anti_dual.e15 * this.e23) - (right_anti_dual.e25 * this.e31) - (right_anti_dual.e35 * this.e12))
        );
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(float3(0.0), -(wedge.e23 * other.e415) - (wedge.e31 * other.e425) - (wedge.e12 * other.e435)), 
            /* e15, e25, e35, e3215 */
            float4(float3(1.0), 0.0) * float4(other.group0.xyz, 0.0) * float4(wedge.group1.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
public extension AntiMotor {
    public AntiMotor project_orthogonally_onto(Motor other) {
        AntiMotor right_anti_dual = AntiMotor(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        AntiMotor wedge = AntiMotor(
            /* e23, e31, e12, scalar */
            float4(((float3(right_anti_dual.scalar) * this.group0.xyz) + (float3(this.scalar) * right_anti_dual.group0.xyz)), right_anti_dual.scalar * this.scalar), 
            /* e15, e25, e35, e3215 */
            (float4(right_anti_dual.scalar) * this.group1) + (float4(this.scalar) * right_anti_dual.group1) + float4(float3(0.0), -(right_anti_dual.e23 * this.e15) - (right_anti_dual.e31 * this.e25) - (right_anti_dual.e12 * this.e35) - (right_anti_dual.e15 * this.e23) - (right_anti_dual.e25 * this.e31) - (right_anti_dual.e35 * this.e12))
        );
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((float3(other.e12345) * wedge.group0.xyz), (wedge.scalar * other.e12345) - (wedge.e23 * other.e415) - (wedge.e31 * other.e425) - (wedge.e12 * other.e435)), 
            /* e15, e25, e35, e3215 */
            float4(((float3(wedge.e3215) * other.group0.xyz) + (float3(other.e12345) * wedge.group1.xyz)), wedge.e3215 * other.e12345)
        );
    }
}
public extension AntiMotor {
    public MultiVector project_orthogonally_onto(MultiVector other) {
        MultiVector right_anti_dual = MultiVector(
            /* scalar, e12345 */
            other.group0.xy.yx * float2(-1.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4(other.group9.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5 */
            other.e3215, 
            /* e15, e25, e35, e45 */
            float4(other.group8.xyz, other.e321 * -1.0), 
            /* e41, e42, e43 */
            other.group7.xyz, 
            /* e23, e31, e12 */
            other.group6.xyz, 
            /* e415, e425, e435, e321 */
            float4(other.group5.xyz, other.e45) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e423, e431, e412 */
            other.group4.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            other.group3.xyz * float3(-1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.group1.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234 */
            other.e4 * -1.0
        );
        MultiVector wedge = MultiVector(
            /* scalar, e12345 */
            float2(this.scalar * right_anti_dual.scalar, (this.scalar * right_anti_dual.e12345) + (this.e3215 * right_anti_dual.e4) - (this.e23 * right_anti_dual.e415) - (this.e31 * right_anti_dual.e425) - (this.e12 * right_anti_dual.e435) - (this.e15 * right_anti_dual.e423) - (this.e25 * right_anti_dual.e431) - (this.e35 * right_anti_dual.e412)), 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * right_anti_dual.group1, 
            /* e5 */
            this.scalar * right_anti_dual.e5, 
            /* e15, e25, e35, e45 */
            float4(((float3(this.scalar) * right_anti_dual.group3.xyz) + (float3(right_anti_dual.scalar) * this.group1.xyz)), this.scalar * right_anti_dual.e45), 
            /* e41, e42, e43 */
            float3(this.scalar) * right_anti_dual.group4.xyz, 
            /* e23, e31, e12 */
            (float3(this.scalar) * right_anti_dual.group5.xyz) + (float3(right_anti_dual.scalar) * this.group0.xyz), 
            /* e415, e425, e435, e321 */
            float4(this.e15 * right_anti_dual.e4, this.e25 * right_anti_dual.e4, this.e35 * right_anti_dual.e4, -(this.e23 * right_anti_dual.e1) - (this.e31 * right_anti_dual.e2) - (this.e12 * right_anti_dual.e3)) + (float4(this.scalar) * right_anti_dual.group6), 
            /* e423, e431, e412 */
            (float3(this.scalar) * right_anti_dual.group7.xyz) + (float3(right_anti_dual.e4) * this.group0.xyz), 
            /* e235, e315, e125 */
            (float3(this.scalar) * right_anti_dual.group8.xyz) + (float3(right_anti_dual.e5) * this.group0.xyz) + (this.group1.zxy * right_anti_dual.group1.yzx) - (this.group1.yzx * right_anti_dual.group1.zxy), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar * right_anti_dual.e4235, this.scalar * right_anti_dual.e4315, this.scalar * right_anti_dual.e4125, -(this.e23 * right_anti_dual.e15) - (this.e31 * right_anti_dual.e25) - (this.e12 * right_anti_dual.e35) - (this.e25 * right_anti_dual.e31) - (this.e35 * right_anti_dual.e12)) + (this.group0 * float4(right_anti_dual.group3.www, right_anti_dual.e3215)) + (this.group1.zxyw * float4(right_anti_dual.group4.xyz.yzx, right_anti_dual.scalar)) - (this.group1.yzxx * float4(right_anti_dual.group4.xyz.zxy, right_anti_dual.e23)), 
            /* e1234 */
            (this.scalar * right_anti_dual.e1234) - (this.e23 * right_anti_dual.e41) - (this.e31 * right_anti_dual.e42) - (this.e12 * right_anti_dual.e43)
        );
        return MultiVector(
            /* scalar, e12345 */
            float2((other.scalar * wedge.e12345) + (other.e12345 * wedge.scalar) + (other.e1 * wedge.e4235) + (other.e2 * wedge.e4315) + (other.e3 * wedge.e4125) + (other.e4 * wedge.e3215) + (other.e5 * wedge.e1234) + (other.e4235 * wedge.e1) + (other.e4315 * wedge.e2) + (other.e4125 * wedge.e3) + (other.e3215 * wedge.e4) + (other.e1234 * wedge.e5) - (other.e15 * wedge.e423) - (other.e25 * wedge.e431) - (other.e35 * wedge.e412) - (other.e45 * wedge.e321) - (other.e41 * wedge.e235) - (other.e42 * wedge.e315) - (other.e43 * wedge.e125) - (other.e23 * wedge.e415) - (other.e31 * wedge.e425) - (other.e12 * wedge.e435) - (other.e415 * wedge.e23) - (other.e425 * wedge.e31) - (other.e435 * wedge.e12) - (other.e321 * wedge.e45) - (other.e423 * wedge.e15) - (other.e431 * wedge.e25) - (other.e412 * wedge.e35) - (other.e235 * wedge.e41) - (other.e315 * wedge.e42) - (other.e125 * wedge.e43), other.e12345 * wedge.e12345), 
            /* e1, e2, e3, e4 */
            float4((other.e415 * wedge.e321) + (other.e321 * wedge.e415) + (other.e4315 * wedge.e12) + (other.e1234 * wedge.e15), (other.e425 * wedge.e321) + (other.e321 * wedge.e425) + (other.e4125 * wedge.e23) + (other.e1234 * wedge.e25), (other.e435 * wedge.e321) + (other.e321 * wedge.e435) + (other.e4235 * wedge.e31) + (other.e1234 * wedge.e35), -(other.e45 * wedge.e1234) - (other.e415 * wedge.e423) - (other.e425 * wedge.e431) - (other.e435 * wedge.e412)) + (float4(other.e12345) * wedge.group1) + (float4(wedge.e12345) * other.group1) + float4((other.group4.xyz * wedge.group9.www), other.e4235 * wedge.e41) + float4((other.group5.xyz.yzx * wedge.group9.zxy), other.e4315 * wedge.e42) + float4((other.group7.xyz.zxy * wedge.group8.xyz.yzx), other.e4125 * wedge.e43) + float4((other.group8.xyz.yzx * wedge.group7.xyz.zxy), other.e1234 * wedge.e45) - (wedge.group9.yzxx * float4(other.group5.xyz.zxy, other.e41)) - float4((float3(wedge.e1234) * other.group3.xyz), other.e412 * wedge.e435) - float4((wedge.group4.xyz * other.group9.www), other.e423 * wedge.e415) - float4((other.group7.xyz.yzx * wedge.group8.xyz.zxy), other.e42 * wedge.e4315) - float4((other.group8.xyz.zxy * wedge.group7.xyz.yzx), other.e43 * wedge.e4125) - float4((wedge.group5.xyz.yzx * other.group9.zxy), other.e431 * wedge.e425), 
            /* e5 */
            (other.e12345 * wedge.e5) + (other.e5 * wedge.e12345) + (other.e15 * wedge.e4235) + (other.e25 * wedge.e4315) + (other.e35 * wedge.e4125) + (other.e45 * wedge.e3215) - (other.e415 * wedge.e235) - (other.e425 * wedge.e315) - (other.e435 * wedge.e125) - (other.e235 * wedge.e415) - (other.e315 * wedge.e425) - (other.e125 * wedge.e435) - (other.e4235 * wedge.e15) - (other.e4315 * wedge.e25) - (other.e4125 * wedge.e35) - (other.e3215 * wedge.e45), 
            /* e15, e25, e35, e45 */
            float4((other.e415 * wedge.e3215) + (other.e315 * wedge.e4125) + (other.e4125 * wedge.e315) + (other.e3215 * wedge.e415), (other.e425 * wedge.e3215) + (other.e125 * wedge.e4235) + (other.e4235 * wedge.e125) + (other.e3215 * wedge.e425), (other.e435 * wedge.e3215) + (other.e235 * wedge.e4315) + (other.e4315 * wedge.e235) + (other.e3215 * wedge.e435), -(other.e435 * wedge.e4125) - (other.e4235 * wedge.e415) - (other.e4315 * wedge.e425) - (other.e4125 * wedge.e435)) + (float4(other.e12345) * wedge.group3) + (float4(wedge.e12345) * other.group3) - (wedge.group9.yzxx * float4(other.group8.xyz.zxy, other.e415)) - float4((wedge.group8.xyz.zxy * other.group9.yzx), other.e425 * wedge.e4315), 
            /* e41, e42, e43 */
            (float3(other.e12345) * wedge.group4.xyz) + (float3(other.e1234) * wedge.group6.xyz) + (float3(wedge.e12345) * other.group4.xyz) + (float3(wedge.e1234) * other.group6.xyz) + (other.group7.xyz.zxy * wedge.group9.yzx) + (wedge.group7.xyz.zxy * other.group9.yzx) - (other.group7.xyz.yzx * wedge.group9.zxy) - (wedge.group7.xyz.yzx * other.group9.zxy), 
            /* e23, e31, e12 */
            (float3(other.e12345) * wedge.group5.xyz) + (float3(other.e3215) * wedge.group7.xyz) + (float3(other.e1234) * wedge.group8.xyz) + (float3(wedge.e12345) * other.group5.xyz) + (float3(wedge.e3215) * other.group7.xyz) + (float3(wedge.e1234) * other.group8.xyz) - (float3(other.e321) * wedge.group9.xyz) - (float3(wedge.e321) * other.group9.xyz), 
            /* e415, e425, e435, e321 */
            (float4(other.e12345) * wedge.group6) + (float4(wedge.e12345) * other.group6) + float4((other.group9.zxy * wedge.group9.yzx), other.e1234 * wedge.e3215) - (other.group9.yzxw * float4(wedge.group9.zxy, wedge.e1234)), 
            /* e423, e431, e412 */
            (float3(other.e12345) * wedge.group7.xyz) + (float3(other.e1234) * wedge.group9.xyz) + (float3(wedge.e12345) * other.group7.xyz) - (float3(wedge.e1234) * other.group9.xyz), 
            /* e235, e315, e125 */
            (float3(other.e12345) * wedge.group8.xyz) + (float3(wedge.e12345) * other.group8.xyz) + (float3(wedge.e3215) * other.group9.xyz) - (float3(other.e3215) * wedge.group9.xyz), 
            /* e4235, e4315, e4125, e3215 */
            (float4(other.e12345) * wedge.group9) + (float4(wedge.e12345) * other.group9), 
            /* e1234 */
            (other.e12345 * wedge.e1234) + (other.e1234 * wedge.e12345)
        );
    }
}
public extension AntiMotor {
    public AntiMotor project_orthogonally_onto(Plane other) {
        AntiPlane right_anti_dual = AntiPlane(
            /* e1, e2, e3, e5 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        AntiFlector wedge = AntiFlector(
            /* e235, e315, e125, e321 */
            float4((this.e23 * right_anti_dual.e5) + (this.e35 * right_anti_dual.e2), (this.e31 * right_anti_dual.e5) + (this.e15 * right_anti_dual.e3), (this.e12 * right_anti_dual.e5) + (this.e25 * right_anti_dual.e1), -(this.e31 * right_anti_dual.e2) - (this.e12 * right_anti_dual.e3)) - (right_anti_dual.group0.zxyx * float4(this.group1.yzx, this.e23)), 
            /* e1, e2, e3, e5 */
            float4(this.scalar) * right_anti_dual.group0
        );
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((wedge.group0.www * other.group0.xyz * float3(-1.0)), (wedge.e1 * other.e4235) + (wedge.e2 * other.e4315) + (wedge.e3 * other.e4125)), 
            /* e15, e25, e35, e3215 */
            float4(((wedge.group0.yzx * other.group0.zxy) - (wedge.group0.zxy * other.group0.yzx)), 0.0)
        );
    }
}
public extension AntiMotor {
    public Scalar project_orthogonally_onto(RoundPoint other) {
        Sphere right_anti_dual = Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234 */
            other.e4 * -1.0
        );
        Sphere wedge = Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar) * right_anti_dual.group0, 
            /* e1234 */
            this.scalar * right_anti_dual.e1234
        );
        return Scalar(
            /* scalar */
            (other.e1 * wedge.e4235) + (other.e2 * wedge.e4315) + (other.e3 * wedge.e4125) + (other.e4 * wedge.e3215) + (other.e5 * wedge.e1234)
        );
    }
}
public extension AntiMotor {
    public Scalar project_orthogonally_onto(Scalar other) {
        return Scalar(
            /* scalar */
            pow(other.scalar, 2) * this.scalar
        );
    }
}
public extension AntiMotor {
    public VersorOdd project_orthogonally_onto(Sphere other) {
        RoundPoint right_anti_dual = RoundPoint(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5 */
            other.e3215
        );
        VersorEven wedge = VersorEven(
            /* e423, e431, e412, e12345 */
            float4(right_anti_dual.e4) * float4(this.group0.xyz, this.e3215), 
            /* e415, e425, e435, e321 */
            float4((float3(right_anti_dual.e4) * this.group1.xyz), -(this.e23 * right_anti_dual.e1) - (this.e31 * right_anti_dual.e2) - (this.e12 * right_anti_dual.e3)), 
            /* e235, e315, e125, e5 */
            float4(((float3(right_anti_dual.e5) * this.group0.xyz) + (this.group1.zxy * right_anti_dual.group0.yzx) - (this.group1.yzx * right_anti_dual.group0.zxy)), this.scalar * right_anti_dual.e5), 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * right_anti_dual.group0
        );
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(other.e4125 * wedge.e431 * -1.0, other.e4235 * wedge.e412 * -1.0, other.e4315 * wedge.e423 * -1.0, (other.e4125 * wedge.e3) + (other.e3215 * wedge.e4) + (other.e1234 * wedge.e5)) + (other.group0.yzxx * float4(wedge.group0.zxy, wedge.e1)) + float4((float3(other.e1234) * wedge.group1.xyz), other.e4315 * wedge.e2), 
            /* e23, e31, e12, e45 */
            float4((other.e3215 * wedge.e423) + (other.e1234 * wedge.e235), (other.e3215 * wedge.e431) + (other.e1234 * wedge.e315), (other.e3215 * wedge.e412) + (other.e1234 * wedge.e125), -(other.e4315 * wedge.e425) - (other.e4125 * wedge.e435)) - (other.group0.xyzx * wedge.group1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(other.e3215) * wedge.group1.xyz) + (other.group0.zxy * wedge.group2.yzx) - (other.group0.yzx * wedge.group2.zxy)), other.e1234 * wedge.e12345), 
            /* e4235, e4315, e4125, e3215 */
            float4(wedge.e12345) * other.group0
        );
    }
}
public extension AntiMotor {
    public VersorOdd project_orthogonally_onto(VersorEven other) {
        VersorOdd right_anti_dual = VersorOdd(
            /* e41, e42, e43, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e1234 */
            float4(other.group2.xyz, other.e4) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.group3.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0)
        );
        VersorOdd wedge = VersorOdd(
            /* e41, e42, e43, scalar */
            float4(this.scalar) * right_anti_dual.group0, 
            /* e23, e31, e12, e45 */
            float4(((float3(this.scalar) * right_anti_dual.group1.xyz) + (float3(right_anti_dual.scalar) * this.group0.xyz)), this.scalar * right_anti_dual.e45), 
            /* e15, e25, e35, e1234 */
            float4(this.e15 * right_anti_dual.scalar, this.e25 * right_anti_dual.scalar, this.e35 * right_anti_dual.scalar, -(this.e23 * right_anti_dual.e41) - (this.e31 * right_anti_dual.e42) - (this.e12 * right_anti_dual.e43)) + (float4(this.scalar) * right_anti_dual.group2), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e35 * right_anti_dual.e42, this.e15 * right_anti_dual.e43, this.e25 * right_anti_dual.e41, -(this.e31 * right_anti_dual.e25) - (this.e12 * right_anti_dual.e35) - (this.e15 * right_anti_dual.e23) - (this.e25 * right_anti_dual.e31) - (this.e35 * right_anti_dual.e12)) + (this.group0 * float4(right_anti_dual.group1.www, right_anti_dual.e3215)) + float4((this.group0.www * right_anti_dual.group3.xyz), this.e3215 * right_anti_dual.scalar) - float4((this.group1.yzx * right_anti_dual.group0.zxy), this.e23 * right_anti_dual.e15)
        );
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (other.group0.zxyw * float4(wedge.group3.yzx, wedge.scalar)) + float4(float3(0.0), (other.e2 * wedge.e4315) + (other.e3 * wedge.e4125) + (other.e4 * wedge.e3215) - (other.e431 * wedge.e25) - (other.e412 * wedge.e35) - (other.e415 * wedge.e23) - (other.e425 * wedge.e31) - (other.e435 * wedge.e12) - (other.e321 * wedge.e45) - (other.e235 * wedge.e41) - (other.e315 * wedge.e42) - (other.e125 * wedge.e43)) + float4((other.group0.www * wedge.group0.xyz), other.e5 * wedge.e1234) + float4((wedge.group2.www * other.group1.xyz), other.e1 * wedge.e4235) - (other.group0.yzxx * float4(wedge.group3.zxy, wedge.e15)), 
            /* e23, e31, e12, e45 */
            float4((other.e12345 * wedge.e23) + (other.e235 * wedge.e1234), (other.e12345 * wedge.e31) + (other.e315 * wedge.e1234), (other.e12345 * wedge.e12) + (other.e125 * wedge.e1234), -(other.e425 * wedge.e4315) - (other.e435 * wedge.e4125)) + (other.group0 * float4(wedge.group3.www, wedge.e45)) - (other.group1.wwwx * wedge.group3.xyzx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(other.e12345) * wedge.group2.xyz) + (float3(wedge.e3215) * other.group1.xyz) + (other.group2.yzx * wedge.group3.zxy) - (other.group2.zxy * wedge.group3.yzx)), other.e12345 * wedge.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345) * wedge.group3
        );
    }
}
public extension AntiMotor {
    public VersorOdd project_orthogonally_onto(VersorOdd other) {
        VersorEven right_anti_dual = VersorEven(
            /* e423, e431, e412, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            float4(other.group2.xyz, other.e3215) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4(other.group3.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0)
        );
        VersorEven wedge = VersorEven(
            /* e423, e431, e412, e12345 */
            (this.group0 * float4(right_anti_dual.group3.www, right_anti_dual.e12345)) + float4(float3(0.0), -(this.e23 * right_anti_dual.e415) - (this.e31 * right_anti_dual.e425) - (this.e12 * right_anti_dual.e435) - (this.e15 * right_anti_dual.e423) - (this.e25 * right_anti_dual.e431) - (this.e35 * right_anti_dual.e412)) + float4((this.group0.www * right_anti_dual.group0.xyz), this.e3215 * right_anti_dual.e4), 
            /* e415, e425, e435, e321 */
            float4(this.e15 * right_anti_dual.e4, this.e25 * right_anti_dual.e4, this.e35 * right_anti_dual.e4, -(this.e23 * right_anti_dual.e1) - (this.e31 * right_anti_dual.e2) - (this.e12 * right_anti_dual.e3)) + (float4(this.scalar) * right_anti_dual.group1), 
            /* e235, e315, e125, e5 */
            float4(((float3(this.scalar) * right_anti_dual.group2.xyz) + (float3(right_anti_dual.e5) * this.group0.xyz) + (this.group1.zxy * right_anti_dual.group3.yzx) - (this.group1.yzx * right_anti_dual.group3.zxy)), this.scalar * right_anti_dual.e5), 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * right_anti_dual.group3
        );
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (wedge.group0.zxyw * float4(other.group3.yzx, other.scalar)) + float4(float3(0.0), (wedge.e2 * other.e4315) + (wedge.e3 * other.e4125) + (wedge.e4 * other.e3215) - (wedge.e431 * other.e25) - (wedge.e412 * other.e35) - (wedge.e415 * other.e23) - (wedge.e425 * other.e31) - (wedge.e435 * other.e12) - (wedge.e321 * other.e45) - (wedge.e235 * other.e41) - (wedge.e315 * other.e42) - (wedge.e125 * other.e43)) + float4((wedge.group0.www * other.group0.xyz), wedge.e5 * other.e1234) + float4((other.group2.www * wedge.group1.xyz), wedge.e1 * other.e4235) - (wedge.group0.yzxx * float4(other.group3.zxy, other.e15)), 
            /* e23, e31, e12, e45 */
            float4((wedge.e12345 * other.e23) + (wedge.e235 * other.e1234), (wedge.e12345 * other.e31) + (wedge.e315 * other.e1234), (wedge.e12345 * other.e12) + (wedge.e125 * other.e1234), -(wedge.e425 * other.e4315) - (wedge.e435 * other.e4125)) + (wedge.group0 * float4(other.group3.www, other.e45)) - (wedge.group1.wwwx * other.group3.xyzx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(wedge.e12345) * other.group2.xyz) + (float3(other.e3215) * wedge.group1.xyz) + (wedge.group2.yzx * other.group3.zxy) - (wedge.group2.zxy * other.group3.yzx)), wedge.e12345 * other.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(wedge.e12345) * other.group3
        );
    }
}
public extension AntiPlane {
    // Fancy infix trick (first half)
    public ProjectOrthogonallyOntoInfixPartial<AntiPlane> operator /(ProjectOrthogonallyOntoInfix rhs) {
        return ProjectOrthogonallyOntoInfixPartial<AntiPlane>(this);
    }
}
public extension ProjectOrthogonallyOntoInfixPartial<AntiPlane> {
    // Fancy infix trick (second half)
    public RoundPoint operator /(AntiCircleRotor rhs) {
        return this.a.project_orthogonally_onto(rhs);
    }
}
public extension AntiPlane {
    public RoundPoint project_orthogonally_onto(AntiCircleRotor other) {
        CircleRotor right_anti_dual = CircleRotor(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        Sphere wedge = Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(this.e3 * right_anti_dual.e425, this.e1 * right_anti_dual.e435, this.e2 * right_anti_dual.e415, -(this.e3 * right_anti_dual.e125) - (this.e5 * right_anti_dual.e321)) - (this.group0.yzxy * float4(right_anti_dual.group1.zxy, right_anti_dual.e315)) - float4((right_anti_dual.group0.xyz * this.group0.www), this.e1 * right_anti_dual.e235), 
            /* e1234 */
            (this.e1 * right_anti_dual.e423) + (this.e2 * right_anti_dual.e431) + (this.e3 * right_anti_dual.e412)
        );
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4((other.e41 * wedge.e3215) + (other.e31 * wedge.e4125), (other.e42 * wedge.e3215) + (other.e12 * wedge.e4235), (other.e43 * wedge.e3215) + (other.e23 * wedge.e4315), -(other.e43 * wedge.e4125) - (other.e45 * wedge.e1234)) - (wedge.group0.yzxx * float4(other.group1.zxy, other.e41)) - float4((float3(wedge.e1234) * other.group2.xyz), other.e42 * wedge.e4315), 
            /* e5 */
            (other.e45 * wedge.e3215) + (other.e15 * wedge.e4235) + (other.e25 * wedge.e4315) + (other.e35 * wedge.e4125)
        );
    }
}
public extension AntiPlane {
    public AntiDipoleInversion project_orthogonally_onto(AntiDipoleInversion other) {
        DipoleInversion right_anti_dual = DipoleInversion(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e1234 */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group3 * float4(1.0, 1.0, 1.0, -1.0)
        );
        CircleRotor wedge = CircleRotor(
            /* e423, e431, e412 */
            (right_anti_dual.group0.xyz.yzx * this.group0.zxy) - (right_anti_dual.group0.xyz.zxy * this.group0.yzx), 
            /* e415, e425, e435, e321 */
            float4(this.e5 * right_anti_dual.e41, this.e5 * right_anti_dual.e42, this.e5 * right_anti_dual.e43, -(this.e2 * right_anti_dual.e31) - (this.e3 * right_anti_dual.e12)) - (this.group0.xyzx * right_anti_dual.group1.wwwx), 
            /* e235, e315, e125, e12345 */
            float4(this.e3 * right_anti_dual.e25 * -1.0, this.e1 * right_anti_dual.e35 * -1.0, this.e2 * right_anti_dual.e15 * -1.0, (this.e3 * right_anti_dual.e4125) + (this.e5 * right_anti_dual.e1234)) + (this.group0.yzxx * float4(right_anti_dual.group2.zxy, right_anti_dual.e4235)) + (this.group0.wwwy * float4(right_anti_dual.group1.xyz, right_anti_dual.e4315))
        );
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(wedge.e12345) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(wedge.e12345) * other.group1, 
            /* e235, e315, e125, e4 */
            float4((float3(wedge.e12345) * other.group2.xyz), (other.e4 * wedge.e12345) - (other.e423 * wedge.e415) - (other.e431 * wedge.e425) - (other.e412 * wedge.e435) - (other.e415 * wedge.e423) - (other.e425 * wedge.e431) - (other.e435 * wedge.e412)), 
            /* e1, e2, e3, e5 */
            float4((other.e415 * wedge.e321) + (other.e321 * wedge.e415) + (other.e315 * wedge.e412) + (other.e1 * wedge.e12345), (other.e425 * wedge.e321) + (other.e321 * wedge.e425) + (other.e125 * wedge.e423) + (other.e2 * wedge.e12345), (other.e435 * wedge.e321) + (other.e321 * wedge.e435) + (other.e235 * wedge.e431) + (other.e3 * wedge.e12345), -(other.e435 * wedge.e125) - (other.e235 * wedge.e415) - (other.e315 * wedge.e425) - (other.e125 * wedge.e435)) + (wedge.group2.yzxw * float4(other.group0.xyz.zxy, other.e5)) - (wedge.group2.zxyx * float4(other.group0.xyz.yzx, other.e415)) - float4((wedge.group0.xyz.yzx * other.group2.zxy), other.e425 * wedge.e315)
        );
    }
}
public extension AntiPlane {
    public AntiPlane project_orthogonally_onto(AntiFlatPoint other) {
        FlatPoint right_anti_dual = FlatPoint(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        Line wedge = Line(
            /* e415, e425, e435 */
            float3(right_anti_dual.e45) * this.group0.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            (this.group0.yzx * right_anti_dual.group0.zxy) - (this.group0.zxy * right_anti_dual.group0.yzx)
        );
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4((float3(other.e321) * wedge.group0.xyz), -(other.e235 * wedge.e415) - (other.e315 * wedge.e425) - (other.e125 * wedge.e435))
        );
    }
}
public extension AntiPlane {
    public AntiFlector project_orthogonally_onto(AntiFlector other) {
        Flector right_anti_dual = Flector(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        Motor wedge = Motor(
            /* e415, e425, e435, e12345 */
            float4((float3(right_anti_dual.e45) * this.group0.xyz * float3(-1.0)), (this.e1 * right_anti_dual.e4235) + (this.e2 * right_anti_dual.e4315) + (this.e3 * right_anti_dual.e4125)), 
            /* e235, e315, e125, e5 */
            float4(((this.group0.yzx * right_anti_dual.group0.zxy) - (this.group0.zxy * right_anti_dual.group0.yzx)), 0.0)
        );
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(wedge.e12345) * other.group0, 
            /* e1, e2, e3, e5 */
            float4(other.e1 * wedge.e12345, other.e2 * wedge.e12345, other.e3 * wedge.e12345, -(other.e235 * wedge.e415) - (other.e315 * wedge.e425) - (other.e125 * wedge.e435)) + (wedge.group0 * float4(other.group0.www, other.e5))
        );
    }
}
public extension AntiPlane {
    public AntiPlane project_orthogonally_onto(AntiLine other) {
        Line right_anti_dual = Line(
            /* e415, e425, e435 */
            other.group0.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            other.group1.xyz * float3(-1.0)
        );
        Plane wedge = Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(this.e3 * right_anti_dual.e425, this.e1 * right_anti_dual.e435, this.e2 * right_anti_dual.e415, -(this.e2 * right_anti_dual.e315) - (this.e3 * right_anti_dual.e125)) - (this.group0.yzxx * float4(right_anti_dual.group0.xyz.zxy, right_anti_dual.e235))
        );
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(other.e12 * wedge.e4315 * -1.0, other.e23 * wedge.e4125 * -1.0, other.e31 * wedge.e4235 * -1.0, (other.e25 * wedge.e4315) + (other.e35 * wedge.e4125)) + (wedge.group0.zxyx * float4(other.group0.xyz.yzx, other.e15))
        );
    }
}
public extension AntiPlane {
    public AntiFlector project_orthogonally_onto(AntiMotor other) {
        Motor right_anti_dual = Motor(
            /* e415, e425, e435, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        Flector wedge = Flector(
            /* e15, e25, e35, e45 */
            float4(float3(1.0), 0.0) * float4(right_anti_dual.group1.www, 0.0) * float4(this.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e3 * right_anti_dual.e425, this.e1 * right_anti_dual.e435, this.e2 * right_anti_dual.e415, -(this.e2 * right_anti_dual.e315) - (this.e3 * right_anti_dual.e125)) - (this.group0.yzxx * float4(right_anti_dual.group0.zxy, right_anti_dual.e235))
        );
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(float3(1.0), 0.0) * float4(other.group1.www, 0.0) * float4(wedge.group1.xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0), 
            /* e1, e2, e3, e5 */
            (wedge.group1.zxyx * float4(other.group0.yzx, other.e15)) + float4(float3(0.0), (other.e25 * wedge.e4315) + (other.e35 * wedge.e4125)) - float4((other.group0.zxy * wedge.group1.yzx), other.e3215 * wedge.e45)
        );
    }
}
public extension AntiPlane {
    public AntiPlane project_orthogonally_onto(AntiPlane other) {
        Plane right_anti_dual = Plane(
            /* e4235, e4315, e4125, e3215 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4((this.e1 * right_anti_dual.e4235) + (this.e2 * right_anti_dual.e4315) + (this.e3 * right_anti_dual.e4125)) * other.group0
        );
    }
}
public extension AntiPlane {
    public AntiPlane project_orthogonally_onto(AntiScalar other) {
        Scalar right_anti_dual = Scalar(
            /* scalar */
            other.e12345 * -1.0
        );
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(other.e12345) * float4(this.e1 * right_anti_dual.scalar, this.e2 * right_anti_dual.scalar, this.e3 * right_anti_dual.scalar, this.e5 * right_anti_dual.scalar)
        );
    }
}
public extension AntiPlane {
    public RoundPoint project_orthogonally_onto(Circle other) {
        Dipole right_anti_dual = Dipole(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35 */
            other.group2.xyz
        );
        Circle wedge = Circle(
            /* e423, e431, e412 */
            (right_anti_dual.group0.xyz.yzx * this.group0.zxy) - (right_anti_dual.group0.xyz.zxy * this.group0.yzx), 
            /* e415, e425, e435, e321 */
            float4(this.e5 * right_anti_dual.e41, this.e5 * right_anti_dual.e42, this.e5 * right_anti_dual.e43, -(this.e2 * right_anti_dual.e31) - (this.e3 * right_anti_dual.e12)) - (this.group0.xyzx * right_anti_dual.group1.wwwx), 
            /* e235, e315, e125 */
            (float3(this.e5) * right_anti_dual.group1.xyz) + (right_anti_dual.group2.xyz.zxy * this.group0.yzx) - (right_anti_dual.group2.xyz.yzx * this.group0.zxy)
        );
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4((other.e412 * wedge.e315) + (other.e415 * wedge.e321) + (other.e321 * wedge.e415) + (other.e315 * wedge.e412), (other.e423 * wedge.e125) + (other.e425 * wedge.e321) + (other.e321 * wedge.e425) + (other.e125 * wedge.e423), (other.e431 * wedge.e235) + (other.e435 * wedge.e321) + (other.e321 * wedge.e435) + (other.e235 * wedge.e431), -(other.e412 * wedge.e435) - (other.e415 * wedge.e423) - (other.e425 * wedge.e431) - (other.e435 * wedge.e412)) - float4((other.group0.xyz.yzx * wedge.group2.xyz.zxy), other.e423 * wedge.e415) - float4((other.group2.xyz.zxy * wedge.group0.xyz.yzx), other.e431 * wedge.e425), 
            /* e5 */
            -(other.e415 * wedge.e235) - (other.e425 * wedge.e315) - (other.e435 * wedge.e125) - (other.e235 * wedge.e415) - (other.e315 * wedge.e425) - (other.e125 * wedge.e435)
        );
    }
}
public extension AntiPlane {
    public AntiDipoleInversion project_orthogonally_onto(CircleRotor other) {
        AntiCircleRotor right_anti_dual = AntiCircleRotor(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        AntiDipoleInversion wedge = AntiDipoleInversion(
            /* e423, e431, e412 */
            (right_anti_dual.group0.xyz.yzx * this.group0.zxy) - (right_anti_dual.group0.xyz.zxy * this.group0.yzx), 
            /* e415, e425, e435, e321 */
            float4(right_anti_dual.e41 * this.e5, right_anti_dual.e42 * this.e5, right_anti_dual.e43 * this.e5, -(right_anti_dual.e31 * this.e2) - (right_anti_dual.e12 * this.e3)) - (right_anti_dual.group1.wwwx * this.group0.xyzx), 
            /* e235, e315, e125, e4 */
            float4((float3(this.e5) * right_anti_dual.group1.xyz), 0.0) + float4((right_anti_dual.group2.zxy * this.group0.yzx), 0.0) - float4((right_anti_dual.group2.yzx * this.group0.zxy), 0.0), 
            /* e1, e2, e3, e5 */
            float4(right_anti_dual.scalar) * this.group0
        );
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(other.e12345) * wedge.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(other.e12345) * wedge.group1, 
            /* e235, e315, e125, e4 */
            float4((float3(other.e12345) * wedge.group2.xyz), (wedge.e4 * other.e12345) - (wedge.e423 * other.e415) - (wedge.e431 * other.e425) - (wedge.e412 * other.e435) - (wedge.e415 * other.e423) - (wedge.e425 * other.e431) - (wedge.e435 * other.e412)), 
            /* e1, e2, e3, e5 */
            float4((wedge.e415 * other.e321) + (wedge.e321 * other.e415) + (wedge.e315 * other.e412) + (wedge.e1 * other.e12345), (wedge.e425 * other.e321) + (wedge.e321 * other.e425) + (wedge.e125 * other.e423) + (wedge.e2 * other.e12345), (wedge.e435 * other.e321) + (wedge.e321 * other.e435) + (wedge.e235 * other.e431) + (wedge.e3 * other.e12345), -(wedge.e435 * other.e125) - (wedge.e235 * other.e415) - (wedge.e315 * other.e425) - (wedge.e125 * other.e435)) + (other.group2.yzxw * float4(wedge.group0.xyz.zxy, wedge.e5)) - (other.group2.zxyx * float4(wedge.group0.xyz.yzx, wedge.e415)) - float4((other.group0.xyz.yzx * wedge.group2.zxy), wedge.e425 * other.e315)
        );
    }
}
public extension AntiPlane {
    public RoundPoint project_orthogonally_onto(Dipole other) {
        Circle right_anti_dual = Circle(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125 */
            other.group2.xyz * float3(-1.0)
        );
        Sphere wedge = Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(this.e3 * right_anti_dual.e425, this.e1 * right_anti_dual.e435, this.e2 * right_anti_dual.e415, -(this.e3 * right_anti_dual.e125) - (this.e5 * right_anti_dual.e321)) - (this.group0.wwwx * float4(right_anti_dual.group0.xyz, right_anti_dual.e235)) - float4((this.group0.yzx * right_anti_dual.group1.zxy), this.e2 * right_anti_dual.e315), 
            /* e1234 */
            (this.e1 * right_anti_dual.e423) + (this.e2 * right_anti_dual.e431) + (this.e3 * right_anti_dual.e412)
        );
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4((other.e41 * wedge.e3215) + (other.e31 * wedge.e4125), (other.e42 * wedge.e3215) + (other.e12 * wedge.e4235), (other.e43 * wedge.e3215) + (other.e23 * wedge.e4315), -(other.e43 * wedge.e4125) - (other.e45 * wedge.e1234)) - (wedge.group0.yzxy * float4(other.group1.zxy, other.e42)) - float4((float3(wedge.e1234) * other.group2.xyz), other.e41 * wedge.e4235), 
            /* e5 */
            (other.e45 * wedge.e3215) + (other.e15 * wedge.e4235) + (other.e25 * wedge.e4315) + (other.e35 * wedge.e4125)
        );
    }
}
public extension AntiPlane {
    public AntiDipoleInversion project_orthogonally_onto(DipoleInversion other) {
        AntiDipoleInversion right_anti_dual = AntiDipoleInversion(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e4 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group3 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        DipoleInversion wedge = DipoleInversion(
            /* e41, e42, e43 */
            float3(right_anti_dual.e4) * this.group0.xyz * float3(-1.0), 
            /* e23, e31, e12, e45 */
            float4(((right_anti_dual.group3.zxy * this.group0.yzx) - (right_anti_dual.group3.yzx * this.group0.zxy)), right_anti_dual.e4 * this.e5 * -1.0), 
            /* e15, e25, e35, e1234 */
            float4(right_anti_dual.e1 * this.e5 * -1.0, right_anti_dual.e2 * this.e5 * -1.0, right_anti_dual.e3 * this.e5 * -1.0, (right_anti_dual.e431 * this.e2) + (right_anti_dual.e412 * this.e3)) + (this.group0.xyzx * float4(right_anti_dual.group3.www, right_anti_dual.e423)), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual.e425 * this.e3, right_anti_dual.e435 * this.e1, right_anti_dual.e415 * this.e2, -(right_anti_dual.e315 * this.e2) - (right_anti_dual.e125 * this.e3)) - (float4(this.e5) * float4(right_anti_dual.group0.xyz, right_anti_dual.e321)) - (this.group0.yzxx * float4(right_anti_dual.group1.zxy, right_anti_dual.e235))
        );
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (float3(other.e1234) * wedge.group3.xyz) - (float3(wedge.e1234) * other.group3.xyz), 
            /* e415, e425, e435, e321 */
            (wedge.group3.yzxw * float4(other.group3.zxy, other.e1234)) - (other.group3.yzxw * float4(wedge.group3.zxy, wedge.e1234)), 
            /* e235, e315, e125, e4 */
            float4(float3(0.0), (other.e1234 * wedge.e45) + (other.e4315 * wedge.e42) + (other.e4125 * wedge.e43) - (other.e42 * wedge.e4315) - (other.e43 * wedge.e4125) - (other.e45 * wedge.e1234)) + float4((wedge.group3.www * other.group3.xyz), other.e4235 * wedge.e41) - (wedge.group3.xyzx * float4(other.group3.www, other.e41)), 
            /* e1, e2, e3, e5 */
            (float4(wedge.e3215) * float4(other.group0.xyz, other.e45)) + (other.group2.wwwy * float4(wedge.group2.xyz, wedge.e4315)) + (wedge.group3.zxyx * float4(other.group1.yzx, other.e15)) + float4((other.group3.yzx * wedge.group1.zxy), other.e35 * wedge.e4125) - (other.group3.zxyw * wedge.group1.yzxw) - (wedge.group2.wwwz * float4(other.group2.xyz, other.e4125)) - float4((wedge.group0.xyz * other.group3.www), other.e4235 * wedge.e15) - float4((other.group1.zxy * wedge.group3.yzx), other.e4315 * wedge.e25)
        );
    }
}
public extension AntiPlane {
    public AntiPlane project_orthogonally_onto(DualNum other) {
        AntiDualNum right_anti_dual = AntiDualNum(
            /* e3215, scalar */
            other.group0.xy * float2(-1.0)
        );
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(other.e12345) * float4(right_anti_dual.scalar * this.e1, right_anti_dual.scalar * this.e2, right_anti_dual.scalar * this.e3, right_anti_dual.scalar * this.e5)
        );
    }
}
public extension AntiPlane {
    public DualNum project_orthogonally_onto(FlatPoint other) {
        AntiFlatPoint right_anti_dual = AntiFlatPoint(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return DualNum(
            /* e5, e12345 */
            float2(-(right_anti_dual.e235 * this.e1 * other.e45) - (right_anti_dual.e315 * this.e2 * other.e45) - (right_anti_dual.e125 * this.e3 * other.e45) - (right_anti_dual.e321 * this.e5 * other.e45), 1.0) * float2(1.0, 0.0)
        );
    }
}
public extension AntiPlane {
    public AntiFlector project_orthogonally_onto(Flector other) {
        AntiFlector right_anti_dual = AntiFlector(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        AntiMotor wedge = AntiMotor(
            /* e23, e31, e12, scalar */
            float4(((right_anti_dual.group1.zxy * this.group0.yzx) - (right_anti_dual.group1.yzx * this.group0.zxy)), 0.0), 
            /* e15, e25, e35, e3215 */
            float4(right_anti_dual.e5 * this.e1, right_anti_dual.e5 * this.e2, right_anti_dual.e5 * this.e3, -(right_anti_dual.e315 * this.e2) - (right_anti_dual.e125 * this.e3) - (right_anti_dual.e321 * this.e5)) - (this.group0.wwwx * float4(right_anti_dual.group1.xyz, right_anti_dual.e235))
        );
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(float3(1.0), 0.0) * float4(wedge.group1.www, 0.0) * float4(other.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), -(wedge.e25 * other.e4315) - (wedge.e35 * other.e4125)) + float4((wedge.group0.zxy * other.group1.yzx), wedge.e3215 * other.e45) - (other.group1.zxyx * float4(wedge.group0.yzx, wedge.e15))
        );
    }
}
public extension AntiPlane {
    public AntiPlane project_orthogonally_onto(Line other) {
        AntiLine right_anti_dual = AntiLine(
            /* e23, e31, e12 */
            other.group0.xyz, 
            /* e15, e25, e35 */
            other.group1.xyz
        );
        AntiFlatPoint wedge = AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4((right_anti_dual.e23 * this.e5) + (right_anti_dual.e35 * this.e2), (right_anti_dual.e31 * this.e5) + (right_anti_dual.e15 * this.e3), (right_anti_dual.e12 * this.e5) + (right_anti_dual.e25 * this.e1), -(right_anti_dual.e31 * this.e2) - (right_anti_dual.e12 * this.e3)) - (this.group0.zxyx * float4(right_anti_dual.group1.xyz.yzx, right_anti_dual.e23))
        );
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4((float3(wedge.e321) * other.group0.xyz), -(wedge.e235 * other.e415) - (wedge.e315 * other.e425) - (wedge.e125 * other.e435))
        );
    }
}
public extension AntiPlane {
    public AntiFlector project_orthogonally_onto(Motor other) {
        AntiMotor right_anti_dual = AntiMotor(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        AntiFlector wedge = AntiFlector(
            /* e235, e315, e125, e321 */
            float4((right_anti_dual.e23 * this.e5) + (right_anti_dual.e35 * this.e2), (right_anti_dual.e31 * this.e5) + (right_anti_dual.e15 * this.e3), (right_anti_dual.e12 * this.e5) + (right_anti_dual.e25 * this.e1), -(right_anti_dual.e31 * this.e2) - (right_anti_dual.e12 * this.e3)) - (this.group0.zxyx * float4(right_anti_dual.group1.yzx, right_anti_dual.e23)), 
            /* e1, e2, e3, e5 */
            float4(right_anti_dual.scalar) * this.group0
        );
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(other.e12345) * wedge.group0, 
            /* e1, e2, e3, e5 */
            float4(wedge.e1 * other.e12345, wedge.e2 * other.e12345, wedge.e3 * other.e12345, -(wedge.e235 * other.e415) - (wedge.e315 * other.e425) - (wedge.e125 * other.e435)) + (other.group0 * float4(wedge.group0.www, wedge.e5))
        );
    }
}
public extension AntiPlane {
    public MultiVector project_orthogonally_onto(MultiVector other) {
        MultiVector right_anti_dual = MultiVector(
            /* scalar, e12345 */
            other.group0.xy.yx * float2(-1.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4(other.group9.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5 */
            other.e3215, 
            /* e15, e25, e35, e45 */
            float4(other.group8.xyz, other.e321 * -1.0), 
            /* e41, e42, e43 */
            other.group7.xyz, 
            /* e23, e31, e12 */
            other.group6.xyz, 
            /* e415, e425, e435, e321 */
            float4(other.group5.xyz, other.e45) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e423, e431, e412 */
            other.group4.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            other.group3.xyz * float3(-1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.group1.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234 */
            other.e4 * -1.0
        );
        MultiVector wedge = MultiVector(
            /* scalar, e12345 */
            float2(0.0, (this.e1 * right_anti_dual.e4235) + (this.e2 * right_anti_dual.e4315) + (this.e3 * right_anti_dual.e4125) + (this.e5 * right_anti_dual.e1234)), 
            /* e1, e2, e3, e4 */
            float4(right_anti_dual.group0.xy.xx, right_anti_dual.scalar, 0.0) * float4(float3(1.0), 0.0) * float4(this.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e5 */
            this.e5 * right_anti_dual.scalar, 
            /* e15, e25, e35, e45 */
            float4(((float3(right_anti_dual.e5) * this.group0.xyz) - (float3(this.e5) * right_anti_dual.group1.xyz)), this.e5 * right_anti_dual.e4 * -1.0), 
            /* e41, e42, e43 */
            float3(right_anti_dual.e4) * this.group0.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            (this.group0.yzx * right_anti_dual.group1.zxy) - (this.group0.zxy * right_anti_dual.group1.yzx), 
            /* e415, e425, e435, e321 */
            float4(this.e5 * right_anti_dual.e41, this.e5 * right_anti_dual.e42, this.e5 * right_anti_dual.e43, -(this.e2 * right_anti_dual.e31) - (this.e3 * right_anti_dual.e12)) - float4((right_anti_dual.group3.www * this.group0.xyz), this.e1 * right_anti_dual.e23), 
            /* e423, e431, e412 */
            (right_anti_dual.group4.xyz.yzx * this.group0.zxy) - (right_anti_dual.group4.xyz.zxy * this.group0.yzx), 
            /* e235, e315, e125 */
            (float3(this.e5) * right_anti_dual.group5.xyz) + (this.group0.yzx * right_anti_dual.group3.zxy) - (this.group0.zxy * right_anti_dual.group3.yzx), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e3 * right_anti_dual.e425, this.e1 * right_anti_dual.e435, this.e2 * right_anti_dual.e415, -(this.e3 * right_anti_dual.e125) - (this.e5 * right_anti_dual.e321)) - (this.group0.wwwx * float4(right_anti_dual.group7.xyz, right_anti_dual.e235)) - float4((this.group0.yzx * right_anti_dual.group6.zxy), this.e2 * right_anti_dual.e315), 
            /* e1234 */
            (this.e1 * right_anti_dual.e423) + (this.e2 * right_anti_dual.e431) + (this.e3 * right_anti_dual.e412)
        );
        return MultiVector(
            /* scalar, e12345 */
            float2((other.scalar * wedge.e12345) + (other.e12345 * wedge.scalar) + (other.e1 * wedge.e4235) + (other.e2 * wedge.e4315) + (other.e3 * wedge.e4125) + (other.e4 * wedge.e3215) + (other.e5 * wedge.e1234) + (other.e4235 * wedge.e1) + (other.e4315 * wedge.e2) + (other.e4125 * wedge.e3) + (other.e3215 * wedge.e4) + (other.e1234 * wedge.e5) - (other.e15 * wedge.e423) - (other.e25 * wedge.e431) - (other.e35 * wedge.e412) - (other.e45 * wedge.e321) - (other.e41 * wedge.e235) - (other.e42 * wedge.e315) - (other.e43 * wedge.e125) - (other.e23 * wedge.e415) - (other.e31 * wedge.e425) - (other.e12 * wedge.e435) - (other.e415 * wedge.e23) - (other.e425 * wedge.e31) - (other.e435 * wedge.e12) - (other.e321 * wedge.e45) - (other.e423 * wedge.e15) - (other.e431 * wedge.e25) - (other.e412 * wedge.e35) - (other.e235 * wedge.e41) - (other.e315 * wedge.e42) - (other.e125 * wedge.e43), other.e12345 * wedge.e12345), 
            /* e1, e2, e3, e4 */
            float4((other.e415 * wedge.e321) + (other.e321 * wedge.e415) + (other.e4315 * wedge.e12) + (other.e1234 * wedge.e15), (other.e425 * wedge.e321) + (other.e321 * wedge.e425) + (other.e4125 * wedge.e23) + (other.e1234 * wedge.e25), (other.e435 * wedge.e321) + (other.e321 * wedge.e435) + (other.e4235 * wedge.e31) + (other.e1234 * wedge.e35), -(other.e45 * wedge.e1234) - (other.e415 * wedge.e423) - (other.e425 * wedge.e431) - (other.e435 * wedge.e412)) + (float4(other.e12345) * wedge.group1) + (float4(wedge.e12345) * other.group1) + float4((other.group4.xyz * wedge.group9.www), other.e4235 * wedge.e41) + float4((other.group5.xyz.yzx * wedge.group9.zxy), other.e4315 * wedge.e42) + float4((other.group7.xyz.zxy * wedge.group8.xyz.yzx), other.e4125 * wedge.e43) + float4((other.group8.xyz.yzx * wedge.group7.xyz.zxy), other.e1234 * wedge.e45) - (wedge.group9.yzxx * float4(other.group5.xyz.zxy, other.e41)) - float4((float3(wedge.e1234) * other.group3.xyz), other.e412 * wedge.e435) - float4((wedge.group4.xyz * other.group9.www), other.e423 * wedge.e415) - float4((other.group7.xyz.yzx * wedge.group8.xyz.zxy), other.e42 * wedge.e4315) - float4((other.group8.xyz.zxy * wedge.group7.xyz.yzx), other.e43 * wedge.e4125) - float4((wedge.group5.xyz.yzx * other.group9.zxy), other.e431 * wedge.e425), 
            /* e5 */
            (other.e12345 * wedge.e5) + (other.e5 * wedge.e12345) + (other.e15 * wedge.e4235) + (other.e25 * wedge.e4315) + (other.e35 * wedge.e4125) + (other.e45 * wedge.e3215) - (other.e415 * wedge.e235) - (other.e425 * wedge.e315) - (other.e435 * wedge.e125) - (other.e235 * wedge.e415) - (other.e315 * wedge.e425) - (other.e125 * wedge.e435) - (other.e4235 * wedge.e15) - (other.e4315 * wedge.e25) - (other.e4125 * wedge.e35) - (other.e3215 * wedge.e45), 
            /* e15, e25, e35, e45 */
            float4((other.e415 * wedge.e3215) + (other.e315 * wedge.e4125) + (other.e4125 * wedge.e315) + (other.e3215 * wedge.e415), (other.e425 * wedge.e3215) + (other.e125 * wedge.e4235) + (other.e4235 * wedge.e125) + (other.e3215 * wedge.e425), (other.e435 * wedge.e3215) + (other.e235 * wedge.e4315) + (other.e4315 * wedge.e235) + (other.e3215 * wedge.e435), -(other.e435 * wedge.e4125) - (other.e4235 * wedge.e415) - (other.e4315 * wedge.e425) - (other.e4125 * wedge.e435)) + (float4(other.e12345) * wedge.group3) + (float4(wedge.e12345) * other.group3) - (wedge.group9.yzxx * float4(other.group8.xyz.zxy, other.e415)) - float4((wedge.group8.xyz.zxy * other.group9.yzx), other.e425 * wedge.e4315), 
            /* e41, e42, e43 */
            (float3(other.e12345) * wedge.group4.xyz) + (float3(other.e1234) * wedge.group6.xyz) + (float3(wedge.e12345) * other.group4.xyz) + (float3(wedge.e1234) * other.group6.xyz) + (other.group7.xyz.zxy * wedge.group9.yzx) + (wedge.group7.xyz.zxy * other.group9.yzx) - (other.group7.xyz.yzx * wedge.group9.zxy) - (wedge.group7.xyz.yzx * other.group9.zxy), 
            /* e23, e31, e12 */
            (float3(other.e12345) * wedge.group5.xyz) + (float3(other.e3215) * wedge.group7.xyz) + (float3(other.e1234) * wedge.group8.xyz) + (float3(wedge.e12345) * other.group5.xyz) + (float3(wedge.e3215) * other.group7.xyz) + (float3(wedge.e1234) * other.group8.xyz) - (float3(other.e321) * wedge.group9.xyz) - (float3(wedge.e321) * other.group9.xyz), 
            /* e415, e425, e435, e321 */
            (float4(other.e12345) * wedge.group6) + (float4(wedge.e12345) * other.group6) + float4((other.group9.zxy * wedge.group9.yzx), other.e1234 * wedge.e3215) - (other.group9.yzxw * float4(wedge.group9.zxy, wedge.e1234)), 
            /* e423, e431, e412 */
            (float3(other.e12345) * wedge.group7.xyz) + (float3(other.e1234) * wedge.group9.xyz) + (float3(wedge.e12345) * other.group7.xyz) - (float3(wedge.e1234) * other.group9.xyz), 
            /* e235, e315, e125 */
            (float3(other.e12345) * wedge.group8.xyz) + (float3(wedge.e12345) * other.group8.xyz) + (float3(wedge.e3215) * other.group9.xyz) - (float3(other.e3215) * wedge.group9.xyz), 
            /* e4235, e4315, e4125, e3215 */
            (float4(other.e12345) * wedge.group9) + (float4(wedge.e12345) * other.group9), 
            /* e1234 */
            (other.e12345 * wedge.e1234) + (other.e1234 * wedge.e12345)
        );
    }
}
public extension AntiPlane {
    public AntiPlane project_orthogonally_onto(Plane other) {
        AntiPlane right_anti_dual = AntiPlane(
            /* e1, e2, e3, e5 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        AntiLine wedge = AntiLine(
            /* e23, e31, e12 */
            (right_anti_dual.group0.zxy * this.group0.yzx) - (right_anti_dual.group0.yzx * this.group0.zxy), 
            /* e15, e25, e35 */
            (float3(right_anti_dual.e5) * this.group0.xyz) - (float3(this.e5) * right_anti_dual.group0.xyz)
        );
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(wedge.e12 * other.e4315, wedge.e23 * other.e4125, wedge.e31 * other.e4235, -(wedge.e25 * other.e4315) - (wedge.e35 * other.e4125)) - (other.group0.zxyx * float4(wedge.group0.xyz.yzx, wedge.e15))
        );
    }
}
public extension AntiPlane {
    public RoundPoint project_orthogonally_onto(RoundPoint other) {
        Sphere right_anti_dual = Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234 */
            other.e4 * -1.0
        );
        AntiScalar wedge = AntiScalar(
            /* e12345 */
            (this.e1 * right_anti_dual.e4235) + (this.e2 * right_anti_dual.e4315) + (this.e3 * right_anti_dual.e4125) + (this.e5 * right_anti_dual.e1234)
        );
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(wedge.e12345) * other.group0, 
            /* e5 */
            wedge.e12345 * other.e5
        );
    }
}
public extension AntiPlane {
    public RoundPoint project_orthogonally_onto(Sphere other) {
        RoundPoint right_anti_dual = RoundPoint(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5 */
            other.e3215
        );
        Dipole wedge = Dipole(
            /* e41, e42, e43 */
            float3(right_anti_dual.e4) * this.group0.xyz * float3(-1.0), 
            /* e23, e31, e12, e45 */
            float4(((this.group0.yzx * right_anti_dual.group0.zxy) - (this.group0.zxy * right_anti_dual.group0.yzx)), this.e5 * right_anti_dual.e4 * -1.0), 
            /* e15, e25, e35 */
            (float3(right_anti_dual.e5) * this.group0.xyz) - (float3(this.e5) * right_anti_dual.group0.xyz)
        );
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(-(wedge.e41 * other.e3215) - (wedge.e31 * other.e4125), -(wedge.e42 * other.e3215) - (wedge.e12 * other.e4235), -(wedge.e43 * other.e3215) - (wedge.e23 * other.e4315), (wedge.e43 * other.e4125) + (wedge.e45 * other.e1234)) + (other.group0.yzxy * float4(wedge.group1.zxy, wedge.e42)) + float4((float3(other.e1234) * wedge.group2.xyz), wedge.e41 * other.e4235), 
            /* e5 */
            -(wedge.e45 * other.e3215) - (wedge.e15 * other.e4235) - (wedge.e25 * other.e4315) - (wedge.e35 * other.e4125)
        );
    }
}
public extension AntiPlane {
    public VersorEven project_orthogonally_onto(VersorEven other) {
        VersorOdd right_anti_dual = VersorOdd(
            /* e41, e42, e43, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e1234 */
            float4(other.group2.xyz, other.e4) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.group3.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0)
        );
        VersorEven wedge = VersorEven(
            /* e423, e431, e412, e12345 */
            float4(this.e2 * right_anti_dual.e43 * -1.0, this.e3 * right_anti_dual.e41 * -1.0, this.e1 * right_anti_dual.e42 * -1.0, (this.e2 * right_anti_dual.e4315) + (this.e3 * right_anti_dual.e4125) + (this.e5 * right_anti_dual.e1234)) + (this.group0.zxyx * float4(right_anti_dual.group0.yzx, right_anti_dual.e4235)), 
            /* e415, e425, e435, e321 */
            float4(this.e5 * right_anti_dual.e41, this.e5 * right_anti_dual.e42, this.e5 * right_anti_dual.e43, -(this.e2 * right_anti_dual.e31) - (this.e3 * right_anti_dual.e12)) - (this.group0.xyzx * right_anti_dual.group1.wwwx), 
            /* e235, e315, e125, e5 */
            float4(((float3(this.e5) * right_anti_dual.group1.xyz) + (this.group0.yzx * right_anti_dual.group2.zxy) - (this.group0.zxy * right_anti_dual.group2.yzx)), this.e5 * right_anti_dual.scalar), 
            /* e1, e2, e3, e4 */
            float4(float3(1.0), 0.0) * float4(right_anti_dual.group0.www, 0.0) * float4(this.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(((float3(other.e12345) * wedge.group0.xyz) + (float3(wedge.e12345) * other.group0.xyz)), other.e12345 * wedge.e12345), 
            /* e415, e425, e435, e321 */
            (float4(other.e12345) * wedge.group1) + (float4(wedge.e12345) * other.group1), 
            /* e235, e315, e125, e5 */
            (float4(other.e12345) * wedge.group2) + (float4(wedge.e12345) * other.group2) + float4(float3(0.0), -(other.e415 * wedge.e235) - (other.e425 * wedge.e315) - (other.e435 * wedge.e125) - (other.e235 * wedge.e415) - (other.e315 * wedge.e425) - (other.e125 * wedge.e435)), 
            /* e1, e2, e3, e4 */
            float4((other.e415 * wedge.e321) + (other.e321 * wedge.e415) + (other.e315 * wedge.e412) + (other.e1 * wedge.e12345), (other.e425 * wedge.e321) + (other.e321 * wedge.e425) + (other.e125 * wedge.e423) + (other.e2 * wedge.e12345), (other.e435 * wedge.e321) + (other.e321 * wedge.e435) + (other.e235 * wedge.e431) + (other.e3 * wedge.e12345), -(other.e412 * wedge.e435) - (other.e415 * wedge.e423) - (other.e425 * wedge.e431) - (other.e435 * wedge.e412)) + (other.group0.zxyw * float4(wedge.group2.yzx, wedge.e4)) + float4((other.group0.www * wedge.group3.xyz), other.e4 * wedge.e12345) - (other.group0.yzxx * float4(wedge.group2.zxy, wedge.e415)) - float4((other.group2.zxy * wedge.group0.yzx), other.e431 * wedge.e425)
        );
    }
}
public extension AntiPlane {
    public AntiDipoleInversion project_orthogonally_onto(VersorOdd other) {
        VersorEven right_anti_dual = VersorEven(
            /* e423, e431, e412, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            float4(other.group2.xyz, other.e3215) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4(other.group3.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0)
        );
        DipoleInversion wedge = DipoleInversion(
            /* e41, e42, e43 */
            float3(right_anti_dual.e4) * this.group0.xyz * float3(-1.0), 
            /* e23, e31, e12, e45 */
            float4(((this.group0.yzx * right_anti_dual.group3.zxy) - (this.group0.zxy * right_anti_dual.group3.yzx)), this.e5 * right_anti_dual.e4 * -1.0), 
            /* e15, e25, e35, e1234 */
            float4(this.e5 * right_anti_dual.e1 * -1.0, this.e5 * right_anti_dual.e2 * -1.0, this.e5 * right_anti_dual.e3 * -1.0, (this.e2 * right_anti_dual.e431) + (this.e3 * right_anti_dual.e412)) + (this.group0.xyzx * float4(right_anti_dual.group2.www, right_anti_dual.e423)), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e3 * right_anti_dual.e425, this.e1 * right_anti_dual.e435, this.e2 * right_anti_dual.e415, -(this.e3 * right_anti_dual.e125) - (this.e5 * right_anti_dual.e321)) - (this.group0.yzxx * float4(right_anti_dual.group1.zxy, right_anti_dual.e235)) - (this.group0.wwwy * float4(right_anti_dual.group0.xyz, right_anti_dual.e315))
        );
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (float3(other.e1234) * wedge.group3.xyz) - (float3(wedge.e1234) * other.group3.xyz), 
            /* e415, e425, e435, e321 */
            (wedge.group3.yzxw * float4(other.group3.zxy, other.e1234)) - (other.group3.yzxw * float4(wedge.group3.zxy, wedge.e1234)), 
            /* e235, e315, e125, e4 */
            (other.group3.xyzx * float4(wedge.group3.www, wedge.e41)) + float4(float3(0.0), (wedge.e42 * other.e4315) + (wedge.e43 * other.e4125) + (wedge.e45 * other.e1234) - (wedge.e4235 * other.e41) - (wedge.e4315 * other.e42) - (wedge.e4125 * other.e43)) - float4((other.group3.www * wedge.group3.xyz), wedge.e1234 * other.e45), 
            /* e1, e2, e3, e5 */
            (float4(wedge.e3215) * float4(other.group0.xyz, other.e45)) + (wedge.group3.zxyz * float4(other.group1.yzx, other.e35)) + (other.group2.wwwy * float4(wedge.group2.xyz, wedge.e4315)) + float4((wedge.group1.zxy * other.group3.yzx), wedge.e4235 * other.e15) - (float4(other.e3215) * float4(wedge.group0.xyz, wedge.e45)) - (wedge.group2.wwwy * float4(other.group2.xyz, other.e4315)) - (other.group3.zxyx * float4(wedge.group1.yzx, wedge.e15)) - float4((wedge.group3.yzx * other.group1.zxy), wedge.e35 * other.e4125)
        );
    }
}
public extension AntiScalar {
    // Fancy infix trick (first half)
    public ProjectOrthogonallyOntoInfixPartial<AntiScalar> operator /(ProjectOrthogonallyOntoInfix rhs) {
        return ProjectOrthogonallyOntoInfixPartial<AntiScalar>(this);
    }
}
public extension ProjectOrthogonallyOntoInfixPartial<AntiScalar> {
    // Fancy infix trick (second half)
    public AntiScalar operator /(AntiScalar rhs) {
        return this.a.project_orthogonally_onto(rhs);
    }
}
public extension AntiScalar {
    public AntiScalar project_orthogonally_onto(AntiScalar other) {
        return AntiScalar(
            /* e12345 */
            pow(other.e12345, 2) * this.e12345 * -1.0
        );
    }
}
public extension AntiScalar {
    public CircleRotor project_orthogonally_onto(CircleRotor other) {
        AntiScalar wedge = AntiScalar(
            /* e12345 */
            (other.group2 * float4(1.0, 1.0, 1.0, -1.0))[3] * this.e12345
        );
        return CircleRotor(
            /* e423, e431, e412 */
            float3(wedge.e12345) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(wedge.e12345) * other.group1, 
            /* e235, e315, e125, e12345 */
            float4(wedge.e12345) * other.group2
        );
    }
}
public extension AntiScalar {
    public DualNum project_orthogonally_onto(DualNum other) {
        return DualNum(
            /* e5, e12345 */
            float2(this.e12345 * other.e12345 * -1.0) * other.group0.xy
        );
    }
}
public extension AntiScalar {
    public Motor project_orthogonally_onto(Motor other) {
        AntiScalar wedge = AntiScalar(
            /* e12345 */
            (other.group0 * float4(1.0, 1.0, 1.0, -1.0))[3] * this.e12345
        );
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(wedge.e12345) * other.group0, 
            /* e235, e315, e125, e5 */
            float4(wedge.e12345) * other.group1
        );
    }
}
public extension AntiScalar {
    public MultiVector project_orthogonally_onto(MultiVector other) {
        AntiScalar wedge = AntiScalar(
            /* e12345 */
            (other.group0.xy.yx * float2(-1.0, 1.0))[0] * this.e12345
        );
        return MultiVector(
            /* scalar, e12345 */
            float2(wedge.e12345) * other.group0.xy, 
            /* e1, e2, e3, e4 */
            float4(wedge.e12345) * other.group1, 
            /* e5 */
            wedge.e12345 * other.e5, 
            /* e15, e25, e35, e45 */
            float4(wedge.e12345) * other.group3, 
            /* e41, e42, e43 */
            float3(wedge.e12345) * other.group4.xyz, 
            /* e23, e31, e12 */
            float3(wedge.e12345) * other.group5.xyz, 
            /* e415, e425, e435, e321 */
            float4(wedge.e12345) * other.group6, 
            /* e423, e431, e412 */
            float3(wedge.e12345) * other.group7.xyz, 
            /* e235, e315, e125 */
            float3(wedge.e12345) * other.group8.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(wedge.e12345) * other.group9, 
            /* e1234 */
            wedge.e12345 * other.e1234
        );
    }
}
public extension AntiScalar {
    public VersorEven project_orthogonally_onto(VersorEven other) {
        AntiScalar wedge = AntiScalar(
            /* e12345 */
            (other.group0 * float4(1.0, 1.0, 1.0, -1.0))[3] * this.e12345
        );
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(wedge.e12345) * other.group0, 
            /* e415, e425, e435, e321 */
            float4(wedge.e12345) * other.group1, 
            /* e235, e315, e125, e5 */
            float4(wedge.e12345) * other.group2, 
            /* e1, e2, e3, e4 */
            float4(wedge.e12345) * other.group3
        );
    }
}
public extension Circle {
    // Fancy infix trick (first half)
    public ProjectOrthogonallyOntoInfixPartial<Circle> operator /(ProjectOrthogonallyOntoInfix rhs) {
        return ProjectOrthogonallyOntoInfixPartial<Circle>(this);
    }
}
public extension ProjectOrthogonallyOntoInfixPartial<Circle> {
    // Fancy infix trick (second half)
    public AntiDipoleInversion operator /(AntiDipoleInversion rhs) {
        return this.a.project_orthogonally_onto(rhs);
    }
}
public extension Circle {
    public AntiDipoleInversion project_orthogonally_onto(AntiDipoleInversion other) {
        DipoleInversion right_anti_dual = DipoleInversion(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e1234 */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group3 * float4(1.0, 1.0, 1.0, -1.0)
        );
        AntiScalar wedge = AntiScalar(
            /* e12345 */
            -(this.e423 * right_anti_dual.e15) - (this.e431 * right_anti_dual.e25) - (this.e412 * right_anti_dual.e35) - (this.e415 * right_anti_dual.e23) - (this.e425 * right_anti_dual.e31) - (this.e435 * right_anti_dual.e12) - (this.e321 * right_anti_dual.e45) - (this.e235 * right_anti_dual.e41) - (this.e315 * right_anti_dual.e42) - (this.e125 * right_anti_dual.e43)
        );
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(wedge.e12345) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(wedge.e12345) * other.group1, 
            /* e235, e315, e125, e4 */
            float4(wedge.e12345) * other.group2, 
            /* e1, e2, e3, e5 */
            float4(wedge.e12345) * other.group3
        );
    }
}
public extension Circle {
    public AntiFlatPoint project_orthogonally_onto(AntiDualNum other) {
        DualNum right_anti_dual = DualNum(
            /* e5, e12345 */
            other.group0.xy
        );
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(other.group0.xy.xx, other.e3215, 0.0) * float4(float3(1.0), 0.0) * float4(float4(this.e423 * right_anti_dual.e5, this.e431 * right_anti_dual.e5, this.e412 * right_anti_dual.e5, this.e321 * right_anti_dual.e5).xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0)
        );
    }
}
public extension Circle {
    public AntiFlatPoint project_orthogonally_onto(AntiFlatPoint other) {
        FlatPoint right_anti_dual = FlatPoint(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(-(this.e423 * right_anti_dual.e15) - (this.e431 * right_anti_dual.e25) - (this.e412 * right_anti_dual.e35) - (this.e321 * right_anti_dual.e45)) * other.group0
        );
    }
}
public extension Circle {
    public AntiFlector project_orthogonally_onto(AntiFlector other) {
        Flector right_anti_dual = Flector(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        AntiScalar wedge = AntiScalar(
            /* e12345 */
            -(this.e423 * right_anti_dual.e15) - (this.e431 * right_anti_dual.e25) - (this.e412 * right_anti_dual.e35) - (this.e321 * right_anti_dual.e45)
        );
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(wedge.e12345) * other.group0, 
            /* e1, e2, e3, e5 */
            float4(wedge.e12345) * other.group1
        );
    }
}
public extension Circle {
    public AntiFlector project_orthogonally_onto(AntiMotor other) {
        Plane wedge = Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(other.e3215) * float4(this.group0.xyz, this.e321)
        );
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(float3(1.0), 0.0) * float4(other.group1.www, 0.0) * float4(wedge.group0.xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0), 
            /* e1, e2, e3, e5 */
            float4(other.e12 * wedge.e4315 * -1.0, other.e23 * wedge.e4125 * -1.0, other.e31 * wedge.e4235 * -1.0, (other.e25 * wedge.e4315) + (other.e35 * wedge.e4125)) + (wedge.group0.zxyx * float4(other.group0.yzx, other.e15))
        );
    }
}
public extension Circle {
    public Circle project_orthogonally_onto(AntiScalar other) {
        Scalar right_anti_dual = Scalar(
            /* scalar */
            other.e12345 * -1.0
        );
        Circle wedge = Circle(
            /* e423, e431, e412 */
            float3(right_anti_dual.scalar) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(right_anti_dual.scalar) * this.group1, 
            /* e235, e315, e125 */
            float3(right_anti_dual.scalar) * this.group2.xyz
        );
        return Circle(
            /* e423, e431, e412 */
            float3(other.e12345) * wedge.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(other.e12345) * wedge.group1, 
            /* e235, e315, e125 */
            float3(other.e12345) * wedge.group2.xyz
        );
    }
}
public extension Circle {
    public Circle project_orthogonally_onto(Circle other) {
        Dipole right_anti_dual = Dipole(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35 */
            other.group2.xyz
        );
        AntiScalar wedge = AntiScalar(
            /* e12345 */
            -(this.e423 * right_anti_dual.e15) - (this.e431 * right_anti_dual.e25) - (this.e412 * right_anti_dual.e35) - (this.e415 * right_anti_dual.e23) - (this.e425 * right_anti_dual.e31) - (this.e435 * right_anti_dual.e12) - (this.e321 * right_anti_dual.e45) - (this.e235 * right_anti_dual.e41) - (this.e315 * right_anti_dual.e42) - (this.e125 * right_anti_dual.e43)
        );
        return Circle(
            /* e423, e431, e412 */
            float3(wedge.e12345) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(wedge.e12345) * other.group1, 
            /* e235, e315, e125 */
            float3(wedge.e12345) * other.group2.xyz
        );
    }
}
public extension Circle {
    public VersorEven project_orthogonally_onto(CircleRotor other) {
        AntiCircleRotor right_anti_dual = AntiCircleRotor(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        CircleRotor wedge = CircleRotor(
            /* e423, e431, e412 */
            float3(right_anti_dual.scalar) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(right_anti_dual.scalar) * this.group1, 
            /* e235, e315, e125, e12345 */
            float4((float3(right_anti_dual.scalar) * this.group2.xyz), -(right_anti_dual.e41 * this.e235) - (right_anti_dual.e42 * this.e315) - (right_anti_dual.e43 * this.e125) - (right_anti_dual.e23 * this.e415) - (right_anti_dual.e31 * this.e425) - (right_anti_dual.e12 * this.e435) - (right_anti_dual.e45 * this.e321) - (right_anti_dual.e15 * this.e423) - (right_anti_dual.e25 * this.e431) - (right_anti_dual.e35 * this.e412))
        );
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(((float3(other.e12345) * wedge.group0.xyz) + (float3(wedge.e12345) * other.group0.xyz)), other.e12345 * wedge.e12345), 
            /* e415, e425, e435, e321 */
            (float4(other.e12345) * wedge.group1) + (float4(wedge.e12345) * other.group1), 
            /* e235, e315, e125, e5 */
            float4((other.e235 * wedge.e12345) + (other.e12345 * wedge.e235), (other.e315 * wedge.e12345) + (other.e12345 * wedge.e315), (other.e125 * wedge.e12345) + (other.e12345 * wedge.e125), -(other.e415 * wedge.e235) - (other.e425 * wedge.e315) - (other.e435 * wedge.e125) - (other.e235 * wedge.e415) - (other.e315 * wedge.e425) - (other.e125 * wedge.e435)), 
            /* e1, e2, e3, e4 */
            float4((other.e412 * wedge.e315) + (other.e415 * wedge.e321) + (other.e321 * wedge.e415) + (other.e315 * wedge.e412), (other.e423 * wedge.e125) + (other.e425 * wedge.e321) + (other.e321 * wedge.e425) + (other.e125 * wedge.e423), (other.e431 * wedge.e235) + (other.e435 * wedge.e321) + (other.e321 * wedge.e435) + (other.e235 * wedge.e431), -(other.e412 * wedge.e435) - (other.e415 * wedge.e423) - (other.e425 * wedge.e431) - (other.e435 * wedge.e412)) - float4((other.group0.xyz.yzx * wedge.group2.zxy), other.e423 * wedge.e415) - float4((wedge.group0.xyz.yzx * other.group2.zxy), other.e431 * wedge.e425)
        );
    }
}
public extension Circle {
    public AntiDipoleInversion project_orthogonally_onto(DipoleInversion other) {
        AntiDipoleInversion right_anti_dual = AntiDipoleInversion(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e4 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group3 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        Sphere wedge = Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(-(right_anti_dual.e4 * this.e235) - (right_anti_dual.e3 * this.e425), -(right_anti_dual.e4 * this.e315) - (right_anti_dual.e1 * this.e435), -(right_anti_dual.e4 * this.e125) - (right_anti_dual.e2 * this.e415), (right_anti_dual.e3 * this.e125) + (right_anti_dual.e5 * this.e321)) + (right_anti_dual.group3.wwwx * float4(this.group0.xyz, this.e235)) + float4((right_anti_dual.group3.yzx * this.group1.zxy), right_anti_dual.e2 * this.e315), 
            /* e1234 */
            -(right_anti_dual.e4 * this.e321) - (right_anti_dual.e1 * this.e423) - (right_anti_dual.e2 * this.e431) - (right_anti_dual.e3 * this.e412)
        );
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (float3(other.e1234) * wedge.group0.xyz) - (float3(wedge.e1234) * other.group3.xyz), 
            /* e415, e425, e435, e321 */
            (wedge.group0.yzxw * float4(other.group3.zxy, other.e1234)) - (other.group3.yzxw * float4(wedge.group0.zxy, wedge.e1234)), 
            /* e235, e315, e125, e4 */
            float4(other.e4235 * wedge.e3215, other.e4315 * wedge.e3215, other.e4125 * wedge.e3215, -(other.e42 * wedge.e4315) - (other.e43 * wedge.e4125) - (other.e45 * wedge.e1234)) - (wedge.group0.xyzx * float4(other.group3.www, other.e41)), 
            /* e1, e2, e3, e5 */
            float4(-(other.e12 * wedge.e4315) - (other.e15 * wedge.e1234), -(other.e23 * wedge.e4125) - (other.e25 * wedge.e1234), -(other.e31 * wedge.e4235) - (other.e35 * wedge.e1234), (other.e25 * wedge.e4315) + (other.e35 * wedge.e4125)) + (float4(wedge.e3215) * float4(other.group0.xyz, other.e45)) + (wedge.group0.zxyx * float4(other.group1.yzx, other.e15))
        );
    }
}
public extension Circle {
    public Circle project_orthogonally_onto(DualNum other) {
        AntiDualNum right_anti_dual = AntiDualNum(
            /* e3215, scalar */
            other.group0.xy * float2(-1.0)
        );
        Circle wedge = Circle(
            /* e423, e431, e412 */
            float3(right_anti_dual.scalar) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(right_anti_dual.scalar) * this.group1, 
            /* e235, e315, e125 */
            float3(right_anti_dual.scalar) * this.group2.xyz
        );
        return Circle(
            /* e423, e431, e412 */
            float3(other.e12345) * wedge.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(other.e12345) * wedge.group1, 
            /* e235, e315, e125 */
            float3(other.e12345) * wedge.group2.xyz
        );
    }
}
public extension Circle {
    public AntiDipoleInversion project_orthogonally_onto(Flector other) {
        AntiFlector right_anti_dual = AntiFlector(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        Sphere wedge = Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual.e3 * this.e425 * -1.0, right_anti_dual.e1 * this.e435 * -1.0, right_anti_dual.e2 * this.e415 * -1.0, (right_anti_dual.e3 * this.e125) + (right_anti_dual.e5 * this.e321)) + (right_anti_dual.group1.wwwx * float4(this.group0.xyz, this.e235)) + float4((right_anti_dual.group1.yzx * this.group1.zxy), right_anti_dual.e2 * this.e315), 
            /* e1234 */
            -(right_anti_dual.e1 * this.e423) - (right_anti_dual.e2 * this.e431) - (right_anti_dual.e3 * this.e412)
        );
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(wedge.e1234) * other.group1.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            float4(((other.group1.zxy * wedge.group0.yzx) - (other.group1.yzx * wedge.group0.zxy)), other.e3215 * wedge.e1234 * -1.0), 
            /* e235, e315, e125, e4 */
            float4(((float3(wedge.e3215) * other.group1.xyz) - (float3(other.e3215) * wedge.group0.xyz)), other.e45 * wedge.e1234 * -1.0), 
            /* e1, e2, e3, e5 */
            float4((float3(wedge.e1234) * other.group0.xyz * float3(-1.0)), (other.e15 * wedge.e4235) + (other.e25 * wedge.e4315) + (other.e35 * wedge.e4125) + (other.e45 * wedge.e3215))
        );
    }
}
public extension Circle {
    public Line project_orthogonally_onto(Line other) {
        AntiLine right_anti_dual = AntiLine(
            /* e23, e31, e12 */
            other.group0.xyz, 
            /* e15, e25, e35 */
            other.group1.xyz
        );
        AntiScalar wedge = AntiScalar(
            /* e12345 */
            -(right_anti_dual.e23 * this.e415) - (right_anti_dual.e31 * this.e425) - (right_anti_dual.e12 * this.e435) - (right_anti_dual.e15 * this.e423) - (right_anti_dual.e25 * this.e431) - (right_anti_dual.e35 * this.e412)
        );
        return Line(
            /* e415, e425, e435 */
            float3(wedge.e12345) * other.group0.xyz, 
            /* e235, e315, e125 */
            float3(wedge.e12345) * other.group1.xyz
        );
    }
}
public extension Circle {
    public VersorEven project_orthogonally_onto(Motor other) {
        AntiMotor right_anti_dual = AntiMotor(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        CircleRotor wedge = CircleRotor(
            /* e423, e431, e412 */
            float3(right_anti_dual.scalar) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(right_anti_dual.scalar) * this.group1, 
            /* e235, e315, e125, e12345 */
            float4((float3(right_anti_dual.scalar) * this.group2.xyz), -(right_anti_dual.e23 * this.e415) - (right_anti_dual.e31 * this.e425) - (right_anti_dual.e12 * this.e435) - (right_anti_dual.e15 * this.e423) - (right_anti_dual.e25 * this.e431) - (right_anti_dual.e35 * this.e412))
        );
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(other.e12345) * float4(wedge.group0.xyz, wedge.e12345), 
            /* e415, e425, e435, e321 */
            float4(((float3(wedge.e12345) * other.group0.xyz) + (float3(other.e12345) * wedge.group1.xyz)), wedge.e321 * other.e12345), 
            /* e235, e315, e125, e5 */
            float4(wedge.e12345 * other.e235, wedge.e12345 * other.e315, wedge.e12345 * other.e125, -(wedge.e415 * other.e235) - (wedge.e425 * other.e315) - (wedge.e435 * other.e125) - (wedge.e235 * other.e415) - (wedge.e315 * other.e425) - (wedge.e125 * other.e435)) + (wedge.group2 * float4(other.group0.www, other.e5)), 
            /* e1, e2, e3, e4 */
            float4((wedge.e412 * other.e315) + (wedge.e321 * other.e415), (wedge.e423 * other.e125) + (wedge.e321 * other.e425), (wedge.e431 * other.e235) + (wedge.e321 * other.e435), -(wedge.e431 * other.e425) - (wedge.e412 * other.e435)) - float4((wedge.group0.xyz.yzx * other.group1.zxy), wedge.e423 * other.e415)
        );
    }
}
public extension Circle {
    public MultiVector project_orthogonally_onto(MultiVector other) {
        MultiVector right_anti_dual = MultiVector(
            /* scalar, e12345 */
            other.group0.xy.yx * float2(-1.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4(other.group9.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5 */
            other.e3215, 
            /* e15, e25, e35, e45 */
            float4(other.group8.xyz, other.e321 * -1.0), 
            /* e41, e42, e43 */
            other.group7.xyz, 
            /* e23, e31, e12 */
            other.group6.xyz, 
            /* e415, e425, e435, e321 */
            float4(other.group5.xyz, other.e45) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e423, e431, e412 */
            other.group4.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            other.group3.xyz * float3(-1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.group1.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234 */
            other.e4 * -1.0
        );
        MultiVector wedge = MultiVector(
            /* scalar, e12345 */
            float2(0.0, -(this.e423 * right_anti_dual.e15) - (this.e431 * right_anti_dual.e25) - (this.e412 * right_anti_dual.e35) - (this.e415 * right_anti_dual.e23) - (this.e425 * right_anti_dual.e31) - (this.e435 * right_anti_dual.e12) - (this.e321 * right_anti_dual.e45) - (this.e235 * right_anti_dual.e41) - (this.e315 * right_anti_dual.e42) - (this.e125 * right_anti_dual.e43)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(right_anti_dual.scalar) * this.group1, 
            /* e423, e431, e412 */
            float3(right_anti_dual.scalar) * this.group0.xyz, 
            /* e235, e315, e125 */
            float3(right_anti_dual.scalar) * this.group2.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(-(this.e425 * right_anti_dual.e3) - (this.e235 * right_anti_dual.e4), -(this.e435 * right_anti_dual.e1) - (this.e315 * right_anti_dual.e4), -(this.e415 * right_anti_dual.e2) - (this.e125 * right_anti_dual.e4), (this.e321 * right_anti_dual.e5) + (this.e125 * right_anti_dual.e3)) + (right_anti_dual.group1.yzxy * float4(this.group1.zxy, this.e315)) + float4((float3(right_anti_dual.e5) * this.group0.xyz), this.e235 * right_anti_dual.e1), 
            /* e1234 */
            -(this.e423 * right_anti_dual.e1) - (this.e431 * right_anti_dual.e2) - (this.e412 * right_anti_dual.e3) - (this.e321 * right_anti_dual.e4)
        );
        return MultiVector(
            /* scalar, e12345 */
            float2((other.scalar * wedge.e12345) + (other.e12345 * wedge.scalar) + (other.e1 * wedge.e4235) + (other.e2 * wedge.e4315) + (other.e3 * wedge.e4125) + (other.e4 * wedge.e3215) + (other.e5 * wedge.e1234) + (other.e4235 * wedge.e1) + (other.e4315 * wedge.e2) + (other.e4125 * wedge.e3) + (other.e3215 * wedge.e4) + (other.e1234 * wedge.e5) - (other.e15 * wedge.e423) - (other.e25 * wedge.e431) - (other.e35 * wedge.e412) - (other.e45 * wedge.e321) - (other.e41 * wedge.e235) - (other.e42 * wedge.e315) - (other.e43 * wedge.e125) - (other.e23 * wedge.e415) - (other.e31 * wedge.e425) - (other.e12 * wedge.e435) - (other.e415 * wedge.e23) - (other.e425 * wedge.e31) - (other.e435 * wedge.e12) - (other.e321 * wedge.e45) - (other.e423 * wedge.e15) - (other.e431 * wedge.e25) - (other.e412 * wedge.e35) - (other.e235 * wedge.e41) - (other.e315 * wedge.e42) - (other.e125 * wedge.e43), other.e12345 * wedge.e12345), 
            /* e1, e2, e3, e4 */
            float4((other.e415 * wedge.e321) + (other.e321 * wedge.e415) + (other.e4315 * wedge.e12) + (other.e1234 * wedge.e15), (other.e425 * wedge.e321) + (other.e321 * wedge.e425) + (other.e4125 * wedge.e23) + (other.e1234 * wedge.e25), (other.e435 * wedge.e321) + (other.e321 * wedge.e435) + (other.e4235 * wedge.e31) + (other.e1234 * wedge.e35), -(other.e45 * wedge.e1234) - (other.e415 * wedge.e423) - (other.e425 * wedge.e431) - (other.e435 * wedge.e412)) + (float4(other.e12345) * wedge.group1) + (float4(wedge.e12345) * other.group1) + float4((other.group4.xyz * wedge.group9.www), other.e4235 * wedge.e41) + float4((other.group5.xyz.yzx * wedge.group9.zxy), other.e4315 * wedge.e42) + float4((other.group7.xyz.zxy * wedge.group8.xyz.yzx), other.e4125 * wedge.e43) + float4((other.group8.xyz.yzx * wedge.group7.xyz.zxy), other.e1234 * wedge.e45) - (wedge.group9.yzxx * float4(other.group5.xyz.zxy, other.e41)) - float4((float3(wedge.e1234) * other.group3.xyz), other.e412 * wedge.e435) - float4((wedge.group4.xyz * other.group9.www), other.e423 * wedge.e415) - float4((other.group7.xyz.yzx * wedge.group8.xyz.zxy), other.e42 * wedge.e4315) - float4((other.group8.xyz.zxy * wedge.group7.xyz.yzx), other.e43 * wedge.e4125) - float4((wedge.group5.xyz.yzx * other.group9.zxy), other.e431 * wedge.e425), 
            /* e5 */
            (other.e12345 * wedge.e5) + (other.e5 * wedge.e12345) + (other.e15 * wedge.e4235) + (other.e25 * wedge.e4315) + (other.e35 * wedge.e4125) + (other.e45 * wedge.e3215) - (other.e415 * wedge.e235) - (other.e425 * wedge.e315) - (other.e435 * wedge.e125) - (other.e235 * wedge.e415) - (other.e315 * wedge.e425) - (other.e125 * wedge.e435) - (other.e4235 * wedge.e15) - (other.e4315 * wedge.e25) - (other.e4125 * wedge.e35) - (other.e3215 * wedge.e45), 
            /* e15, e25, e35, e45 */
            float4((other.e415 * wedge.e3215) + (other.e315 * wedge.e4125) + (other.e4125 * wedge.e315) + (other.e3215 * wedge.e415), (other.e425 * wedge.e3215) + (other.e125 * wedge.e4235) + (other.e4235 * wedge.e125) + (other.e3215 * wedge.e425), (other.e435 * wedge.e3215) + (other.e235 * wedge.e4315) + (other.e4315 * wedge.e235) + (other.e3215 * wedge.e435), -(other.e435 * wedge.e4125) - (other.e4235 * wedge.e415) - (other.e4315 * wedge.e425) - (other.e4125 * wedge.e435)) + (float4(other.e12345) * wedge.group3) + (float4(wedge.e12345) * other.group3) - (wedge.group9.yzxx * float4(other.group8.xyz.zxy, other.e415)) - float4((wedge.group8.xyz.zxy * other.group9.yzx), other.e425 * wedge.e4315), 
            /* e41, e42, e43 */
            (float3(other.e12345) * wedge.group4.xyz) + (float3(other.e1234) * wedge.group6.xyz) + (float3(wedge.e12345) * other.group4.xyz) + (float3(wedge.e1234) * other.group6.xyz) + (other.group7.xyz.zxy * wedge.group9.yzx) + (wedge.group7.xyz.zxy * other.group9.yzx) - (other.group7.xyz.yzx * wedge.group9.zxy) - (wedge.group7.xyz.yzx * other.group9.zxy), 
            /* e23, e31, e12 */
            (float3(other.e12345) * wedge.group5.xyz) + (float3(other.e3215) * wedge.group7.xyz) + (float3(other.e1234) * wedge.group8.xyz) + (float3(wedge.e12345) * other.group5.xyz) + (float3(wedge.e3215) * other.group7.xyz) + (float3(wedge.e1234) * other.group8.xyz) - (float3(other.e321) * wedge.group9.xyz) - (float3(wedge.e321) * other.group9.xyz), 
            /* e415, e425, e435, e321 */
            (float4(other.e12345) * wedge.group6) + (float4(wedge.e12345) * other.group6) + float4((other.group9.zxy * wedge.group9.yzx), other.e1234 * wedge.e3215) - (other.group9.yzxw * float4(wedge.group9.zxy, wedge.e1234)), 
            /* e423, e431, e412 */
            (float3(other.e12345) * wedge.group7.xyz) + (float3(other.e1234) * wedge.group9.xyz) + (float3(wedge.e12345) * other.group7.xyz) - (float3(wedge.e1234) * other.group9.xyz), 
            /* e235, e315, e125 */
            (float3(other.e12345) * wedge.group8.xyz) + (float3(wedge.e12345) * other.group8.xyz) + (float3(wedge.e3215) * other.group9.xyz) - (float3(other.e3215) * wedge.group9.xyz), 
            /* e4235, e4315, e4125, e3215 */
            (float4(other.e12345) * wedge.group9) + (float4(wedge.e12345) * other.group9), 
            /* e1234 */
            (other.e12345 * wedge.e1234) + (other.e1234 * wedge.e12345)
        );
    }
}
public extension Circle {
    public Circle project_orthogonally_onto(Plane other) {
        AntiPlane right_anti_dual = AntiPlane(
            /* e1, e2, e3, e5 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        Sphere wedge = Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual.e3 * this.e425 * -1.0, right_anti_dual.e1 * this.e435 * -1.0, right_anti_dual.e2 * this.e415 * -1.0, (right_anti_dual.e3 * this.e125) + (right_anti_dual.e5 * this.e321)) + (right_anti_dual.group0.wwwx * float4(this.group0.xyz, this.e235)) + float4((right_anti_dual.group0.yzx * this.group1.zxy), right_anti_dual.e2 * this.e315), 
            /* e1234 */
            -(right_anti_dual.e1 * this.e423) - (right_anti_dual.e2 * this.e431) - (right_anti_dual.e3 * this.e412)
        );
        return Circle(
            /* e423, e431, e412 */
            float3(wedge.e1234) * other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            float4(((other.group0.zxy * wedge.group0.yzx) - (other.group0.yzx * wedge.group0.zxy)), other.e3215 * wedge.e1234 * -1.0), 
            /* e235, e315, e125 */
            (float3(wedge.e3215) * other.group0.xyz) - (float3(other.e3215) * wedge.group0.xyz)
        );
    }
}
public extension Circle {
    public Circle project_orthogonally_onto(Sphere other) {
        RoundPoint right_anti_dual = RoundPoint(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5 */
            other.e3215
        );
        Sphere wedge = Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(-(this.e425 * right_anti_dual.e3) - (this.e235 * right_anti_dual.e4), -(this.e435 * right_anti_dual.e1) - (this.e315 * right_anti_dual.e4), -(this.e415 * right_anti_dual.e2) - (this.e125 * right_anti_dual.e4), (this.e321 * right_anti_dual.e5) + (this.e125 * right_anti_dual.e3)) + (right_anti_dual.group0.yzxy * float4(this.group1.zxy, this.e315)) + float4((float3(right_anti_dual.e5) * this.group0.xyz), this.e235 * right_anti_dual.e1), 
            /* e1234 */
            -(this.e423 * right_anti_dual.e1) - (this.e431 * right_anti_dual.e2) - (this.e412 * right_anti_dual.e3) - (this.e321 * right_anti_dual.e4)
        );
        return Circle(
            /* e423, e431, e412 */
            (float3(other.e1234) * wedge.group0.xyz) - (float3(wedge.e1234) * other.group0.xyz), 
            /* e415, e425, e435, e321 */
            float4((other.group0.zxy * wedge.group0.yzx), other.e1234 * wedge.e3215) - (other.group0.yzxw * float4(wedge.group0.zxy, wedge.e1234)), 
            /* e235, e315, e125 */
            (float3(wedge.e3215) * other.group0.xyz) - (float3(other.e3215) * wedge.group0.xyz)
        );
    }
}
public extension Circle {
    public VersorEven project_orthogonally_onto(VersorEven other) {
        VersorOdd right_anti_dual = VersorOdd(
            /* e41, e42, e43, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e1234 */
            float4(other.group2.xyz, other.e4) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.group3.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0)
        );
        CircleRotor wedge = CircleRotor(
            /* e423, e431, e412 */
            float3(right_anti_dual.scalar) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(right_anti_dual.scalar) * this.group1, 
            /* e235, e315, e125, e12345 */
            float4((float3(right_anti_dual.scalar) * this.group2.xyz), -(this.e423 * right_anti_dual.e15) - (this.e431 * right_anti_dual.e25) - (this.e412 * right_anti_dual.e35) - (this.e415 * right_anti_dual.e23) - (this.e425 * right_anti_dual.e31) - (this.e435 * right_anti_dual.e12) - (this.e321 * right_anti_dual.e45) - (this.e235 * right_anti_dual.e41) - (this.e315 * right_anti_dual.e42) - (this.e125 * right_anti_dual.e43))
        );
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(((float3(wedge.e12345) * other.group0.xyz) + (float3(other.e12345) * wedge.group0.xyz)), wedge.e12345 * other.e12345), 
            /* e415, e425, e435, e321 */
            (float4(wedge.e12345) * other.group1) + (float4(other.e12345) * wedge.group1), 
            /* e235, e315, e125, e5 */
            float4(wedge.e12345 * other.e235, wedge.e12345 * other.e315, wedge.e12345 * other.e125, -(wedge.e415 * other.e235) - (wedge.e425 * other.e315) - (wedge.e435 * other.e125) - (wedge.e235 * other.e415) - (wedge.e315 * other.e425) - (wedge.e125 * other.e435)) + (wedge.group2 * float4(other.group0.www, other.e5)), 
            /* e1, e2, e3, e4 */
            float4((wedge.e415 * other.e321) + (wedge.e321 * other.e415) + (wedge.e315 * other.e412) + (wedge.e12345 * other.e1), (wedge.e425 * other.e321) + (wedge.e321 * other.e425) + (wedge.e125 * other.e423) + (wedge.e12345 * other.e2), (wedge.e435 * other.e321) + (wedge.e321 * other.e435) + (wedge.e235 * other.e431) + (wedge.e12345 * other.e3), -(wedge.e412 * other.e435) - (wedge.e415 * other.e423) - (wedge.e425 * other.e431) - (wedge.e435 * other.e412)) + float4((wedge.group0.xyz.zxy * other.group2.yzx), wedge.e12345 * other.e4) - float4((wedge.group0.xyz.yzx * other.group2.zxy), wedge.e423 * other.e415) - float4((wedge.group2.zxy * other.group0.yzx), wedge.e431 * other.e425)
        );
    }
}
public extension Circle {
    public AntiDipoleInversion project_orthogonally_onto(VersorOdd other) {
        VersorEven right_anti_dual = VersorEven(
            /* e423, e431, e412, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            float4(other.group2.xyz, other.e3215) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4(other.group3.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0)
        );
        Sphere wedge = Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(-(this.e425 * right_anti_dual.e3) - (this.e235 * right_anti_dual.e4), -(this.e435 * right_anti_dual.e1) - (this.e315 * right_anti_dual.e4), -(this.e415 * right_anti_dual.e2) - (this.e125 * right_anti_dual.e4), (this.e321 * right_anti_dual.e5) + (this.e125 * right_anti_dual.e3)) + (right_anti_dual.group3.yzxy * float4(this.group1.zxy, this.e315)) + float4((this.group0.xyz * right_anti_dual.group2.www), this.e235 * right_anti_dual.e1), 
            /* e1234 */
            -(this.e423 * right_anti_dual.e1) - (this.e431 * right_anti_dual.e2) - (this.e412 * right_anti_dual.e3) - (this.e321 * right_anti_dual.e4)
        );
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (float3(other.e1234) * wedge.group0.xyz) - (float3(wedge.e1234) * other.group3.xyz), 
            /* e415, e425, e435, e321 */
            (wedge.group0.yzxw * float4(other.group3.zxy, other.e1234)) - float4((wedge.group0.zxy * other.group3.yzx), wedge.e1234 * other.e3215), 
            /* e235, e315, e125, e4 */
            float4(wedge.e3215 * other.e4235, wedge.e3215 * other.e4315, wedge.e3215 * other.e4125, -(wedge.e4315 * other.e42) - (wedge.e4125 * other.e43) - (wedge.e1234 * other.e45)) - (wedge.group0.xyzx * float4(other.group3.www, other.e41)), 
            /* e1, e2, e3, e5 */
            float4(-(wedge.e4315 * other.e12) - (wedge.e1234 * other.e15), -(wedge.e4125 * other.e23) - (wedge.e1234 * other.e25), -(wedge.e4235 * other.e31) - (wedge.e1234 * other.e35), (wedge.e4125 * other.e35) + (wedge.e3215 * other.e45)) + (wedge.group0.zxyx * float4(other.group1.yzx, other.e15)) + (wedge.group0.wwwy * float4(other.group0.xyz, other.e25))
        );
    }
}
public extension CircleRotor {
    // Fancy infix trick (first half)
    public ProjectOrthogonallyOntoInfixPartial<CircleRotor> operator /(ProjectOrthogonallyOntoInfix rhs) {
        return ProjectOrthogonallyOntoInfixPartial<CircleRotor>(this);
    }
}
public extension ProjectOrthogonallyOntoInfixPartial<CircleRotor> {
    // Fancy infix trick (second half)
    public AntiDipoleInversion operator /(AntiDipoleInversion rhs) {
        return this.a.project_orthogonally_onto(rhs);
    }
}
public extension CircleRotor {
    public AntiDipoleInversion project_orthogonally_onto(AntiDipoleInversion other) {
        DipoleInversion right_anti_dual = DipoleInversion(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e1234 */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group3 * float4(1.0, 1.0, 1.0, -1.0)
        );
        AntiScalar wedge = AntiScalar(
            /* e12345 */
            -(this.e423 * right_anti_dual.e15) - (this.e431 * right_anti_dual.e25) - (this.e412 * right_anti_dual.e35) - (this.e415 * right_anti_dual.e23) - (this.e425 * right_anti_dual.e31) - (this.e435 * right_anti_dual.e12) - (this.e321 * right_anti_dual.e45) - (this.e235 * right_anti_dual.e41) - (this.e315 * right_anti_dual.e42) - (this.e125 * right_anti_dual.e43)
        );
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(wedge.e12345) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(wedge.e12345) * other.group1, 
            /* e235, e315, e125, e4 */
            float4(wedge.e12345) * other.group2, 
            /* e1, e2, e3, e5 */
            float4(wedge.e12345) * other.group3
        );
    }
}
public extension CircleRotor {
    public AntiFlatPoint project_orthogonally_onto(AntiDualNum other) {
        DualNum right_anti_dual = DualNum(
            /* e5, e12345 */
            other.group0.xy
        );
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(other.group0.xy.xx, other.e3215, 0.0) * float4(float3(1.0), 0.0) * float4(float4(this.e423 * right_anti_dual.e5, this.e431 * right_anti_dual.e5, this.e412 * right_anti_dual.e5, this.e321 * right_anti_dual.e5).xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0)
        );
    }
}
public extension CircleRotor {
    public AntiFlatPoint project_orthogonally_onto(AntiFlatPoint other) {
        FlatPoint right_anti_dual = FlatPoint(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(-(this.e423 * right_anti_dual.e15) - (this.e431 * right_anti_dual.e25) - (this.e412 * right_anti_dual.e35) - (this.e321 * right_anti_dual.e45)) * other.group0
        );
    }
}
public extension CircleRotor {
    public AntiFlector project_orthogonally_onto(AntiFlector other) {
        Flector right_anti_dual = Flector(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        AntiScalar wedge = AntiScalar(
            /* e12345 */
            -(this.e423 * right_anti_dual.e15) - (this.e431 * right_anti_dual.e25) - (this.e412 * right_anti_dual.e35) - (this.e321 * right_anti_dual.e45)
        );
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(wedge.e12345) * other.group0, 
            /* e1, e2, e3, e5 */
            float4(wedge.e12345) * other.group1
        );
    }
}
public extension CircleRotor {
    public AntiFlector project_orthogonally_onto(AntiMotor other) {
        Plane wedge = Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(other.e3215) * float4(this.group0.xyz, this.e321)
        );
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(float3(1.0), 0.0) * float4(other.group1.www, 0.0) * float4(wedge.group0.xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0), 
            /* e1, e2, e3, e5 */
            float4(other.e12 * wedge.e4315 * -1.0, other.e23 * wedge.e4125 * -1.0, other.e31 * wedge.e4235 * -1.0, (other.e25 * wedge.e4315) + (other.e35 * wedge.e4125)) + (wedge.group0.zxyx * float4(other.group0.yzx, other.e15))
        );
    }
}
public extension CircleRotor {
    public CircleRotor project_orthogonally_onto(AntiScalar other) {
        Scalar right_anti_dual = Scalar(
            /* scalar */
            other.e12345 * -1.0
        );
        CircleRotor wedge = CircleRotor(
            /* e423, e431, e412 */
            float3(right_anti_dual.scalar) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(right_anti_dual.scalar) * this.group1, 
            /* e235, e315, e125, e12345 */
            float4(right_anti_dual.scalar) * this.group2
        );
        return CircleRotor(
            /* e423, e431, e412 */
            float3(other.e12345) * wedge.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(other.e12345) * wedge.group1, 
            /* e235, e315, e125, e12345 */
            float4(other.e12345) * wedge.group2
        );
    }
}
public extension CircleRotor {
    public Circle project_orthogonally_onto(Circle other) {
        Dipole right_anti_dual = Dipole(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35 */
            other.group2.xyz
        );
        AntiScalar wedge = AntiScalar(
            /* e12345 */
            -(this.e423 * right_anti_dual.e15) - (this.e431 * right_anti_dual.e25) - (this.e412 * right_anti_dual.e35) - (this.e415 * right_anti_dual.e23) - (this.e425 * right_anti_dual.e31) - (this.e435 * right_anti_dual.e12) - (this.e321 * right_anti_dual.e45) - (this.e235 * right_anti_dual.e41) - (this.e315 * right_anti_dual.e42) - (this.e125 * right_anti_dual.e43)
        );
        return Circle(
            /* e423, e431, e412 */
            float3(wedge.e12345) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(wedge.e12345) * other.group1, 
            /* e235, e315, e125 */
            float3(wedge.e12345) * other.group2.xyz
        );
    }
}
public extension CircleRotor {
    public VersorEven project_orthogonally_onto(CircleRotor other) {
        AntiCircleRotor right_anti_dual = AntiCircleRotor(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        CircleRotor wedge = CircleRotor(
            /* e423, e431, e412 */
            float3(right_anti_dual.scalar) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(right_anti_dual.scalar) * this.group1, 
            /* e235, e315, e125, e12345 */
            float4((right_anti_dual.group2.www * this.group2.xyz), (right_anti_dual.scalar * this.e12345) - (right_anti_dual.e41 * this.e235) - (right_anti_dual.e42 * this.e315) - (right_anti_dual.e43 * this.e125) - (right_anti_dual.e23 * this.e415) - (right_anti_dual.e31 * this.e425) - (right_anti_dual.e12 * this.e435) - (right_anti_dual.e45 * this.e321) - (right_anti_dual.e15 * this.e423) - (right_anti_dual.e25 * this.e431) - (right_anti_dual.e35 * this.e412))
        );
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(((float3(other.e12345) * wedge.group0.xyz) + (float3(wedge.e12345) * other.group0.xyz)), other.e12345 * wedge.e12345), 
            /* e415, e425, e435, e321 */
            (float4(other.e12345) * wedge.group1) + (float4(wedge.e12345) * other.group1), 
            /* e235, e315, e125, e5 */
            float4((other.e235 * wedge.e12345) + (other.e12345 * wedge.e235), (other.e315 * wedge.e12345) + (other.e12345 * wedge.e315), (other.e125 * wedge.e12345) + (other.e12345 * wedge.e125), -(other.e415 * wedge.e235) - (other.e425 * wedge.e315) - (other.e435 * wedge.e125) - (other.e235 * wedge.e415) - (other.e315 * wedge.e425) - (other.e125 * wedge.e435)), 
            /* e1, e2, e3, e4 */
            float4((other.e412 * wedge.e315) + (other.e415 * wedge.e321) + (other.e321 * wedge.e415) + (other.e315 * wedge.e412), (other.e423 * wedge.e125) + (other.e425 * wedge.e321) + (other.e321 * wedge.e425) + (other.e125 * wedge.e423), (other.e431 * wedge.e235) + (other.e435 * wedge.e321) + (other.e321 * wedge.e435) + (other.e235 * wedge.e431), -(other.e412 * wedge.e435) - (other.e415 * wedge.e423) - (other.e425 * wedge.e431) - (other.e435 * wedge.e412)) - float4((other.group0.xyz.yzx * wedge.group2.zxy), other.e423 * wedge.e415) - float4((wedge.group0.xyz.yzx * other.group2.zxy), other.e431 * wedge.e425)
        );
    }
}
public extension CircleRotor {
    public AntiDipoleInversion project_orthogonally_onto(DipoleInversion other) {
        AntiDipoleInversion right_anti_dual = AntiDipoleInversion(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e4 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group3 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        Sphere wedge = Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(-(right_anti_dual.e4 * this.e235) - (right_anti_dual.e3 * this.e425), -(right_anti_dual.e4 * this.e315) - (right_anti_dual.e1 * this.e435), -(right_anti_dual.e4 * this.e125) - (right_anti_dual.e2 * this.e415), (right_anti_dual.e3 * this.e125) + (right_anti_dual.e5 * this.e321)) + (right_anti_dual.group3.yzxy * float4(this.group1.zxy, this.e315)) + float4((this.group0.xyz * right_anti_dual.group3.www), right_anti_dual.e1 * this.e235), 
            /* e1234 */
            -(right_anti_dual.e4 * this.e321) - (right_anti_dual.e1 * this.e423) - (right_anti_dual.e2 * this.e431) - (right_anti_dual.e3 * this.e412)
        );
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (float3(other.e1234) * wedge.group0.xyz) - (float3(wedge.e1234) * other.group3.xyz), 
            /* e415, e425, e435, e321 */
            (wedge.group0.yzxw * float4(other.group3.zxy, other.e1234)) - (other.group3.yzxw * float4(wedge.group0.zxy, wedge.e1234)), 
            /* e235, e315, e125, e4 */
            float4(other.e4235 * wedge.e3215, other.e4315 * wedge.e3215, other.e4125 * wedge.e3215, -(other.e42 * wedge.e4315) - (other.e43 * wedge.e4125) - (other.e45 * wedge.e1234)) - (wedge.group0.xyzx * float4(other.group3.www, other.e41)), 
            /* e1, e2, e3, e5 */
            float4(-(other.e12 * wedge.e4315) - (other.e15 * wedge.e1234), -(other.e23 * wedge.e4125) - (other.e25 * wedge.e1234), -(other.e31 * wedge.e4235) - (other.e35 * wedge.e1234), (other.e25 * wedge.e4315) + (other.e35 * wedge.e4125)) + (float4(wedge.e3215) * float4(other.group0.xyz, other.e45)) + (wedge.group0.zxyx * float4(other.group1.yzx, other.e15))
        );
    }
}
public extension CircleRotor {
    public VersorEven project_orthogonally_onto(DualNum other) {
        AntiDualNum right_anti_dual = AntiDualNum(
            /* e3215, scalar */
            other.group0.xy * float2(-1.0)
        );
        CircleRotor wedge = CircleRotor(
            /* e423, e431, e412 */
            float3(right_anti_dual.scalar) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(right_anti_dual.scalar) * this.group1, 
            /* e235, e315, e125, e12345 */
            float4(right_anti_dual.scalar) * this.group2
        );
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(other.e12345) * float4(wedge.group0.xyz, wedge.e12345), 
            /* e415, e425, e435, e321 */
            float4(other.e12345) * wedge.group1, 
            /* e235, e315, e125, e5 */
            wedge.group2 * float4(other.group0.xy.yy, other.e12345, other.e5), 
            /* e1, e2, e3, e4 */
            float4(0.0)
        );
    }
}
public extension CircleRotor {
    public AntiDipoleInversion project_orthogonally_onto(Flector other) {
        AntiFlector right_anti_dual = AntiFlector(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        Sphere wedge = Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual.e3 * this.e425 * -1.0, right_anti_dual.e1 * this.e435 * -1.0, right_anti_dual.e2 * this.e415 * -1.0, (right_anti_dual.e3 * this.e125) + (right_anti_dual.e5 * this.e321)) + (right_anti_dual.group1.yzxy * float4(this.group1.zxy, this.e315)) + float4((this.group0.xyz * right_anti_dual.group1.www), right_anti_dual.e1 * this.e235), 
            /* e1234 */
            -(right_anti_dual.e1 * this.e423) - (right_anti_dual.e2 * this.e431) - (right_anti_dual.e3 * this.e412)
        );
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(wedge.e1234) * other.group1.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            float4(((other.group1.zxy * wedge.group0.yzx) - (other.group1.yzx * wedge.group0.zxy)), other.e3215 * wedge.e1234 * -1.0), 
            /* e235, e315, e125, e4 */
            float4(((float3(wedge.e3215) * other.group1.xyz) - (float3(other.e3215) * wedge.group0.xyz)), other.e45 * wedge.e1234 * -1.0), 
            /* e1, e2, e3, e5 */
            float4((float3(wedge.e1234) * other.group0.xyz * float3(-1.0)), (other.e15 * wedge.e4235) + (other.e25 * wedge.e4315) + (other.e35 * wedge.e4125) + (other.e45 * wedge.e3215))
        );
    }
}
public extension CircleRotor {
    public Line project_orthogonally_onto(Line other) {
        AntiLine right_anti_dual = AntiLine(
            /* e23, e31, e12 */
            other.group0.xyz, 
            /* e15, e25, e35 */
            other.group1.xyz
        );
        AntiScalar wedge = AntiScalar(
            /* e12345 */
            -(right_anti_dual.e23 * this.e415) - (right_anti_dual.e31 * this.e425) - (right_anti_dual.e12 * this.e435) - (right_anti_dual.e15 * this.e423) - (right_anti_dual.e25 * this.e431) - (right_anti_dual.e35 * this.e412)
        );
        return Line(
            /* e415, e425, e435 */
            float3(wedge.e12345) * other.group0.xyz, 
            /* e235, e315, e125 */
            float3(wedge.e12345) * other.group1.xyz
        );
    }
}
public extension CircleRotor {
    public VersorEven project_orthogonally_onto(Motor other) {
        AntiMotor right_anti_dual = AntiMotor(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        CircleRotor wedge = CircleRotor(
            /* e423, e431, e412 */
            float3(right_anti_dual.scalar) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(right_anti_dual.scalar) * this.group1, 
            /* e235, e315, e125, e12345 */
            float4((right_anti_dual.group0.www * this.group2.xyz), (right_anti_dual.scalar * this.e12345) - (right_anti_dual.e23 * this.e415) - (right_anti_dual.e31 * this.e425) - (right_anti_dual.e12 * this.e435) - (right_anti_dual.e15 * this.e423) - (right_anti_dual.e25 * this.e431) - (right_anti_dual.e35 * this.e412))
        );
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(other.e12345) * float4(wedge.group0.xyz, wedge.e12345), 
            /* e415, e425, e435, e321 */
            float4(((float3(wedge.e12345) * other.group0.xyz) + (float3(other.e12345) * wedge.group1.xyz)), wedge.e321 * other.e12345), 
            /* e235, e315, e125, e5 */
            float4(wedge.e12345 * other.e235, wedge.e12345 * other.e315, wedge.e12345 * other.e125, -(wedge.e415 * other.e235) - (wedge.e425 * other.e315) - (wedge.e435 * other.e125) - (wedge.e235 * other.e415) - (wedge.e315 * other.e425) - (wedge.e125 * other.e435)) + (wedge.group2 * float4(other.group0.www, other.e5)), 
            /* e1, e2, e3, e4 */
            float4((wedge.e412 * other.e315) + (wedge.e321 * other.e415), (wedge.e423 * other.e125) + (wedge.e321 * other.e425), (wedge.e431 * other.e235) + (wedge.e321 * other.e435), -(wedge.e431 * other.e425) - (wedge.e412 * other.e435)) - float4((wedge.group0.xyz.yzx * other.group1.zxy), wedge.e423 * other.e415)
        );
    }
}
public extension CircleRotor {
    public MultiVector project_orthogonally_onto(MultiVector other) {
        MultiVector right_anti_dual = MultiVector(
            /* scalar, e12345 */
            other.group0.xy.yx * float2(-1.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4(other.group9.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5 */
            other.e3215, 
            /* e15, e25, e35, e45 */
            float4(other.group8.xyz, other.e321 * -1.0), 
            /* e41, e42, e43 */
            other.group7.xyz, 
            /* e23, e31, e12 */
            other.group6.xyz, 
            /* e415, e425, e435, e321 */
            float4(other.group5.xyz, other.e45) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e423, e431, e412 */
            other.group4.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            other.group3.xyz * float3(-1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.group1.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234 */
            other.e4 * -1.0
        );
        MultiVector wedge = MultiVector(
            /* scalar, e12345 */
            float2(0.0, (this.e12345 * right_anti_dual.scalar) - (this.e423 * right_anti_dual.e15) - (this.e431 * right_anti_dual.e25) - (this.e412 * right_anti_dual.e35) - (this.e415 * right_anti_dual.e23) - (this.e425 * right_anti_dual.e31) - (this.e435 * right_anti_dual.e12) - (this.e321 * right_anti_dual.e45) - (this.e235 * right_anti_dual.e41) - (this.e315 * right_anti_dual.e42) - (this.e125 * right_anti_dual.e43)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(right_anti_dual.scalar) * this.group1, 
            /* e423, e431, e412 */
            float3(right_anti_dual.scalar) * this.group0.xyz, 
            /* e235, e315, e125 */
            float3(right_anti_dual.scalar) * this.group2.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(-(this.e425 * right_anti_dual.e3) - (this.e235 * right_anti_dual.e4), -(this.e435 * right_anti_dual.e1) - (this.e315 * right_anti_dual.e4), -(this.e415 * right_anti_dual.e2) - (this.e125 * right_anti_dual.e4), (this.e315 * right_anti_dual.e2) + (this.e125 * right_anti_dual.e3)) + (float4(right_anti_dual.e5) * float4(this.group0.xyz, this.e321)) + (right_anti_dual.group1.yzxx * float4(this.group1.zxy, this.e235)), 
            /* e1234 */
            -(this.e423 * right_anti_dual.e1) - (this.e431 * right_anti_dual.e2) - (this.e412 * right_anti_dual.e3) - (this.e321 * right_anti_dual.e4)
        );
        return MultiVector(
            /* scalar, e12345 */
            float2((other.scalar * wedge.e12345) + (other.e12345 * wedge.scalar) + (other.e1 * wedge.e4235) + (other.e2 * wedge.e4315) + (other.e3 * wedge.e4125) + (other.e4 * wedge.e3215) + (other.e5 * wedge.e1234) + (other.e4235 * wedge.e1) + (other.e4315 * wedge.e2) + (other.e4125 * wedge.e3) + (other.e3215 * wedge.e4) + (other.e1234 * wedge.e5) - (other.e15 * wedge.e423) - (other.e25 * wedge.e431) - (other.e35 * wedge.e412) - (other.e45 * wedge.e321) - (other.e41 * wedge.e235) - (other.e42 * wedge.e315) - (other.e43 * wedge.e125) - (other.e23 * wedge.e415) - (other.e31 * wedge.e425) - (other.e12 * wedge.e435) - (other.e415 * wedge.e23) - (other.e425 * wedge.e31) - (other.e435 * wedge.e12) - (other.e321 * wedge.e45) - (other.e423 * wedge.e15) - (other.e431 * wedge.e25) - (other.e412 * wedge.e35) - (other.e235 * wedge.e41) - (other.e315 * wedge.e42) - (other.e125 * wedge.e43), other.e12345 * wedge.e12345), 
            /* e1, e2, e3, e4 */
            float4((other.e415 * wedge.e321) + (other.e321 * wedge.e415) + (other.e4315 * wedge.e12) + (other.e1234 * wedge.e15), (other.e425 * wedge.e321) + (other.e321 * wedge.e425) + (other.e4125 * wedge.e23) + (other.e1234 * wedge.e25), (other.e435 * wedge.e321) + (other.e321 * wedge.e435) + (other.e4235 * wedge.e31) + (other.e1234 * wedge.e35), -(other.e45 * wedge.e1234) - (other.e415 * wedge.e423) - (other.e425 * wedge.e431) - (other.e435 * wedge.e412)) + (float4(other.e12345) * wedge.group1) + (float4(wedge.e12345) * other.group1) + float4((other.group4.xyz * wedge.group9.www), other.e4235 * wedge.e41) + float4((other.group5.xyz.yzx * wedge.group9.zxy), other.e4315 * wedge.e42) + float4((other.group7.xyz.zxy * wedge.group8.xyz.yzx), other.e4125 * wedge.e43) + float4((other.group8.xyz.yzx * wedge.group7.xyz.zxy), other.e1234 * wedge.e45) - (wedge.group9.yzxx * float4(other.group5.xyz.zxy, other.e41)) - float4((float3(wedge.e1234) * other.group3.xyz), other.e412 * wedge.e435) - float4((wedge.group4.xyz * other.group9.www), other.e423 * wedge.e415) - float4((other.group7.xyz.yzx * wedge.group8.xyz.zxy), other.e42 * wedge.e4315) - float4((other.group8.xyz.zxy * wedge.group7.xyz.yzx), other.e43 * wedge.e4125) - float4((wedge.group5.xyz.yzx * other.group9.zxy), other.e431 * wedge.e425), 
            /* e5 */
            (other.e12345 * wedge.e5) + (other.e5 * wedge.e12345) + (other.e15 * wedge.e4235) + (other.e25 * wedge.e4315) + (other.e35 * wedge.e4125) + (other.e45 * wedge.e3215) - (other.e415 * wedge.e235) - (other.e425 * wedge.e315) - (other.e435 * wedge.e125) - (other.e235 * wedge.e415) - (other.e315 * wedge.e425) - (other.e125 * wedge.e435) - (other.e4235 * wedge.e15) - (other.e4315 * wedge.e25) - (other.e4125 * wedge.e35) - (other.e3215 * wedge.e45), 
            /* e15, e25, e35, e45 */
            float4((other.e415 * wedge.e3215) + (other.e315 * wedge.e4125) + (other.e4125 * wedge.e315) + (other.e3215 * wedge.e415), (other.e425 * wedge.e3215) + (other.e125 * wedge.e4235) + (other.e4235 * wedge.e125) + (other.e3215 * wedge.e425), (other.e435 * wedge.e3215) + (other.e235 * wedge.e4315) + (other.e4315 * wedge.e235) + (other.e3215 * wedge.e435), -(other.e435 * wedge.e4125) - (other.e4235 * wedge.e415) - (other.e4315 * wedge.e425) - (other.e4125 * wedge.e435)) + (float4(other.e12345) * wedge.group3) + (float4(wedge.e12345) * other.group3) - (wedge.group9.yzxx * float4(other.group8.xyz.zxy, other.e415)) - float4((wedge.group8.xyz.zxy * other.group9.yzx), other.e425 * wedge.e4315), 
            /* e41, e42, e43 */
            (float3(other.e12345) * wedge.group4.xyz) + (float3(other.e1234) * wedge.group6.xyz) + (float3(wedge.e12345) * other.group4.xyz) + (float3(wedge.e1234) * other.group6.xyz) + (other.group7.xyz.zxy * wedge.group9.yzx) + (wedge.group7.xyz.zxy * other.group9.yzx) - (other.group7.xyz.yzx * wedge.group9.zxy) - (wedge.group7.xyz.yzx * other.group9.zxy), 
            /* e23, e31, e12 */
            (float3(other.e12345) * wedge.group5.xyz) + (float3(other.e3215) * wedge.group7.xyz) + (float3(other.e1234) * wedge.group8.xyz) + (float3(wedge.e12345) * other.group5.xyz) + (float3(wedge.e3215) * other.group7.xyz) + (float3(wedge.e1234) * other.group8.xyz) - (float3(other.e321) * wedge.group9.xyz) - (float3(wedge.e321) * other.group9.xyz), 
            /* e415, e425, e435, e321 */
            (float4(other.e12345) * wedge.group6) + (float4(wedge.e12345) * other.group6) + float4((other.group9.zxy * wedge.group9.yzx), other.e1234 * wedge.e3215) - (other.group9.yzxw * float4(wedge.group9.zxy, wedge.e1234)), 
            /* e423, e431, e412 */
            (float3(other.e12345) * wedge.group7.xyz) + (float3(other.e1234) * wedge.group9.xyz) + (float3(wedge.e12345) * other.group7.xyz) - (float3(wedge.e1234) * other.group9.xyz), 
            /* e235, e315, e125 */
            (float3(other.e12345) * wedge.group8.xyz) + (float3(wedge.e12345) * other.group8.xyz) + (float3(wedge.e3215) * other.group9.xyz) - (float3(other.e3215) * wedge.group9.xyz), 
            /* e4235, e4315, e4125, e3215 */
            (float4(other.e12345) * wedge.group9) + (float4(wedge.e12345) * other.group9), 
            /* e1234 */
            (other.e12345 * wedge.e1234) + (other.e1234 * wedge.e12345)
        );
    }
}
public extension CircleRotor {
    public Circle project_orthogonally_onto(Plane other) {
        AntiPlane right_anti_dual = AntiPlane(
            /* e1, e2, e3, e5 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        Sphere wedge = Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual.e3 * this.e425 * -1.0, right_anti_dual.e1 * this.e435 * -1.0, right_anti_dual.e2 * this.e415 * -1.0, (right_anti_dual.e3 * this.e125) + (right_anti_dual.e5 * this.e321)) + (right_anti_dual.group0.yzxy * float4(this.group1.zxy, this.e315)) + float4((this.group0.xyz * right_anti_dual.group0.www), right_anti_dual.e1 * this.e235), 
            /* e1234 */
            -(right_anti_dual.e1 * this.e423) - (right_anti_dual.e2 * this.e431) - (right_anti_dual.e3 * this.e412)
        );
        return Circle(
            /* e423, e431, e412 */
            float3(wedge.e1234) * other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            float4(((other.group0.zxy * wedge.group0.yzx) - (other.group0.yzx * wedge.group0.zxy)), other.e3215 * wedge.e1234 * -1.0), 
            /* e235, e315, e125 */
            (float3(wedge.e3215) * other.group0.xyz) - (float3(other.e3215) * wedge.group0.xyz)
        );
    }
}
public extension CircleRotor {
    public Circle project_orthogonally_onto(Sphere other) {
        RoundPoint right_anti_dual = RoundPoint(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5 */
            other.e3215
        );
        Sphere wedge = Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(-(this.e425 * right_anti_dual.e3) - (this.e235 * right_anti_dual.e4), -(this.e435 * right_anti_dual.e1) - (this.e315 * right_anti_dual.e4), -(this.e415 * right_anti_dual.e2) - (this.e125 * right_anti_dual.e4), (this.e315 * right_anti_dual.e2) + (this.e125 * right_anti_dual.e3)) + (float4(right_anti_dual.e5) * float4(this.group0.xyz, this.e321)) + (right_anti_dual.group0.yzxx * float4(this.group1.zxy, this.e235)), 
            /* e1234 */
            -(this.e423 * right_anti_dual.e1) - (this.e431 * right_anti_dual.e2) - (this.e412 * right_anti_dual.e3) - (this.e321 * right_anti_dual.e4)
        );
        return Circle(
            /* e423, e431, e412 */
            (float3(other.e1234) * wedge.group0.xyz) - (float3(wedge.e1234) * other.group0.xyz), 
            /* e415, e425, e435, e321 */
            float4((other.group0.zxy * wedge.group0.yzx), other.e1234 * wedge.e3215) - (other.group0.yzxw * float4(wedge.group0.zxy, wedge.e1234)), 
            /* e235, e315, e125 */
            (float3(wedge.e3215) * other.group0.xyz) - (float3(other.e3215) * wedge.group0.xyz)
        );
    }
}
public extension CircleRotor {
    public VersorEven project_orthogonally_onto(VersorEven other) {
        VersorOdd right_anti_dual = VersorOdd(
            /* e41, e42, e43, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e1234 */
            float4(other.group2.xyz, other.e4) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.group3.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0)
        );
        CircleRotor wedge = CircleRotor(
            /* e423, e431, e412 */
            float3(right_anti_dual.scalar) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(right_anti_dual.scalar) * this.group1, 
            /* e235, e315, e125, e12345 */
            float4((float3(right_anti_dual.scalar) * this.group2.xyz), (this.e12345 * right_anti_dual.scalar) - (this.e423 * right_anti_dual.e15) - (this.e431 * right_anti_dual.e25) - (this.e412 * right_anti_dual.e35) - (this.e415 * right_anti_dual.e23) - (this.e425 * right_anti_dual.e31) - (this.e435 * right_anti_dual.e12) - (this.e321 * right_anti_dual.e45) - (this.e235 * right_anti_dual.e41) - (this.e315 * right_anti_dual.e42) - (this.e125 * right_anti_dual.e43))
        );
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(((float3(wedge.e12345) * other.group0.xyz) + (float3(other.e12345) * wedge.group0.xyz)), wedge.e12345 * other.e12345), 
            /* e415, e425, e435, e321 */
            (float4(wedge.e12345) * other.group1) + (float4(other.e12345) * wedge.group1), 
            /* e235, e315, e125, e5 */
            float4(wedge.e12345 * other.e235, wedge.e12345 * other.e315, wedge.e12345 * other.e125, -(wedge.e415 * other.e235) - (wedge.e425 * other.e315) - (wedge.e435 * other.e125) - (wedge.e235 * other.e415) - (wedge.e315 * other.e425) - (wedge.e125 * other.e435)) + (wedge.group2 * float4(other.group0.www, other.e5)), 
            /* e1, e2, e3, e4 */
            float4((wedge.e415 * other.e321) + (wedge.e321 * other.e415) + (wedge.e315 * other.e412) + (wedge.e12345 * other.e1), (wedge.e425 * other.e321) + (wedge.e321 * other.e425) + (wedge.e125 * other.e423) + (wedge.e12345 * other.e2), (wedge.e435 * other.e321) + (wedge.e321 * other.e435) + (wedge.e235 * other.e431) + (wedge.e12345 * other.e3), -(wedge.e412 * other.e435) - (wedge.e415 * other.e423) - (wedge.e425 * other.e431) - (wedge.e435 * other.e412)) + float4((wedge.group0.xyz.zxy * other.group2.yzx), wedge.e12345 * other.e4) - float4((wedge.group0.xyz.yzx * other.group2.zxy), wedge.e423 * other.e415) - float4((wedge.group2.zxy * other.group0.yzx), wedge.e431 * other.e425)
        );
    }
}
public extension CircleRotor {
    public AntiDipoleInversion project_orthogonally_onto(VersorOdd other) {
        VersorEven right_anti_dual = VersorEven(
            /* e423, e431, e412, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            float4(other.group2.xyz, other.e3215) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4(other.group3.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0)
        );
        Sphere wedge = Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(-(this.e425 * right_anti_dual.e3) - (this.e235 * right_anti_dual.e4), -(this.e435 * right_anti_dual.e1) - (this.e315 * right_anti_dual.e4), -(this.e415 * right_anti_dual.e2) - (this.e125 * right_anti_dual.e4), (this.e315 * right_anti_dual.e2) + (this.e125 * right_anti_dual.e3)) + (float4(right_anti_dual.e5) * float4(this.group0.xyz, this.e321)) + (right_anti_dual.group3.yzxx * float4(this.group1.zxy, this.e235)), 
            /* e1234 */
            -(this.e423 * right_anti_dual.e1) - (this.e431 * right_anti_dual.e2) - (this.e412 * right_anti_dual.e3) - (this.e321 * right_anti_dual.e4)
        );
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (float3(other.e1234) * wedge.group0.xyz) - (float3(wedge.e1234) * other.group3.xyz), 
            /* e415, e425, e435, e321 */
            (wedge.group0.yzxw * float4(other.group3.zxy, other.e1234)) - float4((wedge.group0.zxy * other.group3.yzx), wedge.e1234 * other.e3215), 
            /* e235, e315, e125, e4 */
            float4(wedge.e3215 * other.e4235, wedge.e3215 * other.e4315, wedge.e3215 * other.e4125, -(wedge.e4315 * other.e42) - (wedge.e4125 * other.e43) - (wedge.e1234 * other.e45)) - (wedge.group0.xyzx * float4(other.group3.www, other.e41)), 
            /* e1, e2, e3, e5 */
            float4(-(wedge.e4315 * other.e12) - (wedge.e1234 * other.e15), -(wedge.e4125 * other.e23) - (wedge.e1234 * other.e25), -(wedge.e4235 * other.e31) - (wedge.e1234 * other.e35), (wedge.e4125 * other.e35) + (wedge.e3215 * other.e45)) + (wedge.group0.zxyx * float4(other.group1.yzx, other.e15)) + (wedge.group0.wwwy * float4(other.group0.xyz, other.e25))
        );
    }
}
public extension Dipole {
    // Fancy infix trick (first half)
    public ProjectOrthogonallyOntoInfixPartial<Dipole> operator /(ProjectOrthogonallyOntoInfix rhs) {
        return ProjectOrthogonallyOntoInfixPartial<Dipole>(this);
    }
}
public extension ProjectOrthogonallyOntoInfixPartial<Dipole> {
    // Fancy infix trick (second half)
    public AntiCircleRotor operator /(AntiCircleRotor rhs) {
        return this.a.project_orthogonally_onto(rhs);
    }
}
public extension Dipole {
    public AntiCircleRotor project_orthogonally_onto(AntiCircleRotor other) {
        CircleRotor right_anti_dual = CircleRotor(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        AntiScalar wedge = AntiScalar(
            /* e12345 */
            -(right_anti_dual.e423 * this.e15) - (right_anti_dual.e431 * this.e25) - (right_anti_dual.e412 * this.e35) - (right_anti_dual.e415 * this.e23) - (right_anti_dual.e425 * this.e31) - (right_anti_dual.e435 * this.e12) - (right_anti_dual.e321 * this.e45) - (right_anti_dual.e235 * this.e41) - (right_anti_dual.e315 * this.e42) - (right_anti_dual.e125 * this.e43)
        );
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(wedge.e12345) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(wedge.e12345) * other.group1, 
            /* e15, e25, e35, scalar */
            float4(wedge.e12345) * other.group2
        );
    }
}
public extension Dipole {
    public AntiCircleRotor project_orthogonally_onto(AntiDipoleInversion other) {
        DipoleInversion right_anti_dual = DipoleInversion(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e1234 */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group3 * float4(1.0, 1.0, 1.0, -1.0)
        );
        Sphere wedge = Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((this.e42 * right_anti_dual.e35) + (this.e23 * right_anti_dual.e45) + (this.e45 * right_anti_dual.e23) + (this.e35 * right_anti_dual.e42), (this.e43 * right_anti_dual.e15) + (this.e31 * right_anti_dual.e45) + (this.e45 * right_anti_dual.e31) + (this.e15 * right_anti_dual.e43), (this.e41 * right_anti_dual.e25) + (this.e12 * right_anti_dual.e45) + (this.e45 * right_anti_dual.e12) + (this.e25 * right_anti_dual.e41), -(this.e23 * right_anti_dual.e15) - (this.e31 * right_anti_dual.e25) - (this.e12 * right_anti_dual.e35) - (this.e35 * right_anti_dual.e12)) - float4((this.group0.xyz.zxy * right_anti_dual.group2.yzx), this.e15 * right_anti_dual.e23) - float4((this.group2.xyz.yzx * right_anti_dual.group0.xyz.zxy), this.e25 * right_anti_dual.e31), 
            /* e1234 */
            -(this.e41 * right_anti_dual.e23) - (this.e42 * right_anti_dual.e31) - (this.e43 * right_anti_dual.e12) - (this.e23 * right_anti_dual.e41) - (this.e31 * right_anti_dual.e42) - (this.e12 * right_anti_dual.e43)
        );
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (float3(wedge.e1234) * other.group1.xyz) + (other.group0.xyz.zxy * wedge.group0.yzx) - (other.group0.xyz.yzx * wedge.group0.zxy), 
            /* e23, e31, e12, e45 */
            float4((other.e423 * wedge.e3215) + (other.e235 * wedge.e1234), (other.e431 * wedge.e3215) + (other.e315 * wedge.e1234), (other.e412 * wedge.e3215) + (other.e125 * wedge.e1234), -(other.e425 * wedge.e4315) - (other.e435 * wedge.e4125)) - (other.group1.wwwx * wedge.group0.xyzx), 
            /* e15, e25, e35, scalar */
            float4(other.e125 * wedge.e4315 * -1.0, other.e235 * wedge.e4125 * -1.0, other.e315 * wedge.e4235 * -1.0, (other.e2 * wedge.e4315) + (other.e3 * wedge.e4125) + (other.e5 * wedge.e1234)) + (float4(wedge.e3215) * float4(other.group1.xyz, other.e4)) + (wedge.group0.zxyx * float4(other.group2.yzx, other.e1))
        );
    }
}
public extension Dipole {
    public FlatPoint project_orthogonally_onto(AntiDualNum other) {
        DualNum right_anti_dual = DualNum(
            /* e5, e12345 */
            other.group0.xy
        );
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(other.group0.xy.xx, other.e3215, 0.0) * float4(float3(1.0), 0.0) * float4(float3(this.e41 * right_anti_dual.e5, this.e42 * right_anti_dual.e5, this.e43 * right_anti_dual.e5), 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
public extension Dipole {
    public AntiLine project_orthogonally_onto(AntiFlatPoint other) {
        FlatPoint right_anti_dual = FlatPoint(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        Plane wedge = Plane(
            /* e4235, e4315, e4125, e3215 */
            float4((this.e42 * right_anti_dual.e35) + (this.e23 * right_anti_dual.e45), (this.e43 * right_anti_dual.e15) + (this.e31 * right_anti_dual.e45), (this.e41 * right_anti_dual.e25) + (this.e12 * right_anti_dual.e45), -(this.e31 * right_anti_dual.e25) - (this.e12 * right_anti_dual.e35)) - (right_anti_dual.group0.yzxx * float4(this.group0.xyz.zxy, this.e23))
        );
        return AntiLine(
            /* e23, e31, e12 */
            float3(other.e321) * wedge.group0.xyz * float3(-1.0), 
            /* e15, e25, e35 */
            (other.group0.yzx * wedge.group0.zxy) - (other.group0.zxy * wedge.group0.yzx)
        );
    }
}
public extension Dipole {
    public AntiMotor project_orthogonally_onto(AntiFlector other) {
        Flector right_anti_dual = Flector(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        Plane wedge = Plane(
            /* e4235, e4315, e4125, e3215 */
            float4((this.e42 * right_anti_dual.e35) + (this.e23 * right_anti_dual.e45), (this.e43 * right_anti_dual.e15) + (this.e31 * right_anti_dual.e45), (this.e41 * right_anti_dual.e25) + (this.e12 * right_anti_dual.e45), -(this.e31 * right_anti_dual.e25) - (this.e12 * right_anti_dual.e35)) - (right_anti_dual.group0.yzxx * float4(this.group0.xyz.zxy, this.e23))
        );
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((other.group0.www * wedge.group0.xyz * float3(-1.0)), (other.e1 * wedge.e4235) + (other.e2 * wedge.e4315) + (other.e3 * wedge.e4125)), 
            /* e15, e25, e35, e3215 */
            float4(((other.group0.yzx * wedge.group0.zxy) - (other.group0.zxy * wedge.group0.yzx)), 0.0)
        );
    }
}
public extension Dipole {
    public AntiLine project_orthogonally_onto(AntiLine other) {
        Line right_anti_dual = Line(
            /* e415, e425, e435 */
            other.group0.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            other.group1.xyz * float3(-1.0)
        );
        AntiScalar wedge = AntiScalar(
            /* e12345 */
            -(this.e41 * right_anti_dual.e235) - (this.e42 * right_anti_dual.e315) - (this.e43 * right_anti_dual.e125) - (this.e23 * right_anti_dual.e415) - (this.e31 * right_anti_dual.e425) - (this.e12 * right_anti_dual.e435)
        );
        return AntiLine(
            /* e23, e31, e12 */
            float3(wedge.e12345) * other.group0.xyz, 
            /* e15, e25, e35 */
            float3(wedge.e12345) * other.group1.xyz
        );
    }
}
public extension Dipole {
    public AntiMotor project_orthogonally_onto(AntiMotor other) {
        Motor right_anti_dual = Motor(
            /* e415, e425, e435, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        Motor wedge = Motor(
            /* e415, e425, e435, e12345 */
            float4((float3(right_anti_dual.e5) * this.group0.xyz), -(this.e41 * right_anti_dual.e235) - (this.e42 * right_anti_dual.e315) - (this.e43 * right_anti_dual.e125) - (this.e23 * right_anti_dual.e415) - (this.e31 * right_anti_dual.e425) - (this.e12 * right_anti_dual.e435)), 
            /* e235, e315, e125, e5 */
            float4(float3(1.0), 0.0) * float4(right_anti_dual.group1.www, 0.0) * float4(this.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((float3(wedge.e12345) * other.group0.xyz), (other.scalar * wedge.e12345) - (other.e23 * wedge.e415) - (other.e31 * wedge.e425) - (other.e12 * wedge.e435)), 
            /* e15, e25, e35, e3215 */
            float4(((float3(other.e3215) * wedge.group0.xyz) + (float3(wedge.e12345) * other.group1.xyz)), other.e3215 * wedge.e12345)
        );
    }
}
public extension Dipole {
    public Dipole project_orthogonally_onto(AntiScalar other) {
        Scalar right_anti_dual = Scalar(
            /* scalar */
            other.e12345 * -1.0
        );
        Dipole wedge = Dipole(
            /* e41, e42, e43 */
            float3(right_anti_dual.scalar) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(right_anti_dual.scalar) * this.group1, 
            /* e15, e25, e35 */
            float3(right_anti_dual.scalar) * this.group2.xyz
        );
        return Dipole(
            /* e41, e42, e43 */
            float3(other.e12345) * wedge.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(other.e12345) * wedge.group1, 
            /* e15, e25, e35 */
            float3(other.e12345) * wedge.group2.xyz
        );
    }
}
public extension Dipole {
    public Dipole project_orthogonally_onto(Circle other) {
        Dipole right_anti_dual = Dipole(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35 */
            other.group2.xyz
        );
        Sphere wedge = Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual.e42 * this.e35) + (right_anti_dual.e23 * this.e45) + (right_anti_dual.e45 * this.e23) + (right_anti_dual.e35 * this.e42), (right_anti_dual.e43 * this.e15) + (right_anti_dual.e31 * this.e45) + (right_anti_dual.e45 * this.e31) + (right_anti_dual.e15 * this.e43), (right_anti_dual.e41 * this.e25) + (right_anti_dual.e12 * this.e45) + (right_anti_dual.e45 * this.e12) + (right_anti_dual.e25 * this.e41), -(right_anti_dual.e23 * this.e15) - (right_anti_dual.e31 * this.e25) - (right_anti_dual.e12 * this.e35) - (right_anti_dual.e35 * this.e12)) - float4((right_anti_dual.group0.xyz.zxy * this.group2.xyz.yzx), right_anti_dual.e15 * this.e23) - float4((right_anti_dual.group2.xyz.yzx * this.group0.xyz.zxy), right_anti_dual.e25 * this.e31), 
            /* e1234 */
            -(right_anti_dual.e41 * this.e23) - (right_anti_dual.e42 * this.e31) - (right_anti_dual.e43 * this.e12) - (right_anti_dual.e23 * this.e41) - (right_anti_dual.e31 * this.e42) - (right_anti_dual.e12 * this.e43)
        );
        return Dipole(
            /* e41, e42, e43 */
            (float3(wedge.e1234) * other.group1.xyz) + (other.group0.xyz.zxy * wedge.group0.yzx) - (other.group0.xyz.yzx * wedge.group0.zxy), 
            /* e23, e31, e12, e45 */
            float4((other.e423 * wedge.e3215) + (other.e235 * wedge.e1234), (other.e431 * wedge.e3215) + (other.e315 * wedge.e1234), (other.e412 * wedge.e3215) + (other.e125 * wedge.e1234), -(other.e425 * wedge.e4315) - (other.e435 * wedge.e4125)) - (other.group1.wwwx * wedge.group0.xyzx), 
            /* e15, e25, e35 */
            (float3(wedge.e3215) * other.group1.xyz) + (other.group2.xyz.yzx * wedge.group0.zxy) - (other.group2.xyz.zxy * wedge.group0.yzx)
        );
    }
}
public extension Dipole {
    public VersorOdd project_orthogonally_onto(CircleRotor other) {
        AntiCircleRotor right_anti_dual = AntiCircleRotor(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        DipoleInversion wedge = DipoleInversion(
            /* e41, e42, e43 */
            float3(right_anti_dual.scalar) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(right_anti_dual.scalar) * this.group1, 
            /* e15, e25, e35, e1234 */
            float4((float3(right_anti_dual.scalar) * this.group2.xyz), -(right_anti_dual.e41 * this.e23) - (right_anti_dual.e42 * this.e31) - (right_anti_dual.e43 * this.e12) - (right_anti_dual.e23 * this.e41) - (right_anti_dual.e31 * this.e42) - (right_anti_dual.e12 * this.e43)), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual.e42 * this.e35) + (right_anti_dual.e23 * this.e45) + (right_anti_dual.e45 * this.e23) + (right_anti_dual.e35 * this.e42), (right_anti_dual.e43 * this.e15) + (right_anti_dual.e31 * this.e45) + (right_anti_dual.e45 * this.e31) + (right_anti_dual.e15 * this.e43), (right_anti_dual.e41 * this.e25) + (right_anti_dual.e12 * this.e45) + (right_anti_dual.e45 * this.e12) + (right_anti_dual.e25 * this.e41), -(right_anti_dual.e12 * this.e35) - (right_anti_dual.e15 * this.e23) - (right_anti_dual.e25 * this.e31) - (right_anti_dual.e35 * this.e12)) - float4((right_anti_dual.group0.xyz.zxy * this.group2.xyz.yzx), right_anti_dual.e23 * this.e15) - float4((this.group0.xyz.zxy * right_anti_dual.group2.yzx), right_anti_dual.e31 * this.e25)
        );
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4((other.e412 * wedge.e4315) + (other.e415 * wedge.e1234) + (other.e12345 * wedge.e41), (other.e423 * wedge.e4125) + (other.e425 * wedge.e1234) + (other.e12345 * wedge.e42), (other.e431 * wedge.e4235) + (other.e435 * wedge.e1234) + (other.e12345 * wedge.e43), -(other.e431 * wedge.e25) - (other.e412 * wedge.e35) - (other.e415 * wedge.e23) - (other.e425 * wedge.e31) - (other.e435 * wedge.e12) - (other.e321 * wedge.e45) - (other.e235 * wedge.e41) - (other.e315 * wedge.e42) - (other.e125 * wedge.e43)) - float4((other.group0.xyz.yzx * wedge.group3.zxy), other.e423 * wedge.e15), 
            /* e23, e31, e12, e45 */
            float4((other.e235 * wedge.e1234) + (other.e12345 * wedge.e23), (other.e315 * wedge.e1234) + (other.e12345 * wedge.e31), (other.e125 * wedge.e1234) + (other.e12345 * wedge.e12), -(other.e425 * wedge.e4315) - (other.e435 * wedge.e4125)) + float4((other.group0.xyz * wedge.group3.www), other.e12345 * wedge.e45) - (other.group1.wwwx * wedge.group3.xyzx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(other.e12345) * wedge.group2.xyz) + (float3(wedge.e3215) * other.group1.xyz) + (other.group2.yzx * wedge.group3.zxy) - (other.group2.zxy * wedge.group3.yzx)), other.e12345 * wedge.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345) * wedge.group3
        );
    }
}
public extension Dipole {
    public Dipole project_orthogonally_onto(Dipole other) {
        Circle right_anti_dual = Circle(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125 */
            other.group2.xyz * float3(-1.0)
        );
        AntiScalar wedge = AntiScalar(
            /* e12345 */
            -(right_anti_dual.e423 * this.e15) - (right_anti_dual.e431 * this.e25) - (right_anti_dual.e412 * this.e35) - (right_anti_dual.e415 * this.e23) - (right_anti_dual.e425 * this.e31) - (right_anti_dual.e435 * this.e12) - (right_anti_dual.e321 * this.e45) - (right_anti_dual.e235 * this.e41) - (right_anti_dual.e315 * this.e42) - (right_anti_dual.e125 * this.e43)
        );
        return Dipole(
            /* e41, e42, e43 */
            float3(wedge.e12345) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(wedge.e12345) * other.group1, 
            /* e15, e25, e35 */
            float3(wedge.e12345) * other.group2.xyz
        );
    }
}
public extension Dipole {
    public VersorOdd project_orthogonally_onto(DipoleInversion other) {
        AntiDipoleInversion right_anti_dual = AntiDipoleInversion(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e4 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group3 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        CircleRotor wedge = CircleRotor(
            /* e423, e431, e412 */
            (float3(right_anti_dual.e4) * this.group1.xyz) + (this.group0.xyz.yzx * right_anti_dual.group3.zxy) - (this.group0.xyz.zxy * right_anti_dual.group3.yzx), 
            /* e415, e425, e435, e321 */
            float4((right_anti_dual.e4 * this.e15) + (right_anti_dual.e5 * this.e41), (right_anti_dual.e4 * this.e25) + (right_anti_dual.e5 * this.e42), (right_anti_dual.e4 * this.e35) + (right_anti_dual.e5 * this.e43), -(right_anti_dual.e2 * this.e31) - (right_anti_dual.e3 * this.e12)) - (right_anti_dual.group3.xyzx * this.group1.wwwx), 
            /* e235, e315, e125, e12345 */
            float4((right_anti_dual.e2 * this.e35) + (right_anti_dual.e5 * this.e23), (right_anti_dual.e3 * this.e15) + (right_anti_dual.e5 * this.e31), (right_anti_dual.e1 * this.e25) + (right_anti_dual.e5 * this.e12), -(right_anti_dual.e431 * this.e25) - (right_anti_dual.e412 * this.e35) - (right_anti_dual.e415 * this.e23) - (right_anti_dual.e425 * this.e31) - (right_anti_dual.e435 * this.e12) - (right_anti_dual.e321 * this.e45) - (right_anti_dual.e235 * this.e41) - (right_anti_dual.e315 * this.e42) - (right_anti_dual.e125 * this.e43)) - float4((this.group2.xyz.yzx * right_anti_dual.group3.zxy), right_anti_dual.e423 * this.e15)
        );
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4((wedge.e412 * other.e4315) + (wedge.e415 * other.e1234) + (wedge.e12345 * other.e41), (wedge.e423 * other.e4125) + (wedge.e425 * other.e1234) + (wedge.e12345 * other.e42), (wedge.e431 * other.e4235) + (wedge.e435 * other.e1234) + (wedge.e12345 * other.e43), -(wedge.e431 * other.e25) - (wedge.e412 * other.e35) - (wedge.e415 * other.e23) - (wedge.e425 * other.e31) - (wedge.e435 * other.e12) - (wedge.e321 * other.e45) - (wedge.e235 * other.e41) - (wedge.e315 * other.e42) - (wedge.e125 * other.e43)) - float4((wedge.group0.xyz.yzx * other.group3.zxy), wedge.e423 * other.e15), 
            /* e23, e31, e12, e45 */
            float4((wedge.e235 * other.e1234) + (wedge.e12345 * other.e23), (wedge.e315 * other.e1234) + (wedge.e12345 * other.e31), (wedge.e125 * other.e1234) + (wedge.e12345 * other.e12), -(wedge.e425 * other.e4315) - (wedge.e435 * other.e4125)) + float4((wedge.group0.xyz * other.group3.www), wedge.e12345 * other.e45) - (wedge.group1.wwwx * other.group3.xyzx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(wedge.e12345) * other.group2.xyz) + (float3(other.e3215) * wedge.group1.xyz) + (wedge.group2.yzx * other.group3.zxy) - (wedge.group2.zxy * other.group3.yzx)), wedge.e12345 * other.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(wedge.e12345) * other.group3
        );
    }
}
public extension Dipole {
    public Dipole project_orthogonally_onto(DualNum other) {
        AntiDualNum right_anti_dual = AntiDualNum(
            /* e3215, scalar */
            other.group0.xy * float2(-1.0)
        );
        Dipole wedge = Dipole(
            /* e41, e42, e43 */
            float3(right_anti_dual.scalar) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(right_anti_dual.scalar) * this.group1, 
            /* e15, e25, e35 */
            float3(right_anti_dual.scalar) * this.group2.xyz
        );
        return Dipole(
            /* e41, e42, e43 */
            float3(other.e12345) * wedge.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(other.e12345) * wedge.group1, 
            /* e15, e25, e35 */
            float3(other.e12345) * wedge.group2.xyz
        );
    }
}
public extension Dipole {
    public FlatPoint project_orthogonally_onto(FlatPoint other) {
        AntiFlatPoint right_anti_dual = AntiFlatPoint(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(-(right_anti_dual.e235 * this.e41) - (right_anti_dual.e315 * this.e42) - (right_anti_dual.e125 * this.e43) - (right_anti_dual.e321 * this.e45)) * other.group0
        );
    }
}
public extension Dipole {
    public VersorOdd project_orthogonally_onto(Flector other) {
        AntiFlector right_anti_dual = AntiFlector(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        CircleRotor wedge = CircleRotor(
            /* e423, e431, e412 */
            (this.group0.xyz.yzx * right_anti_dual.group1.zxy) - (this.group0.xyz.zxy * right_anti_dual.group1.yzx), 
            /* e415, e425, e435, e321 */
            float4(right_anti_dual.e5 * this.e41, right_anti_dual.e5 * this.e42, right_anti_dual.e5 * this.e43, -(right_anti_dual.e2 * this.e31) - (right_anti_dual.e3 * this.e12)) - (right_anti_dual.group1.xyzx * this.group1.wwwx), 
            /* e235, e315, e125, e12345 */
            float4((right_anti_dual.e2 * this.e35) + (right_anti_dual.e5 * this.e23), (right_anti_dual.e3 * this.e15) + (right_anti_dual.e5 * this.e31), (right_anti_dual.e1 * this.e25) + (right_anti_dual.e5 * this.e12), -(right_anti_dual.e315 * this.e42) - (right_anti_dual.e125 * this.e43) - (right_anti_dual.e321 * this.e45)) - float4((this.group2.xyz.yzx * right_anti_dual.group1.zxy), right_anti_dual.e235 * this.e41)
        );
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(wedge.e412 * other.e4315, wedge.e423 * other.e4125, wedge.e431 * other.e4235, -(wedge.e431 * other.e25) - (wedge.e412 * other.e35) - (wedge.e321 * other.e45)) - float4((wedge.group0.xyz.yzx * other.group1.zxy), wedge.e423 * other.e15), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), -(wedge.e425 * other.e4315) - (wedge.e435 * other.e4125)) + float4((wedge.group0.xyz * other.group1.www), wedge.e12345 * other.e45) - (wedge.group1.wwwx * other.group1.xyzx), 
            /* e15, e25, e35, e1234 */
            float4((float3(wedge.e12345) * other.group0.xyz), 0.0) + float4((float3(other.e3215) * wedge.group1.xyz), 0.0) + float4((wedge.group2.yzx * other.group1.zxy), 0.0) - float4((wedge.group2.zxy * other.group1.yzx), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(wedge.e12345) * other.group1
        );
    }
}
public extension Dipole {
    public Dipole project_orthogonally_onto(Line other) {
        AntiLine right_anti_dual = AntiLine(
            /* e23, e31, e12 */
            other.group0.xyz, 
            /* e15, e25, e35 */
            other.group1.xyz
        );
        Sphere wedge = Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual.e23 * this.e45) + (right_anti_dual.e35 * this.e42), (right_anti_dual.e31 * this.e45) + (right_anti_dual.e15 * this.e43), (right_anti_dual.e12 * this.e45) + (right_anti_dual.e25 * this.e41), -(right_anti_dual.e31 * this.e25) - (right_anti_dual.e12 * this.e35) - (right_anti_dual.e15 * this.e23) - (right_anti_dual.e25 * this.e31) - (right_anti_dual.e35 * this.e12)) - float4((right_anti_dual.group1.xyz.yzx * this.group0.xyz.zxy), right_anti_dual.e23 * this.e15), 
            /* e1234 */
            -(right_anti_dual.e23 * this.e41) - (right_anti_dual.e31 * this.e42) - (right_anti_dual.e12 * this.e43)
        );
        return Dipole(
            /* e41, e42, e43 */
            float3(wedge.e1234) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4((float3(wedge.e1234) * other.group1.xyz), -(other.e415 * wedge.e4235) - (other.e425 * wedge.e4315) - (other.e435 * wedge.e4125)), 
            /* e15, e25, e35 */
            (float3(wedge.e3215) * other.group0.xyz) + (other.group1.xyz.yzx * wedge.group0.zxy) - (other.group1.xyz.zxy * wedge.group0.yzx)
        );
    }
}
public extension Dipole {
    public VersorOdd project_orthogonally_onto(Motor other) {
        AntiMotor right_anti_dual = AntiMotor(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        DipoleInversion wedge = DipoleInversion(
            /* e41, e42, e43 */
            float3(right_anti_dual.scalar) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(right_anti_dual.scalar) * this.group1, 
            /* e15, e25, e35, e1234 */
            float4((float3(right_anti_dual.scalar) * this.group2.xyz), -(right_anti_dual.e23 * this.e41) - (right_anti_dual.e31 * this.e42) - (right_anti_dual.e12 * this.e43)), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual.e23 * this.e45) + (right_anti_dual.e35 * this.e42), (right_anti_dual.e31 * this.e45) + (right_anti_dual.e15 * this.e43), (right_anti_dual.e12 * this.e45) + (right_anti_dual.e25 * this.e41), -(right_anti_dual.e31 * this.e25) - (right_anti_dual.e12 * this.e35) - (right_anti_dual.e15 * this.e23) - (right_anti_dual.e25 * this.e31) - (right_anti_dual.e35 * this.e12)) - float4((this.group0.xyz.zxy * right_anti_dual.group1.yzx), right_anti_dual.e23 * this.e15)
        );
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(wedge.e1234 * other.e415, wedge.e1234 * other.e425, wedge.e1234 * other.e435, -(wedge.e41 * other.e235) - (wedge.e42 * other.e315) - (wedge.e43 * other.e125) - (wedge.e23 * other.e415) - (wedge.e31 * other.e425) - (wedge.e12 * other.e435)) + float4((wedge.group0.xyz * other.group0.www), wedge.e1234 * other.e5), 
            /* e23, e31, e12, e45 */
            float4(wedge.e1234 * other.e235, wedge.e1234 * other.e315, wedge.e1234 * other.e125, -(wedge.e4235 * other.e415) - (wedge.e4315 * other.e425) - (wedge.e4125 * other.e435)) + (float4(other.e12345) * wedge.group1), 
            /* e15, e25, e35, e1234 */
            float4(((float3(wedge.e3215) * other.group0.xyz) + (float3(other.e12345) * wedge.group2.xyz) + (wedge.group3.zxy * other.group1.yzx) - (wedge.group3.yzx * other.group1.zxy)), wedge.e1234 * other.e12345), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345) * wedge.group3
        );
    }
}
public extension Dipole {
    public MultiVector project_orthogonally_onto(MultiVector other) {
        MultiVector right_anti_dual = MultiVector(
            /* scalar, e12345 */
            other.group0.xy.yx * float2(-1.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4(other.group9.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5 */
            other.e3215, 
            /* e15, e25, e35, e45 */
            float4(other.group8.xyz, other.e321 * -1.0), 
            /* e41, e42, e43 */
            other.group7.xyz, 
            /* e23, e31, e12 */
            other.group6.xyz, 
            /* e415, e425, e435, e321 */
            float4(other.group5.xyz, other.e45) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e423, e431, e412 */
            other.group4.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            other.group3.xyz * float3(-1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.group1.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234 */
            other.e4 * -1.0
        );
        MultiVector wedge = MultiVector(
            /* scalar, e12345 */
            float2(0.0, -(this.e41 * right_anti_dual.e235) - (this.e42 * right_anti_dual.e315) - (this.e43 * right_anti_dual.e125) - (this.e23 * right_anti_dual.e415) - (this.e31 * right_anti_dual.e425) - (this.e12 * right_anti_dual.e435) - (this.e45 * right_anti_dual.e321) - (this.e15 * right_anti_dual.e423) - (this.e25 * right_anti_dual.e431) - (this.e35 * right_anti_dual.e412)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            float4(right_anti_dual.scalar) * float4(this.group2.xyz, this.e45), 
            /* e41, e42, e43 */
            float3(right_anti_dual.scalar) * this.group0.xyz, 
            /* e23, e31, e12 */
            float3(right_anti_dual.scalar) * this.group1.xyz, 
            /* e415, e425, e435, e321 */
            float4((this.e41 * right_anti_dual.e5) + (this.e15 * right_anti_dual.e4), (this.e42 * right_anti_dual.e5) + (this.e25 * right_anti_dual.e4), (this.e43 * right_anti_dual.e5) + (this.e35 * right_anti_dual.e4), -(this.e31 * right_anti_dual.e2) - (this.e12 * right_anti_dual.e3)) - (this.group1.wwwx * right_anti_dual.group1.xyzx), 
            /* e423, e431, e412 */
            (float3(right_anti_dual.e4) * this.group1.xyz) + (this.group0.xyz.yzx * right_anti_dual.group1.zxy) - (this.group0.xyz.zxy * right_anti_dual.group1.yzx), 
            /* e235, e315, e125 */
            (float3(right_anti_dual.e5) * this.group1.xyz) + (this.group2.xyz.zxy * right_anti_dual.group1.yzx) - (this.group2.xyz.yzx * right_anti_dual.group1.zxy), 
            /* e4235, e4315, e4125, e3215 */
            float4((this.e42 * right_anti_dual.e35) + (this.e23 * right_anti_dual.e45) + (this.e45 * right_anti_dual.e23) + (this.e35 * right_anti_dual.e42), (this.e43 * right_anti_dual.e15) + (this.e31 * right_anti_dual.e45) + (this.e45 * right_anti_dual.e31) + (this.e15 * right_anti_dual.e43), (this.e41 * right_anti_dual.e25) + (this.e12 * right_anti_dual.e45) + (this.e45 * right_anti_dual.e12) + (this.e25 * right_anti_dual.e41), -(this.e23 * right_anti_dual.e15) - (this.e31 * right_anti_dual.e25) - (this.e12 * right_anti_dual.e35) - (this.e35 * right_anti_dual.e12)) - float4((this.group0.xyz.zxy * right_anti_dual.group3.yzx), this.e15 * right_anti_dual.e23) - float4((this.group2.xyz.yzx * right_anti_dual.group4.xyz.zxy), this.e25 * right_anti_dual.e31), 
            /* e1234 */
            -(this.e41 * right_anti_dual.e23) - (this.e42 * right_anti_dual.e31) - (this.e43 * right_anti_dual.e12) - (this.e23 * right_anti_dual.e41) - (this.e31 * right_anti_dual.e42) - (this.e12 * right_anti_dual.e43)
        );
        return MultiVector(
            /* scalar, e12345 */
            float2((other.scalar * wedge.e12345) + (other.e12345 * wedge.scalar) + (other.e1 * wedge.e4235) + (other.e2 * wedge.e4315) + (other.e3 * wedge.e4125) + (other.e4 * wedge.e3215) + (other.e5 * wedge.e1234) + (other.e4235 * wedge.e1) + (other.e4315 * wedge.e2) + (other.e4125 * wedge.e3) + (other.e3215 * wedge.e4) + (other.e1234 * wedge.e5) - (other.e15 * wedge.e423) - (other.e25 * wedge.e431) - (other.e35 * wedge.e412) - (other.e45 * wedge.e321) - (other.e41 * wedge.e235) - (other.e42 * wedge.e315) - (other.e43 * wedge.e125) - (other.e23 * wedge.e415) - (other.e31 * wedge.e425) - (other.e12 * wedge.e435) - (other.e415 * wedge.e23) - (other.e425 * wedge.e31) - (other.e435 * wedge.e12) - (other.e321 * wedge.e45) - (other.e423 * wedge.e15) - (other.e431 * wedge.e25) - (other.e412 * wedge.e35) - (other.e235 * wedge.e41) - (other.e315 * wedge.e42) - (other.e125 * wedge.e43), other.e12345 * wedge.e12345), 
            /* e1, e2, e3, e4 */
            float4((other.e415 * wedge.e321) + (other.e321 * wedge.e415) + (other.e4315 * wedge.e12) + (other.e1234 * wedge.e15), (other.e425 * wedge.e321) + (other.e321 * wedge.e425) + (other.e4125 * wedge.e23) + (other.e1234 * wedge.e25), (other.e435 * wedge.e321) + (other.e321 * wedge.e435) + (other.e4235 * wedge.e31) + (other.e1234 * wedge.e35), -(other.e45 * wedge.e1234) - (other.e415 * wedge.e423) - (other.e425 * wedge.e431) - (other.e435 * wedge.e412)) + (float4(other.e12345) * wedge.group1) + (float4(wedge.e12345) * other.group1) + float4((other.group4.xyz * wedge.group9.www), other.e4235 * wedge.e41) + float4((other.group5.xyz.yzx * wedge.group9.zxy), other.e4315 * wedge.e42) + float4((other.group7.xyz.zxy * wedge.group8.xyz.yzx), other.e4125 * wedge.e43) + float4((other.group8.xyz.yzx * wedge.group7.xyz.zxy), other.e1234 * wedge.e45) - (wedge.group9.yzxx * float4(other.group5.xyz.zxy, other.e41)) - float4((float3(wedge.e1234) * other.group3.xyz), other.e412 * wedge.e435) - float4((wedge.group4.xyz * other.group9.www), other.e423 * wedge.e415) - float4((other.group7.xyz.yzx * wedge.group8.xyz.zxy), other.e42 * wedge.e4315) - float4((other.group8.xyz.zxy * wedge.group7.xyz.yzx), other.e43 * wedge.e4125) - float4((wedge.group5.xyz.yzx * other.group9.zxy), other.e431 * wedge.e425), 
            /* e5 */
            (other.e12345 * wedge.e5) + (other.e5 * wedge.e12345) + (other.e15 * wedge.e4235) + (other.e25 * wedge.e4315) + (other.e35 * wedge.e4125) + (other.e45 * wedge.e3215) - (other.e415 * wedge.e235) - (other.e425 * wedge.e315) - (other.e435 * wedge.e125) - (other.e235 * wedge.e415) - (other.e315 * wedge.e425) - (other.e125 * wedge.e435) - (other.e4235 * wedge.e15) - (other.e4315 * wedge.e25) - (other.e4125 * wedge.e35) - (other.e3215 * wedge.e45), 
            /* e15, e25, e35, e45 */
            float4((other.e415 * wedge.e3215) + (other.e315 * wedge.e4125) + (other.e4125 * wedge.e315) + (other.e3215 * wedge.e415), (other.e425 * wedge.e3215) + (other.e125 * wedge.e4235) + (other.e4235 * wedge.e125) + (other.e3215 * wedge.e425), (other.e435 * wedge.e3215) + (other.e235 * wedge.e4315) + (other.e4315 * wedge.e235) + (other.e3215 * wedge.e435), -(other.e435 * wedge.e4125) - (other.e4235 * wedge.e415) - (other.e4315 * wedge.e425) - (other.e4125 * wedge.e435)) + (float4(other.e12345) * wedge.group3) + (float4(wedge.e12345) * other.group3) - (wedge.group9.yzxx * float4(other.group8.xyz.zxy, other.e415)) - float4((wedge.group8.xyz.zxy * other.group9.yzx), other.e425 * wedge.e4315), 
            /* e41, e42, e43 */
            (float3(other.e12345) * wedge.group4.xyz) + (float3(other.e1234) * wedge.group6.xyz) + (float3(wedge.e12345) * other.group4.xyz) + (float3(wedge.e1234) * other.group6.xyz) + (other.group7.xyz.zxy * wedge.group9.yzx) + (wedge.group7.xyz.zxy * other.group9.yzx) - (other.group7.xyz.yzx * wedge.group9.zxy) - (wedge.group7.xyz.yzx * other.group9.zxy), 
            /* e23, e31, e12 */
            (float3(other.e12345) * wedge.group5.xyz) + (float3(other.e3215) * wedge.group7.xyz) + (float3(other.e1234) * wedge.group8.xyz) + (float3(wedge.e12345) * other.group5.xyz) + (float3(wedge.e3215) * other.group7.xyz) + (float3(wedge.e1234) * other.group8.xyz) - (float3(other.e321) * wedge.group9.xyz) - (float3(wedge.e321) * other.group9.xyz), 
            /* e415, e425, e435, e321 */
            (float4(other.e12345) * wedge.group6) + (float4(wedge.e12345) * other.group6) + float4((other.group9.zxy * wedge.group9.yzx), other.e1234 * wedge.e3215) - (other.group9.yzxw * float4(wedge.group9.zxy, wedge.e1234)), 
            /* e423, e431, e412 */
            (float3(other.e12345) * wedge.group7.xyz) + (float3(other.e1234) * wedge.group9.xyz) + (float3(wedge.e12345) * other.group7.xyz) - (float3(wedge.e1234) * other.group9.xyz), 
            /* e235, e315, e125 */
            (float3(other.e12345) * wedge.group8.xyz) + (float3(wedge.e12345) * other.group8.xyz) + (float3(wedge.e3215) * other.group9.xyz) - (float3(other.e3215) * wedge.group9.xyz), 
            /* e4235, e4315, e4125, e3215 */
            (float4(other.e12345) * wedge.group9) + (float4(wedge.e12345) * other.group9), 
            /* e1234 */
            (other.e12345 * wedge.e1234) + (other.e1234 * wedge.e12345)
        );
    }
}
public extension Dipole {
    public Dipole project_orthogonally_onto(Plane other) {
        AntiPlane right_anti_dual = AntiPlane(
            /* e1, e2, e3, e5 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        Circle wedge = Circle(
            /* e423, e431, e412 */
            (this.group0.xyz.yzx * right_anti_dual.group0.zxy) - (this.group0.xyz.zxy * right_anti_dual.group0.yzx), 
            /* e415, e425, e435, e321 */
            float4(right_anti_dual.e5 * this.e41, right_anti_dual.e5 * this.e42, right_anti_dual.e5 * this.e43, -(right_anti_dual.e2 * this.e31) - (right_anti_dual.e3 * this.e12)) - (right_anti_dual.group0.xyzx * this.group1.wwwx), 
            /* e235, e315, e125 */
            (float3(right_anti_dual.e5) * this.group1.xyz) + (this.group2.xyz.zxy * right_anti_dual.group0.yzx) - (this.group2.xyz.yzx * right_anti_dual.group0.zxy)
        );
        return Dipole(
            /* e41, e42, e43 */
            (wedge.group0.xyz.zxy * other.group0.yzx) - (wedge.group0.xyz.yzx * other.group0.zxy), 
            /* e23, e31, e12, e45 */
            float4(wedge.e423 * other.e3215, wedge.e431 * other.e3215, wedge.e412 * other.e3215, -(wedge.e425 * other.e4315) - (wedge.e435 * other.e4125)) - (wedge.group1.wwwx * other.group0.xyzx), 
            /* e15, e25, e35 */
            (float3(other.e3215) * wedge.group1.xyz) + (wedge.group2.xyz.yzx * other.group0.zxy) - (wedge.group2.xyz.zxy * other.group0.yzx)
        );
    }
}
public extension Dipole {
    public Dipole project_orthogonally_onto(Sphere other) {
        RoundPoint right_anti_dual = RoundPoint(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5 */
            other.e3215
        );
        Circle wedge = Circle(
            /* e423, e431, e412 */
            (float3(right_anti_dual.e4) * this.group1.xyz) + (this.group0.xyz.yzx * right_anti_dual.group0.zxy) - (this.group0.xyz.zxy * right_anti_dual.group0.yzx), 
            /* e415, e425, e435, e321 */
            float4((this.e41 * right_anti_dual.e5) + (this.e15 * right_anti_dual.e4), (this.e42 * right_anti_dual.e5) + (this.e25 * right_anti_dual.e4), (this.e43 * right_anti_dual.e5) + (this.e35 * right_anti_dual.e4), -(this.e31 * right_anti_dual.e2) - (this.e12 * right_anti_dual.e3)) - (this.group1.wwwx * right_anti_dual.group0.xyzx), 
            /* e235, e315, e125 */
            (float3(right_anti_dual.e5) * this.group1.xyz) + (this.group2.xyz.zxy * right_anti_dual.group0.yzx) - (this.group2.xyz.yzx * right_anti_dual.group0.zxy)
        );
        return Dipole(
            /* e41, e42, e43 */
            (float3(other.e1234) * wedge.group1.xyz) + (wedge.group0.xyz.zxy * other.group0.yzx) - (wedge.group0.xyz.yzx * other.group0.zxy), 
            /* e23, e31, e12, e45 */
            float4((wedge.e423 * other.e3215) + (wedge.e235 * other.e1234), (wedge.e431 * other.e3215) + (wedge.e315 * other.e1234), (wedge.e412 * other.e3215) + (wedge.e125 * other.e1234), -(wedge.e425 * other.e4315) - (wedge.e435 * other.e4125)) - (wedge.group1.wwwx * other.group0.xyzx), 
            /* e15, e25, e35 */
            (float3(other.e3215) * wedge.group1.xyz) + (wedge.group2.xyz.yzx * other.group0.zxy) - (wedge.group2.xyz.zxy * other.group0.yzx)
        );
    }
}
public extension Dipole {
    public VersorOdd project_orthogonally_onto(VersorEven other) {
        VersorOdd right_anti_dual = VersorOdd(
            /* e41, e42, e43, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e1234 */
            float4(other.group2.xyz, other.e4) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.group3.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0)
        );
        DipoleInversion wedge = DipoleInversion(
            /* e41, e42, e43 */
            float3(right_anti_dual.scalar) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(right_anti_dual.scalar) * this.group1, 
            /* e15, e25, e35, e1234 */
            float4((float3(right_anti_dual.scalar) * this.group2.xyz), -(this.e41 * right_anti_dual.e23) - (this.e42 * right_anti_dual.e31) - (this.e43 * right_anti_dual.e12) - (this.e23 * right_anti_dual.e41) - (this.e31 * right_anti_dual.e42) - (this.e12 * right_anti_dual.e43)), 
            /* e4235, e4315, e4125, e3215 */
            float4((this.e42 * right_anti_dual.e35) + (this.e23 * right_anti_dual.e45) + (this.e45 * right_anti_dual.e23) + (this.e35 * right_anti_dual.e42), (this.e43 * right_anti_dual.e15) + (this.e31 * right_anti_dual.e45) + (this.e45 * right_anti_dual.e31) + (this.e15 * right_anti_dual.e43), (this.e41 * right_anti_dual.e25) + (this.e12 * right_anti_dual.e45) + (this.e45 * right_anti_dual.e12) + (this.e25 * right_anti_dual.e41), -(this.e23 * right_anti_dual.e15) - (this.e31 * right_anti_dual.e25) - (this.e12 * right_anti_dual.e35) - (this.e35 * right_anti_dual.e12)) - float4((this.group0.xyz.zxy * right_anti_dual.group2.yzx), this.e15 * right_anti_dual.e23) - float4((this.group2.xyz.yzx * right_anti_dual.group0.zxy), this.e25 * right_anti_dual.e31)
        );
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (wedge.group3.yzxy * float4(other.group0.zxy, other.e2)) + float4(float3(0.0), (wedge.e4125 * other.e3) + (wedge.e3215 * other.e4) - (wedge.e42 * other.e315) - (wedge.e43 * other.e125) - (wedge.e23 * other.e415) - (wedge.e31 * other.e425) - (wedge.e12 * other.e435) - (wedge.e45 * other.e321) - (wedge.e15 * other.e423) - (wedge.e25 * other.e431) - (wedge.e35 * other.e412)) + float4((wedge.group0.xyz * other.group0.www), wedge.e1234 * other.e5) + float4((wedge.group2.www * other.group1.xyz), wedge.e4235 * other.e1) - float4((wedge.group3.zxy * other.group0.yzx), wedge.e41 * other.e235), 
            /* e23, e31, e12, e45 */
            float4((wedge.e1234 * other.e235) + (wedge.e3215 * other.e423), (wedge.e1234 * other.e315) + (wedge.e3215 * other.e431), (wedge.e1234 * other.e125) + (wedge.e3215 * other.e412), -(wedge.e4315 * other.e425) - (wedge.e4125 * other.e435)) + (float4(other.e12345) * wedge.group1) - (wedge.group3.xyzx * other.group1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(wedge.e3215) * other.group1.xyz) + (float3(other.e12345) * wedge.group2.xyz) + (wedge.group3.zxy * other.group2.yzx) - (wedge.group3.yzx * other.group2.zxy)), wedge.e1234 * other.e12345), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345) * wedge.group3
        );
    }
}
public extension Dipole {
    public VersorOdd project_orthogonally_onto(VersorOdd other) {
        VersorEven right_anti_dual = VersorEven(
            /* e423, e431, e412, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            float4(other.group2.xyz, other.e3215) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4(other.group3.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0)
        );
        CircleRotor wedge = CircleRotor(
            /* e423, e431, e412 */
            (float3(right_anti_dual.e4) * this.group1.xyz) + (this.group0.xyz.yzx * right_anti_dual.group3.zxy) - (this.group0.xyz.zxy * right_anti_dual.group3.yzx), 
            /* e415, e425, e435, e321 */
            float4((this.e41 * right_anti_dual.e5) + (this.e15 * right_anti_dual.e4), (this.e42 * right_anti_dual.e5) + (this.e25 * right_anti_dual.e4), (this.e43 * right_anti_dual.e5) + (this.e35 * right_anti_dual.e4), -(this.e31 * right_anti_dual.e2) - (this.e12 * right_anti_dual.e3)) - (this.group1.wwwx * right_anti_dual.group3.xyzx), 
            /* e235, e315, e125, e12345 */
            float4((this.e23 * right_anti_dual.e5) + (this.e35 * right_anti_dual.e2), (this.e31 * right_anti_dual.e5) + (this.e15 * right_anti_dual.e3), (this.e12 * right_anti_dual.e5) + (this.e25 * right_anti_dual.e1), -(this.e42 * right_anti_dual.e315) - (this.e43 * right_anti_dual.e125) - (this.e23 * right_anti_dual.e415) - (this.e31 * right_anti_dual.e425) - (this.e12 * right_anti_dual.e435) - (this.e45 * right_anti_dual.e321) - (this.e15 * right_anti_dual.e423) - (this.e25 * right_anti_dual.e431) - (this.e35 * right_anti_dual.e412)) - float4((this.group2.xyz.yzx * right_anti_dual.group3.zxy), this.e41 * right_anti_dual.e235)
        );
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4((wedge.e415 * other.e1234) + (wedge.e12345 * other.e41), (wedge.e425 * other.e1234) + (wedge.e12345 * other.e42), (wedge.e435 * other.e1234) + (wedge.e12345 * other.e43), -(wedge.e431 * other.e25) - (wedge.e412 * other.e35) - (wedge.e415 * other.e23) - (wedge.e425 * other.e31) - (wedge.e435 * other.e12) - (wedge.e321 * other.e45) - (wedge.e235 * other.e41) - (wedge.e315 * other.e42) - (wedge.e125 * other.e43)) + float4((wedge.group0.xyz.zxy * other.group3.yzx), wedge.e12345 * other.scalar) - float4((wedge.group0.xyz.yzx * other.group3.zxy), wedge.e423 * other.e15), 
            /* e23, e31, e12, e45 */
            float4((wedge.e235 * other.e1234) + (wedge.e12345 * other.e23), (wedge.e315 * other.e1234) + (wedge.e12345 * other.e31), (wedge.e125 * other.e1234) + (wedge.e12345 * other.e12), -(wedge.e425 * other.e4315) - (wedge.e435 * other.e4125)) + float4((wedge.group0.xyz * other.group3.www), wedge.e12345 * other.e45) - (wedge.group1.wwwx * other.group3.xyzx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(wedge.e12345) * other.group2.xyz) + (float3(other.e3215) * wedge.group1.xyz) + (wedge.group2.yzx * other.group3.zxy) - (wedge.group2.zxy * other.group3.yzx)), wedge.e12345 * other.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(wedge.e12345) * other.group3
        );
    }
}
public extension DipoleInversion {
    // Fancy infix trick (first half)
    public ProjectOrthogonallyOntoInfixPartial<DipoleInversion> operator /(ProjectOrthogonallyOntoInfix rhs) {
        return ProjectOrthogonallyOntoInfixPartial<DipoleInversion>(this);
    }
}
public extension ProjectOrthogonallyOntoInfixPartial<DipoleInversion> {
    // Fancy infix trick (second half)
    public AntiCircleRotor operator /(AntiCircleRotor rhs) {
        return this.a.project_orthogonally_onto(rhs);
    }
}
public extension DipoleInversion {
    public AntiCircleRotor project_orthogonally_onto(AntiCircleRotor other) {
        CircleRotor right_anti_dual = CircleRotor(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        AntiScalar wedge = AntiScalar(
            /* e12345 */
            -(right_anti_dual.e423 * this.e15) - (right_anti_dual.e431 * this.e25) - (right_anti_dual.e412 * this.e35) - (right_anti_dual.e415 * this.e23) - (right_anti_dual.e425 * this.e31) - (right_anti_dual.e435 * this.e12) - (right_anti_dual.e321 * this.e45) - (right_anti_dual.e235 * this.e41) - (right_anti_dual.e315 * this.e42) - (right_anti_dual.e125 * this.e43)
        );
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(wedge.e12345) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(wedge.e12345) * other.group1, 
            /* e15, e25, e35, scalar */
            float4(wedge.e12345) * other.group2
        );
    }
}
public extension DipoleInversion {
    public AntiCircleRotor project_orthogonally_onto(AntiDipoleInversion other) {
        DipoleInversion right_anti_dual = DipoleInversion(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e1234 */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group3 * float4(1.0, 1.0, 1.0, -1.0)
        );
        Sphere wedge = Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual.e42 * this.e35) + (right_anti_dual.e23 * this.e45) + (right_anti_dual.e45 * this.e23) + (right_anti_dual.e35 * this.e42), (right_anti_dual.e43 * this.e15) + (right_anti_dual.e31 * this.e45) + (right_anti_dual.e45 * this.e31) + (right_anti_dual.e15 * this.e43), (right_anti_dual.e41 * this.e25) + (right_anti_dual.e12 * this.e45) + (right_anti_dual.e45 * this.e12) + (right_anti_dual.e25 * this.e41), -(right_anti_dual.e12 * this.e35) - (right_anti_dual.e15 * this.e23) - (right_anti_dual.e25 * this.e31) - (right_anti_dual.e35 * this.e12)) - (this.group2.yzxx * float4(right_anti_dual.group0.xyz.zxy, right_anti_dual.e23)) - float4((this.group0.xyz.zxy * right_anti_dual.group2.yzx), right_anti_dual.e31 * this.e25), 
            /* e1234 */
            -(right_anti_dual.e41 * this.e23) - (right_anti_dual.e42 * this.e31) - (right_anti_dual.e43 * this.e12) - (right_anti_dual.e23 * this.e41) - (right_anti_dual.e31 * this.e42) - (right_anti_dual.e12 * this.e43)
        );
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (float3(wedge.e1234) * other.group1.xyz) + (other.group0.xyz.zxy * wedge.group0.yzx) - (other.group0.xyz.yzx * wedge.group0.zxy), 
            /* e23, e31, e12, e45 */
            float4((other.e423 * wedge.e3215) + (other.e235 * wedge.e1234), (other.e431 * wedge.e3215) + (other.e315 * wedge.e1234), (other.e412 * wedge.e3215) + (other.e125 * wedge.e1234), -(other.e425 * wedge.e4315) - (other.e435 * wedge.e4125)) - (other.group1.wwwx * wedge.group0.xyzx), 
            /* e15, e25, e35, scalar */
            float4(other.e125 * wedge.e4315 * -1.0, other.e235 * wedge.e4125 * -1.0, other.e315 * wedge.e4235 * -1.0, (other.e2 * wedge.e4315) + (other.e3 * wedge.e4125) + (other.e5 * wedge.e1234)) + (float4(wedge.e3215) * float4(other.group1.xyz, other.e4)) + (wedge.group0.zxyx * float4(other.group2.yzx, other.e1))
        );
    }
}
public extension DipoleInversion {
    public AntiMotor project_orthogonally_onto(AntiDualNum other) {
        DualNum right_anti_dual = DualNum(
            /* e5, e12345 */
            other.group0.xy
        );
        Motor wedge = Motor(
            /* e415, e425, e435, e12345 */
            float4(right_anti_dual.e5) * float4(this.group0.xyz, this.e1234), 
            /* e235, e315, e125, e5 */
            float4(right_anti_dual.group0.xy.xx, right_anti_dual.e5, 0.0) * float4(float3(1.0), 0.0) * float4(this.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(float3(0.0), other.scalar * wedge.e12345), 
            /* e15, e25, e35, e3215 */
            float4(other.e3215) * wedge.group0
        );
    }
}
public extension DipoleInversion {
    public AntiLine project_orthogonally_onto(AntiFlatPoint other) {
        FlatPoint right_anti_dual = FlatPoint(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        Plane wedge = Plane(
            /* e4235, e4315, e4125, e3215 */
            float4((this.e42 * right_anti_dual.e35) + (this.e23 * right_anti_dual.e45), (this.e43 * right_anti_dual.e15) + (this.e31 * right_anti_dual.e45), (this.e41 * right_anti_dual.e25) + (this.e12 * right_anti_dual.e45), -(this.e31 * right_anti_dual.e25) - (this.e12 * right_anti_dual.e35)) - (right_anti_dual.group0.yzxx * float4(this.group0.xyz.zxy, this.e23))
        );
        return AntiLine(
            /* e23, e31, e12 */
            float3(other.e321) * wedge.group0.xyz * float3(-1.0), 
            /* e15, e25, e35 */
            (other.group0.yzx * wedge.group0.zxy) - (other.group0.zxy * wedge.group0.yzx)
        );
    }
}
public extension DipoleInversion {
    public AntiMotor project_orthogonally_onto(AntiFlector other) {
        Flector right_anti_dual = Flector(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        Plane wedge = Plane(
            /* e4235, e4315, e4125, e3215 */
            float4((this.e42 * right_anti_dual.e35) + (this.e23 * right_anti_dual.e45), (this.e43 * right_anti_dual.e15) + (this.e31 * right_anti_dual.e45), (this.e41 * right_anti_dual.e25) + (this.e12 * right_anti_dual.e45), -(this.e31 * right_anti_dual.e25) - (this.e12 * right_anti_dual.e35)) - (right_anti_dual.group0.yzxx * float4(this.group0.xyz.zxy, this.e23))
        );
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((other.group0.www * wedge.group0.xyz * float3(-1.0)), (other.e1 * wedge.e4235) + (other.e2 * wedge.e4315) + (other.e3 * wedge.e4125)), 
            /* e15, e25, e35, e3215 */
            float4(((other.group0.yzx * wedge.group0.zxy) - (other.group0.zxy * wedge.group0.yzx)), 0.0)
        );
    }
}
public extension DipoleInversion {
    public AntiLine project_orthogonally_onto(AntiLine other) {
        Line right_anti_dual = Line(
            /* e415, e425, e435 */
            other.group0.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            other.group1.xyz * float3(-1.0)
        );
        AntiScalar wedge = AntiScalar(
            /* e12345 */
            -(this.e41 * right_anti_dual.e235) - (this.e42 * right_anti_dual.e315) - (this.e43 * right_anti_dual.e125) - (this.e23 * right_anti_dual.e415) - (this.e31 * right_anti_dual.e425) - (this.e12 * right_anti_dual.e435)
        );
        return AntiLine(
            /* e23, e31, e12 */
            float3(wedge.e12345) * other.group0.xyz, 
            /* e15, e25, e35 */
            float3(wedge.e12345) * other.group1.xyz
        );
    }
}
public extension DipoleInversion {
    public AntiMotor project_orthogonally_onto(AntiMotor other) {
        Motor right_anti_dual = Motor(
            /* e415, e425, e435, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        Motor wedge = Motor(
            /* e415, e425, e435, e12345 */
            float4((float3(right_anti_dual.e5) * this.group0.xyz), (this.e1234 * right_anti_dual.e5) - (this.e41 * right_anti_dual.e235) - (this.e42 * right_anti_dual.e315) - (this.e43 * right_anti_dual.e125) - (this.e23 * right_anti_dual.e415) - (this.e31 * right_anti_dual.e425) - (this.e12 * right_anti_dual.e435)), 
            /* e235, e315, e125, e5 */
            float4(float3(1.0), 0.0) * float4(right_anti_dual.group1.www, 0.0) * float4(this.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((float3(wedge.e12345) * other.group0.xyz), (other.scalar * wedge.e12345) - (other.e23 * wedge.e415) - (other.e31 * wedge.e425) - (other.e12 * wedge.e435)), 
            /* e15, e25, e35, e3215 */
            float4(((float3(other.e3215) * wedge.group0.xyz) + (float3(wedge.e12345) * other.group1.xyz)), other.e3215 * wedge.e12345)
        );
    }
}
public extension DipoleInversion {
    public DipoleInversion project_orthogonally_onto(AntiScalar other) {
        Scalar right_anti_dual = Scalar(
            /* scalar */
            other.e12345 * -1.0
        );
        DipoleInversion wedge = DipoleInversion(
            /* e41, e42, e43 */
            float3(right_anti_dual.scalar) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(right_anti_dual.scalar) * this.group1, 
            /* e15, e25, e35, e1234 */
            float4(right_anti_dual.scalar) * this.group2, 
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual.scalar) * this.group3
        );
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(other.e12345) * wedge.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(other.e12345) * wedge.group1, 
            /* e15, e25, e35, e1234 */
            float4(other.e12345) * wedge.group2, 
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345) * wedge.group3
        );
    }
}
public extension DipoleInversion {
    public Dipole project_orthogonally_onto(Circle other) {
        Dipole right_anti_dual = Dipole(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35 */
            other.group2.xyz
        );
        Sphere wedge = Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual.e42 * this.e35) + (right_anti_dual.e23 * this.e45) + (right_anti_dual.e45 * this.e23) + (right_anti_dual.e35 * this.e42), (right_anti_dual.e43 * this.e15) + (right_anti_dual.e31 * this.e45) + (right_anti_dual.e45 * this.e31) + (right_anti_dual.e15 * this.e43), (right_anti_dual.e41 * this.e25) + (right_anti_dual.e12 * this.e45) + (right_anti_dual.e45 * this.e12) + (right_anti_dual.e25 * this.e41), -(right_anti_dual.e23 * this.e15) - (right_anti_dual.e31 * this.e25) - (right_anti_dual.e12 * this.e35) - (right_anti_dual.e35 * this.e12)) - float4((right_anti_dual.group0.xyz.zxy * this.group2.yzx), right_anti_dual.e15 * this.e23) - float4((right_anti_dual.group2.xyz.yzx * this.group0.xyz.zxy), right_anti_dual.e25 * this.e31), 
            /* e1234 */
            -(right_anti_dual.e41 * this.e23) - (right_anti_dual.e42 * this.e31) - (right_anti_dual.e43 * this.e12) - (right_anti_dual.e23 * this.e41) - (right_anti_dual.e31 * this.e42) - (right_anti_dual.e12 * this.e43)
        );
        return Dipole(
            /* e41, e42, e43 */
            (float3(wedge.e1234) * other.group1.xyz) + (other.group0.xyz.zxy * wedge.group0.yzx) - (other.group0.xyz.yzx * wedge.group0.zxy), 
            /* e23, e31, e12, e45 */
            float4((other.e423 * wedge.e3215) + (other.e235 * wedge.e1234), (other.e431 * wedge.e3215) + (other.e315 * wedge.e1234), (other.e412 * wedge.e3215) + (other.e125 * wedge.e1234), -(other.e425 * wedge.e4315) - (other.e435 * wedge.e4125)) - (other.group1.wwwx * wedge.group0.xyzx), 
            /* e15, e25, e35 */
            (float3(wedge.e3215) * other.group1.xyz) + (other.group2.xyz.yzx * wedge.group0.zxy) - (other.group2.xyz.zxy * wedge.group0.yzx)
        );
    }
}
public extension DipoleInversion {
    public VersorOdd project_orthogonally_onto(CircleRotor other) {
        AntiCircleRotor right_anti_dual = AntiCircleRotor(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        DipoleInversion wedge = DipoleInversion(
            /* e41, e42, e43 */
            float3(right_anti_dual.scalar) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(right_anti_dual.scalar) * this.group1, 
            /* e15, e25, e35, e1234 */
            float4((right_anti_dual.group2.www * this.group2.xyz), (right_anti_dual.scalar * this.e1234) - (right_anti_dual.e41 * this.e23) - (right_anti_dual.e42 * this.e31) - (right_anti_dual.e43 * this.e12) - (right_anti_dual.e23 * this.e41) - (right_anti_dual.e31 * this.e42) - (right_anti_dual.e12 * this.e43)), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual.e23 * this.e45) + (right_anti_dual.e45 * this.e23) + (right_anti_dual.e35 * this.e42) + (right_anti_dual.scalar * this.e4235), (right_anti_dual.e31 * this.e45) + (right_anti_dual.e45 * this.e31) + (right_anti_dual.e15 * this.e43) + (right_anti_dual.scalar * this.e4315), (right_anti_dual.e12 * this.e45) + (right_anti_dual.e45 * this.e12) + (right_anti_dual.e25 * this.e41) + (right_anti_dual.scalar * this.e4125), -(right_anti_dual.e12 * this.e35) - (right_anti_dual.e15 * this.e23) - (right_anti_dual.e25 * this.e31) - (right_anti_dual.e35 * this.e12)) + float4((right_anti_dual.group0.xyz.yzx * this.group2.zxy), right_anti_dual.scalar * this.e3215) - (this.group2.yzxx * float4(right_anti_dual.group0.xyz.zxy, right_anti_dual.e23)) - float4((this.group0.xyz.zxy * right_anti_dual.group2.yzx), right_anti_dual.e31 * this.e25)
        );
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4((other.e412 * wedge.e4315) + (other.e415 * wedge.e1234) + (other.e12345 * wedge.e41), (other.e423 * wedge.e4125) + (other.e425 * wedge.e1234) + (other.e12345 * wedge.e42), (other.e431 * wedge.e4235) + (other.e435 * wedge.e1234) + (other.e12345 * wedge.e43), -(other.e431 * wedge.e25) - (other.e412 * wedge.e35) - (other.e415 * wedge.e23) - (other.e425 * wedge.e31) - (other.e435 * wedge.e12) - (other.e321 * wedge.e45) - (other.e235 * wedge.e41) - (other.e315 * wedge.e42) - (other.e125 * wedge.e43)) - float4((other.group0.xyz.yzx * wedge.group3.zxy), other.e423 * wedge.e15), 
            /* e23, e31, e12, e45 */
            float4((other.e235 * wedge.e1234) + (other.e12345 * wedge.e23), (other.e315 * wedge.e1234) + (other.e12345 * wedge.e31), (other.e125 * wedge.e1234) + (other.e12345 * wedge.e12), -(other.e425 * wedge.e4315) - (other.e435 * wedge.e4125)) + float4((other.group0.xyz * wedge.group3.www), other.e12345 * wedge.e45) - (other.group1.wwwx * wedge.group3.xyzx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(other.e12345) * wedge.group2.xyz) + (float3(wedge.e3215) * other.group1.xyz) + (other.group2.yzx * wedge.group3.zxy) - (other.group2.zxy * wedge.group3.yzx)), other.e12345 * wedge.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345) * wedge.group3
        );
    }
}
public extension DipoleInversion {
    public Dipole project_orthogonally_onto(Dipole other) {
        Circle right_anti_dual = Circle(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125 */
            other.group2.xyz * float3(-1.0)
        );
        AntiScalar wedge = AntiScalar(
            /* e12345 */
            -(right_anti_dual.e423 * this.e15) - (right_anti_dual.e431 * this.e25) - (right_anti_dual.e412 * this.e35) - (right_anti_dual.e415 * this.e23) - (right_anti_dual.e425 * this.e31) - (right_anti_dual.e435 * this.e12) - (right_anti_dual.e321 * this.e45) - (right_anti_dual.e235 * this.e41) - (right_anti_dual.e315 * this.e42) - (right_anti_dual.e125 * this.e43)
        );
        return Dipole(
            /* e41, e42, e43 */
            float3(wedge.e12345) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(wedge.e12345) * other.group1, 
            /* e15, e25, e35 */
            float3(wedge.e12345) * other.group2.xyz
        );
    }
}
public extension DipoleInversion {
    public VersorOdd project_orthogonally_onto(DipoleInversion other) {
        AntiDipoleInversion right_anti_dual = AntiDipoleInversion(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e4 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group3 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        CircleRotor wedge = CircleRotor(
            /* e423, e431, e412 */
            (float3(right_anti_dual.e4) * this.group1.xyz) + (this.group0.xyz.yzx * right_anti_dual.group3.zxy) - (this.group0.xyz.zxy * right_anti_dual.group3.yzx), 
            /* e415, e425, e435, e321 */
            float4((right_anti_dual.e4 * this.e15) + (right_anti_dual.e5 * this.e41), (right_anti_dual.e4 * this.e25) + (right_anti_dual.e5 * this.e42), (right_anti_dual.e4 * this.e35) + (right_anti_dual.e5 * this.e43), -(right_anti_dual.e2 * this.e31) - (right_anti_dual.e3 * this.e12)) - (right_anti_dual.group3.xyzx * this.group1.wwwx), 
            /* e235, e315, e125, e12345 */
            (right_anti_dual.group3.wwwx * float4(this.group1.xyz, this.e4235)) + float4(float3(0.0), (right_anti_dual.e2 * this.e4315) + (right_anti_dual.e3 * this.e4125) + (right_anti_dual.e5 * this.e1234) - (right_anti_dual.e431 * this.e25) - (right_anti_dual.e412 * this.e35) - (right_anti_dual.e415 * this.e23) - (right_anti_dual.e425 * this.e31) - (right_anti_dual.e435 * this.e12) - (right_anti_dual.e321 * this.e45) - (right_anti_dual.e235 * this.e41) - (right_anti_dual.e315 * this.e42) - (right_anti_dual.e125 * this.e43)) + float4((right_anti_dual.group3.yzx * this.group2.zxy), right_anti_dual.e4 * this.e3215) - (this.group2.yzxx * float4(right_anti_dual.group3.zxy, right_anti_dual.e423))
        );
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4((wedge.e412 * other.e4315) + (wedge.e415 * other.e1234) + (wedge.e12345 * other.e41), (wedge.e423 * other.e4125) + (wedge.e425 * other.e1234) + (wedge.e12345 * other.e42), (wedge.e431 * other.e4235) + (wedge.e435 * other.e1234) + (wedge.e12345 * other.e43), -(wedge.e431 * other.e25) - (wedge.e412 * other.e35) - (wedge.e415 * other.e23) - (wedge.e425 * other.e31) - (wedge.e435 * other.e12) - (wedge.e321 * other.e45) - (wedge.e235 * other.e41) - (wedge.e315 * other.e42) - (wedge.e125 * other.e43)) - float4((wedge.group0.xyz.yzx * other.group3.zxy), wedge.e423 * other.e15), 
            /* e23, e31, e12, e45 */
            float4((wedge.e235 * other.e1234) + (wedge.e12345 * other.e23), (wedge.e315 * other.e1234) + (wedge.e12345 * other.e31), (wedge.e125 * other.e1234) + (wedge.e12345 * other.e12), -(wedge.e425 * other.e4315) - (wedge.e435 * other.e4125)) + float4((wedge.group0.xyz * other.group3.www), wedge.e12345 * other.e45) - (wedge.group1.wwwx * other.group3.xyzx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(wedge.e12345) * other.group2.xyz) + (float3(other.e3215) * wedge.group1.xyz) + (wedge.group2.yzx * other.group3.zxy) - (wedge.group2.zxy * other.group3.yzx)), wedge.e12345 * other.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(wedge.e12345) * other.group3
        );
    }
}
public extension DipoleInversion {
    public VersorOdd project_orthogonally_onto(DualNum other) {
        AntiDualNum right_anti_dual = AntiDualNum(
            /* e3215, scalar */
            other.group0.xy * float2(-1.0)
        );
        DipoleInversion wedge = DipoleInversion(
            /* e41, e42, e43 */
            float3(right_anti_dual.scalar) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(right_anti_dual.scalar) * this.group1, 
            /* e15, e25, e35, e1234 */
            float4(right_anti_dual.scalar) * this.group2, 
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual.scalar) * this.group3
        );
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(other.group0.xy.yy, other.e12345, other.e5) * float4(wedge.group0.xyz, wedge.e1234), 
            /* e23, e31, e12, e45 */
            float4(other.e12345) * wedge.group1, 
            /* e15, e25, e35, e1234 */
            float4(other.e12345) * wedge.group2, 
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345) * wedge.group3
        );
    }
}
public extension DipoleInversion {
    public FlatPoint project_orthogonally_onto(FlatPoint other) {
        AntiFlatPoint right_anti_dual = AntiFlatPoint(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(-(right_anti_dual.e235 * this.e41) - (right_anti_dual.e315 * this.e42) - (right_anti_dual.e125 * this.e43) - (right_anti_dual.e321 * this.e45)) * other.group0
        );
    }
}
public extension DipoleInversion {
    public VersorOdd project_orthogonally_onto(Flector other) {
        AntiFlector right_anti_dual = AntiFlector(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        CircleRotor wedge = CircleRotor(
            /* e423, e431, e412 */
            (this.group0.xyz.yzx * right_anti_dual.group1.zxy) - (this.group0.xyz.zxy * right_anti_dual.group1.yzx), 
            /* e415, e425, e435, e321 */
            float4(right_anti_dual.e5 * this.e41, right_anti_dual.e5 * this.e42, right_anti_dual.e5 * this.e43, -(right_anti_dual.e2 * this.e31) - (right_anti_dual.e3 * this.e12)) - (right_anti_dual.group1.xyzx * this.group1.wwwx), 
            /* e235, e315, e125, e12345 */
            (right_anti_dual.group1.yzxx * float4(this.group2.zxy, this.e4235)) + (right_anti_dual.group1.wwwy * float4(this.group1.xyz, this.e4315)) + float4(float3(0.0), (right_anti_dual.e3 * this.e4125) + (right_anti_dual.e5 * this.e1234) - (right_anti_dual.e315 * this.e42) - (right_anti_dual.e125 * this.e43) - (right_anti_dual.e321 * this.e45)) - float4((right_anti_dual.group1.zxy * this.group2.yzx), right_anti_dual.e235 * this.e41)
        );
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(wedge.e412 * other.e4315, wedge.e423 * other.e4125, wedge.e431 * other.e4235, -(wedge.e431 * other.e25) - (wedge.e412 * other.e35) - (wedge.e321 * other.e45)) - float4((wedge.group0.xyz.yzx * other.group1.zxy), wedge.e423 * other.e15), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), -(wedge.e425 * other.e4315) - (wedge.e435 * other.e4125)) + float4((wedge.group0.xyz * other.group1.www), wedge.e12345 * other.e45) - (wedge.group1.wwwx * other.group1.xyzx), 
            /* e15, e25, e35, e1234 */
            float4((float3(wedge.e12345) * other.group0.xyz), 0.0) + float4((float3(other.e3215) * wedge.group1.xyz), 0.0) + float4((wedge.group2.yzx * other.group1.zxy), 0.0) - float4((wedge.group2.zxy * other.group1.yzx), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(wedge.e12345) * other.group1
        );
    }
}
public extension DipoleInversion {
    public Dipole project_orthogonally_onto(Line other) {
        AntiLine right_anti_dual = AntiLine(
            /* e23, e31, e12 */
            other.group0.xyz, 
            /* e15, e25, e35 */
            other.group1.xyz
        );
        Sphere wedge = Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual.e23 * this.e45) + (right_anti_dual.e35 * this.e42), (right_anti_dual.e31 * this.e45) + (right_anti_dual.e15 * this.e43), (right_anti_dual.e12 * this.e45) + (right_anti_dual.e25 * this.e41), -(right_anti_dual.e31 * this.e25) - (right_anti_dual.e12 * this.e35) - (right_anti_dual.e15 * this.e23) - (right_anti_dual.e25 * this.e31) - (right_anti_dual.e35 * this.e12)) - float4((right_anti_dual.group1.xyz.yzx * this.group0.xyz.zxy), right_anti_dual.e23 * this.e15), 
            /* e1234 */
            -(right_anti_dual.e23 * this.e41) - (right_anti_dual.e31 * this.e42) - (right_anti_dual.e12 * this.e43)
        );
        return Dipole(
            /* e41, e42, e43 */
            float3(wedge.e1234) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4((float3(wedge.e1234) * other.group1.xyz), -(other.e415 * wedge.e4235) - (other.e425 * wedge.e4315) - (other.e435 * wedge.e4125)), 
            /* e15, e25, e35 */
            (float3(wedge.e3215) * other.group0.xyz) + (other.group1.xyz.yzx * wedge.group0.zxy) - (other.group1.xyz.zxy * wedge.group0.yzx)
        );
    }
}
public extension DipoleInversion {
    public VersorOdd project_orthogonally_onto(Motor other) {
        AntiMotor right_anti_dual = AntiMotor(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        DipoleInversion wedge = DipoleInversion(
            /* e41, e42, e43 */
            float3(right_anti_dual.scalar) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(right_anti_dual.scalar) * this.group1, 
            /* e15, e25, e35, e1234 */
            float4((right_anti_dual.group0.www * this.group2.xyz), (right_anti_dual.scalar * this.e1234) - (right_anti_dual.e23 * this.e41) - (right_anti_dual.e31 * this.e42) - (right_anti_dual.e12 * this.e43)), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual.e23 * this.e45) + (right_anti_dual.scalar * this.e4235), (right_anti_dual.e31 * this.e45) + (right_anti_dual.scalar * this.e4315), (right_anti_dual.e12 * this.e45) + (right_anti_dual.scalar * this.e4125), -(right_anti_dual.e31 * this.e25) - (right_anti_dual.e12 * this.e35) - (right_anti_dual.e15 * this.e23) - (right_anti_dual.e25 * this.e31) - (right_anti_dual.e35 * this.e12)) + float4((this.group0.xyz.yzx * right_anti_dual.group1.zxy), right_anti_dual.scalar * this.e3215) - float4((this.group0.xyz.zxy * right_anti_dual.group1.yzx), right_anti_dual.e23 * this.e15)
        );
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(wedge.e1234 * other.e415, wedge.e1234 * other.e425, wedge.e1234 * other.e435, -(wedge.e41 * other.e235) - (wedge.e42 * other.e315) - (wedge.e43 * other.e125) - (wedge.e23 * other.e415) - (wedge.e31 * other.e425) - (wedge.e12 * other.e435)) + float4((wedge.group0.xyz * other.group0.www), wedge.e1234 * other.e5), 
            /* e23, e31, e12, e45 */
            float4(wedge.e1234 * other.e235, wedge.e1234 * other.e315, wedge.e1234 * other.e125, -(wedge.e4235 * other.e415) - (wedge.e4315 * other.e425) - (wedge.e4125 * other.e435)) + (float4(other.e12345) * wedge.group1), 
            /* e15, e25, e35, e1234 */
            float4(((float3(wedge.e3215) * other.group0.xyz) + (float3(other.e12345) * wedge.group2.xyz) + (wedge.group3.zxy * other.group1.yzx) - (wedge.group3.yzx * other.group1.zxy)), wedge.e1234 * other.e12345), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345) * wedge.group3
        );
    }
}
public extension DipoleInversion {
    public MultiVector project_orthogonally_onto(MultiVector other) {
        MultiVector right_anti_dual = MultiVector(
            /* scalar, e12345 */
            other.group0.xy.yx * float2(-1.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4(other.group9.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5 */
            other.e3215, 
            /* e15, e25, e35, e45 */
            float4(other.group8.xyz, other.e321 * -1.0), 
            /* e41, e42, e43 */
            other.group7.xyz, 
            /* e23, e31, e12 */
            other.group6.xyz, 
            /* e415, e425, e435, e321 */
            float4(other.group5.xyz, other.e45) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e423, e431, e412 */
            other.group4.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            other.group3.xyz * float3(-1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.group1.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234 */
            other.e4 * -1.0
        );
        MultiVector wedge = MultiVector(
            /* scalar, e12345 */
            float2(0.0, (this.e1234 * right_anti_dual.e5) + (this.e4235 * right_anti_dual.e1) + (this.e4315 * right_anti_dual.e2) + (this.e4125 * right_anti_dual.e3) + (this.e3215 * right_anti_dual.e4) - (this.e41 * right_anti_dual.e235) - (this.e42 * right_anti_dual.e315) - (this.e43 * right_anti_dual.e125) - (this.e23 * right_anti_dual.e415) - (this.e31 * right_anti_dual.e425) - (this.e12 * right_anti_dual.e435) - (this.e45 * right_anti_dual.e321) - (this.e15 * right_anti_dual.e423) - (this.e25 * right_anti_dual.e431) - (this.e35 * right_anti_dual.e412)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            float4(right_anti_dual.scalar) * float4(this.group2.xyz, this.e45), 
            /* e41, e42, e43 */
            float3(right_anti_dual.scalar) * this.group0.xyz, 
            /* e23, e31, e12 */
            float3(right_anti_dual.scalar) * this.group1.xyz, 
            /* e415, e425, e435, e321 */
            float4((this.e41 * right_anti_dual.e5) + (this.e15 * right_anti_dual.e4), (this.e42 * right_anti_dual.e5) + (this.e25 * right_anti_dual.e4), (this.e43 * right_anti_dual.e5) + (this.e35 * right_anti_dual.e4), -(this.e31 * right_anti_dual.e2) - (this.e12 * right_anti_dual.e3)) - (this.group1.wwwx * right_anti_dual.group1.xyzx), 
            /* e423, e431, e412 */
            (float3(right_anti_dual.e4) * this.group1.xyz) + (this.group0.xyz.yzx * right_anti_dual.group1.zxy) - (this.group0.xyz.zxy * right_anti_dual.group1.yzx), 
            /* e235, e315, e125 */
            (float3(right_anti_dual.e5) * this.group1.xyz) + (this.group2.zxy * right_anti_dual.group1.yzx) - (this.group2.yzx * right_anti_dual.group1.zxy), 
            /* e4235, e4315, e4125, e3215 */
            float4((this.e42 * right_anti_dual.e35) + (this.e23 * right_anti_dual.e45) + (this.e45 * right_anti_dual.e23) + (this.e35 * right_anti_dual.e42), (this.e43 * right_anti_dual.e15) + (this.e31 * right_anti_dual.e45) + (this.e45 * right_anti_dual.e31) + (this.e15 * right_anti_dual.e43), (this.e41 * right_anti_dual.e25) + (this.e12 * right_anti_dual.e45) + (this.e45 * right_anti_dual.e12) + (this.e25 * right_anti_dual.e41), -(this.e23 * right_anti_dual.e15) - (this.e31 * right_anti_dual.e25) - (this.e12 * right_anti_dual.e35) - (this.e35 * right_anti_dual.e12)) + (float4(right_anti_dual.scalar) * this.group3) - (this.group2.yzxy * float4(right_anti_dual.group4.xyz.zxy, right_anti_dual.e31)) - float4((this.group0.xyz.zxy * right_anti_dual.group3.yzx), this.e15 * right_anti_dual.e23), 
            /* e1234 */
            (this.e1234 * right_anti_dual.scalar) - (this.e41 * right_anti_dual.e23) - (this.e42 * right_anti_dual.e31) - (this.e43 * right_anti_dual.e12) - (this.e23 * right_anti_dual.e41) - (this.e31 * right_anti_dual.e42) - (this.e12 * right_anti_dual.e43)
        );
        return MultiVector(
            /* scalar, e12345 */
            float2((other.scalar * wedge.e12345) + (other.e12345 * wedge.scalar) + (other.e1 * wedge.e4235) + (other.e2 * wedge.e4315) + (other.e3 * wedge.e4125) + (other.e4 * wedge.e3215) + (other.e5 * wedge.e1234) + (other.e4235 * wedge.e1) + (other.e4315 * wedge.e2) + (other.e4125 * wedge.e3) + (other.e3215 * wedge.e4) + (other.e1234 * wedge.e5) - (other.e15 * wedge.e423) - (other.e25 * wedge.e431) - (other.e35 * wedge.e412) - (other.e45 * wedge.e321) - (other.e41 * wedge.e235) - (other.e42 * wedge.e315) - (other.e43 * wedge.e125) - (other.e23 * wedge.e415) - (other.e31 * wedge.e425) - (other.e12 * wedge.e435) - (other.e415 * wedge.e23) - (other.e425 * wedge.e31) - (other.e435 * wedge.e12) - (other.e321 * wedge.e45) - (other.e423 * wedge.e15) - (other.e431 * wedge.e25) - (other.e412 * wedge.e35) - (other.e235 * wedge.e41) - (other.e315 * wedge.e42) - (other.e125 * wedge.e43), other.e12345 * wedge.e12345), 
            /* e1, e2, e3, e4 */
            float4((other.e415 * wedge.e321) + (other.e321 * wedge.e415) + (other.e4315 * wedge.e12) + (other.e1234 * wedge.e15), (other.e425 * wedge.e321) + (other.e321 * wedge.e425) + (other.e4125 * wedge.e23) + (other.e1234 * wedge.e25), (other.e435 * wedge.e321) + (other.e321 * wedge.e435) + (other.e4235 * wedge.e31) + (other.e1234 * wedge.e35), -(other.e45 * wedge.e1234) - (other.e415 * wedge.e423) - (other.e425 * wedge.e431) - (other.e435 * wedge.e412)) + (float4(other.e12345) * wedge.group1) + (float4(wedge.e12345) * other.group1) + float4((other.group4.xyz * wedge.group9.www), other.e4235 * wedge.e41) + float4((other.group5.xyz.yzx * wedge.group9.zxy), other.e4315 * wedge.e42) + float4((other.group7.xyz.zxy * wedge.group8.xyz.yzx), other.e4125 * wedge.e43) + float4((other.group8.xyz.yzx * wedge.group7.xyz.zxy), other.e1234 * wedge.e45) - (wedge.group9.yzxx * float4(other.group5.xyz.zxy, other.e41)) - float4((float3(wedge.e1234) * other.group3.xyz), other.e412 * wedge.e435) - float4((wedge.group4.xyz * other.group9.www), other.e423 * wedge.e415) - float4((other.group7.xyz.yzx * wedge.group8.xyz.zxy), other.e42 * wedge.e4315) - float4((other.group8.xyz.zxy * wedge.group7.xyz.yzx), other.e43 * wedge.e4125) - float4((wedge.group5.xyz.yzx * other.group9.zxy), other.e431 * wedge.e425), 
            /* e5 */
            (other.e12345 * wedge.e5) + (other.e5 * wedge.e12345) + (other.e15 * wedge.e4235) + (other.e25 * wedge.e4315) + (other.e35 * wedge.e4125) + (other.e45 * wedge.e3215) - (other.e415 * wedge.e235) - (other.e425 * wedge.e315) - (other.e435 * wedge.e125) - (other.e235 * wedge.e415) - (other.e315 * wedge.e425) - (other.e125 * wedge.e435) - (other.e4235 * wedge.e15) - (other.e4315 * wedge.e25) - (other.e4125 * wedge.e35) - (other.e3215 * wedge.e45), 
            /* e15, e25, e35, e45 */
            float4((other.e415 * wedge.e3215) + (other.e315 * wedge.e4125) + (other.e4125 * wedge.e315) + (other.e3215 * wedge.e415), (other.e425 * wedge.e3215) + (other.e125 * wedge.e4235) + (other.e4235 * wedge.e125) + (other.e3215 * wedge.e425), (other.e435 * wedge.e3215) + (other.e235 * wedge.e4315) + (other.e4315 * wedge.e235) + (other.e3215 * wedge.e435), -(other.e435 * wedge.e4125) - (other.e4235 * wedge.e415) - (other.e4315 * wedge.e425) - (other.e4125 * wedge.e435)) + (float4(other.e12345) * wedge.group3) + (float4(wedge.e12345) * other.group3) - (wedge.group9.yzxx * float4(other.group8.xyz.zxy, other.e415)) - float4((wedge.group8.xyz.zxy * other.group9.yzx), other.e425 * wedge.e4315), 
            /* e41, e42, e43 */
            (float3(other.e12345) * wedge.group4.xyz) + (float3(other.e1234) * wedge.group6.xyz) + (float3(wedge.e12345) * other.group4.xyz) + (float3(wedge.e1234) * other.group6.xyz) + (other.group7.xyz.zxy * wedge.group9.yzx) + (wedge.group7.xyz.zxy * other.group9.yzx) - (other.group7.xyz.yzx * wedge.group9.zxy) - (wedge.group7.xyz.yzx * other.group9.zxy), 
            /* e23, e31, e12 */
            (float3(other.e12345) * wedge.group5.xyz) + (float3(other.e3215) * wedge.group7.xyz) + (float3(other.e1234) * wedge.group8.xyz) + (float3(wedge.e12345) * other.group5.xyz) + (float3(wedge.e3215) * other.group7.xyz) + (float3(wedge.e1234) * other.group8.xyz) - (float3(other.e321) * wedge.group9.xyz) - (float3(wedge.e321) * other.group9.xyz), 
            /* e415, e425, e435, e321 */
            (float4(other.e12345) * wedge.group6) + (float4(wedge.e12345) * other.group6) + float4((other.group9.zxy * wedge.group9.yzx), other.e1234 * wedge.e3215) - (other.group9.yzxw * float4(wedge.group9.zxy, wedge.e1234)), 
            /* e423, e431, e412 */
            (float3(other.e12345) * wedge.group7.xyz) + (float3(other.e1234) * wedge.group9.xyz) + (float3(wedge.e12345) * other.group7.xyz) - (float3(wedge.e1234) * other.group9.xyz), 
            /* e235, e315, e125 */
            (float3(other.e12345) * wedge.group8.xyz) + (float3(wedge.e12345) * other.group8.xyz) + (float3(wedge.e3215) * other.group9.xyz) - (float3(other.e3215) * wedge.group9.xyz), 
            /* e4235, e4315, e4125, e3215 */
            (float4(other.e12345) * wedge.group9) + (float4(wedge.e12345) * other.group9), 
            /* e1234 */
            (other.e12345 * wedge.e1234) + (other.e1234 * wedge.e12345)
        );
    }
}
public extension DipoleInversion {
    public DipoleInversion project_orthogonally_onto(Plane other) {
        AntiPlane right_anti_dual = AntiPlane(
            /* e1, e2, e3, e5 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        CircleRotor wedge = CircleRotor(
            /* e423, e431, e412 */
            (this.group0.xyz.yzx * right_anti_dual.group0.zxy) - (this.group0.xyz.zxy * right_anti_dual.group0.yzx), 
            /* e415, e425, e435, e321 */
            float4(right_anti_dual.e5 * this.e41, right_anti_dual.e5 * this.e42, right_anti_dual.e5 * this.e43, -(right_anti_dual.e2 * this.e31) - (right_anti_dual.e3 * this.e12)) - (right_anti_dual.group0.xyzx * this.group1.wwwx), 
            /* e235, e315, e125, e12345 */
            float4(right_anti_dual.e3 * this.e25 * -1.0, right_anti_dual.e1 * this.e35 * -1.0, right_anti_dual.e2 * this.e15 * -1.0, (right_anti_dual.e3 * this.e4125) + (right_anti_dual.e5 * this.e1234)) + (right_anti_dual.group0.yzxx * float4(this.group2.zxy, this.e4235)) + (right_anti_dual.group0.wwwy * float4(this.group1.xyz, this.e4315))
        );
        return DipoleInversion(
            /* e41, e42, e43 */
            (wedge.group0.xyz.zxy * other.group0.yzx) - (wedge.group0.xyz.yzx * other.group0.zxy), 
            /* e23, e31, e12, e45 */
            float4(wedge.e423 * other.e3215, wedge.e431 * other.e3215, wedge.e412 * other.e3215, -(wedge.e425 * other.e4315) - (wedge.e435 * other.e4125)) - (wedge.group1.wwwx * other.group0.xyzx), 
            /* e15, e25, e35, e1234 */
            float4((float3(other.e3215) * wedge.group1.xyz), 0.0) + float4((wedge.group2.yzx * other.group0.zxy), 0.0) - float4((wedge.group2.zxy * other.group0.yzx), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(wedge.e12345) * other.group0
        );
    }
}
public extension DipoleInversion {
    public DipoleInversion project_orthogonally_onto(Sphere other) {
        RoundPoint right_anti_dual = RoundPoint(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5 */
            other.e3215
        );
        CircleRotor wedge = CircleRotor(
            /* e423, e431, e412 */
            (float3(right_anti_dual.e4) * this.group1.xyz) + (this.group0.xyz.yzx * right_anti_dual.group0.zxy) - (this.group0.xyz.zxy * right_anti_dual.group0.yzx), 
            /* e415, e425, e435, e321 */
            float4((this.e41 * right_anti_dual.e5) + (this.e15 * right_anti_dual.e4), (this.e42 * right_anti_dual.e5) + (this.e25 * right_anti_dual.e4), (this.e43 * right_anti_dual.e5) + (this.e35 * right_anti_dual.e4), -(this.e31 * right_anti_dual.e2) - (this.e12 * right_anti_dual.e3)) - (this.group1.wwwx * right_anti_dual.group0.xyzx), 
            /* e235, e315, e125, e12345 */
            float4(this.e25 * right_anti_dual.e3 * -1.0, this.e35 * right_anti_dual.e1 * -1.0, this.e15 * right_anti_dual.e2 * -1.0, (this.e4315 * right_anti_dual.e2) + (this.e4125 * right_anti_dual.e3) + (this.e3215 * right_anti_dual.e4)) + (float4(right_anti_dual.e5) * float4(this.group1.xyz, this.e1234)) + (right_anti_dual.group0.yzxx * float4(this.group2.zxy, this.e4235))
        );
        return DipoleInversion(
            /* e41, e42, e43 */
            (float3(other.e1234) * wedge.group1.xyz) + (wedge.group0.xyz.zxy * other.group0.yzx) - (wedge.group0.xyz.yzx * other.group0.zxy), 
            /* e23, e31, e12, e45 */
            float4((wedge.e423 * other.e3215) + (wedge.e235 * other.e1234), (wedge.e431 * other.e3215) + (wedge.e315 * other.e1234), (wedge.e412 * other.e3215) + (wedge.e125 * other.e1234), -(wedge.e425 * other.e4315) - (wedge.e435 * other.e4125)) - (wedge.group1.wwwx * other.group0.xyzx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(other.e3215) * wedge.group1.xyz) + (wedge.group2.yzx * other.group0.zxy) - (wedge.group2.zxy * other.group0.yzx)), wedge.e12345 * other.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(wedge.e12345) * other.group0
        );
    }
}
public extension DipoleInversion {
    public VersorOdd project_orthogonally_onto(VersorEven other) {
        VersorOdd right_anti_dual = VersorOdd(
            /* e41, e42, e43, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e1234 */
            float4(other.group2.xyz, other.e4) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.group3.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0)
        );
        DipoleInversion wedge = DipoleInversion(
            /* e41, e42, e43 */
            float3(right_anti_dual.scalar) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(right_anti_dual.scalar) * this.group1, 
            /* e15, e25, e35, e1234 */
            float4((float3(right_anti_dual.scalar) * this.group2.xyz), (this.e1234 * right_anti_dual.scalar) - (this.e41 * right_anti_dual.e23) - (this.e42 * right_anti_dual.e31) - (this.e43 * right_anti_dual.e12) - (this.e23 * right_anti_dual.e41) - (this.e31 * right_anti_dual.e42) - (this.e12 * right_anti_dual.e43)), 
            /* e4235, e4315, e4125, e3215 */
            float4((this.e23 * right_anti_dual.e45) + (this.e45 * right_anti_dual.e23) + (this.e35 * right_anti_dual.e42) + (this.e4235 * right_anti_dual.scalar), (this.e31 * right_anti_dual.e45) + (this.e45 * right_anti_dual.e31) + (this.e15 * right_anti_dual.e43) + (this.e4315 * right_anti_dual.scalar), (this.e12 * right_anti_dual.e45) + (this.e45 * right_anti_dual.e12) + (this.e25 * right_anti_dual.e41) + (this.e4125 * right_anti_dual.scalar), -(this.e12 * right_anti_dual.e35) - (this.e15 * right_anti_dual.e23) - (this.e25 * right_anti_dual.e31) - (this.e35 * right_anti_dual.e12)) + float4((this.group0.xyz.yzx * right_anti_dual.group2.zxy), this.e3215 * right_anti_dual.scalar) - (right_anti_dual.group2.yzxx * float4(this.group0.xyz.zxy, this.e23)) - float4((this.group2.yzx * right_anti_dual.group0.zxy), this.e31 * right_anti_dual.e25)
        );
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (wedge.group3.yzxy * float4(other.group0.zxy, other.e2)) + float4(float3(0.0), (wedge.e4125 * other.e3) + (wedge.e3215 * other.e4) - (wedge.e42 * other.e315) - (wedge.e43 * other.e125) - (wedge.e23 * other.e415) - (wedge.e31 * other.e425) - (wedge.e12 * other.e435) - (wedge.e45 * other.e321) - (wedge.e15 * other.e423) - (wedge.e25 * other.e431) - (wedge.e35 * other.e412)) + float4((wedge.group0.xyz * other.group0.www), wedge.e1234 * other.e5) + float4((wedge.group2.www * other.group1.xyz), wedge.e4235 * other.e1) - float4((wedge.group3.zxy * other.group0.yzx), wedge.e41 * other.e235), 
            /* e23, e31, e12, e45 */
            float4((wedge.e1234 * other.e235) + (wedge.e3215 * other.e423), (wedge.e1234 * other.e315) + (wedge.e3215 * other.e431), (wedge.e1234 * other.e125) + (wedge.e3215 * other.e412), -(wedge.e4315 * other.e425) - (wedge.e4125 * other.e435)) + (float4(other.e12345) * wedge.group1) - (wedge.group3.xyzx * other.group1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(wedge.e3215) * other.group1.xyz) + (float3(other.e12345) * wedge.group2.xyz) + (wedge.group3.zxy * other.group2.yzx) - (wedge.group3.yzx * other.group2.zxy)), wedge.e1234 * other.e12345), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345) * wedge.group3
        );
    }
}
public extension DipoleInversion {
    public VersorOdd project_orthogonally_onto(VersorOdd other) {
        VersorEven right_anti_dual = VersorEven(
            /* e423, e431, e412, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            float4(other.group2.xyz, other.e3215) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4(other.group3.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0)
        );
        CircleRotor wedge = CircleRotor(
            /* e423, e431, e412 */
            (float3(right_anti_dual.e4) * this.group1.xyz) + (this.group0.xyz.yzx * right_anti_dual.group3.zxy) - (this.group0.xyz.zxy * right_anti_dual.group3.yzx), 
            /* e415, e425, e435, e321 */
            float4((this.e41 * right_anti_dual.e5) + (this.e15 * right_anti_dual.e4), (this.e42 * right_anti_dual.e5) + (this.e25 * right_anti_dual.e4), (this.e43 * right_anti_dual.e5) + (this.e35 * right_anti_dual.e4), -(this.e31 * right_anti_dual.e2) - (this.e12 * right_anti_dual.e3)) - (this.group1.wwwx * right_anti_dual.group3.xyzx), 
            /* e235, e315, e125, e12345 */
            (float4(right_anti_dual.e5) * float4(this.group1.xyz, this.e1234)) + (right_anti_dual.group3.yzxx * float4(this.group2.zxy, this.e4235)) + float4(float3(0.0), (this.e4315 * right_anti_dual.e2) + (this.e4125 * right_anti_dual.e3) + (this.e3215 * right_anti_dual.e4) - (this.e42 * right_anti_dual.e315) - (this.e43 * right_anti_dual.e125) - (this.e23 * right_anti_dual.e415) - (this.e31 * right_anti_dual.e425) - (this.e12 * right_anti_dual.e435) - (this.e45 * right_anti_dual.e321) - (this.e15 * right_anti_dual.e423) - (this.e25 * right_anti_dual.e431) - (this.e35 * right_anti_dual.e412)) - float4((this.group2.yzx * right_anti_dual.group3.zxy), this.e41 * right_anti_dual.e235)
        );
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4((wedge.e415 * other.e1234) + (wedge.e12345 * other.e41), (wedge.e425 * other.e1234) + (wedge.e12345 * other.e42), (wedge.e435 * other.e1234) + (wedge.e12345 * other.e43), -(wedge.e431 * other.e25) - (wedge.e412 * other.e35) - (wedge.e415 * other.e23) - (wedge.e425 * other.e31) - (wedge.e435 * other.e12) - (wedge.e321 * other.e45) - (wedge.e235 * other.e41) - (wedge.e315 * other.e42) - (wedge.e125 * other.e43)) + float4((wedge.group0.xyz.zxy * other.group3.yzx), wedge.e12345 * other.scalar) - float4((wedge.group0.xyz.yzx * other.group3.zxy), wedge.e423 * other.e15), 
            /* e23, e31, e12, e45 */
            float4((wedge.e235 * other.e1234) + (wedge.e12345 * other.e23), (wedge.e315 * other.e1234) + (wedge.e12345 * other.e31), (wedge.e125 * other.e1234) + (wedge.e12345 * other.e12), -(wedge.e425 * other.e4315) - (wedge.e435 * other.e4125)) + float4((wedge.group0.xyz * other.group3.www), wedge.e12345 * other.e45) - (wedge.group1.wwwx * other.group3.xyzx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(wedge.e12345) * other.group2.xyz) + (float3(other.e3215) * wedge.group1.xyz) + (wedge.group2.yzx * other.group3.zxy) - (wedge.group2.zxy * other.group3.yzx)), wedge.e12345 * other.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(wedge.e12345) * other.group3
        );
    }
}
public extension DualNum {
    // Fancy infix trick (first half)
    public ProjectOrthogonallyOntoInfixPartial<DualNum> operator /(ProjectOrthogonallyOntoInfix rhs) {
        return ProjectOrthogonallyOntoInfixPartial<DualNum>(this);
    }
}
public extension ProjectOrthogonallyOntoInfixPartial<DualNum> {
    // Fancy infix trick (second half)
    public RoundPoint operator /(AntiCircleRotor rhs) {
        return this.a.project_orthogonally_onto(rhs);
    }
}
public extension DualNum {
    public RoundPoint project_orthogonally_onto(AntiCircleRotor other) {
        CircleRotor right_anti_dual = CircleRotor(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        Plane wedge = Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(this.e5) * float4(right_anti_dual.group0.xyz, right_anti_dual.e321) * float4(-1.0)
        );
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4((other.e41 * wedge.e3215) + (other.e31 * wedge.e4125), (other.e42 * wedge.e3215) + (other.e12 * wedge.e4235), (other.e43 * wedge.e3215) + (other.e23 * wedge.e4315), -(other.e42 * wedge.e4315) - (other.e43 * wedge.e4125)) - (wedge.group0.yzxx * float4(other.group1.zxy, other.e41)), 
            /* e5 */
            (other.e45 * wedge.e3215) + (other.e15 * wedge.e4235) + (other.e25 * wedge.e4315) + (other.e35 * wedge.e4125)
        );
    }
}
public extension DualNum {
    public AntiDipoleInversion project_orthogonally_onto(AntiDipoleInversion other) {
        DipoleInversion right_anti_dual = DipoleInversion(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e1234 */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group3 * float4(1.0, 1.0, 1.0, -1.0)
        );
        Motor wedge = Motor(
            /* e415, e425, e435, e12345 */
            float4(this.e5) * float4(right_anti_dual.group0.xyz, right_anti_dual.e1234), 
            /* e235, e315, e125, e5 */
            float4(this.group0.xy.xx, this.e5, 0.0) * float4(float3(1.0), 0.0) * float4(right_anti_dual.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(wedge.e12345) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(wedge.e12345) * other.group1, 
            /* e235, e315, e125, e4 */
            float4((float3(wedge.e12345) * other.group2.xyz), (other.e4 * wedge.e12345) - (other.e423 * wedge.e415) - (other.e431 * wedge.e425) - (other.e412 * wedge.e435)), 
            /* e1, e2, e3, e5 */
            float4((other.e321 * wedge.e415) + (other.e1 * wedge.e12345), (other.e321 * wedge.e425) + (other.e2 * wedge.e12345), (other.e321 * wedge.e435) + (other.e3 * wedge.e12345), -(other.e425 * wedge.e315) - (other.e435 * wedge.e125) - (other.e235 * wedge.e415) - (other.e315 * wedge.e425) - (other.e125 * wedge.e435)) + float4((other.group0.xyz.zxy * wedge.group1.yzx), other.e5 * wedge.e12345) - (wedge.group1.zxyx * float4(other.group0.xyz.yzx, other.e415))
        );
    }
}
public extension DualNum {
    public DualNum project_orthogonally_onto(AntiScalar other) {
        Scalar right_anti_dual = Scalar(
            /* scalar */
            other.e12345 * -1.0
        );
        return DualNum(
            /* e5, e12345 */
            float2(other.e12345) * float2(this.e5 * right_anti_dual.scalar, this.e12345 * right_anti_dual.scalar)
        );
    }
}
public extension DualNum {
    public RoundPoint project_orthogonally_onto(Circle other) {
        Dipole right_anti_dual = Dipole(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35 */
            other.group2.xyz
        );
        Line wedge = Line(
            /* e415, e425, e435 */
            float3(this.e5) * right_anti_dual.group0.xyz, 
            /* e235, e315, e125 */
            float3(this.e5) * right_anti_dual.group1.xyz
        );
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4((other.e412 * wedge.e315) + (other.e321 * wedge.e415), (other.e423 * wedge.e125) + (other.e321 * wedge.e425), (other.e431 * wedge.e235) + (other.e321 * wedge.e435), -(other.e431 * wedge.e425) - (other.e412 * wedge.e435)) - float4((other.group0.xyz.yzx * wedge.group1.xyz.zxy), other.e423 * wedge.e415), 
            /* e5 */
            -(other.e415 * wedge.e235) - (other.e425 * wedge.e315) - (other.e435 * wedge.e125) - (other.e235 * wedge.e415) - (other.e315 * wedge.e425) - (other.e125 * wedge.e435)
        );
    }
}
public extension DualNum {
    public VersorEven project_orthogonally_onto(CircleRotor other) {
        AntiCircleRotor right_anti_dual = AntiCircleRotor(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        Motor wedge = Motor(
            /* e415, e425, e435, e12345 */
            float4(this.group0.xy.xx, this.e5, this.e12345) * float4(right_anti_dual.group0.xyz, right_anti_dual.scalar), 
            /* e235, e315, e125, e5 */
            float4(this.e5) * float4(right_anti_dual.group1.xyz, right_anti_dual.scalar)
        );
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(wedge.e12345) * float4(other.group0.xyz, other.e12345), 
            /* e415, e425, e435, e321 */
            float4(((float3(other.e12345) * wedge.group0.xyz) + (float3(wedge.e12345) * other.group1.xyz)), other.e321 * wedge.e12345), 
            /* e235, e315, e125, e5 */
            float4(other.e12345 * wedge.e235, other.e12345 * wedge.e315, other.e12345 * wedge.e125, -(other.e415 * wedge.e235) - (other.e425 * wedge.e315) - (other.e435 * wedge.e125) - (other.e235 * wedge.e415) - (other.e315 * wedge.e425) - (other.e125 * wedge.e435)) + (other.group2 * float4(wedge.group0.www, wedge.e5)), 
            /* e1, e2, e3, e4 */
            float4((other.e412 * wedge.e315) + (other.e321 * wedge.e415), (other.e423 * wedge.e125) + (other.e321 * wedge.e425), (other.e431 * wedge.e235) + (other.e321 * wedge.e435), -(other.e431 * wedge.e425) - (other.e412 * wedge.e435)) - float4((other.group0.xyz.yzx * wedge.group1.zxy), other.e423 * wedge.e415)
        );
    }
}
public extension DualNum {
    public RoundPoint project_orthogonally_onto(Dipole other) {
        Circle right_anti_dual = Circle(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125 */
            other.group2.xyz * float3(-1.0)
        );
        Plane wedge = Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(this.e5) * float4(right_anti_dual.group0.xyz, right_anti_dual.e321) * float4(-1.0)
        );
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4((other.e41 * wedge.e3215) + (other.e31 * wedge.e4125), (other.e42 * wedge.e3215) + (other.e12 * wedge.e4235), (other.e43 * wedge.e3215) + (other.e23 * wedge.e4315), -(other.e42 * wedge.e4315) - (other.e43 * wedge.e4125)) - (wedge.group0.yzxx * float4(other.group1.zxy, other.e41)), 
            /* e5 */
            (other.e45 * wedge.e3215) + (other.e15 * wedge.e4235) + (other.e25 * wedge.e4315) + (other.e35 * wedge.e4125)
        );
    }
}
public extension DualNum {
    public AntiDipoleInversion project_orthogonally_onto(DipoleInversion other) {
        AntiDipoleInversion right_anti_dual = AntiDipoleInversion(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e4 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group3 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        Flector wedge = Flector(
            /* e15, e25, e35, e45 */
            float4(this.e5) * float4(right_anti_dual.group3.xyz, right_anti_dual.e4) * float4(-1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e5) * float4(right_anti_dual.group0.xyz, right_anti_dual.e321) * float4(-1.0)
        );
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(other.e1234) * wedge.group1.xyz, 
            /* e415, e425, e435, e321 */
            float4(((other.group3.zxy * wedge.group1.yzx) - (other.group3.yzx * wedge.group1.zxy)), other.e1234 * wedge.e3215), 
            /* e235, e315, e125, e4 */
            float4(float3(0.0), -(other.e42 * wedge.e4315) - (other.e43 * wedge.e4125)) + float4((wedge.group1.www * other.group3.xyz), other.e1234 * wedge.e45) - (wedge.group1.xyzx * float4(other.group3.www, other.e41)), 
            /* e1, e2, e3, e5 */
            (float4(wedge.e3215) * float4(other.group0.xyz, other.e45)) + (other.group2.wwwy * float4(wedge.group0.xyz, wedge.e4315)) + (wedge.group1.zxyx * float4(other.group1.yzx, other.e15)) + float4(float3(0.0), (other.e35 * wedge.e4125) - (other.e4315 * wedge.e25) - (other.e4125 * wedge.e35) - (other.e3215 * wedge.e45)) - float4((other.group1.zxy * wedge.group1.yzx), other.e4235 * wedge.e15)
        );
    }
}
public extension DualNum {
    public DualNum project_orthogonally_onto(DualNum other) {
        DualNum wedge = DualNum(
            /* e5, e12345 */
            float2(other.e12345 * -1.0) * this.group0.xy
        );
        return DualNum(
            /* e5, e12345 */
            float2((other.e5 * wedge.e12345) + (other.e12345 * wedge.e5), other.e12345 * wedge.e12345)
        );
    }
}
public extension DualNum {
    public DualNum project_orthogonally_onto(FlatPoint other) {
        return DualNum(
            /* e5, e12345 */
            float2(pow(other.e45, 2) * this.e5, 1.0) * float2(-1.0, 0.0)
        );
    }
}
public extension DualNum {
    public Motor project_orthogonally_onto(Flector other) {
        AntiFlector right_anti_dual = AntiFlector(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        Flector wedge = Flector(
            /* e15, e25, e35, e45 */
            float4(this.group0.xy.xx, this.e5, 0.0) * float4(float3(1.0), 0.0) * float4(right_anti_dual.group1.xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), right_anti_dual.e321 * this.e5 * -1.0)
        );
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(((other.group1.zxy * wedge.group1.yzx) - (other.group1.yzx * wedge.group1.zxy)), 0.0), 
            /* e235, e315, e125, e5 */
            (wedge.group1.wwwx * float4(other.group1.xyz, other.e15)) + float4(float3(0.0), (other.e25 * wedge.e4315) + (other.e35 * wedge.e4125) + (other.e45 * wedge.e3215) - (other.e4315 * wedge.e25) - (other.e4125 * wedge.e35) - (other.e3215 * wedge.e45)) - (other.group1.wwwx * float4(wedge.group1.xyz, wedge.e15))
        );
    }
}
public extension DualNum {
    public DualNum project_orthogonally_onto(Line other) {
        AntiFlatPoint wedge = AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(this.group0.xy.xx, this.e5, 0.0) * float4(float3(1.0), 0.0) * float4(other.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
        return DualNum(
            /* e5, e12345 */
            float2(-(wedge.e235 * other.e415) - (wedge.e315 * other.e425) - (wedge.e125 * other.e435), 0.0)
        );
    }
}
public extension DualNum {
    public Motor project_orthogonally_onto(Motor other) {
        AntiMotor right_anti_dual = AntiMotor(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        Motor wedge = Motor(
            /* e415, e425, e435, e12345 */
            float4(float3(0.0), right_anti_dual.scalar * this.e12345), 
            /* e235, e315, e125, e5 */
            float4(this.e5) * right_anti_dual.group0
        );
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(((float3(other.e12345) * wedge.group0.xyz) + (float3(wedge.e12345) * other.group0.xyz)), other.e12345 * wedge.e12345), 
            /* e235, e315, e125, e5 */
            (float4(other.e12345) * wedge.group1) + (float4(wedge.e12345) * other.group1) + float4(float3(0.0), -(other.e415 * wedge.e235) - (other.e425 * wedge.e315) - (other.e435 * wedge.e125) - (other.e235 * wedge.e415) - (other.e315 * wedge.e425) - (other.e125 * wedge.e435))
        );
    }
}
public extension DualNum {
    public MultiVector project_orthogonally_onto(MultiVector other) {
        MultiVector right_anti_dual = MultiVector(
            /* scalar, e12345 */
            other.group0.xy.yx * float2(-1.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4(other.group9.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5 */
            other.e3215, 
            /* e15, e25, e35, e45 */
            float4(other.group8.xyz, other.e321 * -1.0), 
            /* e41, e42, e43 */
            other.group7.xyz, 
            /* e23, e31, e12 */
            other.group6.xyz, 
            /* e415, e425, e435, e321 */
            float4(other.group5.xyz, other.e45) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e423, e431, e412 */
            other.group4.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            other.group3.xyz * float3(-1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.group1.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234 */
            other.e4 * -1.0
        );
        MultiVector wedge = MultiVector(
            /* scalar, e12345 */
            float2(0.0, (this.e5 * right_anti_dual.e1234) + (this.e12345 * right_anti_dual.scalar)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            this.e5 * right_anti_dual.scalar, 
            /* e15, e25, e35, e45 */
            float4(this.e5) * right_anti_dual.group1 * float4(-1.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(this.group0.xy.xx, this.e5, 0.0) * float4(float3(1.0), 0.0) * float4(right_anti_dual.group4.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(this.e5) * right_anti_dual.group5.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e5) * float4(right_anti_dual.group7.xyz, right_anti_dual.e321) * float4(-1.0), 
            /* e1234 */
            0.0
        );
        return MultiVector(
            /* scalar, e12345 */
            float2((other.scalar * wedge.e12345) + (other.e12345 * wedge.scalar) + (other.e1 * wedge.e4235) + (other.e2 * wedge.e4315) + (other.e3 * wedge.e4125) + (other.e4 * wedge.e3215) + (other.e5 * wedge.e1234) + (other.e4235 * wedge.e1) + (other.e4315 * wedge.e2) + (other.e4125 * wedge.e3) + (other.e3215 * wedge.e4) + (other.e1234 * wedge.e5) - (other.e15 * wedge.e423) - (other.e25 * wedge.e431) - (other.e35 * wedge.e412) - (other.e45 * wedge.e321) - (other.e41 * wedge.e235) - (other.e42 * wedge.e315) - (other.e43 * wedge.e125) - (other.e23 * wedge.e415) - (other.e31 * wedge.e425) - (other.e12 * wedge.e435) - (other.e415 * wedge.e23) - (other.e425 * wedge.e31) - (other.e435 * wedge.e12) - (other.e321 * wedge.e45) - (other.e423 * wedge.e15) - (other.e431 * wedge.e25) - (other.e412 * wedge.e35) - (other.e235 * wedge.e41) - (other.e315 * wedge.e42) - (other.e125 * wedge.e43), other.e12345 * wedge.e12345), 
            /* e1, e2, e3, e4 */
            float4((other.e415 * wedge.e321) + (other.e321 * wedge.e415) + (other.e4315 * wedge.e12) + (other.e1234 * wedge.e15), (other.e425 * wedge.e321) + (other.e321 * wedge.e425) + (other.e4125 * wedge.e23) + (other.e1234 * wedge.e25), (other.e435 * wedge.e321) + (other.e321 * wedge.e435) + (other.e4235 * wedge.e31) + (other.e1234 * wedge.e35), -(other.e45 * wedge.e1234) - (other.e415 * wedge.e423) - (other.e425 * wedge.e431) - (other.e435 * wedge.e412)) + (float4(other.e12345) * wedge.group1) + (float4(wedge.e12345) * other.group1) + float4((other.group4.xyz * wedge.group9.www), other.e4235 * wedge.e41) + float4((other.group5.xyz.yzx * wedge.group9.zxy), other.e4315 * wedge.e42) + float4((other.group7.xyz.zxy * wedge.group8.xyz.yzx), other.e4125 * wedge.e43) + float4((other.group8.xyz.yzx * wedge.group7.xyz.zxy), other.e1234 * wedge.e45) - (wedge.group9.yzxx * float4(other.group5.xyz.zxy, other.e41)) - float4((float3(wedge.e1234) * other.group3.xyz), other.e412 * wedge.e435) - float4((wedge.group4.xyz * other.group9.www), other.e423 * wedge.e415) - float4((other.group7.xyz.yzx * wedge.group8.xyz.zxy), other.e42 * wedge.e4315) - float4((other.group8.xyz.zxy * wedge.group7.xyz.yzx), other.e43 * wedge.e4125) - float4((wedge.group5.xyz.yzx * other.group9.zxy), other.e431 * wedge.e425), 
            /* e5 */
            (other.e12345 * wedge.e5) + (other.e5 * wedge.e12345) + (other.e15 * wedge.e4235) + (other.e25 * wedge.e4315) + (other.e35 * wedge.e4125) + (other.e45 * wedge.e3215) - (other.e415 * wedge.e235) - (other.e425 * wedge.e315) - (other.e435 * wedge.e125) - (other.e235 * wedge.e415) - (other.e315 * wedge.e425) - (other.e125 * wedge.e435) - (other.e4235 * wedge.e15) - (other.e4315 * wedge.e25) - (other.e4125 * wedge.e35) - (other.e3215 * wedge.e45), 
            /* e15, e25, e35, e45 */
            float4((other.e415 * wedge.e3215) + (other.e315 * wedge.e4125) + (other.e4125 * wedge.e315) + (other.e3215 * wedge.e415), (other.e425 * wedge.e3215) + (other.e125 * wedge.e4235) + (other.e4235 * wedge.e125) + (other.e3215 * wedge.e425), (other.e435 * wedge.e3215) + (other.e235 * wedge.e4315) + (other.e4315 * wedge.e235) + (other.e3215 * wedge.e435), -(other.e435 * wedge.e4125) - (other.e4235 * wedge.e415) - (other.e4315 * wedge.e425) - (other.e4125 * wedge.e435)) + (float4(other.e12345) * wedge.group3) + (float4(wedge.e12345) * other.group3) - (wedge.group9.yzxx * float4(other.group8.xyz.zxy, other.e415)) - float4((wedge.group8.xyz.zxy * other.group9.yzx), other.e425 * wedge.e4315), 
            /* e41, e42, e43 */
            (float3(other.e12345) * wedge.group4.xyz) + (float3(other.e1234) * wedge.group6.xyz) + (float3(wedge.e12345) * other.group4.xyz) + (float3(wedge.e1234) * other.group6.xyz) + (other.group7.xyz.zxy * wedge.group9.yzx) + (wedge.group7.xyz.zxy * other.group9.yzx) - (other.group7.xyz.yzx * wedge.group9.zxy) - (wedge.group7.xyz.yzx * other.group9.zxy), 
            /* e23, e31, e12 */
            (float3(other.e12345) * wedge.group5.xyz) + (float3(other.e3215) * wedge.group7.xyz) + (float3(other.e1234) * wedge.group8.xyz) + (float3(wedge.e12345) * other.group5.xyz) + (float3(wedge.e3215) * other.group7.xyz) + (float3(wedge.e1234) * other.group8.xyz) - (float3(other.e321) * wedge.group9.xyz) - (float3(wedge.e321) * other.group9.xyz), 
            /* e415, e425, e435, e321 */
            (float4(other.e12345) * wedge.group6) + (float4(wedge.e12345) * other.group6) + float4((other.group9.zxy * wedge.group9.yzx), other.e1234 * wedge.e3215) - (other.group9.yzxw * float4(wedge.group9.zxy, wedge.e1234)), 
            /* e423, e431, e412 */
            (float3(other.e12345) * wedge.group7.xyz) + (float3(other.e1234) * wedge.group9.xyz) + (float3(wedge.e12345) * other.group7.xyz) - (float3(wedge.e1234) * other.group9.xyz), 
            /* e235, e315, e125 */
            (float3(other.e12345) * wedge.group8.xyz) + (float3(wedge.e12345) * other.group8.xyz) + (float3(wedge.e3215) * other.group9.xyz) - (float3(other.e3215) * wedge.group9.xyz), 
            /* e4235, e4315, e4125, e3215 */
            (float4(other.e12345) * wedge.group9) + (float4(wedge.e12345) * other.group9), 
            /* e1234 */
            (other.e12345 * wedge.e1234) + (other.e1234 * wedge.e12345)
        );
    }
}
public extension DualNum {
    public DualNum project_orthogonally_onto(Plane other) {
        FlatPoint wedge = FlatPoint(
            /* e15, e25, e35, e45 */
            float4(this.group0.xy.xx, this.e5, 0.0) * float4(float3(1.0), 0.0) * float4(float4(other.e4235 * -1.0, other.e4315 * -1.0, other.e4125 * -1.0, other.e3215).xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0)
        );
        return DualNum(
            /* e5, e12345 */
            float2(-(wedge.e15 * other.e4235) - (wedge.e25 * other.e4315) - (wedge.e35 * other.e4125) - (wedge.e45 * other.e3215), 0.0)
        );
    }
}
public extension DualNum {
    public RoundPoint project_orthogonally_onto(RoundPoint other) {
        AntiScalar wedge = AntiScalar(
            /* e12345 */
            this.e5 * other.e4 * -1.0
        );
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(wedge.e12345) * other.group0, 
            /* e5 */
            wedge.e12345 * other.e5
        );
    }
}
public extension DualNum {
    public RoundPoint project_orthogonally_onto(Sphere other) {
        FlatPoint wedge = FlatPoint(
            /* e15, e25, e35, e45 */
            float4(this.e5) * float4(other.e4235 * -1.0, other.e4315 * -1.0, other.e4125 * -1.0, other.e1234) * float4(-1.0)
        );
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(other.e1234) * wedge.group0, 
            /* e5 */
            -(wedge.e15 * other.e4235) - (wedge.e25 * other.e4315) - (wedge.e35 * other.e4125) - (wedge.e45 * other.e3215)
        );
    }
}
public extension DualNum {
    public VersorEven project_orthogonally_onto(VersorEven other) {
        VersorOdd right_anti_dual = VersorOdd(
            /* e41, e42, e43, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e1234 */
            float4(other.group2.xyz, other.e4) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.group3.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0)
        );
        Motor wedge = Motor(
            /* e415, e425, e435, e12345 */
            float4(this.group0.xy.xx, this.e5, (this.e5 * right_anti_dual.e1234) + (this.e12345 * right_anti_dual.scalar)) * float4(right_anti_dual.group0.xyz, 1.0), 
            /* e235, e315, e125, e5 */
            float4(this.e5) * float4(right_anti_dual.group1.xyz, right_anti_dual.scalar)
        );
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(wedge.e12345) * other.group0, 
            /* e415, e425, e435, e321 */
            float4(((float3(wedge.e12345) * other.group1.xyz) + (float3(other.e12345) * wedge.group0.xyz)), wedge.e12345 * other.e321), 
            /* e235, e315, e125, e5 */
            (float4(wedge.e12345) * other.group2) + (float4(other.e12345) * wedge.group1) + float4(float3(0.0), -(wedge.e415 * other.e235) - (wedge.e425 * other.e315) - (wedge.e435 * other.e125) - (wedge.e235 * other.e415) - (wedge.e315 * other.e425) - (wedge.e125 * other.e435)), 
            /* e1, e2, e3, e4 */
            float4((wedge.e12345 * other.e1) + (wedge.e315 * other.e412), (wedge.e12345 * other.e2) + (wedge.e125 * other.e423), (wedge.e12345 * other.e3) + (wedge.e235 * other.e431), -(wedge.e425 * other.e431) - (wedge.e435 * other.e412)) + (wedge.group0 * float4(other.group1.www, other.e4)) - (other.group0.yzxx * float4(wedge.group1.zxy, wedge.e415))
        );
    }
}
public extension DualNum {
    public AntiDipoleInversion project_orthogonally_onto(VersorOdd other) {
        VersorEven right_anti_dual = VersorEven(
            /* e423, e431, e412, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            float4(other.group2.xyz, other.e3215) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4(other.group3.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0)
        );
        Flector wedge = Flector(
            /* e15, e25, e35, e45 */
            float4(this.e5) * right_anti_dual.group3 * float4(-1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e5) * float4(right_anti_dual.group0.xyz, right_anti_dual.e321) * float4(-1.0)
        );
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(other.e1234) * wedge.group1.xyz, 
            /* e415, e425, e435, e321 */
            float4(((wedge.group1.yzx * other.group3.zxy) - (wedge.group1.zxy * other.group3.yzx)), wedge.e3215 * other.e1234), 
            /* e235, e315, e125, e4 */
            float4(float3(0.0), -(wedge.e4315 * other.e42) - (wedge.e4125 * other.e43)) + float4((wedge.group1.www * other.group3.xyz), wedge.e45 * other.e1234) - (wedge.group1.xyzx * float4(other.group3.www, other.e41)), 
            /* e1, e2, e3, e5 */
            (wedge.group1.zxyy * float4(other.group1.yzx, other.e25)) + (wedge.group1.wwwz * float4(other.group0.xyz, other.e35)) + (other.group2.wwwx * float4(wedge.group0.xyz, wedge.e4235)) + float4(float3(0.0), (wedge.e3215 * other.e45) - (wedge.e25 * other.e4315) - (wedge.e35 * other.e4125) - (wedge.e45 * other.e3215)) - float4((wedge.group1.yzx * other.group1.zxy), wedge.e15 * other.e4235)
        );
    }
}
public extension FlatPoint {
    // Fancy infix trick (first half)
    public ProjectOrthogonallyOntoInfixPartial<FlatPoint> operator /(ProjectOrthogonallyOntoInfix rhs) {
        return ProjectOrthogonallyOntoInfixPartial<FlatPoint>(this);
    }
}
public extension ProjectOrthogonallyOntoInfixPartial<FlatPoint> {
    // Fancy infix trick (second half)
    public AntiCircleRotor operator /(AntiCircleRotor rhs) {
        return this.a.project_orthogonally_onto(rhs);
    }
}
public extension FlatPoint {
    public AntiCircleRotor project_orthogonally_onto(AntiCircleRotor other) {
        CircleRotor right_anti_dual = CircleRotor(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        AntiScalar wedge = AntiScalar(
            /* e12345 */
            -(right_anti_dual.e423 * this.e15) - (right_anti_dual.e431 * this.e25) - (right_anti_dual.e412 * this.e35) - (right_anti_dual.e321 * this.e45)
        );
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(wedge.e12345) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(wedge.e12345) * other.group1, 
            /* e15, e25, e35, scalar */
            float4(wedge.e12345) * other.group2
        );
    }
}
public extension FlatPoint {
    public AntiCircleRotor project_orthogonally_onto(AntiDipoleInversion other) {
        DipoleInversion right_anti_dual = DipoleInversion(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e1234 */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group3 * float4(1.0, 1.0, 1.0, -1.0)
        );
        Plane wedge = Plane(
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual.e42 * this.e35) + (right_anti_dual.e23 * this.e45), (right_anti_dual.e43 * this.e15) + (right_anti_dual.e31 * this.e45), (right_anti_dual.e41 * this.e25) + (right_anti_dual.e12 * this.e45), -(right_anti_dual.e31 * this.e25) - (right_anti_dual.e12 * this.e35)) - (this.group0.yzxx * float4(right_anti_dual.group0.xyz.zxy, right_anti_dual.e23))
        );
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (other.group0.xyz.zxy * wedge.group0.yzx) - (other.group0.xyz.yzx * wedge.group0.zxy), 
            /* e23, e31, e12, e45 */
            float4(other.e423 * wedge.e3215, other.e431 * wedge.e3215, other.e412 * wedge.e3215, -(other.e425 * wedge.e4315) - (other.e435 * wedge.e4125)) - (other.group1.wwwx * wedge.group0.xyzx), 
            /* e15, e25, e35, scalar */
            float4(other.e125 * wedge.e4315 * -1.0, other.e235 * wedge.e4125 * -1.0, other.e315 * wedge.e4235 * -1.0, (other.e2 * wedge.e4315) + (other.e3 * wedge.e4125)) + (float4(wedge.e3215) * float4(other.group1.xyz, other.e4)) + (wedge.group0.zxyx * float4(other.group2.yzx, other.e1))
        );
    }
}
public extension FlatPoint {
    public FlatPoint project_orthogonally_onto(AntiScalar other) {
        Scalar right_anti_dual = Scalar(
            /* scalar */
            other.e12345 * -1.0
        );
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(other.e12345) * float4(this.e15 * right_anti_dual.scalar, this.e25 * right_anti_dual.scalar, this.e35 * right_anti_dual.scalar, this.e45 * right_anti_dual.scalar)
        );
    }
}
public extension FlatPoint {
    public Dipole project_orthogonally_onto(Circle other) {
        Dipole right_anti_dual = Dipole(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35 */
            other.group2.xyz
        );
        Plane wedge = Plane(
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual.e42 * this.e35) + (right_anti_dual.e23 * this.e45), (right_anti_dual.e43 * this.e15) + (right_anti_dual.e31 * this.e45), (right_anti_dual.e41 * this.e25) + (right_anti_dual.e12 * this.e45), -(right_anti_dual.e31 * this.e25) - (right_anti_dual.e12 * this.e35)) - (this.group0.yzxx * float4(right_anti_dual.group0.xyz.zxy, right_anti_dual.e23))
        );
        return Dipole(
            /* e41, e42, e43 */
            (other.group0.xyz.zxy * wedge.group0.yzx) - (other.group0.xyz.yzx * wedge.group0.zxy), 
            /* e23, e31, e12, e45 */
            float4(other.e423 * wedge.e3215, other.e431 * wedge.e3215, other.e412 * wedge.e3215, -(other.e425 * wedge.e4315) - (other.e435 * wedge.e4125)) - (other.group1.wwwx * wedge.group0.xyzx), 
            /* e15, e25, e35 */
            (float3(wedge.e3215) * other.group1.xyz) + (other.group2.xyz.yzx * wedge.group0.zxy) - (other.group2.xyz.zxy * wedge.group0.yzx)
        );
    }
}
public extension FlatPoint {
    public VersorOdd project_orthogonally_onto(CircleRotor other) {
        AntiCircleRotor right_anti_dual = AntiCircleRotor(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        Flector wedge = Flector(
            /* e15, e25, e35, e45 */
            float4(right_anti_dual.scalar) * this.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual.e42 * this.e35) + (right_anti_dual.e23 * this.e45), (right_anti_dual.e43 * this.e15) + (right_anti_dual.e31 * this.e45), (right_anti_dual.e41 * this.e25) + (right_anti_dual.e12 * this.e45), -(right_anti_dual.e31 * this.e25) - (right_anti_dual.e12 * this.e35)) - (this.group0.yzxx * float4(right_anti_dual.group0.xyz.zxy, right_anti_dual.e23))
        );
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(other.e412 * wedge.e4315, other.e423 * wedge.e4125, other.e431 * wedge.e4235, -(other.e431 * wedge.e25) - (other.e412 * wedge.e35) - (other.e321 * wedge.e45)) - float4((other.group0.xyz.yzx * wedge.group1.zxy), other.e423 * wedge.e15), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), -(other.e425 * wedge.e4315) - (other.e435 * wedge.e4125)) + float4((other.group0.xyz * wedge.group1.www), other.e12345 * wedge.e45) - (other.group1.wwwx * wedge.group1.xyzx), 
            /* e15, e25, e35, e1234 */
            float4((float3(other.e12345) * wedge.group0.xyz), 0.0) + float4((float3(wedge.e3215) * other.group1.xyz), 0.0) + float4((other.group2.yzx * wedge.group1.zxy), 0.0) - float4((other.group2.zxy * wedge.group1.yzx), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345) * wedge.group1
        );
    }
}
public extension FlatPoint {
    public Dipole project_orthogonally_onto(Dipole other) {
        Circle right_anti_dual = Circle(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125 */
            other.group2.xyz * float3(-1.0)
        );
        AntiScalar wedge = AntiScalar(
            /* e12345 */
            -(right_anti_dual.e423 * this.e15) - (right_anti_dual.e431 * this.e25) - (right_anti_dual.e412 * this.e35) - (right_anti_dual.e321 * this.e45)
        );
        return Dipole(
            /* e41, e42, e43 */
            float3(wedge.e12345) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(wedge.e12345) * other.group1, 
            /* e15, e25, e35 */
            float3(wedge.e12345) * other.group2.xyz
        );
    }
}
public extension FlatPoint {
    public VersorOdd project_orthogonally_onto(DipoleInversion other) {
        AntiDipoleInversion right_anti_dual = AntiDipoleInversion(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e4 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group3 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        Motor wedge = Motor(
            /* e415, e425, e435, e12345 */
            float4(right_anti_dual.e4 * this.e15, right_anti_dual.e4 * this.e25, right_anti_dual.e4 * this.e35, -(right_anti_dual.e431 * this.e25) - (right_anti_dual.e412 * this.e35) - (right_anti_dual.e321 * this.e45)) - (this.group0.wwwx * float4(right_anti_dual.group3.xyz, right_anti_dual.e423)), 
            /* e235, e315, e125, e5 */
            float4(((right_anti_dual.group3.yzx * this.group0.zxy) - (right_anti_dual.group3.zxy * this.group0.yzx)), 0.0)
        );
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(other.e1234 * wedge.e415, other.e1234 * wedge.e425, other.e1234 * wedge.e435, -(other.e41 * wedge.e235) - (other.e42 * wedge.e315) - (other.e43 * wedge.e125) - (other.e23 * wedge.e415) - (other.e31 * wedge.e425) - (other.e12 * wedge.e435)) + float4((other.group0.xyz * wedge.group0.www), other.e1234 * wedge.e5), 
            /* e23, e31, e12, e45 */
            float4(other.e1234 * wedge.e235, other.e1234 * wedge.e315, other.e1234 * wedge.e125, -(other.e4235 * wedge.e415) - (other.e4315 * wedge.e425) - (other.e4125 * wedge.e435)) + (float4(wedge.e12345) * other.group1), 
            /* e15, e25, e35, e1234 */
            float4(((float3(other.e3215) * wedge.group0.xyz) + (float3(wedge.e12345) * other.group2.xyz) + (other.group3.zxy * wedge.group1.yzx) - (other.group3.yzx * wedge.group1.zxy)), other.e1234 * wedge.e12345), 
            /* e4235, e4315, e4125, e3215 */
            float4(wedge.e12345) * other.group3
        );
    }
}
public extension FlatPoint {
    public FlatPoint project_orthogonally_onto(DualNum other) {
        AntiDualNum right_anti_dual = AntiDualNum(
            /* e3215, scalar */
            other.group0.xy * float2(-1.0)
        );
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(other.e12345) * float4(right_anti_dual.scalar * this.e15, right_anti_dual.scalar * this.e25, right_anti_dual.scalar * this.e35, right_anti_dual.scalar * this.e45)
        );
    }
}
public extension FlatPoint {
    public FlatPoint project_orthogonally_onto(FlatPoint other) {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(other.e45 * this.e45 * -1.0) * other.group0
        );
    }
}
public extension FlatPoint {
    public Flector project_orthogonally_onto(Flector other) {
        AntiFlector right_anti_dual = AntiFlector(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        Motor wedge = Motor(
            /* e415, e425, e435, e12345 */
            float4(this.e45) * float4(right_anti_dual.group1.xyz, right_anti_dual.e321) * float4(-1.0), 
            /* e235, e315, e125, e5 */
            float4(((right_anti_dual.group1.yzx * this.group0.zxy) - (right_anti_dual.group1.zxy * this.group0.yzx)), 0.0)
        );
        return Flector(
            /* e15, e25, e35, e45 */
            float4((other.e4125 * wedge.e315) + (other.e3215 * wedge.e415), (other.e4235 * wedge.e125) + (other.e3215 * wedge.e425), (other.e4315 * wedge.e235) + (other.e3215 * wedge.e435), -(other.e4315 * wedge.e425) - (other.e4125 * wedge.e435)) + (float4(wedge.e12345) * other.group0) - (other.group1.yzxx * float4(wedge.group1.zxy, wedge.e415)), 
            /* e4235, e4315, e4125, e3215 */
            float4(wedge.e12345) * other.group1
        );
    }
}
public extension FlatPoint {
    public FlatPoint project_orthogonally_onto(Line other) {
        AntiLine right_anti_dual = AntiLine(
            /* e23, e31, e12 */
            other.group0.xyz, 
            /* e15, e25, e35 */
            other.group1.xyz
        );
        Plane wedge = Plane(
            /* e4235, e4315, e4125, e3215 */
            float4((float3(this.e45) * right_anti_dual.group0.xyz), -(right_anti_dual.e23 * this.e15) - (right_anti_dual.e31 * this.e25) - (right_anti_dual.e12 * this.e35))
        );
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4((other.e415 * wedge.e3215) + (other.e315 * wedge.e4125), (other.e425 * wedge.e3215) + (other.e125 * wedge.e4235), (other.e435 * wedge.e3215) + (other.e235 * wedge.e4315), -(other.e425 * wedge.e4315) - (other.e435 * wedge.e4125)) - (wedge.group0.yzxx * float4(other.group1.xyz.zxy, other.e415))
        );
    }
}
public extension FlatPoint {
    public Flector project_orthogonally_onto(Motor other) {
        AntiMotor right_anti_dual = AntiMotor(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        Flector wedge = Flector(
            /* e15, e25, e35, e45 */
            float4(right_anti_dual.scalar) * this.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4((float3(this.e45) * right_anti_dual.group0.xyz), -(right_anti_dual.e23 * this.e15) - (right_anti_dual.e31 * this.e25) - (right_anti_dual.e12 * this.e35))
        );
        return Flector(
            /* e15, e25, e35, e45 */
            float4((wedge.e4125 * other.e315) + (wedge.e3215 * other.e415), (wedge.e4235 * other.e125) + (wedge.e3215 * other.e425), (wedge.e4315 * other.e235) + (wedge.e3215 * other.e435), -(wedge.e4315 * other.e425) - (wedge.e4125 * other.e435)) + (float4(other.e12345) * wedge.group0) - (wedge.group1.yzxx * float4(other.group1.zxy, other.e415)), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345) * wedge.group1
        );
    }
}
public extension FlatPoint {
    public MultiVector project_orthogonally_onto(MultiVector other) {
        MultiVector right_anti_dual = MultiVector(
            /* scalar, e12345 */
            other.group0.xy.yx * float2(-1.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4(other.group9.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5 */
            other.e3215, 
            /* e15, e25, e35, e45 */
            float4(other.group8.xyz, other.e321 * -1.0), 
            /* e41, e42, e43 */
            other.group7.xyz, 
            /* e23, e31, e12 */
            other.group6.xyz, 
            /* e415, e425, e435, e321 */
            float4(other.group5.xyz, other.e45) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e423, e431, e412 */
            other.group4.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            other.group3.xyz * float3(-1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.group1.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234 */
            other.e4 * -1.0
        );
        MultiVector wedge = MultiVector(
            /* scalar, e12345 */
            float2(0.0, -(this.e15 * right_anti_dual.e423) - (this.e25 * right_anti_dual.e431) - (this.e35 * right_anti_dual.e412) - (this.e45 * right_anti_dual.e321)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            float4(right_anti_dual.scalar) * this.group0, 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(((float3(right_anti_dual.e4) * this.group0.xyz) - (float3(this.e45) * right_anti_dual.group1.xyz)), 0.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            (this.group0.zxy * right_anti_dual.group1.yzx) - (this.group0.yzx * right_anti_dual.group1.zxy), 
            /* e4235, e4315, e4125, e3215 */
            float4((this.e35 * right_anti_dual.e42) + (this.e45 * right_anti_dual.e23), (this.e15 * right_anti_dual.e43) + (this.e45 * right_anti_dual.e31), (this.e25 * right_anti_dual.e41) + (this.e45 * right_anti_dual.e12), -(this.e25 * right_anti_dual.e31) - (this.e35 * right_anti_dual.e12)) - (this.group0.yzxx * float4(right_anti_dual.group4.xyz.zxy, right_anti_dual.e23)), 
            /* e1234 */
            0.0
        );
        return MultiVector(
            /* scalar, e12345 */
            float2((other.scalar * wedge.e12345) + (other.e12345 * wedge.scalar) + (other.e1 * wedge.e4235) + (other.e2 * wedge.e4315) + (other.e3 * wedge.e4125) + (other.e4 * wedge.e3215) + (other.e5 * wedge.e1234) + (other.e4235 * wedge.e1) + (other.e4315 * wedge.e2) + (other.e4125 * wedge.e3) + (other.e3215 * wedge.e4) + (other.e1234 * wedge.e5) - (other.e15 * wedge.e423) - (other.e25 * wedge.e431) - (other.e35 * wedge.e412) - (other.e45 * wedge.e321) - (other.e41 * wedge.e235) - (other.e42 * wedge.e315) - (other.e43 * wedge.e125) - (other.e23 * wedge.e415) - (other.e31 * wedge.e425) - (other.e12 * wedge.e435) - (other.e415 * wedge.e23) - (other.e425 * wedge.e31) - (other.e435 * wedge.e12) - (other.e321 * wedge.e45) - (other.e423 * wedge.e15) - (other.e431 * wedge.e25) - (other.e412 * wedge.e35) - (other.e235 * wedge.e41) - (other.e315 * wedge.e42) - (other.e125 * wedge.e43), other.e12345 * wedge.e12345), 
            /* e1, e2, e3, e4 */
            float4((other.e415 * wedge.e321) + (other.e321 * wedge.e415) + (other.e4315 * wedge.e12) + (other.e1234 * wedge.e15), (other.e425 * wedge.e321) + (other.e321 * wedge.e425) + (other.e4125 * wedge.e23) + (other.e1234 * wedge.e25), (other.e435 * wedge.e321) + (other.e321 * wedge.e435) + (other.e4235 * wedge.e31) + (other.e1234 * wedge.e35), -(other.e45 * wedge.e1234) - (other.e415 * wedge.e423) - (other.e425 * wedge.e431) - (other.e435 * wedge.e412)) + (float4(other.e12345) * wedge.group1) + (float4(wedge.e12345) * other.group1) + float4((other.group4.xyz * wedge.group9.www), other.e4235 * wedge.e41) + float4((other.group5.xyz.yzx * wedge.group9.zxy), other.e4315 * wedge.e42) + float4((other.group7.xyz.zxy * wedge.group8.xyz.yzx), other.e4125 * wedge.e43) + float4((other.group8.xyz.yzx * wedge.group7.xyz.zxy), other.e1234 * wedge.e45) - (wedge.group9.yzxx * float4(other.group5.xyz.zxy, other.e41)) - float4((float3(wedge.e1234) * other.group3.xyz), other.e412 * wedge.e435) - float4((wedge.group4.xyz * other.group9.www), other.e423 * wedge.e415) - float4((other.group7.xyz.yzx * wedge.group8.xyz.zxy), other.e42 * wedge.e4315) - float4((other.group8.xyz.zxy * wedge.group7.xyz.yzx), other.e43 * wedge.e4125) - float4((wedge.group5.xyz.yzx * other.group9.zxy), other.e431 * wedge.e425), 
            /* e5 */
            (other.e12345 * wedge.e5) + (other.e5 * wedge.e12345) + (other.e15 * wedge.e4235) + (other.e25 * wedge.e4315) + (other.e35 * wedge.e4125) + (other.e45 * wedge.e3215) - (other.e415 * wedge.e235) - (other.e425 * wedge.e315) - (other.e435 * wedge.e125) - (other.e235 * wedge.e415) - (other.e315 * wedge.e425) - (other.e125 * wedge.e435) - (other.e4235 * wedge.e15) - (other.e4315 * wedge.e25) - (other.e4125 * wedge.e35) - (other.e3215 * wedge.e45), 
            /* e15, e25, e35, e45 */
            float4((other.e415 * wedge.e3215) + (other.e315 * wedge.e4125) + (other.e4125 * wedge.e315) + (other.e3215 * wedge.e415), (other.e425 * wedge.e3215) + (other.e125 * wedge.e4235) + (other.e4235 * wedge.e125) + (other.e3215 * wedge.e425), (other.e435 * wedge.e3215) + (other.e235 * wedge.e4315) + (other.e4315 * wedge.e235) + (other.e3215 * wedge.e435), -(other.e435 * wedge.e4125) - (other.e4235 * wedge.e415) - (other.e4315 * wedge.e425) - (other.e4125 * wedge.e435)) + (float4(other.e12345) * wedge.group3) + (float4(wedge.e12345) * other.group3) - (wedge.group9.yzxx * float4(other.group8.xyz.zxy, other.e415)) - float4((wedge.group8.xyz.zxy * other.group9.yzx), other.e425 * wedge.e4315), 
            /* e41, e42, e43 */
            (float3(other.e12345) * wedge.group4.xyz) + (float3(other.e1234) * wedge.group6.xyz) + (float3(wedge.e12345) * other.group4.xyz) + (float3(wedge.e1234) * other.group6.xyz) + (other.group7.xyz.zxy * wedge.group9.yzx) + (wedge.group7.xyz.zxy * other.group9.yzx) - (other.group7.xyz.yzx * wedge.group9.zxy) - (wedge.group7.xyz.yzx * other.group9.zxy), 
            /* e23, e31, e12 */
            (float3(other.e12345) * wedge.group5.xyz) + (float3(other.e3215) * wedge.group7.xyz) + (float3(other.e1234) * wedge.group8.xyz) + (float3(wedge.e12345) * other.group5.xyz) + (float3(wedge.e3215) * other.group7.xyz) + (float3(wedge.e1234) * other.group8.xyz) - (float3(other.e321) * wedge.group9.xyz) - (float3(wedge.e321) * other.group9.xyz), 
            /* e415, e425, e435, e321 */
            (float4(other.e12345) * wedge.group6) + (float4(wedge.e12345) * other.group6) + float4((other.group9.zxy * wedge.group9.yzx), other.e1234 * wedge.e3215) - (other.group9.yzxw * float4(wedge.group9.zxy, wedge.e1234)), 
            /* e423, e431, e412 */
            (float3(other.e12345) * wedge.group7.xyz) + (float3(other.e1234) * wedge.group9.xyz) + (float3(wedge.e12345) * other.group7.xyz) - (float3(wedge.e1234) * other.group9.xyz), 
            /* e235, e315, e125 */
            (float3(other.e12345) * wedge.group8.xyz) + (float3(wedge.e12345) * other.group8.xyz) + (float3(wedge.e3215) * other.group9.xyz) - (float3(other.e3215) * wedge.group9.xyz), 
            /* e4235, e4315, e4125, e3215 */
            (float4(other.e12345) * wedge.group9) + (float4(wedge.e12345) * other.group9), 
            /* e1234 */
            (other.e12345 * wedge.e1234) + (other.e1234 * wedge.e12345)
        );
    }
}
public extension FlatPoint {
    public FlatPoint project_orthogonally_onto(Plane other) {
        AntiPlane right_anti_dual = AntiPlane(
            /* e1, e2, e3, e5 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        Line wedge = Line(
            /* e415, e425, e435 */
            float3(this.e45) * right_anti_dual.group0.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            (right_anti_dual.group0.yzx * this.group0.zxy) - (right_anti_dual.group0.zxy * this.group0.yzx)
        );
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4((wedge.e415 * other.e3215) + (wedge.e315 * other.e4125), (wedge.e425 * other.e3215) + (wedge.e125 * other.e4235), (wedge.e435 * other.e3215) + (wedge.e235 * other.e4315), -(wedge.e425 * other.e4315) - (wedge.e435 * other.e4125)) - (other.group0.yzxx * float4(wedge.group1.xyz.zxy, wedge.e415))
        );
    }
}
public extension FlatPoint {
    public Dipole project_orthogonally_onto(Sphere other) {
        RoundPoint right_anti_dual = RoundPoint(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5 */
            other.e3215
        );
        Line wedge = Line(
            /* e415, e425, e435 */
            (float3(right_anti_dual.e4) * this.group0.xyz) - (float3(this.e45) * right_anti_dual.group0.xyz), 
            /* e235, e315, e125 */
            (this.group0.zxy * right_anti_dual.group0.yzx) - (this.group0.yzx * right_anti_dual.group0.zxy)
        );
        return Dipole(
            /* e41, e42, e43 */
            float3(other.e1234) * wedge.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4((float3(other.e1234) * wedge.group1.xyz), -(wedge.e415 * other.e4235) - (wedge.e425 * other.e4315) - (wedge.e435 * other.e4125)), 
            /* e15, e25, e35 */
            (float3(other.e3215) * wedge.group0.xyz) + (wedge.group1.xyz.yzx * other.group0.zxy) - (wedge.group1.xyz.zxy * other.group0.yzx)
        );
    }
}
public extension FlatPoint {
    public VersorOdd project_orthogonally_onto(VersorEven other) {
        VersorOdd right_anti_dual = VersorOdd(
            /* e41, e42, e43, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e1234 */
            float4(other.group2.xyz, other.e4) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.group3.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0)
        );
        Flector wedge = Flector(
            /* e15, e25, e35, e45 */
            float4(right_anti_dual.scalar) * this.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4((this.e35 * right_anti_dual.e42) + (this.e45 * right_anti_dual.e23), (this.e15 * right_anti_dual.e43) + (this.e45 * right_anti_dual.e31), (this.e25 * right_anti_dual.e41) + (this.e45 * right_anti_dual.e12), -(this.e25 * right_anti_dual.e31) - (this.e35 * right_anti_dual.e12)) - (this.group0.yzxx * float4(right_anti_dual.group0.zxy, right_anti_dual.e23))
        );
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (wedge.group1.yzxx * float4(other.group0.zxy, other.e1)) + float4(float3(0.0), (wedge.e4315 * other.e2) + (wedge.e4125 * other.e3) + (wedge.e3215 * other.e4) - (wedge.e25 * other.e431) - (wedge.e35 * other.e412) - (wedge.e45 * other.e321)) - (other.group0.yzxx * float4(wedge.group1.zxy, wedge.e15)), 
            /* e23, e31, e12, e45 */
            (other.group0 * float4(wedge.group1.www, wedge.e45)) + float4(float3(0.0), -(wedge.e4315 * other.e425) - (wedge.e4125 * other.e435)) - (wedge.group1.xyzx * other.group1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4((float3(wedge.e3215) * other.group1.xyz), 0.0) + float4((float3(other.e12345) * wedge.group0.xyz), 0.0) + float4((wedge.group1.zxy * other.group2.yzx), 0.0) - float4((wedge.group1.yzx * other.group2.zxy), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345) * wedge.group1
        );
    }
}
public extension FlatPoint {
    public VersorOdd project_orthogonally_onto(VersorOdd other) {
        VersorEven right_anti_dual = VersorEven(
            /* e423, e431, e412, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            float4(other.group2.xyz, other.e3215) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4(other.group3.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0)
        );
        Motor wedge = Motor(
            /* e415, e425, e435, e12345 */
            float4(this.e15 * right_anti_dual.e4, this.e25 * right_anti_dual.e4, this.e35 * right_anti_dual.e4, -(this.e25 * right_anti_dual.e431) - (this.e35 * right_anti_dual.e412) - (this.e45 * right_anti_dual.e321)) - (this.group0.wwwx * float4(right_anti_dual.group3.xyz, right_anti_dual.e423)), 
            /* e235, e315, e125, e5 */
            float4(((this.group0.zxy * right_anti_dual.group3.yzx) - (this.group0.yzx * right_anti_dual.group3.zxy)), 0.0)
        );
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (wedge.group0 * float4(other.group2.www, other.scalar)) + float4(float3(0.0), -(wedge.e415 * other.e23) - (wedge.e425 * other.e31) - (wedge.e435 * other.e12) - (wedge.e235 * other.e41) - (wedge.e315 * other.e42) - (wedge.e125 * other.e43)) + float4((wedge.group0.www * other.group0.xyz), wedge.e5 * other.e1234), 
            /* e23, e31, e12, e45 */
            float4(wedge.e235 * other.e1234, wedge.e315 * other.e1234, wedge.e125 * other.e1234, -(wedge.e415 * other.e4235) - (wedge.e425 * other.e4315) - (wedge.e435 * other.e4125)) + (float4(wedge.e12345) * other.group1), 
            /* e15, e25, e35, e1234 */
            float4(((float3(wedge.e12345) * other.group2.xyz) + (float3(other.e3215) * wedge.group0.xyz) + (wedge.group1.yzx * other.group3.zxy) - (wedge.group1.zxy * other.group3.yzx)), wedge.e12345 * other.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(wedge.e12345) * other.group3
        );
    }
}
public extension Flector {
    // Fancy infix trick (first half)
    public ProjectOrthogonallyOntoInfixPartial<Flector> operator /(ProjectOrthogonallyOntoInfix rhs) {
        return ProjectOrthogonallyOntoInfixPartial<Flector>(this);
    }
}
public extension ProjectOrthogonallyOntoInfixPartial<Flector> {
    // Fancy infix trick (second half)
    public AntiCircleRotor operator /(AntiCircleRotor rhs) {
        return this.a.project_orthogonally_onto(rhs);
    }
}
public extension Flector {
    public AntiCircleRotor project_orthogonally_onto(AntiCircleRotor other) {
        CircleRotor right_anti_dual = CircleRotor(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        AntiScalar wedge = AntiScalar(
            /* e12345 */
            -(right_anti_dual.e423 * this.e15) - (right_anti_dual.e431 * this.e25) - (right_anti_dual.e412 * this.e35) - (right_anti_dual.e321 * this.e45)
        );
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(wedge.e12345) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(wedge.e12345) * other.group1, 
            /* e15, e25, e35, scalar */
            float4(wedge.e12345) * other.group2
        );
    }
}
public extension Flector {
    public AntiCircleRotor project_orthogonally_onto(AntiDipoleInversion other) {
        DipoleInversion right_anti_dual = DipoleInversion(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e1234 */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group3 * float4(1.0, 1.0, 1.0, -1.0)
        );
        Plane wedge = Plane(
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual.e42 * this.e35) + (right_anti_dual.e23 * this.e45), (right_anti_dual.e43 * this.e15) + (right_anti_dual.e31 * this.e45), (right_anti_dual.e41 * this.e25) + (right_anti_dual.e12 * this.e45), -(right_anti_dual.e31 * this.e25) - (right_anti_dual.e12 * this.e35)) - (this.group0.yzxx * float4(right_anti_dual.group0.xyz.zxy, right_anti_dual.e23))
        );
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (other.group0.xyz.zxy * wedge.group0.yzx) - (other.group0.xyz.yzx * wedge.group0.zxy), 
            /* e23, e31, e12, e45 */
            float4(other.e423 * wedge.e3215, other.e431 * wedge.e3215, other.e412 * wedge.e3215, -(other.e425 * wedge.e4315) - (other.e435 * wedge.e4125)) - (other.group1.wwwx * wedge.group0.xyzx), 
            /* e15, e25, e35, scalar */
            float4(other.e125 * wedge.e4315 * -1.0, other.e235 * wedge.e4125 * -1.0, other.e315 * wedge.e4235 * -1.0, (other.e2 * wedge.e4315) + (other.e3 * wedge.e4125)) + (float4(wedge.e3215) * float4(other.group1.xyz, other.e4)) + (wedge.group0.zxyx * float4(other.group2.yzx, other.e1))
        );
    }
}
public extension Flector {
    public Flector project_orthogonally_onto(AntiScalar other) {
        Scalar right_anti_dual = Scalar(
            /* scalar */
            other.e12345 * -1.0
        );
        Flector wedge = Flector(
            /* e15, e25, e35, e45 */
            float4(right_anti_dual.scalar) * this.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual.scalar) * this.group1
        );
        return Flector(
            /* e15, e25, e35, e45 */
            float4(other.e12345) * wedge.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345) * wedge.group1
        );
    }
}
public extension Flector {
    public Dipole project_orthogonally_onto(Circle other) {
        Dipole right_anti_dual = Dipole(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35 */
            other.group2.xyz
        );
        Plane wedge = Plane(
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual.e42 * this.e35) + (right_anti_dual.e23 * this.e45), (right_anti_dual.e43 * this.e15) + (right_anti_dual.e31 * this.e45), (right_anti_dual.e41 * this.e25) + (right_anti_dual.e12 * this.e45), -(right_anti_dual.e31 * this.e25) - (right_anti_dual.e12 * this.e35)) - (this.group0.yzxx * float4(right_anti_dual.group0.xyz.zxy, right_anti_dual.e23))
        );
        return Dipole(
            /* e41, e42, e43 */
            (other.group0.xyz.zxy * wedge.group0.yzx) - (other.group0.xyz.yzx * wedge.group0.zxy), 
            /* e23, e31, e12, e45 */
            float4(other.e423 * wedge.e3215, other.e431 * wedge.e3215, other.e412 * wedge.e3215, -(other.e425 * wedge.e4315) - (other.e435 * wedge.e4125)) - (other.group1.wwwx * wedge.group0.xyzx), 
            /* e15, e25, e35 */
            (float3(wedge.e3215) * other.group1.xyz) + (other.group2.xyz.yzx * wedge.group0.zxy) - (other.group2.xyz.zxy * wedge.group0.yzx)
        );
    }
}
public extension Flector {
    public VersorOdd project_orthogonally_onto(CircleRotor other) {
        AntiCircleRotor right_anti_dual = AntiCircleRotor(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        Flector wedge = Flector(
            /* e15, e25, e35, e45 */
            float4(right_anti_dual.scalar) * this.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual.e23 * this.e45) + (right_anti_dual.scalar * this.e4235), (right_anti_dual.e31 * this.e45) + (right_anti_dual.scalar * this.e4315), (right_anti_dual.e12 * this.e45) + (right_anti_dual.scalar * this.e4125), -(right_anti_dual.e31 * this.e25) - (right_anti_dual.e12 * this.e35)) + float4((right_anti_dual.group0.xyz.yzx * this.group0.zxy), right_anti_dual.scalar * this.e3215) - (this.group0.yzxx * float4(right_anti_dual.group0.xyz.zxy, right_anti_dual.e23))
        );
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(other.e412 * wedge.e4315, other.e423 * wedge.e4125, other.e431 * wedge.e4235, -(other.e431 * wedge.e25) - (other.e412 * wedge.e35) - (other.e321 * wedge.e45)) - float4((other.group0.xyz.yzx * wedge.group1.zxy), other.e423 * wedge.e15), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), -(other.e425 * wedge.e4315) - (other.e435 * wedge.e4125)) + float4((other.group0.xyz * wedge.group1.www), other.e12345 * wedge.e45) - (other.group1.wwwx * wedge.group1.xyzx), 
            /* e15, e25, e35, e1234 */
            float4((float3(other.e12345) * wedge.group0.xyz), 0.0) + float4((float3(wedge.e3215) * other.group1.xyz), 0.0) + float4((other.group2.yzx * wedge.group1.zxy), 0.0) - float4((other.group2.zxy * wedge.group1.yzx), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345) * wedge.group1
        );
    }
}
public extension Flector {
    public Dipole project_orthogonally_onto(Dipole other) {
        Circle right_anti_dual = Circle(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125 */
            other.group2.xyz * float3(-1.0)
        );
        AntiScalar wedge = AntiScalar(
            /* e12345 */
            -(right_anti_dual.e423 * this.e15) - (right_anti_dual.e431 * this.e25) - (right_anti_dual.e412 * this.e35) - (right_anti_dual.e321 * this.e45)
        );
        return Dipole(
            /* e41, e42, e43 */
            float3(wedge.e12345) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(wedge.e12345) * other.group1, 
            /* e15, e25, e35 */
            float3(wedge.e12345) * other.group2.xyz
        );
    }
}
public extension Flector {
    public VersorOdd project_orthogonally_onto(DipoleInversion other) {
        AntiDipoleInversion right_anti_dual = AntiDipoleInversion(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e4 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group3 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        Motor wedge = Motor(
            /* e415, e425, e435, e12345 */
            (float4(right_anti_dual.e4) * float4(this.group0.xyz, this.e3215)) + float4(float3(0.0), (right_anti_dual.e1 * this.e4235) + (right_anti_dual.e2 * this.e4315) + (right_anti_dual.e3 * this.e4125) - (right_anti_dual.e431 * this.e25) - (right_anti_dual.e412 * this.e35) - (right_anti_dual.e321 * this.e45)) - (this.group0.wwwx * float4(right_anti_dual.group3.xyz, right_anti_dual.e423)), 
            /* e235, e315, e125, e5 */
            float4(((right_anti_dual.group3.yzx * this.group0.zxy) - (right_anti_dual.group3.zxy * this.group0.yzx)), 0.0)
        );
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(other.e1234 * wedge.e415, other.e1234 * wedge.e425, other.e1234 * wedge.e435, -(other.e41 * wedge.e235) - (other.e42 * wedge.e315) - (other.e43 * wedge.e125) - (other.e23 * wedge.e415) - (other.e31 * wedge.e425) - (other.e12 * wedge.e435)) + float4((other.group0.xyz * wedge.group0.www), other.e1234 * wedge.e5), 
            /* e23, e31, e12, e45 */
            float4(other.e1234 * wedge.e235, other.e1234 * wedge.e315, other.e1234 * wedge.e125, -(other.e4235 * wedge.e415) - (other.e4315 * wedge.e425) - (other.e4125 * wedge.e435)) + (float4(wedge.e12345) * other.group1), 
            /* e15, e25, e35, e1234 */
            float4(((float3(other.e3215) * wedge.group0.xyz) + (float3(wedge.e12345) * other.group2.xyz) + (other.group3.zxy * wedge.group1.yzx) - (other.group3.yzx * wedge.group1.zxy)), other.e1234 * wedge.e12345), 
            /* e4235, e4315, e4125, e3215 */
            float4(wedge.e12345) * other.group3
        );
    }
}
public extension Flector {
    public Flector project_orthogonally_onto(DualNum other) {
        AntiDualNum right_anti_dual = AntiDualNum(
            /* e3215, scalar */
            other.group0.xy * float2(-1.0)
        );
        Flector wedge = Flector(
            /* e15, e25, e35, e45 */
            float4(right_anti_dual.scalar) * this.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual.scalar) * this.group1
        );
        return Flector(
            /* e15, e25, e35, e45 */
            float4(other.e12345) * wedge.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345) * wedge.group1
        );
    }
}
public extension Flector {
    public FlatPoint project_orthogonally_onto(FlatPoint other) {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(other.e45 * this.e45 * -1.0) * other.group0
        );
    }
}
public extension Flector {
    public Flector project_orthogonally_onto(Flector other) {
        AntiFlector right_anti_dual = AntiFlector(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        Motor wedge = Motor(
            /* e415, e425, e435, e12345 */
            float4((float3(this.e45) * right_anti_dual.group1.xyz * float3(-1.0)), (right_anti_dual.e1 * this.e4235) + (right_anti_dual.e2 * this.e4315) + (right_anti_dual.e3 * this.e4125) - (right_anti_dual.e321 * this.e45)), 
            /* e235, e315, e125, e5 */
            float4(((right_anti_dual.group1.yzx * this.group0.zxy) - (right_anti_dual.group1.zxy * this.group0.yzx)), 0.0)
        );
        return Flector(
            /* e15, e25, e35, e45 */
            float4((other.e4125 * wedge.e315) + (other.e3215 * wedge.e415), (other.e4235 * wedge.e125) + (other.e3215 * wedge.e425), (other.e4315 * wedge.e235) + (other.e3215 * wedge.e435), -(other.e4315 * wedge.e425) - (other.e4125 * wedge.e435)) + (float4(wedge.e12345) * other.group0) - (other.group1.yzxx * float4(wedge.group1.zxy, wedge.e415)), 
            /* e4235, e4315, e4125, e3215 */
            float4(wedge.e12345) * other.group1
        );
    }
}
public extension Flector {
    public FlatPoint project_orthogonally_onto(Line other) {
        AntiLine right_anti_dual = AntiLine(
            /* e23, e31, e12 */
            other.group0.xyz, 
            /* e15, e25, e35 */
            other.group1.xyz
        );
        Plane wedge = Plane(
            /* e4235, e4315, e4125, e3215 */
            float4((float3(this.e45) * right_anti_dual.group0.xyz), -(right_anti_dual.e23 * this.e15) - (right_anti_dual.e31 * this.e25) - (right_anti_dual.e12 * this.e35))
        );
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4((other.e415 * wedge.e3215) + (other.e315 * wedge.e4125), (other.e425 * wedge.e3215) + (other.e125 * wedge.e4235), (other.e435 * wedge.e3215) + (other.e235 * wedge.e4315), -(other.e425 * wedge.e4315) - (other.e435 * wedge.e4125)) - (wedge.group0.yzxx * float4(other.group1.xyz.zxy, other.e415))
        );
    }
}
public extension Flector {
    public Flector project_orthogonally_onto(Motor other) {
        AntiMotor right_anti_dual = AntiMotor(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        Flector wedge = Flector(
            /* e15, e25, e35, e45 */
            float4(right_anti_dual.scalar) * this.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual.scalar * this.e4235, right_anti_dual.scalar * this.e4315, right_anti_dual.scalar * this.e4125, -(right_anti_dual.e23 * this.e15) - (right_anti_dual.e31 * this.e25) - (right_anti_dual.e12 * this.e35)) + (right_anti_dual.group0 * float4(this.group0.www, this.e3215))
        );
        return Flector(
            /* e15, e25, e35, e45 */
            float4((wedge.e4125 * other.e315) + (wedge.e3215 * other.e415), (wedge.e4235 * other.e125) + (wedge.e3215 * other.e425), (wedge.e4315 * other.e235) + (wedge.e3215 * other.e435), -(wedge.e4315 * other.e425) - (wedge.e4125 * other.e435)) + (float4(other.e12345) * wedge.group0) - (wedge.group1.yzxx * float4(other.group1.zxy, other.e415)), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345) * wedge.group1
        );
    }
}
public extension Flector {
    public MultiVector project_orthogonally_onto(MultiVector other) {
        MultiVector right_anti_dual = MultiVector(
            /* scalar, e12345 */
            other.group0.xy.yx * float2(-1.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4(other.group9.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5 */
            other.e3215, 
            /* e15, e25, e35, e45 */
            float4(other.group8.xyz, other.e321 * -1.0), 
            /* e41, e42, e43 */
            other.group7.xyz, 
            /* e23, e31, e12 */
            other.group6.xyz, 
            /* e415, e425, e435, e321 */
            float4(other.group5.xyz, other.e45) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e423, e431, e412 */
            other.group4.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            other.group3.xyz * float3(-1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.group1.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234 */
            other.e4 * -1.0
        );
        MultiVector wedge = MultiVector(
            /* scalar, e12345 */
            float2(0.0, (this.e4235 * right_anti_dual.e1) + (this.e4315 * right_anti_dual.e2) + (this.e4125 * right_anti_dual.e3) + (this.e3215 * right_anti_dual.e4) - (this.e15 * right_anti_dual.e423) - (this.e25 * right_anti_dual.e431) - (this.e35 * right_anti_dual.e412) - (this.e45 * right_anti_dual.e321)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            float4(right_anti_dual.scalar) * this.group0, 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(((float3(right_anti_dual.e4) * this.group0.xyz) - (float3(this.e45) * right_anti_dual.group1.xyz)), 0.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            (this.group0.zxy * right_anti_dual.group1.yzx) - (this.group0.yzx * right_anti_dual.group1.zxy), 
            /* e4235, e4315, e4125, e3215 */
            float4((this.e35 * right_anti_dual.e42) + (this.e45 * right_anti_dual.e23), (this.e15 * right_anti_dual.e43) + (this.e45 * right_anti_dual.e31), (this.e25 * right_anti_dual.e41) + (this.e45 * right_anti_dual.e12), -(this.e25 * right_anti_dual.e31) - (this.e35 * right_anti_dual.e12)) + (float4(right_anti_dual.scalar) * this.group1) - (this.group0.yzxx * float4(right_anti_dual.group4.xyz.zxy, right_anti_dual.e23)), 
            /* e1234 */
            0.0
        );
        return MultiVector(
            /* scalar, e12345 */
            float2((other.scalar * wedge.e12345) + (other.e12345 * wedge.scalar) + (other.e1 * wedge.e4235) + (other.e2 * wedge.e4315) + (other.e3 * wedge.e4125) + (other.e4 * wedge.e3215) + (other.e5 * wedge.e1234) + (other.e4235 * wedge.e1) + (other.e4315 * wedge.e2) + (other.e4125 * wedge.e3) + (other.e3215 * wedge.e4) + (other.e1234 * wedge.e5) - (other.e15 * wedge.e423) - (other.e25 * wedge.e431) - (other.e35 * wedge.e412) - (other.e45 * wedge.e321) - (other.e41 * wedge.e235) - (other.e42 * wedge.e315) - (other.e43 * wedge.e125) - (other.e23 * wedge.e415) - (other.e31 * wedge.e425) - (other.e12 * wedge.e435) - (other.e415 * wedge.e23) - (other.e425 * wedge.e31) - (other.e435 * wedge.e12) - (other.e321 * wedge.e45) - (other.e423 * wedge.e15) - (other.e431 * wedge.e25) - (other.e412 * wedge.e35) - (other.e235 * wedge.e41) - (other.e315 * wedge.e42) - (other.e125 * wedge.e43), other.e12345 * wedge.e12345), 
            /* e1, e2, e3, e4 */
            float4((other.e415 * wedge.e321) + (other.e321 * wedge.e415) + (other.e4315 * wedge.e12) + (other.e1234 * wedge.e15), (other.e425 * wedge.e321) + (other.e321 * wedge.e425) + (other.e4125 * wedge.e23) + (other.e1234 * wedge.e25), (other.e435 * wedge.e321) + (other.e321 * wedge.e435) + (other.e4235 * wedge.e31) + (other.e1234 * wedge.e35), -(other.e45 * wedge.e1234) - (other.e415 * wedge.e423) - (other.e425 * wedge.e431) - (other.e435 * wedge.e412)) + (float4(other.e12345) * wedge.group1) + (float4(wedge.e12345) * other.group1) + float4((other.group4.xyz * wedge.group9.www), other.e4235 * wedge.e41) + float4((other.group5.xyz.yzx * wedge.group9.zxy), other.e4315 * wedge.e42) + float4((other.group7.xyz.zxy * wedge.group8.xyz.yzx), other.e4125 * wedge.e43) + float4((other.group8.xyz.yzx * wedge.group7.xyz.zxy), other.e1234 * wedge.e45) - (wedge.group9.yzxx * float4(other.group5.xyz.zxy, other.e41)) - float4((float3(wedge.e1234) * other.group3.xyz), other.e412 * wedge.e435) - float4((wedge.group4.xyz * other.group9.www), other.e423 * wedge.e415) - float4((other.group7.xyz.yzx * wedge.group8.xyz.zxy), other.e42 * wedge.e4315) - float4((other.group8.xyz.zxy * wedge.group7.xyz.yzx), other.e43 * wedge.e4125) - float4((wedge.group5.xyz.yzx * other.group9.zxy), other.e431 * wedge.e425), 
            /* e5 */
            (other.e12345 * wedge.e5) + (other.e5 * wedge.e12345) + (other.e15 * wedge.e4235) + (other.e25 * wedge.e4315) + (other.e35 * wedge.e4125) + (other.e45 * wedge.e3215) - (other.e415 * wedge.e235) - (other.e425 * wedge.e315) - (other.e435 * wedge.e125) - (other.e235 * wedge.e415) - (other.e315 * wedge.e425) - (other.e125 * wedge.e435) - (other.e4235 * wedge.e15) - (other.e4315 * wedge.e25) - (other.e4125 * wedge.e35) - (other.e3215 * wedge.e45), 
            /* e15, e25, e35, e45 */
            float4((other.e415 * wedge.e3215) + (other.e315 * wedge.e4125) + (other.e4125 * wedge.e315) + (other.e3215 * wedge.e415), (other.e425 * wedge.e3215) + (other.e125 * wedge.e4235) + (other.e4235 * wedge.e125) + (other.e3215 * wedge.e425), (other.e435 * wedge.e3215) + (other.e235 * wedge.e4315) + (other.e4315 * wedge.e235) + (other.e3215 * wedge.e435), -(other.e435 * wedge.e4125) - (other.e4235 * wedge.e415) - (other.e4315 * wedge.e425) - (other.e4125 * wedge.e435)) + (float4(other.e12345) * wedge.group3) + (float4(wedge.e12345) * other.group3) - (wedge.group9.yzxx * float4(other.group8.xyz.zxy, other.e415)) - float4((wedge.group8.xyz.zxy * other.group9.yzx), other.e425 * wedge.e4315), 
            /* e41, e42, e43 */
            (float3(other.e12345) * wedge.group4.xyz) + (float3(other.e1234) * wedge.group6.xyz) + (float3(wedge.e12345) * other.group4.xyz) + (float3(wedge.e1234) * other.group6.xyz) + (other.group7.xyz.zxy * wedge.group9.yzx) + (wedge.group7.xyz.zxy * other.group9.yzx) - (other.group7.xyz.yzx * wedge.group9.zxy) - (wedge.group7.xyz.yzx * other.group9.zxy), 
            /* e23, e31, e12 */
            (float3(other.e12345) * wedge.group5.xyz) + (float3(other.e3215) * wedge.group7.xyz) + (float3(other.e1234) * wedge.group8.xyz) + (float3(wedge.e12345) * other.group5.xyz) + (float3(wedge.e3215) * other.group7.xyz) + (float3(wedge.e1234) * other.group8.xyz) - (float3(other.e321) * wedge.group9.xyz) - (float3(wedge.e321) * other.group9.xyz), 
            /* e415, e425, e435, e321 */
            (float4(other.e12345) * wedge.group6) + (float4(wedge.e12345) * other.group6) + float4((other.group9.zxy * wedge.group9.yzx), other.e1234 * wedge.e3215) - (other.group9.yzxw * float4(wedge.group9.zxy, wedge.e1234)), 
            /* e423, e431, e412 */
            (float3(other.e12345) * wedge.group7.xyz) + (float3(other.e1234) * wedge.group9.xyz) + (float3(wedge.e12345) * other.group7.xyz) - (float3(wedge.e1234) * other.group9.xyz), 
            /* e235, e315, e125 */
            (float3(other.e12345) * wedge.group8.xyz) + (float3(wedge.e12345) * other.group8.xyz) + (float3(wedge.e3215) * other.group9.xyz) - (float3(other.e3215) * wedge.group9.xyz), 
            /* e4235, e4315, e4125, e3215 */
            (float4(other.e12345) * wedge.group9) + (float4(wedge.e12345) * other.group9), 
            /* e1234 */
            (other.e12345 * wedge.e1234) + (other.e1234 * wedge.e12345)
        );
    }
}
public extension Flector {
    public Flector project_orthogonally_onto(Plane other) {
        AntiPlane right_anti_dual = AntiPlane(
            /* e1, e2, e3, e5 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        Motor wedge = Motor(
            /* e415, e425, e435, e12345 */
            float4((float3(this.e45) * right_anti_dual.group0.xyz * float3(-1.0)), (right_anti_dual.e1 * this.e4235) + (right_anti_dual.e2 * this.e4315) + (right_anti_dual.e3 * this.e4125)), 
            /* e235, e315, e125, e5 */
            float4(((right_anti_dual.group0.yzx * this.group0.zxy) - (right_anti_dual.group0.zxy * this.group0.yzx)), 0.0)
        );
        return Flector(
            /* e15, e25, e35, e45 */
            float4((wedge.e415 * other.e3215) + (wedge.e315 * other.e4125), (wedge.e425 * other.e3215) + (wedge.e125 * other.e4235), (wedge.e435 * other.e3215) + (wedge.e235 * other.e4315), -(wedge.e425 * other.e4315) - (wedge.e435 * other.e4125)) - (other.group0.yzxx * float4(wedge.group1.zxy, wedge.e415)), 
            /* e4235, e4315, e4125, e3215 */
            float4(wedge.e12345) * other.group0
        );
    }
}
public extension Flector {
    public DipoleInversion project_orthogonally_onto(Sphere other) {
        RoundPoint right_anti_dual = RoundPoint(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5 */
            other.e3215
        );
        Motor wedge = Motor(
            /* e415, e425, e435, e12345 */
            float4(this.e45 * right_anti_dual.e1 * -1.0, this.e45 * right_anti_dual.e2 * -1.0, this.e45 * right_anti_dual.e3 * -1.0, (this.e4315 * right_anti_dual.e2) + (this.e4125 * right_anti_dual.e3) + (this.e3215 * right_anti_dual.e4)) + (right_anti_dual.group0.wwwx * float4(this.group0.xyz, this.e4235)), 
            /* e235, e315, e125, e5 */
            float4(((this.group0.zxy * right_anti_dual.group0.yzx) - (this.group0.yzx * right_anti_dual.group0.zxy)), 0.0)
        );
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(other.e1234) * wedge.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4((float3(other.e1234) * wedge.group1.xyz), -(wedge.e415 * other.e4235) - (wedge.e425 * other.e4315) - (wedge.e435 * other.e4125)), 
            /* e15, e25, e35, e1234 */
            float4(((float3(other.e3215) * wedge.group0.xyz) + (wedge.group1.yzx * other.group0.zxy) - (wedge.group1.zxy * other.group0.yzx)), wedge.e12345 * other.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(wedge.e12345) * other.group0
        );
    }
}
public extension Flector {
    public VersorOdd project_orthogonally_onto(VersorEven other) {
        VersorOdd right_anti_dual = VersorOdd(
            /* e41, e42, e43, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e1234 */
            float4(other.group2.xyz, other.e4) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.group3.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0)
        );
        Flector wedge = Flector(
            /* e15, e25, e35, e45 */
            float4(right_anti_dual.scalar) * this.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4((this.e45 * right_anti_dual.e23) + (this.e4235 * right_anti_dual.scalar), (this.e45 * right_anti_dual.e31) + (this.e4315 * right_anti_dual.scalar), (this.e45 * right_anti_dual.e12) + (this.e4125 * right_anti_dual.scalar), -(this.e25 * right_anti_dual.e31) - (this.e35 * right_anti_dual.e12)) + (right_anti_dual.group0.yzxw * float4(this.group0.zxy, this.e3215)) - (this.group0.yzxx * float4(right_anti_dual.group0.zxy, right_anti_dual.e23))
        );
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (wedge.group1.yzxx * float4(other.group0.zxy, other.e1)) + float4(float3(0.0), (wedge.e4315 * other.e2) + (wedge.e4125 * other.e3) + (wedge.e3215 * other.e4) - (wedge.e25 * other.e431) - (wedge.e35 * other.e412) - (wedge.e45 * other.e321)) - (other.group0.yzxx * float4(wedge.group1.zxy, wedge.e15)), 
            /* e23, e31, e12, e45 */
            (other.group0 * float4(wedge.group1.www, wedge.e45)) + float4(float3(0.0), -(wedge.e4315 * other.e425) - (wedge.e4125 * other.e435)) - (wedge.group1.xyzx * other.group1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4((float3(wedge.e3215) * other.group1.xyz), 0.0) + float4((float3(other.e12345) * wedge.group0.xyz), 0.0) + float4((wedge.group1.zxy * other.group2.yzx), 0.0) - float4((wedge.group1.yzx * other.group2.zxy), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345) * wedge.group1
        );
    }
}
public extension Flector {
    public VersorOdd project_orthogonally_onto(VersorOdd other) {
        VersorEven right_anti_dual = VersorEven(
            /* e423, e431, e412, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            float4(other.group2.xyz, other.e3215) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4(other.group3.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0)
        );
        Motor wedge = Motor(
            /* e415, e425, e435, e12345 */
            (right_anti_dual.group3.wwwx * float4(this.group0.xyz, this.e4235)) + float4(float3(0.0), (this.e4315 * right_anti_dual.e2) + (this.e4125 * right_anti_dual.e3) + (this.e3215 * right_anti_dual.e4) - (this.e25 * right_anti_dual.e431) - (this.e35 * right_anti_dual.e412) - (this.e45 * right_anti_dual.e321)) - (this.group0.wwwx * float4(right_anti_dual.group3.xyz, right_anti_dual.e423)), 
            /* e235, e315, e125, e5 */
            float4(((this.group0.zxy * right_anti_dual.group3.yzx) - (this.group0.yzx * right_anti_dual.group3.zxy)), 0.0)
        );
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (wedge.group0 * float4(other.group2.www, other.scalar)) + float4(float3(0.0), -(wedge.e415 * other.e23) - (wedge.e425 * other.e31) - (wedge.e435 * other.e12) - (wedge.e235 * other.e41) - (wedge.e315 * other.e42) - (wedge.e125 * other.e43)) + float4((wedge.group0.www * other.group0.xyz), wedge.e5 * other.e1234), 
            /* e23, e31, e12, e45 */
            float4(wedge.e235 * other.e1234, wedge.e315 * other.e1234, wedge.e125 * other.e1234, -(wedge.e415 * other.e4235) - (wedge.e425 * other.e4315) - (wedge.e435 * other.e4125)) + (float4(wedge.e12345) * other.group1), 
            /* e15, e25, e35, e1234 */
            float4(((float3(wedge.e12345) * other.group2.xyz) + (float3(other.e3215) * wedge.group0.xyz) + (wedge.group1.yzx * other.group3.zxy) - (wedge.group1.zxy * other.group3.yzx)), wedge.e12345 * other.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(wedge.e12345) * other.group3
        );
    }
}
public extension Line {
    // Fancy infix trick (first half)
    public ProjectOrthogonallyOntoInfixPartial<Line> operator /(ProjectOrthogonallyOntoInfix rhs) {
        return ProjectOrthogonallyOntoInfixPartial<Line>(this);
    }
}
public extension ProjectOrthogonallyOntoInfixPartial<Line> {
    // Fancy infix trick (second half)
    public AntiDipoleInversion operator /(AntiDipoleInversion rhs) {
        return this.a.project_orthogonally_onto(rhs);
    }
}
public extension Line {
    public AntiDipoleInversion project_orthogonally_onto(AntiDipoleInversion other) {
        DipoleInversion right_anti_dual = DipoleInversion(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e1234 */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group3 * float4(1.0, 1.0, 1.0, -1.0)
        );
        AntiScalar wedge = AntiScalar(
            /* e12345 */
            -(right_anti_dual.e41 * this.e235) - (right_anti_dual.e42 * this.e315) - (right_anti_dual.e43 * this.e125) - (right_anti_dual.e23 * this.e415) - (right_anti_dual.e31 * this.e425) - (right_anti_dual.e12 * this.e435)
        );
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(wedge.e12345) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(wedge.e12345) * other.group1, 
            /* e235, e315, e125, e4 */
            float4(wedge.e12345) * other.group2, 
            /* e1, e2, e3, e5 */
            float4(wedge.e12345) * other.group3
        );
    }
}
public extension Line {
    public Line project_orthogonally_onto(AntiScalar other) {
        Scalar right_anti_dual = Scalar(
            /* scalar */
            other.e12345 * -1.0
        );
        Line wedge = Line(
            /* e415, e425, e435 */
            float3(right_anti_dual.scalar) * this.group0.xyz, 
            /* e235, e315, e125 */
            float3(right_anti_dual.scalar) * this.group1.xyz
        );
        return Line(
            /* e415, e425, e435 */
            float3(other.e12345) * wedge.group0.xyz, 
            /* e235, e315, e125 */
            float3(other.e12345) * wedge.group1.xyz
        );
    }
}
public extension Line {
    public Circle project_orthogonally_onto(Circle other) {
        Dipole right_anti_dual = Dipole(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35 */
            other.group2.xyz
        );
        AntiScalar wedge = AntiScalar(
            /* e12345 */
            -(right_anti_dual.e41 * this.e235) - (right_anti_dual.e42 * this.e315) - (right_anti_dual.e43 * this.e125) - (right_anti_dual.e23 * this.e415) - (right_anti_dual.e31 * this.e425) - (right_anti_dual.e12 * this.e435)
        );
        return Circle(
            /* e423, e431, e412 */
            float3(wedge.e12345) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(wedge.e12345) * other.group1, 
            /* e235, e315, e125 */
            float3(wedge.e12345) * other.group2.xyz
        );
    }
}
public extension Line {
    public VersorEven project_orthogonally_onto(CircleRotor other) {
        AntiCircleRotor right_anti_dual = AntiCircleRotor(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        Motor wedge = Motor(
            /* e415, e425, e435, e12345 */
            float4((float3(right_anti_dual.scalar) * this.group0.xyz), -(right_anti_dual.e41 * this.e235) - (right_anti_dual.e42 * this.e315) - (right_anti_dual.e43 * this.e125) - (right_anti_dual.e23 * this.e415) - (right_anti_dual.e31 * this.e425) - (right_anti_dual.e12 * this.e435)), 
            /* e235, e315, e125, e5 */
            float4(float3(1.0), 0.0) * float4(this.group1.xyz, 0.0) * float4(right_anti_dual.group2.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(wedge.e12345) * float4(other.group0.xyz, other.e12345), 
            /* e415, e425, e435, e321 */
            float4(((float3(other.e12345) * wedge.group0.xyz) + (float3(wedge.e12345) * other.group1.xyz)), other.e321 * wedge.e12345), 
            /* e235, e315, e125, e5 */
            float4(other.e12345 * wedge.e235, other.e12345 * wedge.e315, other.e12345 * wedge.e125, -(other.e415 * wedge.e235) - (other.e425 * wedge.e315) - (other.e435 * wedge.e125) - (other.e235 * wedge.e415) - (other.e315 * wedge.e425) - (other.e125 * wedge.e435)) + (other.group2 * float4(wedge.group0.www, wedge.e5)), 
            /* e1, e2, e3, e4 */
            float4((other.e412 * wedge.e315) + (other.e321 * wedge.e415), (other.e423 * wedge.e125) + (other.e321 * wedge.e425), (other.e431 * wedge.e235) + (other.e321 * wedge.e435), -(other.e431 * wedge.e425) - (other.e412 * wedge.e435)) - float4((other.group0.xyz.yzx * wedge.group1.zxy), other.e423 * wedge.e415)
        );
    }
}
public extension Line {
    public AntiDipoleInversion project_orthogonally_onto(DipoleInversion other) {
        AntiDipoleInversion right_anti_dual = AntiDipoleInversion(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e4 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group3 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        Plane wedge = Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(-(right_anti_dual.e4 * this.e235) - (right_anti_dual.e3 * this.e425), -(right_anti_dual.e4 * this.e315) - (right_anti_dual.e1 * this.e435), -(right_anti_dual.e4 * this.e125) - (right_anti_dual.e2 * this.e415), (right_anti_dual.e2 * this.e315) + (right_anti_dual.e3 * this.e125)) + (right_anti_dual.group3.yzxx * float4(this.group0.xyz.zxy, this.e235))
        );
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(other.e1234) * wedge.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(((other.group3.zxy * wedge.group0.yzx) - (other.group3.yzx * wedge.group0.zxy)), other.e1234 * wedge.e3215), 
            /* e235, e315, e125, e4 */
            float4(other.e4235 * wedge.e3215, other.e4315 * wedge.e3215, other.e4125 * wedge.e3215, -(other.e42 * wedge.e4315) - (other.e43 * wedge.e4125)) - (wedge.group0.xyzx * float4(other.group3.www, other.e41)), 
            /* e1, e2, e3, e5 */
            float4(other.e12 * wedge.e4315 * -1.0, other.e23 * wedge.e4125 * -1.0, other.e31 * wedge.e4235 * -1.0, (other.e25 * wedge.e4315) + (other.e35 * wedge.e4125)) + (float4(wedge.e3215) * float4(other.group0.xyz, other.e45)) + (wedge.group0.zxyx * float4(other.group1.yzx, other.e15))
        );
    }
}
public extension Line {
    public Line project_orthogonally_onto(DualNum other) {
        AntiDualNum right_anti_dual = AntiDualNum(
            /* e3215, scalar */
            other.group0.xy * float2(-1.0)
        );
        Line wedge = Line(
            /* e415, e425, e435 */
            float3(right_anti_dual.scalar) * this.group0.xyz, 
            /* e235, e315, e125 */
            float3(right_anti_dual.scalar) * this.group1.xyz
        );
        return Line(
            /* e415, e425, e435 */
            float3(other.e12345) * wedge.group0.xyz, 
            /* e235, e315, e125 */
            float3(other.e12345) * wedge.group1.xyz
        );
    }
}
public extension Line {
    public Motor project_orthogonally_onto(Flector other) {
        AntiFlector right_anti_dual = AntiFlector(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        Plane wedge = Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual.e3 * this.e425 * -1.0, right_anti_dual.e1 * this.e435 * -1.0, right_anti_dual.e2 * this.e415 * -1.0, (right_anti_dual.e2 * this.e315) + (right_anti_dual.e3 * this.e125)) + (right_anti_dual.group1.yzxx * float4(this.group0.xyz.zxy, this.e235))
        );
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(((other.group1.zxy * wedge.group0.yzx) - (other.group1.yzx * wedge.group0.zxy)), 0.0), 
            /* e235, e315, e125, e5 */
            float4(other.e3215 * wedge.e4235 * -1.0, other.e3215 * wedge.e4315 * -1.0, other.e3215 * wedge.e4125 * -1.0, (other.e25 * wedge.e4315) + (other.e35 * wedge.e4125) + (other.e45 * wedge.e3215)) + (wedge.group0.wwwx * float4(other.group1.xyz, other.e15))
        );
    }
}
public extension Line {
    public Line project_orthogonally_onto(Line other) {
        AntiLine right_anti_dual = AntiLine(
            /* e23, e31, e12 */
            other.group0.xyz, 
            /* e15, e25, e35 */
            other.group1.xyz
        );
        AntiScalar wedge = AntiScalar(
            /* e12345 */
            -(right_anti_dual.e23 * this.e415) - (right_anti_dual.e31 * this.e425) - (right_anti_dual.e12 * this.e435)
        );
        return Line(
            /* e415, e425, e435 */
            float3(wedge.e12345) * other.group0.xyz, 
            /* e235, e315, e125 */
            float3(wedge.e12345) * other.group1.xyz
        );
    }
}
public extension Line {
    public Motor project_orthogonally_onto(Motor other) {
        AntiMotor right_anti_dual = AntiMotor(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        Motor wedge = Motor(
            /* e415, e425, e435, e12345 */
            float4((float3(right_anti_dual.scalar) * this.group0.xyz), -(right_anti_dual.e23 * this.e415) - (right_anti_dual.e31 * this.e425) - (right_anti_dual.e12 * this.e435)), 
            /* e235, e315, e125, e5 */
            float4(float3(1.0), 0.0) * float4(this.group1.xyz, 0.0) * float4(right_anti_dual.group0.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(((float3(other.e12345) * wedge.group0.xyz) + (float3(wedge.e12345) * other.group0.xyz)), other.e12345 * wedge.e12345), 
            /* e235, e315, e125, e5 */
            (float4(other.e12345) * wedge.group1) + (float4(wedge.e12345) * other.group1) + float4(float3(0.0), -(other.e415 * wedge.e235) - (other.e425 * wedge.e315) - (other.e435 * wedge.e125) - (other.e235 * wedge.e415) - (other.e315 * wedge.e425) - (other.e125 * wedge.e435))
        );
    }
}
public extension Line {
    public MultiVector project_orthogonally_onto(MultiVector other) {
        MultiVector right_anti_dual = MultiVector(
            /* scalar, e12345 */
            other.group0.xy.yx * float2(-1.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4(other.group9.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5 */
            other.e3215, 
            /* e15, e25, e35, e45 */
            float4(other.group8.xyz, other.e321 * -1.0), 
            /* e41, e42, e43 */
            other.group7.xyz, 
            /* e23, e31, e12 */
            other.group6.xyz, 
            /* e415, e425, e435, e321 */
            float4(other.group5.xyz, other.e45) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e423, e431, e412 */
            other.group4.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            other.group3.xyz * float3(-1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.group1.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234 */
            other.e4 * -1.0
        );
        MultiVector wedge = MultiVector(
            /* scalar, e12345 */
            float2(0.0, -(this.e415 * right_anti_dual.e23) - (this.e425 * right_anti_dual.e31) - (this.e435 * right_anti_dual.e12) - (this.e235 * right_anti_dual.e41) - (this.e315 * right_anti_dual.e42) - (this.e125 * right_anti_dual.e43)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(right_anti_dual.group0.xy.xx, right_anti_dual.scalar, 0.0) * float4(float3(1.0), 0.0) * float4(this.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(right_anti_dual.scalar) * this.group1.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(-(this.e425 * right_anti_dual.e3) - (this.e235 * right_anti_dual.e4), -(this.e435 * right_anti_dual.e1) - (this.e315 * right_anti_dual.e4), -(this.e415 * right_anti_dual.e2) - (this.e125 * right_anti_dual.e4), (this.e315 * right_anti_dual.e2) + (this.e125 * right_anti_dual.e3)) + (right_anti_dual.group1.yzxx * float4(this.group0.xyz.zxy, this.e235)), 
            /* e1234 */
            0.0
        );
        return MultiVector(
            /* scalar, e12345 */
            float2((other.scalar * wedge.e12345) + (other.e12345 * wedge.scalar) + (other.e1 * wedge.e4235) + (other.e2 * wedge.e4315) + (other.e3 * wedge.e4125) + (other.e4 * wedge.e3215) + (other.e5 * wedge.e1234) + (other.e4235 * wedge.e1) + (other.e4315 * wedge.e2) + (other.e4125 * wedge.e3) + (other.e3215 * wedge.e4) + (other.e1234 * wedge.e5) - (other.e15 * wedge.e423) - (other.e25 * wedge.e431) - (other.e35 * wedge.e412) - (other.e45 * wedge.e321) - (other.e41 * wedge.e235) - (other.e42 * wedge.e315) - (other.e43 * wedge.e125) - (other.e23 * wedge.e415) - (other.e31 * wedge.e425) - (other.e12 * wedge.e435) - (other.e415 * wedge.e23) - (other.e425 * wedge.e31) - (other.e435 * wedge.e12) - (other.e321 * wedge.e45) - (other.e423 * wedge.e15) - (other.e431 * wedge.e25) - (other.e412 * wedge.e35) - (other.e235 * wedge.e41) - (other.e315 * wedge.e42) - (other.e125 * wedge.e43), other.e12345 * wedge.e12345), 
            /* e1, e2, e3, e4 */
            float4((other.e415 * wedge.e321) + (other.e321 * wedge.e415) + (other.e4315 * wedge.e12) + (other.e1234 * wedge.e15), (other.e425 * wedge.e321) + (other.e321 * wedge.e425) + (other.e4125 * wedge.e23) + (other.e1234 * wedge.e25), (other.e435 * wedge.e321) + (other.e321 * wedge.e435) + (other.e4235 * wedge.e31) + (other.e1234 * wedge.e35), -(other.e45 * wedge.e1234) - (other.e415 * wedge.e423) - (other.e425 * wedge.e431) - (other.e435 * wedge.e412)) + (float4(other.e12345) * wedge.group1) + (float4(wedge.e12345) * other.group1) + float4((other.group4.xyz * wedge.group9.www), other.e4235 * wedge.e41) + float4((other.group5.xyz.yzx * wedge.group9.zxy), other.e4315 * wedge.e42) + float4((other.group7.xyz.zxy * wedge.group8.xyz.yzx), other.e4125 * wedge.e43) + float4((other.group8.xyz.yzx * wedge.group7.xyz.zxy), other.e1234 * wedge.e45) - (wedge.group9.yzxx * float4(other.group5.xyz.zxy, other.e41)) - float4((float3(wedge.e1234) * other.group3.xyz), other.e412 * wedge.e435) - float4((wedge.group4.xyz * other.group9.www), other.e423 * wedge.e415) - float4((other.group7.xyz.yzx * wedge.group8.xyz.zxy), other.e42 * wedge.e4315) - float4((other.group8.xyz.zxy * wedge.group7.xyz.yzx), other.e43 * wedge.e4125) - float4((wedge.group5.xyz.yzx * other.group9.zxy), other.e431 * wedge.e425), 
            /* e5 */
            (other.e12345 * wedge.e5) + (other.e5 * wedge.e12345) + (other.e15 * wedge.e4235) + (other.e25 * wedge.e4315) + (other.e35 * wedge.e4125) + (other.e45 * wedge.e3215) - (other.e415 * wedge.e235) - (other.e425 * wedge.e315) - (other.e435 * wedge.e125) - (other.e235 * wedge.e415) - (other.e315 * wedge.e425) - (other.e125 * wedge.e435) - (other.e4235 * wedge.e15) - (other.e4315 * wedge.e25) - (other.e4125 * wedge.e35) - (other.e3215 * wedge.e45), 
            /* e15, e25, e35, e45 */
            float4((other.e415 * wedge.e3215) + (other.e315 * wedge.e4125) + (other.e4125 * wedge.e315) + (other.e3215 * wedge.e415), (other.e425 * wedge.e3215) + (other.e125 * wedge.e4235) + (other.e4235 * wedge.e125) + (other.e3215 * wedge.e425), (other.e435 * wedge.e3215) + (other.e235 * wedge.e4315) + (other.e4315 * wedge.e235) + (other.e3215 * wedge.e435), -(other.e435 * wedge.e4125) - (other.e4235 * wedge.e415) - (other.e4315 * wedge.e425) - (other.e4125 * wedge.e435)) + (float4(other.e12345) * wedge.group3) + (float4(wedge.e12345) * other.group3) - (wedge.group9.yzxx * float4(other.group8.xyz.zxy, other.e415)) - float4((wedge.group8.xyz.zxy * other.group9.yzx), other.e425 * wedge.e4315), 
            /* e41, e42, e43 */
            (float3(other.e12345) * wedge.group4.xyz) + (float3(other.e1234) * wedge.group6.xyz) + (float3(wedge.e12345) * other.group4.xyz) + (float3(wedge.e1234) * other.group6.xyz) + (other.group7.xyz.zxy * wedge.group9.yzx) + (wedge.group7.xyz.zxy * other.group9.yzx) - (other.group7.xyz.yzx * wedge.group9.zxy) - (wedge.group7.xyz.yzx * other.group9.zxy), 
            /* e23, e31, e12 */
            (float3(other.e12345) * wedge.group5.xyz) + (float3(other.e3215) * wedge.group7.xyz) + (float3(other.e1234) * wedge.group8.xyz) + (float3(wedge.e12345) * other.group5.xyz) + (float3(wedge.e3215) * other.group7.xyz) + (float3(wedge.e1234) * other.group8.xyz) - (float3(other.e321) * wedge.group9.xyz) - (float3(wedge.e321) * other.group9.xyz), 
            /* e415, e425, e435, e321 */
            (float4(other.e12345) * wedge.group6) + (float4(wedge.e12345) * other.group6) + float4((other.group9.zxy * wedge.group9.yzx), other.e1234 * wedge.e3215) - (other.group9.yzxw * float4(wedge.group9.zxy, wedge.e1234)), 
            /* e423, e431, e412 */
            (float3(other.e12345) * wedge.group7.xyz) + (float3(other.e1234) * wedge.group9.xyz) + (float3(wedge.e12345) * other.group7.xyz) - (float3(wedge.e1234) * other.group9.xyz), 
            /* e235, e315, e125 */
            (float3(other.e12345) * wedge.group8.xyz) + (float3(wedge.e12345) * other.group8.xyz) + (float3(wedge.e3215) * other.group9.xyz) - (float3(other.e3215) * wedge.group9.xyz), 
            /* e4235, e4315, e4125, e3215 */
            (float4(other.e12345) * wedge.group9) + (float4(wedge.e12345) * other.group9), 
            /* e1234 */
            (other.e12345 * wedge.e1234) + (other.e1234 * wedge.e12345)
        );
    }
}
public extension Line {
    public Line project_orthogonally_onto(Plane other) {
        AntiPlane right_anti_dual = AntiPlane(
            /* e1, e2, e3, e5 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        Plane wedge = Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual.e3 * this.e425 * -1.0, right_anti_dual.e1 * this.e435 * -1.0, right_anti_dual.e2 * this.e415 * -1.0, (right_anti_dual.e2 * this.e315) + (right_anti_dual.e3 * this.e125)) + (right_anti_dual.group0.yzxx * float4(this.group0.xyz.zxy, this.e235))
        );
        return Line(
            /* e415, e425, e435 */
            (other.group0.zxy * wedge.group0.yzx) - (other.group0.yzx * wedge.group0.zxy), 
            /* e235, e315, e125 */
            (float3(wedge.e3215) * other.group0.xyz) - (float3(other.e3215) * wedge.group0.xyz)
        );
    }
}
public extension Line {
    public Circle project_orthogonally_onto(Sphere other) {
        RoundPoint right_anti_dual = RoundPoint(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5 */
            other.e3215
        );
        Plane wedge = Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(-(this.e425 * right_anti_dual.e3) - (this.e235 * right_anti_dual.e4), -(this.e435 * right_anti_dual.e1) - (this.e315 * right_anti_dual.e4), -(this.e415 * right_anti_dual.e2) - (this.e125 * right_anti_dual.e4), (this.e315 * right_anti_dual.e2) + (this.e125 * right_anti_dual.e3)) + (right_anti_dual.group0.yzxx * float4(this.group0.xyz.zxy, this.e235))
        );
        return Circle(
            /* e423, e431, e412 */
            float3(other.e1234) * wedge.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(((wedge.group0.yzx * other.group0.zxy) - (wedge.group0.zxy * other.group0.yzx)), wedge.e3215 * other.e1234), 
            /* e235, e315, e125 */
            (float3(wedge.e3215) * other.group0.xyz) - (float3(other.e3215) * wedge.group0.xyz)
        );
    }
}
public extension Line {
    public VersorEven project_orthogonally_onto(VersorEven other) {
        VersorOdd right_anti_dual = VersorOdd(
            /* e41, e42, e43, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e1234 */
            float4(other.group2.xyz, other.e4) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.group3.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0)
        );
        Motor wedge = Motor(
            /* e415, e425, e435, e12345 */
            float4((float3(right_anti_dual.scalar) * this.group0.xyz), -(this.e415 * right_anti_dual.e23) - (this.e425 * right_anti_dual.e31) - (this.e435 * right_anti_dual.e12) - (this.e235 * right_anti_dual.e41) - (this.e315 * right_anti_dual.e42) - (this.e125 * right_anti_dual.e43)), 
            /* e235, e315, e125, e5 */
            float4(float3(1.0), 0.0) * float4(this.group1.xyz, 0.0) * float4(right_anti_dual.group0.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(wedge.e12345) * other.group0, 
            /* e415, e425, e435, e321 */
            float4(((float3(wedge.e12345) * other.group1.xyz) + (float3(other.e12345) * wedge.group0.xyz)), wedge.e12345 * other.e321), 
            /* e235, e315, e125, e5 */
            (float4(wedge.e12345) * other.group2) + (float4(other.e12345) * wedge.group1) + float4(float3(0.0), -(wedge.e415 * other.e235) - (wedge.e425 * other.e315) - (wedge.e435 * other.e125) - (wedge.e235 * other.e415) - (wedge.e315 * other.e425) - (wedge.e125 * other.e435)), 
            /* e1, e2, e3, e4 */
            float4((wedge.e12345 * other.e1) + (wedge.e315 * other.e412), (wedge.e12345 * other.e2) + (wedge.e125 * other.e423), (wedge.e12345 * other.e3) + (wedge.e235 * other.e431), -(wedge.e425 * other.e431) - (wedge.e435 * other.e412)) + (wedge.group0 * float4(other.group1.www, other.e4)) - (other.group0.yzxx * float4(wedge.group1.zxy, wedge.e415))
        );
    }
}
public extension Line {
    public AntiDipoleInversion project_orthogonally_onto(VersorOdd other) {
        VersorEven right_anti_dual = VersorEven(
            /* e423, e431, e412, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            float4(other.group2.xyz, other.e3215) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4(other.group3.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0)
        );
        Plane wedge = Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(-(this.e425 * right_anti_dual.e3) - (this.e235 * right_anti_dual.e4), -(this.e435 * right_anti_dual.e1) - (this.e315 * right_anti_dual.e4), -(this.e415 * right_anti_dual.e2) - (this.e125 * right_anti_dual.e4), (this.e315 * right_anti_dual.e2) + (this.e125 * right_anti_dual.e3)) + (right_anti_dual.group3.yzxx * float4(this.group0.xyz.zxy, this.e235))
        );
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(other.e1234) * wedge.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(((wedge.group0.yzx * other.group3.zxy) - (wedge.group0.zxy * other.group3.yzx)), wedge.e3215 * other.e1234), 
            /* e235, e315, e125, e4 */
            float4(wedge.e3215 * other.e4235, wedge.e3215 * other.e4315, wedge.e3215 * other.e4125, -(wedge.e4315 * other.e42) - (wedge.e4125 * other.e43)) - (wedge.group0.xyzx * float4(other.group3.www, other.e41)), 
            /* e1, e2, e3, e5 */
            float4(wedge.e4315 * other.e12 * -1.0, wedge.e4125 * other.e23 * -1.0, wedge.e4235 * other.e31 * -1.0, (wedge.e4125 * other.e35) + (wedge.e3215 * other.e45)) + (wedge.group0.zxyx * float4(other.group1.yzx, other.e15)) + (wedge.group0.wwwy * float4(other.group0.xyz, other.e25))
        );
    }
}
public extension Motor {
    // Fancy infix trick (first half)
    public ProjectOrthogonallyOntoInfixPartial<Motor> operator /(ProjectOrthogonallyOntoInfix rhs) {
        return ProjectOrthogonallyOntoInfixPartial<Motor>(this);
    }
}
public extension ProjectOrthogonallyOntoInfixPartial<Motor> {
    // Fancy infix trick (second half)
    public RoundPoint operator /(AntiCircleRotor rhs) {
        return this.a.project_orthogonally_onto(rhs);
    }
}
public extension Motor {
    public RoundPoint project_orthogonally_onto(AntiCircleRotor other) {
        CircleRotor right_anti_dual = CircleRotor(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        Plane wedge = Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(this.e5) * float4(right_anti_dual.group0.xyz, right_anti_dual.e321) * float4(-1.0)
        );
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4((other.e41 * wedge.e3215) + (other.e31 * wedge.e4125), (other.e42 * wedge.e3215) + (other.e12 * wedge.e4235), (other.e43 * wedge.e3215) + (other.e23 * wedge.e4315), -(other.e42 * wedge.e4315) - (other.e43 * wedge.e4125)) - (wedge.group0.yzxx * float4(other.group1.zxy, other.e41)), 
            /* e5 */
            (other.e45 * wedge.e3215) + (other.e15 * wedge.e4235) + (other.e25 * wedge.e4315) + (other.e35 * wedge.e4125)
        );
    }
}
public extension Motor {
    public AntiDipoleInversion project_orthogonally_onto(AntiDipoleInversion other) {
        DipoleInversion right_anti_dual = DipoleInversion(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e1234 */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group3 * float4(1.0, 1.0, 1.0, -1.0)
        );
        Motor wedge = Motor(
            /* e415, e425, e435, e12345 */
            float4((float3(this.e5) * right_anti_dual.group0.xyz), (right_anti_dual.e1234 * this.e5) - (right_anti_dual.e41 * this.e235) - (right_anti_dual.e42 * this.e315) - (right_anti_dual.e43 * this.e125) - (right_anti_dual.e23 * this.e415) - (right_anti_dual.e31 * this.e425) - (right_anti_dual.e12 * this.e435)), 
            /* e235, e315, e125, e5 */
            float4(float3(1.0), 0.0) * float4(this.group1.www, 0.0) * float4(right_anti_dual.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(wedge.e12345) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(wedge.e12345) * other.group1, 
            /* e235, e315, e125, e4 */
            float4((float3(wedge.e12345) * other.group2.xyz), (other.e4 * wedge.e12345) - (other.e423 * wedge.e415) - (other.e431 * wedge.e425) - (other.e412 * wedge.e435)), 
            /* e1, e2, e3, e5 */
            float4((other.e321 * wedge.e415) + (other.e1 * wedge.e12345), (other.e321 * wedge.e425) + (other.e2 * wedge.e12345), (other.e321 * wedge.e435) + (other.e3 * wedge.e12345), -(other.e425 * wedge.e315) - (other.e435 * wedge.e125) - (other.e235 * wedge.e415) - (other.e315 * wedge.e425) - (other.e125 * wedge.e435)) + float4((other.group0.xyz.zxy * wedge.group1.yzx), other.e5 * wedge.e12345) - (wedge.group1.zxyx * float4(other.group0.xyz.yzx, other.e415))
        );
    }
}
public extension Motor {
    public Motor project_orthogonally_onto(AntiScalar other) {
        Scalar right_anti_dual = Scalar(
            /* scalar */
            other.e12345 * -1.0
        );
        Motor wedge = Motor(
            /* e415, e425, e435, e12345 */
            float4(right_anti_dual.scalar) * this.group0, 
            /* e235, e315, e125, e5 */
            float4(right_anti_dual.scalar) * this.group1
        );
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(other.e12345) * wedge.group0, 
            /* e235, e315, e125, e5 */
            float4(other.e12345) * wedge.group1
        );
    }
}
public extension Motor {
    public AntiDipoleInversion project_orthogonally_onto(Circle other) {
        Dipole right_anti_dual = Dipole(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35 */
            other.group2.xyz
        );
        Motor wedge = Motor(
            /* e415, e425, e435, e12345 */
            float4((float3(this.e5) * right_anti_dual.group0.xyz), -(right_anti_dual.e41 * this.e235) - (right_anti_dual.e42 * this.e315) - (right_anti_dual.e43 * this.e125) - (right_anti_dual.e23 * this.e415) - (right_anti_dual.e31 * this.e425) - (right_anti_dual.e12 * this.e435)), 
            /* e235, e315, e125, e5 */
            float4(float3(1.0), 0.0) * float4(this.group1.www, 0.0) * float4(right_anti_dual.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(wedge.e12345) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(wedge.e12345) * other.group1, 
            /* e235, e315, e125, e4 */
            float4((float3(wedge.e12345) * other.group2.xyz), -(other.e423 * wedge.e415) - (other.e431 * wedge.e425) - (other.e412 * wedge.e435)), 
            /* e1, e2, e3, e5 */
            float4((other.e412 * wedge.e315) + (other.e321 * wedge.e415), (other.e423 * wedge.e125) + (other.e321 * wedge.e425), (other.e431 * wedge.e235) + (other.e321 * wedge.e435), -(other.e415 * wedge.e235) - (other.e425 * wedge.e315) - (other.e435 * wedge.e125) - (other.e315 * wedge.e425) - (other.e125 * wedge.e435)) - float4((other.group0.xyz.yzx * wedge.group1.zxy), other.e235 * wedge.e415)
        );
    }
}
public extension Motor {
    public VersorEven project_orthogonally_onto(CircleRotor other) {
        AntiCircleRotor right_anti_dual = AntiCircleRotor(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        Motor wedge = Motor(
            /* e415, e425, e435, e12345 */
            float4(right_anti_dual.scalar * this.e415, right_anti_dual.scalar * this.e425, right_anti_dual.scalar * this.e435, -(right_anti_dual.e41 * this.e235) - (right_anti_dual.e42 * this.e315) - (right_anti_dual.e43 * this.e125) - (right_anti_dual.e23 * this.e415) - (right_anti_dual.e31 * this.e425) - (right_anti_dual.e12 * this.e435)) + float4((right_anti_dual.group0.xyz * this.group1.www), right_anti_dual.scalar * this.e12345), 
            /* e235, e315, e125, e5 */
            float4(((float3(right_anti_dual.scalar) * this.group1.xyz) + (float3(this.e5) * right_anti_dual.group1.xyz)), right_anti_dual.scalar * this.e5)
        );
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(wedge.e12345) * float4(other.group0.xyz, other.e12345), 
            /* e415, e425, e435, e321 */
            float4(((float3(other.e12345) * wedge.group0.xyz) + (float3(wedge.e12345) * other.group1.xyz)), other.e321 * wedge.e12345), 
            /* e235, e315, e125, e5 */
            float4(other.e12345 * wedge.e235, other.e12345 * wedge.e315, other.e12345 * wedge.e125, -(other.e415 * wedge.e235) - (other.e425 * wedge.e315) - (other.e435 * wedge.e125) - (other.e235 * wedge.e415) - (other.e315 * wedge.e425) - (other.e125 * wedge.e435)) + (other.group2 * float4(wedge.group0.www, wedge.e5)), 
            /* e1, e2, e3, e4 */
            float4((other.e412 * wedge.e315) + (other.e321 * wedge.e415), (other.e423 * wedge.e125) + (other.e321 * wedge.e425), (other.e431 * wedge.e235) + (other.e321 * wedge.e435), -(other.e431 * wedge.e425) - (other.e412 * wedge.e435)) - float4((other.group0.xyz.yzx * wedge.group1.zxy), other.e423 * wedge.e415)
        );
    }
}
public extension Motor {
    public RoundPoint project_orthogonally_onto(Dipole other) {
        Circle right_anti_dual = Circle(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125 */
            other.group2.xyz * float3(-1.0)
        );
        Plane wedge = Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(this.e5) * float4(right_anti_dual.group0.xyz, right_anti_dual.e321) * float4(-1.0)
        );
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4((other.e41 * wedge.e3215) + (other.e31 * wedge.e4125), (other.e42 * wedge.e3215) + (other.e12 * wedge.e4235), (other.e43 * wedge.e3215) + (other.e23 * wedge.e4315), -(other.e42 * wedge.e4315) - (other.e43 * wedge.e4125)) - (wedge.group0.yzxx * float4(other.group1.zxy, other.e41)), 
            /* e5 */
            (other.e45 * wedge.e3215) + (other.e15 * wedge.e4235) + (other.e25 * wedge.e4315) + (other.e35 * wedge.e4125)
        );
    }
}
public extension Motor {
    public AntiDipoleInversion project_orthogonally_onto(DipoleInversion other) {
        AntiDipoleInversion right_anti_dual = AntiDipoleInversion(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e4 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group3 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        Flector wedge = Flector(
            /* e15, e25, e35, e45 */
            float4(this.e5) * float4(right_anti_dual.group3.xyz, right_anti_dual.e4) * float4(-1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(-(right_anti_dual.e4 * this.e235) - (right_anti_dual.e3 * this.e425), -(right_anti_dual.e4 * this.e315) - (right_anti_dual.e1 * this.e435), -(right_anti_dual.e4 * this.e125) - (right_anti_dual.e2 * this.e415), (right_anti_dual.e2 * this.e315) + (right_anti_dual.e3 * this.e125)) + (right_anti_dual.group3.yzxx * float4(this.group0.zxy, this.e235)) - (float4(this.e5) * float4(right_anti_dual.group0.xyz, right_anti_dual.e321))
        );
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(other.e1234) * wedge.group1.xyz, 
            /* e415, e425, e435, e321 */
            float4(((other.group3.zxy * wedge.group1.yzx) - (other.group3.yzx * wedge.group1.zxy)), other.e1234 * wedge.e3215), 
            /* e235, e315, e125, e4 */
            float4(float3(0.0), -(other.e42 * wedge.e4315) - (other.e43 * wedge.e4125)) + float4((wedge.group1.www * other.group3.xyz), other.e1234 * wedge.e45) - (wedge.group1.xyzx * float4(other.group3.www, other.e41)), 
            /* e1, e2, e3, e5 */
            (float4(wedge.e3215) * float4(other.group0.xyz, other.e45)) + (other.group2.wwwy * float4(wedge.group0.xyz, wedge.e4315)) + (wedge.group1.zxyx * float4(other.group1.yzx, other.e15)) + float4(float3(0.0), (other.e35 * wedge.e4125) - (other.e4315 * wedge.e25) - (other.e4125 * wedge.e35) - (other.e3215 * wedge.e45)) - float4((other.group1.zxy * wedge.group1.yzx), other.e4235 * wedge.e15)
        );
    }
}
public extension Motor {
    public Motor project_orthogonally_onto(DualNum other) {
        AntiDualNum right_anti_dual = AntiDualNum(
            /* e3215, scalar */
            other.group0.xy * float2(-1.0)
        );
        Motor wedge = Motor(
            /* e415, e425, e435, e12345 */
            float4(right_anti_dual.scalar) * this.group0, 
            /* e235, e315, e125, e5 */
            float4(right_anti_dual.scalar) * this.group1
        );
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(other.e12345) * wedge.group0, 
            /* e235, e315, e125, e5 */
            float4(other.group0.xy.yy, other.e12345, (other.e5 * wedge.e12345) + (other.e12345 * wedge.e5)) * float4(wedge.group1.xyz, 1.0)
        );
    }
}
public extension Motor {
    public DualNum project_orthogonally_onto(FlatPoint other) {
        return DualNum(
            /* e5, e12345 */
            float2(pow(other.e45, 2) * this.e5, 1.0) * float2(-1.0, 0.0)
        );
    }
}
public extension Motor {
    public Motor project_orthogonally_onto(Flector other) {
        AntiFlector right_anti_dual = AntiFlector(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        Flector wedge = Flector(
            /* e15, e25, e35, e45 */
            float4(float3(1.0), 0.0) * float4(this.group1.www, 0.0) * float4(right_anti_dual.group1.xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            (right_anti_dual.group1.yzxx * float4(this.group0.zxy, this.e235)) + float4(float3(0.0), (right_anti_dual.e2 * this.e315) + (right_anti_dual.e3 * this.e125)) - float4((right_anti_dual.group1.zxy * this.group0.yzx), right_anti_dual.e321 * this.e5)
        );
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(((other.group1.zxy * wedge.group1.yzx) - (other.group1.yzx * wedge.group1.zxy)), 0.0), 
            /* e235, e315, e125, e5 */
            (wedge.group1.wwwx * float4(other.group1.xyz, other.e15)) + float4(float3(0.0), (other.e25 * wedge.e4315) + (other.e35 * wedge.e4125) + (other.e45 * wedge.e3215) - (other.e4315 * wedge.e25) - (other.e4125 * wedge.e35) - (other.e3215 * wedge.e45)) - (other.group1.wwwx * float4(wedge.group1.xyz, wedge.e15))
        );
    }
}
public extension Motor {
    public Motor project_orthogonally_onto(Line other) {
        AntiLine right_anti_dual = AntiLine(
            /* e23, e31, e12 */
            other.group0.xyz, 
            /* e15, e25, e35 */
            other.group1.xyz
        );
        Motor wedge = Motor(
            /* e415, e425, e435, e12345 */
            float4(float3(0.0), -(right_anti_dual.e23 * this.e415) - (right_anti_dual.e31 * this.e425) - (right_anti_dual.e12 * this.e435)), 
            /* e235, e315, e125, e5 */
            float4(float3(1.0), 0.0) * float4(right_anti_dual.group0.xyz, 0.0) * float4(this.group1.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(float3(1.0), 0.0) * float4(other.group0.xyz, 0.0) * float4(wedge.group0.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e235, e315, e125, e5 */
            float4((float3(wedge.e12345) * other.group1.xyz), -(other.e415 * wedge.e235) - (other.e425 * wedge.e315) - (other.e435 * wedge.e125) - (other.e235 * wedge.e415) - (other.e315 * wedge.e425) - (other.e125 * wedge.e435))
        );
    }
}
public extension Motor {
    public Motor project_orthogonally_onto(Motor other) {
        AntiMotor right_anti_dual = AntiMotor(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        Motor wedge = Motor(
            /* e415, e425, e435, e12345 */
            float4((right_anti_dual.group0.www * this.group0.xyz), (right_anti_dual.scalar * this.e12345) - (right_anti_dual.e23 * this.e415) - (right_anti_dual.e31 * this.e425) - (right_anti_dual.e12 * this.e435)), 
            /* e235, e315, e125, e5 */
            float4(((float3(right_anti_dual.scalar) * this.group1.xyz) + (float3(this.e5) * right_anti_dual.group0.xyz)), right_anti_dual.scalar * this.e5)
        );
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(((float3(other.e12345) * wedge.group0.xyz) + (float3(wedge.e12345) * other.group0.xyz)), other.e12345 * wedge.e12345), 
            /* e235, e315, e125, e5 */
            (float4(other.e12345) * wedge.group1) + (float4(wedge.e12345) * other.group1) + float4(float3(0.0), -(other.e415 * wedge.e235) - (other.e425 * wedge.e315) - (other.e435 * wedge.e125) - (other.e235 * wedge.e415) - (other.e315 * wedge.e425) - (other.e125 * wedge.e435))
        );
    }
}
public extension Motor {
    public MultiVector project_orthogonally_onto(MultiVector other) {
        MultiVector right_anti_dual = MultiVector(
            /* scalar, e12345 */
            other.group0.xy.yx * float2(-1.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4(other.group9.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5 */
            other.e3215, 
            /* e15, e25, e35, e45 */
            float4(other.group8.xyz, other.e321 * -1.0), 
            /* e41, e42, e43 */
            other.group7.xyz, 
            /* e23, e31, e12 */
            other.group6.xyz, 
            /* e415, e425, e435, e321 */
            float4(other.group5.xyz, other.e45) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e423, e431, e412 */
            other.group4.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            other.group3.xyz * float3(-1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.group1.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234 */
            other.e4 * -1.0
        );
        MultiVector wedge = MultiVector(
            /* scalar, e12345 */
            float2(0.0, (this.e12345 * right_anti_dual.scalar) + (this.e5 * right_anti_dual.e1234) - (this.e415 * right_anti_dual.e23) - (this.e425 * right_anti_dual.e31) - (this.e435 * right_anti_dual.e12) - (this.e235 * right_anti_dual.e41) - (this.e315 * right_anti_dual.e42) - (this.e125 * right_anti_dual.e43)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            this.e5 * right_anti_dual.scalar, 
            /* e15, e25, e35, e45 */
            float4(this.e5) * right_anti_dual.group1 * float4(-1.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(((float3(this.e5) * right_anti_dual.group4.xyz) + (float3(right_anti_dual.scalar) * this.group0.xyz)), 0.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            (float3(this.e5) * right_anti_dual.group5.xyz) + (float3(right_anti_dual.scalar) * this.group1.xyz), 
            /* e4235, e4315, e4125, e3215 */
            float4(-(this.e425 * right_anti_dual.e3) - (this.e235 * right_anti_dual.e4), -(this.e435 * right_anti_dual.e1) - (this.e315 * right_anti_dual.e4), -(this.e415 * right_anti_dual.e2) - (this.e125 * right_anti_dual.e4), (this.e315 * right_anti_dual.e2) + (this.e125 * right_anti_dual.e3)) + (right_anti_dual.group1.yzxx * float4(this.group0.zxy, this.e235)) - float4((right_anti_dual.group7.xyz * this.group1.www), this.e5 * right_anti_dual.e321), 
            /* e1234 */
            0.0
        );
        return MultiVector(
            /* scalar, e12345 */
            float2((other.scalar * wedge.e12345) + (other.e12345 * wedge.scalar) + (other.e1 * wedge.e4235) + (other.e2 * wedge.e4315) + (other.e3 * wedge.e4125) + (other.e4 * wedge.e3215) + (other.e5 * wedge.e1234) + (other.e4235 * wedge.e1) + (other.e4315 * wedge.e2) + (other.e4125 * wedge.e3) + (other.e3215 * wedge.e4) + (other.e1234 * wedge.e5) - (other.e15 * wedge.e423) - (other.e25 * wedge.e431) - (other.e35 * wedge.e412) - (other.e45 * wedge.e321) - (other.e41 * wedge.e235) - (other.e42 * wedge.e315) - (other.e43 * wedge.e125) - (other.e23 * wedge.e415) - (other.e31 * wedge.e425) - (other.e12 * wedge.e435) - (other.e415 * wedge.e23) - (other.e425 * wedge.e31) - (other.e435 * wedge.e12) - (other.e321 * wedge.e45) - (other.e423 * wedge.e15) - (other.e431 * wedge.e25) - (other.e412 * wedge.e35) - (other.e235 * wedge.e41) - (other.e315 * wedge.e42) - (other.e125 * wedge.e43), other.e12345 * wedge.e12345), 
            /* e1, e2, e3, e4 */
            float4((other.e415 * wedge.e321) + (other.e321 * wedge.e415) + (other.e4315 * wedge.e12) + (other.e1234 * wedge.e15), (other.e425 * wedge.e321) + (other.e321 * wedge.e425) + (other.e4125 * wedge.e23) + (other.e1234 * wedge.e25), (other.e435 * wedge.e321) + (other.e321 * wedge.e435) + (other.e4235 * wedge.e31) + (other.e1234 * wedge.e35), -(other.e45 * wedge.e1234) - (other.e415 * wedge.e423) - (other.e425 * wedge.e431) - (other.e435 * wedge.e412)) + (float4(other.e12345) * wedge.group1) + (float4(wedge.e12345) * other.group1) + float4((other.group4.xyz * wedge.group9.www), other.e4235 * wedge.e41) + float4((other.group5.xyz.yzx * wedge.group9.zxy), other.e4315 * wedge.e42) + float4((other.group7.xyz.zxy * wedge.group8.xyz.yzx), other.e4125 * wedge.e43) + float4((other.group8.xyz.yzx * wedge.group7.xyz.zxy), other.e1234 * wedge.e45) - (wedge.group9.yzxx * float4(other.group5.xyz.zxy, other.e41)) - float4((float3(wedge.e1234) * other.group3.xyz), other.e412 * wedge.e435) - float4((wedge.group4.xyz * other.group9.www), other.e423 * wedge.e415) - float4((other.group7.xyz.yzx * wedge.group8.xyz.zxy), other.e42 * wedge.e4315) - float4((other.group8.xyz.zxy * wedge.group7.xyz.yzx), other.e43 * wedge.e4125) - float4((wedge.group5.xyz.yzx * other.group9.zxy), other.e431 * wedge.e425), 
            /* e5 */
            (other.e12345 * wedge.e5) + (other.e5 * wedge.e12345) + (other.e15 * wedge.e4235) + (other.e25 * wedge.e4315) + (other.e35 * wedge.e4125) + (other.e45 * wedge.e3215) - (other.e415 * wedge.e235) - (other.e425 * wedge.e315) - (other.e435 * wedge.e125) - (other.e235 * wedge.e415) - (other.e315 * wedge.e425) - (other.e125 * wedge.e435) - (other.e4235 * wedge.e15) - (other.e4315 * wedge.e25) - (other.e4125 * wedge.e35) - (other.e3215 * wedge.e45), 
            /* e15, e25, e35, e45 */
            float4((other.e415 * wedge.e3215) + (other.e315 * wedge.e4125) + (other.e4125 * wedge.e315) + (other.e3215 * wedge.e415), (other.e425 * wedge.e3215) + (other.e125 * wedge.e4235) + (other.e4235 * wedge.e125) + (other.e3215 * wedge.e425), (other.e435 * wedge.e3215) + (other.e235 * wedge.e4315) + (other.e4315 * wedge.e235) + (other.e3215 * wedge.e435), -(other.e435 * wedge.e4125) - (other.e4235 * wedge.e415) - (other.e4315 * wedge.e425) - (other.e4125 * wedge.e435)) + (float4(other.e12345) * wedge.group3) + (float4(wedge.e12345) * other.group3) - (wedge.group9.yzxx * float4(other.group8.xyz.zxy, other.e415)) - float4((wedge.group8.xyz.zxy * other.group9.yzx), other.e425 * wedge.e4315), 
            /* e41, e42, e43 */
            (float3(other.e12345) * wedge.group4.xyz) + (float3(other.e1234) * wedge.group6.xyz) + (float3(wedge.e12345) * other.group4.xyz) + (float3(wedge.e1234) * other.group6.xyz) + (other.group7.xyz.zxy * wedge.group9.yzx) + (wedge.group7.xyz.zxy * other.group9.yzx) - (other.group7.xyz.yzx * wedge.group9.zxy) - (wedge.group7.xyz.yzx * other.group9.zxy), 
            /* e23, e31, e12 */
            (float3(other.e12345) * wedge.group5.xyz) + (float3(other.e3215) * wedge.group7.xyz) + (float3(other.e1234) * wedge.group8.xyz) + (float3(wedge.e12345) * other.group5.xyz) + (float3(wedge.e3215) * other.group7.xyz) + (float3(wedge.e1234) * other.group8.xyz) - (float3(other.e321) * wedge.group9.xyz) - (float3(wedge.e321) * other.group9.xyz), 
            /* e415, e425, e435, e321 */
            (float4(other.e12345) * wedge.group6) + (float4(wedge.e12345) * other.group6) + float4((other.group9.zxy * wedge.group9.yzx), other.e1234 * wedge.e3215) - (other.group9.yzxw * float4(wedge.group9.zxy, wedge.e1234)), 
            /* e423, e431, e412 */
            (float3(other.e12345) * wedge.group7.xyz) + (float3(other.e1234) * wedge.group9.xyz) + (float3(wedge.e12345) * other.group7.xyz) - (float3(wedge.e1234) * other.group9.xyz), 
            /* e235, e315, e125 */
            (float3(other.e12345) * wedge.group8.xyz) + (float3(wedge.e12345) * other.group8.xyz) + (float3(wedge.e3215) * other.group9.xyz) - (float3(other.e3215) * wedge.group9.xyz), 
            /* e4235, e4315, e4125, e3215 */
            (float4(other.e12345) * wedge.group9) + (float4(wedge.e12345) * other.group9), 
            /* e1234 */
            (other.e12345 * wedge.e1234) + (other.e1234 * wedge.e12345)
        );
    }
}
public extension Motor {
    public Motor project_orthogonally_onto(Plane other) {
        AntiPlane right_anti_dual = AntiPlane(
            /* e1, e2, e3, e5 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        Flector wedge = Flector(
            /* e15, e25, e35, e45 */
            float4(float3(1.0), 0.0) * float4(this.group1.www, 0.0) * float4(right_anti_dual.group0.xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual.e3 * this.e425 * -1.0, right_anti_dual.e1 * this.e435 * -1.0, right_anti_dual.e2 * this.e415 * -1.0, (right_anti_dual.e2 * this.e315) + (right_anti_dual.e3 * this.e125)) + (right_anti_dual.group0.yzxx * float4(this.group0.zxy, this.e235))
        );
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(((wedge.group1.yzx * other.group0.zxy) - (wedge.group1.zxy * other.group0.yzx)), 0.0), 
            /* e235, e315, e125, e5 */
            float4(wedge.e3215 * other.e4235, wedge.e3215 * other.e4315, wedge.e3215 * other.e4125, -(wedge.e25 * other.e4315) - (wedge.e35 * other.e4125) - (wedge.e45 * other.e3215)) - (other.group0.wwwx * float4(wedge.group1.xyz, wedge.e15))
        );
    }
}
public extension Motor {
    public RoundPoint project_orthogonally_onto(RoundPoint other) {
        AntiScalar wedge = AntiScalar(
            /* e12345 */
            this.e5 * other.e4 * -1.0
        );
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(wedge.e12345) * other.group0, 
            /* e5 */
            wedge.e12345 * other.e5
        );
    }
}
public extension Motor {
    public AntiDipoleInversion project_orthogonally_onto(Sphere other) {
        RoundPoint right_anti_dual = RoundPoint(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5 */
            other.e3215
        );
        Flector wedge = Flector(
            /* e15, e25, e35, e45 */
            float4(this.e5) * right_anti_dual.group0 * float4(-1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(-(this.e425 * right_anti_dual.e3) - (this.e235 * right_anti_dual.e4), -(this.e435 * right_anti_dual.e1) - (this.e315 * right_anti_dual.e4), -(this.e415 * right_anti_dual.e2) - (this.e125 * right_anti_dual.e4), (this.e315 * right_anti_dual.e2) + (this.e125 * right_anti_dual.e3)) + (right_anti_dual.group0.yzxx * float4(this.group0.zxy, this.e235))
        );
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(other.e1234) * wedge.group1.xyz, 
            /* e415, e425, e435, e321 */
            float4(((wedge.group1.yzx * other.group0.zxy) - (wedge.group1.zxy * other.group0.yzx)), wedge.e3215 * other.e1234), 
            /* e235, e315, e125, e4 */
            float4(((float3(wedge.e3215) * other.group0.xyz) - (float3(other.e3215) * wedge.group1.xyz)), wedge.e45 * other.e1234), 
            /* e1, e2, e3, e5 */
            float4((float3(other.e1234) * wedge.group0.xyz), -(wedge.e15 * other.e4235) - (wedge.e25 * other.e4315) - (wedge.e35 * other.e4125) - (wedge.e45 * other.e3215))
        );
    }
}
public extension Motor {
    public VersorEven project_orthogonally_onto(VersorEven other) {
        VersorOdd right_anti_dual = VersorOdd(
            /* e41, e42, e43, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e1234 */
            float4(other.group2.xyz, other.e4) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.group3.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0)
        );
        Motor wedge = Motor(
            /* e415, e425, e435, e12345 */
            (float4(this.e5) * float4(right_anti_dual.group0.xyz, right_anti_dual.e1234)) + (float4(right_anti_dual.scalar) * this.group0) + float4(float3(0.0), -(this.e415 * right_anti_dual.e23) - (this.e425 * right_anti_dual.e31) - (this.e435 * right_anti_dual.e12) - (this.e235 * right_anti_dual.e41) - (this.e315 * right_anti_dual.e42) - (this.e125 * right_anti_dual.e43)), 
            /* e235, e315, e125, e5 */
            float4(((float3(this.e5) * right_anti_dual.group1.xyz) + (float3(right_anti_dual.scalar) * this.group1.xyz)), this.e5 * right_anti_dual.scalar)
        );
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(wedge.e12345) * other.group0, 
            /* e415, e425, e435, e321 */
            float4(((float3(wedge.e12345) * other.group1.xyz) + (float3(other.e12345) * wedge.group0.xyz)), wedge.e12345 * other.e321), 
            /* e235, e315, e125, e5 */
            (float4(wedge.e12345) * other.group2) + (float4(other.e12345) * wedge.group1) + float4(float3(0.0), -(wedge.e415 * other.e235) - (wedge.e425 * other.e315) - (wedge.e435 * other.e125) - (wedge.e235 * other.e415) - (wedge.e315 * other.e425) - (wedge.e125 * other.e435)), 
            /* e1, e2, e3, e4 */
            float4((wedge.e12345 * other.e1) + (wedge.e315 * other.e412), (wedge.e12345 * other.e2) + (wedge.e125 * other.e423), (wedge.e12345 * other.e3) + (wedge.e235 * other.e431), -(wedge.e425 * other.e431) - (wedge.e435 * other.e412)) + (wedge.group0 * float4(other.group1.www, other.e4)) - (other.group0.yzxx * float4(wedge.group1.zxy, wedge.e415))
        );
    }
}
public extension Motor {
    public AntiDipoleInversion project_orthogonally_onto(VersorOdd other) {
        VersorEven right_anti_dual = VersorEven(
            /* e423, e431, e412, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            float4(other.group2.xyz, other.e3215) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4(other.group3.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0)
        );
        Flector wedge = Flector(
            /* e15, e25, e35, e45 */
            float4(this.e5) * right_anti_dual.group3 * float4(-1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(-(this.e235 * right_anti_dual.e4) - (this.e5 * right_anti_dual.e423), -(this.e315 * right_anti_dual.e4) - (this.e5 * right_anti_dual.e431), -(this.e125 * right_anti_dual.e4) - (this.e5 * right_anti_dual.e412), (this.e315 * right_anti_dual.e2) + (this.e125 * right_anti_dual.e3)) + (right_anti_dual.group3.yzxx * float4(this.group0.zxy, this.e235)) - float4((this.group0.yzx * right_anti_dual.group3.zxy), this.e5 * right_anti_dual.e321)
        );
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(other.e1234) * wedge.group1.xyz, 
            /* e415, e425, e435, e321 */
            float4(((wedge.group1.yzx * other.group3.zxy) - (wedge.group1.zxy * other.group3.yzx)), wedge.e3215 * other.e1234), 
            /* e235, e315, e125, e4 */
            float4(float3(0.0), -(wedge.e4315 * other.e42) - (wedge.e4125 * other.e43)) + float4((wedge.group1.www * other.group3.xyz), wedge.e45 * other.e1234) - (wedge.group1.xyzx * float4(other.group3.www, other.e41)), 
            /* e1, e2, e3, e5 */
            (wedge.group1.zxyy * float4(other.group1.yzx, other.e25)) + (wedge.group1.wwwz * float4(other.group0.xyz, other.e35)) + (other.group2.wwwx * float4(wedge.group0.xyz, wedge.e4235)) + float4(float3(0.0), (wedge.e3215 * other.e45) - (wedge.e25 * other.e4315) - (wedge.e35 * other.e4125) - (wedge.e45 * other.e3215)) - float4((wedge.group1.yzx * other.group1.zxy), wedge.e15 * other.e4235)
        );
    }
}
public extension MultiVector {
    // Fancy infix trick (first half)
    public ProjectOrthogonallyOntoInfixPartial<MultiVector> operator /(ProjectOrthogonallyOntoInfix rhs) {
        return ProjectOrthogonallyOntoInfixPartial<MultiVector>(this);
    }
}
public extension ProjectOrthogonallyOntoInfixPartial<MultiVector> {
    // Fancy infix trick (second half)
    public MultiVector operator /(AntiCircleRotor rhs) {
        return this.a.project_orthogonally_onto(rhs);
    }
}
public extension MultiVector {
    public MultiVector project_orthogonally_onto(AntiCircleRotor other) {
        CircleRotor right_anti_dual = CircleRotor(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        MultiVector wedge = MultiVector(
            /* scalar, e12345 */
            float2(0.0, (right_anti_dual.e12345 * this.scalar) - (right_anti_dual.e423 * this.e15) - (right_anti_dual.e431 * this.e25) - (right_anti_dual.e412 * this.e35) - (right_anti_dual.e415 * this.e23) - (right_anti_dual.e425 * this.e31) - (right_anti_dual.e435 * this.e12) - (right_anti_dual.e321 * this.e45) - (right_anti_dual.e235 * this.e41) - (right_anti_dual.e315 * this.e42) - (right_anti_dual.e125 * this.e43)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(this.scalar) * right_anti_dual.group1, 
            /* e423, e431, e412 */
            float3(this.scalar) * right_anti_dual.group0.xyz, 
            /* e235, e315, e125 */
            float3(this.scalar) * right_anti_dual.group2.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual.e425 * this.e3) + (right_anti_dual.e235 * this.e4), (right_anti_dual.e435 * this.e1) + (right_anti_dual.e315 * this.e4), (right_anti_dual.e415 * this.e2) + (right_anti_dual.e125 * this.e4), -(right_anti_dual.e315 * this.e2) - (right_anti_dual.e125 * this.e3)) - (float4(this.e5) * float4(right_anti_dual.group0.xyz, right_anti_dual.e321)) - (this.group1.yzxx * float4(right_anti_dual.group1.zxy, right_anti_dual.e235)), 
            /* e1234 */
            (right_anti_dual.e423 * this.e1) + (right_anti_dual.e431 * this.e2) + (right_anti_dual.e412 * this.e3) + (right_anti_dual.e321 * this.e4)
        );
        return MultiVector(
            /* scalar, e12345 */
            float2((other.scalar * wedge.e12345) - (other.e41 * wedge.e235) - (other.e42 * wedge.e315) - (other.e43 * wedge.e125) - (other.e23 * wedge.e415) - (other.e31 * wedge.e425) - (other.e12 * wedge.e435) - (other.e45 * wedge.e321) - (other.e15 * wedge.e423) - (other.e25 * wedge.e431) - (other.e35 * wedge.e412), 0.0), 
            /* e1, e2, e3, e4 */
            float4((other.e41 * wedge.e3215) + (other.e31 * wedge.e4125), (other.e42 * wedge.e3215) + (other.e12 * wedge.e4235), (other.e43 * wedge.e3215) + (other.e23 * wedge.e4315), -(other.e43 * wedge.e4125) - (other.e45 * wedge.e1234)) - (wedge.group9.yzxx * float4(other.group1.zxy, other.e41)) - float4((float3(wedge.e1234) * other.group2.xyz), other.e42 * wedge.e4315), 
            /* e5 */
            (other.e45 * wedge.e3215) + (other.e15 * wedge.e4235) + (other.e25 * wedge.e4315) + (other.e35 * wedge.e4125), 
            /* e15, e25, e35, e45 */
            float4(wedge.e12345) * float4(other.group2.xyz, other.e45), 
            /* e41, e42, e43 */
            float3(wedge.e12345) * other.group0.xyz, 
            /* e23, e31, e12 */
            float3(wedge.e12345) * other.group1.xyz, 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
public extension MultiVector {
    public MultiVector project_orthogonally_onto(AntiDipoleInversion other) {
        DipoleInversion right_anti_dual = DipoleInversion(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e1234 */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group3 * float4(1.0, 1.0, 1.0, -1.0)
        );
        MultiVector wedge = MultiVector(
            /* scalar, e12345 */
            float2(0.0, (right_anti_dual.e1234 * this.e5) + (right_anti_dual.e4235 * this.e1) + (right_anti_dual.e4315 * this.e2) + (right_anti_dual.e4125 * this.e3) + (right_anti_dual.e3215 * this.e4) - (right_anti_dual.e41 * this.e235) - (right_anti_dual.e42 * this.e315) - (right_anti_dual.e43 * this.e125) - (right_anti_dual.e23 * this.e415) - (right_anti_dual.e31 * this.e425) - (right_anti_dual.e12 * this.e435) - (right_anti_dual.e45 * this.e321) - (right_anti_dual.e15 * this.e423) - (right_anti_dual.e25 * this.e431) - (right_anti_dual.e35 * this.e412)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            float4(this.scalar) * float4(right_anti_dual.group2.xyz, right_anti_dual.e45), 
            /* e41, e42, e43 */
            float3(this.scalar) * right_anti_dual.group0.xyz, 
            /* e23, e31, e12 */
            float3(this.scalar) * right_anti_dual.group1.xyz, 
            /* e415, e425, e435, e321 */
            float4((right_anti_dual.e41 * this.e5) + (right_anti_dual.e15 * this.e4), (right_anti_dual.e42 * this.e5) + (right_anti_dual.e25 * this.e4), (right_anti_dual.e43 * this.e5) + (right_anti_dual.e35 * this.e4), -(right_anti_dual.e31 * this.e2) - (right_anti_dual.e12 * this.e3)) - (right_anti_dual.group1.wwwx * this.group1.xyzx), 
            /* e423, e431, e412 */
            (float3(this.e4) * right_anti_dual.group1.xyz) + (right_anti_dual.group0.xyz.yzx * this.group1.zxy) - (right_anti_dual.group0.xyz.zxy * this.group1.yzx), 
            /* e235, e315, e125 */
            (float3(this.e5) * right_anti_dual.group1.xyz) + (right_anti_dual.group2.zxy * this.group1.yzx) - (right_anti_dual.group2.yzx * this.group1.zxy), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual.e42 * this.e35) + (right_anti_dual.e23 * this.e45) + (right_anti_dual.e45 * this.e23) + (right_anti_dual.e35 * this.e42), (right_anti_dual.e43 * this.e15) + (right_anti_dual.e31 * this.e45) + (right_anti_dual.e45 * this.e31) + (right_anti_dual.e15 * this.e43), (right_anti_dual.e41 * this.e25) + (right_anti_dual.e12 * this.e45) + (right_anti_dual.e45 * this.e12) + (right_anti_dual.e25 * this.e41), -(right_anti_dual.e23 * this.e15) - (right_anti_dual.e31 * this.e25) - (right_anti_dual.e12 * this.e35) - (right_anti_dual.e35 * this.e12)) + (float4(this.scalar) * right_anti_dual.group3) - (right_anti_dual.group2.yzxy * float4(this.group4.xyz.zxy, this.e31)) - float4((right_anti_dual.group0.xyz.zxy * this.group3.yzx), right_anti_dual.e15 * this.e23), 
            /* e1234 */
            (right_anti_dual.e1234 * this.scalar) - (right_anti_dual.e41 * this.e23) - (right_anti_dual.e42 * this.e31) - (right_anti_dual.e43 * this.e12) - (right_anti_dual.e23 * this.e41) - (right_anti_dual.e31 * this.e42) - (right_anti_dual.e12 * this.e43)
        );
        return MultiVector(
            /* scalar, e12345 */
            float2((other.e4 * wedge.e3215) + (other.e1 * wedge.e4235) + (other.e2 * wedge.e4315) + (other.e3 * wedge.e4125) + (other.e5 * wedge.e1234) - (other.e423 * wedge.e15) - (other.e431 * wedge.e25) - (other.e412 * wedge.e35) - (other.e415 * wedge.e23) - (other.e425 * wedge.e31) - (other.e435 * wedge.e12) - (other.e321 * wedge.e45) - (other.e235 * wedge.e41) - (other.e315 * wedge.e42) - (other.e125 * wedge.e43), 0.0), 
            /* e1, e2, e3, e4 */
            float4((other.e412 * wedge.e315) + (other.e415 * wedge.e321) + (other.e321 * wedge.e415) + (other.e315 * wedge.e412), (other.e423 * wedge.e125) + (other.e425 * wedge.e321) + (other.e321 * wedge.e425) + (other.e125 * wedge.e423), (other.e431 * wedge.e235) + (other.e435 * wedge.e321) + (other.e321 * wedge.e435) + (other.e235 * wedge.e431), -(other.e412 * wedge.e435) - (other.e415 * wedge.e423) - (other.e425 * wedge.e431) - (other.e435 * wedge.e412)) + (float4(wedge.e12345) * float4(other.group3.xyz, other.e4)) - float4((other.group0.xyz.yzx * wedge.group8.xyz.zxy), other.e423 * wedge.e415) - float4((wedge.group7.xyz.yzx * other.group2.zxy), other.e431 * wedge.e425), 
            /* e5 */
            (other.e5 * wedge.e12345) - (other.e415 * wedge.e235) - (other.e425 * wedge.e315) - (other.e435 * wedge.e125) - (other.e235 * wedge.e415) - (other.e315 * wedge.e425) - (other.e125 * wedge.e435), 
            /* e15, e25, e35, e45 */
            float4((other.e415 * wedge.e3215) + (other.e315 * wedge.e4125), (other.e425 * wedge.e3215) + (other.e125 * wedge.e4235), (other.e435 * wedge.e3215) + (other.e235 * wedge.e4315), -(other.e425 * wedge.e4315) - (other.e435 * wedge.e4125)) - (wedge.group9.yzxx * float4(other.group2.zxy, other.e415)), 
            /* e41, e42, e43 */
            (float3(wedge.e1234) * other.group1.xyz) + (other.group0.xyz.zxy * wedge.group9.yzx) - (other.group0.xyz.yzx * wedge.group9.zxy), 
            /* e23, e31, e12 */
            (float3(wedge.e3215) * other.group0.xyz) + (float3(wedge.e1234) * other.group2.xyz) - (float3(other.e321) * wedge.group9.xyz), 
            /* e415, e425, e435, e321 */
            float4(wedge.e12345) * other.group1, 
            /* e423, e431, e412 */
            float3(wedge.e12345) * other.group0.xyz, 
            /* e235, e315, e125 */
            float3(wedge.e12345) * other.group2.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
public extension MultiVector {
    public MultiVector project_orthogonally_onto(AntiDualNum other) {
        DualNum right_anti_dual = DualNum(
            /* e5, e12345 */
            other.group0.xy
        );
        MultiVector wedge = MultiVector(
            /* scalar, e12345 */
            float2(0.0, (right_anti_dual.e5 * this.e1234) + (right_anti_dual.e12345 * this.scalar)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            right_anti_dual.e5 * this.scalar, 
            /* e15, e25, e35, e45 */
            float4(right_anti_dual.e5) * this.group1, 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(right_anti_dual.group0.xy.xx, right_anti_dual.e5, 0.0) * float4(float3(1.0), 0.0) * float4(this.group4.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(right_anti_dual.e5) * this.group5.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual.e5) * float4(this.group7.xyz, this.e321), 
            /* e1234 */
            0.0
        );
        return MultiVector(
            /* scalar, e12345 */
            float2((other.e3215 * wedge.e4) + (other.scalar * wedge.e12345), 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            other.e3215 * wedge.e45 * -1.0, 
            /* e15, e25, e35, e45 */
            float4(other.group0.xy.xx, other.e3215, 0.0) * float4(float3(1.0), 0.0) * float4(wedge.group6.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(other.e3215) * wedge.group9.xyz * float3(-1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), other.e3215 * wedge.e12345), 
            /* e1234 */
            0.0
        );
    }
}
public extension MultiVector {
    public MultiVector project_orthogonally_onto(AntiFlatPoint other) {
        FlatPoint right_anti_dual = FlatPoint(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        MultiVector wedge = MultiVector(
            /* scalar, e12345 */
            float2(0.0, -(right_anti_dual.e15 * this.e423) - (right_anti_dual.e25 * this.e431) - (right_anti_dual.e35 * this.e412) - (right_anti_dual.e45 * this.e321)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            float4(this.scalar) * right_anti_dual.group0, 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(((float3(this.e4) * right_anti_dual.group0.xyz) - (float3(right_anti_dual.e45) * this.group1.xyz)), 0.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            (right_anti_dual.group0.zxy * this.group1.yzx) - (right_anti_dual.group0.yzx * this.group1.zxy), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual.e35 * this.e42) + (right_anti_dual.e45 * this.e23), (right_anti_dual.e15 * this.e43) + (right_anti_dual.e45 * this.e31), (right_anti_dual.e25 * this.e41) + (right_anti_dual.e45 * this.e12), -(right_anti_dual.e25 * this.e31) - (right_anti_dual.e35 * this.e12)) - (right_anti_dual.group0.yzxx * float4(this.group4.xyz.zxy, this.e23)), 
            /* e1234 */
            0.0
        );
        return MultiVector(
            /* scalar, e12345 */
            float2(-(other.e235 * wedge.e41) - (other.e315 * wedge.e42) - (other.e125 * wedge.e43) - (other.e321 * wedge.e45), 0.0), 
            /* e1, e2, e3, e4 */
            float4((float3(other.e321) * wedge.group6.xyz), 0.0) + float4((wedge.group7.xyz.zxy * other.group0.yzx), 0.0) - float4((wedge.group7.xyz.yzx * other.group0.zxy), 0.0), 
            /* e5 */
            -(other.e235 * wedge.e415) - (other.e315 * wedge.e425) - (other.e125 * wedge.e435), 
            /* e15, e25, e35, e45 */
            float4(((other.group0.yzx * wedge.group9.zxy) - (other.group0.zxy * wedge.group9.yzx)), 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            (float3(wedge.e1234) * other.group0.xyz) - (float3(other.e321) * wedge.group9.xyz), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), other.e321 * wedge.e12345), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(wedge.e12345) * other.group0.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
public extension MultiVector {
    public MultiVector project_orthogonally_onto(AntiFlector other) {
        Flector right_anti_dual = Flector(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        MultiVector wedge = MultiVector(
            /* scalar, e12345 */
            float2(0.0, (right_anti_dual.e4235 * this.e1) + (right_anti_dual.e4315 * this.e2) + (right_anti_dual.e4125 * this.e3) + (right_anti_dual.e3215 * this.e4) - (right_anti_dual.e15 * this.e423) - (right_anti_dual.e25 * this.e431) - (right_anti_dual.e35 * this.e412) - (right_anti_dual.e45 * this.e321)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            float4(this.scalar) * right_anti_dual.group0, 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(((float3(this.e4) * right_anti_dual.group0.xyz) - (float3(right_anti_dual.e45) * this.group1.xyz)), 0.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            (right_anti_dual.group0.zxy * this.group1.yzx) - (right_anti_dual.group0.yzx * this.group1.zxy), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual.e35 * this.e42) + (right_anti_dual.e45 * this.e23), (right_anti_dual.e15 * this.e43) + (right_anti_dual.e45 * this.e31), (right_anti_dual.e25 * this.e41) + (right_anti_dual.e45 * this.e12), -(right_anti_dual.e25 * this.e31) - (right_anti_dual.e35 * this.e12)) + (float4(this.scalar) * right_anti_dual.group1) - (right_anti_dual.group0.yzxx * float4(this.group4.xyz.zxy, this.e23)), 
            /* e1234 */
            0.0
        );
        return MultiVector(
            /* scalar, e12345 */
            float2((other.e1 * wedge.e4235) + (other.e2 * wedge.e4315) + (other.e3 * wedge.e4125) + (other.e5 * wedge.e1234) - (other.e235 * wedge.e41) - (other.e315 * wedge.e42) - (other.e125 * wedge.e43) - (other.e321 * wedge.e45), 0.0), 
            /* e1, e2, e3, e4 */
            float4((float3(other.e321) * wedge.group6.xyz), 0.0) + float4((float3(wedge.e12345) * other.group1.xyz), 0.0) + float4((wedge.group7.xyz.zxy * other.group0.yzx), 0.0) - float4((wedge.group7.xyz.yzx * other.group0.zxy), 0.0), 
            /* e5 */
            (other.e5 * wedge.e12345) - (other.e235 * wedge.e415) - (other.e315 * wedge.e425) - (other.e125 * wedge.e435), 
            /* e15, e25, e35, e45 */
            float4(((other.group0.yzx * wedge.group9.zxy) - (other.group0.zxy * wedge.group9.yzx)), 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            (float3(wedge.e1234) * other.group0.xyz) - (float3(other.e321) * wedge.group9.xyz), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), other.e321 * wedge.e12345), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(wedge.e12345) * other.group0.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
public extension MultiVector {
    public MultiVector project_orthogonally_onto(AntiLine other) {
        Line right_anti_dual = Line(
            /* e415, e425, e435 */
            other.group0.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            other.group1.xyz * float3(-1.0)
        );
        MultiVector wedge = MultiVector(
            /* scalar, e12345 */
            float2(0.0, -(right_anti_dual.e415 * this.e23) - (right_anti_dual.e425 * this.e31) - (right_anti_dual.e435 * this.e12) - (right_anti_dual.e235 * this.e41) - (right_anti_dual.e315 * this.e42) - (right_anti_dual.e125 * this.e43)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(this.group0.xy.xx, this.scalar, 0.0) * float4(float3(1.0), 0.0) * float4(right_anti_dual.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(this.scalar) * right_anti_dual.group1.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual.e425 * this.e3) + (right_anti_dual.e235 * this.e4), (right_anti_dual.e435 * this.e1) + (right_anti_dual.e315 * this.e4), (right_anti_dual.e415 * this.e2) + (right_anti_dual.e125 * this.e4), -(right_anti_dual.e315 * this.e2) - (right_anti_dual.e125 * this.e3)) - (this.group1.yzxx * float4(right_anti_dual.group0.xyz.zxy, right_anti_dual.e235)), 
            /* e1234 */
            0.0
        );
        return MultiVector(
            /* scalar, e12345 */
            float2(-(other.e23 * wedge.e415) - (other.e31 * wedge.e425) - (other.e12 * wedge.e435) - (other.e15 * wedge.e423) - (other.e25 * wedge.e431) - (other.e35 * wedge.e412), 0.0), 
            /* e1, e2, e3, e4 */
            float4((other.group0.xyz.yzx * wedge.group9.zxy), 0.0) - float4((float3(wedge.e1234) * other.group1.xyz), 0.0) - float4((other.group0.xyz.zxy * wedge.group9.yzx), 0.0), 
            /* e5 */
            (other.e15 * wedge.e4235) + (other.e25 * wedge.e4315) + (other.e35 * wedge.e4125), 
            /* e15, e25, e35, e45 */
            float4(wedge.group0.xy.yy, wedge.e12345, 0.0) * float4(float3(1.0), 0.0) * float4(other.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(wedge.e12345) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
public extension MultiVector {
    public MultiVector project_orthogonally_onto(AntiMotor other) {
        Motor right_anti_dual = Motor(
            /* e415, e425, e435, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        MultiVector wedge = MultiVector(
            /* scalar, e12345 */
            float2(0.0, (right_anti_dual.e12345 * this.scalar) + (right_anti_dual.e5 * this.e1234) - (right_anti_dual.e415 * this.e23) - (right_anti_dual.e425 * this.e31) - (right_anti_dual.e435 * this.e12) - (right_anti_dual.e235 * this.e41) - (right_anti_dual.e315 * this.e42) - (right_anti_dual.e125 * this.e43)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            right_anti_dual.e5 * this.scalar, 
            /* e15, e25, e35, e45 */
            float4(right_anti_dual.e5) * this.group1, 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(((float3(right_anti_dual.e5) * this.group4.xyz) + (float3(this.scalar) * right_anti_dual.group0.xyz)), 0.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            (float3(right_anti_dual.e5) * this.group5.xyz) + (float3(this.scalar) * right_anti_dual.group1.xyz), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual.e425 * this.e3) + (right_anti_dual.e235 * this.e4), (right_anti_dual.e435 * this.e1) + (right_anti_dual.e315 * this.e4), (right_anti_dual.e415 * this.e2) + (right_anti_dual.e125 * this.e4), -(right_anti_dual.e315 * this.e2) - (right_anti_dual.e125 * this.e3)) + float4((this.group7.xyz * right_anti_dual.group1.www), right_anti_dual.e5 * this.e321) - (this.group1.yzxx * float4(right_anti_dual.group0.zxy, right_anti_dual.e235)), 
            /* e1234 */
            0.0
        );
        return MultiVector(
            /* scalar, e12345 */
            float2((other.scalar * wedge.e12345) + (other.e3215 * wedge.e4) - (other.e23 * wedge.e415) - (other.e31 * wedge.e425) - (other.e12 * wedge.e435) - (other.e15 * wedge.e423) - (other.e25 * wedge.e431) - (other.e35 * wedge.e412), 0.0), 
            /* e1, e2, e3, e4 */
            float4((other.group0.yzx * wedge.group9.zxy), 0.0) - float4((float3(other.e3215) * wedge.group4.xyz), 0.0) - float4((float3(wedge.e1234) * other.group1.xyz), 0.0) - float4((other.group0.zxy * wedge.group9.yzx), 0.0), 
            /* e5 */
            (other.e15 * wedge.e4235) + (other.e25 * wedge.e4315) + (other.e35 * wedge.e4125) - (other.e3215 * wedge.e45), 
            /* e15, e25, e35, e45 */
            float4(((float3(other.e3215) * wedge.group6.xyz) + (float3(wedge.e12345) * other.group1.xyz)), 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            (float3(other.e3215) * wedge.group7.xyz) + (float3(wedge.e12345) * other.group0.xyz), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(other.e3215) * wedge.group9.xyz * float3(-1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), other.e3215 * wedge.e12345), 
            /* e1234 */
            0.0
        );
    }
}
public extension MultiVector {
    public MultiVector project_orthogonally_onto(AntiPlane other) {
        Plane right_anti_dual = Plane(
            /* e4235, e4315, e4125, e3215 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        MultiVector wedge = MultiVector(
            /* scalar, e12345 */
            float2(0.0, (this.e1 * right_anti_dual.e4235) + (this.e2 * right_anti_dual.e4315) + (this.e3 * right_anti_dual.e4125) + (this.e4 * right_anti_dual.e3215)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
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
            float4(this.scalar) * right_anti_dual.group0, 
            /* e1234 */
            0.0
        );
        return MultiVector(
            /* scalar, e12345 */
            float2((other.e1 * wedge.e4235) + (other.e2 * wedge.e4315) + (other.e3 * wedge.e4125) + (other.e5 * wedge.e1234), 0.0), 
            /* e1, e2, e3, e4 */
            float4(wedge.group0.xy.yy, wedge.e12345, 0.0) * float4(float3(1.0), 0.0) * float4(other.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e5 */
            other.e5 * wedge.e12345, 
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
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
public extension MultiVector {
    public MultiVector project_orthogonally_onto(AntiScalar other) {
        Scalar right_anti_dual = Scalar(
            /* scalar */
            other.e12345 * -1.0
        );
        MultiVector wedge = MultiVector(
            /* scalar, e12345 */
            float2(right_anti_dual.scalar) * this.group0.xy, 
            /* e1, e2, e3, e4 */
            float4(right_anti_dual.scalar) * this.group1, 
            /* e5 */
            this.e5 * right_anti_dual.scalar, 
            /* e15, e25, e35, e45 */
            float4(right_anti_dual.scalar) * this.group3, 
            /* e41, e42, e43 */
            float3(right_anti_dual.scalar) * this.group4.xyz, 
            /* e23, e31, e12 */
            float3(right_anti_dual.scalar) * this.group5.xyz, 
            /* e415, e425, e435, e321 */
            float4(right_anti_dual.scalar) * this.group6, 
            /* e423, e431, e412 */
            float3(right_anti_dual.scalar) * this.group7.xyz, 
            /* e235, e315, e125 */
            float3(right_anti_dual.scalar) * this.group8.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual.scalar) * this.group9, 
            /* e1234 */
            this.e1234 * right_anti_dual.scalar
        );
        return MultiVector(
            /* scalar, e12345 */
            float2(other.e12345) * wedge.group0.xy, 
            /* e1, e2, e3, e4 */
            float4(other.e12345) * wedge.group1, 
            /* e5 */
            other.e12345 * wedge.e5, 
            /* e15, e25, e35, e45 */
            float4(other.e12345) * wedge.group3, 
            /* e41, e42, e43 */
            float3(other.e12345) * wedge.group4.xyz, 
            /* e23, e31, e12 */
            float3(other.e12345) * wedge.group5.xyz, 
            /* e415, e425, e435, e321 */
            float4(other.e12345) * wedge.group6, 
            /* e423, e431, e412 */
            float3(other.e12345) * wedge.group7.xyz, 
            /* e235, e315, e125 */
            float3(other.e12345) * wedge.group8.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345) * wedge.group9, 
            /* e1234 */
            other.e12345 * wedge.e1234
        );
    }
}
public extension MultiVector {
    public MultiVector project_orthogonally_onto(Circle other) {
        Dipole right_anti_dual = Dipole(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35 */
            other.group2.xyz
        );
        MultiVector wedge = MultiVector(
            /* scalar, e12345 */
            float2(0.0, -(right_anti_dual.e41 * this.e235) - (right_anti_dual.e42 * this.e315) - (right_anti_dual.e43 * this.e125) - (right_anti_dual.e23 * this.e415) - (right_anti_dual.e31 * this.e425) - (right_anti_dual.e12 * this.e435) - (right_anti_dual.e45 * this.e321) - (right_anti_dual.e15 * this.e423) - (right_anti_dual.e25 * this.e431) - (right_anti_dual.e35 * this.e412)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            float4(this.scalar) * float4(right_anti_dual.group2.xyz, right_anti_dual.e45), 
            /* e41, e42, e43 */
            float3(this.scalar) * right_anti_dual.group0.xyz, 
            /* e23, e31, e12 */
            float3(this.scalar) * right_anti_dual.group1.xyz, 
            /* e415, e425, e435, e321 */
            float4((right_anti_dual.e41 * this.e5) + (right_anti_dual.e15 * this.e4), (right_anti_dual.e42 * this.e5) + (right_anti_dual.e25 * this.e4), (right_anti_dual.e43 * this.e5) + (right_anti_dual.e35 * this.e4), -(right_anti_dual.e31 * this.e2) - (right_anti_dual.e12 * this.e3)) - (right_anti_dual.group1.wwwx * this.group1.xyzx), 
            /* e423, e431, e412 */
            (float3(this.e4) * right_anti_dual.group1.xyz) + (right_anti_dual.group0.xyz.yzx * this.group1.zxy) - (right_anti_dual.group0.xyz.zxy * this.group1.yzx), 
            /* e235, e315, e125 */
            (float3(this.e5) * right_anti_dual.group1.xyz) + (right_anti_dual.group2.xyz.zxy * this.group1.yzx) - (right_anti_dual.group2.xyz.yzx * this.group1.zxy), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual.e42 * this.e35) + (right_anti_dual.e23 * this.e45) + (right_anti_dual.e45 * this.e23) + (right_anti_dual.e35 * this.e42), (right_anti_dual.e43 * this.e15) + (right_anti_dual.e31 * this.e45) + (right_anti_dual.e45 * this.e31) + (right_anti_dual.e15 * this.e43), (right_anti_dual.e41 * this.e25) + (right_anti_dual.e12 * this.e45) + (right_anti_dual.e45 * this.e12) + (right_anti_dual.e25 * this.e41), -(right_anti_dual.e23 * this.e15) - (right_anti_dual.e31 * this.e25) - (right_anti_dual.e12 * this.e35) - (right_anti_dual.e35 * this.e12)) - float4((right_anti_dual.group0.xyz.zxy * this.group3.yzx), right_anti_dual.e15 * this.e23) - float4((right_anti_dual.group2.xyz.yzx * this.group4.xyz.zxy), right_anti_dual.e25 * this.e31), 
            /* e1234 */
            -(right_anti_dual.e41 * this.e23) - (right_anti_dual.e42 * this.e31) - (right_anti_dual.e43 * this.e12) - (right_anti_dual.e23 * this.e41) - (right_anti_dual.e31 * this.e42) - (right_anti_dual.e12 * this.e43)
        );
        return MultiVector(
            /* scalar, e12345 */
            float2(-(other.e423 * wedge.e15) - (other.e431 * wedge.e25) - (other.e412 * wedge.e35) - (other.e415 * wedge.e23) - (other.e425 * wedge.e31) - (other.e435 * wedge.e12) - (other.e321 * wedge.e45) - (other.e235 * wedge.e41) - (other.e315 * wedge.e42) - (other.e125 * wedge.e43), 0.0), 
            /* e1, e2, e3, e4 */
            float4((other.e412 * wedge.e315) + (other.e415 * wedge.e321) + (other.e321 * wedge.e415) + (other.e315 * wedge.e412), (other.e423 * wedge.e125) + (other.e425 * wedge.e321) + (other.e321 * wedge.e425) + (other.e125 * wedge.e423), (other.e431 * wedge.e235) + (other.e435 * wedge.e321) + (other.e321 * wedge.e435) + (other.e235 * wedge.e431), -(other.e412 * wedge.e435) - (other.e415 * wedge.e423) - (other.e425 * wedge.e431) - (other.e435 * wedge.e412)) - float4((other.group0.xyz.yzx * wedge.group8.xyz.zxy), other.e423 * wedge.e415) - float4((other.group2.xyz.zxy * wedge.group7.xyz.yzx), other.e431 * wedge.e425), 
            /* e5 */
            -(other.e415 * wedge.e235) - (other.e425 * wedge.e315) - (other.e435 * wedge.e125) - (other.e235 * wedge.e415) - (other.e315 * wedge.e425) - (other.e125 * wedge.e435), 
            /* e15, e25, e35, e45 */
            float4((other.e415 * wedge.e3215) + (other.e315 * wedge.e4125), (other.e425 * wedge.e3215) + (other.e125 * wedge.e4235), (other.e435 * wedge.e3215) + (other.e235 * wedge.e4315), -(other.e425 * wedge.e4315) - (other.e435 * wedge.e4125)) - (wedge.group9.yzxx * float4(other.group2.xyz.zxy, other.e415)), 
            /* e41, e42, e43 */
            (float3(wedge.e1234) * other.group1.xyz) + (other.group0.xyz.zxy * wedge.group9.yzx) - (other.group0.xyz.yzx * wedge.group9.zxy), 
            /* e23, e31, e12 */
            (float3(wedge.e3215) * other.group0.xyz) + (float3(wedge.e1234) * other.group2.xyz) - (float3(other.e321) * wedge.group9.xyz), 
            /* e415, e425, e435, e321 */
            float4(wedge.e12345) * other.group1, 
            /* e423, e431, e412 */
            float3(wedge.e12345) * other.group0.xyz, 
            /* e235, e315, e125 */
            float3(wedge.e12345) * other.group2.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
public extension MultiVector {
    public MultiVector project_orthogonally_onto(CircleRotor other) {
        AntiCircleRotor right_anti_dual = AntiCircleRotor(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        MultiVector wedge = MultiVector(
            /* scalar, e12345 */
            float2(right_anti_dual.scalar * this.scalar, (right_anti_dual.scalar * this.e12345) - (right_anti_dual.e41 * this.e235) - (right_anti_dual.e42 * this.e315) - (right_anti_dual.e43 * this.e125) - (right_anti_dual.e23 * this.e415) - (right_anti_dual.e31 * this.e425) - (right_anti_dual.e12 * this.e435) - (right_anti_dual.e45 * this.e321) - (right_anti_dual.e15 * this.e423) - (right_anti_dual.e25 * this.e431) - (right_anti_dual.e35 * this.e412)), 
            /* e1, e2, e3, e4 */
            float4(right_anti_dual.scalar) * this.group1, 
            /* e5 */
            right_anti_dual.scalar * this.e5, 
            /* e15, e25, e35, e45 */
            (float4(right_anti_dual.scalar) * this.group3) + (float4(this.scalar) * float4(right_anti_dual.group2.xyz, right_anti_dual.e45)), 
            /* e41, e42, e43 */
            (float3(right_anti_dual.scalar) * this.group4.xyz) + (float3(this.scalar) * right_anti_dual.group0.xyz), 
            /* e23, e31, e12 */
            (float3(right_anti_dual.scalar) * this.group5.xyz) + (float3(this.scalar) * right_anti_dual.group1.xyz), 
            /* e415, e425, e435, e321 */
            float4((right_anti_dual.e15 * this.e4) + (right_anti_dual.scalar * this.e415), (right_anti_dual.e25 * this.e4) + (right_anti_dual.scalar * this.e425), (right_anti_dual.e35 * this.e4) + (right_anti_dual.scalar * this.e435), -(right_anti_dual.e31 * this.e2) - (right_anti_dual.e12 * this.e3)) + float4((float3(this.e5) * right_anti_dual.group0.xyz), right_anti_dual.scalar * this.e321) - (right_anti_dual.group1.wwwx * this.group1.xyzx), 
            /* e423, e431, e412 */
            (float3(right_anti_dual.scalar) * this.group7.xyz) + (float3(this.e4) * right_anti_dual.group1.xyz) + (right_anti_dual.group0.xyz.yzx * this.group1.zxy) - (right_anti_dual.group0.xyz.zxy * this.group1.yzx), 
            /* e235, e315, e125 */
            (float3(right_anti_dual.scalar) * this.group8.xyz) + (float3(this.e5) * right_anti_dual.group1.xyz) + (right_anti_dual.group2.zxy * this.group1.yzx) - (right_anti_dual.group2.yzx * this.group1.zxy), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual.e23 * this.e45) + (right_anti_dual.e45 * this.e23) + (right_anti_dual.e35 * this.e42) + (right_anti_dual.scalar * this.e4235), (right_anti_dual.e31 * this.e45) + (right_anti_dual.e45 * this.e31) + (right_anti_dual.e15 * this.e43) + (right_anti_dual.scalar * this.e4315), (right_anti_dual.e12 * this.e45) + (right_anti_dual.e45 * this.e12) + (right_anti_dual.e25 * this.e41) + (right_anti_dual.scalar * this.e4125), -(right_anti_dual.e23 * this.e15) - (right_anti_dual.e31 * this.e25) - (right_anti_dual.e12 * this.e35) - (right_anti_dual.e35 * this.e12)) + float4((right_anti_dual.group0.xyz.yzx * this.group3.zxy), right_anti_dual.scalar * this.e3215) - (right_anti_dual.group2.yzxy * float4(this.group4.xyz.zxy, this.e31)) - float4((right_anti_dual.group0.xyz.zxy * this.group3.yzx), right_anti_dual.e15 * this.e23), 
            /* e1234 */
            (right_anti_dual.scalar * this.e1234) - (right_anti_dual.e41 * this.e23) - (right_anti_dual.e42 * this.e31) - (right_anti_dual.e43 * this.e12) - (right_anti_dual.e23 * this.e41) - (right_anti_dual.e31 * this.e42) - (right_anti_dual.e12 * this.e43)
        );
        return MultiVector(
            /* scalar, e12345 */
            float2((other.e12345 * wedge.scalar) - (other.e423 * wedge.e15) - (other.e431 * wedge.e25) - (other.e412 * wedge.e35) - (other.e415 * wedge.e23) - (other.e425 * wedge.e31) - (other.e435 * wedge.e12) - (other.e321 * wedge.e45) - (other.e235 * wedge.e41) - (other.e315 * wedge.e42) - (other.e125 * wedge.e43), other.e12345 * wedge.e12345), 
            /* e1, e2, e3, e4 */
            float4((other.e415 * wedge.e321) + (other.e321 * wedge.e415) + (other.e315 * wedge.e412) + (other.e12345 * wedge.e1), (other.e425 * wedge.e321) + (other.e321 * wedge.e425) + (other.e125 * wedge.e423) + (other.e12345 * wedge.e2), (other.e435 * wedge.e321) + (other.e321 * wedge.e435) + (other.e235 * wedge.e431) + (other.e12345 * wedge.e3), -(other.e412 * wedge.e435) - (other.e415 * wedge.e423) - (other.e425 * wedge.e431) - (other.e435 * wedge.e412)) + float4((other.group0.xyz.zxy * wedge.group8.xyz.yzx), other.e12345 * wedge.e4) - float4((other.group0.xyz.yzx * wedge.group8.xyz.zxy), other.e423 * wedge.e415) - float4((wedge.group7.xyz.yzx * other.group2.zxy), other.e431 * wedge.e425), 
            /* e5 */
            (other.e12345 * wedge.e5) - (other.e415 * wedge.e235) - (other.e425 * wedge.e315) - (other.e435 * wedge.e125) - (other.e235 * wedge.e415) - (other.e315 * wedge.e425) - (other.e125 * wedge.e435), 
            /* e15, e25, e35, e45 */
            float4((other.e315 * wedge.e4125) + (other.e12345 * wedge.e15), (other.e125 * wedge.e4235) + (other.e12345 * wedge.e25), (other.e235 * wedge.e4315) + (other.e12345 * wedge.e35), -(other.e425 * wedge.e4315) - (other.e435 * wedge.e4125)) + float4((wedge.group9.www * other.group1.xyz), other.e12345 * wedge.e45) - (wedge.group9.yzxx * float4(other.group2.zxy, other.e415)), 
            /* e41, e42, e43 */
            (float3(other.e12345) * wedge.group4.xyz) + (float3(wedge.e1234) * other.group1.xyz) + (other.group0.xyz.zxy * wedge.group9.yzx) - (other.group0.xyz.yzx * wedge.group9.zxy), 
            /* e23, e31, e12 */
            (float3(other.e12345) * wedge.group5.xyz) + (float3(wedge.e3215) * other.group0.xyz) + (float3(wedge.e1234) * other.group2.xyz) - (float3(other.e321) * wedge.group9.xyz), 
            /* e415, e425, e435, e321 */
            (float4(other.e12345) * wedge.group6) + (float4(wedge.e12345) * other.group1), 
            /* e423, e431, e412 */
            (float3(other.e12345) * wedge.group7.xyz) + (float3(wedge.e12345) * other.group0.xyz), 
            /* e235, e315, e125 */
            (float3(other.e12345) * wedge.group8.xyz) + (float3(wedge.e12345) * other.group2.xyz), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345) * wedge.group9, 
            /* e1234 */
            other.e12345 * wedge.e1234
        );
    }
}
public extension MultiVector {
    public MultiVector project_orthogonally_onto(Dipole other) {
        Circle right_anti_dual = Circle(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125 */
            other.group2.xyz * float3(-1.0)
        );
        MultiVector wedge = MultiVector(
            /* scalar, e12345 */
            float2(0.0, -(right_anti_dual.e423 * this.e15) - (right_anti_dual.e431 * this.e25) - (right_anti_dual.e412 * this.e35) - (right_anti_dual.e415 * this.e23) - (right_anti_dual.e425 * this.e31) - (right_anti_dual.e435 * this.e12) - (right_anti_dual.e321 * this.e45) - (right_anti_dual.e235 * this.e41) - (right_anti_dual.e315 * this.e42) - (right_anti_dual.e125 * this.e43)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(this.scalar) * right_anti_dual.group1, 
            /* e423, e431, e412 */
            float3(this.scalar) * right_anti_dual.group0.xyz, 
            /* e235, e315, e125 */
            float3(this.scalar) * right_anti_dual.group2.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual.e425 * this.e3) + (right_anti_dual.e235 * this.e4), (right_anti_dual.e435 * this.e1) + (right_anti_dual.e315 * this.e4), (right_anti_dual.e415 * this.e2) + (right_anti_dual.e125 * this.e4), -(right_anti_dual.e321 * this.e5) - (right_anti_dual.e125 * this.e3)) - (this.group1.yzxy * float4(right_anti_dual.group1.zxy, right_anti_dual.e315)) - float4((float3(this.e5) * right_anti_dual.group0.xyz), right_anti_dual.e235 * this.e1), 
            /* e1234 */
            (right_anti_dual.e423 * this.e1) + (right_anti_dual.e431 * this.e2) + (right_anti_dual.e412 * this.e3) + (right_anti_dual.e321 * this.e4)
        );
        return MultiVector(
            /* scalar, e12345 */
            float2(-(other.e41 * wedge.e235) - (other.e42 * wedge.e315) - (other.e43 * wedge.e125) - (other.e23 * wedge.e415) - (other.e31 * wedge.e425) - (other.e12 * wedge.e435) - (other.e45 * wedge.e321) - (other.e15 * wedge.e423) - (other.e25 * wedge.e431) - (other.e35 * wedge.e412), 0.0), 
            /* e1, e2, e3, e4 */
            float4((other.e41 * wedge.e3215) + (other.e31 * wedge.e4125), (other.e42 * wedge.e3215) + (other.e12 * wedge.e4235), (other.e43 * wedge.e3215) + (other.e23 * wedge.e4315), -(other.e43 * wedge.e4125) - (other.e45 * wedge.e1234)) - (wedge.group9.yzxy * float4(other.group1.zxy, other.e42)) - float4((float3(wedge.e1234) * other.group2.xyz), other.e41 * wedge.e4235), 
            /* e5 */
            (other.e45 * wedge.e3215) + (other.e15 * wedge.e4235) + (other.e25 * wedge.e4315) + (other.e35 * wedge.e4125), 
            /* e15, e25, e35, e45 */
            float4(wedge.e12345) * float4(other.group2.xyz, other.e45), 
            /* e41, e42, e43 */
            float3(wedge.e12345) * other.group0.xyz, 
            /* e23, e31, e12 */
            float3(wedge.e12345) * other.group1.xyz, 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
public extension MultiVector {
    public MultiVector project_orthogonally_onto(DipoleInversion other) {
        AntiDipoleInversion right_anti_dual = AntiDipoleInversion(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e4 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group3 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        MultiVector wedge = MultiVector(
            /* scalar, e12345 */
            float2(0.0, (right_anti_dual.e4 * this.e3215) + (right_anti_dual.e1 * this.e4235) + (right_anti_dual.e2 * this.e4315) + (right_anti_dual.e3 * this.e4125) + (right_anti_dual.e5 * this.e1234) - (right_anti_dual.e423 * this.e15) - (right_anti_dual.e431 * this.e25) - (right_anti_dual.e412 * this.e35) - (right_anti_dual.e415 * this.e23) - (right_anti_dual.e425 * this.e31) - (right_anti_dual.e435 * this.e12) - (right_anti_dual.e321 * this.e45) - (right_anti_dual.e235 * this.e41) - (right_anti_dual.e315 * this.e42) - (right_anti_dual.e125 * this.e43)), 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * float4(right_anti_dual.group3.xyz, right_anti_dual.e4), 
            /* e5 */
            right_anti_dual.e5 * this.scalar, 
            /* e15, e25, e35, e45 */
            (float4(right_anti_dual.e5) * this.group1) - (float4(this.e5) * float4(right_anti_dual.group3.xyz, right_anti_dual.e4)), 
            /* e41, e42, e43 */
            (float3(this.e4) * right_anti_dual.group3.xyz) - (float3(right_anti_dual.e4) * this.group1.xyz), 
            /* e23, e31, e12 */
            (right_anti_dual.group3.zxy * this.group1.yzx) - (right_anti_dual.group3.yzx * this.group1.zxy), 
            /* e415, e425, e435, e321 */
            float4((right_anti_dual.e4 * this.e15) + (right_anti_dual.e5 * this.e41), (right_anti_dual.e4 * this.e25) + (right_anti_dual.e5 * this.e42), (right_anti_dual.e4 * this.e35) + (right_anti_dual.e5 * this.e43), -(right_anti_dual.e2 * this.e31) - (right_anti_dual.e3 * this.e12)) + (float4(this.scalar) * right_anti_dual.group1) - float4((this.group3.www * right_anti_dual.group3.xyz), right_anti_dual.e1 * this.e23), 
            /* e423, e431, e412 */
            (float3(right_anti_dual.e4) * this.group5.xyz) + (float3(this.scalar) * right_anti_dual.group0.xyz) + (this.group4.xyz.yzx * right_anti_dual.group3.zxy) - (this.group4.xyz.zxy * right_anti_dual.group3.yzx), 
            /* e235, e315, e125 */
            (float3(right_anti_dual.e5) * this.group5.xyz) + (float3(this.scalar) * right_anti_dual.group2.xyz) + (right_anti_dual.group3.yzx * this.group3.zxy) - (right_anti_dual.group3.zxy * this.group3.yzx), 
            /* e4235, e4315, e4125, e3215 */
            (right_anti_dual.group3.yzxw * this.group6.zxyw) + (right_anti_dual.group3.wwwx * float4(this.group7.xyz, this.e235)) + float4((right_anti_dual.group1.yzx * this.group1.zxy), right_anti_dual.e2 * this.e315) + float4((this.group1.www * right_anti_dual.group2.xyz), right_anti_dual.e3 * this.e125) - (float4(this.e5) * float4(right_anti_dual.group0.xyz, right_anti_dual.e321)) - (this.group1.yzxy * float4(right_anti_dual.group1.zxy, right_anti_dual.e315)) - float4((this.group8.xyz * right_anti_dual.group2.www), right_anti_dual.e235 * this.e1) - float4((right_anti_dual.group3.zxy * this.group6.yzx), right_anti_dual.e125 * this.e3), 
            /* e1234 */
            (right_anti_dual.e423 * this.e1) + (right_anti_dual.e431 * this.e2) + (right_anti_dual.e412 * this.e3) + (right_anti_dual.e321 * this.e4) - (right_anti_dual.e4 * this.e321) - (right_anti_dual.e1 * this.e423) - (right_anti_dual.e2 * this.e431) - (right_anti_dual.e3 * this.e412)
        );
        return MultiVector(
            /* scalar, e12345 */
            float2((other.e1234 * wedge.e5) + (other.e4235 * wedge.e1) + (other.e4315 * wedge.e2) + (other.e4125 * wedge.e3) + (other.e3215 * wedge.e4) - (other.e41 * wedge.e235) - (other.e42 * wedge.e315) - (other.e43 * wedge.e125) - (other.e23 * wedge.e415) - (other.e31 * wedge.e425) - (other.e12 * wedge.e435) - (other.e45 * wedge.e321) - (other.e15 * wedge.e423) - (other.e25 * wedge.e431) - (other.e35 * wedge.e412), 0.0), 
            /* e1, e2, e3, e4 */
            (float4(other.e1234) * wedge.group3) + (other.group3.yzxy * float4(wedge.group5.xyz.zxy, wedge.e42)) + float4((other.group0.xyz * wedge.group9.www), other.e4235 * wedge.e41) + float4((other.group1.yzx * wedge.group9.zxy), other.e4125 * wedge.e43) - (float4(wedge.e1234) * float4(other.group2.xyz, other.e45)) - (wedge.group9.yzxz * float4(other.group1.zxy, other.e43)) - float4((wedge.group4.xyz * other.group3.www), other.e41 * wedge.e4235) - float4((wedge.group5.xyz.yzx * other.group3.zxy), other.e42 * wedge.e4315), 
            /* e5 */
            (other.e45 * wedge.e3215) + (other.e15 * wedge.e4235) + (other.e25 * wedge.e4315) + (other.e35 * wedge.e4125) - (other.e4235 * wedge.e15) - (other.e4315 * wedge.e25) - (other.e4125 * wedge.e35) - (other.e3215 * wedge.e45), 
            /* e15, e25, e35, e45 */
            float4((other.e4125 * wedge.e315) + (other.e3215 * wedge.e415), (other.e4235 * wedge.e125) + (other.e3215 * wedge.e425), (other.e4315 * wedge.e235) + (other.e3215 * wedge.e435), -(other.e4315 * wedge.e425) - (other.e4125 * wedge.e435)) + (float4(wedge.e12345) * float4(other.group2.xyz, other.e45)) - float4((wedge.group8.xyz.zxy * other.group3.yzx), other.e4235 * wedge.e415), 
            /* e41, e42, e43 */
            (float3(other.e1234) * wedge.group6.xyz) + (float3(wedge.e12345) * other.group0.xyz) + (wedge.group7.xyz.zxy * other.group3.yzx) - (wedge.group7.xyz.yzx * other.group3.zxy), 
            /* e23, e31, e12 */
            (float3(other.e1234) * wedge.group8.xyz) + (float3(other.e3215) * wedge.group7.xyz) + (float3(wedge.e12345) * other.group1.xyz) - (float3(wedge.e321) * other.group3.xyz), 
            /* e415, e425, e435, e321 */
            (wedge.group9.yzxw * float4(other.group3.zxy, other.e1234)) - (other.group3.yzxw * float4(wedge.group9.zxy, wedge.e1234)), 
            /* e423, e431, e412 */
            (float3(other.e1234) * wedge.group9.xyz) - (float3(wedge.e1234) * other.group3.xyz), 
            /* e235, e315, e125 */
            (float3(wedge.e3215) * other.group3.xyz) - (float3(other.e3215) * wedge.group9.xyz), 
            /* e4235, e4315, e4125, e3215 */
            float4(wedge.e12345) * other.group3, 
            /* e1234 */
            other.e1234 * wedge.e12345
        );
    }
}
public extension MultiVector {
    public MultiVector project_orthogonally_onto(DualNum other) {
        AntiDualNum right_anti_dual = AntiDualNum(
            /* e3215, scalar */
            other.group0.xy * float2(-1.0)
        );
        MultiVector wedge = MultiVector(
            /* scalar, e12345 */
            float2(right_anti_dual.scalar * this.scalar, (right_anti_dual.e3215 * this.e4) + (right_anti_dual.scalar * this.e12345)), 
            /* e1, e2, e3, e4 */
            float4(right_anti_dual.scalar) * this.group1, 
            /* e5 */
            right_anti_dual.scalar * this.e5, 
            /* e15, e25, e35, e45 */
            float4(right_anti_dual.scalar) * this.group3, 
            /* e41, e42, e43 */
            float3(right_anti_dual.scalar) * this.group4.xyz, 
            /* e23, e31, e12 */
            float3(right_anti_dual.scalar) * this.group5.xyz, 
            /* e415, e425, e435, e321 */
            float4(right_anti_dual.scalar) * this.group6, 
            /* e423, e431, e412 */
            float3(right_anti_dual.scalar) * this.group7.xyz, 
            /* e235, e315, e125 */
            float3(right_anti_dual.scalar) * this.group8.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual.group0.xy.yy, right_anti_dual.scalar, (right_anti_dual.e3215 * this.scalar) + (right_anti_dual.scalar * this.e3215)) * float4(this.group9.xyz, 1.0), 
            /* e1234 */
            right_anti_dual.scalar * this.e1234
        );
        return MultiVector(
            /* scalar, e12345 */
            float2((other.e5 * wedge.e1234) + (other.e12345 * wedge.scalar), other.e12345 * wedge.e12345), 
            /* e1, e2, e3, e4 */
            float4(other.e12345) * wedge.group1, 
            /* e5 */
            (other.e5 * wedge.e12345) + (other.e12345 * wedge.e5), 
            /* e15, e25, e35, e45 */
            float4(other.e12345) * wedge.group3, 
            /* e41, e42, e43 */
            float3(other.e12345) * wedge.group4.xyz, 
            /* e23, e31, e12 */
            float3(other.e12345) * wedge.group5.xyz, 
            /* e415, e425, e435, e321 */
            float4(other.e12345) * wedge.group6, 
            /* e423, e431, e412 */
            float3(other.e12345) * wedge.group7.xyz, 
            /* e235, e315, e125 */
            float3(other.e12345) * wedge.group8.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345) * wedge.group9, 
            /* e1234 */
            other.e12345 * wedge.e1234
        );
    }
}
public extension MultiVector {
    public MultiVector project_orthogonally_onto(FlatPoint other) {
        AntiFlatPoint right_anti_dual = AntiFlatPoint(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        MultiVector wedge = MultiVector(
            /* scalar, e12345 */
            float2(0.0, -(right_anti_dual.e235 * this.e41) - (right_anti_dual.e315 * this.e42) - (right_anti_dual.e125 * this.e43) - (right_anti_dual.e321 * this.e45)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), right_anti_dual.e321 * this.scalar), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(this.scalar) * right_anti_dual.group0.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4((float3(this.e4) * right_anti_dual.group0.xyz), -(right_anti_dual.e235 * this.e1) - (right_anti_dual.e315 * this.e2) - (right_anti_dual.e125 * this.e3) - (right_anti_dual.e321 * this.e5)), 
            /* e1234 */
            right_anti_dual.e321 * this.e4
        );
        return MultiVector(
            /* scalar, e12345 */
            float2(-(other.e15 * wedge.e423) - (other.e25 * wedge.e431) - (other.e35 * wedge.e412) - (other.e45 * wedge.e321), 0.0), 
            /* e1, e2, e3, e4 */
            float4(wedge.e1234) * other.group0 * float4(-1.0), 
            /* e5 */
            (other.e15 * wedge.e4235) + (other.e25 * wedge.e4315) + (other.e35 * wedge.e4125) + (other.e45 * wedge.e3215), 
            /* e15, e25, e35, e45 */
            float4(wedge.e12345) * other.group0, 
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
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
public extension MultiVector {
    public MultiVector project_orthogonally_onto(Flector other) {
        AntiFlector right_anti_dual = AntiFlector(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        MultiVector wedge = MultiVector(
            /* scalar, e12345 */
            float2(0.0, (right_anti_dual.e1 * this.e4235) + (right_anti_dual.e2 * this.e4315) + (right_anti_dual.e3 * this.e4125) + (right_anti_dual.e5 * this.e1234) - (right_anti_dual.e235 * this.e41) - (right_anti_dual.e315 * this.e42) - (right_anti_dual.e125 * this.e43) - (right_anti_dual.e321 * this.e45)), 
            /* e1, e2, e3, e4 */
            float4(this.group0.xy.xx, this.scalar, 0.0) * float4(float3(1.0), 0.0) * float4(right_anti_dual.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e5 */
            right_anti_dual.e5 * this.scalar, 
            /* e15, e25, e35, e45 */
            float4(((float3(right_anti_dual.e5) * this.group1.xyz) - (float3(this.e5) * right_anti_dual.group1.xyz)), right_anti_dual.e5 * this.e4), 
            /* e41, e42, e43 */
            float3(this.e4) * right_anti_dual.group1.xyz, 
            /* e23, e31, e12 */
            (right_anti_dual.group1.zxy * this.group1.yzx) - (right_anti_dual.group1.yzx * this.group1.zxy), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), -(right_anti_dual.e2 * this.e31) - (right_anti_dual.e3 * this.e12)) + float4((this.group4.xyz * right_anti_dual.group1.www), right_anti_dual.e321 * this.scalar) - float4((this.group3.www * right_anti_dual.group1.xyz), right_anti_dual.e1 * this.e23), 
            /* e423, e431, e412 */
            (this.group4.xyz.yzx * right_anti_dual.group1.zxy) - (this.group4.xyz.zxy * right_anti_dual.group1.yzx), 
            /* e235, e315, e125 */
            (float3(right_anti_dual.e5) * this.group5.xyz) + (float3(this.scalar) * right_anti_dual.group0.xyz) + (right_anti_dual.group1.yzx * this.group3.zxy) - (right_anti_dual.group1.zxy * this.group3.yzx), 
            /* e4235, e4315, e4125, e3215 */
            (right_anti_dual.group1.wwwx * float4(this.group7.xyz, this.e235)) + float4(float3(0.0), (right_anti_dual.e5 * this.e321) - (right_anti_dual.e315 * this.e2) - (right_anti_dual.e125 * this.e3) - (right_anti_dual.e321 * this.e5)) + float4((right_anti_dual.group1.yzx * this.group6.zxy), right_anti_dual.e3 * this.e125) + float4((this.group1.www * right_anti_dual.group0.xyz), right_anti_dual.e2 * this.e315) - float4((right_anti_dual.group1.zxy * this.group6.yzx), right_anti_dual.e235 * this.e1), 
            /* e1234 */
            (right_anti_dual.e321 * this.e4) - (right_anti_dual.e1 * this.e423) - (right_anti_dual.e2 * this.e431) - (right_anti_dual.e3 * this.e412)
        );
        return MultiVector(
            /* scalar, e12345 */
            float2((other.e4235 * wedge.e1) + (other.e4315 * wedge.e2) + (other.e4125 * wedge.e3) + (other.e3215 * wedge.e4) - (other.e15 * wedge.e423) - (other.e25 * wedge.e431) - (other.e35 * wedge.e412) - (other.e45 * wedge.e321), 0.0), 
            /* e1, e2, e3, e4 */
            float4(-(other.e15 * wedge.e1234) - (other.e4125 * wedge.e31), -(other.e25 * wedge.e1234) - (other.e4235 * wedge.e12), -(other.e35 * wedge.e1234) - (other.e4315 * wedge.e23), (other.e4315 * wedge.e42) + (other.e4125 * wedge.e43)) + (other.group1.yzxx * float4(wedge.group5.xyz.zxy, wedge.e41)) - float4((wedge.group4.xyz * other.group1.www), other.e45 * wedge.e1234), 
            /* e5 */
            (other.e15 * wedge.e4235) + (other.e25 * wedge.e4315) + (other.e35 * wedge.e4125) + (other.e45 * wedge.e3215) - (other.e4235 * wedge.e15) - (other.e4315 * wedge.e25) - (other.e4125 * wedge.e35) - (other.e3215 * wedge.e45), 
            /* e15, e25, e35, e45 */
            float4((other.e4125 * wedge.e315) + (other.e3215 * wedge.e415), (other.e4235 * wedge.e125) + (other.e3215 * wedge.e425), (other.e4315 * wedge.e235) + (other.e3215 * wedge.e435), -(other.e4315 * wedge.e425) - (other.e4125 * wedge.e435)) + (float4(wedge.e12345) * other.group0) - float4((wedge.group8.xyz.zxy * other.group1.yzx), other.e4235 * wedge.e415), 
            /* e41, e42, e43 */
            (wedge.group7.xyz.zxy * other.group1.yzx) - (wedge.group7.xyz.yzx * other.group1.zxy), 
            /* e23, e31, e12 */
            (float3(other.e3215) * wedge.group7.xyz) - (float3(wedge.e321) * other.group1.xyz), 
            /* e415, e425, e435, e321 */
            float4(((other.group1.zxy * wedge.group9.yzx) - (other.group1.yzx * wedge.group9.zxy)), other.e3215 * wedge.e1234 * -1.0), 
            /* e423, e431, e412 */
            float3(wedge.e1234) * other.group1.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            (float3(wedge.e3215) * other.group1.xyz) - (float3(other.e3215) * wedge.group9.xyz), 
            /* e4235, e4315, e4125, e3215 */
            float4(wedge.e12345) * other.group1, 
            /* e1234 */
            0.0
        );
    }
}
public extension MultiVector {
    public MultiVector project_orthogonally_onto(Line other) {
        AntiLine right_anti_dual = AntiLine(
            /* e23, e31, e12 */
            other.group0.xyz, 
            /* e15, e25, e35 */
            other.group1.xyz
        );
        MultiVector wedge = MultiVector(
            /* scalar, e12345 */
            float2(0.0, -(right_anti_dual.e23 * this.e415) - (right_anti_dual.e31 * this.e425) - (right_anti_dual.e12 * this.e435) - (right_anti_dual.e15 * this.e423) - (right_anti_dual.e25 * this.e431) - (right_anti_dual.e35 * this.e412)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            float4(this.group0.xy.xx, this.scalar, 0.0) * float4(float3(1.0), 0.0) * float4(right_anti_dual.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(this.scalar) * right_anti_dual.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4((float3(this.e4) * right_anti_dual.group1.xyz), -(right_anti_dual.e23 * this.e1) - (right_anti_dual.e31 * this.e2) - (right_anti_dual.e12 * this.e3)), 
            /* e423, e431, e412 */
            float3(this.e4) * right_anti_dual.group0.xyz, 
            /* e235, e315, e125 */
            (float3(this.e5) * right_anti_dual.group0.xyz) + (right_anti_dual.group1.xyz.zxy * this.group1.yzx) - (right_anti_dual.group1.xyz.yzx * this.group1.zxy), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual.e23 * this.e45) + (right_anti_dual.e35 * this.e42), (right_anti_dual.e31 * this.e45) + (right_anti_dual.e15 * this.e43), (right_anti_dual.e12 * this.e45) + (right_anti_dual.e25 * this.e41), -(right_anti_dual.e31 * this.e25) - (right_anti_dual.e12 * this.e35) - (right_anti_dual.e15 * this.e23) - (right_anti_dual.e25 * this.e31) - (right_anti_dual.e35 * this.e12)) - float4((right_anti_dual.group1.xyz.yzx * this.group4.xyz.zxy), right_anti_dual.e23 * this.e15), 
            /* e1234 */
            -(right_anti_dual.e23 * this.e41) - (right_anti_dual.e31 * this.e42) - (right_anti_dual.e12 * this.e43)
        );
        return MultiVector(
            /* scalar, e12345 */
            float2(-(other.e415 * wedge.e23) - (other.e425 * wedge.e31) - (other.e435 * wedge.e12) - (other.e235 * wedge.e41) - (other.e315 * wedge.e42) - (other.e125 * wedge.e43), 0.0), 
            /* e1, e2, e3, e4 */
            float4((other.e415 * wedge.e321) + (other.e315 * wedge.e412), (other.e425 * wedge.e321) + (other.e125 * wedge.e423), (other.e435 * wedge.e321) + (other.e235 * wedge.e431), -(other.e425 * wedge.e431) - (other.e435 * wedge.e412)) - float4((other.group1.xyz.zxy * wedge.group7.xyz.yzx), other.e415 * wedge.e423), 
            /* e5 */
            -(other.e415 * wedge.e235) - (other.e425 * wedge.e315) - (other.e435 * wedge.e125) - (other.e235 * wedge.e415) - (other.e315 * wedge.e425) - (other.e125 * wedge.e435), 
            /* e15, e25, e35, e45 */
            float4((other.e415 * wedge.e3215) + (other.e315 * wedge.e4125), (other.e425 * wedge.e3215) + (other.e125 * wedge.e4235), (other.e435 * wedge.e3215) + (other.e235 * wedge.e4315), -(other.e425 * wedge.e4315) - (other.e435 * wedge.e4125)) - (wedge.group9.yzxx * float4(other.group1.xyz.zxy, other.e415)), 
            /* e41, e42, e43 */
            float3(wedge.e1234) * other.group0.xyz, 
            /* e23, e31, e12 */
            float3(wedge.e1234) * other.group1.xyz, 
            /* e415, e425, e435, e321 */
            float4(wedge.group0.xy.yy, wedge.e12345, 0.0) * float4(float3(1.0), 0.0) * float4(other.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(wedge.e12345) * other.group1.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
public extension MultiVector {
    public MultiVector project_orthogonally_onto(Motor other) {
        AntiMotor right_anti_dual = AntiMotor(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        MultiVector wedge = MultiVector(
            /* scalar, e12345 */
            float2(right_anti_dual.scalar * this.scalar, (right_anti_dual.scalar * this.e12345) + (right_anti_dual.e3215 * this.e4) - (right_anti_dual.e23 * this.e415) - (right_anti_dual.e31 * this.e425) - (right_anti_dual.e12 * this.e435) - (right_anti_dual.e15 * this.e423) - (right_anti_dual.e25 * this.e431) - (right_anti_dual.e35 * this.e412)), 
            /* e1, e2, e3, e4 */
            float4(right_anti_dual.scalar) * this.group1, 
            /* e5 */
            right_anti_dual.scalar * this.e5, 
            /* e15, e25, e35, e45 */
            float4(((float3(right_anti_dual.scalar) * this.group3.xyz) + (float3(this.scalar) * right_anti_dual.group1.xyz)), right_anti_dual.scalar * this.e45), 
            /* e41, e42, e43 */
            float3(right_anti_dual.scalar) * this.group4.xyz, 
            /* e23, e31, e12 */
            (float3(right_anti_dual.scalar) * this.group5.xyz) + (float3(this.scalar) * right_anti_dual.group0.xyz), 
            /* e415, e425, e435, e321 */
            float4(right_anti_dual.e15 * this.e4, right_anti_dual.e25 * this.e4, right_anti_dual.e35 * this.e4, -(right_anti_dual.e23 * this.e1) - (right_anti_dual.e31 * this.e2) - (right_anti_dual.e12 * this.e3)) + (float4(right_anti_dual.scalar) * this.group6), 
            /* e423, e431, e412 */
            (float3(right_anti_dual.scalar) * this.group7.xyz) + (float3(this.e4) * right_anti_dual.group0.xyz), 
            /* e235, e315, e125 */
            (float3(right_anti_dual.scalar) * this.group8.xyz) + (float3(this.e5) * right_anti_dual.group0.xyz) + (right_anti_dual.group1.zxy * this.group1.yzx) - (right_anti_dual.group1.yzx * this.group1.zxy), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual.scalar * this.e4235, right_anti_dual.scalar * this.e4315, right_anti_dual.scalar * this.e4125, -(right_anti_dual.e23 * this.e15) - (right_anti_dual.e31 * this.e25) - (right_anti_dual.e12 * this.e35) - (right_anti_dual.e25 * this.e31) - (right_anti_dual.e35 * this.e12)) + (right_anti_dual.group0 * float4(this.group3.www, this.e3215)) + (right_anti_dual.group1.zxyw * float4(this.group4.xyz.yzx, this.scalar)) - (right_anti_dual.group1.yzxx * float4(this.group4.xyz.zxy, this.e23)), 
            /* e1234 */
            (right_anti_dual.scalar * this.e1234) - (right_anti_dual.e23 * this.e41) - (right_anti_dual.e31 * this.e42) - (right_anti_dual.e12 * this.e43)
        );
        return MultiVector(
            /* scalar, e12345 */
            float2((other.e12345 * wedge.scalar) + (other.e5 * wedge.e1234) - (other.e415 * wedge.e23) - (other.e425 * wedge.e31) - (other.e435 * wedge.e12) - (other.e235 * wedge.e41) - (other.e315 * wedge.e42) - (other.e125 * wedge.e43), other.e12345 * wedge.e12345), 
            /* e1, e2, e3, e4 */
            float4((other.e415 * wedge.e321) + (other.e12345 * wedge.e1), (other.e425 * wedge.e321) + (other.e12345 * wedge.e2), (other.e435 * wedge.e321) + (other.e12345 * wedge.e3), -(other.e425 * wedge.e431) - (other.e435 * wedge.e412)) + float4((wedge.group7.xyz.zxy * other.group1.yzx), other.e12345 * wedge.e4) - float4((wedge.group7.xyz.yzx * other.group1.zxy), other.e415 * wedge.e423), 
            /* e5 */
            (other.e12345 * wedge.e5) + (other.e5 * wedge.e12345) - (other.e415 * wedge.e235) - (other.e425 * wedge.e315) - (other.e435 * wedge.e125) - (other.e235 * wedge.e415) - (other.e315 * wedge.e425) - (other.e125 * wedge.e435), 
            /* e15, e25, e35, e45 */
            float4((other.e12345 * wedge.e15) + (other.e315 * wedge.e4125), (other.e12345 * wedge.e25) + (other.e125 * wedge.e4235), (other.e12345 * wedge.e35) + (other.e235 * wedge.e4315), -(other.e425 * wedge.e4315) - (other.e435 * wedge.e4125)) + (other.group0 * float4(wedge.group9.www, wedge.e45)) - (wedge.group9.yzxx * float4(other.group1.zxy, other.e415)), 
            /* e41, e42, e43 */
            (float3(other.e12345) * wedge.group4.xyz) + (float3(wedge.e1234) * other.group0.xyz), 
            /* e23, e31, e12 */
            (float3(other.e12345) * wedge.group5.xyz) + (float3(wedge.e1234) * other.group1.xyz), 
            /* e415, e425, e435, e321 */
            float4(((float3(other.e12345) * wedge.group6.xyz) + (float3(wedge.e12345) * other.group0.xyz)), other.e12345 * wedge.e321), 
            /* e423, e431, e412 */
            float3(other.e12345) * wedge.group7.xyz, 
            /* e235, e315, e125 */
            (float3(other.e12345) * wedge.group8.xyz) + (float3(wedge.e12345) * other.group1.xyz), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345) * wedge.group9, 
            /* e1234 */
            other.e12345 * wedge.e1234
        );
    }
}
public extension MultiVector {
    public MultiVector project_orthogonally_onto(MultiVector other) {
        MultiVector right_anti_dual = MultiVector(
            /* scalar, e12345 */
            other.group0.xy.yx * float2(-1.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4(other.group9.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5 */
            other.e3215, 
            /* e15, e25, e35, e45 */
            float4(other.group8.xyz, other.e321 * -1.0), 
            /* e41, e42, e43 */
            other.group7.xyz, 
            /* e23, e31, e12 */
            other.group6.xyz, 
            /* e415, e425, e435, e321 */
            float4(other.group5.xyz, other.e45) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e423, e431, e412 */
            other.group4.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            other.group3.xyz * float3(-1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.group1.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234 */
            other.e4 * -1.0
        );
        MultiVector wedge = MultiVector(
            /* scalar, e12345 */
            float2(right_anti_dual.scalar * this.scalar, (right_anti_dual.scalar * this.e12345) + (right_anti_dual.e12345 * this.scalar) + (right_anti_dual.e1 * this.e4235) + (right_anti_dual.e2 * this.e4315) + (right_anti_dual.e3 * this.e4125) + (right_anti_dual.e4 * this.e3215) + (right_anti_dual.e5 * this.e1234) + (right_anti_dual.e4235 * this.e1) + (right_anti_dual.e4315 * this.e2) + (right_anti_dual.e4125 * this.e3) + (right_anti_dual.e3215 * this.e4) + (right_anti_dual.e1234 * this.e5) - (right_anti_dual.e15 * this.e423) - (right_anti_dual.e25 * this.e431) - (right_anti_dual.e35 * this.e412) - (right_anti_dual.e45 * this.e321) - (right_anti_dual.e41 * this.e235) - (right_anti_dual.e42 * this.e315) - (right_anti_dual.e43 * this.e125) - (right_anti_dual.e23 * this.e415) - (right_anti_dual.e31 * this.e425) - (right_anti_dual.e12 * this.e435) - (right_anti_dual.e415 * this.e23) - (right_anti_dual.e425 * this.e31) - (right_anti_dual.e435 * this.e12) - (right_anti_dual.e321 * this.e45) - (right_anti_dual.e423 * this.e15) - (right_anti_dual.e431 * this.e25) - (right_anti_dual.e412 * this.e35) - (right_anti_dual.e235 * this.e41) - (right_anti_dual.e315 * this.e42) - (right_anti_dual.e125 * this.e43)), 
            /* e1, e2, e3, e4 */
            (float4(right_anti_dual.scalar) * this.group1) + (float4(this.scalar) * right_anti_dual.group1), 
            /* e5 */
            (right_anti_dual.scalar * this.e5) + (right_anti_dual.e5 * this.scalar), 
            /* e15, e25, e35, e45 */
            (float4(right_anti_dual.scalar) * this.group3) + (float4(right_anti_dual.e5) * this.group1) + (float4(this.scalar) * right_anti_dual.group3) - (float4(this.e5) * right_anti_dual.group1), 
            /* e41, e42, e43 */
            (float3(right_anti_dual.scalar) * this.group4.xyz) + (float3(this.scalar) * right_anti_dual.group4.xyz) + (float3(this.e4) * right_anti_dual.group1.xyz) - (float3(right_anti_dual.e4) * this.group1.xyz), 
            /* e23, e31, e12 */
            (float3(right_anti_dual.scalar) * this.group5.xyz) + (float3(this.scalar) * right_anti_dual.group5.xyz) + (right_anti_dual.group1.zxy * this.group1.yzx) - (right_anti_dual.group1.yzx * this.group1.zxy), 
            /* e415, e425, e435, e321 */
            float4((right_anti_dual.e4 * this.e15) + (right_anti_dual.e5 * this.e41) + (right_anti_dual.e15 * this.e4) + (right_anti_dual.e41 * this.e5), (right_anti_dual.e4 * this.e25) + (right_anti_dual.e5 * this.e42) + (right_anti_dual.e25 * this.e4) + (right_anti_dual.e42 * this.e5), (right_anti_dual.e4 * this.e35) + (right_anti_dual.e5 * this.e43) + (right_anti_dual.e35 * this.e4) + (right_anti_dual.e43 * this.e5), -(right_anti_dual.e1 * this.e23) - (right_anti_dual.e2 * this.e31) - (right_anti_dual.e3 * this.e12) - (right_anti_dual.e12 * this.e3)) + (float4(right_anti_dual.scalar) * this.group6) + (float4(this.scalar) * right_anti_dual.group6) - (this.group1.xyzy * float4(right_anti_dual.group3.www, right_anti_dual.e31)) - float4((this.group3.www * right_anti_dual.group1.xyz), right_anti_dual.e23 * this.e1), 
            /* e423, e431, e412 */
            (float3(right_anti_dual.scalar) * this.group7.xyz) + (float3(right_anti_dual.e4) * this.group5.xyz) + (float3(this.scalar) * right_anti_dual.group7.xyz) + (float3(this.e4) * right_anti_dual.group5.xyz) + (right_anti_dual.group4.xyz.yzx * this.group1.zxy) + (this.group4.xyz.yzx * right_anti_dual.group1.zxy) - (right_anti_dual.group4.xyz.zxy * this.group1.yzx) - (this.group4.xyz.zxy * right_anti_dual.group1.yzx), 
            /* e235, e315, e125 */
            (float3(right_anti_dual.scalar) * this.group8.xyz) + (float3(right_anti_dual.e5) * this.group5.xyz) + (float3(this.scalar) * right_anti_dual.group8.xyz) + (float3(this.e5) * right_anti_dual.group5.xyz) + (right_anti_dual.group1.yzx * this.group3.zxy) + (right_anti_dual.group3.zxy * this.group1.yzx) - (right_anti_dual.group1.zxy * this.group3.yzx) - (right_anti_dual.group3.yzx * this.group1.zxy), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual.e2 * this.e435) + (right_anti_dual.e5 * this.e423) + (right_anti_dual.e45 * this.e23) + (right_anti_dual.e425 * this.e3), (right_anti_dual.e3 * this.e415) + (right_anti_dual.e5 * this.e431) + (right_anti_dual.e45 * this.e31) + (right_anti_dual.e435 * this.e1), (right_anti_dual.e1 * this.e425) + (right_anti_dual.e5 * this.e412) + (right_anti_dual.e45 * this.e12) + (right_anti_dual.e415 * this.e2), -(right_anti_dual.e15 * this.e23) - (right_anti_dual.e25 * this.e31) - (right_anti_dual.e35 * this.e12) - (right_anti_dual.e321 * this.e5)) + (float4(right_anti_dual.scalar) * this.group9) + (float4(this.scalar) * right_anti_dual.group9) + float4((right_anti_dual.group5.xyz * this.group3.www), right_anti_dual.e2 * this.e315) + float4((right_anti_dual.group8.xyz * this.group1.www), right_anti_dual.e3 * this.e125) + float4((right_anti_dual.group4.xyz.yzx * this.group3.zxy), right_anti_dual.e1 * this.e235) + float4((this.group4.xyz.yzx * right_anti_dual.group3.zxy), right_anti_dual.e5 * this.e321) - (this.group1.yzxz * float4(right_anti_dual.group6.zxy, right_anti_dual.e125)) - (this.group3.yzxx * float4(right_anti_dual.group4.xyz.zxy, right_anti_dual.e23)) - float4((float3(this.e5) * right_anti_dual.group7.xyz), right_anti_dual.e31 * this.e25) - float4((this.group8.xyz * right_anti_dual.group1.www), right_anti_dual.e235 * this.e1) - float4((this.group4.xyz.zxy * right_anti_dual.group3.yzx), right_anti_dual.e12 * this.e35) - float4((right_anti_dual.group1.zxy * this.group6.yzx), right_anti_dual.e315 * this.e2), 
            /* e1234 */
            (right_anti_dual.scalar * this.e1234) + (right_anti_dual.e321 * this.e4) + (right_anti_dual.e423 * this.e1) + (right_anti_dual.e431 * this.e2) + (right_anti_dual.e412 * this.e3) + (right_anti_dual.e1234 * this.scalar) - (right_anti_dual.e1 * this.e423) - (right_anti_dual.e2 * this.e431) - (right_anti_dual.e3 * this.e412) - (right_anti_dual.e4 * this.e321) - (right_anti_dual.e41 * this.e23) - (right_anti_dual.e42 * this.e31) - (right_anti_dual.e43 * this.e12) - (right_anti_dual.e23 * this.e41) - (right_anti_dual.e31 * this.e42) - (right_anti_dual.e12 * this.e43)
        );
        return MultiVector(
            /* scalar, e12345 */
            float2((other.scalar * wedge.e12345) + (other.e12345 * wedge.scalar) + (other.e1 * wedge.e4235) + (other.e2 * wedge.e4315) + (other.e3 * wedge.e4125) + (other.e4 * wedge.e3215) + (other.e5 * wedge.e1234) + (other.e4235 * wedge.e1) + (other.e4315 * wedge.e2) + (other.e4125 * wedge.e3) + (other.e3215 * wedge.e4) + (other.e1234 * wedge.e5) - (other.e15 * wedge.e423) - (other.e25 * wedge.e431) - (other.e35 * wedge.e412) - (other.e45 * wedge.e321) - (other.e41 * wedge.e235) - (other.e42 * wedge.e315) - (other.e43 * wedge.e125) - (other.e23 * wedge.e415) - (other.e31 * wedge.e425) - (other.e12 * wedge.e435) - (other.e415 * wedge.e23) - (other.e425 * wedge.e31) - (other.e435 * wedge.e12) - (other.e321 * wedge.e45) - (other.e423 * wedge.e15) - (other.e431 * wedge.e25) - (other.e412 * wedge.e35) - (other.e235 * wedge.e41) - (other.e315 * wedge.e42) - (other.e125 * wedge.e43), other.e12345 * wedge.e12345), 
            /* e1, e2, e3, e4 */
            float4((other.e415 * wedge.e321) + (other.e321 * wedge.e415) + (other.e4315 * wedge.e12) + (other.e1234 * wedge.e15), (other.e425 * wedge.e321) + (other.e321 * wedge.e425) + (other.e4125 * wedge.e23) + (other.e1234 * wedge.e25), (other.e435 * wedge.e321) + (other.e321 * wedge.e435) + (other.e4235 * wedge.e31) + (other.e1234 * wedge.e35), -(other.e45 * wedge.e1234) - (other.e415 * wedge.e423) - (other.e425 * wedge.e431) - (other.e435 * wedge.e412)) + (float4(other.e12345) * wedge.group1) + (float4(wedge.e12345) * other.group1) + float4((other.group4.xyz * wedge.group9.www), other.e4235 * wedge.e41) + float4((other.group5.xyz.yzx * wedge.group9.zxy), other.e4315 * wedge.e42) + float4((other.group7.xyz.zxy * wedge.group8.xyz.yzx), other.e4125 * wedge.e43) + float4((other.group8.xyz.yzx * wedge.group7.xyz.zxy), other.e1234 * wedge.e45) - (wedge.group9.yzxx * float4(other.group5.xyz.zxy, other.e41)) - float4((float3(wedge.e1234) * other.group3.xyz), other.e412 * wedge.e435) - float4((wedge.group4.xyz * other.group9.www), other.e423 * wedge.e415) - float4((other.group7.xyz.yzx * wedge.group8.xyz.zxy), other.e42 * wedge.e4315) - float4((other.group8.xyz.zxy * wedge.group7.xyz.yzx), other.e43 * wedge.e4125) - float4((wedge.group5.xyz.yzx * other.group9.zxy), other.e431 * wedge.e425), 
            /* e5 */
            (other.e12345 * wedge.e5) + (other.e5 * wedge.e12345) + (other.e15 * wedge.e4235) + (other.e25 * wedge.e4315) + (other.e35 * wedge.e4125) + (other.e45 * wedge.e3215) - (other.e415 * wedge.e235) - (other.e425 * wedge.e315) - (other.e435 * wedge.e125) - (other.e235 * wedge.e415) - (other.e315 * wedge.e425) - (other.e125 * wedge.e435) - (other.e4235 * wedge.e15) - (other.e4315 * wedge.e25) - (other.e4125 * wedge.e35) - (other.e3215 * wedge.e45), 
            /* e15, e25, e35, e45 */
            float4((other.e415 * wedge.e3215) + (other.e315 * wedge.e4125) + (other.e4125 * wedge.e315) + (other.e3215 * wedge.e415), (other.e425 * wedge.e3215) + (other.e125 * wedge.e4235) + (other.e4235 * wedge.e125) + (other.e3215 * wedge.e425), (other.e435 * wedge.e3215) + (other.e235 * wedge.e4315) + (other.e4315 * wedge.e235) + (other.e3215 * wedge.e435), -(other.e435 * wedge.e4125) - (other.e4235 * wedge.e415) - (other.e4315 * wedge.e425) - (other.e4125 * wedge.e435)) + (float4(other.e12345) * wedge.group3) + (float4(wedge.e12345) * other.group3) - (wedge.group9.yzxx * float4(other.group8.xyz.zxy, other.e415)) - float4((wedge.group8.xyz.zxy * other.group9.yzx), other.e425 * wedge.e4315), 
            /* e41, e42, e43 */
            (float3(other.e12345) * wedge.group4.xyz) + (float3(other.e1234) * wedge.group6.xyz) + (float3(wedge.e12345) * other.group4.xyz) + (float3(wedge.e1234) * other.group6.xyz) + (other.group7.xyz.zxy * wedge.group9.yzx) + (wedge.group7.xyz.zxy * other.group9.yzx) - (other.group7.xyz.yzx * wedge.group9.zxy) - (wedge.group7.xyz.yzx * other.group9.zxy), 
            /* e23, e31, e12 */
            (float3(other.e12345) * wedge.group5.xyz) + (float3(other.e3215) * wedge.group7.xyz) + (float3(other.e1234) * wedge.group8.xyz) + (float3(wedge.e12345) * other.group5.xyz) + (float3(wedge.e3215) * other.group7.xyz) + (float3(wedge.e1234) * other.group8.xyz) - (float3(other.e321) * wedge.group9.xyz) - (float3(wedge.e321) * other.group9.xyz), 
            /* e415, e425, e435, e321 */
            (float4(other.e12345) * wedge.group6) + (float4(wedge.e12345) * other.group6) + float4((other.group9.zxy * wedge.group9.yzx), other.e1234 * wedge.e3215) - (other.group9.yzxw * float4(wedge.group9.zxy, wedge.e1234)), 
            /* e423, e431, e412 */
            (float3(other.e12345) * wedge.group7.xyz) + (float3(other.e1234) * wedge.group9.xyz) + (float3(wedge.e12345) * other.group7.xyz) - (float3(wedge.e1234) * other.group9.xyz), 
            /* e235, e315, e125 */
            (float3(other.e12345) * wedge.group8.xyz) + (float3(wedge.e12345) * other.group8.xyz) + (float3(wedge.e3215) * other.group9.xyz) - (float3(other.e3215) * wedge.group9.xyz), 
            /* e4235, e4315, e4125, e3215 */
            (float4(other.e12345) * wedge.group9) + (float4(wedge.e12345) * other.group9), 
            /* e1234 */
            (other.e12345 * wedge.e1234) + (other.e1234 * wedge.e12345)
        );
    }
}
public extension MultiVector {
    public MultiVector project_orthogonally_onto(Plane other) {
        AntiPlane right_anti_dual = AntiPlane(
            /* e1, e2, e3, e5 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        MultiVector wedge = MultiVector(
            /* scalar, e12345 */
            float2(0.0, (right_anti_dual.e1 * this.e4235) + (right_anti_dual.e2 * this.e4315) + (right_anti_dual.e3 * this.e4125) + (right_anti_dual.e5 * this.e1234)), 
            /* e1, e2, e3, e4 */
            float4(this.group0.xy.xx, this.scalar, 0.0) * float4(float3(1.0), 0.0) * float4(right_anti_dual.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e5 */
            right_anti_dual.e5 * this.scalar, 
            /* e15, e25, e35, e45 */
            float4(((float3(right_anti_dual.e5) * this.group1.xyz) - (float3(this.e5) * right_anti_dual.group0.xyz)), right_anti_dual.e5 * this.e4), 
            /* e41, e42, e43 */
            float3(this.e4) * right_anti_dual.group0.xyz, 
            /* e23, e31, e12 */
            (right_anti_dual.group0.zxy * this.group1.yzx) - (right_anti_dual.group0.yzx * this.group1.zxy), 
            /* e415, e425, e435, e321 */
            float4(right_anti_dual.e5 * this.e41, right_anti_dual.e5 * this.e42, right_anti_dual.e5 * this.e43, -(right_anti_dual.e2 * this.e31) - (right_anti_dual.e3 * this.e12)) - float4((this.group3.www * right_anti_dual.group0.xyz), right_anti_dual.e1 * this.e23), 
            /* e423, e431, e412 */
            (this.group4.xyz.yzx * right_anti_dual.group0.zxy) - (this.group4.xyz.zxy * right_anti_dual.group0.yzx), 
            /* e235, e315, e125 */
            (float3(right_anti_dual.e5) * this.group5.xyz) + (right_anti_dual.group0.yzx * this.group3.zxy) - (right_anti_dual.group0.zxy * this.group3.yzx), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual.e3 * this.e425 * -1.0, right_anti_dual.e1 * this.e435 * -1.0, right_anti_dual.e2 * this.e415 * -1.0, (right_anti_dual.e3 * this.e125) + (right_anti_dual.e5 * this.e321)) + (right_anti_dual.group0.wwwx * float4(this.group7.xyz, this.e235)) + float4((right_anti_dual.group0.yzx * this.group6.zxy), right_anti_dual.e2 * this.e315), 
            /* e1234 */
            -(right_anti_dual.e1 * this.e423) - (right_anti_dual.e2 * this.e431) - (right_anti_dual.e3 * this.e412)
        );
        return MultiVector(
            /* scalar, e12345 */
            float2((wedge.e1 * other.e4235) + (wedge.e2 * other.e4315) + (wedge.e3 * other.e4125) + (wedge.e4 * other.e3215), 0.0), 
            /* e1, e2, e3, e4 */
            float4(-(wedge.e41 * other.e3215) - (wedge.e31 * other.e4125), -(wedge.e42 * other.e3215) - (wedge.e12 * other.e4235), -(wedge.e43 * other.e3215) - (wedge.e23 * other.e4315), (wedge.e42 * other.e4315) + (wedge.e43 * other.e4125)) + (other.group0.yzxx * float4(wedge.group5.xyz.zxy, wedge.e41)), 
            /* e5 */
            -(wedge.e15 * other.e4235) - (wedge.e25 * other.e4315) - (wedge.e35 * other.e4125) - (wedge.e45 * other.e3215), 
            /* e15, e25, e35, e45 */
            float4((wedge.e415 * other.e3215) + (wedge.e315 * other.e4125), (wedge.e425 * other.e3215) + (wedge.e125 * other.e4235), (wedge.e435 * other.e3215) + (wedge.e235 * other.e4315), -(wedge.e425 * other.e4315) - (wedge.e435 * other.e4125)) - (other.group0.yzxx * float4(wedge.group8.xyz.zxy, wedge.e415)), 
            /* e41, e42, e43 */
            (wedge.group7.xyz.zxy * other.group0.yzx) - (wedge.group7.xyz.yzx * other.group0.zxy), 
            /* e23, e31, e12 */
            (float3(other.e3215) * wedge.group7.xyz) - (float3(wedge.e321) * other.group0.xyz), 
            /* e415, e425, e435, e321 */
            float4(((wedge.group9.yzx * other.group0.zxy) - (wedge.group9.zxy * other.group0.yzx)), wedge.e1234 * other.e3215 * -1.0), 
            /* e423, e431, e412 */
            float3(wedge.e1234) * other.group0.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            (float3(wedge.e3215) * other.group0.xyz) - (float3(other.e3215) * wedge.group9.xyz), 
            /* e4235, e4315, e4125, e3215 */
            float4(wedge.e12345) * other.group0, 
            /* e1234 */
            0.0
        );
    }
}
public extension MultiVector {
    public MultiVector project_orthogonally_onto(RoundPoint other) {
        Sphere right_anti_dual = Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234 */
            other.e4 * -1.0
        );
        MultiVector wedge = MultiVector(
            /* scalar, e12345 */
            float2(0.0, (this.e1 * right_anti_dual.e4235) + (this.e2 * right_anti_dual.e4315) + (this.e3 * right_anti_dual.e4125) + (this.e4 * right_anti_dual.e3215) + (this.e5 * right_anti_dual.e1234)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
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
            float4(this.scalar) * right_anti_dual.group0, 
            /* e1234 */
            this.scalar * right_anti_dual.e1234
        );
        return MultiVector(
            /* scalar, e12345 */
            float2((wedge.e4235 * other.e1) + (wedge.e4315 * other.e2) + (wedge.e4125 * other.e3) + (wedge.e3215 * other.e4) + (wedge.e1234 * other.e5), 0.0), 
            /* e1, e2, e3, e4 */
            float4(wedge.e12345) * other.group0, 
            /* e5 */
            wedge.e12345 * other.e5, 
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
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
public extension MultiVector {
    public Scalar project_orthogonally_onto(Scalar other) {
        return Scalar(
            /* scalar */
            pow(other.scalar, 2) * this.scalar
        );
    }
}
public extension MultiVector {
    public MultiVector project_orthogonally_onto(Sphere other) {
        RoundPoint right_anti_dual = RoundPoint(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5 */
            other.e3215
        );
        MultiVector wedge = MultiVector(
            /* scalar, e12345 */
            float2(0.0, (this.e4235 * right_anti_dual.e1) + (this.e4315 * right_anti_dual.e2) + (this.e4125 * right_anti_dual.e3) + (this.e3215 * right_anti_dual.e4) + (this.e1234 * right_anti_dual.e5)), 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * right_anti_dual.group0, 
            /* e5 */
            this.scalar * right_anti_dual.e5, 
            /* e15, e25, e35, e45 */
            (float4(right_anti_dual.e5) * this.group1) - (float4(this.e5) * right_anti_dual.group0), 
            /* e41, e42, e43 */
            (float3(this.e4) * right_anti_dual.group0.xyz) - (float3(right_anti_dual.e4) * this.group1.xyz), 
            /* e23, e31, e12 */
            (this.group1.yzx * right_anti_dual.group0.zxy) - (this.group1.zxy * right_anti_dual.group0.yzx), 
            /* e415, e425, e435, e321 */
            float4((this.e15 * right_anti_dual.e4) + (this.e41 * right_anti_dual.e5), (this.e25 * right_anti_dual.e4) + (this.e42 * right_anti_dual.e5), (this.e35 * right_anti_dual.e4) + (this.e43 * right_anti_dual.e5), -(this.e31 * right_anti_dual.e2) - (this.e12 * right_anti_dual.e3)) - (right_anti_dual.group0.xyzx * float4(this.group3.www, this.e23)), 
            /* e423, e431, e412 */
            (float3(right_anti_dual.e4) * this.group5.xyz) + (this.group4.xyz.yzx * right_anti_dual.group0.zxy) - (this.group4.xyz.zxy * right_anti_dual.group0.yzx), 
            /* e235, e315, e125 */
            (float3(right_anti_dual.e5) * this.group5.xyz) + (this.group3.zxy * right_anti_dual.group0.yzx) - (this.group3.yzx * right_anti_dual.group0.zxy), 
            /* e4235, e4315, e4125, e3215 */
            float4(-(this.e425 * right_anti_dual.e3) - (this.e235 * right_anti_dual.e4), -(this.e435 * right_anti_dual.e1) - (this.e315 * right_anti_dual.e4), -(this.e415 * right_anti_dual.e2) - (this.e125 * right_anti_dual.e4), (this.e321 * right_anti_dual.e5) + (this.e125 * right_anti_dual.e3)) + (right_anti_dual.group0.yzxy * float4(this.group6.zxy, this.e315)) + float4((float3(right_anti_dual.e5) * this.group7.xyz), this.e235 * right_anti_dual.e1), 
            /* e1234 */
            -(this.e321 * right_anti_dual.e4) - (this.e423 * right_anti_dual.e1) - (this.e431 * right_anti_dual.e2) - (this.e412 * right_anti_dual.e3)
        );
        return MultiVector(
            /* scalar, e12345 */
            float2((wedge.e1 * other.e4235) + (wedge.e2 * other.e4315) + (wedge.e3 * other.e4125) + (wedge.e4 * other.e3215) + (wedge.e5 * other.e1234), 0.0), 
            /* e1, e2, e3, e4 */
            float4(-(wedge.e41 * other.e3215) - (wedge.e31 * other.e4125), -(wedge.e42 * other.e3215) - (wedge.e12 * other.e4235), -(wedge.e43 * other.e3215) - (wedge.e23 * other.e4315), (wedge.e45 * other.e1234) + (wedge.e43 * other.e4125)) + (other.group0.yzxx * float4(wedge.group5.xyz.zxy, wedge.e41)) + float4((float3(other.e1234) * wedge.group3.xyz), wedge.e42 * other.e4315), 
            /* e5 */
            -(wedge.e15 * other.e4235) - (wedge.e25 * other.e4315) - (wedge.e35 * other.e4125) - (wedge.e45 * other.e3215), 
            /* e15, e25, e35, e45 */
            float4((wedge.e415 * other.e3215) + (wedge.e315 * other.e4125), (wedge.e425 * other.e3215) + (wedge.e125 * other.e4235), (wedge.e435 * other.e3215) + (wedge.e235 * other.e4315), -(wedge.e425 * other.e4315) - (wedge.e435 * other.e4125)) - (other.group0.yzxx * float4(wedge.group8.xyz.zxy, wedge.e415)), 
            /* e41, e42, e43 */
            (float3(other.e1234) * wedge.group6.xyz) + (wedge.group7.xyz.zxy * other.group0.yzx) - (wedge.group7.xyz.yzx * other.group0.zxy), 
            /* e23, e31, e12 */
            (float3(other.e3215) * wedge.group7.xyz) + (float3(other.e1234) * wedge.group8.xyz) - (float3(wedge.e321) * other.group0.xyz), 
            /* e415, e425, e435, e321 */
            (wedge.group9.yzxw * float4(other.group0.zxy, other.e1234)) - float4((wedge.group9.zxy * other.group0.yzx), wedge.e1234 * other.e3215), 
            /* e423, e431, e412 */
            (float3(other.e1234) * wedge.group9.xyz) - (float3(wedge.e1234) * other.group0.xyz), 
            /* e235, e315, e125 */
            (float3(wedge.e3215) * other.group0.xyz) - (float3(other.e3215) * wedge.group9.xyz), 
            /* e4235, e4315, e4125, e3215 */
            float4(wedge.e12345) * other.group0, 
            /* e1234 */
            wedge.e12345 * other.e1234
        );
    }
}
public extension MultiVector {
    public MultiVector project_orthogonally_onto(VersorEven other) {
        VersorOdd right_anti_dual = VersorOdd(
            /* e41, e42, e43, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e1234 */
            float4(other.group2.xyz, other.e4) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.group3.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0)
        );
        MultiVector wedge = MultiVector(
            /* scalar, e12345 */
            float2(this.scalar * right_anti_dual.scalar, (this.e12345 * right_anti_dual.scalar) + (this.e1 * right_anti_dual.e4235) + (this.e2 * right_anti_dual.e4315) + (this.e3 * right_anti_dual.e4125) + (this.e4 * right_anti_dual.e3215) + (this.e5 * right_anti_dual.e1234) - (this.e415 * right_anti_dual.e23) - (this.e425 * right_anti_dual.e31) - (this.e435 * right_anti_dual.e12) - (this.e321 * right_anti_dual.e45) - (this.e423 * right_anti_dual.e15) - (this.e431 * right_anti_dual.e25) - (this.e412 * right_anti_dual.e35) - (this.e235 * right_anti_dual.e41) - (this.e315 * right_anti_dual.e42) - (this.e125 * right_anti_dual.e43)), 
            /* e1, e2, e3, e4 */
            float4(right_anti_dual.scalar) * this.group1, 
            /* e5 */
            this.e5 * right_anti_dual.scalar, 
            /* e15, e25, e35, e45 */
            (float4(this.scalar) * float4(right_anti_dual.group2.xyz, right_anti_dual.e45)) + (float4(right_anti_dual.scalar) * this.group3), 
            /* e41, e42, e43 */
            (float3(this.scalar) * right_anti_dual.group0.xyz) + (float3(right_anti_dual.scalar) * this.group4.xyz), 
            /* e23, e31, e12 */
            (float3(this.scalar) * right_anti_dual.group1.xyz) + (float3(right_anti_dual.scalar) * this.group5.xyz), 
            /* e415, e425, e435, e321 */
            float4((this.e5 * right_anti_dual.e41) + (this.e415 * right_anti_dual.scalar), (this.e5 * right_anti_dual.e42) + (this.e425 * right_anti_dual.scalar), (this.e5 * right_anti_dual.e43) + (this.e435 * right_anti_dual.scalar), -(this.e2 * right_anti_dual.e31) - (this.e3 * right_anti_dual.e12)) + float4((this.group1.www * right_anti_dual.group2.xyz), this.e321 * right_anti_dual.scalar) - (this.group1.xyzx * right_anti_dual.group1.wwwx), 
            /* e423, e431, e412 */
            (float3(this.e4) * right_anti_dual.group1.xyz) + (float3(right_anti_dual.scalar) * this.group7.xyz) + (this.group1.zxy * right_anti_dual.group0.yzx) - (this.group1.yzx * right_anti_dual.group0.zxy), 
            /* e235, e315, e125 */
            (float3(this.e5) * right_anti_dual.group1.xyz) + (float3(right_anti_dual.scalar) * this.group8.xyz) + (this.group1.yzx * right_anti_dual.group2.zxy) - (this.group1.zxy * right_anti_dual.group2.yzx), 
            /* e4235, e4315, e4125, e3215 */
            float4((this.e35 * right_anti_dual.e42) + (this.e45 * right_anti_dual.e23) + (this.e23 * right_anti_dual.e45) + (this.e4235 * right_anti_dual.scalar), (this.e15 * right_anti_dual.e43) + (this.e45 * right_anti_dual.e31) + (this.e31 * right_anti_dual.e45) + (this.e4315 * right_anti_dual.scalar), (this.e25 * right_anti_dual.e41) + (this.e45 * right_anti_dual.e12) + (this.e12 * right_anti_dual.e45) + (this.e4125 * right_anti_dual.scalar), -(this.e15 * right_anti_dual.e23) - (this.e25 * right_anti_dual.e31) - (this.e35 * right_anti_dual.e12) - (this.e12 * right_anti_dual.e35)) + (float4(this.scalar) * right_anti_dual.group3) + float4((this.group4.xyz.yzx * right_anti_dual.group2.zxy), this.e3215 * right_anti_dual.scalar) - (right_anti_dual.group2.yzxx * float4(this.group4.xyz.zxy, this.e23)) - float4((this.group3.yzx * right_anti_dual.group0.zxy), this.e31 * right_anti_dual.e25), 
            /* e1234 */
            (this.scalar * right_anti_dual.e1234) + (this.e1234 * right_anti_dual.scalar) - (this.e41 * right_anti_dual.e23) - (this.e42 * right_anti_dual.e31) - (this.e43 * right_anti_dual.e12) - (this.e23 * right_anti_dual.e41) - (this.e31 * right_anti_dual.e42) - (this.e12 * right_anti_dual.e43)
        );
        return MultiVector(
            /* scalar, e12345 */
            float2((wedge.scalar * other.e12345) + (wedge.e4235 * other.e1) + (wedge.e4315 * other.e2) + (wedge.e4125 * other.e3) + (wedge.e3215 * other.e4) + (wedge.e1234 * other.e5) - (wedge.e15 * other.e423) - (wedge.e25 * other.e431) - (wedge.e35 * other.e412) - (wedge.e45 * other.e321) - (wedge.e41 * other.e235) - (wedge.e42 * other.e315) - (wedge.e43 * other.e125) - (wedge.e23 * other.e415) - (wedge.e31 * other.e425) - (wedge.e12 * other.e435), wedge.e12345 * other.e12345), 
            /* e1, e2, e3, e4 */
            float4((wedge.e1 * other.e12345) + (wedge.e415 * other.e321) + (wedge.e321 * other.e415) + (wedge.e315 * other.e412), (wedge.e2 * other.e12345) + (wedge.e425 * other.e321) + (wedge.e321 * other.e425) + (wedge.e125 * other.e423), (wedge.e3 * other.e12345) + (wedge.e435 * other.e321) + (wedge.e321 * other.e435) + (wedge.e235 * other.e431), -(wedge.e415 * other.e423) - (wedge.e425 * other.e431) - (wedge.e435 * other.e412) - (wedge.e412 * other.e435)) + (float4(wedge.e12345) * other.group3) + float4((wedge.group7.xyz.zxy * other.group2.yzx), wedge.e4 * other.e12345) - float4((wedge.group7.xyz.yzx * other.group2.zxy), wedge.e423 * other.e415) - float4((wedge.group8.xyz.zxy * other.group0.yzx), wedge.e431 * other.e425), 
            /* e5 */
            (wedge.e12345 * other.e5) + (wedge.e5 * other.e12345) - (wedge.e415 * other.e235) - (wedge.e425 * other.e315) - (wedge.e435 * other.e125) - (wedge.e235 * other.e415) - (wedge.e315 * other.e425) - (wedge.e125 * other.e435), 
            /* e15, e25, e35, e45 */
            float4((wedge.e4125 * other.e315) + (wedge.e3215 * other.e415), (wedge.e4235 * other.e125) + (wedge.e3215 * other.e425), (wedge.e4315 * other.e235) + (wedge.e3215 * other.e435), -(wedge.e4315 * other.e425) - (wedge.e4125 * other.e435)) + (float4(other.e12345) * wedge.group3) - (wedge.group9.yzxx * float4(other.group2.zxy, other.e415)), 
            /* e41, e42, e43 */
            (float3(wedge.e1234) * other.group1.xyz) + (float3(other.e12345) * wedge.group4.xyz) + (wedge.group9.yzx * other.group0.zxy) - (wedge.group9.zxy * other.group0.yzx), 
            /* e23, e31, e12 */
            (float3(wedge.e3215) * other.group0.xyz) + (float3(wedge.e1234) * other.group2.xyz) + (float3(other.e12345) * wedge.group5.xyz) - (float3(other.e321) * wedge.group9.xyz), 
            /* e415, e425, e435, e321 */
            (float4(wedge.e12345) * other.group1) + (float4(other.e12345) * wedge.group6), 
            /* e423, e431, e412 */
            (float3(wedge.e12345) * other.group0.xyz) + (float3(other.e12345) * wedge.group7.xyz), 
            /* e235, e315, e125 */
            (float3(wedge.e12345) * other.group2.xyz) + (float3(other.e12345) * wedge.group8.xyz), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345) * wedge.group9, 
            /* e1234 */
            wedge.e1234 * other.e12345
        );
    }
}
public extension MultiVector {
    public MultiVector project_orthogonally_onto(VersorOdd other) {
        VersorEven right_anti_dual = VersorEven(
            /* e423, e431, e412, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            float4(other.group2.xyz, other.e3215) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4(other.group3.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0)
        );
        MultiVector wedge = MultiVector(
            /* scalar, e12345 */
            float2(0.0, (this.scalar * right_anti_dual.e12345) + (this.e4235 * right_anti_dual.e1) + (this.e4315 * right_anti_dual.e2) + (this.e4125 * right_anti_dual.e3) + (this.e3215 * right_anti_dual.e4) + (this.e1234 * right_anti_dual.e5) - (this.e15 * right_anti_dual.e423) - (this.e25 * right_anti_dual.e431) - (this.e35 * right_anti_dual.e412) - (this.e45 * right_anti_dual.e321) - (this.e41 * right_anti_dual.e235) - (this.e42 * right_anti_dual.e315) - (this.e43 * right_anti_dual.e125) - (this.e23 * right_anti_dual.e415) - (this.e31 * right_anti_dual.e425) - (this.e12 * right_anti_dual.e435)), 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * right_anti_dual.group3, 
            /* e5 */
            this.scalar * right_anti_dual.e5, 
            /* e15, e25, e35, e45 */
            (float4(right_anti_dual.e5) * this.group1) - (float4(this.e5) * right_anti_dual.group3), 
            /* e41, e42, e43 */
            (float3(this.e4) * right_anti_dual.group3.xyz) - (float3(right_anti_dual.e4) * this.group1.xyz), 
            /* e23, e31, e12 */
            (this.group1.yzx * right_anti_dual.group3.zxy) - (this.group1.zxy * right_anti_dual.group3.yzx), 
            /* e415, e425, e435, e321 */
            float4((this.e15 * right_anti_dual.e4) + (this.e41 * right_anti_dual.e5), (this.e25 * right_anti_dual.e4) + (this.e42 * right_anti_dual.e5), (this.e35 * right_anti_dual.e4) + (this.e43 * right_anti_dual.e5), -(this.e31 * right_anti_dual.e2) - (this.e12 * right_anti_dual.e3)) + (float4(this.scalar) * right_anti_dual.group1) - (right_anti_dual.group3.xyzx * float4(this.group3.www, this.e23)), 
            /* e423, e431, e412 */
            (float3(this.scalar) * right_anti_dual.group0.xyz) + (float3(right_anti_dual.e4) * this.group5.xyz) + (this.group4.xyz.yzx * right_anti_dual.group3.zxy) - (this.group4.xyz.zxy * right_anti_dual.group3.yzx), 
            /* e235, e315, e125 */
            (float3(this.scalar) * right_anti_dual.group2.xyz) + (float3(right_anti_dual.e5) * this.group5.xyz) + (this.group3.zxy * right_anti_dual.group3.yzx) - (this.group3.yzx * right_anti_dual.group3.zxy), 
            /* e4235, e4315, e4125, e3215 */
            (this.group6.zxyw * float4(right_anti_dual.group3.yzx, right_anti_dual.e5)) + float4((this.group7.xyz * right_anti_dual.group2.www), this.e235 * right_anti_dual.e1) + float4((this.group1.zxy * right_anti_dual.group1.yzx), this.e315 * right_anti_dual.e2) + float4((this.group1.www * right_anti_dual.group2.xyz), this.e125 * right_anti_dual.e3) - (float4(this.e5) * float4(right_anti_dual.group0.xyz, right_anti_dual.e321)) - (this.group1.yzxy * float4(right_anti_dual.group1.zxy, right_anti_dual.e315)) - float4((this.group8.xyz * right_anti_dual.group3.www), this.e1 * right_anti_dual.e235) - float4((this.group6.yzx * right_anti_dual.group3.zxy), this.e3 * right_anti_dual.e125), 
            /* e1234 */
            (this.e1 * right_anti_dual.e423) + (this.e2 * right_anti_dual.e431) + (this.e3 * right_anti_dual.e412) + (this.e4 * right_anti_dual.e321) - (this.e321 * right_anti_dual.e4) - (this.e423 * right_anti_dual.e1) - (this.e431 * right_anti_dual.e2) - (this.e412 * right_anti_dual.e3)
        );
        return MultiVector(
            /* scalar, e12345 */
            float2((wedge.e12345 * other.scalar) + (wedge.e1 * other.e4235) + (wedge.e2 * other.e4315) + (wedge.e3 * other.e4125) + (wedge.e4 * other.e3215) + (wedge.e5 * other.e1234) - (wedge.e415 * other.e23) - (wedge.e425 * other.e31) - (wedge.e435 * other.e12) - (wedge.e321 * other.e45) - (wedge.e423 * other.e15) - (wedge.e431 * other.e25) - (wedge.e412 * other.e35) - (wedge.e235 * other.e41) - (wedge.e315 * other.e42) - (wedge.e125 * other.e43), 0.0), 
            /* e1, e2, e3, e4 */
            (other.group3.yzxx * float4(wedge.group5.xyz.zxy, wedge.e41)) + float4((wedge.group9.zxy * other.group1.yzx), wedge.e43 * other.e4125) + float4((wedge.group9.www * other.group0.xyz), wedge.e45 * other.e1234) + float4((other.group2.www * wedge.group3.xyz), wedge.e42 * other.e4315) - (float4(wedge.e1234) * float4(other.group2.xyz, other.e45)) - (wedge.group9.yzxz * float4(other.group1.zxy, other.e43)) - float4((wedge.group4.xyz * other.group3.www), wedge.e4235 * other.e41) - float4((wedge.group5.xyz.yzx * other.group3.zxy), wedge.e4315 * other.e42), 
            /* e5 */
            (wedge.e4235 * other.e15) + (wedge.e4315 * other.e25) + (wedge.e4125 * other.e35) + (wedge.e3215 * other.e45) - (wedge.e15 * other.e4235) - (wedge.e25 * other.e4315) - (wedge.e35 * other.e4125) - (wedge.e45 * other.e3215), 
            /* e15, e25, e35, e45 */
            float4((wedge.e415 * other.e3215) + (wedge.e315 * other.e4125), (wedge.e425 * other.e3215) + (wedge.e125 * other.e4235), (wedge.e435 * other.e3215) + (wedge.e235 * other.e4315), -(wedge.e425 * other.e4315) - (wedge.e435 * other.e4125)) + (float4(wedge.e12345) * float4(other.group2.xyz, other.e45)) - (other.group3.yzxx * float4(wedge.group8.xyz.zxy, wedge.e415)), 
            /* e41, e42, e43 */
            (float3(wedge.e12345) * other.group0.xyz) + (float3(other.e1234) * wedge.group6.xyz) + (wedge.group7.xyz.zxy * other.group3.yzx) - (wedge.group7.xyz.yzx * other.group3.zxy), 
            /* e23, e31, e12 */
            (float3(wedge.e12345) * other.group1.xyz) + (float3(other.e1234) * wedge.group8.xyz) + (float3(other.e3215) * wedge.group7.xyz) - (float3(wedge.e321) * other.group3.xyz), 
            /* e415, e425, e435, e321 */
            (wedge.group9.yzxw * float4(other.group3.zxy, other.e1234)) - float4((wedge.group9.zxy * other.group3.yzx), wedge.e1234 * other.e3215), 
            /* e423, e431, e412 */
            (float3(other.e1234) * wedge.group9.xyz) - (float3(wedge.e1234) * other.group3.xyz), 
            /* e235, e315, e125 */
            (float3(wedge.e3215) * other.group3.xyz) - (float3(other.e3215) * wedge.group9.xyz), 
            /* e4235, e4315, e4125, e3215 */
            float4(wedge.e12345) * other.group3, 
            /* e1234 */
            wedge.e12345 * other.e1234
        );
    }
}
public extension Plane {
    // Fancy infix trick (first half)
    public ProjectOrthogonallyOntoInfixPartial<Plane> operator /(ProjectOrthogonallyOntoInfix rhs) {
        return ProjectOrthogonallyOntoInfixPartial<Plane>(this);
    }
}
public extension ProjectOrthogonallyOntoInfixPartial<Plane> {
    // Fancy infix trick (second half)
    public Plane operator /(AntiScalar rhs) {
        return this.a.project_orthogonally_onto(rhs);
    }
}
public extension Plane {
    public Plane project_orthogonally_onto(AntiScalar other) {
        Scalar right_anti_dual = Scalar(
            /* scalar */
            other.e12345 * -1.0
        );
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345) * float4(this.e4235 * right_anti_dual.scalar, this.e4315 * right_anti_dual.scalar, this.e4125 * right_anti_dual.scalar, this.e3215 * right_anti_dual.scalar)
        );
    }
}
public extension Plane {
    public DipoleInversion project_orthogonally_onto(CircleRotor other) {
        Plane wedge = Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345 * -1.0) * this.group0
        );
        return DipoleInversion(
            /* e41, e42, e43 */
            (other.group0.xyz.zxy * wedge.group0.yzx) - (other.group0.xyz.yzx * wedge.group0.zxy), 
            /* e23, e31, e12, e45 */
            float4(other.e423 * wedge.e3215, other.e431 * wedge.e3215, other.e412 * wedge.e3215, -(other.e425 * wedge.e4315) - (other.e435 * wedge.e4125)) - (other.group1.wwwx * wedge.group0.xyzx), 
            /* e15, e25, e35, e1234 */
            float4((float3(wedge.e3215) * other.group1.xyz), 0.0) + float4((other.group2.yzx * wedge.group0.zxy), 0.0) - float4((other.group2.zxy * wedge.group0.yzx), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345) * wedge.group0
        );
    }
}
public extension Plane {
    public DipoleInversion project_orthogonally_onto(DipoleInversion other) {
        AntiDipoleInversion right_anti_dual = AntiDipoleInversion(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e4 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group3 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        AntiScalar wedge = AntiScalar(
            /* e12345 */
            (right_anti_dual.e4 * this.e3215) + (right_anti_dual.e1 * this.e4235) + (right_anti_dual.e2 * this.e4315) + (right_anti_dual.e3 * this.e4125)
        );
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(wedge.e12345) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(wedge.e12345) * other.group1, 
            /* e15, e25, e35, e1234 */
            float4(wedge.e12345) * other.group2, 
            /* e4235, e4315, e4125, e3215 */
            float4(wedge.e12345) * other.group3
        );
    }
}
public extension Plane {
    public Plane project_orthogonally_onto(DualNum other) {
        AntiDualNum right_anti_dual = AntiDualNum(
            /* e3215, scalar */
            other.group0.xy * float2(-1.0)
        );
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345) * float4(right_anti_dual.scalar * this.e4235, right_anti_dual.scalar * this.e4315, right_anti_dual.scalar * this.e4125, right_anti_dual.scalar * this.e3215)
        );
    }
}
public extension Plane {
    public Flector project_orthogonally_onto(Flector other) {
        AntiFlector right_anti_dual = AntiFlector(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        AntiScalar wedge = AntiScalar(
            /* e12345 */
            (right_anti_dual.e1 * this.e4235) + (right_anti_dual.e2 * this.e4315) + (right_anti_dual.e3 * this.e4125)
        );
        return Flector(
            /* e15, e25, e35, e45 */
            float4(wedge.e12345) * other.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4(wedge.e12345) * other.group1
        );
    }
}
public extension Plane {
    public Flector project_orthogonally_onto(Motor other) {
        Plane wedge = Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345 * -1.0) * this.group0
        );
        return Flector(
            /* e15, e25, e35, e45 */
            float4((other.e415 * wedge.e3215) + (other.e315 * wedge.e4125), (other.e425 * wedge.e3215) + (other.e125 * wedge.e4235), (other.e435 * wedge.e3215) + (other.e235 * wedge.e4315), -(other.e425 * wedge.e4315) - (other.e435 * wedge.e4125)) - (wedge.group0.yzxx * float4(other.group1.zxy, other.e415)), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345) * wedge.group0
        );
    }
}
public extension Plane {
    public MultiVector project_orthogonally_onto(MultiVector other) {
        MultiVector right_anti_dual = MultiVector(
            /* scalar, e12345 */
            other.group0.xy.yx * float2(-1.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4(other.group9.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5 */
            other.e3215, 
            /* e15, e25, e35, e45 */
            float4(other.group8.xyz, other.e321 * -1.0), 
            /* e41, e42, e43 */
            other.group7.xyz, 
            /* e23, e31, e12 */
            other.group6.xyz, 
            /* e415, e425, e435, e321 */
            float4(other.group5.xyz, other.e45) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e423, e431, e412 */
            other.group4.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            other.group3.xyz * float3(-1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.group1.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234 */
            other.e4 * -1.0
        );
        MultiVector wedge = MultiVector(
            /* scalar, e12345 */
            float2(0.0, (right_anti_dual.e1 * this.e4235) + (right_anti_dual.e2 * this.e4315) + (right_anti_dual.e3 * this.e4125) + (right_anti_dual.e4 * this.e3215)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
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
            float4(right_anti_dual.scalar) * this.group0, 
            /* e1234 */
            0.0
        );
        return MultiVector(
            /* scalar, e12345 */
            float2((other.scalar * wedge.e12345) + (other.e12345 * wedge.scalar) + (other.e1 * wedge.e4235) + (other.e2 * wedge.e4315) + (other.e3 * wedge.e4125) + (other.e4 * wedge.e3215) + (other.e5 * wedge.e1234) + (other.e4235 * wedge.e1) + (other.e4315 * wedge.e2) + (other.e4125 * wedge.e3) + (other.e3215 * wedge.e4) + (other.e1234 * wedge.e5) - (other.e15 * wedge.e423) - (other.e25 * wedge.e431) - (other.e35 * wedge.e412) - (other.e45 * wedge.e321) - (other.e41 * wedge.e235) - (other.e42 * wedge.e315) - (other.e43 * wedge.e125) - (other.e23 * wedge.e415) - (other.e31 * wedge.e425) - (other.e12 * wedge.e435) - (other.e415 * wedge.e23) - (other.e425 * wedge.e31) - (other.e435 * wedge.e12) - (other.e321 * wedge.e45) - (other.e423 * wedge.e15) - (other.e431 * wedge.e25) - (other.e412 * wedge.e35) - (other.e235 * wedge.e41) - (other.e315 * wedge.e42) - (other.e125 * wedge.e43), other.e12345 * wedge.e12345), 
            /* e1, e2, e3, e4 */
            float4((other.e415 * wedge.e321) + (other.e321 * wedge.e415) + (other.e4315 * wedge.e12) + (other.e1234 * wedge.e15), (other.e425 * wedge.e321) + (other.e321 * wedge.e425) + (other.e4125 * wedge.e23) + (other.e1234 * wedge.e25), (other.e435 * wedge.e321) + (other.e321 * wedge.e435) + (other.e4235 * wedge.e31) + (other.e1234 * wedge.e35), -(other.e45 * wedge.e1234) - (other.e415 * wedge.e423) - (other.e425 * wedge.e431) - (other.e435 * wedge.e412)) + (float4(other.e12345) * wedge.group1) + (float4(wedge.e12345) * other.group1) + float4((other.group4.xyz * wedge.group9.www), other.e4235 * wedge.e41) + float4((other.group5.xyz.yzx * wedge.group9.zxy), other.e4315 * wedge.e42) + float4((other.group7.xyz.zxy * wedge.group8.xyz.yzx), other.e4125 * wedge.e43) + float4((other.group8.xyz.yzx * wedge.group7.xyz.zxy), other.e1234 * wedge.e45) - (wedge.group9.yzxx * float4(other.group5.xyz.zxy, other.e41)) - float4((float3(wedge.e1234) * other.group3.xyz), other.e412 * wedge.e435) - float4((wedge.group4.xyz * other.group9.www), other.e423 * wedge.e415) - float4((other.group7.xyz.yzx * wedge.group8.xyz.zxy), other.e42 * wedge.e4315) - float4((other.group8.xyz.zxy * wedge.group7.xyz.yzx), other.e43 * wedge.e4125) - float4((wedge.group5.xyz.yzx * other.group9.zxy), other.e431 * wedge.e425), 
            /* e5 */
            (other.e12345 * wedge.e5) + (other.e5 * wedge.e12345) + (other.e15 * wedge.e4235) + (other.e25 * wedge.e4315) + (other.e35 * wedge.e4125) + (other.e45 * wedge.e3215) - (other.e415 * wedge.e235) - (other.e425 * wedge.e315) - (other.e435 * wedge.e125) - (other.e235 * wedge.e415) - (other.e315 * wedge.e425) - (other.e125 * wedge.e435) - (other.e4235 * wedge.e15) - (other.e4315 * wedge.e25) - (other.e4125 * wedge.e35) - (other.e3215 * wedge.e45), 
            /* e15, e25, e35, e45 */
            float4((other.e415 * wedge.e3215) + (other.e315 * wedge.e4125) + (other.e4125 * wedge.e315) + (other.e3215 * wedge.e415), (other.e425 * wedge.e3215) + (other.e125 * wedge.e4235) + (other.e4235 * wedge.e125) + (other.e3215 * wedge.e425), (other.e435 * wedge.e3215) + (other.e235 * wedge.e4315) + (other.e4315 * wedge.e235) + (other.e3215 * wedge.e435), -(other.e435 * wedge.e4125) - (other.e4235 * wedge.e415) - (other.e4315 * wedge.e425) - (other.e4125 * wedge.e435)) + (float4(other.e12345) * wedge.group3) + (float4(wedge.e12345) * other.group3) - (wedge.group9.yzxx * float4(other.group8.xyz.zxy, other.e415)) - float4((wedge.group8.xyz.zxy * other.group9.yzx), other.e425 * wedge.e4315), 
            /* e41, e42, e43 */
            (float3(other.e12345) * wedge.group4.xyz) + (float3(other.e1234) * wedge.group6.xyz) + (float3(wedge.e12345) * other.group4.xyz) + (float3(wedge.e1234) * other.group6.xyz) + (other.group7.xyz.zxy * wedge.group9.yzx) + (wedge.group7.xyz.zxy * other.group9.yzx) - (other.group7.xyz.yzx * wedge.group9.zxy) - (wedge.group7.xyz.yzx * other.group9.zxy), 
            /* e23, e31, e12 */
            (float3(other.e12345) * wedge.group5.xyz) + (float3(other.e3215) * wedge.group7.xyz) + (float3(other.e1234) * wedge.group8.xyz) + (float3(wedge.e12345) * other.group5.xyz) + (float3(wedge.e3215) * other.group7.xyz) + (float3(wedge.e1234) * other.group8.xyz) - (float3(other.e321) * wedge.group9.xyz) - (float3(wedge.e321) * other.group9.xyz), 
            /* e415, e425, e435, e321 */
            (float4(other.e12345) * wedge.group6) + (float4(wedge.e12345) * other.group6) + float4((other.group9.zxy * wedge.group9.yzx), other.e1234 * wedge.e3215) - (other.group9.yzxw * float4(wedge.group9.zxy, wedge.e1234)), 
            /* e423, e431, e412 */
            (float3(other.e12345) * wedge.group7.xyz) + (float3(other.e1234) * wedge.group9.xyz) + (float3(wedge.e12345) * other.group7.xyz) - (float3(wedge.e1234) * other.group9.xyz), 
            /* e235, e315, e125 */
            (float3(other.e12345) * wedge.group8.xyz) + (float3(wedge.e12345) * other.group8.xyz) + (float3(wedge.e3215) * other.group9.xyz) - (float3(other.e3215) * wedge.group9.xyz), 
            /* e4235, e4315, e4125, e3215 */
            (float4(other.e12345) * wedge.group9) + (float4(wedge.e12345) * other.group9), 
            /* e1234 */
            (other.e12345 * wedge.e1234) + (other.e1234 * wedge.e12345)
        );
    }
}
public extension Plane {
    public Plane project_orthogonally_onto(Plane other) {
        AntiPlane right_anti_dual = AntiPlane(
            /* e1, e2, e3, e5 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual.e1 * this.e4235) + (right_anti_dual.e2 * this.e4315) + (right_anti_dual.e3 * this.e4125)) * other.group0
        );
    }
}
public extension Plane {
    public Sphere project_orthogonally_onto(Sphere other) {
        RoundPoint right_anti_dual = RoundPoint(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5 */
            other.e3215
        );
        AntiScalar wedge = AntiScalar(
            /* e12345 */
            (this.e4235 * right_anti_dual.e1) + (this.e4315 * right_anti_dual.e2) + (this.e4125 * right_anti_dual.e3) + (this.e3215 * right_anti_dual.e4)
        );
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(wedge.e12345) * other.group0, 
            /* e1234 */
            wedge.e12345 * other.e1234
        );
    }
}
public extension Plane {
    public VersorOdd project_orthogonally_onto(VersorEven other) {
        Plane wedge = Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345 * -1.0) * this.group0
        );
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(wedge.e4125 * other.e431 * -1.0, wedge.e4235 * other.e412 * -1.0, wedge.e4315 * other.e423 * -1.0, (wedge.e4315 * other.e2) + (wedge.e4125 * other.e3) + (wedge.e3215 * other.e4)) + (wedge.group0.yzxx * float4(other.group0.zxy, other.e1)), 
            /* e23, e31, e12, e45 */
            float4(wedge.e3215 * other.e423, wedge.e3215 * other.e431, wedge.e3215 * other.e412, -(wedge.e4315 * other.e425) - (wedge.e4125 * other.e435)) - (wedge.group0.xyzx * other.group1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4((float3(wedge.e3215) * other.group1.xyz), 0.0) + float4((wedge.group0.zxy * other.group2.yzx), 0.0) - float4((wedge.group0.yzx * other.group2.zxy), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345) * wedge.group0
        );
    }
}
public extension Plane {
    public VersorOdd project_orthogonally_onto(VersorOdd other) {
        VersorEven right_anti_dual = VersorEven(
            /* e423, e431, e412, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            float4(other.group2.xyz, other.e3215) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4(other.group3.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0)
        );
        AntiScalar wedge = AntiScalar(
            /* e12345 */
            (this.e4235 * right_anti_dual.e1) + (this.e4315 * right_anti_dual.e2) + (this.e4125 * right_anti_dual.e3) + (this.e3215 * right_anti_dual.e4)
        );
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(wedge.e12345) * other.group0, 
            /* e23, e31, e12, e45 */
            float4(wedge.e12345) * other.group1, 
            /* e15, e25, e35, e1234 */
            float4(wedge.e12345) * other.group2, 
            /* e4235, e4315, e4125, e3215 */
            float4(wedge.e12345) * other.group3
        );
    }
}
public extension RoundPoint {
    // Fancy infix trick (first half)
    public ProjectOrthogonallyOntoInfixPartial<RoundPoint> operator /(ProjectOrthogonallyOntoInfix rhs) {
        return ProjectOrthogonallyOntoInfixPartial<RoundPoint>(this);
    }
}
public extension ProjectOrthogonallyOntoInfixPartial<RoundPoint> {
    // Fancy infix trick (second half)
    public RoundPoint operator /(AntiCircleRotor rhs) {
        return this.a.project_orthogonally_onto(rhs);
    }
}
public extension RoundPoint {
    public RoundPoint project_orthogonally_onto(AntiCircleRotor other) {
        CircleRotor right_anti_dual = CircleRotor(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        Sphere wedge = Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual.e425 * this.e3) + (right_anti_dual.e235 * this.e4), (right_anti_dual.e435 * this.e1) + (right_anti_dual.e315 * this.e4), (right_anti_dual.e415 * this.e2) + (right_anti_dual.e125 * this.e4), -(right_anti_dual.e315 * this.e2) - (right_anti_dual.e125 * this.e3)) - (float4(this.e5) * float4(right_anti_dual.group0.xyz, right_anti_dual.e321)) - (this.group0.yzxx * float4(right_anti_dual.group1.zxy, right_anti_dual.e235)), 
            /* e1234 */
            (right_anti_dual.e423 * this.e1) + (right_anti_dual.e431 * this.e2) + (right_anti_dual.e412 * this.e3) + (right_anti_dual.e321 * this.e4)
        );
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4((other.e41 * wedge.e3215) + (other.e31 * wedge.e4125), (other.e42 * wedge.e3215) + (other.e12 * wedge.e4235), (other.e43 * wedge.e3215) + (other.e23 * wedge.e4315), -(other.e43 * wedge.e4125) - (other.e45 * wedge.e1234)) - (wedge.group0.yzxx * float4(other.group1.zxy, other.e41)) - float4((float3(wedge.e1234) * other.group2.xyz), other.e42 * wedge.e4315), 
            /* e5 */
            (other.e45 * wedge.e3215) + (other.e15 * wedge.e4235) + (other.e25 * wedge.e4315) + (other.e35 * wedge.e4125)
        );
    }
}
public extension RoundPoint {
    public AntiDipoleInversion project_orthogonally_onto(AntiDipoleInversion other) {
        DipoleInversion right_anti_dual = DipoleInversion(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e1234 */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group3 * float4(1.0, 1.0, 1.0, -1.0)
        );
        CircleRotor wedge = CircleRotor(
            /* e423, e431, e412 */
            (float3(this.e4) * right_anti_dual.group1.xyz) + (right_anti_dual.group0.xyz.yzx * this.group0.zxy) - (right_anti_dual.group0.xyz.zxy * this.group0.yzx), 
            /* e415, e425, e435, e321 */
            float4((right_anti_dual.e41 * this.e5) + (right_anti_dual.e15 * this.e4), (right_anti_dual.e42 * this.e5) + (right_anti_dual.e25 * this.e4), (right_anti_dual.e43 * this.e5) + (right_anti_dual.e35 * this.e4), -(right_anti_dual.e31 * this.e2) - (right_anti_dual.e12 * this.e3)) - (right_anti_dual.group1.wwwx * this.group0.xyzx), 
            /* e235, e315, e125, e12345 */
            float4(right_anti_dual.e25 * this.e3 * -1.0, right_anti_dual.e35 * this.e1 * -1.0, right_anti_dual.e15 * this.e2 * -1.0, (right_anti_dual.e4315 * this.e2) + (right_anti_dual.e4125 * this.e3) + (right_anti_dual.e3215 * this.e4)) + (float4(this.e5) * float4(right_anti_dual.group1.xyz, right_anti_dual.e1234)) + (this.group0.yzxx * float4(right_anti_dual.group2.zxy, right_anti_dual.e4235))
        );
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(wedge.e12345) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(wedge.e12345) * other.group1, 
            /* e235, e315, e125, e4 */
            float4((float3(wedge.e12345) * other.group2.xyz), (other.e4 * wedge.e12345) - (other.e423 * wedge.e415) - (other.e431 * wedge.e425) - (other.e412 * wedge.e435) - (other.e415 * wedge.e423) - (other.e425 * wedge.e431) - (other.e435 * wedge.e412)), 
            /* e1, e2, e3, e5 */
            float4((other.e415 * wedge.e321) + (other.e321 * wedge.e415) + (other.e315 * wedge.e412) + (other.e1 * wedge.e12345), (other.e425 * wedge.e321) + (other.e321 * wedge.e425) + (other.e125 * wedge.e423) + (other.e2 * wedge.e12345), (other.e435 * wedge.e321) + (other.e321 * wedge.e435) + (other.e235 * wedge.e431) + (other.e3 * wedge.e12345), -(other.e435 * wedge.e125) - (other.e235 * wedge.e415) - (other.e315 * wedge.e425) - (other.e125 * wedge.e435)) + (wedge.group2.yzxw * float4(other.group0.xyz.zxy, other.e5)) - (wedge.group2.zxyx * float4(other.group0.xyz.yzx, other.e415)) - float4((wedge.group0.xyz.yzx * other.group2.zxy), other.e425 * wedge.e315)
        );
    }
}
public extension RoundPoint {
    public DualNum project_orthogonally_onto(AntiDualNum other) {
        return DualNum(
            /* e5, e12345 */
            float2(pow(other.e3215, 2) * this.e4, 1.0) * float2(-1.0, 0.0)
        );
    }
}
public extension RoundPoint {
    public AntiPlane project_orthogonally_onto(AntiFlatPoint other) {
        FlatPoint right_anti_dual = FlatPoint(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        Line wedge = Line(
            /* e415, e425, e435 */
            (float3(this.e4) * right_anti_dual.group0.xyz) - (float3(right_anti_dual.e45) * this.group0.xyz), 
            /* e235, e315, e125 */
            (right_anti_dual.group0.zxy * this.group0.yzx) - (right_anti_dual.group0.yzx * this.group0.zxy)
        );
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4((float3(other.e321) * wedge.group0.xyz), -(other.e235 * wedge.e415) - (other.e315 * wedge.e425) - (other.e125 * wedge.e435))
        );
    }
}
public extension RoundPoint {
    public AntiFlector project_orthogonally_onto(AntiFlector other) {
        Flector right_anti_dual = Flector(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        Motor wedge = Motor(
            /* e415, e425, e435, e12345 */
            float4(right_anti_dual.e45 * this.e1 * -1.0, right_anti_dual.e45 * this.e2 * -1.0, right_anti_dual.e45 * this.e3 * -1.0, (right_anti_dual.e4315 * this.e2) + (right_anti_dual.e4125 * this.e3) + (right_anti_dual.e3215 * this.e4)) + (this.group0.wwwx * float4(right_anti_dual.group0.xyz, right_anti_dual.e4235)), 
            /* e235, e315, e125, e5 */
            float4(((right_anti_dual.group0.zxy * this.group0.yzx) - (right_anti_dual.group0.yzx * this.group0.zxy)), 0.0)
        );
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(wedge.e12345) * other.group0, 
            /* e1, e2, e3, e5 */
            float4(other.e1 * wedge.e12345, other.e2 * wedge.e12345, other.e3 * wedge.e12345, -(other.e235 * wedge.e415) - (other.e315 * wedge.e425) - (other.e125 * wedge.e435)) + (wedge.group0 * float4(other.group0.www, other.e5))
        );
    }
}
public extension RoundPoint {
    public AntiPlane project_orthogonally_onto(AntiLine other) {
        Line right_anti_dual = Line(
            /* e415, e425, e435 */
            other.group0.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            other.group1.xyz * float3(-1.0)
        );
        Plane wedge = Plane(
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual.e425 * this.e3) + (right_anti_dual.e235 * this.e4), (right_anti_dual.e435 * this.e1) + (right_anti_dual.e315 * this.e4), (right_anti_dual.e415 * this.e2) + (right_anti_dual.e125 * this.e4), -(right_anti_dual.e315 * this.e2) - (right_anti_dual.e125 * this.e3)) - (this.group0.yzxx * float4(right_anti_dual.group0.xyz.zxy, right_anti_dual.e235))
        );
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(other.e12 * wedge.e4315 * -1.0, other.e23 * wedge.e4125 * -1.0, other.e31 * wedge.e4235 * -1.0, (other.e25 * wedge.e4315) + (other.e35 * wedge.e4125)) + (wedge.group0.zxyx * float4(other.group0.xyz.yzx, other.e15))
        );
    }
}
public extension RoundPoint {
    public AntiFlector project_orthogonally_onto(AntiMotor other) {
        Motor right_anti_dual = Motor(
            /* e415, e425, e435, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        Flector wedge = Flector(
            /* e15, e25, e35, e45 */
            float4(right_anti_dual.e5) * this.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual.e425 * this.e3) + (right_anti_dual.e235 * this.e4), (right_anti_dual.e435 * this.e1) + (right_anti_dual.e315 * this.e4), (right_anti_dual.e415 * this.e2) + (right_anti_dual.e125 * this.e4), -(right_anti_dual.e315 * this.e2) - (right_anti_dual.e125 * this.e3)) - (this.group0.yzxx * float4(right_anti_dual.group0.zxy, right_anti_dual.e235))
        );
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(float3(1.0), 0.0) * float4(other.group1.www, 0.0) * float4(wedge.group1.xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0), 
            /* e1, e2, e3, e5 */
            (wedge.group1.zxyx * float4(other.group0.yzx, other.e15)) + float4(float3(0.0), (other.e25 * wedge.e4315) + (other.e35 * wedge.e4125)) - float4((other.group0.zxy * wedge.group1.yzx), other.e3215 * wedge.e45)
        );
    }
}
public extension RoundPoint {
    public AntiPlane project_orthogonally_onto(AntiPlane other) {
        Plane right_anti_dual = Plane(
            /* e4235, e4315, e4125, e3215 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4((right_anti_dual.e4235 * this.e1) + (right_anti_dual.e4315 * this.e2) + (right_anti_dual.e4125 * this.e3) + (right_anti_dual.e3215 * this.e4)) * other.group0
        );
    }
}
public extension RoundPoint {
    public RoundPoint project_orthogonally_onto(AntiScalar other) {
        Scalar right_anti_dual = Scalar(
            /* scalar */
            other.e12345 * -1.0
        );
        RoundPoint wedge = RoundPoint(
            /* e1, e2, e3, e4 */
            float4(right_anti_dual.scalar) * this.group0, 
            /* e5 */
            this.e5 * right_anti_dual.scalar
        );
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(other.e12345) * wedge.group0, 
            /* e5 */
            other.e12345 * wedge.e5
        );
    }
}
public extension RoundPoint {
    public RoundPoint project_orthogonally_onto(Circle other) {
        Dipole right_anti_dual = Dipole(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35 */
            other.group2.xyz
        );
        Circle wedge = Circle(
            /* e423, e431, e412 */
            (float3(this.e4) * right_anti_dual.group1.xyz) + (right_anti_dual.group0.xyz.yzx * this.group0.zxy) - (right_anti_dual.group0.xyz.zxy * this.group0.yzx), 
            /* e415, e425, e435, e321 */
            float4((right_anti_dual.e41 * this.e5) + (right_anti_dual.e15 * this.e4), (right_anti_dual.e42 * this.e5) + (right_anti_dual.e25 * this.e4), (right_anti_dual.e43 * this.e5) + (right_anti_dual.e35 * this.e4), -(right_anti_dual.e31 * this.e2) - (right_anti_dual.e12 * this.e3)) - (right_anti_dual.group1.wwwx * this.group0.xyzx), 
            /* e235, e315, e125 */
            (float3(this.e5) * right_anti_dual.group1.xyz) + (right_anti_dual.group2.xyz.zxy * this.group0.yzx) - (right_anti_dual.group2.xyz.yzx * this.group0.zxy)
        );
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4((other.e412 * wedge.e315) + (other.e415 * wedge.e321) + (other.e321 * wedge.e415) + (other.e315 * wedge.e412), (other.e423 * wedge.e125) + (other.e425 * wedge.e321) + (other.e321 * wedge.e425) + (other.e125 * wedge.e423), (other.e431 * wedge.e235) + (other.e435 * wedge.e321) + (other.e321 * wedge.e435) + (other.e235 * wedge.e431), -(other.e412 * wedge.e435) - (other.e415 * wedge.e423) - (other.e425 * wedge.e431) - (other.e435 * wedge.e412)) - float4((other.group0.xyz.yzx * wedge.group2.xyz.zxy), other.e423 * wedge.e415) - float4((other.group2.xyz.zxy * wedge.group0.xyz.yzx), other.e431 * wedge.e425), 
            /* e5 */
            -(other.e415 * wedge.e235) - (other.e425 * wedge.e315) - (other.e435 * wedge.e125) - (other.e235 * wedge.e415) - (other.e315 * wedge.e425) - (other.e125 * wedge.e435)
        );
    }
}
public extension RoundPoint {
    public AntiDipoleInversion project_orthogonally_onto(CircleRotor other) {
        AntiCircleRotor right_anti_dual = AntiCircleRotor(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        AntiDipoleInversion wedge = AntiDipoleInversion(
            /* e423, e431, e412 */
            (float3(this.e4) * right_anti_dual.group1.xyz) + (right_anti_dual.group0.xyz.yzx * this.group0.zxy) - (right_anti_dual.group0.xyz.zxy * this.group0.yzx), 
            /* e415, e425, e435, e321 */
            float4((right_anti_dual.e41 * this.e5) + (right_anti_dual.e15 * this.e4), (right_anti_dual.e42 * this.e5) + (right_anti_dual.e25 * this.e4), (right_anti_dual.e43 * this.e5) + (right_anti_dual.e35 * this.e4), -(right_anti_dual.e31 * this.e2) - (right_anti_dual.e12 * this.e3)) - (right_anti_dual.group1.wwwx * this.group0.xyzx), 
            /* e235, e315, e125, e4 */
            float4(((float3(this.e5) * right_anti_dual.group1.xyz) + (right_anti_dual.group2.zxy * this.group0.yzx) - (right_anti_dual.group2.yzx * this.group0.zxy)), right_anti_dual.scalar * this.e4), 
            /* e1, e2, e3, e5 */
            float4(right_anti_dual.scalar) * float4(this.group0.xyz, this.e5)
        );
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(other.e12345) * wedge.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(other.e12345) * wedge.group1, 
            /* e235, e315, e125, e4 */
            float4((float3(other.e12345) * wedge.group2.xyz), (wedge.e4 * other.e12345) - (wedge.e423 * other.e415) - (wedge.e431 * other.e425) - (wedge.e412 * other.e435) - (wedge.e415 * other.e423) - (wedge.e425 * other.e431) - (wedge.e435 * other.e412)), 
            /* e1, e2, e3, e5 */
            float4((wedge.e415 * other.e321) + (wedge.e321 * other.e415) + (wedge.e315 * other.e412) + (wedge.e1 * other.e12345), (wedge.e425 * other.e321) + (wedge.e321 * other.e425) + (wedge.e125 * other.e423) + (wedge.e2 * other.e12345), (wedge.e435 * other.e321) + (wedge.e321 * other.e435) + (wedge.e235 * other.e431) + (wedge.e3 * other.e12345), -(wedge.e435 * other.e125) - (wedge.e235 * other.e415) - (wedge.e315 * other.e425) - (wedge.e125 * other.e435)) + (other.group2.yzxw * float4(wedge.group0.xyz.zxy, wedge.e5)) - (other.group2.zxyx * float4(wedge.group0.xyz.yzx, wedge.e415)) - float4((other.group0.xyz.yzx * wedge.group2.zxy), wedge.e425 * other.e315)
        );
    }
}
public extension RoundPoint {
    public RoundPoint project_orthogonally_onto(Dipole other) {
        Circle right_anti_dual = Circle(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125 */
            other.group2.xyz * float3(-1.0)
        );
        Sphere wedge = Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual.e425 * this.e3) + (right_anti_dual.e235 * this.e4), (right_anti_dual.e435 * this.e1) + (right_anti_dual.e315 * this.e4), (right_anti_dual.e415 * this.e2) + (right_anti_dual.e125 * this.e4), -(right_anti_dual.e321 * this.e5) - (right_anti_dual.e125 * this.e3)) - (this.group0.yzxy * float4(right_anti_dual.group1.zxy, right_anti_dual.e315)) - float4((float3(this.e5) * right_anti_dual.group0.xyz), right_anti_dual.e235 * this.e1), 
            /* e1234 */
            (right_anti_dual.e423 * this.e1) + (right_anti_dual.e431 * this.e2) + (right_anti_dual.e412 * this.e3) + (right_anti_dual.e321 * this.e4)
        );
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4((other.e41 * wedge.e3215) + (other.e31 * wedge.e4125), (other.e42 * wedge.e3215) + (other.e12 * wedge.e4235), (other.e43 * wedge.e3215) + (other.e23 * wedge.e4315), -(other.e43 * wedge.e4125) - (other.e45 * wedge.e1234)) - (wedge.group0.yzxy * float4(other.group1.zxy, other.e42)) - float4((float3(wedge.e1234) * other.group2.xyz), other.e41 * wedge.e4235), 
            /* e5 */
            (other.e45 * wedge.e3215) + (other.e15 * wedge.e4235) + (other.e25 * wedge.e4315) + (other.e35 * wedge.e4125)
        );
    }
}
public extension RoundPoint {
    public AntiDipoleInversion project_orthogonally_onto(DipoleInversion other) {
        AntiDipoleInversion right_anti_dual = AntiDipoleInversion(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e4 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group3 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        DipoleInversion wedge = DipoleInversion(
            /* e41, e42, e43 */
            (float3(this.e4) * right_anti_dual.group3.xyz) - (float3(right_anti_dual.e4) * this.group0.xyz), 
            /* e23, e31, e12, e45 */
            (right_anti_dual.group3.zxyw * this.group0.yzxw) - float4((right_anti_dual.group3.yzx * this.group0.zxy), right_anti_dual.e4 * this.e5), 
            /* e15, e25, e35, e1234 */
            float4(right_anti_dual.e1 * this.e5 * -1.0, right_anti_dual.e2 * this.e5 * -1.0, right_anti_dual.e3 * this.e5 * -1.0, (right_anti_dual.e431 * this.e2) + (right_anti_dual.e412 * this.e3) + (right_anti_dual.e321 * this.e4)) + (this.group0.xyzx * float4(right_anti_dual.group3.www, right_anti_dual.e423)), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual.e425 * this.e3) + (right_anti_dual.e235 * this.e4), (right_anti_dual.e435 * this.e1) + (right_anti_dual.e315 * this.e4), (right_anti_dual.e415 * this.e2) + (right_anti_dual.e125 * this.e4), -(right_anti_dual.e315 * this.e2) - (right_anti_dual.e125 * this.e3)) - (float4(this.e5) * float4(right_anti_dual.group0.xyz, right_anti_dual.e321)) - (this.group0.yzxx * float4(right_anti_dual.group1.zxy, right_anti_dual.e235))
        );
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (float3(other.e1234) * wedge.group3.xyz) - (float3(wedge.e1234) * other.group3.xyz), 
            /* e415, e425, e435, e321 */
            (wedge.group3.yzxw * float4(other.group3.zxy, other.e1234)) - (other.group3.yzxw * float4(wedge.group3.zxy, wedge.e1234)), 
            /* e235, e315, e125, e4 */
            float4(float3(0.0), (other.e1234 * wedge.e45) + (other.e4315 * wedge.e42) + (other.e4125 * wedge.e43) - (other.e42 * wedge.e4315) - (other.e43 * wedge.e4125) - (other.e45 * wedge.e1234)) + float4((wedge.group3.www * other.group3.xyz), other.e4235 * wedge.e41) - (wedge.group3.xyzx * float4(other.group3.www, other.e41)), 
            /* e1, e2, e3, e5 */
            (float4(wedge.e3215) * float4(other.group0.xyz, other.e45)) + (other.group2.wwwy * float4(wedge.group2.xyz, wedge.e4315)) + (wedge.group3.zxyx * float4(other.group1.yzx, other.e15)) + float4((other.group3.yzx * wedge.group1.zxy), other.e35 * wedge.e4125) - (other.group3.zxyw * wedge.group1.yzxw) - (wedge.group2.wwwz * float4(other.group2.xyz, other.e4125)) - float4((wedge.group0.xyz * other.group3.www), other.e4235 * wedge.e15) - float4((other.group1.zxy * wedge.group3.yzx), other.e4315 * wedge.e25)
        );
    }
}
public extension RoundPoint {
    public VersorEven project_orthogonally_onto(DualNum other) {
        AntiDualNum right_anti_dual = AntiDualNum(
            /* e3215, scalar */
            other.group0.xy * float2(-1.0)
        );
        VersorEven wedge = VersorEven(
            /* e423, e431, e412, e12345 */
            float4(float3(0.0), right_anti_dual.e3215 * this.e4), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e235, e315, e125, e5 */
            float4(float3(0.0), right_anti_dual.scalar * this.e5), 
            /* e1, e2, e3, e4 */
            float4(right_anti_dual.scalar) * this.group0
        );
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(other.e12345) * wedge.group0, 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e235, e315, e125, e5 */
            float4(other.group0.xy.yy, other.e12345, (other.e5 * wedge.e12345) + (other.e12345 * wedge.e5)) * float4(wedge.group2.xyz, 1.0), 
            /* e1, e2, e3, e4 */
            float4(other.e12345) * wedge.group3
        );
    }
}
public extension RoundPoint {
    public RoundPoint project_orthogonally_onto(FlatPoint other) {
        AntiFlatPoint right_anti_dual = AntiFlatPoint(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        Sphere wedge = Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((float3(this.e4) * right_anti_dual.group0.xyz), -(right_anti_dual.e235 * this.e1) - (right_anti_dual.e315 * this.e2) - (right_anti_dual.e125 * this.e3) - (right_anti_dual.e321 * this.e5)), 
            /* e1234 */
            right_anti_dual.e321 * this.e4
        );
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(wedge.e1234) * other.group0 * float4(-1.0), 
            /* e5 */
            (other.e15 * wedge.e4235) + (other.e25 * wedge.e4315) + (other.e35 * wedge.e4125) + (other.e45 * wedge.e3215)
        );
    }
}
public extension RoundPoint {
    public AntiDipoleInversion project_orthogonally_onto(Flector other) {
        AntiFlector right_anti_dual = AntiFlector(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        DipoleInversion wedge = DipoleInversion(
            /* e41, e42, e43 */
            float3(this.e4) * right_anti_dual.group1.xyz, 
            /* e23, e31, e12, e45 */
            float4(((right_anti_dual.group1.zxy * this.group0.yzx) - (right_anti_dual.group1.yzx * this.group0.zxy)), right_anti_dual.e5 * this.e4), 
            /* e15, e25, e35, e1234 */
            float4(((float3(right_anti_dual.e5) * this.group0.xyz) - (float3(this.e5) * right_anti_dual.group1.xyz)), right_anti_dual.e321 * this.e4), 
            /* e4235, e4315, e4125, e3215 */
            float4((float3(this.e4) * right_anti_dual.group0.xyz), -(right_anti_dual.e235 * this.e1) - (right_anti_dual.e315 * this.e2) - (right_anti_dual.e125 * this.e3) - (right_anti_dual.e321 * this.e5))
        );
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(wedge.e1234) * other.group1.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            float4(((wedge.group3.yzx * other.group1.zxy) - (wedge.group3.zxy * other.group1.yzx)), wedge.e1234 * other.e3215 * -1.0), 
            /* e235, e315, e125, e4 */
            (other.group1.xyzx * float4(wedge.group3.www, wedge.e41)) + float4(float3(0.0), (wedge.e42 * other.e4315) + (wedge.e43 * other.e4125)) - float4((other.group1.www * wedge.group3.xyz), wedge.e1234 * other.e45), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), (wedge.e4315 * other.e25) + (wedge.e4125 * other.e35) + (wedge.e3215 * other.e45) - (wedge.e35 * other.e4125)) + float4((wedge.group1.zxy * other.group1.yzx), wedge.e4235 * other.e15) - (float4(other.e3215) * float4(wedge.group0.xyz, wedge.e45)) - (wedge.group2.wwwy * float4(other.group0.xyz, other.e4315)) - (other.group1.zxyx * float4(wedge.group1.yzx, wedge.e15))
        );
    }
}
public extension RoundPoint {
    public RoundPoint project_orthogonally_onto(Line other) {
        AntiLine right_anti_dual = AntiLine(
            /* e23, e31, e12 */
            other.group0.xyz, 
            /* e15, e25, e35 */
            other.group1.xyz
        );
        Circle wedge = Circle(
            /* e423, e431, e412 */
            float3(this.e4) * right_anti_dual.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4((float3(this.e4) * right_anti_dual.group1.xyz), -(right_anti_dual.e23 * this.e1) - (right_anti_dual.e31 * this.e2) - (right_anti_dual.e12 * this.e3)), 
            /* e235, e315, e125 */
            (float3(this.e5) * right_anti_dual.group0.xyz) + (right_anti_dual.group1.xyz.zxy * this.group0.yzx) - (right_anti_dual.group1.xyz.yzx * this.group0.zxy)
        );
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4((wedge.e412 * other.e315) + (wedge.e321 * other.e415), (wedge.e423 * other.e125) + (wedge.e321 * other.e425), (wedge.e431 * other.e235) + (wedge.e321 * other.e435), -(wedge.e431 * other.e425) - (wedge.e412 * other.e435)) - float4((wedge.group0.xyz.yzx * other.group1.xyz.zxy), wedge.e423 * other.e415), 
            /* e5 */
            -(wedge.e415 * other.e235) - (wedge.e425 * other.e315) - (wedge.e435 * other.e125) - (wedge.e235 * other.e415) - (wedge.e315 * other.e425) - (wedge.e125 * other.e435)
        );
    }
}
public extension RoundPoint {
    public VersorEven project_orthogonally_onto(Motor other) {
        AntiMotor right_anti_dual = AntiMotor(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        VersorEven wedge = VersorEven(
            /* e423, e431, e412, e12345 */
            float4(this.e4) * float4(right_anti_dual.group0.xyz, right_anti_dual.e3215), 
            /* e415, e425, e435, e321 */
            float4((float3(this.e4) * right_anti_dual.group1.xyz), -(right_anti_dual.e23 * this.e1) - (right_anti_dual.e31 * this.e2) - (right_anti_dual.e12 * this.e3)), 
            /* e235, e315, e125, e5 */
            float4(((float3(this.e5) * right_anti_dual.group0.xyz) + (right_anti_dual.group1.zxy * this.group0.yzx) - (right_anti_dual.group1.yzx * this.group0.zxy)), right_anti_dual.scalar * this.e5), 
            /* e1, e2, e3, e4 */
            float4(right_anti_dual.scalar) * this.group0
        );
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(other.e12345) * wedge.group0, 
            /* e415, e425, e435, e321 */
            float4(((float3(other.e12345) * wedge.group1.xyz) + (float3(wedge.e12345) * other.group0.xyz)), other.e12345 * wedge.e321), 
            /* e235, e315, e125, e5 */
            (float4(other.e12345) * wedge.group2) + (float4(wedge.e12345) * other.group1) + float4(float3(0.0), -(other.e415 * wedge.e235) - (other.e425 * wedge.e315) - (other.e435 * wedge.e125) - (other.e235 * wedge.e415) - (other.e315 * wedge.e425) - (other.e125 * wedge.e435)), 
            /* e1, e2, e3, e4 */
            float4((other.e12345 * wedge.e1) + (other.e315 * wedge.e412), (other.e12345 * wedge.e2) + (other.e125 * wedge.e423), (other.e12345 * wedge.e3) + (other.e235 * wedge.e431), -(other.e425 * wedge.e431) - (other.e435 * wedge.e412)) + (other.group0 * float4(wedge.group1.www, wedge.e4)) - (wedge.group0.yzxx * float4(other.group1.zxy, other.e415))
        );
    }
}
public extension RoundPoint {
    public MultiVector project_orthogonally_onto(MultiVector other) {
        MultiVector right_anti_dual = MultiVector(
            /* scalar, e12345 */
            other.group0.xy.yx * float2(-1.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4(other.group9.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5 */
            other.e3215, 
            /* e15, e25, e35, e45 */
            float4(other.group8.xyz, other.e321 * -1.0), 
            /* e41, e42, e43 */
            other.group7.xyz, 
            /* e23, e31, e12 */
            other.group6.xyz, 
            /* e415, e425, e435, e321 */
            float4(other.group5.xyz, other.e45) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e423, e431, e412 */
            other.group4.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            other.group3.xyz * float3(-1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.group1.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234 */
            other.e4 * -1.0
        );
        MultiVector wedge = MultiVector(
            /* scalar, e12345 */
            float2(0.0, (right_anti_dual.e4235 * this.e1) + (right_anti_dual.e4315 * this.e2) + (right_anti_dual.e4125 * this.e3) + (right_anti_dual.e3215 * this.e4) + (right_anti_dual.e1234 * this.e5)), 
            /* e1, e2, e3, e4 */
            float4(right_anti_dual.scalar) * this.group0, 
            /* e5 */
            right_anti_dual.scalar * this.e5, 
            /* e15, e25, e35, e45 */
            (float4(right_anti_dual.e5) * this.group0) - (float4(this.e5) * right_anti_dual.group1), 
            /* e41, e42, e43 */
            (float3(this.e4) * right_anti_dual.group1.xyz) - (float3(right_anti_dual.e4) * this.group0.xyz), 
            /* e23, e31, e12 */
            (right_anti_dual.group1.zxy * this.group0.yzx) - (right_anti_dual.group1.yzx * this.group0.zxy), 
            /* e415, e425, e435, e321 */
            float4((right_anti_dual.e15 * this.e4) + (right_anti_dual.e41 * this.e5), (right_anti_dual.e25 * this.e4) + (right_anti_dual.e42 * this.e5), (right_anti_dual.e35 * this.e4) + (right_anti_dual.e43 * this.e5), -(right_anti_dual.e31 * this.e2) - (right_anti_dual.e12 * this.e3)) - (this.group0.xyzx * float4(right_anti_dual.group3.www, right_anti_dual.e23)), 
            /* e423, e431, e412 */
            (float3(this.e4) * right_anti_dual.group5.xyz) + (right_anti_dual.group4.xyz.yzx * this.group0.zxy) - (right_anti_dual.group4.xyz.zxy * this.group0.yzx), 
            /* e235, e315, e125 */
            (float3(this.e5) * right_anti_dual.group5.xyz) + (right_anti_dual.group3.zxy * this.group0.yzx) - (right_anti_dual.group3.yzx * this.group0.zxy), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual.e425 * this.e3) + (right_anti_dual.e235 * this.e4), (right_anti_dual.e435 * this.e1) + (right_anti_dual.e315 * this.e4), (right_anti_dual.e415 * this.e2) + (right_anti_dual.e125 * this.e4), -(right_anti_dual.e321 * this.e5) - (right_anti_dual.e125 * this.e3)) - (this.group0.yzxy * float4(right_anti_dual.group6.zxy, right_anti_dual.e315)) - float4((float3(this.e5) * right_anti_dual.group7.xyz), right_anti_dual.e235 * this.e1), 
            /* e1234 */
            (right_anti_dual.e321 * this.e4) + (right_anti_dual.e423 * this.e1) + (right_anti_dual.e431 * this.e2) + (right_anti_dual.e412 * this.e3)
        );
        return MultiVector(
            /* scalar, e12345 */
            float2((other.scalar * wedge.e12345) + (other.e12345 * wedge.scalar) + (other.e1 * wedge.e4235) + (other.e2 * wedge.e4315) + (other.e3 * wedge.e4125) + (other.e4 * wedge.e3215) + (other.e5 * wedge.e1234) + (other.e4235 * wedge.e1) + (other.e4315 * wedge.e2) + (other.e4125 * wedge.e3) + (other.e3215 * wedge.e4) + (other.e1234 * wedge.e5) - (other.e15 * wedge.e423) - (other.e25 * wedge.e431) - (other.e35 * wedge.e412) - (other.e45 * wedge.e321) - (other.e41 * wedge.e235) - (other.e42 * wedge.e315) - (other.e43 * wedge.e125) - (other.e23 * wedge.e415) - (other.e31 * wedge.e425) - (other.e12 * wedge.e435) - (other.e415 * wedge.e23) - (other.e425 * wedge.e31) - (other.e435 * wedge.e12) - (other.e321 * wedge.e45) - (other.e423 * wedge.e15) - (other.e431 * wedge.e25) - (other.e412 * wedge.e35) - (other.e235 * wedge.e41) - (other.e315 * wedge.e42) - (other.e125 * wedge.e43), other.e12345 * wedge.e12345), 
            /* e1, e2, e3, e4 */
            float4((other.e415 * wedge.e321) + (other.e321 * wedge.e415) + (other.e4315 * wedge.e12) + (other.e1234 * wedge.e15), (other.e425 * wedge.e321) + (other.e321 * wedge.e425) + (other.e4125 * wedge.e23) + (other.e1234 * wedge.e25), (other.e435 * wedge.e321) + (other.e321 * wedge.e435) + (other.e4235 * wedge.e31) + (other.e1234 * wedge.e35), -(other.e45 * wedge.e1234) - (other.e415 * wedge.e423) - (other.e425 * wedge.e431) - (other.e435 * wedge.e412)) + (float4(other.e12345) * wedge.group1) + (float4(wedge.e12345) * other.group1) + float4((other.group4.xyz * wedge.group9.www), other.e4235 * wedge.e41) + float4((other.group5.xyz.yzx * wedge.group9.zxy), other.e4315 * wedge.e42) + float4((other.group7.xyz.zxy * wedge.group8.xyz.yzx), other.e4125 * wedge.e43) + float4((other.group8.xyz.yzx * wedge.group7.xyz.zxy), other.e1234 * wedge.e45) - (wedge.group9.yzxx * float4(other.group5.xyz.zxy, other.e41)) - float4((float3(wedge.e1234) * other.group3.xyz), other.e412 * wedge.e435) - float4((wedge.group4.xyz * other.group9.www), other.e423 * wedge.e415) - float4((other.group7.xyz.yzx * wedge.group8.xyz.zxy), other.e42 * wedge.e4315) - float4((other.group8.xyz.zxy * wedge.group7.xyz.yzx), other.e43 * wedge.e4125) - float4((wedge.group5.xyz.yzx * other.group9.zxy), other.e431 * wedge.e425), 
            /* e5 */
            (other.e12345 * wedge.e5) + (other.e5 * wedge.e12345) + (other.e15 * wedge.e4235) + (other.e25 * wedge.e4315) + (other.e35 * wedge.e4125) + (other.e45 * wedge.e3215) - (other.e415 * wedge.e235) - (other.e425 * wedge.e315) - (other.e435 * wedge.e125) - (other.e235 * wedge.e415) - (other.e315 * wedge.e425) - (other.e125 * wedge.e435) - (other.e4235 * wedge.e15) - (other.e4315 * wedge.e25) - (other.e4125 * wedge.e35) - (other.e3215 * wedge.e45), 
            /* e15, e25, e35, e45 */
            float4((other.e415 * wedge.e3215) + (other.e315 * wedge.e4125) + (other.e4125 * wedge.e315) + (other.e3215 * wedge.e415), (other.e425 * wedge.e3215) + (other.e125 * wedge.e4235) + (other.e4235 * wedge.e125) + (other.e3215 * wedge.e425), (other.e435 * wedge.e3215) + (other.e235 * wedge.e4315) + (other.e4315 * wedge.e235) + (other.e3215 * wedge.e435), -(other.e435 * wedge.e4125) - (other.e4235 * wedge.e415) - (other.e4315 * wedge.e425) - (other.e4125 * wedge.e435)) + (float4(other.e12345) * wedge.group3) + (float4(wedge.e12345) * other.group3) - (wedge.group9.yzxx * float4(other.group8.xyz.zxy, other.e415)) - float4((wedge.group8.xyz.zxy * other.group9.yzx), other.e425 * wedge.e4315), 
            /* e41, e42, e43 */
            (float3(other.e12345) * wedge.group4.xyz) + (float3(other.e1234) * wedge.group6.xyz) + (float3(wedge.e12345) * other.group4.xyz) + (float3(wedge.e1234) * other.group6.xyz) + (other.group7.xyz.zxy * wedge.group9.yzx) + (wedge.group7.xyz.zxy * other.group9.yzx) - (other.group7.xyz.yzx * wedge.group9.zxy) - (wedge.group7.xyz.yzx * other.group9.zxy), 
            /* e23, e31, e12 */
            (float3(other.e12345) * wedge.group5.xyz) + (float3(other.e3215) * wedge.group7.xyz) + (float3(other.e1234) * wedge.group8.xyz) + (float3(wedge.e12345) * other.group5.xyz) + (float3(wedge.e3215) * other.group7.xyz) + (float3(wedge.e1234) * other.group8.xyz) - (float3(other.e321) * wedge.group9.xyz) - (float3(wedge.e321) * other.group9.xyz), 
            /* e415, e425, e435, e321 */
            (float4(other.e12345) * wedge.group6) + (float4(wedge.e12345) * other.group6) + float4((other.group9.zxy * wedge.group9.yzx), other.e1234 * wedge.e3215) - (other.group9.yzxw * float4(wedge.group9.zxy, wedge.e1234)), 
            /* e423, e431, e412 */
            (float3(other.e12345) * wedge.group7.xyz) + (float3(other.e1234) * wedge.group9.xyz) + (float3(wedge.e12345) * other.group7.xyz) - (float3(wedge.e1234) * other.group9.xyz), 
            /* e235, e315, e125 */
            (float3(other.e12345) * wedge.group8.xyz) + (float3(wedge.e12345) * other.group8.xyz) + (float3(wedge.e3215) * other.group9.xyz) - (float3(other.e3215) * wedge.group9.xyz), 
            /* e4235, e4315, e4125, e3215 */
            (float4(other.e12345) * wedge.group9) + (float4(wedge.e12345) * other.group9), 
            /* e1234 */
            (other.e12345 * wedge.e1234) + (other.e1234 * wedge.e12345)
        );
    }
}
public extension RoundPoint {
    public RoundPoint project_orthogonally_onto(Plane other) {
        AntiPlane right_anti_dual = AntiPlane(
            /* e1, e2, e3, e5 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        Dipole wedge = Dipole(
            /* e41, e42, e43 */
            float3(this.e4) * right_anti_dual.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(((right_anti_dual.group0.zxy * this.group0.yzx) - (right_anti_dual.group0.yzx * this.group0.zxy)), right_anti_dual.e5 * this.e4), 
            /* e15, e25, e35 */
            (float3(right_anti_dual.e5) * this.group0.xyz) - (float3(this.e5) * right_anti_dual.group0.xyz)
        );
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(-(wedge.e41 * other.e3215) - (wedge.e31 * other.e4125), -(wedge.e42 * other.e3215) - (wedge.e12 * other.e4235), -(wedge.e43 * other.e3215) - (wedge.e23 * other.e4315), (wedge.e42 * other.e4315) + (wedge.e43 * other.e4125)) + (other.group0.yzxx * float4(wedge.group1.zxy, wedge.e41)), 
            /* e5 */
            -(wedge.e45 * other.e3215) - (wedge.e15 * other.e4235) - (wedge.e25 * other.e4315) - (wedge.e35 * other.e4125)
        );
    }
}
public extension RoundPoint {
    public RoundPoint project_orthogonally_onto(RoundPoint other) {
        Sphere right_anti_dual = Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234 */
            other.e4 * -1.0
        );
        AntiScalar wedge = AntiScalar(
            /* e12345 */
            (this.e1 * right_anti_dual.e4235) + (this.e2 * right_anti_dual.e4315) + (this.e3 * right_anti_dual.e4125) + (this.e4 * right_anti_dual.e3215) + (this.e5 * right_anti_dual.e1234)
        );
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(wedge.e12345) * other.group0, 
            /* e5 */
            wedge.e12345 * other.e5
        );
    }
}
public extension RoundPoint {
    public RoundPoint project_orthogonally_onto(Sphere other) {
        RoundPoint right_anti_dual = RoundPoint(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5 */
            other.e3215
        );
        Dipole wedge = Dipole(
            /* e41, e42, e43 */
            (float3(this.e4) * right_anti_dual.group0.xyz) - (float3(right_anti_dual.e4) * this.group0.xyz), 
            /* e23, e31, e12, e45 */
            float4((right_anti_dual.group0.zxy * this.group0.yzx), right_anti_dual.e5 * this.e4) - (right_anti_dual.group0.yzxw * float4(this.group0.zxy, this.e5)), 
            /* e15, e25, e35 */
            (float3(right_anti_dual.e5) * this.group0.xyz) - (float3(this.e5) * right_anti_dual.group0.xyz)
        );
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(-(wedge.e41 * other.e3215) - (wedge.e31 * other.e4125), -(wedge.e42 * other.e3215) - (wedge.e12 * other.e4235), -(wedge.e43 * other.e3215) - (wedge.e23 * other.e4315), (wedge.e43 * other.e4125) + (wedge.e45 * other.e1234)) + (other.group0.yzxy * float4(wedge.group1.zxy, wedge.e42)) + float4((float3(other.e1234) * wedge.group2.xyz), wedge.e41 * other.e4235), 
            /* e5 */
            -(wedge.e45 * other.e3215) - (wedge.e15 * other.e4235) - (wedge.e25 * other.e4315) - (wedge.e35 * other.e4125)
        );
    }
}
public extension RoundPoint {
    public VersorEven project_orthogonally_onto(VersorEven other) {
        VersorOdd right_anti_dual = VersorOdd(
            /* e41, e42, e43, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e1234 */
            float4(other.group2.xyz, other.e4) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.group3.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0)
        );
        VersorEven wedge = VersorEven(
            /* e423, e431, e412, e12345 */
            float4(this.e2 * right_anti_dual.e43 * -1.0, this.e3 * right_anti_dual.e41 * -1.0, this.e1 * right_anti_dual.e42 * -1.0, (this.e3 * right_anti_dual.e4125) + (this.e4 * right_anti_dual.e3215) + (this.e5 * right_anti_dual.e1234)) + (this.group0.zxyx * float4(right_anti_dual.group0.yzx, right_anti_dual.e4235)) + (this.group0.wwwy * float4(right_anti_dual.group1.xyz, right_anti_dual.e4315)), 
            /* e415, e425, e435, e321 */
            float4((this.e4 * right_anti_dual.e15) + (this.e5 * right_anti_dual.e41), (this.e4 * right_anti_dual.e25) + (this.e5 * right_anti_dual.e42), (this.e4 * right_anti_dual.e35) + (this.e5 * right_anti_dual.e43), -(this.e2 * right_anti_dual.e31) - (this.e3 * right_anti_dual.e12)) - (this.group0.xyzx * right_anti_dual.group1.wwwx), 
            /* e235, e315, e125, e5 */
            float4(((float3(this.e5) * right_anti_dual.group1.xyz) + (this.group0.yzx * right_anti_dual.group2.zxy) - (this.group0.zxy * right_anti_dual.group2.yzx)), this.e5 * right_anti_dual.scalar), 
            /* e1, e2, e3, e4 */
            float4(right_anti_dual.scalar) * this.group0
        );
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(((float3(other.e12345) * wedge.group0.xyz) + (float3(wedge.e12345) * other.group0.xyz)), other.e12345 * wedge.e12345), 
            /* e415, e425, e435, e321 */
            (float4(other.e12345) * wedge.group1) + (float4(wedge.e12345) * other.group1), 
            /* e235, e315, e125, e5 */
            (float4(other.e12345) * wedge.group2) + (float4(wedge.e12345) * other.group2) + float4(float3(0.0), -(other.e415 * wedge.e235) - (other.e425 * wedge.e315) - (other.e435 * wedge.e125) - (other.e235 * wedge.e415) - (other.e315 * wedge.e425) - (other.e125 * wedge.e435)), 
            /* e1, e2, e3, e4 */
            float4((other.e415 * wedge.e321) + (other.e321 * wedge.e415) + (other.e315 * wedge.e412) + (other.e1 * wedge.e12345), (other.e425 * wedge.e321) + (other.e321 * wedge.e425) + (other.e125 * wedge.e423) + (other.e2 * wedge.e12345), (other.e435 * wedge.e321) + (other.e321 * wedge.e435) + (other.e235 * wedge.e431) + (other.e3 * wedge.e12345), -(other.e412 * wedge.e435) - (other.e415 * wedge.e423) - (other.e425 * wedge.e431) - (other.e435 * wedge.e412)) + (other.group0.zxyw * float4(wedge.group2.yzx, wedge.e4)) + float4((other.group0.www * wedge.group3.xyz), other.e4 * wedge.e12345) - (other.group0.yzxx * float4(wedge.group2.zxy, wedge.e415)) - float4((other.group2.zxy * wedge.group0.yzx), other.e431 * wedge.e425)
        );
    }
}
public extension RoundPoint {
    public AntiDipoleInversion project_orthogonally_onto(VersorOdd other) {
        VersorEven right_anti_dual = VersorEven(
            /* e423, e431, e412, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            float4(other.group2.xyz, other.e3215) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4(other.group3.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0)
        );
        DipoleInversion wedge = DipoleInversion(
            /* e41, e42, e43 */
            (float3(this.e4) * right_anti_dual.group3.xyz) - (float3(right_anti_dual.e4) * this.group0.xyz), 
            /* e23, e31, e12, e45 */
            (this.group0.yzxw * float4(right_anti_dual.group3.zxy, right_anti_dual.e5)) - float4((this.group0.zxy * right_anti_dual.group3.yzx), this.e5 * right_anti_dual.e4), 
            /* e15, e25, e35, e1234 */
            float4(this.e5 * right_anti_dual.e1 * -1.0, this.e5 * right_anti_dual.e2 * -1.0, this.e5 * right_anti_dual.e3 * -1.0, (this.e2 * right_anti_dual.e431) + (this.e3 * right_anti_dual.e412) + (this.e4 * right_anti_dual.e321)) + (this.group0.xyzx * float4(right_anti_dual.group2.www, right_anti_dual.e423)), 
            /* e4235, e4315, e4125, e3215 */
            float4((this.e3 * right_anti_dual.e425) + (this.e4 * right_anti_dual.e235), (this.e1 * right_anti_dual.e435) + (this.e4 * right_anti_dual.e315), (this.e2 * right_anti_dual.e415) + (this.e4 * right_anti_dual.e125), -(this.e3 * right_anti_dual.e125) - (this.e5 * right_anti_dual.e321)) - (this.group0.yzxx * float4(right_anti_dual.group1.zxy, right_anti_dual.e235)) - float4((float3(this.e5) * right_anti_dual.group0.xyz), this.e2 * right_anti_dual.e315)
        );
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (float3(other.e1234) * wedge.group3.xyz) - (float3(wedge.e1234) * other.group3.xyz), 
            /* e415, e425, e435, e321 */
            (wedge.group3.yzxw * float4(other.group3.zxy, other.e1234)) - (other.group3.yzxw * float4(wedge.group3.zxy, wedge.e1234)), 
            /* e235, e315, e125, e4 */
            (other.group3.xyzx * float4(wedge.group3.www, wedge.e41)) + float4(float3(0.0), (wedge.e42 * other.e4315) + (wedge.e43 * other.e4125) + (wedge.e45 * other.e1234) - (wedge.e4235 * other.e41) - (wedge.e4315 * other.e42) - (wedge.e4125 * other.e43)) - float4((other.group3.www * wedge.group3.xyz), wedge.e1234 * other.e45), 
            /* e1, e2, e3, e5 */
            (float4(wedge.e3215) * float4(other.group0.xyz, other.e45)) + (wedge.group3.zxyz * float4(other.group1.yzx, other.e35)) + (other.group2.wwwy * float4(wedge.group2.xyz, wedge.e4315)) + float4((wedge.group1.zxy * other.group3.yzx), wedge.e4235 * other.e15) - (float4(other.e3215) * float4(wedge.group0.xyz, wedge.e45)) - (wedge.group2.wwwy * float4(other.group2.xyz, other.e4315)) - (other.group3.zxyx * float4(wedge.group1.yzx, wedge.e15)) - float4((wedge.group3.yzx * other.group1.zxy), wedge.e35 * other.e4125)
        );
    }
}
public extension Scalar {
    // Fancy infix trick (first half)
    public ProjectOrthogonallyOntoInfixPartial<Scalar> operator /(ProjectOrthogonallyOntoInfix rhs) {
        return ProjectOrthogonallyOntoInfixPartial<Scalar>(this);
    }
}
public extension ProjectOrthogonallyOntoInfixPartial<Scalar> {
    // Fancy infix trick (second half)
    public AntiCircleRotor operator /(AntiCircleRotor rhs) {
        return this.a.project_orthogonally_onto(rhs);
    }
}
public extension Scalar {
    public AntiCircleRotor project_orthogonally_onto(AntiCircleRotor other) {
        CircleRotor right_anti_dual = CircleRotor(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        CircleRotor wedge = CircleRotor(
            /* e423, e431, e412 */
            float3(this.scalar) * right_anti_dual.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(this.scalar) * right_anti_dual.group1, 
            /* e235, e315, e125, e12345 */
            float4(this.scalar) * right_anti_dual.group2
        );
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(wedge.e12345) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(wedge.e12345) * other.group1, 
            /* e15, e25, e35, scalar */
            float4((float3(wedge.e12345) * other.group2.xyz), (other.scalar * wedge.e12345) - (other.e41 * wedge.e235) - (other.e42 * wedge.e315) - (other.e43 * wedge.e125) - (other.e23 * wedge.e415) - (other.e31 * wedge.e425) - (other.e12 * wedge.e435) - (other.e45 * wedge.e321) - (other.e15 * wedge.e423) - (other.e25 * wedge.e431) - (other.e35 * wedge.e412))
        );
    }
}
public extension Scalar {
    public AntiCircleRotor project_orthogonally_onto(AntiDipoleInversion other) {
        DipoleInversion right_anti_dual = DipoleInversion(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e1234 */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group3 * float4(1.0, 1.0, 1.0, -1.0)
        );
        DipoleInversion wedge = DipoleInversion(
            /* e41, e42, e43 */
            float3(this.scalar) * right_anti_dual.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(this.scalar) * right_anti_dual.group1, 
            /* e15, e25, e35, e1234 */
            float4(this.scalar) * right_anti_dual.group2, 
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar) * right_anti_dual.group3
        );
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (float3(wedge.e1234) * other.group1.xyz) + (other.group0.xyz.zxy * wedge.group3.yzx) - (other.group0.xyz.yzx * wedge.group3.zxy), 
            /* e23, e31, e12, e45 */
            float4((other.e423 * wedge.e3215) + (other.e235 * wedge.e1234), (other.e431 * wedge.e3215) + (other.e315 * wedge.e1234), (other.e412 * wedge.e3215) + (other.e125 * wedge.e1234), -(other.e425 * wedge.e4315) - (other.e435 * wedge.e4125)) - (other.group1.wwwx * wedge.group3.xyzx), 
            /* e15, e25, e35, scalar */
            (float4(wedge.e3215) * float4(other.group1.xyz, other.e4)) + (wedge.group3.zxyx * float4(other.group2.yzx, other.e1)) + float4(float3(0.0), (other.e2 * wedge.e4315) + (other.e3 * wedge.e4125) + (other.e5 * wedge.e1234) - (other.e431 * wedge.e25) - (other.e412 * wedge.e35) - (other.e415 * wedge.e23) - (other.e425 * wedge.e31) - (other.e435 * wedge.e12) - (other.e321 * wedge.e45) - (other.e235 * wedge.e41) - (other.e315 * wedge.e42) - (other.e125 * wedge.e43)) - float4((other.group2.zxy * wedge.group3.yzx), other.e423 * wedge.e15)
        );
    }
}
public extension Scalar {
    public AntiDualNum project_orthogonally_onto(AntiDualNum other) {
        return AntiDualNum(
            /* e3215, scalar */
            float2(other.scalar * this.scalar) * other.group0.xy
        );
    }
}
public extension Scalar {
    public Scalar project_orthogonally_onto(AntiFlatPoint other) {
        return Scalar(
            /* scalar */
            pow(other.e321, 2) * this.scalar
        );
    }
}
public extension Scalar {
    public AntiMotor project_orthogonally_onto(AntiFlector other) {
        Flector right_anti_dual = Flector(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        Flector wedge = Flector(
            /* e15, e25, e35, e45 */
            float4(this.scalar) * right_anti_dual.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar) * right_anti_dual.group1
        );
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((other.group0.www * wedge.group1.xyz * float3(-1.0)), (other.e1 * wedge.e4235) + (other.e2 * wedge.e4315) + (other.e3 * wedge.e4125) - (other.e321 * wedge.e45)), 
            /* e15, e25, e35, e3215 */
            float4(((other.group0.yzx * wedge.group1.zxy) - (other.group0.zxy * wedge.group1.yzx)), 0.0)
        );
    }
}
public extension Scalar {
    public Scalar project_orthogonally_onto(AntiLine other) {
        Line right_anti_dual = Line(
            /* e415, e425, e435 */
            other.group0.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            other.group1.xyz * float3(-1.0)
        );
        Line wedge = Line(
            /* e415, e425, e435 */
            float3(this.scalar) * right_anti_dual.group0.xyz, 
            /* e235, e315, e125 */
            float3(this.scalar) * right_anti_dual.group1.xyz
        );
        return Scalar(
            /* scalar */
            -(other.e23 * wedge.e415) - (other.e31 * wedge.e425) - (other.e12 * wedge.e435)
        );
    }
}
public extension Scalar {
    public AntiMotor project_orthogonally_onto(AntiMotor other) {
        Motor right_anti_dual = Motor(
            /* e415, e425, e435, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        Motor wedge = Motor(
            /* e415, e425, e435, e12345 */
            float4(this.scalar) * right_anti_dual.group0, 
            /* e235, e315, e125, e5 */
            float4(this.scalar) * right_anti_dual.group1
        );
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((float3(wedge.e12345) * other.group0.xyz), (other.scalar * wedge.e12345) - (other.e23 * wedge.e415) - (other.e31 * wedge.e425) - (other.e12 * wedge.e435)), 
            /* e15, e25, e35, e3215 */
            float4(((float3(other.e3215) * wedge.group0.xyz) + (float3(wedge.e12345) * other.group1.xyz)), other.e3215 * wedge.e12345)
        );
    }
}
public extension Scalar {
    public Scalar project_orthogonally_onto(AntiPlane other) {
        Plane wedge = Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar) * float4(other.group0.xyz, other.e5 * -1.0)
        );
        return Scalar(
            /* scalar */
            (other.e1 * wedge.e4235) + (other.e2 * wedge.e4315) + (other.e3 * wedge.e4125)
        );
    }
}
public extension Scalar {
    public Scalar project_orthogonally_onto(AntiScalar other) {
        return Scalar(
            /* scalar */
            pow(other.e12345, 2) * this.scalar * -1.0
        );
    }
}
public extension Scalar {
    public Scalar project_orthogonally_onto(Circle other) {
        Dipole right_anti_dual = Dipole(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35 */
            other.group2.xyz
        );
        Dipole wedge = Dipole(
            /* e41, e42, e43 */
            float3(this.scalar) * right_anti_dual.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(this.scalar) * right_anti_dual.group1, 
            /* e15, e25, e35 */
            float3(this.scalar) * right_anti_dual.group2.xyz
        );
        return Scalar(
            /* scalar */
            -(other.e423 * wedge.e15) - (other.e431 * wedge.e25) - (other.e412 * wedge.e35) - (other.e415 * wedge.e23) - (other.e425 * wedge.e31) - (other.e435 * wedge.e12) - (other.e321 * wedge.e45) - (other.e235 * wedge.e41) - (other.e315 * wedge.e42) - (other.e125 * wedge.e43)
        );
    }
}
public extension Scalar {
    public AntiCircleRotor project_orthogonally_onto(CircleRotor other) {
        AntiCircleRotor right_anti_dual = AntiCircleRotor(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        AntiCircleRotor wedge = AntiCircleRotor(
            /* e41, e42, e43 */
            float3(this.scalar) * right_anti_dual.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(this.scalar) * right_anti_dual.group1, 
            /* e15, e25, e35, scalar */
            float4(this.scalar) * right_anti_dual.group2
        );
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(other.e12345) * wedge.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(other.e12345) * wedge.group1, 
            /* e15, e25, e35, scalar */
            float4((float3(other.e12345) * wedge.group2.xyz), (wedge.scalar * other.e12345) - (wedge.e41 * other.e235) - (wedge.e42 * other.e315) - (wedge.e43 * other.e125) - (wedge.e23 * other.e415) - (wedge.e31 * other.e425) - (wedge.e12 * other.e435) - (wedge.e45 * other.e321) - (wedge.e15 * other.e423) - (wedge.e25 * other.e431) - (wedge.e35 * other.e412))
        );
    }
}
public extension Scalar {
    public Scalar project_orthogonally_onto(Dipole other) {
        Circle right_anti_dual = Circle(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125 */
            other.group2.xyz * float3(-1.0)
        );
        Circle wedge = Circle(
            /* e423, e431, e412 */
            float3(this.scalar) * right_anti_dual.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(this.scalar) * right_anti_dual.group1, 
            /* e235, e315, e125 */
            float3(this.scalar) * right_anti_dual.group2.xyz
        );
        return Scalar(
            /* scalar */
            -(wedge.e423 * other.e15) - (wedge.e431 * other.e25) - (wedge.e412 * other.e35) - (wedge.e415 * other.e23) - (wedge.e425 * other.e31) - (wedge.e435 * other.e12) - (wedge.e321 * other.e45) - (wedge.e235 * other.e41) - (wedge.e315 * other.e42) - (wedge.e125 * other.e43)
        );
    }
}
public extension Scalar {
    public AntiCircleRotor project_orthogonally_onto(DipoleInversion other) {
        AntiDipoleInversion right_anti_dual = AntiDipoleInversion(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e4 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group3 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        AntiDipoleInversion wedge = AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(this.scalar) * right_anti_dual.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(this.scalar) * right_anti_dual.group1, 
            /* e235, e315, e125, e4 */
            float4(this.scalar) * right_anti_dual.group2, 
            /* e1, e2, e3, e5 */
            float4(this.scalar) * right_anti_dual.group3
        );
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (float3(other.e1234) * wedge.group1.xyz) + (wedge.group0.xyz.zxy * other.group3.yzx) - (wedge.group0.xyz.yzx * other.group3.zxy), 
            /* e23, e31, e12, e45 */
            float4((wedge.e423 * other.e3215) + (wedge.e235 * other.e1234), (wedge.e431 * other.e3215) + (wedge.e315 * other.e1234), (wedge.e412 * other.e3215) + (wedge.e125 * other.e1234), -(wedge.e425 * other.e4315) - (wedge.e435 * other.e4125)) - (wedge.group1.wwwx * other.group3.xyzx), 
            /* e15, e25, e35, scalar */
            (float4(other.e3215) * float4(wedge.group1.xyz, wedge.e4)) + (other.group3.zxyx * float4(wedge.group2.yzx, wedge.e1)) + float4(float3(0.0), (wedge.e2 * other.e4315) + (wedge.e3 * other.e4125) + (wedge.e5 * other.e1234) - (wedge.e431 * other.e25) - (wedge.e412 * other.e35) - (wedge.e415 * other.e23) - (wedge.e425 * other.e31) - (wedge.e435 * other.e12) - (wedge.e321 * other.e45) - (wedge.e235 * other.e41) - (wedge.e315 * other.e42) - (wedge.e125 * other.e43)) - float4((wedge.group2.zxy * other.group3.yzx), wedge.e423 * other.e15)
        );
    }
}
public extension Scalar {
    public AntiDualNum project_orthogonally_onto(DualNum other) {
        AntiDualNum right_anti_dual = AntiDualNum(
            /* e3215, scalar */
            other.group0.xy * float2(-1.0)
        );
        return AntiDualNum(
            /* e3215, scalar */
            float2(other.e12345) * float2(right_anti_dual.e3215 * this.scalar, right_anti_dual.scalar * this.scalar)
        );
    }
}
public extension Scalar {
    public Scalar project_orthogonally_onto(FlatPoint other) {
        return Scalar(
            /* scalar */
            pow(other.e45, 2) * this.scalar * -1.0
        );
    }
}
public extension Scalar {
    public AntiMotor project_orthogonally_onto(Flector other) {
        AntiFlector right_anti_dual = AntiFlector(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        AntiFlector wedge = AntiFlector(
            /* e235, e315, e125, e321 */
            float4(this.scalar) * right_anti_dual.group0, 
            /* e1, e2, e3, e5 */
            float4(this.scalar) * right_anti_dual.group1
        );
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((wedge.group0.www * other.group1.xyz * float3(-1.0)), (wedge.e1 * other.e4235) + (wedge.e2 * other.e4315) + (wedge.e3 * other.e4125) - (wedge.e321 * other.e45)), 
            /* e15, e25, e35, e3215 */
            float4(((wedge.group0.yzx * other.group1.zxy) - (wedge.group0.zxy * other.group1.yzx)), 0.0)
        );
    }
}
public extension Scalar {
    public Scalar project_orthogonally_onto(Line other) {
        AntiLine right_anti_dual = AntiLine(
            /* e23, e31, e12 */
            other.group0.xyz, 
            /* e15, e25, e35 */
            other.group1.xyz
        );
        AntiLine wedge = AntiLine(
            /* e23, e31, e12 */
            float3(this.scalar) * right_anti_dual.group0.xyz, 
            /* e15, e25, e35 */
            float3(this.scalar) * right_anti_dual.group1.xyz
        );
        return Scalar(
            /* scalar */
            -(wedge.e23 * other.e415) - (wedge.e31 * other.e425) - (wedge.e12 * other.e435)
        );
    }
}
public extension Scalar {
    public AntiMotor project_orthogonally_onto(Motor other) {
        AntiMotor right_anti_dual = AntiMotor(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        AntiMotor wedge = AntiMotor(
            /* e23, e31, e12, scalar */
            float4(this.scalar) * right_anti_dual.group0, 
            /* e15, e25, e35, e3215 */
            float4(this.scalar) * right_anti_dual.group1
        );
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((float3(other.e12345) * wedge.group0.xyz), (wedge.scalar * other.e12345) - (wedge.e23 * other.e415) - (wedge.e31 * other.e425) - (wedge.e12 * other.e435)), 
            /* e15, e25, e35, e3215 */
            float4(((float3(wedge.e3215) * other.group0.xyz) + (float3(other.e12345) * wedge.group1.xyz)), wedge.e3215 * other.e12345)
        );
    }
}
public extension Scalar {
    public MultiVector project_orthogonally_onto(MultiVector other) {
        MultiVector right_anti_dual = MultiVector(
            /* scalar, e12345 */
            other.group0.xy.yx * float2(-1.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4(other.group9.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5 */
            other.e3215, 
            /* e15, e25, e35, e45 */
            float4(other.group8.xyz, other.e321 * -1.0), 
            /* e41, e42, e43 */
            other.group7.xyz, 
            /* e23, e31, e12 */
            other.group6.xyz, 
            /* e415, e425, e435, e321 */
            float4(other.group5.xyz, other.e45) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e423, e431, e412 */
            other.group4.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            other.group3.xyz * float3(-1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.group1.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234 */
            other.e4 * -1.0
        );
        MultiVector wedge = MultiVector(
            /* scalar, e12345 */
            float2(this.scalar) * right_anti_dual.group0.xy, 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * right_anti_dual.group1, 
            /* e5 */
            right_anti_dual.e5 * this.scalar, 
            /* e15, e25, e35, e45 */
            float4(this.scalar) * right_anti_dual.group3, 
            /* e41, e42, e43 */
            float3(this.scalar) * right_anti_dual.group4.xyz, 
            /* e23, e31, e12 */
            float3(this.scalar) * right_anti_dual.group5.xyz, 
            /* e415, e425, e435, e321 */
            float4(this.scalar) * right_anti_dual.group6, 
            /* e423, e431, e412 */
            float3(this.scalar) * right_anti_dual.group7.xyz, 
            /* e235, e315, e125 */
            float3(this.scalar) * right_anti_dual.group8.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar) * right_anti_dual.group9, 
            /* e1234 */
            right_anti_dual.e1234 * this.scalar
        );
        return MultiVector(
            /* scalar, e12345 */
            float2((other.scalar * wedge.e12345) + (other.e12345 * wedge.scalar) + (other.e1 * wedge.e4235) + (other.e2 * wedge.e4315) + (other.e3 * wedge.e4125) + (other.e4 * wedge.e3215) + (other.e5 * wedge.e1234) + (other.e4235 * wedge.e1) + (other.e4315 * wedge.e2) + (other.e4125 * wedge.e3) + (other.e3215 * wedge.e4) + (other.e1234 * wedge.e5) - (other.e15 * wedge.e423) - (other.e25 * wedge.e431) - (other.e35 * wedge.e412) - (other.e45 * wedge.e321) - (other.e41 * wedge.e235) - (other.e42 * wedge.e315) - (other.e43 * wedge.e125) - (other.e23 * wedge.e415) - (other.e31 * wedge.e425) - (other.e12 * wedge.e435) - (other.e415 * wedge.e23) - (other.e425 * wedge.e31) - (other.e435 * wedge.e12) - (other.e321 * wedge.e45) - (other.e423 * wedge.e15) - (other.e431 * wedge.e25) - (other.e412 * wedge.e35) - (other.e235 * wedge.e41) - (other.e315 * wedge.e42) - (other.e125 * wedge.e43), other.e12345 * wedge.e12345), 
            /* e1, e2, e3, e4 */
            float4((other.e415 * wedge.e321) + (other.e321 * wedge.e415) + (other.e4315 * wedge.e12) + (other.e1234 * wedge.e15), (other.e425 * wedge.e321) + (other.e321 * wedge.e425) + (other.e4125 * wedge.e23) + (other.e1234 * wedge.e25), (other.e435 * wedge.e321) + (other.e321 * wedge.e435) + (other.e4235 * wedge.e31) + (other.e1234 * wedge.e35), -(other.e45 * wedge.e1234) - (other.e415 * wedge.e423) - (other.e425 * wedge.e431) - (other.e435 * wedge.e412)) + (float4(other.e12345) * wedge.group1) + (float4(wedge.e12345) * other.group1) + float4((other.group4.xyz * wedge.group9.www), other.e4235 * wedge.e41) + float4((other.group5.xyz.yzx * wedge.group9.zxy), other.e4315 * wedge.e42) + float4((other.group7.xyz.zxy * wedge.group8.xyz.yzx), other.e4125 * wedge.e43) + float4((other.group8.xyz.yzx * wedge.group7.xyz.zxy), other.e1234 * wedge.e45) - (wedge.group9.yzxx * float4(other.group5.xyz.zxy, other.e41)) - float4((float3(wedge.e1234) * other.group3.xyz), other.e412 * wedge.e435) - float4((wedge.group4.xyz * other.group9.www), other.e423 * wedge.e415) - float4((other.group7.xyz.yzx * wedge.group8.xyz.zxy), other.e42 * wedge.e4315) - float4((other.group8.xyz.zxy * wedge.group7.xyz.yzx), other.e43 * wedge.e4125) - float4((wedge.group5.xyz.yzx * other.group9.zxy), other.e431 * wedge.e425), 
            /* e5 */
            (other.e12345 * wedge.e5) + (other.e5 * wedge.e12345) + (other.e15 * wedge.e4235) + (other.e25 * wedge.e4315) + (other.e35 * wedge.e4125) + (other.e45 * wedge.e3215) - (other.e415 * wedge.e235) - (other.e425 * wedge.e315) - (other.e435 * wedge.e125) - (other.e235 * wedge.e415) - (other.e315 * wedge.e425) - (other.e125 * wedge.e435) - (other.e4235 * wedge.e15) - (other.e4315 * wedge.e25) - (other.e4125 * wedge.e35) - (other.e3215 * wedge.e45), 
            /* e15, e25, e35, e45 */
            float4((other.e415 * wedge.e3215) + (other.e315 * wedge.e4125) + (other.e4125 * wedge.e315) + (other.e3215 * wedge.e415), (other.e425 * wedge.e3215) + (other.e125 * wedge.e4235) + (other.e4235 * wedge.e125) + (other.e3215 * wedge.e425), (other.e435 * wedge.e3215) + (other.e235 * wedge.e4315) + (other.e4315 * wedge.e235) + (other.e3215 * wedge.e435), -(other.e435 * wedge.e4125) - (other.e4235 * wedge.e415) - (other.e4315 * wedge.e425) - (other.e4125 * wedge.e435)) + (float4(other.e12345) * wedge.group3) + (float4(wedge.e12345) * other.group3) - (wedge.group9.yzxx * float4(other.group8.xyz.zxy, other.e415)) - float4((wedge.group8.xyz.zxy * other.group9.yzx), other.e425 * wedge.e4315), 
            /* e41, e42, e43 */
            (float3(other.e12345) * wedge.group4.xyz) + (float3(other.e1234) * wedge.group6.xyz) + (float3(wedge.e12345) * other.group4.xyz) + (float3(wedge.e1234) * other.group6.xyz) + (other.group7.xyz.zxy * wedge.group9.yzx) + (wedge.group7.xyz.zxy * other.group9.yzx) - (other.group7.xyz.yzx * wedge.group9.zxy) - (wedge.group7.xyz.yzx * other.group9.zxy), 
            /* e23, e31, e12 */
            (float3(other.e12345) * wedge.group5.xyz) + (float3(other.e3215) * wedge.group7.xyz) + (float3(other.e1234) * wedge.group8.xyz) + (float3(wedge.e12345) * other.group5.xyz) + (float3(wedge.e3215) * other.group7.xyz) + (float3(wedge.e1234) * other.group8.xyz) - (float3(other.e321) * wedge.group9.xyz) - (float3(wedge.e321) * other.group9.xyz), 
            /* e415, e425, e435, e321 */
            (float4(other.e12345) * wedge.group6) + (float4(wedge.e12345) * other.group6) + float4((other.group9.zxy * wedge.group9.yzx), other.e1234 * wedge.e3215) - (other.group9.yzxw * float4(wedge.group9.zxy, wedge.e1234)), 
            /* e423, e431, e412 */
            (float3(other.e12345) * wedge.group7.xyz) + (float3(other.e1234) * wedge.group9.xyz) + (float3(wedge.e12345) * other.group7.xyz) - (float3(wedge.e1234) * other.group9.xyz), 
            /* e235, e315, e125 */
            (float3(other.e12345) * wedge.group8.xyz) + (float3(wedge.e12345) * other.group8.xyz) + (float3(wedge.e3215) * other.group9.xyz) - (float3(other.e3215) * wedge.group9.xyz), 
            /* e4235, e4315, e4125, e3215 */
            (float4(other.e12345) * wedge.group9) + (float4(wedge.e12345) * other.group9), 
            /* e1234 */
            (other.e12345 * wedge.e1234) + (other.e1234 * wedge.e12345)
        );
    }
}
public extension Scalar {
    public Scalar project_orthogonally_onto(Plane other) {
        AntiPlane wedge = AntiPlane(
            /* e1, e2, e3, e5 */
            float4(this.scalar) * float4(other.e4235 * -1.0, other.e4315 * -1.0, other.e4125 * -1.0, other.e3215)
        );
        return Scalar(
            /* scalar */
            (wedge.e1 * other.e4235) + (wedge.e2 * other.e4315) + (wedge.e3 * other.e4125)
        );
    }
}
public extension Scalar {
    public Scalar project_orthogonally_onto(RoundPoint other) {
        Sphere right_anti_dual = Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234 */
            other.e4 * -1.0
        );
        Sphere wedge = Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar) * right_anti_dual.group0, 
            /* e1234 */
            this.scalar * right_anti_dual.e1234
        );
        return Scalar(
            /* scalar */
            (other.e1 * wedge.e4235) + (other.e2 * wedge.e4315) + (other.e3 * wedge.e4125) + (other.e4 * wedge.e3215) + (other.e5 * wedge.e1234)
        );
    }
}
public extension Scalar {
    public Scalar project_orthogonally_onto(Scalar other) {
        return Scalar(
            /* scalar */
            pow(other.scalar, 2) * this.scalar
        );
    }
}
public extension Scalar {
    public Scalar project_orthogonally_onto(Sphere other) {
        RoundPoint right_anti_dual = RoundPoint(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5 */
            other.e3215
        );
        RoundPoint wedge = RoundPoint(
            /* e1, e2, e3, e4 */
            float4(this.scalar) * right_anti_dual.group0, 
            /* e5 */
            right_anti_dual.e5 * this.scalar
        );
        return Scalar(
            /* scalar */
            (wedge.e1 * other.e4235) + (wedge.e2 * other.e4315) + (wedge.e3 * other.e4125) + (wedge.e4 * other.e3215) + (wedge.e5 * other.e1234)
        );
    }
}
public extension Scalar {
    public VersorOdd project_orthogonally_onto(VersorEven other) {
        VersorOdd right_anti_dual = VersorOdd(
            /* e41, e42, e43, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e1234 */
            float4(other.group2.xyz, other.e4) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.group3.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0)
        );
        VersorOdd wedge = VersorOdd(
            /* e41, e42, e43, scalar */
            float4(this.scalar) * right_anti_dual.group0, 
            /* e23, e31, e12, e45 */
            float4(this.scalar) * right_anti_dual.group1, 
            /* e15, e25, e35, e1234 */
            float4(this.scalar) * right_anti_dual.group2, 
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar) * right_anti_dual.group3
        );
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (other.group0.zxyw * float4(wedge.group3.yzx, wedge.scalar)) + float4(float3(0.0), (other.e2 * wedge.e4315) + (other.e3 * wedge.e4125) + (other.e4 * wedge.e3215) - (other.e431 * wedge.e25) - (other.e412 * wedge.e35) - (other.e415 * wedge.e23) - (other.e425 * wedge.e31) - (other.e435 * wedge.e12) - (other.e321 * wedge.e45) - (other.e235 * wedge.e41) - (other.e315 * wedge.e42) - (other.e125 * wedge.e43)) + float4((other.group0.www * wedge.group0.xyz), other.e5 * wedge.e1234) + float4((wedge.group2.www * other.group1.xyz), other.e1 * wedge.e4235) - (other.group0.yzxx * float4(wedge.group3.zxy, wedge.e15)), 
            /* e23, e31, e12, e45 */
            float4((other.e12345 * wedge.e23) + (other.e235 * wedge.e1234), (other.e12345 * wedge.e31) + (other.e315 * wedge.e1234), (other.e12345 * wedge.e12) + (other.e125 * wedge.e1234), -(other.e425 * wedge.e4315) - (other.e435 * wedge.e4125)) + (other.group0 * float4(wedge.group3.www, wedge.e45)) - (other.group1.wwwx * wedge.group3.xyzx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(other.e12345) * wedge.group2.xyz) + (float3(wedge.e3215) * other.group1.xyz) + (other.group2.yzx * wedge.group3.zxy) - (other.group2.zxy * wedge.group3.yzx)), other.e12345 * wedge.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345) * wedge.group3
        );
    }
}
public extension Scalar {
    public VersorOdd project_orthogonally_onto(VersorOdd other) {
        VersorEven right_anti_dual = VersorEven(
            /* e423, e431, e412, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            float4(other.group2.xyz, other.e3215) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4(other.group3.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0)
        );
        VersorEven wedge = VersorEven(
            /* e423, e431, e412, e12345 */
            float4(this.scalar) * right_anti_dual.group0, 
            /* e415, e425, e435, e321 */
            float4(this.scalar) * right_anti_dual.group1, 
            /* e235, e315, e125, e5 */
            float4(this.scalar) * right_anti_dual.group2, 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * right_anti_dual.group3
        );
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (wedge.group0.zxyw * float4(other.group3.yzx, other.scalar)) + float4(float3(0.0), (wedge.e2 * other.e4315) + (wedge.e3 * other.e4125) + (wedge.e4 * other.e3215) - (wedge.e431 * other.e25) - (wedge.e412 * other.e35) - (wedge.e415 * other.e23) - (wedge.e425 * other.e31) - (wedge.e435 * other.e12) - (wedge.e321 * other.e45) - (wedge.e235 * other.e41) - (wedge.e315 * other.e42) - (wedge.e125 * other.e43)) + float4((wedge.group0.www * other.group0.xyz), wedge.e5 * other.e1234) + float4((other.group2.www * wedge.group1.xyz), wedge.e1 * other.e4235) - (wedge.group0.yzxx * float4(other.group3.zxy, other.e15)), 
            /* e23, e31, e12, e45 */
            float4((wedge.e12345 * other.e23) + (wedge.e235 * other.e1234), (wedge.e12345 * other.e31) + (wedge.e315 * other.e1234), (wedge.e12345 * other.e12) + (wedge.e125 * other.e1234), -(wedge.e425 * other.e4315) - (wedge.e435 * other.e4125)) + (wedge.group0 * float4(other.group3.www, other.e45)) - (wedge.group1.wwwx * other.group3.xyzx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(wedge.e12345) * other.group2.xyz) + (float3(other.e3215) * wedge.group1.xyz) + (wedge.group2.yzx * other.group3.zxy) - (wedge.group2.zxy * other.group3.yzx)), wedge.e12345 * other.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(wedge.e12345) * other.group3
        );
    }
}
public extension Sphere {
    // Fancy infix trick (first half)
    public ProjectOrthogonallyOntoInfixPartial<Sphere> operator /(ProjectOrthogonallyOntoInfix rhs) {
        return ProjectOrthogonallyOntoInfixPartial<Sphere>(this);
    }
}
public extension ProjectOrthogonallyOntoInfixPartial<Sphere> {
    // Fancy infix trick (second half)
    public AntiDualNum operator /(AntiDualNum rhs) {
        return this.a.project_orthogonally_onto(rhs);
    }
}
public extension Sphere {
    public AntiDualNum project_orthogonally_onto(AntiDualNum other) {
        return AntiDualNum(
            /* e3215, scalar */
            float2(other.e3215 * this.e1234) * other.group0.xy
        );
    }
}
public extension Sphere {
    public AntiMotor project_orthogonally_onto(AntiMotor other) {
        AntiScalar wedge = AntiScalar(
            /* e12345 */
            (other.group1 * float4(-1.0, -1.0, -1.0, 1.0))[3] * this.e1234
        );
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(wedge.e12345) * other.group0, 
            /* e15, e25, e35, e3215 */
            float4(wedge.e12345) * other.group1
        );
    }
}
public extension Sphere {
    public Sphere project_orthogonally_onto(AntiScalar other) {
        Scalar right_anti_dual = Scalar(
            /* scalar */
            other.e12345 * -1.0
        );
        Sphere wedge = Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual.scalar) * this.group0, 
            /* e1234 */
            right_anti_dual.scalar * this.e1234
        );
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345) * wedge.group0, 
            /* e1234 */
            other.e12345 * wedge.e1234
        );
    }
}
public extension Sphere {
    public DipoleInversion project_orthogonally_onto(CircleRotor other) {
        AntiCircleRotor right_anti_dual = AntiCircleRotor(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        Sphere wedge = Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual.scalar) * this.group0, 
            /* e1234 */
            right_anti_dual.scalar * this.e1234
        );
        return DipoleInversion(
            /* e41, e42, e43 */
            (float3(wedge.e1234) * other.group1.xyz) + (other.group0.xyz.zxy * wedge.group0.yzx) - (other.group0.xyz.yzx * wedge.group0.zxy), 
            /* e23, e31, e12, e45 */
            float4((other.e423 * wedge.e3215) + (other.e235 * wedge.e1234), (other.e431 * wedge.e3215) + (other.e315 * wedge.e1234), (other.e412 * wedge.e3215) + (other.e125 * wedge.e1234), -(other.e425 * wedge.e4315) - (other.e435 * wedge.e4125)) - (other.group1.wwwx * wedge.group0.xyzx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(wedge.e3215) * other.group1.xyz) + (other.group2.yzx * wedge.group0.zxy) - (other.group2.zxy * wedge.group0.yzx)), other.e12345 * wedge.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345) * wedge.group0
        );
    }
}
public extension Sphere {
    public DipoleInversion project_orthogonally_onto(DipoleInversion other) {
        AntiDipoleInversion right_anti_dual = AntiDipoleInversion(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e4 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group3 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        AntiScalar wedge = AntiScalar(
            /* e12345 */
            (right_anti_dual.e4 * this.e3215) + (right_anti_dual.e1 * this.e4235) + (right_anti_dual.e2 * this.e4315) + (right_anti_dual.e3 * this.e4125) + (right_anti_dual.e5 * this.e1234)
        );
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(wedge.e12345) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(wedge.e12345) * other.group1, 
            /* e15, e25, e35, e1234 */
            float4(wedge.e12345) * other.group2, 
            /* e4235, e4315, e4125, e3215 */
            float4(wedge.e12345) * other.group3
        );
    }
}
public extension Sphere {
    public VersorOdd project_orthogonally_onto(DualNum other) {
        AntiDualNum right_anti_dual = AntiDualNum(
            /* e3215, scalar */
            other.group0.xy * float2(-1.0)
        );
        Sphere wedge = Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual.scalar) * this.group0, 
            /* e1234 */
            right_anti_dual.scalar * this.e1234
        );
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(float3(0.0), other.e5 * wedge.e1234), 
            /* e23, e31, e12, e45 */
            float4(0.0), 
            /* e15, e25, e35, e1234 */
            float4(float3(0.0), other.e12345 * wedge.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345) * wedge.group0
        );
    }
}
public extension Sphere {
    public Flector project_orthogonally_onto(Flector other) {
        AntiFlector right_anti_dual = AntiFlector(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        AntiScalar wedge = AntiScalar(
            /* e12345 */
            (right_anti_dual.e1 * this.e4235) + (right_anti_dual.e2 * this.e4315) + (right_anti_dual.e3 * this.e4125) + (right_anti_dual.e5 * this.e1234)
        );
        return Flector(
            /* e15, e25, e35, e45 */
            float4(wedge.e12345) * other.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4(wedge.e12345) * other.group1
        );
    }
}
public extension Sphere {
    public VersorOdd project_orthogonally_onto(Motor other) {
        AntiMotor right_anti_dual = AntiMotor(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        Sphere wedge = Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual.scalar) * this.group0, 
            /* e1234 */
            right_anti_dual.scalar * this.e1234
        );
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(wedge.e1234) * float4(other.group0.xyz, other.e5), 
            /* e23, e31, e12, e45 */
            float4((float3(wedge.e1234) * other.group1.xyz), -(other.e415 * wedge.e4235) - (other.e425 * wedge.e4315) - (other.e435 * wedge.e4125)), 
            /* e15, e25, e35, e1234 */
            float4(((float3(wedge.e3215) * other.group0.xyz) + (other.group1.yzx * wedge.group0.zxy) - (other.group1.zxy * wedge.group0.yzx)), other.e12345 * wedge.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345) * wedge.group0
        );
    }
}
public extension Sphere {
    public MultiVector project_orthogonally_onto(MultiVector other) {
        MultiVector right_anti_dual = MultiVector(
            /* scalar, e12345 */
            other.group0.xy.yx * float2(-1.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4(other.group9.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5 */
            other.e3215, 
            /* e15, e25, e35, e45 */
            float4(other.group8.xyz, other.e321 * -1.0), 
            /* e41, e42, e43 */
            other.group7.xyz, 
            /* e23, e31, e12 */
            other.group6.xyz, 
            /* e415, e425, e435, e321 */
            float4(other.group5.xyz, other.e45) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e423, e431, e412 */
            other.group4.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            other.group3.xyz * float3(-1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.group1.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234 */
            other.e4 * -1.0
        );
        MultiVector wedge = MultiVector(
            /* scalar, e12345 */
            float2(0.0, (right_anti_dual.e1 * this.e4235) + (right_anti_dual.e2 * this.e4315) + (right_anti_dual.e3 * this.e4125) + (right_anti_dual.e4 * this.e3215) + (right_anti_dual.e5 * this.e1234)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
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
            float4(right_anti_dual.scalar) * this.group0, 
            /* e1234 */
            right_anti_dual.scalar * this.e1234
        );
        return MultiVector(
            /* scalar, e12345 */
            float2((other.scalar * wedge.e12345) + (other.e12345 * wedge.scalar) + (other.e1 * wedge.e4235) + (other.e2 * wedge.e4315) + (other.e3 * wedge.e4125) + (other.e4 * wedge.e3215) + (other.e5 * wedge.e1234) + (other.e4235 * wedge.e1) + (other.e4315 * wedge.e2) + (other.e4125 * wedge.e3) + (other.e3215 * wedge.e4) + (other.e1234 * wedge.e5) - (other.e15 * wedge.e423) - (other.e25 * wedge.e431) - (other.e35 * wedge.e412) - (other.e45 * wedge.e321) - (other.e41 * wedge.e235) - (other.e42 * wedge.e315) - (other.e43 * wedge.e125) - (other.e23 * wedge.e415) - (other.e31 * wedge.e425) - (other.e12 * wedge.e435) - (other.e415 * wedge.e23) - (other.e425 * wedge.e31) - (other.e435 * wedge.e12) - (other.e321 * wedge.e45) - (other.e423 * wedge.e15) - (other.e431 * wedge.e25) - (other.e412 * wedge.e35) - (other.e235 * wedge.e41) - (other.e315 * wedge.e42) - (other.e125 * wedge.e43), other.e12345 * wedge.e12345), 
            /* e1, e2, e3, e4 */
            float4((other.e415 * wedge.e321) + (other.e321 * wedge.e415) + (other.e4315 * wedge.e12) + (other.e1234 * wedge.e15), (other.e425 * wedge.e321) + (other.e321 * wedge.e425) + (other.e4125 * wedge.e23) + (other.e1234 * wedge.e25), (other.e435 * wedge.e321) + (other.e321 * wedge.e435) + (other.e4235 * wedge.e31) + (other.e1234 * wedge.e35), -(other.e45 * wedge.e1234) - (other.e415 * wedge.e423) - (other.e425 * wedge.e431) - (other.e435 * wedge.e412)) + (float4(other.e12345) * wedge.group1) + (float4(wedge.e12345) * other.group1) + float4((other.group4.xyz * wedge.group9.www), other.e4235 * wedge.e41) + float4((other.group5.xyz.yzx * wedge.group9.zxy), other.e4315 * wedge.e42) + float4((other.group7.xyz.zxy * wedge.group8.xyz.yzx), other.e4125 * wedge.e43) + float4((other.group8.xyz.yzx * wedge.group7.xyz.zxy), other.e1234 * wedge.e45) - (wedge.group9.yzxx * float4(other.group5.xyz.zxy, other.e41)) - float4((float3(wedge.e1234) * other.group3.xyz), other.e412 * wedge.e435) - float4((wedge.group4.xyz * other.group9.www), other.e423 * wedge.e415) - float4((other.group7.xyz.yzx * wedge.group8.xyz.zxy), other.e42 * wedge.e4315) - float4((other.group8.xyz.zxy * wedge.group7.xyz.yzx), other.e43 * wedge.e4125) - float4((wedge.group5.xyz.yzx * other.group9.zxy), other.e431 * wedge.e425), 
            /* e5 */
            (other.e12345 * wedge.e5) + (other.e5 * wedge.e12345) + (other.e15 * wedge.e4235) + (other.e25 * wedge.e4315) + (other.e35 * wedge.e4125) + (other.e45 * wedge.e3215) - (other.e415 * wedge.e235) - (other.e425 * wedge.e315) - (other.e435 * wedge.e125) - (other.e235 * wedge.e415) - (other.e315 * wedge.e425) - (other.e125 * wedge.e435) - (other.e4235 * wedge.e15) - (other.e4315 * wedge.e25) - (other.e4125 * wedge.e35) - (other.e3215 * wedge.e45), 
            /* e15, e25, e35, e45 */
            float4((other.e415 * wedge.e3215) + (other.e315 * wedge.e4125) + (other.e4125 * wedge.e315) + (other.e3215 * wedge.e415), (other.e425 * wedge.e3215) + (other.e125 * wedge.e4235) + (other.e4235 * wedge.e125) + (other.e3215 * wedge.e425), (other.e435 * wedge.e3215) + (other.e235 * wedge.e4315) + (other.e4315 * wedge.e235) + (other.e3215 * wedge.e435), -(other.e435 * wedge.e4125) - (other.e4235 * wedge.e415) - (other.e4315 * wedge.e425) - (other.e4125 * wedge.e435)) + (float4(other.e12345) * wedge.group3) + (float4(wedge.e12345) * other.group3) - (wedge.group9.yzxx * float4(other.group8.xyz.zxy, other.e415)) - float4((wedge.group8.xyz.zxy * other.group9.yzx), other.e425 * wedge.e4315), 
            /* e41, e42, e43 */
            (float3(other.e12345) * wedge.group4.xyz) + (float3(other.e1234) * wedge.group6.xyz) + (float3(wedge.e12345) * other.group4.xyz) + (float3(wedge.e1234) * other.group6.xyz) + (other.group7.xyz.zxy * wedge.group9.yzx) + (wedge.group7.xyz.zxy * other.group9.yzx) - (other.group7.xyz.yzx * wedge.group9.zxy) - (wedge.group7.xyz.yzx * other.group9.zxy), 
            /* e23, e31, e12 */
            (float3(other.e12345) * wedge.group5.xyz) + (float3(other.e3215) * wedge.group7.xyz) + (float3(other.e1234) * wedge.group8.xyz) + (float3(wedge.e12345) * other.group5.xyz) + (float3(wedge.e3215) * other.group7.xyz) + (float3(wedge.e1234) * other.group8.xyz) - (float3(other.e321) * wedge.group9.xyz) - (float3(wedge.e321) * other.group9.xyz), 
            /* e415, e425, e435, e321 */
            (float4(other.e12345) * wedge.group6) + (float4(wedge.e12345) * other.group6) + float4((other.group9.zxy * wedge.group9.yzx), other.e1234 * wedge.e3215) - (other.group9.yzxw * float4(wedge.group9.zxy, wedge.e1234)), 
            /* e423, e431, e412 */
            (float3(other.e12345) * wedge.group7.xyz) + (float3(other.e1234) * wedge.group9.xyz) + (float3(wedge.e12345) * other.group7.xyz) - (float3(wedge.e1234) * other.group9.xyz), 
            /* e235, e315, e125 */
            (float3(other.e12345) * wedge.group8.xyz) + (float3(wedge.e12345) * other.group8.xyz) + (float3(wedge.e3215) * other.group9.xyz) - (float3(other.e3215) * wedge.group9.xyz), 
            /* e4235, e4315, e4125, e3215 */
            (float4(other.e12345) * wedge.group9) + (float4(wedge.e12345) * other.group9), 
            /* e1234 */
            (other.e12345 * wedge.e1234) + (other.e1234 * wedge.e12345)
        );
    }
}
public extension Sphere {
    public Plane project_orthogonally_onto(Plane other) {
        AntiPlane right_anti_dual = AntiPlane(
            /* e1, e2, e3, e5 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual.e1 * this.e4235) + (right_anti_dual.e2 * this.e4315) + (right_anti_dual.e3 * this.e4125) + (right_anti_dual.e5 * this.e1234)) * other.group0
        );
    }
}
public extension Sphere {
    public Sphere project_orthogonally_onto(Sphere other) {
        RoundPoint right_anti_dual = RoundPoint(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5 */
            other.e3215
        );
        AntiScalar wedge = AntiScalar(
            /* e12345 */
            (right_anti_dual.e1 * this.e4235) + (right_anti_dual.e2 * this.e4315) + (right_anti_dual.e3 * this.e4125) + (right_anti_dual.e4 * this.e3215) + (right_anti_dual.e5 * this.e1234)
        );
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(wedge.e12345) * other.group0, 
            /* e1234 */
            wedge.e12345 * other.e1234
        );
    }
}
public extension Sphere {
    public VersorOdd project_orthogonally_onto(VersorEven other) {
        VersorOdd right_anti_dual = VersorOdd(
            /* e41, e42, e43, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e1234 */
            float4(other.group2.xyz, other.e4) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.group3.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0)
        );
        Sphere wedge = Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual.scalar) * this.group0, 
            /* e1234 */
            this.e1234 * right_anti_dual.scalar
        );
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(wedge.e4125 * other.e431 * -1.0, wedge.e4235 * other.e412 * -1.0, wedge.e4315 * other.e423 * -1.0, (wedge.e4125 * other.e3) + (wedge.e3215 * other.e4) + (wedge.e1234 * other.e5)) + (wedge.group0.yzxx * float4(other.group0.zxy, other.e1)) + float4((float3(wedge.e1234) * other.group1.xyz), wedge.e4315 * other.e2), 
            /* e23, e31, e12, e45 */
            float4((wedge.e3215 * other.e423) + (wedge.e1234 * other.e235), (wedge.e3215 * other.e431) + (wedge.e1234 * other.e315), (wedge.e3215 * other.e412) + (wedge.e1234 * other.e125), -(wedge.e4315 * other.e425) - (wedge.e4125 * other.e435)) - (wedge.group0.xyzx * other.group1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(wedge.e3215) * other.group1.xyz) + (wedge.group0.zxy * other.group2.yzx) - (wedge.group0.yzx * other.group2.zxy)), wedge.e1234 * other.e12345), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345) * wedge.group0
        );
    }
}
public extension Sphere {
    public VersorOdd project_orthogonally_onto(VersorOdd other) {
        VersorEven right_anti_dual = VersorEven(
            /* e423, e431, e412, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            float4(other.group2.xyz, other.e3215) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4(other.group3.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0)
        );
        AntiScalar wedge = AntiScalar(
            /* e12345 */
            (this.e4235 * right_anti_dual.e1) + (this.e4315 * right_anti_dual.e2) + (this.e4125 * right_anti_dual.e3) + (this.e3215 * right_anti_dual.e4) + (this.e1234 * right_anti_dual.e5)
        );
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(wedge.e12345) * other.group0, 
            /* e23, e31, e12, e45 */
            float4(wedge.e12345) * other.group1, 
            /* e15, e25, e35, e1234 */
            float4(wedge.e12345) * other.group2, 
            /* e4235, e4315, e4125, e3215 */
            float4(wedge.e12345) * other.group3
        );
    }
}
public extension VersorEven {
    // Fancy infix trick (first half)
    public ProjectOrthogonallyOntoInfixPartial<VersorEven> operator /(ProjectOrthogonallyOntoInfix rhs) {
        return ProjectOrthogonallyOntoInfixPartial<VersorEven>(this);
    }
}
public extension ProjectOrthogonallyOntoInfixPartial<VersorEven> {
    // Fancy infix trick (second half)
    public RoundPoint operator /(AntiCircleRotor rhs) {
        return this.a.project_orthogonally_onto(rhs);
    }
}
public extension VersorEven {
    public RoundPoint project_orthogonally_onto(AntiCircleRotor other) {
        CircleRotor right_anti_dual = CircleRotor(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        Sphere wedge = Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual.e425 * this.e3) + (right_anti_dual.e235 * this.e4), (right_anti_dual.e435 * this.e1) + (right_anti_dual.e315 * this.e4), (right_anti_dual.e415 * this.e2) + (right_anti_dual.e125 * this.e4), -(right_anti_dual.e315 * this.e2) - (right_anti_dual.e125 * this.e3)) - (float4(this.e5) * float4(right_anti_dual.group0.xyz, right_anti_dual.e321)) - (this.group3.yzxx * float4(right_anti_dual.group1.zxy, right_anti_dual.e235)), 
            /* e1234 */
            (right_anti_dual.e423 * this.e1) + (right_anti_dual.e431 * this.e2) + (right_anti_dual.e412 * this.e3) + (right_anti_dual.e321 * this.e4)
        );
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4((other.e41 * wedge.e3215) + (other.e31 * wedge.e4125), (other.e42 * wedge.e3215) + (other.e12 * wedge.e4235), (other.e43 * wedge.e3215) + (other.e23 * wedge.e4315), -(other.e43 * wedge.e4125) - (other.e45 * wedge.e1234)) - (wedge.group0.yzxx * float4(other.group1.zxy, other.e41)) - float4((float3(wedge.e1234) * other.group2.xyz), other.e42 * wedge.e4315), 
            /* e5 */
            (other.e45 * wedge.e3215) + (other.e15 * wedge.e4235) + (other.e25 * wedge.e4315) + (other.e35 * wedge.e4125)
        );
    }
}
public extension VersorEven {
    public AntiDipoleInversion project_orthogonally_onto(AntiDipoleInversion other) {
        DipoleInversion right_anti_dual = DipoleInversion(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e1234 */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group3 * float4(1.0, 1.0, 1.0, -1.0)
        );
        CircleRotor wedge = CircleRotor(
            /* e423, e431, e412 */
            (float3(this.e4) * right_anti_dual.group1.xyz) + (right_anti_dual.group0.xyz.yzx * this.group3.zxy) - (right_anti_dual.group0.xyz.zxy * this.group3.yzx), 
            /* e415, e425, e435, e321 */
            float4((right_anti_dual.e41 * this.e5) + (right_anti_dual.e15 * this.e4), (right_anti_dual.e42 * this.e5) + (right_anti_dual.e25 * this.e4), (right_anti_dual.e43 * this.e5) + (right_anti_dual.e35 * this.e4), -(right_anti_dual.e31 * this.e2) - (right_anti_dual.e12 * this.e3)) - (right_anti_dual.group1.wwwx * this.group3.xyzx), 
            /* e235, e315, e125, e12345 */
            (float4(this.e5) * float4(right_anti_dual.group1.xyz, right_anti_dual.e1234)) + (this.group3.yzxx * float4(right_anti_dual.group2.zxy, right_anti_dual.e4235)) + float4(float3(0.0), (right_anti_dual.e4315 * this.e2) + (right_anti_dual.e4125 * this.e3) + (right_anti_dual.e3215 * this.e4) - (right_anti_dual.e42 * this.e315) - (right_anti_dual.e43 * this.e125) - (right_anti_dual.e23 * this.e415) - (right_anti_dual.e31 * this.e425) - (right_anti_dual.e12 * this.e435) - (right_anti_dual.e45 * this.e321) - (right_anti_dual.e15 * this.e423) - (right_anti_dual.e25 * this.e431) - (right_anti_dual.e35 * this.e412)) - float4((right_anti_dual.group2.yzx * this.group3.zxy), right_anti_dual.e41 * this.e235)
        );
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(wedge.e12345) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(wedge.e12345) * other.group1, 
            /* e235, e315, e125, e4 */
            float4((float3(wedge.e12345) * other.group2.xyz), (other.e4 * wedge.e12345) - (other.e423 * wedge.e415) - (other.e431 * wedge.e425) - (other.e412 * wedge.e435) - (other.e415 * wedge.e423) - (other.e425 * wedge.e431) - (other.e435 * wedge.e412)), 
            /* e1, e2, e3, e5 */
            float4((other.e415 * wedge.e321) + (other.e321 * wedge.e415) + (other.e315 * wedge.e412) + (other.e1 * wedge.e12345), (other.e425 * wedge.e321) + (other.e321 * wedge.e425) + (other.e125 * wedge.e423) + (other.e2 * wedge.e12345), (other.e435 * wedge.e321) + (other.e321 * wedge.e435) + (other.e235 * wedge.e431) + (other.e3 * wedge.e12345), -(other.e435 * wedge.e125) - (other.e235 * wedge.e415) - (other.e315 * wedge.e425) - (other.e125 * wedge.e435)) + (wedge.group2.yzxw * float4(other.group0.xyz.zxy, other.e5)) - (wedge.group2.zxyx * float4(other.group0.xyz.yzx, other.e415)) - float4((wedge.group0.xyz.yzx * other.group2.zxy), other.e425 * wedge.e315)
        );
    }
}
public extension VersorEven {
    public AntiFlector project_orthogonally_onto(AntiDualNum other) {
        DualNum right_anti_dual = DualNum(
            /* e5, e12345 */
            other.group0.xy
        );
        Flector wedge = Flector(
            /* e15, e25, e35, e45 */
            float4(right_anti_dual.e5) * this.group3, 
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual.e5) * float4(this.group0.xyz, this.e321)
        );
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(other.group0.xy.xx, other.e3215, 0.0) * float4(float3(1.0), 0.0) * float4(wedge.group1.xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), other.e3215 * wedge.e45 * -1.0)
        );
    }
}
public extension VersorEven {
    public AntiFlector project_orthogonally_onto(AntiFlatPoint other) {
        FlatPoint right_anti_dual = FlatPoint(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        Motor wedge = Motor(
            /* e415, e425, e435, e12345 */
            float4(right_anti_dual.e15 * this.e4, right_anti_dual.e25 * this.e4, right_anti_dual.e35 * this.e4, -(right_anti_dual.e25 * this.e431) - (right_anti_dual.e35 * this.e412) - (right_anti_dual.e45 * this.e321)) - (right_anti_dual.group0.wwwx * float4(this.group3.xyz, this.e423)), 
            /* e235, e315, e125, e5 */
            float4(((right_anti_dual.group0.zxy * this.group3.yzx) - (right_anti_dual.group0.yzx * this.group3.zxy)), 0.0)
        );
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(wedge.e12345) * other.group0, 
            /* e1, e2, e3, e5 */
            float4((other.group0.www * wedge.group0.xyz), -(other.e235 * wedge.e415) - (other.e315 * wedge.e425) - (other.e125 * wedge.e435))
        );
    }
}
public extension VersorEven {
    public AntiFlector project_orthogonally_onto(AntiFlector other) {
        Flector right_anti_dual = Flector(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        Motor wedge = Motor(
            /* e415, e425, e435, e12345 */
            (this.group3.wwwx * float4(right_anti_dual.group0.xyz, right_anti_dual.e4235)) + float4(float3(0.0), (right_anti_dual.e4315 * this.e2) + (right_anti_dual.e4125 * this.e3) + (right_anti_dual.e3215 * this.e4) - (right_anti_dual.e25 * this.e431) - (right_anti_dual.e35 * this.e412) - (right_anti_dual.e45 * this.e321)) - (right_anti_dual.group0.wwwx * float4(this.group3.xyz, this.e423)), 
            /* e235, e315, e125, e5 */
            float4(((right_anti_dual.group0.zxy * this.group3.yzx) - (right_anti_dual.group0.yzx * this.group3.zxy)), 0.0)
        );
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(wedge.e12345) * other.group0, 
            /* e1, e2, e3, e5 */
            float4(other.e1 * wedge.e12345, other.e2 * wedge.e12345, other.e3 * wedge.e12345, -(other.e235 * wedge.e415) - (other.e315 * wedge.e425) - (other.e125 * wedge.e435)) + (wedge.group0 * float4(other.group0.www, other.e5))
        );
    }
}
public extension VersorEven {
    public AntiPlane project_orthogonally_onto(AntiLine other) {
        Line right_anti_dual = Line(
            /* e415, e425, e435 */
            other.group0.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            other.group1.xyz * float3(-1.0)
        );
        Plane wedge = Plane(
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual.e425 * this.e3) + (right_anti_dual.e235 * this.e4), (right_anti_dual.e435 * this.e1) + (right_anti_dual.e315 * this.e4), (right_anti_dual.e415 * this.e2) + (right_anti_dual.e125 * this.e4), -(right_anti_dual.e315 * this.e2) - (right_anti_dual.e125 * this.e3)) - (this.group3.yzxx * float4(right_anti_dual.group0.xyz.zxy, right_anti_dual.e235))
        );
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(other.e12 * wedge.e4315 * -1.0, other.e23 * wedge.e4125 * -1.0, other.e31 * wedge.e4235 * -1.0, (other.e25 * wedge.e4315) + (other.e35 * wedge.e4125)) + (wedge.group0.zxyx * float4(other.group0.xyz.yzx, other.e15))
        );
    }
}
public extension VersorEven {
    public AntiFlector project_orthogonally_onto(AntiMotor other) {
        Motor right_anti_dual = Motor(
            /* e415, e425, e435, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        Flector wedge = Flector(
            /* e15, e25, e35, e45 */
            float4(right_anti_dual.e5) * this.group3, 
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual.e235 * this.e4) + (right_anti_dual.e5 * this.e423), (right_anti_dual.e315 * this.e4) + (right_anti_dual.e5 * this.e431), (right_anti_dual.e125 * this.e4) + (right_anti_dual.e5 * this.e412), -(right_anti_dual.e315 * this.e2) - (right_anti_dual.e125 * this.e3)) + float4((right_anti_dual.group0.yzx * this.group3.zxy), right_anti_dual.e5 * this.e321) - (this.group3.yzxx * float4(right_anti_dual.group0.zxy, right_anti_dual.e235))
        );
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(float3(1.0), 0.0) * float4(other.group1.www, 0.0) * float4(wedge.group1.xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0), 
            /* e1, e2, e3, e5 */
            (wedge.group1.zxyx * float4(other.group0.yzx, other.e15)) + float4(float3(0.0), (other.e25 * wedge.e4315) + (other.e35 * wedge.e4125)) - float4((other.group0.zxy * wedge.group1.yzx), other.e3215 * wedge.e45)
        );
    }
}
public extension VersorEven {
    public AntiPlane project_orthogonally_onto(AntiPlane other) {
        Plane right_anti_dual = Plane(
            /* e4235, e4315, e4125, e3215 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4((right_anti_dual.e4235 * this.e1) + (right_anti_dual.e4315 * this.e2) + (right_anti_dual.e4125 * this.e3) + (right_anti_dual.e3215 * this.e4)) * other.group0
        );
    }
}
public extension VersorEven {
    public VersorEven project_orthogonally_onto(AntiScalar other) {
        Scalar right_anti_dual = Scalar(
            /* scalar */
            other.e12345 * -1.0
        );
        VersorEven wedge = VersorEven(
            /* e423, e431, e412, e12345 */
            float4(right_anti_dual.scalar) * this.group0, 
            /* e415, e425, e435, e321 */
            float4(right_anti_dual.scalar) * this.group1, 
            /* e235, e315, e125, e5 */
            float4(right_anti_dual.scalar) * this.group2, 
            /* e1, e2, e3, e4 */
            float4(right_anti_dual.scalar) * this.group3
        );
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(other.e12345) * wedge.group0, 
            /* e415, e425, e435, e321 */
            float4(other.e12345) * wedge.group1, 
            /* e235, e315, e125, e5 */
            float4(other.e12345) * wedge.group2, 
            /* e1, e2, e3, e4 */
            float4(other.e12345) * wedge.group3
        );
    }
}
public extension VersorEven {
    public AntiDipoleInversion project_orthogonally_onto(Circle other) {
        Dipole right_anti_dual = Dipole(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35 */
            other.group2.xyz
        );
        CircleRotor wedge = CircleRotor(
            /* e423, e431, e412 */
            (float3(this.e4) * right_anti_dual.group1.xyz) + (right_anti_dual.group0.xyz.yzx * this.group3.zxy) - (right_anti_dual.group0.xyz.zxy * this.group3.yzx), 
            /* e415, e425, e435, e321 */
            float4((right_anti_dual.e41 * this.e5) + (right_anti_dual.e15 * this.e4), (right_anti_dual.e42 * this.e5) + (right_anti_dual.e25 * this.e4), (right_anti_dual.e43 * this.e5) + (right_anti_dual.e35 * this.e4), -(right_anti_dual.e31 * this.e2) - (right_anti_dual.e12 * this.e3)) - (right_anti_dual.group1.wwwx * this.group3.xyzx), 
            /* e235, e315, e125, e12345 */
            float4((right_anti_dual.e23 * this.e5) + (right_anti_dual.e35 * this.e2), (right_anti_dual.e31 * this.e5) + (right_anti_dual.e15 * this.e3), (right_anti_dual.e12 * this.e5) + (right_anti_dual.e25 * this.e1), -(right_anti_dual.e42 * this.e315) - (right_anti_dual.e43 * this.e125) - (right_anti_dual.e23 * this.e415) - (right_anti_dual.e31 * this.e425) - (right_anti_dual.e12 * this.e435) - (right_anti_dual.e45 * this.e321) - (right_anti_dual.e15 * this.e423) - (right_anti_dual.e25 * this.e431) - (right_anti_dual.e35 * this.e412)) - float4((right_anti_dual.group2.xyz.yzx * this.group3.zxy), right_anti_dual.e41 * this.e235)
        );
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(wedge.e12345) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(wedge.e12345) * other.group1, 
            /* e235, e315, e125, e4 */
            float4((float3(wedge.e12345) * other.group2.xyz), -(other.e423 * wedge.e415) - (other.e431 * wedge.e425) - (other.e412 * wedge.e435) - (other.e415 * wedge.e423) - (other.e425 * wedge.e431) - (other.e435 * wedge.e412)), 
            /* e1, e2, e3, e5 */
            float4((other.e412 * wedge.e315) + (other.e415 * wedge.e321) + (other.e321 * wedge.e415) + (other.e315 * wedge.e412), (other.e423 * wedge.e125) + (other.e425 * wedge.e321) + (other.e321 * wedge.e425) + (other.e125 * wedge.e423), (other.e431 * wedge.e235) + (other.e435 * wedge.e321) + (other.e321 * wedge.e435) + (other.e235 * wedge.e431), -(other.e415 * wedge.e235) - (other.e425 * wedge.e315) - (other.e435 * wedge.e125) - (other.e125 * wedge.e435)) - float4((other.group0.xyz.yzx * wedge.group2.zxy), other.e235 * wedge.e415) - float4((other.group2.xyz.zxy * wedge.group0.xyz.yzx), other.e315 * wedge.e425)
        );
    }
}
public extension VersorEven {
    public VersorEven project_orthogonally_onto(CircleRotor other) {
        AntiCircleRotor right_anti_dual = AntiCircleRotor(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        VersorEven wedge = VersorEven(
            /* e423, e431, e412, e12345 */
            float4((right_anti_dual.e23 * this.e4) + (right_anti_dual.scalar * this.e423), (right_anti_dual.e31 * this.e4) + (right_anti_dual.scalar * this.e431), (right_anti_dual.e12 * this.e4) + (right_anti_dual.scalar * this.e412), -(right_anti_dual.e42 * this.e315) - (right_anti_dual.e43 * this.e125) - (right_anti_dual.e23 * this.e415) - (right_anti_dual.e31 * this.e425) - (right_anti_dual.e12 * this.e435) - (right_anti_dual.e45 * this.e321) - (right_anti_dual.e15 * this.e423) - (right_anti_dual.e25 * this.e431) - (right_anti_dual.e35 * this.e412)) + float4((right_anti_dual.group0.xyz.yzx * this.group3.zxy), right_anti_dual.scalar * this.e12345) - float4((right_anti_dual.group0.xyz.zxy * this.group3.yzx), right_anti_dual.e41 * this.e235), 
            /* e415, e425, e435, e321 */
            float4((right_anti_dual.e15 * this.e4) + (right_anti_dual.scalar * this.e415), (right_anti_dual.e25 * this.e4) + (right_anti_dual.scalar * this.e425), (right_anti_dual.e35 * this.e4) + (right_anti_dual.scalar * this.e435), -(right_anti_dual.e31 * this.e2) - (right_anti_dual.e12 * this.e3)) + float4((right_anti_dual.group0.xyz * this.group2.www), right_anti_dual.scalar * this.e321) - (right_anti_dual.group1.wwwx * this.group3.xyzx), 
            /* e235, e315, e125, e5 */
            float4(((float3(right_anti_dual.scalar) * this.group2.xyz) + (float3(this.e5) * right_anti_dual.group1.xyz) + (right_anti_dual.group2.zxy * this.group3.yzx) - (right_anti_dual.group2.yzx * this.group3.zxy)), right_anti_dual.scalar * this.e5), 
            /* e1, e2, e3, e4 */
            float4(right_anti_dual.scalar) * this.group3
        );
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(((float3(other.e12345) * wedge.group0.xyz) + (float3(wedge.e12345) * other.group0.xyz)), other.e12345 * wedge.e12345), 
            /* e415, e425, e435, e321 */
            (float4(other.e12345) * wedge.group1) + (float4(wedge.e12345) * other.group1), 
            /* e235, e315, e125, e5 */
            float4(other.e12345 * wedge.e235, other.e12345 * wedge.e315, other.e12345 * wedge.e125, -(other.e415 * wedge.e235) - (other.e425 * wedge.e315) - (other.e435 * wedge.e125) - (other.e235 * wedge.e415) - (other.e315 * wedge.e425) - (other.e125 * wedge.e435)) + (other.group2 * float4(wedge.group0.www, wedge.e5)), 
            /* e1, e2, e3, e4 */
            float4((other.e415 * wedge.e321) + (other.e321 * wedge.e415) + (other.e315 * wedge.e412) + (other.e12345 * wedge.e1), (other.e425 * wedge.e321) + (other.e321 * wedge.e425) + (other.e125 * wedge.e423) + (other.e12345 * wedge.e2), (other.e435 * wedge.e321) + (other.e321 * wedge.e435) + (other.e235 * wedge.e431) + (other.e12345 * wedge.e3), -(other.e412 * wedge.e435) - (other.e415 * wedge.e423) - (other.e425 * wedge.e431) - (other.e435 * wedge.e412)) + float4((other.group0.xyz.zxy * wedge.group2.yzx), other.e12345 * wedge.e4) - float4((other.group0.xyz.yzx * wedge.group2.zxy), other.e423 * wedge.e415) - float4((other.group2.zxy * wedge.group0.yzx), other.e431 * wedge.e425)
        );
    }
}
public extension VersorEven {
    public RoundPoint project_orthogonally_onto(Dipole other) {
        Circle right_anti_dual = Circle(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125 */
            other.group2.xyz * float3(-1.0)
        );
        Sphere wedge = Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual.e425 * this.e3) + (right_anti_dual.e235 * this.e4), (right_anti_dual.e435 * this.e1) + (right_anti_dual.e315 * this.e4), (right_anti_dual.e415 * this.e2) + (right_anti_dual.e125 * this.e4), -(right_anti_dual.e321 * this.e5) - (right_anti_dual.e125 * this.e3)) - (this.group3.yzxy * float4(right_anti_dual.group1.zxy, right_anti_dual.e315)) - float4((right_anti_dual.group0.xyz * this.group2.www), right_anti_dual.e235 * this.e1), 
            /* e1234 */
            (right_anti_dual.e423 * this.e1) + (right_anti_dual.e431 * this.e2) + (right_anti_dual.e412 * this.e3) + (right_anti_dual.e321 * this.e4)
        );
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4((other.e41 * wedge.e3215) + (other.e31 * wedge.e4125), (other.e42 * wedge.e3215) + (other.e12 * wedge.e4235), (other.e43 * wedge.e3215) + (other.e23 * wedge.e4315), -(other.e43 * wedge.e4125) - (other.e45 * wedge.e1234)) - (wedge.group0.yzxy * float4(other.group1.zxy, other.e42)) - float4((float3(wedge.e1234) * other.group2.xyz), other.e41 * wedge.e4235), 
            /* e5 */
            (other.e45 * wedge.e3215) + (other.e15 * wedge.e4235) + (other.e25 * wedge.e4315) + (other.e35 * wedge.e4125)
        );
    }
}
public extension VersorEven {
    public AntiDipoleInversion project_orthogonally_onto(DipoleInversion other) {
        AntiDipoleInversion right_anti_dual = AntiDipoleInversion(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e4 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group3 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        DipoleInversion wedge = DipoleInversion(
            /* e41, e42, e43 */
            (float3(this.e4) * right_anti_dual.group3.xyz) - (float3(right_anti_dual.e4) * this.group3.xyz), 
            /* e23, e31, e12, e45 */
            (right_anti_dual.group3.zxyw * this.group3.yzxw) - float4((right_anti_dual.group3.yzx * this.group3.zxy), right_anti_dual.e4 * this.e5), 
            /* e15, e25, e35, e1234 */
            (this.group3.xyzx * float4(right_anti_dual.group3.www, right_anti_dual.e423)) + float4(float3(0.0), (right_anti_dual.e431 * this.e2) + (right_anti_dual.e412 * this.e3) + (right_anti_dual.e321 * this.e4) - (right_anti_dual.e1 * this.e423) - (right_anti_dual.e2 * this.e431) - (right_anti_dual.e3 * this.e412)) - float4((this.group2.www * right_anti_dual.group3.xyz), right_anti_dual.e4 * this.e321), 
            /* e4235, e4315, e4125, e3215 */
            (float4(right_anti_dual.e5) * float4(this.group0.xyz, this.e321)) + (right_anti_dual.group3.yzxz * float4(this.group1.zxy, this.e125)) + float4((right_anti_dual.group1.yzx * this.group3.zxy), right_anti_dual.e1 * this.e235) + float4((this.group3.www * right_anti_dual.group2.xyz), right_anti_dual.e2 * this.e315) - (float4(this.e5) * float4(right_anti_dual.group0.xyz, right_anti_dual.e321)) - (right_anti_dual.group2.wwwy * float4(this.group2.xyz, this.e2)) - (this.group3.yzxx * float4(right_anti_dual.group1.zxy, right_anti_dual.e235)) - float4((right_anti_dual.group3.zxy * this.group1.yzx), right_anti_dual.e125 * this.e3)
        );
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (float3(other.e1234) * wedge.group3.xyz) - (float3(wedge.e1234) * other.group3.xyz), 
            /* e415, e425, e435, e321 */
            (wedge.group3.yzxw * float4(other.group3.zxy, other.e1234)) - (other.group3.yzxw * float4(wedge.group3.zxy, wedge.e1234)), 
            /* e235, e315, e125, e4 */
            float4(float3(0.0), (other.e1234 * wedge.e45) + (other.e4315 * wedge.e42) + (other.e4125 * wedge.e43) - (other.e42 * wedge.e4315) - (other.e43 * wedge.e4125) - (other.e45 * wedge.e1234)) + float4((wedge.group3.www * other.group3.xyz), other.e4235 * wedge.e41) - (wedge.group3.xyzx * float4(other.group3.www, other.e41)), 
            /* e1, e2, e3, e5 */
            (float4(wedge.e3215) * float4(other.group0.xyz, other.e45)) + (other.group2.wwwy * float4(wedge.group2.xyz, wedge.e4315)) + (wedge.group3.zxyx * float4(other.group1.yzx, other.e15)) + float4((other.group3.yzx * wedge.group1.zxy), other.e35 * wedge.e4125) - (other.group3.zxyw * wedge.group1.yzxw) - (wedge.group2.wwwz * float4(other.group2.xyz, other.e4125)) - float4((wedge.group0.xyz * other.group3.www), other.e4235 * wedge.e15) - float4((other.group1.zxy * wedge.group3.yzx), other.e4315 * wedge.e25)
        );
    }
}
public extension VersorEven {
    public VersorEven project_orthogonally_onto(DualNum other) {
        AntiDualNum right_anti_dual = AntiDualNum(
            /* e3215, scalar */
            other.group0.xy * float2(-1.0)
        );
        VersorEven wedge = VersorEven(
            /* e423, e431, e412, e12345 */
            float4(right_anti_dual.group0.xy.yy, right_anti_dual.scalar, (right_anti_dual.e3215 * this.e4) + (right_anti_dual.scalar * this.e12345)) * float4(this.group0.xyz, 1.0), 
            /* e415, e425, e435, e321 */
            float4(right_anti_dual.scalar) * this.group1, 
            /* e235, e315, e125, e5 */
            float4(right_anti_dual.scalar) * this.group2, 
            /* e1, e2, e3, e4 */
            float4(right_anti_dual.scalar) * this.group3
        );
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(other.e12345) * wedge.group0, 
            /* e415, e425, e435, e321 */
            float4(other.e12345) * wedge.group1, 
            /* e235, e315, e125, e5 */
            float4(other.group0.xy.yy, other.e12345, (other.e5 * wedge.e12345) + (other.e12345 * wedge.e5)) * float4(wedge.group2.xyz, 1.0), 
            /* e1, e2, e3, e4 */
            float4(other.e12345) * wedge.group3
        );
    }
}
public extension VersorEven {
    public RoundPoint project_orthogonally_onto(FlatPoint other) {
        AntiFlatPoint right_anti_dual = AntiFlatPoint(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        Sphere wedge = Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((float3(this.e4) * right_anti_dual.group0.xyz), -(right_anti_dual.e235 * this.e1) - (right_anti_dual.e315 * this.e2) - (right_anti_dual.e125 * this.e3) - (right_anti_dual.e321 * this.e5)), 
            /* e1234 */
            right_anti_dual.e321 * this.e4
        );
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(wedge.e1234) * other.group0 * float4(-1.0), 
            /* e5 */
            (other.e15 * wedge.e4235) + (other.e25 * wedge.e4315) + (other.e35 * wedge.e4125) + (other.e45 * wedge.e3215)
        );
    }
}
public extension VersorEven {
    public AntiDipoleInversion project_orthogonally_onto(Flector other) {
        AntiFlector right_anti_dual = AntiFlector(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        DipoleInversion wedge = DipoleInversion(
            /* e41, e42, e43 */
            float3(this.e4) * right_anti_dual.group1.xyz, 
            /* e23, e31, e12, e45 */
            float4(((right_anti_dual.group1.zxy * this.group3.yzx) - (right_anti_dual.group1.yzx * this.group3.zxy)), right_anti_dual.e5 * this.e4), 
            /* e15, e25, e35, e1234 */
            (this.group3 * float4(right_anti_dual.group1.www, right_anti_dual.e321)) + float4(float3(0.0), -(right_anti_dual.e2 * this.e431) - (right_anti_dual.e3 * this.e412)) - (right_anti_dual.group1.xyzx * float4(this.group2.www, this.e423)), 
            /* e4235, e4315, e4125, e3215 */
            (right_anti_dual.group1.yzxy * float4(this.group1.zxy, this.e315)) + (right_anti_dual.group1.wwwz * float4(this.group0.xyz, this.e125)) + float4(float3(0.0), (right_anti_dual.e5 * this.e321) - (right_anti_dual.e315 * this.e2) - (right_anti_dual.e125 * this.e3) - (right_anti_dual.e321 * this.e5)) + float4((this.group3.www * right_anti_dual.group0.xyz), right_anti_dual.e1 * this.e235) - float4((right_anti_dual.group1.zxy * this.group1.yzx), right_anti_dual.e235 * this.e1)
        );
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(wedge.e1234) * other.group1.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            float4(((wedge.group3.yzx * other.group1.zxy) - (wedge.group3.zxy * other.group1.yzx)), wedge.e1234 * other.e3215 * -1.0), 
            /* e235, e315, e125, e4 */
            (other.group1.xyzx * float4(wedge.group3.www, wedge.e41)) + float4(float3(0.0), (wedge.e42 * other.e4315) + (wedge.e43 * other.e4125)) - float4((other.group1.www * wedge.group3.xyz), wedge.e1234 * other.e45), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), (wedge.e4315 * other.e25) + (wedge.e4125 * other.e35) + (wedge.e3215 * other.e45) - (wedge.e35 * other.e4125)) + float4((wedge.group1.zxy * other.group1.yzx), wedge.e4235 * other.e15) - (float4(other.e3215) * float4(wedge.group0.xyz, wedge.e45)) - (wedge.group2.wwwy * float4(other.group0.xyz, other.e4315)) - (other.group1.zxyx * float4(wedge.group1.yzx, wedge.e15))
        );
    }
}
public extension VersorEven {
    public AntiDipoleInversion project_orthogonally_onto(Line other) {
        AntiLine right_anti_dual = AntiLine(
            /* e23, e31, e12 */
            other.group0.xyz, 
            /* e15, e25, e35 */
            other.group1.xyz
        );
        CircleRotor wedge = CircleRotor(
            /* e423, e431, e412 */
            float3(this.e4) * right_anti_dual.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4((float3(this.e4) * right_anti_dual.group1.xyz), -(right_anti_dual.e23 * this.e1) - (right_anti_dual.e31 * this.e2) - (right_anti_dual.e12 * this.e3)), 
            /* e235, e315, e125, e12345 */
            float4((right_anti_dual.e23 * this.e5) + (right_anti_dual.e35 * this.e2), (right_anti_dual.e31 * this.e5) + (right_anti_dual.e15 * this.e3), (right_anti_dual.e12 * this.e5) + (right_anti_dual.e25 * this.e1), -(right_anti_dual.e31 * this.e425) - (right_anti_dual.e12 * this.e435) - (right_anti_dual.e15 * this.e423) - (right_anti_dual.e25 * this.e431) - (right_anti_dual.e35 * this.e412)) - float4((right_anti_dual.group1.xyz.yzx * this.group3.zxy), right_anti_dual.e23 * this.e415)
        );
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(1.0), 0.0) * float4(other.group0.xyz, 0.0) * float4(wedge.group2.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e235, e315, e125, e4 */
            float4((float3(wedge.e12345) * other.group1.xyz), -(wedge.e423 * other.e415) - (wedge.e431 * other.e425) - (wedge.e412 * other.e435)), 
            /* e1, e2, e3, e5 */
            float4((wedge.e412 * other.e315) + (wedge.e321 * other.e415), (wedge.e423 * other.e125) + (wedge.e321 * other.e425), (wedge.e431 * other.e235) + (wedge.e321 * other.e435), -(wedge.e415 * other.e235) - (wedge.e425 * other.e315) - (wedge.e435 * other.e125) - (wedge.e315 * other.e425) - (wedge.e125 * other.e435)) - float4((wedge.group0.xyz.yzx * other.group1.xyz.zxy), wedge.e235 * other.e415)
        );
    }
}
public extension VersorEven {
    public VersorEven project_orthogonally_onto(Motor other) {
        AntiMotor right_anti_dual = AntiMotor(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        VersorEven wedge = VersorEven(
            /* e423, e431, e412, e12345 */
            (right_anti_dual.group0 * float4(this.group3.www, this.e12345)) + float4(float3(0.0), -(right_anti_dual.e23 * this.e415) - (right_anti_dual.e31 * this.e425) - (right_anti_dual.e12 * this.e435) - (right_anti_dual.e15 * this.e423) - (right_anti_dual.e25 * this.e431) - (right_anti_dual.e35 * this.e412)) + float4((right_anti_dual.group0.www * this.group0.xyz), right_anti_dual.e3215 * this.e4), 
            /* e415, e425, e435, e321 */
            float4(right_anti_dual.e15 * this.e4, right_anti_dual.e25 * this.e4, right_anti_dual.e35 * this.e4, -(right_anti_dual.e23 * this.e1) - (right_anti_dual.e31 * this.e2) - (right_anti_dual.e12 * this.e3)) + (float4(right_anti_dual.scalar) * this.group1), 
            /* e235, e315, e125, e5 */
            float4(((float3(right_anti_dual.scalar) * this.group2.xyz) + (float3(this.e5) * right_anti_dual.group0.xyz) + (right_anti_dual.group1.zxy * this.group3.yzx) - (right_anti_dual.group1.yzx * this.group3.zxy)), right_anti_dual.scalar * this.e5), 
            /* e1, e2, e3, e4 */
            float4(right_anti_dual.scalar) * this.group3
        );
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(other.e12345) * wedge.group0, 
            /* e415, e425, e435, e321 */
            float4(((float3(other.e12345) * wedge.group1.xyz) + (float3(wedge.e12345) * other.group0.xyz)), other.e12345 * wedge.e321), 
            /* e235, e315, e125, e5 */
            (float4(other.e12345) * wedge.group2) + (float4(wedge.e12345) * other.group1) + float4(float3(0.0), -(other.e415 * wedge.e235) - (other.e425 * wedge.e315) - (other.e435 * wedge.e125) - (other.e235 * wedge.e415) - (other.e315 * wedge.e425) - (other.e125 * wedge.e435)), 
            /* e1, e2, e3, e4 */
            float4((other.e12345 * wedge.e1) + (other.e315 * wedge.e412), (other.e12345 * wedge.e2) + (other.e125 * wedge.e423), (other.e12345 * wedge.e3) + (other.e235 * wedge.e431), -(other.e425 * wedge.e431) - (other.e435 * wedge.e412)) + (other.group0 * float4(wedge.group1.www, wedge.e4)) - (wedge.group0.yzxx * float4(other.group1.zxy, other.e415))
        );
    }
}
public extension VersorEven {
    public MultiVector project_orthogonally_onto(MultiVector other) {
        MultiVector right_anti_dual = MultiVector(
            /* scalar, e12345 */
            other.group0.xy.yx * float2(-1.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4(other.group9.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5 */
            other.e3215, 
            /* e15, e25, e35, e45 */
            float4(other.group8.xyz, other.e321 * -1.0), 
            /* e41, e42, e43 */
            other.group7.xyz, 
            /* e23, e31, e12 */
            other.group6.xyz, 
            /* e415, e425, e435, e321 */
            float4(other.group5.xyz, other.e45) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e423, e431, e412 */
            other.group4.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            other.group3.xyz * float3(-1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.group1.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234 */
            other.e4 * -1.0
        );
        MultiVector wedge = MultiVector(
            /* scalar, e12345 */
            float2(0.0, (right_anti_dual.scalar * this.e12345) + (right_anti_dual.e4235 * this.e1) + (right_anti_dual.e4315 * this.e2) + (right_anti_dual.e4125 * this.e3) + (right_anti_dual.e3215 * this.e4) + (right_anti_dual.e1234 * this.e5) - (right_anti_dual.e15 * this.e423) - (right_anti_dual.e25 * this.e431) - (right_anti_dual.e35 * this.e412) - (right_anti_dual.e45 * this.e321) - (right_anti_dual.e41 * this.e235) - (right_anti_dual.e42 * this.e315) - (right_anti_dual.e43 * this.e125) - (right_anti_dual.e23 * this.e415) - (right_anti_dual.e31 * this.e425) - (right_anti_dual.e12 * this.e435)), 
            /* e1, e2, e3, e4 */
            float4(right_anti_dual.scalar) * this.group3, 
            /* e5 */
            right_anti_dual.scalar * this.e5, 
            /* e15, e25, e35, e45 */
            (float4(right_anti_dual.e5) * this.group3) - (float4(this.e5) * right_anti_dual.group1), 
            /* e41, e42, e43 */
            (float3(this.e4) * right_anti_dual.group1.xyz) - (float3(right_anti_dual.e4) * this.group3.xyz), 
            /* e23, e31, e12 */
            (right_anti_dual.group1.zxy * this.group3.yzx) - (right_anti_dual.group1.yzx * this.group3.zxy), 
            /* e415, e425, e435, e321 */
            float4((right_anti_dual.e15 * this.e4) + (right_anti_dual.e41 * this.e5), (right_anti_dual.e25 * this.e4) + (right_anti_dual.e42 * this.e5), (right_anti_dual.e35 * this.e4) + (right_anti_dual.e43 * this.e5), -(right_anti_dual.e31 * this.e2) - (right_anti_dual.e12 * this.e3)) + (float4(right_anti_dual.scalar) * this.group1) - (this.group3.xyzx * float4(right_anti_dual.group3.www, right_anti_dual.e23)), 
            /* e423, e431, e412 */
            (float3(right_anti_dual.scalar) * this.group0.xyz) + (float3(this.e4) * right_anti_dual.group5.xyz) + (right_anti_dual.group4.xyz.yzx * this.group3.zxy) - (right_anti_dual.group4.xyz.zxy * this.group3.yzx), 
            /* e235, e315, e125 */
            (float3(right_anti_dual.scalar) * this.group2.xyz) + (float3(this.e5) * right_anti_dual.group5.xyz) + (right_anti_dual.group3.zxy * this.group3.yzx) - (right_anti_dual.group3.yzx * this.group3.zxy), 
            /* e4235, e4315, e4125, e3215 */
            (float4(right_anti_dual.e5) * float4(this.group0.xyz, this.e321)) + (right_anti_dual.group1.yzxy * float4(this.group1.zxy, this.e315)) + float4((right_anti_dual.group8.xyz * this.group3.www), right_anti_dual.e1 * this.e235) + float4((right_anti_dual.group6.yzx * this.group3.zxy), right_anti_dual.e3 * this.e125) - (right_anti_dual.group6.zxyw * float4(this.group3.yzx, this.e5)) - float4((right_anti_dual.group7.xyz * this.group2.www), right_anti_dual.e235 * this.e1) - float4((right_anti_dual.group1.zxy * this.group1.yzx), right_anti_dual.e315 * this.e2) - float4((right_anti_dual.group1.www * this.group2.xyz), right_anti_dual.e125 * this.e3), 
            /* e1234 */
            (right_anti_dual.e321 * this.e4) + (right_anti_dual.e423 * this.e1) + (right_anti_dual.e431 * this.e2) + (right_anti_dual.e412 * this.e3) - (right_anti_dual.e1 * this.e423) - (right_anti_dual.e2 * this.e431) - (right_anti_dual.e3 * this.e412) - (right_anti_dual.e4 * this.e321)
        );
        return MultiVector(
            /* scalar, e12345 */
            float2((other.scalar * wedge.e12345) + (other.e12345 * wedge.scalar) + (other.e1 * wedge.e4235) + (other.e2 * wedge.e4315) + (other.e3 * wedge.e4125) + (other.e4 * wedge.e3215) + (other.e5 * wedge.e1234) + (other.e4235 * wedge.e1) + (other.e4315 * wedge.e2) + (other.e4125 * wedge.e3) + (other.e3215 * wedge.e4) + (other.e1234 * wedge.e5) - (other.e15 * wedge.e423) - (other.e25 * wedge.e431) - (other.e35 * wedge.e412) - (other.e45 * wedge.e321) - (other.e41 * wedge.e235) - (other.e42 * wedge.e315) - (other.e43 * wedge.e125) - (other.e23 * wedge.e415) - (other.e31 * wedge.e425) - (other.e12 * wedge.e435) - (other.e415 * wedge.e23) - (other.e425 * wedge.e31) - (other.e435 * wedge.e12) - (other.e321 * wedge.e45) - (other.e423 * wedge.e15) - (other.e431 * wedge.e25) - (other.e412 * wedge.e35) - (other.e235 * wedge.e41) - (other.e315 * wedge.e42) - (other.e125 * wedge.e43), other.e12345 * wedge.e12345), 
            /* e1, e2, e3, e4 */
            float4((other.e415 * wedge.e321) + (other.e321 * wedge.e415) + (other.e4315 * wedge.e12) + (other.e1234 * wedge.e15), (other.e425 * wedge.e321) + (other.e321 * wedge.e425) + (other.e4125 * wedge.e23) + (other.e1234 * wedge.e25), (other.e435 * wedge.e321) + (other.e321 * wedge.e435) + (other.e4235 * wedge.e31) + (other.e1234 * wedge.e35), -(other.e45 * wedge.e1234) - (other.e415 * wedge.e423) - (other.e425 * wedge.e431) - (other.e435 * wedge.e412)) + (float4(other.e12345) * wedge.group1) + (float4(wedge.e12345) * other.group1) + float4((other.group4.xyz * wedge.group9.www), other.e4235 * wedge.e41) + float4((other.group5.xyz.yzx * wedge.group9.zxy), other.e4315 * wedge.e42) + float4((other.group7.xyz.zxy * wedge.group8.xyz.yzx), other.e4125 * wedge.e43) + float4((other.group8.xyz.yzx * wedge.group7.xyz.zxy), other.e1234 * wedge.e45) - (wedge.group9.yzxx * float4(other.group5.xyz.zxy, other.e41)) - float4((float3(wedge.e1234) * other.group3.xyz), other.e412 * wedge.e435) - float4((wedge.group4.xyz * other.group9.www), other.e423 * wedge.e415) - float4((other.group7.xyz.yzx * wedge.group8.xyz.zxy), other.e42 * wedge.e4315) - float4((other.group8.xyz.zxy * wedge.group7.xyz.yzx), other.e43 * wedge.e4125) - float4((wedge.group5.xyz.yzx * other.group9.zxy), other.e431 * wedge.e425), 
            /* e5 */
            (other.e12345 * wedge.e5) + (other.e5 * wedge.e12345) + (other.e15 * wedge.e4235) + (other.e25 * wedge.e4315) + (other.e35 * wedge.e4125) + (other.e45 * wedge.e3215) - (other.e415 * wedge.e235) - (other.e425 * wedge.e315) - (other.e435 * wedge.e125) - (other.e235 * wedge.e415) - (other.e315 * wedge.e425) - (other.e125 * wedge.e435) - (other.e4235 * wedge.e15) - (other.e4315 * wedge.e25) - (other.e4125 * wedge.e35) - (other.e3215 * wedge.e45), 
            /* e15, e25, e35, e45 */
            float4((other.e415 * wedge.e3215) + (other.e315 * wedge.e4125) + (other.e4125 * wedge.e315) + (other.e3215 * wedge.e415), (other.e425 * wedge.e3215) + (other.e125 * wedge.e4235) + (other.e4235 * wedge.e125) + (other.e3215 * wedge.e425), (other.e435 * wedge.e3215) + (other.e235 * wedge.e4315) + (other.e4315 * wedge.e235) + (other.e3215 * wedge.e435), -(other.e435 * wedge.e4125) - (other.e4235 * wedge.e415) - (other.e4315 * wedge.e425) - (other.e4125 * wedge.e435)) + (float4(other.e12345) * wedge.group3) + (float4(wedge.e12345) * other.group3) - (wedge.group9.yzxx * float4(other.group8.xyz.zxy, other.e415)) - float4((wedge.group8.xyz.zxy * other.group9.yzx), other.e425 * wedge.e4315), 
            /* e41, e42, e43 */
            (float3(other.e12345) * wedge.group4.xyz) + (float3(other.e1234) * wedge.group6.xyz) + (float3(wedge.e12345) * other.group4.xyz) + (float3(wedge.e1234) * other.group6.xyz) + (other.group7.xyz.zxy * wedge.group9.yzx) + (wedge.group7.xyz.zxy * other.group9.yzx) - (other.group7.xyz.yzx * wedge.group9.zxy) - (wedge.group7.xyz.yzx * other.group9.zxy), 
            /* e23, e31, e12 */
            (float3(other.e12345) * wedge.group5.xyz) + (float3(other.e3215) * wedge.group7.xyz) + (float3(other.e1234) * wedge.group8.xyz) + (float3(wedge.e12345) * other.group5.xyz) + (float3(wedge.e3215) * other.group7.xyz) + (float3(wedge.e1234) * other.group8.xyz) - (float3(other.e321) * wedge.group9.xyz) - (float3(wedge.e321) * other.group9.xyz), 
            /* e415, e425, e435, e321 */
            (float4(other.e12345) * wedge.group6) + (float4(wedge.e12345) * other.group6) + float4((other.group9.zxy * wedge.group9.yzx), other.e1234 * wedge.e3215) - (other.group9.yzxw * float4(wedge.group9.zxy, wedge.e1234)), 
            /* e423, e431, e412 */
            (float3(other.e12345) * wedge.group7.xyz) + (float3(other.e1234) * wedge.group9.xyz) + (float3(wedge.e12345) * other.group7.xyz) - (float3(wedge.e1234) * other.group9.xyz), 
            /* e235, e315, e125 */
            (float3(other.e12345) * wedge.group8.xyz) + (float3(wedge.e12345) * other.group8.xyz) + (float3(wedge.e3215) * other.group9.xyz) - (float3(other.e3215) * wedge.group9.xyz), 
            /* e4235, e4315, e4125, e3215 */
            (float4(other.e12345) * wedge.group9) + (float4(wedge.e12345) * other.group9), 
            /* e1234 */
            (other.e12345 * wedge.e1234) + (other.e1234 * wedge.e12345)
        );
    }
}
public extension VersorEven {
    public AntiDipoleInversion project_orthogonally_onto(Plane other) {
        AntiPlane right_anti_dual = AntiPlane(
            /* e1, e2, e3, e5 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        DipoleInversion wedge = DipoleInversion(
            /* e41, e42, e43 */
            float3(this.e4) * right_anti_dual.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(((right_anti_dual.group0.zxy * this.group3.yzx) - (right_anti_dual.group0.yzx * this.group3.zxy)), right_anti_dual.e5 * this.e4), 
            /* e15, e25, e35, e1234 */
            float4(right_anti_dual.e5 * this.e1, right_anti_dual.e5 * this.e2, right_anti_dual.e5 * this.e3, -(right_anti_dual.e2 * this.e431) - (right_anti_dual.e3 * this.e412)) - (right_anti_dual.group0.xyzx * float4(this.group2.www, this.e423)), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual.e3 * this.e425 * -1.0, right_anti_dual.e1 * this.e435 * -1.0, right_anti_dual.e2 * this.e415 * -1.0, (right_anti_dual.e3 * this.e125) + (right_anti_dual.e5 * this.e321)) + (right_anti_dual.group0.yzxx * float4(this.group1.zxy, this.e235)) + (right_anti_dual.group0.wwwy * float4(this.group0.xyz, this.e315))
        );
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(wedge.e1234) * other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            float4(((wedge.group3.yzx * other.group0.zxy) - (wedge.group3.zxy * other.group0.yzx)), wedge.e1234 * other.e3215 * -1.0), 
            /* e235, e315, e125, e4 */
            float4(wedge.e4235 * other.e3215 * -1.0, wedge.e4315 * other.e3215 * -1.0, wedge.e4125 * other.e3215 * -1.0, (wedge.e42 * other.e4315) + (wedge.e43 * other.e4125)) + (other.group0.xyzx * float4(wedge.group3.www, wedge.e41)), 
            /* e1, e2, e3, e5 */
            float4(wedge.e12 * other.e4315, wedge.e23 * other.e4125, wedge.e31 * other.e4235, -(wedge.e25 * other.e4315) - (wedge.e35 * other.e4125)) - (float4(other.e3215) * float4(wedge.group0.xyz, wedge.e45)) - (other.group0.zxyx * float4(wedge.group1.yzx, wedge.e15))
        );
    }
}
public extension VersorEven {
    public RoundPoint project_orthogonally_onto(RoundPoint other) {
        Sphere right_anti_dual = Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234 */
            other.e4 * -1.0
        );
        AntiScalar wedge = AntiScalar(
            /* e12345 */
            (right_anti_dual.e4235 * this.e1) + (right_anti_dual.e4315 * this.e2) + (right_anti_dual.e4125 * this.e3) + (right_anti_dual.e3215 * this.e4) + (right_anti_dual.e1234 * this.e5)
        );
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(wedge.e12345) * other.group0, 
            /* e5 */
            wedge.e12345 * other.e5
        );
    }
}
public extension VersorEven {
    public AntiDipoleInversion project_orthogonally_onto(Sphere other) {
        RoundPoint right_anti_dual = RoundPoint(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5 */
            other.e3215
        );
        DipoleInversion wedge = DipoleInversion(
            /* e41, e42, e43 */
            (float3(this.e4) * right_anti_dual.group0.xyz) - (float3(right_anti_dual.e4) * this.group3.xyz), 
            /* e23, e31, e12, e45 */
            float4((right_anti_dual.group0.zxy * this.group3.yzx), right_anti_dual.e5 * this.e4) - (right_anti_dual.group0.yzxw * float4(this.group3.zxy, this.e5)), 
            /* e15, e25, e35, e1234 */
            float4(right_anti_dual.e5 * this.e1, right_anti_dual.e5 * this.e2, right_anti_dual.e5 * this.e3, -(right_anti_dual.e2 * this.e431) - (right_anti_dual.e3 * this.e412) - (right_anti_dual.e4 * this.e321)) - (right_anti_dual.group0.xyzx * float4(this.group2.www, this.e423)), 
            /* e4235, e4315, e4125, e3215 */
            float4(-(right_anti_dual.e3 * this.e425) - (right_anti_dual.e4 * this.e235), -(right_anti_dual.e1 * this.e435) - (right_anti_dual.e4 * this.e315), -(right_anti_dual.e2 * this.e415) - (right_anti_dual.e4 * this.e125), (right_anti_dual.e3 * this.e125) + (right_anti_dual.e5 * this.e321)) + (right_anti_dual.group0.yzxx * float4(this.group1.zxy, this.e235)) + float4((float3(right_anti_dual.e5) * this.group0.xyz), right_anti_dual.e2 * this.e315)
        );
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (float3(other.e1234) * wedge.group3.xyz) - (float3(wedge.e1234) * other.group0.xyz), 
            /* e415, e425, e435, e321 */
            (wedge.group3.yzxw * float4(other.group0.zxy, other.e1234)) - (other.group0.yzxw * float4(wedge.group3.zxy, wedge.e1234)), 
            /* e235, e315, e125, e4 */
            float4(wedge.e4235 * other.e3215 * -1.0, wedge.e4315 * other.e3215 * -1.0, wedge.e4125 * other.e3215 * -1.0, (wedge.e42 * other.e4315) + (wedge.e43 * other.e4125) + (wedge.e45 * other.e1234)) + (other.group0.xyzx * float4(wedge.group3.www, wedge.e41)), 
            /* e1, e2, e3, e5 */
            float4((wedge.e12 * other.e4315) + (wedge.e15 * other.e1234), (wedge.e23 * other.e4125) + (wedge.e25 * other.e1234), (wedge.e31 * other.e4235) + (wedge.e35 * other.e1234), -(wedge.e25 * other.e4315) - (wedge.e35 * other.e4125)) - (float4(other.e3215) * float4(wedge.group0.xyz, wedge.e45)) - (other.group0.zxyx * float4(wedge.group1.yzx, wedge.e15))
        );
    }
}
public extension VersorEven {
    public VersorEven project_orthogonally_onto(VersorEven other) {
        VersorOdd right_anti_dual = VersorOdd(
            /* e41, e42, e43, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e1234 */
            float4(other.group2.xyz, other.e4) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.group3.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0)
        );
        VersorEven wedge = VersorEven(
            /* e423, e431, e412, e12345 */
            (float4(right_anti_dual.scalar) * this.group0) + (this.group3.wwwx * float4(right_anti_dual.group1.xyz, right_anti_dual.e4235)) + float4(float3(0.0), (this.e2 * right_anti_dual.e4315) + (this.e3 * right_anti_dual.e4125) + (this.e4 * right_anti_dual.e3215) - (this.e431 * right_anti_dual.e25) - (this.e412 * right_anti_dual.e35) - (this.e415 * right_anti_dual.e23) - (this.e425 * right_anti_dual.e31) - (this.e435 * right_anti_dual.e12) - (this.e321 * right_anti_dual.e45) - (this.e235 * right_anti_dual.e41) - (this.e315 * right_anti_dual.e42) - (this.e125 * right_anti_dual.e43)) + float4((this.group3.zxy * right_anti_dual.group0.yzx), this.e5 * right_anti_dual.e1234) - float4((this.group3.yzx * right_anti_dual.group0.zxy), this.e423 * right_anti_dual.e15), 
            /* e415, e425, e435, e321 */
            float4((this.e5 * right_anti_dual.e41) + (this.e4 * right_anti_dual.e15), (this.e5 * right_anti_dual.e42) + (this.e4 * right_anti_dual.e25), (this.e5 * right_anti_dual.e43) + (this.e4 * right_anti_dual.e35), -(this.e2 * right_anti_dual.e31) - (this.e3 * right_anti_dual.e12)) + (float4(right_anti_dual.scalar) * this.group1) - (this.group3.xyzx * right_anti_dual.group1.wwwx), 
            /* e235, e315, e125, e5 */
            float4(((float3(this.e5) * right_anti_dual.group1.xyz) + (float3(right_anti_dual.scalar) * this.group2.xyz) + (this.group3.yzx * right_anti_dual.group2.zxy) - (this.group3.zxy * right_anti_dual.group2.yzx)), this.e5 * right_anti_dual.scalar), 
            /* e1, e2, e3, e4 */
            float4(right_anti_dual.scalar) * this.group3
        );
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(((float3(other.e12345) * wedge.group0.xyz) + (float3(wedge.e12345) * other.group0.xyz)), other.e12345 * wedge.e12345), 
            /* e415, e425, e435, e321 */
            (float4(other.e12345) * wedge.group1) + (float4(wedge.e12345) * other.group1), 
            /* e235, e315, e125, e5 */
            (float4(other.e12345) * wedge.group2) + (float4(wedge.e12345) * other.group2) + float4(float3(0.0), -(other.e415 * wedge.e235) - (other.e425 * wedge.e315) - (other.e435 * wedge.e125) - (other.e235 * wedge.e415) - (other.e315 * wedge.e425) - (other.e125 * wedge.e435)), 
            /* e1, e2, e3, e4 */
            float4((other.e415 * wedge.e321) + (other.e321 * wedge.e415) + (other.e315 * wedge.e412) + (other.e1 * wedge.e12345), (other.e425 * wedge.e321) + (other.e321 * wedge.e425) + (other.e125 * wedge.e423) + (other.e2 * wedge.e12345), (other.e435 * wedge.e321) + (other.e321 * wedge.e435) + (other.e235 * wedge.e431) + (other.e3 * wedge.e12345), -(other.e412 * wedge.e435) - (other.e415 * wedge.e423) - (other.e425 * wedge.e431) - (other.e435 * wedge.e412)) + (other.group0.zxyw * float4(wedge.group2.yzx, wedge.e4)) + float4((other.group0.www * wedge.group3.xyz), other.e4 * wedge.e12345) - (other.group0.yzxx * float4(wedge.group2.zxy, wedge.e415)) - float4((other.group2.zxy * wedge.group0.yzx), other.e431 * wedge.e425)
        );
    }
}
public extension VersorEven {
    public AntiDipoleInversion project_orthogonally_onto(VersorOdd other) {
        VersorEven right_anti_dual = VersorEven(
            /* e423, e431, e412, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            float4(other.group2.xyz, other.e3215) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4(other.group3.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0)
        );
        DipoleInversion wedge = DipoleInversion(
            /* e41, e42, e43 */
            (float3(this.e4) * right_anti_dual.group3.xyz) - (float3(right_anti_dual.e4) * this.group3.xyz), 
            /* e23, e31, e12, e45 */
            (this.group3.yzxw * float4(right_anti_dual.group3.zxy, right_anti_dual.e5)) - (right_anti_dual.group3.yzxw * float4(this.group3.zxy, this.e5)), 
            /* e15, e25, e35, e1234 */
            (this.group3.xyzx * float4(right_anti_dual.group2.www, right_anti_dual.e423)) + float4(float3(0.0), (right_anti_dual.e431 * this.e2) + (right_anti_dual.e412 * this.e3) + (right_anti_dual.e321 * this.e4) - (right_anti_dual.e2 * this.e431) - (right_anti_dual.e3 * this.e412) - (right_anti_dual.e4 * this.e321)) - (right_anti_dual.group3.xyzx * float4(this.group2.www, this.e423)), 
            /* e4235, e4315, e4125, e3215 */
            (right_anti_dual.group3.yzxz * float4(this.group1.zxy, this.e125)) + float4((right_anti_dual.group1.yzx * this.group3.zxy), right_anti_dual.e5 * this.e321) + float4((right_anti_dual.group2.www * this.group0.xyz), right_anti_dual.e2 * this.e315) + float4((this.group3.www * right_anti_dual.group2.xyz), right_anti_dual.e1 * this.e235) - (float4(this.e5) * float4(right_anti_dual.group0.xyz, right_anti_dual.e321)) - (this.group3.yzxx * float4(right_anti_dual.group1.zxy, right_anti_dual.e235)) - float4((right_anti_dual.group3.zxy * this.group1.yzx), right_anti_dual.e315 * this.e2) - float4((right_anti_dual.group3.www * this.group2.xyz), right_anti_dual.e125 * this.e3)
        );
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (float3(other.e1234) * wedge.group3.xyz) - (float3(wedge.e1234) * other.group3.xyz), 
            /* e415, e425, e435, e321 */
            (wedge.group3.yzxw * float4(other.group3.zxy, other.e1234)) - (other.group3.yzxw * float4(wedge.group3.zxy, wedge.e1234)), 
            /* e235, e315, e125, e4 */
            (other.group3.xyzx * float4(wedge.group3.www, wedge.e41)) + float4(float3(0.0), (wedge.e42 * other.e4315) + (wedge.e43 * other.e4125) + (wedge.e45 * other.e1234) - (wedge.e4235 * other.e41) - (wedge.e4315 * other.e42) - (wedge.e4125 * other.e43)) - float4((other.group3.www * wedge.group3.xyz), wedge.e1234 * other.e45), 
            /* e1, e2, e3, e5 */
            (float4(wedge.e3215) * float4(other.group0.xyz, other.e45)) + (wedge.group3.zxyz * float4(other.group1.yzx, other.e35)) + (other.group2.wwwy * float4(wedge.group2.xyz, wedge.e4315)) + float4((wedge.group1.zxy * other.group3.yzx), wedge.e4235 * other.e15) - (float4(other.e3215) * float4(wedge.group0.xyz, wedge.e45)) - (wedge.group2.wwwy * float4(other.group2.xyz, other.e4315)) - (other.group3.zxyx * float4(wedge.group1.yzx, wedge.e15)) - float4((wedge.group3.yzx * other.group1.zxy), wedge.e35 * other.e4125)
        );
    }
}
public extension VersorOdd {
    // Fancy infix trick (first half)
    public ProjectOrthogonallyOntoInfixPartial<VersorOdd> operator /(ProjectOrthogonallyOntoInfix rhs) {
        return ProjectOrthogonallyOntoInfixPartial<VersorOdd>(this);
    }
}
public extension ProjectOrthogonallyOntoInfixPartial<VersorOdd> {
    // Fancy infix trick (second half)
    public AntiCircleRotor operator /(AntiCircleRotor rhs) {
        return this.a.project_orthogonally_onto(rhs);
    }
}
public extension VersorOdd {
    public AntiCircleRotor project_orthogonally_onto(AntiCircleRotor other) {
        CircleRotor right_anti_dual = CircleRotor(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        CircleRotor wedge = CircleRotor(
            /* e423, e431, e412 */
            float3(this.scalar) * right_anti_dual.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(this.scalar) * right_anti_dual.group1, 
            /* e235, e315, e125, e12345 */
            float4((float3(this.scalar) * right_anti_dual.group2.xyz), (right_anti_dual.e12345 * this.scalar) - (right_anti_dual.e423 * this.e15) - (right_anti_dual.e431 * this.e25) - (right_anti_dual.e412 * this.e35) - (right_anti_dual.e415 * this.e23) - (right_anti_dual.e425 * this.e31) - (right_anti_dual.e435 * this.e12) - (right_anti_dual.e321 * this.e45) - (right_anti_dual.e235 * this.e41) - (right_anti_dual.e315 * this.e42) - (right_anti_dual.e125 * this.e43))
        );
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(wedge.e12345) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(wedge.e12345) * other.group1, 
            /* e15, e25, e35, scalar */
            float4((float3(wedge.e12345) * other.group2.xyz), (other.scalar * wedge.e12345) - (other.e41 * wedge.e235) - (other.e42 * wedge.e315) - (other.e43 * wedge.e125) - (other.e23 * wedge.e415) - (other.e31 * wedge.e425) - (other.e12 * wedge.e435) - (other.e45 * wedge.e321) - (other.e15 * wedge.e423) - (other.e25 * wedge.e431) - (other.e35 * wedge.e412))
        );
    }
}
public extension VersorOdd {
    public AntiCircleRotor project_orthogonally_onto(AntiDipoleInversion other) {
        DipoleInversion right_anti_dual = DipoleInversion(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e1234 */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group3 * float4(1.0, 1.0, 1.0, -1.0)
        );
        DipoleInversion wedge = DipoleInversion(
            /* e41, e42, e43 */
            float3(this.scalar) * right_anti_dual.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(this.scalar) * right_anti_dual.group1, 
            /* e15, e25, e35, e1234 */
            float4((float3(this.scalar) * right_anti_dual.group2.xyz), (right_anti_dual.e1234 * this.scalar) - (right_anti_dual.e41 * this.e23) - (right_anti_dual.e42 * this.e31) - (right_anti_dual.e43 * this.e12) - (right_anti_dual.e23 * this.e41) - (right_anti_dual.e31 * this.e42) - (right_anti_dual.e12 * this.e43)), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual.e23 * this.e45) + (right_anti_dual.e45 * this.e23) + (right_anti_dual.e35 * this.e42) + (right_anti_dual.e4235 * this.scalar), (right_anti_dual.e31 * this.e45) + (right_anti_dual.e45 * this.e31) + (right_anti_dual.e15 * this.e43) + (right_anti_dual.e4315 * this.scalar), (right_anti_dual.e12 * this.e45) + (right_anti_dual.e45 * this.e12) + (right_anti_dual.e25 * this.e41) + (right_anti_dual.e4125 * this.scalar), -(right_anti_dual.e12 * this.e35) - (right_anti_dual.e15 * this.e23) - (right_anti_dual.e25 * this.e31) - (right_anti_dual.e35 * this.e12)) + float4((right_anti_dual.group0.xyz.yzx * this.group2.zxy), right_anti_dual.e3215 * this.scalar) - (this.group2.yzxx * float4(right_anti_dual.group0.xyz.zxy, right_anti_dual.e23)) - float4((right_anti_dual.group2.yzx * this.group0.zxy), right_anti_dual.e31 * this.e25)
        );
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (float3(wedge.e1234) * other.group1.xyz) + (other.group0.xyz.zxy * wedge.group3.yzx) - (other.group0.xyz.yzx * wedge.group3.zxy), 
            /* e23, e31, e12, e45 */
            float4((other.e423 * wedge.e3215) + (other.e235 * wedge.e1234), (other.e431 * wedge.e3215) + (other.e315 * wedge.e1234), (other.e412 * wedge.e3215) + (other.e125 * wedge.e1234), -(other.e425 * wedge.e4315) - (other.e435 * wedge.e4125)) - (other.group1.wwwx * wedge.group3.xyzx), 
            /* e15, e25, e35, scalar */
            (float4(wedge.e3215) * float4(other.group1.xyz, other.e4)) + (wedge.group3.zxyx * float4(other.group2.yzx, other.e1)) + float4(float3(0.0), (other.e2 * wedge.e4315) + (other.e3 * wedge.e4125) + (other.e5 * wedge.e1234) - (other.e431 * wedge.e25) - (other.e412 * wedge.e35) - (other.e415 * wedge.e23) - (other.e425 * wedge.e31) - (other.e435 * wedge.e12) - (other.e321 * wedge.e45) - (other.e235 * wedge.e41) - (other.e315 * wedge.e42) - (other.e125 * wedge.e43)) - float4((other.group2.zxy * wedge.group3.yzx), other.e423 * wedge.e15)
        );
    }
}
public extension VersorOdd {
    public AntiMotor project_orthogonally_onto(AntiDualNum other) {
        DualNum right_anti_dual = DualNum(
            /* e5, e12345 */
            other.group0.xy
        );
        Motor wedge = Motor(
            /* e415, e425, e435, e12345 */
            float4(right_anti_dual.group0.xy.xx, right_anti_dual.e5, (right_anti_dual.e5 * this.e1234) + (right_anti_dual.e12345 * this.scalar)) * float4(this.group0.xyz, 1.0), 
            /* e235, e315, e125, e5 */
            float4(right_anti_dual.e5) * float4(this.group1.xyz, this.scalar)
        );
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(float3(0.0), other.scalar * wedge.e12345), 
            /* e15, e25, e35, e3215 */
            float4(other.e3215) * wedge.group0
        );
    }
}
public extension VersorOdd {
    public AntiMotor project_orthogonally_onto(AntiFlatPoint other) {
        FlatPoint right_anti_dual = FlatPoint(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        Flector wedge = Flector(
            /* e15, e25, e35, e45 */
            float4(this.scalar) * right_anti_dual.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual.e35 * this.e42) + (right_anti_dual.e45 * this.e23), (right_anti_dual.e15 * this.e43) + (right_anti_dual.e45 * this.e31), (right_anti_dual.e25 * this.e41) + (right_anti_dual.e45 * this.e12), -(right_anti_dual.e25 * this.e31) - (right_anti_dual.e35 * this.e12)) - (right_anti_dual.group0.yzxx * float4(this.group0.zxy, this.e23))
        );
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(other.e321) * float4(wedge.group1.xyz, wedge.e45) * float4(-1.0), 
            /* e15, e25, e35, e3215 */
            float4(((other.group0.yzx * wedge.group1.zxy) - (other.group0.zxy * wedge.group1.yzx)), 0.0)
        );
    }
}
public extension VersorOdd {
    public AntiMotor project_orthogonally_onto(AntiFlector other) {
        Flector right_anti_dual = Flector(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        Flector wedge = Flector(
            /* e15, e25, e35, e45 */
            float4(this.scalar) * right_anti_dual.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual.e45 * this.e23) + (right_anti_dual.e4235 * this.scalar), (right_anti_dual.e45 * this.e31) + (right_anti_dual.e4315 * this.scalar), (right_anti_dual.e45 * this.e12) + (right_anti_dual.e4125 * this.scalar), -(right_anti_dual.e25 * this.e31) - (right_anti_dual.e35 * this.e12)) + (this.group0.yzxw * float4(right_anti_dual.group0.zxy, right_anti_dual.e3215)) - (right_anti_dual.group0.yzxx * float4(this.group0.zxy, this.e23))
        );
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((other.group0.www * wedge.group1.xyz * float3(-1.0)), (other.e1 * wedge.e4235) + (other.e2 * wedge.e4315) + (other.e3 * wedge.e4125) - (other.e321 * wedge.e45)), 
            /* e15, e25, e35, e3215 */
            float4(((other.group0.yzx * wedge.group1.zxy) - (other.group0.zxy * wedge.group1.yzx)), 0.0)
        );
    }
}
public extension VersorOdd {
    public AntiMotor project_orthogonally_onto(AntiLine other) {
        Line right_anti_dual = Line(
            /* e415, e425, e435 */
            other.group0.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            other.group1.xyz * float3(-1.0)
        );
        Motor wedge = Motor(
            /* e415, e425, e435, e12345 */
            float4((float3(this.scalar) * right_anti_dual.group0.xyz), -(right_anti_dual.e415 * this.e23) - (right_anti_dual.e425 * this.e31) - (right_anti_dual.e435 * this.e12) - (right_anti_dual.e235 * this.e41) - (right_anti_dual.e315 * this.e42) - (right_anti_dual.e125 * this.e43)), 
            /* e235, e315, e125, e5 */
            float4(float3(1.0), 0.0) * float4(right_anti_dual.group1.xyz, 0.0) * float4(this.group0.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((float3(wedge.e12345) * other.group0.xyz), -(other.e23 * wedge.e415) - (other.e31 * wedge.e425) - (other.e12 * wedge.e435)), 
            /* e15, e25, e35, e3215 */
            float4(float3(1.0), 0.0) * float4(other.group1.xyz, 0.0) * float4(wedge.group0.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
public extension VersorOdd {
    public AntiMotor project_orthogonally_onto(AntiMotor other) {
        Motor right_anti_dual = Motor(
            /* e415, e425, e435, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        Motor wedge = Motor(
            /* e415, e425, e435, e12345 */
            (float4(right_anti_dual.e5) * float4(this.group0.xyz, this.e1234)) + (float4(this.scalar) * right_anti_dual.group0) + float4(float3(0.0), -(right_anti_dual.e415 * this.e23) - (right_anti_dual.e425 * this.e31) - (right_anti_dual.e435 * this.e12) - (right_anti_dual.e235 * this.e41) - (right_anti_dual.e315 * this.e42) - (right_anti_dual.e125 * this.e43)), 
            /* e235, e315, e125, e5 */
            float4(((float3(right_anti_dual.e5) * this.group1.xyz) + (float3(this.scalar) * right_anti_dual.group1.xyz)), right_anti_dual.e5 * this.scalar)
        );
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((float3(wedge.e12345) * other.group0.xyz), (other.scalar * wedge.e12345) - (other.e23 * wedge.e415) - (other.e31 * wedge.e425) - (other.e12 * wedge.e435)), 
            /* e15, e25, e35, e3215 */
            float4(((float3(other.e3215) * wedge.group0.xyz) + (float3(wedge.e12345) * other.group1.xyz)), other.e3215 * wedge.e12345)
        );
    }
}
public extension VersorOdd {
    public Scalar project_orthogonally_onto(AntiPlane other) {
        Plane wedge = Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar) * float4(other.group0.xyz, other.e5 * -1.0)
        );
        return Scalar(
            /* scalar */
            (other.e1 * wedge.e4235) + (other.e2 * wedge.e4315) + (other.e3 * wedge.e4125)
        );
    }
}
public extension VersorOdd {
    public VersorOdd project_orthogonally_onto(AntiScalar other) {
        Scalar right_anti_dual = Scalar(
            /* scalar */
            other.e12345 * -1.0
        );
        VersorOdd wedge = VersorOdd(
            /* e41, e42, e43, scalar */
            float4(right_anti_dual.scalar) * this.group0, 
            /* e23, e31, e12, e45 */
            float4(right_anti_dual.scalar) * this.group1, 
            /* e15, e25, e35, e1234 */
            float4(right_anti_dual.scalar) * this.group2, 
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual.scalar) * this.group3
        );
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(other.e12345) * wedge.group0, 
            /* e23, e31, e12, e45 */
            float4(other.e12345) * wedge.group1, 
            /* e15, e25, e35, e1234 */
            float4(other.e12345) * wedge.group2, 
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345) * wedge.group3
        );
    }
}
public extension VersorOdd {
    public AntiCircleRotor project_orthogonally_onto(Circle other) {
        Dipole right_anti_dual = Dipole(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35 */
            other.group2.xyz
        );
        DipoleInversion wedge = DipoleInversion(
            /* e41, e42, e43 */
            float3(this.scalar) * right_anti_dual.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(this.scalar) * right_anti_dual.group1, 
            /* e15, e25, e35, e1234 */
            float4((float3(this.scalar) * right_anti_dual.group2.xyz), -(right_anti_dual.e41 * this.e23) - (right_anti_dual.e42 * this.e31) - (right_anti_dual.e43 * this.e12) - (right_anti_dual.e23 * this.e41) - (right_anti_dual.e31 * this.e42) - (right_anti_dual.e12 * this.e43)), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual.e42 * this.e35) + (right_anti_dual.e23 * this.e45) + (right_anti_dual.e45 * this.e23) + (right_anti_dual.e35 * this.e42), (right_anti_dual.e43 * this.e15) + (right_anti_dual.e31 * this.e45) + (right_anti_dual.e45 * this.e31) + (right_anti_dual.e15 * this.e43), (right_anti_dual.e41 * this.e25) + (right_anti_dual.e12 * this.e45) + (right_anti_dual.e45 * this.e12) + (right_anti_dual.e25 * this.e41), -(right_anti_dual.e23 * this.e15) - (right_anti_dual.e31 * this.e25) - (right_anti_dual.e12 * this.e35) - (right_anti_dual.e35 * this.e12)) - float4((right_anti_dual.group0.xyz.zxy * this.group2.yzx), right_anti_dual.e15 * this.e23) - float4((right_anti_dual.group2.xyz.yzx * this.group0.zxy), right_anti_dual.e25 * this.e31)
        );
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (float3(wedge.e1234) * other.group1.xyz) + (other.group0.xyz.zxy * wedge.group3.yzx) - (other.group0.xyz.yzx * wedge.group3.zxy), 
            /* e23, e31, e12, e45 */
            float4((other.e423 * wedge.e3215) + (other.e235 * wedge.e1234), (other.e431 * wedge.e3215) + (other.e315 * wedge.e1234), (other.e412 * wedge.e3215) + (other.e125 * wedge.e1234), -(other.e425 * wedge.e4315) - (other.e435 * wedge.e4125)) - (other.group1.wwwx * wedge.group3.xyzx), 
            /* e15, e25, e35, scalar */
            float4((other.e415 * wedge.e3215) + (other.e315 * wedge.e4125), (other.e425 * wedge.e3215) + (other.e125 * wedge.e4235), (other.e435 * wedge.e3215) + (other.e235 * wedge.e4315), -(other.e431 * wedge.e25) - (other.e412 * wedge.e35) - (other.e415 * wedge.e23) - (other.e425 * wedge.e31) - (other.e435 * wedge.e12) - (other.e321 * wedge.e45) - (other.e235 * wedge.e41) - (other.e315 * wedge.e42) - (other.e125 * wedge.e43)) - float4((other.group2.xyz.zxy * wedge.group3.yzx), other.e423 * wedge.e15)
        );
    }
}
public extension VersorOdd {
    public VersorOdd project_orthogonally_onto(CircleRotor other) {
        AntiCircleRotor right_anti_dual = AntiCircleRotor(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        VersorOdd wedge = VersorOdd(
            /* e41, e42, e43, scalar */
            float4(((float3(right_anti_dual.scalar) * this.group0.xyz) + (float3(this.scalar) * right_anti_dual.group0.xyz)), right_anti_dual.scalar * this.scalar), 
            /* e23, e31, e12, e45 */
            (float4(right_anti_dual.scalar) * this.group1) + (float4(this.scalar) * right_anti_dual.group1), 
            /* e15, e25, e35, e1234 */
            float4(right_anti_dual.scalar * this.e15, right_anti_dual.scalar * this.e25, right_anti_dual.scalar * this.e35, -(right_anti_dual.e41 * this.e23) - (right_anti_dual.e42 * this.e31) - (right_anti_dual.e43 * this.e12) - (right_anti_dual.e23 * this.e41) - (right_anti_dual.e31 * this.e42) - (right_anti_dual.e12 * this.e43)) + (right_anti_dual.group2 * float4(this.group0.www, this.e1234)), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual.e23 * this.e45) + (right_anti_dual.e45 * this.e23) + (right_anti_dual.e35 * this.e42) + (right_anti_dual.scalar * this.e4235), (right_anti_dual.e31 * this.e45) + (right_anti_dual.e45 * this.e31) + (right_anti_dual.e15 * this.e43) + (right_anti_dual.scalar * this.e4315), (right_anti_dual.e12 * this.e45) + (right_anti_dual.e45 * this.e12) + (right_anti_dual.e25 * this.e41) + (right_anti_dual.scalar * this.e4125), -(right_anti_dual.e12 * this.e35) - (right_anti_dual.e15 * this.e23) - (right_anti_dual.e25 * this.e31) - (right_anti_dual.e35 * this.e12)) + float4((right_anti_dual.group0.xyz.yzx * this.group2.zxy), right_anti_dual.scalar * this.e3215) - (this.group2.yzxx * float4(right_anti_dual.group0.xyz.zxy, right_anti_dual.e23)) - float4((right_anti_dual.group2.yzx * this.group0.zxy), right_anti_dual.e31 * this.e25)
        );
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4((other.e415 * wedge.e1234) + (other.e12345 * wedge.e41), (other.e425 * wedge.e1234) + (other.e12345 * wedge.e42), (other.e435 * wedge.e1234) + (other.e12345 * wedge.e43), -(other.e431 * wedge.e25) - (other.e412 * wedge.e35) - (other.e415 * wedge.e23) - (other.e425 * wedge.e31) - (other.e435 * wedge.e12) - (other.e321 * wedge.e45) - (other.e235 * wedge.e41) - (other.e315 * wedge.e42) - (other.e125 * wedge.e43)) + float4((other.group0.xyz.zxy * wedge.group3.yzx), other.e12345 * wedge.scalar) - float4((other.group0.xyz.yzx * wedge.group3.zxy), other.e423 * wedge.e15), 
            /* e23, e31, e12, e45 */
            float4((other.e235 * wedge.e1234) + (other.e12345 * wedge.e23), (other.e315 * wedge.e1234) + (other.e12345 * wedge.e31), (other.e125 * wedge.e1234) + (other.e12345 * wedge.e12), -(other.e425 * wedge.e4315) - (other.e435 * wedge.e4125)) + float4((other.group0.xyz * wedge.group3.www), other.e12345 * wedge.e45) - (other.group1.wwwx * wedge.group3.xyzx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(other.e12345) * wedge.group2.xyz) + (float3(wedge.e3215) * other.group1.xyz) + (other.group2.yzx * wedge.group3.zxy) - (other.group2.zxy * wedge.group3.yzx)), other.e12345 * wedge.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345) * wedge.group3
        );
    }
}
public extension VersorOdd {
    public AntiCircleRotor project_orthogonally_onto(Dipole other) {
        Circle right_anti_dual = Circle(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125 */
            other.group2.xyz * float3(-1.0)
        );
        CircleRotor wedge = CircleRotor(
            /* e423, e431, e412 */
            float3(this.scalar) * right_anti_dual.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(this.scalar) * right_anti_dual.group1, 
            /* e235, e315, e125, e12345 */
            float4((float3(this.scalar) * right_anti_dual.group2.xyz), -(right_anti_dual.e423 * this.e15) - (right_anti_dual.e431 * this.e25) - (right_anti_dual.e412 * this.e35) - (right_anti_dual.e415 * this.e23) - (right_anti_dual.e425 * this.e31) - (right_anti_dual.e435 * this.e12) - (right_anti_dual.e321 * this.e45) - (right_anti_dual.e235 * this.e41) - (right_anti_dual.e315 * this.e42) - (right_anti_dual.e125 * this.e43))
        );
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(wedge.e12345) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(wedge.e12345) * other.group1, 
            /* e15, e25, e35, scalar */
            float4((float3(wedge.e12345) * other.group2.xyz), -(wedge.e423 * other.e15) - (wedge.e431 * other.e25) - (wedge.e412 * other.e35) - (wedge.e415 * other.e23) - (wedge.e425 * other.e31) - (wedge.e435 * other.e12) - (wedge.e321 * other.e45) - (wedge.e235 * other.e41) - (wedge.e315 * other.e42) - (wedge.e125 * other.e43))
        );
    }
}
public extension VersorOdd {
    public VersorOdd project_orthogonally_onto(DipoleInversion other) {
        AntiDipoleInversion right_anti_dual = AntiDipoleInversion(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e4 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group3 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        VersorEven wedge = VersorEven(
            /* e423, e431, e412, e12345 */
            (right_anti_dual.group3.zxyy * float4(this.group0.yzx, this.e4315)) + float4(float3(0.0), (right_anti_dual.e3 * this.e4125) + (right_anti_dual.e5 * this.e1234) - (right_anti_dual.e431 * this.e25) - (right_anti_dual.e412 * this.e35) - (right_anti_dual.e415 * this.e23) - (right_anti_dual.e425 * this.e31) - (right_anti_dual.e435 * this.e12) - (right_anti_dual.e321 * this.e45) - (right_anti_dual.e235 * this.e41) - (right_anti_dual.e315 * this.e42) - (right_anti_dual.e125 * this.e43)) + float4((right_anti_dual.group0.xyz * this.group0.www), right_anti_dual.e4 * this.e3215) + float4((right_anti_dual.group2.www * this.group1.xyz), right_anti_dual.e1 * this.e4235) - float4((right_anti_dual.group3.yzx * this.group0.zxy), right_anti_dual.e423 * this.e15), 
            /* e415, e425, e435, e321 */
            float4((right_anti_dual.e4 * this.e15) + (right_anti_dual.e5 * this.e41), (right_anti_dual.e4 * this.e25) + (right_anti_dual.e5 * this.e42), (right_anti_dual.e4 * this.e35) + (right_anti_dual.e5 * this.e43), -(right_anti_dual.e2 * this.e31) - (right_anti_dual.e3 * this.e12)) + (float4(this.scalar) * right_anti_dual.group1) - (right_anti_dual.group3.xyzx * this.group1.wwwx), 
            /* e235, e315, e125, e5 */
            float4(((float3(right_anti_dual.e5) * this.group1.xyz) + (float3(this.scalar) * right_anti_dual.group2.xyz) + (right_anti_dual.group3.yzx * this.group2.zxy) - (right_anti_dual.group3.zxy * this.group2.yzx)), right_anti_dual.e5 * this.scalar), 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * float4(right_anti_dual.group3.xyz, right_anti_dual.e4)
        );
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (other.group3.yzxy * float4(wedge.group0.zxy, wedge.e2)) + float4(float3(0.0), (other.e4125 * wedge.e3) + (other.e3215 * wedge.e4) - (other.e42 * wedge.e315) - (other.e43 * wedge.e125) - (other.e23 * wedge.e415) - (other.e31 * wedge.e425) - (other.e12 * wedge.e435) - (other.e45 * wedge.e321) - (other.e15 * wedge.e423) - (other.e25 * wedge.e431) - (other.e35 * wedge.e412)) + float4((other.group0.xyz * wedge.group0.www), other.e1234 * wedge.e5) + float4((other.group2.www * wedge.group1.xyz), other.e4235 * wedge.e1) - float4((other.group3.zxy * wedge.group0.yzx), other.e41 * wedge.e235), 
            /* e23, e31, e12, e45 */
            float4((other.e1234 * wedge.e235) + (other.e3215 * wedge.e423), (other.e1234 * wedge.e315) + (other.e3215 * wedge.e431), (other.e1234 * wedge.e125) + (other.e3215 * wedge.e412), -(other.e4315 * wedge.e425) - (other.e4125 * wedge.e435)) + (float4(wedge.e12345) * other.group1) - (other.group3.xyzx * wedge.group1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(other.e3215) * wedge.group1.xyz) + (float3(wedge.e12345) * other.group2.xyz) + (other.group3.zxy * wedge.group2.yzx) - (other.group3.yzx * wedge.group2.zxy)), other.e1234 * wedge.e12345), 
            /* e4235, e4315, e4125, e3215 */
            float4(wedge.e12345) * other.group3
        );
    }
}
public extension VersorOdd {
    public VersorOdd project_orthogonally_onto(DualNum other) {
        AntiDualNum right_anti_dual = AntiDualNum(
            /* e3215, scalar */
            other.group0.xy * float2(-1.0)
        );
        VersorOdd wedge = VersorOdd(
            /* e41, e42, e43, scalar */
            float4(right_anti_dual.scalar) * this.group0, 
            /* e23, e31, e12, e45 */
            float4(right_anti_dual.scalar) * this.group1, 
            /* e15, e25, e35, e1234 */
            float4(right_anti_dual.scalar) * this.group2, 
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual.group0.xy.yy, right_anti_dual.scalar, (right_anti_dual.e3215 * this.scalar) + (right_anti_dual.scalar * this.e3215)) * float4(this.group3.xyz, 1.0)
        );
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(other.group0.xy.yy, other.e12345, (other.e5 * wedge.e1234) + (other.e12345 * wedge.scalar)) * float4(wedge.group0.xyz, 1.0), 
            /* e23, e31, e12, e45 */
            float4(other.e12345) * wedge.group1, 
            /* e15, e25, e35, e1234 */
            float4(other.e12345) * wedge.group2, 
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345) * wedge.group3
        );
    }
}
public extension VersorOdd {
    public AntiCircleRotor project_orthogonally_onto(FlatPoint other) {
        AntiFlatPoint right_anti_dual = AntiFlatPoint(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        CircleRotor wedge = CircleRotor(
            /* e423, e431, e412 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), right_anti_dual.e321 * this.scalar), 
            /* e235, e315, e125, e12345 */
            float4((float3(this.scalar) * right_anti_dual.group0.xyz), -(right_anti_dual.e235 * this.e41) - (right_anti_dual.e315 * this.e42) - (right_anti_dual.e125 * this.e43) - (right_anti_dual.e321 * this.e45))
        );
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), wedge.e12345 * other.e45), 
            /* e15, e25, e35, scalar */
            float4((wedge.group2.www * other.group0.xyz), -(wedge.e423 * other.e15) - (wedge.e431 * other.e25) - (wedge.e412 * other.e35) - (wedge.e321 * other.e45))
        );
    }
}
public extension VersorOdd {
    public VersorOdd project_orthogonally_onto(Flector other) {
        AntiFlector right_anti_dual = AntiFlector(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        VersorEven wedge = VersorEven(
            /* e423, e431, e412, e12345 */
            (right_anti_dual.group1.zxyx * float4(this.group0.yzx, this.e4235)) + float4(float3(0.0), (right_anti_dual.e2 * this.e4315) + (right_anti_dual.e3 * this.e4125) + (right_anti_dual.e5 * this.e1234) - (right_anti_dual.e315 * this.e42) - (right_anti_dual.e125 * this.e43) - (right_anti_dual.e321 * this.e45)) - (this.group0.zxyx * float4(right_anti_dual.group1.yzx, right_anti_dual.e235)), 
            /* e415, e425, e435, e321 */
            (this.group0 * float4(right_anti_dual.group1.www, right_anti_dual.e321)) + float4(float3(0.0), -(right_anti_dual.e2 * this.e31) - (right_anti_dual.e3 * this.e12)) - (right_anti_dual.group1.xyzx * this.group1.wwwx), 
            /* e235, e315, e125, e5 */
            float4(((float3(right_anti_dual.e5) * this.group1.xyz) + (float3(this.scalar) * right_anti_dual.group0.xyz) + (right_anti_dual.group1.yzx * this.group2.zxy) - (right_anti_dual.group1.zxy * this.group2.yzx)), right_anti_dual.e5 * this.scalar), 
            /* e1, e2, e3, e4 */
            float4(float3(1.0), 0.0) * float4(this.group0.www, 0.0) * float4(right_anti_dual.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (other.group1.yzxx * float4(wedge.group0.zxy, wedge.e1)) + float4(float3(0.0), (other.e4315 * wedge.e2) + (other.e4125 * wedge.e3) + (other.e3215 * wedge.e4) - (other.e25 * wedge.e431) - (other.e35 * wedge.e412) - (other.e45 * wedge.e321)) - (wedge.group0.yzxx * float4(other.group1.zxy, other.e15)), 
            /* e23, e31, e12, e45 */
            (wedge.group0 * float4(other.group1.www, other.e45)) + float4(float3(0.0), -(other.e4315 * wedge.e425) - (other.e4125 * wedge.e435)) - (other.group1.xyzx * wedge.group1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4((float3(other.e3215) * wedge.group1.xyz), 0.0) + float4((float3(wedge.e12345) * other.group0.xyz), 0.0) + float4((other.group1.zxy * wedge.group2.yzx), 0.0) - float4((other.group1.yzx * wedge.group2.zxy), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(wedge.e12345) * other.group1
        );
    }
}
public extension VersorOdd {
    public AntiCircleRotor project_orthogonally_onto(Line other) {
        AntiLine right_anti_dual = AntiLine(
            /* e23, e31, e12 */
            other.group0.xyz, 
            /* e15, e25, e35 */
            other.group1.xyz
        );
        DipoleInversion wedge = DipoleInversion(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12, e45 */
            float4(float3(1.0), 0.0) * float4(right_anti_dual.group0.xyz, 0.0) * float4(this.group0.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e15, e25, e35, e1234 */
            float4((float3(this.scalar) * right_anti_dual.group1.xyz), -(right_anti_dual.e23 * this.e41) - (right_anti_dual.e31 * this.e42) - (right_anti_dual.e12 * this.e43)), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual.e23 * this.e45) + (right_anti_dual.e35 * this.e42), (right_anti_dual.e31 * this.e45) + (right_anti_dual.e15 * this.e43), (right_anti_dual.e12 * this.e45) + (right_anti_dual.e25 * this.e41), -(right_anti_dual.e31 * this.e25) - (right_anti_dual.e12 * this.e35) - (right_anti_dual.e15 * this.e23) - (right_anti_dual.e25 * this.e31) - (right_anti_dual.e35 * this.e12)) - float4((right_anti_dual.group1.xyz.yzx * this.group0.zxy), right_anti_dual.e23 * this.e15)
        );
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(wedge.e1234) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4((float3(wedge.e1234) * other.group1.xyz), -(wedge.e4235 * other.e415) - (wedge.e4315 * other.e425) - (wedge.e4125 * other.e435)), 
            /* e15, e25, e35, scalar */
            float4((wedge.e4125 * other.e315) + (wedge.e3215 * other.e415), (wedge.e4235 * other.e125) + (wedge.e3215 * other.e425), (wedge.e4315 * other.e235) + (wedge.e3215 * other.e435), -(wedge.e42 * other.e315) - (wedge.e43 * other.e125) - (wedge.e23 * other.e415) - (wedge.e31 * other.e425) - (wedge.e12 * other.e435)) - float4((other.group1.xyz.zxy * wedge.group3.yzx), wedge.e41 * other.e235)
        );
    }
}
public extension VersorOdd {
    public VersorOdd project_orthogonally_onto(Motor other) {
        AntiMotor right_anti_dual = AntiMotor(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        VersorOdd wedge = VersorOdd(
            /* e41, e42, e43, scalar */
            float4(right_anti_dual.scalar) * this.group0, 
            /* e23, e31, e12, e45 */
            float4(((float3(right_anti_dual.scalar) * this.group1.xyz) + (float3(this.scalar) * right_anti_dual.group0.xyz)), right_anti_dual.scalar * this.e45), 
            /* e15, e25, e35, e1234 */
            float4(right_anti_dual.e15 * this.scalar, right_anti_dual.e25 * this.scalar, right_anti_dual.e35 * this.scalar, -(right_anti_dual.e23 * this.e41) - (right_anti_dual.e31 * this.e42) - (right_anti_dual.e12 * this.e43)) + (float4(right_anti_dual.scalar) * this.group2), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual.e35 * this.e42, right_anti_dual.e15 * this.e43, right_anti_dual.e25 * this.e41, -(right_anti_dual.e31 * this.e25) - (right_anti_dual.e12 * this.e35) - (right_anti_dual.e15 * this.e23) - (right_anti_dual.e25 * this.e31) - (right_anti_dual.e35 * this.e12)) + (right_anti_dual.group0 * float4(this.group1.www, this.e3215)) + float4((right_anti_dual.group0.www * this.group3.xyz), right_anti_dual.e3215 * this.scalar) - float4((right_anti_dual.group1.yzx * this.group0.zxy), right_anti_dual.e23 * this.e15)
        );
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (other.group0 * float4(wedge.group2.www, wedge.scalar)) + float4(float3(0.0), -(other.e415 * wedge.e23) - (other.e425 * wedge.e31) - (other.e435 * wedge.e12) - (other.e235 * wedge.e41) - (other.e315 * wedge.e42) - (other.e125 * wedge.e43)) + float4((other.group0.www * wedge.group0.xyz), other.e5 * wedge.e1234), 
            /* e23, e31, e12, e45 */
            float4(other.e235 * wedge.e1234, other.e315 * wedge.e1234, other.e125 * wedge.e1234, -(other.e415 * wedge.e4235) - (other.e425 * wedge.e4315) - (other.e435 * wedge.e4125)) + (float4(other.e12345) * wedge.group1), 
            /* e15, e25, e35, e1234 */
            float4(((float3(other.e12345) * wedge.group2.xyz) + (float3(wedge.e3215) * other.group0.xyz) + (other.group1.yzx * wedge.group3.zxy) - (other.group1.zxy * wedge.group3.yzx)), other.e12345 * wedge.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345) * wedge.group3
        );
    }
}
public extension VersorOdd {
    public MultiVector project_orthogonally_onto(MultiVector other) {
        MultiVector right_anti_dual = MultiVector(
            /* scalar, e12345 */
            other.group0.xy.yx * float2(-1.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4(other.group9.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5 */
            other.e3215, 
            /* e15, e25, e35, e45 */
            float4(other.group8.xyz, other.e321 * -1.0), 
            /* e41, e42, e43 */
            other.group7.xyz, 
            /* e23, e31, e12 */
            other.group6.xyz, 
            /* e415, e425, e435, e321 */
            float4(other.group5.xyz, other.e45) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e423, e431, e412 */
            other.group4.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            other.group3.xyz * float3(-1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.group1.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234 */
            other.e4 * -1.0
        );
        MultiVector wedge = MultiVector(
            /* scalar, e12345 */
            float2(right_anti_dual.scalar * this.scalar, (right_anti_dual.e12345 * this.scalar) + (right_anti_dual.e1 * this.e4235) + (right_anti_dual.e2 * this.e4315) + (right_anti_dual.e3 * this.e4125) + (right_anti_dual.e4 * this.e3215) + (right_anti_dual.e5 * this.e1234) - (right_anti_dual.e415 * this.e23) - (right_anti_dual.e425 * this.e31) - (right_anti_dual.e435 * this.e12) - (right_anti_dual.e321 * this.e45) - (right_anti_dual.e423 * this.e15) - (right_anti_dual.e431 * this.e25) - (right_anti_dual.e412 * this.e35) - (right_anti_dual.e235 * this.e41) - (right_anti_dual.e315 * this.e42) - (right_anti_dual.e125 * this.e43)), 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * right_anti_dual.group1, 
            /* e5 */
            right_anti_dual.e5 * this.scalar, 
            /* e15, e25, e35, e45 */
            (float4(right_anti_dual.scalar) * float4(this.group2.xyz, this.e45)) + (float4(this.scalar) * right_anti_dual.group3), 
            /* e41, e42, e43 */
            (float3(right_anti_dual.scalar) * this.group0.xyz) + (float3(this.scalar) * right_anti_dual.group4.xyz), 
            /* e23, e31, e12 */
            (float3(right_anti_dual.scalar) * this.group1.xyz) + (float3(this.scalar) * right_anti_dual.group5.xyz), 
            /* e415, e425, e435, e321 */
            float4((right_anti_dual.e5 * this.e41) + (right_anti_dual.e415 * this.scalar), (right_anti_dual.e5 * this.e42) + (right_anti_dual.e425 * this.scalar), (right_anti_dual.e5 * this.e43) + (right_anti_dual.e435 * this.scalar), -(right_anti_dual.e2 * this.e31) - (right_anti_dual.e3 * this.e12)) + float4((right_anti_dual.group1.www * this.group2.xyz), right_anti_dual.e321 * this.scalar) - (right_anti_dual.group1.xyzx * this.group1.wwwx), 
            /* e423, e431, e412 */
            (float3(right_anti_dual.e4) * this.group1.xyz) + (float3(this.scalar) * right_anti_dual.group7.xyz) + (right_anti_dual.group1.zxy * this.group0.yzx) - (right_anti_dual.group1.yzx * this.group0.zxy), 
            /* e235, e315, e125 */
            (float3(right_anti_dual.e5) * this.group1.xyz) + (float3(this.scalar) * right_anti_dual.group8.xyz) + (right_anti_dual.group1.yzx * this.group2.zxy) - (right_anti_dual.group1.zxy * this.group2.yzx), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual.e35 * this.e42) + (right_anti_dual.e45 * this.e23) + (right_anti_dual.e23 * this.e45) + (right_anti_dual.e4235 * this.scalar), (right_anti_dual.e15 * this.e43) + (right_anti_dual.e45 * this.e31) + (right_anti_dual.e31 * this.e45) + (right_anti_dual.e4315 * this.scalar), (right_anti_dual.e25 * this.e41) + (right_anti_dual.e45 * this.e12) + (right_anti_dual.e12 * this.e45) + (right_anti_dual.e4125 * this.scalar), -(right_anti_dual.e15 * this.e23) - (right_anti_dual.e25 * this.e31) - (right_anti_dual.e35 * this.e12) - (right_anti_dual.e12 * this.e35)) + (float4(right_anti_dual.scalar) * this.group3) + float4((right_anti_dual.group4.xyz.yzx * this.group2.zxy), right_anti_dual.e3215 * this.scalar) - (this.group2.yzxx * float4(right_anti_dual.group4.xyz.zxy, right_anti_dual.e23)) - float4((right_anti_dual.group3.yzx * this.group0.zxy), right_anti_dual.e31 * this.e25), 
            /* e1234 */
            (right_anti_dual.scalar * this.e1234) + (right_anti_dual.e1234 * this.scalar) - (right_anti_dual.e41 * this.e23) - (right_anti_dual.e42 * this.e31) - (right_anti_dual.e43 * this.e12) - (right_anti_dual.e23 * this.e41) - (right_anti_dual.e31 * this.e42) - (right_anti_dual.e12 * this.e43)
        );
        return MultiVector(
            /* scalar, e12345 */
            float2((other.scalar * wedge.e12345) + (other.e12345 * wedge.scalar) + (other.e1 * wedge.e4235) + (other.e2 * wedge.e4315) + (other.e3 * wedge.e4125) + (other.e4 * wedge.e3215) + (other.e5 * wedge.e1234) + (other.e4235 * wedge.e1) + (other.e4315 * wedge.e2) + (other.e4125 * wedge.e3) + (other.e3215 * wedge.e4) + (other.e1234 * wedge.e5) - (other.e15 * wedge.e423) - (other.e25 * wedge.e431) - (other.e35 * wedge.e412) - (other.e45 * wedge.e321) - (other.e41 * wedge.e235) - (other.e42 * wedge.e315) - (other.e43 * wedge.e125) - (other.e23 * wedge.e415) - (other.e31 * wedge.e425) - (other.e12 * wedge.e435) - (other.e415 * wedge.e23) - (other.e425 * wedge.e31) - (other.e435 * wedge.e12) - (other.e321 * wedge.e45) - (other.e423 * wedge.e15) - (other.e431 * wedge.e25) - (other.e412 * wedge.e35) - (other.e235 * wedge.e41) - (other.e315 * wedge.e42) - (other.e125 * wedge.e43), other.e12345 * wedge.e12345), 
            /* e1, e2, e3, e4 */
            float4((other.e415 * wedge.e321) + (other.e321 * wedge.e415) + (other.e4315 * wedge.e12) + (other.e1234 * wedge.e15), (other.e425 * wedge.e321) + (other.e321 * wedge.e425) + (other.e4125 * wedge.e23) + (other.e1234 * wedge.e25), (other.e435 * wedge.e321) + (other.e321 * wedge.e435) + (other.e4235 * wedge.e31) + (other.e1234 * wedge.e35), -(other.e45 * wedge.e1234) - (other.e415 * wedge.e423) - (other.e425 * wedge.e431) - (other.e435 * wedge.e412)) + (float4(other.e12345) * wedge.group1) + (float4(wedge.e12345) * other.group1) + float4((other.group4.xyz * wedge.group9.www), other.e4235 * wedge.e41) + float4((other.group5.xyz.yzx * wedge.group9.zxy), other.e4315 * wedge.e42) + float4((other.group7.xyz.zxy * wedge.group8.xyz.yzx), other.e4125 * wedge.e43) + float4((other.group8.xyz.yzx * wedge.group7.xyz.zxy), other.e1234 * wedge.e45) - (wedge.group9.yzxx * float4(other.group5.xyz.zxy, other.e41)) - float4((float3(wedge.e1234) * other.group3.xyz), other.e412 * wedge.e435) - float4((wedge.group4.xyz * other.group9.www), other.e423 * wedge.e415) - float4((other.group7.xyz.yzx * wedge.group8.xyz.zxy), other.e42 * wedge.e4315) - float4((other.group8.xyz.zxy * wedge.group7.xyz.yzx), other.e43 * wedge.e4125) - float4((wedge.group5.xyz.yzx * other.group9.zxy), other.e431 * wedge.e425), 
            /* e5 */
            (other.e12345 * wedge.e5) + (other.e5 * wedge.e12345) + (other.e15 * wedge.e4235) + (other.e25 * wedge.e4315) + (other.e35 * wedge.e4125) + (other.e45 * wedge.e3215) - (other.e415 * wedge.e235) - (other.e425 * wedge.e315) - (other.e435 * wedge.e125) - (other.e235 * wedge.e415) - (other.e315 * wedge.e425) - (other.e125 * wedge.e435) - (other.e4235 * wedge.e15) - (other.e4315 * wedge.e25) - (other.e4125 * wedge.e35) - (other.e3215 * wedge.e45), 
            /* e15, e25, e35, e45 */
            float4((other.e415 * wedge.e3215) + (other.e315 * wedge.e4125) + (other.e4125 * wedge.e315) + (other.e3215 * wedge.e415), (other.e425 * wedge.e3215) + (other.e125 * wedge.e4235) + (other.e4235 * wedge.e125) + (other.e3215 * wedge.e425), (other.e435 * wedge.e3215) + (other.e235 * wedge.e4315) + (other.e4315 * wedge.e235) + (other.e3215 * wedge.e435), -(other.e435 * wedge.e4125) - (other.e4235 * wedge.e415) - (other.e4315 * wedge.e425) - (other.e4125 * wedge.e435)) + (float4(other.e12345) * wedge.group3) + (float4(wedge.e12345) * other.group3) - (wedge.group9.yzxx * float4(other.group8.xyz.zxy, other.e415)) - float4((wedge.group8.xyz.zxy * other.group9.yzx), other.e425 * wedge.e4315), 
            /* e41, e42, e43 */
            (float3(other.e12345) * wedge.group4.xyz) + (float3(other.e1234) * wedge.group6.xyz) + (float3(wedge.e12345) * other.group4.xyz) + (float3(wedge.e1234) * other.group6.xyz) + (other.group7.xyz.zxy * wedge.group9.yzx) + (wedge.group7.xyz.zxy * other.group9.yzx) - (other.group7.xyz.yzx * wedge.group9.zxy) - (wedge.group7.xyz.yzx * other.group9.zxy), 
            /* e23, e31, e12 */
            (float3(other.e12345) * wedge.group5.xyz) + (float3(other.e3215) * wedge.group7.xyz) + (float3(other.e1234) * wedge.group8.xyz) + (float3(wedge.e12345) * other.group5.xyz) + (float3(wedge.e3215) * other.group7.xyz) + (float3(wedge.e1234) * other.group8.xyz) - (float3(other.e321) * wedge.group9.xyz) - (float3(wedge.e321) * other.group9.xyz), 
            /* e415, e425, e435, e321 */
            (float4(other.e12345) * wedge.group6) + (float4(wedge.e12345) * other.group6) + float4((other.group9.zxy * wedge.group9.yzx), other.e1234 * wedge.e3215) - (other.group9.yzxw * float4(wedge.group9.zxy, wedge.e1234)), 
            /* e423, e431, e412 */
            (float3(other.e12345) * wedge.group7.xyz) + (float3(other.e1234) * wedge.group9.xyz) + (float3(wedge.e12345) * other.group7.xyz) - (float3(wedge.e1234) * other.group9.xyz), 
            /* e235, e315, e125 */
            (float3(other.e12345) * wedge.group8.xyz) + (float3(wedge.e12345) * other.group8.xyz) + (float3(wedge.e3215) * other.group9.xyz) - (float3(other.e3215) * wedge.group9.xyz), 
            /* e4235, e4315, e4125, e3215 */
            (float4(other.e12345) * wedge.group9) + (float4(wedge.e12345) * other.group9), 
            /* e1234 */
            (other.e12345 * wedge.e1234) + (other.e1234 * wedge.e12345)
        );
    }
}
public extension VersorOdd {
    public VersorOdd project_orthogonally_onto(Plane other) {
        AntiPlane right_anti_dual = AntiPlane(
            /* e1, e2, e3, e5 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        VersorEven wedge = VersorEven(
            /* e423, e431, e412, e12345 */
            float4(right_anti_dual.e2 * this.e43 * -1.0, right_anti_dual.e3 * this.e41 * -1.0, right_anti_dual.e1 * this.e42 * -1.0, (right_anti_dual.e2 * this.e4315) + (right_anti_dual.e3 * this.e4125) + (right_anti_dual.e5 * this.e1234)) + (right_anti_dual.group0.zxyx * float4(this.group0.yzx, this.e4235)), 
            /* e415, e425, e435, e321 */
            float4(right_anti_dual.e5 * this.e41, right_anti_dual.e5 * this.e42, right_anti_dual.e5 * this.e43, -(right_anti_dual.e2 * this.e31) - (right_anti_dual.e3 * this.e12)) - (right_anti_dual.group0.xyzx * this.group1.wwwx), 
            /* e235, e315, e125, e5 */
            float4(((float3(right_anti_dual.e5) * this.group1.xyz) + (right_anti_dual.group0.yzx * this.group2.zxy) - (right_anti_dual.group0.zxy * this.group2.yzx)), right_anti_dual.e5 * this.scalar), 
            /* e1, e2, e3, e4 */
            float4(float3(1.0), 0.0) * float4(this.group0.www, 0.0) * float4(right_anti_dual.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(other.e4125 * wedge.e431 * -1.0, other.e4235 * wedge.e412 * -1.0, other.e4315 * wedge.e423 * -1.0, (other.e4315 * wedge.e2) + (other.e4125 * wedge.e3) + (other.e3215 * wedge.e4)) + (other.group0.yzxx * float4(wedge.group0.zxy, wedge.e1)), 
            /* e23, e31, e12, e45 */
            float4(other.e3215 * wedge.e423, other.e3215 * wedge.e431, other.e3215 * wedge.e412, -(other.e4315 * wedge.e425) - (other.e4125 * wedge.e435)) - (other.group0.xyzx * wedge.group1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4((float3(other.e3215) * wedge.group1.xyz), 0.0) + float4((other.group0.zxy * wedge.group2.yzx), 0.0) - float4((other.group0.yzx * wedge.group2.zxy), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(wedge.e12345) * other.group0
        );
    }
}
public extension VersorOdd {
    public Scalar project_orthogonally_onto(RoundPoint other) {
        Sphere right_anti_dual = Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234 */
            other.e4 * -1.0
        );
        Sphere wedge = Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar) * right_anti_dual.group0, 
            /* e1234 */
            right_anti_dual.e1234 * this.scalar
        );
        return Scalar(
            /* scalar */
            (other.e1 * wedge.e4235) + (other.e2 * wedge.e4315) + (other.e3 * wedge.e4125) + (other.e4 * wedge.e3215) + (other.e5 * wedge.e1234)
        );
    }
}
public extension VersorOdd {
    public Scalar project_orthogonally_onto(Scalar other) {
        return Scalar(
            /* scalar */
            pow(other.scalar, 2) * this.scalar
        );
    }
}
public extension VersorOdd {
    public VersorOdd project_orthogonally_onto(Sphere other) {
        RoundPoint right_anti_dual = RoundPoint(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5 */
            other.e3215
        );
        VersorEven wedge = VersorEven(
            /* e423, e431, e412, e12345 */
            float4(right_anti_dual.e2 * this.e43 * -1.0, right_anti_dual.e3 * this.e41 * -1.0, right_anti_dual.e1 * this.e42 * -1.0, (right_anti_dual.e3 * this.e4125) + (right_anti_dual.e4 * this.e3215) + (right_anti_dual.e5 * this.e1234)) + (right_anti_dual.group0.zxyx * float4(this.group0.yzx, this.e4235)) + (right_anti_dual.group0.wwwy * float4(this.group1.xyz, this.e4315)), 
            /* e415, e425, e435, e321 */
            float4((right_anti_dual.e4 * this.e15) + (right_anti_dual.e5 * this.e41), (right_anti_dual.e4 * this.e25) + (right_anti_dual.e5 * this.e42), (right_anti_dual.e4 * this.e35) + (right_anti_dual.e5 * this.e43), -(right_anti_dual.e2 * this.e31) - (right_anti_dual.e3 * this.e12)) - (right_anti_dual.group0.xyzx * this.group1.wwwx), 
            /* e235, e315, e125, e5 */
            float4(((float3(right_anti_dual.e5) * this.group1.xyz) + (right_anti_dual.group0.yzx * this.group2.zxy) - (right_anti_dual.group0.zxy * this.group2.yzx)), right_anti_dual.e5 * this.scalar), 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * right_anti_dual.group0
        );
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(other.e4125 * wedge.e431 * -1.0, other.e4235 * wedge.e412 * -1.0, other.e4315 * wedge.e423 * -1.0, (other.e4125 * wedge.e3) + (other.e3215 * wedge.e4) + (other.e1234 * wedge.e5)) + (other.group0.yzxx * float4(wedge.group0.zxy, wedge.e1)) + float4((float3(other.e1234) * wedge.group1.xyz), other.e4315 * wedge.e2), 
            /* e23, e31, e12, e45 */
            float4((other.e3215 * wedge.e423) + (other.e1234 * wedge.e235), (other.e3215 * wedge.e431) + (other.e1234 * wedge.e315), (other.e3215 * wedge.e412) + (other.e1234 * wedge.e125), -(other.e4315 * wedge.e425) - (other.e4125 * wedge.e435)) - (other.group0.xyzx * wedge.group1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(other.e3215) * wedge.group1.xyz) + (other.group0.zxy * wedge.group2.yzx) - (other.group0.yzx * wedge.group2.zxy)), other.e1234 * wedge.e12345), 
            /* e4235, e4315, e4125, e3215 */
            float4(wedge.e12345) * other.group0
        );
    }
}
public extension VersorOdd {
    public VersorOdd project_orthogonally_onto(VersorEven other) {
        VersorOdd right_anti_dual = VersorOdd(
            /* e41, e42, e43, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e1234 */
            float4(other.group2.xyz, other.e4) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.group3.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0)
        );
        VersorOdd wedge = VersorOdd(
            /* e41, e42, e43, scalar */
            float4(((float3(right_anti_dual.scalar) * this.group0.xyz) + (float3(this.scalar) * right_anti_dual.group0.xyz)), right_anti_dual.scalar * this.scalar), 
            /* e23, e31, e12, e45 */
            (float4(right_anti_dual.scalar) * this.group1) + (float4(this.scalar) * right_anti_dual.group1), 
            /* e15, e25, e35, e1234 */
            (float4(right_anti_dual.scalar) * this.group2) + (float4(this.scalar) * right_anti_dual.group2) + float4(float3(0.0), -(right_anti_dual.e41 * this.e23) - (right_anti_dual.e42 * this.e31) - (right_anti_dual.e43 * this.e12) - (right_anti_dual.e23 * this.e41) - (right_anti_dual.e31 * this.e42) - (right_anti_dual.e12 * this.e43)), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual.e23 * this.e45) + (right_anti_dual.e45 * this.e23) + (right_anti_dual.e35 * this.e42) + (right_anti_dual.e4235 * this.scalar), (right_anti_dual.e31 * this.e45) + (right_anti_dual.e45 * this.e31) + (right_anti_dual.e15 * this.e43) + (right_anti_dual.e4315 * this.scalar), (right_anti_dual.e12 * this.e45) + (right_anti_dual.e45 * this.e12) + (right_anti_dual.e25 * this.e41) + (right_anti_dual.e4125 * this.scalar), -(right_anti_dual.e12 * this.e35) - (right_anti_dual.e15 * this.e23) - (right_anti_dual.e25 * this.e31) - (right_anti_dual.e35 * this.e12)) + (right_anti_dual.group0.yzxw * float4(this.group2.zxy, this.e3215)) + float4((right_anti_dual.group0.www * this.group3.xyz), right_anti_dual.e3215 * this.scalar) - (this.group2.yzxx * float4(right_anti_dual.group0.zxy, right_anti_dual.e23)) - float4((right_anti_dual.group2.yzx * this.group0.zxy), right_anti_dual.e31 * this.e25)
        );
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (other.group0.zxyw * float4(wedge.group3.yzx, wedge.scalar)) + float4(float3(0.0), (other.e2 * wedge.e4315) + (other.e3 * wedge.e4125) + (other.e4 * wedge.e3215) - (other.e431 * wedge.e25) - (other.e412 * wedge.e35) - (other.e415 * wedge.e23) - (other.e425 * wedge.e31) - (other.e435 * wedge.e12) - (other.e321 * wedge.e45) - (other.e235 * wedge.e41) - (other.e315 * wedge.e42) - (other.e125 * wedge.e43)) + float4((other.group0.www * wedge.group0.xyz), other.e5 * wedge.e1234) + float4((wedge.group2.www * other.group1.xyz), other.e1 * wedge.e4235) - (other.group0.yzxx * float4(wedge.group3.zxy, wedge.e15)), 
            /* e23, e31, e12, e45 */
            float4((other.e12345 * wedge.e23) + (other.e235 * wedge.e1234), (other.e12345 * wedge.e31) + (other.e315 * wedge.e1234), (other.e12345 * wedge.e12) + (other.e125 * wedge.e1234), -(other.e425 * wedge.e4315) - (other.e435 * wedge.e4125)) + (other.group0 * float4(wedge.group3.www, wedge.e45)) - (other.group1.wwwx * wedge.group3.xyzx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(other.e12345) * wedge.group2.xyz) + (float3(wedge.e3215) * other.group1.xyz) + (other.group2.yzx * wedge.group3.zxy) - (other.group2.zxy * wedge.group3.yzx)), other.e12345 * wedge.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345) * wedge.group3
        );
    }
}
public extension VersorOdd {
    public VersorOdd project_orthogonally_onto(VersorOdd other) {
        VersorEven right_anti_dual = VersorEven(
            /* e423, e431, e412, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            float4(other.group2.xyz, other.e3215) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4(other.group3.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0)
        );
        VersorEven wedge = VersorEven(
            /* e423, e431, e412, e12345 */
            (float4(this.scalar) * right_anti_dual.group0) + (right_anti_dual.group3.wwwx * float4(this.group1.xyz, this.e4235)) + float4(float3(0.0), (right_anti_dual.e2 * this.e4315) + (right_anti_dual.e3 * this.e4125) + (right_anti_dual.e4 * this.e3215) - (right_anti_dual.e431 * this.e25) - (right_anti_dual.e412 * this.e35) - (right_anti_dual.e415 * this.e23) - (right_anti_dual.e425 * this.e31) - (right_anti_dual.e435 * this.e12) - (right_anti_dual.e321 * this.e45) - (right_anti_dual.e235 * this.e41) - (right_anti_dual.e315 * this.e42) - (right_anti_dual.e125 * this.e43)) + float4((right_anti_dual.group3.zxy * this.group0.yzx), right_anti_dual.e5 * this.e1234) - float4((right_anti_dual.group3.yzx * this.group0.zxy), right_anti_dual.e423 * this.e15), 
            /* e415, e425, e435, e321 */
            float4((right_anti_dual.e5 * this.e41) + (right_anti_dual.e4 * this.e15), (right_anti_dual.e5 * this.e42) + (right_anti_dual.e4 * this.e25), (right_anti_dual.e5 * this.e43) + (right_anti_dual.e4 * this.e35), -(right_anti_dual.e2 * this.e31) - (right_anti_dual.e3 * this.e12)) + (float4(this.scalar) * right_anti_dual.group1) - (right_anti_dual.group3.xyzx * this.group1.wwwx), 
            /* e235, e315, e125, e5 */
            float4(((float3(right_anti_dual.e5) * this.group1.xyz) + (float3(this.scalar) * right_anti_dual.group2.xyz) + (right_anti_dual.group3.yzx * this.group2.zxy) - (right_anti_dual.group3.zxy * this.group2.yzx)), right_anti_dual.e5 * this.scalar), 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * right_anti_dual.group3
        );
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (wedge.group0.zxyw * float4(other.group3.yzx, other.scalar)) + float4(float3(0.0), (wedge.e2 * other.e4315) + (wedge.e3 * other.e4125) + (wedge.e4 * other.e3215) - (wedge.e431 * other.e25) - (wedge.e412 * other.e35) - (wedge.e415 * other.e23) - (wedge.e425 * other.e31) - (wedge.e435 * other.e12) - (wedge.e321 * other.e45) - (wedge.e235 * other.e41) - (wedge.e315 * other.e42) - (wedge.e125 * other.e43)) + float4((wedge.group0.www * other.group0.xyz), wedge.e5 * other.e1234) + float4((other.group2.www * wedge.group1.xyz), wedge.e1 * other.e4235) - (wedge.group0.yzxx * float4(other.group3.zxy, other.e15)), 
            /* e23, e31, e12, e45 */
            float4((wedge.e12345 * other.e23) + (wedge.e235 * other.e1234), (wedge.e12345 * other.e31) + (wedge.e315 * other.e1234), (wedge.e12345 * other.e12) + (wedge.e125 * other.e1234), -(wedge.e425 * other.e4315) - (wedge.e435 * other.e4125)) + (wedge.group0 * float4(other.group3.www, other.e45)) - (wedge.group1.wwwx * other.group3.xyzx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(wedge.e12345) * other.group2.xyz) + (float3(other.e3215) * wedge.group1.xyz) + (wedge.group2.yzx * other.group3.zxy) - (wedge.group2.zxy * other.group3.yzx)), wedge.e12345 * other.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(wedge.e12345) * other.group3
        );
    }
}

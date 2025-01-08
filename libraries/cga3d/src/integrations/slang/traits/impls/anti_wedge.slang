implementing cga3d;
using traits;
public extension AntiCircleRotor {
    // Fancy infix trick (first half)
    public AntiWedgeInfixPartial<AntiCircleRotor> operator /(AntiWedgeInfix rhs) {
        return AntiWedgeInfixPartial<AntiCircleRotor>(this);
    }
}
public extension AntiWedgeInfixPartial<AntiCircleRotor> {
    // Fancy infix trick (second half)
    public Scalar operator /(AntiDipoleInversion rhs) {
        return this.a.anti_wedge(rhs);
    }
}
public extension AntiCircleRotor {
    public Scalar anti_wedge(AntiDipoleInversion other) {
        return Scalar(
            /* scalar */
            -(this.e41 * other.e235) - (this.e42 * other.e315) - (this.e43 * other.e125) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435) - (this.e45 * other.e321) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412)
        );
    }
}
public extension AntiCircleRotor {
    public AntiPlane anti_wedge(AntiDualNum other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(other.e3215) * float4(this.group0.xyz, this.e45)
        );
    }
}
public extension AntiCircleRotor {
    public Scalar anti_wedge(AntiFlatPoint other) {
        return Scalar(
            /* scalar */
            -(this.e41 * other.e235) - (this.e42 * other.e315) - (this.e43 * other.e125) - (this.e45 * other.e321)
        );
    }
}
public extension AntiCircleRotor {
    public Scalar anti_wedge(AntiFlector other) {
        return Scalar(
            /* scalar */
            -(this.e41 * other.e235) - (this.e42 * other.e315) - (this.e43 * other.e125) - (this.e45 * other.e321)
        );
    }
}
public extension AntiCircleRotor {
    public AntiPlane anti_wedge(AntiMotor other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(other.e3215) * float4(this.group0.xyz, this.e45)
        );
    }
}
public extension AntiCircleRotor {
    public AntiCircleRotor anti_wedge(AntiScalar other) {
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(other.e12345) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(other.e12345) * this.group1, 
            /* e15, e25, e35, scalar */
            float4(other.e12345) * this.group2
        );
    }
}
public extension AntiCircleRotor {
    public Scalar anti_wedge(Circle other) {
        return Scalar(
            /* scalar */
            -(this.e41 * other.e235) - (this.e42 * other.e315) - (this.e43 * other.e125) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435) - (this.e45 * other.e321) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412)
        );
    }
}
public extension AntiCircleRotor {
    public AntiCircleRotor anti_wedge(CircleRotor other) {
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(other.e12345) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(other.e12345) * this.group1, 
            /* e15, e25, e35, scalar */
            float4((float3(other.e12345) * this.group2.xyz), (this.scalar * other.e12345) - (this.e41 * other.e235) - (this.e42 * other.e315) - (this.e43 * other.e125) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435) - (this.e45 * other.e321) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412))
        );
    }
}
public extension AntiCircleRotor {
    public RoundPoint anti_wedge(DipoleInversion other) {
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4((this.e41 * other.e3215) + (this.e31 * other.e4125), (this.e42 * other.e3215) + (this.e12 * other.e4235), (this.e43 * other.e3215) + (this.e23 * other.e4315), -(this.e43 * other.e4125) - (this.e45 * other.e1234)) - (other.group3.yzxx * float4(this.group1.zxy, this.e41)) - float4((other.group2.www * this.group2.xyz), this.e42 * other.e4315), 
            /* e5 */
            (this.e45 * other.e3215) + (this.e15 * other.e4235) + (this.e25 * other.e4315) + (this.e35 * other.e4125)
        );
    }
}
public extension AntiCircleRotor {
    public AntiCircleRotor anti_wedge(DualNum other) {
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(other.e12345) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(other.e12345) * this.group1, 
            /* e15, e25, e35, scalar */
            float4(other.e12345) * this.group2
        );
    }
}
public extension AntiCircleRotor {
    public RoundPoint anti_wedge(Flector other) {
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4((this.e41 * other.e3215) + (this.e31 * other.e4125), (this.e42 * other.e3215) + (this.e12 * other.e4235), (this.e43 * other.e3215) + (this.e23 * other.e4315), -(this.e42 * other.e4315) - (this.e43 * other.e4125)) - (other.group1.yzxx * float4(this.group1.zxy, this.e41)), 
            /* e5 */
            (this.e45 * other.e3215) + (this.e15 * other.e4235) + (this.e25 * other.e4315) + (this.e35 * other.e4125)
        );
    }
}
public extension AntiCircleRotor {
    public Scalar anti_wedge(Line other) {
        return Scalar(
            /* scalar */
            -(this.e41 * other.e235) - (this.e42 * other.e315) - (this.e43 * other.e125) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435)
        );
    }
}
public extension AntiCircleRotor {
    public AntiCircleRotor anti_wedge(Motor other) {
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(other.e12345) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(other.e12345) * this.group1, 
            /* e15, e25, e35, scalar */
            float4((float3(other.e12345) * this.group2.xyz), (this.scalar * other.e12345) - (this.e41 * other.e235) - (this.e42 * other.e315) - (this.e43 * other.e125) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435))
        );
    }
}
public extension AntiCircleRotor {
    public MultiVector anti_wedge(MultiVector other) {
        return MultiVector(
            /* scalar, e12345 */
            float2((this.scalar * other.e12345) - (this.e41 * other.e235) - (this.e42 * other.e315) - (this.e43 * other.e125) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435) - (this.e45 * other.e321) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412), 0.0), 
            /* e1, e2, e3, e4 */
            float4((this.e41 * other.e3215) + (this.e31 * other.e4125), (this.e42 * other.e3215) + (this.e12 * other.e4235), (this.e43 * other.e3215) + (this.e23 * other.e4315), -(this.e43 * other.e4125) - (this.e45 * other.e1234)) - (other.group9.yzxx * float4(this.group1.zxy, this.e41)) - float4((float3(other.e1234) * this.group2.xyz), this.e42 * other.e4315), 
            /* e5 */
            (this.e45 * other.e3215) + (this.e15 * other.e4235) + (this.e25 * other.e4315) + (this.e35 * other.e4125), 
            /* e15, e25, e35, e45 */
            float4(other.e12345) * float4(this.group2.xyz, this.e45), 
            /* e41, e42, e43 */
            float3(other.e12345) * this.group0.xyz, 
            /* e23, e31, e12 */
            float3(other.e12345) * this.group1.xyz, 
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
public extension AntiCircleRotor {
    public RoundPoint anti_wedge(Plane other) {
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4((this.e41 * other.e3215) + (this.e31 * other.e4125), (this.e42 * other.e3215) + (this.e12 * other.e4235), (this.e43 * other.e3215) + (this.e23 * other.e4315), -(this.e42 * other.e4315) - (this.e43 * other.e4125)) - (other.group0.yzxx * float4(this.group1.zxy, this.e41)), 
            /* e5 */
            (this.e45 * other.e3215) + (this.e15 * other.e4235) + (this.e25 * other.e4315) + (this.e35 * other.e4125)
        );
    }
}
public extension AntiCircleRotor {
    public RoundPoint anti_wedge(Sphere other) {
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4((this.e41 * other.e3215) + (this.e31 * other.e4125), (this.e42 * other.e3215) + (this.e12 * other.e4235), (this.e43 * other.e3215) + (this.e23 * other.e4315), -(this.e43 * other.e4125) - (this.e45 * other.e1234)) - (other.group0.yzxx * float4(this.group1.zxy, this.e41)) - float4((float3(other.e1234) * this.group2.xyz), this.e42 * other.e4315), 
            /* e5 */
            (this.e45 * other.e3215) + (this.e15 * other.e4235) + (this.e25 * other.e4315) + (this.e35 * other.e4125)
        );
    }
}
public extension AntiCircleRotor {
    public AntiCircleRotor anti_wedge(VersorEven other) {
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(other.e12345) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(other.e12345) * this.group1, 
            /* e15, e25, e35, scalar */
            float4((float3(other.e12345) * this.group2.xyz), (this.scalar * other.e12345) - (this.e41 * other.e235) - (this.e42 * other.e315) - (this.e43 * other.e125) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435) - (this.e45 * other.e321) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412))
        );
    }
}
public extension AntiCircleRotor {
    public RoundPoint anti_wedge(VersorOdd other) {
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4((this.e41 * other.e3215) + (this.e31 * other.e4125), (this.e42 * other.e3215) + (this.e12 * other.e4235), (this.e43 * other.e3215) + (this.e23 * other.e4315), -(this.e43 * other.e4125) - (this.e45 * other.e1234)) - (other.group3.yzxx * float4(this.group1.zxy, this.e41)) - float4((other.group2.www * this.group2.xyz), this.e42 * other.e4315), 
            /* e5 */
            (this.e45 * other.e3215) + (this.e15 * other.e4235) + (this.e25 * other.e4315) + (this.e35 * other.e4125)
        );
    }
}
public extension AntiDipoleInversion {
    // Fancy infix trick (first half)
    public AntiWedgeInfixPartial<AntiDipoleInversion> operator /(AntiWedgeInfix rhs) {
        return AntiWedgeInfixPartial<AntiDipoleInversion>(this);
    }
}
public extension AntiWedgeInfixPartial<AntiDipoleInversion> {
    // Fancy infix trick (second half)
    public Scalar operator /(AntiCircleRotor rhs) {
        return this.a.anti_wedge(rhs);
    }
}
public extension AntiDipoleInversion {
    public Scalar anti_wedge(AntiCircleRotor other) {
        return Scalar(
            /* scalar */
            -(other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e45 * this.e321) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412)
        );
    }
}
public extension AntiDipoleInversion {
    public RoundPoint anti_wedge(AntiDipoleInversion other) {
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4((other.e412 * this.e315) + (other.e415 * this.e321) + (other.e321 * this.e415) + (other.e315 * this.e412), (other.e423 * this.e125) + (other.e425 * this.e321) + (other.e321 * this.e425) + (other.e125 * this.e423), (other.e431 * this.e235) + (other.e435 * this.e321) + (other.e321 * this.e435) + (other.e235 * this.e431), -(other.e412 * this.e435) - (other.e415 * this.e423) - (other.e425 * this.e431) - (other.e435 * this.e412)) - float4((other.group0.xyz.yzx * this.group2.zxy), other.e423 * this.e415) - float4((this.group0.xyz.yzx * other.group2.zxy), other.e431 * this.e425), 
            /* e5 */
            -(other.e415 * this.e235) - (other.e425 * this.e315) - (other.e435 * this.e125) - (other.e235 * this.e415) - (other.e315 * this.e425) - (other.e125 * this.e435)
        );
    }
}
public extension AntiDipoleInversion {
    public AntiMotor anti_wedge(AntiDualNum other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(other.e3215) * float4(this.group0.xyz, this.e4), 
            /* e15, e25, e35, e3215 */
            float4(other.group0.xy.xx, other.e3215, 0.0) * float4(float3(1.0), 0.0) * float4(this.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
public extension AntiDipoleInversion {
    public AntiPlane anti_wedge(AntiFlatPoint other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4((this.e412 * other.e315) + (this.e415 * other.e321), (this.e423 * other.e125) + (this.e425 * other.e321), (this.e431 * other.e235) + (this.e435 * other.e321), -(this.e425 * other.e315) - (this.e435 * other.e125)) - (other.group0.zxyx * float4(this.group0.xyz.yzx, this.e415))
        );
    }
}
public extension AntiDipoleInversion {
    public AntiPlane anti_wedge(AntiFlector other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4((this.e412 * other.e315) + (this.e415 * other.e321), (this.e423 * other.e125) + (this.e425 * other.e321), (this.e431 * other.e235) + (this.e435 * other.e321), -(this.e425 * other.e315) - (this.e435 * other.e125)) - (other.group0.zxyx * float4(this.group0.xyz.yzx, this.e415))
        );
    }
}
public extension AntiDipoleInversion {
    public Scalar anti_wedge(AntiLine other) {
        return Scalar(
            /* scalar */
            -(this.e423 * other.e15) - (this.e431 * other.e25) - (this.e412 * other.e35) - (this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12)
        );
    }
}
public extension AntiDipoleInversion {
    public AntiMotor anti_wedge(AntiMotor other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((float3(other.e3215) * this.group0.xyz), (this.e4 * other.e3215) - (this.e423 * other.e15) - (this.e431 * other.e25) - (this.e412 * other.e35) - (this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12)), 
            /* e15, e25, e35, e3215 */
            float4(float3(1.0), 0.0) * float4(other.group1.www, 0.0) * float4(this.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
public extension AntiDipoleInversion {
    public AntiDipoleInversion anti_wedge(AntiScalar other) {
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(other.e12345) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(other.e12345) * this.group1, 
            /* e235, e315, e125, e4 */
            float4(other.e12345) * this.group2, 
            /* e1, e2, e3, e5 */
            float4(other.e12345) * this.group3
        );
    }
}
public extension AntiDipoleInversion {
    public RoundPoint anti_wedge(Circle other) {
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4((this.e412 * other.e315) + (this.e415 * other.e321) + (this.e321 * other.e415) + (this.e315 * other.e412), (this.e423 * other.e125) + (this.e425 * other.e321) + (this.e321 * other.e425) + (this.e125 * other.e423), (this.e431 * other.e235) + (this.e435 * other.e321) + (this.e321 * other.e435) + (this.e235 * other.e431), -(this.e412 * other.e435) - (this.e415 * other.e423) - (this.e425 * other.e431) - (this.e435 * other.e412)) - float4((this.group0.xyz.yzx * other.group2.xyz.zxy), this.e423 * other.e415) - float4((other.group0.xyz.yzx * this.group2.zxy), this.e431 * other.e425), 
            /* e5 */
            -(this.e415 * other.e235) - (this.e425 * other.e315) - (this.e435 * other.e125) - (this.e235 * other.e415) - (this.e315 * other.e425) - (this.e125 * other.e435)
        );
    }
}
public extension AntiDipoleInversion {
    public AntiDipoleInversion anti_wedge(CircleRotor other) {
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(other.e12345) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(other.e12345) * this.group1, 
            /* e235, e315, e125, e4 */
            float4((float3(other.e12345) * this.group2.xyz), (this.e4 * other.e12345) - (this.e423 * other.e415) - (this.e431 * other.e425) - (this.e412 * other.e435) - (this.e415 * other.e423) - (this.e425 * other.e431) - (this.e435 * other.e412)), 
            /* e1, e2, e3, e5 */
            float4((this.e415 * other.e321) + (this.e321 * other.e415) + (this.e315 * other.e412) + (this.e1 * other.e12345), (this.e425 * other.e321) + (this.e321 * other.e425) + (this.e125 * other.e423) + (this.e2 * other.e12345), (this.e435 * other.e321) + (this.e321 * other.e435) + (this.e235 * other.e431) + (this.e3 * other.e12345), -(this.e435 * other.e125) - (this.e235 * other.e415) - (this.e315 * other.e425) - (this.e125 * other.e435)) + (other.group2.yzxw * float4(this.group0.xyz.zxy, this.e5)) - (other.group2.zxyx * float4(this.group0.xyz.yzx, this.e415)) - float4((other.group0.xyz.yzx * this.group2.zxy), this.e425 * other.e315)
        );
    }
}
public extension AntiDipoleInversion {
    public Scalar anti_wedge(Dipole other) {
        return Scalar(
            /* scalar */
            -(this.e423 * other.e15) - (this.e431 * other.e25) - (this.e412 * other.e35) - (this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12) - (this.e321 * other.e45) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43)
        );
    }
}
public extension AntiDipoleInversion {
    public AntiCircleRotor anti_wedge(DipoleInversion other) {
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (float3(other.e1234) * this.group1.xyz) + (this.group0.xyz.zxy * other.group3.yzx) - (this.group0.xyz.yzx * other.group3.zxy), 
            /* e23, e31, e12, e45 */
            float4((this.e423 * other.e3215) + (this.e235 * other.e1234), (this.e431 * other.e3215) + (this.e315 * other.e1234), (this.e412 * other.e3215) + (this.e125 * other.e1234), -(this.e425 * other.e4315) - (this.e435 * other.e4125)) - (this.group1.wwwx * other.group3.xyzx), 
            /* e15, e25, e35, scalar */
            (float4(other.e3215) * float4(this.group1.xyz, this.e4)) + (other.group3.zxyx * float4(this.group2.yzx, this.e1)) + float4(float3(0.0), (this.e2 * other.e4315) + (this.e3 * other.e4125) + (this.e5 * other.e1234) - (this.e431 * other.e25) - (this.e412 * other.e35) - (this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12) - (this.e321 * other.e45) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43)) - float4((this.group2.zxy * other.group3.yzx), this.e423 * other.e15)
        );
    }
}
public extension AntiDipoleInversion {
    public AntiDipoleInversion anti_wedge(DualNum other) {
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(other.e12345) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(other.e12345) * this.group1, 
            /* e235, e315, e125, e4 */
            float4(other.e12345) * this.group2, 
            /* e1, e2, e3, e5 */
            float4(other.e12345) * this.group3
        );
    }
}
public extension AntiDipoleInversion {
    public Scalar anti_wedge(FlatPoint other) {
        return Scalar(
            /* scalar */
            -(this.e423 * other.e15) - (this.e431 * other.e25) - (this.e412 * other.e35) - (this.e321 * other.e45)
        );
    }
}
public extension AntiDipoleInversion {
    public AntiCircleRotor anti_wedge(Flector other) {
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (this.group0.xyz.zxy * other.group1.yzx) - (this.group0.xyz.yzx * other.group1.zxy), 
            /* e23, e31, e12, e45 */
            float4(this.e423 * other.e3215, this.e431 * other.e3215, this.e412 * other.e3215, -(this.e425 * other.e4315) - (this.e435 * other.e4125)) - (this.group1.wwwx * other.group1.xyzx), 
            /* e15, e25, e35, scalar */
            (float4(other.e3215) * float4(this.group1.xyz, this.e4)) + (other.group1.zxyx * float4(this.group2.yzx, this.e1)) + float4(float3(0.0), (this.e2 * other.e4315) + (this.e3 * other.e4125) - (this.e431 * other.e25) - (this.e412 * other.e35) - (this.e321 * other.e45)) - float4((this.group2.zxy * other.group1.yzx), this.e423 * other.e15)
        );
    }
}
public extension AntiDipoleInversion {
    public RoundPoint anti_wedge(Line other) {
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4((this.e412 * other.e315) + (this.e321 * other.e415), (this.e423 * other.e125) + (this.e321 * other.e425), (this.e431 * other.e235) + (this.e321 * other.e435), -(this.e431 * other.e425) - (this.e412 * other.e435)) - float4((this.group0.xyz.yzx * other.group1.xyz.zxy), this.e423 * other.e415), 
            /* e5 */
            -(this.e415 * other.e235) - (this.e425 * other.e315) - (this.e435 * other.e125) - (this.e235 * other.e415) - (this.e315 * other.e425) - (this.e125 * other.e435)
        );
    }
}
public extension AntiDipoleInversion {
    public AntiDipoleInversion anti_wedge(Motor other) {
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(other.e12345) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(other.e12345) * this.group1, 
            /* e235, e315, e125, e4 */
            float4((float3(other.e12345) * this.group2.xyz), (this.e4 * other.e12345) - (this.e423 * other.e415) - (this.e431 * other.e425) - (this.e412 * other.e435)), 
            /* e1, e2, e3, e5 */
            float4((this.e321 * other.e415) + (this.e1 * other.e12345), (this.e321 * other.e425) + (this.e2 * other.e12345), (this.e321 * other.e435) + (this.e3 * other.e12345), -(this.e425 * other.e315) - (this.e435 * other.e125) - (this.e235 * other.e415) - (this.e315 * other.e425) - (this.e125 * other.e435)) + float4((this.group0.xyz.zxy * other.group1.yzx), this.e5 * other.e12345) - (other.group1.zxyx * float4(this.group0.xyz.yzx, this.e415))
        );
    }
}
public extension AntiDipoleInversion {
    public MultiVector anti_wedge(MultiVector other) {
        return MultiVector(
            /* scalar, e12345 */
            float2((this.e4 * other.e3215) + (this.e1 * other.e4235) + (this.e2 * other.e4315) + (this.e3 * other.e4125) + (this.e5 * other.e1234) - (this.e423 * other.e15) - (this.e431 * other.e25) - (this.e412 * other.e35) - (this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12) - (this.e321 * other.e45) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43), 0.0), 
            /* e1, e2, e3, e4 */
            float4((this.e412 * other.e315) + (this.e415 * other.e321) + (this.e321 * other.e415) + (this.e315 * other.e412), (this.e423 * other.e125) + (this.e425 * other.e321) + (this.e321 * other.e425) + (this.e125 * other.e423), (this.e431 * other.e235) + (this.e435 * other.e321) + (this.e321 * other.e435) + (this.e235 * other.e431), -(this.e412 * other.e435) - (this.e415 * other.e423) - (this.e425 * other.e431) - (this.e435 * other.e412)) + (float4(other.e12345) * float4(this.group3.xyz, this.e4)) - float4((this.group0.xyz.yzx * other.group8.xyz.zxy), this.e423 * other.e415) - float4((other.group7.xyz.yzx * this.group2.zxy), this.e431 * other.e425), 
            /* e5 */
            (this.e5 * other.e12345) - (this.e415 * other.e235) - (this.e425 * other.e315) - (this.e435 * other.e125) - (this.e235 * other.e415) - (this.e315 * other.e425) - (this.e125 * other.e435), 
            /* e15, e25, e35, e45 */
            float4((this.e415 * other.e3215) + (this.e315 * other.e4125), (this.e425 * other.e3215) + (this.e125 * other.e4235), (this.e435 * other.e3215) + (this.e235 * other.e4315), -(this.e425 * other.e4315) - (this.e435 * other.e4125)) - (other.group9.yzxx * float4(this.group2.zxy, this.e415)), 
            /* e41, e42, e43 */
            (float3(other.e1234) * this.group1.xyz) + (this.group0.xyz.zxy * other.group9.yzx) - (this.group0.xyz.yzx * other.group9.zxy), 
            /* e23, e31, e12 */
            (float3(other.e3215) * this.group0.xyz) + (float3(other.e1234) * this.group2.xyz) - (float3(this.e321) * other.group9.xyz), 
            /* e415, e425, e435, e321 */
            float4(other.e12345) * this.group1, 
            /* e423, e431, e412 */
            float3(other.e12345) * this.group0.xyz, 
            /* e235, e315, e125 */
            float3(other.e12345) * this.group2.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
public extension AntiDipoleInversion {
    public AntiCircleRotor anti_wedge(Plane other) {
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (this.group0.xyz.zxy * other.group0.yzx) - (this.group0.xyz.yzx * other.group0.zxy), 
            /* e23, e31, e12, e45 */
            float4(this.e423 * other.e3215, this.e431 * other.e3215, this.e412 * other.e3215, -(this.e425 * other.e4315) - (this.e435 * other.e4125)) - (this.group1.wwwx * other.group0.xyzx), 
            /* e15, e25, e35, scalar */
            float4(this.e125 * other.e4315 * -1.0, this.e235 * other.e4125 * -1.0, this.e315 * other.e4235 * -1.0, (this.e2 * other.e4315) + (this.e3 * other.e4125)) + (float4(other.e3215) * float4(this.group1.xyz, this.e4)) + (other.group0.zxyx * float4(this.group2.yzx, this.e1))
        );
    }
}
public extension AntiDipoleInversion {
    public AntiCircleRotor anti_wedge(Sphere other) {
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (float3(other.e1234) * this.group1.xyz) + (this.group0.xyz.zxy * other.group0.yzx) - (this.group0.xyz.yzx * other.group0.zxy), 
            /* e23, e31, e12, e45 */
            float4((this.e423 * other.e3215) + (this.e235 * other.e1234), (this.e431 * other.e3215) + (this.e315 * other.e1234), (this.e412 * other.e3215) + (this.e125 * other.e1234), -(this.e425 * other.e4315) - (this.e435 * other.e4125)) - (this.group1.wwwx * other.group0.xyzx), 
            /* e15, e25, e35, scalar */
            float4(this.e125 * other.e4315 * -1.0, this.e235 * other.e4125 * -1.0, this.e315 * other.e4235 * -1.0, (this.e2 * other.e4315) + (this.e3 * other.e4125) + (this.e5 * other.e1234)) + (float4(other.e3215) * float4(this.group1.xyz, this.e4)) + (other.group0.zxyx * float4(this.group2.yzx, this.e1))
        );
    }
}
public extension AntiDipoleInversion {
    public AntiDipoleInversion anti_wedge(VersorEven other) {
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(other.e12345) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(other.e12345) * this.group1, 
            /* e235, e315, e125, e4 */
            float4((float3(other.e12345) * this.group2.xyz), (this.e4 * other.e12345) - (this.e423 * other.e415) - (this.e431 * other.e425) - (this.e412 * other.e435) - (this.e415 * other.e423) - (this.e425 * other.e431) - (this.e435 * other.e412)), 
            /* e1, e2, e3, e5 */
            float4((this.e415 * other.e321) + (this.e321 * other.e415) + (this.e315 * other.e412) + (this.e1 * other.e12345), (this.e425 * other.e321) + (this.e321 * other.e425) + (this.e125 * other.e423) + (this.e2 * other.e12345), (this.e435 * other.e321) + (this.e321 * other.e435) + (this.e235 * other.e431) + (this.e3 * other.e12345), -(this.e435 * other.e125) - (this.e235 * other.e415) - (this.e315 * other.e425) - (this.e125 * other.e435)) + float4((this.group0.xyz.zxy * other.group2.yzx), this.e5 * other.e12345) - (other.group2.zxyx * float4(this.group0.xyz.yzx, this.e415)) - float4((this.group2.zxy * other.group0.yzx), this.e425 * other.e315)
        );
    }
}
public extension AntiDipoleInversion {
    public AntiCircleRotor anti_wedge(VersorOdd other) {
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (float3(other.e1234) * this.group1.xyz) + (this.group0.xyz.zxy * other.group3.yzx) - (this.group0.xyz.yzx * other.group3.zxy), 
            /* e23, e31, e12, e45 */
            float4((this.e423 * other.e3215) + (this.e235 * other.e1234), (this.e431 * other.e3215) + (this.e315 * other.e1234), (this.e412 * other.e3215) + (this.e125 * other.e1234), -(this.e425 * other.e4315) - (this.e435 * other.e4125)) - (this.group1.wwwx * other.group3.xyzx), 
            /* e15, e25, e35, scalar */
            (float4(other.e3215) * float4(this.group1.xyz, this.e4)) + (other.group3.zxyx * float4(this.group2.yzx, this.e1)) + float4(float3(0.0), (this.e2 * other.e4315) + (this.e3 * other.e4125) + (this.e5 * other.e1234) - (this.e431 * other.e25) - (this.e412 * other.e35) - (this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12) - (this.e321 * other.e45) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43)) - float4((this.group2.zxy * other.group3.yzx), this.e423 * other.e15)
        );
    }
}
public extension AntiDualNum {
    // Fancy infix trick (first half)
    public AntiWedgeInfixPartial<AntiDualNum> operator /(AntiWedgeInfix rhs) {
        return AntiWedgeInfixPartial<AntiDualNum>(this);
    }
}
public extension AntiWedgeInfixPartial<AntiDualNum> {
    // Fancy infix trick (second half)
    public AntiPlane operator /(AntiCircleRotor rhs) {
        return this.a.anti_wedge(rhs);
    }
}
public extension AntiDualNum {
    public AntiPlane anti_wedge(AntiCircleRotor other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(this.e3215) * float4(other.group0.xyz, other.e45) * float4(-1.0)
        );
    }
}
public extension AntiDualNum {
    public AntiMotor anti_wedge(AntiDipoleInversion other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(this.e3215) * float4(other.group0.xyz, other.e4), 
            /* e15, e25, e35, e3215 */
            float4(this.group0.xy.xx, this.e3215, 0.0) * float4(float3(1.0), 0.0) * float4(other.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
public extension AntiDualNum {
    public AntiDualNum anti_wedge(AntiScalar other) {
        return AntiDualNum(
            /* e3215, scalar */
            float2(other.e12345) * this.group0.xy
        );
    }
}
public extension AntiDualNum {
    public AntiLine anti_wedge(Circle other) {
        return AntiLine(
            /* e23, e31, e12 */
            float3(this.e3215) * other.group0.xyz, 
            /* e15, e25, e35 */
            float3(this.e3215) * other.group1.xyz
        );
    }
}
public extension AntiDualNum {
    public AntiMotor anti_wedge(CircleRotor other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(this.group0.xy.xx, this.e3215, this.scalar) * float4(other.group0.xyz, other.e12345), 
            /* e15, e25, e35, e3215 */
            float4(this.e3215) * float4(other.group1.xyz, other.e12345)
        );
    }
}
public extension AntiDualNum {
    public AntiPlane anti_wedge(Dipole other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(this.e3215) * float4(other.group0.xyz, other.e45) * float4(-1.0)
        );
    }
}
public extension AntiDualNum {
    public AntiFlector anti_wedge(DipoleInversion other) {
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(this.e3215) * float4(other.group3.xyz, other.e1234) * float4(-1.0), 
            /* e1, e2, e3, e5 */
            float4(this.e3215) * float4(other.group0.xyz, other.e45) * float4(-1.0)
        );
    }
}
public extension AntiDualNum {
    public AntiDualNum anti_wedge(DualNum other) {
        return AntiDualNum(
            /* e3215, scalar */
            float2(other.e12345) * this.group0.xy
        );
    }
}
public extension AntiDualNum {
    public DualNum anti_wedge(FlatPoint other) {
        return DualNum(
            /* e5, e12345 */
            float2(this.e3215 * other.e45, 1.0) * float2(-1.0, 0.0)
        );
    }
}
public extension AntiDualNum {
    public AntiFlector anti_wedge(Flector other) {
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(this.group0.xy.xx, this.e3215, 0.0) * float4(float3(1.0), 0.0) * float4(other.group1.xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), this.e3215 * other.e45 * -1.0)
        );
    }
}
public extension AntiDualNum {
    public FlatPoint anti_wedge(Line other) {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(this.group0.xy.xx, this.e3215, 0.0) * float4(float3(1.0), 0.0) * float4(other.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
public extension AntiDualNum {
    public AntiMotor anti_wedge(Motor other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(float3(0.0), this.scalar * other.e12345), 
            /* e15, e25, e35, e3215 */
            float4(this.e3215) * other.group0
        );
    }
}
public extension AntiDualNum {
    public MultiVector anti_wedge(MultiVector other) {
        return MultiVector(
            /* scalar, e12345 */
            float2((this.e3215 * other.e4) + (this.scalar * other.e12345), 0.0), 
            /* e1, e2, e3, e4 */
            float4(this.group0.xy.xx, this.e3215, 0.0) * float4(float3(1.0), 0.0) * float4(other.group4.xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0), 
            /* e5 */
            this.e3215 * other.e45 * -1.0, 
            /* e15, e25, e35, e45 */
            float4(this.group0.xy.xx, this.e3215, 0.0) * float4(float3(1.0), 0.0) * float4(other.group6.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(this.e3215) * other.group7.xyz, 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), this.e3215 * other.e1234 * -1.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(this.e3215) * other.group9.xyz * float3(-1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), this.e3215 * other.e12345), 
            /* e1234 */
            0.0
        );
    }
}
public extension AntiDualNum {
    public AntiFlatPoint anti_wedge(Plane other) {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(this.group0.xy.xx, this.e3215, 0.0) * float4(float3(1.0), 0.0) * float4(other.group0.xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0)
        );
    }
}
public extension AntiDualNum {
    public Scalar anti_wedge(RoundPoint other) {
        return Scalar(
            /* scalar */
            this.e3215 * other.e4
        );
    }
}
public extension AntiDualNum {
    public AntiFlatPoint anti_wedge(Sphere other) {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(this.e3215) * float4(other.group0.xyz, other.e1234) * float4(-1.0)
        );
    }
}
public extension AntiDualNum {
    public AntiMotor anti_wedge(VersorEven other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(this.group0.xy.xx, this.e3215, (this.e3215 * other.e4) + (this.scalar * other.e12345)) * float4(other.group0.xyz, 1.0), 
            /* e15, e25, e35, e3215 */
            float4(this.e3215) * float4(other.group1.xyz, other.e12345)
        );
    }
}
public extension AntiDualNum {
    public AntiFlector anti_wedge(VersorOdd other) {
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(this.e3215) * float4(other.group3.xyz, other.e1234) * float4(-1.0), 
            /* e1, e2, e3, e5 */
            float4(this.e3215) * float4(other.group0.xyz, other.e45) * float4(-1.0)
        );
    }
}
public extension AntiFlatPoint {
    // Fancy infix trick (first half)
    public AntiWedgeInfixPartial<AntiFlatPoint> operator /(AntiWedgeInfix rhs) {
        return AntiWedgeInfixPartial<AntiFlatPoint>(this);
    }
}
public extension AntiWedgeInfixPartial<AntiFlatPoint> {
    // Fancy infix trick (second half)
    public Scalar operator /(AntiCircleRotor rhs) {
        return this.a.anti_wedge(rhs);
    }
}
public extension AntiFlatPoint {
    public Scalar anti_wedge(AntiCircleRotor other) {
        return Scalar(
            /* scalar */
            -(other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (other.e45 * this.e321)
        );
    }
}
public extension AntiFlatPoint {
    public AntiPlane anti_wedge(AntiDipoleInversion other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4((other.e412 * this.e315) + (other.e415 * this.e321), (other.e423 * this.e125) + (other.e425 * this.e321), (other.e431 * this.e235) + (other.e435 * this.e321), -(other.e425 * this.e315) - (other.e435 * this.e125)) - (this.group0.zxyx * float4(other.group0.xyz.yzx, other.e415))
        );
    }
}
public extension AntiFlatPoint {
    public AntiFlatPoint anti_wedge(AntiScalar other) {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(other.e12345) * this.group0
        );
    }
}
public extension AntiFlatPoint {
    public AntiPlane anti_wedge(Circle other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4((this.e315 * other.e412) + (this.e321 * other.e415), (this.e125 * other.e423) + (this.e321 * other.e425), (this.e235 * other.e431) + (this.e321 * other.e435), -(this.e315 * other.e425) - (this.e125 * other.e435)) - float4((other.group0.xyz.yzx * this.group0.zxy), this.e235 * other.e415)
        );
    }
}
public extension AntiFlatPoint {
    public AntiFlector anti_wedge(CircleRotor other) {
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(other.e12345) * this.group0, 
            /* e1, e2, e3, e5 */
            float4((this.e315 * other.e412) + (this.e321 * other.e415), (this.e125 * other.e423) + (this.e321 * other.e425), (this.e235 * other.e431) + (this.e321 * other.e435), -(this.e315 * other.e425) - (this.e125 * other.e435)) - float4((other.group0.xyz.yzx * this.group0.zxy), this.e235 * other.e415)
        );
    }
}
public extension AntiFlatPoint {
    public Scalar anti_wedge(Dipole other) {
        return Scalar(
            /* scalar */
            -(this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43) - (this.e321 * other.e45)
        );
    }
}
public extension AntiFlatPoint {
    public AntiMotor anti_wedge(DipoleInversion other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(this.e235 * other.e1234, this.e315 * other.e1234, this.e125 * other.e1234, -(this.e315 * other.e42) - (this.e125 * other.e43) - (this.e321 * other.e45)) - float4((this.group0.www * other.group3.xyz), this.e235 * other.e41), 
            /* e15, e25, e35, e3215 */
            float4(((this.group0.yzx * other.group3.zxy) - (this.group0.zxy * other.group3.yzx)), 0.0)
        );
    }
}
public extension AntiFlatPoint {
    public AntiFlatPoint anti_wedge(DualNum other) {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(other.e12345) * this.group0
        );
    }
}
public extension AntiFlatPoint {
    public Scalar anti_wedge(FlatPoint other) {
        return Scalar(
            /* scalar */
            this.e321 * other.e45 * -1.0
        );
    }
}
public extension AntiFlatPoint {
    public AntiMotor anti_wedge(Flector other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(this.e321) * float4(other.group1.xyz, other.e45) * float4(-1.0), 
            /* e15, e25, e35, e3215 */
            float4(((this.group0.yzx * other.group1.zxy) - (this.group0.zxy * other.group1.yzx)), 0.0)
        );
    }
}
public extension AntiFlatPoint {
    public AntiPlane anti_wedge(Line other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4((float3(this.e321) * other.group0.xyz), -(this.e235 * other.e415) - (this.e315 * other.e425) - (this.e125 * other.e435))
        );
    }
}
public extension AntiFlatPoint {
    public AntiFlector anti_wedge(Motor other) {
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(other.e12345) * this.group0, 
            /* e1, e2, e3, e5 */
            float4((this.group0.www * other.group0.xyz), -(this.e235 * other.e415) - (this.e315 * other.e425) - (this.e125 * other.e435))
        );
    }
}
public extension AntiFlatPoint {
    public MultiVector anti_wedge(MultiVector other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(-(this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43) - (this.e321 * other.e45), 0.0), 
            /* e1, e2, e3, e4 */
            float4((float3(this.e321) * other.group6.xyz), 0.0) + float4((other.group7.xyz.zxy * this.group0.yzx), 0.0) - float4((other.group7.xyz.yzx * this.group0.zxy), 0.0), 
            /* e5 */
            -(this.e235 * other.e415) - (this.e315 * other.e425) - (this.e125 * other.e435), 
            /* e15, e25, e35, e45 */
            float4(((this.group0.yzx * other.group9.zxy) - (this.group0.zxy * other.group9.yzx)), 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            (float3(other.e1234) * this.group0.xyz) - (float3(this.e321) * other.group9.xyz), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), this.e321 * other.e12345), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(other.e12345) * this.group0.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
public extension AntiFlatPoint {
    public AntiLine anti_wedge(Plane other) {
        return AntiLine(
            /* e23, e31, e12 */
            float3(this.e321) * other.group0.xyz * float3(-1.0), 
            /* e15, e25, e35 */
            (this.group0.yzx * other.group0.zxy) - (this.group0.zxy * other.group0.yzx)
        );
    }
}
public extension AntiFlatPoint {
    public AntiLine anti_wedge(Sphere other) {
        return AntiLine(
            /* e23, e31, e12 */
            (float3(other.e1234) * this.group0.xyz) - (float3(this.e321) * other.group0.xyz), 
            /* e15, e25, e35 */
            (this.group0.yzx * other.group0.zxy) - (this.group0.zxy * other.group0.yzx)
        );
    }
}
public extension AntiFlatPoint {
    public AntiFlector anti_wedge(VersorEven other) {
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(other.e12345) * this.group0, 
            /* e1, e2, e3, e5 */
            float4((this.e315 * other.e412) + (this.e321 * other.e415), (this.e125 * other.e423) + (this.e321 * other.e425), (this.e235 * other.e431) + (this.e321 * other.e435), -(this.e315 * other.e425) - (this.e125 * other.e435)) - (this.group0.zxyx * float4(other.group0.yzx, other.e415))
        );
    }
}
public extension AntiFlatPoint {
    public AntiMotor anti_wedge(VersorOdd other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(this.e235 * other.e1234, this.e315 * other.e1234, this.e125 * other.e1234, -(this.e315 * other.e42) - (this.e125 * other.e43) - (this.e321 * other.e45)) - (this.group0.wwwx * float4(other.group3.xyz, other.e41)), 
            /* e15, e25, e35, e3215 */
            float4(((this.group0.yzx * other.group3.zxy) - (this.group0.zxy * other.group3.yzx)), 0.0)
        );
    }
}
public extension AntiFlector {
    // Fancy infix trick (first half)
    public AntiWedgeInfixPartial<AntiFlector> operator /(AntiWedgeInfix rhs) {
        return AntiWedgeInfixPartial<AntiFlector>(this);
    }
}
public extension AntiWedgeInfixPartial<AntiFlector> {
    // Fancy infix trick (second half)
    public Scalar operator /(AntiCircleRotor rhs) {
        return this.a.anti_wedge(rhs);
    }
}
public extension AntiFlector {
    public Scalar anti_wedge(AntiCircleRotor other) {
        return Scalar(
            /* scalar */
            -(other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (other.e45 * this.e321)
        );
    }
}
public extension AntiFlector {
    public AntiPlane anti_wedge(AntiDipoleInversion other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4((other.e412 * this.e315) + (other.e415 * this.e321), (other.e423 * this.e125) + (other.e425 * this.e321), (other.e431 * this.e235) + (other.e435 * this.e321), -(other.e425 * this.e315) - (other.e435 * this.e125)) - (this.group0.zxyx * float4(other.group0.xyz.yzx, other.e415))
        );
    }
}
public extension AntiFlector {
    public AntiFlector anti_wedge(AntiScalar other) {
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(other.e12345) * this.group0, 
            /* e1, e2, e3, e5 */
            float4(other.e12345) * this.group1
        );
    }
}
public extension AntiFlector {
    public AntiPlane anti_wedge(Circle other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4((this.e315 * other.e412) + (this.e321 * other.e415), (this.e125 * other.e423) + (this.e321 * other.e425), (this.e235 * other.e431) + (this.e321 * other.e435), -(this.e315 * other.e425) - (this.e125 * other.e435)) - float4((other.group0.xyz.yzx * this.group0.zxy), this.e235 * other.e415)
        );
    }
}
public extension AntiFlector {
    public AntiFlector anti_wedge(CircleRotor other) {
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(other.e12345) * this.group0, 
            /* e1, e2, e3, e5 */
            float4((this.e321 * other.e415) + (this.e1 * other.e12345), (this.e321 * other.e425) + (this.e2 * other.e12345), (this.e321 * other.e435) + (this.e3 * other.e12345), -(this.e315 * other.e425) - (this.e125 * other.e435)) + float4((other.group0.xyz.zxy * this.group0.yzx), this.e5 * other.e12345) - float4((other.group0.xyz.yzx * this.group0.zxy), this.e235 * other.e415)
        );
    }
}
public extension AntiFlector {
    public Scalar anti_wedge(Dipole other) {
        return Scalar(
            /* scalar */
            -(this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43) - (this.e321 * other.e45)
        );
    }
}
public extension AntiFlector {
    public AntiMotor anti_wedge(DipoleInversion other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(float3(0.0), (this.e2 * other.e4315) + (this.e3 * other.e4125) + (this.e5 * other.e1234) - (this.e315 * other.e42) - (this.e125 * other.e43) - (this.e321 * other.e45)) + float4((other.group2.www * this.group0.xyz), this.e1 * other.e4235) - float4((this.group0.www * other.group3.xyz), this.e235 * other.e41), 
            /* e15, e25, e35, e3215 */
            float4(((this.group0.yzx * other.group3.zxy) - (this.group0.zxy * other.group3.yzx)), 0.0)
        );
    }
}
public extension AntiFlector {
    public AntiFlector anti_wedge(DualNum other) {
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(other.e12345) * this.group0, 
            /* e1, e2, e3, e5 */
            float4(other.e12345) * this.group1
        );
    }
}
public extension AntiFlector {
    public Scalar anti_wedge(FlatPoint other) {
        return Scalar(
            /* scalar */
            this.e321 * other.e45 * -1.0
        );
    }
}
public extension AntiFlector {
    public AntiMotor anti_wedge(Flector other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((this.group0.www * other.group1.xyz * float3(-1.0)), (this.e1 * other.e4235) + (this.e2 * other.e4315) + (this.e3 * other.e4125) - (this.e321 * other.e45)), 
            /* e15, e25, e35, e3215 */
            float4(((this.group0.yzx * other.group1.zxy) - (this.group0.zxy * other.group1.yzx)), 0.0)
        );
    }
}
public extension AntiFlector {
    public AntiPlane anti_wedge(Line other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4((float3(this.e321) * other.group0.xyz), -(this.e235 * other.e415) - (this.e315 * other.e425) - (this.e125 * other.e435))
        );
    }
}
public extension AntiFlector {
    public AntiFlector anti_wedge(Motor other) {
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(other.e12345) * this.group0, 
            /* e1, e2, e3, e5 */
            float4(this.e1 * other.e12345, this.e2 * other.e12345, this.e3 * other.e12345, -(this.e235 * other.e415) - (this.e315 * other.e425) - (this.e125 * other.e435)) + (other.group0 * float4(this.group0.www, this.e5))
        );
    }
}
public extension AntiFlector {
    public MultiVector anti_wedge(MultiVector other) {
        return MultiVector(
            /* scalar, e12345 */
            float2((this.e1 * other.e4235) + (this.e2 * other.e4315) + (this.e3 * other.e4125) + (this.e5 * other.e1234) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43) - (this.e321 * other.e45), 0.0), 
            /* e1, e2, e3, e4 */
            float4((float3(this.e321) * other.group6.xyz), 0.0) + float4((float3(other.e12345) * this.group1.xyz), 0.0) + float4((other.group7.xyz.zxy * this.group0.yzx), 0.0) - float4((other.group7.xyz.yzx * this.group0.zxy), 0.0), 
            /* e5 */
            (this.e5 * other.e12345) - (this.e235 * other.e415) - (this.e315 * other.e425) - (this.e125 * other.e435), 
            /* e15, e25, e35, e45 */
            float4(((this.group0.yzx * other.group9.zxy) - (this.group0.zxy * other.group9.yzx)), 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            (float3(other.e1234) * this.group0.xyz) - (float3(this.e321) * other.group9.xyz), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), this.e321 * other.e12345), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(other.e12345) * this.group0.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
public extension AntiFlector {
    public AntiMotor anti_wedge(Plane other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((this.group0.www * other.group0.xyz * float3(-1.0)), (this.e1 * other.e4235) + (this.e2 * other.e4315) + (this.e3 * other.e4125)), 
            /* e15, e25, e35, e3215 */
            float4(((this.group0.yzx * other.group0.zxy) - (this.group0.zxy * other.group0.yzx)), 0.0)
        );
    }
}
public extension AntiFlector {
    public AntiMotor anti_wedge(Sphere other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(this.e321 * other.e4235 * -1.0, this.e321 * other.e4315 * -1.0, this.e321 * other.e4125 * -1.0, (this.e2 * other.e4315) + (this.e3 * other.e4125) + (this.e5 * other.e1234)) + float4((float3(other.e1234) * this.group0.xyz), this.e1 * other.e4235), 
            /* e15, e25, e35, e3215 */
            float4(((this.group0.yzx * other.group0.zxy) - (this.group0.zxy * other.group0.yzx)), 0.0)
        );
    }
}
public extension AntiFlector {
    public AntiFlector anti_wedge(VersorEven other) {
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(other.e12345) * this.group0, 
            /* e1, e2, e3, e5 */
            float4((this.e321 * other.e415) + (this.e1 * other.e12345), (this.e321 * other.e425) + (this.e2 * other.e12345), (this.e321 * other.e435) + (this.e3 * other.e12345), -(this.e315 * other.e425) - (this.e125 * other.e435)) + (other.group0.zxyw * float4(this.group0.yzx, this.e5)) - (this.group0.zxyx * float4(other.group0.yzx, other.e415))
        );
    }
}
public extension AntiFlector {
    public AntiMotor anti_wedge(VersorOdd other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(float3(0.0), (this.e2 * other.e4315) + (this.e3 * other.e4125) + (this.e5 * other.e1234) - (this.e315 * other.e42) - (this.e125 * other.e43) - (this.e321 * other.e45)) + float4((other.group2.www * this.group0.xyz), this.e1 * other.e4235) - (this.group0.wwwx * float4(other.group3.xyz, other.e41)), 
            /* e15, e25, e35, e3215 */
            float4(((this.group0.yzx * other.group3.zxy) - (this.group0.zxy * other.group3.yzx)), 0.0)
        );
    }
}
public extension AntiLine {
    // Fancy infix trick (first half)
    public AntiWedgeInfixPartial<AntiLine> operator /(AntiWedgeInfix rhs) {
        return AntiWedgeInfixPartial<AntiLine>(this);
    }
}
public extension AntiWedgeInfixPartial<AntiLine> {
    // Fancy infix trick (second half)
    public Scalar operator /(AntiDipoleInversion rhs) {
        return this.a.anti_wedge(rhs);
    }
}
public extension AntiLine {
    public Scalar anti_wedge(AntiDipoleInversion other) {
        return Scalar(
            /* scalar */
            -(other.e423 * this.e15) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12)
        );
    }
}
public extension AntiLine {
    public AntiLine anti_wedge(AntiScalar other) {
        return AntiLine(
            /* e23, e31, e12 */
            float3(other.e12345) * this.group0.xyz, 
            /* e15, e25, e35 */
            float3(other.e12345) * this.group1.xyz
        );
    }
}
public extension AntiLine {
    public Scalar anti_wedge(Circle other) {
        return Scalar(
            /* scalar */
            -(this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412)
        );
    }
}
public extension AntiLine {
    public AntiMotor anti_wedge(CircleRotor other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((float3(other.e12345) * this.group0.xyz), -(this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412)), 
            /* e15, e25, e35, e3215 */
            float4(float3(1.0), 0.0) * float4(this.group1.xyz, 0.0) * float4(other.group2.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
public extension AntiLine {
    public AntiPlane anti_wedge(DipoleInversion other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(-(this.e12 * other.e4315) - (this.e15 * other.e1234), -(this.e23 * other.e4125) - (this.e25 * other.e1234), -(this.e31 * other.e4235) - (this.e35 * other.e1234), (this.e25 * other.e4315) + (this.e35 * other.e4125)) + (other.group3.zxyx * float4(this.group0.xyz.yzx, this.e15))
        );
    }
}
public extension AntiLine {
    public AntiLine anti_wedge(DualNum other) {
        return AntiLine(
            /* e23, e31, e12 */
            float3(other.e12345) * this.group0.xyz, 
            /* e15, e25, e35 */
            float3(other.e12345) * this.group1.xyz
        );
    }
}
public extension AntiLine {
    public AntiPlane anti_wedge(Flector other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(this.e12 * other.e4315 * -1.0, this.e23 * other.e4125 * -1.0, this.e31 * other.e4235 * -1.0, (this.e25 * other.e4315) + (this.e35 * other.e4125)) + (other.group1.zxyx * float4(this.group0.xyz.yzx, this.e15))
        );
    }
}
public extension AntiLine {
    public Scalar anti_wedge(Line other) {
        return Scalar(
            /* scalar */
            -(this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435)
        );
    }
}
public extension AntiLine {
    public AntiMotor anti_wedge(Motor other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((float3(other.e12345) * this.group0.xyz), -(this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435)), 
            /* e15, e25, e35, e3215 */
            float4(float3(1.0), 0.0) * float4(this.group1.xyz, 0.0) * float4(other.group0.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
public extension AntiLine {
    public MultiVector anti_wedge(MultiVector other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(-(this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412), 0.0), 
            /* e1, e2, e3, e4 */
            float4((this.group0.xyz.yzx * other.group9.zxy), 0.0) - float4((float3(other.e1234) * this.group1.xyz), 0.0) - float4((this.group0.xyz.zxy * other.group9.yzx), 0.0), 
            /* e5 */
            (this.e15 * other.e4235) + (this.e25 * other.e4315) + (this.e35 * other.e4125), 
            /* e15, e25, e35, e45 */
            float4(other.group0.xy.yy, other.e12345, 0.0) * float4(float3(1.0), 0.0) * float4(this.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(other.e12345) * this.group0.xyz, 
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
public extension AntiLine {
    public AntiPlane anti_wedge(Plane other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(this.e12 * other.e4315 * -1.0, this.e23 * other.e4125 * -1.0, this.e31 * other.e4235 * -1.0, (this.e25 * other.e4315) + (this.e35 * other.e4125)) + (other.group0.zxyx * float4(this.group0.xyz.yzx, this.e15))
        );
    }
}
public extension AntiLine {
    public AntiPlane anti_wedge(Sphere other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(-(this.e12 * other.e4315) - (this.e15 * other.e1234), -(this.e23 * other.e4125) - (this.e25 * other.e1234), -(this.e31 * other.e4235) - (this.e35 * other.e1234), (this.e25 * other.e4315) + (this.e35 * other.e4125)) + (other.group0.zxyx * float4(this.group0.xyz.yzx, this.e15))
        );
    }
}
public extension AntiLine {
    public AntiMotor anti_wedge(VersorEven other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((float3(other.e12345) * this.group0.xyz), -(this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412)), 
            /* e15, e25, e35, e3215 */
            float4(float3(1.0), 0.0) * float4(this.group1.xyz, 0.0) * float4(other.group0.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
public extension AntiLine {
    public AntiPlane anti_wedge(VersorOdd other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(-(this.e12 * other.e4315) - (this.e15 * other.e1234), -(this.e23 * other.e4125) - (this.e25 * other.e1234), -(this.e31 * other.e4235) - (this.e35 * other.e1234), (this.e25 * other.e4315) + (this.e35 * other.e4125)) + (other.group3.zxyx * float4(this.group0.xyz.yzx, this.e15))
        );
    }
}
public extension AntiMotor {
    // Fancy infix trick (first half)
    public AntiWedgeInfixPartial<AntiMotor> operator /(AntiWedgeInfix rhs) {
        return AntiWedgeInfixPartial<AntiMotor>(this);
    }
}
public extension AntiWedgeInfixPartial<AntiMotor> {
    // Fancy infix trick (second half)
    public AntiPlane operator /(AntiCircleRotor rhs) {
        return this.a.anti_wedge(rhs);
    }
}
public extension AntiMotor {
    public AntiPlane anti_wedge(AntiCircleRotor other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(this.e3215) * float4(other.group0.xyz, other.e45) * float4(-1.0)
        );
    }
}
public extension AntiMotor {
    public AntiMotor anti_wedge(AntiDipoleInversion other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((float3(this.e3215) * other.group0.xyz), (other.e4 * this.e3215) - (other.e423 * this.e15) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12)), 
            /* e15, e25, e35, e3215 */
            float4(float3(1.0), 0.0) * float4(this.group1.www, 0.0) * float4(other.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
public extension AntiMotor {
    public AntiMotor anti_wedge(AntiScalar other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(other.e12345) * this.group0, 
            /* e15, e25, e35, e3215 */
            float4(other.e12345) * this.group1
        );
    }
}
public extension AntiMotor {
    public AntiMotor anti_wedge(Circle other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((float3(this.e3215) * other.group0.xyz), -(this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412)), 
            /* e15, e25, e35, e3215 */
            float4(float3(1.0), 0.0) * float4(this.group1.www, 0.0) * float4(other.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
public extension AntiMotor {
    public AntiMotor anti_wedge(CircleRotor other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(this.e23 * other.e12345, this.e31 * other.e12345, this.e12 * other.e12345, -(this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412)) + float4((other.group0.xyz * this.group1.www), this.scalar * other.e12345), 
            /* e15, e25, e35, e3215 */
            float4(((float3(this.e3215) * other.group1.xyz) + (float3(other.e12345) * this.group1.xyz)), this.e3215 * other.e12345)
        );
    }
}
public extension AntiMotor {
    public AntiPlane anti_wedge(Dipole other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4((other.group0.xyz * this.group1.www * float3(-1.0)), this.e3215 * other.e45 * -1.0)
        );
    }
}
public extension AntiMotor {
    public AntiFlector anti_wedge(DipoleInversion other) {
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(this.e3215) * float4(other.group3.xyz, other.e1234) * float4(-1.0), 
            /* e1, e2, e3, e5 */
            float4(-(this.e12 * other.e4315) - (this.e15 * other.e1234), -(this.e23 * other.e4125) - (this.e25 * other.e1234), -(this.e31 * other.e4235) - (this.e35 * other.e1234), (this.e25 * other.e4315) + (this.e35 * other.e4125)) + (other.group3.zxyx * float4(this.group0.yzx, this.e15)) - float4((other.group0.xyz * this.group1.www), this.e3215 * other.e45)
        );
    }
}
public extension AntiMotor {
    public AntiMotor anti_wedge(DualNum other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(other.e12345) * this.group0, 
            /* e15, e25, e35, e3215 */
            float4(other.e12345) * this.group1
        );
    }
}
public extension AntiMotor {
    public DualNum anti_wedge(FlatPoint other) {
        return DualNum(
            /* e5, e12345 */
            float2(this.e3215 * other.e45, 1.0) * float2(-1.0, 0.0)
        );
    }
}
public extension AntiMotor {
    public AntiFlector anti_wedge(Flector other) {
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(float3(1.0), 0.0) * float4(this.group1.www, 0.0) * float4(other.group1.xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0), 
            /* e1, e2, e3, e5 */
            (other.group1.zxyx * float4(this.group0.yzx, this.e15)) + float4(float3(0.0), (this.e25 * other.e4315) + (this.e35 * other.e4125)) - float4((this.group0.zxy * other.group1.yzx), this.e3215 * other.e45)
        );
    }
}
public extension AntiMotor {
    public AntiMotor anti_wedge(Line other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(float3(0.0), -(this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435)), 
            /* e15, e25, e35, e3215 */
            float4(float3(1.0), 0.0) * float4(other.group0.xyz, 0.0) * float4(this.group1.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
public extension AntiMotor {
    public AntiMotor anti_wedge(Motor other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((float3(other.e12345) * this.group0.xyz), (this.scalar * other.e12345) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435)), 
            /* e15, e25, e35, e3215 */
            float4(((float3(this.e3215) * other.group0.xyz) + (float3(other.e12345) * this.group1.xyz)), this.e3215 * other.e12345)
        );
    }
}
public extension AntiMotor {
    public MultiVector anti_wedge(MultiVector other) {
        return MultiVector(
            /* scalar, e12345 */
            float2((this.scalar * other.e12345) + (this.e3215 * other.e4) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412), 0.0), 
            /* e1, e2, e3, e4 */
            float4((this.group0.yzx * other.group9.zxy), 0.0) - float4((float3(this.e3215) * other.group4.xyz), 0.0) - float4((float3(other.e1234) * this.group1.xyz), 0.0) - float4((this.group0.zxy * other.group9.yzx), 0.0), 
            /* e5 */
            (this.e15 * other.e4235) + (this.e25 * other.e4315) + (this.e35 * other.e4125) - (this.e3215 * other.e45), 
            /* e15, e25, e35, e45 */
            float4(((float3(this.e3215) * other.group6.xyz) + (float3(other.e12345) * this.group1.xyz)), 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            (float3(this.e3215) * other.group7.xyz) + (float3(other.e12345) * this.group0.xyz), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), this.e3215 * other.e1234 * -1.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(this.e3215) * other.group9.xyz * float3(-1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), this.e3215 * other.e12345), 
            /* e1234 */
            0.0
        );
    }
}
public extension AntiMotor {
    public AntiFlector anti_wedge(Plane other) {
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(float3(1.0), 0.0) * float4(this.group1.www, 0.0) * float4(other.group0.xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0), 
            /* e1, e2, e3, e5 */
            float4(this.e12 * other.e4315 * -1.0, this.e23 * other.e4125 * -1.0, this.e31 * other.e4235 * -1.0, (this.e25 * other.e4315) + (this.e35 * other.e4125)) + (other.group0.zxyx * float4(this.group0.yzx, this.e15))
        );
    }
}
public extension AntiMotor {
    public Scalar anti_wedge(RoundPoint other) {
        return Scalar(
            /* scalar */
            this.e3215 * other.e4
        );
    }
}
public extension AntiMotor {
    public AntiFlector anti_wedge(Sphere other) {
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(this.e3215) * float4(other.group0.xyz, other.e1234) * float4(-1.0), 
            /* e1, e2, e3, e5 */
            float4(-(this.e12 * other.e4315) - (this.e15 * other.e1234), -(this.e23 * other.e4125) - (this.e25 * other.e1234), -(this.e31 * other.e4235) - (this.e35 * other.e1234), (this.e25 * other.e4315) + (this.e35 * other.e4125)) + (other.group0.zxyx * float4(this.group0.yzx, this.e15))
        );
    }
}
public extension AntiMotor {
    public AntiMotor anti_wedge(VersorEven other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            (float4(this.e3215) * float4(other.group0.xyz, other.e4)) + (float4(other.e12345) * this.group0) + float4(float3(0.0), -(this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412)), 
            /* e15, e25, e35, e3215 */
            float4(((float3(this.e3215) * other.group1.xyz) + (float3(other.e12345) * this.group1.xyz)), this.e3215 * other.e12345)
        );
    }
}
public extension AntiMotor {
    public AntiFlector anti_wedge(VersorOdd other) {
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(this.e3215) * float4(other.group3.xyz, other.e1234) * float4(-1.0), 
            /* e1, e2, e3, e5 */
            float4(-(this.e15 * other.e1234) - (this.e3215 * other.e41), -(this.e25 * other.e1234) - (this.e3215 * other.e42), -(this.e35 * other.e1234) - (this.e3215 * other.e43), (this.e25 * other.e4315) + (this.e35 * other.e4125)) + (other.group3.zxyx * float4(this.group0.yzx, this.e15)) - float4((this.group0.zxy * other.group3.yzx), this.e3215 * other.e45)
        );
    }
}
public extension AntiPlane {
    // Fancy infix trick (first half)
    public AntiWedgeInfixPartial<AntiPlane> operator /(AntiWedgeInfix rhs) {
        return AntiWedgeInfixPartial<AntiPlane>(this);
    }
}
public extension AntiWedgeInfixPartial<AntiPlane> {
    // Fancy infix trick (second half)
    public AntiPlane operator /(AntiScalar rhs) {
        return this.a.anti_wedge(rhs);
    }
}
public extension AntiPlane {
    public AntiPlane anti_wedge(AntiScalar other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(other.e12345) * this.group0
        );
    }
}
public extension AntiPlane {
    public AntiPlane anti_wedge(CircleRotor other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(other.e12345) * this.group0
        );
    }
}
public extension AntiPlane {
    public Scalar anti_wedge(DipoleInversion other) {
        return Scalar(
            /* scalar */
            (this.e1 * other.e4235) + (this.e2 * other.e4315) + (this.e3 * other.e4125) + (this.e5 * other.e1234)
        );
    }
}
public extension AntiPlane {
    public AntiPlane anti_wedge(DualNum other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(other.e12345) * this.group0
        );
    }
}
public extension AntiPlane {
    public Scalar anti_wedge(Flector other) {
        return Scalar(
            /* scalar */
            (this.e1 * other.e4235) + (this.e2 * other.e4315) + (this.e3 * other.e4125)
        );
    }
}
public extension AntiPlane {
    public AntiPlane anti_wedge(Motor other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(other.e12345) * this.group0
        );
    }
}
public extension AntiPlane {
    public MultiVector anti_wedge(MultiVector other) {
        return MultiVector(
            /* scalar, e12345 */
            float2((this.e1 * other.e4235) + (this.e2 * other.e4315) + (this.e3 * other.e4125) + (this.e5 * other.e1234), 0.0), 
            /* e1, e2, e3, e4 */
            float4(other.group0.xy.yy, other.e12345, 0.0) * float4(float3(1.0), 0.0) * float4(this.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e5 */
            this.e5 * other.e12345, 
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
public extension AntiPlane {
    public Scalar anti_wedge(Plane other) {
        return Scalar(
            /* scalar */
            (this.e1 * other.e4235) + (this.e2 * other.e4315) + (this.e3 * other.e4125)
        );
    }
}
public extension AntiPlane {
    public Scalar anti_wedge(Sphere other) {
        return Scalar(
            /* scalar */
            (this.e1 * other.e4235) + (this.e2 * other.e4315) + (this.e3 * other.e4125) + (this.e5 * other.e1234)
        );
    }
}
public extension AntiPlane {
    public AntiPlane anti_wedge(VersorEven other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(other.e12345) * this.group0
        );
    }
}
public extension AntiPlane {
    public Scalar anti_wedge(VersorOdd other) {
        return Scalar(
            /* scalar */
            (this.e1 * other.e4235) + (this.e2 * other.e4315) + (this.e3 * other.e4125) + (this.e5 * other.e1234)
        );
    }
}
public extension AntiScalar {
    // Fancy infix trick (first half)
    public AntiWedgeInfixPartial<AntiScalar> operator /(AntiWedgeInfix rhs) {
        return AntiWedgeInfixPartial<AntiScalar>(this);
    }
}
public extension AntiWedgeInfixPartial<AntiScalar> {
    // Fancy infix trick (second half)
    public AntiCircleRotor operator /(AntiCircleRotor rhs) {
        return this.a.anti_wedge(rhs);
    }
}
public extension AntiScalar {
    public AntiCircleRotor anti_wedge(AntiCircleRotor other) {
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(this.e12345) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(this.e12345) * other.group1, 
            /* e15, e25, e35, scalar */
            float4(this.e12345) * other.group2
        );
    }
}
public extension AntiScalar {
    public AntiDipoleInversion anti_wedge(AntiDipoleInversion other) {
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(this.e12345) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(this.e12345) * other.group1, 
            /* e235, e315, e125, e4 */
            float4(this.e12345) * other.group2, 
            /* e1, e2, e3, e5 */
            float4(this.e12345) * other.group3
        );
    }
}
public extension AntiScalar {
    public AntiDualNum anti_wedge(AntiDualNum other) {
        return AntiDualNum(
            /* e3215, scalar */
            float2(this.e12345) * other.group0.xy
        );
    }
}
public extension AntiScalar {
    public AntiFlatPoint anti_wedge(AntiFlatPoint other) {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(this.e12345) * other.group0
        );
    }
}
public extension AntiScalar {
    public AntiFlector anti_wedge(AntiFlector other) {
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(this.e12345) * other.group0, 
            /* e1, e2, e3, e5 */
            float4(this.e12345) * other.group1
        );
    }
}
public extension AntiScalar {
    public AntiLine anti_wedge(AntiLine other) {
        return AntiLine(
            /* e23, e31, e12 */
            float3(this.e12345) * other.group0.xyz, 
            /* e15, e25, e35 */
            float3(this.e12345) * other.group1.xyz
        );
    }
}
public extension AntiScalar {
    public AntiMotor anti_wedge(AntiMotor other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(this.e12345) * other.group0, 
            /* e15, e25, e35, e3215 */
            float4(this.e12345) * other.group1
        );
    }
}
public extension AntiScalar {
    public AntiPlane anti_wedge(AntiPlane other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(this.e12345) * other.group0
        );
    }
}
public extension AntiScalar {
    public AntiScalar anti_wedge(AntiScalar other) {
        return AntiScalar(
            /* e12345 */
            other.e12345 * this.e12345
        );
    }
}
public extension AntiScalar {
    public Circle anti_wedge(Circle other) {
        return Circle(
            /* e423, e431, e412 */
            float3(this.e12345) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(this.e12345) * other.group1, 
            /* e235, e315, e125 */
            float3(this.e12345) * other.group2.xyz
        );
    }
}
public extension AntiScalar {
    public CircleRotor anti_wedge(CircleRotor other) {
        return CircleRotor(
            /* e423, e431, e412 */
            float3(this.e12345) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(this.e12345) * other.group1, 
            /* e235, e315, e125, e12345 */
            float4(this.e12345) * other.group2
        );
    }
}
public extension AntiScalar {
    public Dipole anti_wedge(Dipole other) {
        return Dipole(
            /* e41, e42, e43 */
            float3(this.e12345) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(this.e12345) * other.group1, 
            /* e15, e25, e35 */
            float3(this.e12345) * other.group2.xyz
        );
    }
}
public extension AntiScalar {
    public DipoleInversion anti_wedge(DipoleInversion other) {
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(this.e12345) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(this.e12345) * other.group1, 
            /* e15, e25, e35, e1234 */
            float4(this.e12345) * other.group2, 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * other.group3
        );
    }
}
public extension AntiScalar {
    public DualNum anti_wedge(DualNum other) {
        return DualNum(
            /* e5, e12345 */
            float2(this.e12345) * other.group0.xy
        );
    }
}
public extension AntiScalar {
    public FlatPoint anti_wedge(FlatPoint other) {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(this.e12345) * other.group0
        );
    }
}
public extension AntiScalar {
    public Flector anti_wedge(Flector other) {
        return Flector(
            /* e15, e25, e35, e45 */
            float4(this.e12345) * other.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * other.group1
        );
    }
}
public extension AntiScalar {
    public Line anti_wedge(Line other) {
        return Line(
            /* e415, e425, e435 */
            float3(this.e12345) * other.group0.xyz, 
            /* e235, e315, e125 */
            float3(this.e12345) * other.group1.xyz
        );
    }
}
public extension AntiScalar {
    public Motor anti_wedge(Motor other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(this.e12345) * other.group0, 
            /* e235, e315, e125, e5 */
            float4(this.e12345) * other.group1
        );
    }
}
public extension AntiScalar {
    public MultiVector anti_wedge(MultiVector other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(this.e12345) * other.group0.xy, 
            /* e1, e2, e3, e4 */
            float4(this.e12345) * other.group1, 
            /* e5 */
            this.e12345 * other.e5, 
            /* e15, e25, e35, e45 */
            float4(this.e12345) * other.group3, 
            /* e41, e42, e43 */
            float3(this.e12345) * other.group4.xyz, 
            /* e23, e31, e12 */
            float3(this.e12345) * other.group5.xyz, 
            /* e415, e425, e435, e321 */
            float4(this.e12345) * other.group6, 
            /* e423, e431, e412 */
            float3(this.e12345) * other.group7.xyz, 
            /* e235, e315, e125 */
            float3(this.e12345) * other.group8.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * other.group9, 
            /* e1234 */
            this.e12345 * other.e1234
        );
    }
}
public extension AntiScalar {
    public Plane anti_wedge(Plane other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * other.group0
        );
    }
}
public extension AntiScalar {
    public RoundPoint anti_wedge(RoundPoint other) {
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(this.e12345) * other.group0, 
            /* e5 */
            this.e12345 * other.e5
        );
    }
}
public extension AntiScalar {
    public Scalar anti_wedge(Scalar other) {
        return Scalar(
            /* scalar */
            this.e12345 * other.scalar
        );
    }
}
public extension AntiScalar {
    public Sphere anti_wedge(Sphere other) {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * other.group0, 
            /* e1234 */
            this.e12345 * other.e1234
        );
    }
}
public extension AntiScalar {
    public VersorEven anti_wedge(VersorEven other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(this.e12345) * other.group0, 
            /* e415, e425, e435, e321 */
            float4(this.e12345) * other.group1, 
            /* e235, e315, e125, e5 */
            float4(this.e12345) * other.group2, 
            /* e1, e2, e3, e4 */
            float4(this.e12345) * other.group3
        );
    }
}
public extension AntiScalar {
    public VersorOdd anti_wedge(VersorOdd other) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(this.e12345) * other.group0, 
            /* e23, e31, e12, e45 */
            float4(this.e12345) * other.group1, 
            /* e15, e25, e35, e1234 */
            float4(this.e12345) * other.group2, 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * other.group3
        );
    }
}
public extension Circle {
    // Fancy infix trick (first half)
    public AntiWedgeInfixPartial<Circle> operator /(AntiWedgeInfix rhs) {
        return AntiWedgeInfixPartial<Circle>(this);
    }
}
public extension AntiWedgeInfixPartial<Circle> {
    // Fancy infix trick (second half)
    public Scalar operator /(AntiCircleRotor rhs) {
        return this.a.anti_wedge(rhs);
    }
}
public extension Circle {
    public Scalar anti_wedge(AntiCircleRotor other) {
        return Scalar(
            /* scalar */
            -(other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e45 * this.e321) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412)
        );
    }
}
public extension Circle {
    public RoundPoint anti_wedge(AntiDipoleInversion other) {
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4((other.e412 * this.e315) + (other.e415 * this.e321) + (other.e321 * this.e415) + (other.e315 * this.e412), (other.e423 * this.e125) + (other.e425 * this.e321) + (other.e321 * this.e425) + (other.e125 * this.e423), (other.e431 * this.e235) + (other.e435 * this.e321) + (other.e321 * this.e435) + (other.e235 * this.e431), -(other.e412 * this.e435) - (other.e415 * this.e423) - (other.e425 * this.e431) - (other.e435 * this.e412)) - float4((other.group0.xyz.yzx * this.group2.xyz.zxy), other.e423 * this.e415) - float4((this.group0.xyz.yzx * other.group2.zxy), other.e431 * this.e425), 
            /* e5 */
            -(other.e415 * this.e235) - (other.e425 * this.e315) - (other.e435 * this.e125) - (other.e235 * this.e415) - (other.e315 * this.e425) - (other.e125 * this.e435)
        );
    }
}
public extension Circle {
    public AntiLine anti_wedge(AntiDualNum other) {
        return AntiLine(
            /* e23, e31, e12 */
            float3(other.e3215) * this.group0.xyz, 
            /* e15, e25, e35 */
            float3(other.e3215) * this.group1.xyz
        );
    }
}
public extension Circle {
    public AntiPlane anti_wedge(AntiFlatPoint other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4((other.e315 * this.e412) + (other.e321 * this.e415), (other.e125 * this.e423) + (other.e321 * this.e425), (other.e235 * this.e431) + (other.e321 * this.e435), -(other.e315 * this.e425) - (other.e125 * this.e435)) - float4((this.group0.xyz.yzx * other.group0.zxy), other.e235 * this.e415)
        );
    }
}
public extension Circle {
    public AntiPlane anti_wedge(AntiFlector other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4((other.e315 * this.e412) + (other.e321 * this.e415), (other.e125 * this.e423) + (other.e321 * this.e425), (other.e235 * this.e431) + (other.e321 * this.e435), -(other.e315 * this.e425) - (other.e125 * this.e435)) - float4((this.group0.xyz.yzx * other.group0.zxy), other.e235 * this.e415)
        );
    }
}
public extension Circle {
    public Scalar anti_wedge(AntiLine other) {
        return Scalar(
            /* scalar */
            -(other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412)
        );
    }
}
public extension Circle {
    public AntiMotor anti_wedge(AntiMotor other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((float3(other.e3215) * this.group0.xyz), -(other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412)), 
            /* e15, e25, e35, e3215 */
            float4(float3(1.0), 0.0) * float4(other.group1.www, 0.0) * float4(this.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
public extension Circle {
    public Circle anti_wedge(AntiScalar other) {
        return Circle(
            /* e423, e431, e412 */
            float3(other.e12345) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(other.e12345) * this.group1, 
            /* e235, e315, e125 */
            float3(other.e12345) * this.group2.xyz
        );
    }
}
public extension Circle {
    public RoundPoint anti_wedge(Circle other) {
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4((other.e412 * this.e315) + (other.e415 * this.e321) + (other.e321 * this.e415) + (other.e315 * this.e412), (other.e423 * this.e125) + (other.e425 * this.e321) + (other.e321 * this.e425) + (other.e125 * this.e423), (other.e431 * this.e235) + (other.e435 * this.e321) + (other.e321 * this.e435) + (other.e235 * this.e431), -(other.e412 * this.e435) - (other.e415 * this.e423) - (other.e425 * this.e431) - (other.e435 * this.e412)) - float4((other.group0.xyz.yzx * this.group2.xyz.zxy), other.e423 * this.e415) - float4((other.group2.xyz.zxy * this.group0.xyz.yzx), other.e431 * this.e425), 
            /* e5 */
            -(other.e415 * this.e235) - (other.e425 * this.e315) - (other.e435 * this.e125) - (other.e235 * this.e415) - (other.e315 * this.e425) - (other.e125 * this.e435)
        );
    }
}
public extension Circle {
    public AntiDipoleInversion anti_wedge(CircleRotor other) {
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(other.e12345) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(other.e12345) * this.group1, 
            /* e235, e315, e125, e4 */
            float4((float3(other.e12345) * this.group2.xyz), -(this.e423 * other.e415) - (this.e431 * other.e425) - (this.e412 * other.e435) - (this.e415 * other.e423) - (this.e425 * other.e431) - (this.e435 * other.e412)), 
            /* e1, e2, e3, e5 */
            float4((this.e412 * other.e315) + (this.e415 * other.e321) + (this.e321 * other.e415) + (this.e315 * other.e412), (this.e423 * other.e125) + (this.e425 * other.e321) + (this.e321 * other.e425) + (this.e125 * other.e423), (this.e431 * other.e235) + (this.e435 * other.e321) + (this.e321 * other.e435) + (this.e235 * other.e431), -(this.e415 * other.e235) - (this.e425 * other.e315) - (this.e435 * other.e125) - (this.e125 * other.e435)) - float4((this.group0.xyz.yzx * other.group2.zxy), this.e235 * other.e415) - float4((this.group2.xyz.zxy * other.group0.xyz.yzx), this.e315 * other.e425)
        );
    }
}
public extension Circle {
    public Scalar anti_wedge(Dipole other) {
        return Scalar(
            /* scalar */
            -(this.e423 * other.e15) - (this.e431 * other.e25) - (this.e412 * other.e35) - (this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12) - (this.e321 * other.e45) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43)
        );
    }
}
public extension Circle {
    public AntiCircleRotor anti_wedge(DipoleInversion other) {
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (float3(other.e1234) * this.group1.xyz) + (this.group0.xyz.zxy * other.group3.yzx) - (this.group0.xyz.yzx * other.group3.zxy), 
            /* e23, e31, e12, e45 */
            float4((this.e423 * other.e3215) + (this.e235 * other.e1234), (this.e431 * other.e3215) + (this.e315 * other.e1234), (this.e412 * other.e3215) + (this.e125 * other.e1234), -(this.e425 * other.e4315) - (this.e435 * other.e4125)) - (this.group1.wwwx * other.group3.xyzx), 
            /* e15, e25, e35, scalar */
            float4((this.e415 * other.e3215) + (this.e315 * other.e4125), (this.e425 * other.e3215) + (this.e125 * other.e4235), (this.e435 * other.e3215) + (this.e235 * other.e4315), -(this.e431 * other.e25) - (this.e412 * other.e35) - (this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12) - (this.e321 * other.e45) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43)) - float4((this.group2.xyz.zxy * other.group3.yzx), this.e423 * other.e15)
        );
    }
}
public extension Circle {
    public Circle anti_wedge(DualNum other) {
        return Circle(
            /* e423, e431, e412 */
            float3(other.e12345) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(other.e12345) * this.group1, 
            /* e235, e315, e125 */
            float3(other.e12345) * this.group2.xyz
        );
    }
}
public extension Circle {
    public Scalar anti_wedge(FlatPoint other) {
        return Scalar(
            /* scalar */
            -(this.e423 * other.e15) - (this.e431 * other.e25) - (this.e412 * other.e35) - (this.e321 * other.e45)
        );
    }
}
public extension Circle {
    public AntiCircleRotor anti_wedge(Flector other) {
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (this.group0.xyz.zxy * other.group1.yzx) - (this.group0.xyz.yzx * other.group1.zxy), 
            /* e23, e31, e12, e45 */
            float4(this.e423 * other.e3215, this.e431 * other.e3215, this.e412 * other.e3215, -(this.e425 * other.e4315) - (this.e435 * other.e4125)) - (this.group1.wwwx * other.group1.xyzx), 
            /* e15, e25, e35, scalar */
            float4((this.e415 * other.e3215) + (this.e315 * other.e4125), (this.e425 * other.e3215) + (this.e125 * other.e4235), (this.e435 * other.e3215) + (this.e235 * other.e4315), -(this.e431 * other.e25) - (this.e412 * other.e35) - (this.e321 * other.e45)) - float4((this.group2.xyz.zxy * other.group1.yzx), this.e423 * other.e15)
        );
    }
}
public extension Circle {
    public RoundPoint anti_wedge(Line other) {
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4((this.e412 * other.e315) + (this.e321 * other.e415), (this.e423 * other.e125) + (this.e321 * other.e425), (this.e431 * other.e235) + (this.e321 * other.e435), -(this.e431 * other.e425) - (this.e412 * other.e435)) - float4((this.group0.xyz.yzx * other.group1.xyz.zxy), this.e423 * other.e415), 
            /* e5 */
            -(this.e415 * other.e235) - (this.e425 * other.e315) - (this.e435 * other.e125) - (this.e235 * other.e415) - (this.e315 * other.e425) - (this.e125 * other.e435)
        );
    }
}
public extension Circle {
    public AntiDipoleInversion anti_wedge(Motor other) {
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(other.e12345) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(other.e12345) * this.group1, 
            /* e235, e315, e125, e4 */
            float4((float3(other.e12345) * this.group2.xyz), -(this.e423 * other.e415) - (this.e431 * other.e425) - (this.e412 * other.e435)), 
            /* e1, e2, e3, e5 */
            float4((this.e412 * other.e315) + (this.e321 * other.e415), (this.e423 * other.e125) + (this.e321 * other.e425), (this.e431 * other.e235) + (this.e321 * other.e435), -(this.e415 * other.e235) - (this.e425 * other.e315) - (this.e435 * other.e125) - (this.e315 * other.e425) - (this.e125 * other.e435)) - float4((this.group0.xyz.yzx * other.group1.zxy), this.e235 * other.e415)
        );
    }
}
public extension Circle {
    public MultiVector anti_wedge(MultiVector other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(-(this.e423 * other.e15) - (this.e431 * other.e25) - (this.e412 * other.e35) - (this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12) - (this.e321 * other.e45) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43), 0.0), 
            /* e1, e2, e3, e4 */
            float4((this.e412 * other.e315) + (this.e415 * other.e321) + (this.e321 * other.e415) + (this.e315 * other.e412), (this.e423 * other.e125) + (this.e425 * other.e321) + (this.e321 * other.e425) + (this.e125 * other.e423), (this.e431 * other.e235) + (this.e435 * other.e321) + (this.e321 * other.e435) + (this.e235 * other.e431), -(this.e412 * other.e435) - (this.e415 * other.e423) - (this.e425 * other.e431) - (this.e435 * other.e412)) - float4((this.group0.xyz.yzx * other.group8.xyz.zxy), this.e423 * other.e415) - float4((this.group2.xyz.zxy * other.group7.xyz.yzx), this.e431 * other.e425), 
            /* e5 */
            -(this.e415 * other.e235) - (this.e425 * other.e315) - (this.e435 * other.e125) - (this.e235 * other.e415) - (this.e315 * other.e425) - (this.e125 * other.e435), 
            /* e15, e25, e35, e45 */
            float4((this.e415 * other.e3215) + (this.e315 * other.e4125), (this.e425 * other.e3215) + (this.e125 * other.e4235), (this.e435 * other.e3215) + (this.e235 * other.e4315), -(this.e425 * other.e4315) - (this.e435 * other.e4125)) - (other.group9.yzxx * float4(this.group2.xyz.zxy, this.e415)), 
            /* e41, e42, e43 */
            (float3(other.e1234) * this.group1.xyz) + (this.group0.xyz.zxy * other.group9.yzx) - (this.group0.xyz.yzx * other.group9.zxy), 
            /* e23, e31, e12 */
            (float3(other.e3215) * this.group0.xyz) + (float3(other.e1234) * this.group2.xyz) - (float3(this.e321) * other.group9.xyz), 
            /* e415, e425, e435, e321 */
            float4(other.e12345) * this.group1, 
            /* e423, e431, e412 */
            float3(other.e12345) * this.group0.xyz, 
            /* e235, e315, e125 */
            float3(other.e12345) * this.group2.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
public extension Circle {
    public Dipole anti_wedge(Plane other) {
        return Dipole(
            /* e41, e42, e43 */
            (this.group0.xyz.zxy * other.group0.yzx) - (this.group0.xyz.yzx * other.group0.zxy), 
            /* e23, e31, e12, e45 */
            float4(this.e423 * other.e3215, this.e431 * other.e3215, this.e412 * other.e3215, -(this.e425 * other.e4315) - (this.e435 * other.e4125)) - (this.group1.wwwx * other.group0.xyzx), 
            /* e15, e25, e35 */
            (float3(other.e3215) * this.group1.xyz) + (this.group2.xyz.yzx * other.group0.zxy) - (this.group2.xyz.zxy * other.group0.yzx)
        );
    }
}
public extension Circle {
    public Dipole anti_wedge(Sphere other) {
        return Dipole(
            /* e41, e42, e43 */
            (float3(other.e1234) * this.group1.xyz) + (this.group0.xyz.zxy * other.group0.yzx) - (this.group0.xyz.yzx * other.group0.zxy), 
            /* e23, e31, e12, e45 */
            float4((this.e423 * other.e3215) + (this.e235 * other.e1234), (this.e431 * other.e3215) + (this.e315 * other.e1234), (this.e412 * other.e3215) + (this.e125 * other.e1234), -(this.e425 * other.e4315) - (this.e435 * other.e4125)) - (this.group1.wwwx * other.group0.xyzx), 
            /* e15, e25, e35 */
            (float3(other.e3215) * this.group1.xyz) + (this.group2.xyz.yzx * other.group0.zxy) - (this.group2.xyz.zxy * other.group0.yzx)
        );
    }
}
public extension Circle {
    public AntiDipoleInversion anti_wedge(VersorEven other) {
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(other.e12345) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(other.e12345) * this.group1, 
            /* e235, e315, e125, e4 */
            float4((float3(other.e12345) * this.group2.xyz), -(this.e423 * other.e415) - (this.e431 * other.e425) - (this.e412 * other.e435) - (this.e415 * other.e423) - (this.e425 * other.e431) - (this.e435 * other.e412)), 
            /* e1, e2, e3, e5 */
            float4((this.e412 * other.e315) + (this.e415 * other.e321) + (this.e321 * other.e415) + (this.e315 * other.e412), (this.e423 * other.e125) + (this.e425 * other.e321) + (this.e321 * other.e425) + (this.e125 * other.e423), (this.e431 * other.e235) + (this.e435 * other.e321) + (this.e321 * other.e435) + (this.e235 * other.e431), -(this.e415 * other.e235) - (this.e425 * other.e315) - (this.e435 * other.e125) - (this.e125 * other.e435)) - float4((this.group0.xyz.yzx * other.group2.zxy), this.e235 * other.e415) - float4((this.group2.xyz.zxy * other.group0.yzx), this.e315 * other.e425)
        );
    }
}
public extension Circle {
    public AntiCircleRotor anti_wedge(VersorOdd other) {
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (float3(other.e1234) * this.group1.xyz) + (this.group0.xyz.zxy * other.group3.yzx) - (this.group0.xyz.yzx * other.group3.zxy), 
            /* e23, e31, e12, e45 */
            float4((this.e423 * other.e3215) + (this.e235 * other.e1234), (this.e431 * other.e3215) + (this.e315 * other.e1234), (this.e412 * other.e3215) + (this.e125 * other.e1234), -(this.e425 * other.e4315) - (this.e435 * other.e4125)) - (this.group1.wwwx * other.group3.xyzx), 
            /* e15, e25, e35, scalar */
            float4((this.e415 * other.e3215) + (this.e315 * other.e4125), (this.e425 * other.e3215) + (this.e125 * other.e4235), (this.e435 * other.e3215) + (this.e235 * other.e4315), -(this.e431 * other.e25) - (this.e412 * other.e35) - (this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12) - (this.e321 * other.e45) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43)) - float4((this.group2.xyz.zxy * other.group3.yzx), this.e423 * other.e15)
        );
    }
}
public extension CircleRotor {
    // Fancy infix trick (first half)
    public AntiWedgeInfixPartial<CircleRotor> operator /(AntiWedgeInfix rhs) {
        return AntiWedgeInfixPartial<CircleRotor>(this);
    }
}
public extension AntiWedgeInfixPartial<CircleRotor> {
    // Fancy infix trick (second half)
    public AntiCircleRotor operator /(AntiCircleRotor rhs) {
        return this.a.anti_wedge(rhs);
    }
}
public extension CircleRotor {
    public AntiCircleRotor anti_wedge(AntiCircleRotor other) {
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(this.e12345) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(this.e12345) * other.group1, 
            /* e15, e25, e35, scalar */
            float4((float3(this.e12345) * other.group2.xyz), (other.scalar * this.e12345) - (other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e45 * this.e321) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412))
        );
    }
}
public extension CircleRotor {
    public AntiDipoleInversion anti_wedge(AntiDipoleInversion other) {
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(this.e12345) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(this.e12345) * other.group1, 
            /* e235, e315, e125, e4 */
            float4((float3(this.e12345) * other.group2.xyz), (other.e4 * this.e12345) - (other.e423 * this.e415) - (other.e431 * this.e425) - (other.e412 * this.e435) - (other.e415 * this.e423) - (other.e425 * this.e431) - (other.e435 * this.e412)), 
            /* e1, e2, e3, e5 */
            float4((other.e415 * this.e321) + (other.e321 * this.e415) + (other.e315 * this.e412) + (other.e1 * this.e12345), (other.e425 * this.e321) + (other.e321 * this.e425) + (other.e125 * this.e423) + (other.e2 * this.e12345), (other.e435 * this.e321) + (other.e321 * this.e435) + (other.e235 * this.e431) + (other.e3 * this.e12345), -(other.e435 * this.e125) - (other.e235 * this.e415) - (other.e315 * this.e425) - (other.e125 * this.e435)) + (this.group2.yzxw * float4(other.group0.xyz.zxy, other.e5)) - (this.group2.zxyx * float4(other.group0.xyz.yzx, other.e415)) - float4((this.group0.xyz.yzx * other.group2.zxy), other.e425 * this.e315)
        );
    }
}
public extension CircleRotor {
    public AntiMotor anti_wedge(AntiDualNum other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(other.group0.xy.xx, other.e3215, other.scalar) * float4(this.group0.xyz, this.e12345), 
            /* e15, e25, e35, e3215 */
            float4(other.e3215) * float4(this.group1.xyz, this.e12345)
        );
    }
}
public extension CircleRotor {
    public AntiFlector anti_wedge(AntiFlatPoint other) {
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(this.e12345) * other.group0, 
            /* e1, e2, e3, e5 */
            float4((other.e315 * this.e412) + (other.e321 * this.e415), (other.e125 * this.e423) + (other.e321 * this.e425), (other.e235 * this.e431) + (other.e321 * this.e435), -(other.e315 * this.e425) - (other.e125 * this.e435)) - float4((this.group0.xyz.yzx * other.group0.zxy), other.e235 * this.e415)
        );
    }
}
public extension CircleRotor {
    public AntiFlector anti_wedge(AntiFlector other) {
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(this.e12345) * other.group0, 
            /* e1, e2, e3, e5 */
            float4((other.e321 * this.e415) + (other.e1 * this.e12345), (other.e321 * this.e425) + (other.e2 * this.e12345), (other.e321 * this.e435) + (other.e3 * this.e12345), -(other.e315 * this.e425) - (other.e125 * this.e435)) + float4((this.group0.xyz.zxy * other.group0.yzx), other.e5 * this.e12345) - float4((this.group0.xyz.yzx * other.group0.zxy), other.e235 * this.e415)
        );
    }
}
public extension CircleRotor {
    public AntiMotor anti_wedge(AntiLine other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((float3(this.e12345) * other.group0.xyz), -(other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412)), 
            /* e15, e25, e35, e3215 */
            float4(float3(1.0), 0.0) * float4(other.group1.xyz, 0.0) * float4(this.group2.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
public extension CircleRotor {
    public AntiMotor anti_wedge(AntiMotor other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(other.e23 * this.e12345, other.e31 * this.e12345, other.e12 * this.e12345, -(other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412)) + float4((this.group0.xyz * other.group1.www), other.scalar * this.e12345), 
            /* e15, e25, e35, e3215 */
            float4(((float3(other.e3215) * this.group1.xyz) + (float3(this.e12345) * other.group1.xyz)), other.e3215 * this.e12345)
        );
    }
}
public extension CircleRotor {
    public AntiPlane anti_wedge(AntiPlane other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(this.e12345) * other.group0
        );
    }
}
public extension CircleRotor {
    public CircleRotor anti_wedge(AntiScalar other) {
        return CircleRotor(
            /* e423, e431, e412 */
            float3(other.e12345) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(other.e12345) * this.group1, 
            /* e235, e315, e125, e12345 */
            float4(other.e12345) * this.group2
        );
    }
}
public extension CircleRotor {
    public AntiDipoleInversion anti_wedge(Circle other) {
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(this.e12345) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(this.e12345) * other.group1, 
            /* e235, e315, e125, e4 */
            float4((float3(this.e12345) * other.group2.xyz), -(other.e423 * this.e415) - (other.e431 * this.e425) - (other.e412 * this.e435) - (other.e415 * this.e423) - (other.e425 * this.e431) - (other.e435 * this.e412)), 
            /* e1, e2, e3, e5 */
            float4((other.e412 * this.e315) + (other.e415 * this.e321) + (other.e321 * this.e415) + (other.e315 * this.e412), (other.e423 * this.e125) + (other.e425 * this.e321) + (other.e321 * this.e425) + (other.e125 * this.e423), (other.e431 * this.e235) + (other.e435 * this.e321) + (other.e321 * this.e435) + (other.e235 * this.e431), -(other.e415 * this.e235) - (other.e425 * this.e315) - (other.e435 * this.e125) - (other.e125 * this.e435)) - float4((other.group0.xyz.yzx * this.group2.zxy), other.e235 * this.e415) - float4((other.group2.xyz.zxy * this.group0.xyz.yzx), other.e315 * this.e425)
        );
    }
}
public extension CircleRotor {
    public VersorEven anti_wedge(CircleRotor other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(((float3(other.e12345) * this.group0.xyz) + (float3(this.e12345) * other.group0.xyz)), other.e12345 * this.e12345), 
            /* e415, e425, e435, e321 */
            (float4(other.e12345) * this.group1) + (float4(this.e12345) * other.group1), 
            /* e235, e315, e125, e5 */
            float4((other.e235 * this.e12345) + (other.e12345 * this.e235), (other.e315 * this.e12345) + (other.e12345 * this.e315), (other.e125 * this.e12345) + (other.e12345 * this.e125), -(other.e415 * this.e235) - (other.e425 * this.e315) - (other.e435 * this.e125) - (other.e235 * this.e415) - (other.e315 * this.e425) - (other.e125 * this.e435)), 
            /* e1, e2, e3, e4 */
            float4((other.e412 * this.e315) + (other.e415 * this.e321) + (other.e321 * this.e415) + (other.e315 * this.e412), (other.e423 * this.e125) + (other.e425 * this.e321) + (other.e321 * this.e425) + (other.e125 * this.e423), (other.e431 * this.e235) + (other.e435 * this.e321) + (other.e321 * this.e435) + (other.e235 * this.e431), -(other.e412 * this.e435) - (other.e415 * this.e423) - (other.e425 * this.e431) - (other.e435 * this.e412)) - float4((other.group0.xyz.yzx * this.group2.zxy), other.e423 * this.e415) - float4((this.group0.xyz.yzx * other.group2.zxy), other.e431 * this.e425)
        );
    }
}
public extension CircleRotor {
    public AntiCircleRotor anti_wedge(Dipole other) {
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(this.e12345) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(this.e12345) * other.group1, 
            /* e15, e25, e35, scalar */
            float4((float3(this.e12345) * other.group2.xyz), -(this.e423 * other.e15) - (this.e431 * other.e25) - (this.e412 * other.e35) - (this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12) - (this.e321 * other.e45) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43))
        );
    }
}
public extension CircleRotor {
    public VersorOdd anti_wedge(DipoleInversion other) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4((this.e412 * other.e4315) + (this.e415 * other.e1234) + (this.e12345 * other.e41), (this.e423 * other.e4125) + (this.e425 * other.e1234) + (this.e12345 * other.e42), (this.e431 * other.e4235) + (this.e435 * other.e1234) + (this.e12345 * other.e43), -(this.e431 * other.e25) - (this.e412 * other.e35) - (this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12) - (this.e321 * other.e45) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43)) - float4((this.group0.xyz.yzx * other.group3.zxy), this.e423 * other.e15), 
            /* e23, e31, e12, e45 */
            float4((this.e235 * other.e1234) + (this.e12345 * other.e23), (this.e315 * other.e1234) + (this.e12345 * other.e31), (this.e125 * other.e1234) + (this.e12345 * other.e12), -(this.e425 * other.e4315) - (this.e435 * other.e4125)) + float4((this.group0.xyz * other.group3.www), this.e12345 * other.e45) - (this.group1.wwwx * other.group3.xyzx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(this.e12345) * other.group2.xyz) + (float3(other.e3215) * this.group1.xyz) + (this.group2.yzx * other.group3.zxy) - (this.group2.zxy * other.group3.yzx)), this.e12345 * other.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * other.group3
        );
    }
}
public extension CircleRotor {
    public VersorEven anti_wedge(DualNum other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(other.e12345) * float4(this.group0.xyz, this.e12345), 
            /* e415, e425, e435, e321 */
            float4(other.e12345) * this.group1, 
            /* e235, e315, e125, e5 */
            this.group2 * float4(other.group0.xy.yy, other.e12345, other.e5), 
            /* e1, e2, e3, e4 */
            float4(0.0)
        );
    }
}
public extension CircleRotor {
    public AntiCircleRotor anti_wedge(FlatPoint other) {
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), this.e12345 * other.e45), 
            /* e15, e25, e35, scalar */
            float4((this.group2.www * other.group0.xyz), -(this.e423 * other.e15) - (this.e431 * other.e25) - (this.e412 * other.e35) - (this.e321 * other.e45))
        );
    }
}
public extension CircleRotor {
    public VersorOdd anti_wedge(Flector other) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(this.e412 * other.e4315, this.e423 * other.e4125, this.e431 * other.e4235, -(this.e431 * other.e25) - (this.e412 * other.e35) - (this.e321 * other.e45)) - float4((this.group0.xyz.yzx * other.group1.zxy), this.e423 * other.e15), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), -(this.e425 * other.e4315) - (this.e435 * other.e4125)) + float4((this.group0.xyz * other.group1.www), this.e12345 * other.e45) - (this.group1.wwwx * other.group1.xyzx), 
            /* e15, e25, e35, e1234 */
            float4((float3(this.e12345) * other.group0.xyz), 0.0) + float4((float3(other.e3215) * this.group1.xyz), 0.0) + float4((this.group2.yzx * other.group1.zxy), 0.0) - float4((this.group2.zxy * other.group1.yzx), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * other.group1
        );
    }
}
public extension CircleRotor {
    public AntiDipoleInversion anti_wedge(Line other) {
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(1.0), 0.0) * float4(other.group0.xyz, 0.0) * float4(this.group2.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e235, e315, e125, e4 */
            float4((float3(this.e12345) * other.group1.xyz), -(this.e423 * other.e415) - (this.e431 * other.e425) - (this.e412 * other.e435)), 
            /* e1, e2, e3, e5 */
            float4((this.e412 * other.e315) + (this.e321 * other.e415), (this.e423 * other.e125) + (this.e321 * other.e425), (this.e431 * other.e235) + (this.e321 * other.e435), -(this.e415 * other.e235) - (this.e425 * other.e315) - (this.e435 * other.e125) - (this.e315 * other.e425) - (this.e125 * other.e435)) - float4((this.group0.xyz.yzx * other.group1.xyz.zxy), this.e235 * other.e415)
        );
    }
}
public extension CircleRotor {
    public VersorEven anti_wedge(Motor other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(other.e12345) * float4(this.group0.xyz, this.e12345), 
            /* e415, e425, e435, e321 */
            float4(((float3(this.e12345) * other.group0.xyz) + (float3(other.e12345) * this.group1.xyz)), this.e321 * other.e12345), 
            /* e235, e315, e125, e5 */
            float4(this.e12345 * other.e235, this.e12345 * other.e315, this.e12345 * other.e125, -(this.e415 * other.e235) - (this.e425 * other.e315) - (this.e435 * other.e125) - (this.e235 * other.e415) - (this.e315 * other.e425) - (this.e125 * other.e435)) + (this.group2 * float4(other.group0.www, other.e5)), 
            /* e1, e2, e3, e4 */
            float4((this.e412 * other.e315) + (this.e321 * other.e415), (this.e423 * other.e125) + (this.e321 * other.e425), (this.e431 * other.e235) + (this.e321 * other.e435), -(this.e431 * other.e425) - (this.e412 * other.e435)) - float4((this.group0.xyz.yzx * other.group1.zxy), this.e423 * other.e415)
        );
    }
}
public extension CircleRotor {
    public MultiVector anti_wedge(MultiVector other) {
        return MultiVector(
            /* scalar, e12345 */
            float2((this.e12345 * other.scalar) - (this.e423 * other.e15) - (this.e431 * other.e25) - (this.e412 * other.e35) - (this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12) - (this.e321 * other.e45) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43), this.e12345 * other.e12345), 
            /* e1, e2, e3, e4 */
            float4((this.e415 * other.e321) + (this.e321 * other.e415) + (this.e315 * other.e412) + (this.e12345 * other.e1), (this.e425 * other.e321) + (this.e321 * other.e425) + (this.e125 * other.e423) + (this.e12345 * other.e2), (this.e435 * other.e321) + (this.e321 * other.e435) + (this.e235 * other.e431) + (this.e12345 * other.e3), -(this.e412 * other.e435) - (this.e415 * other.e423) - (this.e425 * other.e431) - (this.e435 * other.e412)) + float4((this.group0.xyz.zxy * other.group8.xyz.yzx), this.e12345 * other.e4) - float4((this.group0.xyz.yzx * other.group8.xyz.zxy), this.e423 * other.e415) - float4((other.group7.xyz.yzx * this.group2.zxy), this.e431 * other.e425), 
            /* e5 */
            (this.e12345 * other.e5) - (this.e415 * other.e235) - (this.e425 * other.e315) - (this.e435 * other.e125) - (this.e235 * other.e415) - (this.e315 * other.e425) - (this.e125 * other.e435), 
            /* e15, e25, e35, e45 */
            float4((this.e315 * other.e4125) + (this.e12345 * other.e15), (this.e125 * other.e4235) + (this.e12345 * other.e25), (this.e235 * other.e4315) + (this.e12345 * other.e35), -(this.e425 * other.e4315) - (this.e435 * other.e4125)) + float4((other.group9.www * this.group1.xyz), this.e12345 * other.e45) - (other.group9.yzxx * float4(this.group2.zxy, this.e415)), 
            /* e41, e42, e43 */
            (float3(this.e12345) * other.group4.xyz) + (float3(other.e1234) * this.group1.xyz) + (this.group0.xyz.zxy * other.group9.yzx) - (this.group0.xyz.yzx * other.group9.zxy), 
            /* e23, e31, e12 */
            (float3(this.e12345) * other.group5.xyz) + (float3(other.e3215) * this.group0.xyz) + (float3(other.e1234) * this.group2.xyz) - (float3(this.e321) * other.group9.xyz), 
            /* e415, e425, e435, e321 */
            (float4(this.e12345) * other.group6) + (float4(other.e12345) * this.group1), 
            /* e423, e431, e412 */
            (float3(this.e12345) * other.group7.xyz) + (float3(other.e12345) * this.group0.xyz), 
            /* e235, e315, e125 */
            (float3(this.e12345) * other.group8.xyz) + (float3(other.e12345) * this.group2.xyz), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * other.group9, 
            /* e1234 */
            this.e12345 * other.e1234
        );
    }
}
public extension CircleRotor {
    public DipoleInversion anti_wedge(Plane other) {
        return DipoleInversion(
            /* e41, e42, e43 */
            (this.group0.xyz.zxy * other.group0.yzx) - (this.group0.xyz.yzx * other.group0.zxy), 
            /* e23, e31, e12, e45 */
            float4(this.e423 * other.e3215, this.e431 * other.e3215, this.e412 * other.e3215, -(this.e425 * other.e4315) - (this.e435 * other.e4125)) - (this.group1.wwwx * other.group0.xyzx), 
            /* e15, e25, e35, e1234 */
            float4((float3(other.e3215) * this.group1.xyz), 0.0) + float4((this.group2.yzx * other.group0.zxy), 0.0) - float4((this.group2.zxy * other.group0.yzx), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * other.group0
        );
    }
}
public extension CircleRotor {
    public RoundPoint anti_wedge(RoundPoint other) {
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(this.e12345) * other.group0, 
            /* e5 */
            this.e12345 * other.e5
        );
    }
}
public extension CircleRotor {
    public Scalar anti_wedge(Scalar other) {
        return Scalar(
            /* scalar */
            this.e12345 * other.scalar
        );
    }
}
public extension CircleRotor {
    public DipoleInversion anti_wedge(Sphere other) {
        return DipoleInversion(
            /* e41, e42, e43 */
            (float3(other.e1234) * this.group1.xyz) + (this.group0.xyz.zxy * other.group0.yzx) - (this.group0.xyz.yzx * other.group0.zxy), 
            /* e23, e31, e12, e45 */
            float4((this.e423 * other.e3215) + (this.e235 * other.e1234), (this.e431 * other.e3215) + (this.e315 * other.e1234), (this.e412 * other.e3215) + (this.e125 * other.e1234), -(this.e425 * other.e4315) - (this.e435 * other.e4125)) - (this.group1.wwwx * other.group0.xyzx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(other.e3215) * this.group1.xyz) + (this.group2.yzx * other.group0.zxy) - (this.group2.zxy * other.group0.yzx)), this.e12345 * other.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * other.group0
        );
    }
}
public extension CircleRotor {
    public VersorEven anti_wedge(VersorEven other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(((float3(this.e12345) * other.group0.xyz) + (float3(other.e12345) * this.group0.xyz)), this.e12345 * other.e12345), 
            /* e415, e425, e435, e321 */
            (float4(this.e12345) * other.group1) + (float4(other.e12345) * this.group1), 
            /* e235, e315, e125, e5 */
            float4(this.e12345 * other.e235, this.e12345 * other.e315, this.e12345 * other.e125, -(this.e415 * other.e235) - (this.e425 * other.e315) - (this.e435 * other.e125) - (this.e235 * other.e415) - (this.e315 * other.e425) - (this.e125 * other.e435)) + (this.group2 * float4(other.group0.www, other.e5)), 
            /* e1, e2, e3, e4 */
            float4((this.e415 * other.e321) + (this.e321 * other.e415) + (this.e315 * other.e412) + (this.e12345 * other.e1), (this.e425 * other.e321) + (this.e321 * other.e425) + (this.e125 * other.e423) + (this.e12345 * other.e2), (this.e435 * other.e321) + (this.e321 * other.e435) + (this.e235 * other.e431) + (this.e12345 * other.e3), -(this.e412 * other.e435) - (this.e415 * other.e423) - (this.e425 * other.e431) - (this.e435 * other.e412)) + float4((this.group0.xyz.zxy * other.group2.yzx), this.e12345 * other.e4) - float4((this.group0.xyz.yzx * other.group2.zxy), this.e423 * other.e415) - float4((this.group2.zxy * other.group0.yzx), this.e431 * other.e425)
        );
    }
}
public extension CircleRotor {
    public VersorOdd anti_wedge(VersorOdd other) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4((this.e415 * other.e1234) + (this.e12345 * other.e41), (this.e425 * other.e1234) + (this.e12345 * other.e42), (this.e435 * other.e1234) + (this.e12345 * other.e43), -(this.e431 * other.e25) - (this.e412 * other.e35) - (this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12) - (this.e321 * other.e45) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43)) + float4((this.group0.xyz.zxy * other.group3.yzx), this.e12345 * other.scalar) - float4((this.group0.xyz.yzx * other.group3.zxy), this.e423 * other.e15), 
            /* e23, e31, e12, e45 */
            float4((this.e235 * other.e1234) + (this.e12345 * other.e23), (this.e315 * other.e1234) + (this.e12345 * other.e31), (this.e125 * other.e1234) + (this.e12345 * other.e12), -(this.e425 * other.e4315) - (this.e435 * other.e4125)) + float4((this.group0.xyz * other.group3.www), this.e12345 * other.e45) - (this.group1.wwwx * other.group3.xyzx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(this.e12345) * other.group2.xyz) + (float3(other.e3215) * this.group1.xyz) + (this.group2.yzx * other.group3.zxy) - (this.group2.zxy * other.group3.yzx)), this.e12345 * other.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * other.group3
        );
    }
}
public extension Dipole {
    // Fancy infix trick (first half)
    public AntiWedgeInfixPartial<Dipole> operator /(AntiWedgeInfix rhs) {
        return AntiWedgeInfixPartial<Dipole>(this);
    }
}
public extension AntiWedgeInfixPartial<Dipole> {
    // Fancy infix trick (second half)
    public Scalar operator /(AntiDipoleInversion rhs) {
        return this.a.anti_wedge(rhs);
    }
}
public extension Dipole {
    public Scalar anti_wedge(AntiDipoleInversion other) {
        return Scalar(
            /* scalar */
            -(other.e423 * this.e15) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e321 * this.e45) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43)
        );
    }
}
public extension Dipole {
    public AntiPlane anti_wedge(AntiDualNum other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(other.e3215) * float4(this.group0.xyz, this.e45)
        );
    }
}
public extension Dipole {
    public Scalar anti_wedge(AntiFlatPoint other) {
        return Scalar(
            /* scalar */
            -(other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43) - (other.e321 * this.e45)
        );
    }
}
public extension Dipole {
    public Scalar anti_wedge(AntiFlector other) {
        return Scalar(
            /* scalar */
            -(other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43) - (other.e321 * this.e45)
        );
    }
}
public extension Dipole {
    public AntiPlane anti_wedge(AntiMotor other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4((this.group0.xyz * other.group1.www), other.e3215 * this.e45)
        );
    }
}
public extension Dipole {
    public Dipole anti_wedge(AntiScalar other) {
        return Dipole(
            /* e41, e42, e43 */
            float3(other.e12345) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(other.e12345) * this.group1, 
            /* e15, e25, e35 */
            float3(other.e12345) * this.group2.xyz
        );
    }
}
public extension Dipole {
    public Scalar anti_wedge(Circle other) {
        return Scalar(
            /* scalar */
            -(other.e423 * this.e15) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e321 * this.e45) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43)
        );
    }
}
public extension Dipole {
    public AntiCircleRotor anti_wedge(CircleRotor other) {
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(other.e12345) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(other.e12345) * this.group1, 
            /* e15, e25, e35, scalar */
            float4((float3(other.e12345) * this.group2.xyz), -(other.e423 * this.e15) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e321 * this.e45) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43))
        );
    }
}
public extension Dipole {
    public RoundPoint anti_wedge(DipoleInversion other) {
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4((this.e41 * other.e3215) + (this.e31 * other.e4125), (this.e42 * other.e3215) + (this.e12 * other.e4235), (this.e43 * other.e3215) + (this.e23 * other.e4315), -(this.e43 * other.e4125) - (this.e45 * other.e1234)) - (other.group3.yzxy * float4(this.group1.zxy, this.e42)) - float4((this.group2.xyz * other.group2.www), this.e41 * other.e4235), 
            /* e5 */
            (this.e45 * other.e3215) + (this.e15 * other.e4235) + (this.e25 * other.e4315) + (this.e35 * other.e4125)
        );
    }
}
public extension Dipole {
    public Dipole anti_wedge(DualNum other) {
        return Dipole(
            /* e41, e42, e43 */
            float3(other.e12345) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(other.e12345) * this.group1, 
            /* e15, e25, e35 */
            float3(other.e12345) * this.group2.xyz
        );
    }
}
public extension Dipole {
    public RoundPoint anti_wedge(Flector other) {
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4((this.e41 * other.e3215) + (this.e31 * other.e4125), (this.e42 * other.e3215) + (this.e12 * other.e4235), (this.e43 * other.e3215) + (this.e23 * other.e4315), -(this.e42 * other.e4315) - (this.e43 * other.e4125)) - (other.group1.yzxx * float4(this.group1.zxy, this.e41)), 
            /* e5 */
            (this.e45 * other.e3215) + (this.e15 * other.e4235) + (this.e25 * other.e4315) + (this.e35 * other.e4125)
        );
    }
}
public extension Dipole {
    public Scalar anti_wedge(Line other) {
        return Scalar(
            /* scalar */
            -(this.e41 * other.e235) - (this.e42 * other.e315) - (this.e43 * other.e125) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435)
        );
    }
}
public extension Dipole {
    public AntiCircleRotor anti_wedge(Motor other) {
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(other.e12345) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(other.e12345) * this.group1, 
            /* e15, e25, e35, scalar */
            float4((float3(other.e12345) * this.group2.xyz), -(this.e41 * other.e235) - (this.e42 * other.e315) - (this.e43 * other.e125) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435))
        );
    }
}
public extension Dipole {
    public MultiVector anti_wedge(MultiVector other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(-(this.e41 * other.e235) - (this.e42 * other.e315) - (this.e43 * other.e125) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435) - (this.e45 * other.e321) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412), 0.0), 
            /* e1, e2, e3, e4 */
            float4((this.e41 * other.e3215) + (this.e31 * other.e4125), (this.e42 * other.e3215) + (this.e12 * other.e4235), (this.e43 * other.e3215) + (this.e23 * other.e4315), -(this.e43 * other.e4125) - (this.e45 * other.e1234)) - (other.group9.yzxy * float4(this.group1.zxy, this.e42)) - float4((float3(other.e1234) * this.group2.xyz), this.e41 * other.e4235), 
            /* e5 */
            (this.e45 * other.e3215) + (this.e15 * other.e4235) + (this.e25 * other.e4315) + (this.e35 * other.e4125), 
            /* e15, e25, e35, e45 */
            float4(other.e12345) * float4(this.group2.xyz, this.e45), 
            /* e41, e42, e43 */
            float3(other.e12345) * this.group0.xyz, 
            /* e23, e31, e12 */
            float3(other.e12345) * this.group1.xyz, 
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
public extension Dipole {
    public RoundPoint anti_wedge(Plane other) {
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4((this.e41 * other.e3215) + (this.e31 * other.e4125), (this.e42 * other.e3215) + (this.e12 * other.e4235), (this.e43 * other.e3215) + (this.e23 * other.e4315), -(this.e42 * other.e4315) - (this.e43 * other.e4125)) - (other.group0.yzxx * float4(this.group1.zxy, this.e41)), 
            /* e5 */
            (this.e45 * other.e3215) + (this.e15 * other.e4235) + (this.e25 * other.e4315) + (this.e35 * other.e4125)
        );
    }
}
public extension Dipole {
    public RoundPoint anti_wedge(Sphere other) {
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4((this.e41 * other.e3215) + (this.e31 * other.e4125), (this.e42 * other.e3215) + (this.e12 * other.e4235), (this.e43 * other.e3215) + (this.e23 * other.e4315), -(this.e43 * other.e4125) - (this.e45 * other.e1234)) - (other.group0.yzxy * float4(this.group1.zxy, this.e42)) - float4((float3(other.e1234) * this.group2.xyz), this.e41 * other.e4235), 
            /* e5 */
            (this.e45 * other.e3215) + (this.e15 * other.e4235) + (this.e25 * other.e4315) + (this.e35 * other.e4125)
        );
    }
}
public extension Dipole {
    public AntiCircleRotor anti_wedge(VersorEven other) {
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(other.e12345) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(other.e12345) * this.group1, 
            /* e15, e25, e35, scalar */
            float4((float3(other.e12345) * this.group2.xyz), -(this.e41 * other.e235) - (this.e42 * other.e315) - (this.e43 * other.e125) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435) - (this.e45 * other.e321) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412))
        );
    }
}
public extension Dipole {
    public RoundPoint anti_wedge(VersorOdd other) {
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4((this.e41 * other.e3215) + (this.e31 * other.e4125), (this.e42 * other.e3215) + (this.e12 * other.e4235), (this.e43 * other.e3215) + (this.e23 * other.e4315), -(this.e43 * other.e4125) - (this.e45 * other.e1234)) - (other.group3.yzxy * float4(this.group1.zxy, this.e42)) - float4((this.group2.xyz * other.group2.www), this.e41 * other.e4235), 
            /* e5 */
            (this.e45 * other.e3215) + (this.e15 * other.e4235) + (this.e25 * other.e4315) + (this.e35 * other.e4125)
        );
    }
}
public extension DipoleInversion {
    // Fancy infix trick (first half)
    public AntiWedgeInfixPartial<DipoleInversion> operator /(AntiWedgeInfix rhs) {
        return AntiWedgeInfixPartial<DipoleInversion>(this);
    }
}
public extension AntiWedgeInfixPartial<DipoleInversion> {
    // Fancy infix trick (second half)
    public RoundPoint operator /(AntiCircleRotor rhs) {
        return this.a.anti_wedge(rhs);
    }
}
public extension DipoleInversion {
    public RoundPoint anti_wedge(AntiCircleRotor other) {
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(-(other.e41 * this.e3215) - (other.e31 * this.e4125), -(other.e42 * this.e3215) - (other.e12 * this.e4235), -(other.e43 * this.e3215) - (other.e23 * this.e4315), (other.e43 * this.e4125) + (other.e45 * this.e1234)) + (this.group3.yzxx * float4(other.group1.zxy, other.e41)) + float4((this.group2.www * other.group2.xyz), other.e42 * this.e4315), 
            /* e5 */
            -(other.e45 * this.e3215) - (other.e15 * this.e4235) - (other.e25 * this.e4315) - (other.e35 * this.e4125)
        );
    }
}
public extension DipoleInversion {
    public AntiCircleRotor anti_wedge(AntiDipoleInversion other) {
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (float3(this.e1234) * other.group1.xyz) + (other.group0.xyz.zxy * this.group3.yzx) - (other.group0.xyz.yzx * this.group3.zxy), 
            /* e23, e31, e12, e45 */
            float4((other.e423 * this.e3215) + (other.e235 * this.e1234), (other.e431 * this.e3215) + (other.e315 * this.e1234), (other.e412 * this.e3215) + (other.e125 * this.e1234), -(other.e425 * this.e4315) - (other.e435 * this.e4125)) - (other.group1.wwwx * this.group3.xyzx), 
            /* e15, e25, e35, scalar */
            (float4(this.e3215) * float4(other.group1.xyz, other.e4)) + (this.group3.zxyx * float4(other.group2.yzx, other.e1)) + float4(float3(0.0), (other.e2 * this.e4315) + (other.e3 * this.e4125) + (other.e5 * this.e1234) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e321 * this.e45) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43)) - float4((other.group2.zxy * this.group3.yzx), other.e423 * this.e15)
        );
    }
}
public extension DipoleInversion {
    public AntiFlector anti_wedge(AntiDualNum other) {
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(other.e3215) * float4(this.group3.xyz, this.e1234), 
            /* e1, e2, e3, e5 */
            float4(other.e3215) * float4(this.group0.xyz, this.e45)
        );
    }
}
public extension DipoleInversion {
    public AntiMotor anti_wedge(AntiFlatPoint other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(other.e235 * this.e1234, other.e315 * this.e1234, other.e125 * this.e1234, -(other.e315 * this.e42) - (other.e125 * this.e43) - (other.e321 * this.e45)) - float4((other.group0.www * this.group3.xyz), other.e235 * this.e41), 
            /* e15, e25, e35, e3215 */
            float4(((other.group0.yzx * this.group3.zxy) - (other.group0.zxy * this.group3.yzx)), 0.0)
        );
    }
}
public extension DipoleInversion {
    public AntiMotor anti_wedge(AntiFlector other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(float3(0.0), (other.e2 * this.e4315) + (other.e3 * this.e4125) + (other.e5 * this.e1234) - (other.e315 * this.e42) - (other.e125 * this.e43) - (other.e321 * this.e45)) + float4((this.group2.www * other.group0.xyz), other.e1 * this.e4235) - float4((other.group0.www * this.group3.xyz), other.e235 * this.e41), 
            /* e15, e25, e35, e3215 */
            float4(((other.group0.yzx * this.group3.zxy) - (other.group0.zxy * this.group3.yzx)), 0.0)
        );
    }
}
public extension DipoleInversion {
    public AntiPlane anti_wedge(AntiLine other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4((other.e12 * this.e4315) + (other.e15 * this.e1234), (other.e23 * this.e4125) + (other.e25 * this.e1234), (other.e31 * this.e4235) + (other.e35 * this.e1234), -(other.e25 * this.e4315) - (other.e35 * this.e4125)) - (this.group3.zxyx * float4(other.group0.xyz.yzx, other.e15))
        );
    }
}
public extension DipoleInversion {
    public AntiFlector anti_wedge(AntiMotor other) {
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(other.e3215) * float4(this.group3.xyz, this.e1234), 
            /* e1, e2, e3, e5 */
            float4((other.e12 * this.e4315) + (other.e15 * this.e1234), (other.e23 * this.e4125) + (other.e25 * this.e1234), (other.e31 * this.e4235) + (other.e35 * this.e1234), -(other.e25 * this.e4315) - (other.e35 * this.e4125)) + float4((this.group0.xyz * other.group1.www), other.e3215 * this.e45) - (this.group3.zxyx * float4(other.group0.yzx, other.e15))
        );
    }
}
public extension DipoleInversion {
    public Scalar anti_wedge(AntiPlane other) {
        return Scalar(
            /* scalar */
            (other.e1 * this.e4235) + (other.e2 * this.e4315) + (other.e3 * this.e4125) + (other.e5 * this.e1234)
        );
    }
}
public extension DipoleInversion {
    public DipoleInversion anti_wedge(AntiScalar other) {
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(other.e12345) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(other.e12345) * this.group1, 
            /* e15, e25, e35, e1234 */
            float4(other.e12345) * this.group2, 
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345) * this.group3
        );
    }
}
public extension DipoleInversion {
    public AntiCircleRotor anti_wedge(Circle other) {
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (float3(this.e1234) * other.group1.xyz) + (other.group0.xyz.zxy * this.group3.yzx) - (other.group0.xyz.yzx * this.group3.zxy), 
            /* e23, e31, e12, e45 */
            float4((other.e423 * this.e3215) + (other.e235 * this.e1234), (other.e431 * this.e3215) + (other.e315 * this.e1234), (other.e412 * this.e3215) + (other.e125 * this.e1234), -(other.e425 * this.e4315) - (other.e435 * this.e4125)) - (other.group1.wwwx * this.group3.xyzx), 
            /* e15, e25, e35, scalar */
            float4((other.e415 * this.e3215) + (other.e315 * this.e4125), (other.e425 * this.e3215) + (other.e125 * this.e4235), (other.e435 * this.e3215) + (other.e235 * this.e4315), -(other.e431 * this.e25) - (other.e412 * this.e35) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e321 * this.e45) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43)) - float4((other.group2.xyz.zxy * this.group3.yzx), other.e423 * this.e15)
        );
    }
}
public extension DipoleInversion {
    public VersorOdd anti_wedge(CircleRotor other) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4((other.e412 * this.e4315) + (other.e415 * this.e1234) + (other.e12345 * this.e41), (other.e423 * this.e4125) + (other.e425 * this.e1234) + (other.e12345 * this.e42), (other.e431 * this.e4235) + (other.e435 * this.e1234) + (other.e12345 * this.e43), -(other.e431 * this.e25) - (other.e412 * this.e35) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e321 * this.e45) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43)) - float4((other.group0.xyz.yzx * this.group3.zxy), other.e423 * this.e15), 
            /* e23, e31, e12, e45 */
            float4((other.e235 * this.e1234) + (other.e12345 * this.e23), (other.e315 * this.e1234) + (other.e12345 * this.e31), (other.e125 * this.e1234) + (other.e12345 * this.e12), -(other.e425 * this.e4315) - (other.e435 * this.e4125)) + float4((other.group0.xyz * this.group3.www), other.e12345 * this.e45) - (other.group1.wwwx * this.group3.xyzx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(other.e12345) * this.group2.xyz) + (float3(this.e3215) * other.group1.xyz) + (other.group2.yzx * this.group3.zxy) - (other.group2.zxy * this.group3.yzx)), other.e12345 * this.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345) * this.group3
        );
    }
}
public extension DipoleInversion {
    public RoundPoint anti_wedge(Dipole other) {
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(-(other.e41 * this.e3215) - (other.e31 * this.e4125), -(other.e42 * this.e3215) - (other.e12 * this.e4235), -(other.e43 * this.e3215) - (other.e23 * this.e4315), (other.e43 * this.e4125) + (other.e45 * this.e1234)) + (this.group3.yzxy * float4(other.group1.zxy, other.e42)) + float4((other.group2.xyz * this.group2.www), other.e41 * this.e4235), 
            /* e5 */
            -(other.e45 * this.e3215) - (other.e15 * this.e4235) - (other.e25 * this.e4315) - (other.e35 * this.e4125)
        );
    }
}
public extension DipoleInversion {
    public AntiDipoleInversion anti_wedge(DipoleInversion other) {
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (float3(this.e1234) * other.group3.xyz) - (float3(other.e1234) * this.group3.xyz), 
            /* e415, e425, e435, e321 */
            (other.group3.yzxw * float4(this.group3.zxy, this.e1234)) - (this.group3.yzxw * float4(other.group3.zxy, other.e1234)), 
            /* e235, e315, e125, e4 */
            (this.group3.xyzx * float4(other.group3.www, other.e41)) + float4(float3(0.0), (other.e42 * this.e4315) + (other.e43 * this.e4125) + (other.e45 * this.e1234) - (other.e1234 * this.e45) - (other.e4315 * this.e42) - (other.e4125 * this.e43)) - float4((this.group3.www * other.group3.xyz), other.e4235 * this.e41), 
            /* e1, e2, e3, e5 */
            (other.group3.zxyw * this.group1.yzxw) + (this.group2.wwwz * float4(other.group2.xyz, other.e4125)) + float4((this.group0.xyz * other.group3.www), other.e4235 * this.e15) + float4((other.group1.zxy * this.group3.yzx), other.e4315 * this.e25) - (float4(this.e3215) * float4(other.group0.xyz, other.e45)) - (other.group2.wwwy * float4(this.group2.xyz, this.e4315)) - (this.group3.zxyx * float4(other.group1.yzx, other.e15)) - float4((other.group3.yzx * this.group1.zxy), other.e35 * this.e4125)
        );
    }
}
public extension DipoleInversion {
    public VersorOdd anti_wedge(DualNum other) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(other.group0.xy.yy, other.e12345, other.e5) * float4(this.group0.xyz, this.e1234), 
            /* e23, e31, e12, e45 */
            float4(other.e12345) * this.group1, 
            /* e15, e25, e35, e1234 */
            float4(other.e12345) * this.group2, 
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345) * this.group3
        );
    }
}
public extension DipoleInversion {
    public RoundPoint anti_wedge(FlatPoint other) {
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(this.e1234) * other.group0, 
            /* e5 */
            -(this.e4235 * other.e15) - (this.e4315 * other.e25) - (this.e4125 * other.e35) - (this.e3215 * other.e45)
        );
    }
}
public extension DipoleInversion {
    public AntiDipoleInversion anti_wedge(Flector other) {
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(this.e1234) * other.group1.xyz, 
            /* e415, e425, e435, e321 */
            float4(((this.group3.zxy * other.group1.yzx) - (this.group3.yzx * other.group1.zxy)), this.e1234 * other.e3215), 
            /* e235, e315, e125, e4 */
            float4(float3(0.0), -(this.e42 * other.e4315) - (this.e43 * other.e4125)) + float4((other.group1.www * this.group3.xyz), this.e1234 * other.e45) - (other.group1.xyzx * float4(this.group3.www, this.e41)), 
            /* e1, e2, e3, e5 */
            (float4(other.e3215) * float4(this.group0.xyz, this.e45)) + (this.group2.wwwy * float4(other.group0.xyz, other.e4315)) + (other.group1.zxyx * float4(this.group1.yzx, this.e15)) + float4(float3(0.0), (this.e35 * other.e4125) - (this.e4315 * other.e25) - (this.e4125 * other.e35) - (this.e3215 * other.e45)) - float4((this.group1.zxy * other.group1.yzx), this.e4235 * other.e15)
        );
    }
}
public extension DipoleInversion {
    public AntiCircleRotor anti_wedge(Line other) {
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(this.e1234) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4((float3(this.e1234) * other.group1.xyz), -(this.e4235 * other.e415) - (this.e4315 * other.e425) - (this.e4125 * other.e435)), 
            /* e15, e25, e35, scalar */
            float4((this.e4125 * other.e315) + (this.e3215 * other.e415), (this.e4235 * other.e125) + (this.e3215 * other.e425), (this.e4315 * other.e235) + (this.e3215 * other.e435), -(this.e42 * other.e315) - (this.e43 * other.e125) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435)) - float4((other.group1.xyz.zxy * this.group3.yzx), this.e41 * other.e235)
        );
    }
}
public extension DipoleInversion {
    public VersorOdd anti_wedge(Motor other) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(this.e1234 * other.e415, this.e1234 * other.e425, this.e1234 * other.e435, -(this.e41 * other.e235) - (this.e42 * other.e315) - (this.e43 * other.e125) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435)) + float4((this.group0.xyz * other.group0.www), this.e1234 * other.e5), 
            /* e23, e31, e12, e45 */
            float4(this.e1234 * other.e235, this.e1234 * other.e315, this.e1234 * other.e125, -(this.e4235 * other.e415) - (this.e4315 * other.e425) - (this.e4125 * other.e435)) + (float4(other.e12345) * this.group1), 
            /* e15, e25, e35, e1234 */
            float4(((float3(this.e3215) * other.group0.xyz) + (float3(other.e12345) * this.group2.xyz) + (this.group3.zxy * other.group1.yzx) - (this.group3.yzx * other.group1.zxy)), this.e1234 * other.e12345), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345) * this.group3
        );
    }
}
public extension DipoleInversion {
    public MultiVector anti_wedge(MultiVector other) {
        return MultiVector(
            /* scalar, e12345 */
            float2((this.e1234 * other.e5) + (this.e4235 * other.e1) + (this.e4315 * other.e2) + (this.e4125 * other.e3) + (this.e3215 * other.e4) - (this.e41 * other.e235) - (this.e42 * other.e315) - (this.e43 * other.e125) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435) - (this.e45 * other.e321) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412), 0.0), 
            /* e1, e2, e3, e4 */
            (float4(this.e1234) * other.group3) + (this.group3.yzxy * float4(other.group5.xyz.zxy, other.e42)) + float4((this.group0.xyz * other.group9.www), this.e4235 * other.e41) + float4((this.group1.yzx * other.group9.zxy), this.e4125 * other.e43) - (float4(other.e1234) * float4(this.group2.xyz, this.e45)) - (other.group9.yzxz * float4(this.group1.zxy, this.e43)) - float4((other.group4.xyz * this.group3.www), this.e41 * other.e4235) - float4((other.group5.xyz.yzx * this.group3.zxy), this.e42 * other.e4315), 
            /* e5 */
            (this.e45 * other.e3215) + (this.e15 * other.e4235) + (this.e25 * other.e4315) + (this.e35 * other.e4125) - (this.e4235 * other.e15) - (this.e4315 * other.e25) - (this.e4125 * other.e35) - (this.e3215 * other.e45), 
            /* e15, e25, e35, e45 */
            float4((this.e4125 * other.e315) + (this.e3215 * other.e415), (this.e4235 * other.e125) + (this.e3215 * other.e425), (this.e4315 * other.e235) + (this.e3215 * other.e435), -(this.e4315 * other.e425) - (this.e4125 * other.e435)) + (float4(other.e12345) * float4(this.group2.xyz, this.e45)) - float4((other.group8.xyz.zxy * this.group3.yzx), this.e4235 * other.e415), 
            /* e41, e42, e43 */
            (float3(this.e1234) * other.group6.xyz) + (float3(other.e12345) * this.group0.xyz) + (other.group7.xyz.zxy * this.group3.yzx) - (other.group7.xyz.yzx * this.group3.zxy), 
            /* e23, e31, e12 */
            (float3(this.e1234) * other.group8.xyz) + (float3(this.e3215) * other.group7.xyz) + (float3(other.e12345) * this.group1.xyz) - (float3(other.e321) * this.group3.xyz), 
            /* e415, e425, e435, e321 */
            (other.group9.yzxw * float4(this.group3.zxy, this.e1234)) - (this.group3.yzxw * float4(other.group9.zxy, other.e1234)), 
            /* e423, e431, e412 */
            (float3(this.e1234) * other.group9.xyz) - (float3(other.e1234) * this.group3.xyz), 
            /* e235, e315, e125 */
            (float3(other.e3215) * this.group3.xyz) - (float3(this.e3215) * other.group9.xyz), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345) * this.group3, 
            /* e1234 */
            this.e1234 * other.e12345
        );
    }
}
public extension DipoleInversion {
    public AntiDipoleInversion anti_wedge(Plane other) {
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(this.e1234) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(((this.group3.zxy * other.group0.yzx) - (this.group3.yzx * other.group0.zxy)), this.e1234 * other.e3215), 
            /* e235, e315, e125, e4 */
            float4(this.e4235 * other.e3215, this.e4315 * other.e3215, this.e4125 * other.e3215, -(this.e42 * other.e4315) - (this.e43 * other.e4125)) - (other.group0.xyzx * float4(this.group3.www, this.e41)), 
            /* e1, e2, e3, e5 */
            float4(this.e12 * other.e4315 * -1.0, this.e23 * other.e4125 * -1.0, this.e31 * other.e4235 * -1.0, (this.e25 * other.e4315) + (this.e35 * other.e4125)) + (float4(other.e3215) * float4(this.group0.xyz, this.e45)) + (other.group0.zxyx * float4(this.group1.yzx, this.e15))
        );
    }
}
public extension DipoleInversion {
    public Scalar anti_wedge(RoundPoint other) {
        return Scalar(
            /* scalar */
            (this.e1234 * other.e5) + (this.e4235 * other.e1) + (this.e4315 * other.e2) + (this.e4125 * other.e3) + (this.e3215 * other.e4)
        );
    }
}
public extension DipoleInversion {
    public AntiDipoleInversion anti_wedge(Sphere other) {
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (float3(this.e1234) * other.group0.xyz) - (float3(other.e1234) * this.group3.xyz), 
            /* e415, e425, e435, e321 */
            (other.group0.yzxw * float4(this.group3.zxy, this.e1234)) - (this.group3.yzxw * float4(other.group0.zxy, other.e1234)), 
            /* e235, e315, e125, e4 */
            float4(this.e4235 * other.e3215, this.e4315 * other.e3215, this.e4125 * other.e3215, -(this.e42 * other.e4315) - (this.e43 * other.e4125) - (this.e45 * other.e1234)) - (other.group0.xyzx * float4(this.group3.www, this.e41)), 
            /* e1, e2, e3, e5 */
            float4(-(this.e12 * other.e4315) - (this.e15 * other.e1234), -(this.e23 * other.e4125) - (this.e25 * other.e1234), -(this.e31 * other.e4235) - (this.e35 * other.e1234), (this.e25 * other.e4315) + (this.e35 * other.e4125)) + (float4(other.e3215) * float4(this.group0.xyz, this.e45)) + (other.group0.zxyx * float4(this.group1.yzx, this.e15))
        );
    }
}
public extension DipoleInversion {
    public VersorOdd anti_wedge(VersorEven other) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (this.group3.yzxy * float4(other.group0.zxy, other.e2)) + float4(float3(0.0), (this.e4125 * other.e3) + (this.e3215 * other.e4) - (this.e42 * other.e315) - (this.e43 * other.e125) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435) - (this.e45 * other.e321) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412)) + float4((this.group0.xyz * other.group0.www), this.e1234 * other.e5) + float4((this.group2.www * other.group1.xyz), this.e4235 * other.e1) - float4((this.group3.zxy * other.group0.yzx), this.e41 * other.e235), 
            /* e23, e31, e12, e45 */
            float4((this.e1234 * other.e235) + (this.e3215 * other.e423), (this.e1234 * other.e315) + (this.e3215 * other.e431), (this.e1234 * other.e125) + (this.e3215 * other.e412), -(this.e4315 * other.e425) - (this.e4125 * other.e435)) + (float4(other.e12345) * this.group1) - (this.group3.xyzx * other.group1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(this.e3215) * other.group1.xyz) + (float3(other.e12345) * this.group2.xyz) + (this.group3.zxy * other.group2.yzx) - (this.group3.yzx * other.group2.zxy)), this.e1234 * other.e12345), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345) * this.group3
        );
    }
}
public extension DipoleInversion {
    public AntiDipoleInversion anti_wedge(VersorOdd other) {
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (float3(this.e1234) * other.group3.xyz) - (float3(other.e1234) * this.group3.xyz), 
            /* e415, e425, e435, e321 */
            (other.group3.yzxw * float4(this.group3.zxy, this.e1234)) - (this.group3.yzxw * float4(other.group3.zxy, other.e1234)), 
            /* e235, e315, e125, e4 */
            float4(float3(0.0), (this.e4235 * other.e41) + (this.e4315 * other.e42) + (this.e4125 * other.e43) - (this.e42 * other.e4315) - (this.e43 * other.e4125) - (this.e45 * other.e1234)) + float4((other.group3.www * this.group3.xyz), this.e1234 * other.e45) - (other.group3.xyzx * float4(this.group3.www, this.e41)), 
            /* e1, e2, e3, e5 */
            (float4(other.e3215) * float4(this.group0.xyz, this.e45)) + (this.group2.wwwy * float4(other.group2.xyz, other.e4315)) + (other.group3.zxyx * float4(this.group1.yzx, this.e15)) + float4((this.group3.yzx * other.group1.zxy), this.e35 * other.e4125) - (float4(this.e3215) * float4(other.group0.xyz, other.e45)) - (this.group3.zxyz * float4(other.group1.yzx, other.e35)) - (other.group2.wwwy * float4(this.group2.xyz, this.e4315)) - float4((this.group1.zxy * other.group3.yzx), this.e4235 * other.e15)
        );
    }
}
public extension DualNum {
    // Fancy infix trick (first half)
    public AntiWedgeInfixPartial<DualNum> operator /(AntiWedgeInfix rhs) {
        return AntiWedgeInfixPartial<DualNum>(this);
    }
}
public extension AntiWedgeInfixPartial<DualNum> {
    // Fancy infix trick (second half)
    public AntiCircleRotor operator /(AntiCircleRotor rhs) {
        return this.a.anti_wedge(rhs);
    }
}
public extension DualNum {
    public AntiCircleRotor anti_wedge(AntiCircleRotor other) {
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(this.e12345) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(this.e12345) * other.group1, 
            /* e15, e25, e35, scalar */
            float4(this.e12345) * other.group2
        );
    }
}
public extension DualNum {
    public AntiDipoleInversion anti_wedge(AntiDipoleInversion other) {
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(this.e12345) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(this.e12345) * other.group1, 
            /* e235, e315, e125, e4 */
            float4(this.e12345) * other.group2, 
            /* e1, e2, e3, e5 */
            float4(this.e12345) * other.group3
        );
    }
}
public extension DualNum {
    public AntiDualNum anti_wedge(AntiDualNum other) {
        return AntiDualNum(
            /* e3215, scalar */
            float2(this.e12345) * other.group0.xy
        );
    }
}
public extension DualNum {
    public AntiFlatPoint anti_wedge(AntiFlatPoint other) {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(this.e12345) * other.group0
        );
    }
}
public extension DualNum {
    public AntiFlector anti_wedge(AntiFlector other) {
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(this.e12345) * other.group0, 
            /* e1, e2, e3, e5 */
            float4(this.e12345) * other.group1
        );
    }
}
public extension DualNum {
    public AntiLine anti_wedge(AntiLine other) {
        return AntiLine(
            /* e23, e31, e12 */
            float3(this.e12345) * other.group0.xyz, 
            /* e15, e25, e35 */
            float3(this.e12345) * other.group1.xyz
        );
    }
}
public extension DualNum {
    public AntiMotor anti_wedge(AntiMotor other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(this.e12345) * other.group0, 
            /* e15, e25, e35, e3215 */
            float4(this.e12345) * other.group1
        );
    }
}
public extension DualNum {
    public AntiPlane anti_wedge(AntiPlane other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(this.e12345) * other.group0
        );
    }
}
public extension DualNum {
    public DualNum anti_wedge(AntiScalar other) {
        return DualNum(
            /* e5, e12345 */
            float2(other.e12345) * this.group0.xy
        );
    }
}
public extension DualNum {
    public Circle anti_wedge(Circle other) {
        return Circle(
            /* e423, e431, e412 */
            float3(this.e12345) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(this.e12345) * other.group1, 
            /* e235, e315, e125 */
            float3(this.e12345) * other.group2.xyz
        );
    }
}
public extension DualNum {
    public VersorEven anti_wedge(CircleRotor other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(this.e12345) * float4(other.group0.xyz, other.e12345), 
            /* e415, e425, e435, e321 */
            float4(this.e12345) * other.group1, 
            /* e235, e315, e125, e5 */
            other.group2 * float4(this.group0.xy.yy, this.e12345, this.e5), 
            /* e1, e2, e3, e4 */
            float4(0.0)
        );
    }
}
public extension DualNum {
    public Dipole anti_wedge(Dipole other) {
        return Dipole(
            /* e41, e42, e43 */
            float3(this.e12345) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(this.e12345) * other.group1, 
            /* e15, e25, e35 */
            float3(this.e12345) * other.group2.xyz
        );
    }
}
public extension DualNum {
    public VersorOdd anti_wedge(DipoleInversion other) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(this.group0.xy.yy, this.e12345, this.e5) * float4(other.group0.xyz, other.e1234), 
            /* e23, e31, e12, e45 */
            float4(this.e12345) * other.group1, 
            /* e15, e25, e35, e1234 */
            float4(this.e12345) * other.group2, 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * other.group3
        );
    }
}
public extension DualNum {
    public DualNum anti_wedge(DualNum other) {
        return DualNum(
            /* e5, e12345 */
            float2((other.e5 * this.e12345) + (other.e12345 * this.e5), other.e12345 * this.e12345)
        );
    }
}
public extension DualNum {
    public FlatPoint anti_wedge(FlatPoint other) {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(this.e12345) * other.group0
        );
    }
}
public extension DualNum {
    public Flector anti_wedge(Flector other) {
        return Flector(
            /* e15, e25, e35, e45 */
            float4(this.e12345) * other.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * other.group1
        );
    }
}
public extension DualNum {
    public Line anti_wedge(Line other) {
        return Line(
            /* e415, e425, e435 */
            float3(this.e12345) * other.group0.xyz, 
            /* e235, e315, e125 */
            float3(this.e12345) * other.group1.xyz
        );
    }
}
public extension DualNum {
    public Motor anti_wedge(Motor other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(this.e12345) * other.group0, 
            /* e235, e315, e125, e5 */
            float4(this.group0.xy.yy, this.e12345, (this.e5 * other.e12345) + (this.e12345 * other.e5)) * float4(other.group1.xyz, 1.0)
        );
    }
}
public extension DualNum {
    public MultiVector anti_wedge(MultiVector other) {
        return MultiVector(
            /* scalar, e12345 */
            float2((this.e5 * other.e1234) + (this.e12345 * other.scalar), this.e12345 * other.e12345), 
            /* e1, e2, e3, e4 */
            float4(this.e12345) * other.group1, 
            /* e5 */
            (this.e5 * other.e12345) + (this.e12345 * other.e5), 
            /* e15, e25, e35, e45 */
            float4(this.e12345) * other.group3, 
            /* e41, e42, e43 */
            float3(this.e12345) * other.group4.xyz, 
            /* e23, e31, e12 */
            float3(this.e12345) * other.group5.xyz, 
            /* e415, e425, e435, e321 */
            float4(this.e12345) * other.group6, 
            /* e423, e431, e412 */
            float3(this.e12345) * other.group7.xyz, 
            /* e235, e315, e125 */
            float3(this.e12345) * other.group8.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * other.group9, 
            /* e1234 */
            this.e12345 * other.e1234
        );
    }
}
public extension DualNum {
    public Plane anti_wedge(Plane other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * other.group0
        );
    }
}
public extension DualNum {
    public RoundPoint anti_wedge(RoundPoint other) {
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(this.e12345) * other.group0, 
            /* e5 */
            this.e12345 * other.e5
        );
    }
}
public extension DualNum {
    public Scalar anti_wedge(Scalar other) {
        return Scalar(
            /* scalar */
            this.e12345 * other.scalar
        );
    }
}
public extension DualNum {
    public VersorOdd anti_wedge(Sphere other) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(float3(0.0), this.e5 * other.e1234), 
            /* e23, e31, e12, e45 */
            float4(0.0), 
            /* e15, e25, e35, e1234 */
            float4(float3(0.0), this.e12345 * other.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * other.group0
        );
    }
}
public extension DualNum {
    public VersorEven anti_wedge(VersorEven other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(this.e12345) * other.group0, 
            /* e415, e425, e435, e321 */
            float4(this.e12345) * other.group1, 
            /* e235, e315, e125, e5 */
            float4(this.group0.xy.yy, this.e12345, (this.e5 * other.e12345) + (this.e12345 * other.e5)) * float4(other.group2.xyz, 1.0), 
            /* e1, e2, e3, e4 */
            float4(this.e12345) * other.group3
        );
    }
}
public extension DualNum {
    public VersorOdd anti_wedge(VersorOdd other) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(this.group0.xy.yy, this.e12345, (this.e5 * other.e1234) + (this.e12345 * other.scalar)) * float4(other.group0.xyz, 1.0), 
            /* e23, e31, e12, e45 */
            float4(this.e12345) * other.group1, 
            /* e15, e25, e35, e1234 */
            float4(this.e12345) * other.group2, 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * other.group3
        );
    }
}
public extension FlatPoint {
    // Fancy infix trick (first half)
    public AntiWedgeInfixPartial<FlatPoint> operator /(AntiWedgeInfix rhs) {
        return AntiWedgeInfixPartial<FlatPoint>(this);
    }
}
public extension AntiWedgeInfixPartial<FlatPoint> {
    // Fancy infix trick (second half)
    public Scalar operator /(AntiDipoleInversion rhs) {
        return this.a.anti_wedge(rhs);
    }
}
public extension FlatPoint {
    public Scalar anti_wedge(AntiDipoleInversion other) {
        return Scalar(
            /* scalar */
            -(other.e423 * this.e15) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e321 * this.e45)
        );
    }
}
public extension FlatPoint {
    public DualNum anti_wedge(AntiDualNum other) {
        return DualNum(
            /* e5, e12345 */
            float2(other.e3215 * this.e45, 1.0) * float2(1.0, 0.0)
        );
    }
}
public extension FlatPoint {
    public Scalar anti_wedge(AntiFlatPoint other) {
        return Scalar(
            /* scalar */
            other.e321 * this.e45 * -1.0
        );
    }
}
public extension FlatPoint {
    public Scalar anti_wedge(AntiFlector other) {
        return Scalar(
            /* scalar */
            other.e321 * this.e45 * -1.0
        );
    }
}
public extension FlatPoint {
    public DualNum anti_wedge(AntiMotor other) {
        return DualNum(
            /* e5, e12345 */
            float2(other.e3215 * this.e45, 1.0) * float2(1.0, 0.0)
        );
    }
}
public extension FlatPoint {
    public FlatPoint anti_wedge(AntiScalar other) {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(other.e12345) * this.group0
        );
    }
}
public extension FlatPoint {
    public Scalar anti_wedge(Circle other) {
        return Scalar(
            /* scalar */
            -(other.e423 * this.e15) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e321 * this.e45)
        );
    }
}
public extension FlatPoint {
    public AntiCircleRotor anti_wedge(CircleRotor other) {
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), other.e12345 * this.e45), 
            /* e15, e25, e35, scalar */
            float4((other.group2.www * this.group0.xyz), -(other.e423 * this.e15) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e321 * this.e45))
        );
    }
}
public extension FlatPoint {
    public RoundPoint anti_wedge(DipoleInversion other) {
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(other.e1234) * this.group0 * float4(-1.0), 
            /* e5 */
            (other.e4235 * this.e15) + (other.e4315 * this.e25) + (other.e4125 * this.e35) + (other.e3215 * this.e45)
        );
    }
}
public extension FlatPoint {
    public FlatPoint anti_wedge(DualNum other) {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(other.e12345) * this.group0
        );
    }
}
public extension FlatPoint {
    public DualNum anti_wedge(Flector other) {
        return DualNum(
            /* e5, e12345 */
            float2((this.e15 * other.e4235) + (this.e25 * other.e4315) + (this.e35 * other.e4125) + (this.e45 * other.e3215), 0.0)
        );
    }
}
public extension FlatPoint {
    public FlatPoint anti_wedge(Motor other) {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(other.e12345) * this.group0
        );
    }
}
public extension FlatPoint {
    public MultiVector anti_wedge(MultiVector other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(-(this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412) - (this.e45 * other.e321), 0.0), 
            /* e1, e2, e3, e4 */
            float4(other.e1234) * this.group0 * float4(-1.0), 
            /* e5 */
            (this.e15 * other.e4235) + (this.e25 * other.e4315) + (this.e35 * other.e4125) + (this.e45 * other.e3215), 
            /* e15, e25, e35, e45 */
            float4(other.e12345) * this.group0, 
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
public extension FlatPoint {
    public DualNum anti_wedge(Plane other) {
        return DualNum(
            /* e5, e12345 */
            float2((this.e15 * other.e4235) + (this.e25 * other.e4315) + (this.e35 * other.e4125) + (this.e45 * other.e3215), 0.0)
        );
    }
}
public extension FlatPoint {
    public RoundPoint anti_wedge(Sphere other) {
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(other.e1234) * this.group0 * float4(-1.0), 
            /* e5 */
            (this.e15 * other.e4235) + (this.e25 * other.e4315) + (this.e35 * other.e4125) + (this.e45 * other.e3215)
        );
    }
}
public extension FlatPoint {
    public AntiCircleRotor anti_wedge(VersorEven other) {
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), this.e45 * other.e12345), 
            /* e15, e25, e35, scalar */
            float4((float3(other.e12345) * this.group0.xyz), -(this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412) - (this.e45 * other.e321))
        );
    }
}
public extension FlatPoint {
    public RoundPoint anti_wedge(VersorOdd other) {
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(other.e1234) * this.group0 * float4(-1.0), 
            /* e5 */
            (this.e15 * other.e4235) + (this.e25 * other.e4315) + (this.e35 * other.e4125) + (this.e45 * other.e3215)
        );
    }
}
public extension Flector {
    // Fancy infix trick (first half)
    public AntiWedgeInfixPartial<Flector> operator /(AntiWedgeInfix rhs) {
        return AntiWedgeInfixPartial<Flector>(this);
    }
}
public extension AntiWedgeInfixPartial<Flector> {
    // Fancy infix trick (second half)
    public RoundPoint operator /(AntiCircleRotor rhs) {
        return this.a.anti_wedge(rhs);
    }
}
public extension Flector {
    public RoundPoint anti_wedge(AntiCircleRotor other) {
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(-(other.e41 * this.e3215) - (other.e31 * this.e4125), -(other.e42 * this.e3215) - (other.e12 * this.e4235), -(other.e43 * this.e3215) - (other.e23 * this.e4315), (other.e42 * this.e4315) + (other.e43 * this.e4125)) + (this.group1.yzxx * float4(other.group1.zxy, other.e41)), 
            /* e5 */
            -(other.e45 * this.e3215) - (other.e15 * this.e4235) - (other.e25 * this.e4315) - (other.e35 * this.e4125)
        );
    }
}
public extension Flector {
    public AntiCircleRotor anti_wedge(AntiDipoleInversion other) {
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (other.group0.xyz.zxy * this.group1.yzx) - (other.group0.xyz.yzx * this.group1.zxy), 
            /* e23, e31, e12, e45 */
            float4(other.e423 * this.e3215, other.e431 * this.e3215, other.e412 * this.e3215, -(other.e425 * this.e4315) - (other.e435 * this.e4125)) - (other.group1.wwwx * this.group1.xyzx), 
            /* e15, e25, e35, scalar */
            (float4(this.e3215) * float4(other.group1.xyz, other.e4)) + (this.group1.zxyx * float4(other.group2.yzx, other.e1)) + float4(float3(0.0), (other.e2 * this.e4315) + (other.e3 * this.e4125) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e321 * this.e45)) - float4((other.group2.zxy * this.group1.yzx), other.e423 * this.e15)
        );
    }
}
public extension Flector {
    public AntiFlector anti_wedge(AntiDualNum other) {
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(other.group0.xy.xx, other.e3215, 0.0) * float4(float3(1.0), 0.0) * float4(this.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), other.e3215 * this.e45)
        );
    }
}
public extension Flector {
    public AntiMotor anti_wedge(AntiFlatPoint other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(other.e321) * float4(this.group1.xyz, this.e45) * float4(-1.0), 
            /* e15, e25, e35, e3215 */
            float4(((other.group0.yzx * this.group1.zxy) - (other.group0.zxy * this.group1.yzx)), 0.0)
        );
    }
}
public extension Flector {
    public AntiMotor anti_wedge(AntiFlector other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((other.group0.www * this.group1.xyz * float3(-1.0)), (other.e1 * this.e4235) + (other.e2 * this.e4315) + (other.e3 * this.e4125) - (other.e321 * this.e45)), 
            /* e15, e25, e35, e3215 */
            float4(((other.group0.yzx * this.group1.zxy) - (other.group0.zxy * this.group1.yzx)), 0.0)
        );
    }
}
public extension Flector {
    public AntiPlane anti_wedge(AntiLine other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(other.e12 * this.e4315, other.e23 * this.e4125, other.e31 * this.e4235, -(other.e25 * this.e4315) - (other.e35 * this.e4125)) - (this.group1.zxyx * float4(other.group0.xyz.yzx, other.e15))
        );
    }
}
public extension Flector {
    public AntiFlector anti_wedge(AntiMotor other) {
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(float3(1.0), 0.0) * float4(other.group1.www, 0.0) * float4(this.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), -(other.e25 * this.e4315) - (other.e35 * this.e4125)) + float4((other.group0.zxy * this.group1.yzx), other.e3215 * this.e45) - (this.group1.zxyx * float4(other.group0.yzx, other.e15))
        );
    }
}
public extension Flector {
    public Scalar anti_wedge(AntiPlane other) {
        return Scalar(
            /* scalar */
            (other.e1 * this.e4235) + (other.e2 * this.e4315) + (other.e3 * this.e4125)
        );
    }
}
public extension Flector {
    public Flector anti_wedge(AntiScalar other) {
        return Flector(
            /* e15, e25, e35, e45 */
            float4(other.e12345) * this.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345) * this.group1
        );
    }
}
public extension Flector {
    public AntiCircleRotor anti_wedge(Circle other) {
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (other.group0.xyz.zxy * this.group1.yzx) - (other.group0.xyz.yzx * this.group1.zxy), 
            /* e23, e31, e12, e45 */
            float4(other.e423 * this.e3215, other.e431 * this.e3215, other.e412 * this.e3215, -(other.e425 * this.e4315) - (other.e435 * this.e4125)) - (other.group1.wwwx * this.group1.xyzx), 
            /* e15, e25, e35, scalar */
            float4((other.e415 * this.e3215) + (other.e315 * this.e4125), (other.e425 * this.e3215) + (other.e125 * this.e4235), (other.e435 * this.e3215) + (other.e235 * this.e4315), -(other.e431 * this.e25) - (other.e412 * this.e35) - (other.e321 * this.e45)) - float4((other.group2.xyz.zxy * this.group1.yzx), other.e423 * this.e15)
        );
    }
}
public extension Flector {
    public VersorOdd anti_wedge(CircleRotor other) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(other.e412 * this.e4315, other.e423 * this.e4125, other.e431 * this.e4235, -(other.e431 * this.e25) - (other.e412 * this.e35) - (other.e321 * this.e45)) - float4((other.group0.xyz.yzx * this.group1.zxy), other.e423 * this.e15), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), -(other.e425 * this.e4315) - (other.e435 * this.e4125)) + float4((other.group0.xyz * this.group1.www), other.e12345 * this.e45) - (other.group1.wwwx * this.group1.xyzx), 
            /* e15, e25, e35, e1234 */
            float4((float3(other.e12345) * this.group0.xyz), 0.0) + float4((float3(this.e3215) * other.group1.xyz), 0.0) + float4((other.group2.yzx * this.group1.zxy), 0.0) - float4((other.group2.zxy * this.group1.yzx), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345) * this.group1
        );
    }
}
public extension Flector {
    public RoundPoint anti_wedge(Dipole other) {
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(-(other.e41 * this.e3215) - (other.e31 * this.e4125), -(other.e42 * this.e3215) - (other.e12 * this.e4235), -(other.e43 * this.e3215) - (other.e23 * this.e4315), (other.e42 * this.e4315) + (other.e43 * this.e4125)) + (this.group1.yzxx * float4(other.group1.zxy, other.e41)), 
            /* e5 */
            -(other.e45 * this.e3215) - (other.e15 * this.e4235) - (other.e25 * this.e4315) - (other.e35 * this.e4125)
        );
    }
}
public extension Flector {
    public AntiDipoleInversion anti_wedge(DipoleInversion other) {
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(other.e1234) * this.group1.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            float4(((other.group3.yzx * this.group1.zxy) - (other.group3.zxy * this.group1.yzx)), other.e1234 * this.e3215 * -1.0), 
            /* e235, e315, e125, e4 */
            (this.group1.xyzx * float4(other.group3.www, other.e41)) + float4(float3(0.0), (other.e42 * this.e4315) + (other.e43 * this.e4125)) - float4((this.group1.www * other.group3.xyz), other.e1234 * this.e45), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), (other.e4315 * this.e25) + (other.e4125 * this.e35) + (other.e3215 * this.e45) - (other.e35 * this.e4125)) + float4((other.group1.zxy * this.group1.yzx), other.e4235 * this.e15) - (float4(this.e3215) * float4(other.group0.xyz, other.e45)) - (other.group2.wwwy * float4(this.group0.xyz, this.e4315)) - (this.group1.zxyx * float4(other.group1.yzx, other.e15))
        );
    }
}
public extension Flector {
    public Flector anti_wedge(DualNum other) {
        return Flector(
            /* e15, e25, e35, e45 */
            float4(other.e12345) * this.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345) * this.group1
        );
    }
}
public extension Flector {
    public DualNum anti_wedge(FlatPoint other) {
        return DualNum(
            /* e5, e12345 */
            float2(-(other.e15 * this.e4235) - (other.e25 * this.e4315) - (other.e35 * this.e4125) - (other.e45 * this.e3215), 0.0)
        );
    }
}
public extension Flector {
    public Motor anti_wedge(Flector other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(((other.group1.yzx * this.group1.zxy) - (other.group1.zxy * this.group1.yzx)), 0.0), 
            /* e235, e315, e125, e5 */
            (other.group1.wwwx * float4(this.group1.xyz, this.e15)) + float4(float3(0.0), (other.e4315 * this.e25) + (other.e4125 * this.e35) + (other.e3215 * this.e45) - (other.e25 * this.e4315) - (other.e35 * this.e4125) - (other.e45 * this.e3215)) - (this.group1.wwwx * float4(other.group1.xyz, other.e15))
        );
    }
}
public extension Flector {
    public FlatPoint anti_wedge(Line other) {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4((this.e4125 * other.e315) + (this.e3215 * other.e415), (this.e4235 * other.e125) + (this.e3215 * other.e425), (this.e4315 * other.e235) + (this.e3215 * other.e435), -(this.e4315 * other.e425) - (this.e4125 * other.e435)) - (this.group1.yzxx * float4(other.group1.xyz.zxy, other.e415))
        );
    }
}
public extension Flector {
    public Flector anti_wedge(Motor other) {
        return Flector(
            /* e15, e25, e35, e45 */
            float4((this.e4125 * other.e315) + (this.e3215 * other.e415), (this.e4235 * other.e125) + (this.e3215 * other.e425), (this.e4315 * other.e235) + (this.e3215 * other.e435), -(this.e4315 * other.e425) - (this.e4125 * other.e435)) + (float4(other.e12345) * this.group0) - (this.group1.yzxx * float4(other.group1.zxy, other.e415)), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345) * this.group1
        );
    }
}
public extension Flector {
    public MultiVector anti_wedge(MultiVector other) {
        return MultiVector(
            /* scalar, e12345 */
            float2((this.e4235 * other.e1) + (this.e4315 * other.e2) + (this.e4125 * other.e3) + (this.e3215 * other.e4) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412) - (this.e45 * other.e321), 0.0), 
            /* e1, e2, e3, e4 */
            float4(-(this.e15 * other.e1234) - (this.e4125 * other.e31), -(this.e25 * other.e1234) - (this.e4235 * other.e12), -(this.e35 * other.e1234) - (this.e4315 * other.e23), (this.e4315 * other.e42) + (this.e4125 * other.e43)) + (this.group1.yzxx * float4(other.group5.xyz.zxy, other.e41)) - float4((other.group4.xyz * this.group1.www), this.e45 * other.e1234), 
            /* e5 */
            (this.e15 * other.e4235) + (this.e25 * other.e4315) + (this.e35 * other.e4125) + (this.e45 * other.e3215) - (this.e4235 * other.e15) - (this.e4315 * other.e25) - (this.e4125 * other.e35) - (this.e3215 * other.e45), 
            /* e15, e25, e35, e45 */
            float4((this.e4125 * other.e315) + (this.e3215 * other.e415), (this.e4235 * other.e125) + (this.e3215 * other.e425), (this.e4315 * other.e235) + (this.e3215 * other.e435), -(this.e4315 * other.e425) - (this.e4125 * other.e435)) + (float4(other.e12345) * this.group0) - float4((other.group8.xyz.zxy * this.group1.yzx), this.e4235 * other.e415), 
            /* e41, e42, e43 */
            (other.group7.xyz.zxy * this.group1.yzx) - (other.group7.xyz.yzx * this.group1.zxy), 
            /* e23, e31, e12 */
            (float3(this.e3215) * other.group7.xyz) - (float3(other.e321) * this.group1.xyz), 
            /* e415, e425, e435, e321 */
            float4(((this.group1.zxy * other.group9.yzx) - (this.group1.yzx * other.group9.zxy)), this.e3215 * other.e1234 * -1.0), 
            /* e423, e431, e412 */
            float3(other.e1234) * this.group1.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            (float3(other.e3215) * this.group1.xyz) - (float3(this.e3215) * other.group9.xyz), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345) * this.group1, 
            /* e1234 */
            0.0
        );
    }
}
public extension Flector {
    public Motor anti_wedge(Plane other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(((this.group1.zxy * other.group0.yzx) - (this.group1.yzx * other.group0.zxy)), 0.0), 
            /* e235, e315, e125, e5 */
            float4(this.e3215 * other.e4235 * -1.0, this.e3215 * other.e4315 * -1.0, this.e3215 * other.e4125 * -1.0, (this.e25 * other.e4315) + (this.e35 * other.e4125) + (this.e45 * other.e3215)) + (other.group0.wwwx * float4(this.group1.xyz, this.e15))
        );
    }
}
public extension Flector {
    public Scalar anti_wedge(RoundPoint other) {
        return Scalar(
            /* scalar */
            (this.e4235 * other.e1) + (this.e4315 * other.e2) + (this.e4125 * other.e3) + (this.e3215 * other.e4)
        );
    }
}
public extension Flector {
    public AntiDipoleInversion anti_wedge(Sphere other) {
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(other.e1234) * this.group1.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            float4(((this.group1.zxy * other.group0.yzx) - (this.group1.yzx * other.group0.zxy)), this.e3215 * other.e1234 * -1.0), 
            /* e235, e315, e125, e4 */
            float4(((float3(other.e3215) * this.group1.xyz) - (float3(this.e3215) * other.group0.xyz)), this.e45 * other.e1234 * -1.0), 
            /* e1, e2, e3, e5 */
            float4((float3(other.e1234) * this.group0.xyz * float3(-1.0)), (this.e15 * other.e4235) + (this.e25 * other.e4315) + (this.e35 * other.e4125) + (this.e45 * other.e3215))
        );
    }
}
public extension Flector {
    public VersorOdd anti_wedge(VersorEven other) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (this.group1.yzxx * float4(other.group0.zxy, other.e1)) + float4(float3(0.0), (this.e4315 * other.e2) + (this.e4125 * other.e3) + (this.e3215 * other.e4) - (this.e25 * other.e431) - (this.e35 * other.e412) - (this.e45 * other.e321)) - (other.group0.yzxx * float4(this.group1.zxy, this.e15)), 
            /* e23, e31, e12, e45 */
            (other.group0 * float4(this.group1.www, this.e45)) + float4(float3(0.0), -(this.e4315 * other.e425) - (this.e4125 * other.e435)) - (this.group1.xyzx * other.group1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4((float3(this.e3215) * other.group1.xyz), 0.0) + float4((float3(other.e12345) * this.group0.xyz), 0.0) + float4((this.group1.zxy * other.group2.yzx), 0.0) - float4((this.group1.yzx * other.group2.zxy), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345) * this.group1
        );
    }
}
public extension Flector {
    public AntiDipoleInversion anti_wedge(VersorOdd other) {
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(other.e1234) * this.group1.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            float4(((this.group1.zxy * other.group3.yzx) - (this.group1.yzx * other.group3.zxy)), this.e3215 * other.e1234 * -1.0), 
            /* e235, e315, e125, e4 */
            (this.group1.xyzx * float4(other.group3.www, other.e41)) + float4(float3(0.0), (this.e4315 * other.e42) + (this.e4125 * other.e43)) - float4((this.group1.www * other.group3.xyz), this.e45 * other.e1234), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), (this.e25 * other.e4315) + (this.e35 * other.e4125) + (this.e45 * other.e3215) - (this.e3215 * other.e45)) + float4((this.group1.yzx * other.group1.zxy), this.e15 * other.e4235) - (this.group1.zxyy * float4(other.group1.yzx, other.e25)) - (this.group1.wwwz * float4(other.group0.xyz, other.e35)) - (other.group2.wwwx * float4(this.group0.xyz, this.e4235))
        );
    }
}
public extension Line {
    // Fancy infix trick (first half)
    public AntiWedgeInfixPartial<Line> operator /(AntiWedgeInfix rhs) {
        return AntiWedgeInfixPartial<Line>(this);
    }
}
public extension AntiWedgeInfixPartial<Line> {
    // Fancy infix trick (second half)
    public Scalar operator /(AntiCircleRotor rhs) {
        return this.a.anti_wedge(rhs);
    }
}
public extension Line {
    public Scalar anti_wedge(AntiCircleRotor other) {
        return Scalar(
            /* scalar */
            -(other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435)
        );
    }
}
public extension Line {
    public RoundPoint anti_wedge(AntiDipoleInversion other) {
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4((other.e412 * this.e315) + (other.e321 * this.e415), (other.e423 * this.e125) + (other.e321 * this.e425), (other.e431 * this.e235) + (other.e321 * this.e435), -(other.e431 * this.e425) - (other.e412 * this.e435)) - float4((other.group0.xyz.yzx * this.group1.xyz.zxy), other.e423 * this.e415), 
            /* e5 */
            -(other.e415 * this.e235) - (other.e425 * this.e315) - (other.e435 * this.e125) - (other.e235 * this.e415) - (other.e315 * this.e425) - (other.e125 * this.e435)
        );
    }
}
public extension Line {
    public FlatPoint anti_wedge(AntiDualNum other) {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(other.group0.xy.xx, other.e3215, 0.0) * float4(float3(1.0), 0.0) * float4(this.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
public extension Line {
    public AntiPlane anti_wedge(AntiFlatPoint other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4((float3(other.e321) * this.group0.xyz), -(other.e235 * this.e415) - (other.e315 * this.e425) - (other.e125 * this.e435))
        );
    }
}
public extension Line {
    public AntiPlane anti_wedge(AntiFlector other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4((float3(other.e321) * this.group0.xyz), -(other.e235 * this.e415) - (other.e315 * this.e425) - (other.e125 * this.e435))
        );
    }
}
public extension Line {
    public Scalar anti_wedge(AntiLine other) {
        return Scalar(
            /* scalar */
            -(other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435)
        );
    }
}
public extension Line {
    public AntiMotor anti_wedge(AntiMotor other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(float3(0.0), -(other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435)), 
            /* e15, e25, e35, e3215 */
            float4(float3(1.0), 0.0) * float4(this.group0.xyz, 0.0) * float4(other.group1.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
public extension Line {
    public Line anti_wedge(AntiScalar other) {
        return Line(
            /* e415, e425, e435 */
            float3(other.e12345) * this.group0.xyz, 
            /* e235, e315, e125 */
            float3(other.e12345) * this.group1.xyz
        );
    }
}
public extension Line {
    public RoundPoint anti_wedge(Circle other) {
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4((other.e412 * this.e315) + (other.e321 * this.e415), (other.e423 * this.e125) + (other.e321 * this.e425), (other.e431 * this.e235) + (other.e321 * this.e435), -(other.e431 * this.e425) - (other.e412 * this.e435)) - float4((other.group0.xyz.yzx * this.group1.xyz.zxy), other.e423 * this.e415), 
            /* e5 */
            -(other.e415 * this.e235) - (other.e425 * this.e315) - (other.e435 * this.e125) - (other.e235 * this.e415) - (other.e315 * this.e425) - (other.e125 * this.e435)
        );
    }
}
public extension Line {
    public AntiDipoleInversion anti_wedge(CircleRotor other) {
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(1.0), 0.0) * float4(this.group0.xyz, 0.0) * float4(other.group2.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e235, e315, e125, e4 */
            float4((float3(other.e12345) * this.group1.xyz), -(other.e423 * this.e415) - (other.e431 * this.e425) - (other.e412 * this.e435)), 
            /* e1, e2, e3, e5 */
            float4((other.e412 * this.e315) + (other.e321 * this.e415), (other.e423 * this.e125) + (other.e321 * this.e425), (other.e431 * this.e235) + (other.e321 * this.e435), -(other.e415 * this.e235) - (other.e425 * this.e315) - (other.e435 * this.e125) - (other.e315 * this.e425) - (other.e125 * this.e435)) - float4((other.group0.xyz.yzx * this.group1.xyz.zxy), other.e235 * this.e415)
        );
    }
}
public extension Line {
    public Scalar anti_wedge(Dipole other) {
        return Scalar(
            /* scalar */
            -(other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435)
        );
    }
}
public extension Line {
    public AntiCircleRotor anti_wedge(DipoleInversion other) {
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(other.e1234) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4((float3(other.e1234) * this.group1.xyz), -(other.e4235 * this.e415) - (other.e4315 * this.e425) - (other.e4125 * this.e435)), 
            /* e15, e25, e35, scalar */
            float4((other.e4125 * this.e315) + (other.e3215 * this.e415), (other.e4235 * this.e125) + (other.e3215 * this.e425), (other.e4315 * this.e235) + (other.e3215 * this.e435), -(other.e42 * this.e315) - (other.e43 * this.e125) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435)) - float4((this.group1.xyz.zxy * other.group3.yzx), other.e41 * this.e235)
        );
    }
}
public extension Line {
    public Line anti_wedge(DualNum other) {
        return Line(
            /* e415, e425, e435 */
            float3(other.e12345) * this.group0.xyz, 
            /* e235, e315, e125 */
            float3(other.e12345) * this.group1.xyz
        );
    }
}
public extension Line {
    public FlatPoint anti_wedge(Flector other) {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4((other.e4125 * this.e315) + (other.e3215 * this.e415), (other.e4235 * this.e125) + (other.e3215 * this.e425), (other.e4315 * this.e235) + (other.e3215 * this.e435), -(other.e4315 * this.e425) - (other.e4125 * this.e435)) - (other.group1.yzxx * float4(this.group1.xyz.zxy, this.e415))
        );
    }
}
public extension Line {
    public DualNum anti_wedge(Line other) {
        return DualNum(
            /* e5, e12345 */
            float2(-(other.e415 * this.e235) - (other.e425 * this.e315) - (other.e435 * this.e125) - (other.e235 * this.e415) - (other.e315 * this.e425) - (other.e125 * this.e435), 0.0)
        );
    }
}
public extension Line {
    public Motor anti_wedge(Motor other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(float3(1.0), 0.0) * float4(this.group0.xyz, 0.0) * float4(other.group0.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e235, e315, e125, e5 */
            float4((float3(other.e12345) * this.group1.xyz), -(this.e415 * other.e235) - (this.e425 * other.e315) - (this.e435 * other.e125) - (this.e235 * other.e415) - (this.e315 * other.e425) - (this.e125 * other.e435))
        );
    }
}
public extension Line {
    public MultiVector anti_wedge(MultiVector other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(-(this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43), 0.0), 
            /* e1, e2, e3, e4 */
            float4((this.e415 * other.e321) + (this.e315 * other.e412), (this.e425 * other.e321) + (this.e125 * other.e423), (this.e435 * other.e321) + (this.e235 * other.e431), -(this.e425 * other.e431) - (this.e435 * other.e412)) - float4((this.group1.xyz.zxy * other.group7.xyz.yzx), this.e415 * other.e423), 
            /* e5 */
            -(this.e415 * other.e235) - (this.e425 * other.e315) - (this.e435 * other.e125) - (this.e235 * other.e415) - (this.e315 * other.e425) - (this.e125 * other.e435), 
            /* e15, e25, e35, e45 */
            float4((this.e415 * other.e3215) + (this.e315 * other.e4125), (this.e425 * other.e3215) + (this.e125 * other.e4235), (this.e435 * other.e3215) + (this.e235 * other.e4315), -(this.e425 * other.e4315) - (this.e435 * other.e4125)) - (other.group9.yzxx * float4(this.group1.xyz.zxy, this.e415)), 
            /* e41, e42, e43 */
            float3(other.e1234) * this.group0.xyz, 
            /* e23, e31, e12 */
            float3(other.e1234) * this.group1.xyz, 
            /* e415, e425, e435, e321 */
            float4(other.group0.xy.yy, other.e12345, 0.0) * float4(float3(1.0), 0.0) * float4(this.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(other.e12345) * this.group1.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
public extension Line {
    public FlatPoint anti_wedge(Plane other) {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4((this.e415 * other.e3215) + (this.e315 * other.e4125), (this.e425 * other.e3215) + (this.e125 * other.e4235), (this.e435 * other.e3215) + (this.e235 * other.e4315), -(this.e425 * other.e4315) - (this.e435 * other.e4125)) - (other.group0.yzxx * float4(this.group1.xyz.zxy, this.e415))
        );
    }
}
public extension Line {
    public Dipole anti_wedge(Sphere other) {
        return Dipole(
            /* e41, e42, e43 */
            float3(other.e1234) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4((float3(other.e1234) * this.group1.xyz), -(this.e415 * other.e4235) - (this.e425 * other.e4315) - (this.e435 * other.e4125)), 
            /* e15, e25, e35 */
            (float3(other.e3215) * this.group0.xyz) + (this.group1.xyz.yzx * other.group0.zxy) - (this.group1.xyz.zxy * other.group0.yzx)
        );
    }
}
public extension Line {
    public AntiDipoleInversion anti_wedge(VersorEven other) {
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(1.0), 0.0) * float4(this.group0.xyz, 0.0) * float4(other.group0.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e235, e315, e125, e4 */
            float4((float3(other.e12345) * this.group1.xyz), -(this.e415 * other.e423) - (this.e425 * other.e431) - (this.e435 * other.e412)), 
            /* e1, e2, e3, e5 */
            float4((this.e415 * other.e321) + (this.e315 * other.e412), (this.e425 * other.e321) + (this.e125 * other.e423), (this.e435 * other.e321) + (this.e235 * other.e431), -(this.e425 * other.e315) - (this.e435 * other.e125) - (this.e235 * other.e415) - (this.e315 * other.e425) - (this.e125 * other.e435)) - float4((this.group1.xyz.zxy * other.group0.yzx), this.e415 * other.e235)
        );
    }
}
public extension Line {
    public AntiCircleRotor anti_wedge(VersorOdd other) {
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(other.e1234) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4((float3(other.e1234) * this.group1.xyz), -(this.e415 * other.e4235) - (this.e425 * other.e4315) - (this.e435 * other.e4125)), 
            /* e15, e25, e35, scalar */
            float4((this.e415 * other.e3215) + (this.e315 * other.e4125), (this.e425 * other.e3215) + (this.e125 * other.e4235), (this.e435 * other.e3215) + (this.e235 * other.e4315), -(this.e425 * other.e31) - (this.e435 * other.e12) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43)) - float4((this.group1.xyz.zxy * other.group3.yzx), this.e415 * other.e23)
        );
    }
}
public extension Motor {
    // Fancy infix trick (first half)
    public AntiWedgeInfixPartial<Motor> operator /(AntiWedgeInfix rhs) {
        return AntiWedgeInfixPartial<Motor>(this);
    }
}
public extension AntiWedgeInfixPartial<Motor> {
    // Fancy infix trick (second half)
    public AntiCircleRotor operator /(AntiCircleRotor rhs) {
        return this.a.anti_wedge(rhs);
    }
}
public extension Motor {
    public AntiCircleRotor anti_wedge(AntiCircleRotor other) {
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(this.e12345) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(this.e12345) * other.group1, 
            /* e15, e25, e35, scalar */
            float4((float3(this.e12345) * other.group2.xyz), (other.scalar * this.e12345) - (other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435))
        );
    }
}
public extension Motor {
    public AntiDipoleInversion anti_wedge(AntiDipoleInversion other) {
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(this.e12345) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(this.e12345) * other.group1, 
            /* e235, e315, e125, e4 */
            float4((float3(this.e12345) * other.group2.xyz), (other.e4 * this.e12345) - (other.e423 * this.e415) - (other.e431 * this.e425) - (other.e412 * this.e435)), 
            /* e1, e2, e3, e5 */
            float4((other.e321 * this.e415) + (other.e1 * this.e12345), (other.e321 * this.e425) + (other.e2 * this.e12345), (other.e321 * this.e435) + (other.e3 * this.e12345), -(other.e425 * this.e315) - (other.e435 * this.e125) - (other.e235 * this.e415) - (other.e315 * this.e425) - (other.e125 * this.e435)) + float4((other.group0.xyz.zxy * this.group1.yzx), other.e5 * this.e12345) - (this.group1.zxyx * float4(other.group0.xyz.yzx, other.e415))
        );
    }
}
public extension Motor {
    public AntiMotor anti_wedge(AntiDualNum other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(float3(0.0), other.scalar * this.e12345), 
            /* e15, e25, e35, e3215 */
            float4(other.e3215) * this.group0
        );
    }
}
public extension Motor {
    public AntiFlector anti_wedge(AntiFlatPoint other) {
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(this.e12345) * other.group0, 
            /* e1, e2, e3, e5 */
            float4((other.group0.www * this.group0.xyz), -(other.e235 * this.e415) - (other.e315 * this.e425) - (other.e125 * this.e435))
        );
    }
}
public extension Motor {
    public AntiFlector anti_wedge(AntiFlector other) {
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(this.e12345) * other.group0, 
            /* e1, e2, e3, e5 */
            float4(other.e1 * this.e12345, other.e2 * this.e12345, other.e3 * this.e12345, -(other.e235 * this.e415) - (other.e315 * this.e425) - (other.e125 * this.e435)) + (this.group0 * float4(other.group0.www, other.e5))
        );
    }
}
public extension Motor {
    public AntiMotor anti_wedge(AntiLine other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((float3(this.e12345) * other.group0.xyz), -(other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435)), 
            /* e15, e25, e35, e3215 */
            float4(float3(1.0), 0.0) * float4(other.group1.xyz, 0.0) * float4(this.group0.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
public extension Motor {
    public AntiMotor anti_wedge(AntiMotor other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((float3(this.e12345) * other.group0.xyz), (other.scalar * this.e12345) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435)), 
            /* e15, e25, e35, e3215 */
            float4(((float3(other.e3215) * this.group0.xyz) + (float3(this.e12345) * other.group1.xyz)), other.e3215 * this.e12345)
        );
    }
}
public extension Motor {
    public AntiPlane anti_wedge(AntiPlane other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(this.e12345) * other.group0
        );
    }
}
public extension Motor {
    public Motor anti_wedge(AntiScalar other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(other.e12345) * this.group0, 
            /* e235, e315, e125, e5 */
            float4(other.e12345) * this.group1
        );
    }
}
public extension Motor {
    public AntiDipoleInversion anti_wedge(Circle other) {
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(this.e12345) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(this.e12345) * other.group1, 
            /* e235, e315, e125, e4 */
            float4((float3(this.e12345) * other.group2.xyz), -(other.e423 * this.e415) - (other.e431 * this.e425) - (other.e412 * this.e435)), 
            /* e1, e2, e3, e5 */
            float4((other.e412 * this.e315) + (other.e321 * this.e415), (other.e423 * this.e125) + (other.e321 * this.e425), (other.e431 * this.e235) + (other.e321 * this.e435), -(other.e415 * this.e235) - (other.e425 * this.e315) - (other.e435 * this.e125) - (other.e315 * this.e425) - (other.e125 * this.e435)) - float4((other.group0.xyz.yzx * this.group1.zxy), other.e235 * this.e415)
        );
    }
}
public extension Motor {
    public VersorEven anti_wedge(CircleRotor other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(this.e12345) * float4(other.group0.xyz, other.e12345), 
            /* e415, e425, e435, e321 */
            float4(((float3(other.e12345) * this.group0.xyz) + (float3(this.e12345) * other.group1.xyz)), other.e321 * this.e12345), 
            /* e235, e315, e125, e5 */
            float4(other.e12345 * this.e235, other.e12345 * this.e315, other.e12345 * this.e125, -(other.e415 * this.e235) - (other.e425 * this.e315) - (other.e435 * this.e125) - (other.e235 * this.e415) - (other.e315 * this.e425) - (other.e125 * this.e435)) + (other.group2 * float4(this.group0.www, this.e5)), 
            /* e1, e2, e3, e4 */
            float4((other.e412 * this.e315) + (other.e321 * this.e415), (other.e423 * this.e125) + (other.e321 * this.e425), (other.e431 * this.e235) + (other.e321 * this.e435), -(other.e431 * this.e425) - (other.e412 * this.e435)) - float4((other.group0.xyz.yzx * this.group1.zxy), other.e423 * this.e415)
        );
    }
}
public extension Motor {
    public AntiCircleRotor anti_wedge(Dipole other) {
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(this.e12345) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(this.e12345) * other.group1, 
            /* e15, e25, e35, scalar */
            float4((float3(this.e12345) * other.group2.xyz), -(other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435))
        );
    }
}
public extension Motor {
    public VersorOdd anti_wedge(DipoleInversion other) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(other.e1234 * this.e415, other.e1234 * this.e425, other.e1234 * this.e435, -(other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435)) + float4((other.group0.xyz * this.group0.www), other.e1234 * this.e5), 
            /* e23, e31, e12, e45 */
            float4(other.e1234 * this.e235, other.e1234 * this.e315, other.e1234 * this.e125, -(other.e4235 * this.e415) - (other.e4315 * this.e425) - (other.e4125 * this.e435)) + (float4(this.e12345) * other.group1), 
            /* e15, e25, e35, e1234 */
            float4(((float3(other.e3215) * this.group0.xyz) + (float3(this.e12345) * other.group2.xyz) + (other.group3.zxy * this.group1.yzx) - (other.group3.yzx * this.group1.zxy)), other.e1234 * this.e12345), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * other.group3
        );
    }
}
public extension Motor {
    public Motor anti_wedge(DualNum other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(other.e12345) * this.group0, 
            /* e235, e315, e125, e5 */
            float4(other.group0.xy.yy, other.e12345, (other.e5 * this.e12345) + (other.e12345 * this.e5)) * float4(this.group1.xyz, 1.0)
        );
    }
}
public extension Motor {
    public FlatPoint anti_wedge(FlatPoint other) {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(this.e12345) * other.group0
        );
    }
}
public extension Motor {
    public Flector anti_wedge(Flector other) {
        return Flector(
            /* e15, e25, e35, e45 */
            float4((other.e4125 * this.e315) + (other.e3215 * this.e415), (other.e4235 * this.e125) + (other.e3215 * this.e425), (other.e4315 * this.e235) + (other.e3215 * this.e435), -(other.e4315 * this.e425) - (other.e4125 * this.e435)) + (float4(this.e12345) * other.group0) - (other.group1.yzxx * float4(this.group1.zxy, this.e415)), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * other.group1
        );
    }
}
public extension Motor {
    public Motor anti_wedge(Line other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(float3(1.0), 0.0) * float4(other.group0.xyz, 0.0) * float4(this.group0.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e235, e315, e125, e5 */
            float4((float3(this.e12345) * other.group1.xyz), -(other.e415 * this.e235) - (other.e425 * this.e315) - (other.e435 * this.e125) - (other.e235 * this.e415) - (other.e315 * this.e425) - (other.e125 * this.e435))
        );
    }
}
public extension Motor {
    public Motor anti_wedge(Motor other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(((float3(other.e12345) * this.group0.xyz) + (float3(this.e12345) * other.group0.xyz)), other.e12345 * this.e12345), 
            /* e235, e315, e125, e5 */
            (float4(other.e12345) * this.group1) + (float4(this.e12345) * other.group1) + float4(float3(0.0), -(other.e415 * this.e235) - (other.e425 * this.e315) - (other.e435 * this.e125) - (other.e235 * this.e415) - (other.e315 * this.e425) - (other.e125 * this.e435))
        );
    }
}
public extension Motor {
    public MultiVector anti_wedge(MultiVector other) {
        return MultiVector(
            /* scalar, e12345 */
            float2((this.e12345 * other.scalar) + (this.e5 * other.e1234) - (this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43), this.e12345 * other.e12345), 
            /* e1, e2, e3, e4 */
            float4((this.e415 * other.e321) + (this.e12345 * other.e1), (this.e425 * other.e321) + (this.e12345 * other.e2), (this.e435 * other.e321) + (this.e12345 * other.e3), -(this.e425 * other.e431) - (this.e435 * other.e412)) + float4((other.group7.xyz.zxy * this.group1.yzx), this.e12345 * other.e4) - float4((other.group7.xyz.yzx * this.group1.zxy), this.e415 * other.e423), 
            /* e5 */
            (this.e12345 * other.e5) + (this.e5 * other.e12345) - (this.e415 * other.e235) - (this.e425 * other.e315) - (this.e435 * other.e125) - (this.e235 * other.e415) - (this.e315 * other.e425) - (this.e125 * other.e435), 
            /* e15, e25, e35, e45 */
            float4((this.e12345 * other.e15) + (this.e315 * other.e4125), (this.e12345 * other.e25) + (this.e125 * other.e4235), (this.e12345 * other.e35) + (this.e235 * other.e4315), -(this.e425 * other.e4315) - (this.e435 * other.e4125)) + (this.group0 * float4(other.group9.www, other.e45)) - (other.group9.yzxx * float4(this.group1.zxy, this.e415)), 
            /* e41, e42, e43 */
            (float3(this.e12345) * other.group4.xyz) + (float3(other.e1234) * this.group0.xyz), 
            /* e23, e31, e12 */
            (float3(this.e12345) * other.group5.xyz) + (float3(other.e1234) * this.group1.xyz), 
            /* e415, e425, e435, e321 */
            float4(((float3(this.e12345) * other.group6.xyz) + (float3(other.e12345) * this.group0.xyz)), this.e12345 * other.e321), 
            /* e423, e431, e412 */
            float3(this.e12345) * other.group7.xyz, 
            /* e235, e315, e125 */
            (float3(this.e12345) * other.group8.xyz) + (float3(other.e12345) * this.group1.xyz), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * other.group9, 
            /* e1234 */
            this.e12345 * other.e1234
        );
    }
}
public extension Motor {
    public Flector anti_wedge(Plane other) {
        return Flector(
            /* e15, e25, e35, e45 */
            float4((this.e415 * other.e3215) + (this.e315 * other.e4125), (this.e425 * other.e3215) + (this.e125 * other.e4235), (this.e435 * other.e3215) + (this.e235 * other.e4315), -(this.e425 * other.e4315) - (this.e435 * other.e4125)) - (other.group0.yzxx * float4(this.group1.zxy, this.e415)), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * other.group0
        );
    }
}
public extension Motor {
    public RoundPoint anti_wedge(RoundPoint other) {
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(this.e12345) * other.group0, 
            /* e5 */
            this.e12345 * other.e5
        );
    }
}
public extension Motor {
    public Scalar anti_wedge(Scalar other) {
        return Scalar(
            /* scalar */
            this.e12345 * other.scalar
        );
    }
}
public extension Motor {
    public VersorOdd anti_wedge(Sphere other) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(other.e1234) * float4(this.group0.xyz, this.e5), 
            /* e23, e31, e12, e45 */
            float4((float3(other.e1234) * this.group1.xyz), -(this.e415 * other.e4235) - (this.e425 * other.e4315) - (this.e435 * other.e4125)), 
            /* e15, e25, e35, e1234 */
            float4(((float3(other.e3215) * this.group0.xyz) + (this.group1.yzx * other.group0.zxy) - (this.group1.zxy * other.group0.yzx)), this.e12345 * other.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * other.group0
        );
    }
}
public extension Motor {
    public VersorEven anti_wedge(VersorEven other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(this.e12345) * other.group0, 
            /* e415, e425, e435, e321 */
            float4(((float3(this.e12345) * other.group1.xyz) + (float3(other.e12345) * this.group0.xyz)), this.e12345 * other.e321), 
            /* e235, e315, e125, e5 */
            (float4(this.e12345) * other.group2) + (float4(other.e12345) * this.group1) + float4(float3(0.0), -(this.e415 * other.e235) - (this.e425 * other.e315) - (this.e435 * other.e125) - (this.e235 * other.e415) - (this.e315 * other.e425) - (this.e125 * other.e435)), 
            /* e1, e2, e3, e4 */
            float4((this.e12345 * other.e1) + (this.e315 * other.e412), (this.e12345 * other.e2) + (this.e125 * other.e423), (this.e12345 * other.e3) + (this.e235 * other.e431), -(this.e425 * other.e431) - (this.e435 * other.e412)) + (this.group0 * float4(other.group1.www, other.e4)) - (other.group0.yzxx * float4(this.group1.zxy, this.e415))
        );
    }
}
public extension Motor {
    public VersorOdd anti_wedge(VersorOdd other) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (this.group0 * float4(other.group2.www, other.scalar)) + float4(float3(0.0), -(this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43)) + float4((this.group0.www * other.group0.xyz), this.e5 * other.e1234), 
            /* e23, e31, e12, e45 */
            float4(this.e235 * other.e1234, this.e315 * other.e1234, this.e125 * other.e1234, -(this.e415 * other.e4235) - (this.e425 * other.e4315) - (this.e435 * other.e4125)) + (float4(this.e12345) * other.group1), 
            /* e15, e25, e35, e1234 */
            float4(((float3(this.e12345) * other.group2.xyz) + (float3(other.e3215) * this.group0.xyz) + (this.group1.yzx * other.group3.zxy) - (this.group1.zxy * other.group3.yzx)), this.e12345 * other.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * other.group3
        );
    }
}
public extension MultiVector {
    // Fancy infix trick (first half)
    public AntiWedgeInfixPartial<MultiVector> operator /(AntiWedgeInfix rhs) {
        return AntiWedgeInfixPartial<MultiVector>(this);
    }
}
public extension AntiWedgeInfixPartial<MultiVector> {
    // Fancy infix trick (second half)
    public MultiVector operator /(AntiCircleRotor rhs) {
        return this.a.anti_wedge(rhs);
    }
}
public extension MultiVector {
    public MultiVector anti_wedge(AntiCircleRotor other) {
        return MultiVector(
            /* scalar, e12345 */
            float2((other.scalar * this.e12345) - (other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e45 * this.e321) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412), 0.0), 
            /* e1, e2, e3, e4 */
            float4(-(other.e41 * this.e3215) - (other.e31 * this.e4125), -(other.e42 * this.e3215) - (other.e12 * this.e4235), -(other.e43 * this.e3215) - (other.e23 * this.e4315), (other.e43 * this.e4125) + (other.e45 * this.e1234)) + (this.group9.yzxx * float4(other.group1.zxy, other.e41)) + float4((float3(this.e1234) * other.group2.xyz), other.e42 * this.e4315), 
            /* e5 */
            -(other.e45 * this.e3215) - (other.e15 * this.e4235) - (other.e25 * this.e4315) - (other.e35 * this.e4125), 
            /* e15, e25, e35, e45 */
            float4(this.e12345) * float4(other.group2.xyz, other.e45), 
            /* e41, e42, e43 */
            float3(this.e12345) * other.group0.xyz, 
            /* e23, e31, e12 */
            float3(this.e12345) * other.group1.xyz, 
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
    public MultiVector anti_wedge(AntiDipoleInversion other) {
        return MultiVector(
            /* scalar, e12345 */
            float2((other.e4 * this.e3215) + (other.e1 * this.e4235) + (other.e2 * this.e4315) + (other.e3 * this.e4125) + (other.e5 * this.e1234) - (other.e423 * this.e15) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e321 * this.e45) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43), 0.0), 
            /* e1, e2, e3, e4 */
            float4((other.e412 * this.e315) + (other.e415 * this.e321) + (other.e321 * this.e415) + (other.e315 * this.e412), (other.e423 * this.e125) + (other.e425 * this.e321) + (other.e321 * this.e425) + (other.e125 * this.e423), (other.e431 * this.e235) + (other.e435 * this.e321) + (other.e321 * this.e435) + (other.e235 * this.e431), -(other.e412 * this.e435) - (other.e415 * this.e423) - (other.e425 * this.e431) - (other.e435 * this.e412)) + (float4(this.e12345) * float4(other.group3.xyz, other.e4)) - float4((other.group0.xyz.yzx * this.group8.xyz.zxy), other.e423 * this.e415) - float4((this.group7.xyz.yzx * other.group2.zxy), other.e431 * this.e425), 
            /* e5 */
            (other.e5 * this.e12345) - (other.e415 * this.e235) - (other.e425 * this.e315) - (other.e435 * this.e125) - (other.e235 * this.e415) - (other.e315 * this.e425) - (other.e125 * this.e435), 
            /* e15, e25, e35, e45 */
            float4((other.e415 * this.e3215) + (other.e315 * this.e4125), (other.e425 * this.e3215) + (other.e125 * this.e4235), (other.e435 * this.e3215) + (other.e235 * this.e4315), -(other.e425 * this.e4315) - (other.e435 * this.e4125)) - (this.group9.yzxx * float4(other.group2.zxy, other.e415)), 
            /* e41, e42, e43 */
            (float3(this.e1234) * other.group1.xyz) + (other.group0.xyz.zxy * this.group9.yzx) - (other.group0.xyz.yzx * this.group9.zxy), 
            /* e23, e31, e12 */
            (float3(this.e3215) * other.group0.xyz) + (float3(this.e1234) * other.group2.xyz) - (float3(other.e321) * this.group9.xyz), 
            /* e415, e425, e435, e321 */
            float4(this.e12345) * other.group1, 
            /* e423, e431, e412 */
            float3(this.e12345) * other.group0.xyz, 
            /* e235, e315, e125 */
            float3(this.e12345) * other.group2.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
public extension MultiVector {
    public MultiVector anti_wedge(AntiDualNum other) {
        return MultiVector(
            /* scalar, e12345 */
            float2((other.e3215 * this.e4) + (other.scalar * this.e12345), 0.0), 
            /* e1, e2, e3, e4 */
            float4(other.group0.xy.xx, other.e3215, 0.0) * float4(float3(1.0), 0.0) * float4(this.group4.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e5 */
            other.e3215 * this.e45, 
            /* e15, e25, e35, e45 */
            float4(other.group0.xy.xx, other.e3215, 0.0) * float4(float3(1.0), 0.0) * float4(this.group6.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(other.e3215) * this.group7.xyz, 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), other.e3215 * this.e1234), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(other.e3215) * this.group9.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), other.e3215 * this.e12345), 
            /* e1234 */
            0.0
        );
    }
}
public extension MultiVector {
    public MultiVector anti_wedge(AntiFlatPoint other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(-(other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43) - (other.e321 * this.e45), 0.0), 
            /* e1, e2, e3, e4 */
            float4((float3(other.e321) * this.group6.xyz), 0.0) + float4((this.group7.xyz.zxy * other.group0.yzx), 0.0) - float4((this.group7.xyz.yzx * other.group0.zxy), 0.0), 
            /* e5 */
            -(other.e235 * this.e415) - (other.e315 * this.e425) - (other.e125 * this.e435), 
            /* e15, e25, e35, e45 */
            float4(((other.group0.yzx * this.group9.zxy) - (other.group0.zxy * this.group9.yzx)), 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            (float3(this.e1234) * other.group0.xyz) - (float3(other.e321) * this.group9.xyz), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), other.e321 * this.e12345), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(this.e12345) * other.group0.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
public extension MultiVector {
    public MultiVector anti_wedge(AntiFlector other) {
        return MultiVector(
            /* scalar, e12345 */
            float2((other.e1 * this.e4235) + (other.e2 * this.e4315) + (other.e3 * this.e4125) + (other.e5 * this.e1234) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43) - (other.e321 * this.e45), 0.0), 
            /* e1, e2, e3, e4 */
            float4((float3(other.e321) * this.group6.xyz), 0.0) + float4((float3(this.e12345) * other.group1.xyz), 0.0) + float4((this.group7.xyz.zxy * other.group0.yzx), 0.0) - float4((this.group7.xyz.yzx * other.group0.zxy), 0.0), 
            /* e5 */
            (other.e5 * this.e12345) - (other.e235 * this.e415) - (other.e315 * this.e425) - (other.e125 * this.e435), 
            /* e15, e25, e35, e45 */
            float4(((other.group0.yzx * this.group9.zxy) - (other.group0.zxy * this.group9.yzx)), 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            (float3(this.e1234) * other.group0.xyz) - (float3(other.e321) * this.group9.xyz), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), other.e321 * this.e12345), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(this.e12345) * other.group0.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
public extension MultiVector {
    public MultiVector anti_wedge(AntiLine other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(-(other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412), 0.0), 
            /* e1, e2, e3, e4 */
            float4((float3(this.e1234) * other.group1.xyz), 0.0) + float4((other.group0.xyz.zxy * this.group9.yzx), 0.0) - float4((other.group0.xyz.yzx * this.group9.zxy), 0.0), 
            /* e5 */
            -(other.e15 * this.e4235) - (other.e25 * this.e4315) - (other.e35 * this.e4125), 
            /* e15, e25, e35, e45 */
            float4(this.group0.xy.yy, this.e12345, 0.0) * float4(float3(1.0), 0.0) * float4(other.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(this.e12345) * other.group0.xyz, 
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
    public MultiVector anti_wedge(AntiMotor other) {
        return MultiVector(
            /* scalar, e12345 */
            float2((other.scalar * this.e12345) + (other.e3215 * this.e4) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412), 0.0), 
            /* e1, e2, e3, e4 */
            float4((float3(other.e3215) * this.group4.xyz), 0.0) + float4((float3(this.e1234) * other.group1.xyz), 0.0) + float4((other.group0.zxy * this.group9.yzx), 0.0) - float4((other.group0.yzx * this.group9.zxy), 0.0), 
            /* e5 */
            (other.e3215 * this.e45) - (other.e15 * this.e4235) - (other.e25 * this.e4315) - (other.e35 * this.e4125), 
            /* e15, e25, e35, e45 */
            float4(((float3(other.e3215) * this.group6.xyz) + (float3(this.e12345) * other.group1.xyz)), 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            (float3(other.e3215) * this.group7.xyz) + (float3(this.e12345) * other.group0.xyz), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), other.e3215 * this.e1234), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(other.e3215) * this.group9.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), other.e3215 * this.e12345), 
            /* e1234 */
            0.0
        );
    }
}
public extension MultiVector {
    public MultiVector anti_wedge(AntiPlane other) {
        return MultiVector(
            /* scalar, e12345 */
            float2((other.e1 * this.e4235) + (other.e2 * this.e4315) + (other.e3 * this.e4125) + (other.e5 * this.e1234), 0.0), 
            /* e1, e2, e3, e4 */
            float4(this.group0.xy.yy, this.e12345, 0.0) * float4(float3(1.0), 0.0) * float4(other.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e5 */
            other.e5 * this.e12345, 
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
    public MultiVector anti_wedge(AntiScalar other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(other.e12345) * this.group0.xy, 
            /* e1, e2, e3, e4 */
            float4(other.e12345) * this.group1, 
            /* e5 */
            other.e12345 * this.e5, 
            /* e15, e25, e35, e45 */
            float4(other.e12345) * this.group3, 
            /* e41, e42, e43 */
            float3(other.e12345) * this.group4.xyz, 
            /* e23, e31, e12 */
            float3(other.e12345) * this.group5.xyz, 
            /* e415, e425, e435, e321 */
            float4(other.e12345) * this.group6, 
            /* e423, e431, e412 */
            float3(other.e12345) * this.group7.xyz, 
            /* e235, e315, e125 */
            float3(other.e12345) * this.group8.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345) * this.group9, 
            /* e1234 */
            other.e12345 * this.e1234
        );
    }
}
public extension MultiVector {
    public MultiVector anti_wedge(Circle other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(-(other.e423 * this.e15) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e321 * this.e45) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43), 0.0), 
            /* e1, e2, e3, e4 */
            float4((other.e412 * this.e315) + (other.e415 * this.e321) + (other.e321 * this.e415) + (other.e315 * this.e412), (other.e423 * this.e125) + (other.e425 * this.e321) + (other.e321 * this.e425) + (other.e125 * this.e423), (other.e431 * this.e235) + (other.e435 * this.e321) + (other.e321 * this.e435) + (other.e235 * this.e431), -(other.e412 * this.e435) - (other.e415 * this.e423) - (other.e425 * this.e431) - (other.e435 * this.e412)) - float4((other.group0.xyz.yzx * this.group8.xyz.zxy), other.e423 * this.e415) - float4((other.group2.xyz.zxy * this.group7.xyz.yzx), other.e431 * this.e425), 
            /* e5 */
            -(other.e415 * this.e235) - (other.e425 * this.e315) - (other.e435 * this.e125) - (other.e235 * this.e415) - (other.e315 * this.e425) - (other.e125 * this.e435), 
            /* e15, e25, e35, e45 */
            float4((other.e415 * this.e3215) + (other.e315 * this.e4125), (other.e425 * this.e3215) + (other.e125 * this.e4235), (other.e435 * this.e3215) + (other.e235 * this.e4315), -(other.e425 * this.e4315) - (other.e435 * this.e4125)) - (this.group9.yzxx * float4(other.group2.xyz.zxy, other.e415)), 
            /* e41, e42, e43 */
            (float3(this.e1234) * other.group1.xyz) + (other.group0.xyz.zxy * this.group9.yzx) - (other.group0.xyz.yzx * this.group9.zxy), 
            /* e23, e31, e12 */
            (float3(this.e3215) * other.group0.xyz) + (float3(this.e1234) * other.group2.xyz) - (float3(other.e321) * this.group9.xyz), 
            /* e415, e425, e435, e321 */
            float4(this.e12345) * other.group1, 
            /* e423, e431, e412 */
            float3(this.e12345) * other.group0.xyz, 
            /* e235, e315, e125 */
            float3(this.e12345) * other.group2.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
public extension MultiVector {
    public MultiVector anti_wedge(CircleRotor other) {
        return MultiVector(
            /* scalar, e12345 */
            float2((other.e12345 * this.scalar) - (other.e423 * this.e15) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e321 * this.e45) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43), other.e12345 * this.e12345), 
            /* e1, e2, e3, e4 */
            float4((other.e415 * this.e321) + (other.e321 * this.e415) + (other.e315 * this.e412) + (other.e12345 * this.e1), (other.e425 * this.e321) + (other.e321 * this.e425) + (other.e125 * this.e423) + (other.e12345 * this.e2), (other.e435 * this.e321) + (other.e321 * this.e435) + (other.e235 * this.e431) + (other.e12345 * this.e3), -(other.e412 * this.e435) - (other.e415 * this.e423) - (other.e425 * this.e431) - (other.e435 * this.e412)) + float4((other.group0.xyz.zxy * this.group8.xyz.yzx), other.e12345 * this.e4) - float4((other.group0.xyz.yzx * this.group8.xyz.zxy), other.e423 * this.e415) - float4((this.group7.xyz.yzx * other.group2.zxy), other.e431 * this.e425), 
            /* e5 */
            (other.e12345 * this.e5) - (other.e415 * this.e235) - (other.e425 * this.e315) - (other.e435 * this.e125) - (other.e235 * this.e415) - (other.e315 * this.e425) - (other.e125 * this.e435), 
            /* e15, e25, e35, e45 */
            float4((other.e315 * this.e4125) + (other.e12345 * this.e15), (other.e125 * this.e4235) + (other.e12345 * this.e25), (other.e235 * this.e4315) + (other.e12345 * this.e35), -(other.e425 * this.e4315) - (other.e435 * this.e4125)) + float4((this.group9.www * other.group1.xyz), other.e12345 * this.e45) - (this.group9.yzxx * float4(other.group2.zxy, other.e415)), 
            /* e41, e42, e43 */
            (float3(other.e12345) * this.group4.xyz) + (float3(this.e1234) * other.group1.xyz) + (other.group0.xyz.zxy * this.group9.yzx) - (other.group0.xyz.yzx * this.group9.zxy), 
            /* e23, e31, e12 */
            (float3(other.e12345) * this.group5.xyz) + (float3(this.e3215) * other.group0.xyz) + (float3(this.e1234) * other.group2.xyz) - (float3(other.e321) * this.group9.xyz), 
            /* e415, e425, e435, e321 */
            (float4(other.e12345) * this.group6) + (float4(this.e12345) * other.group1), 
            /* e423, e431, e412 */
            (float3(other.e12345) * this.group7.xyz) + (float3(this.e12345) * other.group0.xyz), 
            /* e235, e315, e125 */
            (float3(other.e12345) * this.group8.xyz) + (float3(this.e12345) * other.group2.xyz), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345) * this.group9, 
            /* e1234 */
            other.e12345 * this.e1234
        );
    }
}
public extension MultiVector {
    public MultiVector anti_wedge(Dipole other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(-(other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e45 * this.e321) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412), 0.0), 
            /* e1, e2, e3, e4 */
            float4(-(other.e41 * this.e3215) - (other.e31 * this.e4125), -(other.e42 * this.e3215) - (other.e12 * this.e4235), -(other.e43 * this.e3215) - (other.e23 * this.e4315), (other.e43 * this.e4125) + (other.e45 * this.e1234)) + (this.group9.yzxy * float4(other.group1.zxy, other.e42)) + float4((float3(this.e1234) * other.group2.xyz), other.e41 * this.e4235), 
            /* e5 */
            -(other.e45 * this.e3215) - (other.e15 * this.e4235) - (other.e25 * this.e4315) - (other.e35 * this.e4125), 
            /* e15, e25, e35, e45 */
            float4(this.e12345) * float4(other.group2.xyz, other.e45), 
            /* e41, e42, e43 */
            float3(this.e12345) * other.group0.xyz, 
            /* e23, e31, e12 */
            float3(this.e12345) * other.group1.xyz, 
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
    public MultiVector anti_wedge(DipoleInversion other) {
        return MultiVector(
            /* scalar, e12345 */
            float2((other.e1234 * this.e5) + (other.e4235 * this.e1) + (other.e4315 * this.e2) + (other.e4125 * this.e3) + (other.e3215 * this.e4) - (other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e45 * this.e321) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412), 0.0), 
            /* e1, e2, e3, e4 */
            (float4(this.e1234) * float4(other.group2.xyz, other.e45)) + (this.group9.yzxz * float4(other.group1.zxy, other.e43)) + float4((this.group4.xyz * other.group3.www), other.e41 * this.e4235) + float4((this.group5.xyz.yzx * other.group3.zxy), other.e42 * this.e4315) - (float4(other.e1234) * this.group3) - (other.group3.yzxy * float4(this.group5.xyz.zxy, this.e42)) - float4((other.group0.xyz * this.group9.www), other.e4235 * this.e41) - float4((other.group1.yzx * this.group9.zxy), other.e4125 * this.e43), 
            /* e5 */
            (other.e4235 * this.e15) + (other.e4315 * this.e25) + (other.e4125 * this.e35) + (other.e3215 * this.e45) - (other.e45 * this.e3215) - (other.e15 * this.e4235) - (other.e25 * this.e4315) - (other.e35 * this.e4125), 
            /* e15, e25, e35, e45 */
            float4((other.e4125 * this.e315) + (other.e3215 * this.e415), (other.e4235 * this.e125) + (other.e3215 * this.e425), (other.e4315 * this.e235) + (other.e3215 * this.e435), -(other.e4315 * this.e425) - (other.e4125 * this.e435)) + (float4(this.e12345) * float4(other.group2.xyz, other.e45)) - float4((this.group8.xyz.zxy * other.group3.yzx), other.e4235 * this.e415), 
            /* e41, e42, e43 */
            (float3(other.e1234) * this.group6.xyz) + (float3(this.e12345) * other.group0.xyz) + (this.group7.xyz.zxy * other.group3.yzx) - (this.group7.xyz.yzx * other.group3.zxy), 
            /* e23, e31, e12 */
            (float3(other.e1234) * this.group8.xyz) + (float3(other.e3215) * this.group7.xyz) + (float3(this.e12345) * other.group1.xyz) - (float3(this.e321) * other.group3.xyz), 
            /* e415, e425, e435, e321 */
            (other.group3.yzxw * float4(this.group9.zxy, this.e1234)) - (this.group9.yzxw * float4(other.group3.zxy, other.e1234)), 
            /* e423, e431, e412 */
            (float3(this.e1234) * other.group3.xyz) - (float3(other.e1234) * this.group9.xyz), 
            /* e235, e315, e125 */
            (float3(other.e3215) * this.group9.xyz) - (float3(this.e3215) * other.group3.xyz), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * other.group3, 
            /* e1234 */
            other.e1234 * this.e12345
        );
    }
}
public extension MultiVector {
    public MultiVector anti_wedge(DualNum other) {
        return MultiVector(
            /* scalar, e12345 */
            float2((other.e5 * this.e1234) + (other.e12345 * this.scalar), other.e12345 * this.e12345), 
            /* e1, e2, e3, e4 */
            float4(other.e12345) * this.group1, 
            /* e5 */
            (other.e5 * this.e12345) + (other.e12345 * this.e5), 
            /* e15, e25, e35, e45 */
            float4(other.e12345) * this.group3, 
            /* e41, e42, e43 */
            float3(other.e12345) * this.group4.xyz, 
            /* e23, e31, e12 */
            float3(other.e12345) * this.group5.xyz, 
            /* e415, e425, e435, e321 */
            float4(other.e12345) * this.group6, 
            /* e423, e431, e412 */
            float3(other.e12345) * this.group7.xyz, 
            /* e235, e315, e125 */
            float3(other.e12345) * this.group8.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345) * this.group9, 
            /* e1234 */
            other.e12345 * this.e1234
        );
    }
}
public extension MultiVector {
    public MultiVector anti_wedge(FlatPoint other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(-(other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412) - (other.e45 * this.e321), 0.0), 
            /* e1, e2, e3, e4 */
            float4(this.e1234) * other.group0, 
            /* e5 */
            -(other.e15 * this.e4235) - (other.e25 * this.e4315) - (other.e35 * this.e4125) - (other.e45 * this.e3215), 
            /* e15, e25, e35, e45 */
            float4(this.e12345) * other.group0, 
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
    public MultiVector anti_wedge(Flector other) {
        return MultiVector(
            /* scalar, e12345 */
            float2((other.e4235 * this.e1) + (other.e4315 * this.e2) + (other.e4125 * this.e3) + (other.e3215 * this.e4) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412) - (other.e45 * this.e321), 0.0), 
            /* e1, e2, e3, e4 */
            float4((other.e15 * this.e1234) + (other.e4125 * this.e31), (other.e25 * this.e1234) + (other.e4235 * this.e12), (other.e35 * this.e1234) + (other.e4315 * this.e23), -(other.e4315 * this.e42) - (other.e4125 * this.e43)) + float4((this.group4.xyz * other.group1.www), other.e45 * this.e1234) - (other.group1.yzxx * float4(this.group5.xyz.zxy, this.e41)), 
            /* e5 */
            (other.e4235 * this.e15) + (other.e4315 * this.e25) + (other.e4125 * this.e35) + (other.e3215 * this.e45) - (other.e15 * this.e4235) - (other.e25 * this.e4315) - (other.e35 * this.e4125) - (other.e45 * this.e3215), 
            /* e15, e25, e35, e45 */
            float4((other.e4125 * this.e315) + (other.e3215 * this.e415), (other.e4235 * this.e125) + (other.e3215 * this.e425), (other.e4315 * this.e235) + (other.e3215 * this.e435), -(other.e4315 * this.e425) - (other.e4125 * this.e435)) + (float4(this.e12345) * other.group0) - float4((this.group8.xyz.zxy * other.group1.yzx), other.e4235 * this.e415), 
            /* e41, e42, e43 */
            (this.group7.xyz.zxy * other.group1.yzx) - (this.group7.xyz.yzx * other.group1.zxy), 
            /* e23, e31, e12 */
            (float3(other.e3215) * this.group7.xyz) - (float3(this.e321) * other.group1.xyz), 
            /* e415, e425, e435, e321 */
            float4(((other.group1.yzx * this.group9.zxy) - (other.group1.zxy * this.group9.yzx)), other.e3215 * this.e1234), 
            /* e423, e431, e412 */
            float3(this.e1234) * other.group1.xyz, 
            /* e235, e315, e125 */
            (float3(other.e3215) * this.group9.xyz) - (float3(this.e3215) * other.group1.xyz), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * other.group1, 
            /* e1234 */
            0.0
        );
    }
}
public extension MultiVector {
    public MultiVector anti_wedge(Line other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(-(other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43), 0.0), 
            /* e1, e2, e3, e4 */
            float4((other.e415 * this.e321) + (other.e315 * this.e412), (other.e425 * this.e321) + (other.e125 * this.e423), (other.e435 * this.e321) + (other.e235 * this.e431), -(other.e425 * this.e431) - (other.e435 * this.e412)) - float4((other.group1.xyz.zxy * this.group7.xyz.yzx), other.e415 * this.e423), 
            /* e5 */
            -(other.e415 * this.e235) - (other.e425 * this.e315) - (other.e435 * this.e125) - (other.e235 * this.e415) - (other.e315 * this.e425) - (other.e125 * this.e435), 
            /* e15, e25, e35, e45 */
            float4((other.e415 * this.e3215) + (other.e315 * this.e4125), (other.e425 * this.e3215) + (other.e125 * this.e4235), (other.e435 * this.e3215) + (other.e235 * this.e4315), -(other.e425 * this.e4315) - (other.e435 * this.e4125)) - (this.group9.yzxx * float4(other.group1.xyz.zxy, other.e415)), 
            /* e41, e42, e43 */
            float3(this.e1234) * other.group0.xyz, 
            /* e23, e31, e12 */
            float3(this.e1234) * other.group1.xyz, 
            /* e415, e425, e435, e321 */
            float4(this.group0.xy.yy, this.e12345, 0.0) * float4(float3(1.0), 0.0) * float4(other.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(this.e12345) * other.group1.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
public extension MultiVector {
    public MultiVector anti_wedge(Motor other) {
        return MultiVector(
            /* scalar, e12345 */
            float2((other.e12345 * this.scalar) + (other.e5 * this.e1234) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43), other.e12345 * this.e12345), 
            /* e1, e2, e3, e4 */
            float4((other.e415 * this.e321) + (other.e12345 * this.e1), (other.e425 * this.e321) + (other.e12345 * this.e2), (other.e435 * this.e321) + (other.e12345 * this.e3), -(other.e425 * this.e431) - (other.e435 * this.e412)) + float4((this.group7.xyz.zxy * other.group1.yzx), other.e12345 * this.e4) - float4((this.group7.xyz.yzx * other.group1.zxy), other.e415 * this.e423), 
            /* e5 */
            (other.e12345 * this.e5) + (other.e5 * this.e12345) - (other.e415 * this.e235) - (other.e425 * this.e315) - (other.e435 * this.e125) - (other.e235 * this.e415) - (other.e315 * this.e425) - (other.e125 * this.e435), 
            /* e15, e25, e35, e45 */
            float4((other.e12345 * this.e15) + (other.e315 * this.e4125), (other.e12345 * this.e25) + (other.e125 * this.e4235), (other.e12345 * this.e35) + (other.e235 * this.e4315), -(other.e425 * this.e4315) - (other.e435 * this.e4125)) + (other.group0 * float4(this.group9.www, this.e45)) - (this.group9.yzxx * float4(other.group1.zxy, other.e415)), 
            /* e41, e42, e43 */
            (float3(other.e12345) * this.group4.xyz) + (float3(this.e1234) * other.group0.xyz), 
            /* e23, e31, e12 */
            (float3(other.e12345) * this.group5.xyz) + (float3(this.e1234) * other.group1.xyz), 
            /* e415, e425, e435, e321 */
            float4(((float3(other.e12345) * this.group6.xyz) + (float3(this.e12345) * other.group0.xyz)), other.e12345 * this.e321), 
            /* e423, e431, e412 */
            float3(other.e12345) * this.group7.xyz, 
            /* e235, e315, e125 */
            (float3(other.e12345) * this.group8.xyz) + (float3(this.e12345) * other.group1.xyz), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345) * this.group9, 
            /* e1234 */
            other.e12345 * this.e1234
        );
    }
}
public extension MultiVector {
    public MultiVector anti_wedge(MultiVector other) {
        return MultiVector(
            /* scalar, e12345 */
            float2((other.scalar * this.e12345) + (other.e12345 * this.scalar) + (other.e1 * this.e4235) + (other.e2 * this.e4315) + (other.e3 * this.e4125) + (other.e4 * this.e3215) + (other.e5 * this.e1234) + (other.e4235 * this.e1) + (other.e4315 * this.e2) + (other.e4125 * this.e3) + (other.e3215 * this.e4) + (other.e1234 * this.e5) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412) - (other.e45 * this.e321) - (other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e321 * this.e45) - (other.e423 * this.e15) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43), other.e12345 * this.e12345), 
            /* e1, e2, e3, e4 */
            float4((other.e15 * this.e1234) + (other.e415 * this.e321) + (other.e321 * this.e415) + (other.e4125 * this.e31), (other.e25 * this.e1234) + (other.e425 * this.e321) + (other.e321 * this.e425) + (other.e4235 * this.e12), (other.e35 * this.e1234) + (other.e435 * this.e321) + (other.e321 * this.e435) + (other.e4315 * this.e23), -(other.e415 * this.e423) - (other.e425 * this.e431) - (other.e435 * this.e412) - (other.e1234 * this.e45)) + (float4(other.e12345) * this.group1) + (float4(this.e12345) * other.group1) + (this.group9.yzxx * float4(other.group5.xyz.zxy, other.e41)) + float4((this.group4.xyz * other.group9.www), other.e45 * this.e1234) + float4((other.group7.xyz.zxy * this.group8.xyz.yzx), other.e42 * this.e4315) + float4((other.group8.xyz.yzx * this.group7.xyz.zxy), other.e43 * this.e4125) - (other.group9.yzxy * float4(this.group5.xyz.zxy, this.e42)) - float4((float3(other.e1234) * this.group3.xyz), other.e4125 * this.e43) - float4((other.group4.xyz * this.group9.www), other.e423 * this.e415) - float4((other.group5.xyz.yzx * this.group9.zxy), other.e431 * this.e425) - float4((other.group7.xyz.yzx * this.group8.xyz.zxy), other.e412 * this.e435) - float4((other.group8.xyz.zxy * this.group7.xyz.yzx), other.e4235 * this.e41), 
            /* e5 */
            (other.e12345 * this.e5) + (other.e5 * this.e12345) + (other.e4235 * this.e15) + (other.e4315 * this.e25) + (other.e4125 * this.e35) + (other.e3215 * this.e45) - (other.e15 * this.e4235) - (other.e25 * this.e4315) - (other.e35 * this.e4125) - (other.e45 * this.e3215) - (other.e415 * this.e235) - (other.e425 * this.e315) - (other.e435 * this.e125) - (other.e235 * this.e415) - (other.e315 * this.e425) - (other.e125 * this.e435), 
            /* e15, e25, e35, e45 */
            float4((other.e415 * this.e3215) + (other.e315 * this.e4125) + (other.e4125 * this.e315) + (other.e3215 * this.e415), (other.e425 * this.e3215) + (other.e125 * this.e4235) + (other.e4235 * this.e125) + (other.e3215 * this.e425), (other.e435 * this.e3215) + (other.e235 * this.e4315) + (other.e4315 * this.e235) + (other.e3215 * this.e435), -(other.e435 * this.e4125) - (other.e4235 * this.e415) - (other.e4315 * this.e425) - (other.e4125 * this.e435)) + (float4(other.e12345) * this.group3) + (float4(this.e12345) * other.group3) - (this.group9.yzxx * float4(other.group8.xyz.zxy, other.e415)) - float4((this.group8.xyz.zxy * other.group9.yzx), other.e425 * this.e4315), 
            /* e41, e42, e43 */
            (float3(other.e12345) * this.group4.xyz) + (float3(other.e1234) * this.group6.xyz) + (float3(this.e12345) * other.group4.xyz) + (float3(this.e1234) * other.group6.xyz) + (other.group7.xyz.zxy * this.group9.yzx) + (this.group7.xyz.zxy * other.group9.yzx) - (other.group7.xyz.yzx * this.group9.zxy) - (this.group7.xyz.yzx * other.group9.zxy), 
            /* e23, e31, e12 */
            (float3(other.e12345) * this.group5.xyz) + (float3(other.e3215) * this.group7.xyz) + (float3(other.e1234) * this.group8.xyz) + (float3(this.e12345) * other.group5.xyz) + (float3(this.e3215) * other.group7.xyz) + (float3(this.e1234) * other.group8.xyz) - (float3(other.e321) * this.group9.xyz) - (float3(this.e321) * other.group9.xyz), 
            /* e415, e425, e435, e321 */
            (float4(other.e12345) * this.group6) + (float4(this.e12345) * other.group6) + (other.group9.yzxw * float4(this.group9.zxy, this.e1234)) - float4((other.group9.zxy * this.group9.yzx), other.e1234 * this.e3215), 
            /* e423, e431, e412 */
            (float3(other.e12345) * this.group7.xyz) + (float3(this.e12345) * other.group7.xyz) + (float3(this.e1234) * other.group9.xyz) - (float3(other.e1234) * this.group9.xyz), 
            /* e235, e315, e125 */
            (float3(other.e12345) * this.group8.xyz) + (float3(other.e3215) * this.group9.xyz) + (float3(this.e12345) * other.group8.xyz) - (float3(this.e3215) * other.group9.xyz), 
            /* e4235, e4315, e4125, e3215 */
            (float4(other.e12345) * this.group9) + (float4(this.e12345) * other.group9), 
            /* e1234 */
            (other.e12345 * this.e1234) + (other.e1234 * this.e12345)
        );
    }
}
public extension MultiVector {
    public MultiVector anti_wedge(Plane other) {
        return MultiVector(
            /* scalar, e12345 */
            float2((this.e1 * other.e4235) + (this.e2 * other.e4315) + (this.e3 * other.e4125) + (this.e4 * other.e3215), 0.0), 
            /* e1, e2, e3, e4 */
            float4((this.e41 * other.e3215) + (this.e31 * other.e4125), (this.e42 * other.e3215) + (this.e12 * other.e4235), (this.e43 * other.e3215) + (this.e23 * other.e4315), -(this.e42 * other.e4315) - (this.e43 * other.e4125)) - (other.group0.yzxx * float4(this.group5.xyz.zxy, this.e41)), 
            /* e5 */
            (this.e15 * other.e4235) + (this.e25 * other.e4315) + (this.e35 * other.e4125) + (this.e45 * other.e3215), 
            /* e15, e25, e35, e45 */
            float4((this.e415 * other.e3215) + (this.e315 * other.e4125), (this.e425 * other.e3215) + (this.e125 * other.e4235), (this.e435 * other.e3215) + (this.e235 * other.e4315), -(this.e425 * other.e4315) - (this.e435 * other.e4125)) - (other.group0.yzxx * float4(this.group8.xyz.zxy, this.e415)), 
            /* e41, e42, e43 */
            (this.group7.xyz.zxy * other.group0.yzx) - (this.group7.xyz.yzx * other.group0.zxy), 
            /* e23, e31, e12 */
            (float3(other.e3215) * this.group7.xyz) - (float3(this.e321) * other.group0.xyz), 
            /* e415, e425, e435, e321 */
            float4(((this.group9.zxy * other.group0.yzx) - (this.group9.yzx * other.group0.zxy)), this.e1234 * other.e3215), 
            /* e423, e431, e412 */
            float3(this.e1234) * other.group0.xyz, 
            /* e235, e315, e125 */
            (float3(other.e3215) * this.group9.xyz) - (float3(this.e3215) * other.group0.xyz), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * other.group0, 
            /* e1234 */
            0.0
        );
    }
}
public extension MultiVector {
    public MultiVector anti_wedge(RoundPoint other) {
        return MultiVector(
            /* scalar, e12345 */
            float2((this.e4235 * other.e1) + (this.e4315 * other.e2) + (this.e4125 * other.e3) + (this.e3215 * other.e4) + (this.e1234 * other.e5), 0.0), 
            /* e1, e2, e3, e4 */
            float4(this.e12345) * other.group0, 
            /* e5 */
            this.e12345 * other.e5, 
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
    public Scalar anti_wedge(Scalar other) {
        return Scalar(
            /* scalar */
            this.e12345 * other.scalar
        );
    }
}
public extension MultiVector {
    public MultiVector anti_wedge(Sphere other) {
        return MultiVector(
            /* scalar, e12345 */
            float2((this.e1 * other.e4235) + (this.e2 * other.e4315) + (this.e3 * other.e4125) + (this.e4 * other.e3215) + (this.e5 * other.e1234), 0.0), 
            /* e1, e2, e3, e4 */
            float4((this.e41 * other.e3215) + (this.e31 * other.e4125), (this.e42 * other.e3215) + (this.e12 * other.e4235), (this.e43 * other.e3215) + (this.e23 * other.e4315), -(this.e45 * other.e1234) - (this.e43 * other.e4125)) - (other.group0.yzxx * float4(this.group5.xyz.zxy, this.e41)) - float4((float3(other.e1234) * this.group3.xyz), this.e42 * other.e4315), 
            /* e5 */
            (this.e15 * other.e4235) + (this.e25 * other.e4315) + (this.e35 * other.e4125) + (this.e45 * other.e3215), 
            /* e15, e25, e35, e45 */
            float4((this.e415 * other.e3215) + (this.e315 * other.e4125), (this.e425 * other.e3215) + (this.e125 * other.e4235), (this.e435 * other.e3215) + (this.e235 * other.e4315), -(this.e425 * other.e4315) - (this.e435 * other.e4125)) - (other.group0.yzxx * float4(this.group8.xyz.zxy, this.e415)), 
            /* e41, e42, e43 */
            (float3(other.e1234) * this.group6.xyz) + (this.group7.xyz.zxy * other.group0.yzx) - (this.group7.xyz.yzx * other.group0.zxy), 
            /* e23, e31, e12 */
            (float3(other.e3215) * this.group7.xyz) + (float3(other.e1234) * this.group8.xyz) - (float3(this.e321) * other.group0.xyz), 
            /* e415, e425, e435, e321 */
            float4((this.group9.zxy * other.group0.yzx), this.e1234 * other.e3215) - (this.group9.yzxw * float4(other.group0.zxy, other.e1234)), 
            /* e423, e431, e412 */
            (float3(this.e1234) * other.group0.xyz) - (float3(other.e1234) * this.group9.xyz), 
            /* e235, e315, e125 */
            (float3(other.e3215) * this.group9.xyz) - (float3(this.e3215) * other.group0.xyz), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * other.group0, 
            /* e1234 */
            this.e12345 * other.e1234
        );
    }
}
public extension MultiVector {
    public MultiVector anti_wedge(VersorEven other) {
        return MultiVector(
            /* scalar, e12345 */
            float2((this.scalar * other.e12345) + (this.e4235 * other.e1) + (this.e4315 * other.e2) + (this.e4125 * other.e3) + (this.e3215 * other.e4) + (this.e1234 * other.e5) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412) - (this.e45 * other.e321) - (this.e41 * other.e235) - (this.e42 * other.e315) - (this.e43 * other.e125) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435), this.e12345 * other.e12345), 
            /* e1, e2, e3, e4 */
            float4((this.e1 * other.e12345) + (this.e415 * other.e321) + (this.e321 * other.e415) + (this.e315 * other.e412), (this.e2 * other.e12345) + (this.e425 * other.e321) + (this.e321 * other.e425) + (this.e125 * other.e423), (this.e3 * other.e12345) + (this.e435 * other.e321) + (this.e321 * other.e435) + (this.e235 * other.e431), -(this.e415 * other.e423) - (this.e425 * other.e431) - (this.e435 * other.e412) - (this.e412 * other.e435)) + (float4(this.e12345) * other.group3) + float4((this.group7.xyz.zxy * other.group2.yzx), this.e4 * other.e12345) - float4((this.group7.xyz.yzx * other.group2.zxy), this.e423 * other.e415) - float4((this.group8.xyz.zxy * other.group0.yzx), this.e431 * other.e425), 
            /* e5 */
            (this.e12345 * other.e5) + (this.e5 * other.e12345) - (this.e415 * other.e235) - (this.e425 * other.e315) - (this.e435 * other.e125) - (this.e235 * other.e415) - (this.e315 * other.e425) - (this.e125 * other.e435), 
            /* e15, e25, e35, e45 */
            float4((this.e4125 * other.e315) + (this.e3215 * other.e415), (this.e4235 * other.e125) + (this.e3215 * other.e425), (this.e4315 * other.e235) + (this.e3215 * other.e435), -(this.e4315 * other.e425) - (this.e4125 * other.e435)) + (float4(other.e12345) * this.group3) - (this.group9.yzxx * float4(other.group2.zxy, other.e415)), 
            /* e41, e42, e43 */
            (float3(this.e1234) * other.group1.xyz) + (float3(other.e12345) * this.group4.xyz) + (this.group9.yzx * other.group0.zxy) - (this.group9.zxy * other.group0.yzx), 
            /* e23, e31, e12 */
            (float3(this.e3215) * other.group0.xyz) + (float3(this.e1234) * other.group2.xyz) + (float3(other.e12345) * this.group5.xyz) - (float3(other.e321) * this.group9.xyz), 
            /* e415, e425, e435, e321 */
            (float4(this.e12345) * other.group1) + (float4(other.e12345) * this.group6), 
            /* e423, e431, e412 */
            (float3(this.e12345) * other.group0.xyz) + (float3(other.e12345) * this.group7.xyz), 
            /* e235, e315, e125 */
            (float3(this.e12345) * other.group2.xyz) + (float3(other.e12345) * this.group8.xyz), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345) * this.group9, 
            /* e1234 */
            this.e1234 * other.e12345
        );
    }
}
public extension MultiVector {
    public MultiVector anti_wedge(VersorOdd other) {
        return MultiVector(
            /* scalar, e12345 */
            float2((this.e12345 * other.scalar) + (this.e1 * other.e4235) + (this.e2 * other.e4315) + (this.e3 * other.e4125) + (this.e4 * other.e3215) + (this.e5 * other.e1234) - (this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12) - (this.e321 * other.e45) - (this.e423 * other.e15) - (this.e431 * other.e25) - (this.e412 * other.e35) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43), 0.0), 
            /* e1, e2, e3, e4 */
            (float4(this.e1234) * float4(other.group2.xyz, other.e45)) + (this.group9.yzxz * float4(other.group1.zxy, other.e43)) + float4((this.group4.xyz * other.group3.www), this.e4235 * other.e41) + float4((this.group5.xyz.yzx * other.group3.zxy), this.e4315 * other.e42) - (other.group3.yzxx * float4(this.group5.xyz.zxy, this.e41)) - float4((this.group9.zxy * other.group1.yzx), this.e43 * other.e4125) - float4((this.group9.www * other.group0.xyz), this.e45 * other.e1234) - float4((other.group2.www * this.group3.xyz), this.e42 * other.e4315), 
            /* e5 */
            (this.e15 * other.e4235) + (this.e25 * other.e4315) + (this.e35 * other.e4125) + (this.e45 * other.e3215) - (this.e4235 * other.e15) - (this.e4315 * other.e25) - (this.e4125 * other.e35) - (this.e3215 * other.e45), 
            /* e15, e25, e35, e45 */
            float4((this.e415 * other.e3215) + (this.e315 * other.e4125), (this.e425 * other.e3215) + (this.e125 * other.e4235), (this.e435 * other.e3215) + (this.e235 * other.e4315), -(this.e425 * other.e4315) - (this.e435 * other.e4125)) + (float4(this.e12345) * float4(other.group2.xyz, other.e45)) - (other.group3.yzxx * float4(this.group8.xyz.zxy, this.e415)), 
            /* e41, e42, e43 */
            (float3(this.e12345) * other.group0.xyz) + (float3(other.e1234) * this.group6.xyz) + (this.group7.xyz.zxy * other.group3.yzx) - (this.group7.xyz.yzx * other.group3.zxy), 
            /* e23, e31, e12 */
            (float3(this.e12345) * other.group1.xyz) + (float3(other.e1234) * this.group8.xyz) + (float3(other.e3215) * this.group7.xyz) - (float3(this.e321) * other.group3.xyz), 
            /* e415, e425, e435, e321 */
            float4((this.group9.zxy * other.group3.yzx), this.e1234 * other.e3215) - (this.group9.yzxw * float4(other.group3.zxy, other.e1234)), 
            /* e423, e431, e412 */
            (float3(this.e1234) * other.group3.xyz) - (float3(other.e1234) * this.group9.xyz), 
            /* e235, e315, e125 */
            (float3(other.e3215) * this.group9.xyz) - (float3(this.e3215) * other.group3.xyz), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * other.group3, 
            /* e1234 */
            this.e12345 * other.e1234
        );
    }
}
public extension Plane {
    // Fancy infix trick (first half)
    public AntiWedgeInfixPartial<Plane> operator /(AntiWedgeInfix rhs) {
        return AntiWedgeInfixPartial<Plane>(this);
    }
}
public extension AntiWedgeInfixPartial<Plane> {
    // Fancy infix trick (second half)
    public RoundPoint operator /(AntiCircleRotor rhs) {
        return this.a.anti_wedge(rhs);
    }
}
public extension Plane {
    public RoundPoint anti_wedge(AntiCircleRotor other) {
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(-(other.e41 * this.e3215) - (other.e31 * this.e4125), -(other.e42 * this.e3215) - (other.e12 * this.e4235), -(other.e43 * this.e3215) - (other.e23 * this.e4315), (other.e42 * this.e4315) + (other.e43 * this.e4125)) + (this.group0.yzxx * float4(other.group1.zxy, other.e41)), 
            /* e5 */
            -(other.e45 * this.e3215) - (other.e15 * this.e4235) - (other.e25 * this.e4315) - (other.e35 * this.e4125)
        );
    }
}
public extension Plane {
    public AntiCircleRotor anti_wedge(AntiDipoleInversion other) {
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (other.group0.xyz.zxy * this.group0.yzx) - (other.group0.xyz.yzx * this.group0.zxy), 
            /* e23, e31, e12, e45 */
            float4(other.e423 * this.e3215, other.e431 * this.e3215, other.e412 * this.e3215, -(other.e425 * this.e4315) - (other.e435 * this.e4125)) - (other.group1.wwwx * this.group0.xyzx), 
            /* e15, e25, e35, scalar */
            float4(other.e125 * this.e4315 * -1.0, other.e235 * this.e4125 * -1.0, other.e315 * this.e4235 * -1.0, (other.e2 * this.e4315) + (other.e3 * this.e4125)) + (float4(this.e3215) * float4(other.group1.xyz, other.e4)) + (this.group0.zxyx * float4(other.group2.yzx, other.e1))
        );
    }
}
public extension Plane {
    public AntiFlatPoint anti_wedge(AntiDualNum other) {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(other.group0.xy.xx, other.e3215, 0.0) * float4(float3(1.0), 0.0) * float4(this.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
public extension Plane {
    public AntiLine anti_wedge(AntiFlatPoint other) {
        return AntiLine(
            /* e23, e31, e12 */
            float3(other.e321) * this.group0.xyz * float3(-1.0), 
            /* e15, e25, e35 */
            (other.group0.yzx * this.group0.zxy) - (other.group0.zxy * this.group0.yzx)
        );
    }
}
public extension Plane {
    public AntiMotor anti_wedge(AntiFlector other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((other.group0.www * this.group0.xyz * float3(-1.0)), (other.e1 * this.e4235) + (other.e2 * this.e4315) + (other.e3 * this.e4125)), 
            /* e15, e25, e35, e3215 */
            float4(((other.group0.yzx * this.group0.zxy) - (other.group0.zxy * this.group0.yzx)), 0.0)
        );
    }
}
public extension Plane {
    public AntiPlane anti_wedge(AntiLine other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(other.e12 * this.e4315, other.e23 * this.e4125, other.e31 * this.e4235, -(other.e25 * this.e4315) - (other.e35 * this.e4125)) - (this.group0.zxyx * float4(other.group0.xyz.yzx, other.e15))
        );
    }
}
public extension Plane {
    public AntiFlector anti_wedge(AntiMotor other) {
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(float3(1.0), 0.0) * float4(other.group1.www, 0.0) * float4(this.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e1, e2, e3, e5 */
            float4(other.e12 * this.e4315, other.e23 * this.e4125, other.e31 * this.e4235, -(other.e25 * this.e4315) - (other.e35 * this.e4125)) - (this.group0.zxyx * float4(other.group0.yzx, other.e15))
        );
    }
}
public extension Plane {
    public Scalar anti_wedge(AntiPlane other) {
        return Scalar(
            /* scalar */
            (other.e1 * this.e4235) + (other.e2 * this.e4315) + (other.e3 * this.e4125)
        );
    }
}
public extension Plane {
    public Plane anti_wedge(AntiScalar other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345) * this.group0
        );
    }
}
public extension Plane {
    public Dipole anti_wedge(Circle other) {
        return Dipole(
            /* e41, e42, e43 */
            (other.group0.xyz.zxy * this.group0.yzx) - (other.group0.xyz.yzx * this.group0.zxy), 
            /* e23, e31, e12, e45 */
            float4(other.e423 * this.e3215, other.e431 * this.e3215, other.e412 * this.e3215, -(other.e425 * this.e4315) - (other.e435 * this.e4125)) - (other.group1.wwwx * this.group0.xyzx), 
            /* e15, e25, e35 */
            (float3(this.e3215) * other.group1.xyz) + (other.group2.xyz.yzx * this.group0.zxy) - (other.group2.xyz.zxy * this.group0.yzx)
        );
    }
}
public extension Plane {
    public DipoleInversion anti_wedge(CircleRotor other) {
        return DipoleInversion(
            /* e41, e42, e43 */
            (other.group0.xyz.zxy * this.group0.yzx) - (other.group0.xyz.yzx * this.group0.zxy), 
            /* e23, e31, e12, e45 */
            float4(other.e423 * this.e3215, other.e431 * this.e3215, other.e412 * this.e3215, -(other.e425 * this.e4315) - (other.e435 * this.e4125)) - (other.group1.wwwx * this.group0.xyzx), 
            /* e15, e25, e35, e1234 */
            float4((float3(this.e3215) * other.group1.xyz), 0.0) + float4((other.group2.yzx * this.group0.zxy), 0.0) - float4((other.group2.zxy * this.group0.yzx), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345) * this.group0
        );
    }
}
public extension Plane {
    public RoundPoint anti_wedge(Dipole other) {
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(-(other.e41 * this.e3215) - (other.e31 * this.e4125), -(other.e42 * this.e3215) - (other.e12 * this.e4235), -(other.e43 * this.e3215) - (other.e23 * this.e4315), (other.e42 * this.e4315) + (other.e43 * this.e4125)) + (this.group0.yzxx * float4(other.group1.zxy, other.e41)), 
            /* e5 */
            -(other.e45 * this.e3215) - (other.e15 * this.e4235) - (other.e25 * this.e4315) - (other.e35 * this.e4125)
        );
    }
}
public extension Plane {
    public AntiDipoleInversion anti_wedge(DipoleInversion other) {
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(other.e1234) * this.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            float4(((other.group3.yzx * this.group0.zxy) - (other.group3.zxy * this.group0.yzx)), other.e1234 * this.e3215 * -1.0), 
            /* e235, e315, e125, e4 */
            float4(other.e4235 * this.e3215 * -1.0, other.e4315 * this.e3215 * -1.0, other.e4125 * this.e3215 * -1.0, (other.e42 * this.e4315) + (other.e43 * this.e4125)) + (this.group0.xyzx * float4(other.group3.www, other.e41)), 
            /* e1, e2, e3, e5 */
            float4(other.e12 * this.e4315, other.e23 * this.e4125, other.e31 * this.e4235, -(other.e25 * this.e4315) - (other.e35 * this.e4125)) - (float4(this.e3215) * float4(other.group0.xyz, other.e45)) - (this.group0.zxyx * float4(other.group1.yzx, other.e15))
        );
    }
}
public extension Plane {
    public Plane anti_wedge(DualNum other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345) * this.group0
        );
    }
}
public extension Plane {
    public DualNum anti_wedge(FlatPoint other) {
        return DualNum(
            /* e5, e12345 */
            float2(-(other.e15 * this.e4235) - (other.e25 * this.e4315) - (other.e35 * this.e4125) - (other.e45 * this.e3215), 0.0)
        );
    }
}
public extension Plane {
    public Motor anti_wedge(Flector other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(((other.group1.yzx * this.group0.zxy) - (other.group1.zxy * this.group0.yzx)), 0.0), 
            /* e235, e315, e125, e5 */
            float4(other.e3215 * this.e4235, other.e3215 * this.e4315, other.e3215 * this.e4125, -(other.e25 * this.e4315) - (other.e35 * this.e4125) - (other.e45 * this.e3215)) - (this.group0.wwwx * float4(other.group1.xyz, other.e15))
        );
    }
}
public extension Plane {
    public FlatPoint anti_wedge(Line other) {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4((other.e415 * this.e3215) + (other.e315 * this.e4125), (other.e425 * this.e3215) + (other.e125 * this.e4235), (other.e435 * this.e3215) + (other.e235 * this.e4315), -(other.e425 * this.e4315) - (other.e435 * this.e4125)) - (this.group0.yzxx * float4(other.group1.xyz.zxy, other.e415))
        );
    }
}
public extension Plane {
    public Flector anti_wedge(Motor other) {
        return Flector(
            /* e15, e25, e35, e45 */
            float4((other.e415 * this.e3215) + (other.e315 * this.e4125), (other.e425 * this.e3215) + (other.e125 * this.e4235), (other.e435 * this.e3215) + (other.e235 * this.e4315), -(other.e425 * this.e4315) - (other.e435 * this.e4125)) - (this.group0.yzxx * float4(other.group1.zxy, other.e415)), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345) * this.group0
        );
    }
}
public extension Plane {
    public MultiVector anti_wedge(MultiVector other) {
        return MultiVector(
            /* scalar, e12345 */
            float2((other.e1 * this.e4235) + (other.e2 * this.e4315) + (other.e3 * this.e4125) + (other.e4 * this.e3215), 0.0), 
            /* e1, e2, e3, e4 */
            float4(-(other.e41 * this.e3215) - (other.e31 * this.e4125), -(other.e42 * this.e3215) - (other.e12 * this.e4235), -(other.e43 * this.e3215) - (other.e23 * this.e4315), (other.e42 * this.e4315) + (other.e43 * this.e4125)) + (this.group0.yzxx * float4(other.group5.xyz.zxy, other.e41)), 
            /* e5 */
            -(other.e15 * this.e4235) - (other.e25 * this.e4315) - (other.e35 * this.e4125) - (other.e45 * this.e3215), 
            /* e15, e25, e35, e45 */
            float4((other.e415 * this.e3215) + (other.e315 * this.e4125), (other.e425 * this.e3215) + (other.e125 * this.e4235), (other.e435 * this.e3215) + (other.e235 * this.e4315), -(other.e425 * this.e4315) - (other.e435 * this.e4125)) - (this.group0.yzxx * float4(other.group8.xyz.zxy, other.e415)), 
            /* e41, e42, e43 */
            (other.group7.xyz.zxy * this.group0.yzx) - (other.group7.xyz.yzx * this.group0.zxy), 
            /* e23, e31, e12 */
            (float3(this.e3215) * other.group7.xyz) - (float3(other.e321) * this.group0.xyz), 
            /* e415, e425, e435, e321 */
            float4(((other.group9.yzx * this.group0.zxy) - (other.group9.zxy * this.group0.yzx)), other.e1234 * this.e3215 * -1.0), 
            /* e423, e431, e412 */
            float3(other.e1234) * this.group0.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            (float3(other.e3215) * this.group0.xyz) - (float3(this.e3215) * other.group9.xyz), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345) * this.group0, 
            /* e1234 */
            0.0
        );
    }
}
public extension Plane {
    public Line anti_wedge(Plane other) {
        return Line(
            /* e415, e425, e435 */
            (other.group0.yzx * this.group0.zxy) - (other.group0.zxy * this.group0.yzx), 
            /* e235, e315, e125 */
            (float3(other.e3215) * this.group0.xyz) - (float3(this.e3215) * other.group0.xyz)
        );
    }
}
public extension Plane {
    public Scalar anti_wedge(RoundPoint other) {
        return Scalar(
            /* scalar */
            (this.e4235 * other.e1) + (this.e4315 * other.e2) + (this.e4125 * other.e3) + (this.e3215 * other.e4)
        );
    }
}
public extension Plane {
    public Circle anti_wedge(Sphere other) {
        return Circle(
            /* e423, e431, e412 */
            float3(other.e1234) * this.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            float4(((this.group0.zxy * other.group0.yzx) - (this.group0.yzx * other.group0.zxy)), this.e3215 * other.e1234 * -1.0), 
            /* e235, e315, e125 */
            (float3(other.e3215) * this.group0.xyz) - (float3(this.e3215) * other.group0.xyz)
        );
    }
}
public extension Plane {
    public VersorOdd anti_wedge(VersorEven other) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(this.e4125 * other.e431 * -1.0, this.e4235 * other.e412 * -1.0, this.e4315 * other.e423 * -1.0, (this.e4315 * other.e2) + (this.e4125 * other.e3) + (this.e3215 * other.e4)) + (this.group0.yzxx * float4(other.group0.zxy, other.e1)), 
            /* e23, e31, e12, e45 */
            float4(this.e3215 * other.e423, this.e3215 * other.e431, this.e3215 * other.e412, -(this.e4315 * other.e425) - (this.e4125 * other.e435)) - (this.group0.xyzx * other.group1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4((float3(this.e3215) * other.group1.xyz), 0.0) + float4((this.group0.zxy * other.group2.yzx), 0.0) - float4((this.group0.yzx * other.group2.zxy), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345) * this.group0
        );
    }
}
public extension Plane {
    public AntiDipoleInversion anti_wedge(VersorOdd other) {
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(other.e1234) * this.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            float4(((this.group0.zxy * other.group3.yzx) - (this.group0.yzx * other.group3.zxy)), this.e3215 * other.e1234 * -1.0), 
            /* e235, e315, e125, e4 */
            float4(this.e3215 * other.e4235 * -1.0, this.e3215 * other.e4315 * -1.0, this.e3215 * other.e4125 * -1.0, (this.e4315 * other.e42) + (this.e4125 * other.e43)) + (this.group0.xyzx * float4(other.group3.www, other.e41)), 
            /* e1, e2, e3, e5 */
            float4(this.e4315 * other.e12, this.e4125 * other.e23, this.e4235 * other.e31, -(this.e4125 * other.e35) - (this.e3215 * other.e45)) - (this.group0.zxyx * float4(other.group1.yzx, other.e15)) - (this.group0.wwwy * float4(other.group0.xyz, other.e25))
        );
    }
}
public extension RoundPoint {
    // Fancy infix trick (first half)
    public AntiWedgeInfixPartial<RoundPoint> operator /(AntiWedgeInfix rhs) {
        return AntiWedgeInfixPartial<RoundPoint>(this);
    }
}
public extension AntiWedgeInfixPartial<RoundPoint> {
    // Fancy infix trick (second half)
    public Scalar operator /(AntiDualNum rhs) {
        return this.a.anti_wedge(rhs);
    }
}
public extension RoundPoint {
    public Scalar anti_wedge(AntiDualNum other) {
        return Scalar(
            /* scalar */
            other.e3215 * this.e4
        );
    }
}
public extension RoundPoint {
    public Scalar anti_wedge(AntiMotor other) {
        return Scalar(
            /* scalar */
            other.e3215 * this.e4
        );
    }
}
public extension RoundPoint {
    public RoundPoint anti_wedge(AntiScalar other) {
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(other.e12345) * this.group0, 
            /* e5 */
            other.e12345 * this.e5
        );
    }
}
public extension RoundPoint {
    public RoundPoint anti_wedge(CircleRotor other) {
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(other.e12345) * this.group0, 
            /* e5 */
            other.e12345 * this.e5
        );
    }
}
public extension RoundPoint {
    public Scalar anti_wedge(DipoleInversion other) {
        return Scalar(
            /* scalar */
            (other.e1234 * this.e5) + (other.e4235 * this.e1) + (other.e4315 * this.e2) + (other.e4125 * this.e3) + (other.e3215 * this.e4)
        );
    }
}
public extension RoundPoint {
    public RoundPoint anti_wedge(DualNum other) {
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(other.e12345) * this.group0, 
            /* e5 */
            other.e12345 * this.e5
        );
    }
}
public extension RoundPoint {
    public Scalar anti_wedge(Flector other) {
        return Scalar(
            /* scalar */
            (other.e4235 * this.e1) + (other.e4315 * this.e2) + (other.e4125 * this.e3) + (other.e3215 * this.e4)
        );
    }
}
public extension RoundPoint {
    public RoundPoint anti_wedge(Motor other) {
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(other.e12345) * this.group0, 
            /* e5 */
            other.e12345 * this.e5
        );
    }
}
public extension RoundPoint {
    public MultiVector anti_wedge(MultiVector other) {
        return MultiVector(
            /* scalar, e12345 */
            float2((other.e4235 * this.e1) + (other.e4315 * this.e2) + (other.e4125 * this.e3) + (other.e3215 * this.e4) + (other.e1234 * this.e5), 0.0), 
            /* e1, e2, e3, e4 */
            float4(other.e12345) * this.group0, 
            /* e5 */
            other.e12345 * this.e5, 
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
public extension RoundPoint {
    public Scalar anti_wedge(Plane other) {
        return Scalar(
            /* scalar */
            (other.e4235 * this.e1) + (other.e4315 * this.e2) + (other.e4125 * this.e3) + (other.e3215 * this.e4)
        );
    }
}
public extension RoundPoint {
    public Scalar anti_wedge(Sphere other) {
        return Scalar(
            /* scalar */
            (this.e1 * other.e4235) + (this.e2 * other.e4315) + (this.e3 * other.e4125) + (this.e4 * other.e3215) + (this.e5 * other.e1234)
        );
    }
}
public extension RoundPoint {
    public RoundPoint anti_wedge(VersorEven other) {
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(other.e12345) * this.group0, 
            /* e5 */
            this.e5 * other.e12345
        );
    }
}
public extension RoundPoint {
    public Scalar anti_wedge(VersorOdd other) {
        return Scalar(
            /* scalar */
            (this.e1 * other.e4235) + (this.e2 * other.e4315) + (this.e3 * other.e4125) + (this.e4 * other.e3215) + (this.e5 * other.e1234)
        );
    }
}
public extension Scalar {
    // Fancy infix trick (first half)
    public AntiWedgeInfixPartial<Scalar> operator /(AntiWedgeInfix rhs) {
        return AntiWedgeInfixPartial<Scalar>(this);
    }
}
public extension AntiWedgeInfixPartial<Scalar> {
    // Fancy infix trick (second half)
    public Scalar operator /(AntiScalar rhs) {
        return this.a.anti_wedge(rhs);
    }
}
public extension Scalar {
    public Scalar anti_wedge(AntiScalar other) {
        return Scalar(
            /* scalar */
            other.e12345 * this.scalar
        );
    }
}
public extension Scalar {
    public Scalar anti_wedge(CircleRotor other) {
        return Scalar(
            /* scalar */
            other.e12345 * this.scalar
        );
    }
}
public extension Scalar {
    public Scalar anti_wedge(DualNum other) {
        return Scalar(
            /* scalar */
            other.e12345 * this.scalar
        );
    }
}
public extension Scalar {
    public Scalar anti_wedge(Motor other) {
        return Scalar(
            /* scalar */
            other.e12345 * this.scalar
        );
    }
}
public extension Scalar {
    public Scalar anti_wedge(MultiVector other) {
        return Scalar(
            /* scalar */
            other.e12345 * this.scalar
        );
    }
}
public extension Scalar {
    public Scalar anti_wedge(VersorEven other) {
        return Scalar(
            /* scalar */
            this.scalar * other.e12345
        );
    }
}
public extension Sphere {
    // Fancy infix trick (first half)
    public AntiWedgeInfixPartial<Sphere> operator /(AntiWedgeInfix rhs) {
        return AntiWedgeInfixPartial<Sphere>(this);
    }
}
public extension AntiWedgeInfixPartial<Sphere> {
    // Fancy infix trick (second half)
    public RoundPoint operator /(AntiCircleRotor rhs) {
        return this.a.anti_wedge(rhs);
    }
}
public extension Sphere {
    public RoundPoint anti_wedge(AntiCircleRotor other) {
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(-(other.e41 * this.e3215) - (other.e31 * this.e4125), -(other.e42 * this.e3215) - (other.e12 * this.e4235), -(other.e43 * this.e3215) - (other.e23 * this.e4315), (other.e43 * this.e4125) + (other.e45 * this.e1234)) + (this.group0.yzxx * float4(other.group1.zxy, other.e41)) + float4((float3(this.e1234) * other.group2.xyz), other.e42 * this.e4315), 
            /* e5 */
            -(other.e45 * this.e3215) - (other.e15 * this.e4235) - (other.e25 * this.e4315) - (other.e35 * this.e4125)
        );
    }
}
public extension Sphere {
    public AntiCircleRotor anti_wedge(AntiDipoleInversion other) {
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (float3(this.e1234) * other.group1.xyz) + (other.group0.xyz.zxy * this.group0.yzx) - (other.group0.xyz.yzx * this.group0.zxy), 
            /* e23, e31, e12, e45 */
            float4((other.e423 * this.e3215) + (other.e235 * this.e1234), (other.e431 * this.e3215) + (other.e315 * this.e1234), (other.e412 * this.e3215) + (other.e125 * this.e1234), -(other.e425 * this.e4315) - (other.e435 * this.e4125)) - (other.group1.wwwx * this.group0.xyzx), 
            /* e15, e25, e35, scalar */
            float4(other.e125 * this.e4315 * -1.0, other.e235 * this.e4125 * -1.0, other.e315 * this.e4235 * -1.0, (other.e2 * this.e4315) + (other.e3 * this.e4125) + (other.e5 * this.e1234)) + (float4(this.e3215) * float4(other.group1.xyz, other.e4)) + (this.group0.zxyx * float4(other.group2.yzx, other.e1))
        );
    }
}
public extension Sphere {
    public AntiFlatPoint anti_wedge(AntiDualNum other) {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(other.e3215) * float4(this.group0.xyz, this.e1234)
        );
    }
}
public extension Sphere {
    public AntiLine anti_wedge(AntiFlatPoint other) {
        return AntiLine(
            /* e23, e31, e12 */
            (float3(this.e1234) * other.group0.xyz) - (float3(other.e321) * this.group0.xyz), 
            /* e15, e25, e35 */
            (other.group0.yzx * this.group0.zxy) - (other.group0.zxy * this.group0.yzx)
        );
    }
}
public extension Sphere {
    public AntiMotor anti_wedge(AntiFlector other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(other.e321 * this.e4235 * -1.0, other.e321 * this.e4315 * -1.0, other.e321 * this.e4125 * -1.0, (other.e2 * this.e4315) + (other.e3 * this.e4125) + (other.e5 * this.e1234)) + float4((float3(this.e1234) * other.group0.xyz), other.e1 * this.e4235), 
            /* e15, e25, e35, e3215 */
            float4(((other.group0.yzx * this.group0.zxy) - (other.group0.zxy * this.group0.yzx)), 0.0)
        );
    }
}
public extension Sphere {
    public AntiPlane anti_wedge(AntiLine other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4((other.e12 * this.e4315) + (other.e15 * this.e1234), (other.e23 * this.e4125) + (other.e25 * this.e1234), (other.e31 * this.e4235) + (other.e35 * this.e1234), -(other.e25 * this.e4315) - (other.e35 * this.e4125)) - (this.group0.zxyx * float4(other.group0.xyz.yzx, other.e15))
        );
    }
}
public extension Sphere {
    public AntiFlector anti_wedge(AntiMotor other) {
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(other.e3215) * float4(this.group0.xyz, this.e1234), 
            /* e1, e2, e3, e5 */
            float4((other.e12 * this.e4315) + (other.e15 * this.e1234), (other.e23 * this.e4125) + (other.e25 * this.e1234), (other.e31 * this.e4235) + (other.e35 * this.e1234), -(other.e25 * this.e4315) - (other.e35 * this.e4125)) - (this.group0.zxyx * float4(other.group0.yzx, other.e15))
        );
    }
}
public extension Sphere {
    public Scalar anti_wedge(AntiPlane other) {
        return Scalar(
            /* scalar */
            (other.e1 * this.e4235) + (other.e2 * this.e4315) + (other.e3 * this.e4125) + (other.e5 * this.e1234)
        );
    }
}
public extension Sphere {
    public Sphere anti_wedge(AntiScalar other) {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345) * this.group0, 
            /* e1234 */
            other.e12345 * this.e1234
        );
    }
}
public extension Sphere {
    public Dipole anti_wedge(Circle other) {
        return Dipole(
            /* e41, e42, e43 */
            (float3(this.e1234) * other.group1.xyz) + (other.group0.xyz.zxy * this.group0.yzx) - (other.group0.xyz.yzx * this.group0.zxy), 
            /* e23, e31, e12, e45 */
            float4((other.e423 * this.e3215) + (other.e235 * this.e1234), (other.e431 * this.e3215) + (other.e315 * this.e1234), (other.e412 * this.e3215) + (other.e125 * this.e1234), -(other.e425 * this.e4315) - (other.e435 * this.e4125)) - (other.group1.wwwx * this.group0.xyzx), 
            /* e15, e25, e35 */
            (float3(this.e3215) * other.group1.xyz) + (other.group2.xyz.yzx * this.group0.zxy) - (other.group2.xyz.zxy * this.group0.yzx)
        );
    }
}
public extension Sphere {
    public DipoleInversion anti_wedge(CircleRotor other) {
        return DipoleInversion(
            /* e41, e42, e43 */
            (float3(this.e1234) * other.group1.xyz) + (other.group0.xyz.zxy * this.group0.yzx) - (other.group0.xyz.yzx * this.group0.zxy), 
            /* e23, e31, e12, e45 */
            float4((other.e423 * this.e3215) + (other.e235 * this.e1234), (other.e431 * this.e3215) + (other.e315 * this.e1234), (other.e412 * this.e3215) + (other.e125 * this.e1234), -(other.e425 * this.e4315) - (other.e435 * this.e4125)) - (other.group1.wwwx * this.group0.xyzx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(this.e3215) * other.group1.xyz) + (other.group2.yzx * this.group0.zxy) - (other.group2.zxy * this.group0.yzx)), other.e12345 * this.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345) * this.group0
        );
    }
}
public extension Sphere {
    public RoundPoint anti_wedge(Dipole other) {
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(-(other.e41 * this.e3215) - (other.e31 * this.e4125), -(other.e42 * this.e3215) - (other.e12 * this.e4235), -(other.e43 * this.e3215) - (other.e23 * this.e4315), (other.e43 * this.e4125) + (other.e45 * this.e1234)) + (this.group0.yzxy * float4(other.group1.zxy, other.e42)) + float4((float3(this.e1234) * other.group2.xyz), other.e41 * this.e4235), 
            /* e5 */
            -(other.e45 * this.e3215) - (other.e15 * this.e4235) - (other.e25 * this.e4315) - (other.e35 * this.e4125)
        );
    }
}
public extension Sphere {
    public AntiDipoleInversion anti_wedge(DipoleInversion other) {
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (float3(this.e1234) * other.group3.xyz) - (float3(other.e1234) * this.group0.xyz), 
            /* e415, e425, e435, e321 */
            (other.group3.yzxw * float4(this.group0.zxy, this.e1234)) - (this.group0.yzxw * float4(other.group3.zxy, other.e1234)), 
            /* e235, e315, e125, e4 */
            float4(other.e4235 * this.e3215 * -1.0, other.e4315 * this.e3215 * -1.0, other.e4125 * this.e3215 * -1.0, (other.e42 * this.e4315) + (other.e43 * this.e4125) + (other.e45 * this.e1234)) + (this.group0.xyzx * float4(other.group3.www, other.e41)), 
            /* e1, e2, e3, e5 */
            float4((other.e12 * this.e4315) + (other.e15 * this.e1234), (other.e23 * this.e4125) + (other.e25 * this.e1234), (other.e31 * this.e4235) + (other.e35 * this.e1234), -(other.e25 * this.e4315) - (other.e35 * this.e4125)) - (float4(this.e3215) * float4(other.group0.xyz, other.e45)) - (this.group0.zxyx * float4(other.group1.yzx, other.e15))
        );
    }
}
public extension Sphere {
    public VersorOdd anti_wedge(DualNum other) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(float3(0.0), other.e5 * this.e1234), 
            /* e23, e31, e12, e45 */
            float4(0.0), 
            /* e15, e25, e35, e1234 */
            float4(float3(0.0), other.e12345 * this.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345) * this.group0
        );
    }
}
public extension Sphere {
    public RoundPoint anti_wedge(FlatPoint other) {
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(this.e1234) * other.group0, 
            /* e5 */
            -(other.e15 * this.e4235) - (other.e25 * this.e4315) - (other.e35 * this.e4125) - (other.e45 * this.e3215)
        );
    }
}
public extension Sphere {
    public AntiDipoleInversion anti_wedge(Flector other) {
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(this.e1234) * other.group1.xyz, 
            /* e415, e425, e435, e321 */
            float4(((other.group1.yzx * this.group0.zxy) - (other.group1.zxy * this.group0.yzx)), other.e3215 * this.e1234), 
            /* e235, e315, e125, e4 */
            float4(((float3(other.e3215) * this.group0.xyz) - (float3(this.e3215) * other.group1.xyz)), other.e45 * this.e1234), 
            /* e1, e2, e3, e5 */
            float4((float3(this.e1234) * other.group0.xyz), -(other.e15 * this.e4235) - (other.e25 * this.e4315) - (other.e35 * this.e4125) - (other.e45 * this.e3215))
        );
    }
}
public extension Sphere {
    public Dipole anti_wedge(Line other) {
        return Dipole(
            /* e41, e42, e43 */
            float3(this.e1234) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4((float3(this.e1234) * other.group1.xyz), -(other.e415 * this.e4235) - (other.e425 * this.e4315) - (other.e435 * this.e4125)), 
            /* e15, e25, e35 */
            (float3(this.e3215) * other.group0.xyz) + (other.group1.xyz.yzx * this.group0.zxy) - (other.group1.xyz.zxy * this.group0.yzx)
        );
    }
}
public extension Sphere {
    public VersorOdd anti_wedge(Motor other) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(this.e1234) * float4(other.group0.xyz, other.e5), 
            /* e23, e31, e12, e45 */
            float4((float3(this.e1234) * other.group1.xyz), -(other.e415 * this.e4235) - (other.e425 * this.e4315) - (other.e435 * this.e4125)), 
            /* e15, e25, e35, e1234 */
            float4(((float3(this.e3215) * other.group0.xyz) + (other.group1.yzx * this.group0.zxy) - (other.group1.zxy * this.group0.yzx)), other.e12345 * this.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345) * this.group0
        );
    }
}
public extension Sphere {
    public MultiVector anti_wedge(MultiVector other) {
        return MultiVector(
            /* scalar, e12345 */
            float2((other.e1 * this.e4235) + (other.e2 * this.e4315) + (other.e3 * this.e4125) + (other.e4 * this.e3215) + (other.e5 * this.e1234), 0.0), 
            /* e1, e2, e3, e4 */
            float4(-(other.e41 * this.e3215) - (other.e31 * this.e4125), -(other.e42 * this.e3215) - (other.e12 * this.e4235), -(other.e43 * this.e3215) - (other.e23 * this.e4315), (other.e45 * this.e1234) + (other.e43 * this.e4125)) + (this.group0.yzxx * float4(other.group5.xyz.zxy, other.e41)) + float4((float3(this.e1234) * other.group3.xyz), other.e42 * this.e4315), 
            /* e5 */
            -(other.e15 * this.e4235) - (other.e25 * this.e4315) - (other.e35 * this.e4125) - (other.e45 * this.e3215), 
            /* e15, e25, e35, e45 */
            float4((other.e415 * this.e3215) + (other.e315 * this.e4125), (other.e425 * this.e3215) + (other.e125 * this.e4235), (other.e435 * this.e3215) + (other.e235 * this.e4315), -(other.e425 * this.e4315) - (other.e435 * this.e4125)) - (this.group0.yzxx * float4(other.group8.xyz.zxy, other.e415)), 
            /* e41, e42, e43 */
            (float3(this.e1234) * other.group6.xyz) + (other.group7.xyz.zxy * this.group0.yzx) - (other.group7.xyz.yzx * this.group0.zxy), 
            /* e23, e31, e12 */
            (float3(this.e3215) * other.group7.xyz) + (float3(this.e1234) * other.group8.xyz) - (float3(other.e321) * this.group0.xyz), 
            /* e415, e425, e435, e321 */
            (other.group9.yzxw * float4(this.group0.zxy, this.e1234)) - float4((other.group9.zxy * this.group0.yzx), other.e1234 * this.e3215), 
            /* e423, e431, e412 */
            (float3(this.e1234) * other.group9.xyz) - (float3(other.e1234) * this.group0.xyz), 
            /* e235, e315, e125 */
            (float3(other.e3215) * this.group0.xyz) - (float3(this.e3215) * other.group9.xyz), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345) * this.group0, 
            /* e1234 */
            other.e12345 * this.e1234
        );
    }
}
public extension Sphere {
    public Circle anti_wedge(Plane other) {
        return Circle(
            /* e423, e431, e412 */
            float3(this.e1234) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(((other.group0.yzx * this.group0.zxy) - (other.group0.zxy * this.group0.yzx)), other.e3215 * this.e1234), 
            /* e235, e315, e125 */
            (float3(other.e3215) * this.group0.xyz) - (float3(this.e3215) * other.group0.xyz)
        );
    }
}
public extension Sphere {
    public Scalar anti_wedge(RoundPoint other) {
        return Scalar(
            /* scalar */
            (other.e1 * this.e4235) + (other.e2 * this.e4315) + (other.e3 * this.e4125) + (other.e4 * this.e3215) + (other.e5 * this.e1234)
        );
    }
}
public extension Sphere {
    public Circle anti_wedge(Sphere other) {
        return Circle(
            /* e423, e431, e412 */
            (float3(this.e1234) * other.group0.xyz) - (float3(other.e1234) * this.group0.xyz), 
            /* e415, e425, e435, e321 */
            (other.group0.yzxw * float4(this.group0.zxy, this.e1234)) - float4((other.group0.zxy * this.group0.yzx), other.e1234 * this.e3215), 
            /* e235, e315, e125 */
            (float3(other.e3215) * this.group0.xyz) - (float3(this.e3215) * other.group0.xyz)
        );
    }
}
public extension Sphere {
    public VersorOdd anti_wedge(VersorEven other) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(this.e4125 * other.e431 * -1.0, this.e4235 * other.e412 * -1.0, this.e4315 * other.e423 * -1.0, (this.e4125 * other.e3) + (this.e3215 * other.e4) + (this.e1234 * other.e5)) + (this.group0.yzxx * float4(other.group0.zxy, other.e1)) + float4((float3(this.e1234) * other.group1.xyz), this.e4315 * other.e2), 
            /* e23, e31, e12, e45 */
            float4((this.e3215 * other.e423) + (this.e1234 * other.e235), (this.e3215 * other.e431) + (this.e1234 * other.e315), (this.e3215 * other.e412) + (this.e1234 * other.e125), -(this.e4315 * other.e425) - (this.e4125 * other.e435)) - (this.group0.xyzx * other.group1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(this.e3215) * other.group1.xyz) + (this.group0.zxy * other.group2.yzx) - (this.group0.yzx * other.group2.zxy)), this.e1234 * other.e12345), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345) * this.group0
        );
    }
}
public extension Sphere {
    public AntiDipoleInversion anti_wedge(VersorOdd other) {
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (float3(this.e1234) * other.group3.xyz) - (float3(other.e1234) * this.group0.xyz), 
            /* e415, e425, e435, e321 */
            float4((this.group0.zxy * other.group3.yzx), this.e1234 * other.e3215) - (this.group0.yzxw * float4(other.group3.zxy, other.e1234)), 
            /* e235, e315, e125, e4 */
            float4(this.e3215 * other.e4235 * -1.0, this.e3215 * other.e4315 * -1.0, this.e3215 * other.e4125 * -1.0, (this.e4315 * other.e42) + (this.e4125 * other.e43) + (this.e1234 * other.e45)) + (this.group0.xyzx * float4(other.group3.www, other.e41)), 
            /* e1, e2, e3, e5 */
            float4((this.e4315 * other.e12) + (this.e1234 * other.e15), (this.e4125 * other.e23) + (this.e1234 * other.e25), (this.e4235 * other.e31) + (this.e1234 * other.e35), -(this.e4125 * other.e35) - (this.e3215 * other.e45)) - (this.group0.zxyx * float4(other.group1.yzx, other.e15)) - (this.group0.wwwy * float4(other.group0.xyz, other.e25))
        );
    }
}
public extension VersorEven {
    // Fancy infix trick (first half)
    public AntiWedgeInfixPartial<VersorEven> operator /(AntiWedgeInfix rhs) {
        return AntiWedgeInfixPartial<VersorEven>(this);
    }
}
public extension AntiWedgeInfixPartial<VersorEven> {
    // Fancy infix trick (second half)
    public AntiCircleRotor operator /(AntiCircleRotor rhs) {
        return this.a.anti_wedge(rhs);
    }
}
public extension VersorEven {
    public AntiCircleRotor anti_wedge(AntiCircleRotor other) {
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(this.e12345) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(this.e12345) * other.group1, 
            /* e15, e25, e35, scalar */
            float4((float3(this.e12345) * other.group2.xyz), (other.scalar * this.e12345) - (other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e45 * this.e321) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412))
        );
    }
}
public extension VersorEven {
    public AntiDipoleInversion anti_wedge(AntiDipoleInversion other) {
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(this.e12345) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(this.e12345) * other.group1, 
            /* e235, e315, e125, e4 */
            float4((float3(this.e12345) * other.group2.xyz), (other.e4 * this.e12345) - (other.e423 * this.e415) - (other.e431 * this.e425) - (other.e412 * this.e435) - (other.e415 * this.e423) - (other.e425 * this.e431) - (other.e435 * this.e412)), 
            /* e1, e2, e3, e5 */
            float4((other.e415 * this.e321) + (other.e321 * this.e415) + (other.e315 * this.e412) + (other.e1 * this.e12345), (other.e425 * this.e321) + (other.e321 * this.e425) + (other.e125 * this.e423) + (other.e2 * this.e12345), (other.e435 * this.e321) + (other.e321 * this.e435) + (other.e235 * this.e431) + (other.e3 * this.e12345), -(other.e435 * this.e125) - (other.e235 * this.e415) - (other.e315 * this.e425) - (other.e125 * this.e435)) + float4((other.group0.xyz.zxy * this.group2.yzx), other.e5 * this.e12345) - (this.group2.zxyx * float4(other.group0.xyz.yzx, other.e415)) - float4((other.group2.zxy * this.group0.yzx), other.e425 * this.e315)
        );
    }
}
public extension VersorEven {
    public AntiMotor anti_wedge(AntiDualNum other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(other.group0.xy.xx, other.e3215, (other.e3215 * this.e4) + (other.scalar * this.e12345)) * float4(this.group0.xyz, 1.0), 
            /* e15, e25, e35, e3215 */
            float4(other.e3215) * float4(this.group1.xyz, this.e12345)
        );
    }
}
public extension VersorEven {
    public AntiFlector anti_wedge(AntiFlatPoint other) {
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(this.e12345) * other.group0, 
            /* e1, e2, e3, e5 */
            float4((other.e315 * this.e412) + (other.e321 * this.e415), (other.e125 * this.e423) + (other.e321 * this.e425), (other.e235 * this.e431) + (other.e321 * this.e435), -(other.e315 * this.e425) - (other.e125 * this.e435)) - (other.group0.zxyx * float4(this.group0.yzx, this.e415))
        );
    }
}
public extension VersorEven {
    public AntiFlector anti_wedge(AntiFlector other) {
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(this.e12345) * other.group0, 
            /* e1, e2, e3, e5 */
            float4((other.e321 * this.e415) + (other.e1 * this.e12345), (other.e321 * this.e425) + (other.e2 * this.e12345), (other.e321 * this.e435) + (other.e3 * this.e12345), -(other.e315 * this.e425) - (other.e125 * this.e435)) + (this.group0.zxyw * float4(other.group0.yzx, other.e5)) - (other.group0.zxyx * float4(this.group0.yzx, this.e415))
        );
    }
}
public extension VersorEven {
    public AntiMotor anti_wedge(AntiLine other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((float3(this.e12345) * other.group0.xyz), -(other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412)), 
            /* e15, e25, e35, e3215 */
            float4(float3(1.0), 0.0) * float4(other.group1.xyz, 0.0) * float4(this.group0.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
public extension VersorEven {
    public AntiMotor anti_wedge(AntiMotor other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            (float4(other.e3215) * float4(this.group0.xyz, this.e4)) + (float4(this.e12345) * other.group0) + float4(float3(0.0), -(other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412)), 
            /* e15, e25, e35, e3215 */
            float4(((float3(other.e3215) * this.group1.xyz) + (float3(this.e12345) * other.group1.xyz)), other.e3215 * this.e12345)
        );
    }
}
public extension VersorEven {
    public AntiPlane anti_wedge(AntiPlane other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(this.e12345) * other.group0
        );
    }
}
public extension VersorEven {
    public VersorEven anti_wedge(AntiScalar other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(other.e12345) * this.group0, 
            /* e415, e425, e435, e321 */
            float4(other.e12345) * this.group1, 
            /* e235, e315, e125, e5 */
            float4(other.e12345) * this.group2, 
            /* e1, e2, e3, e4 */
            float4(other.e12345) * this.group3
        );
    }
}
public extension VersorEven {
    public AntiDipoleInversion anti_wedge(Circle other) {
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(this.e12345) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(this.e12345) * other.group1, 
            /* e235, e315, e125, e4 */
            float4((float3(this.e12345) * other.group2.xyz), -(other.e423 * this.e415) - (other.e431 * this.e425) - (other.e412 * this.e435) - (other.e415 * this.e423) - (other.e425 * this.e431) - (other.e435 * this.e412)), 
            /* e1, e2, e3, e5 */
            float4((other.e412 * this.e315) + (other.e415 * this.e321) + (other.e321 * this.e415) + (other.e315 * this.e412), (other.e423 * this.e125) + (other.e425 * this.e321) + (other.e321 * this.e425) + (other.e125 * this.e423), (other.e431 * this.e235) + (other.e435 * this.e321) + (other.e321 * this.e435) + (other.e235 * this.e431), -(other.e415 * this.e235) - (other.e425 * this.e315) - (other.e435 * this.e125) - (other.e125 * this.e435)) - float4((other.group0.xyz.yzx * this.group2.zxy), other.e235 * this.e415) - float4((other.group2.xyz.zxy * this.group0.yzx), other.e315 * this.e425)
        );
    }
}
public extension VersorEven {
    public VersorEven anti_wedge(CircleRotor other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(((float3(other.e12345) * this.group0.xyz) + (float3(this.e12345) * other.group0.xyz)), other.e12345 * this.e12345), 
            /* e415, e425, e435, e321 */
            (float4(other.e12345) * this.group1) + (float4(this.e12345) * other.group1), 
            /* e235, e315, e125, e5 */
            float4(other.e12345 * this.e235, other.e12345 * this.e315, other.e12345 * this.e125, -(other.e415 * this.e235) - (other.e425 * this.e315) - (other.e435 * this.e125) - (other.e235 * this.e415) - (other.e315 * this.e425) - (other.e125 * this.e435)) + (other.group2 * float4(this.group0.www, this.e5)), 
            /* e1, e2, e3, e4 */
            float4((other.e415 * this.e321) + (other.e321 * this.e415) + (other.e315 * this.e412) + (other.e12345 * this.e1), (other.e425 * this.e321) + (other.e321 * this.e425) + (other.e125 * this.e423) + (other.e12345 * this.e2), (other.e435 * this.e321) + (other.e321 * this.e435) + (other.e235 * this.e431) + (other.e12345 * this.e3), -(other.e412 * this.e435) - (other.e415 * this.e423) - (other.e425 * this.e431) - (other.e435 * this.e412)) + float4((other.group0.xyz.zxy * this.group2.yzx), other.e12345 * this.e4) - float4((other.group0.xyz.yzx * this.group2.zxy), other.e423 * this.e415) - float4((other.group2.zxy * this.group0.yzx), other.e431 * this.e425)
        );
    }
}
public extension VersorEven {
    public AntiCircleRotor anti_wedge(Dipole other) {
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(this.e12345) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(this.e12345) * other.group1, 
            /* e15, e25, e35, scalar */
            float4((float3(this.e12345) * other.group2.xyz), -(other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e45 * this.e321) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412))
        );
    }
}
public extension VersorEven {
    public VersorOdd anti_wedge(DipoleInversion other) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (other.group3.yzxy * float4(this.group0.zxy, this.e2)) + float4(float3(0.0), (other.e4125 * this.e3) + (other.e3215 * this.e4) - (other.e42 * this.e315) - (other.e43 * this.e125) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e45 * this.e321) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412)) + float4((other.group0.xyz * this.group0.www), other.e1234 * this.e5) + float4((other.group2.www * this.group1.xyz), other.e4235 * this.e1) - float4((other.group3.zxy * this.group0.yzx), other.e41 * this.e235), 
            /* e23, e31, e12, e45 */
            float4((other.e1234 * this.e235) + (other.e3215 * this.e423), (other.e1234 * this.e315) + (other.e3215 * this.e431), (other.e1234 * this.e125) + (other.e3215 * this.e412), -(other.e4315 * this.e425) - (other.e4125 * this.e435)) + (float4(this.e12345) * other.group1) - (other.group3.xyzx * this.group1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(other.e3215) * this.group1.xyz) + (float3(this.e12345) * other.group2.xyz) + (other.group3.zxy * this.group2.yzx) - (other.group3.yzx * this.group2.zxy)), other.e1234 * this.e12345), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * other.group3
        );
    }
}
public extension VersorEven {
    public VersorEven anti_wedge(DualNum other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(other.e12345) * this.group0, 
            /* e415, e425, e435, e321 */
            float4(other.e12345) * this.group1, 
            /* e235, e315, e125, e5 */
            float4(other.group0.xy.yy, other.e12345, (other.e5 * this.e12345) + (other.e12345 * this.e5)) * float4(this.group2.xyz, 1.0), 
            /* e1, e2, e3, e4 */
            float4(other.e12345) * this.group3
        );
    }
}
public extension VersorEven {
    public AntiCircleRotor anti_wedge(FlatPoint other) {
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), other.e45 * this.e12345), 
            /* e15, e25, e35, scalar */
            float4((float3(this.e12345) * other.group0.xyz), -(other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412) - (other.e45 * this.e321))
        );
    }
}
public extension VersorEven {
    public VersorOdd anti_wedge(Flector other) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (other.group1.yzxx * float4(this.group0.zxy, this.e1)) + float4(float3(0.0), (other.e4315 * this.e2) + (other.e4125 * this.e3) + (other.e3215 * this.e4) - (other.e25 * this.e431) - (other.e35 * this.e412) - (other.e45 * this.e321)) - (this.group0.yzxx * float4(other.group1.zxy, other.e15)), 
            /* e23, e31, e12, e45 */
            (this.group0 * float4(other.group1.www, other.e45)) + float4(float3(0.0), -(other.e4315 * this.e425) - (other.e4125 * this.e435)) - (other.group1.xyzx * this.group1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4((float3(other.e3215) * this.group1.xyz), 0.0) + float4((float3(this.e12345) * other.group0.xyz), 0.0) + float4((other.group1.zxy * this.group2.yzx), 0.0) - float4((other.group1.yzx * this.group2.zxy), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * other.group1
        );
    }
}
public extension VersorEven {
    public AntiDipoleInversion anti_wedge(Line other) {
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(1.0), 0.0) * float4(other.group0.xyz, 0.0) * float4(this.group0.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e235, e315, e125, e4 */
            float4((float3(this.e12345) * other.group1.xyz), -(other.e415 * this.e423) - (other.e425 * this.e431) - (other.e435 * this.e412)), 
            /* e1, e2, e3, e5 */
            float4((other.e415 * this.e321) + (other.e315 * this.e412), (other.e425 * this.e321) + (other.e125 * this.e423), (other.e435 * this.e321) + (other.e235 * this.e431), -(other.e425 * this.e315) - (other.e435 * this.e125) - (other.e235 * this.e415) - (other.e315 * this.e425) - (other.e125 * this.e435)) - float4((other.group1.xyz.zxy * this.group0.yzx), other.e415 * this.e235)
        );
    }
}
public extension VersorEven {
    public VersorEven anti_wedge(Motor other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(other.e12345) * this.group0, 
            /* e415, e425, e435, e321 */
            float4(((float3(other.e12345) * this.group1.xyz) + (float3(this.e12345) * other.group0.xyz)), other.e12345 * this.e321), 
            /* e235, e315, e125, e5 */
            (float4(other.e12345) * this.group2) + (float4(this.e12345) * other.group1) + float4(float3(0.0), -(other.e415 * this.e235) - (other.e425 * this.e315) - (other.e435 * this.e125) - (other.e235 * this.e415) - (other.e315 * this.e425) - (other.e125 * this.e435)), 
            /* e1, e2, e3, e4 */
            float4((other.e12345 * this.e1) + (other.e315 * this.e412), (other.e12345 * this.e2) + (other.e125 * this.e423), (other.e12345 * this.e3) + (other.e235 * this.e431), -(other.e425 * this.e431) - (other.e435 * this.e412)) + (other.group0 * float4(this.group1.www, this.e4)) - (this.group0.yzxx * float4(other.group1.zxy, other.e415))
        );
    }
}
public extension VersorEven {
    public MultiVector anti_wedge(MultiVector other) {
        return MultiVector(
            /* scalar, e12345 */
            float2((other.scalar * this.e12345) + (other.e4235 * this.e1) + (other.e4315 * this.e2) + (other.e4125 * this.e3) + (other.e3215 * this.e4) + (other.e1234 * this.e5) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412) - (other.e45 * this.e321) - (other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435), other.e12345 * this.e12345), 
            /* e1, e2, e3, e4 */
            float4((other.e1 * this.e12345) + (other.e415 * this.e321) + (other.e321 * this.e415) + (other.e315 * this.e412), (other.e2 * this.e12345) + (other.e425 * this.e321) + (other.e321 * this.e425) + (other.e125 * this.e423), (other.e3 * this.e12345) + (other.e435 * this.e321) + (other.e321 * this.e435) + (other.e235 * this.e431), -(other.e415 * this.e423) - (other.e425 * this.e431) - (other.e435 * this.e412) - (other.e412 * this.e435)) + (float4(other.e12345) * this.group3) + float4((other.group7.xyz.zxy * this.group2.yzx), other.e4 * this.e12345) - float4((other.group7.xyz.yzx * this.group2.zxy), other.e423 * this.e415) - float4((other.group8.xyz.zxy * this.group0.yzx), other.e431 * this.e425), 
            /* e5 */
            (other.e12345 * this.e5) + (other.e5 * this.e12345) - (other.e415 * this.e235) - (other.e425 * this.e315) - (other.e435 * this.e125) - (other.e235 * this.e415) - (other.e315 * this.e425) - (other.e125 * this.e435), 
            /* e15, e25, e35, e45 */
            float4((other.e4125 * this.e315) + (other.e3215 * this.e415), (other.e4235 * this.e125) + (other.e3215 * this.e425), (other.e4315 * this.e235) + (other.e3215 * this.e435), -(other.e4315 * this.e425) - (other.e4125 * this.e435)) + (float4(this.e12345) * other.group3) - (other.group9.yzxx * float4(this.group2.zxy, this.e415)), 
            /* e41, e42, e43 */
            (float3(other.e1234) * this.group1.xyz) + (float3(this.e12345) * other.group4.xyz) + (other.group9.yzx * this.group0.zxy) - (other.group9.zxy * this.group0.yzx), 
            /* e23, e31, e12 */
            (float3(other.e3215) * this.group0.xyz) + (float3(other.e1234) * this.group2.xyz) + (float3(this.e12345) * other.group5.xyz) - (float3(this.e321) * other.group9.xyz), 
            /* e415, e425, e435, e321 */
            (float4(other.e12345) * this.group1) + (float4(this.e12345) * other.group6), 
            /* e423, e431, e412 */
            (float3(other.e12345) * this.group0.xyz) + (float3(this.e12345) * other.group7.xyz), 
            /* e235, e315, e125 */
            (float3(other.e12345) * this.group2.xyz) + (float3(this.e12345) * other.group8.xyz), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * other.group9, 
            /* e1234 */
            other.e1234 * this.e12345
        );
    }
}
public extension VersorEven {
    public VersorOdd anti_wedge(Plane other) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(other.e4125 * this.e431 * -1.0, other.e4235 * this.e412 * -1.0, other.e4315 * this.e423 * -1.0, (other.e4315 * this.e2) + (other.e4125 * this.e3) + (other.e3215 * this.e4)) + (other.group0.yzxx * float4(this.group0.zxy, this.e1)), 
            /* e23, e31, e12, e45 */
            float4(other.e3215 * this.e423, other.e3215 * this.e431, other.e3215 * this.e412, -(other.e4315 * this.e425) - (other.e4125 * this.e435)) - (other.group0.xyzx * this.group1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4((float3(other.e3215) * this.group1.xyz), 0.0) + float4((other.group0.zxy * this.group2.yzx), 0.0) - float4((other.group0.yzx * this.group2.zxy), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * other.group0
        );
    }
}
public extension VersorEven {
    public RoundPoint anti_wedge(RoundPoint other) {
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(this.e12345) * other.group0, 
            /* e5 */
            other.e5 * this.e12345
        );
    }
}
public extension VersorEven {
    public Scalar anti_wedge(Scalar other) {
        return Scalar(
            /* scalar */
            other.scalar * this.e12345
        );
    }
}
public extension VersorEven {
    public VersorOdd anti_wedge(Sphere other) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(other.e4125 * this.e431 * -1.0, other.e4235 * this.e412 * -1.0, other.e4315 * this.e423 * -1.0, (other.e4125 * this.e3) + (other.e3215 * this.e4) + (other.e1234 * this.e5)) + (other.group0.yzxx * float4(this.group0.zxy, this.e1)) + float4((float3(other.e1234) * this.group1.xyz), other.e4315 * this.e2), 
            /* e23, e31, e12, e45 */
            float4((other.e3215 * this.e423) + (other.e1234 * this.e235), (other.e3215 * this.e431) + (other.e1234 * this.e315), (other.e3215 * this.e412) + (other.e1234 * this.e125), -(other.e4315 * this.e425) - (other.e4125 * this.e435)) - (other.group0.xyzx * this.group1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(other.e3215) * this.group1.xyz) + (other.group0.zxy * this.group2.yzx) - (other.group0.yzx * this.group2.zxy)), other.e1234 * this.e12345), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * other.group0
        );
    }
}
public extension VersorEven {
    public VersorEven anti_wedge(VersorEven other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(((float3(other.e12345) * this.group0.xyz) + (float3(this.e12345) * other.group0.xyz)), other.e12345 * this.e12345), 
            /* e415, e425, e435, e321 */
            (float4(other.e12345) * this.group1) + (float4(this.e12345) * other.group1), 
            /* e235, e315, e125, e5 */
            (float4(other.e12345) * this.group2) + (float4(this.e12345) * other.group2) + float4(float3(0.0), -(other.e415 * this.e235) - (other.e425 * this.e315) - (other.e435 * this.e125) - (other.e235 * this.e415) - (other.e315 * this.e425) - (other.e125 * this.e435)), 
            /* e1, e2, e3, e4 */
            float4((other.e415 * this.e321) + (other.e321 * this.e415) + (other.e315 * this.e412) + (other.e1 * this.e12345), (other.e425 * this.e321) + (other.e321 * this.e425) + (other.e125 * this.e423) + (other.e2 * this.e12345), (other.e435 * this.e321) + (other.e321 * this.e435) + (other.e235 * this.e431) + (other.e3 * this.e12345), -(other.e412 * this.e435) - (other.e415 * this.e423) - (other.e425 * this.e431) - (other.e435 * this.e412)) + (other.group0.zxyw * float4(this.group2.yzx, this.e4)) + float4((other.group0.www * this.group3.xyz), other.e4 * this.e12345) - (other.group0.yzxx * float4(this.group2.zxy, this.e415)) - float4((other.group2.zxy * this.group0.yzx), other.e431 * this.e425)
        );
    }
}
public extension VersorEven {
    public VersorOdd anti_wedge(VersorOdd other) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (this.group0.zxyw * float4(other.group3.yzx, other.scalar)) + float4(float3(0.0), (this.e2 * other.e4315) + (this.e3 * other.e4125) + (this.e4 * other.e3215) - (this.e431 * other.e25) - (this.e412 * other.e35) - (this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12) - (this.e321 * other.e45) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43)) + float4((this.group0.www * other.group0.xyz), this.e5 * other.e1234) + float4((other.group2.www * this.group1.xyz), this.e1 * other.e4235) - (this.group0.yzxx * float4(other.group3.zxy, other.e15)), 
            /* e23, e31, e12, e45 */
            float4((this.e12345 * other.e23) + (this.e235 * other.e1234), (this.e12345 * other.e31) + (this.e315 * other.e1234), (this.e12345 * other.e12) + (this.e125 * other.e1234), -(this.e425 * other.e4315) - (this.e435 * other.e4125)) + (this.group0 * float4(other.group3.www, other.e45)) - (this.group1.wwwx * other.group3.xyzx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(this.e12345) * other.group2.xyz) + (float3(other.e3215) * this.group1.xyz) + (this.group2.yzx * other.group3.zxy) - (this.group2.zxy * other.group3.yzx)), this.e12345 * other.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * other.group3
        );
    }
}
public extension VersorOdd {
    // Fancy infix trick (first half)
    public AntiWedgeInfixPartial<VersorOdd> operator /(AntiWedgeInfix rhs) {
        return AntiWedgeInfixPartial<VersorOdd>(this);
    }
}
public extension AntiWedgeInfixPartial<VersorOdd> {
    // Fancy infix trick (second half)
    public RoundPoint operator /(AntiCircleRotor rhs) {
        return this.a.anti_wedge(rhs);
    }
}
public extension VersorOdd {
    public RoundPoint anti_wedge(AntiCircleRotor other) {
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(-(other.e41 * this.e3215) - (other.e31 * this.e4125), -(other.e42 * this.e3215) - (other.e12 * this.e4235), -(other.e43 * this.e3215) - (other.e23 * this.e4315), (other.e43 * this.e4125) + (other.e45 * this.e1234)) + (this.group3.yzxx * float4(other.group1.zxy, other.e41)) + float4((this.group2.www * other.group2.xyz), other.e42 * this.e4315), 
            /* e5 */
            -(other.e45 * this.e3215) - (other.e15 * this.e4235) - (other.e25 * this.e4315) - (other.e35 * this.e4125)
        );
    }
}
public extension VersorOdd {
    public AntiCircleRotor anti_wedge(AntiDipoleInversion other) {
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (float3(this.e1234) * other.group1.xyz) + (other.group0.xyz.zxy * this.group3.yzx) - (other.group0.xyz.yzx * this.group3.zxy), 
            /* e23, e31, e12, e45 */
            float4((other.e423 * this.e3215) + (other.e235 * this.e1234), (other.e431 * this.e3215) + (other.e315 * this.e1234), (other.e412 * this.e3215) + (other.e125 * this.e1234), -(other.e425 * this.e4315) - (other.e435 * this.e4125)) - (other.group1.wwwx * this.group3.xyzx), 
            /* e15, e25, e35, scalar */
            (float4(this.e3215) * float4(other.group1.xyz, other.e4)) + (this.group3.zxyx * float4(other.group2.yzx, other.e1)) + float4(float3(0.0), (other.e2 * this.e4315) + (other.e3 * this.e4125) + (other.e5 * this.e1234) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e321 * this.e45) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43)) - float4((other.group2.zxy * this.group3.yzx), other.e423 * this.e15)
        );
    }
}
public extension VersorOdd {
    public AntiFlector anti_wedge(AntiDualNum other) {
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(other.e3215) * float4(this.group3.xyz, this.e1234), 
            /* e1, e2, e3, e5 */
            float4(other.e3215) * float4(this.group0.xyz, this.e45)
        );
    }
}
public extension VersorOdd {
    public AntiMotor anti_wedge(AntiFlatPoint other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(other.e235 * this.e1234, other.e315 * this.e1234, other.e125 * this.e1234, -(other.e315 * this.e42) - (other.e125 * this.e43) - (other.e321 * this.e45)) - (other.group0.wwwx * float4(this.group3.xyz, this.e41)), 
            /* e15, e25, e35, e3215 */
            float4(((other.group0.yzx * this.group3.zxy) - (other.group0.zxy * this.group3.yzx)), 0.0)
        );
    }
}
public extension VersorOdd {
    public AntiMotor anti_wedge(AntiFlector other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(float3(0.0), (other.e2 * this.e4315) + (other.e3 * this.e4125) + (other.e5 * this.e1234) - (other.e315 * this.e42) - (other.e125 * this.e43) - (other.e321 * this.e45)) + float4((this.group2.www * other.group0.xyz), other.e1 * this.e4235) - (other.group0.wwwx * float4(this.group3.xyz, this.e41)), 
            /* e15, e25, e35, e3215 */
            float4(((other.group0.yzx * this.group3.zxy) - (other.group0.zxy * this.group3.yzx)), 0.0)
        );
    }
}
public extension VersorOdd {
    public AntiPlane anti_wedge(AntiLine other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4((other.e12 * this.e4315) + (other.e15 * this.e1234), (other.e23 * this.e4125) + (other.e25 * this.e1234), (other.e31 * this.e4235) + (other.e35 * this.e1234), -(other.e25 * this.e4315) - (other.e35 * this.e4125)) - (this.group3.zxyx * float4(other.group0.xyz.yzx, other.e15))
        );
    }
}
public extension VersorOdd {
    public AntiFlector anti_wedge(AntiMotor other) {
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(other.e3215) * float4(this.group3.xyz, this.e1234), 
            /* e1, e2, e3, e5 */
            float4((other.e15 * this.e1234) + (other.e3215 * this.e41), (other.e25 * this.e1234) + (other.e3215 * this.e42), (other.e35 * this.e1234) + (other.e3215 * this.e43), -(other.e25 * this.e4315) - (other.e35 * this.e4125)) + float4((other.group0.zxy * this.group3.yzx), other.e3215 * this.e45) - (this.group3.zxyx * float4(other.group0.yzx, other.e15))
        );
    }
}
public extension VersorOdd {
    public Scalar anti_wedge(AntiPlane other) {
        return Scalar(
            /* scalar */
            (other.e1 * this.e4235) + (other.e2 * this.e4315) + (other.e3 * this.e4125) + (other.e5 * this.e1234)
        );
    }
}
public extension VersorOdd {
    public VersorOdd anti_wedge(AntiScalar other) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(other.e12345) * this.group0, 
            /* e23, e31, e12, e45 */
            float4(other.e12345) * this.group1, 
            /* e15, e25, e35, e1234 */
            float4(other.e12345) * this.group2, 
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345) * this.group3
        );
    }
}
public extension VersorOdd {
    public AntiCircleRotor anti_wedge(Circle other) {
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (float3(this.e1234) * other.group1.xyz) + (other.group0.xyz.zxy * this.group3.yzx) - (other.group0.xyz.yzx * this.group3.zxy), 
            /* e23, e31, e12, e45 */
            float4((other.e423 * this.e3215) + (other.e235 * this.e1234), (other.e431 * this.e3215) + (other.e315 * this.e1234), (other.e412 * this.e3215) + (other.e125 * this.e1234), -(other.e425 * this.e4315) - (other.e435 * this.e4125)) - (other.group1.wwwx * this.group3.xyzx), 
            /* e15, e25, e35, scalar */
            float4((other.e415 * this.e3215) + (other.e315 * this.e4125), (other.e425 * this.e3215) + (other.e125 * this.e4235), (other.e435 * this.e3215) + (other.e235 * this.e4315), -(other.e431 * this.e25) - (other.e412 * this.e35) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e321 * this.e45) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43)) - float4((other.group2.xyz.zxy * this.group3.yzx), other.e423 * this.e15)
        );
    }
}
public extension VersorOdd {
    public VersorOdd anti_wedge(CircleRotor other) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4((other.e415 * this.e1234) + (other.e12345 * this.e41), (other.e425 * this.e1234) + (other.e12345 * this.e42), (other.e435 * this.e1234) + (other.e12345 * this.e43), -(other.e431 * this.e25) - (other.e412 * this.e35) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e321 * this.e45) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43)) + float4((other.group0.xyz.zxy * this.group3.yzx), other.e12345 * this.scalar) - float4((other.group0.xyz.yzx * this.group3.zxy), other.e423 * this.e15), 
            /* e23, e31, e12, e45 */
            float4((other.e235 * this.e1234) + (other.e12345 * this.e23), (other.e315 * this.e1234) + (other.e12345 * this.e31), (other.e125 * this.e1234) + (other.e12345 * this.e12), -(other.e425 * this.e4315) - (other.e435 * this.e4125)) + float4((other.group0.xyz * this.group3.www), other.e12345 * this.e45) - (other.group1.wwwx * this.group3.xyzx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(other.e12345) * this.group2.xyz) + (float3(this.e3215) * other.group1.xyz) + (other.group2.yzx * this.group3.zxy) - (other.group2.zxy * this.group3.yzx)), other.e12345 * this.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345) * this.group3
        );
    }
}
public extension VersorOdd {
    public RoundPoint anti_wedge(Dipole other) {
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(-(other.e41 * this.e3215) - (other.e31 * this.e4125), -(other.e42 * this.e3215) - (other.e12 * this.e4235), -(other.e43 * this.e3215) - (other.e23 * this.e4315), (other.e43 * this.e4125) + (other.e45 * this.e1234)) + (this.group3.yzxy * float4(other.group1.zxy, other.e42)) + float4((other.group2.xyz * this.group2.www), other.e41 * this.e4235), 
            /* e5 */
            -(other.e45 * this.e3215) - (other.e15 * this.e4235) - (other.e25 * this.e4315) - (other.e35 * this.e4125)
        );
    }
}
public extension VersorOdd {
    public AntiDipoleInversion anti_wedge(DipoleInversion other) {
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (float3(this.e1234) * other.group3.xyz) - (float3(other.e1234) * this.group3.xyz), 
            /* e415, e425, e435, e321 */
            (other.group3.yzxw * float4(this.group3.zxy, this.e1234)) - (this.group3.yzxw * float4(other.group3.zxy, other.e1234)), 
            /* e235, e315, e125, e4 */
            (this.group3.xyzx * float4(other.group3.www, other.e41)) + float4(float3(0.0), (other.e42 * this.e4315) + (other.e43 * this.e4125) + (other.e45 * this.e1234) - (other.e4235 * this.e41) - (other.e4315 * this.e42) - (other.e4125 * this.e43)) - float4((this.group3.www * other.group3.xyz), other.e1234 * this.e45), 
            /* e1, e2, e3, e5 */
            (float4(other.e3215) * float4(this.group0.xyz, this.e45)) + (other.group3.zxyz * float4(this.group1.yzx, this.e35)) + (this.group2.wwwy * float4(other.group2.xyz, other.e4315)) + float4((other.group1.zxy * this.group3.yzx), other.e4235 * this.e15) - (float4(this.e3215) * float4(other.group0.xyz, other.e45)) - (other.group2.wwwy * float4(this.group2.xyz, this.e4315)) - (this.group3.zxyx * float4(other.group1.yzx, other.e15)) - float4((other.group3.yzx * this.group1.zxy), other.e35 * this.e4125)
        );
    }
}
public extension VersorOdd {
    public VersorOdd anti_wedge(DualNum other) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(other.group0.xy.yy, other.e12345, (other.e5 * this.e1234) + (other.e12345 * this.scalar)) * float4(this.group0.xyz, 1.0), 
            /* e23, e31, e12, e45 */
            float4(other.e12345) * this.group1, 
            /* e15, e25, e35, e1234 */
            float4(other.e12345) * this.group2, 
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345) * this.group3
        );
    }
}
public extension VersorOdd {
    public RoundPoint anti_wedge(FlatPoint other) {
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(this.e1234) * other.group0, 
            /* e5 */
            -(other.e15 * this.e4235) - (other.e25 * this.e4315) - (other.e35 * this.e4125) - (other.e45 * this.e3215)
        );
    }
}
public extension VersorOdd {
    public AntiDipoleInversion anti_wedge(Flector other) {
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(this.e1234) * other.group1.xyz, 
            /* e415, e425, e435, e321 */
            float4(((other.group1.yzx * this.group3.zxy) - (other.group1.zxy * this.group3.yzx)), other.e3215 * this.e1234), 
            /* e235, e315, e125, e4 */
            float4(float3(0.0), -(other.e4315 * this.e42) - (other.e4125 * this.e43)) + float4((other.group1.www * this.group3.xyz), other.e45 * this.e1234) - (other.group1.xyzx * float4(this.group3.www, this.e41)), 
            /* e1, e2, e3, e5 */
            (other.group1.zxyy * float4(this.group1.yzx, this.e25)) + (other.group1.wwwz * float4(this.group0.xyz, this.e35)) + (this.group2.wwwx * float4(other.group0.xyz, other.e4235)) + float4(float3(0.0), (other.e3215 * this.e45) - (other.e25 * this.e4315) - (other.e35 * this.e4125) - (other.e45 * this.e3215)) - float4((other.group1.yzx * this.group1.zxy), other.e15 * this.e4235)
        );
    }
}
public extension VersorOdd {
    public AntiCircleRotor anti_wedge(Line other) {
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(this.e1234) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4((float3(this.e1234) * other.group1.xyz), -(other.e415 * this.e4235) - (other.e425 * this.e4315) - (other.e435 * this.e4125)), 
            /* e15, e25, e35, scalar */
            float4((other.e415 * this.e3215) + (other.e315 * this.e4125), (other.e425 * this.e3215) + (other.e125 * this.e4235), (other.e435 * this.e3215) + (other.e235 * this.e4315), -(other.e425 * this.e31) - (other.e435 * this.e12) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43)) - float4((other.group1.xyz.zxy * this.group3.yzx), other.e415 * this.e23)
        );
    }
}
public extension VersorOdd {
    public VersorOdd anti_wedge(Motor other) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (other.group0 * float4(this.group2.www, this.scalar)) + float4(float3(0.0), -(other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43)) + float4((other.group0.www * this.group0.xyz), other.e5 * this.e1234), 
            /* e23, e31, e12, e45 */
            float4(other.e235 * this.e1234, other.e315 * this.e1234, other.e125 * this.e1234, -(other.e415 * this.e4235) - (other.e425 * this.e4315) - (other.e435 * this.e4125)) + (float4(other.e12345) * this.group1), 
            /* e15, e25, e35, e1234 */
            float4(((float3(other.e12345) * this.group2.xyz) + (float3(this.e3215) * other.group0.xyz) + (other.group1.yzx * this.group3.zxy) - (other.group1.zxy * this.group3.yzx)), other.e12345 * this.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345) * this.group3
        );
    }
}
public extension VersorOdd {
    public MultiVector anti_wedge(MultiVector other) {
        return MultiVector(
            /* scalar, e12345 */
            float2((other.e12345 * this.scalar) + (other.e1 * this.e4235) + (other.e2 * this.e4315) + (other.e3 * this.e4125) + (other.e4 * this.e3215) + (other.e5 * this.e1234) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e321 * this.e45) - (other.e423 * this.e15) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43), 0.0), 
            /* e1, e2, e3, e4 */
            (this.group3.yzxx * float4(other.group5.xyz.zxy, other.e41)) + float4((other.group9.zxy * this.group1.yzx), other.e43 * this.e4125) + float4((other.group9.www * this.group0.xyz), other.e45 * this.e1234) + float4((this.group2.www * other.group3.xyz), other.e42 * this.e4315) - (float4(other.e1234) * float4(this.group2.xyz, this.e45)) - (other.group9.yzxz * float4(this.group1.zxy, this.e43)) - float4((other.group4.xyz * this.group3.www), other.e4235 * this.e41) - float4((other.group5.xyz.yzx * this.group3.zxy), other.e4315 * this.e42), 
            /* e5 */
            (other.e4235 * this.e15) + (other.e4315 * this.e25) + (other.e4125 * this.e35) + (other.e3215 * this.e45) - (other.e15 * this.e4235) - (other.e25 * this.e4315) - (other.e35 * this.e4125) - (other.e45 * this.e3215), 
            /* e15, e25, e35, e45 */
            float4((other.e415 * this.e3215) + (other.e315 * this.e4125), (other.e425 * this.e3215) + (other.e125 * this.e4235), (other.e435 * this.e3215) + (other.e235 * this.e4315), -(other.e425 * this.e4315) - (other.e435 * this.e4125)) + (float4(other.e12345) * float4(this.group2.xyz, this.e45)) - (this.group3.yzxx * float4(other.group8.xyz.zxy, other.e415)), 
            /* e41, e42, e43 */
            (float3(other.e12345) * this.group0.xyz) + (float3(this.e1234) * other.group6.xyz) + (other.group7.xyz.zxy * this.group3.yzx) - (other.group7.xyz.yzx * this.group3.zxy), 
            /* e23, e31, e12 */
            (float3(other.e12345) * this.group1.xyz) + (float3(this.e1234) * other.group8.xyz) + (float3(this.e3215) * other.group7.xyz) - (float3(other.e321) * this.group3.xyz), 
            /* e415, e425, e435, e321 */
            (other.group9.yzxw * float4(this.group3.zxy, this.e1234)) - float4((other.group9.zxy * this.group3.yzx), other.e1234 * this.e3215), 
            /* e423, e431, e412 */
            (float3(this.e1234) * other.group9.xyz) - (float3(other.e1234) * this.group3.xyz), 
            /* e235, e315, e125 */
            (float3(other.e3215) * this.group3.xyz) - (float3(this.e3215) * other.group9.xyz), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345) * this.group3, 
            /* e1234 */
            other.e12345 * this.e1234
        );
    }
}
public extension VersorOdd {
    public AntiDipoleInversion anti_wedge(Plane other) {
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(this.e1234) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(((other.group0.yzx * this.group3.zxy) - (other.group0.zxy * this.group3.yzx)), other.e3215 * this.e1234), 
            /* e235, e315, e125, e4 */
            float4(other.e3215 * this.e4235, other.e3215 * this.e4315, other.e3215 * this.e4125, -(other.e4315 * this.e42) - (other.e4125 * this.e43)) - (other.group0.xyzx * float4(this.group3.www, this.e41)), 
            /* e1, e2, e3, e5 */
            float4(other.e4315 * this.e12 * -1.0, other.e4125 * this.e23 * -1.0, other.e4235 * this.e31 * -1.0, (other.e4125 * this.e35) + (other.e3215 * this.e45)) + (other.group0.zxyx * float4(this.group1.yzx, this.e15)) + (other.group0.wwwy * float4(this.group0.xyz, this.e25))
        );
    }
}
public extension VersorOdd {
    public Scalar anti_wedge(RoundPoint other) {
        return Scalar(
            /* scalar */
            (other.e1 * this.e4235) + (other.e2 * this.e4315) + (other.e3 * this.e4125) + (other.e4 * this.e3215) + (other.e5 * this.e1234)
        );
    }
}
public extension VersorOdd {
    public AntiDipoleInversion anti_wedge(Sphere other) {
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (float3(this.e1234) * other.group0.xyz) - (float3(other.e1234) * this.group3.xyz), 
            /* e415, e425, e435, e321 */
            (other.group0.yzxw * float4(this.group3.zxy, this.e1234)) - float4((other.group0.zxy * this.group3.yzx), other.e1234 * this.e3215), 
            /* e235, e315, e125, e4 */
            float4(other.e3215 * this.e4235, other.e3215 * this.e4315, other.e3215 * this.e4125, -(other.e4315 * this.e42) - (other.e4125 * this.e43) - (other.e1234 * this.e45)) - (other.group0.xyzx * float4(this.group3.www, this.e41)), 
            /* e1, e2, e3, e5 */
            float4(-(other.e4315 * this.e12) - (other.e1234 * this.e15), -(other.e4125 * this.e23) - (other.e1234 * this.e25), -(other.e4235 * this.e31) - (other.e1234 * this.e35), (other.e4125 * this.e35) + (other.e3215 * this.e45)) + (other.group0.zxyx * float4(this.group1.yzx, this.e15)) + (other.group0.wwwy * float4(this.group0.xyz, this.e25))
        );
    }
}
public extension VersorOdd {
    public VersorOdd anti_wedge(VersorEven other) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (other.group0.zxyw * float4(this.group3.yzx, this.scalar)) + float4(float3(0.0), (other.e2 * this.e4315) + (other.e3 * this.e4125) + (other.e4 * this.e3215) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e321 * this.e45) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43)) + float4((other.group0.www * this.group0.xyz), other.e5 * this.e1234) + float4((this.group2.www * other.group1.xyz), other.e1 * this.e4235) - (other.group0.yzxx * float4(this.group3.zxy, this.e15)), 
            /* e23, e31, e12, e45 */
            float4((other.e12345 * this.e23) + (other.e235 * this.e1234), (other.e12345 * this.e31) + (other.e315 * this.e1234), (other.e12345 * this.e12) + (other.e125 * this.e1234), -(other.e425 * this.e4315) - (other.e435 * this.e4125)) + (other.group0 * float4(this.group3.www, this.e45)) - (other.group1.wwwx * this.group3.xyzx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(other.e12345) * this.group2.xyz) + (float3(this.e3215) * other.group1.xyz) + (other.group2.yzx * this.group3.zxy) - (other.group2.zxy * this.group3.yzx)), other.e12345 * this.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345) * this.group3
        );
    }
}
public extension VersorOdd {
    public AntiDipoleInversion anti_wedge(VersorOdd other) {
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (float3(this.e1234) * other.group3.xyz) - (float3(other.e1234) * this.group3.xyz), 
            /* e415, e425, e435, e321 */
            (other.group3.yzxw * float4(this.group3.zxy, this.e1234)) - (this.group3.yzxw * float4(other.group3.zxy, other.e1234)), 
            /* e235, e315, e125, e4 */
            (this.group3.xyzx * float4(other.group3.www, other.e41)) + float4(float3(0.0), (other.e42 * this.e4315) + (other.e43 * this.e4125) + (other.e45 * this.e1234) - (other.e4235 * this.e41) - (other.e4315 * this.e42) - (other.e4125 * this.e43)) - float4((this.group3.www * other.group3.xyz), other.e1234 * this.e45), 
            /* e1, e2, e3, e5 */
            (float4(other.e3215) * float4(this.group0.xyz, this.e45)) + (other.group3.zxyz * float4(this.group1.yzx, this.e35)) + (this.group2.wwwy * float4(other.group2.xyz, other.e4315)) + float4((other.group1.zxy * this.group3.yzx), other.e4235 * this.e15) - (float4(this.e3215) * float4(other.group0.xyz, other.e45)) - (other.group2.wwwy * float4(this.group2.xyz, this.e4315)) - (this.group3.zxyx * float4(other.group1.yzx, other.e15)) - float4((other.group3.yzx * this.group1.zxy), other.e35 * this.e4125)
        );
    }
}

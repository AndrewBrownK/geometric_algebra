implementing cga3d;
using traits;
public extension AntiCircleRotor {
    // Fancy infix trick (first half)
    public WedgeInfixPartial<AntiCircleRotor> operator /(WedgeInfix rhs) {
        return WedgeInfixPartial<AntiCircleRotor>(this);
    }
}
public extension WedgeInfixPartial<AntiCircleRotor> {
    // Fancy infix trick (second half)
    public VersorOdd operator /(AntiCircleRotor rhs) {
        return this.a.wedge(rhs);
    }
}
public extension AntiCircleRotor {
    public VersorOdd wedge(AntiCircleRotor other) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(((float3(other.scalar) * this.group0.xyz) + (float3(this.scalar) * other.group0.xyz)), other.scalar * this.scalar), 
            /* e23, e31, e12, e45 */
            (float4(other.scalar) * this.group1) + (float4(this.scalar) * other.group1), 
            /* e15, e25, e35, e1234 */
            float4((other.e15 * this.scalar) + (other.scalar * this.e15), (other.e25 * this.scalar) + (other.scalar * this.e25), (other.e35 * this.scalar) + (other.scalar * this.e35), -(other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43)), 
            /* e4235, e4315, e4125, e3215 */
            float4((other.e42 * this.e35) + (other.e23 * this.e45) + (other.e45 * this.e23) + (other.e35 * this.e42), (other.e43 * this.e15) + (other.e31 * this.e45) + (other.e45 * this.e31) + (other.e15 * this.e43), (other.e41 * this.e25) + (other.e12 * this.e45) + (other.e45 * this.e12) + (other.e25 * this.e41), -(other.e12 * this.e35) - (other.e15 * this.e23) - (other.e25 * this.e31) - (other.e35 * this.e12)) - (this.group2.yzxx * float4(other.group0.xyz.zxy, other.e23)) - float4((this.group0.xyz.zxy * other.group2.yzx), other.e31 * this.e25)
        );
    }
}
public extension AntiCircleRotor {
    public VersorEven wedge(AntiDipoleInversion other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4((this.e42 * other.e3) + (this.e23 * other.e4) + (this.scalar * other.e423), (this.e43 * other.e1) + (this.e31 * other.e4) + (this.scalar * other.e431), (this.e41 * other.e2) + (this.e12 * other.e4) + (this.scalar * other.e412), -(this.e42 * other.e315) - (this.e43 * other.e125) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435) - (this.e45 * other.e321) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412)) - float4((this.group0.xyz.zxy * other.group3.yzx), this.e41 * other.e235), 
            /* e415, e425, e435, e321 */
            float4((this.e15 * other.e4) + (this.scalar * other.e415), (this.e25 * other.e4) + (this.scalar * other.e425), (this.e35 * other.e4) + (this.scalar * other.e435), -(this.e31 * other.e2) - (this.e12 * other.e3)) + float4((this.group0.xyz * other.group3.www), this.scalar * other.e321) - (this.group1.wwwx * other.group3.xyzx), 
            /* e235, e315, e125, e5 */
            float4(((float3(this.scalar) * other.group2.xyz) + (float3(other.e5) * this.group1.xyz) + (this.group2.zxy * other.group3.yzx) - (this.group2.yzx * other.group3.zxy)), this.scalar * other.e5), 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * float4(other.group3.xyz, other.e4)
        );
    }
}
public extension AntiCircleRotor {
    public VersorOdd wedge(AntiDualNum other) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(other.scalar) * float4(this.group0.xyz, this.scalar), 
            /* e23, e31, e12, e45 */
            float4(other.scalar) * this.group1, 
            /* e15, e25, e35, e1234 */
            float4(other.group0.xy.yy, other.scalar, 0.0) * float4(float3(1.0), 0.0) * float4(this.group2.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), this.scalar * other.e3215)
        );
    }
}
public extension AntiCircleRotor {
    public CircleRotor wedge(AntiFlatPoint other) {
        return CircleRotor(
            /* e423, e431, e412 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), this.scalar * other.e321), 
            /* e235, e315, e125, e12345 */
            float4((this.group2.www * other.group0.xyz), -(this.e41 * other.e235) - (this.e42 * other.e315) - (this.e43 * other.e125) - (this.e45 * other.e321))
        );
    }
}
public extension AntiCircleRotor {
    public VersorEven wedge(AntiFlector other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(this.e42 * other.e3, this.e43 * other.e1, this.e41 * other.e2, -(this.e42 * other.e315) - (this.e43 * other.e125) - (this.e45 * other.e321)) - float4((this.group0.xyz.zxy * other.group1.yzx), this.e41 * other.e235), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), -(this.e31 * other.e2) - (this.e12 * other.e3)) + float4((this.group0.xyz * other.group1.www), this.scalar * other.e321) - (this.group1.wwwx * other.group1.xyzx), 
            /* e235, e315, e125, e5 */
            float4(((float3(this.scalar) * other.group0.xyz) + (float3(other.e5) * this.group1.xyz) + (this.group2.zxy * other.group1.yzx) - (this.group2.yzx * other.group1.zxy)), this.scalar * other.e5), 
            /* e1, e2, e3, e4 */
            float4(float3(1.0), 0.0) * float4(this.group2.www, 0.0) * float4(other.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
public extension AntiCircleRotor {
    public DipoleInversion wedge(AntiLine other) {
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12, e45 */
            float4(float3(1.0), 0.0) * float4(other.group0.xyz, 0.0) * float4(this.group2.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e15, e25, e35, e1234 */
            float4((float3(this.scalar) * other.group1.xyz), -(this.e41 * other.e23) - (this.e42 * other.e31) - (this.e43 * other.e12)), 
            /* e4235, e4315, e4125, e3215 */
            float4((this.e42 * other.e35) + (this.e45 * other.e23), (this.e43 * other.e15) + (this.e45 * other.e31), (this.e41 * other.e25) + (this.e45 * other.e12), -(this.e23 * other.e15) - (this.e31 * other.e25) - (this.e12 * other.e35) - (this.e25 * other.e31) - (this.e35 * other.e12)) - float4((this.group0.xyz.zxy * other.group1.xyz.yzx), this.e15 * other.e23)
        );
    }
}
public extension AntiCircleRotor {
    public VersorOdd wedge(AntiMotor other) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(other.scalar) * float4(this.group0.xyz, this.scalar), 
            /* e23, e31, e12, e45 */
            float4(((float3(this.scalar) * other.group0.xyz) + (float3(other.scalar) * this.group1.xyz)), this.e45 * other.scalar), 
            /* e15, e25, e35, e1234 */
            float4((this.e15 * other.scalar) + (this.scalar * other.e15), (this.e25 * other.scalar) + (this.scalar * other.e25), (this.e35 * other.scalar) + (this.scalar * other.e35), -(this.e41 * other.e23) - (this.e42 * other.e31) - (this.e43 * other.e12)), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e45 * other.e23, this.e45 * other.e31, this.e45 * other.e12, -(this.e31 * other.e25) - (this.e12 * other.e35) - (this.e15 * other.e23) - (this.e25 * other.e31) - (this.e35 * other.e12)) + (other.group1.zxyw * float4(this.group0.xyz.yzx, this.scalar)) - (other.group1.yzxx * float4(this.group0.xyz.zxy, this.e23))
        );
    }
}
public extension AntiCircleRotor {
    public AntiDipoleInversion wedge(AntiPlane other) {
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (this.group0.xyz.yzx * other.group0.zxy) - (this.group0.xyz.zxy * other.group0.yzx), 
            /* e415, e425, e435, e321 */
            float4(this.e41 * other.e5, this.e42 * other.e5, this.e43 * other.e5, -(this.e31 * other.e2) - (this.e12 * other.e3)) - (this.group1.wwwx * other.group0.xyzx), 
            /* e235, e315, e125, e4 */
            float4((float3(other.e5) * this.group1.xyz), 0.0) + float4((this.group2.zxy * other.group0.yzx), 0.0) - float4((this.group2.yzx * other.group0.zxy), 0.0), 
            /* e1, e2, e3, e5 */
            float4(this.scalar) * other.group0
        );
    }
}
public extension AntiCircleRotor {
    public AntiScalar wedge(AntiScalar other) {
        return AntiScalar(
            /* e12345 */
            this.scalar * other.e12345
        );
    }
}
public extension AntiCircleRotor {
    public CircleRotor wedge(Circle other) {
        return CircleRotor(
            /* e423, e431, e412 */
            float3(this.scalar) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(this.scalar) * other.group1, 
            /* e235, e315, e125, e12345 */
            float4((float3(this.scalar) * other.group2.xyz), -(this.e41 * other.e235) - (this.e42 * other.e315) - (this.e43 * other.e125) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435) - (this.e45 * other.e321) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412))
        );
    }
}
public extension AntiCircleRotor {
    public CircleRotor wedge(CircleRotor other) {
        return CircleRotor(
            /* e423, e431, e412 */
            float3(this.scalar) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(this.scalar) * other.group1, 
            /* e235, e315, e125, e12345 */
            float4((this.group2.www * other.group2.xyz), (this.scalar * other.e12345) - (this.e41 * other.e235) - (this.e42 * other.e315) - (this.e43 * other.e125) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435) - (this.e45 * other.e321) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412))
        );
    }
}
public extension AntiCircleRotor {
    public DipoleInversion wedge(Dipole other) {
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(this.scalar) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(this.scalar) * other.group1, 
            /* e15, e25, e35, e1234 */
            float4((float3(this.scalar) * other.group2.xyz), -(this.e41 * other.e23) - (this.e42 * other.e31) - (this.e43 * other.e12) - (this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43)), 
            /* e4235, e4315, e4125, e3215 */
            float4((this.e42 * other.e35) + (this.e23 * other.e45) + (this.e45 * other.e23) + (this.e35 * other.e42), (this.e43 * other.e15) + (this.e31 * other.e45) + (this.e45 * other.e31) + (this.e15 * other.e43), (this.e41 * other.e25) + (this.e12 * other.e45) + (this.e45 * other.e12) + (this.e25 * other.e41), -(this.e12 * other.e35) - (this.e15 * other.e23) - (this.e25 * other.e31) - (this.e35 * other.e12)) - float4((this.group0.xyz.zxy * other.group2.xyz.yzx), this.e23 * other.e15) - float4((other.group0.xyz.zxy * this.group2.yzx), this.e31 * other.e25)
        );
    }
}
public extension AntiCircleRotor {
    public DipoleInversion wedge(DipoleInversion other) {
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(this.scalar) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(this.scalar) * other.group1, 
            /* e15, e25, e35, e1234 */
            float4((this.group2.www * other.group2.xyz), (this.scalar * other.e1234) - (this.e41 * other.e23) - (this.e42 * other.e31) - (this.e43 * other.e12) - (this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43)), 
            /* e4235, e4315, e4125, e3215 */
            float4((this.e23 * other.e45) + (this.e45 * other.e23) + (this.e35 * other.e42) + (this.scalar * other.e4235), (this.e31 * other.e45) + (this.e45 * other.e31) + (this.e15 * other.e43) + (this.scalar * other.e4315), (this.e12 * other.e45) + (this.e45 * other.e12) + (this.e25 * other.e41) + (this.scalar * other.e4125), -(this.e12 * other.e35) - (this.e15 * other.e23) - (this.e25 * other.e31) - (this.e35 * other.e12)) + float4((this.group0.xyz.yzx * other.group2.zxy), this.scalar * other.e3215) - (other.group2.yzxx * float4(this.group0.xyz.zxy, this.e23)) - float4((other.group0.xyz.zxy * this.group2.yzx), this.e31 * other.e25)
        );
    }
}
public extension AntiCircleRotor {
    public Motor wedge(DualNum other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(other.group0.xy.xx, other.e5, other.e12345) * float4(this.group0.xyz, this.scalar), 
            /* e235, e315, e125, e5 */
            float4(other.e5) * float4(this.group1.xyz, this.scalar)
        );
    }
}
public extension AntiCircleRotor {
    public Flector wedge(FlatPoint other) {
        return Flector(
            /* e15, e25, e35, e45 */
            float4(this.scalar) * other.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4((this.e42 * other.e35) + (this.e23 * other.e45), (this.e43 * other.e15) + (this.e31 * other.e45), (this.e41 * other.e25) + (this.e12 * other.e45), -(this.e31 * other.e25) - (this.e12 * other.e35)) - (other.group0.yzxx * float4(this.group0.xyz.zxy, this.e23))
        );
    }
}
public extension AntiCircleRotor {
    public Flector wedge(Flector other) {
        return Flector(
            /* e15, e25, e35, e45 */
            float4(this.scalar) * other.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4((this.e23 * other.e45) + (this.scalar * other.e4235), (this.e31 * other.e45) + (this.scalar * other.e4315), (this.e12 * other.e45) + (this.scalar * other.e4125), -(this.e31 * other.e25) - (this.e12 * other.e35)) + float4((this.group0.xyz.yzx * other.group0.zxy), this.scalar * other.e3215) - (other.group0.yzxx * float4(this.group0.xyz.zxy, this.e23))
        );
    }
}
public extension AntiCircleRotor {
    public Motor wedge(Line other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((float3(this.scalar) * other.group0.xyz), -(this.e41 * other.e235) - (this.e42 * other.e315) - (this.e43 * other.e125) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435)), 
            /* e235, e315, e125, e5 */
            float4(float3(1.0), 0.0) * float4(other.group1.xyz, 0.0) * float4(this.group2.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
public extension AntiCircleRotor {
    public Motor wedge(Motor other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(this.scalar * other.e415, this.scalar * other.e425, this.scalar * other.e435, -(this.e41 * other.e235) - (this.e42 * other.e315) - (this.e43 * other.e125) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435)) + float4((this.group0.xyz * other.group1.www), this.scalar * other.e12345), 
            /* e235, e315, e125, e5 */
            float4(((float3(this.scalar) * other.group1.xyz) + (float3(other.e5) * this.group1.xyz)), this.scalar * other.e5)
        );
    }
}
public extension AntiCircleRotor {
    public MultiVector wedge(MultiVector other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(this.scalar * other.scalar, (this.scalar * other.e12345) - (this.e41 * other.e235) - (this.e42 * other.e315) - (this.e43 * other.e125) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435) - (this.e45 * other.e321) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412)), 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * other.group1, 
            /* e5 */
            this.scalar * other.e5, 
            /* e15, e25, e35, e45 */
            (float4(this.scalar) * other.group3) + (float4(other.scalar) * float4(this.group2.xyz, this.e45)), 
            /* e41, e42, e43 */
            (float3(this.scalar) * other.group4.xyz) + (float3(other.scalar) * this.group0.xyz), 
            /* e23, e31, e12 */
            (float3(this.scalar) * other.group5.xyz) + (float3(other.scalar) * this.group1.xyz), 
            /* e415, e425, e435, e321 */
            float4((this.e15 * other.e4) + (this.scalar * other.e415), (this.e25 * other.e4) + (this.scalar * other.e425), (this.e35 * other.e4) + (this.scalar * other.e435), -(this.e31 * other.e2) - (this.e12 * other.e3)) + float4((float3(other.e5) * this.group0.xyz), this.scalar * other.e321) - (this.group1.wwwx * other.group1.xyzx), 
            /* e423, e431, e412 */
            (float3(this.scalar) * other.group7.xyz) + (float3(other.e4) * this.group1.xyz) + (this.group0.xyz.yzx * other.group1.zxy) - (this.group0.xyz.zxy * other.group1.yzx), 
            /* e235, e315, e125 */
            (float3(this.scalar) * other.group8.xyz) + (float3(other.e5) * this.group1.xyz) + (this.group2.zxy * other.group1.yzx) - (this.group2.yzx * other.group1.zxy), 
            /* e4235, e4315, e4125, e3215 */
            float4((this.e23 * other.e45) + (this.e45 * other.e23) + (this.e35 * other.e42) + (this.scalar * other.e4235), (this.e31 * other.e45) + (this.e45 * other.e31) + (this.e15 * other.e43) + (this.scalar * other.e4315), (this.e12 * other.e45) + (this.e45 * other.e12) + (this.e25 * other.e41) + (this.scalar * other.e4125), -(this.e23 * other.e15) - (this.e31 * other.e25) - (this.e12 * other.e35) - (this.e35 * other.e12)) + float4((this.group0.xyz.yzx * other.group3.zxy), this.scalar * other.e3215) - (this.group2.yzxy * float4(other.group4.xyz.zxy, other.e31)) - float4((this.group0.xyz.zxy * other.group3.yzx), this.e15 * other.e23), 
            /* e1234 */
            (this.scalar * other.e1234) - (this.e41 * other.e23) - (this.e42 * other.e31) - (this.e43 * other.e12) - (this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43)
        );
    }
}
public extension AntiCircleRotor {
    public Plane wedge(Plane other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar) * other.group0
        );
    }
}
public extension AntiCircleRotor {
    public AntiDipoleInversion wedge(RoundPoint other) {
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (float3(other.e4) * this.group1.xyz) + (this.group0.xyz.yzx * other.group0.zxy) - (this.group0.xyz.zxy * other.group0.yzx), 
            /* e415, e425, e435, e321 */
            float4((this.e41 * other.e5) + (this.e15 * other.e4), (this.e42 * other.e5) + (this.e25 * other.e4), (this.e43 * other.e5) + (this.e35 * other.e4), -(this.e31 * other.e2) - (this.e12 * other.e3)) - (this.group1.wwwx * other.group0.xyzx), 
            /* e235, e315, e125, e4 */
            float4(((float3(other.e5) * this.group1.xyz) + (this.group2.zxy * other.group0.yzx) - (this.group2.yzx * other.group0.zxy)), this.scalar * other.e4), 
            /* e1, e2, e3, e5 */
            float4(this.scalar) * float4(other.group0.xyz, other.e5)
        );
    }
}
public extension AntiCircleRotor {
    public AntiCircleRotor wedge(Scalar other) {
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(other.scalar) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(other.scalar) * this.group1, 
            /* e15, e25, e35, scalar */
            float4(other.scalar) * this.group2
        );
    }
}
public extension AntiCircleRotor {
    public Sphere wedge(Sphere other) {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar) * other.group0, 
            /* e1234 */
            this.scalar * other.e1234
        );
    }
}
public extension AntiCircleRotor {
    public VersorEven wedge(VersorEven other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4((this.e23 * other.e4) + (this.scalar * other.e423), (this.e31 * other.e4) + (this.scalar * other.e431), (this.e12 * other.e4) + (this.scalar * other.e412), -(this.e42 * other.e315) - (this.e43 * other.e125) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435) - (this.e45 * other.e321) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412)) + float4((this.group0.xyz.yzx * other.group3.zxy), this.scalar * other.e12345) - float4((this.group0.xyz.zxy * other.group3.yzx), this.e41 * other.e235), 
            /* e415, e425, e435, e321 */
            float4((this.e15 * other.e4) + (this.scalar * other.e415), (this.e25 * other.e4) + (this.scalar * other.e425), (this.e35 * other.e4) + (this.scalar * other.e435), -(this.e31 * other.e2) - (this.e12 * other.e3)) + float4((this.group0.xyz * other.group2.www), this.scalar * other.e321) - (this.group1.wwwx * other.group3.xyzx), 
            /* e235, e315, e125, e5 */
            float4(((float3(this.scalar) * other.group2.xyz) + (float3(other.e5) * this.group1.xyz) + (this.group2.zxy * other.group3.yzx) - (this.group2.yzx * other.group3.zxy)), this.scalar * other.e5), 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * other.group3
        );
    }
}
public extension AntiCircleRotor {
    public VersorOdd wedge(VersorOdd other) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(((float3(this.scalar) * other.group0.xyz) + (float3(other.scalar) * this.group0.xyz)), this.scalar * other.scalar), 
            /* e23, e31, e12, e45 */
            (float4(this.scalar) * other.group1) + (float4(other.scalar) * this.group1), 
            /* e15, e25, e35, e1234 */
            float4(this.scalar * other.e15, this.scalar * other.e25, this.scalar * other.e35, -(this.e41 * other.e23) - (this.e42 * other.e31) - (this.e43 * other.e12) - (this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43)) + (this.group2 * float4(other.group0.www, other.e1234)), 
            /* e4235, e4315, e4125, e3215 */
            float4((this.e23 * other.e45) + (this.e45 * other.e23) + (this.e35 * other.e42) + (this.scalar * other.e4235), (this.e31 * other.e45) + (this.e45 * other.e31) + (this.e15 * other.e43) + (this.scalar * other.e4315), (this.e12 * other.e45) + (this.e45 * other.e12) + (this.e25 * other.e41) + (this.scalar * other.e4125), -(this.e12 * other.e35) - (this.e15 * other.e23) - (this.e25 * other.e31) - (this.e35 * other.e12)) + float4((this.group0.xyz.yzx * other.group2.zxy), this.scalar * other.e3215) - (other.group2.yzxx * float4(this.group0.xyz.zxy, this.e23)) - float4((this.group2.yzx * other.group0.zxy), this.e31 * other.e25)
        );
    }
}
public extension AntiDipoleInversion {
    // Fancy infix trick (first half)
    public WedgeInfixPartial<AntiDipoleInversion> operator /(WedgeInfix rhs) {
        return WedgeInfixPartial<AntiDipoleInversion>(this);
    }
}
public extension WedgeInfixPartial<AntiDipoleInversion> {
    // Fancy infix trick (second half)
    public VersorEven operator /(AntiCircleRotor rhs) {
        return this.a.wedge(rhs);
    }
}
public extension AntiDipoleInversion {
    public VersorEven wedge(AntiCircleRotor other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4((other.e42 * this.e3) + (other.e23 * this.e4) + (other.scalar * this.e423), (other.e43 * this.e1) + (other.e31 * this.e4) + (other.scalar * this.e431), (other.e41 * this.e2) + (other.e12 * this.e4) + (other.scalar * this.e412), -(other.e42 * this.e315) - (other.e43 * this.e125) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e45 * this.e321) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412)) - float4((other.group0.xyz.zxy * this.group3.yzx), other.e41 * this.e235), 
            /* e415, e425, e435, e321 */
            float4((other.e15 * this.e4) + (other.scalar * this.e415), (other.e25 * this.e4) + (other.scalar * this.e425), (other.e35 * this.e4) + (other.scalar * this.e435), -(other.e31 * this.e2) - (other.e12 * this.e3)) + float4((other.group0.xyz * this.group3.www), other.scalar * this.e321) - (other.group1.wwwx * this.group3.xyzx), 
            /* e235, e315, e125, e5 */
            float4(((float3(other.scalar) * this.group2.xyz) + (float3(this.e5) * other.group1.xyz) + (other.group2.zxy * this.group3.yzx) - (other.group2.yzx * this.group3.zxy)), other.scalar * this.e5), 
            /* e1, e2, e3, e4 */
            float4(other.scalar) * float4(this.group3.xyz, this.e4)
        );
    }
}
public extension AntiDipoleInversion {
    public DipoleInversion wedge(AntiDipoleInversion other) {
        return DipoleInversion(
            /* e41, e42, e43 */
            (float3(this.e4) * other.group3.xyz) - (float3(other.e4) * this.group3.xyz), 
            /* e23, e31, e12, e45 */
            (other.group3.zxyw * float4(this.group3.yzx, this.e4)) - (this.group3.zxyw * float4(other.group3.yzx, other.e4)), 
            /* e15, e25, e35, e1234 */
            (this.group3.xyzx * float4(other.group3.www, other.e423)) + float4(float3(0.0), (other.e431 * this.e2) + (other.e412 * this.e3) + (other.e321 * this.e4) - (other.e4 * this.e321) - (other.e2 * this.e431) - (other.e3 * this.e412)) - float4((this.group3.www * other.group3.xyz), other.e1 * this.e423), 
            /* e4235, e4315, e4125, e3215 */
            (other.group3.yzxw * this.group1.zxyw) + (this.group2.wwwz * float4(other.group2.xyz, other.e3)) + float4((this.group0.xyz * other.group3.www), other.e1 * this.e235) + float4((other.group1.yzx * this.group3.zxy), other.e2 * this.e315) - (float4(this.e5) * float4(other.group0.xyz, other.e321)) - (other.group2.wwwy * float4(this.group2.xyz, this.e2)) - (this.group3.yzxx * float4(other.group1.zxy, other.e235)) - float4((other.group3.zxy * this.group1.yzx), other.e125 * this.e3)
        );
    }
}
public extension AntiDipoleInversion {
    public VersorEven wedge(AntiDualNum other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(other.group0.xy.yy, other.scalar, other.e3215) * float4(this.group0.xyz, this.e4), 
            /* e415, e425, e435, e321 */
            float4(other.scalar) * this.group1, 
            /* e235, e315, e125, e5 */
            float4(other.scalar) * float4(this.group2.xyz, this.e5), 
            /* e1, e2, e3, e4 */
            float4(other.scalar) * float4(this.group3.xyz, this.e4)
        );
    }
}
public extension AntiDipoleInversion {
    public Sphere wedge(AntiFlatPoint other) {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((this.group2.www * other.group0.xyz), -(this.e1 * other.e235) - (this.e2 * other.e315) - (this.e3 * other.e125) - (this.e5 * other.e321)), 
            /* e1234 */
            this.e4 * other.e321
        );
    }
}
public extension AntiDipoleInversion {
    public DipoleInversion wedge(AntiFlector other) {
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(this.e4) * other.group1.xyz, 
            /* e23, e31, e12, e45 */
            float4(((this.group3.yzx * other.group1.zxy) - (this.group3.zxy * other.group1.yzx)), this.e4 * other.e5), 
            /* e15, e25, e35, e1234 */
            float4(float3(0.0), -(this.e431 * other.e2) - (this.e412 * other.e3)) + float4((other.group1.www * this.group3.xyz), this.e4 * other.e321) - (other.group1.xyzx * float4(this.group3.www, this.e423)), 
            /* e4235, e4315, e4125, e3215 */
            (float4(other.e5) * float4(this.group0.xyz, this.e321)) + (this.group2.wwwy * float4(other.group0.xyz, other.e2)) + (other.group1.yzxx * float4(this.group1.zxy, this.e235)) + float4(float3(0.0), (this.e125 * other.e3) - (this.e2 * other.e315) - (this.e3 * other.e125) - (this.e5 * other.e321)) - float4((this.group1.yzx * other.group1.zxy), this.e1 * other.e235)
        );
    }
}
public extension AntiDipoleInversion {
    public CircleRotor wedge(AntiLine other) {
        return CircleRotor(
            /* e423, e431, e412 */
            float3(this.e4) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4((float3(this.e4) * other.group1.xyz), -(this.e1 * other.e23) - (this.e2 * other.e31) - (this.e3 * other.e12)), 
            /* e235, e315, e125, e12345 */
            float4((this.e2 * other.e35) + (this.e5 * other.e23), (this.e3 * other.e15) + (this.e5 * other.e31), (this.e1 * other.e25) + (this.e5 * other.e12), -(this.e431 * other.e25) - (this.e412 * other.e35) - (this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12)) - float4((other.group1.xyz.yzx * this.group3.zxy), this.e423 * other.e15)
        );
    }
}
public extension AntiDipoleInversion {
    public VersorEven wedge(AntiMotor other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(this.e4 * other.e23, this.e4 * other.e31, this.e4 * other.e12, -(this.e423 * other.e15) - (this.e431 * other.e25) - (this.e412 * other.e35) - (this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12)) + float4((this.group0.xyz * other.group0.www), this.e4 * other.e3215), 
            /* e415, e425, e435, e321 */
            float4(this.e4 * other.e15, this.e4 * other.e25, this.e4 * other.e35, -(this.e1 * other.e23) - (this.e2 * other.e31) - (this.e3 * other.e12)) + (float4(other.scalar) * this.group1), 
            /* e235, e315, e125, e5 */
            float4(((float3(this.e5) * other.group0.xyz) + (float3(other.scalar) * this.group2.xyz) + (this.group3.yzx * other.group1.zxy) - (this.group3.zxy * other.group1.yzx)), this.e5 * other.scalar), 
            /* e1, e2, e3, e4 */
            float4(other.scalar) * float4(this.group3.xyz, this.e4)
        );
    }
}
public extension AntiDipoleInversion {
    public DipoleInversion wedge(AntiPlane other) {
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(this.e4) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(((this.group3.yzx * other.group0.zxy) - (this.group3.zxy * other.group0.yzx)), this.e4 * other.e5), 
            /* e15, e25, e35, e1234 */
            float4(this.e1 * other.e5, this.e2 * other.e5, this.e3 * other.e5, -(this.e431 * other.e2) - (this.e412 * other.e3)) - (other.group0.xyzx * float4(this.group3.www, this.e423)), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e425 * other.e3 * -1.0, this.e435 * other.e1 * -1.0, this.e415 * other.e2 * -1.0, (this.e315 * other.e2) + (this.e125 * other.e3)) + (float4(other.e5) * float4(this.group0.xyz, this.e321)) + (other.group0.yzxx * float4(this.group1.zxy, this.e235))
        );
    }
}
public extension AntiDipoleInversion {
    public Sphere wedge(Circle other) {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((this.e4 * other.e235) + (this.e3 * other.e425), (this.e4 * other.e315) + (this.e1 * other.e435), (this.e4 * other.e125) + (this.e2 * other.e415), -(this.e3 * other.e125) - (this.e5 * other.e321)) - (this.group3.wwwx * float4(other.group0.xyz, other.e235)) - float4((this.group3.yzx * other.group1.zxy), this.e2 * other.e315), 
            /* e1234 */
            (this.e4 * other.e321) + (this.e1 * other.e423) + (this.e2 * other.e431) + (this.e3 * other.e412)
        );
    }
}
public extension AntiDipoleInversion {
    public Sphere wedge(CircleRotor other) {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((this.e4 * other.e235) + (this.e3 * other.e425), (this.e4 * other.e315) + (this.e1 * other.e435), (this.e4 * other.e125) + (this.e2 * other.e415), -(this.e3 * other.e125) - (this.e5 * other.e321)) - (this.group3.yzxy * float4(other.group1.zxy, other.e315)) - float4((other.group0.xyz * this.group3.www), this.e1 * other.e235), 
            /* e1234 */
            (this.e4 * other.e321) + (this.e1 * other.e423) + (this.e2 * other.e431) + (this.e3 * other.e412)
        );
    }
}
public extension AntiDipoleInversion {
    public CircleRotor wedge(Dipole other) {
        return CircleRotor(
            /* e423, e431, e412 */
            (float3(this.e4) * other.group1.xyz) + (other.group0.xyz.yzx * this.group3.zxy) - (other.group0.xyz.zxy * this.group3.yzx), 
            /* e415, e425, e435, e321 */
            float4((this.e4 * other.e15) + (this.e5 * other.e41), (this.e4 * other.e25) + (this.e5 * other.e42), (this.e4 * other.e35) + (this.e5 * other.e43), -(this.e2 * other.e31) - (this.e3 * other.e12)) - (this.group3.xyzx * other.group1.wwwx), 
            /* e235, e315, e125, e12345 */
            float4((this.e2 * other.e35) + (this.e5 * other.e23), (this.e3 * other.e15) + (this.e5 * other.e31), (this.e1 * other.e25) + (this.e5 * other.e12), -(this.e431 * other.e25) - (this.e412 * other.e35) - (this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12) - (this.e321 * other.e45) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43)) - float4((other.group2.xyz.yzx * this.group3.zxy), this.e423 * other.e15)
        );
    }
}
public extension AntiDipoleInversion {
    public CircleRotor wedge(DipoleInversion other) {
        return CircleRotor(
            /* e423, e431, e412 */
            (float3(this.e4) * other.group1.xyz) + (other.group0.xyz.yzx * this.group3.zxy) - (other.group0.xyz.zxy * this.group3.yzx), 
            /* e415, e425, e435, e321 */
            float4((this.e4 * other.e15) + (this.e5 * other.e41), (this.e4 * other.e25) + (this.e5 * other.e42), (this.e4 * other.e35) + (this.e5 * other.e43), -(this.e2 * other.e31) - (this.e3 * other.e12)) - (this.group3.xyzx * other.group1.wwwx), 
            /* e235, e315, e125, e12345 */
            (this.group3.wwwx * float4(other.group1.xyz, other.e4235)) + float4(float3(0.0), (this.e2 * other.e4315) + (this.e3 * other.e4125) + (this.e5 * other.e1234) - (this.e431 * other.e25) - (this.e412 * other.e35) - (this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12) - (this.e321 * other.e45) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43)) + float4((this.group3.yzx * other.group2.zxy), this.e4 * other.e3215) - (other.group2.yzxx * float4(this.group3.zxy, this.e423))
        );
    }
}
public extension AntiDipoleInversion {
    public Flector wedge(DualNum other) {
        return Flector(
            /* e15, e25, e35, e45 */
            float4(other.e5) * float4(this.group3.xyz, this.e4), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.e5) * float4(this.group0.xyz, this.e321)
        );
    }
}
public extension AntiDipoleInversion {
    public Motor wedge(FlatPoint other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(this.e4 * other.e15, this.e4 * other.e25, this.e4 * other.e35, -(this.e431 * other.e25) - (this.e412 * other.e35) - (this.e321 * other.e45)) - (other.group0.wwwx * float4(this.group3.xyz, this.e423)), 
            /* e235, e315, e125, e5 */
            float4(((this.group3.yzx * other.group0.zxy) - (this.group3.zxy * other.group0.yzx)), 0.0)
        );
    }
}
public extension AntiDipoleInversion {
    public Motor wedge(Flector other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            (float4(this.e4) * float4(other.group0.xyz, other.e3215)) + float4(float3(0.0), (this.e1 * other.e4235) + (this.e2 * other.e4315) + (this.e3 * other.e4125) - (this.e431 * other.e25) - (this.e412 * other.e35) - (this.e321 * other.e45)) - (other.group0.wwwx * float4(this.group3.xyz, this.e423)), 
            /* e235, e315, e125, e5 */
            float4(((this.group3.yzx * other.group0.zxy) - (this.group3.zxy * other.group0.yzx)), 0.0)
        );
    }
}
public extension AntiDipoleInversion {
    public Plane wedge(Line other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4((this.e4 * other.e235) + (this.e3 * other.e425), (this.e4 * other.e315) + (this.e1 * other.e435), (this.e4 * other.e125) + (this.e2 * other.e415), -(this.e2 * other.e315) - (this.e3 * other.e125)) - (this.group3.yzxx * float4(other.group0.xyz.zxy, other.e235))
        );
    }
}
public extension AntiDipoleInversion {
    public Flector wedge(Motor other) {
        return Flector(
            /* e15, e25, e35, e45 */
            float4(other.e5) * float4(this.group3.xyz, this.e4), 
            /* e4235, e4315, e4125, e3215 */
            float4((this.e4 * other.e235) + (this.e3 * other.e425), (this.e4 * other.e315) + (this.e1 * other.e435), (this.e4 * other.e125) + (this.e2 * other.e415), -(this.e2 * other.e315) - (this.e3 * other.e125)) + (float4(other.e5) * float4(this.group0.xyz, this.e321)) - (this.group3.yzxx * float4(other.group0.zxy, other.e235))
        );
    }
}
public extension AntiDipoleInversion {
    public MultiVector wedge(MultiVector other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, (this.e4 * other.e3215) + (this.e1 * other.e4235) + (this.e2 * other.e4315) + (this.e3 * other.e4125) + (this.e5 * other.e1234) - (this.e423 * other.e15) - (this.e431 * other.e25) - (this.e412 * other.e35) - (this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12) - (this.e321 * other.e45) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43)), 
            /* e1, e2, e3, e4 */
            float4(other.scalar) * float4(this.group3.xyz, this.e4), 
            /* e5 */
            this.e5 * other.scalar, 
            /* e15, e25, e35, e45 */
            (float4(other.e5) * float4(this.group3.xyz, this.e4)) - (float4(this.e5) * other.group1), 
            /* e41, e42, e43 */
            (float3(this.e4) * other.group1.xyz) - (float3(other.e4) * this.group3.xyz), 
            /* e23, e31, e12 */
            (this.group3.yzx * other.group1.zxy) - (this.group3.zxy * other.group1.yzx), 
            /* e415, e425, e435, e321 */
            float4((this.e4 * other.e15) + (this.e5 * other.e41), (this.e4 * other.e25) + (this.e5 * other.e42), (this.e4 * other.e35) + (this.e5 * other.e43), -(this.e2 * other.e31) - (this.e3 * other.e12)) + (float4(other.scalar) * this.group1) - float4((other.group3.www * this.group3.xyz), this.e1 * other.e23), 
            /* e423, e431, e412 */
            (float3(this.e4) * other.group5.xyz) + (float3(other.scalar) * this.group0.xyz) + (other.group4.xyz.yzx * this.group3.zxy) - (other.group4.xyz.zxy * this.group3.yzx), 
            /* e235, e315, e125 */
            (float3(this.e5) * other.group5.xyz) + (float3(other.scalar) * this.group2.xyz) + (this.group3.yzx * other.group3.zxy) - (this.group3.zxy * other.group3.yzx), 
            /* e4235, e4315, e4125, e3215 */
            (float4(other.e5) * float4(this.group0.xyz, this.e321)) + (other.group1.yzxy * float4(this.group1.zxy, this.e315)) + float4((other.group8.xyz * this.group2.www), this.e235 * other.e1) + float4((this.group3.zxy * other.group6.yzx), this.e125 * other.e3) - (this.group3.yzxw * other.group6.zxyw) - (this.group3.wwwx * float4(other.group7.xyz, other.e235)) - float4((this.group1.yzx * other.group1.zxy), this.e2 * other.e315) - float4((other.group1.www * this.group2.xyz), this.e3 * other.e125), 
            /* e1234 */
            (this.e4 * other.e321) + (this.e1 * other.e423) + (this.e2 * other.e431) + (this.e3 * other.e412) - (this.e423 * other.e1) - (this.e431 * other.e2) - (this.e412 * other.e3) - (this.e321 * other.e4)
        );
    }
}
public extension AntiDipoleInversion {
    public AntiScalar wedge(Plane other) {
        return AntiScalar(
            /* e12345 */
            (this.e4 * other.e3215) + (this.e1 * other.e4235) + (this.e2 * other.e4315) + (this.e3 * other.e4125)
        );
    }
}
public extension AntiDipoleInversion {
    public DipoleInversion wedge(RoundPoint other) {
        return DipoleInversion(
            /* e41, e42, e43 */
            (float3(this.e4) * other.group0.xyz) - (float3(other.e4) * this.group3.xyz), 
            /* e23, e31, e12, e45 */
            float4((this.group3.yzx * other.group0.zxy), this.e4 * other.e5) - (this.group3.zxyw * other.group0.yzxw), 
            /* e15, e25, e35, e1234 */
            float4(this.e1 * other.e5, this.e2 * other.e5, this.e3 * other.e5, -(this.e431 * other.e2) - (this.e412 * other.e3) - (this.e321 * other.e4)) - (other.group0.xyzx * float4(this.group3.www, this.e423)), 
            /* e4235, e4315, e4125, e3215 */
            float4(-(this.e425 * other.e3) - (this.e235 * other.e4), -(this.e435 * other.e1) - (this.e315 * other.e4), -(this.e415 * other.e2) - (this.e125 * other.e4), (this.e315 * other.e2) + (this.e125 * other.e3)) + (float4(other.e5) * float4(this.group0.xyz, this.e321)) + (other.group0.yzxx * float4(this.group1.zxy, this.e235))
        );
    }
}
public extension AntiDipoleInversion {
    public AntiDipoleInversion wedge(Scalar other) {
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(other.scalar) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(other.scalar) * this.group1, 
            /* e235, e315, e125, e4 */
            float4(other.scalar) * this.group2, 
            /* e1, e2, e3, e5 */
            float4(other.scalar) * this.group3
        );
    }
}
public extension AntiDipoleInversion {
    public AntiScalar wedge(Sphere other) {
        return AntiScalar(
            /* e12345 */
            (this.e4 * other.e3215) + (this.e1 * other.e4235) + (this.e2 * other.e4315) + (this.e3 * other.e4125) + (this.e5 * other.e1234)
        );
    }
}
public extension AntiDipoleInversion {
    public DipoleInversion wedge(VersorEven other) {
        return DipoleInversion(
            /* e41, e42, e43 */
            (float3(this.e4) * other.group3.xyz) - (float3(other.e4) * this.group3.xyz), 
            /* e23, e31, e12, e45 */
            float4((this.group3.yzx * other.group3.zxy), this.e4 * other.e5) - (this.group3.zxyw * other.group3.yzxw), 
            /* e15, e25, e35, e1234 */
            float4(float3(0.0), (this.e1 * other.e423) + (this.e2 * other.e431) + (this.e3 * other.e412) - (this.e431 * other.e2) - (this.e412 * other.e3) - (this.e321 * other.e4)) + float4((other.group2.www * this.group3.xyz), this.e4 * other.e321) - (other.group3.xyzx * float4(this.group3.www, this.e423)), 
            /* e4235, e4315, e4125, e3215 */
            (float4(other.e5) * float4(this.group0.xyz, this.e321)) + (this.group2.wwwy * float4(other.group2.xyz, other.e2)) + (other.group3.yzxx * float4(this.group1.zxy, this.e235)) + float4((this.group3.zxy * other.group1.yzx), this.e125 * other.e3) - (float4(this.e5) * float4(other.group0.xyz, other.e321)) - (this.group3.yzxz * float4(other.group1.zxy, other.e125)) - float4((this.group1.yzx * other.group3.zxy), this.e1 * other.e235) - float4((other.group3.www * this.group2.xyz), this.e2 * other.e315)
        );
    }
}
public extension AntiDipoleInversion {
    public VersorEven wedge(VersorOdd other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (this.group3.zxyy * float4(other.group0.yzx, other.e4315)) + float4(float3(0.0), (this.e3 * other.e4125) + (this.e5 * other.e1234) - (this.e431 * other.e25) - (this.e412 * other.e35) - (this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12) - (this.e321 * other.e45) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43)) + float4((this.group0.xyz * other.group0.www), this.e4 * other.e3215) + float4((this.group2.www * other.group1.xyz), this.e1 * other.e4235) - float4((this.group3.yzx * other.group0.zxy), this.e423 * other.e15), 
            /* e415, e425, e435, e321 */
            float4((this.e4 * other.e15) + (this.e5 * other.e41), (this.e4 * other.e25) + (this.e5 * other.e42), (this.e4 * other.e35) + (this.e5 * other.e43), -(this.e2 * other.e31) - (this.e3 * other.e12)) + (float4(other.scalar) * this.group1) - (this.group3.xyzx * other.group1.wwwx), 
            /* e235, e315, e125, e5 */
            float4(((float3(this.e5) * other.group1.xyz) + (float3(other.scalar) * this.group2.xyz) + (this.group3.yzx * other.group2.zxy) - (this.group3.zxy * other.group2.yzx)), this.e5 * other.scalar), 
            /* e1, e2, e3, e4 */
            float4(other.scalar) * float4(this.group3.xyz, this.e4)
        );
    }
}
public extension AntiDualNum {
    // Fancy infix trick (first half)
    public WedgeInfixPartial<AntiDualNum> operator /(WedgeInfix rhs) {
        return WedgeInfixPartial<AntiDualNum>(this);
    }
}
public extension WedgeInfixPartial<AntiDualNum> {
    // Fancy infix trick (second half)
    public VersorOdd operator /(AntiCircleRotor rhs) {
        return this.a.wedge(rhs);
    }
}
public extension AntiDualNum {
    public VersorOdd wedge(AntiCircleRotor other) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(this.scalar) * float4(other.group0.xyz, other.scalar), 
            /* e23, e31, e12, e45 */
            float4(this.scalar) * other.group1, 
            /* e15, e25, e35, e1234 */
            float4(this.group0.xy.yy, this.scalar, 0.0) * float4(float3(1.0), 0.0) * float4(other.group2.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), other.scalar * this.e3215)
        );
    }
}
public extension AntiDualNum {
    public VersorEven wedge(AntiDipoleInversion other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(this.group0.xy.yy, this.scalar, this.e3215) * float4(other.group0.xyz, other.e4), 
            /* e415, e425, e435, e321 */
            float4(this.scalar) * other.group1, 
            /* e235, e315, e125, e5 */
            float4(this.scalar) * float4(other.group2.xyz, other.e5), 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * float4(other.group3.xyz, other.e4)
        );
    }
}
public extension AntiDualNum {
    public AntiDualNum wedge(AntiDualNum other) {
        return AntiDualNum(
            /* e3215, scalar */
            float2((other.e3215 * this.scalar) + (other.scalar * this.e3215), other.scalar * this.scalar)
        );
    }
}
public extension AntiDualNum {
    public AntiFlatPoint wedge(AntiFlatPoint other) {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(this.scalar) * other.group0
        );
    }
}
public extension AntiDualNum {
    public AntiFlector wedge(AntiFlector other) {
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(this.scalar) * other.group0, 
            /* e1, e2, e3, e5 */
            float4(this.scalar) * other.group1
        );
    }
}
public extension AntiDualNum {
    public AntiLine wedge(AntiLine other) {
        return AntiLine(
            /* e23, e31, e12 */
            float3(this.scalar) * other.group0.xyz, 
            /* e15, e25, e35 */
            float3(this.scalar) * other.group1.xyz
        );
    }
}
public extension AntiDualNum {
    public AntiMotor wedge(AntiMotor other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(this.scalar) * other.group0, 
            /* e15, e25, e35, e3215 */
            float4(this.group0.xy.yy, this.scalar, (this.e3215 * other.scalar) + (this.scalar * other.e3215)) * float4(other.group1.xyz, 1.0)
        );
    }
}
public extension AntiDualNum {
    public AntiPlane wedge(AntiPlane other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(this.scalar) * other.group0
        );
    }
}
public extension AntiDualNum {
    public AntiScalar wedge(AntiScalar other) {
        return AntiScalar(
            /* e12345 */
            this.scalar * other.e12345
        );
    }
}
public extension AntiDualNum {
    public Circle wedge(Circle other) {
        return Circle(
            /* e423, e431, e412 */
            float3(this.scalar) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(this.scalar) * other.group1, 
            /* e235, e315, e125 */
            float3(this.scalar) * other.group2.xyz
        );
    }
}
public extension AntiDualNum {
    public CircleRotor wedge(CircleRotor other) {
        return CircleRotor(
            /* e423, e431, e412 */
            float3(this.scalar) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(this.scalar) * other.group1, 
            /* e235, e315, e125, e12345 */
            float4(this.scalar) * other.group2
        );
    }
}
public extension AntiDualNum {
    public Dipole wedge(Dipole other) {
        return Dipole(
            /* e41, e42, e43 */
            float3(this.scalar) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(this.scalar) * other.group1, 
            /* e15, e25, e35 */
            float3(this.scalar) * other.group2.xyz
        );
    }
}
public extension AntiDualNum {
    public DipoleInversion wedge(DipoleInversion other) {
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(this.scalar) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(this.scalar) * other.group1, 
            /* e15, e25, e35, e1234 */
            float4(this.scalar) * other.group2, 
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar) * other.group3
        );
    }
}
public extension AntiDualNum {
    public DualNum wedge(DualNum other) {
        return DualNum(
            /* e5, e12345 */
            float2(this.scalar) * other.group0.xy
        );
    }
}
public extension AntiDualNum {
    public FlatPoint wedge(FlatPoint other) {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(this.scalar) * other.group0
        );
    }
}
public extension AntiDualNum {
    public Flector wedge(Flector other) {
        return Flector(
            /* e15, e25, e35, e45 */
            float4(this.scalar) * other.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar) * other.group1
        );
    }
}
public extension AntiDualNum {
    public Line wedge(Line other) {
        return Line(
            /* e415, e425, e435 */
            float3(this.scalar) * other.group0.xyz, 
            /* e235, e315, e125 */
            float3(this.scalar) * other.group1.xyz
        );
    }
}
public extension AntiDualNum {
    public Motor wedge(Motor other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(this.scalar) * other.group0, 
            /* e235, e315, e125, e5 */
            float4(this.scalar) * other.group1
        );
    }
}
public extension AntiDualNum {
    public MultiVector wedge(MultiVector other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(this.scalar * other.scalar, (this.e3215 * other.e4) + (this.scalar * other.e12345)), 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * other.group1, 
            /* e5 */
            this.scalar * other.e5, 
            /* e15, e25, e35, e45 */
            float4(this.scalar) * other.group3, 
            /* e41, e42, e43 */
            float3(this.scalar) * other.group4.xyz, 
            /* e23, e31, e12 */
            float3(this.scalar) * other.group5.xyz, 
            /* e415, e425, e435, e321 */
            float4(this.scalar) * other.group6, 
            /* e423, e431, e412 */
            float3(this.scalar) * other.group7.xyz, 
            /* e235, e315, e125 */
            float3(this.scalar) * other.group8.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(this.group0.xy.yy, this.scalar, (this.e3215 * other.scalar) + (this.scalar * other.e3215)) * float4(other.group9.xyz, 1.0), 
            /* e1234 */
            this.scalar * other.e1234
        );
    }
}
public extension AntiDualNum {
    public Plane wedge(Plane other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar) * other.group0
        );
    }
}
public extension AntiDualNum {
    public VersorEven wedge(RoundPoint other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(float3(0.0), this.e3215 * other.e4), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e235, e315, e125, e5 */
            float4(float3(0.0), this.scalar * other.e5), 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * other.group0
        );
    }
}
public extension AntiDualNum {
    public AntiDualNum wedge(Scalar other) {
        return AntiDualNum(
            /* e3215, scalar */
            float2(other.scalar) * this.group0.xy
        );
    }
}
public extension AntiDualNum {
    public Sphere wedge(Sphere other) {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar) * other.group0, 
            /* e1234 */
            this.scalar * other.e1234
        );
    }
}
public extension AntiDualNum {
    public VersorEven wedge(VersorEven other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(this.group0.xy.yy, this.scalar, (this.e3215 * other.e4) + (this.scalar * other.e12345)) * float4(other.group0.xyz, 1.0), 
            /* e415, e425, e435, e321 */
            float4(this.scalar) * other.group1, 
            /* e235, e315, e125, e5 */
            float4(this.scalar) * other.group2, 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * other.group3
        );
    }
}
public extension AntiDualNum {
    public VersorOdd wedge(VersorOdd other) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(this.scalar) * other.group0, 
            /* e23, e31, e12, e45 */
            float4(this.scalar) * other.group1, 
            /* e15, e25, e35, e1234 */
            float4(this.scalar) * other.group2, 
            /* e4235, e4315, e4125, e3215 */
            float4(this.group0.xy.yy, this.scalar, (this.e3215 * other.scalar) + (this.scalar * other.e3215)) * float4(other.group3.xyz, 1.0)
        );
    }
}
public extension AntiFlatPoint {
    // Fancy infix trick (first half)
    public WedgeInfixPartial<AntiFlatPoint> operator /(WedgeInfix rhs) {
        return WedgeInfixPartial<AntiFlatPoint>(this);
    }
}
public extension WedgeInfixPartial<AntiFlatPoint> {
    // Fancy infix trick (second half)
    public CircleRotor operator /(AntiCircleRotor rhs) {
        return this.a.wedge(rhs);
    }
}
public extension AntiFlatPoint {
    public CircleRotor wedge(AntiCircleRotor other) {
        return CircleRotor(
            /* e423, e431, e412 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), other.scalar * this.e321), 
            /* e235, e315, e125, e12345 */
            float4((other.group2.www * this.group0.xyz), -(other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (other.e45 * this.e321))
        );
    }
}
public extension AntiFlatPoint {
    public Sphere wedge(AntiDipoleInversion other) {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((other.group2.www * this.group0.xyz * float3(-1.0)), (other.e1 * this.e235) + (other.e2 * this.e315) + (other.e3 * this.e125) + (other.e5 * this.e321)), 
            /* e1234 */
            other.e4 * this.e321 * -1.0
        );
    }
}
public extension AntiFlatPoint {
    public AntiFlatPoint wedge(AntiDualNum other) {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(other.scalar) * this.group0
        );
    }
}
public extension AntiFlatPoint {
    public AntiDualNum wedge(AntiFlector other) {
        return AntiDualNum(
            /* e3215, scalar */
            float2((this.e235 * other.e1) + (this.e315 * other.e2) + (this.e125 * other.e3) + (this.e321 * other.e5), 0.0)
        );
    }
}
public extension AntiFlatPoint {
    public AntiFlatPoint wedge(AntiMotor other) {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(other.scalar) * this.group0
        );
    }
}
public extension AntiFlatPoint {
    public AntiDualNum wedge(AntiPlane other) {
        return AntiDualNum(
            /* e3215, scalar */
            float2((this.e235 * other.e1) + (this.e315 * other.e2) + (this.e125 * other.e3) + (this.e321 * other.e5), 0.0)
        );
    }
}
public extension AntiFlatPoint {
    public AntiScalar wedge(Dipole other) {
        return AntiScalar(
            /* e12345 */
            -(this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43) - (this.e321 * other.e45)
        );
    }
}
public extension AntiFlatPoint {
    public AntiScalar wedge(DipoleInversion other) {
        return AntiScalar(
            /* e12345 */
            -(this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43) - (this.e321 * other.e45)
        );
    }
}
public extension AntiFlatPoint {
    public AntiDualNum wedge(DualNum other) {
        return AntiDualNum(
            /* e3215, scalar */
            float2(this.e321 * other.e5, 1.0) * float2(1.0, 0.0)
        );
    }
}
public extension AntiFlatPoint {
    public AntiScalar wedge(FlatPoint other) {
        return AntiScalar(
            /* e12345 */
            this.e321 * other.e45 * -1.0
        );
    }
}
public extension AntiFlatPoint {
    public AntiScalar wedge(Flector other) {
        return AntiScalar(
            /* e12345 */
            this.e321 * other.e45 * -1.0
        );
    }
}
public extension AntiFlatPoint {
    public AntiDualNum wedge(Motor other) {
        return AntiDualNum(
            /* e3215, scalar */
            float2(this.e321 * other.e5, 1.0) * float2(1.0, 0.0)
        );
    }
}
public extension AntiFlatPoint {
    public MultiVector wedge(MultiVector other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, -(this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43) - (this.e321 * other.e45)), 
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
            float4(float3(0.0), this.e321 * other.scalar), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(other.scalar) * this.group0.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4((float3(other.e4) * this.group0.xyz * float3(-1.0)), (this.e235 * other.e1) + (this.e315 * other.e2) + (this.e125 * other.e3) + (this.e321 * other.e5)), 
            /* e1234 */
            this.e321 * other.e4 * -1.0
        );
    }
}
public extension AntiFlatPoint {
    public Sphere wedge(RoundPoint other) {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((float3(other.e4) * this.group0.xyz * float3(-1.0)), (this.e235 * other.e1) + (this.e315 * other.e2) + (this.e125 * other.e3) + (this.e321 * other.e5)), 
            /* e1234 */
            this.e321 * other.e4 * -1.0
        );
    }
}
public extension AntiFlatPoint {
    public AntiFlatPoint wedge(Scalar other) {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(other.scalar) * this.group0
        );
    }
}
public extension AntiFlatPoint {
    public Sphere wedge(VersorEven other) {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((float3(other.e4) * this.group0.xyz * float3(-1.0)), (this.e235 * other.e1) + (this.e315 * other.e2) + (this.e125 * other.e3) + (this.e321 * other.e5)), 
            /* e1234 */
            this.e321 * other.e4 * -1.0
        );
    }
}
public extension AntiFlatPoint {
    public CircleRotor wedge(VersorOdd other) {
        return CircleRotor(
            /* e423, e431, e412 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), this.e321 * other.scalar), 
            /* e235, e315, e125, e12345 */
            float4((float3(other.scalar) * this.group0.xyz), -(this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43) - (this.e321 * other.e45))
        );
    }
}
public extension AntiFlector {
    // Fancy infix trick (first half)
    public WedgeInfixPartial<AntiFlector> operator /(WedgeInfix rhs) {
        return WedgeInfixPartial<AntiFlector>(this);
    }
}
public extension WedgeInfixPartial<AntiFlector> {
    // Fancy infix trick (second half)
    public VersorEven operator /(AntiCircleRotor rhs) {
        return this.a.wedge(rhs);
    }
}
public extension AntiFlector {
    public VersorEven wedge(AntiCircleRotor other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(other.e42 * this.e3, other.e43 * this.e1, other.e41 * this.e2, -(other.e42 * this.e315) - (other.e43 * this.e125) - (other.e45 * this.e321)) - float4((other.group0.xyz.zxy * this.group1.yzx), other.e41 * this.e235), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), -(other.e31 * this.e2) - (other.e12 * this.e3)) + float4((other.group0.xyz * this.group1.www), other.scalar * this.e321) - (other.group1.wwwx * this.group1.xyzx), 
            /* e235, e315, e125, e5 */
            float4(((float3(other.scalar) * this.group0.xyz) + (float3(this.e5) * other.group1.xyz) + (other.group2.zxy * this.group1.yzx) - (other.group2.yzx * this.group1.zxy)), other.scalar * this.e5), 
            /* e1, e2, e3, e4 */
            float4(float3(1.0), 0.0) * float4(other.group2.www, 0.0) * float4(this.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
public extension AntiFlector {
    public DipoleInversion wedge(AntiDipoleInversion other) {
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(other.e4) * this.group1.xyz * float3(-1.0), 
            /* e23, e31, e12, e45 */
            float4(((other.group3.zxy * this.group1.yzx) - (other.group3.yzx * this.group1.zxy)), other.e4 * this.e5 * -1.0), 
            /* e15, e25, e35, e1234 */
            (this.group1.xyzx * float4(other.group3.www, other.e423)) + float4(float3(0.0), (other.e431 * this.e2) + (other.e412 * this.e3)) - float4((this.group1.www * other.group3.xyz), other.e4 * this.e321), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), (other.e2 * this.e315) + (other.e3 * this.e125) + (other.e5 * this.e321) - (other.e125 * this.e3)) + float4((other.group1.yzx * this.group1.zxy), other.e1 * this.e235) - (float4(this.e5) * float4(other.group0.xyz, other.e321)) - (other.group2.wwwy * float4(this.group0.xyz, this.e2)) - (this.group1.yzxx * float4(other.group1.zxy, other.e235))
        );
    }
}
public extension AntiFlector {
    public AntiFlector wedge(AntiDualNum other) {
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(other.scalar) * this.group0, 
            /* e1, e2, e3, e5 */
            float4(other.scalar) * this.group1
        );
    }
}
public extension AntiFlector {
    public AntiDualNum wedge(AntiFlatPoint other) {
        return AntiDualNum(
            /* e3215, scalar */
            float2(-(other.e235 * this.e1) - (other.e315 * this.e2) - (other.e125 * this.e3) - (other.e321 * this.e5), 0.0)
        );
    }
}
public extension AntiFlector {
    public AntiMotor wedge(AntiFlector other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(((other.group1.zxy * this.group1.yzx) - (other.group1.yzx * this.group1.zxy)), 0.0), 
            /* e15, e25, e35, e3215 */
            (other.group1.wwwx * float4(this.group1.xyz, this.e235)) + float4(float3(0.0), (other.e2 * this.e315) + (other.e3 * this.e125) + (other.e5 * this.e321) - (other.e315 * this.e2) - (other.e125 * this.e3) - (other.e321 * this.e5)) - (this.group1.wwwx * float4(other.group1.xyz, other.e235))
        );
    }
}
public extension AntiFlector {
    public AntiFlatPoint wedge(AntiLine other) {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4((this.e2 * other.e35) + (this.e5 * other.e23), (this.e3 * other.e15) + (this.e5 * other.e31), (this.e1 * other.e25) + (this.e5 * other.e12), -(this.e2 * other.e31) - (this.e3 * other.e12)) - (this.group1.zxyx * float4(other.group1.xyz.yzx, other.e23))
        );
    }
}
public extension AntiFlector {
    public AntiFlector wedge(AntiMotor other) {
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4((this.e2 * other.e35) + (this.e5 * other.e23), (this.e3 * other.e15) + (this.e5 * other.e31), (this.e1 * other.e25) + (this.e5 * other.e12), -(this.e2 * other.e31) - (this.e3 * other.e12)) + (float4(other.scalar) * this.group0) - (this.group1.zxyx * float4(other.group1.yzx, other.e23)), 
            /* e1, e2, e3, e5 */
            float4(other.scalar) * this.group1
        );
    }
}
public extension AntiFlector {
    public AntiMotor wedge(AntiPlane other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(((this.group1.yzx * other.group0.zxy) - (this.group1.zxy * other.group0.yzx)), 0.0), 
            /* e15, e25, e35, e3215 */
            float4(this.e5 * other.e1 * -1.0, this.e5 * other.e2 * -1.0, this.e5 * other.e3 * -1.0, (this.e315 * other.e2) + (this.e125 * other.e3) + (this.e321 * other.e5)) + (other.group0.wwwx * float4(this.group1.xyz, this.e235))
        );
    }
}
public extension AntiFlector {
    public Sphere wedge(Circle other) {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(this.e3 * other.e425, this.e1 * other.e435, this.e2 * other.e415, -(this.e3 * other.e125) - (this.e5 * other.e321)) - (this.group1.wwwx * float4(other.group0.xyz, other.e235)) - float4((this.group1.yzx * other.group1.zxy), this.e2 * other.e315), 
            /* e1234 */
            (this.e1 * other.e423) + (this.e2 * other.e431) + (this.e3 * other.e412)
        );
    }
}
public extension AntiFlector {
    public Sphere wedge(CircleRotor other) {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(this.e3 * other.e425, this.e1 * other.e435, this.e2 * other.e415, -(this.e3 * other.e125) - (this.e5 * other.e321)) - (this.group1.yzxy * float4(other.group1.zxy, other.e315)) - float4((other.group0.xyz * this.group1.www), this.e1 * other.e235), 
            /* e1234 */
            (this.e1 * other.e423) + (this.e2 * other.e431) + (this.e3 * other.e412)
        );
    }
}
public extension AntiFlector {
    public CircleRotor wedge(Dipole other) {
        return CircleRotor(
            /* e423, e431, e412 */
            (other.group0.xyz.yzx * this.group1.zxy) - (other.group0.xyz.zxy * this.group1.yzx), 
            /* e415, e425, e435, e321 */
            float4(this.e5 * other.e41, this.e5 * other.e42, this.e5 * other.e43, -(this.e2 * other.e31) - (this.e3 * other.e12)) - (this.group1.xyzx * other.group1.wwwx), 
            /* e235, e315, e125, e12345 */
            float4((this.e2 * other.e35) + (this.e5 * other.e23), (this.e3 * other.e15) + (this.e5 * other.e31), (this.e1 * other.e25) + (this.e5 * other.e12), -(this.e315 * other.e42) - (this.e125 * other.e43) - (this.e321 * other.e45)) - float4((other.group2.xyz.yzx * this.group1.zxy), this.e235 * other.e41)
        );
    }
}
public extension AntiFlector {
    public CircleRotor wedge(DipoleInversion other) {
        return CircleRotor(
            /* e423, e431, e412 */
            (other.group0.xyz.yzx * this.group1.zxy) - (other.group0.xyz.zxy * this.group1.yzx), 
            /* e415, e425, e435, e321 */
            float4(this.e5 * other.e41, this.e5 * other.e42, this.e5 * other.e43, -(this.e2 * other.e31) - (this.e3 * other.e12)) - (this.group1.xyzx * other.group1.wwwx), 
            /* e235, e315, e125, e12345 */
            (this.group1.yzxx * float4(other.group2.zxy, other.e4235)) + (this.group1.wwwy * float4(other.group1.xyz, other.e4315)) + float4(float3(0.0), (this.e3 * other.e4125) + (this.e5 * other.e1234) - (this.e315 * other.e42) - (this.e125 * other.e43) - (this.e321 * other.e45)) - float4((this.group1.zxy * other.group2.yzx), this.e235 * other.e41)
        );
    }
}
public extension AntiFlector {
    public Flector wedge(DualNum other) {
        return Flector(
            /* e15, e25, e35, e45 */
            float4(other.group0.xy.xx, other.e5, 0.0) * float4(float3(1.0), 0.0) * float4(this.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), this.e321 * other.e5)
        );
    }
}
public extension AntiFlector {
    public Motor wedge(FlatPoint other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(other.e45) * float4(this.group1.xyz, this.e321) * float4(-1.0), 
            /* e235, e315, e125, e5 */
            float4(((this.group1.yzx * other.group0.zxy) - (this.group1.zxy * other.group0.yzx)), 0.0)
        );
    }
}
public extension AntiFlector {
    public Motor wedge(Flector other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((float3(other.e45) * this.group1.xyz * float3(-1.0)), (this.e1 * other.e4235) + (this.e2 * other.e4315) + (this.e3 * other.e4125) - (this.e321 * other.e45)), 
            /* e235, e315, e125, e5 */
            float4(((this.group1.yzx * other.group0.zxy) - (this.group1.zxy * other.group0.yzx)), 0.0)
        );
    }
}
public extension AntiFlector {
    public Plane wedge(Line other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(this.e3 * other.e425, this.e1 * other.e435, this.e2 * other.e415, -(this.e2 * other.e315) - (this.e3 * other.e125)) - (this.group1.yzxx * float4(other.group0.xyz.zxy, other.e235))
        );
    }
}
public extension AntiFlector {
    public Flector wedge(Motor other) {
        return Flector(
            /* e15, e25, e35, e45 */
            float4(float3(1.0), 0.0) * float4(other.group1.www, 0.0) * float4(this.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), -(this.e2 * other.e315) - (this.e3 * other.e125)) + float4((this.group1.zxy * other.group0.yzx), this.e321 * other.e5) - (this.group1.yzxx * float4(other.group0.zxy, other.e235))
        );
    }
}
public extension AntiFlector {
    public MultiVector wedge(MultiVector other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, (this.e1 * other.e4235) + (this.e2 * other.e4315) + (this.e3 * other.e4125) + (this.e5 * other.e1234) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43) - (this.e321 * other.e45)), 
            /* e1, e2, e3, e4 */
            float4(other.group0.xy.xx, other.scalar, 0.0) * float4(float3(1.0), 0.0) * float4(this.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e5 */
            this.e5 * other.scalar, 
            /* e15, e25, e35, e45 */
            float4(((float3(other.e5) * this.group1.xyz) - (float3(this.e5) * other.group1.xyz)), this.e5 * other.e4 * -1.0), 
            /* e41, e42, e43 */
            float3(other.e4) * this.group1.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            (this.group1.yzx * other.group1.zxy) - (this.group1.zxy * other.group1.yzx), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), -(this.e2 * other.e31) - (this.e3 * other.e12)) + float4((other.group4.xyz * this.group1.www), this.e321 * other.scalar) - float4((other.group3.www * this.group1.xyz), this.e1 * other.e23), 
            /* e423, e431, e412 */
            (other.group4.xyz.yzx * this.group1.zxy) - (other.group4.xyz.zxy * this.group1.yzx), 
            /* e235, e315, e125 */
            (float3(this.e5) * other.group5.xyz) + (float3(other.scalar) * this.group0.xyz) + (this.group1.yzx * other.group3.zxy) - (this.group1.zxy * other.group3.yzx), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), (this.e315 * other.e2) + (this.e125 * other.e3) + (this.e321 * other.e5) - (this.e5 * other.e321)) + float4((this.group1.zxy * other.group6.yzx), this.e235 * other.e1) - (this.group1.wwwx * float4(other.group7.xyz, other.e235)) - float4((this.group1.yzx * other.group6.zxy), this.e3 * other.e125) - float4((other.group1.www * this.group0.xyz), this.e2 * other.e315), 
            /* e1234 */
            (this.e1 * other.e423) + (this.e2 * other.e431) + (this.e3 * other.e412) - (this.e321 * other.e4)
        );
    }
}
public extension AntiFlector {
    public AntiScalar wedge(Plane other) {
        return AntiScalar(
            /* e12345 */
            (this.e1 * other.e4235) + (this.e2 * other.e4315) + (this.e3 * other.e4125)
        );
    }
}
public extension AntiFlector {
    public DipoleInversion wedge(RoundPoint other) {
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(other.e4) * this.group1.xyz * float3(-1.0), 
            /* e23, e31, e12, e45 */
            float4(((this.group1.yzx * other.group0.zxy) - (this.group1.zxy * other.group0.yzx)), this.e5 * other.e4 * -1.0), 
            /* e15, e25, e35, e1234 */
            float4(((float3(other.e5) * this.group1.xyz) - (float3(this.e5) * other.group0.xyz)), this.e321 * other.e4 * -1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4((float3(other.e4) * this.group0.xyz * float3(-1.0)), (this.e235 * other.e1) + (this.e315 * other.e2) + (this.e125 * other.e3) + (this.e321 * other.e5))
        );
    }
}
public extension AntiFlector {
    public AntiFlector wedge(Scalar other) {
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(other.scalar) * this.group0, 
            /* e1, e2, e3, e5 */
            float4(other.scalar) * this.group1
        );
    }
}
public extension AntiFlector {
    public AntiScalar wedge(Sphere other) {
        return AntiScalar(
            /* e12345 */
            (this.e1 * other.e4235) + (this.e2 * other.e4315) + (this.e3 * other.e4125) + (this.e5 * other.e1234)
        );
    }
}
public extension AntiFlector {
    public DipoleInversion wedge(VersorEven other) {
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(other.e4) * this.group1.xyz * float3(-1.0), 
            /* e23, e31, e12, e45 */
            float4(((this.group1.yzx * other.group3.zxy) - (this.group1.zxy * other.group3.yzx)), this.e5 * other.e4 * -1.0), 
            /* e15, e25, e35, e1234 */
            (this.group1.xyzx * float4(other.group2.www, other.e423)) + float4(float3(0.0), (this.e2 * other.e431) + (this.e3 * other.e412)) - (other.group3 * float4(this.group1.www, this.e321)), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), (this.e315 * other.e2) + (this.e125 * other.e3) + (this.e321 * other.e5) - (this.e5 * other.e321)) + float4((this.group1.zxy * other.group1.yzx), this.e235 * other.e1) - (this.group1.yzxy * float4(other.group1.zxy, other.e315)) - (this.group1.wwwz * float4(other.group0.xyz, other.e125)) - float4((other.group3.www * this.group0.xyz), this.e1 * other.e235)
        );
    }
}
public extension AntiFlector {
    public VersorEven wedge(VersorOdd other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (this.group1.zxyx * float4(other.group0.yzx, other.e4235)) + float4(float3(0.0), (this.e2 * other.e4315) + (this.e3 * other.e4125) + (this.e5 * other.e1234) - (this.e315 * other.e42) - (this.e125 * other.e43) - (this.e321 * other.e45)) - (other.group0.zxyx * float4(this.group1.yzx, this.e235)), 
            /* e415, e425, e435, e321 */
            (other.group0 * float4(this.group1.www, this.e321)) + float4(float3(0.0), -(this.e2 * other.e31) - (this.e3 * other.e12)) - (this.group1.xyzx * other.group1.wwwx), 
            /* e235, e315, e125, e5 */
            float4(((float3(this.e5) * other.group1.xyz) + (float3(other.scalar) * this.group0.xyz) + (this.group1.yzx * other.group2.zxy) - (this.group1.zxy * other.group2.yzx)), this.e5 * other.scalar), 
            /* e1, e2, e3, e4 */
            float4(float3(1.0), 0.0) * float4(other.group0.www, 0.0) * float4(this.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
public extension AntiLine {
    // Fancy infix trick (first half)
    public WedgeInfixPartial<AntiLine> operator /(WedgeInfix rhs) {
        return WedgeInfixPartial<AntiLine>(this);
    }
}
public extension WedgeInfixPartial<AntiLine> {
    // Fancy infix trick (second half)
    public DipoleInversion operator /(AntiCircleRotor rhs) {
        return this.a.wedge(rhs);
    }
}
public extension AntiLine {
    public DipoleInversion wedge(AntiCircleRotor other) {
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12, e45 */
            float4(float3(1.0), 0.0) * float4(this.group0.xyz, 0.0) * float4(other.group2.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e15, e25, e35, e1234 */
            float4((float3(other.scalar) * this.group1.xyz), -(other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12)), 
            /* e4235, e4315, e4125, e3215 */
            float4((other.e42 * this.e35) + (other.e45 * this.e23), (other.e43 * this.e15) + (other.e45 * this.e31), (other.e41 * this.e25) + (other.e45 * this.e12), -(other.e23 * this.e15) - (other.e31 * this.e25) - (other.e12 * this.e35) - (other.e25 * this.e31) - (other.e35 * this.e12)) - float4((other.group0.xyz.zxy * this.group1.xyz.yzx), other.e15 * this.e23)
        );
    }
}
public extension AntiLine {
    public CircleRotor wedge(AntiDipoleInversion other) {
        return CircleRotor(
            /* e423, e431, e412 */
            float3(other.e4) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4((float3(other.e4) * this.group1.xyz), -(other.e1 * this.e23) - (other.e2 * this.e31) - (other.e3 * this.e12)), 
            /* e235, e315, e125, e12345 */
            float4((other.e2 * this.e35) + (other.e5 * this.e23), (other.e3 * this.e15) + (other.e5 * this.e31), (other.e1 * this.e25) + (other.e5 * this.e12), -(other.e431 * this.e25) - (other.e412 * this.e35) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12)) - float4((this.group1.xyz.yzx * other.group3.zxy), other.e423 * this.e15)
        );
    }
}
public extension AntiLine {
    public AntiLine wedge(AntiDualNum other) {
        return AntiLine(
            /* e23, e31, e12 */
            float3(other.scalar) * this.group0.xyz, 
            /* e15, e25, e35 */
            float3(other.scalar) * this.group1.xyz
        );
    }
}
public extension AntiLine {
    public AntiFlatPoint wedge(AntiFlector other) {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4((other.e2 * this.e35) + (other.e5 * this.e23), (other.e3 * this.e15) + (other.e5 * this.e31), (other.e1 * this.e25) + (other.e5 * this.e12), -(other.e2 * this.e31) - (other.e3 * this.e12)) - (other.group1.zxyx * float4(this.group1.xyz.yzx, this.e23))
        );
    }
}
public extension AntiLine {
    public AntiDualNum wedge(AntiLine other) {
        return AntiDualNum(
            /* e3215, scalar */
            float2(-(other.e23 * this.e15) - (other.e31 * this.e25) - (other.e12 * this.e35) - (other.e15 * this.e23) - (other.e25 * this.e31) - (other.e35 * this.e12), 0.0)
        );
    }
}
public extension AntiLine {
    public AntiMotor wedge(AntiMotor other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(float3(1.0), 0.0) * float4(this.group0.xyz, 0.0) * float4(other.group0.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e15, e25, e35, e3215 */
            float4((float3(other.scalar) * this.group1.xyz), -(this.e23 * other.e15) - (this.e31 * other.e25) - (this.e12 * other.e35) - (this.e15 * other.e23) - (this.e25 * other.e31) - (this.e35 * other.e12))
        );
    }
}
public extension AntiLine {
    public AntiFlatPoint wedge(AntiPlane other) {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4((this.e23 * other.e5) + (this.e35 * other.e2), (this.e31 * other.e5) + (this.e15 * other.e3), (this.e12 * other.e5) + (this.e25 * other.e1), -(this.e31 * other.e2) - (this.e12 * other.e3)) - (other.group0.zxyx * float4(this.group1.xyz.yzx, this.e23))
        );
    }
}
public extension AntiLine {
    public AntiScalar wedge(Circle other) {
        return AntiScalar(
            /* e12345 */
            -(this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412)
        );
    }
}
public extension AntiLine {
    public AntiScalar wedge(CircleRotor other) {
        return AntiScalar(
            /* e12345 */
            -(this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412)
        );
    }
}
public extension AntiLine {
    public Sphere wedge(Dipole other) {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((this.e23 * other.e45) + (this.e35 * other.e42), (this.e31 * other.e45) + (this.e15 * other.e43), (this.e12 * other.e45) + (this.e25 * other.e41), -(this.e31 * other.e25) - (this.e12 * other.e35) - (this.e15 * other.e23) - (this.e25 * other.e31) - (this.e35 * other.e12)) - float4((this.group1.xyz.yzx * other.group0.xyz.zxy), this.e23 * other.e15), 
            /* e1234 */
            -(this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43)
        );
    }
}
public extension AntiLine {
    public Sphere wedge(DipoleInversion other) {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((this.e23 * other.e45) + (this.e35 * other.e42), (this.e31 * other.e45) + (this.e15 * other.e43), (this.e12 * other.e45) + (this.e25 * other.e41), -(this.e31 * other.e25) - (this.e12 * other.e35) - (this.e15 * other.e23) - (this.e25 * other.e31) - (this.e35 * other.e12)) - float4((this.group1.xyz.yzx * other.group0.xyz.zxy), this.e23 * other.e15), 
            /* e1234 */
            -(this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43)
        );
    }
}
public extension AntiLine {
    public AntiFlatPoint wedge(DualNum other) {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(other.group0.xy.xx, other.e5, 0.0) * float4(float3(1.0), 0.0) * float4(this.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
public extension AntiLine {
    public Plane wedge(FlatPoint other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4((float3(other.e45) * this.group0.xyz), -(this.e23 * other.e15) - (this.e31 * other.e25) - (this.e12 * other.e35))
        );
    }
}
public extension AntiLine {
    public Plane wedge(Flector other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4((float3(other.e45) * this.group0.xyz), -(this.e23 * other.e15) - (this.e31 * other.e25) - (this.e12 * other.e35))
        );
    }
}
public extension AntiLine {
    public AntiScalar wedge(Line other) {
        return AntiScalar(
            /* e12345 */
            -(this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435)
        );
    }
}
public extension AntiLine {
    public Motor wedge(Motor other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(float3(0.0), -(this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435)), 
            /* e235, e315, e125, e5 */
            float4(float3(1.0), 0.0) * float4(this.group0.xyz, 0.0) * float4(other.group1.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
public extension AntiLine {
    public MultiVector wedge(MultiVector other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, -(this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            float4(other.group0.xy.xx, other.scalar, 0.0) * float4(float3(1.0), 0.0) * float4(this.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(other.scalar) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4((float3(other.e4) * this.group1.xyz), -(this.e23 * other.e1) - (this.e31 * other.e2) - (this.e12 * other.e3)), 
            /* e423, e431, e412 */
            float3(other.e4) * this.group0.xyz, 
            /* e235, e315, e125 */
            (float3(other.e5) * this.group0.xyz) + (this.group1.xyz.zxy * other.group1.yzx) - (this.group1.xyz.yzx * other.group1.zxy), 
            /* e4235, e4315, e4125, e3215 */
            float4((this.e23 * other.e45) + (this.e35 * other.e42), (this.e31 * other.e45) + (this.e15 * other.e43), (this.e12 * other.e45) + (this.e25 * other.e41), -(this.e31 * other.e25) - (this.e12 * other.e35) - (this.e15 * other.e23) - (this.e25 * other.e31) - (this.e35 * other.e12)) - float4((this.group1.xyz.yzx * other.group4.xyz.zxy), this.e23 * other.e15), 
            /* e1234 */
            -(this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43)
        );
    }
}
public extension AntiLine {
    public Circle wedge(RoundPoint other) {
        return Circle(
            /* e423, e431, e412 */
            float3(other.e4) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4((float3(other.e4) * this.group1.xyz), -(this.e23 * other.e1) - (this.e31 * other.e2) - (this.e12 * other.e3)), 
            /* e235, e315, e125 */
            (float3(other.e5) * this.group0.xyz) + (this.group1.xyz.zxy * other.group0.yzx) - (this.group1.xyz.yzx * other.group0.zxy)
        );
    }
}
public extension AntiLine {
    public AntiLine wedge(Scalar other) {
        return AntiLine(
            /* e23, e31, e12 */
            float3(other.scalar) * this.group0.xyz, 
            /* e15, e25, e35 */
            float3(other.scalar) * this.group1.xyz
        );
    }
}
public extension AntiLine {
    public CircleRotor wedge(VersorEven other) {
        return CircleRotor(
            /* e423, e431, e412 */
            float3(other.e4) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4((float3(other.e4) * this.group1.xyz), -(this.e23 * other.e1) - (this.e31 * other.e2) - (this.e12 * other.e3)), 
            /* e235, e315, e125, e12345 */
            float4((this.e23 * other.e5) + (this.e35 * other.e2), (this.e31 * other.e5) + (this.e15 * other.e3), (this.e12 * other.e5) + (this.e25 * other.e1), -(this.e31 * other.e425) - (this.e12 * other.e435) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412)) - float4((this.group1.xyz.yzx * other.group3.zxy), this.e23 * other.e415)
        );
    }
}
public extension AntiLine {
    public DipoleInversion wedge(VersorOdd other) {
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12, e45 */
            float4(float3(1.0), 0.0) * float4(this.group0.xyz, 0.0) * float4(other.group0.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e15, e25, e35, e1234 */
            float4((float3(other.scalar) * this.group1.xyz), -(this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43)), 
            /* e4235, e4315, e4125, e3215 */
            float4((this.e23 * other.e45) + (this.e35 * other.e42), (this.e31 * other.e45) + (this.e15 * other.e43), (this.e12 * other.e45) + (this.e25 * other.e41), -(this.e31 * other.e25) - (this.e12 * other.e35) - (this.e15 * other.e23) - (this.e25 * other.e31) - (this.e35 * other.e12)) - float4((this.group1.xyz.yzx * other.group0.zxy), this.e23 * other.e15)
        );
    }
}
public extension AntiMotor {
    // Fancy infix trick (first half)
    public WedgeInfixPartial<AntiMotor> operator /(WedgeInfix rhs) {
        return WedgeInfixPartial<AntiMotor>(this);
    }
}
public extension WedgeInfixPartial<AntiMotor> {
    // Fancy infix trick (second half)
    public VersorOdd operator /(AntiCircleRotor rhs) {
        return this.a.wedge(rhs);
    }
}
public extension AntiMotor {
    public VersorOdd wedge(AntiCircleRotor other) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(this.scalar) * float4(other.group0.xyz, other.scalar), 
            /* e23, e31, e12, e45 */
            float4(((float3(other.scalar) * this.group0.xyz) + (float3(this.scalar) * other.group1.xyz)), other.e45 * this.scalar), 
            /* e15, e25, e35, e1234 */
            float4((other.e15 * this.scalar) + (other.scalar * this.e15), (other.e25 * this.scalar) + (other.scalar * this.e25), (other.e35 * this.scalar) + (other.scalar * this.e35), -(other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12)), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.e45 * this.e23, other.e45 * this.e31, other.e45 * this.e12, -(other.e31 * this.e25) - (other.e12 * this.e35) - (other.e15 * this.e23) - (other.e25 * this.e31) - (other.e35 * this.e12)) + (this.group1.zxyw * float4(other.group0.xyz.yzx, other.scalar)) - (this.group1.yzxx * float4(other.group0.xyz.zxy, other.e23))
        );
    }
}
public extension AntiMotor {
    public VersorEven wedge(AntiDipoleInversion other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(other.e4 * this.e23, other.e4 * this.e31, other.e4 * this.e12, -(other.e423 * this.e15) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12)) + float4((other.group0.xyz * this.group0.www), other.e4 * this.e3215), 
            /* e415, e425, e435, e321 */
            float4(other.e4 * this.e15, other.e4 * this.e25, other.e4 * this.e35, -(other.e1 * this.e23) - (other.e2 * this.e31) - (other.e3 * this.e12)) + (float4(this.scalar) * other.group1), 
            /* e235, e315, e125, e5 */
            float4(((float3(other.e5) * this.group0.xyz) + (float3(this.scalar) * other.group2.xyz) + (other.group3.yzx * this.group1.zxy) - (other.group3.zxy * this.group1.yzx)), other.e5 * this.scalar), 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * float4(other.group3.xyz, other.e4)
        );
    }
}
public extension AntiMotor {
    public AntiMotor wedge(AntiDualNum other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(other.scalar) * this.group0, 
            /* e15, e25, e35, e3215 */
            float4(other.group0.xy.yy, other.scalar, (other.e3215 * this.scalar) + (other.scalar * this.e3215)) * float4(this.group1.xyz, 1.0)
        );
    }
}
public extension AntiMotor {
    public AntiFlatPoint wedge(AntiFlatPoint other) {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(this.scalar) * other.group0
        );
    }
}
public extension AntiMotor {
    public AntiFlector wedge(AntiFlector other) {
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4((other.e2 * this.e35) + (other.e5 * this.e23), (other.e3 * this.e15) + (other.e5 * this.e31), (other.e1 * this.e25) + (other.e5 * this.e12), -(other.e2 * this.e31) - (other.e3 * this.e12)) + (float4(this.scalar) * other.group0) - (other.group1.zxyx * float4(this.group1.yzx, this.e23)), 
            /* e1, e2, e3, e5 */
            float4(this.scalar) * other.group1
        );
    }
}
public extension AntiMotor {
    public AntiMotor wedge(AntiLine other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(float3(1.0), 0.0) * float4(other.group0.xyz, 0.0) * float4(this.group0.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e15, e25, e35, e3215 */
            float4((float3(this.scalar) * other.group1.xyz), -(other.e23 * this.e15) - (other.e31 * this.e25) - (other.e12 * this.e35) - (other.e15 * this.e23) - (other.e25 * this.e31) - (other.e35 * this.e12))
        );
    }
}
public extension AntiMotor {
    public AntiMotor wedge(AntiMotor other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(((float3(other.scalar) * this.group0.xyz) + (float3(this.scalar) * other.group0.xyz)), other.scalar * this.scalar), 
            /* e15, e25, e35, e3215 */
            (float4(other.scalar) * this.group1) + (float4(this.scalar) * other.group1) + float4(float3(0.0), -(other.e23 * this.e15) - (other.e31 * this.e25) - (other.e12 * this.e35) - (other.e15 * this.e23) - (other.e25 * this.e31) - (other.e35 * this.e12))
        );
    }
}
public extension AntiMotor {
    public AntiFlector wedge(AntiPlane other) {
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4((this.e23 * other.e5) + (this.e35 * other.e2), (this.e31 * other.e5) + (this.e15 * other.e3), (this.e12 * other.e5) + (this.e25 * other.e1), -(this.e31 * other.e2) - (this.e12 * other.e3)) - (other.group0.zxyx * float4(this.group1.yzx, this.e23)), 
            /* e1, e2, e3, e5 */
            float4(this.scalar) * other.group0
        );
    }
}
public extension AntiMotor {
    public AntiScalar wedge(AntiScalar other) {
        return AntiScalar(
            /* e12345 */
            this.scalar * other.e12345
        );
    }
}
public extension AntiMotor {
    public CircleRotor wedge(Circle other) {
        return CircleRotor(
            /* e423, e431, e412 */
            float3(this.scalar) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(this.scalar) * other.group1, 
            /* e235, e315, e125, e12345 */
            float4((float3(this.scalar) * other.group2.xyz), -(this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412))
        );
    }
}
public extension AntiMotor {
    public CircleRotor wedge(CircleRotor other) {
        return CircleRotor(
            /* e423, e431, e412 */
            float3(this.scalar) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(this.scalar) * other.group1, 
            /* e235, e315, e125, e12345 */
            float4((this.group0.www * other.group2.xyz), (this.scalar * other.e12345) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412))
        );
    }
}
public extension AntiMotor {
    public DipoleInversion wedge(Dipole other) {
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(this.scalar) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(this.scalar) * other.group1, 
            /* e15, e25, e35, e1234 */
            float4((float3(this.scalar) * other.group2.xyz), -(this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43)), 
            /* e4235, e4315, e4125, e3215 */
            float4((this.e23 * other.e45) + (this.e35 * other.e42), (this.e31 * other.e45) + (this.e15 * other.e43), (this.e12 * other.e45) + (this.e25 * other.e41), -(this.e31 * other.e25) - (this.e12 * other.e35) - (this.e15 * other.e23) - (this.e25 * other.e31) - (this.e35 * other.e12)) - float4((other.group0.xyz.zxy * this.group1.yzx), this.e23 * other.e15)
        );
    }
}
public extension AntiMotor {
    public DipoleInversion wedge(DipoleInversion other) {
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(this.scalar) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(this.scalar) * other.group1, 
            /* e15, e25, e35, e1234 */
            float4((this.group0.www * other.group2.xyz), (this.scalar * other.e1234) - (this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43)), 
            /* e4235, e4315, e4125, e3215 */
            float4((this.e23 * other.e45) + (this.scalar * other.e4235), (this.e31 * other.e45) + (this.scalar * other.e4315), (this.e12 * other.e45) + (this.scalar * other.e4125), -(this.e31 * other.e25) - (this.e12 * other.e35) - (this.e15 * other.e23) - (this.e25 * other.e31) - (this.e35 * other.e12)) + float4((other.group0.xyz.yzx * this.group1.zxy), this.scalar * other.e3215) - float4((other.group0.xyz.zxy * this.group1.yzx), this.e23 * other.e15)
        );
    }
}
public extension AntiMotor {
    public Motor wedge(DualNum other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(float3(0.0), this.scalar * other.e12345), 
            /* e235, e315, e125, e5 */
            float4(other.e5) * this.group0
        );
    }
}
public extension AntiMotor {
    public Flector wedge(FlatPoint other) {
        return Flector(
            /* e15, e25, e35, e45 */
            float4(this.scalar) * other.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4((float3(other.e45) * this.group0.xyz), -(this.e23 * other.e15) - (this.e31 * other.e25) - (this.e12 * other.e35))
        );
    }
}
public extension AntiMotor {
    public Flector wedge(Flector other) {
        return Flector(
            /* e15, e25, e35, e45 */
            float4(this.scalar) * other.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar * other.e4235, this.scalar * other.e4315, this.scalar * other.e4125, -(this.e23 * other.e15) - (this.e31 * other.e25) - (this.e12 * other.e35)) + (this.group0 * float4(other.group0.www, other.e3215))
        );
    }
}
public extension AntiMotor {
    public Motor wedge(Line other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((float3(this.scalar) * other.group0.xyz), -(this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435)), 
            /* e235, e315, e125, e5 */
            float4(float3(1.0), 0.0) * float4(other.group1.xyz, 0.0) * float4(this.group0.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
public extension AntiMotor {
    public Motor wedge(Motor other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((this.group0.www * other.group0.xyz), (this.scalar * other.e12345) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435)), 
            /* e235, e315, e125, e5 */
            float4(((float3(this.scalar) * other.group1.xyz) + (float3(other.e5) * this.group0.xyz)), this.scalar * other.e5)
        );
    }
}
public extension AntiMotor {
    public MultiVector wedge(MultiVector other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(this.scalar * other.scalar, (this.scalar * other.e12345) + (this.e3215 * other.e4) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412)), 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * other.group1, 
            /* e5 */
            this.scalar * other.e5, 
            /* e15, e25, e35, e45 */
            float4(((float3(this.scalar) * other.group3.xyz) + (float3(other.scalar) * this.group1.xyz)), this.scalar * other.e45), 
            /* e41, e42, e43 */
            float3(this.scalar) * other.group4.xyz, 
            /* e23, e31, e12 */
            (float3(this.scalar) * other.group5.xyz) + (float3(other.scalar) * this.group0.xyz), 
            /* e415, e425, e435, e321 */
            float4(this.e15 * other.e4, this.e25 * other.e4, this.e35 * other.e4, -(this.e23 * other.e1) - (this.e31 * other.e2) - (this.e12 * other.e3)) + (float4(this.scalar) * other.group6), 
            /* e423, e431, e412 */
            (float3(this.scalar) * other.group7.xyz) + (float3(other.e4) * this.group0.xyz), 
            /* e235, e315, e125 */
            (float3(this.scalar) * other.group8.xyz) + (float3(other.e5) * this.group0.xyz) + (this.group1.zxy * other.group1.yzx) - (this.group1.yzx * other.group1.zxy), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar * other.e4235, this.scalar * other.e4315, this.scalar * other.e4125, -(this.e23 * other.e15) - (this.e31 * other.e25) - (this.e12 * other.e35) - (this.e25 * other.e31) - (this.e35 * other.e12)) + (this.group0 * float4(other.group3.www, other.e3215)) + (this.group1.zxyw * float4(other.group4.xyz.yzx, other.scalar)) - (this.group1.yzxx * float4(other.group4.xyz.zxy, other.e23)), 
            /* e1234 */
            (this.scalar * other.e1234) - (this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43)
        );
    }
}
public extension AntiMotor {
    public Plane wedge(Plane other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar) * other.group0
        );
    }
}
public extension AntiMotor {
    public VersorEven wedge(RoundPoint other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(other.e4) * float4(this.group0.xyz, this.e3215), 
            /* e415, e425, e435, e321 */
            float4((float3(other.e4) * this.group1.xyz), -(this.e23 * other.e1) - (this.e31 * other.e2) - (this.e12 * other.e3)), 
            /* e235, e315, e125, e5 */
            float4(((float3(other.e5) * this.group0.xyz) + (this.group1.zxy * other.group0.yzx) - (this.group1.yzx * other.group0.zxy)), this.scalar * other.e5), 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * other.group0
        );
    }
}
public extension AntiMotor {
    public AntiMotor wedge(Scalar other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(other.scalar) * this.group0, 
            /* e15, e25, e35, e3215 */
            float4(other.scalar) * this.group1
        );
    }
}
public extension AntiMotor {
    public Sphere wedge(Sphere other) {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar) * other.group0, 
            /* e1234 */
            this.scalar * other.e1234
        );
    }
}
public extension AntiMotor {
    public VersorEven wedge(VersorEven other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (this.group0 * float4(other.group3.www, other.e12345)) + float4(float3(0.0), -(this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412)) + float4((this.group0.www * other.group0.xyz), this.e3215 * other.e4), 
            /* e415, e425, e435, e321 */
            float4(this.e15 * other.e4, this.e25 * other.e4, this.e35 * other.e4, -(this.e23 * other.e1) - (this.e31 * other.e2) - (this.e12 * other.e3)) + (float4(this.scalar) * other.group1), 
            /* e235, e315, e125, e5 */
            float4(((float3(this.scalar) * other.group2.xyz) + (float3(other.e5) * this.group0.xyz) + (this.group1.zxy * other.group3.yzx) - (this.group1.yzx * other.group3.zxy)), this.scalar * other.e5), 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * other.group3
        );
    }
}
public extension AntiMotor {
    public VersorOdd wedge(VersorOdd other) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(this.scalar) * other.group0, 
            /* e23, e31, e12, e45 */
            float4(((float3(this.scalar) * other.group1.xyz) + (float3(other.scalar) * this.group0.xyz)), this.scalar * other.e45), 
            /* e15, e25, e35, e1234 */
            float4(this.e15 * other.scalar, this.e25 * other.scalar, this.e35 * other.scalar, -(this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43)) + (float4(this.scalar) * other.group2), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e35 * other.e42, this.e15 * other.e43, this.e25 * other.e41, -(this.e31 * other.e25) - (this.e12 * other.e35) - (this.e15 * other.e23) - (this.e25 * other.e31) - (this.e35 * other.e12)) + (this.group0 * float4(other.group1.www, other.e3215)) + float4((this.group0.www * other.group3.xyz), this.e3215 * other.scalar) - float4((this.group1.yzx * other.group0.zxy), this.e23 * other.e15)
        );
    }
}
public extension AntiPlane {
    // Fancy infix trick (first half)
    public WedgeInfixPartial<AntiPlane> operator /(WedgeInfix rhs) {
        return WedgeInfixPartial<AntiPlane>(this);
    }
}
public extension WedgeInfixPartial<AntiPlane> {
    // Fancy infix trick (second half)
    public AntiDipoleInversion operator /(AntiCircleRotor rhs) {
        return this.a.wedge(rhs);
    }
}
public extension AntiPlane {
    public AntiDipoleInversion wedge(AntiCircleRotor other) {
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (other.group0.xyz.yzx * this.group0.zxy) - (other.group0.xyz.zxy * this.group0.yzx), 
            /* e415, e425, e435, e321 */
            float4(other.e41 * this.e5, other.e42 * this.e5, other.e43 * this.e5, -(other.e31 * this.e2) - (other.e12 * this.e3)) - (other.group1.wwwx * this.group0.xyzx), 
            /* e235, e315, e125, e4 */
            float4((float3(this.e5) * other.group1.xyz), 0.0) + float4((other.group2.zxy * this.group0.yzx), 0.0) - float4((other.group2.yzx * this.group0.zxy), 0.0), 
            /* e1, e2, e3, e5 */
            float4(other.scalar) * this.group0
        );
    }
}
public extension AntiPlane {
    public DipoleInversion wedge(AntiDipoleInversion other) {
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(other.e4) * this.group0.xyz * float3(-1.0), 
            /* e23, e31, e12, e45 */
            float4(((other.group3.zxy * this.group0.yzx) - (other.group3.yzx * this.group0.zxy)), other.e4 * this.e5 * -1.0), 
            /* e15, e25, e35, e1234 */
            float4(other.e1 * this.e5 * -1.0, other.e2 * this.e5 * -1.0, other.e3 * this.e5 * -1.0, (other.e431 * this.e2) + (other.e412 * this.e3)) + (this.group0.xyzx * float4(other.group3.www, other.e423)), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.e425 * this.e3, other.e435 * this.e1, other.e415 * this.e2, -(other.e315 * this.e2) - (other.e125 * this.e3)) - (float4(this.e5) * float4(other.group0.xyz, other.e321)) - (this.group0.yzxx * float4(other.group1.zxy, other.e235))
        );
    }
}
public extension AntiPlane {
    public AntiPlane wedge(AntiDualNum other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(other.scalar) * this.group0
        );
    }
}
public extension AntiPlane {
    public AntiDualNum wedge(AntiFlatPoint other) {
        return AntiDualNum(
            /* e3215, scalar */
            float2(-(other.e235 * this.e1) - (other.e315 * this.e2) - (other.e125 * this.e3) - (other.e321 * this.e5), 0.0)
        );
    }
}
public extension AntiPlane {
    public AntiMotor wedge(AntiFlector other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(((other.group1.zxy * this.group0.yzx) - (other.group1.yzx * this.group0.zxy)), 0.0), 
            /* e15, e25, e35, e3215 */
            float4(other.e5 * this.e1, other.e5 * this.e2, other.e5 * this.e3, -(other.e315 * this.e2) - (other.e125 * this.e3) - (other.e321 * this.e5)) - (this.group0.wwwx * float4(other.group1.xyz, other.e235))
        );
    }
}
public extension AntiPlane {
    public AntiFlatPoint wedge(AntiLine other) {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4((other.e23 * this.e5) + (other.e35 * this.e2), (other.e31 * this.e5) + (other.e15 * this.e3), (other.e12 * this.e5) + (other.e25 * this.e1), -(other.e31 * this.e2) - (other.e12 * this.e3)) - (this.group0.zxyx * float4(other.group1.xyz.yzx, other.e23))
        );
    }
}
public extension AntiPlane {
    public AntiFlector wedge(AntiMotor other) {
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4((other.e23 * this.e5) + (other.e35 * this.e2), (other.e31 * this.e5) + (other.e15 * this.e3), (other.e12 * this.e5) + (other.e25 * this.e1), -(other.e31 * this.e2) - (other.e12 * this.e3)) - (this.group0.zxyx * float4(other.group1.yzx, other.e23)), 
            /* e1, e2, e3, e5 */
            float4(other.scalar) * this.group0
        );
    }
}
public extension AntiPlane {
    public AntiLine wedge(AntiPlane other) {
        return AntiLine(
            /* e23, e31, e12 */
            (other.group0.zxy * this.group0.yzx) - (other.group0.yzx * this.group0.zxy), 
            /* e15, e25, e35 */
            (float3(other.e5) * this.group0.xyz) - (float3(this.e5) * other.group0.xyz)
        );
    }
}
public extension AntiPlane {
    public Sphere wedge(Circle other) {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(this.e3 * other.e425, this.e1 * other.e435, this.e2 * other.e415, -(this.e3 * other.e125) - (this.e5 * other.e321)) - (this.group0.wwwx * float4(other.group0.xyz, other.e235)) - float4((this.group0.yzx * other.group1.zxy), this.e2 * other.e315), 
            /* e1234 */
            (this.e1 * other.e423) + (this.e2 * other.e431) + (this.e3 * other.e412)
        );
    }
}
public extension AntiPlane {
    public Sphere wedge(CircleRotor other) {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(this.e3 * other.e425, this.e1 * other.e435, this.e2 * other.e415, -(this.e3 * other.e125) - (this.e5 * other.e321)) - (this.group0.yzxy * float4(other.group1.zxy, other.e315)) - float4((other.group0.xyz * this.group0.www), this.e1 * other.e235), 
            /* e1234 */
            (this.e1 * other.e423) + (this.e2 * other.e431) + (this.e3 * other.e412)
        );
    }
}
public extension AntiPlane {
    public Circle wedge(Dipole other) {
        return Circle(
            /* e423, e431, e412 */
            (other.group0.xyz.yzx * this.group0.zxy) - (other.group0.xyz.zxy * this.group0.yzx), 
            /* e415, e425, e435, e321 */
            float4(this.e5 * other.e41, this.e5 * other.e42, this.e5 * other.e43, -(this.e2 * other.e31) - (this.e3 * other.e12)) - (this.group0.xyzx * other.group1.wwwx), 
            /* e235, e315, e125 */
            (float3(this.e5) * other.group1.xyz) + (other.group2.xyz.zxy * this.group0.yzx) - (other.group2.xyz.yzx * this.group0.zxy)
        );
    }
}
public extension AntiPlane {
    public CircleRotor wedge(DipoleInversion other) {
        return CircleRotor(
            /* e423, e431, e412 */
            (other.group0.xyz.yzx * this.group0.zxy) - (other.group0.xyz.zxy * this.group0.yzx), 
            /* e415, e425, e435, e321 */
            float4(this.e5 * other.e41, this.e5 * other.e42, this.e5 * other.e43, -(this.e2 * other.e31) - (this.e3 * other.e12)) - (this.group0.xyzx * other.group1.wwwx), 
            /* e235, e315, e125, e12345 */
            float4(this.e3 * other.e25 * -1.0, this.e1 * other.e35 * -1.0, this.e2 * other.e15 * -1.0, (this.e3 * other.e4125) + (this.e5 * other.e1234)) + (this.group0.yzxx * float4(other.group2.zxy, other.e4235)) + (this.group0.wwwy * float4(other.group1.xyz, other.e4315))
        );
    }
}
public extension AntiPlane {
    public FlatPoint wedge(DualNum other) {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(other.group0.xy.xx, other.e5, 0.0) * float4(float3(1.0), 0.0) * float4(this.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
public extension AntiPlane {
    public Line wedge(FlatPoint other) {
        return Line(
            /* e415, e425, e435 */
            float3(other.e45) * this.group0.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            (this.group0.yzx * other.group0.zxy) - (this.group0.zxy * other.group0.yzx)
        );
    }
}
public extension AntiPlane {
    public Motor wedge(Flector other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((float3(other.e45) * this.group0.xyz * float3(-1.0)), (this.e1 * other.e4235) + (this.e2 * other.e4315) + (this.e3 * other.e4125)), 
            /* e235, e315, e125, e5 */
            float4(((this.group0.yzx * other.group0.zxy) - (this.group0.zxy * other.group0.yzx)), 0.0)
        );
    }
}
public extension AntiPlane {
    public Plane wedge(Line other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(this.e3 * other.e425, this.e1 * other.e435, this.e2 * other.e415, -(this.e2 * other.e315) - (this.e3 * other.e125)) - (this.group0.yzxx * float4(other.group0.xyz.zxy, other.e235))
        );
    }
}
public extension AntiPlane {
    public Flector wedge(Motor other) {
        return Flector(
            /* e15, e25, e35, e45 */
            float4(float3(1.0), 0.0) * float4(other.group1.www, 0.0) * float4(this.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e3 * other.e425, this.e1 * other.e435, this.e2 * other.e415, -(this.e2 * other.e315) - (this.e3 * other.e125)) - (this.group0.yzxx * float4(other.group0.zxy, other.e235))
        );
    }
}
public extension AntiPlane {
    public MultiVector wedge(MultiVector other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, (this.e1 * other.e4235) + (this.e2 * other.e4315) + (this.e3 * other.e4125) + (this.e5 * other.e1234)), 
            /* e1, e2, e3, e4 */
            float4(other.group0.xy.xx, other.scalar, 0.0) * float4(float3(1.0), 0.0) * float4(this.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e5 */
            this.e5 * other.scalar, 
            /* e15, e25, e35, e45 */
            float4(((float3(other.e5) * this.group0.xyz) - (float3(this.e5) * other.group1.xyz)), this.e5 * other.e4 * -1.0), 
            /* e41, e42, e43 */
            float3(other.e4) * this.group0.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            (this.group0.yzx * other.group1.zxy) - (this.group0.zxy * other.group1.yzx), 
            /* e415, e425, e435, e321 */
            float4(this.e5 * other.e41, this.e5 * other.e42, this.e5 * other.e43, -(this.e2 * other.e31) - (this.e3 * other.e12)) - float4((other.group3.www * this.group0.xyz), this.e1 * other.e23), 
            /* e423, e431, e412 */
            (other.group4.xyz.yzx * this.group0.zxy) - (other.group4.xyz.zxy * this.group0.yzx), 
            /* e235, e315, e125 */
            (float3(this.e5) * other.group5.xyz) + (this.group0.yzx * other.group3.zxy) - (this.group0.zxy * other.group3.yzx), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e3 * other.e425, this.e1 * other.e435, this.e2 * other.e415, -(this.e3 * other.e125) - (this.e5 * other.e321)) - (this.group0.wwwx * float4(other.group7.xyz, other.e235)) - float4((this.group0.yzx * other.group6.zxy), this.e2 * other.e315), 
            /* e1234 */
            (this.e1 * other.e423) + (this.e2 * other.e431) + (this.e3 * other.e412)
        );
    }
}
public extension AntiPlane {
    public AntiScalar wedge(Plane other) {
        return AntiScalar(
            /* e12345 */
            (this.e1 * other.e4235) + (this.e2 * other.e4315) + (this.e3 * other.e4125)
        );
    }
}
public extension AntiPlane {
    public Dipole wedge(RoundPoint other) {
        return Dipole(
            /* e41, e42, e43 */
            float3(other.e4) * this.group0.xyz * float3(-1.0), 
            /* e23, e31, e12, e45 */
            float4(((this.group0.yzx * other.group0.zxy) - (this.group0.zxy * other.group0.yzx)), this.e5 * other.e4 * -1.0), 
            /* e15, e25, e35 */
            (float3(other.e5) * this.group0.xyz) - (float3(this.e5) * other.group0.xyz)
        );
    }
}
public extension AntiPlane {
    public AntiPlane wedge(Scalar other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(other.scalar) * this.group0
        );
    }
}
public extension AntiPlane {
    public AntiScalar wedge(Sphere other) {
        return AntiScalar(
            /* e12345 */
            (this.e1 * other.e4235) + (this.e2 * other.e4315) + (this.e3 * other.e4125) + (this.e5 * other.e1234)
        );
    }
}
public extension AntiPlane {
    public DipoleInversion wedge(VersorEven other) {
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(other.e4) * this.group0.xyz * float3(-1.0), 
            /* e23, e31, e12, e45 */
            float4(((this.group0.yzx * other.group3.zxy) - (this.group0.zxy * other.group3.yzx)), this.e5 * other.e4 * -1.0), 
            /* e15, e25, e35, e1234 */
            float4(this.e5 * other.e1 * -1.0, this.e5 * other.e2 * -1.0, this.e5 * other.e3 * -1.0, (this.e2 * other.e431) + (this.e3 * other.e412)) + (this.group0.xyzx * float4(other.group2.www, other.e423)), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e3 * other.e425, this.e1 * other.e435, this.e2 * other.e415, -(this.e3 * other.e125) - (this.e5 * other.e321)) - (this.group0.yzxx * float4(other.group1.zxy, other.e235)) - (this.group0.wwwy * float4(other.group0.xyz, other.e315))
        );
    }
}
public extension AntiPlane {
    public VersorEven wedge(VersorOdd other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(this.e2 * other.e43 * -1.0, this.e3 * other.e41 * -1.0, this.e1 * other.e42 * -1.0, (this.e2 * other.e4315) + (this.e3 * other.e4125) + (this.e5 * other.e1234)) + (this.group0.zxyx * float4(other.group0.yzx, other.e4235)), 
            /* e415, e425, e435, e321 */
            float4(this.e5 * other.e41, this.e5 * other.e42, this.e5 * other.e43, -(this.e2 * other.e31) - (this.e3 * other.e12)) - (this.group0.xyzx * other.group1.wwwx), 
            /* e235, e315, e125, e5 */
            float4(((float3(this.e5) * other.group1.xyz) + (this.group0.yzx * other.group2.zxy) - (this.group0.zxy * other.group2.yzx)), this.e5 * other.scalar), 
            /* e1, e2, e3, e4 */
            float4(float3(1.0), 0.0) * float4(other.group0.www, 0.0) * float4(this.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
public extension AntiScalar {
    // Fancy infix trick (first half)
    public WedgeInfixPartial<AntiScalar> operator /(WedgeInfix rhs) {
        return WedgeInfixPartial<AntiScalar>(this);
    }
}
public extension WedgeInfixPartial<AntiScalar> {
    // Fancy infix trick (second half)
    public AntiScalar operator /(AntiCircleRotor rhs) {
        return this.a.wedge(rhs);
    }
}
public extension AntiScalar {
    public AntiScalar wedge(AntiCircleRotor other) {
        return AntiScalar(
            /* e12345 */
            other.scalar * this.e12345
        );
    }
}
public extension AntiScalar {
    public AntiScalar wedge(AntiDualNum other) {
        return AntiScalar(
            /* e12345 */
            other.scalar * this.e12345
        );
    }
}
public extension AntiScalar {
    public AntiScalar wedge(AntiMotor other) {
        return AntiScalar(
            /* e12345 */
            other.scalar * this.e12345
        );
    }
}
public extension AntiScalar {
    public AntiScalar wedge(MultiVector other) {
        return AntiScalar(
            /* e12345 */
            this.e12345 * other.scalar
        );
    }
}
public extension AntiScalar {
    public AntiScalar wedge(Scalar other) {
        return AntiScalar(
            /* e12345 */
            this.e12345 * other.scalar
        );
    }
}
public extension AntiScalar {
    public AntiScalar wedge(VersorOdd other) {
        return AntiScalar(
            /* e12345 */
            this.e12345 * other.scalar
        );
    }
}
public extension Circle {
    // Fancy infix trick (first half)
    public WedgeInfixPartial<Circle> operator /(WedgeInfix rhs) {
        return WedgeInfixPartial<Circle>(this);
    }
}
public extension WedgeInfixPartial<Circle> {
    // Fancy infix trick (second half)
    public CircleRotor operator /(AntiCircleRotor rhs) {
        return this.a.wedge(rhs);
    }
}
public extension Circle {
    public CircleRotor wedge(AntiCircleRotor other) {
        return CircleRotor(
            /* e423, e431, e412 */
            float3(other.scalar) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(other.scalar) * this.group1, 
            /* e235, e315, e125, e12345 */
            float4((float3(other.scalar) * this.group2.xyz), -(other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e45 * this.e321) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412))
        );
    }
}
public extension Circle {
    public Sphere wedge(AntiDipoleInversion other) {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(-(other.e4 * this.e235) - (other.e3 * this.e425), -(other.e4 * this.e315) - (other.e1 * this.e435), -(other.e4 * this.e125) - (other.e2 * this.e415), (other.e3 * this.e125) + (other.e5 * this.e321)) + (other.group3.wwwx * float4(this.group0.xyz, this.e235)) + float4((other.group3.yzx * this.group1.zxy), other.e2 * this.e315), 
            /* e1234 */
            -(other.e4 * this.e321) - (other.e1 * this.e423) - (other.e2 * this.e431) - (other.e3 * this.e412)
        );
    }
}
public extension Circle {
    public Circle wedge(AntiDualNum other) {
        return Circle(
            /* e423, e431, e412 */
            float3(other.scalar) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(other.scalar) * this.group1, 
            /* e235, e315, e125 */
            float3(other.scalar) * this.group2.xyz
        );
    }
}
public extension Circle {
    public Sphere wedge(AntiFlector other) {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(other.e3 * this.e425 * -1.0, other.e1 * this.e435 * -1.0, other.e2 * this.e415 * -1.0, (other.e3 * this.e125) + (other.e5 * this.e321)) + (other.group1.wwwx * float4(this.group0.xyz, this.e235)) + float4((other.group1.yzx * this.group1.zxy), other.e2 * this.e315), 
            /* e1234 */
            -(other.e1 * this.e423) - (other.e2 * this.e431) - (other.e3 * this.e412)
        );
    }
}
public extension Circle {
    public AntiScalar wedge(AntiLine other) {
        return AntiScalar(
            /* e12345 */
            -(other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412)
        );
    }
}
public extension Circle {
    public CircleRotor wedge(AntiMotor other) {
        return CircleRotor(
            /* e423, e431, e412 */
            float3(other.scalar) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(other.scalar) * this.group1, 
            /* e235, e315, e125, e12345 */
            float4((float3(other.scalar) * this.group2.xyz), -(other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412))
        );
    }
}
public extension Circle {
    public Sphere wedge(AntiPlane other) {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(other.e3 * this.e425 * -1.0, other.e1 * this.e435 * -1.0, other.e2 * this.e415 * -1.0, (other.e3 * this.e125) + (other.e5 * this.e321)) + (other.group0.wwwx * float4(this.group0.xyz, this.e235)) + float4((other.group0.yzx * this.group1.zxy), other.e2 * this.e315), 
            /* e1234 */
            -(other.e1 * this.e423) - (other.e2 * this.e431) - (other.e3 * this.e412)
        );
    }
}
public extension Circle {
    public AntiScalar wedge(Dipole other) {
        return AntiScalar(
            /* e12345 */
            -(this.e423 * other.e15) - (this.e431 * other.e25) - (this.e412 * other.e35) - (this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12) - (this.e321 * other.e45) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43)
        );
    }
}
public extension Circle {
    public AntiScalar wedge(DipoleInversion other) {
        return AntiScalar(
            /* e12345 */
            -(this.e423 * other.e15) - (this.e431 * other.e25) - (this.e412 * other.e35) - (this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12) - (this.e321 * other.e45) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43)
        );
    }
}
public extension Circle {
    public Plane wedge(DualNum other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(other.e5) * float4(this.group0.xyz, this.e321)
        );
    }
}
public extension Circle {
    public AntiScalar wedge(FlatPoint other) {
        return AntiScalar(
            /* e12345 */
            -(this.e423 * other.e15) - (this.e431 * other.e25) - (this.e412 * other.e35) - (this.e321 * other.e45)
        );
    }
}
public extension Circle {
    public AntiScalar wedge(Flector other) {
        return AntiScalar(
            /* e12345 */
            -(this.e423 * other.e15) - (this.e431 * other.e25) - (this.e412 * other.e35) - (this.e321 * other.e45)
        );
    }
}
public extension Circle {
    public Plane wedge(Motor other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(other.e5) * float4(this.group0.xyz, this.e321)
        );
    }
}
public extension Circle {
    public MultiVector wedge(MultiVector other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, -(this.e423 * other.e15) - (this.e431 * other.e25) - (this.e412 * other.e35) - (this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12) - (this.e321 * other.e45) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43)), 
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
            float4(other.scalar) * this.group1, 
            /* e423, e431, e412 */
            float3(other.scalar) * this.group0.xyz, 
            /* e235, e315, e125 */
            float3(other.scalar) * this.group2.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(-(this.e425 * other.e3) - (this.e235 * other.e4), -(this.e435 * other.e1) - (this.e315 * other.e4), -(this.e415 * other.e2) - (this.e125 * other.e4), (this.e321 * other.e5) + (this.e125 * other.e3)) + (other.group1.yzxy * float4(this.group1.zxy, this.e315)) + float4((float3(other.e5) * this.group0.xyz), this.e235 * other.e1), 
            /* e1234 */
            -(this.e423 * other.e1) - (this.e431 * other.e2) - (this.e412 * other.e3) - (this.e321 * other.e4)
        );
    }
}
public extension Circle {
    public Sphere wedge(RoundPoint other) {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(-(this.e425 * other.e3) - (this.e235 * other.e4), -(this.e435 * other.e1) - (this.e315 * other.e4), -(this.e415 * other.e2) - (this.e125 * other.e4), (this.e321 * other.e5) + (this.e125 * other.e3)) + (other.group0.yzxy * float4(this.group1.zxy, this.e315)) + float4((float3(other.e5) * this.group0.xyz), this.e235 * other.e1), 
            /* e1234 */
            -(this.e423 * other.e1) - (this.e431 * other.e2) - (this.e412 * other.e3) - (this.e321 * other.e4)
        );
    }
}
public extension Circle {
    public Circle wedge(Scalar other) {
        return Circle(
            /* e423, e431, e412 */
            float3(other.scalar) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(other.scalar) * this.group1, 
            /* e235, e315, e125 */
            float3(other.scalar) * this.group2.xyz
        );
    }
}
public extension Circle {
    public Sphere wedge(VersorEven other) {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(-(this.e425 * other.e3) - (this.e235 * other.e4), -(this.e435 * other.e1) - (this.e315 * other.e4), -(this.e415 * other.e2) - (this.e125 * other.e4), (this.e321 * other.e5) + (this.e125 * other.e3)) + (other.group3.yzxy * float4(this.group1.zxy, this.e315)) + float4((this.group0.xyz * other.group2.www), this.e235 * other.e1), 
            /* e1234 */
            -(this.e423 * other.e1) - (this.e431 * other.e2) - (this.e412 * other.e3) - (this.e321 * other.e4)
        );
    }
}
public extension Circle {
    public CircleRotor wedge(VersorOdd other) {
        return CircleRotor(
            /* e423, e431, e412 */
            float3(other.scalar) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(other.scalar) * this.group1, 
            /* e235, e315, e125, e12345 */
            float4((float3(other.scalar) * this.group2.xyz), -(this.e423 * other.e15) - (this.e431 * other.e25) - (this.e412 * other.e35) - (this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12) - (this.e321 * other.e45) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43))
        );
    }
}
public extension CircleRotor {
    // Fancy infix trick (first half)
    public WedgeInfixPartial<CircleRotor> operator /(WedgeInfix rhs) {
        return WedgeInfixPartial<CircleRotor>(this);
    }
}
public extension WedgeInfixPartial<CircleRotor> {
    // Fancy infix trick (second half)
    public CircleRotor operator /(AntiCircleRotor rhs) {
        return this.a.wedge(rhs);
    }
}
public extension CircleRotor {
    public CircleRotor wedge(AntiCircleRotor other) {
        return CircleRotor(
            /* e423, e431, e412 */
            float3(other.scalar) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(other.scalar) * this.group1, 
            /* e235, e315, e125, e12345 */
            float4((other.group2.www * this.group2.xyz), (other.scalar * this.e12345) - (other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e45 * this.e321) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412))
        );
    }
}
public extension CircleRotor {
    public Sphere wedge(AntiDipoleInversion other) {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(-(other.e4 * this.e235) - (other.e3 * this.e425), -(other.e4 * this.e315) - (other.e1 * this.e435), -(other.e4 * this.e125) - (other.e2 * this.e415), (other.e3 * this.e125) + (other.e5 * this.e321)) + (other.group3.yzxy * float4(this.group1.zxy, this.e315)) + float4((this.group0.xyz * other.group3.www), other.e1 * this.e235), 
            /* e1234 */
            -(other.e4 * this.e321) - (other.e1 * this.e423) - (other.e2 * this.e431) - (other.e3 * this.e412)
        );
    }
}
public extension CircleRotor {
    public CircleRotor wedge(AntiDualNum other) {
        return CircleRotor(
            /* e423, e431, e412 */
            float3(other.scalar) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(other.scalar) * this.group1, 
            /* e235, e315, e125, e12345 */
            float4(other.scalar) * this.group2
        );
    }
}
public extension CircleRotor {
    public Sphere wedge(AntiFlector other) {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(other.e3 * this.e425 * -1.0, other.e1 * this.e435 * -1.0, other.e2 * this.e415 * -1.0, (other.e3 * this.e125) + (other.e5 * this.e321)) + (other.group1.yzxy * float4(this.group1.zxy, this.e315)) + float4((this.group0.xyz * other.group1.www), other.e1 * this.e235), 
            /* e1234 */
            -(other.e1 * this.e423) - (other.e2 * this.e431) - (other.e3 * this.e412)
        );
    }
}
public extension CircleRotor {
    public AntiScalar wedge(AntiLine other) {
        return AntiScalar(
            /* e12345 */
            -(other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412)
        );
    }
}
public extension CircleRotor {
    public CircleRotor wedge(AntiMotor other) {
        return CircleRotor(
            /* e423, e431, e412 */
            float3(other.scalar) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(other.scalar) * this.group1, 
            /* e235, e315, e125, e12345 */
            float4((other.group0.www * this.group2.xyz), (other.scalar * this.e12345) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412))
        );
    }
}
public extension CircleRotor {
    public Sphere wedge(AntiPlane other) {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(other.e3 * this.e425 * -1.0, other.e1 * this.e435 * -1.0, other.e2 * this.e415 * -1.0, (other.e3 * this.e125) + (other.e5 * this.e321)) + (other.group0.yzxy * float4(this.group1.zxy, this.e315)) + float4((this.group0.xyz * other.group0.www), other.e1 * this.e235), 
            /* e1234 */
            -(other.e1 * this.e423) - (other.e2 * this.e431) - (other.e3 * this.e412)
        );
    }
}
public extension CircleRotor {
    public AntiScalar wedge(Dipole other) {
        return AntiScalar(
            /* e12345 */
            -(this.e423 * other.e15) - (this.e431 * other.e25) - (this.e412 * other.e35) - (this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12) - (this.e321 * other.e45) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43)
        );
    }
}
public extension CircleRotor {
    public AntiScalar wedge(DipoleInversion other) {
        return AntiScalar(
            /* e12345 */
            -(this.e423 * other.e15) - (this.e431 * other.e25) - (this.e412 * other.e35) - (this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12) - (this.e321 * other.e45) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43)
        );
    }
}
public extension CircleRotor {
    public Plane wedge(DualNum other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(other.e5) * float4(this.group0.xyz, this.e321)
        );
    }
}
public extension CircleRotor {
    public AntiScalar wedge(FlatPoint other) {
        return AntiScalar(
            /* e12345 */
            -(this.e423 * other.e15) - (this.e431 * other.e25) - (this.e412 * other.e35) - (this.e321 * other.e45)
        );
    }
}
public extension CircleRotor {
    public AntiScalar wedge(Flector other) {
        return AntiScalar(
            /* e12345 */
            -(this.e423 * other.e15) - (this.e431 * other.e25) - (this.e412 * other.e35) - (this.e321 * other.e45)
        );
    }
}
public extension CircleRotor {
    public Plane wedge(Motor other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(other.e5) * float4(this.group0.xyz, this.e321)
        );
    }
}
public extension CircleRotor {
    public MultiVector wedge(MultiVector other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, (this.e12345 * other.scalar) - (this.e423 * other.e15) - (this.e431 * other.e25) - (this.e412 * other.e35) - (this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12) - (this.e321 * other.e45) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43)), 
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
            float4(other.scalar) * this.group1, 
            /* e423, e431, e412 */
            float3(other.scalar) * this.group0.xyz, 
            /* e235, e315, e125 */
            float3(other.scalar) * this.group2.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(-(this.e425 * other.e3) - (this.e235 * other.e4), -(this.e435 * other.e1) - (this.e315 * other.e4), -(this.e415 * other.e2) - (this.e125 * other.e4), (this.e315 * other.e2) + (this.e125 * other.e3)) + (float4(other.e5) * float4(this.group0.xyz, this.e321)) + (other.group1.yzxx * float4(this.group1.zxy, this.e235)), 
            /* e1234 */
            -(this.e423 * other.e1) - (this.e431 * other.e2) - (this.e412 * other.e3) - (this.e321 * other.e4)
        );
    }
}
public extension CircleRotor {
    public Sphere wedge(RoundPoint other) {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(-(this.e425 * other.e3) - (this.e235 * other.e4), -(this.e435 * other.e1) - (this.e315 * other.e4), -(this.e415 * other.e2) - (this.e125 * other.e4), (this.e315 * other.e2) + (this.e125 * other.e3)) + (float4(other.e5) * float4(this.group0.xyz, this.e321)) + (other.group0.yzxx * float4(this.group1.zxy, this.e235)), 
            /* e1234 */
            -(this.e423 * other.e1) - (this.e431 * other.e2) - (this.e412 * other.e3) - (this.e321 * other.e4)
        );
    }
}
public extension CircleRotor {
    public CircleRotor wedge(Scalar other) {
        return CircleRotor(
            /* e423, e431, e412 */
            float3(other.scalar) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(other.scalar) * this.group1, 
            /* e235, e315, e125, e12345 */
            float4(other.scalar) * this.group2
        );
    }
}
public extension CircleRotor {
    public Sphere wedge(VersorEven other) {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(-(this.e425 * other.e3) - (this.e235 * other.e4), -(this.e435 * other.e1) - (this.e315 * other.e4), -(this.e415 * other.e2) - (this.e125 * other.e4), (this.e315 * other.e2) + (this.e125 * other.e3)) + (float4(other.e5) * float4(this.group0.xyz, this.e321)) + (other.group3.yzxx * float4(this.group1.zxy, this.e235)), 
            /* e1234 */
            -(this.e423 * other.e1) - (this.e431 * other.e2) - (this.e412 * other.e3) - (this.e321 * other.e4)
        );
    }
}
public extension CircleRotor {
    public CircleRotor wedge(VersorOdd other) {
        return CircleRotor(
            /* e423, e431, e412 */
            float3(other.scalar) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(other.scalar) * this.group1, 
            /* e235, e315, e125, e12345 */
            float4((float3(other.scalar) * this.group2.xyz), (this.e12345 * other.scalar) - (this.e423 * other.e15) - (this.e431 * other.e25) - (this.e412 * other.e35) - (this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12) - (this.e321 * other.e45) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43))
        );
    }
}
public extension Dipole {
    // Fancy infix trick (first half)
    public WedgeInfixPartial<Dipole> operator /(WedgeInfix rhs) {
        return WedgeInfixPartial<Dipole>(this);
    }
}
public extension WedgeInfixPartial<Dipole> {
    // Fancy infix trick (second half)
    public DipoleInversion operator /(AntiCircleRotor rhs) {
        return this.a.wedge(rhs);
    }
}
public extension Dipole {
    public DipoleInversion wedge(AntiCircleRotor other) {
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(other.scalar) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(other.scalar) * this.group1, 
            /* e15, e25, e35, e1234 */
            float4((float3(other.scalar) * this.group2.xyz), -(other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43)), 
            /* e4235, e4315, e4125, e3215 */
            float4((other.e42 * this.e35) + (other.e23 * this.e45) + (other.e45 * this.e23) + (other.e35 * this.e42), (other.e43 * this.e15) + (other.e31 * this.e45) + (other.e45 * this.e31) + (other.e15 * this.e43), (other.e41 * this.e25) + (other.e12 * this.e45) + (other.e45 * this.e12) + (other.e25 * this.e41), -(other.e12 * this.e35) - (other.e15 * this.e23) - (other.e25 * this.e31) - (other.e35 * this.e12)) - float4((other.group0.xyz.zxy * this.group2.xyz.yzx), other.e23 * this.e15) - float4((this.group0.xyz.zxy * other.group2.yzx), other.e31 * this.e25)
        );
    }
}
public extension Dipole {
    public CircleRotor wedge(AntiDipoleInversion other) {
        return CircleRotor(
            /* e423, e431, e412 */
            (float3(other.e4) * this.group1.xyz) + (this.group0.xyz.yzx * other.group3.zxy) - (this.group0.xyz.zxy * other.group3.yzx), 
            /* e415, e425, e435, e321 */
            float4((other.e4 * this.e15) + (other.e5 * this.e41), (other.e4 * this.e25) + (other.e5 * this.e42), (other.e4 * this.e35) + (other.e5 * this.e43), -(other.e2 * this.e31) - (other.e3 * this.e12)) - (other.group3.xyzx * this.group1.wwwx), 
            /* e235, e315, e125, e12345 */
            float4((other.e2 * this.e35) + (other.e5 * this.e23), (other.e3 * this.e15) + (other.e5 * this.e31), (other.e1 * this.e25) + (other.e5 * this.e12), -(other.e431 * this.e25) - (other.e412 * this.e35) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e321 * this.e45) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43)) - float4((this.group2.xyz.yzx * other.group3.zxy), other.e423 * this.e15)
        );
    }
}
public extension Dipole {
    public Dipole wedge(AntiDualNum other) {
        return Dipole(
            /* e41, e42, e43 */
            float3(other.scalar) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(other.scalar) * this.group1, 
            /* e15, e25, e35 */
            float3(other.scalar) * this.group2.xyz
        );
    }
}
public extension Dipole {
    public AntiScalar wedge(AntiFlatPoint other) {
        return AntiScalar(
            /* e12345 */
            -(other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43) - (other.e321 * this.e45)
        );
    }
}
public extension Dipole {
    public CircleRotor wedge(AntiFlector other) {
        return CircleRotor(
            /* e423, e431, e412 */
            (this.group0.xyz.yzx * other.group1.zxy) - (this.group0.xyz.zxy * other.group1.yzx), 
            /* e415, e425, e435, e321 */
            float4(other.e5 * this.e41, other.e5 * this.e42, other.e5 * this.e43, -(other.e2 * this.e31) - (other.e3 * this.e12)) - (other.group1.xyzx * this.group1.wwwx), 
            /* e235, e315, e125, e12345 */
            float4((other.e2 * this.e35) + (other.e5 * this.e23), (other.e3 * this.e15) + (other.e5 * this.e31), (other.e1 * this.e25) + (other.e5 * this.e12), -(other.e315 * this.e42) - (other.e125 * this.e43) - (other.e321 * this.e45)) - float4((this.group2.xyz.yzx * other.group1.zxy), other.e235 * this.e41)
        );
    }
}
public extension Dipole {
    public Sphere wedge(AntiLine other) {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((other.e23 * this.e45) + (other.e35 * this.e42), (other.e31 * this.e45) + (other.e15 * this.e43), (other.e12 * this.e45) + (other.e25 * this.e41), -(other.e31 * this.e25) - (other.e12 * this.e35) - (other.e15 * this.e23) - (other.e25 * this.e31) - (other.e35 * this.e12)) - float4((other.group1.xyz.yzx * this.group0.xyz.zxy), other.e23 * this.e15), 
            /* e1234 */
            -(other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43)
        );
    }
}
public extension Dipole {
    public DipoleInversion wedge(AntiMotor other) {
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(other.scalar) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(other.scalar) * this.group1, 
            /* e15, e25, e35, e1234 */
            float4((float3(other.scalar) * this.group2.xyz), -(other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43)), 
            /* e4235, e4315, e4125, e3215 */
            float4((other.e23 * this.e45) + (other.e35 * this.e42), (other.e31 * this.e45) + (other.e15 * this.e43), (other.e12 * this.e45) + (other.e25 * this.e41), -(other.e31 * this.e25) - (other.e12 * this.e35) - (other.e15 * this.e23) - (other.e25 * this.e31) - (other.e35 * this.e12)) - float4((this.group0.xyz.zxy * other.group1.yzx), other.e23 * this.e15)
        );
    }
}
public extension Dipole {
    public Circle wedge(AntiPlane other) {
        return Circle(
            /* e423, e431, e412 */
            (this.group0.xyz.yzx * other.group0.zxy) - (this.group0.xyz.zxy * other.group0.yzx), 
            /* e415, e425, e435, e321 */
            float4(other.e5 * this.e41, other.e5 * this.e42, other.e5 * this.e43, -(other.e2 * this.e31) - (other.e3 * this.e12)) - (other.group0.xyzx * this.group1.wwwx), 
            /* e235, e315, e125 */
            (float3(other.e5) * this.group1.xyz) + (this.group2.xyz.zxy * other.group0.yzx) - (this.group2.xyz.yzx * other.group0.zxy)
        );
    }
}
public extension Dipole {
    public AntiScalar wedge(Circle other) {
        return AntiScalar(
            /* e12345 */
            -(other.e423 * this.e15) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e321 * this.e45) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43)
        );
    }
}
public extension Dipole {
    public AntiScalar wedge(CircleRotor other) {
        return AntiScalar(
            /* e12345 */
            -(other.e423 * this.e15) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e321 * this.e45) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43)
        );
    }
}
public extension Dipole {
    public Sphere wedge(Dipole other) {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((other.e42 * this.e35) + (other.e23 * this.e45) + (other.e45 * this.e23) + (other.e35 * this.e42), (other.e43 * this.e15) + (other.e31 * this.e45) + (other.e45 * this.e31) + (other.e15 * this.e43), (other.e41 * this.e25) + (other.e12 * this.e45) + (other.e45 * this.e12) + (other.e25 * this.e41), -(other.e23 * this.e15) - (other.e31 * this.e25) - (other.e12 * this.e35) - (other.e35 * this.e12)) - float4((other.group0.xyz.zxy * this.group2.xyz.yzx), other.e15 * this.e23) - float4((other.group2.xyz.yzx * this.group0.xyz.zxy), other.e25 * this.e31), 
            /* e1234 */
            -(other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43)
        );
    }
}
public extension Dipole {
    public Sphere wedge(DipoleInversion other) {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((this.e42 * other.e35) + (this.e23 * other.e45) + (this.e45 * other.e23) + (this.e35 * other.e42), (this.e43 * other.e15) + (this.e31 * other.e45) + (this.e45 * other.e31) + (this.e15 * other.e43), (this.e41 * other.e25) + (this.e12 * other.e45) + (this.e45 * other.e12) + (this.e25 * other.e41), -(this.e23 * other.e15) - (this.e31 * other.e25) - (this.e12 * other.e35) - (this.e35 * other.e12)) - float4((this.group0.xyz.zxy * other.group2.yzx), this.e15 * other.e23) - float4((this.group2.xyz.yzx * other.group0.xyz.zxy), this.e25 * other.e31), 
            /* e1234 */
            -(this.e41 * other.e23) - (this.e42 * other.e31) - (this.e43 * other.e12) - (this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43)
        );
    }
}
public extension Dipole {
    public Line wedge(DualNum other) {
        return Line(
            /* e415, e425, e435 */
            float3(other.e5) * this.group0.xyz, 
            /* e235, e315, e125 */
            float3(other.e5) * this.group1.xyz
        );
    }
}
public extension Dipole {
    public Plane wedge(FlatPoint other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4((this.e42 * other.e35) + (this.e23 * other.e45), (this.e43 * other.e15) + (this.e31 * other.e45), (this.e41 * other.e25) + (this.e12 * other.e45), -(this.e31 * other.e25) - (this.e12 * other.e35)) - (other.group0.yzxx * float4(this.group0.xyz.zxy, this.e23))
        );
    }
}
public extension Dipole {
    public Plane wedge(Flector other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4((this.e42 * other.e35) + (this.e23 * other.e45), (this.e43 * other.e15) + (this.e31 * other.e45), (this.e41 * other.e25) + (this.e12 * other.e45), -(this.e31 * other.e25) - (this.e12 * other.e35)) - (other.group0.yzxx * float4(this.group0.xyz.zxy, this.e23))
        );
    }
}
public extension Dipole {
    public AntiScalar wedge(Line other) {
        return AntiScalar(
            /* e12345 */
            -(this.e41 * other.e235) - (this.e42 * other.e315) - (this.e43 * other.e125) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435)
        );
    }
}
public extension Dipole {
    public Motor wedge(Motor other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((float3(other.e5) * this.group0.xyz), -(this.e41 * other.e235) - (this.e42 * other.e315) - (this.e43 * other.e125) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435)), 
            /* e235, e315, e125, e5 */
            float4(float3(1.0), 0.0) * float4(other.group1.www, 0.0) * float4(this.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
public extension Dipole {
    public MultiVector wedge(MultiVector other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, -(this.e41 * other.e235) - (this.e42 * other.e315) - (this.e43 * other.e125) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435) - (this.e45 * other.e321) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            float4(other.scalar) * float4(this.group2.xyz, this.e45), 
            /* e41, e42, e43 */
            float3(other.scalar) * this.group0.xyz, 
            /* e23, e31, e12 */
            float3(other.scalar) * this.group1.xyz, 
            /* e415, e425, e435, e321 */
            float4((this.e41 * other.e5) + (this.e15 * other.e4), (this.e42 * other.e5) + (this.e25 * other.e4), (this.e43 * other.e5) + (this.e35 * other.e4), -(this.e31 * other.e2) - (this.e12 * other.e3)) - (this.group1.wwwx * other.group1.xyzx), 
            /* e423, e431, e412 */
            (float3(other.e4) * this.group1.xyz) + (this.group0.xyz.yzx * other.group1.zxy) - (this.group0.xyz.zxy * other.group1.yzx), 
            /* e235, e315, e125 */
            (float3(other.e5) * this.group1.xyz) + (this.group2.xyz.zxy * other.group1.yzx) - (this.group2.xyz.yzx * other.group1.zxy), 
            /* e4235, e4315, e4125, e3215 */
            float4((this.e42 * other.e35) + (this.e23 * other.e45) + (this.e45 * other.e23) + (this.e35 * other.e42), (this.e43 * other.e15) + (this.e31 * other.e45) + (this.e45 * other.e31) + (this.e15 * other.e43), (this.e41 * other.e25) + (this.e12 * other.e45) + (this.e45 * other.e12) + (this.e25 * other.e41), -(this.e23 * other.e15) - (this.e31 * other.e25) - (this.e12 * other.e35) - (this.e35 * other.e12)) - float4((this.group0.xyz.zxy * other.group3.yzx), this.e15 * other.e23) - float4((this.group2.xyz.yzx * other.group4.xyz.zxy), this.e25 * other.e31), 
            /* e1234 */
            -(this.e41 * other.e23) - (this.e42 * other.e31) - (this.e43 * other.e12) - (this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43)
        );
    }
}
public extension Dipole {
    public Circle wedge(RoundPoint other) {
        return Circle(
            /* e423, e431, e412 */
            (float3(other.e4) * this.group1.xyz) + (this.group0.xyz.yzx * other.group0.zxy) - (this.group0.xyz.zxy * other.group0.yzx), 
            /* e415, e425, e435, e321 */
            float4((this.e41 * other.e5) + (this.e15 * other.e4), (this.e42 * other.e5) + (this.e25 * other.e4), (this.e43 * other.e5) + (this.e35 * other.e4), -(this.e31 * other.e2) - (this.e12 * other.e3)) - (this.group1.wwwx * other.group0.xyzx), 
            /* e235, e315, e125 */
            (float3(other.e5) * this.group1.xyz) + (this.group2.xyz.zxy * other.group0.yzx) - (this.group2.xyz.yzx * other.group0.zxy)
        );
    }
}
public extension Dipole {
    public Dipole wedge(Scalar other) {
        return Dipole(
            /* e41, e42, e43 */
            float3(other.scalar) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(other.scalar) * this.group1, 
            /* e15, e25, e35 */
            float3(other.scalar) * this.group2.xyz
        );
    }
}
public extension Dipole {
    public CircleRotor wedge(VersorEven other) {
        return CircleRotor(
            /* e423, e431, e412 */
            (float3(other.e4) * this.group1.xyz) + (this.group0.xyz.yzx * other.group3.zxy) - (this.group0.xyz.zxy * other.group3.yzx), 
            /* e415, e425, e435, e321 */
            float4((this.e41 * other.e5) + (this.e15 * other.e4), (this.e42 * other.e5) + (this.e25 * other.e4), (this.e43 * other.e5) + (this.e35 * other.e4), -(this.e31 * other.e2) - (this.e12 * other.e3)) - (this.group1.wwwx * other.group3.xyzx), 
            /* e235, e315, e125, e12345 */
            float4((this.e23 * other.e5) + (this.e35 * other.e2), (this.e31 * other.e5) + (this.e15 * other.e3), (this.e12 * other.e5) + (this.e25 * other.e1), -(this.e42 * other.e315) - (this.e43 * other.e125) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435) - (this.e45 * other.e321) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412)) - float4((this.group2.xyz.yzx * other.group3.zxy), this.e41 * other.e235)
        );
    }
}
public extension Dipole {
    public DipoleInversion wedge(VersorOdd other) {
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(other.scalar) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(other.scalar) * this.group1, 
            /* e15, e25, e35, e1234 */
            float4((float3(other.scalar) * this.group2.xyz), -(this.e41 * other.e23) - (this.e42 * other.e31) - (this.e43 * other.e12) - (this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43)), 
            /* e4235, e4315, e4125, e3215 */
            float4((this.e42 * other.e35) + (this.e23 * other.e45) + (this.e45 * other.e23) + (this.e35 * other.e42), (this.e43 * other.e15) + (this.e31 * other.e45) + (this.e45 * other.e31) + (this.e15 * other.e43), (this.e41 * other.e25) + (this.e12 * other.e45) + (this.e45 * other.e12) + (this.e25 * other.e41), -(this.e23 * other.e15) - (this.e31 * other.e25) - (this.e12 * other.e35) - (this.e35 * other.e12)) - float4((this.group0.xyz.zxy * other.group2.yzx), this.e15 * other.e23) - float4((this.group2.xyz.yzx * other.group0.zxy), this.e25 * other.e31)
        );
    }
}
public extension DipoleInversion {
    // Fancy infix trick (first half)
    public WedgeInfixPartial<DipoleInversion> operator /(WedgeInfix rhs) {
        return WedgeInfixPartial<DipoleInversion>(this);
    }
}
public extension WedgeInfixPartial<DipoleInversion> {
    // Fancy infix trick (second half)
    public DipoleInversion operator /(AntiCircleRotor rhs) {
        return this.a.wedge(rhs);
    }
}
public extension DipoleInversion {
    public DipoleInversion wedge(AntiCircleRotor other) {
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(other.scalar) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(other.scalar) * this.group1, 
            /* e15, e25, e35, e1234 */
            float4((other.group2.www * this.group2.xyz), (other.scalar * this.e1234) - (other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43)), 
            /* e4235, e4315, e4125, e3215 */
            float4((other.e23 * this.e45) + (other.e45 * this.e23) + (other.e35 * this.e42) + (other.scalar * this.e4235), (other.e31 * this.e45) + (other.e45 * this.e31) + (other.e15 * this.e43) + (other.scalar * this.e4315), (other.e12 * this.e45) + (other.e45 * this.e12) + (other.e25 * this.e41) + (other.scalar * this.e4125), -(other.e12 * this.e35) - (other.e15 * this.e23) - (other.e25 * this.e31) - (other.e35 * this.e12)) + float4((other.group0.xyz.yzx * this.group2.zxy), other.scalar * this.e3215) - (this.group2.yzxx * float4(other.group0.xyz.zxy, other.e23)) - float4((this.group0.xyz.zxy * other.group2.yzx), other.e31 * this.e25)
        );
    }
}
public extension DipoleInversion {
    public CircleRotor wedge(AntiDipoleInversion other) {
        return CircleRotor(
            /* e423, e431, e412 */
            (float3(other.e4) * this.group1.xyz) + (this.group0.xyz.yzx * other.group3.zxy) - (this.group0.xyz.zxy * other.group3.yzx), 
            /* e415, e425, e435, e321 */
            float4((other.e4 * this.e15) + (other.e5 * this.e41), (other.e4 * this.e25) + (other.e5 * this.e42), (other.e4 * this.e35) + (other.e5 * this.e43), -(other.e2 * this.e31) - (other.e3 * this.e12)) - (other.group3.xyzx * this.group1.wwwx), 
            /* e235, e315, e125, e12345 */
            (other.group3.wwwx * float4(this.group1.xyz, this.e4235)) + float4(float3(0.0), (other.e2 * this.e4315) + (other.e3 * this.e4125) + (other.e5 * this.e1234) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e321 * this.e45) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43)) + float4((other.group3.yzx * this.group2.zxy), other.e4 * this.e3215) - (this.group2.yzxx * float4(other.group3.zxy, other.e423))
        );
    }
}
public extension DipoleInversion {
    public DipoleInversion wedge(AntiDualNum other) {
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(other.scalar) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(other.scalar) * this.group1, 
            /* e15, e25, e35, e1234 */
            float4(other.scalar) * this.group2, 
            /* e4235, e4315, e4125, e3215 */
            float4(other.scalar) * this.group3
        );
    }
}
public extension DipoleInversion {
    public AntiScalar wedge(AntiFlatPoint other) {
        return AntiScalar(
            /* e12345 */
            -(other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43) - (other.e321 * this.e45)
        );
    }
}
public extension DipoleInversion {
    public CircleRotor wedge(AntiFlector other) {
        return CircleRotor(
            /* e423, e431, e412 */
            (this.group0.xyz.yzx * other.group1.zxy) - (this.group0.xyz.zxy * other.group1.yzx), 
            /* e415, e425, e435, e321 */
            float4(other.e5 * this.e41, other.e5 * this.e42, other.e5 * this.e43, -(other.e2 * this.e31) - (other.e3 * this.e12)) - (other.group1.xyzx * this.group1.wwwx), 
            /* e235, e315, e125, e12345 */
            (other.group1.yzxx * float4(this.group2.zxy, this.e4235)) + (other.group1.wwwy * float4(this.group1.xyz, this.e4315)) + float4(float3(0.0), (other.e3 * this.e4125) + (other.e5 * this.e1234) - (other.e315 * this.e42) - (other.e125 * this.e43) - (other.e321 * this.e45)) - float4((other.group1.zxy * this.group2.yzx), other.e235 * this.e41)
        );
    }
}
public extension DipoleInversion {
    public Sphere wedge(AntiLine other) {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((other.e23 * this.e45) + (other.e35 * this.e42), (other.e31 * this.e45) + (other.e15 * this.e43), (other.e12 * this.e45) + (other.e25 * this.e41), -(other.e31 * this.e25) - (other.e12 * this.e35) - (other.e15 * this.e23) - (other.e25 * this.e31) - (other.e35 * this.e12)) - float4((other.group1.xyz.yzx * this.group0.xyz.zxy), other.e23 * this.e15), 
            /* e1234 */
            -(other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43)
        );
    }
}
public extension DipoleInversion {
    public DipoleInversion wedge(AntiMotor other) {
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(other.scalar) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(other.scalar) * this.group1, 
            /* e15, e25, e35, e1234 */
            float4((other.group0.www * this.group2.xyz), (other.scalar * this.e1234) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43)), 
            /* e4235, e4315, e4125, e3215 */
            float4((other.e23 * this.e45) + (other.scalar * this.e4235), (other.e31 * this.e45) + (other.scalar * this.e4315), (other.e12 * this.e45) + (other.scalar * this.e4125), -(other.e31 * this.e25) - (other.e12 * this.e35) - (other.e15 * this.e23) - (other.e25 * this.e31) - (other.e35 * this.e12)) + float4((this.group0.xyz.yzx * other.group1.zxy), other.scalar * this.e3215) - float4((this.group0.xyz.zxy * other.group1.yzx), other.e23 * this.e15)
        );
    }
}
public extension DipoleInversion {
    public CircleRotor wedge(AntiPlane other) {
        return CircleRotor(
            /* e423, e431, e412 */
            (this.group0.xyz.yzx * other.group0.zxy) - (this.group0.xyz.zxy * other.group0.yzx), 
            /* e415, e425, e435, e321 */
            float4(other.e5 * this.e41, other.e5 * this.e42, other.e5 * this.e43, -(other.e2 * this.e31) - (other.e3 * this.e12)) - (other.group0.xyzx * this.group1.wwwx), 
            /* e235, e315, e125, e12345 */
            float4(other.e3 * this.e25 * -1.0, other.e1 * this.e35 * -1.0, other.e2 * this.e15 * -1.0, (other.e3 * this.e4125) + (other.e5 * this.e1234)) + (other.group0.yzxx * float4(this.group2.zxy, this.e4235)) + (other.group0.wwwy * float4(this.group1.xyz, this.e4315))
        );
    }
}
public extension DipoleInversion {
    public AntiScalar wedge(Circle other) {
        return AntiScalar(
            /* e12345 */
            -(other.e423 * this.e15) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e321 * this.e45) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43)
        );
    }
}
public extension DipoleInversion {
    public AntiScalar wedge(CircleRotor other) {
        return AntiScalar(
            /* e12345 */
            -(other.e423 * this.e15) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e321 * this.e45) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43)
        );
    }
}
public extension DipoleInversion {
    public Sphere wedge(Dipole other) {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((other.e42 * this.e35) + (other.e23 * this.e45) + (other.e45 * this.e23) + (other.e35 * this.e42), (other.e43 * this.e15) + (other.e31 * this.e45) + (other.e45 * this.e31) + (other.e15 * this.e43), (other.e41 * this.e25) + (other.e12 * this.e45) + (other.e45 * this.e12) + (other.e25 * this.e41), -(other.e23 * this.e15) - (other.e31 * this.e25) - (other.e12 * this.e35) - (other.e35 * this.e12)) - float4((other.group0.xyz.zxy * this.group2.yzx), other.e15 * this.e23) - float4((other.group2.xyz.yzx * this.group0.xyz.zxy), other.e25 * this.e31), 
            /* e1234 */
            -(other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43)
        );
    }
}
public extension DipoleInversion {
    public Sphere wedge(DipoleInversion other) {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((other.e42 * this.e35) + (other.e23 * this.e45) + (other.e45 * this.e23) + (other.e35 * this.e42), (other.e43 * this.e15) + (other.e31 * this.e45) + (other.e45 * this.e31) + (other.e15 * this.e43), (other.e41 * this.e25) + (other.e12 * this.e45) + (other.e45 * this.e12) + (other.e25 * this.e41), -(other.e12 * this.e35) - (other.e15 * this.e23) - (other.e25 * this.e31) - (other.e35 * this.e12)) - (this.group2.yzxx * float4(other.group0.xyz.zxy, other.e23)) - float4((this.group0.xyz.zxy * other.group2.yzx), other.e31 * this.e25), 
            /* e1234 */
            -(other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43)
        );
    }
}
public extension DipoleInversion {
    public Motor wedge(DualNum other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(other.e5) * float4(this.group0.xyz, this.e1234), 
            /* e235, e315, e125, e5 */
            float4(other.group0.xy.xx, other.e5, 0.0) * float4(float3(1.0), 0.0) * float4(this.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
public extension DipoleInversion {
    public Plane wedge(FlatPoint other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4((this.e42 * other.e35) + (this.e23 * other.e45), (this.e43 * other.e15) + (this.e31 * other.e45), (this.e41 * other.e25) + (this.e12 * other.e45), -(this.e31 * other.e25) - (this.e12 * other.e35)) - (other.group0.yzxx * float4(this.group0.xyz.zxy, this.e23))
        );
    }
}
public extension DipoleInversion {
    public Plane wedge(Flector other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4((this.e42 * other.e35) + (this.e23 * other.e45), (this.e43 * other.e15) + (this.e31 * other.e45), (this.e41 * other.e25) + (this.e12 * other.e45), -(this.e31 * other.e25) - (this.e12 * other.e35)) - (other.group0.yzxx * float4(this.group0.xyz.zxy, this.e23))
        );
    }
}
public extension DipoleInversion {
    public AntiScalar wedge(Line other) {
        return AntiScalar(
            /* e12345 */
            -(this.e41 * other.e235) - (this.e42 * other.e315) - (this.e43 * other.e125) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435)
        );
    }
}
public extension DipoleInversion {
    public Motor wedge(Motor other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((float3(other.e5) * this.group0.xyz), (this.e1234 * other.e5) - (this.e41 * other.e235) - (this.e42 * other.e315) - (this.e43 * other.e125) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435)), 
            /* e235, e315, e125, e5 */
            float4(float3(1.0), 0.0) * float4(other.group1.www, 0.0) * float4(this.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
public extension DipoleInversion {
    public MultiVector wedge(MultiVector other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, (this.e1234 * other.e5) + (this.e4235 * other.e1) + (this.e4315 * other.e2) + (this.e4125 * other.e3) + (this.e3215 * other.e4) - (this.e41 * other.e235) - (this.e42 * other.e315) - (this.e43 * other.e125) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435) - (this.e45 * other.e321) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            float4(other.scalar) * float4(this.group2.xyz, this.e45), 
            /* e41, e42, e43 */
            float3(other.scalar) * this.group0.xyz, 
            /* e23, e31, e12 */
            float3(other.scalar) * this.group1.xyz, 
            /* e415, e425, e435, e321 */
            float4((this.e41 * other.e5) + (this.e15 * other.e4), (this.e42 * other.e5) + (this.e25 * other.e4), (this.e43 * other.e5) + (this.e35 * other.e4), -(this.e31 * other.e2) - (this.e12 * other.e3)) - (this.group1.wwwx * other.group1.xyzx), 
            /* e423, e431, e412 */
            (float3(other.e4) * this.group1.xyz) + (this.group0.xyz.yzx * other.group1.zxy) - (this.group0.xyz.zxy * other.group1.yzx), 
            /* e235, e315, e125 */
            (float3(other.e5) * this.group1.xyz) + (this.group2.zxy * other.group1.yzx) - (this.group2.yzx * other.group1.zxy), 
            /* e4235, e4315, e4125, e3215 */
            float4((this.e42 * other.e35) + (this.e23 * other.e45) + (this.e45 * other.e23) + (this.e35 * other.e42), (this.e43 * other.e15) + (this.e31 * other.e45) + (this.e45 * other.e31) + (this.e15 * other.e43), (this.e41 * other.e25) + (this.e12 * other.e45) + (this.e45 * other.e12) + (this.e25 * other.e41), -(this.e23 * other.e15) - (this.e31 * other.e25) - (this.e12 * other.e35) - (this.e35 * other.e12)) + (float4(other.scalar) * this.group3) - (this.group2.yzxy * float4(other.group4.xyz.zxy, other.e31)) - float4((this.group0.xyz.zxy * other.group3.yzx), this.e15 * other.e23), 
            /* e1234 */
            (this.e1234 * other.scalar) - (this.e41 * other.e23) - (this.e42 * other.e31) - (this.e43 * other.e12) - (this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43)
        );
    }
}
public extension DipoleInversion {
    public CircleRotor wedge(RoundPoint other) {
        return CircleRotor(
            /* e423, e431, e412 */
            (float3(other.e4) * this.group1.xyz) + (this.group0.xyz.yzx * other.group0.zxy) - (this.group0.xyz.zxy * other.group0.yzx), 
            /* e415, e425, e435, e321 */
            float4((this.e41 * other.e5) + (this.e15 * other.e4), (this.e42 * other.e5) + (this.e25 * other.e4), (this.e43 * other.e5) + (this.e35 * other.e4), -(this.e31 * other.e2) - (this.e12 * other.e3)) - (this.group1.wwwx * other.group0.xyzx), 
            /* e235, e315, e125, e12345 */
            float4(this.e25 * other.e3 * -1.0, this.e35 * other.e1 * -1.0, this.e15 * other.e2 * -1.0, (this.e4315 * other.e2) + (this.e4125 * other.e3) + (this.e3215 * other.e4)) + (float4(other.e5) * float4(this.group1.xyz, this.e1234)) + (other.group0.yzxx * float4(this.group2.zxy, this.e4235))
        );
    }
}
public extension DipoleInversion {
    public DipoleInversion wedge(Scalar other) {
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(other.scalar) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(other.scalar) * this.group1, 
            /* e15, e25, e35, e1234 */
            float4(other.scalar) * this.group2, 
            /* e4235, e4315, e4125, e3215 */
            float4(other.scalar) * this.group3
        );
    }
}
public extension DipoleInversion {
    public CircleRotor wedge(VersorEven other) {
        return CircleRotor(
            /* e423, e431, e412 */
            (float3(other.e4) * this.group1.xyz) + (this.group0.xyz.yzx * other.group3.zxy) - (this.group0.xyz.zxy * other.group3.yzx), 
            /* e415, e425, e435, e321 */
            float4((this.e41 * other.e5) + (this.e15 * other.e4), (this.e42 * other.e5) + (this.e25 * other.e4), (this.e43 * other.e5) + (this.e35 * other.e4), -(this.e31 * other.e2) - (this.e12 * other.e3)) - (this.group1.wwwx * other.group3.xyzx), 
            /* e235, e315, e125, e12345 */
            (float4(other.e5) * float4(this.group1.xyz, this.e1234)) + (other.group3.yzxx * float4(this.group2.zxy, this.e4235)) + float4(float3(0.0), (this.e4315 * other.e2) + (this.e4125 * other.e3) + (this.e3215 * other.e4) - (this.e42 * other.e315) - (this.e43 * other.e125) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435) - (this.e45 * other.e321) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412)) - float4((this.group2.yzx * other.group3.zxy), this.e41 * other.e235)
        );
    }
}
public extension DipoleInversion {
    public DipoleInversion wedge(VersorOdd other) {
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(other.scalar) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(other.scalar) * this.group1, 
            /* e15, e25, e35, e1234 */
            float4((float3(other.scalar) * this.group2.xyz), (this.e1234 * other.scalar) - (this.e41 * other.e23) - (this.e42 * other.e31) - (this.e43 * other.e12) - (this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43)), 
            /* e4235, e4315, e4125, e3215 */
            float4((this.e23 * other.e45) + (this.e45 * other.e23) + (this.e35 * other.e42) + (this.e4235 * other.scalar), (this.e31 * other.e45) + (this.e45 * other.e31) + (this.e15 * other.e43) + (this.e4315 * other.scalar), (this.e12 * other.e45) + (this.e45 * other.e12) + (this.e25 * other.e41) + (this.e4125 * other.scalar), -(this.e12 * other.e35) - (this.e15 * other.e23) - (this.e25 * other.e31) - (this.e35 * other.e12)) + float4((this.group0.xyz.yzx * other.group2.zxy), this.e3215 * other.scalar) - (other.group2.yzxx * float4(this.group0.xyz.zxy, this.e23)) - float4((this.group2.yzx * other.group0.zxy), this.e31 * other.e25)
        );
    }
}
public extension DualNum {
    // Fancy infix trick (first half)
    public WedgeInfixPartial<DualNum> operator /(WedgeInfix rhs) {
        return WedgeInfixPartial<DualNum>(this);
    }
}
public extension WedgeInfixPartial<DualNum> {
    // Fancy infix trick (second half)
    public Motor operator /(AntiCircleRotor rhs) {
        return this.a.wedge(rhs);
    }
}
public extension DualNum {
    public Motor wedge(AntiCircleRotor other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(this.group0.xy.xx, this.e5, this.e12345) * float4(other.group0.xyz, other.scalar), 
            /* e235, e315, e125, e5 */
            float4(this.e5) * float4(other.group1.xyz, other.scalar)
        );
    }
}
public extension DualNum {
    public Flector wedge(AntiDipoleInversion other) {
        return Flector(
            /* e15, e25, e35, e45 */
            float4(this.e5) * float4(other.group3.xyz, other.e4) * float4(-1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e5) * float4(other.group0.xyz, other.e321) * float4(-1.0)
        );
    }
}
public extension DualNum {
    public DualNum wedge(AntiDualNum other) {
        return DualNum(
            /* e5, e12345 */
            float2(other.scalar) * this.group0.xy
        );
    }
}
public extension DualNum {
    public AntiDualNum wedge(AntiFlatPoint other) {
        return AntiDualNum(
            /* e3215, scalar */
            float2(other.e321 * this.e5, 1.0) * float2(-1.0, 0.0)
        );
    }
}
public extension DualNum {
    public Flector wedge(AntiFlector other) {
        return Flector(
            /* e15, e25, e35, e45 */
            float4(this.group0.xy.xx, this.e5, 0.0) * float4(float3(1.0), 0.0) * float4(other.group1.xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), other.e321 * this.e5 * -1.0)
        );
    }
}
public extension DualNum {
    public AntiFlatPoint wedge(AntiLine other) {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(this.group0.xy.xx, this.e5, 0.0) * float4(float3(1.0), 0.0) * float4(other.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
public extension DualNum {
    public Motor wedge(AntiMotor other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(float3(0.0), other.scalar * this.e12345), 
            /* e235, e315, e125, e5 */
            float4(this.e5) * other.group0
        );
    }
}
public extension DualNum {
    public FlatPoint wedge(AntiPlane other) {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(this.group0.xy.xx, this.e5, 0.0) * float4(float3(1.0), 0.0) * float4(other.group0.xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0)
        );
    }
}
public extension DualNum {
    public Plane wedge(Circle other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(this.e5) * float4(other.group0.xyz, other.e321) * float4(-1.0)
        );
    }
}
public extension DualNum {
    public Plane wedge(CircleRotor other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(this.e5) * float4(other.group0.xyz, other.e321) * float4(-1.0)
        );
    }
}
public extension DualNum {
    public Line wedge(Dipole other) {
        return Line(
            /* e415, e425, e435 */
            float3(this.e5) * other.group0.xyz, 
            /* e235, e315, e125 */
            float3(this.e5) * other.group1.xyz
        );
    }
}
public extension DualNum {
    public Motor wedge(DipoleInversion other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(this.e5) * float4(other.group0.xyz, other.e1234), 
            /* e235, e315, e125, e5 */
            float4(this.group0.xy.xx, this.e5, 0.0) * float4(float3(1.0), 0.0) * float4(other.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
public extension DualNum {
    public MultiVector wedge(MultiVector other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, (this.e5 * other.e1234) + (this.e12345 * other.scalar)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            this.e5 * other.scalar, 
            /* e15, e25, e35, e45 */
            float4(this.e5) * other.group1 * float4(-1.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(this.group0.xy.xx, this.e5, 0.0) * float4(float3(1.0), 0.0) * float4(other.group4.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(this.e5) * other.group5.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e5) * float4(other.group7.xyz, other.e321) * float4(-1.0), 
            /* e1234 */
            0.0
        );
    }
}
public extension DualNum {
    public FlatPoint wedge(RoundPoint other) {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(this.e5) * other.group0 * float4(-1.0)
        );
    }
}
public extension DualNum {
    public DualNum wedge(Scalar other) {
        return DualNum(
            /* e5, e12345 */
            float2(other.scalar) * this.group0.xy
        );
    }
}
public extension DualNum {
    public AntiScalar wedge(Sphere other) {
        return AntiScalar(
            /* e12345 */
            this.e5 * other.e1234
        );
    }
}
public extension DualNum {
    public Flector wedge(VersorEven other) {
        return Flector(
            /* e15, e25, e35, e45 */
            float4(this.e5) * other.group3 * float4(-1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e5) * float4(other.group0.xyz, other.e321) * float4(-1.0)
        );
    }
}
public extension DualNum {
    public Motor wedge(VersorOdd other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(this.group0.xy.xx, this.e5, (this.e5 * other.e1234) + (this.e12345 * other.scalar)) * float4(other.group0.xyz, 1.0), 
            /* e235, e315, e125, e5 */
            float4(this.e5) * float4(other.group1.xyz, other.scalar)
        );
    }
}
public extension FlatPoint {
    // Fancy infix trick (first half)
    public WedgeInfixPartial<FlatPoint> operator /(WedgeInfix rhs) {
        return WedgeInfixPartial<FlatPoint>(this);
    }
}
public extension WedgeInfixPartial<FlatPoint> {
    // Fancy infix trick (second half)
    public Flector operator /(AntiCircleRotor rhs) {
        return this.a.wedge(rhs);
    }
}
public extension FlatPoint {
    public Flector wedge(AntiCircleRotor other) {
        return Flector(
            /* e15, e25, e35, e45 */
            float4(other.scalar) * this.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4((other.e42 * this.e35) + (other.e23 * this.e45), (other.e43 * this.e15) + (other.e31 * this.e45), (other.e41 * this.e25) + (other.e12 * this.e45), -(other.e31 * this.e25) - (other.e12 * this.e35)) - (this.group0.yzxx * float4(other.group0.xyz.zxy, other.e23))
        );
    }
}
public extension FlatPoint {
    public Motor wedge(AntiDipoleInversion other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(other.e4 * this.e15, other.e4 * this.e25, other.e4 * this.e35, -(other.e431 * this.e25) - (other.e412 * this.e35) - (other.e321 * this.e45)) - (this.group0.wwwx * float4(other.group3.xyz, other.e423)), 
            /* e235, e315, e125, e5 */
            float4(((other.group3.yzx * this.group0.zxy) - (other.group3.zxy * this.group0.yzx)), 0.0)
        );
    }
}
public extension FlatPoint {
    public FlatPoint wedge(AntiDualNum other) {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(other.scalar) * this.group0
        );
    }
}
public extension FlatPoint {
    public AntiScalar wedge(AntiFlatPoint other) {
        return AntiScalar(
            /* e12345 */
            other.e321 * this.e45 * -1.0
        );
    }
}
public extension FlatPoint {
    public Motor wedge(AntiFlector other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(this.e45) * float4(other.group1.xyz, other.e321) * float4(-1.0), 
            /* e235, e315, e125, e5 */
            float4(((other.group1.yzx * this.group0.zxy) - (other.group1.zxy * this.group0.yzx)), 0.0)
        );
    }
}
public extension FlatPoint {
    public Plane wedge(AntiLine other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4((float3(this.e45) * other.group0.xyz), -(other.e23 * this.e15) - (other.e31 * this.e25) - (other.e12 * this.e35))
        );
    }
}
public extension FlatPoint {
    public Flector wedge(AntiMotor other) {
        return Flector(
            /* e15, e25, e35, e45 */
            float4(other.scalar) * this.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4((float3(this.e45) * other.group0.xyz), -(other.e23 * this.e15) - (other.e31 * this.e25) - (other.e12 * this.e35))
        );
    }
}
public extension FlatPoint {
    public Line wedge(AntiPlane other) {
        return Line(
            /* e415, e425, e435 */
            float3(this.e45) * other.group0.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            (other.group0.yzx * this.group0.zxy) - (other.group0.zxy * this.group0.yzx)
        );
    }
}
public extension FlatPoint {
    public AntiScalar wedge(Circle other) {
        return AntiScalar(
            /* e12345 */
            -(other.e423 * this.e15) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e321 * this.e45)
        );
    }
}
public extension FlatPoint {
    public AntiScalar wedge(CircleRotor other) {
        return AntiScalar(
            /* e12345 */
            -(other.e423 * this.e15) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e321 * this.e45)
        );
    }
}
public extension FlatPoint {
    public Plane wedge(Dipole other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4((other.e42 * this.e35) + (other.e23 * this.e45), (other.e43 * this.e15) + (other.e31 * this.e45), (other.e41 * this.e25) + (other.e12 * this.e45), -(other.e31 * this.e25) - (other.e12 * this.e35)) - (this.group0.yzxx * float4(other.group0.xyz.zxy, other.e23))
        );
    }
}
public extension FlatPoint {
    public Plane wedge(DipoleInversion other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4((other.e42 * this.e35) + (other.e23 * this.e45), (other.e43 * this.e15) + (other.e31 * this.e45), (other.e41 * this.e25) + (other.e12 * this.e45), -(other.e31 * this.e25) - (other.e12 * this.e35)) - (this.group0.yzxx * float4(other.group0.xyz.zxy, other.e23))
        );
    }
}
public extension FlatPoint {
    public MultiVector wedge(MultiVector other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, -(this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412) - (this.e45 * other.e321)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            float4(other.scalar) * this.group0, 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(((float3(other.e4) * this.group0.xyz) - (float3(this.e45) * other.group1.xyz)), 0.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            (this.group0.zxy * other.group1.yzx) - (this.group0.yzx * other.group1.zxy), 
            /* e4235, e4315, e4125, e3215 */
            float4((this.e35 * other.e42) + (this.e45 * other.e23), (this.e15 * other.e43) + (this.e45 * other.e31), (this.e25 * other.e41) + (this.e45 * other.e12), -(this.e25 * other.e31) - (this.e35 * other.e12)) - (this.group0.yzxx * float4(other.group4.xyz.zxy, other.e23)), 
            /* e1234 */
            0.0
        );
    }
}
public extension FlatPoint {
    public Line wedge(RoundPoint other) {
        return Line(
            /* e415, e425, e435 */
            (float3(other.e4) * this.group0.xyz) - (float3(this.e45) * other.group0.xyz), 
            /* e235, e315, e125 */
            (this.group0.zxy * other.group0.yzx) - (this.group0.yzx * other.group0.zxy)
        );
    }
}
public extension FlatPoint {
    public FlatPoint wedge(Scalar other) {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(other.scalar) * this.group0
        );
    }
}
public extension FlatPoint {
    public Motor wedge(VersorEven other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(this.e15 * other.e4, this.e25 * other.e4, this.e35 * other.e4, -(this.e25 * other.e431) - (this.e35 * other.e412) - (this.e45 * other.e321)) - (this.group0.wwwx * float4(other.group3.xyz, other.e423)), 
            /* e235, e315, e125, e5 */
            float4(((this.group0.zxy * other.group3.yzx) - (this.group0.yzx * other.group3.zxy)), 0.0)
        );
    }
}
public extension FlatPoint {
    public Flector wedge(VersorOdd other) {
        return Flector(
            /* e15, e25, e35, e45 */
            float4(other.scalar) * this.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4((this.e35 * other.e42) + (this.e45 * other.e23), (this.e15 * other.e43) + (this.e45 * other.e31), (this.e25 * other.e41) + (this.e45 * other.e12), -(this.e25 * other.e31) - (this.e35 * other.e12)) - (this.group0.yzxx * float4(other.group0.zxy, other.e23))
        );
    }
}
public extension Flector {
    // Fancy infix trick (first half)
    public WedgeInfixPartial<Flector> operator /(WedgeInfix rhs) {
        return WedgeInfixPartial<Flector>(this);
    }
}
public extension WedgeInfixPartial<Flector> {
    // Fancy infix trick (second half)
    public Flector operator /(AntiCircleRotor rhs) {
        return this.a.wedge(rhs);
    }
}
public extension Flector {
    public Flector wedge(AntiCircleRotor other) {
        return Flector(
            /* e15, e25, e35, e45 */
            float4(other.scalar) * this.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4((other.e23 * this.e45) + (other.scalar * this.e4235), (other.e31 * this.e45) + (other.scalar * this.e4315), (other.e12 * this.e45) + (other.scalar * this.e4125), -(other.e31 * this.e25) - (other.e12 * this.e35)) + float4((other.group0.xyz.yzx * this.group0.zxy), other.scalar * this.e3215) - (this.group0.yzxx * float4(other.group0.xyz.zxy, other.e23))
        );
    }
}
public extension Flector {
    public Motor wedge(AntiDipoleInversion other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            (float4(other.e4) * float4(this.group0.xyz, this.e3215)) + float4(float3(0.0), (other.e1 * this.e4235) + (other.e2 * this.e4315) + (other.e3 * this.e4125) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e321 * this.e45)) - (this.group0.wwwx * float4(other.group3.xyz, other.e423)), 
            /* e235, e315, e125, e5 */
            float4(((other.group3.yzx * this.group0.zxy) - (other.group3.zxy * this.group0.yzx)), 0.0)
        );
    }
}
public extension Flector {
    public Flector wedge(AntiDualNum other) {
        return Flector(
            /* e15, e25, e35, e45 */
            float4(other.scalar) * this.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4(other.scalar) * this.group1
        );
    }
}
public extension Flector {
    public AntiScalar wedge(AntiFlatPoint other) {
        return AntiScalar(
            /* e12345 */
            other.e321 * this.e45 * -1.0
        );
    }
}
public extension Flector {
    public Motor wedge(AntiFlector other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((float3(this.e45) * other.group1.xyz * float3(-1.0)), (other.e1 * this.e4235) + (other.e2 * this.e4315) + (other.e3 * this.e4125) - (other.e321 * this.e45)), 
            /* e235, e315, e125, e5 */
            float4(((other.group1.yzx * this.group0.zxy) - (other.group1.zxy * this.group0.yzx)), 0.0)
        );
    }
}
public extension Flector {
    public Plane wedge(AntiLine other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4((float3(this.e45) * other.group0.xyz), -(other.e23 * this.e15) - (other.e31 * this.e25) - (other.e12 * this.e35))
        );
    }
}
public extension Flector {
    public Flector wedge(AntiMotor other) {
        return Flector(
            /* e15, e25, e35, e45 */
            float4(other.scalar) * this.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4(other.scalar * this.e4235, other.scalar * this.e4315, other.scalar * this.e4125, -(other.e23 * this.e15) - (other.e31 * this.e25) - (other.e12 * this.e35)) + (other.group0 * float4(this.group0.www, this.e3215))
        );
    }
}
public extension Flector {
    public Motor wedge(AntiPlane other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((float3(this.e45) * other.group0.xyz * float3(-1.0)), (other.e1 * this.e4235) + (other.e2 * this.e4315) + (other.e3 * this.e4125)), 
            /* e235, e315, e125, e5 */
            float4(((other.group0.yzx * this.group0.zxy) - (other.group0.zxy * this.group0.yzx)), 0.0)
        );
    }
}
public extension Flector {
    public AntiScalar wedge(Circle other) {
        return AntiScalar(
            /* e12345 */
            -(other.e423 * this.e15) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e321 * this.e45)
        );
    }
}
public extension Flector {
    public AntiScalar wedge(CircleRotor other) {
        return AntiScalar(
            /* e12345 */
            -(other.e423 * this.e15) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e321 * this.e45)
        );
    }
}
public extension Flector {
    public Plane wedge(Dipole other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4((other.e42 * this.e35) + (other.e23 * this.e45), (other.e43 * this.e15) + (other.e31 * this.e45), (other.e41 * this.e25) + (other.e12 * this.e45), -(other.e31 * this.e25) - (other.e12 * this.e35)) - (this.group0.yzxx * float4(other.group0.xyz.zxy, other.e23))
        );
    }
}
public extension Flector {
    public Plane wedge(DipoleInversion other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4((other.e42 * this.e35) + (other.e23 * this.e45), (other.e43 * this.e15) + (other.e31 * this.e45), (other.e41 * this.e25) + (other.e12 * this.e45), -(other.e31 * this.e25) - (other.e12 * this.e35)) - (this.group0.yzxx * float4(other.group0.xyz.zxy, other.e23))
        );
    }
}
public extension Flector {
    public MultiVector wedge(MultiVector other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, (this.e4235 * other.e1) + (this.e4315 * other.e2) + (this.e4125 * other.e3) + (this.e3215 * other.e4) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412) - (this.e45 * other.e321)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            float4(other.scalar) * this.group0, 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(((float3(other.e4) * this.group0.xyz) - (float3(this.e45) * other.group1.xyz)), 0.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            (this.group0.zxy * other.group1.yzx) - (this.group0.yzx * other.group1.zxy), 
            /* e4235, e4315, e4125, e3215 */
            float4((this.e35 * other.e42) + (this.e45 * other.e23), (this.e15 * other.e43) + (this.e45 * other.e31), (this.e25 * other.e41) + (this.e45 * other.e12), -(this.e25 * other.e31) - (this.e35 * other.e12)) + (float4(other.scalar) * this.group1) - (this.group0.yzxx * float4(other.group4.xyz.zxy, other.e23)), 
            /* e1234 */
            0.0
        );
    }
}
public extension Flector {
    public Motor wedge(RoundPoint other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(this.e45 * other.e1 * -1.0, this.e45 * other.e2 * -1.0, this.e45 * other.e3 * -1.0, (this.e4315 * other.e2) + (this.e4125 * other.e3) + (this.e3215 * other.e4)) + (other.group0.wwwx * float4(this.group0.xyz, this.e4235)), 
            /* e235, e315, e125, e5 */
            float4(((this.group0.zxy * other.group0.yzx) - (this.group0.yzx * other.group0.zxy)), 0.0)
        );
    }
}
public extension Flector {
    public Flector wedge(Scalar other) {
        return Flector(
            /* e15, e25, e35, e45 */
            float4(other.scalar) * this.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4(other.scalar) * this.group1
        );
    }
}
public extension Flector {
    public Motor wedge(VersorEven other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            (other.group3.wwwx * float4(this.group0.xyz, this.e4235)) + float4(float3(0.0), (this.e4315 * other.e2) + (this.e4125 * other.e3) + (this.e3215 * other.e4) - (this.e25 * other.e431) - (this.e35 * other.e412) - (this.e45 * other.e321)) - (this.group0.wwwx * float4(other.group3.xyz, other.e423)), 
            /* e235, e315, e125, e5 */
            float4(((this.group0.zxy * other.group3.yzx) - (this.group0.yzx * other.group3.zxy)), 0.0)
        );
    }
}
public extension Flector {
    public Flector wedge(VersorOdd other) {
        return Flector(
            /* e15, e25, e35, e45 */
            float4(other.scalar) * this.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4((this.e45 * other.e23) + (this.e4235 * other.scalar), (this.e45 * other.e31) + (this.e4315 * other.scalar), (this.e45 * other.e12) + (this.e4125 * other.scalar), -(this.e25 * other.e31) - (this.e35 * other.e12)) + (other.group0.yzxw * float4(this.group0.zxy, this.e3215)) - (this.group0.yzxx * float4(other.group0.zxy, other.e23))
        );
    }
}
public extension Line {
    // Fancy infix trick (first half)
    public WedgeInfixPartial<Line> operator /(WedgeInfix rhs) {
        return WedgeInfixPartial<Line>(this);
    }
}
public extension WedgeInfixPartial<Line> {
    // Fancy infix trick (second half)
    public Motor operator /(AntiCircleRotor rhs) {
        return this.a.wedge(rhs);
    }
}
public extension Line {
    public Motor wedge(AntiCircleRotor other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((float3(other.scalar) * this.group0.xyz), -(other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435)), 
            /* e235, e315, e125, e5 */
            float4(float3(1.0), 0.0) * float4(this.group1.xyz, 0.0) * float4(other.group2.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
public extension Line {
    public Plane wedge(AntiDipoleInversion other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(-(other.e4 * this.e235) - (other.e3 * this.e425), -(other.e4 * this.e315) - (other.e1 * this.e435), -(other.e4 * this.e125) - (other.e2 * this.e415), (other.e2 * this.e315) + (other.e3 * this.e125)) + (other.group3.yzxx * float4(this.group0.xyz.zxy, this.e235))
        );
    }
}
public extension Line {
    public Line wedge(AntiDualNum other) {
        return Line(
            /* e415, e425, e435 */
            float3(other.scalar) * this.group0.xyz, 
            /* e235, e315, e125 */
            float3(other.scalar) * this.group1.xyz
        );
    }
}
public extension Line {
    public Plane wedge(AntiFlector other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(other.e3 * this.e425 * -1.0, other.e1 * this.e435 * -1.0, other.e2 * this.e415 * -1.0, (other.e2 * this.e315) + (other.e3 * this.e125)) + (other.group1.yzxx * float4(this.group0.xyz.zxy, this.e235))
        );
    }
}
public extension Line {
    public AntiScalar wedge(AntiLine other) {
        return AntiScalar(
            /* e12345 */
            -(other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435)
        );
    }
}
public extension Line {
    public Motor wedge(AntiMotor other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((float3(other.scalar) * this.group0.xyz), -(other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435)), 
            /* e235, e315, e125, e5 */
            float4(float3(1.0), 0.0) * float4(this.group1.xyz, 0.0) * float4(other.group0.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
public extension Line {
    public Plane wedge(AntiPlane other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(other.e3 * this.e425 * -1.0, other.e1 * this.e435 * -1.0, other.e2 * this.e415 * -1.0, (other.e2 * this.e315) + (other.e3 * this.e125)) + (other.group0.yzxx * float4(this.group0.xyz.zxy, this.e235))
        );
    }
}
public extension Line {
    public AntiScalar wedge(Dipole other) {
        return AntiScalar(
            /* e12345 */
            -(other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435)
        );
    }
}
public extension Line {
    public AntiScalar wedge(DipoleInversion other) {
        return AntiScalar(
            /* e12345 */
            -(other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435)
        );
    }
}
public extension Line {
    public MultiVector wedge(MultiVector other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, -(this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43)), 
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
            float4(other.group0.xy.xx, other.scalar, 0.0) * float4(float3(1.0), 0.0) * float4(this.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(other.scalar) * this.group1.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(-(this.e425 * other.e3) - (this.e235 * other.e4), -(this.e435 * other.e1) - (this.e315 * other.e4), -(this.e415 * other.e2) - (this.e125 * other.e4), (this.e315 * other.e2) + (this.e125 * other.e3)) + (other.group1.yzxx * float4(this.group0.xyz.zxy, this.e235)), 
            /* e1234 */
            0.0
        );
    }
}
public extension Line {
    public Plane wedge(RoundPoint other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(-(this.e425 * other.e3) - (this.e235 * other.e4), -(this.e435 * other.e1) - (this.e315 * other.e4), -(this.e415 * other.e2) - (this.e125 * other.e4), (this.e315 * other.e2) + (this.e125 * other.e3)) + (other.group0.yzxx * float4(this.group0.xyz.zxy, this.e235))
        );
    }
}
public extension Line {
    public Line wedge(Scalar other) {
        return Line(
            /* e415, e425, e435 */
            float3(other.scalar) * this.group0.xyz, 
            /* e235, e315, e125 */
            float3(other.scalar) * this.group1.xyz
        );
    }
}
public extension Line {
    public Plane wedge(VersorEven other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(-(this.e425 * other.e3) - (this.e235 * other.e4), -(this.e435 * other.e1) - (this.e315 * other.e4), -(this.e415 * other.e2) - (this.e125 * other.e4), (this.e315 * other.e2) + (this.e125 * other.e3)) + (other.group3.yzxx * float4(this.group0.xyz.zxy, this.e235))
        );
    }
}
public extension Line {
    public Motor wedge(VersorOdd other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((float3(other.scalar) * this.group0.xyz), -(this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43)), 
            /* e235, e315, e125, e5 */
            float4(float3(1.0), 0.0) * float4(this.group1.xyz, 0.0) * float4(other.group0.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
public extension Motor {
    // Fancy infix trick (first half)
    public WedgeInfixPartial<Motor> operator /(WedgeInfix rhs) {
        return WedgeInfixPartial<Motor>(this);
    }
}
public extension WedgeInfixPartial<Motor> {
    // Fancy infix trick (second half)
    public Motor operator /(AntiCircleRotor rhs) {
        return this.a.wedge(rhs);
    }
}
public extension Motor {
    public Motor wedge(AntiCircleRotor other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(other.scalar * this.e415, other.scalar * this.e425, other.scalar * this.e435, -(other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435)) + float4((other.group0.xyz * this.group1.www), other.scalar * this.e12345), 
            /* e235, e315, e125, e5 */
            float4(((float3(other.scalar) * this.group1.xyz) + (float3(this.e5) * other.group1.xyz)), other.scalar * this.e5)
        );
    }
}
public extension Motor {
    public Flector wedge(AntiDipoleInversion other) {
        return Flector(
            /* e15, e25, e35, e45 */
            float4(this.e5) * float4(other.group3.xyz, other.e4) * float4(-1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(-(other.e4 * this.e235) - (other.e3 * this.e425), -(other.e4 * this.e315) - (other.e1 * this.e435), -(other.e4 * this.e125) - (other.e2 * this.e415), (other.e2 * this.e315) + (other.e3 * this.e125)) + (other.group3.yzxx * float4(this.group0.zxy, this.e235)) - (float4(this.e5) * float4(other.group0.xyz, other.e321))
        );
    }
}
public extension Motor {
    public Motor wedge(AntiDualNum other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(other.scalar) * this.group0, 
            /* e235, e315, e125, e5 */
            float4(other.scalar) * this.group1
        );
    }
}
public extension Motor {
    public AntiDualNum wedge(AntiFlatPoint other) {
        return AntiDualNum(
            /* e3215, scalar */
            float2(other.e321 * this.e5, 1.0) * float2(-1.0, 0.0)
        );
    }
}
public extension Motor {
    public Flector wedge(AntiFlector other) {
        return Flector(
            /* e15, e25, e35, e45 */
            float4(float3(1.0), 0.0) * float4(this.group1.www, 0.0) * float4(other.group1.xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            (other.group1.yzxx * float4(this.group0.zxy, this.e235)) + float4(float3(0.0), (other.e2 * this.e315) + (other.e3 * this.e125)) - float4((other.group1.zxy * this.group0.yzx), other.e321 * this.e5)
        );
    }
}
public extension Motor {
    public Motor wedge(AntiLine other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(float3(0.0), -(other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435)), 
            /* e235, e315, e125, e5 */
            float4(float3(1.0), 0.0) * float4(other.group0.xyz, 0.0) * float4(this.group1.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
public extension Motor {
    public Motor wedge(AntiMotor other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((other.group0.www * this.group0.xyz), (other.scalar * this.e12345) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435)), 
            /* e235, e315, e125, e5 */
            float4(((float3(other.scalar) * this.group1.xyz) + (float3(this.e5) * other.group0.xyz)), other.scalar * this.e5)
        );
    }
}
public extension Motor {
    public Flector wedge(AntiPlane other) {
        return Flector(
            /* e15, e25, e35, e45 */
            float4(float3(1.0), 0.0) * float4(this.group1.www, 0.0) * float4(other.group0.xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.e3 * this.e425 * -1.0, other.e1 * this.e435 * -1.0, other.e2 * this.e415 * -1.0, (other.e2 * this.e315) + (other.e3 * this.e125)) + (other.group0.yzxx * float4(this.group0.zxy, this.e235))
        );
    }
}
public extension Motor {
    public Plane wedge(Circle other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(this.e5) * float4(other.group0.xyz, other.e321) * float4(-1.0)
        );
    }
}
public extension Motor {
    public Plane wedge(CircleRotor other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(this.e5) * float4(other.group0.xyz, other.e321) * float4(-1.0)
        );
    }
}
public extension Motor {
    public Motor wedge(Dipole other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((float3(this.e5) * other.group0.xyz), -(other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435)), 
            /* e235, e315, e125, e5 */
            float4(float3(1.0), 0.0) * float4(this.group1.www, 0.0) * float4(other.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
public extension Motor {
    public Motor wedge(DipoleInversion other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((float3(this.e5) * other.group0.xyz), (other.e1234 * this.e5) - (other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435)), 
            /* e235, e315, e125, e5 */
            float4(float3(1.0), 0.0) * float4(this.group1.www, 0.0) * float4(other.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
public extension Motor {
    public MultiVector wedge(MultiVector other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, (this.e12345 * other.scalar) + (this.e5 * other.e1234) - (this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            this.e5 * other.scalar, 
            /* e15, e25, e35, e45 */
            float4(this.e5) * other.group1 * float4(-1.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(((float3(this.e5) * other.group4.xyz) + (float3(other.scalar) * this.group0.xyz)), 0.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            (float3(this.e5) * other.group5.xyz) + (float3(other.scalar) * this.group1.xyz), 
            /* e4235, e4315, e4125, e3215 */
            float4(-(this.e425 * other.e3) - (this.e235 * other.e4), -(this.e435 * other.e1) - (this.e315 * other.e4), -(this.e415 * other.e2) - (this.e125 * other.e4), (this.e315 * other.e2) + (this.e125 * other.e3)) + (other.group1.yzxx * float4(this.group0.zxy, this.e235)) - float4((other.group7.xyz * this.group1.www), this.e5 * other.e321), 
            /* e1234 */
            0.0
        );
    }
}
public extension Motor {
    public Flector wedge(RoundPoint other) {
        return Flector(
            /* e15, e25, e35, e45 */
            float4(this.e5) * other.group0 * float4(-1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(-(this.e425 * other.e3) - (this.e235 * other.e4), -(this.e435 * other.e1) - (this.e315 * other.e4), -(this.e415 * other.e2) - (this.e125 * other.e4), (this.e315 * other.e2) + (this.e125 * other.e3)) + (other.group0.yzxx * float4(this.group0.zxy, this.e235))
        );
    }
}
public extension Motor {
    public Motor wedge(Scalar other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(other.scalar) * this.group0, 
            /* e235, e315, e125, e5 */
            float4(other.scalar) * this.group1
        );
    }
}
public extension Motor {
    public AntiScalar wedge(Sphere other) {
        return AntiScalar(
            /* e12345 */
            this.e5 * other.e1234
        );
    }
}
public extension Motor {
    public Flector wedge(VersorEven other) {
        return Flector(
            /* e15, e25, e35, e45 */
            float4(this.e5) * other.group3 * float4(-1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(-(this.e235 * other.e4) - (this.e5 * other.e423), -(this.e315 * other.e4) - (this.e5 * other.e431), -(this.e125 * other.e4) - (this.e5 * other.e412), (this.e315 * other.e2) + (this.e125 * other.e3)) + (other.group3.yzxx * float4(this.group0.zxy, this.e235)) - float4((this.group0.yzx * other.group3.zxy), this.e5 * other.e321)
        );
    }
}
public extension Motor {
    public Motor wedge(VersorOdd other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            (float4(this.e5) * float4(other.group0.xyz, other.e1234)) + (float4(other.scalar) * this.group0) + float4(float3(0.0), -(this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43)), 
            /* e235, e315, e125, e5 */
            float4(((float3(this.e5) * other.group1.xyz) + (float3(other.scalar) * this.group1.xyz)), this.e5 * other.scalar)
        );
    }
}
public extension MultiVector {
    // Fancy infix trick (first half)
    public WedgeInfixPartial<MultiVector> operator /(WedgeInfix rhs) {
        return WedgeInfixPartial<MultiVector>(this);
    }
}
public extension WedgeInfixPartial<MultiVector> {
    // Fancy infix trick (second half)
    public MultiVector operator /(AntiCircleRotor rhs) {
        return this.a.wedge(rhs);
    }
}
public extension MultiVector {
    public MultiVector wedge(AntiCircleRotor other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(other.scalar * this.scalar, (other.scalar * this.e12345) - (other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e45 * this.e321) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412)), 
            /* e1, e2, e3, e4 */
            float4(other.scalar) * this.group1, 
            /* e5 */
            other.scalar * this.e5, 
            /* e15, e25, e35, e45 */
            (float4(other.scalar) * this.group3) + (float4(this.scalar) * float4(other.group2.xyz, other.e45)), 
            /* e41, e42, e43 */
            (float3(other.scalar) * this.group4.xyz) + (float3(this.scalar) * other.group0.xyz), 
            /* e23, e31, e12 */
            (float3(other.scalar) * this.group5.xyz) + (float3(this.scalar) * other.group1.xyz), 
            /* e415, e425, e435, e321 */
            float4((other.e15 * this.e4) + (other.scalar * this.e415), (other.e25 * this.e4) + (other.scalar * this.e425), (other.e35 * this.e4) + (other.scalar * this.e435), -(other.e31 * this.e2) - (other.e12 * this.e3)) + float4((float3(this.e5) * other.group0.xyz), other.scalar * this.e321) - (other.group1.wwwx * this.group1.xyzx), 
            /* e423, e431, e412 */
            (float3(other.scalar) * this.group7.xyz) + (float3(this.e4) * other.group1.xyz) + (other.group0.xyz.yzx * this.group1.zxy) - (other.group0.xyz.zxy * this.group1.yzx), 
            /* e235, e315, e125 */
            (float3(other.scalar) * this.group8.xyz) + (float3(this.e5) * other.group1.xyz) + (other.group2.zxy * this.group1.yzx) - (other.group2.yzx * this.group1.zxy), 
            /* e4235, e4315, e4125, e3215 */
            float4((other.e23 * this.e45) + (other.e45 * this.e23) + (other.e35 * this.e42) + (other.scalar * this.e4235), (other.e31 * this.e45) + (other.e45 * this.e31) + (other.e15 * this.e43) + (other.scalar * this.e4315), (other.e12 * this.e45) + (other.e45 * this.e12) + (other.e25 * this.e41) + (other.scalar * this.e4125), -(other.e23 * this.e15) - (other.e31 * this.e25) - (other.e12 * this.e35) - (other.e35 * this.e12)) + float4((other.group0.xyz.yzx * this.group3.zxy), other.scalar * this.e3215) - (other.group2.yzxy * float4(this.group4.xyz.zxy, this.e31)) - float4((other.group0.xyz.zxy * this.group3.yzx), other.e15 * this.e23), 
            /* e1234 */
            (other.scalar * this.e1234) - (other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43)
        );
    }
}
public extension MultiVector {
    public MultiVector wedge(AntiDipoleInversion other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, (other.e4 * this.e3215) + (other.e1 * this.e4235) + (other.e2 * this.e4315) + (other.e3 * this.e4125) + (other.e5 * this.e1234) - (other.e423 * this.e15) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e321 * this.e45) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43)), 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * float4(other.group3.xyz, other.e4), 
            /* e5 */
            other.e5 * this.scalar, 
            /* e15, e25, e35, e45 */
            (float4(other.e5) * this.group1) - (float4(this.e5) * float4(other.group3.xyz, other.e4)), 
            /* e41, e42, e43 */
            (float3(this.e4) * other.group3.xyz) - (float3(other.e4) * this.group1.xyz), 
            /* e23, e31, e12 */
            (other.group3.zxy * this.group1.yzx) - (other.group3.yzx * this.group1.zxy), 
            /* e415, e425, e435, e321 */
            float4((other.e4 * this.e15) + (other.e5 * this.e41), (other.e4 * this.e25) + (other.e5 * this.e42), (other.e4 * this.e35) + (other.e5 * this.e43), -(other.e2 * this.e31) - (other.e3 * this.e12)) + (float4(this.scalar) * other.group1) - float4((this.group3.www * other.group3.xyz), other.e1 * this.e23), 
            /* e423, e431, e412 */
            (float3(other.e4) * this.group5.xyz) + (float3(this.scalar) * other.group0.xyz) + (this.group4.xyz.yzx * other.group3.zxy) - (this.group4.xyz.zxy * other.group3.yzx), 
            /* e235, e315, e125 */
            (float3(other.e5) * this.group5.xyz) + (float3(this.scalar) * other.group2.xyz) + (other.group3.yzx * this.group3.zxy) - (other.group3.zxy * this.group3.yzx), 
            /* e4235, e4315, e4125, e3215 */
            (other.group3.yzxw * this.group6.zxyw) + (other.group3.wwwx * float4(this.group7.xyz, this.e235)) + float4((other.group1.yzx * this.group1.zxy), other.e2 * this.e315) + float4((this.group1.www * other.group2.xyz), other.e3 * this.e125) - (float4(this.e5) * float4(other.group0.xyz, other.e321)) - (this.group1.yzxy * float4(other.group1.zxy, other.e315)) - float4((this.group8.xyz * other.group2.www), other.e235 * this.e1) - float4((other.group3.zxy * this.group6.yzx), other.e125 * this.e3), 
            /* e1234 */
            (other.e423 * this.e1) + (other.e431 * this.e2) + (other.e412 * this.e3) + (other.e321 * this.e4) - (other.e4 * this.e321) - (other.e1 * this.e423) - (other.e2 * this.e431) - (other.e3 * this.e412)
        );
    }
}
public extension MultiVector {
    public MultiVector wedge(AntiDualNum other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(other.scalar * this.scalar, (other.e3215 * this.e4) + (other.scalar * this.e12345)), 
            /* e1, e2, e3, e4 */
            float4(other.scalar) * this.group1, 
            /* e5 */
            other.scalar * this.e5, 
            /* e15, e25, e35, e45 */
            float4(other.scalar) * this.group3, 
            /* e41, e42, e43 */
            float3(other.scalar) * this.group4.xyz, 
            /* e23, e31, e12 */
            float3(other.scalar) * this.group5.xyz, 
            /* e415, e425, e435, e321 */
            float4(other.scalar) * this.group6, 
            /* e423, e431, e412 */
            float3(other.scalar) * this.group7.xyz, 
            /* e235, e315, e125 */
            float3(other.scalar) * this.group8.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xy.yy, other.scalar, (other.e3215 * this.scalar) + (other.scalar * this.e3215)) * float4(this.group9.xyz, 1.0), 
            /* e1234 */
            other.scalar * this.e1234
        );
    }
}
public extension MultiVector {
    public MultiVector wedge(AntiFlatPoint other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, -(other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43) - (other.e321 * this.e45)), 
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
            float4(float3(0.0), other.e321 * this.scalar), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(this.scalar) * other.group0.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4((float3(this.e4) * other.group0.xyz), -(other.e235 * this.e1) - (other.e315 * this.e2) - (other.e125 * this.e3) - (other.e321 * this.e5)), 
            /* e1234 */
            other.e321 * this.e4
        );
    }
}
public extension MultiVector {
    public MultiVector wedge(AntiFlector other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, (other.e1 * this.e4235) + (other.e2 * this.e4315) + (other.e3 * this.e4125) + (other.e5 * this.e1234) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43) - (other.e321 * this.e45)), 
            /* e1, e2, e3, e4 */
            float4(this.group0.xy.xx, this.scalar, 0.0) * float4(float3(1.0), 0.0) * float4(other.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e5 */
            other.e5 * this.scalar, 
            /* e15, e25, e35, e45 */
            float4(((float3(other.e5) * this.group1.xyz) - (float3(this.e5) * other.group1.xyz)), other.e5 * this.e4), 
            /* e41, e42, e43 */
            float3(this.e4) * other.group1.xyz, 
            /* e23, e31, e12 */
            (other.group1.zxy * this.group1.yzx) - (other.group1.yzx * this.group1.zxy), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), -(other.e2 * this.e31) - (other.e3 * this.e12)) + float4((this.group4.xyz * other.group1.www), other.e321 * this.scalar) - float4((this.group3.www * other.group1.xyz), other.e1 * this.e23), 
            /* e423, e431, e412 */
            (this.group4.xyz.yzx * other.group1.zxy) - (this.group4.xyz.zxy * other.group1.yzx), 
            /* e235, e315, e125 */
            (float3(other.e5) * this.group5.xyz) + (float3(this.scalar) * other.group0.xyz) + (other.group1.yzx * this.group3.zxy) - (other.group1.zxy * this.group3.yzx), 
            /* e4235, e4315, e4125, e3215 */
            (other.group1.wwwx * float4(this.group7.xyz, this.e235)) + float4(float3(0.0), (other.e5 * this.e321) - (other.e315 * this.e2) - (other.e125 * this.e3) - (other.e321 * this.e5)) + float4((other.group1.yzx * this.group6.zxy), other.e3 * this.e125) + float4((this.group1.www * other.group0.xyz), other.e2 * this.e315) - float4((other.group1.zxy * this.group6.yzx), other.e235 * this.e1), 
            /* e1234 */
            (other.e321 * this.e4) - (other.e1 * this.e423) - (other.e2 * this.e431) - (other.e3 * this.e412)
        );
    }
}
public extension MultiVector {
    public MultiVector wedge(AntiLine other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, -(other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            float4(this.group0.xy.xx, this.scalar, 0.0) * float4(float3(1.0), 0.0) * float4(other.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(this.scalar) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4((float3(this.e4) * other.group1.xyz), -(other.e23 * this.e1) - (other.e31 * this.e2) - (other.e12 * this.e3)), 
            /* e423, e431, e412 */
            float3(this.e4) * other.group0.xyz, 
            /* e235, e315, e125 */
            (float3(this.e5) * other.group0.xyz) + (other.group1.xyz.zxy * this.group1.yzx) - (other.group1.xyz.yzx * this.group1.zxy), 
            /* e4235, e4315, e4125, e3215 */
            float4((other.e23 * this.e45) + (other.e35 * this.e42), (other.e31 * this.e45) + (other.e15 * this.e43), (other.e12 * this.e45) + (other.e25 * this.e41), -(other.e31 * this.e25) - (other.e12 * this.e35) - (other.e15 * this.e23) - (other.e25 * this.e31) - (other.e35 * this.e12)) - float4((other.group1.xyz.yzx * this.group4.xyz.zxy), other.e23 * this.e15), 
            /* e1234 */
            -(other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43)
        );
    }
}
public extension MultiVector {
    public MultiVector wedge(AntiMotor other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(other.scalar * this.scalar, (other.scalar * this.e12345) + (other.e3215 * this.e4) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412)), 
            /* e1, e2, e3, e4 */
            float4(other.scalar) * this.group1, 
            /* e5 */
            other.scalar * this.e5, 
            /* e15, e25, e35, e45 */
            float4(((float3(other.scalar) * this.group3.xyz) + (float3(this.scalar) * other.group1.xyz)), other.scalar * this.e45), 
            /* e41, e42, e43 */
            float3(other.scalar) * this.group4.xyz, 
            /* e23, e31, e12 */
            (float3(other.scalar) * this.group5.xyz) + (float3(this.scalar) * other.group0.xyz), 
            /* e415, e425, e435, e321 */
            float4(other.e15 * this.e4, other.e25 * this.e4, other.e35 * this.e4, -(other.e23 * this.e1) - (other.e31 * this.e2) - (other.e12 * this.e3)) + (float4(other.scalar) * this.group6), 
            /* e423, e431, e412 */
            (float3(other.scalar) * this.group7.xyz) + (float3(this.e4) * other.group0.xyz), 
            /* e235, e315, e125 */
            (float3(other.scalar) * this.group8.xyz) + (float3(this.e5) * other.group0.xyz) + (other.group1.zxy * this.group1.yzx) - (other.group1.yzx * this.group1.zxy), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.scalar * this.e4235, other.scalar * this.e4315, other.scalar * this.e4125, -(other.e23 * this.e15) - (other.e31 * this.e25) - (other.e12 * this.e35) - (other.e25 * this.e31) - (other.e35 * this.e12)) + (other.group0 * float4(this.group3.www, this.e3215)) + (other.group1.zxyw * float4(this.group4.xyz.yzx, this.scalar)) - (other.group1.yzxx * float4(this.group4.xyz.zxy, this.e23)), 
            /* e1234 */
            (other.scalar * this.e1234) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43)
        );
    }
}
public extension MultiVector {
    public MultiVector wedge(AntiPlane other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, (other.e1 * this.e4235) + (other.e2 * this.e4315) + (other.e3 * this.e4125) + (other.e5 * this.e1234)), 
            /* e1, e2, e3, e4 */
            float4(this.group0.xy.xx, this.scalar, 0.0) * float4(float3(1.0), 0.0) * float4(other.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e5 */
            other.e5 * this.scalar, 
            /* e15, e25, e35, e45 */
            float4(((float3(other.e5) * this.group1.xyz) - (float3(this.e5) * other.group0.xyz)), other.e5 * this.e4), 
            /* e41, e42, e43 */
            float3(this.e4) * other.group0.xyz, 
            /* e23, e31, e12 */
            (other.group0.zxy * this.group1.yzx) - (other.group0.yzx * this.group1.zxy), 
            /* e415, e425, e435, e321 */
            float4(other.e5 * this.e41, other.e5 * this.e42, other.e5 * this.e43, -(other.e2 * this.e31) - (other.e3 * this.e12)) - float4((this.group3.www * other.group0.xyz), other.e1 * this.e23), 
            /* e423, e431, e412 */
            (this.group4.xyz.yzx * other.group0.zxy) - (this.group4.xyz.zxy * other.group0.yzx), 
            /* e235, e315, e125 */
            (float3(other.e5) * this.group5.xyz) + (other.group0.yzx * this.group3.zxy) - (other.group0.zxy * this.group3.yzx), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.e3 * this.e425 * -1.0, other.e1 * this.e435 * -1.0, other.e2 * this.e415 * -1.0, (other.e3 * this.e125) + (other.e5 * this.e321)) + (other.group0.wwwx * float4(this.group7.xyz, this.e235)) + float4((other.group0.yzx * this.group6.zxy), other.e2 * this.e315), 
            /* e1234 */
            -(other.e1 * this.e423) - (other.e2 * this.e431) - (other.e3 * this.e412)
        );
    }
}
public extension MultiVector {
    public AntiScalar wedge(AntiScalar other) {
        return AntiScalar(
            /* e12345 */
            other.e12345 * this.scalar
        );
    }
}
public extension MultiVector {
    public MultiVector wedge(Circle other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, -(other.e423 * this.e15) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e321 * this.e45) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43)), 
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
            float4(this.scalar) * other.group1, 
            /* e423, e431, e412 */
            float3(this.scalar) * other.group0.xyz, 
            /* e235, e315, e125 */
            float3(this.scalar) * other.group2.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4((other.e425 * this.e3) + (other.e235 * this.e4), (other.e435 * this.e1) + (other.e315 * this.e4), (other.e415 * this.e2) + (other.e125 * this.e4), -(other.e321 * this.e5) - (other.e125 * this.e3)) - (this.group1.yzxy * float4(other.group1.zxy, other.e315)) - float4((float3(this.e5) * other.group0.xyz), other.e235 * this.e1), 
            /* e1234 */
            (other.e423 * this.e1) + (other.e431 * this.e2) + (other.e412 * this.e3) + (other.e321 * this.e4)
        );
    }
}
public extension MultiVector {
    public MultiVector wedge(CircleRotor other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, (other.e12345 * this.scalar) - (other.e423 * this.e15) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e321 * this.e45) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43)), 
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
            float4(this.scalar) * other.group1, 
            /* e423, e431, e412 */
            float3(this.scalar) * other.group0.xyz, 
            /* e235, e315, e125 */
            float3(this.scalar) * other.group2.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4((other.e425 * this.e3) + (other.e235 * this.e4), (other.e435 * this.e1) + (other.e315 * this.e4), (other.e415 * this.e2) + (other.e125 * this.e4), -(other.e315 * this.e2) - (other.e125 * this.e3)) - (float4(this.e5) * float4(other.group0.xyz, other.e321)) - (this.group1.yzxx * float4(other.group1.zxy, other.e235)), 
            /* e1234 */
            (other.e423 * this.e1) + (other.e431 * this.e2) + (other.e412 * this.e3) + (other.e321 * this.e4)
        );
    }
}
public extension MultiVector {
    public MultiVector wedge(Dipole other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, -(other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e45 * this.e321) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            float4(this.scalar) * float4(other.group2.xyz, other.e45), 
            /* e41, e42, e43 */
            float3(this.scalar) * other.group0.xyz, 
            /* e23, e31, e12 */
            float3(this.scalar) * other.group1.xyz, 
            /* e415, e425, e435, e321 */
            float4((other.e41 * this.e5) + (other.e15 * this.e4), (other.e42 * this.e5) + (other.e25 * this.e4), (other.e43 * this.e5) + (other.e35 * this.e4), -(other.e31 * this.e2) - (other.e12 * this.e3)) - (other.group1.wwwx * this.group1.xyzx), 
            /* e423, e431, e412 */
            (float3(this.e4) * other.group1.xyz) + (other.group0.xyz.yzx * this.group1.zxy) - (other.group0.xyz.zxy * this.group1.yzx), 
            /* e235, e315, e125 */
            (float3(this.e5) * other.group1.xyz) + (other.group2.xyz.zxy * this.group1.yzx) - (other.group2.xyz.yzx * this.group1.zxy), 
            /* e4235, e4315, e4125, e3215 */
            float4((other.e42 * this.e35) + (other.e23 * this.e45) + (other.e45 * this.e23) + (other.e35 * this.e42), (other.e43 * this.e15) + (other.e31 * this.e45) + (other.e45 * this.e31) + (other.e15 * this.e43), (other.e41 * this.e25) + (other.e12 * this.e45) + (other.e45 * this.e12) + (other.e25 * this.e41), -(other.e23 * this.e15) - (other.e31 * this.e25) - (other.e12 * this.e35) - (other.e35 * this.e12)) - float4((other.group0.xyz.zxy * this.group3.yzx), other.e15 * this.e23) - float4((other.group2.xyz.yzx * this.group4.xyz.zxy), other.e25 * this.e31), 
            /* e1234 */
            -(other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43)
        );
    }
}
public extension MultiVector {
    public MultiVector wedge(DipoleInversion other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, (other.e1234 * this.e5) + (other.e4235 * this.e1) + (other.e4315 * this.e2) + (other.e4125 * this.e3) + (other.e3215 * this.e4) - (other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e45 * this.e321) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            float4(this.scalar) * float4(other.group2.xyz, other.e45), 
            /* e41, e42, e43 */
            float3(this.scalar) * other.group0.xyz, 
            /* e23, e31, e12 */
            float3(this.scalar) * other.group1.xyz, 
            /* e415, e425, e435, e321 */
            float4((other.e41 * this.e5) + (other.e15 * this.e4), (other.e42 * this.e5) + (other.e25 * this.e4), (other.e43 * this.e5) + (other.e35 * this.e4), -(other.e31 * this.e2) - (other.e12 * this.e3)) - (other.group1.wwwx * this.group1.xyzx), 
            /* e423, e431, e412 */
            (float3(this.e4) * other.group1.xyz) + (other.group0.xyz.yzx * this.group1.zxy) - (other.group0.xyz.zxy * this.group1.yzx), 
            /* e235, e315, e125 */
            (float3(this.e5) * other.group1.xyz) + (other.group2.zxy * this.group1.yzx) - (other.group2.yzx * this.group1.zxy), 
            /* e4235, e4315, e4125, e3215 */
            float4((other.e42 * this.e35) + (other.e23 * this.e45) + (other.e45 * this.e23) + (other.e35 * this.e42), (other.e43 * this.e15) + (other.e31 * this.e45) + (other.e45 * this.e31) + (other.e15 * this.e43), (other.e41 * this.e25) + (other.e12 * this.e45) + (other.e45 * this.e12) + (other.e25 * this.e41), -(other.e23 * this.e15) - (other.e31 * this.e25) - (other.e12 * this.e35) - (other.e35 * this.e12)) + (float4(this.scalar) * other.group3) - (other.group2.yzxy * float4(this.group4.xyz.zxy, this.e31)) - float4((other.group0.xyz.zxy * this.group3.yzx), other.e15 * this.e23), 
            /* e1234 */
            (other.e1234 * this.scalar) - (other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43)
        );
    }
}
public extension MultiVector {
    public MultiVector wedge(DualNum other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, (other.e5 * this.e1234) + (other.e12345 * this.scalar)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            other.e5 * this.scalar, 
            /* e15, e25, e35, e45 */
            float4(other.e5) * this.group1, 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(other.group0.xy.xx, other.e5, 0.0) * float4(float3(1.0), 0.0) * float4(this.group4.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(other.e5) * this.group5.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(other.e5) * float4(this.group7.xyz, this.e321), 
            /* e1234 */
            0.0
        );
    }
}
public extension MultiVector {
    public MultiVector wedge(FlatPoint other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, -(other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412) - (other.e45 * this.e321)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            float4(this.scalar) * other.group0, 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(((float3(this.e4) * other.group0.xyz) - (float3(other.e45) * this.group1.xyz)), 0.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            (other.group0.zxy * this.group1.yzx) - (other.group0.yzx * this.group1.zxy), 
            /* e4235, e4315, e4125, e3215 */
            float4((other.e35 * this.e42) + (other.e45 * this.e23), (other.e15 * this.e43) + (other.e45 * this.e31), (other.e25 * this.e41) + (other.e45 * this.e12), -(other.e25 * this.e31) - (other.e35 * this.e12)) - (other.group0.yzxx * float4(this.group4.xyz.zxy, this.e23)), 
            /* e1234 */
            0.0
        );
    }
}
public extension MultiVector {
    public MultiVector wedge(Flector other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, (other.e4235 * this.e1) + (other.e4315 * this.e2) + (other.e4125 * this.e3) + (other.e3215 * this.e4) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412) - (other.e45 * this.e321)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            float4(this.scalar) * other.group0, 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(((float3(this.e4) * other.group0.xyz) - (float3(other.e45) * this.group1.xyz)), 0.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            (other.group0.zxy * this.group1.yzx) - (other.group0.yzx * this.group1.zxy), 
            /* e4235, e4315, e4125, e3215 */
            float4((other.e35 * this.e42) + (other.e45 * this.e23), (other.e15 * this.e43) + (other.e45 * this.e31), (other.e25 * this.e41) + (other.e45 * this.e12), -(other.e25 * this.e31) - (other.e35 * this.e12)) + (float4(this.scalar) * other.group1) - (other.group0.yzxx * float4(this.group4.xyz.zxy, this.e23)), 
            /* e1234 */
            0.0
        );
    }
}
public extension MultiVector {
    public MultiVector wedge(Line other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, -(other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43)), 
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
            float4(this.group0.xy.xx, this.scalar, 0.0) * float4(float3(1.0), 0.0) * float4(other.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(this.scalar) * other.group1.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4((other.e425 * this.e3) + (other.e235 * this.e4), (other.e435 * this.e1) + (other.e315 * this.e4), (other.e415 * this.e2) + (other.e125 * this.e4), -(other.e315 * this.e2) - (other.e125 * this.e3)) - (this.group1.yzxx * float4(other.group0.xyz.zxy, other.e235)), 
            /* e1234 */
            0.0
        );
    }
}
public extension MultiVector {
    public MultiVector wedge(Motor other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, (other.e12345 * this.scalar) + (other.e5 * this.e1234) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            other.e5 * this.scalar, 
            /* e15, e25, e35, e45 */
            float4(other.e5) * this.group1, 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(((float3(other.e5) * this.group4.xyz) + (float3(this.scalar) * other.group0.xyz)), 0.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            (float3(other.e5) * this.group5.xyz) + (float3(this.scalar) * other.group1.xyz), 
            /* e4235, e4315, e4125, e3215 */
            float4((other.e425 * this.e3) + (other.e235 * this.e4), (other.e435 * this.e1) + (other.e315 * this.e4), (other.e415 * this.e2) + (other.e125 * this.e4), -(other.e315 * this.e2) - (other.e125 * this.e3)) + float4((this.group7.xyz * other.group1.www), other.e5 * this.e321) - (this.group1.yzxx * float4(other.group0.zxy, other.e235)), 
            /* e1234 */
            0.0
        );
    }
}
public extension MultiVector {
    public MultiVector wedge(MultiVector other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(other.scalar * this.scalar, (other.scalar * this.e12345) + (other.e12345 * this.scalar) + (other.e1 * this.e4235) + (other.e2 * this.e4315) + (other.e3 * this.e4125) + (other.e4 * this.e3215) + (other.e5 * this.e1234) + (other.e4235 * this.e1) + (other.e4315 * this.e2) + (other.e4125 * this.e3) + (other.e3215 * this.e4) + (other.e1234 * this.e5) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412) - (other.e45 * this.e321) - (other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e321 * this.e45) - (other.e423 * this.e15) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43)), 
            /* e1, e2, e3, e4 */
            (float4(other.scalar) * this.group1) + (float4(this.scalar) * other.group1), 
            /* e5 */
            (other.scalar * this.e5) + (other.e5 * this.scalar), 
            /* e15, e25, e35, e45 */
            (float4(other.scalar) * this.group3) + (float4(other.e5) * this.group1) + (float4(this.scalar) * other.group3) - (float4(this.e5) * other.group1), 
            /* e41, e42, e43 */
            (float3(other.scalar) * this.group4.xyz) + (float3(this.scalar) * other.group4.xyz) + (float3(this.e4) * other.group1.xyz) - (float3(other.e4) * this.group1.xyz), 
            /* e23, e31, e12 */
            (float3(other.scalar) * this.group5.xyz) + (float3(this.scalar) * other.group5.xyz) + (other.group1.zxy * this.group1.yzx) - (other.group1.yzx * this.group1.zxy), 
            /* e415, e425, e435, e321 */
            float4((other.e4 * this.e15) + (other.e5 * this.e41) + (other.e15 * this.e4) + (other.e41 * this.e5), (other.e4 * this.e25) + (other.e5 * this.e42) + (other.e25 * this.e4) + (other.e42 * this.e5), (other.e4 * this.e35) + (other.e5 * this.e43) + (other.e35 * this.e4) + (other.e43 * this.e5), -(other.e1 * this.e23) - (other.e2 * this.e31) - (other.e3 * this.e12) - (other.e12 * this.e3)) + (float4(other.scalar) * this.group6) + (float4(this.scalar) * other.group6) - (this.group1.xyzy * float4(other.group3.www, other.e31)) - float4((this.group3.www * other.group1.xyz), other.e23 * this.e1), 
            /* e423, e431, e412 */
            (float3(other.scalar) * this.group7.xyz) + (float3(other.e4) * this.group5.xyz) + (float3(this.scalar) * other.group7.xyz) + (float3(this.e4) * other.group5.xyz) + (other.group4.xyz.yzx * this.group1.zxy) + (this.group4.xyz.yzx * other.group1.zxy) - (other.group4.xyz.zxy * this.group1.yzx) - (this.group4.xyz.zxy * other.group1.yzx), 
            /* e235, e315, e125 */
            (float3(other.scalar) * this.group8.xyz) + (float3(other.e5) * this.group5.xyz) + (float3(this.scalar) * other.group8.xyz) + (float3(this.e5) * other.group5.xyz) + (other.group1.yzx * this.group3.zxy) + (other.group3.zxy * this.group1.yzx) - (other.group1.zxy * this.group3.yzx) - (other.group3.yzx * this.group1.zxy), 
            /* e4235, e4315, e4125, e3215 */
            float4((other.e2 * this.e435) + (other.e5 * this.e423) + (other.e45 * this.e23) + (other.e425 * this.e3), (other.e3 * this.e415) + (other.e5 * this.e431) + (other.e45 * this.e31) + (other.e435 * this.e1), (other.e1 * this.e425) + (other.e5 * this.e412) + (other.e45 * this.e12) + (other.e415 * this.e2), -(other.e15 * this.e23) - (other.e25 * this.e31) - (other.e35 * this.e12) - (other.e321 * this.e5)) + (float4(other.scalar) * this.group9) + (float4(this.scalar) * other.group9) + float4((other.group5.xyz * this.group3.www), other.e2 * this.e315) + float4((other.group8.xyz * this.group1.www), other.e3 * this.e125) + float4((other.group4.xyz.yzx * this.group3.zxy), other.e1 * this.e235) + float4((this.group4.xyz.yzx * other.group3.zxy), other.e5 * this.e321) - (this.group1.yzxz * float4(other.group6.zxy, other.e125)) - (this.group3.yzxx * float4(other.group4.xyz.zxy, other.e23)) - float4((float3(this.e5) * other.group7.xyz), other.e31 * this.e25) - float4((this.group8.xyz * other.group1.www), other.e235 * this.e1) - float4((this.group4.xyz.zxy * other.group3.yzx), other.e12 * this.e35) - float4((other.group1.zxy * this.group6.yzx), other.e315 * this.e2), 
            /* e1234 */
            (other.scalar * this.e1234) + (other.e321 * this.e4) + (other.e423 * this.e1) + (other.e431 * this.e2) + (other.e412 * this.e3) + (other.e1234 * this.scalar) - (other.e1 * this.e423) - (other.e2 * this.e431) - (other.e3 * this.e412) - (other.e4 * this.e321) - (other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43)
        );
    }
}
public extension MultiVector {
    public MultiVector wedge(Plane other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, (this.e1 * other.e4235) + (this.e2 * other.e4315) + (this.e3 * other.e4125) + (this.e4 * other.e3215)), 
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
            float4(this.scalar) * other.group0, 
            /* e1234 */
            0.0
        );
    }
}
public extension MultiVector {
    public MultiVector wedge(RoundPoint other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, (this.e4235 * other.e1) + (this.e4315 * other.e2) + (this.e4125 * other.e3) + (this.e3215 * other.e4) + (this.e1234 * other.e5)), 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * other.group0, 
            /* e5 */
            this.scalar * other.e5, 
            /* e15, e25, e35, e45 */
            (float4(other.e5) * this.group1) - (float4(this.e5) * other.group0), 
            /* e41, e42, e43 */
            (float3(this.e4) * other.group0.xyz) - (float3(other.e4) * this.group1.xyz), 
            /* e23, e31, e12 */
            (this.group1.yzx * other.group0.zxy) - (this.group1.zxy * other.group0.yzx), 
            /* e415, e425, e435, e321 */
            float4((this.e15 * other.e4) + (this.e41 * other.e5), (this.e25 * other.e4) + (this.e42 * other.e5), (this.e35 * other.e4) + (this.e43 * other.e5), -(this.e31 * other.e2) - (this.e12 * other.e3)) - (other.group0.xyzx * float4(this.group3.www, this.e23)), 
            /* e423, e431, e412 */
            (float3(other.e4) * this.group5.xyz) + (this.group4.xyz.yzx * other.group0.zxy) - (this.group4.xyz.zxy * other.group0.yzx), 
            /* e235, e315, e125 */
            (float3(other.e5) * this.group5.xyz) + (this.group3.zxy * other.group0.yzx) - (this.group3.yzx * other.group0.zxy), 
            /* e4235, e4315, e4125, e3215 */
            float4(-(this.e425 * other.e3) - (this.e235 * other.e4), -(this.e435 * other.e1) - (this.e315 * other.e4), -(this.e415 * other.e2) - (this.e125 * other.e4), (this.e321 * other.e5) + (this.e125 * other.e3)) + (other.group0.yzxy * float4(this.group6.zxy, this.e315)) + float4((float3(other.e5) * this.group7.xyz), this.e235 * other.e1), 
            /* e1234 */
            -(this.e321 * other.e4) - (this.e423 * other.e1) - (this.e431 * other.e2) - (this.e412 * other.e3)
        );
    }
}
public extension MultiVector {
    public MultiVector wedge(Scalar other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(other.scalar) * this.group0.xy, 
            /* e1, e2, e3, e4 */
            float4(other.scalar) * this.group1, 
            /* e5 */
            this.e5 * other.scalar, 
            /* e15, e25, e35, e45 */
            float4(other.scalar) * this.group3, 
            /* e41, e42, e43 */
            float3(other.scalar) * this.group4.xyz, 
            /* e23, e31, e12 */
            float3(other.scalar) * this.group5.xyz, 
            /* e415, e425, e435, e321 */
            float4(other.scalar) * this.group6, 
            /* e423, e431, e412 */
            float3(other.scalar) * this.group7.xyz, 
            /* e235, e315, e125 */
            float3(other.scalar) * this.group8.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(other.scalar) * this.group9, 
            /* e1234 */
            this.e1234 * other.scalar
        );
    }
}
public extension MultiVector {
    public MultiVector wedge(Sphere other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, (this.e1 * other.e4235) + (this.e2 * other.e4315) + (this.e3 * other.e4125) + (this.e4 * other.e3215) + (this.e5 * other.e1234)), 
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
            float4(this.scalar) * other.group0, 
            /* e1234 */
            this.scalar * other.e1234
        );
    }
}
public extension MultiVector {
    public MultiVector wedge(VersorEven other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, (this.scalar * other.e12345) + (this.e4235 * other.e1) + (this.e4315 * other.e2) + (this.e4125 * other.e3) + (this.e3215 * other.e4) + (this.e1234 * other.e5) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412) - (this.e45 * other.e321) - (this.e41 * other.e235) - (this.e42 * other.e315) - (this.e43 * other.e125) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435)), 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * other.group3, 
            /* e5 */
            this.scalar * other.e5, 
            /* e15, e25, e35, e45 */
            (float4(other.e5) * this.group1) - (float4(this.e5) * other.group3), 
            /* e41, e42, e43 */
            (float3(this.e4) * other.group3.xyz) - (float3(other.e4) * this.group1.xyz), 
            /* e23, e31, e12 */
            (this.group1.yzx * other.group3.zxy) - (this.group1.zxy * other.group3.yzx), 
            /* e415, e425, e435, e321 */
            float4((this.e15 * other.e4) + (this.e41 * other.e5), (this.e25 * other.e4) + (this.e42 * other.e5), (this.e35 * other.e4) + (this.e43 * other.e5), -(this.e31 * other.e2) - (this.e12 * other.e3)) + (float4(this.scalar) * other.group1) - (other.group3.xyzx * float4(this.group3.www, this.e23)), 
            /* e423, e431, e412 */
            (float3(this.scalar) * other.group0.xyz) + (float3(other.e4) * this.group5.xyz) + (this.group4.xyz.yzx * other.group3.zxy) - (this.group4.xyz.zxy * other.group3.yzx), 
            /* e235, e315, e125 */
            (float3(this.scalar) * other.group2.xyz) + (float3(other.e5) * this.group5.xyz) + (this.group3.zxy * other.group3.yzx) - (this.group3.yzx * other.group3.zxy), 
            /* e4235, e4315, e4125, e3215 */
            (this.group6.zxyw * float4(other.group3.yzx, other.e5)) + float4((this.group7.xyz * other.group2.www), this.e235 * other.e1) + float4((this.group1.zxy * other.group1.yzx), this.e315 * other.e2) + float4((this.group1.www * other.group2.xyz), this.e125 * other.e3) - (float4(this.e5) * float4(other.group0.xyz, other.e321)) - (this.group1.yzxy * float4(other.group1.zxy, other.e315)) - float4((this.group8.xyz * other.group3.www), this.e1 * other.e235) - float4((this.group6.yzx * other.group3.zxy), this.e3 * other.e125), 
            /* e1234 */
            (this.e1 * other.e423) + (this.e2 * other.e431) + (this.e3 * other.e412) + (this.e4 * other.e321) - (this.e321 * other.e4) - (this.e423 * other.e1) - (this.e431 * other.e2) - (this.e412 * other.e3)
        );
    }
}
public extension MultiVector {
    public MultiVector wedge(VersorOdd other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(this.scalar * other.scalar, (this.e12345 * other.scalar) + (this.e1 * other.e4235) + (this.e2 * other.e4315) + (this.e3 * other.e4125) + (this.e4 * other.e3215) + (this.e5 * other.e1234) - (this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12) - (this.e321 * other.e45) - (this.e423 * other.e15) - (this.e431 * other.e25) - (this.e412 * other.e35) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43)), 
            /* e1, e2, e3, e4 */
            float4(other.scalar) * this.group1, 
            /* e5 */
            this.e5 * other.scalar, 
            /* e15, e25, e35, e45 */
            (float4(this.scalar) * float4(other.group2.xyz, other.e45)) + (float4(other.scalar) * this.group3), 
            /* e41, e42, e43 */
            (float3(this.scalar) * other.group0.xyz) + (float3(other.scalar) * this.group4.xyz), 
            /* e23, e31, e12 */
            (float3(this.scalar) * other.group1.xyz) + (float3(other.scalar) * this.group5.xyz), 
            /* e415, e425, e435, e321 */
            float4((this.e5 * other.e41) + (this.e415 * other.scalar), (this.e5 * other.e42) + (this.e425 * other.scalar), (this.e5 * other.e43) + (this.e435 * other.scalar), -(this.e2 * other.e31) - (this.e3 * other.e12)) + float4((this.group1.www * other.group2.xyz), this.e321 * other.scalar) - (this.group1.xyzx * other.group1.wwwx), 
            /* e423, e431, e412 */
            (float3(this.e4) * other.group1.xyz) + (float3(other.scalar) * this.group7.xyz) + (this.group1.zxy * other.group0.yzx) - (this.group1.yzx * other.group0.zxy), 
            /* e235, e315, e125 */
            (float3(this.e5) * other.group1.xyz) + (float3(other.scalar) * this.group8.xyz) + (this.group1.yzx * other.group2.zxy) - (this.group1.zxy * other.group2.yzx), 
            /* e4235, e4315, e4125, e3215 */
            float4((this.e35 * other.e42) + (this.e45 * other.e23) + (this.e23 * other.e45) + (this.e4235 * other.scalar), (this.e15 * other.e43) + (this.e45 * other.e31) + (this.e31 * other.e45) + (this.e4315 * other.scalar), (this.e25 * other.e41) + (this.e45 * other.e12) + (this.e12 * other.e45) + (this.e4125 * other.scalar), -(this.e15 * other.e23) - (this.e25 * other.e31) - (this.e35 * other.e12) - (this.e12 * other.e35)) + (float4(this.scalar) * other.group3) + float4((this.group4.xyz.yzx * other.group2.zxy), this.e3215 * other.scalar) - (other.group2.yzxx * float4(this.group4.xyz.zxy, this.e23)) - float4((this.group3.yzx * other.group0.zxy), this.e31 * other.e25), 
            /* e1234 */
            (this.scalar * other.e1234) + (this.e1234 * other.scalar) - (this.e41 * other.e23) - (this.e42 * other.e31) - (this.e43 * other.e12) - (this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43)
        );
    }
}
public extension Plane {
    // Fancy infix trick (first half)
    public WedgeInfixPartial<Plane> operator /(WedgeInfix rhs) {
        return WedgeInfixPartial<Plane>(this);
    }
}
public extension WedgeInfixPartial<Plane> {
    // Fancy infix trick (second half)
    public Plane operator /(AntiCircleRotor rhs) {
        return this.a.wedge(rhs);
    }
}
public extension Plane {
    public Plane wedge(AntiCircleRotor other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(other.scalar) * this.group0
        );
    }
}
public extension Plane {
    public AntiScalar wedge(AntiDipoleInversion other) {
        return AntiScalar(
            /* e12345 */
            (other.e4 * this.e3215) + (other.e1 * this.e4235) + (other.e2 * this.e4315) + (other.e3 * this.e4125)
        );
    }
}
public extension Plane {
    public Plane wedge(AntiDualNum other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(other.scalar) * this.group0
        );
    }
}
public extension Plane {
    public AntiScalar wedge(AntiFlector other) {
        return AntiScalar(
            /* e12345 */
            (other.e1 * this.e4235) + (other.e2 * this.e4315) + (other.e3 * this.e4125)
        );
    }
}
public extension Plane {
    public Plane wedge(AntiMotor other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(other.scalar) * this.group0
        );
    }
}
public extension Plane {
    public AntiScalar wedge(AntiPlane other) {
        return AntiScalar(
            /* e12345 */
            (other.e1 * this.e4235) + (other.e2 * this.e4315) + (other.e3 * this.e4125)
        );
    }
}
public extension Plane {
    public MultiVector wedge(MultiVector other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, (other.e1 * this.e4235) + (other.e2 * this.e4315) + (other.e3 * this.e4125) + (other.e4 * this.e3215)), 
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
            float4(other.scalar) * this.group0, 
            /* e1234 */
            0.0
        );
    }
}
public extension Plane {
    public AntiScalar wedge(RoundPoint other) {
        return AntiScalar(
            /* e12345 */
            (this.e4235 * other.e1) + (this.e4315 * other.e2) + (this.e4125 * other.e3) + (this.e3215 * other.e4)
        );
    }
}
public extension Plane {
    public Plane wedge(Scalar other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(other.scalar) * this.group0
        );
    }
}
public extension Plane {
    public AntiScalar wedge(VersorEven other) {
        return AntiScalar(
            /* e12345 */
            (this.e4235 * other.e1) + (this.e4315 * other.e2) + (this.e4125 * other.e3) + (this.e3215 * other.e4)
        );
    }
}
public extension Plane {
    public Plane wedge(VersorOdd other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(other.scalar) * this.group0
        );
    }
}
public extension RoundPoint {
    // Fancy infix trick (first half)
    public WedgeInfixPartial<RoundPoint> operator /(WedgeInfix rhs) {
        return WedgeInfixPartial<RoundPoint>(this);
    }
}
public extension WedgeInfixPartial<RoundPoint> {
    // Fancy infix trick (second half)
    public AntiDipoleInversion operator /(AntiCircleRotor rhs) {
        return this.a.wedge(rhs);
    }
}
public extension RoundPoint {
    public AntiDipoleInversion wedge(AntiCircleRotor other) {
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (float3(this.e4) * other.group1.xyz) + (other.group0.xyz.yzx * this.group0.zxy) - (other.group0.xyz.zxy * this.group0.yzx), 
            /* e415, e425, e435, e321 */
            float4((other.e41 * this.e5) + (other.e15 * this.e4), (other.e42 * this.e5) + (other.e25 * this.e4), (other.e43 * this.e5) + (other.e35 * this.e4), -(other.e31 * this.e2) - (other.e12 * this.e3)) - (other.group1.wwwx * this.group0.xyzx), 
            /* e235, e315, e125, e4 */
            float4(((float3(this.e5) * other.group1.xyz) + (other.group2.zxy * this.group0.yzx) - (other.group2.yzx * this.group0.zxy)), other.scalar * this.e4), 
            /* e1, e2, e3, e5 */
            float4(other.scalar) * float4(this.group0.xyz, this.e5)
        );
    }
}
public extension RoundPoint {
    public DipoleInversion wedge(AntiDipoleInversion other) {
        return DipoleInversion(
            /* e41, e42, e43 */
            (float3(this.e4) * other.group3.xyz) - (float3(other.e4) * this.group0.xyz), 
            /* e23, e31, e12, e45 */
            (other.group3.zxyw * this.group0.yzxw) - float4((other.group3.yzx * this.group0.zxy), other.e4 * this.e5), 
            /* e15, e25, e35, e1234 */
            float4(other.e1 * this.e5 * -1.0, other.e2 * this.e5 * -1.0, other.e3 * this.e5 * -1.0, (other.e431 * this.e2) + (other.e412 * this.e3) + (other.e321 * this.e4)) + (this.group0.xyzx * float4(other.group3.www, other.e423)), 
            /* e4235, e4315, e4125, e3215 */
            float4((other.e425 * this.e3) + (other.e235 * this.e4), (other.e435 * this.e1) + (other.e315 * this.e4), (other.e415 * this.e2) + (other.e125 * this.e4), -(other.e315 * this.e2) - (other.e125 * this.e3)) - (float4(this.e5) * float4(other.group0.xyz, other.e321)) - (this.group0.yzxx * float4(other.group1.zxy, other.e235))
        );
    }
}
public extension RoundPoint {
    public VersorEven wedge(AntiDualNum other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(float3(0.0), other.e3215 * this.e4), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e235, e315, e125, e5 */
            float4(float3(0.0), other.scalar * this.e5), 
            /* e1, e2, e3, e4 */
            float4(other.scalar) * this.group0
        );
    }
}
public extension RoundPoint {
    public Sphere wedge(AntiFlatPoint other) {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((float3(this.e4) * other.group0.xyz), -(other.e235 * this.e1) - (other.e315 * this.e2) - (other.e125 * this.e3) - (other.e321 * this.e5)), 
            /* e1234 */
            other.e321 * this.e4
        );
    }
}
public extension RoundPoint {
    public DipoleInversion wedge(AntiFlector other) {
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(this.e4) * other.group1.xyz, 
            /* e23, e31, e12, e45 */
            float4(((other.group1.zxy * this.group0.yzx) - (other.group1.yzx * this.group0.zxy)), other.e5 * this.e4), 
            /* e15, e25, e35, e1234 */
            float4(((float3(other.e5) * this.group0.xyz) - (float3(this.e5) * other.group1.xyz)), other.e321 * this.e4), 
            /* e4235, e4315, e4125, e3215 */
            float4((float3(this.e4) * other.group0.xyz), -(other.e235 * this.e1) - (other.e315 * this.e2) - (other.e125 * this.e3) - (other.e321 * this.e5))
        );
    }
}
public extension RoundPoint {
    public Circle wedge(AntiLine other) {
        return Circle(
            /* e423, e431, e412 */
            float3(this.e4) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4((float3(this.e4) * other.group1.xyz), -(other.e23 * this.e1) - (other.e31 * this.e2) - (other.e12 * this.e3)), 
            /* e235, e315, e125 */
            (float3(this.e5) * other.group0.xyz) + (other.group1.xyz.zxy * this.group0.yzx) - (other.group1.xyz.yzx * this.group0.zxy)
        );
    }
}
public extension RoundPoint {
    public VersorEven wedge(AntiMotor other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(this.e4) * float4(other.group0.xyz, other.e3215), 
            /* e415, e425, e435, e321 */
            float4((float3(this.e4) * other.group1.xyz), -(other.e23 * this.e1) - (other.e31 * this.e2) - (other.e12 * this.e3)), 
            /* e235, e315, e125, e5 */
            float4(((float3(this.e5) * other.group0.xyz) + (other.group1.zxy * this.group0.yzx) - (other.group1.yzx * this.group0.zxy)), other.scalar * this.e5), 
            /* e1, e2, e3, e4 */
            float4(other.scalar) * this.group0
        );
    }
}
public extension RoundPoint {
    public Dipole wedge(AntiPlane other) {
        return Dipole(
            /* e41, e42, e43 */
            float3(this.e4) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(((other.group0.zxy * this.group0.yzx) - (other.group0.yzx * this.group0.zxy)), other.e5 * this.e4), 
            /* e15, e25, e35 */
            (float3(other.e5) * this.group0.xyz) - (float3(this.e5) * other.group0.xyz)
        );
    }
}
public extension RoundPoint {
    public Sphere wedge(Circle other) {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((other.e425 * this.e3) + (other.e235 * this.e4), (other.e435 * this.e1) + (other.e315 * this.e4), (other.e415 * this.e2) + (other.e125 * this.e4), -(other.e321 * this.e5) - (other.e125 * this.e3)) - (this.group0.yzxy * float4(other.group1.zxy, other.e315)) - float4((float3(this.e5) * other.group0.xyz), other.e235 * this.e1), 
            /* e1234 */
            (other.e423 * this.e1) + (other.e431 * this.e2) + (other.e412 * this.e3) + (other.e321 * this.e4)
        );
    }
}
public extension RoundPoint {
    public Sphere wedge(CircleRotor other) {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((other.e425 * this.e3) + (other.e235 * this.e4), (other.e435 * this.e1) + (other.e315 * this.e4), (other.e415 * this.e2) + (other.e125 * this.e4), -(other.e315 * this.e2) - (other.e125 * this.e3)) - (float4(this.e5) * float4(other.group0.xyz, other.e321)) - (this.group0.yzxx * float4(other.group1.zxy, other.e235)), 
            /* e1234 */
            (other.e423 * this.e1) + (other.e431 * this.e2) + (other.e412 * this.e3) + (other.e321 * this.e4)
        );
    }
}
public extension RoundPoint {
    public Circle wedge(Dipole other) {
        return Circle(
            /* e423, e431, e412 */
            (float3(this.e4) * other.group1.xyz) + (other.group0.xyz.yzx * this.group0.zxy) - (other.group0.xyz.zxy * this.group0.yzx), 
            /* e415, e425, e435, e321 */
            float4((other.e41 * this.e5) + (other.e15 * this.e4), (other.e42 * this.e5) + (other.e25 * this.e4), (other.e43 * this.e5) + (other.e35 * this.e4), -(other.e31 * this.e2) - (other.e12 * this.e3)) - (other.group1.wwwx * this.group0.xyzx), 
            /* e235, e315, e125 */
            (float3(this.e5) * other.group1.xyz) + (other.group2.xyz.zxy * this.group0.yzx) - (other.group2.xyz.yzx * this.group0.zxy)
        );
    }
}
public extension RoundPoint {
    public CircleRotor wedge(DipoleInversion other) {
        return CircleRotor(
            /* e423, e431, e412 */
            (float3(this.e4) * other.group1.xyz) + (other.group0.xyz.yzx * this.group0.zxy) - (other.group0.xyz.zxy * this.group0.yzx), 
            /* e415, e425, e435, e321 */
            float4((other.e41 * this.e5) + (other.e15 * this.e4), (other.e42 * this.e5) + (other.e25 * this.e4), (other.e43 * this.e5) + (other.e35 * this.e4), -(other.e31 * this.e2) - (other.e12 * this.e3)) - (other.group1.wwwx * this.group0.xyzx), 
            /* e235, e315, e125, e12345 */
            float4(other.e25 * this.e3 * -1.0, other.e35 * this.e1 * -1.0, other.e15 * this.e2 * -1.0, (other.e4315 * this.e2) + (other.e4125 * this.e3) + (other.e3215 * this.e4)) + (float4(this.e5) * float4(other.group1.xyz, other.e1234)) + (this.group0.yzxx * float4(other.group2.zxy, other.e4235))
        );
    }
}
public extension RoundPoint {
    public FlatPoint wedge(DualNum other) {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(other.e5) * this.group0
        );
    }
}
public extension RoundPoint {
    public Line wedge(FlatPoint other) {
        return Line(
            /* e415, e425, e435 */
            (float3(this.e4) * other.group0.xyz) - (float3(other.e45) * this.group0.xyz), 
            /* e235, e315, e125 */
            (other.group0.zxy * this.group0.yzx) - (other.group0.yzx * this.group0.zxy)
        );
    }
}
public extension RoundPoint {
    public Motor wedge(Flector other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(other.e45 * this.e1 * -1.0, other.e45 * this.e2 * -1.0, other.e45 * this.e3 * -1.0, (other.e4315 * this.e2) + (other.e4125 * this.e3) + (other.e3215 * this.e4)) + (this.group0.wwwx * float4(other.group0.xyz, other.e4235)), 
            /* e235, e315, e125, e5 */
            float4(((other.group0.zxy * this.group0.yzx) - (other.group0.yzx * this.group0.zxy)), 0.0)
        );
    }
}
public extension RoundPoint {
    public Plane wedge(Line other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4((other.e425 * this.e3) + (other.e235 * this.e4), (other.e435 * this.e1) + (other.e315 * this.e4), (other.e415 * this.e2) + (other.e125 * this.e4), -(other.e315 * this.e2) - (other.e125 * this.e3)) - (this.group0.yzxx * float4(other.group0.xyz.zxy, other.e235))
        );
    }
}
public extension RoundPoint {
    public Flector wedge(Motor other) {
        return Flector(
            /* e15, e25, e35, e45 */
            float4(other.e5) * this.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4((other.e425 * this.e3) + (other.e235 * this.e4), (other.e435 * this.e1) + (other.e315 * this.e4), (other.e415 * this.e2) + (other.e125 * this.e4), -(other.e315 * this.e2) - (other.e125 * this.e3)) - (this.group0.yzxx * float4(other.group0.zxy, other.e235))
        );
    }
}
public extension RoundPoint {
    public MultiVector wedge(MultiVector other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, (other.e4235 * this.e1) + (other.e4315 * this.e2) + (other.e4125 * this.e3) + (other.e3215 * this.e4) + (other.e1234 * this.e5)), 
            /* e1, e2, e3, e4 */
            float4(other.scalar) * this.group0, 
            /* e5 */
            other.scalar * this.e5, 
            /* e15, e25, e35, e45 */
            (float4(other.e5) * this.group0) - (float4(this.e5) * other.group1), 
            /* e41, e42, e43 */
            (float3(this.e4) * other.group1.xyz) - (float3(other.e4) * this.group0.xyz), 
            /* e23, e31, e12 */
            (other.group1.zxy * this.group0.yzx) - (other.group1.yzx * this.group0.zxy), 
            /* e415, e425, e435, e321 */
            float4((other.e15 * this.e4) + (other.e41 * this.e5), (other.e25 * this.e4) + (other.e42 * this.e5), (other.e35 * this.e4) + (other.e43 * this.e5), -(other.e31 * this.e2) - (other.e12 * this.e3)) - (this.group0.xyzx * float4(other.group3.www, other.e23)), 
            /* e423, e431, e412 */
            (float3(this.e4) * other.group5.xyz) + (other.group4.xyz.yzx * this.group0.zxy) - (other.group4.xyz.zxy * this.group0.yzx), 
            /* e235, e315, e125 */
            (float3(this.e5) * other.group5.xyz) + (other.group3.zxy * this.group0.yzx) - (other.group3.yzx * this.group0.zxy), 
            /* e4235, e4315, e4125, e3215 */
            float4((other.e425 * this.e3) + (other.e235 * this.e4), (other.e435 * this.e1) + (other.e315 * this.e4), (other.e415 * this.e2) + (other.e125 * this.e4), -(other.e321 * this.e5) - (other.e125 * this.e3)) - (this.group0.yzxy * float4(other.group6.zxy, other.e315)) - float4((float3(this.e5) * other.group7.xyz), other.e235 * this.e1), 
            /* e1234 */
            (other.e321 * this.e4) + (other.e423 * this.e1) + (other.e431 * this.e2) + (other.e412 * this.e3)
        );
    }
}
public extension RoundPoint {
    public AntiScalar wedge(Plane other) {
        return AntiScalar(
            /* e12345 */
            (other.e4235 * this.e1) + (other.e4315 * this.e2) + (other.e4125 * this.e3) + (other.e3215 * this.e4)
        );
    }
}
public extension RoundPoint {
    public Dipole wedge(RoundPoint other) {
        return Dipole(
            /* e41, e42, e43 */
            (float3(this.e4) * other.group0.xyz) - (float3(other.e4) * this.group0.xyz), 
            /* e23, e31, e12, e45 */
            float4((other.group0.zxy * this.group0.yzx), other.e5 * this.e4) - (other.group0.yzxw * float4(this.group0.zxy, this.e5)), 
            /* e15, e25, e35 */
            (float3(other.e5) * this.group0.xyz) - (float3(this.e5) * other.group0.xyz)
        );
    }
}
public extension RoundPoint {
    public RoundPoint wedge(Scalar other) {
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(other.scalar) * this.group0, 
            /* e5 */
            this.e5 * other.scalar
        );
    }
}
public extension RoundPoint {
    public AntiScalar wedge(Sphere other) {
        return AntiScalar(
            /* e12345 */
            (this.e1 * other.e4235) + (this.e2 * other.e4315) + (this.e3 * other.e4125) + (this.e4 * other.e3215) + (this.e5 * other.e1234)
        );
    }
}
public extension RoundPoint {
    public DipoleInversion wedge(VersorEven other) {
        return DipoleInversion(
            /* e41, e42, e43 */
            (float3(this.e4) * other.group3.xyz) - (float3(other.e4) * this.group0.xyz), 
            /* e23, e31, e12, e45 */
            (this.group0.yzxw * float4(other.group3.zxy, other.e5)) - float4((this.group0.zxy * other.group3.yzx), this.e5 * other.e4), 
            /* e15, e25, e35, e1234 */
            float4(this.e5 * other.e1 * -1.0, this.e5 * other.e2 * -1.0, this.e5 * other.e3 * -1.0, (this.e2 * other.e431) + (this.e3 * other.e412) + (this.e4 * other.e321)) + (this.group0.xyzx * float4(other.group2.www, other.e423)), 
            /* e4235, e4315, e4125, e3215 */
            float4((this.e3 * other.e425) + (this.e4 * other.e235), (this.e1 * other.e435) + (this.e4 * other.e315), (this.e2 * other.e415) + (this.e4 * other.e125), -(this.e3 * other.e125) - (this.e5 * other.e321)) - (this.group0.yzxx * float4(other.group1.zxy, other.e235)) - float4((float3(this.e5) * other.group0.xyz), this.e2 * other.e315)
        );
    }
}
public extension RoundPoint {
    public VersorEven wedge(VersorOdd other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(this.e2 * other.e43 * -1.0, this.e3 * other.e41 * -1.0, this.e1 * other.e42 * -1.0, (this.e3 * other.e4125) + (this.e4 * other.e3215) + (this.e5 * other.e1234)) + (this.group0.zxyx * float4(other.group0.yzx, other.e4235)) + (this.group0.wwwy * float4(other.group1.xyz, other.e4315)), 
            /* e415, e425, e435, e321 */
            float4((this.e4 * other.e15) + (this.e5 * other.e41), (this.e4 * other.e25) + (this.e5 * other.e42), (this.e4 * other.e35) + (this.e5 * other.e43), -(this.e2 * other.e31) - (this.e3 * other.e12)) - (this.group0.xyzx * other.group1.wwwx), 
            /* e235, e315, e125, e5 */
            float4(((float3(this.e5) * other.group1.xyz) + (this.group0.yzx * other.group2.zxy) - (this.group0.zxy * other.group2.yzx)), this.e5 * other.scalar), 
            /* e1, e2, e3, e4 */
            float4(other.scalar) * this.group0
        );
    }
}
public extension Scalar {
    // Fancy infix trick (first half)
    public WedgeInfixPartial<Scalar> operator /(WedgeInfix rhs) {
        return WedgeInfixPartial<Scalar>(this);
    }
}
public extension WedgeInfixPartial<Scalar> {
    // Fancy infix trick (second half)
    public AntiCircleRotor operator /(AntiCircleRotor rhs) {
        return this.a.wedge(rhs);
    }
}
public extension Scalar {
    public AntiCircleRotor wedge(AntiCircleRotor other) {
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(this.scalar) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(this.scalar) * other.group1, 
            /* e15, e25, e35, scalar */
            float4(this.scalar) * other.group2
        );
    }
}
public extension Scalar {
    public AntiDipoleInversion wedge(AntiDipoleInversion other) {
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(this.scalar) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(this.scalar) * other.group1, 
            /* e235, e315, e125, e4 */
            float4(this.scalar) * other.group2, 
            /* e1, e2, e3, e5 */
            float4(this.scalar) * other.group3
        );
    }
}
public extension Scalar {
    public AntiDualNum wedge(AntiDualNum other) {
        return AntiDualNum(
            /* e3215, scalar */
            float2(this.scalar) * other.group0.xy
        );
    }
}
public extension Scalar {
    public AntiFlatPoint wedge(AntiFlatPoint other) {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(this.scalar) * other.group0
        );
    }
}
public extension Scalar {
    public AntiFlector wedge(AntiFlector other) {
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(this.scalar) * other.group0, 
            /* e1, e2, e3, e5 */
            float4(this.scalar) * other.group1
        );
    }
}
public extension Scalar {
    public AntiLine wedge(AntiLine other) {
        return AntiLine(
            /* e23, e31, e12 */
            float3(this.scalar) * other.group0.xyz, 
            /* e15, e25, e35 */
            float3(this.scalar) * other.group1.xyz
        );
    }
}
public extension Scalar {
    public AntiMotor wedge(AntiMotor other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(this.scalar) * other.group0, 
            /* e15, e25, e35, e3215 */
            float4(this.scalar) * other.group1
        );
    }
}
public extension Scalar {
    public AntiPlane wedge(AntiPlane other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(this.scalar) * other.group0
        );
    }
}
public extension Scalar {
    public AntiScalar wedge(AntiScalar other) {
        return AntiScalar(
            /* e12345 */
            other.e12345 * this.scalar
        );
    }
}
public extension Scalar {
    public Circle wedge(Circle other) {
        return Circle(
            /* e423, e431, e412 */
            float3(this.scalar) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(this.scalar) * other.group1, 
            /* e235, e315, e125 */
            float3(this.scalar) * other.group2.xyz
        );
    }
}
public extension Scalar {
    public CircleRotor wedge(CircleRotor other) {
        return CircleRotor(
            /* e423, e431, e412 */
            float3(this.scalar) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(this.scalar) * other.group1, 
            /* e235, e315, e125, e12345 */
            float4(this.scalar) * other.group2
        );
    }
}
public extension Scalar {
    public Dipole wedge(Dipole other) {
        return Dipole(
            /* e41, e42, e43 */
            float3(this.scalar) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(this.scalar) * other.group1, 
            /* e15, e25, e35 */
            float3(this.scalar) * other.group2.xyz
        );
    }
}
public extension Scalar {
    public DipoleInversion wedge(DipoleInversion other) {
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(this.scalar) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(this.scalar) * other.group1, 
            /* e15, e25, e35, e1234 */
            float4(this.scalar) * other.group2, 
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar) * other.group3
        );
    }
}
public extension Scalar {
    public DualNum wedge(DualNum other) {
        return DualNum(
            /* e5, e12345 */
            float2(this.scalar) * other.group0.xy
        );
    }
}
public extension Scalar {
    public FlatPoint wedge(FlatPoint other) {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(this.scalar) * other.group0
        );
    }
}
public extension Scalar {
    public Flector wedge(Flector other) {
        return Flector(
            /* e15, e25, e35, e45 */
            float4(this.scalar) * other.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar) * other.group1
        );
    }
}
public extension Scalar {
    public Line wedge(Line other) {
        return Line(
            /* e415, e425, e435 */
            float3(this.scalar) * other.group0.xyz, 
            /* e235, e315, e125 */
            float3(this.scalar) * other.group1.xyz
        );
    }
}
public extension Scalar {
    public Motor wedge(Motor other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(this.scalar) * other.group0, 
            /* e235, e315, e125, e5 */
            float4(this.scalar) * other.group1
        );
    }
}
public extension Scalar {
    public MultiVector wedge(MultiVector other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(this.scalar) * other.group0.xy, 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * other.group1, 
            /* e5 */
            other.e5 * this.scalar, 
            /* e15, e25, e35, e45 */
            float4(this.scalar) * other.group3, 
            /* e41, e42, e43 */
            float3(this.scalar) * other.group4.xyz, 
            /* e23, e31, e12 */
            float3(this.scalar) * other.group5.xyz, 
            /* e415, e425, e435, e321 */
            float4(this.scalar) * other.group6, 
            /* e423, e431, e412 */
            float3(this.scalar) * other.group7.xyz, 
            /* e235, e315, e125 */
            float3(this.scalar) * other.group8.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar) * other.group9, 
            /* e1234 */
            other.e1234 * this.scalar
        );
    }
}
public extension Scalar {
    public Plane wedge(Plane other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar) * other.group0
        );
    }
}
public extension Scalar {
    public RoundPoint wedge(RoundPoint other) {
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(this.scalar) * other.group0, 
            /* e5 */
            other.e5 * this.scalar
        );
    }
}
public extension Scalar {
    public Scalar wedge(Scalar other) {
        return Scalar(
            /* scalar */
            other.scalar * this.scalar
        );
    }
}
public extension Scalar {
    public Sphere wedge(Sphere other) {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar) * other.group0, 
            /* e1234 */
            this.scalar * other.e1234
        );
    }
}
public extension Scalar {
    public VersorEven wedge(VersorEven other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(this.scalar) * other.group0, 
            /* e415, e425, e435, e321 */
            float4(this.scalar) * other.group1, 
            /* e235, e315, e125, e5 */
            float4(this.scalar) * other.group2, 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * other.group3
        );
    }
}
public extension Scalar {
    public VersorOdd wedge(VersorOdd other) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(this.scalar) * other.group0, 
            /* e23, e31, e12, e45 */
            float4(this.scalar) * other.group1, 
            /* e15, e25, e35, e1234 */
            float4(this.scalar) * other.group2, 
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar) * other.group3
        );
    }
}
public extension Sphere {
    // Fancy infix trick (first half)
    public WedgeInfixPartial<Sphere> operator /(WedgeInfix rhs) {
        return WedgeInfixPartial<Sphere>(this);
    }
}
public extension WedgeInfixPartial<Sphere> {
    // Fancy infix trick (second half)
    public Sphere operator /(AntiCircleRotor rhs) {
        return this.a.wedge(rhs);
    }
}
public extension Sphere {
    public Sphere wedge(AntiCircleRotor other) {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(other.scalar) * this.group0, 
            /* e1234 */
            other.scalar * this.e1234
        );
    }
}
public extension Sphere {
    public AntiScalar wedge(AntiDipoleInversion other) {
        return AntiScalar(
            /* e12345 */
            (other.e4 * this.e3215) + (other.e1 * this.e4235) + (other.e2 * this.e4315) + (other.e3 * this.e4125) + (other.e5 * this.e1234)
        );
    }
}
public extension Sphere {
    public Sphere wedge(AntiDualNum other) {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(other.scalar) * this.group0, 
            /* e1234 */
            other.scalar * this.e1234
        );
    }
}
public extension Sphere {
    public AntiScalar wedge(AntiFlector other) {
        return AntiScalar(
            /* e12345 */
            (other.e1 * this.e4235) + (other.e2 * this.e4315) + (other.e3 * this.e4125) + (other.e5 * this.e1234)
        );
    }
}
public extension Sphere {
    public Sphere wedge(AntiMotor other) {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(other.scalar) * this.group0, 
            /* e1234 */
            other.scalar * this.e1234
        );
    }
}
public extension Sphere {
    public AntiScalar wedge(AntiPlane other) {
        return AntiScalar(
            /* e12345 */
            (other.e1 * this.e4235) + (other.e2 * this.e4315) + (other.e3 * this.e4125) + (other.e5 * this.e1234)
        );
    }
}
public extension Sphere {
    public AntiScalar wedge(DualNum other) {
        return AntiScalar(
            /* e12345 */
            other.e5 * this.e1234
        );
    }
}
public extension Sphere {
    public AntiScalar wedge(Motor other) {
        return AntiScalar(
            /* e12345 */
            other.e5 * this.e1234
        );
    }
}
public extension Sphere {
    public MultiVector wedge(MultiVector other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, (other.e1 * this.e4235) + (other.e2 * this.e4315) + (other.e3 * this.e4125) + (other.e4 * this.e3215) + (other.e5 * this.e1234)), 
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
            float4(other.scalar) * this.group0, 
            /* e1234 */
            other.scalar * this.e1234
        );
    }
}
public extension Sphere {
    public AntiScalar wedge(RoundPoint other) {
        return AntiScalar(
            /* e12345 */
            (other.e1 * this.e4235) + (other.e2 * this.e4315) + (other.e3 * this.e4125) + (other.e4 * this.e3215) + (other.e5 * this.e1234)
        );
    }
}
public extension Sphere {
    public Sphere wedge(Scalar other) {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(other.scalar) * this.group0, 
            /* e1234 */
            other.scalar * this.e1234
        );
    }
}
public extension Sphere {
    public AntiScalar wedge(VersorEven other) {
        return AntiScalar(
            /* e12345 */
            (this.e4235 * other.e1) + (this.e4315 * other.e2) + (this.e4125 * other.e3) + (this.e3215 * other.e4) + (this.e1234 * other.e5)
        );
    }
}
public extension Sphere {
    public Sphere wedge(VersorOdd other) {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(other.scalar) * this.group0, 
            /* e1234 */
            this.e1234 * other.scalar
        );
    }
}
public extension VersorEven {
    // Fancy infix trick (first half)
    public WedgeInfixPartial<VersorEven> operator /(WedgeInfix rhs) {
        return WedgeInfixPartial<VersorEven>(this);
    }
}
public extension WedgeInfixPartial<VersorEven> {
    // Fancy infix trick (second half)
    public VersorEven operator /(AntiCircleRotor rhs) {
        return this.a.wedge(rhs);
    }
}
public extension VersorEven {
    public VersorEven wedge(AntiCircleRotor other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4((other.e23 * this.e4) + (other.scalar * this.e423), (other.e31 * this.e4) + (other.scalar * this.e431), (other.e12 * this.e4) + (other.scalar * this.e412), -(other.e42 * this.e315) - (other.e43 * this.e125) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e45 * this.e321) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412)) + float4((other.group0.xyz.yzx * this.group3.zxy), other.scalar * this.e12345) - float4((other.group0.xyz.zxy * this.group3.yzx), other.e41 * this.e235), 
            /* e415, e425, e435, e321 */
            float4((other.e15 * this.e4) + (other.scalar * this.e415), (other.e25 * this.e4) + (other.scalar * this.e425), (other.e35 * this.e4) + (other.scalar * this.e435), -(other.e31 * this.e2) - (other.e12 * this.e3)) + float4((other.group0.xyz * this.group2.www), other.scalar * this.e321) - (other.group1.wwwx * this.group3.xyzx), 
            /* e235, e315, e125, e5 */
            float4(((float3(other.scalar) * this.group2.xyz) + (float3(this.e5) * other.group1.xyz) + (other.group2.zxy * this.group3.yzx) - (other.group2.yzx * this.group3.zxy)), other.scalar * this.e5), 
            /* e1, e2, e3, e4 */
            float4(other.scalar) * this.group3
        );
    }
}
public extension VersorEven {
    public DipoleInversion wedge(AntiDipoleInversion other) {
        return DipoleInversion(
            /* e41, e42, e43 */
            (float3(this.e4) * other.group3.xyz) - (float3(other.e4) * this.group3.xyz), 
            /* e23, e31, e12, e45 */
            (other.group3.zxyw * this.group3.yzxw) - float4((other.group3.yzx * this.group3.zxy), other.e4 * this.e5), 
            /* e15, e25, e35, e1234 */
            (this.group3.xyzx * float4(other.group3.www, other.e423)) + float4(float3(0.0), (other.e431 * this.e2) + (other.e412 * this.e3) + (other.e321 * this.e4) - (other.e1 * this.e423) - (other.e2 * this.e431) - (other.e3 * this.e412)) - float4((this.group2.www * other.group3.xyz), other.e4 * this.e321), 
            /* e4235, e4315, e4125, e3215 */
            (float4(other.e5) * float4(this.group0.xyz, this.e321)) + (other.group3.yzxz * float4(this.group1.zxy, this.e125)) + float4((other.group1.yzx * this.group3.zxy), other.e1 * this.e235) + float4((this.group3.www * other.group2.xyz), other.e2 * this.e315) - (float4(this.e5) * float4(other.group0.xyz, other.e321)) - (other.group2.wwwy * float4(this.group2.xyz, this.e2)) - (this.group3.yzxx * float4(other.group1.zxy, other.e235)) - float4((other.group3.zxy * this.group1.yzx), other.e125 * this.e3)
        );
    }
}
public extension VersorEven {
    public VersorEven wedge(AntiDualNum other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(other.group0.xy.yy, other.scalar, (other.e3215 * this.e4) + (other.scalar * this.e12345)) * float4(this.group0.xyz, 1.0), 
            /* e415, e425, e435, e321 */
            float4(other.scalar) * this.group1, 
            /* e235, e315, e125, e5 */
            float4(other.scalar) * this.group2, 
            /* e1, e2, e3, e4 */
            float4(other.scalar) * this.group3
        );
    }
}
public extension VersorEven {
    public Sphere wedge(AntiFlatPoint other) {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((float3(this.e4) * other.group0.xyz), -(other.e235 * this.e1) - (other.e315 * this.e2) - (other.e125 * this.e3) - (other.e321 * this.e5)), 
            /* e1234 */
            other.e321 * this.e4
        );
    }
}
public extension VersorEven {
    public DipoleInversion wedge(AntiFlector other) {
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(this.e4) * other.group1.xyz, 
            /* e23, e31, e12, e45 */
            float4(((other.group1.zxy * this.group3.yzx) - (other.group1.yzx * this.group3.zxy)), other.e5 * this.e4), 
            /* e15, e25, e35, e1234 */
            (this.group3 * float4(other.group1.www, other.e321)) + float4(float3(0.0), -(other.e2 * this.e431) - (other.e3 * this.e412)) - (other.group1.xyzx * float4(this.group2.www, this.e423)), 
            /* e4235, e4315, e4125, e3215 */
            (other.group1.yzxy * float4(this.group1.zxy, this.e315)) + (other.group1.wwwz * float4(this.group0.xyz, this.e125)) + float4(float3(0.0), (other.e5 * this.e321) - (other.e315 * this.e2) - (other.e125 * this.e3) - (other.e321 * this.e5)) + float4((this.group3.www * other.group0.xyz), other.e1 * this.e235) - float4((other.group1.zxy * this.group1.yzx), other.e235 * this.e1)
        );
    }
}
public extension VersorEven {
    public CircleRotor wedge(AntiLine other) {
        return CircleRotor(
            /* e423, e431, e412 */
            float3(this.e4) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4((float3(this.e4) * other.group1.xyz), -(other.e23 * this.e1) - (other.e31 * this.e2) - (other.e12 * this.e3)), 
            /* e235, e315, e125, e12345 */
            float4((other.e23 * this.e5) + (other.e35 * this.e2), (other.e31 * this.e5) + (other.e15 * this.e3), (other.e12 * this.e5) + (other.e25 * this.e1), -(other.e31 * this.e425) - (other.e12 * this.e435) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412)) - float4((other.group1.xyz.yzx * this.group3.zxy), other.e23 * this.e415)
        );
    }
}
public extension VersorEven {
    public VersorEven wedge(AntiMotor other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (other.group0 * float4(this.group3.www, this.e12345)) + float4(float3(0.0), -(other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412)) + float4((other.group0.www * this.group0.xyz), other.e3215 * this.e4), 
            /* e415, e425, e435, e321 */
            float4(other.e15 * this.e4, other.e25 * this.e4, other.e35 * this.e4, -(other.e23 * this.e1) - (other.e31 * this.e2) - (other.e12 * this.e3)) + (float4(other.scalar) * this.group1), 
            /* e235, e315, e125, e5 */
            float4(((float3(other.scalar) * this.group2.xyz) + (float3(this.e5) * other.group0.xyz) + (other.group1.zxy * this.group3.yzx) - (other.group1.yzx * this.group3.zxy)), other.scalar * this.e5), 
            /* e1, e2, e3, e4 */
            float4(other.scalar) * this.group3
        );
    }
}
public extension VersorEven {
    public DipoleInversion wedge(AntiPlane other) {
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(this.e4) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(((other.group0.zxy * this.group3.yzx) - (other.group0.yzx * this.group3.zxy)), other.e5 * this.e4), 
            /* e15, e25, e35, e1234 */
            float4(other.e5 * this.e1, other.e5 * this.e2, other.e5 * this.e3, -(other.e2 * this.e431) - (other.e3 * this.e412)) - (other.group0.xyzx * float4(this.group2.www, this.e423)), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.e3 * this.e425 * -1.0, other.e1 * this.e435 * -1.0, other.e2 * this.e415 * -1.0, (other.e3 * this.e125) + (other.e5 * this.e321)) + (other.group0.yzxx * float4(this.group1.zxy, this.e235)) + (other.group0.wwwy * float4(this.group0.xyz, this.e315))
        );
    }
}
public extension VersorEven {
    public Sphere wedge(Circle other) {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((other.e425 * this.e3) + (other.e235 * this.e4), (other.e435 * this.e1) + (other.e315 * this.e4), (other.e415 * this.e2) + (other.e125 * this.e4), -(other.e321 * this.e5) - (other.e125 * this.e3)) - (this.group3.yzxy * float4(other.group1.zxy, other.e315)) - float4((other.group0.xyz * this.group2.www), other.e235 * this.e1), 
            /* e1234 */
            (other.e423 * this.e1) + (other.e431 * this.e2) + (other.e412 * this.e3) + (other.e321 * this.e4)
        );
    }
}
public extension VersorEven {
    public Sphere wedge(CircleRotor other) {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((other.e425 * this.e3) + (other.e235 * this.e4), (other.e435 * this.e1) + (other.e315 * this.e4), (other.e415 * this.e2) + (other.e125 * this.e4), -(other.e315 * this.e2) - (other.e125 * this.e3)) - (float4(this.e5) * float4(other.group0.xyz, other.e321)) - (this.group3.yzxx * float4(other.group1.zxy, other.e235)), 
            /* e1234 */
            (other.e423 * this.e1) + (other.e431 * this.e2) + (other.e412 * this.e3) + (other.e321 * this.e4)
        );
    }
}
public extension VersorEven {
    public CircleRotor wedge(Dipole other) {
        return CircleRotor(
            /* e423, e431, e412 */
            (float3(this.e4) * other.group1.xyz) + (other.group0.xyz.yzx * this.group3.zxy) - (other.group0.xyz.zxy * this.group3.yzx), 
            /* e415, e425, e435, e321 */
            float4((other.e41 * this.e5) + (other.e15 * this.e4), (other.e42 * this.e5) + (other.e25 * this.e4), (other.e43 * this.e5) + (other.e35 * this.e4), -(other.e31 * this.e2) - (other.e12 * this.e3)) - (other.group1.wwwx * this.group3.xyzx), 
            /* e235, e315, e125, e12345 */
            float4((other.e23 * this.e5) + (other.e35 * this.e2), (other.e31 * this.e5) + (other.e15 * this.e3), (other.e12 * this.e5) + (other.e25 * this.e1), -(other.e42 * this.e315) - (other.e43 * this.e125) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e45 * this.e321) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412)) - float4((other.group2.xyz.yzx * this.group3.zxy), other.e41 * this.e235)
        );
    }
}
public extension VersorEven {
    public CircleRotor wedge(DipoleInversion other) {
        return CircleRotor(
            /* e423, e431, e412 */
            (float3(this.e4) * other.group1.xyz) + (other.group0.xyz.yzx * this.group3.zxy) - (other.group0.xyz.zxy * this.group3.yzx), 
            /* e415, e425, e435, e321 */
            float4((other.e41 * this.e5) + (other.e15 * this.e4), (other.e42 * this.e5) + (other.e25 * this.e4), (other.e43 * this.e5) + (other.e35 * this.e4), -(other.e31 * this.e2) - (other.e12 * this.e3)) - (other.group1.wwwx * this.group3.xyzx), 
            /* e235, e315, e125, e12345 */
            (float4(this.e5) * float4(other.group1.xyz, other.e1234)) + (this.group3.yzxx * float4(other.group2.zxy, other.e4235)) + float4(float3(0.0), (other.e4315 * this.e2) + (other.e4125 * this.e3) + (other.e3215 * this.e4) - (other.e42 * this.e315) - (other.e43 * this.e125) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e45 * this.e321) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412)) - float4((other.group2.yzx * this.group3.zxy), other.e41 * this.e235)
        );
    }
}
public extension VersorEven {
    public Flector wedge(DualNum other) {
        return Flector(
            /* e15, e25, e35, e45 */
            float4(other.e5) * this.group3, 
            /* e4235, e4315, e4125, e3215 */
            float4(other.e5) * float4(this.group0.xyz, this.e321)
        );
    }
}
public extension VersorEven {
    public Motor wedge(FlatPoint other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(other.e15 * this.e4, other.e25 * this.e4, other.e35 * this.e4, -(other.e25 * this.e431) - (other.e35 * this.e412) - (other.e45 * this.e321)) - (other.group0.wwwx * float4(this.group3.xyz, this.e423)), 
            /* e235, e315, e125, e5 */
            float4(((other.group0.zxy * this.group3.yzx) - (other.group0.yzx * this.group3.zxy)), 0.0)
        );
    }
}
public extension VersorEven {
    public Motor wedge(Flector other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            (this.group3.wwwx * float4(other.group0.xyz, other.e4235)) + float4(float3(0.0), (other.e4315 * this.e2) + (other.e4125 * this.e3) + (other.e3215 * this.e4) - (other.e25 * this.e431) - (other.e35 * this.e412) - (other.e45 * this.e321)) - (other.group0.wwwx * float4(this.group3.xyz, this.e423)), 
            /* e235, e315, e125, e5 */
            float4(((other.group0.zxy * this.group3.yzx) - (other.group0.yzx * this.group3.zxy)), 0.0)
        );
    }
}
public extension VersorEven {
    public Plane wedge(Line other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4((other.e425 * this.e3) + (other.e235 * this.e4), (other.e435 * this.e1) + (other.e315 * this.e4), (other.e415 * this.e2) + (other.e125 * this.e4), -(other.e315 * this.e2) - (other.e125 * this.e3)) - (this.group3.yzxx * float4(other.group0.xyz.zxy, other.e235))
        );
    }
}
public extension VersorEven {
    public Flector wedge(Motor other) {
        return Flector(
            /* e15, e25, e35, e45 */
            float4(other.e5) * this.group3, 
            /* e4235, e4315, e4125, e3215 */
            float4((other.e235 * this.e4) + (other.e5 * this.e423), (other.e315 * this.e4) + (other.e5 * this.e431), (other.e125 * this.e4) + (other.e5 * this.e412), -(other.e315 * this.e2) - (other.e125 * this.e3)) + float4((other.group0.yzx * this.group3.zxy), other.e5 * this.e321) - (this.group3.yzxx * float4(other.group0.zxy, other.e235))
        );
    }
}
public extension VersorEven {
    public MultiVector wedge(MultiVector other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, (other.scalar * this.e12345) + (other.e4235 * this.e1) + (other.e4315 * this.e2) + (other.e4125 * this.e3) + (other.e3215 * this.e4) + (other.e1234 * this.e5) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412) - (other.e45 * this.e321) - (other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435)), 
            /* e1, e2, e3, e4 */
            float4(other.scalar) * this.group3, 
            /* e5 */
            other.scalar * this.e5, 
            /* e15, e25, e35, e45 */
            (float4(other.e5) * this.group3) - (float4(this.e5) * other.group1), 
            /* e41, e42, e43 */
            (float3(this.e4) * other.group1.xyz) - (float3(other.e4) * this.group3.xyz), 
            /* e23, e31, e12 */
            (other.group1.zxy * this.group3.yzx) - (other.group1.yzx * this.group3.zxy), 
            /* e415, e425, e435, e321 */
            float4((other.e15 * this.e4) + (other.e41 * this.e5), (other.e25 * this.e4) + (other.e42 * this.e5), (other.e35 * this.e4) + (other.e43 * this.e5), -(other.e31 * this.e2) - (other.e12 * this.e3)) + (float4(other.scalar) * this.group1) - (this.group3.xyzx * float4(other.group3.www, other.e23)), 
            /* e423, e431, e412 */
            (float3(other.scalar) * this.group0.xyz) + (float3(this.e4) * other.group5.xyz) + (other.group4.xyz.yzx * this.group3.zxy) - (other.group4.xyz.zxy * this.group3.yzx), 
            /* e235, e315, e125 */
            (float3(other.scalar) * this.group2.xyz) + (float3(this.e5) * other.group5.xyz) + (other.group3.zxy * this.group3.yzx) - (other.group3.yzx * this.group3.zxy), 
            /* e4235, e4315, e4125, e3215 */
            (float4(other.e5) * float4(this.group0.xyz, this.e321)) + (other.group1.yzxy * float4(this.group1.zxy, this.e315)) + float4((other.group8.xyz * this.group3.www), other.e1 * this.e235) + float4((other.group6.yzx * this.group3.zxy), other.e3 * this.e125) - (other.group6.zxyw * float4(this.group3.yzx, this.e5)) - float4((other.group7.xyz * this.group2.www), other.e235 * this.e1) - float4((other.group1.zxy * this.group1.yzx), other.e315 * this.e2) - float4((other.group1.www * this.group2.xyz), other.e125 * this.e3), 
            /* e1234 */
            (other.e321 * this.e4) + (other.e423 * this.e1) + (other.e431 * this.e2) + (other.e412 * this.e3) - (other.e1 * this.e423) - (other.e2 * this.e431) - (other.e3 * this.e412) - (other.e4 * this.e321)
        );
    }
}
public extension VersorEven {
    public AntiScalar wedge(Plane other) {
        return AntiScalar(
            /* e12345 */
            (other.e4235 * this.e1) + (other.e4315 * this.e2) + (other.e4125 * this.e3) + (other.e3215 * this.e4)
        );
    }
}
public extension VersorEven {
    public DipoleInversion wedge(RoundPoint other) {
        return DipoleInversion(
            /* e41, e42, e43 */
            (float3(this.e4) * other.group0.xyz) - (float3(other.e4) * this.group3.xyz), 
            /* e23, e31, e12, e45 */
            float4((other.group0.zxy * this.group3.yzx), other.e5 * this.e4) - (other.group0.yzxw * float4(this.group3.zxy, this.e5)), 
            /* e15, e25, e35, e1234 */
            float4(other.e5 * this.e1, other.e5 * this.e2, other.e5 * this.e3, -(other.e2 * this.e431) - (other.e3 * this.e412) - (other.e4 * this.e321)) - (other.group0.xyzx * float4(this.group2.www, this.e423)), 
            /* e4235, e4315, e4125, e3215 */
            float4(-(other.e3 * this.e425) - (other.e4 * this.e235), -(other.e1 * this.e435) - (other.e4 * this.e315), -(other.e2 * this.e415) - (other.e4 * this.e125), (other.e3 * this.e125) + (other.e5 * this.e321)) + (other.group0.yzxx * float4(this.group1.zxy, this.e235)) + float4((float3(other.e5) * this.group0.xyz), other.e2 * this.e315)
        );
    }
}
public extension VersorEven {
    public VersorEven wedge(Scalar other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(other.scalar) * this.group0, 
            /* e415, e425, e435, e321 */
            float4(other.scalar) * this.group1, 
            /* e235, e315, e125, e5 */
            float4(other.scalar) * this.group2, 
            /* e1, e2, e3, e4 */
            float4(other.scalar) * this.group3
        );
    }
}
public extension VersorEven {
    public AntiScalar wedge(Sphere other) {
        return AntiScalar(
            /* e12345 */
            (other.e4235 * this.e1) + (other.e4315 * this.e2) + (other.e4125 * this.e3) + (other.e3215 * this.e4) + (other.e1234 * this.e5)
        );
    }
}
public extension VersorEven {
    public DipoleInversion wedge(VersorEven other) {
        return DipoleInversion(
            /* e41, e42, e43 */
            (float3(this.e4) * other.group3.xyz) - (float3(other.e4) * this.group3.xyz), 
            /* e23, e31, e12, e45 */
            (this.group3.yzxw * float4(other.group3.zxy, other.e5)) - (other.group3.yzxw * float4(this.group3.zxy, this.e5)), 
            /* e15, e25, e35, e1234 */
            (this.group3.xyzx * float4(other.group2.www, other.e423)) + float4(float3(0.0), (other.e431 * this.e2) + (other.e412 * this.e3) + (other.e321 * this.e4) - (other.e2 * this.e431) - (other.e3 * this.e412) - (other.e4 * this.e321)) - (other.group3.xyzx * float4(this.group2.www, this.e423)), 
            /* e4235, e4315, e4125, e3215 */
            (other.group3.yzxz * float4(this.group1.zxy, this.e125)) + float4((other.group1.yzx * this.group3.zxy), other.e5 * this.e321) + float4((other.group2.www * this.group0.xyz), other.e2 * this.e315) + float4((this.group3.www * other.group2.xyz), other.e1 * this.e235) - (float4(this.e5) * float4(other.group0.xyz, other.e321)) - (this.group3.yzxx * float4(other.group1.zxy, other.e235)) - float4((other.group3.zxy * this.group1.yzx), other.e315 * this.e2) - float4((other.group3.www * this.group2.xyz), other.e125 * this.e3)
        );
    }
}
public extension VersorEven {
    public VersorEven wedge(VersorOdd other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (float4(other.scalar) * this.group0) + (this.group3.wwwx * float4(other.group1.xyz, other.e4235)) + float4(float3(0.0), (this.e2 * other.e4315) + (this.e3 * other.e4125) + (this.e4 * other.e3215) - (this.e431 * other.e25) - (this.e412 * other.e35) - (this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12) - (this.e321 * other.e45) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43)) + float4((this.group3.zxy * other.group0.yzx), this.e5 * other.e1234) - float4((this.group3.yzx * other.group0.zxy), this.e423 * other.e15), 
            /* e415, e425, e435, e321 */
            float4((this.e5 * other.e41) + (this.e4 * other.e15), (this.e5 * other.e42) + (this.e4 * other.e25), (this.e5 * other.e43) + (this.e4 * other.e35), -(this.e2 * other.e31) - (this.e3 * other.e12)) + (float4(other.scalar) * this.group1) - (this.group3.xyzx * other.group1.wwwx), 
            /* e235, e315, e125, e5 */
            float4(((float3(this.e5) * other.group1.xyz) + (float3(other.scalar) * this.group2.xyz) + (this.group3.yzx * other.group2.zxy) - (this.group3.zxy * other.group2.yzx)), this.e5 * other.scalar), 
            /* e1, e2, e3, e4 */
            float4(other.scalar) * this.group3
        );
    }
}
public extension VersorOdd {
    // Fancy infix trick (first half)
    public WedgeInfixPartial<VersorOdd> operator /(WedgeInfix rhs) {
        return WedgeInfixPartial<VersorOdd>(this);
    }
}
public extension WedgeInfixPartial<VersorOdd> {
    // Fancy infix trick (second half)
    public VersorOdd operator /(AntiCircleRotor rhs) {
        return this.a.wedge(rhs);
    }
}
public extension VersorOdd {
    public VersorOdd wedge(AntiCircleRotor other) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(((float3(other.scalar) * this.group0.xyz) + (float3(this.scalar) * other.group0.xyz)), other.scalar * this.scalar), 
            /* e23, e31, e12, e45 */
            (float4(other.scalar) * this.group1) + (float4(this.scalar) * other.group1), 
            /* e15, e25, e35, e1234 */
            float4(other.scalar * this.e15, other.scalar * this.e25, other.scalar * this.e35, -(other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43)) + (other.group2 * float4(this.group0.www, this.e1234)), 
            /* e4235, e4315, e4125, e3215 */
            float4((other.e23 * this.e45) + (other.e45 * this.e23) + (other.e35 * this.e42) + (other.scalar * this.e4235), (other.e31 * this.e45) + (other.e45 * this.e31) + (other.e15 * this.e43) + (other.scalar * this.e4315), (other.e12 * this.e45) + (other.e45 * this.e12) + (other.e25 * this.e41) + (other.scalar * this.e4125), -(other.e12 * this.e35) - (other.e15 * this.e23) - (other.e25 * this.e31) - (other.e35 * this.e12)) + float4((other.group0.xyz.yzx * this.group2.zxy), other.scalar * this.e3215) - (this.group2.yzxx * float4(other.group0.xyz.zxy, other.e23)) - float4((other.group2.yzx * this.group0.zxy), other.e31 * this.e25)
        );
    }
}
public extension VersorOdd {
    public VersorEven wedge(AntiDipoleInversion other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (other.group3.zxyy * float4(this.group0.yzx, this.e4315)) + float4(float3(0.0), (other.e3 * this.e4125) + (other.e5 * this.e1234) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e321 * this.e45) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43)) + float4((other.group0.xyz * this.group0.www), other.e4 * this.e3215) + float4((other.group2.www * this.group1.xyz), other.e1 * this.e4235) - float4((other.group3.yzx * this.group0.zxy), other.e423 * this.e15), 
            /* e415, e425, e435, e321 */
            float4((other.e4 * this.e15) + (other.e5 * this.e41), (other.e4 * this.e25) + (other.e5 * this.e42), (other.e4 * this.e35) + (other.e5 * this.e43), -(other.e2 * this.e31) - (other.e3 * this.e12)) + (float4(this.scalar) * other.group1) - (other.group3.xyzx * this.group1.wwwx), 
            /* e235, e315, e125, e5 */
            float4(((float3(other.e5) * this.group1.xyz) + (float3(this.scalar) * other.group2.xyz) + (other.group3.yzx * this.group2.zxy) - (other.group3.zxy * this.group2.yzx)), other.e5 * this.scalar), 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * float4(other.group3.xyz, other.e4)
        );
    }
}
public extension VersorOdd {
    public VersorOdd wedge(AntiDualNum other) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(other.scalar) * this.group0, 
            /* e23, e31, e12, e45 */
            float4(other.scalar) * this.group1, 
            /* e15, e25, e35, e1234 */
            float4(other.scalar) * this.group2, 
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xy.yy, other.scalar, (other.e3215 * this.scalar) + (other.scalar * this.e3215)) * float4(this.group3.xyz, 1.0)
        );
    }
}
public extension VersorOdd {
    public CircleRotor wedge(AntiFlatPoint other) {
        return CircleRotor(
            /* e423, e431, e412 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), other.e321 * this.scalar), 
            /* e235, e315, e125, e12345 */
            float4((float3(this.scalar) * other.group0.xyz), -(other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43) - (other.e321 * this.e45))
        );
    }
}
public extension VersorOdd {
    public VersorEven wedge(AntiFlector other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (other.group1.zxyx * float4(this.group0.yzx, this.e4235)) + float4(float3(0.0), (other.e2 * this.e4315) + (other.e3 * this.e4125) + (other.e5 * this.e1234) - (other.e315 * this.e42) - (other.e125 * this.e43) - (other.e321 * this.e45)) - (this.group0.zxyx * float4(other.group1.yzx, other.e235)), 
            /* e415, e425, e435, e321 */
            (this.group0 * float4(other.group1.www, other.e321)) + float4(float3(0.0), -(other.e2 * this.e31) - (other.e3 * this.e12)) - (other.group1.xyzx * this.group1.wwwx), 
            /* e235, e315, e125, e5 */
            float4(((float3(other.e5) * this.group1.xyz) + (float3(this.scalar) * other.group0.xyz) + (other.group1.yzx * this.group2.zxy) - (other.group1.zxy * this.group2.yzx)), other.e5 * this.scalar), 
            /* e1, e2, e3, e4 */
            float4(float3(1.0), 0.0) * float4(this.group0.www, 0.0) * float4(other.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
public extension VersorOdd {
    public DipoleInversion wedge(AntiLine other) {
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12, e45 */
            float4(float3(1.0), 0.0) * float4(other.group0.xyz, 0.0) * float4(this.group0.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e15, e25, e35, e1234 */
            float4((float3(this.scalar) * other.group1.xyz), -(other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43)), 
            /* e4235, e4315, e4125, e3215 */
            float4((other.e23 * this.e45) + (other.e35 * this.e42), (other.e31 * this.e45) + (other.e15 * this.e43), (other.e12 * this.e45) + (other.e25 * this.e41), -(other.e31 * this.e25) - (other.e12 * this.e35) - (other.e15 * this.e23) - (other.e25 * this.e31) - (other.e35 * this.e12)) - float4((other.group1.xyz.yzx * this.group0.zxy), other.e23 * this.e15)
        );
    }
}
public extension VersorOdd {
    public VersorOdd wedge(AntiMotor other) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(other.scalar) * this.group0, 
            /* e23, e31, e12, e45 */
            float4(((float3(other.scalar) * this.group1.xyz) + (float3(this.scalar) * other.group0.xyz)), other.scalar * this.e45), 
            /* e15, e25, e35, e1234 */
            float4(other.e15 * this.scalar, other.e25 * this.scalar, other.e35 * this.scalar, -(other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43)) + (float4(other.scalar) * this.group2), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.e35 * this.e42, other.e15 * this.e43, other.e25 * this.e41, -(other.e31 * this.e25) - (other.e12 * this.e35) - (other.e15 * this.e23) - (other.e25 * this.e31) - (other.e35 * this.e12)) + (other.group0 * float4(this.group1.www, this.e3215)) + float4((other.group0.www * this.group3.xyz), other.e3215 * this.scalar) - float4((other.group1.yzx * this.group0.zxy), other.e23 * this.e15)
        );
    }
}
public extension VersorOdd {
    public VersorEven wedge(AntiPlane other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(other.e2 * this.e43 * -1.0, other.e3 * this.e41 * -1.0, other.e1 * this.e42 * -1.0, (other.e2 * this.e4315) + (other.e3 * this.e4125) + (other.e5 * this.e1234)) + (other.group0.zxyx * float4(this.group0.yzx, this.e4235)), 
            /* e415, e425, e435, e321 */
            float4(other.e5 * this.e41, other.e5 * this.e42, other.e5 * this.e43, -(other.e2 * this.e31) - (other.e3 * this.e12)) - (other.group0.xyzx * this.group1.wwwx), 
            /* e235, e315, e125, e5 */
            float4(((float3(other.e5) * this.group1.xyz) + (other.group0.yzx * this.group2.zxy) - (other.group0.zxy * this.group2.yzx)), other.e5 * this.scalar), 
            /* e1, e2, e3, e4 */
            float4(float3(1.0), 0.0) * float4(this.group0.www, 0.0) * float4(other.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
public extension VersorOdd {
    public AntiScalar wedge(AntiScalar other) {
        return AntiScalar(
            /* e12345 */
            other.e12345 * this.scalar
        );
    }
}
public extension VersorOdd {
    public CircleRotor wedge(Circle other) {
        return CircleRotor(
            /* e423, e431, e412 */
            float3(this.scalar) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(this.scalar) * other.group1, 
            /* e235, e315, e125, e12345 */
            float4((float3(this.scalar) * other.group2.xyz), -(other.e423 * this.e15) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e321 * this.e45) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43))
        );
    }
}
public extension VersorOdd {
    public CircleRotor wedge(CircleRotor other) {
        return CircleRotor(
            /* e423, e431, e412 */
            float3(this.scalar) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(this.scalar) * other.group1, 
            /* e235, e315, e125, e12345 */
            float4((float3(this.scalar) * other.group2.xyz), (other.e12345 * this.scalar) - (other.e423 * this.e15) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e321 * this.e45) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43))
        );
    }
}
public extension VersorOdd {
    public DipoleInversion wedge(Dipole other) {
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(this.scalar) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(this.scalar) * other.group1, 
            /* e15, e25, e35, e1234 */
            float4((float3(this.scalar) * other.group2.xyz), -(other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43)), 
            /* e4235, e4315, e4125, e3215 */
            float4((other.e42 * this.e35) + (other.e23 * this.e45) + (other.e45 * this.e23) + (other.e35 * this.e42), (other.e43 * this.e15) + (other.e31 * this.e45) + (other.e45 * this.e31) + (other.e15 * this.e43), (other.e41 * this.e25) + (other.e12 * this.e45) + (other.e45 * this.e12) + (other.e25 * this.e41), -(other.e23 * this.e15) - (other.e31 * this.e25) - (other.e12 * this.e35) - (other.e35 * this.e12)) - float4((other.group0.xyz.zxy * this.group2.yzx), other.e15 * this.e23) - float4((other.group2.xyz.yzx * this.group0.zxy), other.e25 * this.e31)
        );
    }
}
public extension VersorOdd {
    public DipoleInversion wedge(DipoleInversion other) {
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(this.scalar) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(this.scalar) * other.group1, 
            /* e15, e25, e35, e1234 */
            float4((float3(this.scalar) * other.group2.xyz), (other.e1234 * this.scalar) - (other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43)), 
            /* e4235, e4315, e4125, e3215 */
            float4((other.e23 * this.e45) + (other.e45 * this.e23) + (other.e35 * this.e42) + (other.e4235 * this.scalar), (other.e31 * this.e45) + (other.e45 * this.e31) + (other.e15 * this.e43) + (other.e4315 * this.scalar), (other.e12 * this.e45) + (other.e45 * this.e12) + (other.e25 * this.e41) + (other.e4125 * this.scalar), -(other.e12 * this.e35) - (other.e15 * this.e23) - (other.e25 * this.e31) - (other.e35 * this.e12)) + float4((other.group0.xyz.yzx * this.group2.zxy), other.e3215 * this.scalar) - (this.group2.yzxx * float4(other.group0.xyz.zxy, other.e23)) - float4((other.group2.yzx * this.group0.zxy), other.e31 * this.e25)
        );
    }
}
public extension VersorOdd {
    public Motor wedge(DualNum other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(other.group0.xy.xx, other.e5, (other.e5 * this.e1234) + (other.e12345 * this.scalar)) * float4(this.group0.xyz, 1.0), 
            /* e235, e315, e125, e5 */
            float4(other.e5) * float4(this.group1.xyz, this.scalar)
        );
    }
}
public extension VersorOdd {
    public Flector wedge(FlatPoint other) {
        return Flector(
            /* e15, e25, e35, e45 */
            float4(this.scalar) * other.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4((other.e35 * this.e42) + (other.e45 * this.e23), (other.e15 * this.e43) + (other.e45 * this.e31), (other.e25 * this.e41) + (other.e45 * this.e12), -(other.e25 * this.e31) - (other.e35 * this.e12)) - (other.group0.yzxx * float4(this.group0.zxy, this.e23))
        );
    }
}
public extension VersorOdd {
    public Flector wedge(Flector other) {
        return Flector(
            /* e15, e25, e35, e45 */
            float4(this.scalar) * other.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4((other.e45 * this.e23) + (other.e4235 * this.scalar), (other.e45 * this.e31) + (other.e4315 * this.scalar), (other.e45 * this.e12) + (other.e4125 * this.scalar), -(other.e25 * this.e31) - (other.e35 * this.e12)) + (this.group0.yzxw * float4(other.group0.zxy, other.e3215)) - (other.group0.yzxx * float4(this.group0.zxy, this.e23))
        );
    }
}
public extension VersorOdd {
    public Motor wedge(Line other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((float3(this.scalar) * other.group0.xyz), -(other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43)), 
            /* e235, e315, e125, e5 */
            float4(float3(1.0), 0.0) * float4(other.group1.xyz, 0.0) * float4(this.group0.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
public extension VersorOdd {
    public Motor wedge(Motor other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            (float4(other.e5) * float4(this.group0.xyz, this.e1234)) + (float4(this.scalar) * other.group0) + float4(float3(0.0), -(other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43)), 
            /* e235, e315, e125, e5 */
            float4(((float3(other.e5) * this.group1.xyz) + (float3(this.scalar) * other.group1.xyz)), other.e5 * this.scalar)
        );
    }
}
public extension VersorOdd {
    public MultiVector wedge(MultiVector other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(other.scalar * this.scalar, (other.e12345 * this.scalar) + (other.e1 * this.e4235) + (other.e2 * this.e4315) + (other.e3 * this.e4125) + (other.e4 * this.e3215) + (other.e5 * this.e1234) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e321 * this.e45) - (other.e423 * this.e15) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43)), 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * other.group1, 
            /* e5 */
            other.e5 * this.scalar, 
            /* e15, e25, e35, e45 */
            (float4(other.scalar) * float4(this.group2.xyz, this.e45)) + (float4(this.scalar) * other.group3), 
            /* e41, e42, e43 */
            (float3(other.scalar) * this.group0.xyz) + (float3(this.scalar) * other.group4.xyz), 
            /* e23, e31, e12 */
            (float3(other.scalar) * this.group1.xyz) + (float3(this.scalar) * other.group5.xyz), 
            /* e415, e425, e435, e321 */
            float4((other.e5 * this.e41) + (other.e415 * this.scalar), (other.e5 * this.e42) + (other.e425 * this.scalar), (other.e5 * this.e43) + (other.e435 * this.scalar), -(other.e2 * this.e31) - (other.e3 * this.e12)) + float4((other.group1.www * this.group2.xyz), other.e321 * this.scalar) - (other.group1.xyzx * this.group1.wwwx), 
            /* e423, e431, e412 */
            (float3(other.e4) * this.group1.xyz) + (float3(this.scalar) * other.group7.xyz) + (other.group1.zxy * this.group0.yzx) - (other.group1.yzx * this.group0.zxy), 
            /* e235, e315, e125 */
            (float3(other.e5) * this.group1.xyz) + (float3(this.scalar) * other.group8.xyz) + (other.group1.yzx * this.group2.zxy) - (other.group1.zxy * this.group2.yzx), 
            /* e4235, e4315, e4125, e3215 */
            float4((other.e35 * this.e42) + (other.e45 * this.e23) + (other.e23 * this.e45) + (other.e4235 * this.scalar), (other.e15 * this.e43) + (other.e45 * this.e31) + (other.e31 * this.e45) + (other.e4315 * this.scalar), (other.e25 * this.e41) + (other.e45 * this.e12) + (other.e12 * this.e45) + (other.e4125 * this.scalar), -(other.e15 * this.e23) - (other.e25 * this.e31) - (other.e35 * this.e12) - (other.e12 * this.e35)) + (float4(other.scalar) * this.group3) + float4((other.group4.xyz.yzx * this.group2.zxy), other.e3215 * this.scalar) - (this.group2.yzxx * float4(other.group4.xyz.zxy, other.e23)) - float4((other.group3.yzx * this.group0.zxy), other.e31 * this.e25), 
            /* e1234 */
            (other.scalar * this.e1234) + (other.e1234 * this.scalar) - (other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43)
        );
    }
}
public extension VersorOdd {
    public Plane wedge(Plane other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar) * other.group0
        );
    }
}
public extension VersorOdd {
    public VersorEven wedge(RoundPoint other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(other.e2 * this.e43 * -1.0, other.e3 * this.e41 * -1.0, other.e1 * this.e42 * -1.0, (other.e3 * this.e4125) + (other.e4 * this.e3215) + (other.e5 * this.e1234)) + (other.group0.zxyx * float4(this.group0.yzx, this.e4235)) + (other.group0.wwwy * float4(this.group1.xyz, this.e4315)), 
            /* e415, e425, e435, e321 */
            float4((other.e4 * this.e15) + (other.e5 * this.e41), (other.e4 * this.e25) + (other.e5 * this.e42), (other.e4 * this.e35) + (other.e5 * this.e43), -(other.e2 * this.e31) - (other.e3 * this.e12)) - (other.group0.xyzx * this.group1.wwwx), 
            /* e235, e315, e125, e5 */
            float4(((float3(other.e5) * this.group1.xyz) + (other.group0.yzx * this.group2.zxy) - (other.group0.zxy * this.group2.yzx)), other.e5 * this.scalar), 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * other.group0
        );
    }
}
public extension VersorOdd {
    public VersorOdd wedge(Scalar other) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(other.scalar) * this.group0, 
            /* e23, e31, e12, e45 */
            float4(other.scalar) * this.group1, 
            /* e15, e25, e35, e1234 */
            float4(other.scalar) * this.group2, 
            /* e4235, e4315, e4125, e3215 */
            float4(other.scalar) * this.group3
        );
    }
}
public extension VersorOdd {
    public Sphere wedge(Sphere other) {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar) * other.group0, 
            /* e1234 */
            other.e1234 * this.scalar
        );
    }
}
public extension VersorOdd {
    public VersorEven wedge(VersorEven other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (float4(this.scalar) * other.group0) + (other.group3.wwwx * float4(this.group1.xyz, this.e4235)) + float4(float3(0.0), (other.e2 * this.e4315) + (other.e3 * this.e4125) + (other.e4 * this.e3215) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e321 * this.e45) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43)) + float4((other.group3.zxy * this.group0.yzx), other.e5 * this.e1234) - float4((other.group3.yzx * this.group0.zxy), other.e423 * this.e15), 
            /* e415, e425, e435, e321 */
            float4((other.e5 * this.e41) + (other.e4 * this.e15), (other.e5 * this.e42) + (other.e4 * this.e25), (other.e5 * this.e43) + (other.e4 * this.e35), -(other.e2 * this.e31) - (other.e3 * this.e12)) + (float4(this.scalar) * other.group1) - (other.group3.xyzx * this.group1.wwwx), 
            /* e235, e315, e125, e5 */
            float4(((float3(other.e5) * this.group1.xyz) + (float3(this.scalar) * other.group2.xyz) + (other.group3.yzx * this.group2.zxy) - (other.group3.zxy * this.group2.yzx)), other.e5 * this.scalar), 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * other.group3
        );
    }
}
public extension VersorOdd {
    public VersorOdd wedge(VersorOdd other) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(((float3(other.scalar) * this.group0.xyz) + (float3(this.scalar) * other.group0.xyz)), other.scalar * this.scalar), 
            /* e23, e31, e12, e45 */
            (float4(other.scalar) * this.group1) + (float4(this.scalar) * other.group1), 
            /* e15, e25, e35, e1234 */
            (float4(other.scalar) * this.group2) + (float4(this.scalar) * other.group2) + float4(float3(0.0), -(other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43)), 
            /* e4235, e4315, e4125, e3215 */
            float4((other.e23 * this.e45) + (other.e45 * this.e23) + (other.e35 * this.e42) + (other.e4235 * this.scalar), (other.e31 * this.e45) + (other.e45 * this.e31) + (other.e15 * this.e43) + (other.e4315 * this.scalar), (other.e12 * this.e45) + (other.e45 * this.e12) + (other.e25 * this.e41) + (other.e4125 * this.scalar), -(other.e12 * this.e35) - (other.e15 * this.e23) - (other.e25 * this.e31) - (other.e35 * this.e12)) + (other.group0.yzxw * float4(this.group2.zxy, this.e3215)) + float4((other.group0.www * this.group3.xyz), other.e3215 * this.scalar) - (this.group2.yzxx * float4(other.group0.zxy, other.e23)) - float4((other.group2.yzx * this.group0.zxy), other.e31 * this.e25)
        );
    }
}
